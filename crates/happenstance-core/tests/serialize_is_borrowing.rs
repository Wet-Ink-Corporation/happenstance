//! `Serialize` writes what it was handed; it does not copy it first.
//!
//! # What this file checks, and what it cannot
//!
//! The defect it exists for is measured in heap operations: encoding one 64-tag
//! `SequencedEvent` to postcard cost **140 heap operations to produce 587
//! bytes**, of which **130 produced no output**, because every `Serialize` impl
//! in this crate built an *owned* wire mirror by cloning each field out of the
//! value it had been handed by reference — and `SequencedEvent` did it twice,
//! once into `SequencedEventWire` and once more through the derived impl on that
//! mirror. The full table, across five tag counts and both formats, is
//! `experiments/event-clone-allocations/results/clone-cost.md`.
//!
//! **That measurement cannot be made here.** Counting allocations needs a
//! `#[global_allocator]`, `GlobalAlloc` is an unsafe trait, and this workspace
//! sets `unsafe_code = "forbid"` — which `allow` cannot lift, by design. It is
//! also what CF-34 asks for: performance is measured by a separate harness that
//! is not part of the conformance bar. So the number lives in `experiments/`,
//! outside the gate, and this file holds the two things the gate *can* hold.
//!
//! **One: the control.** The two tag regimes must encode to identical bytes.
//! `Tag::new` ends at `Cow::Owned` and `Tag::from_static` at `Cow::Borrowed`;
//! the experiment subtracts one arm's allocation count from the other's, and
//! that subtraction means nothing unless the outputs are equal. It is asserted
//! here as well as there, because the experiment is outside the gate and this is
//! the half that would silently invalidate every published figure.
//!
//! **Two: the shape.** A `Serialize` impl in this crate copies nothing. That is
//! read out of the source, which is unusual and is the point: the thing being
//! forbidden is a *construction*, the wire mirrors are private, the bytes never
//! move, and so no behavioural test in any format can see the difference. A
//! reviewer is the only other instrument, and reviewers passed this five times.

#![cfg(feature = "serde")]

use happenstance_core::bytes::Bytes;
use happenstance_core::{
    AppendCondition, Event, EventId, EventType, Query, QueryItem, RecordedAt, SequencePosition,
    SequencedEvent, StoreId, Tag, Tags,
};

// ---------------------------------------------------------------------------
// The two arms
// ---------------------------------------------------------------------------

/// VT-22's floor: the tag count every conformant adapter must accept, and so the
/// count a claim about tags is quoted at.
const TAGS: usize = 64;

/// The same sixty-four tags, in the two `Cow` variants.
///
/// The borrowed arm leaks its strings because `Tag::from_static` takes
/// `&'static str` and the point of the arm is the `Cow::Borrowed` variant, not
/// the literal. Sixty-four leaks in one test process, all of them before any
/// comparison, is the price of having the arm at all.
fn tags(owned: bool) -> Tags {
    if owned {
        let pairs: Vec<(String, String)> = (0..TAGS)
            .map(|index| (format!("k{index:02}"), format!("v{index:02}")))
            .collect();
        Tags::from_pairs(
            pairs
                .iter()
                .map(|(key, value)| (key.as_str(), value.as_str()))
                .collect::<Vec<_>>(),
        )
        .expect("the pairs are valid")
    } else {
        (0..TAGS)
            .map(|index| Tag::from_static(format!("k{index:02}:v{index:02}").leak()))
            .collect()
    }
}

fn event_type(owned: bool) -> EventType {
    if owned {
        EventType::new("StudentSubscribed").expect("valid")
    } else {
        EventType::from_static("StudentSubscribed")
    }
}

fn event(owned: bool) -> Event {
    Event::new(event_type(owned), Bytes::from_static(b"{}"))
        .expect("the event type is already validated")
        .with_tags(tags(owned))
}

/// A fixed identity, so byte-identity is not defeated by a minted `StoreId` or a
/// wall-clock `RecordedAt`.
fn sequenced(event: Event) -> SequencedEvent {
    let position = SequencePosition::new(42).expect("42 is non-zero");
    SequencedEvent::new(
        position,
        EventId::new(StoreId::from_bytes([7; 16]), position),
        RecordedAt::from_millis(1_767_225_600_000),
        event,
    )
}

fn query(owned: bool) -> Query {
    Query::from_item(QueryItem::new([event_type(owned)], tags(owned)).expect("constrained"))
}

fn condition(owned: bool) -> AppendCondition {
    AppendCondition::new(query(owned)).after(SequencePosition::new(7).expect("7 is non-zero"))
}

/// One value in both formats.
fn encodings<T: serde::Serialize>(value: &T) -> (Vec<u8>, Vec<u8>) {
    (
        serde_json::to_vec(value).expect("serialises"),
        postcard::to_stdvec(value).expect("serialises"),
    )
}

