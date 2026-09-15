# Testing Strategy

## Philosophy

Test the critical logic where bugs are expensive: unlock rules, persistence integrity, IPC input
validation, execution boundaries, and store state transitions. UI snapshotting is avoided;
behavioral component tests are preferred.

## Test Pyramid

### Rust — `cargo test` (`npm run test:rust`)

| Layer              | What is verified                                                       |
|--------------------|------------------------------------------------------------------------|
| `engine` units     | unlock matrix, capstone AND-semantics, recommendation ordering          |
| `db` units         | migrations apply cleanly + idempotent; repo CRUD round-trips; unique constraints hold |
| `execution` units  | validator rejects oversized/NUL input; path traversal blocked; env allowlist correctness |
| `content` units    | catalog parses; prerequisite graph resolves; quiz/exercise shape valid  |
| integration        | full embedded-content integrity scan (`src-tauri/tests/content_integrity.rs`) |

Compilation itself is intentionally *not* asserted in unit tests (depends on local toolchain);
the runner is exercised through validator/orchestration seams instead.

### Vue — Vitest + Vue Test Utils (`npm run test`)

- **Stores:** `progress.store` (completion → derived stats → lock recomputation),
  `execution.store` lifecycle transitions with mocked ipc services,
  `settings.store` theme resolution incl. `system`.
- **Components:** `QuizPanel` scoring/pass flow, `ExercisePanel` progressive hints +
  solution gating, `LockBadge`/gating display.
- **Utils/composables:** markdown sanitizer behavior, route-target resolution helpers.
- **Preview engine:** `services/preview.ts` — SFC-lite parsing, binding
  collection, TS-relaxation, and generated-document shape (local Vue asset,
  import stripping, failure surfacing). See
  `src/services/__tests__/preview.spec.ts`.
- A throwaway live harness (`cargo run --example …`) exercised the real
  compile→run pipeline during development; it is intentionally not part of the
  default suite (toolchain-dependent, per the Rust section above).

- Environment: jsdom; every test builds a fresh `createPinia()` — no cross-test leakage.

### Desktop E2E

Tauri WebDriver would drive real windows; the repository ships the deterministic layers above and
documents `tauri-driver` as the follow-up harness (requires platform WebView binaries). Manual QA
checklist maps directly to Definition-of-Done items (offline launch, restart persistence,
lock/unlock matrix).

## Conventions

- Tests live beside code (`#[cfg(test)] mod tests`) plus `tests/` for cross-module guarantees.
- No network, no sleeps-based flakiness; time-dependent logic uses injected/instant values.
- Every bug fix adds a regression test first.

## Commands

```bash
npm run test          # vitest (frontend)
npm run test:rust     # cargo test (backend)
npm run typecheck     # vue-tsc --noEmit
npm run build         # production frontend bundle
cargo check           # backend compile gate
```
