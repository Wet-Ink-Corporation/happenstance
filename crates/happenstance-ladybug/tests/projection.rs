//! The projection conformance suite, run against a real LadybugDB directory —
//! **twice**.
//!
//! This target is the mount point for a suite this crate did not write: every
//! rule drives [`LadybugProjectionStore`] through the port and the probe rather
//! than through anything this crate could have arranged in its own favour.
//!
//! # Why the whole file is behind `conformance`
//!
//! That feature is what compiles `impl ProjectionProbe for
//! LadybugProjectionStore` in `src/`, and [`ProjectionFixture::Store`] is bound
//! on that trait — so without it this file does not compile. It implies `driver`,
//! so this target also carries the 1.44 GB archive and the OpenSSL link, which is
//! why `xtask` runs it as a probed step of its own rather than letting
//! `cargo test --workspace --all-features` reach it (ADR-0025 §9).
//!
//! # Two mounts, and what the pair actually falsifies
//!
//! ADR-0025 §3 settles the blocking question **blocking-only** — no `tokio` in a
//! runtime-agnostic adapter — and then makes the decision falsifiable for free.
//! `projection_store_conformance!` is invoked twice:
//!
//! * `blocking` under `happenstance_testkit::__emit_projection_blocking`, which
//!   expands to a plain `#[test]` driven by the testkit's own `block_on` and
//!   **needs no runtime at all**;
//! * `with_tokio` under the default emitter, which expands to `#[tokio::test]`.
//!
//! A store that reached for `spawn_blocking` panics under the first. That is the
//! artefact, and its worth is stated as narrowly as §3 states it: the tokio
//! emitter is a **current-thread** runtime running one task, so blocking in place
//! starves nothing and the two mounts cannot diverge on account of blocking. What
//! the pair falsifies is **runtime-agnosticism**, not the cost of blocking. The
//! cost claim would need concurrent work on a current-thread runtime, which this
//! suite does not have.
//!
//! # What the fixture is made of, which is where two criteria are won or lost
//!
//! The rules cannot see either of these, and a wrong fixture passes every one of
//! them:
//!
//! * **One instance is one fresh temporary directory**, so two instances share
//!   nothing. `commit_rejects_a_foreign_batch` opens the fixture *twice* and
//!   wants two isolated stores.
//! * **Each `connect()` is a second store instance over the one shared
//!   `Arc<Database>`** — [`LadybugProjectionStore::second_handle`], never a
//!   `Clone`. A clone copies the batch stamp and *is* the same store; a second
//!   handle is a different instance onto one graph, which is what PS-1's
//!   out-of-connection observability needs and what the foreign-batch rules are
//!   about. It cannot be a second `Database`: that is refused by a file lock
//!   (ADR-0025 §6), which is the measurement the `Arc` exists for.

#![cfg(feature = "conformance")]
// The house style's test-module exception: a fixture that cannot open its own
// temporary directory is a broken test environment, and `expect` says so at the
// point it happens.
#![allow(clippy::unwrap_used)]

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use happenstance_ladybug::lbug::{SystemConfig, Value};
use happenstance_ladybug::{LadybugProjectionStore, projection_store};
use happenstance_testkit::{Capability, ProjectionFixture};

/// The engine configuration every fixture instance opens with.
///
/// **Not `SystemConfig::default()`, and the difference is the reason
/// [`LadybugProjectionStore::open_with_config`] exists.** The default
/// `max_db_size` is `u32::MAX` — 4 GiB of reserved address space per open
/// database — and this binary opens one per fixture instance, of which the two
/// mounts together make several dozen. `lbug`'s own test configuration makes the
/// same reduction for the same stated reason: *"it limits the number of databases
/// which can be open in a single process"*.
///
/// The buffer pool is capped for the same arithmetic. A default pool is
/// auto-detected from host memory, which is the right answer for one database and
/// the wrong one for forty in a process.
fn fixture_config() -> SystemConfig {
    SystemConfig::default()
        .max_db_size(64 * 1024 * 1024)
        .buffer_pool_size(16 * 1024 * 1024)
}

/// One fixture instance is **one temporary LadybugDB directory**; each `connect`
/// is one more store instance over the single `Database` that holds its lock; two
/// instances share nothing.
#[derive(Debug)]
struct LadybugProjectionFixture {
    directory: PathBuf,
    /// The one store that owns the `Arc<Database>` every handle shares.
    ///
    /// [`Option`] so that [`Drop`] can put the database down *before* removing
    /// the directory: LadybugDB holds a lock on it, and Windows refuses to unlink
    /// an open file. This is the shape `happenstance-sqlite`'s fixture reaches
    /// with a `Mutex<Vec<_>>` of handed-out handles, arrived at from the other
    /// direction — here there is exactly one owner to put down, because a second
    /// `Database` on one directory does not exist.
    store: Option<LadybugProjectionStore>,
}

