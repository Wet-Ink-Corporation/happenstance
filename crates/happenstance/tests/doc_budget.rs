//! Source-reading assertions over the page this crate renders.
//!
//! These cost nothing — no compiler runs — and they cannot be satisfied by an
//! unstyled render, which is the whole reason they exist. The numbers are the
//! signed-off design's (`_design.md`, `## Density budget`).

use std::path::{Path, PathBuf};

/// Doc-comment prose, in columns. A line carrying a URL is exempt: a link
/// target cannot be wrapped, and the crate root already carries three.
const PROSE_COLUMNS: usize = 80;
/// Code inside a doc fence, in columns. This is the budget that keeps the
/// fence from acquiring a horizontal scrollbar at 1024px.
const FENCE_COLUMNS: usize = 72;
/// The crate-root module doc, in lines.
const MODULE_DOC_LINES: usize = 130;
/// Prose lines above the crate root's first fence.
const LINES_TO_FIRST_FENCE: usize = 12;
/// The first sentence of any doc comment, in characters.
const FIRST_SENTENCE: usize = 80;

fn src() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("src")
}

/// Read a source file with its line endings normalised to `\n`.
///
/// Every assertion below matches against literal `\n`, and this repository is
/// developed on Windows with `core.autocrlf = true` and no `.gitattributes` —
/// so a *freshly checked out* file is CRLF and a pattern like `"\n}\n"` never
/// matches, while the same file written by an editor that emits LF passes. The
/// normalisation is here rather than at each call site because the failure is
/// silent in one direction: `find` returning `None` reads as "the item is not
/// in this file", which is indistinguishable from the defect these tests exist
/// to catch.
fn read(name: &str) -> String {
    std::fs::read_to_string(src().join(name))
        .expect("the crate's own source is readable")
        .replace("\r\n", "\n")
}

/// Every rendering source file. `tests.rs` is `#[cfg(test)]`, so nothing in it
/// reaches a page and the budget is not about it.
fn rendering_sources() -> Vec<(String, String)> {
    let mut out = Vec::new();
    for entry in std::fs::read_dir(src()).expect("the crate's own src/ is readable") {
        let path = entry.expect("a readable directory entry").path();
        let name = path
            .file_name()
            .expect("a named file")
            .to_string_lossy()
            .into_owned();
        if name == "tests.rs" || path.extension().is_none_or(|e| e != "rs") {
            continue;
        }
        out.push((name, std::fs::read_to_string(&path).expect("valid utf-8")));
    }
    assert!(out.len() >= 5, "the crate's modules were not found");
    out
}

/// The doc-comment body of a line, if it is one.
fn doc_body(line: &str) -> Option<&str> {
    let trimmed = line.trim_start();
    for marker in ["//!", "///"] {
        if let Some(rest) = trimmed.strip_prefix(marker) {
            return Some(rest.strip_prefix(' ').unwrap_or(rest));
        }
    }
    None
}

/// `(name, line number, body, inside a fence)` for every doc line in the crate.
fn doc_lines() -> Vec<(String, usize, String, bool)> {
    let mut out = Vec::new();
    for (name, source) in rendering_sources() {
        let mut in_fence = false;
        for (index, line) in source.lines().enumerate() {
            let Some(body) = doc_body(line) else { continue };
            if body.trim_start().starts_with("```") {
                in_fence = !in_fence;
                continue;
            }
            out.push((name.clone(), index + 1, body.to_owned(), in_fence));
        }
        assert!(!in_fence, "{name}: an unterminated doc fence");
    }
    out
}

#[test]
fn doc_prose_stays_within_eighty_columns() {
    for (name, line_number, body, in_fence) in doc_lines() {
        if in_fence || body.contains("http") {
            continue;
        }
        // The rendered column is the whole authored line, marker included.
        let columns = body.chars().count() + 4;
        assert!(
            columns <= PROSE_COLUMNS,
            "{name}:{line_number}: doc prose is {columns} columns, over {PROSE_COLUMNS}"
        );
    }
}

#[test]
fn doc_fences_stay_within_seventy_two_columns() {
    for (name, line_number, body, in_fence) in doc_lines() {
        if !in_fence {
            continue;
        }
        assert!(
            body.chars().count() <= FENCE_COLUMNS,
            "{name}:{line_number}: a fence line is {} columns, over {FENCE_COLUMNS}, \
             so it scrolls horizontally at 1024px",
            body.chars().count()
        );
    }
}

#[test]
fn the_module_doc_stays_under_its_line_budget() {
    let root = read("lib.rs");
    let lines = root
        .lines()
        .filter(|line| line.trim_start().starts_with("//!"))
        .count();
    assert!(
        lines <= MODULE_DOC_LINES,
        "the crate-root module doc is {lines} lines, over {MODULE_DOC_LINES}"
    );
}

