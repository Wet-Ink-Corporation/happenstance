//! What the library does to the heap, in counts rather than in nanoseconds.
//!
//! # Why the counts are the quotable half
//!
//! `references/evaluation/review-pre-publication-2026-09-03.md:2836`, on the
//! host this suite runs on:
//!
//! > Quote the allocation columns and not the timings. The counts are identical
//! > to the digit across four separate runs; the wall-clock medians moved by up
//! > to 40% between runs on this host.
//!
//! Every figure this binary prints is reproducible exactly. That makes it the
//! right instrument for questions where a 40% swing would swallow the answer —
//! and it is what makes `results/history/` able to detect a regression that a
//! timing could not distinguish from a busy afternoon.
//!
//! # This binary installs the global allocator; nothing else does
//!
//! A `#[global_allocator]` is a per-binary singleton. Declaring one in
//! `src/lib.rs` — which is what both `experiments/event-clone-allocations` and
//! `experiments/one-connection-latency` do — would install it into every
//! `criterion` target too, and every timing in this crate would then carry an
//! atomic increment per heap operation. So the type is public,
//! `happenstance_benchmarks::counting::Counting`, and the attribute is here.
//!
//! # Single-threaded, and `run.sh` enforces it
//!
//! The counters are process-global rather than per-thread, so an allocation on
//! any other live thread lands in whatever region happens to be open. This
//! binary is single-threaded throughout and the shared runtime is
//! `current_thread`; the one place that is not enough is
//! `SqliteEventStore::read`, which hops each page to `spawn_blocking` — so the
//! SQLite read arm's counts include the pool thread's allocations, and it says
//! so where it is reported.
//!
//! # Usage
//!
//! ```console
//! cargo run --release --bin allocations
//! ```
//!
//! Writes CSV to stdout; `run.sh` tees it into `results/raw/allocations.csv`.
//! Nothing here asserts.

use std::hint::black_box;

use happenstance::{Cbor, DomainEvent, Json, Postcard};
use happenstance_benchmarks::corpus::{
    Corpus, FLOOR_EVENTS_PER_BATCH, FLOOR_TAGS_PER_EVENT, Regime, Shape,
};
use happenstance_benchmarks::counting::{self, Counting, Region};
use happenstance_benchmarks::domain::Recorded;
use happenstance_benchmarks::fixtures::memory::MemoryFixture;
use happenstance_benchmarks::report::{Row, RunRecord};
use happenstance_benchmarks::runtime;
use happenstance_core::{EventStore, Query, ReadOptions, collect};
use happenstance_testkit::Fixture;

/// The counting allocator, installed for this binary and nothing else.
#[global_allocator]
static ALLOCATOR: Counting = Counting;

/// Tag counts swept, ending at VT-22's conformance floor.
const TAG_COUNTS: [usize; 4] = [0, 1, 8, FLOOR_TAGS_PER_EVENT];

/// Payload sizes the codec arms sweep.
const PAYLOAD_SIZES: [usize; 3] = [0, 256, 4_096];

fn main() {
    let mut record = RunRecord::new();
    eprintln!("{}\n", record.conditions);

    event_clone(&mut record);
    batch_construction(&mut record);
    codec(&mut record);
    memory_read(&mut record);

    print!("{}", record.to_csv());
}

/// Pushes one region's four figures as rows.
fn push_region(
    record: &mut RunRecord,
    group: &str,
    arm: &str,
    shape: &str,
    region: Region,
    representative: bool,
) {
    eprintln!("[{group}] {arm} {shape}: {region}");
    #[allow(clippy::cast_precision_loss)]
    for (metric, value, unit) in [
        ("heap-ops", region.counts.heap_ops() as f64, "ops"),
        ("allocs", region.counts.allocs as f64, "ops"),
        ("reallocs", region.counts.reallocs as f64, "ops"),
        ("bytes-requested", region.counts.bytes as f64, "bytes"),
        ("peak-live", region.peak_above_baseline as f64, "bytes"),
    ] {
        record.push(Row {
            group: group.to_owned(),
            arm: arm.to_owned(),
            shape: shape.to_owned(),
            metric: metric.to_owned(),
            value,
            unit: unit.to_owned(),
            representative,
        });
    }
}

