//! The command loop: what one decision costs, and why it is not one append.
//!
//! # What `commit` actually does, per attempt
//!
//! `crates/happenstance/src/command.rs:280-310`, in order: clone the boundary,
//! derive its query, `read_decision_model` — which **collects the whole matched
//! set into a `Vec`** — decode *every* matched event into the application's
//! enum, fold them, call the caller's `decide`, encode what comes back, and
//! append it under an `AppendCondition` anchored at the position the read saw.
//!
//! So **one `commit` is a full replay of its consistency boundary plus one
//! decode per event in it**. An application that writes a thousand events
//! against one aggregate and never snapshots pays a thousand decodes on the
//! thousand-and-first command. That is a property of the design rather than a
//! defect — DCB has no aggregate to snapshot *into* — but it is the number an
//! adopter needs and nobody in this repository has taken it.
//!
//! # Every measured iteration starts from the same boundary, and that took work
//!
//! A command **appends**. The first version of this file seeded a boundary once
//! per sample and then timed `iters` commits against it — so the boundary grew
//! *during* the measured block, and the reported figure was the cost at
//! `history + iters/2` rather than at `history`.
//!
//! That is not a rounding error; it destroyed the signal the group exists to
//! report. The memory arm read **664 µs at a boundary of 0** and **1.43 ms at a
//! boundary of 1,000** — a factor of two where the underlying cost differs by a
//! factor of a thousand. The block growth was the whole measurement.
//!
//! So each iteration now restores the boundary before it runs, **untimed**:
//!
//! * the memory arm rebuilds a `MemoryEventStore` from a pre-encoded
//!   `Vec<Event>`, which is one allocation pass;
//! * the SQLite arm copies a seeded **template file** and opens a store on the
//!   copy, which restores the log, the tag index and `tag_cardinality` exactly.
//!   Truncating with `DELETE` was the alternative and it is wrong: the adapter's
//!   `tag_cardinality` is a selectivity hint that a delete would leave
//!   inconsistent, so the planner would see a store no append could have
//!   produced.
//!
//! # The sweep is boundary size, and it is the only sweep that matters here
//!
//! Payload size and codec are `typed_codec.rs`'s; contention is
//! `store_conditional.rs`'s. What is left is how the cost of a command grows
//! with the length of the history it must fold. The answer should be linear
//! with a slope of one decode; an arm that is worse than linear is a finding.

use std::hint::black_box;
use std::num::NonZeroU32;
use std::path::PathBuf;
use std::time::{Duration, Instant};

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use happenstance::{Cbor, DomainEvent, Json, Postcard, Retry, commit, commit_with};
use happenstance_benchmarks::domain::{Recorded, Total};
use happenstance_benchmarks::fixtures::memory::{MemoryFixture, MemoryHandle};
use happenstance_benchmarks::fixtures::sqlite::SqliteFixture;
use happenstance_benchmarks::runtime;
use happenstance_core::{Event, EventStore, MemoryEventStore};
use happenstance_sqlite::event_store::SqliteEventStore;
use happenstance_testkit::Fixture;

/// How many prior events the boundary must fold before deciding.
///
/// Zero is the cold command — an account's first write, and the cheapest a
/// command can be. A thousand is an aggregate nobody has bounded; the curve
/// between them is what says whether the replay or the append dominates.
const HISTORY_LENGTHS: [usize; 4] = [0, 10, 100, 1_000];

/// The account every arm here writes to.
const ACCOUNT: &str = "a1";

/// Samples per group.
///
/// Ten, and each sample's iterations each pay an untimed restore. See the
/// module documentation for why the restore is not optional.
const SEEDED_SAMPLES: usize = 10;

/// How long criterion may spend measuring one point.
///
/// Two seconds, below criterion's five-second default, and it is load-bearing
/// rather than impatient: criterion picks its iteration count to fill this
/// window, and **every iteration costs an untimed restore**. A five-second
/// window at the cold boundary would ask for tens of thousands of restores to
/// measure a microsecond of work.
const MEASUREMENT_TIME: Duration = Duration::from_secs(2);

/// The boundary length the codec and retry groups hold fixed.
///
/// A hundred, so the decode count dominates the append. At a cold boundary the
/// three codecs would be one encode apart and indistinguishable — which is the
/// measurement that would let somebody conclude the choice does not matter.
const FIXED_HISTORY: usize = 100;

/// A refusal that cannot happen, so `decide` never short-circuits the loop.
///
/// The `decide` closure must be able to fail for `commit`'s signature to hold,
/// and every arm here needs it never to — a refused decision skips the append
/// and would report a decision-only figure as a command figure.
#[derive(Debug, thiserror::Error)]
#[error("the benchmark's decide never refuses")]
struct NeverRefuses;

