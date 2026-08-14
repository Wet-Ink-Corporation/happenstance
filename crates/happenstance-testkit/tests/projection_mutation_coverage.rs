//! The projection suite's proof that it discriminates: the projection mutant
//! registry and its four meta-tests.
//!
//! Before this file, **no projection rule had been shown to reject anything.**
//! `projection_store_conformance!` ran green against `MemoryProjectionStore`,
//! which is the oracle and is *supposed* to pass — and a rule that asserts what
//! the contract says passes forever, certifies nothing, and is indistinguishable
//! from a good rule until an adapter with the corresponding bug passes it too.
//! [ADR-0010](../../../.kb/decisions/0010-the-suite-must-prove-itself.md) is the
//! decision; `SPECIFICATION.md` §6.1's CF-1 – CF-6 are the clauses, and §4.11
//! says in as many words that the projection family "are mutants in CF-1's sense
//! and take CF-1 through CF-5 unchanged".
//!
//! This is the **second instance** of a mechanism the event-store family already
//! runs, not a parallel invention. `tests/mutation_coverage.rs` is the first, and
//! every field name and every semantic below is that file's — so a reviewer reads
//! one shape across both families rather than two dialects.
//!
//! The four tests live in a `mod projection_mutation_coverage` inside this file so
//! that the printed name and
//! `cargo test projection_mutation_coverage::every_projection_rule_has_a_mutant`
//! both resolve. A test name a reviewer cannot paste into `cargo test` is a test
//! name that drifts.
//!
//! # Never quote a pass rate over this registry
//!
//! It is an author-chosen bug set, so the denominator is a choice: a fraction
//! says how representative the author was and reports it as though it said how
//! good the suite is. Report which defects the set covers and which axes it
//! leaves uncovered — which is what [`REGISTRY`]'s own documentation does, and
//! nothing in this binary computes or prints a fraction over it. (ADR-0010 §1;
//! `SPECIFICATION.md` §6.1's closing note.)
//!
//! # CF-5's half is deferred by name, and deliberately not landed empty
//!
//! [`Kind::ConformantVariant`] exists in the shape and **no row uses it yet**. The
//! projection family's conformant variant is the buffering, replay-at-commit
//! store, and it belongs to `buffering-conformant-variant` (HS-S0014). So neither
//! the event-store family's "at least one conformant variant is registered"
//! assertion nor its `conformant_variants_pass_everything` positive control is
//! landed here: over an empty set, both would pass while asserting nothing, which
//! is the vacuity CF-5 exists to prevent reintroduced one level up. The hole is
//! named rather than filled.
//!
//! # Why the whole file is gated off `wasm32`, and what that costs
//!
//! [`std::panic::catch_unwind`] cannot catch on a target with no unwinder, and a
//! meta-test that cannot observe a panic is the whole mechanism gone. One
//! crate-level attribute rather than one per item; `tests/mutation_coverage.rs`
//! carries the same gate, so this is precedent rather than novelty.
//!
//! **The cost, stated rather than discovered later:** the stores in this binary
//! are never type-checked for `wasm32`. A later signature change that only breaks
//! on that target will break the projection wasm harness
//! (`tests/projection_conformance_wasm.rs`) and not this file, and the two then
//! get fixed in separate sittings. The gate step that finds it is
//! `cargo check -p happenstance-testkit --tests --target wasm32-unknown-unknown`,
//! which is mandatory in `cargo xtask ci` — so an *ungated* binary here would
//! break the workspace's wasm step rather than merely this file's.

#![cfg(not(target_arch = "wasm32"))]
// Test code, per the house rule and `tests/mutation_coverage.rs:48`'s precedent.
#![allow(clippy::unwrap_used)]

// `#[path]` is not decoration. The root of a test target resolves `mod foo;`
// against its own *directory* — `tests/` — not against a
// `tests/projection_mutation_coverage/` subdirectory, because the crate-root rule
// and the `mod.rs` rule are the same rule. Without these attributes `mod
// harness;` looks for `tests/harness.rs`, which would put three support files in
// the same directory as the test targets and invite cargo to compile them as
// targets of their own.
#[path = "projection_mutation_coverage/correct.rs"]
mod correct;
#[path = "projection_mutation_coverage/harness.rs"]
mod harness;
#[path = "projection_mutation_coverage/mutants.rs"]
mod mutants;
#[path = "projection_mutation_coverage/variants.rs"]
mod variants;

use harness::{Origin, RUNTIME_PANICS, SubjectReport, Verdict};

// =====================================================================
// The registry, as data
// =====================================================================

/// What kind of store an entry describes.
///
/// The same two kinds the event-store registry carries, with the same meanings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kind {
    /// A store with a named defect, which MUST fail exactly the rules it
    /// declares.
    Mutant,
    /// A store that is legally different from `MemoryProjectionStore` and MUST
    /// pass everything (CF-5).
    ///
    /// The `#[expect(dead_code)]` this arm carried came off with
    /// `read-through-and-rebuild-rules`: `NoBatchReadStore` is the first row to
    /// use it, which is the day the deferral the attribute named ended. The
    /// buffering replay-at-commit variant is still owed and is a *second*
    /// instance rather than this one arriving late.
    ConformantVariant,
}

/// Why a mutant's declared rule fails.
///
/// **This is not optional bookkeeping.** CF-2's `Rejects:` names the hazard in
/// terms: *a mutant which fails the right rule for the wrong reason (a panic in
/// its constructor, an unrelated regression) reads as proof.* Without this field,
/// a mutant that starts panicking on a `RefCell` borrow during a later refactor
/// keeps reading green — it still panics, the meta-test still counts a failure,
/// and the claim "this rule catches this defect" becomes false while the test
/// stays passing.
///
/// # It is one field per mutant, not per (mutant, rule)
///
/// So a mutant that fails three rules claims the same mode for all three. That is
/// a real limitation, left standing deliberately and for the event-store
/// family's stated reason: no registered mutant needs the split, and a per-rule
/// mode would turn every `fails` entry into a tuple for the sake of a case that
/// does not exist. [`Declared::expect`] *is* keyed by rule, and the difference is
/// not an inconsistency — that signal arrived over there and the shape changed;
/// this one has not arrived here.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FailureMode {
    /// The rule's own assertion fired.
    ///
    /// Checked **positively**, by where the panic was raised: every projection
    /// rule's assertions — and the `commit_ok` / `checkpoint_ok` / `probe_read_ok`
    /// helpers that panic on a rule's behalf — live in
    /// `happenstance-testkit/src/projection.rs`, so a panic from anywhere else is
    /// something other than the rule rejecting the store.
    /// [`RUNTIME_PANICS`] is then a second pass over the message, for the
    /// standard-library panic raised *inside* a rule body, which the location
    /// cannot distinguish.
    Assertion,

    /// The store itself panicked, and the message MUST contain this substring.
    ///
    /// No projection mutant uses this arm yet — every defect registered here is
    /// rejected by an assertion in the rule, which is what a well-shaped rule
    /// does. It is declared now because the arm is half of the positive check:
    /// without it, [`FailureMode`] would be a unit struct and the origin check
    /// would have nothing to be the *mirror* of.
    #[expect(
        dead_code,
        reason = "the arm a store-panicking projection mutant will use; declared now \
                  so `Assertion`'s origin check is one half of a pair rather than an \
                  unconditional assertion, and red the day a row uses it"
    )]
    StorePanic(&'static str),
}

