//! SY-38 on the Durable Object: a foreign event claiming this store's own
//! identity at a position it does not hold refuses the whole call, and the
//! write decides it, with no lookup ahead of it.
//!
//! The arrangement is part D's: one local append, then two groups, the claim
//! behind the head of the second, at a position computed from what `append`
//! returned.

use happenstance_core::{EventId, EventStore, RecordedAt, SequencePosition, StoreId};
use happenstance_sync::{
    Classify, FailureClass, IngestError, IngestGroup, IngestStore, Ingested, ReplicatedEvent,
};
use wasm_bindgen_test::wasm_bindgen_test;

use super::support::{Opened, connect, local, log, minted, open, raw};
use crate::event_store::ingest::CloudflareIngestStore;
use crate::event_store::{CloudflareEventStore, CloudflareEventStoreError};

/// The peer whose events sit around each claim.
const PEER: StoreId = StoreId::from_bytes([0x65; 16]);

/// `head`, `count` positions on.
fn past(head: SequencePosition, count: u64) -> SequencePosition {
    SequencePosition::new(head.get() + count).expect("a position past the head")
}

/// An event claiming `id`, carrying `data`.
fn claim(id: EventId, data: &[u8]) -> ReplicatedEvent {
    ReplicatedEvent::new(id, RecordedAt::from_millis(0), local("Local", data, &[]))
}

/// Appends one local event and returns the position it was given.
async fn append_one(store: &CloudflareIngestStore, data: &[u8]) -> SequencePosition {
    store
        .append(&[local("Local", data, &[])], None)
        .await
        .expect("the append lands")
}

/// Everything a refused call must leave as it was.
async fn observed(
    opened: &Opened,
) -> (
    Vec<happenstance_core::SequencedEvent>,
    Option<SequencePosition>,
    happenstance_sync::Watermark,
    super::support::Raw,
) {
    let store = &opened.store;
    (
        log(store).await,
        store.head().await.expect("a head"),
        store.watermark().await.expect("a watermark"),
        raw(&opened.sql),
    )
}

/// The refusal of `claimed`, `Stop`.
fn assert_refused(
    outcome: Result<Ingested, IngestError<CloudflareEventStoreError>>,
    claimed: EventId,
) {
    let error = match outcome {
        Ok(ingested) => panic!("the claim on {claimed} was accepted as {ingested:?}"),
        Err(error) => error,
    };
    assert!(
        matches!(error, IngestError::UnheldOwnIdentity { id, .. } if id == claimed),
        "{error:?}"
    );
    assert_eq!(error.class(), FailureClass::Stop);
}

/// An echo of an event this store holds is a skip, and nothing changes.
///
/// Rejects refusing every own identity.
#[wasm_bindgen_test]
async fn an_echo_of_an_own_event_is_a_skip() {
    let opened = open();
    append_one(&opened.store, b"held").await;
    let before = observed(&opened).await;
    let held = &before.0[0];
    let echo = [ReplicatedEvent::new(
        held.id,
        held.recorded_at,
        held.event.clone(),
    )];

    let echoed = opened
        .store
        .ingest(&[IngestGroup::new(&echo, &[])])
        .await
        .expect("an echo is not an error");

    assert_eq!((echoed.appended, echoed.skipped), (0, 1));
    let after = observed(&opened).await;
    assert_eq!(
        (&after.0, after.1, &after.2),
        (&before.0, before.1, &before.2),
        "the log, head and watermark are unchanged"
    );
    assert_eq!(
        (after.3.events, after.3.tags, after.3.unstamped),
        (before.3.events, before.3.tags, before.3.unstamped),
        "and no row was left behind; a committed skip spends a position, which is a \
         permitted gap"
    );
}

