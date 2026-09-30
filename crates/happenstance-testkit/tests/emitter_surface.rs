//! The extension point CF-23 makes mandatory, held to the list CF-41 promises.
//!
//! # Why this file exists
//!
//! CF-23 `[FROZEN]` says the testkit MUST NOT emit any runtime-specific
//! attribute from its own expansion, and that the per-test wrapper MUST be a
//! parameter supplied by the adapter. The emitter macros this crate ships are
//! the only concrete instances of that parameter anybody has, and
//! `crates/happenstance-cloudflare/tests/durable_object_conformance.rs` is the
//! in-tree proof that the cross-crate reach is load-bearing: three lines, one
//! of which is `emit = happenstance_testkit::emit_wasm`.
//!
//! A name an adapter author is *required* to write is a name they rely on, so
//! CF-41 promises it: the conformance emitters are ordinary documented macros,
//! and renaming or removing one is a major release (ADR-0076). Until `0.4.0`
//! every one of them carried `#[doc(hidden)]` and a `__` prefix, which is
//! Rust's declaration that an item is *not* public API — and is also the marker
//! `cargo-semver-checks` uses to exclude an item, so the one tool that would
//! have reported a rename could not see these names at all.
//!
//! # What it checks
//!
//! * **The promised set is pinned.** [`PROMISED`] is a committed list, and the
//!   exported, rendered emitters must equal it exactly. A rename changes both
//!   the definition and every in-tree caller in one edit, so no compile and no
//!   derived comparison notices it; a list written down separately does.
//! * **`__` means not promised, in both directions.** Every macro the crate
//!   exports, emitter or not and `src/lib.rs` included, carries
//!   `#[doc(hidden)]` if its name starts with `__`, and starts with `__` if it
//!   carries `#[doc(hidden)]`. So a hidden name cannot be promised
//!   and a promised name cannot be hidden, which is the contradiction CF-23 and
//!   the pre-`0.4.0` names published at one commit.
//! * **The front page names every emitter, names nothing else, and states no
//!   count of them.** That is the check this file was first written for, and
//!   it still earns its place: the page once said *"three"* while twelve
//!   shipped.
//!
//! # What this does not verify
//!
//! * **That the `Adapter needs` column is true.** Nothing here compiles a caller
//!   against a stated dependency set.
//! * **Anything outside `SOURCES`, on `wasm32`.** On the host,
//!   `every_exporting_file_is_in_sources` reads `src/` and fails on any file
//!   that has a `#[macro_export]` and is not in this list.
//!   `the_scan_finds_the_emitters_it_is_about` covers the other side.
//! * **An attribute split across lines.** The scan reads one attribute per
//!   line, so it misses a `#[cfg_attr(…,` whose `doc(hidden))]` is on the next
//!   line. rustfmt keeps each attribute this crate uses on a single line.
//! * **What the suite macros define around an emitter.** A hand-written emitter
//!   names `__conformance_fixture` and `$crate::__private`, and neither is
//!   promised; the front page says so, and nothing here holds it.

/// The files that export macros: every emitter, and the suite macros beside
/// them.
const SOURCES: [(&str, &str); 6] = [
    ("src/lib.rs", include_str!("../src/lib.rs")),
    ("src/registry.rs", include_str!("../src/registry.rs")),
    ("src/projection.rs", include_str!("../src/projection.rs")),
    ("src/concurrency.rs", include_str!("../src/concurrency.rs")),
    ("src/bench.rs", include_str!("../src/bench.rs")),
    ("src/model.rs", include_str!("../src/model.rs")),
];

/// The crate's front page — the rendered surface an adapter author lands on.
const FRONT_PAGE: &str = include_str!("../src/lib.rs");

/// CF-41's promised set: the conformance emitters, by name.
///
/// **Committed, not derived, and that is the point.** Removing a name from this
/// list is a major release of `happenstance-testkit`; adding one is a minor.
/// Editing it is how a change to the promise is made visible in review.
const PROMISED: [&str; 10] = [
    "emit_tokio",
    "emit_blocking",
    "emit_wasm",
    "emit_projection_tokio",
    "emit_projection_blocking",
    "emit_projection_wasm",
    "emit_model_tokio",
    "emit_model_blocking",
    "emit_concurrency_tokio",
    "emit_concurrency_blocking",
];

