//! The seal behind [`Boundary`](crate::Boundary).
//!
//! `Sealed` is `pub` inside a private module, which is the only spelling that
//! is both nameable as a supertrait of a public trait and unnameable outside
//! this crate. `pub(crate)` would make `Boundary` "more private than" its own
//! bound; a public marker trait would be no seal at all.

use crate::DecisionModel;

/// Implemented for every decision model, and for tuples of them.
///
/// Nothing else can implement it, because nothing else can name it.
#[allow(
    unreachable_pub,
    reason = "pub in a private module is the sealed-supertrait spelling"
)]
pub trait Sealed {}

impl<M: DecisionModel> Sealed for M {}
