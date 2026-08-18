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
//! * **It does not read a page's contents.** A fence tagged `text`, an untagged
//!   fence, an `ignore`d fence, a hidden panel and an unresolvable clause id are
//!   all invisible to the checks below. Those are the fence walk's, which
//!   `fence-discipline-and-allowance-list` and `hidden-content-resolution` add
//!   to this same module.
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
}

impl Page {
    /// The page at `rel`, tree-relative with `/` separators.
    ///
    /// The module name is derived exactly once, here, so both directions of the
    /// registration check compare the same string. Two spellings of one
    /// derivation is how the halves of an orphan check start disagreeing.
    fn new(rel: &str) -> Self {
        let path = format!("{TREE}/{rel}");
        Self {
            module: module_name(rel),
            index: path == INDEX,
            rel: rel.to_owned(),
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
            out.push(Page::new(&rel));
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
    name.len() > ".md".len() && name[name.len() - ".md".len()..].eq_ignore_ascii_case(".md")
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

/// Every problem the tree carries, composed, in source order.
///
/// The order is the order a contributor reads: the tree first — the pinning
/// check before any other line, then each page — and after it the files that
/// register the tree. Nothing short-circuits and nothing is truncated: a check
/// that stops at the first problem turns one review cycle into six.
fn problems(pages: &[Page], harness: &str) -> Vec<String> {
    let mut problems = Vec::new();
    check_paths(pages, &mut problems);
    check_registration(pages, harness, &mut problems);
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
        Page::new(rel)
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
        check_paths(&[Page::new(&rel)], &mut found);

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
        assert_eq!(Page::new(&inside).path.chars().count(), 31);
        assert_eq!(Page::new(&outside).path.chars().count(), 33);

        for (rel, expected) in [
            (inside.as_str(), 0),
            (outside.as_str(), 1),
            ("a/b/c.md", 1),
            ("adapters/sqlite.md", 0),
        ] {
            let mut problems = Vec::new();
            check_paths(&[Page::new(rel)], &mut problems);
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
}
