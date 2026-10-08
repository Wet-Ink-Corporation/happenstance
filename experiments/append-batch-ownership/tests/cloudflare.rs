//! Cloudflare conformance: every claim the timing rests on, checked on the target
//! the timing runs on, before a figure is printed. `run.sh` runs this first and
//! tees it to `results/raw/conformance-cloudflare.txt`.
//!
//! ```console
//! $ CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER=wasm-bindgen-test-runner \
//!     cargo test --release --target wasm32-unknown-unknown --test cloudflare
//! ```
//!
//! Each test names the wrong implementation it rejects (`p17-es17-brief.md`,
//! "Wrong implementations the experiment's tests must reject").

#![cfg(target_arch = "wasm32")]

use append_batch_ownership::batch::{self, METADATA_LEN, Regime, Shape};
use append_batch_ownership::cloudflare::{ArmError, B0, B1, Fence, O1, Subject};
use append_batch_ownership::measured::measure;
use append_batch_ownership::poll::now_or_never;
use happenstance_cloudflare::CloudflareEventStore;
use happenstance_core::bytes::Bytes;
use happenstance_core::{Event, EventStore, Query, ReadOptions, Tag};
use wasm_bindgen_test::wasm_bindgen_test;

fn shape(batch: usize, tags: usize, payload: usize, regime: Regime) -> Shape {
    Shape {
        batch,
        tags,
        payload,
        regime,
    }
}

fn built(shape: Shape) -> Vec<Event> {
    batch::build(shape, 5).expect("the sweep's constants build")
}

/// Every event the real adapter reads back from `subject`'s object, whole.
fn read_back(store: &CloudflareEventStore) -> Vec<Event> {
    use futures_core::Stream;
    let query = Query::all();
    let mut stream = std::pin::pin!(store.read(&query, ReadOptions::new()));
    let mut out = Vec::new();
    let mut cx = std::task::Context::from_waker(std::task::Waker::noop());
    while let std::task::Poll::Ready(Some(row)) = stream.as_mut().poll_next(&mut cx) {
        out.push(
            row.expect("the real read path decodes every arm's rows")
                .event,
        );
    }
    out
}

/// Rejects: a "move" that silently copies, on the target measured. Pinned
/// here as well as on the host because the allocator underneath is dlmalloc,
/// not the host's.
#[wasm_bindgen_test]
fn a_vec_backed_payload_moves_without_allocating_on_wasm32() {
    let source: Vec<u8> = vec![7; 4096];
    let pointer = source.as_ptr();
    let bytes = Bytes::from(source);

    let (back, counts) = measure(|| Vec::<u8>::from(bytes));

    assert_eq!(back.as_ptr(), pointer);
    assert_eq!(counts.heap_ops(), 0, "{counts}");
}

/// Rejects: an owned arm that is fast because it loses data — drops
/// metadata, reorders tags, truncates a payload. Every arm's rows are read
/// back through the real `CloudflareEventStore::read` and compared as whole
/// `Event`s with the batch, in both regimes, at a tag count above one.
#[wasm_bindgen_test]
fn every_arm_reads_back_the_batch_through_the_real_store() {
    for regime in [Regime::VecBacked, Regime::Static] {
        let shape = shape(16, 8, 1024, regime);
        let expected = built(shape);
        for arm in ["real", "b0", "b1", "o1"] {
            let subject = Subject::open().expect("a fresh object migrates");
            let outcome = match arm {
                "real" => now_or_never(subject.real().append(&built(shape), None))
                    .expect("the adapter never suspends")
                    .map_err(|err| format!("{err:?}")),
                "b0" => B0::over(&subject)
                    .and_then(|b0| b0.append(&built(shape), None))
                    .map_err(|err| format!("{err:?}")),
                "b1" => B1::over(&subject)
                    .and_then(|b1| b1.append(&built(shape), None))
                    .map_err(|err| format!("{err:?}")),
                _ => O1::over(&subject)
                    .and_then(|o1| o1.append_owned(built(shape), None))
                    .map_err(|err| format!("{err:?}")),
            };
            assert!(outcome.is_ok(), "{arm} appended: {outcome:?}");
            let stored = read_back(subject.real());
            assert_eq!(stored.len(), expected.len(), "{arm} {}", regime.label());
            assert_eq!(
                stored,
                expected,
                "{arm} {} stored the batch whole",
                regime.label()
            );
            assert!(
                stored
                    .iter()
                    .all(|event| event.metadata().map(Bytes::len) == Some(METADATA_LEN)),
                "{arm} kept the metadata"
            );
        }
    }
}

/// Rejects: a B0 replica that is not the adapter. For the same batch on a
/// fresh object, B0 must make exactly the heap operations and request exactly
/// the bytes `CloudflareEventStore::append` does — at every tag count the
/// sweep uses, in both regimes.
#[wasm_bindgen_test]
fn b0_allocates_exactly_what_the_adapter_allocates() {
    for regime in [Regime::VecBacked, Regime::Static] {
        for tags in [1, 8, 64] {
            let shape = shape(16, tags, 1024, regime);
            let real_subject = Subject::open().expect("migrates");
            let b0_subject = Subject::open().expect("migrates");
            let b0 = B0::over(&b0_subject).expect("reads the identity");
            let (for_real, for_b0) = (built(shape), built(shape));

            let (real, real_counts) =
                measure(|| now_or_never(real_subject.real().append(&for_real, None)));
            let (copy, copy_counts) = measure(|| b0.append(&for_b0, None));

            assert!(matches!(real, Some(Ok(_))), "the adapter appended");
            assert!(copy.is_ok(), "B0 appended: {copy:?}");
            assert_eq!(
                (real_counts.heap_ops(), real_counts.bytes),
                (copy_counts.heap_ops(), copy_counts.bytes),
                "{shape}: adapter {real_counts} vs B0 {copy_counts}"
            );
        }
    }
}

