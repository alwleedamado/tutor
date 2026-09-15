# Testing, CI, Containers & Operational Docs

The last mile of production engineering: prove it before merging, package it reproducibly, and document how to operate it when it's 3 a.m.

## CI: fast gates per PR

```yaml
# pipeline shape (GitHub-Actions-style)
jobs:
  verify:
    steps:
      - run: npm ci                # lockfile-driven, reproducible
      - run: npm run lint
      - run: npm run typecheck
      - run: npm test              # unit (fast, no I/O)
      - run: npm run test:int      # integration w/ test containers (db)
  build:
    needs: verify
    steps:
      - run: docker build -t api:$GITHUB_SHA .
```

PRs get minutes-fast gates; integration uses throwaway DB containers so tests are hermetic. Failing `npm audit` blocks merge (waivers are explicit PR decisions).

## The Dockerfile: multi-stage, non-root

```dockerfile
FROM node:22-alpine AS deps
WORKDIR /app
COPY package*.json ./
RUN npm ci --omit=dev

FROM node:22-alpine
WORKDIR /app
ENV NODE_ENV=production
COPY --from=deps /app/node_modules ./node_modules
COPY . .
USER node                      # never root — least privilege
EXPOSE 3000
CMD ["node", "server.js"]
```

Stage separation keeps dev tooling out of the image; layer caching makes rebuilds fast; `USER node` means a compromised process isn't root inside the container.

## Deploy: migrations then rollout, gated by health

```text
docker build → run migrations (backward-compatible) → rollout
             → /ready gates traffic → dashboards watched
```

## Operational docs (the unglamorous deliverable)

- **Runbook**: per alert — symptom, diagnosis query, remediation, escalation.
- **API contract**: the OpenAPI file IS the doc, versioned with code.
- **Migration policy**: expand/contract, who reviews schema changes.

## Best practices

- The pipeline is code, versioned, and the *only* path to production.
- Test the runbook: execute it once in a game-day exercise.

## Common mistakes

- Root containers "because it's easier" — container escapes escalate from root.
- Migrations applied manually by whoever remembers.
- No runbook: incidents solved by the one person who happens to know.
