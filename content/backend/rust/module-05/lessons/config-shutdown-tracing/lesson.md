# Configuration, Graceful Shutdown & Tracing

Production readiness is three habits: configuration that fails fast, shutdowns that drain instead of dropping, and logs you can correlate.

## Configuration: layered, validated, fail-fast

```text
defaults ← config file ← environment variables   (later layers win)
```

Validate at boot: types, ranges, required-present. A service that starts with a missing `DATABASE_URL` and fails on first request fails at the worst possible moment. Fail in 50ms at startup instead.

## Graceful shutdown

Orchestrators (and users) stop processes. The contract with SIGTERM:

```text
1. receive SIGTERM
2. stop accepting new connections (close listener)
3. finish in-flight requests (bounded by a shutdown timeout, e.g. 30s)
4. flush logs/metrics, exit 0
```

```rust
// concept
loop {
    tokio::select! {
        conn = listener.accept() => handle(conn),
        _ = shutdown_signal()    => break,     // SIGTERM
    }
}
// drain, then exit
```

Without this, every deploy drops requests mid-flight; with it, deploys are a non-event. Health checks flip to "not ready" first so load balancers stop sending traffic before the drain starts.

## Tracing: structured and correlated

```text
INFO order_service: order created  request_id=abc route=POST /orders ms=42 user=17
```

- **Structured** (JSON in production) — logs are queryable data, not prose.
- **Correlated** — a `request_id` span wraps the request; every line inside carries it. One user complaint → one grep → the whole story.
- Level discipline: ERROR (needs action), WARN (degraded), INFO (state changes), DEBUG (developer detail, off in prod).

## Best practices

- Log state *changes* and outcomes at INFO; never per-row spam.
- Include ids, not payloads — logs are read by humans under pressure and stored by systems with retention.

## Common mistakes

- `println!` logging with no ids — impossible to correlate a request across services.
- Catching SIGTERM with a hard exit — in-flight uploads die on every deploy.
- Secrets in config logged "for debugging".
