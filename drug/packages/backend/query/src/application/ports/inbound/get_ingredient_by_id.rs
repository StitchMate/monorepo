use async_trait::async_trait;

use crate::domain::ingredient::{model::IngredientQueryModel, query::IngredientByIdQuery};

//TODO: Concerete Error Types
#[async_trait]
pub trait GetIngredientByIdUseCase<O, E>
where
    O: From<IngredientQueryModel>,
    E: std::error::Error,
{
    async fn get_ingredient_by_id(&self, query: IngredientByIdQuery, fields: Vec<String>) -> Result<O, E>;
}
