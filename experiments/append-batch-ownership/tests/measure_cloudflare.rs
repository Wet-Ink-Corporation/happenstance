//! The Cloudflare measurement: the sweep, and the k = 8 caller-side run.
//!
//! Run after `tests/cloudflare.rs`, never before: those tests are what make
//! these rows comparable. `run.sh` tees this to `results/raw/cloudflare-*.txt`.
//!
//! # How one row is produced
//!
//! For each sweep point, one fresh Durable Object. Each repetition builds a
//! fresh batch per arm **outside** the timed region, then times the append
//! **and the drop of the batch** — O1 drops inside the call, the borrowed arms
//! right after it, so both pay the teardown and neither pays construction. The
//! four arms run interleaved within a repetition, in an order rotated by the
//! repetition number, so drift in the JS heap lands on every arm alike. Rows are
//! deleted between repetitions, outside the timed region. `WARMUP` repetitions
//! are discarded; the next `N` are summarised.
//!
//! Allocation counts are taken over the same region. They are deterministic,
//! and each row prints the minimum and maximum across the `N` repetitions to
//! show that they are.

#![cfg(target_arch = "wasm32")]

use append_batch_ownership::batch::{self, Regime, Shape};
use append_batch_ownership::cloudflare::{ArmError, B0, B1, Fence, O1, Subject};
use append_batch_ownership::contention::{self, Attempt, CONTENDERS};
use append_batch_ownership::measured::{Counts, measure};
use append_batch_ownership::poll::now_or_never;
use append_batch_ownership::stats::{Summary, now_us};
use happenstance_core::{Event, EventStore, SequencePosition};
use wasm_bindgen_test::{console_log, wasm_bindgen_test};

const WARMUP: usize = 3;
const N: usize = 21;
const TAGS: [usize; 3] = [1, 8, 64];
const PAYLOADS: [usize; 4] = [64, 1024, 16 * 1024, 256 * 1024];
const REGIMES: [Regime; 2] = [Regime::VecBacked, Regime::Static];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Arm {
    Real,
    B0,
    B1,
    O1,
}

const ARMS: [Arm; 4] = [Arm::Real, Arm::B0, Arm::B1, Arm::O1];

impl Arm {
    const fn label(self) -> &'static str {
        match self {
            Self::Real => "adapter",
            Self::B0 => "B0",
            Self::B1 => "B1",
            Self::O1 => "O1",
        }
    }
}

struct Arms {
    subject: Subject,
    b0: B0,
    b1: B1,
    o1: O1,
}

impl Arms {
    fn open() -> Self {
        let subject = Subject::open().expect("a fresh object migrates");
        let b0 = B0::over(&subject).expect("identity");
        let b1 = B1::over(&subject).expect("identity");
        let o1 = O1::over(&subject).expect("identity");
        Self {
            subject,
            b0,
            b1,
            o1,
        }
    }

    /// One timed append of a freshly built batch; the batch is dropped inside
    /// the region for every arm.
    fn once(&self, arm: Arm, batch: Vec<Event>) -> (Counts, f64) {
        let start = now_us();
        let (landed, counts) = measure(|| match arm {
            Arm::Real => {
                let landed = matches!(
                    now_or_never(self.subject.real().append(&batch, None)),
                    Some(Ok(_))
                );
                drop(batch);
                landed
            }
            Arm::B0 => {
                let landed = self.b0.append(&batch, None).is_ok();
                drop(batch);
                landed
            }
            Arm::B1 => {
                let landed = self.b1.append(&batch, None).is_ok();
                drop(batch);
                landed
            }
            Arm::O1 => self.o1.append_owned(batch, None).is_ok(),
        });
        let elapsed = now_us() - start;
        assert!(landed, "{} refused an unconditioned append", arm.label());
        self.subject.clear().expect("rows delete");
        (counts, elapsed)
    }
}

struct Samples {
    times: Vec<f64>,
    ops: Vec<u64>,
    bytes: Vec<u64>,
}