/// The control the experiment's subtraction rests on.
///
/// Every figure in `clone-cost.md` is *owned arm minus borrowed arm at the same
/// tag count and format*. If the two arms ever stop emitting the same bytes, the
/// encoder's own allocations stop cancelling and every published delta becomes a
/// number about two different values. The experiment asserts this too and is
/// outside the gate; this is the copy inside it.
#[test]
fn the_two_tag_regimes_encode_to_identical_bytes() {
    for (label, owned, borrowed) in [
        ("Event", encodings(&event(true)), encodings(&event(false))),
        (
            "SequencedEvent",
            encodings(&sequenced(event(true))),
            encodings(&sequenced(event(false))),
        ),
        ("Query", encodings(&query(true)), encodings(&query(false))),
        (
            "AppendCondition",
            encodings(&condition(true)),
            encodings(&condition(false)),
        ),
    ] {
        assert_eq!(
            owned.0, borrowed.0,
            "`{label}` encodes differently in serde_json depending on which `Cow` \
             variant its tags are in. Every allocation figure published about this \
             crate is a subtraction between those two arms, and a subtraction \
             between two different values is not a measurement"
        );
        assert_eq!(
            owned.1, borrowed.1,
            "`{label}` encodes differently in postcard depending on which `Cow` \
             variant its tags are in — same consequence as above"
        );
    }
}

// ---------------------------------------------------------------------------
// The shape
// ---------------------------------------------------------------------------

/// The three files that carry a hand-written `Serialize` impl.
const SOURCES: [(&str, &str); 3] = [
    ("event.rs", include_str!("../src/event.rs")),
    ("query.rs", include_str!("../src/query.rs")),
    ("append.rs", include_str!("../src/append.rs")),
];

/// Every `impl Serialize for …` body in this crate, as `(file, type, body)`.
///
/// A brace counter rather than an indentation match: the bodies contain `}` at
/// several depths, and the closing brace of an `impl` inside `mod serde_impls`
/// sits at four spaces — as do several lines inside it.
fn serialize_bodies() -> Vec<(&'static str, String, String)> {
    let mut found = Vec::new();
    for (file, source) in SOURCES {
        let mut rest = source;
        while let Some(at) = rest.find("impl Serialize for ") {
            let after = &rest[at + "impl Serialize for ".len()..];
            let name: String = after
                .chars()
                .take_while(|character| character.is_alphanumeric() || *character == '_')
                .collect();
            let open =
                at + after.find('{').expect("an impl has a body") + "impl Serialize for ".len();
            let mut depth = 0_i32;
            let mut end = open;
            for (offset, character) in rest[open..].char_indices() {
                match character {
                    '{' => depth += 1,
                    '}' => {
                        depth -= 1;
                        if depth == 0 {
                            end = open + offset;
                            break;
                        }
                    }
                    _ => {}
                }
            }
            found.push((file, name, rest[open..=end].to_owned()));
            rest = &rest[end..];
        }
    }
    assert!(
        found.len() >= 8,
        "the extractor found only {} `Serialize` impls, which is fewer than this \
         crate has had since phase 4. It has stopped matching rather than the \
         impls having gone",
        found.len()
    );
    found
}

/// A `Serialize` impl copies nothing.
///
/// The wrong implementation this rejects is the one that shipped through
/// `0.2.0-alpha.1` at five sites:
///
/// ```text
/// EventWire {
///     event_type: self.event_type().clone(),
///     data: self.data().clone(),
///     tags: self.tags().clone(),
///     metadata: self.metadata().cloned(),
/// }
/// .serialize(serializer)
/// ```
///
/// An owned mirror built from a borrowed value, so that a derive can write it
/// out — and then, for `SequencedEvent`, the whole `Event` cloned into a second
/// mirror whose derive calls the first impl and clones all four fields again.
/// Twice, exactly, at every tag count in both formats.
///
/// The fix is serde's own idiom and moves no byte: one owned mirror for
/// `Deserialize`, where the bytes must become a value, and one **borrowing**
/// mirror for `Serialize`, with the same `rename`, the same field names in the
/// same order and the same `with` routing.
///
/// Why the source and not the behaviour: the mirrors are private, the output is
/// unchanged, and the crate forbids the `unsafe` a counting allocator needs. No
/// behavioural test in any format can tell the two apart. This is the one
/// instrument left, and the alternative — trusting review — is what let five
/// sites through.
#[test]
fn no_serialize_impl_copies_what_it_was_handed() {
    /// Each is a way of turning a borrow into an owned value.
    ///
    /// `.into()` is here because `self.types().into()` was one of the five: a
    /// `&[EventType]` becoming a `Box<[EventType]>` reads as a conversion and is
    /// a deep copy. If a `Serialize` impl ever needs one of these for a reason,
    /// the reason belongs in a comment beside a narrower assertion, not in a
    /// widening of this list.
    const COPIES: [&str; 5] = [
        ".clone()",
        ".cloned()",
        ".to_vec()",
        ".to_owned()",
        ".into()",
    ];

    let mut failures = Vec::new();
    for (file, name, body) in serialize_bodies() {
        for copy in COPIES {
            if body.contains(copy) {
                failures.push(format!(
                    "`impl Serialize for {name}` ({file}) calls `{copy}`. A \
                     `Serialize` impl is handed the value by reference and writes \
                     it; anything it copies first is a transient allocation that \
                     produces no output. Encoding one 64-tag `SequencedEvent` this \
                     way cost 130 of its 140 heap operations on copies — see \
                     `experiments/event-clone-allocations/results/clone-cost.md`. \
                     Use a borrowing mirror: same `rename`, same fields, same \
                     order, same `with`"
                ));
            }
        }
    }
    assert!(failures.is_empty(), "\n{}", failures.join("\n\n"));
}

