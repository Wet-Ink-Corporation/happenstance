//! Source-reading assertions over the bridge, `docs/carry-your-invariant.md`.
//!
//! The doctests under `cargo test -p xtask --doc` prove the page's programs
//! work. Nothing in that run can see how the page is *composed*: which fence a
//! reader meets first and which one they leave on. These assertions are the
//! composition the bridge's design fixed, made mechanical — the shape
//! `first_encounter.rs` already has for the opening encounter.
//!
//! The numbers and the ordering are the bridge story's
//! (`.bklg/docs-that-teach/application-author-path/invariant-to-appendcondition-bridge/spec.md`,
//! AC-003, AC-004, AC-008 and IQ-2.3). The 90-column prose budget is not
//! asserted: the page's citation links and its table rows cannot wrap.

use std::path::{Path, PathBuf};

/// The bridge, repo-relative.
const PAGE: &str = "docs/carry-your-invariant.md";

/// The marker that opens the wrong-guard fence's mistake.
const WRONG: &str = "// WRONG";

/// The one expression that makes the guard correct: the seats item tagged to
/// the course whose seats the rule is about.
const CORRECT_SEATS: &str = "QueryItem::new([\"SeatHeld\"], held";

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask sits one level below the repository root")
        .to_path_buf()
}

/// Read a repo-relative file with its line endings normalised to `\n`.
fn read(rel: &str) -> String {
    std::fs::read_to_string(root().join(rel))
        .unwrap_or_else(|_| panic!("{rel} is readable"))
        .replace("\r\n", "\n")
}

/// One fenced block: its info string, its 1-based opening line, and its body.
struct Fence {
    info: String,
    line: usize,
    body: Vec<String>,
}

/// Every fenced block on a page, in source order.
fn fences(text: &str) -> Vec<Fence> {
    let mut out: Vec<Fence> = Vec::new();
    let mut open: Option<Fence> = None;
    for (index, line) in text.lines().enumerate() {
        let Some(rest) = line.strip_prefix("```") else {
            if let Some(fence) = open.as_mut() {
                fence.body.push(line.to_owned());
            }
            continue;
        };
        match open.take() {
            Some(fence) => out.push(fence),
            None => {
                open = Some(Fence {
                    info: rest.trim().to_owned(),
                    line: index + 1,
                    body: Vec::new(),
                });
            }
        }
    }
    assert!(open.is_none(), "a fence is opened and never closed");
    out
}

/// The section whose fence must be the first code a reader meets.
const GUARD_HEADING: &str = "## The guard you would write";

/// A fence line, in columns: the design's hard width budget, which leaves
/// four columns before `overflow-x` engages at 1024x768.
const FENCE_COLUMNS: usize = 68;

/// A paragraph, in rendered characters — five lines at 764px.
const PARAGRAPH: usize = 435;

/// The last `## ` heading at or above a 1-based line.
fn section_of(text: &str, line: usize) -> Option<&str> {
    text.lines()
        .take(line)
        .filter(|candidate| candidate.starts_with("## "))
        .last()
}

/// A prose line as it renders: each inline link `[text](target)` keeps only
/// its text, because the target is never shown.
fn rendered(line: &str) -> String {
    let mut out = String::new();
    let mut rest = line;
    while let Some(open) = rest.find("](") {
        let Some(close) = rest[open..].find(')') else {
            break;
        };
        out.push_str(&rest[..open]);
        rest = &rest[open + close + 1..];
    }
    out.push_str(rest);
    out.replace('[', "")
}

/// Every prose paragraph on a page, rendered, outside fences and tables.
///
/// A list item starts a new paragraph, because it renders as its own block.
fn paragraphs(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut current = String::new();
    let mut fenced = false;
    for line in text.lines() {
        if line.starts_with("```") {
            fenced = !fenced;
            continue;
        }
        let item = line
            .split_once(". ")
            .is_some_and(|(n, _)| !n.is_empty() && n.chars().all(|c| c.is_ascii_digit()));
        let ends = line.trim().is_empty() || line.starts_with('|') || item;
        if fenced || ends {
            if !current.is_empty() {
                out.push(std::mem::take(&mut current));
            }
            if fenced || !item {
                continue;
            }
        }
        current.push_str(&rendered(line.trim()));
        current.push(' ');
    }
    if !current.is_empty() {
        out.push(current);
    }
    out
}

