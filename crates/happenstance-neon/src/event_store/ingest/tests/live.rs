//! The port's behaviour against the live endpoint: the endpoint facts the
//! design leans on, then VT-5, SY-1, SY-2, SY-5, SY-11 and the watermark.
//!
//! Every test reads right after it ingests, as most of the event-store suite
//! does after it appends. Under the frontier that relies on a quiet endpoint
//! and `--test-threads=1`, which is how CI's `live-neon` job runs them.

use happenstance_core::{
    AppendCondition, AppendError, Event, EventId, EventStore, Query, QueryItem, StoreLimit, Tags,
};
use happenstance_sync::{Classify, FailureClass, IngestError, IngestGroup, Watermark};

use super::super::{UNHELD_GUC, UNHELD_SQLSTATE, WATERMARK_SQL, refusal_statement};
use super::endpoint::{LiveTransportError, live_store};
use super::{
    append, carrying, event, holds, id, ingest, log, own, peer, rows, select, send, shipped,
    watermark,
};
use crate::config::quote_ident;
use crate::error::NeonError;
use crate::event_store::{NeonEventStore, decode_body};
use crate::transport::{IsolationLevel, SqlRequest, SqlStatement, SqlTransport};

/// The level the design leans on is the level the endpoint runs a
/// two-statement batch at. Measured before only for `Serializable`.
///
/// Rejects an endpoint that ignores `RepeatableRead`, under which a claim on a
/// row committed during the call would be skipped as held.
#[tokio::test]
#[ignore = "needs a live Neon endpoint; set NEON_CONNECTION and run with `-- --ignored`"]
async fn the_endpoint_honours_repeatable_read_on_a_two_statement_batch() {
    let store = live_store().await;
    let level = || SqlStatement::new("SELECT current_setting('transaction_isolation') AS l");
    let response = store
        .transport()
        .round_trip(SqlRequest::batch(
            vec![level(), level()],
            IsolationLevel::RepeatableRead,
        ))
        .await
        .expect("the endpoint answers");
    let body =
        decode_body::<LiveTransportError>(&response, usize::MAX).expect("the batch succeeds");
    assert_eq!(body.result_sets().len(), 2);
    for set in body.result_sets() {
        assert_eq!(set.text(0, "l"), Some("repeatable read"));
    }
}

/// The endpoint renders the refusal with the code and the detail the decoder
/// reads, which `a_refusal_answer_names_the_claim_it_carries` scripts.
///
/// Rejects an endpoint that drops `DETAIL`, under which every refusal would be
/// reported as a bare SQL error.
#[tokio::test]
#[ignore = "needs a live Neon endpoint; set NEON_CONNECTION and run with `-- --ignored`"]
async fn the_endpoint_returns_the_refusals_sqlstate_and_detail() {
    let store = live_store().await;
    let response = store
        .transport()
        .round_trip(SqlRequest::batch(
            vec![
                SqlStatement::new(format!("SELECT set_config('{UNHELD_GUC}', '7', true) AS s")),
                refusal_statement(),
            ],
            IsolationLevel::RepeatableRead,
        ))
        .await
        .expect("the endpoint answers");
    match decode_body::<LiveTransportError>(&response, usize::MAX) {
        Err(NeonError::Sql(sql)) => {
            assert_eq!(sql.code.as_deref(), Some(UNHELD_SQLSTATE), "{sql:?}");
            assert_eq!(sql.detail.as_deref(), Some("7"), "{sql:?}");
        }
        other => panic!("expected the refusal as a SQL error, got {other:?}"),
    }

    // And a setting left empty raises nothing.
    let response = store
        .transport()
        .round_trip(SqlRequest::batch(
            vec![
                SqlStatement::new(format!("SELECT set_config('{UNHELD_GUC}', '', true) AS s")),
                refusal_statement(),
            ],
            IsolationLevel::RepeatableRead,
        ))
        .await
        .expect("the endpoint answers");
    assert!(
        response.is_success(),
        "{}",
        String::from_utf8_lossy(&response.body)
    );
}

