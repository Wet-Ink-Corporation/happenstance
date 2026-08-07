//! The same 27 rules, driven with no async runtime at all.
//!
//! This is the second of the three harnesses the registry has to support
//! (CF-23), and it is the one that keeps the first honest. If a rule — or the
//! machinery around it — ever quietly acquired a dependency on `tokio`, on a
//! timer, or on being polled from more than one thread, this harness would be
//! the thing that noticed: [`block_on`](happenstance_testkit::block_on) is
//! twenty lines, one thread and a park/unpark waker.
//!
//! It also exercises the emitter parameter itself. Nothing here names
//! `#[tokio::test]`; the wrapper is chosen at the call site, which is the whole
//! point of the `emit =` argument.

// The blocking harness needs no runtime at all, but `MemoryEventStore` comes
// from a dev-dependency that is native-only in this crate's manifest.
#![cfg(not(target_arch = "wasm32"))]

use happenstance_core::MemoryEventStore;

happenstance_testkit::event_store_conformance!(
    mod_name = dcb_conformance_blocking,
    emit = happenstance_testkit::__emit_blocking,
    factory = MemoryEventStore::new()
);
