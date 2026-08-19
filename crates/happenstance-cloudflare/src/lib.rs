//! Cloudflare Durable Object adapter for happenstance — the workspace's `!Send`
//! instrument.
//!
//! # Status: bound, and implemented
//!
//! This crate depends on [`worker`] and talks to a real Durable Object's
//! `SqlStorage`. [`js`] and [`sql_storage`] are bindings rather than models:
//! `exec` is `worker::SqlStorage::exec`, a cursor is a `SqlStorageCursor`, and
//! a thrown value is a `worker::Error` kept live behind an [`Rc`](std::rc::Rc).
//!
//! Every [`EventStore`](happenstance_core::EventStore) body is real —
//! `migrate`, `append`, `head`, `contains_event_id` and `read` all execute SQL
//! against the object's own storage — and **no unimplemented body remains
//! anywhere in the crate**. The scoped allow of the `clippy::todo` lint that
//! used to stand below the module list left with the last of them, which is
//! what it was written to do; the lint is denied workspace-wide, so the gate
//! now fails on the first one that comes back.
//!
//! What has *not* happened yet is the conformance suite: this adapter has not
//! run `happenstance_testkit::event_store_conformance!` on `workerd`, and
//! until it has, it is an implementation rather than a conformant adapter. The
//! fixture, the host and the gate step are the next three stories'.
//!
//! # What this crate is for
//!
//! It is the only adapter in the workspace that implements the **bare**
//! [`EventStore`](happenstance_core::EventStore) rather than the derived
//! `SendEventStore`, and the only one whose `Error` is genuinely `!Send`. That
//! makes it the sole instrument for ES-6 — "whether `Error` gains `Send +
//! Sync`" — which the specification settles precisely because the two other
//! in-tree confirmations are free by construction: `MemoryStoreError` is
//! uninhabited and `SqliteEventStoreError` has one placeholder variant, so
//! neither could fail the bound if the bound were wrong.
//!
//! # Findings
//!
//! Made against a stand-in, and each one now either **confirmed** against the
//! real API or corrected in place with the correction stated. A finding quietly
//! deleted is a finding that will be re-discovered.
//!
//! ## 1. A real `JsValue` is `Send + Sync` — and so is `worker`'s own storage
//!
//! `wasm-bindgen` 0.2.126, `src/lib.rs:168-176`:
//!
//! ```text
//! pub struct JsValue {
//!     idx: u32,
//!     _marker: PhantomData<*mut u8>, // not at all threadsafe
//! }
//!
//! #[cfg(not(target_feature = "atomics"))]
//! unsafe impl Send for JsValue {}
//! #[cfg(not(target_feature = "atomics"))]
//! unsafe impl Sync for JsValue {}
//! ```
//!
//! **Confirmed, and it is worse than the stand-in recorded.** Workers builds
//! `wasm32-unknown-unknown` without `atomics`, so `JsValue` — and therefore
//! `worker::Error`, including its `Internal(JsValue)` and `UnknownJsError {
//! original: JsValue, .. }` variants — is `Send + Sync` there; and the host
//! build, where `target_feature = "atomics"` is likewise unset, gets the same
//! two impls. `worker` then writes two more of its own, on the storage handle
//! and on its cursor (`worker-0.8.5/src/sql.rs`). Four `unsafe impl`s in the
//! dependency graph, all of which this crate would inherit by holding one of
//! those types bare. The specification's premise for ES-6, that "an adapter
//! error holding a `JsValue` or an `Rc<str>` satisfies [the unbounded type] so
//! a spawned handler's error cannot cross a `JoinHandle`", is half wrong: the
//! `JsValue` half costs nothing, the `Rc` half costs everything.
//!
//! That is why every JS-side value in this crate is reached through an `Rc` —
//! [`js::JsHandle`] holds `Rc<JsValue>`, [`js::JsThrow`] holds
//! `Rc<worker::Error>`, [`sql_storage::SqlStorage`] holds
//! `Rc<worker::SqlStorage>`. `Rc<T>` is `!Send` for **every** `T`, including a
//! `T` that carries an `unsafe impl Send`. The thrown value stays live; the
//! auto trait does not come with it. It is also not an option to mimic the
//! hatch: this workspace sets `unsafe_code = "forbid"`, so an adapter
//! can only ever *inherit* that escape hatch, never write it.
//!
//! ## 2. Stringifying a thrown value loses a capability, not information the
//!    caller needs
//!
//! [`js::JsThrow`] keeps the thrown value and can call
//! [`js::JsHandle::property`]; [`js::StringifiedThrow`] keeps `String(value)`
//! and cannot. On the one question the port makes a caller ask — was this a
//! conflict? — they answer identically, because a Durable Object surfaces
//! SQLite's own text (`UNIQUE constraint failed: event.position`) through the
//! thrown `Error`'s `message` and exposes no numeric code. **Confirmed against
//! the real API**: `worker` caches `name`, `message` and `code` at conversion
//! and finds no SQLite code to cache, which is why
//! [`js::JsThrow::is_constraint_violation`] probes `code` and then falls
//! through to the message every time. The capability that is genuinely lost is
//! *forward* compatibility: a caller holding the live value can read a field
//! nobody has thought of yet.
//!
//! **Now observed rather than predicted, and from the caller's seat rather than
//! this module's.** The `es6_reconstruction` tests in this file drive a
//! constructed thrown value through `worker`'s real bindings, this crate's real
//! classifier and
//! [`EventStore::append`](happenstance_core::EventStore::append), and rebuild
//! the one fact a caller must branch on — conflict versus transport fault — out
//! of the public surface alone. They carry their own positive control, so they
//! are assertions that can fail rather than assertions that cannot.
//!
//! ## 3. The conflict signal never travels in `Self::Error` anyway
//!
//! `happenstance-core` lifts the DCB concurrency signal out of the adapter's
//! error type and into
//! [`AppendError::ConditionViolated`](happenstance_core::AppendError::ConditionViolated).
//! So the adapter has to classify the constraint violation *before* `Self::Error`
//! is constructed, whatever `Self::Error` is. Note the absence of a
//! `ConditionViolated` variant on
//! [`event_store::CloudflareEventStoreError`] — that is the finding made
//! structural. Stringification therefore cannot cost the caller the conflict
//! signal, because the conflict signal is not in the error type on any adapter.
//!
//! What the error type *does* owe a caller is the other half of the same
//! branch: enough to tell a transport fault from a conflict, from a capacity
//! refusal, from a binding nobody wired up. A classifier that picks the right
//! `AppendError` arm and then discards the evidence satisfies every other check
//! in this repository and leaves that caller with nothing to act on;
//! the `es6_reconstruction` tests name that shape and reject it.
//!
//! ## 4. The `Send` flavour does not imply a `Send` error either
//!
//! [`send_shape::send_flavour::SendStoreWithLocalError`] implements
//! `SendEventStore` — `Send` store, `Send` stream, `Send` future — with a
//! `!Send` `Error`, and compiles. So ES-6's named rule
//! `store_error_crosses_a_join_handle` is unwritable against today's port for
//! *every* adapter, not merely for this one. See [`send_shape`] for the probe
//! that separates "the future is `Send`" from "the error is `Send`".
//!
//! # The ES-6 verdict
//!
//! **Recorded here; minted elsewhere.** On the four findings above, ADR-0009's
//! decision holds and this adapter is the evidence for it rather than the
//! exception to it: a caller recovers conflict-versus-transport from what
//! `append` hands back *without* `Error` carrying a `Send + Sync` bound, so the
//! strength belongs in a downstream marker rather than in the port. Both halves
//! of the clause now have an artefact in this file — the auto-trait half in the
//! `!Send` probes, the information half in `es6_reconstruction` — and a reader
//! asking what ES-6 resolved to finds both without leaving the page.
//!
//! The decision atom that states it, with the alternatives that lost, is
//! `adr-0023-and-atom-resolutions`', authored through `/redkiln:kb-ingest`.
//! Nothing under `.kb/` is written by this crate, and
//! `.kb/decisions/0009-error-send-sync.md` is accepted and immutable.
//!
//! # Capability limits that are not type errors
//!
//! Two, and neither shows up as an `error[E….]`:
//!
//! * **A cursor is not a stable snapshot — and it did not have to be.**
//!   Cloudflare documents that a `SqlStorageCursor` held across an `await`
//!   "does not provide a stable snapshot of query results", and this crate used
//!   to record that as an unresolved choice between laziness and isolation,
//!   citing ES-9. **Both halves of that were wrong, and the correction is the
//!   finding.** ES-9 is `from` *names a position, not an index*; the clauses
//!   that carry the sample obligation are **ES-11** (a read is one sample) and
//!   **ES-12** (all items of one query share it), and ES-11 states outright
//!   that "laziness is therefore permitted and never required". So there was no
//!   dilemma to resolve, only a mechanism to implement: ADR-0011's
//!   ceiling-and-page, which [`event_store::SqlRowStream`] uses. A position
//!   ceiling is captured no later than the first poll, every statement after
//!   the first is bounded by it, and each page is drained into memory before
//!   the caller can suspend — so no cursor ever spans a suspension point and
//!   the collision does not arise. [`sql_storage::SqlError::CursorInvalidated`]
//!   remains as the report for a cursor that *is* outlived by another
//!   statement, which is now only reachable by a caller driving `exec`
//!   directly. ES-11 and ES-12 name **this adapter** as the falsifier they were
//!   most at risk from; the ceiling is affordable here, so it does not bite,
//!   and what it costs is one extra statement per page and the rows those pages
//!   re-read.
//! * **Positions are bounded by 2^53, not 2^64.** Workers SQL widens integers
//!   through a JS number on the way out, so a `SequencePosition` above
//!   `Number.MAX_SAFE_INTEGER` is not round-trippable even though
//!   `NonZeroU64` permits it. A stored value that crossed the line arrives back
//!   as [`sql_storage::SqlValue::Real`] rather than as a narrowed integer, and
//!   is reported as
//!   [`event_store::CloudflareEventStoreError::StoredPosition`].
//!
//! # Targets
//!
//! `wasm32-unknown-unknown` is the target this crate exists for and the only
//! one where its bindings resolve to a live JavaScript heap. It also compiles
//! on the host. The host build is a convenience rather than evidence: nothing
//! outside this crate's tests is `cfg`-gated, so it says only that the crate is
//! portable, not that a Durable Object adapter is — every `worker` binding it
//! links resolves to a stub that panics rather than to a JavaScript heap.
//!
//! What the host build *is* good for is the one thing a contributor needs in
//! their inner loop: the `!Send` probes below run there, under an ordinary
//! `cargo test`, with no wasm toolchain at all. They run on `wasm32` too — see
//! the twin below for why one target is not enough.

