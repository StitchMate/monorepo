use async_trait::async_trait;

use crate::domain::package::{model::PackagesQueryModel, query::PackageByIdQuery};

//TODO: Concerete Error Types
#[async_trait]
pub trait GetPackageByIdUseCase<O, E>
where
    O: From<PackagesQueryModel>
{
    async fn get_package_by_id(&self, query: PackageByIdQuery, fields: Vec<String>) -> Result<O, E>;
}
