//! The projection stores that are **legally different** from the reference one.
//!
//! CF-5's half of the registry, and the mirror of `mutants.rs`: a store here has
//! no defect, differs from `MemoryProjectionStore` in a way the specification
//! permits outright, and MUST pass every rule. That claim is worth as much as the
//! mutant one and is checked by the same meta-tests — a rule over-specified
//! beyond what a clause requires shows up here as a conformant store going red,
//! which is the only signal in this binary that points at the *rule* rather than
//! at the store.
//!
//! # Why a variant needs its own file rather than a `Kind` in `mutants.rs`
//!
//! Because the reading is opposite. Every entry in `mutants.rs` is documented as
//! *the mistake somebody would ship*; every entry here is documented as *the
//! design decision somebody would take*, and putting the two under one module
//! doc would make a reader carry the distinction themselves. The event-store
//! family split them for the same reason
//! (`crates/happenstance-testkit/tests/mutation_coverage/variants.rs`).

use crate::buffering::BufferingProjectionFixture;
use crate::correct::{Defect, MutantBatch, State, apply};
use crate::harness::ProjectionSubject;

/// A batch with **no read path at all**.
///
/// PS-12's second arm, which the clause permits in as many words: an adapter may
/// answer a read through an open batch, or it may offer no way to read one. The
/// buffering and write-behind shapes cannot — a batch that is a list of
/// statements queued for a server has nothing to read *from* until it is sent —
/// and this store is the smallest honest instance of that: it declares
/// [`READS_THROUGH_BATCH`](happenstance_core::ProjectionProbe::READS_THROUGH_BATCH)
/// `false` and leaves `probe_read_through` `unimplemented!()`, which is the
/// specification's own spelling for a member only called when the constant says
/// so (`spec/SPECIFICATION.md:5011-5012`).
///
/// # Why it exists, and what would be wrong without it
///
/// Both arms of a gate need a fixture or one arm is code no test executes.
/// `MemoryProjectionStore` and every mutant in this binary declare `true`, so
/// without this store the skip arm of the two gated rules would be dead — and
/// their `require_read_through!` gate would be a claim about a branch nothing had
/// ever taken. **The skip arm must never end up with zero fixtures behind it.**
///
/// # What it is not
///
/// It is **not** the buffering, replay-at-commit conformant variant, which is a
/// different and larger instrument at the far end of §6's batch-shape axis
/// (`spec/SPECIFICATION.md:5686-5691`). That one has since landed as
/// [`BufferingProjectionStore`](crate::buffering::BufferingProjectionStore), and
/// it declares `READS_THROUGH_BATCH` **`true`** — so the two are opposite arms of
/// the same gate rather than duplicates, and neither can be deleted without
/// leaving one arm with no fixture behind it. Keeping this one minimal — one
/// const and one `unimplemented!()` over the correct core — is what stopped it
/// being presented as the second batch shape.
///
/// The pair of overrides *is* the single property. A store that declared `false`
/// and kept a working read path would be lying in the harmless direction, and one
/// that declared `true` over an `unimplemented!()` would panic the first time a
/// rule believed it; neither is a store anybody ships, and neither is what this
/// variant models.
pub(crate) struct NoBatchReadStore;

impl Defect for NoBatchReadStore {
    const NAME: &'static str = "NoBatchReadStore";

    const READS_THROUGH_BATCH: bool = false;

    fn probe_read_through(state: &State, batch: &MutantBatch<Self>, key: &str) -> Option<u64> {
        let _ = (state, batch, key);
        // `unimplemented!`, which is the specification's own word and is
        // enforceable rather than stylistic here: this workspace denies
        // `clippy::todo` and does not lint `unimplemented`. Reaching this line
        // means a rule called a member the constant above says is absent.
        unimplemented!(
            "`NoBatchReadStore` declares `READS_THROUGH_BATCH = false`, so \
             `probe_read_through` is never called: a rule that reached it \
             ignored the gate"
        )
    }
}

