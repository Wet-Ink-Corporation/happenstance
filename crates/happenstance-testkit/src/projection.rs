//! The projection store rule family: its rules, its single enumeration, its
//! three emitters and its entry macro.
//!
//! # Why a projection rule may not live in `suite.rs`
//!
//! `cargo xtask spec-trace` and `no_orphan_rules` both read `suite.rs` as *the
//! event-store family's* rules module, matching `    pub async fn ` textually. A
//! projection rule written there is claimed by the event-store enumeration it is
//! absent from, and reported as an orphan of the wrong family. So this family
//! carries its own rules, its own enumeration and its own emitters, in one file
//! — which is the shape `model.rs` and `concurrency.rs` already have, and the
//! reason CF-22's *"exactly one place"* is per rule **family** rather than per
//! crate.
//!
//! # What the family costs, stated rather than discovered
//!
//! Four exported macros:
//! [`for_each_projection_store_rule!`](crate::for_each_projection_store_rule),
//! three `#[doc(hidden)]` emitters, and
//! [`projection_store_conformance!`](crate::projection_store_conformance) at the
//! crate root. `happenstance-testkit` carries its own version precisely because
//! a suite change can turn a passing adapter's CI red, and a macro is the
//! hardest surface to walk back.
//!
//! It reuses everything else unchanged: [`Capability`](crate::Capability),
//! [`RuleOutcome`](crate::RuleOutcome), [`block_on`](crate::block_on) and
//! `__emit_rule_names`. There is no projection-local skip type and no second
//! line shape, because an author reading one CI log must not have to learn two.
//!
//! # What a green run here does and does not prove
//!
//! Every rule below passes against `MemoryProjectionStore`, which is the oracle
//! and is *supposed* to pass. That the suite can **fail** a wrong store is a
//! different claim, and it is now discharged rather than carried: each rule
//! names the defect it rejects, and the store carrying that defect is registered
//! as data in `tests/projection_mutation_coverage.rs`, driven through this
//! enumeration, and asserted to fail **exactly** the rules it declares
//! ([ADR-0010](../../../.kb/decisions/0010-the-suite-must-prove-itself.md)).
//! `every_projection_rule_has_a_mutant` is what makes that true of the *next*
//! rule as well: a rule added here without a store that fails it turns the
//! workspace red, and there is no exemption list.
//!
//! What a green run still does **not** prove is that this family is complete.
//! Every rule §4.11 assigns to an adapter's own suite is now written; what is
//! still owed is the **six runner-dependent** ones, which CF-36 moves to the
//! workspace e2e crate because they need a runner rather than a store
//! (`spec/SPECIFICATION.md:5696-5705`). A store that passes everything here has
//! not been observed under replay.
//!
//! The seventeenth landed last and did not land quietly.
//! [`fresh_projection_has_no_checkpoint`](rules::fresh_projection_has_no_checkpoint)
//! was **held out of this module for a day**, because §4.11 assigned it to PS-19
//! and PS-19's `MUST` is scoped *after a successful `reset`* — so writing the
//! rule would have widened a `[FROZEN]` clause by test, convicting adapters of an
//! obligation no sentence stated. The repair was a new clause rather than a line
//! edit: ADR-0030 minted PS-38, whose second sentence is the obligation, and the
//! rule cites that clause rather than the one it was filed under
//! ([ADR-0030](../../../.kb/decisions/0030-the-checkpoint-reports-the-commits-that-happened.md)).
//!
//! # Two rules here are answered by a skip against the reference fixture
//!
//! [`failed_commit_leaves_both_unchanged`](rules::failed_commit_leaves_both_unchanged)
//! is PS-1's second conjunct — the arm about a commit that *reported failure* —
//! and it is gated on
//! [`COMMIT_FAULT`](crate::ProjectionFixture::COMMIT_FAULT).
//! [`refused_reset_changes_nothing`](rules::refused_reset_changes_nothing) is
//! PS-18, and it is gated on
//! [`RESET_REFUSAL`](crate::ProjectionFixture::RESET_REFUSAL). Neither
//! capability is supported by any fixture outside the mutant harness, and
//! `MemoryProjectionFixture` declines both honestly and for reasons it states:
//! the reference store applies both halves of a commit under one write lock and
//! has no write that can be made to fail, and it holds no protection policy, so
//! there is no projection it could decline to reset. A run against the oracle
//! therefore prints two `SKIP` lines, and an adapter that wants either conjunct
//! checked has to supply what its own store can do.
//!
//! A third switch can produce a skip and is not on the fixture at all:
//! [`READS_THROUGH_BATCH`](happenstance_core::ProjectionProbe::READS_THROUGH_BATCH)
//! is a `const` on the **store's** probe impl, because whether a batch can be
//! read through is a property of the batch type. An adapter declaring it `false`
//! — which PS-12 permits outright — gets
//! [`batch_reads_reflect_pending_writes`](rules::batch_reads_reflect_pending_writes)
//! and [`rebuild_is_chunk_size_invariant`](rules::rebuild_is_chunk_size_invariant)
//! as skips naming that constant, with a reason the testkit writes rather than
//! the fixture, for [`NO_CEILING_REASON`](crate::NO_CEILING_REASON)'s reason.
//! `MemoryProjectionStore` declares `true`, so a reference run does not print
//! those two.
//!
//! Read that as the answer to "does a green run mean anything here": for fifteen
//! of the seventeen landed rules it means the store was driven and asserted
//! about; for these two it means what the `SKIP` lines say. Neither rule is
//! decorative — `PartialCommitStore`, `RefusalAsSuccessStore` and
//! `RefusalAfterTheFactStore` fail them by name in
//! `tests/projection_mutation_coverage.rs` — but the demonstration lives against
//! a fixture that can arm a fault and protect a projection, and the reference
//! fixture is neither.
//!
//! One stale sentence, named here because it cannot be repaired here. PS-1's
//! **Rejects:** prose says *"Today no rule can fail it"*
//! (`spec/SPECIFICATION.md:4756-4758`), which stopped being true when
//! `CheckpointOnlyStore` landed and is further from true now. PS-1 is
//! `[FROZEN]`, so the repair is a new decision atom under the repair-frozen-clause
//! discipline rather than a line edit, and it belongs to
//! `unstable-projection-gate-and-clause-disposition` along with the rest of the
//! PS-1 – PS-37 maturity sweep. Nothing in this crate may edit that sentence;
//! this paragraph exists so an adapter author reading the code is not the last
//! person to find out it is stale.

/// Panics unless the projection fixture supports the named capability.
///
/// `suite.rs`'s `must!` in every respect that matters — the same two-variant
/// vocabulary, the same rule that the branch lives in the rule body and never in
/// an emitter — and a second definition rather than a shared one because the
/// only thing that differs is the thing a `macro_rules!` cannot abstract over
/// here: the trait the capability is looked up on
/// ([`ProjectionFixture`](crate::ProjectionFixture), not
/// [`Fixture`](crate::Fixture)). Parameterising the event-store macro over a
/// trait path would mean exporting a private macro to the crate root and
/// rewriting thirty call sites to buy four lines; restating four lines beside
/// the family that uses them is the cheaper half of that trade, and it is the
/// same choice this family made about its emitters.
///
/// The **message** is the other half, and it is not shared prose: the
/// event-store `must!` cites CF-16 and sends the reader to `MemoryFixture`, and
/// neither sentence is true for a projection adapter. What is true here is
/// sharper — PS-1 is only observable from outside the connection that made the
/// commit — and that is what this one says.
///
/// # Its sibling
///
/// `require!` below is the declinable half, and it landed with the first rule
/// that needed it — `failed_commit_leaves_both_unchanged`, gated on
/// `COMMIT_FAULT`. Until that rule existed there was nothing for it to gate, and
/// a `require!` defined earlier would have been an unused macro (a `-D warnings`
/// failure under `unused_macros`) and, worse, a gate nothing calls.
macro_rules! must {
    ($fixture:ident : $capability:ident) => {
        if let Some(reason) = <$fixture as $crate::ProjectionFixture>::$capability.reason() {
            panic!(
                "this projection fixture declines `{}`, which is a MUST and not \
                 a trade the suite can record as a skip. Every rule in this \
                 family reads the read model and the checkpoint back through a \
                 *fresh* handle, because PS-1's coupling is only observable from \
                 outside the connection that made the commit: a store whose \
                 commit is visible to its own session alone satisfies every \
                 single-handle assertion and loses a half the moment anything \
                 else looks. A fixture that cannot open a second handle cannot \
                 observe the invariant this port exists for at all. Hold the \
                 backing store behind an `Arc` or an `Rc` and return a fresh \
                 handle from each `connect` — `fixtures::MemoryProjectionFixture` \
                 is the reference implementation. Reason given: {reason}",
                ::core::stringify!($capability),
            );
        }
    };
}

/// Returns a reported skip unless the projection fixture supports the named
/// capability.
///
/// `must!`'s declinable sibling, and the same second definition rather than a
/// shared one for the same single reason: the trait the capability is looked up
/// on is [`ProjectionFixture`](crate::ProjectionFixture), and `suite.rs`'s
/// `require!` resolves `<$fixture as $crate::Fixture>::$capability`, which a
/// projection fixture does not implement.
///
/// Everything else is the event-store macro's, deliberately unchanged: the same
/// [`RuleOutcome::Skipped`](crate::RuleOutcome::Skipped) return, carrying
/// `stringify!` of the associated const's own identifier so the author is told
/// the name of the thing they can change, and the fixture's own stated reason so
/// the CI log carries the adapter's words rather than the testkit's. There is no
/// projection-local skip type and no second line shape, because an author
/// reading one CI log must not have to learn two.
///
/// # Where it may be used, and where `must!` is required instead
///
/// Only when the rule's **entire** content needs the capability, and only for a
/// capability a fixture may honestly decline. `SECOND_HANDLE` is not one — a
/// fixture that cannot open a second handle cannot observe PS-1 at all — so
/// every rule in this family spells `must!` for it, and the one rule that also
/// spells `require!` spells the `must!` **first**: a fixture that declines both
/// is failing the contract, and reporting that as a skip would hide it behind
/// the trade.
macro_rules! require {
    ($fixture:ident : $capability:ident) => {
        if let Some(reason) = <$fixture as $crate::ProjectionFixture>::$capability.reason() {
            return $crate::RuleOutcome::Skipped {
                capability: ::core::stringify!($capability),
                reason,
            };
        }
    };
}

/// Returns a reported skip unless the fixture's **store** can be read through an
/// open batch.
///
/// [`require!`]'s third sibling, and the one whose switch is not on the fixture
/// at all: [`READS_THROUGH_BATCH`](happenstance_core::ProjectionProbe::READS_THROUGH_BATCH)
/// lives on the probe, beside the store, because whether a batch can be read
/// through is a property of the batch *type* rather than of the fixture's
/// environment. So this cannot be `require!` with a different argument — that
/// macro resolves `<F as ProjectionFixture>::$capability` and stringifies an
/// identifier, and there is no identifier here for it to stringify.
///
/// What it reports instead is [`NO_BATCH_READ_PATH`](crate::NO_BATCH_READ_PATH),
/// a `&'static str` naming the constant with its path, and
/// [`NO_BATCH_READ_PATH_REASON`](crate::NO_BATCH_READ_PATH_REASON), which is
/// **testkit-written** for [`NO_CEILING_REASON`](crate::NO_CEILING_REASON)'s
/// reason and is the second and last instance of that exception. The skip is the
/// same [`RuleOutcome::Skipped`](crate::RuleOutcome::Skipped), rendered by the
/// same `skip_line`, in the same one-line shape: there is no second declension
/// policy here, only a switch in a second place.
macro_rules! require_read_through {
    ($fixture:ident) => {
        // One line, and deliberately over the width: rustfmt's macro-body
        // formatting is not idempotent across a wrapped `as` inside a qualified
        // path — it re-indents the continuation on every run — so a break here
        // makes `cargo fmt --check` fail forever.
        #[rustfmt::skip]
        let __reads_through_batch = <<$fixture as $crate::__private::ProjectionFixture>::Store as $crate::__private::ProjectionProbe>::READS_THROUGH_BATCH;
        if !__reads_through_batch {
            return $crate::RuleOutcome::Skipped {
                capability: $crate::NO_BATCH_READ_PATH,
                reason: $crate::NO_BATCH_READ_PATH_REASON,
            };
        }
    };
}

