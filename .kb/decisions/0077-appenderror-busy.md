---
id: kb-decision-0077
title: A busy store is not a broken one — AppendError::Busy is promised, and the typed loop retries it
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0077
reversibility: low
phase: 17
supersedes: null
superseded_by: null
summary: >-
  Answers kb-open-question-testkit-contention-tolerance-001 with the per-error arm and closes it.
  happenstance-core gains AppendError::Busy(E) on the non_exhaustive AppendError, with a three-part
  contract: the store refused before the batch took any effect, for a transient reason; nothing
  was written; re-running the decision is safe. An outcome the adapter cannot vouch for, such as a
  lost commit acknowledgement, MUST stay Store(E). is_busy() sits beside is_condition_violated(),
  and map_store maps the payload of Busy as it does Store's, keeping the variant. Both owner
  decisions are recorded here as taken on 2026-09-29: 1.0 promises the variant, and happenstance's
  typed commit loop retries Busy inside the same Retry bound as a violated condition, by reading
  and deciding again rather than resubmitting the refused batch. CommandError::Exhausted now
  carries the last AppendError rather than a ConditionViolated, so an exhausted busy store keeps
  its adapter error. The testkit's unreleased FaultyStore::contend_next moves from
  Store(Contended) to Busy(Contended). The adapters reclassify in the same lane: SQLite's BUSY and
  LOCKED before any row, Postgres's 40001 after its retry budget and Neon's after
  SERIALISATION_ATTEMPTS; Cloudflare never emits it. ES-43 is minted FROZEN on the condition that
  its rule, a_busy_append_left_nothing_behind, runs against memory, sqlite and a live Postgres in
  this lane and its mutant BusyAfterWriteStore fails it; otherwise it lands PROVISIONAL with a
  freeze-by-18 row. Five concurrency rules are re-spelled per error with structural floors, and
  k_disjoint_boundaries_never_conflict is narrowed by it: a Busy refusal between disjoint
  boundaries passes. The fixture-side tolerance is rejected. It lands in 0.4.0 because moving a
  refusal from Store to Busy after 1.0 silently breaks every caller matching on the old variant.
depends_on:
  - kb-decision-0066
  - kb-decision-0065
related:
  - kb-decision-0015
  - kb-decision-0022
  - kb-decision-0034
  - kb-decision-0042
  - kb-decision-0044
  - kb-decision-0072
  - kb-open-question-testkit-contention-tolerance-001
  - kb-open-question-es-6-unwritable-rule-001
  - kb-open-question-disjoint-boundaries-no-clause-001
  - kb-reference-busy-timeout-margin-001
  - kb-reference-busy-timeout-adapter-cap-sweep-001
source_paths:
  - crates/happenstance-core/src/error.rs
  - crates/happenstance-core/src/store.rs
  - crates/happenstance/src/command.rs
  - crates/happenstance/tests/retry_without_a_database.rs
  - crates/happenstance-testkit/src/faulty.rs
  - crates/happenstance-testkit/tests/contended_store_instruments.rs
  - spec/SPECIFICATION.md
last_reviewed: 2026-09-30
---

# A busy store is not a broken one — AppendError::Busy is promised, and the typed loop retries it

## The question

Until `0.4.0`, `AppendError` had four variants: `ConditionViolated`, `NoEvents`,
`ExceedsStoreLimit` and `Store(E)`. The one classifier the port offered a caller was
`is_condition_violated()`. A store that refused an append because it was momentarily busy
reported that refusal through `Store(E)`, and a store that had broken did the same. So a generic
caller, or a conformance rule, received the same value for both.

The open question weighed three answers: a per-fixture tolerance in the testkit, a per-error
channel in the contract, or ratifying a busy store as non-conformant. ADR-0066 put the choice in
phase 17's window.

The trigger the atom named, *the first adapter whose conformance run fails on contention rather
than on conformance*, has fired. The Postgres live suite exhausted its `40001` retry budget on a
pull request and surfaced `Failed` where the rule requires a commit or a `ConditionViolated`
(`crates/happenstance-postgres/src/event_store.rs:685-687`). Three of the four published
event-store adapters produce the condition, over three storage shapes: SQLite's file lock
(`SQLITE_BUSY` after ADR-0065's 15 s timeout), pooled PostgreSQL SSI (`40001` after the adapter's
budget) and Neon's one-shot-HTTP SSI (`40001` after `SERIALISATION_ATTEMPTS`). That is a spread,
which answers the atom's objection that one implementor is not one.

