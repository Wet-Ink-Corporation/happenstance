//! The proposed `Projection` trait, declared here rather than in
//! `crates/happenstance`.
//!
//! Everything in this module is the shape the projection-apply brief
//! recommends for the phase-17 apply record, spelled so the compiler can
//! disagree with it:
//!
//! * `apply` is `async` and takes the store's batch, so a projection over a
//!   live transaction can `.await` a statement through it;
//! * the event arrives inside a [`Delivered`] envelope that carries the
//!   [`EventId`] and has **no** accessor for the local `SequencedEvent::position`
//!   (SY-21);
//! * the projection names its own `type Error` (PS-28), convertible from the
//!   store's so `?` works on a batch statement;
//! * `on_error` is a **provided** method defaulting to [`Policy::Halt`] (the
//!   PS-27 seam), written as `-> impl Future` with a block rather than as
//!   `async fn` with a body, because `trait_variant` 0.1.3 copies the block
//!   verbatim into the `Send` flavour and an `async fn` body copied into a
//!   `-> impl Future + Send` signature is not a future.
//!
//! # Why the trait spells `<Self::Store as ProjectionStore>::Batch` in full
//!
//! The shipped runner uses two private aliases, `StoreBatch<P>` and
//! `StoreError<P>`, both written against `<P as Projection>`. Inside a
//! `#[trait_variant::make]` trait that is the wrong trait for the `Send` copy:
//! the macro copies the tokens, so `SendProjection` would bound its own
//! associated type on `<Self as Projection>::Store`, which only exists through
//! the blanket impl `SendProjection` itself generates. `Self::Store` resolves
//! against whichever trait the copy sits in, which is what the two flavours
//! need.

use core::future::Future;

use happenstance::{CodecError, DomainEvent, EventId, ProjectionId, ProjectionStore, Tags};

/// One event handed to [`Projection::apply`], and what is known about it.
///
/// **There is no `position()`.** A convergent projection must not observe the
/// local `SequencePosition` (SY-21), because two replicas that ingested the same
/// events in different orders assign them different local positions, and a
/// read model keyed on one would diverge. What it gets instead is the
/// [`EventId`] — the *origin* store and the origin's position — which is the
/// same on every replica.
///
/// The control fence first — the envelope's accessors exist and compile:
///
/// ```
/// use apply_shape::Delivered;
/// use happenstance::EventId;
///
/// fn inspect(delivered: &Delivered<u8>) -> (EventId, u8) {
///     (delivered.id(), *delivered.event())
/// }
/// # let _ = inspect;
/// ```
///
/// And the refusal the control makes legible: asking the envelope for a
/// position is `error[E0599]` (no method named `position`). The fence claims the
/// code, but RS-62-1 records that rustdoc 1.97.1 does not check it — the
/// compiling fence above is what goes red if `Delivered` is renamed, and the
/// `demonstrate-refusals` feature is what produces the transcript.
///
/// ```compile_fail,E0599
/// use apply_shape::Delivered;
///
/// fn leak(delivered: &Delivered<u8>) {
///     let _ = delivered.position();
/// }
/// # let _ = leak;
/// ```
///
/// Note what is *not* refused: `delivered.id().position()` compiles and
/// returns a `SequencePosition` — the **origin's**. SY-21's MUST NOT has to say
/// "the local position" or that sentence is false on its face; this is the
/// finding the brief asked the record to carry.
///
/// `#[non_exhaustive]` is redundant while every field is private — private
/// fields already refuse a struct literal outside this crate — and is kept
/// because the brief's shape names it and it costs nothing if a field is ever
/// made public.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct Delivered<E> {
    id: EventId,
    event: E,
}

impl<E> Delivered<E> {
    /// Pairs an event with its identity.
    ///
    /// Public, and that is a finding rather than a convenience: an application
    /// unit-testing its own `apply` has to be able to build the argument, and
    /// a runner-only constructor would force every such test through a store.
    /// Nothing about SY-21 is lost — the constructor takes an `EventId`, and a
    /// local position still has nowhere to go.
    #[must_use]
    pub const fn new(id: EventId, event: E) -> Self {
        Self { id, event }
    }

    /// Which store first accepted this event, and where it sat **there**.
    #[must_use]
    pub const fn id(&self) -> EventId {
        self.id
    }

    /// The decoded event.
    #[must_use]
    pub const fn event(&self) -> &E {
        &self.event
    }

