//! The extension point CF-23 makes mandatory, held to the page that names it.
//!
//! # Why this file exists
//!
//! CF-23 `[FROZEN]` says the testkit MUST NOT emit any runtime-specific
//! attribute from its own expansion, and that the per-test wrapper MUST be a
//! parameter supplied by the adapter. The twelve `__emit_*` macros this crate
//! ships are the only concrete instances of that parameter anybody has, and
//! `crates/happenstance-cloudflare/tests/durable_object_conformance.rs` is the
//! in-tree proof that the cross-crate reach is already load-bearing: three
//! lines, one of which is `emit = happenstance_testkit::__emit_wasm`.
//!
//! An author is therefore *required* to write a name that no rendered page
//! shows them, because every one of those macros carries `#[doc(hidden)]`
//! immediately above `#[macro_export]`. The attribute is not decoration and it
//! is not removable: `macro_rules!` lives in a flat crate-root textual
//! namespace, so a private helper is unreachable from a downstream expansion
//! site, and hiding is the only tool the language offers for "exported because
//! it has to be". What the crate can control is whether the **page** names
//! them, and that is what this file checks.
//!
//! # The instrument that existed and did not fire
//!
//! `tests/memory_concurrency_conformance.rs`'s
//! `the_concurrency_page_lists_every_emitter_it_ships` is this check, written
//! for one family's module page after that page said *"one"* while two shipped.
//! It reads `src/concurrency.rs` and nothing else. The crate's **front** page
//! carries a table of the same shape, is the page CF-20's and CF-23's
//! population actually lands on, and was held to nothing — so it said *"Three
//! emitters ship"* while twelve did, and named five of them.
//!
//! # What this does not verify
//!
//! * **Whether those names are a promise.** `#[doc(hidden)]` is Rust's
//!   universal declaration that an item is not public API, and it is also the
//!   marker `cargo-semver-checks` uses to exclude an item from its analysis —
//!   so the one instrument in this repository that would report a rename of
//!   `__emit_wasm` as breaking is the one the attribute switches off. Whether
//!   the crate should support these names or declare them unstable is an open
//!   decision, argued in
//!   `.kb/_intake/remediation-2026-09-04-briefs/emitter-surface-stability.md`,
//!   and nothing here answers it. This file requires the page to *disclose* the
//!   attribute, which is true under either arm.
//! * **That the `Adapter needs` column is true.** Nothing here compiles a caller
//!   against a stated dependency set. The `tests/` invocations demonstrate five
//!   of the twelve; an emitter listed and never demonstrated passes.
//! * **Anything outside the crate's module documentation.** A name restated in
//!   an item's own doc comment is out of reach, exactly as it is for the sibling
//!   check one file over.

/// The files that define `__emit_*` macros.
const SOURCES: [(&str, &str); 5] = [
    ("src/registry.rs", include_str!("../src/registry.rs")),
    ("src/projection.rs", include_str!("../src/projection.rs")),
    ("src/concurrency.rs", include_str!("../src/concurrency.rs")),
    ("src/bench.rs", include_str!("../src/bench.rs")),
    ("src/model.rs", include_str!("../src/model.rs")),
];

/// The crate's front page — the rendered surface an adapter author lands on.
const FRONT_PAGE: &str = include_str!("../src/lib.rs");

/// The attribute the page has to disclose, because a reader of the rendered
/// page cannot see it and it is what decides whether any tool would report a
/// rename.
const HIDDEN: &str = "doc(hidden)";

/// Every spelling of a count that could stand beside the emitter list.
const SPELLED: &[&str] = &[
    "no", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "eleven",
    "twelve", "thirteen",
];

/// Every `__emit_*` macro this crate defines, with the file that defines it.
fn shipped() -> Vec<(&'static str, String)> {
    SOURCES
        .iter()
        .flat_map(|(file, source)| {
            source.lines().filter_map(move |line| {
                line.trim()
                    .strip_prefix("macro_rules! __emit")
                    .and_then(|rest| rest.split_whitespace().next())
                    .map(|rest| (*file, format!("__emit{rest}")))
            })
        })
        .collect()
}

/// The crate's module documentation, `//!` markers stripped.
fn front_page_docs() -> Vec<&'static str> {
    FRONT_PAGE
        .lines()
        .filter_map(|line| line.trim_start().strip_prefix("//!"))
        .collect()
}

