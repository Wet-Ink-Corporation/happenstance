//! A query past SQLite's pushdown limits is served, against a real Durable
//! Object.
//!
//! # Why this target exists rather than the arithmetic that was here first
//!
//! `src/query_sql.rs`'s host tests assert the *width* of the statements the
//! partition produces: how many compound terms, how many bound parameters. That
//! is a claim about a string. It is not a claim that SQLite would have refused
//! the string, and it is not a claim that the merge behind the partition
//! reassembles the right rows — so a chunking fix could ship believed on the
//! strength of a green gate that never ran it. This repository has that defect
//! on file in another form: a `compile_fail` fence in an integration-test
//! target, which cargo never hands to a compiler, was inert for as long as it
//! stood.
//!
//! So the first case below is a **control**: it hands this runtime the
//! unpartitioned statement and asserts the driver refuses it, in this harness,
//! today. Everything after it is the same shape going through the adapter and
//! succeeding. Without the control the passing cases would prove only that
//! nothing went wrong; with it they prove the wall is real, reachable here, and
//! no longer hit.
//!
//! # The two walls, and the two shapes that cross them
//!
//! * `SQLITE_MAX_COMPOUND_SELECT` — **500 terms**. Crossed by
//!   [`ARM_WIDE_ITEMS`] items of one tag each: 1,000 arms, and only 1,000 bound
//!   parameters, so the parameter budget is nowhere near binding and the arm
//!   axis is the one under test.
//! * `SQLITE_MAX_VARIABLE_NUMBER` — **32,766 bound parameters**. Crossed by
//!   [`PARAMETER_WIDE_ITEMS`] items of [`TYPES_PER_ITEM`] event types each:
//!   32,800 parameters, on exactly `MAX_QUERY_ARMS_PER_STATEMENT` arms — so the
//!   arm axis says *one statement* and the parameter axis is the only thing that
//!   can split it. That is the independence the two ceilings exist for, driven
//!   rather than argued.
//!
//! Types rather than tags for the second shape, and it is a cost decision worth
//! stating: an item's parameters are one per tag **and** one per type, and a
//! tag-wide item renders a chain of nested `position IN (SELECT …)`
//! intersections where a type-wide item renders one flat `event_type IN (?,…)`.
//! Both reach the wall; the flat one asks the planner for far less to reach it,
//! which is what keeps this target's runtime in seconds rather than minutes.
//!
//! # Both callers, because only one of them holds the turn
//!
//! The read path and the append-condition guard ask the same question, and the
//! guard asks it inside the append turn with the caller's decision already
//! taken. VT-24 rejects that timing by name. Every shape below is driven through
//! both, and the guard cases include one that must be **violated** — because a
//! merge that answered from its first chunk and stopped would accept an append
//! it should have rejected, which is the silent half of VT-23's named wrong
//! implementation and the half a refusal-only test cannot see.
//!
//! # What this is not
//!
//! Not `workerd`. The shim is a Node process holding real SQLite behind the
//! `DurableObjectState` shape, so the *pushdown* limits below are the real
//! engine's and the *platform's* limits are absent entirely — which is
//! `kb-decision-0023`'s recorded finding and not this target's to fix. What
//! matters here is that `SQLITE_MAX_COMPOUND_SELECT` and
//! `SQLITE_MAX_VARIABLE_NUMBER` are compile-time constants of the engine the
//! shim runs, so the control case measures the same wall a deployed object has.

#![cfg(target_arch = "wasm32")]

use core::future::poll_fn;

use futures_core::Stream;
use happenstance_cloudflare::event_store::CloudflareEventStore;
use happenstance_cloudflare::host::DurableObjectHost;
use happenstance_cloudflare::sql_storage::{SqlStorage, SqlValue};
use happenstance_core::{
    AppendCondition, AppendError, Event, EventStore, Query, QueryItem, ReadOptions, Tags,
};
use wasm_bindgen_test::wasm_bindgen_test;

