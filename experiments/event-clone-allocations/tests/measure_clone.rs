//! Arms 1–6: what one `Event` clone costs, and what the encode path costs.
//!
//! Run after `arms_are_equivalent`, never before. Every row prints the settings
//! that produced it; `run.sh` tees the whole thing to `results/raw/clone.txt`
//! and the tables in `results/*.md` are written by hand from that file.
//!
//! The assertions at the bottom are not decoration. They are the shape of the
//! claim — `t + 2` in one regime and `1` in the other — written so that a future
//! change to `Tag`, `Tags`, `EventType` or `Event` that alters the cost turns
//! this file red instead of silently changing a published figure.

use std::mem::{align_of, size_of};

use event_clone_allocations::arms::{self, Payload, Regime};
use event_clone_allocations::counting::{Counts, measure};
use happenstance_core::{
    Event, EventId, EventType, Query, QueryItem, RecordedAt, SequencePosition, SequencedEvent,
    StoreId, Tag, Tags,
};

/// 64 is VT-22's floor (`MIN_SUPPORTED_TAGS_PER_EVENT`); 128 is
/// `SqliteEventStore::MAX_TAGS_PER_EVENT`, the only documented adapter ceiling in
/// the tree. Both are quoted because the encode delta is linear in the count, so
/// one number is half an answer.
const TAG_COUNTS: [usize; 6] = [0, 1, 8, 32, 64, 128];

fn sequenced(event: Event) -> SequencedEvent {
    let position = SequencePosition::new(42).expect("42 is non-zero");
    SequencedEvent::new(
        position,
        EventId::new(StoreId::from_bytes([7; 16]), position),
        RecordedAt::from_millis(1_767_225_600_000),
        event,
    )
}

/// H3's assertion, produced by a compiler rather than derived on paper.
///
/// The finding proposes `size_of` assertions for exactly these six types and
/// notes that the review pass could not run a compiler, so its own figures were
/// reasoned. These are measured, on the toolchain and target named in
/// `README.md`, and they are what a `tests/layout_budget.rs` in
/// `happenstance-core` would have to be written against.
#[test]
fn type_sizes() {
    println!("\n== size_of / align_of, the six types on the read hot path ==");
    println!("target = {}", std::env::consts::ARCH);
    println!("{:<28} {:>8} {:>8}", "type", "size", "align");
    macro_rules! row {
        ($t:ty) => {
            println!(
                "{:<28} {:>8} {:>8}",
                stringify!($t),
                size_of::<$t>(),
                align_of::<$t>()
            );
        };
    }
    row!(Tag);
    row!(EventType);
    row!(Tags);
    row!(Event);
    row!(SequencedEvent);
    row!(QueryItem);
    println!("-- context, and the one assertion the crate already makes --");
    row!(SequencePosition);
    row!(Option<SequencePosition>);
    row!(Query);
    row!(happenstance_core::bytes::Bytes);
    row!(Option<happenstance_core::bytes::Bytes>);

    // The niche claim `event.rs:970-976` already asserts, re-checked here so a
    // reader can see that this file's numbers come off the same compiler.
    assert_eq!(
        size_of::<SequencePosition>(),
        size_of::<Option<SequencePosition>>(),
        "the NonZeroU64 niche stopped paying for itself"
    );
}

