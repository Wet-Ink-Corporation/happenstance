//! The wrong projection stores.
//!
//! Every store here is a defect somebody would ship. That is CF-4's bar and it is
//! the whole difference between a mutant and a saboteur: `struct AlwaysWrong`
//! satisfies `every_projection_rule_has_a_mutant` mechanically and proves
//! nothing, because no author would have written it. Each entry names the adapter
//! shape it comes from in its `REGISTRY` row.
//!
//! # How to add one
//!
//! 1. Write the defect here as an `impl Defect` overriding **one** step.
//! 2. Add its fixture type to `for_each_projection_mutant!` in
//!    `tests/projection_mutation_coverage.rs`.
//! 3. Add its `Declared` row to `REGISTRY` in the same file: the exact set of
//!    rules it fails, its provenance, its `FailureMode`, and — wherever the rule
//!    carries more than one assertion — an `expect` pin per rule naming the one
//!    this defect is supposed to trip.
//!
//! Steps 2 and 3 are separate on purpose. The type enumeration and the claim
//! about it are the two lists that can drift, and
//! `projection_mutant_registry_is_exhaustive` exists to hold them together.

use core::cell::Cell;

use happenstance_core::{
    Authority, Checkpoint, CommitError, ProjectionId, ResetError, SequencePosition,
};

use crate::correct::{Defect, MutantBatch, MutantError, State, apply};

/// Commits the checkpoint and discards the write set.
///
/// The store `spec/SPECIFICATION.md:5680-5687` names, and the natural shape for
/// any adapter whose read model lives somewhere other than its checkpoint table:
/// the checkpoint write goes through the adapter's own connection and the read
/// model's writes were handed to something else — a second pool, a queue, a
/// client the adapter forgot to flush. `commit` returns `Ok` because the half it
/// can see succeeded.
///
/// It fails `commit_is_atomic_with_the_read_model` at that rule's
/// both-present-or-both-absent assertion, and it passes
/// `commit_advances_the_checkpoint`, because the checkpoint really did advance.
/// The pair is what makes those two rules differential rather than duplicates.
pub(crate) struct CheckpointOnlyStore;

impl Defect for CheckpointOnlyStore {
    const NAME: &'static str = "CheckpointOnlyStore";

    fn commit_writes(
        state: &mut State,
        batch: &mut MutantBatch<Self>,
        key: &str,
        checkpoint: Checkpoint,
    ) {
        // The whole defect, and it is one line short of the correct step: the
        // checkpoint is recorded and `apply(state, batch)` is not called.
        //
        // **Delete this method and the rule goes green.** That reversal is the
        // demonstration — the difference between "the suite asserts PS-1" and
        // "the suite rejects a store that breaks PS-1" is exactly this body.
        let _ = batch;
        state.checkpoints.insert(key.to_owned(), checkpoint);
    }
}

/// A `commit` that returns `Ok` and makes **neither** write durable.
///
/// The adapter that executes the batch inside a transaction it never commits: the
/// statements go out, the connection is returned to the pool, the driver's
/// implicit rollback discards both halves, and the caller was told `Ok`. Every
/// driver that opens a transaction implicitly has produced this, and it is
/// invisible to any test that reads back through the connection that made the
/// write — which is why every rule in this family reads back through a fresh
/// handle.
///
/// It is the store `spec/SPECIFICATION.md:5661` leaves an em-dash for.
/// `commit_advances_the_checkpoint` is the only rule that rejects it, and that is
/// the interesting part: PS-1's MUST is a **coupling** rather than a progress
/// obligation, so "neither" satisfies the clause through its "or not at all" arm
/// and therefore *passes* `commit_is_atomic_with_the_read_model`. It is the one
/// store that separates the two baseline rules.
///
/// That observation is also the evidence
/// `.kb/open-questions/ps-1-states-no-progress-obligation.md` describes. It is
/// recorded here and in this store's provenance and **nowhere else**: PS-1 is
/// `[FROZEN]`, and whether the progress obligation joins it or earns a clause of
/// its own is `ps-clause-pairing-sweep`'s. Registering the mutant is this binary's
/// job; amending the clause is not.
pub(crate) struct UncommittedTransactionStore;

