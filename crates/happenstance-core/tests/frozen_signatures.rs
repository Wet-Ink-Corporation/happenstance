//! Phase 4's proof artefact: the four things a **generic** consumer must be able
//! to do against the frozen signatures.
//!
//! Every function here is bound on [`EventStore`] or [`SendEventStore`] and
//! never on a concrete store, because that is the whole point. A consumer
//! written against `MemoryEventStore` proves nothing about the port — it proves
//! something about `MemoryEventStore`, and the two only look alike while there
//! is one implementation.
//!
//! # Why this file is a compile before it is a test
//!
//! Three of the four cases are discharged the moment this file builds. The
//! assertions exist so that a reader can see what is being claimed and so the
//! arrangement cannot rot into a shape that compiles while meaning nothing, but
//! the artefact is the signature, not the assertion.
//!
//! # What changed between the plan and the freeze, and why it is recorded here
//!
//! The runbook expected this file to require `read` to take its `Query` **by
//! value**, on the grounds that the opaque return type of an RPITIT captures
//! every in-scope lifetime, so a caller cannot return the stream from a function
//! or store it in a struct. ES-13 is `[FROZEN]` on `&Query` and its `Rejects:`
//! line names that exact change as the wrong fix.
//!
//! Both are right, and the resolution is not a compromise: the constraint is
//! that the query must be owned by something that **outlives the stream**, and a
//! *parameter* satisfies that where a local does not. Cases 1 and 2 below take
//! the query by reference from their caller and compile unchanged against the
//! frozen signature. What does not compile is a local, and
//! [`escaping_cases_need_the_query_to_outlive_the_stream`] documents the four
//! diagnostics that arrangement produces, because "it does not compile" is not
//! useful to anyone who has not tried it.

#![cfg(feature = "memory")]
#![allow(clippy::unwrap_used, reason = "test code, per the house style")]

use core::pin::pin;
use std::sync::Arc;

use futures_core::Stream;
// **One flavour imported, not both.** Bringing `SendEventStore` into scope
// beside `EventStore` makes every `store.read(..)` and `store.append(..)` in
// this file `error[E0034]: multiple applicable items in scope`, because a
// `MemoryEventStore` implements both and neither is more specific. `store.rs`
// warns about exactly this; the artefact hit it on the first compile, which is
// the best evidence the warning is worth its space. Case 3 names the `Send`
// flavour through its full path instead.
use happenstance_core::{
    AppendCondition, Event, EventId, EventStore, MemoryEventStore, Query, QueryItem, ReadOptions,
    SequencedEvent, Tags, collect,
};

// =====================================================================
// Case 1 — a read stream is returned from a function
// =====================================================================

/// Returns a stream built inside the function, over a query the **caller** owns.
///
/// This is the case the runbook expected to be impossible under `&Query`. It is
/// possible; what it requires is that the query outlive the returned stream,
/// which a parameter does by construction and a local never can.
fn replay<'a, S: EventStore>(
    store: &'a S,
    query: &'a Query,
) -> impl Stream<Item = Result<SequencedEvent, S::Error>> + 'a {
    store.read(query, ReadOptions::new())
}

#[tokio::test]
async fn a_read_stream_can_be_returned_from_a_generic_function() {
    let store = seeded().await;
    let query = Query::all();

    let events = collect(replay(&store, &query)).await.unwrap();

    assert_eq!(events.len(), 3, "the stream really carried the whole log");
}

// =====================================================================
// Case 2 — a read stream is stored in a struct field
// =====================================================================

/// A consumer that owns a stream between polls, which is what a projection
/// runner is.
struct Replay<T> {
    stream: T,
    seen: usize,
}

/// Builds one over a store and a query the caller owns.
///
/// A free function rather than an inherent one: an `impl<S> Replay<()>` block
/// leaves `S` unconstrained by the self type (`error[E0207]`), because the
/// parameter appears only in the return position.
fn replay_over<'a, S: EventStore>(
    store: &'a S,
    query: &'a Query,
) -> Replay<impl Stream<Item = Result<SequencedEvent, S::Error>> + 'a> {
    Replay {
        stream: store.read(query, ReadOptions::new()),
        seen: 0,
    }
}

