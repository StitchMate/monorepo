use std::sync::Arc;

use base::config::event_bus::EventBusConfig;
use base::config::http::HTTPServerConfig;
use base::config::projector::ProjectorConfig;
use base::{config::repository::RepositoryConfig, domain::entity::error};
use config::{Config, Environment, File};
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use tracing::{info, error};
use tracing::field::valuable;
use valuable::Valuable;

static INSTANCE: OnceCell<Arc<AppConfig>> = OnceCell::new();

#[derive(Debug, Deserialize, Valuable, Clone)]
pub struct UserQueryAdapters {
    pub graphql: Arc<Option<HTTPServerConfig>>,
    pub rest: Arc<Option<HTTPServerConfig>>,
}

#[derive(Debug, Deserialize, Valuable, Clone)]
pub struct UserQueryProjectors {
    pub user: Option<ProjectorConfig>,
}


#[derive(Debug, Deserialize, Valuable, Clone)]
pub struct AppConfig {
    pub repository: Arc<RepositoryConfig>,
    pub adapters: Arc<Option<UserQueryAdapters>>,
    pub projectors: Arc<Option<UserQueryProjectors>>,
    pub eventbus: Arc<Option<EventBusConfig>>
}

impl AppConfig {
    pub fn new(file_name: Option<&String>) -> Result<Arc<Self>, ()> {
        match INSTANCE.get() {
            Some(x) => Ok(x.clone()),
            None => {
                let mut s = Config::builder();

                if file_name.is_some() {
                    s = s.add_source(File::with_name(file_name.unwrap().as_str()))
                }

                s = s.add_source(Environment::with_prefix("sm").separator("__"));

                let config = s
                    .build()
                    .map_err(|e| error!(error = format!("{}", e), "configuration error"))?;

                config
                    .try_deserialize()
                    .map(|x| {
                        let ret: Arc<Self> = Arc::new(x);
                        let _ = INSTANCE.set(ret.clone());
                        info!(config = valuable(&ret), file_path = file_name, "loaded configuration");
                        ret
                    })
                    .map_err(|e| error!(error = format!("{}", e), "configuration error"))
            }
        }
    }
}