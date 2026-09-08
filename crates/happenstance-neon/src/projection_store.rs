//! The Neon-backed [`ProjectionStore`], and the batch that is not a transaction.
//!
//! # What this falsified, and what the port did about it
//!
//! [`ProjectionStore`]'s `Batch` **used to be** a generic associated type,
//! `type Batch<'a> where Self: 'a`, because "a transaction cannot outlive the
//! connection that opened it". Neon has no connection and no transaction, so the
//! lifetime had nothing to borrow from. This crate bound an **owned** type to the
//! GAT — `type Batch<'a> = NeonWriteBatch;` — and it compiled, because a GAT is
//! free to ignore its parameter. ADR-0017 has since removed the parameter, and
//! the binding below is plainly `type Batch = NeonWriteBatch;`.
//!
//! That made this adapter the owned-batch evidence from the far end of the
//! transport axis, and it surfaced two places where the port asked for something
//! this adapter had no way to mean. Both have since been answered rather than
//! left standing. [`ProjectionStore::begin`] is no longer `async` and no longer
//! fallible: there is nothing here to open and nothing that can fail — `begin` is
//! a `Vec::new()` and a stamp — and this adapter is the one PS-6 was written for.
//! [`ProjectionStore::rollback`] is still both, deliberately, because `Drop`
//! cannot await and an adapter holding a real resource needs somewhere to release
//! it; this one holds none, so dropping the statement list is the whole of its
//! rollback.
//!
//! # Where the atomicity actually comes from
//!
//! The port's invariant is that the read-model write and the checkpoint write
//! land together. This adapter gets that not from a transaction handle but from
//! the **non-interactive batch**: `commit` appends the checkpoint `UPSERT` to the
//! accumulated statements and sends all of them as one array in one round trip,
//! which the endpoint runs server-side inside one `BEGIN`/`COMMIT`. The invariant
//! is honoured. What is lost is the ability to *decide* anything between two
//! statements of the batch — which the projection port, unlike the event store
//! port, never asks for.
//!
//! # Except in exactly one place, and this is how that is paid for
//!
//! PS-16's checkpoint regression is a decision: refuse, and leave the read model
//! as it was. A guarded upsert reports a refusal as **zero rows**, and zero rows
//! do not abort anything — so the caller's statements, already earlier in the
//! same batch, would commit with no checkpoint recording them. That is a partial
//! application, which is precisely what PS-1 forbids.
//!
//! `happenstance-postgres` answers it by re-reading inside the transaction and
//! then *dropping* the transaction. There is no transaction here to drop, and no
//! second look to take: the batch is decided before it is sent.
//!
//! So the refusal is made to **fail**, in one more statement of the same batch,
//! and the failure carries its own evidence. After the guarded upsert, a
//! statement reads the checkpoint back and casts a marker string to `bigint`
//! when — and only when — the recorded position is not the one that was
//! attempted. The cast raises SQLSTATE `22P02` with the message `invalid input
//! syntax for type bigint: "hs-regression:<current>:<attempted>"`, which aborts
//! the whole batch — read-model statements included — and reconstructs
//! [`CommitError::CheckpointRegression`] with both positions and **no second
//! round trip**.
//!
//! Two alternatives lost, and both are worth naming because the one chosen looks
//! like a hack and is the only one that is not:
//!
//! * **A `plpgsql` function raising with a custom `ERRCODE`** is the textbook
//!   answer and gives a nicer SQLSTATE. It needs a dollar-quoted body in
//!   `migrations/`, which is exactly what [`crate::migration`]'s deliberately
//!   small splitter cannot see the end of — so it would buy a tidier error by
//!   putting a SQL parser in the migration path.
//! * **Read the checkpoint first, then commit** is two round trips and a
//!   time-of-check race: another runner may advance the checkpoint between the
//!   read and the batch, and the batch would then commit a regression it had
//!   already checked. The single-batch form has no window at all, because the
//!   upsert's `ON CONFLICT DO UPDATE` takes the row lock the guard then reads
//!   under.

use core::sync::atomic::{AtomicU64, Ordering};

use happenstance_core::{
    Authority, Checkpoint, CommitError, ProjectionId, ProjectionStore, ResetError, SequencePosition,
};

use crate::config::NeonConfig;
use crate::error::NeonError;
use crate::transport::{HttpResponse, IsolationLevel, SqlRequest, SqlStatement, SqlTransport};
use crate::wire::ResponseBody;

