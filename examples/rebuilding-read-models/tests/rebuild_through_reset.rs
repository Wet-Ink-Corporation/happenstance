//! A typed-runner rebuild through `reset`, on a read model of two tables (PS-16).
//!
//! The port rule `reset_clears_rows_and_checkpoint_together` proves the
//! adapter's half with one probe table and the probe's own delete. What it
//! cannot show is the composed path an application takes: a typed
//! `Projection` writing two tables through `run_projection`, cleared by the
//! application's own deletes in one `reset` batch, and driven again to the same
//! rows. That is what this file holds, against `happenstance-sqlite`.
//!
//! # The two tables fail differently, on purpose
//!
//! `depot_stock` accumulates (`units = units + excluded.units`), so a table the
//! reset forgot is *doubled* by the rebuild and the sums disagree. `movement` is
//! insert-only, keyed by the event's id, so a table the reset forgot makes the
//! rebuild's commit fail on its primary key. A reset that clears one of the two
//! is caught either way, and by a different symptom.
//!
//! # Reset under `checkpoint_id(&view)`, never under `view.id()`
//!
//! The runner reads and commits under the derived id (PS-25), the name and a
//! digest of the query. `ProjectionStore::reset` and
//! `SqliteProjectionStore::protect_from_reset` both take a bare `ProjectionId`,
//! so `view.id()` type-checks in either place, and both calls then target a row
//! the runner never wrote: the reset clears the tables and leaves the real
//! checkpoint standing, so the rebuild applies nothing and returns `Ok`; the
//! protection protects nothing. Every call below goes through the one `key`
//! each test derives with `checkpoint_id`.
//!
//! # What a protected reset does to the flow
//!
//! `reset` reads the protection mark inside its own `BEGIN IMMEDIATE`, before
//! any statement in the batch, and answers `ResetError::Refused` having written
//! nothing. So a protected view is not rebuilt, and it is not stopped either:
//! protection guards `reset`, not `commit`, and the next run resumes from the
//! checkpoint the refusal left. An operator who wants that view rebuilt lifts
//! the mark (`allow_reset`, an explicit act), or builds it again under a second
//! `ProjectionId`, which is the blue/green route `src/main.rs` demonstrates.
//!
//! # Positions
//!
//! None is written down. Every position compared here was read back from the
//! event store's own log, from `Progressed::through`, or from
//! `ProjectionStore::checkpoint`, because the specification permits gaps.

mod rebuild_through_reset {
    use std::collections::BTreeMap;
    use std::convert::Infallible;
    use std::error::Error;
    use std::io;
    use std::num::{NonZeroU32, NonZeroUsize};
    use std::path::{Path, PathBuf};

    use happenstance::bytes::Bytes;
    use happenstance::{
        Checkpoint, Chunk, Codec, CodecError, CommandOutcome, DecisionModel, Delivered,
        DomainEvent, EventStore, EventType, Json, Progressed, Projection, ProjectionId,
        ProjectionStore, Query, ReadOptions, ResetError, Retry, SequencePosition, Tags,
        checkpoint_id, collect, commit, run_projection,
    };
    use happenstance_sqlite::connection::open_configured;
    use happenstance_sqlite::event_store::SqliteEventStore;
    use happenstance_sqlite::projection_store::{
        SqliteBatch, SqliteProjectionStore, SqliteProjectionStoreError,
    };
    use rusqlite::Connection;
    use rusqlite::types::Value;
    use serde::{Deserialize, Serialize};

    /// What every test here returns. A test is not a library signature, so the
    /// erased error is the right one: each `?` reports its own `Debug`.
    type TestResult = Result<(), Box<dyn Error>>;

    // -----------------------------------------------------------------------
    // The domain
    // -----------------------------------------------------------------------

    const RECEIVED: EventType = EventType::from_static("DepotReceived");
    const SHIPPED: EventType = EventType::from_static("DepotShipped");

