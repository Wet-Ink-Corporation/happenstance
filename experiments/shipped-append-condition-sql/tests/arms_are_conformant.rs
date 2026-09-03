//! Every shape whose figure is allowed to count is a **conformant** store.
//!
//! This runs first in `run.sh`, before any timer, and that ordering is the whole
//! control. A shape that answers a boundary question faster by answering it
//! *less completely* wins every benchmark and is worthless — and that is not
//! hypothetical here. The two bounded shapes add a predicate that **restricts the
//! matched set**, which is precisely the shape of a guard that has stopped
//! noticing conflicts:
//!
//! * `append_is_rejected_when_a_matching_event_exists_after_the_boundary` fails
//!   if the restriction is off by one — `>=` written for `>`, or the boundary
//!   bound one too high.
//! * `racing_conditional_appends_elect_one_winner` fails if it lets two writers
//!   through.
//! * `condition_violation_names_the_conflicting_position` fails if the shape
//!   answers *violated* but names the wrong event, which is what a chain that
//!   restricted the wrong subquery would do.
//! * `read_returns_events_matching_any_query_item` and the whole read family fail
//!   if the boundary predicate leaks onto the read path — where the boundary is
//!   zero, and every position is above it, so a shape that emits the clause is
//!   exercised by all 89 rules rather than only by the ones carrying a boundary.
//!
//! Four shapes × 89 rules = 356 tests. If any is red, that shape's figure is
//! discarded and the failure is recorded in `results/` as what that shape costs —
//! `experiments/append-condition`'s EC-003, carried forward unchanged.
//!
//! Run it with `cargo test --manifest-path
//! experiments/shipped-append-condition-sql/Cargo.toml --test arms_are_conformant`.

mod support;

use support::ProbeFixture;

// The index is `Shape::ALL`'s, and `support::shape_of` is what ties the two
// together — so a shape reordered in `chain.rs` renames these files rather than
// silently running one shape twice.

happenstance_testkit::event_store_conformance!(
    mod_name = chain_as_shipped,
    fixture = ProbeFixture::<0>::new()
);

happenstance_testkit::event_store_conformance!(
    mod_name = chain_bounded_seed,
    fixture = ProbeFixture::<1>::new()
);

happenstance_testkit::event_store_conformance!(
    mod_name = chain_bounded_all_arms,
    fixture = ProbeFixture::<2>::new()
);

happenstance_testkit::event_store_conformance!(
    mod_name = grouped_adr0022,
    fixture = ProbeFixture::<3>::new()
);
