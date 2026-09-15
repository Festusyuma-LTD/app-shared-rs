# shared

Library crate providing the common `axum` error and response types a service crate
returns, so a consuming app only needs to integrate with one set of structs instead of
a bespoke pair per service.

## What it provides

- **`ServiceError`**: the handful of HTTP error shapes a service needs to return —
  `NotFound`, `ServerError`, `Http(code)`, `HttpMessage(code, message)` — with
  `IntoResponse` so it can be returned directly from an `axum` handler as
  `{ "error": "..." }` JSON. `From<anyhow::Error>` collapses any anyhow error into a
  logged `ServerError`.
- **`ServiceResponse<T>`**: wraps a handler's success value (`Ok`/`Status`) or a
  `ServiceError`, rendering either as `{ "data": ... }` or `{ "error": ... }` JSON.
- **`ServiceResult<T>`**: `Result<T, ServiceError>` — the return type service functions
  use internally, convertible into a `ServiceResponse<T>` at the controller boundary.

## Usage

```rust
use shared::error::ServiceError;
use shared::response::{ServiceResponse, ServiceResult};

fn fetch_thing() -> ServiceResult<Thing> {
    Err(ServiceError::HttpMessage(404, "not found".into()))
}

async fn get_thing() -> ServiceResponse<Thing> {
    fetch_thing().into() // ServiceResult<Thing> -> ServiceResponse<Thing>
}
```