/// The projection conformance rules.
///
/// Each rule is an independent async function taking `impl AsyncFn() -> F`: not
/// a fixture, but **how to make one**. A rule that can call `open()` twice can
/// make two genuinely isolated projection stores, which is what
/// `commit_rejects_a_foreign_batch` needs and what no meta-test over the
/// testkit's own fixture could substitute for.
///
/// They are public so an adapter can call one directly when debugging a single
/// failure, exactly as [`crate::rules`] is.
pub mod rules {
    // Every rule panics on failure — that is what a test does, and a `# Panics`
    // section on each of them would say "panics when the adapter is
    // non-conformant" once per rule.
    #![allow(clippy::missing_panics_doc)]

    use happenstance_core::{
        Authority, Checkpoint, CommitError, ProjectionId, ProjectionProbe, ProjectionStore,
        ResetError, SequencePosition,
    };

    use crate::{ProjectionFixture, RuleOutcome};

    /// The probe key every rule writes through.
    ///
    /// One constant rather than a literal per rule, so that a rule reading back
    /// a key it never wrote is a compile-time impossibility rather than a typo
    /// nobody notices for a phase.
    const PROBE_KEY: &str = "depot-7";

    /// The probe value every rule writes.
    ///
    /// Deliberately not `0` or `1`: a store that returns a default rather than
    /// the value it was handed passes a read-back asserting either of those.
    const PROBE_VALUE: u64 = 12;

    /// A second probe key, for the rules that must tell two writes apart.
    ///
    /// `dropped_batch_leaves_store_usable` and
    /// `distinct_projections_advance_independently` both assert about *two*
    /// rows, and asserting about two rows under one key cannot distinguish "the
    /// second write landed" from "the first one did".
    const SECOND_KEY: &str = "depot-11";

    /// The value written under [`SECOND_KEY`].
    ///
    /// Distinct from [`PROBE_VALUE`] for the same reason the keys are distinct:
    /// a store that writes the right key with the wrong value is a defect a
    /// shared value would hide.
    const SECOND_VALUE: u64 = 37;

    /// The replay [`rebuild_is_chunk_size_invariant`] runs at three chunk sizes,
    /// as keys.
    ///
    /// Two keys, one written four times and one twice, arranged so that chunk
    /// size 3 puts a repeated key both **inside** one chunk and **across** a
    /// boundary. Without a key repeated inside a chunk the arithmetic cannot
    /// diverge and the rule passes against a store that cannot read through its
    /// own batch — which is the shape of a rule that certifies PS-13 and PS-14 on
    /// nothing.
    ///
    /// At module scope rather than inside the rule because a `const` after a
    /// statement is `clippy::items_after_statements`, and the gate runs
    /// `-D warnings`.
    const REBUILD_SEQUENCE: [&str; 6] = [
        PROBE_KEY, PROBE_KEY, SECOND_KEY, PROBE_KEY, SECOND_KEY, PROBE_KEY,
    ];

    /// The key an *anchoring* commit writes.
    ///
    /// Separate from both of the above because the rules that anchor — commit
    /// something first so that the state a rejection must leave alone is a
    /// recorded state rather than `NeverRun` — must not have their anchor
    /// confused with the write under test.
    const ANCHOR_KEY: &str = "depot-3";

    // ---------------------------------------------------------------------
    // Helpers
    //
    // Spelled `async fn` without `pub`, which is what keeps them out of both
    // textual scans that read this file: the orphan meta-test's, and
    // `spec_trace::collect_rules`'s. A helper made public would be reported as
    // an orphan, which is the right answer rather than a false positive.
    // ---------------------------------------------------------------------

    /// Commits and unwraps, failing the test with context on error.
    async fn commit_ok<S: ProjectionStore>(
        store: &S,
        batch: S::Batch,
        id: &ProjectionId,
        position: SequencePosition,
        authority: Authority,
    ) {
        if let Err(err) = store.commit(batch, id, position, authority).await {
            panic!("commit should succeed, got {err:?}");
        }
    }

    /// Reads a checkpoint and unwraps, failing the test with context on error.
    async fn checkpoint_ok<S: ProjectionStore>(store: &S, id: &ProjectionId) -> Checkpoint {
        match store.checkpoint(id).await {
            Ok(checkpoint) => checkpoint,
            Err(err) => panic!("reading a checkpoint should succeed, got {err:?}"),
        }
    }

    /// Reads a committed probe row and unwraps, failing the test with context on
    /// error.
    async fn probe_read_ok<S: ProjectionProbe>(store: &S, key: &str) -> Option<u64> {
        match store.probe_read(key).await {
            Ok(row) => row,
            Err(err) => panic!("reading the read model should succeed, got {err:?}"),
        }
    }

    /// Rolls a batch back and unwraps, failing the test with context on error.
    async fn rollback_ok<S: ProjectionStore>(store: &S, batch: S::Batch) {
        if let Err(err) = store.rollback(batch).await {
            panic!("rolling a batch back should succeed, got {err:?}");
        }
    }

    /// Resets and unwraps, failing the test with context on error.
    ///
    /// `commit_ok`'s dual, and a separate helper rather than a generic one over
    /// both because the two operations fail differently on purpose:
    /// [`ResetError`] carries `Refused`, which `commit` cannot produce, and a
    /// shared helper would have to erase one of the two error types to say so.
    async fn reset_ok<S: ProjectionStore>(store: &S, batch: S::Batch, id: &ProjectionId) {
        if let Err(err) = store.reset(batch, id).await {
            panic!("reset should succeed, got {err:?}");
        }
    }

    /// Whether a runner resuming from `checkpoint` would still apply the event
    /// at `position`.
    ///
    /// PS-20's derivation, written once and shared by the two arms of
    /// [`reset_is_not_commit_at_first`] so that *"strictly after a recorded
    /// position, inclusive from the store's first position when `NeverRun`"* is
    /// stated in one place rather than twice with a chance of disagreeing.
    ///
    /// **It is not a runner and must not grow into one.** It decides one thing —
    /// whether one position is still in front of a checkpoint — which is the
    /// whole of what PS-20 constrains and the only part a rule can observe
    /// without a replay driver. The runner belongs to the typed layer.
    fn resumes_over(checkpoint: Checkpoint, position: SequencePosition) -> bool {
        match checkpoint {
            // Nothing has been considered, so the replay starts at the store's
            // first position *inclusive* and this event is in front of it.
            Checkpoint::NeverRun => true,
            // A recorded position has been considered, so a resume is strictly
            // after it. `Rebuilding` answers the same question the same way: it
            // carries a position that has been considered.
            Checkpoint::Live { through } | Checkpoint::Rebuilding { through } => position > through,
            // `Checkpoint` is `#[non_exhaustive]`, so this arm is required. A
            // fourth variant is a resume rule this helper has not been told, and
            // guessing at it would be a rule certifying a state nobody defined.
            _ => panic!(
                "this store reported a checkpoint variant this rule has never \
                 been taught to resume from: {checkpoint:?}. `Checkpoint` grew a \
                 variant and PS-20's derivation has not been extended to cover it"
            ),
        }
    }

    /// The position immediately after `position`.
    ///
    /// [`SequencePosition::next`] returns `Option<Self>` deliberately, so that a
    /// consumer at the top of the key space is *told* rather than looping. A rule
    /// takes the `None` arm as a panic with a message rather than a silent
    /// `unwrap`, because a rule that fell over there would otherwise read as an
    /// adapter failure.
    ///
    /// No literal appears anywhere in this file: every position a rule uses is
    /// [`SequencePosition::FIRST`] or something this function derived from it.
    /// The specification permits gaps, and a rule that assumes density passes
    /// against the reference store and fails a conformant adapter in the field
    /// (CF-6).
    fn after(position: SequencePosition) -> SequencePosition {
        match position.next() {
            Some(next) => next,
            None => panic!(
                "this rule needs the position after {position}, and there is \
                 none: the store is at the top of the key space. That is a real \
                 answer from `SequencePosition::next`, not a rule failure — but \
                 no rule in this family can run against a store there"
            ),
        }
    }

    // ---------------------------------------------------------------------
    // The baseline pair
    //
    // The rest of §4.11 is *differential* against these two: neither of the
    // other rules re-asserts the coupling, and none of them asserts progress.
    // ---------------------------------------------------------------------

    /// A commit moves the projection's checkpoint to the position it was given.
    ///
    /// The baseline the rest of this family is differential against, and the one
    /// rule in it that is not about coupling: it asserts that a commit reported
    /// as successful actually *advanced* something.
    ///
    /// **Rejects:** `UncommittedTransactionStore` — a `commit` that returns `Ok`
    /// and makes neither the read-model row nor the checkpoint durable, which is
    /// what an adapter does when it executes the batch inside a transaction it
    /// never commits. That implementation is not a straw man — PS-1's MUST is a
    /// *coupling* rather than a progress obligation, so "neither" satisfies the
    /// clause through its "or not at all" arm and passes
    /// [`commit_is_atomic_with_the_read_model`] as one of the two states that
    /// rule permits. This is the rule that fails it, and it is the only rule
    /// §4.11's table leaves an em-dash against. Whether the progress obligation
    /// joins PS-1 or earns a clause of its own is open and owned elsewhere
    /// (`.kb/open-questions/ps-1-states-no-progress-obligation.md`); the rule is
    /// written as §4.11's table specifies and settles nothing.
    ///
    /// The checkpoint is read back through a **fresh handle**, so a store whose
    /// commit is only visible to the connection that made it fails here rather
    /// than in the field.
    ///
    /// No literal position appears anywhere in the body: the checkpoint is
    /// compared against the position the commit was actually given, held in a
    /// binding. The specification permits gaps, and a rule that assumes density
    /// passes against the reference store and fails a conformant adapter.
    pub async fn commit_advances_the_checkpoint<F: ProjectionFixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        // `must!`, not `require!`: the read-back below goes through a second
        // handle, and a fixture that cannot open one cannot observe PS-1 at all.
        // That is a fixture failing the contract rather than a trade the suite
        // may record and move past.
        must!(F: SECOND_HANDLE);

        let fixture = open().await;
        let writer = fixture.connect().await;
        let id = ProjectionId::new("commit_advances_the_checkpoint");

        // The position the commit is given, and the only thing the assertion
        // below is allowed to compare against.
        let position = SequencePosition::FIRST;

        let mut batch = writer.begin();
        writer.probe_write(&mut batch, PROBE_KEY, PROBE_VALUE);
        commit_ok(&writer, batch, &id, position, Authority::Live).await;

        let observer = fixture.connect().await;
        assert_eq!(
            checkpoint_ok(&observer, &id).await,
            Checkpoint::Live { through: position },
            "a commit reported as successful must move this projection's \
             checkpoint to the position it was given, and a fresh handle must \
             see it: a `commit` that returns `Ok` and makes nothing durable \
             satisfies PS-1's \"or not at all\" arm and is rejected here"
        );

