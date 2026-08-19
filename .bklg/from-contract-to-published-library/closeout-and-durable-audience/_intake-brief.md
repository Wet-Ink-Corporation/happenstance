---
item: HS-P0019
stage: intake
created: 2026-08-12T03:23:30.195Z
updated: 2026-08-12T03:23:30.195Z
template_sig: ab516678
rendered_sig: f75beaa0
---

# Intake Brief — The whole gate on the assembled library, and a durable audience

## Problem

Sixteen Definition-of-Done scenarios each seen once, in the project that produced
it, is not the same claim as sixteen scenarios observed **as a set on the assembled
library from a clean checkout** — and the charter's DoD preamble asks for the
latter. Separately, `.kb/product/` is empty: no `authority_tier: product` atom
exists, so the next initiative re-derives an audience from scratch. The first
attempt at populating that layer by hand was reverted wholesale (`0269720`) for
producing the directory layout of the process without the process.

## Desired Outcome

The whole gate is green on the assembled tree from a clean checkout (DoD 13), and
DoD 1–12 and 14–15 have been re-observed together rather than remembered
separately. Personas and journeys exist as durable `.kb/product/` atoms with valid
frontmatter, promoted **through the ingest path**, carrying the qualification that
all four rest on secondary evidence and none was directly observed (DoD 16, AC-15,
BR-16). Every answer this initiative settled landed as a decision atom naming the
alternatives that lost, and every consumed open-question atom is **resolved, not
deleted** (BR-15). `redkiln validate --kb` and `redkiln doctor` are clean, with
exactly the six expected `template-drift` advisories and no seventh.

## Constraints

- **Depends on** `retention-and-incomplete-logs`; last in merge order.
- **This is the terminal project** — the one wired to `verify.e2e: cargo xtask ci`,
  the whole gate, which `.redkiln/config.yaml`'s own comment calls this
  repository's Definition of Done. Every other project runs the project-scoped bar.
- **Never hand-author `.kb/` atoms.** Promotion runs through
  `/redkiln:kb-ingest` and closeout. This is the constraint the reverted commit
  exists to remember.
- **Never run `redkiln adopt --templates`.** It would overwrite six deliberate
  customisations and then fail the `backlog` CI job on the absence it created.
- **This project designs nothing.** It carries one brief (`testing`) deliberately;
  forcing `architecture` or `ux` here would produce exactly the stub the
  right-sizing rule exists to prevent.
- **Non-goals:** any code, adapter or release — every sibling project above.

## Open Questions

- **Is the evaluator its own persona**, or an earlier stage of the application
  author's journey? Carried unresolved from the charter. It changes what this
  project promotes, and whether DT-1's *two entry points, one per audience* option
  is even coherent. **Decide it in this project's testing brief, not at
  atom-authoring time.**
- **DoD 13's honest caveat.** It asks for the gate green *"on the exact tree that
  was published"*, and `replication-identity-and-ingest` and
  `retention-and-incomplete-logs` land after publication — so the closeout tree is
  not byte-identical to the published one. This is fine given the gate's constraint
  that retention must not change the published surface, but the closeout report
  must state the difference rather than let the phrase stand unqualified.
- Whether the six `template-drift` advisories are still exactly six at closeout, and
  what to do if a seventh appeared during the initiative — a template someone
  changed without deciding to.
- How the qualification *"all four personas rest on secondary evidence; none was
  directly observed"* is carried in atom frontmatter rather than only in prose,
  so a future reader cannot miss it.

## Proof artefact

**`cargo xtask ci` green on the assembled tree from a clean checkout**, together
with the re-observation record for DoD 1–12 and 14–15 as a set. This would not
exist if the design were wrong: each project's own gate ran against a tree
containing only its work and whatever preceded it, so a genuine interaction between
two independently-green projects is invisible until the whole thing is assembled
and run once. A clean checkout additionally catches everything a warm working tree
hides — uncommitted files, stale build artefacts, a path dependency that should
have been a version.

Secondary and equally required: the `.kb/product/` persona and journey atoms
passing `redkiln validate --kb`, which is the only thing that distinguishes a
durable audience from a paragraph in a closed initiative.

## Clauses

- **None discharged or amended.** This project verifies and records; it decides
  nothing about the specification.
- It **audits** that the clause work of every sibling landed: that
  `publication-and-positioning`'s ledger audit reconciles against
  `spec/SPECIFICATION.md`'s own stated figure, and that no clause a consumer can
  rely on is provisional with an empty falsifier.
- `cargo xtask spec-trace` runs as part of the gate, so the specification's
  cross-references are checked on the assembled tree rather than trusted.

## Gate: Intake

Tick each box once the sections above satisfy it. `redkiln advance` reads these
against `.redkiln/templates/gates/intake.md` — where each box's rationale is
written — and will not leave `intake` until every one is ticked. Keep each box on
one line; the parser matches line by line.

- [x] The problem and desired outcome are stated.
- [x] Constraints and non-goals are recorded.
- [x] Open questions are captured for distillation.
- [x] The proof artefact is named, and it would not exist if the design were wrong.
- [x] For a port freeze: the axis it is most likely to be wrong about is named, and something in the workspace sits at the other end of it.
- [x] The clauses this work discharges or amends are listed by id.
- [x] Where this brief and `SPECIFICATION.md` disagree, the specification wins.
