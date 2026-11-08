//! SY-10's arrangement: one edge between two stores, a merge rule for each of
//! its directions, and the runner the rule plays over it.
//!
//! # Why the rule plays the runner
//!
//! SY-7's reason, one clause on. A merge rule is runner configuration, and the
//! shipped runner has no adjudication hook yet (#136 left it planned), so the
//! rule plays a runner that holds one [`MergeRule`] per direction. What is left
//! to the store is the part it controls: it writes what the runner hands it
//! and nothing of its own. A store that judges on ingest carries one merge rule
//! into every direction of every edge it sits on, and the runner cannot then
//! permit a different one.
//!
//! # Which claims travel with a guard
//!
//! The ones a real runner would send one with. A store records no guard for a
//! local append, because its condition is position-relative (SY-6), so a claim
//! an end appended itself travels unguarded, pushed or pulled. A claim an end
//! ingested keeps the guard it arrived under (SY-30): here, the other spoke's
//! claim, which reached the hub with its origin's position-free guard and is
//! pulled on to the spoke with it. The hub's rule never reads a guard; it
//! judges by fact family (SY-35), as `happenstance-sync`'s
//! `tests/runner/directions.rs` hub does.
//!
//! # The two topologies are two configurations, not two runners
//!
//! [`HUB_AND_SPOKE`] and [`PEER_TO_PEER`] are values of one type, played by the
//! same steps over the same store type. Hub-ness is a property of an edge's
//! directions (SY-9), never of the store or of the runner's code.

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use core::slice;

use happenstance_core::{
    AppendCondition, Event, EventId, EventStore, EventType, Query, QueryItem, RecordedAt,
    SequencePosition, SequencedEvent, StoreId, Tag, Tags,
};
use happenstance_sync::{IngestGroup, IngestStore, Ingested, ReplicatedEvent};

use super::{ensure_counts, ensure_foreign, holds, ingest, opaque, read_all};
use crate::IngestFixture;
use crate::violation::{Violation, ensure};

/// SY-10, the clause every arrangement here reports under.
pub(super) const SY_10: &str = "SY-10";

/// The spoke whose claim reached the hub first, as the hub sees it: an origin,
/// not a store, because the hub sees every spoke and the spoke sees only
/// itself. The next free origin after SY-7's.
const OTHER_SPOKE: StoreId = StoreId::from_bytes([0x7C; 16]);

const SEAT_CLAIMED: EventType = EventType::from_static("SeatClaimed");
const CLAIM_REVERSED: EventType = EventType::from_static("ClaimReversed");

/// The fact family the conflict belongs to: the tag key `seat` (SY-35).
const SEAT: &str = "seat:s10";

/// One event by type and bytes: how this module compares facts.
type Fact = (String, Vec<u8>);

/// What one direction of an edge does with a claim it carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum MergeRule {
    /// Judge the claim by its fact family against what the receiving store
    /// holds, and hand it a reversal where another claim on the seat is
    /// already there: the hub's rule for what spokes push (SY-7).
    Adjudicate,
    /// Take the claim as sent, judged by nobody: a spoke's rule for what the
    /// hub sends, and both directions of an edge whose family has no
    /// adjudicator on it.
    Accept,
}

/// One edge's merge rules, one per direction. The near end is the one that
/// pushes first: the spoke, or the first peer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct Directions {
    /// What the far end does with what the near end pushes.
    pub(super) push: MergeRule,
    /// What the near end does with what it pulls from the far end.
    pub(super) pull: MergeRule,
}

/// The hub adjudicates what the spoke pushes; the spoke accepts what the hub
/// sends.
pub(super) const HUB_AND_SPOKE: Directions = Directions {
    push: MergeRule::Adjudicate,
    pull: MergeRule::Accept,
};

/// Neither peer adjudicates the family on this edge, so both directions
/// accept and the conflict waits for the family's adjudicator (SY-7).
pub(super) const PEER_TO_PEER: Directions = Directions {
    push: MergeRule::Accept,
    pull: MergeRule::Accept,
};

/// One end of the edge, as the runner above it sees it.
#[derive(Debug)]
pub(super) struct End<S> {
    pub(super) store: S,
    pub(super) id: StoreId,
    /// The reversal this end writes if a direction into it adjudicates.
    pub(super) reversal: Event,
    /// This end's own claim, once it has appended one.
    pub(super) claim: Option<EventId>,
}

/// The two ends of one edge, near then far.
#[derive(Debug)]
pub(super) struct Edge<S> {
    pub(super) near: End<S>,
    pub(super) far: End<S>,
}

