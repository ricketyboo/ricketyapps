-- Add down migration script here
ALTER TABLE places
    DROP COLUMN created_at,
    DROP COLUMN updated_at;

ALTER TABLE users
    DROP COLUMN created_at,
    DROP COLUMN updated_at;