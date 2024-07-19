use std::sync::Arc;

use anyhow::anyhow;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use tower_http::cors::CorsLayer;
use tracing::{debug, info};

use crate::{
    application::{
        ports::inbound::get_user_by_id::GetUserByIdUseCase, service::user::query::UserQueryService,
    },
    domain::user::query::UserByIdQuery,
    infrastructure::dtos::user::{
        error::rest::RESTUserQueryError, model::rest::RESTUserQueryModel,
    },
};

pub struct RESTUserQueryAdapter {
    service: Arc<UserQueryService>,
    port: u16
}

async fn get_user(
    State(service): State<Arc<UserQueryService>>,
    Path(user_id): Path<String>,
) -> impl IntoResponse {
    let query = UserByIdQuery { id: user_id };
    debug!(query = ?query, "received query");
    let found_user: Result<RESTUserQueryModel, RESTUserQueryError> = service
        .get_user_by_id(query, vec![])
        .await
        .map_err(|e| e.into());
    debug!(result = ?found_user, "returning result");
    if found_user.is_err() {
        return (StatusCode::NOT_FOUND, Json(found_user.unwrap_err())).into_response();
    }
    return (StatusCode::OK, Json(found_user.unwrap())).into_response();
}

impl RESTUserQueryAdapter {
    pub fn new(service: Arc<UserQueryService>, port: u16) -> Self {
        Self { service, port }
    }
    fn app(&self) -> Router {
        Router::new()
            .route("/user/:user_id", get(get_user))
            .layer(CorsLayer::very_permissive())
            .with_state(self.service.clone())
    }
    pub async fn run(self) -> Result<(), anyhow::Error> {
        let app = self.app();
        let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", self.port)).await?;
        info!(port = self.port, service = "user_query", transport = "rest", "started user-query rest service");
        axum::serve(listener, app).await.map_err(|e| anyhow!(e))
    }
}

#[cfg(test)]
mod tests {
    use axum_test::{TestServer, TestServerConfig};
    use user_common::domain::value_object::{email::Email, name::Name};

    use crate::{
        domain::user::model::UserQueryModel,
        infrastructure::adapters::outbound::user::repository::{
            postgres::MockPostgresUserQueryRepository, UserQueryRepositoryEnum,
        },
    };

    use super::*;

    fn setup_test_adapter(service: Arc<UserQueryService>) -> TestServer {
        let adapter = RESTUserQueryAdapter::new(service, 4000);
        let app = adapter.app();
        let config = TestServerConfig::builder()
            .expect_success_by_default()
            .mock_transport()
            .build();

        TestServer::new_with_config(app, config).unwrap()
    }

    #[tokio::test]
    async fn it_returns_an_existing_user() {
        //Arrange
        let expected = UserQueryModel {
            id: Some("1".into()),
            name: Some(Name {
                first: "test".into(),
                last: "test".into(),
            }),
            email: Some(Email::new("test@test.com")),
            ..Default::default()
        };
        let expected_response: RESTUserQueryModel = expected.clone().into();
        let mut repo = MockPostgresUserQueryRepository::new();
        repo.expect_get_by_id()
            .times(1)
            .returning(move |_id, _fields| {
                let expected = expected.clone();
                Box::pin(async { Ok(expected) })
            });

        let service: Arc<UserQueryService> = Arc::new(UserQueryService::new(Arc::new(
            UserQueryRepositoryEnum::from(repo),
        )));
        let adapter = setup_test_adapter(service);

        //Act
        let response = adapter.get("/user/1").await;

        //Assert
        assert_eq!(response.status_code(), StatusCode::OK);

        assert_eq!(response.json::<RESTUserQueryModel>(), expected_response);
    }
}
