use std::sync::Arc;

use async_trait::async_trait;
use chrono::DateTime;
use chrono::Utc;
use user_common::domain::value_object::email::Email;
use user_common::domain::value_object::name::Name;

use crate::application::ports::inbound::create_customer_projector::CreateUserFromProjectorUseCase;
use crate::application::ports::outbound::repository::UserQueryRepository;
use crate::domain::user::error::service::UserProjectorServiceError;
use crate::infrastructure::adapters::outbound::user::repository::UserQueryRepositoryEnum;

pub trait UserProjectorServiceTrait:
    CreateUserFromProjectorUseCase<UserProjectorServiceError>
{
}

#[derive(Clone)]
pub struct UserProjectorService {
    user_repository: Arc<UserQueryRepositoryEnum>,
}

impl UserProjectorService {
    pub fn new(user_repository: Arc<UserQueryRepositoryEnum>) -> Self {
        Self { user_repository }
    }
}

#[async_trait]
impl CreateUserFromProjectorUseCase<UserProjectorServiceError> for UserProjectorService {
    async fn create_user(
        &self,
        id: &str,
        name: &Name,
        email: &Email,
        created_at: &DateTime<Utc>,
    ) -> Result<(), UserProjectorServiceError> {
        return self
            .user_repository
            .create(id, name, email, created_at)
            .await
            .map_err(|e| e.into());
    }
}

impl UserProjectorServiceTrait for UserProjectorService {}
