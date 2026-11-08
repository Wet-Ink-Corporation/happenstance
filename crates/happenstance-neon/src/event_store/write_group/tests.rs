//! The write-group request shapes and the group read's decoding, with no
//! endpoint (#420).
//!
//! Shape checks of this adapter's own requests, which CF-33 leaves to the
//! adapter: the suite may not count operations, but nothing stops an adapter
//! pinning what it sends. The live half, that no provisional tag survives a
//! commit and that a guard round-trips through a real `bytea`, is in
//! `ingest/tests/groups.rs`, run by CI's `live-neon` job.

use core::future::Future;
use core::num::NonZeroUsize;

use base64::Engine as _;
use happenstance_core::{
    AppendCondition, Event, EventId, Query, QueryItem, RecordedAt, SequencePosition, StoreId,
};
use happenstance_sync::write_group::{GuardCodecError, decode_guard, encode_guard};
use happenstance_sync::{IngestGroup, ReplicatedEvent, Waiting, WriteGroup, WriteGroupStore};

use super::{FOREIGN_ORIGIN, PROVISIONAL_GROUP};
use crate::config::NeonConfig;
use crate::error::NeonError;
use crate::event_store::{APPEND_GROUP, BASE64, FRONTIER, NeonEventStore, ingest};
use crate::transport::{HttpResponse, NullTransport, SqlRequest, SqlTransport};

fn store() -> NeonEventStore<NullTransport> {
    NeonEventStore::new(
        NullTransport::new(),
        NeonConfig::default().with_schema("hs_w"),
    )
}

fn guard(kind: &str) -> AppendCondition {
    AppendCondition::new(Query::from_item(
        QueryItem::of_types([kind]).expect("a valid query item"),
    ))
}

fn shipped(position: u64) -> ReplicatedEvent {
    ReplicatedEvent::new(
        EventId::new(
            StoreId::from_bytes([0x33; 16]),
            SequencePosition::new(position).expect("non-zero"),
        ),
        RecordedAt::from_millis(5),
        Event::new("Shipped", b"x".to_vec()).expect("a valid event type"),
    )
}

/// The stamp is the ingest request's third statement, after the refusal, in
/// the same batch, so no provisional tag can commit, and a refused call
/// stamps nothing. Its one parameter is every group's stored guard, base64,
/// `null` for a group with none, in call order.
///
/// **Named wrong request:** the two-statement request this adapter sent
/// before #420, whose provisional tags would commit; the count fails on it.
#[test]
fn the_ingest_request_ends_with_the_stamp_after_the_refusal() {
    let store = store();
    let (a, b, c) = ([shipped(1)], [shipped(2)], [shipped(3)]);
    let (first, third) = (guard("First"), guard("Third"));
    let groups = [
        IngestGroup::new(&a, &[]).with_guard(Some(&first)),
        IngestGroup::new(&b, &[]),
        IngestGroup::new(&c, &[]).with_guard(Some(&third)),
    ];

    let request = store
        .ingest_request::<std::io::Error>(&groups)
        .expect("every guard encodes");

    let [_, refusal, stamp] = request.statements.as_slice() else {
        panic!(
            "the write, the refusal, the stamp: {:?}",
            request.statements
        );
    };
    assert_eq!(refusal.query, ingest::refusal_statement().query);
    let event = store.config.qualified_event();
    assert!(
        stamp
            .query
            .starts_with(&format!("UPDATE {event} AS e SET write_group = g.first")),
        "{}",
        stamp.query
    );
    assert!(
        stamp
            .query
            .matches("xact_id = pg_current_xact_id()")
            .count()
            == 2
            && stamp.query.contains("write_group < 0"),
        "only this transaction's provisional rows are stamped: {}",
        stamp.query
    );
    let [serde_json::Value::String(guards)] = stamp.params.as_slice() else {
        panic!("one jsonb parameter: {:?}", stamp.params);
    };
    let guards: Vec<Option<String>> =
        serde_json::from_str(guards).expect("the parameter is a JSON array");
    let decoded: Vec<Option<AppendCondition>> = guards
        .iter()
        .map(|stored| {
            stored.as_ref().map(|encoded| {
                let bytes = BASE64.decode(encoded).expect("base64");
                decode_guard(&bytes).expect("the stored form")
            })
        })
        .collect();
    assert_eq!(decoded, [Some(first), None, Some(third)]);
}