impl Defect for UncommittedTransactionStore {
    const NAME: &'static str = "UncommittedTransactionStore";

    fn commit_writes(
        state: &mut State,
        batch: &mut MutantBatch<Self>,
        key: &str,
        checkpoint: Checkpoint,
    ) {
        // The work is prepared and then never committed. Spelled as one `let _`
        // over all four arguments rather than an empty body, so that what the
        // correct step consumes is visibly *not* consumed here.
        let _ = (state, batch, key, checkpoint);
    }
}

/// A commit that applies the rows, fails on the checkpoint, and says so.
///
/// The partial apply §4.11 names for `failed_commit_leaves_both_unchanged`, and
/// the same adapter family `CheckpointOnlyStore` comes from with the failure
/// arriving one statement later: the read model is written row by row on one
/// connection, the checkpoint goes last, and when *that* write raises the adapter
/// reports the error honestly. Nothing is dishonest about the `Err` — the caller
/// is told the commit failed, and believes the batch was refused — but the rows
/// are already durable, so the projection has silently applied part of a batch
/// nobody will ever re-apply, and the checkpoint that would have recorded them is
/// not there.
///
/// It is what any adapter that forgot the transaction does the first time a write
/// fails, and it is invisible to every rule that only ever sees a commit succeed:
/// this store passes all seven of those, because its defect is reachable only
/// once a fault has been armed.
///
/// Modelled as the *rows* surviving rather than the checkpoint, because that is
/// the order a real adapter writes in — the read model is the bulk of the work
/// and the checkpoint is the last statement — and because the rule reads the row
/// back first, so the assertion that fires names the half that actually leaked.
pub(crate) struct PartialCommitStore;

impl Defect for PartialCommitStore {
    const NAME: &'static str = "PartialCommitStore";

    fn commit_under_fault(
        state: &mut State,
        batch: &mut MutantBatch<Self>,
        key: &str,
        checkpoint: Checkpoint,
    ) -> CommitError<MutantError> {
        // The whole defect: the rows go down and the checkpoint does not, and
        // the error is returned anyway. **Delete this method and the rule goes
        // green** — the correct step writes neither half.
        let _ = (key, checkpoint);
        apply(state, batch);
        CommitError::Store(MutantError::CommitFault)
    }
}

/// A `rollback` that releases its connection without issuing `ROLLBACK`.
///
/// The adapter whose batch is a live transaction and whose `rollback` clears the
/// Rust-side buffer, drops the guard and hands the connection back — never
/// sending the one statement that matters. `Ok` comes back, the caller believes
/// the work is undone, and the rows are still there. Every driver with implicit
/// transaction handling makes this available, and a test asserting only that
/// `rollback` returned `Ok` certifies it.
///
/// Modelled here as a `rollback` that **applies** the batch, because the correct
/// core buffers and therefore has no earlier moment at which the statements could
/// have gone out. The observable consequence is identical — rows durable after a
/// rollback that reported success — which is the only thing a conformance rule
/// can see.
pub(crate) struct UnrolledBackStore;

impl Defect for UnrolledBackStore {
    const NAME: &'static str = "UnrolledBackStore";

    fn rollback(state: &mut State, batch: &mut MutantBatch<Self>) {
        apply(state, batch);
    }
}

/// A batch whose `Drop` returns its pooled connection to nothing.
///
/// The defect a reviewer's probe actually found
/// (`spec/SPECIFICATION.md:4898-4910`): `begin` checks a connection out of the
/// pool, `commit` and `rollback` both return it, and the path nobody wrote a test
/// for — dropping the batch bare — leaks it. The store answers `Busy` from then
/// on. It is the store that makes PS-7's *second* half enforceable, because it
/// rolls the abandoned write back perfectly well and is still ruined.
pub(crate) struct PooledConnectionStore;

