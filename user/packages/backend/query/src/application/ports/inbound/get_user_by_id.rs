use async_trait::async_trait;

use crate::domain::user::{model::UserQueryModel, query::UserByIdQuery};

//TODO: Concerete Error Types
#[async_trait]
pub trait GetUserByIdUseCase<O, E>
where
    O: From<UserQueryModel>,
    E: std::error::Error,
{
    async fn get_user_by_id(&self, query: UserByIdQuery, fields: Vec<String>) -> Result<O, E>;
}