impl LadybugProjectionFixture {
    /// A fixture over a fresh temporary directory that nothing else is using.
    ///
    /// A process-local ordinal plus the process id, the shape both other
    /// adapters' fixtures already use: no new dependency, and no two instances in
    /// a run can collide on a path.
    fn new() -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(1);
        let ordinal = NEXT.fetch_add(1, Ordering::Relaxed);
        let mut directory = std::env::temp_dir();
        directory.push(format!(
            "happenstance-ladybug-projection-{}-{ordinal}",
            std::process::id()
        ));
        // A leftover from a previous run would make "a fresh, isolated backing
        // store" a lie, which is the one promise the fixture contract makes — and
        // here it would also be a *stale schema*, which is worse than stale data.
        let _ = std::fs::remove_dir_all(&directory);

        let store = LadybugProjectionStore::open_with_config(&directory, fixture_config())
            .expect("a broken test environment, not a non-conformant adapter");
        Self {
            directory,
            store: Some(store),
        }
    }

    /// The store that owns the database, for the fixture's own use.
    fn owner(&self) -> &LadybugProjectionStore {
        self.store
            .as_ref()
            .expect("the fixture's store is taken only in `Drop`")
    }

    /// The directory this fixture's stores are opened against.
    fn directory(&self) -> &Path {
        &self.directory
    }
}

impl Drop for LadybugProjectionFixture {
    fn drop(&mut self) {
        // The database first, then the directory. Reversing these leaves the
        // directory behind on every run, because the lock is still held.
        drop(self.store.take());
        let _ = std::fs::remove_dir_all(&self.directory);
    }
}

impl ProjectionFixture for LadybugProjectionFixture {
    type Store = LadybugProjectionStore;

    /// A MUST, and this adapter meets it for real — by the one route the engine
    /// leaves open.
    ///
    /// A second `Database::new` on this directory is **refused by a file lock**,
    /// so a second handle cannot be a second database. It is a second
    /// [`LadybugProjectionStore`] over the same `Arc<Database>`, opening its own
    /// `Connection` per call, and that is enough: PS-1's coupling is only
    /// observable from outside the connection that made the commit, and a
    /// connection opened after the commit is outside it.
    const SECOND_HANDLE: Capability = Capability::SUPPORTED;

    /// Declined, and the reason is the store's rather than the fixture's
    /// convenience: `LadybugProjectionStore` holds **no protection policy**.
    ///
    /// PS-18 makes refusal a mechanism the port supplies and leaves *what to
    /// protect* to the domain. This adapter owns one node table and the
    /// transaction that carries it; the read model — which is the whole graph
    /// beside it — belongs to the caller, and nothing in this crate knows which
    /// projection somebody upstream has promised not to rebuild. There is
    /// therefore no path in `reset` that returns `ResetError::Refused`, and a
    /// fixture claiming the capability would fail
    /// `refused_reset_changes_nothing` at its first assertion.
    ///
    /// The alternative that lost is worth naming because it is cheap and it is
    /// wrong: a `__hs_protected` node table consulted by `reset`, invented here
    /// so that one more rule reports `Ran`. That would put a **domain policy**
    /// into an adapter whose whole design keeps the domain out of it — this store
    /// does not even know what labels the read model uses — and would certify a
    /// mechanism no application had asked for.
    const RESET_REFUSAL: Capability = Capability::declined(
        "LadybugProjectionStore holds no protection policy. It owns one node \
         table, __hs_checkpoint, and the transaction that carries it; the read \
         model is the caller's graph and this adapter is never told which labels \
         it uses, let alone which projection somebody upstream has promised not \
         to rebuild. So `reset` returns `Refused` on no path at all. The \
         alternative — a __hs_protected node table consulted by `reset`, invented \
         here so that one more rule reports `Ran` — would put a domain policy \
         into an adapter designed to keep the domain out, and certify a mechanism \
         no application asked for",
    );

    /// Supported, and armed for real.
    ///
    /// See [`LadybugProjectionFixture::arm_commit_fault`] for the injection and
    /// for the two candidates that lost.
    const COMMIT_FAULT: Capability = Capability::SUPPORTED;

