# Phase 17 — The breaking window, released as `0.4.0`

**Goal.** Every change to a published crate that 1.0 needs and that breaks a
signature, decided and landed in one release.

**Why here.** Under `0.x`, `cargo-semver-checks` reports a break and the minor
version absorbs it; after `1.0.0` the same break costs a major and every
downstream manifest. Several decisions the roadmap would otherwise make later —
ADR-0028's and half of ADR-0026's — have an answer that adds or changes a method
on a published trait, so they are made here, before the phases that implement
them. One release rather than several, because each breaking minor asks every
adopter to move once.

**Split at the release** by
[ADR-0072](../../.kb/decisions/0072-phase-17-is-split-at-the-release.md). This phase
keeps every item whose answer changes a published signature or behaviour, and the
`workerd` job whose measurement sets published constants; it ends at `0.4.0`. The
additive items — `QueryItem`'s total constructor, VT-14, VT-30, ES-7 with its
floating-dependency job, the minimal-versions job, CF-40's `MetadataLen`, and the
tuple and `then(&[])` closures — are [phase 17b](17b-after-the-window.md)'s. An item
whose answer turns out to break a published crate comes back here.

**Decisions it settles.** ADR-0028 (moved from phase 14). The published-surface
half of ADR-0026. The `Projection::apply` record the port's freeze left owed.
Whatever phase 16 classified as breaking (ADR-0066), the ES-11 record that
supersedes ADR-0061's keep, and ADR-0022 §9, which ADR-0068 left undecided and
gave to this phase.

**Work**

- [x] **ADR-0028 — what a store may forget, and how it says so.** Settled by
      [ADR-0028](../../.kb/decisions/0028-what-a-store-may-forget.md): the written
      refusal, with an additive reservation, on a compiling provided-method spike
      (`experiments/provided-method-spike/`). Nothing published changes in `0.4.0`.
      ES-41 and PS-22 are frozen. Phase 14 builds the instrument and freezes the rest.
      The item read: ~~Either a port surface through which a store reports history
      it no longer holds (a method on `EventStore`, which breaks all four published
      adapters) or a written refusal stating what a store that has been deleted from
      may look like. Phase 14 then builds the suffix store and the rules against
      whichever this is.~~
- [x] **ADR-0026's published half — a foreign identity's write path.** Settled
      by ADR-0073 against a compiling SQLite spike: the adapter's own row writer,
      core unchanged, VT-10 frozen.
      `crates/happenstance-sync/src/ingest.rs` holds four `todo!()` bodies
      blocked on `happenstance-core` having no write path that preserves a foreign
      `EventId` (VT-10). Decide whether core grows one, or each adapter does, or
      `IngestStore` works without one — before 1.0, because the first two touch
      published crates. Settle it against a compiling spike, not an argument.
- [x] **`Projection::apply`** — settled by
      [ADR-0074](../../.kb/decisions/0074-projection-apply-is-async.md) against a
      compiled and executed spike (`experiments/apply-shape/`): `async`, on the one
      trait, with a `trait_variant`-derived `SendProjection`, handed a
      position-free `Delivered` event and the batch. SY-21 is reworded to the
      *local* position. Phase 18 implements it. The item read: ~~synchronous,
      asynchronous, or given a batch handle it can issue statements through
      (`.kb/open-questions/projection-apply-is-synchronous-against-a-live-store.md`).
      Phase 18 implements it. The record names SY-21 — a convergent projection is
      handed an `EventId` and never a `SequencePosition` — because
      `apply`'s arguments are that clause's surface, and phase 13, which owns the
      rule, runs too late to shape a signature this phase decides.~~