/// What the checkpoint guard's failing cast puts in front of the two positions.
///
/// A marker rather than a bare `current:attempted`, so that the decoder can tell
/// this adapter's deliberate abort from a `22P02` raised by a caller's own
/// statement casting a genuinely bad string. Without it, a read model whose
/// migration turned a `text` column into a `bigint` would be reported as a
/// checkpoint regression.
const REGRESSION_MARKER: &str = "hs-regression:";

/// `Authority::Live` as migration 2's `CHECK` spells it.
const AUTHORITY_LIVE: &str = "live";
/// `Authority::Rebuilding` as migration 2's `CHECK` spells it.
const AUTHORITY_REBUILDING: &str = "rebuilding";

/// The stamp [`NeonWriteBatch::new`] applies, and which no store ever mints.
///
/// See that constructor for why a public `new` on a batch whose whole defence is
/// its provenance has to hand back something every `commit` rejects.
const UNSTAMPED: u64 = 0;

/// Mints the identity one store instance stamps its batches with.
///
/// A **process-global** ordinal, taken once per store at construction. Both
/// properties are about what must not happen, and the other adapters record the
/// same two: it must not be derived from a pointer address, because an allocation
/// can be freed and a new one land where the old one was, giving two stores one
/// identity; and it must not be minted in `Clone`, because a clone is the same
/// store and must accept its origin's batches.
///
/// A counter held *per store* and incremented at [`ProjectionStore::begin`] is
/// the trap that looks equivalent: it hands store A and store B the identical
/// sequence 1, 2, 3…, and the foreign-batch check never fires.
fn mint_stamp() -> u64 {
    static NEXT: AtomicU64 = AtomicU64::new(UNSTAMPED + 1);
    NEXT.fetch_add(1, Ordering::Relaxed)
}

/// An accumulated list of statements, owned outright.
///
/// Not a transaction. Nothing has been sent when one of these exists, and nothing
/// is holding a lock, a snapshot or a connection — because there is no connection
/// to hold. It is a `Vec<SqlStatement>` that becomes a single non-interactive
/// transaction at [`commit`](ProjectionStore::commit) time and is otherwise inert.
///
/// This is what `type Batch` is bound to in the impl below, and it was what the
/// GAT's `'a` was bound to before ADR-0017 removed it: an owned type that never
/// had a use for the parameter.
#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct NeonWriteBatch {
    /// The statements to run, in order, inside one server-side transaction.
    pub statements: Vec<SqlStatement>,
    /// The identity of the store that began this batch.
    ///
    /// Private, and that is the whole of the foreign-batch defence:
    /// [`ProjectionStore::begin`] is the only thing that can put a real value
    /// here.
    stamp: u64,
}

impl NeonWriteBatch {
    /// An empty batch that **no store will commit**.
    ///
    /// This constructor predates the store stamp and is kept, `const`, because
    /// removing it would be a breaking change to buy nothing — but what it hands
    /// back has to be said out loud, because the type looks usable and is not.
    /// A batch carries the identity of the store that began it, and
    /// [`ProjectionStore::begin`] is the only way to obtain that identity. So a
    /// batch from here is stamped [`UNSTAMPED`], which every `commit`, `reset`
    /// and `rollback` rejects as [`CommitError::ForeignBatch`].
    ///
    /// The alternative was to stamp it with the *next* real identity, which
    /// would make it accepted by no store and rejected by all of them with a
    /// different sentence — the same outcome, reached less legibly. Handing back
    /// something plainly foreign is the honest shape: a caller who reaches for
    /// this instead of `begin` finds out on the first commit rather than on the
    /// first concurrent one.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            statements: Vec::new(),
            stamp: UNSTAMPED,
        }
    }

    /// An empty batch stamped with `stamp`. Private; `begin` is the only caller.
    const fn stamped(stamp: u64) -> Self {
        Self {
            statements: Vec::new(),
            stamp,
        }
    }

    /// Queues a statement.
    pub fn push(&mut self, statement: SqlStatement) {
        self.statements.push(statement);
    }

    /// How many statements are queued.
    ///
    /// Worth watching: the whole batch travels in one request body, so a
    /// projection that applies ten thousand events before committing builds a
    /// ten-thousand-statement JSON document and sends it in one round trip.
    #[must_use]
    pub fn len(&self) -> usize {
        self.statements.len()
    }

    /// Whether anything is queued.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.statements.is_empty()
    }
}

