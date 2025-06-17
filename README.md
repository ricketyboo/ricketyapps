# Rust Full Stack Experiments

This is an experimental repo that will likely be cloned off into a different project at some point. I'm using
this to explore a bunch of technology I don't get to use in my day job and will eventually turn this sandbox into a
foundation.

Initial attempts were assembled from various references including examples and projects in the leptos-rs core repo,
but I'm pretty much off script now. The current project structure is a work in progress as I experiment with
different ideas and structures that work for me as I learn the language and frameworks. It will continue to be
refactored as I get more familiar with the quirks and expectations of this tech stack.

Key Stack Elements

- Rust (Nightly, for those sweet signal accessor functions)
- Leptos (Because I love SolidJS, and this is like that, in Rust )
- Axum (A lot of back and forth on Actix or Axum, but I like the way Axum feels more)
- SQLX + Welds (Welds is a very young project but it feels like a nice middleground between raw SQLx and the way too
  much going on impression I got when exploring SeaORM. Go give Welds a look!)
- PostgreSQL (Because I haven't used it in quite a while and I need to get those gears turning again )

Visually this is not very exciting as yet. This has not been the focus. https://open-props.style/ is in play just
to make it look slightly less bad.

There is the ghost of an attempt at making a trip planner in the commit history, but that will likely not resurface due
to the complexity involved in making that app actually useful.
Instead, this is most likely going to turn into a very specialised task tracker, microjournaling app, and personal
relationship/events tracker. An app I've wanted for years but no-one's every really made the one that I actually need.

What works?

- [x] Runtime DB migrations
- [x] User registration and password hashing with Argon2id
- [x] Cookie Based Auth Sessions
- [X] Protected UI Routes (Anon and Authenticated)

What's next?

- [ ] Unit tests for auth endpoints
- [ ] Actual application functionality!
- [ ] Protected API Routes
- [ ] Authorisation and Roles for private/shared resources among multiple users
- [ ] Mobile/Desktop app and code reuse either using Tauri (easier) or Slint (performance!)

Other things I want to experiment with but may not be suited to this project

- [ ] Notification services (Email/IM/Push)
- [ ] Scheduled tasks (ie Cron)
- [ ] OpenTelemetry
- [ ] microservice split with https://github.com/spinframework/spin

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
CREATE DATABASE rickety_apps_dev;
```

Reconnect to newly created database and add extensions.

```postgresql
-- enable https://www.postgresql.org/docs/current/contrib-spi.html#CONTRIB-SPI-MODDATETIME
CREATE EXTENSION IF NOT EXISTS moddatetime;
```

```postgresql
-- create user for the app_helpers
CREATE USER rickety_apps WITH password 'rickety_apps_pass';
GRANT ALL PRIVILEGES ON DATABASE rickety_apps_dev TO rickety_apps;
ALTER DATABASE rickety_apps_dev OWNER TO rickety_apps;
-- check our user was created
SELECT usename, usesysid
FROM pg_user
WHERE usename = 'rickety_apps';
```

Reconnect with app credentials.

Create environment file

```text
DB_USERNAME=rickety_apps
DB_PASSWORD=rickety_apps_pass
DB_HOST=127.0.0.1
DB_PORT=5432
DB_NAME=rickety_apps_dev

# SQLX
DATABASE_URL=postgres://${DB_USERNAME}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}
```

## Subsequent runs

```shell
podman start postgres
cargo leptos watch
```