/// Exported, and deliberately outside the promise (ADR-0076).
///
/// The benchmark pair because a benchmark is not the bar (CF-34), and
/// `__rule_names` because it wraps no test.
const EXEMPT: [&str; 3] = [
    "__emit_benchmark_tokio",
    "__emit_benchmark_blocking",
    "__rule_names",
];

/// Every spelling of a count that could stand beside the emitter list.
const SPELLED: &[&str] = &[
    "no", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "eleven",
    "twelve", "thirteen",
];

/// One `#[macro_export]` macro, as its source declares it.
#[derive(Debug)]
struct Exported {
    file: &'static str,
    name: String,
    hidden: bool,
}

/// Every `#[macro_export]` macro in `source`, and whether its preamble hides
/// it.
///
/// The preamble is the whole unbroken run of attribute and comment lines
/// directly above `macro_rules!`. A doc comment does not end it, because a
/// `#[doc(hidden)]` above a `///` block hides the macro as surely as one below
/// it. Any attribute in the run that mentions `doc(hidden)` counts, which
/// covers the `cfg_attr(…, doc(hidden))` spelling.
fn exported_in(file: &'static str, source: &str) -> Vec<Exported> {
    let lines: Vec<&str> = source.lines().map(str::trim).collect();
    let mut found = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        let Some(rest) = line.strip_prefix("macro_rules! ") else {
            continue;
        };
        let Some(name) = rest.split_whitespace().next() else {
            continue;
        };
        let attributes: Vec<&str> = lines[..i]
            .iter()
            .rev()
            .take_while(|l| l.starts_with("#[") || l.starts_with("//"))
            .copied()
            .filter(|l| l.starts_with("#["))
            .collect();
        if attributes.iter().any(|a| a.starts_with("#[macro_export")) {
            found.push(Exported {
                file,
                name: name.to_owned(),
                hidden: attributes.iter().any(|a| a.contains("doc(hidden)")),
            });
        }
    }
    found
}

/// Whether an exported macro name belongs to the emitter surface this file is
/// about.
fn is_emitter_surface(name: &str) -> bool {
    name.starts_with("emit_") || name.starts_with("__emit_") || name == "__rule_names"
}

/// Every exported macro across `SOURCES`.
fn exported() -> Vec<Exported> {
    SOURCES
        .iter()
        .flat_map(|(file, source)| exported_in(file, source))
        .collect()
}

/// Every exported emitter-surface macro across `SOURCES`.
fn shipped() -> Vec<Exported> {
    exported()
        .into_iter()
        .filter(|m| is_emitter_surface(&m.name))
        .collect()
}

/// The part of CF-41 that applies to every exported macro, not only the
/// emitters: `__` and `#[doc(hidden)]` go together.
fn prefix_violations(exported: &[Exported]) -> Vec<String> {
    exported
        .iter()
        .filter(|m| m.name.starts_with("__") != m.hidden)
        .map(|m| {
            format!(
                "`{}` ({}) breaks the rule that `__` means *not promised*: a `__` \
                 name must be `#[doc(hidden)]` and an unprefixed name must not be",
                m.name, m.file
            )
        })
        .collect()
}

/// CF-41's rule as a function over what ships, so that its negative controls
/// can hand it the shapes it must refuse.
///
/// Returns every violation found; an empty list is a pass.
fn cf_41_violations(shipped: &[Exported]) -> Vec<String> {
    let mut violations = Vec::new();
    for promised in PROMISED {
        match shipped.iter().find(|m| m.name == promised) {
            None => violations.push(format!(
                "`{promised}` is promised by CF-41 and no longer ships: a removal or \
                 a rename is a major release of this crate, and needs a record"
            )),
            Some(m) if m.hidden => violations.push(format!(
                "`{promised}` ({}) is promised and carries `#[doc(hidden)]`, which \
                 takes it off docs.rs and out of `cargo-semver-checks`",
                m.file
            )),
            Some(_) => {}
        }
    }
    for m in shipped {
        let promised = PROMISED.contains(&m.name.as_str());
        let exempt = EXEMPT.contains(&m.name.as_str());
        if !promised && !exempt {
            violations.push(format!(
                "`{}` ({}) is exported and is neither in `PROMISED` nor in `EXEMPT`: \
                 decide which, and write it down",
                m.name, m.file
            ));
        }
    }
    violations.extend(prefix_violations(shipped));
    violations
}

