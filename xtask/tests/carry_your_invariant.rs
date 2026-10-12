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
//! AC-004 and IQ-2.3).

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
