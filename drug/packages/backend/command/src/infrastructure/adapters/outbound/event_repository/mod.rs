use crate::domain::user::aggregate::UserAggregate;
use crate::domain::user::error::repository::UserEventRepositoryError;
#[cfg(test)]
use async_trait::async_trait;
use base::domain::entity::event::EventEnvelope;
use base::domain::entity::snapshot::AggregateSnapshot;
use enum_dispatch::enum_dispatch;
#[cfg(test)]
use mockall::mock;
use std::sync::Arc;

use crate::application::ports::outbound::repository::UserEventRepository;
use user_common::application::ports::outbound::event_bus::UserEventBus;
use user_common::domain::event::UserEvent;

use self::postgres::PostgresUserEventRepository;

pub mod postgres;

#[cfg(test)]
mock! {
    pub UserEventRepository {
    }

    impl Clone for UserEventRepository {
        fn clone(&self) -> Self;
    }

    #[async_trait]
    impl UserEventRepository for UserEventRepository {
        async fn migrate(&self, path: String) -> Result<(), UserEventRepositoryError>;
        async fn aggregate_exists_by_id(&self, aggregate_id: &str) -> Result<(), UserEventRepositoryError>;
        async fn aggregate_exists_by_email(&self, email: &str) -> Result<(), UserEventRepositoryError>;
        async fn store_events(
            &self,
            events: Vec<EventEnvelope<UserEvent>>,
        ) -> Result<(), Vec<UserEventRepositoryError>>;
        async fn retrieve_events(
            &self,
            aggregate_id: String,
            after: Option<String>,
        ) -> Result<Vec<EventEnvelope<UserEvent>>, UserEventRepositoryError>;
        async fn store_snapshot(&self, snapshot: AggregateSnapshot<UserAggregate>) -> Result<(), UserEventRepositoryError>;
        async fn retrieve_latest_snapshot(
            &self,
            aggregate_id: String,
        ) -> Result<Option<AggregateSnapshot<UserAggregate>>, UserEventRepositoryError>;
        async fn retrieve_outbox_events(
            &self,
        ) -> Result<Vec<EventEnvelope<UserEvent>>, UserEventRepositoryError>;
        // Used by outbox pattern to remove events after sending
        async fn send_and_delete_outbox_event<B: UserEventBus + Send + Sync + 'static>(
            &self,
            bus: Arc<B>,
            event: EventEnvelope<UserEvent>,
        ) -> Result<(), UserEventRepositoryError>;
    }
}

#[derive(Clone)]
#[enum_dispatch(UserEventRepository)]
pub enum UserEventRepositoryEnum {
    PostgresUserEventRepository(PostgresUserEventRepository),
    #[cfg(test)]
    MockUserEventRepository(MockUserEventRepository),
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use base::{
        config::repository::PostgresConfig,
        infrastructure::adapters::outbound::{
            event_bus::nats::NATSConnector, storage::postgres::PostgresConnector,
        },
    };
    use chrono::Utc;
    use user_common::{
        domain::{
            event::UserCreatedEvent,
            value_object::{email::Email, name::Name},
        },
        infrastructure::adapters::outbound::user::event_bus::{
            nats::NATSUserEventBus, UserEventBusEnum,
        },
    };

    use super::*;

    #[tokio::test]
    async fn it_works() {
        let config: PostgresConfig = PostgresConfig {
            host: "localhost".into(),
            port: 5432,
            database_name: "postgres".into(),
            username: Some("postgres".into()),
            password: Some("password".into()),
            migration_path: "src/migrations".into(),
        }
        .into();

        let connector = PostgresConnector::new(&config).await.unwrap();

        let prepo = PostgresUserEventRepository { connector };

        let _ = prepo.drop().await;

        let repository: UserEventRepositoryEnum = prepo.into();

        let migrate = repository.migrate("src/migrations".into()).await;

        println!("MIGRATION: {:?}", migrate);

        let nats = NATSConnector::new("nats://localhost:4222".into())
            .await
            .unwrap();

        let bus: Arc<UserEventBusEnum> = Arc::new(NATSUserEventBus { connector: nats }.into());

        let event: UserEvent = UserCreatedEvent {
            id: "test".into(),
            name: Name {
                first: "test".into(),
                last: "test".into(),
            },
            email: Email::new("test@test.com"),
            created_at: Utc::now(),
            event_id: "test".into(),
        }
        .into();

        let metadata = HashMap::new();
        let env: EventEnvelope<UserEvent> = EventEnvelope {
            aggregate_id: "test".into(),
            aggregate_type: "user".into(),
            sequence: "test".into(),
            payload: event,
            metadata: metadata,
            timestamp: Utc::now(),
        };

        let _ = repository.store_events(vec![env.clone()]).await;

        let _ = repository.send_and_delete_outbox_event(bus, env).await;

        let res3 = repository.aggregate_exists_by_email("test@test.com").await;

        println!("EMAIL EXISTS: {:?}", res3);
    }
}
