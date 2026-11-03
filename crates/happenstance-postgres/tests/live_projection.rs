//! The projection conformance suite against the **far end** of PS-2's axis: a
//! Postgres projection store whose batch is a live transaction.
//!
//! Gated exactly as `projection.rs` is, and for the reasons that file carries in
//! full: `#[ignore]` composes with `--all-features`, keeps the target compiled
//! and linted by the default gate, and makes the gated tests appear as `ignored`
//! rather than vanishing from the count.
//!
//! # Running it
//!
//! ```console
//! cargo test -p happenstance-postgres --all-features --test live_projection -- --ignored --list
//! cargo test -p happenstance-postgres --all-features --test live_projection -- --ignored --show-output --test-threads=1
//! ```
//!
//! `--test-threads=1` on the run and not on the listing: each fixture instance
//! opens a pool sized for the concurrency family, the container's
//! `max_connections` is finite, and twenty fixtures opening pools at once is
//! how a rule fails on a pool timeout that says nothing about the adapter.
//! `--list` opens no connection and needs no such flag.
//!
//! # What this adapter reports rather than passes
//!
//! **One** rule is a reported skip — `refused_reset_changes_nothing`, because
//! `RESET_REFUSAL` is declined for the buffered store's reason. Everything else
//! runs, and that is the point of the file: `batch_reads_reflect_pending_writes`
//! and `rebuild_is_chunk_size_invariant`, reported skips on every adapter over a
//! real database until now, execute here for the first time against something
//! that is not an in-process map. The adapter-private test below asserts that
//! on the `RuleOutcome` value rather than on stdout.
//!
//! # The typed runner
//!
//! The last three tests drive this store with `happenstance::run_projection`
//! and a projection whose `async fn apply` awaits `LivePostgresBatch::execute`
//! (ADR-0074, #142): `run_projection_drives_a_row_writing_projection_through_a_live_batch`
//! commits rows and checkpoint together across chunks,
//! `a_server_refused_statement_rolls_the_live_chunk_back_through_the_runner`
//! discards a chunk the server aborted, and
//! `a_commit_fault_takes_the_projections_rows_with_it` rolls back rows already
//! sent when the checkpoint upsert fails. The typed layer is a path-only
//! dev-dependency for them alone.

#![cfg(all(
    feature = "projection-store",
    feature = "conformance",
    not(target_arch = "wasm32")
))]
#![allow(clippy::unwrap_used)]

mod support;

use core::num::NonZeroUsize;

use happenstance::bytes::Bytes;
use happenstance::{
    Chunk, Codec, CodecError, Delivered, DomainEvent, Event, EventStore, EventType, Json,
    MemoryEventStore, Progressed, Projection, ProjectionError, Tags, run_projection,
};
use happenstance_core::{
    Authority, Checkpoint, ProjectionId, ProjectionProbe, ProjectionStore, SequencePosition,
};
use happenstance_postgres::error::PostgresProjectionStoreError;
use happenstance_postgres::live_projection_store::{
    LivePostgresBatch, LivePostgresProjectionStore,
};
use happenstance_postgres::projection_store::PgParam;
use happenstance_postgres::sqlx;
use happenstance_testkit::{ProjectionFixture, RuleOutcome};
use support::LivePostgresProjectionFixture;

