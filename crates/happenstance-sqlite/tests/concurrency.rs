//! The racing half of the bar, against the same real file.
//!
//! `tests/conformance.rs` runs the suite one caller at a time, and one caller at
//! a time cannot tell an atomic check-and-write from a probe followed by an
//! insert: with nobody else running there is no second caller to fit between the
//! two halves (`crates/happenstance-testkit/src/concurrency.rs:16-23`). **This
//! target supplies the second caller** — `CONTENDERS` bare OS threads under
//! [`std::thread::scope`], each driving its own future with the testkit's
//! park-loop `block_on`, each holding its own `rusqlite::Connection` onto one
//! file.
//!
//! # Why this is a separate binary rather than a `mod` in `tests/conformance.rs`
//!
//! Three reasons, and the third is the one that forces it: the family is
//! **opt-in** (an adapter whose store is `!Send` cannot invoke it and must not
//! be expected to), it carries a **different bound** —
//! `F::Store: EventStore + Send` — and it **does not exist on `wasm32`**, which
//! has no threads to race on. The fixture is shared through
//! [`support`] rather than duplicated.
//!
//! # One emitter, and the omission is a decision
//!
//! `crates/happenstance-testkit/tests/memory_concurrency_conformance.rs`
//! invokes the macro **twice**, once per shipped emitter, as CF-23's
//! demonstration that the wrapper is a parameter. Copying that verbatim is the
//! natural move here and it is wrong.
//! `__emit_concurrency_blocking` generates a plain `#[test]` driven by
//! `happenstance_testkit::block_on`, so there is **no tokio runtime anywhere in
//! that test** — including on the thread that builds the fixture and calls
//! `connect()`. ADR-0022 §9 captures the runtime handle at construction, so
//! under the blocking emitter it would be captured as `None` and every read
//! would fail with [`SqliteEventStoreError::NoRuntime`] — a red family that is
//! not about this adapter's logic.
//!
//! So: the **tokio emitter only**, stated rather than left to be rediscovered in
//! five red rules. CF-23 is satisfied for this family inside the testkit, by the
//! harness that can honestly run both.
//!
//! [`SqliteEventStoreError::NoRuntime`]: happenstance_sqlite::event_store::SqliteEventStoreError::NoRuntime
//!
//! # There is no watchdog here, and none may be added
//!
//! CF-33 is `[FROZEN]`: no conformance rule may read a clock, measure elapsed
//! time or assert on an operation count. Liveness rests on the CI job timeout.
//! If a rule hangs, that is evidence about ADR-0022's busy-timeout paragraph and
//! is escalated there — not `#[timeout]`, not a retry loop, not a watchdog
//! thread. The adapter's own busy handler is a different thing and is permitted;
//! `every_connection_carries_the_declared_busy_timeout` is what checks it is
//! finite.

#![cfg(all(feature = "event-store", not(target_arch = "wasm32")))]
#![allow(clippy::unwrap_used)]

mod support;

use happenstance_core::{Event, EventStore, Query, ReadOptions, Tags};
use happenstance_sqlite::connection::BUSY_TIMEOUT_MS;
use happenstance_sqlite::event_store::SqliteEventStoreError;
use happenstance_testkit::{Fixture, block_on};
use support::SqliteFixture;

happenstance_testkit::event_store_concurrency_conformance!(SqliteFixture::new());

/// One event, built the way every rule in the family builds one.
fn event(event_type: &str, tag: (&str, &str)) -> Event {
    Event::new(event_type, &b"{}"[..])
        .unwrap()
        .with_tags(Tags::from_pairs([tag]).unwrap())
}

/// AC-002 — a contender is a bare OS thread, and the store has to serve one.
///
/// # What this is really asserting
///
/// Every contender in this family runs on a thread [`std::thread::scope`]
/// spawned, driving its own future with the testkit's park-loop `block_on`.
/// `tokio`'s runtime context is **thread-local**, so `Handle::try_current()`
/// fails on that thread however healthy the runtime the *test* is running under.
/// A store that resolved its `spawn_blocking` runtime at poll time would answer
/// every one of those threads with [`SqliteEventStoreError::NoRuntime`], and the
/// family's own output could not tell you that: a racing rule renders it as
/// `Attempt::Failed` and the reader rule renders it as a *sighting* — `"a
/// concurrent read failed: {err}"` — which reads as a verdict about atomicity.
///
/// So this runs **before** the family's verdict depends on it, and it exercises
/// both call paths, because they are two different sites: `append` takes the
/// connection mutex on the calling thread, while `read` defers a
/// `spawn_blocking` into `poll_next`. A seam applied to one of them leaves four
/// rules green and one rule wrong about the wrong thing.
///
/// The negative control is the same test with the capture at
/// `SqliteEventStore::with_store_id` removed; it fails on the read half with
/// `NoRuntime`, and the run is cited in this story's `_ledger.md`. It is not
/// left in the tree, because a permanently red test is not a control.
#[tokio::test(flavor = "multi_thread")]
async fn store_serves_a_bare_thread_with_no_ambient_runtime() {
    let fixture = SqliteFixture::new();
    // Constructed here, inside the runtime — which is exactly where ADR-0022 §9
    // says the handle is captured, and exactly where a real caller is.
    let store = fixture.connect().await;

    let (appended, read_back) = std::thread::scope(|scope| {
        let handle = scope.spawn(|| {
            // No ambient reactor on this thread, by construction.
            assert!(
                tokio::runtime::Handle::try_current().is_err(),
                "this thread must have no tokio context, or the test is \
                 asserting nothing"
            );

            let appended = block_on(store.append(&[event("test.raced", ("seam", "1"))], None));
            let read_back = block_on(async {
                happenstance_core::collect(store.read(&Query::all(), ReadOptions::default())).await
            });
            (appended, read_back)
        });
        handle.join().unwrap()
    });

    let position = appended.unwrap_or_else(|err| {
        panic!(
            "append from a bare thread failed: {err} — a contender that cannot \
             reach SQLite is reported as Attempt::Failed and reads as an \
             atomicity verdict"
        )
    });
    let events = read_back.unwrap_or_else(|err| {
        panic!(
            "read from a bare thread failed: {err} — this is the half that \
             arrives dressed as a reader sighting rather than as an error"
        )
    });

    assert_eq!(
        events.len(),
        1,
        "the bare thread's own append must be readable from the bare thread"
    );
    assert_eq!(
        events[0].position, position,
        "compared against the position the store actually assigned, never a \
         literal: AUTOINCREMENT permits gaps"
    );
}

