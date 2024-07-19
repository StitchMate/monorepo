use base::{Event, EventMetadata};
use base::domain::entity::event::EventMetadata;
use chrono::{DateTime, Utc};
use educe::Educe;

use super::value_object::email::Email;
use super::value_object::name::Name;

#[derive(Event, Debug, Clone, PartialEq)]
pub enum UserEvent {
    UserCreated(UserCreatedEvent),
}

#[derive(EventMetadata, Debug, Clone, PartialEq, Educe)]
#[educe(Default)]
#[event_version("v1")]
#[event_type("UserCreated")]
pub struct UserCreatedEvent {
    #[educe(Default(expression = format!("usr_{}", ulid::Ulid::new())))]
    pub id: String,
    pub name: Name,
    pub email: Email,
    #[educe(Default(expression = Utc::now()))]
    pub created_at: DateTime<Utc>,
    #[event_id]
    #[educe(Default(expression = ulid::Ulid::new().to_string()))]
    pub event_id: String,
}

impl From<UserCreatedEvent> for UserEvent {
    fn from(value: UserCreatedEvent) -> Self {
        UserEvent::UserCreated(value)
    }
}