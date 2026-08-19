# Grounding — The two stores that disagree with the port (HS-P0014)

Companion file for planning only. Cited paths verified to exist in this worktree
at grounding time (`git rev-parse --show-toplevel` ==
`D:/repos/happenstance/.claude/worktrees/from-contract-to-published-library`).

## Accepted decision atoms that constrain this project

- **`.kb/decisions/0013-position-assignment-and-visibility.md`** (`kb-decision-0013`,
  ADR-0013, phase 4, accepted). Lifts ES-10 to `[FROZEN]` on the strength of a
  phase-2 experiment and settles the **mechanism** — `xid8` + `pg_snapshot_xmin`
  ("arm C") — as the only one of four arms that both passes the inversion
  detector on concurrent writer pairs and leaves writers unserialised. It is
  explicit that this is *not* ADR-0024: "Which mechanism a real
  `happenstance-postgres` adapter uses is left open for the adapter's own
  phase; this decision only establishes that one affordable mechanism exists."
  Three caveats are binding on whatever ADR-0024 writes, not open for
  re-litigation:
  1. The invariant is **global**, not per-boundary — a per-boundary invariant
     would make the projection checkpoint unsound, costing ~9% throughput at 64
     clients (arm B-tag: 0.935×) versus arm C's near-parity (0.987–1.026×).
     This premise is itself an open question (see below) owned by phase 6, not
     this project.
  2. `head()` on the accepted mechanism is a **visibility frontier**, not
     `max(position)` — a snapshot predicate, not an identity.
  3. Read-your-own-writes does **not** hold: `append` returning `Ok(P)` does not
     promise the next `head()` sees `P`. Staleness is bounded by the longest
     open write transaction *anywhere in the cluster*.
  4. `SequencePosition::next` uses `NonZeroU64::checked_add`, not
     `saturating_add` — cited because any Postgres position-handling code
     touches this type.