/// VT-5 and VT-9: an ingested event keeps its origin's `EventId` and
/// `RecordedAt` exactly, one of them before 1970.
///
/// Rejects re-minting, and restamping with the arrival time.
#[tokio::test]
#[ignore = "needs a live Neon endpoint; set NEON_CONNECTION and run with `-- --ignored`"]
async fn ingest_preserves_foreign_event_id_and_recorded_at() {
    let store = live_store().await;
    let from = peer(0xA1);
    let batch = [
        shipped(id(from, 3), -86_400_000, event("Shipped", "a")),
        shipped(id(from, 7), 1_700_000_000_000, event("Shipped", "b")),
    ];

    let ingested = ingest(&store, &[IngestGroup::new(&batch, &[])])
        .await
        .expect("two new foreign events land");
    assert_eq!(ingested.appended, 2);

    let log = log(&store).await;
    for sent in &batch {
        let held = carrying(&log, sent.id);
        assert_eq!(held.id, sent.id, "the origin identity was re-minted");
        assert_eq!(
            held.recorded_at, sent.recorded_at,
            "the origin's recorded time was restamped"
        );
        assert_eq!(held.event, sent.event);
    }
}

/// SY-11: the same batch again is a skip, not an error, and writes no row; a
/// batch half held and half new lands the new half.
///
/// Rejects raising on the unique violation, and an `ON CONFLICT` with no
/// target or the wrong one, which the server refuses or which skips nothing.
#[tokio::test]
#[ignore = "needs a live Neon endpoint; set NEON_CONNECTION and run with `-- --ignored`"]
async fn redelivery_is_a_skip_not_an_error() {
    let store = live_store().await;
    let from = peer(0xB2);
    let first = [
        shipped(id(from, 1), 10, event("Shipped", "a")),
        shipped(id(from, 2), 20, event("Shipped", "b")),
    ];
    ingest(&store, &[IngestGroup::new(&first, &[])])
        .await
        .expect("the first delivery lands");
    let before = rows(&store).await;

    let again = ingest(&store, &[IngestGroup::new(&first, &[])])
        .await
        .expect("a re-delivery is not an error");
    assert_eq!((again.appended, again.skipped), (0, 2));
    assert_eq!(rows(&store).await, before, "a re-delivery wrote a row");

    let mixed = [
        first[1].clone(),
        shipped(id(from, 3), 30, event("Shipped", "c")),
    ];
    let partial = ingest(&store, &[IngestGroup::new(&mixed, &[])])
        .await
        .expect("a half-held batch is not an error");
    assert_eq!((partial.appended, partial.skipped), (1, 1));
    assert_eq!(rows(&store).await, before + 1);
}

/// The same event twice in one call lands once and counts once as a skip, as
/// `MemoryIngestStore` answers.
///
/// Rejects a statement that errors on a duplicate inside itself.
#[tokio::test]
#[ignore = "needs a live Neon endpoint; set NEON_CONNECTION and run with `-- --ignored`"]
async fn the_same_event_twice_in_one_call_lands_once() {
    let store = live_store().await;
    let twice = shipped(id(peer(0xB3), 5), 5, event("Shipped", "a"));
    let batch = [twice.clone(), twice];

    let ingested = ingest(&store, &[IngestGroup::new(&batch, &[])])
        .await
        .expect("a duplicate inside one call is a skip");
    assert_eq!((ingested.appended, ingested.skipped), (1, 1));
    assert_eq!(rows(&store).await, 1);
}

/// SY-5: ingested events land above every position assigned before the call,
/// whatever positions their origin gave them, and the next append lands above
/// them.
///
/// Compared against the position `append` returned, not against `head`, which
/// is a frontier here. Rejects writing at the origin's position.
#[tokio::test]
#[ignore = "needs a live Neon endpoint; set NEON_CONNECTION and run with `-- --ignored`"]
async fn ingested_events_land_above_every_position_assigned_before_the_call() {
    let store = live_store().await;
    let assigned = append(
        &store,
        &[
            event("Local", "a"),
            event("Local", "b"),
            event("Local", "c"),
        ],
    )
    .await;

    let from = peer(0xC3);
    let batch = [
        shipped(id(from, 1), 1, event("Shipped", "x")),
        shipped(id(from, 2), 2, event("Shipped", "y")),
    ];
    ingest(&store, &[IngestGroup::new(&batch, &[])])
        .await
        .expect("two new foreign events land");

    let log = log(&store).await;
    let landed: Vec<_> = batch
        .iter()
        .map(|sent| carrying(&log, sent.id).position)
        .collect();
    for position in &landed {
        assert!(
            *position > assigned,
            "ingested at {position}, below {assigned}"
        );
    }
    let next = append(&store, &[event("Local", "d")]).await;
    assert!(
        landed.iter().all(|position| next > *position),
        "{next} vs {landed:?}"
    );
}

