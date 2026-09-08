//! The conformance suite, run against a live Neon endpoint.
//!
//! # How this target is gated, and why it is gated *this* way
//!
//! Every test here needs `NEON_CONNECTION` and a reachable endpoint. The default
//! gate must not, because `.redkiln/config.yaml` wires `cargo xtask affected` and
//! `cargo xtask ci --fast` as the verify grains for every other project, and a
//! step that needs a secret breaks all of them.
//!
//! Three gating mechanisms were on the table and only one works.
//!
//! **`required-features` does not.** The default gate runs
//! `cargo test --locked --workspace --all-features`, and `--all-features`
//! *enables* the required feature — so the target would build **and run**,
//! against an endpoint that is not configured.
//!
//! **An environment-variable read that returns early does not, on its own.** A
//! "no secret, pass quietly" path is indistinguishable in CI output from a rule
//! that passed, which is CF-18's argument about omitted skips one layer out. This
//! target therefore has no such path: with `--ignored` and no secret, the fixture
//! panics naming the environment. The green-gate-without-the-secret property is
//! bought by `#[ignore]` and by nothing else, which is the honest division.
//!
//! **`#[ignore]` does.** It composes with `--all-features`, it keeps the target
//! compiled and linted by the default gate — which is what stops this mount from
//! rotting — and it makes the gated tests appear in the default run as `ignored`
//! rather than vanishing from the count. An absent target and a skipped one are
//! different observations and only one of them is acceptable.
//!
//! The macro cannot be told to add `#[ignore]`, but it does not have to be: the
//! emitter is a **parameter**, and the macros below are `__emit_tokio` with one
//! attribute added. That is a whole-invocation gate — it marks every generated
//! test — and is emphatically **not** a `#[cfg]` hiding a rule out of a macro's
//! expansion, which DR-5 forbids for CF-18's reason.
//!
//! # Running it
//!
//! ```console
//! cargo test -p happenstance-neon --all-features --test neon_conformance -- --ignored --list
//! cargo test -p happenstance-neon --all-features --test neon_conformance -- --ignored --show-output
//! ```
//!
//! `--list` first, so "no rule is absent from the run" is proven rather than
//! assumed. `--show-output` because a declined capability's reason is printed only
//! under it, and an unread reason is a trade nobody recorded.
//!
//! # What this adapter reports rather than passes
//!
//! **Nothing, in this target.** All four of `Fixture`'s declinable
//! capabilities — `SECOND_HANDLE`, `REOPEN`, `MID_BATCH_FAULT`, `READ_FAULT` —
//! are declared and armed for real, and all three ceilings are stated, so every
//! rule in the event-store, concurrency and model families runs. The three
//! reported skips this adapter does carry are all in the projection family; see
//! `tests/neon_projection.rs`, which lists them.

#![cfg(not(target_arch = "wasm32"))]
#![allow(clippy::unwrap_used)]

mod support;

use futures_core::Stream;
use happenstance_core::{Event, EventStore, Query, ReadOptions};
use happenstance_neon::event_store::NeonEventStore;
use happenstance_neon::{SqlRequest, SqlStatement, SqlTransport};
use happenstance_testkit::concurrency::CONTENDERS;
use happenstance_testkit::{Capability, Fixture};
use support::NeonFixture;
use support::transport::HyperTransport;

/// `__emit_tokio`, plus `#[ignore]`.
///
/// The reason string is not decoration: `cargo test -- --ignored --list` prints
/// it, so the one command an adapter author runs to find out what is gated also
/// tells them how to ungate it.
macro_rules! emit_ignored_tokio {
    ($($name:ident),* $(,)?) => {
        $(
            #[tokio::test]
            #[ignore = "needs a live Neon endpoint; set NEON_CONNECTION and run with `-- --ignored`"]
            async fn $name() {
                happenstance_testkit::rules::$name(__conformance_fixture)
                    .await
                    .report(::core::stringify!($name));
            }
        )*
    };
}
pub(crate) use emit_ignored_tokio;

/// The concurrency family's emitter, plus `#[ignore]`.
///
/// A separate macro rather than a parameter on the one above, because the
/// families differ in two ways that matter: the rules live under
/// `concurrency::rules`, and each test needs
/// `#[tokio::test(flavor = "multi_thread")]`. A current-thread runtime turns
/// `CONTENDERS` contenders from a race into a queue, and a queue passes every
/// rule in this family for the wrong reason — and here it would do worse than
/// that, because the transport dispatches every round trip onto the captured
/// runtime and a single-threaded one would serialise the whole family.
macro_rules! emit_ignored_concurrency_tokio {
    ($($name:ident),* $(,)?) => {
        $(
            #[tokio::test(flavor = "multi_thread")]
            #[ignore = "needs a live Neon endpoint; set NEON_CONNECTION and run with `-- --ignored`"]
            async fn $name() {
                happenstance_testkit::concurrency::rules::$name(__conformance_fixture)
                    .await
                    .report(::core::stringify!($name));
            }
        )*
    };
}
pub(crate) use emit_ignored_concurrency_tokio;