// ---------------------------------------------------------------------------
// AC-003 / AC-008 — the guard comes first, and the page fits its medium
// ---------------------------------------------------------------------------

/// AC-003: the guard is the first code on the page. The identifiers section
/// once put its newtype fence ahead of it (#232).
#[test]
fn the_guard_is_the_first_code_on_the_page() {
    let page = read(PAGE);
    let first = fences(&page)
        .into_iter()
        .next()
        .expect("the page carries a fence");
    assert_eq!(
        section_of(&page, first.line),
        Some(GUARD_HEADING),
        "{PAGE}:{}: the first code on the page is not under `{GUARD_HEADING}`",
        first.line
    );
}

/// AC-008: no fence scrolls horizontally at 1024x768.
#[test]
fn every_fence_line_holds_the_width_budget() {
    let page = read(PAGE);
    for fence in fences(&page) {
        for (offset, line) in fence.body.iter().enumerate() {
            let columns = line.chars().count();
            assert!(
                columns <= FENCE_COLUMNS,
                "{PAGE}:{}: a fence line is {columns} columns, over \
                 {FENCE_COLUMNS}, so it scrolls at 1024x768",
                fence.line + offset + 1
            );
        }
    }
}

/// AC-008: no paragraph runs past five rendered lines.
#[test]
fn every_paragraph_holds_its_budget() {
    let page = read(PAGE);
    for paragraph in paragraphs(&page) {
        let length = paragraph.trim_end().chars().count();
        assert!(
            length <= PARAGRAPH,
            "a paragraph renders to {length} characters, over {PARAGRAPH}: \
             {paragraph}"
        );
    }
}

/// The paragraph measure is not vacuous: a link keeps its text and loses its
/// target, a list item is its own paragraph, and a long one is caught.
#[test]
fn the_paragraph_measure_splits_items_and_drops_link_targets() {
    assert_eq!(
        rendered("see [the guide](a/b.md#c) now"),
        "see the guide now"
    );
    let found = paragraphs("one\ntwo\n\n1. three\n2. four\n");
    assert_eq!(found, ["one two ", "1. three ", "2. four "]);
    assert!(
        paragraphs(&"word ".repeat(100))
            .iter()
            .any(|p| p.chars().count() > PARAGRAPH)
    );
}

/// Whether a fence is the wrong-guard contrast.
fn is_wrong(fence: &Fence) -> bool {
    fence.body.iter().any(|line| line.contains(WRONG))
}

// ---------------------------------------------------------------------------
// AC-004 / IQ-2.3 — the wrong fence is never the last code on the page
// ---------------------------------------------------------------------------

/// Anti-pattern 7: scroll to the bottom of the bridge and the last code you
/// see must be the correct guard. A link back to it is not code (#233).
#[test]
fn the_last_code_on_the_page_is_the_correct_guard() {
    let page = read(PAGE);
    let all = fences(&page);
    let last = all.last().expect("the page carries a fence");

    assert_eq!(last.info, "rust", "{PAGE}:{}: the last fence", last.line);
    assert!(
        !is_wrong(last),
        "{PAGE}:{}: the last code on the page is the wrong guard",
        last.line
    );
    assert!(
        last.body.iter().any(|line| line.contains(CORRECT_SEATS)),
        "{PAGE}:{}: the last fence does not carry the correct guard's \
         expression `{CORRECT_SEATS}`",
        last.line
    );
}

/// The wrong fence exists, is marked at both ends, and is followed by code.
#[test]
fn the_wrong_fence_is_marked_at_both_ends_and_is_not_last() {
    let page = read(PAGE);
    let all = fences(&page);
    let wrong: Vec<usize> = all
        .iter()
        .enumerate()
        .filter(|(_, fence)| is_wrong(fence))
        .map(|(index, _)| index)
        .collect();

    let [at] = wrong.as_slice() else {
        panic!("{PAGE} carries {} wrong-guard fences, not one", wrong.len());
    };
    let fence = &all[*at];
    assert!(
        fence.body.iter().any(|line| line.contains("// END WRONG")),
        "{PAGE}:{}: the wrong fence is not marked at its end",
        fence.line
    );
    assert!(
        *at + 1 < all.len(),
        "{PAGE}:{}: the wrong fence is the last code on the page",
        fence.line
    );
}
