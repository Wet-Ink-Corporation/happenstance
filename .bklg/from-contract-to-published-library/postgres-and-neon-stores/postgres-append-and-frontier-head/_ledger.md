---
item: "HS-S0062"
stage: implement
created: "2026-08-12"
updated: "2026-08-12"
---

# Acceptance ledger — A real Postgres append path and a frontier head

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Two notes specific to this story. **Conformance evidence must name the live run**, not a compile:
a `todo!()`-free body type-checks against any signature, so `cargo build` is not evidence for any row
here — cite the Postgres CI job's `cargo test -p happenstance-postgres --all-features -- --ignored
--show-output` output. And **AC-011's evidence is an absence**: the check is that no file this story
touched cites `ADR-0024` or a `.kb/decisions/0024-*` path, because no such atom exists.

```yaml
- id: AC-001
  criterion: >-
    GIVEN an adapter author running the suite against a live Postgres, WHEN `append` is called with
    zero events, THEN it returns `AppendError::NoEvents` before the append condition is evaluated
    and before any row, lock or sequence value is taken — so a zero-event call is never an expensive
    no-op — and the batch stays borrowed (`&[Event]`) rather than being collected to satisfy the
    signature.
  satisfied: true
  evidence: "`append` returns `AppendError::NoEvents` from its first statement, before `check_ceilings` and before anything opens a transaction (`crates/happenstance-postgres/src/event_store.rs`, `impl SendEventStore` -> `append`). `events` stays `&[Event]` throughout. Green live: `rules::append_rejects_empty_batch` and `rules::empty_batch_is_refused_before_the_condition_is_evaluated`."
  mount_point: "crates/happenstance-postgres/src/event_store.rs — `impl SendEventStore for PostgresEventStore::append` (replacing the `todo!()` at :143), reached through `event_store_conformance!` in crates/happenstance-postgres/tests/"
  verifying_test: "happenstance_testkit::rules::append_rejects_empty_batch (crates/happenstance-testkit/src/suite.rs:2841) and rules::empty_batch_is_refused_before_the_condition_is_evaluated (suite.rs:2870), via event_store_conformance! in crates/happenstance-postgres/tests/"

- id: AC-002
  criterion: >-
    GIVEN a consuming application whose business invariant is enforced by a conditional append, WHEN
    two such appends race against the same boundary on a store whose writers are not serialised,
    THEN exactly one wins and the loser gets `AppendError::ConditionViolated` carrying the
    conflicting position — because condition evaluation and insert happen on one snapshot inside one
    transaction, never as a probe followed by a write.
  satisfied: true
  evidence: "Condition evaluation and insert are one transaction (`append_once`), and the race is elected by SSI rather than by a lock: a conditional append opens `BEGIN ISOLATION LEVEL SERIALIZABLE`, an unconditional one stays at the pool default and pays nothing. A `40001` retries up to `SERIALISATION_ATTEMPTS`, and the retry sees the winner's committed row and returns `ConditionViolated` -- the answer the caller was owed. Green live: the whole condition family, `racing_conditional_appends_elect_one_winner`, `condition_rejection_leaves_store_unchanged`, `dropped_append_future_leaves_no_partial_batch`."
  mount_point: "crates/happenstance-postgres/src/event_store.rs — `append`'s single transaction (the condition evaluation and the insert), mounted via event_store_conformance! in crates/happenstance-postgres/tests/"
  verifying_test: "happenstance_testkit::rules::racing_conditional_appends_elect_one_winner (crates/happenstance-testkit/src/suite.rs:5325), rules::condition_rejection_leaves_store_unchanged (suite.rs:4910), the condition family (suite.rs:4484-4910), rules::dropped_append_future_leaves_no_partial_batch (suite.rs:3140)"

- id: AC-003
  criterion: >-
    GIVEN the adapter author's fear that the port quietly assumed their storage shape, WHEN they run
    the suite against this store, THEN it passes with positions assigned so that allocation order is
    visibility order without serialising writers — `position` is not `bigserial`, `nextval()` is not
    on the append path, and no store-wide lock is held to commit.
  satisfied: true
  evidence: "SATISFIED WITH A RECORDED CORRECTION to the criterion, agreed with the repository owner before implementation. The clause `nextval() is not on the append path` contradicts the mechanism the rest of this spec mandates: arm C KEEPS `nextval()` and repairs the invariant on the read side (`experiments/position-visibility/schema/arm-c.sql`: 'The append path is the baseline's. nextval() is untouched'), and ADR-0013 rejected both nextval-free arms at 16x and 30x throughput. What IS satisfied, and is what the criterion was written to protect: `position` carries no column default and is not `serial`/`bigserial`/identity (asserted live by `migration_1_does_not_preempt_adr_0024` reading `column_default IS NULL` and `is_identity = 'NO'`), the sequence is read explicitly at the one INSERT that allocates, and no advisory lock or store-wide lock is taken on the append path. Green live: `positions_are_unique`, `positions_are_strictly_monotonic`, `nothing_below_an_observed_position_appears_later`."
  mount_point: "crates/happenstance-postgres/src/event_store.rs — the position-assignment path inside `append` (the xid8 column written from the append transaction), plus the schema in crates/happenstance-postgres/migrations/"
  verifying_test: "happenstance_testkit::rules::nothing_below_an_observed_position_appears_later (CF-13, crates/happenstance-testkit/src/suite.rs:5880), rules::positions_are_unique (suite.rs:1583), rules::positions_are_strictly_monotonic (suite.rs:1614), plus the in-crate live test in crates/happenstance-postgres/tests/ asserting no nextval and no store-wide lock on the append path"

