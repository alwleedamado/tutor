# Unit, Integration, Contract & E2E

Different bugs live at different depths. The pyramid orders tests by scope and cost:

```text
        /  E2E  \\        few — whole system, slow, brittle
      / contract  \\      API boundaries agree
    / integration  \\     modules together (DB, HTTP, stores)
  /     unit        \\   many — pure logic, fast, precise
```

## What each level catches

- **Unit**: logic errors — a pricing function, a reducer, a Rust parser. Milliseconds each.
- **Integration**: modules *disagree* — the service and the real SQL schema, a Pinia store and a component.
- **Contract**: producer and consumer of an API drift apart (field renamed, type changed).
- **E2E**: the whole journey works — but every test costs seconds and buys flakiness risk.

## Push tests down

If a bug is caught by an E2E test, ask: could an integration or unit test have caught it faster? The cheapest level that can catch a bug is where it belongs.

```rust
#[test]
fn discounts_never_go_negative() {
    assert_eq!(total_cents(&[1000], &[-2.0]), 0);
}
```

This unit test runs in microseconds; the same assertion behind a browser test runs in seconds.

## Coverage is not quality

```rust
#[test]
fn covers_nothing() {
    let _ = total_cents(&[], &[]); // runs the line, asserts nothing
}
```

Coverage tells you what code *ran*, not what behavior was **checked**. Use it to find untested areas, never as a target.

## Best practices

- Name tests as behavior: `refuses_negative_discounts`, not `test1`.
- Fast suites get run; keep the unit floor wide and the E2E roof narrow.
- One behavior per test — failure messages should diagnose themselves.

## Common mistakes

- **Ice-cream cone**: mostly-E2E suites that take an hour and fail randomly.
- Coverage targets driving assertion-free tests.
- Testing implementation details (private call sequences) instead of observable behavior.