/// SY-2 and SY-11: compensation lands with the losing event, after it, minted
/// here; a re-delivery writes no second compensation; and a compensation over
/// a ceiling writes neither half.
#[tokio::test]
#[ignore = "needs a live Neon endpoint; set NEON_CONNECTION and run with `-- --ignored`"]
async fn compensation_is_atomic_with_the_losing_event_and_skipped_on_redelivery() {
    let store = live_store().await;
    let own = own(&store).await;
    let from = peer(0xD4);
    let losing = [shipped(id(from, 5), 50, event("Reserved", "seat-1"))];
    let compensation = [event("ReservationCompensated", "seat-1")];

    let first = ingest(&store, &[IngestGroup::new(&losing, &compensation)])
        .await
        .expect("a new losing event and its compensation land");
    assert_eq!(
        (first.appended, first.skipped, first.compensated),
        (1, 0, 1)
    );

    let after = log(&store).await;
    assert_eq!(after.len(), 2);
    let lost = carrying(&after, losing[0].id);
    let answer = after
        .iter()
        .find(|held| held.event == compensation[0])
        .expect("the compensation is held");
    assert!(answer.position > lost.position);
    assert_eq!(
        answer.id,
        EventId::new(own, answer.position),
        "compensation is minted here, like any local append"
    );

    let again = ingest(&store, &[IngestGroup::new(&losing, &compensation)])
        .await
        .expect("a re-delivery is not an error");
    assert_eq!(
        (again.appended, again.skipped, again.compensated),
        (0, 1, 0)
    );
    assert_eq!(rows(&store).await, 2, "compensation written twice");

    let never_fits = [Event::new(
        "ReservationCompensated",
        vec![0_u8; NeonEventStore::<()>::MAX_EVENT_DATA_LEN + 1],
    )
    .expect("a valid event type")];
    let doomed = [shipped(id(from, 6), 60, event("Reserved", "seat-2"))];
    let refused = ingest(&store, &[IngestGroup::new(&doomed, &never_fits)]).await;
    assert!(
        matches!(
            refused,
            Err(IngestError::ExceedsStoreLimit {
                limit: StoreLimit::EventDataLen,
                ..
            })
        ),
        "{refused:?}"
    );
    assert!(!holds(&store, doomed[0].id).await);
}

/// SY-2 inside the transaction: the losing event is inserted, the compensation
/// insert then fails, and the losing event goes with it.
///
/// The fault is a `BEFORE INSERT` trigger installed in this store's schema,
/// because nothing a caller can put in an `Event` fails once the ceilings
/// pass. Rejects a write that commits its halves apart.
#[tokio::test]
#[ignore = "needs a live Neon endpoint; set NEON_CONNECTION and run with `-- --ignored`"]
async fn a_compensation_that_fails_in_the_transaction_takes_the_losing_event_with_it() {
    let store = live_store().await;
    let config = store.config();
    let schema = quote_ident(&config.schema);
    let installed = send(
        &store,
        vec![
            SqlStatement::new(format!(
                "CREATE FUNCTION {schema}.poison() RETURNS trigger LANGUAGE plpgsql AS \
                 $$ BEGIN RAISE EXCEPTION 'poisoned compensation'; END $$"
            )),
            SqlStatement::new(format!(
                "CREATE TRIGGER poison BEFORE INSERT ON {} FOR EACH ROW \
                 WHEN (NEW.event_type = 'Poisoned') EXECUTE FUNCTION {schema}.poison()",
                config.qualified_event()
            )),
        ],
    )
    .await;
    assert!(installed.is_success(), "the fault installs");

    let losing = [shipped(id(peer(0xE5), 1), 1, event("Reserved", "seat"))];
    let poisoned = [event("Poisoned", "seat")];
    let failed = ingest(&store, &[IngestGroup::new(&losing, &poisoned)])
        .await
        .expect_err("the poisoned compensation fails");

    assert!(
        matches!(&failed, IngestError::Store(NeonError::Sql(sql)) if sql.code.as_deref() == Some("P0001")),
        "expected the trigger's raise, got {failed:?}"
    );
    assert_eq!(failed.class(), FailureClass::Stop);
    assert!(
        !holds(&store, losing[0].id).await,
        "the losing event committed without its compensation"
    );
    assert_eq!(rows(&store).await, 0);
}