        RuleOutcome::Ran
    }

    /// The read-model write and the checkpoint write become durable together, or
    /// neither does.
    ///
    /// PS-1, and the invariant the whole port exists for. A read model and its
    /// checkpoint that can disagree is a projection that either replays applied
    /// events after a restart or skips events forever, and no amount of ordering
    /// or retrying fixes it.
    ///
    /// **Rejects:** `CheckpointOnlyStore` — a store that commits the checkpoint
    /// and discards the write set, which is the natural shape for any adapter
    /// whose read model lives somewhere other than its checkpoint table. It is
    /// named by the specification (§4.11) and registered in
    /// `tests/projection_mutation_coverage.rs`, where its declaration also
    /// records the two further rules it fails — a store that applies no rows
    /// fails any rule that reads one back.
    ///
    /// **What it deliberately does not assert.** Only the *coupling*. "Both
    /// absent" is a legal outcome here — PS-1's MUST is a coupling rather than a
    /// progress obligation — and asserting presence would make this rule fail
    /// for a defect [`commit_advances_the_checkpoint`]
    /// already owns. A rule that fails in two places for two reasons tells an
    /// adapter author neither.
    ///
    /// Both halves are read through **fresh handles**, and through
    /// [`ProjectionProbe`] only — never an inherent method on a concrete store,
    /// which no adapter in the field has.
    pub async fn commit_is_atomic_with_the_read_model<F: ProjectionFixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        // `must!` for the same reason as the rule above, and here it is the
        // clause's own reason: PS-1's coupling is a claim about what a *second*
        // observer sees.
        must!(F: SECOND_HANDLE);

        let fixture = open().await;
        let writer = fixture.connect().await;
        let id = ProjectionId::new("commit_is_atomic_with_the_read_model");

        let mut batch = writer.begin();
        writer.probe_write(&mut batch, PROBE_KEY, PROBE_VALUE);
        commit_ok(
            &writer,
            batch,
            &id,
            SequencePosition::FIRST,
            Authority::Live,
        )
        .await;

        let observer = fixture.connect().await;
        let row = probe_read_ok(&observer, PROBE_KEY).await;
        let checkpoint = checkpoint_ok(&observer, &id).await;

        let row_is_durable = row.is_some();
        let checkpoint_is_durable = checkpoint != Checkpoint::NeverRun;

        assert_eq!(
            row_is_durable, checkpoint_is_durable,
            "the read-model write and the checkpoint write must become durable \
             together or not at all, and a fresh handle saw one without the \
             other: row {row:?}, checkpoint {checkpoint:?}. A store that \
             commits the checkpoint and discards the write set replays nothing \
             and skips everything the discarded batch would have written"
        );

        RuleOutcome::Ran
    }

    // ---------------------------------------------------------------------
    // PS-1's second conjunct
    //
    // The pair above observes a commit that succeeded. This one is the only rule
    // in the family that observes a commit that *failed*, which is the other arm
    // of the same clause and needs the store's co-operation to reach at all —
    // hence the one capability gate in the family.
    // ---------------------------------------------------------------------

    /// A `commit` that reported failure left the read model and the checkpoint
    /// exactly as they were.
    ///
    /// PS-1's **second conjunct**, on its own: the clause's "or not at all" arm
    /// is a claim about the failed commit, and no other rule in this family ever
    /// sees one. [`commit_is_atomic_with_the_read_model`] observes the coupling
    /// after a commit that *succeeded*, which is the first conjunct; a store can
    /// keep that one perfectly and still leave a half-applied batch behind
    /// whenever a write fails, and every rule above it would stay green.
    ///
    /// **Rejects:** `PartialCommitStore` — an adapter that applies its
    /// read-model writes row by row and writes the checkpoint last, so a failure
    /// on the checkpoint write leaves the rows durable while `commit` reports the
    /// error honestly. It is the same adapter family `CheckpointOnlyStore` comes
    /// from — a read model that does not live in the store the checkpoint lives
    /// in — with the failure arriving one statement later, and it is what any
    /// adapter that forgot the transaction does the first time a write fails.
    ///
    /// # Why this rule is capability-gated, and the capability is the fixture's
    ///
    /// Nothing a caller holds can make a conformant `commit` fail; that is the
    /// property under test. So the injection belongs to the adapter — a trigger
    /// that raises on the third row, a `CHECK` armed for one write, a connection
    /// killed between the two halves — and
    /// [`COMMIT_FAULT`](crate::ProjectionFixture::COMMIT_FAULT) is where a store
    /// says whether it has one. A store with no way to fail a commit declines and
    /// this rule reports a skip, which is an honest hole rather than a silent
    /// pass. `MemoryProjectionFixture` is exactly such a store, so the reference
    /// run prints that skip: the oracle cannot demonstrate this conjunct, and
    /// saying so is the whole of CF-18.
    ///
    /// # Why the armed commit is *required* to fail
    ///
    /// The `Err` assertion below is a demand on the **fixture**, not on the
    /// store, and the distinction is what keeps the rule inside PS-1. A store is
    /// free to absorb a fault and commit anyway; what it may not do is have a
    /// fixture that declares `COMMIT_FAULT` and arms something the store shrugs
    /// off, because "no fault fired" and "the fault fired and everything landed"
    /// are the same `Ok` over the same full state, and a rule that accepted both
    /// would be a green result about a store nothing had ever faulted. That is
    /// CF-39's argument on the event-store side, where it is a second rule
    /// (`arming_a_mid_batch_fault_makes_the_append_fail`) because ES-18 states
    /// the swallow permission in the clause itself and the two claims had to be
    /// separable. PS-1 states no such permission, so the demand is folded in here
    /// as this rule's precondition rather than split into a rule of its own — and
    /// the residue that split would have bought is named rather than hidden: a
    /// fixture whose `arm_commit_fault` has an *empty body* is caught by this
    /// assertion, but no fixture-level mutant is registered for it, because
    /// nothing in the workspace declares the capability yet except the mutant
    /// harness. The trait's provided body panics, which is what catches the
    /// commoner mistake of declaring the capability and forgetting the override.
    ///
    /// **What it deliberately does not assert.** *Progress.* The checkpoint is
    /// compared against what a fresh handle saw **before** the faulted commit,
    /// exactly as [`rollback_leaves_both_unchanged`] does, so a store that
    /// committed nothing has an unchanged checkpoint here and is rejected by
    /// [`commit_advances_the_checkpoint`] instead, which is the rule that owns
    /// that defect.
    pub async fn failed_commit_leaves_both_unchanged<F: ProjectionFixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        // `must!` first, and the order is load-bearing: a fixture that declines
        // both is failing the contract, and reporting that as a skip would hide
        // a fixture nothing can observe PS-1 through behind a trade it is
        // entitled to make.
        must!(F: SECOND_HANDLE);
        require!(F: COMMIT_FAULT);

        let fixture = open().await;
        let writer = fixture.connect().await;
        let id = ProjectionId::new("failed_commit_leaves_both_unchanged");

        let anchored = SequencePosition::FIRST;

        // The anchor, so that "unchanged" is a *recorded* state rather than the
        // empty one: a store that preserved `NeverRun` and no rows preserved
        // nothing anybody could have broken.
        let mut anchor = writer.begin();
        writer.probe_write(&mut anchor, ANCHOR_KEY, PROBE_VALUE);
        commit_ok(&writer, anchor, &id, anchored, Authority::Live).await;

        let before = checkpoint_ok(&fixture.connect().await, &id).await;

        fixture.arm_commit_fault().await;

        let mut faulted = writer.begin();
        writer.probe_write(&mut faulted, PROBE_KEY, SECOND_VALUE);
        let outcome = writer
            .commit(faulted, &id, after(anchored), Authority::Live)
            .await;

        assert!(
            outcome.is_err(),
            "a fixture declaring `COMMIT_FAULT` supported MUST arm a fault its \
             store cannot absorb, so that the next `commit` answers `Err`. This \
             one armed a fault and the commit succeeded, which is what a fixture \
             whose `arm_commit_fault` has an empty body does — and it turns this \
             rule into a green result about a store nothing has faulted. A store \
             that can absorb every fault its fixture is able to arm must DECLINE \
             the capability with that as its stated reason. Got {outcome:?}"
        );

        let observer = fixture.connect().await;
        assert_eq!(
            probe_read_ok(&observer, PROBE_KEY).await,
            None,
            "a commit that reported failure must leave the read model as it \
             was, and a fresh handle saw a row the failed batch carried. An \
             adapter that applies its rows one statement at a time and writes \
             the checkpoint last keeps those rows when the checkpoint write \
             fails, reports the error honestly, and has silently applied part \
             of a batch its caller believes was refused"
        );
        assert_eq!(
            checkpoint_ok(&observer, &id).await,
            before,
            "a commit that reported failure must leave the checkpoint exactly \
             as the last successful commit left it"
        );

        RuleOutcome::Ran
    }

    // ---------------------------------------------------------------------
    // The commit path, differentially
    //
    // Six rules, each written against exactly one clause and each with a
    // registered store that fails it and nothing else. They are *differential*
    // against the pair above: none of them re-asserts the coupling, and none
    // asserts progress, because a rule that fails in two places for two reasons
    // tells an adapter author neither.
    // ---------------------------------------------------------------------

    /// An explicit `rollback` leaves the read model and the checkpoint exactly
    /// as the last successful commit left them.
    ///
    /// PS-8, and the reason `rollback` stays on the port at all: Rust has no
    /// `async Drop`, so an adapter holding a real transaction has no way to issue
    /// `ROLLBACK` and await its completion from a destructor.
    ///
    /// **Rejects:** `UnrolledBackStore` — a `rollback` that releases its
    /// connection without issuing `ROLLBACK`, so the rows the batch carried stay
    /// durable while the call reports success. An adapter author who tests only
    /// that `rollback` returned `Ok` cannot see it.
    ///
    /// **What it deliberately does not assert.** *Progress.* The checkpoint is
    /// compared against what a fresh handle saw **before** the rollback, not
    /// against `Live { through: P }` — so a store that committed nothing has an
    /// unchanged checkpoint here and is rejected by
    /// [`commit_advances_the_checkpoint`] instead, which is the rule that owns
    /// that defect.
    ///
    /// The anchoring commit exists so the state being preserved is a *recorded*
    /// state: a rollback that preserves `NeverRun` preserves nothing anybody
    /// could have broken.
    pub async fn rollback_leaves_both_unchanged<F: ProjectionFixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        // `must!`, not `require!`: the read-back below goes through a second
        // handle, and a fixture that cannot open one cannot observe the
        // invariant at all.
        must!(F: SECOND_HANDLE);

        let fixture = open().await;
        let writer = fixture.connect().await;
        let id = ProjectionId::new("rollback_leaves_both_unchanged");
        let position = SequencePosition::FIRST;

        let mut anchor = writer.begin();
        writer.probe_write(&mut anchor, ANCHOR_KEY, PROBE_VALUE);
        commit_ok(&writer, anchor, &id, position, Authority::Live).await;

        let before = checkpoint_ok(&fixture.connect().await, &id).await;

        let mut discarded = writer.begin();
        writer.probe_write(&mut discarded, PROBE_KEY, SECOND_VALUE);
        rollback_ok(&writer, discarded).await;

        let observer = fixture.connect().await;
        assert_eq!(
            probe_read_ok(&observer, PROBE_KEY).await,
            None,
            "an explicit rollback must leave the read model as it was, and a \
             fresh handle can still see a row the rolled-back batch carried. A \
             `rollback` that discards its own buffer while the statements it \
             already sent stay durable returns `Ok` and loses nothing an adapter \
             author would notice"
        );
        assert_eq!(
            checkpoint_ok(&observer, &id).await,
            before,
            "an explicit rollback must leave the checkpoint exactly as the last \
             successful commit left it"
        );

        RuleOutcome::Ran
    }

    /// A batch dropped bare rolls back **and** leaves the store usable.
    ///
    /// PS-7, and the second half is the rule. "Rolls back" alone certifies a
    /// store that has permanently lost its only writer, which is why this rule
    /// opens and commits a *second* batch on the same handle and asserts that
    /// second batch's row is readable.
    ///
    /// **Rejects:** `PooledConnectionStore` — an adapter whose `begin` checks out
    /// a pooled connection that `Drop` returns to nothing. A reviewer's probe
    /// already found exactly this: the store answered `Busy` forever afterwards
    /// (`spec/SPECIFICATION.md:4898-4910`). Two further stores fail it at the
    /// second row for unrelated reasons — `CheckpointOnlyStore`, which applies no
    /// rows at all, and `UncommittedTransactionStore`, which makes nothing
    /// durable — and their registry rows say so.
    ///
    /// The non-vacuity anchor is the assertion that the **second** batch's row is
    /// present. A "rolls back" -only rule cannot make it.
    pub async fn dropped_batch_leaves_store_usable<F: ProjectionFixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        must!(F: SECOND_HANDLE);

        let fixture = open().await;
        let writer = fixture.connect().await;
        let id = ProjectionId::new("dropped_batch_leaves_store_usable");
        let position = SequencePosition::FIRST;

        let mut abandoned = writer.begin();
        writer.probe_write(&mut abandoned, PROBE_KEY, PROBE_VALUE);
        // Bare: no `commit`, no `rollback`. This is the line the rule is about.
        drop(abandoned);

        let mut second = writer.begin();
        writer.probe_write(&mut second, SECOND_KEY, SECOND_VALUE);
        commit_ok(&writer, second, &id, position, Authority::Live).await;

        let observer = fixture.connect().await;
        assert_eq!(
            probe_read_ok(&observer, PROBE_KEY).await,
            None,
            "dropping a batch without `commit` or `reset` must roll it back, and \
             a fresh handle saw a row the dropped batch carried"
        );
        assert_eq!(
            probe_read_ok(&observer, SECOND_KEY).await,
            Some(SECOND_VALUE),
            "dropping a batch must leave the store **usable**: a second batch \
             opened on the same handle committed, and its row is not there. An \
             adapter whose `begin` checks out a pooled connection that `Drop` \
             returns to nothing answers `Busy` from here on, and a rule asserting \
             only that the first write rolled back would certify it"
        );
        assert_eq!(
            checkpoint_ok(&observer, &id).await,
            Checkpoint::Live { through: position },
            "the second batch's commit must have moved the checkpoint, or the \
             store is not usable after a dropped batch"
        );

        RuleOutcome::Ran
    }

    /// A batch begun on one store instance is rejected by another, and neither
    /// store moves.
    ///
    /// PS-15. The check is at run time by a stamp minted per store *instance*,
    /// because the type-level fix was compiled and refuted: a lifetime names a
    /// region rather than an instance, so two `&Store` references unify and
    /// `b.commit(a.begin(), …)` still type-checks. The only construction that
    /// names an instance is a generative brand, which forbids the batch escaping
    /// the closure that began it — defeating the caller the hazard is about.
    ///
    /// **Rejects:** `TypeStampedBatchStore` — an adapter whose batch carries an
    /// identity minted per *type* rather than per instance, so every store of
    /// that type accepts every other's batch and corrupts silently.
    ///
    /// # Why this rule does not spell `must!(F: SECOND_HANDLE)`
    ///
    /// It does not want a second handle. It wants two **isolated stores**, which
    /// is what two `open()` calls on the `impl AsyncFn() -> F` every rule is
    /// handed already produce — and that is exactly the affordance the fixture
    /// contract bought (`crates/happenstance-testkit/src/registry.rs:45-49`).
    /// A fixture that declines `SECOND_HANDLE` still runs this rule and still
    /// passes it, which is correct: the capability it declined is not the one
    /// this rule spends.
    pub async fn commit_rejects_a_foreign_batch<F: ProjectionFixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        let first = open().await;
        let second = open().await;
        let origin = first.connect().await;
        let stranger = second.connect().await;

        let id = ProjectionId::new("commit_rejects_a_foreign_batch");
        let position = SequencePosition::FIRST;

        let mut batch = origin.begin();
        origin.probe_write(&mut batch, PROBE_KEY, PROBE_VALUE);

        match stranger.commit(batch, &id, position, Authority::Live).await {
            Err(CommitError::ForeignBatch) => {}
            outcome => panic!(
                "a batch begun on a different store instance must be rejected as \
                 `CommitError::ForeignBatch`, and this store accepted it: \
                 {outcome:?}. The check is one integer comparison on a path that \
                 is already doing I/O — `begin` stamps an identity minted per \
                 store instance and `commit` compares it"
            ),
        }

        for (store, which) in [
            (&origin, "the store the batch was begun on"),
            (&stranger, "the store it was offered to"),
        ] {
            assert_eq!(
                checkpoint_ok(store, &id).await,
                Checkpoint::NeverRun,
                "a rejected commit must leave **both** stores unchanged, and \
                 {which} moved its checkpoint"
            );
            assert_eq!(
                probe_read_ok(store, PROBE_KEY).await,
                None,
                "a rejected commit must leave **both** stores unchanged, and \
                 {which} kept a row the rejected batch carried"
            );
        }

        RuleOutcome::Ran
    }

    /// A commit may name a position no applied write occupies.
    ///
    /// PS-21, and the clause that makes the checkpoint a high-water mark of
    /// **consideration** rather than of application. That is what makes it a
    /// resume point rather than a progress report.
    ///
    /// **Rejects:** `ValidatingCommitStore` — an adapter that validates
    /// `position` against what the batch wrote, which the specification names for
    /// this rule (`spec/SPECIFICATION.md:5682-5687`). It is the sharper hazard
    /// rather than a capability gap: validating is a *reasonable* reading of
    /// "advances `id`'s checkpoint to `position`", it would be equally conformant
    /// without this rule, and it makes a narrow projection re-scan the same range
    /// forever on every restart. Two adapters could disagree and both pass.
    /// `UncommittedTransactionStore` also fails it, at the same assertion and for
    /// its own reason, and its registry row says so.
    pub async fn commit_accepts_a_position_the_batch_did_not_write<F: ProjectionFixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        must!(F: SECOND_HANDLE);

        let fixture = open().await;
        let writer = fixture.connect().await;
        let id = ProjectionId::new("commit_accepts_a_position_the_batch_did_not_write");
        let position = SequencePosition::FIRST;

        // Empty on purpose: this projection considered the range and applied
        // nothing from it, which is the ordinary case for a narrow projection.
        let considered_nothing = writer.begin();
        commit_ok(&writer, considered_nothing, &id, position, Authority::Live).await;

        let observer = fixture.connect().await;
        assert_eq!(
            checkpoint_ok(&observer, &id).await,
            Checkpoint::Live { through: position },
            "the checkpoint is a high-water mark of *consideration*, not of \
             application: a commit MAY name a position no applied write occupies, \
             and an adapter MUST NOT validate `position` against what the batch \
             wrote. A projection that matches forty events in thirty-seven \
             thousand must be able to move past the rest"
        );

        RuleOutcome::Ran
    }

    /// A commit naming a position strictly below the current checkpoint is
    /// refused, and neither half moves.
    ///
    /// PS-22. Under a redeploy where an old runner pod has not yet exited, two
    /// runners share an id and the stale one drags the checkpoint backwards;
    /// every event between the two positions is then applied twice, which is
    /// harmless only for projections that happen to be idempotent. The guard
    /// converts a silent double-apply into a reported error.
    ///
    /// **Rejects:** `UnconditionalCheckpointStore` — the adapter that issues
    /// `UPDATE checkpoint SET position = ?` unconditionally, which is what
    /// everyone writes. `UncommittedTransactionStore` also fails it, because a
    /// store that recorded no checkpoint has nothing to regress against.
    ///
    /// **What it deliberately does not assert.** Anything about an **equal**
    /// position. PS-22 permits accepting one, so a rule that rejected it would
    /// forbid a conformant adapter; no commit in this body names a position the
    /// store already holds.
    pub async fn commit_rejects_a_regressing_position<F: ProjectionFixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        must!(F: SECOND_HANDLE);

        let fixture = open().await;
        let writer = fixture.connect().await;
        let id = ProjectionId::new("commit_rejects_a_regressing_position");

        let earlier = SequencePosition::FIRST;
        let later = after(earlier);

        let mut anchor = writer.begin();
        writer.probe_write(&mut anchor, ANCHOR_KEY, PROBE_VALUE);
        commit_ok(&writer, anchor, &id, later, Authority::Live).await;

        let mut regressing = writer.begin();
        writer.probe_write(&mut regressing, PROBE_KEY, SECOND_VALUE);
        match writer
            .commit(regressing, &id, earlier, Authority::Live)
            .await
        {
            Err(CommitError::CheckpointRegression { current, attempted }) => {
                assert_eq!(
                    (current, attempted),
                    (later, earlier),
                    "`CheckpointRegression` must carry the checkpoint the store \
                     already holds and the position it was actually given, so a \
                     caller can log the gap rather than re-derive it with a \
                     second round trip"
                );
            }
            outcome => panic!(
                "a commit naming a position strictly below the current checkpoint \
                 must be refused as `CommitError::CheckpointRegression`, and this \
                 store answered {outcome:?}. An unconditional `UPDATE checkpoint \
                 SET position = ?` turns a stale runner into a silent \
                 double-apply"
            ),
        }

        let observer = fixture.connect().await;
        assert_eq!(
            probe_read_ok(&observer, PROBE_KEY).await,
            None,
            "a refused commit must leave the read model unchanged, and a fresh \
             handle saw a row the refused batch carried"
        );
        assert_eq!(
            checkpoint_ok(&observer, &id).await,
            Checkpoint::Live { through: later },
            "a refused commit must leave the checkpoint unchanged"
        );

        RuleOutcome::Ran
    }

    /// One `commit` advances exactly one [`ProjectionId`].
    ///
    /// PS-23. Two projections over one store advance at their own rates, and
    /// neither one's commit may disturb the other's checkpoint or its rows.
    ///
    /// **Rejects:** `SingleRowCheckpointStore` — an adapter with a single-row
    /// checkpoint table, which is what a store that has only ever run one
    /// projection will write. It passes every other rule in this family, which is
    /// exactly why this one has to exist. `CheckpointOnlyStore` and
    /// `UncommittedTransactionStore` also fail it, at the row half and the
    /// checkpoint half respectively, and their registry rows say so.
    pub async fn distinct_projections_advance_independently<F: ProjectionFixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        must!(F: SECOND_HANDLE);

        let fixture = open().await;
        let writer = fixture.connect().await;

        let slower = ProjectionId::new("distinct_projections_advance_independently.slower");
        let faster = ProjectionId::new("distinct_projections_advance_independently.faster");

        let earlier = SequencePosition::FIRST;
        let later = after(earlier);

        let mut first = writer.begin();
        writer.probe_write(&mut first, PROBE_KEY, PROBE_VALUE);
        commit_ok(&writer, first, &slower, earlier, Authority::Live).await;

        let mut second = writer.begin();
        writer.probe_write(&mut second, SECOND_KEY, SECOND_VALUE);
        commit_ok(&writer, second, &faster, later, Authority::Live).await;

        let observer = fixture.connect().await;
        assert_eq!(
            checkpoint_ok(&observer, &slower).await,
            Checkpoint::Live { through: earlier },
            "one commit advances exactly one projection, and committing the \
             second one moved the first one's checkpoint. A single-row checkpoint \
             table does this silently, and the projection that lost the race \
             skips every event between the two positions forever"
        );
        assert_eq!(
            checkpoint_ok(&observer, &faster).await,
            Checkpoint::Live { through: later },
            "each projection must read back its own checkpoint"
        );
        assert_eq!(
            probe_read_ok(&observer, PROBE_KEY).await,
            Some(PROBE_VALUE),
            "neither projection's commit may disturb the other's rows, and the \
             row the first commit wrote is not readable"
        );
        assert_eq!(
            probe_read_ok(&observer, SECOND_KEY).await,
            Some(SECOND_VALUE),
            "neither projection's commit may disturb the other's rows, and the \
             row the second commit wrote is not readable"
        );

        RuleOutcome::Ran
    }

    // ---------------------------------------------------------------------
    // Reset
    //
    // `reset` is `commit`'s dual and the port's own undo, and until these five
    // rules landed nothing in the workspace could tell "returned to never run"
    // from "committed at the first position" — the substitute six deployment
    // scenarios out of six reached for, and the one that skips event 1
    // permanently and silently (`RUNBOOK.md:3904-3907`).
    // ---------------------------------------------------------------------

    /// A `reset` applies the caller's batch and returns the checkpoint to
    /// [`Checkpoint::NeverRun`] as **one** unit of work.
    ///
    /// PS-16. The pairing is the claim: both halves, or neither. A store that
    /// clears the rows and leaves the checkpoint is the runbook procedure — two
    /// statements on two connections — and it is what Norvant's night desk ran
    /// at 02:46:31 before the pod died at 02:46:33: the runner restarted, read
    /// the old checkpoint, resumed past it, applied sixty-one events into an
    /// empty table and **reported healthy**
    /// (`spec/E2E-CASES.md:458-481`).
    ///
    /// **Rejects:** `TwoStatementResetStore` — a `reset` that applies the
    /// caller's deletes and never touches the checkpoint. `CommitAtFirstResetStore`
    /// fails it too, at the same assertion and for its own reason, and its
    /// registry row says so.
    ///
    /// # PS-16's failure-injecting half, and why it needs no new capability
    ///
    /// The clause pairs its rule with *"a failure-injecting variant asserting
    /// that a `reset` that errors leaves both halves as they were"*, and the
    /// projection fixture's capability set — `SECOND_HANDLE`, `RESET_REFUSAL`,
    /// `COMMIT_FAULT` — contains nothing that arms a failing **reset**. Minting
    /// a fourth constant inside a rule-writing story is what
    /// `commit-rollback-and-drop-rules` refused to do and what the design
    /// record's 2026-08-14 amendment exists for, so this rule reaches the same
    /// observation through the port's own error path instead: a batch begun on a
    /// **different store instance** is rejected as
    /// [`ResetError::ForeignBatch`], and that is a `reset` that errored, which
    /// any conformant store can produce and no fixture has to arm.
    ///
    /// The two halves are asserted in that order — errored first, successful
    /// second — so "unchanged" is a *recorded* state rather than the empty one.
    /// Both are read through **fresh handles**.
    ///
    /// **What it deliberately does not assert.** That the committed rows were
    /// there to begin with. The before-state is read back and compared against,
    /// rather than asserted to be `Some`, so a store that committed no rows
    /// fails [`commit_is_atomic_with_the_read_model`] — the rule that owns that
    /// defect — instead of failing here for a reason this rule is not about.
    pub async fn reset_clears_rows_and_checkpoint_together<F: ProjectionFixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        // `must!`, not `require!`: both halves are read back through a second
        // handle, and a fixture that cannot open one cannot observe the
        // invariant at all.
        must!(F: SECOND_HANDLE);

        let fixture = open().await;
        let writer = fixture.connect().await;
        let id = ProjectionId::new("reset_clears_rows_and_checkpoint_together");
        let position = SequencePosition::FIRST;

        let mut batch = writer.begin();
        writer.probe_write(&mut batch, PROBE_KEY, PROBE_VALUE);
        writer.probe_write(&mut batch, SECOND_KEY, SECOND_VALUE);
        commit_ok(&writer, batch, &id, position, Authority::Live).await;

        // ---- the half PS-16 pairs its rule with: a `reset` that errors ----
        let before = fixture.connect().await;
        let rows_before = (
            probe_read_ok(&before, PROBE_KEY).await,
            probe_read_ok(&before, SECOND_KEY).await,
        );
        let checkpoint_before = checkpoint_ok(&before, &id).await;

        let stranger_fixture = open().await;
        let stranger = stranger_fixture.connect().await;
        let mut foreign = stranger.begin();
        stranger.probe_delete_all(&mut foreign);

        match writer.reset(foreign, &id).await {
            Err(ResetError::ForeignBatch) => {}
            outcome => panic!(
                "a `reset` handed a batch begun on a different store instance \
                 must be refused as `ResetError::ForeignBatch`, and this store \
                 answered {outcome:?}. The refusal is what makes PS-16's \
                 failure-injecting half reachable without a fixture that can arm \
                 a fault"
            ),
        }

        let after_failure = fixture.connect().await;
        assert_eq!(
            (
                probe_read_ok(&after_failure, PROBE_KEY).await,
                probe_read_ok(&after_failure, SECOND_KEY).await,
            ),
            rows_before,
            "a `reset` that reported failure must leave the read model exactly \
             as it was, and a fresh handle saw it change. The deletes the \
             refused batch carried are the caller's, and a store that applied \
             them anyway has cleared a read model whose caller was told the \
             operation failed"
        );
        assert_eq!(
            checkpoint_ok(&after_failure, &id).await,
            checkpoint_before,
            "a `reset` that reported failure must leave the checkpoint exactly \
             as the last successful commit left it"
        );

        // ---- and the successful one: both halves, together ----
        let mut clearing = writer.begin();
        writer.probe_delete_all(&mut clearing);
        reset_ok(&writer, clearing, &id).await;

        let observer = fixture.connect().await;
        assert_eq!(
            (
                probe_read_ok(&observer, PROBE_KEY).await,
                probe_read_ok(&observer, SECOND_KEY).await,
            ),
            (None, None),
            "a successful `reset` must apply the caller's deletes, and a fresh \
             handle can still read a row the reset batch asked to remove"
        );
        assert_eq!(
            checkpoint_ok(&observer, &id).await,
            Checkpoint::NeverRun,
            "a successful `reset` must return this projection's checkpoint to \
             `NeverRun` in the **same** unit of work as the deletes it applied, \
             and the rows went while the checkpoint stayed. That is the runbook \
             procedure — two statements on two connections — and a runner that \
             restarts between them reads the old checkpoint, resumes past it, \
             and applies the rest of the log into an empty table while reporting \
             healthy"
        );

        RuleOutcome::Ran
    }

    /// A `reset` moves exactly one `(store, ProjectionId)` pair.
    ///
    /// PS-17, `[FROZEN]`, and it is [ADR-0007](../../../.kb/decisions/0007-projection-runner-decodes.md)'s
    /// per-`(store, ProjectionId)` checkpoint applied to the operation that
    /// *removes* one. Kestrel Cold Chain's one projection store holds
    /// `van_stock`, rebuilt several times a day across 138 devices, and
    /// `fgas_ledger`, a hash chain a regulator already holds
    /// (`spec/E2E-CASES.md:482-497`).
    ///
    /// **Rejects:** `TruncatingResetStore` — a `SqliteProjectionStore::reset()`
    /// that truncates the checkpoint table. Cheap, obvious, and it destroys the
    /// append-only ledger sharing the file. `SingleRowCheckpointStore` fails it
    /// too, at the same assertion: one checkpoint row shared by every projection
    /// is the same disaster reached by a different route, and its registry row
    /// says so.
    ///
    /// # Why the reset batch is **empty**, and why that is the sharp version
    ///
    /// [`ProjectionProbe::probe_delete_all`] queues removal of every probe row —
    /// the whole read model, because the port has no idea which rows belong to
    /// which projection and PS-11's seam deliberately does not teach it. A rule
    /// that reset one id with a `probe_delete_all` batch would therefore delete
    /// the sibling's rows *by the caller's own instruction*, and would fail the
    /// oracle for something PS-17 does not constrain.
    ///
    /// Handing `reset` an empty batch is what makes the claim attributable: the
    /// caller asked for **no** deletes, so every row that disappears and every
    /// checkpoint other than this one that moves is the store's own doing.
    ///
    /// The assertion that the reset id's checkpoint did return to `NeverRun` is
    /// this rule's non-vacuity anchor — the same role the second row plays in
    /// [`dropped_batch_leaves_store_usable`]. Without it a `reset` that did
    /// nothing whatever would pass a rule about scoping.
    pub async fn reset_is_scoped_to_one_projection<F: ProjectionFixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        must!(F: SECOND_HANDLE);

        let fixture = open().await;
        let writer = fixture.connect().await;

        // Named after the deployment rather than `a` / `b`: the failure message
        // below is read by someone who has never seen this file.
        let van_stock = ProjectionId::new("reset_is_scoped_to_one_projection.van_stock");
        let fgas_ledger = ProjectionId::new("reset_is_scoped_to_one_projection.fgas_ledger");

        let rebuilt_through = SequencePosition::FIRST;
        let protected_through = after(rebuilt_through);

        let mut rebuildable = writer.begin();
        writer.probe_write(&mut rebuildable, PROBE_KEY, PROBE_VALUE);
        commit_ok(
            &writer,
            rebuildable,
            &van_stock,
            rebuilt_through,
            Authority::Live,
        )
        .await;

        let mut protected = writer.begin();
        writer.probe_write(&mut protected, SECOND_KEY, SECOND_VALUE);
        commit_ok(
            &writer,
            protected,
            &fgas_ledger,
            protected_through,
            Authority::Live,
        )
        .await;

        // What the protected projection looked like before, through a fresh
        // handle and compared against rather than asserted to be present: a
        // store that committed nothing at all is rejected by
        // `commit_is_atomic_with_the_read_model` and
        // `commit_advances_the_checkpoint`, which own that defect, rather than
        // failing here for a reason this rule is not about.
        let before = fixture.connect().await;
        let protected_row_before = probe_read_ok(&before, SECOND_KEY).await;
        let protected_checkpoint_before = checkpoint_ok(&before, &fgas_ledger).await;

        // Empty: the caller asked for no deletes at all, so anything that goes
        // missing below went missing because the store removed it.
        reset_ok(&writer, writer.begin(), &van_stock).await;

        let observer = fixture.connect().await;
        assert_eq!(
            checkpoint_ok(&observer, &fgas_ledger).await,
            protected_checkpoint_before,
            "a `reset` must be scoped to one `(store, ProjectionId)` pair, and \
             resetting one projection moved another one's checkpoint. A `reset` \
             that truncates the checkpoint table is one statement and it returns \
             every projection in the store to `NeverRun` — including the \
             append-only ledger that must never be rebuilt, which is then \
             rebuilt from an event log that no longer holds the events it was \
             built from"
        );
        assert_eq!(
            probe_read_ok(&observer, SECOND_KEY).await,
            protected_row_before,
            "a `reset` must be scoped to one `(store, ProjectionId)` pair, and \
             resetting one projection removed another one's rows — which this \
             reset's batch did not ask for, because it carried no deletes at all"
        );
        assert_eq!(
            checkpoint_ok(&observer, &van_stock).await,
            Checkpoint::NeverRun,
            "the reset projection's own checkpoint must have returned to \
             `NeverRun`, or this rule is asserting scoping about an operation \
             that did nothing"
        );

        RuleOutcome::Ran
    }

    /// A refused `reset` leaves the read model and the checkpoint exactly as
    /// they were, and is never reported as success.
    ///
    /// PS-18. The refusal *mechanism* is the port's and the *policy* is the
    /// domain's — the projection knows that `fgas_ledger` is a hash chain a
    /// regulator already holds, and the store does not — but a policy with no
    /// port-level mechanism is bypassed by anyone holding the store, which is
    /// every operator with a runbook.
    ///
    /// **The operative half is "changes nothing", not the error variant.** A
    /// rule that stopped at `matches!(err, ResetError::Refused)` would certify
    /// the mirror-image defect: a refusal reported perfectly, *after* the rows
    /// have gone.
    ///
    /// **Rejects:** `RefusalAsSuccessStore` — a store whose protection policy is
    /// consulted somewhere other than the write path, so `reset` returns `Ok`
    /// and does the work; and `RefusalAfterTheFactStore` — a store that issues
    /// its deletes and *then* checks the policy, reporting the refusal honestly
    /// over a read model that is already gone.
    ///
    /// # Why this rule is capability-gated, and the gate is the fixture's
    ///
    /// Nothing a caller holds can make a conformant `reset` answer `Refused`;
    /// that is the property under test. So the protection belongs to the store,
    /// [`RESET_REFUSAL`](crate::ProjectionFixture::RESET_REFUSAL) is where a
    /// fixture says whether it has any, and
    /// [`protect_from_reset`](crate::ProjectionFixture::protect_from_reset) is
    /// how this rule names the projection to protect. A store with no protection
    /// policy declines and this rule reports a skip carrying its own stated
    /// reason — which is what `MemoryProjectionStore` does, so the reference run
    /// prints that line rather than a pass nobody earned.
    pub async fn refused_reset_changes_nothing<F: ProjectionFixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        // `must!` first, and the order is load-bearing for
        // `failed_commit_leaves_both_unchanged`'s reason: a fixture that
        // declines both is failing the contract, and reporting that as a skip
        // would hide it behind a trade it was entitled to make.
        must!(F: SECOND_HANDLE);
        require!(F: RESET_REFUSAL);

        let fixture = open().await;
        let writer = fixture.connect().await;
        let id = ProjectionId::new("refused_reset_changes_nothing");
        let position = SequencePosition::FIRST;

        let mut batch = writer.begin();
        writer.probe_write(&mut batch, PROBE_KEY, PROBE_VALUE);
        commit_ok(&writer, batch, &id, position, Authority::Live).await;

        fixture.protect_from_reset(&id).await;

        // The before-state, through a fresh handle and compared against rather
        // than asserted to be present: a store that committed nothing is
        // rejected by the rule that owns that defect, not by this one.
        let before = fixture.connect().await;
        let row_before = probe_read_ok(&before, PROBE_KEY).await;
        let checkpoint_before = checkpoint_ok(&before, &id).await;

        let mut clearing = writer.begin();
        writer.probe_delete_all(&mut clearing);

        match writer.reset(clearing, &id).await {
            Err(ResetError::Refused) => {}
            outcome => panic!(
                "this fixture declares `RESET_REFUSAL` supported and asked its \
                 store to protect this projection, so `reset` must answer \
                 `Err(ResetError::Refused)` — and it answered {outcome:?}. A \
                 refusal reported as success is the operator's runbook \
                 succeeding against the one projection the domain protects"
            ),
        }

        let observer = fixture.connect().await;
        assert_eq!(
            probe_read_ok(&observer, PROBE_KEY).await,
            row_before,
            "a refused `reset` must leave the read model exactly as it was, and \
             a fresh handle saw it change. A store that issues its deletes and \
             checks its protection policy afterwards reports the refusal \
             perfectly honestly over a read model that is already gone, and \
             every rule asserting only the error variant certifies it"
        );
        assert_eq!(
            checkpoint_ok(&observer, &id).await,
            checkpoint_before,
            "a refused `reset` must leave the checkpoint exactly as it was: a \
             refusal is not success, and a projection whose rows survived while \
             its checkpoint went to `NeverRun` is rebuilt from the top over rows \
             that are still there"
        );

        RuleOutcome::Ran
    }

    /// A projection the store has **never seen** reads back as
    /// [`Checkpoint::NeverRun`].
    ///
    /// **PS-38's second sentence**, which is the clause this rule is written
    /// against: *"a `ProjectionId` no successful `commit` has named MUST read as
    /// `Checkpoint::NeverRun`"* (`spec/SPECIFICATION.md:5448-5464`). §4.11 lists
    /// the rule against PS-19 as well, because it is the same distinction that
    /// clause is about — *never run* told apart from *committed at the first
    /// position* — asked before any `reset` has happened; but PS-19's own MUST
    /// stays scoped *after a successful `reset`* and does not reach an unseen id,
    /// and it says so about itself (`:5300-5316`). **Cite PS-38 here.** Between
    /// 2026-08-14 and ADR-0030 this rule was held out of the suite for exactly
    /// that reason: no clause's MUST obliged the read, so asserting it would have
    /// widened a `[FROZEN]` clause by test
    /// ([ADR-0030](../../../.kb/decisions/0030-the-checkpoint-reports-the-commits-that-happened.md)
    /// is what minted the obligation instead of editing PS-19).
    ///
    /// **Rejects:** `PresumedLiveCheckpointStore` — a store that resolves a
    /// missing checkpoint row with `.unwrap_or(Checkpoint::Live { through: FIRST })`.
    /// The specification names that shape by name, because it is the natural one
    /// rather than a contrivance: an adapter whose `reset` writes an explicit
    /// `NeverRun` row satisfies PS-19's MUST verbatim and still answers `Live`
    /// for an id nobody has ever committed (`spec/SPECIFICATION.md:5300-5316`).
    /// PS-38 is what makes that store non-conformant rather than merely
    /// surprising, and its `Rejects` field names the wider version of the same
    /// defect — a store whose backing state lives per *handle* rather than per
    /// store (`:5462-5470`).
    ///
    /// # The assertion is on the **variant**
    ///
    /// Comparing a checkpoint against any [`SequencePosition`] is
    /// simultaneously a CF-6 violation and the exact value the defective store
    /// writes, so a rule written that way cannot tell the two mutants of this
    /// family apart and both walk free. [`Checkpoint`] is a three-variant enum
    /// precisely so this assertion can be made without naming a position
    /// (`spec/SPECIFICATION.md:4643-4658`).
    ///
    /// # Why it spells no capability gate at all
    ///
    /// It reads one checkpoint through one handle and writes nothing, so it
    /// needs neither a second handle nor a protected projection. A fixture
    /// declining every capability still runs it and still passes it, which is
    /// correct: the capabilities it declined are not the ones this rule spends.
    pub async fn fresh_projection_has_no_checkpoint<F: ProjectionFixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;

        // Committed to by nothing, in this rule or any other: every rule in this
        // family names its ids after itself.
        let unseen = ProjectionId::new("fresh_projection_has_no_checkpoint.never-committed-to");

        assert_eq!(
            checkpoint_ok(&store, &unseen).await,
            Checkpoint::NeverRun,
            "a projection this store has never seen must read back as \
             `Checkpoint::NeverRun`. A store that resolves a missing checkpoint \
             row to `Live` has told a runner that a read model nobody has ever \
             built is authoritative and already considered through a position — \
             so the runner resumes past the events it has never applied, and \
             they are skipped permanently and silently"
        );

        RuleOutcome::Ran
    }

    /// `reset` is distinguishable from `commit(empty_batch, id, FIRST, Live)`,
    /// in the checkpoint **and** in the replay that follows it.
    ///
    /// PS-19 and PS-20. The substitute is the one every deployment reaches for —
    /// **six scenarios out of six reached for it and all six got it wrong**
    /// (`RUNBOOK.md:3904-3907`) — and it is wrong in the way nobody ever finds:
    /// the checkpoint reads back as a position, the runner resumes *strictly
    /// after* it, and event 1 is skipped permanently and silently
    /// (`spec/E2E-CASES.md:437-456`).
    ///
    /// **Rejects:** `CommitAtFirstResetStore` — a `reset` implemented as
    /// `commit(batch, id, FIRST, Live)`. It compiles, it returns `Ok`, and the
    /// checkpoint moves, so every rule asserting only that *something changed*
    /// passes it. `TwoStatementResetStore` fails it too, at the same assertion:
    /// a checkpoint that never moved is `Live` as well.
    ///
    /// # Both halves, and neither is sufficient alone
    ///
    /// The **state** half compares the two checkpoints by variant —
    /// `NeverRun` against `Live { .. }` — never by position: comparing a
    /// checkpoint against any [`SequencePosition`] is simultaneously a CF-6
    /// violation and the exact value the defective store writes, so a rule
    /// written that way could not tell the two apart. [`Checkpoint`] is a
    /// three-variant enum precisely so this assertion can be made without naming
    /// a position (`spec/SPECIFICATION.md:4643-4658`). The **consequence** half
    /// derives a resume point from each under the port's own rule — in one
    /// private helper shared by both arms, so the derivation is stated once —
    /// and asserts the event at the store's first position is applied in the
    /// reset case and *not* in the substitute's. The variant
    /// check alone would not see the consequence; the resume check alone would
    /// not see the state.
    ///
    /// No runner is built. PS-20's property is a claim about how a resume point
    /// is derived from a checkpoint, and that derivation is four lines the rule
    /// owns — the runner itself belongs to a later phase and a different crate.
    ///
    /// # Why each checkpoint is read immediately after its own operation
    ///
    /// So that a store which shares one checkpoint row between projections fails
    /// [`reset_is_scoped_to_one_projection`] and
    /// [`distinct_projections_advance_independently`] — the rules that own that
    /// defect — rather than this one, which is about a *single* projection's
    /// state being told apart from another single projection's.
    pub async fn reset_is_not_commit_at_first<F: ProjectionFixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        must!(F: SECOND_HANDLE);

        let fixture = open().await;
        let writer = fixture.connect().await;

        let rebuilt = ProjectionId::new("reset_is_not_commit_at_first.reset");
        let substituted = ProjectionId::new("reset_is_not_commit_at_first.commit_at_first");

        // The store's first position, and the only position this rule names: it
        // is the one the substitute commits at and the one a replay must decide
        // about.
        let first = SequencePosition::FIRST;

        let mut applied = writer.begin();
        writer.probe_write(&mut applied, PROBE_KEY, PROBE_VALUE);
        commit_ok(&writer, applied, &rebuilt, first, Authority::Live).await;

        let mut clearing = writer.begin();
        writer.probe_delete_all(&mut clearing);
        reset_ok(&writer, clearing, &rebuilt).await;

        let after_reset = checkpoint_ok(&fixture.connect().await, &rebuilt).await;

        // The operator's substitute, on a sibling id in the same store: an empty
        // batch committed at the first position, which is what "reset it
        // properly" means to everyone who has not got a `reset`.
        commit_ok(
            &writer,
            writer.begin(),
            &substituted,
            first,
            Authority::Live,
        )
        .await;

        let after_substitute = checkpoint_ok(&fixture.connect().await, &substituted).await;

        assert_ne!(
            core::mem::discriminant(&after_reset),
            core::mem::discriminant(&after_substitute),
            "a projection that was `reset` and one that was committed at the \
             first position must be **distinguishable by variant**, and this \
             store reported {after_reset:?} for the reset one and \
             {after_substitute:?} for the substitute. Collapsing the two is the \
             defect this rule exists for: `commit(empty, id, FIRST)` reads back \
             as a position, so a runner resumes after it"
        );

        assert!(
            resumes_over(after_reset, first),
            "a runner resuming from {after_reset:?} must start at the store's \
             first position **inclusive**, so the event at {first} is applied \
             after a reset. A reset that leaves any position behind makes the \
             rebuild it exists for start one event late"
        );
        assert!(
            !resumes_over(after_substitute, first),
            "a runner resuming from {after_substitute:?} resumes **strictly \
             after** the recorded position, so the event at {first} is not \
             applied — which is exactly why `commit(empty, id, FIRST)` is not a \
             reset. This store answered {after_substitute:?} and a replay would \
             still apply event 1, so the two states are the same state and the \
             rule above only appeared to hold"
        );

        RuleOutcome::Ran
    }

    // ---------------------------------------------------------------------
    // Reading, and rebuilding
    //
    // Three of `ProjectionStore`'s guarantees are about a *read*: whether a
    // batch can see its own pending writes, whether a rebuild produces the same
    // read model however it is chunked, and whether a store rebuilding a
    // projection is distinguishable from one serving it live. The first two are
    // gated on the probe's own switch, because an adapter that buffers its
    // writes has no read path to answer them with and PS-12 permits that
    // outright.
    // ---------------------------------------------------------------------

    /// An open batch can be read through, and sees its own pending writes.
    ///
    /// PS-12. A projection that maintains a counter, a running total or any
    /// other read-modify-write does `get` then `set` **inside** the batch, and a
    /// store whose batch `get` answers from committed state loses every write
    /// the batch has not committed yet — silently, and worst exactly when the
    /// chunk is large.
    ///
    /// **Rejects:** `CommittedReadBatchStore` — a batch `get` that goes to the
    /// store. It is the *natural* shape rather than a contrivance: one round
    /// trip on the connection the batch is already holding is what an adapter
    /// author writes, and it is right for every projection that never reads what
    /// it wrote.
    ///
    /// The assertion is on the **value** written, not on `is_some()`: a store
    /// that answered `Some(0)` for every key would pass a presence check while
    /// losing the write.
    ///
    /// # What a skip here does not cover, stated rather than left to be inferred
    ///
    /// Declaring [`READS_THROUGH_BATCH`](happenstance_core::ProjectionProbe::READS_THROUGH_BATCH)
    /// `false` is a conformant answer — PS-12's second arm permits a batch with
    /// no read path at all — and this rule then reports a skip. What that skip
    /// does **not** say is that the adapter is safe for read-modify-write
    /// projections. It says the opposite: such an adapter has no read path for a
    /// projection's `apply` to use, which is the situation §4.4 describes going
    /// wrong, and [`rebuild_is_chunk_size_invariant`] is unverified for it for
    /// exactly the same reason.
    pub async fn batch_reads_reflect_pending_writes<F: ProjectionFixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        // No `must!`: the read under test is through the *same* handle that owns
        // the batch, by construction. A fixture that declines `SECOND_HANDLE`
        // still runs this rule and still passes it.
        require_read_through!(F);

        let fixture = open().await;
        let writer = fixture.connect().await;

        let mut batch = writer.begin();
        writer.probe_write(&mut batch, PROBE_KEY, PROBE_VALUE);

        assert_eq!(
            writer.probe_read_through(&batch, PROBE_KEY),
            Some(PROBE_VALUE),
            "a store declaring `READS_THROUGH_BATCH` must let an open batch see \
             its own pending writes, and this one answered from committed state. \
             A projection doing `get` then `set` inside one batch then reads the \
             value from *before* the batch began, and every increment after the \
             first is lost — with no error, and least visibly when the chunk is \
             largest"
        );

        RuleOutcome::Ran
    }

    /// A rebuild produces the same read model however it is chunked.
    ///
    /// PS-13 and PS-14, both `[FROZEN]`. A rebuild is run at whatever chunk size
    /// fits the operator's memory budget, and an adapter that lets the chunk size
    /// change the answer turns "how much RAM did we give it" into a correctness
    /// variable nobody logs.
    ///
    /// **Rejects:** `CommittedReadBatchStore`, whose batch `get` answers from
    /// committed state, and `FirstWriteWinsBatchStore`, whose batch stages writes
    /// with `entry().or_insert(…)` and keeps the **first** value for a key — the
    /// natural spelling when the batch is thought of as a dedup buffer. Its reads
    /// through the batch are honest, so it passes the rule above and fails only
    /// this one.
    ///
    /// # The sequence, and why each step is a read-modify-write
    ///
    /// One fixed sequence — `a, a, b, a, b, a` — replayed at chunk sizes **1**,
    /// **3** and whole-log against **three isolated stores**, one `open()` per
    /// run. Each step reads the key *through the batch* and writes one more than
    /// it saw, so the correct final state is `a = 4, b = 2` at every chunking.
    ///
    /// A plain `set` instead of an increment is the trap: last-writer-wins is
    /// chunk-insensitive **by construction**, so a rule written that way passes
    /// against a store that cannot read through its own batch and certifies PS-13
    /// and PS-14 on nothing. The sizes are chosen for the same reason — 1 is the
    /// degenerate case where pending and committed state coincide and a broken
    /// store gets it right, 3 is the smallest size that puts a repeated key both
    /// inside one chunk and across a boundary, and whole-log is what a rebuild
    /// actually runs at.
    ///
    /// # Two things it deliberately does not do
    ///
    /// It compares **read models**, never checkpoints and never positions: the
    /// positions it commits at are its own, deliberately non-contiguous so that a
    /// later edit growing a `position == index + 1` assumption is visible, and
    /// strictly increasing so it does not trip PS-22 for a reason that has
    /// nothing to do with what it is testing.
    ///
    /// And the end-of-run comparison uses `probe_read`, which is the *suite*
    /// reading committed state after the fact. That is not the out-of-band read
    /// PS-13 forbids: the clause constrains what a **projection's** `apply` may
    /// read while it is writing, and every read this rule makes on that path goes
    /// through the batch.
    pub async fn rebuild_is_chunk_size_invariant<F: ProjectionFixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        require_read_through!(F);

        let mut runs = Vec::new();

        for chunk in [1_usize, 3, REBUILD_SEQUENCE.len()] {
            // One `open()` per run: three isolated backing stores, so a run can
            // neither see nor be seen by the others.
            let fixture = open().await;
            let writer = fixture.connect().await;
            let id = ProjectionId::new("rebuild_is_chunk_size_invariant");

            // Non-contiguous on purpose, and strictly increasing across chunks.
            let mut position = SequencePosition::FIRST;

            for slice in REBUILD_SEQUENCE.chunks(chunk) {
                let mut batch = writer.begin();
                for key in slice {
                    // The read-modify-write, entirely through the batch. A plain
                    // `set` here would make this rule chunk-insensitive and
                    // therefore decorative.
                    let seen = writer.probe_read_through(&batch, key).unwrap_or(0);
                    writer.probe_write(&mut batch, key, seen + 1);
                }
                commit_ok(&writer, batch, &id, position, Authority::Rebuilding).await;
                position = after(after(position));
            }

            runs.push((
                chunk,
                probe_read_ok(&writer, PROBE_KEY).await,
                probe_read_ok(&writer, SECOND_KEY).await,
            ));
        }

        let (baseline_chunk, baseline_first, baseline_second) = runs[0];
        for &(chunk, first, second) in &runs[1..] {
            assert_eq!(
                (first, second),
                (baseline_first, baseline_second),
                "a rebuild must produce the same read model however it is \
                 chunked, and replaying the same sequence at chunk size {chunk} \
                 produced a different read model from chunk size \
                 {baseline_chunk}. Every step of that sequence reads its key \
                 through the open batch and writes one more than it saw, so a \
                 store whose batch reads answer from committed state — or whose \
                 batch keeps the first staged value for a key rather than the \
                 last — loses every write after the first *within* a chunk, and \
                 the operator's memory budget silently becomes a correctness \
                 variable"
            );
        }

        RuleOutcome::Ran
    }

    /// A projection being rebuilt is distinguishable from one serving live.
    ///
    /// PS-24. A reader deciding whether the rows in front of it are
    /// authoritative gets its answer from the checkpoint's **variant**, and a
    /// store that rebuilds in place behind a single position field cannot tell
    /// it: the read model is half-built, the checkpoint says `Live`, and the
    /// dashboard reads the wrong number without anything anywhere reporting a
    /// problem.
    ///
    /// **Rejects:** `LiveOnlyCheckpointStore` — a store that ignores `Authority`
    /// and records `Live` whatever the commit claimed. It is the obvious reading
    /// of the port and the only one `Option<SequencePosition>` could express
    /// before [`Checkpoint`] existed.
    ///
    /// Everything it needs is on the port's own signature — `commit`'s
    /// `authority` argument and `checkpoint`'s return — so **no runner and no
    /// rebuild driver is built**; the six runner-dependent rules of §4.11 belong
    /// to the workspace e2e crate under CF-36.
    ///
    /// # Two details the specification fixes and this rule does not soften
    ///
    /// It asserts the **variant** and never the `through` value it carries. And
    /// it asserts nothing at all immediately after the reset that opens it: a
    /// rebuild that has committed nothing reads `NeverRun`, not `Rebuilding`,
    /// and that is correct — both mean the rows are not authoritative and the
    /// reader's decision is the same.
    pub async fn rebuilding_is_distinguishable_from_live<F: ProjectionFixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        // `must!`: every checkpoint below is read back through a fresh handle,
        // because a claim about what a *reader* sees is a claim about what
        // something other than the writer sees.
        must!(F: SECOND_HANDLE);

        let fixture = open().await;
        let writer = fixture.connect().await;
        let id = ProjectionId::new("rebuilding_is_distinguishable_from_live");

        // A rebuild starts by clearing what is there. `reset` is a *step* here
        // and not the subject: the rules that interrogate it are the reset
        // family's.
        let mut clearing = writer.begin();
        writer.probe_delete_all(&mut clearing);
        reset_ok(&writer, clearing, &id).await;

        let first_chunk = SequencePosition::FIRST;
        let second_chunk = after(first_chunk);
        let caught_up = after(second_chunk);

        for (position, key, value) in [
            (first_chunk, PROBE_KEY, PROBE_VALUE),
            (second_chunk, SECOND_KEY, SECOND_VALUE),
        ] {
            let mut batch = writer.begin();
            writer.probe_write(&mut batch, key, value);
            commit_ok(&writer, batch, &id, position, Authority::Rebuilding).await;

            let observer = fixture.connect().await;
            let checkpoint = checkpoint_ok(&observer, &id).await;
            assert!(
                matches!(checkpoint, Checkpoint::Rebuilding { .. }),
                "a commit claiming `Authority::Rebuilding` must leave a \
                 checkpoint a reader can recognise as a rebuild in flight, and \
                 this store reported {checkpoint:?}. A store that rebuilds in \
                 place behind a single position field says `Live` over a \
                 half-built read model, and every reader that asked whether the \
                 rows were authoritative was told yes"
            );
        }

        let mut caught_up_batch = writer.begin();
        writer.probe_write(&mut caught_up_batch, ANCHOR_KEY, PROBE_VALUE);
        commit_ok(&writer, caught_up_batch, &id, caught_up, Authority::Live).await;

        let observer = fixture.connect().await;
        let checkpoint = checkpoint_ok(&observer, &id).await;
        assert!(
            matches!(checkpoint, Checkpoint::Live { .. }),
            "the commit that finishes a rebuild claims `Authority::Live`, and \
             the checkpoint must then say the rows are authoritative — this \
             store reported {checkpoint:?}. A store that cannot leave the \
             rebuilding state leaves every reader treating a finished read model \
             as untrustworthy for ever"
        );

        RuleOutcome::Ran
    }
}

