CREATE TABLE tasks
(
    id         UUID PRIMARY KEY     DEFAULT gen_random_uuid(),
    owner_id   UUID references users (id),
    title      VARCHAR     NOT NULL,
    content    VARCHAR     NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NULL
);

CREATE TRIGGER set_updated_at
    BEFORE UPDATE
    ON tasks
    FOR EACH ROW
EXECUTE FUNCTION update_modified_column();