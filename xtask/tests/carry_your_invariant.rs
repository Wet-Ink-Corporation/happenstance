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

/// The marker that closes it, which must come after [`WRONG`].
const END_WRONG: &str = "// END WRONG";

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
        .unwrap_or_else(|e| panic!("{rel}: {e}"))
        .replace("\r\n", "\n")
}

use xtask::fence::{self, Opener};

/// One fenced block: its info string, its 1-based opening and closing lines,
/// and its body.
struct Fence {
    info: String,
    line: usize,
    end: usize,
    body: Vec<String>,
}

/// Every fenced block on a page, in source order, read by `CommonMark`'s rule
/// in `xtask::fence`: backticks or tildes, indented up to three
/// spaces. A body line keeps its indentation, so an indented fence's lines
/// are measured a little wider than they render, never narrower.
fn fences(text: &str) -> Vec<Fence> {
    let mut out: Vec<Fence> = Vec::new();
    let mut open: Option<(Opener, Fence)> = None;
    for (index, line) in text.lines().enumerate() {
        open = match (open.take(), fence::delimiter(line)) {
            (Some((opener, mut fence)), Some(found)) if found.closes(opener) => {
                fence.end = index + 1;
                out.push(fence);
                None
            }
            (Some((opener, mut fence)), _) => {
                fence.body.push(line.to_owned());
                Some((opener, fence))
            }
            (None, Some(found)) => Some((
                found.opener(),
                Fence {
                    info: found.info.to_owned(),
                    line: index + 1,
                    end: index + 1,
                    body: Vec::new(),
                },
            )),
            (None, None) => None,
        };
    }
    assert!(open.is_none(), "a fence is opened and never closed");
    out
}

/// The part of a fence line a reader sees as code: the line up to any `//`
/// comment. A `//` inside a string literal cuts the line short too, which can
/// only make a match below fail, never pass.
fn code_of(line: &str) -> &str {
    line.split_once("//").map_or(line, |(code, _)| code)
}

