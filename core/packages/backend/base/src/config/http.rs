use serde::Deserialize;
use valuable::Valuable;

#[derive(Debug, Deserialize, Clone, Default, Valuable)]
pub struct HTTPServerConfig {
    pub port: u16,
    pub enabled: Option<bool>
}
