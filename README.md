# Rust Mastery Offline

A production-quality, **fully offline** desktop learning environment for **Rust**, **Vue 3**, and
**Tauri v2** — built with Tauri v2, Vue 3 (`<script setup>` + Pinia), TypeScript, TailwindCSS,
Monaco Editor, and SQLite.

> **Architectural constraint:** the application runs on modern architecture (Pinia, Composition
> API, structured Rust errors). Vuex appears **only** as embedded tutorial text under
> `content/tutorials/` and is never installed, imported, or executed by the app.

## Layout

```
docs/        architecture documentation (start with docs/architecture.md)
src/         Vue 3 frontend (views, stores, services, composables)
src-tauri/   Rust backend (commands, learning engine, SQLite, execution service)
content/     curriculum: rust/ vue/ tauri/ projects/ capstone/ tutorials/
scripts/     build utilities (icon generation)
```

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
- Prerequisite engine: Tauri track unlocks at Rust Module 4; capstone needs Rust-M6 ∧ Vue-M4 ∧
  Tauri-M3 — evaluated generically from declarative prerequisites, never hardcoded routes.
- Sandboxed execution: isolated temp workspace, env allowlist, 5 s execution timeout,
  cancellation registry, deterministic cleanup (see docs/execution-security.md).
- Lesson workspace: resizable **and collapsible** panes (VSCode-style, persisted),
  exercise→quiz auto-advance, notes/bookmarks, and a top-bar **⧉ Reset layout**.
- 🧪 **Playground** view: standalone Rust & Vue scratchpad — Rust via the sandbox,
  Vue via a sandboxed webview preview running a locally bundled Vue runtime
  (zero CDN, null-origin iframe).

- Persistence: versioned SQLite migrations; progress survives restart.
