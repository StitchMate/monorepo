use async_trait::async_trait;
use deadpool_redis::{redis::AsyncCommands, Config, Pool, PoolError, Runtime};
use std::{sync::Arc, time::Duration};
use tokio::time::sleep;
use tracing::debug;

use crate::{
    application::ports::outbound::lock_manager::LockManager,
    domain::entity::error::LockManagerError,
};

pub struct RedisLockManager {
    pool: Arc<Pool>,
}

impl RedisLockManager {
    pub fn new(address: String) -> Result<Self, anyhow::Error> {
        let cfg = Config::from_url(address);
        let pool = cfg.create_pool(Some(Runtime::Tokio1))?;
        return Ok(Self {
            pool: Arc::new(pool),
        });
    }
}

#[async_trait]
impl LockManager for RedisLockManager {
    async fn lock(
        &self,
        aggregate_id: &str,
        ttl: Option<i32>,
        attempt: Option<i32>,
        timeout: Option<Duration>,
    ) -> Result<(), LockManagerError> {
        let mut conn = self.pool.clone().get().await.map_err(|e| match e {
            PoolError::Backend(e) => LockManagerError::ConnectionError(e.to_string()),
            _ => LockManagerError::UnknownError,
        })?;
        let exists: bool = conn
            .exists(aggregate_id)
            .await
            .map_err(|e| LockManagerError::ExistsError(e.to_string()))?;

        if exists {
            if attempt == Some(3) {
                return Err(LockManagerError::LockError("failed to acquire lock".into()));
            }
            sleep(timeout.unwrap_or(Duration::from_millis(25))).await;
            return self
                .lock(
                    aggregate_id,
                    ttl,
                    attempt.map_or(Some(1), |v| Some(v + 1)),
                    timeout,
                )
                .await;
        }
        let _: () = conn
            .set_ex(aggregate_id, true, ttl.unwrap_or(5) as usize)
            .await
            .map_err(|e| LockManagerError::LockError(e.to_string()))?;
        debug!(aggregate_id, "set lock");
        return Ok(());
    }
    async fn unlock(&self, aggregate_id: &str) -> Result<(), LockManagerError> {
        let mut conn = self.pool.clone().get().await.map_err(|e| match e {
            PoolError::Backend(e) => LockManagerError::ConnectionError(e.to_string()),
            _ => LockManagerError::UnknownError,
        })?;
        let _: () = conn
            .pset_ex(aggregate_id, true, 1 as usize)
            .await
            .map_err(|e| LockManagerError::UnlockError(e.to_string()))?;
        debug!(aggregate_id, "remove lock");
        return Ok(());
    }
}
