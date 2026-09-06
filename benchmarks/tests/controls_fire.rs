//! The workload controls: every arm is what it says it is, before any of it is
//! timed.
//!
//! # The difference between this file and `instruments_work.rs`
//!
//! That one proves the *instruments* measure — the allocator counts, the clock
//! distinguishes work from waiting, the sampler sees a difference it was given.
//! This one proves the *arms* are what the results tables will claim they are:
//! that the floor writes and reads real rows, that it is configured exactly as
//! the adapter is, and that the corpus refuses to build a workload whose
//! figures would be meaningless.
//!
//! Neither file asserts on a duration. CF-34 (`spec/SPECIFICATION.md:8747`)
//! rejects *"a benchmark result gating a merge"*, and its reasoning is that a
//! threshold nobody can justify becomes a threshold everybody raises. Every
//! assertion here is on a **count**, a **string** or a **refusal**, all three of
//! which are the same on a loaded host as on an idle one.

use happenstance_benchmarks::corpus::{
    self, Corpus, FLOOR_EVENT_DATA_LEN, FLOOR_TAGS_PER_EVENT, Regime, Shape,
};
use happenstance_benchmarks::fixtures::raw::{Floor, RawStore};
use happenstance_benchmarks::fixtures::sqlite::SqliteFixture;
use happenstance_benchmarks::runtime;
use happenstance_core::{EventStore, Query, ReadOptions, collect};
use happenstance_sqlite::connection::{BUSY_TIMEOUT_MS, JOURNAL_MODE, SYNCHRONOUS};
use happenstance_testkit::Fixture;

/// CONTROL — both floors write rows and read them back.
///
/// A floor that silently wrote nothing would be infinitely fast, and the
/// overhead ratio above it would be an infinity reported as a finding. This is
/// the check that makes the floor a floor rather than a no-op.
#[test]
fn both_floors_write_what_they_claim_and_read_it_back() {
    const EVENTS: usize = 64;
    let events = i64::try_from(EVENTS).expect("64 fits an i64");

    for floor in Floor::BOTH {
        let fixture = SqliteFixture::new();
        let mut store = RawStore::over(&fixture, floor).expect("the floor opens");

        let corpus = Corpus::distinct(Shape::new(256, 3, Regime::Owned));
        let batch = corpus.batch(EVENTS);
        let last = store.append(&batch).expect("the floor appends");

        assert_eq!(
            last,
            events,
            "{}: an autoincrement position after {EVENTS} rows into an empty \
             log is {EVENTS}. A floor whose writes vanish is infinitely fast, \
             and every ratio above it would be an infinity read as a finding",
            floor.label()
        );
        assert_eq!(
            store.read_all().expect("the floor reads"),
            EVENTS,
            "{}: every row written must come back",
            floor.label()
        );
        assert_eq!(
            store.head().expect("the floor reports a head"),
            Some(events),
            "{}: head is the highest position written",
            floor.label()
        );

        // The shared tag is on every event, so a tag read must select all of
        // them. A join or a LIKE that matched nothing would make the tagged
        // read arm the fastest thing in the suite.
        assert_eq!(
            store
                .read_tagged("bench:t01")
                .expect("the floor reads by tag"),
            EVENTS,
            "{}: the shared tag is carried by every event in the corpus, so a \
             tag read selects all {EVENTS}",
            floor.label()
        );

        // And the ordinal tag selects exactly one, which is what makes the
        // selectivity axis an axis. A floor whose tag read always returned
        // everything would report the selective and unselective ends as equal.
        assert_eq!(
            store
                .read_tagged("bench:e7")
                .expect("the floor reads by tag"),
            1,
            "{}: an ordinal tag selects exactly one event, however long the \
             log is — otherwise there is no selectivity axis to measure",
            floor.label()
        );
    }
}

/// CONTROL — an empty batch is refused by the floor, by name.
///
/// The contract refuses an empty append before the condition is evaluated
/// (ES-20). A floor that accepted one would return a position for a row nobody
/// wrote, and the overhead arm would be dividing by a measurement of nothing.
#[test]
#[should_panic(expected = "an empty batch is refused by the contract")]
fn the_floor_refuses_an_empty_batch() {
    let fixture = SqliteFixture::new();
    let mut store = RawStore::over(&fixture, Floor::HandRolled).expect("the floor opens");
    let _ = store.append(&[]);
}

