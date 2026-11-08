//! `NeonSyncPeer`'s request shapes, with no endpoint (#131).
//!
//! Here, beside the ingest tests, because the scripted transport is theirs.
//! What these pin is the peer's whole claim: **one request per pull and one
//! per push**, the page, `waiting` and the responder decided inside the
//! pull's request (SY-37, ADR-0104), and the write, its confirmation and the
//! responder inside the push's. CF-33 leaves request counting to the adapter:
//! the suite may not count operations, and nothing stops an adapter pinning
//! what it sends. The live half is `peer_family.rs`.

use base64::Engine as _;
use happenstance_sync::{EventGroup, PeerLimits, PushBatch, StoreResume, SyncPeer, Waiting};

use super::{
    Classify as _, FailureClass, HttpResponse, IngestError, IngestGroup, IsolationLevel, OWN,
    Scripted, SequencePosition, event, id, peer, shipped,
};
use crate::error::NeonError;
use crate::event_store::{BASE64, NeonPeerError, NeonSyncPeer};

/// One result set as the endpoint renders it in the batch form.
fn set(rows: &[serde_json::Value]) -> serde_json::Value {
    serde_json::json!({
        "command": "SELECT", "rowCount": rows.len(), "fields": [], "rows": rows,
        "rowAsArray": false,
    })
}

/// The identity read's result set, naming `store`.
fn identity(store: happenstance_core::StoreId) -> serde_json::Value {
    set(&[serde_json::json!({ "v": BASE64.encode(store.to_bytes()) })])
}

/// One row of the group read, as `write_group/tests.rs` renders it.
fn row(position: u64, group: u64, scanned: u64) -> serde_json::Value {
    serde_json::json!({
        "position": position.to_string(),
        "event_type": "Grouped",
        "data": BASE64.encode([0xAB, position.to_le_bytes()[0]]),
        "metadata": null,
        "tags": [],
        "origin_store": BASE64.encode([0x11; 16]),
        "origin_position": position.to_string(),
        "recorded_at": "1",
        "write_group_first": group.to_string(),
        "group_guard": null,
        "scanned": scanned.to_string(),
    })
}

impl Scripted {
    /// Queues one 200 answer of `sets`, in the batch form.
    fn answer_batch(&self, sets: &[serde_json::Value]) {
        let body = serde_json::json!({ "results": sets }).to_string();
        self.answers
            .borrow_mut()
            .push_back(HttpResponse::new(200, body.into_bytes()));
    }

    /// The one request this transport was sent, taken out of the record.
    fn only_request(&self) -> crate::transport::SqlRequest {
        let mut sent = self.sent.take();
        assert_eq!(sent.len(), 1, "exactly one request: {sent:?}");
        sent.pop().expect("one request, by the assertion above")
    }
}

/// Each statement's text and parameters, for comparing two requests.
fn shape(statements: &[crate::transport::SqlStatement]) -> Vec<(&str, &[serde_json::Value])> {
    statements
        .iter()
        .map(|statement| (statement.query.as_str(), statement.params.as_slice()))
        .collect()
}

fn limits_of(events: usize) -> PeerLimits {
    let mut limits = PeerLimits::minimum();
    limits.max_batch_events = events;
    limits
}

/// SY-37 and ADR-0104 in one round trip: the page and the identity, two
/// statements of one read-only `REPEATABLE READ` batch, so the page, its
/// `waiting` and the responder are one snapshot. A scan that found fewer
/// rows than its probe reached the end of the log.
///
/// **Named wrong peer:** `StoreSyncPeer` over this store, which reads the
/// page and the identity in two round trips; the count fails on it.
#[tokio::test]
async fn a_pull_is_one_read_only_request_of_the_page_and_the_identity() {
    let transport = Scripted::default();
    transport.answer_batch(&[
        set(&[row(5, 5, 3), row(7, 5, 3), row(6, 6, 3)]),
        identity(OWN),
    ]);
    let peer = NeonSyncPeer::new(transport.store());

    let pulled = peer.pull(None).await.expect("a well-formed answer");

    let request = transport.only_request();
    assert!(request.read_only);
    assert_eq!(request.isolation, IsolationLevel::RepeatableRead);
    let [page, who] = request.statements.as_slice() else {
        panic!("the page, then the identity: {:?}", request.statements);
    };
    let store = transport.store();
    // One past what a page carries, as `StoreSyncPeer` reads (D-2).
    let probe = PeerLimits::minimum().max_batch_events + 1;
    let max = i64::try_from(probe).expect("a small number");
    assert_eq!(page.query, store.groups_statement(0, max).query);
    assert_eq!(who.query, store.identity_statement().query);

    let shape: Vec<(u64, usize)> = pulled
        .batch
        .groups
        .iter()
        .map(|group| (group.events[0].id.position().get(), group.events.len()))
        .collect();
    assert_eq!(shape, [(5, 2), (6, 1)]);
    assert_eq!(pulled.waiting, Waiting::Nothing, "3 scanned of 130 asked");
    assert_eq!(
        pulled.resume,
        StoreResume::after(SequencePosition::new(6).expect("non-zero"))
    );
    assert_eq!(pulled.responder, Some(OWN));
}

