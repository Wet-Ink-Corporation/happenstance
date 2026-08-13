---
item: HS-S0037
stage: spec
created: 2026-08-12T13:46:35.831Z
updated: 2026-08-12T13:46:35.831Z
template_sig: 87bbf1d0
rendered_sig: 3395b1a5
---

# Spec — append: preconditions, then one BEGIN IMMEDIATE transaction

## Scope lock

| Layer | Path |
| --- | --- |
| Initiative (gold source) | [`.bklg/from-contract-to-published-library/initiative.md`](../../initiative.md) |
| Initiative decomposition | [`.bklg/from-contract-to-published-library/_decomposition.md`](../../_decomposition.md) |
| Project | [`.bklg/from-contract-to-published-library/sqlite-durable-store/project.md`](../project.md) |
| This spec | `.bklg/from-contract-to-published-library/sqlite-durable-store/append-atomicity-and-store-limits/spec.md` |
| Key briefs | [`../_decomposition.md`](../_decomposition.md) — **architecture brief §6** (the write path, in full), **§2** (the Accepted atoms that bind), **§11** (what is deliberately the implementer's, including the ceilings' corridor), **§1** (the mount table), **§9** (CF-40 recorded, not settled); **testing brief §3** (what may not be doubled), **§4** (CF-33's no-watchdog rule restated for testing) |
| Signed-off design | [`../_design.md`](../_design.md) — **N/A, no user-facing surface**, approved 2026-08-12. It binds this story to no surface; what it does bind is that "no surface" is not licence to skip rustdoc on the public Rust items this story adds |
| Story map row | [`../_storymap.md`](../_storymap.md) — slice `durable-event-store`, archetype `capability`, `depends_on: schema-migration-and-identity`, `traces_to: AC-007, AC-009` |
| Discover | [`discover.md`](discover.md) — the signal ledger, the seven deferred questions this spec answers, and the named wrong implementation |
| Grounding | [`../_grounding.md`](../_grounding.md) — the Accepted atoms and code patterns this project was grounded against |
| Roadmap pointer | `RUNBOOK.md:4199-4201` (the real `append`); `RUNBOOK.md:4217-4219` (the read-then-write adapter this story exists to make impossible) |

## One-line PR slice

Replace `append`'s `todo!()` with preconditions-then-one-`BEGIN IMMEDIATE`: empty batch refused first, the three declared ceilings raised as `ExceedsStoreLimit`, every guard probed by `EXISTS` before a row of the batch is inserted, returning the caller's own last position.

## Executive summary

`SqliteEventStore::append` is `todo!("SQLite event store: append")`
(`crates/happenstance-sqlite/src/event_store.rs:218-224`). This PR gives it a body,
and the body is mostly *order*: three of its four clauses decide what happens
**before** any SQL runs, and the fourth decides that everything remaining happens
inside exactly one transaction.

**Delta, not restatement.** `project.md` AC-007 says the append is atomic and AC-009
says the limits are declared facts; the architecture brief §6 says which SQL shape
serves that. Neither says what a reviewer *looks at* to know it happened, and the
conformance suite cannot supply it on its own: every append rule in `suite.rs` reads
its result back **through the port**, and `read` is still `todo!()` until the
slice-mate lands. So this story mounts its own real target,
`crates/happenstance-sqlite/tests/append.rs`, which drives `append` through the port
against a real file and then inspects the outcome through a **second raw
`rusqlite::Connection`** — SQL, not the port. That is not a weaker instrument than
the suite, it is a different and in one respect stronger one: "nothing of the refused
value survives" is a claim about what is *in the file*, and a store whose `read` is
also wrong can satisfy a port-only round trip.

Three things this PR fixes that no artifact yet states as an obligation:

1. **The ceilings live on the adapter, not on the fixture.** They are facts about
   this store, so `SqliteEventStore` carries them as documented public constants and
   `append` enforces them; the fixture (slice-mate `sqlite-fixture-and-whole-suite`)
   *restates* them. AC-009's split then has no seam where a number can be declared in
   one place and enforced at another value.
2. **The batch ceiling and the insert statement are one decision.** At the
   specification's own floors a naive multi-row tag insert binds `128 × 64 × 3 =
   24,576` of SQLite's 32,766 bound parameters
   (`crates/happenstance-core/src/limits.rs:34-43`, asserted at `:113-122`). A
   declared `MAX_EVENTS_PER_BATCH` above 128 therefore **cannot** be served by one
   statement, and the fix is chunked inserts *inside the one transaction* — never a
   second transaction, and never a lower ceiling chosen by accident.
3. **`tag_cardinality` is maintained by `append`.** Migration 1 creates the table
   (`schema-migration-and-identity`); nothing yet writes to it, and the
   most-selective-tag probe the whole schema amendment exists for reads it. A table
   that is created and never updated is a silent regression to the serialising plan
   the amendment was made to avoid.

## Context pack

Read this section and you can start. Everything below the Integration contract is
either a boundary or a signposted anchor.

### The decision this story is downstream of

**ADR-0022 is spent here, not written here.** `adr-0022-append-condition-strategy`
(HS-S0035) chose the append-condition SQL strategy from the three candidates at
`crates/happenstance-sqlite/src/lib.rs:56-62` — `BEGIN IMMEDIATE` plus an `EXISTS`
probe, a conditional `INSERT ... SELECT ... WHERE NOT EXISTS`, or a
monotonic-position guard — with a measured number, and it chose the tag storage
(`:62-65`), the busy-timeout value, `synchronous` and the journal mode. Consume all
six. **If the chosen strategy cannot express a guard's `after` against the tag
layout ADR-0022 fixed, that is a finding for the ADR queue, not an improvisation
here** (`discover.md`, question 7).

`schema-migration-and-identity` (HS-S0036) is the hard predecessor, and what it hands
over is not only tables: the indexes the `EXISTS` probe seeks, `AUTOINCREMENT`, the
`UNIQUE(origin_store, origin_position)` pair, the persisted `StoreId` this store
mints `EventId`s from, and **the finite busy timeout**. That last one is the
dependency an implementer forgets: without it, `BEGIN IMMEDIATE` on a contended file
returns `SQLITE_BUSY` *immediately*, which becomes `AppendError::Store`, which the
concurrency family reports as `Attempt::Failed` — a red rule that is not about this
adapter's logic at all.

### The order is the specification

ADR-0012 (`.kb/decisions/0012-append-shape-and-preconditions.md`) fixes it and
`suite.rs` checks it. Four steps, and the first three are decisions made before any
transaction exists:

1. **Empty batch → `AppendError::NoEvents`, before any condition is looked at.** Not
   a stylistic ordering: `ConditionViolated`'s documented meaning is *rebuild the
   decision model and retry*, so a caller whose retry loop branches on
   `is_condition_violated` and receives it for an empty batch never terminates — an
   empty batch will still be empty next time. This is the rule whose first casualty
   was the reference implementation (`crates/happenstance-testkit/src/suite.rs:2854-2892`).
2. **The three declared ceilings → `AppendError::ExceedsStoreLimit { limit, len }`,
   never `AppendError::Store`, never a truncation** (ADR-0015,
   `crates/happenstance-core/src/error.rs:227-244`). The distinction is the whole
   reason the variant exists: a caller that cannot tell "this will never fit here,
   park it and tell a human" from "the disk is full, retry" has to guess, and a sync
   runner that guesses wrong drops an event permanently.
3. **One `BEGIN IMMEDIATE` transaction opens**, taking the write lock at the top and
   holding it to commit.
4. **Inside it: every guard probed, then — and only then — the batch inserted**, and
   the position of the caller's own last event returned.

### `AppendCondition` is a sequence of guards, and `after` is exclusive

`AppendCondition` holds a non-empty `Box<[Guard]>` behind `guards()`
(`crates/happenstance-core/src/append.rs:104-141`, `:181`). Each `Guard` is a
`query` and an `after: Option<SequencePosition>`, and the semantics are:

- the append is **rejected** if the store holds any event matching `query` at a
  position **strictly greater** than `after`;
- `after: None` checks the entire log;
- guards are conjunctive in the sense that matters here — *any* violated guard
  rejects the whole append.

**`after` is exclusive; `ReadOptions::from` is inclusive.** They are one field apart
in the port, and the skeleton already records conflating them as a real historical
bug (`crates/happenstance-sqlite/src/event_store.rs:313-323`). Do not "tidy" the two
senses into one.

**A batch is never evaluated against itself** (ES-21, `spec/SPECIFICATION.md:3516`;
rule `batch_is_not_evaluated_against_its_own_condition`,
`crates/happenstance-testkit/src/suite.rs:3040`). The probe therefore runs against the
state the store already held when `append` began — which, given step 4's ordering, is
free: no row of this batch exists yet when the probes run.

### `BEGIN IMMEDIATE`, and why `BEGIN DEFERRED` is the named mutant

The wrong implementation AC-007 names is a read-then-write adapter. Concretely in
SQLite that is `BEGIN DEFERRED`, or no transaction at all: probe, then insert. **It
passes the entire sequential suite forever** — with one writer there is no window.
It fails only when two writers decide from the same snapshot, and it fails wearing a
disguise: SQLite upgrades a deferred transaction's read lock at the first write, so
the loser gets `SQLITE_BUSY`/`SQLITE_BUSY_SNAPSHOT`, which surfaces as
`AppendError::Store` → `Attempt::Failed` rather than `AppendError::ConditionViolated`
→ `Attempt::Rejected`. The concurrency family distinguishes those two by
construction (`crates/happenstance-testkit/src/concurrency.rs:214-231`) and asserts
exactly one winner, so it catches the mutant within a handful of iterations —
`RUNBOOK.md:4217-4219`'s claim, which nothing in the workspace has ever demonstrated
against a real database.

`BEGIN IMMEDIATE` is what makes the probe and the insert **one decision**. That is
the entire content of AC-007, and everything else in this story is preconditions
around it.

**Carry-forward correction, found at discover and not this story's to discharge.**
`../_storymap.md` sends the `BEGIN DEFERRED` mutant to
`crates/happenstance-testkit/tests/mutation_coverage/mutants.rs`'s `REGISTRY`. That
is the wrong table and will not compile past its own meta-test: a store wrong only in
parallel fails **no** rule of the event-store family, and `mutant_registry_is_exhaustive`
rejects a row whose `fails` list is empty. It belongs in `racers.rs` / `RACERS` with a
rendezvous-closed window. Recorded here, discharged in
`model-family-and-mutant-pass-column` (HS-S0042) — see `discover.md`, *The wrong
implementation*.

### The ceilings are the adapter's own facts, and their corridor is narrow

ADR-0015 (`.kb/decisions/0015-validated-identifiers-and-store-limits.md`) mints
`ExceedsStoreLimit` with exactly three `StoreLimit` variants and states that a
capacity limit must never be enforced by a constructor and never in `Deserialize` —
rejecting at decode destroys the quarantine path a peer needs. For this adapter that
means: **checked inside `append`, before the transaction opens.**

What stating a number commits this store to (`crates/happenstance-testkit/src/contract.rs:237-247`;
`crates/happenstance-testkit/src/suite.rs:4272-4312`): the rule appends a value at
**exactly** the stated ceiling and requires it **accepted**, then one unit larger and
requires it refused as `ExceedsStoreLimit` with the matching `StoreLimit` variant —
and then checks the store to prove nothing of the refused value survives. The
at-the-ceiling append is the anchor; without it a store that refused *everything*
would satisfy every assertion.

The corridor, therefore, is squeezed from both ends:

| Limit | Floor (a MUST) | Ceiling pressure |
| --- | --- | --- |
| `EventDataLen` | 65,536 bytes — VT-21, `crates/happenstance-core/src/limits.rs:22` | the rule allocates ceiling **+ 1**, so SQLite's own ~1 GB makes the suite unrunnable |
| `TagsPerEvent` | 64 — VT-22, `limits.rs:27` | tags-per-event multiplies into the parameter budget below |
| `EventsPerBatch` | 128 — VT-24, `limits.rs:43` | the rule appends a batch of exactly this size and then one larger, twice per run |

Two units questions `discover.md` deferred and this spec answers:

- **The payload ceiling is measured on `Event::data`'s byte length**, not on an
  encoded row and not on the sum of `data` and `metadata`. The rule builds an event
  whose `data` is exactly `MAX_EVENT_DATA_LEN` bytes and requires acceptance
  (`suite.rs:4330-4346`); a ceiling stated in any other unit fails at the boundary in
  one direction or the other.
- **There is no aggregate-bytes ceiling.** `StoreLimit` has exactly three variants and
  none of them is "total batch bytes" (`limits.rs:45-51`). Inventing a fourth refusal
  and reporting it through one of the three would be amending ADR-0015 in passing; a
  batch that fits event-by-event and tag-by-tag and count-wise is accepted, and if it
  fails for SQLite's own reasons that is honestly `AppendError::Store`.

**CF-40's clause home stays open.** ADR-0015 both claims and disclaims the clause that
lets a fixture declare numeric limits, and the open question
(`.kb/open-questions/cf-40-fixture-limits-ownership.md:74-85`) names this phase as
what forces it. This story needs the **capability**, not the clause's home; the
recording and escalation are `sqlite-fixture-and-whole-suite`'s (`../_storymap.md`,
*Coverage*, AC-009). Settling it here in passing is an AC-009 failure.

### The parameter ceiling makes the batch ceiling a design decision

`crates/happenstance-core/src/limits.rs:34-43` does the multiplication once, and
`limits.rs:113-122` asserts it as a test so that raising either floor fails there
rather than in this adapter's write path: at the guaranteed floors, a multi-row tag
insert binds `128 × 64 × 3 = 24,576` parameters against SQLite's
`SQLITE_MAX_VARIABLE_NUMBER` of 32,766 — 23% headroom, and an adapter that binds one
extra parameter per tag is within a factor of 1.3 of the ceiling.

The consequence is direct: **any declared `MAX_EVENTS_PER_BATCH` above the floor
cannot be served by a single multi-row statement.** The batch is inserted in chunks
sized from the parameter budget, all inside the one `BEGIN IMMEDIATE` transaction.
Chunking the *statements* is fine; chunking the *transaction* would destroy
atomicity and is the second way to fail ES-18 in this PR.

### Return the caller's own last position — the store head is a different number

ES-19 (`spec/SPECIFICATION.md:3435`): `append` returns the position assigned to the
**last event of this batch**, in slice order, strictly ascending and **not
necessarily contiguous**. Rules: `append_returns_last_written_position`
(`suite.rs:2623`) and `batch_positions_follow_slice_order` (`:2953`).

Returning `head()` instead passes on a quiescent store and is wrong the moment a
second connection commits between this transaction's commit and the head query — and
this adapter's whole point is that a second connection exists.
`append_returns_last_written_position` reads the claim back through a second handle
where the fixture offers one, which is exactly what catches a position sourced from a
per-session cache.

`AUTOINCREMENT` permits gaps and no code and no test may assume `+1` (`CLAUDE.md`,
*never assert on literal position values*; the testkit's `GappedPositionStore` is a
*conformant* store that assigns in steps of seven from 4,096).

### Contention is a wait, not an error — and there is no watchdog

With `CONTENDERS` connections on one file, `BEGIN IMMEDIATE` on a busy database
returns `SQLITE_BUSY` immediately unless a busy handler is configured; the busy
timeout `schema-migration-and-identity` sets is what turns that into a wait. CF-33
forbids a **rule** reading a clock and says nothing about an adapter's own retry
policy — but there is **no watchdog anywhere in the suite** and there must not be
one (`crates/happenstance-testkit/src/concurrency.rs:24-43`), so an *unbounded*
handler converts a livelock into a hung CI job that names no rule. Finite and
generous.

The testing corollary is a hard prohibition on this PR: **no timeout, watchdog, sleep
or retry loop may be added around any conformance rule or any test in this story.**
If something hangs locally, that is evidence for ADR-0022's busy-timeout paragraph,
not a reason to reach for `#[timeout]` (testing brief §4).

### The persona-journey slice this realizes

The journey is *"Learn when you are finished"* — the adapter author's loop, from a
signature that type-checks to a suite that says pass or fail and names why
(`.bklg/from-contract-to-published-library/initiative.md:245-246`). This story is
the moment in that loop where the adapter **stops being able to lie about
atomicity**: the two ways to get `append` wrong are both sequentially
indistinguishable from correct, so a passing single-threaded run is not evidence and
the author needs the suite to be the thing that decides. It also serves the
application author's fear directly — a lost update through a conditional append that
was silently unconditional is exactly the production contract defect
`initiative.md:204-208` names.

### Boundaries the implementer will be tempted to cross

- **Do not implement `read`, `head` or `contains_event_id` to "test append properly".**
  They are `lazy-read-with-snapshot-ceiling`'s and this slice's later stories'. This
  target verifies through raw SQL precisely so the boundary holds.
- **Do not cache a head.** ADR-0013's corollary is already in the skeleton
  (`crates/happenstance-sqlite/src/event_store.rs:226-235`): one file backs several
  handles, so `append` must not update a cached field that a second connection would
  then contradict.
- **Do not delete `#![allow(clippy::todo)]`.** DR-01: it dies with the *last*
  `todo!()`, in `instrument-markers-removed-and-gate-green`, not before
  (`crates/happenstance-sqlite/src/lib.rs:76-80`).
- **Do not add a `StoreLimit` variant, a `MAX_*` constant to `happenstance-core`, or
  any public item to the contract crate.** VT-24 forbids the constant in terms, and
  architecture brief §5's rule — adapter-private, one implementor is not a spread —
  applies here as it does to query decomposition.
- **Do not enforce a ceiling by letting SQLite refuse the row.** Binding an oversized
  blob and mapping `SQLITE_TOOBIG` to `AppendError::Store` is the second mutant
  `discover.md` names, and it is invisible while the fixture declares no ceilings:
  the rule skips legitimately and the run stays green.

## Integration contract

- **Archetype**: `capability` — a user-observable slice through the adapter, from the
  port's `append` signature to bytes on disk, exercised by a real `cargo test` target.
- **Slice / milestone**: `durable-event-store`. Slice-mates, implemented in one
  context and mounted as one integrated surface: `schema-migration-and-identity`
  (predecessor), `lazy-read-with-snapshot-ceiling`, `wide-query-chunked-not-refused`,
  `sqlite-fixture-and-whole-suite`.
- **Mount point**: `crates/happenstance-sqlite/tests/append.rs` — a **new, real
  `cargo test` target**, run by `cargo test -p happenstance-sqlite` inside
  `cargo xtask ci --fast` and by `cargo xtask affected --base main`. It is this
  story's composition root because the slice's ultimate root,
  `crates/happenstance-sqlite/tests/conformance.rs`, cannot invoke
  `event_store_conformance!` yet: every append rule but two reads its result back
  through the port, and `read` is `todo!()` until the slice-mate lands
  (`crates/happenstance-testkit/src/suite.rs:2630`, `:2906`). This target is **not**
  retired when `conformance.rs` arrives — it asks the one question the conformance
  suite cannot: *what is in the file after a refusal*, read through a connection the
  port never touched. Two testkit rules need only `append` and are invoked here by
  name through `happenstance_testkit::rules`
  (`crates/happenstance-testkit/src/lib.rs:189`): `append_rejects_empty_batch`
  (`suite.rs:2841`) and `empty_batch_is_refused_before_the_condition_is_evaluated`
  (`suite.rs:2870`). Architecture brief AC-A01 is honoured in substance: reachable
  from `tests/`, never from a `mod` that only `cargo check` sees.
- **Wires into**:
  - `crates/happenstance-core/src/append.rs:104-141`, `:181` — `AppendCondition`,
    `Guard { query, after }`, `guards()`. The shape every probe is derived from.
  - `crates/happenstance-core/src/error.rs:214-249` — `AppendError`'s variants; this
    story is the first code in the workspace to *produce* `ExceedsStoreLimit` from a
    real database.
  - `crates/happenstance-core/src/limits.rs:22-51`, `:113-122` — the three floors,
    the `StoreLimit` enum, and the parameter arithmetic the batch ceiling is chosen
    against.
  - `crates/happenstance-core/src/event.rs:321-326` — `Event`'s four fields; `data`'s
    byte length is the unit of the payload ceiling and `tags` supplies both the
    `event_tag` rows and the tag-count ceiling.
  - `crates/happenstance-core/src/identity.rs:97-124` — `EventId::new(StoreId, SequencePosition)`;
    the origin pair every appended row carries, keyed on the `StoreId`
    `schema-migration-and-identity` persisted.
  - `crates/happenstance-sqlite/src/event_store.rs:83-101`, `:218-224` — the
    `Arc<Mutex<Connection>>` this store serialises its writers through, and the
    `todo!()` this PR replaces.
  - `crates/happenstance-sqlite/tests/shapes.rs` — the type-level guard that must
    still hold after `SqliteEventStore` gains constants and `append` gains a body:
    no `rusqlite` handle borrowing the connection may reach a field.
  - `crates/happenstance-testkit/src/contract.rs:237-279` — the three `MAX_*`
    associated constants the fixture will mirror from this story's adapter constants,
    and the doc that states what a number commits a store to.
- **Renders surfaces**: **none.** `../_design.md` records N/A — no user-facing
  surface — for this whole project, approved at the design sign-off gate. The public
  Rust items this story adds (three ceiling constants and `append`'s own rustdoc) sit
  inside that determination and each carries an `# Errors` section naming conditions
  rather than error types (`standards/rust/70-rustdoc-obligations.md`).
- **Conformance rule(s)**: this story **adds no conformance rule** — every rule it
  must satisfy already exists in `for_each_event_store_rule!`, so the
  literal-position bar and the mutant-registry obligation are both vacuous *for this
  PR* (the `BEGIN DEFERRED` racer is HS-S0042's). The rules it makes passable, by
  name in `crates/happenstance-testkit/src/suite.rs`: `append_rejects_empty_batch`
  (`:2841`), `empty_batch_is_refused_before_the_condition_is_evaluated` (`:2870`),
  `append_returns_last_written_position` (`:2623`), `append_is_atomic` (`:2651`),
  `batch_positions_follow_slice_order` (`:2953`), `batch_is_not_evaluated_against_its_own_condition`
  (`:3040`), `dropped_append_future_leaves_no_partial_batch` (`:3140`),
  `reissued_conditional_batch_lands_once` (`:3252`),
  `condition_without_after_rejects_any_match` (`:4484`),
  `condition_after_ignores_events_at_the_boundary` (`:4531`), `condition_matches_on_tags`
  (`:4607`), `condition_against_an_empty_store_admits_the_append` (`:4713`),
  `condition_rejection_leaves_store_unchanged` (`:4910`),
  `store_accepts_the_guaranteed_minimum_payload` (`:3775`),
  `store_accepts_the_guaranteed_minimum_tag_count` (`:3829`),
  `store_accepts_the_guaranteed_minimum_batch_size` (`:3954`) and
  `append_reports_exceeded_store_limits` (`:4282`). All but the first two stay
  unrunnable until `read` and the fixture land in the same slice.
- **Clause(s)**: implements, and amends none — **ES-18** atomicity
  (`spec/SPECIFICATION.md:3330`, `[FROZEN]`), **ES-19** the returned position
  (`:3435`, `[FROZEN]`), **ES-20** an empty batch is refused and refused first
  (`:3480`, `[FROZEN]`), **ES-21** a batch's own events are not evaluated against its
  own condition (`:3516`, `[FROZEN]`), **ES-25 – ES-28** condition semantics
  (`:3695`, `:3753`, `:3794`, `:3855`, all `[FROZEN]`), **VT-25** a capacity refusal
  is distinguishable from a store failure (`:1580`, `[FROZEN]`). It supplies the
  enforcement half of **VT-21 – VT-24** (`:1483`, `[PROVISIONAL]`) and of **CF-40**
  (`:7661`, `[PROVISIONAL]`) whose clause *home* it explicitly does not settle. **No
  `[FROZEN]` clause is changed and no marker is moved in this PR**; `EventStore`
  itself is frozen at `:371` and this story implements it.
- **Advances DoD scenario**: initiative **DoD 3** — *"The durable store passes the
  suite for real"* (`.bklg/from-contract-to-published-library/initiative.md:366-368`).
  This story is the half of DoD 3 that makes the concurrency case *meaningful*: with
  a probe-then-insert append, 64 contenders prove nothing, because the wrong
  implementation is green until they race. Project rows: **AC-007** and **AC-009**.

## PR boundary

```
crates/happenstance-sqlite/src/**
crates/happenstance-sqlite/tests/append.rs
.bklg/from-contract-to-published-library/sqlite-durable-store/append-atomicity-and-store-limits/**
```

**In this PR**

- A real `SendEventStore::append` body on `SqliteEventStore`: the two precondition
  gates, one `BEGIN IMMEDIATE` transaction, an `EXISTS` probe per `Guard`, the batch
  inserted in parameter-budget-sized chunks inside that one transaction, and the
  caller's own last position returned.
- Three documented public ceiling constants on `SqliteEventStore`, each above its
  VT floor and inside the corridor above, with rustdoc stating the unit and the fact
  that the number is a property of this adapter rather than a trade.
- Every appended row's `origin_store` / `origin_position` / `recorded_at`, its
  `event_tag` rows including the covering `event_type`, and the `tag_cardinality`
  maintenance that makes the most-selective-tag probe possible.
- `crates/happenstance-sqlite/tests/append.rs` — the new target: `append` driven
  through the port against a real temporary file, outcomes verified through a second
  raw `rusqlite::Connection`, plus the two testkit rules that need only `append`.
- `append`'s rustdoc: an `# Errors` section naming the conditions — empty batch, each
  of the three ceilings, a violated condition, and the driver — not the types.
- The module-doc paragraph on `crates/happenstance-sqlite/src/event_store.rs`
  describing the write path, updated to what the code now does, *only* where it
  concerns `append`.

**Explicitly not in this PR**

- `read`, `ReadCursor::fetch_page`, `head`, `contains_event_id` — slice-mates'
  (`lazy-read-with-snapshot-ceiling`, and the read path's later stories).
- `SqliteFixture`, `crates/happenstance-sqlite/tests/conformance.rs`, the capability
  declarations (`SECOND_HANDLE`, `REOPEN`, `MID_BATCH_FAULT`'s declension reason) and
  any `event_store_conformance!` invocation — `sqlite-fixture-and-whole-suite`.
- Migration 1, the pragmas, the busy-timeout value, the persisted `StoreId` —
  `schema-migration-and-identity`'s, consumed here.
- The `BEGIN DEFERRED` racer row and any edit to
  `crates/happenstance-testkit/tests/**` — `model-family-and-mutant-pass-column`.
- `CONTENDERS`, the runtime `Handle` seam, and anything in
  `crates/happenstance-testkit/src/concurrency.rs` — `concurrency-family-and-contender-count`.
- CF-40's clause home, ES-35's marker, CF-14's deferral, and any
  `spec/SPECIFICATION.md` edit or ADR authoring.
- `#![allow(clippy::todo)]`, `lib.rs`'s status banner, `Cargo.toml`'s stale
  description, `publish = false` — DR-01 and architecture brief §8.

The implementer **may** touch the composition-root and wiring files named in the
Integration contract to mount this slice — creating `tests/append.rs` and adjusting
`crates/happenstance-sqlite/src/event_store.rs`'s constructors where `append` needs
them. That is mounting, not scope drift.

**Merge DoD one-liner** — `cargo xtask affected --base main` green with
`crates/happenstance-sqlite/tests/append.rs` writing to a real file, every refusal
verified through a second raw connection to show nothing survived, the three ceilings
exact at both boundaries, no `todo!()` removed outside `append`, and no timeout,
sleep or retry loop anywhere in the diff.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **An empty batch is refused first** | `append(&[], Some(&c))` returns `AppendError::NoEvents` **even when `c` matches** — the emptiness check precedes condition evaluation. `ConditionViolated` here would make a conforming caller's retry loop non-terminating. | ES-20, `spec/SPECIFICATION.md:3480`; `crates/happenstance-testkit/src/suite.rs:2854-2892`; `.kb/decisions/0012-append-shape-and-preconditions.md` |
| **The three ceilings are checked before the transaction opens** | In order after the emptiness gate, each raising `AppendError::ExceedsStoreLimit { limit, len }` with the matching `StoreLimit` variant. Never `AppendError::Store`; never a truncation; never a partial batch. `len` carries the offending magnitude. | ADR-0015, `.kb/decisions/0015-validated-identifiers-and-store-limits.md`; `crates/happenstance-core/src/error.rs:227-244`; `../_decomposition.md` architecture brief §6 |
| **Each ceiling is exact at both ends** | A value at exactly the stated ceiling is **accepted**; one unit larger is refused; and after the refusal nothing of the refused value is in the file. The at-the-ceiling acceptance is the anchor — without it a store that refused everything would satisfy the rule. | `crates/happenstance-testkit/src/suite.rs:4272-4312`, `:4330-4360`; `crates/happenstance-testkit/src/contract.rs:237-252` |
| **The ceilings are adapter facts, stated as public constants** | `MAX_EVENT_DATA_LEN` (bytes of `Event::data`), `MAX_TAGS_PER_EVENT`, `MAX_EVENTS_PER_BATCH` on `SqliteEventStore`, each documented with its unit and its reason. The fixture mirrors these, so a number can never be declared at one value and enforced at another. | `crates/happenstance-testkit/src/contract.rs:253-279`; `../_decomposition.md` architecture brief §11 |
| **Each ceiling sits inside its corridor** | Strictly above VT-21's 65,536 bytes / VT-22's 64 tags / VT-24's 128 events, and low enough that the suite can allocate ceiling + 1 twice per run. SQLite's own ~1 GB is out of the corridor and would make the suite unrunnable. | `crates/happenstance-core/src/limits.rs:22-43`; `spec/SPECIFICATION.md:1483-1512`; architecture brief §11 |
| **No fourth kind of refusal** | `StoreLimit` has exactly three variants and none is "total batch bytes". A batch that passes all three ceilings and still fails is honestly `AppendError::Store`. | `crates/happenstance-core/src/limits.rs:45-51`; `discover.md`, question 5 |
| **One `BEGIN IMMEDIATE` transaction; probe and insert are one decision** | The write lock is taken at the top and held to commit. `BEGIN DEFERRED` or no transaction is the named wrong implementation: it passes every sequential rule and loses the race as `Attempt::Failed` rather than `Attempt::Rejected`. | `../_decomposition.md` architecture brief §6; `crates/happenstance-testkit/src/concurrency.rs:214-231`; `RUNBOOK.md:4217-4219` |
| **One `EXISTS` probe per guard, before any row of this batch exists** | Each `Guard` rejects on a match **strictly above** its `after`; `after: None` checks the whole log; any violated guard rejects the whole append. Whether the probes are one compound statement or one per guard is the implementer's. | `crates/happenstance-core/src/append.rs:121-141`, `:181`; ES-25 – ES-28, `spec/SPECIFICATION.md:3695-3882`; architecture brief §11 |
| **`after` is exclusive and stays that way** | Do not unify it with `ReadOptions::from`, which is inclusive. The two senses are one field apart in the port and the skeleton records conflating them as a real bug. | ES-26, `spec/SPECIFICATION.md:3753`; `crates/happenstance-sqlite/src/event_store.rs:313-323` |
| **A batch never conflicts with itself** | The condition is evaluated only against events the store already held when `append` began. Ordering makes this free: the probes run before any insert. | ES-21, `spec/SPECIFICATION.md:3516`; `crates/happenstance-testkit/src/suite.rs:3040` |
| **A rejected append leaves the file byte-identical** | `AppendError::ConditionViolated` (never `Store`), and the transaction rolls back with no partial rows in `event`, `event_tag` or `tag_cardinality`. Verified through a second raw connection, not through the port. | ES-18, `spec/SPECIFICATION.md:3330`; `crates/happenstance-testkit/src/suite.rs:2651`, `:4910` |
| **`append` returns the caller's own last position** | The position assigned to the last event of *this* batch, in slice order, strictly ascending, not necessarily contiguous. Never `head()`, which is a different number the moment a second connection commits. | ES-19, `spec/SPECIFICATION.md:3435`; `crates/happenstance-testkit/src/suite.rs:2623`, `:2953` |
| **Gaps are permitted; nothing assumes `+1`** | `AUTOINCREMENT` never reuses a position after a delete, and a conformant store may assign in steps of seven from 4,096. No code or test compares against a literal position. | `CLAUDE.md`, *never assert on literal position values*; `crates/happenstance-sqlite/src/event_store.rs:56-58`; `crates/happenstance-testkit/README.md:118-122` |
| **Every appended row carries its identity and its stamp** | `origin_store` = this store's persisted `StoreId`, `origin_position` = the row's own assigned position, `recorded_at` written **once** at append. The pair is `UNIQUE` together, which is both `contains_event_id`'s index and ingest's duplicate guard. | `crates/happenstance-core/src/identity.rs:97-124`; `crates/happenstance-sqlite/src/event_store.rs:237-244`; `.kb/decisions/0014-event-identity-and-recorded-time.md` |
| **The insert is chunked to the parameter budget, the transaction is not** | At the floors a multi-row tag insert binds 24,576 of SQLite's 32,766 parameters, so any batch ceiling above 128 needs several statements. All of them inside the one `BEGIN IMMEDIATE`. Chunking the transaction destroys atomicity. | `crates/happenstance-core/src/limits.rs:34-43`, `:113-122` |
| **`tag_cardinality` is maintained here** | Migration 1 creates it; `append` is what writes to it. A table created and never updated silently restores the serialising query plan the schema amendment exists to avoid. | `../_decomposition.md` architecture brief §7; `RUNBOOK.md:4178-4187` |
| **Contention waits; it does not surface as a store error** | The busy timeout the schema story configured is what makes `BEGIN IMMEDIATE` wait rather than return `SQLITE_BUSY` → `AppendError::Store` → `Attempt::Failed`. This story adds no retry loop of its own. | architecture brief §6; `crates/happenstance-testkit/src/concurrency.rs:24-43` |
| **No watchdog, timeout, sleep or retry around any test** | CF-33 forbids a rule reading a clock, and the suite has no watchdog by design. A hang is evidence about ADR-0022's busy-timeout paragraph. | testing brief §4; `crates/happenstance-testkit/src/concurrency.rs:24-43` |
| **`append`'s rustdoc names conditions, not types** | An `# Errors` section covering the empty batch, each of the three ceilings, a violated condition and the driver. | `standards/rust/70-rustdoc-obligations.md`; `crates/happenstance-core/src/error.rs:214-249` |
| **The shape guard still holds** | `SqliteEventStore: Send + Sync`, `SqliteReadStream: Send + Unpin`, `SqliteEventStoreError: Error + Send + Sync + 'static` after `append` gains a body and the store gains constants. No `Transaction`, `Statement` or `Rows` may reach a field. | `crates/happenstance-sqlite/tests/shapes.rs`; `crates/happenstance-sqlite/src/event_store.rs:248-271`; ADR-0001/ADR-0008 via `CLAUDE.md` constraint 3 |
| **The feature powerset still compiles clean** | Anything shared between the two feature-gated modules is gated on `any(feature = "event-store", feature = "projection-store")`, or `--no-default-features` builds it dead and `-D warnings` fails. | `crates/happenstance-sqlite/Cargo.toml:28-33`; `CLAUDE.md`, *Commands* |

## Data and migrations

**No schema change. This story writes rows into the schema `schema-migration-and-identity`
(HS-S0036) created, and it must not alter it.** If `append` cannot be written against
migration 1 as landed, that is a finding for HS-S0036 and the ADR queue — not an
`ALTER TABLE` here. There is no data to migrate: the crate has never written a byte.

**What one successful `append` writes, inside one transaction.**

| Object | What `append` does to it | The wrong shape it forbids |
| --- | --- | --- |
| `event` | One row per batch event, in slice order; `position` assigned by `AUTOINCREMENT`; `event_type`, `data`, `metadata`, the canonical sorted `tags` encoding, `origin_store`, `origin_position`, `recorded_at` | Assigning positions outside the transaction, or writing `recorded_at` anywhere but here — a stamp written on read or re-stamped on open is exactly what `recorded_time_survives_a_reopen` exists to reject |
| `event_tag` | One row per (tag, event), key `(tag, position)`, **`event_type` carried as the covering column** | Omitting the covering column, which turns a type-constrained `QueryItem` into a join back to `event` walked under the write lock |
| `tag_cardinality` | Incremented for each tag written | Leaving it at zero: the most-selective-tag probe then has no counts and falls back to the plan the schema amendment was made to avoid |
| `UNIQUE(origin_store, origin_position)` | Satisfied by construction — one store, one position per row | Relying on the constraint to *detect* a bug rather than to serve `contains_event_id`; a violation here is a real defect, not a retry |

**What a refused `append` writes: nothing.** Whether the refusal is a precondition
(`NoEvents`, `ExceedsStoreLimit`) or a violated guard (`ConditionViolated`), the file
is byte-identical afterwards. The precondition refusals never open a transaction at
all; the guard refusal rolls one back. `sqlite_sequence`'s counter is part of that
rollback and is not a position the caller ever saw — and gaps are permitted anyway,
so no test may assert on it.

**Statement chunking is not transaction chunking.** A batch larger than the parameter
budget allows is inserted with several statements inside the same `BEGIN IMMEDIATE`.
Committing between chunks would produce a partially-applied batch, which is the third
way to fail ES-18 in this PR and the one that would still pass a single-threaded read
back.

**Reversibility.** None is offered and none is owed: the crate is `publish = false`,
nothing outside this workspace consumes the file format, and the first published
version of `happenstance-sqlite` is `publication-and-positioning`'s decision
(`../project.md`, *Out of scope*).

## Acceptance criteria

Ten criteria, each written from the goal of a persona named at
`.bklg/from-contract-to-published-library/initiative.md:203-225` and each crossing
the whole stack — from the port's `append` signature to bytes in a real SQLite file,
read back through a connection the port never touched. The two personas that recur
are the **adapter author**, whose journey is *"Learn when you are finished"*
(`initiative.md:245-246`), and the **application author**, whose stated fear is a
contract defect reaching production through a second independent reader
(`initiative.md:204-208`).

Every criterion's verification names a real target. `tests/append.rs` below is
`crates/happenstance-sqlite/tests/append.rs`, this story's mount point; test names
inside it are this PR's to create.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | GIVEN an application author whose retry loop branches on `is_condition_violated` and rebuilds its decision model on every rejection, WHEN it calls `append` with an empty slice and a condition that a matching event in the store would violate, THEN it receives `AppendError::NoEvents` and not `ConditionViolated`, so the loop terminates instead of rebuilding a model that will produce the same empty batch forever. | `tests/append.rs::empty_batch_is_refused_before_any_condition_is_looked_at`, plus the two testkit rules invoked by name through `happenstance_testkit::rules` — `append_rejects_empty_batch` (`crates/happenstance-testkit/src/suite.rs:2841`) and `empty_batch_is_refused_before_the_condition_is_evaluated` (`:2870`) — both against a real file. |
| AC-002 | GIVEN an adapter author who must tell a sync runner apart a batch that will never fit in this store from a disk that is momentarily full, WHEN a batch exceeds any of the three ceilings this adapter declares, THEN `append` returns `AppendError::ExceedsStoreLimit { limit, len }` carrying the matching `StoreLimit` variant and the offending magnitude — never `AppendError::Store`, never a truncation — and a value at exactly the ceiling is still accepted. | `tests/append.rs::each_ceiling_is_exact_at_both_ends` — for each of `EventDataLen`, `TagsPerEvent`, `EventsPerBatch`: append at exactly the declared constant and assert acceptance, then at constant + 1 and assert the variant. Mirrors `crates/happenstance-testkit/src/suite.rs:4272-4360`, which the fixture story will run for real. |
| AC-003 | GIVEN an application author whose second, independent reader must never build an answer from a torn log, WHEN any `append` is refused — empty batch, any ceiling, or a violated guard — THEN a **second raw `rusqlite::Connection`** opened on the same file afterwards finds no row of the refused batch in `event`, `event_tag` or `tag_cardinality`, so the refusal cost the log nothing. | `tests/append.rs::a_refused_append_leaves_the_file_unchanged` — row counts and content in all three tables captured before and compared after, through a connection the store never held. Covers the port-blind claim `crates/happenstance-testkit/src/suite.rs:4910` makes through the port. |
| AC-004 | GIVEN an adapter author who cannot trust a green single-threaded run, WHEN two handles onto one file decide from the same state and both attempt a conditional append, THEN the loser is refused as `AppendError::ConditionViolated` rather than surfacing a driver error, because the guard probe and the insert happen inside one `BEGIN IMMEDIATE` transaction that takes the write lock at the top and holds it to commit. | `tests/append.rs::two_handles_racing_one_condition_yield_one_winner_and_one_rejection` — two `SqliteEventStore` handles on one file, the losing attempt asserted to be `ConditionViolated` and not `Store`, mirroring the `Attempt::Rejected` / `Attempt::Failed` split at `crates/happenstance-testkit/src/concurrency.rs:214-231`. The full family is `concurrency-family-and-contender-count`'s. |
| AC-005 | GIVEN an application author who modelled one consistency boundary as several guards, WHEN `append` evaluates an `AppendCondition`, THEN a guard is violated only by a match at a position **strictly greater** than its `after`, a guard whose `after` is `None` is violated by any match at all, any one violated guard refuses the whole append, and no event of the batch being written is ever evaluated against the batch's own condition. | `tests/append.rs::guard_after_is_exclusive_at_the_boundary`, `::a_guard_without_after_sees_the_whole_log`, `::any_violated_guard_refuses_the_whole_batch`, `::a_batch_never_conflicts_with_itself` — the last one appending a batch whose own events match its own condition and asserting acceptance (ES-21, `spec/SPECIFICATION.md:3516`). |
| AC-006 | GIVEN an application author who records the position their own write landed at, WHEN `append` succeeds while other connections are committing, THEN the returned `SequencePosition` is the one assigned to the last event of **this** batch in slice order — not the store head, and never assumed to be a predecessor plus one. | `tests/append.rs::append_returns_the_callers_own_last_position` — the returned value compared against the positions actually read back from `event` for this batch's rows through the raw connection, with a second handle committing in between; and `::batch_positions_follow_slice_order` asserting strict ascent without literal values (`CLAUDE.md`, *never assert on literal position values*). |
| AC-007 | GIVEN an application author whose second reader identifies an event across stores and whose sync runner must reject a duplicate on ingest, WHEN a batch is appended, THEN every row carries `origin_store` = this store's persisted `StoreId`, `origin_position` = its own assigned position and a `recorded_at` stamped exactly once here, its `event_tag` rows carry the covering `event_type`, and `tag_cardinality` is incremented for each tag written. | `tests/append.rs::every_appended_row_carries_its_identity_and_stamp` and `::tag_cardinality_is_maintained_by_append` — all assertions made through the raw second connection against the tables `schema-migration-and-identity` created, including a non-zero count per written tag. |
| AC-008 | GIVEN an adapter author who declared a batch ceiling above the specification's 128-event floor, WHEN a batch at exactly that ceiling is appended, THEN it lands whole in one transaction — the insert is split into parameter-budget-sized statements, the transaction is not split, and no partially-applied batch is ever observable. | `tests/append.rs::a_batch_at_the_declared_ceiling_lands_whole` — a full-ceiling batch with the maximum tags per event, verified row-for-row through the raw connection; and `::a_failure_mid_batch_leaves_nothing`, which forces a failure after the first chunk and asserts zero rows. Budget arithmetic is fixed at `crates/happenstance-core/src/limits.rs:34-43`, asserted at `:113-122`. |
| AC-009 | GIVEN an adapter author running a suite that has no watchdog anywhere in it by design, WHEN several connections contend for the write lock, THEN contention is a bounded **wait** rather than an `AppendError::Store`, because the finite busy timeout `schema-migration-and-identity` configured is consumed rather than replaced — and no timeout, watchdog, `sleep` or retry loop is introduced anywhere in this diff. | `tests/append.rs::contention_waits_rather_than_erroring` — a contended `BEGIN IMMEDIATE` asserted to succeed rather than return `Store`; plus a reviewer check that `rg -n "timeout\|sleep\|retry" crates/happenstance-sqlite/tests/ crates/happenstance-sqlite/src/` finds nothing added around a test (testing brief §4, `../_decomposition.md:691-696`; `crates/happenstance-testkit/src/concurrency.rs:24-43`). |
| AC-010 | GIVEN an evaluator reading this adapter's public surface in one sitting, WHEN they open `SqliteEventStore`, THEN the three ceilings are documented public constants stating their unit and that they are facts about this adapter, `append`'s rustdoc carries an `# Errors` section naming conditions rather than error types, `crates/happenstance-sqlite/tests/shapes.rs` still holds, the feature powerset still compiles clean — and nothing beyond this adapter has been decided: no `StoreLimit` variant added, no item added to `happenstance-core`, no `spec/SPECIFICATION.md` edit, no ADR authored, and CF-40's clause home still recorded as open. | `cargo test -p happenstance-sqlite --test shapes`; `cargo doc -p happenstance-sqlite` and `cargo clippy -D warnings` inside `cargo xtask affected --base main`; `cargo xtask ci --fast`'s feature-powerset step; and a reviewer diff check that `crates/happenstance-core/**`, `spec/SPECIFICATION.md`, `.kb/**` and `crates/happenstance-testkit/**` are untouched — `.kb/open-questions/cf-40-fixture-limits-ownership.md:74-85` unchanged. |

Traceability: project **AC-007** (atomic append, probe-then-insert rejected) is
carried by AC-003, AC-004, AC-005, AC-006 and AC-008; project **AC-009** (limits are
declared facts) is carried by AC-002, with AC-010 holding the half of it that says
CF-40 is not settled in passing. AC-001, AC-007 and AC-009 are the preconditions and
row-level obligations both project ACs rest on.

## Interaction quality

**Composition family: N/A, and the determination is signed off.** `../_design.md`
records *"N/A — no user-facing surface"* for this whole project, approved
2026-08-12, and states the reasoning it verified rather than assumed: `project.md`'s
AC-001 – AC-016 name no screen, route or view; `initiative.md:196` puts any screen,
UI or documentation site out of scope; and this project's briefs file carries an
architecture brief and a testing brief with no UX section. There is therefore no
presentation, placement, transience, density budget, hierarchy or named
anti-pattern for this story to honour, and inventing one would contradict a
signed-off design.

**What the signed-off design does bind here** is its own corollary, recorded in this
spec's scope lock: *"no surface" is not licence to skip rustdoc on the public Rust
items this story adds.* In a library the public API **is** the surface an evaluator
looks at, and it is carried as **AC-010** — a table row, not a bullet — verified by
`cargo doc`, by `-D warnings`, and by a reviewer reading the `# Errors` section
against `standards/rust/70-rustdoc-obligations.md`.

**State family, in the medium this story actually has.** The five state invariants
translate directly onto a fallible API call, and each is already carried by an
`AC-###` row above. This section only says which:

| State invariant | The library-medium reading | Carried by | Verified by |
| --- | --- | --- | --- |
| **In-place, not a context jump** | A refusal returns the caller to their own retry loop with enough information to act, rather than throwing them out of it. `ExceedsStoreLimit` says *park this and tell a human*; `ConditionViolated` says *rebuild and retry*; `NoEvents` says *your input was empty*. Collapsing any of the three into `Store` is the context jump. | AC-001, AC-002 | `tests/append.rs::empty_batch_is_refused_before_any_condition_is_looked_at`, `::each_ceiling_is_exact_at_both_ends` |
| **Non-occlusion** | A refusal never obscures what the store already held: no partial rows, no bumped counters visible to anyone, nothing that a later reader would have to distinguish from a real write. | AC-003 | `tests/append.rs::a_refused_append_leaves_the_file_unchanged`, read through a second raw connection |
| **Preserved selection** | The caller's own position is preserved and returned — the store head is a different number the moment another connection commits, and returning it silently substitutes someone else's write for the caller's. | AC-006 | `tests/append.rs::append_returns_the_callers_own_last_position` with a concurrent committer |
| **Reversibility** | Every refusal is a complete rollback, and a mid-batch failure inside a chunked insert rolls back all chunks: the unit of reversal is the batch, never the statement. | AC-003, AC-008 | `tests/append.rs::a_failure_mid_batch_leaves_nothing` |
| **Reachability** | Every behaviour above is reachable through the `EventStore` port alone, from a real `cargo test` target — no adapter-specific escape hatch, no `mod` that only `cargo check` sees (architecture brief AC-A01). | AC-004, AC-010 | `crates/happenstance-sqlite/tests/append.rs` run by `cargo test -p happenstance-sqlite`; `tests/shapes.rs` |

**The anti-pattern this story's medium does have** is the one named throughout the
context pack: an implementation that is *sequentially indistinguishable from
correct*. `BEGIN DEFERRED` probe-then-insert and a ceiling enforced by letting
SQLite refuse the row both render perfectly under every single-threaded assertion.
AC-004 and AC-002 exist to make each of them fail, which is why neither is stated
here as prose.

## Error conditions

| id | condition | required behaviour |
| --- | --- | --- |
| EC-001 | `append` called with an empty slice, with or without a condition | `AppendError::NoEvents`, decided before the condition is read and before any transaction opens (`crates/happenstance-core/src/error.rs:214-249`; ADR-0012) |
| EC-002 | Any event whose `data` byte length exceeds `SqliteEventStore::MAX_EVENT_DATA_LEN` | `AppendError::ExceedsStoreLimit { limit: StoreLimit::EventDataLen, len }`, `len` being the offending byte length. Measured on `Event::data` alone — not on an encoded row, not on `data + metadata` |
| EC-003 | Any event carrying more tags than `MAX_TAGS_PER_EVENT` | `ExceedsStoreLimit { limit: StoreLimit::TagsPerEvent, len }` |
| EC-004 | A batch longer than `MAX_EVENTS_PER_BATCH` | `ExceedsStoreLimit { limit: StoreLimit::EventsPerBatch, len }` |
| EC-005 | A batch that fits all three ceilings but that SQLite still refuses | `AppendError::Store`, honestly. There is no fourth `StoreLimit` variant and none may be invented; reporting an aggregate-bytes refusal through one of the three would amend ADR-0015 in passing (`crates/happenstance-core/src/limits.rs:45-51`) |
| EC-006 | Any guard matched at a position strictly above its `after` | `AppendError::ConditionViolated`, with a complete rollback (EC-009). Never `Store`, which is what the `BEGIN DEFERRED` mutant produces instead |
| EC-007 | `SQLITE_BUSY` returned after the configured busy timeout has genuinely elapsed | `AppendError::Store`. This is the honest outcome, and it is why the timeout must be finite and generous rather than absent (fast failure) or infinite (a hang no rule names) |
| EC-008 | `SQLITE_TOOBIG`, or any driver error raised by binding an oversized value | `AppendError::Store` — and reaching it for a value the ceilings should have refused is the second named mutant. The ceilings are enforced before the transaction opens precisely so this path is unreachable for oversized payloads (`discover.md`, *The second mutant*) |
| EC-009 | A failure at any point after `BEGIN IMMEDIATE` — a chunk, a tag insert, a `tag_cardinality` update, the commit | Roll back the whole transaction; no row of the batch survives in any of the three tables. `sqlite_sequence`'s counter rolling back is not a position any caller saw, and gaps are permitted anyway |
| EC-010 | `UNIQUE(origin_store, origin_position)` violated | A real defect in this adapter, surfaced as `AppendError::Store` and never retried or swallowed. The constraint exists to serve `contains_event_id` and ingest's duplicate guard, not to detect our own bug (`.kb/decisions/0014-event-identity-and-recorded-time.md`) |
| EC-011 | The store's `Mutex<Connection>` is poisoned by a panic in another thread | Surfaced as `AppendError::Store` rather than propagating a panic across the port boundary; a poisoned lock must not be silently recovered as though the interrupted work had completed (`crates/happenstance-sqlite/src/event_store.rs:83-101`) |

Every one of EC-001 – EC-006 is a condition named in `append`'s `# Errors` rustdoc,
by condition rather than by type (AC-010).

## Non-functional

| id | requirement | why it binds here |
| --- | --- | --- |
| NF-001 | The write lock is held for the probe, the inserts and the commit — and nothing else. No I/O, no allocation of the whole batch's encoded form, and no work that could be done before `BEGIN IMMEDIATE` is done inside it. | The concurrency family will run `CONTENDERS` writers against one file; lock hold time is the difference between a slow suite and a suite that appears to hang, and there is no watchdog to tell them apart (`crates/happenstance-testkit/src/concurrency.rs:24-43`) |
| NF-002 | The busy timeout is finite and generous. This story consumes the value `schema-migration-and-identity` set and ADR-0022 records; it neither hard-codes a second value nor adds a retry loop of its own. | An unbounded handler converts a livelock into a hung CI job that names no rule; a zero timeout converts contention into `AppendError::Store` and a red concurrency family that is not about append's logic |
| NF-003 | Memory is bounded by the chunk, not by the batch. Chunk size is derived from SQLite's parameter budget rather than from a literal, so raising a ceiling cannot silently exceed it. | `crates/happenstance-core/src/limits.rs:34-43` does the arithmetic once and `:113-122` asserts it, so raising a floor fails there rather than in this write path |
| NF-004 | No new dependency in `crates/happenstance-sqlite/Cargo.toml`. Temporary files, ordinals and chunking are written against what is already there. | The testing brief's fixture rule (`../_decomposition.md`, testing brief §3) and `cargo deny`'s standing in `cargo xtask ci` |
| NF-005 | No `#[async_trait]`, no added `Send` bound, and nothing that changes the port's two-flavour derivation. `SqliteEventStore` remains `Send + Sync` and `SqliteEventStoreError` remains `Error + Send + Sync + 'static`. | `CLAUDE.md` constraints 1 and 3 (ADR-0001, ADR-0008, ADR-0009); `crates/happenstance-sqlite/tests/shapes.rs` is the standing guard |
| NF-006 | The MSRV floor of 1.97.1 holds; nothing in this diff requires a newer toolchain feature. | `.kb/decisions/0029-msrv-raised-to-1-97-1.md` — the floor moves by ADR, never in silence |
| NF-007 | `cargo fmt` and `cargo clippy -D warnings` clean across the feature powerset, including `--no-default-features`. Anything shared between the two feature-gated modules is gated on `any(feature = "event-store", feature = "projection-store")`. | `crates/happenstance-sqlite/Cargo.toml:28-33`; the `cargo hack` step in `cargo xtask ci` |

## Implementation notes (non-prescriptive)

These are observations, not instructions. Where the spec is silent the choice is the
implementer's — the constraints above are what must hold however it is written.

- **Write the two precondition gates as plain early returns before any connection is
  locked.** They need no database at all, which is the point: ADR-0015's quarantine
  argument is that a caller must learn a value will never fit *here* without the
  store having to try.
- **Whether the guards are one compound `EXISTS` statement or one statement per guard
  is open by design** (architecture brief §11, `discover.md` question 1). One
  statement per guard is the obvious first cut and short-circuits on the first
  violation; a compound `EXISTS ... OR EXISTS ...` is one round trip. Either satisfies
  every AC; if the difference turns out to be measurable, that measurement belongs in
  ADR-0022's evidence, not in an unexplained choice here.
- **Derive the chunk size, do not pick it.** Something of the shape
  `params_per_row.max(1)` divided into a budget constant below 32,766 keeps the
  relationship between the batch ceiling and the statement visible to the next
  reader, and makes raising `MAX_EVENTS_PER_BATCH` a safe edit.
- **The ceilings want to be `pub const` associated items on `SqliteEventStore`**, so
  the fixture can name them rather than restate them. A number typed twice is the
  seam AC-009 exists to close.
- **`recorded_at` is stamped once, here.** ADR-0014's whole point is that the stamp is
  a property of the append, not of the read; re-stamping on reopen is the defect
  `recorded_time_survives_a_reopen` was written for and which nothing in the workspace
  has ever been able to fail.
- **A single `INSERT ... RETURNING position` per chunk** is one way to learn the
  assigned positions without a second query under the lock; so is `last_insert_rowid()`
  per row. Both are fine — what is not fine is inferring positions arithmetically,
  because `AUTOINCREMENT` permits gaps.
- **Resist implementing `read` to check your work.** The raw second connection in
  `tests/append.rs` exists so that the boundary with `lazy-read-with-snapshot-ceiling`
  holds, and it is the stronger instrument anyway: it can see rows a broken `read`
  would hide.
- **If ADR-0022's chosen strategy cannot express a guard's `after` against the tag
  layout it fixed, stop and record it** (`discover.md`, question 7). That is a finding
  for the ADR queue; improvising a different tag layout here would leave the ADR and
  the code disagreeing with nothing to detect it.

## Tests and CI (merge gate)

Grounded in the testing brief's tier table (`../_decomposition.md`, testing brief §1)
and its merge-gate command table (§6). No tier here is invented for this story, and
no test in it doubles `rusqlite`, the filesystem or the runtime (§3).

| tier | command / path | proves |
| --- | --- | --- |
| Static | `cargo xtask lints && cargo xtask spec-trace` (`reachability_static`, `.redkiln/config.yaml:48`) | `-D warnings` and `fmt` clean; the specification's citation count has not fallen — this story cites `[FROZEN]` clauses and changes none (AC-010, NF-007) |
| Static (review) | `rg -n "todo!" crates/happenstance-sqlite/src/` | `append`'s `todo!()` is gone and no other one is — `read`, `head` and `contains_event_id` are still marked, and `#![allow(clippy::todo)]` is still present (DR-01) |
| Static (review) | `rg -n "timeout\|sleep\|retry" crates/happenstance-sqlite/` over the diff | AC-009's hard prohibition: no watchdog, timeout, sleep or retry loop added around any test |
| Unit / type-level | `cargo test -p happenstance-sqlite --test shapes` (`crates/happenstance-sqlite/tests/shapes.rs`) | `SqliteEventStore: Send + Sync`, `SqliteReadStream: Send + Unpin`, the error bounds — still holding after the store gains constants and `append` gains a body. No `Transaction`, `Statement` or `Rows` reached a field (AC-010, NF-005) |
| Capability (this story's real proof) | `cargo test -p happenstance-sqlite --test append` (`crates/happenstance-sqlite/tests/append.rs`, new) | AC-001 – AC-009: `append` driven through the port against a real temporary file, every outcome verified through a second raw `rusqlite::Connection`. The refusal claims in particular are claims about the *file*, which no port-level round trip can make |
| Conformance (borrowed, two rules) | the same target, invoking `happenstance_testkit::rules::append_rejects_empty_batch` (`crates/happenstance-testkit/src/suite.rs:2841`) and `empty_batch_is_refused_before_the_condition_is_evaluated` (`:2870`) through `crates/happenstance-testkit/src/lib.rs:189` | AC-001 against the sibling's own contract rather than against a locally rewritten version of it. These are the only two append rules that do not read their result back through `read` |
| Conformance (whole suite) | `cargo test -p happenstance-sqlite --test conformance` — **not this story's**; `sqlite-fixture-and-whole-suite` mounts it | The seventeen rules this story makes passable (Integration contract, *Conformance rule(s)*) become runnable only when `read` and the fixture land in the same slice. Listed so their absence here is a stated boundary rather than a gap |
| Concurrency family | `event_store_concurrency_conformance!` — **not this story's**; `concurrency-family-and-contender-count` mounts it | The `BEGIN DEFERRED` mutant's rejection at scale. AC-004 carries the two-handle case that makes the claim locally falsifiable in the meantime |
| Mutation coverage | `crates/happenstance-testkit/tests/mutation_coverage/racers.rs` — **not this story's**; `model-family-and-mutant-pass-column` (HS-S0042) | That the rule *can* fail. Recorded here because this story is where the storymap's `mutants.rs` / `REGISTRY` destination was found to be wrong (context pack, *Carry-forward correction*) |
| Story merge gate | `cargo xtask affected --base main` (`affected_gate`, `.redkiln/config.yaml:40`) | The whole of the above that this diff can break, package-scoped — the command redkiln wires to the story grain |
| Project integration grain | `cargo xtask ci --fast` (`integration_scoped`, `.redkiln/config.yaml:55`) | The bar this non-terminal project meets, including the feature powerset (NF-007). Green here is a precondition for looking at the ledger, never a substitute for it (`RUNBOOK.md:38-42`) |

**Ledger obligation.** `require_ledger: true` (`.redkiln/config.yaml:67`) — every
AC-001 – AC-010 row in `_ledger.md` carries cited evidence before this story leaves
`implement`. AC-002's evidence should cite the *run output* for the ceiling assertions
rather than only a green binary, because the failure mode AC-009 of the project names
is a rule that skips while everything stays green (`discover.md`, *The second mutant*).

## Risks and coupling (PR-scoped)

| risk | likelihood / impact | containment inside this PR |
| --- | --- | --- |
| **`schema-migration-and-identity` lands without a busy timeout, or with a zero one.** `BEGIN IMMEDIATE` then returns `SQLITE_BUSY` immediately, contention becomes `AppendError::Store`, and AC-004 fails while looking like an atomicity bug. | Medium / High | AC-004 and AC-009 both assert on the *variant*, so the symptom is diagnosed rather than merely observed. The fix is in the predecessor's PR, not a retry loop here (NF-002) |
| **The lock is held too long and the concurrency family appears to hang.** There is no watchdog anywhere in the suite by design, so a livelock is a CI job that times out naming no rule. | Medium / High | NF-001 keeps everything possible outside `BEGIN IMMEDIATE`; AC-009 forbids papering over it. A hang is evidence for ADR-0022's busy-timeout paragraph (testing brief §4) |
| **A ceiling is chosen outside its corridor.** Too low fails the VT-21 – VT-24 floors; too high (SQLite's own ~1 GB) makes the suite unrunnable, because the rule allocates ceiling + 1 twice per run. | Medium / Medium | The corridor table in the context pack fixes both ends; AC-002 asserts at both boundaries locally, before the fixture story ever runs the rule |
| **A number is declared in the fixture at one value and enforced in `append` at another.** The rule then fails at a boundary for a reason that looks like a logic bug. | Medium / High | AC-010 requires the ceilings to be public constants on `SqliteEventStore`; the fixture story mirrors them rather than restating them (`crates/happenstance-testkit/src/contract.rs:253-279`) |
| **The batch ceiling is chosen above 128 and served by one statement.** It works until a full batch with many tags exceeds 32,766 bound parameters, then fails as a driver error at exactly the size the fixture declared as acceptable. | Medium / High | AC-008 appends a full-ceiling batch with maximum tags per event as a normal test, so the parameter budget is exercised every run rather than at the boundary someone else finds |
| **Chunking the transaction instead of the statements.** A committed partial batch passes a single-threaded read-back and fails ES-18. | Low / High | AC-008's `::a_failure_mid_batch_leaves_nothing` forces a failure after the first chunk; AC-003 verifies through the raw connection |
| **`after` quietly unified with `ReadOptions::from`.** One is exclusive, the other inclusive, and they are one field apart in the port. | Medium / Medium | AC-005 asserts the boundary case specifically — an event exactly at `after` must not violate the guard (`crates/happenstance-sqlite/src/event_store.rs:313-323` records this as a real historical bug) |
| **Scope creep into `read` to verify the work.** The natural instinct, and it would collide with `lazy-read-with-snapshot-ceiling` in the same slice. | High / Medium | The mount point is designed around it: verification is through a second raw `rusqlite::Connection`, and the PR boundary names `read` as out |
| **CF-40 settled in passing** by writing the fixture-limits clause home into this PR. | Low / High | Explicitly an AC-009 failure at project grain; AC-010 makes "`.kb/**` and `spec/SPECIFICATION.md` untouched" a checked criterion (`.kb/open-questions/cf-40-fixture-limits-ownership.md`) |
| **ADR-0022 not yet accepted when this story starts.** Its six decisions — strategy, tag storage, busy timeout, `synchronous`, journal mode, contender count — are consumed here, not made here. | Medium / High | `adr-0022-append-condition-strategy` (HS-S0035) precedes this slice in the merge order (`../_storymap.md`, *Merge order*). If a decision is missing, record the gap and escalate rather than improvising (`discover.md`, question 7) |

**Coupling, stated plainly.** This PR touches `crates/happenstance-sqlite/src/**` and
adds one test target. It touches no sibling crate, so nothing outside
`happenstance-sqlite` can regress from it — which is also why
`cargo xtask affected --base main` is a sufficient story gate and
`cargo xtask ci --fast` is the project's, not this story's.

## Dependencies

**Blocks on**

- **`schema-migration-and-identity` (HS-S0036)** — hard. It supplies the `event`,
  `event_tag` and `tag_cardinality` tables, the indexes the `EXISTS` probe seeks,
  `AUTOINCREMENT`, the `UNIQUE(origin_store, origin_position)` pair, the persisted
  `StoreId` this store mints `EventId`s from, and the finite busy timeout without
  which `BEGIN IMMEDIATE` on a contended file fails instantly. `append` cannot be
  written against a schema that does not exist, and it must not alter the one that
  does (*Data and migrations*).
- **`adr-0022-append-condition-strategy` (HS-S0035)** — sequencing, not a `depends_on`
  edge in the item's own frontmatter, and named here because the six decisions this
  story consumes are all its (`../_storymap.md`, *Merge order*). This story spends
  ADR-0022; it does not write it.

**Unlocks**

- **`sqlite-fixture-and-whole-suite` (HS-S0040)** — directly. The fixture declares the
  three ceilings by mirroring the constants this story adds, and
  `append_reports_exceeded_store_limits` cannot run against a `todo!()`.
- **`concurrency-family-and-contender-count`** — the family is meaningless against a
  probe-then-insert append; this story is what makes the race a real question.
- **`model-family-and-mutant-pass-column` (HS-S0042)** — inherits the carry-forward
  correction recorded in the context pack: the `BEGIN DEFERRED` negative control
  belongs in `crates/happenstance-testkit/tests/mutation_coverage/racers.rs`'s
  `RACERS`, not in `mutants.rs`'s `REGISTRY`.

**Parallel, same slice, no edge either way** — `lazy-read-with-snapshot-ceiling` and
`wide-query-chunked-not-refused`. They are implemented in the same context and mounted
as one integrated surface; neither is a precondition of this story and this story is
not a precondition of them.

## Anchors (progressive disclosure)

Every path below exists in this worktree. Open them at the moment named — not before,
and not instead of the context pack, which is sufficient to start.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `.kb/decisions/0012-append-shape-and-preconditions.md` | The Accepted atom that fixes the precondition order and states that a condition is evaluated only against events the store already held. The ordering is a rule, not a style. | Before writing the first line of `append` | AC-001, AC-005 |
| `crates/happenstance-testkit/src/suite.rs:2841-2892` | The two rules this story invokes by name, and the reference implementation's own first casualty — the empty-batch-with-a-matching-condition case, written out. | While writing AC-001's test | AC-001 |
| `.kb/decisions/0015-validated-identifiers-and-store-limits.md` | Mints `ExceedsStoreLimit` with exactly three variants and forbids enforcing a capacity limit in a constructor or in `Deserialize`. It is why the ceilings are checked inside `append`, and why there is no fourth kind of refusal. | Before choosing where the ceiling checks live | AC-002, AC-010 |
| `crates/happenstance-testkit/src/contract.rs:237-279` | States, in the sibling's own words, what declaring a number commits a store to — at the ceiling accepted, one unit larger refused with the matching variant, nothing of the refused value surviving. The fixture will mirror the constants this story adds. | Before choosing the three numbers | AC-002 |
| `crates/happenstance-testkit/src/suite.rs:4272-4360` | The rule body itself: the at-the-ceiling append that is the anchor, and the ceiling + 1 allocation that puts an upper bound on how large a ceiling may be. | When the corridor's upper end feels arbitrary | AC-002 |
| `crates/happenstance-core/src/limits.rs:22-51`, `:113-122` | The three floors, the `StoreLimit` enum, and the parameter arithmetic done once and asserted as a test — `128 × 64 × 3 = 24,576` of 32,766. This is what makes the batch ceiling a design decision rather than a preference. | Before writing the insert statement | AC-002, AC-008, NF-003 |
| `crates/happenstance-core/src/append.rs:104-141`, `:181` | `AppendCondition`, `Guard { query, after }` and `guards()` — the exact shape every `EXISTS` probe is derived from, including that the sequence is non-empty. | While writing the probe | AC-005 |
| `crates/happenstance-sqlite/src/event_store.rs:218-244`, `:313-323` | The `todo!()` this PR replaces, the skeleton's own note that `append` must not update a cached head, and the recorded historical bug of conflating exclusive `after` with inclusive `ReadOptions::from`. | Immediately — it is the file being edited | AC-005, AC-006, AC-007 |
| `crates/happenstance-core/src/identity.rs:97-124` | `EventId::new(StoreId, SequencePosition)` — the origin pair every appended row carries and the reason `UNIQUE(origin_store, origin_position)` exists. | While writing the row insert | AC-007 |
| `.kb/decisions/0014-event-identity-and-recorded-time.md` | Why `recorded_at` is stamped exactly once at append and read back rather than re-derived. The rule that depends on it has had no negative control since phase 4. | While deciding where the timestamp comes from | AC-007 |
| `crates/happenstance-testkit/src/concurrency.rs:24-43`, `:214-231` | CF-33's no-watchdog paragraph, and the `Attempt::Rejected` / `Attempt::Failed` split that is the entire mechanism distinguishing a correct append from the `BEGIN DEFERRED` mutant. | Before AC-004's test, and again if anything hangs | AC-004, AC-009, NF-001 |
| `crates/happenstance-testkit/tests/mutation_coverage/racers.rs` | Where the `BEGIN DEFERRED` negative control actually belongs, and the rendezvous discipline it must use — an atomic counter and bounded `yield_now`, never `thread::sleep`. Confirms the carry-forward correction rather than taking this spec's word for it. | Only if tempted to add the mutant row in this PR (it is HS-S0042's) | AC-004 |
| `crates/happenstance-testkit/README.md:108-122` | `mutant_registry_is_exhaustive`'s rule about an empty `fails` list, and `GappedPositionStore` — a *conformant* store assigning positions in steps of seven from 4,096, which is what fails any `+1` assumption. | Before writing any position assertion | AC-006 |
| `spec/SPECIFICATION.md:3330-3560`, `:3695-3882`, `:1483-1600` | ES-18 – ES-21, ES-25 – ES-28 and VT-21 – VT-25 in their normative wording, each with its maturity marker and the wrong implementation it forbids. The clause wins wherever a summary here disagrees. | When an AC's exact boundary is in doubt | AC-002 – AC-006 |
| `../_decomposition.md` (architecture brief §6, §7, §11; testing brief §3, §4) | The write path in full, the `tag_cardinality` rationale, the ceilings' corridor as the brief states it, what may not be doubled, and the no-watchdog rule restated for testing. | Before starting, and again before adding any test helper | all |
| `.kb/open-questions/cf-40-fixture-limits-ownership.md` | The open question this story must *not* close, and the atom's own statement that it has no standing to resolve it. Read it to confirm it stays untouched. | At review, before ticking AC-010 | AC-010 |
| `standards/rust/70-rustdoc-obligations.md` | The `# Errors` obligation — name conditions, not types — and the rest of the rustdoc bar the three new public constants must clear. | While writing the constants' and `append`'s docs | AC-010 |
| `crates/happenstance-sqlite/tests/shapes.rs` | The type-level guard that must still hold once the store gains constants and `append` gains a body: no `rusqlite` handle borrowing the connection may reach a field. | Before the first commit, and after any struct field is added | AC-010, NF-005 |
| `RUNBOOK.md:4199-4201`, `:4217-4222` | The plan-of-record entry for the real `append`, and the read-then-write claim this story exists to make demonstrable — a claim nothing in the workspace has ever tested against a real database. | For orientation on why this story exists | AC-004 |
| `discover.md` | The signal ledger, the seven questions and their disposition, and both named mutants written out at length. | If any decision here seems unmotivated | all |

## Clarifications resolved during spec

1. **The AC count is ten, as the first pass enumerated** — AC-001 through AC-010,
   no additions and no drops. The ledger carries exactly these ten ids.
2. **Discover question 1 — one compound `EXISTS` or one per guard — stays open, and
   deliberately.** Architecture brief §11 leaves it to the compiler and a
   measurement, and no AC can distinguish the two. Fixed instead is the observable:
   every guard evaluated against the state the store already held, inside the
   transaction that inserts, before any row of this batch exists (AC-005).
3. **Discover question 2 — the ceilings' numbers — resolve to a corridor, not to
   three literals.** The corridor is fixed in the context pack: strictly above
   65,536 bytes / 64 tags / 128 events, and low enough that the suite can allocate
   ceiling + 1 twice per run. Choosing the literals is the implementer's, because the
   numbers are facts about this adapter that the fixture then mirrors. AC-002 holds
   whatever they are.
4. **Discover question 4 — the payload ceiling's unit — is settled: `Event::data`'s
   byte length.** Not an encoded row, not `data + metadata`. The rule builds an event
   whose `data` is exactly `MAX_EVENT_DATA_LEN` bytes and requires acceptance
   (`crates/happenstance-testkit/src/suite.rs:4330-4346`); any other unit fails at the
   boundary in one direction or the other.
5. **Discover question 5 — an aggregate-bytes refusal — is settled: there is none.**
   `StoreLimit` has exactly three variants (`crates/happenstance-core/src/limits.rs:45-51`)
   and inventing a fourth, or reporting an aggregate refusal through one of the three,
   would amend ADR-0015 in passing. A batch that fits event-by-event, tag-by-tag and
   count-wise is accepted; if SQLite then refuses it, that is honestly
   `AppendError::Store` (EC-005).
6. **Discover questions 3, 6 and 7 stay where discover put them.** The busy-timeout
   value is ADR-0022's and is consumed (NF-002); CF-40's clause home is recorded open
   and is an AC-010 criterion *not* to settle; and a strategy that cannot express a
   guard's `after` against ADR-0022's tag layout is a finding for the ADR queue.
7. **The composition family of RFC §6.7 is N/A on a signed-off design**, not skipped.
   `../_design.md` records *"N/A — no user-facing surface"* for the whole project with
   its verification written out. What survives is the state family, translated onto a
   fallible API call, and each of its five invariants is carried by an AC-### row —
   never by a prose bullet, which `redkiln verify` would not extract.
8. **The storymap's destination for the `BEGIN DEFERRED` negative control is wrong,
   and this spec does not fix it.** `racers.rs` / `RACERS`, not `mutants.rs` /
   `REGISTRY`: a store wrong only in parallel fails no rule of the event-store family
   and `mutant_registry_is_exhaustive` rejects an empty `fails` list. Recorded in the
   context pack, in the risks and in the anchors; discharged by
   `model-family-and-mutant-pass-column` (HS-S0042), whose PR it is.
9. **The mount point is not retired when `tests/conformance.rs` arrives.** It answers
   the one question the conformance suite cannot ask — *what is in the file after a
   refusal*, read through a connection the port never touched — which is why AC-003
   exists as its own criterion rather than as a clause of AC-002 and AC-005.
