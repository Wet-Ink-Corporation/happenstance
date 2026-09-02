//! Reach assertions over the worked-example handoff.
//!
//! No compiler runs here, and that is deliberate. The extraction that makes
//! this example's explanation *surfaced* rather than paraphrased is an
//! `include_str!` line and a file; the page that makes it *reachable* is prose
//! and two links. Every one of those can be deleted, reworded or quietly
//! re-pointed while the workspace still builds, every doctest still passes and
//! `cargo doc` still exits 0. These assertions are the ones that go red.
//!
//! They live in the example's own crate rather than in a sixth `xtask` lint
//! because `cargo test --locked --workspace --all-features` already sweeps a
//! `publish = false` crate, so the check costs no new gate step and no new
//! target — and because the artefact most of it guards is this crate's own
//! module documentation.
//!
//! Reading a sibling artefact as text is this repository's own precedent for
//! the class: `cargo xtask lints` is five such checks, and
//! `examples/course-subscriptions/tests/runs.rs` already reads `main.rs` for
//! the claims a run cannot observe.

use std::path::{Path, PathBuf};

/// The single copy of the example's own explanation.
const OVERVIEW: &str = "examples/course-subscriptions/src/overview.md";
/// The example's source, and the second render path for `OVERVIEW`'s bytes.
const MAIN: &str = "examples/course-subscriptions/src/main.rs";
/// The handoff page.
const PAGE: &str = "docs/read-the-worked-example.md";
/// The slice-mate's bridge page, which carries the one link into `PAGE`.
const BRIDGE: &str = "docs/carry-your-invariant.md";
/// HS-P0020's harness — the `include_str!` line that makes `PAGE` a page.
const HARNESS: &str = "xtask/src/narrative.rs";

/// The doctest module `PAGE` must be registered under.
const MODULE: &str = "mod read_the_worked_example {";
/// The include the harness must carry, spelled as the checker spells it.
const INCLUDE: &str = "include_str!(\"../../docs/read-the-worked-example.md\")";

/// The DT-1 anchor's heading text, on the page that owns it.
const ANCHOR_HEADING: &str = "## Where your streams went";
/// The fragment `PAGE` cites it by.
const ANCHOR_LINK: &str = "carry-your-invariant.md#where-your-streams-went";

/// The one type name the seam sentence is allowed to carry.
const SEAM: &str = "happenstance::commit";
/// The crate name the merged example does not import, and the page may not
/// name: post-merge the design's permission for it is declined.
const WRONG_CRATE: &str = "happenstance_core";

/// The prior model is named on exactly one page in the whole set — the
/// bridge's `## Where your streams went` — and this page is not it.
const PRIOR_MODEL: [&str; 4] = [
    "aggregate",
    "your aggregates",
    "one stream per entity",
    "which stream",
];

/// Link text that tells a screen-reader user nothing out of a link list.
const EMPTY_LINK_TEXT: [&str; 4] = ["here", "this", "link", "click here"];

/// Repo-relative path budget, HS-P0020's, enforced by its own checker too.
const PATH_CHARS: usize = 32;
/// The H1 is the narrative index's left column at 70 columns.
const H1_CHARS: usize = 40;
/// A narrative page, in source lines.
const PAGE_LINES: usize = 250;
/// Prose source wrap. Tables and single-token URLs are exempt.
const WRAP: usize = 90;
/// Sidebar-TOC clipping budget for a `##`. This page has none, and the
/// assertion is kept so that acquiring one is a decision rather than a drift.
const HEADING_CHARS: usize = 22;

/// The repository root, from this crate's manifest directory.
fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("the example sits two levels below the repository root")
        .to_path_buf()
}

/// Read a repo-relative file with its line endings normalised to `\n`.
///
/// This repository is developed on Windows with `core.autocrlf = true`, so a
/// freshly checked-out file is CRLF and every `find` below would answer a
/// different question on a clean clone than in an editor's buffer.
fn read(rel: &str) -> String {
    std::fs::read_to_string(root().join(rel))
        .unwrap_or_else(|_| panic!("{rel} is readable"))
        .replace("\r\n", "\n")
}

/// The page's blocks: everything after the answered-need line, blank-separated.
///
/// The composition is five slots in a fixed order, so a block index is the
/// cheapest way to assert placement without parsing markdown.
fn blocks(text: &str) -> Vec<String> {
    text.split("\n\n")
        .map(str::trim)
        .filter(|block| !block.is_empty())
        .filter(|block| !block.starts_with("# ") && !block.starts_with("> **Answers:**"))
        .map(str::to_owned)
        .collect()
}

/// Sentence count, counted the way a reader counts: terminators followed by a
/// space or the end of the block.
fn sentences(block: &str) -> usize {
    let flat = block.replace('\n', " ");
    let bytes = flat.as_bytes();
    let mut count = 0;
    for (index, byte) in bytes.iter().enumerate() {
        if matches!(byte, b'.' | b'?' | b'!')
            && bytes.get(index + 1).is_none_or(|next| *next == b' ')
            && !flat[..index].ends_with("etc")
        {
            count += 1;
        }
    }
    count
}

