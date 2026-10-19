//! The ingest port's pure facts: what needs no JavaScript heap, so a plain
//! `cargo test -p happenstance-cloudflare` runs it, and the `wasm32` runner runs
//! it again on the target the store lives on.

use core::marker::PhantomData;

use happenstance_core::{
    Event, EventId, EventStore, EventType, RecordedAt, SequencePosition, StoreId, StoreLimit, Tag,
    Tags,
};
use happenstance_sync::{Classify, FailureClass, IngestStore, ReplicatedEvent};

use super::{CloudflareIngestStore, StoredOrigin};
use crate::event_store::{Ceilings, CloudflareEventStore, CloudflareEventStoreError};
use crate::js::JsHandle;
use crate::not_send_probe::{NotSend as _, Probe};
use crate::sql_storage::SqlError;

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

/// 2^53 − 1, `Number.MAX_SAFE_INTEGER`, written out rather than read from the
/// constant under test.
const SAFE: u64 = 9_007_199_254_740_991;

/// An event from a peer, claiming `position` at `recorded_at`.
fn from_peer(position: u64, recorded_at: i64) -> ReplicatedEvent {
    let position = SequencePosition::new(position).expect("a nonzero position");
    ReplicatedEvent::new(
        EventId::new(StoreId::from_bytes([0x5a; 16]), position),
        RecordedAt::from_millis(recorded_at),
        sized(1, 0, None),
    )
}

/// An origin this runtime cannot bind exactly is refused by value, not
/// narrowed. `worker` binds an integer as `i as f64`, so past 2^53 − 1 two
/// origin positions, or two recorded times, would be stored as one.
///
/// Rejects `position_as_i64`'s saturation (which would store `u64::MAX` as
/// `i64::MAX`, and then as a rounded JS number), a bound written as `<` where
/// `<=` is meant, and a recorded time checked on one side only.
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
fn an_origin_past_the_safe_integer_is_refused_not_narrowed() {
    let safe = StoredOrigin::try_from(&from_peer(SAFE, 0)).expect("2^53 - 1 binds exactly");
    assert_eq!(
        (safe.store, safe.position),
        (StoreId::from_bytes([0x5a; 16]), 9_007_199_254_740_991_i64)
    );

    for past in [SAFE + 1, u64::MAX] {
        let refused = StoredOrigin::try_from(&from_peer(past, 0));
        assert!(
            matches!(
                refused,
                Err(CloudflareEventStoreError::OriginPositionOutOfRange { position })
                    if position.get() == past
            ),
            "origin position {past} must be refused by value: {refused:?}"
        );
    }

    let edge = 9_007_199_254_740_991_i64;
    for millis in [edge, -edge] {
        assert!(
            StoredOrigin::try_from(&from_peer(1, millis)).is_ok(),
            "{millis} ms binds exactly"
        );
    }
    for millis in [edge + 1, -edge - 1, i64::MIN] {
        let refused = StoredOrigin::try_from(&from_peer(1, millis));
        assert!(
            matches!(
                refused,
                Err(CloudflareEventStoreError::RecordedAtOutOfRange { recorded_at })
                    if recorded_at.as_millis() == millis
            ),
            "a recorded time of {millis} ms must be refused by value: {refused:?}"
        );
    }
}

/// Every way this adapter fails states a class (SY-36), row by row from the
/// plan's table, `PartialBatch`'s delegation to its cause included.
///
/// Rejects a wildcard answering `Stop` for a re-entrant borrow, which the next
/// attempt finds free, and a `PartialBatch` classified apart from the failure
/// that stopped the append.
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
fn cloudflare_errors_state_their_class() {
    let position = SequencePosition::new(3).expect("nonzero");
    let partial = |cause: CloudflareEventStoreError| CloudflareEventStoreError::PartialBatch {
        from: position,
        cause: Box::new(cause),
        while_discarding: Box::new(CloudflareEventStoreError::CorruptTags),
    };
    let rows = [
        (SqlError::AlreadyBorrowed.into(), FailureClass::Transient),
        (SqlError::CursorInvalidated.into(), FailureClass::Transient),
        (SqlError::StorageLimitExceeded.into(), FailureClass::Stop),
        (
            SqlError::internal("a thrown storage error").into(),
            FailureClass::Stop,
        ),
        (
            CloudflareEventStoreError::OriginPositionOutOfRange { position },
            FailureClass::Park,
        ),
        (
            CloudflareEventStoreError::RecordedAtOutOfRange {
                recorded_at: RecordedAt::from_millis(i64::MAX),
            },
            FailureClass::Park,
        ),
        (
            partial(SqlError::AlreadyBorrowed.into()),
            FailureClass::Transient,
        ),
        (
            partial(CloudflareEventStoreError::CorruptTags),
            FailureClass::Stop,
        ),
        (
            CloudflareEventStoreError::RowShape {
                expected: 1,
                actual: 2,
            },
            FailureClass::Stop,
        ),
        (
            CloudflareEventStoreError::ColumnType {
                column: "event.data",
            },
            FailureClass::Stop,
        ),
        (
            EventType::new("").expect_err("an empty event type").into(),
            FailureClass::Stop,
        ),
        (
            Tag::new("").expect_err("an empty tag").into(),
            FailureClass::Stop,
        ),
        (CloudflareEventStoreError::CorruptTags, FailureClass::Stop),
        (
            CloudflareEventStoreError::UnstampedEvent { position },
            FailureClass::Stop,
        ),
        (
            CloudflareEventStoreError::StoredPosition { raw: 0 },
            FailureClass::Stop,
        ),
    ];
    for (error, class) in rows {
        assert_eq!(error.class(), class, "{error:?}");
    }
}

/// The ingest store binds the **bare** ports, the only ones a `!Send` store
/// has, and is `!Send` itself. Named through its one constructor, which takes
/// the store and the storage object `transactionSync` lives on and nothing
/// else.
///
/// Rejects a `Send`-bound port or impl, which would not compile here, and a
/// wrapper that restored `Send` by holding something thread-safe.
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
fn ingest_store_satisfies_the_bare_bounds() {
    fn bare<S: EventStore + IngestStore>() {}
    // The constructor's signature, pinned: an owned store (the wrapper keeps
    // it) and the storage object, and no identity, which `store_id` reads
    // every time.
    fn built_by<F>(_: F)
    where
        F: Fn(
            CloudflareEventStore,
            JsHandle,
        ) -> Result<CloudflareIngestStore, CloudflareEventStoreError>,
    {
    }

    bare::<CloudflareIngestStore>();
    built_by(CloudflareIngestStore::open);
    assert!(
        !Probe::<CloudflareIngestStore>(PhantomData).is_send(),
        "a Durable Object's ingest store must be !Send"
    );
    assert!(
        Probe::<fn(CloudflareEventStore, JsHandle)>(PhantomData).is_send(),
        "the control: the probe answers Send where it holds"
    );
}

// The tests that need a JavaScript heap, in their own files.
#[cfg(target_arch = "wasm32")]
mod claims;
#[cfg(target_arch = "wasm32")]
mod reads;
#[cfg(target_arch = "wasm32")]
mod support;
#[cfg(target_arch = "wasm32")]
mod writes;
