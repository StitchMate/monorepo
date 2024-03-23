CREATE TABLE users(
    id TEXT UNIQUE PRIMARY KEY,
    first_name TEXT,
    last_name TEXT,
    email TEXT,
    created_at TIMESTAMPTZ,
    updated_at TIMESTAMPTZ
);