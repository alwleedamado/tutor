# Curriculum Architecture

How the expanded curriculum is organized, stored, and loaded. This document describes the
system as of the curriculum expansion (18 tracks); the original three-track design is
preserved as the head of the catalog.

## Track registry

The catalog is derived from a **declarative registry** in `src-tauri/src/content/mod.rs`:

```rust
pub static TRACKS: &[TrackDecl] = &[
    TrackDecl { id: "rust",  dir: "rust",  title: …, description: …, category: "Languages" },
    …18 entries…
]
```

- `id` — stable route/identity (`/web-performance/…`).
- `dir` — content directory under `content/`; may group domains (`web/performance`).
- `category` — dashboard grouping only (Foundations, Languages, Frontend, Desktop,
  Backend, Architecture, Security, Performance, Quality, Operations).

Adding a track = add content + one registry row. No engine or UI code changes.

## Catalog order

Registry order defines default recommendation order:

1. Core tracks — `rust`, `vue`, `tauri` (original curriculum, unchanged)
2. Foundations — `http`, `architecture`, `testing`, `git`, `api-design`, `databases`,
   `linux`, `devops`, `observability`
3. Specialized — `vue-architecture`, `web-performance`, `web-security`,
   `web-accessibility`, `backend-rust`, `backend-express`

## Directory layout

```text
content/
├── rust/  vue/  tauri/            # original tracks (1-segment dirs)
├── web/{http,performance,security,accessibility}/
├── backend/{rust,node-express}/
├── architecture/  testing/  git/  api-design/
├── databases/  linux-tooling/  devops/  observability/
├── skills/skills.json             # declarative skill graph
├── projects/<id>/project.json     # incl. cross-track projects A–E
└── capstone/capstone.json         # unchanged
```

The loader accepts module dirs one level under the track dir (multi-segment track dirs
supported). A declared track with no content yet is skipped at load time; the
`content_integrity` test enforces that every declared track ships.

## Content model extensions (all serde-defaulted — old content unchanged)

| Entity | New fields |
| --- | --- |
| Track | `category` |
| Module | `difficulty` ("beginner"/"intermediate"/"advanced"), `skills[]` |
| Lesson | `skills[]` (skill-graph evidence), `related[]` (informational cross-links) |

Lesson bodies follow the established `lesson.json`-inline or sibling `lesson.md` convention.

## Prerequisites

Module-level `prerequisites: [moduleId]` remain the only gating mechanism — fully generic
evaluated by the engine (see `docs/prerequisite-model.md`). Lesson-level metadata
(`skills`, `related`) is declarative/derived and never gates navigation.

## Skill graph

`content/skills/skills.json` declares 62 skill nodes with `requires` edges. Evidence flows
from content: a skill is *demonstrated* when every lesson and module declaring it is
completed (see `docs/skill-model.md`).

## Anti-duplication principle

Foundations exist once and are prerequisite inputs to every specialized track that needs
them (HTTP → security/performance/backends; testing → production backends; architecture →
vue-architecture). Specialized tracks reference foundation concepts rather than re-teaching
them; cross-links (`related`) make the connections navigable.
