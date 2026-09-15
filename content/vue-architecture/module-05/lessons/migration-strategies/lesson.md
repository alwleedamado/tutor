# Migration & Rescue Strategies

Inheriting a legacy Vue app? The goal is not a rewrite — it is a **rescue**: improve structure continuously while the product keeps shipping. The strangler pattern, applied to frontend:

## Phase 0 — safety net

Characterization tests over the critical flows (Vitest + mocked API): pin what the app *does* before changing anything. No tests, no refactor — you cannot keep behavior you cannot observe.

## Phase 1 — skeleton + seams

Introduce the target structure *beside* the old code:

```text
src/
  features/        ← new world (empty shells: one pilot feature)
  services/        ← typed api clients
  legacy/          ← explicitly named: where old code goes to die
```

New rule, effective immediately: legacy code stops growing. Fixes to old features may move them into `features/` when touched.

## Phase 2 — migrate route by route

One route per PR: move its page into a feature module, extract its API calls to a service, keep the URL identical. Old and new coexist behind the router — users see nothing; imports tell the truth about progress.

## Phase 3 — starve the legacy

Bug fixes land in the new modules. Legacy helpers stop receiving features. Each migration PR reports a number: imports into `legacy/` remaining.

## Phase 4 — delete

When a legacy module has zero importers, delete it. Deletion is the only honest completion metric.

## Measuring progress

```text
imports into legacy/:  412 → 380 → 240 → 90 → 0
```

Not weeks elapsed, not enthusiasm — mechanical dependency counts.

## Best practices

- Every migration step is shippable and revertible; no long-lived branch.
- Pilot with a medium-complexity feature (hard enough to reveal problems, small enough to finish).

## Common mistakes

- Big-bang rewrite: months of no features, then a terrifying cutover.
- Migrating the hardest screen first to "learn everything" — you learn it while blocked.
- Coexistence without a rule: new code copied into legacy patterns because it was faster.
