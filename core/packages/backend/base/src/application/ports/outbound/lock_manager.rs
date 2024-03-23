use std::time::Duration;

use async_trait::async_trait;

use crate::domain::entity::error::LockManagerError;

#[async_trait]
pub trait LockManager {
    async fn lock(
        &self,
        aggregate_id: &str,
        ttl: Option<i32>,
        attempt: Option<i32>,
        timeout: Option<Duration>,
    ) -> Result<(), LockManagerError>;
    async fn unlock(&self, aggregate_id: &str) -> Result<(), LockManagerError>;
}
