-- Add up migration script here
CREATE TABLE places
(
    id   UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR NOT NULL
)