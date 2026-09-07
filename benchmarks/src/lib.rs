//! The standing benchmark suite for happenstance.
//!
//! # What this crate is, in one paragraph
//!
//! It measures the shipped crates — `happenstance-core`, `happenstance` and
//! `happenstance-sqlite` — against each other, against raw SQL on the same
//! file, and against their own history. It lives outside the workspace, it is
//! invoked by a human typing `./run.sh`, and **no result it produces can turn a
//! merge red**. That is CF-34 (`spec/SPECIFICATION.md:8747`), whose own
//! `Rejects:` line is *"a benchmark result gating a merge"*.
//!
//! # The five things that are true of every figure here
//!
//! 1. **Conformance first.** Every arm that gets timed has passed
//!    `event_store_conformance!` in `tests/conformance_first.rs` before any
//!    figure taken through it is kept. A wrong arm is always the fastest.
//! 2. **The regime is stated.** [`corpus::Regime`] is a required parameter with
//!    no default, because a suite that builds its events out of `from_static`
//!    constants measures a regime with a 66× cheaper clone and reports it as
//!    the library's.
//! 3. **Absolutes from criterion, ratios from [`paired`].** criterion runs group
//!    members sequentially, and on this class of host sequential arms have
//!    drifted 2.7–3.0× — larger than the effects being measured.
//! 4. **Counts before timings.** [`counting`] produces figures that were
//!    identical to the digit across four runs on a host whose wall-clock
//!    medians moved 40%. Where an allocation count and a duration answer the
//!    same question, the count is what gets quoted.
//! 5. **No threshold, at any budget.** The only assertions in this crate are in
//!    `tests/`, and every one of them fires when the *instrument* has stopped
//!    working — never when a number is large.
//!
//! # What lives where
//!
//! | Module | Answers |
//! | --- | --- |
//! | [`corpus`] | what the workload is, and in which allocation regime |
//! | [`domain`] | the typed layer's workload: one event, one model, two projections |
//! | [`fixtures`] | the three arms and the two floors they are measured above |
//! | [`runtime`] | one executor for all of them, so it is not a confound |
//! | [`paired`] | the round-robin sampler every ratio comes from |
//! | [`counting`] | the allocator, installed only by `src/bin/allocations.rs` |
//! | [`cpu`] | processor time, and the ratio that exposes a blocking call |
//! | [`report`] | conditions, rows, and the record a run commits |
//!
//! # What it deliberately does not do
//!
//! * **No comparison against a peer library.**
//!   `references/seeds/measured-not-claimed.md:215-222` declines it: *"
//!   cross-language, cross-design throughput comparisons are contested by
//!   default and the credibility cost outweighs the positioning. Own history,
//!   cross-adapter within the workspace, and overhead above the bare database
//!   are the comparisons worth standing behind."* That decision was reaffirmed
//!   when this crate was commissioned; overturning it is a deliberate act, not
//!   a drift.
//! * **No re-measurement of what an experiment already answered.** Reactor
//!   stall and `PAGE_SIZE` are `experiments/one-connection-latency/`; poll cost
//!   and fan-out amplification are `experiments/polling-cost/`; the
//!   append-condition SQL is `experiments/append-condition/` and
//!   `experiments/shipped-append-condition-sql/`. They are cited, not repeated.
//! * **No third significant figure.** One machine, one run per cell. Every
//!   claim is a ratio between arms of one run, or an order of magnitude.

pub mod corpus;
pub mod counting;
pub mod cpu;
pub mod domain;
pub mod fixtures;
pub mod paired;
pub mod report;
pub mod runtime;
