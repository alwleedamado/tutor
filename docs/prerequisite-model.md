# Prerequisite Model

Prerequisites remain **declarative data evaluated by a generic engine**. No track, gate, or
unlock rule is hardcoded in components or routes.

## Mechanism (unchanged by the expansion)

- Each `module.json` declares `prerequisites: [moduleId]` — module ids may span tracks.
- The engine (`engine::module_states`) marks a module **locked** unless *every* prerequisite
  module is fully completed (all lessons).
- The router guard blocks lesson routes for locked modules; the capstone evaluates its own
  prerequisite list from `capstone.json`.
- Bootstrap/progress commands recompute states after every mutation, so UI and DB never
  drift.

## The full prerequisite graph

```text
CORE (unchanged)
  rust-01..06            sequential chain
  vue-01..04             sequential chain, independent of rust/tauri
  tauri-01               requires rust-04
  tauri-02..03           sequential
  capstone               rust-06 ∧ vue-04 ∧ tauri-03

FOUNDATIONS (open substrate)
  http-01                []            http-02, http-03 require http-01
  arch-01                []            arch-02, arch-03 sequential
  test-01                []            test-02 requires test-01
  git-01                 []            git-02 requires git-01
  db-01                  []            db-02 requires db-01
  linux-01               []            linux-02 requires linux-01
  api-01                 requires http-01        api-02 requires api-01
  devops-01              requires linux-01       devops-02 requires devops-01
  obs-01                 []

SPECIALIZED
  varch-01  requires vue-01          varch-02..05 sequential (04 also 03)
  perf-01   requires http-01
  perf-02   requires http-02
  perf-03   requires perf-01 ∧ vue-01
  perf-04   requires perf-03
  perf-05   requires perf-04
  sec-01    requires http-01
  sec-02    requires http-03         sec-03 requires sec-02
  sec-04    requires sec-01          sec-05 requires sec-03 ∧ sec-04
  a11y-01   requires vue-01          a11y-02..04 sequential
  brust-01  requires rust-04 ∧ http-01
  brust-02  requires brust-01        brust-03 requires brust-01
  brust-04  requires brust-02        brust-05 requires brust-03 ∧ brust-04
  express-01 requires http-01
  express-02 requires express-01     express-03 requires express-02
  express-04 requires express-02
  express-05 requires express-02 ∧ sec-01
  express-06 requires express-04 ∧ express-05 ∧ test-01 ∧ devops-01 ∧ obs-01
```

## Invariants (test-enforced)

`src-tauri/tests/content_integrity.rs::declarative_gates_match_the_curriculum_contract`
asserts the contract holds over the real embedded content:

- Original gates never regress (tauri-01 → rust-04; vue independence; capstone triple).
- Specialization edges exist as documented above (perf/sec → http; brust → rust-04 ∧ http;
  express-06 → testing+devops+obs; devops → linux; …).
- Every prerequisite references an existing module (loader-level validation).
- End-to-end unlock flow: full completion opens everything; one missing lesson re-locks
  its dependents.

## Design rules

1. **Foundations are shared prerequisites** — HTTP feeds security/performance/backends;
   testing feeds production backends; linux feeds devops. Taught once, referenced widely.
2. **Cross-track edges use module ids** — the engine is already track-agnostic.
3. **No new gating mechanisms** — skills and recommendations are derived views, never gates.
