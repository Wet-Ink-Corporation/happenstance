//! A query past a Durable Object's pushdown limits is served, against the
//! shim, which enforces `workerd`'s statement walls.
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
//! So the first two cases are **controls**: one hands this runtime a statement
//! one past each of `workerd`'s walls and asserts the driver refuses it, and one
//! hands it the widest statement this adapter can emit and asserts it is
//! accepted. Everything after them goes through the adapter and succeeds.
//! Without the controls the passing cases would prove only that nothing went
//! wrong; with them they prove the walls are real, reachable here, and no
//! longer hit.
//!
//! # The walls, and the shapes that cross them
//!
//! The shim opens its database with `workerd`'s four statement limits —
//! measured inside `workerd` by `harness/workerd/src/probe.rs` — so the walls
//! here are a Durable Object's rather than SQLite's compiled defaults:
//!
//! * **5 compound `SELECT` terms.** Crossed by [`ARM_WIDE_ITEMS`] items of one
//!   tag each: 1,000 arms, 200 statements.
//! * **100 bound parameters.** An item binds 0 to 3 parameters whatever its
//!   width, so no query can cross this wall through the adapter. What is driven
//!   instead is the shape that *used* to cross it: items of [`TYPES_PER_ITEM`]
//!   event types each, which bound one parameter per type and now bind one.
//! * **An expression depth of 100.** The old tag chain nested one level per tag
//!   and was refused at 46 tags in one item. Crossed by one item of 46 tags and
//!   one of the declared 1,024.
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
//! `DurableObjectState` shape, opened with `workerd`'s four statement limits and
//! none of its other properties — which is `kb-decision-0023`'s recorded finding
//! and not this target's to fix. `harness/workerd` runs the conformance rules
//! inside `workerd` itself.

#![cfg(target_arch = "wasm32")]

use core::future::poll_fn;

use futures_core::Stream;
use happenstance_cloudflare::event_store::CloudflareEventStore;
use happenstance_cloudflare::host::DurableObjectHost;
use happenstance_cloudflare::sql_storage::{SqlStorage, SqlValue};
use happenstance_core::{
    AppendCondition, AppendError, Event, EventStore, Query, QueryItem, ReadOptions, Tag, Tags,
};
use wasm_bindgen_test::wasm_bindgen_test;

/// Items enough to pass the old 500-term compound-`SELECT` default twice
/// over, and `workerd`'s 5 two hundred times.
///
/// One tag each, so the query is 1,000 arms of one parameter apiece — the arm
/// axis binding, the parameter axis idle.
const ARM_WIDE_ITEMS: usize = 1_000;

/// Items in the type-wide queries: two full chunks and one more, so the plan
/// is three statements and the last item is alone in the third.
///
/// The read and both parameter-axis guards use it. Under the old rendering
/// those guards were 400 items of 82 types, 32,800 parameters split on the
/// parameter axis. Each item now binds one parameter, so with the shipped
/// constants no query crosses the parameter axis end to end; its only
/// coverage is the host partition tests in `src/query_sql.rs`. What these
/// guards keep is a type-wide guard cut into several statements by the arm
/// axis (the owner's call on review finding F1, 2026-10-05).
const TYPE_WIDE_ITEMS: usize = CloudflareEventStore::MAX_QUERY_ARMS_PER_STATEMENT * 2 + 1;

/// Event types per item of the type-wide queries.
///
/// Under the rendering this target was written against, `400 * 82 = 32,800`
/// parameters cleared `SQLITE_MAX_VARIABLE_NUMBER`'s 32,766 by 34. An item now
/// binds its types as one JSON array, so 82 types is one parameter.
const TYPES_PER_ITEM: usize = 82;

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

/// An event of the type item `index` of a type-wide query names first.
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

/// `items` items of `TYPES_PER_ITEM` distinct event types each.
///
/// The type sets are disjoint across items, so an event of one type is matched
/// by exactly one item and "which chunk answered" is observable.
fn type_wide_query(items: usize) -> Query {
    Query::from_items((0..items).map(|index| {
        let base = index * TYPES_PER_ITEM;
        QueryItem::of_types((base..base + TYPES_PER_ITEM).map(|n| format!("T{n}")))
            .expect("an item constraining types is constructible")
    }))
    .expect("a non-empty item list is a query")
}

/// `TYPE_WIDE_ITEMS` type-wide items: the parameter-axis guards' query.
fn parameter_wide_query() -> Query {
    type_wide_query(TYPE_WIDE_ITEMS)
}

/// The tags `t:0` to `t:<width - 1>`.
fn tag_set(width: usize) -> Tags {
    (0..width)
        .map(|n| Tag::new(format!("t:{n}")).expect("a valid tag"))
        .collect()
}

