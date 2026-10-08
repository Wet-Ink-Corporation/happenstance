//! VT-35's property test: `ProjectionId::new` agrees with an oracle written in
//! `char` predicates, independently of the `const` byte walk it runs.
//!
//! Experiment E11 proved the byte walk equal to the `char` walk for VT-14's
//! rules alone. What it did not prove is the **composition** this file pins: the
//! VT-14 checks in their documented order, then the reserved prefixes on top,
//! with nothing normalised on the way through. The public API only, so the
//! oracle cannot borrow the implementation's helpers (TST-02).

use happenstance_core::{InvalidProjectionId, MAX_PROJECTION_ID_LEN, ProjectionId};
use proptest::prelude::*;

/// The explicit bidirectional formatting controls VT-14 refuses: U+202A–U+202E
/// and U+2066–U+2069.
const BIDI: [char; 9] = [
    '\u{202A}', '\u{202B}', '\u{202C}', '\u{202D}', '\u{202E}', '\u{2066}', '\u{2067}', '\u{2068}',
    '\u{2069}',
];

/// What VT-35 says `new` returns, in its documented order, stated over `char`s.
///
/// A value carrying both a control character and a bidirectional control is
/// refused for whichever comes **first**, reading left to right: VT-14's walk
/// is one pass over the value, not one pass per class. This file's first run
/// found that, against an oracle that checked every `Cc` before any bidi.
fn oracle(s: &str) -> Result<&str, InvalidProjectionId> {
    if s.is_empty() {
        return Err(InvalidProjectionId::Empty);
    }
    if s.len() > MAX_PROJECTION_ID_LEN {
        return Err(InvalidProjectionId::TooLong { len: s.len() });
    }
    for c in s.chars() {
        if c.is_control() {
            return Err(InvalidProjectionId::ControlCharacter);
        }
        if BIDI.contains(&c) {
            return Err(InvalidProjectionId::BidirectionalControl);
        }
    }
    for prefix in ["happenstance/", "sync/"] {
        if s.as_bytes().starts_with(prefix.as_bytes()) {
            return Err(InvalidProjectionId::Reserved { prefix });
        }
    }
    Ok(s)
}

/// Characters chosen to sit on every boundary the validator decides.
fn edge_char() -> impl Strategy<Value = char> {
    prop::sample::select(vec![
        'a', '/', '\0', '\n', '\u{85}', '\u{200C}', '\u{200D}', '\u{202E}', '\u{2066}', '\u{2069}',
        'é', '👩',
    ])
}

/// Candidate ids: arbitrary strings, the same behind a prefix near the
/// reservation, strings from the edge alphabet, and lengths around the bound.
fn candidate() -> impl Strategy<Value = String> {
    prop_oneof![
        any::<String>(),
        (
            prop::sample::select(vec!["sync/", "happenstance/", "Sync/", "sync", ""]),
            any::<String>(),
        )
            .prop_map(|(prefix, rest)| format!("{prefix}{rest}")),
        prop::collection::vec(edge_char(), 0..16).prop_map(|chars| chars.into_iter().collect()),
        (250_usize..=260, prop::option::of(edge_char())).prop_map(|(len, tail)| {
            let mut s = "x".repeat(len);
            s.extend(tail);
            s
        }),
    ]
}

proptest! {
    #[test]
    fn new_agrees_with_a_char_level_oracle(s in candidate()) {
        let expected = oracle(&s).map(str::to_owned);
        let actual = ProjectionId::new(s).map(|id| id.as_str().to_owned());
        prop_assert_eq!(actual, expected);
    }
}
