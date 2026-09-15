# Frontend Architecture

## Stack

Vue 3 (`<script setup>` Composition API) · TypeScript · Vite · Vue Router 4 · **Pinia (only)**
· TailwindCSS v4 · Monaco Editor (bundled locally, no CDN).

## Layer Rules

```
views/components  → composables → stores → services/ipc.ts → Tauri
```

- Components: presentation + interaction. No direct `invoke()` calls, no SQL knowledge, no
  business decisions beyond trivial view state.
- Composables: reusable behavior (`useTheme`, `useMarkdown`, `useShortcut`). No global mutable
  module state — all shared state lives in Pinia.
- Stores: own state + actions; actions call services and normalize results.
- Services: one function per IPC command, typed request/response DTOs from `src/types/domain.ts`.
  This is the only file allowed to import `@tauri-apps/api/core`.

## Folder Layout

```
src/
├── main.ts                 # createApp, pinia, router, theme bootstrap
├── App.vue                 # shell + router-view (+ bootstrap error screen)
├── assets/styles/main.css  # Tailwind entry, tokens, shell NEVER scrolls
├── types/domain.ts         # Shared DTO contract mirrored by serde structs
├── constants/starters.ts   # Shared editor starters (lessons + playground)
├── router/index.ts         # Route table + single prerequisite guard
├── services/
│   ├── ipc.ts              # Typed invoke wrappers (single Tauri seam)
│   └── preview.ts          # Offline Vue webview-preview engine
├── stores/                 # progress, editor, settings, lessons,
│                           # project, execution, library (Pinia only)
├── composables/            # useMarkdown, useShortcut
├── components/
│   ├── common/SplitPane.vue  # resizable + collapsible two-pane splitter
│   └── layout/ dashboard/ editor/ lesson/
└── views/                  # incl. LessonView (pane tree), PlaygroundView
```

## Workspace layout system (VSCode model)

The shell (`html/body/#app`) is `height:100%; overflow:hidden` — **the page
never scrolls**; every pane owns its scrolling. Views are full-height pane
trees built from `SplitPane` (`components/common/SplitPane.vue`):

- Drag dividers; double-click resets that split's proportions.
- Chevron buttons on each divider **collapse/expand either side**; expanding
  restores the pane's previous size.
- Keyboard: dividers are `role="separator"`; arrows adjust (Shift = bigger
  step), Enter resets.
- Sizes + collapse state persist per `storageKey`
  (`localStorage["rm-split:<key>"]`); the top bar's **⧉ Reset layout** clears
  every `rm-split:*` key and reloads.

Lesson workspace tree: `[outline | content | code(editor‖output/preview)]`
over `[quiz/exercise/notes tabs]` — keys `lesson-main/mid/work/code`.

## Vue webview preview

`services/preview.ts` builds a standalone HTML document (template +
`<script setup>` + style extraction, light TS relaxation, import stripping)
executed against a locally bundled Vue (`public/vendor/vue.global.prod.js`,
zero CDN). `PreviewFrame.vue` renders it in a `sandbox="allow-scripts"`
iframe (null origin → no app/IPC access) with debounced rebuilds, theme
awareness, an error overlay and a `defineEmits` log bar. The runtime URL is
resolved **absolutely against the app's baseURI** — sandboxed iframes must
never rely on their own base resolution.

## Learning flow

Lesson → run/play (rustc sandbox or webview preview per track) → **solving
the exercise auto-advances to the quiz** (when present: `ExercisePanel` emits
`solved`, `LessonView` switches tab) → mark lesson complete.

## Routing Contract

Required: `/`, `/rust/:module/:lesson`, `/vue/:module/:lesson`, `/tauri/:module/:lesson`,
`/project/:id`, `/settings`. Additional: `/playground/:lang?` (standalone
Rust/Vue scratchpad with persistent per-language buffers), plus track entry
shortcuts `/rust` · `/vue` · `/tauri` and `/:track/:module` — both resolve
through `TrackEntryView` to the next actionable lesson. Track/module/lesson
segments use slugs resolved through `lessons.store` lookup helpers.

Prerequisite enforcement lives **once**, in `router/index.ts`: the global guard asks the progress
store whether the target module/capstone is locked and redirects to the dashboard otherwise.
Entry routes (no lesson param) are explicitly exempted from the lesson check.
Components never duplicate this logic.

All views except `DashboardView` are lazy-loaded via dynamic `import()` for route-level code
splitting.


## State Ownership

| Store            | Owns                                                            |
|------------------|-----------------------------------------------------------------|
| lessons          | Catalog, selected track/module/lesson, search query/results     |
| progress         | Completed lesson keys, module stats, lock flags, next up        |
| editor           | Open documents keyed by scope (`track:module:lesson`), cursors  |
| execution        | Run lifecycle (`idle/compiling/running/done/error`), output     |
| settings         | Theme, fontSize, tabSize, reducedMotion, highContrast           |
| project          | Project specs, milestone check state, completion                |
| library          | Notes list, bookmarks set                                       |

## Styling & Theming

Tailwind v4 with CSS-first config. A custom variant `@custom-variant dark` keys off a `.dark`
class on `<html>` applied by `useTheme` (supports `system`). Design tokens (colors, fonts)
declared via `@theme`; semantic utility classes defined once in `main.css`. Fonts are the system
stack — zero remote fonts.

## Accessibility

Semantic landmarks, labelled controls (`aria-label`), focus-visible rings, keyboard navigation for
the lesson workspace (Ctrl+Enter run, Ctrl+S save), respects `prefers-reduced-motion` unless the
user overrides it, WCAG 2.2 AA contrast targets in both themes.

## Testing

Vitest + Vue Test Utils + jsdom. Stores tested with mocked services; components tested with real
Pinia instances created per-test (`createPinia()`), never a global singleton. Monaco is excluded
from component tests behind a thin `CodeEditor` wrapper so tests stay fast and deterministic.
