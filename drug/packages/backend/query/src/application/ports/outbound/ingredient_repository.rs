use async_trait::async_trait;
use chrono::{DateTime, Utc};
use enum_dispatch::enum_dispatch;
use crate::domain::ingredient::error::repository::IngredientQueryRepositoryError;
use crate::domain::ingredient::model::IngredientQueryModel;
use crate::domain::quantity::model::QuantityQueryModel;

//TODO: Concrete Error Types
#[async_trait]
#[enum_dispatch]
pub trait IngredientQueryRepository {
    async fn migrate(&self, path: String) -> Result<(), IngredientQueryRepositoryError>;
    async fn get_by_id(
        &self,
        id: &str,
        fields: Vec<String>,
    ) -> Result<IngredientQueryModel, IngredientQueryRepositoryError>;
    async fn get_by_unii(
        &self,
        unii: &str,
        fields: Vec<String>,
    ) -> Result<IngredientQueryModel, IngredientQueryRepositoryError>;
    async fn create(
        &self,
        id: &str,
        name: &str,
        unii: &str,
        ingredient_type: &str,
        quantity: &QuantityQueryModel,
        ingredient_type: &str,
        ingredient_type: &str,
        created_at: &DateTime<Utc>,
    ) -> Result<(), IngredientQueryRepositoryError>;
    async fn update(
        &self,
        id: &str,
        name: &str,
        unii: &str,
        ingredient_type: &str,
        quantity: &QuantityQueryModel,
        ingredient_type: &str,
        ingredient_type: &str,
        updated_at: &DateTime<Utc>,
    ) -> Result<(), IngredientQueryRepositoryError>;
}
