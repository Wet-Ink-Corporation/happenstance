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

**Decisions it settles.** ADR-0028 (moved from phase 14). The published-surface
half of ADR-0026. The `Projection::apply` record the port's freeze left owed.
Whatever phase 16 classified as breaking (ADR-0066), the ES-11 record that
supersedes ADR-0061's keep, and ADR-0022 §9, which ADR-0068 left undecided and
gave to this phase.

**Work**

- [ ] **ADR-0028 — what a store may forget, and how it says so.** Either a port
      surface through which a store reports history it no longer holds (a method on
      `EventStore`, which breaks all four published adapters) or a written refusal
      stating what a store that has been deleted from may look like. Phase 14 then
      builds the suffix store and the rules against whichever this is.
- [ ] **ADR-0026's published half — a foreign identity's write path.**
      `crates/happenstance-sync/src/ingest.rs` holds four `todo!()` bodies
      blocked on `happenstance-core` having no write path that preserves a foreign
      `EventId` (VT-10). Decide whether core grows one, or each adapter does, or
      `IngestStore` works without one — before 1.0, because the first two touch
      published crates. Settle it against a compiling spike, not an argument.
- [ ] **`Projection::apply`** — synchronous, asynchronous, or given a batch
      handle it can issue statements through
      (`.kb/open-questions/projection-apply-is-synchronous-against-a-live-store.md`).
      Phase 18 implements it. The record names SY-21 — a convergent projection is
      handed an `EventId` and never a `SequencePosition` — because
      `apply`'s arguments are that clause's surface, and phase 13, which owns the
      rule, runs too late to shape a signature this phase decides.
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
      - `then-empty-emission-idiom-and-the-nothing-to-do-channel`.
      - `tuple-boundary-heterogeneous-event-type` — option A forecloses B and C,
        so the choice is the break, not the build.
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
      assertion or a CI job rather than a signature — but ES-7's freeze rides the
      record that answers it, and that record is written here.
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
- [ ] **ADR-0069's total `QueryItem` constructor.** An infallible constructor
      taking a first `EventType` as its own parameter, any further types and a
      `Tags`, canonicalising exactly as `QueryItem::new` does
      (`crates/happenstance-core/src/query.rs:56-76`). Additive, so it is not on
      the breaking list; it ships in `0.4.0` with this window's other
      `happenstance-core` work. The spelling (`QueryItem::of` is ADR-0069's
      candidate) is settled here by compiling it, and the session log records it.
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
- [ ] **A minimal-versions CI job.** After a lockstep `1.0.0` the crates version
      independently (ADR-0066), and each adapter declares the core it needs as
      `happenstance-core = "1.N"`. A lower bound nothing ever resolves against is
      a guess, so a sibling job builds the workspace at its minimal versions and
      keeps every declared bound honest. A sibling rather than a gate step: it
      needs a nightly resolver.
- [ ] **The clause follow-ups phase 16 gave this phase.** Each `freeze-by-17` row
      in [the 1.0 dispositions](../ledgers.md), and what freezes it:
      - **VT-10** — the foreign-identity spike above, with SQLite implementing
        `IngestStore` beside `append`. It must not foreclose SY-14's
        bounded-round-trip ingest, which phase 13 measures.
      - **VT-14** — an RTL identifier corpus check (Arabic, Hebrew and Persian,
        with mixed LTR) comes back empty, and the E11 reproduction goes into
        `experiments/`.
      - **VT-30** — ADR-0054's alias and builder-state questions decided in one
        pass; limb 2 retired by a record or by a multi-guard benchmark scenario.
      - **ES-7** — frozen in the record that answers
        `trait-variant-caret-resolves-past-the-locked-gate` and ES-17's ownership,
        its falsifier restated to cover a consumer's unlocked resolve.
      - **ES-11, ES-12** — the ES-11 record above.
      - **ES-17** — the measurement or the freezing record, above.
      - **ES-41** — with ADR-0028. The transport half is already answered: Neon
        and Cloudflare each probe membership in one read-only round trip over the
        pair VT-8 indexes.
      - **PS-9, PS-11** — together, in the `Projection::apply` record, with the
        failure-policy seam's shape. Phase 18 confirms both by building PS-27's
        seam without a generic write.
      - **PS-15** — decide whether `rollback` refuses a foreign batch (breaking),
        or narrow the MUST to `commit` and `reset`.
      - **PS-22** — ADR-0028 states that retention never rewinds a checkpoint
        over kept rows.
      - **PS-23** — freeze *exactly one* id per commit, or record that a multi-id
        atomic commit arrives after 1.0 as an additive defaulted method rather
        than a change to `commit`.
      - **PS-24** — freeze `Authority::Rebuilding` as a kept variant. Phase 18
        decides whether the typed runner ever emits it.

      And the decisions taken here for clauses another phase freezes: **ES-39,
      ES-40, SY-32 and CF-27** (ADR-0028's shape; phase 14 builds and freezes);
      **SY-21** (inside the `apply` record, which must name it; phase 18 freezes);
      **PS-25** (the derived-id remedy or the digest-in-checkpoint one — the
      second changes the frozen port's `commit`, so the choice is made here);
      **CF-40**'s `MetadataLen` build under ADR-0043 (phase 13 then decides the
      budget unit); and **PS-38**'s documented no-lagging-replica obligation
      (phase 18 freezes it with PS-23).
- [ ] **Release `0.4.0`.** `cargo-semver-checks` against the `0.3.x` registry
      baseline reports breaks, and each one it reports traces to a decision above.
      `CHANGELOG.md`'s `[Unreleased]` entries — SQLite's fifteen-second busy
      timeout (ADR-0065) and `FaultyStore::contend_next` — ship in `0.4.0`;
      there is no `0.3.3` (`wi-052920`).

**Proof artefact.** `0.4.0` on crates.io, and a table in its changelog entry
mapping every major finding `cargo-semver-checks` reported to the decision that
caused it — a break with no row is one nobody decided.

**Exit criteria**

- [ ] ADR-0028 accepted; ES-39 is no longer `[DEFERRED]` on a surface 1.0
      promises, or is renewed past 1.0 with the disposition phase 16 gave it.
- [ ] The foreign-identity question is answered against a compiling spike.
- [ ] ADR-0022 §9's reproduction runs against `happenstance-sqlite` and
      `happenstance-postgres`, and a record classifies its remedy as additive or
      breaking; a breaking remedy has landed.
- [ ] `QueryItem`'s total constructor (ADR-0069) and the guard-plan
      `LIST SUBQUERY` assertion (ADR-0068) are in the tree.
- [ ] The `apply` record is accepted.
- [ ] Every open question phase 16 classified as breaking is answered or closed.
- [ ] `0.4.0` is released and its semver findings are fully traced.
- [ ] The `workerd` and minimal-versions jobs exist, each has been watched
      failing once, and the SQL-text wall and partition widths are recorded as
      measured on `workerd`.
- [ ] Every `freeze-by-17` clause in [`ledgers.md`](../ledgers.md)'s 1.0
      dispositions is `[FROZEN]`, or re-dispositioned by a record that says why.
- [ ] The specification is reconciled against this phase's changes (session
      protocol step 6), and `cargo xtask spec-trace` passes.

**Estimate.** 5–8 days. The range is honest: the spread is mostly the
foreign-identity spike, which nobody has attempted.

**Session log**