/// A page the declared budget ends stops before the group it cannot carry,
/// resumes after the last group it did, and says the log goes on.
///
/// Rejects a page that splits the third group, or that says `Nothing`
/// because it ran out of page rather than out of log.
#[tokio::test]
async fn a_page_the_budget_ends_resumes_after_its_last_group_and_says_unknown() {
    let transport = Scripted::default();
    transport.answer_batch(&[
        set(&[row(1, 1, 4), row(2, 2, 4), row(3, 3, 4)]),
        identity(OWN),
    ]);
    let peer = NeonSyncPeer::with_limits(transport.store(), limits_of(2));

    let pulled = peer.pull(None).await.expect("a well-formed answer");

    assert_eq!(pulled.batch.events().count(), 2);
    assert_eq!(pulled.waiting, Waiting::Unknown);
    assert_eq!(
        pulled.resume,
        StoreResume::after(SequencePosition::new(2).expect("non-zero"))
    );
}

/// A pull from a token asks for the groups above it.
#[tokio::test]
async fn a_pull_from_a_token_reads_above_it() {
    let transport = Scripted::default();
    transport.answer_batch(&[set(&[]), identity(OWN)]);
    let peer = NeonSyncPeer::new(transport.store());
    let from = StoreResume::after(SequencePosition::new(41).expect("non-zero"));

    let pulled = peer.pull(Some(&from)).await.expect("an empty answer");

    let request = transport.only_request();
    let max = i64::try_from(PeerLimits::minimum().max_batch_events + 1).expect("small");
    assert_eq!(
        request.statements[0].query,
        transport.store().groups_statement(41, max).query
    );
    assert_eq!(pulled.batch, PushBatch::empty());
    assert_eq!(pulled.resume, from, "nothing forwarded, so the token stays");
    assert_eq!(pulled.waiting, Waiting::Nothing);
}

/// The write, its confirmation and the responder in one round trip: the
/// ingest's three statements, then the watermark and the identity, in one
/// `REPEATABLE READ` batch, so the ack describes the transaction that wrote
/// the push.
///
/// **Named wrong peer:** `StoreSyncPeer` over this store, which sends the
/// ingest, the watermark and the identity as three round trips.
#[tokio::test]
async fn a_push_is_one_request_that_writes_confirms_and_names_the_store() {
    let origin = peer(0x44);
    let transport = Scripted::default();
    transport.answer_batch(&[
        set(&[serde_json::json!({
            "appended": "2", "skipped": "1", "compensated": "0", "unheld": "",
        })]),
        serde_json::json!({ "command": "DO", "rowCount": null, "fields": [], "rows": [] }),
        serde_json::json!({ "command": "UPDATE", "rowCount": 1, "fields": [], "rows": [] }),
        set(&[serde_json::json!({ "s": BASE64.encode(origin.to_bytes()), "p": "7" })]),
        identity(OWN),
    ]);
    let peer = NeonSyncPeer::new(transport.store());
    let a = [
        shipped(id(origin, 3), 1, event("Shipped", "a")),
        shipped(id(origin, 5), 2, event("Shipped", "a")),
    ];
    let b = [shipped(id(origin, 7), 3, event("Shipped", "b"))];
    let batch = PushBatch::new(vec![
        EventGroup::new(None, a.to_vec()),
        EventGroup::new(None, b.to_vec()),
    ]);

    let ack = peer.push(&batch).await.expect("a well-formed answer");

    let request = transport.only_request();
    assert!(!request.read_only);
    assert_eq!(request.isolation, IsolationLevel::RepeatableRead);
    let store = transport.store();
    let groups = [IngestGroup::new(&a, &[]), IngestGroup::new(&b, &[])];
    let mut expected = store
        .ingest_request::<std::io::Error>(&groups)
        .expect("no guard to encode")
        .statements;
    expected.push(store.watermark_statement());
    expected.push(store.identity_statement());
    assert_eq!(shape(&request.statements), shape(&expected));

    assert_eq!((ack.appended, ack.skipped), (2, 1));
    assert_eq!(ack.confirmed.get(origin), SequencePosition::new(7));
    assert_eq!(ack.responder, Some(OWN));
}

