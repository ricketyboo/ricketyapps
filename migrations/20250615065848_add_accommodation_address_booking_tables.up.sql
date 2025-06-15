CREATE TABLE addresses
(
    id         UUID PRIMARY KEY     DEFAULT gen_random_uuid(),
    owner_id   UUID        NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    line1      VARCHAR     NOT NULL,
    line2      VARCHAR     NULL,
    line3      VARCHAR     NULL,
    province   VARCHAR     NULL,
    city       VARCHAR     NULL,
    postcode   VARCHAR     NULL,
    country    VARCHAR     NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NULL,
    UNIQUE (owner_id, line1, line2, line3)
);

CREATE TABLE accommodations
(
    id         UUID PRIMARY KEY     DEFAULT gen_random_uuid(),
    name       VARCHAR     NOT NULL,
    owner_id   UUID        NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    address_id UUID        NULL REFERENCES addresses (id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NULL,
    UNIQUE (owner_id, name)
);

CREATE TABLE trip_accommodation_booking
(
    id               UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    trip_id          UUID        NOT NULL REFERENCES trips (id) ON DELETE CASCADE,
    accommodation_id UUID        NOT NULL REFERENCES accommodations (id) ON DELETE CASCADE,
    check_in         TIMESTAMPTZ NOT NULL,
    check_out        TIMESTAMPTZ NOT NULL
);

CREATE TRIGGER set_updated_at
    BEFORE UPDATE
    ON accommodations
    FOR EACH ROW
EXECUTE FUNCTION update_modified_column();

CREATE TRIGGER set_updated_at
    BEFORE UPDATE
    ON addresses
    FOR EACH ROW
EXECUTE FUNCTION update_modified_column();