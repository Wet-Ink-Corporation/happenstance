//! The projection conformance suite, run against a real SQLite file.
//!
//! This target is the mount point for a suite this crate did not write:
//! `happenstance_testkit::projection_store_conformance!` expands to one
//! `#[tokio::test]` per rule, and every one of them drives
//! [`SqliteProjectionStore`] through the port and the probe rather than through
//! anything this crate could have arranged in its own favour.
//!
//! # Why the whole file is behind two features
//!
//! `conformance` is what compiles `impl ProjectionProbe for
//! SqliteProjectionStore` in `src/`, and [`ProjectionFixture::Store`] is bound on
//! that trait — so without it this file does not compile, and with a bare
//! `cargo test -p happenstance-sqlite` it must therefore not exist. The gate runs
//! `--all-features` (`xtask/src/affected.rs`, `xtask/src/main.rs`), which is what
//! makes the target actually run rather than quietly configure out.
//!
//! # What the fixture is made of, which is where two criteria are won or lost
//!
//! The rules cannot see either of these, and a wrong fixture passes every one of
//! them:
//!
//! * **One instance is one fresh temporary file**, so two instances share
//!   nothing. `commit_rejects_a_foreign_batch` wants two *isolated stores*, which
//!   is two `open()` calls on the fixture factory — not two handles.
//! * **Each `connect()` opens a real [`rusqlite::Connection`]** through
//!   `SqliteProjectionStore::open`, never an `Arc` clone of one in-process store.
//!   `MemoryProjectionFixture` is the shape to read and precisely the shape not
//!   to copy here (`crates/happenstance-testkit/src/fixtures.rs`), and
//!   `SqliteProjectionStore::open_in_memory` is the constructor that looks right
//!   and is not: a private in-memory database is per-*connection*, so the second
//!   `connect()` would open a second, empty database and every rule that reads
//!   back through a fresh handle would fail.
//!
//! # The four targeted tests beside the suite
//!
//! The borrowed suite cannot see an adapter-private failure mode, so four of them
//! are asserted here directly: a corrupt stored position, a corrupt stored
//! authority, a cloned store accepting its origin's batch, and two concurrent
//! opens of one path. They are **not** additions to the conformance suite, which
//! this project may not touch — they are this crate's own tests, in this crate's
//! own target.

#![cfg(all(feature = "projection-store", feature = "conformance"))]
// The house style's test-module exception: a fixture that cannot connect is a
// broken test environment, and `expect` says so at the point it happens.
#![allow(clippy::unwrap_used)]

use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};

use happenstance_core::{
    Authority, Checkpoint, CommitError, ProjectionId, ProjectionProbe, ProjectionStore, ResetError,
    SequencePosition,
};
use happenstance_sqlite::projection_store::{
    SCHEMA_VERSION, SqliteProjectionStore, SqliteProjectionStoreError,
};
use happenstance_testkit::{Capability, ProjectionFixture};

/// One fixture instance is **one temporary SQLite file**; each `connect` is one
/// more real connection onto it; two instances share nothing.
#[derive(Debug)]
struct SqliteProjectionFixture {
    path: PathBuf,
    /// The handles this fixture has handed out, kept so that [`Drop`] can close
    /// them before the file is removed.
    ///
    /// A handle owns an `Arc` onto one `rusqlite::Connection`, so a clone parked
    /// here keeps that connection open after the rule has dropped its own — which
    /// is what makes the cleanup below able to unlink the file on a platform that
    /// refuses to unlink an open one.
    handles: Mutex<Vec<SqliteProjectionStore>>,
}

impl SqliteProjectionFixture {
    /// A fixture over a fresh temporary file that nothing else is using.
    ///
    /// A process-local ordinal plus the process id, the shape
    /// `tests/support/mod.rs` already uses: no new dependency, and no two
    /// instances in a run can collide on a path.
    fn new() -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(1);
        let ordinal = NEXT.fetch_add(1, Ordering::Relaxed);
        let mut path = std::env::temp_dir();
        path.push(format!(
            "happenstance-sqlite-projection-{}-{ordinal}.db",
            std::process::id()
        ));
        // A leftover from a previous run would make "a fresh, isolated backing
        // store" a lie, which is the one promise the fixture contract makes.
        remove_database(&path);
        Self {
            path,
            handles: Mutex::new(Vec::new()),
        }
    }

    /// The file this fixture's stores are opened against.
    fn path(&self) -> &Path {
        &self.path
    }

    /// A connection onto this fixture's file that no store owns.
    ///
    /// How the targeted tests below corrupt a row or arm a fault: both are
    /// operations no conformant caller can reach through the port, which is the
    /// whole reason they need a connection of their own.
    fn raw_connection(&self) -> rusqlite::Connection {
        rusqlite::Connection::open(&self.path)
            .expect("a broken test environment, not a non-conformant adapter")
    }
}

