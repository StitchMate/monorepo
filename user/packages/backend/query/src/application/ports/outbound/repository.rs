use async_trait::async_trait;
use chrono::{DateTime, Utc};
use enum_dispatch::enum_dispatch;
use user_common::domain::value_object::{email::Email, name::Name};

use crate::domain::user::error::repository::UserQueryRepositoryError;
use crate::domain::user::model::UserQueryModel;

//TODO: Concrete Error Types
#[async_trait]
#[enum_dispatch]
pub trait UserQueryRepository {
    async fn migrate(&self, path: String) -> Result<(), UserQueryRepositoryError>;
    async fn get_by_id(
        &self,
        id: &str,
        fields: Vec<String>,
    ) -> Result<UserQueryModel, UserQueryRepositoryError>;
    async fn create(
        &self,
        id: &str,
        name: &Name,
        email: &Email,
        created_at: &DateTime<Utc>,
    ) -> Result<(), UserQueryRepositoryError>;
}
