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
//! One of the tree's six enumerated limits is a property of *this* walk and is
//! stated here. The other four that belong to the machine — what a compiled
//! fence does and does not establish, what `RUSTDOCFLAGS` reaches, and what a
//! failure actually names — are properties of the compile mechanism and are
//! stated where they hold, in `xtask/src/narrative.rs`. A plain path rather
//! than an intra-doc link, because that file is compiled into the **lib**
//! target and this module into the bin target: the two never link, and a link
//! across the boundary resolves to nothing while failing nothing.
//!
//! * **Registration proves that a page is compiled. It does not prove that the
//!   page is correct.** A page this module reports as registered is a page
//!   rustdoc will hand to the compiler. Whether its prose still describes what
//!   its fences do is a question nothing in this repository asks; adjacency is
//!   what makes it reviewable by a human, and nothing makes it mechanical. What
//!   a compiled fence does and does not establish is stated in full in
//!   `xtask/src/narrative.rs`, not restated here.
//! * **A Rust example deliberately tagged `text` is neither compiled nor
//!   flagged.** The fence walk rejects an untagged fence and refuses any info
//!   string it does not enumerate — in every spelling rustdoc accepts, which is
//!   what the section below is about — and the allowance list makes every
//!   `ignore` a thing a human approved with a reason attached. But `text` is a
//!   legitimate tag for prose, and a block whose info string carries no rustdoc
//!   tag at all *is* prose as far as the compiler is concerned, so an author who
//!   wants a Rust block the compiler never sees can still have one by calling it
//!   something else. This list narrows that hole and does not close it. The
//!   hole is executed rather than asserted: `docs/text-fences.md` is a retained
//!   fixture whose `text` fence is deliberately false about the library, walked
//!   by this module on every gate run and reported by nothing, and a test pins
//!   that silence so the day it closes is the day this bullet fails a build.
//!   The only other instrument is the reviewer who reads the tag in the diff.
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
//! * **A citation that resolves says nothing about the sentence above it.**
//!   [`check_citations`] answers whether `spec/SPECIFICATION.md` declares the
//!   clause a page names. Whether the claim the page makes is true, and whether
//!   the page *defers* to that clause rather than quietly restating it, are the
//!   reviewer's half of BR-09 and nothing here can see either. Provenance is
//!   what rots and what is checked; comprehension is neither.
//! * **A clause-shaped token inside a fence is not told apart from prose.** The
//!   whole page is scanned, fences included, deliberately: a clause id in a
//!   comment above an example is still a claim, and excluding fenced regions
//!   would put a hiding place inside the one region that *is* the checked
//!   artifact. The cost is stated rather than parsed around — a clause id that
//!   is genuinely data inside an example is reported, and the answer is to
//!   spell it outside the fence. There is no allowance list for it, and adding
//!   one is a petition in its own change with its own falsification.
//! * **A near-miss in a family the specification declares nowhere is silent.**
//!   `ES-4O` inside the declared `ES-` family is reported as unreadable; `QQ-4O`
//!   is not, because nothing distinguishes it from prose that happens to carry
//!   two capitals and a hyphen. The check is scoped to families the document
//!   itself declares, and that scoping is what keeps it from firing on correct
//!   sentences.
//! * **A doubly-declared clause id collapses upstream.** `clause_ids` returns a
//!   set, so a document declaring one id twice resolves exactly as one
//!   declaring it once, and nothing in this repository looks for the duplicate.
//! * **An anchor can survive while the reasoning around it is rewritten.**
//!   [`FROZEN_DOC_MUSTS`] holds one verbatim phrase per pinned clause and
//!   checks that the phrase is still there. A rewrite that keeps the sentence
//!   and guts the paragraph explaining it is a re-discharge this check cannot
//!   detect. `.kb/governance/rewrite-the-referent-never-the-reasoning.md` is
//!   the human rule that covers it, and it is a human rule because no string
//!   match can be one.
//! * **A clause wording its documentation obligation outside
//!   [`DOCUMENTATION_OBLIGATIONS`] is invisible to the derived scan.** The
//!   phrase list is six entries and deliberately short. A clause that says "the
//!   reader has to be told" instead of "MUST document" is never a candidate, so
//!   the pin cannot notice it going unclassified — which is the one direction
//!   the derived-versus-hand-written comparison cannot protect.
//! * **The pin proves a discharge is *present*, never that it is *adequate*.**
//!   A site carrying its anchor is a site whose load-bearing sentence is still
//!   there. Whether the paragraph around it still teaches an adapter author
//!   what the clause needs them to know is a review question, and this step
//!   asks none.
//! * **The harness is matched as text, so reformatting it can break this check
//!   without breaking the compile.** Splitting an `include_str!` across lines, or
//!   writing a `mod` line that does not start with `mod ` after trimming, makes a
//!   registered page look unregistered. It is the coupling
//!   `lint_constitution::check_harness` already lives with, recorded here rather
//!   than defended against with a parser.
//! * **This step says nothing about whether any page teaches anybody
//!   anything.** A green run means the tree is where it is pinned, holds pages,
//!   and that every page is offered to the compiler. That is the whole of it;
//!   comprehension is HS-P0024's friction log, and no run of this step
//!   substitutes for it.
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

use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use anyhow::{Context, Result, bail};

use crate::spec_trace::{clause_ids, workspace_root};

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

/// The specification the pin resolves against and scans for candidates.
///
/// A second spelling of a path `spec_trace::SPEC` already holds, and it is a
/// second spelling on purpose rather than by accident: that constant is private
/// to the module that owns the parser, this module's one permitted edit to that
/// file is a deletion, and widening its visibility to save one string would be
/// this story reaching into a neighbour's file for a convenience. The two agree
/// or the run fails loudly — `clause_ids` resolves through its own constant
/// immediately before this read, so a divergence is a missing-file error on the
/// next line rather than a silent scan of nothing.
const SPECIFICATION: &str = "spec/SPECIFICATION.md";

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
            let text = read_page(root, &rel)?;
            out.push(Page::new(&rel, &text));
        }
    }
    Ok(())
}

/// One page's text, read once, at enumeration.
///
/// Hard error rather than a skipped page, and the call site `?`-propagates it:
/// a scanner that silently steps over what it cannot read reports green over
/// exactly the file it failed to inspect (RS-81-2,
/// `standards/rust/81-checks-that-cannot-be-types.md:95`). The `let Ok(text) =
/// … else { continue }` that would shorten the page list instead is the named
/// wrong implementation.
///
/// # Errors
///
/// When the page cannot be read, naming the page by its tree-relative path.
fn read_page(root: &Path, rel: &str) -> Result<String> {
    fs::read_to_string(root.join(TREE).join(rel)).with_context(|| format!("reading {TREE}/{rel}"))
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

/// The specification's clause ids, resolved once per run.
///
/// Read once and passed by reference to every check that needs it. Two calls
/// would read and parse a 9,070-line document twice inside one step for no new
/// information, which is the contract
/// [`crate::spec_trace::clause_ids`] states on its consumers.
struct Clauses {
    /// Every clause id `spec/SPECIFICATION.md` declares.
    ids: BTreeSet<String>,
    /// The families those ids sit in, as their two-letter prefixes.
    ///
    /// **Derived from the set, never listed.** A literal list of the six
    /// families here would be the fourth, and `SECTIONS`' own doc comment
    /// (`xtask/src/spec_trace.rs:107-120`) already says why the fourth is the
    /// one that can half-land: a seventh family added in that one place would
    /// have every id of it reported here as an undeclared family while
    /// `spec-trace` counted it happily.
    families: BTreeSet<String>,
}

impl Clauses {
    fn new(ids: BTreeSet<String>) -> Self {
        let families = ids
            .iter()
            .filter_map(|id| id.split_once('-'))
            .map(|(letters, _)| letters.to_owned())
            .collect();
        Self { ids, families }
    }

    /// The declared families as a reader spells them: `CF-, ES-, …`.
    ///
    /// In the message rather than in a constant, so an author who meant a
    /// constitution rule sees what the specification actually declares instead
    /// of guessing — and so the sentence cannot go stale when a seventh lands.
    fn declared(&self) -> String {
        self.families
            .iter()
            .map(|letters| format!("{letters}-"))
            .collect::<Vec<_>>()
            .join(", ")
    }
}

/// A token on a page shaped like a clause citation.
#[derive(Debug, PartialEq, Eq)]
struct Cited<'a> {
    /// The token as written: `ES-40`, `ES-4O`, `ES-`, `ES -40`.
    text: &'a str,
    /// The family's two letters, hyphen excluded: `ES`.
    letters: &'a str,
    /// The clause id, when the token parses as one at all.
    id: Option<&'a str>,
}

/// Every citation-shaped token on one line, in source order.
///
/// The scan, split from the decision so both boundaries are assertable against
/// a `&str` with no id set, no page and no filesystem.
///
/// **Two boundaries, and they are the load-bearing half.** The character before
/// the two capitals must not be ASCII-alphanumeric, which is what keeps
/// `ADR-0001` from being read as a dangling `DR-0001`; and the character after
/// the digits must not be a hyphen, which is what keeps `RS-81-1` — this
/// repository's own constitution rule ids, which a page may legitimately cite —
/// from being read as a dangling `RS-81`. A checker missing either fires on
/// prose that is **correct**, which is worse than missing a defect: it teaches
/// contributors to delete true sentences.
///
/// `HS-S0142` fails the shape outright, because a letter and not a digit
/// follows the hyphen. `ES-` and `ES-4O` do not parse but are still returned,
/// with `id: None`, because the caller — not this scan — decides whether the
/// family is one the specification declares.
fn clause_citations(line: &str) -> Vec<Cited<'_>> {
    let bytes = line.as_bytes();
    let mut out = Vec::new();

    for start in 0..bytes.len() {
        if !bytes[start].is_ascii_uppercase()
            || !bytes.get(start + 1).is_some_and(u8::is_ascii_uppercase)
        {
            continue;
        }
        // The leading boundary. `ADR-0001` is the token it exists to protect:
        // without it the scan reads a dangling `DR-0001` out of the middle of
        // one, and a page citing an ADR correctly fails the gate.
        if line[..start]
            .chars()
            .next_back()
            .is_some_and(|character| character.is_ascii_alphanumeric())
        {
            continue;
        }

        // A hyphen, or one space and then a hyphen. The spaced form never
        // parses — `ES -40` is the typo rather than a spelling — and it is
        // recognised only far enough to say so.
        let (hyphen, spaced) = match bytes.get(start + 2) {
            Some(b'-') => (start + 2, false),
            Some(b' ') if bytes.get(start + 3) == Some(&b'-') => (start + 3, true),
            _ => continue,
        };

        let letters = &line[start..start + 2];
        let digits_end = hyphen
            + 1
            + bytes[hyphen + 1..]
                .iter()
                .take_while(|byte| byte.is_ascii_digit())
                .count();
        let has_digits = digits_end > hyphen + 1;
        let after = bytes.get(digits_end).copied();

        if spaced {
            // `ES --` is punctuation rather than a mis-typed citation, so the
            // spaced form is reported only when digits actually follow.
            if has_digits {
                out.push(Cited {
                    text: &line[start..digits_end],
                    letters,
                    id: None,
                });
            }
            continue;
        }

        if !has_digits {
            // `HS-S0142` fails the shape outright: a letter, not a digit,
            // follows the hyphen. `ES-` alone does not parse and is reported.
            if after.is_none_or(|byte| !byte.is_ascii_alphanumeric()) {
                out.push(Cited {
                    text: &line[start..digits_end],
                    letters,
                    id: None,
                });
            }
            continue;
        }

        match after {
            // The trailing boundary. `RS-81-1` — this repository's own
            // constitution rule ids, which a page may legitimately cite — is
            // the token it exists to protect.
            Some(b'-') => {}
            // `ES-4O`, a letter for a zero. The whole word is carried so the
            // message quotes what the author actually wrote.
            Some(byte) if byte.is_ascii_alphanumeric() => {
                let word_end = digits_end
                    + bytes[digits_end..]
                        .iter()
                        .take_while(|byte| byte.is_ascii_alphanumeric())
                        .count();
                out.push(Cited {
                    text: &line[start..word_end],
                    letters,
                    id: None,
                });
            }
            _ => {
                let id = &line[start..digits_end];
                out.push(Cited {
                    text: id,
                    letters,
                    id: Some(id),
                });
            }
        }
    }
    out
}