impl Samples {
    fn new() -> Self {
        Self {
            times: Vec::with_capacity(N),
            ops: Vec::with_capacity(N),
            bytes: Vec::with_capacity(N),
        }
    }

    fn row(&self, label: &str, shape: Shape) -> Summary {
        let summary = Summary::of(&self.times).expect("N > 0");
        let range = |values: &[u64]| {
            (
                values.iter().copied().min().unwrap_or(0),
                values.iter().copied().max().unwrap_or(0),
            )
        };
        let (ops_min, ops_max) = range(&self.ops);
        let (bytes_min, bytes_max) = range(&self.bytes);
        let per_event = ops_min / u64::try_from(shape.batch.max(1)).unwrap_or(1);
        console_log!(
            "{shape} arm={label:<7} heap_ops={ops_min}..{ops_max} per_event={per_event} \
             bytes={bytes_min}..{bytes_max} {summary}"
        );
        summary
    }
}

fn sweep(batch: usize) {
    console_log!("== cloudflare sweep, batch={batch}, warmup={WARMUP}, n={N} ==");
    for regime in REGIMES {
        for payload in PAYLOADS {
            for tags in TAGS {
                let shape = Shape {
                    batch,
                    tags,
                    payload,
                    regime,
                };
                let arms = Arms::open();
                let mut samples: Vec<Samples> = ARMS.iter().map(|_| Samples::new()).collect();
                for rep in 0..WARMUP + N {
                    for offset in 0..ARMS.len() {
                        let index = (rep + offset) % ARMS.len();
                        let (Some(arm), Some(into)) = (ARMS.get(index), samples.get_mut(index))
                        else {
                            continue;
                        };
                        let built = batch::build(shape, rep).expect("builds");
                        let (counts, elapsed) = arms.once(*arm, built);
                        if rep >= WARMUP {
                            into.times.push(elapsed);
                            into.ops.push(counts.heap_ops());
                            into.bytes.push(counts.bytes);
                        }
                    }
                }
                let summaries: Vec<Summary> = ARMS
                    .iter()
                    .zip(&samples)
                    .map(|(arm, samples)| samples.row(arm.label(), shape))
                    .collect();
                if let [_, _, b1, o1] = summaries.as_slice() {
                    let saving = (b1.median - o1.median) / b1.median * 100.0;
                    console_log!(
                        "{shape} rule: O1 vs B1 saving={saving:.1}% o1_median<b1_q1={} \
                         fires={}",
                        o1.median < b1.q1,
                        o1.median < 0.9 * b1.median && o1.median < b1.q1
                    );
                }
            }
        }
    }
}

#[wasm_bindgen_test]
fn sweep_batch_1() {
    sweep(1);
}

#[wasm_bindgen_test]
fn sweep_batch_16() {
    sweep(16);
}

#[wasm_bindgen_test]
fn sweep_batch_128() {
    sweep(128);
}

/// The caller-side scenarios: what a contender does with its batch between
/// attempts.
#[derive(Debug, Clone, Copy)]
enum Caller {
    /// Raw port, borrowed: the same batch is resent. Free.
    B1Resend,
    /// Raw port, owned: a clone per attempt, because the call consumes it and a
    /// refusal gives nothing back.
    O1ClonePerAttempt,
    /// Typed-layer shape, borrowed: the command loop rebuilds every attempt.
    B1Rebuild,
    /// Typed-layer shape, owned: rebuilt every attempt, so moving costs nothing.
    O1Rebuild,
    /// The adapter's own path, resending: the reference.
    B0Resend,
}

const CALLERS: [Caller; 5] = [
    Caller::B0Resend,
    Caller::B1Resend,
    Caller::O1ClonePerAttempt,
    Caller::B1Rebuild,
    Caller::O1Rebuild,
];

fn salt(contender: usize) -> usize {
    contender * 1000
}

fn attempt_outcome(outcome: Result<SequencePosition, ArmError>) -> Attempt<ArmError> {
    match outcome {
        Ok(position) => Attempt::Committed(position),
        Err(ArmError::Conflict(_)) => Attempt::Refused,
        Err(other) => Attempt::Failed(other),
    }
}