/// Every row an ingest writes, foreign or compensation, carries the
/// provisional tag, and the compensation reads its group's index to do so.
///
/// Rejects a compensation written without one: the stamp would leave it
/// `NULL`, a group of its own, which a relay forwards apart from its losing
/// event (SY-2).
#[test]
fn every_ingested_row_is_written_under_a_provisional_tag() {
    let store = store();
    let rows = [shipped(1)];
    let reversal = [Event::new("Reversed", b"r".to_vec()).expect("a valid event type")];

    let request = store
        .ingest_request::<std::io::Error>(&[IngestGroup::new(&rows, &reversal)])
        .expect("no guard to encode");
    let write = &request.statements.first().expect("the write").query;

    assert_eq!(
        write.matches(&format!(", {PROVISIONAL_GROUP} ")).count(),
        2,
        "both inserts, the foreign rows' and the compensation's: {write}"
    );
    assert!(write.contains("u.tags, u.grp FROM"), "{write}");
}

/// The foreign rows' origin spells the same provisional tag the compensation
/// takes, so the stamp's `GROUP BY write_group` sees one tag per group.
#[test]
fn the_foreign_origin_ends_with_the_provisional_group() {
    assert!(FOREIGN_ORIGIN.ends_with(&format!(", {PROVISIONAL_GROUP}")));
}

/// An append names its group in the statement that writes it: the ninth
/// column, the lowest position the statement gives out.
#[test]
fn an_append_writes_its_lowest_position_as_its_group() {
    let store = store();
    let event = Event::new("Grouped", b"g".to_vec()).expect("a valid event type");

    let sql = &store
        .insert_request(core::slice::from_ref(&event))
        .statements[0]
        .query;

    assert!(sql.contains("recorded_at, write_group) SELECT"), "{sql}");
    assert!(sql.contains(&format!(", {APPEND_GROUP} FROM")), "{sql}");
}

/// The group read is one read-only statement. It carries the frontier in the
/// scan that names the groups and in both arms that read their rows, scans
/// one row past the budget but names groups from the budget's rows only,
/// reads the rows by key, and orders them by group, never by position.
///
/// **Named wrong statement:** the one outer read
/// `write_group IN (…) OR (write_group IS NULL AND position IN (…))` this
/// adapter sent first, which Postgres runs as a filter over every row of the
/// table; the `OR` and join assertions fail on it.
#[test]
fn the_group_read_is_one_read_only_statement_ordered_by_group() {
    let request = store().groups_request(7, 4);

    assert!(request.read_only);
    let [statement] = request.statements.as_slice() else {
        panic!("one statement: {:?}", request.statements);
    };
    let sql = &statement.query;
    assert!(
        statement.params.is_empty(),
        "the bounds are interpolated integers"
    );
    assert_eq!(sql.matches(FRONTIER).count(), 3, "{sql}");
    assert!(
        sql.contains("ORDER BY position LIMIT 5)"),
        "the probe: {sql}"
    );
    assert!(sql.contains("ORDER BY p LIMIT 4)"), "the budget: {sql}");
    let event = store().config.qualified_event();
    assert!(
        sql.contains(&format!(
            "FROM ids JOIN {event} AS e ON e.write_group = ids.g"
        )) && sql.contains(&format!("FROM ids JOIN {event} AS e ON e.position = ids.g"))
            && sql.contains(" UNION ALL "),
        "{sql}"
    );
    assert!(!sql.contains(" OR "), "{sql}");
    assert!(sql.ends_with("ORDER BY sort_group, sort_position"), "{sql}");
}

/// A transport that answers every request with one body.
struct Answering(String);

impl SqlTransport for &Answering {
    type Error = std::io::Error;

    fn round_trip(
        &self,
        _request: SqlRequest,
    ) -> impl Future<Output = Result<HttpResponse, Self::Error>> {
        // Each answer owns its body, as a real response does.
        core::future::ready(Ok(HttpResponse::new(200, self.0.clone().into_bytes())))
    }

    fn reads_settled(&self) -> impl Future<Output = ()> {
        core::future::ready(())
    }
}

/// One row of the group read as the endpoint renders it.
fn row(position: u64, group: u64, guard: Option<&str>, scanned: u64) -> serde_json::Value {
    serde_json::json!({
        "position": position.to_string(),
        "event_type": "Grouped",
        "data": BASE64.encode([0xAB]),
        "metadata": null,
        "tags": [],
        "origin_store": BASE64.encode([0x11; 16]),
        "origin_position": position.to_string(),
        "recorded_at": "1",
        "write_group_first": group.to_string(),
        "group_guard": guard,
        "scanned": scanned.to_string(),
    })
}

