# ADR-0060 is written and wants an atom

**Date:** 2026-09-08
**Kind:** decision record, staged for `/redkiln:kb-ingest`
**Long form:** `references/adr/0060-ps-2s-axis-re-evaluated.md`
**Re-evaluates:** ADR-0036, whose *decision* it reaffirms and whose *reason* it replaces
**Amends:** PS-2's `Rule` (the MUST, the maturity and the `Cases` are untouched)

## The one question

PS-2 requires the projection port to stay unfrozen until two adapters at opposite
ends of the batch-shape axis pass the suite. ADR-0036 applied that bar at phase 6
and found part 2 unmet. Phases 10b and 11 changed the facts twice, in opposite
directions. **What does the bar say now, and does the port still ship gated?**

## Two findings

**One end is occupied.** ADR-0036 recorded that *"no cannot-hold-across-await
adapter has passed at all."* `happenstance-neon` is that adapter and it passes.
The adapter population went from one to four — SQLite, Postgres, Neon, Ladybug —
across a file, a pooled server, a one-shot HTTP proxy and an embedded graph
database, and phase 11's pre-registered condition *"the batch needs a field the
port cannot express"* did not fire.

**The other end is unobservable, not merely unbuilt.** Both drivers PS-2 named are
refuted by different mechanisms — `sqlx`'s transaction cannot come from a total
synchronous `begin`, `rusqlite`'s is `!Send`. But the deeper problem is not the
drivers: a store whose batch genuinely *is* a live transaction can implement the
port, and must declare `READS_THROUGH_BATCH = false`, which
`probe_live_transaction_shape.rs`'s own comment calls *"a false statement about
this store"*. `probe_read_through` is synchronous, infallible and takes
`&Self::Batch`, while a driver borrows its connection mutably and the statement is
I/O. **So a conformant live-transaction adapter reports the same capability
profile as a buffering one, and the suite cannot tell the ends apart.**

## The decision

**The port keeps its gate at `0.2.0`, and the reason is new.** ADR-0036's reason —
only one adapter, at one end — has expired. The gate stays because lifting
`unstable-projection` would make a semver promise out of `begin` (total,
synchronous, infallible), `probe_write` (synchronous, infallible) and
`probe_read_through` (synchronous, infallible, `&Batch`) — **precisely the
signatures the finding shows to be what forbids the second shape**. Freezing them
would make a promise out of the defect.

That reason is indifferent to a fifth adapter arriving, which the old one was not.
The asymmetry ADR-0036 recorded still decides the tie: a frozen port is a promise,
a published version can be yanked but never removed, and the cost of gating is
that a consumer types a feature name. **PS-3 keeps `[PROVISIONAL]`** — a SHOULD
still being satisfied is not a moved marker.

**A signature change is proposed and deliberately not made:**
`probe_read_through` as `&mut Self::Batch`, async, returning `Result`. It is a
breaking change to a trait testkit consumers implement, it is PS-2's owner's call
rather than an adapter lane's, and it is **not sufficient alone** — `probe_write`
being synchronous and infallible still forces a live-transaction store to buffer,
so the honest scope is the whole probe seam.

## Why ADR-0036 is not superseded

Its decision stands. Only its reason is replaced, and the two are recorded
separately rather than merged: *"only one adapter has run the suite"* was true when
written, and is exactly the kind of sentence that would otherwise be quietly
rewritten into something it never said.

## Falsifier

Reopened by a live-transaction adapter that passes the suite **and** can declare
`READS_THROUGH_BATCH = true` truthfully, or by a fifth adapter finding something
the port cannot express. **Not** reopened by another adapter at the buffered end
passing — the decision is written to be indifferent to that.
