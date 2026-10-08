//! The projection store port.
//!
//! # Status: frozen, and how that was earned
//!
//! Like [`EventStore`](crate::EventStore), this port is **frozen** and inside
//! the crate's semver promise, since ADR-0063. It was not, from `0.2.0` until
//! that decision: it shipped behind an off-by-default `unstable-projection`
//! feature with a written exemption, and the reason it did is worth keeping
//! here because it is the reason the freeze means something now.
//!
//! The bar the specification set (PS-2) was not a count of adapters but a
//! **spread**: the suite green against two adapters at opposite ends of the
//! batch-shape axis — one whose batch is an owned write set replayed at commit,
//! and one whose batch *is* a live transaction. Every implementation there had
//! ever been sat at the first end, and ADR-0060 found the second was not merely
//! unbuilt but unreachable through this port's own signatures: `begin` was
//! total, synchronous and infallible while every route to a real driver's
//! transaction is `async` and fallible; and the probe seam the suite drives a
//! read model through was synchronous, infallible and took `&Self::Batch`,
//! while a driver borrows its connection mutably to issue a statement. A
//! live-transaction store could implement the port only by declaring
//! `READS_THROUGH_BATCH = false` — a false statement about itself — so the
//! suite could not tell the two ends apart. A skeleton did not report any of
//! this because `todo!()` has type `!` and coerces to anything.
//!
//! ADR-0062 moved those signatures: `begin` is `async` and fallible, and the
//! three batch-touching probe members take `&mut Self::Batch` and return a
//! future of a `Result`. `happenstance-postgres` then built the far end — a
//! store whose batch is a `sqlx::Transaction`, declaring `READS_THROUGH_BATCH =
//! true` truthfully — and it passed all seventeen rules against a live server,
//! with PS-12's read-through rule reported as a run rather than a skip for the
//! first time on any adapter over a real database. The two ends disagreed about
//! nothing the port had to move for. That is the measurement PS-2 asked for,
//! and ADR-0063 is the freeze taken on it.
//!
//! **What is frozen is this port.** The typed layer's `Projection::apply` is
//! synchronous, so an application can push into a buffered batch and cannot
//! issue a statement into a live one; the runner built over this port stays
//! behind `happenstance`'s own `unstable-projection` for that reason, and it is
//! the typed layer's axis rather than this port's.
//!
//! Changing a signature here is now a breaking change with a decision record
//! behind it, which is what *frozen* means in this workspace.
//!
//! # The invariant that drives the design
//!
//! A read model and its checkpoint must move together. If the read-model write
//! commits and the checkpoint write does not, a restart replays events that
//! were already applied; if the checkpoint commits first, a crash silently
//! skips events. Neither is acceptable, and no amount of ordering or retrying
//! fixes it — the two writes must be **one** transaction.
//!
//! So the port cannot offer `apply()` and `set_checkpoint()` as independent
//! calls. It hands out an adapter-owned batch and takes it back at commit time
//! together with the position.
//!
//! # What is out of scope at 0.1: a projection that writes back into the log
//!
//! A projection that emits events into the event store as a side effect of
//! applying one is **out of scope for this port at 0.1**, and this paragraph is
//! the port saying so rather than leaving it to be discovered
//! (`spec/SPECIFICATION.md` §4's PS-31).
//!
//! It is a consequence of a decision already taken, not a question still open.
//! An `EventId` is a store-assigned `(StoreId, SequencePosition)` pair minted at
//! append (VT-5), and [`EventStore::append`](crate::EventStore::append) refuses a
//! caller-supplied one (VT-10). An outward-writing projection needs exactly what
//! VT-10 refuses: a write whose identity the *caller* chooses, so that a rebuild
//! re-emitting the same event is a no-op rather than a second fact. Without that,
//! every rebuild duplicates every emitted event, which is the wrong
//! implementation silence here produces. Revisiting it means a deliberate
//! idempotent-emission seam, and VT-5 is what would make one expressible.
//!
//! # The batch is owned, and is not required to be a live transaction
//!
//! [`ProjectionStore::Batch`] is a plain associated type with **no lifetime
//! parameter**. It used to be a generic associated type borrowing from the
//! store, on the reasoning that a transaction cannot outlive its connection.
//! Two compiled results retired that shape and neither is about `Send`: the
//! `error[E0195]` every impl hit, written out once beside the doctest that
//! disposes of it in [`# Implementing it`](ProjectionStore#implementing-it),
//! and a store carrying a lifetime of its own, which made rustc 1.97.1 *ICE*
//! while reporting the region error the GAT's `where Self: 'a` produced. A
//! batch may still *be* a live transaction — the port stops requiring one,
//! which is what lets an adapter with no connection at all implement it.
//!
//! # The `Send` flavour's extra requirement is documented, not declared
//!
//! [`SendProjectionStore`] transitively requires `Batch: Send`, because the
//! batch crosses an await between [`begin`](ProjectionStore::begin) and
//! [`commit`](ProjectionStore::commit). That bound cannot be written on the
//! associated type: `trait_variant` copies associated-type bounds **verbatim**
//! into the bare flavour, so `type Batch: Send;` would impose `Send` on the
//! flavour that exists precisely for `wasm32`, where nothing is. A caller who
//! needs a spawnable runner writes `S::Batch: Send` in their own `where`
//! clause.
//!
//! Projections that cannot be made transactional with their checkpoint must
//! instead be made **idempotent**, so that replaying an event is harmless. That
//! is a property of the projection, not of this port.
//!
//! # Naming what the port hands back
//!
//! ```
//! use happenstance_core::{
//!     Authority, Checkpoint, CommitError, ProjectionId, ResetError, SequencePosition,
//! };
//!
//! // Every state the port can report is one a caller can name.
//! let checkpoint = Checkpoint::Live {
//!     through: SequencePosition::FIRST,
//! };
//! assert!(matches!(checkpoint, Checkpoint::Live { .. }));
//!
//! // `commit` and `reset` fail differently, so a caller matching one is never
//! // offered the other's arms.
//! let refused: ResetError<core::convert::Infallible> = ResetError::Refused;
//! let foreign: CommitError<core::convert::Infallible> = CommitError::ForeignBatch;
//! assert!(matches!(refused, ResetError::Refused));
//! assert!(matches!(foreign, CommitError::ForeignBatch));
//!
//! let _rebuild = (ProjectionId::from_static("van_stock"), Authority::Rebuilding);
//! ```

use alloc::borrow::Cow;
use alloc::string::String;
#[cfg(feature = "conformance")]
use core::future::Future;

use crate::event::SequencePosition;
use crate::identity::StoreId;
use crate::validate;

/// Longest permitted projection identifier, in bytes.
///
/// The same bound as [`MAX_EVENT_TYPE_LEN`](crate::MAX_EVENT_TYPE_LEN) and
/// [`MAX_TAG_LEN`](crate::MAX_TAG_LEN), and it applies to every
/// [`ProjectionId`], including one a runner derives from a projection's name
/// (ADR-0082 §D5).
pub const MAX_PROJECTION_ID_LEN: usize = 255;

/// The prefix [`ProjectionId::sync_watermark`] renders, and one of the two
/// [`RESERVED`] entries.
const SYNC_PREFIX: &str = "sync/";

/// Prefixes no caller-supplied id may begin with, matched as exact bytes.
///
/// Private: the list is documented on [`ProjectionId::new`] and in VT-35, and
/// widening it later refuses ids that are valid today, which is a behaviour
/// break whatever this constant's visibility.
const RESERVED: [&str; 2] = ["happenstance/", SYNC_PREFIX];