- id: AC-004
  criterion: >-
    GIVEN an application author who has just observed position P from `append`, WHEN they call
    `head()`, THEN they get the visibility frontier — which may legitimately trail P — and not
    `max(position)`; on an empty store they get `None`; and nothing in the crate claims
    read-your-own-writes, because it does not hold.
  satisfied: true
  evidence: "`head` is `max(position) WHERE xact_id < pg_snapshot_xmin(pg_current_snapshot())` -- the frontier, not the maximum -- and returns `None` on an empty store. Read-your-own-writes is not claimed anywhere in the crate; `read_stream.rs`'s module docs state the opposite explicitly. Green live: `head_of_an_empty_store_is_none`, `head_is_the_highest_visible_position` (a bound, not an equality), `head_advances_across_two_handles`. NOTE the measured caveat under AC-008: `head_is_the_highest_visible_position` is one of the read-after-write rules the frontier can push behind, observed failing in one serial run of five."
  mount_point: "crates/happenstance-postgres/src/event_store.rs — `head` (replacing the `todo!()` at :160), mounted via event_store_conformance! in crates/happenstance-postgres/tests/"
  verifying_test: "happenstance_testkit::rules::head_is_the_highest_visible_position (crates/happenstance-testkit/src/suite.rs:1798), rules::head_of_an_empty_store_is_none (suite.rs:1731), rules::head_advances_across_two_handles (suite.rs:1855), plus the in-crate live test in crates/happenstance-postgres/tests/ comparing head() against the frontier query and against max(position)"

- id: AC-005
  criterion: >-
    GIVEN a reader streaming a long result set while other writers commit underneath it, WHEN the
    stream is polled across several `FETCH`es, THEN no row appears beneath a position the reader has
    already yielded — because the frontier predicate was composed into the cursor's `DECLARE` inside
    one `BEGIN ISOLATION LEVEL REPEATABLE READ` transaction and evaluated once, never re-evaluated
    per chunk.
  satisfied: false
  evidence: "NOT SATISFIED, and recorded as ES-11's falsifier rather than worked around. The frontier predicate IS composed into the cursor's `DECLARE` inside one `BEGIN ISOLATION LEVEL REPEATABLE READ READ ONLY` and is evaluated once, never per `FETCH` -- that half holds. What does not hold is the clause's timing requirement: ES-11 asks for the read's state to be fixed no later than the FIRST POLL, and an async driver's first poll can only start the round trip that takes the snapshot. `read_result_is_stable_under_concurrent_append` and `query_items_share_one_snapshot` fail deterministically on that gap. Spawning the open onto the runtime at first poll was tried and does not close it (5 failures in 5 runs). Opening in `read` would close it and would break ADR-0011 and AC-012. Written up at `crates/happenstance-postgres/src/read_stream.rs` (module docs) and `.kb/_intake/2026-09-06-es-11-falsified-by-an-async-driver.md`; the clause amendment is an ADR and is not this story's to take."
  mount_point: "crates/happenstance-postgres/src/read_stream.rs — the DECLARE inside the existing REPEATABLE READ transaction (:37-57), reached from `PostgresEventStore::read` (event_store.rs:124-134)"
  verifying_test: "happenstance_testkit::rules::nothing_below_an_observed_position_appears_later reached through the read path (crates/happenstance-testkit/src/suite.rs:5880), rules::read_from_a_gap_position (suite.rs:1490), plus the in-crate live test in crates/happenstance-postgres/tests/ holding a PgReadStream open across an interleaved append-and-commit"

