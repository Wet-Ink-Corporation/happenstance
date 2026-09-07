//! The concurrency family, against the reference implementation.
//!
//! `memory_conformance.rs`'s sibling, and it runs in the same two directions: it
//! checks that `MemoryEventStore` survives eight contenders, and — because a
//! concurrency suite no store can pass is a bug in the suite — that the rules
//! are a faithful reading of the contract rather than a stress test with
//! assertions bolted on.
//!
//! `MemoryEventStore` is an `RwLock` around a `Vec`, so it serialises its
//! writers and passes these rules for a structural reason rather than a lucky
//! one. That is worth being explicit about: **this harness is the demonstration
//! that the rules can be passed, not the demonstration that they can fail.**
//! The second half lives in `tests/mutation_coverage.rs`, where five racing
//! stores that are each wrong in one way are driven through the same five rules
//! and their verdicts pinned — `the_concurrency_rules_reject_exactly_what_they_claim`.
//!
//! Native only, and the `cfg` is load-bearing rather than tidy:
//! `happenstance_testkit::concurrency` does not exist on
//! `wasm32-unknown-unknown`, which has no threads to race on. An adapter with a
//! wasm32 harness gates its own invocation exactly like this.
//!
//! Both shipped emitters are exercised. CF-23's content is that the wrapper is a
//! parameter, and this family can have only two — `wasm_bindgen_test` is
//! unreachable for a rule set that does not exist on that target.

#![cfg(not(target_arch = "wasm32"))]

use core::future::Future;

use happenstance_testkit::Fixture as _;
use happenstance_testkit::fixtures::{MemoryFixture, MemoryHandle};

happenstance_testkit::event_store_concurrency_conformance!(MemoryFixture::new());

happenstance_testkit::event_store_concurrency_conformance!(
    mod_name = dcb_concurrency_conformance_blocking,
    emit = happenstance_testkit::__emit_concurrency_blocking,
    fixture = MemoryFixture::new()
);

/// The module page lists every emitter this family ships, and states no count.
///
/// M-2. `concurrency.rs`'s page opened with *"One emitter ships rather than
/// three"* for as long as two have shipped, and that paragraph is not a stale
/// comment — it is the **cost statement** an adapter author reads before
/// deciding whether to invoke this family. Told the only wrapper is
/// `#[tokio::test(flavor = "multi_thread")]`, an adapter with no runtime
/// concludes that racing costs it a `tokio` dev-dependency with
/// `rt-multi-thread`. It does not: `__emit_concurrency_blocking` needs nothing,
/// races exactly as hard, and is demonstrated by the second invocation in this
/// very file. The population that pays is the one the two-flavour design exists
/// for.
///
/// So the check is on the shape rather than on the number. The page carries a
/// table whose rows *are* the count, in `lib.rs:63-67`'s shape, and this test
/// holds those rows to the `macro_rules!` definitions in the same file — a
/// third emitter added without a row turns it red, which a written-out number
/// cannot do because a number is falsified by an edit that never touches it.
///
/// **Rejects: a page that tells a runtime-free adapter it must bring a
/// runtime.**
///
/// # What this does not verify
///
/// That the `Adapter needs` column is true — nothing here compiles a caller
/// against the stated dependency set; the second invocation above is what
/// demonstrates the blocking row, and an emitter added with a row and no
/// demonstration passes this test. And it reads only the module page, so a
/// count restated in an item's own doc comment is out of its reach.
#[test]
fn the_concurrency_page_lists_every_emitter_it_ships() {
    const PAGE: &str = include_str!("../src/concurrency.rs");
    // Every spelling a count beside the list could take. Hoisted above the
    // statements because `clippy::items_after_statements` is denied, which is
    // itself the house rule that an item is visible from the top of its scope
    // whatever line it is written on.
    const SPELLED: &[&str] = &["no", "one", "two", "three", "four", "1", "2", "3", "4"];

    let shipped: Vec<&str> = PAGE
        .lines()
        .filter_map(|line| line.trim().strip_prefix("macro_rules! __emit_concurrency_"))
        .filter_map(|rest| rest.split_whitespace().next())
        .collect();
    assert!(
        !shipped.is_empty(),
        "no `__emit_concurrency_*` macro was found in `concurrency.rs`, so this \
         test is reading the wrong file and would pass against a page that \
         listed nothing"
    );

    // The module page only: `//!`, which ends at the first line that is not one.
    let page: Vec<&str> = PAGE
        .lines()
        .take_while(|line| line.starts_with("//!") || line.trim().is_empty())
        .collect();

    let listed: Vec<&str> = page
        .iter()
        .filter_map(|line| line.trim().strip_prefix("//! |"))
        .filter_map(|row| row.split('|').next())
        .filter_map(|cell| cell.trim().strip_prefix("`__emit_concurrency_"))
        .filter_map(|rest| rest.split('`').next())
        .collect();

    for emitter in &shipped {
        assert!(
            listed.contains(emitter),
            "`__emit_concurrency_{emitter}` ships and the module page's emitter \
             table does not list it. The table is what an adapter author counts; \
             an emitter missing from it is a cost they are told they must pay \
             and need not."
        );
    }
    for emitter in &listed {
        assert!(
            shipped.contains(emitter),
            "the module page's emitter table lists \
             `__emit_concurrency_{emitter}`, which this file does not define"
        );
    }

    // And no count beside the list. A number written out is falsified by an
    // edit that never touches it, which is exactly how this page came to say
    // "one" while two shipped.
    for word in SPELLED {
        for phrase in [format!("{word} emitter"), format!("{word} emitters")] {
            assert!(
                !page.join(" ").to_lowercase().contains(&phrase),
                "the module page says {phrase:?}. The table below it is the \
                 count; a number written beside a list is the claim this \
                 finding was about"
            );
        }
    }
}

