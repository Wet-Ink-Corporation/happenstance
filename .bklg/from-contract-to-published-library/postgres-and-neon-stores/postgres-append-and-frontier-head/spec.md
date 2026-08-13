---
item: HS-S0062
stage: spec
created: 2026-08-12T13:47:00.713Z
updated: 2026-08-12T13:47:00.713Z
template_sig: 87bbf1d0
rendered_sig: 0349a5ae
---

# Spec — A real Postgres append path and a frontier head

## Scope lock

| What | Path |
| ---- | ---- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` |
| Project item | `.bklg/from-contract-to-published-library/postgres-and-neon-stores/project.md` |
| Project briefs (architecture / testing / deployment) | `.bklg/from-contract-to-published-library/postgres-and-neon-stores/_decomposition.md` — Architecture §2 (composition roots), §4 (Postgres data flow), §7 (gate wiring), §9.3 (what is genuinely open); Testing §6 (the two gate command lists), §7 (fixtures) |
| Project grounding | `.bklg/from-contract-to-published-library/postgres-and-neon-stores/_grounding.md` |
| Signed-off design | `.bklg/from-contract-to-published-library/postgres-and-neon-stores/_design.md` — **no user-facing surface**, and that determination is what was approved (`_design.md:86-95`) |
| Story map (this story's row) | `.bklg/from-contract-to-published-library/postgres-and-neon-stores/_storymap.md:61`, expanded at `:96-108` |
| This spec | `.bklg/from-contract-to-published-library/postgres-and-neon-stores/postgres-append-and-frontier-head/spec.md` |
| Roadmap / plan of record | `RUNBOOK.md:4311-4390` (phase 10 in full); `RUNBOOK.md:606` (ES-10 – ES-12 residual exposure, this phase as owner) |

Traces to project **AC-002** (`project.md:228-232`), **AC-004** (`project.md:238-239`)
and **AC-010** (`project.md:263-266`), sharing AC-002 and AC-004 with
`postgres-rule-controls` and AC-010 with `neon-append-and-read-over-http` on the
splits `_storymap.md:218-226` states. `depends_on`:
`postgres-schema-and-live-fixture` — migration 1, `PostgresFixture` /
`ConcurrentFixture`, the in-crate gating mechanism and the live-Postgres CI job
all arrive from it and are **consumed here, not rebuilt**.

## One-line PR slice

An adapter author gets a real `append` / `head` / `contains_event_id` that buys
ES-10's visibility invariant, refuses over-limit batches with
`ExceedsStoreLimit`, and turns `event_store_conformance!` and
`event_store_model_conformance!` green against a live pinned Postgres.

## Executive summary

**Pointer.** `crates/happenstance-postgres` compiles today and implements
nothing. `impl SendEventStore for PostgresEventStore` carries four methods
(`crates/happenstance-postgres/src/event_store.rs:121-175`); exactly one of them
— `read` — has a real body, deliberately, because `read` is not `async` and so
its whole body has to be real (`:124-134`, and the state machine it returns at
`read_stream.rs:37-73`). The other three are `todo!()` with the reason written
above them: `append` is "blocked on the ES-10 decision… which is what decides
whether this is one statement or three" (`:141-143`), `head` is `todo!()` rather
than `SELECT max(position)` because "writing the cheap version now would encode
the answer by accident" (`:147-160`), and `contains_event_id` needs two columns
the intended schema does not have (`:163-174`).

**Delta this PR lands.** Those three bodies, for real, against the live pinned
Postgres the dependency story stands up — and the two conformance families
invoked against it from `crates/happenstance-postgres/tests/`. Concretely:
`append` as one transaction that refuses the empty batch before it evaluates
anything, evaluates the append condition and inserts on one snapshot, and
assigns positions under `xid8` + `pg_snapshot_xmin` so that writers do **not**
serialise; `head` returning the **visibility frontier** rather than
`max(position)`; the frontier predicate composed *into* the cursor's `DECLARE`
inside the same `REPEATABLE READ` transaction, so the snapshot cannot move under
a caller between `FETCH`es; `contains_event_id` answering from real origin
columns with its frontier disagreement recorded rather than settled; the three
store ceilings refused through `AppendError::ExceedsStoreLimit { limit:
StoreLimit::… }` and never through `AppendError::Store`; and
`event_store_conformance!` plus `event_store_model_conformance!` green in the
live job, with `cargo xtask ci` still Docker-free and network-free on a clean
checkout.

**What it deliberately does not land.** It does not *choose* the mechanism.
ADR-0024 does not exist, and nothing in this PR may be written as though it did
(`_decomposition.md` Architecture §Intent, and §3 tension 1). What this story
wires is the arm ADR-0013 already established as affordable on a phase-2
measurement — the only one of four that both passed an inversion detector and
left writers unserialised (`.kb/decisions/0013-position-assignment-and-visibility.md`,
`experiments/position-visibility/README.md:13-19`) — so that
`adr-0024-position-visibility-mechanism` has a real adapter to re-measure rather
than four SQL strategies (`.kb/open-questions/postgres-arm-c-structural-cost.md`).
It also does not land the concurrency family (`postgres-concurrency-family`), the
`nextval()` negative control or the poll-padding decorator
(`postgres-rule-controls`), the consumer-facing structural bill
(`postgres-structural-bill`), or the projection store.

## Context pack

The decisions this story must honour, stated as decisions. Read this section and
you can start; everything deeper is a signposted anchor.

**1. `head` is a frontier, and that is a contract fact, not an implementation
detail.** ES-10 is `[FROZEN]`: once any reader has observed position *P*, no
subsequent read may yield an event at a position ≤ *P* that was not already
visible (`spec/SPECIFICATION.md:2816-2831`). Under `xid8` + `pg_snapshot_xmin`
that is bought by admitting only rows below
`pg_snapshot_xmin(pg_current_snapshot())`, and the clause already spells out what
follows: `head()` reports a **frontier** rather than `max(position)`,
read-your-own-writes does **not** hold — `append` returning `Ok(P)` does not
promise the next `head()` is at or above *P* — and staleness is bounded by the
longest open write transaction *anywhere in the cluster*, measured at 0.688 ms
with no holder and 4010.719 ms behind an unrelated five-second write in an
unrelated database (`spec/SPECIFICATION.md:2833-2842`). This is why ES-30's
`head_is_the_highest_visible_position` asserts a **bound**, not an equality
(`crates/happenstance-testkit/src/suite.rs:1798`). An implementer who "fixes" a
surprising `head` by returning `max(position)` has silently reverted ES-10 and
the suite will not necessarily say so.

**2. The mechanism is wired here; the choice is ADR-0024's.** This is the seam
most likely to be collapsed in either direction. Do not write "per ADR-0024" —
there is no such atom, and citing one that AC-001 must still write is the failure
Architecture §3 tension 1 names explicitly. Equally, do not treat the wiring as a
decision: the record this story owes is *what was wired and on whose prior
authority* — ADR-0013's phase-2 measurement, which rejected two serialising
strategies at 16× and 30× throughput cost and rejected tag-keyed advisory locks
because they buy a per-boundary invariant ES-10 does not state
(`experiments/position-visibility/README.md:13-19`). If the re-measurement in
`adr-0024-position-visibility-mechanism` overturns this arm, that is a recorded
rework, not a defect in this story.

**3. A serialised sequence table passes every rule in this story and destroys
the project.** `UPDATE hs_sequence SET n = n + 1 RETURNING n` inside the append
transaction makes allocation order commit order, so ES-10 holds trivially and the
signature never moves (`crates/happenstance-postgres/src/event_store.rs:43-53`).
It is also `MemoryEventStore` with network latency, which deletes the only axis
this crate exists to occupy — the far end of position allocation, empty at both
ends today (`RUNBOOK.md:685-702`). The instrument that catches it is
`postgres-concurrency-family`, a slice-mate; this story must not reach for the
shape that makes that story impossible to pass honestly.

**4. `read` must stay lazy, and the frontier goes *inside* the `DECLARE`.**
`EventStore::read` returns the stream at the top level and is not `async`
(ADR-0001/ADR-0008, `CLAUDE.md` constraint 3). Nothing is acquired until the
first poll; the first poll opens `BEGIN ISOLATION LEVEL REPEATABLE READ` and
`DECLARE`s a server-side cursor, and every subsequent chunk is a `FETCH`
(`crates/happenstance-postgres/src/read_stream.rs:37-57`). The visibility
predicate belongs in that `DECLARE`, evaluated once against the transaction's
snapshot. Evaluating it per chunk is the easy way to get the read half wrong: the
frontier advances between `FETCH`es and rows appear beneath positions the caller
has already seen — which is ES-10's violation arriving through the read path
instead of the write path. Keyset pagination is named and rejected for the same
reason (`read_stream.rs:59-73`, ES-11/ES-12,
`.kb/decisions/0011-read-laziness-and-isolation.md`).

**5. The empty batch is refused before the condition is evaluated.** ADR-0012
fixes both the borrowed batch and the ordering (`.kb/decisions/0012-append-shape-and-preconditions.md`).
An append of zero events is refused first, so it never becomes an expensive
no-op that took a lock, and never becomes a conditional check with nothing to
write.

**6. A ceiling is a fact; a refusal has one spelling.** The three limits are
`Option<usize>` on the fixture and not `Capability`, because a store reporting it
has no ceiling is stating a fact and owes nobody a reason
(`crates/happenstance-testkit/src/contract.rs:213-279`). Stating a number commits
the store: the rule appends exactly that many bytes and requires acceptance, then
one byte more and requires
`AppendError::ExceedsStoreLimit { limit: StoreLimit::EventDataLen, .. }` — never
`AppendError::Store`, and never a truncation (`suite.rs:4316-4457`). Independently,
VT-21 – VT-24's floors (65,536 bytes, 128 events per batch and the tag and query
minima) must be *accepted* — `store_accepts_the_guaranteed_minimum_payload`
(`suite.rs:3775`, `crates/happenstance-core/src/limits.rs:22-54`). The two rules
are independent and both are owed an answer. The fixture constants are the
dependency story's; the **adapter behaviour behind them** — the refusal path and
the accepted floors — is this story's, and a number that is not where the real
ceiling sits fails in one direction or the other.

**7. Never assert on literal position values.** The specification permits gaps
and this is the project where gaps stop being hypothetical: under a frontier
mechanism a position may be assigned, committed and never contiguous. Compare
against positions the store actually assigned. `cargo xtask lint-position-literals`
is a `REQUIRED` gate step (`xtask/src/main.rs:389`, `:687`) and CF-6 is why.

**8. The live suite is gated by whole invocation, never by hiding a rule.** The
default gate stays Docker-free and network-free (DR-9, project AC-011), so the
two macro invocations this story adds must not run in
`cargo test --workspace --all-features` with no server reachable. The gating
mechanism — `#[ignore]`, `required-features`, or an env read — arrives from
`postgres-schema-and-live-fixture` and is consumed here. The one hard constraint
is DR-5's: gating the whole invocation is a different act from `#[cfg]`-ing a
conformance rule out of a macro's expansion, and only the second is forbidden
(`_decomposition.md` Architecture §7, Testing §6).

