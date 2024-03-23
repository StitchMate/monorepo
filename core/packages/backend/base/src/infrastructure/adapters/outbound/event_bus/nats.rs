use std::{sync::Arc, time::Duration};

use anyhow::Result;
use async_nats::{jetstream::Context, ConnectErrorKind, ConnectOptions};
use tracing::{info, error, warn};

use crate::domain::entity::error::EventBusError;

#[derive(Debug)]
pub struct NATSConnector {
    pub jetstream_client: Context,
}

//TODO: Better Errors
impl NATSConnector {
    pub async fn new(address: String) -> Result<Arc<Self>> {
        let client = async_nats::connect_with_options(
            address,
            ConnectOptions::new()
                .no_echo()
                .ping_interval(Duration::from_secs(15))
                .connection_timeout(Duration::from_secs(15))
                .event_callback(|e| async move {
                    match e {
                        async_nats::Event::Connected => info!("{e}"),
                        async_nats::Event::Disconnected => error!("{e}"),
                        async_nats::Event::ServerError(_) => error!("{e}"),
                        async_nats::Event::ClientError(_) => error!("{e}"),
                        _ => warn!("{e}"),
                    }
                }),
        )
        .await.map_err(|e| match e.kind() {
            ConnectErrorKind::TimedOut => EventBusError::ConnectionError(e.to_string()),
            ConnectErrorKind::Authentication => EventBusError::AuthenticationError(e.to_string()),
            _ => EventBusError::UnknownError // TODO: Cover these
        })?;
        let jetstream = async_nats::jetstream::new(client);
        
        return Ok(Arc::new(Self {
            jetstream_client: jetstream
        }))
    }
}
