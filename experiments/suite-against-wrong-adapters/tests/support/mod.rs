//! Shared test support: the fixtures the suite is handed, and the harness that
//! turns a failing rule back into data instead of a dead test binary.

// Shared by three test targets, and no one target uses every item:
// `controls_are_conformant.rs` reaches only `CorrectFixture`, and
// `defect_is_real.rs` reaches no probe machinery at all. A shared `tests/`
// module is compiled once per target, so the alternative to this allow is three
// copies of the harness. The precedent is
// `experiments/append-condition/tests/support/mod.rs:24`.
#![allow(dead_code)]

pub mod wrong_fixtures;
pub mod harness;
