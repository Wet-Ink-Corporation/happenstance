//! The ingest port's pure facts: what needs no JavaScript heap, so a plain
//! `cargo test -p happenstance-cloudflare` runs it, and the `wasm32` runner runs
//! it again on the target the store lives on.

use happenstance_core::{Event, StoreLimit, Tag, Tags};

use crate::event_store::Ceilings;

/// An event of `data_len` bytes, `tags` tags and, if given, `metadata_len`
/// bytes of metadata.
fn sized(data_len: usize, tags: usize, metadata_len: Option<usize>) -> Event {
    let tags: Tags = (0..tags)
        .map(|n| Tag::new(format!("t{n}")).expect("a valid tag"))
        .collect();
    let event = Event::new("Sized", vec![0_u8; data_len])
        .expect("a valid event type")
        .with_tags(tags);
    match metadata_len {
        Some(len) => event.with_metadata(vec![0_u8; len]),
        None => event,
    }
}

/// Small ceilings, so a test crosses each with a handful of bytes.
const SMALL: Ceilings = Ceilings {
    event_data_len: 4,
    tags_per_event: 2,
    events_per_batch: 3,
    metadata_len: 5,
};

/// `Ceilings::exceeded` names the first ceiling crossed, in the order
/// `check_ceilings` has always refused in: the batch length, then each event's
/// data, tags and metadata. Exactly at a ceiling is not over it.
///
/// Rejects an off-by-one at any ceiling, a reordered check (a batch over two
/// ceilings at once names the earlier one), and a check of the first event
/// only. It pins the refactor that moved `check_ceilings`' body here.
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
fn ceilings_name_the_first_exceeded_limit_in_order() {
    let at = [
        sized(4, 2, Some(5)),
        sized(4, 2, None),
        sized(0, 0, Some(0)),
    ];
    assert_eq!(
        SMALL.exceeded(at.len(), &at),
        None,
        "exactly at every ceiling"
    );

    let four = [
        sized(5, 3, Some(6)),
        sized(0, 0, None),
        sized(0, 0, None),
        sized(0, 0, None),
    ];
    assert_eq!(
        SMALL.exceeded(four.len(), &four),
        Some((StoreLimit::EventsPerBatch, 4)),
        "the batch length comes first"
    );
    assert_eq!(
        SMALL.exceeded(1, &[sized(0, 0, None), sized(5, 3, Some(6))]),
        Some((StoreLimit::EventDataLen, 5)),
        "then data, and on an event behind the head"
    );
    assert_eq!(
        SMALL.exceeded(1, &[sized(4, 3, Some(6))]),
        Some((StoreLimit::TagsPerEvent, 3)),
        "then tags"
    );
    assert_eq!(
        SMALL.exceeded(1, &[sized(4, 2, Some(6))]),
        Some((StoreLimit::MetadataLen, 6)),
        "then metadata"
    );
    assert_eq!(
        SMALL.exceeded(
            3,
            &[sized(4, 2, None), sized(4, 2, Some(5)), sized(0, 3, None)]
        ),
        Some((StoreLimit::TagsPerEvent, 3)),
        "the third event is checked as closely as the first"
    );
}