/// Items enough to pass `SQLITE_MAX_COMPOUND_SELECT`'s 500 terms, twice over.
///
/// One tag each, so the query is 1,000 arms and 1,000 bound parameters — the
/// arm axis binding, the parameter axis idle.
const ARM_WIDE_ITEMS: usize = 1_000;

/// Items in the parameter-wide query: the chunk width **exactly**.
///
/// So the arm partition would return one statement, and anything that splits
/// this query split it on the other axis.
const PARAMETER_WIDE_ITEMS: usize = CloudflareEventStore::MAX_QUERY_ARMS_PER_STATEMENT;

/// Event types per item of the parameter-wide query.
///
/// `400 * 82 = 32,800`, which clears `SQLITE_MAX_VARIABLE_NUMBER`'s 32,766 by 34
/// — deliberately the smallest margin that crosses it, because every parameter
/// past the wall is planner work this target pays for and proves nothing extra.
const TYPES_PER_ITEM: usize = 82;

/// `SQLITE_MAX_VARIABLE_NUMBER`, the default this engine is built with.
const BOUND_PARAMETERS: usize = 32_766;

/// A fresh Durable Object with the schema applied.
fn store() -> CloudflareEventStore {
    let host = DurableObjectHost::new();
    let store = CloudflareEventStore::new(host.storage());
    store.migrate().expect("the schema applies");
    // The host is dropped here and the storage handle keeps the object alive:
    // `SqlStorage` is what the adapter holds, and cloning it aliases the object
    // rather than copying it.
    store
}

/// Drains a stream through `poll_next` and nothing else.
///
/// Hand-rolled for the reason this crate's other targets give: there is no
/// `futures-util` here, and reaching the next item through `poll_next` is what a
/// caller of the bare, `!Send` flavour actually does.
async fn drain<S: Stream>(stream: S) -> Vec<S::Item> {
    let mut stream = Box::pin(stream);
    let mut out = Vec::new();
    while let Some(item) = poll_fn(|cx| stream.as_mut().poll_next(cx)).await {
        out.push(item);
    }
    out
}

/// Every position the store yields for `query`, in the order it yields them.
async fn positions(store: &CloudflareEventStore, query: &Query) -> Vec<u64> {
    drain(store.read(query, ReadOptions::new()))
        .await
        .into_iter()
        .map(|item| item.expect("every item of the read decodes").position.get())
        .collect()
}

/// An event carrying the single tag `subject:s<index>`.
fn tagged(index: usize) -> Event {
    Event::new("Subject", &b"{}"[..])
        .expect("a valid event type")
        .with_tags(
            Tags::from_pairs([("subject", format!("s{index}").as_str())]).expect("a valid tag"),
        )
}

/// An event of the type item `index` of the parameter-wide query names first.
fn typed(index: usize) -> Event {
    Event::new(format!("T{}", index * TYPES_PER_ITEM), &b"{}"[..]).expect("a valid event type")
}

/// `ARM_WIDE_ITEMS` single-tag items; item *i* names `subject:s<i>`.
fn arm_wide_query() -> Query {
    Query::from_items((0..ARM_WIDE_ITEMS).map(|index| {
        QueryItem::tagged(
            Tags::from_pairs([("subject", format!("s{index}").as_str())]).expect("a valid tag"),
        )
        .expect("an item carrying a tag is constructible")
    }))
    .expect("a non-empty item list is a query")
}

/// `PARAMETER_WIDE_ITEMS` items of `TYPES_PER_ITEM` distinct event types each.
///
/// The type sets are disjoint across items, so an event of one type is matched
/// by exactly one item and "which chunk answered" is observable.
fn parameter_wide_query() -> Query {
    Query::from_items((0..PARAMETER_WIDE_ITEMS).map(|index| {
        let base = index * TYPES_PER_ITEM;
        QueryItem::of_types((base..base + TYPES_PER_ITEM).map(|n| format!("T{n}")))
            .expect("an item constraining types is constructible")
    }))
    .expect("a non-empty item list is a query")
}

