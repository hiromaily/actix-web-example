# actix-web-sample

Sample Todo management web application based on Clean architecture using [actix-web](https://actix.rs/docs/) and [sea-orm](https://www.sea-ql.org/SeaORM/). Database, Hash algorithm are defined as traits object and these are initialized by Dependency Injection(DI). However dynamic dispatch has overhead and I'm not willing to use it in production. Dependencies must be salved by compiler by feature flag. I use several patterns in this code.

This API server generates OpenAPI spec using [apistos](https://crates.io/crates/apistos).

## Status

This repository is not maintained anymore due to that [various-web-frameworks-rust](https://github.com/hiromaily/various-web-frameworks-rust) is maintained for various web server including actix.

## Requirements

- Docker
- [hurl](https://github.com/Orange-OpenSource/hurl) for e2e
- [wrk](https://github.com/wg/wrk) for HTTP benchmarking
  - MacOS: `brew install wrk`
- [httpstat](https://github.com/davecheney/httpstat) for checking HTTP statistics

## How to run

### Configure toml

[toml configuration file](./config/settings.toml)

if toml.enabled is `true`, PostgreSQL must be run before running server.

```toml
[db]
enabled = true
```

### Run server

```sh
# run Postgresql if needed
docker compose up

# run server
make run

# run e2e
make req

# check OpenAPI
open http://127.0.0.1:8080/openapi.json
```

## TODO

### Designing

- Functionality

  - Admin
    - admin user login
    - Read/Add/Update/Delete user
    - Read all users
  - User
    - user login
    - Read/Add/Update/Delete todo-list for user

- Database schema

  - Users Table
    - id (Primary Key)
    - name (VARCHAR)
    - email (VARCHAR, unique)
    - password (VARCHAR)
  - Todos Table
    - id (Primary Key)
    - user_id (Foreign Key references users(id))
    - title (VARCHAR)
    - description (TEXT)
    - status (VARCHAR) // e.g., 'pending', 'completed'
    - created_at (TIMESTAMP)
    - updated_at (TIMESTAMP)

- APIs [/api/v1]
  - health
    - health check: [GET] `/health`
  - admin
    - admin login: [POST] `/admin/login`
    - Show User List: [GET] `/admin/users`
    - Show User: [GET] `/admin/users/{user_id}`
    - Add User: [POST] `/admin/users`
    - Update User: [PUT] `/admin/users/{user_id}`
    - Remove User: [DELETE] `/admin/users/{user_id}`
  - app
    - client login: [POST] `/app/login`
    - Show Todos for Specific User: [GET] `/app/users/{user_id}/todos`
    - Add Todo: [POST] `/app/users/{user_id}/todos`
    - Update Todo for Specific User: [PUT] `/app/users/{user_id}/todos/{id}`
    - Remove Todo for Specific User: [DELETE] `/app/users/{user_id}/todos/{id}`
- Pages [/]
  - admin
    - `/admin/`
    - `/admin/login`
  - app
    - `/app/`
    - `/app/login`

#### WIP: HTTP status code

- POST (Creating Data)
  - 201 Created: The request has been fulfilled, resulting in the creation of a new resource. Include a Location header to point to the URL of the new resource.
  - 400 Bad Request: The server could not understand the request due to invalid syntax.
  - 401 Unauthorized: The client must authenticate itself to get the requested response.
  - 403 Forbidden: The client does not have access rights to the content.
- GET (Retrieving Data)
  - 200 OK: The request was successful, and the server is returning the requested resource.
  - 400 Bad Request: The server could not understand the request due to invalid syntax.
  - 401 Unauthorized: The client must authenticate itself to get the requested response.
  - 403 Forbidden: The client does not have access rights to the content.
  - 404 Not Found: The server can not find the requested resource.
- PUT (Updating Data)
  - 200 OK: The request was successful, and the server is returning the updated resource.
  - 400 Bad Request: The server could not understand the request due to invalid syntax.
  - 401 Unauthorized: The client must authenticate itself to get the requested response.
  - 403 Forbidden: The client does not have access rights to the content.
  - 404 Not Found: The server can not find the requested resource.
- DELETE (Deleting Data)
  - 204 No Content: The request was successful, but the server is not returning any content.
  - 400 Bad Request: The server could not understand the request due to invalid syntax.
  - 401 Unauthorized: The client must authenticate itself to get the requested response.
  - 403 Forbidden: The client does not have access rights to the content.
  - 404 Not Found: The server can not find the requested resource.
- General Error Codes
  - 500 Internal Server Error: The server has encountered a situation it doesn't know how to handle.

### Implementation

- [x] Simple Logger
  - [actix_web::middleware::Logger](https://docs.rs/actix-web/latest/actix_web/middleware/struct.Logger.html)
- [x] Set router on app server
- [x] CORS
- [x] Generator schema code using sea-orm
- [x] Create handler layer
- [x] Create `usecase` layer as trait object (it must be abstract type)
- [x] Connecting to Database using sea-orm
- [x] Create repositories for PostgreSQL users implementation
- [x] Create repositories for PostgreSQL todos implementation
- [x] Implement admin usecase
- [x] Implement app usecase
- [x] Hash implementation and hash password before inserting, searching
- [x] TodoStatus enum generated by sea-orm cli must implement `from_str()`.
- [x] set timeout for connecting database when not running
- [x] JWT craits, token response after login
- [x] middleware for jwt authentication
- [x] refactoring auth usecase decoupling from admin/app states. establish auth state.
- [x] admin role can access any user's todo, but user role is limited to only own todo.
- [x] e2e by [hurl](https://hurl.dev/)
- [x] exporting OpenAPI documentation
- [x] OpenAPI
- [x] more implementation for jwt and configuration on toml
- [x] more implementation for hash and configuration on toml
- [x] feature flag must be integrated into DI functionality
- [x] Unittest / Benchmark
- [x] CI environment
- [ ] Create repositories for on memory implementation using [airone](https://crates.io/crates/airone). This feature is required for CI.
- [ ] HTTP 2 / TLS(Transport Layer Security)
- [x] ~~another pluggable option of framework like [axum](https://github.com/tokio-rs/axum)~~

## Tech Stacks

### Web framework: [Actix web](https://actix.rs/)

- [official docs](https://actix.rs/docs/)
- [crates.io: actix-web](https://crates.io/crates/actix-web)

### ORM [sea-orm](https://www.sea-ql.org/SeaORM/)

- [crates.io: sea-orm](https://crates.io/crates/sea-orm)
- [sea-orm-cli](https://www.sea-ql.org/sea-orm-tutorial/ch01-04-entity-generation.html)
- [Database & Async Runtime](https://www.sea-ql.org/SeaORM/docs/install-and-config/database-and-async-runtime/)

### Hash algorithm

Refer to [RustCrypto/password-hashes](https://github.com/RustCrypto/password-hashes)

- [pbkdf2](https://crates.io/crates/pbkdf2)
  - 100k dl per day
- [scrypt](https://crates.io/crates/scrypt)
  - 23k dl per day
  - somehow too slow
- [argon2](https://crates.io/crates/argon2)
  - 10k dl per day
- [bcrypt](https://crates.io/crates/bcrypt)
  - 7.5k dl per day

### JWT

- [jsonwebtoken](https://crates.io/crates/jsonwebtoken)
  - 90k dl per day, upper trend
- [jwt-simple](https://crates.io/crates/jwt-simple)
  - 10k dl per day, upper trend
  - somehow `v0.12` can't build and development is stopped since then 2023
- [smpl_jwt](https://crates.io/crates/smpl_jwt)
  - 10k dl per day
- [jwt](https://crates.io/crates/jwt)
  - 5k dl per day

### OpenAPI Documentation (not code generation for server)

After running, access to [openapi.json](http://127.0.0.1:8080/openapi.json)

#### [apistos](https://crates.io/crates/apistos)

- [github: apistos](https://github.com/netwo-io/apistos)
- [example](https://github.com/netwo-io/apistos/tree/main/examples)
  - [example: rust-farm/gcc-api](https://github.com/buraksenyurt/rust-farm/blob/2b8ac3cb410ee10868b0e12df2724b734b7d8dfc/Practices/green-code-challenge/gcc-api/src/main.rs#L9)
- [Documenting API for Actix-web](https://medium.com/netwo/documenting-api-for-actix-web-b575adb841a1)

## Environment variables

| Item     | explanation | example |
| -------- | ----------- | ------- |
| RUST_LOG | log level   | info    |

## References to develop

- [How to Work With TOML Files in Rust](https://www.makeuseof.com/working-with-toml-files-in-rust/)
- [How to pass a Trait object via app_data to Actix Web?](https://users.rust-lang.org/t/how-to-pass-a-trait-object-via-app-data-to-actix-web/79096)
- [actix-web で Data<dyn trait> を使い回す](https://teratail.com/questions/kb8b224km8a6hl)
- [Struct actix_web::web::Data](https://docs.rs/actix-web/latest/actix_web/web/struct.Data.html)
- [Should handlers be functions only?](https://github.com/actix/actix-web/discussions/2321)
  - [sod-actix-web](https://github.com/thill/sod-actix-web/tree/main)
- [Resolving not-object-safe error with trait having async methods](https://users.rust-lang.org/t/resolving-not-object-safe-error-with-trait-having-async-methods/105175)
