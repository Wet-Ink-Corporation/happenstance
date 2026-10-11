//! VT-14's falsifier, run against the validator `happenstance-core` ships.
//!
//! Two questions, answered separately (see `README.md` for the pre-registered
//! criteria):
//!
//! * [`sweep`] is E11 reproduced: every Unicode scalar value, in each
//!   [`Position`], through [`Constructor::verdict`], compared against the
//!   oracle [`expected`]. It asks whether the shipped check *is* VT-14.
//! * [`check_corpus`] and [`check_pairs`] run the hand-written Arabic, Hebrew
//!   and Persian [`CORPUS`]. They ask whether a legitimate RTL identifier is
//!   refused — the question VT-14's marker asks.
//!
//! Every judge is a closure, so the tests can hand the same harness a known
//! wrong validator and watch it fail.

mod blocks;
mod corpus;
mod sweep;
mod verdict;

pub use blocks::{Block, RTL_BLOCKS};
pub use corpus::{
    CORPUS, CorpusReport, Entry, Escape, Insertion, MIN_ENTRIES_PER_SCRIPT, MIN_MIXED_PER_SCRIPT,
    NEGATIVE_INSERTS, PAIRS, Pair, PairRefusal, Refusal, Script, ShapeProblem, check_corpus,
    check_pairs, shape_problems,
};
pub use sweep::{Disagreement, MAX_RECORDED, Position, SCALAR_VALUE_COUNT, SweepReport, sweep};
pub use verdict::{Constructor, Verdict, expected, key_value_verdict};