impl Defect for PooledConnectionStore {
    const NAME: &'static str = "PooledConnectionStore";

    fn release_the_connection(connection: &Cell<bool>) {
        // Returned to nothing.
        let _ = connection;
    }
}

/// A batch stamped per *type* rather than per instance.
///
/// The identity is a constant, so every store of this type accepts every other
/// one's batch. It is what an adapter author writes when they read "stamp the
/// batch" as "tag it with which store *kind* made it" — a `const`, a `Default`, a
/// hash of the connection string — and it is indistinguishable from correct in
/// any test that holds one store.
///
/// The corruption it permits is silent and cross-instance: a runner holding two
/// stores commits a batch of one projection's rows into the other's database.
pub(crate) struct TypeStampedBatchStore;

impl Defect for TypeStampedBatchStore {
    const NAME: &'static str = "TypeStampedBatchStore";

    fn mint_stamp() -> u64 {
        // A per-type identity. Note that it compares *equal* across instances,
        // which is the whole defect; the value itself is arbitrary.
        0
    }
}

/// A `commit` that validates `position` against what the batch wrote.
///
/// Named by the specification for this rule
/// (`spec/SPECIFICATION.md:5682-5687`), and the reason it is worth registering is
/// that it is **reasonable**: "advances `id`'s checkpoint to `position`" reads
/// like a claim about applied work, and without PS-21's rule an adapter that
/// enforced it would be exactly as conformant as one that did not. Two stores
/// could disagree and both pass, which is a silent interoperability difference
/// between two backends an application might swap.
///
/// What it costs in the field: a narrow projection — forty matches in
/// thirty-seven thousand events a day — can never move its checkpoint past a
/// range it applied nothing from, so it re-scans that range forever on every
/// restart.
pub(crate) struct ValidatingCommitStore;

impl Defect for ValidatingCommitStore {
    const NAME: &'static str = "ValidatingCommitStore";

    fn validate_position(batch: &MutantBatch<Self>) -> Option<CommitError<MutantError>> {
        if batch.writes.is_empty() {
            return Some(CommitError::Store(MutantError::PositionNotApplied));
        }
        None
    }
}

/// `UPDATE checkpoint SET position = ?`, unconditionally.
///
/// What everyone writes, and it is correct until two runners share an id. Under a
/// redeploy where an old pod has not yet exited, the stale runner drags the
/// checkpoint backwards and every event between the two positions is applied a
/// second time — harmless only for projections that happen to be idempotent,
/// which the port offers as an escape hatch rather than requiring.
///
/// The guard this store is missing is what converts that silent double-apply into
/// a reported `CheckpointRegression`.
pub(crate) struct UnconditionalCheckpointStore;

impl Defect for UnconditionalCheckpointStore {
    const NAME: &'static str = "UnconditionalCheckpointStore";

    fn regression(
        recorded: Checkpoint,
        position: SequencePosition,
    ) -> Option<CommitError<MutantError>> {
        // No comparison at all: the write below will overwrite whatever is there.
        let _ = (recorded, position);
        None
    }
}

/// A single-row checkpoint table.
///
/// What a store that has only ever run one projection will write: one row, one
/// position column, no key. Every projection shares it, so the fastest one drags
/// every other one's checkpoint forward and the slower ones skip every event
/// between the two positions — permanently, and with nothing reported.
///
/// It passes every other rule in this family, which is precisely why PS-23 needs
/// a rule of its own.
pub(crate) struct SingleRowCheckpointStore;

impl Defect for SingleRowCheckpointStore {
    const NAME: &'static str = "SingleRowCheckpointStore";

