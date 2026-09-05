//! Queries: how callers select the slice of the log they care about.

use alloc::boxed::Box;
use alloc::vec::Vec;

use crate::error::InvalidQuery;
use crate::event::{EventType, SequencePosition};
use crate::tag::Tags;

/// One clause of a [`Query`].
///
/// An event matches an item when **both** hold:
///
/// * its type is one of `types` — OR semantics, and an empty `types` matches any
///   type;
/// * its tags contain **all** of `tags` — AND semantics, and an empty `tags`
///   matches any tags.
///
/// At least one of the two must be non-empty; an item constraining neither would
/// match everything, which is [`Query::all`]'s job.
///
/// # Examples
///
/// ```
/// use happenstance_core::{QueryItem, Tags};
///
/// // "Any StudentSubscribed or StudentUnsubscribed event for course c1."
/// let item = QueryItem::new(
///     ["StudentSubscribed", "StudentUnsubscribed"],
///     Tags::from_pairs([("course", "c1")])?,
/// )?;
///
/// assert_eq!(item.types().len(), 2);
/// # Ok::<(), Box<dyn core::error::Error>>(())
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryItem {
    types: Box<[EventType]>,
    tags: Tags,
}

impl QueryItem {
    /// Creates a query item.
    ///
    /// Types are deduplicated and sorted so that two items expressing the same
    /// constraint compare equal.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidQuery::UnconstrainedItem`] if both `types` and `tags`
    /// are empty, or [`InvalidQuery::EventType`] if a type string is invalid.
    ///
    /// The `InvalidQuery: From<T::Error>` bound is what lets this accept both
    /// `&str` (fallible conversion) and an already-built [`EventType`]
    /// (infallible), rather than forcing callers into one or the other.
    pub fn new<T>(types: impl IntoIterator<Item = T>, tags: Tags) -> Result<Self, InvalidQuery>
    where
        T: TryInto<EventType>,
        InvalidQuery: From<T::Error>,
    {
        let mut types: Vec<EventType> = types
            .into_iter()
            .map(|ty| ty.try_into().map_err(InvalidQuery::from))
            .collect::<Result<_, _>>()?;
        types.sort_unstable();
        types.dedup();

        if types.is_empty() && tags.is_empty() {
            return Err(InvalidQuery::UnconstrainedItem);
        }

        Ok(Self {
            types: types.into_boxed_slice(),
            tags,
        })
    }

    /// Creates an item constrained only by type.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidQuery::UnconstrainedItem`] if `types` is empty, or
    /// [`InvalidQuery::EventType`] if a type string is invalid.
    pub fn of_types<T>(types: impl IntoIterator<Item = T>) -> Result<Self, InvalidQuery>
    where
        T: TryInto<EventType>,
        InvalidQuery: From<T::Error>,
    {
        Self::new(types, Tags::empty())
    }

    /// Creates an item constrained only by tags.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidQuery::UnconstrainedItem`] if `tags` is empty.
    pub fn tagged(tags: Tags) -> Result<Self, InvalidQuery> {
        Self::new(core::iter::empty::<EventType>(), tags)
    }

    /// The types this item accepts, sorted. Empty means "any type".
    pub fn types(&self) -> &[EventType] {
        &self.types
    }

    /// The tags an event must carry. Empty means "any tags".
    pub fn tags(&self) -> &Tags {
        &self.tags
    }

    /// Whether an event with this type and these tags matches.
    pub fn matches(&self, event_type: &EventType, tags: &Tags) -> bool {
        let type_ok = self.types.is_empty() || self.types.binary_search(event_type).is_ok();
        type_ok && tags.contains_all(&self.tags)
    }
}