/// A claim three positions past the head refuses the call, naming it, with
/// nothing written, the counter included, so the next append lands densely.
///
/// Rejects dropping the claim, checking only the batch's head, and a commit
/// per group.
#[wasm_bindgen_test]
async fn an_unheld_own_identity_stops_the_ingest_and_writes_nothing() {
    let opened = open();
    let own = opened.store.store_id().await.expect("the identity reads");
    let head = append_one(&opened.store, b"local").await;
    let claimed = EventId::new(own, past(head, 3));
    let first = [minted(PEER, 1, 1, b"c2-1", &[])];
    let second = [
        minted(PEER, 2, 2, b"c2-2", &[]),
        claim(claimed, b"c2-claim"),
    ];
    let before = observed(&opened).await;

    let refused = opened
        .store
        .ingest(&[
            IngestGroup::new(&first, &[]),
            IngestGroup::new(&second, &[]),
        ])
        .await;

    assert_refused(refused, claimed);
    assert_eq!(observed(&opened).await, before, "nothing was written");
    for id in [first[0].id, second[0].id, claimed] {
        assert!(!opened.store.contains_event_id(id).await.expect("reads"));
    }
    let next = append_one(&opened.store, b"after").await;
    assert_eq!(next, past(head, 1), "the refused call spent no position");
}

/// A group carrying an unheld claim writes no compensation, and neither does
/// the group before it: the call is refused whole.
#[wasm_bindgen_test]
async fn a_compensation_whose_group_carries_an_unheld_own_identity_is_not_written() {
    let opened = open();
    let own = opened.store.store_id().await.expect("the identity reads");
    let head = append_one(&opened.store, b"local").await;
    let claimed = EventId::new(own, past(head, 5));
    let first = [minted(PEER, 3, 3, b"c3-1", &[])];
    let second = [
        minted(PEER, 4, 4, b"c3-2", &[]),
        claim(claimed, b"c3-claim"),
    ];
    let answer = [local("Compensated", b"c3-answer", &[])];
    let before = observed(&opened).await;

    let refused = opened
        .store
        .ingest(&[
            IngestGroup::new(&first, &answer),
            IngestGroup::new(&second, &answer),
        ])
        .await;

    assert_refused(refused, claimed);
    assert_eq!(
        observed(&opened).await,
        before,
        "no compensation was written"
    );
}

/// The groups of C4's arrangement: a group the call's compensation answers,
/// and a group carrying `claimed`.
fn hidden(
    claimed: EventId,
) -> (
    [ReplicatedEvent; 1],
    [happenstance_core::Event; 1],
    [ReplicatedEvent; 2],
) {
    (
        [minted(PEER, 11, 11, b"c4-1", &[])],
        [local("Compensated", b"c4-answer", &[])],
        [
            minted(PEER, 12, 12, b"c4-2", &[]),
            claim(claimed, b"c4-claim"),
        ],
    )
}

/// A claim on the identity the call's own compensation takes is refused:
/// the stamp is skipped, but the claimed position is above the head read as
/// the call's first read, so the row it met is this call's.
///
/// A twin store proves the arrangement for this allocator first: one append,
/// then the first group alone, whose compensation lands two past the head.
///
/// Rejects refusing only the claims that land, and a head read after the
/// first write.
#[wasm_bindgen_test]
async fn a_claim_the_calls_own_compensation_would_hide_is_refused() {
    let twin = open();
    let twin_own = twin.store.store_id().await.expect("the identity reads");
    let twin_head = append_one(&twin.store, b"local").await;
    let (first, answer, _) = hidden(EventId::new(twin_own, past(twin_head, 2)));
    twin.store
        .ingest(&[IngestGroup::new(&first, &answer)])
        .await
        .expect("the twin's group lands");
    let compensation = log(&twin.store).await.pop().expect("the compensation");
    assert_eq!(
        compensation.id,
        EventId::new(twin_own, past(twin_head, 2)),
        "the arrangement holds for this allocator"
    );

    let opened = open();
    let own = opened.store.store_id().await.expect("the identity reads");
    let head = append_one(&opened.store, b"local").await;
    let claimed = EventId::new(own, past(head, 2));
    let (first, answer, second) = hidden(claimed);
    let before = observed(&opened).await;

    let refused = opened
        .store
        .ingest(&[
            IngestGroup::new(&first, &answer),
            IngestGroup::new(&second, &[]),
        ])
        .await;

    assert_refused(refused, claimed);
    assert_eq!(observed(&opened).await, before, "nothing was written");
}