fn contended_once(arms: &Arms, caller: Caller, shape: Shape) -> (Counts, f64, contention::Tally) {
    // What each contender holds before the run starts: its first decision's
    // batch. Built outside the region; the rebuild scenarios ignore it.
    let held: Vec<Vec<Event>> = (0..CONTENDERS)
        .map(|contender| batch::build(shape, salt(contender)).expect("builds"))
        .collect();
    let start = now_us();
    let (tally, counts) = measure(|| {
        contention::run(|contender, decided_at| {
            let Ok(fence) = Fence::on_boundary(decided_at) else {
                return Attempt::Failed(ArmError::Row("boundary tag"));
            };
            let Some(mine) = held.get(contender) else {
                return Attempt::Failed(ArmError::Row("no such contender"));
            };
            let rebuilt = || batch::build(shape, salt(contender)).expect("builds");
            attempt_outcome(match caller {
                Caller::B0Resend => arms.b0.append(mine, Some(&fence)),
                Caller::B1Resend => arms.b1.append(mine, Some(&fence)),
                // The clone is the subject: the owned shape's price for keeping
                // a batch a refusal would otherwise destroy.
                Caller::O1ClonePerAttempt => arms.o1.append_owned(mine.clone(), Some(&fence)),
                Caller::B1Rebuild => arms.b1.append(&rebuilt(), Some(&fence)),
                Caller::O1Rebuild => arms.o1.append_owned(rebuilt(), Some(&fence)),
            })
        })
    });
    let elapsed = now_us() - start;
    drop(held);
    let tally = tally.expect("no attempt failed for a reason other than its condition");
    arms.subject.clear().expect("rows delete");
    (counts, elapsed, tally)
}

#[wasm_bindgen_test]
fn contention_k8_batch_128() {
    const CONTENDED_WARMUP: usize = 2;
    console_log!(
        "== cloudflare k={CONTENDERS} contention, batch=128 payload=1024 regime=vec, \
         warmup={CONTENDED_WARMUP}, n={N} =="
    );
    for tags in TAGS {
        let shape = Shape {
            batch: 128,
            tags,
            payload: 1024,
            regime: Regime::VecBacked,
        };
        let arms = Arms::open();
        let mut samples: Vec<(Vec<f64>, Vec<u64>, Vec<u64>)> = CALLERS
            .iter()
            .map(|_| (Vec::new(), Vec::new(), Vec::new()))
            .collect();
        let mut tallies = Vec::new();
        for rep in 0..CONTENDED_WARMUP + N {
            for offset in 0..CALLERS.len() {
                let index = (rep + offset) % CALLERS.len();
                let (Some(caller), Some(into)) = (CALLERS.get(index), samples.get_mut(index))
                else {
                    continue;
                };
                let (counts, elapsed, tally) = contended_once(&arms, *caller, shape);
                if rep >= CONTENDED_WARMUP {
                    into.0.push(elapsed);
                    into.1.push(counts.heap_ops());
                    into.2.push(counts.bytes);
                    tallies.push(tally);
                }
            }
        }
        for (caller, (times, ops, bytes)) in CALLERS.iter().zip(&samples) {
            let summary = Summary::of(times).expect("N > 0");
            let tally = tallies.first().copied().expect("ran");
            assert!(
                tallies.iter().all(|seen| *seen == tally),
                "the schedule is deterministic"
            );
            console_log!(
                "{shape} caller={:<18} attempts={} commits={} rejections={} \
                 heap_ops={}..{} bytes={}..{} {summary}",
                format!("{caller:?}"),
                tally.attempts,
                tally.commits,
                tally.rejections,
                ops.iter().min().copied().unwrap_or(0),
                ops.iter().max().copied().unwrap_or(0),
                bytes.iter().min().copied().unwrap_or(0),
                bytes.iter().max().copied().unwrap_or(0),
            );
        }
    }
}
