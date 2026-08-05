//! Queries: how callers select the slice of the log they care about.

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::num::NonZeroUsize;

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
/// use happenstance::{QueryItem, Tags};
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
/// empty item list is not a legal query, so it is not representable.
///
/// Items combine with OR: an event matches the query when it matches any item.
///
/// # Examples
///
/// ```
/// use happenstance::{Event, Query, QueryItem, Tags};
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
    /// # Errors
    ///
    /// Infallible in practice; the signature mirrors [`Query::from_items`].
    pub fn from_item(item: QueryItem) -> Result<Self, InvalidQuery> {
        Self::from_items([item])
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
/// use happenstance::{ReadOptions, SequencePosition};
///
/// // The specification's own example: the 50 events at or before position 321,
/// // newest first.
/// let options = ReadOptions::new()
///     .from(SequencePosition::new(321).expect("non-zero"))
///     .backwards()
///     .limit(50);
///
/// assert!(options.backwards);
/// assert_eq!(options.limit.map(std::num::NonZeroUsize::get), Some(50));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[non_exhaustive]
pub struct ReadOptions {
    /// Where to start, **inclusive**. `None` starts at the first event when
    /// reading forwards, or the last when reading backwards.
    pub from: Option<SequencePosition>,
    /// Read in descending position order instead of ascending.
    pub backwards: bool,
    /// Stop after this many events. `None` reads all matches.
    pub limit: Option<NonZeroUsize>,
}

impl ReadOptions {
    /// Default options: forwards, from the beginning, unlimited.
    pub const fn new() -> Self {
        Self {
            from: None,
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
    #[must_use]
    pub const fn backwards(mut self) -> Self {
        self.backwards = true;
        self
    }

    /// Reads at most `limit` events. A `limit` of zero is ignored, since
    /// requesting nothing is never what the caller meant.
    #[must_use]
    pub const fn limit(mut self, limit: usize) -> Self {
        self.limit = NonZeroUsize::new(limit);
        self
    }
}

#[cfg(feature = "serde")]
mod serde_impls {
    use super::{Query, QueryItem, ReadOptions};
    use crate::event::EventType;
    use crate::tag::Tags;
    use alloc::boxed::Box;
    use alloc::vec::Vec;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    #[derive(Serialize, Deserialize)]
    #[serde(rename = "QueryItem")]
    struct QueryItemWire {
        #[serde(default, skip_serializing_if = "<[EventType]>::is_empty")]
        types: Box<[EventType]>,
        #[serde(default, skip_serializing_if = "Tags::is_empty")]
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

    impl Serialize for Query {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            // `None` is the match-all query; `Some(items)` is a filtered one.
            match self {
                Self::All => serializer.serialize_none(),
                Self::Items(items) => serializer.serialize_some(items),
            }
        }
    }

    impl<'de> Deserialize<'de> for Query {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            match Option::<Vec<QueryItem>>::deserialize(deserializer)? {
                None => Ok(Self::All),
                Some(items) => Self::from_items(items).map_err(serde::de::Error::custom),
            }
        }
    }

    #[derive(Default, Serialize, Deserialize)]
    #[serde(rename = "ReadOptions", default)]
    struct ReadOptionsWire {
        from: Option<crate::event::SequencePosition>,
        backwards: bool,
        limit: Option<core::num::NonZeroUsize>,
    }

    impl Serialize for ReadOptions {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            ReadOptionsWire {
                from: self.from,
                backwards: self.backwards,
                limit: self.limit,
            }
            .serialize(serializer)
        }
    }

    impl<'de> Deserialize<'de> for ReadOptions {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let wire = ReadOptionsWire::deserialize(deserializer)?;
            Ok(Self {
                from: wire.from,
                backwards: wire.backwards,
                limit: wire.limit,
            })
        }
    }
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
    fn zero_limit_is_ignored() {
        assert_eq!(ReadOptions::new().limit(0).limit, None);
    }
}
