//! The bar, run whole against a real file on disk.
//!
//! An adapter that compiles is not an adapter. This target is where
//! `happenstance-sqlite` stops being an instrument: one `SqliteFixture`, one
//! macro invocation, and every rule in `for_each_event_store_rule!` emitted as
//! its own `#[tokio::test]` driven against SQLite.
//!
//! The fixture itself lives in [`support`] — shared with
//! `tests/concurrency.rs`, and under `tests/support/` rather than at
//! `tests/support.rs` because Cargo compiles every file *directly* under
//! `tests/` as its own binary. Read that module for what the fixture is made
//! of; three of this project's criteria are won or lost in its body rather than
//! in any rule's output.
//!
//! The tokio harness, so: native only.

#![cfg(all(feature = "event-store", not(target_arch = "wasm32")))]
#![allow(clippy::unwrap_used)]

mod support;

use support::SqliteFixture;

happenstance_testkit::event_store_conformance!(SqliteFixture::new());
