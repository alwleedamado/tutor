# Server State in Vue Apps

Server data is different: the truth is remote, your copy is a **cache**, and caches need policies — when to fetch, when it's stale, what invalidates it.

## A minimal typed fetch composable

```ts
export function useApi<T>(fetcher: () => Promise<T>) {
  const data = ref<T | null>(null)
  const loading = ref(false)
  const error = ref<string | null>(null)
  async function run() {
    loading.value = true; error.value = null
    try { data.value = await fetcher() }
    catch (e) { error.value = String(e) }
    finally { loading.value = false }
  }
  return { data, loading, error, run }
}
```

Loading/error live in the *data layer*, so every screen renders them the same way — consistency is architecture, not styling.

## Cache semantics in plain terms

- **Stale-while-revalidate**: show cached data immediately, refetch in the background.
- **Invalidation by key**: after `POST /comments`, invalidate the `['comments', postId]` key — not everything.
- **Deduplication**: three components asking for the same key share one request.

Libraries (TanStack Query, SWRV) package these policies; a small hand-rolled version teaches you exactly what they buy you.

## Mutations and the cache

```ts
async function addComment(postId: string, body: string) {
  await commentsApi.create(postId, body)
  await invalidate(['comments', postId])   // precise: only affected cache
}
```

Targeted invalidation beats refetch-the-world and beats optimistic updates you never reconcile.

## Best practices

- One key scheme for the whole app (`[resource, id, params]`) — keys are the address system of your cache.
- Errors are state, not toasts-only: the UI must render the failed mode (with a retry).
- Abort in-flight requests on unmount/route change (`AbortController`).

## Common mistakes

- Copying server responses into local refs — now two copies disagree.
- Polling as a default (costs requests, still lags) instead of invalidation on mutation.
- Loading flags scattered per component with five different spinner styles.
