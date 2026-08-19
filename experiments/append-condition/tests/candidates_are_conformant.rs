//! Every arm whose figure is allowed to count is a **conformant** store.
//!
//! A wrong arm is always the fastest. That is not a cynical remark, it is the
//! reason this file exists and runs first: an append-condition strategy that
//! skips the probe, or a tag storage that answers a superset query with an
//! equality, wins every benchmark in this crate and is worthless. AC-001 makes
//! passing the suite the precondition for a figure counting, and EC-003 says
//! what happens to an arm that is fast and fails — its number is discarded and
//! the failure goes in the record as what that arm costs.
//!
//! # Which crosses run here, and why not all nine
//!
//! Five of the nine combinations, chosen so that every arm on both axes is
//! covered by at least one conformant run:
//!
//! * all three **strategies** at the join table, which is the tag storage the
//!   architecture brief recommends and the one the write path is most sensitive
//!   to;
//! * the chosen strategy at all three **tag storages**.
//!
//! The four remaining crosses vary two things at once, and a conformance run
//! that varies two things tells you less than either of the runs above.
//!
//! Run it with `cargo test --manifest-path experiments/append-condition/Cargo.toml
//! --test candidates_are_conformant`.

mod support;

use append_condition_probes::{
    BeginImmediateProbe, CanonicalBlob, ConditionalInsert, JoinTable, Json1, MonotonicGuard,
};
use support::CandidateFixture;

// --- the three strategies, at the join table ---------------------------------

happenstance_testkit::event_store_conformance!(
    mod_name = begin_immediate_probe_join_table,
    fixture = CandidateFixture::<BeginImmediateProbe, JoinTable>::new()
);

happenstance_testkit::event_store_conformance!(
    mod_name = conditional_insert_join_table,
    fixture = CandidateFixture::<ConditionalInsert, JoinTable>::new()
);

happenstance_testkit::event_store_conformance!(
    mod_name = monotonic_guard_join_table,
    fixture = CandidateFixture::<MonotonicGuard, JoinTable>::new()
);

// --- the three tag storages, at the recommended strategy ---------------------

happenstance_testkit::event_store_conformance!(
    mod_name = begin_immediate_probe_canonical_blob,
    fixture = CandidateFixture::<BeginImmediateProbe, CanonicalBlob>::new()
);

happenstance_testkit::event_store_conformance!(
    mod_name = begin_immediate_probe_json1,
    fixture = CandidateFixture::<BeginImmediateProbe, Json1>::new()
);
