---
item: HS-P0018
stage: intake
created: 2026-08-12T03:23:27.141Z
updated: 2026-08-12T03:23:27.141Z
template_sig: ab516678
rendered_sig: 51b811c0
---

# Intake Brief — What a store may forget, and how a reader finds out

## Problem

The completeness axis has nothing at either end. Every store in this workspace
holds its whole log, so nothing has ever asked what happens when one does not — and
a reader that quietly builds a wrong answer from a truncated log is the failure
mode nothing currently detects. It fails silently, which is the worst available
property for a system whose entire premise is that the log is the truth. The
specification is correspondingly quiet: ES-39, CF-27 and SY-32 are all deferred.

## Desired Outcome

What a store may forget, and how a reader finds out, has a written answer on file
under the same bar as replication: a decision atom or an explicit reasoned refusal
(DoD 15, AC-14). We know it worked when a store holding only a **suffix** of its own
log exists as an instrument, a reader is run against it, and the reader **fails
loudly** — or the refusal to define this is recorded as a decision with its reasons.
The corresponding open-question atom is resolved rather than deleted, and
`redkiln validate --kb` is clean.

## Constraints

- **Depends on** `replication-identity-and-ingest` (it needs SY-32); blocks
  `closeout-and-durable-audience`.
- **The answer is constrained to what requires no published-surface change.**
  Decided at the decomposition gate. ES-39 and ES-40 are `EventStore` clauses and
  this project lands *after* `0.2.0` is published; under 0.x a change to a published
  port is a minor bump, meaning a `0.3.0` this initiative's exit criteria do not
  contemplate. `RUNBOOK.md` already states that an explicit written refusal is a
  legitimate answer, so the constraint costs nothing the charter wanted. **If the
  work concludes that the honest answer requires a surface change, that is a finding
  to escalate — not a licence to make the change.** Rejected alternatives: accepting
  a `0.3.0` outside the exit criteria, and pulling this project ahead of publication
  (the largest re-wire of the DAG).
- **An explicit written refusal is a legitimate outcome** — *"deletion is out of
  scope for `EventStore`, and here is what a deleted-from store looks like to a
  reader"* — and must be recorded as one, not treated as a failure to decide.
- **Non-goals**, each naming its owner: replication identity and the merge rule →
  `replication-identity-and-ingest`; crypto-shredding, lawful-deletion tooling or a
  retention policy engine → not in the charter's scope at any grain; amending a
  published `[FROZEN]` clause → a new decision atom and a re-plan.

## Open Questions

- **ADR-0028** — what a store is permitted to forget, and how it says so (ES-39,
  CF-27, SY-32).
- **DT-7 — one undifferentiated signal, or a distinction between transient,
  benign-permanent and meaningful-permanent?** A shipped peer framework found one
  undifferentiated signal insufficient in production; distinguishing costs the
  reader complexity they may not want.
- **Where does the suffix-store instrument live?** It is an instrument first and a
  target never — it exists to make a reader fail — so it must not accidentally
  become a published adapter.
- Whether "fails loudly" is expressible in the existing error vocabulary, or needs
  one it does not have. **This is the question most likely to collide with the
  no-surface-change constraint**, and it should be answered early rather than
  discovered late.
- Whether CF-27's deferral survives contact with a Durable Object that can evict.

## Proof artefact

**A store holding only a suffix of its own log, and a reader that fails loudly
against it.** This would not exist if the design were wrong: every existing store
holds its whole log, so a contract that had quietly assumed completeness passes
every test in the workspace and produces a confidently wrong answer the first time
it meets a truncated one. The instrument only earns its keep by making something
fail. If the outcome is a reasoned refusal instead, the artefact is the written
decision atom together with the suffix store as the *illustration* of what a reader
is on its own against — the refusal still has to show the reader what it is
refusing to protect them from.

## Clauses

- **ES-39** `[DEFERRED]` — answered by **ADR-0028**, within the no-surface-change
  constraint.
- **ES-40** — its far end exercised by the suffix store.
- **CF-27** `[DEFERRED]` — re-read against a runtime that can evict.
- **SY-32** `[DEFERRED]` — inherited answered from
  `replication-identity-and-ingest`; consumed here, not re-decided.
- Nothing `[FROZEN]` is amended, and nothing published at `0.2.0` changes shape. A
  conclusion that it must is a finding to escalate.

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