/// Names a read model within a projection store.
///
/// Distinct projections advance independently, so each needs its own
/// checkpoint, and the id is that checkpoint's key: a store keeps it byte for
/// byte (PS-39).
///
/// Validated, like its siblings [`EventType`](crate::EventType) and
/// [`Tag`](crate::Tag), by the same rules plus one (VT-35, ADR-0082). There
/// is no infallible way in, because two constructors enforcing different rules
/// is the defect that makes an invalid value reachable through the weaker one.
///
/// # Examples
///
/// ```
/// use happenstance_core::ProjectionId;
///
/// let id = ProjectionId::new("van_stock")?;
/// assert_eq!(id.as_str(), "van_stock");
/// # Ok::<(), happenstance_core::InvalidProjectionId>(())
/// ```
///
/// A string does not become an id by conversion alone. There is no
/// `From<&str>`, because an infallible door would bypass the rules:
///
/// ```compile_fail
/// use happenstance_core::ProjectionId;
///
/// let id: ProjectionId = "van_stock".into();
/// # let _ = id;
/// ```
///
/// Backed by `Cow<'static, str>` rather than `Box<str>` so that
/// [`from_static`](Self::from_static) can build one in a `const` without
/// allocating, the trade VT-32 records for [`EventType`](crate::EventType).
#[derive(Debug, Clone)]
pub struct ProjectionId(Cow<'static, str>);

impl ProjectionId {
    /// Creates a projection identifier.
    ///
    /// The value is kept exactly as given: nothing is trimmed, folded or
    /// normalised, so an accepted id's [`as_str`](Self::as_str) is byte for
    /// byte its input.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidProjectionId`], checking in this order, if the value is
    /// empty; longer than [`MAX_PROJECTION_ID_LEN`] bytes; contains a character
    /// in Unicode general category `Cc` or one of the explicit bidirectional
    /// formatting controls U+202A–U+202E or U+2066–U+2069, reporting whichever
    /// comes first; or begins with one of the reserved prefixes `happenstance/`
    /// and `sync/`, compared as exact bytes. `Cf` in general is accepted, so the
    /// joiners U+200C and U+200D that Persian, Hindi and emoji sequences need
    /// are kept.
    ///
    /// A `sync/` id is built only by [`sync_watermark`](Self::sync_watermark).
    pub fn new(value: impl Into<String>) -> Result<Self, InvalidProjectionId> {
        let value = value.into();
        match refusal(&value) {
            None => Ok(Self(Cow::Owned(value))),
            Some(refused) => Err(refused),
        }
    }

    /// Creates a projection identifier from a string literal, validating at
    /// compile time.
    ///
    /// Enforces exactly the rules [`new`](Self::new) enforces — one function
    /// checks both — so a `ProjectionId` is always a validated value, with some
    /// of that validation having happened before the program ran.
    ///
    /// # Panics
    ///
    /// Panics if the value would be rejected by [`new`](Self::new). **Where
    /// that panic surfaces depends on the call site, and the difference is
    /// sharp enough to be worth stating:**
    ///
    /// | Call site | When the invalid value is caught |
    /// |---|---|
    /// | a free `const` | `cargo check`, as `error[E0080]` |
    /// | an associated `const` that is read somewhere | `cargo build` |
    /// | an associated `const` that is never read | **never** |
    /// | a `let` binding | at run time, as a panic |
    ///
    /// The third row is the one to design around: an associated const is
    /// evaluated lazily, so an invalid one that nothing reads survives `check`,
    /// `clippy`, `build` and `test`. Prefer a free `const` for anything whose
    /// validity you want the compiler to guarantee:
    ///
    /// ```
    /// use happenstance_core::ProjectionId;
    ///
    /// const VAN_STOCK: ProjectionId = ProjectionId::from_static("van_stock");
    /// # let _ = VAN_STOCK;
    /// ```
    ///
    /// An invalid value at the same free `const` site is a compile error, and
    /// a reserved prefix is invalid:
    ///
    /// ```compile_fail
    /// use happenstance_core::ProjectionId;
    ///
    /// const X: ProjectionId = ProjectionId::from_static("sync/x");
    /// # let _ = X;
    /// ```
    ///
    /// Spelled bare `compile_fail` rather than `compile_fail,E0080`: rustdoc on
    /// 1.97.1 silently ignores an error-code annotation it cannot match, so the
    /// stricter-looking spelling is the weaker check. It is also deliberately
    /// weaker than a `trybuild` snapshot — it does not pin the diagnostic — and
    /// ADR-0015 records that phase 6 owns the `trybuild` dependency decision.
    /// The passing example above it is the control: the two differ only in the
    /// literal, so the failure is the validator's and nothing else's.
    #[must_use]
    pub const fn from_static(value: &'static str) -> Self {
        match refusal(value) {
            None => Self(Cow::Borrowed(value)),
            #[expect(
                clippy::panic,
                reason = "the sanctioned const construction path (VT-35, as VT-32's \
                          `EventType::from_static`): at a free `const` this panic is a \
                          compile error, and `new` is the fallible door"
            )]
            Some(refused) => panic!("{}", refused.const_message()),
        }
    }

    /// The replication watermark for `peer`: `sync/` followed by the peer's
    /// [`StoreId`] as 32 lowercase hex digits, 37 bytes in all.
    ///
    /// The **only** constructor of a `sync/` id (SY-31, VT-35). Every other one
    /// refuses the prefix, so no application projection can collide with a
    /// watermark, and the reservation is enforced rather than advised: there is
    /// no unchecked constructor behind it.
    ///
    /// The rendered format is part of VT-35 and frozen, because changing it
    /// orphans every persisted watermark. A peer restored from a backup
    /// re-mints its `StoreId` and so gets a new watermark, which is correct: a
    /// new incarnation is new history.
    ///
    /// ```
    /// use happenstance_core::{ProjectionId, StoreId};
    ///
    /// let peer = StoreId::from_bytes([0xab; 16]);
    /// let watermark = ProjectionId::sync_watermark(peer);
    /// assert_eq!(watermark.as_str(), "sync/abababababababababababababababab");
    /// assert!(ProjectionId::new(watermark.as_str()).is_err());
    /// ```
    #[must_use]
    pub fn sync_watermark(peer: StoreId) -> Self {
        Self(Cow::Owned(alloc::format!("{SYNC_PREFIX}{peer}")))
    }

    /// The identifier as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// The one validator both of [`ProjectionId`]'s checking doors call (RS-11-1):
/// VT-14's rules through [`validate::check`], then the reserved prefixes.
const fn refusal(value: &str) -> Option<InvalidProjectionId> {
    match validate::check(value, MAX_PROJECTION_ID_LEN) {
        validate::Refusal::Accepted => match reserved_prefix(value) {
            Some(prefix) => Some(InvalidProjectionId::Reserved { prefix }),
            None => None,
        },
        validate::Refusal::Empty => Some(InvalidProjectionId::Empty),
        validate::Refusal::TooLong => Some(InvalidProjectionId::TooLong { len: value.len() }),
        validate::Refusal::ControlCharacter => Some(InvalidProjectionId::ControlCharacter),
        validate::Refusal::BidirectionalControl => Some(InvalidProjectionId::BidirectionalControl),
    }
}

/// The [`RESERVED`] entry `value` begins with, compared as exact bytes.
///
/// `while` loops rather than `starts_with`, which is not `const` (RS-11-3).
/// Every index is below a length checked first, the shape `validate.rs` uses.
const fn reserved_prefix(value: &str) -> Option<&'static str> {
    let bytes = value.as_bytes();
    let mut k = 0;
    while k < RESERVED.len() {
        let prefix = RESERVED[k].as_bytes();
        if prefix.len() <= bytes.len() {
            let mut i = 0;
            let mut matched = true;
            while i < prefix.len() {
                if bytes[i] != prefix[i] {
                    matched = false;
                    break;
                }
                i += 1;
            }
            if matched {
                return Some(RESERVED[k]);
            }
        }
        k += 1;
    }
    None
}

// `Eq`, `Ord` and `Hash` are written out rather than derived, for the reason
// `EventType`'s are: `Borrow<str>` promises the borrowed form hashes and
// compares identically to the owner, and these bodies are where that promise
// is discharged.
impl PartialEq for ProjectionId {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}

impl Eq for ProjectionId {}