/// A read that fails under contention is named as a read that failed.
///
/// L2-04. `incomplete_batches` folds a failed read into the same `Vec<String>`
/// it fills with part-written batch names, and the rule's only assertion over
/// that vector reports it under ES-18: *"either every event of a batch is
/// visible or none is"*, with a `[FROZEN]` clause id attached. An adapter whose
/// read transiently fails while a writer is working — `SQLITE_BUSY` past the
/// handler's ceiling, a pool with no reader slot, a 503 from a one-shot HTTP
/// backend — is told by name that its `append` is writing rows outside a
/// transaction, and goes looking for a missing `BEGIN` in code that has one.
///
/// The helper's own documentation is right that a read failing under contention
/// is a defect this rule is entitled to name. What is wrong is the channel, and
/// the pattern for fixing it is 260 lines above in the same file: two opposite
/// defects get two assertions, because *"a single message describing both is a
/// message that identifies neither"*.
///
/// **Rejects: a correct assertion printing an incorrect diagnosis.**
#[test]
#[should_panic(expected = "could not be read while an append was in flight")]
fn a_read_that_fails_under_contention_is_not_reported_as_a_partial_batch() {
    happenstance_testkit::block_on(
        happenstance_testkit::concurrency::rules::a_concurrent_reader_never_sees_a_partial_batch(
            || async { FlakyReadFixture::new() },
        ),
    )
    .report("a_concurrent_reader_never_sees_a_partial_batch");
}

/// A store whose every read fails, and which is otherwise the reference one.
///
/// The input no in-process store produces, and the only way to reach the
/// read-failure arm of `incomplete_batches`: `MemoryEventStore` cannot fail a
/// read, and no registered racer makes one fail — the arm was reachable in
/// principle and by nothing in the tree. `u32::MAX` armings rather than one
/// because the reader polls in a loop and the rule takes a throwaway reading
/// before the loop starts.
#[derive(Debug)]
struct FlakyReadFixture(happenstance_testkit::SendFaultyStore<MemoryHandle>);

impl FlakyReadFixture {
    fn new() -> Self {
        let handle = happenstance_testkit::block_on(MemoryFixture::new().connect());
        Self(happenstance_testkit::SendFaultyStore::new(handle).fail_next_read(u32::MAX))
    }
}

impl happenstance_testkit::Fixture for FlakyReadFixture {
    type Store = happenstance_testkit::SendFaultyStore<MemoryHandle>;

    const SECOND_HANDLE: happenstance_testkit::Capability =
        happenstance_testkit::Capability::SUPPORTED;

    const REOPEN: happenstance_testkit::Capability = happenstance_testkit::Capability::declined(
        "the wrapped store is a MemoryEventStore, so there is no durable medium \
         to reopen over and the wrapper adds none",
    );

    fn connect(&self) -> impl Future<Output = Self::Store> {
        core::future::ready(self.0.clone())
    }
}
