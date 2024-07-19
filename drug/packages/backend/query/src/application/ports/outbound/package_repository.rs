use async_trait::async_trait;
use chrono::{DateTime, Utc};
use enum_dispatch::enum_dispatch;
use mockall::automock;
use user_common::domain::value_object::quantity::Quantity;
use crate::domain::contains::model::ContainsQueryModel;
use crate::domain::package::error::repository::PackageQueryRepositoryError;
use crate::domain::package::model::PackagesQueryModel;


//TODO: Concrete Error Types
#[async_trait]
#[enum_dispatch]
pub trait PackageQueryRepository {
    async fn migrate(&self, path: String) -> Result<(), PackageQueryRepositoryError>;
    async fn get_by_id(
        &self,
        id: &str,
        fields: Vec<String>,
    ) -> Result<PackagesQueryModel, PackageQueryRepositoryError>;
    // async fn get_by_package_ndc(
    //     &self,
    //     package_ndc: &str,
    //     fields: Vec<String>,
    // ) -> Result<PackagesQueryModel, PackageQueryRepositoryError>;
    // async fn create(
    //     id: &str,
    //     package_ndc: &str,
    //     quantity: &Quantity,
    //     contains: &Vec<ContainsQueryModel>,
    //     created_at: &DateTime<Utc>,
    // ) -> Result<(), PackageQueryRepositoryError>;
    // async fn update(
    //     &self,
    //     id: &str,
    //     package_ndc: &str,
    //     quantity: &Quantity,
    //     contains: &Vec<ContainsQueryModel>,
    //     updated_at: &DateTime<Utc>,
    // ) -> Result<(), PackageQueryRepositoryError>;
}
