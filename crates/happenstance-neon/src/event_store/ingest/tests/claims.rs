//! SY-38 against the live endpoint: a claim on this store's own identity that
//! it did not hold when the call began refuses the whole call.
//!
//! The family's rule verifies the hidden-claim half only where the store puts
//! a compensation two positions past the head, and on this store a refused
//! call has consumed sequence values first, so the rule reports that half as
//! skipped. Here every compensation position is drawn after every foreign row
//! of the call has met the origin index, so a claim on a position a
//! compensation would take lands, and is refused as landed.
//! `a_claim_on_the_position_the_calls_own_compensation_would_take_is_refused`
//! verifies that, on an arrangement a twin store shows this allocation
//! produces.

use happenstance_core::{EventId, SequencePosition, StoreId};
use happenstance_sync::{
    Classify, FailureClass, IngestError, IngestGroup, Ingested, ReplicatedEvent,
};

use super::endpoint::{LiveTransport, live_store};
use super::{
    append, carrying, event, holds, id, ingest, log, own, peer, rows, send, shipped, watermark,
};
use crate::event_store::NeonEventStore;
use crate::transport::SqlStatement;

/// The position `count` past `position`.
fn past(position: SequencePosition, count: usize) -> SequencePosition {
    (0..count).fold(position, |position, _| {
        position
            .next()
            .expect("test positions are far from the top")
    })
}

/// The arrangement the unheld-claim tests start from.
///
/// One local event is held. Then a call of two groups, the second of which
/// carries, at index 1, an event claiming this store's own identity `ahead`
/// positions past the head the append returned. The claim is neither first in
/// its group nor in the first group, so a store that inspects only the head
/// of a batch, or commits group by group, fails the tests below.
struct Claimed {
    /// The identity the forged event claims.
    claim: EventId,
    /// The first group's foreign events, all new.
    first: [ReplicatedEvent; 1],
    /// The second group's: one new foreign event, then the claim.
    second: [ReplicatedEvent; 2],
}

impl Claimed {
    async fn on(store: &NeonEventStore<LiveTransport>, ahead: usize) -> Self {
        let held = append(store, &[event("Local", "a")]).await;
        let claim = EventId::new(own(store).await, past(held, ahead));
        let from = peer(0x4A);
        Self {
            claim,
            first: [shipped(id(from, 1), 1, event("Shipped", "f1"))],
            second: [
                shipped(id(from, 2), 2, event("Shipped", "f2")),
                shipped(claim, 3, event("Local", "forged")),
            ],
        }
    }

    fn groups(&self) -> [IngestGroup<'_>; 2] {
        [
            IngestGroup::new(&self.first, &[]),
            IngestGroup::new(&self.second, &[]),
        ]
    }

    /// Every event the call carries, the claim last.
    fn sent(&self) -> [EventId; 3] {
        [self.first[0].id, self.second[0].id, self.claim]
    }
}

/// Asserts `refused` is the unheld-identity refusal naming `claim`, class Stop.
fn assert_refused<E: Classify + core::fmt::Debug>(
    refused: Result<Ingested, IngestError<E>>,
    claim: EventId,
) {
    let refused = refused.expect_err("an unheld own identity refuses the ingest");
    assert!(
        matches!(refused, IngestError::UnheldOwnIdentity { id, .. } if id == claim),
        "expected UnheldOwnIdentity naming {claim}, got {refused:?}"
    );
    assert_eq!(refused.class(), FailureClass::Stop);
}

/// An own-identity event this store holds is an ordinary re-delivery: a skip,
/// decided by the insert. Rejects refusing every own identity.
#[tokio::test]
#[ignore = "needs a live Neon endpoint; set NEON_CONNECTION and run with `-- --ignored`"]
async fn an_echo_of_an_own_event_is_a_skip() {
    let store = live_store().await;
    let held = append(&store, &[event("Local", "a")]).await;
    let before = log(&store).await;

    let back_home = [shipped(
        EventId::new(own(&store).await, held),
        1,
        event("Local", "a"),
    )];
    let skipped = ingest(&store, &[IngestGroup::new(&back_home, &[])])
        .await
        .expect("an echo of a held event is not an error");
    assert_eq!((skipped.appended, skipped.skipped), (0, 1));
    assert_eq!(log(&store).await, before);
    assert_eq!(rows(&store).await, 1);
}