// ---------------------------------------------------------------------------
// The golden vectors
// ---------------------------------------------------------------------------

/// The exact bytes these four values encoded to **before** the borrowing mirrors
/// landed, in both formats.
///
/// `wire.rs` already pins a bare `Event` in postcard and says why a round trip
/// cannot stand in: a round trip is symmetric, so it passes happily on an
/// encoding that no deployed peer can read. These four extend that reasoning to
/// the values whose `Serialize` impls this file rewrote — a rewrite whose whole
/// claim is that it moved no byte, made against private mirrors that no public
/// signature describes.
///
/// Captured at `701191b`, the commit before the rewrite, by dumping the same
/// values through the same helpers, and pinned here unchanged. The full 64-tag
/// values were compared the same way and are byte-identical across all four
/// types in both formats — 4,124 bytes — but they are not pinned, because a
/// 587-byte hex literal is a thing nobody reads and therefore nobody checks.
/// One tag, one type, one payload byte and one metadata byte is enough to carry
/// every field, every `Option` arm and every length prefix.
///
/// If one of these fails, the question is not "which literal do I update". It is
/// which peer wrote the bytes on the other side.
#[test]
fn the_encoding_is_the_one_that_shipped() {
    let one_tag: Tags = [Tag::new("k00:v00").expect("valid")].into_iter().collect();
    let event = Event::new(
        EventType::new("A").expect("valid"),
        Bytes::from_static(b"\x11\x22"),
    )
    .expect("valid")
    .with_tags(one_tag.clone())
    .with_metadata(Bytes::from_static(b"\x33"));
    let query = Query::from_item(
        QueryItem::new([EventType::new("A").expect("valid")], one_tag).expect("constrained"),
    );

    for (label, value, json, postcard) in [
        (
            "Event",
            encodings(&event),
            r#"{"event_type":"A","data":"ESI=","tags":["k00:v00"],"metadata":"Mw=="}"#,
            // event_type "A"; data 2 bytes; tags: one 7-byte tag; metadata: Some, 1 byte.
            &[
                0x01, 0x41, 0x02, 0x11, 0x22, 0x01, 0x07, 0x6b, 0x30, 0x30, 0x3a, 0x76, 0x30, 0x30,
                0x01, 0x01, 0x33,
            ][..],
        ),
        (
            "SequencedEvent",
            encodings(&sequenced(event.clone())),
            r#"{"position":42,"id":{"store":"07070707070707070707070707070707","position":42},"recorded_at":1767225600000,"event":{"event_type":"A","data":"ESI=","tags":["k00:v00"],"metadata":"Mw=="}}"#,
            // position; id (16 store bytes then the position); recorded_at as a
            // varint; then the Event above, byte for byte.
            &[
                0x2a, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07,
                0x07, 0x07, 0x07, 0x2a, 0x80, 0xa0, 0xd5, 0xed, 0xee, 0x66, 0x01, 0x41, 0x02, 0x11,
                0x22, 0x01, 0x07, 0x6b, 0x30, 0x30, 0x3a, 0x76, 0x30, 0x30, 0x01, 0x01, 0x33,
            ][..],
        ),
        (
            "Query",
            encodings(&query),
            r#"{"Items":[{"types":["A"],"tags":["k00:v00"]}]}"#,
            // The external tag `01` for `Items` — WF-1's whole point, and the
            // byte `All` would not have.
            &[
                0x01, 0x01, 0x01, 0x01, 0x41, 0x01, 0x07, 0x6b, 0x30, 0x30, 0x3a, 0x76, 0x30, 0x30,
            ][..],
        ),
        (
            "AppendCondition",
            encodings(
                &AppendCondition::new(query.clone())
                    .after(SequencePosition::new(7).expect("7 is non-zero")),
            ),
            r#"{"guards":[{"query":{"Items":[{"types":["A"],"tags":["k00:v00"]}]},"after":7}]}"#,
            // One guard: the Query above, then `after` as `Some(7)`.
            &[
                0x01, 0x01, 0x01, 0x01, 0x01, 0x41, 0x01, 0x07, 0x6b, 0x30, 0x30, 0x3a, 0x76, 0x30,
                0x30, 0x01, 0x07,
            ][..],
        ),
    ] {
        assert_eq!(
            String::from_utf8(value.0).expect("serde_json emits UTF-8"),
            json,
            "`{label}`'s serde_json encoding changed"
        );
        assert_eq!(
            value.1, postcard,
            "`{label}`'s postcard encoding changed. postcard writes fields in \
             declaration order with no names, so a mirror whose fields were \
             reordered still round-trips against itself and decodes nothing a \
             deployed peer wrote"
        );
    }
}
