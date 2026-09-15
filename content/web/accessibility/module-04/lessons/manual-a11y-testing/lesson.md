# Keyboard Walkthroughs & Screen Readers

Automation catches rule violations; humans catch *usability*. Two manual techniques cover most of it.

## The keyboard walkthrough (10 minutes, no tools)

Fix a route, then:

1. Tab from the address bar — every control reachable? Order sensible?
2. Focus visible at every stop?
3. Enter/Space activate? Arrows work in lists/tabs/dropdowns?
4. Escape closes overlays?
5. Any trap that isn't a modal?
6. Can you complete the core task (login, checkout) keyboard-only?

Write findings as tickets with the step number — reproducible, fixable, retestable.

## Screen reader smoke test

NVDA (Windows, free) or VoiceOver (macOS, built-in):

- **Headings list** (NVDA: Insert+F7-ish navigation): does the outline tell the page's story?
- **Forms mode**: does each field announce its label before the value?
- **Buttons**: do they announce real names ("Submit order") or junk ("button graphic")?
- **Live updates**: is "Saved" or "3 results" actually spoken?

You are not simulating blindness — you are listening for structural nonsense, which is obvious immediately.

## Contrast & focus visibility

- Body text ≥ 4.5:1 (AA); large text ≥ 3:1. DevTools has a contrast checker per color.
- Check focus styles in dark mode — the classic place focus becomes invisible.
- Check disabled-state text too; "light gray on white" is a common 2.9:1 failure.

## Best practices

- Manual audit after every new widget pattern (a dialog, a data grid), not per page.
- Pair up: one drives keyboard-only, one writes tickets.

## Common mistakes

- Auditing only the homepage — flows are where focus traps live.
- Treating the screen reader as magic: learn three commands (next, click, headings list) and you can smoke test.
- Fixing contrast by darkening text without checking disabled/placeholder states.