/// SY-38: the claim refuses the whole call, and nothing is written: not the
/// claim, not the new foreign event beside it, not the group before it. The
/// store still writes afterwards, and its watermark has not moved.
///
/// Rejects dropping the claim as a skip, checking only the head of the batch,
/// and committing group by group.
#[tokio::test]
#[ignore = "needs a live Neon endpoint; set NEON_CONNECTION and run with `-- --ignored`"]
async fn an_unheld_own_identity_stops_the_ingest_and_writes_nothing() {
    let store = live_store().await;
    let claimed = Claimed::on(&store, 3).await;
    let marks = watermark(&store).await;

    assert_refused(ingest(&store, &claimed.groups()).await, claimed.claim);

    for sent in claimed.sent() {
        assert!(
            !holds(&store, sent).await,
            "{sent} was written by a refused ingest"
        );
    }
    assert_eq!(rows(&store).await, 1);
    assert_eq!(watermark(&store).await, marks);
    let after = append(&store, &[event("Local", "b")]).await;
    assert!(
        holds(&store, EventId::new(own(&store).await, after)).await,
        "the store still writes"
    );
}

/// The whole call rolls back, so the compensation goes too: the refusing
/// group's, and the one for the group before it.
#[tokio::test]
#[ignore = "needs a live Neon endpoint; set NEON_CONNECTION and run with `-- --ignored`"]
async fn a_compensation_whose_group_carries_an_unheld_own_identity_is_not_written() {
    let store = live_store().await;
    let claimed = Claimed::on(&store, 3).await;
    let before = log(&store).await;
    let answers = [[event("Compensated", "one")], [event("Compensated", "two")]];

    let refused = ingest(
        &store,
        &[
            IngestGroup::new(&claimed.first, &answers[0]),
            IngestGroup::new(&claimed.second, &answers[1]),
        ],
    )
    .await;
    assert_refused(refused, claimed.claim);
    assert_eq!(log(&store).await, before);
    assert_eq!(rows(&store).await, 1, "a refused ingest left rows behind");
}

/// A claim on the position this call's own compensation would take is
/// refused, and as the claim, not as a unique violation.
///
/// Group 1 lands a foreign event and carries a compensation; group 2 carries
/// a foreign event and the claim. The call draws its three foreign positions
/// first and the compensation's after them, so the compensation would take
/// the position four past the head. A twin store shows that allocation is
/// real here: the same call with the claim replaced by a peer's event puts
/// its compensation exactly there.
///
/// Rejects a compensation not gated on the claims, under which the refusal
/// surfaces as `23505` on the origin index, and any reordering that writes a
/// compensation before the call's last foreign row, under which the claim is
/// skipped against it and two events share one identity.
#[tokio::test]
#[ignore = "needs a live Neon endpoint; set NEON_CONNECTION and run with `-- --ignored`"]
async fn a_claim_on_the_position_the_calls_own_compensation_would_take_is_refused() {
    let from = peer(0x5B);
    let first = [shipped(id(from, 1), 1, event("Shipped", "f1"))];
    let answer = [event("Compensated", "c1")];

    let twin = live_store().await;
    let twin_head = append(&twin, &[event("Local", "a")]).await;
    let stand_in = [
        shipped(id(from, 2), 2, event("Shipped", "f2")),
        shipped(id(peer(0x5C), 1), 3, event("Shipped", "stand-in")),
    ];
    ingest(
        &twin,
        &[
            IngestGroup::new(&first, &answer),
            IngestGroup::new(&stand_in, &[]),
        ],
    )
    .await
    .expect("three new foreign events and a compensation land");
    let compensated = log(&twin)
        .await
        .into_iter()
        .find(|held| held.event == answer[0])
        .expect("the compensation is held");
    assert_eq!(
        compensated.id,
        EventId::new(own(&twin).await, past(twin_head, 4)),
        "the compensation is drawn after all three foreign positions"
    );

    let store = live_store().await;
    let head = append(&store, &[event("Local", "a")]).await;
    let before = log(&store).await;
    let claim = EventId::new(own(&store).await, past(head, 4));
    let second = [
        shipped(id(from, 2), 2, event("Shipped", "f2")),
        shipped(claim, 3, event("Local", "forged")),
    ];

    let refused = ingest(
        &store,
        &[
            IngestGroup::new(&first, &answer),
            IngestGroup::new(&second, &[]),
        ],
    )
    .await;
    assert_refused(refused, claim);
    assert_eq!(log(&store).await, before);
    assert_eq!(rows(&store).await, 1);
}

