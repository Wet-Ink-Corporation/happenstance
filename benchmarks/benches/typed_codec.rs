//! The codec: what encoding a domain event costs, in all three formats.
//!
//! # Why this is a group and not a footnote
//!
//! `happenstance-core` never looks inside a payload — ADR-0003 makes it opaque
//! `Bytes`. `happenstance` is the crate whose *entire job* is encoding, so the
//! codec is not an implementation detail of the typed layer, it is the typed
//! layer's cost. Every `commit` encodes what the caller decided; every
//! `run_projection` decodes every event it applies. Nothing in the repository
//! has ever priced either.
//!
//! # The three codecs cross, which is why all three are swept together
//!
//! `Json` is the default and the one a first program gets. It is also the one
//! whose payload cost is a *multiplier*: ADR-0016 encodes `Bytes` as base64, so
//! a 64 KiB body becomes about 87 KiB of JSON before serde has looked at the
//! structure. `Postcard` is non-self-describing and pays almost nothing for
//! structure; `Cbor` sits between them.
//!
//! So which encoder wins depends entirely on the ratio between payload bytes
//! and structural fields — and a benchmark taken at one payload size would pick
//! a winner and be wrong at every other size. The sweep is what makes the
//! answer a curve rather than an opinion.
//!
//! # What the review already measured, and what is left
//!
//! `references/evaluation/review-pre-publication-2026-09-03.md:2756` measured
//! encoding **one 64-tag `SequencedEvent`** to postcard at **140 heap
//! operations to produce 587 bytes, of which 130 (93%) are transient clones
//! that produce no output**. That is the *envelope* path — the wire format
//! `happenstance-sync` uses — and it is not what this file measures.
//!
//! This measures the **application payload** path: `DomainEvent::encode` and
//! `::decode`, which is what `commit` and `run_projection` call. The two are
//! different code and the second is the one every application pays on every
//! command. The allocation half of it is in `src/bin/allocations.rs`, reported
//! in heap operations rather than nanoseconds, because
//! `review-pre-publication-2026-09-03.md:2836` is explicit: *"Quote the
//! allocation columns and not the timings."*
//!
//! # Encoded size is reported beside encode time
//!
//! A codec that is fast and produces twice the bytes has moved the cost to the
//! store, where it becomes a bigger row, a bigger page and a bigger WAL. The
//! size is printed once per point, outside the timed region, so the two are
//! never quoted apart.

use std::hint::black_box;

use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use happenstance::{Cbor, DomainEvent, Json, Postcard};
use happenstance_benchmarks::domain::Recorded;

/// Payload sizes swept, ending at VT-21's conformance floor of 64 KiB.
///
/// Zero is included and is the point of the whole sweep: at zero payload the
/// figure is pure structural cost, which is where `Postcard` should win by the
/// widest margin and where `Json`'s base64 multiplier cannot help it.
const PAYLOAD_SIZES: [usize; 4] = [0, 256, 4_096, 65_536];

/// Encodes one domain event, per codec, per payload size.
fn encode(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("codec/encode");

    for payload in PAYLOAD_SIZES {
        let event = Recorded::new("a1", 7, payload);
        group.throughput(Throughput::Bytes(payload as u64));

        // The encoded size, printed once. A codec that is fast and doubles the
        // bytes has moved the cost into the store rather than removed it.
        for (name, encoded) in [
            ("json", event.encode(&Json).expect("json encodes").len()),
            ("cbor", event.encode(&Cbor).expect("cbor encodes").len()),
            (
                "postcard",
                event.encode(&Postcard).expect("postcard encodes").len(),
            ),
        ] {
            println!("codec/encode payload={payload}B {name} -> {encoded}B");
        }

        group.bench_with_input(
            BenchmarkId::new("json", payload),
            &event,
            |bencher, event| {
                bencher.iter(|| black_box(event.encode(&Json).expect("json encodes")));
            },
        );
        group.bench_with_input(
            BenchmarkId::new("cbor", payload),
            &event,
            |bencher, event| {
                bencher.iter(|| black_box(event.encode(&Cbor).expect("cbor encodes")));
            },
        );
        group.bench_with_input(
            BenchmarkId::new("postcard", payload),
            &event,
            |bencher, event| {
                bencher.iter(|| black_box(event.encode(&Postcard).expect("postcard encodes")));
            },
        );
    }

    group.finish();
}

