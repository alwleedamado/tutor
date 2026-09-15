# Password Hashing, Sessions & Tokens

Authentication done wrong is unrecoverable — leaked hashes and hijacked sessions end trust permanently. The mechanics are settled; the discipline is applying them.

## Passwords: slow, salted, one-way

```text
argon2 (preferred) / bcrypt:  per-user salt + deliberately slow KDF
NEVER:                        MD5/SHA-256 alone (billions of guesses/sec),
                              plaintext, reversible "encryption"
```

Why slow? An attacker with your DB tries guesses offline. A fast hash rates them in billions/second; argon2 makes each guess expensive. Per-user salts defeat precomputed rainbow tables. Login rate limiting completes the defense.

## Sessions vs JWTs — the real tradeoff

| | Server session | Stateless JWT |
| --- | --- | --- |
| lookup | every request (store hit) | none (signature check) |
| revocation | instant (delete row) | only at expiry |
| scale | shared session store | nothing to share |

Stateless tokens cannot be un-issued — a stolen 24h JWT is valid for 24 hours no matter what. Production pattern: **short-lived access token (5-15 min) + refresh token with rotation** (each use issues a fresh refresh token; reuse of an old one = theft signal → revoke the family).

## Storage of tokens

`HttpOnly, Secure, SameSite` cookies keep tokens out of JavaScript (XSS can't read them). `localStorage` is readable by any injected script — avoid for auth tokens.

## Authorization on top

Authentication says who; authorization says what. Check **server-side on every request** (middleware/extractor), model roles as data (RBAC), and default-deny.

## Best practices

- Argon2id with library defaults; migrate hashes lazily on next login when upgrading parameters.
- Session invalidation endpoints (logout everywhere) exist and are tested.

## Common mistakes

- Long-lived JWTs with no revocation story.
- Storing password "hints" or logging passwords (yes, it happens).
- Client-side authorization checks *only* — the API must enforce, the UI merely hides.
