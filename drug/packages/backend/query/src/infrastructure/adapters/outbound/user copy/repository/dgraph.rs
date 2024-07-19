use async_trait::async_trait;
use base::config::repository::DgraphConfig;
use base::domain::entity::error::RepositoryError;
use chrono::DateTime;
use chrono::Utc;
#[cfg(test)]
use mockall::automock;
use tonic::transport::Channel;
use user_common::domain::value_object::email::Email;
use user_common::domain::value_object::name::Name;

use crate::{
    application::ports::outbound::repository::UserQueryRepository,
    domain::user::error::repository::UserQueryRepositoryError, domain::user::model::UserQueryModel,
    infrastructure::dtos::user::model::sql::SQLUserQueryModel,
};

const TABLE_NAME: &str = "users";

#[derive(Clone, Debug)]
pub struct DgraphUserQueryRepository {
    pub client: Arc<DgraphClient<Channel>>,
}

impl DgraphUserQueryRepository {
    pub async fn new(config: &DgraphConfig) -> Result<Self, UserQueryRepositoryError> {
        let client = DgraphClient::new(vec![config.url.clone()]).await.map_err(|e| {
            if e.to_string().contains("authentication") {
                RepositoryError::AuthenticationError(e.to_string())
            } else {
                RepositoryError::ConnectionError(e.to_string())
            }
        })?;
        return Ok(Self {
            client: Arc::new(client)
        })
    }

    #[cfg(test)]
    async fn _drop(&self) -> Result<(), anyhow::Error> {
        let drop_query = format!("{{ delete {{ <_uid_> * * . }} }}");
        let _ = self.client.query(drop_query).await?;
        Ok(())
    }
}

#[async_trait]
#[cfg_attr(test, automock)]
impl UserQueryRepository for DgraphUserQueryRepository {
    async fn migrate(&self, path: String) -> Result<(), UserQueryRepositoryError> {
        // Dgraph migration logic here (if needed)
        Ok(())
    }

    async fn get_by_id(
        &self,
        id: &str,
        _fields: Vec<String>,
    ) -> Result<UserQueryModel, UserQueryRepositoryError> {
        let query = format!(r#"
            {{
                getUser(func: eq(id, "{}")) {{
                    uid
                    id
                    first_name
                    last_name
                    email
                    created_at
                    updated_at
                }}
            }}"#, id);
        let response = self.client.query(query).await.map_err(|e| {
            RepositoryError::RetrieveError(e.to_string()).into()
        })?;
        
        let user_data = response.json().map_err(|e| {
            RepositoryError::RetrieveError(e.to_string()).into()
        })?;
        
        let user: SQLUserQueryModel = serde_json::from_value(user_data).map_err(|e| {
            RepositoryError::RetrieveError(e.to_string()).into()
        })?;
        
        Ok(user.into())
    }

    async fn create(
        &self,
        id: &str,
        name: &Name,
        email: &Email,
        created_at: &DateTime<Utc>,
    ) -> Result<(), UserQueryRepositoryError> {
        let user_insert_query = format!(r#"
            {{
                set {{
                    _:user <id> "{}" .
                    _:user <first_name> "{}" .
                    _:user <last_name> "{}" .
                    _:user <email> "{}" .
                    _:user <created_at> "{}" .
                    _:user <updated_at> "{}" .
                }}
            }}"#, id, name.first, name.last, email.to_string(), created_at, created_at);
        
        let response = self.client.query(user_insert_query).await.map_err(|e| {
            RepositoryError::PersistError(e.to_string()).into()
        })?;
        
        Ok(())
    }
}
