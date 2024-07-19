use async_trait::async_trait;

use crate::domain::precise_ingredient::{model::PreciseIngredientQueryModel, query::PreciseIngredientByIdQuery};

//TODO: Concerete Error Types
#[async_trait]
pub trait GetPreciseIngredientByIdUseCase<O, E>
where
    O: From<PreciseIngredientQueryModel>,
    E: std::error::Error,
{
    async fn get_precise_ingredient_by_id(&self, query: PreciseIngredientByIdQuery, fields: Vec<String>) -> Result<O, E>;
}