/// An answer of `rows`, in the single-statement form.
fn answering(rows: &[serde_json::Value]) -> Answering {
    Answering(
        serde_json::json!({ "command": "SELECT", "rowCount": rows.len(), "fields": [], "rows": rows })
            .to_string(),
    )
}

/// Reads one page through the port, bound on the bare flavour.
async fn read<S: WriteGroupStore>(
    store: &S,
    max: usize,
) -> Result<happenstance_sync::GroupPage, S::Error> {
    store
        .read_groups(None, NonZeroUsize::new(max).expect("non-zero"))
        .await
}

/// Interleaved groups G = {5, 7} and H = {6, 8}, answered in group order,
/// read back as two whole groups, G's guard decoded from its first row; a
/// scan that found fewer rows than it asked for says nothing is waiting.
#[tokio::test]
async fn interleaved_groups_answered_in_group_order_read_back_whole() {
    let stored = BASE64.encode(encode_guard(&guard("Shipped")).expect("encodes"));
    let transport = answering(&[
        row(5, 5, Some(&stored), 4),
        row(7, 5, None, 4),
        row(6, 6, None, 4),
        row(8, 6, None, 4),
    ]);
    let store = NeonEventStore::new(&transport, NeonConfig::default());

    let page = read(&store, 4).await.expect("a well-formed answer");

    let shape: Vec<(u64, usize)> = page
        .groups
        .iter()
        .map(|group| (group.first().get(), group.events().len()))
        .collect();
    assert_eq!(shape, [(5, 2), (6, 2)]);
    let guards: Vec<Option<&AppendCondition>> = page.groups.iter().map(WriteGroup::guard).collect();
    assert_eq!(guards, [Some(&guard("Shipped")), None]);
    assert_eq!(
        page.waiting,
        Waiting::Nothing,
        "4 scanned of the 5 asked for"
    );
}

/// A scan that filled its probe says something may be waiting, even when
/// every group it named fitted.
#[tokio::test]
async fn a_scan_that_filled_its_probe_says_unknown() {
    let transport = answering(&[row(1, 1, None, 3), row(2, 2, None, 3)]);
    let store = NeonEventStore::new(&transport, NeonConfig::default());

    let page = read(&store, 2).await.expect("a well-formed answer");

    assert_eq!(page.groups.len(), 2);
    assert_eq!(
        page.waiting,
        Waiting::Unknown,
        "3 scanned of the 3 asked for"
    );
}

/// The same groups answered by position, as a read that forgot to order by
/// group would return them, are refused as `GroupOrder`, class `Stop`,
/// rather than paged with G torn in two.
#[tokio::test]
async fn groups_answered_by_position_are_refused() {
    use happenstance_sync::Classify as _;

    let transport = answering(&[
        row(5, 5, None, 4),
        row(6, 6, None, 4),
        row(7, 5, None, 4),
        row(8, 6, None, 4),
    ]);
    let store = NeonEventStore::new(&transport, NeonConfig::default());

    let refused = read(&store, 8).await;

    let Err(error @ NeonError::GroupOrder(_)) = refused else {
        panic!("expected GroupOrder, got {refused:?}");
    };
    assert_eq!(error.class(), happenstance_sync::FailureClass::Stop);
}

/// A guard column holding a format no build of the codec wrote stops the
/// read with the group it belongs to, class `Stop`, rather than forwarding
/// the group without its guard.
#[tokio::test]
async fn a_guard_the_codec_cannot_read_stops_the_group_read() {
    use happenstance_sync::Classify as _;

    let unreadable = BASE64.encode([9]);
    let transport = answering(&[row(3, 3, Some(&unreadable), 1)]);
    let store = NeonEventStore::new(&transport, NeonConfig::default());

    let refused = read(&store, 4).await;

    let Err(
        error @ NeonError::StoredGuard {
            first,
            source: GuardCodecError::UnknownFormat { found: 9, .. },
        },
    ) = refused
    else {
        panic!("expected StoredGuard, got {refused:?}");
    };
    assert_eq!(first.get(), 3);
    assert_eq!(error.class(), happenstance_sync::FailureClass::Stop);
}

/// A guard a group cannot be stored with is refused before anything is sent,
/// named by the group's index, class `Stop`.
#[test]
fn an_unstorable_guard_classifies_stop() {
    use happenstance_sync::Classify as _;

    let error: NeonError<std::io::Error> = NeonError::UnstorableGuard {
        group: 2,
        source: decode_guard(&[]).expect_err("no bytes are no guard"),
    };

    assert_eq!(error.class(), happenstance_sync::FailureClass::Stop);
}
