//! The conformance suite, run against a live pinned Postgres.
//!
//! # How this target is gated, and why it is gated *this* way
//!
//! Every test here needs a Docker daemon. The default gate must not, because
//! `.redkiln/config.yaml` wires `cargo xtask affected` and `cargo xtask ci
//! --fast` as the verify grains for **every other project in this initiative**,
//! and a step that needs a container breaks all of them.
//!
//! Three gating mechanisms were on the table and only one works.
//!
//! **`required-features` does not.** The default gate runs `cargo test --locked
//! --workspace --all-features` and `cargo clippy --workspace --all-targets
//! --all-features`, and `--all-features` *enables* the required feature — so the
//! target would build **and run**, against a server that is not there. It is
//! listed as admissible in this project's architecture brief; it is not, and
//! saying so here is cheaper than the next reader rediscovering it.
//!
//! **An environment-variable read that returns early does not, on its own.** A
//! "no server, pass quietly" path is indistinguishable in CI output from a rule
//! that passed, which is CF-18's argument about omitted skips applied one layer
//! out.
//!
//! **`#[ignore]` does.** It composes with `--all-features`, it keeps the target
//! compiled and linted by the default gate — which is what stops this mount from
//! rotting — and it makes the gated tests appear in the default run as
//! `ignored` rather than vanishing from the count. An absent target and a
//! skipped one are different observations and only one of them is acceptable.
//!
//! The macro cannot be told to add `#[ignore]`, but it does not have to be: the
//! emitter is a **parameter**, and [`emit_ignored_tokio`] below is
//! `__emit_tokio` with one attribute added. That is a whole-invocation gate — it
//! marks every generated test — and is emphatically **not** a `#[cfg]` hiding a
//! rule out of a macro's expansion, which DR-5 forbids for CF-18's reason: a
//! rule silently omitted is indistinguishable from a rule that passed.
//!
//! # Running it
//!
//! ```console
//! cargo test -p happenstance-postgres --all-features -- --ignored --list
//! cargo test -p happenstance-postgres --all-features -- --ignored --show-output
//! ```
//!
//! `--list` first, so "no rule is absent from the run" is proven rather than
//! assumed. `--show-output` because a declined capability's reason is printed
//! only under it, and an unread reason is a trade nobody recorded.

#![cfg(not(target_arch = "wasm32"))]
#![allow(clippy::unwrap_used)]

mod support;

use happenstance_postgres::event_store::PostgresEventStore;
use happenstance_postgres::sqlx::{Row, query};
use happenstance_testkit::concurrency::CONTENDERS;
use happenstance_testkit::{Capability, Fixture};
use support::PostgresFixture;

/// `__emit_tokio`, plus `#[ignore]`.
///
/// The reason string is not decoration: `cargo test -- --ignored --list` prints
/// it, so the one command an adapter author runs to find out what is gated also
/// tells them how to ungate it.
macro_rules! emit_ignored_tokio {
    ($($name:ident),* $(,)?) => {
        $(
            #[tokio::test]
            #[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container)"]
            async fn $name() {
                happenstance_testkit::rules::$name(__conformance_fixture)
                    .await
                    .report(::core::stringify!($name));
            }
        )*
    };
}
pub(crate) use emit_ignored_tokio;

happenstance_testkit::event_store_conformance!(
    mod_name = dcb_conformance,
    emit = crate::emit_ignored_tokio,
    fixture = PostgresFixture::new()
);

// ---------------------------------------------------------------------------
// This story's own tests.
//
// `append`, `head` and `contains_event_id` are still `todo!()`, so every rule in
// the expansion above would panic if run. What is provable now is the schema,
// the isolation and the fixture's own shape — and those are proved through the
// public `PostgresEventStore::pool()` accessor and raw SQL, which is the one
// channel available before the adapter has a body.
// ---------------------------------------------------------------------------

