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
- [x] **The breaking open questions phase 16 listed**, each answered in its own
      record or closed with a reason. Phase 16 classified every candidate the
      split named (ADR-0066), and these are the ones whose answer can change a
      published signature, or that 1.0 cannot promise around without an answer.
      In `.kb/open-questions/`:
      - `should-codec-be-sealed` — sealing a public trait after 1.0 is a major.
        Answered by [ADR-0083](../../.kb/decisions/0083-codec-stays-unsealed-through-1-x.md),
        accepted by the owner on 2026-10-08.
      - `projection-batch-sql-seam-statement-type` (ADR-0084 accepted, #44; Neon's `push` narrowed) — the statement type a SQL
        batch exposes becomes a published promise the moment the runner ungates.
        Answered by [ADR-0084](../../.kb/decisions/0084-the-projection-batch-sql-seam-is-final.md),
        accepted by the owner on 2026-10-08.
      - `projection-id-is-unvalidated` (closed by ADR-0082, lane L10), **with SY-31's reserved `sync/` prefix**:
        refusing an id that is valid today is a break to `happenstance-core`, so
        the sync runner's reservation is decided here, not at phase 13.
      - ~~`es-17-two-adapter-measurement-is-unscheduled` — take ADR-0055's restated
        measurement and act on it, or freeze `&[Event]` by a record.~~ **Settled by [ADR-0080](../../.kb/decisions/0080-append-keeps-a-borrowed-batch.md)**: measured, `&[Event]` frozen. ES-17 below.
      - ~~`no-fixture-tolerance-for-transient-contention` — a `Busy` variant on the
        `#[non_exhaustive]` `AppendError` is additive to add, but whether 1.0
        promises one is decided in this window, not after it.~~ **Settled by
        [ADR-0077](../../.kb/decisions/0077-appenderror-busy.md)** in lane L5:
        `AppendError::Busy` is promised, the typed loop retries it inside
        `Retry`, SQLite, Postgres and Neon report it, and ES-43 is `[FROZEN]`
        with its freeze condition met. The question is superseded.
      - ~~`cf-23-emitter-names-mandatory-and-marked-unstable` — phase 16 decided
        the policy (ADR-0066); the renames it implies land here.~~ **Settled by
        [ADR-0076](../../.kb/decisions/0076-the-cf-23-emitters-are-public-api.md)**:
        the ten conformance emitters are un-hidden and renamed without `__`
        (`emit_tokio` and its siblings), CF-41 `[FROZEN]` pins them, and the
        question is superseded.
      - ~~`es-6-names-an-unwritable-rule`, sub-question 4 — already decided by
        ADR-0066: driver error payloads re-exported under ADR-0044 are inside the
        promise, under the driver-major limit. What is left is writing that into
        ES-6's prose.~~ **Written** in lane L5: ES-6's payload paragraph states the
        promise for `Store(E)` and `Busy(E)` alike, with its census. Sub-questions
        1–3 stay open in the atom, and are not this window's.

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
- [x] **ADR-0022 §9's reproduction** (ADR-0068; reproduced, ADR-0081 accepted 2026-10-08). A store built on one runtime
      and read after that runtime is gone, against both adapters that capture a
      runtime `Handle` at construction — `happenstance-sqlite`
      (`crates/happenstance-sqlite/src/event_store.rs:513`,
      `projection_store.rs:234`) and `happenstance-postgres`
      (`crates/happenstance-postgres/src/event_store.rs:337`). About twenty lines,
      and they decide the classification: if the remedy changes what the existing
      `open` / `new` capture, or which variant a stranded read reports, it is a
      behaviour change on two published adapters and lands in `0.4.0`; if it is a
      new constructor, it is additive and may land after 1.0 (ADR-0058). Either
      way the reproduction is written here, and
      `.kb/open-questions/adr-0022-falsifiers-have-fired.md` closes for §9.
- [x] **The guard-plan assertion ADR-0068 left owed.** An adapter test in
      `happenstance-sqlite` that runs `EXPLAIN QUERY PLAN` on the multi-tag
      append-condition guard, on the SQLite the crate bundles, and fails on a
      `LIST SUBQUERY` — the first limb of ADR-0068's replacement falsifier for
      ADR-0022 §8, which until this test exists fires only by hand. The read
      path's plan is asserted (`event_store.rs:2939`), the guard's at `:3191`.
      Additive, and it rides this window because ADR-0068 named no other owner.
      (ES-27's `Rejects:` repair, the other thing ADR-0068 left owed, landed at
      phase 16.)
- [ ] ~~**ADR-0069's total `QueryItem` constructor.**~~ Moved to
      [phase 17b](17b-after-the-window.md) by ADR-0072: additive.
- [x] **VT-6 for Postgres and Neon: mint-per-open, or not**
      (`.kb/open-questions/postgres-neon-store-id-has-no-restore-detection.md`).
      Phase 13 closes the restore gap, but it runs after this window, and
      mint-per-open is the one remedy that changes behaviour on a published
      crate. So the choice between it and the additive remedies — a re-mint
      method, detection, a documented procedure — is taken here. If neither
      adapter takes mint-per-open, the session log says so and phase 13 builds
      only the additive arms; mint-per-open after this window is a post-1.0
      major.
      Answered by [ADR-0086](../../.kb/decisions/0086-postgres-and-neon-keep-mint-once.md),
      accepted by the owner on 2026-10-08: neither adapter mints per open, so phase 13
      builds only the additive arms.
- [x] **Execute ADR-0057 — the testkit version key is dropped.** Done in lane L4:
      the workspace entry for `happenstance-testkit` carries no `version`, and
      `cargo xtask package-check` refuses a publishable crate whose testkit
      dev-dependency carries one, written or inherited. The item read: ~~Accepted
      and not done: the workspace still declares `happenstance-testkit = { version =
      "0.3.2", … }` at the root manifest's testkit line. ADR-0066 states conformance
      as *"passes `happenstance-testkit` X.Y"*, which is a sentence about how an
      adapter names the testkit it ran, so the key goes in the breaking release
      rather than after it.~~
- [x] **Remove `happenstance-core`'s empty `unstable-projection` feature.** Done in
      lane L4; the `xtask` test is now `the_retired_projection_feature_is_gone`.
      The item read: ~~It was declared in `happenstance-core`'s `[features]`. It
      gates nothing, and ADR-0063
      §2 kept it only because removing a Cargo feature is a break
      (`references/adr/0063-the-projection-port-is-frozen.md:48-56`). ADR-0066
      puts the removal in `0.4.0`, which is why it is not on 1.0's
      semver-exemption list. The `xtask` tests ADR-0063 inverted to hold the name —
      `the_retired_projection_feature_is_still_declared_and_empty` among them —
      are turned round again to hold its absence, not deleted.~~
- [x] **`happenstance-postgres`'s `naive-arm`.** Removed as a feature in lane L4:
      it is the rustc cfg `happenstance_naive_arm`, which no manifest can set, the
      crate root says it is not API, and CI's live-postgres job runs both targets
      under it. The item read: ~~removed, or declared outside semver in the crate
      root and on ADR-0066's exemption list. A stranger can turn it on, so 1.0
      either promises it or says in writing that it does not.~~
- [x] **The ES-11 record, settling ES-11 and ES-12 together** (ADR-0087, accepted 2026-10-08: the fence held, 0 red of 1,500). It supersedes
      ADR-0061's choice to keep ES-11 `[PROVISIONAL]` — a reasonable choice while
      `happenstance-neon` was held out of the release set, and not one that
      survives Neon being one of 1.0's nine crates. It decides whether Neon's
      conformance claim carries ES-11 as a named, documented exception, or whether
      a one-shot-HTTP shape that satisfies ES-11 exists; ES-12 is frozen on the
      same axis, with `query_items_share_one_snapshot` red on a one-shot-HTTP
      adapter as its falsifier (fired: CI run 37504851570). When it lands, the owner re-adds `conformance against a live Neon endpoint` to the `Protect main` ruleset (id 22926481) and confirms it is required again (`wi-0f1291`).
- [x] **A `workerd` sibling job** in `.github/workflows/ci.yml`, shaped like
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
- [x] **The clause follow-ups phase 16 gave this phase.** Each `freeze-by-17` row
      in [the 1.0 dispositions](../ledgers.md), and what freezes it:
      - **VT-10** — the foreign-identity spike above, with SQLite implementing
        `IngestStore` beside `append`. It must not foreclose SY-14's
        bounded-round-trip ingest, which phase 13 measures.
      - ~~**VT-14**, **VT-30**, **ES-7**~~ — `freeze-by-17b` since ADR-0072; each
        is additive under its recommended answer.
      - **ES-11, ES-12** — the ES-11 record above.
      - ~~**ES-17** — the measurement or the freezing record, above.~~ **Frozen by ADR-0080.**
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
- [ ] #87 · **Release `0.4.0`.** `cargo-semver-checks` against the `0.3.x` registry
      baseline reports breaks, and each one it reports traces to a decision above.
      `CHANGELOG.md`'s `[Unreleased]` entries — SQLite's fifteen-second busy
      timeout (ADR-0065) and `FaultyStore::contend_next` — ship in `0.4.0`;
      there is no `0.3.3` (`wi-052920`).

**Proof artefact.** `0.4.0` on crates.io, and a table in its changelog entry
mapping every major finding `cargo-semver-checks` reported to the decision that
caused it — a break with no row is one nobody decided.

The run that fills the table is `cargo semver-checks check-release --workspace
--baseline-version 0.3.2 --release-type minor`. The last flag is required, not a
preference. Once the manifests read `0.4.0`, the tool treats `0.3.2 → 0.4.0` as a
major bump and skips every lint (`0 checks: 0 pass, 254 skip`). The default
invocation, and CI's `cargo-semver-checks-action` with default settings, then
report nothing, and the table would come out empty. Forcing `minor` makes the tool
report each break as though it were not allowed. The table also carries
hand-written rows for the breaks the tool does not report: `happenstance-core`'s
removed `unstable-projection` feature, which the `0.3.2` manifest declares but the
tool passed over, and the renamed `#[doc(hidden)]` emitters. The tool does report
`naive-arm` and `PostgresEventStore::new_naive` (`feature_missing`,
`inherent_method_missing`), and those rows trace to the `naive-arm` item above.

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
- [x] ADR-0022 §9's reproduction runs against `happenstance-sqlite` and
      `happenstance-postgres`, and a record classifies its remedy as additive or
      breaking; a breaking remedy has landed.
- [x] The guard-plan `LIST SUBQUERY` assertion (ADR-0068) is in the tree.
      (`QueryItem`'s total constructor moved to 17b with ADR-0072.)
- [x] The `apply` record is accepted (ADR-0074).
- [x] Every open question phase 16 classified as breaking is answered or closed.
- [ ] `0.4.0` is released and its semver findings are fully traced.
- [x] The `workerd` job exists, has been watched failing once, and the SQL-text
      wall and partition widths are recorded as measured on `workerd`, locally
      and on a deployed Durable Object. (The minimal-versions job moved to 17b
      with ADR-0072.)
- [x] Every `freeze-by-17` clause in [`ledgers.md`](../ledgers.md)'s 1.0
      dispositions is `[FROZEN]`, or re-dispositioned by a record that says why.
      VT-14, VT-30 and ES-7 were re-dispositioned to `freeze-by-17b` by ADR-0072.
- [x] The specification is reconciled against this phase's changes (session
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

- 2026-09-30 — **L4, the surface renames and the manifest breaks**, on
  `lane/p17-surface-renames`. The first breaking PR, so the workspace, the testkit
  and the six examples move to `0.4.0`; the changelog heading stays
  `[Unreleased]`, and the README and install lines wait for the release PR.
  **Stage 1, manifests.** ADR-0057 is executed: the workspace's testkit entry
  carries no `version`, and `package-check` gains
  `testkit_dev_dependencies_are_versionless`, with negative controls built from
  the old Postgres and Cloudflare lines. `happenstance-core`'s empty
  `unstable-projection` is removed, and the `xtask` test is turned round to
  `the_retired_projection_feature_is_gone`. `benchmarks/` asked for it and no
  longer does. `happenstance-postgres`'s `naive-arm` is now the rustc cfg
  `happenstance_naive_arm`, declared in `[workspace.lints.rust]`'s check-cfg; the
  crate root says it is not API, and a new last step in CI's live-postgres job
  runs both naive-arm targets under it. That step had not run when this was
  written. **Stage 2, CF-23.**
  [ADR-0076](../../.kb/decisions/0076-the-cf-23-emitters-are-public-api.md)
  un-hides the ten conformance emitters and drops their `__` prefix
  (`emit_tokio` and its siblings), as a hard rename with no aliases.
  `__emit_rule_names` becomes `__rule_names`. It and the benchmark pair stay
  hidden and outside the promise. ADR-0076 partly supersedes ADR-0066 §5, whose
  row on the decision map is annotated. CF-41 is minted `[FROZEN]` in §6.6. Its
  rule is `the_promised_emitters_are_exactly_the_pinned_list`, which checks
  against a committed list, with two negative controls. The cf-23 open question is
  superseded. `happenstance-ladybug`, touched and outside CI, was built with its
  driver and ran its projection suite locally (42 passed). The `0.4.0` semver trace
  owes hand-written rows for two changes `cargo-semver-checks` does not report:
  core's removed `unstable-projection` feature and the hidden emitter names.
  `naive-arm` and `new_naive` are reported by the tool, but only under
  `--release-type minor`. At `0.4.0` its default skips every lint, so the proof
  artefact now names the flag.
  Citations shifted by both stages were repointed.

- 2026-09-30 — **L5, `AppendError::Busy`**, on `lane/p17-busy`. Three stages.
  **Stage 1, the contract.**
  [ADR-0077](../../.kb/decisions/0077-appenderror-busy.md) records the owner's two
  decisions: 1.0 promises `AppendError::Busy(E)`, and `happenstance`'s commit loop
  retries it inside the same `Retry` bound. The loop re-decides rather than
  re-sending, and `CommandError::Exhausted.source` becomes `AppendError<E>`.
  `Busy` renders its own message, `is_busy()` and a `map_store` arm come with it,
  and `FaultyStore::contend_next` moves to `Busy(Contended)`. ES-43 is minted
  after ES-25, and ES-6's payload paragraph is written.
  **Stage 2, the testkit.** `a_busy_append_left_nothing_behind` joins the
  concurrency family, with `BusyAfterWriteStore` as its mutant, which also fails
  four more rules. The re-spelling reached five racing rules, not three, under
  structural floors, and `k_disjoint` gained a floor of its own: a boundary with no
  winner rejected nobody. `CONTENDERS` stays 64.
  **Stage 3, the adapters.** SQLite reports `Busy` for a `SQLITE_BUSY` or
  `SQLITE_LOCKED` from `BEGIN IMMEDIATE`, and for nothing later. Postgres reports
  it for the last `40001` of an exhausted budget; a deadlock and a lost `COMMIT`
  stay `Store`. Neon reports it for the last `40001` after
  `SERIALISATION_ATTEMPTS`; a transport failure stays `Store`. Cloudflare
  documents why it never reports `Busy`.
  **Verified.** SQLite:
  - a held write lock under a 50 ms timeout is `Busy`, writes nothing, then lands;
    the old `store_error` mapping fails that test;
  - the concurrency family runs in-crate under a 1 ms timeout, green in eleven
    runs; a throwaway count saw about 260 real `Busy` answers per run, 61 of 64 in
    the new rule;
  - the whole crate passes.
  Postgres:
  - the retry loop is split out and tested offline, with a scripted `SQLSTATE`:
    exhaustion is `Busy` after exactly eight attempts, and `40P01` returns at once;
  - the event-store and concurrency suites passed 108 of 108 against a
    `postgres:17.10` container at `--test-threads=1`, as CI runs them. A parallel
    local run exhausts the fixture's pool, so it is not a finding.
  Neon: a scripted transport shows exhaustion is `Busy`, and `23505` is `Store`
  on the first attempt. Its live half is CI's `live-neon` job, because
  `NEON_CONNECTION` is not available locally. ES-43's freeze condition is met, so
  it stays `[FROZEN]` and no ledger row is owed.
- 2026-09-30 — **`happenstance-ladybug` retired**, on the owner's call
  (`wi-630032`): "too many issues". [ADR-0078](../../.kb/decisions/0078-happenstance-ladybug-is-retired.md)
  excludes it from the workspace and keeps the directory as a frozen record,
  because its files are cited by line. `lbug` left `Cargo.lock`. The gate lost its
  two Ladybug steps, the `ladybug-configured` subcommand and four `--exclude`
  arguments. `affected` now honours the manifest's `exclude` key and treats the
  frozen directory as inert. CI lost the frozen `ladybug` job and the `msrv` job's
  `--exclude`. Citations into `xtask/src/main.rs`, `xtask/src/affected.rs` and
  `ci.yml` were repointed. Spec, CLAUDE.md and README edits keep their line counts. Verified: `cargo check --workspace --all-features --all-targets`, `cargo test -p xtask`, xtask clippy `-D warnings`, fmt, the `-D warnings` workspace doc build, `lints`, `lint-kb`, `lint-constitution`, `spec-trace`, `lint-workflows`, and the temper gate green. The full `cargo xtask ci` was not run.
- 2026-10-01 — **Lane L6a: the `workerd` job, landed red on purpose.**
  `harness/workerd` is a new workspace member and is never published. It holds a
  real `#[durable_object]` class that runs one conformance rule by name. The
  dispatch emitter expands `for_each_event_store_rule!` into a `match`, and the
  runner's names come from the same enumeration through `__rule_names`, so a
  dropped rule fails as `no such rule`. Two rules open two isolated stores, and a
  Durable Object has one database. So `happenstance-cloudflare` gains
  `CloudflareEventStore::namespaced` and `TableNamespace`: additive, owner-approved
  in the phase plan, and **a 1.0 promise, flagged in the PR**. The `workerd` CI job
  has a local leg (`@cloudflare/vitest-pool-workers`, lockfile committed) and a
  deployed leg (`CLOUDFLARE_API_TOKEN`).
  **Measured locally** (`experiments/durable-object-limits`):
  - compound `SELECT` 5, bound parameters 100, statement 100,000 B, expression
    depth 100, on workerd 1.20260815.1 and 1.20261001.1;
  - the row wall is 2,199,995 B on the first and 8,388,637 B on the second, so
    the deployed leg settles it;
  - the adapter evaluates at most **5 query items** and **45 tags in one item**.
  **Red, locally:** 96 of 97 pass. The failure is
  `store_evaluates_a_query_at_the_guaranteed_minimum_item_count`: 128 parameters
  against 100. L6b fixes the rendering and the constants.
  **The red run, in CI** (PR #34,
  https://github.com/Wet-Ink-Corporation/happenstance/actions/runs/36957404625/job/110683349880):
  97 collected and 97 executed, 96 passed. The same single failure for the same
  reason; the CI walls match local workerd 1.20260815.1. **The deployed leg did not
  run.** `wrangler deploy` was refused with `Authentication error [code: 10000]`,
  and then `Cannot use the access token from location: 172.212.163.227
  [code: 9109]`: the token carries an IP filter that excludes GitHub's runners, and
  its Workers permissions are still unverified. That is the owner's to fix. Still
  owed: the deployed measurement, and the vacuity control on a throwaway branch.
  **Verified:** `cargo xtask ci` green before review, the temper gate green after
  it, `cargo xtask wasm`, `lints`, `lint-kb`, and three literal-table-name mutants
  each caught by the namespace tests. Findings worth carrying:
  - a panicking rule traps the wasm instance, so the harness relays the panic to
    JavaScript;
  - the Worker's entry point is JavaScript, because a Rust entry point dies with
    the trap.
- 2026-10-02 — **L6a: the deployed leg ran.** After the owner lifted the token's
  IP filter, the deploy succeeded. The Worker then answered "not configured" for
  the whole readiness window, even though `wrangler secret put` reported success,
  and the readiness loop fell through silently. Fixed at `8a0591f8`: the per-run
  token goes in `wrangler deploy --var` (one atomic upload), and the loop fails
  loudly.
  **Run 36966608470**
  (https://github.com/Wet-Ink-Corporation/happenstance/actions/runs/36966608470/job/110711625329):
  96 executed, 93 passed. VT-23 failed as expected. Two rules got `500 Worker not
  found.`, which is Cloudflare's propagation error before any harness code runs.
  `deployed.mjs` now retries exactly that response once, on a fresh object, and
  logs it; a rule's own failure is never retried.
  **Deployed walls:** compound 5, parameters 100, statement 100,000 B, depth 100,
  adapter 5 items and 45 tags per item — all as local. The row wall is
  **8,388,637 B**, the newer workerd's limit rather than the documented 2 MB.
  CF-40's metadata ceiling (17b) is carved from it.
  `experiments/durable-object-limits/results/run-workerd-deployed-2026-10-02.txt`.
- 2026-10-02 — **L6a: both legs red for the one intended reason.** Run
  36967951105
  (https://github.com/Wet-Ink-Corporation/happenstance/actions/runs/36967951105/job/110715683682).
  Local: 97 collected and executed, 96 passed. Deployed: 96 executed, 95 passed,
  no platform retry needed. The only failure on both legs is VT-23's
  `store_evaluates_a_query_at_the_guaranteed_minimum_item_count`. The row walls
  reproduced: 2,199,995 B local, 8,388,637 B deployed. Still owed: the vacuity
  control on a throwaway branch.
- 2026-10-05 — **L6b: the rendering, on branch `lane/p17-workerd-green`** (cut from
  `1f92d088`, where L6a merged red). Not committed; left for review. The owner
  answered D1–D8 of `.temper/plans/p17-l6b-workerd-green.md` in chat, and
  [ADR-0079](../../.kb/decisions/0079-a-query-item-binds-a-constant-number-of-parameters.md)
  records them.
  - **Step 0 first:** a new probe axis showed `json_each(?)` allowed inside a real
    Durable Object, with no wall to 100,000 elements in one parameter.
  - An item's tags and types each travel as one JSON array, so an arm binds 0–3
    parameters and its text does not grow with its width.
    `MAX_QUERY_ARMS_PER_STATEMENT` is 5 and `MAX_QUERY_PARAMETERS_PER_STATEMENT`
    is 90. `planned_statement_count` returns new values (128 one-tag items: 26),
    so the `0.4.0` trace table needs a **hand row** for it beside core's removed
    feature and the emitter renames.
  - The shim opens `node:sqlite` with `workerd`'s four statement limits and fails
    closed, so the gate went red on VT-23 before the fix. Node 22 lacks the
    option, so the gate job gained `actions/setup-node` at `"24"` (the owner's
    conditional allowance; nothing else in `.github/` changed).
  - Local `workerd` 1.20260815.1: 97 of 97 rules, and the probe reports no wall
    to 1,024 items, condition items or tags in one item. The new per-item wall is
    the length of one JSON parameter: 8,527 tags of 255 B.
    `experiments/durable-object-limits/results/run-workerd-local-2026-10-05.txt`.
  - Not yet run: the `workerd` CI job on this branch, both legs. The deployed
    transcript, the run URL and the exit-criterion tick wait for it.
  - The WF-11, CF-14 and CF-17 ledger bases are rewritten; their dispositions are
    unchanged.
- 2026-10-06 — **L6b merged green (#35, `6a3adf6a`).** CI run 37419423991 on
  `32ea2ada`
  (https://github.com/Wet-Ink-Corporation/happenstance/actions/runs/37419423991/job/112125223626):
  local `workerd` 97 of 97 rules and the probe; the deployed object 96 of 96, its
  Worker deleted after. Every gate runner passed on Node 24.
  - The deployed probe measured the per-item wall at **32,514** max-length tags,
    against ADR-0079's prediction of about 32,500, and no wall to 1,024 items,
    condition items or tags in one item, or to 100,000 `json_each` elements.
    `experiments/durable-object-limits/results/run-workerd-deployed-2026-10-06.txt`;
    the README's † cells are filled.
  - The `workerd` exit criterion is ticked: it was watched failing at L6a (run
    36957404625) and both walls are recorded, locally and deployed.
  - Two assertion messages in `query_sql.rs` lost their line continuations and
    carried runs of spaces (already on main); fixed here (`wi-06b54f`).
  - Still owed: the vacuity control, and the `0.4.0` trace table's hand row for
    `planned_statement_count`.
- 2026-10-07 — **An unattended overnight session starts the queue.** The
  handover named `80e1a213` and did not know #39, which merged during the
  session as `896d48c` (the live Neon job is not a required check until L8,
  `wi-0f1291`); `git log` was trusted over it.
  - **The vacuity control ran, owed since L6a.** Draft PR #40 (branch
    `lane/p17-vacuity-control`, kept) put a match guard in `emit_dispatch` that
    drops `query_item_types_are_or`. Run 37570009097
    (https://github.com/Wet-Ink-Corporation/happenstance/actions/runs/37570009097/job/112626278503):
    local `workerd` 97 collected, 96 passed, the one failure
    `query_item_types_are_or` with `no such rule: expected 404 to be 200`; the
    deployed object 96 executed, one failed, `FAIL query_item_types_are_or [404]
    no such rule`. A dropped rule is red by name on both legs. Closed unmerged.
    The same run's live Neon job went red on `query_items_share_one_snapshot`
    (107 of 108), the known ES-12 race: one more data point for L8.
  - **`wi-13bd3b`, PR #33's review follow-up**, on `lane/p17-affected-followup`.
    `declared_excludes` matched its key by prefix, so an `excludes` or
    `exclude-note` line above the real key stopped the scan short of it, and a
    present key it could not parse read as excluding nothing. It now matches
    the whole key and fails closed with the manifest's path (`wi-9e72a4`, a
    two-way default); a `#` comment inside a multi-line array, which the old
    reader took for part of an entry, is stripped. Three tests carry the wrong
    implementations and were red against the old body. Citations into
    `xtask/src/main.rs` (`lint_narrative.rs`, `narrative_doctests.rs`,
    `proof.rs`) and the shifted `affected.rs` lines were repointed by anchor, and
    `assert_covers_manifest`'s stale paragraph rewritten. Verified: the temper
    gate, `cargo test -p xtask`, xtask clippy `-D warnings`, `spec-trace`,
    `lints`, `lint-kb`; `temper:rust-reviewer`'s first round asked for changes
    (two citations four lines off, error reasons unpinned); all were made, and
    its second round approved the tree. Greptile then found that a `#` inside a
    quoted entry was cut as a comment; `quoted_entries` now reads the array
    quote-aware and refuses a backslash escape rather than misreading it.
  - **A pre-existing flake, seen while gating this:**
    `mutation_coverage::the_concurrency_rules_reject_exactly_what_they_claim`
    failed once in two runs of `cargo test --workspace --all-features --test
    mutation_coverage`: `RacingProbeStore` passed `exactly_one_of_n_contenders_commits`
    because no race happened to interleave. Under the temper gate while another
    cargo build ran, it was red three runs in three, on `GlobalVersionStore`
    against `k_disjoint_boundaries_never_conflict`; on an idle machine, green.
    The rendezvous in `tests/mutation_coverage/racers.rs` is bounded by a yield
    count, which an oversubscribed host exhausts before the cohort arrives. Nothing in this diff touches the
    testkit. It is recorded, not fixed; a mutant that must lose a race needs a
    forced interleaving, not a retry.
    **Fixed since, by #58 (`527dc08b`, `wi-bde4fa`):** the yield budgets are
    gone from `racers.rs`. Each mutant store wraps a `Handle` that counts as a
    contender from `connect` until its first read or its drop, and a cohort
    releases when every live contender is waiting in it. A contender with no
    other live contender is a cohort of one, released on arrival. No
    conformance rule changed. The reasoning is at `Shared::cohort`, under *Why
    it cannot hang*, which now also states the one precondition a new rule must
    meet: no handle that has never read may be held open across a race unless
    it appends in that race. Measured: 46 failures in 400 runs before, 0 in
    600 after. A parallel fix of the same design (#56) was reconciled onto #58,
    and also ran green 10 in 10 with a 0–35 ms sleep injected ahead of the
    probe, where the yield-bounded version was red 3 in 3.
- 2026-10-07 — **PR #41 merged as `4fbfefa`** (the vacuity control's record and
  `wi-13bd3b`). **The guard-plan assertion ADR-0068 left owed**, on
  `lane/p17-guard-plan`. `evaluate` in `happenstance-sqlite` now builds its
  statements through three private functions, `guard_boundary`,
  `guard_statements` and `guard_statement`, so a test plans the exact text and
  binds `append` runs under `BEGIN IMMEDIATE`; no public item moved.
  `the_guard_plan_has_no_list_subquery` runs `EXPLAIN QUERY PLAN` over five guard
  shapes (two tags with and without a boundary, three tags, two items, two tags
  with types) on the bundled SQLite, with `tag_cardinality` seeded so the
  selective tag seeds, and asserts a `SEARCH seed` row, no `LIST SUBQUERY`, and
  `position=?` on every chained seek. **The falsifier did not fire.** Two standing
  mutants keep both limbs from being decorative: the pre-`8c8b215` uncorrelated
  `IN (SELECT …)` chain plans a `LIST SUBQUERY`, and a dropped-alias `EXISTS` plans
  a chained seek without `position=?` and no `LIST SUBQUERY`. Both limbs were also
  watched red by hand against `query_sql.rs`, restored byte for byte. The stale
  `event_store.rs:2418` now reads `:2939`, the read-path plan test it named at
  `230065f`; 29 other citations shifted by the extraction were repointed by
  difflib, accepted atoms by `:N` only. Verified: the temper gate, `spec-trace`,
  `lints`, `lint-constitution`, `lint-kb`; `temper:rust-reviewer` approved, and its
  one minor (the correlation limb had no standing mutant) was added. Citations
  already stale on `main` (the spec's `:284`, `:998`, `:1133`, `:1364`; ADR-0055's
  and ADR-0058's ranges; `remint-identity-precondition-is-trust-only.md:59`) were
  left for a sweep that reads each referent.
- 2026-10-07 — **PR #42 merged as `6235224`. L7: ES-17 is frozen on
  `&[Event]` (ADR-0080)**, on `lane/p17-es17`. The measurement ADR-0012's falsifier
  and ADR-0055 asked for, in `experiments/append-batch-ownership/`: Cloudflare,
  the adapter whose write path an owned batch could shorten, through calibrated
  replica arms (`wi-8b2786`), because `event_store_benchmarks!` is compiled out on
  wasm32. B0 is the shipped write path and allocates exactly what
  `CloudflareEventStore::append` does at all 72 sweep points; B1 is borrowed with
  one Rust copy per payload; O1 owns the batch and moves its buffers. A decision
  rule was fixed before the run (`wi-95d2b2`; self-attested, the README says so
  and quotes the two earlier records): freeze unless O1 beats B1 by more than 10%
  and below B1's lower quartile at batch 128, payloads up to 16 KiB. **It fired in
  0 of the 9 decision cells.** Owning saves exactly 2 heap operations per event;
  a raw caller resending one batch under a by-value `append` pays 90–95% more heap
  operations at k = 8 contenders, and the typed loop, which re-decides, pays
  nothing either way. Two cells outside the region fired and an independent
  re-run moved them, which reads as noise. The stronger finding is internal: the
  shipped Cloudflare path makes 27–29% more heap operations than B1 and
  `worker`'s `exec_raw` would bind with no Rust copy at all — a follow-up with no
  signature change, not decided here. ES-17 is `[FROZEN]`, line-neutrally; its
  open question is superseded; the ledger's next free ADR is 0088, with 0081–0087
  reserved by phase 17 lanes in flight. Not measured: `workerd` or a deployed
  object, Postgres, and effects under about 20% of wall time. Verified: the
  experiment's fmt, both clippies, host tests under the default harness, the six
  wasm32 conformance tests, `spec-trace`, `lints`, `lint-kb`; two independent
  reviews (the second re-ran the sweep and matched all 288 deterministic rows).
  Records PRs opened this session and left for the owner: #43 (ADR-0083, the codec
  stays unsealed), #44 (ADR-0084, the SQL seam, now also proposing a Postgres
  parameter-count check after a real gap was found) and #45 (ADR-0086, VT-6
  mint-once), each `proposed`.
- 2026-10-07 — **PR #46 merged as `3462bf8`** (L7, ES-17 frozen on `&[Event]`,
  ADR-0080). Its deployed `workerd` leg failed once with
  `500 Durable Object reset because its code was updated.` on one rule
  (run 37580786041, job 112659800922), as #44's had; `workerd` is not a required
  check, and its one re-run was spent. **The deployed leg now treats that answer
  as a platform miss**, on `lane/p17-workerd-reset`: `harness/workerd/platform-miss.mjs`
  matches the exact `500` body, and a rule's own message that merely mentions a
  reset, or a `404` carrying the same words, is still a failure. Red first: the new
  case failed against the old matcher (1 of 6), then 6 of 6. Verified: vitest on
  `test/platform-miss.test.ts`, `spec-trace`, `lints`, `lint-kb`. No Rust changed.
- 2026-10-07 — **L9: ADR-0022 §9 is reproduced, and its remedy is breaking**, on
  `lane/p17-runtime-seam`, PR open and **not to be merged until the owner accepts
  [ADR-0081](../../.kb/decisions/0081-a-store-hops-onto-the-runtime-it-is-called-on.md)**,
  which is `proposed`. A store built on one runtime and driven from another after
  the first is dropped reported `Worker(JoinError::Cancelled)`, never `NoRuntime`:
  a `happenstance-sqlite` read and every `SqliteProjectionStore` method, and every
  `PostgresEventStore` operation (a read hung while the capturing runtime was alive
  but undriven). Remedy B prefers `Handle::try_current()` and falls back to the
  captured handle at four sites; no signature changes, so it is a behaviour change
  on two published crates and rides `0.4.0`. ADR-0068's falsifier did not fire.
  **Not fixed by B:** a pooled `sqlx` connection opened on a dropped runtime ends in
  `PoolTimedOut` or hangs; ADR-0081 makes it a documented obligation and asks the
  owner whether that is enough. The review added one disclosure: on Postgres the
  calling runtime now needs tokio's time and I/O drivers. Tests:
  `tests/runtime_seam.rs` in both crates (Postgres's `#[ignore]` and live, run by a
  new list/run/assert trio in `live-postgres`). Verified: temper gate green
  (`fbf30a6067548135`, before the merge of `main` and the disclosure); sqlite
  `runtime_seam` 4/4, `concurrency` 10/10, `read` 20/20; Postgres live
  `runtime_seam` 4/4 six times and `postgres_conformance` 108/108;
  `spec-trace`, `lints`, `lint-constitution`, `lint-kb`. `temper:rust-reviewer`
  found no blocker or major on this tree, reported in prose, because its verdict
  JSON binds to the main checkout's tree rather than this worktree.
- 2026-10-07 — **PR #47 merged as `12540a7`** (the deployed `workerd` leg retries
  a Durable Object reset). **Neon N2: `NeonWriteBatch::push` is narrowed**, on
  `lane/p17-neon-push`, by the owner's default that it rides `0.4.0`.
  - **What changed.** `push` now takes `(&'static str, Vec<serde_json::Value>)`.
    A computed statement goes through the new `push_raw_sql(SqlStatement)`. The
    `statements` field is private, read through `statements()`, because
    `#[non_exhaustive]` does not stop `batch.statements.push(…)`. The probes
    moved to `push_raw_sql`, since their table name is computed.
  - **Tests.** Three unit tests, and two `compile_fail` doctests (E0308 for an
    interpolated `String`, E0616 for the field). A reviewer compiled both
    snippets by hand to confirm each error code.
  - **Records.** A BREAKING CHANGELOG entry. The shape is ADR-0084 §2.4's, which
    is still `proposed` in #44; if the owner amends it on acceptance, this
    follows.
  - **Review fixes.** The review asked for six fixes, all made:
    - the parameter-count doc names `reset`;
    - a stale open-question citation;
    - three spec citations into Neon's `projection_store.rs` were already off
      target and now land (`:294`, `:430-445`, `:570-637`);
    - the CHANGELOG's migration line;
    - two doc nits.
  - **Verified.**
    - The temper gate (green on `--no-cache`, after the known `mutation_coverage`
      racing-mutant flake).
    - `cargo test -p happenstance-neon --all-features`.
    - `spec-trace`, `lints`, `lint-kb`, `lint-constitution`.
    - `cargo xtask wasm`, with Node 24 on `PATH`; Node 22 fails the Cloudflare
      shim.
  - **Not verified.** The probe's `push_raw_sql` path against a live endpoint
    in this session. CI's `live-neon` runs it.
- 2026-10-07 — **PR #49 merged as `4278816`** (Neon's `push` narrowed). **L8: the
  ES-11 fence works on Neon, and ADR-0087 is `proposed`**, on
  `lane/p17-adr-0087-es11`. Not merged until the owner decides it.
  - **The spike.** Draft PR #50, `lane/p17-es11-fence`, head `1177cfc`. It is
    never merged; the branch is kept.
    - It adds a required `SqlTransport::reads_settled()`, and a std-only
      `ReadLedger` whose release comes from the transport's own I/O, so the
      rule's single task cannot deadlock it.
    - `NeonEventStore::append` waits once for every read its transport
      dispatched earlier.
    - Offline tests reproduce the race over a hand-polled fake transport. A1 was
      red before the wait.
  - **The rule, and its amendment.** Pre-registered in
    `experiments/es-11-fence/README.md` before any run, then amended before the
    first counted run. The review's finding W1 was that a frontier-lagged
    `before` scored as a false falsifier. The first attempt, run 37591126575,
    is a pilot excluded by name, and the README says who saw its rows.
  - **Measured.** CI's live-neon job, run 37594816236, attempts 1–3:
    - baseline: 172 red of 1,500 (92 in es11, 80 in es12), all C2, Clopper–Pearson
      95% 9.9–13.2%;
    - fence: 0 red of 1,500, rule-of-three 95% bound 0.20%;
    - 0 anchors, 0 errors;
    - both racing rules green in all three attempts, and the live suite 109/109
      each time.
  - **Owner decisions** (ADR-0087 §11):
    - D1: the fence shape, a semver-major break of `SqlTransport`;
    - D2: publish the ledger types;
    - D6: freeze ES-11/ES-12 when the fence lands on `main`;
    - D8: restore `wi-0f1291`'s required check when the fence lands, not when
      the record does;
    - D10: mark ADR-0061 superseded on acceptance;
    - D11: correct "Postgres gets both halves";
    - D12: land the break in `0.4.0`.
  - **Not measured.**
    - A wasm32 `fetch` transport; reasoning only.
    - The cross-handle and cross-process case.
    - Contention under the fence.
  - **Verified.** `spec-trace`, `lints`, `lint-kb` (xtask rebuilt in this
    worktree first), and `run.sh tally` regenerating the committed tally byte for
    byte. The spike: temper gate `--no-cache` green, two independent reviews (the
    first requested the W1–W7 fixes), `cargo xtask wasm`.
- 2026-10-07 — **The `0.4.0` trace table is drafted, not released**, on
  `lane/p17-trace-table`, at the end of `CHANGELOG.md`'s `[Unreleased]`.
  - **The tool run.** `cargo semver-checks check-release --workspace
    --baseline-version 0.3.2 --release-type minor`, cargo-semver-checks 0.51.0,
    against `main` at `4278816`.
  - **Clean.** `happenstance`, `happenstance-core`, `happenstance-sqlite` and
    `happenstance-cloudflare` reported no break.
  - **Six tool rows:**
    - three on `happenstance-neon`: the `push` narrowing, reported as a parameter
      count, a removed field and a hidden field (#49);
    - two on `happenstance-postgres`: `naive-arm` and `new_naive` (lane L4);
    - one on `happenstance-testkit`: the `k_disjoint` rename (ADR-0077).
  - **Seven hand rows:**
    - core's `unstable-projection`, which the tool passes over as an `unstable-*`
      feature;
    - the hidden emitters (ADR-0076);
    - `planned_statement_count`'s values (ADR-0079);
    - `Busy` replacing `Store`, and the typed retry (ADR-0077);
    - SQLite's 15 s timeout (ADR-0065);
    - `CommandError::Exhausted.source`'s type, and the renamed rule's changed
      acceptance (both added in review, Greptile on #52).
  - **Every row has a decision.** Four `proposed` records are listed as pending
    (ADR-0081, 0082, 0084, 0087). The release box stays open: nothing is
    published or tagged.
- 2026-10-07 — **PR #52 merged as `151f5c8`** (the `0.4.0` trace table, drafted).
  **L10: `ProjectionId` is validated (ADR-0082, `accepted`)**, on
  `lane/p17-projection-id`. The PR stays open, and is not to merge until the owner
  approves the one lint exception below (H-05).
  - **What `ProjectionId::new` does now.**
    - It returns `Result<ProjectionId, InvalidProjectionId>`.
    - It refuses VT-14's set: empty, a `Cc` control, or a bidirectional control.
      Refusals are reported left to right, in the order `validate::check` walks.
    - It refuses more than 255 bytes (`MAX_PROJECTION_ID_LEN`).
    - It refuses the reserved prefixes `happenstance/` and `sync/`, compared as
      exact bytes (`wi-2155ac`).
    - An accepted id is kept byte for byte.
  - **New constructors and conversions.**
    - `const fn from_static` goes through the same validator, so a literal id is
      checked at compile time.
    - `sync_watermark(StoreId)` is the only way to a `sync/` id (`wi-279dbb`).
    - There is no `From<&str>` or `From<String>`; `TryFrom` and `FromStr`
      validate.
  - **Spec.** VT-35 `[FROZEN]`, and PS-39 `[PROVISIONAL]`: the testkit rule
    `projection_ids_round_trip_by_bytes`, seven id pairs and six new mutants
    (case, truncation, `latin1`, slug, trailing space, canonical equivalence; the
    last two added in review).
  - **The lint exception (H-05).** `from_static`'s const-context `panic!` needs
    `#[expect(clippy::panic)]`, the form the temper hook offers. `event.rs` has the
    same panic without the attribute.
  - **Records.** `projection-id-is-unvalidated` is closed. The CHANGELOG's
    BREAKING entry carries a SQL recipe for checkpoint rows stranded under a
    now-invalid id (option A).
  - **Merging `main`.** Five conflicts resolved by hand. The clause counts are
    now 205 IDs: 154 frozen, 32 provisional, 12 deferred.
  - **Verified.**
    - Temper gate green, uncached, on the merged tree (`093568961dabf253`, after
      review). Two independent reviews: the first requested F1–F8 (the major one:
      PS-39 tested no trim or normalisation), the second verified all eight and
      found four citation slips (W1–W4), fixed here. Its remaining blocker is the
      H-05 approval.
    - `spec-trace`, `lints` and `lint-kb`.
    - PS-39 on memory, SQLite and both Postgres stores, the last two against a
      live 17.10.
  - **Not verified.** Neon's PS-39 (CI runs it), and the wasm32 Durable Object
    run on this host's Node 22.
- 2026-10-08 — **The owner's decisions are enacted, and the exit pass is green.**
  The owner decided four questions in a Weigh-In sitting (#55):
  - accept ADR-0081, 0083, 0084 and 0086;
  - approve the `#[expect]` on `ProjectionId::from_static`;
  - accept ADR-0087, scoped to one transport (D3);
  - rewrite the racing mutants.
  - **Merged, in order:**
    - #53 (`5ff913d`): L10, `ProjectionId` validated (ADR-0082).
    - #43 (`e4eecac`): ADR-0083 accepted.
    - #55 (`d1bd3c9`): the decision intake.
    - #44 (`344583f`): ADR-0084 accepted.
    - #45 (`5e01958`): ADR-0086 accepted.
    - #48 (`00c7ebe`): L9, ADR-0081 accepted. Merging `main` broke its new test,
      which still treated `ProjectionId::new` as infallible; the gate caught it.
    - #51 (`6876f53`): ADR-0087 accepted, ADR-0061 superseded.
    - #57 (`6411311`): the ES-11 fence lands, and ES-11 and ES-12 are frozen;
      156 clauses are frozen and 30 provisional.
    - #58 (`527dc08`): the racing mutants wait for a counted party. Before, 46
      failures in 400 loaded runs; after, 0 in 600.
  - **Spike #50** is closed unmerged, as planned.
  - **Every open question each record answered** is closed as superseded.
  - **The full gate.** `cargo xtask ci` on `527dc08` passes: *all checks
    passed*. The racing-mutant test, red twice there before #58, is green
    under the whole suite. Five optional steps did not run because their
    tools are not on this host: `cargo hack` (two), `cargo deny`, and the
    two nightly docs.rs builds.
  - **Exit criteria ticked:**
    - ADR-0022 §9 (the remedy landed in #48);
    - the breaking open questions (each answered by an accepted record);
    - the `freeze-by-17` clauses (no row remains);
    - the specification reconciled (`spec-trace` passes on `527dc08`).
  - **Boxes ticked:**
    - the breaking open questions;
    - the `workerd` sibling job (its exit criterion was already met);
    - the clause follow-ups (VT-10, ES-11 and ES-12 are all frozen).
  - **One exit criterion is open:** `0.4.0` released. The trace table's one
    pending row, ADR-0084's Postgres parameter-count check, is built as H12
    (#89, `wi-7e9a97`) and waits for the owner's merge.
  - **The owner's, outstanding:**
    - re-add `conformance against a live Neon endpoint` to the `Protect main`
      ruleset now that the fence is on `main` (ADR-0087 D8, `wi-0f1291`);
    - #97 · the `0.4.0` release itself.
