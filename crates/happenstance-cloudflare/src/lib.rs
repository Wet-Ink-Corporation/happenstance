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
//! against the object's own storage — and **no `todo!()` remains anywhere in
//! the crate**. The scoped `#![allow(clippy::todo)]` that used to stand below
//! the module list left with the last of them, which is what it was written to
//! do.
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
//! ## 4. The `Send` flavour does not imply a `Send` error either
//!
//! [`send_shape::send_flavour::SendStoreWithLocalError`] implements
//! `SendEventStore` — `Send` store, `Send` stream, `Send` future — with a
//! `!Send` `Error`, and compiles. So ES-6's named rule
//! `store_error_crosses_a_join_handle` is unwritable against today's port for
//! *every* adapter, not merely for this one. See [`send_shape`] for the probe
//! that separates "the future is `Send`" from "the error is `Send`".
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
