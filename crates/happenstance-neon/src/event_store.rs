//! The Neon-backed [`EventStore`], and the buffered thing it calls a stream.
//!
//! # Why the bare flavour
//!
//! [`SendEventStore`](happenstance_core::SendEventStore) would require every
//! future this crate produces to be `Send`, and the `wasm32` transport's future
//! is a `wasm_bindgen_futures::JsFuture` wrapping a `Promise`, which is not.
//! Implementing the bare [`EventStore`] once is what lets the same source
//! compile for the host and for `wasm32-unknown-unknown`.
//!
//! Note precisely what that claims. `store.rs:17-18` states the implication in
//! one direction only:
//!
//! ```text
//!     impl SendEventStore  ==>  EventStore comes free
//!     impl EventStore      =/=> SendEventStore  (cannot conjure Send)
//! ```
//!
//! So this crate compiles *for both targets implementing the bare flavour on
//! each*. It does **not** satisfy `SendEventStore` on either target, including
//! the host, even when the transport happens to be `Send` — the impl is written
//! once, on the weaker trait, and there is no way back up.
//!
//! Both halves of that were compiled rather than reasoned about. Asserting
//! `NeonEventStore<NullTransport>: SendEventStore` gives
//! `error[E0277]: the trait bound … is not satisfied`. Trying to *fix* that by
//! adding a second impl — `impl<T: SqlTransport + Send + Sync> SendEventStore
//! for NeonEventStore<T>` alongside the bare one — gives
//! `error[E0119]: conflicting implementations of trait EventStore`, naming
//! `trait_variant`'s blanket `impl<T: SendEventStore> EventStore for T`. The two
//! flavours are therefore **mutually exclusive per type**, not a lattice an
//! adapter can sit at two points of.
//!
//! # The schema
//!
//! Authoritative in `migrations/0001_neon_log.sql`, rendered by
//! [`crate::migration`]. It is not restated here: a prose copy of a schema is a
//! copy that drifts, and this module used to carry one that had drifted into
//! saying `bigserial` with no origin columns, no `recorded_at` and no `xact_id`.
//! What is worth reading before the file: `position` carries **no column
//! default** and is not `bigserial`; the sequence is read explicitly at the one
//! `INSERT` that allocates; and each row stamps `pg_current_xact_id()` into an
//! `xid8` column, which is the mechanism every read and `head` filter on.
//!
//! # ES-10, and what this adapter pays for it
//!
//! ES-10 says position order is visibility order: once a reader has seen position
//! *P*, nothing at or below *P* may appear later. `nextval()` allocates **outside**
//! the transaction, so a naive store breaks it — a writer takes 99, a writer that
//! started later takes 100 and commits first, and a reader who conditioned on
//! `after: Some(100)` never sees the 99 that would have violated its boundary.
//!
//! This adapter buys the invariant the same way `happenstance-postgres` does, on
//! ADR-0024's prior authority, and **re-measured against this endpoint rather
//! than inherited from it** — because that ADR's staleness numbers were taken
//! against a dedicated container and this is a shared Neon branch carrying a
//! pooler and Neon's own compute processes, either of which could hold
//! `pg_snapshot_xmin` back. Ten append-then-read cycles against the live
//! endpoint: the frontier had already passed the appended position on the first
//! read, ten of ten. The round trip is 80-100 ms and the frontier's own lag is
//! sub-millisecond, so the margin is three orders of magnitude, which is a
//! different situation from the Postgres adapter's and is worth saying rather
//! than assuming.
//!
//! What follows for a caller, unsoftened: [`head`](EventStore::head) reports a
//! **frontier** rather than the highest position assigned, and
//! **read-your-own-writes is not promised**. Both are capability limits rather
//! than defects.
//!
//! # The append is two statements, and that is not a regression
//!
//! This module used to document a single `WITH probe AS (…) , ins AS (…)` CTE,
//! on the argument that one statement is one snapshot. The argument was right
//! and the conclusion was wrong, because of a fact about the *endpoint* rather
//! than about SQL: `Neon-Batch-Isolation-Level` is honoured on a request carrying
//! **two or more** statements and **ignored** on the single-statement form. Both
//! halves are measured — `current_setting('transaction_isolation')` answers
//! `serializable` in a two-statement batch carrying the header and
//! `read committed` in a one-statement request carrying the same header.
//!
//! A single CTE therefore runs at `READ COMMITTED`, where two racers both find
//! no conflict and both insert. That is the lost update this crate's
//! [`ProbeThenWriteStore`] exists to name, arriving through the door left open
//! while the other one was being closed. So the conditional append is a
//! two-statement non-interactive batch: a probe that reports the conflicting
//! position, and a guarded `INSERT … WHERE NOT EXISTS`, both on one
//! `SERIALIZABLE` snapshot in one round trip. `conflicting_position` survives,
//! which is what the CTE was for.
//!
//! Measured at 64 simultaneous contenders on one boundary against the live
//! endpoint, six runs: exactly one commit and sixty-three
//! `ConditionViolated` every time, and no contender needing more than one retry.
//!
//! That measurement is real and it is **not** what sets the retry count, which is
//! worth saying because taking it at face value is how this adapter shipped a
//! number that failed half its runs. Contenders on *disjoint* boundaries do not
//! drain the way contenders on one boundary do: the SSI predicate lock is
//! relation-wide on a small table, so a retry that finds no conflict on its own
//! boundary goes straight back into the same fight.
//! [`NeonEventStore::SERIALISATION_ATTEMPTS`] carries both numbers and the table
//! that decides between them.

use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};

use base64::Engine as _;
use base64::engine::general_purpose::STANDARD as BASE64;
use futures_core::Stream;
use happenstance_core::{
    AppendCondition, AppendError, ConditionViolated, Event, EventId, EventStore, Query,
    ReadOptions, RecordedAt, SequencePosition, SequencedEvent, StoreId, StoreLimit, Tag, Tags,
};

use crate::config::NeonConfig;
use crate::error::NeonError;
use crate::query_sql::predicate;
use crate::transport::{HttpResponse, SqlRequest, SqlStatement, SqlTransport};
use crate::wire::{ResponseBody, ResultSet};

/// The columns every read selects, and the names the decoder looks them up by.
///
/// Named once because the `SELECT` and the decoder must agree, and a lookup by
/// name against a drifting projection list is a class of bug that shows up as a
/// `None` rather than as a type error.
///
/// Every `bigint` is cast `::text` and every `bytea` is `encode(…, 'base64')`,
/// which is the whole of this adapter's answer to "the endpoint renders
/// everything as JSON": a `bigint` would otherwise arrive as a string anyway and
/// a `bytea` as `\x…` hex at 100% expansion, where base64 costs 33%.
///
/// `tags` is `to_jsonb(tags)` and **not** the bare column, which is not a
/// preference. A `text[]` selected raw is rendered by the endpoint's driver from
/// Postgres' array literal, and it gets the empty case wrong: `{}` comes back as
/// `[""]` — a one-element array holding an empty string — where the correct
/// answer is `[]`. Measured, and it is the exact failure that turned
/// `read should succeed` into `StoredTag(Empty)` on nineteen rules of the first
/// live run, because every event with no tags decoded as an event with one
/// invalid tag. `to_jsonb` makes the server produce the JSON, so `{}` is `[]`
/// and an array element is a string because Postgres said so rather than because
/// a parser guessed.
const SELECTED_COLUMNS: &str = "position::text AS position, \
                                event_type, \
                                encode(data, 'base64') AS data, \
                                encode(metadata, 'base64') AS metadata, \
                                to_jsonb(tags) AS tags, \
                                encode(origin_store, 'base64') AS origin_store, \
                                origin_position::text AS origin_position, \
                                recorded_at::text AS recorded_at";

