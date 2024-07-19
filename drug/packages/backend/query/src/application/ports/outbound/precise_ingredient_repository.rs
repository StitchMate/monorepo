use async_trait::async_trait;
use chrono::{DateTime, Utc};
use enum_dispatch::enum_dispatch;

use crate::domain::ingredient::model::IngredientQueryModel;
use crate::domain::manufacturer::model::ManufacturerQueryModel;
use crate::domain::package::model::PackagesQueryModel;
use crate::domain::precise_ingredient::error::repository::PreciseIngredientQueryRepositoryError;
use crate::domain::precise_ingredient::model::PreciseIngredientQueryModel;

//TODO: Concrete Error Types
#[async_trait]
#[enum_dispatch]
pub trait PreciseIngredientQueryRepository {
    async fn migrate(&self, path: String) -> Result<(), PreciseIngredientQueryRepositoryError>;
    async fn get_by_id(
        &self,
        id: &str,
        fields: Vec<String>,
    ) -> Result<PreciseIngredientQueryModel, PreciseIngredientQueryRepositoryError>;
    async fn get_by_anda(
        &self,
        anda: &str,
        fields: Vec<String>,
    ) -> Result<PreciseIngredientQueryModel, PreciseIngredientQueryRepositoryError>;
    async fn create(
        id: &str,
        name: &str,
        anda: &str,
        ingredients: &Vec<IngredientQueryModel>,
        manufacturer: &ManufacturerQueryModel,
        name: &str, // this is generic name that is country based
        brand_name: &str,
        product_ndc: &str,
        product_type: &str,
        packages: &Vec<PackagesQueryModel>,
        form: &str,
        route: &str,
        schedule: &str,
        anda: &str,
        spl_set_id: &str,
        created_at: &DateTime<Utc>,
    ) -> Result<(), PreciseIngredientQueryRepositoryError>;
    async fn update(
        &self,
        id: &str,
        name: &str,
        ingredients: &Vec<IngredientQueryModel>,
        manufacturer: &ManufacturerQueryModel,
        name: &str, // this is generic name that is country based
        brand_name: &str,
        product_ndc: &str,
        product_type: &str,
        packages: &Vec<PackagesQueryModel>,
        form: &str,
        route: &str,
        schedule: &str,
        anda: &str,
        spl_set_id: &str,
        updated_at: &DateTime<Utc>,
    ) -> Result<(), PreciseIngredientQueryRepositoryError>;
}
