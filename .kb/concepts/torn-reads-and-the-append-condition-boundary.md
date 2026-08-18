---
id: kb-concept-torn-read-append-boundary-001
title: Why the append condition does not catch a torn read
kind: concept
status: accepted
authority_tier: note
summary: >-
  The obvious objection to spending anything on read isolation is that the append condition
  re-checks at the store, so a stale read is caught on write. It is not, and the reason is that
  the boundary is derived from the read itself: read_decision_model returns the maximum position
  it observed and AppendCondition::after_opt consumes it, so a torn read that missed an event
  below its own observed maximum produces a condition the store evaluates as satisfied. The
  failure is a silently accepted append, not a rejected one — the direction that loses data
  rather than the direction that retries. Two structural reasons stand behind it: a fold across
  two states can be internally inconsistent in a way no condition can express, and a read-only
  consumer never appends, so for it the read is the only isolation there is. This is the
  mechanism ADR-0011's sampling instant, ADR-0012's guards and ADR-0013's visibility invariant
  each protect; none states it, because each assumes it.
depends_on: []
related:
  - kb-decision-0012
  - kb-decision-0013
  - kb-decision-0022
source_paths:
  - .kb/_intake/0011-read-laziness-and-isolation.md
  - references/adr/0011-read-laziness-and-isolation.md
  - crates/happenstance-core/src/store.rs
  - crates/happenstance-core/src/append.rs
  - spec/SPECIFICATION.md
last_reviewed: 2026-08-10
---

# Why the append condition does not catch a torn read

## The objection

DCB's decision procedure is read, decide, append-with-a-condition, and the store re-evaluates
the condition at commit time. So the natural question, asked before spending anything on read
isolation, is: if a read tears — if the handler observes an inconsistent slice of the log — does
the condition catch it on write? The append is, after all, the one place the store gets a second
look.

## Why it does not

The condition's boundary is not independent evidence checked against the read. It is **derived
from the read itself**, and that is what breaks the safety net. `read_decision_model`
(`store.rs:205-215`) folds the matched events and returns the maximum position it observed
alongside the decision state; `AppendCondition::after_opt` consumes exactly that maximum as its
`after` boundary. The condition the store checks at commit is therefore built entirely out of
what the read believed it saw.

A torn read that missed an event below its own observed maximum — say it saw positions
`{5, 12}` and, because of the tear, never saw an event at position `9` that also matches its
query — returns `12` as the maximum anyway, because `12` really was observed. The resulting
condition says, in effect, "reject if anything matching this query exists after position 12."
Position 9 is below 12. The condition is silent about it by construction: the boundary tells the
store to ignore precisely the range the tear hid the event in. **A torn read does not turn into a
rejected append; it turns into an accepted one.** The failure mode is silent data loss on commit,
not a spurious retry — the more dangerous of the two directions, because nothing about the
append's success signals that anything was wrong.

## Two structural reasons, not one

This is not an implementation detail of one method that a smarter fold could route around.

**A decision model assembled from two inconsistent states can be internally inconsistent in a way
no condition can express.** The condition checks that nothing *new* arrived after the boundary; it
has no vocabulary for "the state I folded was never a real snapshot of anything." A condition is a
predicate over what happened *after* a point, not an assertion about the *coherence* of what
happened before it.

**A read-only consumer never appends at all.** Projection runners, replay tooling, exports — for
every one of them, the read is the entirety of the isolation guarantee available. There is no
downstream re-check to lean on, because there is no downstream write to attach one to.

## Why this belongs to more than one document

This mechanism is the shared premise behind three separate decisions, and none of them states it —
each assumes a reader already has it in hand. ADR-0011's requirement that a read be evaluated
against one sampled state, no later than the first poll, exists to prevent the tear described
above from happening at all. ADR-0012's guard boundaries on `AppendCondition` are sound only
because the boundary they check is meaningful, which requires the read that produced it not to
have torn. ADR-0013's visibility invariant — that no event may become visible at a position at or
below one already observed — is precisely what keeps a `read`'s observed maximum from becoming a
boundary that quietly excludes something still in flight. Each decision spends real cost
(a ceiling, a guard sequence, a frozen invariant) protecting this one property, and a reader who
does not have the mechanism in hand will read each cost as arbitrary rather than as three
instances of the same defense.
