# Rust Mastery Offline — System Architecture

## Overview

Rust Mastery Offline is a fully offline desktop learning environment. Originally built for
Rust, Vue 3, and Tauri v2, the curriculum has expanded (see `docs/curriculum-architecture.md`)
to **18 tracks** — three core tracks plus shared engineering foundations and specialized
tracks — backed by a declarative skill graph, rich per-lesson progress, and a deterministic
recommendation engine. The application architecture itself is unchanged by the expansion:
additive model fields, a declarative track registry, and derived views.

```
┌────────────────────────────────────────────┐
│                Vue 3 UI                    │
├────────────────────────────────────────────┤
│ Components / Pages / Layouts               │
├────────────────────────────────────────────┤
│ Composables                                │
├────────────────────────────────────────────┤
│ Pinia Stores                               │
├────────────────────────────────────────────┤
│ Application Services                       │
├────────────────────────────────────────────┤
│ Tauri IPC Client                           │
╞════════════════════════════════════════════╡
│              Tauri Boundary                │
╞════════════════════════════════════════════╡
│ Rust Application Services                  │
├────────────────────────────────────────────┤
│ Domain / Learning Engine                   │
├────────────────────────────────────────────┤
│ Persistence / SQLite                       │
├────────────────────────────────────────────┤
│ Compiler / Execution Services              │
└────────────────────────────────────────────┘
```

## Core Principles

1. **Offline-first.** Every asset (lessons, editor, fonts, icons) ships inside the binary or the
   bundled frontend. Runtime never performs a network request.
2. **Application ≠ tutorial technology.** The app runs on modern architecture (Pinia, Composition
   API, structured Rust errors). Vuex exists *only* as embedded educational text under
   `content/tutorials/` — it is never installed, imported, registered, or executed by the runtime.
3. **Thin boundaries.** Tauri commands are thin adapters over Rust services; Vue components are
   thin presentation over stores/composables. No business logic in either layer.
4. **Explicit errors.** A single `AppError` enum crosses IPC as a structured JSON object — never
   `Result<T, String>`.
5. **Least privilege.** The Tauri capability file grants `core:default` only; no shell/fs/http
   plugins are exposed to the webview.
6. **Deterministic behavior.** Execution timeouts, cleanup, and unlock evaluation are pure,
   testable functions wherever possible.

## Major Subsystems

| Subsystem            | Location                     | Responsibility                                        |
|----------------------|------------------------------|-------------------------------------------------------|
| Content Loader       | `src-tauri/src/content`      | Embed & validate curriculum (`include_dir`), declarative track registry, build catalog, skill graph |
| Learning Engine      | `src-tauri/src/engine`       | Unlock evaluation, completion derivation, skill states, deterministic recommendations |
| Persistence          | `src-tauri/src/db`           | SQLite via `rusqlite`, versioned migrations, attempt-stats aggregation |
| Execution Service    | `src-tauri/src/execution`    | Sandboxed rustc/cargo runs, timeout, cancellation        |
| Command Layer        | `src-tauri/src/commands`     | Thin `#[tauri::command]` adapters                       |
| IPC Client           | `src/services/ipc.ts`        | Typed wrappers around `invoke`, single serialization seam |
| Stores               | `src/stores/*.store.ts`      | Pinia state; the ONLY cross-component state mechanism    |
| Views / Components   | `src/views`, `src/components`| Presentation & interaction only                          |

## Curriculum Expansion Surface (additive)

- **Track registry** — `content::TRACKS` (id, dir, title, description, category). The catalog
  is data; adding a track never touches engine or UI code.
- **Model extensions** — `Track.category`, `Module.difficulty/skills`, `Lesson.skills/related`
  (all serde-defaulted; original content unchanged).
- **Skill graph** — `content/skills/skills.json` evaluated in `engine::skills`
  (`docs/skill-model.md`); states and recommendations ship inside `ProgressSummary`.
- **Rich progress** — per-lesson states (not-started → attempted → practicing → completed →
  mastered) derived from completions + aggregated quiz/exercise attempts.
- **Generic track routes** — `/:track/:module?/:lesson?` validated against the catalog in the
  router guard; core-track named routes preserved for deep links.
- **Runner rule** — compiled-language tracks (rust, tauri, backend-rust) run the rustc
  sandbox; every other track previews in the sandboxed webview (`starters.ts` is the single
  source of that rule).

## Data Flow

- **Startup:** `get_bootstrap` returns `{ catalog, progress, settings }` in one IPC round-trip.
  The catalog (all lessons, quizzes, exercises, examples) is embedded at compile time, so first
  paint requires no filesystem traversal.
- **Mutations:** UI → store action → service → command → service → DB → updated summary returned
  to keep store and database consistent without polling.
- **Execution:** editor code → `execution.store` → `run_rust_code` → sandboxed process tree →
  `RunResult { stdout, stderr, diagnostics, timedOut }`.

## Error Strategy

Rust: `thiserror`-based `AppError` with variants per subsystem, serialized as
`{ kind: string, message: string }`. Frontend: services reject with typed `AppErrorDto`; stores
surface user-readable messages; components never parse error strings.

## Security Posture

- Student Rust code never executes inside the Tauri process (see `execution-security.md`).
- All IPC inputs validated Rust-side (sizes, path containment, enum membership).
- CSP restricts scripts/styles/connect sources to self (+ `wasm-unsafe-eval`, blob workers).
- The database lives in the OS app-data dir; student sandboxes live in temp dirs and can never
  reach it.