// =====================================================================
// The enumeration, the emitters, and the macro
// =====================================================================

/// Hands the complete projection store rule set to `$callback`.
///
/// This family's single enumeration (CF-22). It lives here beside the rules it
/// names, for the reason the module documentation gives: a projection rule in
/// `suite.rs` is an orphan of the event-store enumeration.
///
/// # Examples
///
/// ```
/// macro_rules! rule_names {
///     ($($name:ident),* $(,)?) => { [ $( stringify!($name) ),* ] };
/// }
///
/// let names = happenstance_testkit::for_each_projection_store_rule!(rule_names);
/// assert!(names.contains(&"commit_is_atomic_with_the_read_model"));
/// ```
#[macro_export]
macro_rules! for_each_projection_store_rule {
    // Raw token trees rather than `$cb:path`, for the reason
    // `for_each_event_store_rule!` records: a parsed `path` fragment cannot sit
    // in callee position inside an expression, which would forbid the
    // `let names = …` form the example above and the orphan meta-test both need.
    ($($callback:tt)+) => {
        $($callback)+! {
            // --- The baseline pair -----------------------------------------
            commit_advances_the_checkpoint,
            commit_is_atomic_with_the_read_model,

            // --- PS-1's second conjunct ------------------------------------
            failed_commit_leaves_both_unchanged,

            // --- The commit path, differentially ---------------------------
            rollback_leaves_both_unchanged,
            dropped_batch_leaves_store_usable,
            commit_rejects_a_foreign_batch,
            commit_accepts_a_position_the_batch_did_not_write,
            commit_rejects_a_regressing_position,
            distinct_projections_advance_independently,

            // --- Reset -----------------------------------------------------
            reset_clears_rows_and_checkpoint_together,
            reset_is_scoped_to_one_projection,
            refused_reset_changes_nothing,
            fresh_projection_has_no_checkpoint,
            reset_is_not_commit_at_first,

            // --- Reading, and rebuilding -----------------------------------
            batch_reads_reflect_pending_writes,
            rebuild_is_chunk_size_invariant,
            rebuilding_is_distinguishable_from_live,
        }
    };
}

