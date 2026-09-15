# Dependency Rules & Circular Dependencies

The feature skeleton is only as good as the arrows between modules. Codify the allowed graph:

```text
feature  → shared, domain, api clients, stores
store    → domain, api
shared   → (nothing internal)
domain   → (nothing app-level)
featureA → featureB   ✘ (prefer router, stores, or domain events)
```

## Why cycles poison builds and brains

A↔B means neither module can be understood, tested, or deleted alone. Bundlers resolve it in *some* order; your init order silently depends on it — a class of bug that appears only "sometimes".

## Breaking a cycle

1. Find what A and B actually need from each other.
2. Extract that downward: a shared/ helper, a domain rule, or a store both consume.
3. Both now import the lower module; the A→B and B→A arrows are gone.

```text
before:  A → B → A
after:   A → C ← B      (C is shared/domain/store)
```

## Enforcement beats memory

- **eslint-plugin-boundaries** or dependency-cruiser: encode the graph, fail CI on violations.
- PR checklist: "which dependency arrows did this PR add?"
- Deep-import lint rule: forbid imports that skip a feature's index.ts.

## Best practices

- Cross-feature communication: navigate (router), share (store), or emit through domain events — not imports.
- Duplication is sometimes cheaper than the wrong abstraction; a 10-line helper duplicated in two features may be fine. Extract to `shared/` when the *third* user appears.

## Common mistakes

- "Just this once" cross-feature import — the first exception is the last.
- God stores: everything imports one store → that store can never change.
- Solving cycles with dynamic imports; the graph is still tangled, only lazily.
