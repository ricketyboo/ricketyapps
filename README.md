# Rust Full Stack Experiments

Learning the stack. This is an experimental repo that will be turning into a different project eventually.

Assembling from various references including example and projects folders in the leptos-rs core repo.

The current project structure is not as clean as I'd like, and will be refactored once I'm out of experimenting stage and into actual knowing how the pieces fit stage.   

Current version:

- Rust (Nightly)
- SQLX
- PostgreSQL
- Axum
- Leptos

Previous version was using Welds and was API only.  

## Initial dev environment setup

Install tooling

```shell
rustup target add wasm32-unknown-unknown
cargo install sqlx-cli
cargo install --locked cargo-leptos
```

```shell
podman pull postgres
podman run -d --name postgres --env POSTGRES_USER=postgres --env POSTGRES_PASSWORD=password  -p 5432:5432 -v pg-data:/var/lib/postgresql/data postgres
podman logs postgres
```

Connect to postgres and create the new app database.

```postgresql
CREATE DATABASE planner_dev;
```

Reconnect to newly created database and add extensions.

```postgresql
-- enable https://www.postgresql.org/docs/current/contrib-spi.html#CONTRIB-SPI-MODDATETIME
CREATE EXTENSION IF NOT EXISTS moddatetime;
```

```postgresql
-- create user for the app_helpers
CREATE USER planner WITH password 'planner_pass';
GRANT ALL PRIVILEGES ON DATABASE planner_dev TO planner;
ALTER DATABASE planner_dev OWNER TO planner;
-- check our user was created
SELECT usename, usesysid
FROM pg_user
WHERE usename = 'planner';
```

Reconnect with app credentials.

Create environment file

```text
DB_USERNAME=planner
DB_PASSWORD=planner_pass
DB_HOST=127.0.0.1
DB_PORT=5432
DB_NAME=planner_dev

# SQLX
DATABASE_URL=postgres://${DB_USERNAME}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}
```

## Subsequent runs

```shell
podman start postgres
cargo leptos watch
```