#![doc(html_no_source)]

pub mod event_store;
pub mod js;
mod query_sql;
pub mod send_shape;
pub mod sql_storage;

// The same condition on the definition and on every caller. Gate only the
// caller and the module survives where nothing calls it.
// Left un-gated it is dead code on wasm, and `dead_code` is an error under the
// gate's `-D warnings` — a failure that lands on the mandatory `wasm32 build of
// the Cloudflare adapter` step, with a message about an unused function that
// says nothing about targets.
#[cfg(all(test, target_arch = "wasm32"))]
mod test_object;

pub use event_store::{CloudflareEventStore, CloudflareEventStoreError, SqlRowStream};
pub use js::{JsHandle, JsThrow, StringifiedThrow};
pub use sql_storage::{SqlCursor, SqlError, SqlRow, SqlStorage, SqlValue};

/// Compiled proof that the types this crate exists for are `!Send`.
///
/// Autoref specialisation, the only way to observe the *absence* of an auto
/// trait on stable: method resolution tries inherent candidates before trait
/// ones and discards an inherent candidate whose bounds do not hold, so
/// `is_send()` resolves to the inherent method when `T: Send` and falls through
/// to the trait method when it does not. A compile-time decision, reported as a
/// runtime `bool`.
///
/// Lifted from `happenstance-testkit/tests/local_conformance.rs:266-289`, which
/// is where the workspace first needed it.
///
/// **No longer gated off `wasm32`.** It used to be, on the argument that "a
/// dev-dependency that only exists to run four assertions is a dev-dependency
/// `cargo deny` has to clear on every run". That argument inverted the moment
/// `worker` landed: `wasm-bindgen-test` is a target-scoped dev-dependency this
/// crate needs anyway, and — decisively — the auto-trait leak these assertions
/// exist to catch is written `#[cfg(not(target_feature = "atomics"))]`, so it
/// can only be observed on the target it is compiled for. A host-only probe
/// would have been a detector pointing away from the thing it detects.
#[cfg(test)]
mod not_send_probe {
    use core::marker::PhantomData;

