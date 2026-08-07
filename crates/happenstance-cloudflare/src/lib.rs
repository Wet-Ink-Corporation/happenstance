//! Cloudflare Durable Object adapter for happenstance — the workspace's `!Send`
//! instrument.
//!
//! # Status: not implemented
//!
//! Every type here is real and every body is `todo!()`. That is the whole
//! design: a skeleton exists to be disagreed with by a type checker, and a
//! skeleton that stubs its associated types has stubbed the only part a type
//! checker can disagree about. Phase 9 fills the bodies in and swaps
//! [`sql_storage`] for the real `worker` bindings.
//!
//! # What this crate is for
//!
//! It is the only adapter in the workspace that implements the **bare**
//! [`EventStore`](happenstance_core::EventStore) rather than the derived
//! `SendEventStore`, and the only one whose `Error` is genuinely `!Send`. That
//! makes it the sole instrument for ES-6 — "whether `Error` gains `Send +
//! Sync`" — which the specification defers precisely because the two in-tree
//! confirmations are free by construction: `MemoryStoreError` is uninhabited and
//! `SqliteEventStoreError` has one placeholder variant, so neither could fail
//! the bound if the bound were wrong.
//!
//! # What is modelled, and what is not
//!
//! There is no dependency on `worker`. [`js::JsHandle`] and [`sql_storage`]
//! reproduce the four properties of a Durable Object's storage that any
//! signature can see — `!Send`, `!Sync`, a **synchronous** `exec`, and a cursor
//! that is not a snapshot — and nothing else. See [`sql_storage`] for why each
//! one is load-bearing.
//!
//! # Findings
//!
//! ## 1. A real `JsValue` is `Send + Sync` on the target Workers builds
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
//! Workers builds `wasm32-unknown-unknown` without `atomics`, so `JsValue` —
//! and therefore `worker::Error`, including its `Internal(JsValue)` and
//! `UnknownJsError { original: JsValue, .. }` variants — is `Send + Sync`
//! there. The specification's premise for ES-6, that "an adapter error holding
//! a `JsValue` or an `Rc<str>` satisfies [the unbounded type] `so a spawned
//! handler's error cannot cross a JoinHandle`", is half wrong: the `JsValue`
//! half costs nothing, the `Rc` half costs everything.
//!
//! That is why [`js::JsHandle`] holds an `Rc<str>` rather than mimicking the
//! `unsafe impl`. An instrument whose `!Send`-ness disappears under a `cfg`
//! cannot falsify a bound. It is also not an option here: this workspace sets
//! `unsafe_code = "forbid"`, so an adapter can only ever *inherit* that escape
//! hatch by holding a `JsValue`, never write it.
//!
//! ## 2. Stringifying a `JsValue` loses a capability, not information the
//!    caller needs
//!
//! [`js::JsThrow`] keeps the thrown value and can call
//! [`js::JsHandle::property`]; [`js::StringifiedThrow`] keeps
//! `String(value)` and cannot. On the one question the port makes a caller ask
//! — was this a conflict? — they answer identically, because a Durable Object
//! surfaces SQLite's own text (`UNIQUE constraint failed: event.position`)
//! through the thrown `Error`'s `message` and exposes no numeric code. The
//! capability that is genuinely lost is *forward* compatibility: a caller
//! holding the live value can read a field nobody has thought of yet.
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
//! * **A lazy read stream is not a stable snapshot.** Cloudflare documents that
//!   a `SqlStorageCursor` held across an `await` "does not provide a stable
//!   snapshot of query results". ES-9 requires the stream to be lazy, so this
//!   adapter has to choose between honouring laziness and honouring snapshot
//!   isolation. [`event_store::SqlRowStream`] models the detection rather than
//!   the fix; the fix is either buffering the whole result set at first poll
//!   (which defeats streaming a large replay) or a rule that says a read is a
//!   snapshot only until the first `await`.
//! * **Positions are bounded by 2^53, not 2^64.** Workers SQL widens integers
//!   through a JS number on the way out, so a `SequencePosition` above
//!   `Number.MAX_SAFE_INTEGER` is not round-trippable even though
//!   `NonZeroU64` permits it. Reported as
//!   [`event_store::CloudflareEventStoreError::StoredPosition`].
//!
//! # Targets
//!
//! Compiles on `wasm32-unknown-unknown`, which is the target it exists for, and
//! on the host. The host build is a convenience rather than evidence: nothing in
//! the stand-in is `cfg`-gated, so it says only that the crate is portable, not
//! that a Durable Object adapter is.

#![doc(html_no_source)]
// `clippy::todo` is denied workspace-wide. Scoped here rather than left open in
// the workspace manifest so that it is visible in review and disappears with the
// last `todo!()` rather than outliving it. Phase 9 removes both the bodies and
// this line.
#![allow(clippy::todo)]

pub mod event_store;
pub mod js;
pub mod send_shape;
pub mod sql_storage;

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
/// Gated off `wasm32` alongside its only caller. The probe reports a
/// compile-time fact through a runtime `bool`, so it needs a test harness to
/// report it, and `wasm32-unknown-unknown` has none without `wasm-bindgen-test`
/// — which this crate does not depend on, because a dev-dependency that only
/// exists to run four assertions is a dev-dependency `cargo deny` has to clear
/// on every run. Left un-gated it is dead code on wasm, and `dead_code` is an
/// error under the gate's `-D warnings`.
#[cfg(all(test, not(target_arch = "wasm32")))]
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

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
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
    #[test]
    fn the_probe_is_not_vacuous() {
        assert_send!(
            happenstance_core::SequencePosition,
            "it is a NonZeroU64 and the probe is meant to say so"
        );
        assert_send!(
            StringifiedThrow,
            "it holds a String, which is the entire point of the stringified shape"
        );
    }

    #[test]
    fn the_js_boundary_types_are_not_send() {
        assert_not_send!(JsHandle, "it holds an Rc<str>, and Rc is what removes Send");
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
    #[test]
    fn the_error_type_is_not_send() {
        assert_not_send!(
            CloudflareEventStoreError,
            "ES-6 is undecidable against error types that are Send by construction"
        );
    }

    /// Finding 4: the derived flavour's obligations are all satisfied and the
    /// error is still `!Send`.
    #[test]
    fn the_send_flavour_does_not_imply_a_send_error() {
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
