use std::sync::Arc;

use async_trait::async_trait;
use base::infrastructure::adapters::outbound::storage::sqlite::SqliteConnector;
use anyhow::anyhow;
use sqlx::Sqlite;

use crate::{application::ports::outbound::repository::UserQueryRepository, domain::model::UserQueryModel, infrastructure::dtos::sql::SQLUserQueryModel};

const TABLE_NAME: &str = "users";

#[derive(Clone, Debug)]
pub struct SQLiteUserQueryRepository {
    pub connector: Arc<SqliteConnector>
}

impl SQLiteUserQueryRepository {
    #[cfg(test)]
    async fn drop(&self) -> Result<(), anyhow::Error> {
        let drop_query = format!("DELETE FROM {}", TABLE_NAME);

        let plan = sqlx::query::<Sqlite>(&drop_query);
        let _ = plan.execute(&self.connector.pool).await?;

        Ok(())
    }
}

#[async_trait]
impl UserQueryRepository for SQLiteUserQueryRepository {
    async fn migrate(&self, path: String) -> Result<(), anyhow::Error> {
        return self.connector.migrate(path).await.map_err(|x| anyhow!(x));
    }

    // TODO: Add field selection, which probably requires maintaining a map here or on the SQL DTO for mapping from the
    // requested field to the correct field.
    async fn get_by_id(&self, id: &str, _fields:Vec<String>) ->  Result<UserQueryModel, anyhow::Error> {
        let query = format!(
            "SELECT * FROM {} u WHERE u.id = ?1 LIMIT 1",
            TABLE_NAME
        );
        let mut plan = sqlx::query_as::<Sqlite, SQLUserQueryModel>(&query);
        plan = plan.bind(&id);
        let result = plan.fetch_one(&self.connector.pool).await;
        return result.map(|x| x.into()).map_err(|e| anyhow!(e));
    }
}