# Detecting Architecture Erosion

Architecture doesn't fail loudly — it erodes: one pragmatic import, one "temporary" bypass, repeated for a year. Detection is a skill you practice in review and with tooling.

## The audit checklist

Hunt for these in any Vue repo:

```text
□ shared/ imports features            (upward dependency — violation)
□ deep imports into feature internals (bypassed public APIs)
□ components importing api clients    (boundary bypass)
□ one store imported by 80% of files  (god store)
□ two features importing each other   (cycle)
□ "utils" files with unrelated logic  (zero cohesion)
```

## Reading the import graph

```bash
npx dependency-cruiser src --output-type dot   # visualize arrows
npx madge --circular src/                      # find cycles mechanically
```

Tools codify what the team agreed: *rules that used to live in someone's head become CI failures.*

## Human signals (often earlier than tool signals)

- The same three files collide in every PR.
- New features start by working *around* a module instead of *with* it.
- Nobody can explain what a module is for without reading all of it.
- The phrase "don't touch that file" is load-bearing.

## Best practices

- One review question catches most erosion early: **"which dependency arrows did this PR add?"**
- Fix violations at landing time; erosion compounds — a violation merged today is tomorrow's architecture.
- Re-run the audit quarterly; track the trend, not perfection.

## Common mistakes

- Tooling with zero agreed rules: reports nobody reads.
- Chasing 100% graph purity while features stall — boundaries serve delivery.
- Treating "it works" as evidence there's no erosion; erosion is about the *next* change.
