//! The `Send`/`Sync` facts this adapter's flavour choice rests on, re-checked
//! against the **real** driver.
//!
//! # Why this file exists at all
//!
//! These four assertions are `src/stand_in.rs`'s, re-pointed rather than deleted
//! with it. ADR-0025 makes that an exit criterion rather than a step, and the
//! reason is that they are the only artefact which re-checks the
//! [`SendProjectionStore`] choice *after* the swap — and they reach the same
//! conclusion by a **different mechanism** on each side of it.
//!
//! `unsafe_code` is `forbid` in this workspace, so the stand-in could only reach
//! `Send + Sync` by being built out of `Send + Sync` parts; the compiler derived
//! it. `lbug` reaches it by `unsafe impl Send for Database {}` and three more
//! like it, hand-written over a pointer into C++
//! (`lbug-0.20.3/src/database.rs:14-15`, `src/connection.rs:119-120`). A
//! type checker cannot tell those apart, which is exactly why the stand-in was a
//! usable instrument — and equally why the assertions had to be re-run: the
//! stand-in could not be wrong in the direction `lbug` could be wrong.
//!
//! # What the four grew into
//!
//! The stand-in asserted about `lbug`'s types alone, because it had no adapter to
//! assert about. Those four are kept verbatim in
//! [`the_drivers_types_are_send_and_sync`]. The adapter's own types are added
//! beside them, and they are the half that actually binds: `SendProjectionStore`
//! marks the *futures* `Send` and says nothing about the associated types they
//! carry, so `GraphWriteSet: Send` and `LadybugProjectionStoreError: Send` are
//! obligations a runner discovers and this file states.

// `LadybugProjectionStore` and every type here names an `lbug` type, so without
// the feature there is nothing to assert about.
#![cfg(feature = "driver")]

use happenstance_ladybug::lbug::{Connection, Database, Error, QueryResult};
use happenstance_ladybug::{
    GraphStatement, GraphWriteSet, LadybugProjectionStore, LadybugProjectionStoreError,
};

const fn assert_send<T: Send>() {}
const fn assert_sync<T: Sync>() {}

/// The stand-in module's calibration facts, restated where the compiler can
/// check them against the driver they were a stand-in for.
///
/// If `lbug` is later found to differ, the fix is not here — it is the adapter's
/// flavour choice, reopened.
#[test]
fn the_drivers_types_are_send_and_sync() {
    assert_send::<Database>();
    assert_sync::<Database>();
    assert_send::<Connection<'_>>();
    assert_sync::<Connection<'_>>();

    // `Send` and deliberately **not** `Sync`: the real result holds an
    // interior-mutable cursor into C++, and an adapter that tried to share one
    // across threads must fail to compile. Asserting `Sync` here would be the
    // wrong assertion, so it is named rather than silently absent.
    assert_send::<QueryResult<'_>>();

    // The driver's error crosses into this adapter's error enum through
    // `#[from]`, so if this ever stopped holding the adapter's would too.
    assert_send::<Error>();
    assert_sync::<Error>();
}

/// The adapter's own types, which is the half a caller feels.
///
/// `tests/port_shape.rs` shows *why* these matter — `tokio::spawn` rejects a
/// runner that holds a batch across an await unless `S::Batch: Send` — and this
/// is where the bound is discharged for this concrete store rather than assumed
/// of a generic one.
#[test]
fn the_adapters_types_are_send_and_sync() {
    assert_send::<LadybugProjectionStore>();
    assert_sync::<LadybugProjectionStore>();

    // The batch is the whole point of the owned shape: it holds no connection,
    // no transaction and no borrow, so it can cross a thread and be held across
    // an await by a runner that has not yet decided to commit.
    assert_send::<GraphWriteSet>();
    assert_sync::<GraphWriteSet>();
    assert_send::<GraphStatement>();

    assert_send::<LadybugProjectionStoreError>();
    assert_sync::<LadybugProjectionStoreError>();
}