/// One row of the registry: a store, and the exact claim made about it.
///
/// # Why this is a hand-written `const` and not generated
///
/// Two proposals arrive here and both must be refused.
///
/// **Generating the table from observed outcomes** turns the proof artefact into
/// a snapshot test, and a snapshot of a mutant registry asserts nothing at all:
/// every future regression is "expected", which is the exact failure mode the
/// registry exists to prevent.
///
/// **Putting the declaration on the impl as an associated const** lets a reviewer
/// edit the claim in the same keystroke as the bug it describes, and the claim
/// then stops being a check. The distance between the store and the claim about
/// it is the mechanism.
///
/// # Why this type is duplicated from the event-store registry rather than shared
///
/// The choice was left open to the implementer, and it went this way for a
/// mechanical reason rather than a stylistic one: `Declared`, [`Kind`] and
/// [`FailureMode`] are declared *inside* `tests/mutation_coverage.rs`, not in one
/// of its `#[path]` support modules, so sharing them would mean moving them out
/// of that file — and this story's PR boundary forbids re-declaring or re-scoping
/// `Declared` there. Duplicating also keeps the two families' evolution
/// independent, which matters because the meta-tests compare a registry against
/// an *enumeration* and never one family's field semantics against the other's.
///
/// What is **not** open, and is enforced by review rather than by a compiler:
/// the field names and their semantics match, one for one, so a reader of either
/// file recognises the other.
#[derive(Debug)]
struct Declared {
    /// Matches the store's own [`harness::ProjectionSubject::NAME`].
    name: &'static str,
    /// Mutant or conformant variant.
    kind: Kind,
    /// The **exact** set of rules this store fails. Empty iff
    /// [`Kind::ConformantVariant`].
    fails: &'static [&'static str],
    /// CF-4: the real adapter shape or scenario that makes this store plausible.
    /// Never empty.
    provenance: &'static str,
    /// How the declared rules fail. Never consulted for a conformant variant.
    mode: FailureMode,
    /// Per-rule pins: `(rule, substring)` pairs naming the **exact assertion**
    /// this mutant is expected to trip in that rule.
    ///
    /// [`FailureMode`] says the rule rejected the store; it does not say *which*
    /// of the rule's assertions did the rejecting, and a rule may carry more than
    /// one.
    ///
    /// It is keyed by rule **from the first commit**, rather than as one
    /// `Option<&'static str>` per mutant. The event-store family started with one
    /// pin per mutant and paid for it: a mutant acquired a second declared
    /// failure with a different message and the pin came off rather than the
    /// shape changing, which is the wrong way round
    /// (`tests/mutation_coverage.rs:167-185`). Starting keyed costs one tuple per
    /// pin and removes the reason to ever drop one.
    ///
    /// A pin is optional per `(mutant, rule)`: an empty slice pins nothing, and a
    /// rule absent from the slice is unpinned. What is *not* optional is that a
    /// pin naming a rule the mutant does not declare is an error —
    /// `projection_mutant_registry_is_exhaustive` rejects it, because a pin on a
    /// rule that never runs is a claim nothing evaluates.
    expect: &'static [(&'static str, &'static str)],
}

