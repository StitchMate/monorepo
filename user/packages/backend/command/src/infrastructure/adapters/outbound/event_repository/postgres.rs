use std::sync::Arc;

use async_trait::async_trait;
use base::config::repository::PostgresConfig;
use base::domain::entity::error::{EventBusError, RepositoryError};
use base::{
    domain::entity::event::EventEnvelope,
    infrastructure::adapters::outbound::storage::postgres::PostgresConnector,
};
use base::{
    domain::entity::{event::Event, snapshot::AggregateSnapshot},
    infrastructure::dto::storage::sql::{SQLAggregateSnapshot, SQLEventEnvelope},
};
use serde_json::json;
use sqlx::Postgres;
use user_common::{
    application::ports::outbound::event_bus::UserEventBus, domain::event::UserEvent,
    infrastructure::dto::user::event::sql::SQLUserEvent,
};

use crate::domain::user::error::repository::UserEventRepositoryError;
use crate::{
    application::ports::outbound::repository::UserEventRepository,
    domain::user::aggregate::UserAggregate,
    infrastructure::dtos::user::aggregate::sql::SQLUserAggregate,
};
use tracing::debug;

const EVENT_TABLE_NAME: &str = "user_events";
const SNAPSHOT_TABLE_NAME: &str = "user_snapshots";
const OUTBOX_TABLE_NAME: &str = "user_outbox_events";

#[derive(Clone, Debug)]
pub struct PostgresUserEventRepository {
    pub connector: Arc<PostgresConnector>,
}

impl PostgresUserEventRepository {
    pub async fn new(config: &PostgresConfig) -> Result<Self, UserEventRepositoryError> {
        let connector = PostgresConnector::new(config).await.map_err(|e| {
            if e.to_string().contains("authentication") {
                RepositoryError::AuthenticationError(e.to_string())
            } else {
                RepositoryError::ConnectionError(e.to_string())
            }
        })?;
        return Ok(Self {
            connector
        })
    }
    #[cfg(test)]
    pub async fn drop(&self) -> Result<(), anyhow::Error> {
        let drop_queries = [
            format!("DELETE FROM {}", EVENT_TABLE_NAME),
            format!("DELETE FROM {}", SNAPSHOT_TABLE_NAME),
            format!("DELETE FROM {}", OUTBOX_TABLE_NAME),
        ];

        for query in drop_queries {
            let plan = sqlx::query::<Postgres>(&query);
            let _ = plan.execute(&self.connector.pool).await?;
        }

        Ok(())
    }
}

