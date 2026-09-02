---
item: HS-S0037
stage: discover
created: 2026-08-12T13:02:00.379Z
updated: 2026-08-12T13:02:00.379Z
template_sig: 86ce4036
rendered_sig: 54c02846
---

# Discover — append: preconditions, then one BEGIN IMMEDIATE transaction

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Slice one-line: replace `append`'s `todo!()` with preconditions-then-one-`BEGIN IMMEDIATE` — empty batch refused first, the three declared ceilings raised as `ExceedsStoreLimit`, every guard probed by `EXISTS` before a row of the batch is inserted, returning the caller's own last position. | `_storymap.md`, *Slices* table, row `durable-event-store` / `append-atomicity-and-store-limits` | The **order** is the specification: refuse, then check ceilings, then open the transaction. Three of the four clauses are about what happens before any SQL runs. |
| **AC-007** — the append is atomic and a probe-then-insert is rejected: one transaction, returning the caller's own last position; the named wrong implementation is a read-then-write adapter, which "passes the sequential rule forever and fails the concurrency family within a handful of iterations." | `project.md`, AC-007; `RUNBOOK.md:4217-4219` | The AC names its own mutant. This story owns the implementation half; `concurrency-family-and-contender-count` owns the family that rejects the wrong one and `model-family-and-mutant-pass-column` owns the registry row (`_storymap.md`, *Coverage*). |
| **AC-009** — the store's limits are declared facts, and the rule that needs them runs: real numeric ceilings, VT-21 – VT-24 pass, `append_reports_exceeded_store_limits` **runs rather than skipping**, and CF-40's clause home stays recorded as open. | `project.md`, AC-009 | This story owns "`append` enforces the ceilings"; `sqlite-fixture-and-whole-suite` owns "the fixture states them" (`_storymap.md`, *Coverage*). Enforcement without declaration is a skip; declaration without enforcement is a red rule. |
| `dependsOn: schema-migration-and-identity` (HS-S0036) — it supplies the tables, the indexes the `EXISTS` probe seeks, `AUTOINCREMENT`, the `UNIQUE` origin pair, and the busy timeout without which `BEGIN IMMEDIATE` on a contended file returns `SQLITE_BUSY` immediately. | `_storymap.md`, *Merge order* item 2; `_decomposition.md`, *Architecture brief*, §6 | The busy-timeout dependency is the one an implementer forgets: without it, contention arrives as `AppendError::Store` and reads as a conformance failure about atomicity. |
| ADR-0012: an empty batch is refused (`AppendError::NoEvents`) **before** any condition is evaluated; a condition is evaluated only against events the store already held when `append` began, never against the batch being written. | `.kb/decisions/0012-append-shape-and-preconditions.md`; `crates/happenstance-core/src/error.rs:225` | The ordering is itself a rule — `empty_batch_is_refused_before_the_condition_is_evaluated` (`crates/happenstance-testkit/src/suite.rs:2870`). |
| `AppendCondition` is a non-empty sequence of `Guard { query, after }` behind `guards()`; a guard is violated by a match at a position **strictly greater** than its `after`, and a guard whose `after` is `None` is violated by any match at all. | `crates/happenstance-core/src/append.rs:20-21`, `:132-141`, `:181` | Every guard is its own probe. `after` is **exclusive** here, unlike `ReadOptions::from`, which is inclusive — the two are one field apart in the port and the skeleton already recorded conflating them as a real bug (`crates/happenstance-sqlite/src/event_store.rs:347-350`). |
| ADR-0015 mints `AppendError::ExceedsStoreLimit { limit: StoreLimit, len }` with exactly three variants, and states plainly that "a capacity limit must never be enforced by a constructor and must never be enforced in `Deserialize`". | `.kb/decisions/0015-validated-identifiers-and-store-limits.md`; `crates/happenstance-core/src/error.rs:238` | For this adapter that means the three ceilings are checked **inside `append`**, before the transaction opens (`_decomposition.md`, *Architecture brief*, §6). |
| What stating a number commits the store to: `append_reports_exceeded_store_limits` appends a payload of exactly the stated ceiling and requires it **accepted**, then one byte larger and requires it refused as `ExceedsStoreLimit { limit: StoreLimit::EventDataLen, .. }` — never as `AppendError::Store`, and never by truncating. | `crates/happenstance-testkit/src/contract.rs:237-247`; `crates/happenstance-testkit/src/suite.rs:4271-4281` | The at-the-ceiling append is the anchor: "without it a store that refused *everything* would satisfy every assertion below." So `append` must be exact at the boundary in both directions. |
| The ceiling is constrained from both sides: VT-21's floor is 65,536 bytes, and the rule allocates the ceiling **plus one byte**, so a ceiling of SQLite's own ~1 GB makes the suite unrunnable. | `_decomposition.md`, *Architecture brief*, §11; `crates/happenstance-core/src/limits.rs:22`; `spec/SPECIFICATION.md:1483-1512` | An adapter-enforced policy ceiling above the floor and far below SQLite's is the only shape that works — and it is "a **fact about this adapter**", documented in the README and enforced in `append`, not a trade. |
| The return value is the **caller's own last position**, not the store head; `AUTOINCREMENT` permits gaps and no code or rule may assume `+1`. | `_decomposition.md`, *Architecture brief*, §6; `crates/happenstance-testkit/src/suite.rs:2623`, `:2953`; `CLAUDE.md` | `GappedPositionStore` — a *conformant* store assigning positions in steps of seven from 4,096 — is what fails a rule that forgets (`crates/happenstance-testkit/README.md:118-122`). |
| `BEGIN DEFERRED` (or no transaction) is the SQLite spelling of read-then-write: a lost race then arrives as a driver error rather than as `AppendError::ConditionViolated`, and the concurrency family distinguishes those by construction — `Attempt::Rejected` versus `Attempt::Failed`. | `_decomposition.md`, *Architecture brief*, §6; `crates/happenstance-testkit/src/concurrency.rs:219-231` | The mutant is discriminated by an existing mechanism, so this story's job is to make the *correct* side observable, not to invent a check. |
| There is **no watchdog anywhere in the suite**, and there must not be one (CF-33). A store that deadlocks hangs the binary and only the CI job timeout notices — a cost `RUNBOOK.md:2669-2674` states is to be chosen "rather than discovered at phase 8". | `project.md`, risks; `crates/happenstance-testkit/tests/mutation_coverage/harness.rs:19-35` | "`BEGIN IMMEDIATE` plus a lock held to commit is precisely where that risk lives." A busy handler must be finite and generous, never infinite. |
| CF-40 `[PROVISIONAL]` — its *ownership* between ADR-0012 and ADR-0015 is contradicted inside ADR-0015 itself and is an accepted open question the KB says it has no standing to resolve. | `.kb/open-questions/cf-40-fixture-limits-ownership.md`; `spec/SPECIFICATION.md:7661` | This project needs the **capability**, not the clause's home. Record and escalate to the ADR queue; settling it in passing is an AC-009 failure. |

