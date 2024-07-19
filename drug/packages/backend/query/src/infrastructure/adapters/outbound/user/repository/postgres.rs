use std::sync::Arc;

use async_trait::async_trait;
use base::config::repository::PostgresConfig;
use base::domain::entity::error::RepositoryError;
use base::infrastructure::adapters::outbound::storage::postgres::PostgresConnector;
use chrono::DateTime;
use chrono::Utc;
#[cfg(test)]
use mockall::automock;
use sqlx::Error;
use sqlx::Postgres;
use tracing::debug;
use user_common::domain::value_object::email::Email;
use user_common::domain::value_object::name::Name;

use crate::{
    application::ports::outbound::repository::UserQueryRepository,
    domain::user::error::repository::UserQueryRepositoryError, domain::user::model::UserQueryModel,
    infrastructure::dtos::user::model::sql::SQLUserQueryModel,
};

const TABLE_NAME: &str = "users";

#[derive(Clone, Debug)]
pub struct PostgresUserQueryRepository {
    pub connector: Arc<PostgresConnector>,
}

impl PostgresUserQueryRepository {
    pub async fn new(config: &PostgresConfig) -> Result<Self, UserQueryRepositoryError> {
        let connector = PostgresConnector::new(config).await.map_err(|e| {
            if e.to_string().contains("authentication") {
                RepositoryError::AuthenticationError(e.to_string())
            } else {
                RepositoryError::ConnectionError(e.to_string())
            }
        })?;
        return Ok(Self {
            connector
        })
    }
    #[cfg(test)]
    async fn _drop(&self) -> Result<(), anyhow::Error> {
        let drop_query = format!("DELETE FROM {}", TABLE_NAME);

        let plan = sqlx::query::<Postgres>(&drop_query);
        let _ = plan.execute(&self.connector.pool).await?;

        Ok(())
    }
}

#[async_trait]
#[cfg_attr(test, automock)]
impl UserQueryRepository for PostgresUserQueryRepository {
    async fn migrate(&self, path: String) -> Result<(), UserQueryRepositoryError> {
        return self.connector.migrate(path).await.map_err(|e| e.into());
    }

    // TODO: Add field selection, which probably requires maintaining a map here or on the SQL DTO for mapping from the
    // requested field to the correct field.
    async fn get_by_id(
        &self,
        id: &str,
        _fields: Vec<String>,
    ) -> Result<UserQueryModel, UserQueryRepositoryError> {
        let query = format!("SELECT * FROM {} u WHERE u.id = $1 LIMIT 1", TABLE_NAME);
        debug!(query = query, "performing postgres query");
        let mut plan = sqlx::query_as::<Postgres, SQLUserQueryModel>(&query);
        plan = plan.bind(&id);
        let result = plan.fetch_one(&self.connector.pool).await;
        debug!(result = ?result, "received result from postgres");
        return result
            .map(|x| x.into())
            .map_err(|e| {
                match e {
                    Error::RowNotFound => UserQueryRepositoryError::UserNotFound("id".into(), id.into()).into(),
                    _ => RepositoryError::RetrieveError(e.to_string()).into()
                }
            } );
    }

    async fn create(
        &self,
        id: &str,
        name: &Name,
        email: &Email,
        created_at: &DateTime<Utc>,
    ) -> Result<(), UserQueryRepositoryError> {
        let user_fields = vec![
            "id",
            "first_name",
            "last_name",
            "email",
            "created_at",
            "updated_at",
        ];
        let user_placeholders: Vec<String> = (0..user_fields.len())
            .map(|x| format!("${}", (x + 1).to_string()))
            .collect();
        let user_placeholder_str = user_placeholders.join(", ");
        let user_insert_query = format!(
            "INSERT INTO {} ({}) VALUES ( {} )",
            TABLE_NAME,
            user_fields.join(", "),
            user_placeholder_str
        );
        let user_insert_plan = sqlx::query::<Postgres>(&user_insert_query);
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
            return Err(user_insert
                .map_err(|e| RepositoryError::PersistError(e.to_string()).into())
                .unwrap_err());
        }
        return Ok(());
    }
}
