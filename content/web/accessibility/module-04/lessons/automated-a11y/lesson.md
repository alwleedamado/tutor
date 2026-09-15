# Automated Accessibility Testing

Automation reads the **accessibility tree** — what assistive tech actually sees, derived from the DOM plus ARIA. Rule-based checkers (axe-core is the standard) interrogate that tree mechanically.

## What automation catches well

- Missing alt attributes, empty button names
- Missing form labels, broken `for`/`id` pairs
- Contrast below AA
- Invalid ARIA (unknown roles, missing required states)
- Duplicate ids, lang attributes, landmark structure

## What it cannot catch

- Whether focus order *makes sense*
- Whether alt text is *meaningful*
- Whether error messages are *helpful*
- Keyboard traps in custom widgets (partially detectable)

Rule of thumb: automation finds roughly a third to half of issues — the structural ones — cheaply and permanently.

## In Vitest

```ts
import { mount } from '@vue/test-utils'
import { axe } from 'vitest-axe'
import LoginForm from './LoginForm.vue'

it('has no axe violations', async () => {
  const wrapper = mount(LoginForm)
  expect(await axe(wrapper.element)).toHaveNoViolations()
})

it('announces errors', async () => {
  const w = mount(LoginForm)
  await w.find('form').trigger('submit')
  expect(w.find('[role="alert"]').exists()).toBe(true)
})
```

The second test is the important kind: assert the *behavior* (announced errors), not just the absence of rule violations.

## CI as a gate

- Per-component axe assertions in the unit suite (fast, targeted).
- Full-page scans in E2E for critical routes.
- New violations fail the build; baselined legacy debt shrinks over time.

## Best practices

- Write behavioral a11y assertions (`role="alert"` exists, focus moved into the dialog) alongside axe.
- Update axe versions regularly; rules improve.

## Common mistakes

- Treating a green axe report as "accessible" — it means "no rule violations found".
- Disabling rules to go green (silencing the smoke alarm).
- Scanning only in dev, never in CI — regressions return next sprint.