## Questions

Open questions to resolve before specifying.

1. **Are the guard probes one compound statement or one per guard?** Deferred to
   `spec` — architecture brief §11 leaves it to the compiler and a measurement.
   Discover fixes only the observable: every guard is evaluated against the state
   the store already held, inside the same transaction that inserts, before any
   row of this batch exists.
2. **What are the three ceilings' numbers?** Deferred to `spec`, with the
   corridor fixed here: strictly above VT-21's 65,536-byte floor
   (`crates/happenstance-core/src/limits.rs:22`), and low enough that the rule can
   allocate ceiling + 1 without the suite becoming unrunnable. VT-22's tag floor
   is 64 and VT-24's batch floor is 128 (`spec/SPECIFICATION.md:1513`, `:1556`).
3. **What busy-timeout value?** ADR-0022's, consumed here. Discover fixes only
   that it is finite — CF-33 forbids a *rule* reading a clock and says nothing
   about an adapter's own retry policy, but an unbounded handler converts a
   livelock into a hung CI job that names no rule.
4. **Does the payload ceiling measure encoded bytes or the caller's `Bytes`
   length?** Deferred to `spec`, and it must not be deferred silently: the rule
   appends exactly `MAX_EVENT_DATA_LEN` bytes of `data` and requires acceptance,
   so if the store's real ceiling is on an encoded row rather than on `data`, the
   declared number is in the wrong unit and the rule fails at the boundary.
5. **Does `append` raise `ExceedsStoreLimit` for a batch that fits individually
   but not in aggregate?** Deferred to `spec`. `StoreLimit` has exactly three
   variants and none of them is "total batch bytes"; inventing a fourth would be
   amending ADR-0015 in passing.
6. **CF-40's clause home.** *Explicitly not settled here.* Recorded as open and
   escalated to the ADR queue, per AC-009's own wording and the open-question
   atom's own statement of standing.
7. **The append-condition SQL strategy.** Consumed from ADR-0022, not chosen here.
   If the chosen strategy cannot express a guard's `after` against the tag layout
   ADR-0022 fixed, that is a finding for the ADR queue, not an improvisation.

## Decision

This is the slice where the adapter stops being able to lie about atomicity.
`append` today is a `todo!()`, and the two ways to get it wrong are both
*sequentially indistinguishable from correct*: evaluating the condition outside
the transaction that assigns the position, and enforcing a capacity ceiling by
some route other than `AppendError::ExceedsStoreLimit`. The spec will cover: the
precondition order — empty batch → `AppendError::NoEvents` before any condition is
looked at, then the three declared ceilings → `ExceedsStoreLimit` with the right
`StoreLimit` variant, both **before** any transaction opens, per ADR-0015's
quarantine argument; one `BEGIN IMMEDIATE` transaction taking the write lock at
the top and holding it to commit; an `EXISTS` probe per `Guard`, evaluated
strictly above that guard's `after` and only against events the store already
held; insertion of the batch in slice order; the return of the **caller's own**
last position rather than the store head; the three ceilings enforced exactly at
their boundary in both directions, in units that match what the fixture will
declare; and a finite busy timeout so contention is a wait rather than an
`AppendError::Store`. This story adds **no conformance rule** — every rule it
must satisfy already exists in `for_each_event_store_rule!` — so the
literal-position bar is vacuous for it, but its return value is precisely where a
`+1` assumption would be introduced, and `AUTOINCREMENT` means gaps are real. No
`[FROZEN]` clause is amended: `EventStore` is frozen (`spec/SPECIFICATION.md:371`)
and this story implements it rather than changing it; CF-40 is `[PROVISIONAL]`
and its ownership is recorded as open, not resolved.

