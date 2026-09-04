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

use happenstance_testkit::fixtures::MemoryFixture;

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
