# Auth, Auditing & Supply-Chain Hygiene

Wiring auth into Express is mechanical; the discipline is in enforcement, audit trails, and the packages you trust.

## Authn/authz wiring

```js
// authenticate once, attach req.user
app.use('/api', authenticate)        // verifies session cookie or JWT

// authorize per route — server-side, always
app.delete('/api/users/:id', requireRole('admin'), deleteUser)
app.get('/api/orders/:id', requireOwnership(Order), getOrder)
```

The UI hiding a button is UX; `requireRole` on the route is security. Authorization is checked on **every** request server-side, default-deny.

## Security event logging

```js
// who / what / outcome — never the secret itself
log.warn({ event: 'auth_failed', email, ip: req.ip, requestId: req.id })
log.info({ event: 'role_changed', target: userId, by: adminId })
```

Log the *facts* of security events (failed logins, permission changes, token revocations) so incidents are reconstructable. Never log tokens, passwords, or full session identifiers — the audit log must not become the leak.

## Supply-chain hygiene

```text
lockfile committed        → reproducible installs, auditable versions
npm audit in CI           → known CVEs fail the build (or are consciously waived)
pinned major versions     → upgrades are reviewed events, not surprises
minimal dependencies      → every package is attack surface (see express-03)
install scripts reviewed  → npm install executes code
```

An `npm audit` failing the build only works if waivers are visible decisions (documented in the PR), not noise to ignore.

## Best practices

- Secret rotation drill: change `SESSION_SECRET` in staging; know what breaks (sessions invalidate — is that acceptable?).
- Central auth middleware ownership; route-level helpers stay small and composable.

## Common mistakes

- `.env` with real secrets committed (git history is forever — rotate, don't just delete).
- Logging `Authorization` headers "for debugging".
- Auth middleware only on routes the frontend links to — attackers don't use your frontend.
