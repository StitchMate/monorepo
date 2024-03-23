use serde::{Deserialize, Serialize};
use valuable::Valuable;

#[derive(Debug, Clone, Serialize, Deserialize, Valuable)]
pub struct NATSConfig {
    pub address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Valuable)]
#[serde(tag = "type", content = "config")]
#[serde(rename_all = "lowercase")]
pub enum EventBusBackend {
    NATS(NATSConfig),
}

#[derive(Debug, Clone, Deserialize, Serialize, Valuable)]
pub struct EventBusConfig {
    #[serde(flatten)]
    pub backend: EventBusBackend,
}
