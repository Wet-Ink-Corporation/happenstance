//! The typed layer's workload: one domain, small enough to read and shaped like
//! the ones the examples use.
//!
//! # Why a domain at all, when `crate::corpus` already builds events
//!
//! Because the typed layer measures something the store arms cannot reach.
//! `crate::corpus` produces `happenstance_core::Event` — an opaque `Bytes`
//! payload the contract never looks inside. `happenstance::commit` produces a
//! *decision*: it reads a decision model's query, decodes every matched event
//! into an application enum, folds them, calls the caller back, encodes what
//! comes out and appends it under a condition. One `commit` is a full replay
//! plus *N* decodes, and no store benchmark prices any of that.
//!
//! # Why it is deliberately the examples' shape
//!
//! `examples/transfers-on-sqlite/src/main.rs` and
//! `examples/course-subscriptions/src/main.rs` are the two programs a reader
//! meets first, and both are a `DomainEvent` enum over an identity newtype that
//! caches its own `Tag`. Measuring a different shape would produce figures a
//! reader could not map onto the code they are looking at.
//!
//! The one addition is [`Recorded::payload`], a `Vec<u8>` whose length is the
//! caller's. Without it every codec figure would be taken at one payload size,
//! and the three codecs' curves cross — `serde_json`'s base64 expansion is a
//! constant factor on payload and a fixed cost on structure, so which encoder
//! wins depends entirely on the ratio between them.
//!
//! # Where the encoding cost actually lands
//!
//! `references/evaluation/review-pre-publication-2026-09-03.md:2756` measured
//! encoding **one** 64-tag `SequencedEvent` to postcard at **140 heap
//! operations to produce 587 bytes, of which 130 (93%) are transient clones
//! that produce no output**. At the crate's own two conformance floors — 128
//! events, 64 tags — a sync runner pushing one batch does 16,896 transient
//! allocations that produce no bytes. That is why `benches/typed_codec.rs`
//! sweeps tag count as well as payload size, and why `src/bin/allocations.rs`
//! reports the encode path in heap operations rather than in nanoseconds.

use std::sync::LazyLock;

use happenstance::bytes::Bytes;
use happenstance::{
    Codec, CodecError, DecisionModel, DomainEvent, EventType, InvalidTag, Projection, ProjectionId,
    ProjectionStore, Tag, Tags,
};
use happenstance_core::MemoryProjectionBatch;
use happenstance_sqlite::projection_store::SqliteProjectionStore;
use rusqlite::types::Value;
use serde::{Deserialize, Serialize};

/// An account, and the tag it is scoped by.
///
/// The tag is cached beside the string rather than rebuilt per call, exactly as
/// `AccountId` does in `examples/transfers-on-sqlite`. That is not an
/// optimisation for the benchmark's sake: it is what the examples teach, so
/// measuring anything else would price a shape nobody writes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountId {
    id: String,
    #[serde(skip)]
    tag: Option<Tag>,
}

impl AccountId {
    /// Validates the identifier and caches its tag.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidTag`] if `id` cannot be part of a `key:value` tag.
    pub fn new(id: impl Into<String>) -> Result<Self, InvalidTag> {
        let id = id.into();
        let tag = Tag::key_value("account", &id)?;
        Ok(Self { id, tag: Some(tag) })
    }

    /// The identifier.
    pub fn as_str(&self) -> &str {
        &self.id
    }

    /// The cached tag.
    ///
    /// # Panics
    ///
    /// Panics on a value that came back from a decode, where `tag` is `None`
    /// because it is `#[serde(skip)]`. Nothing in this crate takes the tag off
    /// a decoded value — the runner and the command loop both tag from the
    /// *encoding* side — and a panic naming that is better than a silent
    /// re-validation on a measured path.
    pub fn tag(&self) -> &Tag {
        self.tag
            .as_ref()
            .expect("tags are taken from encoding-side values, never decoded ones")
    }
}

/// The benchmark domain: one event, carrying a payload of the caller's size.
///
/// One variant rather than three. The examples have three because a ledger
/// needs three; a benchmark needs the *encode and decode paths*, and a second
/// variant would add a discriminant to every measurement without adding a
/// question.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Recorded {
    /// Something happened to an account, with an opaque body.
    Happened {
        /// Whose account.
        account: AccountId,
        /// A monotonic counter, so a fold has something to accumulate.
        amount: u32,
        /// The body, at whatever size the caller asked for.
        payload: Vec<u8>,
    },
}

impl Recorded {
    /// One event for `account`, carrying `payload_bytes` of body.
    ///
    /// # Panics
    ///
    /// Panics if `account` cannot be a tag, which would mean the caller built a
    /// malformed corpus.
    pub fn new(account: &str, amount: u32, payload_bytes: usize) -> Self {
        Self::Happened {
            account: AccountId::new(account).expect("the benchmark's own account ids are valid"),
            amount,
            payload: vec![0x5A; payload_bytes],
        }
    }
}

impl DomainEvent for Recorded {
    const EVENT_TYPES: &'static [EventType] = &[EventType::from_static("BenchmarkHappened")];

