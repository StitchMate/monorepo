use std::sync::Arc;

use async_trait::async_trait;
use tracing::debug;

use crate::{application::ports::inbound::get_package_by_id::GetPackageByIdUseCase, domain::package::{model::PackagesQueryModel, query::PackageByIdQuery}, infrastructure::adapters::outbound::package::repository::PackageQueryRepositoryEnum};
use crate::application::ports::outbound::package_repository::PackageQueryRepository;

pub trait PackageQueryServiceTrait<O: From<PackagesQueryModel>>:
    GetPackageByIdUseCase<O, anyhow::Error>
{
}

pub struct PackageQueryService {
    package_repository: Arc<PackageQueryRepositoryEnum>
}

impl PackageQueryService {
    pub fn new(package_repository: Arc<PackageQueryRepositoryEnum>) -> Self {
        Self { package_repository }
    }
}

#[async_trait]
impl<O> GetPackageByIdUseCase<O, anyhow::Error> for PackageQueryService
where
    O: From<PackagesQueryModel>,
{
    async fn get_package_by_id(
        &self,
        query: PackageByIdQuery,
        fields: Vec<String>,
    ) -> Result<O, anyhow::Error> {
        self.package_repository
            .get_by_id(&query.id, fields)
            .await
            .map_err(|e| {
                e.into()
            })
            .map(|x| x.into())
    }
}
impl<O: From<PackagesQueryModel>> PackageQueryServiceTrait<O> for PackageQueryService {}
