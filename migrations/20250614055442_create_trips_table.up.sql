CREATE TABLE trips
(
    id         UUID PRIMARY KEY     DEFAULT gen_random_uuid(),
    name       VARCHAR     NOT NULL,
    owner_id   UUID        NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    start_at TIMESTAMPTZ NULL,
    end_at TIMESTAMPTZ NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NULL
);

CREATE TRIGGER set_updated_at
    BEFORE UPDATE
    ON trips
    FOR EACH ROW
EXECUTE FUNCTION update_modified_column();