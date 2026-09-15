# Accessible Forms, Validation & Dialogs

Forms and dialogs are where accessibility work pays off most — and where Vue apps most often break it.

## Fields: label, hint, error

```html
<label for="email">Email</label>
<input
  id="email"
  v-model="email"
  type="email"
  :aria-invalid="!!error"
  aria-describedby="email-hint email-error"
/>
<p id="email-hint">We never share it.</p>
<p v-if="error" id="email-error" role="alert">{{ error }}</p>
```

- `label for` names the field (clicking the label focuses it too).
- `aria-describedby` links hint and error text into the field's announcement.
- `role="alert"` announces the error as it appears.
- `aria-invalid` signals state — never color alone.

## Dialogs: the full duty list

1. **On open**: move focus into the dialog (the first control or the heading).
2. **While open**: trap Tab inside; Escape closes; background is `aria-hidden`/inert.
3. **On close**: restore focus to the element that opened it.

```html
<div role="dialog" aria-modal="true" aria-labelledby="dlg-title">
  <h2 id="dlg-title">Confirm deletion</h2>
  <button type="button" @click="confirm">Delete</button>
  <button type="button" @click="close">Cancel</button>
</div>
```

A `useFocusTrap` composable (listener for Tab, wrap at the edges, `document.activeElement` saved on open) makes this reusable across every modal.

## Best practices

- Validate on submit + on blur, announce politely; never yank focus to an error.
- Placeholders are not labels — they disappear while typing.
- `<fieldset>` + `<legend>` for grouped controls (radio sets).

## Common mistakes

- Error text that only says "Invalid input" with a red border (unnannounced, ambiguous).
- Modals without focus trap: Tab silently lands in the page behind.
- Closing on overlay click without also supporting Escape.