/// Removes a database and both write-ahead-log sidecars, ignoring absence.
fn remove_database(path: &Path) {
    for suffix in ["", "-wal", "-shm"] {
        let mut sidecar = path.to_path_buf().into_os_string();
        sidecar.push(suffix);
        let _ = std::fs::remove_file(PathBuf::from(sidecar));
    }
}

impl Drop for SqliteProjectionFixture {
    fn drop(&mut self) {
        if let Ok(mut handles) = self.handles.lock() {
            handles.clear();
        }
        remove_database(&self.path);
    }
}

impl ProjectionFixture for SqliteProjectionFixture {
    type Store = SqliteProjectionStore;

    /// A MUST, and this adapter meets it for real: a second `connect` is a
    /// second `rusqlite::Connection` onto the same file, not a refcount clone.
    /// PS-1's coupling is only observable from outside the connection that made
    /// the commit, and that is what this buys.
    const SECOND_HANDLE: Capability = Capability::SUPPORTED;

    /// Declined, and the reason is the store's rather than the fixture's
    /// convenience: `SqliteProjectionStore` holds **no protection policy**.
    ///
    /// PS-18 makes refusal a mechanism the port supplies and leaves *what to
    /// protect* to the domain; this adapter owns the checkpoint table and the
    /// transaction that carries it, and knows nothing about which projection a
    /// regulator already holds a hash chain for. There is therefore no path in
    /// `reset` that returns `ResetError::Refused`, and a fixture that claimed the
    /// capability would fail `refused_reset_changes_nothing` at its first
    /// assertion.
    ///
    /// The alternative that lost is worth naming, because it is cheap and it is
    /// wrong: a protected-projections table consulted by `reset`, invented here so
    /// that one more rule reports `Ran`. That would put a **policy** into an
    /// adapter whose whole design keeps the read model — and therefore the
    /// domain — out of it, and would certify a mechanism no application had asked
    /// for. Declining is this adapter's real answer, and PS-18's own falsifier is
    /// *"evaluated by asking whether the SQLite adapter implemented it"*: this
    /// line is that answer.
    const RESET_REFUSAL: Capability = Capability::declined(
        "SqliteProjectionStore holds no protection policy: the read model belongs \
         to the caller and this adapter owns only the checkpoint row and the \
         transaction that carries it, so there is no projection it could decline \
         to reset and `reset` returns `Refused` on no path at all. The \
         alternative — a protected-projections table invented here so that one \
         more rule reports `Ran` — would put a domain policy into an adapter \
         designed to keep the domain out, and certify a mechanism no application \
         asked for",
    );

    /// Supported, and armed for real. This adapter can be made to fail a commit
    /// **between** the read-model writes and the checkpoint write, because both
    /// live in one SQLite transaction and a trigger can abort the second half.
    ///
    /// That is the only way PS-1's second conjunct is observable at all — nothing
    /// a caller holds can make a conformant `commit` fail — so declining here
    /// would have left `failed_commit_leaves_both_unchanged` a reported skip
    /// against the first adapter in this workspace able to answer it.
    const COMMIT_FAULT: Capability = Capability::SUPPORTED;

    /// One more real connection onto this fixture's file.
    ///
    /// Genuinely `async` work rather than a refcount bump: it opens a file,
    /// configures the connection and applies the checkpoint migration. It panics
    /// rather than returning a `Result`, per the trait — a fixture that cannot
    /// connect is a broken test environment, not a non-conformant adapter.
    async fn connect(&self) -> Self::Store {
        let store = SqliteProjectionStore::open(&self.path)
            .expect("a broken test environment, not a non-conformant adapter");
        self.handles
            .lock()
            .expect("the fixture's handle list was poisoned")
            .push(store.clone());
        store
    }

