# Project Model

Projects are first-class learning entities. The expansion keeps the existing model and adds
**cross-track projects** that combine skills from multiple tracks.

## Specification (unchanged shape)

`content/projects/<id>/project.json`:

```json
{
  "id": "cross-a-accessible-vue",
  "title": "Project A — Accessible Vue Application",
  "trackId": "web-accessibility",       // optional, display only
  "moduleId": "a11y-04",                // optional, GATES the project
  "summary": "…",
  "requirements": ["…"],
  "milestones": [{ "id": "m1", "title": "…", "detail": "…" }],
  "starterFiles": [{ "name", "language", "code" }],
  "hints": ["…"],
  "completionCriteria": ["…"],
  "estimatedHours": 6
}
```

- Gating reuses the prerequisite engine: a project tied to a locked module is blocked by
  the router guard — no new mechanism.
- Milestone completion persists in `project_states` (existing table).
- Integrity tests enforce actionable specs: requirements, milestones, hints, and
  completion criteria are all non-empty.

## The original project set (preserved)

Twelve track projects (`rust-01…06`, `vue-01…04`, `tauri-01…03`) plus the original
capstone (`capstone-rust-playground`) remain unchanged and keep their gates.

## Cross-track projects (new)

| Project | Combines | Gate |
| --- | --- | --- |
| A — Accessible Vue Application | Vue · Vue Architecture · A11y · Testing | `a11y-04` |
| B — High-Performance Vue Application | Vue · Architecture · Performance · HTTP (measure before/after) | `perf-05` |
| C — Secure Full-Stack Application | Vue · backend (Rust or Express) · Security · API Design · Databases · Testing | `sec-05` |
| D — Production Backend (Rust vs Express) | Backend Rust · Express · API Design · Testing · Observability · DevOps | `brust-05` |
| E — Production Full-Stack System | Vue + backend + PostgreSQL + auth + security + a11y + performance + testing + Docker + CI/CD + observability | `express-06` |

Each cross-track project ships starter files, milestone ladder, hints, and completion
criteria designed around the curriculum's teaching rules:

- **Measurement is mandatory** (B): baseline metrics before optimization; the report pairs
  every change with numbers.
- **Design before code** (C, E): threat model / OpenAPI / ADR precede implementation.
- **Honest comparison** (D): identical contract implemented twice, compared with evidence.
- **Production-quality, not enterprise ceremony** (E): one vertical slice end-to-end, real
  gates in CI, a runbook — no microservices-for-show.

## Recommendation integration

The recommendation engine surfaces unlocked-but-unfinished projects ("Its module is
unlocked and the project has not been completed yet") — completed projects are never
re-recommended (unit-tested).