/// An empty push writes nothing and still confirms and names the store:
/// one read-only request of the watermark and the identity.
#[tokio::test]
async fn an_empty_push_is_one_read_only_request_for_the_watermark_and_the_identity() {
    let transport = Scripted::default();
    transport.answer_batch(&[set(&[]), identity(OWN)]);
    let peer = NeonSyncPeer::new(transport.store());

    let ack = peer
        .push(&PushBatch::empty())
        .await
        .expect("an empty answer");

    let request = transport.only_request();
    assert!(request.read_only);
    let store = transport.store();
    let queries: Vec<&str> = request
        .statements
        .iter()
        .map(|statement| statement.query.as_str())
        .collect();
    assert_eq!(
        queries,
        [
            store.watermark_statement().query.as_str(),
            store.identity_statement().query.as_str(),
        ]
    );
    assert_eq!((ack.appended, ack.skipped), (0, 0));
    assert_eq!(ack.responder, Some(OWN));
}

/// SY-18 and SY-36: a push the declared limits refuse is refused before the
/// wire, each limit with its class: an event no split can carry is `Park`,
/// a batch a split can cure is `Split`.
#[tokio::test]
async fn a_push_over_the_declared_limits_sends_nothing() {
    let origin = peer(0x45);
    let limits = PeerLimits::minimum();
    let small = |position| shipped(id(origin, position), 1, event("Shipped", "a"));
    let sized = |position, len: usize| {
        shipped(
            id(origin, position),
            1,
            happenstance_core::Event::new("Shipped", vec![0_u8; len]).expect("a valid type"),
        )
    };
    let one_group = |events: Vec<_>| PushBatch::new(vec![EventGroup::new(None, events)]);
    let half = limits.max_batch_bytes / 2 + 1;

    let transport = Scripted::default();
    let peer = NeonSyncPeer::with_limits(transport.store(), limits_of(2));
    let too_many = peer
        .push(&one_group(vec![small(1), small(2), small(3)]))
        .await
        .expect_err("three events over a limit of two");
    assert!(
        matches!(
            too_many,
            NeonPeerError::TooManyEvents {
                actual: 3,
                limit: 2,
                ..
            }
        ),
        "{too_many:?}"
    );
    assert_eq!(too_many.class(), FailureClass::Split);

    let peer = NeonSyncPeer::new(transport.store());
    let too_large = peer
        .push(&one_group(vec![sized(1, limits.max_event_bytes + 1)]))
        .await
        .expect_err("one byte over max_event_bytes");
    assert!(
        matches!(too_large, NeonPeerError::EventTooLarge { .. }),
        "{too_large:?}"
    );
    assert_eq!(too_large.class(), FailureClass::Park);

    let too_heavy = peer
        .push(&one_group(vec![sized(1, half), sized(2, half)]))
        .await
        .expect_err("two halves and two bytes over max_batch_bytes");
    assert!(
        matches!(too_heavy, NeonPeerError::BatchTooLarge { .. }),
        "{too_heavy:?}"
    );
    assert_eq!(too_heavy.class(), FailureClass::Split);

    assert_eq!(transport.round_trips.get(), 0);
}

/// The endpoint's refusal of the write is the ingest's refusal, mapped as
/// the port maps it: SY-38's `HS038` names the claim, as `Stop`.
#[tokio::test]
async fn a_refused_push_is_the_ingest_refusal() {
    let transport = Scripted::answering(
        400,
        br#"{"message":"m","code":"HS038","detail":"1","hint":null,"severity":"ERROR"}"#,
    );
    let peer = NeonSyncPeer::new(transport.store());
    let claim = shipped(id(OWN, 9), 1, event("Local", "forged"));
    let claimed = claim.id;

    let refused = peer
        .push(&PushBatch::new(vec![EventGroup::new(None, vec![claim])]))
        .await
        .expect_err("the endpoint refused the write");

    assert!(
        matches!(
            &refused,
            NeonPeerError::Ingest(IngestError::UnheldOwnIdentity { id, .. }) if *id == claimed
        ),
        "{refused:?}"
    );
    assert_eq!(refused.class(), FailureClass::Stop);
    assert_eq!(transport.round_trips.get(), 1);
}

