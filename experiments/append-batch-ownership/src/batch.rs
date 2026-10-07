//! The batches every arm is fed: one deterministic builder, two payload regimes.
//!
//! # Why two regimes, and why the second is not optional
//!
//! An owned arm can only *move* a payload whose buffer it is the sole owner of.
//! `Bytes::from(Vec<u8>)` with `len == capacity` is that case: `Vec::from(Bytes)`
//! hands the same buffer back without allocating
//! (`bytes-1.12.1/src/bytes.rs:1197-1219`, the `KIND_VEC` arm). Every other
//! `Bytes` — a `from_static`, or one whose refcount another holder keeps above
//! one — makes `Vec::from` allocate and copy, so in that regime owning the batch
//! saves nothing at all. Reporting only the first regime would be reporting the
//! one in which the owned shape cannot lose.
//!
//! # Why the tags are built with `from_pairs`
//!
//! `Tag::from_static` borrows its text and clones for free; `Tags::from_pairs`
//! owns it. The second is what an application does, and it is the regime in
//! which the borrow's per-event clone costs `t + 2` allocations
//! (`experiments/event-clone-allocations/results/raw/clone.txt`).

use std::fmt;
use std::sync::LazyLock;

use happenstance_core::bytes::Bytes;
use happenstance_core::{Event, InvalidEventType, InvalidTag, Tags};

/// The event type every measured event carries.
pub const EVENT_TYPE: &str = "AppendMeasured";

/// The one tag every event carries, and the boundary the contention scenario
/// fences on. It counts towards [`Shape::tags`].
pub const BOUNDARY: (&str, &str) = ("boundary", "shared");

/// The metadata length every event carries, so the metadata column is written
/// and compared rather than left `NULL` — the omission ES-17's `Rejects:` line
/// names.
pub const METADATA_LEN: usize = 32;

/// The largest payload the sweep uses: 256 KiB, a quarter of
/// `CloudflareFixture::MAX_EVENT_DATA_LEN`.
pub const MAX_PAYLOAD: usize = 256 * 1024;

/// The source of every static-regime payload: one buffer, built once, sliced.
/// A `static` so that a slice of it is `&'static [u8]`, which is what
/// `Bytes::from_static` requires.
static PATTERN: LazyLock<Vec<u8>> = LazyLock::new(|| filler(MAX_PAYLOAD, 0));

/// Where a payload's buffer comes from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Regime {
    /// `Bytes::from(Vec<u8>)` with `len == capacity`: unique, so a move is O(1).
    VecBacked,
    /// `Bytes::from_static`: shared, so a "move" into a `Vec` still copies.
    Static,
}

impl Regime {
    /// The label printed beside every row.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::VecBacked => "vec",
            Self::Static => "static",
        }
    }
}

/// One sweep point.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Shape {
    /// Events per append.
    pub batch: usize,
    /// Tags per event, the boundary tag included.
    pub tags: usize,
    /// Payload bytes per event.
    pub payload: usize,
    /// Where the payload and metadata buffers come from.
    pub regime: Regime,
}

impl fmt::Display for Shape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "batch={:<4} tags={:<3} payload={:<7} regime={:<6}",
            self.batch,
            self.tags,
            self.payload,
            self.regime.label()
        )
    }
}

/// Why a batch could not be built. Every variant is a bug in the sweep's
/// constants rather than a runtime condition, and is reported rather than
/// panicked on.
#[derive(Debug)]
pub enum BuildError {
    /// [`EVENT_TYPE`] was refused.
    EventType(InvalidEventType),
    /// A generated tag was refused.
    Tag(InvalidTag),
    /// A payload above [`MAX_PAYLOAD`] was asked for in the static regime.
    PayloadTooLarge(usize),
    /// A tag count of zero was asked for; every event carries [`BOUNDARY`].
    NoTags,
}

impl fmt::Display for BuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EventType(err) => write!(f, "event type refused: {err}"),
            Self::Tag(err) => write!(f, "tag refused: {err}"),
            Self::PayloadTooLarge(len) => write!(f, "payload {len} exceeds {MAX_PAYLOAD}"),
            Self::NoTags => f.write_str("every event carries the boundary tag"),
        }
    }
}

impl std::error::Error for BuildError {}

impl From<InvalidEventType> for BuildError {
    fn from(err: InvalidEventType) -> Self {
        Self::EventType(err)
    }
}

impl From<InvalidTag> for BuildError {
    fn from(err: InvalidTag) -> Self {
        Self::Tag(err)
    }
}

/// `len` non-zero-patterned bytes, varied by `seed`, with `len == capacity`.
fn filler(len: usize, seed: usize) -> Vec<u8> {
    (0..len)
        .map(|offset| u8::try_from((offset + seed) % 251).unwrap_or(u8::MAX))
        .collect()
}

/// A buffer of `len` bytes in `regime`, its contents a function of `seed`.
///
/// The `Vec` is built from an exact-size iterator, so `len == capacity` and
/// `Bytes::from` takes the promotable, movable representation.
fn buffer(regime: Regime, len: usize, seed: usize) -> Result<Bytes, BuildError> {
    match regime {
        Regime::VecBacked => Ok(Bytes::from(filler(len, seed))),
        Regime::Static => {
            let pattern: &'static [u8] = PATTERN.as_slice();
            pattern
                .get(..len)
                .map(Bytes::from_static)
                .ok_or(BuildError::PayloadTooLarge(len))
        }
    }
}

/// The tags of event `index`: [`BOUNDARY`] plus `count - 1` keyed by position.
fn tags(count: usize, index: usize) -> Result<Tags, BuildError> {
    let extra = count.checked_sub(1).ok_or(BuildError::NoTags)?;
    let keys: Vec<String> = (0..extra).map(|n| format!("k{n:03}")).collect();
    let value = format!("e{index:05}");
    let pairs =
        std::iter::once(BOUNDARY).chain(keys.iter().map(|key| (key.as_str(), value.as_str())));
    Ok(Tags::from_pairs(pairs)?)
}

/// One batch at `shape`. `variant` varies the payload bytes between batches so
/// two appends of "the same" shape are not byte-identical rows.
///
/// # Errors
///
/// Returns [`BuildError`] if the shape asks for something the constants above
/// cannot supply.
pub fn build(shape: Shape, variant: usize) -> Result<Vec<Event>, BuildError> {
    (0..shape.batch)
        .map(|index| {
            let seed = variant.wrapping_add(index);
            Ok(
                Event::new(EVENT_TYPE, buffer(shape.regime, shape.payload, seed)?)?
                    .with_tags(tags(shape.tags, index)?)
                    .with_metadata(buffer(shape.regime, METADATA_LEN, seed.wrapping_add(7))?),
            )
        })
        .collect()
}
