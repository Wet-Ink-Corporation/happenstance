//! A read stream with each event rewritten or dropped on its way out: how the
//! wrong stores here misreport what they hold. Hand-written because the
//! harness carries `futures-core` only, and boxed so the adapter is `Unpin`
//! without a pin projection.

use core::pin::Pin;
use core::task::{Context, Poll};

use futures_core::Stream;
use happenstance_core::SequencedEvent;

/// `inner`, with `rewrite` applied to every event: `Some` passes it on
/// (changed or not), `None` drops it. Errors pass through untouched.
pub(crate) struct MapStream<St, F> {
    inner: Pin<Box<St>>,
    rewrite: F,
}

impl<St, F> MapStream<St, F> {
    pub(crate) fn new(inner: St, rewrite: F) -> Self {
        Self {
            inner: Box::pin(inner),
            rewrite,
        }
    }
}

impl<St, F, E> Stream for MapStream<St, F>
where
    St: Stream<Item = Result<SequencedEvent, E>>,
    F: FnMut(SequencedEvent) -> Option<SequencedEvent> + Unpin,
{
    type Item = Result<SequencedEvent, E>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();
        loop {
            match this.inner.as_mut().poll_next(cx) {
                Poll::Ready(Some(Ok(event))) => {
                    if let Some(event) = (this.rewrite)(event) {
                        return Poll::Ready(Some(Ok(event)));
                    }
                }
                other => return other,
            }
        }
    }
}