/// Every projection store this binary drives.
///
/// Adding one is three edits: write the defect in
/// `projection_mutation_coverage/mutants.rs`, add its type to
/// [`for_each_projection_mutant!`], add its row here. The enumeration and the
/// claim are deliberately separate — they are the two lists that drift, and
/// `projection_mutant_registry_is_exhaustive` is what holds them together.
///
/// # What this set covers, and what it does not
///
/// **Scope: the projection store port only.** Every store here implements
/// `ProjectionStore` and `ProjectionProbe`, and every rule it is driven through
/// comes from `for_each_projection_store_rule!`. The event-store port's mutants
/// are `tests/mutation_coverage.rs`'s and nothing below says anything about them.
///
/// ADR-0010 requires both halves of that sentence and forbids the third thing
/// anyone would write instead. **Never quote a pass rate over this table.** The
/// denominator is an author's choice, so a fraction says how representative the
/// author was while reading as though it said how good the suite is.
///
/// *Covered:* the three ways a commit's halves come apart — the checkpoint
/// written without the read model, a commit that returns `Ok` having written
/// neither, and a commit that returns `Err` having written one.
///
/// *Not covered, and each of these is a real axis rather than an oversight:*
///
/// * **A conformant variant.** CF-5's projection half is
///   `buffering-conformant-variant` (HS-S0014), and until it lands nothing in
///   this binary proves the harness can report a *pass* it did not have to
///   report. See this file's module documentation for why an empty positive
///   control is worse than none.
/// * **Every rule §4.11 lists that has not landed yet.** Eight of the seventeen
///   are still owed at the time of writing, and CF-1 is what forces a mutant to
///   arrive with each of them rather than after them. The uncovered *axes* are
///   therefore the rules themselves, and they are enumerated in
///   `spec/SPECIFICATION.md:5658-5671` rather than restated here.
/// * **A fixture whose `arm_commit_fault` does nothing.** `PartialCommitStore`
///   is a wrong *store*; the wrong *fixture* — one that declares `COMMIT_FAULT`
///   and arms nothing, so `failed_commit_leaves_both_unchanged` passes over a
///   commit that never failed — is what the event-store family registers as
///   `NoopFaultFixture` and this binary does not. The rule's first assertion
///   rejects it, so the hole is in the *demonstration* rather than in the suite,
///   and it is named here rather than left to be inferred from an absence.
/// * **A store with a medium outside the process.** Every store here is a
///   `BTreeMap` behind an `Rc`, so nothing in this binary can model a defect
///   whose observation needs a real restart, a real connection pool or a real
///   transaction manager. That limitation is the event-store family's too
///   (`tests/mutation_coverage.rs`, "Real faults"), and it is stated so that a
///   green run here is not read as evidence about durability.
const REGISTRY: &[Declared] = &[
    Declared {
        name: "CheckpointOnlyStore",
        kind: Kind::Mutant,
        // Three rules, and two of them arrived when `commit-rollback-and-drop-rules`
        // landed. That growth is the exactness meta-test working rather than a
        // widening: a store that applies no rows fails any rule that reads one
        // back, and the alternative — weakening the new rule so the old
        // declaration survived — is the repair to refuse. Neither of the two is
        // covered by this store alone.
        fails: &[
            "commit_is_atomic_with_the_read_model",
            "dropped_batch_leaves_store_usable",
            "distinct_projections_advance_independently",
        ],
        provenance: "an adapter whose read model does not live in the same store as its \
                     checkpoint — a Redis or search-index projection with its checkpoint \
                     in Postgres, or a batch handed to a client the adapter forgot to \
                     flush. The checkpoint write goes through the connection `commit` can \
                     see and succeeds, so `Ok` is returned honestly. Named by the \
                     specification itself (§4.11) as one of the three stores the \
                     projection suite owes",
        mode: FailureMode::Assertion,
        expect: &[
            (
                "commit_is_atomic_with_the_read_model",
                "must become durable together or not at all",
            ),
            (
                "dropped_batch_leaves_store_usable",
                "must leave the store **usable**",
            ),
            (
                "distinct_projections_advance_independently",
                "may disturb the other's rows, and the row the first commit wrote",
            ),
        ],
    },
    Declared {
        name: "UncommittedTransactionStore",
        kind: Kind::Mutant,
        // Five of the eight, and that is inflation rather than vacuity: a store
        // that makes nothing durable trips every rule whose *anchor* is a
        // committed state, at that anchor rather than at the property the rule is
        // named for. The bar the event-store registry sets for tolerating it is
        // met — no rule here is covered by this store alone except
        // `commit_advances_the_checkpoint`, which is the one it was written for
        // and the one §4.11's table leaves an em-dash against.
        fails: &[
            "commit_advances_the_checkpoint",
            "dropped_batch_leaves_store_usable",
            "commit_accepts_a_position_the_batch_did_not_write",
            "commit_rejects_a_regressing_position",
            "distinct_projections_advance_independently",
            // The sixth arrived with `reset-rules`, at the same anchor and for
            // the same reason: the substitute's `commit(empty, id, FIRST)` makes
            // nothing durable, so the checkpoint that was supposed to be
            // *distinguishable* from a reset one reads `NeverRun` as well. The
            // repair to refuse would be weakening the new rule so this
            // declaration survived.
            "reset_is_not_commit_at_first",
            // The seventh, from `read-through-and-rebuild-rules`, at the same
            // anchor again: a store that commits nothing has no checkpoint for a
            // reader to recognise a rebuild in, so the first `Rebuilding`
            // assertion sees `NeverRun`.
            "rebuilding_is_distinguishable_from_live",
        ],
        provenance: "an adapter whose `commit` executes the batch inside a transaction it \
                     never commits — the statements go out, the connection returns to the \
                     pool, the driver's implicit rollback discards both halves and the \
                     caller was told `Ok`. It is the store §4.11's table leaves an \
                     em-dash for, and the one that separates the two baseline rules: PS-1's \
                     MUST is a coupling rather than a progress obligation, so \"neither \
                     write durable\" satisfies its \"or not at all\" arm and passes \
                     `commit_is_atomic_with_the_read_model`. That is also the shape \
                     `.kb/open-questions/ps-1-states-no-progress-obligation.md` describes; \
                     recording the observation here is the whole of what this row does \
                     about it, because PS-1 is [FROZEN] and its repair is \
                     `ps-clause-pairing-sweep`'s",
        mode: FailureMode::Assertion,
        expect: &[
            (
                "commit_advances_the_checkpoint",
                "must move this projection's checkpoint to the position it was given",
            ),
            (
                "dropped_batch_leaves_store_usable",
                "must leave the store **usable**",
            ),
            (
                "commit_accepts_a_position_the_batch_did_not_write",
                "high-water mark of *consideration*, not of application",
            ),
            (
                "commit_rejects_a_regressing_position",
                "must be refused as `CommitError::CheckpointRegression`",
            ),
            (
                "distinct_projections_advance_independently",
                "one commit advances exactly one projection",
            ),
            (
                "reset_is_not_commit_at_first",
                "must be **distinguishable by variant**",
            ),
            (
                "rebuilding_is_distinguishable_from_live",
                "must leave a checkpoint a reader can recognise as a rebuild",
            ),
        ],
    },
    Declared {
        name: "PartialCommitStore",
        kind: Kind::Mutant,
        // Exactly one, and by construction rather than by luck: its defect lives
        // in `commit_under_fault`, which the correct core reaches only after
        // `arm_commit_fault` has been called — and the only rule that arms one is
        // the rule this store is written for. Every other rule drives the correct
        // core unchanged.
        fails: &["failed_commit_leaves_both_unchanged"],
        provenance: "an adapter that writes its read-model rows one statement at a time and \
                     writes the checkpoint last, with no transaction around the pair. When \
                     the checkpoint write raises — a constraint, a lost connection, a \
                     deadlock victim — it reports the failure honestly, and the rows it \
                     already wrote stay. It is `CheckpointOnlyStore`'s adapter family with \
                     the failure one statement later, and it is what every adapter that \
                     forgot the transaction does the first time a write fails. §4.11 names \
                     the shape for this rule: \"a partial apply that reports failure\"",
        mode: FailureMode::Assertion,
        // Pinned at the read-model half specifically. The rule carries three
        // assertions — the fixture's armed fault must fire, the read model must
        // be unchanged, the checkpoint must be — and this store is supposed to
        // trip the middle one. Were it ever to start failing at the first, the
        // row would be certifying that the *fixture* is broken while reading as
        // proof that the rule catches a partial apply.
        expect: &[(
            "failed_commit_leaves_both_unchanged",
            "a commit that reported failure must leave the read model as it was",
        )],
    },
    Declared {
        name: "UnrolledBackStore",
        kind: Kind::Mutant,
        fails: &["rollback_leaves_both_unchanged"],
        provenance: "an adapter whose batch is a live transaction and whose `rollback` \
                     clears its own statement buffer, drops the guard and hands the \
                     connection back — without ever sending `ROLLBACK`. Every driver with \
                     implicit transaction handling makes it available, `Ok` comes back, \
                     and a test asserting only that `rollback` returned `Ok` certifies it",
        mode: FailureMode::Assertion,
        expect: &[(
            "rollback_leaves_both_unchanged",
            "must leave the read model as it was",
        )],
    },
    Declared {
        name: "PooledConnectionStore",
        kind: Kind::Mutant,
        fails: &["dropped_batch_leaves_store_usable"],
        provenance: "an adapter whose `begin` checks a connection out of a pool and whose \
                     batch `Drop` returns it to nothing. `commit` and `rollback` both \
                     return it, so only the path nobody writes a test for leaks — and the \
                     store answers `Busy` for ever after. This is not hypothetical: a \
                     reviewer's probe found exactly this store, which is why PS-7 carries \
                     a second half at all (`spec/SPECIFICATION.md:4898-4910`)",
        mode: FailureMode::Assertion,
        expect: &[("dropped_batch_leaves_store_usable", "commit should succeed")],
    },
    Declared {
        name: "TypeStampedBatchStore",
        kind: Kind::Mutant,
        // Two rules, one defect, and the second one is `reset`'s half of the
        // first: PS-15's stamp is checked by `commit` *and* by `reset`, and
        // `reset_clears_rows_and_checkpoint_together` reaches PS-16's
        // failure-injecting half through exactly that refusal — a batch begun on
        // another store instance is the one `reset` failure a caller can produce
        // without a fixture that can arm a fault. A store that accepts every
        // instance's batch answers `Ok` there and clears a read model whose
        // owner was never asked.
        fails: &[
            "commit_rejects_a_foreign_batch",
            "reset_clears_rows_and_checkpoint_together",
        ],
        provenance: "an adapter that reads \"stamp the batch\" as \"tag it with which \
                     store *kind* made it\" — a `const`, a `Default`, a hash of the \
                     connection string — so every instance accepts every other instance's \
                     batch. §4.11's Rejects column calls this \"every adapter writable \
                     today\", and it is indistinguishable from correct in any test that \
                     holds one store: what it permits is a runner with two stores \
                     committing one projection's rows into the other's database",
        mode: FailureMode::Assertion,
        expect: &[
            (
                "commit_rejects_a_foreign_batch",
                "must be rejected as `CommitError::ForeignBatch`",
            ),
            (
                "reset_clears_rows_and_checkpoint_together",
                "must be refused as `ResetError::ForeignBatch`",
            ),
        ],
    },
    Declared {
        name: "ValidatingCommitStore",
        kind: Kind::Mutant,
        // The second arrived with `reset-rules` and is the same defect seen from
        // the other side: the operator's substitute *is*
        // `commit(empty_batch, id, FIRST)`, so a store that refuses a commit
        // whose batch applied nothing refuses the substitute itself. Modelling
        // the substitute with a non-empty batch would have kept this
        // declaration at one rule and stopped modelling what six deployment
        // scenarios actually typed.
        fails: &[
            "commit_accepts_a_position_the_batch_did_not_write",
            "reset_is_not_commit_at_first",
        ],
        provenance: "an adapter that validates `position` against what the batch wrote — \
                     named by the specification itself for this rule \
                     (`spec/SPECIFICATION.md:5680-5685`). The point of registering it is \
                     that the misreading is *reasonable*: \"advances `id`'s checkpoint to \
                     `position`\" reads like a claim about applied work, and without \
                     PS-21's rule this store would be exactly as conformant as the oracle. \
                     What it costs is a narrow projection re-scanning the same range for \
                     ever on every restart",
        mode: FailureMode::Assertion,
        expect: &[
            (
                "commit_accepts_a_position_the_batch_did_not_write",
                "commit should succeed",
            ),
            ("reset_is_not_commit_at_first", "commit should succeed"),
        ],
    },
    Declared {
        name: "UnconditionalCheckpointStore",
        kind: Kind::Mutant,
        fails: &["commit_rejects_a_regressing_position"],
        provenance: "the adapter that issues `UPDATE checkpoint SET position = ?` \
                     unconditionally, which is what everyone writes and which is correct \
                     until two runners share an id. Under a redeploy where an old pod has \
                     not yet exited, the stale runner drags the checkpoint backwards and \
                     every event between the two positions is applied twice — harmless \
                     only for projections that happen to be idempotent, which this port \
                     offers as an escape hatch rather than requiring",
        mode: FailureMode::Assertion,
        expect: &[(
            "commit_rejects_a_regressing_position",
            "must be refused as `CommitError::CheckpointRegression`",
        )],
    },
    Declared {
        name: "SingleRowCheckpointStore",
        kind: Kind::Mutant,
        // Two rules, and the second arrived with `reset-rules` for the reason
        // the first one is here: one checkpoint row shared by every projection
        // is a *scoping* defect, and `reset` is the second operation that is
        // scoped to one `(store, ProjectionId)` pair. Resetting one projection
        // returns the shared row to `NeverRun` and every other projection in the
        // store reads as never built. Neither rule is covered by this store
        // alone.
        fails: &[
            "distinct_projections_advance_independently",
            "reset_is_scoped_to_one_projection",
        ],
        provenance: "a checkpoint table with one row, one position column and no key — \
                     what a store that has only ever run one projection will write. Every \
                     projection shares the row, so the fastest one drags the others \
                     forward and the slower ones skip every event between the two \
                     positions, permanently and with nothing reported. §4.11 names it, and \
                     its own Rejects note observes that it passes every other rule",
        mode: FailureMode::Assertion,
        expect: &[
            (
                "distinct_projections_advance_independently",
                "one commit advances exactly one projection",
            ),
            (
                "reset_is_scoped_to_one_projection",
                "resetting one projection moved another one's checkpoint",
            ),
        ],
    },
    Declared {
        name: "TwoStatementResetStore",
        kind: Kind::Mutant,
        // Three rules from one overridden step, and the inflation is the defect
        // being *visible* rather than the store being broken in three ways: a
        // `reset` that never touches the checkpoint leaves it `Live`, and the
        // three rules that look at a checkpoint after a reset all see it. It is
        // the only store covering `reset_clears_rows_and_checkpoint_together`'s
        // checkpoint half, which is the half PS-16 is about.
        fails: &[
            "reset_clears_rows_and_checkpoint_together",
            "reset_is_scoped_to_one_projection",
            "reset_is_not_commit_at_first",
        ],
        provenance: "the runbook procedure: `DELETE FROM read_model` on one connection and \
                     `UPDATE checkpoints SET …` on another, which is what Norvant's night desk \
                     executed. The truncate committed at 02:46:31 and the pod died at 02:46:33, \
                     so the second statement never ran; the runner restarted, read the old \
                     checkpoint, resumed past it, applied sixty-one events into an empty table \
                     and reported healthy (`spec/E2E-CASES.md:458-481`). §4.11 names it for this \
                     rule",
        mode: FailureMode::Assertion,
        expect: &[
            (
                "reset_clears_rows_and_checkpoint_together",
                "must return this projection's checkpoint to",
            ),
            (
                "reset_is_scoped_to_one_projection",
                "own checkpoint must have returned to",
            ),
            (
                "reset_is_not_commit_at_first",
                "must be **distinguishable by variant**",
            ),
        ],
    },
    Declared {
        name: "TruncatingResetStore",
        kind: Kind::Mutant,
        // Exactly one, and it is the rule that needs a sibling id to be
        // failable at all: this store resets the projection it was asked about
        // perfectly, so every rule holding one projection passes it.
        fails: &["reset_is_scoped_to_one_projection"],
        provenance: "a `SqliteProjectionStore::reset()` that truncates the checkpoint table — \
                     one statement, no `WHERE`, and obviously correct until a second projection \
                     shares the file. §4.11 names it. What it destroys in the field is the \
                     append-only ledger sharing that file: Kestrel Cold Chain rebuilds \
                     `van_stock` several times a day across 138 devices, and `fgas_ledger` is a \
                     hash chain a regulator already holds and must never be rebuilt \
                     (`spec/E2E-CASES.md:482-497`)",
        mode: FailureMode::Assertion,
        expect: &[(
            "reset_is_scoped_to_one_projection",
            "resetting one projection moved another one's checkpoint",
        )],
    },
    Declared {
        name: "CommitAtFirstResetStore",
        kind: Kind::Mutant,
        // Three rules, for `TwoStatementResetStore`'s reason and with the same
        // reading: a `reset` that leaves a *position* behind is seen by every
        // rule that looks at a checkpoint after a reset. What separates the two
        // stores is not which rules they fail but what they model — one is the
        // statement that never ran, the other is the statement someone wrote on
        // purpose.
        fails: &[
            "reset_clears_rows_and_checkpoint_together",
            "reset_is_scoped_to_one_projection",
            "reset_is_not_commit_at_first",
        ],
        provenance: "`reset` implemented as `commit(batch, id, FIRST, Live)` — the substitute \
                     **all six deployment scenarios reached for and all six got wrong** \
                     (`RUNBOOK.md:3904-3907`), and the one an operator types at 03:18 because it \
                     is the only thing the port used to offer. It compiles, it returns `Ok`, the \
                     rows go and the checkpoint moves, so everything anybody checks afterwards \
                     looks right. Event 1 is then skipped permanently and silently, because a \
                     runner resumes strictly after the position it reads \
                     (`spec/E2E-CASES.md:437-456`)",
        mode: FailureMode::Assertion,
        expect: &[
            (
                "reset_clears_rows_and_checkpoint_together",
                "must return this projection's checkpoint to",
            ),
            (
                "reset_is_scoped_to_one_projection",
                "own checkpoint must have returned to",
            ),
            (
                "reset_is_not_commit_at_first",
                "must be **distinguishable by variant**",
            ),
        ],
    },
    Declared {
        name: "RefusalAsSuccessStore",
        kind: Kind::Mutant,
        fails: &["refused_reset_changes_nothing"],
        provenance: "an adapter whose protected-projections policy is enforced anywhere except \
                     inside `reset` — in the admin UI, in an application-side wrapper, in a code \
                     review convention. The policy is real and documented, and every runbook, \
                     migration script and operator holding the store goes straight past it. \
                     PS-18 exists for exactly this: the port supplies the mechanism, because a \
                     policy with no port-level mechanism is bypassed by anyone holding the store",
        mode: FailureMode::Assertion,
        // Pinned at the first of the rule's three assertions: this store is
        // supposed to be caught by *answering `Ok`*, and if it ever started
        // failing at one of the state assertions instead, the row would be
        // certifying something its provenance does not describe.
        expect: &[(
            "refused_reset_changes_nothing",
            "must answer `Err(ResetError::Refused)`",
        )],
    },
    Declared {
        name: "RefusalAfterTheFactStore",
        kind: Kind::Mutant,
        fails: &["refused_reset_changes_nothing"],
        provenance: "an adapter that issues the caller's deletes and checks its protection \
                     policy afterwards — the policy check at the end of a method that begins \
                     with the work, or a trigger that fires after the statement it was meant to \
                     prevent. The `Err` it returns is honest, the caller believes the ledger is \
                     intact, and it is not. It is the mirror-image defect a rule stopping at \
                     `matches!(err, ResetError::Refused)` would certify, which is why PS-18's \
                     operative half is *changes nothing*",
        mode: FailureMode::Assertion,
        // Pinned at the read-model half specifically, because this store's whole
        // claim is that the error variant is right and the state is not.
        expect: &[(
            "refused_reset_changes_nothing",
            "must leave the read model exactly as it was",
        )],
    },
    Declared {
        name: "PresumedLiveCheckpointStore",
        kind: Kind::Mutant,
        // Two rules from one `unwrap_or` argument, and the second is not
        // inflation: `commit_rejects_a_foreign_batch` asserts that a rejected
        // commit left both stores at `NeverRun`, which is a question about an id
        // neither store has ever seen. A store that answers `Live` there answers
        // `Live` there. Narrowing the defect further would mean inventing a
        // store that resolves a missing row differently depending on who asks,
        // which is not an adapter anybody writes.
        fails: &[
            "fresh_projection_has_no_checkpoint",
            "commit_rejects_a_foreign_batch",
        ],
        provenance: "an adapter whose `checkpoint` resolves a missing row with \
                     `.unwrap_or(Checkpoint::Live { through: FIRST })`, which is what an author \
                     writes when the position column is `NOT NULL DEFAULT 1`. The specification \
                     names this shape itself and names it as the natural one rather than a \
                     contrivance (`spec/SPECIFICATION.md:5232-5243`): paired with a `reset` that \
                     records an explicit `NeverRun`, it satisfies PS-19's MUST verbatim and \
                     still tells a runner that a read model nobody has ever built is \
                     authoritative",
        mode: FailureMode::Assertion,
        expect: &[
            (
                "fresh_projection_has_no_checkpoint",
                "a projection this store has never seen must read back as",
            ),
            ("commit_rejects_a_foreign_batch", "moved its checkpoint"),
        ],
    },
    Declared {
        name: "CommittedReadBatchStore",
        kind: Kind::Mutant,
        // Two rules, and both are the same defect seen at two magnifications:
        // one write lost inside one batch, and a whole rebuild whose answer
        // depends on the chunk size. Neither rule is covered by this store alone.
        fails: &[
            "batch_reads_reflect_pending_writes",
            "rebuild_is_chunk_size_invariant",
        ],
        provenance: "a batch `get` implemented as one round trip on the connection the batch is \
                     already holding — which is what an adapter author writes, and which is right \
                     for every projection that never reads what it wrote. §4.11 names the shape \
                     for PS-12's rule. For a read-modify-write projection it loses every write \
                     the batch has not committed yet, silently, and loses most of them exactly \
                     when the chunk is largest",
        mode: FailureMode::Assertion,
        expect: &[
            (
                "batch_reads_reflect_pending_writes",
                "must let an open batch see",
            ),
            (
                "rebuild_is_chunk_size_invariant",
                "must produce the same read model however it is",
            ),
        ],
    },
    Declared {
        name: "FirstWriteWinsBatchStore",
        kind: Kind::Mutant,
        // Exactly one, and it is the store that makes the chunk rule worth
        // having: its reads through the batch are honest, so PS-12's rule passes
        // it and only the chunked replay can see the defect.
        fails: &["rebuild_is_chunk_size_invariant"],
        provenance: "a batch that stages writes with `entry().or_insert(…)`, keeping the first \
                     value staged for a key rather than the last — the natural spelling when the \
                     batch is thought of as a dedup buffer. It reads back through the batch \
                     honestly, so nothing about it looks wrong until a rebuild run at one event \
                     per chunk and the same rebuild run whole produce different read models",
        mode: FailureMode::Assertion,
        expect: &[(
            "rebuild_is_chunk_size_invariant",
            "must produce the same read model however it is",
        )],
    },
    Declared {
        name: "LiveOnlyCheckpointStore",
        kind: Kind::Mutant,
        fails: &["rebuilding_is_distinguishable_from_live"],
        provenance: "a rebuild in place behind a single position field: `Authority` arrives at \
                     `commit` and is dropped, so every checkpoint claims the rows are \
                     authoritative. It is the obvious reading of the port and the only one \
                     `Option<SequencePosition>` could express before `Checkpoint` had three \
                     variants (`spec/SPECIFICATION.md:5344-5352`). A reader asking whether the \
                     rows in front of it can be trusted is told yes over a half-built read model. \
                     It preserves `NeverRun` for an id it has never seen, because its defect is \
                     that `Rebuilding` is unrepresentable rather than that a missing row reads \
                     wrongly",
        mode: FailureMode::Assertion,
        expect: &[(
            "rebuilding_is_distinguishable_from_live",
            "must leave a checkpoint a reader can recognise as a rebuild",
        )],
    },
    Declared {
        name: "NoBatchReadStore",
        kind: Kind::ConformantVariant,
        // CF-5's first projection row, and `fails` is empty because the store is
        // *correct*: PS-12 permits a batch with no read path in as many words.
        // The two rules gated on that switch report a skip against it, which
        // `projection_conformant_variants_pass_everything` accepts only because
        // the store itself declares the switch `false`.
        fails: &[],
        provenance: "an adapter that buffers its writes and offers no read path on the open batch \
                     — the write-behind and queued-statement shapes, whose batch has nothing to \
                     read *from* until it is sent. PS-12 permits it outright as the second arm of \
                     its MUST, so declining honestly is a conformant answer rather than a \
                     failure. It exists because both arms of a gate need a fixture or one arm is \
                     code no test executes: every other store in this binary, and the reference \
                     store, declare `READS_THROUGH_BATCH = true`",
        mode: FailureMode::Assertion,
        expect: &[],
    },
];

