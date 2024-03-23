use base::domain::entity::event::EventMetadata;
use chrono::{serde::ts_microseconds, DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    domain::{
        event::{UserCreatedEvent, UserEvent},
        value_object::email::Email,
    },
    infrastructure::dto::name::nats::NATSName,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "event_wrapper_type")]
pub enum NATSUserEvent {
    UserCreated(NATSUserCreatedEvent),
}

impl EventMetadata for NATSUserEvent {
    fn event_id(&self) -> String {
        match self {
            Self::UserCreated(e) => e.event_id(),
        }
    }
    fn event_type(&self) -> String {
        match self {
            Self::UserCreated(e) => e.event_type(),
        }
    }
    fn event_version(&self) -> String {
        match self {
            Self::UserCreated(e) => e.event_version(),
        }
    }
}

impl Default for NATSUserEvent {
    fn default() -> Self {
        Self::UserCreated(NATSUserCreatedEvent::default())
    }
}

impl From<NATSUserEvent> for UserEvent {
    fn from(value: NATSUserEvent) -> Self {
        match value {
            NATSUserEvent::UserCreated(e) => UserEvent::UserCreated(e.into()),
        }
    }
}

impl From<UserEvent> for NATSUserEvent {
    fn from(value: UserEvent) -> Self {
        match value {
            UserEvent::UserCreated(e) => Self::UserCreated(e.into()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NATSUserCreatedEvent {
    pub id: String,
    pub name: NATSName,
    pub email: String,
    #[serde(with = "ts_microseconds")]
    pub created_at: DateTime<Utc>,
    pub event_id: String,
    pub event_type: String,
    pub event_version: String,
}

impl EventMetadata for NATSUserCreatedEvent {
    fn event_id(&self) -> String {
        self.event_id.clone()
    }
    fn event_type(&self) -> String {
        self.event_type.clone()
    }
    fn event_version(&self) -> String {
        self.event_version.clone()
    }
}

impl From<UserCreatedEvent> for NATSUserCreatedEvent {
    fn from(value: UserCreatedEvent) -> Self {
        let UserCreatedEvent {
            id,
            name,
            email,
            event_id,
            created_at,
        } = value.clone();
        Self {
            id,
            name: name.into(),
            email: email.into(),
            created_at,
            event_id,
            event_type: value.event_type(),
            event_version: value.event_version(),
        }
    }
}

impl From<NATSUserCreatedEvent> for UserCreatedEvent {
    fn from(value: NATSUserCreatedEvent) -> Self {
        UserCreatedEvent {
            id: value.id,
            email: Email::new(value.email),
            name: value.name.into(),
            created_at: value.created_at,
            event_id: value.event_id,
        }
    }
}

impl From<NATSUserCreatedEvent> for NATSUserEvent {
    fn from(value: NATSUserCreatedEvent) -> Self {
        NATSUserEvent::UserCreated(value)
    }
}
