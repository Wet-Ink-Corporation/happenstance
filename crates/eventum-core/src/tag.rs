//! Tags: the domain-specific metadata that DCB uses to correlate events.

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt;

use crate::error::InvalidTag;

/// Longest permitted tag, in bytes.
///
/// Bounded so that storage adapters can index tags in a fixed-width column
/// without silently truncating.
pub const MAX_TAG_LEN: usize = 255;

/// A single domain-specific label attached to an [`Event`](crate::Event).
///
/// The DCB specification treats a tag as an opaque string. By convention tags
/// take a `key:value` shape such as `course:c1`, and [`Tag::key`] and
/// [`Tag::value`] expose that convention without enforcing it — a tag with no
/// colon is perfectly legal.
///
/// # Examples
///
/// ```
/// use eventum_core::Tag;
///
/// let tag = Tag::key_value("course", "c1")?;
/// assert_eq!(tag.as_str(), "course:c1");
/// assert_eq!(tag.key(), Some("course"));
/// assert_eq!(tag.value(), Some("c1"));
///
/// // Opaque tags are legal too.
/// let opaque = Tag::new("archived")?;
/// assert_eq!(opaque.key(), None);
/// # Ok::<(), eventum_core::InvalidTag>(())
/// ```
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Tag(Box<str>);

impl Tag {
    /// Creates a tag from an arbitrary string.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidTag`] if the value is empty, longer than
    /// [`MAX_TAG_LEN`] bytes, or contains ASCII control characters.
    pub fn new(value: impl Into<String>) -> Result<Self, InvalidTag> {
        let value = value.into();
        if value.is_empty() {
            return Err(InvalidTag::Empty);
        }
        if value.len() > MAX_TAG_LEN {
            return Err(InvalidTag::TooLong { len: value.len() });
        }
        if value.chars().any(char::is_control) {
            return Err(InvalidTag::ControlCharacter);
        }
        Ok(Self(value.into_boxed_str()))
    }

    /// Creates a `key:value` tag, the conventional DCB shape.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidTag`] if either part is empty, if `key` itself contains
    /// a colon (which would make [`Tag::key`] ambiguous), or if the joined
    /// value fails [`Tag::new`]'s validation.
    pub fn key_value(key: &str, value: &str) -> Result<Self, InvalidTag> {
        if key.is_empty() || value.is_empty() {
            return Err(InvalidTag::Empty);
        }
        if key.contains(':') {
            return Err(InvalidTag::ColonInKey);
        }
        let mut joined = String::with_capacity(key.len() + 1 + value.len());
        joined.push_str(key);
        joined.push(':');
        joined.push_str(value);
        Self::new(joined)
    }

    /// The tag as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// The portion before the first `:`, if the tag follows the `key:value`
    /// convention.
    pub fn key(&self) -> Option<&str> {
        self.0.split_once(':').map(|(key, _)| key)
    }

    /// The portion after the first `:`, if the tag follows the `key:value`
    /// convention.
    pub fn value(&self) -> Option<&str> {
        self.0.split_once(':').map(|(_, value)| value)
    }
}

impl fmt::Debug for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // A tag is conceptually a string; showing it as `Tag("course:c1")`
        // rather than a struct keeps assertion failures readable.
        write!(f, "Tag({:?})", &*self.0)
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl AsRef<str> for Tag {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl TryFrom<&str> for Tag {
    type Error = InvalidTag;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<String> for Tag {
    type Error = InvalidTag;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

/// A canonical set of [`Tag`]s: always sorted, always deduplicated.
///
/// Canonicalisation is enforced at construction, so a `Tags` value can never be
/// observed unsorted. That buys three things:
///
/// * [`Tags::contains_all`] is a linear merge-scan rather than a nested loop;
/// * `PartialEq`/`Hash` mean set equality, not insertion-order equality;
/// * storage adapters get a stable serialisation to build an index on.
///
/// # Examples
///
/// ```
/// use eventum_core::{Tag, Tags};
///
/// let tags: Tags = [Tag::new("b")?, Tag::new("a")?, Tag::new("b")?]
///     .into_iter()
///     .collect();
///
/// // Sorted and deduplicated.
/// assert_eq!(tags.len(), 2);
/// assert_eq!(tags.iter().map(Tag::as_str).collect::<Vec<_>>(), ["a", "b"]);
///
/// // Order of construction does not affect equality.
/// let other: Tags = [Tag::new("a")?, Tag::new("b")?].into_iter().collect();
/// assert_eq!(tags, other);
/// # Ok::<(), eventum_core::InvalidTag>(())
/// ```
#[derive(Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Tags(Box<[Tag]>);

impl Tags {
    /// The empty tag set.
    pub fn empty() -> Self {
        Self(Box::default())
    }

    /// Builds a canonical tag set from `key:value` pairs.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidTag`] if any pair fails [`Tag::key_value`]'s validation.
    ///
    /// # Examples
    ///
    /// ```
    /// use eventum_core::Tags;
    ///
    /// let tags = Tags::from_pairs([("course", "c1"), ("student", "s1")])?;
    /// assert_eq!(tags.len(), 2);
    /// # Ok::<(), eventum_core::InvalidTag>(())
    /// ```
    pub fn from_pairs<'a>(
        pairs: impl IntoIterator<Item = (&'a str, &'a str)>,
    ) -> Result<Self, InvalidTag> {
        pairs
            .into_iter()
            .map(|(key, value)| Tag::key_value(key, value))
            .collect::<Result<Vec<_>, _>>()
            .map(|tags| tags.into_iter().collect())
    }

    /// Number of distinct tags.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Whether the set is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Whether `tag` is present. Runs in `O(log n)`.
    pub fn contains(&self, tag: &Tag) -> bool {
        self.0.binary_search(tag).is_ok()
    }

    /// Whether every tag in `subset` is present in `self`.
    ///
    /// This is the AND-semantics that a [`QueryItem`](crate::QueryItem) applies
    /// to its tags. Both sides are sorted, so this is a single `O(n + m)`
    /// merge-scan.
    ///
    /// # Examples
    ///
    /// ```
    /// use eventum_core::Tags;
    ///
    /// let event = Tags::from_pairs([("course", "c1"), ("student", "s1")])?;
    /// let wanted = Tags::from_pairs([("course", "c1")])?;
    ///
    /// assert!(event.contains_all(&wanted));
    /// assert!(!wanted.contains_all(&event));
    /// # Ok::<(), eventum_core::InvalidTag>(())
    /// ```
    pub fn contains_all(&self, subset: &Self) -> bool {
        let mut haystack = self.0.iter();
        // For each needle, advance the haystack until we reach or pass it.
        // Both are sorted, so a single pass suffices.
        subset.0.iter().all(|needle| {
            for candidate in haystack.by_ref() {
                match candidate.cmp(needle) {
                    core::cmp::Ordering::Less => {}
                    core::cmp::Ordering::Equal => return true,
                    core::cmp::Ordering::Greater => return false,
                }
            }
            false
        })
    }

    /// Iterates the tags in canonical (sorted) order.
    pub fn iter(&self) -> core::slice::Iter<'_, Tag> {
        self.0.iter()
    }

    /// The tags as a sorted, deduplicated slice.
    pub fn as_slice(&self) -> &[Tag] {
        &self.0
    }
}

impl FromIterator<Tag> for Tags {
    /// Sorts and deduplicates, producing the canonical form.
    fn from_iter<I: IntoIterator<Item = Tag>>(iter: I) -> Self {
        let mut tags: Vec<Tag> = iter.into_iter().collect();
        tags.sort_unstable();
        tags.dedup();
        Self(tags.into_boxed_slice())
    }
}

impl<'a> IntoIterator for &'a Tags {
    type Item = &'a Tag;
    type IntoIter = core::slice::Iter<'a, Tag>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl From<Tags> for Vec<Tag> {
    fn from(tags: Tags) -> Self {
        tags.0.into_vec()
    }
}

impl fmt::Debug for Tags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_set()
            .entries(self.0.iter().map(Tag::as_str))
            .finish()
    }
}