/// A projection store over Neon's serverless `/sql` HTTP endpoint.
///
/// ```
/// use happenstance_core::ProjectionStore;
/// use happenstance_neon::{NeonConfig, NeonProjectionStore, NullTransport};
///
/// fn takes_a_projection_store<P: ProjectionStore>(_store: &P) {}
///
/// let store = NeonProjectionStore::new(NullTransport::new(), NeonConfig::default());
/// takes_a_projection_store(&store);
/// ```
#[derive(Debug, Clone)]
pub struct NeonProjectionStore<T> {
    transport: T,
    config: NeonConfig,
    /// This instance's identity, compared against the one every
    /// [`NeonWriteBatch`] carries.
    ///
    /// Copied by `Clone` rather than re-minted, because a clone is the same store
    /// and must accept its origin's batches.
    stamp: u64,
}

impl<T> NeonProjectionStore<T> {
    /// Builds a projection store over `transport`.
    ///
    /// **No longer `const`**, and the reason is the stamp above: an identity that
    /// distinguishes two store instances cannot be a compile-time constant,
    /// because a `const fn` would give every instance the same one and
    /// `commit_rejects_a_foreign_batch` would pass only for stores that never
    /// existed at the same time.
    #[must_use]
    pub fn new(transport: T, config: NeonConfig) -> Self {
        Self {
            transport,
            config,
            stamp: mint_stamp(),
        }
    }

    /// The transport this store sends round trips through.
    pub const fn transport(&self) -> &T {
        &self.transport
    }

    /// The table names and limits this store was built with.
    pub const fn config(&self) -> &NeonConfig {
        &self.config
    }
}

impl<T: SqlTransport> NeonProjectionStore<T> {
    /// The `SELECT` that reads one projection's checkpoint.
    fn checkpoint_request(&self, id: &ProjectionId) -> SqlRequest {
        let sql = format!(
            "SELECT position::text AS position, authority FROM {} WHERE projection_id = $1",
            self.config.qualified_checkpoint()
        );
        SqlRequest::single(SqlStatement::with_params(
            sql,
            vec![serde_json::Value::String(id.as_str().to_owned())],
        ))
        .read_only()
    }

    /// The batch's statements, the checkpoint `UPSERT` and its guard, as one
    /// array.
    ///
    /// `ReadCommitted` explicitly rather than
    /// [`NeonConfig::isolation`](crate::NeonConfig::isolation), and it is a
    /// decision rather than an oversight: `ON CONFLICT DO UPDATE` re-reads the
    /// conflicting row under a row lock, which is the serialisation this needs,
    /// and `SERIALIZABLE` would turn every contended commit on one projection
    /// into a `40001` this store has no retry loop for. The event store's append
    /// needs the stronger level because its guard is a *predicate* over rows that
    /// do not exist yet; a checkpoint is one row with a primary key.
    fn commit_request(
        &self,
        batch: NeonWriteBatch,
        id: &ProjectionId,
        position: SequencePosition,
        authority: Authority,
    ) -> SqlRequest {
        let checkpoint = self.config.qualified_checkpoint();
        let mut statements = batch.statements;
        let stored = as_i64(position).to_string();

        statements.push(SqlStatement::with_params(
            format!(
                "INSERT INTO {checkpoint} AS c (projection_id, position, authority) \
                 VALUES ($1, $2::bigint, $3) \
                 ON CONFLICT (projection_id) DO UPDATE \
                 SET position = excluded.position, authority = excluded.authority \
                 WHERE c.position <= excluded.position"
            ),
            vec![
                serde_json::Value::String(id.as_str().to_owned()),
                serde_json::Value::String(stored.clone()),
                serde_json::Value::String(authority_to_row(authority).to_owned()),
            ],
        ));

        // The guard. See the module documentation for why a refusal has to be a
        // *failure* here rather than zero rows, and for the two alternatives that
        // lost.
        statements.push(SqlStatement::with_params(
            format!(
                "SELECT CASE WHEN c.position = $2::bigint THEN '0' \
                 ELSE '{REGRESSION_MARKER}' || c.position || ':' || $2::bigint END::bigint AS ok \
                 FROM {checkpoint} c WHERE c.projection_id = $1"
            ),
            vec![
                serde_json::Value::String(id.as_str().to_owned()),
                serde_json::Value::String(stored),
            ],
        ));

        SqlRequest::batch(statements, IsolationLevel::ReadCommitted)
    }