/// The crate's module documentation, `//!` markers stripped.
fn front_page_docs() -> Vec<&'static str> {
    FRONT_PAGE
        .lines()
        .filter_map(|line| line.trim_start().strip_prefix("//!"))
        .collect()
}

/// Every emitter-surface identifier the front page writes.
fn named_on_the_front_page() -> Vec<String> {
    let mut names = Vec::new();
    for line in front_page_docs() {
        let words = line.split(|c: char| !(c.is_alphanumeric() || c == '_'));
        names.extend(
            words
                .filter(|word| is_emitter_surface(word))
                .map(str::to_owned),
        );
    }
    names
}

/// The scan is reading a corpus rather than an empty list (RS-81-4).
#[test]
fn the_scan_finds_the_emitters_it_is_about() {
    let shipped = shipped();
    assert!(
        shipped.len() >= PROMISED.len() + EXEMPT.len(),
        "the scan found {} exported emitter macros, fewer than the {} this file \
         names: either a definition moved out of `SOURCES` or the lexer has \
         stopped reading them",
        shipped.len(),
        PROMISED.len() + EXEMPT.len()
    );
    assert!(
        !front_page_docs().is_empty(),
        "the crate's module documentation is empty, so every assertion below \
         would pass over a page that says nothing"
    );
}

/// CF-41: the promised emitters are exactly the pinned list, rendered, and no
/// `__` name is promised.
///
/// **The wrong implementation this rejects shipped** until `0.4.0`: every
/// emitter CF-23 obliged an adapter to name carried `#[doc(hidden)]` and a `__`
/// prefix, so a `[FROZEN]` clause required a name the crate's own convention
/// said could vanish, and the one tool that would report its removal was
/// switched off by the attribute.
#[test]
fn the_promised_emitters_are_exactly_the_pinned_list() {
    let mut violations = cf_41_violations(&shipped());
    for v in prefix_violations(&exported()) {
        if !violations.contains(&v) {
            violations.push(v);
        }
    }
    assert!(
        violations.is_empty(),
        "CF-41 is violated:\n{}",
        violations.join("\n")
    );
}

