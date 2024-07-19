use async_trait::async_trait;

use crate::domain::manufacturer::{model::ManufacturerQueryModel, query::ManufacturerByIdQuery};

//TODO: Concerete Error Types
#[async_trait]
pub trait GetManufacturerByIdUseCase<O, E>
where
    O: From<ManufacturerQueryModel>,
    E: std::error::Error,
{
    async fn get_manufacturer_by_id(&self, query: ManufacturerByIdQuery, fields: Vec<String>) -> Result<O, E>;
}
