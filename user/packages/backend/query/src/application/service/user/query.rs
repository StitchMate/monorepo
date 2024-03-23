use std::sync::Arc;

use async_trait::async_trait;
use tracing::debug;

use crate::{
    application::ports::{
        inbound::get_user_by_id::GetUserByIdUseCase, outbound::repository::UserQueryRepository,
    },
    domain::user::error::service::UserQueryServiceError,
    domain::user::{model::UserQueryModel, query::UserByIdQuery},
    infrastructure::adapters::outbound::user::repository::UserQueryRepositoryEnum,
};

pub trait UserQueryServiceTrait<O: From<UserQueryModel>>:
    GetUserByIdUseCase<O, UserQueryServiceError>
{
}

pub struct UserQueryService {
    user_repository: Arc<UserQueryRepositoryEnum>,
}

impl UserQueryService {
    pub fn new(user_repository: Arc<UserQueryRepositoryEnum>) -> Self {
        Self { user_repository }
    }
}

#[async_trait]
impl<O> GetUserByIdUseCase<O, UserQueryServiceError> for UserQueryService
where
    O: From<UserQueryModel>,
{
    async fn get_user_by_id(
        &self,
        query: UserByIdQuery,
        fields: Vec<String>,
    ) -> Result<O, UserQueryServiceError> {
        self.user_repository
            .get_by_id(&query.id, fields)
            .await
            .map_err(|e| {
                e.into()
            })
            .map(|x| x.into())
    }
}
impl<O: From<UserQueryModel>> UserQueryServiceTrait<O> for UserQueryService {}