/// The bytes the `to_binding` clone copies for one batch: every `Text` and
/// `Blob` bound, per event row, per tag row and once for the stamp.
fn binding_clone_bytes(events: &[Event]) -> u64 {
    let per_event = |event: &Event| {
        let type_len = event.event_type().as_str().len();
        let tags_blob = 1 + event
            .tags()
            .iter()
            .map(|tag| tag.as_str().len() + 1)
            .sum::<usize>();
        let tag_rows: usize = event
            .tags()
            .iter()
            .map(|tag: &Tag| tag.as_str().len() + type_len)
            .sum();
        type_len
            + event.data().len()
            + event.metadata().map_or(0, Bytes::len)
            + tags_blob
            + tag_rows
    };
    let stamp = 16;
    u64::try_from(events.iter().map(per_event).sum::<usize>() + stamp).expect("fits")
}

/// Rejects: a B1 that saves more than the clone it claims to remove — a mirror
/// of `SqlStorage::exec` that skips the cursor's bookkeeping would be faster
/// for a reason unrelated to the binding. B0 − B1 must be exactly the
/// `to_binding` clones: `batch × (4 + 2t) + 1` heap operations, and their
/// bytes.
#[wasm_bindgen_test]
fn b1_is_b0_minus_the_binding_clones() {
    for tags in [1, 8, 64] {
        let shape = shape(16, tags, 1024, Regime::VecBacked);
        let subject = Subject::open().expect("migrates");
        let (b0, b1) = (
            B0::over(&subject).expect("identity"),
            B1::over(&subject).expect("identity"),
        );
        let (for_b0, for_b1) = (built(shape), built(shape));

        let (_, b0_counts) = measure(|| b0.append(&for_b0, None));
        let (_, b1_counts) = measure(|| b1.append(&for_b1, None));

        let clones = u64::try_from(shape.batch * (4 + 2 * tags) + 1).expect("fits");
        assert_eq!(
            b0_counts.heap_ops() - b1_counts.heap_ops(),
            clones,
            "{shape}: B0 {b0_counts} B1 {b1_counts}"
        );
        assert_eq!(
            b0_counts.bytes - b1_counts.bytes,
            binding_clone_bytes(&for_b0),
            "{shape}"
        );
    }
}

/// Rejects: an O1 that saves something other than the moved buffers, and an
/// O1 that claims a saving in the regime where a move is impossible. In the
/// `Vec` regime O1 is B1 less one allocation each for data and metadata per
/// event, and their bytes; in the static regime it is B1 exactly.
#[wasm_bindgen_test]
fn o1_is_b1_minus_the_moved_buffers_in_the_vec_regime_only() {
    for regime in [Regime::VecBacked, Regime::Static] {
        let shape = shape(16, 8, 1024, regime);
        let subject = Subject::open().expect("migrates");
        let (b1, o1) = (
            B1::over(&subject).expect("identity"),
            O1::over(&subject).expect("identity"),
        );
        let (lent, given) = (built(shape), built(shape));

        let (_, b1_counts) = measure(|| b1.append(&lent, None));
        let (_, o1_counts) = measure(|| o1.append_owned(given, None));

        let (ops, bytes) = match regime {
            Regime::VecBacked => (
                u64::try_from(2 * shape.batch).expect("fits"),
                u64::try_from(shape.batch * (shape.payload + METADATA_LEN)).expect("fits"),
            ),
            Regime::Static => (0, 0),
        };
        assert_eq!(
            (
                b1_counts.heap_ops() - o1_counts.heap_ops(),
                b1_counts.bytes - o1_counts.bytes
            ),
            (ops, bytes),
            "{shape}: B1 {b1_counts} O1 {o1_counts}"
        );
    }
}

/// Rejects: a contention scenario whose fence does not fence. A contender
/// that decided before another committed must be refused, and must leave the
/// object as it found it; one that decided after must land.
#[wasm_bindgen_test]
fn a_fenced_append_is_refused_once_the_boundary_has_moved() {
    let shape = shape(4, 8, 64, Regime::VecBacked);
    let subject = Subject::open().expect("migrates");
    let (b1, o1) = (
        B1::over(&subject).expect("identity"),
        O1::over(&subject).expect("identity"),
    );
    let boundary = Fence::on_boundary(None).expect("the boundary tag is valid");

    let first = b1
        .append(&built(shape), Some(&boundary))
        .expect("nothing is past the boundary yet");
    let borrowed_late = b1.append(&built(shape), Some(&boundary));
    let owned_late = o1.append_owned(built(shape), Some(&boundary));
    let fresh = Fence::on_boundary(Some(first)).expect("valid");
    let second = o1.append_owned(built(shape), Some(&fresh));

    assert!(
        matches!(borrowed_late, Err(ArmError::Conflict(at)) if at == first),
        "{borrowed_late:?}"
    );
    assert!(
        matches!(owned_late, Err(ArmError::Conflict(at)) if at == first),
        "{owned_late:?}"
    );
    assert!(second.is_ok(), "{second:?}");
    assert_eq!(read_back(subject.real()).len(), 2 * shape.batch);
}
