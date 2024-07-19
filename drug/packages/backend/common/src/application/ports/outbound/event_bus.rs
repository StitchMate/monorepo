use async_trait::async_trait;
use base::domain::entity::{error::EventBusError, event::EventEnvelope};
use enum_dispatch::enum_dispatch;
use futures::{stream::BoxStream, Future};

use crate::domain::event::UserEvent;

#[async_trait]
#[enum_dispatch]
pub trait UserEventBus {
    async fn send_event(&self, event: EventEnvelope<UserEvent>) -> Result<(), EventBusError>;
    async fn receive_events(
        &self,
        topic: &str,
        durable_name: &str
    ) -> Result<
        (
            i64,
            BoxStream<
                '_,
                (
                    EventEnvelope<UserEvent>,
                    impl Future<Output = Result<(), anyhow::Error>> + Sync + Send,
                ),
            >,
        ),
        anyhow::Error,
    >;
}