/// Emits one `#[tokio::test]` per projection rule.
///
/// The caller's crate needs `tokio` with `macros` and `rt` in its
/// `dev-dependencies`; the attribute resolves in the caller's scope, not here.
#[doc(hidden)]
#[macro_export]
macro_rules! __emit_projection_tokio {
    ($($name:ident),* $(,)?) => {
        $(
            #[tokio::test]
            async fn $name() {
                $crate::__private::projection_rules::$name(__conformance_fixture)
                    .await
                    .report(::core::stringify!($name));
            }
        )*
    };
}

/// Emits one plain `#[test]` per projection rule, driven by
/// [`block_on`](crate::block_on).
///
/// No runtime, no dependency, one thread. This is the harness that proves the
/// projection family never quietly needs `tokio` — and, because `block_on`
/// imposes no `Send` bound, that it works against a `!Send` adapter.
#[doc(hidden)]
#[macro_export]
macro_rules! __emit_projection_blocking {
    ($($name:ident),* $(,)?) => {
        $(
            #[test]
            fn $name() {
                $crate::__private::block_on($crate::__private::projection_rules::$name(__conformance_fixture))
                    .report(::core::stringify!($name));
            }
        )*
    };
}

/// Emits one `#[wasm_bindgen_test]` per projection rule.
///
/// The caller's crate needs `wasm-bindgen-test` in its `dev-dependencies`, gated
/// on `cfg(target_arch = "wasm32")`; as with the tokio emitter the attribute
/// resolves in the caller's scope.
///
/// This is the one projection emitter that does not call
/// [`RuleOutcome::report`](crate::RuleOutcome::report), and the reason is the
/// target rather than taste: `println!` writes nowhere on
/// `wasm32-unknown-unknown` — measured, not assumed — so a skip reported through
/// it leaves no record on the only target the two-flavour design exists for.
/// `console_log!` is what the runner captures. An emitter that called `report`
/// here would compile, run, pass, and discard every stated reason.
#[doc(hidden)]
#[macro_export]
macro_rules! __emit_projection_wasm {
    ($($name:ident),* $(,)?) => {
        $(
            #[::wasm_bindgen_test::wasm_bindgen_test]
            async fn $name() {
                let __outcome = $crate::__private::projection_rules::$name(__conformance_fixture).await;
                if let Some(__line) = __outcome.skip_line(::core::stringify!($name)) {
                    ::wasm_bindgen_test::console_log!("{}", __line);
                }
            }
        )*
    };
}