/// **The ES-17 numerator: what one `Event::clone()` costs, in both regimes.**
///
/// ES-17 (`spec/SPECIFICATION.md:3345`) keeps `append`'s batch borrowed, and its
/// `[PROVISIONAL]` marker is falsified by *"a measurement on a real adapter
/// showing the per-event clone is a material fraction of append cost"*.
///
/// This is **not** that measurement, and saying so is the point.
/// `.kb/open-questions/es-17-two-adapter-measurement-is-unscheduled.md` records
/// what the falsifier actually asks for: *two builds of the same SQLite
/// adapter, differing only in `append`'s ownership of its batch, measured on
/// the same harness*. Nothing here is that, and nothing here lifts the marker.
///
/// What this supplies is the numerator whoever builds those two arms will need
/// — and the warning that comes with it. The two regimes are **66 heap
/// operations apart at the tag floor**, and the interned arm is *flat in tag
/// count*, so a benchmark author who reached for `Tag::from_static` constants
/// without choosing to would measure a regime with a 66× cheaper clone, report
/// the clone immaterial, and lift a frozen marker on evidence that could not
/// have gone the other way.
///
/// The first and second clones are both reported: a payload built with
/// `Bytes::from(Vec<u8>)` starts promotable and allocates a shared header on
/// its **first** clone only, so the two differ by one and quoting the wrong one
/// is an off-by-one in a published figure.
fn event_clone(record: &mut RunRecord) {
    for regime in Regime::BOTH {
        for tags in TAG_COUNTS {
            let corpus = Corpus::uniform(Shape::new(1_024, tags, regime));
            let event = corpus.event(0);
            let shape = corpus.shape().label();

            let (warm, first) = counting::measure(|| event.clone());
            push_region(
                record,
                "clone/event-first",
                regime.label(),
                &shape,
                first,
                regime.is_representative(),
            );

            let (_kept, second) = counting::measure(|| warm.clone());
            push_region(
                record,
                "clone/event-steady-state",
                regime.label(),
                &shape,
                second,
                regime.is_representative(),
            );
        }
    }
}

/// What building a batch at VT-24's floor costs, and what cloning it costs.
///
/// The denominator half of the same question: `append` borrows its batch, so a
/// caller who wants to retry keeps their own copy — and this is what that copy
/// costs at the conformance floors, where a store must accept 128 events and 64
/// tags each.
fn batch_construction(record: &mut RunRecord) {
    for regime in Regime::BOTH {
        for tags in [3_usize, FLOOR_TAGS_PER_EVENT] {
            let corpus = Corpus::uniform(Shape::new(1_024, tags, regime));
            let shape = format!("{}/batch-{FLOOR_EVENTS_PER_BATCH}", corpus.shape().label());

            let (batch, built) = counting::measure(|| corpus.batch(FLOOR_EVENTS_PER_BATCH));
            push_region(
                record,
                "batch/build",
                regime.label(),
                &shape,
                built,
                regime.is_representative(),
            );

            let (_kept, cloned) = counting::measure(|| batch.clone());
            push_region(
                record,
                "batch/clone",
                regime.label(),
                &shape,
                cloned,
                regime.is_representative(),
            );
        }
    }
}

