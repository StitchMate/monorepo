use std::sync::Arc;

use async_trait::async_trait;
use base::infrastructure::adapters::outbound::storage::postgres::PostgresConnector;
use mockall::automock;
use crate::application::ports::outbound::package_repository::PackageQueryRepository;
use crate::domain::package::error::repository::PackageQueryRepositoryError;
use crate::domain::package::model::PackagesQueryModel;

const TABLE_NAME: &str = "packages";

#[derive(Clone, Debug)]
pub struct PostgresPackagesQueryRepository {
    pub connector: Arc<PostgresConnector>,
}

#[async_trait]
#[automock]
impl PackageQueryRepository for PostgresPackagesQueryRepository {
    async fn migrate(&self, path: String) -> Result<(), PackageQueryRepositoryError> {
        return self.connector.migrate(path).await.map_err(|e| e.into());
    }

    // TODO: Add field selection, which probably requires maintaining a map here or on the SQL DTO for mapping from the
    // requested field to the correct field.
    async fn get_by_id(
        &self,
        id: &str,
        _fields: Vec<String>,
    ) -> Result<PackagesQueryModel, PackageQueryRepositoryError> {
        return Err(PackageQueryRepositoryError::UnknownError)
    }
}
