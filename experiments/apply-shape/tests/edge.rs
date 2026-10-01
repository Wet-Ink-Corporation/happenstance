//! The `!Send` end, run on the host.
//!
//! `src/edge.rs` is what the wasm32 check type-checks; this runs it, so the
//! `!Send` case is shown to *work* and not only to compile. Awaited in place on
//! a current-thread runtime — nothing is spawned, which is the only way a
//! `!Send` future is ever driven.

#![allow(clippy::unwrap_used)]

use apply_shape::domain::{POISON, tick, undecodable};
use apply_shape::edge::{EdgeStore, EdgeTally, run_edge};
use happenstance::MemoryEventStore;

#[tokio::test(flavor = "current_thread")]
async fn a_non_send_projection_over_a_non_send_store_runs_and_skips() {
    let events = MemoryEventStore::with_events([
        tick("a").unwrap(),
        tick(POISON).unwrap(),
        undecodable().unwrap(),
        tick("a").unwrap(),
    ]);
    let head = events.last_position();
    let store = EdgeStore::default();
    let mut tally = EdgeTally::new("edge");

    let ran = run_edge(&events, &store, &mut tally).await.unwrap();

    assert_eq!((ran.applied, ran.skipped), (2, 2));
    assert_eq!(ran.through, head);
    assert_eq!(store.get("a"), Some(2));
    assert_eq!(store.get("skipped"), Some(2));
    // The decode failure never reached `apply`, so three ids were seen, not four.
    assert_eq!(tally.seen.borrow().len(), 3);
}