impl PartialOrd for ProjectionId {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ProjectionId {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl core::hash::Hash for ProjectionId {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.as_str().hash(state);
    }
}

impl core::borrow::Borrow<str> for ProjectionId {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<str> for ProjectionId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl core::fmt::Display for ProjectionId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl TryFrom<&str> for ProjectionId {
    type Error = InvalidProjectionId;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<String> for ProjectionId {
    type Error = InvalidProjectionId;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl core::str::FromStr for ProjectionId {
    type Err = InvalidProjectionId;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// A [`ProjectionId`] failed validation (VT-35).
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum InvalidProjectionId {
    /// The id was empty. It is the primary key of a checkpoint row, and an
    /// empty key names nothing.
    #[error("a projection id must not be empty")]
    Empty,
    /// The id exceeded [`MAX_PROJECTION_ID_LEN`] bytes.
    #[error("a projection id must be at most {max} bytes, got {len}", max = MAX_PROJECTION_ID_LEN)]
    TooLong {
        /// The rejected length, in bytes.
        len: usize,
    },
    /// The id contained a control character (Unicode `Cc`), NUL among them,
    /// which a Postgres `text` column refuses at `commit`.
    #[error("a projection id must not contain control characters")]
    ControlCharacter,
    /// The id contained one of the explicit bidirectional formatting controls:
    /// U+202A–U+202E or U+2066–U+2069.
    ///
    /// Separate from [`ControlCharacter`](Self::ControlCharacter) for the reason
    /// [`InvalidTag::BidirectionalControl`](crate::InvalidTag::BidirectionalControl)
    /// gives.
    #[error("a projection id must not contain bidirectional formatting controls")]
    BidirectionalControl,
    /// The id began with a reserved prefix, compared as exact bytes.
    ///
    /// `sync/` belongs to SY-31's replication watermark and is minted only by
    /// [`ProjectionId::sync_watermark`]. `happenstance/` is held for ids this
    /// library may mint later, so that minting one is not a breaking change.
    #[error("a projection id must not begin with the reserved prefix `{prefix}`")]
    Reserved {
        /// The reserved prefix the value began with, verbatim.
        prefix: &'static str,
    },
}

impl InvalidProjectionId {
    /// The message [`ProjectionId::from_static`] panics with: a literal per
    /// variant, because a `const` panic cannot format a field.
    const fn const_message(&self) -> &'static str {
        match self {
            Self::Empty => "a projection id must not be empty",
            Self::TooLong { .. } => "a projection id must be at most MAX_PROJECTION_ID_LEN bytes",
            Self::ControlCharacter => "a projection id must not contain control characters",
            Self::BidirectionalControl => {
                "a projection id must not contain bidirectional formatting controls"
            }
            Self::Reserved { .. } => {
                "a projection id must not begin with a reserved prefix (`happenstance/` or `sync/`)"
            }
        }
    }
}

impl From<core::convert::Infallible> for InvalidProjectionId {
    /// Lets a caller accept `impl TryInto<ProjectionId, Error: Into<InvalidProjectionId>>`
    /// and so take both a `&str`, which converts fallibly, and an already-built
    /// [`ProjectionId`], whose conversion cannot fail, with no second error
    /// type.
    ///
    /// The match has no arms because [`Infallible`](core::convert::Infallible)
    /// has no values, which the compiler accepts as exhaustive.
    fn from(never: core::convert::Infallible) -> Self {
        match never {}
    }
}

/// Where a projection has been brought to, and whether its rows can be trusted.
///
/// An enum rather than `(Option<SequencePosition>, bool)` because the tuple can
/// spell `(None, true)` — authoritative, never run — which means nothing. That
/// is this crate's "illegal states are unrepresentable" line, applied where a
/// reader would otherwise write `if let Some(p) = checkpoint` and get it wrong.
/// A bare `Option<SequencePosition>`, the weaker alternative, cannot distinguish
/// a rebuild in flight from an authoritative read model at all.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Checkpoint {
    /// Never run, or reset, or rebuilding with nothing committed yet.
    ///
    /// Distinct from a checkpoint at the first position: an operator who
    /// substitutes `commit(empty_batch, id, FIRST, _)` for "never run" makes a
    /// runner resume *after* event 1, which is then skipped permanently and
    /// silently.
    NeverRun,

    /// Considered through `through`, and the read model is authoritative.
    Live {
        /// The position this projection has considered up to. A runner resumes
        /// strictly after it.
        through: SequencePosition,
    },

    /// A rebuild is in flight, considered through `through`. Rows are **not**
    /// authoritative.
    Rebuilding {
        /// The position the rebuild has considered up to.
        through: SequencePosition,
    },
}

/// What a commit claims about the rows it leaves behind.
///
/// Taken by value at [`commit`](ProjectionStore::commit) so the claim is made at
/// write time and [`checkpoint`](ProjectionStore::checkpoint) can report it
/// without inferring it. A rebuild in place therefore cannot be spelled as an
/// ordinary commit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Authority {
    /// The rows this commit leaves behind are authoritative.
    Live,
    /// A rebuild is in flight; the rows are not yet authoritative.
    Rebuilding,
}

/// Why a [`commit`](ProjectionStore::commit) failed.
///
/// Generic over the adapter's own error rather than carrying a `String`, for the
/// same reason [`AppendError`](crate::AppendError) is: a caller must be able to
/// tell a port-level outcome from an adapter failure without pattern-matching on
/// text. `#[non_exhaustive]`, so a match needs a wildcard arm.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum CommitError<E> {
    /// The batch was begun on a different store instance.
    ///
    /// Rejected at run time rather than at compile time, and deliberately.
    /// Tying the batch to the receiver's lifetime was compiled and refuted: a
    /// lifetime names a *region*, not an *instance*, so two `&Store` references
    /// unify to a common region and `b.commit(a.begin(), …)` still type-checks.
    /// The only type-level fix is a generative brand, which forbids the batch
    /// escaping the closure that begins it — defeating the caller the hazard is
    /// about. So an adapter stamps an identity minted per store instance at
    /// [`begin`](ProjectionStore::begin) and compares it here; with an owned
    /// batch the stamp is a field and the check is an integer comparison.
    #[error("the batch was begun on a different store instance")]
    ForeignBatch,

    /// `position` is below the checkpoint already recorded.
    ///
    /// Both values are carried so a caller can log the gap rather than re-derive
    /// it with a second round trip.
    #[error("checkpoint regression: {current} is recorded, {attempted} was attempted")]
    CheckpointRegression {
        /// The checkpoint the store already holds.
        current: SequencePosition,
        /// The position the caller tried to move it to.
        attempted: SequencePosition,
    },

    /// The adapter failed for its own reasons.
    #[error(transparent)]
    Store(E),
}

/// Why a [`reset`](ProjectionStore::reset) failed.
///
/// Separate from [`CommitError`] so a caller matching `commit`'s result never
/// has to consider [`Refused`](Self::Refused), which `commit` cannot produce.
/// One merged `ProjectionError<E>` was the alternative and it lost:
/// `#[non_exhaustive]` already forces a wildcard arm without also forcing dead
/// ones.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum ResetError<E> {
    /// The batch was begun on a different store instance.
    ///
    /// See [`CommitError::ForeignBatch`] for why this is a run-time rejection.
    #[error("the batch was begun on a different store instance")]
    ForeignBatch,

    /// This store declines to reset this projection.
    ///
    /// The variant is bare on purpose. The port supplies the mechanism and the
    /// domain decides what to protect, so a `&'static str` here would push a
    /// domain sentence through a port type that cannot validate it, cannot
    /// localise it and cannot keep it in step with the policy that produced it.
    /// An operator learns *that* the reset was declined from this variant, and
    /// *which policy* declined it from the store that holds the policy. A
    /// refusal is not success: it leaves the rows and the checkpoint unchanged.
    #[error("this store declines to reset this projection")]
    Refused,

    /// The adapter failed for its own reasons.
    #[error(transparent)]
    Store(E),
}

