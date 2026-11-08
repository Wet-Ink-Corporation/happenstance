//! SY-7's arrangement: two stores that each take the same conflicting pair,
//! and the runner the rule plays above them, which holds the assignment.
//!
//! # Why the rule plays the runner
//!
//! SY-7 makes adjudication a role, and a role is runner configuration, not a
//! property of the port. No runner in `happenstance-sync` authors a
//! compensation yet, so the rule plays that runner, as the model family plays
//! the runner that drives its peers. What is left to the store is the one part
//! of authorship it controls: the identity a compensation is minted under,
//! which is how a log records who wrote it.
//!
//! # Why the two stores never meet in the rule
//!
//! Forwarding the adjudicator's reversal to the other store would hold that
//! store to keeping a foreign identity, which is VT-5's rule's subject: a
//! store that re-mints on ingest would show the reversal as its own and fail
//! here for VT-5's reason. The meeting belongs to the falsification test, in
//! `tests.rs` beside this file, which runs on the reference store.

use alloc::format;
use alloc::vec::Vec;
use core::slice;

use happenstance_core::{
    Event, EventId, EventStore, EventType, Query, QueryItem, ReadOptions, RecordedAt,
    SequencePosition, SequencedEvent, StoreId, Tag, Tags, collect,
};
use happenstance_sync::{IngestGroup, IngestStore, Ingested, ReplicatedEvent};

use super::{ensure_counts, ensure_foreign, ingest, opaque, read_all};
use crate::IngestFixture;
use crate::violation::{Violation, ensure};

/// SY-7, the clause every arrangement here reports under.
pub(super) const SY_7: &str = "SY-7";

/// The two origins whose claims conflict, the next free ones after VT-36's.
const ORIGIN_A: StoreId = StoreId::from_bytes([0x7A; 16]);
const ORIGIN_B: StoreId = StoreId::from_bytes([0x7B; 16]);

const SEAT_CLAIMED: EventType = EventType::from_static("SeatClaimed");
const CLAIM_REVERSED: EventType = EventType::from_static("ClaimReversed");

/// The fact family the conflict belongs to: the tag key `seat` (SY-35).
const SEAT: &str = "seat:s7";

/// Who the runner lets author a compensation for the seat family.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Assignment {
    /// The first side alone: what SY-7 requires.
    Adjudicator,
    /// Both sides: the configuration SY-7 forbids, which only its
    /// falsification test runs.
    #[cfg(test)]
    EverySide,
}

/// One store, as the runner above it sees it.
#[derive(Debug)]
pub(super) struct Side<S> {
    pub(super) store: S,
    pub(super) id: StoreId,
    /// Whether the assignment names this side the family's adjudicator.
    pub(super) adjudicates: bool,
    /// What this side was asked to write in answer to the conflict.
    pub(super) reversal: Event,
}

/// Two sides that each took the same conflicting pair, in opposite orders.
#[derive(Debug)]
pub(super) struct Conflict<S> {
    /// The assigned adjudicator, then the side the assignment did not name
    /// (or, under [`Assignment::EverySide`], named as well).
    pub(super) sides: [Side<S>; 2],
}

/// Opens two stores and hands each the conflicting pair: the first takes A's
/// claim then B's, the second B's then A's, so the claim that arrives second,
/// and loses, differs between them.
///
/// # Errors
///
/// A [`Violation`] of SY-7 if the fixture gives both stores one identity, if
/// either cannot say who it is, or if an ingest is refused or miscounted.
pub(super) async fn take_the_conflicting_pair<F: IngestFixture>(
    open: &impl AsyncFn() -> F,
    assignment: Assignment,
) -> Result<Conflict<F::Store>, Violation> {
    let claims = [claim(ORIGIN_A)?, claim(ORIGIN_B)?];
    let guard = seat_guard()?;
    let adjudicator = side(open, true, b"reversed-by-the-adjudicator").await?;
    let other = side(
        open,
        assignment != Assignment::Adjudicator,
        b"reversed-by-the-other",
    )
    .await?;
    ensure(adjudicator.id != other.id, SY_7, || {
        format!(
            "two stores the fixture opened both have the incarnation {}; a fixture must give \
             each store a fresh one",
            adjudicator.id
        )
    })?;

    let [a, b] = &claims;
    for (side, order) in [(&adjudicator, [a, b]), (&other, [b, a])] {
        let [first, second] = order;
        let took = deliver(side, first, &guard).await?;
        ensure_counts(
            took,
            (1, 0, 0),
            "the first claim, with nothing to conflict",
            SY_7,
        )?;
        let took = deliver(side, second, &guard).await?;
        let compensated = usize::from(side.adjudicates);
        let what = if side.adjudicates {
            "the conflicting claim at an adjudicator, with its reversal"
        } else {
            "the conflicting claim at a side that does not adjudicate"
        };
        ensure_counts(took, (1, 0, compensated), what, SY_7)?;
    }
    Ok(Conflict {
        sides: [adjudicator, other],
    })
}

