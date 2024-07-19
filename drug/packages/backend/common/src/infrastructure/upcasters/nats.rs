use std::collections::HashMap;

use enum_dispatch::enum_dispatch;
use once_cell::sync::Lazy;

use crate::infrastructure::dto::nats_backup::{NATSUserCreatedEvent, NATSUserEvent};

#[enum_dispatch]
pub trait NATSUserEventUpcaster {
    fn can_upcast(&self, event_type: &str, event_version: &str) -> bool;
    fn upcast(&self, event: NATSUserEvent) -> Option<NATSUserEvent>;
}

//Example Upcaster
#[enum_dispatch(NATSUserEventUpcaster)]
pub enum NATSUserEventUpcasterEnum {
    NATSUserCreatedAddEmail(NATSUserCreatedAddEmail),
}

//TODO: Currently useless, but here's an example
pub struct NATSUserCreatedAddEmail;

impl NATSUserEventUpcaster for NATSUserCreatedAddEmail {
    fn can_upcast(&self, event_type: &str, event_version: &str) -> bool {
        event_version == "v1" && event_type == "UserCreated"
    }

    fn upcast(&self, event: NATSUserEvent) -> Option<NATSUserEvent> {
        match event {
            NATSUserEvent::UserCreated(event) => Some(
                NATSUserCreatedEvent {
                    id: event.id,
                    name: event.name,
                    email: event.email,
                    created_at: event.created_at,
                    event_id: event.event_id,
                    event_type: event.event_type,
                    event_version: event.event_version,
                }
                .into(),
            ),
        }
    }
}

pub static NATS_USER_EVENT_UPCASTERS: Lazy<HashMap<String, Vec<NATSUserEventUpcasterEnum>>> =
    Lazy::new(HashMap::new);