/// A store that holds read models and their replay checkpoints.
///
/// See the [module documentation](self) for the transactional invariant this
/// shape exists to enforce, for why the batch is owned, and for how its freeze
/// was earned.
///
/// Two traits, one set of doc attributes. `trait_variant` derives
/// [`SendProjectionStore`] from [`ProjectionStore`] and copies this block onto
/// it, so every sentence here is written to hold on whichever of the two pages
/// you opened. [`ProjectionStore`] states no `Send` requirement and is what
/// generic code binds; [`SendProjectionStore`] adds it and hands back
/// [`ProjectionStore`] free. The same split as
/// [`EventStore`](crate::EventStore), for the same reason.
///
/// # Implementing it
///
/// The whole port, on a store small enough to read at a glance. Note what is
/// *absent*: there is no `Self::Batch<'_>` anywhere, because there is no
/// lifetime to spell. An implementer writes `type Batch = MyBatch;` and then
/// `async fn commit(&self, batch: MyBatch, …)`, and it compiles.
///
/// That used to be the trap that explained why this port had no adapters. While
/// `Batch` was a generic associated type, the trait declared
/// `batch: Self::Batch<'_>`, so naming the concrete type in the impl declared a
/// different set of lifetime generics and rustc answered
/// `error[E0195]: lifetime parameters or bounds on method 'commit' do not match
/// the trait declaration` — with nothing in the workspace saying the literal
/// `Self::Batch<'_>` was required, and nothing to copy. This example is the
/// disposition of that trap: it compiles on every CI run, so it cannot rot back.
///
/// ```
/// use happenstance_core::{
///     Authority, Checkpoint, CommitError, ProjectionId, ProjectionStore, ResetError,
///     SequencePosition,
/// };
///
/// /// The adapter's own batch. Owned, and not a live transaction.
/// #[derive(Debug, Default)]
/// struct ToyBatch {
///     rows: Vec<(String, u64)>,
/// }
///
/// #[derive(Debug, Default)]
/// struct ToyStore {
///     committed: std::cell::RefCell<Vec<(String, u64)>>,
///     checkpoint: std::cell::Cell<Option<SequencePosition>>,
/// }
///
/// impl ProjectionStore for ToyStore {
///     type Error = ToyError;
///
///     // No lifetime, and no `where Self: 'a`.
///     type Batch = ToyBatch;
///
///     // `async` and fallible on the port because a batch that is a live
///     // transaction has a round trip and a failure here. A buffer has neither:
///     // this body never yields and never fails, and costs what it looks like.
///     async fn begin(&self) -> Result<ToyBatch, ToyError> {
///         Ok(ToyBatch::default())
///     }
///
///     async fn checkpoint(&self, _id: &ProjectionId) -> Result<Checkpoint, ToyError> {
///         Ok(match self.checkpoint.get() {
///             None => Checkpoint::NeverRun,
///             Some(through) => Checkpoint::Live { through },
///         })
///     }
///
///     // The concrete type, spelled straight out. This is the line that used
///     // to be `error[E0195]`.
///     async fn commit(
///         &self,
///         batch: ToyBatch,
///         _id: &ProjectionId,
///         position: SequencePosition,
///         _authority: Authority,
///     ) -> Result<(), CommitError<ToyError>> {
///         // The rows and the checkpoint, or neither.
///         self.committed.borrow_mut().extend(batch.rows);
///         self.checkpoint.set(Some(position));
///         Ok(())
///     }
///
///     async fn reset(
///         &self,
///         batch: ToyBatch,
///         _id: &ProjectionId,
///     ) -> Result<(), ResetError<ToyError>> {
///         // The caller's batch carries the deletes; this store's whole read
///         // model is the vector, so clearing it is the same unit of work.
///         drop(batch);
///         self.committed.borrow_mut().clear();
///         self.checkpoint.set(None);
///         Ok(())
///     }
///
///     async fn rollback(&self, batch: ToyBatch) -> Result<(), ToyError> {
///         drop(batch);
///         Ok(())
///     }
/// }
///
/// #[derive(Debug)]
/// struct ToyError;
/// impl core::fmt::Display for ToyError {
///     fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
///         f.write_str("the toy store failed")
///     }
/// }
/// impl core::error::Error for ToyError {}
///
/// # #[tokio::main(flavor = "current_thread")]
/// # async fn main() -> Result<(), Box<dyn core::error::Error>> {
/// let store = ToyStore::default();
/// let id = ProjectionId::new("toy")?;
///
/// let mut batch = store.begin().await?;
/// batch.rows.push(("depot-7".to_owned(), 12));
/// store
///     .commit(batch, &id, SequencePosition::FIRST, Authority::Live)
///     .await?;
///
/// assert_eq!(
///     store.checkpoint(&id).await?,
///     Checkpoint::Live { through: SequencePosition::FIRST },
/// );
/// # Ok(())
/// # }
/// ```
#[trait_variant::make(SendProjectionStore: Send)]
pub trait ProjectionStore {
    /// How this adapter fails.
    type Error: core::error::Error + 'static;

    /// The adapter's write set.
    ///
    /// **Owned, and not required to be a live transaction.** Only the adapter
    /// knows what it is: a SQL transaction, a buffered statement list, a graph
    /// write set. Applying an event mutates it; it becomes durable only when
    /// handed to [`commit`](ProjectionStore::commit) or
    /// [`reset`](ProjectionStore::reset).
    ///
    /// No `Send` bound is written here, and it is not an oversight.
    /// [`SendProjectionStore`] transitively requires `Batch: Send`, but
    /// `trait_variant` copies associated-type bounds verbatim into the bare
    /// flavour — so `type Batch: Send;` would impose `Send` on the flavour that
    /// exists for `wasm32`, where nothing is. The requirement is documented
    /// here and written by the caller as `S::Batch: Send` when they need it.
    type Batch;

    /// Opens a write set.
    ///
    /// `async` and fallible, and it was neither until ADR-0062. The old shape
    /// argued that an `async fn begin() -> Result<…>` *implies a round trip*,
    /// which an adapter on a one-shot HTTP transport cannot afford. It does
    /// not imply one: an `async fn` whose body never awaits is a future that
    /// is ready at its first poll, and `Ok(Vec::new())` is what every buffering
    /// adapter in this workspace writes here. What the old shape *forbade* was
    /// a batch that is a live transaction, because every route to a real
    /// driver's transaction is `async` and fallible — `sqlx`'s
    /// `Pool::begin().await?` has no synchronous spelling and private fields —
    /// and PS-6's own falsifier, *an adapter that must reserve something from
    /// the server before the first write*, is exactly that `BEGIN`.
    ///
    /// The discipline the old signature enforced is now the implementer's:
    /// **an adapter with nothing to reserve MUST NOT spend a round trip here.**
    /// A transport that can count its requests can assert it, and
    /// `happenstance-neon` does.
    ///
    /// # Errors
    ///
    /// Returns the adapter's error if a write set could not be opened — a
    /// connection that could not be acquired, a `BEGIN` that was refused. A
    /// buffering adapter has no failing path and returns `Ok` unconditionally.
    async fn begin(&self) -> Result<Self::Batch, Self::Error>;

    /// How far this projection has been brought, and whether its rows are
    /// authoritative.
    ///
    /// Returns a [`Checkpoint`], never an `Option<SequencePosition>`; the
    /// reason the tuple form lost is recorded on that type. Resume a replay by
    /// feeding `through` to [`ReadOptions::from`](crate::ReadOptions::from)
    /// after advancing past it.
    ///
    /// PS-38, provisional: the answer reflects every commit this store has
    /// acknowledged, through any handle onto it, and an adapter over replicated
    /// storage answers from the primary, never from a replica that may lag it —
    /// a runner that resumes from a stale checkpoint applies again what it
    /// already committed. No conformance rule can observe a lagging replica in
    /// process, so this is documented rather than checked, and if PS-38's
    /// falsifier fires the obligation narrows to reads through the same handle.
    ///
    /// # Errors
    ///
    /// Returns the adapter's error if the checkpoint cannot be read.
    async fn checkpoint(&self, id: &ProjectionId) -> Result<Checkpoint, Self::Error>;

