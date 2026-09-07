//! The workloads, and the one axis that decides whether any of them mean
//! anything.
//!
//! # The regime trap, which this module exists to make unhittable
//!
//! `references/evaluation/review-pre-publication-2026-09-03.md:2724` measured
//! one `Event::clone()` at **66 heap operations requesting 2,001 bytes** at
//! VT-22's 64-tag conformance floor, against **1 operation and 1,536 bytes**
//! when the tags came from `Tag::from_static`. Exact at 1, 8, 32 and 64 tags:
//! the interned arm is *flat in tag count* and does not respond to the axis at
//! all.
//!
//! That is not a curiosity. ES-17 — the clause that keeps `append`'s batch
//! borrowed — is `[PROVISIONAL]` and is falsified by "a measurement on a real
//! adapter showing the per-event clone is a material fraction of append cost".
//! The interned arm is the one a benchmark author writes *without choosing to*,
//! because `from_static` constants are what a test fixture naturally holds. A
//! suite built that way measures a regime with a 66× cheaper clone, reports the
//! clone immaterial, and lifts a marker on evidence that could not have gone
//! the other way. The review's own words: *"a falsifier that can be satisfied
//! by construction"*.
//!
//! So [`Regime`] is a required parameter of every constructor here, there is no
//! default, and every table this crate emits carries a regime column.
//!
//! # What the regime governs, and why it is one enum and not three
//!
//! It governs all three allocating parts of an [`Event`] together:
//!
//! | Part | [`Regime::Interned`] | [`Regime::Owned`] |
//! | --- | --- | --- |
//! | `EventType` | `EventType::from_static` — a `Cow::Borrowed`, no allocation | `EventType::new(String)` — one allocation |
//! | `Tags` | `Tag::from_static` × *n* — the boxed slice only | `Tag::key_value` × *n* — *n* + 1 allocations |
//! | `Bytes` payload | `Bytes::from_static` — no header, clone is a no-op | `Bytes::from(Vec<u8>)` — promotable, and its **first** clone allocates a shared header |
//!
//! One enum rather than three axes because they are one trap, not three, and
//! because no application is in a mixed state: a payload arrives decoded from a
//! wire and a tag is built from a domain identifier, so **`Owned` is the regime
//! a real caller is in**. `Interned` is here as the *control* — the arm that
//! shows what a suite written the natural way would have reported — and it is
//! labelled that way wherever it appears.
//!
//! The `Bytes` row is the subtlety the review also caught: a payload built with
//! `Bytes::from(Vec<u8>)` starts promotable, so its first clone allocates a
//! shared header and costs 67 where the second costs 66.
//!
//! # Why `Interned` cannot build a distinct-tag corpus
//!
//! `Tag::from_static` takes a `&'static str`, so per-event distinct tags are
//! impossible without leaking. That is not a limitation being worked around —
//! it *is* the trap, stated in the type system's own terms: a fixture holding
//! constants necessarily gives every event the same tags. [`Corpus::distinct`]
//! therefore refuses the interned regime by name rather than silently handing
//! back a corpus in which every tag matches every query.
//!
//! # Where the numbers come from
//!
//! The sizes below are the specification's own conformance floors, not
//! convenient round numbers. VT-21, VT-22, VT-23 and VT-24 are what *every*
//! conformant store must survive, and it is only at them that
//! `references/evaluation/review-pre-publication-2026-09-03.md`'s I-5 (a 40×
//! quadratic tag dedup) and X-1 (51,600 parameters against SQLite's 32,766)
//! appear at all. A suite that measured at eight tags and a 64-byte payload
//! would report neither.

use bytes::Bytes;
use happenstance_core::{Event, EventType, Query, QueryItem, Tag, Tags};

/// VT-21's `MIN_SUPPORTED_EVENT_DATA_LEN`: 64 KiB.
///
/// The payload size every conformant store must accept
/// (`spec/SPECIFICATION.md:1513`). `happenstance-sqlite` states a ceiling of
/// 1 MiB, sixteen times this.
pub const FLOOR_EVENT_DATA_LEN: usize = 65_536;

