use chrono::{DateTime, Utc};
use educe::Educe;

use super::aggregate::Aggregate;

#[derive(Clone, Debug, Educe)]
#[educe(Default)]
pub struct AggregateSnapshot<S>
where
    S: Aggregate + Default
{
     /// The id of the aggregate instance that this is a snapshot for
    pub aggregate_id: String,
    /// The type of aggregate instance
    pub aggregate_type: String,
    /// The current state of the aggregate instance (e.g. the snapshot data)
    pub payload: S,
    /// The last committed event sequence ULID for this aggregate instance.
    pub last_sequence: String,
    /// The id of this snapshot
    #[educe(Default(expression = "format!(\"snp_{}\", ulid::Ulid::new())"))]
    pub snapshot_id: String,
    /// Timestamp of when this event was produced
    pub timestamp: DateTime<Utc>,
}

pub trait Snapshot<A>
where
    A: Aggregate + Default
{
    fn snapshot(&self) -> Option<AggregateSnapshot<A>> where Self: Sized;
}

pub trait ApplySnapshot<A>
where
    A: Aggregate + Default
{
    fn apply_snapshot(&mut self, snapshot: AggregateSnapshot<A>);
}