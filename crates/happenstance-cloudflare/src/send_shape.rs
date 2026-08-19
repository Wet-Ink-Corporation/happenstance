//! Compile probes for ES-6: `Send` on the *future* is not `Send` on the
//! *error*.
//!
//! # The distinction, and why it cost phase 1 a rule
//!
//! `Send` on a future is a property of the state machine — of the values held
//! **across** a suspension point. `Send` on that future's `Output` is a property
//! of a different type entirely. A future with no suspension points has an empty
//! witness set and is `Send` no matter what it eventually returns, so a
//! conformance rule that spawns an adapter's `append` future and reports "it
//! compiled" has checked the wrong obligation and will pass against an error
//! type that can never cross a `JoinHandle`.
//!
//! ES-6's named rule is `store_error_crosses_a_join_handle`. This module is the
//! demonstration that the name has to be taken literally: the *error* has to
//! cross it, not the future that produces it.
//!
//! Both halves are checked by the toolchain rather than asserted in prose —
//! [`assert_future_is_send`] carries a passing doctest and
//! [`assert_output_is_send`] carries a `compile_fail,E0277` one, so `cargo test
//! --doc` re-proves both on every run and rustdoc verifies the error code.

use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};

use happenstance_core::{AppendError, SequencePosition};

use crate::event_store::CloudflareEventStoreError;

/// A future that **is** `Send` and whose `Output` is **not**.
///
/// Written as a hand-rolled unit struct rather than an `async` block on purpose:
/// a coroutine's auto traits are inferred, and an inferred property is a bad
/// instrument. A field-less struct is `Send` by construction, so the only thing
/// left for the probes below to disagree about is the `Output`.
///
/// This is not a strawman. It is the shape of any adapter that builds its error
/// value at the point of return — which is every adapter, because an error is
/// what a fallible operation produces last.
///
/// No `#[non_exhaustive]`, unlike every other public struct in this workspace:
/// the attribute makes the unit constructor unnameable downstream, and a probe
/// nobody outside the crate can construct cannot be a probe. It is also honest —
/// this type is a fixed instrument, not API that grows.
#[derive(Debug, Clone, Copy, Default)]
pub struct AppendFutureShape;

impl Future for AppendFutureShape {
    type Output = Result<SequencePosition, AppendError<CloudflareEventStoreError>>;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        todo!("phase 9: this probe exists for its type, not its behaviour")
    }
}

/// The obligation a *decorative* rule checks: that the future is `Send`.
///
/// [`AppendFutureShape`] satisfies it while carrying a `!Send` error, which is
/// the finding.
///
/// ```
/// use happenstance_cloudflare::send_shape::{AppendFutureShape, assert_future_is_send};
///
/// // Compiles. The future is `Send`; its error is not, and nothing here notices.
/// let _future = assert_future_is_send(AppendFutureShape);
/// ```
pub fn assert_future_is_send<F: Future + Send>(future: F) -> F {
    future
}

/// The obligation ES-6 actually asks about: that the future's `Output` is
/// `Send`.
///
/// ```compile_fail,E0277
/// use happenstance_cloudflare::send_shape::{AppendFutureShape, assert_output_is_send};
///
/// // error[E0277]: `Rc<worker::Error>` cannot be sent between threads safely
/// let _future = assert_output_is_send(AppendFutureShape);
/// ```
pub fn assert_output_is_send<F: Future>(future: F) -> F
where
    F::Output: Send,
{
    future
}

/// A `SendEventStore` whose `Error` is `!Send`.
///
/// This is the probe that matters, and it is deliberately kept in its own module
/// so that only one of the two flavour names is in scope (CLAUDE.md's fourth
/// binding constraint; two would make every method call `error[E0034]`).
///
/// If this compiles — and it does — then the *derived* flavour does not imply a
/// `Send` error either, and `store_error_crosses_a_join_handle` cannot be
/// written against today's port for **any** adapter, not merely for the `!Send`
/// one. That is a stronger statement than ES-6 makes and it is what ADR-0009
/// has to answer.
pub mod send_flavour {
    use core::pin::Pin;
    use core::task::{Context, Poll};

    use futures_core::Stream;
    use happenstance_core::{
        AppendCondition, AppendError, Event, EventId, Query, ReadOptions, SendEventStore,
        SequencePosition, SequencedEvent,
    };

    use crate::event_store::CloudflareEventStoreError;

    /// A store that satisfies every `Send` obligation the derived flavour
    /// states — the store is `Send`, the stream is `Send`, the future is `Send`
    /// — while its `Error` is not.
    ///
    /// It is not a Cloudflare adapter and never will be. It lives here because
    /// this is the crate that owns ES-6, and "a rule that no adapter can fail is
    /// decorative" requires naming the wrong implementation the rule must
    /// reject.
    ///
    /// Not `#[non_exhaustive]`, for the reason given on
    /// [`AppendFutureShape`](super::AppendFutureShape).
    #[derive(Debug, Clone, Copy, Default)]
    pub struct SendStoreWithLocalError;

    /// A `Send` stream whose `Item` is not.
    #[derive(Debug, Clone, Copy, Default)]
    pub struct SendStreamWithLocalError;

    impl Stream for SendStreamWithLocalError {
        type Item = Result<SequencedEvent, CloudflareEventStoreError>;

        fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
            Poll::Ready(None)
        }
    }

    impl SendEventStore for SendStoreWithLocalError {
        type Error = CloudflareEventStoreError;

        fn read(
            &self,
            _query: &Query,
            _options: ReadOptions,
        ) -> impl Stream<Item = Result<SequencedEvent, Self::Error>> + Send {
            SendStreamWithLocalError
        }

        async fn append(
            &self,
            _events: &[Event],
            _condition: Option<&AppendCondition>,
        ) -> Result<SequencePosition, AppendError<Self::Error>> {
            todo!("phase 9: this probe exists for its type, not its behaviour")
        }

        // `head` and `contains_event_id` are here because the trait requires
        // them, and they are worth having: both return `Self::Error` *bare*,
        // without `AppendError` wrapping it, so they widen finding 4 — the
        // derived flavour does not imply a `Send` error on any method, not just
        // on the one whose error is wrapped. Bodies stay `todo!()` so the probe
        // keeps proving a fact about types and nothing about behaviour.
        async fn head(&self) -> Result<Option<SequencePosition>, Self::Error> {
            todo!("phase 9: this probe exists for its type, not its behaviour")
        }

        async fn contains_event_id(&self, _id: EventId) -> Result<bool, Self::Error> {
            todo!("phase 9: this probe exists for its type, not its behaviour")
        }
    }
}
