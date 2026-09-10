//! Postgres-backed [`SendEventStore`].
//!
//! # Status
//!
//! Implemented, and run rather than asserted: **107 of 107** gated tests pass
//! against a live PostgreSQL 17.10 — the 95 rules of `event_store_conformance!`,
//! the 5 of `event_store_concurrency_conformance!` at `CONTENDERS = 64`, the
//! model family, and this crate's own six.
//!
//! One of those 105 is a **stated declension rather than a run**, and it is named
//! here rather than left to be discovered in a log: the fixture declines
//! `READ_YOUR_OWN_WRITES`, so `ops_agree_with_the_model` reports a skip carrying
//! the reason. The model family predicts whether a conditional append will be
//! rejected from what a read showed it, and on this store those are deliberately
//! different sets — see *What this store costs a caller* on the crate root. The
//! counts above read 101 and 89 until the `0.2.0` pass. The family was 93 rules
//! and had been since before this crate cleared it, so both numbers were
//! arithmetic on a stale one; it is 95 now, because ES-27 gained the two rules
//! that check a condition item's tags are AND-ed rather than keyed on the first.
//!
//! The concurrency family is the one that matters most: it is the first time an
//! adapter in this portfolio has cleared it against a store whose writers are
//! **not** serialised, which is the whole reason this crate is in the tree.
//!
//! The projection store beside this one is implemented too, and the crate carries
//! no `#![allow(clippy::todo)]` any more — it left with the last stub, which is
//! the contract it was written under.
//!
//! # Cancellation
//!
//! **A dropped `append` future cannot be cancelled by this adapter, and the
//! batch may already have committed.** ES-23 is `[FROZEN]` and obliges every
//! adapter to say which of its two outcomes it has; this is ours, and it is the
//! less comfortable one.
//!
//! The mechanism is `on_runtime`, which is `Handle::spawn(work).await`. Dropping
//! the caller's future drops only the `JoinHandle`. Tokio **detaches** a spawned
//! task rather than cancelling it, so the transaction underneath carries on: the
//! `INSERT` runs, the `COMMIT` runs, and there is nobody left to tell. Nothing
//! in this adapter observes the drop, so nothing can react to it.
//!
//! A caller **MUST NOT** read a dropped `append` future as evidence either way —
//! not that it committed, and not that it did not. The resolution is ES-24's:
//! reissue the identical batch. Event identity makes the reissue idempotent, so
//! the second attempt either lands (the first did not commit) or is refused as a
//! duplicate (it did), and both answers are true ones.
//!
//! This is precisely the shape ES-23's own `Rejects:` names, reached through
//! `spawn` rather than `spawn_blocking`. It is **not** `happenstance-sqlite`'s
//! answer and must not be described in its words: that adapter says there is
//! nothing to drop, because its `append` has no suspension point at all. Ours
//! has one, and what is on the other side of it keeps running.
//!
//! # The schema
//!
//! Authoritative in `migrations/0001_event_log.sql`, compiled in by
//! [`crate::migration::MIGRATION_1`] and applied by [`crate::migration::apply`].
//! It is not restated here: the SQLite adapter keeps its schema as a Rust
//! constant mirrored into a doc comment and needs a test comparing the two
//! against `sqlite_master`, because the copies drift. A file has no second copy
//! to drift from.
//!
//! What is worth reading before the file: `position` carries **no column
//! default** and is not `bigserial`; the sequence is read explicitly at the one
//! `INSERT` that allocates; and each row stamps `pg_current_xact_id()` into an
//! `xid8` column, which is the mechanism every read and `head` filter on.
//!
//! `position` is deliberately **not** `bigserial`. That is the whole subject of
//! this module's ES-10 note below: a `serial` column is `nextval()`, and
//! `nextval()` is where the invariant is lost.
//!
//! # ES-10, and what this adapter pays for it
//!
//! ES-10 says position order is visibility order: once a reader has seen
//! position *P*, nothing at or below *P* may appear later. A naive Postgres
//! adapter breaks it, because `nextval()` allocates outside the transaction — a
//! writer takes 99, a writer that started later takes 100 and commits first, and
//! a reader who conditioned on `after: Some(100)` never sees the 99 that would
//! have violated its boundary. There is no error, no rejected append and no
//! failing test; there is a read model that is correct about a state the
//! business forbids.
//!
//! **This adapter buys the invariant with `xid8` + `pg_snapshot_xmin`, and the
//! choice was measured rather than preferred.** ADR-0024 is the record. Each row
//! stamps `pg_current_xact_id()`; every read and `head` admit only rows below
//! `pg_snapshot_xmin(pg_current_snapshot())`, the frontier beneath which no
//! transaction can still be in flight. The mechanism does not deny that
//! `nextval()` allocates outside the transaction — it stops treating position
//! order as *visibility* order and moves the guard to the read side.
//!
//! What that asks of this adapter's types and its append path:
//!
//! * **Writers do not serialise at all**, which is the property this crate
//!   exists to have, and [`SendEventStore::append`]'s signature is untouched.
//!   What the async boundary did force is the conditional path: under
//!   `READ COMMITTED` two writers racing one boundary both find no conflict and
//!   both win, so a conditional append runs `SERIALIZABLE` — whose SSI is
//!   optimistic and takes no locks, so disjoint boundaries still proceed in
//!   parallel — and retries a `40001`. Holding a write lock instead is the move
//!   this crate exists *not* to make.
//! * **The cost is read-side, and it is structural rather than incremental.**
//!   Every read gains the visibility predicate, and the predicate is composed
//!   into the cursor's `DECLARE` inside one `REPEATABLE READ` transaction and
//!   evaluated once. Not per `FETCH`: the frontier advances between chunks, and
//!   a per-chunk predicate lets rows appear beneath positions the caller has
//!   already been handed — ES-10's own violation arriving through the read path
//!   while the write path was being fixed.
//! * **`head` reports the frontier rather than the maximum position**, so
//!   freshly committed events are invisible until the frontier passes them. That
//!   is a **capability** limit rather than a type error: the adapter compiles,
//!   and then read-your-own-writes does not hold.
//!
//! **The two numbers.** Steady state, measured against this built adapter with
//! the predicate removed as the paired baseline: the median ratio straddles 1.0
//! at every concurrency level with about a 20% spread, so no cost large enough
//! to matter is measurable — and the arms that cost 16x and 30x would be
//! unmissable at that precision. Staleness, which is the real bill: a median of
//! 0.593 ms unloaded and **4,799 ms behind a five-second write transaction held
//! anywhere on the cluster**, against an unguarded arm unaffected by the same
//! hold at 0.595 ms.
//!
//! **The alternative that lost, named once so it is not re-proposed.** A
//! tag-keyed advisory lock is the cheapest rival — 0.935 of baseline at 64
//! writers, nearly free — and it lost on the **invariant** rather than on cost:
//! it buys a per-boundary property where ES-10 states a global one, which makes
//! `AppendCondition` sound and the projection checkpoint unsound. Ranking the
//! arms by throughput would have selected it. The two that serialise every
//! writer, a sequence table and a constant advisory lock, are correct and lost
//! on cost at 0.062 and 0.033.
//!
//! What follows for a caller, and the crate does not soften it: `head` is a
//! frontier, read-your-own-writes does not hold and is not claimed, and the
//! staleness bound is the longest open write transaction on the cluster — which
//! a consumer neither controls nor can necessarily observe. The crate root
//! states that bill where a consumer meets it first.
//!
//! `references/adr/0024-position-visibility-mechanism.md` carries the argument,
//! the losing arms and what each lost on, and the parts this decision inherits
//! rather than owns.

