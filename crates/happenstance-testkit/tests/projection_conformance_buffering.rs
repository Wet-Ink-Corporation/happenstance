//! The projection conformance suite, driven against the **second batch shape**.
//!
//! `projection_conformance.rs` runs the same rules against
//! [`MemoryProjectionStore`](happenstance_core::MemoryProjectionStore), whose
//! batch is a materialised delta. This file runs them, unchanged, against
//! [`BufferingProjectionStore`](crate::buffering::BufferingProjectionStore),
//! whose batch is a replayable op journal and which acquires no handle, no
//! transaction and no lock between `begin` and `commit` — PS-4's shape
//! (`spec/SPECIFICATION.md:4849-4856`), and the far end of §6's batch-shape axis
//! (`:5686-5691`).
//!
//! **No rule is added, changed, weakened or gated by this file.** It is one
//! `projection_store_conformance!` invocation and one assertion the suite cannot
//! make about itself. If a rule rejects this store, the finding is about the
//! *rule* (CF-6) and is reported rather than absorbed — the store is legal, and
//! `projection_conformant_variants_pass_everything` says so in its own failure
//! message.
//!
//! What this pair buys an adapter author is the right to read a green projection
//! run as evidence about the **port** rather than about one store: two shapes,
//! same rules, one `cargo xtask ci`. What it does **not** buy is PS-2, which is
//! `[FROZEN]`, asks for two *adapters* at opposite ends of the axis, and names
//! the two-instrument monoculture in its own `Rejects:` clause
//! (`spec/SPECIFICATION.md:4760-4775`).
//!
//! The tokio harness, so: native only, exactly as `projection_conformance.rs`
//! is. The store itself reaches for nothing host-only — `core`, `Rc`, `RefCell`
//! and `BTreeMap` — so the `wasm32` `--tests` check over this crate stays green
//! rather than being made green by this line.

#![cfg(not(target_arch = "wasm32"))]
// Test code, per the house rule and `tests/projection_mutation_coverage.rs:62`'s
// precedent.
#![allow(clippy::unwrap_used)]

use happenstance_core::{
    Authority, Checkpoint, ProjectionId, ProjectionProbe, ProjectionStore, SequencePosition,
};
use happenstance_testkit::ProjectionFixture;

// One definition, two consumers. `#[path]` for the reason
// `projection_mutation_coverage.rs:64-71` records: a test target's root resolves
// `mod foo;` against `tests/`, so without the attribute this would look for
// `tests/buffering.rs` and cargo would compile it as a target of its own. A
// second copy of the store beside this file would be a store the registry does
// not describe.
#[path = "projection_mutation_coverage/buffering.rs"]
mod buffering;

use buffering::BufferingProjectionFixture;

happenstance_testkit::projection_store_conformance!(BufferingProjectionFixture::new());

/// The read model and the checkpoint change at `commit` and at no earlier
/// moment.
///
/// The one assertion a green suite cannot make on its own, and the reason it is
/// here rather than left to the rules: a store that quietly applied on write
/// would still pass every projection rule, because every rule observes the store
/// *after* the operation it is about. A variant that passes everything because
/// it has become a second copy of the reference store is a vacuous green, and
/// this is what rejects it.
///
/// It reads committed state **out of band** — `probe_read` and `checkpoint` go
/// to the store, never through the batch — while a batch is open and carrying
/// three staged ops, and asserts nothing moved. Then it commits and asserts
/// everything did.
///
/// No position is named as a literal anywhere: the checkpoint is compared
/// against the position this test handed the store, which is the only position
/// it is entitled to know (`CLAUDE.md`, *The rule that matters*).
#[tokio::test]
async fn the_read_model_changes_only_at_commit() {
    let fixture = BufferingProjectionFixture::new();
    let writer = fixture.connect().await;
    let id = ProjectionId::new("the_read_model_changes_only_at_commit");

    let anchored = SequencePosition::FIRST;
    let next = anchored.next().expect("a second position exists");

    // The anchor, so that "unchanged" is a *recorded* state rather than the
    // empty one: a store that had committed nothing would preserve nothing
    // anybody could have broken.
    let mut anchor = writer.begin().await.unwrap();
    writer.probe_write(&mut anchor, "anchor", 1).await.unwrap();
    writer
        .commit(anchor, &id, anchored, Authority::Live)
        .await
        .unwrap();

    let observer = fixture.connect().await;
    let rows_before = observer.probe_read("anchor").await.unwrap();
    let checkpoint_before = observer.checkpoint(&id).await.unwrap();
    assert_eq!(
        checkpoint_before,
        Checkpoint::Live { through: anchored },
        "the anchoring commit must have recorded the position it was handed, or \
         the rest of this test is comparing the empty state against itself"
    );

    let mut batch = writer.begin().await.unwrap();
    writer.probe_write(&mut batch, "staged", 7).await.unwrap();
    writer.probe_delete_all(&mut batch).await.unwrap();
    writer.probe_write(&mut batch, "staged", 9).await.unwrap();

    // Three ops queued, including one that would clear the whole read model.
    // Committed state is read through a *fresh* handle, so nothing about this
    // observation depends on the handle that owns the batch.
    let during = fixture.connect().await;
    assert_eq!(
        during.probe_read("anchor").await.unwrap(),
        rows_before,
        "a batch that has not committed must not have touched the read model, \
         and a fresh handle saw the anchor row change while the batch was still \
         open. That is apply-on-write wearing this store's name: it would pass \
         every projection rule, because every rule observes the store after the \
         operation it is about"
    );
    assert_eq!(
        during.probe_read("staged").await.unwrap(),
        None,
        "a row staged in an open batch must not be visible in committed state. \
         This store's batch is a journal queued for a server it holds no \
         connection to; a value readable here was never queued at all"
    );
    assert_eq!(
        during.checkpoint(&id).await.unwrap(),
        checkpoint_before,
        "an open batch must not move the checkpoint. A store that advanced it at \
         `begin` — or at each staged write — reports progress over a read model \
         that has not been written"
    );

    // Read-through is the *other* answer, and asserting it here is what keeps
    // the row above from being satisfied by a store that simply lost the write.
    assert_eq!(
        writer
            .probe_read_through(&mut batch, "staged")
            .await
            .unwrap(),
        Some(9),
        "the journal must answer for its own last staged write, or the assertion \
         above is satisfied by a store that dropped it"
    );

    writer
        .commit(batch, &id, next, Authority::Live)
        .await
        .unwrap();

    let after = fixture.connect().await;
    assert_eq!(
        after.probe_read("staged").await.unwrap(),
        Some(9),
        "`commit` is the moment the journal reaches the read model, and the last \
         write for a key must win — replayed in order rather than collapsed at \
         stage time"
    );
    assert_eq!(
        after.probe_read("anchor").await.unwrap(),
        None,
        "the queued `DeleteAll` must have been replayed **in its place** in the \
         journal: it comes after the anchor was committed and before the second \
         staged write, so the anchor row is gone and the staged row is not. A \
         store that collapsed its batch into a delta plus a flag would have to \
         decide this at stage time"
    );
    assert_eq!(
        after.checkpoint(&id).await.unwrap(),
        Checkpoint::Live { through: next },
        "the checkpoint moves in the same step as the rows, to the position this \
         commit was handed"
    );
}