/// The visibility frontier, as an `AND`-able fragment.
///
/// Everything beneath `pg_snapshot_xmin(pg_current_snapshot())` has settled. See
/// the module documentation for what it costs.
const FRONTIER: &str = "xact_id < pg_snapshot_xmin(pg_current_snapshot())";

/// An event store over Neon's serverless `/sql` HTTP endpoint.
///
/// ```
/// use happenstance_core::EventStore;
/// use happenstance_neon::{NeonConfig, NeonEventStore, NullTransport};
///
/// // Binding the *bare* flavour, per the contract crate's guidance.
/// fn takes_a_store<S: EventStore>(_store: &S) {}
///
/// let store = NeonEventStore::new(NullTransport::new(), NeonConfig::default());
/// takes_a_store(&store);
/// ```
#[derive(Debug, Clone)]
pub struct NeonEventStore<T> {
    transport: T,
    config: NeonConfig,
}

impl<T> NeonEventStore<T> {
    /// The largest `data` payload this store accepts, in **bytes of
    /// [`Event::data`]** — not of an encoded row, and not of `data` and
    /// `metadata` together.
    ///
    /// An adapter policy rather than a server limit, and the arithmetic behind it
    /// is this adapter's own rather than borrowed. A payload travels base64, so
    /// it costs 4/3 of its length in the request body and again in any response
    /// that reads it back; 131,072 bytes is 174,764 characters each way, against
    /// [`MAX_RESPONSE_BYTES`](crate::MAX_RESPONSE_BYTES)'s 64 MiB. Twice VT-21's
    /// 65,536-byte floor, so
    /// `store_accepts_the_guaranteed_minimum_payload` has headroom rather than
    /// sitting exactly on the line.
    ///
    /// It is deliberately **not** the 1 MiB `happenstance-sqlite` and
    /// `happenstance-postgres` agree on. Those two are wire-efficient binary
    /// protocols with no per-request body to buffer; this one buffers the whole
    /// request and the whole response in memory as JSON at both ends, and a
    /// ceiling that ignored that would be a number copied rather than derived.
    pub const MAX_EVENT_DATA_LEN: usize = 131_072;

    /// The largest number of tags on one event this store accepts.
    ///
    /// Twice VT-22's floor of 64. Tags live in one `text[]` column, so a tag
    /// costs no extra row and no extra statement; the cost is the GIN index's,
    /// which builds one entry per element.
    pub const MAX_TAGS_PER_EVENT: usize = 128;

    /// The largest number of events this store accepts in one append.
    ///
    /// VT-24's floor exactly, and the one of the three where this adapter is
    /// genuinely tighter than its siblings — `happenstance-postgres` states 256.
    /// The batch is one JSON document held whole in memory at both ends and the
    /// response holds one row per event, so the honest ceiling is low and stated
    /// rather than high and discovered as a 413.
    pub const MAX_EVENTS_PER_BATCH: usize = 128;

    /// How many times a conditional append re-runs after a serialisation failure.
    ///
    /// **Re-derived against this endpoint rather than copied from
    /// `happenstance-postgres`**, because the critique of the plan was right that
    /// a number justified by "the retry is only ever needed once in the
    /// two-writer case" says nothing about a transport with a thousand times the
    /// transaction latency and 64 contenders.
    ///
    /// **The one-boundary measurement says three, and it is the wrong
    /// measurement.** 64 simultaneous conditional appends on *one* boundary
    /// against the live endpoint, six runs: exactly one commit and sixty-three
    /// `ConditionViolated` every time, between 0 and 25 contenders seeing a
    /// `40001` at all, and no contender ever reaching a third attempt. The reason
    /// is structural — the moment the winner commits, every retry *sees* the
    /// winner's row and answers `ConditionViolated` instead of racing again, so
    /// the population drains in one round.
    ///
    /// **Disjoint boundaries do not drain, and that is what sets this number.**
    /// `k_disjoint_boundaries_admit_exactly_k_commits` runs twelve contenders
    /// over four separate boundaries, and the SSI predicate lock is *not* per
    /// boundary: with a small table the planner takes a sequential scan and the
    /// lock is relation-wide, so every contender conflicts with every other
    /// regardless of which boundary it is racing. A retry that finds no conflict
    /// on its own boundary goes back into the same fight, and the population
    /// drains one round at a time rather than all at once.
    ///
    /// Measured against that rule, four runs per value:
    ///
    /// | attempts | result |
    /// |---|---|
    /// | 3 | **failed 2 of 4** — a contender exhausted and answered `40001` |
    /// | 4 | 4 of 4 green |
    /// | 5 | 4 of 4 green |
    /// | 8 | 5 of 5 green |
    ///
    /// Eight is double the smallest sufficient value rather than one above it,
    /// because the observed floor sits one attempt away from a failure that
    /// reproduced half the time — a margin of one on a measurement that noisy is
    /// not a margin. It costs nothing when it is not needed: a retry happens only
    /// after a `40001`, and the retry itself is the backoff, at one round trip
    /// each.
    ///
    /// Exhaustion is not silent: it surfaces as
    /// `AppendError::Store(NeonError::Sql(…))` carrying SQLSTATE `40001`, which is
    /// a named, documented outcome rather than a `ConditionViolated` this adapter
    /// invented — and it is exactly what the failures at three looked like.
    pub const SERIALISATION_ATTEMPTS: u32 = 8;

    /// Builds a store over `transport`.
    pub const fn new(transport: T, config: NeonConfig) -> Self {
        Self { transport, config }
    }

    /// The transport this store sends round trips through.
    pub const fn transport(&self) -> &T {
        &self.transport
    }

    /// The table names and limits this store was built with.
    pub const fn config(&self) -> &NeonConfig {
        &self.config
    }

    /// Refuses a batch that exceeds one of this store's stated ceilings.
    ///
    /// Checked **before** anything reaches the wire, so an over-limit batch costs
    /// no round trip and is reported as [`AppendError::ExceedsStoreLimit`] rather
    /// than as an HTTP failure. A sync runner has to tell "this will never fit
    /// here, park it" from "the endpoint 413'd, retry", and flattening the two
    /// destroys the distinction. Truncation is forbidden outright.
    fn check_ceilings<E>(events: &[Event]) -> Result<(), AppendError<NeonError<E>>> {
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
}

impl<T: SqlTransport> NeonEventStore<T> {
    /// The single `SELECT` a read compiles to.
    ///
    /// One statement, because a read has nothing to be atomic with — and one
    /// round trip, because this endpoint has no cursor to fetch a second page
    /// from. ES-11's "fix the state no later than the first poll" is bought here
    /// by there being exactly one statement: the whole answer is one snapshot by
    /// construction, and there is no later statement that could be bounded
    /// against a ceiling.
    fn read_request(&self, query: &Query, options: ReadOptions) -> SqlRequest {
        let mut next = 1;
        let predicate = predicate(query, &mut next);

        let mut clauses = vec![predicate.sql().to_owned(), FRONTIER.to_owned()];

        // `from` and `to` are both **inclusive**, in both directions, and both
        // are range predicates over positions rather than index seeks: the
        // specification permits gaps, this adapter produces them, so a position
        // the caller names may not exist.
        //
        // Their roles do not swap with direction; their *position-order*
        // comparisons do. Copying the forward branch's `position <= to` into the
        // backward one is the bug ES-8's `Rejects:` describes, and it is correct
        // reading forwards — so only `read_to_under_backwards_bounds_the_older_end`
        // sees it.
        let (start_op, stop_op) = if options.backwards {
            ("<=", ">=")
        } else {
            (">=", "<=")
        };
        // Interpolated rather than bound, and the exception to this crate's
        // "every value is bound" rule is deliberate and narrow: these are `i64`s
        // formatted by Rust, so there is no string for an injection to live in,
        // and `happenstance-postgres/src/read_stream.rs` makes the same call at
        // the same three sites. A caller-supplied *string* never reaches SQL
        // this way.
        if let Some(from) = options.from {
            clauses.push(format!("position {start_op} {}", as_i64(from)));
        }
        if let Some(to) = options.to {
            clauses.push(format!("position {stop_op} {}", as_i64(to)));
        }

        let order = if options.backwards { "DESC" } else { "ASC" };
        // `Some(0)` must emit `LIMIT 0` and read nothing, which is why this is
        // `map_or_else` over the option rather than a truthiness check.
        let limit = options
            .limit
            .map_or_else(String::new, |limit| format!(" LIMIT {limit}"));

        // `ORDER BY {table}.position`, **qualified**, and the qualification is
        // load-bearing rather than tidy. Postgres resolves a bare name in
        // `ORDER BY` against the SELECT's OUTPUT columns first, and this SELECT
        // aliases `position::text AS position` — so `ORDER BY position` sorts the
        // *text*: 1, 10, 100, 101, …, 109, 11, 110. `WHERE` has no such rule and
        // was always right, which is why the bug survived every rule with fewer
        // than ten events and appeared at 128 as
        // `store_accepts_the_guaranteed_minimum_batch_size` returning the batch in
        // lexicographic order, and in the model family as
        // "positions must be strictly increasing, but 2 follows 10".
        let event_table = self.config.qualified_event();
        let sql = format!(
            "SELECT {SELECTED_COLUMNS} FROM {event_table} WHERE {} \
             ORDER BY {event_table}.position {order}{limit}",
            clauses.join(" AND ")
        );

        // Marked read-only, which the endpoint honours only on the batch form and
        // which this is not. Stated anyway: it is true, it costs nothing, and
        // `SqlRequest::headers` is the one place that decides whether to send it.
        SqlRequest::single(SqlStatement::with_params(sql, predicate.into_params())).read_only()
    }

