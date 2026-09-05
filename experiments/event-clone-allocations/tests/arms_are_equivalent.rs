//! Conformance first: an arm that is fast and wrong wins every benchmark.
//!
//! Nothing in `results/` is derived until this file passes. It establishes four
//! things, and the measurement is meaningless without all four:
//!
//! 1. **The two tag regimes build the same event.** `Tags::from_pairs` and
//!    `Tag::from_static` differ in the `Cow` variant and in nothing else — same
//!    type string, same sixty-four tag strings, same payload, `==`.
//! 2. **They encode to the same bytes.** In `serde_json` *and* in `postcard`,
//!    for `Event` and for `SequencedEvent`. This is the control that stops the
//!    cheap arm from winning by encoding less; without it, "the static arm made
//!    fewer allocations" would be consistent with "the static arm dropped the
//!    tags".
//! 3. **Both regimes round-trip**, in a self-describing format and in a
//!    non-self-describing one, the postcard half **framed** — the same
//!    two-format matrix and the same framing trick
//!    `crates/happenstance-core/tests/wire.rs` uses, for the reason stated in
//!    its header: a field-count desynchronisation surfaces in postcard as a
//!    convenient parse error unless there are bytes after the value.
//! 4. **The read-path replicas are faithful.** `readpath::read_before` is a
//!    transcription of `memory.rs:302-333`; it is compared against the **real**
//!    `MemoryEventStore::read` over the same values across a grid of 160
//!    option/query combinations, and `read_after` is compared against it over
//!    the same grid. A replica that had drifted fails here rather than producing
//!    a number about code that does not exist.

use event_clone_allocations::arms::{self, Payload, Regime};
use event_clone_allocations::readpath::{self, block_on_ready, read_real};
use happenstance_core::bytes::Bytes;
use happenstance_core::{
    Event, EventId, EventStore, EventType, MemoryEventStore, Query, QueryItem, ReadOptions,
    RecordedAt, SequencePosition, SequencedEvent, StoreId, Tags,
};

const REGIMES: [Regime; 2] = [Regime::Owned, Regime::Static];
const TAG_COUNTS: [usize; 5] = [0, 1, 8, 64, 128];

/// A fixed identity, so that the byte-identity comparison is not defeated by a
/// randomly minted `StoreId` or a wall-clock `RecordedAt`.
fn sequenced(event: Event) -> SequencedEvent {
    let position = SequencePosition::new(42).expect("42 is non-zero");
    SequencedEvent::new(
        position,
        EventId::new(StoreId::from_bytes([7; 16]), position),
        RecordedAt::from_millis(1_767_225_600_000),
        event,
    )
}

/// The two counts this experiment quotes are the two the tree actually promises.
///
/// The literal table used to be exactly the floor, and the assertion below was
/// `len() == MIN_SUPPORTED_TAGS_PER_EVENT`. It was extended to 128 on 2026-09-04
/// because the encode-path delta is linear in the tag count, so a figure quoted
/// only at the floor is half an answer — and 128 is not an arbitrary second
/// point, it is `SqliteEventStore::MAX_TAGS_PER_EVENT`, the only documented
/// adapter ceiling in the workspace.
///
/// Both ends are still asserted, so the table cannot drift away from either: it
/// must *cover* the floor and *end at* the ceiling.
#[test]
fn the_floor_and_the_documented_ceiling_are_both_covered() {
    assert_eq!(
        happenstance_core::MIN_SUPPORTED_TAGS_PER_EVENT,
        64,
        "VT-22's floor moved; the 64-tag arms are no longer measuring the floor"
    );
    assert!(
        arms::TAG_LITERALS.len() >= happenstance_core::MIN_SUPPORTED_TAGS_PER_EVENT,
        "the literal table must cover the floor"
    );
    assert_eq!(
        arms::TAG_LITERALS.len(),
        128,
        "the table ends at `SqliteEventStore::MAX_TAGS_PER_EVENT`. If that          constant moves, move this one with it — a ceiling row measured against          a number no adapter declares is a row about nothing"
    );
}

