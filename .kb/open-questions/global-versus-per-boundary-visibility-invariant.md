---
id: kb-open-question-global-vs-boundary-visibility-001
title: Whether the visibility invariant needs to be global
kind: open_question
status: accepted
authority_tier: note
summary: >-
  ADR-0013 freezes the visibility invariant globally rather than per consistency boundary, and
  says openly that it closed the question by decision rather than by evidence. The argument is
  that a per-boundary invariant would keep AppendCondition sound and make the projection checkpoint
  unsound, because a checkpoint is a single position across all boundaries. The measured cost of
  the choice is real and small: arm B-tag, the per-boundary lock, ran at 0.935 of baseline at 64
  clients, so the global invariant is worth about nine percent of throughput at that load. What is
  not decided is whether the premise holds — it rests on the projection checkpoint being global,
  which phase 6 has not frozen. Refuted by a projection checkpoint design that turns out to be
  boundary-scoped, which would remove the argument the global choice rests on and make that nine
  percent real money. Owned by phase 6, the ProjectionStore freeze. If it fires, this section of
  ADR-0013 reopens and needs its own decision.
depends_on: []
related:
  - kb-decision-0013
  - kb-decision-0018
  - kb-reference-position-visibility-experiment-001
  - kb-open-question-projection-batch-no-apply-001
source_paths:
  - .kb/_intake/0013-position-assignment-and-visibility.md
  - references/adr/0013-position-assignment-and-visibility.md
  - experiments/position-visibility/
  - spec/SPECIFICATION.md
last_reviewed: 2026-08-13
---

# Whether the visibility invariant needs to be global

## What is true today

ADR-0013 (`.kb/decisions/0013-position-assignment-and-visibility.md`) lifts ES-10 from `[PROVISIONAL]`
to `[FROZEN]`, and in doing so states the visibility invariant as a global property: "Once any
reader has observed an event at position *P*, no subsequent read against that store may yield an
event at a position ≤ *P* that was not already visible." Decision §3, "Caveat two," names the
alternative directly and says the ADR is choosing rather than discovering: "this ADR keeps the
invariant **global**, and says so out loud." The experiment behind the lift measured a
per-boundary alternative — arm B-tag, a transaction-scoped advisory lock keyed by the consistency
boundary's tag — and found it "nearly free": a throughput ratio of 0.935 against arm C's 1.026 at
64 clients, and on disjoint tags it reproduces the baseline inversion byte-for-byte, meaning it
genuinely buys only a per-boundary guarantee, not a global one.

The ADR's own words on why it chose global anyway: "The experiment measured that the two are not
the same property and deliberately refused to settle which one happenstance needs... because DCB
evaluates an `AppendCondition` against a boundary and ES-10 may therefore be stronger than its
consumers require." ADR-0013 supplies the argument the experiment didn't have: "A per-boundary
invariant would make `AppendCondition` sound and the projection checkpoint unsound, and the
checkpoint is the harder consumer." The mechanism: `head()` is deliberately not query-scoped
(ES-30), so a projection runner resumes from one global position that covers boundaries it never
reads. Under a per-boundary invariant, a runner could observe position 100 on boundary X,
checkpoint, and later have 99 become visible on boundary Y — `from: checkpoint.next()` then skips
99 permanently, with no error anywhere.

The ADR states its own honesty about the closure explicitly, in the "What this ADR leaves open"
table: "Closed *by decision* in decision §3, not by evidence, and the decision is reversible."

## What is not decided

Whether the premise — that the projection checkpoint is necessarily global — actually holds. It
rests entirely on how `ProjectionStore`'s checkpoint is shaped, and that port is not yet frozen;
phase 6 owns it. If a future checkpoint design turns out to be boundary-scoped rather than global
(tracking one position per boundary rather than one position across all of them), the argument
ADR-0013 makes for keeping the invariant global evaporates, and the roughly nine percent of
throughput at 64 clients that arm B-tag would have bought becomes a live trade rather than a
foreclosed one.

## What forces it

Phase 6, the `ProjectionStore` freeze. ADR-0013 names this explicitly as the owner and the
trigger: "**Refuted by:** a projection checkpoint that is boundary-scoped rather than global, which
would remove the argument decision §3 rests on and make arm B-tag's 9% real money. **Owner: phase
6**, which freezes `ProjectionStore` and with it the checkpoint's shape. If that shape changes,
this section reopens and needs its own ADR." Because this is a `[FROZEN]` clause (ES-10) resting on
a decision rather than on evidence, reopening it cannot happen by editing ADR-0013 — it requires a
new ADR, per this repository's standing rule that a frozen clause changes by ADR and not by edit.

## Ordered sub-questions

1. When phase 6 designs the projection checkpoint, is a global-position checkpoint actually the
   right shape on its own merits, independent of this question — or is it itself being chosen
   partly because ES-10 is already frozen globally, which would make the two decisions
   circularly reinforcing rather than independently justified?
2. If a boundary-scoped checkpoint is seriously considered, does its design get evaluated against
   arm B-tag's 9%-at-64-clients cost data before or after the checkpoint shape is otherwise
   settled?
3. Does reopening ES-10 under a boundary-scoped checkpoint imply reopening `AppendCondition`'s
   boundary semantics too, given the ADR's own statement that a per-boundary invariant would make
   `AppendCondition` sound while a global one does not need to worry about it either way?
4. Is there a hybrid worth naming before phase 6 — a checkpoint that is per-boundary but
   accompanied by a global watermark for the "have I seen everything below here" case the current
   argument relies on?
