//! The narrative tree's consistency check (`docs/`).
//!
//! `docs/` claims its Rust examples compile against the crates a reader
//! installed. [`crate::narrative_doctests`] discharges that claim for every page
//! the harness names. This module discharges the half a compiler cannot state:
//! that the tree is where the gate thinks it is, that it is not empty, and that
//! the set of pages on disk and the set of registrations in
//! [`HARNESS`] are the same set.
//!
//! # What this does not verify
//!
//! Stated first, because a check whose limits are undocumented is read as a
//! guarantee — [`crate::lint_constitution`] opens with the same argument, and
//! this corpus is the one most likely to be quoted as evidence of something it
//! never checked.
//!
//! * **Registration proves that a page is compiled. It does not prove that the
//!   page is correct.** A page this module reports as registered is a page
//!   rustdoc will hand to the compiler. Whether its prose still describes what
//!   its fences do is a question nothing in this repository asks; adjacency is
//!   what makes it reviewable by a human, and nothing makes it mechanical.
//! * **A compile failure names the harness, not the markdown.**
//!   `cargo test -p xtask --doc` reports ``xtask\src\../../docs/<page>.md -
//!   narrative::<page> (line N)``: the module resolves the page and the line
//!   resolves the location, and the path in front of both is `xtask/src/`.
//!   Registration is what keeps even that much true, and it is the whole of what
//!   it buys.
//! * **A Rust example deliberately tagged `text` is neither compiled nor
//!   flagged.** The fence walk rejects an untagged fence and refuses any info
//!   string it does not enumerate — in every spelling rustdoc accepts, which is
//!   what the section below is about — and the allowance list makes every
//!   `ignore` a thing a human approved with a reason attached. But `text` is a
//!   legitimate tag for prose, and a block whose info string carries no rustdoc
//!   tag at all *is* prose as far as the compiler is concerned, so an author who
//!   wants a Rust block the compiler never sees can still have one by calling it
//!   something else. This list narrows that hole and does not close it.
//! * **A block indented by four spaces or more is invisible to this walk.**
//!   `CommonMark` makes it an indented code block rather than a fence, and
//!   rustdoc compiles it — measured, not assumed. What escapes is therefore the
//!   *tagging* rule and not the compiler: an indented block carries no info
//!   string at all, so it cannot claim `ignore` and cannot opt out of anything.
//!   A diagnostic hole rather than a way through, and
//!   [`crate::narrative_doctests`] still compiles what is inside it.
//! * **A disclosure marker in a spelling this set does not carry is invisible.**
//!   [`HIDDEN_MARKERS`] is seven tokens, and a renderer that folds content on
//!   some eighth directive — a `<div>` with a theme's collapse class, a
//!   generator's own shortcode — passes untouched. The set is a closed
//!   enumeration of what DT-7 was decided against, not a proof that nothing can
//!   fold.
//! * **Disclosure produced outside the pinned tree is not this scan's
//!   business.** The rule is scoped to [`TREE`], deliberately: a scanner is
//!   scoped to the directory whose behaviour it constrains. Prose elsewhere in
//!   the repository may use `<details>` and this step will never say so.
//! * **The scan reads source, and cannot know what a renderer does with it.**
//!   It reports the bytes a page carries. Whether a particular host collapses,
//!   ignores or escapes them is outside anything this repository can observe,
//!   which is the whole reason the markers are rejected rather than measured.
//! * **An unresolvable clause id is still invisible here.** That is the
//!   `specification-pin` milestone's, landing in this same module.
//! * **The harness is matched as text, so reformatting it can break this check
//!   without breaking the compile.** Splitting an `include_str!` across lines, or
//!   writing a `mod` line that does not start with `mod ` after trimming, makes a
//!   registered page look unregistered. It is the coupling
//!   `lint_constitution::check_harness` already lives with, recorded here rather
//!   than defended against with a parser.
//! * **Nothing here says a page teaches anybody anything.** A green run of this
//!   step means the tree is where it is pinned, holds pages, and that every page
//!   is offered to the compiler. That is the whole of it.
//!
//! # Why the module is `lint_narrative` and the subcommand is `narrative`
//!
//! The module follows [`crate::lint_constitution`]'s precedent, and it cannot be
//! `xtask/src/narrative.rs` because that path is [`HARNESS`]'s own value — the
//! *lib*-target file this module reads as text. The subcommand is `narrative`
//! and the step's name is the claim it makes, `every narrative page is checked`,
//! because the banner is the only thing telling a reader which of the tree's two
//! steps failed. The asymmetry is deliberate rather than a slip.
//!
//! The two targets never link. `xtask` has a lib target and a bin target that do
//! not share modules: the harness is compiled by rustdoc out of
//! `xtask/src/lib.rs`, and this checker is a bin-crate module declared from
//! `xtask/src/main.rs`. So the bridge between them is `fs::read_to_string` and a
//! literal match, exactly as `check_harness` reads `xtask/src/constitution.rs`.
//! A `mod lint_narrative;` added to `xtask/src/lib.rs` by mistake would compile
//! clean and check nothing.
//!
//! # Why this check runs on every `cargo xtask affected` invocation
//!
//! [`crate::affected`] runs a fixed list of file-reading checks before it decides
//! which packages a diff touched, and `lint-constitution` is deliberately *not*
//! on it. This checker is, and the divergence is stated here rather than left for
//! the next contributor to read as a slip in one of the two.
//!
//! The argument is that module's own (`xtask/src/affected.rs:28-36`): these are
//! file reads that finish inside the time cargo takes to decide `xtask` is up to
//! date, and a story whose whole deliverable is prose is exactly the case a
//! package-shaped gate reads nothing for. `.redkiln/config.yaml` wires
//! `cargo xtask affected --base main` as the story grain every story in the
//! documentation initiative is held to, so a checker absent from that list is a
//! checker those stories never run. It is a convention, not a decision, and it
//! is discharged by this paragraph.
//!
//! # How a fence is recognised, and why it is not a `starts_with`
//!
//! Measured against the compiler rather than reasoned about. `rustdoc --test`
//! collects `` ```ignore ``, `` ```rust ignore ``, `` ```ignore,rust `` and
//! `` ~~~ignore `` as doctests and reports all four *ignored*; it collects a
//! three-space-indented fence and an untagged four-backtick block and compiles
//! both. Every one of those is a Rust block that a walk keyed on
//! `info.starts_with("rust")`, or on `line.starts_with("```")`, never sees —
//! and `` ```ignore `` is rustdoc's own canonical spelling, so it is the first
//! one an author reaches for.
//!
//! So the info string is read the way rustdoc reads it: split on `,`, a space
//! and a tab, empty parts dropped, and the block is a doctest when the token
//! list is empty or **any** token is a tag rustdoc itself defines. This tree's
//! accepted set is then matched over every token rather than over everything
//! after the first, which is what makes `ignore` a thing an
//! [`IGNORE_ALLOWANCES`] entry has to name wherever in the info string it
//! appears.
//!
//! Recognised by rustdoc and permitted here are two different sets, and the gap
//! is deliberate: `edition2024` and `test_harness` are rustdoc's, so a fence
//! carrying one is a doctest and every rule below applies to it — and neither is
//! on the accepted list, so it is *also* an unrecognised info string. A tree
//! whose examples pin their own edition is a tree whose examples stopped being
//! checked against the workspace's.
//!
//! Fences are delimited by `` ``` `` or `~~~`, indented by up to three spaces,
//! and closed only by at least as many of the same character with nothing after
//! them. That last rule is what makes a four-backtick block *quote* the fences
//! inside it, rather than a step-over of this parser's own: the previous
//! spelling toggled on any four-backtick line whatever its info string, so
//! `` ````ignore `` was an ignore-class doctest the walk never examined and one
//! stray opener disabled the walk for the rest of the page.
//!
//! # Why this tree's `ignore` rule is stricter than the constitution's
//!
//! `lint_constitution` permits an `ignore` fence when the line above it is an
//! `<!-- ignore: <reason> -->` comment. Under this tree the comment form is not
//! accepted at all: an `ignore`-class fence is permitted only by an entry in
//! [`IGNORE_ALLOWANCES`]. Two `ignore` rules in one repository is exactly the
//! shape a later contributor reads as a mistake in one of them, so the reason is
//! written here rather than inferred.
//!
//! A comment is reviewable only in the diff that introduced it, and a stale one
//! is undetectable — it sits above a fence that has changed underneath it and
//! reads as a live approval. A `const` array is one place a reviewer reads in
//! full without a `git log`, and it is *sweepable*: an entry naming a fence that
//! no longer exists, or one naming a fence that no longer opts out, is itself a
//! problem. That is the whole trade, and it is the same argument the
//! bidirectional registration check above rests on.
//!
//! Nothing about `lint_constitution` changes. The two corpora are supposed to
//! differ here, so a shared helper would have to be parameterised by exactly the
//! difference — which is the rule.
//!
//! # The constants are contracts, not details
//!
//! [`TREE`]'s *value* is a repository-wide contract: moving `docs/` without
//! editing that line fails the gate, which is the point of pinning it rather than
//! discovering by convention. `docs/README.md:25-29` is its prose mirror and has
//! to stay true. [`HARNESS`] names a file this module reads as text and never
//! links.

use std::fs;
use std::path::Path;

use anyhow::{Context, Result, bail};

use crate::spec_trace::workspace_root;

/// The narrative tree, pinned by path.
///
/// Moving or renaming the tree without editing this line fails the gate: the
/// read below is `?`-propagated, so the error names the path that was expected
/// rather than reporting an empty tree.
const TREE: &str = "docs";

/// The tree's index, which is the only file under [`TREE`] that is not a page.
///
/// It is routing rather than teaching, so [`HARNESS`] deliberately does not
/// register it — the shape `lint_constitution`'s `ROUTER` already has.
///
/// The exemption is from *registration* only, and it is one-way. The index is
/// still walked and still scanned, and [`check_fences`] refuses a Rust-class
/// fence on it outright: unregistered means nothing compiles it, so an example
/// here would be the one page in the pinned tree shipping unchecked.
const INDEX: &str = "docs/README.md";

/// The lib-crate harness whose `include_str!` lines register every page.
///
/// Read as text and never linked: it belongs to the other cargo target. The two
/// literals `include_str!("../../docs/<page>")` and `mod <module> {` are the
/// contract, and that coupling is stated as a limit in this module's docs.
const HARNESS: &str = "xtask/src/narrative.rs";

/// The longest repo-relative page path the terminal surface can carry.
///
/// The location prefix a reader has to see first is budgeted at 48 columns of an
/// 80-column log, and `cargo test --doc` puts its own 16-character
/// ``xtask\src\../../`` in front of every page path it names. 48 less 16 is this
/// number, and the checker enforces the stricter of the two surfaces because a
/// path that fits the compile surface necessarily fits this one.
const PATH_BUDGET: usize = 32;

