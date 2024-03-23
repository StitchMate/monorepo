use crate::application::ports::outbound::event_bus::UserEventBus;
use crate::domain::event::UserEvent;
use base::domain::entity::error::EventBusError;
use base::domain::entity::event::EventEnvelope;
use enum_dispatch::enum_dispatch;
use futures_util::stream::BoxStream;
use futures_util::Future;

use self::nats::NATSUserEventBus;

pub mod nats;

#[enum_dispatch(UserEventBus)]
pub enum UserEventBusEnum {
    NATSUserEventBus(NATSUserEventBus),
}