/// Hands every registered projection mutant type to `$callback`.
///
/// The list of **types**, which is a different list from [`REGISTRY`]'s list of
/// **claims** — see that constant for why the two are kept apart.
macro_rules! for_each_projection_mutant {
    ($($callback:tt)+) => {
        $($callback)+! {
            crate::correct::MutantFixture<crate::mutants::CheckpointOnlyStore>,
            crate::correct::MutantFixture<crate::mutants::UncommittedTransactionStore>,
            crate::correct::MutantFixture<crate::mutants::PartialCommitStore>,

            crate::correct::MutantFixture<crate::mutants::UnrolledBackStore>,
            crate::correct::MutantFixture<crate::mutants::PooledConnectionStore>,
            crate::correct::MutantFixture<crate::mutants::TypeStampedBatchStore>,
            crate::correct::MutantFixture<crate::mutants::ValidatingCommitStore>,
            crate::correct::MutantFixture<crate::mutants::UnconditionalCheckpointStore>,
            crate::correct::MutantFixture<crate::mutants::SingleRowCheckpointStore>,

            crate::correct::MutantFixture<crate::mutants::TwoStatementResetStore>,
            crate::correct::MutantFixture<crate::mutants::TruncatingResetStore>,
            crate::correct::MutantFixture<crate::mutants::CommitAtFirstResetStore>,
            crate::correct::MutantFixture<crate::mutants::RefusalAsSuccessStore>,
            crate::correct::MutantFixture<crate::mutants::RefusalAfterTheFactStore>,
            crate::correct::MutantFixture<crate::mutants::PresumedLiveCheckpointStore>,

            crate::correct::MutantFixture<crate::mutants::CommittedReadBatchStore>,
            crate::correct::MutantFixture<crate::mutants::FirstWriteWinsBatchStore>,
            crate::correct::MutantFixture<crate::mutants::LiveOnlyCheckpointStore>,

            crate::correct::MutantFixture<crate::variants::NoBatchReadStore>,
        }
    };
}