/// A filter over the log.
///
/// Modelled as an enum rather than a `Vec<QueryItem>` because the specification
/// requires a query to hold at least one item **or** to match everything. An
/// empty item list is not a legal query, and is not representable from outside
/// this crate: [`Items`](Self::Items) is `#[non_exhaustive]`, so the only way in
/// is [`from_items`](Self::from_items), which refuses an empty sequence.
///
/// That seal is load-bearing rather than tidy. An `AppendCondition` built on a
/// query with no items is a condition nothing can ever violate — a conditional
/// append that is silently unconditional, which is a lost update with no
/// diagnostic anywhere.
///
/// Items combine with OR: an event matches the query when it matches any item.
///
/// # Examples
///
/// ```
/// use happenstance_core::{Event, Query, QueryItem, Tags};
///
/// let query = Query::from_items([
///     QueryItem::of_types(["CourseDefined"])?,
///     QueryItem::tagged(Tags::from_pairs([("student", "s1")])?)?,
/// ])?;
///
/// let event = Event::new("CourseDefined", &b"{}"[..])?;
/// assert!(query.matches(event.event_type(), event.tags()));
///
/// assert!(Query::all().matches(event.event_type(), event.tags()));
/// # Ok::<(), Box<dyn core::error::Error>>(())
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum Query {
    /// Matches every event in the store.
    #[default]
    All,
    /// Matches events satisfying at least one item.
    ///
    /// `#[non_exhaustive]` so that no downstream crate can build one directly.
    /// Inside this crate the variant is ordinary; outside it, it is matchable
    /// but not constructible.
    ///
    /// **Downstream, the tuple spelling is not one of the ways to match it.**
    /// `Query::Items(..)` is `error[E0603]: tuple variant `Items` is private`,
    /// because a tuple pattern resolves through the variant's *constructor* and
    /// `#[non_exhaustive]` is precisely what makes that constructor crate-private.
    /// The struct spellings reach the fields without naming the constructor, so
    /// `Query::Items { .. }` and `Query::Items { 0: held, .. }` both compile.
    /// Measured on 1.97.1 by `query_items_is_not_constructible_downstream` in
    /// `happenstance-testkit`. Most callers want [`Query::items`] and never meet
    /// this.
    #[non_exhaustive]
    Items(Box<[QueryItem]>),
}

impl Query {
    /// The query that matches everything.
    pub fn all() -> Self {
        Self::All
    }

    /// Builds a query from one or more items.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidQuery::NoItems`] if `items` is empty. Use
    /// [`Query::all`] to match everything.
    pub fn from_items(items: impl IntoIterator<Item = QueryItem>) -> Result<Self, InvalidQuery> {
        let items: Vec<QueryItem> = items.into_iter().collect();
        if items.is_empty() {
            return Err(InvalidQuery::NoItems);
        }
        Ok(Self::Items(items.into_boxed_slice()))
    }

    /// Convenience for the single-item case.
    ///
    /// Infallible, and typed that way: one item is never zero items, so the
    /// `Result` this used to return could not be `Err` and only taught callers
    /// to write a `?` that never fired.
    #[must_use]
    pub fn from_item(item: QueryItem) -> Self {
        Self::Items(alloc::vec![item].into_boxed_slice())
    }

    /// The items, or `None` for [`Query::All`].
    pub fn items(&self) -> Option<&[QueryItem]> {
        match self {
            Self::All => None,
            Self::Items(items) => Some(items),
        }
    }

    /// Whether this query matches everything.
    pub const fn is_all(&self) -> bool {
        matches!(self, Self::All)
    }

    /// Whether an event with this type and these tags matches.
    ///
    /// Adapters that can push filtering down into storage should do so and use
    /// this only as a reference; adapters that cannot may filter in memory with
    /// it directly.
    pub fn matches(&self, event_type: &EventType, tags: &Tags) -> bool {
        match self {
            Self::All => true,
            Self::Items(items) => items.iter().any(|item| item.matches(event_type, tags)),
        }
    }
}

/// How to traverse the matched events.
///
/// # Examples
///
/// ```
/// use happenstance_core::{ReadOptions, SequencePosition};
///
/// // The specification's own example: the 50 events at or before position 321,
/// // newest first.
/// let options = ReadOptions::new()
///     .from(SequencePosition::new(321).expect("non-zero"))
///     .backwards()
///     .limit(50);
///
/// assert!(options.backwards);
/// assert_eq!(options.limit, Some(50));
/// ```
///
/// A closed window, which is what `to` is for — a backfill worker owning
/// `[1, H]` while a tail worker owns everything above it:
///
/// ```
/// use happenstance_core::{ReadOptions, SequencePosition};
///
/// let head = SequencePosition::new(53_000_000).expect("non-zero");
/// let backfill = ReadOptions::new().from(SequencePosition::FIRST).to(head);
///
/// assert_eq!(backfill.to, Some(head));
/// ```
///
/// A limit of zero reads nothing, which is what makes `.limit(budget - fetched)`
/// safe at parity:
///
/// ```
/// use happenstance_core::ReadOptions;
///
/// assert_eq!(ReadOptions::new().limit(0).limit, Some(0));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[non_exhaustive]
pub struct ReadOptions {
    /// Where to start, **inclusive**. `None` starts at the first event when
    /// reading forwards, or the last when reading backwards.
    ///
    /// A threshold, not a seek: `from` need not name an event that exists. A
    /// read from a position nothing occupies yields the next matching event
    /// above it (or below, reading backwards), rather than erroring or coming
    /// back empty.
    pub from: Option<SequencePosition>,
    /// Where to stop, **inclusive**. `None` reads to the end.
    ///
    /// Under [`backwards`](Self::backwards), `from` remains the starting
    /// (higher) bound and `to` the stopping (lower) one — the two swap roles in
    /// position order, not in meaning.
    pub to: Option<SequencePosition>,
    /// Read in descending position order instead of ascending.
    pub backwards: bool,
    /// Stop after this many events. `None` reads all matches; `Some(0)` reads
    /// none.
    pub limit: Option<usize>,
}

