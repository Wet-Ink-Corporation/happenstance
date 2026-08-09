//! Shared validation for the two identifier types, written so that one body
//! serves both the runtime constructor and the `const` one.
//!
//! VT-32 requires `from_static` to be *exactly* as strong as `new`, and the
//! cheapest way to guarantee that is to have one function and no second copy of
//! the rules. That forces the whole check into a `const fn`, which is why this
//! is a byte walk with a `while` loop rather than the `chars().any(..)` it
//! replaces: iterator adaptors and `char` predicates are not available in a
//! `const` context.
//!
//! The byte walk was checked against the `char` walk over all 1,112,064 Unicode
//! scalar values, each tested alone and embedded between two ASCII letters,
//! with zero disagreements (ADR-0015, experiment E11). The two positions matter
//! because a lead byte at index 0 and a lead byte mid-string take different
//! paths through the loop.

/// Why a candidate identifier was refused, or that it was not.
///
/// Deliberately not an error type: `EventType` and `Tag` have their own, with
/// different names for the same refusals, and mapping one enum onto two is
/// cheaper than making them share an error they would both have to re-export.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Refusal {
    /// Nothing is wrong with the value.
    Accepted,
    /// The value is empty.
    Empty,
    /// The value is longer than the caller's byte bound.
    TooLong,
    /// The value contains a character in Unicode general category `Cc`.
    ControlCharacter,
    /// The value contains one of the seven explicit bidirectional formatting
    /// controls.
    BidirectionalControl,
}

/// Applies VT-14's rules to `value`, in a `const` context or out of one.
///
/// Rejects, in this order: an empty value; one longer than `max_len` bytes; any
/// character in Unicode general category `Cc`; and the seven explicit
/// bidirectional formatting controls U+202A–U+202E and U+2066–U+2069.
///
/// It deliberately does **not** reject Unicode category `Cf` generally, which
/// VT-14 forbids in terms: a blanket `Cf` ban would take U+200C and U+200D with
/// it, and Persian, Hindi and emoji sequences require both. The seven below are
/// the only `Cf` codepoints no script needs, and each is a log-spoofing vector —
/// an event type that renders as one thing in every console and matches another
/// in every query.
///
/// The closed list is also what makes this affordable. `core` exposes no `Cf`
/// predicate, and pulling in a Unicode tables crate to get one would be the
/// largest dependency in the contract crate by an order of magnitude.
pub(crate) const fn check(value: &str, max_len: usize) -> Refusal {
    if value.is_empty() {
        return Refusal::Empty;
    }
    if value.len() > max_len {
        return Refusal::TooLong;
    }

    // `value` is valid UTF-8 by construction, so the multi-byte arms below can
    // read ahead without re-validating: a 0xC2 or 0xE2 lead byte is guaranteed
    // to be followed by its continuation bytes.
    let bytes = value.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];

        // C0 controls and DEL. One byte each.
        if b < 0x20 || b == 0x7F {
            return Refusal::ControlCharacter;
        }

        // C1 controls, U+0080–U+009F. Two bytes, and the reason `is_control`
        // was never an ASCII test: these are `Cc` and are not ASCII.
        if b == 0xC2 && i + 1 < bytes.len() && matches!(bytes[i + 1], 0x80..=0x9F) {
            return Refusal::ControlCharacter;
        }

        // U+202A–U+202E (the overrides) and U+2066–U+2069 (the isolates).
        // Three bytes each, sharing the 0xE2 lead.
        if b == 0xE2 && i + 2 < bytes.len() {
            let second = bytes[i + 1];
            let third = bytes[i + 2];
            if (second == 0x80 && matches!(third, 0xAA..=0xAE))
                || (second == 0x81 && matches!(third, 0xA6..=0xA9))
            {
                return Refusal::BidirectionalControl;
            }
        }

        i += 1;
    }

    Refusal::Accepted
}

#[cfg(test)]
mod tests {
    use super::{Refusal, check};

    const MAX: usize = 255;

    #[test]
    fn accepts_an_ordinary_identifier() {
        assert!(matches!(check("StudentSubscribed", MAX), Refusal::Accepted));
    }

    #[test]
    fn rejects_empty_and_over_long() {
        assert!(matches!(check("", MAX), Refusal::Empty));
        assert!(matches!(check("ab", 1), Refusal::TooLong));
    }

    #[test]
    fn rejects_c0_del_and_c1_controls() {
        assert!(matches!(check("a\u{0}b", MAX), Refusal::ControlCharacter));
        assert!(matches!(check("a\u{7F}b", MAX), Refusal::ControlCharacter));
        // U+0085 NEL is `Cc` and is not ASCII, which is the case the old
        // docstrings claimed was out of scope and the old code rejected anyway.
        assert!(matches!(check("a\u{85}b", MAX), Refusal::ControlCharacter));
    }

    #[test]
    fn rejects_all_seven_bidirectional_controls() {
        for c in ['\u{202A}', '\u{202B}', '\u{202C}', '\u{202D}', '\u{202E}'] {
            let mid = alloc::format!("a{c}b");
            assert!(matches!(check(&mid, MAX), Refusal::BidirectionalControl));
        }
        for c in ['\u{2066}', '\u{2067}', '\u{2068}', '\u{2069}'] {
            let mid = alloc::format!("a{c}b");
            assert!(matches!(check(&mid, MAX), Refusal::BidirectionalControl));
        }
    }

    #[test]
    fn accepts_the_format_characters_scripts_need() {
        // VT-14 forbids a blanket `Cf` rejection. ZWNJ and ZWJ are `Cf` and are
        // required by Persian, Hindi and emoji sequences.
        assert!(matches!(check("a\u{200C}b", MAX), Refusal::Accepted));
        assert!(matches!(check("a\u{200D}b", MAX), Refusal::Accepted));
        // U+200B stays legal too, and is the residual hazard VT-14 records:
        // two visually identical tags are two consistency boundaries.
        assert!(matches!(check("a\u{200B}b", MAX), Refusal::Accepted));
    }

    #[test]
    fn accepts_neighbours_of_the_closed_list() {
        // U+2029 and U+202F bracket the override run; U+2065 and U+206A bracket
        // the isolate run. A range written one byte wide either way takes these.
        assert!(matches!(check("a\u{2029}b", MAX), Refusal::Accepted));
        assert!(matches!(check("a\u{202F}b", MAX), Refusal::Accepted));
        assert!(matches!(check("a\u{2065}b", MAX), Refusal::Accepted));
        assert!(matches!(check("a\u{206A}b", MAX), Refusal::Accepted));
    }

    #[test]
    fn finds_an_offender_at_either_end() {
        assert!(matches!(
            check("\u{202E}ab", MAX),
            Refusal::BidirectionalControl
        ));
        assert!(matches!(
            check("ab\u{202E}", MAX),
            Refusal::BidirectionalControl
        ));
    }

    // Asserting on a constant is the entire point here: the claim is that the
    // check runs at compile time, and the only way to state that is to make the
    // compiler run it. A non-`const` call would still pass an ordinary
    // assertion, which is what would make this test vacuous.
    #[expect(
        clippy::assertions_on_constants,
        reason = "the constant is the subject: it proves `check` is const-evaluable"
    )]
    #[test]
    fn evaluates_in_a_const_context() {
        const VERDICT: bool = matches!(check("CourseDefined", MAX), Refusal::Accepted);
        assert!(VERDICT);
    }
}
