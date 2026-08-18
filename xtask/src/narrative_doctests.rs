//! The narrative tree's compile step: enumerate the pages, then compile them.
//!
//! # What this does not verify
//!
//! Stated first, because a check whose limits are undocumented is read as a
//! guarantee — the same argument [`crate::lint_constitution`] opens with.
//!
//! * **It does not check that an example still demonstrates its claim.** The
//!   compiler has no opinion about the sentence above a fence, so a page whose
//!   prose has drifted away from its code passes this step unchanged. Adjacency
//!   is what makes that reviewable by a human; nothing here makes it mechanical.
//! * **It does not lint the fences.** Doctests receive neither the workspace
//!   `[lints]` table nor `cargo clippy`, which does not lint doctests at all, so
//!   `unwrap_used = "deny"` is unenforced inside every example in the tree.
//! * **The file a failure names is the harness, not the page.**
//!   `xtask/src/narrative.rs` carries that residual in full: the module resolves
//!   the page and the line resolves the location, and the path in front of both
//!   is `xtask/src/`.
//! * **It does not read the pages.** A fence tagged `text`, an untagged fence,
//!   an `ignore`d fence, a hidden panel, an unresolvable clause id and a page
//!   nobody registered are all invisible here. Those are the *checker's*, and
//!   the checker does not exist yet — `narrative-checker-mounted-with-pinned-path`
//!   and the stories after it build it. This step compiles what the harness
//!   names and asserts that the set is not empty; that is the whole of it.
//! * **What `RUSTDOCFLAGS=-D warnings` enforces inside a narrative fence is
//!   unmeasured here.** This step reaches `rustdoc` through an extra
//!   `cargo run -p xtask` hop that `xtask/src/constitution.rs`'s probe never had, so
//!   that probe's finding is deliberately not inherited. Re-running it is
//!   `documented-blind-spots-and-their-proofs`'.
//! * **A green banner here says nothing about whether the page teaches.**
//!
//! # Why a subcommand rather than a bare `cargo test --doc`
//!
//! `cargo test` exits 0 on `running 0 tests`, so the obvious spelling of this
//! step — a `cargo test --locked -p xtask --doc -- narrative::` in `REQUIRED` —
//! is green on a repository where the harness is mis-mounted, the tree is empty,
//! or no page was ever registered. [`crate::proof`] makes the argument at
//! length: a step that a *deletion* fails and an *emptying* passes is checking
//! the filename. So the doctests are asserted out of `--list` first, and the
//! count is printed, which is `RUNBOOK.md:918-925`'s lesson applied one corpus
//! further on.
//!
//! What this deliberately does **not** do is compare the pages on disk against
//! the modules in the harness. That comparison is the checker's (AC-005 of
//! `narrative-checker-mounted-with-pinned-path`), and two versions of one check
//! in the tree is how one error message ends up answering two questions.

use std::collections::BTreeSet;
use std::process::Command;

use anyhow::{Context, Result, bail};

/// The substring every narrative doctest's name carries.
///
/// Matched as a substring of the *name*, never as a path prefix: libtest spells
/// the location `xtask\src\../../docs/<page>.md` on Windows and
/// `xtask/src/../../docs/<page>.md` elsewhere, and a check that reads either is
/// a check that is wrong on one of them (NF-008).
const NARRATIVE_MARKER: &str = "narrative::";

/// The gate step's name, which is also the claim it makes.
///
/// Named once, here, because `REQUIRED` and the tests that hold that entry to
/// `probe: None` and to its position both have to agree, and a second spelling
/// is a second thing to get wrong — the argument [`crate::proof`]'s
/// `REGISTRY_PACKAGE` already makes.
pub(crate) const STEP: &str = "the narrative tree's examples compile";

/// The harness the pages are registered in.
const HARNESS: &str = "xtask/src/narrative.rs";

/// The target that harness must be declared from.
///
/// The pinned `TREE` constant and the tree-wide sweep that uses it are the
/// checker's, and land with `narrative-checker-mounted-with-pinned-path`; these
/// two paths are here because they are what the failure below has to name.
const DOCTEST_ROOT: &str = "xtask/src/lib.rs";

/// The invocation both halves of this step share.
///
/// One argv, used twice, so the enumeration and the run cannot drift into
/// asking about different builds — and so the second reuses the first's build
/// rather than paying for a second one.
const DOC_TEST: &[&str] = &["test", "--locked", "-p", "xtask", "--doc"];

