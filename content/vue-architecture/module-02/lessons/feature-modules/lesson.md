# Feature Modules & Public APIs

Type-first folders (`components/`, `stores/`, `composables/`) scatter one feature across five directories. **Feature-first** keeps a change in one place:

```text
src/
  features/
    checkout/
      index.ts          ← the ONLY door in
      components/
      composables/
      api.ts
      store.ts
    billing/
      ...
  shared/
    ui/                 ← tiny, stable, no feature imports
    lib/
  domain/
```

## The public API discipline

`index.ts` is the module's contract. Everything not exported there is *private* and can be renamed, split, or deleted freely:

```ts
// features/checkout/index.ts
export { CheckoutPage } from './components/CheckoutPage.vue'
export { useCheckout } from './composables/useCheckout'
export type { CheckoutSummary } from './types'
```

```ts
// ✅ legal — through the door
import { useCheckout } from '@/features/checkout'
// ❌ illegal — deep import reaching past the contract
import { useCheckout } from '@/features/checkout/composables/useCheckout'
```

## Why it matters

Deep imports mean internal layout can never change without breaking consumers. A curated API makes the feature refactorable *from the inside* forever.

## Best practices

- New code: "which feature does this belong to?" before "which file type?"
- `shared/` stays tiny and boring — it is imported by everyone, so it must change rarely.
- A feature may import `shared/` and `domain/`; never another feature's internals.

## Common mistakes

- `shared/` growing into a junk drawer (zero cohesion with a nice name).
- Index files re-exporting *everything* (`export *`) — no curation, no contract.
- Features importing features; cross-feature needs go through domain, shared, stores, or the router.
