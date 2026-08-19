//! The tag-storage arms, measured where the decision is actually paid: a probe
//! held under the `BEGIN IMMEDIATE` write lock, against a **populated** store.
//!
//! # Why this control exists, given that the harness ran verbatim
//!
//! `event_store_benchmarks!`'s three scenarios are the workloads phase 8 asked
//! for and they all ran (`tests/measure.rs`). None of them, though, evaluates a
//! condition against a store that already holds tens of thousands of events —
//! the contended scenario races against an empty one, and the replay scenario's
//! single timer covers seeding as well as the two reads, so the arm that writes
//! *more* rows per event looks slower on a figure a reader will take for a read
//! cost. Both are honest properties of the harness rather than defects, and both
//! are recorded as such.
//!
//! AC-003 nevertheless asks for all three tag storages measured *under a probe
//! held under the write lock*. No scenario does that, so the measurement is the
//! **caller's** — which is precisely the seam CF-23 makes a parameter and
//! `bench.rs`'s own module docs delegate: the harness defines the shared
//! workloads, the caller's emitter measures what its own decision needs. It is
//! labelled a caller-side control everywhere it is quoted and replaces no
//! harness figure.
//!
//! # Why one test and not three
//!
//! The arms are measured **round-robin inside one process**, one operation each
//! per round, rather than one arm at a time. Three sequential tests were written
//! first and were thrown away: two runs an hour apart disagreed about the
//! *ordering* of the filtered read, because the host slowed down partway through
//! and the arm that happened to run late wore it. Interleaving makes a machine
//! that gets busy affect all three equally, which is the only thing that makes a
//! ratio between them mean anything.
//!
//! Every store is checkpointed after seeding and before anything is timed, so a
//! read figure is not a function of how long a write-ahead log happens to be.

mod support;

use append_condition_probes::{
    BeginImmediateProbe, CandidateStore, CanonicalBlob, JoinTable, JoinTableGrouped, Json1,
    TagStorage,
};
use happenstance_core::{Event, EventStore, Query, ReadOptions, collect};
use happenstance_testkit::block_on;
use happenstance_testkit::fixtures::{condition_after, query_of, tagged_event};
use support::CandidateFixture;

/// Events seeded before anything is timed.
///
/// Large enough that an unindexed arm has to work: at 20,000 events a full scan
/// is a real scan rather than a page the cache was going to hold anyway.
const SEED: usize = 50_000;

/// Events per seeding append. Well under the 128 every store must accept.
const CHUNK: usize = 100;

/// Rounds. Each round times one of every operation against each of the three
/// arms, in turn.
const ROUNDS: usize = 25;

/// One event in this many carries the tag the probe and the filtered read
/// select on.
const HOT_IN: usize = 3;

/// What one arm cost, in microseconds, across every round.
#[derive(Debug, Default)]
struct Samples {
    conditional_append: Vec<u128>,
    unconditional_append: Vec<u128>,
    selective_read: Vec<u128>,
    broad_read: Vec<u128>,
    unfiltered_read: Vec<u128>,
    selective_matched: usize,
    broad_matched: usize,
    total: usize,
}

impl Samples {
    fn median(values: &mut [u128]) -> u128 {
        values.sort_unstable();
        values[values.len() / 2]
    }

    fn report(&mut self, arm: &str, conditions: &str) {
        let conditional = Self::median(&mut self.conditional_append);
        let unconditional = Self::median(&mut self.unconditional_append);
        println!(
            "TAGSTORE\t{arm}\tconditional_append_us={conditional}\t\
             unconditional_append_us={unconditional}\tprobe_cost_us={}\t\
             selective_read_us={}\tbroad_read_us={}\tunfiltered_read_us={}\t\
             selective_matched={}\tbroad_matched={}\ttotal={}\t\
             seeded={SEED}\trounds={ROUNDS}\t{conditions}",
            conditional.saturating_sub(unconditional),
            Self::median(&mut self.selective_read),
            Self::median(&mut self.broad_read),
            Self::median(&mut self.unfiltered_read),
            self.selective_matched,
            self.broad_matched,
            self.total,
        );
    }
}

/// Seeds one arm's store and hands it back, checkpointed and ready to time.
fn seeded_store<T: TagStorage>(
    fixture: &CandidateFixture<BeginImmediateProbe, T>,
) -> CandidateStore<BeginImmediateProbe, T> {
    let store = fixture.open();
    let mut written = 0usize;
    while written < SEED {
        let take = CHUNK.min(SEED - written);
        let batch: Vec<Event> = (0..take).map(|offset| seeded(written + offset)).collect();
        block_on(store.append(&batch, None)).expect("seeding must succeed");
        written += take;
    }
    store
}

