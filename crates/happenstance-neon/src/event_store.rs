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
//! adapter can sit at two points of. An adapter that wants both must be two
//! types, and this one is one type on the weaker trait so that one source file
//! serves both targets.
//!
//! # Intended schema
//!
//! ```sql
//! CREATE TABLE event (
//!     position   bigserial PRIMARY KEY,
//!     event_type text      NOT NULL,
//!     data       bytea     NOT NULL,
//!     metadata   bytea,
//!     tags       text[]    NOT NULL  -- canonical sorted encoding
//! );
//!
//! CREATE INDEX event_tags_idx ON event USING gin (tags);
//! CREATE INDEX event_type_idx ON event (event_type, position);
//! ```
//!
//! `bigserial` is `nextval()`, which allocates **outside** the transaction, so
//! positions become visible out of order. That is the visibility problem
//! `happenstance-postgres` exists to measure; this crate inherits it and adds
//! the fact that it cannot hold a transaction open across the measurement.

use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};

use futures_core::Stream;
use happenstance_core::{
    AppendCondition, AppendError, ConditionViolated, Event, EventId, EventStore, Query,
    ReadOptions, SequencePosition, SequencedEvent,
};

use crate::config::NeonConfig;
use crate::error::NeonError;
use crate::transport::{HttpResponse, SqlRequest, SqlTransport};

/// An event store over Neon's serverless `/sql` HTTP endpoint.
///
/// # Status: not implemented
///
/// Every body is `todo!()`. The types are the point; see the [crate
/// documentation](crate) for what they are asserting.
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
}

impl<T: SqlTransport> NeonEventStore<T> {
    /// The single `SELECT` a read compiles to.
    ///
    /// One statement, because a read has nothing to be atomic with.
    fn read_request(&self, _query: &Query, _options: ReadOptions) -> SqlRequest {
        todo!("neon: compile a Query and ReadOptions into one SELECT")
    }

    /// The `EXISTS` probe the two-statement append sends first.
    ///
    /// Used only by [`ProbeThenWriteStore`], which exists to be wrong.
    fn probe_request(&self, _condition: &AppendCondition) -> SqlRequest {
        todo!("neon: compile an AppendCondition into a conflict probe")
    }

    /// The unconditional `INSERT … RETURNING position`.
    fn insert_request(&self, _events: &[Event]) -> SqlRequest {
        todo!("neon: compile events into an INSERT … RETURNING position")
    }

