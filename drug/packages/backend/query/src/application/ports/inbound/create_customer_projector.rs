use async_trait::async_trait;
use chrono::{DateTime, Utc};
use user_common::domain::value_object::email::Email;
use user_common::domain::value_object::name::Name;

#[async_trait]
pub trait CreateUserFromProjectorUseCase<E>
where
    E: std::error::Error,
{
    async fn create_user(
        &self,
        id: &str,
        name: &Name,
        email: &Email,
        created_at: &DateTime<Utc>,
    ) -> Result<(), E>;
}
