# Architecture Decision Records & Tradeoffs

Six months from now, someone will ask "why is it built this way?" An **ADR** (architecture decision record) is the cheap, durable answer: one markdown file per significant decision.

## A minimal ADR

```markdown
# ADR-007: Pinia for client state

## Context
Vue 3 app, growing shared state, multiple stores needed.

## Options
1. Pinia — first-party, TS-native, devtools
2. Vuex 4 — legacy, mutation ceremony
3. Composables only — fine until state is truly shared

## Decision
Pinia for shared state; composables for component-local concerns.

## Consequences
+ TypeScript inference, no mutations
- One more concept for newcomers (mitigated by docs)
```

## The decision process that feeds ADRs

```text
Problem → Constraints → Options → Tradeoffs → Decision → Measurement
```

A tradeoff table forces honesty:

| Option | Fits team | Reversible | Ops cost |
| --- | --- | --- | --- |
| SQLite | yes | yes | low |
| Postgres server | yes | no | higher |

## Reversibility is the hidden axis

Two-way-door decisions (rename a module, swap a library behind a port) deserve less ceremony than one-way-door decisions (database engine, public API shape). Spend ADR effort where reversal is expensive.

## Best practices

- ADRs are **immutable history**: to change a decision, write a new ADR that supersedes it.
- Number them; link them from the code that embodies the decision.
- Record *options rejected and why* — that is the part memory loses first.

## Common mistakes

- ADRs as bureaucracy for every trivial choice — reserve them for expensive/structural decisions.
- Writing the decision without consequences (every decision costs something; say what).
- Debate-by-vibes: no constraints or measurement criteria, so decisions get relitigated forever.