#[async_trait]
impl UserEventRepository for PostgresUserEventRepository {
    async fn migrate(&self, path: String) -> Result<(), UserEventRepositoryError> {
        return self.connector.migrate(path).await.map_err(|e| e.into());
    }
    async fn aggregate_exists_by_id(
        &self,
        aggregate_id: &str,
    ) -> Result<(), UserEventRepositoryError> {
        let query = format!(
            "SELECT sequence FROM {} WHERE aggregate_id = $1 AND aggregate_type = $2 ORDER BY sequence DESC LIMIT 1",
            EVENT_TABLE_NAME
        );
        debug!(query = query, "performing query");
        let mut plan = sqlx::query::<Postgres>(&query);
        plan = plan.bind(aggregate_id).bind("user");
        let results = plan.fetch_all(&self.connector.pool).await;
        match results {
            Err(e) => return Err(RepositoryError::PersistError(e.to_string()).into()),
            Ok(v) => {
                if v.len() > 0 {
                    return Ok(());
                }
                return Err(UserEventRepositoryError::UserDoesNotExist(
                    "id".into(),
                    aggregate_id.into(),
                ));
            }
        };
    }
    async fn aggregate_exists_by_email(&self, email: &str) -> Result<(), UserEventRepositoryError> {
        let query = format!(
            "SELECT sequence FROM {} WHERE payload->>'email' = $1 AND aggregate_type = $2 ORDER BY sequence DESC LIMIT 1",
            EVENT_TABLE_NAME
        );
        debug!(query = query, "performing query");
        let mut plan = sqlx::query::<Postgres>(&query);
        plan = plan.bind(email).bind("user");
        let results = plan.fetch_all(&self.connector.pool).await;
        match results {
            Err(e) => return Err(RepositoryError::PersistError(e.to_string()).into()),
            Ok(v) => {
                if v.len() > 0 {
                    return Ok(());
                }
                return Err(UserEventRepositoryError::UserDoesNotExist(
                    "email".into(),
                    email.into(),
                ));
            }
        };
    }
    async fn store_events(
        &self,
        events: Vec<EventEnvelope<UserEvent>>,
    ) -> Result<(), Vec<UserEventRepositoryError>> {
        let fields = vec![
            "aggregate_type",
            "aggregate_id",
            "sequence",
            "event_type",
            "event_version",
            "payload",
            "metadata",
            "timestamp",
        ];
        let placeholders: Vec<String> = (0..fields.len())
            .map(|x| format!("${}", (x + 1).to_string()))
            .collect();
        let placeholder_str = placeholders.join(", ");
        let query = format!(
            "INSERT INTO {} ({}) VALUES ( {} )",
            EVENT_TABLE_NAME,
            fields.join(", "),
            placeholder_str
        );
        let outbox_query = format!(
            "INSERT INTO {} ({}) VALUES ( {} )",
            OUTBOX_TABLE_NAME,
            fields.join(", "),
            placeholder_str
        );
        debug!(query = query, outbox_query = outbox_query, "performing insert");
        let mut results = vec![];
        for x in events {
            let mut tx = self
                .connector
                .pool
                .begin()
                .await
                .map_err(|e| vec![RepositoryError::TransactionError(e.to_string()).into()])?;
            let plan = sqlx::query::<Postgres>(&query);
            let outbox_plan = sqlx::query::<Postgres>(&outbox_query);
            let (event_type, event_version) = (&x.payload.event_type(), &x.payload.event_version());
            let enum_sql: SQLUserEvent = x.payload.into();
            let insert = plan
                .bind(&x.aggregate_type)
                .bind(&x.aggregate_id)
                .bind(&x.sequence)
                .bind(&event_type)
                .bind(&event_version)
                .bind(json!(enum_sql))
                .bind(json!(x.metadata))
                .bind(&x.timestamp)
                .execute(&mut *tx)
                .await;
            let outbox_insert = outbox_plan
                .bind(&x.aggregate_type)
                .bind(&x.aggregate_id)
                .bind(&x.sequence)
                .bind(&event_type)
                .bind(&event_version)
                .bind(json!(enum_sql))
                .bind(json!(x.metadata))
                .bind(&x.timestamp)
                .execute(&mut *tx)
                .await;
            if outbox_insert.is_err() {
                results.push(outbox_insert)
            }
            tx.commit()
                .await
                .map_err(|e| vec![RepositoryError::TransactionError(e.to_string()).into()])?;
            results.push(insert);
        }
        let mut err: Vec<UserEventRepositoryError> = vec![];
        for result in results {
            match result {
                Err(e) => err.push(RepositoryError::PersistError(e.to_string()).into()),
                _ => {}
            }
        }
        if err.len() > 0 {
            return Err(err);
        }
        return Ok(());
    }
    async fn retrieve_events(
        &self,
        aggregate_id: String,
        after: Option<String>,
    ) -> Result<Vec<EventEnvelope<UserEvent>>, UserEventRepositoryError> {
        let fields = vec![
            "aggregate_type",
            "aggregate_id",
            "sequence",
            "event_type",
            "event_version",
            "payload",
            "metadata",
            "timestamp",
        ];
        let query = match after {
            None => format!(
                "SELECT {} FROM {} WHERE aggregate_id = $1",
                fields.join(", "),
                EVENT_TABLE_NAME
            ),
            Some(_) => format!(
                "SELECT {} FROM {} WHERE aggregate_id = $1 AND sequence > $2 ORDER BY sequence ASC",
                fields.join(", "),
                EVENT_TABLE_NAME
            ),
        };
        debug!(query = query, "performing query");
        let mut plan = sqlx::query_as::<Postgres, SQLEventEnvelope<SQLUserEvent>>(&query);
        plan = match after {
            None => plan.bind(aggregate_id),
            Some(x) => plan.bind(aggregate_id).bind(x),
        };
        let results = plan.fetch_all(&self.connector.pool).await;
        debug!(results = ?results, "received results");
        match results {
            Err(e) => return Err(RepositoryError::RetrieveError(e.to_string()).into()),
            _ => {}
        };
        let mut resp: Vec<EventEnvelope<UserEvent>> = vec![];
        // TODO: We should figure out a way to check the version of the event stored in the database vs. what's defined in code
        // and log out an error and/or push an error that warns us that we are using an old event that has
        // no upcaster to cast it to a newer version
        for raw_event in results.unwrap() {
            //TODO: Add upcasters
            let envelope: EventEnvelope<UserEvent> = raw_event.clone().into();
            resp.push(envelope)
        }
        return Ok(resp);
    }
    async fn store_snapshot(
        &self,
        snapshot: AggregateSnapshot<UserAggregate>,
    ) -> Result<(), UserEventRepositoryError> {
        let fields = vec![
            "aggregate_type",
            "aggregate_id",
            "payload",
            "last_sequence",
            "snapshot_id",
            "timestamp",
        ];
        let placeholders: Vec<String> = (0..fields.len())
            .map(|x| format!("?{}", (x + 1).to_string()))
            .collect();
        let placeholder_str = placeholders.join(", ");
        let query = format!(
            "INSERT INTO {} ({}) VALUES ( {} )",
            SNAPSHOT_TABLE_NAME,
            fields.join(", "),
            placeholder_str
        );
        debug!(query = query, "performing insert");
        let plan = sqlx::query::<Postgres>(&query);
        let enum_sql: SQLUserAggregate = snapshot.payload.clone().into();
        let insert = plan
            .bind(snapshot.aggregate_type)
            .bind(snapshot.aggregate_id)
            .bind(json!(enum_sql).to_string())
            .bind(snapshot.last_sequence)
            .bind(snapshot.snapshot_id)
            .bind(snapshot.timestamp)
            .fetch_optional(&self.connector.pool)
            .await;
        match insert {
            Err(e) => {
                return Err(RepositoryError::PersistError(e.to_string()).into());
            }
            _ => return Ok(()),
        }
    }
    async fn retrieve_latest_snapshot(
        &self,
        aggregate_id: String,
    ) -> Result<Option<AggregateSnapshot<UserAggregate>>, UserEventRepositoryError> {
        let fields = vec![
            "aggregate_type",
            "aggregate_id",
            "payload",
            "last_sequence",
            "snapshot_id",
            "timestamp",
        ];
        let query = format!(
            "SELECT {} FROM {} WHERE aggregate_id = $1 ORDER BY snapshot_id DESC LIMIT 1",
            fields.join(", "),
            SNAPSHOT_TABLE_NAME
        );
        debug!(query = query, "performing query");
        let plan = sqlx::query_as::<Postgres, SQLAggregateSnapshot<SQLUserAggregate>>(&query)
            .bind(aggregate_id);
        let result = plan.fetch_optional(&self.connector.pool).await;
        debug!(result = ?result, "received results");
        match result {
            Err(e) => return Err(RepositoryError::RetrieveError(e.to_string()).into()),
            _ => {}
        };
        match result.unwrap() {
            None => Ok(None),
            Some(x) => Ok(Some(x.into())),
        }
    }
    async fn retrieve_outbox_events(
        &self,
    ) -> Result<Vec<EventEnvelope<UserEvent>>, UserEventRepositoryError> {
        let fields = vec![
            "aggregate_type",
            "aggregate_id",
            "sequence",
            "event_type",
            "event_version",
            "payload",
            "metadata",
            "timestamp",
        ];
        let query = format!("SELECT {} FROM {}", fields.join(", "), OUTBOX_TABLE_NAME);
        debug!(query = query, "performing query");
        let plan = sqlx::query_as::<Postgres, SQLEventEnvelope<SQLUserEvent>>(&query);
        let results = plan.fetch_all(&self.connector.pool).await;
        debug!(results = ?results, "received results");
        match results {
            Err(e) => return Err(RepositoryError::RetrieveError(e.to_string()).into()),
            _ => {}
        };
        let mut resp: Vec<EventEnvelope<UserEvent>> = vec![];
        //TODO: Add upcasters
        for raw_event in results.unwrap() {
            resp.push(raw_event.clone().into());
        }
        return Ok(resp);
    }
    // Used by outbox pattern to remove events after sending
    async fn send_and_delete_outbox_event<B: UserEventBus + Sync + Send>(
        &self,
        bus: Arc<B>,
        event: EventEnvelope<UserEvent>,
    ) -> Result<(), UserEventRepositoryError> {
        let query = format!("DELETE FROM {} WHERE sequence = $1", OUTBOX_TABLE_NAME);
        debug!(query = query, "performing query");
        let mut plan = sqlx::query::<Postgres>(&query);
        plan = plan.bind(event.sequence.clone());
        let mut tx = self.connector.pool.begin().await.map_err(|e| {
            <RepositoryError as Into<UserEventRepositoryError>>::into(
                RepositoryError::TransactionError(e.to_string()),
            )
        })?;
        bus.send_event(event)
            .await
            .map_err(|e| <EventBusError as Into<UserEventRepositoryError>>::into(e))?;
        let result = plan.execute(&mut *tx).await;
        match result {
            Err(e) => {
                tx.rollback().await.map_err(|e| {
                    <RepositoryError as Into<UserEventRepositoryError>>::into(
                        RepositoryError::TransactionError(e.to_string()),
                    )
                })?;
                return Err(RepositoryError::DeleteError(e.to_string()).into());
            }
            _ => {
                tx.commit().await.map_err(|e| {
                    <RepositoryError as Into<UserEventRepositoryError>>::into(
                        RepositoryError::TransactionError(e.to_string()),
                    )
                })?;
                return Ok(());
            }
        }
    }
}