/// Every clause a page cites, resolved against the specification.
///
/// Three problem forms, worded apart on purpose: a reviewer scanning a log
/// should be able to tell "you meant a different family", "I could not read
/// this" and "this clause does not exist" apart without opening the page.
///
/// The whole page is scanned, fenced blocks included — see this module's limits.
fn check_citations(page: &Page, clauses: &Clauses, found: &mut Vec<Found>) {
    for (index, line) in page.text.lines().enumerate() {
        // One problem per occurrence, never per id: each occurrence is a line a
        // contributor has to edit, and a list that hides four of five is the
        // six-review-cycle failure `_decomposition.md:218-219` names.
        for cited in clause_citations(line) {
            let at = index + 1;

            if !clauses.families.contains(cited.letters) {
                // Well formed, it is a problem naming the families that do
                // exist, so an author who meant a constitution rule spells it
                // in full. Malformed, it is not distinguishable from prose —
                // the limit this module states rather than a silent skip.
                if let Some(id) = cited.id {
                    found.push((
                        at,
                        format!(
                            "cites `{id}`, and SPECIFICATION.md has no `{}-` family; the \
                             families it declares are {}",
                            cited.letters,
                            clauses.declared()
                        ),
                    ));
                }
                continue;
            }

            match cited.id {
                // Resolved. Silent, deliberately: a check that reports every
                // resolving citation makes the one line that matters harder to
                // find.
                Some(id) if clauses.ids.contains(id) => {}
                Some(id) => found.push((
                    at,
                    format!("cites `{id}`, which SPECIFICATION.md does not define"),
                )),
                None => found.push((
                    at,
                    format!(
                        "`{}` looks like a clause citation and does not parse; a citation \
                         the checker cannot read is one nothing checks",
                        cited.text
                    ),
                )),
            }
        }
    }
}

/// Every problem one page carries, composed and in line order.
fn check_page(
    page: &Page,
    allowances: &[(&str, &str, &str)],
    usage: &mut [Usage],
    clauses: &Clauses,
    problems: &mut Vec<String>,
) {
    let mut found: Vec<Found> = Vec::new();
    check_fences(page, allowances, usage, &mut found);
    check_hidden_markers(page, &mut found);
    check_citations(page, clauses, &mut found);
    // Stable, so two problems on one line keep the order they were found in and
    // a marker interleaves with a fence problem by line rather than by check.
    found.sort_by_key(|(line, _)| *line);
    for (line, message) in found {
        problems.push(format!("{}:{line} — {message}", page.path));
    }
}

/// The wordings a clause uses to place an obligation on documentation.
///
/// The derived half of the pin's third assertion: a line scan of
/// `spec/SPECIFICATION.md` for these phrases, over clause bodies whose
/// whitespace has been collapsed first, so a phrase broken across a soft wrap is
/// still found — `PS-34`'s is, and a line-keyed scan misses it.
///
/// **Short on purpose, and its limit is stated here rather than discovered.** A
/// clause wording its documentation obligation in none of these — "the reader
/// has to be told", "this belongs in the doc comment" — is invisible to the
/// scan, so the pin cannot notice it going unclassified. A long list of
/// near-synonyms would buy recall nobody can verify and would hide exactly this
/// sentence.
const DOCUMENTATION_OBLIGATIONS: &[&str] = &[
    "MUST document",
    "MUST state",
    "MUST say",
    "documentation MUST",
    "MUST be documented",
    "belongs in the port's documentation",
];

/// What the pin decided about one candidate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Disposition {
    /// Discharged in the contract's own documentation, at `site`, by `anchor`.
    ///
    /// The anchor is a **verbatim phrase**, never the clause id, and that is
    /// forced rather than preferred: as `main` stands, not one discharge site
    /// names its clause id, so an id-as-anchor pin fails on every entry the day
    /// it lands — and the repair a contributor reaches for is widening the match
    /// until it passes, which empties the check. [`guard_pin`] refuses it.
    Pinned {
        /// Repo-relative, `/`-separated.
        site: &'static str,
        /// The load-bearing sentence of the discharge, not a nearby convenience.
        anchor: &'static str,
    },
    /// Not pinned, with the reason the next reader meets beside the entry.
    Excluded {
        /// One line. Why the derivation rule does not reach this candidate.
        reason: &'static str,
    },
}

/// One candidate documentation obligation, disposed of.
#[derive(Debug, Clone, Copy)]
struct DocumentationMust {
    /// The clause id, resolved through [`crate::spec_trace::clause_ids`].
    clause: &'static str,
    /// Pinned with a site and an anchor, or excluded with a reason.
    disposition: Disposition,
}

/// Every documentation obligation `spec/SPECIFICATION.md` words, disposed of.
///
/// **The derivation rule, where the next reader meets it.** A candidate is
/// **pinned** when (a) its clause is `[FROZEN]`, (b) its obligation falls on
/// **the contract's own documentation** — not on an adapter's, not on a
/// fixture's, and not on this specification's own prose — **and** (c) the
/// discharge exists today. Everything the scan finds and the rule does not
/// reach is **excluded**, with its reason on the entry rather than in a
/// paragraph somewhere else.
///
/// **(c) is a condition and not a footnote**, because pinning an undischarged
/// obligation lands the gate red on a tree nobody broke, and the only repair is
/// editing a `happenstance-core` doc comment — which is HS-P0023's under
/// `.kb/governance/rewrite-the-referent-never-the-reasoning.md`, not this pin's.
/// `ES-26`, `PS-31` and `PS-36` fail **only** (c): each is `[FROZEN]`, each
/// puts its obligation on the contract's own documentation, and nothing
/// discharges it. They are the follow-up this pin hands forward, named here so
/// the finding is where the reader meets the array rather than only in the
/// re-derivation record — a rule stated above an array it does not explain is
/// the defect this pin exists to prevent, one level up.
///
/// The set is **re-derived** against the document as it stands rather than
/// copied from either of the two statements that disagree about it, and the
/// re-derivation is written down in
/// `.bklg/docs-that-teach/checked-documentation-surface/frozen-documentation-must-pin/`.
/// No count is written anywhere here: `UNCLAIMED_PENDING_ADR` already refused
/// that trade (`xtask/src/spec_trace.rs:1975-1978`), because a comment naming a
/// total above an array holding a different one is the defect this pin exists
/// to prevent, one level up. The count is the array's length and nothing
/// restates it.
///
/// An entry the scan no longer finds, and a candidate no entry classifies, are
/// both problems — and they are two different sentences, because they mean
/// opposite things about which side moved.
const FROZEN_DOC_MUSTS: &[DocumentationMust] = &[
    DocumentationMust {
        clause: "VT-13",
        disposition: Disposition::Pinned {
            site: "crates/happenstance-core/src/event.rs",
            anchor: "resumes with `ReadOptions::from(checkpoint.next()?)`, and that is sound \
                     on a store with gaps",
        },
    },
    DocumentationMust {
        clause: "VT-15",
        disposition: Disposition::Pinned {
            site: "crates/happenstance-core/src/tag.rs",
            anchor: "# Equality is byte equality, and nothing is normalised",
        },
    },
    DocumentationMust {
        clause: "VT-17",
        disposition: Disposition::Pinned {
            site: "crates/happenstance-core/src/tag.rs",
            anchor: "a key may legally appear more than once",
        },
    },
    DocumentationMust {
        clause: "VT-21",
        disposition: Disposition::Excluded {
            reason: "the obligation is on a store — `MUST document its actual limit` — not on \
                     the contract's own documentation",
        },
    },
    DocumentationMust {
        clause: "VT-22",
        disposition: Disposition::Excluded {
            reason: "as VT-21: the limit a store documents is the store's, not the contract's",
        },
    },
    DocumentationMust {
        clause: "VT-24",
        disposition: Disposition::Excluded {
            reason: "as VT-21: the limit a store documents is the store's, not the contract's",
        },
    },
    DocumentationMust {
        clause: "VT-32",
        disposition: Disposition::Pinned {
            site: "crates/happenstance-core/src/event.rs",
            anchor: "an invalid one that nothing reads survives `check`, `clippy`, `build` \
                     and `test`",
        },
    },
    DocumentationMust {
        clause: "VT-33",
        disposition: Disposition::Pinned {
            site: "crates/happenstance-core/src/tag.rs",
            anchor: "Not the amortised O(1) `Extend` usually implies.",
        },
    },
    DocumentationMust {
        clause: "ES-19",
        disposition: Disposition::Pinned {
            site: "crates/happenstance-core/src/store.rs",
            anchor: "It is not a sound `after` for a follow-up condition",
        },
    },
    DocumentationMust {
        clause: "ES-23",
        disposition: Disposition::Pinned {
            site: "crates/happenstance-core/src/store.rs",
            anchor: "# Cancellation",
        },
    },
    DocumentationMust {
        clause: "ES-24",
        disposition: Disposition::Pinned {
            site: "crates/happenstance-core/src/store.rs",
            anchor: "at-most-once under verbatim reissue",
        },
    },
    DocumentationMust {
        clause: "ES-26",
        disposition: Disposition::Excluded {
            reason: "`[FROZEN]` and on the contract's own documentation, and **not discharged**: \
                     both halves are documented but nothing says the asymmetry is deliberate. \
                     Anchoring it needs a doc-comment edit, which is HS-P0023's under \
                     `.kb/governance/rewrite-the-referent-never-the-reasoning.md`",
        },
    },
    DocumentationMust {
        clause: "ES-35",
        disposition: Disposition::Excluded {
            reason: "`[PROVISIONAL]`, and the obligation is on an adapter that declines \
                     durability rather than on the contract",
        },
    },
    DocumentationMust {
        clause: "ES-40",
        disposition: Disposition::Excluded {
            reason: "`[PROVISIONAL]`. Its discharge at `append.rs:29` is present today and \
                     would anchor on `# A claim about one store's log, not about the world`, \
                     so this becomes a pinned entry the day the clause freezes",
        },
    },
    DocumentationMust {
        clause: "PS-31",
        disposition: Disposition::Excluded {
            reason: "`[FROZEN]` and on the port's own documentation, and **not discharged**: \
                     `projection.rs` does not say that an event-emitting projection is out of \
                     scope at 0.1. HS-P0023's edit",
        },
    },
    DocumentationMust {
        clause: "PS-34",
        disposition: Disposition::Excluded {
            reason: "`[PROVISIONAL — contingent on PS-5]`, and dead the moment `type Batch;` \
                     lands",
        },
    },
    DocumentationMust {
        clause: "PS-36",
        disposition: Disposition::Excluded {
            reason: "`[FROZEN]` and on the port's own documentation, and **not discharged**: \
                     `projection.rs` does not document that the `Send` flavour transitively \
                     requires `Batch: Send`. HS-P0023's edit",
        },
    },
    DocumentationMust {
        clause: "SY-32",
        disposition: Disposition::Excluded {
            reason: "`[DEFERRED]`, and `SyncPeer` is `happenstance-sync`'s port rather than \
                     the contract's",
        },
    },
    DocumentationMust {
        clause: "CF-35",
        disposition: Disposition::Excluded {
            reason: "the obligation is on this specification's own clauses and is discharged \
                     by `cargo xtask spec-trace` (CF-38), not by a doc comment",
        },
    },
    DocumentationMust {
        clause: "CF-39",
        disposition: Disposition::Excluded {
            reason: "the obligation is on a fixture — `The fixture MUST state the mechanism` \
                     — not on the contract's own documentation",
        },
    },
    DocumentationMust {
        clause: "CF-40",
        disposition: Disposition::Excluded {
            reason: "as CF-39: the ceilings a fixture states are the fixture's",
        },
    },
];