/// Decodes one domain event, per codec, per payload size.
///
/// The path a projection runner takes once per event it applies, and the path
/// `commit` takes once per event in its decision model's replay. On a warm
/// aggregate the decode count *is* the replay length, so this figure multiplied
/// by log length is most of what `typed_command.rs` reports.
fn decode(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("codec/decode");

    for payload in PAYLOAD_SIZES {
        let event = Recorded::new("a1", 7, payload);
        let event_type = event.event_type();
        group.throughput(Throughput::Bytes(payload as u64));

        let json = event.encode(&Json).expect("json encodes");
        let cbor = event.encode(&Cbor).expect("cbor encodes");
        let postcard = event.encode(&Postcard).expect("postcard encodes");

        group.bench_with_input(BenchmarkId::new("json", payload), &json, |bencher, data| {
            bencher.iter(|| {
                black_box(
                    <Recorded as DomainEvent>::decode(&Json, &event_type, data)
                        .expect("json decodes"),
                );
            });
        });
        group.bench_with_input(BenchmarkId::new("cbor", payload), &cbor, |bencher, data| {
            bencher.iter(|| {
                black_box(
                    <Recorded as DomainEvent>::decode(&Cbor, &event_type, data)
                        .expect("cbor decodes"),
                );
            });
        });
        group.bench_with_input(
            BenchmarkId::new("postcard", payload),
            &postcard,
            |bencher, data| {
                bencher.iter(|| {
                    black_box(
                        <Recorded as DomainEvent>::decode(&Postcard, &event_type, data)
                            .expect("postcard decodes"),
                    );
                });
            },
        );
    }

    group.finish();
}

/// The round trip, which is what a rebuild actually performs.
///
/// A projection rebuilding from position 1 decodes every event it ever wrote.
/// The two halves are measured apart above because they are different code with
/// different costs — `serde_json`'s decode is dominated by parsing where its
/// encode is dominated by formatting — and together here because that is the
/// shape a runner pays.
fn round_trip(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("codec/round-trip");

    for payload in PAYLOAD_SIZES {
        let event = Recorded::new("a1", 7, payload);
        let event_type = event.event_type();
        group.throughput(Throughput::Bytes(payload as u64));

        group.bench_with_input(
            BenchmarkId::new("json", payload),
            &event,
            |bencher, event| {
                bencher.iter(|| {
                    let data = event.encode(&Json).expect("json encodes");
                    black_box(
                        <Recorded as DomainEvent>::decode(&Json, &event_type, &data)
                            .expect("json decodes"),
                    );
                });
            },
        );
        group.bench_with_input(
            BenchmarkId::new("cbor", payload),
            &event,
            |bencher, event| {
                bencher.iter(|| {
                    let data = event.encode(&Cbor).expect("cbor encodes");
                    black_box(
                        <Recorded as DomainEvent>::decode(&Cbor, &event_type, &data)
                            .expect("cbor decodes"),
                    );
                });
            },
        );
        group.bench_with_input(
            BenchmarkId::new("postcard", payload),
            &event,
            |bencher, event| {
                bencher.iter(|| {
                    let data = event.encode(&Postcard).expect("postcard encodes");
                    black_box(
                        <Recorded as DomainEvent>::decode(&Postcard, &event_type, &data)
                            .expect("postcard decodes"),
                    );
                });
            },
        );
    }

    group.finish();
}

criterion_group!(benches, encode, decode, round_trip);
criterion_main!(benches);
