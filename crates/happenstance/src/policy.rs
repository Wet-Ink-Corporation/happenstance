//! What a projection does with an event it cannot take (PS-27).
//!
//! Its own module rather than a section of `runner.rs`, for the reasons
//! `delivered.rs` gives: `runner.rs` is the longest file in the crate, every
//! line added above `run_projection` moves the citations that name it, and
//! `tests/doc_budget.rs` reads `src/*.rs` and nothing below it.

use happenstance_core::EventId;

use crate::codec::CodecError;

/// What the runner does with an event the projection could not take.
///
/// [`Projection::on_error`](crate::Projection::on_error) answers with one.
/// Its default answers [`Halt`](Self::Halt), so a projection that says
/// nothing stops at the first event it cannot take, as it always has.
///
/// `#[non_exhaustive]`, so a later answer is not a breaking change. An
/// application constructs a `Policy` and never needs to match one.
///
/// # Skip, and record that you did
///
/// A projection that skips an unreadable payload, and writes a record of it
/// into the batch that moves the checkpoint past it. Nothing commits the
/// record without the move, and nothing commits the move without the record.
/// The fence needs `memory` and `json`, which are both defaults.
///
/// ```
/// use happenstance::{bytes::Bytes, Codec, CodecError, DomainEvent};
/// use happenstance::{ApplyFailure, Delivered, Policy, Projection};
/// use happenstance::{Chunk, Event, EventStore, EventType, Json, Tags};
/// use happenstance::{MemoryEventStore, MemoryProjectionBatch};
/// use happenstance::{MemoryProjectionStore, MemoryProjectionStoreError};
/// use happenstance::{ProjectionId, run_projection};
/// # use std::error::Error;
/// # #[tokio::main] async fn main() -> Result<(), Box<dyn Error>> {
/// # #[derive(serde::Serialize, serde::Deserialize)]
/// # enum Part { Shredded }
/// # const SHREDDED: EventType = EventType::from_static("PartShredded");
/// # impl DomainEvent for Part {
/// #     const EVENT_TYPES: &'static [EventType] = &[SHREDDED];
/// #     fn event_type(&self) -> EventType { SHREDDED }
/// #     fn tags(&self) -> Tags { Tags::empty() }
/// #     fn encode<C: Codec>(&self, c: &C) -> Result<Bytes, CodecError> {
/// #         c.encode(self)
/// #     }
/// #     fn decode<C: Codec>(c: &C, _t: &EventType, d: &Bytes)
/// #         -> Result<Self, CodecError> { c.decode(d) }
/// # }
///
/// struct Shreds { id: ProjectionId, scope: Tags, shredded: u64 }
///
/// impl Projection for Shreds {
///     type Event = Part;
///     type Store = MemoryProjectionStore;
///     type Error = MemoryProjectionStoreError;
///     fn id(&self) -> &ProjectionId { &self.id }
///     fn scope(&self) -> &Tags { &self.scope }
///     async fn apply(
///         &mut self,
///         _: Delivered<Part>,
///         batch: &mut MemoryProjectionBatch,
///     ) -> Result<(), MemoryProjectionStoreError> {
///         self.shredded += 1;
///         batch.write("shredded", self.shredded);
///         Ok(())
///     }
///     async fn on_error(
///         &mut self,
///         failure: &ApplyFailure<'_, Self::Error>,
///         batch: &mut MemoryProjectionBatch,
///     ) -> Result<Policy, MemoryProjectionStoreError> {
///         match failure {
///             ApplyFailure::Decode { id, .. } => {
///                 batch.write(format!("skipped:{id}"), 1);
///                 Ok(Policy::Skip)
///             }
///             _ => Ok(Policy::Halt),
///         }
///     }
/// }
///
/// let events = MemoryEventStore::new();
/// let good = Event::new("PartShredded", Json.encode(&Part::Shredded)?)?;
/// let bad = Event::new("PartShredded", &b"unreadable"[..])?;
/// events.append(&[good, bad], None).await?;
///
/// let models = MemoryProjectionStore::new();
/// let mut shreds = Shreds {
///     id: ProjectionId::new("shreds")?,
///     scope: Tags::empty(),
///     shredded: 0,
/// };
/// let chunk = Chunk::of(64.try_into()?);
/// let done =
///     run_projection(&events, &models, &mut shreds, &Json, chunk)
///         .await?;
///
/// assert_eq!((done.applied, done.skipped), (1, 1));
/// let poison = events.snapshot()[1].id;
/// assert_eq!(models.get(&format!("skipped:{poison}")), Some(1));
/// # Ok::<(), Box<dyn Error>>(()) }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Policy {
    /// Discard the chunk, leave the checkpoint where it was, and report.
    Halt,
    /// Leave this event out and carry on in the same batch.
    ///
    /// The checkpoint moves past it when the chunk commits, together with
    /// whatever `on_error` wrote.
    ///
    /// Not universal: it carries on only in a batch that is still usable. A
    /// buffered batch, such as SQLite's, meets its database only at commit,
    /// so a statement the database refuses fails the commit and no policy is
    /// asked. A batch that *is* a live transaction, as the
    /// `LivePostgresBatch` of `happenstance-postgres` is, sends each statement
    /// as `apply` runs, and one the server refuses aborts the whole
    /// transaction: the record `on_error` writes is refused with it, and the
    /// chunk commits nothing.
    ///
    /// The runner sets no savepoint around `apply`, because that would be a
    /// write into a batch it knows nothing about, which PS-9 forbids (ADR-0074
    /// decision 6). The savepoint is the application's. An `apply` that may
    /// meet a server refusal issues `SAVEPOINT` before its statements,
    /// `RELEASE SAVEPOINT` after them, and `ROLLBACK TO SAVEPOINT` before it
    /// returns the refusal. `on_error` is then handed a usable batch holding
    /// none of that event's writes, and its record commits with the
    /// checkpoint. `happenstance-postgres`'s live tests run this pattern.
    Skip,
}