/// Every `pub async fn` declared at the top level of this file's `rules` module.
///
/// rustfmt pins the shape of those declarations, which is what makes a textual
/// scan defensible here: a rule is always `    pub async fn <name>` at one level
/// of indentation inside `pub mod rules`. It is the same prefix
/// `spec_trace::collect_rules` matches, so the two cannot disagree about what a
/// rule is.
#[cfg(test)]
fn declared_projection_rules() -> Vec<&'static str> {
    const SOURCE: &str = include_str!("projection.rs");
    const PREFIX: &str = "    pub async fn ";

    SOURCE
        .lines()
        .filter_map(|line| line.strip_prefix(PREFIX))
        .map(|rest| {
            rest.split(|c: char| !c.is_alphanumeric() && c != '_')
                .next()
                .unwrap_or("")
        })
        .collect()
}

/// CF-24, for the projection family: no rule may exist in
/// [`rules`](crate::projection::rules) without appearing in
/// [`for_each_projection_store_rule!`](crate::for_each_projection_store_rule).
///
/// # What this checks, and what it cannot
///
/// One direction is free and must stay free: every emitter expands to
/// `$crate::__private::projection_rules::$name`, so a name in the enumeration with no rule
/// behind it is `error[E0425]` in every harness. An emitter resolving rules
/// through a `HashMap<&str, fn>` would convert that compile error into a
/// run-time one and is forbidden for exactly that reason.
///
/// The *orphan* direction — a rule with no registration — is the one Rust's lack
/// of reflection makes non-free, and it is what CF-24 is about. It is checked
/// here by parsing this module's own source, which `include_str!` bakes into the
/// binary, so the test needs neither a filesystem nor a particular working
/// directory.
///
/// It does **not** catch a rule introduced by a macro expansion, by a `pub use`
/// re-export, or from a `#[path]`-included file. The scan is fail-loud rather
/// than fail-open: were it ever to match nothing — the module renamed, the file
/// reformatted, `include_str!` pointing elsewhere — the second assertion reports
/// every registered rule as missing rather than passing silently. A scan that
/// quietly matches nothing is a meta-test that has stopped existing.
#[cfg(test)]
#[test]
fn no_orphan_projection_rules() {
    let registered = crate::for_each_projection_store_rule!(crate::__emit_rule_names);
    let declared = declared_projection_rules();

    let orphans: Vec<_> = declared
        .iter()
        .filter(|name| !registered.contains(*name))
        .collect();
    assert!(
        orphans.is_empty(),
        "these rules exist in `projection::rules` but are absent from \
         `for_each_projection_store_rule!`, so no harness runs them: {orphans:?}"
    );

    let missing: Vec<_> = registered
        .iter()
        .filter(|name| !declared.contains(*name))
        .collect();
    assert!(
        missing.is_empty(),
        "these rules are registered but were not found in `projection.rs` — \
         either they moved, or the source scan no longer matches the way they \
         are written: {missing:?}"
    );
}

