//! What an exported macro's expansion is allowed to name in somebody else's
//! crate.
//!
//! # The rule, and why it is a rule rather than a habit
//!
//! `crates/happenstance-testkit/src/lib.rs`'s onboarding page states it as an
//! absolute:
//!
//! > That line expands in *your* crate, which is why the expansion never assumes
//! > what you have in scope: it spells the fixture trait as
//! > `$crate::__private::ProjectionFixture`, through a hidden module this crate
//! > keeps for the purpose.
//!
//! `__private` exists for that sentence, and `CHANGELOG.md` cites the sentence
//! to classify a documentation change as *not* a MINOR event — *"No item became
//! `pub` and no rule changed"*. Both readings depend on the expansion naming
//! nothing else, and two of the five suite macros named something else.
//!
//! The consequence is not that a caller might write a wrong path. It is that a
//! caller who wrote **no** path now depends on one.
//! `crates/happenstance-sqlite/tests/concurrency.rs` is a single line —
//! `happenstance_testkit::event_store_concurrency_conformance!(SqliteFixture::new());`
//! — and it compiles only while `$crate::concurrency::ConcurrentFixture`
//! resolves. Moving `ConcurrentFixture` to the crate root, demoting
//! `pub mod concurrency` to private with selective re-exports, or relocating
//! `BenchmarkParams` is therefore a **major** break of this crate that breaks
//! one-line callers, and `cargo-semver-checks` cannot see it: it reads item
//! paths, not macro bodies.
//!
//! # The second direction
//!
//! The same rule broken the other way. `projection.rs`'s `require_read_through!`
//! names `ProjectionProbe` bare, resolving against that file's own `use` rather
//! than against the expansion site, while its two siblings thirty and eighty
//! lines above qualify everything. It is not `#[macro_export]`ed, so it is
//! latent — but that module's own header argues that these three helpers get
//! **copied per family**, and the copy is where a bare path becomes an
//! `error[E0405]` inside a macro the author did not write.
//!
//! # What this does not check, stated because a green here is read as coverage
//!
//! * **Macro paths.** `$crate::__emit_tokio`, `$crate::for_each_event_store_rule!`
//!   and `$crate::event_store_conformance!` are exempt, and the exemption is
//!   mechanical rather than a preference: `macro_rules!` lives in a flat
//!   crate-root textual namespace, so a macro is not reachable through a module
//!   at all. Whether those names are a *promise* is the open question C2-03
//!   routes to §6.6 and to `HS-S0091`, and nothing here answers it.
//! * **Whether a `__private` re-export is the right one.** The check knows that
//!   a path is routed, not that it resolves to the item the author meant. The
//!   compiler owns that half and owns it completely.
//! * **Anything outside a `macro_rules!` body.** A rule function naming a
//!   private path is ordinary; this file is about text that lands in a stranger's
//!   crate.
//! * **`as` in a cast.** [`trait_after_as`] treats a primitive type name after
//!   `as` as a cast and skips it, which is a lexer's approximation of a parser
//!   and is stated here rather than left to be discovered — the same standard
//!   `xtask/src/lints.rs` holds its own scanners to (RS-81-2).

use std::collections::BTreeSet;

/// The files carrying `macro_rules!` definitions in this crate.
///
/// `include_str!` rather than a directory walk, so the set is fixed at compile
/// time and a file that stops existing is a build error rather than a silently
/// shorter scan. [`every_macro_carrying_file_is_scanned`] is what stops the list
/// going stale in the other direction.
const SOURCES: [(&str, &str); 7] = [
    ("src/lib.rs", include_str!("../src/lib.rs")),
    ("src/registry.rs", include_str!("../src/registry.rs")),
    ("src/suite.rs", include_str!("../src/suite.rs")),
    ("src/projection.rs", include_str!("../src/projection.rs")),
    ("src/concurrency.rs", include_str!("../src/concurrency.rs")),
    ("src/bench.rs", include_str!("../src/bench.rs")),
    ("src/model.rs", include_str!("../src/model.rs")),
];

/// The module every item path in an exported expansion must be rooted at.
const PRIVATE: &str = "$crate::__private::";

/// The prefix of a `$crate::` path this check treats as a macro rather than an
/// item. See the module docs for why macros cannot be routed through a module.
const MACRO_PREFIXES: [&str; 2] = ["__emit", "for_each"];

/// Primitive type names, so a cast is not read as a qualified path.
const PRIMITIVES: [&str; 14] = [
    "u8", "u16", "u32", "u64", "u128", "usize", "i8", "i16", "i32", "i64", "i128", "isize", "f32",
    "f64",
];

