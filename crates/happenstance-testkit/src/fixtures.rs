//! Small builders shared by the conformance rules, the reference [`Fixture`]
//! implementation, and the property-test generators.
//!
//! Public because adapter authors writing their own extra tests should express
//! them in the same vocabulary the suite uses.

use core::future::Future;
use std::sync::Arc;

use futures_core::Stream;
use happenstance_core::{
    AppendCondition, AppendError, Event, EventType, MemoryEventStore, Query, QueryItem,
    ReadOptions, SendEventStore, SequencePosition, SequencedEvent, Tags,
};

use crate::contract::{Capability, Fixture};

/// Builds an event of `event_type` with no tags.
///
/// # Panics
///
/// Panics if `event_type` is not a valid [`EventType`]. Test-only helper.
#[must_use]
pub fn event(event_type: &str) -> Event {
    Event::new(event_type, &b"{}"[..]).expect("valid event type")
}

/// Builds an event of `event_type` carrying `key:value` tags.
///
/// # Panics
///
/// Panics if `event_type` or any tag pair is invalid. Test-only helper.
#[must_use]
pub fn tagged_event(event_type: &str, tags: &[(&str, &str)]) -> Event {
    event(event_type).with_tags(self::tags(tags))
}

/// Builds an event with a distinct payload, for checking round-tripping.
///
/// # Panics
///
/// Panics if `event_type` is invalid. Test-only helper.
#[must_use]
pub fn event_with_payload(event_type: &str, payload: &'static [u8]) -> Event {
    Event::new(event_type, payload).expect("valid event type")
}

/// Builds an event whose payload is owned rather than `'static`.
///
/// [`event_with_payload`] takes `&'static [u8]` because every payload the suite
/// had until the value edges landed was a literal. A rule that has to build
/// 65,536 bytes cannot be: the buffer is computed, so it lives on the heap and
/// there is no literal to borrow from. `Bytes` is not named in the signature —
/// `Vec<u8>` converts into it — which keeps `bytes` out of this crate's manifest
/// for the sake of one parameter.
///
/// # Panics
///
/// Panics if `event_type` is invalid. Test-only helper.
#[must_use]
pub fn event_with_owned_payload(event_type: &str, payload: Vec<u8>) -> Event {
    Event::new(event_type, payload).expect("valid event type")
}

/// Builds a canonical tag set from `key:value` pairs.
///
/// # Panics
///
/// Panics if any pair is invalid. Test-only helper.
#[must_use]
pub fn tags(pairs: &[(&str, &str)]) -> Tags {
    Tags::from_pairs(pairs.iter().copied()).expect("valid tags")
}

/// Builds an [`EventType`].
///
/// # Panics
///
/// Panics if `value` is invalid. Test-only helper.
#[must_use]
pub fn event_type(value: &str) -> EventType {
    EventType::new(value).expect("valid event type")
}

/// Builds a single-item query constrained by type.
///
/// # Panics
///
/// Panics if `types` is empty or holds an invalid type. Test-only helper.
#[must_use]
pub fn query_of_types(types: &[&str]) -> Query {
    Query::from_item(QueryItem::of_types(types.iter().copied()).expect("valid types"))
        .expect("non-empty query")
}

/// Builds a single-item query constrained by tags.
///
/// # Panics
///
/// Panics if `pairs` is empty or invalid. Test-only helper.
#[must_use]
pub fn query_tagged(pairs: &[(&str, &str)]) -> Query {
    Query::from_item(QueryItem::tagged(tags(pairs)).expect("non-empty tags"))
        .expect("non-empty query")
}

/// Builds a single-item query constrained by both type and tags.
///
/// # Panics
///
/// Panics if either constraint is invalid. Test-only helper.
#[must_use]
pub fn query_of(types: &[&str], pairs: &[(&str, &str)]) -> Query {
    Query::from_item(QueryItem::new(types.iter().copied(), tags(pairs)).expect("valid query item"))
        .expect("non-empty query")
}

/// Builds a query item constrained by type alone.
///
/// The item builders exist because a **multi-item** query is where an adapter's
/// generated SQL is most likely to be wrong — an unparenthesised `WHERE a OR b
/// AND position >= ?`, one statement per item unioned without a final sort — and
/// a suite that could only build single-item queries conveniently was a suite
/// that mostly built single-item queries.
///
/// # Panics
///
/// Panics if `types` is empty or holds an invalid type. Test-only helper.
#[must_use]
pub fn item_of_types(types: &[&str]) -> QueryItem {
    QueryItem::of_types(types.iter().copied()).expect("valid types")
}

