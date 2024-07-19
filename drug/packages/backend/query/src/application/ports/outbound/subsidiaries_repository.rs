use async_trait::async_trait;
use chrono::{DateTime, Utc};
use enum_dispatch::enum_dispatch;

use crate::domain::subsidiaries::{error::repository::SubsidiariesQueryRepositoryError, model::SubsidiariesQueryModel};


//TODO: Concrete Error Types
#[async_trait]
#[enum_dispatch]
pub trait SubsidiariesQueryRepository {
    async fn migrate(&self, path: String) -> Result<(), SubsidiariesQueryRepositoryError>;
    async fn get_by_id(
        &self,
        id: &str,
        fields: Vec<String>,
    ) -> Result<SubsidiariesQueryModel, SubsidiariesQueryRepositoryError>;
    async fn create(
        id: &str,
        name: &str,
        company_type: &Vec<&str>,
        created_at: &DateTime<Utc>,
    ) -> Result<(), SubsidiariesQueryRepositoryError>;
    async fn update(
        &self,
        id: &str,
        name: &str,
        company_type: &Vec<&str>,
        updated_at: &DateTime<Utc>,
    ) -> Result<(), SubsidiariesQueryRepositoryError>;
}