#[test]
fn the_first_fence_is_within_twelve_prose_lines() {
    let root = read("lib.rs");
    let doc: Vec<&str> = root
        .lines()
        .filter(|line| line.trim_start().starts_with("//!"))
        .collect();

    let opens = doc
        .iter()
        .position(|line| doc_body(line).is_some_and(|b| b.starts_with("```")))
        .expect("the crate root carries a fence");
    assert!(
        opens <= LINES_TO_FIRST_FENCE,
        "the first fence starts {opens} prose lines down, over {LINES_TO_FIRST_FENCE}"
    );

    // And it is the only fence on that page: a second one would demote the
    // first, which is the page's primary hierarchy signal.
    let fences = doc
        .iter()
        .filter(|line| doc_body(line).is_some_and(|b| b.starts_with("```")))
        .count();
    assert_eq!(fences, 2, "the crate root has exactly one fence");
}

#[test]
fn first_sentences_fit_the_item_table() {
    for (name, source) in rendering_sources() {
        let mut awaiting = true;
        let mut sentence = String::new();
        for (index, line) in source.lines().enumerate() {
            let Some(body) = doc_body(line) else {
                awaiting = true;
                sentence.clear();
                continue;
            };
            if body.trim_start().starts_with("```") || !awaiting || body.trim().is_empty() {
                continue;
            }
            if !sentence.is_empty() {
                sentence.push(' ');
            }
            sentence.push_str(body.trim());
            let end = sentence.find(". ").or_else(|| {
                sentence
                    .ends_with('.')
                    .then(|| sentence.len().saturating_sub(1))
            });
            if let Some(end) = end {
                assert!(
                    end < FIRST_SENTENCE,
                    "{name}:{}: a first sentence is {} characters, over {FIRST_SENTENCE} — \
                     rustdoc's item table truncates it with an ellipsis",
                    index + 1,
                    end + 1
                );
                awaiting = false;
                sentence.clear();
            }
        }
    }
}

#[test]
fn every_emitted_path_is_crate_qualified() {
    let source = read("composition.rs");
    let start = source
        .find("macro_rules! impl_boundary_for_tuple")
        .expect("the composition macro is defined here");
    let end = source[start..]
        .find("\n}\n")
        .expect("the macro definition is terminated")
        + start;
    let body = &source[start..end];

    // Hygiene covers locals and labels, not item paths: those resolve where
    // the expansion lands. Every item the expansion names must therefore be
    // rooted — `$crate::` for ours, `::` for the standard library.
    for name in [
        "Boundary",
        "Sealed",
        "Query",
        "InvalidQuery",
        "Codec",
        "CodecError",
        "SequencedEvent",
        "Vec",
        "Result",
        "Option",
    ] {
        let mut at = 0;
        let mut seen = 0;
        while let Some(found) = body[at..].find(name) {
            let index = at + found;
            at = index + name.len();

            let before = body[..index].chars().next_back();
            let after = body[at..].chars().next();
            let part_of_a_longer_word = before.is_some_and(|c| c.is_alphanumeric() || c == '_')
                || after.is_some_and(|c| c.is_alphanumeric() || c == '_');
            if part_of_a_longer_word {
                continue;
            }
            // A doc comment inside the matcher is prose, not an emitted path.
            let line_start = body[..index].rfind('\n').map_or(0, |n| n + 1);
            if body[line_start..index].trim_start().starts_with("///") {
                continue;
            }

            seen += 1;
            assert!(
                body[..index].ends_with("::"),
                "the expansion names `{name}` unrooted at byte {index}; hygiene does not \
                 cover item paths, so it would resolve in the caller's crate"
            );
        }
        assert!(seen > 0, "`{name}` was not found in the macro body at all");
    }
}

#[test]
fn the_glob_reexport_survives_and_nothing_shadows_it() {
    let root = read("lib.rs");

    assert!(
        root.contains("pub use happenstance_core::*;"),
        "the contract crate's glob re-export must survive"
    );

    // No item of ours may take a contract name. A second `Query` on the page
    // is two entries with one name, and a silent breaking change to a facade
    // the crate's own docs promise is path-compatible.
    for shadowed in ["Query", "EventStore", "Tags", "Event"] {
        for declaration in [
            "pub trait ",
            "pub enum ",
            "pub struct ",
            "pub type ",
            "pub fn ",
        ] {
            let banned = format!("{declaration}{shadowed}");
            for (name, source) in rendering_sources() {
                assert!(
                    !source.contains(&banned),
                    "{name} declares `{banned}`, which shadows a contract name"
                );
            }
        }
        assert!(
            !root.contains(&format!("as {shadowed};")),
            "the crate root re-exports something as `{shadowed}`"
        );
    }
}