/// The model family's emitter, plus `#[ignore]`.
macro_rules! emit_ignored_model_tokio {
    ($($name:ident),* $(,)?) => {
        $(
            #[tokio::test]
            #[ignore = "needs a live Neon endpoint; set NEON_CONNECTION and run with `-- --ignored`"]
            async fn $name() {
                happenstance_testkit::model::rules::$name(__conformance_fixture)
                    .await
                    .report(::core::stringify!($name));
            }
        )*
    };
}
pub(crate) use emit_ignored_model_tokio;

happenstance_testkit::event_store_conformance!(
    mod_name = dcb_conformance,
    emit = crate::emit_ignored_tokio,
    fixture = NeonFixture::new()
);

// The concurrency family. This is the bar the crate's whole append design exists
// to clear: sixty-four contenders on one boundary, over a transport with no
// session, no interactive transaction and nothing serialising the writers except
// the endpoint's own `SERIALIZABLE` batch.
//
// `NeonFixture` is named at the call site rather than returned as an opaque
// `impl Fixture`, because an opaque type carries only the bounds written on it
// and `F::Store: Send` would be unprovable — which is the one line
// `ConcurrentFixture` exists for.
happenstance_testkit::event_store_concurrency_conformance!(
    mod_name = dcb_concurrency_conformance,
    emit = crate::emit_ignored_concurrency_tokio,
    fixture = NeonFixture::new()
);

// The model family: the suite's rules checked against a reference model under
// `proptest`, which is why the testkit dev-dependency names that feature.
happenstance_testkit::event_store_model_conformance!(
    mod_name = dcb_model_conformance,
    emit = crate::emit_ignored_model_tokio,
    fixture = NeonFixture::new()
);

// ---------------------------------------------------------------------------
// This adapter's own tests: the things no borrowed rule can see.
// ---------------------------------------------------------------------------

/// The columns migration 1 is allowed to have, and their shapes.
///
/// Held as data rather than as a sequence of assertions so that the "exactly
/// these" check below can be a set comparison: a tenth column added in passing
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
    ("xact_id", "xid8", false),
];

#[tokio::test]
#[ignore = "needs a live Neon endpoint; set NEON_CONNECTION and run with `-- --ignored`"]
async fn migration_1_creates_the_settled_column_set() {
    let fixture = NeonFixture::new();
    let _store = fixture.connect().await;

    let rows = query(
        "SELECT column_name, data_type, is_nullable FROM information_schema.columns \
         WHERE table_schema = $1 AND table_name = 'event' ORDER BY ordinal_position",
        vec![serde_json::Value::String(fixture.schema().to_owned())],
    )
    .await;

    let found: Vec<(String, String, bool)> = rows
        .iter()
        .map(|row| {
            (
                text(row, "column_name"),
                text(row, "data_type"),
                text(row, "is_nullable") == "YES",
            )
        })
        .collect();
    let expected: Vec<(String, String, bool)> = SETTLED_COLUMNS
        .iter()
        .map(|(name, ty, nullable)| ((*name).to_owned(), (*ty).to_owned(), *nullable))
        .collect();

    assert_eq!(
        found, expected,
        "migration 1's column set drifted from the settled nine. A tenth column is not \
         a formality: it is where the visibility mechanism gets answered by accident."
    );

    let indexes = query(
        "SELECT indexname FROM pg_indexes WHERE schemaname = $1 AND tablename = 'event'",
        vec![serde_json::Value::String(fixture.schema().to_owned())],
    )
    .await;
    let names: Vec<String> = indexes.iter().map(|row| text(row, "indexname")).collect();
    for expected in [
        "event_tags_idx",
        "event_type_idx",
        "event_origin_idx",
        "event_xact_idx",
    ] {
        assert!(
            names.iter().any(|name| name == expected),
            "index `{expected}` is missing; found {names:?}"
        );
    }
}

/// `position` is not `bigserial`, checked by a server rather than by a grep.
///
/// The no-server half of this lives in `src/migration.rs` as a text scan. This is
/// the half a real server can disagree with: a `bigserial` and a `bigint` with a
/// sequence default are the same thing to Postgres and different things to a
/// reader of the SQL.
#[tokio::test]
#[ignore = "needs a live Neon endpoint; set NEON_CONNECTION and run with `-- --ignored`"]
async fn migration_1_leaves_the_allocation_site_explicit() {
    let fixture = NeonFixture::new();
    let _store = fixture.connect().await;

    let rows = query(
        "SELECT coalesce(column_default, '') AS column_default, is_identity \
         FROM information_schema.columns \
         WHERE table_schema = $1 AND table_name = 'event' AND column_name = 'position'",
        vec![serde_json::Value::String(fixture.schema().to_owned())],
    )
    .await;

    assert_eq!(rows.len(), 1, "the position column should exist");
    assert_eq!(
        text(&rows[0], "column_default"),
        "",
        "`position` has acquired a column default. A default is `nextval()`, and \
         `nextval()` allocating outside the transaction is where ES-10 is lost — which \
         is the question the frontier predicate exists to answer deliberately."
    );
    assert_eq!(
        text(&rows[0], "is_identity"),
        "NO",
        "`position` has become an identity column, which is a sequence by another name"
    );
}

