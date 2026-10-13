//! Which construction of the object a request ran in, and which wasm instance.
//!
//! A cross-restart rule ([`crate::restart`]) passes vacuously if the restart it
//! was written for never happened: the after-half then runs against the same
//! in-memory state as the before-half, and every wrong store that keeps its
//! state in the object passes too. So the object names itself. Each
//! construction mints an [`Incarnation`], the before-half's answer carries it,
//! and the after-half compares it with its own through [`check_restart`] before
//! the rule runs.
//!
//! The wasm instance gets a nonce of its own ([`Incarnation::of_wasm_instance`]).
//! A local eviction (`evictDurableObject`, or `workerd`'s inactivity timer)
//! tears down the object and keeps the module instance; a trap discards the
//! instance. The two nonces together tell an eviction from a trap.

use core::fmt;
use core::str::FromStr;

/// One construction of the object, or one wasm instance, named by a nonce.
///
/// A newtype over the raw bits, private, rather than a bare `u64`: the only
/// operations that mean anything are equality and the text form, and a `u64`
/// would also offer arithmetic and silent mixing with positions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Incarnation(u64);

impl Incarnation {
    /// A fresh nonce from the runtime's `Math.random()`.
    ///
    /// Two draws, each contributing the bits of a double in `[0, 1)`; the
    /// rotation puts the second draw's mantissa over the first draw's exponent
    /// bits, which are nearly constant. A collision between two incarnations of
    /// one object would read as "not restarted", which fails a rule rather than
    /// passing one, so a nonce needs to be distinct, not secret.
    ///
    /// Only meaningful inside a JavaScript runtime: on the host, `js_sys` has
    /// no `Math` to call. Nothing on the host constructs an object, and the
    /// host tests build incarnations with [`Incarnation::from_raw`].
    #[must_use]
    pub fn mint() -> Self {
        let first = worker::js_sys::Math::random().to_bits();
        let second = worker::js_sys::Math::random().to_bits();
        Self(first ^ second.rotate_left(32))
    }

    /// The incarnation whose raw bits are `raw`.
    #[must_use]
    pub const fn from_raw(raw: u64) -> Self {
        Self(raw)
    }

    /// This wasm instance's nonce, minted on first use and kept for the
    /// instance's life.
    ///
    /// A `thread_local!` rather than a `static`: a `OnceCell` is not `Sync`,
    /// and a wasm instance has one thread, so the thread-local *is* the
    /// instance-global without asserting a `Sync` nobody needs.
    #[must_use]
    pub fn of_wasm_instance() -> Self {
        thread_local! {
            static WASM: core::cell::OnceCell<Incarnation> = const { core::cell::OnceCell::new() };
        }
        WASM.with(|cell| *cell.get_or_init(Self::mint))
    }
}

impl fmt::Display for Incarnation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:016x}", self.0)
    }
}

/// Text that is not sixteen hexadecimal digits, handed back.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvalidIncarnation(pub String);

impl fmt::Display for InvalidIncarnation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "not an incarnation (16 hex digits): {:?}", self.0)
    }
}

impl core::error::Error for InvalidIncarnation {}

impl FromStr for Incarnation {
    type Err = InvalidIncarnation;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        // Exactly the form `Display` writes, so a round trip is the only way in.
        if text.len() != 16 || !text.bytes().all(|b| b.is_ascii_hexdigit()) {
            return Err(InvalidIncarnation(text.to_owned()));
        }
        u64::from_str_radix(text, 16)
            .map(Self)
            .map_err(|_| InvalidIncarnation(text.to_owned()))
    }
}

/// What the runner says it did between the two halves of a rule.
///
/// A closed set rather than a `bool`, so a request that says neither is refused
/// by name instead of defaulting to one of them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Expectation {
    /// The runner restarted the object between the halves.
    Restarted,
    /// The runner did nothing between the halves: the control.
    SameInstance,
}

impl FromStr for Expectation {
    type Err = InvalidExpectation;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        match text {
            "restarted" => Ok(Self::Restarted),
            "same" => Ok(Self::SameInstance),
            other => Err(InvalidExpectation(other.to_owned())),
        }
    }
}

