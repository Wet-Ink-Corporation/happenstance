---
id: kb-open-question-references-adr-correction-policy-001
title: CLAUDE.md never says whether a references/adr/ record may be corrected in place, and spec-trace gates line ranges into it
kind: open_question
status: accepted
authority_tier: note
summary: >-
  CLAUDE.md makes an accepted .kb/decisions/ atom immutable and describes references/adr/ as the
  full original record kept for citation, up to 1,508 lines carrying compiler transcripts, rejected
  alternatives and measurement tables a summary cannot hold - but it never states whether that
  second, longer record may itself be amended in place once a fact inside it goes stale. The gap is
  not hypothetical: references/adr/0012-append-shape-and-preconditions.md:174 states Event's tag
  type as Box<str> and prices a clone at a flat two allocations, and both are wrong today - ADR-0015
  changed the backing field to Cow<'static, str>, and the actual cost is measured at t+2 allocations
  for t tags, 66 at the specification's own 64-tag floor. The same staleness recurs in
  spec/SPECIFICATION.md's rationale prose for ES-17, which is the specification's own text rather
  than the ADR's, and is a separate correction with a separate owner. What forces the question to
  have a real mechanical consequence rather than being purely editorial is cargo xtask spec-trace,
  which cites references/adr/ by line range from spec/SPECIFICATION.md - so any in-place correction
  that shifts a line number is a citation break the gate would catch, and any correction technique
  chosen has to survive that check by construction. What is not decided is whether references/adr/
  is corrected the way rewrite-the-referent-never-the-reasoning treats a decision atom's identifier
  - fixed in place because the substance is unchanged - or left as dated evidence the way
  references/evaluation/ is treated elsewhere in this corpus, wrong-and-timestamped rather than
  corrected. Forced by the next citation into a references/adr/ record found to be factually stale.
depends_on: []
related:
  - kb-governance-referent-not-reasoning-001
  - kb-decision-0055
  - kb-open-question-es-17-two-adapter-measurement-001
source_paths:
  - .kb/_intake/remediation-2026-09-04-briefs/append-batch-ownership.md
  - CLAUDE.md
  - references/adr/0012-append-shape-and-preconditions.md
  - spec/SPECIFICATION.md
last_reviewed: 2026-09-07
---

# `CLAUDE.md` never says whether a `references/adr/` record may be corrected in place, and `spec-trace` gates line ranges into it

## What is true today

`CLAUDE.md` draws a clear line for the short-form atoms: "an **accepted
decision atom is immutable** — validation checks each one against `HEAD`, so
correcting one means writing a new atom that supersedes it, never editing the
body." It describes the long-form companion differently, and deliberately:
`references/adr/` "holds the full original records, up to 1,508 lines,
carrying the compiler transcripts, the rejected alternatives and the
measurement tables a summary cannot hold." Nowhere does it say whether *that*
record — the one CLAUDE.md itself calls a citable evidentiary record rather
than a decision atom — may be corrected in place once something inside it is
simply wrong about the world.

The gap surfaced in a remediation brief on `EventStore::append`'s ownership
question (`append-batch-ownership.md`), which found a concrete instance of it
while chasing an unrelated question. `references/adr/0012-append-shape-and-preconditions.md:172-174`
states:

```
- `Event`'s expensive fields are `Bytes` (`event.rs:185`, `:187`), which is
  refcounted, so `event.clone()` bumps a counter rather than copying a payload.
  What actually copies is one `Box<str>` for the type and one boxed tag slice.
```

Both factual claims are wrong today, verified against the working tree. The
type is wrong: `crates/happenstance-core/src/event.rs:47` and
`crates/happenstance-core/src/tag.rs:79` back `EventType` and `Tag` with
`Cow<'static, str>`, not `Box<str>` — changed by ADR-0015, which is itself
accepted (`references/adr/0015-validated-identifiers-and-store-limits.md:245`).
And the count is wrong: `Tags` is `Box<[Tag]>`, each owned `Tag` its own
allocation, so a clone costs `t + 2` allocations for `t` tags rather than a
flat two — measured at 66 allocations at the specification's own 64-tag floor
(VT-22), against `experiments/event-clone-allocations/results/raw/clone.txt`.
The same stale claim recurs, independently, inside
`spec/SPECIFICATION.md:3370-3373`'s own rationale prose for clause ES-17 —
that is the specification's text, not the ADR's, and is a separate correction
with a separate owner, but it shows the staleness is not confined to one file.

What makes this more than an editorial curiosity is `cargo xtask spec-trace`,
a CI gate step that cites `references/adr/` **by line range** from
`spec/SPECIFICATION.md`. A record in that tree is not free-floating prose —
other files point into it at specific lines, and the gate checks that those
citations resolve. Any technique for correcting a stale fact in place has to
survive that check by construction: an edit that adds or removes lines above
a cited range breaks the citation, and `spec-trace` is exactly the instrument
built to notice.

## What is not decided

Whether `references/adr/` records are corrected in place, on the model
`kb-governance-referent-not-reasoning-001` already applies one layer up — a
rename or a fix that leaves what the document *asserts* unchanged may be
rewritten, while reasoning inside a still-standing decision is never touched
— or whether they are instead treated the way `references/evaluation/` is
treated elsewhere in this corpus: dated evidence, wrong-and-timestamped rather
than corrected, because the record's value is showing what was believed true
on the day it was written. `references/adr/` is not quite either precedent
exactly: unlike `references/evaluation/`, its content is not itself a
point-in-time finding — it is the long-form justification for a decision that
is still in force, and a stale fact inside it can mislead a reader relying on
the record for the *current* state of the code, not for the history of the
decision. Unlike a decision atom's identifier, though, the content at stake
here is substantive (a type, a cost) rather than a name.

## What forces it

The next citation into a `references/adr/` record whose content is found to
be factually stale against the working tree — which this brief already
demonstrates happens, since it is the second such staleness identified in the
same investigation (the other being `spec/SPECIFICATION.md`'s own rationale
prose, tracked separately). Each time this recurs without a stated policy, the
next author re-derives the same choice — fix it, leave it dated, or ask —
independently.

## Ordered sub-questions

1. Does `references/adr/` get the `rewrite-the-referent-never-the-reasoning`
   treatment (correct facts that do not change what the record decided,
   leave the decision and its reasoning untouched), or the
   `references/evaluation/`-style treatment (leave it as dated, and let a
   reader consult `last_reviewed`-style provenance to know it may be stale)?
2. If in-place correction is the answer, does the correcting edit carry any
   marker distinguishing it from the original text — the way a decision atom
   change would require a whole new superseding atom — or is a plain edit
   sufficient because the record is evidence rather than a decision?
3. Who verifies, after any correction, that `cargo xtask spec-trace`'s cited
   line ranges into the corrected file still resolve — is that a manual step
   in the correction itself, or does the gate catch a break automatically on
   the next run?
