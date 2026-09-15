# Menus, Tabs, Tables & Dynamic Content

Composite patterns have canonical ARIA structures. Learn four you will meet in every app.

## Tabs

```html
<div role="tablist" aria-label="Billing period">
  <button role="tab" id="tab-m" :aria-selected="active === 'm'" aria-controls="panel-m">Monthly</button>
  <button role="tab" id="tab-y" :aria-selected="active === 'y'" aria-controls="panel-y">Yearly</button>
</div>
<div id="panel-m" role="tabpanel" aria-labelledby="tab-m" v-show="active === 'm'">…</div>
```

Tabs **switch visible content in place**. If clicking navigates to another URL, they are links — not tabs. Arrow keys move between tabs; only the selected panel is in the tab order.

## Menu vs navigation

A `menu` holds *actions* (delete, duplicate). Site navigation is `nav` + links. The most common misuse: nav bars with `role="menu"` — readers then expect arrow-key menus and stop announcing destinations as links.

## Tables

```html
<table>
  <caption>Invoice history</caption>
  <tr><th scope="col">Date</th><th scope="col">Amount</th></tr>
  <tr><td>2026-01-05</td><td>€12.99</td></tr>
</table>
```

`caption` names the table; `th scope` binds headers to cells. Without them, a reader announces cell values with no context.

## Dynamic content announcements

```html
<div aria-live="polite" role="status">{{ syncMessage }}</div>
```

Mounted *before* the update happens (a persistent region), polite for routine updates ("3 results", "Saved"). Assertive only for urgent failures.

## Best practices

- Copy the APG (WAI-ARIA Authoring Practices) keyboard model for any widget you build — don't invent one.
- Interactive controls inside panels: keep them reachable only when visible.

## Common mistakes

- `role="menu"` on site navigation.
- Tables built from divs with no headers.
- Live region mounted in the same tick as the message (mount-then-announce fails).
