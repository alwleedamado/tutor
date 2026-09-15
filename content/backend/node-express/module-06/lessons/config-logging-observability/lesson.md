# Configuration, Structured Logging & Observability

"Enterprise-grade" here means: when something breaks at 3 a.m., one person with a dashboard and a query finds the cause. Three practices deliver that.

## Configuration: 12-factor

```text
code = defaults only      env = the actual values      secrets = secret store
```

```js
const config = {
  port: Number(process.env.PORT ?? 3000),
  dbUrl: requireEnv('DATABASE_URL'),          // missing → refuse to boot
  logLevel: process.env.LOG_LEVEL ?? 'info'
}
```

Read once at boot, validate types/ranges, fail fast. The same image runs in every environment; only env differs.

## Structured logging with request correlation

```js
import pino from 'pino'
const log = pino({ level: config.logLevel })

app.use((req, res, next) => {
  req.id = randomUUID()
  req.log = log.child({ requestId: req.id, route: req.path })
  const t0 = performance.now()
  res.on('finish', () => req.log.info({
    status: res.statusCode, ms: Math.round(performance.now() - t0)
  }, 'request done'))
  next()
})
```

Every log line carries `requestId` + route + duration. One user complaint → grep the requestId → the entire request story. That single habit is half of observability.

## Metrics + health: the operational minimum

- **RED per endpoint**: request Rate, Error rate, Duration histogram (p50/p95/p99).
- **Saturation**: event-loop lag, memory, DB pool waits.
- **Health/readiness** endpoints (express-04) feeding orchestrators and dashboards.

Alert on symptoms users feel (error rate, p99 latency, queue age) — not on CPU percentages.

## Best practices

- Log state changes and outcomes at INFO; reserve DEBUG for what you'd want only during an investigation.
- Never log secrets, tokens, or full payloads (PII discipline).

## Common mistakes

- `console.log` with no ids — correlation becomes archaeology.
- Averages-only latency dashboards (p50 looks fine while p99 is on fire).
- Config read via `process.env` scattered through 40 files, unvalidatable.