    /// The `SELECT` that reports the highest conflicting position, if any.
    ///
    /// # No frontier predicate here, deliberately
    ///
    /// `head` and `read` carry `xact_id < pg_snapshot_xmin(…)`; this does not.
    /// The frontier exists so a *reader* never sees a position appear beneath one
    /// it has already observed. A condition is not a read: it asks whether the
    /// store holds a conflicting event at all, and a committed row above the
    /// frontier is held. Filtering it out would admit an append the boundary
    /// forbids — the same silent corruption the mechanism exists to prevent,
    /// arriving through the door left open while the other one was being closed.
    fn probe_statement(&self, condition: &AppendCondition, next: &mut usize) -> SqlStatement {
        let event = self.config.qualified_event();
        let mut params = Vec::new();
        let arms: Vec<String> = condition
            .guards()
            .iter()
            .map(|guard| {
                let built = predicate(&guard.query, next);
                let sql = format!(
                    "SELECT max(position) AS c FROM {event} WHERE {} AND position > {}",
                    built.sql(),
                    // `None` is position 0, not a literal NULL. `position > NULL`
                    // is NULL for every row, so the guard would match nothing and
                    // the condition would admit every append — a rejection that
                    // looks like an acceptance.
                    guard.after.map_or(0, as_i64)
                );
                params.extend(built.into_params());
                sql
            })
            .collect();

        // `max` across the guards, computed server-side rather than by n round
        // trips: any guard violated violates the condition, and the position
        // reported is the highest any of them found.
        let sql = format!(
            "SELECT max(c)::text AS conflict FROM ({}) g",
            arms.join(" UNION ALL ")
        );
        SqlStatement::with_params(sql, params)
    }

    /// The `EXISTS` probe the two-statement append sends first.
    ///
    /// Used only by [`ProbeThenWriteStore`], which exists to be wrong: on its own
    /// this statement is one round trip in its own implicit transaction, with
    /// nothing connecting it to the insert that follows.
    fn probe_request(&self, condition: &AppendCondition) -> SqlRequest {
        let mut next = 1;
        SqlRequest::single(self.probe_statement(condition, &mut next)).read_only()
    }

    /// The `INSERT … RETURNING position`, guarded or not.
    ///
    /// # Why the positions are paired with the batch by `row_number`
    ///
    /// `nextval` is read explicitly — `position` carries no column default — and
    /// the values come back in one set-returning subquery. Pairing them with the
    /// caller's slice by `row_number() OVER (ORDER BY p)` against
    /// `WITH ORDINALITY` is what makes `batch_positions_follow_slice_order` and
    /// `positions_are_strictly_monotonic` hold **by construction** rather than by
    /// whatever order the planner happened to emit rows in.
    ///
    /// `next` is the placeholder index the guard predicate starts from, and it is
    /// 2 rather than 1: `$1` is the whole batch as one `jsonb` document. Getting
    /// that wrong binds the events where an event type was expected, with no type
    /// error anywhere.
    fn insert_statement(&self, events: &[Event], guard: Option<&AppendCondition>) -> SqlStatement {
        let event_table = self.config.qualified_event();
        let mut params = vec![serde_json::Value::String(encode_batch(events))];

        let mut next = 2;
        let filter = guard.map_or_else(String::new, |condition| {
            let arms: Vec<String> = condition
                .guards()
                .iter()
                .map(|guard| {
                    let built = predicate(&guard.query, &mut next);
                    let sql = format!(
                        "({} AND position > {})",
                        built.sql(),
                        guard.after.map_or(0, as_i64)
                    );
                    params.extend(built.into_params());
                    sql
                })
                .collect();
            format!(
                " WHERE NOT EXISTS (SELECT 1 FROM {event_table} WHERE {})",
                arms.join(" OR ")
            )
        });

        let sql = format!(
            "INSERT INTO {event_table} \
             (position, event_type, data, metadata, tags, origin_store, origin_position, recorded_at) \
             SELECT a.position, b.event_type, decode(b.data, 'base64'), \
             decode(b.metadata, 'base64'), b.tags, \
             (SELECT v FROM {meta} WHERE k = 'store_id'), a.position, \
             (extract(epoch FROM statement_timestamp()) * 1000)::bigint \
             FROM (SELECT ord, e->>'t' AS event_type, e->>'d' AS data, e->>'m' AS metadata, \
             ARRAY(SELECT jsonb_array_elements_text(e->'g')) AS tags \
             FROM jsonb_array_elements($1::jsonb) WITH ORDINALITY AS t(e, ord)) b \
             JOIN (SELECT row_number() OVER (ORDER BY p) AS ord, p AS position \
             FROM (SELECT nextval({sequence}) AS p \
             FROM generate_series(1, jsonb_array_length($1::jsonb))) s) a \
             USING (ord){filter} \
             RETURNING position::text AS position",
            meta = self.config.qualified_meta(),
            sequence = self.config.sequence_literal(),
        );
        SqlStatement::with_params(sql, params)
    }

    /// The unconditional `INSERT … RETURNING position`.
    ///
    /// One statement at `ReadCommitted`, and the split from the conditional form
    /// is the same one `happenstance-postgres` makes: an unconditional append
    /// asserts nothing about the log's state, so it cannot conflict with anything
    /// and pays nothing for an isolation level it has no use for. The endpoint
    /// would ignore the header on a single statement in any case.
    fn insert_request(&self, events: &[Event]) -> SqlRequest {
        SqlRequest::single(self.insert_statement(events, None))
    }