// ---------------------------------------------------------------------------
// The control: this runtime really does refuse the unpartitioned statement
// ---------------------------------------------------------------------------

/// Both walls, hit deliberately, through the storage handle rather than the
/// adapter.
///
/// This is the case that makes every other case below mean something. It builds
/// the two statements a translation with no partition would have built — the
/// same arm count and the same parameter count the adapter's own plan carries in
/// total — and asserts the driver refuses each of them. If SQLite ever stopped
/// refusing, this case fails and says so, rather than the rest of the target
/// silently degrading into a suite that proves nothing.
#[wasm_bindgen_test]
fn the_unpartitioned_statement_is_refused_by_this_runtime() {
    let host = DurableObjectHost::new();
    let sql: SqlStorage = host.storage();
    let store = CloudflareEventStore::new(sql.clone());
    store.migrate().expect("the schema applies");

    // One compound SELECT of ARM_WIDE_ITEMS terms.
    let arms: Vec<&str> = vec!["SELECT position FROM event_tag WHERE tag = ?"; ARM_WIDE_ITEMS];
    let bindings: Vec<SqlValue> = (0..ARM_WIDE_ITEMS)
        .map(|index| SqlValue::Text(format!("subject:s{index}")))
        .collect();
    let refused = sql.exec(&arms.join(" UNION "), &bindings);
    assert!(
        refused.is_err(),
        "this runtime accepted a compound SELECT of {ARM_WIDE_ITEMS} terms, so \
         SQLITE_MAX_COMPOUND_SELECT is not what this target believes it is and \
         every case below is passing for the wrong reason"
    );

    // One statement binding more than SQLITE_MAX_VARIABLE_NUMBER parameters.
    let width = PARAMETER_WIDE_ITEMS * TYPES_PER_ITEM;
    assert!(
        width > BOUND_PARAMETERS,
        "the fixture must exceed SQLITE_MAX_VARIABLE_NUMBER or it proves nothing"
    );
    let placeholders = vec!["?"; width].join(",");
    let bindings: Vec<SqlValue> = (0..width)
        .map(|n| SqlValue::Text(format!("T{n}")))
        .collect();
    let refused = sql.exec(
        &format!("SELECT position FROM event WHERE event_type IN ({placeholders})"),
        &bindings,
    );
    assert!(
        refused.is_err(),
        "this runtime accepted a statement binding {width} parameters, so \
         SQLITE_MAX_VARIABLE_NUMBER is not what this target believes it is"
    );
}

// ---------------------------------------------------------------------------
// The read path
// ---------------------------------------------------------------------------

/// The arm axis, through `EventStore::read`.
///
/// The two matching events sit at opposite ends of the item list — item 0 and
/// item 999 — so they land in different chunks of the plan. A store that served
/// its first chunk and stopped returns one event; a store with no partition at
/// all returns an error.
#[wasm_bindgen_test]
async fn a_read_past_the_compound_select_ceiling_is_served_from_every_chunk() {
    let store = store();
    store
        .append(&[tagged(0), tagged(ARM_WIDE_ITEMS - 1)], None)
        .await
        .expect("the seed lands");

    let query = arm_wide_query();
    assert!(
        CloudflareEventStore::planned_statement_count(&query) > 1,
        "at one statement this would be testing the narrow path and the merge \
         would be dead code"
    );

    let seen = positions(&store, &query).await;
    assert_eq!(
        seen.len(),
        2,
        "both matching events must come back: one lies in the first chunk of the \
         plan and one in the last"
    );
    assert!(
        seen[0] < seen[1],
        "the merge must yield in position order across chunks, because the next \
         page resumes from the last position yielded: {seen:?}"
    );
}

