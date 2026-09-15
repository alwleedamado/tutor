# Components, Composables, Stores & Routing Boundaries

Four homes, four jobs — most "messy Vue app" problems are code living in the wrong home:

| Home | Owns | Lifetime |
| --- | --- | --- |
| Component | rendering + local UI state | mounts/unmounts |
| Composable | reusable behavior, caller-scoped state | per caller |
| Store (Pinia) | genuinely shared state | app lifetime |
| Route component | composition root for a feature subtree | per navigation |

## Composable vs store

A composable is a *function* — each caller gets its own state:

```ts
const a = useCounter() // independent
const b = useCounter() // independent
```

A store is a *singleton* — every caller shares one state:

```ts
const cart = useCartStore() // same everywhere
```

Rule of thumb: start with a composable; promote to a store only when two unrelated components must share the same instance.

## Route components are composition roots

A page assembles a feature: it triggers loading, composes feature components, and handles route params — but implements almost nothing itself:

```vue
<script setup lang="ts">
const route = useRoute()
const { order, loading, error } = useOrder(route.params.id)
</script>
<template>
  <OrderSkeleton v-if="loading" />
  <OrderError v-else-if="error" :error="error" />
  <OrderDetails v-else-if="order" :order="order" />
</template>
```

## Best practices

- Composables return functions + refs; stores return shared state + actions.
- One composable per behavior (`useFetch`, `useFocusTrap`) — compose them, don't build god-composables.
- Route files stay under ~100 lines; grow past that and a feature module is asking to exist.

## Common mistakes

- Everything in a giant shared store because "props are annoying" (prop drilling has better fixes).
- Route components doing API calls inline instead of through composables/services.
- Composables secretly sharing module-level refs — accidental global state with a local-looking API.
