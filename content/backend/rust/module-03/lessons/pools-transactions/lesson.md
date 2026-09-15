# Connection Pools & Transactions

Database connections are expensive (handshake, auth, memory). Pools amortize them; transactions make multi-step writes atomic. Both are service-layer concerns.

## Pool discipline

```text
size:        ≈ expected concurrent DB work (not total users!)
             Postgres: (cores × 2-4) is a ceiling, not a floor — measure
health:      idle timeout, connection lifetime (cloud LBs kill old conns)
acquire:     timeout + metric — pool waits are your early warning
```

The cardinal rule: **never hold a connection across slow external I/O**. Calling a payment API while holding a pool slot turns one slow vendor into zero DB capacity.

## Transaction boundaries live in services

```rust
// service — owns the atomic boundary
async fn transfer(&self, from: u64, to: u64, cents: i64) -> Result<(), AppError> {
    let mut tx = self.pool.begin().await?;
    self.repo.debit(&mut tx, from, cents).await?;
    self.repo.credit(&mut tx, to, cents).await?;
    self.repo.audit_log(&mut tx, from, to, cents).await?;
    tx.commit().await?;      // all or nothing
    Ok(())
}
```

Repositories take `&mut tx` and execute within it — they do not open their own. The use-case defines what "all or nothing" means.

## Isolation, chosen deliberately

Default (Read Committed) fits most workloads. Serializable only where money demands it — and handle serialization failures by retrying. Higher isolation trades concurrency for consistency; pick per use-case, not globally.

## Best practices

- One transaction per use-case; keep its body short and DB-only (no HTTP inside).
- Health checks `SELECT 1` through the pool — proving the pool, not the DB alone.
- Emit pool stats (waiting, in-use) as metrics; exhaustion announces itself there first.

## Common mistakes

- Pool sized to user count (10k users ≠ 10k connections — the DB dies first).
- Repos managing their own transactions → three commits that should be one.
- `SELECT 1` liveness while every pooled connection is stuck on locks.
