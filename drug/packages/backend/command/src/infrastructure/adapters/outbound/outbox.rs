use async_trait::async_trait;
use futures::future::try_join_all;
use std::sync::Arc;
use tracing::{debug, error, info};

use base::{application::ports::inbound::outbox::Outbox, config::outbox::Timeout};

use user_common::infrastructure::adapters::outbound::user::event_bus::UserEventBusEnum;

use crate::application::ports::outbound::repository::UserEventRepository;

use super::event_repository::UserEventRepositoryEnum;

pub struct UserEventOutboxWorker {
    bus: Arc<UserEventBusEnum>,
    repository: Arc<UserEventRepositoryEnum>,
    timeout: u32,
}

impl UserEventOutboxWorker {
    pub fn new(
        bus: Arc<UserEventBusEnum>,
        repository: Arc<UserEventRepositoryEnum>,
        timeout: &Timeout,
    ) -> Self {
        Self {
            bus,
            repository,
            timeout: timeout.value(),
        }
    }
}

#[async_trait]
impl Outbox for UserEventOutboxWorker {
    async fn run(&self) -> Result<(), anyhow::Error> {
        info!("starting outbox worker");
        let mut interval =
            tokio::time::interval(std::time::Duration::from_secs(self.timeout as u64));

        loop {
            interval.tick().await;
            match self.repository.retrieve_outbox_events().await {
                Ok(events) => {
                    if events.len() == 0 {
                        continue;
                    }
                    debug!(number = events.len(), "found events in outbox");
                    let subtasks = events.into_iter().map(|event| {
                        let repository = self.repository.clone();
                        let bus = self.bus.clone();
                        tokio::spawn(async move {
                            match repository.send_and_delete_outbox_event(bus, event).await {
                                Ok(_) => {}
                                Err(e) => {
                                    error!(error = ?e, "failed to send outbox event");
                                }
                            }
                        })
                    });
                    match try_join_all(subtasks).await {
                        Err(e) => {
                            error!(error = ?e, "failed to process outbox event");
                        }
                        _ => {}
                    }
                }
                Err(e) => {
                    error!(error = ?e, "failed to retrieve outbox events");
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, time::Duration};

    use base::{
        domain::entity::event::EventEnvelope,
        infrastructure::adapters::outbound::event_bus::nats::NATSConnector,
    };
    use chrono::Utc;
    use futures_util::StreamExt;
    use tokio::time::sleep;
    use user_common::application::ports::outbound::event_bus::UserEventBus;
    use user_common::{
        domain::{
            event::UserCreatedEvent,
            value_object::{email::Email, name::Name},
        },
        infrastructure::adapters::outbound::user::event_bus::nats::NATSUserEventBus,
    };

    use crate::infrastructure::adapters::outbound::event_repository::MockUserEventRepository;

    use user_common::domain::event::UserEvent;

    use super::*;

    #[tokio::test]
    async fn it_works() {
        let mut repo = MockUserEventRepository::new();

        repo.expect_retrieve_outbox_events().returning(|| {
            Ok(vec![EventEnvelope {
                aggregate_id: "test".into(),
                aggregate_type: "user".into(),
                sequence: "test".into(),
                metadata: HashMap::new(),
                timestamp: Utc::now(),
                payload: UserEvent::UserCreated(UserCreatedEvent {
                    id: "test".into(),
                    name: Name::new("test", "test"),
                    email: Email::new("test@test.com"),
                    ..Default::default()
                }),
            }])
        });

        repo.expect_send_and_delete_outbox_event().returning(
            |_bus: Arc<UserEventBusEnum>, event| {
                println!("SENDING EVENT: {:?}", event);
                return Ok(());
            },
        );

        let event_repo: Arc<UserEventRepositoryEnum> = Arc::new(repo.into());

        let nats = NATSConnector::new("nats://localhost:4222".into())
            .await
            .unwrap();

        // TODO: We should mock this
        let bus: Arc<UserEventBusEnum> = Arc::new(NATSUserEventBus { connector: nats }.into());

        let outbox_worker: UserEventOutboxWorker =
            UserEventOutboxWorker::new(bus.clone(), event_repo, &Timeout(5));

        tokio::spawn(async move {
            let (_, mut stream) = bus
                .receive_events("user.*", "test_outbox_consumer")
                .await
                .unwrap();
            while let Some((event, _ack)) = stream.next().await {
                println!("RECEIVED: {:?}", event.payload);
            }
        });

        tokio::spawn(async move {
            match outbox_worker.run().await {
                Err(e) => println!("ERROR PROJECTOR: {:?}", e),
                _ => {}
            }
        });

        sleep(Duration::from_millis(3000)).await;
    }
}
