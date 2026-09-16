# Rust Mastery Offline

A production-quality, **fully offline** desktop learning environment — the original core
teaches **Rust**, **Vue 3**, and **Tauri v2**, now expanded to **18 tracks** that take a
mid-junior developer to production-capable engineer: shared engineering foundations
(HTTP, architecture, testing, git, databases, DevOps, observability), specialist tracks
(Vue Architecture, Web Performance, Web Security, Backend Rust, Production Express, Web
Accessibility), a declarative skill graph, cross-track projects, and a deterministic
recommendation engine.

Built with Tauri v2, Vue 3 (`<script setup>` + Pinia), TypeScript, TailwindCSS, Monaco
Editor, and SQLite.

> **Architectural constraint:** the application runs on modern architecture (Pinia, Composition
> API, structured Rust errors). Vuex appears **only** as embedded tutorial text under
> `content/tutorials/` and is never installed, imported, or executed by the app.

## Layout

```
docs/        architecture documentation (start with docs/architecture.md)
src/         Vue 3 frontend (views, stores, services, composables)
src-tauri/   Rust backend (commands, learning engine, skill graph, SQLite, execution service)
content/     curriculum: 18 tracks, skills/skills.json, projects/, capstone/, tutorials/
scripts/     build utilities (icon generation)
```

## Curriculum (18 tracks)

**Core** — the original curriculum, unchanged and first in the catalog:

- 🦀 **Rust Mastery** — fundamentals → ownership → data modeling → generics/traits →
  concurrency → advanced (M4 unlocks the Tauri track)
- 💚 **Vue 3 Mastery** — fundamentals → Pinia-first state management → routing → advanced
  tooling (independent of the Rust track)
- 🛡️ **Tauri Desktop Development** — commands/IPC → filesystem & SQLite → advanced

**Foundations** — shared substrate for every specialist track:

| Track | Focus |
| --- | --- |
| 🌐 HTTP & Web Platform | DNS→TLS, methods/headers, cookies, caching, CORS/CSP |
| 🏛️ Software Architecture & Design | cohesion/coupling, ports & adapters, ADRs, technical debt |
| ✅ Testing & Quality Engineering | test pyramid, doubles, fixtures, regression strategy |
| 🌿 Git & Professional Workflow | branches, rebase, conflicts, PRs, recovery |
| 🔌 API Design & Integration | resource modeling, pagination/idempotency, OpenAPI, versioning |
| 🗄️ Database Engineering | normalization, SQL beyond CRUD, transactions, indexes, pools |
| 🐧 Linux & Developer Tooling | shell, permissions, processes, ports, HTTP debugging |
| 🐳 DevOps, CI/CD & Containers | Docker, compose, pipelines, deploys & rollback |
| 📈 Observability | structured logs, metrics, traces, health/readiness, alerting |

**Specialists** — production depth per discipline:

| Track | Focus |
| --- | --- |
| 🧱 Vue Architecture | app boundaries, feature modules, state architecture, evolution |
| ⚡ Web Performance Engineering | rendering pipeline, network, bundles, runtime, measurement |
| 🔐 Web Security | threat modeling, browser/app security, auth, secure development |
| ♿ Web Accessibility Engineering | WCAG/POUR, ARIA discipline, accessible Vue, a11y testing |
| ⚙️ Backend Rust Development | layered async services, APIs, databases, auth, production ops |
| 🚂 Production Node.js & Express | event loop, Express layering, package discipline, CI/containers |

**Projects** — 13 track projects, 5 cross-track projects (accessible Vue app, measured
performance optimization, secure full-stack, Rust-vs-Express comparison, and the
production full-stack system), plus the original Rust Playground capstone.

## Learning platform

- **Skill graph** — 62 declarative skills (`content/skills/skills.json`) with dependency
  edges; evidence derives from completed lessons/modules; shown on the `/tracks` catalog.
- **Rich progress** — per-lesson states (not-started → attempted → practicing → completed →
  mastered) from completions, quiz scores and exercise attempts — not just a boolean.
- **Deterministic recommendations** — unblock prerequisites → continue → review failed
  quizzes → close skill gaps → start unlocked projects; every suggestion explains why.
- **Grouped navigation** — dashboard and sidebar grouped by discipline (Backend, Frontend,
  Security, Architecture, Foundations, Operations) with lock tooltips naming the exact
  blockers.
- **Offline search** — across lessons, modules, tracks, projects, skills, code examples and
  personal notes.

## Development

```bash
npm install
npm run icons          # generate src-tauri/icons/icon.ico + 1024px icon.png (once)
npm run tauri dev      # desktop shell with HMR
npm run verify         # typecheck + vitest + cargo test
```

## Releases (macOS / Linux / Windows)

`.github/workflows/release.yml` builds installers for all three platforms and publishes a
GitHub Release automatically:

1. Bump `version` in `src-tauri/tauri.conf.json` if needed.
2. Commit, tag, and push:
   ```bash
   git tag v1.0.0        # must match the app version
   git push origin v1.0.0
   ```
3. CI builds per-platform bundles (`msi`/`nsis` on Windows, `dmg` on macOS,
   `deb`/`rpm`/`AppImage` on Linux) and publishes them on the release page.
   `workflow_dispatch` (Actions tab → Release → Run) does the same without a tag.

Builds are **unsigned**: macOS shows a Gatekeeper prompt (right-click → Open, or
`xattr -cr RustMasteryOffline.app`); Windows may show a SmartScreen warning.

## Definition-of-Done highlights

- Offline-first: curriculum embedded in the binary, Monaco bundled locally, zero network calls.
- Prerequisite engine: fully generic over the 18-track graph — Tauri unlocks at Rust M4,
  specialists gate on foundations (e.g. Backend tracks require HTTP; Express M6 requires
  Testing + DevOps + Observability), capstone needs Rust-M6 ∧ Vue-M4 ∧ Tauri-M3. Evaluated
  from declarative `module.json` data, never hardcoded routes; enforced by tests
  (`content_integrity.rs`).
- Sandboxed execution: isolated temp workspace, env allowlist, 5 s execution timeout,
  cancellation registry, deterministic cleanup (see docs/execution-security.md). Web-track
  exercises run in a sandboxed null-origin webview preview — never rustc.
- Lesson workspace: resizable **and collapsible** panes (VSCode-style, persisted),
  exercise→quiz auto-advance, notes/bookmarks, and a top-bar **⧉ Reset layout**.
- 🧪 **Playground** view: standalone Rust & Vue scratchpad — Rust via the sandbox,
  Vue via a sandboxed webview preview running a locally bundled Vue runtime
  (zero CDN, null-origin iframe).
- Curriculum gates tested: content integrity, unlock flow over the full catalog, skill-graph
  consistency, original-contract preservation (see `src-tauri/tests/content_integrity.rs`).
- Persistence: versioned SQLite migrations; progress (completions, attempts, quiz scores,
  skills, projects) survives restart.

## Documentation

Start with `docs/architecture.md`, then: `curriculum-architecture.md`, `learning-paths.md`,
`prerequisite-model.md`, `skill-model.md`, `project-model.md`, `content-architecture.md`,
`execution-security.md`, `persistence.md`, `testing-strategy.md` and the per-stack
architecture docs.
