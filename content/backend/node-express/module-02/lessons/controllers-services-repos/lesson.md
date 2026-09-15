# Controllers, Services & Repositories

Express gives you the pipeline; you supply the layers. Three roles keep a growing API sane:

```text
controller    transport:   parse, delegate, respond        (no rules, no SQL)
service       use-cases:   business rules, orchestration   (no req/res, no SQL)
repository    storage:     queries and nothing else        (no rules)
```

## The flow

```js
// controller — thin on purpose
export async function createOrder(req, res, next) {
  try {
    const order = await orderService.create(req.body, req.user)
    res.status(201).json(order)
  } catch (e) { next(e) }
}

// service — where the rules live and tests point
export async function createOrder(input, user) {
  const data = CreateOrderSchema.parse(input)       // boundary validation
  if (data.totalCents < 0) throw new AppError(422, 'VALIDATION', 'total < 0')
  const customer = await customersRepo.find(user.id)
  return ordersRepo.insert({ ...data, customerId: customer.id })
}

// repository — SQL only
export function insert(order) { /* parameterized INSERT */ }
```

## Why bother

- The **service** is unit-testable with a fake repository — no HTTP, no database.
- Changing Postgres → something else touches only repositories.
- Every reviewer knows where a change belongs before opening the file.

## Validation at the boundary

Schema validation (zod/joi) belongs at the controller edge — the trust boundary — producing typed, trusted values for the service. Deeper layers don't re-validate.

## Best practices

- One service per use-case; services may call other services and repositories, never `req`/`res`.
- Repositories expose intention-named queries (`findPendingByCustomer`), not generic query builders leaked upward.

## Common mistakes

- Fat controllers (rules inline) — untestable without HTTP and duplicated across endpoints.
- SQL scattered in services — now every query change is an architecture change.
- Skipping the service layer "because it's simple" — the first rule always arrives.
