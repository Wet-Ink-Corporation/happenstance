---
id: kb-open-question-adr-0022-falsifiers-fired-001
title: Two of ADR-0022's falsifiers have fired, a third cannot fire as written, and nobody has re-opened
kind: open_question
status: accepted
authority_tier: note
summary: >-
  ADR-0022 wrote three conditions under which it would be re-opened. The 2026-09-03
  pre-publication review checked all three, two have fired and the third is unfireable as written,
  and no re-opening has happened. Section 11 said re-open if any run ever reports busy > 0; busy > 0
  was observed at the shipped CONTENDERS = 64, one launch in seven, the first nonzero busy count
  anywhere in this tree, and the shape matters — busy = 1 and busy = 2 with exhausted = 0, meaning
  attempts that entered the handler rather than attempts that ran the 5,000 ms budget out, so the
  margin held while the premise did not. Section 9 said re-open if a deployment shows the captured
  tokio Handle costing something the inline path does not; the capture is unconditional and
  irreversible, which makes SqliteEventStoreError::NoRuntime unreachable for any store outliving the
  runtime it was constructed on, so the variant ADR-0022 kept in order to preserve a real meaning has
  lost it by a different route than the one that section anticipated. Section 16's falsifier for
  section 8 cannot fire as written, because the shape it names as the re-open trigger — a GROUP BY
  with HAVING COUNT(DISTINCT tag) aggregate — is not what the adapter emits; measured, the chain
  that does ship loses to that aggregate in nine of nine two-tag cells, which is the finding the
  falsifier was written to catch and which its own wording excludes. ADR-0022 is accepted and
  immutable, so a fired falsifier cannot amend it: what is not decided is whether it is superseded,
  re-opened with a scoped amendment, or explicitly ratified as still correct with the firings
  recorded against it — and who takes that call. Forced by phase 12, after which the pragma set is a
  documented property of a published adapter.
depends_on: []
related:
  - kb-decision-0022
  - kb-reference-busy-timeout-margin-001
  - kb-reference-append-condition-experiment-001
  - kb-open-question-es-17-two-adapter-measurement-001
  - kb-open-question-testkit-contention-tolerance-001
  - kb-governance-what-may-refute-a-finding-001
source_paths:
  - .kb/_intake/2026-09-03-pre-publication-review.md
  - references/evaluation/review-pre-publication-2026-09-03.md
  - experiments/busy-timeout-margin/
  - references/adr/0022-append-condition-strategy.md
  - crates/happenstance-sqlite/src/event_store.rs
last_reviewed: 2026-09-04
---

# Two of ADR-0022's falsifiers have fired, a third cannot fire as written, and nobody has re-opened

## What is true today

ADR-0022 (`kb-decision-0022`) closes with a section (`references/adr/0022-append-condition-strategy.md:589`)
naming, for each of its verdicts, the condition under which a later reader should treat it as
overturned rather than affirmed — the discipline this repository already applies to ES-17
(`kb-open-question-es-17-two-adapter-measurement-001`). The 2026-09-03 pre-publication review
checked every one of them against the two experiments it ran, `busy-timeout-margin` and
`shipped-append-condition-sql`, and found three results, not zero.

**Section 11 fired.** The pragma section's falsifier reads: *"Re-open the busy timeout if any run
ever reports `busy > 0`, which would mean five seconds stopped being generous"* (`0022:614-616`).
At the shipped `CONTENDERS = 64`, launched seven times under SQLite's own busy handler, one launch
produced `busy > 0` — the first nonzero busy count anywhere in this tree. The shape of the firing
matters and was checked against the raw counters rather than a summary that overstated it: two rows
read `busy=1 … exhausted=0` and `busy=2 … exhausted=0` — three attempts that *entered* the busy
handler, and `exhausted=0` on both, meaning no attempt ran the full 5,000 ms budget out. The margin
claim (1.3x–1.4x on the plateau) is unaffected; what broke is the premise the falsifier was testing,
not the number it was protecting.

**Section 9 fired, by a different route than it anticipated.** Its falsifier: *"Re-open it if a
deployment shows the captured `Handle` costing something the inline path does not"* (`0022:609-610`).
The captured `Handle` is unconditional and irreversible — no constructor, setter or builder can
clear or replace it — so a store built inside one Tokio runtime and served from another gets
`append` and `head` calls that run inline and succeed, and every `read` call that hangs or yields
one cancelled-task item and terminates. `SqliteEventStoreError::NoRuntime`, the variant ADR-0022
kept specifically because this design choice preserved its meaning, is unreachable for such a store.
The cost is real, as the falsifier anticipated; it is a correctness gap rather than the performance
delta the falsifier's wording pictured.

**Section 16's own falsifier for section 8 cannot fire as written.** It reads: *"Re-open it if a
future SQLite pushes predicates through an aggregate"* (`0022:606-607`) — naming a future compiler
change as the trigger. But the aggregate in question, `GROUP BY position HAVING COUNT(DISTINCT tag)
= n`, is not a future capability; it is the form every one of ADR-0022's own published figures was
measured against, and the adapter that ships emits a different shape, a correlated intersection
chain, instead. Measured against the aggregate for the first time, the shipped chain loses in nine
of nine two-tag cells, at 1.54x–1.86x. The condition the falsifier was written to detect has already
happened, on the code that shipped — its wording just points at the wrong mechanism to notice it.

## What is not decided

ADR-0022 is `accepted` and immutable under KB authority rule 1: a fired falsifier is evidence, not
license to edit the decision's body, and recording "falsifier fired" as a status line inside it
would be exactly that edit, made worse — a later reader could no longer tell whether the decision
was signed with that knowledge or acquired it afterward. What is not decided is which of three
moves resolves it: supersede ADR-0022 with a new decision atom carrying `supersedes: [kb-decision-0022]`;
re-open it with a scoped amendment addressing only the pragma and runtime-seam sections; or ratify
it explicitly as still correct, with the firings recorded from outside as accepted costs — the third
is defensible, since section 11 fired with a broken premise rather than a broken margin (`busy > 0`
with `exhausted = 0` throughout). Nobody has taken that call, and the routing question — who owns the
testkit-facing contention-tolerance question section 11's firing raises, versus who owns section 9's
runtime-seam correction — is likewise open.

## What forces it

Phase 12, when the pragma set (`synchronous`, the busy timeout, the runtime-seam choice) becomes a
documented property of a published adapter rather than an internal decision this workspace can
revisit freely. Superseding, re-opening or ratifying ADR-0022 before then is materially cheaper than
after: a downstream crate that has pinned against the current behavior is not yet a stakeholder.