/// This module's own path, which is the file a stale allowance is a defect in.
///
/// The shape [`HARNESS`] already has, one file over: a problem is reported
/// against the file that carries the mistake, and an entry of
/// [`IGNORE_ALLOWANCES`] that names nothing is a mistake here rather than on the
/// page it names.
const CHECKER: &str = "xtask/src/lint_narrative.rs";

/// Fences permitted to opt out of the compiler, enumerated.
///
/// `(page path, line-or-anchor, reason)`. The page path is repo-relative and
/// `/`-separated, so one entry means the same fence on Windows and on CI. The
/// line-or-anchor is a line number when it is all digits and otherwise a
/// substring of the fence's own body: **prefer the anchor**, because inserting a
/// paragraph above a fence moves every line-keyed entry below it, and an anchor
/// does not move at all. Either way the sweep reports the drift.
///
/// The reason is prose and this module never interprets it. It exists so a
/// reviewer reading the list in full knows what was approved and why — which is
/// the whole argument for a list rather than `lint_constitution`'s
/// `<!-- ignore: … -->` comment, and it is written out in this module's docs.
///
/// It ships empty, and it grows by review rather than as the repair for a
/// failing gate.
const IGNORE_ALLOWANCES: &[(&str, &str, &str)] = &[];

/// Hidden-content markers rejected anywhere under [`TREE`].
///
/// DT-7's resolution, as a token set. `_design.md` `## Pattern decision` D2,
/// signed off 2026-08-17 with no conditions, settled that scoped divergence is
/// written as visible level-3 subsections while it stays small and becomes one
/// page per scope past the threshold — and that hidden panels are rejected by
/// this checker, by file and line. There is no third state where scoped content
/// is present but hidden.
///
/// There is deliberately **no allowance list** for these, and that is a decision
/// rather than an omission. [`IGNORE_ALLOWANCES`] exists because the need for an
/// uncompiled fence is real, enumerable, and detectable when it goes stale; a
/// hidden-panel allowance would be permission to reintroduce, one page at a
/// time, a mechanism whose behaviour nothing in this repository can observe —
/// and no sweep can detect that.
///
/// Shrinking this set re-opens DT-7 and requires a new design record, not an
/// edit. A test pins it and names which token moved.
///
/// Every token is ASCII-lowercase, which is what makes the case-folded match in
/// [`check_hidden_markers`] correct rather than accidentally correct. The
/// trailing space in `{{#tab ` is load-bearing: without it the token shadows
/// `{{#tabs` and one `{{#tabs}}` line would report twice.
///
/// `pub(crate)` for one reason: [`crate::narrative_doctests`]'s fixture test
/// asserts the compiled page carries none of these, and it held a private copy
/// of the list while this constant did not exist. Two spellings of a set that
/// may only change by a new design record is one that can satisfy the pin below
/// and drift anyway, so the copy is deleted and this is the set.
pub(crate) const HIDDEN_MARKERS: &[&str] = &[
    "<details",
    "<summary",
    "{{#tabs",
    "{{#tab ",
    "{{#endtabs",
    "```admonish",
    "<!-- tab",
];

/// The gate step's name, which is also the claim it makes.
///
/// Named once, here, because `REQUIRED`, `lint_steps` and the tests that hold
/// that entry to `probe: None` all have to agree — and `steps_named` panics on a
/// name absent from `REQUIRED`, so a second spelling is a build-time bug rather
/// than a step that silently selects nothing.
pub(crate) const STEP: &str = "every narrative page is checked";

/// One page of the narrative tree.
#[derive(Debug)]
struct Page {
    /// Repo-relative, `/`-separated: `docs/append-conditions.md`.
    path: String,
    /// Tree-relative: `append-conditions.md`.
    rel: String,
    /// The doctest module the harness must declare: `append_conditions`.
    module: String,
    /// Whether this is the tree's index rather than a page.
    index: bool,
    /// The page's whole text, read once at enumeration.
    ///
    /// Held rather than re-read, so the fence walk and the marker scan are two
    /// checks over one read rather than two traversals of the tree.
    text: String,
}

impl Page {
    /// The page at `rel`, tree-relative with `/` separators.
    ///
    /// The module name is derived exactly once, here, so both directions of the
    /// registration check compare the same string. Two spellings of one
    /// derivation is how the halves of an orphan check start disagreeing.
    fn new(rel: &str, text: &str) -> Self {
        let path = format!("{TREE}/{rel}");
        Self {
            module: module_name(rel),
            index: path == INDEX,
            rel: rel.to_owned(),
            text: text.to_owned(),
            path,
        }
    }
}

/// The doctest module name a page is registered under.
///
/// The tree-relative path, minus the `.md`, with `/` and `-` mapped to `_`:
/// `append-conditions.md` becomes `append_conditions` and
/// `adapters/sqlite.md` becomes `adapters_sqlite`. Pure, and called from exactly
/// one place.
fn module_name(rel: &str) -> String {
    rel.strip_suffix(".md")
        .unwrap_or(rel)
        .replace(['/', '-'], "_")
}

/// Every page under [`TREE`], in path order.
///
/// Sorted once, here, so "source order" is a property of the page list rather
/// than something each check has to remember: `read_dir`'s order is undefined.
///
/// # Errors
///
/// Fails when [`TREE`] cannot be read — the case that must name the path the
/// gate expected rather than report an empty tree.
fn pages(root: &Path) -> Result<Vec<Page>> {
    let mut out = Vec::new();
    collect(root, "", &mut out)?;
    out.sort_by(|left, right| left.path.cmp(&right.path));
    Ok(out)
}

/// Every markdown file at or below `rel_dir`, which is tree-relative.
fn collect(root: &Path, rel_dir: &str, out: &mut Vec<Page>) -> Result<()> {
    let here = if rel_dir.is_empty() {
        TREE.to_owned()
    } else {
        format!("{TREE}/{rel_dir}")
    };

    let entries = fs::read_dir(root.join(&here)).with_context(|| format!("reading {here}"))?;

    // Names first, sorted, so a directory and a file at the same level are
    // walked in one defined order rather than the filesystem's.
    let mut found: Vec<(String, bool)> = Vec::new();
    for entry in entries {
        let entry = entry.with_context(|| format!("reading an entry of {here}"))?;
        let name = entry.file_name().to_string_lossy().into_owned();
        let is_dir = entry
            .file_type()
            .with_context(|| format!("reading the type of {here}/{name}"))?
            .is_dir();
        found.push((name, is_dir));
    }
    found.sort();

    for (name, is_dir) in found {
        let rel = if rel_dir.is_empty() {
            name.clone()
        } else {
            format!("{rel_dir}/{name}")
        };
        if is_dir {
            collect(root, &rel, out)?;
        } else if is_markdown(&name) {
            // Hard error rather than a skipped page: a scanner that silently
            // steps over what it cannot read reports green over exactly the file
            // it failed to inspect.
            let text = fs::read_to_string(root.join(TREE).join(&rel))
                .with_context(|| format!("reading {TREE}/{rel}"))?;
            out.push(Page::new(&rel, &text));
        }
    }
    Ok(())
}

/// Whether a file name is a markdown page, however it is cased.
///
/// Case-insensitive because `README.MD` is the same file to Windows and a
/// different one to a suffix comparison, and a page the walk declines to see is
/// a page nothing below ever checks.
fn is_markdown(name: &str) -> bool {
    name.len() > ".md".len()
        && name
            .get(name.len() - ".md".len()..)
            .is_some_and(|ext| ext.eq_ignore_ascii_case(".md"))
}

/// Refuses a tree with no pages in it.
///
/// A pinned constant without this guard reports green over a tree someone
/// emptied, which is the one failure this whole step exists to refuse. The index
/// is not a page: a tree holding only its own routing table holds nothing to
/// check.
///
/// # Errors
///
/// When [`TREE`] holds no page.
fn guard_not_vacuous(pages: &[Page]) -> Result<()> {
    if pages.iter().all(|page| page.index) {
        bail!("{TREE} holds no pages, so every check below is vacuous");
    }
    Ok(())
}

/// The location-prefix budget, enforced before any other line is emitted.
///
/// A path over [`PATH_BUDGET`], or one nested a third directory level under
/// [`TREE`], pushes the location off the first visual row of an 80-column log
/// and starves the surface a reviewer reads. The page-length and title budgets
/// stay review rules; this one is a gate rule because it protects the terminal
/// surface rather than the editorial one.
fn check_paths(pages: &[Page], problems: &mut Vec<String>) {
    for page in pages {
        let length = page.path.chars().count();
        if length > PATH_BUDGET {
            problems.push(format!(
                "{}:1 — the page path is {length} characters; the budget is {PATH_BUDGET}, \
                 because the 48-column location prefix also has to hold the 16-character \
                 `xtask\\src\\../../` a doctest name carries",
                page.path
            ));
        }
        if page.rel.matches('/').count() > 1 {
            problems.push(format!(
                "{}:1 — a third directory level under {TREE}; the location prefix budget \
                 allows {TREE}/ and at most one directory below it",
                page.path
            ));
        }
    }
}

/// Every page is registered, and every registration names a page.
///
/// Both directions, and the reverse one is what earns the check its keep:
/// `cfg(doctest)` hides a module left behind by a renamed page from every step
/// but `cargo test`, so a reviewer reading the harness cannot tell a live
/// registration from a dead one.
fn check_registration(pages: &[Page], harness: &str, problems: &mut Vec<String>) {
    for page in pages.iter().filter(|page| !page.index) {
        let include = format!("include_str!(\"../../{TREE}/{}\")", page.rel);
        let declared = format!("mod {} {{", page.module);
        if !harness.contains(&include) {
            problems.push(format!(
                "{HARNESS} — does not include {}; its examples are never compiled",
                page.rel
            ));
        }
        if !harness.contains(&declared) {
            problems.push(format!(
                "{HARNESS} — no `mod {}`; one module per page is what keeps a doctest \
                 failure's line number relative to the page",
                page.module
            ));
        }
    }

    for line in harness.lines() {
        let Some(rest) = line.trim().strip_prefix("mod ") else {
            continue;
        };
        let name = rest.trim_end_matches(" {");
        if !pages.iter().any(|page| !page.index && page.module == name) {
            problems.push(format!("{HARNESS} — `mod {name}` names no page in {TREE}"));
        }
    }
}

/// One fenced block, parsed far enough to check it.
#[derive(Debug)]
struct Fence {
    /// The info string as written, e.g. `rust,compile_fail,E0277`.
    info: String,
    /// 1-based line of the opening fence.
    line: usize,
    /// The fence's contents.
    body: String,
    /// Whether a closing fence was ever found.
    closed: bool,
}

/// How an [`IGNORE_ALLOWANCES`] entry was used during one walk.
#[derive(Debug, Default, Clone, Copy)]
struct Usage {
    /// How many `ignore`-class fences this entry permitted.
    permitted: usize,
    /// Whether it located a fence that is present and does not opt out.
    named_an_open_fence: bool,
}

/// One problem on one page, before it is composed into a line.
type Found = (usize, String);

