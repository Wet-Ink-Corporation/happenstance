//! Driving a future that never suspends, without a runtime.
//!
//! Both stores measured here complete `append` on its first poll:
//! `MemoryEventStore::append` awaits nothing (`memory.rs`, one write lock), and
//! `CloudflareEventStore::append` says so of itself — "this whole body contains
//! no `.await`" (`event_store.rs`, in `append`). So a runtime would be a
//! dependency bought to poll once, and on wasm32 a blocking executor cannot
//! exist at all. A future that *did* suspend is reported as `None` rather than
//! spun on, which would hang the single JS thread.

use std::future::Future;
use std::pin::pin;
use std::task::{Context, Poll, Waker};

/// Polls `future` once; `Some` with its output if it completed.
pub fn now_or_never<F: Future>(future: F) -> Option<F::Output> {
    let mut future = pin!(future);
    let mut cx = Context::from_waker(Waker::noop());
    match future.as_mut().poll(&mut cx) {
        Poll::Ready(output) => Some(output),
        Poll::Pending => None,
    }
}