/// Enumerate the narrative doctests, assert the set is not empty, then run them.
///
/// # Errors
///
/// When the doctests cannot be enumerated, when the enumeration names none of
/// them, or when a page's examples fail to compile or their assertions fail.
pub(crate) fn run() -> Result<()> {
    let listing = list()?;
    let pages = enumerated_pages(&listing)?;

    println!("  {} page(s)' examples enumerated", pages.len());

    let status = Command::new("cargo")
        .args(DOC_TEST)
        .args(["--", NARRATIVE_MARKER])
        .status()
        .context("failed to launch `cargo test -p xtask --doc` for the narrative tree")?;

    if !status.success() {
        bail!("the narrative tree's examples failed with {status}");
    }

    Ok(())
}

/// Every doctest name libtest reports for `xtask`'s lib target.
///
/// The whole listing rather than the parsed names, because the parse is pure and
/// the tests drive it against fixture strings.
fn list() -> Result<String> {
    let output = Command::new("cargo")
        .args(DOC_TEST)
        .args(["--", "--list"])
        .output()
        .context("failed to launch `cargo test -p xtask --doc -- --list`")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!(
            "could not enumerate `xtask`'s doctests:\n{}\n\n\
             This is not the same answer as \"the tree holds no examples\": the lib \
             target did not build, or `cargo` could not be reached. Diagnosing the \
             empty case depends on the two never sharing a message.",
            stderr.trim()
        );
    }

    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

/// The page modules a `--list` listing names.
///
/// Pages, not doctests: two examples on one page are one page, because the count
/// this step prints is a fact about the tree rather than about `rustdoc`.
fn narrative_pages(listing: &str) -> BTreeSet<String> {
    listing
        .lines()
        .filter_map(|line| line.trim().strip_suffix(": test"))
        .filter_map(|name| name.split_once(NARRATIVE_MARKER))
        .map(|(_, module)| {
            module
                .split_whitespace()
                .next()
                .unwrap_or(module)
                .to_owned()
        })
        .collect()
}

