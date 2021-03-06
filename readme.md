# Todo-actix

This is a simple todo server application to showcase the integration of the following technologies:

- [rust](https://www.rust-lang.org/): the language used for this project
- [actix-web](https://actix.rs/docs/actix-web/): the web framework used for this project
- [utoipa](https://github.com/juhaku/utoipa): the openapi specification generator used for this project
- [utoipa-swagger-ui](https://github.com/juhaku/utoipa): the swagger ui generator used for this project
- [sea-orm](https://www.sea-ql.org/SeaORM/docs/index): the ORM and database migration management tool used for this project
- [postgres](https://www.postgresql.org/): the database used for this project
- [serde](https://serde.rs/): the serialization library used for this project
- [dotenv](https://github.com/dotenv-rs/dotenv): the environment variable management library used for this project
- [config](https://docs.rs/config/latest/config/): the configuration management library used for this project
- [tracing](https://docs.rs/tracing/latest/tracing/): the tracing library used for this project
- [opentelemtry](https://opentelemetry.io/): the open telemetry library used for this project
- [sentry](https://docs.rs/sentry/latest/sentry/): the error reporting library used for this project
- [consul](https://www.consul.io/): the service discovery library used for this project

Next things I would like to do:

- [x] Add production grade logging
- [x] Add Sentry error reporting
- [x] Add consul service registration
- [ ] Add an endpoint listing todos with apache arrow
- [ ] Add some tests
- [ ] Look into `anyhow` for error handling
- [ ] Make a gRPC version of the API
- [ ] Revisit the project structure once it has been more than a few months and adjust it accordingly

## Why actix-web?

- It's the most mature and stable web framework in the Rust ecosystem.
- It has the most contributors.
- Supports web socket and HTTP/2.
- Can be used with utoipa to generate OpenAPI specifications.
- Most performant rust web framework according to TechEmpower Web Framewok Benchmarks round 19 and second to best (by 0.4 %) on round 20.
- Sentry integration.

## Why SeaORM?

- Complete solution in interfacing with databases
- Acts as a both a database ORM and a migrations management tool at the same time.
- Not a lot of migrations management tool in Rust (Diesel is the only alternative).
- Pure rust driver for PostgreSQL.

Read more @ https://www.sea-ql.org/SeaORM/docs/internal-design/diesel

## Starting DB, Jaeger UI (Visualize logs) and Consul (Service Discovery)

```sh
$ docker-compose up
```

## Create 'todo-actix' database

```sh
$ docker-compose exec postgres bash -c "psql -U postgres -tc \"SELECT 1 FROM pg_database WHERE datname = 'todo-actix'\" | grep -q 1" || docker-compose exec postgres bash -c "createdb -U postgres 'todo-actix'"
```

## Running migrations

```sh
$ cd migrations
$ cargo run
```

## Running in development mode

```sh
$ cargo run
```

Run in watch mode (cargo-watch is required):

```sh
cargo watch -x 'run --bin todo-actix'
```

## Usage

The api can be explored and tested out in the browser through the [swagger-ui](https://swagger.io/swagger-ui/) interface available at http://127.0.0.1:8080/docs/. The logs can be viewed in the [Jaeger UI](http://localhost:16686/) interface available at http://localhost:16686/search?service=todo-actix. Consul is used to register the service with the consul agent. It can be viewed in the [Consul UI](http://localhost:8500/).

### List Todos:

```sh
$ curl -sv -X 'GET' http://127.0.0.1:8080/v1/todo/list?page=1&todos_per_page=10 | jq
>
> GET /v1/todo/list HTTP/1.1
> Host: 127.0.0.1:8080
> User-Agent: curl/7.68.0
> Accept: */*
>
< HTTP/1.1 200 OK
< content-length: 108
< content-type: application/json
< date: Sat, 25 Jun 2022 16:17:58 GMT
<
{
  "todos": [
    {
      "id": 1,
      "title": "First Todo ever!",
      "completed": false
    }
  ],
  "page": 1,
  "todos_per_page": 10,
  "num_pages": 1
}
```

### Get Todo:

```sh
$ Coming soon...
```

### Create Todo:

```sh
$ curl -sv -X 'POST' \
  'http://localhost:8080/v1/todo/create' \
  -H 'Content-Type: application/json' \
  -d '{
  "title": "First Todo ever!"
}' | jq
> POST /v1/todo/create HTTP/1.1
> Host: localhost:8080
> User-Agent: curl/7.68.0
> Accept: */*
> Content-Type: application/json
> Content-Length: 34
>
< HTTP/1.1 201 Created
< content-length: 63
< content-type: application/json
< date: Sat, 25 Jun 2022 16:23:09 GMT
<
{
  "todo": {
    "id": 1,
    "title": "First Todo ever!",
    "completed": false
  }
}
```

### Update Todo:

```sh
$ coming soon...
```

### Delete Todo:

```sh
$ coming soon...
```

### Healthcheck:

```sh
$ curl -sv -X 'GET' http://127.0.0.1:8080/healthcheck | jq
> GET /healthcheck HTTP/1.1
> Host: 127.0.0.1:8080
> User-Agent: curl/7.79.1
> Accept: */*
>
* Mark bundle as not supporting multiuse
< HTTP/1.1 200 OK
< content-length: 22
< content-type: application/json
< date: Sun, 26 Jun 2022 19:45:37 GMT
{
  "status": "Available"
}
```

## Building

### Dev

```sh
$ cargo build
```

### Release

```sh
$ cargo build --release
```

Read more @ https://doc.rust-lang.org/book/ch14-00-more-about-cargo.html.
