mod config;
use base::application::ports::inbound::projector::Projector;
use base::config::event_bus::EventBusBackend;
use base::config::repository::RepositoryBackend;
use base::domain::entity::error::RepositoryError;
use base::infrastructure::adapters::outbound::event_bus::nats::NATSConnector;
use config::AppConfig;
use std::sync::Arc;
use user_common::infrastructure::adapters::outbound::user::event_bus::nats::NATSUserEventBus;
use user_common::infrastructure::adapters::outbound::user::event_bus::UserEventBusEnum;
use user_query::application::service::user::projector::UserProjectorService;
use user_query::domain::user::error::repository::UserQueryRepositoryError;
use user_query::infrastructure::adapters::inbound::projector::UserEventProjector;
use user_query::infrastructure::adapters::inbound::rest::RESTUserQueryAdapter;

use chrono::Utc;
use tokio::signal::unix::{signal, SignalKind};
use tracing::error;
use user_query::application::ports::outbound::repository::UserQueryRepository;
use user_query::application::service::user::query::UserQueryService;
use user_query::infrastructure::adapters::inbound::graphql::GraphQLUserQueryAdapter;
use user_query::infrastructure::adapters::outbound::user::repository::{
    postgres::PostgresUserQueryRepository, UserQueryRepositoryEnum,
};

use tokio::signal;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    match tracing_subscriber::fmt()
        .json()
        .with_max_level(tracing::Level::DEBUG)
        .with_current_span(false)
        .try_init()
    {
        Ok(_) => {},
        Err(e) => println!("{{\"timestamp\":\"{}\",\"level\":\"ERROR\",\"fields\":{{\"message\":\"{}\"}},\"target\":\"{}\"}}", Utc::now(), e, env!("CARGO_BIN_NAME")),
    }

    let config = AppConfig::new(Some(&"./config.yaml".to_string()));

    if config.is_err() {
        std::process::exit(1);
    }

    let config = config.unwrap();

    let repository_config = match &config.repository.backend {
        RepositoryBackend::Postgres(cfg) => cfg,
        _ => unimplemented!(),
    };

    let repository: Arc<UserQueryRepositoryEnum> =
        match PostgresUserQueryRepository::new(&repository_config).await {
            Ok(x) => Arc::new(x.into()),
            Err(e) => match e {
                UserQueryRepositoryError::RepositoryError(e) => match e {
                    RepositoryError::AuthenticationError(e) => {
                        error!(error = e, "failed to authenticate to postgres");
                        std::process::exit(1);
                    }
                    RepositoryError::ConnectionError(e) => {
                        error!(error = e, "failed to connect to postgres");
                        std::process::exit(1);
                    }
                    _ => unimplemented!(),
                },
                _ => unimplemented!(),
            },
        };
    if let Err(e) = repository
        .migrate(repository_config.migration_path.to_string())
        .await
    {
        error!(error = ? e, "failed to migrate");
        std::process::exit(1);
    }

    let query_service = Arc::new(UserQueryService::new(repository.clone()));

    let adapters_config = &config.adapters;

    let mut running_services = vec![];

    if adapters_config.is_some() {
        if let Some(graphql_config) = adapters_config.as_ref().clone().unwrap().graphql.as_ref() {
            if graphql_config.enabled.is_none() || graphql_config.enabled == Some(true) {
                let graphql_adapter =
                    GraphQLUserQueryAdapter::new(query_service.clone(), graphql_config.port);

                let graphql = tokio::spawn(async move {
                    match graphql_adapter.run().await {
                        Err(e) => {
                            error!(error = ?e, "failed to start user-query graphql service");
                            std::process::exit(1);
                        }
                        _ => {}
                    }
                });
                running_services.push(graphql);
            }
        }

        if let Some(rest_config) = adapters_config.as_ref().clone().unwrap().rest.as_ref() {
            if rest_config.enabled.is_none() || rest_config.enabled == Some(true) {
                let rest_adapter =
                    RESTUserQueryAdapter::new(query_service.clone(), rest_config.port);

                let rest = tokio::spawn(async move {
                    match rest_adapter.run().await {
                        Err(e) => {
                            error!(error = ?e, "failed to start user-query rest service");
                            std::process::exit(1);
                        }
                        _ => {}
                    }
                });
                running_services.push(rest);
            }
        }
    }

    let eventbus_config = &config.eventbus;

    let mut eventbus: Option<Arc<UserEventBusEnum>> = None;

    if let Some(eventbus_config) = eventbus_config.as_ref() {
        match &eventbus_config.backend {
            EventBusBackend::NATS(nats_config) => {
                match NATSConnector::new(nats_config.address.clone()).await {
                    Ok(nats_connector) => {
                        eventbus = Some(Arc::new(
                            NATSUserEventBus {
                                connector: nats_connector,
                            }
                            .into(),
                        ))
                    }
                    Err(e) => {
                        error!(error = ?e, "failed to connect to event bus");
                    }
                }
            }
        }
    }

    let projectors_config = &config.projectors;

    if projectors_config.is_some() && projectors_config.as_ref().clone().unwrap().user.is_some() {
        let user_projector_config = projectors_config.as_ref().clone().unwrap().user.unwrap();
        if let Some(bus) = eventbus {
            let projector_service = UserProjectorService::new(repository.clone());
            let user_projector_adapter = UserEventProjector::new(
                bus,
                projector_service,
                user_projector_config.topic.clone(),
                user_projector_config.durable_name.clone(),
            );
            let user_projector = tokio::spawn(async move {
                match user_projector_adapter.run().await {
                    Err(e) => {
                        error!(error = ?e, "failed to start user-query projector");
                        std::process::exit(1);
                    }
                    _ => {}
                }
            });
            running_services.push(user_projector);
        } else {
            error!("failed to start projector due to unheathly eventbus");
        }
    }

    let mut sigterm = signal(SignalKind::terminate())?;

    tokio::select! {
        _ = signal::ctrl_c() => {
            for service in running_services {
                service.abort();
            }
            println!("Ctrl-C received, shuting down");
            std::process::exit(0);
        }
        _ = sigterm.recv() => {
            for service in running_services {
                service.abort();
            }
            println!("terminate signal received, shutting down");
            std::process::exit(0);
        }
    }
}
