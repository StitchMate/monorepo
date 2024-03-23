use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use base::domain::entity::{
    aggregate::Aggregate,
    command::HandleCommand,
    event::{ApplyEvent, EventEnvelope, EventMetadata},
    snapshot::ApplySnapshot,
};
use chrono::Utc;
use user_common::domain::event::UserEvent;

use crate::{
    application::ports::{
        inbound::create_user::CreateUserUseCase, outbound::repository::UserEventRepository,
    },
    domain::user::{
        aggregate::{CommandContext, UserAggregate},
        command::CreateUserCommand,
        error::{
            command::UserCommandError, repository::UserEventRepositoryError,
            service::UserCommandServiceError,
        },
    },
    infrastructure::adapters::outbound::event_repository::UserEventRepositoryEnum,
};

pub trait UserCommandServiceTrait<O: From<UserAggregate>>:
    CreateUserUseCase<O, UserCommandServiceError>
{
}

pub struct UserCommandService {
    user_event_repository: Arc<UserEventRepositoryEnum>,
}

impl UserCommandService {
    pub fn new(user_event_repository: Arc<UserEventRepositoryEnum>) -> Self {
        Self {
            user_event_repository,
        }
    }
    async fn _reconstitute_aggregate(
        &self,
        id: &str,
    ) -> Result<UserAggregate, UserEventRepositoryError> {
        if let Err(e) = self.user_event_repository.aggregate_exists_by_id(id).await {
            return Err(e);
        }
        let mut aggregate = UserAggregate::default();
        let events;
        match self
            .user_event_repository
            .retrieve_latest_snapshot(id.to_string())
            .await?
        {
            Some(x) => {
                aggregate.apply_snapshot(x.clone());
                events = self
                    .user_event_repository
                    .retrieve_events(id.to_string(), Some(x.last_sequence))
                    .await?;
            }
            None => {
                events = self
                    .user_event_repository
                    .retrieve_events(id.to_string(), None)
                    .await?;
            }
        };
        events.into_iter().for_each(|x| match x.payload {
            UserEvent::UserCreated(event) => aggregate.apply(event),
        });
        return Ok(aggregate);
    }
}

#[async_trait]
impl<O> CreateUserUseCase<O, UserCommandServiceError> for UserCommandService
where
    O: From<UserAggregate>,
{
    async fn create_user(
        &self,
        command: CreateUserCommand,
        _fields: Vec<String>,
    ) -> Result<O, UserCommandServiceError> {
        let mut aggregate = UserAggregate::default();
        let events = aggregate
            .handle(
                command.into(),
                Arc::new(CommandContext {
                    repository: self.user_event_repository.clone(),
                }),
            )
            .await
            .map_err(|e| <UserCommandError as Into<UserCommandServiceError>>::into(e))?;
        events.iter().for_each(|event| match event {
            UserEvent::UserCreated(e) => aggregate.apply(e.to_owned()),
        });
        if aggregate.aggregate_id().is_none() {
            return Err(UserCommandError::UnknownError.into());
        }
        let wrapped_events = events
            .iter()
            .map(|x| match x {
                UserEvent::UserCreated(e) => Some(EventEnvelope::<UserEvent> {
                    aggregate_id: aggregate.aggregate_id().unwrap(),
                    aggregate_type: aggregate.aggregate_type(),
                    sequence: e.event_id(),
                    payload: x.to_owned(),
                    metadata: HashMap::new(),
                    timestamp: Utc::now(),
                }),
            })
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect();
        self.user_event_repository
            .store_events(wrapped_events)
            .await
            .map_err(|e| {
                e.into_iter()
                    .map(|ie| <UserEventRepositoryError as Into<UserCommandServiceError>>::into(ie))
                    .collect::<Vec<UserCommandServiceError>>()[0].clone()
            })?;
        return Ok(aggregate.into());
    }
}

impl<O: From<UserAggregate>> UserCommandServiceTrait<O> for UserCommandService {}
