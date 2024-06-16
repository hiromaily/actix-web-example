# actix-web-sample

Sample Todo management web application based on Clean architecture using [actix-web](https://actix.rs/docs/) and [sea-orm](https://www.sea-ql.org/SeaORM/).

## Status

WIP

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

### Implementation

- [x] Simple Logger
  - [actix_web::middleware::Logger](https://docs.rs/actix-web/latest/actix_web/middleware/struct.Logger.html)
- [x] Set router on app server
- [x] CORS
- [x] Generator schema code using sea-orm
- [x] Create handler
- [x] Create `usecase` layer as trait object (it must be abstract type)
- [x] Connecting to Database using sea-orm
- [ ] Create repositories for PostgreSQL implementation
- [ ] Create repositories for on memory implementation
- [ ] Hash password before inserting, searching
- [ ] JWT
- [ ] Create handlers for html response
- [ ] Unittest / Benchmark
- [ ] CI environment
- [ ] Fix lint error

## About [sea-orm](https://www.sea-ql.org/SeaORM/)

read documents.

- [Database & Async Runtime](https://www.sea-ql.org/SeaORM/docs/install-and-config/database-and-async-runtime/)
- [crates.io: sea-orm](https://crates.io/crates/sea-orm)
- [sea-orm-cli](https://www.sea-ql.org/sea-orm-tutorial/ch01-04-entity-generation.html)

## Environment variables

| Item     | explanation | example |
| -------- | ----------- | ------- |
| RUST_LOG | log level   | info    |

## References

- [How to Work With TOML Files in Rust](https://www.makeuseof.com/working-with-toml-files-in-rust/)
- [How to pass a Trait object via app_data to Actix Web?](https://users.rust-lang.org/t/how-to-pass-a-trait-object-via-app-data-to-actix-web/79096)
- [actix-web で Data<dyn trait> を使い回す](https://teratail.com/questions/kb8b224km8a6hl)
- [Struct actix_web::web::Data](https://docs.rs/actix-web/latest/actix_web/web/struct.Data.html)
- [Should handlers be functions only?](https://github.com/actix/actix-web/discussions/2321)
  - [sod-actix-web](https://github.com/thill/sod-actix-web/tree/main)
- [Resolving not-object-safe error with trait having async methods](https://users.rust-lang.org/t/resolving-not-object-safe-error-with-trait-having-async-methods/105175)
