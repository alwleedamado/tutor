# Refactoring a Growing Vue App (Guided)

The patient: `OrderPage.vue`, 900 lines — `fetch` calls, toast handling, discount rules, table rendering, three dialogs. It works. Every change to it is terrifying. Here is the safe path out.

## Step 0 — characterization tests

Before moving anything, pin current behavior:

```ts
// Vitest: mount OrderPage with a mocked api; assert what renders
// These tests are the safety net for every following step.
```

## Step 1 — extract the API layer

Move raw calls behind a typed client:

```ts
// services/ordersApi.ts
export const ordersApi = {
  list: (): Promise<OrderDto[]> => http.get('/orders'),
  refund: (id: string) => http.post(`/orders/${id}/refund`)
}
```

The component now calls `ordersApi.list()` — same behavior, one place owns transport.

## Step 2 — move state where it belongs

Discount rules are not UI; shared order state is not component state:

```ts
// domain/pricing.ts — pure functions, unit-tested directly
// stores/orders.ts — shared order state + actions calling ordersApi
```

## Step 3 — extract presentation

Table, dialogs, summary become child components with explicit props/emits. `OrderPage.vue` shrinks to composition: ~80 lines.

## The discipline

- One extraction per commit, tests green between commits.
- Behavior identical at every step — refactoring is not rewriting.
- Delete nothing "while you're there" (that's a different PR).

## Best practices

- Refactor toward the architecture you decided (features, services, stores) — not into a new ad-hoc shape.
- Small PRs: "extract ordersApi" reviews in minutes; "refactored OrderPage" reviews never.

## Common mistakes

- Skipping tests because "it's just moving code" — moving code is exactly when tests pay.
- Extracting everything at once; land steps independently so a revert is cheap.
- New bugs snuck in via "small cleanups" during the refactor.
