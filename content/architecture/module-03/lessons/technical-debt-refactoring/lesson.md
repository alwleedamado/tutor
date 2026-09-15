# Technical Debt & Refactoring Strategy

Debt is a metaphor — manage it like financial debt: know the principal, the interest rate, and the repayment plan.

## The debt quadrant

| | Deliberate | Inadvertent |
| --- | --- | --- |
| **Prudent** | "We ship without tests for the demo; ticket filed" | "We didn't know better — now we do" |
| **Reckless** | "No time for design" (every sprint) | "What is layering?" |

Prudent-deliberate debt is a strategy. The rest accrues interest in every change.

## Detecting erosion

- The same files collide in every PR (change amplification).
- New features start by *working around* the architecture instead of *with* it.
- Imports point backwards; the "temporary" exception from last quarter is load-bearing.
- Onboarding time keeps climbing while docs stay flat.

## Refactoring under feature pressure

You will never get a debt-free sprint. Refactor **alongside** features using seams:

1. Find the seam — a boundary where behavior can change without editing callers (a trait, a composable, a module API).
2. **Parallel change** (expand → migrate → contract): add the new shape, move callers one by one, delete the old shape.
3. Every step keeps tests green; commits stay small and reviewable.

```text
expand:   add formatPriceV2() beside V1
migrate:  switch call sites one PR at a time
contract: delete V1 once the last caller is gone
```

## Best practices

- Track debt where work is planned (tickets/ADRs), not in folklore.
- Pay interest deliberately: touch a module → leave it more cohesive than you found it.
- Measure refactors by the next feature's cost, not by abstract "cleanliness".

## Common mistakes

- Big-bang rewrites that pause features for months and rarely ship.
- Refactoring without tests: you cannot keep behavior if you cannot observe it.
- "Temporary" hacks without a repayment ticket — that is how interest compounds silently.