/// Every `](target)` on a page, in source order, with the link's text.
fn links(text: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    let flat = text.replace('\n', " ");
    let mut rest = flat.as_str();
    while let Some(open) = rest.find('[') {
        let after = &rest[open + 1..];
        let Some(close) = after.find("](") else {
            break;
        };
        let target = &after[close + 2..];
        let Some(end) = target.find(')') else {
            break;
        };
        out.push((
            after[..close].trim().to_owned(),
            target[..end].trim().to_owned(),
        ));
        rest = &target[end + 1..];
    }
    out
}

/// AC-001 — the explanation moved, and there is exactly one copy of it.
#[test]
fn overview_is_the_only_copy() {
    let overview = read(OVERVIEW);
    assert!(
        overview.trim().len() > 400,
        "{OVERVIEW} is empty or a stub; the extraction moves the whole module doc"
    );

    let main = read(MAIN);
    assert!(
        main.contains("#![doc = include_str!(\"overview.md\")]"),
        "{MAIN} does not include {OVERVIEW}; verbatim is a property of the build, \
         and without this line it is a property of nobody"
    );
    assert!(
        !main
            .lines()
            .any(|line| line.trim_start().starts_with("//!")),
        "{MAIN} still carries a `//!` module-doc line; the move left a second copy behind"
    );

    let opening = overview
        .lines()
        .find(|line| !line.trim().is_empty())
        .expect("the explanation has an opening line");
    assert!(
        !read(PAGE).contains(opening),
        "{PAGE} pastes the explanation's opening line; two copies is the defect, \
         not a tolerance"
    );
}

/// AC-002 — the page is registered, one module for one included file.
#[test]
fn page_is_registered() {
    let harness = read(HARNESS);
    assert!(
        harness.contains(MODULE),
        "{HARNESS} declares no `{MODULE}`; a page the harness does not name is \
         the checker's `unregistered` state, not a page"
    );
    assert!(
        harness.contains(INCLUDE),
        "{HARNESS} does not include {PAGE}; its content is never compiled"
    );

    let start = harness.find(MODULE).expect("the module was found above");
    let block = &harness[start..];
    let end = block.find("\n}").expect("the module block is closed");
    assert_eq!(
        block[..end].matches("include_str!").count(),
        1,
        "the `read_the_worked_example` module carries more than one `include_str!`; \
         concatenated includes report a failure at a line counted from the first file"
    );
}

/// AC-003 — one answered-need, in HS-P0021's notation, above everything.
#[test]
fn exactly_one_answered_need() {
    let page = read(PAGE);
    let declared: Vec<(usize, &str)> = page
        .lines()
        .enumerate()
        .filter(|(_, line)| line.trim_start().starts_with("> **Answers:**"))
        .map(|(index, line)| (index + 1, line))
        .collect();
    assert_eq!(
        declared.len(),
        1,
        "{PAGE} declares {} answered-needs; a page answers one need",
        declared.len()
    );

    let (line, text) = declared[0];
    let first_body = page
        .lines()
        .enumerate()
        .skip(1)
        .find(|(_, body)| !body.trim().is_empty())
        .map(|(index, _)| index + 1)
        .expect("the page has a body");
    assert_eq!(
        line, first_body,
        "the answered-need line is not the first block element under the H1"
    );
    assert!(
        text.contains("`orientation`"),
        "{PAGE} does not declare `orientation`; its success condition is that the \
         reader leaves, correctly, within one screen"
    );
    assert!(
        text.contains(" — ") && text.trim_end().ends_with('?'),
        "the declaration is not in HS-P0021's notation: a backticked token, an em \
         dash, and a question in the reader's voice"
    );
}

/// AC-004 — the reader is oriented, and meets the DT-1 anchor, before leaving.
#[test]
fn orientation_precedes_departure() {
    let page = read(PAGE);
    let body = blocks(&page);
    assert!(
        body.len() >= 5,
        "{PAGE} has {} blocks; the composition is orientation, the DT-1 anchor, \
         the seam sentence, the explanation link, and the source link",
        body.len()
    );

    assert!(
        sentences(&body[0]) <= 2,
        "the orientation is {} sentences; two is a ceiling, and past it the \
         explanatory work belongs on the bridge page",
        sentences(&body[0])
    );

    let anchor = page
        .find(ANCHOR_LINK)
        .unwrap_or_else(|| panic!("{PAGE} does not cite {ANCHOR_LINK}"));
    let departure = page
        .find("overview.md)")
        .unwrap_or_else(|| panic!("{PAGE} does not link {OVERVIEW}"));
    assert!(
        anchor < departure,
        "the DT-1 anchor is cited after the outbound link; a reader who follows \
         the link does not come back"
    );

    for word in PRIOR_MODEL {
        assert!(
            !page.to_lowercase().contains(word),
            "{PAGE} names the prior model (`{word}`); it is named in exactly one \
             place in the page set, and that place is {BRIDGE}"
        );
    }
}

