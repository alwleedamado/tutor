# Cohesion, Coupling & Dependency Direction

Two words decide whether a codebase ages well: **cohesion** (things that change together live together) and **coupling** (how much one module must know about another to work).

## The dependency arrow

Every import is an arrow. Healthy systems point arrows in **one direction**:

```text
UI (components)  →  application services  →  domain logic  →  infrastructure
```

The moment domain logic imports a UI type, or a service reaches back into a component, the arrow points backwards and every future change fans out.

## In Rust

```rust
// domain: knows nothing about HTTP or the database
pub struct Invoice { pub id: u64, pub total_cents: i64 }

impl Invoice {
    pub fn total_euros(&self) -> f64 { self.total_cents as f64 / 100.0 }
}

// transport: knows the domain, the domain never knows transport
fn invoice_json(inv: &Invoice) -> String {
    format!("{{ \"id\": {}, \"total\": {} }}", inv.id, inv.total_euros())
}
```

`Invoice` is highly cohesive (money logic together) and loosely coupled (no serde, no sqlx types inside the domain).

## In Vue

```ts
// ❌ low cohesion: pricing rules inside a button component
// ✅ pricing rules live in a domain module the component calls
import { totalPrice } from '@/domain/pricing'
const total = computed(() => totalPrice(items.value))
```

## Best practices

- Group by **change**, not by technical layer alone: code that changes together sits together.
- One-way dependencies: UI → services → domain. Enforce with review checklists and lint rules.
- Prefer **narrow** interfaces between modules: small public APIs are cheap to keep stable.
- When two modules always change together, merge them; when one changes without the other, split them.

## Common mistakes

- **Utils dumping ground**: a `utils.ts` with 40 unrelated functions is zero cohesion with a name.
- **Backward imports**: domain code importing DTO types from the HTTP layer "just for convenience".
- **Everything shares everything**: a `shared/` folder every feature imports from becomes a coupling hub — keep it tiny and stable.