    fn checkpoint_key(id: &ProjectionId) -> String {
        // The schema has one row and no key column, so the id never reaches it.
        let _ = id;
        "checkpoint".to_owned()
    }
}

/// The runbook procedure: clear the table, then update the checkpoint — two
/// statements, and only the first one ran.
///
/// This is what Norvant's night desk executed at 02:46:31 and what the pod's
/// death at 02:46:33 turned into an outage: the rows went, the checkpoint
/// stayed, the runner restarted, read the old checkpoint, resumed *past* it and
/// applied sixty-one events into an empty table while **reporting healthy**
/// (`spec/E2E-CASES.md:458-481`). Nothing about it looks wrong from inside the
/// process that ran it — the deletes committed, the operator saw no error, and
/// the second statement is the one nobody watches.
///
/// It is the shape any adapter reaches for when `reset` is written as two
/// round trips rather than as one unit of work, and it is the reason PS-16's
/// obligation is a *pairing* rather than two obligations that happen to sit
/// beside each other.
pub(crate) struct TwoStatementResetStore;

impl Defect for TwoStatementResetStore {
    const NAME: &'static str = "TwoStatementResetStore";

    fn reset_writes(state: &mut State, batch: &mut MutantBatch<Self>, key: &str) {
        // The whole defect, and it is one line short of the correct step: the
        // caller's deletes are applied and the checkpoint is left exactly where
        // it was. **Delete this method and the rules go green.**
        let _ = key;
        apply(state, batch);
    }
}

/// `DELETE FROM projection_checkpoints` — no `WHERE`.
///
/// The `SqliteProjectionStore::reset()` §4.11 names: truncating the checkpoint
/// table is one statement, it is obviously correct while a store runs one
/// projection, and it stays obviously correct right up to the day a second
/// projection shares the file. Kestrel Cold Chain's does: `van_stock` is rebuilt
/// several times a day across 138 devices, and `fgas_ledger` is a hash chain a
/// regulator already holds and must never be rebuilt at all
/// (`spec/E2E-CASES.md:482-497`).
///
/// It applies the caller's deletes correctly and returns the projection it was
/// asked about to `NeverRun` correctly, so every rule that holds one projection
/// passes it. What it also does is return every *other* projection in the store
/// to `NeverRun`, which the next runner reads as "never built" and rebuilds from
/// an event log that no longer holds the events the ledger was built from.
pub(crate) struct TruncatingResetStore;

impl Defect for TruncatingResetStore {
    const NAME: &'static str = "TruncatingResetStore";

    fn reset_writes(state: &mut State, batch: &mut MutantBatch<Self>, key: &str) {
        apply(state, batch);
        // The whole defect: the statement has no `WHERE`, so the key it was
        // given never reaches it.
        let _ = key;
        state.checkpoints.clear();
    }
}

/// `reset` implemented as `commit(batch, id, FIRST, Live)`.
///
/// The substitute **all six deployment scenarios reached for and all six got
/// wrong** (`RUNBOOK.md:3904-3907`), written into a store rather than typed at a
/// 03:18 prompt. It compiles, it returns `Ok`, the checkpoint moves and the rows
/// go — so every rule asserting that a reset *changed something* passes it, and
/// so does every operator who checks afterwards that the read model is empty.
///
/// What it costs is event 1, permanently and silently: the checkpoint reads back
/// as a position, a runner resumes **strictly after** the position it reads, and
/// nothing anywhere reports that the first event of the rebuild was never
/// applied (`spec/E2E-CASES.md:437-456`).
pub(crate) struct CommitAtFirstResetStore;

impl Defect for CommitAtFirstResetStore {
    const NAME: &'static str = "CommitAtFirstResetStore";

    fn reset_writes(state: &mut State, batch: &mut MutantBatch<Self>, key: &str) {
        apply(state, batch);
        // The whole defect, and it is the *same* line the correct `commit` step
        // writes — which is the point. `reset` here is a commit at the first
        // position wearing another name.
        state.checkpoints.insert(
            key.to_owned(),
            Checkpoint::Live {
                through: SequencePosition::FIRST,
            },
        );
    }
}