/// Whether the far end takes a push or declines it (SY-33).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Answer {
    Refused,
    Taken,
}

/// The hub-and-spoke walk, under `directions`. The hub holds another spoke's
/// claim on the seat, ingested under its origin's guard. The spoke appends
/// its own under the guard "no claim on this seat". Its push is refused, and
/// neither log may change (SY-34). It pulls the other spoke's claim, guard and
/// all, then re-pushes and is taken, and pulls whatever the hub answered with.
///
/// Each step's counts are checked against what the runner handed the
/// receiving store: one claim, and a reversal only where that direction
/// adjudicated and found the seat already claimed.
///
/// # Errors
///
/// A [`Violation`] of SY-10 if the fixture gives both stores one identity, if
/// either cannot say who it is, if an ingest or an append is refused or
/// miscounted, or if the refused push changed either log.
pub(super) async fn hub_and_spoke<F: IngestFixture>(
    open: &impl AsyncFn() -> F,
    directions: Directions,
) -> Result<Edge<F::Store>, Violation> {
    let guard = seat_guard()?;
    let mut edge = open_edge(open, b"reversed-by-the-spoke", b"reversed-by-the-hub").await?;
    let first = ReplicatedEvent::new(
        EventId::new(OTHER_SPOKE, SequencePosition::FIRST),
        RecordedAt::from_millis(1_800_000_000_000),
        claim(b"claimed-by-the-other-spoke")?,
    );
    carry(
        MergeRule::Adjudicate,
        &edge.far,
        &first,
        Some(&guard),
        "the first claim to the hub",
    )
    .await?;

    let own = own_claim(&mut edge.near, b"claimed-by-the-spoke", &guard).await?;
    push(directions.push, &edge, &own, Answer::Refused).await?;
    pull(directions.pull, &edge, &guard).await?;
    push(directions.push, &edge, &own, Answer::Taken).await?;
    pull(directions.pull, &edge, &guard).await?;
    Ok(edge)
}

/// The peer-to-peer walk, under `directions`: each peer appends its own claim
/// under the guard, the first pushes its claim to the second, and pulls the
/// second's back. Both claims are local appends, so both travel unguarded.
///
/// # Errors
///
/// As [`hub_and_spoke`].
pub(super) async fn peer_to_peer<F: IngestFixture>(
    open: &impl AsyncFn() -> F,
    directions: Directions,
) -> Result<Edge<F::Store>, Violation> {
    let guard = seat_guard()?;
    let mut edge = open_edge(open, b"reversed-by-the-first", b"reversed-by-the-second").await?;
    let near = own_claim(&mut edge.near, b"claimed-by-the-first", &guard).await?;
    own_claim(&mut edge.far, b"claimed-by-the-second", &guard).await?;
    push(directions.push, &edge, &near, Answer::Taken).await?;
    pull(directions.pull, &edge, &guard).await?;
    Ok(edge)
}

/// Both ends hold the same facts, `expected` by type and bytes; each still
/// holds its own claim under its own identity; and nothing else is held.
///
/// # Errors
///
/// A [`Violation`] of SY-10 naming the end whose log differs, or whose own
/// claim is gone.
pub(super) async fn judge<S: EventStore + IngestStore>(
    edge: &Edge<S>,
    expected: &[(EventType, &[u8])],
) -> Result<(), Violation> {
    let mut expected: Vec<Fact> = expected
        .iter()
        .map(|(event_type, data)| (String::from(event_type.as_str()), data.to_vec()))
        .collect();
    expected.sort();
    for (end, name) in [(&edge.near, "near"), (&edge.far, "far")] {
        let held = facts(&read_all(&end.store, SY_10).await?);
        ensure(held == expected, SY_10, || {
            format!(
                "the {name} end {} holds {held:?} by type and bytes; under this edge's merge \
                 rules it should hold {expected:?}",
                end.id
            )
        })?;
        if let Some(claim) = end.claim {
            ensure(holds(&end.store, claim, SY_10).await?, SY_10, || {
                format!(
                    "the {name} end {} no longer holds its own claim {claim}: a store must not \
                     give up its own fact because a peer declined it or disagreed",
                    end.id
                )
            })?;
        }
    }
    Ok(())
}