- id: AC-006
  criterion: >-
    GIVEN an operator asking "does this store already hold event X", WHEN `contains_event_id` is
    called, THEN it answers from real `origin_store` / `origin_position` columns rather than
    `todo!()`, and the frontier disagreement (a committed row above the frontier is invisible to
    `read`) is answered deliberately and written down in the method's rustdoc — which answer was
    chosen and why — with ES-41 left [PROVISIONAL] and replication semantics left to HS-P0017.
  satisfied: true
  evidence: "`contains_event_id` reads real `origin_store`/`origin_position` columns, which this story added to migration 1 and which `insert_batch` binds directly rather than stamping in a follow-up `UPDATE`. The frontier disagreement is answered deliberately and the answer is written in the method's rustdoc: it does NOT carry the frontier predicate, so a committed-but-invisible row answers `true`, because a transient disagreement with `read` is cheaper than the permanent duplicate that answering `false` would let a replication ingest create. ES-41 left `[PROVISIONAL]`; no replication semantics settled here. Green live: `contains_event_id_reports_membership`, `event_ids_are_unique_within_a_store`."
  mount_point: "crates/happenstance-postgres/src/event_store.rs — `contains_event_id` (replacing the `todo!()` at :173) and its rustdoc; the origin columns in crates/happenstance-postgres/migrations/"
  verifying_test: "happenstance_testkit::rules::contains_event_id_reports_membership (crates/happenstance-testkit/src/suite.rs:2551) and rules::event_ids_are_unique_within_a_store (suite.rs:2092) in the live job; the recorded answer checked by cargo xtask ci's REQUIRED rustdoc step plus a human read"

- id: AC-007
  criterion: >-
    GIVEN a sync runner that must distinguish "this will never fit here, park it" from "the disk is
    full, retry", WHEN it appends a payload, tag count or batch above this store's stated ceiling,
    THEN it gets `AppendError::ExceedsStoreLimit { limit: StoreLimit::… }` — never
    `AppendError::Store`, never a silent truncation — and, independently, the contract's guaranteed
    minima (65,536-byte payload, 128-event batch, and the tag and query minima) are accepted.
  satisfied: true
  evidence: "`check_ceilings` runs before the transaction opens and reports `AppendError::ExceedsStoreLimit { limit: StoreLimit::* }`, never `Store` and never by truncating. Green live: `append_reports_exceeded_store_limits`, `store_accepts_the_guaranteed_minimum_payload`, `store_accepts_the_guaranteed_minimum_tag_count`, `store_accepts_the_guaranteed_minimum_batch_size`, `store_evaluates_a_query_at_the_guaranteed_minimum_item_count`."
  mount_point: "crates/happenstance-postgres/src/event_store.rs — `append`'s limit checks and their mapping to AppendError::ExceedsStoreLimit, against the fixture ceilings PostgresFixture declares"
  verifying_test: "happenstance_testkit::rules::append_reports_exceeded_store_limits (crates/happenstance-testkit/src/suite.rs:4282), rules::store_accepts_the_guaranteed_minimum_payload (suite.rs:3775), rules::store_accepts_the_guaranteed_minimum_tag_count (suite.rs:3829), rules::store_accepts_the_guaranteed_minimum_batch_size (suite.rs:3954), rules::store_evaluates_a_query_at_the_guaranteed_minimum_item_count (suite.rs:3880)"

- id: AC-008
  criterion: >-
    GIVEN an adapter author who needs "an executable definition of correct" rather than a prose
    specification to interpret, WHEN the live-Postgres job runs, THEN `event_store_conformance!` and
    `event_store_model_conformance!` are both invoked from `crates/happenstance-postgres/tests/` in
    their full macro expansion and every rule reports pass, fail, or a skip carrying the fixture's
    stated reason — no rule is `#[cfg]`-ed out and none is absent from the run.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-postgres/tests/ — the event_store_conformance!(mod_name = …, emit = …, fixture = PostgresFixture::…) and event_store_model_conformance! invocations (Architecture §2 Root B)"
  verifying_test: "the two macro invocations in crates/happenstance-postgres/tests/ (crates/happenstance-testkit/src/lib.rs:312, crates/happenstance-testkit/src/model.rs:782), run as `cargo test -p happenstance-postgres --all-features -- --ignored --show-output` in the Postgres CI job; rule count run compared against the macros' expansion"

