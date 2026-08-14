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
//! Both rules below pass against `MemoryProjectionStore`, which is the oracle
//! and is *supposed* to pass. That the suite can **fail** a wrong store is a
//! different claim, and the store that carries it —`CheckpointOnlyStore`, which
//! commits the checkpoint and discards the write set — arrives with
//! `projection-mutant-registry`. Until it does, each rule's own documentation
//! names the defect it rejects and this paragraph says the debt is carried
//! rather than discharged
//! ([ADR-0010](../../../.kb/decisions/0010-the-suite-must-prove-itself.md)).

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
/// # Why there is no `require!` beside it yet
///
/// Nothing gates on a declinable capability yet: `RESET_REFUSAL`'s rule is
/// PS-18's and arrives with the reset family. A `require!` defined now would be
/// an unused macro — a `-D warnings` failure under `unused_macros` — and, worse,
/// a gate nothing calls, which is the decorative shape this repository's own
/// corollary names. It lands with the first rule that needs it.
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
        Authority, Checkpoint, ProjectionId, ProjectionProbe, ProjectionStore, SequencePosition,
    };

    use crate::{ProjectionFixture, RuleOutcome};

    /// The probe key both rules write through.
    ///
    /// One constant rather than a literal per rule, so that a rule reading back
    /// a key it never wrote is a compile-time impossibility rather than a typo
    /// nobody notices for a phase.
    const PROBE_KEY: &str = "depot-7";

    /// The probe value both rules write.
    ///
    /// Deliberately not `0` or `1`: a store that returns a default rather than
    /// the value it was handed passes a read-back asserting either of those.
    const PROBE_VALUE: u64 = 12;

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

    // ---------------------------------------------------------------------
    // The baseline pair
    //
    // Two rules, not seventeen. The rest of §4.11 is *differential* against
    // these two, so they land first and alone; a third rule here would be a
    // third rule with no mutant behind it.
    // ---------------------------------------------------------------------

    /// A commit moves the projection's checkpoint to the position it was given.
    ///
    /// The baseline the rest of this family is differential against, and the one
    /// rule in it that is not about coupling: it asserts that a commit reported
    /// as successful actually *advanced* something.
    ///
    /// **Rejects:** a `commit` that returns `Ok` and makes neither the
    /// read-model row nor the checkpoint durable. That implementation is not a
    /// straw man — PS-1's MUST is a *coupling* rather than a progress
    /// obligation, so "neither" satisfies the clause through its "or not at all"
    /// arm and passes [`commit_is_atomic_with_the_read_model`]
    /// as one of the two states that rule permits. This is the rule that fails
    /// it. Whether the progress obligation joins PS-1 or earns a clause of its
    /// own is open and owned elsewhere
    /// (`.kb/open-questions/ps-1-states-no-progress-obligation.md`); the rule is
    /// written as §4.11's table specifies and settles nothing.
    ///
    /// The store that embodies the defect is `projection-mutant-registry`'s and
    /// is not in the tree yet, which is a knowingly-carried one-story debt with
    /// a named discharger rather than an exemption.
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
    /// named by the specification (§4.11) and lands with
    /// `projection-mutant-registry` (HS-S0008), the story that discharges this
    /// rule's carried debt. Two further stores fail this rule for unrelated
    /// reasons and arrive with it: `TruncatingResetStore` and
    /// `ValidatingCommitStore`.
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
            commit_advances_the_checkpoint,
            commit_is_atomic_with_the_read_model,
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
                $crate::projection::rules::$name(__conformance_fixture)
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
                $crate::block_on($crate::projection::rules::$name(__conformance_fixture))
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
                let __outcome = $crate::projection::rules::$name(__conformance_fixture).await;
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
/// `$crate::projection::rules::$name`, so a name in the enumeration with no rule
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

/// Two `open()` calls make two isolated backing stores, not two handles onto
/// one.
///
/// The property every rule taking `impl AsyncFn() -> F` rests on, checked
/// against the reference fixture. `commit_rejects_a_foreign_batch` will depend
/// on it directly one slice later: a batch begun on one store and committed to
/// another must be rejected, and a fixture handing back one shared store makes
/// that rule unwritable rather than failing.
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
