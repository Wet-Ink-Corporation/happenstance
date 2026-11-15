//! The scenarios of ES-39's reader experiment (#156), compiled as one binary.
//!
//! Each test asserts what was **observed**. Where the observation is a hazard,
//! its doc comment says so: the assertion is a record, not a desired
//! behaviour, and a later tree that changes the behaviour turns it red so the
//! record is re-read.

mod floor;
mod r1_runner;
mod r2_ingest_destination;
mod r2_ingest_source;
mod r3_decision;
#[cfg(test)]
mod support;
