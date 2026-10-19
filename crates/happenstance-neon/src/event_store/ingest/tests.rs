//! The ingest port's own tests, each naming the clause it exercises.
//!
//! Everything in this file needs no endpoint: each test runs over a scripted
//! transport that records what it was sent and answers bodies shaped as the
//! endpoint's. The live tests are in `live.rs` and `claims.rs`, and the family
//! in `family.rs`, all `#[ignore]`d as every live test in this crate is. None
//! asserts a literal local position: the literal positions are **origin**
//! positions, facts a peer supplies.

use core::cell::{Cell, RefCell};
use core::future::Future;
use core::pin::pin;
use core::task::{Context, Poll, Waker};
use std::collections::VecDeque;

use happenstance_core::{
    Event, EventId, EventStore, EventType, Query, ReadOptions, RecordedAt, SequencePosition,
    SequencedEvent, StoreId, StoreLimit, Tag, Tags, collect,
};
use happenstance_sync::{
    Classify, FailureClass, IngestError, IngestGroup, IngestStore, Ingested, ReplicatedEvent,
    Watermark,
};

use super::{NeonIngestStore, UNHELD_GUC, UNHELD_SQLSTATE, refusal_statement};
use crate::config::NeonConfig;
use crate::error::{NeonError, NeonSqlError};
use crate::event_store::{NeonEventStore, decode_body};
use crate::transport::{
    HttpResponse, IsolationLevel, NullTransport, NullTransportError, ReadLedger, SqlRequest,
    SqlStatement, SqlTransport,
};
use crate::wire::ResultSet;

mod claims;
mod endpoint;
mod family;
mod live;

/// A store another peer runs.
fn peer(byte: u8) -> StoreId {
    StoreId::from_bytes([byte; 16])
}

/// The identity `store` minted at its own `position`.
fn id(store: StoreId, position: u64) -> EventId {
    EventId::new(
        store,
        SequencePosition::new(position).expect("test origin positions are non-zero"),
    )
}

/// An event of `kind`, tagged `k:<tag>`.
fn event(kind: &str, tag: &str) -> Event {
    Event::new(kind, b"payload".to_vec())
        .expect("test event types are valid")
        .with_tags(Tags::from_pairs([("k", tag)]).expect("test tags are valid"))
}

/// An event as a peer ships it.
fn shipped(id: EventId, millis: i64, event: Event) -> ReplicatedEvent {
    ReplicatedEvent::new(id, RecordedAt::from_millis(millis), event)
}