/// The largest origin position a `bigint` holds lands and reads back. With
/// `an_origin_position_past_bigint_is_refused_not_saturated` it brackets the
/// conversion; rejects an off-by-one bound.
#[tokio::test]
#[ignore = "needs a live Neon endpoint; set NEON_CONNECTION and run with `-- --ignored`"]
async fn the_largest_bigint_origin_position_lands() {
    let store = live_store().await;
    let largest = u64::try_from(i64::MAX).expect("i64::MAX is positive");
    let batch = [shipped(id(peer(0x07), largest), 1, event("Shipped", "a"))];

    let ingested = ingest(&store, &[IngestGroup::new(&batch, &[])])
        .await
        .expect("i64::MAX is a bigint");
    assert_eq!(ingested.appended, 1);
    assert_eq!(carrying(&log(&store).await, batch[0].id).id, batch[0].id);
}

/// `contains_event_id` answers for a foreign identity, and not for another
/// origin at the same position, nor for this store's identity at the row's
/// local position: a row carries one identity.
#[tokio::test]
#[ignore = "needs a live Neon endpoint; set NEON_CONNECTION and run with `-- --ignored`"]
async fn contains_event_id_answers_for_a_foreign_id() {
    let store = live_store().await;
    let sent = id(peer(0x17), 42);
    assert!(!holds(&store, sent).await);

    let batch = [shipped(sent, 1, event("Shipped", "a"))];
    ingest(&store, &[IngestGroup::new(&batch, &[])])
        .await
        .expect("a new foreign event lands");

    assert!(holds(&store, sent).await);
    assert!(!holds(&store, id(peer(0x18), 42)).await);
    let landed = carrying(&log(&store).await, sent).position;
    assert!(!holds(&store, EventId::new(own(&store).await, landed)).await);
}

/// SY-1: an ingest evaluates no condition, including one every append would
/// violate now, and a conditional append after it sees the ingested event as
/// the conflict.
#[tokio::test]
#[ignore = "needs a live Neon endpoint; set NEON_CONNECTION and run with `-- --ignored`"]
async fn ingest_ignores_append_conditions() {
    let store = live_store().await;
    let seat = Query::from_item(
        QueryItem::tagged(Tags::from_pairs([("k", "seat-9")]).expect("a valid tag"))
            .expect("a tagged item is a valid query item"),
    );
    let decided_at = append(&store, &[event("Reserved", "seat-9")]).await;

    let taken = AppendCondition::new(seat.clone());
    let violated = EventStore::append(&store, &[event("Reserved", "seat-9")], Some(&taken))
        .await
        .expect_err("the seat is taken");
    assert!(violated.is_condition_violated(), "{violated:?}");

    let rival = [shipped(id(peer(0x29), 3), 3, event("Reserved", "seat-9"))];
    let ingested = ingest(&store, &[IngestGroup::new(&rival, &[])])
        .await
        .expect("an ingest evaluates no condition");
    assert_eq!(ingested.appended, 1);

    let landed = carrying(&log(&store).await, rival[0].id).position;
    let since_decision = AppendCondition::new(seat).after_every_guard(decided_at);
    match EventStore::append(
        &store,
        &[event("Reserved", "seat-9")],
        Some(&since_decision),
    )
    .await
    {
        Err(AppendError::ConditionViolated(violated)) => {
            assert_eq!(violated.conflicting_position, Some(landed));
        }
        other => panic!("expected the ingested event as the conflict, got {other:?}"),
    }
}