**9. Grow the error enum; do not flatten `sqlx::Error::Database`.**
`PostgresEventStoreError` is `#[non_exhaustive]` and already carries the decode
variants that exist because Postgres column types are weaker than the contract's
(`bigint` admits zero, `SequencePosition` is a `NonZeroU64`)
(`crates/happenstance-postgres/src/error.rs:29-58`). The mechanism's own decode
failures — an `xid8` that will not parse, a frontier row that is absent — get
variants of their own. What is forbidden is a variant per `SQLSTATE`: `sqlx`
already exposes the code, and per-code variants are a second, worse copy of it.
Neither enum gains a `ConditionViolated` variant; a condition violation is not an
adapter failure (ADR-0009, `error.rs:1-11`).

**10. `contains_event_id` inherits a question, and records it rather than
settling it.** It needs `origin_store` and `origin_position` columns the intended
schema does not have (`event_store.rs:163-174`). Under a frontier mechanism it
also inherits a genuine disagreement: answering `true` for a committed row above
the frontier makes this method contradict `read`; answering `false` makes ingest
re-accept an event the store already holds. ES-41 is `[PROVISIONAL]`
(`spec/SPECIFICATION.md:4381`) and replication semantics belong to HS-P0017. Pick
an answer, implement it, and write down which one and why — do not settle
replication here.

**The persona slice.** The user of this project is an adapter author and a
consuming application (`_storymap.md:19-23`), and this story is the first point
in the whole initiative where the journey *Learn when you are finished*
(`.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md`)
runs against a store that can genuinely fail. Every implementation that has ever
passed the suite serialises its writers and assigns positions under a lock held
to commit — four adapters, one storage shape (`RUNBOOK.md:665-674`). E2E-01
("once a position is observed, nothing below it appears later") is written today
with a ⚠ against it saying that a real adapter needs `happenstance-postgres`,
"does not exist" (`spec/E2E-CASES.md:52-58`). What the adapter author meets when
this story lands is a conformance run whose pass column was earned by a pooled,
networked, non-serialising store, and initiative **AC-06** — *the adapter
author's storage shape is not quietly assumed* — stops resting on a fixture.

## Integration contract