/// VT-22's `MIN_SUPPORTED_TAGS_PER_EVENT`: 64 tags.
///
/// `spec/SPECIFICATION.md:1543`, sized as eight times the maximum any of the
/// six scenarios actually uses, and chosen to keep a 100-event batch inside
/// SQLite's 32,766 variable ceiling at three parameters per tag.
pub const FLOOR_TAGS_PER_EVENT: usize = 64;

/// VT-23's `MIN_SUPPORTED_QUERY_ITEMS`: 128 items (`spec/SPECIFICATION.md:1565`).
pub const FLOOR_QUERY_ITEMS: usize = 128;

/// VT-24's `MIN_SUPPORTED_EVENTS_PER_BATCH`: 128 events
/// (`spec/SPECIFICATION.md:1586`).
pub const FLOOR_EVENTS_PER_BATCH: usize = 128;

/// The event type every corpus here uses, as a compile-time constant.
///
/// One value shared by both regimes on purpose: the *string* must be identical
/// across arms or a query written for one would not match the other, and then
/// the regime column would be measuring two different workloads. What differs
/// is only how the value is built.
const BENCH_EVENT_TYPE: &str = "BenchmarkRecorded";

/// A 64 KiB static buffer, sliced to whatever payload size an interned corpus
/// asks for.
///
/// `Bytes::from_static` over a slice of this allocates nothing and clones for
/// free, which is exactly the control [`Regime::Interned`] exists to be.
static STATIC_PAYLOAD: [u8; FLOOR_EVENT_DATA_LEN] = [0x5A; FLOOR_EVENT_DATA_LEN];

/// [`FLOOR_TAGS_PER_EVENT`] compile-time tags, for the interned regime.
///
/// Every event in an interned corpus carries a prefix of this table, which is
/// what makes such a corpus useless for any selectivity question and perfectly
/// suited to the clone and encode questions. See the module docs.
static STATIC_TAGS: [Tag; FLOOR_TAGS_PER_EVENT] = [
    Tag::from_static("bench:t00"),
    Tag::from_static("bench:t01"),
    Tag::from_static("bench:t02"),
    Tag::from_static("bench:t03"),
    Tag::from_static("bench:t04"),
    Tag::from_static("bench:t05"),
    Tag::from_static("bench:t06"),
    Tag::from_static("bench:t07"),
    Tag::from_static("bench:t08"),
    Tag::from_static("bench:t09"),
    Tag::from_static("bench:t10"),
    Tag::from_static("bench:t11"),
    Tag::from_static("bench:t12"),
    Tag::from_static("bench:t13"),
    Tag::from_static("bench:t14"),
    Tag::from_static("bench:t15"),
    Tag::from_static("bench:t16"),
    Tag::from_static("bench:t17"),
    Tag::from_static("bench:t18"),
    Tag::from_static("bench:t19"),
    Tag::from_static("bench:t20"),
    Tag::from_static("bench:t21"),
    Tag::from_static("bench:t22"),
    Tag::from_static("bench:t23"),
    Tag::from_static("bench:t24"),
    Tag::from_static("bench:t25"),
    Tag::from_static("bench:t26"),
    Tag::from_static("bench:t27"),
    Tag::from_static("bench:t28"),
    Tag::from_static("bench:t29"),
    Tag::from_static("bench:t30"),
    Tag::from_static("bench:t31"),
    Tag::from_static("bench:t32"),
    Tag::from_static("bench:t33"),
    Tag::from_static("bench:t34"),
    Tag::from_static("bench:t35"),
    Tag::from_static("bench:t36"),
    Tag::from_static("bench:t37"),
    Tag::from_static("bench:t38"),
    Tag::from_static("bench:t39"),
    Tag::from_static("bench:t40"),
    Tag::from_static("bench:t41"),
    Tag::from_static("bench:t42"),
    Tag::from_static("bench:t43"),
    Tag::from_static("bench:t44"),
    Tag::from_static("bench:t45"),
    Tag::from_static("bench:t46"),
    Tag::from_static("bench:t47"),
    Tag::from_static("bench:t48"),
    Tag::from_static("bench:t49"),
    Tag::from_static("bench:t50"),
    Tag::from_static("bench:t51"),
    Tag::from_static("bench:t52"),
    Tag::from_static("bench:t53"),
    Tag::from_static("bench:t54"),
    Tag::from_static("bench:t55"),
    Tag::from_static("bench:t56"),
    Tag::from_static("bench:t57"),
    Tag::from_static("bench:t58"),
    Tag::from_static("bench:t59"),
    Tag::from_static("bench:t60"),
    Tag::from_static("bench:t61"),
    Tag::from_static("bench:t62"),
    Tag::from_static("bench:t63"),
];

