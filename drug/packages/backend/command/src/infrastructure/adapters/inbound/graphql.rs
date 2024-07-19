use std::sync::Arc;

use anyhow::anyhow;
use async_graphql::ErrorExtensions;
use async_graphql::{http::GraphiQLSource, Context, EmptySubscription, Object, Result, Schema};
use async_graphql_axum::GraphQL;
use axum::{
    response::{self, IntoResponse},
    routing::post_service,
    Router,
};
use tracing::{debug, info};

use crate::{
    application::{
        ports::inbound::create_user::CreateUserUseCase, service::user::command::UserCommandService,
    },
    domain::user::command::CreateUserCommand,
    infrastructure::dtos::user::{
        aggregate::graphql::GraphQLUserCommandModel, command::graphql::GraphQLCreateUserCommand,
        error::graphql::GraphQLUserCommandError,
    },
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct GraphQLUserCommandAdapter {
    schema: Schema<Query, Mutation, EmptySubscription>,
    port: u16
}

pub struct Query;

#[Object(extends)]
impl Query {
    #[graphql(visible = false)]
    async fn _graphql_user_command_version(&self) -> String {
        VERSION.into()
    }
}

pub struct Mutation;

#[Object(extends)]
impl Mutation {
    async fn create_user(
        &self,
        ctx: &Context<'_>,
        input: GraphQLCreateUserCommand,
    ) -> Result<Option<GraphQLUserCommandModel>> {
        let service = ctx.data::<Arc<UserCommandService>>().unwrap();
        let command: CreateUserCommand = input.into();
        debug!(command = ?command, "received command");
        let result = service.create_user(command, vec![]).await.map_err(|e| {
            debug!(error = ?e, "recieved error");
            let transformed_err: GraphQLUserCommandError = e.into();
            match &transformed_err {
                GraphQLUserCommandError::UnknownError => transformed_err.extend_with(|_, e| {
                    e.set("code", "unknown_error");
                }),
                GraphQLUserCommandError::EmailAddressInUse(email) => {
                    transformed_err.extend_with(|_, e| {
                        e.set("code", "email_in_use");
                        e.set("param", "email");
                        e.set("param_value", email);
                    })
                }
            }
        });
        if result.is_err() {
            ctx.add_error(
                result
                    .unwrap_err()
                    .into_server_error(ctx.item.pos),
            );
            return Ok(None);
        }
        debug!(result =?result, "returning result");
        return Ok(Some(result.unwrap()));
    }
}

async fn _graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/").finish())
}

impl GraphQLUserCommandAdapter {
    pub fn new(service: Arc<UserCommandService>, port: u16) -> Self {
        let schema = Schema::build(Query, Mutation, EmptySubscription)
            .enable_federation()
            .data(service)
            .finish();
        Self { schema, port }
    }
    fn _schema(&self) -> Schema<Query, Mutation, EmptySubscription> {
        self.schema.clone()
    }
    fn app(&self) -> Router {
        Router::new().route("/", post_service(GraphQL::new(self.schema.clone())))
    }
    pub async fn run(self) -> Result<(), anyhow::Error> {
        let app = self.app();
        let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", self.port)).await?;
        info!(port = self.port, service = "user_command", transport = "graphql", "started user-command graphql service");
        axum::serve(listener, app).await.map_err(|e| anyhow!(e))
    }
}

#[cfg(test)]
mod tests {
    use axum::http::StatusCode;
    use axum_test::{TestServer, TestServerConfig};
    use serde_json::{json, Value};
    use user_common::infrastructure::dto::name::graphql::GraphQLName;

    use crate::{
        domain::user::error::repository::UserEventRepositoryError,
        infrastructure::adapters::outbound::event_repository::{
            MockUserEventRepository, UserEventRepositoryEnum,
        },
    };

    use super::*;

    fn setup_test_adapter(
        service: Arc<UserCommandService>,
    ) -> (TestServer, Schema<Query, Mutation, EmptySubscription>) {
        let adapter = GraphQLUserCommandAdapter::new(service, 3000);
        let app = adapter.app();
        let config = TestServerConfig::builder()
            .expect_success_by_default()
            .mock_transport()
            .build();

        (
            TestServer::new_with_config(app, config).unwrap(),
            adapter._schema(),
        )
    }