#[cfg(feature = "serde")]
mod serde_impls {
    use super::{Tag, Tags};
    use alloc::string::String;
    use alloc::vec::Vec;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    impl Serialize for Tag {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            serializer.serialize_str(self.as_str())
        }
    }

    impl<'de> Deserialize<'de> for Tag {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let raw = String::deserialize(deserializer)?;
            Self::new(raw).map_err(serde::de::Error::custom)
        }
    }

    impl Serialize for Tags {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            self.as_slice().serialize(serializer)
        }
    }

    impl<'de> Deserialize<'de> for Tags {
        /// Re-canonicalises on the way in: a peer that sends unsorted or
        /// duplicated tags cannot smuggle a non-canonical `Tags` into memory.
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let raw = Vec::<Tag>::deserialize(deserializer)?;
            Ok(raw.into_iter().collect())
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;

    fn tags(values: &[&str]) -> Tags {
        values.iter().map(|v| Tag::new(*v).unwrap()).collect()
    }

    #[test]
    fn rejects_invalid_tags() {
        assert!(matches!(Tag::new(""), Err(InvalidTag::Empty)));
        assert!(matches!(
            Tag::new("a\u{0}b"),
            Err(InvalidTag::ControlCharacter)
        ));
        assert!(matches!(
            Tag::new("x".repeat(MAX_TAG_LEN + 1)),
            Err(InvalidTag::TooLong { .. })
        ));
        assert!(matches!(
            Tag::key_value("a:b", "c"),
            Err(InvalidTag::ColonInKey)
        ));
    }

    #[test]
    fn key_value_splits_on_first_colon_only() {
        let tag = Tag::new("url:https://example.com").unwrap();
        assert_eq!(tag.key(), Some("url"));
        assert_eq!(tag.value(), Some("https://example.com"));
    }

    #[test]
    fn canonicalisation_is_idempotent() {
        let once = tags(&["c", "a", "b", "a"]);
        let twice: Tags = once.iter().cloned().collect();
        assert_eq!(once, twice);
        assert_eq!(once.len(), 3);
    }

    #[test]
    fn contains_all_is_subset_semantics() {
        let event = tags(&["a", "b", "c"]);

        assert!(event.contains_all(&Tags::empty()));
        assert!(event.contains_all(&tags(&["a", "c"])));
        assert!(event.contains_all(&event));

        assert!(!event.contains_all(&tags(&["a", "d"])));
        assert!(!event.contains_all(&tags(&["d"])));
        // Partial overlap must not count as a match.
        assert!(!tags(&["a"]).contains_all(&tags(&["a", "b"])));
    }

    #[test]
    fn contains_all_handles_interleaved_ordering() {
        // Regression guard for the merge-scan: needles must not consume
        // haystack entries they have already passed.
        let event = tags(&["a", "c", "e", "g"]);
        assert!(event.contains_all(&tags(&["a", "e"])));
        assert!(event.contains_all(&tags(&["c", "g"])));
        assert!(!event.contains_all(&tags(&["b", "e"])));
        assert!(!event.contains_all(&tags(&["a", "f"])));
    }
}