use futures_core::Stream;
use happenstance_core::{
    AppendCondition, AppendError, ConditionViolated, Event, EventId, Query, ReadOptions,
    RecordedAt, SendEventStore, SequencePosition, SequencedEvent, StoreId, StoreLimit,
};
use sqlx::{PgPool, Postgres, Transaction};
use std::future::Future;
use std::hash::{BuildHasher, Hasher, RandomState};
use std::sync::{Arc, OnceLock};
use std::time::Duration;
use tokio::runtime::Handle;

use crate::error::PostgresEventStoreError;
use crate::read_stream::PgReadStream;

/// A Postgres-backed event store.
///
/// Holds a [`PgPool`] rather than a connection, which is the shape difference
/// that makes this crate an instrument: readers and writers do not queue behind
/// one another, so nothing about the storage layer supplies ES-10 for free.
///
/// # What this store does not promise
///
/// It buys ES-10 on the read side, with `xid8` + `pg_snapshot_xmin`, so
/// [`head`](SendEventStore::head) reports a **visibility frontier** rather than
/// the highest position assigned, and **read-your-own-writes does not hold**.
/// Staleness is bounded by the longest open write transaction anywhere on the
/// cluster. Those are capability limits rather than defects; the crate root
/// states them in full, and [`append`](SendEventStore::append) and
/// [`head`](SendEventStore::head) each state the half a caller meets there.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct PostgresEventStore {
    pool: PgPool,
    /// This store's identity, read from `store_meta` on first use.
    ///
    /// Lazy because [`new`](Self::new) is neither `async` nor fallible and must
    /// stay that way — it is the constructor an application calls while wiring a
    /// pool, long before it knows whether the schema exists. `OnceLock` rather
    /// than a `tokio` cell so the crate takes no new dependency for it; the
    /// benign race is two handles reading the same committed row and one
    /// `set` losing, which costs a round trip and changes no value.
    store_id: Arc<OnceLock<StoreId>>,
    /// The runtime every operation hops onto, captured at construction.
    ///
    /// # Why a store needs this at all
    ///
    /// `sqlx` requires a tokio runtime in thread-local scope, and this store is
    /// called from threads that have none. That is not hypothetical: the
    /// conformance suite's concurrency family runs each contender on a raw OS
    /// thread driving `block_on`, deliberately -- the testkit has no runtime
    /// dependency and must not acquire one, because CF-20 and CF-23 exist so the
    /// suite runs on `wasm32` and under a caller-supplied harness. Without a
    /// bridge, every rule in that family panics inside `sqlx`'s `missing_rt`,
    /// which is exactly what the first run of it did.
    ///
    /// Captured here rather than looked up per call because a store is
    /// *constructed* inside the harness's runtime and *used* outside it. The
    /// same shape `happenstance-sqlite` carries, for the same reason one layer
    /// down.
    ///
    /// # Why `spawn` and not `Handle::enter`
    ///
    /// `enter` is smaller and does not work. Its `EnterGuard` is `!Send`, so
    /// holding one across an `await` makes the future `!Send` and
    /// `SendEventStore` stops being implementable -- ADR-0001's constraint
    /// arriving from the other direction. Spawning moves the work onto the
    /// runtime that owns the reactor and leaves the caller's bare `block_on`
    /// waiting on a `JoinHandle`, which is a plain future and needs nothing.
    runtime: Option<Handle>,
    /// Whether reads carry the visibility frontier.
    ///
    /// Always `Visibility::Frontier` for a store any consumer can build.
    /// `Visibility::Naive` exists only behind the off-by-default `naive-arm`
    /// feature, and only so that a conformance rule can be shown to REJECT the
    /// implementation this adapter deliberately is not. See
    /// [`new_naive`](Self::new_naive).
    visibility: Visibility,
}