    /// Arms a trigger that aborts the **checkpoint** half of the next commit.
    ///
    /// Installed through a connection of the fixture's own, exactly as
    /// `tests/append.rs::a_failure_mid_batch_leaves_nothing` installs the event
    /// store's: the injection belongs to the adapter, and this is the adapter's.
    /// `BEFORE INSERT` and `BEFORE UPDATE` are both armed because the checkpoint
    /// write is an upsert, so which of the two SQLite reaches depends on whether
    /// the projection has committed before.
    ///
    /// It stays armed for the life of this fixture instance. That is honest
    /// rather than convenient: the rule that arms it commits exactly once
    /// afterwards, and every rule gets its own fixture, so "fires once" and
    /// "fires from now on" are the same run.
    async fn arm_commit_fault(&self) {
        self.raw_connection()
            .execute_batch(
                "CREATE TRIGGER IF NOT EXISTS projection_commit_fault_insert
                 BEFORE INSERT ON projection_checkpoint
                 BEGIN SELECT RAISE(ABORT, 'the fixture armed a commit fault'); END;
                 CREATE TRIGGER IF NOT EXISTS projection_commit_fault_update
                 BEFORE UPDATE ON projection_checkpoint
                 BEGIN SELECT RAISE(ABORT, 'the fixture armed a commit fault'); END;",
            )
            .expect("a broken test environment, not a non-conformant adapter");
    }
}

happenstance_testkit::projection_store_conformance!(SqliteProjectionFixture::new());

// -------------------------------------------------------------------------
// The adapter-private tests the borrowed suite cannot make
// -------------------------------------------------------------------------

/// The probe key these tests write through, distinct from every key the suite
/// uses so a leaked file could never make one of them pass.
const KEY: &str = "depot-99";

/// The value written under [`KEY`]. Not `0` or `1`: a store answering with a
/// default would pass a read-back asserting either.
const VALUE: u64 = 4_211;

/// Commits one probe row at `position`, failing the test with context on error.
async fn commit_one(store: &SqliteProjectionStore, id: &ProjectionId, position: SequencePosition) {
    let mut batch = store.begin();
    store.probe_write(&mut batch, KEY, VALUE);
    store
        .commit(batch, id, position, Authority::Live)
        .await
        .expect("the commit should succeed");
}

/// The suite runs against a **file**, and a second `connect` is a second
/// connection onto it.
///
/// AC-001 and AC-002. Neither is visible to any rule: a fixture whose `connect`
/// cloned one in-process handle passes every one of them, and so does a fixture
/// over `open_in_memory` right up until a rule reads back through a fresh handle.
#[tokio::test]
async fn the_fixture_backs_the_suite_with_a_real_file_and_real_connections() {
    let fixture = SqliteProjectionFixture::new();
    let writer = fixture.connect().await;
    let id = ProjectionId::new("the_fixture_backs_the_suite_with_a_real_file");

    commit_one(&writer, &id, SequencePosition::FIRST).await;

    assert!(
        fixture.path().exists(),
        "the fixture must back the suite with a real temporary file on disk, and \
         nothing was written at {}. An in-memory stand-in passes most of the \
         suite and proves nothing about durable storage",
        fixture.path().display()
    );

    let observer = fixture.connect().await;
    assert_eq!(
        observer.probe_read(KEY).await.expect("the read succeeds"),
        Some(VALUE),
        "a second `connect` must be a second connection onto the same file, and \
         this one could not see what the first committed"
    );
    assert_eq!(
        observer.checkpoint(&id).await.expect("the read succeeds"),
        Checkpoint::Live {
            through: SequencePosition::FIRST
        },
        "a fresh handle must observe the checkpoint the first one committed"
    );
}

/// A stored position that is not a `SequencePosition` is reported, never
/// defaulted.
///
/// AC-004's second half, and EC-003. `SequencePosition` wraps a `NonZeroU64`, so
/// a stored `0` has no representation — and the two silent answers available
/// (`Live { through: 1 }`, or `NeverRun`) are both a lie about a row that exists.
#[tokio::test]
async fn a_corrupt_stored_position_is_reported_rather_than_defaulted() {
    let fixture = SqliteProjectionFixture::new();
    let store = fixture.connect().await;
    let id = ProjectionId::new("a_corrupt_stored_position");

    commit_one(&store, &id, SequencePosition::FIRST).await;

    fixture
        .raw_connection()
        .execute(
            "UPDATE projection_checkpoint SET position = 0 WHERE projection_id = ?",
            [id.as_str()],
        )
        .expect("the corruption is the test environment's, not the adapter's");

    match store.checkpoint(&id).await {
        Err(SqliteProjectionStoreError::InvalidPosition(0)) => {}
        outcome => panic!(
            "a stored position of `0` must surface as `InvalidPosition`, and this \
             store answered {outcome:?}. `Live {{ through: 1 }}` and `NeverRun` \
             are both silent lies about a row that is there"
        ),
    }
}