    /// One more store instance over the same `Arc<Database>`.
    ///
    /// Genuinely a second handle rather than a refcount bump of the *store*: it
    /// mints a new batch stamp, so a write set begun on one handle is foreign to
    /// another, which is what `commit_rejects_a_foreign_batch` and this crate's
    /// own cross-handle test are about. It panics rather than returning a
    /// `Result`, per the trait — a fixture that cannot connect is a broken test
    /// environment, not a non-conformant adapter.
    async fn connect(&self) -> Self::Store {
        self.owner().second_handle()
    }

    /// Makes the next commit fail on its **read-model** statement, by moving the
    /// probe table's primary key out from under the `MERGE` the store issues.
    ///
    /// `__hs_probe` is recreated with `PRIMARY KEY(v)` instead of `PRIMARY
    /// KEY(k)`, through a connection of the fixture's own — exactly as
    /// `happenstance-sqlite`'s fixture installs its trigger through one, and on
    /// the same principle: the injection belongs to the adapter, and this is the
    /// adapter's read model. `probe_write` issues `MERGE (p:__hs_probe {k: $k})
    /// SET p.v = $v`; a `MERGE` that finds no match must `CREATE`, and a `CREATE`
    /// whose primary key is not in the pattern is refused —
    ///
    /// ```text
    /// Binder exception: Create node p expects primary key v as input.
    /// ```
    ///
    /// — while `MATCH (p:__hs_probe {k: $k}) RETURN p.v`, which is the read path
    /// both this rule and the store use afterwards, still binds and answers
    /// `None`. That combination is the whole constraint, and it is much narrower
    /// than it looks.
    ///
    /// # Why this fires *before* the read-model write rather than after it
    ///
    /// Stated because it is a real limitation of this injection and the rule
    /// cannot see it. `happenstance-sqlite`'s fixture faults the **checkpoint**
    /// half, so its run of `failed_commit_leaves_both_unchanged` also refutes the
    /// specific wrong implementation the rule's own message names — one that
    /// applies its rows, fails to write the checkpoint, and keeps the rows. This
    /// adapter's run does not reach that state: the commit fails at the first
    /// statement of the batch.
    ///
    /// It is not for want of trying, and the reason is a property of the
    /// **schema** rather than of the adapter. The checkpoint read-back the rule
    /// performs immediately afterwards needs `__hs_checkpoint` intact down to the
    /// types of both properties *and* the anchor node's values, so every fault
    /// arm-able on that table breaks the assertion it is arming for:
    ///
    /// * `ALTER TABLE __hs_checkpoint DROP authority` makes the checkpoint
    ///   `MERGE` fail — and `checkpoint()` then fails too, with
    ///   `Binder exception: Cannot find property authority for c`. Measured, not
    ///   predicted; it is what the first draft of this fixture did.
    /// * Retyping `authority` to `UINT64` was the near miss. It would fault the
    ///   *last* statement, after the row landed — but dropping and re-adding a
    ///   property nulls it on every existing node, so the anchor checkpoint
    ///   becomes unreadable and the rule's `before` comparison has nothing to
    ///   compare.
    ///
    /// # Two more candidates that lost, named because they look better
    ///
    /// **A duplicate primary key on a pre-planted `CREATE`** is what
    /// `experiments/ladybug-driver-probes/` measured and what ADR-0025 §8 names.
    /// It raises — that is confirmed — but nothing a fixture can reach makes the
    /// *store* issue a `CREATE`: the checkpoint write is a `MERGE`, which matches
    /// rather than conflicts, and so is the probe write. The mechanism is real
    /// and the store gives it nothing to collide with, which is a finding about
    /// the injection rather than about the engine.
    ///
    /// **A second connection holding an open write transaction** exercises
    /// LadybugDB's one-writer rule for real —
    /// `Cannot start a new write transaction in the system. Only one write
    /// transaction at a time is allowed in the system.` — and it is what
    /// `LadybugProjectionStoreError::WriteTransactionInUse` is classified from.
    /// It is not used here for two reasons, and the second is the serious one.
    /// It fails the commit at `BEGIN`, before either half is attempted, which is
    /// further from "part way" rather than closer. And **the driver segfaults**
    /// on the path immediately beyond it: a connection that issues a statement
    /// after its own `BEGIN TRANSACTION` was refused crashes the process with
    /// `STATUS_ACCESS_VIOLATION`. This adapter cannot reach that path — `commit`
    /// returns the moment `BEGIN` fails — but a fixture that armed the fault this
    /// way would be one refactor away from a test binary that dies instead of
    /// failing.
    ///
    /// It stays armed for the life of this fixture instance. That is honest
    /// rather than convenient: the rule that arms it commits exactly once
    /// afterwards, and every rule gets its own fixture, so "fires once" and
    /// "fires from now on" are the same run.
    async fn arm_commit_fault(&self) {
        let connection = self
            .owner()
            .connect()
            .expect("a broken test environment, not a non-conformant adapter");
        for statement in [
            "DROP TABLE __hs_probe",
            "CREATE NODE TABLE __hs_probe(k STRING, v UINT64, PRIMARY KEY(v))",
        ] {
            connection
                .query(statement)
                .expect("a broken test environment, not a non-conformant adapter");
        }
    }
}