    pub(crate) struct Probe<T>(pub(crate) PhantomData<T>);

    pub(crate) trait NotSend {
        fn is_send(&self) -> bool {
            false
        }
    }

    impl<T> NotSend for Probe<T> {}

    impl<T: Send> Probe<T> {
        // `&self` is the entire mechanism: an associated function is resolved by
        // path and would never fall through to the trait candidate.
        #[allow(clippy::unused_self)]
        pub(crate) fn is_send(&self) -> bool {
            true
        }
    }
}

/// The four assertions, written once and run on both targets.
///
/// A twin is only worth having if it fails for the same reasons, so the two
/// test modules below are wrappers over these four functions rather than two
/// copies of them. The names are the test names, so a failure names the same
/// fact wherever it happens.
#[cfg(test)]
mod not_send_assertions {
    use core::marker::PhantomData;

    use super::not_send_probe::{NotSend as _, Probe};
    use super::{CloudflareEventStore, CloudflareEventStoreError, JsHandle, SqlRowStream};
    use crate::js::StringifiedThrow;
    use crate::send_shape::send_flavour::{SendStoreWithLocalError, SendStreamWithLocalError};

    macro_rules! assert_not_send {
        ($type:ty, $why:literal) => {
            assert!(
                !Probe::<$type>(PhantomData).is_send(),
                concat!(stringify!($type), " must be !Send: ", $why)
            );
        };
    }

