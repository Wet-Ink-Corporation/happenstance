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
//! **Two: the pin.** Four golden vectors, so that a change to the mirrors that
//! moves a byte arrives as a diff somebody has to look at. `wire.rs` already
//! pins a bare `Event` in postcard for that reason; these extend it to the four
//! values whose impls the fix rewrites.
//!
//! # What is not here, and where it went
//!
//! The fix itself. A borrowing mirror at each of the five sites — same `rename`,
//! same fields in the same order, same payload routing — was written and
//! measured on this branch at `2f11eb7`: **140 heap operations → 8** for a
//! 64-tag `SequencedEvent` in postcard, zero delta at every tag count in both
//! formats, and byte-identical output across all four values in both formats,
//! checked against the bytes the previous commit produced.
//!
//! It is reverted, and not because anything was wrong with it. It adds 66 lines
//! to `event.rs` and 30 to `query.rs` above `mod tests`, which renumbers three
//! test functions that `spec/SPECIFICATION.md` cites by line, past
//! `spec-trace`'s twelve-line tolerance. Repointing those four citations means
//! editing the specification, which the lane that measured this was not
//! permitted to do. The proposal, the diff and the four repoints are in
//! `.kb/_intake/remediation-2026-09-04-briefs/serialize-borrows-what-it-writes.md`.
//!
//! The guard that belongs beside the fix — a check that no `Serialize` impl in
//! this crate calls `.clone()`, `.cloned()`, `.to_vec()`, `.to_owned()` or
//! `.into()` — is in that brief rather than here, because it is red without the
//! fix. It reads the source, which is unusual and is the point: the mirrors are
//! private, the bytes do not move, and the workspace forbids the `unsafe` a
//! counting allocator needs, so no behavioural test in any format can tell an
//! owned mirror from a borrowing one. Review was the only other instrument and
//! it passed this at five sites.

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