/// An event that could not be applied, as `on_error` is handed it.
///
/// Both halves are here because the skip that motivates the seam, a
/// crypto-shredded payload, is a *decode* failure rather than an apply one.
/// An enum rather than a struct with an optional source: the two halves carry
/// differently typed sources, and a failure that is neither cannot be built.
///
/// The sources are borrowed. The runner keeps the error, because on
/// [`Policy::Halt`] it reports it in
/// [`ProjectionError`](crate::ProjectionError).
///
/// # There is no arrival position, on purpose
///
/// The failure carries the event's [`EventId`] and, like
/// [`Delivered`](crate::Delivered), nothing that yields the local position:
/// a record keyed on it would differ between replicas that hold the same log
/// (SY-21). The first fence compiles, and is the control.
///
/// ```
/// use happenstance::{ApplyFailure, CodecError, EventId};
/// fn which(failure: &ApplyFailure<'_, CodecError>) -> EventId {
///     failure.id()
/// }
/// # let _ = which;
/// ```
///
/// The second asks for the position, and does not compile. Rustdoc 1.97.1
/// does not check the error code on a `compile_fail` fence (RS-62-1), so the
/// `E0599` is a statement of intent and the control keeps this one honest.
///
/// ```compile_fail,E0599
/// use happenstance::{ApplyFailure, CodecError};
/// fn leak(failure: &ApplyFailure<'_, CodecError>) {
///     let _ = failure.position();
/// }
/// # let _ = leak;
/// ```
///
/// Both the enum and its variants are `#[non_exhaustive]`, so a later fact
/// can join without a break. Match a variant with `..`, and add a `_` arm.
#[derive(Debug)]
#[non_exhaustive]
pub enum ApplyFailure<'a, A> {
    /// The payload did not decode into the projection's event type.
    #[non_exhaustive]
    Decode {
        /// The event's identity: where it originated, not where it arrived.
        id: EventId,
        /// The codec's refusal.
        source: &'a CodecError,
    },
    /// `apply` refused the decoded event.
    #[non_exhaustive]
    Apply {
        /// The event's identity: where it originated, not where it arrived.
        id: EventId,
        /// The projection's own refusal.
        source: &'a A,
    },
}

impl<'a, A> ApplyFailure<'a, A> {
    /// A decode failure, for an application unit-testing its `on_error`.
    ///
    /// The runner is the one caller in this crate. It takes an [`EventId`],
    /// so a local position has nowhere to go.
    #[must_use]
    pub const fn decode(id: EventId, source: &'a CodecError) -> Self {
        Self::Decode { id, source }
    }

    /// An apply failure, for an application unit-testing its `on_error`.
    #[must_use]
    pub const fn apply(id: EventId, source: &'a A) -> Self {
        Self::Apply { id, source }
    }

    /// The identity of the event that failed, whichever half failed.
    ///
    /// By value, because an [`EventId`] is `Copy`.
    #[must_use]
    pub const fn id(&self) -> EventId {
        match self {
            Self::Decode { id, .. } | Self::Apply { id, .. } => *id,
        }
    }
}

#[cfg(test)]
mod tests {
    use happenstance_core::{EventId, SequencePosition, StoreId};

    use super::ApplyFailure;
    use crate::codec::CodecError;

    #[test]
    fn either_half_names_the_event_it_failed_on() {
        let position = SequencePosition::new(9).expect("nine is not zero");
        let id = EventId::new(StoreId::from_bytes([3; 16]), position);
        let refusal = CodecError::UnknownTag {
            tag: "a stand-in".into(),
        };

        let decode = ApplyFailure::<'_, CodecError>::decode(id, &refusal);
        let apply = ApplyFailure::apply(id, &refusal);

        assert_eq!(decode.id(), id);
        assert_eq!(apply.id(), id);
        assert!(matches!(decode, ApplyFailure::Decode { .. }));
        assert!(matches!(apply, ApplyFailure::Apply { .. }));
    }
}