    macro_rules! assert_send {
        ($type:ty, $why:literal) => {
            assert!(
                Probe::<$type>(PhantomData).is_send(),
                concat!(stringify!($type), " must be Send: ", $why)
            );
        };
    }

    /// Without a positive control this whole module would also pass if the probe
    /// were simply broken and always answered `false` — which is exactly the
    /// vacuity ES-6 exists to remove.
    pub(crate) fn the_probe_is_not_vacuous() {
        assert_send!(
            happenstance_core::SequencePosition,
            "it is a NonZeroU64 and the probe is meant to say so"
        );
        assert_send!(
            StringifiedThrow,
            "it holds a String, which is the entire point of the stringified shape"
        );
        assert_send!(
            worker::SqlStorage,
            "worker writes `unsafe impl Send` on it, which is the hatch this crate must not inherit"
        );
    }

    pub(crate) fn the_js_boundary_types_are_not_send() {
        assert_not_send!(
            JsHandle,
            "it holds an Rc<JsValue>, and Rc is what removes Send from a value that has it"
        );
        assert_not_send!(
            CloudflareEventStore,
            "a Durable Object is a single-threaded actor reached through a JS handle"
        );
        assert_not_send!(
            SqlRowStream,
            "a live cursor cannot leave the object's thread"
        );
    }

    /// ES-6, stated as an assertion. This is the only error type in the
    /// workspace that can fail a `Send + Sync` bound on `EventStore::Error`.
    pub(crate) fn the_error_type_is_not_send() {
        assert_not_send!(
            CloudflareEventStoreError,
            "ES-6 is undecidable against error types that are Send by construction"
        );
    }

    /// Finding 4: the derived flavour's obligations are all satisfied and the
    /// error is still `!Send`.
    pub(crate) fn the_send_flavour_does_not_imply_a_send_error() {
        assert_send!(SendStoreWithLocalError, "the trait has a Send supertrait");
        assert_send!(
            SendStreamWithLocalError,
            "SendEventStore::read must return a Send stream"
        );
        assert_not_send!(
            <SendStoreWithLocalError as happenstance_core::SendEventStore>::Error,
            "and yet the error it yields cannot cross a thread"
        );
    }
}

/// The host half, reachable by a plain `cargo test -p happenstance-cloudflare`
/// with no wasm toolchain installed at all.
#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::not_send_assertions as probe;

    #[test]
    fn the_probe_is_not_vacuous() {
        probe::the_probe_is_not_vacuous();
    }

    #[test]
    fn the_js_boundary_types_are_not_send() {
        probe::the_js_boundary_types_are_not_send();
    }

    #[test]
    fn the_error_type_is_not_send() {
        probe::the_error_type_is_not_send();
    }

    #[test]
    fn the_send_flavour_does_not_imply_a_send_error() {
        probe::the_send_flavour_does_not_imply_a_send_error();
    }
}