/// The watermark is the highest origin position held per origin store, this
/// store's own included, and empty on a fresh store.
///
/// The oracle is the log read back and folded here, which shares nothing with
/// the `GROUP BY`, and each foreign mark is also held against the maximum
/// sent.
#[tokio::test]
#[ignore = "needs a live Neon endpoint; set NEON_CONNECTION and run with `-- --ignored`"]
async fn watermark_is_max_origin_position_per_origin() {
    let store = live_store().await;
    assert!(watermark(&store).await.is_empty());

    let (one, two) = (peer(0x31), peer(0x32));
    let batch = [
        shipped(id(one, 2), 2, event("Shipped", "a")),
        shipped(id(one, 9), 9, event("Shipped", "b")),
        shipped(id(one, 5), 5, event("Shipped", "c")),
        shipped(id(two, 4), 4, event("Shipped", "d")),
    ];
    ingest(
        &store,
        &[
            IngestGroup::new(&batch[..2], &[]),
            IngestGroup::new(&batch[2..], &[]),
        ],
    )
    .await
    .expect("four new foreign events land");
    append(&store, &[event("Local", "a"), event("Local", "b")]).await;

    let mut expected = Watermark::new();
    for held in log(&store).await {
        expected.advance(held.id.store(), held.id.position());
    }
    let marks = watermark(&store).await;
    assert_eq!(marks, expected);
    assert_eq!(marks.len(), 3);
    for from in [one, two] {
        let sent = batch
            .iter()
            .filter(|shipped| shipped.id.store() == from)
            .map(|shipped| shipped.id.position())
            .max();
        assert_eq!(marks.get(from), sent);
    }
}

/// The watermark's query can be served by the origin index with no sort.
///
/// The planner's switches make its choice deterministic on a tiny table, and
/// they ask the property: can the index serve this. Transaction-local, and in
/// the same request as the `EXPLAIN`, because there is no session to set them
/// on. Rejects dropping `origin_position IS NOT NULL`, which plans a
/// sequential scan and a sort, and grouping by an expression.
#[tokio::test]
#[ignore = "needs a live Neon endpoint; set NEON_CONNECTION and run with `-- --ignored`"]
async fn watermark_reads_the_origin_index_without_a_sort() {
    let store = live_store().await;
    let batch = [
        shipped(id(peer(0x33), 1), 1, event("Shipped", "a")),
        shipped(id(peer(0x34), 2), 2, event("Shipped", "b")),
    ];
    ingest(&store, &[IngestGroup::new(&batch, &[])])
        .await
        .expect("two new foreign events land");

    let watermark_sql = WATERMARK_SQL.replace("{event}", &store.config().qualified_event());
    let mut statements: Vec<SqlStatement> = [
        "enable_seqscan",
        "enable_bitmapscan",
        "enable_hashagg",
        "enable_sort",
    ]
    .into_iter()
    .map(|switch| SqlStatement::new(format!("SET LOCAL {switch} = off")))
    .collect();
    statements.push(SqlStatement::new(format!("EXPLAIN {watermark_sql}")));
    let response = send(&store, statements).await;
    let body =
        decode_body::<LiveTransportError>(&response, usize::MAX).expect("the explain succeeds");
    let plan: Vec<&str> = body
        .result_sets()
        .last()
        .expect("the explain answers")
        .rows
        .iter()
        .filter_map(|row| row.get("QUERY PLAN")?.as_str())
        .collect();
    let plan = plan.join("\n");
    assert!(plan.contains("event_origin_idx"), "{plan}");
    assert!(!plan.contains("Sort"), "{plan}");
}

/// A cross-check that needs no port at all: what the store's raw table holds
/// for one ingested event is the origin the peer sent, in the columns the
/// origin index covers.
#[tokio::test]
#[ignore = "needs a live Neon endpoint; set NEON_CONNECTION and run with `-- --ignored`"]
async fn an_ingested_row_stores_the_origin_it_was_sent() {
    let store = live_store().await;
    let sent = id(peer(0x35), 77);
    ingest(
        &store,
        &[IngestGroup::new(
            &[shipped(sent, -1, event("Shipped", "a"))],
            &[],
        )],
    )
    .await
    .expect("a new foreign event lands");
    let table = store.config().qualified_event();
    let stored = select(
        &store,
        format!(
            "SELECT encode(origin_store, 'base64') AS s, origin_position::text AS p, \
             recorded_at::text AS r FROM {table}"
        ),
    )
    .await;
    assert_eq!(stored.rows.len(), 1);
    assert_eq!(stored.text(0, "p"), Some("77"));
    assert_eq!(stored.text(0, "r"), Some("-1"));
    assert_eq!(
        stored.text(0, "s"),
        Some("NTU1NTU1NTU1NTU1NTU1NQ=="),
        "`[0x35; 16]`, base64"
    );
}
