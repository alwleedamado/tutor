# Pagination, Filtering & Idempotency

Three production API essentials that are easy to demo and easy to get wrong at scale.

## Cursor over offset

```sql
-- offset: page 5001 walks 100,000 rows first
SELECT * FROM orders ORDER BY created_at DESC LIMIT 20 OFFSET 100000;

-- keyset: starts exactly at the cursor, constant cost
SELECT * FROM orders
 WHERE (created_at, id) < ($1, $2)
 ORDER BY created_at DESC, id DESC
 LIMIT 20;
```

Offset pages also *shift* when rows are inserted (duplicate/missing rows). Cursors need a stable, unique sort key — `(created_at, id)`, not `created_at` alone.

## Filtering and sorting safely

Filters arrive as validated, typed parameters and become **parameterized** query fragments — never string-concatenated SQL:

```rust
// builder concept: WHERE clause built from whitelisted keys
let allowed = ["status", "customer_id"];
// unknown key -> 400, not interpolation
```

Sorting keys are whitelisted too (`sort=created_at`, never arbitrary column names interpolated).

## Idempotency

Networks retry. A `POST /charge` retried after a timeout must not charge twice:

```text
client sends: Idempotency-Key: k1
server:       first time → execute, store (key → result)
              repeat    → return stored result, do NOT execute again
```

Store keys with a TTL (24h is typical) scoped per operation; a repeated key with a *different* payload is a 422.

## Best practices

- Page size caps (`?limit=` clamped server-side) — clients can request 1,000,000.
- Every list endpoint documents its cursor format and default order.
- Idempotency on all POSTs with side effects (payments, creations).

## Common mistakes

- Trusting client-provided `offset` for deep pagination and wondering why p95 dies.
- Building SQL with `format!()` from query params — injection waiting to happen.
- Retrying POSTs without idempotency keys and double-charging real users.