- [ ] **The breaking open questions phase 16 listed**, each answered in its own
      record or closed with a reason. Phase 16 classified every candidate the
      split named (ADR-0066), and these are the ones whose answer can change a
      published signature, or that 1.0 cannot promise around without an answer.
      In `.kb/open-questions/`:
      - `should-codec-be-sealed` — sealing a public trait after 1.0 is a major.
      - `projection-batch-sql-seam-statement-type` — the statement type a SQL
        batch exposes becomes a published promise the moment the runner ungates.
      - `projection-id-is-unvalidated`, **with SY-31's reserved `sync/` prefix**:
        refusing an id that is valid today is a break to `happenstance-core`, so
        the sync runner's reservation is decided here, not at phase 13.
      - `es-17-two-adapter-measurement-is-unscheduled` — take ADR-0055's restated
        measurement and act on it, or freeze `&[Event]` by a record. ES-17 below.
      - `no-fixture-tolerance-for-transient-contention` — a `Busy` variant on the
        `#[non_exhaustive]` `AppendError` is additive to add, but whether 1.0
        promises one is decided in this window, not after it.
      - `cf-23-emitter-names-mandatory-and-marked-unstable` — phase 16 decided
        the policy (ADR-0066); the renames it implies land here.
      - `es-6-names-an-unwritable-rule`, sub-question 4 — already decided by
        ADR-0066: driver error payloads re-exported under ADR-0044 are inside the
        promise, under the driver-major limit. What is left is writing that into
        ES-6's prose.

      **Not here, and on purpose.** Three of the split's candidates were
      classified additive. `read-page-budget-is-unspecified` goes after 1.0.
      `trait-variant-caret-resolves-past-the-locked-gate` is answered by a pin, an
      assertion or a CI job rather than a signature; ES-7's freeze rides the
      record that answers it, and ADR-0072 moved that record to phase 17b. So did
      `then-empty-emission-idiom-and-the-nothing-to-do-channel` and
      `tuple-boundary-heterogeneous-event-type`, whose recommended answers break
      nothing.
      `cloudflare-worker-feature-gate` was closed at phase 16: the crate has no
      features table to gate. And `adapter-version-lockstep-and-cf-32` was closed
      by ADR-0066's versioning section rather than sent on.
- [ ] **ADR-0022 §9's reproduction** (ADR-0068). A store built on one runtime
      and read after that runtime is gone, against both adapters that capture a
      runtime `Handle` at construction — `happenstance-sqlite`
      (`crates/happenstance-sqlite/src/event_store.rs:512`,
      `projection_store.rs:234`) and `happenstance-postgres`
      (`crates/happenstance-postgres/src/event_store.rs:314`). About twenty lines,
      and they decide the classification: if the remedy changes what the existing
      `open` / `new` capture, or which variant a stranded read reports, it is a
      behaviour change on two published adapters and lands in `0.4.0`; if it is a
      new constructor, it is additive and may land after 1.0 (ADR-0058). Either
      way the reproduction is written here, and
      `.kb/open-questions/adr-0022-falsifiers-have-fired.md` closes for §9.
- [ ] **The guard-plan assertion ADR-0068 left owed.** An adapter test in
      `happenstance-sqlite` that runs `EXPLAIN QUERY PLAN` on the multi-tag
      append-condition guard, on the SQLite the crate bundles, and fails on a
      `LIST SUBQUERY` — the first limb of ADR-0068's replacement falsifier for
      ADR-0022 §8, which until this test exists fires only by hand. The read
      path's plan is already asserted (`event_store.rs:2418`); the guard's is not.
      Additive, and it rides this window because ADR-0068 named no other owner.
      (ES-27's `Rejects:` repair, the other thing ADR-0068 left owed, landed at
      phase 16.)
- [ ] ~~**ADR-0069's total `QueryItem` constructor.**~~ Moved to
      [phase 17b](17b-after-the-window.md) by ADR-0072: additive.
- [ ] **VT-6 for Postgres and Neon: mint-per-open, or not**
      (`.kb/open-questions/postgres-neon-store-id-has-no-restore-detection.md`).
      Phase 13 closes the restore gap, but it runs after this window, and
      mint-per-open is the one remedy that changes behaviour on a published
      crate. So the choice between it and the additive remedies — a re-mint
      method, detection, a documented procedure — is taken here. If neither
      adapter takes mint-per-open, the session log says so and phase 13 builds
      only the additive arms; mint-per-open after this window is a post-1.0
      major.
- [ ] **Execute ADR-0057 — the testkit version key is dropped.** Accepted and not
      done: the workspace still declares `happenstance-testkit = { version =
      "0.3.2", … }` at `Cargo.toml:62`. ADR-0066 states conformance as *"passes
      `happenstance-testkit` X.Y"*, which is a sentence about how an adapter names
      the testkit it ran, so the key goes in the breaking release rather than
      after it.