/// A fence whose closing delimiter has not been found yet.
///
/// A named struct rather than the tuple the precedent carried, because the
/// closing rule needs the opener's delimiter *and* its length: a four-backtick
/// block is closed by four backticks and holds the three-backtick fences in
/// between as content, which is how a page quotes fenced material without being
/// flagged for what it quotes.
#[derive(Debug)]
struct Open {
    /// `` ` `` or `~`. A tilde fence is not closed by backticks.
    delimiter: char,
    /// How many of them the opener carried. A closer needs at least as many.
    length: usize,
    /// 0-based line of the opener.
    start: usize,
    /// The opener's info string, trimmed.
    info: String,
    /// Every line since, verbatim.
    body: Vec<String>,
}

impl Open {
    /// The finished fence, whether or not anything closed it.
    fn into_fence(self, closed: bool) -> Fence {
        Fence {
            info: self.info,
            line: self.start + 1,
            body: self.body.join("\n"),
            closed,
        }
    }
}

/// The fence delimiter a line carries: its character, its length, and the rest.
///
/// `CommonMark` allows a fence to be indented by up to three spaces and to be
/// written with `~` instead of a backtick, and rustdoc compiles both — measured,
/// not assumed. A parser keyed on `line.starts_with("```")` sees neither, which
/// makes indentation and a tilde two ways past every rule below.
fn fence_marker(line: &str) -> Option<(char, usize, &str)> {
    let mut rest = line;
    for _ in 0..3 {
        let Some(shorter) = rest.strip_prefix(' ') else {
            break;
        };
        rest = shorter;
    }

    let delimiter = rest
        .chars()
        .next()
        .filter(|character| *character == '`' || *character == '~')?;
    // Both delimiters are one byte, so the count is also the byte offset.
    let length = rest
        .chars()
        .take_while(|character| *character == delimiter)
        .count();
    if length < 3 {
        return None;
    }
    Some((delimiter, length, rest[length..].trim()))
}

/// Every fenced block on a page, in source order.
///
/// A closing fence carries the same delimiter as its opener, is at least as
/// long, and has nothing after it. That one rule replaces the precedent's
/// unconditional four-backtick step-over and does its job better: an inner
/// three-backtick fence is *content* of the four-backtick block that opened
/// before it, so a page quoting fenced material still reports nothing for what
/// it quotes — while `` ````ignore ``, which the step-over made invisible, is
/// now a fence like any other.
fn fences(text: &str) -> Vec<Fence> {
    let mut out = Vec::new();
    let mut open: Option<Open> = None;

    for (index, line) in text.lines().enumerate() {
        let marker = fence_marker(line);
        match open.take() {
            Some(mut current) => {
                let closes = marker.is_some_and(|(delimiter, length, rest)| {
                    delimiter == current.delimiter && length >= current.length && rest.is_empty()
                });
                if closes {
                    out.push(current.into_fence(true));
                } else {
                    current.body.push(line.to_owned());
                    open = Some(current);
                }
            }
            None => {
                if let Some((delimiter, length, info)) = marker {
                    open = Some(Open {
                        delimiter,
                        length,
                        start: index,
                        info: info.to_owned(),
                        body: Vec::new(),
                    });
                }
            }
        }
    }

    // The precedent parser drops an unpaired opener on the floor, so its info
    // string is never examined — an opt-out route inherited by copying. Fixed
    // here rather than in `lint_constitution`, whose corpus is not this one's.
    // It covers the four-backtick case too, which the step-over never could: an
    // unterminated quoted block silently swallowed the rest of the page.
    if let Some(current) = open {
        out.push(current.into_fence(false));
    }
    out
}

/// The info string's parts, tokenised the way rustdoc tokenises them.
///
/// Split on `,`, a space and a tab, with the empty parts dropped, so
/// `` ```rust ignore ``, `` ```rust,ignore `` and `` ```ignore,rust `` are one
/// block to the compiler and are one block here. Reading the info string as a
/// single string and asking whether it *starts with* `rust` is how
/// `` ```ignore `` — rustdoc's own canonical spelling — walked out of this check
/// entirely.
fn info_tokens(info: &str) -> Vec<&str> {
    info.split([',', ' ', '\t'])
        .filter(|token| !token.is_empty())
        .collect()
}

/// Whether rustdoc will hand this block to the compiler.
///
/// An empty info string is Rust, and so is any info string carrying a tag
/// rustdoc defines. Anything else — `text`, `markdown`, `console` — is prose,
/// and prose is none of this walk's business.
///
/// Where this and rustdoc's own rule differ, this one says "Rust" more often:
/// rustdoc demotes `` ```console ignore `` to prose because an unknown tag came
/// first, and here it stays a fence that needs an allowance. That direction
/// costs a contributor an explicit tag; the other direction costs the tree a
/// silent opt-out.
fn is_doctest(tokens: &[&str]) -> bool {
    tokens.is_empty() || tokens.iter().copied().any(is_rustdoc_tag)
}

/// Whether a token is a tag rustdoc itself defines.
///
/// Recognised is not permitted — see this module's docs. The point of the wider
/// set is only to decide whether the rules apply at all; which tokens this tree
/// *accepts* is the closed match in [`check_fences`], and it is narrower.
fn is_rustdoc_tag(token: &str) -> bool {
    matches!(
        token,
        "rust" | "ignore" | "no_run" | "should_panic" | "compile_fail" | "test_harness"
    ) || token.starts_with("ignore-")
        || is_edition(token)
        || is_error_code(token)
}

/// Whether a token is a rustdoc `editionNNNN` tag.
fn is_edition(token: &str) -> bool {
    token
        .strip_prefix("edition")
        .is_some_and(|year| year.len() == 4 && year.chars().all(|digit| digit.is_ascii_digit()))
}

/// Fence discipline, and the allowance list that is the only way out of it.
///
/// Five rules and one bookkeeping duty. A Rust-class fence on the tree's index
/// is rejected outright, because [`INDEX`] is never registered and so nothing
/// ever compiles what is in it. An untagged fence is rejected because rustdoc
/// compiles it as Rust regardless. The info string's tokens are matched against
/// a **closed** set — every token, not everything after the first — so a
/// spelling nobody enumerated is a hard error rather than a novel opt-out that
/// passes unnoticed. An `ignore`-class fence is permitted only by an
/// [`IGNORE_ALLOWANCES`] entry — never by a comment above it. And the error-code
/// rules are carried over from `check_fences` unchanged.
///
/// The bookkeeping is `usage`: which entry permitted which fence, recorded here
/// so [`check_allowances`] gets its reverse sweep out of the same walk.
fn check_fences(
    page: &Page,
    allowances: &[(&str, &str, &str)],
    usage: &mut [Usage],
    found: &mut Vec<Found>,
) {
    for fence in fences(&page.text) {
        let at = fence.line;

        if !fence.closed {
            found.push((
                at,
                "a fence opened here is never closed; every line below it reads as \
                 fenced content, which is an opt-out nothing reports"
                    .to_owned(),
            ));
        }

        let info = fence.info.as_str();
        let tokens = info_tokens(info);
        if !is_doctest(&tokens) {
            // A non-Rust tag is prose, and prose is not this walk's business.
            continue;
        }

        if page.index {
            // The one page the harness deliberately never registers. Reported
            // here rather than left as a documented limit, because a `rust`
            // fence on the index is compiled by nothing and would otherwise be
            // reported by nothing either.
            found.push((
                at,
                "the index is routing, not a page; it is never registered, so its \
                 examples are never compiled"
                    .to_owned(),
            ));
            continue;
        }

        if tokens.is_empty() {
            found.push((
                at,
                "an untagged fence is compiled as Rust; tag it `rust` or `text`".to_owned(),
            ));
            continue;
        }

        let mut recognised = true;
        let mut ignored = false;
        let mut compile_fail = false;
        let mut code: Option<&str> = None;
        for token in &tokens {
            match *token {
                "rust" | "no_run" | "should_panic" => {}
                "ignore" => ignored = true,
                "compile_fail" => compile_fail = true,
                other if is_error_code(other) => code = Some(other),
                // No accepting arm. This one character is the whole of AC-002,
                // and it now sees `ignore` wherever in the info string it sits.
                _ => recognised = false,
            }
        }
        if !recognised {
            found.push((at, format!("unrecognised fence info string `{info}`")));
        }
        if code.is_some() && !compile_fail {
            found.push((
                at,
                "an error code on a fence that is not `compile_fail`".to_owned(),
            ));
        }
        if compile_fail
            && let Some(code) = code
            && !names_outside_a_fence_marker(&page.text, code)
        {
            // rustdoc accepts a `compile_fail` whose code never matches, so the
            // prose naming the code is the part a reader can check.
            found.push((
                at,
                format!("the fence claims `{code}` and the page's prose never names it"),
            ));
        }

        let named: Vec<usize> = allowances
            .iter()
            .enumerate()
            .filter(|(_, entry)| allowance_names(entry, &page.path, at, &fence.body))
            .map(|(index, _)| index)
            .collect();

        if ignored {
            for &index in &named {
                usage[index].permitted += 1;
            }
            match named.len() {
                0 => found.push((
                    at,
                    "an `ignore` fence needs an `IGNORE_ALLOWANCES` entry naming it; \
                     a comment above the fence does not permit it, because a comment is \
                     reviewable only in the diff that introduced it"
                        .to_owned(),
                )),
                1 => {}
                _ => found.push((
                    at,
                    format!(
                        "two or more `IGNORE_ALLOWANCES` entries name this fence ({}); \
                         deleting one would leave the other silently authorising it",
                        named
                            .iter()
                            .map(|index| format!("`{}`", allowances[*index].2))
                            .collect::<Vec<_>>()
                            .join(", ")
                    ),
                )),
            }
        } else {
            for &index in &named {
                usage[index].named_an_open_fence = true;
            }
        }
    }
}

/// Whether a page names `code` somewhere other than a fence's own info string.
///
/// The info string is where the claim is *made*, so counting it as the prose
/// that supports it makes the rule vacuous — it would be satisfied by the very
/// line it is checking. [`fence_marker`] rather than a `starts_with`, so a
/// tilde-delimited or indented opener is excluded on the same terms.
fn names_outside_a_fence_marker(text: &str, code: &str) -> bool {
    text.lines()
        .filter(|line| fence_marker(line).is_none())
        .any(|line| line.contains(code))
}

/// Whether an info-string part is a rustc error code.
fn is_error_code(part: &str) -> bool {
    part.len() == 5
        && part.starts_with('E')
        && part[1..].chars().all(|digit| digit.is_ascii_digit())
}

/// Whether an allowance entry names this fence.
///
/// The page path is compared separator-normalised, so an entry written
/// `docs/adapters/sqlite.md` matches the same page discovered on Windows: a list
/// that silently matches nothing on one platform grants permission on one runner
/// and denies it on another. The key is a line number when it is all digits, and
/// otherwise an anchor matched inside the fence's own body.
fn allowance_names(entry: &(&str, &str, &str), page: &str, line: usize, body: &str) -> bool {
    let (path, key, _) = *entry;
    if path.replace('\\', "/") != page {
        return false;
    }
    if is_line_key(key) {
        key.parse::<usize>().is_ok_and(|wanted| wanted == line)
    } else {
        body.contains(key)
    }
}

