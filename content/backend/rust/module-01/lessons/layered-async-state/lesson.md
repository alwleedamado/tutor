# Async Services & Application State

Async Rust serves thousands of concurrent requests on a handful of threads — if you keep the executor unblocked and state sharing deliberate.

## Sharing state

```rust
use std::sync::{Arc, RwLock};

#[derive(Default)]
struct OrderRepo { rows: RwLock<Vec<(u64, String)>> }

struct AppState {
    repo: Arc<OrderRepo>,   // cheap clone per handler
}
```

The framework holds one `AppState`; each handler receives cheap `Arc` clones. No global mutable statics — state is injected, therefore testable.

## The four layers

```text
Transport       handlers, extractors        (thin)
Application     services: use-cases, tx boundaries
Domain          rules, types — pure, sync, no I/O
Infrastructure  pools, http clients, clock
```

## Async hazards

| Hazard | Why it hurts |
| --- | --- |
| `std::fs::read` in a handler | blocks the executor thread — every task on it stalls |
| lock guard held across `.await` | deadlock/starvation risk |
| unbounded `spawn` per request | task creation without limit → memory death |
| CPU-heavy work inline | starves other tasks; use `spawn_blocking` |

```rust
// ✅ async file IO keeps the executor free
let body = tokio::fs::read_to_string(path).await?;
// ✅ CPU-bound work off the executor
let hashed = tokio::task::spawn_blocking(move || hash(input)).await?;
```

## Best practices

- Domain stays synchronous and pure; async lives at the I/O edges.
- Locks: short, never across await; prefer message passing (channels) for hot paths.
- Every background task is bounded and cancellable (Module 5 makes this rigorous).

## Common mistakes

- Blocking calls in async context — the whole worker thread freezes.
- `Arc<Mutex<Everything>>` as a design pattern instead of careful ownership.
- Spawning tasks with no join/cancel path — leaks on every request.
