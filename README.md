## Initial dev environment setup

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
-- create user for the app
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
podman logs postgres
```