/// The encode and decode paths, per codec.
///
/// `references/evaluation/review-pre-publication-2026-09-03.md:2756` measured
/// the *envelope* path — the wire format `happenstance-sync` uses — at **140
/// heap operations to produce 587 bytes, 93% of them transient clones that
/// produce no output**. This is the *application payload* path,
/// `DomainEvent::encode`/`::decode`, which is what `commit` and
/// `run_projection` call on every event. Different code, and the one every
/// application pays.
///
/// Encoded size is reported beside the operation count, because a codec that
/// allocates less and produces more bytes has moved the cost into the store
/// rather than removed it.
fn codec(record: &mut RunRecord) {
    for payload in PAYLOAD_SIZES {
        let event = Recorded::new("a1", 7, payload);
        let shape = format!("owned/{payload}B");

        macro_rules! codec_arm {
            ($name:literal, $codec:expr) => {{
                let (encoded, encode_cost) =
                    counting::measure(|| event.encode(&$codec).expect("encoding succeeds"));
                eprintln!("[codec/encode] {} {shape} -> {}B", $name, encoded.len());
                push_region(record, "codec/encode", $name, &shape, encode_cost, true);

                let event_type = event.event_type();
                let (_decoded, decode_cost) = counting::measure(|| {
                    <Recorded as DomainEvent>::decode(&$codec, &event_type, &encoded)
                        .expect("decoding succeeds")
                });
                push_region(record, "codec/decode", $name, &shape, decode_cost, true);

                #[allow(clippy::cast_precision_loss)]
                record.push(Row {
                    group: "codec/encoded-size".to_owned(),
                    arm: $name.to_owned(),
                    shape: shape.clone(),
                    metric: "bytes".to_owned(),
                    value: encoded.len() as f64,
                    unit: "bytes".to_owned(),
                    representative: true,
                });
            }};
        }

        codec_arm!("json", Json);
        codec_arm!("cbor", Cbor);
        codec_arm!("postcard", Postcard);
    }
}

/// **The H2 arm: `limit(1)` against `limit(None)` on the memory store.**
///
/// `MemoryEventStore::read` materialises the whole matched set into a
/// `Vec<SequencedEvent>` — cloning every event — before `limit` truncates it
/// (`crates/happenstance-core/src/memory.rs:296-336`).
/// `references/evaluation/review-pre-publication-2026-09-03.md:2792` measured
/// the two at **1.0000× to four significant figures** on a million events:
/// asking for one event out of a million costs what asking for all of them
/// costs, at 4,000,020 heap operations and 229,995,520 bytes.
///
/// The log here is ten thousand rather than a million — the shape shows at any
/// length, and a million-event seed would put minutes into `run.sh` for a ratio
/// that does not move. What the arm is watching for is the ratio departing from
/// 1.0, which is what a repair would look like.
///
/// The store documents itself as not built for scale, and nobody is entitled to
/// be surprised that it is slow. That is not what is reported here: what is
/// reported is that its snapshot is priced as *cheap* and its `limit` as a
/// *reduction*, and that neither is true.
fn memory_read(record: &mut RunRecord) {
    const LOG: usize = 10_000;

    let corpus = Corpus::distinct(Shape::new(256, 3, Regime::Owned));
    let store = runtime::block_on(async {
        let fixture = MemoryFixture::new();
        let store = fixture.connect().await;
        let mut written = 0;
        while written < LOG {
            let take = 128.min(LOG - written);
            let chunk: Vec<_> = (written..written + take).map(|n| corpus.event(n)).collect();
            store
                .append(&chunk, None)
                .await
                .expect("MemoryEventStore's error type is uninhabited");
            written += take;
        }
        store
    });

    let shape = format!("owned/256B/3tags/{LOG}-events");

    for (name, options) in [
        ("limit-none", ReadOptions::new()),
        ("limit-1", ReadOptions::new().limit(1)),
        ("backwards-limit-1", ReadOptions::new().backwards().limit(1)),
    ] {
        let (events, region) = counting::measure(|| {
            runtime::block_on(collect(store.read(&Query::all(), options)))
                .expect("the read succeeds")
        });
        eprintln!("[read/memory] {name} returned {} events", events.len());
        push_region(record, "read/memory", name, &shape, region, true);
        black_box(events);
    }

    // And `head()`, the method ES-30 requires so that the composed spelling
    // above is never the only way to ask for the top of the log.
    let (position, region) = counting::measure(|| {
        runtime::block_on(store.head()).expect("MemoryEventStore's error type is uninhabited")
    });
    black_box(position);
    push_region(record, "read/memory", "head", &shape, region, true);
}