/// Builds a query item constrained by tags alone.
///
/// # Panics
///
/// Panics if `pairs` is empty or invalid. Test-only helper.
#[must_use]
pub fn item_tagged(pairs: &[(&str, &str)]) -> QueryItem {
    QueryItem::tagged(tags(pairs)).expect("non-empty tags")
}

/// Builds a query from several items, which combine with OR.
///
/// # Panics
///
/// Panics if `items` is empty. Test-only helper.
#[must_use]
pub fn query_of_items(items: impl IntoIterator<Item = QueryItem>) -> Query {
    Query::from_items(items).expect("non-empty query")
}

/// Builds an append condition over the whole log.
#[must_use]
pub fn condition(query: Query) -> AppendCondition {
    AppendCondition::new(query)
}

/// Builds an append condition restricted to events after `position`.
///
/// # Panics
///
/// Panics if `position` is zero. Test-only helper.
#[must_use]
pub fn condition_after(query: Query, position: u64) -> AppendCondition {
    AppendCondition::new(query).after(SequencePosition::new(position).expect("non-zero position"))
}

// -------------------------------------------------------------------------
// The reference fixture
// -------------------------------------------------------------------------

/// One handle onto a [`MemoryFixture`]'s store.
///
/// # Why a newtype rather than `impl EventStore for Arc<S>`
///
/// "Just add a blanket `impl<S: EventStore> EventStore for Arc<S>`" is the first
/// thing anyone tries, and it is very likely `error[E0119]` in
/// `happenstance-core`: `trait_variant` already emits a blanket
/// `impl<T: SendEventStore> EventStore for T`, and coherence cannot rule out
/// that some downstream `Arc<S>` implements `SendEventStore` — proving it does
/// not would need negative reasoning the trait solver does not have. So the
/// shared handle has to be a distinct type that delegates, and delegating by
/// hand is the price of the shape.
///
/// It implements [`SendEventStore`], not merely
/// [`EventStore`](happenstance_core::EventStore): [`MemoryEventStore`] is a
/// native, thread-safe store, and a handle that quietly weakened that would stop
/// exercising the flavour the reference implementation actually claims.
#[derive(Debug, Clone)]
pub struct MemoryHandle(Arc<MemoryEventStore>);

impl SendEventStore for MemoryHandle {
    type Error = <MemoryEventStore as SendEventStore>::Error;

    fn read(
        &self,
        query: &Query,
        options: ReadOptions,
    ) -> impl Stream<Item = Result<SequencedEvent, Self::Error>> + Send {
        self.0.read(query, options)
    }

    async fn append(
        &self,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<Self::Error>> {
        self.0.append(events, condition).await
    }
}

/// The reference [`Fixture`]: one [`MemoryEventStore`], any number of handles.
///
/// This is the oracle the suite is validated against, and it is also the worked
/// example an adapter author reads before writing their own fixture. Two things
/// in it are worth copying:
///
/// * `connect` is an `Arc` clone. A handle **owns a refcount** into the backing
///   store rather than borrowing a lifetime from the fixture, which is what lets
///   [`Fixture::Store`] be an ordinary associated type instead of a GAT. A
///   pool-backed fixture does the same thing with a pool; a `!Send` fixture does
///   it with an `Rc`.
/// * `REOPEN` is declined **with the real reason**. `MemoryFixture` is the
///   workspace's first fixture to decline anything, which is what makes the skip
///   machinery non-vacuous — until something declines a capability, "a skip is
///   reported" is a claim about code no fixture executes.
#[derive(Debug, Default)]
pub struct MemoryFixture(Arc<MemoryEventStore>);

impl MemoryFixture {
    /// Creates a fixture over a new, empty [`MemoryEventStore`].
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a fixture over an **existing** store, which is a way of
    /// deliberately breaking isolation.
    ///
    /// This is not a convenience, and it is not what an adapter's fixture should
    /// look like. It exists so that
    /// [`two_fixture_instances_observe_none_of_each_others_appends`](crate::rules::two_fixture_instances_observe_none_of_each_others_appends)
    /// can be shown to fail: handing every instance the same `Arc` reproduces,
    /// in memory, the file-backed adapter that points every fixture at one
    /// temporary path. A rule no implementation can fail is decorative, and this
    /// is the implementation that fails this one.
    ///
    /// Nothing else in the workspace should call it.
    #[must_use]
    pub fn sharing(store: Arc<MemoryEventStore>) -> Self {
        Self(store)
    }
}

impl Fixture for MemoryFixture {
    type Store = MemoryHandle;

