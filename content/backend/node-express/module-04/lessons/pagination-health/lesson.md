# Pagination, Idempotency & Health Endpoints

Three small features that separate demos from production APIs.

## Cursor pagination with caps

```js
// GET /orders?limit=20&cursor=<opaque>
app.get('/orders', async (req, res) => {
  const limit = Math.min(parseInt(req.query.limit) || 20, 100)  // server-side cap
  const orders = await ordersRepo.page({ cursor: req.query.cursor, limit })
  res.json({
    data: orders,
    nextCursor: orders.length === limit ? orders.at(-1).id : null
  })
})
```

Caps are server-enforced — clients can request `limit=1000000` and get 100. Cursors are opaque strings; clients never construct them.

## Idempotency-Key

```js
app.post('/charges', async (req, res, next) => {
  const key = req.get('Idempotency-Key')
  if (!key) return next(new AppError(400, 'VALIDATION', 'Idempotency-Key required'))
  const existing = await idempotencyStore.get(key)
  if (existing) return res.json(existing)          // replay, no re-execution
  const result = await chargesService.create(req.body)
  await idempotencyStore.put(key, result, { ttl: 24 * 3600 })
  res.status(201).json(result)
})
```

Network retries are inevitable; this makes them safe. Repeated key + different payload → 422 (client bug, say so).

## /health vs /ready

```js
app.get('/health', (req, res) => res.json({ status: 'ok' }))   // liveness: process exists

app.get('/ready', async (req, res) => {                        // readiness: can serve?
  try { await db.query('SELECT 1'); res.json({ status: 'ready' }) }
  catch { res.status(503).json({ status: 'not-ready' }) }
})
```

Liveness failing → the orchestrator restarts you. Readiness failing → the load balancer stops sending traffic (and resumes later). Confusing them causes restart loops during dependency hiccups.

## Best practices

- Every list endpoint: capped limit + cursor + documented default order.
- Readiness checks the real dependencies, cheaply and with a timeout.

## Common mistakes

- Offset pagination on growing tables (page 10000 scans 200k rows).
- 500 on duplicate submissions instead of idempotent replay.
- `/ready` that only returns static 200 — it verifies nothing, so it lies.