/// A declined `RESET_REFUSAL` reaches the author as a **reported skip carrying
/// the fixture's own reason**, not as a pass and not as an absence.
///
/// CF-18 on real values for one of the two capabilities the reference fixture
/// declines: `MemoryProjectionFixture` supports `SECOND_HANDLE`, declines
/// `RESET_REFUSAL` because the store holds no protection policy, and declines
/// `COMMIT_FAULT` because it applies both halves of a commit under one write
/// lock. So a reference run answers **two** rules with a skip —
/// `failed_commit_leaves_both_unchanged` (`COMMIT_FAULT`) and
/// `refused_reset_changes_nothing` (`RESET_REFUSAL`) — and this test owns the
/// second of them.
///
/// **The set itself is pinned elsewhere, and that is deliberate.**
/// `assert_reference_projection_declensions`
/// (`crates/happenstance-testkit/tests/mutation_coverage.rs:3553`) asserts the
/// reference fixture's skip set by *equality*, in enumeration order, with each
/// skip's capability and stated reason. Read that assertion as the authority for
/// which rules skip and how many: a count restated in prose is a number nothing
/// in the gate reads, and this family has already had four of them go stale.
///
/// # Why the assertion is on the value and never on stdout
///
/// [`RuleOutcome::report`](crate::RuleOutcome::report) writes to stdout, which
/// libtest suppresses for a *passing* test unless `--show-output` is passed, and
/// which does not exist at all on `wasm32-unknown-unknown`
/// (`crates/happenstance-testkit/src/contract.rs:515-531`). A test that scraped
/// the printed line would therefore assert nothing on the one target the
/// two-flavour design exists for. The machine-checked half is this equality; the
/// human-facing half is the line `skip_line` renders from the same two fields.
///
/// The expected reason is read back from the fixture's own `const` rather than
/// repeated here as a literal, for the reason the event-store family's version
/// gives: two copies of the sentence would let the report carry someone else's
/// words while this test stayed green.
#[cfg(test)]
#[test]
fn a_declined_reset_refusal_is_reported_with_the_fixtures_reason() {
    use crate::fixtures::MemoryProjectionFixture;
    use crate::{ProjectionFixture, RuleOutcome};

    let stated = <MemoryProjectionFixture as ProjectionFixture>::RESET_REFUSAL
        .reason()
        .expect("the reference projection fixture declines `RESET_REFUSAL`");

    let outcome = crate::block_on(rules::refused_reset_changes_nothing(async || {
        MemoryProjectionFixture::new()
    }));

    assert_eq!(
        outcome,
        RuleOutcome::Skipped {
            capability: "RESET_REFUSAL",
            reason: stated,
        },
        "a fixture whose store holds no protection policy must get \
         `refused_reset_changes_nothing` as a skip naming the associated const \
         it can change and carrying its own stated reason — never a silent pass, \
         and never a rule `#[cfg]`-ed out of the binary"
    );
}

