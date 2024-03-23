use std::{str::FromStr, sync::Arc};

use anyhow::Result;
use sqlx::{migrate::Migrator, sqlite::SqliteConnectOptions, Pool, SqlitePool};

use crate::{config::repository::SqliteConfig, domain::entity::error::RepositoryError};

#[derive(Debug)]
pub struct SqliteConnector {
    pub pool: SqlitePool,
}

impl SqliteConnector {
    pub async fn new(config: SqliteConfig) -> Result<Arc<Self>> {
        match Pool::connect_with(SqliteConnectOptions::from_str(&config.location).unwrap()).await {
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