/// Doc-comment text with its markers stripped and its whitespace collapsed.
///
/// Applied to **both** sides of every anchor match, which is what lets an anchor
/// be written as one readable sentence while the discharge it names is wrapped
/// across three source lines.
///
/// The alternative was choosing anchors that happen to sit on one source line,
/// and it lost: every anchor available today does fit one line, so the trap is
/// invisible until somebody re-wraps a paragraph — and the remedy a false
/// positive teaches is deleting the entry, which is the one thing this pin
/// cannot survive.
fn normalised(text: &str) -> String {
    let mut out = String::new();
    for line in text.lines() {
        let line = line.trim_start();
        let line = line
            .strip_prefix("///")
            .or_else(|| line.strip_prefix("//!"))
            .or_else(|| line.strip_prefix("//"))
            .unwrap_or(line);
        for word in line.split_ascii_whitespace() {
            if !out.is_empty() {
                out.push(' ');
            }
            out.push_str(word);
        }
    }
    out
}

/// Whether `text` names `clause` as a whole identifier.
///
/// Clause ids nest — `ES-1` inside `ES-17`, `VT-3` inside `VT-33`, `ES-4` inside
/// `ES-40` — and a bare `contains` is a substring test. The shape is copied from
/// `names_rule` (`xtask/src/lints.rs:492-500`) rather than shared with it: that
/// defect shipped once already in this repository, where a longer rule name
/// silently discharged a shorter rule's obligation and inflated the arithmetic
/// built on the same match. Two twelve-line boundary tests in two modules is
/// cheaper than one helper that makes two checkers move together.
fn names_clause(text: &str, clause: &str) -> bool {
    let ident = |character: char| character.is_ascii_alphanumeric() || character == '-';
    text.match_indices(clause).any(|(at, _)| {
        let before = text[..at].chars().next_back().is_none_or(|c| !ident(c));
        let after = text[at + clause.len()..]
            .chars()
            .next()
            .is_none_or(|c| !ident(c));
        before && after
    })
}

/// The clause a specification line declares, if it declares one.
///
/// Resolution stays [`Clauses::ids`]' — membership and nothing else — so no
/// second family list exists. What this decides is *declaration position*: the
/// line's first citation-shaped token, at the very start once heading and bold
/// markers are stripped, and not closed immediately by `**`, which is how §1
/// writes a cross-reference. That much is `spec_trace::clause_id`'s shape,
/// copied rather than shared (Note 8) — and
/// `tests::the_declaration_scan_attributes_every_clause_the_parser_declares`
/// holds the two to the same set over the real document, so a divergence fails
/// instead of drifting.
fn declared_clause<'a>(line: &'a str, clauses: &Clauses) -> Option<&'a str> {
    let stripped = line.trim_start().trim_start_matches('#').trim_start();
    let stripped = stripped.strip_prefix("**").unwrap_or(stripped);
    let id = clause_citations(stripped).into_iter().next()?.id?;
    if !stripped.starts_with(id) || !clauses.ids.contains(id) {
        return None;
    }
    // `**ES-40**` closes immediately: that is a cross-reference, and §1 is full
    // of them. `**PS-1 — …` and `**CF-30 is …` continue inside the bold run.
    (!stripped[id.len()..].starts_with('*')).then_some(id)
}

/// Every clause whose body words a documentation obligation.
///
/// The derived half of assertion 3, and the reason a clause the sibling branch
/// adds surfaces as a gate failure on the merge-forward commit rather than as a
/// closeout re-check.
fn documentation_candidates(spec: &str, clauses: &Clauses) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    let mut current: Option<String> = None;
    let mut body = String::new();

    for line in spec.lines() {
        if let Some(id) = declared_clause(line, clauses) {
            if let Some(previous) = current.take()
                && words_an_obligation(&body)
            {
                out.insert(previous);
            }
            body.clear();
            current = Some(id.to_owned());
        }
        body.push_str(line);
        body.push('\n');
    }
    if let Some(previous) = current
        && words_an_obligation(&body)
    {
        out.insert(previous);
    }
    out
}

/// Whether one clause body carries any of [`DOCUMENTATION_OBLIGATIONS`].
///
/// Normalised first, so a phrase a soft wrap split across two lines is still
/// found. `PS-34`'s is exactly that, and it is why the scan is not line-keyed.
fn words_an_obligation(body: &str) -> bool {
    let body = normalised(body);
    DOCUMENTATION_OBLIGATIONS
        .iter()
        .any(|phrase| body.contains(phrase))
}

/// Refuses a pin that cannot fail, before any check over it runs.
///
/// Three ways an enumeration becomes decorative, and all three are hard errors
/// rather than problems, because each one makes the checks below say nothing
/// while looking like they said something.
///
/// # Errors
///
/// When [`FROZEN_DOC_MUSTS`] is empty; when it names one clause twice, which
/// would let a dedup absorb an entry and make the derived-versus-hand-written
/// comparison lie about which side moved; or when an anchor names its own
/// clause id — as `main` stands, no discharge site names its clause id, so an
/// id-as-anchor entry fails the day it lands and the repair a contributor
/// reaches for is widening the match until it passes.
fn guard_pin(pin: &[DocumentationMust]) -> Result<()> {
    if pin.is_empty() {
        bail!(
            "the frozen documentation MUST pin enumerates nothing, so every check over it is vacuous"
        );
    }

    let mut seen: BTreeSet<&str> = BTreeSet::new();
    for entry in pin {
        if !seen.insert(entry.clause) {
            bail!(
                "the pin names `{}` twice; a duplicate lets one entry be absorbed and makes \
                 the derived-versus-hand-written comparison lie about which side moved",
                entry.clause
            );
        }
        if let Disposition::Pinned { anchor, .. } = entry.disposition
            && names_clause(anchor, entry.clause)
        {
            bail!(
                "the pin's anchor for `{}` names the clause id; no discharge site names its \
                 id, so an id-as-anchor entry fails on every site there is",
                entry.clause
            );
        }
    }
    Ok(())
}

/// Assertion 1 — every id the pin names is one the specification declares.
///
/// Membership in [`Clauses::ids`], which is exact rather than a substring test,
/// so `ES-1` cannot be satisfied by a document declaring only `ES-17`. The
/// problem names the id and the pin, never a page: a renumbered clause is not a
/// page's fault.
fn check_pin_resolution(pin: &[DocumentationMust], clauses: &Clauses, problems: &mut Vec<String>) {
    for entry in pin {
        if !clauses.ids.contains(entry.clause) {
            problems.push(format!(
                "{CHECKER} — the pin names `{}`, which SPECIFICATION.md does not declare",
                entry.clause
            ));
        }
    }
}

/// Assertion 2, the decision half — the discharging text is still there.
///
/// Both sides are [`normalised`] first, so re-wrapping a paragraph is not a gate
/// failure. A missing *file* is the caller's problem to report, with a different
/// sentence: a contributor who reads "the anchor is gone" and goes looking for a
/// missing file has been sent to the wrong place.
fn check_anchor(clause: &str, site: &str, anchor: &str, text: &str, problems: &mut Vec<String>) {
    if !normalised(text).contains(&normalised(anchor)) {
        problems.push(format!(
            "{site} — the discharge of `{clause}` no longer carries `{anchor}`; the \
             discharging text moved"
        ));
    }
}

/// Assertion 3 — the derived candidates and the hand-written enumeration agree.
///
/// Two directions, two sentences, and RS-81-5's requirement is that they are
/// never one: a candidate nobody classified means the **document grew**, and an
/// entry that is no longer a candidate means the **enumeration is stale**. Both
/// name the ids. "The counts disagree" would be neither.
fn check_pin_census(
    pin: &[DocumentationMust],
    clauses: &Clauses,
    derived: &BTreeSet<String>,
    problems: &mut Vec<String>,
) {
    let classified: BTreeSet<&str> = pin.iter().map(|entry| entry.clause).collect();

    for id in derived {
        if !classified.contains(id.as_str()) {
            problems.push(format!(
                "{CHECKER} — SPECIFICATION.md words a documentation obligation in `{id}`, \
                 which the pin classifies neither way; the document grew"
            ));
        }
    }
    for entry in pin {
        // An id the document no longer declares cannot be a candidate either,
        // and assertion 1 has already said so in the sentence that fits. Saying
        // it twice would give one defect two messages, the second of which
        // blames the enumeration for a clause that was renumbered under it.
        if !clauses.ids.contains(entry.clause) {
            continue;
        }
        if !derived.contains(entry.clause) {
            problems.push(format!(
                "{CHECKER} — the pin classifies `{}`, which words no documentation \
                 obligation in SPECIFICATION.md; the enumeration is stale",
                entry.clause
            ));
        }
    }
}