/// Times one round against one arm.
fn round<T: TagStorage>(
    store: &CandidateStore<BeginImmediateProbe, T>,
    selective: &Query,
    broad: &Query,
    index: usize,
    samples: &mut Samples,
) {
    // The probe: a conditional append whose guard is re-anchored at the current
    // head, so every round measures the *happy* path — the guard finds nothing
    // after its boundary and the batch lands. That is what the adapter runs on
    // every uncontended write, and the case the decision is about.
    let head = block_on(store.head())
        .expect("head must succeed")
        .expect("the store is not empty");
    let condition = condition_after(selective.clone(), head.get());
    let event = seeded(SEED + index * 2);

    let started = std::time::Instant::now();
    block_on(store.append(core::slice::from_ref(&event), Some(&condition)))
        .expect("the guard is anchored at head, so nothing can be after it");
    samples
        .conditional_append
        .push(started.elapsed().as_micros());

    // The baseline the probe is read against: the same single-event append with
    // no condition at all. Without it the probe figure is a transaction commit
    // with a probe somewhere inside it and a reader cannot tell which of the two
    // moved. `conditional - unconditional` is the number the decision is about.
    let event = seeded(SEED + index * 2 + 1);
    let started = std::time::Instant::now();
    block_on(store.append(core::slice::from_ref(&event), None))
        .expect("an unconditional append must succeed");
    samples
        .unconditional_append
        .push(started.elapsed().as_micros());

    // The **selective** read, and it is the one the decision turns on. A
    // consistency-boundary query matches a handful of events out of a log, so
    // the cost is dominated by *finding* them rather than by materialising
    // them — which is the half a tag storage decides. The broad read below is
    // the opposite shape and is here so the record can say which regime a
    // figure came from.
    let started = std::time::Instant::now();
    let events =
        block_on(collect(store.read(selective, ReadOptions::new()))).expect("read must succeed");
    samples.selective_read.push(started.elapsed().as_micros());
    samples.selective_matched = events.len();

    // The broad read: a third of the log. Materialising `SequencedEvent`s
    // dominates here, and materialisation is identical on all three arms, so
    // this figure is expected to be flat — which is what makes it a check on
    // the instrument rather than a result.
    let started = std::time::Instant::now();
    let events =
        block_on(collect(store.read(broad, ReadOptions::new()))).expect("read must succeed");
    samples.broad_read.push(started.elapsed().as_micros());
    samples.broad_matched = events.len();

    // The control on the control: an unfiltered replay touches no tag storage
    // at all, so a large difference between arms here would mean the three runs
    // are not comparable and nothing else in the table may be read as a ratio.
    let started = std::time::Instant::now();
    let events = block_on(collect(store.read(&Query::all(), ReadOptions::new())))
        .expect("read must succeed");
    samples.unfiltered_read.push(started.elapsed().as_micros());
    samples.total = events.len();
}

/// One seeded event; one in [`HOT_IN`] carries the tag under test.
fn seeded(index: usize) -> Event {
    let shard = if index.is_multiple_of(HOT_IN) {
        "hot"
    } else {
        "cold"
    };
    // A second, high-cardinality tag, so the join table answers a multi-tag
    // question rather than a degenerate one and `tag_cardinality` has something
    // to be about.
    let row = format!("r{}", index % 97);
    tagged_event("Seeded", &[("shard", shard), ("row", &row)])
}

#[test]
fn the_three_tag_storages_under_one_probe() {
    let join_fixture = CandidateFixture::<BeginImmediateProbe, JoinTable>::new();
    let grouped_fixture = CandidateFixture::<BeginImmediateProbe, JoinTableGrouped>::new();
    let blob_fixture = CandidateFixture::<BeginImmediateProbe, CanonicalBlob>::new();
    let json_fixture = CandidateFixture::<BeginImmediateProbe, Json1>::new();

    let join = seeded_store(&join_fixture);
    let grouped = seeded_store(&grouped_fixture);
    let blob = seeded_store(&blob_fixture);
    let json = seeded_store(&json_fixture);

    // Fold every write-ahead log into its main database before anything is
    // timed. Without it the first arm measured reads a short WAL and the last
    // one reads a long one, which is a figure about the order the arms were
    // written in.
    for path in [
        join_fixture.path(),
        grouped_fixture.path(),
        blob_fixture.path(),
        json_fixture.path(),
    ] {
        let connection = rusqlite::Connection::open(path).expect("checkpointing connection");
        connection
            .execute_batch("PRAGMA wal_checkpoint(TRUNCATE); ANALYZE;")
            .expect("checkpoint and analyze");
    }

    // Selective: one high-cardinality tag, matching about one event in 97.
    // Broad: one low-cardinality tag, matching about one in three.
    let selective = query_of(&["Seeded"], &[("row", "r7")]);
    let broad = query_of(&["Seeded"], &[("shard", "hot")]);
    let mut join_samples = Samples::default();
    let mut grouped_samples = Samples::default();
    let mut blob_samples = Samples::default();
    let mut json_samples = Samples::default();

    for index in 0..ROUNDS {
        round(&join, &selective, &broad, index, &mut join_samples);
        round(&grouped, &selective, &broad, index, &mut grouped_samples);
        round(&blob, &selective, &broad, index, &mut blob_samples);
        round(&json, &selective, &broad, index, &mut json_samples);
    }

    let conditions = join.durability().conditions();
    join_samples.report("join-table", &conditions);
    grouped_samples.report("join-table-grouped", &conditions);
    blob_samples.report("canonical-blob", &conditions);
    json_samples.report("json1", &conditions);

    assert!(
        join_samples.selective_matched > 0
            && join_samples.selective_matched < join_samples.broad_matched
            && join_samples.broad_matched < join_samples.total,
        "the two filtered reads must select proper subsets, and the selective \
         one a smaller subset than the broad one — {} < {} < {} is what makes \
         the pair a comparison rather than one number twice",
        join_samples.selective_matched,
        join_samples.broad_matched,
        join_samples.total
    );
    assert_eq!(
        (
            blob_samples.selective_matched,
            json_samples.selective_matched
        ),
        (
            join_samples.selective_matched,
            join_samples.selective_matched
        ),
        "the three arms must agree about what matched, or they are not three \
         encodings of one question"
    );
}