/// One event as the typed layer writes it, encoded by `codec`.
///
/// Built through `DomainEvent` rather than by hand so a seeded history decodes
/// as bytes the library actually produces.
///
/// # Panics
///
/// Panics if the benchmark's own event cannot be built or encoded.
fn encoded(amount: u32, codec: &impl happenstance::Codec) -> Event {
    let event = Recorded::new(ACCOUNT, amount, 64);
    Event::new(
        event.event_type(),
        event.encode(codec).expect("encoding succeeds"),
    )
    .expect("the benchmark's own event type is valid")
    .with_tags(event.tags())
}

/// `count` events for [`ACCOUNT`], encoded once and reused by every restore.
fn history(count: usize, codec: &impl happenstance::Codec) -> Vec<Event> {
    (0..count)
        .map(|n| {
            encoded(
                u32::try_from(n % 1_000).expect("the modulus fits a u32"),
                codec,
            )
        })
        .collect()
}

/// A memory store holding exactly `events`, rebuilt from scratch.
///
/// `MemoryEventStore::with_events` assigns positions from one, so this is the
/// same log every time — not a store that has accumulated whatever the previous
/// iteration appended.
async fn restored_memory(events: &[Event]) -> MemoryHandle {
    let store = MemoryEventStore::with_events(events.iter().cloned());
    // `async` rather than a `block_on` inside: every caller is already inside
    // `runtime::block_on`, and tokio refuses a nested one with a panic. The
    // future is `core::future::ready`, so awaiting it costs a poll.
    MemoryFixture::sharing(std::sync::Arc::new(store))
        .connect()
        .await
}

/// A SQLite database file seeded with `events`, kept as a template to copy.
///
/// Returned with its fixture, which must outlive every copy: dropping it
/// removes the template.
fn sqlite_template(events: &[Event]) -> (SqliteFixture, PathBuf) {
    let fixture = SqliteFixture::new();
    runtime::block_on(async {
        let store = fixture.connect().await;
        let ceiling = <SqliteFixture as Fixture>::MAX_EVENTS_PER_BATCH.unwrap_or(128);
        for chunk in events.chunks(ceiling) {
            store.append(chunk, None).await.expect("seeding must land");
        }
    });
    // Fold the write-ahead log into the main file, so one `fs::copy` carries
    // the whole database rather than a main file plus a sidecar the copy would
    // leave behind.
    fixture.checkpoint();
    let path = fixture.path().to_path_buf();
    (fixture, path)
}

/// Copies `template` to a fresh path and opens a store on the copy.
///
/// Returns the fixture so the copy is removed when it drops.
///
/// # Panics
///
/// Panics if the copy or the open fails — a broken measurement environment,
/// not a finding.
async fn restored_sqlite(template: &PathBuf) -> (SqliteFixture, SqliteEventStore) {
    let fixture = SqliteFixture::new();
    std::fs::copy(template, fixture.path()).expect("the template copies");
    let store = fixture.connect().await;
    (fixture, store)
}

/// Runs one command against `store`, and returns what it cost.
async fn one_command(store: &impl EventStore) -> Duration {
    let boundary = Total::new(ACCOUNT).expect("the benchmark's account is valid");
    let started = Instant::now();
    let committed = commit(store, boundary, Retry::once(), |_: &Total| {
        Ok::<_, NeverRefuses>(vec![Recorded::new(ACCOUNT, 1, 64)])
    })
    .await
    .expect("the command lands");
    let elapsed = started.elapsed();
    let _ = black_box(committed);
    elapsed
}

/// One command against a boundary of a stated size, on both arms.
fn boundary_size(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("command/boundary-size");
    group.sample_size(SEEDED_SAMPLES);
    group.measurement_time(MEASUREMENT_TIME);

    for length in HISTORY_LENGTHS {
        let events = history(length, &Json);
        let (_template_fixture, template) = sqlite_template(&events);

        group.bench_function(BenchmarkId::new("memory", length), |bencher| {
            bencher.iter_custom(|iters| {
                runtime::block_on(async {
                    let mut total = Duration::ZERO;
                    for _ in 0..iters {
                        // Restored before every iteration, untimed. Without
                        // this the boundary grows across the block and the
                        // figure is about the growth. See the module docs.
                        let store = restored_memory(&events).await;
                        total += one_command(&store).await;
                    }
                    total
                })
            });
        });

        group.bench_function(BenchmarkId::new("sqlite", length), |bencher| {
            bencher.iter_custom(|iters| {
                runtime::block_on(async {
                    let mut total = Duration::ZERO;
                    for _ in 0..iters {
                        let (_fixture, store) = restored_sqlite(&template).await;
                        total += one_command(&store).await;
                    }
                    total
                })
            });
        });
    }

    group.finish();
}

