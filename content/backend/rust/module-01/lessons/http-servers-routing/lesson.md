# HTTP Servers, Routing & the Request Lifecycle

A Rust backend service follows one shape: a **router** maps `(method, path)` to a **handler**; the handler parses input, delegates to services, and shapes a response. Frameworks (axum, actix) package this; the ideas are framework-free.

## The lifecycle

```text
socket → parse request → middleware (outer→inner) → router match
       → extractors (Path/Json/State) → handler → service → response
       → middleware (inner→outer) → serialize → socket
```

## The shape (axum-style)

```rust
// Router::new()
//   .route("/orders/:id", get(order_by_id))
//   .layer(TraceLayer)                 // middleware onion
//   .with_state(AppState { db })
//
// async fn order_by_id(Path(id): Path<u64>, State(st): State<AppState>)
//     -> Result<Json<Order>, AppError> {
//     let order = st.service.find(id).await?;  // delegate, don't implement
//     Ok(Json(order))
// }
```

Extractors declare what a handler needs; the router wires it. The handler stays a thin adapter — parse, delegate, respond.

## Middleware is an onion

Each layer wraps the next: a tracing layer logs the request before and the response after; an auth layer can reject before the handler runs at all.

```text
request → [trace → auth → handler] → response
```

## Best practices

- Handlers: no SQL, no business rules — parse, delegate, serialize.
- Routing table in one module; handlers grouped per feature.
- Reject bad input at the extractor boundary (typed paths, validated DTOs).

## Common mistakes

- Business logic in handlers — now it can't be reused or unit-tested without HTTP.
- One giant `match` router with no nesting — grows into a 500-line match.
- Doing heavy blocking work in the handler instead of `spawn_blocking`/async I/O.