    /// The whole append as **one** statement: a CTE that probes and inserts
    /// against one snapshot, and returns both the conflicting position and the
    /// appended one.
    ///
    /// ```sql
    /// WITH probe AS (
    ///     SELECT min(position) AS conflict
    ///       FROM event
    ///      WHERE position > $1          -- AppendCondition::after
    ///        AND (<the condition's Query, as a disjunction of items>)
    /// ), ins AS (
    ///     INSERT INTO event (event_type, data, metadata, tags)
    ///     SELECT * FROM unnest($2::text[], $3::bytea[], $4::bytea[], $5::text[][])
    ///      WHERE NOT EXISTS (SELECT 1 FROM probe WHERE conflict IS NOT NULL)
    ///     RETURNING position
    /// )
    /// SELECT (SELECT max(position) FROM ins)   AS appended,
    ///        (SELECT conflict      FROM probe) AS conflict;
    /// ```
    ///
    /// This answers the ledger's open question in the affirmative: the condition
    /// and the write **do** collapse into one statement, and
    /// `ConditionViolated::conflicting_position` survives the collapse — because
    /// `min(position)` is computed in the same CTE, on the same snapshot, and
    /// projected out alongside the insert's result. A plain
    /// `INSERT … SELECT … WHERE NOT EXISTS` returns zero rows and cannot say
    /// *which* event conflicted; adding the `probe` CTE costs one extra index
    /// scan on every append, including the overwhelmingly common uncontended
    /// one, and buys the position back.
    fn conditional_append_request(
        &self,
        _events: &[Event],
        _condition: Option<&AppendCondition>,
    ) -> SqlRequest {
        todo!("neon: compile the whole append into one CTE statement")
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

    async fn append(
        &self,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<Self::Error>> {
        if events.is_empty() {
            return Err(AppendError::NoEvents);
        }

        let request = self.conditional_append_request(events, condition);
        let response = self
            .transport
            .round_trip(request)
            .await
            .map_err(|err| AppendError::Store(NeonError::Transport(err)))?;

        match decode_append_response::<T::Error>(&response, self.config.max_response_bytes) {
            Ok(AppendOutcome::Appended(position)) => Ok(position),
            Ok(AppendOutcome::Conflict(position)) => Err(AppendError::ConditionViolated(
                ConditionViolated::at(position),
            )),
            Err(err) => Err(AppendError::Store(err)),
        }
    }

    async fn head(&self) -> Result<Option<SequencePosition>, Self::Error> {
        // `SELECT max(position) FROM event`, one statement and therefore exactly
        // one round trip — this endpoint has no cursor and no interactive
        // transaction, so every operation here costs one and only one.
        //
        // The answer is *not* trustworthy on the schema at the top of this
        // module: `bigserial` allocates outside the transaction, so `max` can
        // name a position whose predecessors have not committed yet. Phase 10
        // writes this body against whatever `happenstance-postgres` measures its
        // way to, not before.
        todo!("neon: SELECT max(position), once the visibility question is settled")
    }

    async fn contains_event_id(&self, _id: EventId) -> Result<bool, Self::Error> {
        // `SELECT EXISTS (SELECT 1 FROM event WHERE origin_store = $1 AND
        // origin_position = $2)` — again one statement, one round trip.
        //
        // The intended schema above carries no origin columns at all, so this is
        // the method that adds them plus their unique index; an ingest path that
        // cannot ask this question cannot be idempotent.
        todo!("neon: EXISTS on (origin_store, origin_position), which the schema still lacks")
    }
}

/// What the one-statement append's single row says.
///
/// Both variants are matched in `append` and constructed only by a `todo!()`
/// decoder, so `dead_code` fires today and stops firing on the day the decoder
/// is written — which is what `expect` rather than `allow` is for.
#[expect(
    dead_code,
    reason = "constructed by decode_append_response, which is todo!() until phase 10"
)]
enum AppendOutcome {
    /// The insert happened; this is the last position it assigned.
    Appended(SequencePosition),
    /// The insert did not happen, and this is the conflicting position the
    /// `probe` CTE found on the same snapshot.
    Conflict(SequencePosition),
}

/// Turns the endpoint's answer into an outcome.
fn decode_append_response<E>(
    _response: &HttpResponse,
    _max_response_bytes: usize,
) -> Result<AppendOutcome, NeonError<E>> {
    todo!("neon: decode the appended/conflict row")
}

/// Turns the endpoint's answer into the whole result set, at once.
fn decode_read_response<E>(
    _response: &HttpResponse,
    _max_response_bytes: usize,
) -> Result<Vec<SequencedEvent>, NeonError<E>> {
    todo!("neon: decode a buffered result set into SequencedEvents")
}

/// The read "stream": one round trip, then a buffer being drained.
///
/// # This is the honest part of the crate
///
/// [`EventStore::read`]'s doc comment says the stream is what "lets an adapter
/// stream a million-event replay without buffering it". This adapter cannot do
/// that and no version of it ever will, because Neon's `/sql` endpoint has no
/// cursor: the response is one JSON document, capped at
/// [`MAX_RESPONSE_BYTES`](crate::transport::MAX_RESPONSE_BYTES), and it either arrives whole or fails with
/// [`NeonError::ResponseTooLarge`]. The `Stream` impl below is therefore a
/// *shape*, not a capability — it satisfies the port's signature exactly and
/// honours roughly none of the port's prose.
///
/// The one property it does honour is **laziness**. `read` is not `async`, so
/// nothing is sent when it is called; the request sits unsent in the state
/// machine below until the first `poll_next`. That is load-bearing on
/// `wasm32`, where issuing a `fetch` outside a polled future is not merely
/// wasteful but happens off the event loop the runtime owns.
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
    /// Boxed because the future is
    /// `<T as SqlTransport>::round_trip`'s opaque return type, which has no
    /// name and so cannot be a struct field. `dyn Future + 'a` with **no**
    /// `Send` — that is the bound a `JsFuture` can meet.
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
    /// [`MAX_RESPONSE_BYTES`](crate::transport::MAX_RESPONSE_BYTES) constant, and the difference is the difference
    /// between a builder that works and one that compiles.
    /// [`NeonConfig::with_max_response_bytes`](crate::NeonConfig::with_max_response_bytes)
    /// lowers the ceiling; the append path already honoured it and this one
    /// hard-coded the constant, so a caller could set the field, watch the
    /// setter clamp it correctly, and have reads ignore it silently. Nothing
    /// could have caught that: the value is only consulted after a round trip,
    /// and there is no transport to make one with.
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