- **`.kb/decisions/0009-error-send-sync.md`** (`kb-decision-0009`, ADR-0009).
  Governs the shape of `PostgresEventStoreError` / `NeonError` — error stays
  unbounded, no forced `Send`, consistent with
  `crates/happenstance-postgres/src/error.rs:1-30`, which already declines to
  flatten `sqlx::Error::Database` into per-`SQLSTATE` variants and instead
  carries decoding-failure variants (a `bigint` position that isn't
  `NonZeroU64`, `text` that isn't a valid `EventType`/`Tag`).
- **`.kb/decisions/0011-read-laziness-and-isolation.md`** (`kb-decision-0011`,
  ADR-0011). Underlies ES-11/ES-12 (read is a snapshot; all `Query` items share
  one snapshot) and ES-42 (`read`'s return carries no `Unpin` bound, discharged
  by ADR-0011's E11 erasure wrapper) — both PROVISIONAL clauses this project's
  AC-013 must report the exercised status of.
- **`.kb/decisions/0012-append-shape-and-preconditions.md`** (`kb-decision-0012`,
  ADR-0012). `append` keeps its borrowed batch; phase 4 explicitly declined to
  measure what this project must now measure for real (position-allocation
  cost against a live adapter).
- **`.kb/decisions/0015-validated-identifiers-and-store-limits.md`**
  (`kb-decision-0015`, ADR-0015). Source of VT-21–VT-24's guaranteed minima
  (`spec/SPECIFICATION.md:1483-1567`) that AC-010 requires this project's two
  fixtures to either pass or refuse through the declined-capability /
  `AppendError::ExceedsStoreLimit` path — never an opaque `AppendError::Store`.
- **ADR-0024 does not exist yet.** It is referenced throughout
  `crates/happenstance-postgres/src/lib.rs:28-47` (as an "open decision"),
  `.kb/open-questions/postgres-arm-c-structural-cost.md`, and this project's
  own AC-001 as the ADR this project is responsible for **writing and getting
  accepted**. Confirmed absent from `.kb/decisions/` (highest numbered atom on
  disk is `0029-msrv-raised-to-1-97-1.md`; no `0024-*` file exists). This is
  not a gap in grounding — authoring ADR-0024 is literally AC-001's job — but
  briefs must not cite it as if it already exists.

## Open questions this project inherits (read, not silently resolved)

- **`.kb/open-questions/postgres-arm-c-structural-cost.md`**
  (`kb-open-question-postgres-arm-c-cost-001`). States plainly that the
  experiment "measured four SQL strategies, not four implementations of
  `SendEventStore`" and names four concrete gaps between "a SQL strategy
  passed" and "an adapter passed": no connection pooling, no transaction
  lifetime tied to a trait method's async boundary, no cursor, no error
  mapping. Explicitly "Owned by phase 10 and by ADR-0024" — i.e., this
  project. Its four ordered sub-questions (whether `sqlx`'s pooling model lets
  `append` hold arm C's snapshot without contorting the signature; whether
  `nothing_below_an_observed_position_appears_later` still passes against a
  real adapter; what staleness bound the docs should actually promise; whether
  arm B-tag gets reconsidered) are the shape AC-001's "measured cost" section
  should answer, not merely gesture at.
- **`.kb/open-questions/global-versus-per-boundary-visibility-invariant.md`**
  (`kb-open-question-global-vs-boundary-visibility-001`). ADR-0013 admits it
  chose the global invariant "by decision... not by evidence," resting on the
  projection checkpoint being global — a premise `projection-store-freeze`
  (phase 6) has not yet settled. This project's `_intake-brief.md:59-73`
  correctly marks this "carried in... **not** reopened here" — the grounding
  note preserves that boundary: ADR-0024 measures and documents arm C's cost,
  it does not re-litigate global-vs-per-boundary.

## Dependency status (schedule tension to flag, not silently resolve)

- **`.bklg/from-contract-to-published-library/projection-store-freeze/project.md`**
  (HS-P0010) is this project's declared dependency
  (`_intake-brief.md:37-39`), and its frontmatter currently reads
  `status: planning`, `stage: storymap` — i.e. not yet built/accepted. AC-005
  ("`PostgresProjectionStore` passes the projection suite against the owned
  `Batch` frozen by `projection-store-freeze`") is blocked on that project
  reaching a frozen `ProjectionStore` port and a `Batch` shape before this
  project's AC-005 work can start. Briefs should sequence AC-005 (and any
  `NeonProjectionStore` work) after that dependency, not treat it as already
  available.

## Existing code this project must extend, not replace

- **`crates/happenstance-postgres/src/`** — `lib.rs` (crate doc naming the
  three open decisions: ES-10 mechanism, tag matching strategy, TLS backend —
  TLS is *settled* against `sqlx`'s `tls-rustls` failing `cargo deny`'s licence
  allowlist on `webpki-roots` (`CDLA-Permissive-2.0`), not on `ring`), `error.rs`
  (two independent, `#[non_exhaustive]` `thiserror` enums — one per port, per
  ADR-0009's "no forced coupling" reasoning), `event_store.rs`,
  `projection_store.rs`, `read_stream.rs`. Crate currently `publish = false`
  with `#![allow(clippy::todo)]` scoped and commented "Phase 10 removes both
  the bodies and this line" (`lib.rs:59-63`) — AC-012 is exactly that removal.
- **`crates/happenstance-neon/src/`** — `lib.rs`'s doc comment is unusually
  load-bearing for this project and should be treated as near-normative:
    - The capability table (`lib.rs:18-24`): no connection, no transaction
      handle, no cursor, exactly one round trip per operation, and a **hard**
      64 MiB response cap (`pub const MAX_RESPONSE_BYTES: usize = 64 * 1024 *
      1024` at `crates/happenstance-neon/src/transport.rs:54`) — this is
      AC-010's "Neon's 64 MiB response cap" anchor, already a named constant,
      not something to invent.
    - `ProbeThenWriteStore` (`event_store.rs`, re-exported at `lib.rs:126`) is
      **already written** as the documented wrong implementation: it
      type-checks against `EventStore` perfectly and is silently unsound (two
      round trips, no shared snapshot, a conflicting append in the window is
      invisible to the probe). Briefs must not reinvent this — it is the
      fixture-adjacent "named wrong implementation" CLAUDE.md's conformance
      section asks every new rule to have, already in-tree for the real
      adapter to avoid repeating.
    - The crate doc **already contains the answer to AC-008**
      (`conflicting_position`, `lib.rs:46-77`): a single-statement CTE
      (quoted in full, `WITH probe AS (...), ins AS (...) SELECT ...`) that
      computes the probe and the write on one snapshot and returns both the
      appended position and the conflicting one when nothing was appended.
      The doc states the cost honestly — "one extra aggregate index scan on
      every append including the uncontended ones" — and a correctness
      precondition: it needs `IsolationLevel::Serializable`, which is why
      `NeonConfig`'s default isolation is `Serializable` rather than
      Postgres's `ReadCommitted`. AC-008 is therefore substantially a
      **verification-on-a-live-endpoint** task (does the CTE behave as
      documented against real Neon?) plus writing the decision record that
      corrects "the decision ledger's standing assumption" the crate doc
      itself says was wrong (`lib.rs:72-73`: "contrary to the standing
      assumption in the decision ledger, the collapse **keeps**
      `conflicting_position`").
    - Bare-flavour-only is a compiled fact, not a design choice up for
      revisiting: `lib.rs:79-89` documents that `NeonEventStore` implements
      only `EventStore`, never `SendEventStore`, on either target, and that
      trying to add a second `SendEventStore` impl produces
      `error[E0119]` against `trait_variant`'s blanket impl — i.e. the two
      flavours are "mutually exclusive per type," matching ADR-0001's
      one-direction implication table at `crates/happenstance-core/src/store.rs:17-18`.
    - `transport.rs`'s `SqlTransport` is a one-method trait with
      `NullTransport` as the in-tree stand-in; a real HTTP client (TLS on
      host, `wasm-bindgen` `fetch` on `wasm32`) is explicitly **not** built
      yet and is named as a cost the crate "cannot demonstrate... only that
      the shape above it does not need to know which one it has"
      (`lib.rs:91-99`). AC-006's "real Neon `/sql` endpoint" therefore
      requires building a real `SqlTransport` impl as part of this project's
      scope, not assuming one exists.

## Conformance-suite machinery this project drives (existing, not to be reinvented)

- **`crates/happenstance-testkit/src/contract.rs`** defines `Fixture` and
  `Capability`. `Capability::SUPPORTED` / `Capability::declined(reason:
  &'static str)` (declining with `""` is rejected by the constructor per its
  own doc, `contract.rs:393` shows the failing example). `SECOND_HANDLE` and
  `REOPEN` are **required** deliberate answers per fixture; `MID_BATCH_FAULT`
  defaults to declined (`contract.rs:207-211`) because "an in-memory store has
  no fault to inject." `MAX_EVENT_DATA_LEN` / `MAX_TAGS_PER_EVENT` /
  `MAX_EVENTS_PER_BATCH` are deliberately `Option<usize>`, not `Capability`,
  because a ceiling is "a store reporting a fact about itself," not a trade
  (`contract.rs:213-253`) — this is the exact mechanism AC-010 asks the Neon
  fixture to use for its 64 MiB cap and whatever numeric ceilings the Postgres
  fixture states.
- **`crates/happenstance-testkit/src/fixtures.rs`** — `MemoryFixture`
  (`SECOND_HANDLE = Capability::SUPPORTED`, `REOPEN` declined with a real
  reason, `fixtures.rs:230-280`) is the reference pattern both new fixtures
  should follow structurally.
- **CF-13** (the rule AC-002 names) lives via
  `nothing_below_an_observed_position_appears_later`, exercised today by the
  fixture `PreCommitPositionStore` (referenced in
  `crates/happenstance-testkit/tests/mutation_coverage/mutants.rs` and
  `.../correct.rs`, and `src/suite.rs`) — the **fixture** instrument
  RUNBOOK.md's instrument-portfolio table names for the position-allocation
  axis ("fixture, phase 3 — `PreCommitPositionStore` fails
  `nothing_below_an_observed_position_appears_later` deterministically on one
  thread (CF-13). Adapter at phase 10" — `RUNBOOK.md`, instrument-portfolio
  table). This project supplies the **adapter** column phase 10 was always
  going to need.
- **`concurrency::CONTENDERS`** = `8`, defined at
  `crates/happenstance-testkit/src/concurrency.rs:206` — the literal value
  AC-003 must run `event_store_concurrency_conformance!` at.
- **Mutation harness** — `crates/happenstance-testkit/tests/mutation_coverage.rs`
  and its `REGISTRY`; grepping the tree today shows **no** `PostgresEventStore`
  or `NeonEventStore` entry anywhere in `happenstance-testkit`, confirming
  AC-004's "the phase-3 mutant harness runs with `PostgresEventStore` in the
  pass column" is a real, not-yet-done addition, not already wired.
- **CI wiring today** (`xtask/src/main.rs`) mentions `happenstance-neon` only
  for its **wasm32 build** (`main.rs:266-277`, `:571-575`, `:736`, `:789`) —
  there is no live-Postgres or live-Neon job yet. AC-011's "Postgres and Neon
  suites run in their own CI jobs" and "wasm32 build of `happenstance-neon`
  remains part of the default gate" are both real additions layered onto the
  existing wasm32 step, which must not regress.

## Patterns to follow

- Two independent `#[non_exhaustive]` `thiserror` enums, one per port, never a
  `ConditionViolated` variant on either (that stays `AppendError`'s) —
  `crates/happenstance-postgres/src/error.rs:1-30`.
- Crate-level doc comment as the load-bearing design record for a skeleton
  (open decisions, capability tables, the "trap" a naive implementation
  falls into) — both `lib.rs` files already do this; new bodies should keep
  the doc comments in sync rather than let code and doc diverge as `todo!()`
  bodies get filled in.
- `#![allow(clippy::todo)]` scoped per-crate with a phase-naming comment,
  removed in the same change that removes the last `todo!()` — both crates
  today, per CLAUDE.md's skeleton definition.

## Tensions / risks to flag in briefs

1. **AC-005 (`PostgresProjectionStore`) depends on a project still at
   `stage: storymap`.** `projection-store-freeze` (HS-P0010) has not frozen
   `ProjectionStore` or its `Batch` shape yet. Sequence AC-005 (and any
   `NeonProjectionStore` scope) after that dependency lands; do not plan it in
   parallel with AC-001–AC-004/006–004.
2. **ADR-0024 does not exist.** Every brief referencing "ADR-0024" is
   referencing a decision record this project must write and get accepted
   (AC-001) — not an existing anchor. Do not cite it as prior art.
3. **The global-vs-per-boundary invariant is out of scope but load-bearing.**
   ADR-0013's global choice (which ADR-0024 inherits) rests on an unsettled
   premise owned by `projection-store-freeze`. If that project's `Batch`/
   checkpoint design turns out boundary-scoped, ADR-0013 (and by extension
   whatever ADR-0024 measures) reopens. Briefs should note this as a
   downstream risk rather than silently assuming the global invariant is
   permanently settled.
4. **`conflicting_position`'s standing assumption is already known-wrong in
   the decision ledger**, per the Neon crate doc's own words
   (`lib.rs:72-73`) — AC-008 is partly a correction of an existing incorrect
   assumption, which briefs should state plainly rather than treat as a
   fresh open question.
5. **The Neon HTTP transport does not exist yet** (`NullTransport` is the only
   in-tree impl). AC-006/AC-007's "real Neon `/sql` endpoint" work is gated on
   building a real `SqlTransport`, which is schedule-relevant scope not
   currently reflected as a separate line in the ACs.