#[test]
fn clone_cost() {
    println!("\n== arm 1 & 2: `event.clone()`, one event, by tag regime ==");
    println!(
        "{:<28} {:<22} {:>5}  {}",
        "regime", "payload", "tags", "counts"
    );

    let mut rows = Vec::new();
    for regime in [Regime::Owned, Regime::Static] {
        for shape in [Payload::Static, Payload::Vec] {
            for count in TAG_COUNTS {
                let event = arms::event(regime, count, shape);
                // For `Bytes::from(Vec<u8>)` the *first* clone promotes the
                // buffer to a shared representation and allocates once for the
                // `Shared` header; every clone after that is a refcount bump.
                // Measuring the second clone as well is what separates "the
                // payload is free" from "the payload is free from the second
                // clone onwards", which is not the same promise.
                let (first, first_counts) = measure(|| event.clone());
                let (second, second_counts) = measure(|| event.clone());
                drop(first);
                drop(second);

                println!(
                    "{:<28} {:<22} {:>5}  first: {first_counts}",
                    regime.label(),
                    shape.label(),
                    count
                );
                println!("{:<28} {:<22} {:>5}  again: {second_counts}", "", "", "");
                rows.push((regime, shape, count, first_counts, second_counts));
            }
        }
    }

    println!("\n-- the claim, checked --");
    for (regime, shape, count, first, second) in &rows {
        let (regime, shape, count) = (*regime, *shape, *count);
        // The steady-state clone: the payload has been promoted (or never needed
        // promoting), so this is the number the four documentation sites are
        // making a claim about.
        let steady = second.heap_ops();
        let expected = match (regime, count) {
            // `Box<[Tag]>` of length zero allocates nothing, so an untagged
            // event in the owned regime is one allocation, not two: the
            // `EventType`. The "two allocations" figure was never right for a
            // tagless event either, in the other direction.
            (Regime::Owned, 0) => 1,
            (Regime::Owned, tags) => tags as u64 + 2,
            // One allocation for the boxed slice, none for its contents, and
            // none at all when the slice is empty. This is the regime a
            // benchmark author lands in without choosing it.
            (Regime::Static, 0) => 0,
            (Regime::Static, _) => 1,
        };
        assert_eq!(
            steady,
            expected,
            "steady-state clone: {} @ {count} tags, {}",
            regime.label(),
            shape.label()
        );

        let promotion = first.heap_ops() - second.heap_ops();
        let expected_promotion = u64::from(shape == Payload::Vec);
        assert_eq!(
            promotion,
            expected_promotion,
            "first-clone promotion: {} @ {count} tags, {}",
            regime.label(),
            shape.label()
        );
    }
    println!("all clone-cost assertions hold");
}