/// Whether any line of `fence` carries `needle` outside a comment.
fn carries_in_code(fence: &Fence, needle: &str) -> bool {
    fence.body.iter().any(|line| code_of(line).contains(needle))
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
/// A fence is whatever [`fences`] reads as one, delimiters included. A list
/// item starts a new paragraph, because it renders as its own block.
fn paragraphs(text: &str) -> Vec<String> {
    let spans: Vec<(usize, usize)> = fences(text)
        .iter()
        .map(|fence| (fence.line, fence.end))
        .collect();
    let mut out = Vec::new();
    let mut current = String::new();
    for (index, line) in text.lines().enumerate() {
        let fenced = spans
            .iter()
            .any(|(open, end)| (*open..=*end).contains(&(index + 1)));
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

/// The constructor only the guard calls: a fence without it is not the guard,
/// whatever section it sits in.
const GUARD_CALL: &str = "AppendCondition::new";

/// AC-003's check: the first fence on `page` sits under [`GUARD_HEADING`] and
/// builds the guard, [`GUARD_CALL`], outside a comment.
fn first_fence_is_the_guard(page: &str) -> Result<(), String> {
    let first = fences(page)
        .into_iter()
        .next()
        .ok_or("the page carries no fence")?;
    if section_of(page, first.line) != Some(GUARD_HEADING) {
        return Err(format!(
            "line {}: the first code on the page is not under `{GUARD_HEADING}`",
            first.line
        ));
    }
    if !carries_in_code(&first, GUARD_CALL) {
        return Err(format!(
            "line {}: the first fence does not build the guard, `{GUARD_CALL}`",
            first.line
        ));
    }
    Ok(())
}

/// AC-003: the guard is the first code on the page. The identifiers section
/// once put its newtype fence ahead of it (#232).
#[test]
fn the_guard_is_the_first_code_on_the_page() -> Result<(), String> {
    first_fence_is_the_guard(&read(PAGE)).map_err(|e| format!("{PAGE}: {e}"))
}

/// A fence that is in the right section but is not the guard is refused, and
/// so is a guard whose `AppendCondition::new` is only a comment (#279).
#[test]
fn a_first_fence_without_the_guard_is_refused() {
    let guard = format!("{GUARD_HEADING}\n\n```rust\n");
    for body in ["fn main() {}\n", "// AppendCondition::new(rule)\n"] {
        let page = format!("{guard}{body}```\n");
        assert!(
            first_fence_is_the_guard(&page).is_err(),
            "a first fence carrying no guard passed: {page:?}"
        );
    }
    let page = format!("{guard}let c = AppendCondition::new(rule);\n```\n");
    assert_eq!(first_fence_is_the_guard(&page), Ok(()));
}

/// Every fence line on `page` over [`FENCE_COLUMNS`], as `(line, columns)`,
/// 1-based.
fn fence_lines_over_width(page: &str) -> Vec<(usize, usize)> {
    fences(page)
        .iter()
        .flat_map(|fence| {
            fence
                .body
                .iter()
                .enumerate()
                .map(|(offset, line)| (fence.line + offset + 1, line.chars().count()))
        })
        .filter(|(_, columns)| *columns > FENCE_COLUMNS)
        .collect()
}

/// AC-008: no fence scrolls horizontally at 1024x768.
#[test]
fn every_fence_line_holds_the_width_budget() {
    let over = fence_lines_over_width(&read(PAGE));
    assert!(
        over.is_empty(),
        "{PAGE}: fence lines over {FENCE_COLUMNS} columns, as (line, columns), \
         which scroll at 1024x768: {over:?}"
    );
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

/// Anti-pattern 7's check: the last fence on `page` is Rust, is not the wrong
/// guard, and carries [`CORRECT_SEATS`] outside a comment.
fn last_fence_is_the_correct_guard(page: &str) -> Result<(), String> {
    let all = fences(page);
    let last = all.last().ok_or("the page carries no fence")?;
    if last.info != "rust" {
        return Err(format!(
            "line {}: the last fence is `{}`",
            last.line, last.info
        ));
    }
    if is_wrong(last) {
        return Err(format!(
            "line {}: the last code on the page is the wrong guard",
            last.line
        ));
    }
    if !carries_in_code(last, CORRECT_SEATS) {
        return Err(format!(
            "line {}: the last fence does not carry the correct guard's \
             expression `{CORRECT_SEATS}`",
            last.line
        ));
    }
    Ok(())
}

/// Anti-pattern 7: scroll to the bottom of the bridge and the last code you
/// see must be the correct guard. A link back to it is not code (#233).
#[test]
fn the_last_code_on_the_page_is_the_correct_guard() -> Result<(), String> {
    last_fence_is_the_correct_guard(&read(PAGE)).map_err(|e| format!("{PAGE}: {e}"))
}

/// The correct expression in a comment, whole-line or trailing, is not the
/// correct guard (#279).
#[test]
fn a_last_fence_naming_the_guard_only_in_a_comment_is_refused() {
    for body in [
        "// let seats = QueryItem::new([\"SeatHeld\"], held.clone())?;\n",
        "fn main() {} // QueryItem::new([\"SeatHeld\"], held\n",
    ] {
        let page = format!("```rust\n{body}```\n");
        assert!(
            last_fence_is_the_correct_guard(&page).is_err(),
            "a commented-out guard passed as the last code: {page:?}"
        );
    }
    let page = "```rust\nlet s = QueryItem::new([\"SeatHeld\"], held)?;\n```\n";
    assert_eq!(last_fence_is_the_correct_guard(page), Ok(()));
}

/// The check behind "marked at both ends, and not last": exactly one fence
/// carries [`WRONG`], it carries `// END WRONG` after it, and code follows.
fn wrong_fence_is_marked_and_not_last(page: &str) -> Result<(), String> {
    let all = fences(page);
    let wrong: Vec<usize> = all
        .iter()
        .enumerate()
        .filter(|(_, fence)| is_wrong(fence))
        .map(|(index, _)| index)
        .collect();

    let [at] = wrong.as_slice() else {
        return Err(format!("{} wrong-guard fences, not one", wrong.len()));
    };
    let fence = all.get(*at).ok_or("the wrong fence's index is a fence")?;
    let opens = fence.body.iter().position(|line| line.contains(WRONG));
    let ends = fence.body.iter().rposition(|line| line.contains(END_WRONG));
    if !matches!((opens, ends), (Some(open), Some(end)) if open < end) {
        return Err(format!(
            "line {}: the wrong fence is not marked `{END_WRONG}` after its \
             `{WRONG}`",
            fence.line
        ));
    }
    if *at + 1 >= all.len() {
        return Err(format!(
            "line {}: the wrong fence is the last code on the page",
            fence.line
        ));
    }
    Ok(())
}

/// The wrong fence exists, is marked at both ends, and is followed by code.
#[test]
fn the_wrong_fence_is_marked_at_both_ends_and_is_not_last() -> Result<(), String> {
    wrong_fence_is_marked_and_not_last(&read(PAGE)).map_err(|e| format!("{PAGE}: {e}"))
}

/// An end marker above the opening marker marks nothing (#279).
#[test]
fn an_end_marker_before_the_wrong_marker_is_refused() {
    let next = "```rust\nfn main() {}\n```\n";
    let reversed = format!("```rust\n// END WRONG.\n// WRONG - why\n```\n{next}");
    assert!(
        wrong_fence_is_marked_and_not_last(&reversed).is_err(),
        "a wrong fence closed before it opened passed: {reversed:?}"
    );
    let ordered = format!("```rust\n// WRONG - why\n// END WRONG.\n```\n{next}");
    assert_eq!(wrong_fence_is_marked_and_not_last(&ordered), Ok(()));
}

// ---------------------------------------------------------------------------
// #279 — every fence `CommonMark` opens is a fence here
// ---------------------------------------------------------------------------

/// A tilde fence and a backtick fence indented by up to three spaces are both
/// fences to rustdoc and to the site's renderer, so each is seen, with its
/// info string and its body, and neither is counted as prose.
#[test]
fn a_tilde_fence_and_an_indented_backtick_fence_are_seen() {
    for page in [
        "~~~rust\n// WRONG\n~~~\n",
        "  ```rust\n// WRONG\n  ```\n",
        "~~~~rust\n// WRONG\n~~~~\n",
    ] {
        let found = fences(page);
        let [fence] = found.as_slice() else {
            panic!("{} fences seen in {page:?}, not one", found.len());
        };
        assert_eq!(
            (fence.info.as_str(), fence.line, fence.body.as_slice()),
            ("rust", 1, ["// WRONG".to_owned()].as_slice()),
            "{page:?}"
        );
        assert!(
            paragraphs(page).is_empty(),
            "a fence read as prose: {page:?}"
        );
    }
}

/// A delimiter of the other character, or a shorter run, does not close a
/// fence; four spaces of indentation is not a fence at all.
#[test]
fn only_a_matching_delimiter_closes_a_fence() {
    let found = fences("````rust\n```\n~~~\n````\n");
    let [fence] = found.as_slice() else {
        panic!("{} fences seen, not one", found.len());
    };
    assert_eq!(fence.body, ["```", "~~~"]);
    assert!(fences("    ```rust\n").is_empty());
}

/// The defect #279 names, whole: a correct page with a trailing tilde fence
/// carrying the wrong marker and a 100-column line.
#[test]
fn a_trailing_tilde_wrong_fence_is_the_last_code() {
    let page = format!(
        "```rust\nlet s = QueryItem::new([\"SeatHeld\"], held)?;\n```\n\
         ~~~rust\n// WRONG\n{}\n~~~\n",
        "x".repeat(100)
    );
    assert!(
        last_fence_is_the_correct_guard(&page).is_err(),
        "a trailing tilde wrong fence was not the last code"
    );
    assert_eq!(fence_lines_over_width(&page), [(6, 100)]);
}