/// An `expect=` value that is neither `restarted` nor `same`, handed back.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvalidExpectation(pub String);

impl fmt::Display for InvalidExpectation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "expect= is restarted or same, not {:?}", self.0)
    }
}

impl core::error::Error for InvalidExpectation {}

/// The runner's claim and the object's own witness disagree.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RestartMismatch {
    /// A restart was expected and the after-half ran in the before-half's
    /// instance: a rule run now would prove nothing about a restart.
    NotObserved {
        /// The incarnation both halves ran in.
        incarnation: Incarnation,
    },
    /// No restart was expected and the instance changed anyway: the control is
    /// not a control.
    Unexpected {
        /// The before-half's incarnation.
        before: Incarnation,
        /// The after-half's incarnation.
        now: Incarnation,
    },
}

impl fmt::Display for RestartMismatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotObserved { incarnation } => {
                write!(f, "restart not observed: both halves ran in {incarnation}")
            }
            Self::Unexpected { before, now } => {
                write!(f, "restart not expected: {before} became {now}")
            }
        }
    }
}

impl core::error::Error for RestartMismatch {}

/// Whether the instance the after-half runs in (`now`) is the one `expect`
/// says it should be, given the before-half's (`before`).
///
/// # Errors
///
/// [`RestartMismatch::NotObserved`] when a restart was expected and the
/// incarnation is unchanged; [`RestartMismatch::Unexpected`] when none was and
/// it changed.
pub fn check_restart(
    expect: Expectation,
    before: Incarnation,
    now: Incarnation,
) -> Result<(), RestartMismatch> {
    match (expect, before == now) {
        (Expectation::Restarted, false) | (Expectation::SameInstance, true) => Ok(()),
        (Expectation::Restarted, true) => Err(RestartMismatch::NotObserved { incarnation: now }),
        (Expectation::SameInstance, false) => Err(RestartMismatch::Unexpected { before, now }),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        Expectation, Incarnation, InvalidExpectation, InvalidIncarnation, RestartMismatch,
        check_restart,
    };

    const A: Incarnation = Incarnation::from_raw(0x0123_4567_89ab_cdef);
    const B: Incarnation = Incarnation::from_raw(7);

    #[test]
    fn an_incarnation_round_trips_through_its_text() {
        assert_eq!(A.to_string(), "0123456789abcdef");
        assert_eq!(B.to_string(), "0000000000000007");
        assert_eq!("0123456789abcdef".parse(), Ok(A));
        assert_eq!(B.to_string().parse(), Ok(B));
    }

    #[test]
    fn an_incarnation_refuses_text_its_display_never_writes() {
        for text in [
            "",
            "7",
            "+123456789abcdef",
            "0123456789abcdeg",
            "0123456789abcdef0",
        ] {
            assert_eq!(
                text.parse::<Incarnation>(),
                Err(InvalidIncarnation(text.to_owned())),
                "{text:?}"
            );
        }
    }

    #[test]
    fn an_expectation_is_restarted_or_same_and_nothing_else() {
        assert_eq!("restarted".parse(), Ok(Expectation::Restarted));
        assert_eq!("same".parse(), Ok(Expectation::SameInstance));
        assert_eq!(
            "".parse::<Expectation>(),
            Err(InvalidExpectation(String::new()))
        );
    }

    #[test]
    fn a_restart_is_observed_only_when_the_incarnation_changed() {
        assert_eq!(check_restart(Expectation::Restarted, A, B), Ok(()));
        assert_eq!(
            check_restart(Expectation::Restarted, A, A),
            Err(RestartMismatch::NotObserved { incarnation: A })
        );
    }

    #[test]
    fn the_control_holds_only_when_the_incarnation_did_not_change() {
        assert_eq!(check_restart(Expectation::SameInstance, A, A), Ok(()));
        assert_eq!(
            check_restart(Expectation::SameInstance, A, B),
            Err(RestartMismatch::Unexpected { before: A, now: B })
        );
    }
}