/// One encode, measured. Returns the counts and the byte length, because the
/// byte length is half the control: two arms whose counts differ and whose
/// output lengths differ have not been compared.
fn encode<T: serde::Serialize>(value: &T, format: Format) -> (Counts, usize) {
    match format {
        Format::Json => {
            let (bytes, counts) = measure(|| serde_json::to_vec(value).expect("serialises"));
            (counts, bytes.len())
        }
        Format::Postcard => {
            let (bytes, counts) = measure(|| postcard::to_stdvec(value).expect("serialises"));
            (counts, bytes.len())
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Format {
    Json,
    Postcard,
}

impl Format {
    const fn label(self) -> &'static str {
        match self {
            Self::Json => "serde_json",
            Self::Postcard => "postcard",
        }
    }
}

#[test]
fn encode_cost() {
    println!("\n== arm 3 & 4: `Serialize for Event` / `Serialize for SequencedEvent` ==");
    println!(
        "Each row is one encode of one value. `delta` is the owned row minus the\n\
         static row at the same tag count and format: the encoder emits identical\n\
         bytes in both (proved in `arms_are_equivalent`), so the difference is the\n\
         wire mirror's clone and nothing else."
    );
    println!(
        "\n{:<10} {:<14} {:>5} {:<28} {:>7}  {}",
        "value", "format", "tags", "regime", "out_len", "counts"
    );

    for format in [Format::Json, Format::Postcard] {
        for count in TAG_COUNTS {
            let mut event_counts = [Counts::default(); 2];
            let mut sequenced_counts = [Counts::default(); 2];

            for (slot, regime) in [Regime::Owned, Regime::Static].into_iter().enumerate() {
                // `Bytes::from_static` throughout, so no promotion allocation
                // lands in the encode arms and every allocation counted belongs
                // to the encoder or to the mirror's clone.
                let event = arms::event(regime, count, Payload::Static);
                // Warm the value: `Bytes::from_static` needs no promotion, but
                // taking one clone first makes the arms identical in state as
                // well as in value.
                drop(event.clone());

                let (counts, len) = encode(&event, format);
                println!(
                    "{:<10} {:<14} {:>5} {:<28} {:>7}  {counts}",
                    "Event",
                    format.label(),
                    count,
                    regime.label(),
                    len
                );
                event_counts[slot] = counts;

                let sequenced = sequenced(event);
                let (counts, len) = encode(&sequenced, format);
                println!(
                    "{:<10} {:<14} {:>5} {:<28} {:>7}  {counts}",
                    "Sequenced",
                    format.label(),
                    count,
                    regime.label(),
                    len
                );
                sequenced_counts[slot] = counts;
            }

            let event_delta = event_counts[0].heap_ops() as i64 - event_counts[1].heap_ops() as i64;
            let sequenced_delta =
                sequenced_counts[0].heap_ops() as i64 - sequenced_counts[1].heap_ops() as i64;
            println!(
                "{:<10} {:<14} {:>5} {:<28} {:>7}  delta(Event)={event_delta}  \
                 delta(Sequenced)={sequenced_delta}  ratio={}",
                "--",
                format.label(),
                count,
                "owned - static",
                "",
                if event_delta == 0 {
                    "n/a".to_owned()
                } else {
                    format!("{:.2}", sequenced_delta as f64 / event_delta as f64)
                }
            );

            // The mirror clones `event_type` (one allocation in the owned
            // regime, none in the static one) and every tag (one each, none in
            // the static one). The boxed slice itself is cloned in both, so it
            // cancels.
            assert_eq!(
                event_delta,
                if count == 0 { 1 } else { count as i64 + 1 },
                "Event encode delta at {count} tags in {}",
                format.label()
            );
            // Twice, because `Serialize for SequencedEvent` clones the whole
            // `Event` into `SequencedEventWire` and the derived impl for that
            // mirror then calls `Serialize for Event`, which builds `EventWire`
            // and clones all four fields again.
            assert_eq!(
                sequenced_delta,
                2 * event_delta,
                "SequencedEvent encode delta at {count} tags in {}",
                format.label()
            );
        }
    }
    println!("all encode-cost assertions hold");
}

/// Not one of the six arms, and reported because AE-2 names it in the same
/// breath: `Serialize for Query` calls `items.to_vec()`, which deep-clones every
/// `QueryItem`, each of which owns a `Tags` and a `Box<[EventType]>`.
#[test]
fn query_encode_cost() {
    println!("\n== adjacent: `Serialize for Query`, one item, by tag regime ==");
    for count in [0_usize, 8, 64] {
        for regime in [Regime::Owned, Regime::Static] {
            let tags = if count == 0 {
                Tags::empty()
            } else {
                arms::tags(regime, count)
            };
            let types: Vec<EventType> = if count == 0 {
                vec![match regime {
                    Regime::Owned => EventType::new(arms::EVENT_TYPE).expect("valid"),
                    Regime::Static => EventType::from_static(arms::EVENT_TYPE),
                }]
            } else {
                Vec::new()
            };
            let query = Query::from_item(QueryItem::new(types, tags).expect("constrained"));
            let (counts, len) = encode(&query, Format::Postcard);
            println!(
                "{:<28} {:>5} tags  out_len={len:<5}  {counts}",
                regime.label(),
                count
            );
        }
    }
}

/// A control on the instrument itself: cloning a `Tag` in each regime, which is
/// the smallest unit the whole `t + 2` claim is built out of.
#[test]
fn tag_clone_is_the_unit() {
    let owned = Tag::new("k00:v00").expect("valid");
    let borrowed = Tag::from_static("k00:v00");
    let (kept, owned_counts) = measure(|| owned.clone());
    drop(kept);
    let (kept, borrowed_counts) = measure(|| borrowed.clone());
    drop(kept);
    println!("\n== control: one `Tag::clone` ==");
    println!("Tag::new        {owned_counts}");
    println!("Tag::from_static {borrowed_counts}");
    assert_eq!(owned_counts.heap_ops(), 1, "an owned tag clone allocates");
    assert_eq!(
        borrowed_counts.heap_ops(),
        0,
        "a borrowed tag clone does not"
    );
}
