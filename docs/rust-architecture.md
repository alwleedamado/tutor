# Rust Architecture (src-tauri)

## Crate Layout

```
src-tauri/src/
├── main.rs              # entry point → rust_mastery_lib::run()
├── lib.rs               # tauri::Builder wiring, managed state, module graph
├── error.rs             # AppError (thiserror) + IPC-safe serialization
├── state.rs             # AppState { db: Mutex<Connection>, … }
├── db/
│   ├── mod.rs           # open(), MIGRATIONS runner (transactional, versioned)
│   └── repo.rs          # repositories: progress, notes, settings, projects…
├── content/
│   ├── mod.rs           # include_dir! embedding + parsing into Catalog
│   └── model.rs         # Track/Module/Lesson/Quiz/Exercise serde DTOs
├── engine/
│   ├── mod.rs           # learning engine: unlock/completion/recommendation
│   └── tests            # inline unit tests (pure functions)
├── execution/
│   ├── mod.rs           # orchestration: compile → run → collect → cleanup
│   ├── validator.rs     # input validation + workspace containment checks
│   └── registry.rs      # cancellation registry (Arc<Mutex<Child>> + flags)
└── commands/
    ├── mod.rs           # command registry
    ├── bootstrap.rs     # get_bootstrap
    ├── progress.rs      # complete/uncomplete, quiz/exercise attempts
    ├── notes.rs         # notes + bookmarks CRUD
    ├── settings.rs      # settings load/save, editor_state
    ├── projects.rs      # project milestone persistence
    └── exec.rs          # run_rust_code, cancel_execution, run_cargo_test, system info
```

## Rules

1. **Commands are adapters.** Parse args → call service → map error. No SQL, no subprocesses, no
   business rules inside command bodies.
2. **Domain purity.** `engine::*` is a pure function library over `(catalog, completed_set)` —
   trivially unit-testable without Tauri or SQLite.
3. **State discipline.** Managed state: `AppState` (DB mutex), `ExecutionRegistry`
   (cancellation handles). No `static mut`, no lazy globals except an idempotent system-info cache.
4. **Structured errors.** Every fallible command returns `Result<T, AppError>`; `AppError`
   serializes as `{ kind, message }` (camelCase) matching `AppErrorDto` in TypeScript.
5. **Validation at the boundary.** Sizes, slugs, and paths are validated before any service runs;
   frontend data is treated as untrusted.

## Concurrency Model

Tauri v2 runs async commands on its tokio runtime. Blocking SQLite calls run directly (fast,
indexed queries); process execution uses `tokio::process` with `tokio::time` timeouts. The DB
`Mutex<Connection>` is held only for the duration of a statement batch — long operations
(compilation) never contend with it.

## Testing

- Unit tests: engine unlock matrix, migrations apply/idempotent, repository round-trips,
  execution validator (path containment, size caps).
- Integration tests (`src-tauri/tests/`): full content-integrity scan of the embedded catalog
  (every prereq resolves, every quiz answer in range, every lesson has markdown).
- Run with `npm run test:rust` (`cargo test --manifest-path src-tauri/Cargo.toml`).