/// The target half — the twin, and the reason a host-only probe was not enough.
///
/// `unsafe impl Send for JsValue` is written `#[cfg(not(target_feature =
/// "atomics"))]`, so whether this crate's types are `Send` is a question with
/// two answers until both targets are asked. The positive control travels with
/// the twin: a control that only runs on the host proves nothing about a probe
/// compiled for another target.
#[cfg(all(test, target_arch = "wasm32"))]
mod wasm_tests {
    use wasm_bindgen_test::wasm_bindgen_test;

    use super::not_send_assertions as probe;

    #[wasm_bindgen_test]
    fn the_probe_is_not_vacuous() {
        probe::the_probe_is_not_vacuous();
    }

    #[wasm_bindgen_test]
    fn the_js_boundary_types_are_not_send() {
        probe::the_js_boundary_types_are_not_send();
    }

    #[wasm_bindgen_test]
    fn the_error_type_is_not_send() {
        probe::the_error_type_is_not_send();
    }

    #[wasm_bindgen_test]
    fn the_send_flavour_does_not_imply_a_send_error() {
        probe::the_send_flavour_does_not_imply_a_send_error();
    }
}

/// ES-6's **other** half: not "can the error cross a thread", but "does the
/// error still say anything a caller can act on".
///
/// The auto-trait half lives above and is a fact about types. This module is a
/// fact about *contents*, and it is deliberately in the same file: a reader who
/// comes to this crate asking what ES-6 resolved to should find both answers
/// without leaving the page.
///
/// # What a caller is actually deciding
///
/// Someone holding an [`AppendError`](happenstance_core::AppendError) at the
/// edge of their own handler has exactly one branch to take, and the two arms
/// are expensive in opposite directions:
///
/// * **A conflict** — another writer got there first. Retrying the same batch
///   is wrong; the decision model has to be re-read and rebuilt.
/// * **A transport fault** — the store failed for a reason that has nothing to
///   do with the caller's condition. Rebuilding the decision model is wasted
///   work; the right move is to retry.
///
/// Getting that backwards costs either a livelock against a condition that will
/// never pass, or a silently dropped command. So the question this module asks
/// is whether a caller can *recover* the distinction from what `append` hands
/// back — reading only what a downstream crate could read.
///
/// # Why this cannot be a conformance rule
///
/// Every event-store rule asserts on the success path or on a store-produced
/// `AppendError`, and none reads an adapter error's *contents* — a portable rule
/// could not, without asserting on some particular adapter's internals. The
/// portable neighbour is already covered elsewhere:
/// `ViolationAsStoreErrorStore` in the testkit's mutation coverage rejects a
/// violation reported on the wrong `AppendError` arm, for every adapter. What is
/// left is unportable by construction, which is exactly why the workspace's only
/// `!Send` adapter is the only instrument for the clause.
///
/// # Why the tier is `wasm32`
///
/// `worker`'s bindings resolve to panicking stubs off the target, so a store
/// cannot be *driven* on the host at all — only the type-level probes above can
/// run there, and they do. Everything here needs a live JS heap.
#[cfg(all(test, target_arch = "wasm32"))]
mod es6_reconstruction {
    use happenstance_core::{AppendCondition, AppendError, Event, EventStore, Query, QueryItem};
    use wasm_bindgen_test::wasm_bindgen_test;

    use super::event_store::{CloudflareEventStore, CloudflareEventStoreError};
    use crate::sql_storage::SqlError;
    use crate::test_object::{arm_throw, durable_object};

    /// The exact text a Durable Object's SQLite surfaces through the thrown
    /// `Error`'s `message` when the uniqueness an append condition rests on is
    /// violated. Constructed here rather than round-tripped through a live
    /// object, so this artefact does not wait on the `workerd` runner.
    const CONSTRAINT_TEXT: &str = "UNIQUE constraint failed: event.position";

    /// A failure that is emphatically *not* a conflict.
    const TRANSPORT_TEXT: &str = "network connection lost";

    /// One migrated store over a fresh object, reached the only way there is.
    fn open() -> (crate::sql_storage::SqlStorage, CloudflareEventStore) {
        let sql = durable_object();
        let store = CloudflareEventStore::new(sql.clone());
        store.migrate().expect("the schema applies");
        (sql, store)
    }

    fn event(event_type: &str) -> Event {
        Event::new(event_type.to_owned(), &b"payload"[..]).expect("a valid event type")
    }

    fn condition_on(event_type: &str) -> AppendCondition {
        AppendCondition::new(Query::from_item(
            QueryItem::of_types([event_type.to_owned()]).expect("a valid query item"),
        ))
    }

