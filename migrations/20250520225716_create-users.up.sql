-- Add up migration script here
CREATE TABLE users
(
    id            UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username      VARCHAR NOT NULL UNIQUE,
    password_hash VARCHAR NOT NULL
)