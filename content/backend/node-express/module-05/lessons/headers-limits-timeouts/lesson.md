# Secure Headers, Rate Limits & Timeouts

Perimeter hardening is a short list — but every line must name the attack it stops, or it's decoration.

## Security headers (helmet's defaults, mapped to attacks)

| Header | Stops |
| --- | --- |
| Content-Security-Policy | XSS amplification, injected scripts |
| Strict-Transport-Security (HSTS) | protocol downgrade to HTTP |
| X-Content-Type-Options: nosniff | MIME-type confusion attacks |
| X-Frame-Options / frame-ancestors | clickjacking |
| Referrer-Policy | token leakage via Referer |

```js
import helmet from 'helmet'
app.use(helmet())          // sane defaults; tune CSP deliberately
```

Headers don't fix injection — they cap what a successful injection can do, and stop whole attack classes outright.

## Limits: body size and request rate

```js
app.use(express.json({ limit: '1mb' }))       // memory exhaustion defense

const authLimiter = rateLimit({ windowMs: 60_000, limit: 10 })
app.use('/api/auth', authLimiter)             // credential stuffing defense
app.use('/api', apiLimiter)                   // general abuse ceiling
```

An unbounded JSON body is a free memory-exhaustion DoS (`Content-Length: 4GB`). Unthrottled auth endpoints are a free credential-stuffing range. Rate-limit responses carry `429` + `Retry-After` — defined behavior, not mystery hangs.

## Timeouts everywhere

```js
const server = app.setTimeout(30_000)                 // hung clients
await fetch(upstream, { signal: AbortSignal.timeout(5_000) })  // hung dependencies
```

A client that opens sockets and never finishes (slow-loris) pins connections; an upstream that never responds pins your request handlers. Timeouts reclaim resources — every outbound call gets one.

## Best practices

- Rate limits keyed by identity where authenticated, IP where not, strictest on auth/reset endpoints.
- Tune CSP once, with report-only mode first — then enforce.

## Common mistakes

- Helmet installed, CSP disabled forever "because inline scripts".
- Rate limiting only `/login` while `/forgot-password` (email bombs) and `/signup` (fake accounts) stay open.
- No upstream timeouts: one slow dependency turns your API into a connection sponge.
