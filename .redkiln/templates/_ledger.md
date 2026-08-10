---
item: "{{item}}"
stage: implement
created: "{{created}}"
updated: "{{updated}}"
---

# Acceptance ledger — {{title}}

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

```yaml
- id: AC-001
  criterion: "" # the acceptance criterion, verbatim from spec.md
  satisfied: false
  evidence: "" # file:line and/or verifying test id — required once satisfied
  mount_point: "" # the real composition-root / render-path this AC is reachable through
  verifying_test: "" # the real-path test that proves it
```