| Field | Value |
| ----- | ----- |
| **Archetype** | `capability` — a user-observable slice through every layer of this repository's medium: a port impl, a conformance run that reports pass or a named failure, and a public API a caller meets. |
| **Slice / milestone** | `postgres-live-suite`. **Slice-mates**, implemented in one context and mounted as one integrated surface: `postgres-schema-and-live-fixture` (foundation, upstream of this story), `postgres-concurrency-family` and `postgres-rule-controls` (both downstream of it) — `_storymap.md:60-63`, merge order at `:246-250`. |
| **Mount point** | `crates/happenstance-postgres/src/event_store.rs` — the `impl SendEventStore for PostgresEventStore` block (`:121-175`), which is Architecture §2's **Root A**. It is mounted *for real* at **Root B**: the integration-test target under `crates/happenstance-postgres/tests/` that `postgres-schema-and-live-fixture` creates, where `event_store_conformance!(mod_name = …, emit = …, fixture = PostgresFixture::…)` and `event_store_model_conformance!` are invoked (`crates/happenstance-testkit/src/lib.rs:312`). Neither crate has a `tests/` directory today; its file name is the slice's to settle in one context, and nothing in-tree names it yet. A body filled without those invocations is the "constructed but unmounted" failure in this medium — a `todo!()` body type-checks against any signature, so "it compiles" counts for nothing (`RUNBOOK.md:3061-3068`). |
| **Wires into** | **Consumed from the dependency story**: migration 1 (identity, time, tag storage and the mechanism's column), `PostgresFixture`, `concurrency::ConcurrentFixture`, the whole-invocation gating mechanism and the live-Postgres CI job. **Contract types**: `SendEventStore`, `AppendCondition`, `AppendError`, `SequencePosition`, `Query`, `ReadOptions`, `SequencedEvent`, `EventId` (`crates/happenstance-core/src/store.rs`), and `StoreLimit` / `MIN_SUPPORTED_*` (`crates/happenstance-core/src/limits.rs:22-54`). **Testkit**: `Fixture` and its three `Option<usize>` ceilings (`crates/happenstance-testkit/src/contract.rs:213-279`), the two conformance macros, and `happenstance_testkit::rules` (`crates/happenstance-testkit/src/lib.rs:189`) — read-only here; driving the rule functions directly is `postgres-rule-controls`'. **In-crate**: `PgReadStream` (`read_stream.rs:37-73`), `PostgresEventStoreError` (`error.rs:29-58`), the `event-store` feature (`Cargo.toml`). |
| **Renders surfaces** | **None.** `_design.md` records no user-facing surface for this project and the sign-off approved that determination (`_design.md:10-41`, `:86-95`). `design.capture` is deliberately absent from `.redkiln/config.yaml`, making the perceptual review a declared skip, not a silent pass. |
| **Public items** | No new `pub` type or free function. What changes is the *behaviour* behind four already-public method signatures on `impl SendEventStore for PostgresEventStore`, plus new `#[non_exhaustive]` variants on `PostgresEventStoreError` for the mechanism's decode failures (`error.rs:29-58`). Adding a variant to a `#[non_exhaustive]` enum is not a breaking change, and `publish = false` is still on the crate (`Cargo.toml:12`) — its removal is `deskeleton-and-package-readiness`'. |
| **Conformance rule(s)** | Observed by, and named rather than gestured at: `nothing_below_an_observed_position_appears_later` (CF-13, `crates/happenstance-testkit/src/suite.rs:5880`) — the rule the whole mechanism exists to pass; `head_is_the_highest_visible_position` and `head_of_an_empty_store_is_none` (`suite.rs:1798`, ES-30); `store_accepts_the_guaranteed_minimum_payload` (`suite.rs:3775`, VT-21 – VT-24); `append_reports_exceeded_store_limits` (`suite.rs:4316-4457`); `racing_conditional_appends_elect_one_winner`; and every other rule in `event_store_conformance!`'s expansion plus the whole `event_store_model_conformance!` family — the suite runs entire, never a hand-picked subset. **This story adds no rule to `happenstance-testkit`**; the testkit is a must-not-change seam here (Architecture §1, Testing §Intent). |
| **Clause(s)** | **Discharges evidence for** ES-10 (`spec/SPECIFICATION.md:2816`, `[FROZEN]`), ES-11 and ES-12 (`[PROVISIONAL]`, bought by `BEGIN REPEATABLE READ` + `DECLARE CURSOR`), ES-30 (`:3941`), and VT-21 – VT-24 (`:1483`, `:1556`). **Amends none.** No `[FROZEN]` marker moves in this PR and no ADR is owed by it; regenerating §7.1–§7.2's markers is `far-end-discharge-record`'s, by `cargo xtask spec-trace --write`, never by hand. ES-41 (`:4381`) is touched by `contains_event_id` and stays `[PROVISIONAL]` — the answer is recorded, the clause is not amended. |
| **Advances DoD scenario** | Initiative **DoD 5** — *a store that does not serialise its writers passes the suite, with the position-visibility cost measured rather than estimated.* This story lands the first half (the store, passing) and hands the second half (the measurement) to `adr-0024-position-visibility-mechanism`; DoD 5 is not green until both land. It also makes **E2E-01** writable against an adapter that can genuinely fail it (`spec/E2E-CASES.md:52-58`, `RUNBOOK.md:4385`) and advances initiative **AC-06**. |

## PR boundary

`redkiln verify --grain story` reads the first fenced block under this heading and
fails on any file changed outside it.

```
crates/happenstance-postgres/src/**
crates/happenstance-postgres/tests/**
crates/happenstance-postgres/migrations/**
crates/happenstance-postgres/Cargo.toml
.bklg/from-contract-to-published-library/postgres-and-neon-stores/postgres-append-and-frontier-head/**
```

The boundary is one crate wide on purpose. `happenstance-core` and
`happenstance-testkit` are must-not-change seams here (Architecture §1) and no
other adapter is touched — the dependency rule forbids it. The migrations glob is
present because this story may need to *amend* migration 1 (see **Data and
migrations**); if it does not, the directory simply does not appear in the diff.
The implementer may also touch the composition-root and wiring files named in the
Integration contract to mount this slice; here that is the `tests/` target and the
crate's own `Cargo.toml`, both inside the boundary already, and that is not scope
drift.

**In this PR**

- `append`, `head` and `contains_event_id` implemented for real, replacing the
  three `todo!()`s at `event_store.rs:143`, `:160` and `:173`.
- The frontier predicate composed into `PgReadStream`'s `DECLARE`, inside the
  existing `REPEATABLE READ` transaction (`read_stream.rs:37-73`).
- New `#[non_exhaustive]` variants on `PostgresEventStoreError` for the
  mechanism's decode failures.
- `event_store_conformance!` and `event_store_model_conformance!` invoked from
  `crates/happenstance-postgres/tests/`, behind the dependency story's
  whole-invocation gate, green in the live-Postgres CI job.
- `Cargo.toml` only if the model family's `proptest` feature must be enabled on
  the `happenstance-testkit` dev-dependency (`crates/happenstance-testkit/src/lib.rs:180-183`),
  or a `sqlx` feature is genuinely required — a manifest decision made
  deliberately and recorded, never incidentally (`Cargo.toml:18-22`).
- The `origin_store` / `origin_position` columns `contains_event_id` needs,
  folded into migration 1 in coordination with the slice-mate that owns it.
- This story's own `_ledger.md`.

**Explicitly not in this PR**

- **Choosing the ES-10 mechanism.** ADR-0024, its re-measurement against the real
  adapter and the long-running-transaction scenario are
  `adr-0024-position-visibility-mechanism`'s (project AC-001). Nothing here cites
  ADR-0024 as prior art, because it does not exist.
- `event_store_concurrency_conformance!` and the `ConcurrentFixture` invocation —
  `postgres-concurrency-family` (project AC-003).
- The `nextval()` negative control, the mutant-control run of
  `happenstance_testkit::rules::*` from the Postgres side, and the poll-padding
  decorator over `PreCommitPositionStore` that `spec/SPECIFICATION.md:2844-2850`
  names as owed by this phase — all `postgres-rule-controls`
  (`.kb/open-questions/poll-count-bounds-the-visibility-rule.md`).
- The consumer-facing structural bill in the crate's rustdoc, and the decision on
  whether a clause of its own is owed — `postgres-structural-bill` (project
  AC-009).
- `PostgresProjectionStore`, which stays `todo!()` — and therefore the crate-level
  `#![allow(clippy::todo)]` (`src/lib.rs:59-63`) and `publish = false`
  (`Cargo.toml:12`) both stay. Their removal is `deskeleton-and-package-readiness`'.
- Any change to `happenstance-core`, `happenstance-testkit`, another adapter, the
  `REQUIRED` step array in `xtask/src/main.rs`, or `.github/workflows/ci.yml`.
- Settling replication semantics for `contains_event_id` (HS-P0017), reopening the
  global-vs-per-boundary invariant (owned by HS-P0010,
  `.kb/open-questions/global-versus-per-boundary-visibility-invariant.md`), or
  settling CF-40's contested ownership (`.kb/open-questions/cf-40-fixture-limits-ownership.md`).
- Regenerating `spec/SPECIFICATION.md`'s §7.1–§7.2 markers or writing the far-end
  discharge record — `far-end-discharge-record` (project AC-013).

**Merge DoD (one line).** `event_store_conformance!` and
`event_store_model_conformance!` are green against a live pinned Postgres in the
project's CI job with no rule absent from the run, `cargo xtask ci` is green on a
clean checkout with no Docker and no network, and `cargo xtask affected --base main`
is green on the story's tree.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| `append` is one transaction, and the empty batch is refused first | Zero events is refused before the append condition is evaluated and before anything is locked, so it is neither an expensive no-op nor a conditional check with nothing to write. The batch stays borrowed — `&[Event]`, not an owned collection. → AC-001 | `.kb/decisions/0012-append-shape-and-preconditions.md`; `crates/happenstance-postgres/src/event_store.rs:136-144` |
| The condition and the insert see one snapshot | The append condition's evaluation and the insert are the same transaction, so a probe followed by a write is not what happens. Whether Postgres reuses Neon's single-statement CTE shape or exploits the interactive transaction it has and Neon does not is genuinely open and the implementer's call — recorded, not defaulted. → AC-002 | `_decomposition.md` Architecture §9.3; `crates/happenstance-neon/src/lib.rs:46-77` (the CTE, for comparison only) |
| Positions are assigned so that allocation order is visibility order, without serialising writers | The arm wired is `xid8` + `pg_snapshot_xmin`: store the transaction's `xid8` beside the row, admit only rows below `pg_snapshot_xmin(pg_current_snapshot())`. `position` is deliberately **not** `bigserial`. The two serialising arms are rejected on ADR-0013's prior measurement, not on preference — and a serialised sequence table would pass this story's rules while making `postgres-concurrency-family` unpassable honestly. → AC-003, AC-011 | `.kb/decisions/0013-position-assignment-and-visibility.md`; `experiments/position-visibility/README.md:13-19`; `crates/happenstance-postgres/src/event_store.rs:22-24`, `:43-75` |
| `head` returns the visibility frontier, not `max(position)` | `max(position) WHERE xid < pg_snapshot_xmin(pg_current_snapshot())`, which trails the maximum. On an empty store it is `None`. Read-your-own-writes does not hold and is not claimed: `append` returning `Ok(P)` makes no promise about the next `head()`. ES-30's rule asserts a bound rather than an equality precisely to admit this. → AC-004 | `spec/SPECIFICATION.md:2833-2842`, `:3941`; `crates/happenstance-postgres/src/event_store.rs:146-161`; `crates/happenstance-testkit/src/suite.rs:1798` |
| The frontier predicate lives inside the cursor's `DECLARE` | Composed into the `DECLARE` in the same `BEGIN ISOLATION LEVEL REPEATABLE READ` transaction, evaluated once against that snapshot. Not per `FETCH`: the frontier advances between chunks, and a per-chunk predicate lets rows appear beneath positions the caller has already seen — ES-10's violation arriving through the read path. Keyset pagination remains rejected. → AC-005 | `crates/happenstance-postgres/src/read_stream.rs:37-73`; `.kb/decisions/0011-read-laziness-and-isolation.md`; `spec/SPECIFICATION.md` ES-11/ES-12 |
| `read` stays non-`async` and acquires nothing until the first poll | `read` returns the stream at the top level; the connection, the transaction and the cursor are all acquired inside the first `poll_next`. The frontier must not be computed eagerly in `read` — that needs an await `read` cannot have, and the two tests in `memory.rs` that guard this shape exist because it has been broken before. → AC-012 | `CLAUDE.md` constraint 3; `.kb/decisions/0001-async-port-flavours.md`; `crates/happenstance-postgres/src/event_store.rs:124-134`; `read_stream.rs:37-57` |
| `contains_event_id` answers from real columns, and its disagreement is recorded | `origin_store` / `origin_position` are added to migration 1. Under a frontier, a committed row above the frontier is invisible to `read`: answering `true` contradicts the stream, answering `false` makes ingest re-accept an event the store holds. Choose, implement, and write down which and why. ES-41 stays `[PROVISIONAL]`; replication semantics are HS-P0017's. → AC-006 | `crates/happenstance-postgres/src/event_store.rs:163-174`; `spec/SPECIFICATION.md:4381`; `_decomposition.md` Architecture §4 |
| Over-limit appends are refused with `ExceedsStoreLimit`, never `Store` | `AppendError::ExceedsStoreLimit { limit: StoreLimit::EventDataLen \| TagsPerEvent \| EventsPerBatch, .. }`. Never `AppendError::Store`, never a truncation, and nothing of the refused value survives. The rule appends exactly the stated ceiling and requires acceptance, then one more and requires the refusal — a number in the wrong place fails in one direction or the other. → AC-007 | `crates/happenstance-testkit/src/suite.rs:4316-4457`; `crates/happenstance-testkit/src/contract.rs:213-279`; `crates/happenstance-core/src/limits.rs:54-74` |
| The guaranteed minima are accepted, independently of any ceiling | VT-21's 65,536-byte payload floor and VT-24's 128-event batch floor are *accepted* by the adapter, and the tag and query minima with them. This rule and the ceiling rule are independent; both are owed an answer. → AC-007 | `crates/happenstance-testkit/src/suite.rs:3775`; `crates/happenstance-core/src/limits.rs:22-43`; `spec/SPECIFICATION.md:1483`, `:1556` |
| Both conformance families are invoked, entire | `event_store_conformance!(mod_name = …, emit = …, fixture = PostgresFixture::…)` and `event_store_model_conformance!` from `crates/happenstance-postgres/tests/`. The model family is behind the testkit's `proptest` feature and `not(target_arch = "wasm32")`, so the dev-dependency must enable it. The full macro expansion, never a hand-picked subset — a rule absent from the run is indistinguishable in CI output from a rule that passed. → AC-008 | `crates/happenstance-testkit/src/lib.rs:312`, `:180-183`; `_decomposition.md` Testing §6 |
| The default gate stays Docker-free and network-free | The invocations are gated whole by the dependency story's mechanism, so `cargo test --workspace --all-features` exits zero with no server reachable. `#[cfg]`-ing a rule out of the expansion is the forbidden move; gating the invocation is not the same act. | `_decomposition.md` Architecture §7, Testing §6; `project.md` DR-5, DR-9; `.redkiln/config.yaml:40`, `:55` |
| No literal position values anywhere in this story's tests | The specification permits gaps and this adapter produces them. Compare against positions the store assigned, never against `[1, 2, 3]`. The lint is a `REQUIRED` gate step, so this fails the build rather than a review. → AC-010 | `CLAUDE.md` (conformance section, CF-6); `xtask/src/main.rs:389`, `:687` |
| The error enum grows; `sqlx::Error::Database` is not flattened | New `#[non_exhaustive]` variants for the mechanism's decode failures (an unparseable `xid8`, an absent frontier row). No variant per `SQLSTATE` — `sqlx` already exposes the code. No `ConditionViolated` variant on either enum. → AC-009 | `.kb/decisions/0009-error-send-sync.md`; `crates/happenstance-postgres/src/error.rs:1-11`, `:29-58` |
| The `Send` flavour is what is implemented, and generic code still binds the bare one | `impl SendEventStore for PostgresEventStore` gives `EventStore` free, and the implication runs only that way. No `#[async_trait]`, ever — it injects `+ Send` and makes the `wasm32` target impossible. The two in-crate tests asserting the bare bound and `Send + Sync` stay and must still pass. → AC-012 | `CLAUDE.md` constraints 1 and 4; `.kb/decisions/0001-async-port-flavours.md`; `crates/happenstance-postgres/src/event_store.rs:118-121`, `:177-198` |
| ADR-0024 is not cited as prior art | No file under `.kb/decisions/` matches `0024-*`. The record this story writes is *what was wired and on whose prior authority* — ADR-0013's phase-2 measurement — leaving the choice, its structural bill and the re-measurement to the story that owns them. → AC-011 | `_decomposition.md` Architecture §Intent and §3 tension 1; `.kb/open-questions/postgres-arm-c-structural-cost.md` |
| Evidence is owed per criterion | `require_ledger: true`, and `require_commit_provenance: true` for any file changed inside the declared boundary. | `.redkiln/config.yaml:67`, `:73` |

## Data and migrations

**Real, and shared with the slice-mate that owns the file.** Migration 1 belongs
to `postgres-schema-and-live-fixture` — identity and time columns (`EventId`,
`recorded_at`, discharged at phase 4 and named so nobody re-derives them,
`RUNBOOK.md:249-253`), tag storage, and the column the mechanism adds. This story
consumes it and needs **two additions** to it:

- **The mechanism's column.** An `xid8` column beside each row, written from the
  append transaction and read by the frontier predicate. It is named in the
  dependency's one-liner as "the mechanism's column" (`_storymap.md:60`), so the
  coordination is expected; what this story owes is that the column the append
  path writes and the column the migration creates are the same column.
- **`origin_store` and `origin_position`**, which `contains_event_id` needs and
  the intended schema at `crates/happenstance-postgres/src/event_store.rs:9-20`
  does not have (`:163-174`).

**One migration, not two.** Both additions fold into migration 1 rather than
becoming a migration 2. Neither crate has ever shipped a schema to a real
consumer — both are `publish = false` today — so there is no prior deployment to
migrate data out of, and the deployment brief says so in terms: this is a
*schema-authoring* concern and **N/A for backfill**
(`_decomposition.md` Deployment brief, *Migration and backfill*). A second
migration here would be a permanent scar recording an intra-slice sequencing
accident.

**`position` is not `bigserial`, and that is the schema's load-bearing fact.** A
`serial` column is `nextval()`, and `nextval()` is where the invariant is lost —
a writer takes 99, a writer that started later takes 100 and commits first, and a
reader that conditioned on `after: Some(100)` never sees the 99 that would have
violated its boundary. There is no error and no rejected append; there is a read
model that is correct about a state the business forbids
(`crates/happenstance-postgres/src/event_store.rs:22-35`,
`crates/happenstance-postgres/src/lib.rs:14-21`).

**`sqlx` features are a deliberate manifest decision.** The crate ships with no
`migrate` and no `macros` feature, each with a stated reason —
compile-time query checking needs a live `DATABASE_URL` this crate does not have,
and "the schema is prose in `event_store.rs` until phase 10 makes it real"
(`crates/happenstance-postgres/Cargo.toml:18-22`). Phase 10 is now. Whether
`migrate` earns its place is Architecture §9.3's open question and the dependency
story's to answer; if this story changes a feature flag, it is recorded as a
decision in the ledger, not slipped into a manifest.

**Test-data isolation** — schema-per-fixture-instance versus
database-per-fixture-instance — is `postgres-schema-and-live-fixture`'s call
(Architecture §9.3), consumed here. What this story must not do is depend on an
empty database: the suite's rules run against a fixture that hands out isolated
backing stores, and a rule that only passes on a virgin schema is a rule that
passes for the wrong reason.

## Acceptance criteria

Twelve criteria. The "user" of this project is an **adapter author** and the
**consuming application** behind them (`_storymap.md:19-23`,
`.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md:114-181`),
so each criterion is written as that persona's goal crossing the full stack —
port impl → live server → conformance run → reported outcome — not as a bare
capability. Every verification names a real rule function or a real command.
`R:` abbreviates `crates/happenstance-testkit/src/suite.rs`.

| id | criterion | verification |
| -- | --------- | ------------ |
| AC-001 | **GIVEN** an adapter author running the suite against a live Postgres, **WHEN** `append` is called with zero events, **THEN** it returns `AppendError::NoEvents` *before* the append condition is evaluated and before any row, lock or sequence value is taken — so a zero-event call is never an expensive no-op — and the batch stays borrowed (`&[Event]`) rather than being collected to satisfy the signature. | `rules::append_rejects_empty_batch` (`R:2841`) and `rules::empty_batch_is_refused_before_the_condition_is_evaluated` (`R:2870`), reached through `event_store_conformance!` in `crates/happenstance-postgres/tests/` in the live-Postgres job. |
| AC-002 | **GIVEN** a consuming application whose business invariant is enforced by a conditional append, **WHEN** two such appends race against the same boundary on a store whose writers are *not* serialised, **THEN** exactly one wins and the loser gets `AppendError::ConditionViolated` carrying the conflicting position — because condition evaluation and insert happen on one snapshot inside one transaction, never as a probe followed by a write. | `rules::racing_conditional_appends_elect_one_winner` (`R:5325`), `rules::condition_rejection_leaves_store_unchanged` (`R:4910`) and the whole condition family (`R:4484` – `R:4910`); `rules::dropped_append_future_leaves_no_partial_batch` (`R:3140`); all inside `event_store_conformance!` in the live job. |
| AC-003 | **GIVEN** the adapter author's fear that the port quietly assumed their storage shape (`personas-and-journeys.md:152-181`), **WHEN** they run the suite against this store, **THEN** it passes with positions assigned so that allocation order is visibility order *without* serialising writers — `position` is not `bigserial`, `nextval()` is not on the append path, and no store-wide lock is held to commit. | `rules::nothing_below_an_observed_position_appears_later` (CF-13, `R:5880`), `rules::positions_are_unique` (`R:1583`), `rules::positions_are_strictly_monotonic` (`R:1614`) — plus an in-crate live test asserting the append path issues no `nextval` and takes no advisory or table lock (grep-level assertion over the executed SQL is acceptable; the throughput *measurement* is `adr-0024-position-visibility-mechanism`'s). |
| AC-004 | **GIVEN** an application author who has just observed position *P* from `append`, **WHEN** they call `head()`, **THEN** they get the **visibility frontier** — which may legitimately trail *P* — and not `max(position)`; on an empty store they get `None`; and nothing in the crate claims read-your-own-writes, because it does not hold. | `rules::head_is_the_highest_visible_position` (`R:1798` — asserts a *bound*, deliberately), `rules::head_of_an_empty_store_is_none` (`R:1731`), `rules::head_advances_across_two_handles` (`R:1855`); plus an in-crate live test that compares `head()` against the frontier query and against `max(position)` and fails if the two are conflated. |
| AC-005 | **GIVEN** a reader streaming a long result set while other writers commit underneath it, **WHEN** the stream is polled across several `FETCH`es, **THEN** no row appears beneath a position the reader has already yielded — because the frontier predicate was composed into the cursor's `DECLARE` inside one `BEGIN ISOLATION LEVEL REPEATABLE READ` transaction and evaluated once, never re-evaluated per chunk. | `rules::nothing_below_an_observed_position_appears_later` (`R:5880`) reached through the read path, `rules::read_from_a_gap_position` (`R:1490`); plus an in-crate live test that holds a `PgReadStream` open across an interleaved append-and-commit and asserts the yielded positions are non-decreasing with no late insertion. |
| AC-006 | **GIVEN** an operator asking "does this store already hold event *X*", **WHEN** `contains_event_id` is called, **THEN** it answers from real `origin_store` / `origin_position` columns rather than `todo!()`, and the frontier disagreement (a committed row above the frontier is invisible to `read`) is **answered deliberately and written down** in the method's rustdoc — which answer was chosen and why — with ES-41 left `[PROVISIONAL]` and replication semantics left to HS-P0017. | `rules::contains_event_id_reports_membership` (`R:2551`) and `rules::event_ids_are_unique_within_a_store` (`R:2092`) in the live job; the recorded answer verified by the `REQUIRED` rustdoc step of `cargo xtask ci` plus a human read of the method's docs. |
| AC-007 | **GIVEN** a sync runner that must distinguish "this will never fit here, park it" from "the disk is full, retry", **WHEN** it appends a payload, tag count or batch above this store's stated ceiling, **THEN** it gets `AppendError::ExceedsStoreLimit { limit: StoreLimit::… }` — never `AppendError::Store`, never a silent truncation — and, independently, the contract's guaranteed minima (65,536-byte payload, 128-event batch, and the tag and query minima) are **accepted**. | `rules::append_reports_exceeded_store_limits` (`R:4282`), `rules::store_accepts_the_guaranteed_minimum_payload` (`R:3775`), `rules::store_accepts_the_guaranteed_minimum_tag_count` (`R:3829`), `rules::store_accepts_the_guaranteed_minimum_batch_size` (`R:3954`), `rules::store_evaluates_a_query_at_the_guaranteed_minimum_item_count` (`R:3880`). |
| AC-008 | **GIVEN** an adapter author who needs "an executable definition of correct" rather than a prose specification to interpret (`personas-and-journeys.md:126-129`), **WHEN** the live-Postgres job runs, **THEN** `event_store_conformance!` and `event_store_model_conformance!` are both invoked from `crates/happenstance-postgres/tests/` in their **full macro expansion** and every rule reports pass, fail, or a skip carrying the fixture's stated reason — no rule is `#[cfg]`-ed out and none is absent from the run. | The two invocations in `crates/happenstance-postgres/tests/` (`crates/happenstance-testkit/src/lib.rs:312`, `crates/happenstance-testkit/src/model.rs:782`); the job command `cargo test -p happenstance-postgres --all-features -- --ignored --show-output` (`_decomposition.md` Testing §6), whose `--show-output` capture is the evidence that the rule count run matches the rule count the macros expand to. |
| AC-009 | **GIVEN** an adapter author debugging a failure at 2am, **WHEN** the mechanism's own decode paths fail (an `xid8` that will not parse, an absent frontier row, a `bigint` position that decoded as zero against `SequencePosition`'s `NonZeroU64`), **THEN** they meet a named `#[non_exhaustive]` variant on `PostgresEventStoreError` naming the cause — not a flattened `sqlx::Error::Database`, not a variant per `SQLSTATE`, and not a `ConditionViolated` variant on either enum, because a condition violation is not an adapter failure. | In-crate unit tests over the new variants' construction and `Display`; `cargo clippy --workspace --all-targets --all-features -D warnings` (`REQUIRED`); the condition family (`R:4484` – `R:4910`) proving a violation arrives as `AppendError::ConditionViolated`, and a static check that no `0024`-numbered or per-`SQLSTATE` variant was added (`crates/happenstance-postgres/src/error.rs:29-58`). |
| AC-010 | **GIVEN** that this is the first adapter in the portfolio whose positions genuinely have gaps, **WHEN** any test this story adds asserts about positions, **THEN** it compares against positions the store actually assigned and never against a literal (`[1, 2, 3]`) — so a legitimately gappy conformant store cannot fail a test for being conformant. | `cargo xtask lint-position-literals` — a `REQUIRED` gate step, so this fails the build rather than a review (`xtask/src/main.rs:389`, `:687`; `lints::no_position_literals`). |
| AC-011 | **GIVEN** a future reader auditing why this store works the way it does, **WHEN** they read the crate's module docs and this story's ledger, **THEN** they find *what was wired and on whose prior authority* — ADR-0013's phase-2 measurement, which rejected two serialising arms at 16×/30× throughput cost — and **no citation of ADR-0024**, which does not exist and whose choice, structural bill and re-measurement belong to the story that owns them. | Static: a repository-wide check that no file this story touches cites `ADR-0024` or a `.kb/decisions/0024-*` path (no such file exists — `ls .kb/decisions/` is the check); the `REQUIRED` `cargo doc` step plus a human read of the module rustdoc; ledger evidence citing `.kb/decisions/0013-position-assignment-and-visibility.md` and `experiments/position-visibility/README.md:13-19`. |
| AC-012 | **GIVEN** the `wasm32` / `!Send` half of the portfolio that this crate must not regress, **WHEN** this story lands, **THEN** `read` is still not `async` and still returns the stream at the top level, nothing (connection, transaction, cursor, frontier) is acquired until the first poll, `#[async_trait]` appears nowhere, and generic code can still bind the bare `EventStore` at this concrete store. | The two in-crate tests at `crates/happenstance-postgres/src/event_store.rs:177-198` (`send_flavour_satisfies_the_bare_bound`, `store_is_send_and_sync`) still green; a live test that constructs a stream and drops it unpolled, asserting no pool checkout occurred; `cargo xtask ci`'s four wasm32 steps and `cargo xtask affected --base main` green. |