/// Two `open()` calls make two isolated backing stores, not two handles onto
/// one.
///
/// The property every rule taking `impl AsyncFn() -> F` rests on, checked
/// against the reference fixture. `commit_rejects_a_foreign_batch` depends on it
/// directly: a batch begun on one store and committed to another must be
/// rejected, and a fixture handing back one shared store makes that rule
/// unwritable rather than failing.
///
/// Driven by [`block_on`](crate::block_on) rather than `#[tokio::test]`, because
/// `tokio` is a dev-dependency of the *non-wasm32* target table only and this
/// file's unit tests are type-checked for `wasm32-unknown-unknown` by the
/// mandatory conformance-harness step.
#[cfg(test)]
#[test]
fn two_opens_make_two_isolated_stores() {
    use happenstance_core::{
        Authority, Checkpoint, ProjectionId, ProjectionStore, SequencePosition,
    };

    use crate::ProjectionFixture;
    use crate::fixtures::MemoryProjectionFixture;

    crate::block_on(async {
        let open = async || MemoryProjectionFixture::new();

        let first = open().await;
        let second = open().await;

        let writer = first.connect().await;
        let observer = second.connect().await;
        let id = ProjectionId::new("two_opens_make_two_isolated_stores");
        let position = SequencePosition::FIRST;

        let mut batch = writer.begin();
        happenstance_core::ProjectionProbe::probe_write(&writer, &mut batch, "depot-7", 12);
        writer
            .commit(batch, &id, position, Authority::Live)
            .await
            .expect("committing to the first fixture's store should succeed");

        // The control. Without it the isolation assertions below would hold for
        // a fixture whose `commit` silently did nothing.
        assert_eq!(
            writer
                .checkpoint(&id)
                .await
                .expect("reading the first store's checkpoint should succeed"),
            Checkpoint::Live { through: position },
            "the commit must actually have landed in the first fixture's store"
        );

        assert_eq!(
            observer
                .checkpoint(&id)
                .await
                .expect("reading the second store's checkpoint should succeed"),
            Checkpoint::NeverRun,
            "two fixture instances must share no backing store, but the second \
             saw a checkpoint committed through the first"
        );
        assert_eq!(
            happenstance_core::ProjectionProbe::probe_read(&observer, "depot-7")
                .await
                .expect("reading the second store's read model should succeed"),
            None,
            "two fixture instances must share no backing store, but the second \
             saw a row written through the first"
        );
    });
}
