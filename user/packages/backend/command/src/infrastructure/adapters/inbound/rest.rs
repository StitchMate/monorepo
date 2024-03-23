use std::sync::Arc;

use anyhow::anyhow;
use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use base::domain::entity::command::VerifyCommand;
use base::infrastructure::dto::transport::rest::RESTResponseBuilder;
use tracing::{debug, info};

use crate::{
    application::{
        ports::inbound::create_user::CreateUserUseCase, service::user::command::UserCommandService,
    },
    domain::user::{command::CreateUserCommand, error::service::UserCommandServiceError},
    infrastructure::dtos::user::{
        aggregate::rest::RESTUserCommandModel,
        command::rest::RESTCreateUserCommand,
        error::rest::{RESTUserCommandError, RESTUserError},
    },
};

pub struct RESTUserCommandAdapter {
    service: Arc<UserCommandService>,
    port: u16
}

async fn create_user(
    State(service): State<Arc<UserCommandService>>,
    Json(command): Json<RESTCreateUserCommand>,
) -> impl IntoResponse {
    let mut command: CreateUserCommand = command.into();
    debug!(command = ?command, "received command");
    //TODO: Add right error type to verify
    if let Err(e) = command.verify(Arc::new(())).await {
        let errors = e
            .into_iter()
            //TODO: Add Command Errors
            .map(|_x| RESTUserCommandError::ApiError(RESTUserError::unknown_error()))
            .collect::<Vec<RESTUserCommandError>>();
        return (StatusCode::BAD_REQUEST, Json(errors)).into_response();
    }
    //TODO: We can enable ? sugaring here when we get concerete error types...
    let mut response_builder: RESTResponseBuilder<RESTUserCommandModel, RESTUserCommandError> =
        RESTResponseBuilder::new();
    let mut status_code: StatusCode = StatusCode::CREATED;
    match service
        .create_user(command.into(), vec![])
        .await
        .map_err(|e| {
            <UserCommandServiceError as Into<RESTUserCommandError>>::into(e)
        }) {
        Err(e) => {
            debug!(result =?e, "returning result");
            let error_type = match e.clone() {
                RESTUserCommandError::ApiError(e) => e.code,
                RESTUserCommandError::InvalidRequestError(e) => e.code,
            };
            response_builder = response_builder.errors(vec![e]);
            if error_type == Some("unknown_error".into()) {
                status_code = StatusCode::INTERNAL_SERVER_ERROR;
            } else {
                status_code = StatusCode::BAD_REQUEST;
            }
        }
        Ok(created_user) => {
            debug!(result =?created_user, "returning result");
            response_builder = response_builder.data(created_user);
        }
    };
    (status_code, Json(response_builder.build())).into_response()
}

impl RESTUserCommandAdapter {
    pub fn new(service: Arc<UserCommandService>, port: u16) -> Self {
        Self { service, port }
    }
    fn app(&self) -> Router {
        Router::new()
            .route("/user", post(create_user))
            .with_state(self.service.clone())
    }
    pub async fn run(self) -> Result<(), anyhow::Error> {
        let app = self.app();
        let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", self.port)).await?;
        info!(port = self.port, service = "user_command", transport = "rest", "started user-command rest service");
        axum::serve(listener, app).await.map_err(|e| anyhow!(e))
    }
}

#[cfg(test)]
mod tests {
    use axum_test::{TestServer, TestServerConfig};
    use base::infrastructure::dto::transport::rest::RESTResponse;
    use user_common::{
        domain::value_object::{email::Email, name::Name},
        infrastructure::dto::name::rest::RESTName,
    };

    use crate::{
        domain::user::{aggregate::UserAggregate, error::repository::UserEventRepositoryError},
        infrastructure::adapters::outbound::event_repository::{
            MockUserEventRepository, UserEventRepositoryEnum,
        },
    };

    use super::*;

    fn setup_test_adapter(service: Arc<UserCommandService>) -> TestServer {
        let adapter = RESTUserCommandAdapter::new(service, 3001);
        let app = adapter.app();
        let config = TestServerConfig::builder()
            .expect_success_by_default()
            .mock_transport()
            .build();

        TestServer::new_with_config(app, config).unwrap()
    }

    #[tokio::test]
    async fn it_creates_a_non_existing_user() {
        //Arrange
        let expected = UserAggregate {
            id: Some("1".into()),
            name: Some(Name {
                first: "test".into(),
                last: "test".into(),
            }),
            email: Some(Email::new("test@test.com")),
            ..Default::default()
        };
        let expected_response: RESTUserCommandModel = expected.into();
        let mut repo = MockUserEventRepository::new();
        repo.expect_aggregate_exists_by_email()
            .times(1)
            .returning(|email| {
                Err(UserEventRepositoryError::UserDoesNotExist(
                    "email".into(),
                    email.into(),
                ))
            });
        repo.expect_store_events().times(1).returning(|_| Ok(()));

        let service: Arc<UserCommandService> = Arc::new(UserCommandService::new(Arc::new(
            UserEventRepositoryEnum::from(repo),
        )));
        let adapter = setup_test_adapter(service);

        //Act
        let response = adapter
            .post("/user")
            .json(&RESTCreateUserCommand {
                name: Some(RESTName {
                    first: "test".into(),
                    last: "test".into(),
                }),
                email: Some("test@test.com".into()),
            })
            .await;

        //Assert
        assert_eq!(response.status_code(), StatusCode::CREATED);

        let data = response
            .json::<RESTResponse<RESTUserCommandModel, RESTUserCommandError>>()
            .data()
            .unwrap();

        assert!(data.id.is_some());
        assert!(data.created_at.is_some());
        assert!(data.updated_at.is_some());
        assert_eq!(data.name, expected_response.name);
        assert_eq!(data.email, expected_response.email);
    }
    #[tokio::test]
    async fn it_fails_to_create_an_existing_user() {
        //Arrange
        let expected_error = RESTUserCommandError::InvalidRequestError(RESTUserError {
            code: Some("email_in_use".into()),
            message: Some("The provided email address is already used by another account".into()),
            param_value: Some("test@test.com".into()),
            param: Some(format!("payload:{}", "email")),
            doc_url: None,
        });
        let mut repo = MockUserEventRepository::new();
        repo.expect_aggregate_exists_by_email()
            .times(1)
            .returning(|_email| Ok(()));

        let service: Arc<UserCommandService> = Arc::new(UserCommandService::new(Arc::new(
            UserEventRepositoryEnum::from(repo),
        )));
        let adapter = setup_test_adapter(service);

        //Act
        let response = adapter
            .post("/user")
            .json(&RESTCreateUserCommand {
                name: Some(RESTName {
                    first: "test".into(),
                    last: "test".into(),
                }),
                email: Some("test@test.com".into()),
            })
            .expect_failure()
            .await;

        //Assert
        assert_eq!(response.status_code(), StatusCode::BAD_REQUEST);

        let data: RESTResponse<RESTUserCommandModel, RESTUserCommandError> =
            response.json::<RESTResponse<RESTUserCommandModel, RESTUserCommandError>>();

        assert!(data.errors().is_some());

        let errors = data.errors().unwrap();
        assert!(errors.len() == 1);
        assert_eq!(errors[0], expected_error);
    }
}