    /// The whole conditional append as **one round trip of two statements**.
    ///
    /// See the module documentation for why this is not the single CTE this file
    /// used to describe. In short: the endpoint ignores
    /// `Neon-Batch-Isolation-Level` below two statements, so the CTE would run at
    /// `READ COMMITTED` and two racers would both probe empty and both insert.
    ///
    /// The probe is not decoration and is not a `SELECT 1` making up the numbers.
    /// It is what carries `ConditionViolated::conflicting_position`: the guarded
    /// `INSERT` reports a refusal as zero rows, and zero rows carry no position.
    /// Both statements run on one `SERIALIZABLE` snapshot, so the position the
    /// probe reports is the position the insert refused against.
    fn conditional_append_request(
        &self,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> SqlRequest {
        let Some(condition) = condition else {
            return self.insert_request(events);
        };
        let mut next = 1;
        let probe = self.probe_statement(condition, &mut next);
        SqlRequest::batch(
            vec![probe, self.insert_statement(events, Some(condition))],
            self.config.isolation,
        )
    }

    /// One attempt at a conditional append.
    async fn append_once(
        &self,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<AppendOutcome, NeonError<T::Error>> {
        let request = self.conditional_append_request(events, condition);
        let response = self
            .transport
            .round_trip(request)
            .await
            .map_err(NeonError::Transport)?;
        decode_append_response::<T::Error>(&response, self.config.max_response_bytes)
    }
}

impl<T: SqlTransport> EventStore for NeonEventStore<T> {
    type Error = NeonError<T::Error>;

    fn read(
        &self,
        query: &Query,
        options: ReadOptions,
    ) -> impl Stream<Item = Result<SequencedEvent, Self::Error>> {
        NeonReadStream::unsent(
            &self.transport,
            self.read_request(query, options),
            self.config.max_response_bytes,
        )
    }

    /// Appends a batch and returns the position of its last event.
    ///
    /// # What `Ok(P)` does and does not promise
    ///
    /// It promises that the batch is committed, at positions ending at *P*, and
    /// that the append condition held when it was evaluated.
    ///
    /// It does **not** promise that the next [`head`](EventStore::head) is at or
    /// above *P*. This store reports a visibility frontier, the frontier trails
    /// the positions already assigned, and **read-your-own-writes is not
    /// promised**. Measured against this endpoint the margin is large — the
    /// frontier's lag is sub-millisecond against a round trip of 80-100 ms — but
    /// it is a margin rather than a guarantee, and a caller who needs the
    /// position should keep the one this method returned.
    ///
    /// # Errors
    ///
    /// [`AppendError::NoEvents`] for an empty batch, refused before the condition
    /// is evaluated; [`AppendError::ExceedsStoreLimit`] for a batch over one of
    /// this store's stated ceilings, refused before anything reaches the wire;
    /// [`AppendError::ConditionViolated`] when the condition found a matching
    /// event, which is not an adapter failure; and [`AppendError::Store`] for
    /// anything the transport or the endpoint reports — including a `40001` that
    /// survived [`SERIALISATION_ATTEMPTS`](NeonEventStore::SERIALISATION_ATTEMPTS)
    /// attempts, which is a named outcome rather than a violation this adapter
    /// invented.
    async fn append(
        &self,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<Self::Error>> {
        // ES-18 is explicit that this precedes condition evaluation, so a
        // zero-event call is never an expensive no-op.
        if events.is_empty() {
            return Err(AppendError::NoEvents);
        }
        Self::check_ceilings(events)?;

        let mut attempt = 0;
        let outcome = loop {
            attempt += 1;
            match self.append_once(events, condition).await {
                Err(NeonError::Sql(sql))
                    if sql.is_serialization_failure() && attempt < Self::SERIALISATION_ATTEMPTS =>
                {
                    // Nothing was committed — the endpoint aborted the whole
                    // batch — so re-running is not a partial retry. The next
                    // attempt reads a log that now contains the winner's rows,
                    // and answers `ConditionViolated` rather than racing again.
                }
                Err(error) => return Err(AppendError::Store(error)),
                Ok(outcome) => break outcome,
            }
        };

        match outcome {
            AppendOutcome::Appended(position) => Ok(position),
            AppendOutcome::Conflict(position) => Err(AppendError::ConditionViolated(
                ConditionViolated::at(position),
            )),
        }
    }

    /// The highest position beneath this store's **visibility frontier**, or
    /// `None` when nothing is visible yet.
    ///
    /// # What a frontier is, and why it is not `max(position)`
    ///
    /// A head is a promise that nothing at or below it will appear later
    /// (ES-10). `max(position)` cannot make that promise: `nextval()` allocates
    /// outside the transaction, so the highest position assigned can name a row
    /// whose predecessors are still in flight. Each row therefore stamps its
    /// appending transaction's `xid8`, and this reports the frontier beneath
    /// which nothing can still be running.
    ///
    /// The frontier legitimately **trails** the position
    /// [`append`](EventStore::append) just returned; ES-30's
    /// `head_is_the_highest_visible_position` asserts a *bound* rather than an
    /// equality precisely to admit that.
    ///
    /// # Errors
    ///
    /// The adapter's error if the round trip fails, or if a stored position
    /// cannot be decoded — which is a corrupt store and is deliberately not
    /// spelled the same way as an empty one.
    async fn head(&self) -> Result<Option<SequencePosition>, Self::Error> {
        let sql = format!(
            "SELECT max(position)::text AS head FROM {} WHERE {FRONTIER}",
            self.config.qualified_event()
        );
        let response = self
            .transport
            .round_trip(SqlRequest::single(SqlStatement::new(sql)).read_only())
            .await
            .map_err(NeonError::Transport)?;
        let body = decode_body::<T::Error>(&response, self.config.max_response_bytes)?;
        let result = first_result_set::<T::Error>(&body)?;

        // `transpose`, not `and_then`: a stored value that fails to decode is a
        // corrupt store and must not be spelled the same way as an empty one.
        result.text(0, "head").map(position_from_text).transpose()
    }

    /// Whether this store holds the event `id` names.
    ///
    /// # The frontier disagreement, answered rather than settled
    ///
    /// This asks whether the store *holds* an event, and under the frontier
    /// mechanism that is a different question from whether `read` would yield it:
    /// a committed row above the frontier is held but invisible. **This method
    /// answers `true` for a held-but-invisible row** — it does not carry the
    /// frontier predicate — for the reason `happenstance-postgres` gives at the
    /// same method. Answering `false` would let a replication ingest re-accept an
    /// event the store already holds, and duplicate history is not recoverable by
    /// retrying; answering `true` makes this briefly disagree with `read`, and
    /// that disagreement resolves itself as the frontier advances.
    ///
    /// Recorded, not settled. ES-41 stays `[PROVISIONAL]`.
    ///
    /// # Errors
    ///
    /// The adapter's error if the round trip fails or the answer does not have
    /// the shape this adapter asked for.
    async fn contains_event_id(&self, id: EventId) -> Result<bool, Self::Error> {
        let sql = format!(
            "SELECT (EXISTS (SELECT 1 FROM {} WHERE origin_store = decode($1, 'base64') \
             AND origin_position = $2::bigint))::text AS found",
            self.config.qualified_event()
        );
        let params = vec![
            serde_json::Value::String(BASE64.encode(id.store().to_bytes())),
            serde_json::Value::String(as_i64(id.position()).to_string()),
        ];
        let response = self
            .transport
            .round_trip(SqlRequest::single(SqlStatement::with_params(sql, params)).read_only())
            .await
            .map_err(NeonError::Transport)?;
        let body = decode_body::<T::Error>(&response, self.config.max_response_bytes)?;
        let result = first_result_set::<T::Error>(&body)?;
        Ok(result.text(0, "found") == Some("true"))
    }
}

/// What the two-statement append's answer says.
#[derive(Debug)]
enum AppendOutcome {
    /// The insert happened; this is the last position it assigned.
    Appended(SequencePosition),
    /// The insert did not happen, and this is the conflicting position the probe
    /// found on the same snapshot.
    Conflict(SequencePosition),
}

/// Encodes the whole batch as the one `jsonb` document the `INSERT` unpacks.
///
/// The keys are one character each, and that is arithmetic rather than golf: the
/// batch ceiling is 128 events, so `"event_type"`/`"metadata"`/`"tags"` against
/// `t`/`m`/`g` is around three kilobytes of key names per request that carry no
/// information. The `INSERT` names them in exactly one place.
fn encode_batch(events: &[Event]) -> String {
    let array: Vec<serde_json::Value> = events
        .iter()
        .map(|event| {
            serde_json::json!({
                "t": event.event_type().as_str(),
                "d": BASE64.encode(event.data()),
                "m": event.metadata().map(|bytes| BASE64.encode(bytes)),
                "g": event.tags().iter().map(Tag::as_str).collect::<Vec<_>>(),
            })
        })
        .collect();
    serde_json::Value::Array(array).to_string()
}

/// Reads a response, in the order the failures actually happen.
///
/// Size first, because a body over the ceiling is refused without being parsed;
/// then status, because the endpoint puts SQL errors in a 4xx body and they must
/// arrive structurally rather than as a string; then the shape.
fn decode_body<E>(
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
        return Err(sql_error(response));
    }
    ResponseBody::parse(&response.body).map_err(NeonError::MalformedResponse)
}

/// A non-2xx body as the richest error it can be read as.
///
/// A failing statement fails the **whole** batch and answers with a single
/// top-level error object — not a partial `results` array — so there is exactly
/// one shape to try here. When it does not parse, the status and a truncated body
/// are all there is, and saying so is better than inventing a SQLSTATE.
fn sql_error<E>(response: &HttpResponse) -> NeonError<E> {
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
    )
}

/// The first result set, or the shape complaint.
fn first_result_set<E>(body: &ResponseBody) -> Result<&ResultSet, NeonError<E>> {
    body.result_sets()
        .first()
        .ok_or(NeonError::MissingColumn { column: "results" })
}

/// Turns the endpoint's answer into an outcome.
///
/// Branches on how many result sets came back rather than on a flag threaded
/// down from `append`, because that is the thing the endpoint actually
/// distinguishes: a conditional append is two statements and an unconditional one
/// is a single statement, and the two answer with the batch and the single form
/// respectively.
fn decode_append_response<E>(
    response: &HttpResponse,
    max_response_bytes: usize,
) -> Result<AppendOutcome, NeonError<E>> {
    let body = decode_body::<E>(response, max_response_bytes)?;
    let sets = body.result_sets();

    let (conflict, inserted) = match sets {
        [inserted] => (None, inserted),
        [probe, inserted] => (probe.text(0, "conflict"), inserted),
        _ => return Err(NeonError::MissingColumn { column: "results" }),
    };

    if let Some(conflict) = conflict {
        return Ok(AppendOutcome::Conflict(position_from_text(conflict)?));
    }

    // `max` rather than the last row: `RETURNING` promises no order, and the
    // positions are paired with the caller's slice by `row_number` inside the
    // statement, so the highest is the last event's.
    let last = inserted
        .rows
        .iter()
        .filter_map(|row| row.get("position")?.as_str())
        .map(position_from_text::<E>)
        .try_fold(
            None::<SequencePosition>,
            |highest, position| -> Result<_, NeonError<E>> {
                let position = position?;
                Ok(Some(highest.map_or(position, |held| held.max(position))))
            },
        )?;

    // No conflict and no rows is neither outcome, and it is the answer a guard
    // that silently matched nothing would produce. Named rather than folded into
    // one of the two, because a caller told `ConditionViolated` here would retry
    // forever.
    last.map(AppendOutcome::Appended)
        .ok_or(NeonError::MissingColumn { column: "position" })
}

/// Turns the endpoint's answer into the whole result set, at once.
fn decode_read_response<E>(
    response: &HttpResponse,
    max_response_bytes: usize,
) -> Result<Vec<SequencedEvent>, NeonError<E>> {
    let body = decode_body::<E>(response, max_response_bytes)?;
    first_result_set::<E>(&body)?
        .rows
        .iter()
        .map(decode_row)
        .collect()
}

/// Turns one row into a contract event.
///
/// Every failure here is a **stored** value that no longer satisfies a contract
/// type — a `bigint` position that is zero or negative against a `NonZeroU64`, a
/// `text` event type or tag carrying something validation now rejects. Each gets
/// a named variant rather than a panic, because the alternative to a variant is a
/// panic in a library.
fn decode_row<E>(row: &serde_json::Value) -> Result<SequencedEvent, NeonError<E>> {
    let text = |column: &'static str| -> Result<&str, NeonError<E>> {
        row.get(column)
            .and_then(serde_json::Value::as_str)
            .ok_or(NeonError::MissingColumn { column })
    };
    let optional =
        |column: &str| -> Option<&str> { row.get(column).and_then(serde_json::Value::as_str) };

