CREATE TABLE user_events(
    aggregate_type TEXT,
    aggregate_id TEXT,
    event_id TEXT,
    sequence TEXT,
    event_type TEXT,
    event_version TEXT,
    payload JSONB,
    metadata JSONB,
    timestamp TIMESTAMPTZ
);

CREATE TABLE user_snapshots(
    aggregate_type TEXT,
    aggregate_id TEXT,
    payload JSONB,
    last_sequence TEXT,
    snapshot_id TEXT,
    timestamp TIMESTAMPTZ
);

CREATE TABLE user_outbox_events(
    aggregate_type TEXT,
    aggregate_id TEXT,
    event_id TEXT,
    sequence TEXT,
    event_type TEXT,
    event_version TEXT,
    payload JSONB,
    metadata JSONB,
    timestamp TIMESTAMPTZ
);