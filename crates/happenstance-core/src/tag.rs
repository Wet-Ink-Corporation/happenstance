//! Tags: the domain-specific metadata that DCB uses to correlate events.

use alloc::borrow::Cow;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt;

use crate::error::InvalidTag;
use crate::validate;

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
/// Like [`EventType`](crate::EventType), backed by `Cow<'static, str>` so that a
/// tag written in the source costs no allocation and a tag arriving from a peer
/// at run time is still representable.
///
/// # Examples
///
/// ```
/// use happenstance_core::Tag;
///
/// let tag = Tag::key_value("course", "c1")?;
/// assert_eq!(tag.as_str(), "course:c1");
/// assert_eq!(tag.key(), Some("course"));
/// assert_eq!(tag.value(), Some("c1"));
///
/// // Opaque tags are legal too.
/// let opaque = Tag::new("archived")?;
/// assert_eq!(opaque.key(), None);
/// # Ok::<(), happenstance_core::InvalidTag>(())
/// ```
#[derive(Clone)]
pub struct Tag(Cow<'static, str>);

impl Tag {
    /// Creates a tag from an arbitrary string.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidTag`] if the value is empty, longer than
    /// [`MAX_TAG_LEN`] bytes, contains a character in Unicode general category
    /// `Cc`, or contains one of the seven explicit bidirectional formatting
    /// controls.
    pub fn new(value: impl Into<String>) -> Result<Self, InvalidTag> {
        let value = value.into();
        match validate::check(&value, MAX_TAG_LEN) {
            validate::Refusal::Accepted => Ok(Self(Cow::Owned(value))),
            validate::Refusal::Empty => Err(InvalidTag::Empty),
            validate::Refusal::TooLong => Err(InvalidTag::TooLong { len: value.len() }),
            validate::Refusal::ControlCharacter => Err(InvalidTag::ControlCharacter),
            validate::Refusal::BidirectionalControl => Err(InvalidTag::BidirectionalControl),
        }
    }

    /// Creates a tag from a string literal, validating at compile time.
    ///
    /// Enforces exactly the rules [`new`](Self::new) enforces.
    ///
    /// # Panics
    ///
    /// Panics if the value would be rejected by [`new`](Self::new). See
    /// [`EventType::from_static`](crate::EventType::from_static) for where that
    /// panic surfaces, which depends on the kind of call site and is not
    /// uniform.
    #[must_use]
    pub const fn from_static(value: &'static str) -> Self {
        match validate::check(value, MAX_TAG_LEN) {
            validate::Refusal::Accepted => Self(Cow::Borrowed(value)),
            validate::Refusal::Empty => panic!("a tag must not be empty"),
            validate::Refusal::TooLong => panic!("a tag must be at most MAX_TAG_LEN bytes"),
            validate::Refusal::ControlCharacter => {
                panic!("a tag must not contain control characters")
            }
            validate::Refusal::BidirectionalControl => {
                panic!("a tag must not contain bidirectional formatting controls")
            }
        }
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
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// The portion before the first `:`, if the tag follows the `key:value`
    /// convention.
    #[must_use]
    pub fn key(&self) -> Option<&str> {
        self.0.split_once(':').map(|(key, _)| key)
    }

    /// The portion after the first `:`, if the tag follows the `key:value`
    /// convention.
    #[must_use]
    pub fn value(&self) -> Option<&str> {
        self.0.split_once(':').map(|(_, value)| value)
    }
}

// Hand-written for the reason given on `EventType`: a derive would be correct
// today and quietly wrong after a second field.
impl PartialEq for Tag {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}

impl Eq for Tag {}

impl PartialOrd for Tag {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Tag {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl core::hash::Hash for Tag {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.as_str().hash(state);
    }
}

impl core::borrow::Borrow<str> for Tag {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl core::str::FromStr for Tag {
    type Err = InvalidTag;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
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
/// use happenstance_core::{Tag, Tags};
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
/// # Ok::<(), happenstance_core::InvalidTag>(())
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
    /// use happenstance_core::Tags;
    ///
    /// let tags = Tags::from_pairs([("course", "c1"), ("student", "s1")])?;
    /// assert_eq!(tags.len(), 2);
    /// # Ok::<(), happenstance_core::InvalidTag>(())
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
    /// use happenstance_core::Tags;
    ///
    /// let event = Tags::from_pairs([("course", "c1"), ("student", "s1")])?;
    /// let wanted = Tags::from_pairs([("course", "c1")])?;
    ///
    /// assert!(event.contains_all(&wanted));
    /// assert!(!wanted.contains_all(&event));
    /// # Ok::<(), happenstance_core::InvalidTag>(())
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
    #[must_use]
    pub fn as_slice(&self) -> &[Tag] {
        &self.0
    }

    /// Every value stored under `key`, in canonical order.
    ///
    /// Returns **all** matches rather than one, because repeated keys are legal
    /// — the contract does not enforce the `key:value` convention, and
    /// `Tags::from_pairs([("tenant", "a"), ("tenant", "b")])` is a two-element
    /// set, deduplication being on the whole tag string. An accessor returning
    /// a single value would make that ambiguity invisible rather than resolving
    /// it, which is why there is no `get(key)`.
    ///
    /// Empty when nothing uses the key.
    ///
    /// # Examples
    ///
    /// ```
    /// use happenstance_core::Tags;
    ///
    /// let tags = Tags::from_pairs([("tenant", "a"), ("course", "c1"), ("tenant", "b")])?;
    /// let tenants: Vec<_> = tags.values_of("tenant").collect();
    /// assert_eq!(tenants, ["a", "b"]);
    /// assert_eq!(tags.values_of("absent").count(), 0);
    /// # Ok::<(), happenstance_core::InvalidTag>(())
    /// ```
    pub fn values_of<'a>(&'a self, key: &'a str) -> impl Iterator<Item = &'a str> + 'a {
        // A binary search rather than a scan, and it is VT-16's canonicalisation
        // that pays for it: `Tags` is sorted by the byte ordering of the *whole*
        // tag string, so every tag beginning `"key:"` occupies one contiguous
        // run — lexicographic order cannot interleave a prefix's members with
        // anything else. `partition_point` needs a monotone predicate, and
        // `before_key_run` is one: across a sorted slice it is true then false.
        let start = self
            .0
            .partition_point(|tag| before_key_run(tag.as_str(), key));
        self.0[start..]
            .iter()
            .map(Tag::as_str)
            .take_while(move |tag| in_key_run(tag, key))
            // `key.len() + 1` is the byte after the `:`, and `:` is ASCII, so
            // this is always a character boundary.
            .map(move |tag| &tag[key.len() + 1..])
    }
}

/// Whether `tag` sorts strictly before the run of tags prefixed `"<key>:"`.
///
/// Equivalent to `tag < format!("{key}:")` without building that string, which
/// matters because this runs inside a binary search on the hot path of every
/// tag lookup.
fn before_key_run(tag: &str, key: &str) -> bool {
    let t = tag.as_bytes();
    let k = key.as_bytes();
    let shared = if t.len() < k.len() { t.len() } else { k.len() };

    let mut i = 0;
    while i < shared {
        if t[i] != k[i] {
            return t[i] < k[i];
        }
        i += 1;
    }

    if t.len() <= k.len() {
        // `tag` is a prefix of `key`, or equal to it. Either way it sorts
        // before `"key:"`, because that string is strictly longer.
        true
    } else {
        // `key` is a prefix of `tag`; the byte after decides against ':'.
        t[k.len()] < b':'
    }
}

/// Whether `tag` is in the run — it begins `"<key>:"` and has something after.
fn in_key_run(tag: &str, key: &str) -> bool {
    let t = tag.as_bytes();
    let k = key.as_bytes();
    t.len() > k.len() && t[k.len()] == b':' && t[..k.len()] == *k
}

impl IntoIterator for Tags {
    type Item = Tag;
    type IntoIter = alloc::vec::IntoIter<Tag>;

    /// Consumes the set, yielding owned tags in canonical order.
    ///
    /// The borrowing form already existed; this one is what lets a caller take
    /// the tags out of an event it owns without cloning each one.
    fn into_iter(self) -> Self::IntoIter {
        Vec::from(self.0).into_iter()
    }
}

impl Extend<Tag> for Tags {
    /// Adds tags and re-canonicalises.
    ///
    /// **Not the amortised O(1) `Extend` usually implies.** Each call sorts and
    /// deduplicates the whole set, so extending *n* times costs *n* sorts;
    /// build the tags first and `collect` once where you can.
    fn extend<I: IntoIterator<Item = Tag>>(&mut self, iter: I) {
        let mut tags = Vec::from(core::mem::take(&mut self.0));
        tags.extend(iter);
        tags.sort_unstable();
        tags.dedup();
        self.0 = tags.into_boxed_slice();
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

    fn values_of(set: &Tags, key: &str) -> Vec<String> {
        set.values_of(key).map(String::from).collect()
    }

    #[test]
    fn values_of_finds_every_value_under_a_repeated_key() {
        // VT-17: repeated keys are legal, and an accessor returning one value
        // would hide the second rather than resolve it.
        let set = tags(&["tenant:a", "course:c1", "tenant:b"]);
        assert_eq!(values_of(&set, "tenant"), ["a", "b"]);
        assert_eq!(values_of(&set, "course"), ["c1"]);
    }

    #[test]
    fn values_of_is_empty_for_an_unused_key() {
        let set = tags(&["course:c1"]);
        assert!(values_of(&set, "student").is_empty());
        assert!(values_of(&tags(&[]), "course").is_empty());
    }

    #[test]
    fn values_of_does_not_match_a_key_that_is_merely_a_prefix() {
        // The binary search bounds the run of tags beginning `"key:"`. A tag
        // beginning `"keyless:"` shares the first three bytes and must not be
        // swept in, and `"key"` with no colon at all is not in the run either.
        let set = tags(&["key", "keyless:x", "key:real", "keyz:y"]);
        assert_eq!(values_of(&set, "key"), ["real"]);
    }

    #[test]
    fn values_of_handles_the_boundaries_of_the_sorted_run() {
        // A run at the very start of the set, and one at the very end: an
        // off-by-one in `before_key_run` shows up at exactly these two places
        // and nowhere in the middle.
        let set = tags(&["aaa:first", "mmm:middle", "zzz:last"]);
        assert_eq!(values_of(&set, "aaa"), ["first"]);
        assert_eq!(values_of(&set, "zzz"), ["last"]);
        assert_eq!(values_of(&set, "mmm"), ["middle"]);
    }

    #[test]
    fn values_of_keeps_a_value_containing_a_colon_whole() {
        // Only the first colon separates; `Tag::value` says so, and `values_of`
        // must agree with it rather than splitting again.
        let set = tags(&["url:https://example.test"]);
        assert_eq!(values_of(&set, "url"), ["https://example.test"]);
    }

    #[test]
    fn values_of_agrees_with_tag_value_on_an_empty_value() {
        // `Tag::key_value` refuses an empty half, but `Tag::new("key:")` is
        // reachable directly, and for that tag `Tag::value()` is `Some("")`.
        // `values_of` yields the empty string to match. The alternative —
        // skipping it — would make the two accessors disagree about whether the
        // tag has a value at all, which is worse than an empty answer.
        let set = tags(&["key:", "key:v"]);
        assert_eq!(set.as_slice()[0].value(), Some(""));
        assert_eq!(values_of(&set, "key"), ["", "v"]);
    }

    #[test]
    fn a_borrowed_and_an_owned_tag_are_one_value() {
        // The property `Borrow<str>` promises, and the reason `Eq`/`Hash` are
        // hand-written to agree with `str`.
        use core::borrow::Borrow;
        let borrowed = Tag::from_static("course:c1");
        let owned = Tag::new("course:c1").unwrap();
        assert_eq!(borrowed, owned);
        let key: &str = borrowed.borrow();
        assert_eq!(key, owned.as_str());
        assert_eq!(tags(&["course:c1"]).len(), [borrowed, owned].len() - 1);
    }

    #[test]
    fn extend_re_canonicalises() {
        let mut set = tags(&["b", "a"]);
        set.extend([Tag::new("a").unwrap(), Tag::new("c").unwrap()]);
        assert_eq!(
            set.iter().map(Tag::as_str).collect::<Vec<_>>(),
            ["a", "b", "c"]
        );
    }

    #[test]
    fn owned_into_iterator_yields_canonical_order() {
        let owned: Vec<Tag> = tags(&["b", "a"]).into_iter().collect();
        assert_eq!(
            owned.iter().map(Tag::as_str).collect::<Vec<_>>(),
            ["a", "b"]
        );
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
