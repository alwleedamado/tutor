# Choosing Dependencies Like an Engineer

Every dependency is a long-term relationship: its bugs become your bugs, its CVEs become your incidents, its abandonment becomes your migration. Choose with a checklist, not a vibe.

## The evaluation checklist

```text
purpose       what exact problem? could the platform already solve it?
maintenance   last release, issue backlog trend, number of maintainers
security      known CVEs, install scripts, transitive dependency count
alternatives  two real competitors + the do-it-yourself option
ops           config burden, license, does it need infrastructure?
exit cost     how localized is removal if this goes bad?
```

## The platform keeps absorbing packages

Modern Node ships `fetch`, `structuredClone`, `crypto`, a test runner, `AbortController`. Before installing, ask: does Node do this now? (lodash → native array methods; node-fetch → fetch; uuid → `crypto.randomUUID`.)

## Worked examples

| Need | Decision | Why |
| --- | --- | --- |
| 11-line string helper | write it | trivial, exit cost zero |
| rate limiting with store backends | package | real distributed complexity, security-sensitive |
| deep-clone JSON data | `structuredClone` | built in |
| JWT with rotation semantics | package | security-critical, don't hand-roll |

## Supply-chain reality

`npm install` executes code (install scripts) and adds transitive trust. Typosquats (`reqeust`), hijacked maintainer accounts, and protest-ware are real incidents. Size matters: every dependency is attack surface.

## Best practices

- Add dependencies in a reviewed PR with the checklist filled in the description.
- Prefer focused, maintained packages over "does everything" belts.
- Review `npm ls` occasionally — prune what nothing imports.

## Common mistakes

- "It's popular" as the whole argument (popularity is one maintenance signal, not purpose).
- Installing a package to avoid learning a 15-line standard API.
- No lockfile discipline — unreviewed version drift is unreviewed code.