    /// Applies `batch` and moves `id`'s checkpoint to `position`, as one unit.
    ///
    /// `position` is the position *considered*, not the position applied, so a
    /// batch that wrote nothing still advances the checkpoint. `authority` is
    /// the claim this commit makes about the rows it leaves behind.
    ///
    /// # Errors
    ///
    /// Returns [`CommitError::ForeignBatch`] if `batch` was begun on a different
    /// store instance, [`CommitError::CheckpointRegression`] if `position` is
    /// below the checkpoint already recorded, and [`CommitError::Store`] if the
    /// adapter itself failed. The batch is consumed either way, and a failed
    /// commit leaves both the read model and the checkpoint unchanged.
    async fn commit(
        &self,
        batch: Self::Batch,
        id: &ProjectionId,
        position: SequencePosition,
        authority: Authority,
    ) -> Result<(), CommitError<Self::Error>>;

    /// Applies `batch` and returns `id` to [`Checkpoint::NeverRun`], as one
    /// unit.
    ///
    /// The dual of [`commit`](ProjectionStore::commit): the caller's own batch
    /// carries the deletes, because **the port has no idea what the read model
    /// is**. The alternative — a `reset` that clears the rows itself — would
    /// oblige the adapter to know which tables belong to a [`ProjectionId`],
    /// which is exactly the knowledge this port keeps out.
    ///
    /// # Errors
    ///
    /// Returns [`ResetError::ForeignBatch`] if `batch` was begun on a different
    /// store instance, [`ResetError::Refused`] if this store declines to reset
    /// this projection, and [`ResetError::Store`] if the adapter itself failed.
    /// A refusal, like a failure, leaves both halves unchanged.
    async fn reset(
        &self,
        batch: Self::Batch,
        id: &ProjectionId,
    ) -> Result<(), ResetError<Self::Error>>;

    /// Discards the batch without committing.
    ///
    /// Exists because `Drop` cannot await: an adapter holding a real resource
    /// needs somewhere to release it. Dropping a batch bare must also roll back
    /// and leave the store usable.
    ///
    /// # Errors
    ///
    /// Returns the adapter's error if the rollback fails.
    async fn rollback(&self, batch: Self::Batch) -> Result<(), Self::Error>;
}

/// The write seam the projection conformance suite runs through.
///
/// Without it, generic code holding an adapter's batch can do exactly two things
/// with it — commit it or roll it back. There is no way to put a row *into* the
/// batch and no way to look at the read model afterwards, so the rule carrying
/// this port's entire reason for existing — the read-model write and the
/// checkpoint write become durable together or not at all — cannot be written,
/// and a suite built on [`ProjectionStore`] alone degenerates into a checkpoint
/// test that a store writing *only* checkpoints passes.
///
/// An adapter that wants to be certified implements this beside its
/// [`ProjectionStore`] impl. It is not part of the runtime surface: it exists so
/// a suite that has never heard of the adapter can drive its read model.
///
/// # Why this lives in the contract crate rather than the testkit
///
/// It looks like a testing utility and belongs beside the port anyway, and the
/// reason is coherence rather than taste. An adapter crate implementing a
/// *testkit* trait for its own type is legal — the type is local, so the orphan
/// rule is satisfied. But the natural place to write such an impl is the
/// adapter's own `tests/` directory, and **that is a different crate**. There,
/// neither the trait nor the type is local, `impl ProjectionProbe for MyStore`
/// is rejected by the orphan rule, and the only way out is a **non-dev**
/// dependency on the testkit, feature-gated. Putting the trait here keeps the
/// impl in the adapter's own `src/`, beside the store and behind the adapter's
/// own feature, and costs one flag on a dependency it already has rather than a
/// new edge in its graph:
///
/// ```toml
/// [dependencies]
/// # No feature: the port and every item the `impl ProjectionStore` below
/// # names are unconditional since ADR-0063. (From `0.2.0` until then this
/// # line had to carry `unstable-projection`, because the port impl in an
/// # adapter's `src/` cannot be made optional and the port was gated.)
/// happenstance-core = "…"
///
/// [features]
/// # Forwards to the contract crate. The `impl ProjectionProbe` lives in `src/`
/// # under `#[cfg(feature = "conformance")]`, because a crate cannot `cfg` on a
/// # dependency's feature — which is also why a `[dev-dependencies]` entry
/// # would not work: it does not exist for the lib build the impl compiles in.
/// conformance = ["happenstance-core/conformance"]
///
/// [dev-dependencies]
/// happenstance-testkit = "…"
/// ```
///
/// **Nothing inside this workspace can fail the wrong version of that
/// decision.** Every fixture here already lives in a crate that depends on the
/// testkit, so the trait would be local, the impls local, the orphan rule
/// silent, and the whole gate green. The falsifier is
/// `documented-extension-surface` (HS-S0015), which builds an outside author's
/// fixture from this documentation alone — named here so the placement is not
/// "simplified" into the testkit on grounds of diff size before it runs.
///
/// # Both flavours, one trait
///
/// The bound is bare [`ProjectionStore`], and there is deliberately no
/// `SendProjectionProbe`. `trait_variant` emits a blanket impl, so
/// [`SendProjectionStore`] implies [`ProjectionStore`] and a `Send` adapter
/// already satisfies this supertrait bound. A second trait would collide with
/// that blanket impl — `error[E0275]` — which is the shape ADR-0008 records.
#[cfg(feature = "conformance")]
pub trait ProjectionProbe: ProjectionStore {
    /// Whether this adapter offers any read path on an open batch.
    ///
    /// Declaring `false` is a conformant answer, not a failure: many adapters
    /// buffer their writes and cannot read them back before commit. A store
    /// declaring `false` still has the read-through rule **emitted as a
    /// reported skip carrying its reason** — never omitted — because a rule
    /// absent from the binary is indistinguishable in CI output from a rule
    /// that passed.
    const READS_THROUGH_BATCH: bool;

    /// Writes one probe row into an open batch.
    ///
    /// A future of a `Result`, because the batch may be a live transaction and
    /// then this is a statement: I/O that yields and can fail. A buffering
    /// adapter pushes onto a vector, never awaits and never fails, and its
    /// body reads `async move { batch.push(…); Ok(()) }`. Nothing becomes
    /// durable until the batch reaches [`commit`](ProjectionStore::commit)
    /// either way.
    ///
    /// Spelled `-> impl Future<…>` with **no `+ Send`**, for the reason
    /// [`probe_read`](Self::probe_read) gives.
    ///
    /// # Errors
    ///
    /// Returns the adapter's error if the statement was refused. A refused
    /// statement on a live transaction may poison it; the suite treats any
    /// error here as the batch being unusable and does not commit it.
    fn probe_write(
        &self,
        batch: &mut Self::Batch,
        key: &str,
        value: u64,
    ) -> impl Future<Output = Result<(), Self::Error>>;

    /// Queues deletion of every probe row into an open batch.
    ///
    /// Exists so [`reset`](ProjectionStore::reset) is checkable *without the
    /// suite knowing what a read model is*. `reset` takes the caller's own
    /// deletes, so a suite with no way to express "delete everything" cannot
    /// exercise it at all. Shaped like [`probe_write`](Self::probe_write) for
    /// the same reason.
    ///
    /// # Errors
    ///
    /// Returns the adapter's error if the statement was refused.
    fn probe_delete_all(
        &self,
        batch: &mut Self::Batch,
    ) -> impl Future<Output = Result<(), Self::Error>>;

    /// Reads one probe row from the **committed** read model.
    ///
    /// The one asynchronous member, and not arbitrarily so: the other three
    /// mutate a batch the caller already holds, while this is real I/O against
    /// the store.
    ///
    /// Spelled `-> impl Future<…>` rather than `async fn`, and with **no
    /// `+ Send`**. This trait is not under `#[trait_variant::make]`, so `async
    /// fn` here would fire `async_fn_in_trait` under the gate's `-D warnings`;
    /// writing the desugaring by hand also puts the *absence* of the `Send`
    /// bound at the declaration, where a reader can see it. A `Send` bound
    /// would break `wasm32` and could not be relaxed later without a breaking
    /// change — and it is not needed, because suite code binds the weaker
    /// flavour and takes its per-test wrapper as a parameter.
    ///
    /// # Errors
    ///
    /// Returns the adapter's error if the read model cannot be read.
    fn probe_read(&self, key: &str) -> impl Future<Output = Result<Option<u64>, Self::Error>>;

