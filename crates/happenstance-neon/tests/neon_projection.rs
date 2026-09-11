//! The projection conformance suite, run against a live Neon endpoint.
//!
//! # How this target is gated, and why it is gated *this* way
//!
//! The same argument as `neon_conformance.rs`, which carries it in full: every
//! test here needs `NEON_CONNECTION`, the default gate must not,
//! `required-features` does not work because the gate runs `--all-features`, and
//! an environment-variable early return does not work because "no secret, pass
//! quietly" is indistinguishable in CI output from a rule that passed.
//!
//! The `#![cfg(…)]` above is a different mechanism doing a different job. It is
//! not a gate against the *endpoint*; it is the two features the impl under test
//! lives behind. `projection-store` forwards `happenstance-core`'s own
//! `unstable-projection`, and `conformance` is what compiles
//! `impl ProjectionProbe for NeonProjectionStore`. With either off there is no
//! store to mount a suite over, and the target compiles to nothing rather than
//! failing to compile.
//!
//! # Running it
//!
//! ```console
//! cargo test -p happenstance-neon --all-features --test neon_projection -- --ignored --list
//! cargo test -p happenstance-neon --all-features --test neon_projection -- --ignored --show-output
//! ```
//!
//! # What this adapter reports rather than passes
//!
//! **Three rules, all of them here, and none of them a gap.** This is the whole
//! of the crate's capability skip list.
//!
//! * `refused_reset_changes_nothing`, because `RESET_REFUSAL` is declined — the
//!   store holds no protection policy, and inventing one so that a rule reports
//!   `Ran` would put a domain decision inside an adapter designed to keep the
//!   domain out. The reason is written in this store's own words rather than
//!   copied from the adapter next door, which is CF-18's requirement.
//! * `batch_reads_reflect_pending_writes` and `rebuild_is_chunk_size_invariant`,
//!   because `READS_THROUGH_BATCH` is `false` — a `NeonWriteBatch` has been sent
//!   to the endpoint exactly never, and answering from committed state is what
//!   PS-12 forbids by name. It is **forced** rather than chosen twice over:
//!   `probe_read_through` is synchronous and infallible, and every answer this
//!   adapter could give costs an HTTPS round trip.
//!
//! All three print their reason under `--show-output`. None is omitted from the
//! binary, which is the distinction CF-18 exists to hold.

#![cfg(all(
    feature = "projection-store",
    feature = "conformance",
    not(target_arch = "wasm32")
))]
#![allow(clippy::unwrap_used)]

mod support;

use happenstance_core::{
    Authority, Checkpoint, CommitError, ProjectionId, ProjectionStore, SequencePosition,
};
use happenstance_neon::{SqlRequest, SqlStatement, SqlTransport};
use happenstance_testkit::ProjectionFixture;
use support::NeonProjectionFixture;
use support::transport::HyperTransport;

/// The projection family's emitter, plus `#[ignore]`.
///
/// A local macro rather than a parameter on the testkit's, for the reason
/// `neon_conformance.rs` gives: the emitter is already a parameter of the mount,
/// so adding one attribute needs no change to the testkit at all.
macro_rules! emit_ignored_projection_tokio {
    ($($name:ident),* $(,)?) => {
        $(
            #[tokio::test]
            #[ignore = "needs a live Neon endpoint; set NEON_CONNECTION and run with `-- --ignored`"]
            async fn $name() {
                happenstance_testkit::projection::rules::$name(__conformance_fixture)
                    .await
                    .report(::core::stringify!($name));
            }
        )*
    };
}
pub(crate) use emit_ignored_projection_tokio;

happenstance_testkit::projection_store_conformance!(
    mod_name = projection_conformance,
    emit = crate::emit_ignored_projection_tokio,
    fixture = NeonProjectionFixture::new()
);

// -------------------------------------------------------------------------
// The adapter-private tests the borrowed suite cannot make
// -------------------------------------------------------------------------

/// Migration 2's `CHECK` refuses an authority the contract cannot represent.
///
/// It matters because `authority_to_row` writes `'unknown'` for a
/// `#[non_exhaustive]` variant this adapter has never seen — the arm exists so
/// that such a value fails **where it happens** rather than being stored as
/// `live` and reported later as a live projection that is really a half-finished
/// rebuild.
#[tokio::test]
#[ignore = "needs a live Neon endpoint; set NEON_CONNECTION and run with `-- --ignored`"]
async fn the_authority_check_refuses_a_third_value() {
    let fixture = NeonProjectionFixture::new();
    let _store = fixture.connect().await;

    let response = HyperTransport::shared()
        .round_trip(SqlRequest::single(SqlStatement::new(format!(
            "INSERT INTO {} (projection_id, position, authority) \
             VALUES ('third-value', 1, 'unknown')",
            fixture.config().qualified_checkpoint()
        ))))
        .await
        .expect("a broken test environment: the endpoint did not answer");

    assert!(
        !response.is_success(),
        "migration 2's CHECK accepted an authority `Checkpoint` cannot represent, so a \
         value this adapter writes only for an unrecognised variant would be stored and \
         then read back as something the port can name"
    );
}