    /// The batch's statements plus the checkpoint `DELETE`, as one array.
    ///
    /// The delete is the whole of a reset's own contribution: absence of the row
    /// **is** [`Checkpoint::NeverRun`], which is why this does not write a
    /// sentinel. A sentinel position would make a reset indistinguishable from a
    /// commit at the first position, which is the defect PS-16's rule exists to
    /// reject.
    fn reset_request(&self, batch: NeonWriteBatch, id: &ProjectionId) -> SqlRequest {
        let mut statements = batch.statements;
        statements.push(SqlStatement::with_params(
            format!(
                "DELETE FROM {} WHERE projection_id = $1",
                self.config.qualified_checkpoint()
            ),
            vec![serde_json::Value::String(id.as_str().to_owned())],
        ));
        SqlRequest::batch(statements, IsolationLevel::ReadCommitted)
    }
}

// The `+ 'static` that used to sit on this `impl` header is gone, and its removal
// is a consequence rather than a choice.
//
// `ProjectionStore` declared `type Batch<'a> where Self: 'a` and then took
// `Self::Batch<'_>` by value in `commit` and `rollback`. That anonymous lifetime
// was late-bound and universally quantified, so the compiler had to discharge
// `NeonProjectionStore<T>: 'a` for *every* `'a` — which is `T: 'static` and
// nothing weaker. Without it, `cargo check` reported four `error[E0311]`, two of
// them pointing into `happenstance-core/src/projection.rs` itself.
//
// The consequence was a port constraint nobody had written down: **any
// projection-store adapter generic over a type parameter was forced to `'static`
// by the GAT**, whether or not its batch borrowed anything. This one's batch
// borrows nothing at all. That finding went to phase 6, ADR-0017 removed the
// lifetime, and with no `'a` left to quantify over the obligation has nothing to
// discharge — so the bound goes with it.
impl<T: SqlTransport> ProjectionStore for NeonProjectionStore<T> {
    type Error = NeonError<T::Error>;

    // The same owned batch as before, now bound to an associated type that no
    // longer carries a lifetime this adapter had no use for.
    type Batch = NeonWriteBatch;

    // No round trip, and now no `async` and no `Result` either. There is nothing
    // to open and nothing that can fail: this adapter is the one PS-6 is written
    // for, and the shape finally says so.
    fn begin(&self) -> Self::Batch {
        NeonWriteBatch::stamped(self.stamp)
    }

    /// Reads `id`'s checkpoint, or [`Checkpoint::NeverRun`] when it has no row.
    ///
    /// # Errors
    ///
    /// The adapter's error if the round trip fails, or if the row is one this
    /// adapter could not have written.
    async fn checkpoint(&self, id: &ProjectionId) -> Result<Checkpoint, Self::Error> {
        let request = self.checkpoint_request(id);
        let response = self
            .transport
            .round_trip(request)
            .await
            .map_err(NeonError::Transport)?;
        decode_checkpoint::<T::Error>(&response, self.config.max_response_bytes)
    }

    /// Applies `batch` and advances `id`'s checkpoint to `position`, as one unit.
    ///
    /// # Errors
    ///
    /// * [`CommitError::ForeignBatch`] if `batch` was begun on a different store
    ///   instance — or by [`NeonWriteBatch::new`], which begins nothing — decided
    ///   before anything is serialised;
    /// * [`CommitError::CheckpointRegression`] if `position` is below the one
    ///   recorded; equal is accepted, because a batch that wrote nothing new
    ///   still records that its events were considered;
    /// * [`CommitError::Store`] if the transport or the endpoint failed.
    async fn commit(
        &self,
        batch: Self::Batch,
        id: &ProjectionId,
        position: SequencePosition,
        authority: Authority,
    ) -> Result<(), CommitError<Self::Error>> {
        if batch.stamp != self.stamp {
            // Before anything is serialised, which is what makes "neither store
            // moved" structural rather than something to remember.
            return Err(CommitError::ForeignBatch);
        }

        // The only round trip in the whole port, and the only moment at which any
        // of this became durable.
        let request = self.commit_request(batch, id, position, authority);
        let response = self
            .transport
            .round_trip(request)
            .await
            .map_err(|error| CommitError::Store(NeonError::Transport(error)))?;
        decode_commit::<T::Error>(&response, self.config.max_response_bytes, position)
    }