impl ReadOptions {
    /// Default options: forwards, from the beginning, unlimited.
    pub const fn new() -> Self {
        Self {
            from: None,
            to: None,
            backwards: false,
            limit: None,
        }
    }

    /// Starts at `position`, inclusive.
    #[must_use]
    pub const fn from(mut self, position: SequencePosition) -> Self {
        self.from = Some(position);
        self
    }

    /// Reads newest-first.
    ///
    /// One-way: there is no `forwards()`, because the default already is.
    #[must_use]
    pub const fn backwards(mut self) -> Self {
        self.backwards = true;
        self
    }

    /// Stops at `position`, inclusive.
    ///
    /// This is the caller-side spelling of a window, and it must not be confused
    /// with a store's internal pagination: **one `read` is one sample; *n*
    /// chunked reads are *n* samples.** Nothing in the contract makes two `read`
    /// calls one snapshot. What makes stitching windows together sound is the
    /// visibility invariant — no event ever becomes visible below a position a
    /// reader has already observed — and not any isolation promise about `read`.
    #[must_use]
    pub const fn to(mut self, position: SequencePosition) -> Self {
        self.to = Some(position);
        self
    }

    /// Reads at most `limit` events.
    ///
    /// **A limit of zero reads nothing.** This is a deliberate divergence from
    /// the DCB reference implementation, which treats `limit: 0` as unlimited
    /// through JavaScript falsiness — a coherent reading of `0` in a language
    /// where `0` is falsy, and not one available here. It matches SQL's
    /// `LIMIT 0` and every paging API instead.
    ///
    /// The caller this protects is the one writing `.limit(budget - fetched)`:
    /// under the old behaviour, reaching parity turned a paging loop into a full
    /// scan of the log, silently.
    #[must_use]
    pub const fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }
}

#[cfg(feature = "serde")]
mod serde_impls {
    //! Wire mirrors. **Every field is written on every serialisation** — no
    //! `skip_serializing_if`, no `#[serde(default)]` — and neither attribute may
    //! come back (WF-2; ADR-0016 §3 and §4).
    //!
    //! Skipping makes the encoding positional in a format that is not
    //! self-describing: postcard has no field names to resynchronise against, so
    //! an absent `types` is read as whatever the next field's bytes happen to
    //! be. The saving was one byte per absent field. `#[serde(default)]` costs
    //! nothing on the write side and only widens what the decoder accepts,
    //! which is a second undocumented format nothing describes.
    //!
    use super::{Query, QueryItem};
    use crate::event::EventType;
    use crate::tag::Tags;
    use alloc::boxed::Box;
    use alloc::vec::Vec;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    #[derive(Serialize, Deserialize)]
    #[serde(rename = "QueryItem")]
    struct QueryItemWire {
        types: Box<[EventType]>,
        tags: Tags,
    }

