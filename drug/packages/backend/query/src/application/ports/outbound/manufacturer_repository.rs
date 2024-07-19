use async_trait::async_trait;
use chrono::{DateTime, Utc};
use enum_dispatch::enum_dispatch;
use crate::domain::manufacturer::error::repository::ManufacturerQueryRepositoryError;
use crate::domain::manufacturer::model::ManufacturerQueryModel;
use crate::domain::subsidiaries::model::SubsidiariesQueryModel;

//TODO: Concrete Error Types
#[async_trait]
#[enum_dispatch]
pub trait ManufacturerQueryRepository {
    async fn migrate(&self, path: String) -> Result<(), ManufacturerQueryRepositoryError>;
    async fn get_by_id(
        &self,
        id: &str,
        fields: Vec<String>,
    ) -> Result<ManufacturerQueryModel, ManufacturerQueryRepositoryError>;
    async fn create(
        &self,
        id: &str,
        name: &str,
        subsidiaries: &Vec<SubsidiariesQueryModel>,
        created_at: &DateTime<Utc>,
    ) -> Result<(), ManufacturerQueryRepositoryError>;
    async fn update(
        &self,
        id: &str,
        name: &str,
        subsidiaries: &Vec<SubsidiariesQueryModel>,
        updated_at: &DateTime<Utc>,
    ) -> Result<(), ManufacturerQueryRepositoryError>;
}