    /// Applies `batch` and returns `id` to [`Checkpoint::NeverRun`], as one unit.
    ///
    /// `commit`'s dual. It deletes nothing this adapter chose: the read model is
    /// the caller's, and an adapter that truncated a table of its own naming
    /// would be scoped to something the port deliberately never told it.
    ///
    /// # Errors
    ///
    /// * [`ResetError::ForeignBatch`] if `batch` was begun on a different store
    ///   instance;
    /// * [`ResetError::Store`] if the transport or the endpoint failed.
    ///
    /// [`ResetError::Refused`] is never returned: this store holds no protection
    /// policy, which is PS-18's mechanism left unexercised by a store with
    /// nothing to protect.
    async fn reset(
        &self,
        batch: Self::Batch,
        id: &ProjectionId,
    ) -> Result<(), ResetError<Self::Error>> {
        if batch.stamp != self.stamp {
            return Err(ResetError::ForeignBatch);
        }

        let request = self.reset_request(batch, id);
        let response = self
            .transport
            .round_trip(request)
            .await
            .map_err(NeonError::Transport)
            .map_err(ResetError::Store)?;
        decode_plain::<T::Error>(&response, self.config.max_response_bytes)
            .map_err(ResetError::Store)
    }

    /// Discards the batch without committing.
    ///
    /// Nothing was ever sent, so this is a drop — which is also the evidence PS-7
    /// asks for: for a buffering adapter, "dropping a batch must roll back" is
    /// free, and the store is usable afterwards because nothing was ever checked
    /// out to return.
    ///
    /// # Errors
    ///
    /// [`NeonError::ForeignBatch`] if `batch` was begun on a different store
    /// instance. The batch is consumed either way; the refusal is how a caller
    /// learns it was holding the wrong one, on the call that was meant to be the
    /// cleanup.
    async fn rollback(&self, batch: Self::Batch) -> Result<(), Self::Error> {
        if batch.stamp != self.stamp {
            return Err(NeonError::ForeignBatch);
        }
        drop(batch);
        Ok(())
    }
}

/// The checkpoint row, or [`Checkpoint::NeverRun`] when there is none.
///
/// An absent row is `NeverRun` rather than `None`: the port's return type is a
/// three-variant enum, so the row must also carry which authority the last commit
/// claimed.
fn decode_checkpoint<E>(
    response: &HttpResponse,
    max_response_bytes: usize,
) -> Result<Checkpoint, NeonError<E>> {
    let body = read_body::<E>(response, max_response_bytes)?;
    let Some(result) = body.result_sets().first() else {
        return Err(NeonError::MissingColumn { column: "results" });
    };
    let Some(position) = result.text(0, "position") else {
        return Ok(Checkpoint::NeverRun);
    };
    let through = position_from_text(position)?;
    match result.text(0, "authority") {
        Some(AUTHORITY_LIVE) => Ok(Checkpoint::Live { through }),
        Some(AUTHORITY_REBUILDING) => Ok(Checkpoint::Rebuilding { through }),
        _ => Err(NeonError::MissingColumn {
            column: "authority",
        }),
    }
}

/// Confirms every statement in the batch reported success, and reads a refusal
/// out of the failure when it did not.
fn decode_commit<E>(
    response: &HttpResponse,
    max_response_bytes: usize,
    attempted: SequencePosition,
) -> Result<(), CommitError<NeonError<E>>> {
    match read_body::<E>(response, max_response_bytes) {
        Ok(_) => Ok(()),
        Err(NeonError::Sql(sql)) => {
            if let Some(current) = regression_position(&sql.message) {
                // Both values, from the message the guard put them in. The
                // alternative is a second round trip to ask what the store had —
                // which this endpoint would answer against a *later* snapshot,
                // so the pair reported would not be the pair the guard saw.
                return Err(CommitError::CheckpointRegression {
                    current: current.map_err(CommitError::Store)?,
                    attempted,
                });
            }
            Err(CommitError::Store(NeonError::Sql(sql)))
        }
        Err(other) => Err(CommitError::Store(other)),
    }
}

/// The recorded position out of the guard's deliberate cast failure, if that is
/// what this message is.
///
/// `None` means "some other `22P02`", which must stay a plain store error — a
/// caller's own read-model statement casting a bad string is not a checkpoint
/// regression, and reporting it as one would send an operator to the wrong table.
fn regression_position<E>(message: &str) -> Option<Result<SequencePosition, NeonError<E>>> {
    let marked = message.split_once(REGRESSION_MARKER)?.1;
    let (current, _) = marked.split_once(':')?;
    Some(position_from_text(current))
}

/// Confirms the endpoint accepted the whole batch.
fn decode_plain<E>(response: &HttpResponse, max_response_bytes: usize) -> Result<(), NeonError<E>> {
    read_body::<E>(response, max_response_bytes).map(|_| ())
}