/// A `reset` that **removes** its checkpoint row instead of recording an
/// explicit `NeverRun` one.
///
/// `DELETE FROM checkpoints WHERE projection_id = ?`, which is what an adapter
/// author writes first and what
/// [`MemoryProjectionStore`](happenstance_core::MemoryProjectionStore) — the
/// oracle every conformance harness in this workspace runs against — actually
/// does. The correct core in this binary deliberately does the *other* legal
/// thing (`correct.rs`'s [`Defect::reset_writes`], which inserts an explicit
/// `Checkpoint::NeverRun`), so without this row the registry contained no store
/// on the removal side of a seam PS-19 and PS-38 both stand on.
///
/// # Why it is a conformant variant and not a mutant
///
/// Because both answers are legal and the port says so. After the removal,
/// `checkpoint(id)` finds no row and resolves it through
/// [`Defect::missing_checkpoint`] to `Checkpoint::NeverRun` — which is what PS-19
/// requires after a successful `reset` and what PS-38's second sentence requires
/// of an id no commit has named. The store's observable behaviour is identical to
/// the correct core's; what differs is that *"was reset"* and *"was never seen"*
/// become the same state in storage, which is a design decision an adapter takes
/// and not a mistake it makes.
///
/// # What it is here to catch, which is a defect in a **rule**
///
/// This row exists because of a specific failure that happened, not as
/// symmetry. `fresh_projection_has_no_checkpoint` was written, shipped, and then
/// withdrawn for a day when a review found it convicted
/// [`PresumedLiveCheckpointStore`](crate::mutants::PresumedLiveCheckpointStore)
/// of an obligation no clause stated. **CF-5's positive control could not have
/// caught that**, because no conformant variant exercised the missing-row seam at
/// all: every legal store in the registry left the correct core's `reset_writes`
/// and `missing_checkpoint` in place, so a rule over-specified on either was
/// invisible to `projection_conformant_variants_pass_everything`.
///
/// It occupies the legal half of the shape §4.11 named. The instruction this row
/// answers asked for *"the legal `.unwrap_or(Checkpoint::Live { through: FIRST })`
/// store"*, and that shape is **no longer legal**: it was conformant under PS-19
/// alone, and PS-38 — minted after the instruction was written — rejects it, which
/// is why `PresumedLiveCheckpointStore` is a mutant rather than the variant here.
/// What remains legal on that seam is the removal-versus-sentinel choice, and this
/// row is the arm the registry lacked. A future rule that asserts a reset leaves a
/// row *behind*, or that an unseen id is distinguishable from a reset one, goes
/// red here — against a store the specification permits — instead of being caught
/// by a human refusing an acceptance criterion.
pub(crate) struct AbsentAfterResetStore;

impl Defect for AbsentAfterResetStore {
    const NAME: &'static str = "AbsentAfterResetStore";

    fn reset_writes(state: &mut State, batch: &mut MutantBatch<Self>, key: &str) {
        // The caller's own deletes, exactly as the correct core applies them —
        // the port has no idea what the read model is, and this store is not
        // different about that.
        apply(state, batch);
        // The one difference, and it is one line: the row goes rather than being
        // overwritten with a sentinel. `missing_checkpoint` is left alone, so the
        // absence still reads as `NeverRun`.
        state.checkpoints.remove(key);
    }
}

/// The third conformant variant, wired into *this* binary.
///
/// The store itself is `projection_mutation_coverage/buffering.rs`, which is
/// deliberately not this file: it is also included by
/// `tests/projection_conformance_buffering.rs`, and everything else in this
/// module depends on the `Defect` seam that target cannot see. What belongs here
/// is the **registration** — the trait that makes a fixture enumerable by the
/// mutant harness, which is this binary's and only this binary's.
///
/// [`BufferingProjectionStore`](crate::buffering::BufferingProjectionStore) is a
/// standalone store rather than a `Defect` over the correct core for the same
/// reason it is a variant at all: a `Defect` is the correct store with **one
/// step** replaced, and this one differs in what a batch *is*. Expressing that as
/// a step override would need either a seam per method or a store whose "one
/// defect" is the whole of it, and the registry would then be describing the
/// instrument instead of the shape.
///
/// It is [`NoBatchReadStore`]'s opposite in the one property that matters to
/// CF-5's control: it declares `READS_THROUGH_BATCH = true`, so the two rules
/// gated on that switch **execute** against a conformant variant rather than
/// skipping against the only one there was.
impl ProjectionSubject for BufferingProjectionFixture {
    const NAME: &'static str = "BufferingProjectionStore";

    fn open() -> Self {
        Self::new()
    }
}