#[test]
fn the_two_tag_tables_agree() {
    // The pairs are derived from the literals by splitting, and the literals are
    // rebuilt from the pairs by joining. Asserting the round trip over the whole
    // table is what makes "the same tag strings" a checked claim rather than a
    // comment — and it covers the 64 literals added for the ceiling row, which a
    // hard-coded 64 would have skipped.
    for (literal, (key, value)) in arms::TAG_LITERALS
        .iter()
        .zip(arms::tag_pairs(arms::TAG_LITERALS.len()))
    {
        assert_eq!(*literal, format!("{key}:{value}"));
    }
}

#[test]
fn the_regimes_build_equal_events() {
    for count in TAG_COUNTS {
        for shape in [Payload::Static, Payload::Vec] {
            let owned = arms::event(Regime::Owned, count, shape);
            let borrowed = arms::event(Regime::Static, count, shape);
            assert_eq!(
                owned,
                borrowed,
                "the regimes diverged at {count} tags, {}",
                shape.label()
            );
            assert_eq!(owned.tags().len(), count);
            assert_eq!(owned.data(), borrowed.data());
        }
    }
}

#[test]
fn the_regimes_encode_to_identical_bytes() {
    for count in TAG_COUNTS {
        for shape in [Payload::Static, Payload::Vec] {
            let owned = arms::event(Regime::Owned, count, shape);
            let borrowed = arms::event(Regime::Static, count, shape);

            assert_eq!(
                serde_json::to_vec(&owned).expect("Event serialises"),
                serde_json::to_vec(&borrowed).expect("Event serialises"),
                "JSON diverged at {count} tags"
            );
            assert_eq!(
                postcard::to_stdvec(&owned).expect("Event serialises"),
                postcard::to_stdvec(&borrowed).expect("Event serialises"),
                "postcard diverged at {count} tags"
            );

            let owned = sequenced(owned);
            let borrowed = sequenced(borrowed);
            assert_eq!(
                serde_json::to_vec(&owned).expect("SequencedEvent serialises"),
                serde_json::to_vec(&borrowed).expect("SequencedEvent serialises"),
                "sequenced JSON diverged at {count} tags"
            );
            assert_eq!(
                postcard::to_stdvec(&owned).expect("SequencedEvent serialises"),
                postcard::to_stdvec(&borrowed).expect("SequencedEvent serialises"),
                "sequenced postcard diverged at {count} tags"
            );
        }
    }
}

/// `wire.rs`'s framing trick: encode the value with a sentinel after it, decode
/// with `take_from_bytes`, and require the sentinel back untouched. A decoder
/// that read one field too few would swallow part of the sentinel and be caught
/// here, where a bare `from_bytes` would report a tidy `DeserializeUnexpectedEnd`
/// and hide it.
fn postcard_round_trip_framed<T>(value: &T, label: &str)
where
    T: serde::Serialize + serde::de::DeserializeOwned + PartialEq + core::fmt::Debug,
{
    const SENTINEL: [u8; 4] = [0xDE, 0xAD, 0xBE, 0xEF];
    let mut framed = postcard::to_stdvec(value).expect("value serialises");
    framed.extend_from_slice(&SENTINEL);

    let (decoded, rest) =
        postcard::take_from_bytes::<T>(&framed).expect("the framed value decodes");
    assert_eq!(&decoded, value, "{label}: postcard round trip lost data");
    assert_eq!(rest, SENTINEL, "{label}: the decoder desynchronised");
}

#[test]
fn both_regimes_round_trip_in_both_formats() {
    for regime in REGIMES {
        for count in TAG_COUNTS {
            for shape in [Payload::Static, Payload::Vec] {
                let label = format!("{} @ {count} tags, {}", regime.label(), shape.label());
                let event = arms::event(regime, count, shape);

                let json = serde_json::to_vec(&event).expect("Event serialises");
                let back: Event = serde_json::from_slice(&json).expect("Event deserialises");
                assert_eq!(back, event, "{label}: JSON round trip lost data");
                postcard_round_trip_framed(&event, &label);

                let sequenced = sequenced(event);
                let json = serde_json::to_vec(&sequenced).expect("SequencedEvent serialises");
                let back: SequencedEvent =
                    serde_json::from_slice(&json).expect("SequencedEvent deserialises");
                assert_eq!(
                    back, sequenced,
                    "{label}: sequenced JSON round trip lost data"
                );
                postcard_round_trip_framed(&sequenced, &label);
            }
        }
    }
}