/// Drives every registered store through every registered projection rule.
fn reports() -> Vec<SubjectReport> {
    macro_rules! run_each {
        ($($subject:ty),* $(,)?) => {
            ::std::vec![ $( crate::harness::run_subject::<$subject>() ),* ]
        };
    }

    for_each_projection_mutant!(run_each)
}

/// The `NAME` of every registered store, from the store types themselves.
fn registered_names() -> Vec<&'static str> {
    macro_rules! names {
        ($($subject:ty),* $(,)?) => {
            ::std::vec![ $( <$subject as crate::harness::ProjectionSubject>::NAME ),* ]
        };
    }

    for_each_projection_mutant!(names)
}

/// Every projection rule name, from the family's own single enumeration.
///
/// This is the universe every membership check in this file is resolved against.
/// There is no second list of rule names anywhere here, on purpose: a hand-kept
/// list would make `every_projection_rule_has_a_mutant` go green on the day a
/// rule is added, which is the day it must go red.
fn all_projection_rules() -> Vec<&'static str> {
    happenstance_testkit::for_each_projection_store_rule!(happenstance_testkit::__emit_rule_names)
        .to_vec()
}

/// The registry row for `name`, if there is one.
fn declared(name: &str) -> Option<&'static Declared> {
    REGISTRY.iter().find(|entry| entry.name == name)
}