/// Ingests through the port, bound on the bare flavour, as generic code binds
/// it.
async fn ingest<S: IngestStore>(
    store: &S,
    groups: &[IngestGroup<'_>],
) -> Result<Ingested, IngestError<S::Error>> {
    store.ingest(groups).await
}

/// The watermark through the port.
async fn watermark<S: IngestStore>(store: &S) -> Watermark
where
    S::Error: core::fmt::Debug,
{
    store
        .watermark()
        .await
        .expect("the watermark of a reachable store reads")
}

/// Every event in the store beneath its frontier, in position order.
async fn log<S: EventStore>(store: &S) -> Vec<SequencedEvent>
where
    S::Error: core::fmt::Debug,
{
    collect(store.read(&Query::all(), ReadOptions::default()))
        .await
        .expect("the log of a reachable store reads")
}

/// The one event in `log` carrying `id`.
fn carrying(log: &[SequencedEvent], id: EventId) -> &SequencedEvent {
    let mut matching = log.iter().filter(|event| event.id == id);
    let found = matching
        .next()
        .unwrap_or_else(|| panic!("no event carries {id}"));
    assert!(matching.next().is_none(), "two events carry {id}");
    found
}

/// Whether `store` holds `id`.
async fn holds<S: EventStore>(store: &S, id: EventId) -> bool
where
    S::Error: core::fmt::Debug,
{
    store
        .contains_event_id(id)
        .await
        .expect("a membership probe of a reachable store answers")
}

/// One local append, answering the position the store assigned.
async fn append<S: EventStore>(store: &S, events: &[Event]) -> SequencePosition
where
    S::Error: core::fmt::Debug,
{
    store
        .append(events, None)
        .await
        .expect("an unconditional append to a reachable store lands")
}

/// `store`'s identity, asked through the port.
async fn own<T: SqlTransport>(store: &NeonIngestStore<T>) -> StoreId {
    IngestStore::store_id(store)
        .await
        .expect("the wrapper answers the identity `open` read")
}

/// `statements` as one request at `READ COMMITTED`, and its answer.
async fn send<T: SqlTransport>(
    store: &NeonIngestStore<T>,
    statements: Vec<SqlStatement>,
) -> HttpResponse
where
    T::Error: core::fmt::Debug,
{
    let request = match statements.len() {
        1 => SqlRequest::single(statements.into_iter().next().expect("one statement")),
        _ => SqlRequest::batch(statements, IsolationLevel::ReadCommitted),
    };
    store
        .event_store()
        .transport()
        .round_trip(request)
        .await
        .expect("a reachable endpoint answers")
}

/// The first result set of `sql`'s answer, which must succeed.
async fn select<T: SqlTransport>(store: &NeonIngestStore<T>, sql: String) -> ResultSet
where
    T::Error: core::fmt::Debug,
{
    let response = send(store, vec![SqlStatement::new(sql)]).await;
    let body = decode_body::<T::Error>(&response, usize::MAX).expect("the query succeeds");
    body.result_sets()
        .first()
        .cloned()
        .expect("a query answers a result set")
}

/// Rows in the store's event table, read raw, frontier or not.
async fn rows<T: SqlTransport>(store: &NeonIngestStore<T>) -> u64
where
    T::Error: core::fmt::Debug,
{
    let table = store.event_store().config().qualified_event();
    select(store, format!("SELECT count(*)::text AS n FROM {table}"))
        .await
        .text(0, "n")
        .and_then(|n| n.parse().ok())
        .expect("a count is a number")
}

/// The identity a scripted store is built around.
const OWN: StoreId = StoreId::from_bytes([9; 16]);

/// A transport that answers each round trip with the next scripted response,
/// records every request it was sent, and fences writes on a ledger the test
/// holds. A request past the script fails, so a call that reaches the wire
/// unexpectedly is visible in its error as well as in the record.
#[derive(Debug, Default)]
struct Scripted {
    answers: RefCell<VecDeque<HttpResponse>>,
    sent: RefCell<Vec<SqlRequest>>,
    round_trips: Cell<usize>,
    ledger: ReadLedger,
}

impl Scripted {
    /// A transport that answers once, with `status` and `body`.
    fn answering(status: u16, body: &'static [u8]) -> Self {
        let scripted = Self::default();
        scripted
            .answers
            .borrow_mut()
            .push_back(HttpResponse::new(status, body));
        scripted
    }

    /// A store over this transport, around [`OWN`].
    fn store(&self) -> NeonIngestStore<&Self> {
        NeonIngestStore {
            store: NeonEventStore::new(self, NeonConfig::default().with_schema("hs_1")),
            store_id: OWN,
        }
    }
}

impl SqlTransport for &Scripted {
    type Error = std::io::Error;

    fn round_trip(
        &self,
        request: SqlRequest,
    ) -> impl Future<Output = Result<HttpResponse, Self::Error>> {
        self.round_trips.set(self.round_trips.get() + 1);
        self.sent.borrow_mut().push(request);
        let answer = self.answers.borrow_mut().pop_front();
        core::future::ready(answer.ok_or_else(|| std::io::Error::other("nothing scripted")))
    }

    fn reads_settled(&self) -> impl Future<Output = ()> {
        self.ledger.settled()
    }
}

/// The answer a call that landed two, skipped one and compensated one gets:
/// the counts row, then the refusal's empty `DO` set.
const LANDED: &[u8] = br#"{"results":[{"fields":[],"rows":[{"appended":"2","skipped":"1","compensated":"1","unheld":""}],"command":"SELECT","rowCount":1,"rowAsArray":false},{"fields":[],"rows":[],"command":"DO","rowCount":null,"rowAsArray":false}]}"#;

/// Constraint 4, one port over: generic code binds the bare flavours, and the
/// wrapper satisfies them at a concrete transport.
#[test]
fn ingest_store_satisfies_the_bare_bounds() {
    fn assert_bounds<S: EventStore + IngestStore>() {}
    assert_bounds::<NeonIngestStore<NullTransport>>();
}

/// SY-11: an empty pull, and a group with nothing foreign in it, write
/// nothing and send nothing.
///
/// Rejects writing compensation for a group with nothing new, and a request
/// for nothing.
#[tokio::test]
async fn an_empty_ingest_never_reaches_the_transport() {
    let transport = Scripted::default();
    let store = transport.store();
    let compensation = [event("Compensated", "a")];
    let empty: [IngestGroup<'_>; 0] = [];
    let no_foreign = [IngestGroup::new(&[], &compensation)];

    for groups in [&empty[..], &no_foreign[..]] {
        let ingested = ingest(&store, groups)
            .await
            .expect("an ingest of nothing foreign sends nothing, so cannot fail");
        assert_eq!(ingested, Ingested::default());
    }
    assert_eq!(transport.round_trips.get(), 0);
}

/// VT-25: the ceilings are checked against the whole call's worst case, before
/// the wire. A hundred foreign events and twenty-nine compensations are 129,
/// one over the batch ceiling; a compensation behind a group's head over the
/// data ceiling, and a foreign event's second over the tag ceiling, are
/// refused as those.
///
/// Rejects counting only the foreign events, checking only a group's head, and
/// checking after the send.
#[tokio::test]
async fn ceilings_are_checked_before_the_wire_against_the_worst_case() {
    let transport = Scripted::default();
    let store = transport.store();
    let from = peer(0x31);
    let foreign: Vec<ReplicatedEvent> = (1..=100)
        .map(|position| shipped(id(from, position), 1, event("Shipped", "a")))
        .collect();
    let compensation: Vec<Event> = (0..29).map(|_| event("Compensated", "a")).collect();

    let refused = ingest(&store, &[IngestGroup::new(&foreign, &compensation)])
        .await
        .expect_err("129 events exceed a ceiling of 128");
    assert!(
        matches!(
            refused,
            IngestError::ExceedsStoreLimit {
                limit: StoreLimit::EventsPerBatch,
                len: 129,
                ..
            }
        ),
        "{refused:?}"
    );
    assert_eq!(refused.class(), FailureClass::Split);

    let data_len = NeonEventStore::<NullTransport>::MAX_EVENT_DATA_LEN + 1;
    let oversized = [
        event("Compensated", "a"),
        Event::new("Compensated", vec![0_u8; data_len]).expect("a valid event type"),
    ];
    let losing = [shipped(id(from, 1), 1, event("Reserved", "seat"))];
    let refused = ingest(&store, &[IngestGroup::new(&losing, &oversized)])
        .await
        .expect_err("a compensation over the data ceiling never fits");
    assert!(
        matches!(
            refused,
            IngestError::ExceedsStoreLimit { limit: StoreLimit::EventDataLen, len, .. }
                if len == data_len
        ),
        "{refused:?}"
    );
    assert_eq!(refused.class(), FailureClass::Park);

    let tag_count = NeonEventStore::<NullTransport>::MAX_TAGS_PER_EVENT + 1;
    let tags: Tags = (0..tag_count)
        .map(|n| Tag::new(format!("t:{n}")).expect("a valid tag"))
        .collect();
    let crowded = [
        shipped(id(from, 1), 1, event("Shipped", "a")),
        shipped(
            id(from, 2),
            2,
            Event::new("Shipped", b"x".to_vec())
                .expect("a valid event type")
                .with_tags(tags),
        ),
    ];
    let refused = ingest(&store, &[IngestGroup::new(&crowded, &[])])
        .await
        .expect_err("a foreign event over the tag ceiling never fits");
    assert!(
        matches!(
            refused,
            IngestError::ExceedsStoreLimit { limit: StoreLimit::TagsPerEvent, len, .. }
                if len == tag_count
        ),
        "{refused:?}"
    );
    assert_eq!(refused.class(), FailureClass::Park);
    assert_eq!(transport.round_trips.get(), 0, "a refusal reached the wire");
}

/// An origin position past `i64::MAX` is refused through the port too, naming
/// it, before the wire.
#[tokio::test]
async fn an_origin_position_past_bigint_is_refused_through_ingest() {
    let transport = Scripted::default();
    let store = transport.store();
    let batch = [shipped(id(peer(0x41), u64::MAX), 1, event("Shipped", "a"))];

    let refused = ingest(&store, &[IngestGroup::new(&batch, &[])])
        .await
        .expect_err("an origin position past a bigint cannot be stored");
    assert!(
        matches!(
            refused,
            IngestError::Store(NeonError::OriginPositionOutOfRange { position: u64::MAX })
        ),
        "{refused:?}"
    );
    assert_eq!(refused.class(), FailureClass::Park);
    assert_eq!(transport.round_trips.get(), 0);
}

/// The refusal travels as this adapter's SQLSTATE, through a setting local to
/// the transaction, and the statement that raises it reads the setting the
/// write sets.
///
/// Rejects a session-level setting, which can outlive its request behind the
/// proxy, and a refusal reading a different name than the one set.
#[test]
fn the_refusal_is_raised_with_this_adapters_sqlstate_and_a_transaction_local_setting() {
    let store = NeonEventStore::new(NullTransport::new(), NeonConfig::default());
    let sent = [shipped(id(peer(0x42), 1), 1, event("Shipped", "a"))];
    let request = store
        .ingest_request::<NullTransportError>(&[IngestGroup::new(&sent, &[])])
        .expect("an origin position that fits a bigint");

    let write = &request.statements[0].query;
    let setter = format!("set_config('{UNHELD_GUC}', ");
    let at = write
        .find(&setter)
        .expect("the write sets the claim's index");
    assert!(
        write[at..].contains(", true) AS unheld"),
        "the setting must be transaction-local: {write}"
    );
    let refusal = refusal_statement().query;
    assert_eq!(request.statements[1].query, refusal);
    assert_eq!(
        refusal
            .matches(&format!("current_setting('{UNHELD_GUC}'"))
            .count(),
        2,
        "the refusal reads the setting the write sets, and nothing else: {refusal}"
    );
    assert!(refusal.contains(&format!("ERRCODE = '{UNHELD_SQLSTATE}'")));
    assert!(refusal.contains(&format!("DETAIL = current_setting('{UNHELD_GUC}')")));
}

/// Two groups of two events, the third of which the refusal will name.
fn two_groups() -> [[ReplicatedEvent; 2]; 2] {
    let from = peer(0x43);
    [
        [
            shipped(id(from, 1), 1, event("Shipped", "a")),
            shipped(id(from, 2), 2, event("Shipped", "b")),
        ],
        [
            shipped(id(OWN, 7), 3, event("Local", "forged")),
            shipped(id(from, 4), 4, event("Shipped", "d")),
        ],
    ]
}

/// SY-38: the refusal names the event the peer sent at the 1-based index the
/// server reported, as `Stop`. A refusal carrying no usable index stays the
/// SQL error it is, still `Stop`.
///
/// Rejects an id rebuilt from the cached identity, an off-by-one on the
/// index, and a panic on a malformed detail.
#[tokio::test]
async fn a_refusal_answer_names_the_claim_it_carries() {
    const NAMES_THREE: &[u8] = br#"{"message":"an ingested event claims this store's own identity","code":"HS038","detail":"3","hint":null,"severity":"ERROR"}"#;
    const NO_INDEX: [&[u8]; 4] = [
        br#"{"message":"m","code":"HS038","detail":"0","hint":null,"severity":"ERROR"}"#,
        br#"{"message":"m","code":"HS038","detail":"5","hint":null,"severity":"ERROR"}"#,
        br#"{"message":"m","code":"HS038","detail":"x","hint":null,"severity":"ERROR"}"#,
        br#"{"message":"m","code":"HS038","detail":null,"hint":null,"severity":"ERROR"}"#,
    ];
    let groups = two_groups();
    let groups = [
        IngestGroup::new(&groups[0], &[]),
        IngestGroup::new(&groups[1], &[]),
    ];

    let transport = Scripted::answering(400, NAMES_THREE);
    let refused = ingest(&transport.store(), &groups)
        .await
        .expect_err("the endpoint refused the call");
    assert!(
        matches!(refused, IngestError::UnheldOwnIdentity { id, .. } if id == groups[1].events[0].id),
        "{refused:?}"
    );
    assert_eq!(refused.class(), FailureClass::Stop);
    assert_eq!(transport.round_trips.get(), 1);
    assert_eq!(transport.sent.borrow()[0].statements.len(), 2);

    for body in NO_INDEX {
        let transport = Scripted::answering(400, body);
        let refused = ingest(&transport.store(), &groups)
            .await
            .expect_err("the endpoint refused the call");
        assert!(
            matches!(&refused, IngestError::Store(NeonError::Sql(sql)) if sql.code.as_deref() == Some(UNHELD_SQLSTATE)),
            "{refused:?}"
        );
        assert_eq!(refused.class(), FailureClass::Stop);
    }
}

/// A serialisation failure or a deadlock is `Busy`, sent once: the endpoint
/// answered, so it aborted the batch whole, and the runner owns the retry. A
/// unique violation is the store's own failure.
///
/// Rejects retrying inside `ingest`, and reporting an aborted transaction as
/// `Store`.
#[tokio::test]
async fn a_serialisation_failure_or_deadlock_is_busy_and_sent_once() {
    const ABORTED: [&[u8]; 2] = [
        br#"{"message":"could not serialize access due to concurrent update","code":"40001","detail":null,"hint":null,"severity":"ERROR"}"#,
        br#"{"message":"deadlock detected","code":"40P01","detail":null,"hint":null,"severity":"ERROR"}"#,
    ];
    const UNIQUE: &[u8] = br#"{"message":"duplicate key value violates unique constraint","code":"23505","detail":null,"hint":null,"severity":"ERROR"}"#;
    let sent = [shipped(id(peer(0x44), 1), 1, event("Shipped", "a"))];
    let groups = [IngestGroup::new(&sent, &[])];

    for body in ABORTED {
        let transport = Scripted::answering(400, body);
        let refused = ingest(&transport.store(), &groups)
            .await
            .expect_err("the endpoint aborted the call");
        assert!(
            matches!(refused, IngestError::Busy(NeonError::Sql(_))),
            "{refused:?}"
        );
        assert_eq!(refused.class(), FailureClass::Transient);
        assert_eq!(transport.round_trips.get(), 1, "retried inside ingest");
    }

    let transport = Scripted::answering(400, UNIQUE);
    let refused = ingest(&transport.store(), &groups)
        .await
        .expect_err("the endpoint refused the call");
    assert!(
        matches!(&refused, IngestError::Store(NeonError::Sql(sql)) if sql.is_unique_violation()),
        "{refused:?}"
    );
    assert_eq!(refused.class(), FailureClass::Stop);
}

/// The counts come from the first result set, and a missing one is named.
///
/// Rejects reading the refusal's result set, and a missing count read as zero.
#[tokio::test]
async fn a_success_answer_decodes_its_three_counts() {
    const NO_COMPENSATED: &[u8] = br#"{"results":[{"fields":[],"rows":[{"appended":"2","skipped":"1","unheld":""}],"command":"SELECT","rowCount":1,"rowAsArray":false},{"fields":[],"rows":[{"compensated":"1"}],"command":"DO","rowCount":null,"rowAsArray":false}]}"#;
    let sent = [shipped(id(peer(0x45), 1), 1, event("Shipped", "a"))];
    let groups = [IngestGroup::new(&sent, &[])];

    let transport = Scripted::answering(200, LANDED);
    let ingested = ingest(&transport.store(), &groups)
        .await
        .expect("the answer decodes");
    assert_eq!(
        (ingested.appended, ingested.skipped, ingested.compensated),
        (2, 1, 1)
    );

    let transport = Scripted::answering(200, NO_COMPENSATED);
    let refused = ingest(&transport.store(), &groups)
        .await
        .expect_err("a count is missing");
    assert!(
        matches!(
            refused,
            IngestError::Store(NeonError::MissingColumn {
                column: "compensated"
            })
        ),
        "{refused:?}"
    );
}

/// The identity is read once, at `open`, from the meta row; an absent row and
/// a malformed one are named.
#[tokio::test]
async fn the_identity_is_read_once_at_open() {
    /// Sixteen bytes `0x01..=0x10`, base64.
    const HELD: &[u8] = br#"{"fields":[],"command":"SELECT","rowCount":1,"rows":[{"v":"AQIDBAUGBwgJCgsMDQ4PEA=="}]}"#;
    const ABSENT: &[u8] = br#"{"fields":[],"command":"SELECT","rowCount":0,"rows":[]}"#;
    /// Fifteen bytes.
    const SHORT: &[u8] =
        br#"{"fields":[],"command":"SELECT","rowCount":1,"rows":[{"v":"AQIDBAUGBwgJCgsMDQ4P"}]}"#;
    let open =
        |transport| NeonIngestStore::open(NeonEventStore::new(transport, NeonConfig::default()));

    let transport = Scripted::answering(200, HELD);
    let store = open(&transport).await.expect("a held identity reads");
    let expected: [u8; 16] = core::array::from_fn(|n| u8::try_from(n + 1).expect("under 17"));
    assert_eq!(own(&store).await, StoreId::from_bytes(expected));
    assert_eq!(own(&store).await, StoreId::from_bytes(expected));
    assert_eq!(transport.round_trips.get(), 1, "read once");
    assert!(transport.sent.borrow()[0].read_only);

    let transport = Scripted::answering(200, ABSENT);
    assert!(matches!(
        open(&transport).await,
        Err(NeonError::MissingIdentity)
    ));
    let transport = Scripted::answering(200, SHORT);
    assert!(matches!(
        open(&transport).await,
        Err(NeonError::MalformedIdentity { len: 15 })
    ));
}

/// The watermark is one row per origin, folded; a malformed identity is named.
#[tokio::test]
async fn the_watermark_answer_folds_per_origin() {
    /// `[1; 16]` at 9 and `[2; 16]` at 4.
    const TWO: &[u8] = br#"{"fields":[],"command":"SELECT","rowCount":2,"rows":[{"s":"AQEBAQEBAQEBAQEBAQEBAQ==","p":"9"},{"s":"AgICAgICAgICAgICAgICAg==","p":"4"}]}"#;
    const SHORT: &[u8] =
        br#"{"fields":[],"command":"SELECT","rowCount":1,"rows":[{"s":"AQEB","p":"9"}]}"#;

    let transport = Scripted::answering(200, TWO);
    let marks = IngestStore::watermark(&transport.store())
        .await
        .expect("the answer decodes");
    let mut expected = Watermark::new();
    expected.advance(peer(1), SequencePosition::new(9).expect("non-zero"));
    expected.advance(peer(2), SequencePosition::new(4).expect("non-zero"));
    assert_eq!(marks, expected);
    assert!(transport.sent.borrow()[0].read_only);

    let transport = Scripted::answering(200, SHORT);
    assert!(matches!(
        IngestStore::watermark(&transport.store()).await,
        Err(NeonError::MalformedIdentity { len: 3 })
    ));
}

/// SY-36: each failure states its class, at the boundaries a runner acts on.
#[test]
fn neon_errors_state_their_class() {
    use FailureClass::{Park, Split, Stop, Transient};
    type E = NeonError<NullTransportError>;

    let sql = |code: Option<&str>| -> E {
        NeonError::Sql(NeonSqlError {
            message: "scripted".into(),
            code: code.map(Into::into),
            detail: None,
            hint: None,
            severity: None,
        })
    };
    let http = |status| -> E {
        NeonError::Http {
            status,
            body: "scripted".into(),
        }
    };
    let mut cases = vec![
        (NeonError::Transport(NullTransportError), Transient),
        (http(408), Transient),
        (http(429), Transient),
        (http(500), Transient),
        (http(503), Transient),
        (http(599), Transient),
        (http(413), Split),
        (http(400), Stop),
        (http(401), Stop),
        (http(403), Stop),
        (http(404), Stop),
        (sql(None), Stop),
        (sql(Some(UNHELD_SQLSTATE)), Stop),
        (NeonError::ResponseTooLarge { bytes: 2, limit: 1 }, Stop),
        (
            NeonError::MalformedResponse(
                serde_json::from_str::<serde_json::Value>("{").expect_err("not JSON"),
            ),
            Stop,
        ),
        (NeonError::InvalidPosition { value: 0 }, Stop),
        (NeonError::MissingColumn { column: "c" }, Stop),
        (
            NeonError::StoredEventType(EventType::new("").expect_err("an empty type is invalid")),
            Stop,
        ),
        (
            NeonError::StoredTag(Tag::new("").expect_err("an empty tag is invalid")),
            Stop,
        ),
        (NeonError::UnstampedEvent { position: 1 }, Stop),
        (NeonError::MalformedIdentity { len: 3 }, Stop),
        (NeonError::MalformedPayload, Stop),
        (NeonError::ForeignBatch, Stop),
        (NeonError::MissingIdentity, Stop),
        (NeonError::OriginPositionOutOfRange { position: 1 }, Park),
    ];
    for code in [
        "08006", "53300", "40001", "40P01", "57P01", "57P02", "57P03",
    ] {
        cases.push((sql(Some(code)), Transient));
    }
    for code in ["23505", "P0001", "42P01", "22P02"] {
        cases.push((sql(Some(code)), Stop));
    }
    for (error, class) in cases {
        assert_eq!(error.class(), class, "{error:?}");
    }
}

/// ES-11: an ingest is a write, so it leaves only after every read the same
/// transport dispatched earlier has been answered.
///
/// Polled by hand, with no runtime and no timer. Rejects an ingest that skips
/// the fence.
#[test]
fn ingest_waits_for_earlier_reads_to_settle() {
    let transport = Scripted::answering(200, LANDED);
    let store = transport.store();
    let ticket = transport.ledger.dispatch();
    let sent = [shipped(id(peer(0x46), 1), 1, event("Shipped", "a"))];
    let groups = [IngestGroup::new(&sent, &[])];
    let mut cx = Context::from_waker(Waker::noop());

    let mut ingesting = pin!(IngestStore::ingest(&store, &groups));
    assert!(ingesting.as_mut().poll(&mut cx).is_pending());
    assert_eq!(
        transport.round_trips.get(),
        0,
        "sent past an unanswered read"
    );

    ticket.settle();
    match ingesting.poll(&mut cx) {
        Poll::Ready(Ok(ingested)) => assert_eq!(ingested.appended, 2),
        other => panic!("expected the ingest to land once the read settled, got {other:?}"),
    }
    assert_eq!(transport.round_trips.get(), 1);
}
