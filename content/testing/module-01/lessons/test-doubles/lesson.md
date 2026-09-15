# Mocks, Stubs, Fakes & Property-Based Testing

A **test double** replaces a real dependency in tests. The taxonomy matters because each has a different failure mode:

| Double | Answers "what?" | Example |
| --- | --- | --- |
| **Stub** | returns canned data | fake HTTP response |
| **Fake** | working lightweight impl | in-memory repo, SQLite file |
| **Mock** | asserts interactions happened | "save() was called once" |
| **Spy** | records calls for later inspection | wrapped notifier |

## Preference order at boundaries

Fakes first. A fake exercises real logic (an in-memory repo can still break on a duplicate key); a mock only replays your assumptions:

```rust
struct FakeUserRepo { rows: std::cell::RefCell<Vec<(u64, String)>> }
// implements UserRepo with real-ish behavior, no SQL
```

Mocks are for interactions with no meaningful return value — "we emailed the user" is exactly a mock's job.

## Example-based vs property-based

Example tests pin known cases. Property-based tests state an **invariant** and let the framework hunt for counterexamples:

```text
property: reversing a list twice yields the original
for all Vec<i64> xs:  xs.clone().reverse().reverse() == xs
```

Classic properties: encode/decode round-trips, serialization symmetry, idempotence, commutativity. In JS land, `fast-check` does the same for Vitest.

## Best practices

- Assert on **outcomes** (returned data, state changes) before asserting on calls.
- One concept per double: don't stub and spy and mock the same object in one test.
- Give property tests shrinking-friendly types (the framework must be able to minimize failures).

## Common mistakes

- Mocking everything → tests that verify the implementation, break on any refactor, and catch nothing.
- String-matching exact error texts of a library you don't own.
- Property tests over noisy domains without narrowing generators.