/// AC-002's other half — `NoRuntime` still names a state that can occur.
///
/// ADR-0022 §9 chose to capture a handle at construction and keep
/// `Handle::try_current()` as the fallback, and recorded that the rejected
/// option — running the statement inline on the calling thread — would have made
/// [`SqliteEventStoreError::NoRuntime`] **unreachable**, so that the variant and
/// the module-doc paragraph describing it would have had to go in the same
/// change.
///
/// This is what keeps that sentence honest: a store both *constructed* and
/// *driven* with no runtime anywhere still reports `NoRuntime`, so the variant
/// documents a reachable state rather than a historical one. A plain `#[test]`
/// on purpose — `#[tokio::test]` would supply the very thing being withheld.
#[test]
fn a_store_with_no_runtime_anywhere_reports_no_runtime() {
    let fixture = SqliteFixture::new();
    let store = block_on(fixture.connect());

    let outcome = block_on(async {
        happenstance_core::collect(store.read(&Query::all(), ReadOptions::default())).await
    });

    match outcome {
        Err(SqliteEventStoreError::NoRuntime(_)) => {}
        Err(other) => panic!("expected NoRuntime, and the read failed as: {other}"),
        Ok(_) => panic!(
            "the read succeeded with no runtime anywhere, so the statement ran \
             inline on the calling thread — that is ADR-0022 §9's rejected \
             option (b), and under it the NoRuntime variant and its module-doc \
             paragraph document a state that cannot occur"
        ),
    }
}

/// AC-003 — the busy timeout is a property of **every** connection.
///
/// # Why this reads the pragma back rather than checking the constructor
///
/// With `CONTENDERS + 1` connections on one file, `BEGIN IMMEDIATE` against a
/// busy database returns `SQLITE_BUSY` *immediately* unless a busy handler is
/// configured — and that error becomes `AppendError::Store`, which
/// `Attempt::of` maps to `Attempt::Failed`
/// (`crates/happenstance-testkit/src/concurrency.rs:224-233`). A store that
/// probes outside its write lock and a store that leaks `SQLITE_BUSY` fail the
/// *same* rules and look identical in the output. Only one of those is a finding
/// about `append`.
///
/// `schema-migration-and-identity` already landed the timeout on the first
/// connection. What is new under contention is that the second, third and Nth
/// handle `SqliteFixture::connect()` opens carry it too — which is why this asks
/// the live connection instead of trusting that a constructor was called.
///
/// Finite is asserted as hard as present: an unbounded handler would produce the
/// same zero `SQLITE_BUSY` count and convert a livelock into a hung job that
/// names no rule (CF-33 — there is no watchdog).
#[tokio::test(flavor = "multi_thread")]
async fn every_connection_carries_the_declared_busy_timeout() {
    let fixture = SqliteFixture::new();

    let declared = i64::try_from(BUSY_TIMEOUT_MS).unwrap();
    assert!(
        declared > 0,
        "an unbounded busy handler is forbidden: it converts a livelock into a \
         hung CI job that names no rule"
    );

    let first = fixture.connect().await;
    let second = fixture.connect().await;
    let third = fixture.connect().await;

    for (ordinal, store) in [("first", &first), ("second", &second), ("third", &third)] {
        let settings = store.settings().unwrap();
        assert_eq!(
            settings.busy_timeout_ms(),
            declared,
            "{ordinal} connection: a handle without the busy timeout turns \
             ordinary contention into Attempt::Failed, which is a conformance \
             verdict about the wrong thing"
        );
    }
}

/// AC-007 — the fixture relocation minted no third test binary.
///
/// Cargo compiles every file *directly* under `tests/` as its own target, so the
/// obvious place to share a fixture — `tests/support.rs` — would silently become
/// a third binary containing no `#[test]`: it would build `rusqlite`, link, run
/// nothing and report success. `tests/support/mod.rs` is a plain module instead,
/// and this is the assertion that says so out of the file system rather than out
/// of a comment.
#[test]
fn the_shared_fixture_is_a_module_and_not_a_third_test_target() {
    let tests = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests");

    assert!(
        tests.join("support").join("mod.rs").is_file(),
        "the shared fixture lives at tests/support/mod.rs"
    );
    assert!(
        !tests.join("support.rs").exists(),
        "tests/support.rs would be compiled as its own test binary — a target \
         that builds the driver, runs nothing, and reports success"
    );
}
