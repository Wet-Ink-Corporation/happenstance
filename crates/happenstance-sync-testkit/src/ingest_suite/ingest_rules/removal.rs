//! SY-40's arrangements: a store that had events removed outside the port,
//! then offered them again.

use alloc::format;

use happenstance_core::{EventId, EventStore, SequencePosition, StoreId};
use happenstance_sync::{IngestError, IngestGroup, IngestStore, ReplicatedEvent};
use happenstance_testkit::Removed;

use super::{
    COMPENSATED, LOCAL, append, ensure_counts, ensure_foreign, ensure_unchanged, head, held_once,
    holds, ingest, observe, opaque, read_all, shipped,
};
use crate::IngestFixture;
use crate::violation::{Violation, ensure};

/// SY-40, the clause every check here reports under.
pub(super) const SY_40: &str = "SY-40";

/// SY-40's origin, the next free one after SY-14's.
const REMOVAL_ORIGIN: StoreId = StoreId::from_bytes([0x7E; 16]);

/// When the rule's foreign events were recorded at their origin.
const RECORDED: i64 = 1_800_000_000_000;

/// Removes `positions` through the fixture, and refuses a fixture that
/// declared the capability and removed nothing.
async fn remove<F: IngestFixture>(
    fixture: &F,
    positions: &[SequencePosition],
) -> Result<(), Violation> {
    let removed = fixture.remove_outside_port(positions).await;
    ensure(removed == Removed::Deleted, SY_40, || {
        format!(
            "the fixture declares REMOVAL supported and its `remove_outside_port` answered \
             {removed:?}; a fixture that grants the capability overrides it, or the rule \
             shows nothing about a removed event"
        )
    })
}

/// The store does not hold `id`, which was removed.
async fn ensure_unheld<S: EventStore>(store: &S, id: EventId, what: &str) -> Result<(), Violation> {
    ensure(!holds(store, id, SY_40).await?, SY_40, || {
        format!(
            "{what}, and the store holds {id}, which was removed outside the port; a store \
             records the identity of every event removed from it and skips it on re-delivery, \
             never appending it again"
        )
    })
}

/// Arrangement (a): three foreign events, one group each; the middle one is
/// removed; the same three groups again, through the handle held before the
/// removal and through one connected after it, land nothing.
pub(super) async fn a_removed_foreign_event<F: IngestFixture>(
    fixture: &F,
) -> Result<(), Violation> {
    let held_before = fixture.connect().await;
    ensure_foreign(&held_before, &[REMOVAL_ORIGIN], SY_40).await?;
    let sent = [
        shipped(REMOVAL_ORIGIN, 1, RECORDED, b"sy40-a-1", SY_40)?,
        shipped(REMOVAL_ORIGIN, 2, RECORDED + 1, b"sy40-a-2", SY_40)?,
        shipped(REMOVAL_ORIGIN, 3, RECORDED + 2, b"sy40-a-3", SY_40)?,
    ];
    let groups = sent
        .each_ref()
        .map(|event| IngestGroup::new(core::slice::from_ref(event), &[]));
    let landed = ingest(&held_before, &groups, SY_40).await?;
    ensure_counts(landed, (3, 0, 0), "three new foreign events", SY_40)?;
    let [first, middle, last] = &sent;
    let at = held_once(&read_all(&held_before, SY_40).await?, &middle.event, SY_40)?.position;

    remove(fixture, &[at]).await?;

    ensure_unheld(&held_before, middle.id, "the fixture removed it").await?;
    // By their bytes, never their identities, so a store that re-mints on
    // ingest fails VT-5's rule and not this one.
    let log = read_all(&held_before, SY_40).await?;
    held_once(&log, &first.event, SY_40)?;
    held_once(&log, &last.event, SY_40)?;
    ensure(
        log.iter().all(|row| row.event != middle.event),
        SY_40,
        || {
            format!(
                "the fixture answered Deleted for {at} and the event it held there is still held"
            )
        },
    )?;
    let before = observe(&held_before, SY_40).await?;
    let what = "the same three groups again, one of them removed";
    let again = ingest(&held_before, &groups, SY_40).await?;
    ensure_counts(again, (0, 3, 0), what, SY_40)?;
    ensure_unchanged(&held_before, &before, what, SY_40).await?;
    ensure_unheld(&held_before, middle.id, what).await?;

    let fresh = fixture.connect().await;
    let what = "the same three groups again, through a handle connected after the removal";
    let again = ingest(&fresh, &groups, SY_40).await?;
    ensure_counts(again, (0, 3, 0), what, SY_40)?;
    ensure_unchanged(&fresh, &before, what, SY_40).await?;
    ensure_unheld(&fresh, middle.id, what).await
}