    let position = position_from_text(text("position")?)?;
    let data = decode_base64(text("data")?)?;

    let mut event = Event::new(text("event_type")?, data).map_err(NeonError::StoredEventType)?;

    let stored_tags = row
        .get("tags")
        .and_then(serde_json::Value::as_array)
        .ok_or(NeonError::MissingColumn { column: "tags" })?;
    if !stored_tags.is_empty() {
        let tags = stored_tags
            .iter()
            .map(|tag| Tag::new(tag.as_str().unwrap_or_default()).map_err(NeonError::StoredTag))
            .collect::<Result<Tags, _>>()?;
        event = event.with_tags(tags);
    }

    // `None` and `Some(empty)` are different values and a conformance rule says
    // so, which is why this is a `map` over the option rather than an
    // `unwrap_or_default`.
    if let Some(metadata) = optional("metadata") {
        event = event.with_metadata(decode_base64(metadata)?);
    }

    // The identity columns are nullable, because a replication ingest may hold
    // rows minted elsewhere. A row this store wrote always has both.
    let id = match (optional("origin_store"), optional("origin_position")) {
        (Some(store), Some(origin)) => {
            let decoded = decode_base64(store)?;
            let bytes: [u8; 16] = decoded
                .as_slice()
                .try_into()
                .map_err(|_| NeonError::MalformedIdentity { len: decoded.len() })?;
            EventId::new(StoreId::from_bytes(bytes), position_from_text(origin)?)
        }
        _ => {
            return Err(NeonError::UnstampedEvent {
                position: position.get(),
            });
        }
    };