/// Each side's log holds exactly the reversals its role allows, each filed
/// under that side's own identity.
///
/// # Errors
///
/// A [`Violation`] of SY-7 for a log holding a reversal its side was not
/// assigned to write, a reversal missing from an adjudicator's log, or one
/// filed under another store's identity.
pub(super) async fn judge<S: EventStore + IngestStore>(
    conflict: &Conflict<S>,
) -> Result<(), Violation> {
    for side in &conflict.sides {
        let log = read_all(&side.store, SY_7).await?;
        let reversals = reversals(&log);
        let expected = usize::from(side.adjudicates);
        ensure(reversals.len() == expected, SY_7, || {
            format!(
                "the store {} holds {} reversals of the seat family's conflict, and the \
                 assignment lets it write {expected}",
                side.id,
                reversals.len()
            )
        })?;
        for reversal in reversals {
            ensure(reversal.id.store() == side.id, SY_7, || {
                format!(
                    "the adjudicator {} wrote a reversal, and its log files it as {}: the log \
                     says a store the assignment did not name authored a compensation",
                    side.id, reversal.id
                )
            })?;
        }
    }
    Ok(())
}

/// The reversals in `log`, found by type, never by decoding (SY-35).
pub(super) fn reversals(log: &[SequencedEvent]) -> Vec<&SequencedEvent> {
    log.iter()
        .filter(|row| *row.event_type() == CLAIM_REVERSED)
        .collect()
}

/// Opens one store and states its role.
async fn side<F: IngestFixture>(
    open: &impl AsyncFn() -> F,
    adjudicates: bool,
    reversal: &[u8],
) -> Result<Side<F::Store>, Violation> {
    let store = open().await.connect().await;
    let id = ensure_foreign(&store, &[ORIGIN_A, ORIGIN_B], SY_7).await?;
    let reversal = opaque(CLAIM_REVERSED, reversal, SY_7)?.with_tags(seat());
    Ok(Side {
        store,
        id,
        adjudicates,
        reversal,
    })
}

/// The runner's step for one pulled claim: read the guard as evidence (SY-6)
/// against what the side holds, and answer a conflict with a reversal only
/// where the assignment says this side adjudicates. The store evaluates
/// nothing (SY-1).
async fn deliver<S: EventStore + IngestStore>(
    side: &Side<S>,
    claim: &ReplicatedEvent,
    guard: &Query,
) -> Result<Ingested, Violation> {
    let held = collect(side.store.read(guard, ReadOptions::new()))
        .await
        .map_err(|error| {
            Violation::new(
                SY_7,
                format!("the store refused a read of the guard's query: {error}"),
            )
        })?;
    let compensation = if !held.is_empty() && side.adjudicates {
        slice::from_ref(&side.reversal)
    } else {
        &[]
    };
    let group = IngestGroup::new(slice::from_ref(claim), compensation);
    ingest(&side.store, &[group], SY_7).await
}

/// `origin`'s claim on the seat, the first event in its log, with bytes of its
/// own so a store that dedupes on content still takes both.
fn claim(origin: StoreId) -> Result<ReplicatedEvent, Violation> {
    let event = opaque(SEAT_CLAIMED, origin.to_string().as_bytes(), SY_7)?.with_tags(seat());
    Ok(ReplicatedEvent::new(
        EventId::new(origin, SequencePosition::FIRST),
        RecordedAt::from_millis(1_800_000_000_000),
        event,
    ))
}

/// The query both claims were decided under: no claim on this seat yet.
fn seat_guard() -> Result<Query, Violation> {
    let item = QueryItem::new([SEAT_CLAIMED], seat()).map_err(|error| {
        Violation::new(SY_7, format!("the rule's own query is invalid: {error}"))
    })?;
    Ok(Query::from_item(item))
}

fn seat() -> Tags {
    [Tag::from_static(SEAT)].into_iter().collect()
}

#[cfg(test)]
mod tests;