happenstance_testkit::projection_store_conformance!(
    mod_name = blocking,
    emit = happenstance_testkit::__emit_projection_blocking,
    fixture = LadybugProjectionFixture::new()
);

happenstance_testkit::projection_store_conformance!(
    mod_name = with_tokio,
    emit = happenstance_testkit::__emit_projection_tokio,
    fixture = LadybugProjectionFixture::new()
);

// -------------------------------------------------------------------------
// The adapter-private tests the borrowed suite cannot make
// -------------------------------------------------------------------------

/// The probe key these tests write through, distinct from every key the suite
/// uses so a leaked directory could never make one of them pass.
const KEY: &str = "depot-99";

/// The value written under [`KEY`]. Deliberately **above `i64::MAX`**, which is
/// the whole of ADR-0025 §1 exercised in one constant: this is a position no
/// signed `bigint` can hold, and the two adapters over one carry a fallible
/// converter that this adapter does not need.
const VALUE: u64 = u64::MAX - 1;

/// A stored position round-trips through `UINT64` above `i64::MAX`.
///
/// ADR-0025 §1, and the reason `PositionOutOfRange` is deleted rather than kept
/// as cheap insurance: there is no narrowing for it to describe. The probe
/// measured this against a bare `lbug`; this measures it **through the adapter**,
/// which is the claim the deleted variant was about.
#[tokio::test]
async fn a_position_above_i64_max_round_trips_through_the_store() {
    use happenstance_core::{
        Authority, Checkpoint, ProjectionId, ProjectionStore, SequencePosition,
    };

    let fixture = LadybugProjectionFixture::new();
    let writer = fixture.connect().await;
    let id = ProjectionId::new("a_position_above_i64_max");
    let position = SequencePosition::new(VALUE).expect("u64::MAX - 1 is not zero");

    let batch = writer.begin();
    writer
        .commit(batch, &id, position, Authority::Live)
        .await
        .expect("a UINT64 property holds the whole of a NonZeroU64");

    assert_eq!(
        fixture
            .connect()
            .await
            .checkpoint(&id)
            .await
            .expect("the read succeeds"),
        Checkpoint::Live { through: position },
        "a position above i64::MAX must round-trip exactly. happenstance-sqlite \
         and happenstance-postgres both narrow here and carry a fallible \
         converter for it; this adapter's column is UINT64 and does not"
    );
}

/// A stored `0` is reported, never defaulted.
///
/// EC-003's shape, and the one honest case
/// [`projection_store::LadybugProjectionStoreError::MalformedCheckpoint`] narrowed
/// to. `SequencePosition` wraps a `NonZeroU64`, so a stored zero has no
/// representation — and both silent answers available (`Live { through: 1 }`, and
/// `NeverRun`) are a lie about a node that exists.
#[tokio::test]
async fn a_stored_zero_is_reported_rather_than_defaulted() {
    use happenstance_core::{Authority, ProjectionId, ProjectionStore, SequencePosition};

    let fixture = LadybugProjectionFixture::new();
    let store = fixture.connect().await;
    let id = ProjectionId::new("a_stored_zero");

    let batch = store.begin();
    store
        .commit(batch, &id, SequencePosition::FIRST, Authority::Live)
        .await
        .expect("the commit should succeed");

    let connection = store.connect().expect("the fixture can connect");
    let mut prepared = connection
        .prepare("MATCH (c:__hs_checkpoint {projection: $p}) SET c.position = 0")
        .expect("the corruption is the test environment's, not the adapter's");
    connection
        .execute(
            &mut prepared,
            vec![("p", Value::String(id.as_str().to_owned()))],
        )
        .expect("the corruption is the test environment's, not the adapter's");

    match store.checkpoint(&id).await {
        Err(projection_store::LadybugProjectionStoreError::MalformedCheckpoint {
            value: 0,
            ..
        }) => {}
        outcome => panic!(
            "a stored position of `0` must surface as `MalformedCheckpoint`, and \
             this store answered {outcome:?}. `Live {{ through: 1 }}` and \
             `NeverRun` are both silent lies about a node that is there"
        ),
    }
}