/// The lines with every string literal's contents removed.
///
/// Necessary rather than tidy: `must!`'s body is one long `panic!` message that
/// contains the words *"can record as a skip"*, and a scanner that reads a
/// panic message as code reports `a` as an unqualified trait. The flag is
/// carried across lines because a `\`-continued literal stays open, which is
/// exactly the shape those messages use.
///
/// It is not a Rust lexer — raw strings, byte strings and character literals are
/// not modelled — and this crate's macros use none of them (RS-81-2).
fn without_strings(body: &[String]) -> Vec<String> {
    let mut out = Vec::new();
    let mut inside = false;
    for line in body {
        let mut kept = String::new();
        let mut chars = line.chars();
        while let Some(c) = chars.next() {
            match c {
                '\\' if inside => {
                    chars.next();
                }
                '"' => inside = !inside,
                _ if !inside => kept.push(c),
                _ => {}
            }
        }
        out.push(kept);
    }
    out
}

/// One `macro_rules!` definition: its name, whether it is exported, and the
/// lines of its body with comments and string literals removed.
struct Macro {
    file: &'static str,
    name: String,
    exported: bool,
    body: Vec<String>,
}

/// Every `macro_rules!` in `source`.
///
/// The body runs from the definition line to the first line that is exactly `}`
/// at column zero, which is what rustfmt produces for every macro in this crate
/// and is the same shape `declared_projection_rules` relies on one file over. A
/// macro formatted otherwise would end this scan early; the vacuity guards below
/// are what would notice.
fn macros_of(file: &'static str, source: &str) -> Vec<Macro> {
    let lines: Vec<&str> = source.lines().collect();
    let mut macros = Vec::new();
    for (n, line) in lines.iter().enumerate() {
        let Some(rest) = line.strip_prefix("macro_rules! ") else {
            continue;
        };
        let name = rest.trim_end_matches(" {").trim().to_owned();
        let exported = lines[..n]
            .iter()
            .rev()
            .take_while(|above| {
                let t = above.trim_start();
                t.starts_with('#') || t.starts_with("///") || t.starts_with("//")
            })
            .any(|above| above.trim() == "#[macro_export]");
        let body: Vec<String> = lines[n + 1..]
            .iter()
            .take_while(|body| *body != &"}")
            .filter(|body| !body.trim_start().starts_with("//"))
            .map(|body| (*body).to_owned())
            .collect();
        let body = without_strings(&body);
        macros.push(Macro {
            file,
            name,
            exported,
            body,
        });
    }
    macros
}

/// Every `$crate::`-rooted path in one line, as the text following `$crate::`
/// up to the next character that cannot be part of a path.
fn crate_paths(line: &str) -> Vec<String> {
    let mut paths = Vec::new();
    let mut rest = line;
    while let Some(at) = rest.find("$crate::") {
        rest = &rest[at + "$crate::".len()..];
        let end = rest
            .find(|c: char| !(c.is_alphanumeric() || c == '_' || c == ':' || c == '$'))
            .unwrap_or(rest.len());
        let tail = rest[end..].trim_start();
        let path = rest[..end].to_owned();
        // A macro invocation is `$crate::name!(…)`; a macro handed on as `$emit`
        // is a bare path and is caught by `MACRO_PREFIXES` instead.
        if !tail.starts_with('!') {
            paths.push(path);
        }
        rest = &rest[end..];
    }
    paths
}

/// The trait named after a qualified-path `as`, for each occurrence in a line.
///
/// Casts are skipped by looking the following token up in [`PRIMITIVES`]. That
/// is a lexer's approximation and the module docs say so.
fn trait_after_as(line: &str) -> Vec<String> {
    let mut traits = Vec::new();
    let mut rest = line;
    while let Some(at) = rest.find(" as ") {
        rest = &rest[at + " as ".len()..];
        let end = rest
            .find(|c: char| !(c.is_alphanumeric() || c == '_' || c == ':' || c == '$'))
            .unwrap_or(rest.len());
        let named = rest[..end].to_owned();
        if !named.is_empty() && !PRIMITIVES.contains(&named.as_str()) {
            traits.push(named);
        }
        rest = &rest[end..];
    }
    traits
}

fn all_macros() -> Vec<Macro> {
    SOURCES
        .iter()
        .flat_map(|(file, source)| macros_of(file, source))
        .collect()
}