/// A protection policy consulted somewhere other than the write path.
///
/// The adapter whose "protected projections" list is enforced in the admin UI,
/// in a wrapper the application calls, or in a code review convention — anywhere
/// except inside `reset`. Its `reset` does the work and reports success, which
/// is what every runbook, migration script and operator holding the store gets.
///
/// PS-18 exists because a policy with no port-level mechanism is bypassed by
/// anyone holding the store, and this is that store: the policy is real,
/// documented and completely absent from the one code path that could enforce
/// it.
pub(crate) struct RefusalAsSuccessStore;

impl Defect for RefusalAsSuccessStore {
    const NAME: &'static str = "RefusalAsSuccessStore";

    fn refusal(
        state: &mut State,
        batch: &mut MutantBatch<Self>,
        protected: bool,
    ) -> Option<ResetError<MutantError>> {
        // The whole defect: the flag is read and discarded, so the reset below
        // proceeds and answers `Ok`.
        let _ = (state, batch, protected);
        None
    }
}

/// A refusal issued **after** the deletes have gone out.
///
/// The mirror-image defect, and the one a rule that stopped at
/// `matches!(err, ResetError::Refused)` would certify: this store deletes first
/// and checks its policy second, so the error it returns is perfectly honest and
/// the read model it returns it over is already gone. An adapter reaches this
/// shape by putting the policy check at the end of a method that begins with the
/// work, or by enforcing it in a trigger that fires after the statement it was
/// meant to prevent.
///
/// The caller is told the reset was refused, believes the ledger is intact, and
/// finds out otherwise the next time anybody reads it.
pub(crate) struct RefusalAfterTheFactStore;

impl Defect for RefusalAfterTheFactStore {
    const NAME: &'static str = "RefusalAfterTheFactStore";

    fn refusal(
        state: &mut State,
        batch: &mut MutantBatch<Self>,
        protected: bool,
    ) -> Option<ResetError<MutantError>> {
        if protected {
            // The whole defect: the caller's deletes are applied and *then* the
            // refusal is returned. The checkpoint is left alone, which is what
            // makes this the honest-looking half — the store really did decline
            // to move it.
            apply(state, batch);
            return Some(ResetError::Refused);
        }
        None
    }
}

/// A missing checkpoint row resolved as `Live { through: FIRST }`.
///
/// The specification names this shape itself, and names it as the *natural* one
/// rather than a contrivance (`spec/SPECIFICATION.md:5300-5316`): an adapter
/// whose `reset` records an explicit `NeverRun` and whose `checkpoint` resolves a
/// missing row with `.unwrap_or(…)` has to put *something* in the `unwrap_or`,
/// and `Live { through: FIRST }` is what an author writes when the checkpoint
/// column is `NOT NULL DEFAULT 1`.
///
/// It satisfies PS-19's MUST verbatim — after a successful reset the row is
/// there and says `NeverRun` — and tells a runner that a read model nobody has
/// ever built is authoritative and already considered through the first
/// position. Every event at that position is then skipped on the first run of
/// every projection the store has never seen.
///
/// # Why this row was withdrawn for a day, and what brought it back
///
/// It was registered, then **withdrawn on 2026-08-14**, because satisfying PS-19
/// verbatim was the whole of what any clause obliged: convicting this store was
/// convicting it of an obligation no sentence stated, which is CF-6's failure in
/// the direction a mutant registry cannot see. What brought it back is a clause,
/// not a re-reading — ADR-0030 minted PS-38, whose second sentence is *"a
/// `ProjectionId` no successful `commit` has named MUST read as
/// `Checkpoint::NeverRun`"* (`spec/SPECIFICATION.md:5448-5464`). This store
/// answers `Live { through: FIRST }` for exactly such an id, so it is
/// non-conformant against a `MUST` that exists rather than against a rule that
/// reached past one. Its conformant neighbour on the same seam is
/// [`AbsentAfterResetStore`](crate::variants::AbsentAfterResetStore).
pub(crate) struct PresumedLiveCheckpointStore;

