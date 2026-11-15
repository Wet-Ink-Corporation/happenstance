//! The bar, run whole against a real file on disk.
//!
//! An adapter that compiles is not an adapter. This target is where
//! `happenstance-sqlite` stops being an instrument: one `SqliteFixture`, one
//! macro invocation, and every rule in `for_each_event_store_rule!` emitted as
//! its own `#[tokio::test]` driven against SQLite.
//!
//! The fixture itself lives in [`support`] — shared with
//! `tests/concurrency.rs`, and under `tests/support/` rather than at
//! `tests/support.rs` because Cargo compiles every file *directly* under
//! `tests/` as its own binary. Read that module for what the fixture is made
//! of; three of this project's criteria are won or lost in its body rather than
//! in any rule's output.
//!
//! # The model family runs here too, and it is the one that reads generated SQL
//!
//! `event_store_conformance!` is a list of examples somebody wrote down. The
//! model family is not: it generates a `query × from × backwards × limit ×
//! condition × position policy` space and asks whether this adapter's SQL agrees
//! with an independently-written model of the contract on sequences nobody
//! chose. Against a store whose statements were written to satisfy named
//! examples, that is the first instrument in the workspace capable of finding a
//! *generated* disagreement — a page boundary, a chunk merge, a `resume_from`
//! inclusivity — that is right on every example.
//!
//! Both families mount **here**, sharing one [`support::SqliteFixture`]
//! definition, because they answer one question — *has this adapter passed the
//! bar* — and a reader should meet them in one output. The racing family is the
//! one that does not, and `tests/concurrency.rs` says why.
//!
//! ## Why there is no `#![cfg(feature = "proptest")]` here
//!
//! `event_store_model_conformance!` exists only behind the testkit's
//! off-by-default `proptest` feature, and the reference harness
//! `crates/happenstance-testkit/tests/memory_model_conformance.rs:27` gates
//! itself on exactly that. **That form is unavailable here.**
//! `cfg(feature = "…")` names the *current* crate's features, so this crate
//! cannot `cfg` on a dependency's feature; the feature is enabled at the
//! `[dev-dependencies]` site instead and the invocation is gated on
//! `not(target_arch = "wasm32")` alone, which the crate-level attribute below
//! already supplies. `cargo test -p happenstance-sqlite` with no flags is
//! therefore the whole command **for the event store** — both families in this
//! file, with no extra flag to remember — which is what the project's testing
//! brief promises.
//!
//! It is not the whole command for the *crate*, and this sentence used to say it
//! was. `projection-store` left `default` under PS-3's verdict and ADR-0036, and
//! `conformance` was never in it, so `tests/projection.rs` configures out under
//! the flagless command by design and says so in its own header. The projection
//! bar is `cargo test -p happenstance-sqlite --all-features`, which is what the
//! gate runs. Anything in a test target that reaches
//! `happenstance_sqlite::projection_store` therefore owes a `cfg` at its own
//! boundary; `tests/migration.rs`'s
//! `projection_store_open_configures_its_connection` is the one that had to
//! learn this, having been an `error[E0433]` under default features rather than
//! a skipped test.
//!
//! ## Why only the tokio emitter, and why that is a decision rather than an
//! omission
//!
//! The reference harness invokes **both** shipped emitters, as CF-23's
//! demonstration that the wrapper is still a parameter. Copying that is wrong
//! here for a mechanical reason: `emit_model_blocking` drives the rule under
//! the testkit's own `block_on` with **no tokio runtime anywhere**, and this
//! adapter's read stream defers a `spawn_blocking` into `poll_next` — a hop that
//! resolves through the handle captured at construction and, failing that,
//! `Handle::try_current()`. Under the blocking emitter both are absent, so every
//! generated `read` would fail with `SqliteEventStoreError::NoRuntime`: a
//! *runtime* fact, not a conformance defect, arriving as a red rule. CF-23 is
//! satisfied for this family inside the testkit, by the harness that can
//! honestly run both.
//!
//! ## The pass column, and where it is honestly discharged
//!
//! `RUNBOOK.md`'s phase-8 proof artefact asks for *"the phase-3 mutant harness
//! re-run with `SqliteEventStore` in the pass column"*, and the pass-column
//! meta-test is `mutation_coverage::conformant_variants_pass_everything` — whose
//! subjects are `Rc`/`RefCell` primitives built inside the **testkit's own** test
//! binary. Registering this adapter there was considered and **refused**, for two
//! reasons: it would make a published crate dev-depend on its own consumer, and
//! it would drag `rusqlite` and a tokio runtime into a harness deliberately built
//! to need neither.
//!
//! What that meta-test actually asserts is *every registered rule driven against
//! a conformant store*, and that is precisely what this file is — the sequential
//! family, the model family, and `tests/concurrency.rs`'s racing family, all
//! against one conformant `SqliteFixture` on a real file. The refusal is recorded
//! here, where a reviewer holding the runbook meets it, rather than only in a
//! ledger a code reader will never open.
//!
//! The tokio harness, so: native only.

