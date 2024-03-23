use std::sync::Arc;

use anyhow::anyhow;
use async_nats::jetstream::consumer::{push, DeliverPolicy};
use async_nats::jetstream::stream::Config;
use async_nats::jetstream::Message;
use async_nats::{header::NATS_MESSAGE_ID, jetstream::consumer::AckPolicy, HeaderMap};
use async_trait::async_trait;
use base::domain::entity::error::EventBusError;
use base::domain::entity::event::EventMetadata;
use base::{
    domain::entity::event::EventEnvelope,
    infrastructure::{
        adapters::outbound::event_bus::nats::NATSConnector, dto::event_bus::nats::NATSEventEnvelope,
    },
};
use futures_util::stream::{BoxStream, StreamExt};
use futures_util::Future;
use tracing::{error, warn};

use crate::application::ports::outbound::event_bus::UserEventBus;
use crate::domain::event::UserEvent;
use crate::infrastructure::dto::user::event::nats::NATSUserEvent;
use crate::infrastructure::upcasters::user::nats::{
    NATSUserEventUpcaster, NATS_USER_EVENT_UPCASTERS,
};

pub type AckFuture = impl Future<Output = Result<(), anyhow::Error>> + Sync + Send;

async fn async_ack(message: Message) -> Result<(), anyhow::Error> {
    message.ack().await.map_err(|e| anyhow!(e))
}

#[derive(Clone)]
pub struct NATSUserEventBus {
    pub connector: Arc<NATSConnector>,
}

impl NATSUserEventBus {
    fn event_to_topic(&self, event: &EventEnvelope<UserEvent>) -> String {
        match event.payload {
            UserEvent::UserCreated(_) => "user.created".into(),
        }
    }
}

#[async_trait]
impl UserEventBus for NATSUserEventBus {
    async fn send_event(&self, event: EventEnvelope<UserEvent>) -> Result<(), EventBusError> {
        let topic = self.event_to_topic(&event);
        let envelope: NATSEventEnvelope<NATSUserEvent> = NATSEventEnvelope {
            aggregate_id: event.aggregate_id,
            aggregate_type: event.aggregate_type,
            sequence: event.sequence,
            payload: event.payload.into(),
            metadata: event.metadata,
            timestamp: event.timestamp,
        };
        let event_id = &envelope.payload.event_id();
        let serialized: String = envelope.into();
        let mut headers: HeaderMap = HeaderMap::new();
        headers.insert(NATS_MESSAGE_ID, event_id.as_str());
        let result = self
            .connector
            .jetstream_client
            .publish_with_headers(topic, headers, serialized.into())
            .await;
        if result.is_err() {
            return Err(EventBusError::SendError(result.unwrap_err().to_string()));
        }
        Ok(())
    }
    async fn receive_events(
        &self,
        topic: &str,
        durable_name: &str,
    ) -> Result<(i64, BoxStream<'_, (EventEnvelope<UserEvent>, AckFuture)>), anyhow::Error> {
        let stream = self
            .connector
            .jetstream_client
            .get_or_create_stream(Config {
                name: "user_events".into(),
                subjects: vec!["user.*".into()],
                ..Default::default()
            })
            .await
            .map_err(|e| anyhow!(e))?;

        let consumer = stream
            .create_consumer(push::Config {
                durable_name: Some(durable_name.to_string()),
                ack_policy: AckPolicy::All,
                deliver_policy: DeliverPolicy::New,
                deliver_subject: format!(
                    "consumer_{}_{}",
                    durable_name,
                    topic.replace(".*", "_wildcard")
                ),
                filter_subjects: vec![topic.into()],
                ..Default::default()
            })
            .await
            .map_err(|e| anyhow!(e))?;

        let stream = consumer.messages().await?.map(move |x| {
            let message = x.unwrap();
            let mut event: Option<NATSEventEnvelope<NATSUserEvent>> = None;
            match serde_json::from_slice::<NATSEventEnvelope<NATSUserEvent>>(&message.payload) {
                Ok(x) => event = Some(x),
                Err(e) => {
                    warn!(payload = ?message.payload, "received unknown event payload. acking to clear queue");
                    let message = message.clone();
                    tokio::spawn(async move {
                        if let Err(e) = message.ack().await {
                            error!(error = ? e, event = ?&message.payload, "failed to ack");
                        }
                    });
                }
            }
            if event.is_none() {
                return (None, Box::pin(async_ack(message)));
            }
            let mut event = event.unwrap();
            if let Some(upcasters) = NATS_USER_EVENT_UPCASTERS.get(&format!(
                "{}_{}",
                &event.payload.event_type(),
                &event.payload.event_version()
            )) {
                upcasters.iter().for_each(|upcaster| {
                    if upcaster
                        .can_upcast(&event.payload.event_type(), &event.payload.event_version())
                    {
                        if let Some(upcasted) = upcaster.upcast(event.payload.clone()) {
                            event.payload = upcasted
                        }
                    }
                })
            }
            let transformed: EventEnvelope<UserEvent> = event.into();

            (Some(transformed), Box::pin(async_ack(message)))
        }).filter_map(|(x, y)| async move { 
            match x {
                Some(x) => Some((x, y)),
                None => None
            }
        });
        // NOTE: 0 is indicative that this a push consumer and will receive unlimited events to the stream vs. a pull consumer
        Ok((-1, stream.boxed()))
    }
}