/// The pin, checked three ways, against one workspace root.
///
/// The I/O half: one read per pinned discharge site, and one scan of the
/// specification text the caller already read.
fn check_pin(
    root: &Path,
    pin: &[DocumentationMust],
    clauses: &Clauses,
    spec: &str,
    problems: &mut Vec<String>,
) {
    check_pin_resolution(pin, clauses, problems);

    for entry in pin {
        let Disposition::Pinned { site, anchor } = entry.disposition else {
            continue;
        };
        match fs::read_to_string(root.join(site)) {
            Ok(text) => check_anchor(entry.clause, site, anchor, &text, problems),
            Err(err) => problems.push(format!(
                "{site} — reading the discharge site for `{}` failed ({err}); the path moved",
                entry.clause
            )),
        }
    }

    check_pin_census(
        pin,
        clauses,
        &documentation_candidates(spec, clauses),
        problems,
    );
}

/// Every problem the tree carries, composed, in source order.
///
/// The order is the order a contributor reads: the tree first — the pinning
/// check before any other line, then each page in path order and each page's
/// problems in line order — and after it the two files that register and permit,
/// the harness and this module. Nothing short-circuits and nothing is truncated:
/// a check that stops at the first problem turns one review cycle into six.
fn problems(pages: &[Page], harness: &str, clauses: &Clauses) -> Vec<String> {
    let mut problems = Vec::new();
    check_paths(pages, &mut problems);

    let mut usage = vec![Usage::default(); IGNORE_ALLOWANCES.len()];
    for page in pages {
        check_page(page, IGNORE_ALLOWANCES, &mut usage, clauses, &mut problems);
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
    check(&workspace_root()?)
}

/// The whole step, against one workspace root.
///
/// Split from [`run`] only so the hard-error postures are assertable: a root
/// with no `spec/SPECIFICATION.md` must fail naming the *specification*, and a
/// `run` that discovers its own root cannot be handed one.
///
/// # Errors
///
/// The same conditions [`run`] documents.
fn check(root: &Path) -> Result<()> {
    // Resolved first, and exactly once. First because a specification the
    // checker cannot read is the *checker's* failure, and doing it before the
    // tree walk is what stops that failure arriving with a page's name on it.
    // Once because the set is shared by every check below that needs it.
    let clauses = Clauses::new(clause_ids(root)?);
    guard_pin(FROZEN_DOC_MUSTS)?;
    let spec = fs::read_to_string(root.join(SPECIFICATION))
        .with_context(|| format!("reading {SPECIFICATION}"))?;

    let pages = pages(root)?;
    guard_not_vacuous(&pages)?;

    let harness =
        fs::read_to_string(root.join(HARNESS)).with_context(|| format!("reading {HARNESS}"))?;
    let mut problems = problems(&pages, &harness, &clauses);
    check_pin(root, FROZEN_DOC_MUSTS, &clauses, &spec, &mut problems);

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

    use std::fmt::Write as _;
    use std::path::PathBuf;

    use super::*;
    use crate::REQUIRED;

    /// The composition root every mount assertion reads as text.
    const ROOT_MODULE: &str = "xtask/src/main.rs";

    /// One id per family the specification declares today, hand-built.
    ///
    /// Hand-built rather than read from `spec/SPECIFICATION.md`: the parse is
    /// `clause_ids`' own story's, and §1.3's hand count already checks it
    /// against the real document on every gate run, so a parallel fixture
    /// corpus here is exactly what the testing brief forbids. `ES-40` is the
    /// one the fixture page cites.
    const DECLARED: &[&str] = &["VT-1", "WF-1", "ES-40", "PS-1", "SY-1", "CF-1"];

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
        let clauses = Clauses::new(DECLARED.iter().map(|id| (*id).to_owned()).collect());
        let mut usage = vec![Usage::default(); allowances.len()];
        let mut problems = Vec::new();
        for page in pages {
            check_page(page, allowances, &mut usage, &clauses, &mut problems);
        }
        check_allowances(allowances, &usage, &mut problems);
        problems
    }

    /// Every problem one page carries under a hand-built resolution, composed
    /// through the same [`check_page`] a real run composes it through.
    ///
    /// The id set is hand-built rather than read from `spec/SPECIFICATION.md`:
    /// the parse is `clause_ids`' own story's, and §1.3's hand count already
    /// checks it against the real document on every gate run, so a parallel
    /// fixture corpus here is what the testing brief forbids.
    ///
    /// [`walk`] above resolves [`DECLARED`], and the only clause-shaped token
    /// the fence and marker fixtures carry is the `ES-40` [`FIXTURE_PAGE`]
    /// cites — which [`DECLARED`] declares. So the citation check contributes
    /// no problem to those tests and their assertions mean exactly what they
    /// meant before. The citation half is exercised here and by the recorded
    /// `cargo xtask narrative` run.
    fn cited(rel: &str, text: &str, ids: &[&str]) -> Vec<String> {
        let clauses = Clauses::new(ids.iter().map(|id| (*id).to_owned()).collect());
        let mut usage: Vec<Usage> = Vec::new();
        let mut problems = Vec::new();
        check_page(
            &page_with(rel, text),
            &[],
            &mut usage,
            &clauses,
            &mut problems,
        );
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

        let found = problems(&pages, &harness, &Clauses::new(BTreeSet::new()));

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

    /// Two obligations in one place: the limits this module's own checks
    /// create, and the `affected::run` divergence recorded rather than left
    /// implicit.
    ///
    /// One claim moved. This test used to require the phrase *names the
    /// harness, not the markdown*, and the run recorded in
    /// `observed-failure-falsification`'s `_falsification.md` measured that
    /// sentence to be false — the report names the *page's* path behind
    /// `xtask\src\../../`, and never this harness's filename. The record beats
    /// the forecast, so the claim was corrected where it holds
    /// (`xtask/src/narrative.rs`) and this module now points at that file
    /// instead of carrying a second, wrong copy.
    #[test]
    fn the_module_states_its_own_limits_and_its_one_divergence() {
        let docs: String = production_source()
            .lines()
            .take_while(|line| line.starts_with("//!"))
            .collect::<Vec<_>>()
            .join("\n");

        for claim in [
            "does not prove that the",
            "stated where they hold, in `xtask/src/narrative.rs`",
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

    // ======================================================================
    // narrative-citation-resolution
    // ======================================================================

    // ---- AC-001: the families are derived, and the resolver is called once --

    /// The fictional `ZZ-` family is the whole assertion: a checker holding a
    /// literal list of the six prefixes reports `ZZ-9` as an *undeclared
    /// family*, and one deriving its prefixes from the resolved set reports it
    /// as a *dangling id*. Adding a seventh family to `SECTIONS` therefore needs
    /// no second edit here.
    #[test]
    fn the_recognised_families_are_derived_from_the_resolved_set() {
        let found = cited(
            "append-conditions.md",
            "the boundary is checked as ZZ-9 requires\n",
            &["ZZ-1"],
        );

        assert_eq!(found.len(), 1, "got: {found:?}");
        assert!(
            found[0].contains("which SPECIFICATION.md does not define"),
            "a derived family must make `ZZ-9` a dangling id, not an unknown family, \
             got: {}",
            found[0]
        );
    }

    /// The one-call-per-run contract the resolver's own spec states on its
    /// consumers, held as text rather than as a review note: two calls read and
    /// parse a 9,070-line document twice inside one step.
    #[test]
    fn the_specification_is_resolved_once_per_run() {
        let source = production_source();

        assert_eq!(
            source.matches("clause_ids(").count(),
            1,
            "exactly one call site, before the page loop"
        );
        assert!(
            source.contains("let clauses = Clauses::new(clause_ids(root)?);"),
            "the set is resolved once in `check` and passed by reference"
        );
    }

    /// The foundation's marker is deleted by its first caller, not left to rot.
    #[test]
    fn the_resolvers_dead_code_marker_is_gone() {
        let resolver = read("xtask/src/spec_trace.rs");

        assert!(
            !resolver.contains("expect(\n        dead_code")
                && !resolver.contains("expect(dead_code"),
            "an unfulfilled expectation is a warning and the gate is `-D warnings`"
        );
        assert!(
            !resolver.contains("allow(dead_code"),
            "`allow` would rot into a permanent exemption, which is why it was `expect`"
        );
    }

    // ---- AC-002: a resolving citation passes, silently ---------------------

    #[test]
    fn a_citation_naming_a_declared_clause_is_not_a_problem() {
        let found = cited(
            "append-conditions.md",
            "a writer cannot be overtaken between reading and appending (ES-40).\n",
            DECLARED,
        );

        assert!(
            found.is_empty(),
            "provenance is checked, not narrated: {found:?}"
        );
    }

    // ---- AC-003: a dangling id names the page, the line and the id ----------

    /// The composed line `_design.md:333` draws, character for character.
    #[test]
    fn a_dangling_id_names_the_page_the_line_and_the_id() {
        let text = "one\ntwo\nthree\nfour\nfive\nsix\nseven\neight\nnine\nten\neleven\n\
                    a claim resting on ES-99\n";

        let found = cited("append-conditions.md", text, DECLARED);

        assert_eq!(found.len(), 1, "got: {found:?}");
        assert_eq!(
            found[0],
            "docs/append-conditions.md:12 — cites `ES-99`, which SPECIFICATION.md does not \
             define"
        );

        let (location, _) = found[0]
            .split_once(" — ")
            .unwrap_or_else(|| panic!("no em dash separator in `{}`", found[0]));
        assert!(
            location.chars().count() <= 48,
            "the location prefix is budgeted at 48 columns of an 80-column log, got: \
             {location}"
        );
    }

    // ---- AC-004: never a silent skip, and the three wordings stay apart -----

    #[test]
    fn a_citation_shaped_token_in_an_undeclared_family_is_a_problem() {
        let found = cited("append-conditions.md", "as XX-7 requires\n", DECLARED);

        assert_eq!(found.len(), 1, "got: {found:?}");
        assert!(
            found[0].contains("`XX-7`") && found[0].contains("no `XX-` family"),
            "the problem must name the token and its family, got: {}",
            found[0]
        );
        for family in ["CF-", "ES-", "PS-", "SY-", "VT-", "WF-"] {
            assert!(
                found[0].contains(family),
                "the message must list the families the specification declares so an author \
                 who meant a constitution rule can spell it in full; `{family}` missing from: \
                 {}",
                found[0]
            );
        }
    }

    /// `spec_trace::citations` skips a span it cannot read, which is safe there;
    /// here the span *is* the check, so a citation the checker cannot read is
    /// one nothing verifies. The wording stays distinct from the dangling form.
    #[test]
    fn a_near_miss_inside_a_declared_family_is_a_problem() {
        for text in [
            "the ES- family\n",
            "as ES-4O requires\n",
            "as ES -40 says\n",
        ] {
            let found = cited("append-conditions.md", text, DECLARED);

            assert_eq!(found.len(), 1, "`{text}` got: {found:?}");
            assert!(
                found[0].contains("looks like a clause citation and does not parse"),
                "`{text}` must report the cannot-read wording, got: {}",
                found[0]
            );
            assert!(
                !found[0].contains("does not define"),
                "`{text}` must stay distinguishable from a dangling id, got: {}",
                found[0]
            );
        }
    }

    // ---- AC-005: both boundaries, on prose that is correct ------------------

    #[test]
    fn a_constitution_rule_id_is_not_a_clause_citation() {
        for text in ["RS-81-1 says so\n", "and RS-00-1 too\n"] {
            assert!(
                cited("append-conditions.md", text, DECLARED).is_empty(),
                "`{text}` is a constitution rule id a page may legitimately cite"
            );
        }
    }

    #[test]
    fn an_adr_reference_is_not_a_clause_citation() {
        for text in ["ADR-0001 forbids it\n", "ADR-0029 raised the MSRV\n"] {
            assert!(
                cited("append-conditions.md", text, DECLARED).is_empty(),
                "`{text}` must not be read as `DR-0001`; the leading boundary is what stops it"
            );
        }
    }

    #[test]
    fn a_backlog_item_id_is_not_a_clause_citation() {
        for text in ["HS-S0142 is this story\n", "reserved for HS-P0020\n"] {
            assert!(
                cited("append-conditions.md", text, DECLARED).is_empty(),
                "`{text}` fails the shape outright: no digit follows the hyphen"
            );
        }
    }

    /// And the check is not simply always-empty: the same page with a real
    /// citation on it still resolves, and with a dangling one still reports.
    #[test]
    fn the_boundaries_are_both_load_bearing() {
        let text = "RS-81-1 and ADR-0001 and HS-S0142, then ES-40.\n";
        assert!(
            cited("append-conditions.md", text, DECLARED).is_empty(),
            "correct prose beside a resolving citation reports nothing"
        );

        let dangling = "RS-81-1 and ADR-0001 and HS-S0142, then ES-99.\n";
        assert_eq!(
            cited("append-conditions.md", dangling, DECLARED).len(),
            1,
            "the carve-outs must not have emptied the check"
        );
    }

    // ---- AC-006: every problem, source order, fences included ---------------

    #[test]
    fn five_dangling_ids_on_one_page_report_five_problems_in_source_order() {
        let text = "ES-91\nES-92\nES-93\nES-94\nES-95\n";

        let found = cited("append-conditions.md", text, DECLARED);

        assert_eq!(found.len(), 5, "got: {found:?}");
        for (index, problem) in found.iter().enumerate() {
            assert!(
                problem.starts_with(&format!("docs/append-conditions.md:{}", index + 1)),
                "problems read in source order, got: {problem}"
            );
            assert!(
                !problem.contains("more"),
                "no problem list is truncated: {problem}"
            );
        }
    }

    /// Excluding fenced regions would put a hiding place inside the one region
    /// that *is* the checked artifact.
    #[test]
    fn a_clause_id_inside_a_fence_is_still_checked() {
        let text = "a claim\n\n```rust\n// as ES-99 requires\nfn main() {}\n```\n";

        let found = cited("append-conditions.md", text, DECLARED);

        assert_eq!(found.len(), 1, "got: {found:?}");
        assert!(
            found[0].starts_with("docs/append-conditions.md:4 — "),
            "got: {}",
            found[0]
        );
    }

    /// The page text the module already read is what the check consumes; a
    /// second read doubles the I/O the `affected::run` argument rests on and can
    /// see a different file mid-edit.
    #[test]
    fn the_citation_check_consumes_the_page_text_already_read() {
        let source = production_source();

        assert_eq!(
            source.matches("read_page(").count(),
            2,
            "one definition and one call site: the page is read exactly once"
        );
        assert!(source.contains("fn check_citations(page: &Page, clauses: &Clauses"));
    }

    // ---- AC-007: the artifact that broke is the one named -------------------

    /// The resolver's error, propagated unchanged. A page blamed for the
    /// specification's condition does not satisfy this row, so the chain is
    /// asserted to carry no page path at all.
    #[test]
    fn an_unreadable_specification_is_propagated_not_swallowed() {
        let err = check(Path::new("this-root-does-not-exist"))
            .expect_err("a missing specification must end the run");

        let chain = format!("{err:#}");
        assert!(
            chain.contains("reading spec/SPECIFICATION.md"),
            "the chain must name the specification, got: {chain}"
        );
        assert!(
            !chain.contains(TREE),
            "the pages must not be blamed for the specification's condition, got: {chain}"
        );
    }

    /// RS-81-2, one medium over. Driven against the read seam the walk uses,
    /// because making a *present* page unreadable needs a temp directory and
    /// `xtask` deliberately carries no `tempfile` dev-dependency (DR-12).
    #[test]
    fn an_unreadable_page_is_a_hard_error_naming_the_page() {
        let err = read_page(&workspace_root().unwrap(), "no-such-page.md")
            .expect_err("a page that cannot be read must never be a skipped page");

        let chain = format!("{err:#}");
        assert!(
            chain.contains("reading docs/no-such-page.md"),
            "the chain must name the page, got: {chain}"
        );
        assert!(
            production_source().contains("read_page(root, &rel)?"),
            "the walk `?`-propagates it, so the page list can never be silently short by \
             one — the `let Ok(text) = … else {{ continue }}` is the rejected shape"
        );
    }

    // ---- AC-008: the limits, before the guarantee ---------------------------

    #[test]
    fn the_citation_check_documents_its_limits_before_its_guarantee() {
        let source = production_source();

        let limits = source
            .find("# What this does not verify")
            .expect("the module states no limits at all");
        let guarantee = source
            .find("fn check_citations")
            .expect("the citation check is missing");
        assert!(
            limits < guarantee,
            "a check whose limits are undocumented is read as a guarantee"
        );

        for limit in [
            "says nothing about the sentence above it",
            "is not told apart from prose",
            "collapses upstream",
        ] {
            assert!(
                source.contains(limit),
                "the module must state `{limit}` among its limits"
            );
        }
    }

    // ======================================================================
    // frozen-documentation-must-pin
    // ======================================================================

    fn pinned(clause: &'static str, site: &'static str, anchor: &'static str) -> DocumentationMust {
        DocumentationMust {
            clause,
            disposition: Disposition::Pinned { site, anchor },
        }
    }

    fn excluded(clause: &'static str) -> DocumentationMust {
        DocumentationMust {
            clause,
            disposition: Disposition::Excluded {
                reason: "a reason a reviewer reads",
            },
        }
    }

    /// A specification slice declaring every clause a pin names, each wording an
    /// obligation — so the census is silent and a test can drive the other two
    /// assertions without it.
    fn slice_for(pin: &[DocumentationMust]) -> String {
        let mut out = String::new();
        for entry in pin {
            let _ = write!(
                out,
                "#### {} — a clause shaped like the document's\n\n[FROZEN]\n\n\
                 The port's documentation MUST state the thing.\n\n",
                entry.clause
            );
        }
        out
    }

    /// Every problem a pin carries, against the real checkout, over a slice that
    /// classifies exactly what the pin does.
    fn pin_problems(pin: &[DocumentationMust]) -> Vec<String> {
        let ids: Vec<&str> = pin.iter().map(|entry| entry.clause).collect();
        pin_problems_over(pin, &ids, &slice_for(pin))
    }

    /// The same, with the resolution and the specification slice chosen.
    fn pin_problems_over(pin: &[DocumentationMust], ids: &[&str], spec: &str) -> Vec<String> {
        let clauses = Clauses::new(ids.iter().map(|id| (*id).to_owned()).collect());
        let mut problems = Vec::new();
        check_pin(
            &workspace_root().unwrap(),
            pin,
            &clauses,
            spec,
            &mut problems,
        );
        problems
    }

    // ---- AC-001: one enumeration, every candidate disposed of ---------------

    #[test]
    fn every_pin_entry_is_disposed_with_a_site_and_an_anchor() {
        assert!(!FROZEN_DOC_MUSTS.is_empty());

        let mut seen: BTreeSet<&str> = BTreeSet::new();
        for entry in FROZEN_DOC_MUSTS {
            assert!(!entry.clause.is_empty());
            assert!(
                seen.insert(entry.clause),
                "`{}` is enumerated twice",
                entry.clause
            );
            match entry.disposition {
                Disposition::Pinned { site, anchor } => {
                    assert!(site.starts_with("crates/"), "`{}`: {site}", entry.clause);
                    assert!(!anchor.trim().is_empty(), "`{}`", entry.clause);
                    assert!(
                        !names_clause(anchor, entry.clause),
                        "`{}`'s anchor names its own clause id; no site does",
                        entry.clause
                    );
                }
                Disposition::Excluded { reason } => {
                    assert!(
                        reason.trim().len() > 20,
                        "`{}` is excluded with no reason a reviewer can read",
                        entry.clause
                    );
                }
            }
        }
        assert!(
            FROZEN_DOC_MUSTS
                .iter()
                .any(|entry| matches!(entry.disposition, Disposition::Pinned { .. })),
            "an enumeration of nothing but exclusions is a pin that cannot fail"
        );
    }

    /// The trade `UNCLAIMED_PENDING_ADR` already made
    /// (`xtask/src/spec_trace.rs:1975-1978`): a count written into a comment can
    /// come to disagree with the array beneath it, which is the defect BR-10
    /// exists to prevent, one level up.
    #[test]
    fn the_pin_holds_no_written_count() {
        let source = production_source();
        let block = source
            .split("const FROZEN_DOC_MUSTS")
            .next()
            .expect("split always yields a first part");
        let comment: String = block
            .lines()
            .rev()
            .take_while(|line| !line.trim().is_empty())
            .collect::<Vec<_>>()
            .join(" ");

        // The three numbers that could come to disagree with the array beneath
        // the comment: its length, and the two halves of its disposition.
        let all = FROZEN_DOC_MUSTS.len();
        let held = FROZEN_DOC_MUSTS
            .iter()
            .filter(|entry| matches!(entry.disposition, Disposition::Pinned { .. }))
            .count();
        let tokens: Vec<&str> = comment
            .split(|character: char| !character.is_ascii_alphanumeric())
            .collect();

        for total in [all, held, all - held] {
            let numeral = total.to_string();
            assert!(
                !tokens.contains(&numeral.as_str()),
                "`{numeral}` stands alone in the pin's comment, where it can come to \
                 disagree with the array beneath it: {comment}"
            );
            // Number words, for the totals whose word is not also an ordinary
            // English one — `one` is, which is why the table starts at two.
            for (value, word) in [
                (2, "two"),
                (8, "eight"),
                (13, "thirteen"),
                (21, "twenty-one"),
            ] {
                assert!(
                    value != total || !comment.contains(word),
                    "`{word}` in the pin's comment is a written count: {comment}"
                );
            }
        }
    }

    // ---- AC-002: every pinned id resolves, through the shared set -----------

    #[test]
    fn a_pinned_id_absent_from_the_specification_is_a_problem() {
        let pin = [excluded("ES-19"), excluded("ES-23"), excluded("ES-24")];
        let ids = ["ES-19", "ES-24"];

        let found = pin_problems_over(&pin, &ids, &slice_for(&pin));

        assert_eq!(found.len(), 1, "got: {found:?}");
        assert!(
            found[0].contains("ES-23") && found[0].starts_with(CHECKER),
            "the problem names the missing id and the pin, never a page, got: {}",
            found[0]
        );
        assert!(
            !found[0].contains(TREE),
            "a renumbered clause is not a page's fault, got: {}",
            found[0]
        );
    }

    // ---- AC-003: the path moved and the text moved are two problems ---------

    #[test]
    fn a_missing_anchor_and_a_missing_site_are_two_different_problems() {
        let pin = [
            pinned(
                "ES-23",
                "crates/happenstance-core/src/store.rs",
                "a sentence that store.rs has never carried",
            ),
            pinned(
                "ES-24",
                "crates/no-such-crate/src/gone.rs",
                "# Cancellation",
            ),
        ];

        let found = pin_problems(&pin);

        assert_eq!(found.len(), 2, "got: {found:?}");
        assert!(
            found[0].contains("the discharging text moved") && found[0].contains("ES-23"),
            "got: {}",
            found[0]
        );
        assert!(
            found[1].contains("the path moved") && found[1].contains("ES-24"),
            "got: {}",
            found[1]
        );
        assert!(
            !found[0].contains("the path moved") && !found[1].contains("the discharging text"),
            "a contributor sent to a missing file by an anchor problem is sent to the wrong \
             place: {found:?}"
        );
    }

    /// The only test that touches the real checkout's discharge sites, and the
    /// one that would fail on the day a `happenstance-core` doc comment
    /// un-discharges a `[FROZEN]` clause.
    #[test]
    fn the_whole_pin_holds_against_the_real_tree() {
        let root = workspace_root().unwrap();
        let clauses = Clauses::new(clause_ids(&root).unwrap());
        let spec = read(SPECIFICATION);
        let mut problems = Vec::new();

        check_pin(&root, FROZEN_DOC_MUSTS, &clauses, &spec, &mut problems);

        assert!(problems.is_empty(), "the pin does not hold: {problems:?}");
    }

    // ---- AC-004: reflow-insensitive, and whole-identifier ------------------

    /// The criterion that protects the pin from its own users: the remedy a
    /// contributor reaches for when a check fires on an innocuous edit is to
    /// edit the check, and the check *is* the pin.
    #[test]
    fn the_anchor_match_survives_a_reflowed_doc_comment() {
        let anchor = "at-most-once under verbatim reissue";
        for text in [
            "/// **A conditional append is at-most-once under verbatim reissue.**\n",
            "    /// **A conditional append is at-most-once under\n    /// verbatim reissue.**\n",
            "//! at-most-once     under\n//!    verbatim reissue\n",
        ] {
            let mut problems = Vec::new();
            check_anchor(
                "ES-24",
                "crates/happenstance-core/src/store.rs",
                anchor,
                text,
                &mut problems,
            );
            assert!(problems.is_empty(), "`{text}` got: {problems:?}");
        }
    }

    /// `ES-1` inside `ES-17`, `VT-3` inside `VT-33`. The defect shipped once
    /// already in this repository, for conformance rule names.
    #[test]
    fn a_short_clause_id_does_not_match_a_longer_one() {
        let pin = [excluded("ES-1"), excluded("VT-3")];

        let found = pin_problems_over(&pin, &["ES-17", "VT-33"], &slice_for(&pin));

        assert_eq!(
            found.len(),
            2,
            "neither short id resolves against the longer one: {found:?}"
        );

        assert!(!names_clause("ES-17 is a different clause", "ES-1"));
        assert!(!names_clause("VT-33 is a different clause", "VT-3"));
        assert!(names_clause("as VT-3 requires", "VT-3"));
    }

    // ---- AC-005: which side moved, in two different sentences ---------------

    #[test]
    fn an_unclassified_candidate_says_the_document_grew() {
        let pin = [excluded("ES-19")];
        let spec = format!(
            "{}#### ES-23 — a clause the pin never classified\n\n\
             The port's documentation MUST state the thing.\n",
            slice_for(&pin)
        );

        let found = pin_problems_over(&pin, &["ES-19", "ES-23"], &spec);

        assert_eq!(found.len(), 1, "got: {found:?}");
        assert!(
            found[0].contains("ES-23") && found[0].contains("the document grew"),
            "the message must name the id and attribute the movement to the document, got: {}",
            found[0]
        );
    }

    #[test]
    fn an_entry_that_is_no_longer_a_candidate_says_the_enumeration_is_stale() {
        let pin = [excluded("ES-19"), excluded("ES-23")];
        let spec = format!(
            "{}#### ES-23 — a clause that words no documentation obligation\n\n[FROZEN]\n",
            slice_for(&pin[..1])
        );

        let found = pin_problems_over(&pin, &["ES-19", "ES-23"], &spec);

        assert_eq!(found.len(), 1, "got: {found:?}");
        assert!(
            found[0].contains("ES-23") && found[0].contains("the enumeration is stale"),
            "the message must name the id and attribute the movement to the enumeration, \
             got: {}",
            found[0]
        );
        assert!(
            !found[0].contains("the document grew"),
            "RS-81-5: never one sentence for two unrelated bugs, got: {}",
            found[0]
        );
    }

    /// The second declaration heuristic, held to the first over the real
    /// document — the assertion [`declared_clause`]'s own comment promises.
    ///
    /// [`declared_clause`] decides *declaration position* the way
    /// `spec_trace::clause_id` decides it, copied rather than shared (Note 8),
    /// and a copy is only honest while something holds the two to one answer.
    /// [`the_whole_pin_holds_against_the_real_tree`] cannot: it fails only when
    /// a divergence moves a **candidate**, and an attribution that slides a
    /// documentation obligation onto the wrong neighbour, or drops a clause
    /// wording none, leaves the candidate set untouched and passes it. So the
    /// re-derivation's `declared=200 ids=200 missing=[] extra=[]` measurement
    /// is made here on every run instead of being recorded once.
    ///
    /// One direction is structural and one is live, and saying which is the
    /// point: [`declared_clause`] returns only ids [`Clauses::ids`] contains,
    /// so `extra` is empty by construction and `missing` is what can fail
    /// today — a clause the parser declares that the line scan never sees at a
    /// declaration position. `extra` is still differenced and printed, because
    /// a later widening of that membership filter is exactly the change that
    /// would make it live, and a diagnostic added after the fact is one nobody
    /// reads the failure with.
    #[test]
    fn the_declaration_scan_attributes_every_clause_the_parser_declares() {
        let root = workspace_root().unwrap();
        let clauses = Clauses::new(clause_ids(&root).unwrap());
        let spec = read(SPECIFICATION);

        let attributed: BTreeSet<String> = spec
            .lines()
            .filter_map(|line| declared_clause(line, &clauses))
            .map(str::to_owned)
            .collect();

        let missing: Vec<&String> = clauses.ids.difference(&attributed).collect();
        let extra: Vec<&String> = attributed.difference(&clauses.ids).collect();

        // The two differences rather than `assert_eq!` on the sets themselves:
        // both sides carry every clause the document declares, and a failure
        // that prints two of those in full buries the handful of ids that
        // actually moved. Empty on both sides is the same claim.
        assert!(
            missing.is_empty() && extra.is_empty(),
            "the two declaration heuristics disagree over {SPECIFICATION}, so the candidate \
             scan attributes documentation obligations by a rule `clause_ids` does not share: \
             missing={missing:?} extra={extra:?}"
        );
    }

    // ---- AC-006: every problem, in one run, never truncated ------------------

    #[test]
    fn every_pin_problem_is_reported_not_just_the_first() {
        let pin = [
            excluded("ES-99"),
            pinned(
                "ES-23",
                "crates/happenstance-core/src/store.rs",
                "a sentence that store.rs has never carried",
            ),
        ];
        let spec = format!(
            "{}#### VT-15 — a candidate nobody classified\n\n\
             The documentation MUST state the thing.\n",
            slice_for(&pin)
        );

        let found = pin_problems_over(&pin, &["ES-23", "VT-15"], &spec);

        assert_eq!(found.len(), 3, "got: {found:?}");
        assert!(found[0].contains("ES-99"), "got: {}", found[0]);
        assert!(found[1].contains("ES-23"), "got: {}", found[1]);
        assert!(found[2].contains("VT-15"), "got: {}", found[2]);
    }

    #[test]
    fn the_pin_never_truncates_its_problem_list() {
        let pin: Vec<DocumentationMust> = (0..20)
            .map(|_| excluded("ES-99"))
            .enumerate()
            .map(|(index, mut entry)| {
                // Twenty distinct unresolvable ids, without twenty literals.
                entry.clause = [
                    "CF-91", "CF-92", "CF-93", "CF-94", "CF-95", "CF-96", "CF-97", "CF-98",
                    "CF-99", "PS-91", "PS-92", "PS-93", "PS-94", "PS-95", "PS-96", "PS-97",
                    "PS-98", "PS-99", "SY-91", "SY-92",
                ][index];
                entry
            })
            .collect();

        let found = pin_problems_over(&pin, &[], "");

        assert_eq!(found.len(), 20, "got {} problems", found.len());
        assert!(
            found.iter().all(|problem| !problem.contains("more")),
            "no problem list is truncated: {found:?}"
        );
    }

    // ---- AC-007: location first, inside the 48-column prefix ---------------

    /// Driven from the pin array and the candidate scan rather than from five
    /// literals, so a future condition cannot escape the budget by being added
    /// somewhere this test does not look.
    #[test]
    fn every_pin_problem_line_begins_with_its_artifact() {
        let pin = [
            excluded("ES-99"),
            pinned(
                "ES-23",
                "crates/no-such-crate/src/gone.rs",
                "# Cancellation",
            ),
            pinned(
                "ES-24",
                "crates/happenstance-core/src/store.rs",
                "a sentence that store.rs has never carried",
            ),
            excluded("VT-1"),
        ];
        let spec = format!(
            "{}#### VT-1 — a clause that words no documentation obligation\n\n[FROZEN]\n\n\
             #### VT-15 — a candidate nobody classified\n\n\
             The documentation MUST state the thing.\n",
            slice_for(&pin[..3])
        );

        let found = pin_problems_over(&pin, &["ES-23", "ES-24", "VT-1", "VT-15"], &spec);

        assert_eq!(
            found.len(),
            5,
            "every condition the pin can emit: {found:?}"
        );
        for problem in &found {
            let (location, message) = problem
                .split_once(" — ")
                .unwrap_or_else(|| panic!("no em dash separator in `{problem}`"));
            assert!(
                location.contains("/src/"),
                "the first visual row must begin with a repo-relative path, got: {location}"
            );
            assert!(
                location.chars().count() <= 48,
                "the location prefix is budgeted at 48 columns, got: {location}"
            );
            assert!(!message.is_empty());
            assert!(
                problem
                    .chars()
                    .all(|c| c != '\u{1b}' && !('\u{2500}'..='\u{257f}').contains(&c)),
                "hierarchy is position and adjacency only, got: {problem}"
            );
        }
    }

    // ---- EC-004/005/006: a pin that cannot fail is refused outright ---------

    #[test]
    fn an_empty_pin_is_a_hard_error() {
        let err = guard_pin(&[]).expect_err("an enumeration of nothing must not pass");

        assert!(err.to_string().contains("vacuous"), "got: {err}");
    }

    #[test]
    fn a_pin_naming_one_clause_twice_is_a_hard_error() {
        let pin = [excluded("ES-23"), excluded("ES-23")];

        let err = guard_pin(&pin).expect_err("a duplicate lets one entry be absorbed");

        assert!(err.to_string().contains("ES-23"), "got: {err}");
    }

    /// Verified on `main`: not one discharge site names its clause id, so an
    /// id-as-anchor entry fails on every site there is — and the repair a
    /// contributor reaches for is widening the match until it passes.
    #[test]
    fn an_anchor_equal_to_its_clause_id_is_a_hard_error() {
        let pin = [pinned(
            "ES-23",
            "crates/happenstance-core/src/store.rs",
            "ES-23",
        )];

        let err = guard_pin(&pin).expect_err("an id-as-anchor pin empties the check");

        assert!(err.to_string().contains("ES-23"), "got: {err}");

        // And the premise the rule rests on, held rather than asserted: the
        // *shipping* half of every discharge site — the doc comments a reader
        // meets, `#[cfg(test)]` excluded, exactly as `production_source` splits
        // this module — names no clause id, so an id-as-anchor pin would fail on
        // every entry there is.
        for entry in FROZEN_DOC_MUSTS {
            if let Disposition::Pinned { site, .. } = entry.disposition {
                let text = read(site);
                let shipping = text.split("#[cfg(test)]").next().unwrap();
                assert!(
                    !names_clause(shipping, entry.clause),
                    "{site} names `{}` outside its tests; the anchor rule's premise has changed",
                    entry.clause
                );
            }
        }
    }

    // ---- AC-008: the limits come before the pin, and claim nothing ---------

    #[test]
    fn the_module_states_the_pins_limits_before_the_pin() {
        let source = production_source();

        let limits = source
            .find("# What this does not verify")
            .expect("the module states no limits at all");
        let pin = source
            .find("const FROZEN_DOC_MUSTS")
            .expect("the pin is missing");
        assert!(
            limits < pin,
            "a check whose limits are undocumented is read as a guarantee"
        );

        for limit in [
            "reasoning around it is rewritten",
            "invisible to the derived scan",
            "never that it is *adequate*",
        ] {
            assert!(
                source.contains(limit),
                "the module must state `{limit}` among its limits"
            );
        }
    }

    #[test]
    fn no_pin_message_claims_a_discharge_is_correct() {
        let pin = [
            excluded("ES-99"),
            pinned(
                "ES-23",
                "crates/no-such-crate/src/gone.rs",
                "# Cancellation",
            ),
            pinned(
                "ES-24",
                "crates/happenstance-core/src/store.rs",
                "a sentence that store.rs has never carried",
            ),
        ];
        let spec = format!(
            "{}#### VT-15 — a candidate nobody classified\n\n\
             The documentation MUST state the thing.\n",
            slice_for(&pin)
        );

        let found = pin_problems_over(&pin, &["ES-23", "ES-24", "VT-15"], &spec);
        assert!(!found.is_empty());

        for problem in &found {
            for claim in [
                "verified", "correct", "proves", "teaches", "✓", "badge", "shield",
            ] {
                assert!(
                    !problem.contains(claim),
                    "`{claim}` reads as a claim the pin cannot make: {problem}"
                );
            }
        }
    }

    // ======================================================================
    // documented-blind-spots-and-their-proofs
    // ======================================================================

    /// The heading both modules must carry, first.
    const LIMITS_HEADING: &str = "What this does not verify";

    /// The retained `text`-fence fixture, tree-relative.
    ///
    /// Retained rather than deleted after its walk: limit 5's proof has to stay
    /// re-runnable, and a limit whose demonstration was thrown away is a limit
    /// stated rather than executed. Removing it is a three-file change — the
    /// page, its registration in [`HARNESS`], and the pinning test below — and
    /// that is the signal that limit 5's status changed.
    const TEXT_FIXTURE: &str = "text-fences.md";

    /// The record limit 3's prose cites for its three transcripts.
    ///
    /// Asserted as a *string in the docs* and never read: nothing in
    /// `xtask/src/` may open `.bklg/`, or a clean checkout would need the
    /// backlog to pass its own gate.
    const LIMITS_RECORD: &str = "_limits-evidence.md";

    /// One of the six limits `_decomposition.md` Note 10 enumerates.
    ///
    /// `owners` is the module or modules whose docs must state it: limits 1-4
    /// are properties of the compile mechanism and belong to [`HARNESS`],
    /// limit 5 is a property of the fence walk and belongs to [`CHECKER`], and
    /// limit 6 belongs, unhedged, to both. A module that states a limit it does
    /// not own is two spellings of one sentence, and one of them goes stale.
    ///
    /// `claim` and `instrument` are the two halves of the bullet shape
    /// `lint_constitution.rs:15-28` uses — what is not verified, and what the
    /// real instrument is. They are checked against the *same* bullet rather
    /// than against the file, because a limit whose compensating instrument is
    /// named three paragraphs away is a limit a reader meets as an apology.
    struct Limit {
        n: u8,
        owners: &'static [&'static str],
        claim: &'static str,
        instrument: &'static str,
    }

    /// Note 10's six, additive-only. None may be dropped, softened or reordered.
    const NOTE_TEN: &[Limit] = &[
        Limit {
            n: 1,
            owners: &[HARNESS],
            claim: "still demonstrating the claim above it",
            instrument: "friction log",
        },
        Limit {
            n: 2,
            owners: &[HARNESS],
            claim: "does not lint what is inside a fence",
            instrument: "reviewer reading the diff",
        },
        Limit {
            n: 3,
            owners: &[HARNESS],
            claim: "RUSTDOCFLAGS=-D warnings",
            instrument: LIMITS_RECORD,
        },
        Limit {
            n: 4,
            owners: &[HARNESS],
            claim: "the path it prints is the page's own",
            instrument: "the doctest's module name",
        },
        Limit {
            n: 5,
            owners: &[CHECKER],
            claim: "tagged `text` is neither compiled nor flagged",
            instrument: "docs/text-fences.md",
        },
        Limit {
            n: 6,
            owners: &[HARNESS, CHECKER],
            claim: "says nothing about whether any page teaches",
            instrument: "friction log",
        },
    ];

    /// A hedge on limit 6 — the one clause that turns it back into a promise.
    const HEDGES: &[&str] = &[" but ", " however", " although", " except", " unless "];

    /// The half of a module that ships, for the two that carry a test block.
    ///
    /// `read` returns the whole file, and this module's own `#[cfg(test)]` code
    /// quotes the very tokens the prose scan forbids — so scanning the file
    /// rather than the shipping half would fail on the test that exists to
    /// forbid them. The split is [`production_source`]'s, generalised to a path.
    fn shipping_source(module: &str) -> String {
        read(module)
            .split("#[cfg(test)]")
            .next()
            .unwrap_or_default()
            .to_owned()
    }

    /// A module's leading `//!` block, one entry per line, marker stripped.
    fn module_docs(source: &str) -> Vec<String> {
        source
            .lines()
            .take_while(|line| line.starts_with("//!"))
            .map(|line| line.trim_start_matches("//!").trim().to_owned())
            .collect()
    }

    /// The bullets of the [`LIMITS_HEADING`] section, one string per bullet.
    ///
    /// Continuation lines are folded into their bullet, which is what makes the
    /// claim-and-instrument pair checkable against one bullet rather than
    /// against the whole file.
    fn limits_bullets(docs: &[String]) -> Vec<String> {
        let mut out: Vec<String> = Vec::new();
        let mut inside = false;

        for line in docs {
            if let Some(heading) = line.strip_prefix("# ") {
                inside = heading.trim() == LIMITS_HEADING;
                continue;
            }
            if !inside {
                continue;
            }
            if let Some(bullet) = line.strip_prefix("* ") {
                out.push(bullet.to_owned());
            } else if !line.is_empty()
                && let Some(last) = out.last_mut()
            {
                last.push(' ');
                last.push_str(line);
            }
        }

        out
    }

    /// Every way one module's limits section can be wrong, in one pass.
    ///
    /// A `Vec` rather than an assertion so the negative arms below can name
    /// which wrong implementation was rejected, and so a module missing three
    /// limits reports three problems rather than the first.
    fn limits_problems(module: &str, source: &str) -> Vec<String> {
        let mut problems = Vec::new();
        let docs = module_docs(source);

        match docs.iter().find_map(|line| line.strip_prefix("# ")) {
            None => problems.push(format!(
                "{module} — the module docs carry no `#` heading at all, so the limits \
                 cannot be first"
            )),
            Some(heading) if heading.trim() != LIMITS_HEADING => problems.push(format!(
                "{module} — the first heading is `{}`; `{LIMITS_HEADING}` must come first, \
                 because a check whose limits are undocumented is read as a guarantee",
                heading.trim()
            )),
            Some(_) => {}
        }

        let bullets = limits_bullets(&docs);
        if bullets.is_empty() {
            problems.push(format!(
                "{module} — the limits section carries no bulleted body; six sentences run \
                 together in a paragraph satisfy every substring check and no reader"
            ));
        }

        for limit in NOTE_TEN {
            let stating = bullets.iter().find(|bullet| bullet.contains(limit.claim));

            if limit.owners.contains(&module) {
                match stating {
                    None => problems.push(format!(
                        "{module} — limit {} is missing: no bullet states `{}`",
                        limit.n, limit.claim
                    )),
                    Some(bullet) if !bullet.contains(limit.instrument) => problems.push(format!(
                        "{module} — limit {}'s bullet does not name the real instrument \
                         (`{}`); a limit stated without one reads as an apology rather than \
                         a redirection",
                        limit.n, limit.instrument
                    )),
                    Some(_) => {}
                }
            } else if stating.is_some() {
                problems.push(format!(
                    "{module} — restates limit {}, which holds in {}; two spellings of one \
                     limit is two things to update and one that goes stale",
                    limit.n, limit.owners[0]
                ));
            }
        }

        let teaching = NOTE_TEN
            .iter()
            .find(|limit| limit.n == 6)
            .expect("Note 10's sixth limit must be enumerated");
        if let Some(bullet) = bullets
            .iter()
            .find(|bullet| bullet.contains(teaching.claim))
        {
            for hedge in HEDGES {
                assert!(
                    !hedge.is_empty(),
                    "an empty hedge token would match every bullet"
                );
                if bullet.contains(hedge) {
                    problems.push(format!(
                        "{module} — limit 6 is hedged with `{}`; it is one unhedged sentence, \
                         and a clause qualifying it is how a green run becomes evidence of \
                         comprehension",
                        hedge.trim()
                    ));
                }
            }
        }

        problems
    }

    /// `source` with the bullet stating `claim` deleted, continuations and all.
    fn without_bullet(source: &str, claim: &str) -> String {
        let mut out: Vec<&str> = Vec::new();
        let mut dropping = false;

        for line in source.lines() {
            let body = line.strip_prefix("//!").map(str::trim_start);
            if body.is_some_and(|body| body.starts_with("* ")) {
                dropping = line.contains(claim);
            } else if dropping && body.is_none_or(str::is_empty) {
                dropping = false;
            }
            if !dropping {
                out.push(line);
            }
        }

        out.join("\n")
    }

    // ---- AC-001 / AC-002 / AC-003: the section, first, in both modules -----

    /// The shape a contributor meets: a real rustdoc heading, first, with a
    /// bulleted body — not a `//` comment and not a bullet under a preamble.
    ///
    /// Read as *text* through `workspace_root()` rather than asserted through
    /// rustdoc, because rustdoc cannot see either file the way this needs:
    /// `cargo doc -p xtask` documents the **lib** target only, so the bin
    /// crate's module docs are rendered by nothing in the gate — which is why
    /// `lint_constitution.rs:5`'s `[crate::constitution]` has never failed a
    /// step that denies every rustdoc warning. The technique is
    /// `lint_constitution.rs:423-425`'s, one file over.
    #[test]
    fn both_modules_state_every_limit_they_own_and_none_of_the_others() {
        for module in [HARNESS, CHECKER] {
            let found = limits_problems(module, &shipping_source(module));
            assert!(found.is_empty(), "{module} — got: {found:#?}");
        }
    }

    /// The bin crate has no path to the lib target's modules, and the
    /// `#[cfg(doctest)]` page modules do not exist under `cargo doc` at all, so
    /// a link either way is a broken intra-doc link — a hard error under the
    /// `documentation` step's `-D warnings`, not a warning.
    #[test]
    fn the_cross_target_reference_is_a_plain_path_and_not_an_intra_doc_link() {
        let harness_docs = module_docs(&shipping_source(HARNESS)).join("\n");
        let checker_docs = module_docs(&shipping_source(CHECKER)).join("\n");

        assert!(
            checker_docs.contains(HARNESS),
            "the checker must point at {HARNESS} by path for the four limits it does not state"
        );
        assert!(
            harness_docs.contains(CHECKER),
            "the harness must point at {CHECKER} by path for the limit it does not state"
        );

        for (module, docs) in [(HARNESS, &harness_docs), (CHECKER, &checker_docs)] {
            for link in ["[`crate::lint_narrative`]", "[`crate::narrative`]"] {
                assert!(
                    !docs.contains(link),
                    "{module} — `{link}` spans the target boundary and resolves to nothing"
                );
            }
        }
    }

    // ---- AC-004: limit 1 carries its inherently-untestable clause ----------

    /// The precedent is `xtask/src/constitution.rs:20-25`: a limit documented
    /// with no test behind it, because none is possible. A test that gestured
    /// at this one would be worse than none — it is the decorative instrument
    /// RS-81-1 names, the one a reader deletes the real instrument for.
    #[test]
    fn limit_one_says_no_mechanical_test_can_close_it() {
        let bullets = limits_bullets(&module_docs(&read(HARNESS)));
        let bullet = bullets
            .iter()
            .find(|bullet| bullet.contains("still demonstrating the claim above it"))
            .expect("the harness must state limit 1");

        assert!(
            bullet.contains("no mechanical test can close"),
            "limit 1 must say it is inherently untestable, got: {bullet}"
        );
        assert!(
            bullet.contains("semantic"),
            "limit 1 must say *why* no test is possible — the gap is semantic, not \
             mechanical, got: {bullet}"
        );
    }

    // ---- AC-005: limit 3 states the measurement, not either citation -------

    /// The two claims this repository holds disagree, so Note 10 item 3 asks
    /// for a measurement. The prose must carry what the re-run did and the
    /// toolchain it did it on; a measurement without its toolchain is a claim
    /// about nothing.
    #[test]
    fn limit_three_states_the_measurement_and_cites_the_record() {
        let bullets = limits_bullets(&module_docs(&read(HARNESS)));
        let bullet = bullets
            .iter()
            .find(|bullet| bullet.contains("RUSTDOCFLAGS=-D warnings"))
            .expect("the harness must state limit 3");

        for required in [LIMITS_RECORD, "1.97.1", "non_snake_case"] {
            assert!(
                bullet.contains(required),
                "limit 3 must carry `{required}`, got: {bullet}"
            );
        }
    }

    // ---- AC-009: three named wrong implementations, each rejected ----------

    /// Wrong implementation 1: the section is present, complete, and no longer
    /// the first thing a reader meets.
    #[test]
    fn a_limits_section_moved_below_another_heading_is_rejected() {
        let moved = format!(
            "//! # How the harness works\n//!\n//! A preamble that arrived first.\n//!\n{}",
            read(HARNESS)
        );

        let found = limits_problems(HARNESS, &moved);

        assert!(
            found
                .iter()
                .any(|problem| problem.contains("must come first")),
            "a section below another heading must be rejected, got: {found:#?}"
        );
    }

    /// Wrong implementation 2: five of the six.
    #[test]
    fn a_module_missing_one_of_its_limits_is_rejected() {
        let source = read(HARNESS);
        assert!(
            source.contains("does not lint what is inside a fence"),
            "the mutation below removes nothing unless limit 2 is there to remove"
        );

        let dropped = without_bullet(&source, "does not lint what is inside a fence");
        assert!(
            !dropped.contains("does not lint what is inside a fence"),
            "the mutation must remove the whole bullet, continuations and all"
        );

        let found = limits_problems(HARNESS, &dropped);

        assert!(
            found
                .iter()
                .any(|problem| problem.contains("limit 2 is missing")),
            "a dropped limit must be rejected by number, got: {found:#?}"
        );
    }

    /// Wrong implementation 3: limit 6 with a clause that gives it back.
    #[test]
    fn a_hedged_teaching_sentence_is_rejected() {
        let hedged = read(HARNESS).replace(
            "says nothing about whether any page teaches",
            "says nothing about whether any page teaches, but the gate does check every \
             fence in the tree, so",
        );

        let found = limits_problems(HARNESS, &hedged);

        assert!(
            found.iter().any(|problem| problem.contains("hedged with")),
            "a hedged limit 6 must be rejected, got: {found:#?}"
        );
    }

    /// Wrong implementation 4, AC-003's negative direction: the checker
    /// restating a limit that holds in the harness. This is what the two-place
    /// split costs if nothing enforces it.
    #[test]
    fn a_module_restating_the_other_modules_limit_is_rejected() {
        let restated = shipping_source(CHECKER).replace(
            "//! # What this does not verify\n",
            "//! # What this does not verify\n//!\n//! * **A failure names the harness, and \
             the path it prints is the page's own.** Restated here, which is the defect.\n",
        );

        let found = limits_problems(CHECKER, &restated);

        assert!(
            found
                .iter()
                .any(|problem| problem.contains("restates limit 4")),
            "a restated limit must be rejected by number, got: {found:#?}"
        );
    }

    // ---- AC-006 / AC-007: limit 5 is walked, and pinned by a test ----------

    /// The whole of limit 5's proof: the real fence walk, over the real
    /// retained fixture, reporting **nothing**.
    ///
    /// If a later change teaches the walk to inspect `text` fences, this test
    /// fails — and its message says what to do about it, which is what makes
    /// the coupling loud rather than annoying.
    #[test]
    fn the_text_fixture_is_not_flagged_by_the_real_fence_walk() {
        let text = read(&format!("{TREE}/{TEXT_FIXTURE}"));

        assert!(
            text.contains("```text"),
            "the fixture must carry a `text` fence or this test proves nothing"
        );

        let found = walk(&[page_with(TEXT_FIXTURE, &text)], IGNORE_ALLOWANCES);

        assert!(
            found.is_empty(),
            "limit 5 is closed: the fence walk now reports a `text` fence. Delete limit 5 \
             from {CHECKER}'s `# {LIMITS_HEADING}` section in this same change, and delete \
             this test with it — the limits section is now wrong. got: {found:#?}"
        );
    }

    /// A retained fixture nobody registered is a fixture the compile step never
    /// sees and the orphan check reports. Both directions, and the path budget
    /// that protects the terminal surface.
    #[test]
    fn the_text_fixture_is_registered_in_both_directions_and_inside_the_budget() {
        let harness = read(HARNESS);
        let page = page(TEXT_FIXTURE);

        assert!(
            harness.contains(&format!("include_str!(\"../../{TREE}/{TEXT_FIXTURE}\")")),
            "{HARNESS} must include {TEXT_FIXTURE}"
        );
        assert!(
            harness.contains(&format!("mod {} {{", page.module)),
            "{HARNESS} must declare `mod {}`",
            page.module
        );
        assert!(
            page.path.chars().count() <= PATH_BUDGET,
            "{} is {} characters; the budget is {PATH_BUDGET}",
            page.path,
            page.path.chars().count()
        );
        assert_eq!(
            page.rel.matches('/').count(),
            0,
            "the fixture must not add a directory level under {TREE}"
        );
    }

    /// A page nobody can route to is a page nobody reads. One row, appended,
    /// with the rows already there left where they were.
    #[test]
    fn the_text_fixture_has_an_index_row_and_reorders_nothing() {
        let index = read(INDEX);

        assert!(
            index.contains(&format!("]({TEXT_FIXTURE})")),
            "{INDEX} must route to {TEXT_FIXTURE}"
        );

        let existing = index
            .find("](append-conditions.md)")
            .expect("the index must still route to the page that was already there");
        let added = index
            .find(&format!("]({TEXT_FIXTURE})"))
            .expect("the index must route to the fixture");

        assert!(
            existing < added,
            "the new row must be appended, not inserted above the rows already there"
        );
    }

    // ---- AC-011: nothing anywhere claims the surface proves teaching -------

    /// The initiative's top-ranked risk, and this is the story most tempted to
    /// breach it, because it is the one that gets to describe what the machine
    /// does.
    #[test]
    fn neither_module_carries_a_mark_claiming_the_documentation_is_checked() {
        for module in [HARNESS, CHECKER] {
            let source = shipping_source(module);
            for mark in ["verified", "badge", "shield", "✅"] {
                assert!(
                    !source.contains(mark),
                    "{module} — `{mark}` reads as a claim neither module can make"
                );
            }
        }
    }
}