/// Size, then status, then shape — the order the failures actually happen in.
fn read_body<E>(
    response: &HttpResponse,
    max_response_bytes: usize,
) -> Result<ResponseBody, NeonError<E>> {
    if response.body.len() > max_response_bytes {
        return Err(NeonError::ResponseTooLarge {
            bytes: response.body.len(),
            limit: max_response_bytes,
        });
    }
    if !response.is_success() {
        return Err(
            serde_json::from_slice::<crate::error::NeonSqlError>(&response.body).map_or_else(
                |_| NeonError::Http {
                    status: response.status,
                    body: String::from_utf8_lossy(&response.body)
                        .chars()
                        .take(512)
                        .collect::<String>()
                        .into(),
                },
                NeonError::Sql,
            ),
        );
    }
    ResponseBody::parse(&response.body).map_err(NeonError::MalformedResponse)
}

/// A stored `bigint`, which arrives as a JSON string, as a [`SequencePosition`].
fn position_from_text<E>(stored: &str) -> Result<SequencePosition, NeonError<E>> {
    let value: i64 = stored
        .parse()
        .map_err(|_| NeonError::InvalidPosition { value: 0 })?;
    u64::try_from(value)
        .ok()
        .and_then(SequencePosition::new)
        .ok_or(NeonError::InvalidPosition { value })
}

/// A [`SequencePosition`] as the `bigint` the schema stores.
fn as_i64(position: SequencePosition) -> i64 {
    i64::try_from(position.get()).unwrap_or(i64::MAX)
}

/// [`Authority`] as it is stored.
///
/// `Authority` is `#[non_exhaustive]`, so a variant this adapter has never seen
/// is possible. It is written as a value migration 2's `CHECK` refuses, rather
/// than silently stored as `live` — reporting a half-finished rebuild as a live
/// projection is the failure this arm exists to prevent, and a constraint
/// violation names it where it happened.
const fn authority_to_row(authority: Authority) -> &'static str {
    match authority {
        Authority::Live => AUTHORITY_LIVE,
        Authority::Rebuilding => AUTHORITY_REBUILDING,
        _ => "unknown",
    }
}

/// The suite's own read model, as SQL rendered for one configuration.
///
/// Behind the `conformance` feature, because a test table shipped inside an
/// application's database is a defect no test in this repository could catch —
/// every test enables the feature. It is deliberately not a file in
/// `migrations/`: a `.sql` file there is, by construction, something an operator
/// applies.
#[cfg(feature = "conformance")]
#[must_use]
pub fn probe_table(config: &NeonConfig) -> String {
    format!(
        "CREATE TABLE IF NOT EXISTS {} (k text NOT NULL PRIMARY KEY, v bigint NOT NULL)",
        qualified_probe(config)
    )
}

/// The probe table, quoted and schema-qualified.
#[cfg(feature = "conformance")]
fn qualified_probe(config: &NeonConfig) -> String {
    format!(
        "{}.{}",
        crate::config::quote_ident(&config.schema),
        crate::config::quote_ident("projection_probe")
    )
}

/// The conformance suite's write and read seam.
///
/// It lives here, in `src/`, and not in `tests/`: that is a different crate,
/// where neither [`ProjectionProbe`] nor [`NeonProjectionStore`] is local and the
/// orphan rule answers `error[E0117]`.
///
/// [`ProjectionProbe`]: happenstance_core::ProjectionProbe
#[cfg(feature = "conformance")]
impl<T: SqlTransport> happenstance_core::ProjectionProbe for NeonProjectionStore<T> {
    /// `false`, and it is **forced** rather than chosen.
    ///
    /// Two things make it so and either would be enough. A [`NeonWriteBatch`] is
    /// a list of statements that has been sent to the endpoint exactly never, so
    /// there is no open transaction to read through — and answering from
    /// *committed* state is what PS-12 forbids by name. And
    /// [`probe_read_through`](happenstance_core::ProjectionProbe::probe_read_through)
    /// is synchronous and infallible, while every answer this adapter can give
    /// costs an HTTPS round trip.
    ///
    /// The two rules that would have used it are emitted as reported skips
    /// carrying this reason rather than omitted.
    const READS_THROUGH_BATCH: bool = false;

    /// Queues one probe row. Synchronous and infallible, because queueing a
    /// statement into a buffer the caller owns cannot fail.
    fn probe_write(&self, batch: &mut Self::Batch, key: &str, value: u64) {
        batch.push(SqlStatement::with_params(
            format!(
                "INSERT INTO {} (k, v) VALUES ($1, $2::bigint) \
                 ON CONFLICT (k) DO UPDATE SET v = excluded.v",
                qualified_probe(&self.config)
            ),
            vec![
                serde_json::Value::String(key.to_owned()),
                // `cast_signed` rather than a fallible conversion: `bigint` is
                // signed, the bit pattern round-trips every `u64` exactly, and a
                // saturating conversion would map two values to one.
                serde_json::Value::String(value.cast_signed().to_string()),
            ],
        ));
    }

