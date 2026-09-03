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
//!  --> src/lib.rs:6:19
//!   |
//! 6 |     let _ = store.read(&query, options);
//!   |                   ^^^^ multiple `read` found
//!   |
//!   = note: candidate #1 is defined in an impl of the trait `SendEventStore` for the type `MemoryEventStore`
//!   = note: candidate #2 is defined in an impl of the trait `EventStore` for the type `TraitVariantBlanketType`
//! help: disambiguate the method for candidate #2
//!   |
//! 6 -     let _ = store.read(&query, options);
//! 6 +     let _ = EventStore::read(&store, &query, options);
//!   |
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

// =====================================================================
// ES-13's regression pin, and the guard that keeps it where cargo looks
// =====================================================================

/// ES-13's regression pin: the query must outlive the stream.
///
/// A carrier, not an API — the two fences below are the artefact, and a doc
/// comment needs an item to sit on. It sits in `src/` because rustdoc collects
/// doctests from the **lib target only**. It lived in
/// `crates/happenstance-core/tests/frozen_signatures.rs` from phase 4 until
/// F1-04, where nothing ever compiled it; the guard below is what stops it
/// drifting back there.
///
/// ES-13 freezes [`EventStore::read`] on `&Query` and names taking the query
/// **by value** as the wrong fix. What the reference costs is that the opaque
/// return type captures the query's lifetime, so the query has to be owned by
/// something that outlives the stream: a *parameter* is, and a local never can
/// be. The diagnostic depends on how the return type is spelled, which is why
/// ES-13 states the defect rather than pinning a code:
///
/// | Arrangement | Diagnostic |
/// |---|---|
/// | `store.read(&Query::all(), ..)` bound to a `let` | `error[E0716]` |
/// | returned, bare `-> impl Stream<..>` | `error[E0597]` |
/// | returned, `+ '_` or `+ 'a` | `error[E0515]` |
/// | returned inside a struct, bare | `error[E0597]` |
///
/// All four are the same defect reported from two ends. Without a lifetime bound
/// the compiler reasons from the *borrow* — `&query` must outlive the return and
/// `query` drops at the end of the function. With one, the opaque type is
/// required to live for `'a`, so it reasons from the *value* instead. The cause
/// either way is that the opaque type captures the query's lifetime.
///
/// # The control
///
/// Paired per RS-62-1, and it carries the half of the check the refusal cannot:
/// it is the one thing here that goes red when `read`, `Query` or `ReadOptions`
/// is renamed or moved behind a feature, which a lone `compile_fail` fence
/// reports as passing.
///
/// ```
/// use futures_core::Stream;
/// use happenstance_core::{EventStore, Query, ReadOptions, SequencedEvent};
///
/// // The query comes from the caller, so it outlives the returned stream.
/// fn replay<'a, S: EventStore>(
///     store: &'a S,
///     query: &'a Query,
/// ) -> impl Stream<Item = Result<SequencedEvent, S::Error>> + 'a {
///     store.read(query, ReadOptions::new())
/// }
/// # fn main() {}
/// ```
///
/// # The refusal
///
/// The same function over a **local** query. The fence carries no error code:
/// the table above has four of them, and rustdoc 1.97.1 compares an annotated
/// code, finds no match, and reports the fence as passing anyway (RS-62-1).
///
/// ```compile_fail
/// use futures_core::Stream;
/// use happenstance_core::{EventStore, Query, ReadOptions, SequencedEvent};
///
/// fn escapes<S: EventStore>(store: &S) -> impl Stream<Item = Result<SequencedEvent, S::Error>> {
///     let query = Query::all();
///     store.read(&query, ReadOptions::new())
/// }
/// # fn main() {}
/// ```
#[doc(hidden)]
pub fn es_13_the_query_must_outlive_the_stream() {}