/// Which allocation regime a corpus is built in. See the module docs — this is
/// the axis worth 66× at the specification's own tag floor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Regime {
    /// Everything from a `&'static str`: `EventType::from_static`,
    /// `Tag::from_static`, `Bytes::from_static`.
    ///
    /// **The control, not the workload.** No application is in this regime; a
    /// benchmark author is, by accident, and the column exists so the accident
    /// is visible rather than published.
    Interned,
    /// Everything allocated per event, which is what a decoded payload and a
    /// domain-derived tag actually cost. **This is the regime a caller is in**,
    /// and the one every headline figure must come from.
    Owned,
}

impl Regime {
    /// Both regimes, in the order a table should print them: the workload
    /// first, its control second.
    pub const BOTH: [Self; 2] = [Self::Owned, Self::Interned];

    /// The short label a results column carries.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Interned => "interned",
            Self::Owned => "owned",
        }
    }

    /// Whether figures from this regime may stand as a headline number.
    ///
    /// `false` for [`Interned`](Self::Interned), and every emitter checks it
    /// before writing a row into `results/`. A number that cannot be a headline
    /// is still worth printing — it is the control — but it must never be the
    /// figure a reader meets first.
    pub const fn is_representative(self) -> bool {
        matches!(self, Self::Owned)
    }
}

/// The shape of one event: how big, how many tags, and in which regime.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Shape {
    payload_bytes: usize,
    tags: usize,
    regime: Regime,
}

impl Shape {
    /// Builds a shape.
    ///
    /// # Panics
    ///
    /// Panics if `payload_bytes` exceeds [`FLOOR_EVENT_DATA_LEN`] in the
    /// interned regime — [`STATIC_PAYLOAD`] is exactly that long and slicing
    /// past it would be a bounds panic later, at a call site that says nothing
    /// about why. Panics if `tags` exceeds [`FLOOR_TAGS_PER_EVENT`] in the
    /// interned regime, for the same reason against [`STATIC_TAGS`].
    ///
    /// Both are refused here, before any store is reached, so a caller's typo
    /// is a message about the corpus rather than a failure inside a timed
    /// region reported as latency.
    pub fn new(payload_bytes: usize, tags: usize, regime: Regime) -> Self {
        if regime == Regime::Interned {
            assert!(
                payload_bytes <= FLOOR_EVENT_DATA_LEN,
                "the interned regime slices one {FLOOR_EVENT_DATA_LEN}-byte \
                 static buffer, so it cannot produce a {payload_bytes}-byte \
                 payload; use Regime::Owned, which is the regime a caller is \
                 in anyway"
            );
            assert!(
                tags <= FLOOR_TAGS_PER_EVENT,
                "the interned regime holds {FLOOR_TAGS_PER_EVENT} compile-time \
                 tags, so it cannot produce {tags} of them; use Regime::Owned"
            );
        }
        Self {
            payload_bytes,
            tags,
            regime,
        }
    }

    /// How many bytes each event's payload carries.
    pub const fn payload_bytes(self) -> usize {
        self.payload_bytes
    }

    /// How many tags each event carries.
    pub const fn tags(self) -> usize {
        self.tags
    }