#[tokio::test]
async fn a_read_stream_can_be_held_in_a_struct_field() {
    let store = seeded().await;
    let query = Query::all();

    let mut replay = replay_over(&store, &query);
    let drained = collect(pin!(&mut replay.stream)).await.unwrap();
    replay.seen = drained.len();

    assert_eq!(replay.seen, 3, "the field really held a live stream");
}

// =====================================================================
// Case 3 — a replay is spawned under the `Send` flavour
// =====================================================================

/// Holds the stream across an await inside a real `tokio::spawn`.
///
/// This is where ADR-0008's `Self: Sync` reasoning is either right or is
/// discovered to be wrong, and it is the arrangement that rejects an
/// `async fn read(..) -> Result<impl Stream, E>` refactor: under that shape
/// `trait_variant` marks the *future* `Send` and a test asserting `Send` on the
/// return value is satisfied by the wrong thing. Only holding the stream across
/// a suspension point inside a spawned task distinguishes them.
///
/// `memory.rs`'s `spawns_from_generic` makes the same claim for the reference
/// store; this one makes it for a **generic** consumer, which is the half the
/// runbook's artefact asks for.
fn spawn_replay<S>(store: Arc<S>, query: Query) -> tokio::task::JoinHandle<usize>
where
    S: happenstance_core::SendEventStore + Send + Sync + 'static,
{
    tokio::spawn(async move {
        let stream = happenstance_core::SendEventStore::read(&*store, &query, ReadOptions::new());
        // The stream is held across this await, inside a task the executor may
        // move between threads. That is the load-bearing part: it is what makes
        // the *stream* prove `Send` rather than merely the future that produced
        // it, and it is why an `async fn read` refactor cannot satisfy this.
        collect(stream).await.map_or(0, |events| events.len())
    })
}

#[tokio::test]
async fn a_replay_can_be_spawned_under_the_send_flavour() {
    let store = Arc::new(seeded().await);

    let counted = spawn_replay(Arc::clone(&store), Query::all())
        .await
        .unwrap();

    assert_eq!(counted, 3, "the spawned task really drained the log");
}

// =====================================================================
// Case 4 — the caller still owns its events after appending them
// =====================================================================

/// Appends a batch and then takes one of its events apart.
///
/// The runbook framed this as a thing the signatures forbade. They do not, and
/// the honest statement is worth having in the artefact rather than a case that
/// passes for the wrong reason: `append` borrows, so the caller never gives the
/// `Vec` away and `into_parts` is reachable afterwards — as this compiles and
/// runs to show.
///
/// What `&[Event]` actually costs is inside the **adapter**, which must clone
/// each event to keep it. That cost is ES-17's, its falsifier is a measurement
/// on a real adapter, and phase 8 owns the benchmark. A phase-4 artefact cannot
/// demonstrate it, and one that claimed to would be measuring nothing.
async fn append_then_disassemble<S: EventStore>(store: &S, events: Vec<Event>) -> (usize, usize) {
    let condition =
        AppendCondition::new(Query::from_item(QueryItem::of_types(["Retained"]).unwrap()));
    let _ = store.append(&events, Some(&condition)).await;

    let mut owned = events;
    let parts = owned.pop().unwrap().into_parts();
    (owned.len(), parts.data.len())
}

#[tokio::test]
async fn a_caller_still_owns_its_batch_after_appending_it() {
    let store = MemoryEventStore::new();
    let batch = vec![
        Event::new("Retained", &b"first"[..]).unwrap(),
        Event::new("Retained", &b"second"[..]).unwrap(),
    ];

    let (remaining, payload_len) = append_then_disassemble(&store, batch).await;

    assert_eq!(remaining, 1);
    assert_eq!(payload_len, b"second".len());
}

// =====================================================================
// The regression pin: what does NOT compile, and with which diagnostic
// =====================================================================