/// The five result sets a landed one-group push is answered with: counts,
/// the refusal's `DO`, the stamp's `UPDATE`, the watermark, the identity.
fn landed_push(counts: serde_json::Value) -> Vec<serde_json::Value> {
    vec![
        set(&[counts]),
        serde_json::json!({ "command": "DO", "rowCount": null, "fields": [], "rows": [] }),
        serde_json::json!({ "command": "UPDATE", "rowCount": 1, "fields": [], "rows": [] }),
        set(&[]),
        identity(OWN),
    ]
}

/// The counts row of one landed event.
fn one_landed() -> serde_json::Value {
    serde_json::json!({ "appended": "1", "skipped": "0", "compensated": "0", "unheld": "" })
}

/// A one-group push from `peer(0x47)`.
fn one_group() -> PushBatch {
    PushBatch::new(vec![EventGroup::new(
        None,
        vec![shipped(id(peer(0x47), 1), 1, event("Shipped", "a"))],
    )])
}

/// A push answered with one result set too few, or one too many, committed
/// and cannot be read: `Answer`, naming both counts, class `Stop`.
///
/// Rejects lenient indexing, which reads the watermark and the identity from
/// whatever sets sit at or past the write's, and the old
/// `MissingColumn { "results" }`, which named neither count.
#[tokio::test]
async fn a_push_answered_with_the_wrong_number_of_result_sets_is_answer() {
    let mut too_few = landed_push(one_landed());
    too_few.remove(3);
    let mut too_many = landed_push(one_landed());
    too_many.push(identity(OWN));

    for (sets, actual) in [(too_few, 4), (too_many, 6)] {
        let transport = Scripted::default();
        transport.answer_batch(&sets);
        let peer = NeonSyncPeer::new(transport.store());

        let refused = peer
            .push(&one_group())
            .await
            .expect_err("an answer shaped unlike its request");

        assert!(
            matches!(
                &refused,
                NeonPeerError::Answer(NeonError::ResultSets { expected: 5, actual: a }) if *a == actual
            ),
            "{refused:?}"
        );
        assert_eq!(refused.class(), FailureClass::Stop);
    }
}

/// A pull answered with a set too many is refused the same way, as `Read`.
#[tokio::test]
async fn a_pull_answered_with_the_wrong_number_of_result_sets_is_read() {
    let transport = Scripted::default();
    transport.answer_batch(&[set(&[]), identity(OWN), identity(OWN)]);
    let peer = NeonSyncPeer::new(transport.store());

    let refused = peer.pull(None).await.expect_err("three sets for two reads");

    assert!(
        matches!(
            refused,
            NeonPeerError::Read(NeonError::ResultSets {
                expected: 2,
                actual: 3
            })
        ),
        "{refused:?}"
    );
}

/// A counts row without `appended`, in an answer that committed, is
/// `Answer`, not the ingest's `Store`: the write landed and re-delivery is a
/// skip, which `Ingest(Store)`'s unknown outcome does not promise.
#[tokio::test]
async fn a_push_whose_counts_lack_a_column_is_answer_not_ingest() {
    let transport = Scripted::default();
    transport.answer_batch(&landed_push(serde_json::json!({
        "skipped": "0", "compensated": "0", "unheld": "",
    })));
    let peer = NeonSyncPeer::new(transport.store());

    let refused = peer
        .push(&one_group())
        .await
        .expect_err("a counts row without `appended`");

    assert!(
        matches!(
            refused,
            NeonPeerError::Answer(NeonError::MissingColumn { column: "appended" })
        ),
        "{refused:?}"
    );
}

/// SY-36: a store arm states the store error's class, not one fixed by the
/// arm. A pull that got no answer is `Transient`, one the endpoint refused as
/// too large is `Split`, and each equals its `NeonError`'s class. Mirrors
/// `store_peer.rs`'s `an_unread_identity_states_the_store_errors_class`.
///
/// Rejects a `Read` arm fixed at one class.
#[tokio::test]
async fn a_failed_pull_states_the_store_errors_class() {
    let unanswered = Scripted::default();
    let too_large = Scripted::answering(413, b"request too large");

    for (transport, class) in [
        (&unanswered, FailureClass::Transient),
        (&too_large, FailureClass::Split),
    ] {
        let refused = NeonSyncPeer::new(transport.store())
            .pull(None)
            .await
            .expect_err("the pull failed");

        let NeonPeerError::Read(store) = &refused else {
            panic!("a failed pull is `Read`: {refused:?}");
        };
        assert_eq!(store.class(), class);
        assert_eq!(refused.class(), class);
    }
}
