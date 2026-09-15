# Security Middleware & Secure Configuration

The service's perimeter is a set of middlewares, each defending a named attack — and a configuration story that never leaks secrets.

## Middleware with a threat in mind

| Middleware | Defends against |
| --- | --- |
| rate limiting | brute force, credential stuffing, DoS-by-cost |
| body size limit | memory exhaustion (`Content-Length: 4GB`) |
| timeouts | slow-loris style resource pinning |
| CORS config | unauthorized cross-origin reads in users' browsers |
| security headers | XSS amplification, MIME sniffing, clickjacking |

Rate limit per identity/IP with sane defaults (e.g. 100 req/min, burstable), and return `429` with `Retry-After` — a defined behavior, not a mystery timeout.

## CORS, deliberately

```text
Access-Control-Allow-Origin: https://app.example.com   ← specific
Access-Control-Allow-Credentials: true                 ← only with specific origins
```

`Access-Control-Allow-Origin: *` with credentials is not a configuration — it's a hole. List your actual frontends.

## Configuration that fails fast and never leaks

```rust
struct Config { db_url: String, port: u16 }

impl Config {
    fn from_env() -> Result<Config, String> {
        Ok(Config {
            db_url: env::var("DATABASE_URL").map_err(|_| "DATABASE_URL missing")?,
            port: env::var("PORT").ok().and_then(|p| p.parse().ok()).unwrap_or(8080),
        })
    }
}
```

- Missing critical config → refuse to boot (fail fast beats 3 a.m. discovery).
- Secrets from environment/secret manager — never source control, never compiled constants, never logs.
- Log configuration *presence*, not values (`db configured: true`).

## Best practices

- Every middleware exists to answer "what attack does this stop?" — otherwise delete it.
- A config schema validated at boot (types, ranges) turns misconfig into an instant error.

## Common mistakes

- `debug_assert`/trace logs that print tokens or secrets.
- Copy-pasted CORS `"*"` "because it works".
- Rate limiting only the login page while password reset and signup are open seasons.