/// A stored authority this build does not know is reported, never inferred.
///
/// AC-004's first half from the other side: the authority is **read back from a
/// column**, so a value that is not one of the two discriminants is a corrupt
/// row rather than an invitation to assume `Live`.
#[tokio::test]
async fn a_corrupt_stored_authority_is_reported_rather_than_assumed_live() {
    let fixture = SqliteProjectionFixture::new();
    let store = fixture.connect().await;
    let id = ProjectionId::new("a_corrupt_stored_authority");

    commit_one(&store, &id, SequencePosition::FIRST).await;

    fixture
        .raw_connection()
        .execute(
            "UPDATE projection_checkpoint SET authority = 'sideways' WHERE projection_id = ?",
            [id.as_str()],
        )
        .expect("the corruption is the test environment's, not the adapter's");

    match store.checkpoint(&id).await {
        Err(SqliteProjectionStoreError::InvalidAuthority(found)) => {
            assert_eq!(
                found, "sideways",
                "the error must carry what was actually stored, so an operator \
                 can find the row rather than guess at it"
            );
        }
        outcome => panic!(
            "a stored authority this build does not know must surface as \
             `InvalidAuthority`, and this store answered {outcome:?}. Assuming \
             `Live` tells a reader that a half-built read model is authoritative"
        ),
    }
}

/// A **clone** of a store accepts the batch its origin began.
///
/// AC-006's second half. The stamp is minted per store *instance* and copied by
/// `Clone`, because a clone is the same store: it shares the `Arc`, the
/// connection and the file. A stamp derived from a pointer address, or minted in
/// `Clone`, makes a cloned store reject its own batches — and half the suite then
/// fails in ways that look like anything but this.
#[tokio::test]
async fn a_cloned_store_accepts_the_batch_its_origin_began() {
    let fixture = SqliteProjectionFixture::new();
    let origin = fixture.connect().await;
    let clone = origin.clone();
    let id = ProjectionId::new("a_cloned_store_accepts_its_origins_batch");

    let mut batch = origin.begin();
    origin.probe_write(&mut batch, KEY, VALUE);

    clone
        .commit(batch, &id, SequencePosition::FIRST, Authority::Live)
        .await
        .expect(
            "a clone is the same store — same `Arc`, same connection, same file — \
             so it must accept a batch its origin began",
        );

    assert_eq!(
        fixture
            .connect()
            .await
            .probe_read(KEY)
            .await
            .expect("the read succeeds"),
        Some(VALUE),
        "the clone's commit must be durable, or the acceptance above meant nothing"
    );
}

/// A batch begun on one **handle** is refused by another handle onto the same
/// file, by `commit`, by `reset` and by `rollback`.
///
/// AC-006's first half, and the semantics this adapter chooses where the port
/// leaves room: a second `connect` is a second `SqliteProjectionStore`, so it is
/// a different store instance and its stamp differs. Nothing is issued to SQLite
/// before the comparison, which is why the second half of this test can assert
/// that neither store moved.
#[tokio::test]
async fn a_batch_from_another_handle_is_refused_by_every_method_that_takes_one() {
    let fixture = SqliteProjectionFixture::new();
    let origin = fixture.connect().await;
    let stranger = fixture.connect().await;
    let id = ProjectionId::new("a_batch_from_another_handle");

    let mut batch = origin.begin();
    origin.probe_write(&mut batch, KEY, VALUE);
    match stranger
        .commit(batch, &id, SequencePosition::FIRST, Authority::Live)
        .await
    {
        Err(CommitError::ForeignBatch) => {}
        outcome => panic!("`commit` must answer `ForeignBatch`, and answered {outcome:?}"),
    }

    let mut batch = origin.begin();
    origin.probe_delete_all(&mut batch);
    match stranger.reset(batch, &id).await {
        Err(ResetError::ForeignBatch) => {}
        outcome => panic!("`reset` must answer `ForeignBatch`, and answered {outcome:?}"),
    }

    match stranger.rollback(origin.begin()).await {
        Err(SqliteProjectionStoreError::ForeignBatch) => {}
        outcome => panic!("`rollback` must answer `ForeignBatch`, and answered {outcome:?}"),
    }

    assert_eq!(
        stranger.checkpoint(&id).await.expect("the read succeeds"),
        Checkpoint::NeverRun,
        "a refusal decided before any SQL is issued cannot have moved the \
         checkpoint"
    );
    assert_eq!(
        stranger.probe_read(KEY).await.expect("the read succeeds"),
        None,
        "a refusal decided before any SQL is issued cannot have written a row"
    );
}

