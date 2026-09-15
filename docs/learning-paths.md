# Learning Paths

Deterministic, prerequisite-honoring routes through the expanded curriculum. Paths are
**derived from the prerequisite graph**, not hardcoded — the engine simply evaluates the
declarative gates; these documents describe intended journeys.

## The core spine (unchanged)

```text
Rust Mastery (M1→M6)          Vue 3 Mastery (M1→M4)
        │                            │
        └──────► Tauri (needs Rust M4) ────────► Capstone (Rust M6 ∧ Vue M4 ∧ Tauri M3)
```

## From mid-junior to production-capable engineer

The expansion encodes one coherent progression:

```text
Programming        Rust / Vue fundamentals (existing core)
    ↓
Language mastery   Rust M4–M6, Vue M3–M4
    ↓
Web fundamentals   HTTP & Web Platform (foundations)
    ↓
Framework depth    Vue Architecture · Backend Rust · Production Express
    ↓
Cross-cutting      Architecture · Testing · Git (foundations, any time after start)
    ↓
Specialization     Web Security · Web Performance · Web Accessibility
    ↓
Deployment         Linux → DevOps (Docker, CI/CD)
    ↓
Observability      Observability & Production Operations
    ↓
Production         Cross-track projects A–E → original capstone
```

## Recommended paths

**Frontend engineer**
`vue M1–M4` → `http` → `architecture M1–M2` → `vue-architecture` → `testing` →
`web-accessibility` → `web-performance` → Project A → Project B

**Backend engineer (Rust)**
`rust M1–M5` → `http` → `databases` → `api-design` → `testing` → `backend-rust` →
`linux` → `devops` → `observability` → Project D → Project E

**Backend engineer (Node)**
`http` → `databases` → `api-design` → `testing` → `backend-express M1–M6` →
`linux` → `devops` → `observability` → Project D → Project E

**Security engineer**
`http` → `web-security M1–M5` → `backend-rust M4` or `express M5` → Project C

## How the app suggests paths

- The dashboard's **Recommended for you** strip and the **Continue** hero are produced by
  the deterministic recommendation engine (`engine::skills::recommend`) — prerequisites
  first, then continuation, review, skill gaps, then unlocked projects.
- Skill states on `/tracks` show what is demonstrated and what each skill needs next.
- Nothing here is hardcoded per track: paths emerge from `module.json` prerequisites and
  the skill graph.

## Parallelism

Rust and Vue remain independent from the first module. Foundations (HTTP, git, testing,
databases, architecture) are open from the start — they are shared substrate, not gates on
the core spine.