- id: AC-009
  criterion: >-
    GIVEN an adapter author debugging a failure at 2am, WHEN the mechanism's own decode paths fail
    (an `xid8` that will not parse, an absent frontier row, a `bigint` position that decoded as zero
    against `SequencePosition`'s `NonZeroU64`), THEN they meet a named `#[non_exhaustive]` variant on
    `PostgresEventStoreError` naming the cause — not a flattened `sqlx::Error::Database`, not a
    variant per `SQLSTATE`, and not a `ConditionViolated` variant on either enum, because a
    condition violation is not an adapter failure.
  satisfied: true
  evidence: "Four new `#[non_exhaustive]` variants on `PostgresEventStoreError`, each named for a cause rather than a code: `MissingIdentity`, `MalformedIdentity { len }`, `UnstampedEvent { position }`, `Worker(JoinError)`, `NoRuntime`. No variant per `SQLSTATE` -- the `40001` retry matches on `sqlx`'s own `DatabaseError::code`, which `error.rs` says in terms is why no such variant exists. No `ConditionViolated` variant on either enum; a violation travels as the contract's `AppendError::ConditionViolated`, proven green by the condition family."
  mount_point: "crates/happenstance-postgres/src/error.rs — the new #[non_exhaustive] variants on PostgresEventStoreError (:29-58), surfaced through `impl SendEventStore for PostgresEventStore`'s `type Error`"
  verifying_test: "in-crate unit tests over the new variants' construction and Display in crates/happenstance-postgres/src/error.rs; `cargo clippy --workspace --all-targets --all-features -D warnings` (REQUIRED); the condition family (crates/happenstance-testkit/src/suite.rs:4484-4910) proving a violation arrives as AppendError::ConditionViolated"

- id: AC-010
  criterion: >-
    GIVEN that this is the first adapter in the portfolio whose positions genuinely have gaps, WHEN
    any test this story adds asserts about positions, THEN it compares against positions the store
    actually assigned and never against a literal ([1, 2, 3]) — so a legitimately gappy conformant
    store cannot fail a test for being conformant.
  satisfied: true
  evidence: "`cargo xtask lint-position-literals` is a REQUIRED gate step and is green. No test this story added asserts on a literal position; the story-local live tests compare against positions the store assigned, and `insert_bare_row`'s explicit argument is a primary key being made distinct, not a position being asserted on."
  mount_point: "crates/happenstance-postgres/tests/ and crates/happenstance-postgres/src/ — every assertion this story adds about positions"
  verifying_test: "cargo xtask lint-position-literals — a REQUIRED gate step (xtask/src/main.rs:389, :687; lints::no_position_literals)"

- id: AC-011
  criterion: >-
    GIVEN a future reader auditing why this store works the way it does, WHEN they read the crate's
    module docs and this story's ledger, THEN they find what was wired and on whose prior authority —
    ADR-0013's phase-2 measurement, which rejected two serialising arms at 16×/30× throughput cost —
    and no citation of ADR-0024, which does not exist and whose choice, structural bill and
    re-measurement belong to the story that owns them.
  satisfied: true
  evidence: "The crate cites ADR-0013 and `experiments/position-visibility/` as the mechanism's prior authority, in `migrations/0001_event_log.sql` and `src/read_stream.rs`. `grep -rn 'ADR-0024' crates/happenstance-postgres/` returns nothing and `.kb/decisions/0024-*` does not exist. The choice, the structural bill and the re-measurement are left to the stories that own them."
  mount_point: "crates/happenstance-postgres/src/lib.rs and src/event_store.rs — the module rustdoc recording the wired arm and its prior authority; no .kb/decisions/0024-* path is cited anywhere in the diff"
  verifying_test: "static check that no file changed by this story cites ADR-0024 or a .kb/decisions/0024-* path (no such file exists — `ls .kb/decisions/`); cargo xtask ci's REQUIRED docs step plus a human read of the module rustdoc against .kb/decisions/0013-position-assignment-and-visibility.md and experiments/position-visibility/README.md:13-19"

- id: AC-012
  criterion: >-
    GIVEN the `wasm32` / `!Send` half of the portfolio that this crate must not regress, WHEN this
    story lands, THEN `read` is still not `async` and still returns the stream at the top level,
    nothing (connection, transaction, cursor, frontier) is acquired until the first poll,
    `#[async_trait]` appears nowhere, and generic code can still bind the bare `EventStore` at this
    concrete store.
  satisfied: true
  evidence: "`read` is still not `async` and still returns the stream at the top level; `send_flavour_satisfies_the_bare_bound` and `store_is_send_and_sync` are unchanged and green. `#[async_trait]` appears nowhere in the crate. The stream stays lazy -- `PgReadStream::new` performs no I/O and takes no pool checkout, and the spawn happens on the first poll rather than at construction. `cargo xtask ci --fast`'s four wasm32 steps green."
  mount_point: "crates/happenstance-postgres/src/event_store.rs — the `fn read` signature (:124-134, not async) and the in-crate bound tests (:177-198); acquisition deferred into PgReadStream's first poll (read_stream.rs:37-57)"
  verifying_test: "crates/happenstance-postgres/src/event_store.rs:177-198 — send_flavour_satisfies_the_bare_bound and store_is_send_and_sync; the in-crate live test in crates/happenstance-postgres/tests/ constructing a stream and dropping it unpolled with no pool checkout; cargo xtask ci's four wasm32 steps and cargo xtask affected --base main"
```
