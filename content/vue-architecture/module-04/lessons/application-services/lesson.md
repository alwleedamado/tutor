# Application Services, API Clients & DTOs

Between your components and the network sits an **application service layer**: typed API clients that own transport, and mapping that turns wire shapes (DTOs) into domain models.

## The typed client

```ts
// features/orders/api.ts — the only place that knows about HTTP
class ApiError extends Error { constructor(public status: number) { super(`api ${status}`) } }

export const ordersApi = {
  async list(): Promise<OrderDto[]> {
    const res = await fetch('/api/orders')
    if (!res.ok) throw new ApiError(res.status)
    return res.json()
  }
}
```

Auth headers, base URLs, retries, error normalization: all here, changed once, tested once.

## DTO → domain mapping

```ts
// dto (wire):        { id: "7", total_cents: 1299, placed_at: "2026-01-05T10:00:00Z" }
// domain (app):      { id: 7, totalCents: 1299, placedAt: Date }

export function toOrder(dto: OrderDto): Order {
  return { id: Number(dto.id), totalCents: dto.total_cents, placedAt: new Date(dto.placed_at) }
}
```

Components never see snake_case, string numbers, or date strings. Validation happens **once**, here — downstream code trusts the types.

## Injecting services

Composables accept their dependencies, so tests can substitute fakes:

```ts
export function useOrders(api: OrdersApi = ordersApi) {
  const orders = ref<Order[] | null>(null)
  const load = async () => { orders.value = (await api.list()).map(toOrder) }
  return { orders, load }
}
// test: useOrders(fakeApi) — no network, no mocks of internals
```

Provide/inject offers the same inversion at the component-tree scale (per-theme or per-tenant clients).

## Best practices

- One client module per feature; a tiny shared `http` core handles headers/errors.
- Mappers are pure functions — trivially unit-testable.
- Domain types express invariants (branded `OrderId`, `Date` objects), DTOs stay boring.

## Common mistakes

- `fetch` calls scattered in components — transport concerns multiplied by every screen.
- Skipping the mapping layer ("just cast it") — the wire shape infects the whole app.
- Business rules slipping into the client layer; it maps and transports, it does not decide.
