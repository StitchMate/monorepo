use async_trait::async_trait;

use crate::domain::subsidiaries::{model::SubsidiariesQueryModel, query::SubsidiariesByIdQuery};

//TODO: Concerete Error Types
#[async_trait]
pub trait GetSubsidiariesByIdUseCase<O, E>
where
    O: From<SubsidiariesQueryModel>,
    E: std::error::Error,
{
    async fn get_subsidiaries_by_id(&self, query: SubsidiariesByIdQuery, fields: Vec<String>) -> Result<O, E>;
}