/// A doc fence inside an integration test target is never handed to a compiler.
///
/// rustdoc collects doctests from the **lib target only**, so a fence in a
/// `tests/` file is compiled as prose: it cannot fail, and it cannot be told
/// apart from one that has stopped being Rust. Measured rather than reasoned
/// about — `crates/happenstance-core/tests/frozen_signatures.rs` carried a
/// `compile_fail` fence claiming to pin ES-13 from phase 4 until F1-04, and with
/// that fence's body replaced by a line of English both
/// `cargo test -p happenstance-core --doc` and
/// `cargo test -p happenstance-core --test frozen_signatures` still exit `0`.
///
/// So the pin moved to `es_13_the_query_must_outlive_the_stream` above, and this
/// is the standing guard on the move: it fails if that file opens a doc fence
/// again. It reads the file rather than reasoning about it, the way
/// `mod module_doc` below reads this one.
///
/// ```
/// // Anchored on the package root rather than on relative depth, per RS-62-5:
/// // the path a packaged `.crate` resolves is then the path checked here.
/// const ARTEFACT: &str = include_str!(concat!(
///     env!("CARGO_MANIFEST_DIR"),
///     "/tests/frozen_signatures.rs"
/// ));
///
/// // Spelled with escapes on purpose. A literal fence inside a doctest inside a
/// // doc comment is three nested parsers deep, and the needle is the one thing
/// // here that must not be guessed at by any of them.
/// const FENCE: &str = "\u{60}\u{60}\u{60}";
///
/// fn main() {
///     let opened: Vec<usize> = ARTEFACT
///         .lines()
///         .enumerate()
///         .filter(|(_, line)| {
///             let text = line.trim_start();
///             text.strip_prefix("//!")
///                 .or_else(|| text.strip_prefix("///"))
///                 .is_some_and(|body| body.trim_start().starts_with(FENCE))
///         })
///         .map(|(index, _)| index + 1)
///         .collect();
///
///     assert!(
///         opened.is_empty(),
///         "tests/frozen_signatures.rs opens a doc fence at line(s) {opened:?}. \
///          cargo hands a doc fence to a compiler from the lib target only, so \
///          whatever that one claims to pin is decorative. Move it beside \
///          `es_13_the_query_must_outlive_the_stream` in src/store.rs."
///     );
/// }
/// ```
#[doc(hidden)]
pub fn a_doc_fence_in_an_integration_test_target_is_never_compiled() {}

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

/// The module documentation, read as a deliverable rather than as decoration.
///
/// Everything here reads this file's own `//!` comment. That is unusual and it is
/// deliberate: the section `## Import one flavour, not both` exists so that an
/// adapter author whose build just failed with `error[E0034]` finds rustc's own
/// words in the file they already have open, and every part of that claim — the
/// transcript being verbatim, the diagnosis surviving without the caret, the fix
/// arriving before the pointer — is a property of the *text*, invisible to every
/// other instrument this workspace owns. A doc comment nothing checks is exactly
/// the rot the section was rewritten to repair.
///
/// It is `cfg(test)` with no feature gate, unlike the type-level module above it:
/// the text is there in every feature configuration, so nothing here may be
/// contingent on one.
#[cfg(test)]
mod module_doc {
    #![allow(clippy::unwrap_used, reason = "test code, per the house style")]

    use alloc::string::String;
    use alloc::vec::Vec;

    /// This file's own source. The module doc is the deliverable, so it is
    /// asserted rather than trusted.
    const SOURCE: &str = include_str!("store.rs");

    /// The third of the four heading entries — the one every change lands inside.
    const SECTION: &str = "## Import one flavour, not both";

