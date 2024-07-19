use std::sync::Arc;

use async_trait::async_trait;
use base::domain::{entity::event::EventEnvelope, entity::snapshot::AggregateSnapshot};
use enum_dispatch::enum_dispatch;
use user_common::{
    application::ports::outbound::event_bus::UserEventBus, domain::event::UserEvent,
};

use crate::domain::user::{aggregate::UserAggregate, error::repository::UserEventRepositoryError};


//TODO: Concrete Error Types
#[async_trait]
#[enum_dispatch]
pub trait UserEventRepository {
    async fn migrate(&self, path: String) -> Result<(), UserEventRepositoryError>;
    async fn aggregate_exists_by_id(
        &self,
        aggregate_id: &str,
    ) -> Result<(), UserEventRepositoryError>;
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
    async fn store_snapshot(
        &self,
        snapshot: AggregateSnapshot<UserAggregate>,
    ) -> Result<(), UserEventRepositoryError>;
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
