# actix-web-sample

Sample project using actix-web

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

- APIs

  - Show User List: [GET] `/users`
  - Show User: [GET] `/users/{user_id}`
  - Add User: [POST] `/users`
  - Update User: [PUT] `/users/{user_id}`
  - Remove User: [DELETE] `/users/{user_id}`
  - Show Todos for Specific User: [GET] `/users/{user_id}/todos`
  - Add Todo: [POST] `/users/{user_id}/todos`
  - Update Todo for Specific User: [PUT] `/users/{user_id}/todos/{id}`
  - Remove Todo for Specific User: [DELETE] `/users/{user_id}/todos/{id}`

### Implementation

- [ ] Simple Logger
  - [actix_web::middleware::Logger](https://docs.rs/actix-web/latest/actix_web/middleware/struct.Logger.html)
- [ ] Connecting to Database using sea-orm
- [ ] Generator schema code using sea-orm

## Environment variables

| Item     | explanation | example |
| -------- | ----------- | ------- |
| RUST_LOG | log level   | info    |

## References

- [How to Work With TOML Files in Rust](https://www.makeuseof.com/working-with-toml-files-in-rust/)