    let recorded_at =
        RecordedAt::from_millis(text("recorded_at")?.parse::<i64>().map_err(|_| {
            NeonError::MissingColumn {
                column: "recorded_at",
            }
        })?);
    Ok(SequencedEvent::new(position, id, recorded_at, event))
}

/// A `bytea` column, which every `SELECT` here asks for as base64.
fn decode_base64<E>(value: &str) -> Result<Vec<u8>, NeonError<E>> {
    // Postgres' `encode(…, 'base64')` wraps at 76 characters, so the newlines are
    // expected rather than a corruption. The permissive engine is the one that
    // ignores them.
    BASE64
        .decode(value.replace(['\n', '\r'], ""))
        .map_err(|_| NeonError::MalformedPayload)
}

/// Decodes a stored `bigint` — which arrives as a JSON string — into a
/// [`SequencePosition`].
fn position_from_text<E>(stored: &str) -> Result<SequencePosition, NeonError<E>> {
    let value: i64 = stored.parse().map_err(|_| NeonError::InvalidPosition {
        // A value that is not an integer at all is reported as the one thing
        // `InvalidPosition` can carry. It is unreachable through a `::text` cast
        // of a `bigint` column and is not worth a variant of its own.
        value: 0,
    })?;
    u64::try_from(value)
        .ok()
        .and_then(SequencePosition::new)
        .ok_or(NeonError::InvalidPosition { value })
}

/// A position as the `bigint` the schema stores.
fn as_i64(position: SequencePosition) -> i64 {
    i64::try_from(position.get()).unwrap_or(i64::MAX)
}

/// The read "stream": one round trip, then a buffer being drained.
///
/// # This is the honest part of the crate
///
/// [`EventStore::read`]'s doc comment says the stream is what "lets an adapter
/// stream a million-event replay without buffering it". This adapter cannot do
/// that and no version of it ever will, because Neon's `/sql` endpoint has no
/// cursor: the response is one JSON document, capped at
/// [`MAX_RESPONSE_BYTES`](crate::transport::MAX_RESPONSE_BYTES), and it either
/// arrives whole or fails with [`NeonError::ResponseTooLarge`]. The `Stream` impl
/// below is therefore a *shape*, not a capability — it satisfies the port's
/// signature exactly and honours roughly none of the port's prose.
///
/// The one property it does honour is **laziness**. `read` is not `async`, so
/// nothing is sent when it is called; the request sits unsent in the state
/// machine below until the first `poll_next`. That is load-bearing on `wasm32`,
/// where issuing a `fetch` outside a polled future is not merely wasteful but
/// happens off the event loop the runtime owns.
///
/// # What this adapter cannot promise about ES-11
///
/// Half of ES-11 is free here and the other half is not, and the difference is
/// worth stating where a caller meets it. **One read is one statement**, so there
/// is no second sample for the answer to drift against: the paging defect ES-11
/// is mostly about — a store that re-queries per page and grows under the
/// caller's feet — is unreachable by construction.
///
/// What is *not* promised is that the snapshot the endpoint takes precedes an
/// `append` this caller issues immediately afterwards. The request is dispatched
/// at the first poll, which is the earliest the port permits, and then it is one
/// of two independent requests to a proxy that hands each to whichever backend it
/// likes. There is no session, no queue and no protocol ordering between them.
///
/// This is measured rather than hedged: over the conformance transport,
/// `read_result_is_stable_under_concurrent_append` fails 3 runs in 20 over
/// HTTP/1.1 and 1 in 40 over a single HTTP/2 connection — always in the same
/// direction, with the read seeing an event appended after it was issued. A
/// caller that needs the two ordered must sequence them itself; this store has
/// nothing it could do about it, and the transport's own documentation carries
/// the numbers.
///
/// ADR-0061 settled the clause question those numbers raised. ES-11's sufficiency
/// condition for an asynchronous driver — *"a read spawned at its first poll and
/// an append spawned afterwards land in the same queue in that order"* — was a
/// fact about pooled drivers stated as one about async drivers generally, and it
/// is corrected to require an ordering primitive the *store* honours rather than
/// only the client. **This store does not satisfy ES-11**, and that is a stated
/// limitation rather than a defect awaiting a fix.
pub struct NeonReadStream<'a, T: SqlTransport> {
    state: ReadState<'a, T>,
    max_response_bytes: usize,
}

/// Where in its single round trip a [`NeonReadStream`] is.
enum ReadState<'a, T: SqlTransport> {
    /// Nothing has been sent yet.
    Unsent {
        /// The transport to send through.
        transport: &'a T,
        /// The request to send, exactly once.
        request: SqlRequest,
    },
    /// The one and only round trip is in flight.
    ///
    /// Boxed because the future is `<T as SqlTransport>::round_trip`'s opaque
    /// return type, which has no name and so cannot be a struct field.
    /// `dyn Future + 'a` with **no** `Send` — that is the bound a `JsFuture` can
    /// meet.
    InFlight(Pin<Box<dyn Future<Output = Result<HttpResponse, T::Error>> + 'a>>),
    /// The whole result set, in memory, being handed out one event at a time.
    Draining(std::vec::IntoIter<SequencedEvent>),
    /// Terminal.
    Done,
}

impl<'a, T: SqlTransport> NeonReadStream<'a, T> {
    /// A stream that has not sent its request yet.
    ///
    /// `max_response_bytes` is a parameter rather than the
    /// [`MAX_RESPONSE_BYTES`](crate::transport::MAX_RESPONSE_BYTES) constant, and
    /// the difference is the difference between a builder that works and one that
    /// compiles.
    /// [`NeonConfig::with_max_response_bytes`](crate::NeonConfig::with_max_response_bytes)
    /// lowers the ceiling; the append path already honoured it and this one
    /// hard-coded the constant, so a caller could set the field, watch the setter
    /// clamp it correctly, and have reads ignore it silently.
    fn unsent(transport: &'a T, request: SqlRequest, max_response_bytes: usize) -> Self {
        Self {
            state: ReadState::Unsent { transport, request },
            max_response_bytes,
        }
    }
}

impl<T: SqlTransport> core::fmt::Debug for NeonReadStream<'_, T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let state = match self.state {
            ReadState::Unsent { .. } => "unsent",
            ReadState::InFlight(_) => "in flight",
            ReadState::Draining(_) => "draining",
            ReadState::Done => "done",
        };
        f.debug_struct("NeonReadStream")
            .field("state", &state)
            .field("max_response_bytes", &self.max_response_bytes)
            .finish()
    }
}

impl<T: SqlTransport> Stream for NeonReadStream<'_, T> {
    type Item = Result<SequencedEvent, NeonError<T::Error>>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        // Every field is `Unpin` — a `Pin<Box<_>>`, a shared reference and a
        // `vec::IntoIter` — so the pin projection is the trivial one and the
        // state machine below can move states around freely.
        let this = self.get_mut();
        let max_response_bytes = this.max_response_bytes;

        loop {
            match core::mem::replace(&mut this.state, ReadState::Done) {
                ReadState::Unsent { transport, request } => {
                    this.state = ReadState::InFlight(Box::pin(transport.round_trip(request)));
                }
                ReadState::InFlight(mut in_flight) => match in_flight.as_mut().poll(cx) {
                    Poll::Pending => {
                        this.state = ReadState::InFlight(in_flight);
                        return Poll::Pending;
                    }
                    Poll::Ready(Err(err)) => {
                        return Poll::Ready(Some(Err(NeonError::Transport(err))));
                    }
                    Poll::Ready(Ok(response)) => {
                        match decode_read_response::<T::Error>(&response, max_response_bytes) {
                            Ok(events) => this.state = ReadState::Draining(events.into_iter()),
                            Err(err) => return Poll::Ready(Some(Err(err))),
                        }
                    }
                },
                ReadState::Draining(mut rows) => match rows.next() {
                    Some(event) => {
                        this.state = ReadState::Draining(rows);
                        return Poll::Ready(Some(Ok(event)));
                    }
                    None => return Poll::Ready(None),
                },
                ReadState::Done => return Poll::Ready(None),
            }
        }
    }
}

/// The named attempt: an `append` that **probes and writes in two round trips**.
///
/// # It compiles, and that is the finding
///
/// Nothing in [`EventStore::append`]'s signature forbids two round trips, so this
/// type-checks exactly as readily as [`NeonEventStore`] does. A table of adapter
/// shapes that recorded only compiler errors would therefore show this crate as
/// fully compatible with the port — and it is the least capable adapter in the
/// workspace.
///
/// # Why it is wrong here
///
/// Postgres over a real connection can wrap the probe and the write in
/// `BEGIN ISOLATION LEVEL SERIALIZABLE … COMMIT`, so the two statements share a
/// snapshot and the probe's answer is still true when the insert lands. Neon's
/// `/sql` endpoint has **no interactive transaction**: each round trip is its own
/// implicit transaction, and there is no handle to enrol the second one in the
/// first. So the two statements below are two independent transactions with
/// nothing between them, and the window between them is a full network round
/// trip — tens of milliseconds, not microseconds. A concurrent append that
/// commits inside that window is invisible to the probe and unopposed by the
/// insert, and the store silently accepts a write its append condition forbade.
///
/// That is a **lost update**, not a slow path: the failure is silent, produces no
/// error, and is indistinguishable after the fact from a legitimate append.
///
/// # The distinction this type is easy to misread as
///
/// [`NeonEventStore`]'s own `append` is *also* two statements. The difference is
/// not the count: it is that its two travel in **one request**, which the
/// endpoint runs inside one `BEGIN`/`COMMIT` at `SERIALIZABLE`, so they share a
/// snapshot and one of two racers is aborted with `40001`. This type's two
/// travel in two requests, and nothing joins them. "Two statements" is safe and
/// "two round trips" is not, and the whole of this crate's append design is that
/// sentence.
#[derive(Debug, Clone)]
pub struct ProbeThenWriteStore<T> {
    inner: NeonEventStore<T>,
}

