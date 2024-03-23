use serde::Deserialize;
use valuable::Valuable;

#[derive(Debug, Deserialize, Clone, Default, Valuable)]
pub struct ProjectorConfig {
    pub topic: String,
    pub durable_name: String,
}
