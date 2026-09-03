//! What SQLite's busy handler actually costs a contender, and how few cores it
//! takes for 5,000 ms to stop being enough.
//!
//! # The question
//!
//! `crates/happenstance-sqlite/src/connection.rs:62` ships
//! `BUSY_TIMEOUT_MS = 5_000` as a constant, and it is the only liveness bound
//! anywhere in the system: CF-33 is `[FROZEN]` and forbids the conformance suite
//! from carrying a watchdog, so nothing else can time-bound a contended run. The
//! constant's own documentation says *"Five seconds was measured to absorb
//! 64-way contention with zero `SQLITE_BUSY` (ADR-0022 §11)"*.
//!
//! That measurement is real and it is `experiments/append-condition`'s. Its
//! recorded conditions are a 20-logical-core i9, `--release`, and its own test
//! target run alone. `crates/happenstance-testkit/src/concurrency.rs:238` sets
//! `CONTENDERS = 64`, `crates/happenstance-sqlite/tests/concurrency.rs` mounts
//! the family, and `xtask/src/main.rs`'s tests step runs
//! `cargo test --locked --workspace --all-features` — **debug**, with
//! `--test-threads` at its default, so five 64-contender rules overlap. Those
//! are different conditions, and the gap between them has never been measured.
//!
//! # What this crate is
//!
//! `experiments/append-condition`'s `tests/contention_at_64.rs` shape, re-run
//! under the gate's configuration instead of the record's, with two instruments
//! added:
//!
//! * a **counting busy handler** ([`busy`]) that reproduces
//!   `sqliteDefaultBusyCallback`'s back-off schedule exactly and reports, per
//!   contender, the accumulated milliseconds spent inside it. Nothing anywhere in
//!   the tree reports a nonzero busy count today, which is why ADR-0022 §11's own
//!   falsifier — *"re-open if any run ever reports `busy > 0`"* — has never been
//!   able to fire. Making it able to fire is the point;
//! * a **verified core constraint** ([`cores`]), because `taskset` does not
//!   exist on Windows and an affinity call that silently failed would produce a
//!   twenty-core number wearing a two-core label.
//!
//! # What is copied and what is new
//!
//! `candidate`, `strategy`, `tags` and `durability` are copied verbatim from
//! `experiments/append-condition/src/`, with **one** deliberate difference:
//! `configure` chooses between [`busy::counting_busy_handler`] and
//! `Connection::busy_timeout`'s default one, on [`busy::handler`]. The copy is
//! wholesale and the diff is one function, so the control below can be checked
//! against the recorded table rather than merely compared to it — and the
//! `HS_HANDLER=default` arm of that switch is the control *for the instrument*,
//! which reruns the identical harness with nothing installed but SQLite's own
//! handler.
//!
//! Copying rather than depending is what keeps that diff possible:
//! `experiments/append-condition` exposes no door to its `rusqlite::Connection`,
//! and `Connection::busy_handler` takes a bare `fn(i32) -> bool` that has to be
//! installed on the connection before it is used.
//!
//! # The control comes first
//!
//! A debug figure is only a delta if it is a delta *from* something. So
//! `tests/busy_margin.rs` in `--release` at full cores must first reproduce
//! ADR-0022 §11's recorded row — median 2,724,759 µs, maximum 3,496,577 µs,
//! `busy = 0` for `begin-immediate-probe` at 64 contenders — before any
//! low-core figure is read. `run.sh` runs it first for that reason, and
//! `results/README` records what it actually produced rather than what it was
//! supposed to.
//!
//! # What this crate is deliberately not
//!
//! Not a gate step, and it must never become one (CF-34). Not a proposal to
//! change `BUSY_TIMEOUT_MS`, `CONTENDERS`, or anything under `crates/` — nothing
//! outside this directory is touched. Not an adapter: the store here is the same
//! measurement candidate `experiments/append-condition` used, which is not
//! `happenstance-sqlite` and does not claim to be.

#![forbid(unsafe_code)]

mod candidate;
pub mod busy;
pub mod cores;
mod durability;
mod strategy;
mod tags;

pub use busy::{BUSY_TIMEOUT_MS, BusyStats, Handler};
pub use candidate::{CandidateStore, SqliteProbeError};
pub use cores::Constraint;
pub use durability::{Durability, DurabilityRefused, JournalMode, Synchronous};
pub use strategy::{AppendStrategy, BeginImmediateProbe, ConditionalInsert, MonotonicGuard};
pub use tags::{CanonicalBlob, JoinTable, JoinTableGrouped, Json1, TagStorage};