impl Defect for PresumedLiveCheckpointStore {
    const NAME: &'static str = "PresumedLiveCheckpointStore";

    fn missing_checkpoint() -> Checkpoint {
        // The whole defect, and it is the `unwrap_or` argument: a row that is
        // not there is read as a projection that has been run.
        Checkpoint::Live {
            through: SequencePosition::FIRST,
        }
    }
}

/// A batch `get` that answers from committed state.
///
/// One round trip on the connection the batch is already holding, which is what
/// an adapter author writes and what is perfectly right for every projection
/// that never reads what it wrote. For one that does — a counter, a running
/// total, any read-modify-write — it silently loses every write the batch has
/// not committed yet, and loses most of them exactly when the chunk is largest.
///
/// The specification names this shape for PS-12's rule, and the reason it is
/// worth registering is that nothing about it looks wrong: the batch really is
/// applied at commit, the value really is in the store afterwards, and the only
/// window in which it is invisible is the one the projection is standing in.
pub(crate) struct CommittedReadBatchStore;

impl Defect for CommittedReadBatchStore {
    const NAME: &'static str = "CommittedReadBatchStore";

    fn probe_read_through(state: &State, batch: &MutantBatch<Self>, key: &str) -> Option<u64> {
        // The whole defect: the pending writes are never consulted.
        let _ = batch;
        state.rows.get(key).copied()
    }
}

/// A batch that stages writes with `entry().or_insert(…)`.
///
/// The natural spelling when the batch is thought of as a **dedup buffer**:
/// every key appears once, the first value staged for it wins, and a second
/// write for the same key inside one batch is discarded as a duplicate. Its
/// reads through the batch are entirely honest — it answers with what it staged
/// — so it passes PS-12's rule and fails only the chunked replay.
///
/// What it costs is that a rebuild's answer depends on how the operator chunked
/// it: at one event per chunk every write is the first for its key and the store
/// is correct, and at any larger size every repeat inside a chunk is thrown away.
pub(crate) struct FirstWriteWinsBatchStore;

impl Defect for FirstWriteWinsBatchStore {
    const NAME: &'static str = "FirstWriteWinsBatchStore";

    fn stage_write(batch: &mut MutantBatch<Self>, key: &str, value: u64) {
        // The whole defect, and it is one call: `or_insert` where the correct
        // step writes `insert`.
        batch.writes.entry(key.to_owned()).or_insert(value);
    }
}

/// A rebuild in place, behind one position field.
///
/// The obvious reading of the port, and the only one `Option<SequencePosition>`
/// could express before [`Checkpoint`] had three variants: `Authority` arrives at
/// `commit` and is dropped, so every checkpoint says the rows are authoritative
/// whatever the commit claimed. A reader deciding whether to trust the rows in
/// front of it is told yes over a read model that is half-built, and the
/// dashboard shows the wrong number with nothing anywhere reporting a problem.
///
/// It preserves `NeverRun` for an id it has never seen — its defect is precisely
/// that `Rebuilding` is unrepresentable, not that a missing row reads wrongly —
/// so it does not also fail the rule that asks about an unseen projection, and
/// its declaration stays true.
pub(crate) struct LiveOnlyCheckpointStore;

impl Defect for LiveOnlyCheckpointStore {
    const NAME: &'static str = "LiveOnlyCheckpointStore";

    fn checkpoint_for(authority: Authority, position: SequencePosition) -> Checkpoint {
        // The whole defect: the claim is read and dropped.
        let _ = authority;
        Checkpoint::Live { through: position }
    }
}