/// The checkpoint guard aborts the whole batch, read model included.
///
/// The borrowed rule checks that a regression is refused and that both positions
/// come back. This is the half only this adapter owes: its refusal is a
/// *deliberate cast failure* in one more statement of the same non-interactive
/// batch, and what has to be true is that the caller's own statements — already
/// in that batch — go down with it. A guarded upsert that merely wrote zero rows
/// would pass the port's rule and leave a partially applied read model behind.
#[tokio::test]
#[ignore = "needs a live Neon endpoint; set NEON_CONNECTION and run with `-- --ignored`"]
async fn a_refused_checkpoint_takes_the_read_model_down_with_it() {
    use happenstance_core::ProjectionProbe as _;

    let fixture = NeonProjectionFixture::new();
    let store = fixture.connect().await;
    let id = ProjectionId::new("regression-probe");

    let mut batch = store.begin().await.unwrap();
    store.probe_write(&mut batch, "k", 1).await.unwrap();
    store
        .commit(
            batch,
            &id,
            SequencePosition::new(5).unwrap(),
            Authority::Live,
        )
        .await
        .expect("the first commit should succeed");

    let mut behind = store.begin().await.unwrap();
    store.probe_write(&mut behind, "k", 99).await.unwrap();
    let error = store
        .commit(
            behind,
            &id,
            SequencePosition::new(3).unwrap(),
            Authority::Live,
        )
        .await
        .expect_err("a commit below the recorded checkpoint must be refused");

    match error {
        CommitError::CheckpointRegression { current, attempted } => {
            assert_eq!(current.get(), 5);
            assert_eq!(attempted.get(), 3);
        }
        other => panic!("expected a checkpoint regression, got {other:?}"),
    }

    assert_eq!(
        store.probe_read("k").await.expect("the probe should read"),
        Some(1),
        "the refused commit's read-model write survived, so the abort did not take the \
         whole batch with it — which is PS-1's second conjunct failing in the one place \
         a non-interactive batch could fail it"
    );
    assert_eq!(
        store.checkpoint(&id).await.expect("the checkpoint reads"),
        Checkpoint::Live {
            through: SequencePosition::new(5).unwrap()
        }
    );
}

/// Every capability constant is an answer, and the declined one is this store's.
///
/// Runs with no endpoint, for the reason `neon_conformance.rs` gives:
/// `Capability::declined("")` fires at codegen, which `cargo clippy` never
/// reaches.
#[test]
fn capability_constants_are_answered_not_defaulted() {
    assert!(NeonProjectionFixture::SECOND_HANDLE.is_supported());
    assert!(NeonProjectionFixture::COMMIT_FAULT.is_supported());
    assert!(!NeonProjectionFixture::RESET_REFUSAL.is_supported());

    let reason = NeonProjectionFixture::RESET_REFUSAL
        .reason()
        .expect("a declined capability states a reason");
    assert!(
        reason.contains("NeonProjectionStore"),
        "a declension must be about THIS store. CF-18's finding is a reason inherited \
         from the testkit's own words, and a reason copied from the adapter next door is \
         the same defect with a different source: {reason}"
    );
    assert_ne!(
        Some(reason),
        Silent::RESET_REFUSAL.reason(),
        "RESET_REFUSAL is the testkit's silent default, which says only that the fixture \
         did not answer"
    );
}

/// A fixture that answers nothing, so the defaults can be named.
struct Silent;

impl ProjectionFixture for Silent {
    type Store = happenstance_neon::NeonProjectionStore<HyperTransport>;
    async fn connect(&self) -> Self::Store {
        unreachable!("`Silent` exists to be read, never constructed")
    }
}

/// The skip list is three, and it is asserted rather than described.
///
/// The phase's stated proof artefact is *which* rules this adapter cannot pass
/// and why. A prose list in a module header drifts; this is the same claim as an
/// expression over the constants the rules actually gate on, so a capability that
/// was declared or declined without the header moving fails here.
#[test]
fn the_capability_skip_list_is_exactly_three() {
    use happenstance_core::ProjectionProbe;

    let declined_capabilities = usize::from(!NeonProjectionFixture::RESET_REFUSAL.is_supported());
    let read_through_rules = if <happenstance_neon::NeonProjectionStore<HyperTransport> as ProjectionProbe>::READS_THROUGH_BATCH
    {
        0
    } else {
        // `batch_reads_reflect_pending_writes` and `rebuild_is_chunk_size_invariant`.
        2
    };
    assert_eq!(
        declined_capabilities + read_through_rules,
        3,
        "the skip list moved. It is the phase's proof artefact, so it moves in the \
         module header and in the report at the same time or not at all."
    );
}
