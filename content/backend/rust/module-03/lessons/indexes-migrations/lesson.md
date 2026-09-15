# Indexes, Constraints & Migrations

Schemas are not set-and-forget. Three disciplines keep them healthy under a live service: indexes that match queries, constraints as the last line of defense, and migrations that never take you offline.

## Index the queries you actually run

```sql
-- query: recent orders per customer
SELECT * FROM orders WHERE customer_id = $1 ORDER BY created_at DESC LIMIT 20;
-- index that serves it exactly:
CREATE INDEX idx_orders_customer_recent ON orders (customer_id, created_at DESC);
```

Read `EXPLAIN QUERY PLAN`: `SCAN` on a big table = missing index; `SEARCH USING INDEX` = served. Indexes speed reads and tax writes — add them from observed patterns, not every column.

## Constraints are the last line of defense

```sql
ALTER TABLE orders ADD CONSTRAINT total_nonneg CHECK (total_cents >= 0);
ALTER TABLE orders ADD CONSTRAINT fk_customer
  FOREIGN KEY (customer_id) REFERENCES customers(id);
```

Application validation has bugs; constraints don't. Unique constraints also make retry logic possible (`ON CONFLICT DO NOTHING` instead of check-then-insert races).

## Zero-downtime migrations: expand → contract

Deploying a `NOT NULL` column in one step locks the table and breaks the running version. Instead, across two releases:

```text
release 1 (expand): add nullable column; deploy code writing both old+new
         (online):  backfill in batches
release 2 (contract): enforce NOT NULL; drop the old column
```

Every intermediate state runs with both the old and the new code.

## Best practices

- Migrations are versioned, ordered, forward-only files in the repo (this app's own SQLite migrations do exactly that).
- Batch backfills (`WHERE id BETWEEN ... LIMIT 10k`) to keep transactions short.
- Test migrations against a production-shaped database, not an empty one.

## Common mistakes

- One giant migration locking a hot table for minutes.
- Adding indexes "for later" — every one taxes every write.
- Editing an already-shipped migration instead of adding a new one.
