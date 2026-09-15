# Skill Model

Progress tracking beyond module completion: a declarative **skill graph** with evidence
derived from real learning activity.

## Declaration

`content/skills/skills.json`:

```json
{
  "version": 1,
  "skills": [
    {
      "id": "rust-ownership",
      "label": "Rust Ownership & Borrowing",
      "category": "Languages",
      "summary": "Moves, borrows, slices and lifetime intuition.",
      "requires": ["rust-fundamentals"]
    }
  ]
}
```

62 skills form a dependency graph mirroring the curriculum's shape:

```text
Rust Ownership → Rust Async → Backend Rust → Production Backend
HTTP → Caching → Web Performance → Performance Engineering
Semantic HTML → ARIA → Accessible Vue → Accessibility Engineering
```

## Evidence

Content declares evidence; the engine derives states:

- A **lesson** granting skill `s` is evidence (lesson `skills[]`).
- A **module** granting skill `s` is evidence (module `skills[]`).
- A skill is **demonstrated** when every evidence lesson is completed and every evidence
  module is completed.
- **Gateway skills** (no direct evidence) are demonstrated only through their
  `requires` closure — dependency edges never substitute for unfinished evidence.
- A skill's **dependencies are met** when all `requires` are demonstrated;
  `missingDependencies` names what to learn next (by label — explainable).

Computation lives in `engine::skills` (pure functions over `(catalog, graph, done_keys)`,
unit-tested). States ship in `ProgressSummary.skills` and render on `/tracks` and in
search.

## Example chains

```text
http-fundamentals ─→ http-browser-model ─→ sec-browser ─→ sec-app ─→ sec-practice
rust-generics-traits ─→ tauri-fundamentals ─→ tauri-advanced
vue-fundamentals ─→ vue-arch-fundamentals ─→ vue-arch-features ─→ vue-arch-services
```

Completing a module marks its skill's module evidence done; the graph then lights up
downstream skills whose dependencies are now met ("unlocked, in progress").

## Rich lesson progress

Per-lesson states are derived (no new tables) from completions + attempt evidence:

| State | Meaning |
| --- | --- |
| not-started | no completions, no attempts |
| attempted | exercise tried (or quiz attempted), <3 attempts, no solution reveal |
| practicing | ≥3 attempts or solution revealed without solving |
| completed | lesson completed (but quiz not passed / exercise unsolved) |
| mastered | completed + quiz passed + exercise solved without revealing the solution |

States ship in `ProgressSummary.lessonProgress` and back the Progress view. Attempts and
quiz scores persist in the existing `quiz_attempts` / `exercise_attempts` tables (aggregated
by new repo queries) — no schema change was required.

## What the model deliberately avoids

- No gamification points — skills describe competence, not score.
- No time tracking — activity evidence (attempts, quiz scores, completions) is what the
  platform can measure honestly offline.
- Skills never gate navigation; prerequisites (module-level) remain the only gate.