/// A refused claim does not wedge local appends: the position it claimed is
/// handed out to a local append in due course, and is then held.
#[wasm_bindgen_test]
async fn local_appends_are_not_wedged_after_a_refused_claim() {
    let opened = open();
    let own = opened.store.store_id().await.expect("the identity reads");
    let head = append_one(&opened.store, b"local").await;
    let claimed = EventId::new(own, past(head, 8));
    let refused = opened
        .store
        .ingest(&[IngestGroup::new(&[claim(claimed, b"c5-claim")], &[])])
        .await;
    assert_refused(refused, claimed);

    let mut reached = None;
    for n in 0..16 {
        let position = append_one(&opened.store, format!("c5-{n}").as_bytes()).await;
        if position >= claimed.position() {
            reached = Some(position);
            break;
        }
    }

    assert_eq!(reached, Some(claimed.position()));
    assert!(
        opened
            .store
            .contains_event_id(claimed)
            .await
            .expect("reads")
    );
}

/// After the store is re-minted, the same groups land as foreign: the claim
/// names the retired identity, which is now another store's. Earlier local
/// rows keep the identity they were stamped with.
#[wasm_bindgen_test]
async fn a_claim_lands_as_foreign_after_the_store_is_reminted() {
    let opened = open();
    let retired = opened.store.store_id().await.expect("the identity reads");
    let head = append_one(&opened.store, b"local").await;
    let claimed = EventId::new(retired, past(head, 3));
    let first = [minted(PEER, 1, 1, b"c6-1", &[])];
    let second = [
        minted(PEER, 2, 2, b"c6-2", &[]),
        claim(claimed, b"c6-claim"),
    ];

    opened
        .store
        .store
        .remint_identity()
        .expect("the re-mint lands");
    let landed = opened
        .store
        .ingest(&[
            IngestGroup::new(&first, &[]),
            IngestGroup::new(&second, &[]),
        ])
        .await
        .expect("a claim on a retired identity is foreign");

    assert_eq!((landed.appended, landed.skipped), (3, 0));
    assert!(
        opened
            .store
            .contains_event_id(claimed)
            .await
            .expect("reads")
    );
    assert!(
        opened
            .store
            .contains_event_id(EventId::new(retired, head))
            .await
            .expect("reads"),
        "the earlier local row keeps the identity it was stamped with"
    );
}

/// The claim is judged by the identity current when the call runs, read
/// inside it: a re-mint through another handle reaches this one.
///
/// Rejects an identity cached at construction.
#[wasm_bindgen_test]
async fn the_claim_is_judged_by_the_identity_current_when_the_call_runs() {
    let opened = open();
    let old = opened.store.store_id().await.expect("the identity reads");
    let head = append_one(&opened.store, b"local").await;
    let other = connect(&opened.host, CloudflareEventStore::new(opened.sql.clone()));
    let new = other.store.remint_identity().expect("the re-mint lands");

    let on_old = opened
        .store
        .ingest(&[IngestGroup::new(
            &[claim(EventId::new(old, past(head, 3)), b"old")],
            &[],
        )])
        .await
        .expect("a claim on the old identity is foreign now");
    let on_new = EventId::new(new, past(head, 3));
    let refused = opened
        .store
        .ingest(&[IngestGroup::new(&[claim(on_new, b"new")], &[])])
        .await;

    assert_eq!((on_old.appended, on_old.skipped), (1, 0));
    assert_refused(refused, on_new);
}