    /// The decoded event, by value.
    #[must_use]
    pub fn into_event(self) -> E {
        self.event
    }
}

/// What the runner does with an event that could not be applied.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Policy {
    /// Discard the chunk, leave the checkpoint where it was, and report.
    Halt,
    /// Leave this event out and carry on in the same batch. The checkpoint
    /// advances past it when the chunk commits.
    Skip,
}

/// Why an event could not be applied, as [`Projection::on_error`] sees it.
///
/// Both halves are here because the motivating skip — a crypto-shredded
/// payload — is a *decode* failure, not an apply failure. The failure carries
/// the [`EventId`] and, like [`Delivered`], no local position.
#[derive(Debug)]
#[non_exhaustive]
pub enum ApplyFailure<'a, A> {
    /// The payload did not decode into the projection's event type.
    #[non_exhaustive]
    Decode {
        /// The event's identity.
        id: EventId,
        /// The codec's refusal.
        source: &'a CodecError,
    },
    /// [`Projection::apply`] refused the decoded event.
    #[non_exhaustive]
    Apply {
        /// The event's identity.
        id: EventId,
        /// The projection's own refusal.
        source: &'a A,
    },
}

impl<A> ApplyFailure<'_, A> {
    /// The identity of the event that failed.
    #[must_use]
    pub const fn id(&self) -> EventId {
        match self {
            Self::Decode { id, .. } | Self::Apply { id, .. } => *id,
        }
    }
}

/// A read model an application builds from its own decoded events — the
/// proposed shape.
///
/// Declared once, without `Send`; `trait_variant` derives [`SendProjection`]
/// and a blanket `impl<T: SendProjection> Projection for T`. An application
/// implements exactly one of the two, as adapters already do for the ports
/// (RS-20-4).
#[trait_variant::make(SendProjection: Send)]
pub trait Projection {
    /// The application's own event enum. The runner decodes.
    type Event: DomainEvent;

    /// The store this read model and its checkpoint live in.
    type Store: ProjectionStore;

    /// The projection's own refusal (PS-28).
    ///
    /// `From` the store's error so `batch.execute(..).await?` works inside
    /// `apply` without a hand-written `map_err`. No default: `A = W` would
    /// re-bless forging an application refusal into the adapter's error.
    type Error: core::error::Error + From<<Self::Store as ProjectionStore>::Error> + 'static;

    /// Which read model this is, within [`Self::Store`].
    fn id(&self) -> &ProjectionId;

    /// The tags this projection is scoped to.
    fn scope(&self) -> &Tags;

    /// Applies one delivered event into the store's open write set.
    ///
    /// `async` so a live batch can be written through; a buffered batch's
    /// implementation never awaits, and its future is ready at the first poll.
    async fn apply(
        &mut self,
        event: Delivered<Self::Event>,
        batch: &mut <Self::Store as ProjectionStore>::Batch,
    ) -> Result<(), Self::Error>;

    /// Decides what happens to an event that could not be applied.
    ///
    /// Provided, defaulting to [`Policy::Halt`], so a projection that says
    /// nothing keeps today's behaviour. An override may write a skip record
    /// **into `batch`** — through the concrete store's inherent API, never
    /// through a library-owned vocabulary — so the record commits with the
    /// checkpoint that moves past the event (PS-9, PS-11).
    ///
    /// Spelled `-> impl Future` with a block, not `async fn` with a body:
    /// `trait_variant` 0.1.3 copies the block into the `Send` flavour's
    /// `-> impl Future<..> + Send` signature, where an `async fn` body would be
    /// a plain `Result` rather than a future.
    ///
    /// The parameters are named, not `_`-prefixed, and discarded in the body.
    /// `trait_variant` 0.1.3's blanket impl forwards every parameter **by
    /// name**, so an `_failure` in the declaration is a used underscore binding
    /// in generated code, and `clippy::used_underscore_binding` (pedantic)
    /// rejects it at the trait — a finding, recorded in `README.md`.
    fn on_error(
        &mut self,
        failure: &ApplyFailure<'_, Self::Error>,
        batch: &mut <Self::Store as ProjectionStore>::Batch,
    ) -> impl Future<Output = Result<Policy, Self::Error>> {
        let _ = failure;
        let _ = batch;
        async { Ok(Policy::Halt) }
    }
}
