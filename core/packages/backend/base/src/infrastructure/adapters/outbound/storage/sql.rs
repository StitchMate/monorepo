use sqlx::migrate::MigrateError;

use crate::domain::entity::error::RepositoryError;

impl From<MigrateError> for RepositoryError {
    fn from(value: MigrateError) -> Self {
        RepositoryError::MigrationError(value.to_string())
    }
}