---
id: kb-reference-macros-ceremony-second-example-001
title: The ceremony ratio re-measured on a second worked example is 6.3 per cent, and strengthens ADR-0033's verdict
kind: reference
status: accepted
authority_tier: note
summary: >-
  At lane/rendered-pages b87b17b, the impl DomainEvent share of each worked example's line count:
  course-subscriptions 39/511 (7.6 per cent), transfers-on-sqlite 35/663 (5.3 per cent), combined
  74/1174 (6.3 per cent) — below ADR-0033's own 78a2170 figure of 7.5 per cent. Only the
  measurement's cheapest figure was re-taken; the original 29-range hand classification behind
  kb-reference-macros-ceremony-measurement-001 was not re-run.
depends_on: []
related:
  - kb-reference-macros-ceremony-measurement-001
  - kb-decision-0033
  - kb-open-question-event-type-positional-mapping-001
source_paths:
  - .kb/_intake/remediation-2026-09-04-briefs/adr-0033-reopen-ground.md
last_reviewed: 2026-09-07
---

# The ceremony ratio holds harder on a second worked example

## What this is a pointer to

`kb-reference-macros-ceremony-measurement-001` is ADR-0033's evidence base: a 29-range hand
classification of `examples/course-subscriptions/src/main.rs` at `78a2170`, taken 2026-08-16,
before the workspace had a second worked example against a real store. This atom is a re-taking
of that measurement's *cheapest figure* — `impl DomainEvent`'s line count as a share of the file
— against both worked examples as they stand at `lane/rendered-pages` `b87b17b`, taken
2026-09-04. It is not a re-run of the 29-range classification, which this atom's source brief
declines to re-adjudicate.

## The numbers

| Substrate | `impl DomainEvent` | file | share |
|---|---:|---:|---:|
| `course-subscriptions/src/main.rs` @ `78a2170` (the original atom) | 40 (`:209-247`) | 532 | 7.5% |
| `course-subscriptions/src/main.rs` @ `b87b17b` | 39 (`:188-226`) | 511 | 7.6% |
| `transfers-on-sqlite/src/main.rs` @ `b87b17b` | **35** (`:281-315`) | 663 | **5.3%** |
| both worked examples, combined | 74 | 1,174 | **6.3%** |

## What it shows

**The second worked example pulls the ratio down.** 35 lines of 663 against 40 of 532: both
examples carry three event variants, the `impl DomainEvent` block is a near-fixed cost, and the
domain code around it grew on the newer example while the mapping block did not. That is the
structural explanation the original atom already gave, now confirmed against a substrate it never
saw — a strengthening of ADR-0033's verdict that macro-generated mapping ceremony is not worth
building for 0.1, not a weakening of it.

**The original atom's own substrate has moved, correctly left unedited.** `course-subscriptions/src/main.rs`
is 21 lines shorter today than at `78a2170` and its `impl DomainEvent` block now sits at
`:188-226` rather than `:209-247`. `kb-reference-macros-ceremony-measurement-001` stays pinned to
its own commit under the reference layer's dating rule — repairing a dated measurement in place is
how it stops being a measurement — so a reader re-deriving the ratio from that atom's line ranges
against `HEAD` gets a different file than the one it describes. This atom is the newer,
independently dated measurement that sits alongside it rather than replacing it.

## What this does not settle

This atom re-takes only the line-count share, the cheapest of the original measurement's figures.
Whether the full 29-range hand classification should be re-taken against either worked example is
not decided here — it is a judgement call about what a hand classification counts, not a
mechanical re-run, and this atom's source brief is explicit that it is not entitled to make that
call. It also says nothing about `Y-2`'s separate finding — that the hand-written `event_type`
mapping is spelled positionally with no compiler check, six lines that the volume measurement is
structurally unable to see because it is denominated in lines and the mapping hazard is not a
volume question. See `kb-open-question-event-type-positional-mapping-001`.

## Conditions

Taken 2026-09-04 against `lane/rendered-pages` at commit `b87b17b`, by `wc -l` and grep for each
`impl DomainEvent` block's line bounds. The line numbers cited are subject to the same drift this
atom records for the original measurement: they will go stale the next time either worked example
is edited.
