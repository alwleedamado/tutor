# Fixtures, Isolation & Flaky Tests

A test you cannot trust is worse than no test: it trains the team to ignore red. Trust comes from **isolation** — every test starts from the same known state.

## Fixtures and factories

- **Factories** create the data a test needs with sensible defaults:

```rust
fn user(name: &str) -> User { User { id: 1, name: name.into(), active: true } }
// tests override only what they care about
let admin = User { role: Role::Admin, ..user("ada") };
```

- **Transactional fixtures** wrap each DB test in a transaction and roll back:

```sql
BEGIN;
-- test sees a fresh database every time
ROLLBACK;
```

## Order independence

A suite must pass in any order, in parallel, and on a fresh machine. Shared mutable fixtures (a module-level `static Vec`) make green suites that break on CI at 3 a.m.

## Diagnosing flakes: the big four

1. **Time** — real clocks, sleeps, midnight/timezone edges. Inject a clock; never `sleep` in tests.
2. **Order/state** — passes alone, fails in the suite. Randomize order locally to reproduce.
3. **Network/IO** — real HTTP in unit tests. Cut the wire at the port boundary.
4. **Concurrency** — race in async code. Reduce to a deterministic interleaving or assert with timeouts.

```ts
// ❌ flaky: depends on wall-clock and sleep timing
await sleep(150)
// ✅ deterministic: await the observable state
await waitFor(() => expect(screen.getByRole('status')).toHaveTextContent('saved'))
```

## Best practices

- Defaults in factories encode invariants; tests override only the relevant field.
- Quarantine flaky tests immediately, then fix the cause — never "retry until green".
- Cleanup belongs in teardown hooks, not the next test's setup.

## Common mistakes

- `sleep`-based waiting (slow AND flaky).
- Tests that depend on execution order or on data left by earlier tests.
- Unique-constraint collisions because factories reuse fixed ids across parallel tests.