/// Every `__emit…` identifier the front page writes.
fn named_on_the_front_page() -> Vec<String> {
    let mut names = Vec::new();
    for line in front_page_docs() {
        let mut rest = line;
        while let Some(at) = rest.find("__emit") {
            rest = &rest[at..];
            let end = rest
                .find(|c: char| !(c.is_alphanumeric() || c == '_'))
                .unwrap_or(rest.len());
            names.push(rest[..end].to_owned());
            rest = &rest[end..];
        }
    }
    names
}

/// The scan is reading a corpus rather than an empty list (RS-81-4).
#[test]
fn the_scan_finds_the_emitters_it_is_about() {
    let shipped = shipped();
    assert!(
        shipped.len() >= 12,
        "this crate ships twelve `__emit_*` macros across four rule families and \
         the scan found {}: either a definition moved out of `SOURCES` or the \
         lexer has stopped reading them",
        shipped.len()
    );
    assert!(
        !front_page_docs().is_empty(),
        "the crate's module documentation is empty, so every assertion below \
         would pass over a page that says nothing"
    );
}

/// C2-03: the page an adapter author lands on names every emitter they may be
/// required to write.
///
/// **The wrong implementation this rejects shipped**: the front page's table
/// carried three rows, all of them the event-store family's, while twelve
/// emitters existed — so an author of a model, benchmark or concurrency harness
/// on a runtime other than tokio had no rendered name to write at all, and the
/// `#[doc(hidden)]` on the macro meant searching the API for one returned
/// nothing.
#[test]
fn the_front_page_names_every_emitter_this_crate_ships() {
    let named = named_on_the_front_page();
    let mut missing = Vec::new();
    for (file, emitter) in shipped() {
        if !named.contains(&emitter) {
            missing.push(format!("{emitter} ({file})"));
        }
    }
    assert!(
        missing.is_empty(),
        "these `__emit_*` macros ship and the crate's front page never names \
         them. CF-23 makes the wrapper an adapter-supplied parameter, and \
         `#[doc(hidden)]` means an author who searches the rendered API for one \
         finds no item, no signature and no stability statement — the page is \
         the whole of what they have: {missing:?}"
    );
}

/// The reverse direction: the page names nothing that does not exist.
#[test]
fn the_front_page_names_no_emitter_this_crate_does_not_ship() {
    let shipped: Vec<String> = shipped().into_iter().map(|(_, name)| name).collect();
    let mut absent = Vec::new();
    for name in named_on_the_front_page() {
        if !shipped.contains(&name) {
            absent.push(name);
        }
    }
    assert!(
        absent.is_empty(),
        "the front page tells an adapter author to write these, and this crate \
         defines none of them: {absent:?}"
    );
}

/// C2-03: the page discloses that the names it hands out are `#[doc(hidden)]`.
///
/// True under either arm of the open decision, which is why it is checkable
/// while the decision is not. The requirement is *derived*: it applies only
/// while the macros actually carry the attribute, so a release that stops
/// hiding them retires this assertion by itself.
#[test]
fn the_front_page_discloses_that_the_emitters_are_hidden() {
    let hidden = SOURCES
        .iter()
        .any(|(_, source)| source.lines().any(|line| line.trim() == "#[doc(hidden)]"));
    assert!(
        hidden,
        "no `__emit_*` macro carries `#[doc(hidden)]` any more, so this \
         assertion's premise is gone and it is what must move — not the page"
    );
    assert!(
        front_page_docs().iter().any(|line| line.contains(HIDDEN)),
        "every emitter the page tells an author to write carries \
         `#[doc(hidden)]`, which is Rust's declaration that an item is not \
         public API and is the marker `cargo-semver-checks` uses to exclude it \
         from its analysis. A reader of the rendered page cannot see the \
         attribute and cannot see the consequence. The page has to say it."
    );
}

/// No written-out count beside the emitter list.
///
/// The same rule the concurrency page's sibling check applies, and for the same
/// reason: a number is falsified by an edit that never touches it. This page
/// said *"Three emitters ship"* through the nine that landed after it.
#[test]
fn the_front_page_states_no_emitter_count() {
    for line in front_page_docs() {
        let words: Vec<String> = line
            .split_whitespace()
            .map(|w| {
                w.trim_matches(|c: char| !c.is_alphanumeric())
                    .to_ascii_lowercase()
            })
            .collect();
        for (i, word) in words.iter().enumerate() {
            if word != "emitter" && word != "emitters" {
                continue;
            }
            let Some(before) = i.checked_sub(1).map(|j| words[j].as_str()) else {
                continue;
            };
            assert!(
                !SPELLED.contains(&before),
                "`{before} {word}` is a count of a set this file can enumerate, \
                 written into prose where nothing enumerates it. The table's \
                 rows are the count. Line: {line:?}"
            );
        }
    }
}
