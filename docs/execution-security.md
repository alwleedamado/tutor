# Execution Security Model

## Threat Model

Student-authored Rust source is **hostile input**: arbitrary code that must be compiled and run
without endangering app data, secrets, or system stability. Student code therefore never executes
inside the Tauri process.

## Sandbox Design (defense in depth)

1. **Isolation by process.** Compilation and execution happen in short-lived child processes
   spawned by the execution service, never in-process.
2. **Disposable workspaces.** Each run gets a fresh directory under
   `%TEMP%/rust-mastery-sandbox/<uuid>/` containing only `main.rs`. The DB path, app resources,
   and app secrets are never placed inside it.
3. **Environment allowlist.** Child processes receive only: `PATH`, `SystemRoot`, `SystemDrive`,
   `TEMP/TMP`, `USERPROFILE`, `HOMEDRIVE/HOMEPATH`, `COMSPEC`, `PATHEXT`,
   `NUMBER_OF_PROCESSORS`, `PROCESSOR_ARCHITECTURE`, `WINDIR`. App-specific env is stripped.
4. **Hard timeouts.** Compile ≤ 30 s (cold toolchain warm-up), execution ≤ 5 s. On expiry the
   child is killed and the result flagged `timedOut: true`.
5. **Cancellation registry.** Every run registers an `Arc<Mutex<Child>>` + cancel flag keyed by
   execution id; `cancel_execution` flips the flag and kills the process tree handle. A polling
   wait loop (100 ms) reacts to cancel/timeout promptly while still collecting partial output.
6. **Output caps.** stdout/stderr truncated at 64 KiB each to bound memory.
7. **Deterministic cleanup.** Drop-guard deletes the workspace regardless of success, timeout, or
   panic; a best-effort sweep removes stale sandbox dirs at startup (> 1 h old).
8. **Path containment.** `run_cargo_test(path)` rejects absolute escapes, `..` traversal, and any
   path outside managed workspace roots before touching the filesystem.

## Input Validation (Rust-side, untrusted frontend)

- Source length ≤ 64 KiB; reject NUL bytes.
- Execution ids must match the registry; unknown ids are a validation error, not a panic.
- All command arguments deserialized into typed structs; enum-like strings validated explicitly.

## Diagnostics as Pedagogy

Compiler stderr is parsed line-wise into structured diagnostics
(`severity ∈ {error, warning, note, help}`, message, optional span header), letting the UI render
educational error cards instead of raw walls of text.

## In-Webview Vue Preview (added)

Vue-track exercises run in a **sandboxed iframe** (`sandbox="allow-scripts"` →
null origin) instead of the rustc sandbox:

- The preview document embeds a locally bundled `vue.global.prod.js` (Vite
  `?url` asset) — still zero network.
- Runtime template compilation requires `unsafe-eval`; CSP `script-src` now
  includes it. This is scoped to the app's own webview executing *locally
  authored* snippet code; the Rust-side sandbox guarantees are unchanged.
- Because the iframe is null-origin and `connect-src 'self' ipc:` remains,
  preview code cannot reach app data, storage, or the IPC bridge.
- `defineEmits` calls surface in an in-preview event log so component
  contracts stay observable without a parent host component.

## Known Limitations (documented honestly)


- Windows Job Objects / cgroups-grade CPU+memory caps are not yet enforced; timeouts + output caps
  provide the primary containment. Native debuggers/Miri remain manual tools taught in lessons.
- Compiled student code could still open network sockets within its 5 s window; the sandbox
  mitigates persistence and data access, not per-run egress. WASM execution is the designated
  fallback for future stricter isolation needs (the CSP already reserves `wasm-unsafe-eval`).

These limitations are recorded deliberately — the security doc must never overclaim.