/// Drives the one store whose batch offers no read path.
///
/// Named here rather than spelled inside the test that uses it, because the type
/// is a `MutantFixture<…>` three segments deep and the assertion it feeds is
/// about the *outcome*, not about how the subject is spelled.
fn run_no_batch_read_store() -> SubjectReport {
    harness::run_subject::<correct::MutantFixture<variants::NoBatchReadStore>>()
}

// =====================================================================
// The meta-tests
// =====================================================================

/// CF-1 – CF-4, for the projection family.
///
/// Wrapped in a module whose name matches this binary's, so that
/// `cargo test projection_mutation_coverage::every_projection_rule_has_a_mutant`
/// resolves.
mod projection_mutation_coverage {
    use super::{
        Declared, FailureMode, Kind, Origin, REGISTRY, RUNTIME_PANICS, Verdict,
        all_projection_rules, declared, registered_names, reports, run_no_batch_read_store,
    };

    /// Every rule the registry claims a mutant for.
    fn covered() -> Vec<&'static str> {
        REGISTRY
            .iter()
            .filter(|entry| entry.kind == Kind::Mutant)
            .flat_map(|entry| entry.fails.iter().copied())
            .collect()
    }

    /// CF-1. Every projection rule is paired with at least one mutant that fails
    /// it.
    ///
    /// This is what makes a decorative rule *unwriteable* rather than merely
    /// discouraged: adding a rule to `for_each_projection_store_rule!` fails this
    /// test until its wrong implementation is named. There is **no exemption
    /// list and there must never be one** — a tracker with entries is CF-1 being
    /// *observed* rather than enforced, and the whole argument of ADR-0010 is
    /// that observation does not scale.
    ///
    /// The coupling this creates is deliberate and is the point:
    /// `commit-rollback-and-drop-rules`, `reset-rules` and
    /// `read-through-and-rebuild-rules` each land rules, and none of them can
    /// land one without a store that fails it.
    ///
    /// If a rule ever turns up that is clearly right and has no plausible failing
    /// implementation, ADR-0010 is explicit about the honest response: record
    /// that in the clause and **retire the rule**, rather than inventing a
    /// saboteur to satisfy this test — which
    /// [`every_projection_mutant_states_its_provenance`] would reject anyway.
    ///
    /// It names **every** uncovered rule rather than the first, because a
    /// first-failure-only message makes the second gap invisible until the first
    /// is closed.
    #[test]
    fn every_projection_rule_has_a_mutant() {
        let rules = all_projection_rules();
        let covered = covered();

        let decorative: Vec<_> = rules
            .iter()
            .filter(|name| !covered.contains(*name))
            .collect();
        assert!(
            decorative.is_empty(),
            "these projection rules have no mutant: no store in this binary can \
             fail them, so they certify nothing. Write the wrong implementation \
             into `tests/projection_mutation_coverage/mutants.rs`, enumerate it in \
             `for_each_projection_mutant!` and declare it in `REGISTRY` \
             (ADR-0010 §1): {decorative:?}"
        );
    }

    /// CF-2. The registry and the store enumeration agree, and every name in it
    /// is real.
    ///
    /// Six assertions, and each of them is inherited from a defect the
    /// event-store family actually hit rather than derived from first
    /// principles — re-deriving that list reliably produces four of the six.
    #[test]
    fn projection_mutant_registry_is_exhaustive() {
        let rules = all_projection_rules();
        let registered = registered_names();

        for name in &registered {
            assert!(
                declared(name).is_some(),
                "`{name}` is enumerated in `for_each_projection_mutant!` but has \
                 no `REGISTRY` row, so it is driven and nothing is claimed about it"
            );
        }

        for entry in REGISTRY {
            assert!(
                registered.contains(&entry.name),
                "`{}` has a `REGISTRY` row but is absent from \
                 `for_each_projection_mutant!`, so its claim is never checked \
                 against a running store",
                entry.name
            );
        }

        let mut seen: Vec<&str> = REGISTRY.iter().map(|entry| entry.name).collect();
        let total = seen.len();
        seen.sort_unstable();
        seen.dedup();
        assert_eq!(
            seen.len(),
            total,
            "two `REGISTRY` rows share a name, so one of them is unreachable by \
             every lookup in this file"
        );

        // The same check on the other list. Both sides can duplicate and the two
        // duplications fail differently: a repeated `REGISTRY` row is shadowed by
        // `declared`'s `find`, while a store type repeated in
        // `for_each_projection_mutant!` is *driven twice* and silently doubles its
        // contribution to every aggregate built from `reports()`.
        let mut driven = registered.clone();
        let driven_total = driven.len();
        driven.sort_unstable();
        driven.dedup();
        assert_eq!(
            driven.len(),
            driven_total,
            "a store type appears twice in `for_each_projection_mutant!`, so it is \
             driven through every rule twice and counted twice"
        );

        for entry in REGISTRY {
            // The typo catcher, and it earns its own assertion. A misspelled rule
            // name in a `fails` list is invisible to the other meta-tests:
            // `every_projection_rule_has_a_mutant` only fires if the *real* rule
            // has no other mutant, and
            // `projection_mutants_fail_exactly_their_declared_rules` would report
            // it as "declares a rule it does not fail" — true, and pointing at the
            // wrong thing.
            for rule in entry.fails {
                assert!(
                    rules.contains(rule),
                    "`{}` declares `{rule}`, which is not a projection rule. Check \
                     the spelling against `for_each_projection_store_rule!`",
                    entry.name
                );
            }

            // A pin on a rule the mutant does not declare never executes: the pin
            // check runs inside `assert_declared_failure`, which is only reached
            // for a declared rule. An unreachable pin reads as coverage and is
            // not, which is the same vacuity `expect` exists to prevent.
            for (pinned, _) in entry.expect {
                assert!(
                    entry.fails.contains(pinned),
                    "`{}` pins an assertion for `{pinned}`, which it does not \
                     declare as a failure — so the pin is never evaluated. Either \
                     add the rule to `fails` or drop the pin",
                    entry.name
                );
            }

            match entry.kind {
                Kind::Mutant => assert!(
                    !entry.fails.is_empty(),
                    "`{}` is a mutant that fails nothing, which is a conformant \
                     store filed under the wrong kind",
                    entry.name
                ),
                Kind::ConformantVariant => assert!(
                    entry.fails.is_empty(),
                    "`{}` is a conformant variant that declares failures; a variant \
                     that fails a rule is either a mutant or evidence the rule is \
                     over-specified (CF-6)",
                    entry.name
                ),
            }
        }
    }

    /// CF-3. Both directions: a mutant fails every rule it declares, and every
    /// rule it does not declare either **passes** or **skips for a reason the
    /// fixture stated in advance**.
    ///
    /// The second half is the load-bearing one. A mutant that fails everything
    /// proves nothing about the rule it was written for; it proves the store is
    /// broken. Exactness is what makes the registry a map from rules to the bugs
    /// they catch.
    ///
    /// # Why the second half is not simply "passes"
    ///
    /// Because a skip is neither a pass nor a failure, and the event-store family
    /// measured what happens when it is read as one: a rule that never executed a
    /// line counted as evidence the mutant was correct there. The fix is not to
    /// demand a pass, it is to make the hole a **checked** consequence of
    /// something the fixture declared. A skip must carry a `(capability, reason)`
    /// pair the fixture itself declines; a skip arriving from anywhere else means
    /// a rule stopped running and nothing noticed. Between that and
    /// [`assert_declared_failure`]'s treatment of the declared side — where a skip
    /// is a hole in the map and never a pass — a `require!` gate added to a
    /// projection rule by accident has no green path through this test.
    #[test]
    fn projection_mutants_fail_exactly_their_declared_rules() {
        for report in reports() {
            let Some(entry) = declared(report.name) else {
                continue; // `projection_mutant_registry_is_exhaustive` owns this.
            };
            if entry.kind != Kind::Mutant {
                continue;
            }

            for (rule, verdict) in &report.outcomes {
                if entry.fails.contains(rule) {
                    assert_declared_failure(entry, rule, verdict);
                } else {
                    assert_undeclared_outcome(entry, &report.declines, rule, verdict);
                }
            }
        }
    }

    /// The undeclared half of CF-3: a pass, or a skip the fixture accounted for.
    fn assert_undeclared_outcome(
        entry: &Declared,
        declines: &[(&'static str, &'static str)],
        rule: &str,
        verdict: &Verdict,
    ) {
        match verdict {
            Verdict::Passed => {}
            Verdict::Skipped { capability, reason } => assert!(
                declines.contains(&(*capability, *reason)),
                "`{}` skipped `{rule}` citing `{capability}`, which its fixture \
                 does not decline — so this rule stopped running for a reason \
                 nothing in the registry accounts for, and the cell that reads \
                 \"passes every rule it does not declare\" is empty. Declines: \
                 {declines:?}",
                entry.name
            ),
            Verdict::Panicked { .. } => panic!(
                "`{}` failed `{rule}`, which it does not declare. Either the mutant \
                 is broken in more ways than it claims — the commonest way a mutant \
                 set decays — or the declaration is short a line. Saw: {}",
                entry.name,
                verdict.describe()
            ),
        }
    }

    /// The declared-failure half of CF-3, including CF-2's *wrong reason* hazard.
    fn assert_declared_failure(entry: &Declared, rule: &str, verdict: &Verdict) {
        let Verdict::Panicked { message, origin } = verdict else {
            // Three outcomes, three readings, and conflating the last two is
            // exactly what breaks CF-3. A rule that *passed* means the defect is
            // invisible to it and the declaration is wrong. A rule that *skipped*
            // never executed a line, so it is neither a pass nor a failure — it is
            // a hole in the map.
            panic!(
                "`{}` declares that it fails `{rule}`, but the rule {}. {}",
                entry.name,
                verdict.describe(),
                match verdict {
                    Verdict::Skipped { .. } =>
                        "A declared failure that never ran is a hole in the map, not \
                         a pass: either the mutant's fixture must supply the \
                         capability, or the declaration belongs on a different rule",
                    _ =>
                        "Either the rule does not catch this defect after all — which \
                         makes it decorative for this mutant — or the declaration \
                         names the wrong rule",
                }
            );
        };

        match entry.mode {
            FailureMode::Assertion => {
                // The positive half, and the one that carries the claim. Every
                // projection rule assertion is raised in
                // `happenstance-testkit/src/projection.rs`; anything raised
                // elsewhere is the store, the fixture contract, or the testkit
                // falling over, and none of those is the rule rejecting this
                // defect. A substring denylist over the message cannot make this
                // distinction: the event-store family measured a mutant whose
                // modelled defect had been *deleted* still reading as proof,
                // because the panic that replaced it came from a provided body
                // whose text was on no denylist.
                assert!(
                    origin
                        .as_ref()
                        .is_some_and(Origin::is_a_projection_rule_body),
                    "`{}` is declared to fail `{rule}` by the rule's own assertion, \
                     but the panic was not raised in the projection rules module. A \
                     mutant that fails the right rule for the wrong reason reads as \
                     proof and is not one — and a mutant whose modelled defect has \
                     been deleted fails exactly this way. Saw: {}",
                    entry.name,
                    verdict.describe()
                );

                let runtime = RUNTIME_PANICS
                    .iter()
                    .find(|needle| message.contains(**needle));
                assert!(
                    runtime.is_none(),
                    "`{}` is declared to fail `{rule}` by the rule's own assertion, \
                     but the rule fell over instead ({:?} in the message, raised at \
                     its own line). Message: {message}",
                    entry.name,
                    runtime.copied().unwrap_or_default()
                );
            }
            FailureMode::StorePanic(expected) => {
                // The mirror of the `Assertion` arm's positive check. Without it
                // `StorePanic` is a bare substring test: any panic anywhere whose
                // message contains the needle certifies the row — including one of
                // the rules module's own assertions, which is precisely the
                // substitution CF-2 forbids.
                assert!(
                    origin
                        .as_ref()
                        .is_none_or(|at| !at.is_a_projection_rule_body()),
                    "`{}` is declared to fail `{rule}` by the store panicking, but \
                     the panic was raised in the projection rules module — so a rule \
                     assertion is standing in for the store falling over. Saw: {}",
                    entry.name,
                    verdict.describe()
                );

                assert!(
                    message.contains(expected),
                    "`{}` is declared to fail `{rule}` by panicking with \
                     {expected:?}, but the message was: {message}",
                    entry.name
                );
            }
        }

        // Applies under both modes: `mode` says *how* the rule rejected the store,
        // `expect` says *which* of the rule's assertions did it.
        if let Some((_, expected)) = entry.expect.iter().find(|(pinned, _)| *pinned == rule) {
            assert!(
                message.contains(expected),
                "`{}` is declared to fail `{rule}` at {expected:?}, but a different \
                 assertion fired. The mutant still fails the rule, so nothing above \
                 catches this — and what its provenance claims it demonstrates is no \
                 longer what it demonstrates. Message: {message}",
                entry.name
            );
        }
    }

    /// CF-5. A store that is legally different from the reference one **passes
    /// everything**.
    ///
    /// The positive control, and it is not the mirror of
    /// [`projection_mutants_fail_exactly_their_declared_rules`] — it is the only
    /// assertion in this binary that can point at a **rule** rather than at a
    /// store. A rule over-specified beyond what its clause requires fails a
    /// conformant variant, and every other test here would read that as the
    /// store being wrong.
    ///
    /// It landed with `read-through-and-rebuild-rules`, which is the story that
    /// gave [`Kind::ConformantVariant`] its first row. Before that the set was
    /// empty and this test would have passed while asserting nothing, which is
    /// the vacuity CF-5 exists to prevent arriving one level up; that is why the
    /// hole was named rather than filled with an empty control.
    ///
    /// # A skip is accounted for, never waved through
    ///
    /// `NoBatchReadStore` declares `READS_THROUGH_BATCH = false`, so the two
    /// rules gated on it report a skip rather than running. That is the outcome
    /// PS-12's second arm requires, and it is accepted here **only** because the
    /// subject itself declares the switch — the `(capability, reason)` pair has
    /// to be one `harness::declines` collected from the store. A skip from
    /// anywhere else means a rule stopped running and nothing noticed.
    #[test]
    fn projection_conformant_variants_pass_everything() {
        let mut variants = 0_usize;

        for report in reports() {
            let Some(entry) = declared(report.name) else {
                continue; // `projection_mutant_registry_is_exhaustive` owns this.
            };
            if entry.kind != Kind::ConformantVariant {
                continue;
            }
            variants += 1;

            for (rule, verdict) in &report.outcomes {
                match verdict {
                    Verdict::Passed => {}
                    Verdict::Skipped { capability, reason } => assert!(
                        report.declines.contains(&(*capability, *reason)),
                        "`{}` is a conformant variant and skipped `{rule}` citing \
                         `{capability}`, which it does not declare — so a rule \
                         stopped running for a reason nothing accounts for. \
                         Declares: {:?}",
                        entry.name,
                        report.declines
                    ),
                    Verdict::Panicked { .. } => panic!(
                        "`{}` is registered as a conformant variant and failed \
                         `{rule}`. Either it is not conformant — in which case it \
                         is a mutant and its row is wrong — or the rule is \
                         over-specified beyond what its clause requires, which is \
                         the defect no other test in this binary can see. Saw: {}",
                        entry.name,
                        verdict.describe()
                    ),
                }
            }
        }

        assert!(
            variants > 0,
            "no conformant variant was driven, so this test passed over an empty \
             set and asserted nothing — which is the vacuity CF-5 exists to \
             prevent, one level up. At least one `Kind::ConformantVariant` row \
             must be registered and enumerated"
        );
    }

    /// A store with no read path on its batch is told so **by name**, in the
    /// vocabulary every other declension uses.
    ///
    /// CF-18's second projection instance, and the first one whose switch is not
    /// a fixture `Capability`: `ProjectionProbe::READS_THROUGH_BATCH` lives on
    /// the store's probe impl. The skip is still the same
    /// [`RuleOutcome::Skipped`] rendered by the same `skip_line`, and this test
    /// asserts **both** of its fields — the capability names the constant an
    /// adapter author can go and change, path and all, and the reason is the
    /// testkit's own const rather than a literal repeated here.
    ///
    /// Asserted on the **value**, never on stdout, for the reason
    /// `RuleOutcome::report`'s own documentation gives: libtest suppresses a
    /// passing test's output without `--show-output`, and `println!` writes
    /// nowhere at all on `wasm32-unknown-unknown`.
    #[test]
    fn a_batch_with_no_read_path_is_reported_as_a_skip() {
        let report = run_no_batch_read_store();

        let skipped: Vec<&str> = report
            .outcomes
            .iter()
            .filter(|(_, verdict)| matches!(verdict, Verdict::Skipped { .. }))
            .map(|(rule, _)| *rule)
            .collect();
        assert_eq!(
            skipped,
            vec![
                "batch_reads_reflect_pending_writes",
                "rebuild_is_chunk_size_invariant",
            ],
            "exactly the two rules that read through an open batch may skip \
             against a store declaring `READS_THROUGH_BATCH = false`, in \
             enumeration order — a rule that joined this set lost its way to a \
             read path, and one that left it stopped being gated. Saw {skipped:?}"
        );

        let expected = Verdict::Skipped {
            capability: happenstance_testkit::NO_BATCH_READ_PATH,
            reason: happenstance_testkit::NO_BATCH_READ_PATH_REASON,
        }
        .describe();
        for (rule, verdict) in &report.outcomes {
            if !skipped.contains(rule) {
                continue;
            }
            assert_eq!(
                verdict.describe(),
                expected,
                "`{rule}`'s skip must name `ProjectionProbe::READS_THROUGH_BATCH` \
                 — the constant an adapter author can actually change, and not a \
                 fixture const that does not exist — and carry the testkit's own \
                 stated reason, compared against the exported constants rather \
                 than against literals repeated here"
            );
        }
    }

    /// CF-4. Every registered store names the real shape that makes it plausible.
    ///
    /// This is what rejects the saboteur — `struct AlwaysWrong` — which satisfies
    /// CF-1 mechanically and proves nothing, because no author would have written
    /// it. The mutant that earns its place is the one someone would ship.
    ///
    /// Conformant variants are held to it too, with no `Kind` exemption: a variant
    /// owes an account of *why it is legally different*, or it is just a second
    /// copy of the reference store.
    #[test]
    fn every_projection_mutant_states_its_provenance() {
        for entry in REGISTRY {
            assert!(
                !entry.provenance.trim().is_empty(),
                "`{}` states no provenance. Name the adapter shape or the scenario \
                 that makes it plausible; where the provenance is \"the \
                 specification names this store\", say so",
                entry.name
            );
        }
    }
}
