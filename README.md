<<<<<<< HEAD
# customer-service-rust
=======
# customer-service

A REST microservice for managing customers, written in Rust with [summer-rs](https://spring-rs.github.io/), a Spring Boot-inspired application framework. It is a learning project that maps Spring Boot concepts (plugins, dependency injection, externalized configuration, layered design) onto the Rust ecosystem.

## Tech stack

| Concern | Choice |
|---|---|
| Language | Rust (stable) |
| Framework | summer-rs 0.5.x (plugin-based, built on axum and Tokio) |
| HTTP | `summer-web` (axum) |
| Database access | `summer-sqlx` (sqlx) with PostgreSQL |
| Validation | `validator` |
| Error handling | `thiserror` + `anyhow`, mapped to HTTP responses via axum `IntoResponse` |
| Logging | `tracing` |
| Configuration | TOML (`config/app.toml`) |

## API

Base path: `/api/v1`

| Method | Path | Description | Success | Errors | Status |
|---|---|---|---|---|---|
| POST | `/api/v1/customers` | Create a customer | 201 | 400, 409 | Implemented |
| GET | `/api/v1/customers/{id}` | Get a customer by id | 200 | 404 | Implemented |
| GET | `/api/v1/customers` | Get a List of customers | 200 | 404 | Implemented |
| GET | `/health` | Readiness check (runs `SELECT 1`) | 200 | 500 | Implemented |
| GET | `/api/v1/customers?page=&size=` | List customers (paged) | 200 | 400 | Planned |
| PUT | `/api/v1/customers/{id}` | Update a customer | 200 | 400, 404, 409 | Planned |
| DELETE | `/api/v1/customers/{id}` | Delete a customer | 204 | 404 | Planned |

### Request body (create / update)

```json
{
  "name": "Jane Doe",
  "email": "jane@example.com"
}
```

Validation rules:
- `name`: 1 to 100 characters
- `email`: must be a valid email address and unique

### Response body

```json
{
  "id": 1,
  "name": "Jane Doe",
  "email": "jane@example.com"
}
```

### Error format

All errors return a JSON body with a stable code and a message:

```json
{
  "code": "NOT_FOUND",
  "message": "customer 42 not found"
}
```

| HTTP status | `code` | When |
|---|---|---|
| 400 | `VALIDATION_ERROR` | Request body fails validation |
| 404 | `NOT_FOUND` | Customer does not exist |
| 409 | `CONFLICT` | Email already exists (unique constraint) |
| 500 | `INTERNAL_ERROR` | Unexpected failure; details are logged, never returned to the client |

### Pagination

`GET /api/v1/customers` accepts `page` (0-based, default 0) and `size` (default 20, capped at 100). Results are ordered by `id`.

## Project structure

```
customer-service/
├── Cargo.toml
├── config/
│   └── app.toml          # web port and database connection
├── migrations/           # SQL migrations (sqlx)
└── src/
    ├── main.rs           # App bootstrap: registers plugins
    ├── routes.rs         # HTTP handlers (controller layer)
    ├── service.rs        # Business logic and database access
    ├── model.rs          # Entities, request DTOs, pagination params
    └── error.rs          # AppError and its HTTP response mapping
```

Request flow: `routes` (extract + validate) → `service` (query the database) → `error` (convert failures to HTTP responses).

## Prerequisites

- [Rust](https://rustup.rs/) (stable toolchain)
- PostgreSQL 14 or later (or Docker)
- [`sqlx-cli`](https://crates.io/crates/sqlx-cli) for migrations:
  ```bash
  cargo install sqlx-cli --no-default-features --features postgres
  ```

On Windows, building in WSL or a container can avoid linker and toolchain issues.

## Getting started

### 1. Start PostgreSQL

Using Docker:

```bash
docker run --name customers-db \
  -e POSTGRES_USER=app \
  -e POSTGRES_PASSWORD=secret \
  -e POSTGRES_DB=customers \
  -p 5432:5432 -d postgres:16
```

### 2. Configure

Edit `config/app.toml`:

```toml
[web]
port = 8080

[sqlx]
uri = "postgres://app:secret@localhost:5432/customers"
```

Do not commit real credentials. Use environment-specific config files or environment variable overrides for anything beyond local development (see the spring-rs docs on configuration).

### 3. Run migrations

```bash
export DATABASE_URL=postgres://app:secret@localhost:5432/customers
sqlx migrate run
```

On PowerShell: `$env:DATABASE_URL="postgres://app:secret@localhost:5432/customers"`

The initial migration creates:

```sql
CREATE TABLE customers (
    id         BIGSERIAL PRIMARY KEY,
    name       TEXT NOT NULL,
    email      TEXT NOT NULL UNIQUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
```

### 4. Run the service

```bash
cargo run
```

The first build compiles all dependencies and can take several minutes. Later builds are much faster.

### 5. Try it

```bash
# Create
curl -i -X POST http://localhost:8080/api/v1/customers \
  -H "Content-Type: application/json" \
  -d '{"name":"Jane Doe","email":"jane@example.com"}'

# Get
curl -i http://localhost:8080/api/v1/customers/1

# Health
curl -i http://localhost:8080/health
```

On Windows PowerShell, use `curl.exe` (plain `curl` is an alias for `Invoke-WebRequest`) or `Invoke-RestMethod`.

## Development

| Command | Purpose |
|---|---|
| `cargo check` | Fast type-check without building a binary (use this most) |
| `cargo run` | Build and start the service |
| `cargo fmt` | Format code |
| `cargo clippy` | Lint |
| `cargo test` | Run tests |
| `cargo build --release` | Optimized build for deployment |

## Roadmap

- [ ] Integration tests with Testcontainers (PostgreSQL) and `reqwest`
- [ ] JWT authentication middleware
- [ ] Structured request logging and OpenTelemetry tracing (`spring-opentelemetry`)
- [ ] Liveness endpoint separate from readiness
- [ ] Multi-stage Dockerfile (distroless runtime image)
- [ ] Kubernetes / OpenShift deployment manifests with probes
- [ ] OpenAPI documentation
>>>>>>> 848623e (initial project baseline)
