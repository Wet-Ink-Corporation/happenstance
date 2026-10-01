//! Built only under `--features demonstrate-refusals`, and expected to fail.
//!
//! The two `compile_fail` doctests claim `E0599` and `E0277`; rustdoc does not
//! check those claims (RS-62-1). These are the same two calls in a real build,
//! so `results/refusals.txt` is the compiler's own words.

use crate::edge::EdgeTally;
use crate::{Delivered, SendProjection};

/// Asks the envelope for the local position it deliberately does not carry.
/// Expected: `error[E0599]: no method named `position``.
pub fn leak(delivered: &Delivered<u8>) {
    let _ = delivered.position();
}

/// Asks the `Send` flavour of a `!Send` projection.
/// Expected: `error[E0277]`, `EdgeTally: SendProjection` is not satisfied.
pub fn spawn_the_edge() {
    fn spawnable<P: SendProjection>(_: &P) {}
    spawnable(&EdgeTally::new("edge"));
}
