//! How a sync rule fails: a value, not a panic.

use alloc::string::String;
use core::fmt;

/// A peer did something a sync rule forbids.
///
/// Every rule in [`rules`](crate::rules) returns `Result<RuleOutcome,
/// Violation>`, and every emitter's test function returns
/// `Result<(), Violation>`, so a failure reaches libtest — or
/// `wasm-bindgen-test`, which accepts the same shape — as the test's error,
/// named by the clause it breaks. The event-store and projection families fail
/// by panicking instead; this family returns the failure because a rule that
/// cannot fail without a panic would need a `panic!` in this crate's library
/// code, which the workspace's standard forbids.
///
/// `Debug` prints what `Display` prints. Libtest reports a returned error
/// through `Debug`, and a derived one would bury the message in field syntax.
#[derive(Clone, PartialEq, Eq)]
pub struct Violation {
    clause: &'static str,
    message: String,
}

impl Violation {
    /// A breach of `clause`, described by `message`.
    #[must_use]
    pub fn new(clause: &'static str, message: impl Into<String>) -> Self {
        Self {
            clause,
            message: message.into(),
        }
    }

    /// The specification clause the peer broke, as `"SY-1"`.
    #[must_use]
    pub const fn clause(&self) -> &'static str {
        self.clause
    }

    /// What the peer did, in the rule's words.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for Violation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} violated: {}", self.clause, self.message)
    }
}

impl fmt::Debug for Violation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl core::error::Error for Violation {}

/// `Ok(())` when `holds`, otherwise a [`Violation`] of `clause` whose message
/// is built only on the failing path.
pub(crate) fn ensure(
    holds: bool,
    clause: &'static str,
    message: impl FnOnce() -> String,
) -> Result<(), Violation> {
    if holds {
        Ok(())
    } else {
        Err(Violation::new(clause, message()))
    }
}
