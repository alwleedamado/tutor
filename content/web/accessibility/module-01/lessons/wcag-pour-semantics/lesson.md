# WCAG, POUR & Semantic HTML

Accessibility engineering starts with two frameworks: **WCAG** (the standard) and **semantic HTML** (the cheapest, strongest tool).

## WCAG in one minute

WCAG organizes requirements into four principles — **POUR** — with three conformance levels (A, AA, AAA). AA is the normal production/legal target.

- **Perceivable** — information reaches all senses: alt text, captions, contrast.
- **Operable** — everything works without a mouse: keyboard, focus, timing.
- **Understandable** — predictable behavior, clear errors, readable language.
- **Robust** — valid markup that assistive tech can parse.

## Semantic HTML is 80% of the job

```html
<!-- native semantics: focusable, labelled, announced — zero ARIA needed -->
<button type="button" @click="save">Save</button>

<label for="email">Email</label>
<input id="email" type="email" required />

<nav aria-label="Main">…</nav>
<main>…</main>
```

A native `<button>` ships focus handling, Enter/Space activation, and screen-reader announcement. A `div` with a click handler ships none of it — and no amount of ARIA fully recreates the native behavior.

## Accessible names

Every control needs a name a screen reader can announce: button text, `label for`, `aria-label` when no visible text exists. Icon-only buttons are the classic failure (`aria-label="Close"` or it announces "button").

## Best practices

- Reach for an HTML element before a `div` + ARIA.
- Run WCAG checks against AA as the baseline; treat AAA as selective extras.
- Write alt text that conveys *purpose* ("Search"), not appearance ("magnifying glass icon").

## Common mistakes

- Divs with click handlers (invisible to keyboard and readers).
- Icon-only buttons with no accessible name.
- Captions/labels skipped because "the design didn't include them" — the design is wrong, fix the design.
