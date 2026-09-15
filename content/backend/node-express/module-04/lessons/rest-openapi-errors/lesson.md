# REST Design, OpenAPI & Standardized Errors

An API is a contract. Contract-first means the agreement is written, reviewed, and derived from — not reverse-engineered from code.

## Contract-first with OpenAPI

```yaml
paths:
  /orders/{id}:
    get:
      parameters: [{ name: id, in: path, required: true, schema: { type: integer } }]
      responses:
        "200": { $ref: "#/components/schemas/Order" }
        "404": { $ref: "#/components/schemas/Error" }
```

The spec is the reviewable artifact: schemas, status codes, error shapes agreed before implementation. Validation (zod/joi) and docs generate from it; drift becomes a CI failure instead of a production surprise.

## One error envelope, everywhere

```json
// 422
{
  "error": {
    "code": "VALIDATION",
    "message": "email must be a valid address",
    "requestId": "8f3c..."
  }
}
```

Clients branch on `code`, never on message strings. `requestId` links a user complaint to logs instantly.

## Status discipline

| Situation | Status | Code |
| --- | --- | --- |
| unparseable body | 400 | VALIDATION |
| parsed but invalid | 422 | VALIDATION |
| no/invalid credentials | 401 | UNAUTHENTICATED |
| authenticated, not allowed | 403 | FORBIDDEN |
| doesn't exist | 404 | NOT_FOUND |
| state conflict | 409 | CONFLICT |
| unexpected | 500 | INTERNAL |

Never 200 with an error body — it breaks every client, monitor, and retry policy built on HTTP semantics.

## Best practices

- Error responses never leak stack traces or internal messages (`err.expose` gates what's public).
- Every endpoint documented, including failures — the failure table is half the contract.

## Common mistakes

- Ad-hoc error shapes per endpoint (clients grow a parser per route).
- Hand-editing generated docs until they lie — generate or gate in CI.
- Returning 500 for validation because "it threw".