    /// Reads one probe row through an **open** batch, committed state included.
    ///
    /// Only called when [`READS_THROUGH_BATCH`](Self::READS_THROUGH_BATCH) is
    /// `true`; may be `unimplemented!()` otherwise.
    ///
    /// # This signature is the one a live transaction can meet
    ///
    /// `&mut Self::Batch` because a driver borrows its connection mutably to
    /// issue a statement — `sqlx`'s `Executor for &mut Transaction`,
    /// `rusqlite`'s `&mut Transaction`; a future because the statement is I/O
    /// and yields; a `Result` because a statement on a live transaction can
    /// fail. Until ADR-0062 it was `fn(&self, &Self::Batch, &str) -> Option<u64>`,
    /// and `tests/probe_live_transaction_shape.rs` recorded what that cost: a
    /// store whose batch *is* a transaction could satisfy it only by declaring
    /// `READS_THROUGH_BATCH = false`, a false statement about itself, so the
    /// suite could not tell a live-transaction adapter from a buffering one.
    /// That file now runs the honest body instead.
    ///
    /// A buffering adapter that can read its own pending set — a map layered
    /// over committed state — answers from the map without awaiting. One that
    /// cannot declares `false` and leaves this `unimplemented!()`, which is a
    /// conformant answer and the reported-skip path.
    ///
    /// # Errors
    ///
    /// Returns the adapter's error if the read could not be issued.
    fn probe_read_through(
        &self,
        batch: &mut Self::Batch,
        key: &str,
    ) -> impl Future<Output = Result<Option<u64>, Self::Error>>;
}

#[cfg(test)]
mod tests {
    // Local override of the workspace `unwrap_used = "deny"`, as the house style
    // permits for test modules.
    #![allow(clippy::unwrap_used)]

    use alloc::rc::Rc;

    use alloc::string::{String, ToString};

    use super::{Authority, Checkpoint, CommitError, ProjectionId, ProjectionStore, ResetError};
    use super::{InvalidProjectionId, MAX_PROJECTION_ID_LEN};
    use crate::event::SequencePosition;
    use crate::identity::StoreId;

    /// The witness store's error. Real, so `Self::Error`'s bound is discharged
    /// by something other than `!`.
    #[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
    #[error("the witness store failed")]
    struct WitnessError;

    /// An owned batch that is deliberately **not** `Send`.
    ///
    /// The `Rc` is the assertion. `trait_variant` copies associated-type bounds
    /// verbatim into the bare flavour, so writing `type Batch: Send;` on the
    /// trait would make this struct an illegal `Batch` — and would break the
    /// `wasm32` target the bare flavour exists for (PS-36).
    #[derive(Debug, Default)]
    struct WitnessBatch {
        writes: usize,
        not_send: Rc<()>,
    }

    /// A zero-sized witness that the port is *implementable*.
    ///
    /// Not a store worth shipping and not a step towards one:
    /// `MemoryProjectionStore` is a separate deliverable, behind the `memory`
    /// feature, with a doctest and real state. This is the smallest thing that
    /// makes AC-001, AC-004 and AC-005 tests rather than assertions about text.
    #[derive(Debug, Default)]
    struct Witness;

    impl ProjectionStore for Witness {
        type Error = WitnessError;

        type Batch = WitnessBatch;

        async fn begin(&self) -> Result<Self::Batch, Self::Error> {
            Ok(WitnessBatch::default())
        }

        async fn checkpoint(&self, _id: &ProjectionId) -> Result<Checkpoint, Self::Error> {
            Ok(Checkpoint::NeverRun)
        }

        async fn commit(
            &self,
            batch: Self::Batch,
            _id: &ProjectionId,
            _position: SequencePosition,
            _authority: Authority,
        ) -> Result<(), CommitError<Self::Error>> {
            drop(batch);
            Ok(())
        }

        async fn reset(
            &self,
            batch: Self::Batch,
            _id: &ProjectionId,
        ) -> Result<(), ResetError<Self::Error>> {
            drop(batch);
            Ok(())
        }

        async fn rollback(&self, batch: Self::Batch) -> Result<(), Self::Error> {
            drop(batch);
            Ok(())
        }
    }

    /// AC-001. All six items, implemented and driven.
    #[tokio::test]
    async fn the_port_is_implementable_with_an_owned_batch() {
        let store = Witness;
        let id = ProjectionId::from_static("witness");

        assert_eq!(store.checkpoint(&id).await.unwrap(), Checkpoint::NeverRun);

        let mut batch = store.begin().await.unwrap();
        batch.writes += 1;
        store
            .commit(batch, &id, SequencePosition::FIRST, Authority::Live)
            .await
            .unwrap();

        store
            .reset(store.begin().await.unwrap(), &id)
            .await
            .unwrap();
        store.rollback(store.begin().await.unwrap()).await.unwrap();
    }

    /// AC-002. No `_` arm: a fourth state fails this test rather than widening
    /// it silently.
    #[test]
    fn checkpoint_names_every_state_and_no_others() {
        fn considered_through(checkpoint: Checkpoint) -> Option<SequencePosition> {
            match checkpoint {
                Checkpoint::NeverRun => None,
                Checkpoint::Live { through } | Checkpoint::Rebuilding { through } => Some(through),
            }
        }

        fn is_authoritative(checkpoint: Checkpoint) -> bool {
            match checkpoint {
                Checkpoint::Live { .. } => true,
                Checkpoint::NeverRun | Checkpoint::Rebuilding { .. } => false,
            }
        }

        assert_eq!(considered_through(Checkpoint::NeverRun), None);
        assert_eq!(
            considered_through(Checkpoint::Live {
                through: SequencePosition::FIRST
            }),
            Some(SequencePosition::FIRST)
        );
        assert_eq!(
            considered_through(Checkpoint::Rebuilding {
                through: SequencePosition::FIRST
            }),
            Some(SequencePosition::FIRST)
        );

        // The pair `(None, true)` the tuple could spell has no counterpart
        // here: every variant that is authoritative also carries a position.
        assert!(!is_authoritative(Checkpoint::NeverRun));
        assert!(is_authoritative(Checkpoint::Live {
            through: SequencePosition::FIRST
        }));
        assert!(!is_authoritative(Checkpoint::Rebuilding {
            through: SequencePosition::FIRST
        }));
    }

    /// AC-003. Two enums, each exhaustively matchable inside this crate over
    /// exactly the arms its own operation can produce.
    #[test]
    fn commit_and_reset_errors_stay_separate() {
        fn commit_arm(error: &CommitError<WitnessError>) -> &'static str {
            match error {
                CommitError::ForeignBatch => "foreign",
                CommitError::CheckpointRegression { .. } => "regression",
                CommitError::Store(_) => "store",
            }
        }