/// No permanent wedge after a refused claim, and a refused claim leaves no row
/// behind to collide with.
///
/// The sequence never returns a value twice, so what this pins is that the
/// local append that reaches the claimed position lands, carrying this
/// store's identity. The claim is eight past the head and the refused call
/// consumed three, so the fifth append reaches it.
#[tokio::test]
#[ignore = "needs a live Neon endpoint; set NEON_CONNECTION and run with `-- --ignored`"]
async fn local_appends_are_not_wedged_after_a_refused_claim() {
    let store = live_store().await;
    let claimed = Claimed::on(&store, 8).await;
    assert_refused(ingest(&store, &claimed.groups()).await, claimed.claim);

    let mut assigned = Vec::new();
    for _ in 0..16 {
        let position = append(&store, &[event("Local", "next")]).await;
        assigned.push(position);
        if position >= claimed.claim.position() {
            break;
        }
    }
    assert!(
        assigned.contains(&claimed.claim.position()),
        "no append reached the claimed position, so this proves nothing: {assigned:?}"
    );
    assert!(holds(&store, claimed.claim).await);
}

/// The remedy SY-38 names converges, by the manual re-mint ADR-0098 says is
/// the only recovery here until #104 decides the procedure: after the
/// identity row is replaced and a fresh handle opened, the same groups land
/// whole, the claim as the foreign event it now is, and earlier local rows
/// keep the identity they were minted under.
#[tokio::test]
#[ignore = "needs a live Neon endpoint; set NEON_CONNECTION and run with `-- --ignored`"]
async fn a_claim_lands_as_foreign_after_a_manual_remint() {
    let store = live_store().await;
    let claimed = Claimed::on(&store, 3).await;
    assert_refused(ingest(&store, &claimed.groups()).await, claimed.claim);
    let old: StoreId = own(&store).await;

    let meta = store.config().qualified_meta();
    let reminted = send(
        &store,
        vec![SqlStatement::new(format!(
            "UPDATE {meta} SET v = uuid_send(gen_random_uuid()) WHERE k = 'store_id'"
        ))],
    )
    .await;
    assert!(reminted.is_success(), "the identity row is replaced");
    let reminted = NeonEventStore::new(
        // A cheap handle onto the same client, which the new store keeps.
        store.transport().clone(),
        store.config().clone(),
    );
    assert_ne!(own(&reminted).await, old);

    let landed = ingest(&reminted, &claimed.groups())
        .await
        .expect("after the re-mint the claim is a foreign event");
    assert_eq!((landed.appended, landed.skipped), (3, 0));
    let log = log(&reminted).await;
    assert_eq!(carrying(&log, claimed.claim).id.store(), old);
    let local = log
        .iter()
        .find(|held| held.event == event("Local", "a"))
        .expect("the arrangement's local event is held");
    assert_eq!(local.id.store(), old, "a re-mint restamped an earlier row");
}