**Coverage of the traced project ACs.** Project **AC-002** (the visibility rule
is one the adapter had to work to pass) ← story AC-003, AC-004, AC-005, AC-011 —
this story lands the *green half*; the `nextval()` control that proves the rule
would have rejected the naive arm is `postgres-rule-controls`', and project
AC-002 is not fully discharged until it lands. Project **AC-004** (Postgres
passes the whole suite for real) ← story AC-001, AC-002, AC-006, AC-008, AC-009,
AC-012 — again the *suite* half; the mutant control is `postgres-rule-controls`'.
Project **AC-010** (both stores declare their real limits) ← story AC-007, for
the Postgres side only; the Neon side is `neon-fixture-and-live-job`'s.

## Interaction quality

**Composition family — declared N/A, not silently skipped.** This story renders
no surface. `_design.md` records the no-surface determination for the whole
project and a human approved *that determination* at the `/redkiln:plan` design
sign-off gate (`_design.md:10-41`, `:86-95`), and `design.capture` is
deliberately absent from `.redkiln/config.yaml:75-81`, which makes the perceptual
review a declared skip. There is therefore no presentation, placement,
transience, density budget or hierarchy invariant to carry, and no design-named
anti-pattern to bind — inventing one here would be re-deciding a signed-off
determination.

