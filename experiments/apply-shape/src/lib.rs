//! The apply-shape spike: can the typed layer's `Projection::apply` become
//! `async`, take a batch it can issue statements through, and be handed a
//! [`Delivered`] envelope with no local position?
//!
//! Not a workspace member, not in the gate, and not a proposal for this
//! crate's own API: [`shape`] declares the trait the projection-apply brief
//! recommends so the compiler can argue with it, and [`runner`] drives it. See
//! `README.md` for the question, the criteria, the runs and the verdict.

pub mod domain;
pub mod edge;
pub mod runner;
pub mod shape;

#[cfg(feature = "demonstrate-refusals")]
mod demonstrate;

pub use runner::{Ran, RunError, RunErrorFor, run};
pub use shape::{ApplyFailure, Delivered, Policy, Projection, SendProjection};
