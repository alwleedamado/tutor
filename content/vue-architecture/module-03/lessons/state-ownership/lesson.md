# State Ownership: Local → Composable → Pinia → Server → Persistent

Every piece of state has a *narrowest correct home*. Pick the widest home and you get a global-state dumping ground; pick narrower than needed and you get prop drilling. The ladder:

```text
1. local ref        lives and dies with the component
2. composable       shared behavior, caller-scoped state
3. Pinia store      genuinely shared across the app
4. server state     a CACHE of remote truth (not client truth)
5. persistent       survives restarts (SQLite/localStorage)
```

## Walking the ladder

```ts
// 1 — local: nobody else needs this
const isOpen = ref(false)

// 2 — composable: behavior reused, state per caller
const { data, loading } = useFetch('/api/orders')

// 3 — store: one instance, many consumers
defineStore('cart', { state: () => ({ items: [] as CartItem[] }) })

// 4 — server state: refetch/invalidation semantics apply
// 5 — persistent: settings, drafts, editor buffers (this app's SQLite)
```

## The decision questions

1. Who reads it? (one component → local)
2. Is its source of truth remote? (→ server state with cache semantics)
3. Must it survive reload? (→ persistent)
4. Do unrelated components share one instance? (→ store)
5. Otherwise → composable.

## Normalize before you share

Nested, denormalized server data (orders with inline customers) makes stores painful. Store entities by id and reference:

```ts
state: () => ({
  orders: {} as Record<string, Order>,
  customers: {} as Record<string, Customer>,
  orderIds: [] as string[]
})
```

Update a customer once; every order sees it.

## Best practices

- Default to the narrowest home; widening later is cheap, unwinding global state is not.
- Server data gets cache rules (staleness, invalidation) — never just "put it in the store and forget".
- Vuex mutations are history — Pinia's `actions` mutate directly; the architecture lesson (ownership, not mechanics) is what lasts.

## Common mistakes

- Every UI toggle promoted to Pinia "so anything can read it".
- Duplicating server data into local refs "for convenience" — instant staleness bugs.
- Persisting volatile state (loading flags) or not persisting real user data (drafts, settings).
