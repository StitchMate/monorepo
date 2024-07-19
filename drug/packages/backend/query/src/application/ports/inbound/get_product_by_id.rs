use async_trait::async_trait;

use crate::domain::product::{model::ProductQueryModel, query::ProductByIdQuery};

//TODO: Concerete Error Types
#[async_trait]
pub trait GetProductByIdUseCase<O, E>
where
    O: From<ProductQueryModel>,
    E: std::error::Error,
{
    async fn get_product_by_id(&self, query: ProductByIdQuery, fields: Vec<String>) -> Result<O, E>;
}
