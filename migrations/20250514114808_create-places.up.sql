-- Add up migration script here
CREATE TABLE IF NOT EXISTS places
(
    id   SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL
)