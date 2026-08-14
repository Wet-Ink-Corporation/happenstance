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

use happenstance_core::Checkpoint;

use crate::correct::{Defect, MutantBatch, State};

/// Commits the checkpoint and discards the write set.
///
/// The store `spec/SPECIFICATION.md:5678-5685` names, and the natural shape for
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
/// It is the store `spec/SPECIFICATION.md:5659` leaves an em-dash for.
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