/// The columns migration 1 is allowed to have, and their shapes.
///
/// Held as data rather than as a sequence of assertions so that the "exactly
/// these" check below can be a set comparison: a ninth column added in passing
/// must fail a test rather than pass review.
const SETTLED_COLUMNS: &[(&str, &str, bool)] = &[
    // (name, data_type, is_nullable)
    ("position", "bigint", false),
    ("event_type", "text", false),
    ("data", "bytea", false),
    ("metadata", "bytea", true),
    ("tags", "ARRAY", false),
    ("origin_store", "bytea", true),
    ("origin_position", "bigint", true),
    ("recorded_at", "bigint", false),
    // The visibility mechanism, added by `postgres-append-and-frontier-head` on
    // ADR-0013's prior measurement. It was deliberately absent while the schema
    // story owned this file, so that a schema could not answer the mechanism
    // question by accident; it is deliberately present now.
    ("xact_id", "xid8", false),
];

#[tokio::test]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container)"]
async fn migration_1_creates_the_settled_column_set() {
    let fixture = PostgresFixture::new();
    let store = fixture.connect().await;

    let rows = query(
        "SELECT column_name, data_type, is_nullable \
         FROM information_schema.columns \
         WHERE table_schema = $1 AND table_name = 'event' \
         ORDER BY ordinal_position",
    )
    .bind(fixture.schema())
    .fetch_all(store.pool())
    .await
    .expect("the event table should exist after migration 1");

    let found: Vec<(String, String, bool)> = rows
        .iter()
        .map(|row| {
            (
                row.get::<String, _>("column_name"),
                row.get::<String, _>("data_type"),
                row.get::<String, _>("is_nullable") == "YES",
            )
        })
        .collect();

    let expected: Vec<(String, String, bool)> = SETTLED_COLUMNS
        .iter()
        .map(|(name, ty, nullable)| ((*name).to_owned(), (*ty).to_owned(), *nullable))
        .collect();

    assert_eq!(
        found, expected,
        "migration 1's column set drifted from the settled eight. \
         A ninth column is not a formality: it is where ADR-0024 gets answered by accident."
    );

    // Both indexes the schema promises, read back rather than assumed.
    let indexes =
        query("SELECT indexname FROM pg_indexes WHERE schemaname = $1 AND tablename = 'event'")
            .bind(fixture.schema())
            .fetch_all(store.pool())
            .await
            .expect("pg_indexes should answer");

    let names: Vec<String> = indexes
        .iter()
        .map(|row| row.get::<String, _>("indexname"))
        .collect();

    for expected in ["event_tags_idx", "event_type_idx", "event_origin_idx"] {
        assert!(
            names.iter().any(|name| name == expected),
            "index `{expected}` is missing; found {names:?}"
        );
    }
}

/// The guard that keeps this story from settling a later story's question.
///
/// The no-server half of this lives in `src/migration.rs` as a text scan. This
/// is the half a real server can disagree with: a `bigserial` and a `bigint`
/// with a sequence default are the same thing to Postgres and different things
/// to a reader of the SQL.
#[tokio::test]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container)"]
async fn migration_1_does_not_preempt_adr_0024() {
    let fixture = PostgresFixture::new();
    let store = fixture.connect().await;

    let row = query(
        "SELECT column_default, is_identity \
         FROM information_schema.columns \
         WHERE table_schema = $1 AND table_name = 'event' AND column_name = 'position'",
    )
    .bind(fixture.schema())
    .fetch_one(store.pool())
    .await
    .expect("the position column should exist");

    assert!(
        row.get::<Option<String>, _>("column_default").is_none(),
        "`position` has acquired a column default. A default is `nextval()`, and \
         `nextval()` allocating outside the transaction is where ES-10 is lost — \
         which is the question ADR-0024 exists to answer deliberately."
    );
    assert_eq!(
        row.get::<String, _>("is_identity"),
        "NO",
        "`position` has become an identity column, which is a sequence by another name"
    );
}