#![cfg(all(feature = "event-store", not(target_arch = "wasm32")))]
#![allow(clippy::unwrap_used)]

mod support;

use support::SqliteFixture;

happenstance_testkit::event_store_conformance!(SqliteFixture::new());

happenstance_testkit::event_store_model_conformance!(SqliteFixture::new());

/// The fixture's raw `DELETE` removes the rows, from the same store, and
/// leaves the allocator where it was (#147).
///
/// The guard rule in the suite checks the removal itself. This adds what only
/// this adapter's schema decides: the `StoreId` survives, and the next append
/// lands above the removed tail because `sqlite_sequence` kept the high-water
/// mark. That last assertion is ES-38's content for this adapter, recorded here
/// as evidence; the rule is `positions_are_not_reused_after_removal`'s (#151).
///
/// Only the trait and the port are used, so `support` grows nothing that would
/// be dead code in `tests/concurrency.rs`, the other target that declares it.
#[tokio::test]
async fn removal_outside_the_port_deletes_the_row_and_keeps_the_allocator() {
    use happenstance_core::{EventStore, Query, ReadOptions, collect};
    use happenstance_testkit::fixtures::{query_tagged, tagged_event};
    use happenstance_testkit::{Fixture, Removed};

    let fixture = SqliteFixture::new();
    let store = fixture.connect().await;
    let mut appended = Vec::new();
    for (event_type, tag) in [("Kept", "kept"), ("Middle", "middle"), ("Tail", "tail")] {
        let position = store
            .append(&[tagged_event(event_type, &[("removal", tag)])], None)
            .await
            .unwrap();
        appended.push(position);
    }
    let [kept, middle, tail] = appended[..] else {
        unreachable!("three appends returned three positions")
    };

    assert_eq!(
        fixture.remove_outside_port(&[middle, tail]).await,
        Removed::Deleted
    );

    let held = collect(store.read(&Query::all(), ReadOptions::new()))
        .await
        .unwrap();
    assert_eq!(
        held.iter().map(|event| event.position).collect::<Vec<_>>(),
        [kept],
        "only the event that was not named is left"
    );
    let survivor = held.first().unwrap();
    let by_tag = collect(store.read(&query_tagged(&[("removal", "tail")]), ReadOptions::new()))
        .await
        .unwrap();
    assert!(
        by_tag.is_empty(),
        "a tag query for a removed event finds nothing, so its tag rows went too"
    );

    let next = store
        .append(&[tagged_event("Next", &[("removal", "next")])], None)
        .await
        .unwrap();
    let after = collect(store.read(&Query::all(), ReadOptions::new()))
        .await
        .unwrap();
    let appended_next = after.iter().find(|event| event.position == next).unwrap();
    assert_eq!(
        appended_next.id.store(),
        survivor.id.store(),
        "the same incarnation: a removal does not re-mint the StoreId"
    );
    assert!(
        next > tail,
        "the next append took {next}, at or below the removed tail at {tail}: \
         the allocator rewound"
    );
}
