use std::sync::Arc;

use anyhow::anyhow;
use async_trait::async_trait;
use base::infrastructure::adapters::outbound::storage::sqlite::SqliteConnector;
use chrono::DateTime;
use chrono::Utc;
use sqlx::Sqlite;
use user_common::domain::value_object::email::Email;
use user_common::domain::value_object::name::Name;

use crate::{
    application::ports::outbound::repository::UserQueryRepository,
    domain::user::model::UserQueryModel, infrastructure::dtos::user::model::sql::SQLUserQueryModel,
};

const TABLE_NAME: &str = "users";

#[derive(Clone, Debug)]
pub struct SQLiteUserQueryRepository {
    pub connector: Arc<SqliteConnector>,
}

impl SQLiteUserQueryRepository {
    #[cfg(test)]
    async fn _drop(&self) -> Result<(), anyhow::Error> {
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
    async fn get_by_id(
        &self,
        id: &str,
        _fields: Vec<String>,
    ) -> Result<UserQueryModel, anyhow::Error> {
        let query = format!("SELECT * FROM {} u WHERE u.id = ?1 LIMIT 1", TABLE_NAME);
        let mut plan = sqlx::query_as::<Sqlite, SQLUserQueryModel>(&query);
        plan = plan.bind(&id);
        let result = plan.fetch_one(&self.connector.pool).await;
        return result.map(|x| x.into()).map_err(|e| anyhow!(e));
    }

    async fn create(
        &self,
        id: &str,
        name: &Name,
        email: &Email,
        created_at: &DateTime<Utc>,
    ) -> Result<(), anyhow::Error> {
        let user_fields = vec![
            "id",
            "first_name",
            "last_name",
            "email",
            "created_at",
            "updated_at",
        ];
        let user_placeholders: Vec<String> = (0..user_fields.len())
            .map(|x| format!("?{}", (x + 1).to_string()))
            .collect();
        let user_placeholder_str = user_placeholders.join(", ");
        let user_insert_query = format!(
            "INSERT INTO {} ({}) VALUES ( {} )",
            TABLE_NAME,
            user_fields.join(", "),
            user_placeholder_str
        );
        let user_insert_plan = sqlx::query::<Sqlite>(&user_insert_query);
        let user_insert = user_insert_plan
            .bind(&id)
            .bind(&name.first)
            .bind(&name.last)
            .bind(&email.to_string())
            .bind(&created_at)
            .bind(&created_at)
            .execute(&self.connector.pool)
            .await;
        if user_insert.is_err() {
            return Err(user_insert.unwrap_err().into());
        }
        return Ok(());
    }
}
