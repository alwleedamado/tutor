# Domain Models & DTO Mapping

Three kinds of types, three jobs:

- **Entity**: has identity that persists (a `User` with an id).
- **Value object**: defined by its value, immutable (`Email`, `Money`).
- **DTO**: a wire/transport shape — flat, serializable, boring.

## Making illegal states unrepresentable

Rust newtypes turn stringly-typed bugs into compile errors:

```rust
pub struct Email(String);   // not just a String
pub struct Cents(i64);      // not just an i64 — mixing with dollars won't compile

pub enum OrderStatus {
    Pending,
    Paid { at: Timestamp },
    Shipped { tracking: TrackingCode },
}
```

TypeScript expresses the same intent with discriminated unions:

```ts
type OrderStatus =
  | { kind: 'pending' }
  | { kind: 'paid'; at: string }
  | { kind: 'shipped'; tracking: string }
```

## Mapping at the boundary

The API DTO is *not* the domain type. Serialization concerns (camelCase, nullable dates) live in the DTO; invariants live in the domain:

```rust
// transport DTO
#[derive(serde::Deserialize)]
struct CreateUserDto { email: String }

// boundary mapping — validation happens HERE, once
impl CreateUserDto {
    fn into_domain(self) -> Result<Email, String> { Email::parse(&self.email) }
}
```

## Best practices

- Validate at the boundary **once**, then trust the type: downstream code never re-checks.
- Keep DTOs boring; keep domain types expressive.
- One direction per mapper (`into_domain`, `to_dto`) — bidirectional "converters" hide which side owns truth.

## Common mistakes

- **One type everywhere**: the DTO *is* the domain *is* the DB row — every concern leaks into every layer.
- `type Email = String` aliases: no safety, just a longer name.
- Parsing dates/ids ad hoc in five places instead of at the edge.