## Decision

1. **`AppendError::Busy(E)` is added and promised at 1.0.** Its rustdoc carries the contract:
   *the store refused before the batch took any effect, for a transient reason; nothing was
   written; re-running the decision is safe.* The payload is the adapter's own error, carried for
   diagnostics as `Store(E)` carries it and reachable through `source`. Unlike `Store`, it is not
   `transparent`: its message says the store was busy, so an operator's log line differs between
   a refusal that wrote nothing and a failure that may have written.
2. **An ambiguous outcome MUST stay `Store(E)`.** A lost commit acknowledgement, a connection
   dropped after `COMMIT` was sent, or a timeout whose effect is unknown may each have written.
   `Busy` is a claim about what the store holds, not about why the driver failed, and a store that
   cannot tell is not busy.
3. **`is_busy()` is added, and `map_store` maps `Busy`'s payload** as it maps `Store`'s. The variant
   survives the mapping, because the classification is the store's and a wrapper has no standing
   to change it. A unit test in `error.rs` rejects the one-arm shortcut that maps `Busy` into
   `Store`.
4. **The typed commit loop retries `Busy` inside the same `Retry` bound**, and it takes the full
   re-decide path: a fresh read, a fresh fold and a fresh call of the closure. That was the owner's
   decision, and the choice of path is this record's. A verbatim resubmission would also be safe,
   because nothing landed and the condition still guards the batch. It lost for two reasons: it
   would make one loop two policies to save one read, and a store busy enough to refuse is one
   where other writers are landing, so the fresh read is the one likely to spare the next attempt
   a violation. There is no backoff, for either answer, because an adapter that reports `Busy`
   has already waited out its own bound. Every other append error is returned at once.
5. **`CommandError::Exhausted` carries the last `AppendError<E>`**, not a `ConditionViolated`. An
   exhausted busy store therefore keeps its adapter error, which is the diagnostic that tells a
   hot boundary from an under-provisioned store. This changes a public field's type in
   `happenstance`, a break paid inside the `0.4.0` window.
6. **`FaultyStore::contend_next` now refuses as `Busy(FaultyStoreError::Contended)`.** It was
   `Store(Contended)`. The instrument is unreleased, so moving it costs nothing, and `Busy` is
   exactly what it models: the inner store is never called, so nothing is written. `Contended`
   stays as the payload.
7. **The adapters reclassify in this lane.** SQLite reports `BUSY` and `LOCKED` as `Busy` only when
   they arrive before any row is written, never at `COMMIT` after rows. Postgres reports a `40001`
   that outlived its retry budget as `Busy`, and Neon one that outlived `SERIALISATION_ATTEMPTS`.
   Cloudflare's store is a single-threaded Durable Object and documents that it never emits
   `Busy`.
8. **ES-43 is minted in §3**, after ES-25. ES-6 names `Busy(E)` beside `Store(E)` in its payload
   paragraph, since the busy variant carries the same payloads.

### The per-error arm, and why the per-fixture one lost

A per-fixture capability would have let a fixture declare "I may be busy", and relaxed the
concurrency rules for it. It lost for three reasons:

- **It helps no caller outside the testkit.** A generic retry loop would still receive `Store(E)`
  and have no way to tell a busy refusal from a failure.
- **It would be breaking to remove.** A defaulted `Fixture` constant is additive to add under
  ADR-0042, and a promise once it exists.
- **It leaves the harm on the channel where it lives.** The nearer precedent is VT-25 and CF-40's
  `ExceedsStoreLimit` (ADR-0015), which repaired the same harm, a refusal a caller could not
  classify, on the same channel, by lifting it into the contract's enum.

### How the concurrency rules change, and why this is not a tolerance

Five concurrency rules failed on a busy contender, and all five are re-spelled per error, with
structural floors rather than tolerated fractions. The open question counted three, the ones that
assert every contender commits or that none is refused without a winner
(`positions_are_unique_under_concurrent_appends`, `append_returns_the_callers_own_last_position`
and `exactly_one_of_n_contenders_commits`). Two more read any refusal as a failure as well:
`k_disjoint_boundaries_never_conflict` and `a_concurrent_reader_never_sees_a_partial_batch`.
CF-33 forbids a rule an operation count, and a fraction would be one.

