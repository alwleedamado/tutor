# Learning Engine

## Entity Model

```
Track (rust | vue | tauri)
 └── Module        (id, slug, index, prerequisites[])
      └── Lesson   (id, slug, index, quiz?, exercise?, examples[])
Project            (first-class; belongs to a module or the capstone)
Capstone           (special project with multi-module AND-prerequisites)
```

## Unlock Model — Explicit Prerequisites, Zero Hardcoding

Every module declares its own `prerequisites: [moduleId]` list in `content/<track>/<module>/module.json`.
The engine evaluates that data generically:

```rust
pub fn module_is_unlocked(module: &ModuleDto, completed_modules: &HashSet<String>) -> bool {
    module.prerequisites.iter().all(|p| completed_modules.contains(p))
}
```

Derived facts:

- **Module completion** = every lesson of the module is in the completed-lesson set
  (a module always has lessons, so empty modules can never silently "complete").
- **Lesson unlock** = owning module is unlocked. Lessons inside an unlocked module are freely
  navigable; order is advisory (the outline highlights the recommended next lesson).
- **Track progress** = completed lessons / total lessons per track.
- **Recommendation** = first incomplete lesson scanning tracks in curriculum order
  (`rust → vue → tauri`), modules by `index`, skipping locked modules.

## Concrete Gates Shipped

| Gate                | Prerequisites                              |
|---------------------|--------------------------------------------|
| Rust modules 2..6   | previous Rust module                       |
| Tauri Module 1      | `rust-04` completed                        |
| Tauri Modules 2..3  | previous Tauri module                      |
| Vue modules 2..4    | previous Vue module (independent track)    |
| Capstone            | `rust-06` AND `vue-04` AND `tauri-03`      |

Adding a new gate is a content edit (one JSON array), never a code change.

## Persistence Mapping

The database stores only atomic facts: completed lesson rows and attempt records. Everything else
(locks, percentages, recommendation) is *derived* on read by the engine from
`(catalog, completed_lessons)` — no cached state to invalidate, no drift possible.

## Testing Contract

Pure-function unit tests cover: sequential chains, parallel independent tracks, the Tauri gate
(before/after `rust-04`), capstone AND semantics (each subset fails, all three pass), empty-catalog
safety, and recommendation ordering. See `src-tauri/src/engine/mod.rs#tests`.
