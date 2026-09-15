# Error Boundaries, Loading States & Feature Composition

Production UIs spend most of their time in non-ideal states: loading, empty, partially failed. Architecture decides whether those states are consistent or chaotic.

## One async contract, everywhere

```ts
const { data, loading, error, retry } = useOrders()
```

Every screen renders the same four modes from the same shape — skeleton while loading, empty state, error with retry, data. The pattern is decided once in the data layer, not per component.

## Error boundaries: scope the blast radius

A thrown error in one widget should not blank the entire page. Wrap regions so a failure degrades locally:

```vue
<ErrorBoundary :fallback="WidgetFailed">
  <PriceWidget :order="order" />
</ErrorBoundary>
```

The sidebar widget fails → the sidebar shows a fallback → the rest of the page works. Vue has no built-in boundary component (unlike React), but `onErrorCaptured` gives you the hook to build one in ~40 lines.

## Announce failures accessibly

```vue
<p v-if="error" role="alert">{{ error }}</p>
```

Screen reader users must hear the failure; silent red text is a double exclusion (see the Accessibility track).

## Composing features into pages

```vue
<OrderPage>
  <OrderSummary />   <!-- feature: checkout -->
  <CustomerCard />   <!-- feature: customers -->
  <Timeline />       <!-- feature: activity -->
</OrderPage>
```

Each feature fetches (or receives) its own data, owns its states, and can be developed/tested alone. Pages are compositions, not implementations.

## Best practices

- Centralize error normalization: API failures become one `AppError` shape UIs can render.
- Every async UI has four designed states: loading, empty, error, data.
- Boundaries around third-party widgets and anything with network I/O.

## Common mistakes

- Infinite spinners: no timeout, no error path — the loading state that never resolves.
- One component's throw unmounting the app (no boundary).
- Inconsistent per-component error handling: toast here, red text there, nothing there.
