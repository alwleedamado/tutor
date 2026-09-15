# The Standard Production Stack

A handful of packages form the de-facto Express production stack — each earning its place by answering a named risk. Learn them with their *justification*, not as incantations.

| Package | Purpose | Skip when |
| --- | --- | --- |
| **helmet** | security headers (CSP, HSTS, nosniff, frameguard) | a gateway/proxy already sets them |
| **pino** | structured, fast JSON logging | throwaway script |
| **express-rate-limit** | brute-force/DoS-by-cost protection | a gateway rate-limits already |
| **zod / joi** | schema validation at the boundary | one endpoint with two fields (manual checks fine) |
| **dotenv-style** | load env in local dev | platform injects env already |

## Each maps to a threat

```text
helmet                → XSS amplification, sniffing, clickjacking, downgrade
express-rate-limit    → credential stuffing, resource exhaustion
pino                  → "we have no idea what happened" (unqueryable logs)
zod                   → malformed/trusted-too-soon input
```

If you can't name what a package defends against in *your* deployment, it's weight — or the protection belongs at a different layer (gateway, proxy).

## The deliberate non-defaults

- **ORMs** (Prisma/Drizzle/TypeORM): choose by team/stack, not by default; a query builder or plain SQL is honest for small schemas.
- **Test runner**: node's built-in `node:test` may suffice before pulling Vitest/Jest.
- **GraphQL**: only with a real consumer-driven need, not novelty.

## Best practices

- Introduce the stack in one reviewed commit with the threat named per package.
- Revisit yearly: the platform and your gateway may have absorbed a dependency's job.

## Common mistakes

- Installing the full stack cargo-cult style into a 3-endpoint service.
- Helmet with zero thought to CSP (then fighting it when inline scripts break).
- Two logging libraries because one "felt slow" — measure, then commit to one.