/// Whether a store admits rows its transaction cannot yet vouch for.
///
/// Two arms of one adapter rather than two adapters, which is the point: the
/// naive arm differs from the shipped one in exactly the predicate under test
/// and in nothing else, so a rule that fails against it is failing on the
/// mechanism rather than on an unrelated difference between two codebases.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Visibility {
    /// Reads and `head` carry `xact_id < pg_snapshot_xmin(pg_current_snapshot())`.
    Frontier,
    /// They do not. `head` is `max(position)` and a read admits every committed
    /// row, which is what `nextval()` allocating outside the transaction makes
    /// wrong.
    #[cfg(feature = "naive-arm")]
    Naive,
}

impl Visibility {
    /// The `WHERE` fragment this arm adds, already `AND`-able.
    pub(crate) fn predicate(self) -> &'static str {
        match self {
            Self::Frontier => "xact_id < pg_snapshot_xmin(pg_current_snapshot())",
            // `TRUE` rather than an empty string so every caller can splice it
            // in without a special case, and so the two arms differ by a value
            // rather than by a branch at each site.
            #[cfg(feature = "naive-arm")]
            Self::Naive => "TRUE",
        }
    }
}

impl PostgresEventStore {
    /// The largest `data` payload this store accepts, in **bytes of
    /// [`Event::data`]** — not of an encoded row, and not of `data` and
    /// `metadata` together.
    ///
    /// A **fact about this adapter**, not a trade. Postgres's own ceilings are
    /// far above it and are not the binding constraint: a `bytea` field tops out
    /// near 1 GB, and the wire protocol's `Bind` message near the same. Either
    /// number is unusable as a stated ceiling, because
    /// `append_reports_exceeded_store_limits` allocates the ceiling **plus one
    /// byte** and does it twice per run — a suite that pins a gigabyte per rule
    /// is a suite nobody runs.
    ///
    /// So this is an adapter policy, enforced in `append` before anything
    /// reaches the wire, and it is deliberately the same number
    /// `happenstance-sqlite` states: sixteen times VT-21's 65,536-byte floor.
    /// Two adapters agreeing on a policy number is worth more than two adapters
    /// each deriving a different one from a limit neither is anywhere near.
    pub const MAX_EVENT_DATA_LEN: usize = 1_048_576;

    /// The largest number of tags on one event this store accepts.
    ///
    /// Twice VT-22's floor of 64. Tags live in one `text[]` column rather than
    /// in a join table, so unlike the SQLite adapter a tag costs no extra row
    /// and no extra statement inside the write transaction — the cost here is
    /// the GIN index's, which builds one entry per element. Like the other two,
    /// a policy rather than a server limit; see
    /// [`MAX_EVENT_DATA_LEN`](Self::MAX_EVENT_DATA_LEN).
    pub const MAX_TAGS_PER_EVENT: usize = 128;

    /// The largest number of events this store accepts in one append.
    ///
    /// Twice VT-24's floor of 128, and the one of the three where a **real**
    /// Postgres limit is close enough to be worth writing down. The extended
    /// query protocol carries its parameter count in an `int16`, so one
    /// statement binds at most 65,535 parameters; at eight columns per event
    /// that is 8,191 events in a single multi-row `INSERT … VALUES`. An append
    /// built out of array parameters instead — `UNNEST($1::bigint[], …)` — binds
    /// eight parameters whatever the batch size and has no such bound at all.
    ///
    /// Which of those two shapes `append` takes is not settled here, so this
    /// ceiling is deliberately far below both rather than derived from either:
    /// a number that encodes an unchosen SQL strategy is a decision taken by
    /// accident. `postgres-append-and-frontier-head` may raise it against
    /// evidence, and `append_reports_exceeded_store_limits` is what would find
    /// the current value wrong.
    pub const MAX_EVENTS_PER_BATCH: usize = 256;

