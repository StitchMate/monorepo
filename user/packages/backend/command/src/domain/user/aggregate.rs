use std::sync::Arc;

use async_trait::async_trait;
use base::{
    domain::entity::{
        aggregate::Aggregate,
        command::HandleCommand,
        event::{ApplyEvent, Event},
        snapshot::{AggregateSnapshot, ApplySnapshot, Snapshot},
    },
    Aggregate,
};
use chrono::{DateTime, Utc};
use struct_field_names_as_array::FieldNamesAsArray;
use ulid::Ulid;
use user_common::domain::{
    event::{UserCreatedEvent, UserEvent},
    value_object::{email::Email, name::Name},
};

use crate::{
    application::ports::outbound::repository::UserEventRepository,
    infrastructure::adapters::outbound::event_repository::UserEventRepositoryEnum,
};

use super::{command::UserCommand, error::command::UserCommandError};

#[derive(Clone, Debug, Default, FieldNamesAsArray, Aggregate, PartialEq)]
#[aggregate_type("user")]
#[event_type(UserEvent)]
pub struct UserAggregate {
    #[aggregate_id]
    pub id: Option<String>,
    pub name: Option<Name>,
    pub email: Option<Email>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub last_event: Option<UserEvent>,
    pub applied_events: i32,
}

pub struct CommandContext {
    pub repository: Arc<UserEventRepositoryEnum>,
}

#[async_trait]
impl HandleCommand<UserCommand> for UserAggregate {
    type Error = UserCommandError;
    type Event = UserEvent;
    type Context = CommandContext;

    async fn handle(
        &self,
        command: UserCommand,
        context: Arc<Self::Context>,
    ) -> Result<Vec<Self::Event>, Self::Error> {
        match command {
            UserCommand::CreateUser(command) => {
                if let Ok(_) = context
                    .repository
                    .aggregate_exists_by_email(&command.email.to_string())
                    .await
                {
                    return Err(UserCommandError::UserAlreadyExists(
                        "email".into(),
                        command.email.to_string(),
                    ));
                }
                Ok(vec![UserCreatedEvent {
                    name: command.name,
                    email: command.email,
                    ..Default::default()
                }
                .into()])
            }
        }
    }
}

impl ApplyEvent<UserCreatedEvent, UserAggregate> for UserAggregate {
    fn apply(&mut self, event: UserCreatedEvent) {
        self.last_event = Some(event.to_owned().into());
        self.applied_events += 1;
        self.id = Some(event.id);
        self.name = Some(event.name);
        self.email = Some(event.email);
        self.created_at = Some(event.created_at);
        self.updated_at = Some(event.created_at);
    }
}

impl Snapshot<UserAggregate> for UserAggregate {
    fn snapshot(&self) -> Option<AggregateSnapshot<UserAggregate>>
    where
        Self: Sized,
    {
        if self.applied_events >= 10 {
            let event = self.last_event.as_ref().unwrap();
            let snapshot: AggregateSnapshot<Self> = AggregateSnapshot {
                aggregate_id: self.aggregate_id().unwrap(),
                aggregate_type: self.aggregate_type(),
                payload: self.to_owned(),
                last_sequence: event.event_id(),
                snapshot_id: Ulid::new().to_string(),
                timestamp: Utc::now(),
            };
            return Some(snapshot);
        }
        return None;
    }
}

#[async_trait]
impl ApplySnapshot<UserAggregate> for UserAggregate {
    fn apply_snapshot(&mut self, snapshot: AggregateSnapshot<UserAggregate>) {
        let payload = snapshot.payload;
        self.id = payload.id;
        self.name = payload.name;
        self.email = payload.email;
        self.created_at = payload.created_at;
        self.updated_at = payload.updated_at;
        self.last_event = payload.last_event;
    }
}