/// One fixture instance is one isolated backing store.
///
/// The whole of this adapter's isolation is schema qualification, so this is the
/// test that would catch the day one statement stopped carrying its schema.
#[tokio::test]
#[ignore = "needs a live Neon endpoint; set NEON_CONNECTION and run with `-- --ignored`"]
async fn two_fixture_instances_are_two_backing_stores() {
    let left = NeonFixture::new();
    let right = NeonFixture::new();
    assert_ne!(
        left.schema(),
        right.schema(),
        "two instances share a schema, so they are one backing store wearing two names"
    );

    let left_store = left.connect().await;
    let _right_store = right.connect().await;
    left_store
        .append(&[probe_event()], None)
        .await
        .expect("an unconditional append should succeed");

    assert_eq!(count(&left).await, 1);
    assert_eq!(
        count(&right).await,
        0,
        "the second instance can see the first's row, so the isolation scheme is a lie"
    );
}

/// Two handles from one instance address one backing store.
#[tokio::test]
#[ignore = "needs a live Neon endpoint; set NEON_CONNECTION and run with `-- --ignored`"]
async fn two_handles_from_one_instance_share_a_backing_store() {
    let fixture = NeonFixture::new();
    let first = fixture.connect().await;
    let second = fixture.connect().await;

    first
        .append(&[probe_event()], None)
        .await
        .expect("an unconditional append should succeed");

    let read = happenstance_core::read_decision_model(&second, &Query::all())
        .await
        .expect("the second handle should read");
    assert_eq!(
        read.0.len(),
        1,
        "the second handle cannot see the first's write, so they are not one store"
    );
}

/// `READ_FAULT` is claimed supported, so the injection is exercised directly.
///
/// `happenstance-postgres`'s fixture refuses to ship an untested injection to
/// satisfy a check, and so does this one. The borrowed rule
/// `arming_a_read_fault_makes_the_stream_yield_an_error` covers the same ground;
/// this is the adapter-local half that fails with a message about *this*
/// injection rather than about the port.
#[tokio::test]
#[ignore = "needs a live Neon endpoint; set NEON_CONNECTION and run with `-- --ignored`"]
async fn a_read_fault_reaches_the_caller_as_an_err_item() {
    let fixture = NeonFixture::new();
    let store = fixture.connect().await;
    for _ in 0..4 {
        store
            .append(&[probe_event()], None)
            .await
            .expect("seeding should succeed");
    }

    fixture.arm_read_fault().await;

    let query = Query::all();
    // Drained by hand rather than through `StreamExt::collect`, because
    // `futures-util` is not in this crate's graph and taking it as a
    // dev-dependency for one call is a node added to `Cargo.lock` for a `while
    // let`.
    let mut stream = core::pin::pin!(store.read(&query, ReadOptions::new()));
    let mut items = Vec::new();
    while let Some(item) = core::future::poll_fn(|cx| stream.as_mut().poll_next(cx)).await {
        items.push(item);
    }

    assert!(
        items.iter().any(Result::is_err),
        "the armed read produced {} items and none of them was an `Err`, so the \
         injection was absorbed — which is the outcome CF-39 says must be a declined \
         capability rather than a green rule",
        items.len()
    );
}

/// A fixture that answers only the two required capabilities.
///
/// Its only purpose is to *name* the trait's provided defaults so a test can
/// compare against them. There is no other way to tell "declined deliberately"
/// from "never considered": both are the same three tokens of Rust at the impl
/// site.
struct Defaulted;

impl Fixture for Defaulted {
    type Store = NeonEventStore<HyperTransport>;
    const SECOND_HANDLE: Capability = Capability::SUPPORTED;
    const REOPEN: Capability = Capability::SUPPORTED;
    async fn connect(&self) -> Self::Store {
        unreachable!("`Defaulted` exists to be read, never constructed")
    }
}