/// One fixture instance is one isolated backing store.
///
/// The story-local stand-in for
/// `rules::two_fixture_instances_observe_none_of_each_others_appends`, which the
/// slice-mate turns green through a real `append`. `MemoryFixture::sharing` is
/// the in-tree implementation that deliberately fails this, and it is what this
/// test exists to reject.
#[tokio::test]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container)"]
async fn two_fixture_instances_are_two_backing_stores() {
    let left = PostgresFixture::new();
    let right = PostgresFixture::new();
    assert_ne!(
        left.schema(),
        right.schema(),
        "two instances share a schema, so they are one backing store wearing two names"
    );

    let left_store = left.connect().await;
    let right_store = right.connect().await;

    insert_bare_row(&left_store, 1).await;

    assert_eq!(count_events(&left_store).await, 1);
    assert_eq!(
        count_events(&right_store).await,
        0,
        "the second instance can see the first's row, so the isolation scheme is a lie"
    );
}

/// Two handles from one instance address one backing store.
///
/// `SECOND_HANDLE` is the only MUST among the capabilities. This is its runtime
/// half; the compile-time half is the constant assertion below.
#[tokio::test]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container)"]
async fn two_handles_from_one_instance_share_a_backing_store() {
    let fixture = PostgresFixture::new();
    let first = fixture.connect().await;
    let second = fixture.connect().await;

    insert_bare_row(&first, 1).await;

    assert_eq!(
        count_events(&second).await,
        1,
        "the second handle cannot see the first's write, so they are not one store"
    );
}

/// `REOPEN` is claimed supported, so it is exercised rather than asserted.
///
/// A capability claimed and never run is a claim, not a fact.
#[tokio::test]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container)"]
async fn a_row_survives_a_reopen() {
    let fixture = PostgresFixture::new();
    {
        let store = fixture.connect().await;
        insert_bare_row(&store, 1).await;
    }

    fixture.reopen().await;

    let reopened = fixture.connect().await;
    assert_eq!(
        count_events(&reopened).await,
        1,
        "the row did not survive a reopen, so `REOPEN: SUPPORTED` is not true"
    );
}

/// One instance hands out `CONTENDERS` simultaneous live handles.
///
/// The failure this rejects is not a red test, it is a **hang**: a pool sized
/// below `CONTENDERS` deadlocks rather than failing and CF-33 says there is no
/// watchdog to tell the two apart. Every handle is held open at once and each
/// executes a statement, so a pool that cannot supply them all times out here
/// rather than inside the concurrency family, where the cause would be much
/// further from the symptom.
#[tokio::test(flavor = "multi_thread")]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container)"]
async fn contenders_handles_open_concurrently_without_deadlock() {
    let fixture = PostgresFixture::new();

    let mut stores = Vec::with_capacity(CONTENDERS);
    for _ in 0..CONTENDERS {
        stores.push(fixture.connect().await);
    }

    // Held simultaneously, and all made to do work simultaneously. Opening then
    // dropping each in turn would pass with a pool of one.
    let mut handles = Vec::with_capacity(CONTENDERS);
    for store in &stores {
        let pool = store.pool().clone();
        handles.push(tokio::spawn(async move {
            query("SELECT 1")
                .execute(&pool)
                .await
                .expect("a live handle should answer a trivial query");
        }));
    }
    for handle in handles {
        handle.await.expect("no contender task should panic");
    }

    assert_eq!(stores.len(), CONTENDERS);
}

/// A fixture that answers only the two required capabilities.
///
/// Its only purpose is to *name* the trait's provided defaults so a test can
/// compare against them. There is no other way to tell "declined deliberately"
/// from "never considered": both are the same three tokens of Rust at the impl
/// site, which is the whole hazard `MID_BATCH_FAULT` carries.
struct Defaulted;

impl Fixture for Defaulted {
    type Store = PostgresEventStore;
    const SECOND_HANDLE: Capability = Capability::SUPPORTED;
    const REOPEN: Capability = Capability::SUPPORTED;
    async fn connect(&self) -> Self::Store {
        unreachable!("`Defaulted` exists to be read, never constructed")
    }
}

