use std::sync::Arc;
use async_trait::async_trait;
use dgraph_tonic::Channel;
use dgraph_tonic::Client;
use crate::application::ports::outbound::package_repository::PackageQueryRepository;
use crate::domain::package::error::repository::PackageQueryRepositoryError;
use crate::domain::package::model::PackagesQueryModel;

const TABLE_NAME: &str = "users";

#[derive(Clone, Debug)]
pub struct DgraphPackageQueryRepository {
    pub client: Arc<Client>,
}

impl DgraphPackageQueryRepository {
    pub async fn new() -> Result<Self, PackageQueryRepositoryError> {
        //TODO: Make a client
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
impl PackageQueryRepository for DgraphUserQueryRepository {
    async fn migrate(&self, path: String) -> Result<(), PackageQueryRepositoryError> {
        // Dgraph migration logic here (if needed)
        Ok(())
    }

    async fn get_by_id(
        &self,
        id: &str,
        _fields: Vec<String>,
    ) -> Result<PackagesQueryModel, PackageQueryRepositoryError> {
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
}