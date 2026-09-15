# Tauri Architecture

## Configuration

- `tauri.conf.json` (schema v2): window 1280×800 (min 960×640), `beforeDevCommand: npm run dev`,
  `devUrl: http://localhost:5273`, `frontendDist: ../dist`, bundler targets intentionally left
  empty so local builds produce a raw executable without downloading platform installers.
- Identifier: `dev.rustmastery.offline`. Product name: `Rust Mastery Offline`.

## Capabilities (Least Privilege)

`src-tauri/capabilities/default.json` grants exactly one permission set to the `main` window:
`core:default` (events, basic window ops). **No** `shell`, `fs`, `http`, `dialog`, or `store`
plugins are exposed — every privileged operation goes through an application-defined command that
validates input Rust-side. Custom commands require no capability entries; plugins would.

## CSP

```
default-src 'self';
script-src 'self' 'wasm-unsafe-eval' 'unsafe-eval';  # wasm + Vue preview
style-src  'self' 'unsafe-inline';       # monaco injects styles
img-src    'self' data:;
font-src   'self' data:;
worker-src 'self' blob:;                  # monaco editor worker
connect-src 'self' ipc: http://ipc.localhost;
```

`unsafe-eval` is required by the Vue webview preview (runtime template
compilation of student snippets) — see `docs/execution-security.md` for the
isolation argument (null-origin sandboxed iframe, local content only). No
remote origins anywhere — the offline guarantee is enforced by policy, not
convention.


## IPC Surface (summary)

| Command                 | Direction | Notes                                    |
|-------------------------|-----------|------------------------------------------|
| `get_bootstrap`         | R→F       | catalog + progress + settings, one shot  |
| `complete_lesson` / `uncomplete_lesson` | F→R→F | returns refreshed ProgressSummary |
| `record_quiz_attempt` / `record_exercise_attempt` | F→R | audit trail for analytics-free history |
| `save_note`/`delete_note`/`list_notes`  | CRUD     | scoped by `scopeType:scopeId`  |
| `toggle_bookmark`/`list_bookmarks`      | CRUD     |                                |
| `save_settings`         | F→R       | persisted JSON row                        |
| `save_editor_state`/`get_editor_state`  | F↔R       | per-scope code snapshots                  |
| `save_project_state`/`get_project_states` | F↔R     | milestones + completion                   |
| `run_rust_code`         | F→R       | sandboxed compile+run, `RunResult`        |
| `cancel_execution`      | F→R       | kills a live run by id                    |
| `run_cargo_test`        | F→R       | contained to managed workspaces           |
| `get_system_info`       | R→F       | cached rustc/cargo versions, db path      |
| `reset_all_data`        | F→R       | destructive, confirmed twice in UI        |

Events are deliberately unused: every interaction is a request/response pair, which keeps the
protocol debuggable and avoids hidden ordering.

## Async & Lifecycle

Commands that spawn processes are `async fn` (tokio runtime). Setup hook opens/initializes the
database and sweeps stale sandbox directories. Exit relies on deterministic drop-guards that
remove workspaces.

## Packaging & Offline

All curriculum content is compiled into the binary (`include_dir`); Monaco, fonts, and styles are
bundled by Vite. After installation the app requires zero network access — including no telemetry
and no auto-update fetches.
