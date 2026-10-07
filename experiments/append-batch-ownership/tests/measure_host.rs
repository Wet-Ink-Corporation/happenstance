//! The host-side figures: the clone a borrowed store pays, the batch a
//! typed-layer caller rebuilds, the memory control, and the k = 8 caller-side
//! run on the reference store.
//!
//! Run after `tests/host.rs`. `run.sh` tees this to `results/raw/host.txt`. The
//! timing discipline is `tests/measure_cloudflare.rs`'s: batch built outside the
//! region, append and drop inside, arms interleaved and rotated, `WARMUP`
//! discarded, `N` summarised. Each test holds [`region::hold`] throughout, as
//! `tests/host.rs`'s do, so a plain `cargo test` cannot print one test's
//! allocations in another's row.

#![cfg(not(target_arch = "wasm32"))]

use std::io::Write as _;

use append_batch_ownership::batch::{self, BOUNDARY, Regime, Shape};
use append_batch_ownership::contention::{self, Attempt, CONTENDERS, Tally};
use append_batch_ownership::measured::Counts;
use append_batch_ownership::memory_arms::{BorrowedLog, OwnedLog, Refused};
use append_batch_ownership::poll::now_or_never;
use append_batch_ownership::region::{self, Region};
use append_batch_ownership::stats::{Summary, now_us};
use happenstance_core::{
    AppendCondition, AppendError, Event, EventStore, MemoryEventStore, Query, QueryItem,
    SequencePosition, StoreId, Tags,
};

const WARMUP: usize = 3;
const N: usize = 21;
const TAGS: [usize; 3] = [1, 8, 64];
const REGIMES: [Regime; 2] = [Regime::VecBacked, Regime::Static];
const STORE: StoreId = StoreId::from_bytes([3; 16]);

fn line(text: &str) {
    let mut out = std::io::stdout().lock();
    writeln!(out, "{text}").expect("stdout is open under the test harness");
}

fn shape(batch: usize, tags: usize, payload: usize, regime: Regime) -> Shape {
    Shape {
        batch,
        tags,
        payload,
        regime,
    }
}

fn row(label: &str, shape: Shape, counts: &[Counts], times: &[f64]) {
    let summary = Summary::of(times).expect("N > 0");
    let ops = |pick: fn(&Counts) -> u64| {
        (
            counts.iter().map(pick).min().unwrap_or(0),
            counts.iter().map(pick).max().unwrap_or(0),
        )
    };
    let (ops_min, ops_max) = ops(Counts::heap_ops);
    let (bytes_min, bytes_max) = ops(|c| c.bytes);
    let per_event = ops_min / u64::try_from(shape.batch.max(1)).unwrap_or(1);
    line(&format!(
        "{shape} arm={label:<20} heap_ops={ops_min}..{ops_max} per_event={per_event} \
         bytes={bytes_min}..{bytes_max} {summary}"
    ));
}