/// The arrangement cases 1 and 2 must not be rewritten into.
///
/// A query bound to a **local** cannot outlive a stream returned from the
/// function that declared it, and the diagnostic depends on how the return type
/// is spelled — which is why ES-13 states the defect rather than pinning a code:
///
/// | Arrangement | Diagnostic |
/// |---|---|
/// | `store.read(&Query::all(), ..)` bound to a `let` | `error[E0716]` |
/// | returned, bare `-> impl Stream<..>` | `error[E0597]` |
/// | returned, `+ '_` or `+ 'a` | `error[E0515]` |
/// | returned inside a struct, bare | `error[E0597]` |
///
/// All four are the same defect reported from two ends. Without a lifetime bound
/// the compiler reasons from the *borrow* — `&query` must outlive the return and
/// `query` drops at the end of the function. With one, the opaque type is
/// required to live for `'a`, so it reasons from the *value* instead. The cause
/// either way is that the opaque type captures the query's lifetime.
///
/// ```compile_fail
/// use futures_core::Stream;
/// use happenstance_core::{EventStore, Query, ReadOptions, SequencedEvent};
///
/// fn escapes<S: EventStore>(store: &S) -> impl Stream<Item = Result<SequencedEvent, S::Error>> {
///     let query = Query::all();
///     store.read(&query, ReadOptions::new())
/// }
/// ```
#[expect(
    dead_code,
    reason = "the doctest above is the artefact; this item exists to carry it"
)]
fn escaping_cases_need_the_query_to_outlive_the_stream() {}

// =====================================================================
// Identity does not reach `append`'s signature
// =====================================================================

/// `append` takes events and a condition, and nothing else.
///
/// Exit criterion 10 asks for this out loud: if identity had reached `append`,
/// the signature half of phase 4 would have been reopened by its value half.
/// A foreign identity arrives through the ingest port in the replication crate,
/// never here — so this function type-checks exactly as it did before
/// `SequencedEvent` grew two fields.
async fn append_takes_no_identity<S: EventStore>(store: &S, events: &[Event]) -> bool {
    store.append(events, None).await.is_ok()
}

#[tokio::test]
async fn identity_does_not_reach_appends_signature() {
    let store = MemoryEventStore::new();
    let events = [Event::new("Plain", &b"{}"[..]).unwrap()];

    assert!(append_takes_no_identity(&store, &events).await);

    // And the store stamped one anyway, which is the other half of VT-10: the
    // caller supplies no identity and the store assigns one regardless.
    let stored = store.snapshot();
    assert_eq!(stored.len(), 1);
    assert_eq!(stored[0].id.store(), store.store_id());
}

// =====================================================================
// Membership is reachable generically, without a query
// =====================================================================

/// The operation VT-7 requires and deliberately keeps out of the query language.
///
/// Bound on `EventStore`, so it is the port that offers this and not a
/// particular store.
async fn holds<S: EventStore>(store: &S, id: EventId) -> bool {
    store.contains_event_id(id).await.unwrap_or(false)
}

#[tokio::test]
async fn membership_is_answerable_through_the_port() {
    let store = seeded().await;
    let stored = store.snapshot();

    assert!(holds(&store, stored[0].id).await);

    // An identity from a different incarnation is not held, which is what makes
    // the store half of the pair load-bearing rather than decorative.
    let elsewhere = MemoryEventStore::new();
    elsewhere
        .append(&[Event::new("Elsewhere", &b"{}"[..]).unwrap()], None)
        .await
        .unwrap();
    let foreign = elsewhere.snapshot()[0].id;

    assert!(!holds(&store, foreign).await);
}

// =====================================================================
// Shared arrangement
// =====================================================================

/// Three events, one of them tagged, so that `Query::all()` has something to
/// distinguish it from a scoped query.
async fn seeded() -> MemoryEventStore {
    let store = MemoryEventStore::new();
    store
        .append(
            &[
                Event::new("First", &b"1"[..]).unwrap(),
                Event::new("Second", &b"2"[..])
                    .unwrap()
                    .with_tags(Tags::from_pairs([("edge", "middle")]).unwrap()),
                Event::new("Third", &b"3"[..]).unwrap(),
            ],
            None,
        )
        .await
        .unwrap();
    store
}
