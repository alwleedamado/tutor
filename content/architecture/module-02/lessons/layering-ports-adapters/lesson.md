# Layering, Ports & Adapters

A layered service answers one question: **where does a change land?** The classic four layers:

```text
Transport      HTTP routes, Tauri commands, CLI parsing   (thin!)
Application    use-cases orchestrating domain + ports
Domain         entities, rules — pure, no I/O
Infrastructure DB drivers, HTTP clients, clocks, queues
```

## Ports & adapters

A **port** is an interface the application owns. An **adapter** is infrastructure implementing it.

```rust
// port — owned by the application layer
pub trait UserRepo { fn find(&self, id: u64) -> Option<User>; }

// adapter — infrastructure (rusqlite today, in-memory in tests)
pub struct SqliteUserRepo { conn: rusqlite::Connection }
impl UserRepo for SqliteUserRepo { /* ... */ }

// application — depends on the port, not the adapter
pub struct UserService<R: UserRepo> { repo: R }
```

The identical shape in Express:

```js
// port (interface by convention):  repo/userRepo.js
// adapter:                         repo/pgUserRepo.js
// service depends on the port:     userService(repo, req.body)
```

## Why business logic stays pure

Domain rules that never touch HTTP or SQL can be tested with plain unit tests, survive framework migrations, and be reused across transports — the same `UserService` behind REST and a Tauri command.

## Best practices

- Keep transport **thin**: parse, delegate, serialize. Zero business rules in handlers.
- Define ports in the *application* layer; infrastructure imports them, never the reverse.
- Start with 3-4 layers, not 10 — layers you cannot justify are walls, not boundaries.

## Common mistakes

- **Layer skip**: a handler doing SQL directly, bypassing application and domain.
- **Port explosion**: an interface per class; ports belong at I/O boundaries, not everywhere.
- **Leaky purity**: domain structs carrying serde attributes and DB column names everywhere — mapping happens *at the boundary*.
