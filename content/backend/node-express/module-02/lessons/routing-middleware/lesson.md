# Routing, Middleware & the Request Lifecycle

An Express app is a pipeline of functions. Every request flows through them in registration order — order is the whole game.

## The pipeline

```js
app.use(express.json({ limit: '1mb' }))   // 1. parse bodies (before anything reads req.body)
app.use(requestLogger)                    // 2. log with timing
app.use('/api/orders', ordersRouter)      // 3. feature router
app.use(notFound)                         // 4. 404 fallback
app.use(errorHandler)                     // 5. LAST — catches everything above
```

A parser registered *after* a route means that route sees `req.body === undefined`. An error handler registered before a router never sees that router's errors. Read the file top-to-bottom as the request's timeline.

## Routers per feature

```js
const ordersRouter = express.Router()
ordersRouter.get('/:id', getOrder)
ordersRouter.post('/', requireAuth, createOrder)
app.use('/api/orders', ordersRouter)
```

Route-level middleware (`requireAuth` above) applies to exactly those routes — auth, rate limits, and validation attach at the smallest sensible scope.

## The error-handling contract

Express detects an error handler by its **four-argument signature**:

```js
function errorHandler(err, req, res, next) {
  const status = err.status || 500
  req.log?.error({ err, requestId: req.id })
  res.status(status).json({
    error: { code: err.code || 'INTERNAL',
             message: err.expose ? err.message : 'internal error' }
  })
}
```

`next(err)` from anywhere upstream lands here — one place maps errors to statuses and shapes.

## Best practices

- One `app.js` that reads as the request timeline; routers and middleware extracted to modules.
- 404 as an explicit middleware (Express won't 404 unmatched routes by itself in an API).

## Common mistakes

- `express.json()` missing or late → mysterious `undefined` bodies.
- Async route errors vanishing: wrap or `try/catch → next(err)`; a rejected promise without `next` never reaches the error handler (Express 4).
- Business logic inside middleware — middleware is cross-cutting, not a place for rules.