/// A stored authority this build does not know is reported, never inferred.
///
/// The authority is **read back from a property**, so a value that is not one of
/// the two discriminants is a corrupt node rather than an invitation to assume
/// `Live` — which would tell a reader that a half-built read model is
/// authoritative.
#[tokio::test]
async fn a_stored_authority_this_build_does_not_know_is_reported() {
    use happenstance_core::{Authority, ProjectionId, ProjectionStore, SequencePosition};

    let fixture = LadybugProjectionFixture::new();
    let store = fixture.connect().await;
    let id = ProjectionId::new("a_stored_authority");

    let batch = store.begin();
    store
        .commit(batch, &id, SequencePosition::FIRST, Authority::Live)
        .await
        .expect("the commit should succeed");

    let connection = store.connect().expect("the fixture can connect");
    let mut prepared = connection
        .prepare("MATCH (c:__hs_checkpoint {projection: $p}) SET c.authority = 'sideways'")
        .expect("the corruption is the test environment's, not the adapter's");
    connection
        .execute(
            &mut prepared,
            vec![("p", Value::String(id.as_str().to_owned()))],
        )
        .expect("the corruption is the test environment's, not the adapter's");

    match store.checkpoint(&id).await {
        Err(projection_store::LadybugProjectionStoreError::MalformedAuthority(found)) => {
            assert_eq!(
                found, "sideways",
                "the error must carry what was actually stored, so an operator \
                 can find the node rather than guess at it"
            );
        }
        outcome => panic!(
            "a stored authority this build does not know must surface as \
             `MalformedAuthority`, and this store answered {outcome:?}"
        ),
    }
}

/// A **clone** accepts its origin's write set; a **second handle** does not.
///
/// The pair is the point, and the two halves are one test because a stamp scheme
/// that gets either half wrong usually gets the other wrong in the opposite
/// direction. A clone shares the `Arc`, the database and the directory, so it
/// *is* the same store; a second handle is a different instance onto one graph
/// and its stamp differs. A stamp derived from a pointer address, or minted in
/// `Clone`, inverts both.
#[tokio::test]
async fn a_clone_accepts_its_origins_write_set_and_a_second_handle_does_not() {
    use happenstance_core::{
        Authority, Checkpoint, CommitError, ProjectionId, ProjectionProbe, ProjectionStore,
        ResetError, SequencePosition,
    };

    let fixture = LadybugProjectionFixture::new();
    let origin = fixture.connect().await;
    let clone = origin.clone();
    let stranger = fixture.connect().await;
    let id = ProjectionId::new("a_clone_and_a_second_handle");

    let mut batch = origin.begin();
    origin.probe_write(&mut batch, KEY, VALUE);
    clone
        .commit(batch, &id, SequencePosition::FIRST, Authority::Live)
        .await
        .expect(
            "a clone is the same store — same Arc, same database, same directory \
             — so it must accept a write set its origin began",
        );

    let mut batch = origin.begin();
    origin.probe_write(&mut batch, KEY, 1);
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
        Err(projection_store::LadybugProjectionStoreError::ForeignBatch) => {}
        outcome => panic!("`rollback` must answer `ForeignBatch`, and answered {outcome:?}"),
    }

    assert_eq!(
        stranger.checkpoint(&id).await.expect("the read succeeds"),
        Checkpoint::Live {
            through: SequencePosition::FIRST
        },
        "three refusals decided before any Cypher is issued cannot have moved the \
         checkpoint the clone committed"
    );
    assert_eq!(
        stranger.probe_read(KEY).await.expect("the read succeeds"),
        Some(VALUE),
        "and cannot have overwritten the row the clone's commit wrote"
    );
}

/// A second `Database` on one directory is refused, which is why the store owns
/// an `Arc`.
///
/// ADR-0025 §6's measurement, kept where a change in the engine would fail rather
/// than merely make a paragraph wrong. The `Arc<Database>` in the store's field
/// list is not a performance choice and this is what says so.
#[test]
fn a_second_database_on_one_directory_is_refused() {
    let fixture = LadybugProjectionFixture::new();

    let second = LadybugProjectionStore::open_with_config(fixture.directory(), fixture_config());

    assert!(
        second.is_err(),
        "a second Database on one directory must be refused by LadybugDB's file \
         lock. If this ever starts succeeding, `SECOND_HANDLE` could be answered \
         with a second database and the `Arc<Database>` in the store's fields \
         stops being forced — which is a decision to re-take, not a test to \
         delete"
    );
}