/// What one clone of a batch costs, first and second time, and what building
/// one costs — the two prices a retrying caller can pay.
#[test]
fn a_caller_side_prices() {
    let mut region = region::hold();
    line(&format!(
        "== caller-side prices, batch=128 payload=1024, warmup={WARMUP}, n={N} =="
    ));
    for regime in REGIMES {
        for tags in TAGS {
            let shape = shape(128, tags, 1024, regime);
            // Index 0: building a batch; 1: its first clone; 2: a later clone.
            let mut counts: [Vec<Counts>; 3] = [Vec::new(), Vec::new(), Vec::new()];
            let mut times: [Vec<f64>; 3] = [Vec::new(), Vec::new(), Vec::new()];
            for rep in 0..WARMUP + N {
                let mut marks = [0.0_f64; 4];
                marks[0] = now_us();
                let (original, building) =
                    region.measure(|| batch::build(shape, rep).expect("builds"));
                marks[1] = now_us();
                // Both clones are the subject: the price of keeping a copy.
                let (copy, cloning) = region.measure(|| original.clone());
                marks[2] = now_us();
                let (again, recloning) = region.measure(|| original.clone());
                marks[3] = now_us();
                drop((copy, again, original));
                if rep >= WARMUP {
                    for (slot, spent) in [building, cloning, recloning].into_iter().enumerate() {
                        if let (Some(c), Some(t), Some(from), Some(to)) = (
                            counts.get_mut(slot),
                            times.get_mut(slot),
                            marks.get(slot),
                            marks.get(slot + 1),
                        ) {
                            c.push(spent);
                            t.push(to - from);
                        }
                    }
                }
            }
            let labels = ["build (typed retry)", "first clone", "later clone"];
            for ((label, c), t) in labels.iter().zip(&counts).zip(&times) {
                row(label, shape, c, t);
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum MemoryArm {
    Real,
    Borrowed,
    Owned,
}

const MEMORY_ARMS: [MemoryArm; 3] = [MemoryArm::Real, MemoryArm::Borrowed, MemoryArm::Owned];

fn memory_once(region: &mut Region, arm: MemoryArm, batch: Vec<Event>) -> (Counts, f64) {
    // Each store is created outside the region and dropped after it, so the
    // region holds the append and the batch's drop and nothing else.
    let real = MemoryEventStore::with_store_id(STORE);
    let mut borrowed = BorrowedLog::new(STORE);
    let mut owned = OwnedLog::new(STORE);
    let start = now_us();
    let (landed, counts) = region.measure(|| match arm {
        MemoryArm::Real => {
            let landed = matches!(now_or_never(real.append(&batch, None)), Some(Ok(_)));
            drop(batch);
            landed
        }
        MemoryArm::Borrowed => {
            let landed = borrowed.append(&batch, None).is_ok();
            drop(batch);
            landed
        }
        MemoryArm::Owned => owned.append(batch, None).is_ok(),
    });
    let elapsed = now_us() - start;
    assert!(landed, "{arm:?} refused an unconditioned append");
    drop((real, borrowed, owned));
    (counts, elapsed)
}

/// The control: `MemoryEventStore`, its borrowed replica and the owned one.
#[test]
fn b_memory_control() {
    let mut region = region::hold();
    line(&format!("== memory control, warmup={WARMUP}, n={N} =="));
    for regime in REGIMES {
        for batch_len in [1, 16, 128] {
            for payload in [64, 1024, 16 * 1024] {
                for tags in TAGS {
                    let shape = shape(batch_len, tags, payload, regime);
                    let mut counts: Vec<Vec<Counts>> = vec![Vec::new(); MEMORY_ARMS.len()];
                    let mut times: Vec<Vec<f64>> = vec![Vec::new(); MEMORY_ARMS.len()];
                    for rep in 0..WARMUP + N {
                        for offset in 0..MEMORY_ARMS.len() {
                            let index = (rep + offset) % MEMORY_ARMS.len();
                            let Some(arm) = MEMORY_ARMS.get(index) else {
                                continue;
                            };
                            let built = batch::build(shape, rep).expect("builds");
                            let (spent, elapsed) = memory_once(&mut region, *arm, built);
                            if rep >= WARMUP
                                && let (Some(c), Some(t)) =
                                    (counts.get_mut(index), times.get_mut(index))
                            {
                                c.push(spent);
                                t.push(elapsed);
                            }
                        }
                    }
                    for ((arm, c), t) in MEMORY_ARMS.iter().zip(&counts).zip(&times) {
                        row(&format!("{arm:?}"), shape, c, t);
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Caller {
    RealResend,
    BorrowedResend,
    OwnedClonePerAttempt,
    BorrowedRebuild,
    OwnedRebuild,
}

const CALLERS: [Caller; 5] = [
    Caller::RealResend,
    Caller::BorrowedResend,
    Caller::OwnedClonePerAttempt,
    Caller::BorrowedRebuild,
    Caller::OwnedRebuild,
];

fn fence(boundary: &Tags, decided_at: Option<SequencePosition>) -> AppendCondition {
    let item = QueryItem::tagged(boundary.clone()).expect("one tag is a valid item");
    AppendCondition::new(Query::from_item(item)).after_opt(decided_at)
}

/// Why a contended run stopped before every contender committed, one label per
/// cause, so a failure reads as what happened rather than as an empty batch.
/// `Debug` is written out because it is how a stopped run is reported (the
/// `expect` in `contended_once`), and a derived one would leave every field
/// unread as far as the dead-code lint can see.
enum Stopped {
    /// A replica refused for a reason other than its condition.
    Replica(Refused),
    /// The schedule named a contender with no batch.
    NoSuchContender(usize),
    /// `MemoryEventStore::append` suspended, which `poll.rs` says it never does.
    RealPended,
    /// `MemoryEventStore::append` failed for a reason other than its condition.
    RealFailed(RealError),
}

/// What `MemoryEventStore::append` can fail with.
type RealError = AppendError<<MemoryEventStore as EventStore>::Error>;

impl std::fmt::Debug for Stopped {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Replica(refused) => write!(f, "a replica refused: {refused:?}"),
            Self::NoSuchContender(contender) => write!(f, "no batch for contender {contender}"),
            Self::RealPended => f.write_str("MemoryEventStore::append suspended"),
            Self::RealFailed(error) => write!(f, "MemoryEventStore::append failed: {error:?}"),
        }
    }
}

fn outcome(result: Result<SequencePosition, Refused>) -> Attempt<Stopped> {
    match result {
        Ok(position) => Attempt::Committed(position),
        Err(Refused::Conflict(_)) => Attempt::Refused,
        Err(other) => Attempt::Failed(Stopped::Replica(other)),
    }
}

fn real_outcome(result: Option<Result<SequencePosition, RealError>>) -> Attempt<Stopped> {
    match result {
        Some(Ok(position)) => Attempt::Committed(position),
        Some(Err(AppendError::ConditionViolated(_))) => Attempt::Refused,
        Some(Err(other)) => Attempt::Failed(Stopped::RealFailed(other)),
        None => Attempt::Failed(Stopped::RealPended),
    }
}

fn contended_once(
    region: &mut Region,
    caller: Caller,
    shape: Shape,
    boundary: &Tags,
) -> (Counts, f64, Tally) {
    let held: Vec<Vec<Event>> = (0..CONTENDERS)
        .map(|contender| batch::build(shape, contender * 1000).expect("builds"))
        .collect();
    let real = MemoryEventStore::with_store_id(STORE);
    let mut borrowed = BorrowedLog::new(STORE);
    let mut owned = OwnedLog::new(STORE);
    let start = now_us();
    let (tally, counts) = region.measure(|| {
        contention::run(|contender, decided_at| {
            let condition = fence(boundary, decided_at);
            let Some(mine) = held.get(contender) else {
                return Attempt::Failed(Stopped::NoSuchContender(contender));
            };
            let rebuilt = || batch::build(shape, contender * 1000).expect("builds");
            match caller {
                Caller::RealResend => {
                    real_outcome(now_or_never(real.append(mine, Some(&condition))))
                }
                Caller::BorrowedResend => outcome(borrowed.append(mine, Some(&condition))),
                // The clone is the subject: the owned shape's price for a copy.
                Caller::OwnedClonePerAttempt => {
                    outcome(owned.append(mine.clone(), Some(&condition)))
                }
                Caller::BorrowedRebuild => outcome(borrowed.append(&rebuilt(), Some(&condition))),
                Caller::OwnedRebuild => outcome(owned.append(rebuilt(), Some(&condition))),
            }
        })
    });
    let elapsed = now_us() - start;
    drop((held, real, borrowed, owned));
    (counts, elapsed, tally.expect("only conditions refuse"))
}

/// The caller-side axis under contention, on the reference store.
#[test]
fn c_memory_contention() {
    let mut region = region::hold();
    line(&format!(
        "== memory k={CONTENDERS} contention, batch=128 payload=1024 regime=vec, \
         warmup={WARMUP}, n={N} =="
    ));
    let boundary = Tags::from_pairs([BOUNDARY]).expect("valid");
    for tags in TAGS {
        let shape = shape(128, tags, 1024, Regime::VecBacked);
        let mut counts: Vec<Vec<Counts>> = vec![Vec::new(); CALLERS.len()];
        let mut times: Vec<Vec<f64>> = vec![Vec::new(); CALLERS.len()];
        let mut tallies = Vec::new();
        for rep in 0..WARMUP + N {
            for offset in 0..CALLERS.len() {
                let index = (rep + offset) % CALLERS.len();
                let Some(caller) = CALLERS.get(index) else {
                    continue;
                };
                let (spent, elapsed, tally) =
                    contended_once(&mut region, *caller, shape, &boundary);
                if rep >= WARMUP
                    && let (Some(c), Some(t)) = (counts.get_mut(index), times.get_mut(index))
                {
                    c.push(spent);
                    t.push(elapsed);
                    tallies.push(tally);
                }
            }
        }
        let tally = tallies.first().copied().expect("ran");
        assert!(
            tallies.iter().all(|seen| *seen == tally),
            "the schedule is deterministic"
        );
        line(&format!(
            "{shape} schedule: attempts={} commits={} rejections={}",
            tally.attempts, tally.commits, tally.rejections
        ));
        for ((caller, c), t) in CALLERS.iter().zip(&counts).zip(&times) {
            row(&format!("{caller:?}"), shape, c, t);
        }
    }
}