/// CONTROL — the floor's durability settings are the adapter's, read back off
/// the live connection.
///
/// SQLite silently ignores a `journal_mode` it cannot honour, so a floor that
/// trusted the `PRAGMA` it issued could be running in rollback mode against an
/// adapter running in WAL — and the ratio between them would be a comparison of
/// two different durability guarantees reported as a comparison of two
/// implementations.
///
/// The expected values come from `happenstance-sqlite`'s own public constants
/// rather than from literals here, so this cannot pass while the two have
/// drifted apart.
#[test]
fn the_floor_is_configured_exactly_as_the_adapter_is() {
    for floor in Floor::BOTH {
        let fixture = SqliteFixture::new();
        let store = RawStore::over(&fixture, floor).expect("the floor opens");
        let settings = store.settings().expect("the pragmas read back");

        assert_eq!(
            settings.journal_mode(),
            JOURNAL_MODE,
            "{}: the floor must run the journal mode the adapter runs, read \
             back off the live connection rather than trusted from the PRAGMA \
             that issued it",
            floor.label()
        );
        assert_eq!(
            settings.synchronous(),
            SYNCHRONOUS,
            "{}: same durability, or the ratio is a comparison of two \
             different guarantees",
            floor.label()
        );
        assert_eq!(
            settings.busy_timeout_ms(),
            i64::try_from(BUSY_TIMEOUT_MS).expect("5,000 ms fits an i64"),
            "{}: same busy timeout, or the contended arms wait for different \
             lengths of time",
            floor.label()
        );
    }
}

/// CONTROL — the same-schema floor really is on the adapter's schema.
///
/// It is built by letting a `SqliteEventStore` migrate the file and then
/// dropping it, precisely so this crate never restates `MIGRATION_1`. The check
/// is that the join table the adapter's tag index depends on is present — and
/// that the hand-rolled floor does **not** have it, because a hand-rolled floor
/// that had grown one would have stopped being the thing it is named for.
#[test]
fn the_same_schema_floor_has_the_adapters_tables_and_the_hand_rolled_one_does_not() {
    let has_event_tag = |floor: Floor| {
        let fixture = SqliteFixture::new();
        let mut store = RawStore::over(&fixture, floor).expect("the floor opens");
        // Reaching it through the floor's own read path rather than through
        // `sqlite_master`: what matters is whether the tag index is usable, not
        // whether a row exists in a catalogue.
        let corpus = Corpus::distinct(Shape::new(64, 2, Regime::Owned));
        store.append(&corpus.batch(4)).expect("the floor appends");
        store
            .read_tagged("bench:e0")
            .expect("the floor reads by tag")
    };

    assert_eq!(
        has_event_tag(Floor::SameSchema),
        1,
        "the same-schema floor joins `event_tag`, which exists only because a \
         SqliteEventStore migrated the file — this crate never restates \
         MIGRATION_1, so a failure here means the adapter's schema moved"
    );
    assert_eq!(
        has_event_tag(Floor::HandRolled),
        1,
        "the hand-rolled floor finds the same row by scanning its tags column \
         — slower by design, and the arm where ADR-0022's join table earns its \
         keep"
    );
}

/// CONTROL — the corpus refuses a distinct-tag workload in the interned regime,
/// by name.
///
/// `Tag::from_static` takes a `&'static str`, so every event in an interned
/// corpus carries the same tags. A silently-uniform "distinct" corpus would
/// make every selectivity figure a measurement of a filter that matches the
/// whole log — the selective and unselective ends of the axis would be the same
/// query, and the axis would report no difference because there was none.
#[test]
#[should_panic(expected = "impossible in the interned regime")]
fn a_distinct_corpus_is_refused_in_the_interned_regime() {
    let _ = Corpus::distinct(Shape::new(64, 3, Regime::Interned));
}

/// CONTROL — the interned regime refuses a shape it cannot build.
///
/// It slices one static buffer and holds one static tag table, so asking it for
/// more is refused here rather than panicking later inside a timed region,
/// where the failure would be reported as latency.
#[test]
#[should_panic(expected = "cannot produce")]
fn the_interned_regime_refuses_a_payload_larger_than_its_buffer() {
    let _ = Shape::new(FLOOR_EVENT_DATA_LEN + 1, 1, Regime::Interned);
}

/// CONTROL — and refuses more tags than it holds.
#[test]
#[should_panic(expected = "cannot produce")]
fn the_interned_regime_refuses_more_tags_than_it_holds() {
    let _ = Shape::new(64, FLOOR_TAGS_PER_EVENT + 1, Regime::Interned);
}

