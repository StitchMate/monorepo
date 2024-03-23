use std::collections::HashMap;

use chrono::{DateTime, Utc};

use crate::domain::entity::aggregate::Aggregate;

#[derive(Debug)]
pub struct EventEnvelope<A>
where
    A: Event,
{
    /// The id of the aggregate instance.
    pub aggregate_id: String,
    /// The type of aggregate instance
    pub aggregate_type: String,
    /// The sequence id for an aggregate instance.
    pub sequence: String,
    /// The event payload with all business information.
    pub payload: A,
    /// Additional metadata for use in auditing, logging or debugging purposes.
    pub metadata: HashMap<String, String>,
    /// Timestamp of when this event was produced
    pub timestamp: DateTime<Utc>,
}

impl<A: Event> Clone for EventEnvelope<A> {
    fn clone(&self) -> Self {
        EventEnvelope {
            aggregate_id: self.aggregate_id.clone(),
            aggregate_type: self.aggregate_type.clone(),
            sequence: self.sequence.clone(),
            payload: self.payload.clone(),
            metadata: self.metadata.clone(),
            timestamp: self.timestamp.clone(),
        }
    }
}

pub trait Event: Clone + Sync + Send {
    fn to_string(&self) -> String;

    fn event_id(&self) -> String;

    fn event_type(&self) -> String;

    fn event_version(&self) -> String;
}

pub trait EventMetadata {
    fn event_type(&self) -> String;

    fn event_version(&self) -> String;

    fn event_id(&self) -> String;
}

pub trait ApplyEvent<E, A>
where
    A: Aggregate,
{
    fn apply(&mut self, event: E);
}