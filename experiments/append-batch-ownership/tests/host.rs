//! Host-side conformance: the claims the measurement rests on, checked before a
//! single figure is printed. `run.sh` runs this first and tees it to
//! `results/raw/conformance-host.txt`.
//!
//! The counting allocator's counters are process-global
//! (`event-clone-allocations/src/counting.rs`), so every test here takes
//! [`region::hold`] on its first line and keeps it to its last: one test's
//! allocations cannot land in another's region, and plain `cargo test` passes
//! under the default parallel harness. `run.sh` still passes
//! `--test-threads=1`, which also keeps the harness's own threads quiet.
//!
//! An arm that is cheap because it does less is not a faster arm. Each test
//! names the wrong implementation it rejects.

#![cfg(not(target_arch = "wasm32"))]

use append_batch_ownership::batch::{self, Regime, Shape};
use append_batch_ownership::memory_arms::{BorrowedLog, OwnedLog};
use append_batch_ownership::poll::now_or_never;
use append_batch_ownership::region;
use happenstance_core::MemoryEventStore;
use happenstance_core::bytes::Bytes;
use happenstance_core::{Event, EventStore, Query, ReadOptions, StoreId};

const STORE: StoreId = StoreId::from_bytes([3; 16]);

fn shape(batch: usize, tags: usize, regime: Regime) -> Shape {
    Shape {
        batch,
        tags,
        payload: 1024,
        regime,
    }
}

/// Rejects: a "move" that silently copies. If `Vec::from(Bytes)` allocated, an
/// owned arm in the `Vec`-backed regime would pay the copy it claims to save
/// and the measurement would understate it; if it kept the pointer by copying
/// into a fresh buffer, `as_ptr` would differ.
#[test]
fn vec_backed_bytes_into_vec_moves_without_allocating() {
    let mut region = region::hold();
    let source: Vec<u8> = (0..4096_u16).map(|n| n.to_le_bytes()[0]).collect();
    let pointer = source.as_ptr();
    let bytes = Bytes::from(source);

    let (back, counts) = region.measure(|| Vec::<u8>::from(bytes));

    assert_eq!(back.as_ptr(), pointer, "the buffer came back, not a copy");
    assert_eq!(counts.heap_ops(), 0, "and it allocated nothing: {counts}");
}

/// The other regime, pinned so it cannot be quietly dropped from the report:
/// a static payload is *copied* by the owned arm's conversion, so owning the
/// batch saves nothing for it.
#[test]
fn static_bytes_into_vec_copies() {
    let mut region = region::hold();
    static PAYLOAD: [u8; 4096] = [0x5a; 4096];
    let bytes = Bytes::from_static(&PAYLOAD);

    let (back, counts) = region.measure(|| Vec::<u8>::from(bytes));

    assert_ne!(back.as_ptr(), PAYLOAD.as_ptr());
    assert_eq!(counts.allocs, 1, "one allocation for the copy: {counts}");
    assert_eq!(counts.bytes, 4096);
}

/// Rejects: a borrowed replica that is not the reference store's write path.
/// From an empty log, `BorrowedLog` must allocate exactly what
/// `MemoryEventStore::append` allocates for the same batch.
#[test]
fn borrowed_log_matches_memory_store_allocations() {
    let mut region = region::hold();
    for tags in [1, 8, 64] {
        // Two batches built alike, never one shared: the first clone of a
        // `Vec`-backed `Bytes` promotes it to the shared representation (one
        // allocation per buffer), so a batch the real store had already cloned
        // would let the replica skip that cost. That is how this test first
        // went red, by 2 allocations per event (641 against 385 at one tag).
        let for_real = batch::build(shape(128, tags, Regime::VecBacked), 0).expect("builds");
        let for_copy = batch::build(shape(128, tags, Regime::VecBacked), 0).expect("builds");
        let store = MemoryEventStore::with_store_id(STORE);
        let mut replica = BorrowedLog::new(STORE);

        let (real, real_counts) = region.measure(|| now_or_never(store.append(&for_real, None)));
        let (copy, copy_counts) = region.measure(|| replica.append(&for_copy, None));

        assert!(matches!(real, Some(Ok(_))), "the real store appended");
        assert!(copy.is_ok(), "the replica appended");
        assert_eq!(
            real_counts.heap_ops(),
            copy_counts.heap_ops(),
            "tags={tags}: real {real_counts} vs replica {copy_counts}"
        );
        assert_eq!(real_counts.bytes, copy_counts.bytes, "tags={tags}");
    }
}