- [ ] **Remove `happenstance-core`'s empty `unstable-projection` feature**
      (`crates/happenstance-core/Cargo.toml:72`). It gates nothing, and ADR-0063
      §2 kept it only because removing a Cargo feature is a break
      (`references/adr/0063-the-projection-port-is-frozen.md:48-56`). ADR-0066
      puts the removal in `0.4.0`, which is why it is not on 1.0's
      semver-exemption list. The `xtask` tests ADR-0063 inverted to hold the name —
      `the_retired_projection_feature_is_still_declared_and_empty` among them —
      are turned round again to hold its absence, not deleted.
- [ ] **`happenstance-postgres`'s `naive-arm`**
      (`crates/happenstance-postgres/Cargo.toml:125`): removed, or declared
      outside semver in the crate root and on ADR-0066's exemption list. A
      stranger can turn it on, so 1.0 either promises it or says in writing that
      it does not.
- [ ] **The ES-11 record, settling ES-11 and ES-12 together.** It supersedes
      ADR-0061's choice to keep ES-11 `[PROVISIONAL]` — a reasonable choice while
      `happenstance-neon` was held out of the release set, and not one that
      survives Neon being one of 1.0's nine crates. It decides whether Neon's
      conformance claim carries ES-11 as a named, documented exception, or whether
      a one-shot-HTTP shape that satisfies ES-11 exists; ES-12 is frozen on the
      same axis, with `query_items_share_one_snapshot` red on a one-shot-HTTP
      adapter as its falsifier.
- [ ] **A `workerd` sibling job** in `.github/workflows/ci.yml`, shaped like
      `live-postgres` and `live-neon`: a sibling of `gate`, never a step inside it
      (`.kb/open-questions/no-workerd-class-runner-in-the-gate.md`). ADR-0066 makes
      `happenstance-cloudflare`'s 1.0 claim *conformance on the real runtime*, and
      today the rules run against a `node:sqlite` shim
      (`crates/happenstance-cloudflare/README.md:106-113`). Two measurements ride
      on the job and are taken before either is promised: **the SQL-text wall** —
      the statement length at which a Durable Object refuses — and **ADR-0052's
      partition widths**, the public `MAX_QUERY_ARMS_PER_STATEMENT` and
      `MAX_QUERY_PARAMETERS_PER_STATEMENT`, measured so far only against the shim.
- [ ] ~~**A minimal-versions CI job.**~~ Moved to
      [phase 17b](17b-after-the-window.md) by ADR-0072: it changes no surface.