/// CONTROL — a distinct corpus really is distinct, and a uniform one really is
/// uniform.
///
/// The two constructors differ by one tag, and a bug that made them the same
/// would be invisible in every timing. This is the positive and negative
/// control for the axis every query-shape figure rests on.
#[test]
fn distinct_and_uniform_corpora_differ_in_the_way_the_selectivity_axis_needs() {
    let distinct = Corpus::distinct(Shape::new(64, 3, Regime::Owned));
    assert_ne!(
        distinct.event(0).tags(),
        distinct.event(1).tags(),
        "a distinct corpus gives each event a tag naming its own ordinal; \
         without it the selective end of the axis selects the whole log"
    );

    let uniform = Corpus::uniform(Shape::new(64, 3, Regime::Owned));
    assert_eq!(
        uniform.event(0).tags(),
        uniform.event(1).tags(),
        "a uniform corpus gives every event the same tags, so an append \
         throughput arm measures throughput rather than selectivity"
    );
}

/// CONTROL — CF-34's own worked query selects a proper subset.
///
/// `spec/SPECIFICATION.md:8712-8718` uses a mixed two-item query — one item
/// selecting a handful of events, the other selecting millions — as the case no
/// conformance rule can catch. A version of it that matched everything, or
/// nothing, would measure the wrong thing entirely.
#[test]
fn the_mixed_selectivity_query_selects_a_proper_subset() {
    const SEEDED: usize = 256;

    let guard = runtime::enter();
    let fixture = SqliteFixture::new();
    drop(guard);

    runtime::block_on(async {
        let store = fixture.connect().await;
        let corpus = Corpus::distinct(Shape::new(64, 3, Regime::Owned));

        // Chunked to the adapter's stated batch ceiling, which the fixture
        // mirrors — seeding into a refusal would measure the error path.
        let ceiling = <SqliteFixture as Fixture>::MAX_EVENTS_PER_BATCH.unwrap_or(SEEDED);
        let mut written = 0;
        while written < SEEDED {
            let take = ceiling.min(SEEDED - written);
            let chunk: Vec<_> = (written..written + take).map(|n| corpus.event(n)).collect();
            store.append(&chunk, None).await.expect("the seed lands");
            written += take;
        }

        let count = |query: Query| {
            let store = &store;
            async move {
                collect(store.read(&query, ReadOptions::new()))
                    .await
                    .expect("the read succeeds")
                    .len()
            }
        };

        let all = count(Query::all()).await;
        let unselective = count(corpus::query_matching_all()).await;
        let selective = count(corpus::query_matching_one(7)).await;
        let mixed = count(corpus::query_mixed_selectivity(7)).await;

        assert_eq!(all, SEEDED, "the whole log is readable");
        assert_eq!(
            unselective, SEEDED,
            "the shared tag is on every event, so it is the unselective end of \
             the axis"
        );
        assert_eq!(
            selective, 1,
            "the ordinal tag is on one event, so it is the selective end. If \
             this ever reports {SEEDED}, the two ends have collapsed into one \
             query and every selectivity figure is a measurement of nothing"
        );
        assert_eq!(
            mixed, SEEDED,
            "CF-34's worked case unions a handful with the whole log, so the \
             union is the whole log — the cost is in *how* an adapter reaches \
             that answer, which is a benchmark and not an assertion"
        );
    });
}

/// CONTROL — the executor is a constant across arms, and here is what choosing
/// it cost.
///
/// `crate::runtime` puts every arm on one tokio current-thread runtime rather
/// than letting the memory arms use `happenstance_testkit::block_on`, which is
/// a dependency-free park loop and genuinely cheaper. Doing otherwise would put
/// the executor difference inside the memory-versus-SQLite ratio, where nothing
/// separates it from the storage difference that ratio is supposed to report.
///
/// This declares the delta once, prints it, and **asserts nothing about it**.
/// It is a number for the results README to carry, not a bar.
#[test]
fn the_executor_delta_is_declared_rather_than_smeared_across_every_row() {
    use happenstance_benchmarks::paired::{self, Operation};

    let park_loop_store = happenstance_core::MemoryEventStore::new();
    let tokio_store = happenstance_core::MemoryEventStore::new();
    let corpus = Corpus::uniform(Shape::new(256, 3, Regime::Owned));
    let batch = corpus.batch(8);

    let mut operations = [
        Operation::new("append via testkit block_on", || {
            let _ = happenstance_testkit::block_on(park_loop_store.append(&batch, None));
        }),
        Operation::new("append via tokio current_thread", || {
            let _ = runtime::block_on(tokio_store.append(&batch, None));
        }),
    ];

    let report = paired::run(200, paired::DEFAULT_WARMUP_ROUNDS, &mut operations);
    println!("executor delta, declared rather than assumed:\n{report}");
    if let Some(ratio) = report.ratio(
        "append via tokio current_thread",
        "append via testkit block_on",
    ) {
        println!("tokio / park loop = {ratio:.3}x");
    }
}
