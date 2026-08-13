---
item: "HS-S0100"
stage: implement
created: "2026-08-12"
updated: "2026-08-12"
---

# Acceptance ledger — Both phase-13 open questions resolved in place, not deleted

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

This story compiles nothing — it maps to no workspace package — so every `verifying_test` below is a
gate command or a diff-shape check rather than a Rust test. That is the case `.redkiln/config.yaml:36-40`
names explicitly: a purely package-shaped gate "would compile nothing, read nothing, and call it
green". Cite the command **and** the `file:line` you read in its output; a green run alone is not
evidence for a specific criterion (`_decomposition.md`, *Merge-gate commands*).

```yaml
- id: AC-001
  criterion: "GIVEN a maintainer who needs to know whether the sync message set is still an open design question, WHEN they follow the `sync-message-set-and-format-version` bullet from the index into the atom after this PR, THEN the atom's frontmatter carries the disposition ADR-0027 actually forced against its three ordered sub-questions (.kb/open-questions/sync-message-set-and-format-version.md:83-91) — `status: superseded` plus an outbound `superseded_by` and `related` naming the answering decision atom where all three were reached, or `status` left as-is plus a `related` link and a named residue where they were not — and no status value outside the documented enum is invented."
  satisfied: false
  evidence: ""
  mount_point: ".kb/maps/open-questions-index.md"
  verifying_test: "redkiln validate --kb; git diff -- .kb/open-questions/sync-message-set-and-format-version.md (frontmatter-only hunks)"

- id: AC-002
  criterion: "GIVEN the same maintainer asking whether happenstance's wire format now has to interoperate with anything, WHEN they open `dcb-reference-publishes-no-wire-format.md` after this PR, THEN the atom is still open — a renewal, not a settlement — carrying an outbound `related` link to ADR-0026 and an annotation naming the experiment the renewal is measured against (RUNBOOK.md:4593-4595), and its sub-question 3 (whether re-checking the W7 finding is phase 13's standing task, :97-100) is visibly settled whichever way ADR-0026 settled it."
  satisfied: false
  evidence: ""
  mount_point: ".kb/maps/open-questions-index.md"
  verifying_test: "redkiln validate --kb; git diff -- .kb/open-questions/dcb-reference-publishes-no-wire-format.md (frontmatter-only hunks, `status` unchanged)"

- id: AC-003
  criterion: "GIVEN a maintainer who wants to know what was not known on the day the deferral was taken, WHEN they read either atom's prose after this PR, THEN it is byte-identical to its pre-PR body below the closing `---` — no \"How it was resolved\" section appended, no sentence reworded into its own answer, neither file deleted, renamed or moved — because the value of the record is that it shows the state of knowledge at the time (.kb/open-questions/README.md:40-45; .kb/governance/rewrite-the-referent-never-the-reasoning.md:52-56)."
  satisfied: false
  evidence: ""
  mount_point: ".kb/open-questions/ (both atoms, reached from .kb/maps/open-questions-index.md)"
  verifying_test: "git diff -- .kb/open-questions/ inspected hunk by hunk; git diff --name-status -- .kb/open-questions/ shows exactly two M and no D/R"

- id: AC-004
  criterion: "GIVEN a maintainer who enters only from the index and never opens an atom, WHEN they read the two bullets at .kb/maps/open-questions-index.md:122-134 after this PR, THEN neither tells them a question phase 13 has acted on is untouched: each bullet keeps its position under its existing `##` section, its file link and its `kb-open-question-…` id, and changes only its leading status word and its one sentence — in the index's own *Adding an entry* form, status word first, then id, then one sentence, with a dated `Amended <date>:` clause where the question stayed open, copying the existing precedent at :56-61 rather than inventing a house form."
  satisfied: false
  evidence: ""
  mount_point: ".kb/maps/open-questions-index.md"
  verifying_test: "rg -n \"kb-open-question-sync-message-set-undesigned-001|kb-open-question-dcb-no-published-format-001\" .kb/maps/open-questions-index.md; git diff -- .kb/maps/open-questions-index.md (zero bullets removed/added, zero new ## headings)"

- id: AC-005
  criterion: "GIVEN the repository's rule that nothing under `.kb/` is hand-authored, WHEN this story's changes land, THEN both dispositions arrived through `.kb/_intake/` documents consumed by `/redkiln:kb-ingest` — the wave's record exists under `.kb/_governance/integration-waves/`, `.kb/_intake/` is back to holding only its own README at exit, and the diff touches no file under `.kb/decisions/**` (links are outbound only) and none under `spec/SPECIFICATION.md` (clause work is `clause-arithmetic-and-deferral-renewals`')."
  satisfied: false
  evidence: ""
  mount_point: ".kb/_intake/ → /redkiln:kb-ingest → .kb/_governance/integration-waves/"
  verifying_test: "git diff --name-only read against the spec's PR-boundary block (empty set outside it); ls .kb/_intake/ (no staged document); the wave record path"

- id: AC-006
  criterion: "GIVEN a maintainer who trusts the KB because the gate checks it, WHEN the merge gate runs on this PR, THEN `redkiln validate --kb && redkiln doctor` is green — no dangling `superseded_by`, no accepted decision atom mutated, exactly the six expected `template-drift` advisories and no new finding — the unconditional `cargo xtask lints && cargo xtask spec-trace` stays green even though this story maps to no package, and `_ledger.md` says which of those commands produced the evidence for each AC rather than resting on one green run."
  satisfied: false
  evidence: ""
  mount_point: ".kb/ (validate --kb over the whole corpus) + .redkiln/config.yaml:48 reachability_static"
  verifying_test: "redkiln validate --kb && redkiln doctor; cargo xtask lints && cargo xtask spec-trace; cargo xtask affected --base main; redkiln verify --grain story"
```
