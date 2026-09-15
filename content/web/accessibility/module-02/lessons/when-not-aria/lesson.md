# When NOT to Use ARIA

> **The first rule of ARIA: don't use ARIA if a native element does the job.**

ARIA re-creates semantics that HTML already ships — usually worse and always with more code.

## The div-button anti-pattern

```html
<!-- ❌ needs role, tabindex, keydown for Enter AND Space, focus styles…
        and still announces wrong in some readers -->
<div role="button" tabindex="0" @click="save" @keydown.enter="save">Save</div>

<!-- ✅ all of the above, already correct, zero JS -->
<button type="button" @click="save">Save</button>
```

The div version misses: form submission semantics, disabled state announcements, implicit `type`, activation on Space. Every "fix" is re-implementing the platform.

## ARIA that contradicts semantics

```html
<a role="button" href="/delete">Delete</a>   <!-- navigation disguised as action -->
<h3 role="button">Toggle</h3>                <!-- heading removed from the outline -->
```

Changing a role overrides the native one — a link announced as a button breaks the user's model of what clicking does.

## When ARIA IS right

- Composite widgets HTML lacks: `tablist`, `tree`, `combobox`, `grid`.
- Live regions for async announcements.
- Bridging gaps: `aria-live`, `aria-expanded` on custom disclosure widgets built on native buttons.

The pattern: **native base element + ARIA states/relationships**, never native replaced by ARIA.

## Best practices

- Grep your codebase for `role="button"` — each hit is a review question.
- Before adding any ARIA, search MDN for the native element that already does it.

## Common mistakes

- `role="presentation"`/`aria-hidden` on focusable content (element disappears from readers but is still tabbable — WCAG failure).
- Sprinkling `aria-label` to "fix" unclear button text instead of writing better text.
- Assuming ARIA makes something accessible: ARIA is announcement only — keyboard behavior is still your job.
