//! Central's rule, mounted where central ingests what a device pushes.
//!
//! [`CentralRule`] is a [`happenstance_sync::runner::Adjudicate`]: central's
//! [`StoreSyncPeer`](happenstance_sync::StoreSyncPeer) asks it about each
//! group a device pushes, before the push lands, and lands the group and the
//! refunds it answers in one write (SY-2). Only central's peer mounts it, so
//! only central writes a refund (SY-7); a device's edge keeps the default,
//! which judges nothing. The decision itself is the domain's
//! [`adjudicate`](crate::adjudicate); this module reads what central holds,
//! decodes the push, and encodes the answer.
//!
//! # Why the bare `Adjudicate`
//!
//! For the reason [`Uplink`](crate::Uplink) implements the bare `Exchange`:
//! the story runs on one thread, the runner takes `!Send` rules as it takes
//! `!Send` peers, and the `Send` flavour would buy thread safety nobody uses.
//!
//! # What the refund carries
//!
//! The rule hands back plain [`Event`]s, built from the domain event's type,
//! tags and JSON bytes. An event `commit` writes also carries the typed
//! layer's codec tag in its metadata, and the typed layer has no public way to
//! build one outside `commit` (tracked in #492). A payload without the tag
//! decodes under the reader's own codec, which is JSON in every place here,
//! so the refund reads back the same everywhere.

use happenstance::{
    Boundary, CodecError, DomainEvent, Event, InvalidEventType, InvalidQuery, Json, Query,
    read_decision_model,
};
use happenstance_sync::runner::Adjudicate;
use happenstance_sync::{Classify, EventGroup, FailureClass, ReplicatedEvent};

use crate::domain::{BoxOffice, ShowId, ShowLedger, adjudicate};
use crate::uplink::{Central, CentralError};

/// Central's rule over one show: the first sales to reach central keep their
/// seats, and each later sale lands with its refund.
///
/// Holds its own read handle onto central, as the hook asks: a rule reads
/// what it judges from, and writes nothing.
#[derive(Debug, Clone)]
pub struct CentralRule {
    central: Central,
    show: ShowId,
}

impl CentralRule {
    /// The rule for `show`, reading what `central` holds.
    ///
    /// Takes both by value because the rule keeps them: a `SqliteEventStore`
    /// is a handle onto the same file, not a copy of the log.
    #[must_use]
    pub const fn new(central: Central, show: ShowId) -> Self {
        Self { central, show }
    }

    /// The show as central holds it now.
    async fn held(&self) -> Result<(ShowLedger, Query), RuleError> {
        let mut ledger = ShowLedger::new(&self.show);
        let query = ledger.query().map_err(RuleError::Query)?;
        let (found, _) = read_decision_model(&self.central, &query)
            .await
            .map_err(RuleError::Read)?;
        for event in &found {
            ledger.absorb(event, &Json).map_err(RuleError::Decode)?;
        }
        Ok((ledger, query))
    }
}

impl Adjudicate for CentralRule {
    type Error = RuleError;

    /// The refunds `group` lands with: one for each sale in it that lands
    /// past capacity, after what central holds and what lands before it in
    /// the same push.
    ///
    /// # Errors
    ///
    /// [`RuleError`] if central's history cannot be read or an event cannot
    /// be decoded or encoded. Nothing in the push is written.
    async fn compensation(
        &self,
        group: &EventGroup,
        earlier: &[EventGroup],
    ) -> Result<Vec<Event>, RuleError> {
        let (ledger, query) = self.held().await?;
        let before = earlier
            .iter()
            .map(|group| decode(&query, &group.events))
            .collect::<Result<Vec<_>, _>>()?;
        let arriving = decode(&query, &group.events)?;
        adjudicate(ledger, before, arriving)
            .iter()
            .map(encode)
            .collect()
    }
}

/// The show's events among `events`, decoded. An event outside the show's
/// query is not this rule's to judge, and is passed over.
fn decode<'a>(
    query: &Query,
    events: impl IntoIterator<Item = &'a ReplicatedEvent>,
) -> Result<Vec<BoxOffice>, RuleError> {
    events
        .into_iter()
        .map(|replicated| &replicated.event)
        .filter(|event| query.matches(event.event_type(), event.tags()))
        .map(|event| {
            BoxOffice::decode(&Json, event.event_type(), event.data()).map_err(RuleError::Decode)
        })
        .collect()
}

/// A decided event as the store takes it: its type, its tags, its JSON bytes.
fn encode(event: &BoxOffice) -> Result<Event, RuleError> {
    let data = event.encode(&Json).map_err(RuleError::Encode)?;
    Ok(Event::new(event.event_type(), data)
        .map_err(RuleError::EventType)?
        .with_tags(event.tags()))
}

/// Why central's rule could not judge a push. Every case writes nothing.
#[derive(Debug, thiserror::Error)]
pub enum RuleError {
    /// The show's query could not be derived.
    #[error("the show's query")]
    Query(#[source] InvalidQuery),
    /// Central's history could not be read.
    #[error("reading central's history")]
    Read(#[source] CentralError),
    /// An event central holds, or one in the push, did not decode.
    #[error("decoding an event to judge")]
    Decode(#[source] CodecError),
    /// A refund did not encode.
    #[error("encoding a refund")]
    Encode(#[source] CodecError),
    /// A refund's event type was refused. The type is a declared constant,
    /// so this is a broken declaration, reported rather than assumed away.
    #[error("a refund's event type")]
    EventType(#[source] InvalidEventType),
}

impl Classify for RuleError {
    /// A failed read keeps the class the store stated (SY-36). Everything
    /// else fails the same way on every retry, so it stops the edge.
    fn class(&self) -> FailureClass {
        match self {
            Self::Read(error) => error.class(),
            Self::Query(_) | Self::Decode(_) | Self::Encode(_) | Self::EventType(_) => {
                FailureClass::Stop
            }
        }
    }
}