    /// **The predicate the whole artefact turns on**, and the reason AC-004 can
    /// fail: everything below reads the error through this one function, and it
    /// touches nothing a downstream crate could not.
    ///
    /// `Display` on the public error plus the public
    /// [`source`](core::error::Error::source) chain — no private field, no
    /// `pub(crate)` helper, no `#[cfg(test)]` back door. A test that reached
    /// into the type would keep passing after the information stopped being
    /// recoverable, which is precisely the regression it exists to catch.
    ///
    /// `None` means "this did not arrive on the `Store` channel at all".
    fn what_the_store_said(error: &AppendError<CloudflareEventStoreError>) -> Option<String> {
        let AppendError::Store(store) = error else {
            return None;
        };
        let mut rendered = store.to_string();
        let mut source = core::error::Error::source(store);
        while let Some(link) = source {
            rendered.push_str(" | ");
            rendered.push_str(&link.to_string());
            source = link.source();
        }
        Some(rendered)
    }

    /// Whether a caller can recover, from the error alone, *which* failure this
    /// was — not merely that one happened.
    fn names_the_underlying_failure(
        error: &AppendError<CloudflareEventStoreError>,
        expected: &str,
    ) -> bool {
        what_the_store_said(error).is_some_and(|said| said.contains(expected))
    }

    /// AC-001. A constraint violation reaches the caller on the
    /// **`ConditionViolated` channel**, so their next move is "re-read and
    /// rebuild the decision model" and never "retry the transport".
    ///
    /// Note what is *not* asserted: a `ConditionViolated` variant on
    /// `CloudflareEventStoreError`. There is none, and adding one would look
    /// like the fix and be the defect — the contract lifts the conflict signal
    /// out of every adapter's error type before `Self::Error` is constructed.
    #[wasm_bindgen_test]
    async fn constraint_violation_reaches_the_caller_as_condition_violated() {
        let (sql, store) = open();
        arm_throw(&sql, "INSERT INTO event", CONSTRAINT_TEXT);

        let failure = store
            .append(
                &[event("SeatReserved")],
                Some(&condition_on("SeatReserved")),
            )
            .await
            .expect_err("the armed constraint violation refuses the write");

        assert!(
            failure.is_condition_violated(),
            "a constraint violation must arrive as a conflict, not as a transport fault: {failure:?}"
        );
        assert!(
            what_the_store_said(&failure).is_none(),
            "and therefore not on the Store channel at all: {failure:?}"
        );
    }

    /// AC-002. A transport fault reaches the caller **distinguishably**: on the
    /// `Store` channel, still carrying what the store said, so the caller can
    /// retry rather than rebuild.
    ///
    /// The second assertion is the one that matters. An error that arrives on
    /// the right arm and says nothing is indistinguishable from a network fault,
    /// a storage cap, or a binding nobody wired up — and a caller who cannot
    /// tell those apart cannot choose a recovery.
    #[wasm_bindgen_test]
    async fn transport_fault_reaches_the_caller_distinguishably() {
        let (sql, store) = open();
        arm_throw(&sql, "INSERT INTO event", TRANSPORT_TEXT);

        let failure = store
            .append(
                &[event("SeatReserved")],
                Some(&condition_on("SeatReserved")),
            )
            .await
            .expect_err("the armed transport fault refuses the write");

        assert!(
            !failure.is_condition_violated(),
            "a transport fault is not a conflict: {failure:?}"
        );
        assert!(
            matches!(
                &failure,
                AppendError::Store(CloudflareEventStoreError::Sql(SqlError::Thrown(_)))
            ),
            "it arrives on the Store channel as a live throw: {failure:?}"
        );
        assert!(
            names_the_underlying_failure(&failure, TRANSPORT_TEXT),
            "and the caller can recover what the store said: {:?}",
            what_the_store_said(&failure)
        );
        assert!(
            !names_the_underlying_failure(&failure, "constraint failed"),
            "without it reading as a conflict"
        );
    }