        fn reset_arm(error: &ResetError<WitnessError>) -> &'static str {
            match error {
                ResetError::ForeignBatch => "foreign",
                ResetError::Refused => "refused",
                ResetError::Store(_) => "store",
            }
        }

        assert_eq!(commit_arm(&CommitError::ForeignBatch), "foreign");
        assert_eq!(
            commit_arm(&CommitError::CheckpointRegression {
                current: SequencePosition::FIRST,
                attempted: SequencePosition::FIRST,
            }),
            "regression"
        );
        assert_eq!(reset_arm(&ResetError::ForeignBatch), "foreign");
        assert_eq!(reset_arm(&ResetError::Refused), "refused");

        // The payload is the adapter's own error, recovered by value — not a
        // `String`, and not a `Box<dyn Error>` that has lost its type.
        let recovered = match CommitError::Store(WitnessError) {
            CommitError::Store(error) => error,
            other => panic!("expected Store, got {other:?}"),
        };
        assert_eq!(recovered, WitnessError);
    }

    /// ADR-0062 inverted AC-004. `begin` is a future of a `Result`, and a
    /// buffering store's future is ready at its first poll: no executor is
    /// needed to drive it, which is the compiled form of "costs no round trip".
    #[test]
    fn begin_is_async_and_fallible_and_a_buffer_resolves_at_first_poll() {
        use core::task::{Context, Poll, Waker};

        let store = Witness;
        let mut pending = core::pin::pin!(store.begin());
        let batch = match pending
            .as_mut()
            .poll(&mut Context::from_waker(Waker::noop()))
        {
            Poll::Ready(Ok(batch)) => batch,
            Poll::Ready(Err(error)) => panic!("a buffer cannot fail to open: {error}"),
            Poll::Pending => panic!("a buffer's `begin` yielded, which is a round trip"),
        };
        assert_eq!(batch.writes, 0);
        assert_eq!(Rc::strong_count(&batch.not_send), 1);
    }

    /// AC-005. The bare flavour accepts a `!Send` batch, which is what
    /// `type Batch: Send;` would forbid.
    #[test]
    fn a_non_send_batch_still_implements_the_bare_flavour() {
        fn accepts_the_bare_flavour<P: ProjectionStore>() {}
        accepts_the_bare_flavour::<Witness>();
    }

    /// VT-35. Every refusal VT-14 makes for the sibling identifiers, made here
    /// too, each with its own variant.
    ///
    /// Rejects a `new` that checks only ASCII C0 (`char::is_ascii_control`) and
    /// so misses U+0085; one that skips the bidirectional arm; one that
    /// truncates to the bound instead of refusing; and one whose `TooLong`
    /// reports the bound rather than the length. The NUL case is the one
    /// Postgres and Neon used to report at `commit`, far from the constructor
    /// that accepted it.
    #[test]
    fn projection_id_refuses_what_vt_14_refuses() {
        assert_eq!(ProjectionId::new(""), Err(InvalidProjectionId::Empty));
        assert_eq!(
            ProjectionId::new("a\nb"),
            Err(InvalidProjectionId::ControlCharacter)
        );
        assert_eq!(
            ProjectionId::new("a\0b"),
            Err(InvalidProjectionId::ControlCharacter)
        );
        assert_eq!(
            ProjectionId::new("a\u{85}b"),
            Err(InvalidProjectionId::ControlCharacter)
        );
        assert_eq!(
            ProjectionId::new("a\u{202E}b"),
            Err(InvalidProjectionId::BidirectionalControl)
        );
        assert_eq!(
            ProjectionId::new("x".repeat(MAX_PROJECTION_ID_LEN + 1)),
            Err(InvalidProjectionId::TooLong {
                len: MAX_PROJECTION_ID_LEN + 1
            })
        );

        let longest = ProjectionId::new("x".repeat(MAX_PROJECTION_ID_LEN)).unwrap();
        assert_eq!(longest.as_str().len(), MAX_PROJECTION_ID_LEN);
    }

    /// VT-35 keeps VT-14's refusal of a blanket `Cf` ban: the joiners scripts
    /// need are accepted, and so are the four neighbours of the two closed
    /// bidirectional runs.
    ///
    /// Rejects a blanket `Cf` ban, and a bidirectional range one codepoint too
    /// wide either way.
    #[test]
    fn projection_id_accepts_the_format_characters_scripts_need() {
        for value in [
            "a\u{200C}b",
            "a\u{200D}b",
            "a\u{200B}b",
            "a\u{2029}b",
            "a\u{202F}b",
            "a\u{2065}b",
            "a\u{206A}b",
        ] {
            assert_eq!(ProjectionId::new(value).unwrap().as_str(), value);
        }
    }

    /// The two reserved prefixes are refused, bare and followed by more.
    ///
    /// Rejects a `contains` test, and a test that refuses only the bare prefix
    /// or only a longer value.
    #[test]
    fn projection_id_refuses_the_reserved_prefixes_by_exact_bytes() {
        for value in ["sync/", "sync/peer"] {
            assert_eq!(
                ProjectionId::new(value),
                Err(InvalidProjectionId::Reserved { prefix: "sync/" })
            );
        }
        for value in ["happenstance/", "happenstance/x"] {
            assert_eq!(
                ProjectionId::new(value),
                Err(InvalidProjectionId::Reserved {
                    prefix: "happenstance/"
                })
            );
        }
    }

    /// The reservation is an exact byte prefix: nothing near it is refused,
    /// and nothing accepted is altered.
    ///
    /// Rejects `starts_with("sync")` without the slash; case folding before the
    /// compare; a trim before the check; and a `contains`.
    #[test]
    fn projection_id_accepts_the_neighbours_of_the_reserved_prefixes() {
        for value in [
            "sync",
            "syncope",
            "sync_jobs",
            "sync-x",
            "Sync/x",
            "SYNC/x",
            " sync/x",
            "x/sync/y",
            "happenstance",
            "happenstance-x",
            "Happenstance/x",
        ] {
            assert_eq!(ProjectionId::new(value).unwrap().as_str(), value);
        }
    }

    /// VT-35's MUST that the `const` door is exactly as strong as the runtime
    /// one, asserted on the accepting side. The refusing side is the
    /// `should_panic` tests below.
    ///
    /// Rejects a `const` path with a second, simplified copy of the rules
    /// (RS-11-1).
    #[test]
    fn projection_id_from_static_and_new_agree() {
        const LONGEST: &str = "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx\
                               xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx\
                               xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx\
                               xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx";
        assert_eq!(LONGEST.len(), MAX_PROJECTION_ID_LEN);

        for value in [
            "van_stock",
            LONGEST,
            "a\u{200C}b",
            "a\u{200D}b",
            "a\u{200B}b",
            "a\u{2029}b",
            "a\u{202F}b",
            "a\u{2065}b",
            "a\u{206A}b",
            "sync",
            "syncope",
            "sync_jobs",
            "sync-x",
            "Sync/x",
            "SYNC/x",
            " sync/x",
            "x/sync/y",
            "happenstance",
            "happenstance-x",
            "Happenstance/x",
        ] {
            let runtime = ProjectionId::new(value).unwrap();
            assert_eq!(ProjectionId::from_static(value).as_str(), runtime.as_str());
        }
    }

    /// `from_static` refuses a reserved prefix, which is the refusal VT-35 adds
    /// on top of VT-14 and so the one a copied `const` path would lack.
    #[test]
    #[should_panic(expected = "reserved prefix")]
    fn projection_id_from_static_rejects_a_reserved_prefix() {
        let _ = ProjectionId::from_static("sync/x");
    }

    #[test]
    #[should_panic(expected = "bidirectional formatting controls")]
    fn projection_id_from_static_rejects_a_bidirectional_control() {
        let _ = ProjectionId::from_static("a\u{202E}b");
    }

    #[test]
    #[should_panic(expected = "control characters")]
    fn projection_id_from_static_rejects_a_c1_control() {
        let _ = ProjectionId::from_static("a\u{85}b");
    }

    /// `from_static` is a `const fn`, which is what makes a free `const` the
    /// compile-time-validated spelling the examples teach.
    ///
    /// Rejects dropping `const` from `from_static`: nothing else notices
    /// (RS-11-3).
    #[test]
    fn projection_id_is_const_constructible() {
        const VAN_STOCK: ProjectionId = ProjectionId::from_static("van_stock");

        assert_eq!(VAN_STOCK.as_str(), "van_stock");
        assert_eq!(VAN_STOCK, ProjectionId::new("van_stock").unwrap());
    }

    /// The watermark is `sync/` and the peer's 32 lowercase hex digits, and it
    /// passes every VT-14 rule: `new` refuses it **only** for its prefix, and
    /// the reservation is checked last.
    ///
    /// Rejects a watermark rendered with `Debug` (`StoreId([..])`), with UUID
    /// dashes, or in uppercase hex (a second spelling of one peer); and one
    /// built by a path that `new` would also accept, which would make the
    /// reservation advisory.
    #[test]
    fn sync_watermark_is_reserved_and_otherwise_valid() {
        let zero = ProjectionId::sync_watermark(StoreId::from_bytes([0x00; 16]));
        let ones = ProjectionId::sync_watermark(StoreId::from_bytes([0xff; 16]));
        let mixed = ProjectionId::sync_watermark(StoreId::from_bytes([
            0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab,
            0xcd, 0xef,
        ]));

        assert_eq!(zero.as_str(), "sync/00000000000000000000000000000000");
        assert_eq!(ones.as_str(), "sync/ffffffffffffffffffffffffffffffff");
        assert_eq!(mixed.as_str(), "sync/0123456789abcdef0123456789abcdef");

        for watermark in [&zero, &ones, &mixed] {
            assert_eq!(watermark.as_str().len(), 37);
            assert_eq!(
                ProjectionId::new(watermark.as_str()),
                Err(InvalidProjectionId::Reserved { prefix: "sync/" })
            );
        }
    }

    /// Two peers never share a watermark.
    ///
    /// Rejects a rendering of a truncated `StoreId`, such as its first eight
    /// bytes.
    #[test]
    fn sync_watermarks_of_distinct_peers_are_distinct() {
        let ids = [
            ProjectionId::sync_watermark(StoreId::from_bytes([0x00; 16])),
            ProjectionId::sync_watermark(StoreId::from_bytes([0xff; 16])),
            ProjectionId::sync_watermark(StoreId::from_bytes([
                0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab,
                0xcd, 0xef,
            ])),
        ];
        for (i, left) in ids.iter().enumerate() {
            for right in ids.iter().skip(i + 1) {
                assert_ne!(left, right);
            }
        }

        let mut last = [0x5a; 16];
        let before = ProjectionId::sync_watermark(StoreId::from_bytes(last));
        last[15] = 0x5b;
        let after = ProjectionId::sync_watermark(StoreId::from_bytes(last));
        assert_ne!(before, after);
    }

    /// `Borrow<str>` hashes, compares and orders as the owner does, so both map
    /// kinds can be probed by `&str`.
    ///
    /// Rejects a `Borrow<str>` whose `Hash`, `Eq` or `Ord` disagrees with
    /// `str`'s (RS-12-1). Gated on `std` because `HashMap` is `std`'s.
    #[cfg(feature = "std")]
    #[test]
    fn a_map_keyed_by_projection_id_is_probed_by_str() {
        let mut hashed = std::collections::HashMap::new();
        hashed.insert(ProjectionId::from_static("van_stock"), 1);
        assert_eq!(hashed.get("van_stock"), Some(&1));
        assert!(hashed.contains_key(&ProjectionId::new("van_stock").unwrap()));

        let mut ordered = alloc::collections::BTreeMap::new();
        ordered.insert(ProjectionId::from_static("van_stock"), 2);
        ordered.insert(ProjectionId::from_static("order_status"), 3);
        assert_eq!(ordered.get("van_stock"), Some(&2));
        assert_eq!(ordered.get("order_status"), Some(&3));
        assert!(ordered.contains_key(&ProjectionId::new("van_stock").unwrap()));
    }

    /// Every conversion from a string validates, with the verdict `new` gives.
    ///
    /// Rejects a conversion that forwards to an unchecked path.
    #[test]
    fn projection_id_conversions_all_validate() {
        for (value, refusal) in [
            ("", InvalidProjectionId::Empty),
            ("sync/x", InvalidProjectionId::Reserved { prefix: "sync/" }),
        ] {
            assert_eq!(ProjectionId::try_from(value), Err(refusal.clone()));
            assert_eq!(
                ProjectionId::try_from(String::from(value)),
                Err(refusal.clone())
            );
            assert_eq!(value.parse::<ProjectionId>(), Err(refusal));
        }

        let expected = ProjectionId::from_static("van_stock");
        assert_eq!(ProjectionId::try_from("van_stock"), Ok(expected.clone()));
        assert_eq!(
            ProjectionId::try_from(String::from("van_stock")),
            Ok(expected.clone())
        );
        assert_eq!("van_stock".parse::<ProjectionId>(), Ok(expected));
    }

    /// The messages carry the bound, the length and the prefix.
    ///
    /// Rejects a message that drops any of them.
    #[test]
    fn invalid_projection_id_messages_carry_their_context() {
        let too_long = InvalidProjectionId::TooLong { len: 256 }.to_string();
        assert!(too_long.contains("255"), "{too_long}");
        assert!(too_long.contains("256"), "{too_long}");

        let reserved = InvalidProjectionId::Reserved { prefix: "sync/" }.to_string();
        assert!(reserved.contains("`sync/`"), "{reserved}");
    }
}

