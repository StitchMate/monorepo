use async_trait::async_trait;
use base::application::ports::inbound::projector::{Projector, ProjectorHandler};
use futures_util::StreamExt;
use crate::domain::user::error::service::UserProjectorServiceError;
use std::sync::Arc;
use tracing::{info, error};

use user_common::application::ports::outbound::event_bus::UserEventBus;
use user_common::{
    domain::event::{UserCreatedEvent, UserEvent},
    infrastructure::adapters::outbound::user::event_bus::UserEventBusEnum,
};

use crate::application::ports::inbound::create_customer_projector::CreateUserFromProjectorUseCase;
use crate::application::service::user::projector::UserProjectorService;

pub struct UserEventProjector {
    bus: Arc<UserEventBusEnum>,
    service: UserProjectorService,
    //TODO: Revamp config for pull vs push based consumer
    topic: String,
    durable_name: String
}

impl UserEventProjector {
    pub fn new(bus: Arc<UserEventBusEnum>, service: UserProjectorService, topic: String, durable_name: String) -> Self {
        Self { bus, service, topic, durable_name }
    }
}

#[async_trait]
impl Projector for UserEventProjector {
    async fn run(&self) -> Result<(), anyhow::Error> {
        info!("starting user-query event projector");
        let (_, mut stream) = self.bus.receive_events(&self.topic, &self.durable_name).await?;
        while let Some((event, ack)) = stream.next().await {
            if let Err(e) = match &event.payload {
                UserEvent::UserCreated(e) => self.handle(e.clone()).await,
            } {
                error!(error = ?e, payload = ?event.payload, "could not process event");
                continue;
            }
            if let Err(e) = ack.await {
                error!(error = ? e, event = ?event, "failed to ack");
            }
        }
        return Ok(());
    }
}

#[async_trait]
impl ProjectorHandler<UserCreatedEvent, UserProjectorServiceError> for UserEventProjector {
    async fn handle(&self, event: UserCreatedEvent) -> Result<(), UserProjectorServiceError> {
        let UserCreatedEvent {
            id,
            name,
            email,
            created_at,
            ..
        } = &event;
        self.service
            .create_user(id, name, email, created_at)
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, time::Duration};

    use crate::{
        application::ports::outbound::repository::UserQueryRepository,
        infrastructure::adapters::outbound::user::repository::postgres::MockPostgresUserQueryRepository,
    };
    use base::{
        domain::entity::event::EventEnvelope,
        infrastructure::adapters::outbound::event_bus::nats::NATSConnector,
    };
    use chrono::Utc;
    use tokio::time::sleep;
    use user_common::{
        domain::value_object::{email::Email, name::Name},
        infrastructure::adapters::outbound::user::event_bus::nats::NATSUserEventBus,
    };

    use crate::infrastructure::adapters::outbound::user::repository::UserQueryRepositoryEnum;

    use super::*;

    #[tokio::test]
    async fn it_works() {
        let mut repo = MockPostgresUserQueryRepository::new();

        repo.expect_migrate()
            .returning(|_| Box::pin(async { Ok(()) }));
        repo.expect_create()
            .returning(|_, _, _, _| Box::pin(async { Ok(()) }));

        let query_repo: Arc<UserQueryRepositoryEnum> = Arc::new(repo.into());

        let migrate = query_repo.migrate("src/migrations".into()).await;

        println!("MIGRATION ERROR: {:?}", migrate);

        let service: UserProjectorService = UserProjectorService::new(query_repo);

        let nats = NATSConnector::new("nats://localhost:4222".into())
            .await
            .unwrap();

        let bus: Arc<UserEventBusEnum> = Arc::new(NATSUserEventBus { connector: nats }.into());

        let event: UserEvent = UserCreatedEvent {
            name: Name {
                first: "BOB".into(),
                last: "test".into(),
            },
            email: Email::new("test@test.com"),
            created_at: Utc::now(),
            ..Default::default()
        }
        .into();

        let metadata = HashMap::new();
        let env: EventEnvelope<UserEvent> = EventEnvelope {
            aggregate_id: "BOB2".into(),
            aggregate_type: "user".into(),
            sequence: "BOB2".into(),
            payload: event,
            metadata: metadata.clone(),
            timestamp: Utc::now(),
        };

        let event2: UserEvent = UserCreatedEvent {
            name: Name {
                first: "BOB".into(),
                last: "test".into(),
            },
            email: Email::new("test@test.com"),
            created_at: Utc::now(),
            ..Default::default()
        }
        .into();

        let env2: EventEnvelope<UserEvent> = EventEnvelope {
            aggregate_id: "BOB3".into(),
            aggregate_type: "user".into(),
            sequence: "BOB3".into(),
            payload: event2,
            metadata: metadata,
            timestamp: Utc::now(),
        };

        let projector: UserEventProjector = UserEventProjector::new(bus.clone(), service, "user.*".into(), "user_projector_test_consumer".into());
        tokio::spawn(async move {
            match projector.run().await {
                Err(e) => println!("ERROR PROJECTOR: {:?}", e),
                _ => {}
            }
        });

        let result = bus.send_event(env).await;

        println!("RESULT SEND MESSAGE: {:?}", result);

        let result2 = bus.send_event(env2).await;

        println!("RESULT SEND MESSAGE: {:?}", result2);

        sleep(Duration::from_millis(3000)).await;
    }
}