## The wrong implementation

**The mutant: `BEGIN DEFERRED` probe-then-insert.** Concretely — open a deferred
transaction (or none), run the guards' `EXISTS` probes, and, if none matched,
insert. It is the natural translation of "check, then write", and it is what an
implementer produces when the busy-timeout pain of `BEGIN IMMEDIATE` shows up
first.

It passes the entire sequential suite, **forever**. `append_returns_last_written_position`,
`batch_positions_follow_slice_order`, `empty_batch_is_refused_before_the_condition_is_evaluated`,
every condition rule with one handle — all green, because with one writer there is
no window. It only fails when two writers decide from the same snapshot, and even
then it fails in a way that reads like something else: SQLite upgrades a deferred
transaction's read lock at the first write, so the loser gets `SQLITE_BUSY` or
`SQLITE_BUSY_SNAPSHOT`, which becomes `AppendError::Store` → `Attempt::Failed`
rather than `AppendError::ConditionViolated` → `Attempt::Rejected`
(`crates/happenstance-testkit/src/concurrency.rs:219-231`). The racing rules
distinguish those two by construction and assert "exactly one winner", so the
family catches it within a handful of iterations — which is precisely the claim
`RUNBOOK.md:4217-4219` makes and which nothing in the workspace has ever
demonstrated against a real database.

**Where the negative control must live, and a correction to carry forward.**
`_storymap.md`'s row for `model-family-and-mutant-pass-column` says to add the
`BEGIN DEFERRED` probe-then-insert row to
`crates/happenstance-testkit/tests/mutation_coverage/mutants.rs`'s **`REGISTRY`**.
That is the wrong table, and it will not compile past its own meta-test: a store
that is wrong only in parallel fails **no** rule of the event-store family, and
`mutant_registry_is_exhaustive` rejects a row whose `fails` list is empty
(`crates/happenstance-testkit/tests/mutation_coverage/racers.rs:10-18`;
`crates/happenstance-testkit/README.md:108-113`). It belongs in `racers.rs` and
`RACERS`, as an `Arc`/`Mutex` `SendEventStore` whose window is closed by a
**rendezvous** rather than by luck — an atomic counter and `std::thread::yield_now`,
bounded by a count of yields, never `thread::sleep` (`racers.rs:20-34`) — and its
fixture added to `for_each_racer!` in
`crates/happenstance-testkit/tests/mutation_coverage.rs`. This story is where the
discrepancy is found, so it is recorded here and discharged in HS-S0042 rather
than left for whoever hits the compile error.

**The second mutant, and the suite's blind spot for it: an `append` that enforces
its payload ceiling by letting SQLite refuse the row.** Bind an oversized blob,
let the driver return `SQLITE_TOOBIG`, and map `rusqlite::Error` to
`AppendError::Store`. Every existing check is satisfied *if the fixture declares
no ceilings*: `append_reports_exceeded_store_limits` returns
`RuleOutcome::Skipped { capability: NO_STORE_LIMITS, .. }` and the run is green
(`crates/happenstance-testkit/src/suite.rs:4285-4296`). The rule is on `MUST_SKIP`
(`crates/happenstance-testkit/tests/mutation_coverage.rs:3157-3163`), so a skip is
a *legitimate* outcome and nothing anywhere says the store had a ceiling it simply
did not declare. That is AC-009's "runs rather than skipping" stated as a defect:
the mutant is not a store that refuses wrongly, it is a store that refuses
**correctly by accident** and declares nothing, so the whole VT-21 – VT-24 family
evaporates from the run while the log stays green. The control is split by design
— enforcement here, declaration in `sqlite-fixture-and-whole-suite` (HS-S0040) —
and the ledger entry for AC-009 must cite the *run output* showing `Ran`, not
merely a green binary.

## Gate: Discover

Tick each box once the sections above satisfy it. `redkiln advance` reads these
against `.redkiln/templates/gates/discover.md` — where each box's rationale is
written — and will not leave `discover` until every one is ticked. Keep each box on
one line; the parser matches line by line.

- [x] The problem is framed in one paragraph.
- [x] Prior art and constraints are recorded in the signal ledger.
- [x] Open questions are either answered or explicitly deferred.
- [x] The next stage (spec) has a clear starting point.
- [x] The wrong implementation this work rejects is named.
- [x] No conformance rule added here asserts a literal position value.
- [x] Any `[FROZEN]` clause this touches is changed by a new ADR, written first.
- [x] If a rule here seems wrong, it is fixed and the reason given in the same change.