/// The same command under each codec, at a boundary the decode dominates.
///
/// A hundred prior events, so the figure is a hundred decodes plus one encode
/// plus one append — the regime where the codec choice is the whole difference.
/// At a cold boundary it would be one encode against one append and the three
/// codecs would be indistinguishable, which is the measurement that would let
/// somebody conclude the choice does not matter.
///
/// Each codec seeds its **own** history: a history written by `Json` and read
/// by `Postcard` does not decode, and the arm would be timing an error path
/// rather than a format.
fn codec_choice(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("command/codec-choice");
    group.sample_size(SEEDED_SAMPLES);
    group.measurement_time(MEASUREMENT_TIME);

    macro_rules! codec_arm {
        ($name:literal, $codec:expr) => {{
            let events = history(FIXED_HISTORY, &$codec);
            group.bench_function($name, |bencher| {
                bencher.iter_custom(|iters| {
                    runtime::block_on(async {
                        let mut total = Duration::ZERO;
                        for _ in 0..iters {
                            let store = restored_memory(&events).await;
                            let boundary = Total::new(ACCOUNT).expect("valid");
                            let started = Instant::now();
                            let committed = commit_with(
                                &store,
                                boundary,
                                &$codec,
                                Retry::once(),
                                |_: &Total| {
                                    Ok::<_, NeverRefuses>(vec![Recorded::new(ACCOUNT, 1, 64)])
                                },
                            )
                            .await
                            .expect("the command lands");
                            total += started.elapsed();
                            let _ = black_box(committed);
                        }
                        total
                    })
                });
            });
        }};
    }

    codec_arm!("json", Json);
    codec_arm!("cbor", Cbor);
    codec_arm!("postcard", Postcard);

    group.finish();
}

/// What losing an attempt costs.
///
/// A second writer commits into the same boundary between this command's read
/// and its append, so the condition refuses and `commit` retries — paying for a
/// second full replay, a second round of decodes and a second encode. The
/// uncontended arm beside it is the control, and the ratio between them is what
/// says whether a retry is cheap.
///
/// The interference is injected **inside** the timed region on purpose: a
/// benchmark that set it up beforehand would measure a command whose first
/// attempt already knew it would lose, which is not the shape contention
/// produces.
fn retry_cost(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("command/retry");
    group.sample_size(SEEDED_SAMPLES);
    group.measurement_time(MEASUREMENT_TIME);

    let events = history(FIXED_HISTORY, &Json);

    group.bench_function("uncontended-control", |bencher| {
        bencher.iter_custom(|iters| {
            runtime::block_on(async {
                let mut total = Duration::ZERO;
                for _ in 0..iters {
                    let store = restored_memory(&events).await;
                    total += one_command(&store).await;
                }
                total
            })
        });
    });

    group.bench_function("loses-once-then-wins", |bencher| {
        bencher.iter_custom(|iters| {
            runtime::block_on(async {
                let attempts = NonZeroU32::new(3).expect("three is non-zero");
                let mut total = Duration::ZERO;
                for _ in 0..iters {
                    let store = restored_memory(&events).await;
                    let boundary = Total::new(ACCOUNT).expect("valid");
                    let mut interfered = false;

                    let started = Instant::now();
                    let committed =
                        commit(&store, boundary, Retry::attempts(attempts), |_: &Total| {
                            // The first attempt's decision runs, and only then
                            // is the boundary moved under it — so the append
                            // that follows is refused and the loop retries. The
                            // second attempt reads the interference, finds the
                            // flag already set, and wins.
                            //
                            // Driven by `happenstance_testkit::block_on`, the
                            // dependency-free park loop, and **not** by the
                            // shared tokio runtime: `decide` is a synchronous
                            // callback invoked from inside `commit`'s future,
                            // which is itself inside `runtime::block_on`, and
                            // tokio refuses a nested `block_on` with a panic.
                            // The park loop has no such check and the future it
                            // drives is immediately ready, because
                            // `MemoryEventStore` does no I/O.
                            //
                            // A plain `append` rather than a second `commit`:
                            // the interference only has to move the boundary,
                            // and a nested command would put its own replay
                            // inside this one's timed region.
                            if !interfered {
                                interfered = true;
                                happenstance_testkit::block_on(
                                    store.append(&[encoded(9, &Json)], None),
                                )
                                .expect("the interference lands");
                            }
                            Ok::<_, NeverRefuses>(vec![Recorded::new(ACCOUNT, 1, 64)])
                        })
                        .await
                        .expect("the command lands on its second attempt");
                    total += started.elapsed();

                    assert_eq!(
                        committed.attempts, 2,
                        "the interference must cost exactly one lost attempt — if it \
                         stops costing one, this arm is a duplicate of the control \
                         beside it and the retry ratio is 1.0 for the wrong reason"
                    );
                }
                total
            })
        });
    });

    group.finish();
}

criterion_group!(benches, boundary_size, codec_choice, retry_cost);
criterion_main!(benches);