    /// The transcript, pinned character for character.
    ///
    /// **The one place in this workspace where pinning a paragraph is right.**
    /// Elsewhere a paragraph pin turns ordinary rewording into a build failure;
    /// here rewording *is* the defect, because the block's whole job is to be the
    /// string a stuck reader searches for and pastes into a diff against their own
    /// terminal. Reproduced at the pinned toolchain — `rustc 1.97.1 (8bab26f4f
    /// 2026-07-14)` — by compiling a deliberate double-import; the run's full
    /// stderr, the command and the `rustc --version` are recorded in this story's
    /// folder under `.bklg/docs-that-teach/reach-and-adapter-path/`.
    ///
    /// Candidate #1's `help:` hunk is the one thing dropped, which is the
    /// signed-off design's yield rule (3), exercised at design time. Nothing
    /// further may be cut, and the two `= note:` lines and the internal type name
    /// never yield at all.
    const TRANSCRIPT: &str = "\
error[E0034]: multiple applicable items in scope
 --> src/lib.rs:6:19
  |
6 |     let _ = store.read(&query, options);
  |                   ^^^^ multiple `read` found
  |
  = note: candidate #1 is defined in an impl of the trait `SendEventStore` for the type `MemoryEventStore`
  = note: candidate #2 is defined in an impl of the trait `EventStore` for the type `TraitVariantBlanketType`
help: disambiguate the method for candidate #2
  |
6 -     let _ = store.read(&query, options);
6 +     let _ = EventStore::read(&store, &query, options);
  |";

    /// The section's source-line cap, raised from 30 to 36 at sign-off when copy
    /// fidelity was chosen over the density budget (finding F3). The measured
    /// projection was ~34.
    const SECTION_LINE_CAP: usize = 36;