// ---------------------------------------------------------------------------
// The controls: this host's walls, and the widest statement sitting inside them
// ---------------------------------------------------------------------------

/// Runs `statement` and drains it, so a refusal at step time is seen too.
fn ran(sql: &SqlStorage, statement: &str, bindings: &[SqlValue]) -> Result<(), String> {
    let mut cursor = sql.exec(statement, bindings).map_err(|e| e.to_string())?;
    while let Some(row) = cursor.next_row() {
        row.map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// The four statement-shape walls `workerd` puts on a Durable Object's SQLite,
/// enforced by this host too.
///
/// Measured inside `workerd` by `harness/workerd/src/probe.rs`: 5 compound
/// terms, 100 bound parameters, 100,000-byte statements and an expression depth
/// of 100. Each is asserted on both sides, so a host that refused everything
/// would fail the accepted half.
///
/// It replaces `the_partition_budget_sits_inside_this_hosts_limits`, which
/// asserted a 1,000-term compound refused and a 30,000-parameter statement
/// accepted. That second half went red on `macos-latest` in an earlier form,
/// because `SQLITE_MAX_VARIABLE_NUMBER` is a compile-time option of whichever
/// SQLite the host was built against. Setting the limits at open time is what
/// makes them the same on every host, and the shim refuses to open at all on a
/// Node that ignores them.
#[wasm_bindgen_test]
fn the_host_enforces_workerds_statement_limits() {
    let sql = DurableObjectHost::new().storage();

    let compound = |terms: usize| vec!["SELECT 1"; terms].join(" UNION ALL ");
    assert_eq!(ran(&sql, &compound(5), &[]), Ok(()), "five compound terms");
    assert!(ran(&sql, &compound(6), &[]).is_err(), "six compound terms");

    let variables = |count: usize| -> (String, Vec<SqlValue>) {
        let bindings = (0..count)
            .map(|n| SqlValue::Integer(i64::try_from(n).expect("a small count fits")))
            .collect();
        (format!("SELECT {}", vec!["?"; count].join(",")), bindings)
    };
    let (statement, bindings) = variables(100);
    assert_eq!(
        ran(&sql, &statement, &bindings),
        Ok(()),
        "100 bound parameters"
    );
    let (statement, bindings) = variables(101);
    assert!(
        ran(&sql, &statement, &bindings).is_err(),
        "101 bound parameters"
    );

    let padded = |bytes: usize| format!("SELECT 1 --{}", "x".repeat(bytes - 11));
    assert_eq!(
        ran(&sql, &padded(100_000), &[]),
        Ok(()),
        "a 100,000-byte statement"
    );
    assert!(
        ran(&sql, &padded(100_001), &[]).is_err(),
        "a 100,001-byte statement"
    );

    let depth = |terms: usize| format!("SELECT {}", vec!["1"; terms].join("+"));
    assert_eq!(ran(&sql, &depth(100), &[]), Ok(()), "100 terms of 1+1+...");
    assert!(ran(&sql, &depth(101), &[]).is_err(), "101 terms of 1+1+...");
}

/// The widest statement the adapter emits, executed raw and accepted.
///
/// Five arms of the widest shape — 1,024 tags and 82 types each, three
/// parameters apiece — inside the read wrapper with every bound set: 20
/// parameters and constant text. The direction that can break the store is a
/// host that refuses this, because then every wide statement the partition
/// plans is one it rejects, and for a guard inside the append turn.
///
/// The text is a hand copy of `render_chunk`'s wrapper and `Arm::render`'s
/// widest arm, because neither is reachable from an integration target. If
/// either gains a predicate or a binding, this control does not notice; the
/// host test `the_widest_statement_fits_workerds_text_wall` in
/// `src/event_store.rs` pins the real renderer's width and parameter count.
#[wasm_bindgen_test]
fn a_statement_of_the_widest_chunk_is_accepted_here() {
    let host = DurableObjectHost::new();
    let sql: SqlStorage = host.storage();
    CloudflareEventStore::new(sql.clone())
        .migrate()
        .expect("the schema applies");

    let json = |values: Vec<String>| {
        let quoted: Vec<String> = values.iter().map(|value| format!("\"{value}\"")).collect();
        format!("[{}]", quoted.join(","))
    };
    let arm = "SELECT position FROM event_tag WHERE tag IN (SELECT value FROM json_each(?)) \
               AND event_type IN (SELECT value FROM json_each(?)) \
               GROUP BY position HAVING count(*) = ?";
    let arms = CloudflareEventStore::MAX_QUERY_ARMS_PER_STATEMENT;
    let mut bindings = Vec::new();
    for item in 0..arms {
        bindings.push(SqlValue::Text(json(
            (0..1_024).map(|n| format!("k{item}:v{n}")).collect(),
        )));
        bindings.push(SqlValue::Text(json(
            (0..TYPES_PER_ITEM).map(|n| format!("T{n}")).collect(),
        )));
        bindings.push(SqlValue::Integer(1_024));
    }
    bindings.extend([2, 9, 9, 4, 3].map(SqlValue::Integer));
    let statement = format!(
        "SELECT position FROM event WHERE position IN ({}) \
         AND origin_position IS NOT NULL AND position <= ? AND position >= ? \
         AND position <= ? AND position > ? ORDER BY position ASC LIMIT ?",
        vec![arm; arms].join(" UNION ")
    );
    assert_eq!(
        ran(&sql, &statement, &bindings),
        Ok(()),
        "the widest chunk, {} bytes and {} parameters, must be accepted",
        statement.len(),
        bindings.len()
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

/// The shape that crossed the parameter wall, through `EventStore::read`.
///
/// Each item names 82 types and binds them as one parameter, so the plan is
/// cut by the arm axis alone: three statements, the last holding one item. The
/// first and last items each match one event, in the first and last chunks.
#[wasm_bindgen_test]
async fn a_type_wide_query_binds_one_parameter_per_item_and_is_served() {
    let store = store();
    store
        .append(&[typed(0), typed(TYPE_WIDE_ITEMS - 1)], None)
        .await
        .expect("the seed lands");

    let query = type_wide_query(TYPE_WIDE_ITEMS);
    assert_eq!(
        CloudflareEventStore::planned_statement_count(&query),
        3,
        "a type-wide item binds one parameter, so only the arm axis cuts the plan"
    );

    let seen = positions(&store, &query).await;
    assert_eq!(
        seen.len(),
        2,
        "the first and last items of the query each match one event, and they \
         are in different chunks"
    );
}

/// One item past the expression depth the old tag chain hit, and one of the
/// declared 1,024 tags, read and guarded.
///
/// The old rendering nested one `AND position IN (…)` per tag and was refused
/// by `workerd` at 46. The event carries every tag the item names, so a read
/// must return it and a guard must be violated by it.
#[wasm_bindgen_test]
async fn a_tag_wide_item_past_the_old_expression_depth_wall_is_served() {
    for width in [46, 1_024] {
        let store = store();
        let landed = store
            .append(
                &[Event::new("Subject", &b"{}"[..])
                    .expect("a valid event type")
                    .with_tags(tag_set(width))],
                None,
            )
            .await
            .expect("the seed lands");
        let query =
            Query::from_item(QueryItem::tagged(tag_set(width)).expect("a valid query item"));

        assert_eq!(
            positions(&store, &query).await,
            [landed.get()],
            "an item of {width} tags matches the event carrying them"
        );
        let outcome = store
            .append(
                &[Event::new("Decided", &b"{}"[..]).expect("a valid event type")],
                Some(&AppendCondition::new(query)),
            )
            .await;
        assert!(
            matches!(outcome, Err(AppendError::ConditionViolated(_))),
            "a guard of {width} tags is answered, and violated: {outcome:?}"
        );
    }
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
///
/// Since ADR-0079 no guard crosses the parameter wall (see
/// [`TYPE_WIDE_ITEMS`]): this one is 11 type-wide items, three statements cut
/// on the arm axis. What it still rejects is a guard sent unpartitioned, which
/// the shim refuses as `workerd` does, inside the append turn.
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
///
/// Since ADR-0079 its plan is cut on the arm axis (see [`TYPE_WIDE_ITEMS`]),
/// and the only matching event is named by the last item, alone in the third
/// statement.
#[wasm_bindgen_test]
async fn a_parameter_wide_guard_answers_from_every_chunk_not_the_first() {
    let store = store();
    store
        .append(&[typed(TYPE_WIDE_ITEMS - 1)], None)
        .await
        .expect("the seed lands");

    assert!(
        CloudflareEventStore::planned_statement_count(&parameter_wide_query()) > 1,
        "at one statement a guard that folded only its first chunk would pass"
    );
    let outcome = store
        .append(
            &[Event::new("Decided", &b"{}"[..]).expect("a valid event type")],
            Some(&AppendCondition::new(parameter_wide_query())),
        )
        .await;

    assert!(
        matches!(outcome, Err(AppendError::ConditionViolated(_))),
        "the matching event is named only by the last item, which the partition \
         puts in the last chunk: {outcome:?}"
    );
}