    /// The allocation regime.
    pub const fn regime(self) -> Regime {
        self.regime
    }

    /// The label a results row carries, e.g. `owned/1KiB/3tags`.
    pub fn label(self) -> String {
        format!(
            "{}/{}B/{}tags",
            self.regime.label(),
            self.payload_bytes,
            self.tags
        )
    }
}

/// Builds events to a [`Shape`].
///
/// Held as a value rather than exposed as free functions so a benchmark cannot
/// build half its corpus in one regime and half in another — the shape is
/// captured once, at construction, and every event this builder produces
/// carries it.
#[derive(Debug, Clone)]
pub struct Corpus {
    shape: Shape,
    /// Pre-built in the interned regime, since it is identical for every event
    /// and building it per event would measure a constant.
    interned_tags: Option<Tags>,
    /// Whether each event carries a tag naming its own ordinal.
    ///
    /// A separate field and not inferred from `interned_tags.is_none()`: a
    /// *uniform* corpus in the owned regime also has no interned tags, and
    /// conflating the two would silently give every uniform-owned event a
    /// distinct tag — which is the one difference every selectivity figure
    /// turns on.
    distinct: bool,
}

impl Corpus {
    /// A corpus in which every event carries the same tags.
    ///
    /// Valid in both regimes. Use it for the clone, encode, append-throughput
    /// and replay questions, where what the tags *are* does not matter and only
    /// what they cost does.
    pub fn uniform(shape: Shape) -> Self {
        let interned_tags = (shape.regime == Regime::Interned)
            .then(|| STATIC_TAGS[..shape.tags].iter().cloned().collect());
        Self {
            shape,
            interned_tags,
            distinct: false,
        }
    }

    /// A corpus in which each event carries a tag naming its own ordinal, on
    /// top of `shape.tags() - 1` shared ones.
    ///
    /// This is what every selectivity, query-shape and append-condition arm
    /// needs: a log in which one tag selects one event and another selects all
    /// of them.
    ///
    /// # Panics
    ///
    /// Panics if `shape` is in [`Regime::Interned`]. `Tag::from_static` takes a
    /// `&'static str`, so per-event distinct tags are impossible there without
    /// leaking — and a silently-uniform corpus would make every selectivity
    /// figure a measurement of a filter that matches everything. Panics if
    /// `shape.tags()` is zero, since a distinct corpus needs at least the
    /// ordinal tag.
    pub fn distinct(shape: Shape) -> Self {
        assert!(
            shape.regime == Regime::Owned,
            "a distinct-tag corpus is impossible in the interned regime: \
             Tag::from_static takes a &'static str, so every event would carry \
             the same tags and every selectivity figure would be measuring a \
             filter that matches the whole log. That is the trap, not a \
             limitation — see the module documentation"
        );
        assert!(
            shape.tags > 0,
            "a distinct-tag corpus needs at least the per-event ordinal tag"
        );
        Self {
            shape,
            interned_tags: None,
            distinct: true,
        }
    }

    /// Whether each event carries a tag naming its own ordinal.
    pub const fn is_distinct(&self) -> bool {
        self.distinct
    }

    /// The shape every event from this corpus has.
    pub const fn shape(&self) -> Shape {
        self.shape
    }

    /// The `ordinal`-th event.
    ///
    /// # Panics
    ///
    /// Panics if a tag or event type this builds is refused by the contract,
    /// which would mean the corpus itself is malformed — a broken measurement
    /// environment, not a finding.
    pub fn event(&self, ordinal: usize) -> Event {
        let event_type = match self.shape.regime {
            Regime::Interned => EventType::from_static(BENCH_EVENT_TYPE),
            Regime::Owned => EventType::new(BENCH_EVENT_TYPE.to_owned()).unwrap(),
        };
        Event::new(event_type, self.payload())
            .unwrap()
            .with_tags(self.tags(ordinal))
    }

    /// `count` consecutive events, starting at ordinal zero.
    pub fn batch(&self, count: usize) -> Vec<Event> {
        (0..count).map(|ordinal| self.event(ordinal)).collect()
    }

