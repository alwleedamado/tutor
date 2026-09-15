# Keyboard Accessibility & Focus Management

If it can't be reached and operated with a keyboard, it isn't accessible — full stop. Keyboard testing is also the fastest audit anyone can run.

## The audit: unplug the mouse

1. Tab from the address bar — is every control reachable, in a sensible order?
2. Is focus **always visible**?
3. Do Enter/Space activate the focused control?
4. Does Escape close overlays?
5. Any keyboard trap that isn't a modal?

## Focus visibility is a feature

```css
/* never just outline: none — restyle instead */
:focus-visible {
  outline: 3px solid #14b8a6;
  outline-offset: 2px;
}
```

`:focus-visible` gives mouse users the clean look and keyboard users the indicator they cannot use the app without.

## Skip links

The first Tab stop should offer escape from repetitive nav:

```html
<a href="#main" class="skip-link">Skip to content</a>
```

```css
.skip-link { position: absolute; left: -9999px; }
.skip-link:focus { left: 8px; top: 8px; }
```

## Focus management in apps

- **Dialog opens** → focus moves into it (the dialog, not body).
- **Dialog open** → Tab is trapped inside; Escape closes.
- **Dialog closes** → focus returns to the triggering element.
- **Route changes** → move focus to the new page's `h1` (or main) so screen reader users start at the top, not mid-nav.

## Best practices

- Fix order in the DOM, not with `tabindex` values — positive `tabindex` is almost always a bug.
- Test focus behavior in dark mode too; invisible focus often hides there.

## Common mistakes

- `outline: none` with no replacement.
- Modals that don't trap or restore focus.
- Custom dropdowns that ignore Arrow keys and Escape.
