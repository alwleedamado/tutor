# ARIA Roles, States & Properties

When native HTML can't express a widget, ARIA fills the gap — announced to assistive tech through three kinds of attributes:

- **Roles** — what it *is*: `role="tab"`, `role="dialog"`, `role="alert"`.
- **States** — current condition, changes at runtime: `aria-expanded`, `aria-checked`, `aria-busy`.
- **Properties** — relationships and metadata: `aria-labelledby`, `aria-describedby`, `aria-live`.

## A working example

```html
<button aria-expanded="false" aria-controls="menu" @click="toggle">
  Actions
</button>
<ul id="menu" v-show="open">
  …
</ul>
```

`aria-expanded` flips with `open` — the state change is what users hear ("Actions, collapsed button" → "expanded").

## Landmarks

`header`, `nav`, `main`, `aside`, `footer` map to landmark roles, letting screen-reader users jump between regions. One `main` per page; label repeated landmarks (`aria-label="Footer navigation"`).

## Live regions

Async updates are invisible to readers unless announced:

```html
<div aria-live="polite" role="status">{{ savedMessage }}</div>
```

`polite` waits for a pause; `assertive` interrupts — reserve it for genuine emergencies.

## Labelling

```html
<input id="email" aria-describedby="email-hint email-error" />
<p id="email-hint">We never share it.</p>
<p id="email-error" role="alert">Enter a valid email.</p>
```

`aria-labelledby`/`describedby` compose the element's spoken description from existing content.

## Best practices

- Add ARIA **with** the behavior it promises — `role="checkbox"` without Space handling is a lie.
- Prefer `aria-labelledby` over `aria-label` when visible text exists.
- Test with an actual screen reader (NVDA is free) — ARIA that exists but misfires is worse than none.

## Common mistakes

- `aria-hidden="true"` on the element that currently has focus (focus points at nothing).
- Duplicate roles on native elements (`<button role="button">` — redundant).
- Live regions mounted *during* the update: they must exist beforehand to announce.