- [ ] **The clause follow-ups phase 16 gave this phase.** Each `freeze-by-17` row
      in [the 1.0 dispositions](../ledgers.md), and what freezes it:
      - **VT-10** — the foreign-identity spike above, with SQLite implementing
        `IngestStore` beside `append`. It must not foreclose SY-14's
        bounded-round-trip ingest, which phase 13 measures.
      - ~~**VT-14**, **VT-30**, **ES-7**~~ — `freeze-by-17b` since ADR-0072; each
        is additive under its recommended answer.
      - **ES-11, ES-12** — the ES-11 record above.
      - **ES-17** — the measurement or the freezing record, above.
      - ~~**ES-41** — with ADR-0028. The transport half is already answered: Neon
        and Cloudflare each probe membership in one read-only round trip over the
        pair VT-8 indexes.~~ **Frozen by ADR-0028.**
      - ~~**PS-9, PS-11** — together, in the `Projection::apply` record, with the
        failure-policy seam's shape. Phase 18 confirms both by building PS-27's
        seam without a generic write.~~ **Frozen by ADR-0074**, on the spike's
        three executed legs: a provided `on_error` under `trait_variant`, and a
        skip row through `SqliteBatch` and a live `LivePostgresBatch` with no
        bound on `Batch`.
      - ~~**PS-15** — decide whether `rollback` refuses a foreign batch (breaking),
        or narrow the MUST to `commit` and `reset`.~~ **Narrowed and frozen by
        ADR-0075**, by ADR-0066's own narrowing route. The MUST covers `commit`
        and `reset`; what the clause says of a foreign `rollback` is non-normative.
        Frozen because no other disposition was valid: its falsifier could fire only
        as a major, and no phase before 1.0 has an instrument for it.
      - ~~**PS-22** — ADR-0028 states that retention never rewinds a checkpoint
        over kept rows.~~ **Frozen by ADR-0028**, which states it.
      - ~~**PS-23** — freeze *exactly one* id per commit, or record that a multi-id
        atomic commit arrives after 1.0 as an additive defaulted method rather
        than a change to `commit`.~~ **Frozen by ADR-0075**, both at once: one id
        per commit is the promise, and a multi-id commit arrives, if ever, as a
        refusing provided `commit_all` (`experiments/provided-method-spike/`,
        which compiled it with a `CommitError` variant; ADR-0075 chooses its own
        error type, not yet compiled).
      - ~~**PS-24** — freeze `Authority::Rebuilding` as a kept variant. Phase 18
        decides whether the typed runner ever emits it.~~ **Frozen by ADR-0075**
        as a kept variant.

      And the decisions taken here for clauses another phase freezes: **ES-39,
      ES-40, SY-32 and CF-27** (ADR-0028's shape; phase 14 builds and freezes);
      **SY-21** (inside the `apply` record, which must name it; phase 18 freezes —
      **decided by ADR-0074** and reworded to the arrival position);
      **PS-25** (the derived-id remedy or the digest-in-checkpoint one — the
      second changes the frozen port's `commit`, so the choice is made here —
      **ADR-0074 chose the derived id**, with a 64-bit FNV-1a digest and an
      exposed `checkpoint_id`);
      **CF-40**'s `MetadataLen` build under ADR-0043 moved to 17b with ADR-0072
      (phase 13 then decides the budget unit); and **PS-38**'s documented no-lagging-replica obligation
      (phase 18 freezes it with PS-23) — **written by ADR-0075** into
      `ProjectionStore::checkpoint`'s rustdoc and `happenstance-neon`'s README and
      constructor.
- [ ] **Release `0.4.0`.** `cargo-semver-checks` against the `0.3.x` registry
      baseline reports breaks, and each one it reports traces to a decision above.
      `CHANGELOG.md`'s `[Unreleased]` entries — SQLite's fifteen-second busy
      timeout (ADR-0065) and `FaultyStore::contend_next` — ship in `0.4.0`;
      there is no `0.3.3` (`wi-052920`).

**Proof artefact.** `0.4.0` on crates.io, and a table in its changelog entry
mapping every major finding `cargo-semver-checks` reported to the decision that
caused it — a break with no row is one nobody decided.

**Exit criteria**

- [x] ADR-0028 accepted, and no breaking retention surface is owed after
      `0.4.0`: any report of unheld history is a provided method defaulting to
      `Unknown` (`experiments/provided-method-spike`). ES-39 keeps phase 16's
      disposition, `freeze-by-14`, and is settled there. *(Amended 2026-09-29
      by the owner. The original text, "ES-39 is no longer `[DEFERRED]` on a
      surface 1.0 promises, or is renewed past 1.0 with the disposition phase 16
      gave it", was written at the runbook split, before phase 16 dispositioned
      any clause, and anticipated a renewal that phase 16 did not give.)*
- [x] The foreign-identity question is answered against a compiling spike
      (ADR-0073).
- [ ] ADR-0022 §9's reproduction runs against `happenstance-sqlite` and
      `happenstance-postgres`, and a record classifies its remedy as additive or
      breaking; a breaking remedy has landed.
- [ ] The guard-plan `LIST SUBQUERY` assertion (ADR-0068) is in the tree.
      (`QueryItem`'s total constructor moved to 17b with ADR-0072.)
- [x] The `apply` record is accepted (ADR-0074).
- [ ] Every open question phase 16 classified as breaking is answered or closed.
- [ ] `0.4.0` is released and its semver findings are fully traced.
- [ ] The `workerd` job exists, has been watched failing once, and the SQL-text
      wall and partition widths are recorded as measured on `workerd`, locally
      and on a deployed Durable Object. (The minimal-versions job moved to 17b
      with ADR-0072.)
- [ ] Every `freeze-by-17` clause in [`ledgers.md`](../ledgers.md)'s 1.0
      dispositions is `[FROZEN]`, or re-dispositioned by a record that says why.
      VT-14, VT-30 and ES-7 were re-dispositioned to `freeze-by-17b` by ADR-0072.
- [ ] The specification is reconciled against this phase's changes (session
      protocol step 6), and `cargo xtask spec-trace` passes.

**Estimate.** ~~5–8 days.~~ **25–30 days**, re-estimated at the phase's start
(2026-09-29) after phase 16 added the `workerd` job, ADR-0022 §9, ADR-0069's
constructor and VT-6 to it. A read-only research pass over every item put the
unsplit phase at about 275 hours; ADR-0072 moved the additive part, 8–10 days of
it, to 17b. The spread is the `workerd` harness, `Busy` across three adapters,
the ES-11 fence spike on Neon, and whether ES-17's measurement changes `append`.

**Session log**

- 2026-09-29 — Started. Re-estimated from a read-only research pass over every
  item (one reader per cluster, then a sequencing synthesis). The owner chose to
  split the phase at its release (ADR-0072, phase 17b), to spike the ES-11 fence
  on Neon rather than take a named exception, and to promise `AppendError::Busy`
  with the typed commit loop retrying it inside `Retry`. The owner is providing a
  `NEON_CONNECTION` secret, a Cloudflare API token for a deployed Durable Object
  leg, and Docker for testcontainers Postgres. Work runs in lanes, one PR each:
  kickoff; VT-10; the provided-method spike with ADR-0028; the `apply` and
  port-clause records; the renames and manifest breaks; `Busy`; the `workerd`
  job, then Cloudflare's partition; ES-17; ES-11 on Neon; ADR-0022 §9;
  `ProjectionId`; the codec; the release.

- 2026-09-29 — **L1, VT-10's foreign-identity spike**, on
  `lane/p17-foreign-identity`. ADR-0073: the write path that keeps a foreign
  `EventId` is the adapter's own row writer, and `happenstance-core` grows
  nothing. VT-10's falsifier ran on its named instrument and did not fire:
  `impl SendIngestStore for SqliteEventStore` (`#[cfg(test)]`, with a path-only
  dev-dependency on the unpublished `happenstance-sync`) goes through the one
  `write_batch` `append` calls, and the ingest-only code is a transaction frame, a
  watermark query and a foreign row's bound values. VT-10 is `[FROZEN]`. Neon
  builds a whole ingest batch as one statement from its append's own builders,
  which is SY-14 evidence; it is structural and was not executed. In
  `happenstance-sync` the placeholder identity types are gone (a `u64`
  `RecordedAt` fired VT-9's restated falsifier by construction, so phase 13's item
  was pulled forward), `ingest` takes `IngestGroup`s carrying compensation, and
  `holds` is dropped for `EventStore::contains_event_id`. SY-2's example is read
  as an `IngestGroup`. Pinned for phase 13: an unheld event claiming this
  store's own `StoreId` is ingested, then wedges the local append that reaches
  its position. Done as a workflow: sync types, the two spikes, then three
  adversarial reviewers, correctness, falsifier honesty and repo rules, whose seven
  findings were all fixed. Among them: a saturating origin-position conversion, a
  Neon ingest statement that first duplicated append's, and a marker test that
  could not fail. Spec citations shifted by the edits were repointed across the
  tree, `.kb/_governance` excepted.

- 2026-09-29 — **L2, the provided-method spike and ADR-0028**, on
  `lane/p17-provided-method-spike`. The spike (`experiments/provided-method-spike/`)
  added three provided methods returning `impl Future`, not `async fn`, on a scratch
  worktree of `52aa951`. `EventStore::history`, defaulting to `Unknown`, passed all
  three criteria: every implementor compiled, Send and `!Send`, host and wasm32; it
  spawned from `S: SendEventStore + 'static` with no `Sync`; and `cargo-semver-checks`
  0.50.0 reported nothing against `0.3.2` for `happenstance-core` or `happenstance`,
  while a required-method control reported a major `trait_method_added`.
  `ProjectionStore::commit_all` passed only when the default drops the batch
  before the future exists. The typed `Projection`, a plain trait, cannot be
  spawned generically. ADR-0028 therefore takes the refusal with an additive
  reservation:
  - deletion stays outside the port through 1.x (ES-37 re-dated);
  - ES-38 and ES-40's MAY stand, and E2E-47's third outcome is rejected for 1.x;
  - the floor primitive is rejected;
  - ES-41 and PS-22 are `[FROZEN]`;
  - SY-32's floor is narrowed to resumability, CF-27's report is instrument-local,
    and ES-39 is rewritten and stays `[DEFERRED]` for phase 14.

  ES-41's freeze does **not** fix whether "holds" means held or visible under
  ES-10's frontier: a first draft froze "held" on the argument that `false` would
  let ingest duplicate, which `crates/happenstance-sync/src/ingest.rs:97-101`
  contradicts, and no rule stages the row. The reading is phase 13's work item,
  and Postgres's and Neon's notes still say "recorded, not settled". Neon meets
  the marker's narrow transport falsifier to the letter, and ES-41 now says so and
  why the round trip is accepted. The ES-38 open question's sub-question 3 is
  answered, and `phases/14-retention.md` is restated, its exit criterion and
  proof artefact included. Exit criterion 1 above is left unticked with a
  proposed reading for the owner.

- 2026-09-29 — **Exit criterion 1 amended and ticked, by the owner's ruling.**
  Its second arm ("renewed past 1.0 with the disposition phase 16 gave it")
  predated phase 16's dispositions and assumed a renewal. Phase 16 gave ES-39
  `freeze-by-14`. The criterion now states what it protected: that no breaking
  retention surface is owed after `0.4.0`, which ADR-0028 and the
  provided-method spike discharge. No clause, marker or ledger row moved, and
  `cargo xtask lints` still holds ES-39 to phase 14.

- 2026-09-29 — **L3, the `apply` record and the port-clauses record**, on
  `lane/p17-apply-record`. Records, specification, runbook and documentation only;
  no Rust signature changed. **ADR-0074**: `Projection::apply` becomes `async` on
  the one trait, handed a `Delivered<E>` with `id()` and no local position, and
  the batch. The spike (`experiments/apply-shape/`, merged with L2) chose the
  `Send` mechanism: **`trait_variant`**, because return-type notation is `E0658`
  on 1.97.1. SY-21 is reworded to the arrival position, `SequencedEvent::position`,
  because `EventId::position()` is the origin's. The failure-policy seam is a provided
  `on_error` defaulting to halt. A skip after a server-side failure on a live
  batch needs a savepoint, which phase 18 owes. PS-9 and PS-11 are frozen: all
  three legs ran — the provided method compiled, and skip rows were written through
  `SqliteBatch` and a live `LivePostgresBatch` with no bound on `Batch`. PS-28's
  `ProjectionError<R, W, A>` and PS-25's derived id are phase 18's to build.
  **ADR-0075**: PS-15 narrowed to `commit` and `reset` and frozen, PS-23 frozen
  on the additive `commit_all` route with its own error type, PS-24 frozen as a
  kept variant, and PS-38's obligation written into `ProjectionStore::checkpoint`
  and `happenstance-neon`'s README and constructor. PS-15's narrowing is the route
  ADR-0066 prescribed; the plan's note to keep it provisional was not a valid
  disposition, because its falsifier fires only as a major and no phase before 1.0
  has an instrument. A review pass on the same branch then took the rollback
  sentence out of PS-15's MUST, since no rule checks it; said that the spike
  compiled `commit_all` with a `CommitError` variant and that the separate error
  type is ADR-0075's uncompiled choice; marked the `checkpoint` rustdoc as PS-38's
  provisional reading; recorded the runner-issued savepoint as a PS-9 candidate
  declined by design; and respelled ADR-0074's trait sketch with explicit
  projections, because the `StoreBatch<Self>` aliases fail to compile inside a
  `trait_variant` trait (six `E0277`s, `Self: Sized`). The open question
  `projection-apply-is-synchronous-against-a-live-store` is superseded, and
  `phases/18-typed-runner.md` is restated to build what ADR-0074 decided. Spec
  citations shifted by the edits were repointed across the tree.
