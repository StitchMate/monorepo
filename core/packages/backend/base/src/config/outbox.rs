use serde::Deserialize;
use valuable::Valuable;

#[derive(Deserialize, Debug, Clone, Valuable)]
pub struct Timeout(u32);

impl Timeout {
    pub fn value(&self) -> u32 {
        return self.0
    }
}

impl Default for Timeout {
    fn default() -> Self {
        Timeout(3)
    }
}

#[derive(Debug, Deserialize, Clone, Default, Valuable)]
pub struct OutboxConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub timeout_seconds: Timeout,
}