    /// Queues removal of every probe row, so that
    /// [`reset`](happenstance_core::ProjectionStore::reset) can be checked
    /// without the suite knowing what a read model is.
    fn probe_delete_all(&self, batch: &mut Self::Batch) {
        batch.push(SqlStatement::new(format!(
            "DELETE FROM {}",
            qualified_probe(&self.config)
        )));
    }

    /// Reads one probe row from **committed** state.
    ///
    /// # Errors
    ///
    /// The adapter's error if the round trip fails or the answer does not have
    /// the shape this adapter asked for.
    async fn probe_read(&self, key: &str) -> Result<Option<u64>, Self::Error> {
        let sql = format!(
            "SELECT v::text AS v FROM {} WHERE k = $1",
            qualified_probe(&self.config)
        );
        let response = self
            .transport
            .round_trip(
                SqlRequest::single(SqlStatement::with_params(
                    sql,
                    vec![serde_json::Value::String(key.to_owned())],
                ))
                .read_only(),
            )
            .await
            .map_err(NeonError::Transport)?;
        let body = read_body::<T::Error>(&response, self.config.max_response_bytes)?;
        let Some(result) = body.result_sets().first() else {
            return Err(NeonError::MissingColumn { column: "results" });
        };
        result
            .text(0, "v")
            .map(|stored| {
                stored
                    .parse::<i64>()
                    .map(i64::cast_unsigned)
                    .map_err(|_| NeonError::InvalidPosition { value: 0 })
            })
            .transpose()
    }