    impl Serialize for QueryItem {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            QueryItemWire {
                types: self.types().into(),
                tags: self.tags().clone(),
            }
            .serialize(serializer)
        }
    }

    impl<'de> Deserialize<'de> for QueryItem {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let wire = QueryItemWire::deserialize(deserializer)?;
            Self::new(wire.types.into_vec(), wire.tags).map_err(serde::de::Error::custom)
        }
    }

    /// Mirror of [`Query`], carrying the tag that `Option` could not.
    ///
    /// The previous encoding wrote `All` as `serialize_none` and `Items` as
    /// `serialize_some`, which loses in a way that is easy to miss coming from
    /// C#. `Nullable<T>` is a distinct runtime type carrying its own `bool`, so
    /// `T?` and `T` are never the same value; in serde's data model `Option` is
    /// **transparent** — `serialize_some(v)` emits exactly the bytes `v` emits.
    /// So `Some(Query::Items(..))` was byte-identical to `Query::Items(..)`,
    /// `Some(Query::All)` and `None::<Query>` were both `null`, and `All`
    /// occupied the format's null: the value every buggy peer emits by accident.
    ///
    /// Externally tagged rather than internally tagged because an internal tag
    /// requires a self-describing format and postcard is not one. The tag costs
    /// one byte there (`All` is `[00]`) and buys `null != "All"` in JSON.
    #[derive(Serialize, Deserialize)]
    #[serde(rename = "Query")]
    enum QueryWire {
        All,
        Items(Vec<QueryItem>),
    }

    impl Serialize for Query {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            match self {
                Self::All => QueryWire::All,
                Self::Items(items) => QueryWire::Items(items.to_vec()),
            }
            .serialize(serializer)
        }
    }

    impl<'de> Deserialize<'de> for Query {
        /// Rejects a zero-item `Items`.
        ///
        /// Deserialisation is the other door into a private invariant, and
        /// `Query::from_items` is the only thing that enforces it. An empty
        /// item list is not "match nothing" — it is a query no event can
        /// satisfy arriving where the caller asked for a filter.
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            match QueryWire::deserialize(deserializer)? {
                QueryWire::All => Ok(Self::All),
                QueryWire::Items(items) => {
                    Self::from_items(items).map_err(serde::de::Error::custom)
                }
            }
        }
    }

    // WF-12 (ADR-0016 §13): `ReadOptions` is not on the wire, and no wire
    // mirror lives here for it. `from`/`to` are a store-local position with no
    // meaning at another store, and `backwards`/`limit` are traversal choices
    // a reader makes for itself, not a value any message carries. It appears
    // in no envelope this crate defines; giving it `Serialize`/`Deserialize`
    // anyway would make a decoder's willingness to accept it a promise this
    // crate never intended to keep.
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;

    fn ty(value: &str) -> EventType {
        EventType::new(value).unwrap()
    }

    fn tags(pairs: &[(&str, &str)]) -> Tags {
        Tags::from_pairs(pairs.iter().copied()).unwrap()
    }

    #[test]
    fn empty_query_is_unrepresentable() {
        assert!(matches!(Query::from_items([]), Err(InvalidQuery::NoItems)));
    }

    #[test]
    fn unconstrained_item_is_rejected() {
        assert!(matches!(
            QueryItem::new(core::iter::empty::<EventType>(), Tags::empty()),
            Err(InvalidQuery::UnconstrainedItem)
        ));
    }

    #[test]
    fn types_are_ord_within_an_item() {
        let item = QueryItem::of_types(["B", "A", "B"]).unwrap();
        assert_eq!(item.types().len(), 2);
        assert!(item.matches(&ty("A"), &Tags::empty()));
        assert!(item.matches(&ty("B"), &Tags::empty()));
        assert!(!item.matches(&ty("C"), &Tags::empty()));
    }

    #[test]
    fn tags_are_and_within_an_item() {
        let item = QueryItem::tagged(tags(&[("course", "c1"), ("student", "s1")])).unwrap();

        assert!(item.matches(&ty("Any"), &tags(&[("course", "c1"), ("student", "s1")])));
        // A superset still matches.
        assert!(item.matches(
            &ty("Any"),
            &tags(&[("course", "c1"), ("student", "s1"), ("extra", "e")])
        ));
        // A partial overlap does not.
        assert!(!item.matches(&ty("Any"), &tags(&[("course", "c1")])));
    }

    #[test]
    fn items_are_or_across_a_query() {
        let query = Query::from_items([
            QueryItem::of_types(["A"]).unwrap(),
            QueryItem::tagged(tags(&[("student", "s1")])).unwrap(),
        ])
        .unwrap();

        assert!(query.matches(&ty("A"), &Tags::empty()));
        assert!(query.matches(&ty("Z"), &tags(&[("student", "s1")])));
        assert!(!query.matches(&ty("Z"), &tags(&[("student", "s2")])));
    }

    #[test]
    fn item_combines_type_and_tags_with_and() {
        let item = QueryItem::new(["A"], tags(&[("course", "c1")])).unwrap();

        assert!(item.matches(&ty("A"), &tags(&[("course", "c1")])));
        assert!(!item.matches(&ty("A"), &tags(&[("course", "c2")])));
        assert!(!item.matches(&ty("B"), &tags(&[("course", "c1")])));
    }

    #[test]
    fn zero_limit_means_zero_events() {
        // This test previously asserted the opposite, and the old behaviour is
        // worth naming rather than just deleting: `limit` stored
        // `NonZeroUsize::new(limit)`, so zero became `None` — unlimited — before
        // any adapter saw it. A paging loop writing `.limit(budget - fetched)`
        // therefore read the whole log at exactly the moment its budget ran out.
        assert_eq!(ReadOptions::new().limit(0).limit, Some(0));
        assert_eq!(
            ReadOptions::new().limit,
            None,
            "unset still means unlimited"
        );
    }

    #[test]
    fn to_is_recorded_and_independent_of_from() {
        let lower = SequencePosition::new(10).unwrap();
        let upper = SequencePosition::new(20).unwrap();
        let options = ReadOptions::new().from(lower).to(upper);

        assert_eq!(options.from, Some(lower));
        assert_eq!(options.to, Some(upper));
        // Direction does not reassign the fields; it reverses what they bound.
        assert_eq!(options.backwards().to, Some(upper));
    }

    #[test]
    fn from_item_is_infallible_and_yields_one_item() {
        let query = Query::from_item(QueryItem::of_types(["A"]).unwrap());
        assert_eq!(query.items().map(<[_]>::len), Some(1));
    }
}