/// The projection family's emitter, plus `#[ignore]`. See `projection.rs`.
macro_rules! emit_ignored_projection_tokio {
    ($($name:ident),* $(,)?) => {
        $(
            #[tokio::test]
            #[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container)"]
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
    fixture = LivePostgresProjectionFixture::new()
);

// -------------------------------------------------------------------------
// The adapter-private tests the borrowed suite cannot make
// -------------------------------------------------------------------------

/// PS-12's rule **ran** here, and did not report a skip.
///
/// This is the observation ADR-0060 found the suite unable to make: a
/// live-transaction adapter that can declare `READS_THROUGH_BATCH = true`
/// truthfully, and a read-through rule that executes against it rather than
/// being taken as a reported skip. Asserted on the [`RuleOutcome`] rather than
/// on stdout, because a skip line is a string and `Ran` is a value.
#[tokio::test]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container)"]
async fn the_read_through_rule_runs_rather_than_skipping() {
    const {
        assert!(
            <LivePostgresProjectionFixture as ProjectionFixture>::Store::READS_THROUGH_BATCH,
            "the live store declares the capability its transaction has"
        );
    }

    let outcome =
        happenstance_testkit::projection::rules::batch_reads_reflect_pending_writes(async || {
            LivePostgresProjectionFixture::new()
        })
        .await;
    assert!(
        matches!(outcome, RuleOutcome::Ran),
        "PS-12's rule must execute against a live transaction, not report a skip: {outcome:?}"
    );

    let outcome =
        happenstance_testkit::projection::rules::rebuild_is_chunk_size_invariant(async || {
            LivePostgresProjectionFixture::new()
        })
        .await;
    assert!(
        matches!(outcome, RuleOutcome::Ran),
        "the rebuild rule must execute against a live transaction, not report a skip: {outcome:?}"
    );
}

/// The write is on the server before commit, and it is the server that layers
/// it over committed state.
///
/// A buffering store can pass `batch_reads_reflect_pending_writes` by keeping a
/// shadow map — which its documentation rejects for good reason. This test is
/// the control that the live store does no such thing: the row is observable
/// through the batch's transaction, invisible through a **second** backend
/// session, and gone from both after a rollback.
#[tokio::test]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container)"]
async fn a_pending_write_lives_on_the_server_and_only_in_this_session() {
    let fixture = LivePostgresProjectionFixture::new();
    let writer = fixture.connect().await;
    let observer = fixture.connect().await;

    let mut batch = writer.begin().await.unwrap();
    writer.probe_write(&mut batch, "depot-7", 12).await.unwrap();

    assert_eq!(
        writer
            .probe_read_through(&mut batch, "depot-7")
            .await
            .unwrap(),
        Some(12),
        "the open transaction sees its own statement"
    );
    assert_eq!(
        observer.probe_read("depot-7").await.unwrap(),
        None,
        "a second session sees nothing until commit"
    );

    writer.rollback(batch).await.unwrap();
    assert_eq!(
        writer.probe_read("depot-7").await.unwrap(),
        None,
        "and a rollback undid a statement the server had already received"
    );
}

/// A statement refused mid-batch poisons the transaction, and the seam says so.
///
/// This is what the probe seam's `Result` was added for. The write is refused
/// by the server — a `CHECK` constraint the driver cannot pre-check — and the
/// batch is then unusable: Postgres answers every further statement on it with
/// *"current transaction is aborted"*, which `commit` surfaces as
/// `CommitError::Store` rather than reporting success over a write that never
/// happened.
#[tokio::test]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container)"]
async fn a_refused_statement_poisons_the_batch_and_commit_says_so() {
    let fixture = LivePostgresProjectionFixture::new();
    let pool = fixture.pool_for_test().await;
    let store = fixture.connect().await;
    let id = ProjectionId::from_static("poisoned");

    // A constraint the probe write will violate: `v` must be small.
    sqlx::query("ALTER TABLE projection_probe ADD CONSTRAINT v_is_small CHECK (v < 100)")
        .execute(&pool)
        .await
        .unwrap();

    let mut batch = store.begin().await.unwrap();
    store.probe_write(&mut batch, "ok", 1).await.unwrap();
    store
        .probe_write(&mut batch, "too-big", 1_000)
        .await
        .expect_err("the server refuses the statement, and the seam reports it");

    let refused = store
        .commit(batch, &id, SequencePosition::FIRST, Authority::Live)
        .await
        .expect_err("a poisoned transaction cannot be committed");
    assert!(
        matches!(refused, happenstance_core::CommitError::Store(_)),
        "an aborted transaction is the store's failure, not a regression or a foreign batch: {refused:?}"
    );

    assert_eq!(
        store.probe_read("ok").await.unwrap(),
        None,
        "the statement that succeeded before the poison was rolled back with it"
    );
    assert_eq!(
        store.checkpoint(&id).await.unwrap(),
        Checkpoint::NeverRun,
        "and the checkpoint did not move"
    );

    // The store is still usable afterwards: the poisoned connection went back
    // to the pool rolled back, which is PS-7 for a batch that held a resource.
    let mut fresh = store.begin().await.unwrap();
    store.probe_write(&mut fresh, "after", 2).await.unwrap();
    store
        .commit(fresh, &id, SequencePosition::FIRST, Authority::Live)
        .await
        .unwrap();
    assert_eq!(store.probe_read("after").await.unwrap(), Some(2));
}

/// A surplus value is refused at `execute`, before it reaches the server, and the
/// batch can then no longer commit (ADR-0084).
///
/// The server never saw the refused statement, so it has not aborted the
/// transaction; the adapter must. This rejects a check that refuses the statement
/// locally but leaves the transaction committable — under which `commit` would
/// record the probe write and advance the checkpoint past a statement that never
/// ran.
#[tokio::test]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container)"]
async fn a_surplus_value_is_refused_at_execute_and_the_batch_cannot_commit() {
    let fixture = LivePostgresProjectionFixture::new();
    let store = fixture.connect().await;
    let id = ProjectionId::from_static("arity-live");

    let mut batch = store.begin().await.unwrap();
    store.probe_write(&mut batch, "ok", 1).await.unwrap();

    let refused = batch
        .execute(
            "DELETE FROM projection_probe WHERE k = $1 /* arity-surplus */",
            [PgParam::text("ok"), PgParam::text("unused")],
        )
        .await
        .expect_err("a surplus value must be refused at `execute`");
    assert!(
        matches!(
            refused,
            PostgresProjectionStoreError::ParameterCount {
                statement: 1,
                declared: 1,
                supplied: 2,
                ..
            }
        ),
        "{refused:?}"
    );

    let after = batch
        .execute(
            "DELETE FROM projection_probe WHERE k = $1",
            [PgParam::text("ok")],
        )
        .await
        .expect_err("a batch with a refused statement accepts no more");
    assert!(
        matches!(after, PostgresProjectionStoreError::Poisoned),
        "{after:?}"
    );

    let refused = store
        .commit(batch, &id, SequencePosition::FIRST, Authority::Live)
        .await
        .expect_err("a batch with a refused statement cannot be committed");
    assert!(
        matches!(
            refused,
            happenstance_core::CommitError::Store(PostgresProjectionStoreError::Poisoned)
        ),
        "{refused:?}"
    );

    assert_eq!(
        store.probe_read("ok").await.unwrap(),
        None,
        "the statement issued before the refusal was rolled back with the batch"
    );
    assert_eq!(store.checkpoint(&id).await.unwrap(), Checkpoint::NeverRun);

    // The connection went back to the pool rolled back, so the store is usable.
    let mut fresh = store.begin().await.unwrap();
    store.probe_write(&mut fresh, "after", 2).await.unwrap();
    store
        .commit(fresh, &id, SequencePosition::FIRST, Authority::Live)
        .await
        .unwrap();
    assert_eq!(store.probe_read("after").await.unwrap(), Some(2));
}

/// `reset` refuses a batch poisoned by a local refusal, the same as `commit`,
/// and moves nothing.
///
/// The batch's `probe_delete_all` has already reached the server, so a `reset`
/// that ignored the latch would delete the committed row and the checkpoint.
#[tokio::test]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container)"]
async fn a_poisoned_live_batch_cannot_reset_and_moves_nothing() {
    let fixture = LivePostgresProjectionFixture::new();
    let store = fixture.connect().await;
    let id = ProjectionId::from_static("arity-live-reset");

    let mut setup = store.begin().await.unwrap();
    store.probe_write(&mut setup, "kept", 4_211).await.unwrap();
    store
        .commit(setup, &id, SequencePosition::FIRST, Authority::Live)
        .await
        .unwrap();

    let mut batch = store.begin().await.unwrap();
    store.probe_delete_all(&mut batch).await.unwrap();
    batch
        .execute(
            "DELETE FROM projection_probe WHERE k = $1 /* arity-surplus */",
            [PgParam::text("kept"), PgParam::text("unused")],
        )
        .await
        .expect_err("a surplus value must be refused at `execute`");

    let refused = store
        .reset(batch, &id)
        .await
        .expect_err("a batch with a refused statement cannot be reset");
    assert!(
        matches!(
            refused,
            happenstance_core::ResetError::Store(PostgresProjectionStoreError::Poisoned)
        ),
        "{refused:?}"
    );

    assert_eq!(
        store.probe_read("kept").await.unwrap(),
        Some(4_211),
        "the delete issued before the refusal was rolled back with the batch"
    );
    assert_eq!(
        store.checkpoint(&id).await.unwrap(),
        Checkpoint::Live {
            through: SequencePosition::FIRST
        },
        "and the checkpoint was not deleted"
    );
}

// -------------------------------------------------------------------------
// The typed runner drives it (#142)
// -------------------------------------------------------------------------

const DEPOSITED: EventType = EventType::from_static("Deposited");

/// The read model the projection below writes, one row per account.
const LEDGER_TABLE: &str = "CREATE TABLE ledger (account text PRIMARY KEY, cents bigint NOT NULL)";

/// Every row of the read model, in a stable order.
const LEDGER_ROWS: &str = "SELECT account, cents FROM ledger ORDER BY account";

/// One domain event, encoded through the codec as an `(account, cents)` tuple
/// so this test needs no `serde` derive and no dev-dependency beyond the typed
/// layer itself.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Deposited {
    account: String,
    cents: i64,
}

impl DomainEvent for Deposited {
    const EVENT_TYPES: &'static [EventType] = &[DEPOSITED];

    fn event_type(&self) -> EventType {
        DEPOSITED
    }

    fn tags(&self) -> Tags {
        Tags::empty()
    }

    fn encode<C: Codec>(&self, codec: &C) -> Result<Bytes, CodecError> {
        codec.encode(&(self.account.as_str(), self.cents))
    }

    fn decode<C: Codec>(codec: &C, _: &EventType, data: &Bytes) -> Result<Self, CodecError> {
        let (account, cents): (String, i64) = codec.decode(data)?;
        Ok(Self { account, cents })
    }
}

/// The projection's own refusal (PS-28). One arm, so that `?` on
/// `LivePostgresBatch::execute` inside `apply` is the `From` the typed layer's
/// `type Error` bound promises (ADR-0074 F8).
#[derive(Debug, thiserror::Error)]
enum LedgerError {
    #[error(transparent)]
    Store(#[from] PostgresProjectionStoreError),
}

/// A projection that writes **rows**: every event is an `INSERT … ON CONFLICT`
/// awaited through the live transaction, which a synchronous `apply` could not
/// express at all.
struct Ledger {
    id: ProjectionId,
    scope: Tags,
    /// An account whose deposit is sent to a table the server does not have.
    ///
    /// A knob on this hand-written double rather than a branch in production
    /// code (TST-09): it is how the refusal test gets a statement the *server*
    /// refuses, mid-chunk, after earlier statements already reached it.
    refuse: Option<&'static str>,
}

impl Ledger {
    fn new(refuse: Option<&'static str>) -> Self {
        Self {
            id: ProjectionId::from_static("ledger"),
            scope: Tags::empty(),
            refuse,
        }
    }
}

impl Projection for Ledger {
    type Event = Deposited;
    type Store = LivePostgresProjectionStore;
    type Error = LedgerError;

    fn id(&self) -> &ProjectionId {
        &self.id
    }

    fn scope(&self) -> &Tags {
        &self.scope
    }

    async fn apply(
        &mut self,
        event: Delivered<Deposited>,
        batch: &mut LivePostgresBatch,
    ) -> Result<(), LedgerError> {
        let Deposited { account, cents } = event.into_event();
        let sql = if self.refuse == Some(account.as_str()) {
            "INSERT INTO no_such_table (account, cents) VALUES ($1, $2)"
        } else {
            "INSERT INTO ledger (account, cents) VALUES ($1, $2) \
             ON CONFLICT (account) DO UPDATE SET cents = ledger.cents + excluded.cents"
        };
        // The await a synchronous `apply` could not write, and the `?` that
        // `type Error: From<store error>` turns into a `LedgerError`.
        batch
            .execute(sql, [PgParam::text(account), PgParam::int8(cents)])
            .await?;
        Ok(())
    }
}

/// Appends one `Deposited` per `(account, cents)` pair and returns the
/// positions the store assigned, in order.
///
/// Returned rather than assumed: the specification permits gaps, so a test
/// compares against what the store actually handed out.
async fn deposit(
    events: &MemoryEventStore,
    deposits: &[(&str, i64)],
) -> Result<Vec<SequencePosition>, Box<dyn std::error::Error>> {
    let encoded = deposits
        .iter()
        .map(|&(account, cents)| {
            let data = Deposited {
                account: account.to_owned(),
                cents,
            }
            .encode(&Json)?;
            Ok(Event::new(DEPOSITED, data)?)
        })
        .collect::<Result<Vec<_>, Box<dyn std::error::Error>>>()?;
    events.append(&encoded, None).await?;
    Ok(events
        .snapshot()
        .iter()
        .map(|recorded| recorded.position)
        .collect())
}

/// Creates the read model's table in the fixture's schema.
async fn create_ledger(fixture: &LivePostgresProjectionFixture) -> Result<(), sqlx::Error> {
    sqlx::query(LEDGER_TABLE)
        .execute(&fixture.pool_for_test().await)
        .await?;
    Ok(())
}

/// The read model as a raw `SELECT` on the fixture's pool sees it: committed
/// rows only, read outside any batch.
async fn ledger_rows(
    fixture: &LivePostgresProjectionFixture,
) -> Result<Vec<(String, i64)>, sqlx::Error> {
    sqlx::query_as(LEDGER_ROWS)
        .fetch_all(&fixture.pool_for_test().await)
        .await
}

/// `run_projection` drives a projection whose `apply` awaits a statement into
/// the live transaction, and the rows and the checkpoint land together.
///
/// Five deposits in chunks of two are three commits, and `alice` crosses both
/// chunk boundaries, so a sum that is right says every chunk's statements were
/// committed on the transaction the checkpoint went into. Rejects an adapter
/// whose `commit` commits a different transaction than the statements went
/// into (the rows are absent from the raw read), and a runner that commits
/// under one id and is read under another (`Live` is absent from the second
/// handle).
#[tokio::test]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container)"]
async fn run_projection_drives_a_row_writing_projection_through_a_live_batch() {
    let fixture = LivePostgresProjectionFixture::new();
    create_ledger(&fixture).await.unwrap();
    let events = MemoryEventStore::new();
    let positions = deposit(
        &events,
        &[
            ("alice", 1),
            ("bob", 20),
            ("alice", 300),
            ("bob", 4_000),
            ("alice", 50_000),
        ],
    )
    .await
    .unwrap();
    let last = positions.last().copied();
    let models = fixture.connect().await;
    let mut ledger = Ledger::new(None);

    let progressed = run_projection(&events, &models, &mut ledger, &Json, chunk_of_two())
        .await
        .unwrap();

    assert_eq!(progressed.applied, 5, "every deposit was applied once");
    assert_eq!(
        progressed.through, last,
        "the run reports the last position"
    );
    assert_eq!(
        ledger_rows(&fixture).await.unwrap(),
        vec![("alice".to_owned(), 50_301), ("bob".to_owned(), 4_020)],
        "every chunk's rows are committed, and none twice"
    );
    let second = fixture.connect().await;
    // #107 moves this: the checkpoint is read under `ledger.id()` until
    // `checkpoint_id` lands, and the `Live` assertion fails loudly then.
    assert_eq!(
        second.checkpoint(ledger.id()).await.unwrap(),
        Checkpoint::Live {
            through: last.expect("five deposits were appended")
        },
        "a second handle reads the checkpoint the last chunk committed"
    );
}

/// A statement the server refuses mid-chunk fails the run with the
/// projection's own error, and the whole chunk goes with it.
///
/// The fourth deposit hits a table that does not exist, in the second chunk,
/// after the third deposit's `INSERT` has already reached the server. Rejects a
/// runner that commits a chunk after an `apply` error, and an adapter whose
/// `rollback` leaves an aborted transaction's earlier statements committed.
/// The runner-level twin of
/// `a_refused_statement_poisons_the_batch_and_commit_says_so`.
#[tokio::test]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container)"]
async fn a_server_refused_statement_rolls_the_live_chunk_back_through_the_runner() {
    let fixture = LivePostgresProjectionFixture::new();
    create_ledger(&fixture).await.unwrap();
    let events = MemoryEventStore::new();
    let positions = deposit(
        &events,
        &[
            ("alice", 1),
            ("bob", 20),
            ("alice", 300),
            ("mallory", 4_000),
        ],
    )
    .await
    .unwrap();
    let &[_, second, _, fourth] = positions.as_slice() else {
        panic!("four deposits were appended: {positions:?}");
    };
    let models = fixture.connect().await;
    let mut ledger = Ledger::new(Some("mallory"));

    let refused = run_projection(&events, &models, &mut ledger, &Json, chunk_of_two())
        .await
        .expect_err("the server refused the fourth deposit's statement");

    assert!(
        matches!(
            refused,
            ProjectionError::Apply {
                position,
                source: LedgerError::Store(PostgresProjectionStoreError::Driver(_)),
                rollback: None,
                ..
            } if position == fourth
        ),
        "the projection's own error, at the refused event, with a clean rollback: {refused:?}"
    );
    assert_eq!(
        refused.progress().through,
        Some(second),
        "the first chunk is what the run committed"
    );
    assert_eq!(
        ledger_rows(&fixture).await.unwrap(),
        vec![("alice".to_owned(), 1), ("bob".to_owned(), 20)],
        "the third deposit, applied into the aborted chunk, is not visible"
    );
    assert_eq!(
        fixture
            .connect()
            .await
            .checkpoint(ledger.id())
            .await
            .unwrap(),
        Checkpoint::Live { through: second },
        "the checkpoint sits at the last committed position"
    );
}

/// A commit that fails takes the rows `apply` already sent with it.
///
/// Every `INSERT` of the chunk has reached the server when the armed trigger
/// refuses the checkpoint upsert. Rejects an adapter that commits the
/// read-model statements separately from the checkpoint: PS-1's invariant,
/// exercised by a projection's rows rather than by the probe's.
#[tokio::test]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container)"]
async fn a_commit_fault_takes_the_projections_rows_with_it() {
    let fixture = LivePostgresProjectionFixture::new();
    create_ledger(&fixture).await.unwrap();
    fixture.arm_commit_fault().await;
    let events = MemoryEventStore::new();
    deposit(&events, &[("alice", 1), ("bob", 20), ("alice", 300)])
        .await
        .unwrap();
    let models = fixture.connect().await;
    let mut ledger = Ledger::new(None);

    let refused = run_projection(&events, &models, &mut ledger, &Json, chunk_of_two())
        .await
        .expect_err("the armed trigger refuses the first commit");

    assert!(
        matches!(
            refused,
            ProjectionError::Commit {
                source: happenstance_core::CommitError::Store(
                    PostgresProjectionStoreError::Driver(_)
                ),
                ..
            }
        ),
        "the adapter's own commit failure: {refused:?}"
    );
    assert_eq!(
        refused.progress(),
        Progressed::default(),
        "nothing was committed"
    );
    assert_eq!(
        ledger_rows(&fixture).await.unwrap(),
        Vec::<(String, i64)>::new(),
        "the rows the chunk sent were rolled back with the checkpoint"
    );
    assert_eq!(
        fixture
            .connect()
            .await
            .checkpoint(ledger.id())
            .await
            .unwrap(),
        Checkpoint::NeverRun,
        "and the checkpoint never moved"
    );
}

/// Chunks of two: small enough that five deposits cross two boundaries.
fn chunk_of_two() -> Chunk {
    Chunk::of(NonZeroUsize::MIN.saturating_add(1))
}