    #[tokio::test]
    async fn it_returns_an_existing_user() {
        //Arrange
        let expected = GraphQLUserCommandModel {
            name: Some(GraphQLName {
                first: "test".into(),
                last: "test".into(),
            }),
            email: Some("test@test.com".into()),
            ..Default::default()
        };
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
        let (adapter, _schema) = setup_test_adapter(service);
        const CREATE_USER_MUTATION: &str = "
            mutation CreateUser($input: GraphQLCreateUserCommand!) {
                createUser(input: $input) {
                    id
                    name {
                        first
                        last
                        full
                    }
                    email
                    createdAt
                    updatedAt
                }
            }
        ";
        let variables = json!({
            "input": {
                "firstName": expected.name.as_ref().unwrap().first,
                "lastName": expected.name.as_ref().unwrap().last,
                "email": expected.email.as_ref().unwrap()
            }
        });

        let body = serde_json::to_string(
            &json!({ "query": CREATE_USER_MUTATION, "variables": variables }),
        )
        .unwrap();

        //Act
        let response = adapter.post("/").text(body).await;

        //Assert
        assert_eq!(response.status_code(), StatusCode::OK);

        let response_body: Value = serde_json::from_str(&response.text()).unwrap();

        let data = &response_body["data"]["createUser"];

        assert!(data["id"].is_string());
        assert!(data["createdAt"].is_string());
        assert!(data["updatedAt"].is_string());
        assert_eq!(
            data["name"]["first"],
            expected.name.as_ref().unwrap().first.to_string()
        );
        assert_eq!(
            data["name"]["last"],
            expected.name.as_ref().unwrap().last.to_string()
        );
        assert_eq!(
            data["name"]["full"],
            expected.name.as_ref().unwrap().full_name()
        );
        assert_eq!(data["email"], expected.email.as_ref().unwrap().to_string());
    }

    #[tokio::test]
    async fn it_fails_to_create_an_existing_user() {
        //Arrange
        let create_user = GraphQLUserCommandModel {
            name: Some(GraphQLName {
                first: "test".into(),
                last: "test".into(),
            }),
            email: Some("test@test.com".into()),
            ..Default::default()
        };
        let mut repo = MockUserEventRepository::new();
        repo.expect_aggregate_exists_by_email()
            .times(1)
            .returning(|_email| Ok(()));

        let service: Arc<UserCommandService> = Arc::new(UserCommandService::new(Arc::new(
            UserEventRepositoryEnum::from(repo),
        )));
        let (adapter, _schema) = setup_test_adapter(service);
        const CREATE_USER_MUTATION: &str = "
            mutation CreateUser($input: GraphQLCreateUserCommand!) {
                createUser(input: $input) {
                    id
                    name {
                        first
                        last
                        full
                    }
                    email
                    createdAt
                    updatedAt
                }
            }
        ";
        let variables = json!({
            "input": {
                "firstName": create_user.name.as_ref().unwrap().first,
                "lastName": create_user.name.as_ref().unwrap().last,
                "email": create_user.email.as_ref().unwrap()
            }
        });

        let body = serde_json::to_string(
            &json!({ "query": CREATE_USER_MUTATION, "variables": variables }),
        )
        .unwrap();

        //Act
        let response = adapter.post("/").text(body).await;

        //Assert
        assert_eq!(response.status_code(), StatusCode::OK);

        let response_body: Value = serde_json::from_str(&response.text()).unwrap();

        let errors = &response_body["errors"].as_array().unwrap();

        assert_eq!(errors.len(), 1);
        assert_eq!(
            errors[0]["message"],
            "The provided email address is already in use by another account"
        );
        assert_eq!(errors[0]["extensions"]["code"], "email_in_use");
        assert_eq!(errors[0]["extensions"]["param"], "email");
        assert_eq!(
            errors[0]["extensions"]["param_value"],
            create_user.email.as_ref().unwrap().to_string()
        );
    }
}