    /// Wraps an existing pool.
    ///
    /// Takes a pool rather than a connection string because pool sizing,
    /// timeouts and TLS are the application's business, and because a store that
    /// builds its own pool cannot share one with the projection store beside it.
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool,
            store_id: Arc::new(OnceLock::new()),
            // `try_current` rather than `current`: `new` may legitimately be
            // called outside a runtime, and panicking there would make a library
            // out of a caller's ordering choice. The failure surfaces at the
            // first operation, as `NoRuntime`, if no runtime is found by then
            // either.
            runtime: Handle::try_current().ok(),
            visibility: Visibility::Frontier,
        }
    }

    /// A store with the visibility mechanism **removed**, for use as a negative
    /// control and for nothing else.
    ///
    /// # What this is for
    ///
    /// `nothing_below_an_observed_position_appears_later` (CF-13) is the rule
    /// this whole adapter exists to pass, and a rule no implementation can fail
    /// is decorative. This constructor builds the implementation it must fail:
    /// the same schema, the same append path, the same sequence — and `head` and
    /// `read` with the frontier predicate taken out, which is precisely the
    /// naive `nextval()` store the specification's ES-10 names in its
    /// `Rejects:`.
    ///
    /// Two arms of one adapter, not two adapters. Everything except the
    /// predicate under test is shared, so a rule that fails here is failing on
    /// the mechanism rather than on some unrelated difference.
    ///
    /// # Why it is behind a feature
    ///
    /// Off by default, so no consumer can reach it and the default build does
    /// not contain it. It is exercised once, by
    /// `tests/rule_controls.rs`, and recorded — it is not a second
    /// deliberately-broken fixture kept alive as a maintained instrument.
    #[cfg(feature = "naive-arm")]
    #[cfg_attr(docsrs, doc(cfg(feature = "naive-arm")))]
    pub fn new_naive(pool: PgPool) -> Self {
        Self {
            visibility: Visibility::Naive,
            ..Self::new(pool)
        }
    }

    /// The runtime this store's work runs on.
    ///
    /// The handle captured at construction first, then the caller's current one,
    /// and only then an error. The second chance matters: a store built outside
    /// a runtime and used inside one is a legitimate wiring order.
    ///
    /// # Errors
    ///
    /// [`PostgresEventStoreError::NoRuntime`] when there is no runtime in either
    /// place.
    fn runtime(&self) -> Result<Handle, PostgresEventStoreError> {
        self.runtime
            .clone()
            .or_else(|| Handle::try_current().ok())
            .ok_or(PostgresEventStoreError::NoRuntime)
    }

    /// Runs `work` on this store's runtime and waits for it.
    ///
    /// Every method that touches the server goes through here, so that "which
    /// thread am I on" is answered once rather than at four call sites.
    async fn on_runtime<T, F>(&self, work: F) -> Result<T, PostgresEventStoreError>
    where
        F: Future<Output = Result<T, PostgresEventStoreError>> + Send + 'static,
        T: Send + 'static,
    {
        self.runtime()?
            .spawn(work)
            .await
            .map_err(PostgresEventStoreError::Worker)?
    }

    /// This store's identity, read from `store_meta` and cached.
    ///
    /// # Errors
    ///
    /// [`PostgresEventStoreError::MissingIdentity`] if the row is absent, which
    /// means migration 1 has not been applied, and
    /// [`PostgresEventStoreError::MalformedIdentity`] if it is not sixteen
    /// bytes. Neither is spelled as a driver error, because "the schema is not
    /// there" and "the server refused the query" send a reader to different
    /// places.
    async fn store_id(&self) -> Result<StoreId, PostgresEventStoreError> {
        if let Some(cached) = self.store_id.get() {
            return Ok(*cached);
        }

        let pool = self.pool.clone();
        let stored: Option<Vec<u8>> = self
            .on_runtime(async move {
                sqlx::query_scalar("SELECT v FROM store_meta WHERE k = 'store_id'")
                    .fetch_optional(&pool)
                    .await
                    .map_err(PostgresEventStoreError::from)
            })
            .await?;
        let stored = stored.ok_or(PostgresEventStoreError::MissingIdentity)?;
        let bytes: [u8; 16] = stored
            .as_slice()
            .try_into()
            .map_err(|_| PostgresEventStoreError::MalformedIdentity { len: stored.len() })?;
        let minted = StoreId::from_bytes(bytes);

        // A losing `set` means another handle read the same committed row first.
        // Same value, so the loser simply uses what is there.
        Ok(*self.store_id.get_or_init(|| minted))
    }

    /// Refuses a batch that exceeds one of this store's stated ceilings.
    ///
    /// Checked before the transaction opens, so an over-limit batch costs no
    /// round trip. Reported as [`AppendError::ExceedsStoreLimit`] and never as
    /// `Store`: a sync runner has to tell "this will never fit here, park it"
    /// from "the disk is full, retry", and flattening the two into a driver
    /// error destroys the distinction. Truncation is forbidden outright.
    fn check_ceilings(events: &[Event]) -> Result<(), AppendError<PostgresEventStoreError>> {
        if events.len() > Self::MAX_EVENTS_PER_BATCH {
            return Err(AppendError::ExceedsStoreLimit {
                limit: StoreLimit::EventsPerBatch,
                len: events.len(),
            });
        }
        for event in events {
            if event.data().len() > Self::MAX_EVENT_DATA_LEN {
                return Err(AppendError::ExceedsStoreLimit {
                    limit: StoreLimit::EventDataLen,
                    len: event.data().len(),
                });
            }
            if event.tags().len() > Self::MAX_TAGS_PER_EVENT {
                return Err(AppendError::ExceedsStoreLimit {
                    limit: StoreLimit::TagsPerEvent,
                    len: event.tags().len(),
                });
            }
        }
        Ok(())
    }

    /// The pool this store reads and writes through.
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }
}

// `SendEventStore`, not `EventStore`. A `PgPool` is `Send + Sync` and every
// future below crosses threads happily; implementing the `Send` flavour gives
// the bare one for free, and the implication runs only in that direction.
impl SendEventStore for PostgresEventStore {
    type Error = PostgresEventStoreError;

    fn read(
        &self,
        query: &Query,
        options: ReadOptions,
    ) -> impl Stream<Item = Result<SequencedEvent, Self::Error>> + Send {
        // No `todo!()` here, and that is the point. `read` is not `async`, so
        // this is the one method whose *whole body* has to be real: acquiring
        // the connection is deferred into the stream's first poll, and the
        // laziness stops being a nicety and becomes load-bearing.
        PgReadStream::new(
            self.pool.clone(),
            self.runtime.clone(),
            self.visibility,
            query,
            options,
        )
    }

