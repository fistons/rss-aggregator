# RSS Aggregator

RSS, but in rust

## Disclaimer

**This is still a prototype, not fit for production yet. The API is still subject to breaking change, without warning.
Use at our own risk!**

**Even if it is still a prototype, it's still open to issue, idea, any input.**

**This is my pet project to learn rust, so there is, and there will probably be a lot of bad code, but I'm here to learn
so if you want to highlight something, you're more than welcome!**

## Configuration

All the configuration must be pass through environment variables.

 * `DATABASE_URL` (required): The URL to the postgres database as `postgres://POSTGRES_USER:POSTGRES_PASSWORD@HOST:5432/rss-aggregator`
 * `REDIS_URL`: The redis URL as `redis://HOST`. Default `redis://locahost`
 * `JWT_SECRET`: String used as the key for JWT
 * `PASSWORD_SALT`: String used as a salt for password encoding
 * `RSS_AGGREGATOR_ALLOW_ACCOUNT_CREATION` true/false (default false): Allow user to register an account. Otherwise, an admin should do it
 * `POLLING_INTERVAL`: The number of seconds between feeds update. Default `300`
 * `OTEL_EXPORTER_JAEGER_AGENT_HOST`: Hostname/IP of the jaeger agent. Default `localhost`
 * `OTEL_EXPORTER_JAEGER_AGENT_PORT`: Port of the jaeger agent. Default `6831`
 * `RUST_LOG`: (error/warn/info/debug/trace) Log level. Default `info`

## What does it use

* `diesel` for the database migration since I have difficulties with SeaORM migration
* `SeaORM` for async database stuff
* `ActixWeb` for, well, the web stuff
* `Tracing/Jaeger` for logs and observability

## How to launch it quickly

### Using docker-compose

```shell
docker compose up
```

This will create the databases (postgres + redis), do the necessary migrations and launch the jaeger burrito

## How to init/migrate database

### Using docker

```shell
docker run --rm \
    -v "$PWD:/volume" \
    -w /volume \
    -e "DATABASE_URL=postgres://rss-aggregator:rss-aggregator@localhost/rss-aggregator" \
    -it clux/diesel-cli diesel migration run
```

### Using diesel cli directly

Requires rust and cargo 

```shell
cargo install diesel_cli --no-default-features --features postgres
# Time to take a coffee brake ???
DATABASE_URL="postgres://rss-aggregator:rss-aggregator@localhost/rss-aggregator" diesel migration run
```

## How to (re)build entities

Database migrate must be already done.

```shell
cargo install sea-orm-cli # if needed
DATABASE_URL="postgres://rss-aggregator:rss-aggregator@localhost/rss-aggregator" sea-orm-cli generate entity -o entity/src --with-serde both --expanded-format
```

Cf. SeaORM [documentation](https://www.sea-ql.org/SeaORM/docs/generate-entity/sea-orm-cli)
