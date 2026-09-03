//! The two tag regimes, and the events built in each.
//!
//! # The point of the experiment is that there are two of them
//!
//! `Tag` is `Cow<'static, str>` (`crates/happenstance-core/src/tag.rs:79`) and
//! `Tags` is `Box<[Tag]>` (`:281`). `Box<[T]>: Clone` deep-clones elementwise,
//! so what one `Tags::clone` costs depends entirely on which `Cow` variant the
//! tags are in:
//!
//! * [`Regime::Owned`] — `Tags::from_pairs` → `Tag::key_value` → `Tag::new`, and
//!   `Tag::new` ends at `Cow::Owned(String)` (`tag.rs:93`). Cloning one such tag
//!   allocates. This is the canonical application path: it is the only
//!   constructor that takes a runtime value, so every tag naming a real entity
//!   is in it.
//! * [`Regime::Static`] — `Tag::from_static`, which is `const fn` and ends at
//!   `Cow::Borrowed` (`tag.rs:114`). Cloning one such tag copies two words and
//!   allocates nothing. This is the regime a benchmark author reaches for
//!   without thinking about it, because a `const` tag is what a test fixture
//!   naturally holds.
//!
//! The two build **the same event**. Same type string, same sixty-four tag
//! strings, same payload — `tests/arms_are_equivalent.rs` asserts `==` and
//! asserts byte-identical `serde_json` and `postcard` encodings, because an arm
//! that is cheaper by virtue of encoding less has won nothing.

use happenstance_core::bytes::Bytes;
use happenstance_core::{Event, EventType, Tag, Tags};

/// Which `Cow` variant the tags are in. The whole experiment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Regime {
    /// `Tags::from_pairs` — `Cow::Owned`. The canonical application path.
    Owned,
    /// `Tag::from_static` — `Cow::Borrowed`. The free path, and the trap.
    Static,
}

impl Regime {
    /// The label used in every printed row and every results table.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Owned => "from_pairs (Cow::Owned)",
            Self::Static => "from_static (Cow::Borrowed)",
        }
    }
}

/// The event type string, identical in both regimes.
pub const EVENT_TYPE: &str = "StudentSubscribed";

/// The sixty-four tag strings, identical in both regimes.
///
/// Sixty-four is [`happenstance_core::MIN_SUPPORTED_TAGS_PER_EVENT`] — VT-22's
/// floor, the number every conformant adapter must accept, and therefore the
/// number a cost claim about tags has to be quoted at.
pub const TAG_LITERALS: [&str; 64] = [
    "k00:v00", "k01:v01", "k02:v02", "k03:v03", "k04:v04", "k05:v05", "k06:v06", "k07:v07",
    "k08:v08", "k09:v09", "k10:v10", "k11:v11", "k12:v12", "k13:v13", "k14:v14", "k15:v15",
    "k16:v16", "k17:v17", "k18:v18", "k19:v19", "k20:v20", "k21:v21", "k22:v22", "k23:v23",
    "k24:v24", "k25:v25", "k26:v26", "k27:v27", "k28:v28", "k29:v29", "k30:v30", "k31:v31",
    "k32:v32", "k33:v33", "k34:v34", "k35:v35", "k36:v36", "k37:v37", "k38:v38", "k39:v39",
    "k40:v40", "k41:v41", "k42:v42", "k43:v43", "k44:v44", "k45:v45", "k46:v46", "k47:v47",
    "k48:v48", "k49:v49", "k50:v50", "k51:v51", "k52:v52", "k53:v53", "k54:v54", "k55:v55",
    "k56:v56", "k57:v57", "k58:v58", "k59:v59", "k60:v60", "k61:v61", "k62:v62", "k63:v63",
];

/// The `("k00", "v00")` pairs `Tags::from_pairs` joins back into
/// [`TAG_LITERALS`]. Split here rather than parsed, so the two lists are
/// independently written and `tests/arms_are_equivalent.rs` compares them.
#[must_use]
pub fn tag_pairs(count: usize) -> Vec<(&'static str, &'static str)> {
    TAG_LITERALS[..count]
        .iter()
        .map(|literal| {
            let (key, value) = literal.split_once(':').expect("every literal is key:value");
            (key, value)
        })
        .collect()
}

/// Builds the tag set for a regime at `count` tags.
///
/// # Panics
///
/// Panics if `count` exceeds [`TAG_LITERALS`]'s length, or if a literal fails
/// validation — neither can happen for the literals in this file, and a panic
/// here would mean the fixture, not the measurement, is wrong.
#[must_use]
pub fn tags(regime: Regime, count: usize) -> Tags {
    match regime {
        Regime::Owned => Tags::from_pairs(tag_pairs(count)).expect("the pairs are valid"),
        Regime::Static => TAG_LITERALS[..count]
            .iter()
            .copied()
            .map(Tag::from_static)
            .collect(),
    }
}

/// Builds the event type for a regime.
///
/// # Panics
///
/// Panics if [`EVENT_TYPE`] fails validation, which it does not.
#[must_use]
pub fn event_type(regime: Regime) -> EventType {
    match regime {
        Regime::Owned => EventType::new(EVENT_TYPE).expect("the type is valid"),
        // `from_static` is `const fn`, but it is called here at run time on
        // purpose: a `const` item would be identical and would hide that the
        // difference between the arms is the `Cow` variant rather than the
        // evaluation time.
        Regime::Static => EventType::from_static(EVENT_TYPE),
    }
}

/// How the payload `Bytes` was built, which changes what its *first* clone
/// costs and nothing after that.
///
/// This axis is not in the finding and is measured anyway, because the sentence
/// under review — "payloads are `Bytes`, so a snapshot bumps refcounts rather
/// than copying data" (`memory.rs:30-31`) — is a claim about `Bytes::clone`, and
/// `bytes` 1.x has two representations with different clone costs.
/// [`Payload::Static`] is `Bytes::from_static`, whose clone is genuinely free.
/// [`Payload::Vec`] is `Bytes::from(Vec<u8>)`, which starts *promotable*.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Payload {
    /// `Bytes::from_static` — no owner to share, clone is two words.
    Static,
    /// `Bytes::from(Vec<u8>)` — the shape any decoded or generated payload has.
    Vec,
}

impl Payload {
    /// The label used in every printed row.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Static => "Bytes::from_static",
            Self::Vec => "Bytes::from(Vec<u8>)",
        }
    }
}

/// The payload bytes, identical in content across both [`Payload`] shapes.
pub const PAYLOAD: &[u8] = b"{\"student\":\"s1\",\"course\":\"c1\"}";

/// Builds the payload for a shape.
#[must_use]
pub fn payload(shape: Payload) -> Bytes {
    match shape {
        Payload::Static => Bytes::from_static(PAYLOAD),
        Payload::Vec => Bytes::from(PAYLOAD.to_vec()),
    }
}

/// Builds one event: `count` tags in `regime`, payload in `shape`, no metadata.
///
/// # Panics
///
/// Panics if the fixture strings fail validation, which they do not.
#[must_use]
pub fn event(regime: Regime, count: usize, shape: Payload) -> Event {
    Event::new(event_type(regime), payload(shape))
        .expect("the event type is valid")
        .with_tags(tags(regime, count))
}