/// The page modules, or the failure that says why there are none.
fn enumerated_pages(listing: &str) -> Result<BTreeSet<String>> {
    let pages = narrative_pages(listing);

    if pages.is_empty() {
        bail!(
            "no `{NARRATIVE_MARKER}` doctest was listed, so this step would have \
             compiled nothing and exited 0 over `running 0 tests`.\n\n\
             The pages are registered in `{HARNESS}`, which must be declared from \
             `{DOCTEST_ROOT}`: `cargo test --doc` compiles the *lib* target's \
             doctests only, so a `mod narrative;` added to `xtask/src/main.rs` \
             instead lands in the bin crate, compiles clean, and leaves every page \
             in the tree compiled by nothing. Check that first, then check that the \
             tree still holds a page the harness names."
        );
    }

    Ok(pages)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, reason = "test code, per the house style")]

    use std::fs;
    use std::path::PathBuf;

    use super::*;
    use crate::REQUIRED;
    use crate::spec_trace::workspace_root;

    /// The fixture page this story authors, by path literal.
    ///
    /// A literal rather than a pinned constant on purpose: the tree-wide
    /// `TREE` / `HARNESS` / `HIDDEN_MARKERS` constants and the sweep that uses
    /// them are `narrative-checker-mounted-with-pinned-path`'s, and two
    /// versions of one check in the tree is worse than one narrow check here.
    const FIXTURE_PAGE: &str = "docs/append-conditions.md";

    /// The index the fixture page is routed from.
    const INDEX: &str = "docs/README.md";

    /// The two new modules AC-008 holds to stating their limits first.
    const NEW_MODULES: &[&str] = &["xtask/src/narrative.rs", "xtask/src/narrative_doctests.rs"];

    /// Every spelling of a control a reader would have to act on to read what is
    /// behind it. `_design.md` D2/DT-7 forbids all of them under the tree.
    const HIDDEN_MARKERS: &[&str] = &[
        "<details",
        "<summary",
        "{{#tabs",
        "{{#tab ",
        "{{#endtabs",
        "```admonish",
        "<!-- tab",
    ];

    /// The repo-relative path budget: 48 columns of terminal, less the
    /// 16-character `xtask\src\../../` prefix rustdoc puts in front of a
    /// doctest's name.
    const PATH_BUDGET: usize = 32;

    fn read(rel: &str) -> String {
        let root: PathBuf = workspace_root().unwrap();
        fs::read_to_string(root.join(rel)).unwrap_or_else(|err| panic!("reading {rel}: {err}"))
    }

    /// A listing line in the shape libtest actually prints, including the
    /// backslashed path spelling observed on Windows (NF-008).
    fn listed(page: &str, module: &str, line: u32) -> String {
        format!("xtask\\src\\../../docs/{page}.md - narrative::{module} (line {line}): test")
    }

    fn step_index(name: &str) -> usize {
        REQUIRED
            .iter()
            .position(|step| step.name == name)
            .unwrap_or_else(|| panic!("REQUIRED must contain the `{name}` step"))
    }

    // ---- AC-005 / AC-003: zero matching doctests is a hard failure ----------

    /// `cargo test` exits 0 on `running 0 tests`, so an empty enumeration is the
    /// decorative-step shape `RUNBOOK.md:918-925` records. It must be an error.
    #[test]
    fn an_empty_listing_is_a_problem() {
        assert!(enumerated_pages("").is_err());
    }

    /// The CR-1 mis-mount: `mod narrative;` declared from `xtask/src/main.rs`
    /// lands in the bin crate, compiles clean, and its pages are compiled by
    /// nothing. This listing is what that repository produces, and it is the
    /// only thing in this project that can see it.
    #[test]
    fn a_listing_with_no_narrative_doctest_is_a_problem() {
        let listing = "xtask\\src\\../../standards/rust/80-the-gate.md - \
                       constitution::the_gate (line 12): test\n\
                       \n1 test\n";

        let err = enumerated_pages(listing).unwrap_err().to_string();

        assert!(
            err.contains("xtask/src/narrative.rs"),
            "the failure must name the expected harness path, got: {err}"
        );
        assert!(
            err.contains("xtask/src/lib.rs"),
            "the failure must name the lib target the harness is declared from, got: {err}"
        );
        assert!(
            err.contains("main.rs"),
            "the failure must name the bin-crate mis-mount as the likely cause, got: {err}"
        );
    }

    /// And the positive direction, with the count the step prints on success —
    /// pages, not doctests, so two examples on one page stay one page.
    #[test]
    fn a_listing_with_one_narrative_doctest_is_accepted_and_counted() {
        let listing = format!(
            "{}\n{}\n{}\n\n3 tests\n",
            listed("append-conditions", "append_conditions", 12),
            listed("append-conditions", "append_conditions", 40),
            "xtask\\src\\../../README.md -  (line 31): test",
        );

        let pages = enumerated_pages(&listing).unwrap();

        assert_eq!(pages.len(), 1, "two examples on one page are one page");
        assert!(pages.contains("append_conditions"));
    }

    // ---- AC-004: mandatory, unprobed, locked, scoped, and ordered -----------

    /// `probe: Some(..)` means *skip when absent* (`xtask/src/main.rs:89-102`),
    /// and a documentation step that can skip is `RUNBOOK.md:918-925` again.
    #[test]
    fn the_narrative_step_is_required_and_unprobed() {
        let step = &REQUIRED[step_index(STEP)];
        assert!(
            step.probe.is_none(),
            "the narrative step must be mandatory on every runner, with no tool to install"
        );
    }

    /// The constitution's step is unfiltered, so it compiles these pages too.
    /// `run_steps` bails at the first failing step, so ordering is the whole of
    /// what keeps a broken narrative fence under the narrative banner.
    #[test]
    fn the_narrative_step_precedes_the_constitution_step() {
        assert_eq!(
            step_index(STEP) + 1,
            step_index("the constitution's examples compile"),
            "the narrative step must sit immediately before the constitution's"
        );
    }

    /// rustdoc does not read `RUSTFLAGS`, and an ambient `RUSTDOCFLAGS` leaks
    /// into every other step's rustdoc invocation (RS-80-3).
    #[test]
    fn the_narrative_step_carries_rustdocflags_in_its_own_env() {
        let step = &REQUIRED[step_index(STEP)];
        assert!(
            step.env.contains(&("RUSTDOCFLAGS", "-D warnings")),
            "the step's RUSTDOCFLAGS must be its own, not the process environment's"
        );
    }

    /// RS-80-4: every gate invocation that resolves dependencies passes it.
    #[test]
    fn the_narrative_step_passes_locked() {
        let step = &REQUIRED[step_index(STEP)];
        assert!(step.args.contains(&"--locked"));
    }

    // ---- AC-006: the mount is complete, and in the right family -------------

    /// `steps_named` panics on a name absent from `REQUIRED`, so a half-mount
    /// fails loudly rather than selecting nothing.
    #[test]
    fn the_gate_can_select_the_narrative_step_by_name() {
        let selected = crate::steps_named(&[STEP]);
        assert_eq!(selected.len(), 1);
        assert_eq!(selected[0].name, STEP);
    }

    /// `lint_steps()` is the file-reading family. This step compiles.
    #[test]
    fn the_narrative_step_is_not_a_lint_step() {
        assert!(
            REQUIRED.iter().any(|step| step.name == STEP),
            "the step must exist in REQUIRED before its family can be judged"
        );
        assert!(
            !crate::lint_steps().iter().any(|step| step.name == STEP),
            "the narrative step compiles; it does not belong to the file-reading five"
        );
    }

    // ---- AC-001: the page is readable without acting on anything -----------

    #[test]
    fn the_fixture_page_carries_no_hidden_marker_and_fits_the_path_budget() {
        let page = read(FIXTURE_PAGE);

        for marker in HIDDEN_MARKERS {
            assert!(
                !page.contains(marker),
                "{FIXTURE_PAGE} carries `{marker}`, which DT-7 forbids under the tree"
            );
        }

        assert!(
            FIXTURE_PAGE.len() <= PATH_BUDGET,
            "{FIXTURE_PAGE} is {} characters; the budget is {PATH_BUDGET}",
            FIXTURE_PAGE.len()
        );
    }

    // ---- AC-007: the index routes, and stops being false -------------------

    /// EC-008. The paragraph naming the trees the gate reads by path is the one
    /// sentence in the repository that this change makes false.
    #[test]
    fn the_index_names_the_narrative_tree_as_gate_read() {
        let index = read(INDEX);

        let paragraph = index
            .split("\n\n")
            .find(|para| para.contains("read by the gate"))
            .unwrap_or_else(|| panic!("{INDEX} no longer states which trees the gate reads"));

        assert!(
            !paragraph.contains("Two of those"),
            "{INDEX} still says two trees are gate-read; the narrative tree is a third"
        );
        assert!(
            paragraph.contains("narrative"),
            "the gate-read paragraph must name the narrative tree: {paragraph}"
        );
        assert!(
            paragraph.contains("cargo test -p xtask --doc"),
            "the gate-read paragraph must name the command that compiles it: {paragraph}"
        );
    }

    /// A reader who opened `docs/` wants the pages; a reader who wants the
    /// specification is being redirected, and redirection goes second.
    #[test]
    fn the_narrative_table_precedes_the_pointer_out_table() {
        let index = read(INDEX);

        let narrative = index
            .find("| Page |")
            .unwrap_or_else(|| panic!("{INDEX} carries no narrative routing table"));
        let pointer_out = index
            .find("| Looking for |")
            .unwrap_or_else(|| panic!("{INDEX} lost its pointer-out table"));

        assert!(
            narrative < pointer_out,
            "the narrative table must be the first table on {INDEX}"
        );
        assert!(
            index.contains("(append-conditions.md)"),
            "the narrative table must route to the page"
        );
    }

    // ---- AC-008: the limits are stated first, not last ----------------------

    /// The shape of `xtask/src/lint_constitution.rs:9-13`: a check whose limits
    /// are undocumented is read as a guarantee.
    #[test]
    fn the_new_modules_state_their_limits_first() {
        for module in NEW_MODULES {
            let source = read(module);

            let first_heading = source
                .lines()
                .take_while(|line| line.starts_with("//!"))
                .find_map(|line| line.strip_prefix("//! #"))
                .unwrap_or_else(|| panic!("{module}'s docs carry no headings at all"));

            assert_eq!(
                first_heading.trim(),
                "What this does not verify",
                "{module} must state its limits before anything else"
            );

            assert!(
                source.contains("teach"),
                "{module} must say, unhedged, that this step is silent about teaching"
            );
        }
    }
}