    const SECOND_HANDLE: Capability = Capability::SUPPORTED;

    // The reason is the real one, and it has to be: a fixture that "reopened" by
    // doing nothing would pass `acknowledged_writes_survive_a_reopen`
    // vacuously, and one that reopened by dropping the `Vec` would fail it while
    // being perfectly conformant. Neither answer is worth having, so the honest
    // move is to decline and say why.
    const REOPEN: Capability = Capability::declined(
        "MemoryEventStore is a Vec behind an RwLock, so there is no durable \
         medium to reopen over: discarding process state is indistinguishable \
         from discarding the events",
    );

    fn connect(&self) -> impl Future<Output = Self::Store> {
        // Ready rather than `async move`: acquiring this handle is a refcount
        // bump, and pretending otherwise would hide that a real fixture's
        // `connect` does I/O and this one does not.
        core::future::ready(MemoryHandle(Arc::clone(&self.0)))
    }
}

// -------------------------------------------------------------------------
// Property-test generators
// -------------------------------------------------------------------------

// The `not(target_arch = "wasm32")` half of both `cfg`s below is not redundant
// with the feature, and leaving it off is a compile error rather than a
// stylistic slip. `proptest` is an optional dependency of the *non-wasm32*
// target table only, but a **feature is not target-scoped**: `--all-features`
// sets `feature = "proptest"` on every target, including the one where
// `dep:proptest` resolved to nothing. Without this second condition the module
// is compiled for wasm32 against a crate that is not in the graph, and the
// wasm32 feature-powerset step of `cargo xtask ci` is exactly what finds it.

/// The `proptest` that [`strategies`] speaks.
///
/// Re-exported because exporting a generator without exporting the trait it
/// returns is only half an export. `Strategy` is a `proptest` trait, so
/// [`strategies::any_tag`]'s return type names a type from *this* crate's
/// `proptest`. An adapter that adds its own dependency entry and resolves to a
/// semver-incompatible version gets an `impl Strategy` that does not satisfy its
/// own `proptest!` macro — two identically-spelled traits from two crates — and
/// there is no local edit that fixes it short of pinning. Reaching for
/// `happenstance_testkit::fixtures::proptest` instead makes the mismatch
/// unrepresentable.
#[cfg(all(feature = "proptest", not(target_arch = "wasm32")))]
#[cfg_attr(docsrs, doc(cfg(feature = "proptest")))]
pub use proptest;

/// `proptest` strategies over the contract's value types.
///
/// These live here, in a public module of the testkit, rather than in a test
/// binary, because the testkit's own claim about them is that an adapter pushing
/// query matching down into SQL is asserting the same laws about its `WHERE`
/// clause and should be able to reuse the generators. Generators private to an
/// integration test are reachable by nobody, which made that claim false.
///
/// Behind the off-by-default `proptest` feature: `proptest` does not build for
/// `wasm32-unknown-unknown`, and the mandatory wasm32 step of `cargo xtask ci`
/// must stay buildable. `--all-features` turns it on.
///
/// Write your `proptest!` blocks against the [`proptest`] re-export above rather
/// than a second dependency entry of your own; see its documentation for what
/// goes wrong otherwise.
#[cfg(all(feature = "proptest", not(target_arch = "wasm32")))]
#[cfg_attr(docsrs, doc(cfg(feature = "proptest")))]
pub mod strategies {
    use happenstance_core::{Event, EventType, Query, QueryItem, Tag, Tags};
    use proptest::prelude::{Strategy, prop, prop_oneof};

    /// Generates a tag from a **five-symbol alphabet**.
    ///
    /// The alphabet is the point, and it is why this is exported rather than
    /// described. Over `"a".."e"` a generated `Tags` of length up to six has
    /// collisions and duplicates in almost every case; over anything wider they
    /// are vanishingly unlikely, and a merge-scan that is wrong exactly at a
    /// duplicate boundary is never handed one. An adapter author writing their
    /// own generator will reach for something realistic and will therefore never
    /// generate the inputs that break them.
    ///
    /// # Panics
    ///
    /// Panics during generation if a sampled value is not a valid [`Tag`].
    /// Test-only helper; the alphabet is chosen so it cannot happen.
    ///
    /// # Examples
    ///
    /// ```
    /// use happenstance_testkit::fixtures::strategies;
    ///
    /// // Constructing it is the check: if these were not public, this would not
    /// // compile.
    /// let _tag = strategies::any_tag();
    /// let _tags = strategies::any_tags();
    /// let _event_type = strategies::any_event_type();
    /// ```
    pub fn any_tag() -> impl Strategy<Value = Tag> {
        prop::sample::select(vec!["a", "b", "c", "d", "e"])
            .prop_map(|value| Tag::new(value).expect("valid tag"))
    }