/// AC-005 — the seam is named once, truthfully, at the moment of leaving.
#[test]
fn seam_named_once_before_the_link() {
    let page = read(PAGE);
    assert_eq!(
        page.matches(SEAM).count(),
        1,
        "the seam is named {} times; it is one sentence, immediately before the \
         link, and nowhere else",
        page.matches(SEAM).count()
    );
    assert!(
        !page.contains(WRONG_CRATE),
        "{PAGE} names `{WRONG_CRATE}`; the merged example does not import it, so \
         the sentence would be a confident falsehood delivered as the reader leaves"
    );

    let seam = page.find(SEAM).expect("the seam was found above");
    let anchor = page
        .find(ANCHOR_LINK)
        .unwrap_or_else(|| panic!("{PAGE} does not cite {ANCHOR_LINK}"));
    let departure = page
        .find("overview.md)")
        .unwrap_or_else(|| panic!("{PAGE} does not link {OVERVIEW}"));
    assert!(
        anchor < seam && seam < departure,
        "the seam sentence is not between the DT-1 anchor and the outbound link"
    );

    let overview = read(OVERVIEW);
    for block in blocks(&page) {
        for sentence in block.replace('\n', " ").split(". ") {
            let trimmed = sentence.trim();
            if trimmed.len() < 40 {
                continue;
            }
            assert!(
                !overview.replace('\n', " ").contains(trimmed),
                "{PAGE} restates the destination verbatim: {trimmed:?}"
            );
        }
    }
}

/// AC-006 — the chain from the bridge to the example holds, in both links.
#[test]
fn the_chain_holds() {
    let bridge = read(BRIDGE);
    assert!(
        bridge.contains("read-the-worked-example.md"),
        "{BRIDGE} carries no link to {PAGE}; the good example is unreachable from \
         the page whose reader is looking for it"
    );
    assert!(
        bridge.contains(ANCHOR_HEADING),
        "{BRIDGE} no longer carries `{ANCHOR_HEADING}`, so {PAGE}'s DT-1 citation \
         points at a heading that is not there"
    );

    let page = read(PAGE);
    let found = links(&page);
    let targets: Vec<&str> = found.iter().map(|(_, target)| target.as_str()).collect();
    let overview_at = targets
        .iter()
        .position(|target| target.ends_with("overview.md"))
        .unwrap_or_else(|| panic!("{PAGE} does not link {OVERVIEW}"));
    let source_at = targets
        .iter()
        .position(|target| target.ends_with("main.rs"))
        .unwrap_or_else(|| panic!("{PAGE} does not link {MAIN}"));
    assert!(
        overview_at < source_at,
        "the source link comes before the explanation link; the reader leaves to \
         one file, then to the source"
    );
    assert_eq!(
        source_at,
        targets.len() - 1,
        "the link to {MAIN} is not last on the page"
    );

    for (text, target) in &found {
        assert!(
            !EMPTY_LINK_TEXT.contains(&text.to_lowercase().trim_matches('*')),
            "the link text {text:?} says nothing standing alone in a link list"
        );
        let path = target.split('#').next().unwrap_or(target);
        if path.is_empty() {
            continue;
        }
        assert!(
            root().join("docs").join(path).exists(),
            "the link {target:?} on {PAGE} resolves to no file on disk"
        );
    }
}

/// AC-007 — the page holds the substrate's budgets and introduces no control.
#[test]
fn page_holds_its_budgets() {
    assert!(
        PAGE.chars().count() <= PATH_CHARS,
        "{PAGE} is {} characters; the budget is {PATH_CHARS}",
        PAGE.chars().count()
    );

    let page = read(PAGE);
    let lines: Vec<&str> = page.lines().collect();
    assert!(
        lines.len() <= PAGE_LINES,
        "{PAGE} is {} source lines; the budget is {PAGE_LINES}",
        lines.len()
    );

    let h1: Vec<&str> = lines
        .iter()
        .filter(|line| line.starts_with("# "))
        .copied()
        .collect();
    assert_eq!(h1.len(), 1, "{PAGE} carries {} `h1` headings", h1.len());
    assert!(
        h1[0][2..].chars().count() <= H1_CHARS,
        "the H1 is {} characters; the budget is {H1_CHARS}",
        h1[0][2..].chars().count()
    );

    for line in &lines {
        if let Some(heading) = line.strip_prefix("## ") {
            assert!(
                heading.chars().count() <= HEADING_CHARS,
                "the heading {heading:?} is {} characters; the budget is {HEADING_CHARS}",
                heading.chars().count()
            );
        }
    }

    for (index, line) in lines.iter().enumerate() {
        if line.contains("](") || line.starts_with('|') {
            continue;
        }
        assert!(
            line.chars().count() <= WRAP,
            "{PAGE}:{} wraps at {} columns; the budget is {WRAP}",
            index + 1,
            line.chars().count()
        );
    }

    for marker in ["```", "<details", "<summary", "![", "{{#tab", "<!-- tab"] {
        assert!(
            !page.contains(marker),
            "{PAGE} carries `{marker}`; this page introduces zero fences, zero \
             images and zero affordances, and the enumeration of them is empty"
        );
    }
}