/// Arrangements (b) and (c), on one fixture: one of the store's own events
/// is removed and echoed back beside a new foreign one, which lands while the
/// echo is skipped; then a claim on an identity the store never minted is
/// still refused whole (SY-38).
pub(super) async fn a_removed_own_event<F: IngestFixture>(fixture: &F) -> Result<(), Violation> {
    let store = fixture.connect().await;
    let own = ensure_foreign(&store, &[REMOVAL_ORIGIN], SY_40).await?;
    let mine = opaque(LOCAL, b"sy40-b-local-1", SY_40)?;
    append(&store, core::slice::from_ref(&mine), SY_40).await?;
    append(&store, &[opaque(LOCAL, b"sy40-b-local-2", SY_40)?], SY_40).await?;
    // Owned, because the log it was found in is dropped here and the echo is
    // built from it after the removal: one row, its payload a `Bytes` handle.
    let minted = held_once(&read_all(&store, SY_40).await?, &mine, SY_40)?.clone();
    ensure(minted.id.store() == own, SY_40, || {
        format!(
            "a local append was minted as {}, not under the store's own identity {own}",
            minted.id
        )
    })?;

    remove(fixture, &[minted.position]).await?;

    ensure_unheld(&store, minted.id, "the fixture removed it").await?;
    // The echo owns its event, as a pulled one does; `minted` is still read
    // below for its identity.
    let offered = [
        ReplicatedEvent::new(minted.id, minted.recorded_at, minted.event.clone()),
        shipped(REMOVAL_ORIGIN, 11, RECORDED + 11, b"sy40-b-foreign", SY_40)?,
    ];
    let echoed = store
        .ingest(&[IngestGroup::new(&offered, &[])])
        .await
        .map_err(|error| {
            Violation::new(
                SY_40,
                format!(
                    "the store refused an echo of {}, an event it minted and then had removed \
                     outside the port: {error}; a removed event is skipped as held, never \
                     refused as an unheld own identity",
                    minted.id
                ),
            )
        })?;
    let what = "an echo of a removed own event beside a new foreign one";
    ensure_counts(echoed, (1, 1, 0), what, SY_40)?;
    ensure_unheld(&store, minted.id, what).await?;
    held_once(&read_all(&store, SY_40).await?, &offered[1].event, SY_40)?;

    a_claim_never_held_is_refused(&store, own).await
}

/// Arrangement (c): after a removal, a claim three positions past the head,
/// which the store never minted, refuses the whole call and writes nothing.
async fn a_claim_never_held_is_refused<S: EventStore + IngestStore>(
    store: &S,
    own: StoreId,
) -> Result<(), Violation> {
    let head = head(store, SY_40)
        .await?
        .ok_or_else(|| Violation::new(SY_40, "the store holds nothing after an ingest landed"))?;
    let past = head
        .next()
        .and_then(SequencePosition::next)
        .and_then(SequencePosition::next)
        .ok_or_else(|| Violation::new(SY_40, "the store's head has no position past it"))?;
    let claim = EventId::new(own, past);
    // The claim heads the call and a foreign event follows it, so the shapes
    // SY-38's rule owns (a check of the head only, a group-by-group commit)
    // are not what this arrangement reports.
    let call = [
        ReplicatedEvent::new(
            claim,
            happenstance_core::RecordedAt::from_millis(RECORDED),
            opaque(LOCAL, b"sy40-c-claim", SY_40)?,
        ),
        shipped(REMOVAL_ORIGIN, 12, RECORDED + 12, b"sy40-c-1", SY_40)?,
    ];
    let before = observe(store, SY_40).await?;

    let outcome = store.ingest(&[IngestGroup::new(&call, &[])]).await;

    let named = match &outcome {
        Err(IngestError::UnheldOwnIdentity { id, .. }) => Some(*id),
        _ => None,
    };
    ensure(named == Some(claim), SY_40, || {
        format!(
            "after a removal, a claim on {claim}, three positions past the head and never minted \
             there, was answered {outcome:?}; the removal record holds only identities the store \
             held, so this is still SY-38's refusal of the whole call"
        )
    })?;
    ensure_unchanged(
        store,
        &before,
        "an ingest refused for a claim never held",
        SY_40,
    )
    .await
}

/// Arrangement (d): two foreign events, one group each; the first is
/// removed; one group re-delivering both, carrying a compensation, writes
/// nothing, its compensation included (SY-11).
pub(super) async fn a_removed_group_writes_no_compensation<F: IngestFixture>(
    fixture: &F,
) -> Result<(), Violation> {
    let store = fixture.connect().await;
    ensure_foreign(&store, &[REMOVAL_ORIGIN], SY_40).await?;
    let sent = [
        shipped(REMOVAL_ORIGIN, 21, RECORDED + 21, b"sy40-d-1", SY_40)?,
        shipped(REMOVAL_ORIGIN, 22, RECORDED + 22, b"sy40-d-2", SY_40)?,
    ];
    let groups = sent
        .each_ref()
        .map(|event| IngestGroup::new(core::slice::from_ref(event), &[]));
    let landed = ingest(&store, &groups, SY_40).await?;
    ensure_counts(landed, (2, 0, 0), "two new foreign events", SY_40)?;
    let [removed, _] = &sent;
    let at = held_once(&read_all(&store, SY_40).await?, &removed.event, SY_40)?.position;
    remove(fixture, &[at]).await?;
    ensure_unheld(&store, removed.id, "the fixture removed it").await?;
    let answer = [opaque(COMPENSATED, b"sy40-d-compensation", SY_40)?];
    let before = observe(&store, SY_40).await?;

    let what = "one group of a removed event and a held one, carrying a compensation";
    let again = ingest(&store, &[IngestGroup::new(&sent, &answer)], SY_40).await?;

    ensure_counts(again, (0, 2, 0), what, SY_40)?;
    ensure_unchanged(&store, &before, what, SY_40).await
}