    /// Stock moving in and out of a depot.
    ///
    /// `Clone` because `commit`'s decision is `FnMut` and may run once per
    /// attempt, so the events it returns are produced afresh each time.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    enum Movement {
        /// Units arrived at a depot.
        Received { depot: String, units: u32 },
        /// Units left a depot.
        Shipped { depot: String, units: u32 },
    }

    impl Movement {
        fn received(depot: &str, units: u32) -> Self {
            Self::Received {
                depot: depot.to_owned(),
                units,
            }
        }

        fn shipped(depot: &str, units: u32) -> Self {
            Self::Shipped {
                depot: depot.to_owned(),
                units,
            }
        }

        /// The depot this moves, and the signed change to its stock.
        fn delta(&self) -> (&str, i64) {
            match self {
                Self::Received { depot, units } => (depot, i64::from(*units)),
                Self::Shipped { depot, units } => (depot, -i64::from(*units)),
            }
        }
    }

    impl DomainEvent for Movement {
        const EVENT_TYPES: &'static [EventType] = &[RECEIVED, SHIPPED];

        fn event_type(&self) -> EventType {
            match self {
                Self::Received { .. } => RECEIVED,
                Self::Shipped { .. } => SHIPPED,
            }
        }

        fn tags(&self) -> Tags {
            Tags::empty()
        }

        fn encode<C: Codec>(&self, codec: &C) -> Result<Bytes, CodecError> {
            codec.encode(self)
        }

        fn decode<C: Codec>(
            codec: &C,
            _event_type: &EventType,
            data: &Bytes,
        ) -> Result<Self, CodecError> {
            codec.decode(data)
        }
    }

    /// The boundary the log is written through: every movement, and no rule.
    ///
    /// These tests are about what happens to a read model, so the decision
    /// accepts everything. `commit` is used anyway, rather than a raw
    /// `append`, because it frames the payload with the codec the runner reads.
    #[derive(Debug, Clone)]
    struct Ledger {
        scope: Tags,
    }

    impl DecisionModel for Ledger {
        type Event = Movement;

        fn scope(&self) -> &Tags {
            &self.scope
        }

        fn apply(&mut self, _event: Movement) {}
    }

    /// Six movements over two depots.
    fn first_seed() -> Vec<Movement> {
        vec![
            Movement::received("north", 10),
            Movement::received("south", 7),
            Movement::shipped("north", 3),
            Movement::received("north", 4),
            Movement::shipped("south", 2),
            Movement::shipped("north", 1),
        ]
    }

    /// The stock each depot should hold after `movements`, folded here rather
    /// than written down, so the expectation cannot drift from the seed.
    fn expected(movements: &[Movement]) -> BTreeMap<String, i64> {
        let mut totals = BTreeMap::new();
        for movement in movements {
            let (depot, delta) = movement.delta();
            *totals.entry(depot.to_owned()).or_insert(0) += delta;
        }
        totals
    }

    // -----------------------------------------------------------------------
    // The view: one projection, two tables
    // -----------------------------------------------------------------------

    /// The application's read-model schema. The projection store migrates its
    /// own checkpoint tables and nothing else, so these are created on the
    /// application's connection, as `src/main.rs` does.
    const READ_MODEL_DDL: &str = "\
        CREATE TABLE depot_stock (depot TEXT PRIMARY KEY, units INTEGER NOT NULL) STRICT;
        CREATE TABLE movement (
            event_id TEXT PRIMARY KEY,
            depot TEXT NOT NULL,
            delta INTEGER NOT NULL
        ) STRICT;";

    const STOCK_UPSERT: &str = "\
        INSERT INTO depot_stock (depot, units) VALUES (?1, ?2) \
        ON CONFLICT(depot) DO UPDATE SET units = depot_stock.units + excluded.units";

    const MOVEMENT_INSERT: &str =
        "INSERT INTO movement (event_id, depot, delta) VALUES (?1, ?2, ?3)";

    /// The application's own deletes: one statement per table the view owns.
    const CLEAR_EVERY_TABLE: [&str; 2] = ["DELETE FROM movement", "DELETE FROM depot_stock"];

    const DEPOT_VIEW: ProjectionId = ProjectionId::from_static("depot_view");

    /// Stock per depot, and the ledger of every movement that made it.
    #[derive(Debug)]
    struct DepotView {
        id: ProjectionId,
        scope: Tags,
    }

    impl DepotView {
        fn new() -> Self {
            Self {
                id: DEPOT_VIEW,
                scope: Tags::empty(),
            }
        }
    }

    impl Projection for DepotView {
        type Event = Movement;
        type Store = SqliteProjectionStore;
        type Error = SqliteProjectionStoreError;

        fn id(&self) -> &ProjectionId {
            &self.id
        }

        fn scope(&self) -> &Tags {
            &self.scope
        }

        async fn apply(
            &mut self,
            event: Delivered<Movement>,
            batch: &mut SqliteBatch,
        ) -> Result<(), SqliteProjectionStoreError> {
            let event_id = event.id().to_string();
            let movement = event.into_event();
            let (depot, delta) = movement.delta();

            // The depot is written into both tables, so it is duplicated
            // rather than borrowed: each statement owns its parameters.
            batch.push(
                STOCK_UPSERT,
                [Value::Text(depot.to_owned()), Value::Integer(delta)],
            );
            batch.push(
                MOVEMENT_INSERT,
                [
                    Value::Text(event_id),
                    Value::Text(depot.to_owned()),
                    Value::Integer(delta),
                ],
            );
            Ok(())
        }
    }

    // -----------------------------------------------------------------------
    // Reading the view back
    // -----------------------------------------------------------------------

    /// Every row of both tables, in key order.
    #[derive(Debug, Default, PartialEq, Eq)]
    struct Snapshot {
        stock: Vec<(String, i64)>,
        moves: Vec<(String, String, i64)>,
    }

    impl Snapshot {
        fn totals(&self) -> BTreeMap<String, i64> {
            self.stock.iter().cloned().collect()
        }
    }

    fn snapshot(app: &Connection) -> rusqlite::Result<Snapshot> {
        let mut stock = app.prepare("SELECT depot, units FROM depot_stock ORDER BY depot")?;
        let stock = stock
            .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))?
            .collect::<rusqlite::Result<_>>()?;

        let mut moves =
            app.prepare("SELECT event_id, depot, delta FROM movement ORDER BY event_id")?;
        let moves = moves
            .query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))?
            .collect::<rusqlite::Result<_>>()?;

        Ok(Snapshot { stock, moves })
    }

    /// What a rebuild must reproduce, as the properties that did not hold.
    ///
    /// One predicate, so the test that holds the rebuild and the test that
    /// shows the runbook procedure failing it are judged by the same thing:
    /// the rows the first run wrote, the checkpoint it left, and every event
    /// applied again rather than none.
    fn unrestored(
        before: &Snapshot,
        after: &Snapshot,
        rebuild: Progressed,
        head: SequencePosition,
        seeded: usize,
    ) -> Vec<&'static str> {
        let mut failed = Vec::new();
        if after != before {
            failed.push("rows");
        }
        if rebuild.through != Some(head) {
            failed.push("through");
        }
        if rebuild.applied != seeded {
            failed.push("applied");
        }
        failed
    }

    // -----------------------------------------------------------------------
    // The file, the stores, and the operations on them
    // -----------------------------------------------------------------------

    /// One database file, and every handle onto it.
    ///
    /// `file` is the last field so it drops last: the stores and the
    /// connection are closed before the file under them is removed.
    #[derive(Debug)]
    struct Fixture {
        events: SqliteEventStore,
        models: SqliteProjectionStore,
        app: Connection,
        file: TempFile,
    }

    impl Fixture {
        /// A fresh file named for one test, so tests running in parallel never
        /// meet on one.
        fn open(test: &str) -> Result<Self, Box<dyn Error>> {
            let file = TempFile::new(test)?;
            let events = SqliteEventStore::open(&file.path)?;
            let models = SqliteProjectionStore::open(&file.path)?;
            let app = open_configured(&file.path)?;
            app.execute_batch(READ_MODEL_DDL)?;
            Ok(Self {
                events,
                models,
                app,
                file,
            })
        }

        /// Appends `movements` in one command.
        async fn append(&self, movements: &[Movement]) -> TestResult {
            let ledger = Ledger {
                scope: Tags::empty(),
            };
            let outcome = commit(
                &self.events,
                ledger,
                Retry::attempts(NonZeroU32::MIN),
                |_: &Ledger| Ok::<_, Infallible>(movements.to_vec()),
            )
            .await?;
            if matches!(outcome, CommandOutcome::Committed(_)) {
                Ok(())
            } else {
                Err(format!("{} movement(s) decided, nothing appended", movements.len()).into())
            }
        }

        /// A second projection-store handle onto the same file.
        ///
        /// Opened by the caller *before* a protection is written, so the mark
        /// can only reach it through the file (CF-43).
        fn second_models(&self) -> Result<SqliteProjectionStore, SqliteProjectionStoreError> {
            SqliteProjectionStore::open(&self.file.path)
        }

        /// The position the store assigned the last event in its log.
        async fn head(&self) -> Result<SequencePosition, Box<dyn Error>> {
            let log = collect(self.events.read(&Query::all(), ReadOptions::new())).await?;
            let last = log.last().ok_or("the log is empty")?;
            Ok(last.position)
        }

        /// Drives `view` to the head of the log, `chunk` events per commit.
        async fn run(
            &self,
            view: &mut DepotView,
            chunk: usize,
        ) -> Result<Progressed, Box<dyn Error>> {
            let chunk = Chunk::of(NonZeroUsize::try_from(chunk)?);
            Ok(run_projection(&self.events, &self.models, view, &Json, chunk).await?)
        }
    }

    /// Clears the view's tables and returns `key` to `NeverRun`, through
    /// `models`, as one `reset`.
    ///
    /// `extra` is pushed after the application's own deletes, so a test can
    /// make a later statement in the batch fail.
    ///
    /// The batch is begun on `models`, the handle that resets it: a batch
    /// begun on another handle is refused as `ForeignBatch`.
    async fn clear(
        models: &SqliteProjectionStore,
        key: &ProjectionId,
        extra: &[&'static str],
    ) -> Result<(), ResetError<SqliteProjectionStoreError>> {
        let mut batch = models.begin().await.map_err(ResetError::Store)?;
        for statement in CLEAR_EVERY_TABLE.iter().chain(extra) {
            batch.push(statement, []);
        }
        models.reset(batch, key).await
    }

    /// A database path under the system's temporary directory, removed with
    /// the two files WAL mode keeps beside it when this is dropped.
    #[derive(Debug)]
    struct TempFile {
        path: PathBuf,
    }

    impl TempFile {
        fn new(test: &str) -> io::Result<Self> {
            let name = format!(
                "happenstance-rebuild-through-reset-{}-{test}.db",
                std::process::id()
            );
            let file = Self {
                path: std::env::temp_dir().join(name),
            };
            // A file left by an earlier process that reused this pid would
            // otherwise be opened as this test's "fresh" database.
            file.remove()?;
            Ok(file)
        }

        fn remove(&self) -> io::Result<()> {
            for suffix in ["", "-wal", "-shm"] {
                let mut name = self.path.as_os_str().to_owned();
                name.push(suffix);
                match std::fs::remove_file(Path::new(&name)) {
                    Err(e) if e.kind() != io::ErrorKind::NotFound => return Err(e),
                    _ => {}
                }
            }
            Ok(())
        }
    }

    impl Drop for TempFile {
        fn drop(&mut self) {
            // `Drop` cannot return the failure, so it is reported rather than
            // discarded: a file left behind is a test environment problem worth
            // seeing, not one worth failing a passed test over.
            if let Err(e) = self.remove() {
                eprintln!("could not remove {}: {e}", self.path.display());
            }
        }
    }

    /// The chunk size every run uses unless it is the subject: larger than
    /// any log here, so one commit carries the whole run.
    const CHUNK: usize = 500;

    /// A view built over [`first_seed`], and what that left behind.
    #[derive(Debug)]
    struct Built {
        seed: Vec<Movement>,
        head: SequencePosition,
        before: Snapshot,
    }

    /// Seeds the log, drives `view` to its head, and checks the build is the
    /// one every rule below starts from: caught up, and holding the folded
    /// totals and one ledger row per movement.
    async fn build(
        fx: &Fixture,
        view: &mut DepotView,
        chunk: usize,
    ) -> Result<Built, Box<dyn Error>> {
        let seed = first_seed();
        fx.append(&seed).await?;
        let head = fx.head().await?;

        let progress = fx.run(view, chunk).await?;
        let before = snapshot(&fx.app)?;

        if progress.through != Some(head) {
            return Err(format!("built through {:?}, the head is {head}", progress.through).into());
        }
        if before.totals() != expected(&seed) || before.moves.len() != seed.len() {
            return Err(format!("the build wrote {before:?} for {seed:?}").into());
        }
        Ok(Built { seed, head, before })
    }

    // -----------------------------------------------------------------------
    // The rules
    // -----------------------------------------------------------------------

    /// Build, reset, rebuild: the rows and the checkpoint come back exactly.
    ///
    /// Run with the chunk sizes both ways round, so chunk invariance is shown
    /// across two tables and not only against the port's one probe table.
    ///
    /// Rejects: a reset that clears the checkpoint and one table (`movement`
    /// left behind fails the rebuild's commit on its key; `depot_stock` left
    /// behind doubles every total); a reset that leaves the checkpoint at the
    /// first position rather than `NeverRun` (that event is never re-applied,
    /// and the totals differ); and a reset of `view.id()` instead of
    /// `checkpoint_id(&view)` (the real checkpoint survives, the rebuild
    /// applies nothing into empty tables, and returns `Ok`).
    #[tokio::test]
    async fn rebuild_through_reset_restores_every_table() -> TestResult {
        for (built_at, rebuilt_at) in [(1, CHUNK), (CHUNK, 1)] {
            let fx = Fixture::open(&format!("restores-{built_at}-{rebuilt_at}"))?;
            let mut view = DepotView::new();
            let key = checkpoint_id(&view)?;
            let built = build(&fx, &mut view, built_at).await?;

            clear(&fx.models, &key, &[]).await?;
            assert_eq!(snapshot(&fx.app)?, Snapshot::default());
            assert_eq!(fx.models.checkpoint(&key).await?, Checkpoint::NeverRun);

            let rebuilt = fx.run(&mut view, rebuilt_at).await?;
            let after = snapshot(&fx.app)?;
            assert_eq!(
                unrestored(&built.before, &after, rebuilt, built.head, built.seed.len()),
                Vec::<&str>::new(),
                "built at {built_at}, rebuilt at {rebuilt_at}"
            );
            assert_eq!(
                fx.models.checkpoint(&key).await?,
                Checkpoint::Live {
                    through: built.head
                }
            );
        }
        Ok(())
    }

    /// The runbook procedure fails the predicate the rebuild passes.
    ///
    /// Norvant's night desk: the tables are emptied on the application's own
    /// connection, the process dies before anything touches the checkpoint,
    /// and the restarted runner resumes past it. It reports success, having
    /// applied nothing into empty tables. This is the wrong implementation
    /// `unrestored` exists to reject, so it is written down and shown to fail
    /// it on all three counts; without it, the rule above could be passing
    /// because the predicate is vacuous.
    #[tokio::test]
    async fn the_runbook_procedure_is_caught() -> TestResult {
        let fx = Fixture::open("runbook")?;
        let mut view = DepotView::new();
        let built = build(&fx, &mut view, CHUNK).await?;

        for statement in CLEAR_EVERY_TABLE {
            fx.app.execute(statement, [])?;
        }
        let resumed = fx.run(&mut view, CHUNK).await?;

        let after = snapshot(&fx.app)?;
        assert_eq!(
            unrestored(&built.before, &after, resumed, built.head, built.seed.len()),
            ["rows", "through", "applied"]
        );
        Ok(())
    }

    /// A reset whose batch fails part-way leaves both tables and the
    /// checkpoint as they were.
    ///
    /// The two real deletes come first and succeed; the third names a table
    /// that does not exist. Rejects: a reset that runs its statements outside
    /// one transaction, which has emptied both tables by the time the third
    /// fails, and leaves a view a later run will not refill.
    #[tokio::test]
    async fn failed_reset_leaves_every_table_and_the_checkpoint() -> TestResult {
        let fx = Fixture::open("failed-reset")?;
        let mut view = DepotView::new();
        let key = checkpoint_id(&view)?;
        let built = build(&fx, &mut view, CHUNK).await?;

        let outcome = clear(&fx.models, &key, &["DELETE FROM no_such_table"]).await;

        match &outcome {
            Err(ResetError::Store(SqliteProjectionStoreError::Sqlite(
                rusqlite::Error::SqliteFailure(_, Some(message)),
            ))) if message.contains("no_such_table") => {}
            other => panic!("expected the third statement's failure, got {other:?}"),
        }
        assert_eq!(snapshot(&fx.app)?, built.before);
        assert_eq!(
            fx.models.checkpoint(&key).await?,
            Checkpoint::Live {
                through: built.head
            }
        );
        let idle = fx.run(&mut view, CHUNK).await?;
        assert_eq!((idle.through, idle.applied), (None, 0));
        Ok(())
    }

    /// A protected view refuses its reset, keeps its rows and its checkpoint,
    /// and goes on running from where it was.
    ///
    /// The reset goes through `late`, a handle opened before the mark was
    /// written, so the refusal can only have come from the file. Rejects: a
    /// refusal reported as success (`NoopProtectFixture`'s shape: the tables
    /// are emptied); a store that runs the batch and then checks the mark (the
    /// tables are emptied and the checkpoint kept); and a mark held in the
    /// handle that wrote it, which `late` never sees. The second half rejects
    /// a protection that guards `commit` as well, which would stall the view.
    #[tokio::test]
    async fn protected_reset_is_refused_and_the_view_keeps_running() -> TestResult {
        let fx = Fixture::open("protected")?;
        let mut view = DepotView::new();
        let key = checkpoint_id(&view)?;
        let built = build(&fx, &mut view, CHUNK).await?;
        let late = fx.second_models()?;

        fx.models.protect_from_reset(&key).await?;
        let outcome = clear(&late, &key, &[]).await;

        assert!(
            matches!(outcome, Err(ResetError::Refused)),
            "expected a refusal, got {outcome:?}"
        );
        assert_eq!(snapshot(&fx.app)?, built.before);
        assert_eq!(
            late.checkpoint(&key).await?,
            Checkpoint::Live {
                through: built.head
            }
        );

        let more = vec![
            Movement::received("south", 5),
            Movement::shipped("north", 2),
        ];
        fx.append(&more).await?;
        let head = fx.head().await?;
        let resumed = fx.run(&mut view, CHUNK).await?;

        assert!(head > built.head, "the second append did not move the head");
        assert_eq!(resumed.through, Some(head));
        assert_eq!(resumed.applied, more.len());
        let every = [built.seed, more].concat();
        let after = snapshot(&fx.app)?;
        assert_eq!(after.totals(), expected(&every));
        assert_eq!(after.moves.len(), every.len());
        Ok(())
    }

    /// Lifting the mark is what lets the rebuild through.
    ///
    /// Rejects: an `allow_reset` that does not remove the mark, which answers
    /// this reset `Refused`.
    #[tokio::test]
    async fn allowed_reset_rebuilds_the_protected_view() -> TestResult {
        let fx = Fixture::open("allowed")?;
        let mut view = DepotView::new();
        let key = checkpoint_id(&view)?;
        let built = build(&fx, &mut view, CHUNK).await?;
        fx.models.protect_from_reset(&key).await?;

        fx.models.allow_reset(&key).await?;
        clear(&fx.models, &key, &[]).await?;

        assert_eq!(snapshot(&fx.app)?, Snapshot::default());
        assert_eq!(fx.models.checkpoint(&key).await?, Checkpoint::NeverRun);
        let rebuilt = fx.run(&mut view, CHUNK).await?;
        let after = snapshot(&fx.app)?;
        assert_eq!(
            unrestored(&built.before, &after, rebuilt, built.head, built.seed.len()),
            Vec::<&str>::new()
        );
        Ok(())
    }
}
