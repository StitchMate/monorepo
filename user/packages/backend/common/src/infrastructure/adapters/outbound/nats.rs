//TODO: Types
use std::sync::Arc;

use anyhow::anyhow;
use async_nats::jetstream::consumer::{push, DeliverPolicy};
use async_nats::jetstream::stream::Config;
use async_nats::jetstream::Message;
use async_nats::{header::NATS_MESSAGE_ID, jetstream::consumer::AckPolicy, HeaderMap};
use async_trait::async_trait;
use base::domain::entity::event::EventMetadata;
use base::{
    domain::entity::event::EventEnvelope,
    infrastructure::{
        adapters::outbound::event_bus::nats::NATSConnector, dto::event_bus::nats::NATSEventEnvelope,
    },
};
use futures_util::stream::{BoxStream, StreamExt};
use futures_util::Future;

use crate::application::ports::outbound::event_bus::UserEventBus;
use crate::domain::event::UserEvent;
use crate::infrastructure::dto::nats_backup::NATSUserEvent;
use crate::infrastructure::upcasters::nats::{NATSUserEventUpcaster, NATS_USER_EVENT_UPCASTERS};

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
    async fn send_event(&self, event: EventEnvelope<UserEvent>) -> Result<(), anyhow::Error> {
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
            return Err(anyhow!(result.unwrap_err()));
        }
        Ok(())
    }
    async fn receive_events(
        &self,
        topic: &str,
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
                durable_name: Some(format!("consumer_{}", topic.replace(".*", "_wildcard"))),
                ack_policy: AckPolicy::All,
                deliver_policy: DeliverPolicy::New,
                deliver_subject: format!("consumer_{}", topic.replace(".*", "_wildcard")),
                filter_subjects: vec![topic.into()],
                ..Default::default()
            })
            .await
            .map_err(|e| anyhow!(e))?;

        let stream = consumer.messages().await?.map(move |x| {
            let message = x.unwrap();
            let mut event: NATSEventEnvelope<NATSUserEvent> =
                serde_json::from_slice(&message.payload).unwrap();
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

            (transformed, Box::pin(async_ack(message)))
        });
        // NOTE: 0 is indicative that this a push consumer and will receive unlimited events to the stream vs. a pull consumer
        Ok((-1, stream.boxed()))
    }
}
