use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Default)]
pub struct TransducerConfig {
    pub topic: String,
    pub durable_name: Option<String>,
}