- Every contender commits, or is `ConditionViolated` where a condition exists, or is `Busy`.
- At least one commits, because a store where every contender is busy has made no progress.
- A boundary with no winner rejected nobody, because a violation needs a matching event to have
  landed, and only that boundary's own winner could have written one.
- Positions are unique among the commits, and the store holds every committed batch and nothing
  more.
- A contender answered `Store` still fails.

The floors and the new no-effect rule are what keep this from measuring luck, the hazard the open
question named. `CONTENDERS` stays at 64. Under CF-31 these are changes to what five rules mean,
so they make a testkit major, which `0.4.0` is.

**One of the five is narrowed, and this record authorises it.**
`k_disjoint_boundaries_never_conflict` enforced the DCB independence proposition in its
strong form: commands sharing no consistency boundary all commit. It now enforces a narrower one.
No boundary elects two winners, no disjoint contender is told `ConditionViolated`, and at least one
contender commits, but a disjoint contender refused as `Busy` passes. A store that takes a global
lock, or whose SSI refuses transactions across disjoint boundaries, passes as long as it reports
the refusal as `Busy`. ES-43's first sentence admits exactly that, and live PostgreSQL does it: a
review run of `a_busy_append_left_nothing_behind`, whose contenders each guard a tag only they
write, reported 23 `Busy` answers out of 64 (one run, not reproduced). Holding the strong form would have made the PostgreSQL
adapter non-conformant for refusing, transiently and honestly, what its isolation level refuses.
**The owner decided that independence is a promise about conflict, not liveness**
(`kb-decision-wi-2ab1f3`): `Busy` wrote nothing and the typed loop retries it, so a disjoint
command refused as `Busy` has not been told it conflicts. The rule was renamed to say so, from
`k_disjoint_boundaries_admit_exactly_k_commits` to `k_disjoint_boundaries_never_conflict`, while the
testkit's `0.4.0` major makes the rename free. The clause the proposition is still owed
(`kb-open-question-disjoint-boundaries-no-clause-001`) states this form.

### The new rule, and the freeze condition

`a_busy_append_left_nothing_behind` races contenders that each carry a unique tag. For every
contender answered `Busy`, it reads that tag back and asserts that nothing matches. Its named wrong
implementation is `BusyAfterWriteStore`, which commits and then answers `Busy`. It goes in the
testkit's `tests/`, per CF-29.

ES-43 is `[FROZEN]` on landing only if that rule runs green against `MemoryEventStore`,
`happenstance-sqlite` and a live PostgreSQL in this lane, and `BusyAfterWriteStore` fails it. If
any of those is missing when the lane merges, ES-43 lands `[PROVISIONAL]` with a `freeze-by-18`
row in `runbook/ledgers.md`, as ADR-0066 §2 requires, and this record is not amended. The clause
text says so. Neon's reclassification is verified live only by CI's `live-neon` job, which is
gated on a secret, so a local run of this lane has only a transport fake as Neon evidence.

## Why now

Adding the variant is additive at any time. Moving an adapter's refusal from `Store(e)` to
`Busy(e)` is not. A caller who wrote `Store(e) if e.is_serialization_failure()` compiles unchanged
afterwards and silently stops matching. Inside `0.4.0` that break is paid once, with a changelog
entry. After 1.0 it would be a major for three adapters.

## What this changes for a caller

- `happenstance-core`: a new variant and a new method. Additive at the type level.
- `happenstance`: `commit` and `commit_with` retry `Busy`, and `Exhausted.source` changes type.
  Both are breaking, and both are marked BREAKING in the changelog.
- `happenstance-sqlite`, `-postgres` and `-neon`: refusals that were `Store(E)` arrive as
  `Busy(E)`. That is a behaviour break, marked BREAKING.
- `happenstance-testkit`: `contend_next`'s channel, which is unreleased, and five rules whose
  meaning changes (CF-31), one of them narrowed.

## Falsifier

This record is wrong if any of the following happens:

- An adapter cannot tell, at the point it refuses, whether the batch took effect, and still has to
  report the refusal as retryable. The contract's "nothing was written" would then be
  unimplementable, and the variant would have to split.
- A caller needs to distinguish busy refusals by cause (lock against serialisation) in order to
  choose a retry policy. The payload serves diagnostics, not control flow, so that would need a
  new record.
- The typed loop's shared bound proves wrong in practice, with busy refusals exhausting a bound
  sized for violations. Separate bounds would be an additive `Retry` constructor.