/// The negative control: the `0.3.2` shape — `emit_wasm` spelled `__emit_wasm`
/// and hidden — fails the rule, on both of its counts.
#[test]
fn the_rule_refuses_the_hidden_names_that_shipped_before_it() {
    let old_shape = "\
/// Emits one `#[wasm_bindgen_test]` per rule.
#[doc(hidden)]
#[macro_export]
macro_rules! __emit_wasm {
";
    let mut shipped: Vec<Exported> = shipped()
        .into_iter()
        .filter(|m| m.name != "emit_wasm")
        .collect();
    shipped.extend(exported_in("0.3.2 registry.rs", old_shape));
    let violations = cf_41_violations(&shipped);
    assert!(
        violations
            .iter()
            .any(|v| v.contains("`emit_wasm` is promised")),
        "a promised emitter renamed away passed CF-41's rule: {violations:?}"
    );
    assert!(
        violations.iter().any(|v| v.contains("`__emit_wasm`")),
        "an exported emitter in neither list passed CF-41's rule: {violations:?}"
    );
}

/// The negative control for the other direction: a promised name that is
/// still hidden, and an exempt name that is rendered.
#[test]
fn the_rule_refuses_a_hidden_promise_and_a_rendered_exemption() {
    let hidden_promise = "\
#[doc(hidden)]
#[macro_export]
macro_rules! emit_blocking {
";
    let rendered_exemption = "\
#[macro_export]
macro_rules! __rule_names {
";
    let mut shipped: Vec<Exported> = shipped()
        .into_iter()
        .filter(|m| m.name != "emit_blocking" && m.name != "__rule_names")
        .collect();
    shipped.extend(exported_in("control", hidden_promise));
    shipped.extend(exported_in("control", rendered_exemption));
    let violations = cf_41_violations(&shipped);
    assert!(
        violations
            .iter()
            .any(|v| v.contains("`emit_blocking` (control) is promised and carries")),
        "a promised emitter carrying `#[doc(hidden)]` passed: {violations:?}"
    );
    assert!(
        violations
            .iter()
            .any(|v| v.contains("`__rule_names` (control) breaks the rule")),
        "a `__` name without `#[doc(hidden)]` passed: {violations:?}"
    );
}

/// The negative control for hiding attributes the scan once missed: a
/// `#[doc(hidden)]` placed above the doc block rather than below it, and the
/// `cfg_attr` spelling. Both hide the macro, so both must fail.
#[test]
fn the_rule_sees_a_doc_hidden_above_the_docs_and_under_cfg_attr() {
    let above_the_docs = "\
#[doc(hidden)]
/// Emits one `#[tokio::test]` per rule.
///
/// More prose.
#[macro_export]
macro_rules! emit_tokio {
";
    let under_cfg_attr = "\
/// Emits one `#[test]` per rule.
#[cfg_attr(not(doc), doc(hidden))]
#[macro_export]
macro_rules! emit_blocking {
";
    let mut shipped: Vec<Exported> = shipped()
        .into_iter()
        .filter(|m| m.name != "emit_tokio" && m.name != "emit_blocking")
        .collect();
    shipped.extend(exported_in("control", above_the_docs));
    shipped.extend(exported_in("control", under_cfg_attr));
    let violations = cf_41_violations(&shipped);
    for name in ["emit_tokio", "emit_blocking"] {
        let expected = format!("`{name}` (control) is promised and carries");
        assert!(
            violations.iter().any(|v| v.contains(&expected)),
            "a promised emitter hidden in `{name}`'s preamble passed: {violations:?}"
        );
    }
}

/// The part of CF-41 that is not about emitters applies to the whole crate: a
/// suite macro given a `__` name and left rendered fails it too.
#[test]
fn the_prefix_rule_reaches_macros_that_are_not_emitters() {
    let rendered = "\
#[macro_export]
macro_rules! __event_store_conformance {
";
    let violations = prefix_violations(&exported_in("control", rendered));
    assert!(
        violations
            .iter()
            .any(|v| v.contains("`__event_store_conformance` (control) breaks the rule")),
        "a rendered `__` suite macro passed: {violations:?}"
    );
}

/// `SOURCES` lists every file in `src/` that exports a macro, so the scan
/// sees everything the crate exports. Host only, because it reads the
/// directory.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn every_exporting_file_is_in_sources() -> std::io::Result<()> {
    let src = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut missing = Vec::new();
    for entry in std::fs::read_dir(&src)? {
        let path = entry?.path();
        if path.extension().is_none_or(|ext| ext != "rs") {
            continue;
        }
        let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };
        let exports = std::fs::read_to_string(&path)?
            .lines()
            .any(|l| l.trim_start().starts_with("#[macro_export"));
        let listed = SOURCES
            .iter()
            .any(|(file, _)| file.strip_prefix("src/") == Some(name));
        if exports && !listed {
            missing.push(name.to_owned());
        }
    }
    assert!(
        missing.is_empty(),
        "these files export macros and are not in `SOURCES`, so CF-41's scan \
         cannot see what they export: {missing:?}"
    );
    Ok(())
}

/// C2-03: the page an adapter author lands on names every emitter they may be
/// required to write.
///
/// **The wrong implementation this rejects shipped**: the front page's table
/// carried three rows, all of them the event-store family's, while twelve
/// emitters existed — so an author of a model, benchmark or concurrency harness
/// on a runtime other than tokio had no rendered name to write at all.
#[test]
fn the_front_page_names_every_emitter_this_crate_ships() {
    let named = named_on_the_front_page();
    let missing: Vec<String> = shipped()
        .into_iter()
        .filter(|m| !named.contains(&m.name))
        .map(|m| format!("{} ({})", m.name, m.file))
        .collect();
    assert!(
        missing.is_empty(),
        "these emitter macros ship and the crate's front page never names them. \
         CF-23 makes the wrapper an adapter-supplied parameter, and the page is \
         where an author chooses one: {missing:?}"
    );
}

/// The reverse direction: the page names nothing that does not exist.
#[test]
fn the_front_page_names_no_emitter_this_crate_does_not_ship() {
    let shipped: Vec<String> = shipped().into_iter().map(|m| m.name).collect();
    let absent: Vec<String> = named_on_the_front_page()
        .into_iter()
        .filter(|name| !shipped.contains(name))
        .collect();
    assert!(
        absent.is_empty(),
        "the front page tells an adapter author to write these, and this crate \
         defines none of them: {absent:?}"
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
