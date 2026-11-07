//! The fan-out runner, `run_fan_out`: N views of one projection type over
//! one read of the log (PS-26, PS-29, PS-30, PS-38; E2E-28, E2E-32).
//!
//! `runs` holds the runner to `run_projection`'s outcome, view by view, and
//! counts the reads it issues. `rules` holds the clause rules, each run
//! against the shipped runner and against the wrong runners it rejects.
#![cfg(all(feature = "unstable-projection", feature = "memory", feature = "json"))]

mod rules;
mod runs;
mod support;
