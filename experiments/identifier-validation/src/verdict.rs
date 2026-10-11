//! What a validator said about a value, what VT-14 says it should have said,
//! and the two shipped constructors that are asked.

use happenstance_core::{EventType, InvalidEventType, InvalidTag, Tag};

/// One validator's answer for one value, in VT-14's vocabulary.
///
/// `EventType` and `Tag` have different error enums with the same refusals;
/// this is the common ground both map onto, so their verdicts compare directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verdict {
    /// Accepted, and `as_str` read the value back byte-for-byte.
    Accepted,
    /// Accepted, but `as_str` returned something else. VT-14 does not allow a
    /// constructor to normalise, so this is a finding wherever it appears.
    AcceptedButAltered,
    /// Refused as empty.
    Empty,
    /// Refused as longer than the byte bound.
    TooLong,
    /// Refused as containing a Unicode `Cc` character.
    ControlCharacter,
    /// Refused as containing an explicit bidirectional control: one of the
    /// nine codepoints U+202A–U+202E and U+2066–U+2069.
    BidirectionalControl,
    /// Refused for a reason VT-14 does not name. The error enums are
    /// `#[non_exhaustive]`, so a variant added later lands here rather than
    /// failing to compile — and is reported, because it is not in the clause.
    OtherRefusal,
}

/// VT-14's verdict for one scalar value, computed without the shipped code.
///
/// `char::is_control` is Unicode general category `Cc` exactly, and the
/// bidirectional half is the clause's two ranges, U+202A–U+202E and
/// U+2066–U+2069, written out again. Those ranges hold nine codepoints, and
/// the oracle follows the ranges: ADR-0015's prose calls its E11 oracle a
/// "seven-codepoint closed list", which is the same miscount VT-14's text makes.
pub fn expected(c: char) -> Verdict {
    if c.is_control() {
        Verdict::ControlCharacter
    } else if matches!(c, '\u{202A}'..='\u{202E}' | '\u{2066}'..='\u{2069}') {
        Verdict::BidirectionalControl
    } else {
        Verdict::Accepted
    }
}

/// The two public constructors VT-14 binds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Constructor {
    /// [`EventType::new`].
    EventType,
    /// [`Tag::new`].
    Tag,
}

impl Constructor {
    /// Both constructors, in a fixed order.
    pub const ALL: [Self; 2] = [Self::EventType, Self::Tag];

    /// Asks this constructor about `value` and reports its answer.
    pub fn verdict(self, value: &str) -> Verdict {
        match self {
            Self::EventType => match EventType::new(value) {
                Ok(accepted) => read_back(accepted.as_str(), value),
                Err(refusal) => from_event_type(&refusal),
            },
            Self::Tag => match Tag::new(value) {
                Ok(accepted) => read_back(accepted.as_str(), value),
                Err(refusal) => from_tag(&refusal),
            },
        }
    }
}

/// [`Tag::key_value`]'s verdict on a pair, read back against `key:value`.
pub fn key_value_verdict(key: &str, value: &str) -> Verdict {
    match Tag::key_value(key, value) {
        Ok(accepted) => {
            let read = accepted.as_str();
            let joined = read
                .strip_prefix(key)
                .and_then(|rest| rest.strip_prefix(':'))
                .is_some_and(|rest| rest == value);
            if joined {
                Verdict::Accepted
            } else {
                Verdict::AcceptedButAltered
            }
        }
        Err(refusal) => from_tag(&refusal),
    }
}

fn read_back(read: &str, written: &str) -> Verdict {
    if read == written {
        Verdict::Accepted
    } else {
        Verdict::AcceptedButAltered
    }
}

fn from_event_type(refusal: &InvalidEventType) -> Verdict {
    match refusal {
        InvalidEventType::Empty => Verdict::Empty,
        InvalidEventType::TooLong { .. } => Verdict::TooLong,
        InvalidEventType::ControlCharacter => Verdict::ControlCharacter,
        InvalidEventType::BidirectionalControl => Verdict::BidirectionalControl,
        _ => Verdict::OtherRefusal,
    }
}

fn from_tag(refusal: &InvalidTag) -> Verdict {
    match refusal {
        InvalidTag::Empty => Verdict::Empty,
        InvalidTag::TooLong { .. } => Verdict::TooLong,
        InvalidTag::ControlCharacter => Verdict::ControlCharacter,
        InvalidTag::BidirectionalControl => Verdict::BidirectionalControl,
        _ => Verdict::OtherRefusal,
    }
}
