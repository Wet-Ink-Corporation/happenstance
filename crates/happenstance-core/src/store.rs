//! The event store port, in two flavours.
//!
//! # Why there are two traits
//!
//! `async fn` in a trait produces an anonymous future whose `Send`-ness is fixed
//! by the trait definition. Two of happenstance's targets disagree about that:
//!
//! * **native / tokio** needs `Send` futures, or `tokio::spawn` rejects them;
//! * **`wasm32` / Cloudflare Workers** is single-threaded and its futures are
//!   `!Send`, so a `Send` bound cannot be satisfied there at all.
//!
//! Rather than duplicate the trait, [`EventStore`] is written without any `Send`
//! requirement and `trait_variant` derives [`SendEventStore`] from it. The two
//! are related in exactly one direction, which is the sound one:
//!
//! ```text
//!     impl SendEventStore  ==>  EventStore comes free
//!     impl EventStore      =/=> SendEventStore  (cannot conjure Send)
//! ```
//!
//! ## What that means in practice
//!
//! * **Writing an adapter?** Implement [`SendEventStore`] if your store can be
//!   shared across threads — which is every native adapter. Implement
//!   [`EventStore`] only when you genuinely cannot, as on wasm.
//! * **Writing code that takes a store?** Bound on [`EventStore`]. It is the
//!   weaker requirement, so it accepts both kinds. Reach for
//!   [`SendEventStore`] only where you actually need to cross a thread
//!   boundary.
//!
//! ## Import one flavour, not both
//!
//! Because a `SendEventStore` is also an `EventStore`, bringing **both** names
//! into scope makes method-call syntax ambiguous:
//!
//! ```text
//! error[E0034]: multiple applicable items in scope
//!   |
//!   |     store.read(&query, options)
//!   |           ^^^^ multiple `read` found
//! ```
//!
//! Import only the one you are binding on — [`EventStore`] in almost every
//! case. If you genuinely need both in one module, disambiguate with
//! fully-qualified syntax: `SendEventStore::read(&store, &query, options)`.
//!
//! ## Naming
//!
//! The names carry the distinction deliberately. `trait_variant`'s own
//! convention would call the `!Send` flavour `LocalEventStore`, but "local"
//! already means something else here — a local-first application's on-device
//! store — and conflating the two would be a permanent source of confusion.

use alloc::vec::Vec;

use futures_core::Stream;

use crate::append::AppendCondition;
use crate::error::AppendError;
use crate::event::{Event, SequencePosition, SequencedEvent};
use crate::query::{Query, ReadOptions};

/// A DCB-compliant event store.
///
/// This is the `!Send` flavour and the one to use in generic bounds; see the
/// [module documentation](self) for why. Adapters that can be `Send` should
/// implement [`SendEventStore`] instead and get this for free.
///
/// # Implementing this trait
///
/// The specification's requirements are obligations on the implementer, and
/// [`happenstance-testkit`](https://docs.rs/happenstance-testkit) checks every one of
/// them. An adapter is not finished until it passes that suite.
///
/// For a runnable end-to-end example see
/// [`MemoryEventStore`](crate::MemoryEventStore).
///
/// # Writing generic code over a store
///
/// Bound on this trait, not [`SendEventStore`], unless you need to cross a
/// thread boundary:
///
/// ```
/// use happenstance_core::{EventStore, Query, ReadOptions, collect};
///
/// async fn count_all<S: EventStore>(store: &S) -> Result<usize, S::Error> {
///     let events = collect(store.read(&Query::all(), ReadOptions::new())).await?;
///     Ok(events.len())
/// }
/// ```
#[trait_variant::make(SendEventStore: Send)]
pub trait EventStore {
    /// How this adapter fails for its own reasons.
    ///
    /// Append-condition violations do **not** belong here — they are reported
    /// through [`AppendError::ConditionViolated`], so that callers can tell
    /// "retry the decision" from "something broke" without knowing which
    /// adapter they hold.
    type Error: core::error::Error + 'static;

    /// Reads the events matching `query`, in the order `options` asks for.
    ///
    /// The returned stream is **lazy**: nothing is executed until it is first
    /// polled, and failures surface as `Err` items rather than up front. That
    /// is what lets an adapter stream a million-event replay without buffering
    /// it, and it is why this method is not `async` — putting the stream at the
    /// top level of the return type is what allows [`SendEventStore`] to mark
    /// the *stream* `Send`, not merely the future that produces it.
    ///
    /// Use [`collect`] when a `Vec` is genuinely what you want.
    ///
    /// # Ordering
    ///
    /// Ascending by [`SequencePosition`] unless
    /// [`ReadOptions::backwards`](crate::ReadOptions::backwards) is set.
    /// [`ReadOptions::from`](crate::ReadOptions::from) is **inclusive**.
    fn read(
        &self,
        query: &Query,
        options: ReadOptions,
    ) -> impl Stream<Item = Result<SequencedEvent, Self::Error>>;

    /// Atomically appends `events`, rejecting the write if `condition` matches.
    ///
    /// Returns the position assigned to the **last** appended event. The
    /// specification does not require this, but every caller that checkpoints a
    /// projection or builds a follow-up append condition needs it, and only the
    /// store knows it.
    ///
    /// # Atomicity
    ///
    /// Either every event lands or none does. A rejected append must leave the
    /// store byte-identical.
    ///
    /// # Errors
    ///
    /// * [`AppendError::ConditionViolated`] when the store already holds an
    ///   event matching `condition`. This is routine under contention: rebuild
    ///   the decision model and retry.
    /// * [`AppendError::Store`] for adapter-specific failures.
    async fn append(
        &self,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<Self::Error>>;
}

/// Drains a [`read`](EventStore::read) stream into a `Vec`, stopping at the
/// first error.
///
/// Convenient for tests, small reads, and rebuilding a decision model that fits
/// in memory. Do not use it to replay an entire log.
///
/// Implemented by hand rather than pulling in `futures-util`, so that the
/// contract crate's dependency graph stays as small as its semver surface is
/// stable.
///
/// # Errors
///
/// Returns the first error the stream yields, discarding any events already
/// collected.
pub async fn collect<S, T, E>(stream: S) -> Result<Vec<T>, E>
where
    S: Stream<Item = Result<T, E>>,
{
    use core::task::Poll;

    let mut stream = core::pin::pin!(stream);
    let mut collected = Vec::new();

    core::future::poll_fn(|cx| {
        loop {
            match stream.as_mut().poll_next(cx) {
                Poll::Ready(Some(Ok(item))) => collected.push(item),
                Poll::Ready(Some(Err(err))) => return Poll::Ready(Err(err)),
                Poll::Ready(None) => return Poll::Ready(Ok(())),
                Poll::Pending => return Poll::Pending,
            }
        }
    })
    .await?;

    Ok(collected)
}

/// Reads `query` and reports both the matching events and the position to use
/// in a follow-up [`AppendCondition`].
///
/// This is the read half of the DCB command loop: build a decision model from
/// what you can see, then append conditioned on nothing new having appeared.
/// The returned position is the last one observed, or `None` when nothing
/// matched — which is exactly what
/// [`AppendCondition::after_opt`](crate::AppendCondition::after_opt) expects.
///
/// # Errors
///
/// Returns the adapter's error if the read fails.
pub async fn read_decision_model<S>(
    store: &S,
    query: &Query,
) -> Result<(Vec<SequencedEvent>, Option<SequencePosition>), S::Error>
where
    S: EventStore,
{
    let events = collect(store.read(query, ReadOptions::new())).await?;
    let last = events.last().map(|event| event.position);
    Ok((events, last))
}
