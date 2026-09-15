# Content Architecture

## Location & Embedding

Curriculum lives in `/content`, is validated at build/test time, and is embedded into the binary at
compile time via `include_dir!("$CARGO_MANIFEST_DIR/../content")`. Embedding guarantees offline
availability, atomic versioning with the app, and immunity to file tampering at runtime.

## Directory Convention

```
content/
├── rust/
│   ├── module-01-fundamentals/
│   │   ├── module.json          # id, slug, index, title, summary, prerequisites[], lessons[]
│   │   └── lessons/<slug>/
│   │       ├── lesson.md        # instructional markdown
│   │       └── lesson.json      # metadata + activities bundle (below)
│   └── …
├── vue/module-01-…/ …           # includes state-management pinia/vuex/migration parts
├── tauri/module-01-…/ …
├── projects/<project-id>/project.json
├── capstone/capstone.json
└── tutorials/                   # ISOLATED educational artifacts only
    └── vue/state-management/{pinia,vuex,migration}/…
```

## Lesson Activity Bundle (`lesson.json`)

One declarative bundle per lesson keeps validation simple and loader code small:

```jsonc
{
  "id": "rust-02-l01", "title": "Ownership", "slug": "ownership",
  "minutes": 25, "kind": "concept",
  "objectives": ["Explain move semantics", "…"],
  "smells": [{ "name": "Excessive cloning", "problem": "…", "why": "…",
               "badCode": "…", "analysis": "…", "refactor": "…", "goodCode": "…" }],
  "examples": [{ "name": "moves.rs", "language": "rust", "code": "…" }],
  "exercise": { "id": "ex-1", "prompt": "…", "kind": "implementation|bug-hunt",
                "starterCode": "fn main() { … }", "language": "rust",
                "hints": ["hint 1", "hint 2"], "solution": "fn main() { … }" },
  "quiz": { "id": "quiz-1", "passScore": 70,
            "questions": [{ "id": "q1", "prompt": "…", "options": ["a","b"],
                            "answerIndex": 0, "explanation": "…" }] }
}
```

## Inline lesson bodies

A lesson's markdown may live either in the sibling `lesson.md` (takes
precedence) or inline as the `body` string field of `lesson.json`. Inline
bodies keep the embedded tree compact; the loader accepts both and the
integrity tests enforce non-empty markdown either way.

## Validation Rules (enforced by tests + startup parse)


1. Every prerequisite id resolves to an existing module.
2. Every lesson folder referenced by a module contains both files; ids are unique globally.
3. Quiz answers are within range; quizzes have ≥ 1 question; exercises have ≥ 1 hint.
4. Tracks/modules have stable slugs used verbatim in routes.
5. Tutorial artifacts under `content/tutorials/**` are plain text/markdown/js — they are never
   parsed into the runtime catalog and never executed by the app.

## The Vuex Isolation Rule

Vuex material exists **only** as readable example code and prose under
`content/tutorials/vue/state-management/vuex/`. It is educational text rendered in a `<pre>`
block — not a dependency, not an import, never registered on any Vue app instance. The app's
runtime state layer is Pinia exclusively (see `package.json`: no `vuex` dependency exists).

## Code Smell Teaching Model

Each smell entry follows: Problem → Why it hurts → Bad example → Analysis → Refactoring strategy →
Improved example → linked exercise, so lessons teach judgment rather than trivia.