/// The parameter axis, through `EventStore::read`.
///
/// `PARAMETER_WIDE_ITEMS` is the arm width exactly, so the arm partition alone
/// would plan this as one statement — and that statement binds 32,800
/// parameters, which the control above watched this runtime refuse.
#[wasm_bindgen_test]
async fn a_read_past_the_bound_parameter_ceiling_is_served_from_every_chunk() {
    let store = store();
    store
        .append(&[typed(0), typed(PARAMETER_WIDE_ITEMS - 1)], None)
        .await
        .expect("the seed lands");

    let query = parameter_wide_query();
    assert!(
        CloudflareEventStore::planned_statement_count(&query) > 1,
        "the arm axis says one statement here, so a plan of one means the \
         parameter axis is not partitioning anything"
    );

    let seen = positions(&store, &query).await;
    assert_eq!(
        seen.len(),
        2,
        "the first and last items of the query each match one event, and they \
         are in different chunks"
    );
}

// ---------------------------------------------------------------------------
// The append path, where the failure would arrive inside the turn
// ---------------------------------------------------------------------------

/// The arm axis, as an append-condition guard.
///
/// Nothing matches the guard, so the append must land. What would fail without
/// the partition is not the guard's answer — it is `prepare`, arriving as
/// `AppendError::Store` carrying a raw driver string, with the caller's decision
/// already taken and no honest variant to travel in.
#[wasm_bindgen_test]
async fn an_append_guard_past_the_compound_select_ceiling_is_not_refused() {
    let store = store();
    store
        .append(
            &[Event::new("Seed", &b"{}"[..]).expect("a valid event type")],
            None,
        )
        .await
        .expect("the seed lands");

    store
        .append(
            &[Event::new("Decided", &b"{}"[..]).expect("a valid event type")],
            Some(&AppendCondition::new(arm_wide_query())),
        )
        .await
        .expect("a guard past the compound-SELECT ceiling is chunked, never refused");
}

/// The parameter axis, as an append-condition guard.
#[wasm_bindgen_test]
async fn an_append_guard_past_the_bound_parameter_ceiling_is_not_refused() {
    let store = store();
    store
        .append(
            &[Event::new("Seed", &b"{}"[..]).expect("a valid event type")],
            None,
        )
        .await
        .expect("the seed lands");

    store
        .append(
            &[Event::new("Decided", &b"{}"[..]).expect("a valid event type")],
            Some(&AppendCondition::new(parameter_wide_query())),
        )
        .await
        .expect("a guard past the bound-parameter ceiling is chunked, never refused");
}

/// And the guard answers from **every** chunk, not the first.
///
/// The only event the guard matches is named by the *last* item of the query, so
/// it is in the last chunk of the plan. A merge that folded only its first chunk
/// would find no violation and accept an append that must be rejected — which is
/// worse than the refusal this whole change removes, because it is silent and it
/// is a lost update.
#[wasm_bindgen_test]
async fn a_wide_guard_answers_from_every_chunk_not_the_first() {
    let store = store();
    store
        .append(&[tagged(ARM_WIDE_ITEMS - 1)], None)
        .await
        .expect("the seed lands");

    let outcome = store
        .append(
            &[Event::new("Decided", &b"{}"[..]).expect("a valid event type")],
            Some(&AppendCondition::new(arm_wide_query())),
        )
        .await;

    assert!(
        matches!(outcome, Err(AppendError::ConditionViolated(_))),
        "the only matching event is named by the last item of the query, so a \
         guard that folded its first chunk alone would accept this append: \
         {outcome:?}"
    );
}

/// The same question on the parameter axis, so neither partition is trusted on
/// the other's evidence.
#[wasm_bindgen_test]
async fn a_parameter_wide_guard_answers_from_every_chunk_not_the_first() {
    let store = store();
    store
        .append(&[typed(PARAMETER_WIDE_ITEMS - 1)], None)
        .await
        .expect("the seed lands");

    let outcome = store
        .append(
            &[Event::new("Decided", &b"{}"[..]).expect("a valid event type")],
            Some(&AppendCondition::new(parameter_wide_query())),
        )
        .await;

    assert!(
        matches!(outcome, Err(AppendError::ConditionViolated(_))),
        "the matching event is named only by the last item, which the parameter \
         partition puts in the last chunk: {outcome:?}"
    );
}
