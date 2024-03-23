use std::sync::Arc;

use async_graphql::ErrorExtensions;
use base::config::http::HTTPServerConfig;
use tracing::{debug, info};
use crate::infrastructure::dtos::user::error::graphql::GraphQLUserQueryError;
use anyhow::anyhow;
use async_graphql::{
    http::GraphiQLSource, Context, EmptyMutation, EmptySubscription, Object, Result, Schema,
};
use async_graphql_axum::GraphQL;
use axum::{
    response::{self, IntoResponse},
    routing::post_service,
    Router,
};

use crate::{
    application::{
        ports::inbound::get_user_by_id::GetUserByIdUseCase, service::user::query::UserQueryService,
    },
    domain::user::query::UserByIdQuery,
    infrastructure::dtos::user::model::graphql::GraphQLUserQueryModel,
};

pub struct GraphQLUserQueryAdapter {
    schema: Schema<Query, EmptyMutation, EmptySubscription>,
    port: u16
}

pub struct Query;

#[Object(extends)]
impl Query {
    async fn user(&self, ctx: &Context<'_>, id: String) -> Result<Option<GraphQLUserQueryModel>> {
        let service = ctx.data::<Arc<UserQueryService>>().unwrap();
        let query = UserByIdQuery { id };
        debug!(query = ?query, "received query");
        let result: Result<GraphQLUserQueryModel, _> = service
            .get_user_by_id(query, vec![])
            .await
            .map_err(|e| {
                debug!(error = ?e, "recieved error");
                let transformed_err: GraphQLUserQueryError = e.into();
                match &transformed_err {
                    GraphQLUserQueryError::UnknownError => transformed_err.extend_with(|_, e| {
                        e.set("code", "unknown_error");
                    }),
                    GraphQLUserQueryError::UserNotFound(identifier, value) => {
                        transformed_err.extend_with(|_, e| {
                            e.set("code", "user_not_found");
                            e.set("param", identifier);
                            e.set("param_value", value);
                        })
                    }
                }
            });
        if result.is_err() {
            debug!(result =?result, "returning result");
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

impl GraphQLUserQueryAdapter {
    pub fn new(service: Arc<UserQueryService>, port: u16) -> Self {
        let schema = Schema::build(Query, EmptyMutation, EmptySubscription)
            .enable_federation()
            .data(service)
            .finish();
        Self { schema, port }
    }
    fn _schema(&self) -> Schema<Query, EmptyMutation, EmptySubscription> {
        self.schema.clone()
    }
    fn app(&self) -> Router {
        Router::new().route("/", post_service(GraphQL::new(self.schema.clone())))
    }
    pub async fn run(self) -> Result<(), anyhow::Error> {
        let app = self.app();
        let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", self.port)).await?;
        info!(port = self.port, service = "user_query", transport = "graphql", "started user-query graphql service");
        axum::serve(listener, app).await.map_err(|e| anyhow!(e))
    }
}

#[cfg(test)]
mod tests {
    use axum::http::StatusCode;
    use axum_test::{TestServer, TestServerConfig};
    use chrono::Utc;
    use serde_json::{json, Value};
    use user_common::domain::value_object::{email::Email, name::Name};
    use crate::domain::user::error::repository::UserQueryRepositoryError;

    use crate::{
        domain::user::model::UserQueryModel,
        infrastructure::adapters::outbound::{
            user::repository::postgres::MockPostgresUserQueryRepository,
            user::repository::UserQueryRepositoryEnum,
        },
    };

    use super::*;

    fn setup_test_adapter(
        service: Arc<UserQueryService>,
    ) -> (TestServer, Schema<Query, EmptyMutation, EmptySubscription>) {
        let adapter = GraphQLUserQueryAdapter::new(service, 4001);
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
        let now = Utc::now();
        let expected = UserQueryModel {
            id: Some("1".into()),
            name: Some(Name {
                first: "test".into(),
                last: "test".into(),
            }),
            email: Some(Email::new("test@test.com")),
            created_at: Some(now),
            updated_at: Some(now),
        };
        let expected_response: GraphQLUserQueryModel = expected.clone().into();
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
        let (adapter, _schema) = setup_test_adapter(service);
        const GET_USER_QUERY: &str = "
            query GetUser($id: ID!) {
                user(id: $id) {
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
            "id": "1"
        });

        let body =
            serde_json::to_string(&json!({ "query": GET_USER_QUERY, "variables": variables }))
                .unwrap();

        //Act
        let response = adapter.post("/").text(body).await;

        //Assert
        assert_eq!(response.status_code(), StatusCode::OK);

        let response_body: Value = serde_json::from_str(&response.text()).unwrap();

        let data = &response_body["data"]["user"];

        assert_eq!(data["id"], expected_response.id.unwrap());
        let name = expected_response.name.unwrap();
        assert_eq!(data["name"]["first"], name.first);
        assert_eq!(data["name"]["last"], name.last);
        assert_eq!(
            data["name"]["full"],
            format!("{} {}", name.first, name.last)
        );
        let email = expected_response.email.unwrap();
        assert_eq!(data["email"], email);
        let created_at = expected_response.created_at.unwrap();
        assert_eq!(data["createdAt"], created_at.to_rfc3339());
        let updated_at = expected_response.updated_at.unwrap();
        assert_eq!(data["updatedAt"], updated_at.to_rfc3339());
    }

    #[tokio::test]
    async fn it_errors_non_existing_user() {
        //Arrange
        ;
        let mut repo = MockPostgresUserQueryRepository::new();
        repo.expect_get_by_id()
            .times(1)
            .returning(move |id, _fields| {
                let id = id.to_string().clone();
                Box::pin(async move { Err(UserQueryRepositoryError::UserNotFound("id".into(), id)) })
            });

        let service: Arc<UserQueryService> = Arc::new(UserQueryService::new(Arc::new(
            UserQueryRepositoryEnum::from(repo),
        )));
        let (adapter, _schema) = setup_test_adapter(service);
        const GET_USER_QUERY: &str = "
            query GetUser($id: ID!) {
                user(id: $id) {
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
            "id": "1"
        });

        let body =
            serde_json::to_string(&json!({ "query": GET_USER_QUERY, "variables": variables }))
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
            "A user with the provided id does not exist"
        );
        assert_eq!(errors[0]["extensions"]["code"], "user_not_found");
        assert_eq!(errors[0]["extensions"]["param"], "path:id");
        assert_eq!(
            errors[0]["extensions"]["param_value"].to_string(),
            variables.get("id").unwrap().to_string()
        );
    }
}