/// Whether an allowance key is a line number rather than an anchor.
fn is_line_key(key: &str) -> bool {
    !key.is_empty() && key.chars().all(|character| character.is_ascii_digit())
}

/// The reverse sweep over the allowance list.
///
/// The half that earns the list. Forward, the walk catches a fence nobody
/// approved; here it catches an approval for a fence that is gone — the same
/// asymmetry [`check_registration`] documents, one corpus over. Without it the
/// list accumulates standing permission for code nobody has, and a malformed
/// entry sits inert while reading like a granted permission.
fn check_allowances(
    allowances: &[(&str, &str, &str)],
    usage: &[Usage],
    problems: &mut Vec<String>,
) {
    for (entry, used) in allowances.iter().zip(usage) {
        let (path, key, reason) = *entry;
        let at = format!("the `IGNORE_ALLOWANCES` entry for {path}:{key}");

        if reason.trim().is_empty() {
            problems.push(format!(
                "{CHECKER} — {at} carries no reason; an entry a reviewer cannot read \
                 is a permission nobody knowingly granted"
            ));
            continue;
        }
        if !path.replace('\\', "/").starts_with(&format!("{TREE}/")) {
            problems.push(format!(
                "{CHECKER} — {at} names a path that is not under {TREE}, so it can \
                 never match and can never be swept"
            ));
            continue;
        }
        if key.trim().is_empty() || key == "0" {
            problems.push(format!(
                "{CHECKER} — {at} carries no line-or-anchor, so it names a page rather \
                 than a fence"
            ));
            continue;
        }

        if used.permitted == 0 {
            if used.named_an_open_fence {
                problems.push(format!(
                    "{CHECKER} — {at} names a fence that no longer opts out; a line-keyed \
                     entry drifts the moment a paragraph is inserted above its fence"
                ));
            } else {
                problems.push(format!(
                    "{CHECKER} — {at} names no `ignore` fence in {TREE}"
                ));
            }
        }
    }
}

/// Every hidden-content marker on a page.
///
/// Line-based over the **whole** page, fenced blocks and the index included, and
/// ASCII-case-insensitive because HTML tag names are: a case-sensitive
/// `contains` accepts `<Details>` and `<DETAILS open>`, which is the same class
/// of hole as the elaborate spellings of `ignore` this repository has already
/// been bitten by. A fence-aware or comment-aware scan is the most plausible
/// refinement available and it is refused — a marker quoted in a fence still
/// renders as a page telling a reader to fold something, and there is no
/// allowance path to exempt it.
///
/// One problem per occurrence rather than per line or per page, so the count in
/// the terminal `bail!` is the number of things to fix. `to_ascii_lowercase`
/// leaves non-ASCII bytes alone, so the lowered copy is the same length as the
/// original and no offset can land inside a character.
fn check_hidden_markers(page: &Page, found: &mut Vec<Found>) {
    for (index, line) in page.text.lines().enumerate() {
        let lowered = line.to_ascii_lowercase();
        for token in HIDDEN_MARKERS {
            for _ in lowered.matches(token) {
                found.push((
                    index + 1,
                    format!("`{token}` is a hidden panel; DT-7 forbids it in {TREE}"),
                ));
            }
        }
    }
}

/// Every problem one page carries, composed and in line order.
fn check_page(
    page: &Page,
    allowances: &[(&str, &str, &str)],
    usage: &mut [Usage],
    problems: &mut Vec<String>,
) {
    let mut found: Vec<Found> = Vec::new();
    check_fences(page, allowances, usage, &mut found);
    check_hidden_markers(page, &mut found);
    // Stable, so two problems on one line keep the order they were found in and
    // a marker interleaves with a fence problem by line rather than by check.
    found.sort_by_key(|(line, _)| *line);
    for (line, message) in found {
        problems.push(format!("{}:{line} — {message}", page.path));
    }
}

/// Every problem the tree carries, composed, in source order.
///
/// The order is the order a contributor reads: the tree first — the pinning
/// check before any other line, then each page in path order and each page's
/// problems in line order — and after it the two files that register and permit,
/// the harness and this module. Nothing short-circuits and nothing is truncated:
/// a check that stops at the first problem turns one review cycle into six.
fn problems(pages: &[Page], harness: &str) -> Vec<String> {
    let mut problems = Vec::new();
    check_paths(pages, &mut problems);

    let mut usage = vec![Usage::default(); IGNORE_ALLOWANCES.len()];
    for page in pages {
        check_page(page, IGNORE_ALLOWANCES, &mut usage, &mut problems);
    }

    check_registration(pages, harness, &mut problems);
    check_allowances(IGNORE_ALLOWANCES, &usage, &mut problems);
    problems
}

/// The one line a green run prints.
///
/// One line, because a green check that says nothing is indistinguishable from a
/// check that did not run, and a green check that says ten lines trains people to
/// skip its output. The count is pages, so it is the number the registration
/// check actually compared.
fn summary(pages: &[Page]) -> String {
    let count = pages.iter().filter(|page| !page.index).count();
    format!("  {count} pages, all consistent")
}

