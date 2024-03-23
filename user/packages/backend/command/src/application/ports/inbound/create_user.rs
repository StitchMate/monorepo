use async_trait::async_trait;

use crate::domain::user::{aggregate::UserAggregate, command::CreateUserCommand};

//TODO: Concerete Error Types
#[async_trait]
pub trait CreateUserUseCase<O, E>
where
    O: From<UserAggregate>,
    E: std::error::Error,
{
    async fn create_user(
        &self,
        command: CreateUserCommand,
        fields: Vec<String>,
    ) -> Result<O, E>;
}