/// Every capability constant is an answer, and none is the trait's default.
///
/// This runs with no endpoint, deliberately: it is the belt to
/// `Capability::declined("")`'s suspender, which fires at **codegen** for an
/// associated const — so `cargo build` and `cargo test` catch an empty reason and
/// `cargo clippy` does not. A green clippy proves nothing about these.
#[test]
fn capability_constants_are_answered_not_defaulted() {
    assert!(NeonFixture::SECOND_HANDLE.is_supported());
    assert!(NeonFixture::REOPEN.is_supported());
    assert!(NeonFixture::MID_BATCH_FAULT.is_supported());
    assert!(NeonFixture::READ_FAULT.is_supported());

    // Not left at the trait's provided default. Comparing against the default
    // explicitly is the only way to tell an answer from an omission, because the
    // two are the same three tokens of Rust at the impl site.
    assert_ne!(
        NeonFixture::MID_BATCH_FAULT.reason(),
        Defaulted::MID_BATCH_FAULT.reason(),
        "MID_BATCH_FAULT is the trait's silent default on a store that can co-operate"
    );
    assert_ne!(
        NeonFixture::READ_FAULT.reason(),
        Defaulted::READ_FAULT.reason(),
        "READ_FAULT is the trait's silent default on a store that can co-operate"
    );

    // The ceilings are facts mirrored from the adapter, not literals restated.
    assert_eq!(
        NeonFixture::MAX_EVENT_DATA_LEN,
        Some(NeonEventStore::<HyperTransport>::MAX_EVENT_DATA_LEN)
    );
    assert_eq!(
        NeonFixture::MAX_TAGS_PER_EVENT,
        Some(NeonEventStore::<HyperTransport>::MAX_TAGS_PER_EVENT)
    );
    assert_eq!(
        NeonFixture::MAX_EVENTS_PER_BATCH,
        Some(NeonEventStore::<HyperTransport>::MAX_EVENTS_PER_BATCH)
    );
}

/// `CONTENDERS` handles open at once without deadlocking.
///
/// Cheap here in a way it is not for a pooled adapter — there is no pool to
/// exhaust — which is exactly why it is worth asserting: the failure a pool
/// produces is a *hang* rather than a red test (CF-33: there is no watchdog), and
/// a store that acquired one later would regress this silently.
#[tokio::test(flavor = "multi_thread")]
#[ignore = "needs a live Neon endpoint; set NEON_CONNECTION and run with `-- --ignored`"]
async fn contenders_handles_open_concurrently_without_deadlock() {
    let fixture = NeonFixture::new();
    let mut stores = Vec::with_capacity(CONTENDERS);
    for _ in 0..CONTENDERS {
        stores.push(fixture.connect().await);
    }
    let mut handles = Vec::with_capacity(CONTENDERS);
    for store in stores {
        handles.push(tokio::spawn(async move {
            store.head().await.expect("a live handle should answer");
        }));
    }
    for handle in handles {
        handle.await.expect("no contender task should panic");
    }
}

// --- helpers ---------------------------------------------------------------

/// One minimal event, distinct enough to be counted.
fn probe_event() -> Event {
    Event::new("Probe", b"\x00".to_vec()).expect("a valid event")
}

/// Runs one statement through the shared transport and returns its rows.
async fn query(sql: &str, params: Vec<serde_json::Value>) -> Vec<serde_json::Value> {
    let response = HyperTransport::shared()
        .round_trip(SqlRequest::single(SqlStatement::with_params(sql, params)).read_only())
        .await
        .expect("a broken test environment: the endpoint did not answer");
    assert!(
        response.is_success(),
        "the endpoint refused a test's own statement: {}",
        String::from_utf8_lossy(&response.body)
    );
    happenstance_neon::wire::ResponseBody::parse(&response.body)
        .expect("a 2xx body should parse")
        .result_sets()
        .first()
        .map(|result| result.rows.clone())
        .unwrap_or_default()
}

/// One text column of one row.
fn text(row: &serde_json::Value, column: &str) -> String {
    row.get(column)
        .and_then(serde_json::Value::as_str)
        .unwrap_or_default()
        .to_owned()
}

/// How many rows this fixture's `event` table holds.
async fn count(fixture: &NeonFixture) -> usize {
    let rows = query(
        &format!(
            "SELECT count(*)::text AS n FROM {}",
            fixture.config().qualified_event()
        ),
        Vec::new(),
    )
    .await;
    text(&rows[0], "n").parse().expect("a count is a number")
}

/// The read stream is not `Send`, and this is the compiling statement of that.
///
/// It is not a defect: this adapter implements the **bare** flavour, and the
/// stream holds a future with no `Send` bound so a `JsFuture` could meet it on
/// `wasm32`. Written as a call site rather than as prose so a refactor that
/// quietly added the bound would have to delete this line to compile.
fn the_stream_is_polled_in_place<S: Stream>(_stream: S) {}

#[test]
fn the_read_stream_needs_no_send_bound() {
    let _ =
        the_stream_is_polled_in_place::<happenstance_neon::NeonReadStream<'static, HyperTransport>>;
}