    fn event_type(&self) -> EventType {
        Self::EVENT_TYPES[0].clone()
    }

    fn tags(&self) -> Tags {
        match self {
            Self::Happened { account, .. } => [account.tag().clone()].into_iter().collect(),
        }
    }

    fn encode<C: Codec>(&self, codec: &C) -> Result<Bytes, CodecError> {
        codec.encode(self)
    }

    fn decode<C: Codec>(
        codec: &C,
        _event_type: &EventType,
        data: &Bytes,
    ) -> Result<Self, CodecError> {
        codec.decode(data)
    }
}

/// One account's running total: the decision model `commit` folds.
#[derive(Debug, Clone)]
pub struct Total {
    scope: Tags,
    /// The accumulated amount, public so a bench can assert the fold ran.
    pub held: u64,
}

impl Total {
    /// Scopes a model to one account.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidTag`] if `account` cannot be part of a tag.
    pub fn new(account: &str) -> Result<Self, InvalidTag> {
        Ok(Self {
            scope: Tags::from_pairs([("account", account)])?,
            held: 0,
        })
    }
}

impl DecisionModel for Total {
    type Event = Recorded;

    fn scope(&self) -> &Tags {
        &self.scope
    }

    fn apply(&mut self, event: Self::Event) {
        match event {
            Recorded::Happened { amount, .. } => {
                self.held = self.held.saturating_add(u64::from(amount));
            }
        }
    }
}

/// The projection id every arm here uses.
static BENCH_PROJECTION: LazyLock<ProjectionId> =
    LazyLock::new(|| ProjectionId::new("benchmark-totals"));

/// A read model over one account, into `happenstance-sqlite`'s deferred batch.
///
/// The `apply` body is one upsert, copied in shape from
/// `examples/transfers-on-sqlite/src/main.rs:559` — a projection that did less
/// would price the runner's loop without the write it exists to perform, and
/// one that did more would be measuring an application's SQL rather than the
/// port's.
#[derive(Debug)]
pub struct SqliteTotals {
    scope: Tags,
}

impl SqliteTotals {
    /// Scopes a projection to one account.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidTag`] if `account` cannot be part of a tag.
    pub fn new(account: &str) -> Result<Self, InvalidTag> {
        Ok(Self {
            scope: Tags::from_pairs([("account", account)])?,
        })
    }

    /// The read-model table this projection writes, and nothing else.
    ///
    /// Applied by the benchmark before a run, because a projection store's
    /// migration covers the checkpoint table and not an application's own read
    /// model — which is exactly the split
    /// `examples/transfers-on-sqlite/src/main.rs:129` demonstrates.
    pub const READ_MODEL_DDL: &'static str = "\
CREATE TABLE IF NOT EXISTS account_total (
    account TEXT PRIMARY KEY,
    total   INTEGER NOT NULL
) STRICT;";
}

impl Projection for SqliteTotals {
    type Event = Recorded;
    type Store = SqliteProjectionStore;

    fn id(&self) -> &ProjectionId {
        &BENCH_PROJECTION
    }

    fn scope(&self) -> &Tags {
        &self.scope
    }

    fn apply(
        &mut self,
        event: Self::Event,
        batch: &mut <Self::Store as ProjectionStore>::Batch,
    ) -> Result<(), <Self::Store as ProjectionStore>::Error> {
        let Recorded::Happened {
            account, amount, ..
        } = event;
        batch.push(
            "INSERT INTO account_total (account, total) VALUES (?1, ?2) \
             ON CONFLICT(account) DO UPDATE SET total = account_total.total + excluded.total",
            [
                Value::Text(account.as_str().to_owned()),
                Value::Integer(i64::from(amount)),
            ],
        );
        Ok(())
    }
}

/// The same read model over `happenstance-core`'s in-memory projection store.
///
/// The second arm of the batch-shape axis, and the reason it is worth having:
/// `SqliteProjectionStore`'s batch is a **buffer of deferred SQL** applied at
/// commit, while `MemoryProjectionStore`'s is a map written into directly. A
/// runner figure taken against only one of them would be a figure about that
/// batch shape rather than about the runner.
#[derive(Debug)]
pub struct MemoryTotals {
    scope: Tags,
}

impl MemoryTotals {
    /// Scopes a projection to one account.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidTag`] if `account` cannot be part of a tag.
    pub fn new(account: &str) -> Result<Self, InvalidTag> {
        Ok(Self {
            scope: Tags::from_pairs([("account", account)])?,
        })
    }
}

impl Projection for MemoryTotals {
    type Event = Recorded;
    type Store = happenstance_core::MemoryProjectionStore;

    fn id(&self) -> &ProjectionId {
        &BENCH_PROJECTION
    }

    fn scope(&self) -> &Tags {
        &self.scope
    }

    fn apply(
        &mut self,
        event: Self::Event,
        batch: &mut MemoryProjectionBatch,
    ) -> Result<(), <Self::Store as ProjectionStore>::Error> {
        let Recorded::Happened {
            account, amount, ..
        } = event;
        batch.write(account.as_str().to_owned(), u64::from(amount));
        Ok(())
    }
}