/// Runs every check, reporting all problems rather than the first.
///
/// # Errors
///
/// Fails when the tree is missing or empty, when the harness cannot be read, or
/// when any check finds a problem.
pub(crate) fn run() -> Result<()> {
    let root = workspace_root()?;
    let pages = pages(&root)?;
    guard_not_vacuous(&pages)?;

    let harness =
        fs::read_to_string(root.join(HARNESS)).with_context(|| format!("reading {HARNESS}"))?;
    let problems = problems(&pages, &harness);

    if problems.is_empty() {
        println!("{}", summary(&pages));
        return Ok(());
    }

    for problem in &problems {
        eprintln!("  {problem}");
    }
    bail!("{} problem(s) in {TREE}", problems.len())
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, reason = "test code, per the house style")]

    use std::path::PathBuf;

    use super::*;
    use crate::REQUIRED;

    /// The composition root every mount assertion reads as text.
    const ROOT_MODULE: &str = "xtask/src/main.rs";

    /// The module whose unconditional file-reading list this checker joins.
    const AFFECTED: &str = "xtask/src/affected.rs";

    fn read(rel: &str) -> String {
        let root: PathBuf = workspace_root().unwrap();
        fs::read_to_string(root.join(rel)).unwrap_or_else(|err| panic!("reading {rel}: {err}"))
    }

    fn page(rel: &str) -> Page {
        Page::new(rel, "")
    }

    fn page_with(rel: &str, text: &str) -> Page {
        Page::new(rel, text)
    }

    /// Every problem one page carries under `allowances`, plus the sweep over
    /// them — the two halves of the walk a real run always performs together.
    fn walk(pages: &[Page], allowances: &[(&str, &str, &str)]) -> Vec<String> {
        let mut usage = vec![Usage::default(); allowances.len()];
        let mut problems = Vec::new();
        for page in pages {
            check_page(page, allowances, &mut usage, &mut problems);
        }
        check_allowances(allowances, &usage, &mut problems);
        problems
    }

    /// This module's own source, without its tests — the half whose prose ships.
    fn production_source() -> &'static str {
        include_str!("lint_narrative.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap()
    }

    /// A harness registering `rel` in both of the two ways the check reads.
    fn registration(rel: &str) -> String {
        format!(
            "#[cfg(doctest)]\nmod {} {{\n    #![doc = include_str!(\"../../{TREE}/{rel}\")]\n}}\n",
            module_name(rel)
        )
    }

    fn step_index(name: &str) -> usize {
        REQUIRED
            .iter()
            .position(|step| step.name == name)
            .unwrap_or_else(|| panic!("REQUIRED must contain the `{name}` step"))
    }

    // ---- AC-001: a moved tree is an error naming the path it expected -------

    /// A message reporting *zero pages* does not satisfy AC-001. The read is
    /// `?`-propagated with a context naming the pinned constant, so a
    /// contributor who moved `docs/` learns the tree is pinned rather than
    /// discovering weeks later that nothing was ever read.
    #[test]
    fn a_missing_tree_is_an_error_naming_the_pinned_path() {
        let err = pages(Path::new("this-root-does-not-exist"))
            .expect_err("a missing tree must be an error, never an empty page list");

        let chain = format!("{err:#}");
        assert!(
            chain.contains(TREE),
            "the error chain must name the pinned tree, got: {chain}"
        );
    }

    // ---- AC-002: an empty tree is a hard error, before any check ------------

    #[test]
    fn an_empty_tree_is_a_hard_error() {
        let err = guard_not_vacuous(&[])
            .expect_err("an empty tree must fail before any check below it runs");

        let message = err.to_string();
        assert!(
            message.contains(TREE) && message.contains("vacuous"),
            "the guard must say which tree is empty and that the checks would be vacuous, \
             got: {message}"
        );
    }

    /// A tree holding only its index holds no pages, and the index is never
    /// registered — so counting files rather than pages is the shape that
    /// passes green over an emptied tree.
    #[test]
    fn a_tree_holding_only_its_index_is_still_vacuous() {
        assert!(guard_not_vacuous(&[page("README.md")]).is_err());
    }

    /// And the guard is not simply always-failing.
    #[test]
    fn a_tree_with_one_page_passes_the_guard() {
        assert!(guard_not_vacuous(&[page("append-conditions.md")]).is_ok());
    }

    // ---- AC-003: a page nobody registered is named ------------------------

    #[test]
    fn a_page_the_harness_does_not_include_is_a_problem() {
        let pages = [page("append-conditions.md")];
        let harness = "#[cfg(doctest)]\nmod append_conditions {\n}\n";
        let mut problems = Vec::new();

        check_registration(&pages, harness, &mut problems);

        assert_eq!(problems.len(), 1, "got: {problems:?}");
        assert!(
            problems[0].contains("append-conditions.md") && problems[0].contains("never compiled"),
            "the problem must name the page and say its examples are never compiled, \
             got: {}",
            problems[0]
        );
    }

    /// The two halves are separate problems, exactly as `check_harness` keeps
    /// them separate: one says the page is not compiled, the other says the
    /// failure's line number would stop being relative to the page.
    #[test]
    fn a_page_the_harness_does_not_declare_is_a_distinct_problem() {
        let pages = [page("append-conditions.md")];
        let harness = "#![doc = include_str!(\"../../docs/append-conditions.md\")]\n";
        let mut problems = Vec::new();

        check_registration(&pages, harness, &mut problems);

        assert_eq!(problems.len(), 1, "got: {problems:?}");
        assert!(
            problems[0].contains("mod append_conditions"),
            "the problem must name the module the harness is missing, got: {}",
            problems[0]
        );
    }

    /// The index is not a page and is deliberately never registered; reporting
    /// it as an orphan would make the tree permanently red.
    #[test]
    fn the_index_is_not_expected_to_be_registered() {
        let pages = [page("README.md"), page("append-conditions.md")];
        let mut problems = Vec::new();

        check_registration(&pages, &registration("append-conditions.md"), &mut problems);

        assert!(problems.is_empty(), "got: {problems:?}");
    }

    /// The exemption above is from registration only, and it is one-way. A page
    /// in the pinned tree that no mechanism compiles is exactly what AC-005
    /// refuses — and the index is the one page the harness never registers, so
    /// an example on it would be compiled by nothing and reported by nothing.
    #[test]
    fn a_rust_class_fence_on_the_index_is_a_problem() {
        for text in [
            "```rust\nfn main() {}\n```\n",
            "```\nfn main() {}\n```\n",
            "```rust,ignore\n```\n",
        ] {
            let found = walk(&[page_with("README.md", text)], &[]);

            assert_eq!(found.len(), 1, "`{text}` got: {found:?}");
            assert!(
                found[0].starts_with("docs/README.md:1 — ")
                    && found[0].contains("never registered"),
                "the problem must name the index and say why it cannot carry an example, \
                 got: {}",
                found[0]
            );
        }
    }

    /// Prose on the index is still prose: the rule is about what a compiler
    /// would be handed, not about fences.
    #[test]
    fn a_text_fence_on_the_index_is_not_a_problem() {
        let found = walk(&[page_with("README.md", "```text\nnot rust\n```\n")], &[]);

        assert!(found.is_empty(), "got: {found:?}");
    }

    /// And the real index, as it stands, carries no example at all.
    #[test]
    fn the_real_index_carries_no_rust_fence() {
        let index = read(INDEX);
        let mut usage: Vec<Usage> = Vec::new();
        let mut found = Vec::new();

        check_fences(&page_with("README.md", &index), &[], &mut usage, &mut found);

        assert!(
            found.is_empty(),
            "{INDEX} carries a fence problem: {found:?}"
        );
    }

    // ---- AC-004: a registration nobody deleted is named --------------------

    /// The direction that earns the check its keep. `cfg(doctest)` hides a
    /// module left behind by a renamed page from every step but `cargo test`,
    /// and this direction is unreachable from a real gate run once tree and
    /// harness agree — which is why it is tested against a harness string.
    #[test]
    fn a_registration_naming_no_page_is_a_problem() {
        let pages = [page("append-conditions.md")];
        let harness = format!(
            "{}{}",
            registration("append-conditions.md"),
            "#[cfg(doctest)]\nmod renamed_away {\n}\n"
        );
        let mut problems = Vec::new();

        check_registration(&pages, &harness, &mut problems);

        assert_eq!(problems.len(), 1, "got: {problems:?}");
        assert!(
            problems[0].contains("mod renamed_away") && problems[0].contains(TREE),
            "the problem must name the stale module and the tree it names nothing in, \
             got: {}",
            problems[0]
        );
    }

    /// The companion: the same harness with the page present yields none.
    #[test]
    fn the_same_harness_with_the_page_present_yields_no_problem() {
        let pages = [page("append-conditions.md"), page("renamed-away.md")];
        let harness = format!(
            "{}{}",
            registration("append-conditions.md"),
            registration("renamed-away.md")
        );
        let mut problems = Vec::new();

        check_registration(&pages, &harness, &mut problems);

        assert!(problems.is_empty(), "got: {problems:?}");
    }

    /// Both directions compare the same derived string, so a nested page cannot
    /// be registered forward and orphaned in reverse.
    #[test]
    fn the_module_name_is_one_derivation_for_both_directions() {
        assert_eq!(module_name("append-conditions.md"), "append_conditions");
        assert_eq!(module_name("adapters/sqlite.md"), "adapters_sqlite");

        let pages = [page("adapters/sqlite.md")];
        let mut problems = Vec::new();
        check_registration(&pages, &registration("adapters/sqlite.md"), &mut problems);

        assert!(problems.is_empty(), "got: {problems:?}");
    }

    // ---- AC-005: the mount, on all four invocation paths --------------------

    /// `probe: Some(..)` means *skip when absent* (`xtask/src/main.rs:89-102`),
    /// and a documentation step that can skip is `RUNBOOK.md:918-925` again.
    #[test]
    fn the_checker_step_is_required_and_unprobed() {
        let step = &REQUIRED[step_index(STEP)];
        assert!(
            step.probe.is_none(),
            "the checker must be mandatory on every runner, with no tool to install"
        );
    }

    /// The banner is the only thing telling a reader which half failed, so the
    /// step's name is the claim sentence rather than a noun phrase.
    #[test]
    fn the_step_is_named_as_a_claim() {
        assert_eq!(STEP, "every narrative page is checked");
    }

    /// Compile first, then check: the two banners read in the order
    /// `_design.md`'s `## Composition` draws them.
    #[test]
    fn the_checker_step_follows_the_narrative_compile_step() {
        assert!(
            step_index(crate::narrative_doctests::STEP) < step_index(STEP),
            "the tree's examples are compiled before the tree is checked"
        );
    }

    /// RS-80-4: every gate invocation that resolves dependencies passes it.
    #[test]
    fn the_checker_step_passes_locked_and_names_its_subcommand() {
        let step = &REQUIRED[step_index(STEP)];
        assert!(step.args.contains(&"--locked"));
        assert!(step.args.contains(&"narrative"));
    }

    /// `steps_named` panics on a name absent from `REQUIRED`, so a half-mount
    /// fails the moment `cargo xtask lints` selects it.
    #[test]
    fn the_gate_can_select_the_checker_step_by_name() {
        let selected = crate::steps_named(&[STEP]);
        assert_eq!(selected.len(), 1);
        assert_eq!(selected[0].name, STEP);
    }

    /// It is one of the file-reading checks, so `cargo xtask lints` runs it.
    #[test]
    fn the_checker_step_is_a_lint_step() {
        assert!(
            crate::lint_steps().iter().any(|step| step.name == STEP),
            "`cargo xtask lints` must select the narrative checker"
        );
    }

    /// Reachable by typing a name: the dispatch arm and the help line.
    #[test]
    fn the_subcommand_is_dispatched_and_listed_in_the_help() {
        let main = read(ROOT_MODULE);

        assert!(
            main.contains("mod lint_narrative;"),
            "the checker must be declared from the *bin* crate; a `mod` in lib.rs \
             compiles clean and checks nothing"
        );
        assert!(
            main.contains("Some(\"narrative\") => lint_narrative::run()"),
            "`cargo xtask narrative` must dispatch to the checker"
        );
        assert!(
            main.contains("println!(\"  narrative\");"),
            "`cargo xtask` with no argument must list the subcommand"
        );
    }

    /// The story-grain gate. A prose-only diff selects `xtask`, and the checker
    /// runs before any package selection at all.
    #[test]
    fn the_checker_joins_the_unconditional_file_reading_list() {
        let affected = read(AFFECTED);
        assert!(
            affected.contains("crate::lint_narrative::run()?;"),
            "`cargo xtask affected` must run the checker whatever the diff touched"
        );
    }

    // ---- AC-006: every problem, source order, composed, never truncated -----

    #[test]
    fn every_problem_is_reported_in_source_order_and_none_is_elided() {
        let pages = [
            page("a/b/nested.md"),
            page("append-conditions.md"),
            page("orphan.md"),
        ];
        let harness = format!(
            "{}{}{}",
            registration("a/b/nested.md"),
            registration("append-conditions.md"),
            "#[cfg(doctest)]\nmod renamed_away {\n}\n"
        );

        let found = problems(&pages, &harness);

        assert_eq!(
            found.len(),
            4,
            "every problem is reported, never the first only: {found:?}"
        );
        assert!(
            found.iter().all(|problem| !problem.contains("more")),
            "no problem list is truncated: {found:?}"
        );

        // Source order: the tree first, then the harness that registers it.
        assert!(found[0].starts_with("docs/a/b/nested.md:1 — "));
        assert!(found[1].starts_with("xtask/src/narrative.rs — "));
        assert!(found[2].starts_with("xtask/src/narrative.rs — "));
        assert!(found[3].starts_with("xtask/src/narrative.rs — "));
    }

    /// The composed form, character for character: `{path}:{line}` first, an em
    /// dash, then the message. Nothing is a `Debug` dump or a raw error chain.
    #[test]
    fn a_problem_is_a_composed_line() {
        let rel = format!("{}.md", "a".repeat(25));
        let mut found = Vec::new();
        check_paths(&[page(&rel)], &mut found);

        assert_eq!(found.len(), 1, "got: {found:?}");
        let (location, message) = found[0]
            .split_once(" — ")
            .unwrap_or_else(|| panic!("no em dash separator in `{}`", found[0]));
        assert_eq!(location, format!("{TREE}/{rel}:1"));
        assert!(!message.is_empty());
    }

    // ---- AC-007: a green run prints exactly one line ------------------------

    #[test]
    fn a_green_run_prints_exactly_one_summary_line() {
        let pages = [page("README.md"), page("append-conditions.md")];
        let line = summary(&pages);

        assert_eq!(line.lines().count(), 1, "got: {line:?}");
        assert_eq!(line, "  1 pages, all consistent");
        assert!(!line.contains("skipped"));
    }

    // ---- AC-008: the path budget, at its four corners ----------------------

    #[test]
    fn the_path_budget_is_enforced_at_its_four_corners() {
        let inside = format!("{}.md", "a".repeat(23));
        let outside = format!("{}.md", "a".repeat(25));
        assert_eq!(page(&inside).path.chars().count(), 31);
        assert_eq!(page(&outside).path.chars().count(), 33);

        for (rel, expected) in [
            (inside.as_str(), 0),
            (outside.as_str(), 1),
            ("a/b/c.md", 1),
            ("adapters/sqlite.md", 0),
        ] {
            let mut problems = Vec::new();
            check_paths(&[page(rel)], &mut problems);
            assert_eq!(
                problems.len(),
                expected,
                "docs/{rel} should yield {expected} problem(s), got: {problems:?}"
            );
        }
    }

    // ---- AC-009: the limits are stated first, and nothing claims teaching ---

    /// The shape of `xtask/src/lint_constitution.rs:9-13`: a check whose limits
    /// are undocumented is read as a guarantee.
    #[test]
    fn the_module_states_its_limits_first() {
        let first_heading = production_source()
            .lines()
            .take_while(|line| line.starts_with("//!"))
            .find_map(|line| line.strip_prefix("//! #"))
            .unwrap_or_else(|| panic!("this module's docs carry no headings at all"));

        assert_eq!(
            first_heading.trim(),
            "What this does not verify",
            "the limits must be the first thing in the module's docs"
        );
    }

    /// Two obligations in one place: the limits this story's own checks create,
    /// and the `affected::run` divergence recorded rather than left implicit.
    #[test]
    fn the_module_states_its_own_limits_and_its_one_divergence() {
        let docs: String = production_source()
            .lines()
            .take_while(|line| line.starts_with("//!"))
            .collect::<Vec<_>>()
            .join("\n");

        for claim in [
            "does not prove that the",
            "names the harness, not the markdown",
            "affected",
            "lint-constitution",
            "teach",
        ] {
            assert!(docs.contains(claim), "the module docs must state `{claim}`");
        }
    }

    /// Project `DoD` item 8: nothing here claims the surface proves a page
    /// teaches. `_design.md` anti-pattern 9 forbids the mark that would say so.
    #[test]
    fn nothing_in_the_module_claims_a_page_teaches() {
        let source = production_source();

        for mark in ["verified", "badge", "shield"] {
            assert!(
                !source.contains(mark),
                "`{mark}` reads as a claim this check cannot make"
            );
        }
    }

    // ======================================================================
    // fence-discipline-and-allowance-list
    // ======================================================================

    // ---- AC-001: an untagged fence is compiled as Rust regardless ----------

    /// rustdoc compiles an untagged fence as Rust whatever the author meant, so
    /// silence here means either prose is compiled by accident or Rust is
    /// compiled that nobody decided to check.
    #[test]
    fn an_untagged_fence_is_rejected() {
        let page = page_with("append-conditions.md", "intro\n\n```\nfn main() {}\n```\n");

        let found = walk(&[page], &[]);

        assert_eq!(found.len(), 1, "got: {found:?}");
        assert_eq!(
            found[0],
            "docs/append-conditions.md:3 — an untagged fence is compiled as Rust; \
             tag it `rust` or `text`"
        );
    }

    /// The back door this walk narrows and does not close: a `text` tag is
    /// neither compiled nor flagged, and the module docs say so.
    #[test]
    fn a_text_tagged_fence_is_neither_compiled_nor_flagged() {
        let page = page_with("append-conditions.md", "```text\nnot rust\n```\n");
        assert!(walk(&[page], &[]).is_empty());
    }

    // ---- AC-002: the info string is matched exhaustively -------------------

    /// The whole mechanism is that the match has no accepting wildcard arm. A
    /// novel opt-out spelling is a hard error rather than a silent pass, which
    /// is RS-81-2's posture one medium over.
    #[test]
    fn an_unrecognised_info_string_part_is_a_problem() {
        for info in ["rust,ignore_me", "rust,norun", "rust,edition2027"] {
            let page = page_with("append-conditions.md", &format!("```{info}\n```\n"));
            let found = walk(&[page], &[]);

            assert_eq!(found.len(), 1, "`{info}` got: {found:?}");
            assert!(
                found[0].contains(&format!("unrecognised fence info string `{info}`")),
                "the problem must quote the whole info string, got: {}",
                found[0]
            );
        }
    }

    /// The parts that *are* enumerated stay accepted, so the closed match is a
    /// rule about unknown spellings rather than a rule against every attribute.
    #[test]
    fn the_enumerated_info_string_parts_are_accepted() {
        for info in ["rust", "rust,no_run", "rust,should_panic", "text"] {
            let page = page_with("append-conditions.md", &format!("```{info}\n```\n"));
            assert!(walk(&[page], &[]).is_empty(), "`{info}` should be accepted");
        }
    }

    /// rustdoc splits an info string on `,`, a space and a tab, so all of these
    /// are one block to the compiler — measured with `rustdoc --test`, which
    /// collects each of them and reports it *ignored*. A walk asking whether the
    /// info string *starts with* `rust` calls the first three prose and walks
    /// past, which is an `ignore` in rustdoc's own canonical spelling opting out
    /// of the compiler with the gate green.
    #[test]
    fn every_ignore_spelling_rustdoc_accepts_needs_an_allowance() {
        for info in [
            "ignore",
            "rust ignore",
            "ignore,rust",
            "rust,ignore",
            "ignore rust",
            "ignore,no_run",
        ] {
            let text = format!("```{info}\nfn f() {{}}\n```\n");
            let found = walk(&[page_with("append-conditions.md", &text)], &[]);

            assert_eq!(found.len(), 1, "`{info}` got: {found:?}");
            assert!(
                found[0].starts_with("docs/append-conditions.md:1 — ")
                    && found[0].contains("IGNORE_ALLOWANCES"),
                "`{info}` must need an allowance, naming file and line, got: {}",
                found[0]
            );
        }
    }

    /// And one allowance permits the fence whichever spelling it was written in,
    /// because the walk compares tokens rather than the string.
    #[test]
    fn an_allowance_permits_an_ignore_fence_in_any_of_those_spellings() {
        let allowances = [(
            "docs/append-conditions.md",
            "fn needs_a_database",
            "the example needs a running database",
        )];

        for info in ["ignore", "rust ignore", "ignore,rust"] {
            let text = format!("```{info}\nfn needs_a_database() {{}}\n```\n");
            let found = walk(&[page_with("append-conditions.md", &text)], &allowances);

            assert!(found.is_empty(), "`{info}` got: {found:?}");
        }
    }

    /// `rust` is not the only tag that makes rustdoc compile a block: `no_run`,
    /// `compile_fail` and `should_panic` do it alone, and the probe collects all
    /// three. So the rules apply to them, which the `,zzz` half proves — a walk
    /// that skipped them would accept every unknown token beside them too.
    #[test]
    fn a_rust_class_tag_without_the_rust_token_is_still_walked() {
        for info in ["no_run", "compile_fail", "should_panic"] {
            let clean = format!("```{info}\n```\n");
            assert!(
                walk(&[page_with("append-conditions.md", &clean)], &[]).is_empty(),
                "`{info}` should be accepted"
            );

            let novel = format!("```{info},zzz\n```\n");
            let found = walk(&[page_with("append-conditions.md", &novel)], &[]);

            assert_eq!(found.len(), 1, "`{info},zzz` got: {found:?}");
            assert!(
                found[0].contains("unrecognised fence info string"),
                "got: {}",
                found[0]
            );
        }
    }

    /// An error code alone is a Rust block to rustdoc, so the rule that an error
    /// code needs its `compile_fail` reaches it too.
    #[test]
    fn a_bare_error_code_is_a_rust_fence() {
        let page = page_with("append-conditions.md", "```E0277\n```\n");
        let found = walk(&[page], &[]);

        assert_eq!(found.len(), 1, "got: {found:?}");
        assert!(
            found[0].contains("an error code on a fence that is not `compile_fail`"),
            "got: {}",
            found[0]
        );
    }

    /// Recognised by rustdoc and permitted here are two different sets, and the
    /// asymmetry is the mechanism: a fence carrying one of these *is* a doctest,
    /// so the closed match runs over it — and refuses it, because a page pinning
    /// its own edition is a page that stopped being checked against the
    /// workspace's.
    #[test]
    fn a_rustdoc_tag_this_tree_does_not_accept_is_unrecognised() {
        for info in [
            "edition2024",
            "rust,edition2021",
            "test_harness",
            "ignore-x86",
        ] {
            let page = page_with("append-conditions.md", &format!("```{info}\n```\n"));
            let found = walk(&[page], &[]);

            assert_eq!(found.len(), 1, "`{info}` got: {found:?}");
            assert!(
                found[0].contains(&format!("unrecognised fence info string `{info}`")),
                "got: {}",
                found[0]
            );
        }
    }

    #[test]
    fn an_error_code_without_compile_fail_is_a_problem() {
        let page = page_with("append-conditions.md", "```rust,E0277\n```\n");
        let found = walk(&[page], &[]);

        assert_eq!(found.len(), 1, "got: {found:?}");
        assert!(
            found[0].contains("an error code on a fence that is not `compile_fail`"),
            "got: {}",
            found[0]
        );
    }

    /// rustdoc accepts a `compile_fail` whose code never matches, so the prose
    /// naming the code is the part a reader can check.
    #[test]
    fn a_compile_fail_claiming_a_code_the_prose_never_names_is_a_problem() {
        let page = page_with("append-conditions.md", "```rust,compile_fail,E0277\n```\n");
        let found = walk(&[page], &[]);

        assert_eq!(found.len(), 1, "got: {found:?}");
        assert!(found[0].contains("E0277"), "got: {}", found[0]);

        let named = page_with(
            "append-conditions.md",
            "the trait bound fails with E0277.\n\n```rust,compile_fail,E0277\n```\n",
        );
        assert!(walk(&[named], &[]).is_empty());
    }

    // ---- AC-003: an `ignore` fence needs an enumerated allowance -----------

    #[test]
    fn an_unlisted_ignore_fence_is_rejected() {
        let page = page_with(
            "append-conditions.md",
            "a\n\n```rust,ignore\nfn f() {}\n```\n",
        );
        let found = walk(&[page], &[]);

        assert_eq!(found.len(), 1, "got: {found:?}");
        assert!(
            found[0].starts_with("docs/append-conditions.md:3 — "),
            "the problem must name the page and the fence's line, got: {}",
            found[0]
        );
        assert!(found[0].contains("IGNORE_ALLOWANCES"), "got: {}", found[0]);
    }

    /// The named wrong implementation from the testing brief: `lint_constitution`
    /// permits `ignore` when the line above is `<!-- ignore: … -->`. Under the
    /// narrative tree that comment rescues nothing, because a comment is
    /// reviewable only in the diff that introduced it.
    #[test]
    fn a_comment_above_an_ignore_fence_does_not_permit_it() {
        let page = page_with(
            "append-conditions.md",
            "<!-- ignore: needs a running database -->\n```rust,ignore\n```\n",
        );

        assert_eq!(walk(&[page], &[]).len(), 1);
    }

    #[test]
    fn a_listed_ignore_fence_passes() {
        let page = page_with(
            "append-conditions.md",
            "a\n\n```rust,ignore\nfn f() {}\n```\n",
        );
        let allowances = [(
            "docs/append-conditions.md",
            "3",
            "the example needs a running database",
        )];

        assert!(walk(&[page], &allowances).is_empty());
    }

    /// Both halves of "line-or-anchor" work, and the anchor is the recommended
    /// one because it does not move when a paragraph is inserted above it.
    #[test]
    fn an_anchored_allowance_survives_an_insertion_above_the_fence() {
        let allowances = [(
            "docs/append-conditions.md",
            "fn needs_a_database",
            "the example needs a running database",
        )];
        let fence = "```rust,ignore\nfn needs_a_database() {}\n```\n";

        assert!(walk(&[page_with("append-conditions.md", fence)], &allowances).is_empty());
        assert!(
            walk(
                &[page_with(
                    "append-conditions.md",
                    &format!("a new paragraph\n\n{fence}")
                )],
                &allowances
            )
            .is_empty(),
            "an anchored allowance must survive an insertion above its fence"
        );
    }

    #[test]
    fn an_allowance_with_an_empty_reason_is_a_problem() {
        let page = page_with("append-conditions.md", "```rust,ignore\n```\n");
        let allowances = [("docs/append-conditions.md", "1", "")];

        let found = walk(&[page], &allowances);

        assert_eq!(found.len(), 1, "got: {found:?}");
        assert!(
            found[0].starts_with("xtask/src/lint_narrative.rs — "),
            "got: {}",
            found[0]
        );
        assert!(found[0].contains("reason"), "got: {}", found[0]);
    }

    // ---- AC-004: the list is swept in reverse ------------------------------

    #[test]
    fn a_stale_allowance_is_a_problem() {
        let page = page_with("append-conditions.md", "```rust\n```\n");
        let allowances = [("docs/gone.md", "12", "the example needs a running database")];

        let found = walk(&[page], &allowances);

        assert_eq!(found.len(), 1, "got: {found:?}");
        assert!(found[0].contains("docs/gone.md"), "got: {}", found[0]);
        assert!(found[0].contains("no `ignore` fence"), "got: {}", found[0]);
    }

    /// EC-005, the drift most likely to happen: the fence is still there and no
    /// longer opts out, so the permission is standing and unused.
    #[test]
    fn an_allowance_for_a_fence_that_no_longer_opts_out_is_a_problem() {
        let page = page_with("append-conditions.md", "```rust\n```\n");
        let allowances = [(
            "docs/append-conditions.md",
            "1",
            "the example needs a running database",
        )];

        let found = walk(&[page], &allowances);

        assert_eq!(found.len(), 1, "got: {found:?}");
        assert!(
            found[0].contains("no longer opts out"),
            "the sweep must say the fence stopped opting out, not merely that the \
             entry is stale, got: {}",
            found[0]
        );
    }

    /// EC-003. An inert entry looks like a granted permission to the next reader.
    #[test]
    fn a_malformed_allowance_is_a_problem() {
        for (entry, expected) in [
            (
                ("standards/rust/80-the-gate.md", "1", "why"),
                "not under docs",
            ),
            (("docs/append-conditions.md", "", "why"), "line-or-anchor"),
        ] {
            let page = page_with("append-conditions.md", "```rust,ignore\n```\n");
            let found = walk(&[page], &[entry]);

            assert!(
                found.iter().any(|problem| problem.contains(expected)),
                "{entry:?} should report `{expected}`, got: {found:?}"
            );
        }
    }

    /// EC-004. Otherwise deleting one leaves the other silently authorising the
    /// fence, and the sweep reports neither as stale.
    #[test]
    fn a_duplicate_allowance_is_a_problem() {
        let page = page_with("append-conditions.md", "```rust,ignore\n```\n");
        let allowances = [
            ("docs/append-conditions.md", "1", "the first reason"),
            ("docs/append-conditions.md", "1", "the second reason"),
        ];

        let found = walk(&[page], &allowances);

        assert_eq!(found.len(), 1, "got: {found:?}");
        assert!(
            found[0].contains("two") && found[0].contains("IGNORE_ALLOWANCES"),
            "got: {}",
            found[0]
        );
    }

    // ---- AC-005: no false positive, and the right location ----------------

    /// The corpus quotes fenced material inside a four-backtick block — the
    /// design's own fixture page is written that way. Read as examples, those
    /// inner fences would be compiled as Rust, and flagging them teaches
    /// contributors that the checker cries wolf.
    #[test]
    fn four_backtick_fences_are_not_examples() {
        let text = "````markdown\n```\nfn main() {}\n```\n````\n";

        assert!(fences(text).iter().all(|fence| fence.info != "rust"));
        assert!(
            walk(&[page_with("append-conditions.md", text)], &[]).is_empty(),
            "an untagged fence inside a quoted block is quoted material, not an example"
        );
    }

    /// The other half of that rule, and the one the precedent's unconditional
    /// step-over got wrong. Four backticks are a fence like any other: `rustdoc
    /// --test` collects `` ````ignore `` and reports it *ignored*, and compiles
    /// an untagged four-backtick block. Stepping over both because of their
    /// delimiter is an opt-out with the gate green.
    #[test]
    fn a_four_backtick_fence_carrying_its_own_info_string_is_an_example() {
        for (text, expected) in [
            ("````ignore\nfn f() {}\n````\n", "IGNORE_ALLOWANCES"),
            ("````\nfn main() {}\n````\n", "an untagged fence"),
        ] {
            let found = walk(&[page_with("append-conditions.md", text)], &[]);

            assert_eq!(found.len(), 1, "`{text}` got: {found:?}");
            assert!(
                found[0].starts_with("docs/append-conditions.md:1 — ")
                    && found[0].contains(expected),
                "got: {}",
                found[0]
            );
        }
    }

    /// A quoted block that is never terminated is reported, rather than
    /// disabling the walk for everything below it: the precedent toggled a flag
    /// nothing ever checked at the end, so one stray line was a page-wide
    /// opt-out that printed nothing.
    #[test]
    fn a_stray_four_backtick_opener_does_not_swallow_the_rest_of_the_page() {
        let page = page_with(
            "append-conditions.md",
            "````markdown\n\nquoted prose\n\n```rust,ignore\n```\n",
        );

        let found = walk(&[page], &[]);

        assert_eq!(found.len(), 1, "got: {found:?}");
        assert!(
            found[0].starts_with("docs/append-conditions.md:1 — ")
                && found[0].contains("never closed"),
            "the unterminated opener is what to report, got: {}",
            found[0]
        );
    }

    /// `CommonMark` allows three spaces of indentation and rustdoc compiles what
    /// is inside them — measured. A `strip_prefix("```")` walk lets indentation
    /// past the untagged rule.
    #[test]
    fn a_fence_indented_up_to_three_spaces_is_still_a_fence() {
        for indent in ["", " ", "  ", "   "] {
            let text = format!("{indent}```\nfn main() {{}}\n{indent}```\n");
            let found = walk(&[page_with("append-conditions.md", &text)], &[]);

            assert_eq!(found.len(), 1, "indent `{indent}` got: {found:?}");
            assert!(found[0].contains("an untagged fence"), "got: {}", found[0]);
        }
    }

    /// `~~~` is the other delimiter `CommonMark` defines, and the same probe
    /// collects `` ~~~ignore `` and reports it ignored. A backtick-only parser is
    /// a tilde-shaped way out of every rule above.
    #[test]
    fn a_tilde_fence_is_a_fence() {
        let found = walk(
            &[page_with(
                "append-conditions.md",
                "~~~ignore\nfn f() {}\n~~~\n",
            )],
            &[],
        );

        assert_eq!(found.len(), 1, "got: {found:?}");
        assert!(found[0].contains("IGNORE_ALLOWANCES"), "got: {}", found[0]);
    }

    #[test]
    fn a_problem_names_the_page_and_the_fence_line() {
        let page = page_with("adapters/sqlite.md", "one\ntwo\nthree\n```\n```\n");

        let found = walk(&[page], &[]);

        assert_eq!(found.len(), 1, "got: {found:?}");
        let (location, _) = found[0].split_once(" — ").unwrap();
        assert_eq!(
            location, "docs/adapters/sqlite.md:4",
            "the location is the page and the fence's line, never the harness"
        );
        assert!(
            location.chars().count() <= 48,
            "the location prefix must fit the 48-column budget"
        );
    }

    /// EC-002. The precedent parser drops an unpaired opener on the floor, so
    /// its info string is never examined — an opt-out route inherited by
    /// copying. Fixed in the copy, and not in `lint_constitution`.
    #[test]
    fn an_unterminated_fence_is_a_problem() {
        let page = page_with("append-conditions.md", "a\n\n```rust,ignore\nfn f() {}\n");

        let found = walk(&[page], &[]);

        assert!(
            found.iter().any(|problem| problem.contains("never closed")),
            "an unpaired opener must be reported rather than dropped, got: {found:?}"
        );
    }

    // ---- AC-006: all of them, in source order, never truncated -------------

    #[test]
    fn problems_are_reported_in_source_order() {
        let first = page_with("adapters/sqlite.md", "```rust,ignore\n```\n\n```\n```\n");
        let second = page_with("append-conditions.md", "```rust,zzz\n```\n");

        let found = walk(&[first, second], &[]);

        assert_eq!(found.len(), 3, "got: {found:?}");
        assert!(found[0].starts_with("docs/adapters/sqlite.md:1 — "));
        assert!(found[1].starts_with("docs/adapters/sqlite.md:4 — "));
        assert!(found[2].starts_with("docs/append-conditions.md:1 — "));
    }

    #[test]
    fn every_problem_is_reported_not_the_first() {
        let mut text = String::new();
        for _ in 0..40 {
            text.push_str("```\n```\n");
        }
        let found = walk(&[page_with("append-conditions.md", &text)], &[]);

        assert_eq!(found.len(), 40, "forty problems print as forty lines");
        assert!(
            found.iter().all(|problem| !problem.contains("more")),
            "no `… and N more`: {found:?}"
        );
    }

    // ---- AC-007: the green surface, and the record it owes -----------------

    #[test]
    fn the_module_docs_state_the_text_limit_and_the_divergence() {
        let source = production_source();
        let docs_end = source
            .find("\nuse std::fs;")
            .unwrap_or_else(|| panic!("this module's docs do not end where they used to"));
        let first_check = source
            .find("fn check_fences")
            .unwrap_or_else(|| panic!("this module declares no fence walk"));

        for sentence in ["tagged `text`", "narrows that hole", "<!-- ignore:"] {
            let at = source
                .find(sentence)
                .unwrap_or_else(|| panic!("the module docs must state `{sentence}`"));
            assert!(
                at < docs_end && at < first_check,
                "`{sentence}` must be in the module docs, before the first check"
            );
        }
    }

    /// The claim the docs make has to be the coverage the walk has. They say how
    /// an info string is read, that recognised and permitted are two sets, and
    /// which hole the walk still leaves — the indented block rustdoc compiles
    /// and this walk cannot see.
    #[test]
    fn the_module_docs_state_how_a_fence_is_read() {
        let source = production_source();
        let first_check = source
            .find("fn check_fences")
            .unwrap_or_else(|| panic!("this module declares no fence walk"));

        for sentence in [
            "split on `,`, a space",
            "tag rustdoc itself defines",
            "indented by four spaces or more",
        ] {
            let at = source
                .find(sentence)
                .unwrap_or_else(|| panic!("the module docs must state `{sentence}`"));
            assert!(
                at < first_check,
                "`{sentence}` must precede the first check"
            );
        }
    }

    // ======================================================================
    // hidden-content-resolution
    // ======================================================================

    /// The fixture page from `_design.md` `## The doctest`, verbatim — including
    /// the corrected `happenstance_core::MemoryEventStore` spelling, because
    /// `memory` is a private module and the type is re-exported.
    ///
    /// It is test material and never a file under `docs/`: the wrapped form
    /// below would fail `cargo xtask ci` forever if it were committed, which is
    /// exactly why it is the named wrong implementation.
    const FIXTURE_PAGE: &str = "\
# Appending under a condition

