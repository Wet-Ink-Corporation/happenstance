//! Every arm whose wait is allowed to count is a **conformant** store, under
//! **both** busy handlers.
//!
//! # Why this file exists in a crate that only measures waiting
//!
//! `experiments/append-condition/tests/candidates_are_conformant.rs` runs first
//! for the reason its own module doc gives: a wrong arm is always the fastest.
//! That hazard is real here too — the two arms below are copied from that crate
//! and their strategies are exactly the thing a shortcut would break — but this
//! crate adds a second one that the copy cannot inherit, and it is the reason
//! this file is not merely the sibling's file again.
//!
//! **This crate replaces the store's busy handler.** `src/busy.rs` installs a
//! counting transcription of `sqliteDefaultBusyCallback` in place of the one
//! `Connection::busy_timeout` installs, and that is a change to the store, on
//! the write path, in the exact place contention is resolved. A handler that
//! returned `false` one entry too early would turn a contended-but-fine append
//! into an error; one that returned `true` forever would convert the crate's
//! bounded races into a hang. Either would move every number in `results/` and
//! neither is visible in a timing table — a faster race is what both of them
//! look like.
//!
//! So conformance runs under both handlers, and `run.sh` runs it before it runs
//! a clock:
//!
//! * `HS_HANDLER` unset — the counting handler, which produces every
//!   `wait_ms_*` figure in `results/`;
//! * `HS_HANDLER=default` — SQLite's own, which produces every `busy` figure,
//!   including the headroom sweep where ADR-0022 §11's re-open trigger fires.
//!
//! The handler is chosen once per **process** from the environment
//! (`busy::handler`'s `OnceLock`), so the two runs have to be two `cargo test`
//! invocations rather than two modules here. `run.sh` is where that pairing
//! lives; this file is what each of them runs.
//!
//! # Which crosses, and why only two
//!
//! Two, and they are exactly the two the experiment races — not five as the
//! sibling runs:
//!
//! * `BeginImmediateProbe` at `JoinTable`, the arm ADR-0022 §11's recorded row
//!   was produced by and the arm every control and matrix row here holds fixed;
//! * `MonotonicGuard` at `JoinTable`, the arm `happenstance-sqlite` actually
//!   ships (ADR-0022 §4) and therefore the one whose margin applies to the
//!   adapter.
//!
//! `ConditionalInsert`, `CanonicalBlob`, `Json1` and `JoinTableGrouped` compile
//! in this crate because `src/` was copied wholesale, but no row in `results/`
//! is produced by any of them. Running conformance over an arm this crate never
//! times would be a green tick with nothing behind it, and the sibling
//! experiment already covers all five of its own.
//!
//! Run it with `cargo test --manifest-path
//! experiments/busy-timeout-margin/Cargo.toml --test arms_are_conformant`.

mod support;

use busy_timeout_margin_probes::{BeginImmediateProbe, JoinTable, MonotonicGuard};
use support::CandidateFixture;

happenstance_testkit::event_store_conformance!(
    mod_name = begin_immediate_probe_join_table,
    fixture = CandidateFixture::<BeginImmediateProbe, JoinTable>::new()
);

happenstance_testkit::event_store_conformance!(
    mod_name = monotonic_guard_join_table,
    fixture = CandidateFixture::<MonotonicGuard, JoinTable>::new()
);