/// A store deliberately *not* uniform: two event types, three tag widths, so the
/// query filter and both bounds filters have something to reject. A grid run
/// against a store every query matches would exercise neither branch.
fn mixed_store() -> (MemoryEventStore, Vec<SequencedEvent>) {
    let store = MemoryEventStore::new();
    let mut batch = Vec::new();
    for index in 0..200_usize {
        let event_type = if index % 2 == 0 {
            EventType::new("StudentSubscribed").expect("valid")
        } else {
            EventType::new("CourseDefined").expect("valid")
        };
        let width = [0, 3, 64][index % 3];
        let event = Event::new(event_type, Bytes::from_static(arms::PAYLOAD))
            .expect("valid")
            .with_tags(arms::tags(Regime::Owned, width));
        batch.push(event);
        if batch.len() == readpath::FILL_BATCH || index == 199 {
            block_on_ready(store.append(&batch, None)).expect("append succeeds");
            batch.clear();
        }
    }
    let mirror = read_real(&store, &Query::all(), ReadOptions::new());
    assert_eq!(mirror.len(), 200);
    (store, mirror)
}

#[test]
fn the_replicas_match_the_real_store() {
    let (store, mirror) = mixed_store();

    let queries = [
        ("all", Query::all()),
        (
            "by type",
            Query::from_item(
                QueryItem::new(["StudentSubscribed"], Tags::empty()).expect("constrained"),
            ),
        ),
        (
            "by three tags",
            Query::from_item(
                QueryItem::new(
                    core::iter::empty::<EventType>(),
                    arms::tags(Regime::Owned, 3),
                )
                .expect("constrained"),
            ),
        ),
        (
            "by sixty-four tags",
            Query::from_item(
                QueryItem::new(
                    core::iter::empty::<EventType>(),
                    arms::tags(Regime::Owned, 64),
                )
                .expect("constrained"),
            ),
        ),
    ];

    let bounds: [(Option<u64>, Option<u64>); 4] = [
        (None, None),
        (Some(50), None),
        (None, Some(150)),
        (Some(40), Some(160)),
    ];
    let limits = [None, Some(0), Some(1), Some(7), Some(10_000)];

    let mut combinations = 0_usize;
    for (query_label, query) in &queries {
        for backwards in [false, true] {
            for (from, to) in bounds {
                for limit in limits {
                    let mut options = ReadOptions::new();
                    if backwards {
                        options = options.backwards();
                    }
                    // Backwards swaps which side of the position order each
                    // bound sits on, so the raw pair is applied as-is and the
                    // *store* decides what it means — exactly as a caller would.
                    if let Some(from) = from {
                        options = options.from(SequencePosition::new(from).expect("non-zero"));
                    }
                    if let Some(to) = to {
                        options = options.to(SequencePosition::new(to).expect("non-zero"));
                    }
                    if let Some(limit) = limit {
                        options = options.limit(limit);
                    }

                    let label = format!(
                        "{query_label} backwards={backwards} from={from:?} to={to:?} limit={limit:?}"
                    );
                    let real = read_real(&store, query, options);
                    let before = readpath::read_before(&mirror, query, options);
                    let after = readpath::read_after(&mirror, query, options);

                    assert_eq!(
                        before, real,
                        "{label}: the replica has drifted from memory.rs"
                    );
                    assert_eq!(after, before, "{label}: the fix changes the answer");
                    combinations += 1;
                }
            }
        }
    }

    assert_eq!(combinations, 4 * 2 * 4 * 5, "the grid shrank");
}
