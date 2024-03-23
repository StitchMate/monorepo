use base::domain::entity::event::EventMetadata;
use chrono::{serde::ts_microseconds, DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    domain::{
        event::{UserCreatedEvent, UserEvent},
        value_object::email::Email,
    },
    infrastructure::dto::name::sql::SQLName,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "event_wrapper_type")]
pub enum SQLUserEvent {
    UserCreated(SQLUserCreatedEvent),
}

impl Default for SQLUserEvent {
    fn default() -> Self {
        Self::UserCreated(SQLUserCreatedEvent::default())
    }
}

impl From<UserEvent> for SQLUserEvent {
    fn from(value: UserEvent) -> Self {
        match value {
            UserEvent::UserCreated(e) => Self::UserCreated(e.into()),
        }
    }
}

impl From<SQLUserEvent> for UserEvent {
    fn from(value: SQLUserEvent) -> Self {
        match value {
            SQLUserEvent::UserCreated(e) => UserEvent::UserCreated(e.into()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct SQLUserCreatedEvent {
    pub id: String,
    pub name: SQLName,
    pub email: String,
    #[serde(with = "ts_microseconds")]
    pub created_at: DateTime<Utc>,
    pub event_id: String,
    pub event_type: String,
    pub event_version: String,
}

impl From<UserCreatedEvent> for SQLUserCreatedEvent {
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

impl From<SQLUserCreatedEvent> for UserCreatedEvent {
    fn from(value: SQLUserCreatedEvent) -> Self {
        UserCreatedEvent {
            id: value.id,
            email: Email::new(value.email),
            name: value.name.into(),
            created_at: value.created_at,
            event_id: value.event_id,
        }
    }
}

impl From<SQLUserCreatedEvent> for SQLUserEvent {
    fn from(value: SQLUserCreatedEvent) -> Self {
        SQLUserEvent::UserCreated(value)
    }
}