**State family — it applies, in this repository's medium.** The RFC's state
invariants have exact analogues where the "surface" a user meets is a conformance
run, an error and a public method. Each is carried by a numbered row in the table
above, never by a bullet here, so each one has a ledger row and is gated.

| RFC state invariant | Analogue in this medium | Carried by | How it is verified |
| --- | --- | --- | --- |
| **Non-occlusion** — nothing hides the thing the user came for | No rule is absent from the run, and no failure is swallowed into an opaque `Store` error or a flattened driver error | **AC-008**, **AC-009**, **AC-007** | Full macro expansion with `--show-output` in the live job; the `ExceedsStoreLimit`-vs-`Store` rule; the named decode variants |
| **Preserved scroll / selection** — the user's place does not move under them | A reader's position in the stream cannot be undercut: the snapshot is fixed at `DECLARE` and does not advance between `FETCH`es | **AC-005**, **AC-004** | The interleaved-append live test; `nothing_below_an_observed_position_appears_later` through the read path |
| **In-place, not a context jump** — no hidden work happens off-screen | `read` acquires nothing until first poll and stays non-`async`; no eager frontier computation smuggled into `read` | **AC-012** | The drop-unpolled live test; the two in-crate bound tests |
| **Reversibility** — the user can undo what they just did | The mechanism is wired as *wired-on-prior-authority*, so ADR-0024 overturning this arm is recorded rework, not a defect; and one migration, not two, leaves no scar to unwind | **AC-011** | The no-`ADR-0024`-citation check; the module rustdoc read; **Data and migrations** above |
| **Reachability by one documented route** (keyboard-reachability's analogue) | The whole live suite is reachable by one command an adapter author can copy, and the default gate needs neither Docker nor network | **AC-008**, and NF-001 below | `cargo test -p happenstance-postgres --all-features -- --ignored --show-output`; `cargo xtask ci` green on a clean checkout |

## Error conditions

| id | condition | required behaviour |
| -- | --------- | ------------------ |
| **EC-001** | `append` called with zero events | `AppendError::NoEvents`, raised before condition evaluation and before any lock or sequence value is taken (`crates/happenstance-core/src/error.rs:219-225`). Carries AC-001. |
| **EC-002** | The append condition matches an event above its `after` boundary | `AppendError::ConditionViolated(..)` carrying the conflicting position — the DCB concurrency signal, not a store failure. Neither `PostgresEventStoreError` nor `AppendError` gains a `ConditionViolated` adapter variant (ADR-0009, `crates/happenstance-postgres/src/error.rs:1-11`). The store is left unchanged. Carries AC-002. |
| **EC-003** | A payload, tag count or batch exceeds the fixture's stated ceiling | `AppendError::ExceedsStoreLimit { limit, len }` with the *correct* `StoreLimit` discriminant and the offending length (`crates/happenstance-core/src/error.rs:227-244`, `crates/happenstance-core/src/limits.rs:54-77`). Never `AppendError::Store`; never truncation; nothing of the refused batch survives. Carries AC-007. |
| **EC-004** | The mechanism's own decode fails — an `xid8` that will not parse, or a frontier query returning no row where one is required | A new named `#[non_exhaustive]` variant on `PostgresEventStoreError`. Not `unwrap()`, not a panic across the trait boundary, not a flattened driver error. Carries AC-009. |
| **EC-005** | A `position` column decodes as `0`, which `bigint` admits and `SequencePosition`'s `NonZeroU64` does not | The existing decode variant already in `error.rs:29-58` — this is why those variants exist; do not add a second spelling of the same failure. Carries AC-009. |
| **EC-006** | A `sqlx::Error::Database` arrives carrying a `SQLSTATE` (unique violation, serialization failure, deadlock) | Surfaced through the existing driver-error variant with the code intact and reachable. **No variant per `SQLSTATE`** — `sqlx` already exposes the code and a per-code enum is a second, worse copy of it. Carries AC-009. |
| **EC-007** | The `append` future is dropped mid-flight | No partial batch survives; the transaction is not left open holding the frontier back. `rules::dropped_append_future_leaves_no_partial_batch` (`R:3140`). Carries AC-002. |
| **EC-008** | No Postgres server is reachable | The gated invocations do not run and `cargo test --workspace --all-features` exits zero — the failure mode is "not attempted", never "attempted and hung". A connection failure inside the live job is a *job* failure, not a conformance failure, and must not be reported as a rule outcome (`crates/happenstance-testkit/src/contract.rs:309-321` — `connect` panics rather than returning `Result` for exactly this reason). Carries NF-001. |

## Non-functional

| id | requirement | evidence |
| -- | ----------- | -------- |
| **NF-001** | The default gate stays Docker-free and network-free: `cargo xtask ci` and `cargo xtask ci --fast` are green on a clean checkout with no server and no credentials. The gating is **whole-invocation** (DR-5) — `#[cfg]`-ing a rule out of a macro expansion is the forbidden move and gating the invocation is a different act. | `_decomposition.md` Testing §6, Architecture §7; `project.md` DR-5, DR-9; `.redkiln/config.yaml:40`, `:55` |
| **NF-002** | Writers are not serialised. No store-wide lock, no single-row counter update, no advisory lock held to commit on the append path. The *number* is `adr-0024-position-visibility-mechanism`'s to measure; the *shape* is this story's to not destroy, because `postgres-concurrency-family` cannot pass honestly against a store that serialises. | `crates/happenstance-postgres/src/event_store.rs:43-53`; `experiments/position-visibility/README.md:13-19`; `RUNBOOK.md:685-702` |
| **NF-003** | Reads stay chunked and memory-bounded: the cursor is fetched in chunks and a large result set does not materialise in the adapter. Keyset pagination remains rejected. | `crates/happenstance-postgres/src/read_stream.rs:37-73`; `.kb/decisions/0011-read-laziness-and-isolation.md` |
| **NF-004** | No `#[async_trait]` anywhere, on any path this story touches. The `wasm32` steps of `cargo xtask ci` stay green, and `happenstance-neon`'s `!Send` story is untouched. | `CLAUDE.md` constraint 1; `.kb/decisions/0001-async-port-flavours.md` |
| **NF-005** | The MSRV floor does not move. If a dependency or feature flag this story enables raises the effective floor, that is an ADR-owed change (ADR-0029 amending ADR-0004) and therefore **out of this story** — record it and stop rather than raising it in silence. | `.kb/decisions/0029-msrv-raised-to-1-97-1.md`; `CLAUDE.md` binding constraint 5 |
| **NF-006** | The live job is deterministic enough to be a gate: fixture instances are isolated from each other (`rules::two_fixture_instances_observe_none_of_each_others_appends`, `R:210`) and no rule depends on an empty database, so a re-run on a reused server is not a different test. | `crates/happenstance-testkit/src/contract.rs:213-321`; **Data and migrations** above |

## Implementation notes (non-prescriptive)

These are observations, not instructions; the implementer's judgement wins where
they conflict with something the compiler or the server says.

**Sequence the mechanism before the methods.** The `xid8` column, the frontier
predicate and the decode path are one idea appearing in four places (`append`'s
insert, `head`'s query, the cursor's `DECLARE`, and `contains_event_id`). Writing
the predicate once as a named SQL fragment or a small private helper, and then
composing it, is what keeps the four from drifting — and drift here is silent:
three of the four would still pass most of the suite.

**Get `head` failing first.** `head_is_the_highest_visible_position` asserts a
bound, so a `max(position)` implementation *passes it*. The Red step that
actually bites is the in-crate live test AC-004 names — comparing `head()`
against both the frontier query and `max(position)` — plus CF-13 under
concurrent writers. Write that test before either body.

**One transaction, and let the shape be genuinely open.** Postgres has an
interactive transaction that Neon does not. Whether `append` reuses Neon's
single-statement CTE shape (`crates/happenstance-neon/src/lib.rs:46-77`, for
comparison only) or exploits `BEGIN`/`INSERT … RETURNING`/`COMMIT` is
Architecture §9.3's open question and the implementer's call — record which and
why in the ledger and the module docs. What is not open: the condition and the
insert see one snapshot.

**The `origin_store` / `origin_position` columns are a coordination, not a
negotiation.** They land in migration 1, which `postgres-schema-and-live-fixture`
owns and which is being written in the same slice context. Agree the column names
once and use them in both places; a second migration to add them later is the
scar **Data and migrations** forbids.

**Expect `event_store_model_conformance!` to need a manifest line.** It sits
behind the testkit's `proptest` feature and `not(target_arch = "wasm32")`
(`crates/happenstance-testkit/src/lib.rs:180-183`), so the dev-dependency almost
certainly needs the feature enabled. That is a deliberate manifest decision to be
recorded, not a flag to slip in.

**When a rule fails, suspect the rule last.** CLAUDE.md's standing instruction is
that a wrong rule gets fixed *and explained in the same change* — but
`happenstance-testkit` is a **must-not-change seam** for this project
(Architecture §1). So a rule that looks wrong here is a finding to record and
escalate, not an edit; the boundary block above will reject the diff anyway.

**The trap worth naming twice.** A serialised sequence table passes every
criterion in this table. It is the shape that makes the whole project pointless
(Context pack §3). If the honest way to green starts looking like
`UPDATE hs_sequence SET n = n + 1 RETURNING n`, stop and escalate.

## Tests and CI (merge gate)

Two lists, because the project carries a genuine split: the tree-local gate every
story runs on any machine, and the live job that gates this project
(`_decomposition.md` Testing §6, `project.md` DR-9). Getting green on the first
without the second satisfies nothing this story exists to satisfy.

| tier | command / path | proves |
| ---- | -------------- | ------ |
| **Static** | `cargo xtask ci` (fmt, clippy `-D warnings`, tests, four wasm32 steps, docs, `spec-trace`, `--no-default-features` doc build, `package-check`, `lint-position-literals`) | NF-001 (green with no Docker, no network), NF-004, AC-010, AC-009's clippy half, AC-011's rustdoc half |
| **Static (story grain)** | `cargo xtask affected --base main` — `.redkiln/config.yaml:40` | The story-grain bar redkiln itself runs: only what this diff could break |
| **Static (project grain)** | `cargo xtask ci --fast` — `.redkiln/config.yaml:55` | The non-terminal project's integration bar |
| **Unit (in-crate)** | `cargo test -p happenstance-postgres` — `crates/happenstance-postgres/src/event_store.rs:177-198` plus the new error-variant tests | AC-012's bound tests (`send_flavour_satisfies_the_bare_bound`, `store_is_send_and_sync`), AC-009's variant construction and `Display` |
| **Integration (live)** | `crates/happenstance-postgres/tests/` — the in-crate live tests AC-003, AC-004, AC-005 and AC-012 name (no `nextval`/lock on the append path; `head` vs frontier vs `max(position)`; the interleaved-append stream test; the drop-unpolled test) | The wrong implementations the conformance suite alone does not reject: `max(position)` as `head`, a per-`FETCH` predicate, an eager acquisition in `read` |
| **Conformance (live)** | `event_store_conformance!(mod_name = …, emit = …, fixture = PostgresFixture::…)` in `crates/happenstance-postgres/tests/` — `crates/happenstance-testkit/src/lib.rs:312` | AC-001, AC-002, AC-003, AC-004, AC-006, AC-007, AC-008 — the full expansion, never a subset |
| **Conformance / model (live)** | `event_store_model_conformance!` — `crates/happenstance-testkit/src/model.rs:782`, behind the testkit's `proptest` feature and `not(target_arch = "wasm32")` | AC-008's second family; the state-machine agreement no hand-written rule reaches |
| **CI job (live infrastructure)** | `cargo test -p happenstance-postgres --all-features -- --ignored --show-output` in the Postgres job — `testcontainers`, pinned Postgres minor, sibling of `gate` / `wasm-conformance` / `msrv` / `semver` / `advisories` (`_decomposition.md` Testing §6) | That the green above was earned against a real, pooled, networked, non-serialising server — and, via `--show-output`, that the rule count run equals the rule count the macros expand to (AC-008) |
| **Process** | `redkiln verify --grain story` (`require_ledger: true`, `require_commit_provenance: true` — `.redkiln/config.yaml:67`, `:73`) | Every AC-### row in `_ledger.md` flipped with cited evidence, and no file changed outside the PR boundary |

**Not run here.** `event_store_concurrency_conformance!`
(`crates/happenstance-testkit/src/concurrency.rs:1129`) is
`postgres-concurrency-family`'s; the `nextval()` negative control and the direct
`happenstance_testkit::rules::*` mutant run are `postgres-rule-controls`'; the
position-visibility re-measurement is `adr-0024-position-visibility-mechanism`'s
and is not part of any gate at any tier (`experiments/` is documented as "not in
the gate").

## Risks and coupling (PR-scoped)

| risk | why it bites here | mitigation inside this PR |
| ---- | ----------------- | ------------------------- |
| **The serialising shortcut passes.** Every criterion above is satisfiable by a store that takes one lock and holds it to commit. | It deletes the only axis this crate exists to occupy and makes `postgres-concurrency-family` unpassable honestly — but nothing in *this* story's test set fails. | NF-002 states the shape as a requirement; AC-003's in-crate test asserts no `nextval`, no store-wide lock on the append path; the trap is named twice in the spec on purpose. |
| **`max(position)` as `head`.** ES-30's rule asserts a bound, so the cheap body passes it. | ES-10 is `[FROZEN]`; a silent revert of it is the worst outcome this story can produce, and the suite will not necessarily say so. | AC-004's dedicated in-crate comparison test, written before the body (Implementation notes). |
| **Per-`FETCH` frontier evaluation.** The read half is where ES-10 is most easily lost, and a short test never notices. | The frontier advances between chunks; rows then appear beneath already-yielded positions. | AC-005's interleaved-append live test with a stream held open across a committed write. |
| **Citing ADR-0024.** It is the most natural sentence to write and there is no such atom. | It would launder a decision this story is forbidden to make into the record, and `adr-0024-position-visibility-mechanism` would then be re-deriving its own citation. | AC-011's static check plus the module-doc read; `_decomposition.md` Architecture §3 tension 1. |
| **Migration coupling with the slice-mate.** Two additions to a file another story owns, written in the same context. | A late `migration 2` becomes a permanent scar recording an intra-slice sequencing accident. | **Data and migrations** fixes "one migration, not two"; the migrations glob is inside the PR boundary so the coordination is visible in one diff. |
| **`contains_event_id` over-reaching into replication.** The frontier disagreement is a real, unsettled question and it is tempting to settle it. | HS-P0017 owns replication semantics; ES-41 is `[PROVISIONAL]` and must stay so. | AC-006 requires an answer *recorded in rustdoc*, explicitly not a clause amendment; the PR boundary excludes `spec/`. |
| **The live job flakes and gets `#[ignore]`d one rule at a time.** | That is DR-5's forbidden move arriving gradually rather than at once. | AC-008's rule-count check from `--show-output`; NF-006's isolation requirement; whole-invocation gating only. |
| **Dependency slip.** Everything here consumes `postgres-schema-and-live-fixture`'s fixture, gating and CI job. | Half of this story cannot be *run* without them, only compiled — and a `todo!()`-free body that never ran the suite is exactly the failure CLAUDE.md's "the rule that matters" names. | Same slice, implemented in one context, upstream in the merge order (`_storymap.md:246-250`). If that story is not landed, halt loudly rather than stubbing a fixture. |

## Dependencies

**Blocks on** — `postgres-schema-and-live-fixture`. It supplies migration 1
(identity, time, tag storage, and the mechanism's column), `PostgresFixture` and
`concurrency::ConcurrentFixture`, the whole-invocation gating mechanism, and the
live-Postgres CI job. All four are **consumed here, not rebuilt**. Same slice
(`postgres-live-suite`), one context, upstream in the merge order.

**Unlocks**

- `postgres-concurrency-family` — needs a non-serialising append path to have
  anything to contend on (project AC-003).
- `postgres-rule-controls` — needs a shipped mechanism to contrast the naive
  `nextval()` arm against, and a passing rule set to drive as the mutant control
  (project AC-002, AC-004).
- `adr-0024-position-visibility-mechanism` — needs a real adapter to re-measure,
  rather than four SQL strategies (project AC-001;
  `.kb/open-questions/postgres-arm-c-structural-cost.md`).
- `postgres-structural-bill` — needs the frontier `head` and the absent
  read-your-own-writes to exist before documenting them for a consumer (project
  AC-009).
- `neon-append-and-read-over-http` — the story map makes it depend on this one
  directly (`_storymap.md`, `neon-live-suite` row), sharing project AC-010's
  limits work across the two stores.
- `postgres-projection-store` and, downstream of everything,
  `deskeleton-and-package-readiness` and `far-end-discharge-record`.

## Anchors (progressive disclosure)

Everything load-bearing that is *not* in the Context pack, signposted and bound
to the criterion it serves. Link, open just in time; do not preload the corpus.

| anchor | why it is load-bearing | when to open | serves |
| ------ | ---------------------- | ------------ | ------ |
| `crates/happenstance-postgres/src/event_store.rs` | The mount point itself (`:121-175`), plus the intended schema (`:9-20`) and the two `nextval()` failure narratives (`:22-35`, `:43-53`) written by the person who left the `todo!()`s. Each `todo!()` carries the reason it is one. | First. Before writing any body. | AC-001 … AC-012 |
| `crates/happenstance-postgres/src/read_stream.rs` | The existing `REPEATABLE READ` + `DECLARE CURSOR` state machine (`:37-57`) and the recorded rejection of keyset pagination (`:59-73`). The frontier predicate goes *into* this `DECLARE`. | Before touching the read half. | AC-005, AC-012 |
| `.kb/decisions/0013-position-assignment-and-visibility.md` | The accepted decision this story wires — and the only authority it may cite for the arm. Also the source of ES-30's amended portable form. | Before implementing the mechanism; again when writing the ledger evidence for AC-011. | AC-003, AC-004, AC-011 |
| `experiments/position-visibility/README.md` | The phase-2 measurement (`:13-19`): four SQL strategies, two serialising arms rejected at 16×/30×, tag-keyed advisory locks rejected for buying an invariant ES-10 does not state. This is the "prior authority" AC-011 requires named. | When recording *why* this arm and not another. Not for re-measurement — that is another story's. | AC-003, AC-011 |
| `crates/happenstance-testkit/src/suite.rs` | Every rule this story is judged by, with its own documentation of the wrong implementation it rejects — `head_is_the_highest_visible_position` (`:1798`), `nothing_below_an_observed_position_appears_later` (`:5880`), `append_reports_exceeded_store_limits` (`:4282`), `empty_batch_is_refused_before_the_condition_is_evaluated` (`:2870`), `contains_event_id_reports_membership` (`:2551`). | Open the specific rule the moment a body is failing it — and read its doc comment before suspecting the rule. | AC-001 … AC-008 |
| `crates/happenstance-testkit/src/contract.rs` | `Fixture`, the three `Option<usize>` ceilings and why they are not `Capability` (`:213-279`), and why `connect` panics rather than returning `Result` (`:309-321`) — which is why "the server is down" must never enter the rule channel. | Before wiring the fixture into the macros, and when answering EC-008. | AC-007, AC-008, NF-006 |
| `crates/happenstance-core/src/error.rs` | `AppendError`'s exact variants and the rationale attached to each — `NoEvents` (`:219-225`) and the `ExceedsStoreLimit`-vs-`Store` distinction with the sync-runner consequence spelled out (`:227-244`). | Before writing any error mapping. | AC-001, AC-007, AC-009 |
| `crates/happenstance-core/src/limits.rs` | `StoreLimit`'s three discriminants and `guaranteed_minimum()` (`:22-77`) — the floors AC-007 requires *accepted*, separate from the ceilings AC-007 requires *refused*. | When implementing the limit checks; the two halves are independent and both are owed. | AC-007 |
| `spec/SPECIFICATION.md` | ES-10 in full (`:2816-2842`) — the `[FROZEN]` clause, plus the consequences paragraph naming the frontier `head`, the absence of read-your-own-writes and the measured staleness bound (0.688 ms idle, 4010.719 ms behind an unrelated five-second write). | Before implementing `head`; again before writing `contains_event_id`'s recorded answer (ES-41, `:4381`). | AC-004, AC-005, AC-006 |
| `crates/happenstance-postgres/src/error.rs` | `PostgresEventStoreError`'s `#[non_exhaustive]` shape and the existing decode variants that exist because `bigint` admits zero and `SequencePosition` is a `NonZeroU64` (`:29-58`), plus ADR-0009's no-`ConditionViolated` rule (`:1-11`). | Before adding a variant — the one you need may already be there (EC-005). | AC-009 |
| `.bklg/from-contract-to-published-library/postgres-and-neon-stores/_decomposition.md` | Architecture §2 (composition roots A and B), §4 (Postgres data flow), §7 (gate wiring and DR-5's one hard constraint), §9.3 (what is genuinely open and therefore the implementer's to record); Testing §6 (both gate command lists) and §7 (fixtures, and the one seam not to rebuild). | When a shape question feels undecided — §9.3 tells you whether it is genuinely open or already settled elsewhere. | AC-002, AC-008, NF-001 |
| `.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md` | Persona 2, the adapter author (`:114-181`): the goal ("an executable definition of correct"), the fear (a port that quietly assumed their storage shape), and the Marten precedent — the *why* behind every criterion's framing. | Before writing ledger evidence, and any time an AC starts reading like a bare capability. | AC-003, AC-008 |
| `RUNBOOK.md` | Phase 10 in full (`:4311-4390`), the ES-10 – ES-12 residual exposure and this phase's ownership of it (`:606`), and the four-adapters-one-shape statement this story exists to falsify (`:665-702`). | For orientation on what phase 10 owes overall, and to check a scope instinct against the plan of record. | AC-003, AC-011 |
| `.kb/open-questions/postgres-arm-c-structural-cost.md` | States plainly that the phase-2 experiment "measured four SQL strategies, not four implementations of `SendEventStore`" — the reason ADR-0024 comes *after* this story and not before. | The moment writing "per ADR-0024" feels natural. | AC-011 |
| `.kb/decisions/0011-read-laziness-and-isolation.md`, `.kb/decisions/0012-append-shape-and-preconditions.md`, `.kb/decisions/0001-async-port-flavours.md`, `.kb/decisions/0009-error-send-sync.md` | The four accepted atoms behind, respectively: read laziness and isolation; the borrowed batch and the empty-batch ordering; the two-flavour port and the non-`async` `read`; and the error split that forbids a `ConditionViolated` adapter variant. | One at a time, when the corresponding body is being written. Do not preload all four. | AC-005, AC-001, AC-012, AC-009 |

## Clarifications resolved during spec

1. **The AC set is exactly the twelve the front half decided** — AC-001 … AC-012,
   with no additions and no drops. The two "Behavior and interfaces" rows for
   ceilings and floors both arrow to AC-007 deliberately: they are independent
   rules but one criterion, because a store that refuses correctly and rejects the
   guaranteed minimum has not half-passed, it has failed.
2. **The Docker-free gate is a non-functional requirement here, not an AC.** The
   "Behavior and interfaces" row for it carries no `→ AC-###` arrow, and that is
   correct: project **AC-011** is `postgres-schema-and-live-fixture`'s
   (`_storymap.md` traces it there), and the gating *mechanism* arrives from that
   story. What this story owes is not to break it, which is NF-001's shape.
3. **Interaction quality's composition family is N/A by a signed-off
   determination**, not by omission. `_design.md:86-95` records a human approving
   the no-surface finding itself. The state family still applies and is mapped
   onto existing AC rows rather than given new ones, so every invariant that
   applies has a ledger row.
4. **Project AC-002 and AC-004 are only half-discharged here.** Both require a
   *control* — the `nextval()` arm shown to fail CF-13, and the rule set driven
   directly as the mutant control — and both controls are
   `postgres-rule-controls`'. This spec claims the green half and says so, rather
   than claiming a project AC its PR boundary forbids it from finishing.
5. **`contains_event_id`'s frontier answer is required, but the *choice* is the
   implementer's.** AC-006 mandates that an answer is picked, implemented and
   written down with its reason; it does not mandate which. Anything stronger
   would settle replication semantics that belong to HS-P0017, and anything
   weaker leaves a `todo!()` behind a public method.
6. **Whether `append` uses a CTE or an interactive transaction stays open**
   (Architecture §9.3). This spec requires only that the condition and the insert
   see one snapshot, and that whichever shape is chosen is recorded — a default
   taken silently is the failure, not either shape.
7. **The in-crate live tests are named as obligations, not as file names.**
   Neither `happenstance-postgres` nor `happenstance-neon` has a `tests/`
   directory today; the file layout is the slice's to settle in one context with
   `postgres-schema-and-live-fixture`. Each AC names what the test must prove and
   the directory it lives in, which is what the ledger can be checked against.
