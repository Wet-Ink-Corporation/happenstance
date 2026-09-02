//! Per-codec round trips, driven from **outside** the crate.
//!
//! An integration test compiles as a downstream crate, so a codec that is not
//! re-exported at the crate root is not reachable here at all — which is the
//! difference between a codec that exists and a codec that is mounted.

use happenstance::bytes::Bytes;
use happenstance::{Codec, Event, EventStore, Json, MemoryEventStore, Query, ReadOptions, collect};
use serde::{Deserialize, Serialize};

/// One payload shape, deliberately not the doctest's: a change to one must not
/// quietly repair the other.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct Enrolled {
    student: String,
    seat: u32,
}

fn sample() -> Enrolled {
    Enrolled {
        student: "s1".to_owned(),
        seat: 7,
    }
}

// ---------------------------------------------------------------------------
// AC-001 — JSON under the default feature set, no feature flag typed
// ---------------------------------------------------------------------------

#[test]
fn json_round_trips_under_default_features() {
    // `Json` is a unit struct, so the call site names a codec with no
    // constructor, no configuration and no import beyond the crate root.
    let codec = Json;

    let payload = codec.encode(&sample()).expect("the sample serialises");
    let back: Enrolled = codec.decode(&payload).expect("its own bytes deserialise");

    assert_eq!(back, sample());
    assert_eq!(<Json as Codec>::TAG, "json");
    // The bytes really are JSON, not some other encoding wearing the tag.
    assert!(
        payload.starts_with(b"{"),
        "the default encoding is JSON: {payload:?}"
    );
}

// ---------------------------------------------------------------------------
// AC-002 — a feature adds a codec and takes nothing away
// ---------------------------------------------------------------------------

#[cfg(feature = "postcard")]
#[test]
fn postcard_round_trips() {
    use happenstance::Postcard;

    let payload = Postcard.encode(&sample()).expect("the sample serialises");
    let back: Enrolled = Postcard
        .decode(&payload)
        .expect("its own bytes deserialise");

    assert_eq!(back, sample());
    assert_eq!(<Postcard as Codec>::TAG, "postcard");

    // The feature *added*: JSON still round-trips in the same build, and the
    // two encodings are genuinely different bytes for the same value.
    let json = Json.encode(&sample()).expect("the sample serialises");
    assert_ne!(json, payload, "postcard and JSON produced the same bytes");
    assert!(
        payload.len() < json.len(),
        "the compact format is not compact: {} vs {}",
        payload.len(),
        json.len()
    );
}

#[cfg(feature = "cbor")]
#[test]
fn cbor_round_trips() {
    use happenstance::Cbor;

    let payload = Cbor.encode(&sample()).expect("the sample serialises");
    let back: Enrolled = Cbor.decode(&payload).expect("its own bytes deserialise");

    assert_eq!(back, sample());
    assert_eq!(<Cbor as Codec>::TAG, "cbor");

    let json = Json.encode(&sample()).expect("the sample serialises");
    assert_ne!(json, payload, "CBOR and JSON produced the same bytes");
}

// ---------------------------------------------------------------------------
// AC-004 — what crosses the port is opaque bytes, and encoding writes nothing
// ---------------------------------------------------------------------------

#[tokio::test]
async fn payload_crosses_the_port_as_bytes() {
    let store = MemoryEventStore::new();

    // Encoding is pure: build a payload, and the store has not been touched.
    let payload: Bytes = Json.encode(&sample()).expect("the sample serialises");
    assert!(
        store.is_empty(),
        "encoding wrote to the store; the append is the single irreversible act"
    );

    let event = Event::new("Enrolled", payload.clone()).expect("a valid event type");
    store
        .append(&[event], None)
        .await
        .expect("an unconditional append");

    let read = collect(store.read(&Query::all(), ReadOptions::new()))
        .await
        .expect("the store reads back");
    let first = read.first().expect("one event was appended");

    // Byte-for-byte the codec's own output. Nothing re-encoded it on the way
    // through, and no store had to know what shape it was.
    assert_eq!(first.event.data(), &payload);
}
