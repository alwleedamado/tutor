# Application Boundaries & Dependency Direction

A Vue app grows past "toy" when three distinct concerns appear — and stays healthy only if they stay separated:

```text
UI (components)   →  application services  →  domain logic
     composables,        api clients,          rules, types,
     pages, stores       mapping               calculations
```

## The one rule

**Arrows point toward the domain. Never back.** Components may import composables and services; services may import domain logic; the domain imports nothing from the layers above.

## What a violation looks like

```ts
// ❌ domain importing a store — the domain now knows about Pinia
import { useCartStore } from '@/stores/cart'
export function taxFor(cart: Cart) { /* uses store inside a "pure" rule */ }

// ✅ domain stays pure; the store applies the rule
export function taxFor(subtotalCents: number): number { ... }
// store: this.total = subtotalCents + taxFor(subtotalCents)
```

## What a healthy boundary looks like

```ts
// component: orchestration + rendering only
const { orders, loading, error, load } = useOrders()
```

The component doesn't know where orders come from — swap the API for a Tauri command and the component doesn't change.

## Best practices

- Name the layers in your repo README; make every PR answer "which arrows did this add?"
- Extract a module the moment two components need the same rule.
- Keep domain modules import-free (of app code) — they should run in any test with zero setup.

## Common mistakes

- Business rules accumulating inside components "temporarily".
- The api client imported directly by 30 components — one auth-header change touches all of them.
- Domain types importing DTO types from the transport layer.