impl<T> ProbeThenWriteStore<T> {
    /// Wraps a store, replacing its append with the two-round-trip one.
    pub const fn new(inner: NeonEventStore<T>) -> Self {
        Self { inner }
    }
}

impl<T: SqlTransport> EventStore for ProbeThenWriteStore<T> {
    type Error = NeonError<T::Error>;

    fn read(
        &self,
        query: &Query,
        options: ReadOptions,
    ) -> impl Stream<Item = Result<SequencedEvent, Self::Error>> {
        self.inner.read(query, options)
    }

    /// Appends in two round trips, which is the defect this type exists to name.
    ///
    /// # Errors
    ///
    /// The same set [`NeonEventStore::append`] reports — minus the one that
    /// matters, because two independent transactions never produce a `40001` to
    /// retry and therefore never elect a loser.
    async fn append(
        &self,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<Self::Error>> {
        if events.is_empty() {
            return Err(AppendError::NoEvents);
        }
        NeonEventStore::<T>::check_ceilings(events)?;

        // Round trip one: the probe. Its own implicit transaction, which has
        // committed and released its snapshot by the time this `await` resolves.
        if let Some(condition) = condition {
            let probe = self.inner.probe_request(condition);
            let response = self
                .inner
                .transport
                .round_trip(probe)
                .await
                .map_err(|err| AppendError::Store(NeonError::Transport(err)))?;

            let conflict =
                decode_probe_response::<T::Error>(&response, self.inner.config.max_response_bytes)
                    .map_err(AppendError::Store)?;
            if let Some(position) = conflict {
                return Err(AppendError::ConditionViolated(ConditionViolated::at(
                    position,
                )));
            }
        }

        // Round trip two: the write. Nothing in the type system, and nothing in
        // the protocol, connects it to round trip one.
        let insert = self.inner.insert_request(events);
        let response = self
            .inner
            .transport
            .round_trip(insert)
            .await
            .map_err(|err| AppendError::Store(NeonError::Transport(err)))?;

        match decode_append_response::<T::Error>(&response, self.inner.config.max_response_bytes) {
            Ok(AppendOutcome::Appended(position)) => Ok(position),
            Ok(AppendOutcome::Conflict(position)) => Err(AppendError::ConditionViolated(
                ConditionViolated::at(position),
            )),
            Err(err) => Err(AppendError::Store(err)),
        }
    }

    // Forwarded, like `read`: this type's declared defect is the two-round-trip
    // append and nothing else, and both of these are single-statement questions
    // that the two-round-trip shape has no way to get wrong.
    async fn head(&self) -> Result<Option<SequencePosition>, Self::Error> {
        self.inner.head().await
    }

    async fn contains_event_id(&self, id: EventId) -> Result<bool, Self::Error> {
        self.inner.contains_event_id(id).await
    }
}

/// The conflicting position the probe found, if any.
fn decode_probe_response<E>(
    response: &HttpResponse,
    max_response_bytes: usize,
) -> Result<Option<SequencePosition>, NeonError<E>> {
    let body = decode_body::<E>(response, max_response_bytes)?;
    first_result_set::<E>(&body)?
        .text(0, "conflict")
        .map(position_from_text)
        .transpose()
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::{
        AppendOutcome, NeonEventStore, ProbeThenWriteStore, decode_append_response,
        decode_read_response,
    };
    use crate::config::NeonConfig;
    use crate::error::NeonError;
    use crate::transport::{HttpResponse, NullTransport, SqlTransport};
    use happenstance_core::{
        AppendCondition, Event, EventStore, Query, QueryItem, ReadOptions, SequencePosition,
        SequencedEvent, Tags,
    };

    fn store() -> NeonEventStore<NullTransport> {
        NeonEventStore::new(
            NullTransport::new(),
            NeonConfig::default().with_schema("hs_1"),
        )
    }

    /// Both stores satisfy the **bare** flavour at a concrete transport.
    fn assert_bare_flavour<S: EventStore>() {}

    #[test]
    fn both_stores_are_bare_event_stores() {
        assert_bare_flavour::<NeonEventStore<NullTransport>>();
        assert_bare_flavour::<ProbeThenWriteStore<NullTransport>>();
    }

    #[test]
    fn a_store_can_be_built_without_a_transport() {
        let _wrapped = ProbeThenWriteStore::new(store());
    }

    /// Every statement this adapter emits names its schema.
    ///
    /// The fixture's isolation is one schema per instance and nothing else — the
    /// proxy discards `options=-c search_path=…` — so an unqualified name is an
    /// instance reading another instance's rows. It is worth a test rather than a
    /// review, because the failure is invisible until two fixtures run at once.
    #[test]
    fn no_statement_leaves_a_name_unqualified() {
        let store = store();
        let tags = Tags::from_pairs([("k", "v")]).unwrap();
        let condition =
            AppendCondition::new(Query::from_item(QueryItem::tagged(tags.clone()).unwrap()))
                .after_opt(SequencePosition::new(3));
        let event = Event::new("T", b"x".to_vec()).unwrap().with_tags(tags);

        let requests = [
            store.read_request(&Query::all(), ReadOptions::new()),
            store.probe_request(&condition),
            store.insert_request(core::slice::from_ref(&event)),
            store.conditional_append_request(&[event], Some(&condition)),
        ];
        for request in &requests {
            for statement in &request.statements {
                assert!(
                    statement.query.contains(r#""hs_1"."#),
                    "an unqualified name reaches the server: {}",
                    statement.query
                );
            }
        }
    }

    /// The conditional append must be **two** statements or the isolation header
    /// is not sent at all.
    ///
    /// `SqlRequest::headers` returns an empty vec below two statements, and the
    /// endpoint ignores `Neon-Batch-Isolation-Level` on the single form — both
    /// measured. So a refactor that collapsed this back into one CTE would run at
    /// `READ COMMITTED` and lose an update, with no test failing anywhere else.
    #[test]
    fn a_conditional_append_carries_the_isolation_header() {
        let store = store();
        let tags = Tags::from_pairs([("k", "v")]).unwrap();
        let condition =
            AppendCondition::new(Query::from_item(QueryItem::tagged(tags.clone()).unwrap()));
        let event = Event::new("T", b"x".to_vec()).unwrap().with_tags(tags);

        let request =
            store.conditional_append_request(core::slice::from_ref(&event), Some(&condition));
        assert_eq!(request.statements.len(), 2);
        assert!(
            request
                .headers()
                .iter()
                .any(|(name, value)| *name == "Neon-Batch-Isolation-Level"
                    && *value == "Serializable")
        );

        // And the unconditional one deliberately does not: it asserts nothing, so
        // it pays nothing.
        let unconditional = store.conditional_append_request(&[event], None);
        assert_eq!(unconditional.statements.len(), 1);
        assert!(unconditional.headers().is_empty());
    }

    /// `None` is position zero, never a literal `NULL`.
    ///
    /// `position > NULL` is `NULL` for every row, so the guard would match
    /// nothing and the condition would admit every append — a rejection that
    /// looks like an acceptance, and the one arithmetic slip in this file that no
    /// type could catch.
    #[test]
    fn an_unbounded_guard_compares_against_zero() {
        let store = store();
        let condition = AppendCondition::new(Query::all());
        let request = store.probe_request(&condition);
        assert!(request.statements[0].query.contains("position > 0"));
        assert!(!request.statements[0].query.contains("NULL"));
    }

    /// The guard predicate is numbered from `$2` inside the insert and from `$1`
    /// in the standalone probe.
    #[test]
    fn the_inserts_guard_starts_after_the_batch_placeholder() {
        let store = store();
        let condition = AppendCondition::new(Query::from_item(QueryItem::of_types(["A"]).unwrap()));
        let event = Event::new("T", b"x".to_vec()).unwrap();

        let batch = store.conditional_append_request(&[event], Some(&condition));
        assert!(batch.statements[0].query.contains("event_type IN ($1)"));
        assert!(batch.statements[1].query.contains("event_type IN ($2)"));
        assert_eq!(
            batch.statements[1].params.len(),
            2,
            "the batch, then the type"
        );
    }

    #[test]
    fn a_zero_limit_reads_nothing_rather_than_everything() {
        let store = store();
        let options = ReadOptions::new().limit(0);
        assert!(
            store.read_request(&Query::all(), options).statements[0]
                .query
                .ends_with("LIMIT 0")
        );
    }

    /// The `ORDER BY` names the table's column and not the output alias.
    ///
    /// A bare `ORDER BY position` binds to `position::text AS position` in the
    /// SELECT list, because Postgres resolves output-column names first there —
    /// and then the log comes back in lexicographic order: 1, 10, 100, 101, …,
    /// 11, 110. It is correct for every batch under ten events, which is why the
    /// whole event-store family passed it and only the 128-event minimum-batch
    /// rule and the model family did not.
    #[test]
    fn the_order_by_names_the_column_and_not_the_text_alias() {
        let store = store();
        let sql = store
            .read_request(&Query::all(), ReadOptions::new())
            .statements[0]
            .query
            .clone();
        assert!(
            sql.contains(r#"ORDER BY "hs_1"."event".position ASC"#),
            "the sort must be over the bigint column, not the `::text` projection: {sql}"
        );
    }

    /// Reading backwards swaps the comparisons, not the roles.
    #[test]
    fn backwards_bounds_the_older_end_with_to() {
        let store = store();
        let options = ReadOptions::new()
            .from(SequencePosition::new(9).unwrap())
            .to(SequencePosition::new(2).unwrap())
            .backwards();
        let sql = store.read_request(&Query::all(), options).statements[0]
            .query
            .clone();
        assert!(sql.contains("position <= 9"), "{sql}");
        assert!(sql.contains("position >= 2"), "{sql}");
        assert!(sql.contains("position DESC"), "{sql}");
    }

    /// A body recorded from the live endpoint: a conditional append that won.
    const APPENDED: &[u8] = br#"{"results":[{"fields":[],"rows":[{"conflict":null}],"command":"SELECT","rowCount":1,"rowAsArray":false},{"fields":[],"rows":[{"position":"4"},{"position":"5"}],"command":"INSERT","rowCount":2,"rowAsArray":false}]}"#;

    /// A body recorded from the live endpoint: a conditional append that lost.
    const CONFLICTED: &[u8] = br#"{"results":[{"fields":[],"rows":[{"conflict":"3"}],"command":"SELECT","rowCount":1,"rowAsArray":false},{"fields":[],"rows":[],"command":"INSERT","rowCount":0,"rowAsArray":false}]}"#;

    /// A body recorded from the live endpoint: a serialisation failure.
    const SERIALISATION_FAILURE: &[u8] = br#"{"message":"could not serialize access due to read/write dependencies among transactions","code":"40001","detail":null,"hint":"The transaction might succeed if retried.","severity":"ERROR"}"#;

    fn decode(
        body: &'static [u8],
        status: u16,
    ) -> Result<AppendOutcome, NeonError<std::io::Error>> {
        decode_append_response(&HttpResponse::new(status, body), 64 * 1024)
    }

    #[test]
    fn the_last_position_is_the_highest_the_insert_returned() {
        match decode(APPENDED, 200) {
            Ok(AppendOutcome::Appended(position)) => assert_eq!(position.get(), 5),
            other => panic!("expected an append, got {:?}", other.err()),
        }
    }

    #[test]
    fn a_conflict_carries_the_position_the_probe_found() {
        match decode(CONFLICTED, 200) {
            Ok(AppendOutcome::Conflict(position)) => assert_eq!(position.get(), 3),
            other => panic!("expected a conflict, got {:?}", other.err()),
        }
    }

    /// The retry loop's whole trigger, decoded from the endpoint's own rendering.
    #[test]
    fn a_serialisation_failure_arrives_as_a_sql_error_with_its_sqlstate() {
        let error = decode(SERIALISATION_FAILURE, 400).expect_err("a 400 is not an outcome");
        match error {
            NeonError::Sql(sql) => assert!(
                sql.is_serialization_failure(),
                "the retry loop matches on the SQLSTATE and nothing else"
            ),
            other => panic!("expected a SQL error, got {other:?}"),
        }
    }

    /// A body over the ceiling is refused before it is parsed.
    #[test]
    fn an_oversized_body_is_refused_without_being_read() {
        let error = decode_append_response::<std::io::Error>(&HttpResponse::new(200, APPENDED), 4)
            .expect_err("a body over the ceiling is not an outcome");
        assert!(matches!(error, NeonError::ResponseTooLarge { .. }));
    }

    /// A row recorded from the live endpoint, through the adapter's own `SELECT`.
    const READ_ROW: &[u8] = br#"{"fields":[],"command":"SELECT","rowCount":1,"rows":[{"position":"2","event_type":"Enrolled","data":"3q2+7w==","metadata":null,"tags":["course:c1","student:s1"],"origin_store":"HYfjIWOxSmyhl/na/L7bag==","origin_position":"2","recorded_at":"1757203200000"}]}"#;

    #[test]
    fn a_recorded_row_decodes_into_the_event_that_was_written() {
        let events: Vec<SequencedEvent> =
            decode_read_response::<std::io::Error>(&HttpResponse::new(200, READ_ROW), 64 * 1024)
                .expect("a recorded row decodes");
        assert_eq!(events.len(), 1);
        let event = &events[0];
        assert_eq!(event.position.get(), 2);
        assert_eq!(event.event.event_type().as_str(), "Enrolled");
        assert_eq!(event.event.data().to_vec(), vec![0xde, 0xad, 0xbe, 0xef]);
        assert!(
            event.event.metadata().is_none(),
            "`None` and `Some(empty)` are different values and a rule says so"
        );
        assert_eq!(event.event.tags().len(), 2);
        assert_eq!(event.id.position().get(), 2);
    }

    /// A compiling call site for the contract crate's own generic helper.
    ///
    /// Worth having because [`NeonReadStream`](super::NeonReadStream) borrows
    /// `&self` — it holds the transport reference it will send through — and a
    /// generic helper that took the stream by value with a `'static` bound would
    /// reject it. `read_decision_model` does not, so the borrowing stream is
    /// inside what the port actually promises.
    async fn accepts_the_generic_helper<T: SqlTransport>(
        store: &NeonEventStore<T>,
        query: &Query,
    ) -> Result<(Vec<SequencedEvent>, Option<SequencePosition>), NeonError<T::Error>> {
        happenstance_core::read_decision_model(store, query).await
    }

    #[test]
    fn the_generic_helper_call_site_compiles() {
        let _ = accepts_the_generic_helper::<NullTransport>;
    }

    /// The ceilings clear the floors every store must clear.
    #[test]
    fn the_stated_ceilings_clear_the_specification_floors() {
        const {
            assert!(
                NeonEventStore::<NullTransport>::MAX_EVENT_DATA_LEN
                    >= happenstance_core::MIN_SUPPORTED_EVENT_DATA_LEN
            );
            assert!(
                NeonEventStore::<NullTransport>::MAX_TAGS_PER_EVENT
                    >= happenstance_core::MIN_SUPPORTED_TAGS_PER_EVENT
            );
            assert!(
                NeonEventStore::<NullTransport>::MAX_EVENTS_PER_BATCH
                    >= happenstance_core::MIN_SUPPORTED_EVENTS_PER_BATCH
            );
        }
    }
}