    /// Generates a canonical [`Tags`] of up to six tags from [`any_tag`].
    ///
    /// Includes the empty set, which is a legitimate value and the one an
    /// adapter's `WHERE` clause is most likely to mishandle.
    ///
    /// # Panics
    ///
    /// Panics during generation if [`any_tag`] does. Test-only helper.
    pub fn any_tags() -> impl Strategy<Value = Tags> {
        prop::collection::vec(any_tag(), 0..6).prop_map(|tags| tags.into_iter().collect())
    }

    /// Generates an [`EventType`] from a three-symbol alphabet, for the same
    /// reason [`any_tag`]'s is five.
    ///
    /// # Panics
    ///
    /// Panics during generation if a sampled value is not a valid [`EventType`].
    /// Test-only helper.
    pub fn any_event_type() -> impl Strategy<Value = EventType> {
        prop::sample::select(vec!["A", "B", "C"])
            .prop_map(|value| EventType::new(value).expect("valid event type"))
    }

    /// Generates a [`QueryItem`] over the alphabets above.
    ///
    /// `prop_filter_map` rather than a generator steered away from the empty
    /// case: an item constraining neither types nor tags is rejected at
    /// construction and simply is not representable, but *each* of the two empty
    /// sets on its own is legal and is precisely the input an adapter's `WHERE`
    /// clause mishandles. Filtering the one illegal corner keeps both legal ones.
    pub fn any_query_item() -> impl Strategy<Value = QueryItem> {
        (prop::collection::vec(any_event_type(), 0..3), any_tags())
            .prop_filter_map("an item must constrain types or tags", |(types, tags)| {
                QueryItem::new(types, tags).ok()
            })
    }

    /// Generates a [`Query`], weighted towards `Items` — `All` has one behaviour
    /// and needs one sample, not a third of them.
    ///
    /// # Panics
    ///
    /// Panics during generation if a generated item list is empty, which the
    /// range below makes unreachable. Test-only helper.
    pub fn any_query() -> impl Strategy<Value = Query> {
        prop_oneof![
            1 => prop::strategy::Just(Query::all()),
            4 => prop::collection::vec(any_query_item(), 1..4)
                .prop_map(|items| Query::from_items(items).expect("at least one item")),
        ]
    }

    /// Generates an [`Event`]: a type, a tag set, a small payload, and metadata
    /// half the time.
    ///
    /// Two value edges are **deliberately excluded**, and both are phase 3's
    /// stage 6 rather than an oversight: the empty payload, and `metadata:
    /// Some(<empty>)` as distinct from `None`. Whether those two pairs are
    /// distinguishable through a store is not settled, and a generator that
    /// produced them would be asserting an answer through
    /// [`model`](crate::model)'s round-trip comparison before the clause that
    /// owns the question was written.
    ///
    /// # Panics
    ///
    /// Panics during generation if a sampled type is invalid. Test-only helper.
    pub fn any_event() -> impl Strategy<Value = Event> {
        (
            any_event_type(),
            any_tags(),
            prop::sample::select(vec![&b"{}"[..], &b"[1,2]"[..], &b"payload"[..]]),
            prop::option::of(prop::sample::select(vec![
                &b"trace-id"[..],
                &b"causation"[..],
            ])),
        )
            .prop_map(|(event_type, tags, data, metadata)| {
                // `.as_str().to_owned()` rather than handing over the
                // `EventType` we already hold, and it is not a slip.
                // `Event::new` is bounded `impl TryInto<EventType, Error =
                // InvalidEventType>`, which an already-built `EventType` does
                // **not** satisfy: its conversion is `Infallible`, and that is a
                // different associated type. `QueryItem::new` takes the same
                // argument through `InvalidQuery: From<T::Error>` and therefore
                // accepts both; `Event::new` accepts only the fallible half.
                let event = Event::new(event_type.as_str().to_owned(), data)
                    .expect("an EventType is already valid")
                    .with_tags(tags);
                match metadata {
                    Some(metadata) => event.with_metadata(metadata),
                    None => event,
                }
            })
    }
}
