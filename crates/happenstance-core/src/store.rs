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
//!  --> src/lib.rs:7:19
//!   |
//! 7 |     let _ = store.read(&query, options);
//!   |                   ^^^^ multiple `read` found
//!   |
//!   = note: candidate #1 is defined in an impl of the trait `SendEventStore` for the type `MemoryEventStore`
//!   = note: candidate #2 is defined in an impl of the trait `EventStore` for the type `TraitVariantBlanketType`
//! help: disambiguate the method for candidate #2
//!   |
//! 7 -     let _ = store.read(&query, options);
//! 7 +     let _ = EventStore::read(&store, &query, options);
//! ```
//!
//! In words, because a caret does not survive a search hit or a screen reader:
//! `store.read(…)` is ambiguous because both [`EventStore`] and
//! [`SendEventStore`] are in scope and each of them supplies a `read`.
//!
//! Import only the one you are binding on — [`EventStore`] in almost every
//! case. If you genuinely need both in one module, disambiguate with
//! fully-qualified syntax: `SendEventStore::read(&store, &query, options)`.
//!
//! The block above is a `text` fence and nothing compiles it. The error *code*
//! is asserted by the compiled `rust,compile_fail,E0034` examples in
//! `standards/rust/20-two-flavour-ports.md` and `standards/rust/00-prime-directives.md`;
//! the notes' wording and `TraitVariantBlanketType` are rustc 1.97.1's, asserted by nothing.
//!
//! For why there are two flavours at all, and what an adapter looks like once
//! you accept them, the adapter reading order at `docs/adapter-reading-order.md`
//! sequences what is already written. Named, not linked: it is a page, not an item.
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
use crate::identity::EventId;
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
/// For a runnable end-to-end example see `MemoryEventStore`, which the `memory`
/// feature provides. It is not linked because this item exists without that
/// feature and the link would not resolve (D13).
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
// The workspace's first `#[doc(alias)]`, so the rule it is added under is
// written here rather than left to be inferred. An alias is a *search key* and
// not a pointer: it moves a reader who is already searching and does nothing
// for the reader who is reading, so it never substitutes for the cross-reference
// at the stall in this module's documentation. It is permitted only where the
// string a reader types is one that rustc, the specification, or a recorded
// reader question actually emits, and is not the item's own name or a substring
// of it. Both strings below are rustc's, from the diagnostic reproduced above.
// Rejected: synonym farming (`eventstore`, `es`, `event-store`), and aliases for
// concepts rather than for strings. No pointer-register row is filed: an
// attribute cannot rot independently of the item it sits on, because deleting
// the item deletes the alias.
//
// Two attributes, not the three the design projected, and the reason is
// mechanical rather than editorial. `SendEventStore` is *derived* — the
// `trait_variant` expansion builds it with `..tr.clone()`
// (`trait-variant-0.1.3/src/variant.rs:115-123`), which copies the trait's
// attributes verbatim — so there is no item on which to write a variant-only
// alias, and every alias written here lands on both flavours. Two written is
// four in the search index, which is the coverage the criterion asked for.
//
// Residual risk, named: `TraitVariantBlanketType` is an *internal* name of that
// expansion. If it is renamed upstream the string here goes stale and nothing
// in this repository catches it.
#[doc(alias = "E0034")]
#[doc(alias = "TraitVariantBlanketType")]
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
    /// specification does not require this, but it is what checkpointing and
    /// reporting need, and only the store knows it.
    ///
    /// **It is not a sound `after` for a follow-up condition** unless the
    /// caller has already read up to it. Positions may have gaps and another
    /// writer may hold one *below* this value that this caller never saw, so a
    /// condition anchored here silently excludes exactly the events a condition
    /// exists to catch. The sound `after` comes from a read:
    /// [`read_decision_model`] returns the last position actually observed,
    /// which is what
    /// [`AppendCondition::after_opt`](crate::AppendCondition::after_opt)
    /// expects.
    ///
    /// # Atomicity
    ///
    /// Either every event lands or none does. A rejected append must leave the
    /// store byte-identical.
    ///
    /// # Cancellation
    ///
    /// **Dropping this future does not cancel the append.** An adapter MAY
    /// commit a batch whose future was dropped, and a caller MUST NOT read a
    /// dropped future as evidence that nothing landed. There is deliberately no
    /// error value for it — a dropped future produces no `Result` at all, so
    /// the outcome has nowhere to go, and the contract states the absence of a
    /// promise rather than inventing a variant that could never be delivered.
    ///
    /// What *is* promised is `# Atomicity`, which bounds what the silence can
    /// cost: whichever the adapter does, the batch is applied in full or not at
    /// all and never in part. Each adapter MUST state which of the two it does.
    ///
    /// At the edge this is the ordinary termination path rather than an exotic
    /// one — a client disconnect, a CPU limit, a Durable Object or pod
    /// eviction. The shape that looks cancellation-safe and is not: a pooled
    /// `rusqlite` adapter doing its work in `spawn_blocking`. Dropping the
    /// `JoinHandle` does not cancel the closure, the `COMMIT` runs, the caller
    /// is told nothing, and an operator retries a payment that already went
    /// out.
    ///
    /// # Resolving an unknown outcome
    ///
    /// **A conditional append is at-most-once under verbatim reissue.** Where
    /// the events being appended are themselves matched by the condition's
    /// query, reissuing the identical batch resolves what a dropped future left
    /// unknown: [`AppendError::ConditionViolated`] means the first attempt
    /// landed, `Ok` means it had not and now has. Either way the store holds
    /// exactly one copy. No identity, no idempotency key and no extra operation
    /// — the guarantee falls out of the condition the caller already wrote.
    ///
    /// It has three limits, stated here because a guarantee whose limits are
    /// unstated gets read as universal:
    ///
    /// * An **unconditional** append has no such property. Reissuing it appends
    ///   a second copy, and nothing in this port can prevent that.
    /// * A **conditional** append whose query does not match its own events has
    ///   none either: conditioning on `CourseCapacityChanged` while appending
    ///   `StudentSubscribed` leaves the reissue indistinguishable from a first
    ///   attempt. That is the *common* shape, not a corner — it is what a
    ///   decision that reads one thing and writes another looks like.
    /// * The reissue must be **verbatim**. This is not the
    ///   [`ConditionViolated`](AppendError::ConditionViolated) re-decide path,
    ///   where the correct response is to re-read, rebuild the decision model
    ///   and produce *different* events. The two arrive as the same error value
    ///   and mean opposite things — one says "your write already happened", the
    ///   other says "the world moved, decide again" — and collapsing them is
    ///   the mistake this section exists to prevent.
    ///
    /// A caller needing at-most-once for the second and third shapes supplies
    /// its own dedup in the domain. A natural key in the tags is the mechanism,
    /// and it is queryable; an identity buried in
    /// [`Event::metadata`](crate::Event::metadata) is not, because a query
    /// matches on type and tags only.
    ///
    /// # Errors
    ///
    /// * [`AppendError::NoEvents`] when `events` is empty. The specification
    ///   defines a batch as non-empty, so there is no position to return. This
    ///   check MUST precede the condition check: an empty batch is the caller's
    ///   own bug and retrying cannot fix it, whereas `ConditionViolated` means
    ///   "retry", so reporting the violation for an empty batch puts a correct
    ///   client into a loop that never terminates.
    /// * [`AppendError::ConditionViolated`] when the store already holds an
    ///   event matching `condition`. This is routine under contention: rebuild
    ///   the decision model and retry.
    /// * [`AppendError::Store`] for adapter-specific failures.
    async fn append(
        &self,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<Self::Error>>;

    /// The highest position currently visible in the store, or `None` when it
    /// holds nothing.
    ///
    /// # Why this is required rather than provided
    ///
    /// A provided body would have to be
    /// `read(&Query::all(), backwards().limit(1))`, which is a scan on any store
    /// that cannot push the ordering down, and a blanket implementation cannot
    /// be overridden per adapter. A SQLite adapter wants `SELECT max(position)`.
    /// The cost of `required` is a body in every implementation; the cost of
    /// `provided` is a fast path no adapter can install.
    ///
    /// There is a second reason, and it is the one that settles it: the only
    /// provided form holds `&self` across an `await`, which needs
    /// `where Self: Sync` — and the `!Send` edge adapter that the bare flavour
    /// exists for is not `Sync`, so the convenience body would fail to compile
    /// for exactly the adapter it was meant to spare.
    ///
    /// # What it is for
    ///
    /// Answering "am I caught up?" — a projection runner compares its checkpoint
    /// with this for **equality or ordering**. Do not subtract two positions and
    /// read the difference as a count of outstanding events: positions are an
    /// opaque ordering key and the specification permits gaps, so the difference
    /// is not a number of anything.
    ///
    /// # Errors
    ///
    /// Returns the adapter's error if the head cannot be determined.
    async fn head(&self) -> Result<Option<SequencePosition>, Self::Error>;

    /// Whether this store holds an event with `id`.
    ///
    /// The membership question a replicating peer must be able to ask without
    /// constructing a [`Query`] and without parsing any payload — which is why
    /// [`EventId`] is deliberately outside the query language.
    ///
    /// # Why every store can answer this cheaply
    ///
    /// Every store already holds at most one event per `EventId`, so the index
    /// that answers this exists wherever the contract is honoured, and no
    /// adapter is taxed with a new one. A store that has never ingested anything
    /// can answer without an index at all: if the identifier's store half is not
    /// its own incarnation the answer is `false`, and if it is, the question
    /// reduces to whether that position exists.
    ///
    /// # Errors
    ///
    /// Returns the adapter's error if the lookup fails.
    async fn contains_event_id(&self, id: EventId) -> Result<bool, Self::Error>;
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

// Gated on `memory` because the assertion below has to be *instantiated* at a
// concrete store to mean anything, and `MemoryEventStore` is the only one the
// contract crate has. An uninstantiated generic proves nothing: the projections
// stay opaque and the coercion is never checked.
#[cfg(all(test, feature = "memory"))]
mod tests {
    // Both flavours are in scope here, which the module documentation warns
    // against — but the warning is about *method-call* syntax, and nothing below
    // calls a method. These are type-level assertions only.
    use super::{EventStore, SendEventStore};

    /// ES-5. The two flavours' `Error` projections are the *same type*, not
    /// merely two types that happen to carry the same bounds.
    ///
    /// The assertion is the identity function's body: returning an
    /// `<S as SendEventStore>::Error` where an `<S as EventStore>::Error` is
    /// demanded type-checks only if the compiler can normalise both projections
    /// to one type. Today it can, via the blanket impl `trait_variant` emits
    /// (`type Error = <Self as SendEventStore>::Error`).
    ///
    /// This is deliberately *not* spelled as a bounds check —
    /// `fn f<S: SendEventStore>() where S::Error: core::error::Error` would pass
    /// trivially, since both traits state the same bound and the compiler would
    /// simply believe it. Equality is the property with content: it is what
    /// forbids ES-5's "Rejects" case, abandoning the derivation for two
    /// hand-written traits whose associated types are independent. Under that
    /// shape this fails with `error[E0308]: mismatched types`.
    fn error_projections_are_one_type<S: SendEventStore>(
        error: <S as SendEventStore>::Error,
    ) -> <S as EventStore>::Error {
        error
    }

    #[test]
    fn error_bound_is_identical_on_both_flavours() {
        // Naming the function is what instantiates the coercion check above;
        // the body is where the assertion lives, so there is nothing to run.
        let _ = error_projections_are_one_type::<crate::MemoryEventStore>;
    }
}