    /// The source above this file's first test module.
    ///
    /// Without it every assertion about the production text would be satisfiable
    /// by the tests' own pinned strings, since [`SOURCE`] includes this module too.
    fn production_source() -> &'static str {
        SOURCE.split("#[cfg(all(test").next().unwrap()
    }

    /// The `//!` comment at the top of the file, one entry per line, with the
    /// comment marker and its single following space removed.
    ///
    /// A bare `//!` becomes an empty line, which is what makes the paragraph
    /// splitting below work. `str::lines` drops a trailing carriage return, so this
    /// is correct whichever line ending the working tree happens to carry.
    fn doc_lines() -> Vec<&'static str> {
        SOURCE
            .lines()
            .take_while(|line| line.starts_with("//!"))
            .map(|line| {
                if line.trim_end() == "//!" {
                    ""
                } else {
                    line.strip_prefix("//! ")
                        .expect("every doc line is `//!` or `//! ` and content")
                }
            })
            .collect()
    }

    /// [`doc_lines`] with every fenced block removed, so a heading count or a
    /// markup check cannot be fooled by a line inside a transcript.
    fn doc_lines_outside_fences() -> Vec<&'static str> {
        let mut inside = false;
        let mut kept = Vec::new();
        for line in doc_lines() {
            if line.starts_with("```") {
                inside = !inside;
                continue;
            }
            if !inside {
                kept.push(line);
            }
        }
        kept
    }

    /// The body of `## Import one flavour, not both` — every line after its heading
    /// and up to the next `##`, with the blank lines that bracket it trimmed.
    ///
    /// This is the unit the density budget is denominated in, and it is the same
    /// count the design measured: 13 source lines before this story, ~34 after it.
    fn section_body() -> Vec<&'static str> {
        let lines = doc_lines();
        let start = lines
            .iter()
            .position(|line| *line == SECTION)
            .expect("the heading ladder still carries `## Import one flavour, not both`");
        let rest = &lines[start + 1..];
        let end = rest
            .iter()
            .position(|line| line.starts_with("## "))
            .unwrap_or(rest.len());
        let mut body = &rest[..end];
        while body.first() == Some(&"") {
            body = &body[1..];
        }
        while body.last() == Some(&"") {
            body = &body[..body.len() - 1];
        }
        body.to_vec()
    }

    /// The lines inside the section's `text` fence, fence markers excluded.
    fn fence() -> Vec<&'static str> {
        let body = section_body();
        let open = body
            .iter()
            .position(|line| line.starts_with("```"))
            .expect("the section carries the diagnostic in a fence");
        let rest = &body[open + 1..];
        let close = rest
            .iter()
            .position(|line| line.starts_with("```"))
            .expect("the fence is closed");
        rest[..close].to_vec()
    }

    /// The blank-line-separated paragraphs after the fence closes — elements (c),
    /// (d), (e) and (f) of the signed-off composition, in source order.
    fn paragraphs_after_fence() -> Vec<Vec<&'static str>> {
        let body = section_body();
        let close = body
            .iter()
            .rposition(|line| line.starts_with("```"))
            .expect("the fence is closed");
        let mut paragraphs = Vec::new();
        let mut current: Vec<&'static str> = Vec::new();
        for line in &body[close + 1..] {
            if line.is_empty() {
                if !current.is_empty() {
                    paragraphs.push(core::mem::take(&mut current));
                }
            } else {
                current.push(line);
            }
        }
        if !current.is_empty() {
            paragraphs.push(current);
        }
        paragraphs
    }

    /// Soft-wrapped lines rejoined with single spaces, so a pinned subject string
    /// is not broken by where a line happens to wrap.
    fn joined(lines: &[&str]) -> String {
        let mut out = String::new();
        for (index, line) in lines.iter().enumerate() {
            if index > 0 {
                out.push(' ');
            }
            out.push_str(line);
        }
        out
    }

    /// Whether `prose` names the weaker flavour in its own right rather than only
    /// as the tail of the stronger one — `SendEventStore` contains `EventStore`, so
    /// a naive `contains` cannot tell "both traits are named" from "one is".
    fn names_both_flavours(prose: &str) -> bool {
        prose.contains("SendEventStore")
            && prose.replace("SendEventStore", "").contains("EventStore")
    }

    // ---- AC-001: rustc's own transcript, searchable and verbatim -------------

    /// The block is rustc's output, not a transcription of one.
    ///
    /// Equality rather than a handful of `contains` assertions, because every
    /// weaker form passes on a block somebody has tidied. The reader's move is to
    /// paste their own terminal beside this one and look for a difference, and any
    /// edit at all turns that into a false positive.
    #[test]
    fn the_fence_is_rustcs_own_e0034_output() {
        let expected: Vec<&str> = TRANSCRIPT.lines().collect();
        assert_eq!(
            fence(),
            expected,
            "the `text` fence is no longer the stderr of the recorded reproduction \
             run. It is not prose and it is not edited to fit: reproduce it again at \
             the pinned toolchain and paste what that run emitted"
        );
    }

    /// The two strings the density budget's yield order says never yield.
    ///
    /// Separate from the equality above on purpose. If a later change re-pins the
    /// transcript for a new toolchain, that test moves with it and this one does
    /// not: the `= note:` candidate lines and the internal type name are what a
    /// stuck reader actually searches for, and they survive every re-pin.
    #[test]
    fn the_two_note_lines_and_the_internal_type_name_never_yield() {
        let fence = fence();
        let notes = fence
            .iter()
            .filter(|line| line.trim_start().starts_with("= note:"))
            .count();
        assert_eq!(
            notes, 2,
            "both `= note:` candidate lines are required content: they name the two \
             traits and the type each `read` was found on, and they are the lines a \
             reader pastes into a search box"
        );
        assert!(
            fence
                .iter()
                .any(|line| line.contains("TraitVariantBlanketType")),
            "the internal type name rustc prints in candidate #2 is the string this \
             workspace explains nowhere else; dropping it puts the reader's search \
             back where it started"
        );
    }

    /// Nothing inside the fence but compiler characters.
    ///
    /// A `rust` fence around a diagnostic would buy a green tick that means
    /// nothing — the `ignore`-fence anti-pattern wearing a compiler — and rendered
    /// markup inside the block would survive selection and break the reader's diff.
    #[test]
    fn the_fence_is_uncompiled_plain_text_a_reader_can_paste() {
        let body = section_body();
        assert!(
            body.contains(&"```text"),
            "the diagnostic sits in a `text` fence: nothing compiles a compiler \
             transcript, and a `rust` fence would claim otherwise"
        );
        assert!(
            !body
                .iter()
                .any(|line| line.starts_with("```rust") || line.starts_with("```ignore")),
            "a `rust` or `ignore` fence around the diagnostic is the one tier this \
             story's specification names so that nobody adds it"
        );
        for line in fence() {
            assert!(
                !line.contains("](") && !line.contains('<') && !line.contains("**"),
                "rendered-only markup inside the fence: {line:?}. The block has to \
                 diff clean against a terminal, so it may carry only what rustc \
                 printed"
            );
        }
    }

    // ---- AC-002: the ambiguity named in words, never spatially alone ---------

    /// `^^^^ multiple` is spatial. A screen reader linearises it into nothing and
    /// so does a plain-text search hit, so the diagnosis is restated in words
    /// immediately after the block.
    #[test]
    fn the_ambiguity_is_named_in_words_and_not_only_by_the_caret() {
        let named = joined(&paragraphs_after_fence()[0]);
        assert!(
            named.contains("store.read("),
            "element (c) must name the method whose call is ambiguous: {named}"
        );
        assert!(
            names_both_flavours(&named),
            "element (c) must name both flavours, because the caret line names \
             neither: {named}"
        );
        assert!(
            named.contains("in scope"),
            "element (c) must state the cause — both being in scope — and not merely \
             that something is ambiguous: {named}"
        );
    }

    /// Both trait names appear in the section's prose, not only inside the fence.
    ///
    /// The falsification of the design's anti-pattern 15, executed: delete the
    /// transcript and the diagnosis has to survive.
    #[test]
    fn the_diagnosis_survives_the_fence_being_deleted() {
        let fence = fence();
        let outside: Vec<&str> = section_body()
            .into_iter()
            .filter(|line| !fence.contains(line))
            .collect();
        assert!(
            names_both_flavours(&joined(&outside)),
            "with the transcript removed the remaining prose names one flavour or \
             neither, so colour, highlighting and spatial position are carrying the \
             diagnosis alone"
        );
    }

    // ---- AC-003: unstuck without a hop, the fix ahead of the pointer ---------

    /// The signed-off composition, in source order, with nothing between (c) and (d).
    ///
    /// Four paragraphs and exactly four: separating the diagnosis from its fix is
    /// what turns a self-sufficient page into one that requires a hop, and an
    /// inserted paragraph is how that happens by accident.
    #[test]
    fn the_section_composes_in_the_designs_binding_order() {
        let paragraphs = paragraphs_after_fence();
        assert_eq!(
            paragraphs.len(),
            4,
            "the fence is followed by exactly elements (c) the plain-words \
             diagnosis, (d) the in-place fix, (e) the narrow limit and (f) the \
             pointer. Nothing may be inserted between (c) and (d)"
        );
        assert!(
            joined(&paragraphs[1]).contains("SendEventStore::read(&store, &query, options)"),
            "element (d), the fully-qualified escape hatch, must be the paragraph \
             immediately after the plain-words diagnosis"
        );
        assert!(
            joined(&paragraphs[2]).contains("compile_fail,E0034"),
            "element (e), the narrow limit, follows the fix"
        );
        assert!(
            joined(&paragraphs[3]).contains("docs/adapter-reading-order.md"),
            "element (f), the pointer, is last"
        );
    }

    /// A reader who never follows the pointer is still correctly unstuck.
    ///
    /// Executed rather than asserted: element (f) is deleted here and the rest of
    /// the section still has to resolve the error on its own.
    #[test]
    fn deleting_the_pointer_leaves_the_reader_unstuck() {
        let paragraphs = paragraphs_after_fence();
        let kept: Vec<&str> = paragraphs[..paragraphs.len() - 1]
            .iter()
            .flatten()
            .copied()
            .collect();
        let remaining = joined(&kept);
        assert!(
            !remaining.contains("docs/adapter-reading-order.md"),
            "the pointer is the last paragraph and nothing else"
        );
        assert!(
            remaining.contains("Import only the one"),
            "with the reasoning account gone the import rule must still be here"
        );
        assert!(
            remaining.contains("SendEventStore::read(&store, &query, options)"),
            "with the reasoning account gone the escape hatch must still be here: \
             deleting the destination may not leave a reader stuck on this page"
        );
    }

    // ---- AC-004: the narrow limit, not the broad one -------------------------

    /// The page states what is checked and what is not, and the true half is the
    /// one that is easy to lose.
    #[test]
    fn the_limit_stated_is_the_narrow_one() {
        let limit = joined(&paragraphs_after_fence()[2]);
        assert!(
            limit.contains("`text` fence") && limit.contains("nothing compiles it"),
            "element (e) must say the fence is uncompiled: {limit}"
        );
        assert!(
            limit.contains("compile_fail,E0034")
                && limit.contains("standards/rust/20-two-flavour-ports.md")
                && limit.contains("standards/rust/00-prime-directives.md"),
            "element (e) must name the compiled negatives that really do assert the \
             error code, or it discards a guard that exists: {limit}"
        );
        assert!(
            limit.contains("asserted by nothing"),
            "element (e) must say which half is unasserted — the notes' wording and \
             the internal type name: {limit}"
        );
    }

    /// The broad falsehood, named so it cannot creep back as a simplification.
    #[test]
    fn the_page_never_claims_that_nothing_checks_this() {
        assert!(
            !joined(&doc_lines()).contains("nothing checks this"),
            "\"nothing checks this\" is wrong in the direction that discards a real \
             guard: the error code is asserted by two compiled negatives in the \
             constitution"
        );
    }

    // ---- AC-005: one recessive, correctly-formed pointer ---------------------

    /// Exactly one hop, last, with no heading of its own.
    #[test]
    fn one_recessive_pointer_closes_the_section() {
        let mentions = section_body()
            .iter()
            .filter(|line| line.contains("docs/adapter-reading-order.md"))
            .count();
        assert_eq!(
            mentions, 1,
            "exactly one hop leaves the section. A second pointer is the moment the \
             page starts requiring a reader to leave it"
        );
        let paragraphs = paragraphs_after_fence();
        let pointer = joined(paragraphs.last().unwrap());
        assert!(
            pointer.contains("the adapter reading order"),
            "the link text is a self-describing noun phrase naming the destination, \
             because it is all a screen reader or an extracted link list gives the \
             reader: {pointer}"
        );
        assert!(
            !pointer.contains("http://") && !pointer.contains("https://"),
            "a bare URL is guarded by nothing and is forbidden to this project \
             outright: {pointer}"
        );
        assert!(
            !pointer.contains('['),
            "never an intra-doc link: the destination is a narrative page, not an \
             item, and no link form resolves in all three of the gate's rustdoc \
             builds. The named-but-unlinked cross-reference is the rung this pointer \
             took: {pointer}"
        );
    }

    /// The pointer is carried by order and by having no heading, and by nothing else.
    #[test]
    fn the_pointer_is_recessive_and_the_section_grows_no_appendix() {
        assert!(
            !section_body().iter().any(|line| line.starts_with('#')),
            "the pointer takes no heading of its own; the moment it stops being \
             recessive, a reader starts treating the hop as required"
        );
        let doc = joined(&doc_lines());
        for block in ["See also", "Next steps", "Further reading"] {
            assert!(
                !doc.contains(block),
                "a {block:?} block is a second navigation surface bolted onto a page \
                 that already reads in order"
            );
        }
    }

    // ---- AC-008: the surface obeys the signed-off design ---------------------

    /// Four heading entries, no fifth, no level skipped to reach one.
    #[test]
    fn the_heading_ladder_stays_at_four_entries() {
        let outside = doc_lines_outside_fences();
        let top = outside.iter().filter(|line| line.starts_with("# ")).count();
        let second = outside
            .iter()
            .filter(|line| line.starts_with("## "))
            .count();
        let deeper = outside
            .iter()
            .filter(|line| line.starts_with("### "))
            .count();
        assert_eq!(
            (top, second, deeper),
            (1, 3, 0),
            "the ladder is `# Why there are two traits` and three `##` entries. A \
             fifth entry, or a level skipped to reach one, is a page reorganising \
             itself around a single insertion"
        );
    }

    /// No `##` whose entire body is one sentence — a heading that earns its place
    /// has something under it.
    #[test]
    fn no_heading_carries_a_single_sentence_body() {
        let outside = doc_lines_outside_fences();
        let heads: Vec<usize> = outside
            .iter()
            .enumerate()
            .filter(|(_, line)| line.starts_with("## "))
            .map(|(index, _)| index)
            .collect();
        for (position, start) in heads.iter().enumerate() {
            let end = heads.get(position + 1).copied().unwrap_or(outside.len());
            let filled = outside[start + 1..end]
                .iter()
                .filter(|line| !line.is_empty())
                .count();
            assert!(
                filled >= 3,
                "`{}` carries {filled} lines of body. A `##` whose whole body is one \
                 sentence is a heading doing the work of a sentence",
                outside[*start]
            );
        }
    }

    /// The density budget, with the real number the design measured.
    #[test]
    fn the_section_stays_inside_its_density_budget() {
        let measured = section_body().len();
        assert!(
            measured <= SECTION_LINE_CAP,
            "the section measures {measured} source lines against a cap of \
             {SECTION_LINE_CAP}. The yield order is (1) the limit sentence \
             compresses to one clause, (2) the connective prose around the pointer, \
             (3) the fence's non-`= note:` context lines — and the two `= note:` \
             lines and the internal type name never yield"
        );
    }

    /// Everything added is persistent chrome: nothing folds, tabs or hides.
    #[test]
    fn every_element_is_persistent_chrome() {
        let doc = joined(&doc_lines());
        for widget in ["<details", "<summary", "<div", "<table", "<span", "style="] {
            assert!(
                !doc.contains(widget),
                "{widget:?} in the module doc. This project installs no revealed and \
                 no opened-on-demand affordance of its own, and raw HTML also \
                 defeats rustdoc's own theme contrast"
            );
        }
    }

    // ---- AC-009: the search keys, by a stated rule ---------------------------

    /// The keys are exactly the two strings rustc printed, and each one is still
    /// visible in the transcript on this page.
    ///
    /// The second half is the cheap check the design recorded as *not existing*: an
    /// alias whose string no longer appears in the block it was taken from has gone
    /// stale, and nothing else in the workspace would notice.
    #[test]
    fn every_search_key_is_a_string_rustc_printed_on_this_page() {
        let attributes: Vec<&str> = production_source()
            .lines()
            .filter(|line| line.starts_with("#[doc(alias"))
            .collect();
        assert_eq!(
            attributes,
            [
                "#[doc(alias = \"E0034\")]",
                "#[doc(alias = \"TraitVariantBlanketType\")]"
            ],
            "the rule admits only strings rustc, the specification or a recorded \
             reader question actually emits — not synonyms, and not the item's own \
             name"
        );
        let fence = joined(&fence());
        for key in ["E0034", "TraitVariantBlanketType"] {
            assert!(
                fence.contains(key),
                "the search key {key:?} no longer appears in the transcript it was \
                 taken from, so the key and the page have drifted apart"
            );
            assert!(
                !"SendEventStore".contains(key),
                "a key may not be the item's own name or a substring of it: {key}"
            );
        }
    }

    /// The workspace lockfile, so a positional claim can be tied to a version.
    const LOCKFILE: &str = include_str!("../../../Cargo.lock");

    /// The `trait-variant` release whose expansion was actually read.
    ///
    /// Bumping this constant is not a chore. It is the signal to re-open
    /// `variant.rs` and confirm the copying below still happens, because nothing
    /// else in this repository can see it.
    const TRAIT_VARIANT_VERIFIED: &str = "0.1.3";

    /// The version `Cargo.lock` resolves `trait-variant` to.
    ///
    /// Parsed rather than pinned in the manifest: `trait-variant = "0.1.3"` is a
    /// caret requirement, so `0.1.4` would resolve without the manifest changing.
    /// The lockfile is what the gate builds against (`--locked`), so it is the
    /// only place the *resolved* version can be read.
    fn resolved_trait_variant() -> &'static str {
        LOCKFILE
            .split("name = \"trait-variant\"")
            .nth(1)
            .and_then(|rest| rest.split("version = \"").nth(1))
            .and_then(|rest| rest.split('"').next())
            .expect("the workspace lockfile resolves the derivation's crate")
    }

    /// The attributes sit where `trait_variant` copies them onto the derived flavour.
    ///
    /// Two attributes, not the three the design projected, and the reason is
    /// mechanical rather than editorial: `SendEventStore` is *derived*, and the
    /// expansion rebuilds it with `..tr.clone()`, which copies the trait's
    /// attributes verbatim. There is no second item to write an attribute on, and
    /// two written here are four entries in the search index — a superset of the
    /// three the design asked for. Position is the whole mechanism, so position is
    /// what is asserted.
    ///
    /// **Position is a proxy, and the second assertion is what stops it being a
    /// silent one.** That the attributes sit above the derivation is necessary and
    /// not sufficient: the copying itself is upstream behaviour, and a
    /// `trait-variant` release that stopped doing it would leave this test green
    /// and `SendEventStore` carrying no search key at all. Nothing in this
    /// repository can observe the expansion, so the version is asserted instead —
    /// the gate builds `--locked`, so the resolved version cannot move without
    /// someone changing it deliberately, and changing it is the moment to re-read
    /// `variant.rs`. (The other half of the mechanism has a standing guard already:
    /// `SendEventStore` has no doc comment of its own, so if attributes stopped
    /// being copied, `missing_docs` — `warn` in the workspace lints and `-D
    /// warnings` in the gate — would fail the build.)
    #[test]
    fn the_search_keys_sit_where_trait_variant_copies_them_to_both_flavours() {
        let lines: Vec<&str> = production_source().lines().collect();
        let make = lines
            .iter()
            .position(|line| line.trim_end() == "#[trait_variant::make(SendEventStore: Send)]")
            .expect("the derivation that causes the collision this page documents");
        assert_eq!(lines[make - 2].trim_end(), "#[doc(alias = \"E0034\")]");
        assert_eq!(
            lines[make - 1].trim_end(),
            "#[doc(alias = \"TraitVariantBlanketType\")]"
        );
        assert_eq!(
            lines[make + 1].trim_end(),
            "pub trait EventStore {",
            "the attributes must sit on the trait the derivation copies from, or the \
             `Send` flavour carries no search key at all"
        );
        assert_eq!(
            resolved_trait_variant(),
            TRAIT_VARIANT_VERIFIED,
            "the two attributes above reach `SendEventStore` only because \
             trait-variant {TRAIT_VARIANT_VERIFIED} rebuilds the derived trait with \
             `..tr.clone()` (`trait-variant-{TRAIT_VARIANT_VERIFIED}/src/variant.rs:115-123`), \
             copying the base trait's attributes onto it. The position asserted above \
             cannot see that, so the version stands in for it: read the new \
             `variant.rs`, confirm the attributes are still copied, then move this \
             constant"
        );
    }
}