/// The scan is reading a corpus rather than an empty list.
///
/// Without this, every assertion below passes over a parser that has stopped
/// finding macros — RS-81-4, and the reason `lint_pages`'s guards land with the
/// checker rather than beside it.
#[test]
fn the_scan_finds_the_macros_it_is_about() {
    let macros = all_macros();
    assert!(
        macros.len() >= 20,
        "this crate defines twenty-odd `macro_rules!` and the scan found {}: \
         either the definitions moved out of `SOURCES` or `macros_of` has \
         stopped lexing them",
        macros.len()
    );
    let exported: Vec<&Macro> = macros.iter().filter(|m| m.exported).collect();
    assert!(
        exported.len() >= 15,
        "the exported set is what lands in a stranger's crate and the scan found \
         {}: a `#[macro_export]` this lexer cannot see is a macro this file is \
         not checking",
        exported.len()
    );
    assert!(
        macros.iter().any(|m| !m.exported),
        "the unexported set is the second direction's subject — `must!`, \
         `require!`, `require_read_through!` — and finding none means the \
         `#[macro_export]` lookback is answering `true` for everything"
    );
}

/// `SOURCES` names every file in `src/` that defines a macro.
///
/// The failure this rejects is a new module with an exported macro in it and no
/// row here: the scan would be green and would not have read the file.
#[test]
fn every_macro_carrying_file_is_scanned() {
    let listed: BTreeSet<&str> = SOURCES.iter().map(|(file, _)| *file).collect();
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut missing = Vec::new();
    for entry in std::fs::read_dir(&dir).expect("this crate has a `src/`") {
        let path = entry.expect("a readable directory entry").path();
        if path.extension().is_none_or(|e| e != "rs") {
            continue;
        }
        let body = std::fs::read_to_string(&path).expect("a readable source file");
        if !body.contains("\nmacro_rules! ") {
            continue;
        }
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .expect("a UTF-8 file name")
            .to_owned();
        let rel = format!("src/{name}");
        if !listed.contains(rel.as_str()) {
            missing.push(rel);
        }
    }
    assert!(
        missing.is_empty(),
        "these files define `macro_rules!` and are not in `SOURCES`, so nothing \
         checks what their expansions name in a caller's crate: {missing:?}"
    );
}

/// M-3: every item path an exported expansion names is routed through
/// `__private`.
///
/// **The wrong implementations this rejects both shipped.**
/// `event_store_concurrency_conformance!` reached around the module with
/// `$crate::concurrency::ConcurrentFixture`, and `event_store_benchmarks!` did
/// both things eight lines apart — `$crate::__private::Fixture` on one line and
/// `$crate::bench::BenchmarkParams` on the next.
#[test]
fn an_exported_expansion_names_no_item_outside_private() {
    let mut problems = Vec::new();
    for item in all_macros().iter().filter(|m| m.exported) {
        for line in &item.body {
            for path in crate_paths(line) {
                if MACRO_PREFIXES.iter().any(|prefix| path.starts_with(prefix)) {
                    continue;
                }
                if format!("$crate::{path}").starts_with(PRIVATE) {
                    continue;
                }
                problems.push(format!(
                    "{}: `{}` expands `$crate::{path}`, which freezes that module path for \
                     every caller who wrote one line",
                    item.file, item.name
                ));
            }
        }
    }
    assert!(
        problems.is_empty(),
        "an exported macro's expansion lands in a stranger's crate, and \
         `__private` is the whole of what this crate promises it will name \
         there. Widen `__private` and route the path through it — that is \
         additive, and it is what keeps the module layout movable:\n  {}",
        problems.join("\n  ")
    );
}

/// M-5: inside any macro body, a trait named in a qualified path is
/// `$crate::`-rooted.
///
/// **The wrong implementation this rejects shipped**, and it is the same rule
/// broken in the other direction: `require_read_through!`'s
/// `<<$fixture as $crate::ProjectionFixture>::Store as ProjectionProbe>` names
/// its second trait bare, so it resolves against `projection.rs`'s own `use`
/// rather than against the expansion site. It is unexported today. The module
/// whose header argues these helpers get copied per family is where it stops
/// being latent.
#[test]
fn every_trait_a_macro_names_is_crate_rooted() {
    let mut problems = Vec::new();
    for item in all_macros() {
        for line in &item.body {
            for named in trait_after_as(line) {
                if named.starts_with("$crate::") {
                    continue;
                }
                problems.push(format!(
                    "{}: `{}` names `{named}` after `as` without a `$crate::` root, so it \
                     resolves against this file's imports rather than against the crate the \
                     expansion lands in",
                    item.file, item.name
                ));
            }
        }
    }
    assert!(
        problems.is_empty(),
        "a macro body is text substituted somewhere else; a bare trait name in \
         it is an `error[E0405]` waiting for the first caller who has not \
         imported it, inside an expansion they did not write:\n  {}",
        problems.join("\n  ")
    );
}
