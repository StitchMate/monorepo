use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisConfig {
    pub address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "config")]
#[serde(rename_all = "lowercase")]
pub enum LockManagerBackend {
    Redis(RedisConfig),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LockManagerConfig {
    #[serde(flatten)]
    pub backend: LockManagerBackend,
}