/// The facts the hub-and-spoke walk leaves on both ends under
/// [`HUB_AND_SPOKE`]: both claims, and the hub's reversal.
pub(super) fn hub_and_spoke_facts() -> [(EventType, &'static [u8]); 3] {
    [
        (SEAT_CLAIMED, b"claimed-by-the-other-spoke"),
        (SEAT_CLAIMED, b"claimed-by-the-spoke"),
        (CLAIM_REVERSED, b"reversed-by-the-hub"),
    ]
}

/// The facts the peer-to-peer walk leaves on both ends under
/// [`PEER_TO_PEER`]: both claims, reversed by nobody.
pub(super) fn peer_to_peer_facts() -> [(EventType, &'static [u8]); 2] {
    [
        (SEAT_CLAIMED, b"claimed-by-the-first"),
        (SEAT_CLAIMED, b"claimed-by-the-second"),
    ]
}

/// The reversals in `log`, found by type, never by decoding (SY-35).
#[cfg(test)]
pub(super) fn reversals(log: &[SequencedEvent]) -> Vec<&SequencedEvent> {
    log.iter()
        .filter(|row| *row.event_type() == CLAIM_REVERSED)
        .collect()
}

/// The claims in `log`, found by type.
#[cfg(test)]
pub(super) fn claims(log: &[SequencedEvent]) -> Vec<&SequencedEvent> {
    log.iter()
        .filter(|row| *row.event_type() == SEAT_CLAIMED)
        .collect()
}

/// Opens the edge's two stores and checks they are two.
async fn open_edge<F: IngestFixture>(
    open: &impl AsyncFn() -> F,
    near_reversal: &[u8],
    far_reversal: &[u8],
) -> Result<Edge<F::Store>, Violation> {
    let near = end(open, near_reversal).await?;
    let far = end(open, far_reversal).await?;
    ensure(near.id != far.id, SY_10, || {
        format!(
            "two stores the fixture opened both have the incarnation {}; a fixture must give \
             each store a fresh one",
            near.id
        )
    })?;
    Ok(Edge { near, far })
}

/// Opens one store, asking it who it is before anything else.
async fn end<F: IngestFixture>(
    open: &impl AsyncFn() -> F,
    reversal: &[u8],
) -> Result<End<F::Store>, Violation> {
    let store = open().await.connect().await;
    let id = ensure_foreign(&store, &[OTHER_SPOKE], SY_10).await?;
    let reversal = opaque(CLAIM_REVERSED, reversal, SY_10)?.with_tags(seat());
    Ok(End {
        store,
        id,
        reversal,
        claim: None,
    })
}

/// `end` appends its own claim, decided under `guard` against its own log,
/// and the claim is read back as it would travel: its identity and recorded
/// time as the store assigned them, and no guard.
async fn own_claim<S: EventStore + IngestStore>(
    end: &mut End<S>,
    data: &[u8],
    guard: &AppendCondition,
) -> Result<ReplicatedEvent, Violation> {
    let position = end
        .store
        .append(&[claim(data)?], Some(guard))
        .await
        .map_err(|error| {
            Violation::new(
                SY_10,
                format!(
                    "the store refused its own claim under a guard it holds nothing against: \
                     {error}"
                ),
            )
        })?;
    let log = read_all(&end.store, SY_10).await?;
    let row = log
        .into_iter()
        .find(|row| row.position == position)
        .ok_or_else(|| {
            Violation::new(
                SY_10,
                format!("the store appended a claim at {position} and does not read it back"),
            )
        })?;
    end.claim = Some(row.id);
    Ok(ReplicatedEvent::new(row.id, row.recorded_at, row.event))
}

/// The near end pushes its own `claim` to the far end, unguarded, which takes
/// it under `rule` or refuses it (SY-33). A refusal must leave both logs
/// holding exactly the facts they held before it: the runner leaves the near
/// log alone (SY-34), and the far end took nothing.
async fn push<S: EventStore + IngestStore>(
    rule: MergeRule,
    edge: &Edge<S>,
    claim: &ReplicatedEvent,
    answer: Answer,
) -> Result<(), Violation> {
    if answer == Answer::Taken {
        return carry(rule, &edge.far, claim, None, "the near end's pushed claim").await;
    }
    let before = both_facts(edge).await?;
    // The far end declines, so the runner hands neither store anything.
    let after = both_facts(edge).await?;
    ensure(after == before, SY_10, || {
        format!(
            "a refused push changed the logs: near then far, they held {before:?} by type and \
             bytes before it and {after:?} after"
        )
    })
}

/// Both ends' facts, near then far.
async fn both_facts<S: EventStore>(edge: &Edge<S>) -> Result<[Vec<Fact>; 2], Violation> {
    Ok([
        facts(&read_all(&edge.near.store, SY_10).await?),
        facts(&read_all(&edge.far.store, SY_10).await?),
    ])
}

/// The near end pulls every fact the far end holds and it does not, one group
/// each, and takes each under `rule`. A claim the far end ingested travels
/// with `guard`, the one it arrived under; a claim the far end appended
/// itself travels unguarded; a reversal travels alone and unguarded.
///
/// What the near end lacks is found by type and bytes, which every fact here
/// carries uniquely, and not by identity. An identity-keyed runner sends the
/// same events from a faithful store, and this way a store that re-mints on
/// ingest fails VT-5's rule and not this one.
async fn pull<S: EventStore + IngestStore>(
    rule: MergeRule,
    edge: &Edge<S>,
    guard: &AppendCondition,
) -> Result<(), Violation> {
    let held = facts(&read_all(&edge.near.store, SY_10).await?);
    for row in read_all(&edge.far.store, SY_10).await? {
        if held.contains(&fact(&row)) {
            continue;
        }
        let travelling = (row.id.store() != edge.far.id).then_some(guard);
        let event = ReplicatedEvent::new(row.id, row.recorded_at, row.event);
        if *event.event.event_type() == SEAT_CLAIMED {
            carry(
                rule,
                &edge.near,
                &event,
                travelling,
                "a claim pulled from the far end",
            )
            .await?;
        } else {
            let took = ingest(
                &edge.near.store,
                &[IngestGroup::new(slice::from_ref(&event), &[])],
                SY_10,
            )
            .await?;
            ensure_counts(took, (1, 0, 0), "a reversal pulled from the far end", SY_10)?;
        }
    }
    Ok(())
}

/// The runner's step for one claim crossing into `to` under `rule`. The
/// `guard` the claim travels with, if any, rides on its group (SY-30) as
/// evidence, never an instruction (SY-6). Under [`MergeRule::Adjudicate`]
/// the runner judges by fact family (SY-35), from what `to` holds: another
/// claim on the seat earns a reversal. Under [`MergeRule::Accept`] it hands
/// nothing. The store evaluates nothing (SY-1) and must count exactly what it
/// was handed.
async fn carry<S: EventStore + IngestStore>(
    rule: MergeRule,
    to: &End<S>,
    claim: &ReplicatedEvent,
    guard: Option<&AppendCondition>,
    what: &str,
) -> Result<(), Violation> {
    let contested = rule == MergeRule::Adjudicate && contested(&to.store, claim).await?;
    let compensation = if contested {
        slice::from_ref(&to.reversal)
    } else {
        &[]
    };
    let group = IngestGroup::new(slice::from_ref(claim), compensation).with_guard(guard);
    let took: Ingested = ingest(&to.store, &[group], SY_10).await?;
    ensure_counts(took, (1, 0, usize::from(contested)), what, SY_10)
}

/// Whether `store` holds a claim on the seat other than `claim`, by type,
/// tags and bytes, never decoded.
async fn contested<S: EventStore>(store: &S, claim: &ReplicatedEvent) -> Result<bool, Violation> {
    let family = seat_family()?;
    let incoming: Fact = (
        String::from(claim.event.event_type().as_str()),
        claim.event.data().to_vec(),
    );
    Ok(read_all(store, SY_10)
        .await?
        .iter()
        .any(|row| family.matches(row.event_type(), row.tags()) && fact(row) != incoming))
}

/// `log`'s events by type and bytes, sorted, so two logs that hold the same
/// facts in different orders compare equal.
fn facts(log: &[SequencedEvent]) -> Vec<Fact> {
    let mut facts: Vec<Fact> = log.iter().map(fact).collect();
    facts.sort();
    facts
}

/// One event by type and bytes.
fn fact(row: &SequencedEvent) -> Fact {
    (
        String::from(row.event_type().as_str()),
        row.event.data().to_vec(),
    )
}

/// A claim on the seat carrying `data`, so each claim's bytes are its own.
fn claim(data: &[u8]) -> Result<Event, Violation> {
    Ok(opaque(SEAT_CLAIMED, data, SY_10)?.with_tags(seat()))
}

/// Every claim on the seat: the fact family the hub adjudicates.
fn seat_family() -> Result<Query, Violation> {
    let item = QueryItem::new([SEAT_CLAIMED], seat()).map_err(|error| {
        Violation::new(SY_10, format!("the rule's own query is invalid: {error}"))
    })?;
    Ok(Query::from_item(item))
}

/// The condition every claim is decided under: no claim on this seat yet.
fn seat_guard() -> Result<AppendCondition, Violation> {
    Ok(AppendCondition::new(seat_family()?))
}

fn seat() -> Tags {
    [Tag::from_static(SEAT)].into_iter().collect()
}

#[cfg(test)]
mod tests;