    /// The payload, built the way this corpus's regime says to.
    fn payload(&self) -> Bytes {
        match self.shape.regime {
            Regime::Interned => Bytes::from_static(&STATIC_PAYLOAD[..self.shape.payload_bytes]),
            // `vec![]` rather than `Bytes::from_static(...).to_vec()`: the point
            // of this arm is that the buffer is freshly allocated, so its first
            // clone must pay for the shared header.
            Regime::Owned => Bytes::from(vec![0x5A_u8; self.shape.payload_bytes]),
        }
    }

    /// The tag set for `ordinal`.
    fn tags(&self, ordinal: usize) -> Tags {
        if let Some(interned) = &self.interned_tags {
            return interned.clone();
        }
        if self.shape.tags == 0 {
            return Tags::empty();
        }
        let mut tags: Vec<Tag> = Vec::with_capacity(self.shape.tags);
        if self.distinct {
            // The ordinal tag goes first, and only in a distinct corpus. In a
            // uniform one it is deliberately absent, so every event matches
            // every query and the arm measures throughput rather than
            // selectivity.
            tags.push(Tag::key_value("bench", &format!("e{ordinal}")).unwrap());
        }
        for slot in tags.len()..self.shape.tags {
            tags.push(Tag::key_value("bench", &format!("t{slot:02}")).unwrap());
        }
        tags.into_iter().collect()
    }
}

/// A query over the corpus's shared tag, which every event carries.
///
/// The unselective end of the selectivity axis: it matches the whole log.
///
/// # Panics
///
/// Panics if the query is refused, which would mean this module is malformed.
pub fn query_matching_all() -> Query {
    Query::from_item(QueryItem::tagged(Tags::from_pairs([("bench", "t01")]).unwrap()).unwrap())
}

/// A query over one event's ordinal tag, in a [`Corpus::distinct`] log.
///
/// The selective end of the axis: it matches exactly one event, however long
/// the log is.
///
/// # Panics
///
/// Panics if the query is refused, which would mean this module is malformed.
pub fn query_matching_one(ordinal: usize) -> Query {
    Query::from_item(
        QueryItem::tagged(Tags::from_pairs([("bench", &*format!("e{ordinal}"))]).unwrap()).unwrap(),
    )
}

/// CF-34's own worked case: a two-item query where one item selects a handful
/// of events and the other selects millions.
///
/// `spec/SPECIFICATION.md:8712-8718` uses exactly this shape to argue that
/// complexity is a benchmark and not an assertion — *"an adapter that scans
/// where it should seek passes every rule that can be written"*. It is the one
/// workload in this module that exists because a specification clause names it.
///
/// # Panics
///
/// Panics if the query is refused, which would mean this module is malformed.
pub fn query_mixed_selectivity(selective_ordinal: usize) -> Query {
    Query::from_items([
        QueryItem::tagged(
            Tags::from_pairs([("bench", &*format!("e{selective_ordinal}"))]).unwrap(),
        )
        .unwrap(),
        QueryItem::tagged(Tags::from_pairs([("bench", "t01")]).unwrap()).unwrap(),
    ])
    .unwrap()
}

/// A query at VT-23's 128-item floor, every item selecting one event.
///
/// The arm where I-5's quadratic tag dedup and X-1's parameter-count overflow
/// both live. At `tags_per_item` = [`FLOOR_TAGS_PER_EVENT`] this is 8,192 tags
/// in one query.
///
/// # Panics
///
/// Panics if the query is refused, which would mean this module is malformed.
pub fn query_at_the_item_floor(items: usize, tags_per_item: usize) -> Query {
    let built = (0..items).map(|item| {
        let tags: Tags = (0..tags_per_item)
            .map(|slot| Tag::key_value("bench", &format!("i{item}s{slot}")).unwrap())
            .collect();
        QueryItem::tagged(tags).unwrap()
    });
    Query::from_items(built).unwrap()
}