/// Every capability constant is an answer, and none is the trait's default.
///
/// This runs with no server, deliberately: it is the belt to
/// `Capability::declined("")`'s suspender, which fires at **codegen** for an
/// associated const — so `cargo build` and `cargo test` catch an empty reason
/// and `cargo clippy` does not. A green clippy proves nothing about these.
#[test]
fn capability_constants_are_answered_not_defaulted() {
    assert!(PostgresFixture::SECOND_HANDLE.is_supported());
    assert!(PostgresFixture::REOPEN.is_supported());

    // `MID_BATCH_FAULT` was declined while `append` was `todo!()` and is
    // supported now that it is not. Both are answers; the failure mode this test
    // exists for is the third possibility, which is the trait's silent default
    // on a store that could have co-operated.
    assert!(
        PostgresFixture::MID_BATCH_FAULT.is_supported(),
        "Postgres can fail between two rows of one batch -- an AFTER INSERT trigger          raising on the nth row does it -- so declining is not an honest answer          once `append` exists to fault"
    );

    // Not left at the trait's provided default. Comparing against the default
    // explicitly is the only way to tell an answer from an omission, because the
    // two are the same three tokens of Rust at the impl site.
    assert_ne!(
        PostgresFixture::MID_BATCH_FAULT.reason(),
        Defaulted::MID_BATCH_FAULT.reason(),
        "MID_BATCH_FAULT is the trait's silent default on a store that could co-operate"
    );

    // The ceilings are facts mirrored from the adapter, not literals restated.
    assert_eq!(
        PostgresFixture::MAX_EVENT_DATA_LEN,
        Some(PostgresEventStore::MAX_EVENT_DATA_LEN)
    );
    assert_eq!(
        PostgresFixture::MAX_TAGS_PER_EVENT,
        Some(PostgresEventStore::MAX_TAGS_PER_EVENT)
    );
    assert_eq!(
        PostgresFixture::MAX_EVENTS_PER_BATCH,
        Some(PostgresEventStore::MAX_EVENTS_PER_BATCH)
    );
}

/// The stated ceilings clear the floors every store must clear.
///
/// VT-21, VT-22 and VT-24 make 65,536 bytes, 64 tags and 128 events per batch
/// floors rather than suggestions. A ceiling below one of them is legal and
/// fails `store_accepts_the_guaranteed_minimum_*` — correctly. Catching it here
/// costs nothing and does not need a server.
///
/// Written as `const` blocks rather than runtime assertions, which clippy asked
/// for and which is the stronger form: every operand is a constant, so a ceiling
/// dropped below a floor stops the crate **compiling** rather than reddening a
/// test somebody has to run. That is atom 61's preference and it costs nothing
/// here.
#[test]
fn the_stated_ceilings_clear_the_specification_floors() {
    const {
        assert!(
            PostgresEventStore::MAX_EVENT_DATA_LEN
                >= happenstance_core::MIN_SUPPORTED_EVENT_DATA_LEN
        );
        assert!(
            PostgresEventStore::MAX_TAGS_PER_EVENT
                >= happenstance_core::MIN_SUPPORTED_TAGS_PER_EVENT
        );
        assert!(
            PostgresEventStore::MAX_EVENTS_PER_BATCH
                >= happenstance_core::MIN_SUPPORTED_EVENTS_PER_BATCH
        );
    }
}

// --- helpers ---------------------------------------------------------------

/// Writes one row directly, bypassing `append`, which is still `todo!()`.
///
/// Positions are supplied by the caller because the schema deliberately has no
/// default and the mechanism that will supply them is a later story's. This is
/// not a literal position value in the sense CF-6 forbids — nothing here asserts
/// *on* a position, it only needs the primary key to differ.
async fn insert_bare_row(store: &PostgresEventStore, position: i64) {
    query(
        "INSERT INTO event (position, event_type, data, metadata, tags, recorded_at) \
         VALUES ($1, 'probe', '\\x00'::bytea, NULL, ARRAY[]::text[], 0)",
    )
    .bind(position)
    .execute(store.pool())
    .await
    .expect("a bare insert should succeed against migration 1's table");
}

/// How many rows the `event` table holds, through this handle.
async fn count_events(store: &PostgresEventStore) -> i64 {
    query("SELECT count(*) AS n FROM event")
        .fetch_one(store.pool())
        .await
        .expect("counting rows should succeed")
        .get::<i64, _>("n")
}
