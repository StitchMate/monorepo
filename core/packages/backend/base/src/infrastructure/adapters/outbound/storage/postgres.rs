use std::{str::FromStr, sync::Arc};

use anyhow::Result;
use sqlx::{migrate::Migrator, postgres::PgConnectOptions, PgPool, Pool};
use tracing::info;

use crate::{config::repository::PostgresConfig, domain::entity::error::RepositoryError};

#[derive(Debug)]
pub struct PostgresConnector {
    pub pool: PgPool,
}

impl PostgresConnector {
    pub async fn new(config: &PostgresConfig) -> Result<Arc<Self>> {
        info!(connection_string = &config.to_connection_string(), "using connection string");
        match Pool::connect_with(
            PgConnectOptions::from_str(&config.to_connection_string()).unwrap(),
        )
        .await
        {
            Ok(x) => {
                let ret = Arc::new(Self { pool: x });
                return Ok(ret);
            }
            Err(e) => return Err(e.into()),
        }
    }
    pub async fn migrate(&self, path: String) -> Result<(), RepositoryError> {
        let m = Migrator::new(std::path::Path::new(&path)).await?;
        m.run(&self.pool).await.map_err(|e| e.into())
    }
}