*<!-- answered-need: reserved for HS-P0021 -->*

An append condition is checked against the same boundary the query read, so a
writer that saw a consistent view cannot be overtaken between reading and
appending (ES-40).

```rust
use happenstance_core::MemoryEventStore;

let store = MemoryEventStore::new();
assert_eq!(store.len(), 0);
```

## Per-adapter notes

### happenstance-postgres

Positions are assigned outside the transaction.

### happenstance-sqlite

One writer at a time.
";

    /// The same page with a disclosure wrapper around its scope band.
    ///
    /// Derived from the clean form rather than hand-copied beside it, so the
    /// pair cannot drift into testing two different pages.
    fn wrapped_fixture() -> String {
        format!(
            "{}\n</details>\n",
            FIXTURE_PAGE.replace(
                "## Per-adapter notes",
                "<details>\n<summary>Per-adapter notes</summary>\n\n## Per-adapter notes",
            )
        )
    }

    // ---- AC-001: every marker is a problem, from inside the same walk ------

    #[test]
    fn every_hidden_marker_is_reported_once_per_occurrence() {
        for token in HIDDEN_MARKERS {
            // A closing fence, because ```admonish is itself a fence opener and
            // an unpaired one is separately a problem — the two checks are
            // independent and both are right.
            let text = format!("a claim\n{token}\n```\n");
            let markers: Vec<String> = walk(&[page_with("append-conditions.md", &text)], &[])
                .into_iter()
                .filter(|problem| problem.contains("is a hidden panel"))
                .collect();

            assert_eq!(markers.len(), 1, "`{token}` got: {markers:?}");
            assert!(
                markers[0].starts_with("docs/append-conditions.md:2 — "),
                "the problem must name the page and the line, got: {}",
                markers[0]
            );
        }
    }

    /// EC-004: two markers on one line are two problems, so the count in the
    /// `bail!` is the number of things to fix.
    #[test]
    fn two_markers_on_one_line_are_two_problems() {
        let found = walk(
            &[page_with(
                "append-conditions.md",
                "<details><summary>notes</summary>\n",
            )],
            &[],
        );
        assert_eq!(found.len(), 2, "got: {found:?}");
    }

    /// The marker problem joins the walk's existing accumulator, so a
    /// contributor's other problems on the same page are reported in the same
    /// run and in source order.
    #[test]
    fn marker_and_fence_problems_arrive_in_one_list_in_source_order() {
        let text = "```\nfn main() {}\n```\n\n<details>\n";
        let found = walk(&[page_with("append-conditions.md", text)], &[]);

        assert_eq!(found.len(), 2, "got: {found:?}");
        assert!(found[0].starts_with("docs/append-conditions.md:1 — "));
        assert!(found[1].starts_with("docs/append-conditions.md:5 — "));
    }

    // ---- AC-002: the named wrong implementation, both halves ---------------

    #[test]
    fn the_wrapped_fixture_page_fails_by_file_and_line() {
        let found = walk(
            &[page_with("append-conditions.md", &wrapped_fixture())],
            &[],
        );

        assert_eq!(
            found.len(),
            2,
            "the `<details` and the `<summary` lines, and nothing else: {found:?}"
        );
        assert!(found[0].contains("`<details`"), "got: {}", found[0]);
        assert!(found[1].contains("`<summary`"), "got: {}", found[1]);

        let lines: Vec<&str> = found
            .iter()
            .map(|problem| problem.split_once(" — ").unwrap().0)
            .collect();
        assert_ne!(lines[0], lines[1], "each occurrence names its own line");
    }

    /// Without the clean half the rule cannot be distinguished from one that
    /// rejects every page.
    #[test]
    fn the_same_fixture_page_without_its_wrapper_is_clean() {
        assert!(
            walk(&[page_with("append-conditions.md", FIXTURE_PAGE)], &[]).is_empty(),
            "the fixture page is the artifact the gate compiles; it must pass"
        );
    }

    // ---- AC-003: every spelling a renderer accepts, and no exemption -------

    /// HTML tag names are case-insensitive, so a case-sensitive `contains`
    /// accepts `<Details>` — the same class of hole as the elaborate spellings
    /// of `ignore` this repository has already been bitten by.
    #[test]
    fn a_hidden_marker_is_matched_whatever_its_case() {
        for spelling in ["<details>", "<Details>", "<DETAILS open>", "{{#TABS}}"] {
            let found = walk(&[page_with("append-conditions.md", spelling)], &[]);
            assert_eq!(found.len(), 1, "`{spelling}` got: {found:?}");
        }
    }

    /// EC-003. A fence-aware scan is the most plausible refinement available and
    /// it is refused: a marker quoted in a fence still renders as a page telling
    /// a reader to fold something, and there is no allowance path to exempt it.
    #[test]
    fn a_marker_inside_a_fence_is_still_reported() {
        let found = walk(
            &[page_with(
                "append-conditions.md",
                "```text\n<details>\n```\n",
            )],
            &[],
        );
        assert_eq!(found.len(), 1, "got: {found:?}");
        assert!(found[0].starts_with("docs/append-conditions.md:2 — "));
    }

    /// The index is scanned like every other file, and it is clean today.
    #[test]
    fn the_real_index_carries_no_hidden_marker() {
        let index = read(INDEX);
        let mut found = Vec::new();
        check_hidden_markers(&page_with("README.md", &index), &mut found);

        assert!(
            found.is_empty(),
            "{INDEX} carries a hidden marker: {found:?}"
        );
    }

    // ---- AC-004: the set is pinned, and the failure names which token moved -

    /// RS-81-5. `_design.md` D2 was signed off by a human on 2026-08-17 with no
    /// conditions; shrinking this set re-opens DT-7 and requires a new design
    /// record, not an edit to a `const`.
    #[test]
    fn the_hidden_marker_set_is_pinned_to_the_design() {
        const PINNED: &[&str] = &[
            "<details",
            "<summary",
            "{{#tabs",
            "{{#tab ",
            "{{#endtabs",
            "```admonish",
            "<!-- tab",
        ];

        for token in HIDDEN_MARKERS {
            assert!(
                PINNED.contains(token),
                "`{token}` is in HIDDEN_MARKERS and not in the pin; widening the set is \
                 still a design change"
            );
        }
        for token in PINNED {
            assert!(
                HIDDEN_MARKERS.contains(token),
                "`{token}` left HIDDEN_MARKERS; shrinking it re-opens DT-7 and requires a \
                 new design record, not an edit"
            );
        }
        assert_eq!(HIDDEN_MARKERS.len(), PINNED.len());
    }

    /// What makes the case-folding in `check_hidden_markers` correct rather than
    /// accidentally correct.
    #[test]
    fn every_hidden_marker_is_already_lowercase() {
        for token in HIDDEN_MARKERS {
            assert_eq!(
                *token,
                token.to_ascii_lowercase(),
                "a token that is not lowercase can never match a lowered line"
            );
        }
    }

    // ---- AC-005: the composed line, unbounded, ordered, and no new chrome ---

    /// The `_design.md` `## States` message form, verbatim.
    #[test]
    fn a_marker_problem_is_the_composed_line_the_design_specifies() {
        let found = walk(
            &[page_with("adapters/sqlite.md", "a\n\n\n<details>\n")],
            &[],
        );

        assert_eq!(found.len(), 1, "got: {found:?}");
        assert_eq!(
            found[0],
            "docs/adapters/sqlite.md:4 — `<details` is a hidden panel; DT-7 forbids it in docs"
        );
    }

    #[test]
    fn forty_markers_print_as_forty_lines() {
        let mut text = String::new();
        for _ in 0..40 {
            text.push_str("<details>\n");
        }
        let found = walk(&[page_with("append-conditions.md", &text)], &[]);

        assert_eq!(found.len(), 40);
        assert!(found.iter().all(|problem| !problem.contains("more")));
    }

    #[test]
    fn marker_problems_are_ordered_by_page_then_line() {
        let found = walk(
            &[
                page_with("adapters/sqlite.md", "\n<details>\n"),
                page_with("append-conditions.md", "<summary>\n"),
            ],
            &[],
        );

        assert_eq!(found.len(), 2, "got: {found:?}");
        assert!(found[0].starts_with("docs/adapters/sqlite.md:2 — "));
        assert!(found[1].starts_with("docs/append-conditions.md:1 — "));
    }

    /// This story adds no step, banner, subcommand or spinner of its own: the
    /// composition root does not mention it at all.
    #[test]
    fn the_marker_scan_adds_no_step_or_banner_of_its_own() {
        let main = read(ROOT_MODULE);

        for spelling in ["HIDDEN_MARKERS", "hidden panel", "hidden marker"] {
            assert!(
                !main.contains(spelling),
                "`{spelling}` in the composition root means a second step or banner"
            );
        }
    }

    // ---- AC-006: no allowance path, and no hook for one --------------------

    /// The executable form of "there is no allowance path": no input, however it
    /// is dressed, makes a marker pass.
    #[test]
    fn no_input_makes_a_hidden_marker_pass() {
        for text in [
            "<details>",
            "<!-- allow: this fold is deliberate -->\n<details>",
            "IGNORE_ALLOWANCES names this page\n<details>",
            "```text\n<details>\n```",
            "````markdown\n<details>\n````",
            "prose before <details> and prose after",
        ] {
            let found = walk(&[page_with("append-conditions.md", text)], &[]);
            assert!(
                !found.is_empty(),
                "`{text}` must still be a problem; the absence of an allowance path is \
                 the decision"
            );
        }
    }

    /// And the scan itself carries no escape hatch to reach for.
    #[test]
    fn the_marker_scan_has_no_allowance_environment_or_cfg_hook() {
        let source = production_source();
        let start = source
            .find("fn check_hidden_markers")
            .unwrap_or_else(|| panic!("this module declares no marker scan"));
        let body = &source[start..];
        let end = body.find("\n}\n").map_or(body.len(), |at| at + 3);
        let body = &body[..end];

        for hook in ["IGNORE_ALLOWANCES", "env::var", "cfg(", "feature ="] {
            assert!(
                !body.contains(hook),
                "`{hook}` in the marker scan would be a way to permit a hidden panel"
            );
        }
    }

    /// The three limits this check creates, stated first, in the module's docs.
    #[test]
    fn the_module_docs_state_the_marker_scans_limits() {
        let source = production_source();
        let first_check = source
            .find("fn check_fences")
            .unwrap_or_else(|| panic!("this module declares no fence walk"));

        for sentence in [
            "a spelling this set does not carry",
            "outside the pinned tree",
            "reads source",
        ] {
            let at = source
                .find(sentence)
                .unwrap_or_else(|| panic!("the module docs must state `{sentence}`"));
            assert!(
                at < first_check,
                "`{sentence}` must precede the first check"
            );
        }
    }

    /// The green surface does not change: this story adds no per-page or
    /// per-fence chatter, and the tree as it stands still reports one line.
    #[test]
    fn a_clean_tree_still_reports_one_line_after_the_fence_walk() {
        let clean = page_with(
            "append-conditions.md",
            "a claim\n\n```rust\nfn main() {}\n```\n",
        );
        assert!(walk(&[clean], &[]).is_empty());
        assert_eq!(
            summary(&[page("append-conditions.md")]),
            "  1 pages, all consistent"
        );
    }
}
