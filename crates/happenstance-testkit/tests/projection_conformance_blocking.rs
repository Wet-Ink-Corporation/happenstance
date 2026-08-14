//! The same projection rules, driven with no async runtime at all.
//!
//! The second of the three harnesses CF-23 requires for a rule family, and the
//! one that keeps the first honest: if a projection rule — or the machinery
//! around it — ever quietly acquired a dependency on `tokio`, on a timer, or on
//! being polled from more than one thread, this harness is what would notice.
//! [`block_on`](happenstance_testkit::block_on) is twenty lines, one thread and
//! a park/unpark waker.
//!
//! It also exercises the emitter parameter itself. Nothing here names
//! `#[tokio::test]`; the wrapper is chosen at the call site.

// Not the fixture: `MemoryProjectionFixture` compiles for `wasm32` and
// `projection_conformance_wasm.rs` is the standing proof of it. It is the
// *emitter*: `__emit_projection_blocking` emits a plain `#[test]`, and libtest
// does not exist on `wasm32-unknown-unknown`.
#![cfg(not(target_arch = "wasm32"))]

use happenstance_testkit::fixtures::MemoryProjectionFixture;

happenstance_testkit::projection_store_conformance!(
    mod_name = projection_conformance_blocking,
    emit = happenstance_testkit::__emit_projection_blocking,
    fixture = MemoryProjectionFixture::new()
);
