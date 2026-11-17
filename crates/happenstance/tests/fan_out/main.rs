//! The fan-out runner, `run_fan_out`: N views of one projection type over
//! one read of the log (PS-26, PS-29, PS-30, PS-38; E2E-28, E2E-32).
//!
//! `runs` holds the runner to `run_projection`'s outcome, view by view, and
//! counts the reads it issues. `rules` holds the clause rules, each run
//! against the shipped runner and against the wrong runners it rejects.
//! `pins` holds both runners' source pin (PS-40, E2E-59), here because its
//! failure cases need the recording store `support` already wraps, and
//! `reads` holds a refused checkpoint read, of the pin or the main key.
#![cfg(all(feature = "memory", feature = "json"))]

mod pins;
mod reads;
mod rules;
mod runs;
mod support;