/// Rejects: an owned replica that is fast because it stores something else.
/// Both replicas and the real store must hold the same events, whole.
#[test]
fn both_logs_hold_what_the_memory_store_holds() {
    // Counts nothing, but allocates: held so it cannot land in a counting region.
    let _region = region::hold();
    let built = || batch::build(shape(16, 8, Regime::VecBacked), 11).expect("builds");
    let store = MemoryEventStore::with_store_id(STORE);
    let mut borrowed = BorrowedLog::new(STORE);
    let mut owned = OwnedLog::new(STORE);

    now_or_never(store.append(&built(), None))
        .expect("never pends")
        .expect("appends");
    borrowed.append(&built(), None).expect("appends");
    owned.append(built(), None).expect("appends");

    let real: Vec<Event> = read_all(&store);
    let of = |rows: &[happenstance_core::SequencedEvent]| -> Vec<Event> {
        rows.iter().map(|row| row.event.clone()).collect()
    };
    assert_eq!(real, built());
    assert_eq!(of(borrowed.events()), real);
    assert_eq!(of(owned.events()), real);
}

/// Rejects: an owned replica that "saves" allocations by not storing the
/// event — in the `Vec` regime it must save exactly what one clone of the batch
/// costs (`t + 2` per event, plus one promotion per payload and metadata buffer
/// on its first clone), less the clone's outer `Vec`, and no more.
#[test]
fn owned_log_saves_exactly_the_clone() {
    let mut region = region::hold();
    // Three independent batches, for the promotion reason given in
    // `borrowed_log_matches_memory_store_allocations`. The clone is measured,
    // not kept: it is the price the borrowed arm pays, named.
    let built = || batch::build(shape(128, 8, Regime::VecBacked), 0).expect("builds");
    let (for_borrowed, for_owned, for_clone) = (built(), built(), built());
    let mut borrowed = BorrowedLog::new(STORE);
    let mut owned = OwnedLog::new(STORE);
    let clone_cost = region.measure(|| for_clone.clone()).1;

    let (_, borrowed_counts) = region.measure(|| borrowed.append(&for_borrowed, None));
    let (_, owned_counts) = region.measure(|| owned.append(for_owned, None));

    assert_eq!(
        borrowed_counts.heap_ops() - owned_counts.heap_ops(),
        clone_cost.heap_ops() - 1,
        "the difference is the per-event clone, without the outer Vec: \
         borrowed {borrowed_counts} owned {owned_counts} clone {clone_cost}"
    );
    // And its bytes: the clone's, less the outer `Vec`'s buffer, which `clone`
    // sizes to the batch exactly.
    let outer = u64::try_from(std::mem::size_of_val(for_clone.as_slice())).expect("fits");
    assert_eq!(
        borrowed_counts.bytes - owned_counts.bytes,
        clone_cost.bytes - outer,
        "the difference is the per-event clone's bytes, without the outer Vec: \
         borrowed {borrowed_counts} owned {owned_counts} clone {clone_cost}"
    );
}

fn read_all(store: &MemoryEventStore) -> Vec<Event> {
    use futures_core::Stream;
    let query = Query::all();
    let mut stream = std::pin::pin!(store.read(&query, ReadOptions::new()));
    let mut out = Vec::new();
    let mut cx = std::task::Context::from_waker(std::task::Waker::noop());
    while let std::task::Poll::Ready(Some(row)) = stream.as_mut().poll_next(&mut cx) {
        out.push(row.expect("memory never fails").event);
    }
    out
}