    /// Appends a batch and returns the position of its last event.
    ///
    /// # What `Ok(P)` does and does not promise
    ///
    /// It promises that the batch is committed, at positions ending at *P*, and
    /// that the append condition held when it was evaluated.
    ///
    /// It does **not** promise that the next [`head`](SendEventStore::head) is
    /// at or above *P*, and it does not promise that a read issued immediately
    /// afterwards contains what was just written. This store reports a
    /// visibility frontier, the frontier trails the positions it has already
    /// assigned, and **read-your-own-writes does not hold**.
    ///
    /// That is a documented **capability limit** of this adapter rather than a
    /// bug to report: there is no setting that turns it off, and the staleness
    /// is bounded by the longest open write transaction anywhere on the
    /// cluster — measured at a median of 0.593 ms with no holder, and 4,799 ms
    /// behind a five-second write transaction held in an unrelated database. A
    /// caller who needs the position immediately should keep the one this method
    /// returned rather than reading it back; the crate root explains why reading
    /// it back is not available here.
    ///
    /// # Errors
    ///
    /// [`AppendError::NoEvents`] for an empty batch, refused before the
    /// condition is evaluated; [`AppendError::ExceedsStoreLimit`] for a batch
    /// over one of this store's stated ceilings, refused before anything reaches
    /// the wire; [`AppendError::ConditionViolated`] when the condition found a
    /// matching event, which is not an adapter failure; and
    /// [`AppendError::Store`] for anything the driver or this adapter reports.
    async fn append(
        &self,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<Self::Error>> {
        // Step 1, before anything else touches the server. ES-18's rule
        // `empty_batch_is_refused_before_the_condition_is_evaluated` is explicit
        // that this precedes condition evaluation, so a zero-event call is never
        // an expensive no-op — and `events` stays borrowed, never collected to
        // satisfy the signature.
        if events.is_empty() {
            return Err(AppendError::NoEvents);
        }

        // Step 2, still before the wire. A ceiling refusal is
        // `ExceedsStoreLimit`, never `Store`: a sync runner has to be able to
        // tell "this will never fit here, park it" from "the disk is full,
        // retry", and flattening the two into a driver error destroys that.
        Self::check_ceilings(events)?;

        let store_id = self.store_id().await.map_err(AppendError::Store)?;

        // Owned, because the future crosses onto another runtime and must be
        // `'static`. `Event` is `Bytes`-backed, so the clone is a refcount bump
        // per event rather than a copy of the payload.
        let pool = self.pool.clone();
        let owned: Vec<Event> = events.to_vec();
        let condition = condition.cloned();
        let recorded_at = now();

        match self
            .on_runtime(async move {
                append_in_transaction(&pool, &owned, condition.as_ref(), recorded_at, store_id)
                    .await
            })
            .await
            .map_err(AppendError::Store)?
        {
            AppendOutcome::Committed(last) => Ok(last),
            // A condition violation is not an adapter failure, which is why
            // neither error enum in this crate carries a variant for it. It
            // travels as the contract's own `AppendError::ConditionViolated`.
            AppendOutcome::Violated(at) => Err(AppendError::ConditionViolated(
                at.map_or_else(ConditionViolated::unspecified, ConditionViolated::at),
            )),
        }
    }

    /// The highest position beneath this store's **visibility frontier**, or
    /// `None` when nothing is visible yet.
    ///
    /// # What a frontier is, and why it is not `max(position)`
    ///
    /// A head is a promise that nothing at or below it will appear later
    /// (ES-10). `max(position)` cannot make that promise here: `nextval()`
    /// allocates outside the transaction, so the highest position assigned can
    /// name a row whose predecessors are still in flight. Each row therefore
    /// stamps its appending transaction's `xid8`, and the frontier is
    /// `pg_snapshot_xmin(pg_current_snapshot())` — the transaction id below
    /// which nothing can still be running. Everything beneath it has settled,
    /// and that is what this method reports.
    ///
    /// The frontier legitimately **trails** the position
    /// [`append`](SendEventStore::append) just returned, so
    /// **read-your-own-writes does not hold** and this crate does not claim it;
    /// ES-30's `head_is_the_highest_visible_position` asserts a *bound* rather
    /// than an equality precisely to admit that. How far it trails is bounded by
    /// the longest open write transaction anywhere on the cluster — a median of
    /// 0.593 ms with no holder, and 4,799 ms behind a five-second write held in
    /// an unrelated database, against an unguarded arm unaffected by the same
    /// hold at 0.595 ms. A documented **capability limit**, not a tuning knob:
    /// nothing an operator configures moves it.
    ///
    /// # Errors
    ///
    /// The adapter's own error if the query fails, or if a stored position
    /// cannot be decoded — which is a corrupt store and is deliberately not
    /// spelled the same way as an empty one.
    async fn head(&self) -> Result<Option<SequencePosition>, Self::Error> {
        let pool = self.pool.clone();
        let predicate = self.visibility.predicate();
        let highest: Option<i64> = self
            .on_runtime(async move {
                sqlx::query_scalar(&format!(
                    "SELECT max(position) FROM event WHERE {predicate}"
                ))
                .fetch_one(&pool)
                .await
                .map_err(PostgresEventStoreError::from)
            })
            .await?;

        // `transpose`, not `and_then`: a stored value that fails to decode is a
        // corrupt store and must not be spelled the same way as an empty one.
        highest.map(position_from_row).transpose()
    }

    async fn contains_event_id(&self, id: EventId) -> Result<bool, Self::Error> {
        // # The frontier disagreement, answered rather than settled
        //
        // This asks whether the store *holds* an event, and under the frontier
        // mechanism that is a different question from whether `read` would yield
        // it: a committed row above the frontier is held but invisible. Both
        // answers are defensible and both are wrong somewhere.
        //
        // **This method answers `true` for a held-but-invisible row** — it does
        // **not** carry the frontier predicate. The reason is what each mistake
        // costs. Answering `false` would let a replication ingest re-accept an
        // event the store already holds, and duplicate history is not
        // recoverable by retrying; answering `true` makes this method briefly
        // disagree with `read`, and that disagreement resolves itself as the
        // frontier advances. A transient disagreement is cheaper than a
        // permanent duplicate.
        //
        // This is recorded, not settled. ES-41 stays `[PROVISIONAL]` and the
        // replication semantics that would settle it belong to the project that
        // owns ingest, not to this adapter.
        let store = id.store().to_bytes().to_vec();
        let position = as_i64(id.position());
        let pool = self.pool.clone();
        let found: Option<i32> = self
            .on_runtime(async move {
                sqlx::query_scalar(
                    "SELECT 1 FROM event WHERE origin_store = $1 AND origin_position = $2",
                )
                .bind(store)
                .bind(position)
                .fetch_optional(&pool)
                .await
                .map_err(PostgresEventStoreError::from)
            })
            .await?;
        Ok(found.is_some())
    }
}

/// How many times an append re-runs after a serialisation failure.
///
/// A conditional append runs `SERIALIZABLE`, so two writers racing the same
/// boundary produce a `40001` for one of them rather than two winners. The retry
/// is not a workaround for that: on the second attempt the loser *sees* the
/// winner's committed row and returns `ConditionViolated`, which is the answer
/// the caller was owed all along.
///
/// **This was three, and three is measurably not enough.** The justification it
/// carried — *"the retry is only ever needed once in the two-writer case"* — is
/// true of the two-writer case and says nothing about the case that actually
/// exhausts the budget. `happenstance-neon` re-derived the number against a live
/// endpoint and wrote down why — `NeonEventStore::SERIALISATION_ATTEMPTS` carries
/// the table, and it is deliberately named in prose rather than linked, because
/// `happenstance-neon` is not a dependency of this crate and no adapter may
/// depend on another:
/// `k_disjoint_boundaries_admit_exactly_k_commits` runs twelve contenders over
/// **four separate boundaries**, and the SSI predicate lock is *not* per
/// boundary — with a small table the planner takes a sequential scan and the
/// lock is relation-wide, so every contender conflicts with every other whatever
/// boundary it is racing. A retry that finds no conflict on its own boundary goes
/// straight back into the same fight, and the population drains one round at a
/// time instead of all at once. Measured there, four runs per value: **three
/// failed 2 of 4**, four and five were green 4 of 4, eight green 5 of 5.
///
/// **That cause belongs to Postgres and not to Neon** — Neon *is* Postgres over
/// one-shot HTTP, and the predicate lock is the server's. This adapter kept three
/// because nothing had re-measured it, and CI found it the first time the live
/// suite ran on a pull request: two contenders exhausted the budget and surfaced
/// `Failed("postgres rejected the work")` where the rule requires a commit or a
/// `ConditionViolated`.
///
/// **Eight alone did not fix it, and the reason is the whole finding.** Raised
/// from three, the rule failed again on CI with *more* exhausted contenders —
/// two at three attempts, five at eight. A budget is not a mechanism: without a
/// wait between them, every loser re-enters at once and collides with every other
/// loser, so extra attempts are extra transactions in the same fight. What was
/// missing is spacing, and [`backoff`] is where it now lives — read that first,
/// because this constant only bounds a loop that behaves the way that function
/// makes it behave.
///
/// **The number is adopted rather than re-measured here, and that is stated
/// rather than smoothed over.** Eight comes from Neon's table against a shared
/// cause; it is not four runs per value against a live PostgreSQL server, which
/// is the bar ADR-0024 sets for a choice like this. Re-measuring both this and
/// the backoff window together is owed, and should be done on a CI-sized runner
/// rather than a developer machine — local hardware is precisely what hid this.
///
/// It still costs nothing when it is not needed: a retry happens only after a
/// `40001`. When it is needed the cost is now eight round trips *plus* the waits,
/// which [`BACKOFF_CAP`] bounds at roughly a quarter of a second in the worst
/// case, paid only by a writer already losing a serialisation fight.
const SERIALISATION_ATTEMPTS: u32 = 8;

/// Postgres's `serialization_failure`.
const SERIALIZATION_FAILURE: &str = "40001";

/// An append's outcome, before it is mapped onto the port's error type.
enum AppendOutcome {
    /// Every event landed; the value is the last position assigned.
    Committed(SequencePosition),
    /// A guard matched. The position is a hint, not a promise.
    Violated(Option<SequencePosition>),
}

/// Runs one append: condition and insert on one snapshot, in one transaction.
///
/// # Why the isolation level depends on whether there is a condition
///
/// An unconditional append asserts nothing about the log's state, so it cannot
/// conflict with anything. It runs at the pool's default `READ COMMITTED` and
/// pays nothing.
///
/// A conditional append is a read and a write over the same predicate, which is
/// exactly the shape `READ COMMITTED` cannot make safe: two writers both find no
/// conflicting event, both insert, and both win. The other adapters in this
/// workspace avoid that by holding a write lock — `BEGIN IMMEDIATE` on SQLite —
/// which is precisely the move this crate exists *not* to make, because a store
/// that serialises every writer is `MemoryEventStore` with network latency.
///
/// `SERIALIZABLE` is the arm that buys the invariant without the lock. Postgres's
/// SSI is optimistic: it takes no locks and detects read-write dependency cycles
/// at commit, so two writers on **disjoint** boundaries never conflict and the
/// axis this crate occupies survives. Two writers on the **same** boundary do
/// conflict, and one of them is aborted — which is the elected loser.
///
/// This is a shape decision the spec left open (`_decomposition.md` Architecture
/// §9.3: whether Postgres reuses Neon's single-statement CTE or exploits the
/// interactive transaction it has and Neon does not). It is recorded here rather
/// than defaulted: the interactive transaction is exploited, because a CTE cannot
/// express "retry after a serialisation failure" and Neon's CTE exists to work
/// around the absence of a transaction rather than because it is better.
async fn append_in_transaction(
    pool: &PgPool,
    events: &[Event],
    condition: Option<&AppendCondition>,
    recorded_at: RecordedAt,
    store_id: StoreId,
) -> Result<AppendOutcome, PostgresEventStoreError> {
    let mut attempt = 0;
    loop {
        attempt += 1;
        match append_once(pool, events, condition, recorded_at, store_id).await {
            Err(error) if is_serialisation_failure(&error) && attempt < SERIALISATION_ATTEMPTS => {
                // Nothing was committed — Postgres aborted the whole
                // transaction — so re-running is not a partial retry. The next
                // attempt reads a log that now contains the winner's rows.
                //
                // Waiting first, and it is the half this loop was missing.
                tokio::time::sleep(backoff(attempt)).await;
            }
            other => return other,
        }
    }
}

/// The shortest wait between two attempts, doubled per attempt from here.
const BACKOFF_BASE: Duration = Duration::from_millis(2);

/// The longest wait between two attempts, whatever the exponent says.
///
/// Sixty-four milliseconds is a ceiling rather than a target: with full jitter
/// the *expected* wait is half the window, and `SERIALISATION_ATTEMPTS`
/// exhausting at this cap costs a caller roughly a quarter of a second in the
/// worst case — paid only by a writer already losing a serialisation fight.
const BACKOFF_CAP: Duration = Duration::from_millis(64);

/// How long to wait before re-running an aborted conditional append.
///
/// # Why a retry needs a wait at all, which this adapter did not know until CI
///
/// `SERIALISATION_ATTEMPTS`'s doc block explains why *more* attempts were needed
/// and it is only half the story; the other half is that attempts have to be
/// **spread out**, and nothing here was spreading them.
///
/// `k_disjoint_boundaries_admit_exactly_k_commits` races twelve contenders over
/// four boundaries. The SSI predicate lock is relation-wide on a small table, so
/// all twelve conflict with each other whatever boundary they are racing. With no
/// wait, every loser re-enters at once and collides with every other loser — a
/// thundering herd that a bigger budget makes *worse* rather than better, because
/// each extra attempt is another transaction in the same fight. Measured on CI:
/// the rule failed with two exhausted contenders at three attempts and **five** at
/// eight. The budget was the wrong lever.
///
/// **`happenstance-neon` does not need this and that is why the number did not
/// transfer.** Its retry costs a network round trip to a pooled proxy — on the
/// order of a hundred milliseconds — so its transport supplies the spacing for
/// free, and its doc block says so in terms: *"the retry itself is the backoff"*.
/// That sentence is true there and false here, where a retry against a pooled
/// local connection costs well under a millisecond. Adopting Neon's value without
/// its transport adopted half a mechanism.
///
/// # The shape: exponential, capped, full jitter
///
/// Full jitter — uniform in `[0, ceiling]` rather than `ceiling ± a bit` — is the
/// arm that actually decorrelates a herd. Equal-and-opposite jitter around a
/// common centre leaves the population clustered at that centre, which is the
/// defect being fixed.
///
/// The randomness is [`RandomState`] rather than a `rand` dependency. Each
/// `RandomState::new()` carries a fresh seed, so hashing a constant yields an
/// independent draw per call with no new licence surface, no advisory surface and
/// no MSRV exposure — which the workspace manifest is explicit about wanting for
/// a crate that needs exactly one thing.
///
/// # What it requires of the caller's runtime, and why that is not new
///
/// [`tokio::time::sleep`] needs a runtime with the timer enabled. This adds no
/// requirement: `sqlx`'s pool already enforces an acquire timeout, so a runtime
/// that could not tell the time could never have handed this store a connection.
fn backoff(attempt: u32) -> Duration {
    let shift = (attempt - 1).min(5);
    let ceiling = BACKOFF_BASE.saturating_mul(1_u32 << shift).min(BACKOFF_CAP);
    ceiling.mul_f64(jitter())
}

/// A uniform draw in `[0, 1]`, from the standard library and nothing else.
///
/// `RandomState::new()` is freshly seeded per instance, so the hash of a constant
/// differs between calls. The top thirty-two bits are taken because `f64::from`
/// is lossless on `u32` and a `u64`-to-`f64` cast is not — the same reason the
/// crate's lints would refuse the shorter spelling.
fn jitter() -> f64 {
    let mut hasher = RandomState::new().build_hasher();
    hasher.write_u8(0);
    let bits = u32::try_from(hasher.finish() >> 32).unwrap_or(u32::MAX);
    f64::from(bits) / f64::from(u32::MAX)
}

/// True for Postgres's `40001`, and only for it.
///
/// Matched on the `SQLSTATE` rather than on the message, and deliberately not
/// turned into an error variant: `sqlx` already exposes the code, and a variant
/// per `SQLSTATE` is the taxonomy this crate's `error.rs` explicitly refuses.
fn is_serialisation_failure(error: &PostgresEventStoreError) -> bool {
    let PostgresEventStoreError::Driver(sqlx::Error::Database(database)) = error else {
        return false;
    };
    database.code().as_deref() == Some(SERIALIZATION_FAILURE)
}

/// One attempt: begin, evaluate, insert, commit.
async fn append_once(
    pool: &PgPool,
    events: &[Event],
    condition: Option<&AppendCondition>,
    recorded_at: RecordedAt,
    store_id: StoreId,
) -> Result<AppendOutcome, PostgresEventStoreError> {
    let mut transaction = if condition.is_some() {
        pool.begin_with("BEGIN ISOLATION LEVEL SERIALIZABLE")
            .await?
    } else {
        pool.begin().await?
    };

    if let Some(condition) = condition
        && let Some(conflict) = evaluate(&mut transaction, condition).await?
    {
        {
            // The transaction is dropped without committing, so the store is
            // byte-identical to what it was —
            // `condition_rejection_leaves_store_unchanged` is the rule that
            // holds this to it.
            return Ok(AppendOutcome::Violated(Some(conflict)));
        }
    }

    let last = insert_batch(&mut transaction, events, recorded_at, store_id).await?;
    transaction.commit().await?;
    Ok(AppendOutcome::Committed(last))
}

/// Evaluates every guard, returning the highest conflicting position if any is
/// violated.
///
/// A guard is violated when its query matches an event at a position **strictly
/// greater** than `after`, or at any position when `after` is `None`. Any guard
/// violated violates the condition.
///
/// # No frontier predicate here, deliberately
///
/// `head` and `read` carry `xact_id < pg_snapshot_xmin(…)`; this does not. The
/// frontier exists so a *reader* never sees a position appear beneath one it has
/// already observed. A condition is not a read: it asks whether the store holds
/// a conflicting event at all, and a committed row above the frontier is held.
/// Filtering it out would admit an append the boundary forbids — the same silent
/// corruption this whole mechanism exists to prevent, arriving through the door
/// left open while the other one was being closed.
async fn evaluate(
    transaction: &mut Transaction<'static, Postgres>,
    condition: &AppendCondition,
) -> Result<Option<SequencePosition>, PostgresEventStoreError> {
    let mut highest: Option<i64> = None;

    for guard in condition.guards() {
        let mut next = 1;
        let predicate = crate::query_sql::predicate(&guard.query, &mut next);
        let boundary = guard.after.map_or(0, as_i64);

        // Each guard is parenthesised as a unit and AND'd with its own boundary.
        // The core crate's `AppendCondition` docs warn that an adapter
        // generating SQL must do exactly this, because the precedence bug that
        // threatens the single-guard form becomes n times more likely here.
        let sql = format!(
            "SELECT max(position) FROM event WHERE {} AND position > ${next}",
            predicate.sql()
        );

        let mut query = sqlx::query_scalar::<Postgres, Option<i64>>(&sql);
        for param in predicate.params() {
            query = match param {
                crate::query_sql::Param::EventType(value) => query.bind(value.clone()),
                crate::query_sql::Param::Tags(values) => query.bind(values.clone()),
            };
        }
        let found: Option<i64> = query.bind(boundary).fetch_one(&mut **transaction).await?;

        // `Option`'s own ordering does the merge: `None` sorts below every
        // `Some`, so this is `max` across the guards with no special case.
        highest = highest.max(found);
    }

    // Decoded before any comparison, so a corrupt stored value is a decode error
    // rather than a silently wrong verdict.
    highest.map(position_from_row).transpose()
}

/// Inserts the whole batch as one statement and returns the last position.
///
/// # Why positions are allocated in their own statement
///
/// `nextval('event_position_seq')` is read explicitly rather than as a column
/// default, so the allocation site is visible at the one statement that performs
/// it. Taking the values up front also lets the `origin_position` half of each
/// `EventId` be bound directly, instead of being stamped by a follow-up `UPDATE`
/// the way the SQLite adapter has to.
///
/// # Why one statement rather than a row at a time
///
/// The batch ceiling is 256 events at eight columns — 2,048 bound parameters,
/// comfortably inside the extended protocol's `int16` limit of 65,535, and one
/// round trip instead of 256.
async fn insert_batch(
    transaction: &mut Transaction<'static, Postgres>,
    events: &[Event],
    recorded_at: RecordedAt,
    store_id: StoreId,
) -> Result<SequencePosition, PostgresEventStoreError> {
    let count = i64::try_from(events.len()).unwrap_or(i64::MAX);
    let positions: Vec<i64> =
        sqlx::query_scalar("SELECT nextval('event_position_seq') FROM generate_series(1, $1)")
            .bind(count)
            .fetch_all(&mut **transaction)
            .await?;

    let store = store_id.to_bytes().to_vec();
    let mut builder = sqlx::QueryBuilder::<Postgres>::new(
        "INSERT INTO event \
         (position, event_type, data, metadata, tags, origin_store, origin_position, recorded_at) ",
    );
    builder.push_values(
        events.iter().zip(&positions),
        |mut row, (event, position)| {
            let tags: Vec<String> = event
                .tags()
                .iter()
                .map(|tag| tag.as_str().to_owned())
                .collect();
            row.push_bind(*position)
                .push_bind(event.event_type().as_str().to_owned())
                .push_bind(event.data().to_vec())
                .push_bind(event.metadata().map(|bytes| bytes.to_vec()))
                .push_bind(tags)
                .push_bind(store.clone())
                .push_bind(*position)
                .push_bind(recorded_at.as_millis());
        },
    );
    builder.build().execute(&mut **transaction).await?;

    let last = *positions
        .last()
        .expect("the empty batch was refused before this point");
    position_from_row(last)
}

/// Decodes a stored `bigint` into a [`SequencePosition`].
///
/// `bigint` is signed and admits `0`; `SequencePosition` is a `NonZeroU64`. The
/// rejected alternative is `SequencePosition::new(stored.unsigned_abs())`, which
/// reads like a guard and accepts every negative value, turning a stored `-3`
/// into position 3.
pub(crate) fn position_from_row(stored: i64) -> Result<SequencePosition, PostgresEventStoreError> {
    u64::try_from(stored)
        .ok()
        .and_then(SequencePosition::new)
        .ok_or(PostgresEventStoreError::PositionOutOfRange { value: stored })
}

/// A position as the `bigint` the schema stores.
fn as_i64(position: SequencePosition) -> i64 {
    i64::try_from(position.get()).unwrap_or(i64::MAX)
}

/// The wall clock, in the units [`RecordedAt`] carries.
fn now() -> RecordedAt {
    use std::time::{SystemTime, UNIX_EPOCH};

    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |elapsed| {
            i64::try_from(elapsed.as_millis()).unwrap_or(i64::MAX)
        });
    RecordedAt::from_millis(millis)
}

#[cfg(test)]
mod tests {
    use super::PostgresEventStore;
    use happenstance_core::EventStore;

    /// Constraint 4: generic code binds the *bare* flavour, and a `Send`
    /// implementer must satisfy it. Instantiating the bound at this concrete
    /// store is what checks that `trait_variant`'s derivation actually reached
    /// this impl — an uninstantiated generic proves nothing.
    #[test]
    fn send_flavour_satisfies_the_bare_bound() {
        fn assert_event_store<S: EventStore>() {}
        assert_event_store::<PostgresEventStore>();
    }

    /// The store itself must cross threads, or the pool buys nothing.
    #[test]
    fn store_is_send_and_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<PostgresEventStore>();
    }
}
