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

use crate::correct::{Defect, MutantBatch, State};

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
/// (`spec/SPECIFICATION.md:5686-5691`) and belongs to `buffering-conformant-variant`.
/// Keeping this one minimal — one const and one `unimplemented!()` over the
/// correct core — is what stops it being presented as the second batch shape a
/// later story owes. If that variant also declares `false` when it lands, it is a
/// second instance rather than a duplicate to delete.
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
