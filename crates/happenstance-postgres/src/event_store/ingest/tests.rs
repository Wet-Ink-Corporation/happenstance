//! The ingest port's own tests, each naming the clause it exercises.
//!
//! The first eight need no server. The rest are `#[ignore]`d, as every live test
//! in this crate is, and run with `-- --ignored --test-threads=1` against the
//! server `server` finds. None asserts a literal local position: every one
//! compared was read back from `append` or the log. The literal positions are
//! **origin** positions, facts a peer supplies.

use std::time::Duration;

use happenstance_core::{
    Event, EventId, EventStore, EventType, Query, ReadOptions, RecordedAt, SequencePosition,
    SequencedEvent, StoreId, StoreLimit, Tag, Tags, collect,
};
use happenstance_sync::{
    Classify, FailureClass, IngestError, IngestGroup, IngestStore, Ingested, ReplicatedEvent,
    Watermark,
};
use sqlx::postgres::PgPoolOptions;

use super::{CONFLICT_CLAUSE, OwnedGroup, foreign_statement};
use crate::error::PostgresEventStoreError;
use crate::event_store::tests::server as sqlstate;
use crate::event_store::{NewRow, Origin, PostgresEventStore, insert_statement};

mod claims;
mod family;
mod live;
mod server;

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

/// Ingests through the port, bound on the bare flavour, so the blanket impl
/// `trait_variant` derives is what every test goes through.
async fn ingest<S: IngestStore>(
    store: &S,
    groups: &[IngestGroup<'_>],
) -> Result<Ingested, IngestError<S::Error>> {
    store.ingest(groups).await
}

/// The watermark through the port, bound on the bare flavour.
async fn watermark<S: IngestStore>(store: &S) -> Watermark
where
    S::Error: core::fmt::Debug,
{
    store
        .watermark()
        .await
        .expect("the watermark of a reachable store reads")
}

/// Every event in the store, in position order.
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
async fn holds(store: &PostgresEventStore, id: EventId) -> bool {
    EventStore::contains_event_id(store, id)
        .await
        .expect("a membership probe of a reachable store answers")
}

/// One local append, answering the position the store assigned.
async fn append(store: &PostgresEventStore, events: &[Event]) -> SequencePosition {
    EventStore::append(store, events, None)
        .await
        .expect("an unconditional append to a reachable store lands")
}

/// Rows in the store's `event` table, read raw, frontier or not.
async fn rows(store: &PostgresEventStore) -> i64 {
    sqlx::query_scalar("SELECT count(*) FROM event")
        .fetch_one(store.pool())
        .await
        .expect("a count of a reachable schema answers")
}

/// `store`'s identity, asked through the port.
///
/// Named by its trait, because the store's inherent read has the same name
/// and an inherent method wins a method call.
async fn own(store: &PostgresEventStore) -> StoreId {
    IngestStore::store_id(store)
        .await
        .expect("a reachable, migrated store has an identity")
}

/// A store on a fresh schema of the live server. Its identity is read, and
/// cached, on first use.
async fn live_store() -> PostgresEventStore {
    PostgresEventStore::new(server::fresh_pool().await)
}

/// A store whose pool points nowhere and whose identity has not been read, so
/// any call that reaches the wire fails as `Driver`.
fn unidentified_store() -> PostgresEventStore {
    let pool = PgPoolOptions::new()
        .acquire_timeout(Duration::from_secs(1))
        .connect_lazy("postgres://happenstance@127.0.0.1:1/unreachable")
        .expect("a well-formed URL builds a lazy pool");
    PostgresEventStore::new(pool)
}

/// [`unidentified_store`] with a fixed identity already in its cache, so a
/// call that wrongly reaches the wire fails as `Driver` and not as the
/// refusal under test.
///
/// The cache is the store's private `store_id` field, which this module, a
/// descendant of `event_store`, can reach: so the seam needs no test-only
/// constructor.
fn unreachable_store() -> PostgresEventStore {
    let store = unidentified_store();
    store
        .store_id
        .set(peer(0x01))
        .expect("a store built a line above has read no identity");
    store
}

/// Constraint 4, one port over: generic code binds the bare flavours, and the
/// `Send` implementer satisfies them, and crosses threads.
#[test]
fn ingest_store_satisfies_the_bare_bounds() {
    fn assert_bounds<S: EventStore + IngestStore + Send + Sync>() {}
    assert_bounds::<PostgresEventStore>();
}

/// The port's `store_id` answers from the store's cache, and so never reaches
/// the wire once the identity is held.
///
/// Rejects a port `store_id` that bypasses the cache: this store's pool points
/// nowhere, so a read would fail as `Driver`.
#[tokio::test]
async fn store_id_through_the_port_answers_the_seeded_identity_without_the_wire() {
    let store = unreachable_store();

    let answered = IngestStore::store_id(&store)
        .await
        .expect("a held identity is answered without a read");
    assert_eq!(answered, peer(0x01));
}

/// An ingest whose store cannot read its identity fails as the store's error,
/// classed `Transient`, and only after every refusal that needs no identity.
///
/// Rejects an ingest that treats an unread identity as "no claims" and
/// writes, and one that reads the identity before the ceilings or before the
/// origin check: the same store still refuses a batch over the ceiling as
/// that, and an origin position past a `bigint` as that.
#[tokio::test]
async fn ingest_reports_an_unreadable_identity_as_a_store_error() {
    let store = unidentified_store();
    let from = peer(0x61);
    let one = [shipped(id(from, 1), 1, event("Shipped", "a"))];

    let refused = ingest(&store, &[IngestGroup::new(&one, &[])])
        .await
        .expect_err("an identity on a server that is not there cannot be read");
    assert!(
        matches!(
            refused,
            IngestError::Store(PostgresEventStoreError::Driver(_))
        ),
        "{refused:?}"
    );
    assert_eq!(refused.class(), FailureClass::Transient);

    let foreign: Vec<ReplicatedEvent> = (1..=257)
        .map(|position| shipped(id(from, position), 1, event("Shipped", "a")))
        .collect();
    let refused = ingest(&store, &[IngestGroup::new(&foreign, &[])])
        .await
        .expect_err("257 events exceed a ceiling of 256");
    assert!(
        matches!(
            refused,
            IngestError::ExceedsStoreLimit {
                limit: StoreLimit::EventsPerBatch,
                len: 257,
                ..
            }
        ),
        "{refused:?}"
    );

    let past_a_bigint = SequencePosition::new(u64::MAX).expect("u64::MAX is not zero");
    let unstorable = [shipped(
        EventId::new(from, past_a_bigint),
        1,
        event("Shipped", "a"),
    )];
    let refused = ingest(&store, &[IngestGroup::new(&unstorable, &[])])
        .await
        .expect_err("an origin position past a bigint cannot be stored");
    assert!(
        matches!(
            refused,
            IngestError::Store(PostgresEventStoreError::OriginPositionOutOfRange { .. })
        ),
        "the origin check must answer before the identity read reaches the wire: {refused:?}"
    );
}

/// SY-11: an empty pull, and a group with nothing foreign in it, write
/// nothing and open no transaction for it.
///
/// Rejects writing compensation for a group with nothing new, and reaching the
/// server for nothing: this store's pool points nowhere.
#[tokio::test]
async fn an_empty_ingest_writes_nothing_and_never_reaches_the_server() {
    let store = unreachable_store();
    let compensation = [event("Compensated", "a")];
    let empty: [IngestGroup<'_>; 0] = [];
    let no_foreign = [IngestGroup::new(&[], &compensation)];

    for groups in [&empty[..], &no_foreign[..]] {
        let ingested = ingest(&store, groups)
            .await
            .expect("an ingest of nothing foreign reaches no server, so cannot fail");
        assert_eq!(ingested, Ingested::default());
    }
}

/// VT-25: the ceilings are checked against the whole call's worst case, before
/// the wire. Two hundred foreign events and fifty-seven compensations are 257,
/// one over the batch ceiling; one compensation over the data ceiling is
/// refused as that.
///
/// Rejects counting only the foreign events, and checking after `BEGIN`, which
/// on this store's unreachable pool would answer `Driver`.
#[tokio::test]
async fn ceilings_are_checked_before_the_wire_against_the_worst_case() {
    let store = unreachable_store();
    let from = peer(0x31);
    let foreign: Vec<ReplicatedEvent> = (1..=200)
        .map(|position| shipped(id(from, position), 1, event("Shipped", "a")))
        .collect();
    let compensation: Vec<Event> = (0..57).map(|_| event("Compensated", "a")).collect();

    let refused = ingest(&store, &[IngestGroup::new(&foreign, &compensation)])
        .await
        .expect_err("257 events exceed a ceiling of 256");
    assert!(
        matches!(
            refused,
            IngestError::ExceedsStoreLimit {
                limit: StoreLimit::EventsPerBatch,
                len: 257,
                ..
            }
        ),
        "{refused:?}"
    );
    assert_eq!(refused.class(), FailureClass::Split);

    let oversized = [Event::new(
        "Compensated",
        vec![0_u8; PostgresEventStore::MAX_EVENT_DATA_LEN + 1],
    )
    .expect("a valid event type")];
    let losing = [shipped(id(from, 1), 1, event("Reserved", "seat"))];
    let refused = ingest(&store, &[IngestGroup::new(&losing, &oversized)])
        .await
        .expect_err("a compensation over the data ceiling never fits");
    match refused {
        IngestError::ExceedsStoreLimit {
            limit: StoreLimit::EventDataLen,
            len,
            ..
        } => assert_eq!(len, PostgresEventStore::MAX_EVENT_DATA_LEN + 1),
        ref other => panic!("expected the data ceiling, got {other:?}"),
    }
    assert_eq!(refused.class(), FailureClass::Park);
}

/// An origin position past `i64::MAX` is refused, naming the first one, before
/// the wire.
///
/// Rejects saturating the conversion, as the query-bound helper `as_i64` does:
/// both events would be written at `i64::MAX`, the second would be a silent
/// skip, and the ingest would report `skipped == 1`.
#[tokio::test]
async fn an_origin_position_past_i64_is_refused_not_saturated() {
    let store = unreachable_store();
    let from = peer(0x41);
    let batch = [
        shipped(id(from, 1), 1, event("Shipped", "a")),
        shipped(id(from, u64::MAX - 1), 2, event("Shipped", "b")),
        shipped(id(from, u64::MAX), 3, event("Shipped", "c")),
    ];

    let refused = ingest(&store, &[IngestGroup::new(&batch, &[])])
        .await
        .expect_err("an origin position past a bigint cannot be stored");
    match refused {
        IngestError::Store(PostgresEventStoreError::OriginPositionOutOfRange { position }) => {
            assert_eq!(position.get(), u64::MAX - 1, "not the first offender");
        }
        ref other => panic!("expected OriginPositionOutOfRange, got {other:?}"),
    }
    assert_eq!(refused.class(), FailureClass::Park);
}

/// VT-10: ingest sends append's `INSERT` text with the conflict clause
/// appended, and nothing else changed, so ingested rows take the same columns,
/// the same sequence and the same `xact_id` default as appended ones.
///
/// Rejects a second write path for ingest: any other statement text fails it.
#[test]
fn ingest_sends_appends_insert_plus_the_conflict_clause() {
    let own = peer(0x51);
    let sent = [
        shipped(id(peer(0x52), 4), 4, event("Shipped", "a")),
        shipped(id(peer(0x52), 9), 9, event("Shipped", "b")),
    ];
    let owned = OwnedGroup::try_from_group(&IngestGroup::new(&sent, &[]))
        .expect("both origin positions fit a bigint");
    let positions = [41, 42];
    let recorded_at = RecordedAt::from_millis(1);

    let local = insert_statement(
        own,
        sent.iter().map(|replicated| NewRow {
            event: &replicated.event,
            origin: Origin::Local,
            recorded_at,
        }),
        &positions,
    );
    let foreign = foreign_statement(own, &owned.events, &positions);

    let suffix = foreign.sql().strip_prefix(local.sql()).unwrap_or_else(|| {
        panic!(
            "{:?} does not start with append's {:?}",
            foreign.sql(),
            local.sql()
        )
    });
    // The clause itself is pinned by the live tests, which the server would
    // fail on a clause it cannot infer the index from; this pins that it is
    // the only difference.
    assert_eq!(suffix, CONFLICT_CLAUSE);
}

/// SY-36: each failure states its class, at the boundaries a runner acts on.
#[tokio::test]
async fn postgres_errors_state_their_class() {
    use FailureClass::{Park, Stop, Transient};
    use PostgresEventStoreError as E;

    let panicked = tokio::spawn(async { panic!("a scripted panic, for its JoinError") })
        .await
        .expect_err("the task panicked");
    let aborted = {
        let handle = tokio::spawn(core::future::pending::<()>());
        handle.abort();
        handle.await.expect_err("the task was aborted")
    };
    let cases = [
        (sqlstate("08006"), Transient),
        (sqlstate("53300"), Transient),
        (sqlstate("40001"), Transient),
        (sqlstate("40P01"), Transient),
        (sqlstate("57P01"), Transient),
        (sqlstate("57P02"), Transient),
        (sqlstate("57P03"), Transient),
        (sqlstate("23505"), Stop),
        (sqlstate("P0001"), Stop),
        (sqlstate("42P01"), Stop),
        (
            E::Driver(sqlx::Error::Io(std::io::Error::other("reset"))),
            Transient,
        ),
        (E::Driver(sqlx::Error::PoolTimedOut), Transient),
        (E::Driver(sqlx::Error::PoolClosed), Stop),
        (E::Driver(sqlx::Error::Protocol("scripted".into())), Stop),
        (E::Worker(aborted), Transient),
        (E::Worker(panicked), Stop),
        (
            E::OriginPositionOutOfRange {
                position: SequencePosition::FIRST,
            },
            Park,
        ),
        (E::PositionOutOfRange { value: 0 }, Stop),
        (
            E::EventType(EventType::new("").expect_err("an empty event type is invalid")),
            Stop,
        ),
        (
            E::Tag(Tag::new("").expect_err("an empty tag is invalid")),
            Stop,
        ),
        (E::MissingIdentity, Stop),
        (E::MalformedIdentity { len: 3 }, Stop),
        (
            E::UnstampedEvent {
                position: SequencePosition::FIRST,
            },
            Stop,
        ),
        (E::NoRuntime, Stop),
        (E::UnreadableTransactionId, Stop),
    ];
    for (error, class) in cases {
        assert_eq!(error.class(), class, "{error:?}");
    }
}
