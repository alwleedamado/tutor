# Regression Testing & Test Strategy

A **regression test** converts a fixed bug into a permanent guard. Every bug fix ships with the test that would have caught it — that is how suites grow exactly where the software actually breaks.

## Matching levels to stacks

| Stack | Level | Notes |
| --- | --- | --- |
| Rust core | `cargo test` unit + integration | pure domain tests are microseconds |
| Vue UI | Vitest + Vue Test Utils | mount components, assert rendered behavior |
| APIs | HTTP-level tests (supertest-style, axum oneshot) | status + body contracts |
| Tauri | command-level tests over the lib crate | engine + DB without a webview |
| E2E | few, critical journeys only | expensive; keep them stable |

```rust
// regression: command rejects empty note bodies
#[test]
fn save_note_rejects_empty_body() {
    let err = validate_note("").unwrap_err();
    assert_eq!(err.kind, ErrorKind::Validation);
}
```

## The one-page strategy

A test strategy is not a novel. One page: (1) what we test at which level, (2) what we deliberately do not test and why, (3) quality gates in CI (unit+lint on every PR, integration nightly, E2E before release), (4) how bugs become regression tests.

## CI gates

- Every PR: lint + typecheck + unit tests. Fast enough to never skip.
- Nightly: integration, property tests with higher iteration counts.
- Release: E2E smoke of the critical path.

## Best practices

- Reproduce → failing test → fix → keep both forever.
- Review tests as seriously as production code — they are the safety net for the next reviewer.
- Delete tests that test nothing; a lying suite is worse than a small honest one.

## Common mistakes

- "Fixed it" without a test — the bug will come back on a Friday.
- E2E suites replacing the unit floor instead of capping it.
- Strategy docs nobody can find; keep it one page, next to the code.
