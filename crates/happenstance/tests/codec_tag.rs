//! The codec tag: two encodings in one store, and a tag this build cannot read.
//!
//! ADR-0021 sited the tag in `Event::metadata`, inside a versioned framing
//! region the typed layer owns and no store parses. The framing bytes are
//! spelled out here rather than reached through the crate's own writer: the
//! format is a wire contract, and a test that built it from the writer could
//! not notice the writer changing.

use happenstance::bytes::Bytes;
use happenstance::{
    Boundary, Codec, CodecError, DecisionModel, DomainEvent, Event, EventType, Json,
    MemoryEventStore, Tags, read_decision_model,
};
use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// The framing region, written by hand
// ---------------------------------------------------------------------------

/// `b"hpst"` + framing version + tag + `0xFF`, then whatever metadata the
/// application put there, carried through untouched. `0xFF` cannot appear in
/// UTF-8, so no tag can collide with the byte that ends it.
fn framed(tag: &str, application: &[u8]) -> Bytes {
    let mut out = Vec::from(&b"hpst\x01"[..]);
    out.extend_from_slice(tag.as_bytes());
    out.push(0xFF);
    out.extend_from_slice(application);
    Bytes::from(out)
}

// ---------------------------------------------------------------------------
// One two-variant domain
// ---------------------------------------------------------------------------

const DEFINED: EventType = EventType::from_static("CourseDefined");
const SUBSCRIBED: EventType = EventType::from_static("StudentSubscribed");

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
enum Enrolment {
    Defined { capacity: u32 },
    Subscribed { student: String },
}

impl DomainEvent for Enrolment {
    const EVENT_TYPES: &'static [EventType] = &[DEFINED, SUBSCRIBED];

    fn event_type(&self) -> EventType {
        match self {
            Self::Defined { .. } => DEFINED,
            Self::Subscribed { .. } => SUBSCRIBED,
        }
    }

    fn tags(&self) -> Tags {
        Tags::empty()
    }

    fn encode<C: Codec>(&self, codec: &C) -> Result<Bytes, CodecError> {
        codec.encode(self)
    }

    fn decode<C: Codec>(
        codec: &C,
        event_type: &EventType,
        data: &Bytes,
    ) -> Result<Self, CodecError> {
        if !Self::EVENT_TYPES.contains(event_type) {
            return Err(CodecError::UnknownEventType {
                event_type: event_type.clone(),
            });
        }
        codec.decode(data)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Seats {
    scope: Tags,
    capacity: Option<u32>,
    taken: u32,
}

impl Seats {
    fn for_course(course: &str) -> Self {
        Self {
            scope: Tags::from_pairs([("course", course)]).expect("a valid tag pair"),
            capacity: None,
            taken: 0,
        }
    }
}

impl DecisionModel for Seats {
    type Event = Enrolment;

    fn scope(&self) -> &Tags {
        &self.scope
    }

    fn apply(&mut self, event: Self::Event) {
        match event {
            Enrolment::Defined { capacity } => self.capacity = Some(capacity),
            Enrolment::Subscribed { .. } => self.taken += 1,
        }
    }
}

fn course_tags() -> Tags {
    Tags::from_pairs([("course", "c1")]).expect("a valid tag pair")
}

/// One event, with a payload written by `codec` and a metadata framing region
/// naming `tag`.
fn written<C: Codec>(event: &Enrolment, codec: &C, tag: &str, metadata: &[u8]) -> Event {
    let payload = event.encode(codec).expect("the fixture encodes");
    Event::new(event.event_type(), payload)
        .expect("a valid event type")
        .with_tags(course_tags())
        .with_metadata(framed(tag, metadata))
}

/// Reads the boundary and folds everything it nominated, with `codec` in hand.
async fn fold_with<C: Codec>(store: &MemoryEventStore, codec: &C) -> Result<Seats, CodecError> {
    let mut seats = Seats::for_course("c1");
    let query = seats.query().expect("a constrained boundary");
    let (events, _anchor) = read_decision_model(store, &query)
        .await
        .expect("the memory store reads");
    for event in &events {
        seats.absorb(event, codec)?;
    }
    Ok(seats)
}

// ---------------------------------------------------------------------------
// AC-005 — two encodings coexist, and the caller branches on neither
// ---------------------------------------------------------------------------

#[cfg(feature = "postcard")]
#[tokio::test]
async fn two_encodings_coexist_in_one_store() {
    use happenstance::Postcard;

    let store = MemoryEventStore::with_events([
        written(
            &Enrolment::Defined { capacity: 2 },
            &Json,
            <Json as Codec>::TAG,
            b"",
        ),
        written(
            &Enrolment::Subscribed {
                student: "s1".to_owned(),
            },
            &Postcard,
            <Postcard as Codec>::TAG,
            b"",
        ),
    ]);

    // One codec in hand — the default one. The postcard-tagged event decodes
    // under the codec *its own tag* names, with no branch here on encoding.
    let seats = fold_with(&store, &Json)
        .await
        .expect("both generations fold");

    assert_eq!(seats.capacity, Some(2));
    assert_eq!(seats.taken, 1);
}

#[tokio::test]
async fn application_metadata_after_the_region_is_carried_through() {
    let store = MemoryEventStore::with_events([written(
        &Enrolment::Defined { capacity: 4 },
        &Json,
        <Json as Codec>::TAG,
        b"correlation=abc",
    )]);

    let seats = fold_with(&store, &Json)
        .await
        .expect("the framing region ends where the application's bytes begin");
    assert_eq!(seats.capacity, Some(4));

    // And the application's own bytes are still on the event, untouched.
    let held = store.snapshot();
    let metadata = held
        .first()
        .and_then(|event| event.event.metadata())
        .expect("the event carries metadata");
    assert!(
        metadata.ends_with(b"correlation=abc"),
        "the application's metadata was rewritten: {metadata:?}"
    );
}

#[tokio::test]
async fn an_untagged_event_decodes_with_the_codec_in_hand() {
    // Written before the typed layer existed: no framing region at all. ADR-0021
    // requires this to decode rather than to refuse — refusing would make every
    // such log unreadable with no legal repair.
    let payload = Enrolment::Defined { capacity: 9 }
        .encode(&Json)
        .expect("the fixture encodes");
    let store = MemoryEventStore::with_events([Event::new(DEFINED, payload)
        .expect("a valid event type")
        .with_tags(course_tags())]);

    let seats = fold_with(&store, &Json)
        .await
        .expect("an untagged event decodes with the codec in hand");
    assert_eq!(seats.capacity, Some(9));
}

// ---------------------------------------------------------------------------
// AC-006 — a tag this build cannot honour is a typed refusal
// ---------------------------------------------------------------------------

#[tokio::test]
async fn unknown_tag_is_a_typed_refusal() {
    // No build of this crate carries a `protobuf` codec, in any feature
    // configuration — so this is the one tag that is unresolvable everywhere.
    let store = MemoryEventStore::with_events([written(
        &Enrolment::Defined { capacity: 2 },
        &Json,
        "protobuf",
        b"",
    )]);

    let refusal = fold_with(&store, &Json)
        .await
        .expect_err("a tag this build cannot honour is refused");

    assert!(
        matches!(&refusal, CodecError::UnknownTag { tag } if &**tag == "protobuf"),
        "expected UnknownTag carrying the offending tag, got {refusal:?}"
    );
    // The value, not the category: a reader can act on this at the call site.
    assert!(
        refusal.to_string().contains("protobuf"),
        "the message does not render the tag: {refusal}"
    );
}
