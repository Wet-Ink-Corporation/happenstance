---
id: kb-open-question-cf-17-cf-14-markers-001
title: A declaration MUST landed on CF-17's text; its marker and CF-14's did not move with it
kind: open_question
status: accepted
authority_tier: note
summary: >-
  CF-17 is PROVISIONAL and CF-14 is DEFERRED. A lane closing the audit's L1-2 finding added a
  MUST to CF-17's text — a fixture declaring REOPEN supported MUST discard state over a medium
  outside the process's hold and MUST state the mechanism, or decline the capability with that as
  its stated reason — and landed NoopReopenFixture and a named test,
  reopen_over_claiming_is_undetectable_and_this_is_the_record, as the wrong implementation the
  declaration is written against. CLAUDE.md and CF-17's own bracket are explicit that moving a
  maturity marker is an ADR's act, and adding a MUST to a PROVISIONAL clause's text is not the
  same act as moving it, so neither marker changed. What is open is whether the new declaration
  obligation is itself sufficient grounds to lift CF-17, and whether CF-14's DEFERRED marker
  should be reconsidered now that the rejected shape it names has a documented wrong
  implementation to test against. Both adapters CF-14's deferral names as unanswered — HS-P0013
  and HS-P0014 — still have not answered, and both are the shape where an empty reopen is most
  tempting: a Durable Object's storage surviving an eviction, and a one-shot HTTP client with no
  connection to close at all.
depends_on: []
related:
  - kb-decision-0012
  - kb-open-question-provisional-falsifiers-001
  - kb-open-question-cf-40-ownership-001
  - kb-concept-mutation-kind-tethered-bar-001
  - kb-open-question-dagger-convention-vs-maturity-markers-001
  - kb-open-question-model-only-kind-memberless-001
source_paths:
  - .kb/_intake/remediation-2026-09-04-briefs/stated-only-defects-and-the-reopen-must.md
  - spec/SPECIFICATION.md
last_reviewed: 2026-09-07
---

# A declaration MUST landed on CF-17's text; its marker and CF-14's did not move with it

## What is true today

`spec/SPECIFICATION.md:8108-8129` (CF-17, `[PROVISIONAL]`) now reads: a fixture declaring
`REOPEN` supported **MUST** make `reopen` discard process-level state over a medium that
outlives the process's hold on it, **MUST** state the mechanism, and a fixture over a store with
no such medium **MUST** decline the capability with that as its stated reason. The clause's
`Rejects:` paragraph names `NoopReopenFixture` (`crates/happenstance-testkit/tests/`) as what the
MUST is written against — a fixture that declares `REOPEN` supported and overrides `reopen` with
an empty body over a volatile store, never reaching the trait's panic. It is explicitly **not** a
registered `Kind::Mutant`, because no rule of the event-store family can reject it: arming a
mid-batch fault has a port-observable consequence (the append must answer `Err`, CF-39's
concern), and reopening has none. `reopen_over_claiming_is_undetectable_and_this_is_the_record`
in `tests/mutation_coverage.rs` carries the hazard instead — it drives the fixture through every
rule, asserts it fails none, and asserts it converts three durability rules from reported skips
into passes while an honest twin one line apart reports them as skips.

CF-14 (`spec/SPECIFICATION.md:7992-8024`, `[DEFERRED]`) is unchanged by this work. Its deferral
was already about a different axis — whether a `reopen` capability can be honoured by rusqlite, a
Durable Object and a one-shot HTTP client with one shape, or whether "durable" needs grading —
and one of the three (`happenstance-sqlite`) has already answered with no grading needed, per the
clause's own text. What remains deferred is the two that have not: `happenstance-cloudflare`
(HS-P0013) and `happenstance-neon` (HS-P0014).

## What is not decided

Whether the declaration obligation added to CF-17's text is itself sufficient evidence to lift
the marker from `[PROVISIONAL]` to `[FROZEN]`. The marker's own falsifier — "a legitimate adapter
that is durable and cannot express even a reopen through this contract" — is about the
**capability's shape**, and the new MUST is about **declaring which side of it a fixture is on**;
whether closing the declaration hole moves the needle on the shape question is a judgement the
brief that landed the MUST declined to make, on the ground that CLAUDE.md and the clause's own
bracket reserve marker moves for a decision record.

Separately, whether CF-14's `[DEFERRED]` status should be revisited now that the rejected case
its own clause once said "nothing in the workspace could fail such a store" against — an adapter
acknowledging before durability is guaranteed — sits beside a family (CF-17's `REOPEN` rules)
that has grown a named, driven wrong implementation of the adjacent hazard. Whether that changes
anything about CF-14's own experiment-shaped deferral, which is about grading "durable" across
three unlike adapters rather than about a missing wrong implementation, is unsettled and belongs
to whoever owns both markers.

## What forces it

The two adapters CF-14's deferral names as unanswered reaching a state where their `reopen` (or
their declination of `REOPEN`) can be observed: `happenstance-cloudflare`'s Durable Object
storage surviving an isolate eviction, and `happenstance-neon`'s connectionless HTTP shape, which
has no live handle to invalidate at all and may simply decline the capability. Either answering
is also the event that would let CF-17's falsifier actually discriminate again, since today's only
answer (`happenstance-sqlite`) is the adapter the marker's own text says did not force the split.