/// The trait-level doc block, read as text because it is published on **two**
/// pages.
///
/// The same mechanism as `store.rs`'s module of this name, and the same defect:
/// `#[trait_variant::make(SendProjectionStore: Send)]` rebuilds the derived trait
/// with `..tr.clone()`, so every `///` line above the derivation is rendered
/// verbatim on `SendProjectionStore`'s page too. `SendProjectionStore` has no doc
/// comment of its own, so the copying is load-bearing — `missing_docs` is what
/// would notice it stopping — and the sentences therefore have to be true on
/// whichever page a reader opened.
///
/// The extractor is duplicated from `store.rs` rather than shared: sharing it
/// would put a test-only module in the crate root, which is the file
/// `happenstance`'s `contract_surface.rs` derives every gate from.
#[cfg(test)]
mod derived_flavour_doc {
    #![allow(clippy::unwrap_used, reason = "test code, per the house style")]

    use alloc::string::String;
    use alloc::vec::Vec;

    /// This file's own source. The doc block is the deliverable, so it is read
    /// rather than trusted.
    const SOURCE: &str = include_str!("projection.rs");

    /// The derivation whose expansion copies the block above it onto a second page.
    const DERIVATION: &str = "#[trait_variant::make(SendProjectionStore: Send)]";

    /// The two flavours, in the link form a reader can click from either page.
    const FLAVOURS: [&str; 2] = ["[`ProjectionStore`]", "[`SendProjectionStore`]"];

    /// The `///` lines the derivation copies, marker removed and fences dropped.
    fn copied_prose() -> Vec<&'static str> {
        let lines: Vec<&str> = SOURCE.lines().collect();
        let make = lines
            .iter()
            .position(|line| line.trim_end() == DERIVATION)
            .expect("the derivation that copies this block onto the second page");
        let start = lines[..make]
            .iter()
            .rposition(|line| {
                !(line.starts_with("///") || line.starts_with("//") || line.starts_with("#["))
            })
            .map_or(0, |index| index + 1);
        let mut fenced = false;
        lines[start..make]
            .iter()
            .filter_map(|line| {
                let text = line.strip_prefix("///")?;
                let text = text.strip_prefix(' ').unwrap_or(text);
                if text.trim_start().starts_with("```") {
                    fenced = !fenced;
                    return None;
                }
                if fenced { None } else { Some(text) }
            })
            .collect()
    }

    /// The copied prose in blank-line-separated paragraphs.
    fn paragraphs() -> Vec<String> {
        copied_prose()
            .split(|line| line.is_empty())
            .filter(|block| !block.is_empty())
            .map(|block| block.join(" "))
            .collect()
    }

    /// A paragraph that distinguishes the flavours names them; it does not point.
    #[test]
    fn no_paragraph_tells_the_flavours_apart_by_deixis() {
        /// Pointers that resolve against the page rather than against a name.
        const DEIXIS: [&str; 6] = [
            "this trait",
            "this is",
            "this flavour",
            "this one",
            "the one to use",
            "instead",
        ];
        for paragraph in paragraphs() {
            let lower = paragraph.to_lowercase();
            if !lower.contains("send") {
                continue;
            }
            for pointer in DEIXIS {
                assert!(
                    !lower.contains(pointer),
                    "{DERIVATION} copies this paragraph verbatim onto \
                     `SendProjectionStore`, where {pointer:?} points at the wrong \
                     trait. Name the flavour: {paragraph}"
                );
            }
        }
    }

    /// Both flavours are named, in link form, in the prose a reader lands on.
    #[test]
    fn the_block_names_both_flavours_in_link_form() {
        let prose = copied_prose().join("\n");
        for flavour in FLAVOURS {
            assert!(
                prose.contains(flavour),
                "the block is rendered on both pages, so it must name {flavour} \
                 rather than leave a reader to infer which trait they are on. \
                 Prose as read:\n{prose}"
            );
        }
    }
}
