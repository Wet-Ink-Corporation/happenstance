//! One executor for every arm, because an executor is a confound.
//!
//! # Why not `happenstance_testkit::block_on` for the memory arms
//!
//! It would be the cheaper choice and it is the wrong one. `block_on`
//! (`crates/happenstance-testkit/src/registry.rs:330`) is a fifteen-line park
//! loop with no runtime at all, and it is genuinely faster than driving a future
//! through tokio. Using it for `MemoryEventStore` and tokio for
//! `SqliteEventStore` would put the executor difference *inside* the
//! memory-versus-SQLite ratio, where nothing separates it from the storage
//! difference the ratio is supposed to report.
//!
//! So every timed arm in this crate runs on the same current-thread tokio
//! runtime, and the `block_on`-versus-tokio delta is measured **once**, as a
//! declared control (`tests/controls_fire.rs`), rather than smeared across every
//! row.
//!
//! # Why `current_thread`, and the one place that is not enough
//!
//! `current_thread` because the contention scenario builds its *k* futures
//! before polling any of them and drives them round-robin on one thread — that
//! is `happenstance_testkit::bench`'s design, and it is what keeps the family
//! usable by the `!Send` flavour the whole two-trait port exists for. A
//! multi-threaded runtime would add OS-thread interleaving that the `!Send`
//! flavour cannot have, so the two flavours would stop being measured on the
//! same workload.
//!
//! The exception is `spawn_blocking`. `SqliteEventStore::read` hops every page
//! to the blocking pool, which exists on a `current_thread` runtime too — it is
//! a separate thread pool, not a worker set — so `enable_all` plus the default
//! blocking pool is sufficient and no `rt-multi-thread` flavour is needed for
//! any arm here.
//!
//! # The trap this module exists to make unhittable
//!
//! `SqliteEventStore` captures `Handle::try_current().ok()` **at construction**
//! (`crates/happenstance-sqlite/src/event_store.rs:326-331`), not at poll. A
//! store built outside a runtime therefore carries `None`, and its first `read`
//! fails with `SqliteEventStoreError::NoRuntime` — long after the fixture that
//! built it looked fine. criterion's non-async bench loop has no runtime in
//! scope, which is precisely the shape that hits this.
//!
//! [`enter`] returns the guard that makes construction safe, and
//! [`block_on`] runs a future on the shared runtime. Build every store through
//! one of the two; never with neither.

use std::sync::OnceLock;

use tokio::runtime::{EnterGuard, Runtime};

/// The one runtime, built on first use and never torn down.
static RUNTIME: OnceLock<Runtime> = OnceLock::new();

/// The shared current-thread runtime.
///
/// # Panics
///
/// Panics if the runtime cannot be built, which means the process has no
/// reactor available — a broken measurement environment, not a finding.
pub fn shared() -> &'static Runtime {
    RUNTIME.get_or_init(|| {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("a broken measurement environment, not a finding")
    })
}

/// A guard that puts the shared runtime in scope for the current thread.
///
/// Hold it across **construction** of anything that captures a `Handle` — which
/// in this workspace means every `SqliteEventStore` and every
/// `SqliteProjectionStore`. Dropping it before the store is built is the
/// `NoRuntime` trap the module docs describe.
pub fn enter() -> EnterGuard<'static> {
    shared().enter()
}

/// Drives `future` to completion on the shared runtime.
///
/// # Panics
///
/// Panics if called from inside a runtime context — tokio refuses a nested
/// `block_on`, and the panic naming this call site is more useful than a
/// deadlock.
pub fn block_on<F: Future>(future: F) -> F::Output {
    shared().block_on(future)
}