    /// AC-004. The named wrong error shape, and the same predicate rejecting it.
    ///
    /// This is **not** the blunt mutant — an error whose `Display` renders "a SQL
    /// error occurred" — but the subtle one a careful implementer reaches
    /// honestly: a classifier that distinguishes correctly *inside* `append`,
    /// uses the answer to pick the right `AppendError` arm, and then throws the
    /// evidence away. It satisfies AC-001, it satisfies the `Store`-arm half of
    /// AC-002, and every other check in this repository passes against it.
    ///
    /// Without this control the two tests above are a rule no adapter can fail,
    /// which is the decorative shape the house rules name. It is the same reason
    /// `the_probe_is_not_vacuous` exists one module up.
    #[wasm_bindgen_test]
    async fn an_evidence_discarding_classifier_is_rejected() {
        let (sql, store) = open();
        arm_throw(&sql, "INSERT INTO event", TRANSPORT_TEXT);

        let real = store
            .append(
                &[event("SeatReserved")],
                Some(&condition_on("SeatReserved")),
            )
            .await
            .expect_err("the armed transport fault refuses the write");

        // The wrong shape: right arm, evidence discarded. Built *from* the real
        // failure so the only difference between them is the thing under test.
        let flattened: AppendError<CloudflareEventStoreError> =
            AppendError::Store(CloudflareEventStoreError::CorruptTags);

        assert!(
            names_the_underlying_failure(&real, TRANSPORT_TEXT),
            "the real error names the failure"
        );
        assert!(
            !names_the_underlying_failure(&flattened, TRANSPORT_TEXT),
            "and the evidence-discarding shape does not — so the assertion can fail"
        );
        assert_eq!(
            flattened.is_condition_violated(),
            real.is_condition_violated(),
            "even though the wrong shape picks the same arm, which is why the arm alone is not enough"
        );
    }

    /// AC-003. The reconstruction is reachable by a **downstream** consumer:
    /// generic code binding the bare `EventStore` — the weaker flavour, which
    /// accepts both — over nothing but this crate's public surface.
    ///
    /// Its value is at compile time. If the fact stopped being reachable without
    /// a private field or a `pub(crate)` helper, this function would stop
    /// compiling rather than quietly keep passing.
    #[wasm_bindgen_test]
    async fn the_distinction_is_reachable_from_outside_the_crate() {
        async fn classify_like_a_consumer<S: EventStore>(
            store: &S,
            events: &[Event],
            condition: &AppendCondition,
        ) -> &'static str
        where
            S::Error: core::fmt::Display,
        {
            match store.append(events, Some(condition)).await {
                Ok(_) => "accepted",
                Err(error) if error.is_condition_violated() => "rebuild",
                Err(AppendError::Store(error)) if !error.to_string().is_empty() => "retry",
                Err(_) => "cannot tell",
            }
        }

        let (sql, store) = open();
        let batch = [event("SeatReserved")];
        let condition = condition_on("SeatReserved");

        arm_throw(&sql, "INSERT INTO event", CONSTRAINT_TEXT);
        assert_eq!(
            classify_like_a_consumer(&store, &batch, &condition).await,
            "rebuild"
        );

        arm_throw(&sql, "INSERT INTO event", TRANSPORT_TEXT);
        assert_eq!(
            classify_like_a_consumer(&store, &batch, &condition).await,
            "retry"
        );

        assert_eq!(
            classify_like_a_consumer(&store, &batch, &condition).await,
            "accepted",
            "and with nothing armed the write simply lands"
        );
    }

    /// EC-001, as an assertion. An *unclassifiable* throw must not collapse into
    /// a silent "not a violation" — a caller who is told "transport" about a
    /// conflict retries forever against a condition that will never pass.
    ///
    /// The shape that reaches this is a thrown value `worker` builds out of Rust
    /// rather than out of a throw: there is no live JS value behind it, so the
    /// property lookup has nothing to interrogate and the cached message is all
    /// there is. It still arrives on the `Store` channel carrying that message,
    /// which is the honest answer — "the store failed and this is what it said"
    /// — rather than a fabricated verdict.
    #[wasm_bindgen_test]
    async fn an_unclassifiable_throw_still_says_what_happened() {
        let (sql, store) = open();
        arm_throw(&sql, "INSERT INTO event", "");

        let failure = store
            .append(
                &[event("SeatReserved")],
                Some(&condition_on("SeatReserved")),
            )
            .await
            .expect_err("the armed throw refuses the write");

        assert!(
            !failure.is_condition_violated(),
            "an empty message is not evidence of a conflict: {failure:?}"
        );
        assert!(
            what_the_store_said(&failure).is_some(),
            "and it still arrives on the Store channel rather than vanishing: {failure:?}"
        );
    }
}