    /// Never called, because
    /// [`READS_THROUGH_BATCH`](happenstance_core::ProjectionProbe::READS_THROUGH_BATCH)
    /// is `false`.
    ///
    /// # Panics
    ///
    /// Always. The specification permits exactly this for an adapter whose batch
    /// has no read path, and the panic is the honest answer: any value returned
    /// here would be a claim about pending writes the endpoint has never been
    /// told about.
    fn probe_read_through(&self, _batch: &Self::Batch, _key: &str) -> Option<u64> {
        unimplemented!(
            "a NeonWriteBatch has been sent to the endpoint exactly never, so \
             `READS_THROUGH_BATCH` is `false` and this is never called: there is no \
             open transaction to read through, and answering from committed state is \
             what PS-12 forbids"
        )
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::{NeonProjectionStore, NeonWriteBatch, decode_checkpoint, decode_commit};
    use crate::config::NeonConfig;
    use crate::error::NeonError;
    use crate::transport::{HttpResponse, NullTransport};
    use happenstance_core::{
        Authority, Checkpoint, CommitError, ProjectionId, ProjectionStore, SequencePosition,
    };

    fn projection_id() -> ProjectionId {
        ProjectionId::new("p")
    }

    fn store() -> NeonProjectionStore<NullTransport> {
        NeonProjectionStore::new(
            NullTransport::new(),
            NeonConfig::default().with_schema("hsp_1"),
        )
    }

    /// The bare projection-store flavour, at a concrete transport.
    fn assert_bare_flavour<P: ProjectionStore>() {}

    #[test]
    fn the_store_is_a_bare_projection_store() {
        assert_bare_flavour::<NeonProjectionStore<NullTransport>>();
    }

    /// The owned batch really is owned: it outlives the borrow it was made from,
    /// which a `rusqlite::Transaction<'a>`-shaped batch could not.
    #[test]
    fn the_batch_owns_itself() {
        let batch = {
            let store = store();
            store.begin()
        };
        assert!(batch.is_empty());
    }

    /// Two stores are strangers, and a hand-built batch is a stranger to both.
    #[test]
    fn only_begin_produces_a_committable_batch() {
        let left = store();
        let right = store();
        assert!(matches!(
            poll_once(right.commit(
                left.begin(),
                &projection_id(),
                SequencePosition::new(1).unwrap(),
                Authority::Live,
            )),
            Err(CommitError::ForeignBatch)
        ));
        assert!(matches!(
            poll_once(left.commit(
                NeonWriteBatch::new(),
                &projection_id(),
                SequencePosition::new(1).unwrap(),
                Authority::Live,
            )),
            Err(CommitError::ForeignBatch),
        ));
    }

    /// The smallest driver that will run a future to completion here.
    ///
    /// Every path under test refuses **before** it reaches the transport, so the
    /// future never yields — which is why one poll is sufficient and why this
    /// crate acquires no executor dependency to prove it. `Waker::noop` is what
    /// makes that possible without `unsafe`, which the workspace forbids rather
    /// than denies: the hand-rolled `RawWakerVTable` this would otherwise need is
    /// exactly the shape `unsafe_code = "forbid"` exists to keep out of a test
    /// helper.
    #[allow(
        unused_qualifications,
        reason = "`Future` is not in scope in this module"
    )]
    fn poll_once<F: core::future::Future>(future: F) -> F::Output {
        use core::task::{Context, Poll, Waker};

        let waker = Waker::noop();
        let mut context = Context::from_waker(waker);
        let mut future = core::pin::pin!(future);
        match future.as_mut().poll(&mut context) {
            Poll::Ready(output) => output,
            Poll::Pending => panic!("a refusal decided before the transport cannot pend"),
        }
    }

    /// The commit's last two statements are the upsert and its guard.
    #[test]
    fn a_commit_appends_the_upsert_and_the_regression_guard() {
        let store = store();
        let mut batch = store.begin();
        batch.push(crate::transport::SqlStatement::new("SELECT 1"));
        let request = store.commit_request(
            batch,
            &projection_id(),
            SequencePosition::new(4).unwrap(),
            Authority::Rebuilding,
        );
        assert_eq!(request.statements.len(), 3);
        assert!(request.statements[1].query.contains("ON CONFLICT"));
        assert!(request.statements[2].query.contains("hs-regression:"));
        assert_eq!(
            request.statements[1].params[2],
            serde_json::json!("rebuilding")
        );
        for statement in &request.statements[1..] {
            assert!(
                statement
                    .query
                    .contains(r#""hsp_1"."projection_checkpoint""#)
            );
        }
    }

    /// The body the guard's failing cast actually produces, recorded from the
    /// live endpoint.
    const REGRESSION: &[u8] = br#"{"message":"invalid input syntax for type bigint: \"hs-regression:5:3\"","code":"22P02","detail":null,"hint":null,"severity":"ERROR"}"#;

    /// Someone else's `22P02`, which must not be read as a regression.
    const OTHER_CAST_FAILURE: &[u8] = br#"{"message":"invalid input syntax for type bigint: \"banana\"","code":"22P02","detail":null,"hint":null,"severity":"ERROR"}"#;

    #[test]
    fn a_regression_reconstructs_both_positions_from_one_round_trip() {
        let attempted = SequencePosition::new(3).unwrap();
        let error = decode_commit::<std::io::Error>(
            &HttpResponse::new(400, REGRESSION),
            64 * 1024,
            attempted,
        )
        .expect_err("a refused checkpoint is not a success");
        match error {
            CommitError::CheckpointRegression { current, attempted } => {
                assert_eq!(current.get(), 5);
                assert_eq!(attempted.get(), 3);
            }
            other => panic!("expected a regression, got {other:?}"),
        }
    }

    /// The marker is what keeps a caller's own bad cast out of PS-16's channel.
    #[test]
    fn another_cast_failure_stays_a_store_error() {
        let error = decode_commit::<std::io::Error>(
            &HttpResponse::new(400, OTHER_CAST_FAILURE),
            64 * 1024,
            SequencePosition::new(3).unwrap(),
        )
        .expect_err("a failed statement is not a success");
        assert!(
            matches!(error, CommitError::Store(NeonError::Sql(_))),
            "a read model's own cast failure is not a checkpoint regression"
        );
    }

    const NO_CHECKPOINT: &[u8] =
        br#"{"fields":[],"rows":[],"command":"SELECT","rowCount":0,"rowAsArray":false}"#;
    const REBUILDING: &[u8] = br#"{"fields":[],"command":"SELECT","rowCount":1,"rows":[{"position":"7","authority":"rebuilding"}],"rowAsArray":false}"#;

    #[test]
    fn an_absent_row_is_never_run_and_not_an_error() {
        let checkpoint =
            decode_checkpoint::<std::io::Error>(&HttpResponse::new(200, NO_CHECKPOINT), 64 * 1024)
                .expect("an absent checkpoint is an answer");
        assert_eq!(checkpoint, Checkpoint::NeverRun);
    }

    #[test]
    fn a_stored_authority_survives_the_round_trip() {
        let checkpoint =
            decode_checkpoint::<std::io::Error>(&HttpResponse::new(200, REBUILDING), 64 * 1024)
                .expect("a recorded row decodes");
        assert_eq!(
            checkpoint,
            Checkpoint::Rebuilding {
                through: SequencePosition::new(7).unwrap()
            }
        );
    }
}