/// Two concurrent opens of one path both succeed, and the schema is applied once.
///
/// AC-008's second half, and EC-004. `migrate` runs on **every** `open`, and the
/// fixture opens the same file several times per rule, so two `CREATE TABLE`s can
/// race. `IF NOT EXISTS` inside one `BEGIN IMMEDIATE` is what makes the loser a
/// no-op instead of an error.
///
/// Bare OS threads rather than tasks, and deliberately: an `open` outside any
/// tokio runtime is the configuration `Handle::try_current()` fails in, so this
/// also pins that construction is not where the runtime seam is needed.
#[test]
fn concurrent_opens_of_one_path_all_succeed() {
    /// Enough to make the race real without making the test a load generator.
    const OPENERS: usize = 8;

    let fixture = SqliteProjectionFixture::new();
    let barrier = std::sync::Barrier::new(OPENERS);

    let outcomes: Vec<_> = std::thread::scope(|scope| {
        let handles: Vec<_> = (0..OPENERS)
            .map(|_| {
                scope.spawn(|| {
                    barrier.wait();
                    SqliteProjectionStore::open(fixture.path()).map(|_| ())
                })
            })
            .collect();
        handles
            .into_iter()
            .map(|handle| handle.join().expect("the opener thread did not panic"))
            .collect()
    });

    for outcome in &outcomes {
        assert!(
            outcome.is_ok(),
            "every concurrent `open` of one path must succeed — migration is \
             idempotent and safe under a concurrent open — and one answered \
             {outcome:?}"
        );
    }

    let connection = fixture.raw_connection();
    let checkpoint_tables: i64 = connection
        .query_row(
            "SELECT count(*) FROM sqlite_master WHERE type = 'table' AND name = 'projection_checkpoint'",
            [],
            |row| row.get(0),
        )
        .expect("the read succeeds");
    assert_eq!(
        checkpoint_tables, 1,
        "the checkpoint table must exist exactly once after {OPENERS} concurrent \
         opens"
    );

    let event_tables: i64 = connection
        .query_row(
            "SELECT count(*) FROM sqlite_master WHERE type = 'table' AND name = 'event'",
            [],
            |row| row.get(0),
        )
        .expect("the read succeeds");
    assert_eq!(
        event_tables, 0,
        "the projection store migrates on its **own** connection and its own \
         schedule: opening one must not apply the event store's migration"
    );
}

/// A file written by a build that knows a later migration is refused.
///
/// AC-008's version-marker half. The marker exists so that migration 2 has
/// something to test against — a schema with no version marker cannot be migrated
/// later without guessing — and a marker nothing reads is decoration, so this is
/// the read.
#[tokio::test]
async fn a_newer_projection_schema_is_refused() {
    let fixture = SqliteProjectionFixture::new();
    drop(fixture.connect().await);

    fixture
        .raw_connection()
        .execute(
            "UPDATE projection_meta SET v = ? WHERE k = 'schema_version'",
            [SCHEMA_VERSION + 1],
        )
        .expect("the test environment writes the marker, not the adapter");

    match SqliteProjectionStore::open(fixture.path()) {
        Err(SqliteProjectionStoreError::UnsupportedSchemaVersion { found, supported }) => {
            assert_eq!(
                (found, supported),
                (SCHEMA_VERSION + 1, SCHEMA_VERSION),
                "the refusal must name both versions, so an operator learns which \
                 build to run rather than that something is wrong"
            );
        }
        outcome => panic!(
            "a file at a later projection schema version must be refused rather \
             than operated on blind, and `open` answered {}",
            match outcome {
                Ok(_) => "Ok".to_owned(),
                Err(err) => format!("{err:?}"),
            }
        ),
    }
}