/// The named attempt: an `append` that **probes and writes in two statements**.
///
/// # It compiles, and that is the finding
///
/// Nothing in [`EventStore::append`]'s signature forbids two round trips, so
/// this type-checks exactly as readily as [`NeonEventStore`] does. A table of
/// adapter shapes that recorded only compiler errors would therefore show this
/// crate as fully compatible with the port — and it is the least capable adapter
/// in the workspace.
///
/// # Why it is wrong here
///
/// Postgres over a real connection can wrap the probe and the write in
/// `BEGIN ISOLATION LEVEL SERIALIZABLE … COMMIT`, so the two statements share a
/// snapshot and the probe's answer is still true when the insert lands. Neon's
/// `/sql` endpoint has **no interactive transaction**: each round trip is its
/// own implicit transaction, and there is no handle to enrol the second one in
/// the first. So the two statements below are two independent transactions with
/// nothing between them, and the window between them is a full network
/// round trip — tens of milliseconds, not microseconds. A concurrent append that
/// commits inside that window is invisible to the probe and unopposed by the
/// insert, and the store silently accepts a write its append condition forbade.
///
/// That is a **lost update**, not a slow path: the failure is silent, produces
/// no error, and is indistinguishable after the fact from a legitimate append.
/// [`NeonEventStore`]'s own `append` is the correct shape — a single CTE that
/// probes and inserts against one snapshot — and it is correct because it is
/// *one statement*.
#[derive(Debug, Clone)]
pub struct ProbeThenWriteStore<T> {
    inner: NeonEventStore<T>,
}

impl<T> ProbeThenWriteStore<T> {
    /// Wraps a store, replacing its append with the two-statement one.
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

    async fn append(
        &self,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<Self::Error>> {
        if events.is_empty() {
            return Err(AppendError::NoEvents);
        }

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
                decode_probe_response::<T::Error>(&response).map_err(AppendError::Store)?;
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

        decode_last_position::<T::Error>(&response).map_err(AppendError::Store)
    }

    // Forwarded, like `read`: this type's declared defect is the two-statement
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
    _response: &HttpResponse,
) -> Result<Option<SequencePosition>, NeonError<E>> {
    todo!("neon: decode the probe's min(position)")
}

/// The last position an unconditional insert assigned.
fn decode_last_position<E>(_response: &HttpResponse) -> Result<SequencePosition, NeonError<E>> {
    todo!("neon: decode INSERT … RETURNING position")
}

#[cfg(test)]
mod tests {
    use super::{NeonEventStore, ProbeThenWriteStore};
    use crate::config::NeonConfig;
    use crate::error::NeonError;
    use crate::transport::{NullTransport, SqlTransport};
    use happenstance_core::{EventStore, Query, SequencePosition, SequencedEvent};

    /// Both stores satisfy the **bare** flavour at a concrete transport. This is
    /// the claim the runbook's phrasing makes, and it is checked on whichever
    /// target the test is built for.
    fn assert_bare_flavour<S: EventStore>() {}

    #[test]
    fn both_stores_are_bare_event_stores() {
        assert_bare_flavour::<NeonEventStore<NullTransport>>();
        assert_bare_flavour::<ProbeThenWriteStore<NullTransport>>();
    }

    #[test]
    fn a_store_can_be_built_without_a_transport() {
        let store = NeonEventStore::new(NullTransport::new(), NeonConfig::default());
        let _wrapped = ProbeThenWriteStore::new(store);
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
        // Naming the function is what instantiates it; nothing is polled, so
        // none of the `todo!()` bodies run.
        let _ = accepts_the_generic_helper::<NullTransport>;
    }
}
