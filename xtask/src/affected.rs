//! The story-grain gate: the checks a *change* earns, rather than the checks the
//! *workspace* has.
//!
//! `cargo xtask ci` is the whole gate and stays the release bar. This is the
//! narrower one, run at every `redkiln advance` seam by `verify.affected_gate` in
//! `.redkiln/config.yaml`, where the full powerset-and-deny gate is too slow to
//! run at the grain of a single story. It answers one question — *which packages
//! could this diff have broken?* — and runs fmt, clippy and the tests for those.
//!
//! Cargo has no `--changed`, which is the whole reason this exists.
//!
//! # The two errors it can make, and which one it makes
//!
//! Naming too many packages costs time. Naming too few reports green over an
//! untested regression, which is the failure that matters, so every judgement
//! below is settled in the direction of *more* packages:
//!
//! - The dependency scan is textual. A member's manifest that mentions another
//!   member's package name **anywhere** — including in a comment — is treated as
//!   depending on it. That invents edges; it cannot miss one, because a Cargo
//!   dependency must name its package.
//! - A changed file that maps to no package at all, and a file this module has
//!   never heard of, both widen to the whole workspace rather than narrowing to
//!   nothing.
//! - The changed set is the union of committed, staged, unstaged and untracked
//!   files. A file the diff base cannot see still counts.
//!
//! # Why the file-reading checks always run
//!
//! [`crate::lints`] and [`crate::spec_trace`] run on every invocation, whatever
//! the diff touched. They are file reads that finish in the time cargo takes to
//! decide `xtask` is up to date, and the alternative is worse than it sounds: a
//! story whose whole deliverable is an edit to `SPECIFICATION.md` maps to no
//! package, so a purely package-shaped gate would compile nothing, read nothing,
//! and report green over a clause citing a rule that does not exist. The
//! specification is source in this repository even though rustc never opens it.
//!
//! **Every** lint [`crate::lints`] exports, and that is a checked claim rather
//! than a remembered one: `the_unconditional_block_runs_every_lint_the_module_
//! exports`, below, reads this file and `lints.rs` and holds one to the other.
//! It was not true when it was first written. `lints::stated_rule_counts` was
//! reached by no entry point but `REQUIRED`, so the story that adds a
//! conformance rule — the only change that lint exists for — was the one change
//! that never ran it, and five statements of which lints run had drifted into
//! five different answers with nothing to notice.
//!
//! **Every** file-reading check the *gate* has, which is the wider claim and the
//! one that survives an adversary:
//! `the_unconditional_block_runs_every_file_reading_check_in_the_gate`, below,
//! derives the family from `REQUIRED` — an entry point whose module never builds
//! a `Command` reads documents and compiles nothing — and holds this block to
//! it. The export scan above cannot reach that class, because this repository
//! adds file-reading checks as *modules*: `lint_narrative`, `lint_pages` and
//! `lint_constitution` export nothing from `lints.rs`, so a fourth one wired
//! into the gate and not into this block is green under an export scan and
//! green under every other test in this file. That mutation was run against the
//! tree before it was frozen as a fixture beside the check.
//!
//! Leaving a check off this block is therefore an entry in
//! `OFF_THE_STORY_GRAIN` carrying the argument for it, never an absence — and a
//! step that is *not* a file read is an entry in `RUNS_A_PACKAGE` saying what it
//! runs, reconciled against what its module actually does. Two hand lists, both
//! held to a derivation, because only the difference between the two can say
//! which of them moved.
//!
//! Nothing runs here that `cargo xtask ci` does not. The one thing that runs
//! there and not here is `lint-constitution`, which is a decision with its own
//! written rationale rather than an omission to tidy up; the narrative tree is
//! on this list for the argument recorded beside its call in [`run`], and the
//! constitution is off it for the same argument pointing the other way.
//!
//! # What it does not check
//!
//! Everything in `OPTIONAL` — the feature powersets, `cargo deny`, the nightly
//! `--cfg docsrs` build — and the `wasm32` targets, the packaging assertions and
//! the documentation build. Those are `cargo xtask ci`'s, and a story is not
//! released by passing this. `verify.integration_scoped` runs `ci --fast` and
//! `verify.e2e` runs the whole thing, which is where they are paid for.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{Context, Result, bail};

use crate::spec_trace::workspace_root;

/// The glob-free member roots, matching the workspace manifest's
/// `members = ["crates/*", "examples/*", "xtask"]`.
///
/// A directory here is scanned one level deep for a `Cargo.toml`; a manifest
/// directly at the path is taken as a member itself.
///
/// Duplicating the manifest's list is the risk this constant carries: a fourth
/// member root added to `Cargo.toml` and not here would leave that crate's
/// changes mapping to no package, and — because an unattributable path widens to
/// the whole workspace — the symptom would be a gate that got *slower*, which
/// nobody investigates. [`assert_covers_manifest`] is what makes it a failure
/// instead.
const MEMBER_ROOTS: &[&str] = &["crates", "examples", "xtask"];

/// Paths whose change invalidates every package's build.
///
/// Lockfile, toolchain pin, lint configuration and the workspace manifest: each
/// one reaches every member, and none of them lives inside a member directory.
const WORKSPACE_WIDE: &[&str] = &[
    "Cargo.toml",
    "Cargo.lock",
    "rust-toolchain.toml",
    "clippy.toml",
    "rustfmt.toml",
    "deny.toml",
    ".cargo/config.toml",
];

/// One workspace member.
pub(crate) struct Member {
    /// Its package name, as `cargo -p` spells it.
    name: String,
    /// Its directory, relative to the workspace root, with forward slashes.
    ///
    /// Forward-slashed on every platform because it is compared against git's
    /// output, which uses them on Windows too.
    dir: String,
    /// Its manifest, as an absolute path.
    ///
    /// Carried rather than rebuilt from [`Self::dir`] because [`dependent_map`]
    /// reads it, and a path relative to the workspace root is only openable from
    /// a process whose working directory happens to be that root — which the
    /// gate's is not, since cargo runs `xtask` from wherever it was invoked.
    manifest: PathBuf,
}

/// Run the affected-package gate against `base`.
///
/// `base` is the diff base — `main` when unstated, and the `{{base}}` redkiln
/// substitutes when the gate runs at an advance seam.
///
/// # Errors
///
/// When the workspace root cannot be resolved, when git cannot answer what
/// changed (including a `base` that does not resolve to a commit — a gate that
/// cannot tell what changed must not report green), when a member manifest
/// declares no package name, or when any check it runs fails.
/// The one package this command compiles at its default features rather than at
/// `--all-features`.
///
/// Named as a constant because it is used three times in one function and a
/// typo in any of them is a silent widening rather than an error: the package
/// would simply stay in the `--all-features` selection.
const LADYBUG: &str = "happenstance-ladybug";

pub(crate) fn run(base: Option<&str>) -> Result<()> {
    let base = base.unwrap_or("main");
    let root = workspace_root()?;

    // First, and unconditionally. See the module documentation: the packages
    // these read are not the packages the diff touched.
    println!("\n=== the file-reading checks ===");
    crate::spec_trace::retired_rules()?;
    crate::lints::no_clock()?;
    // CF-24, and it belongs on the story grain for the same reason
    // `stated_rule_counts` does: the story that lands a conformance rule is
    // exactly the one that can leave it out of the macro that drives it, and no
    // package-shaped step reads a `macro_rules!` body.
    crate::lints::rules_are_enumerated()?;
    crate::lints::no_position_literals()?;
    crate::lints::changelog_names_every_rule()?;
    // The one check here that runs the other way — four *documents* held to the
    // code, rather than the code held to a document — and the one this list had
    // dropped. It belongs on the story grain more than any of its neighbours: a
    // story that lands a conformance rule is exactly the change that leaves the
    // testkit's README, both `lib.rs` front pages and `happenstance-core`'s
    // feature comment stating a count the suite no longer has, and
    // `.redkiln/config.yaml` wires this command as that story's gate. The
    // person who breaks is the crates.io reader, and it had already happened
    // three times.
    crate::lints::stated_rule_counts()?;
    crate::lints::testkit_version()?;
    crate::lints::core_alloc_features()?;
    crate::spec_trace::run(crate::spec_trace::Mode::Check)?;
    // The narrative tree, on this list and not `lint-constitution`. The
    // divergence is deliberate and is argued in `lint_narrative`'s own docs: a
    // story whose whole deliverable is a page under `docs/` is exactly the case
    // a package-shaped gate reads nothing for, and `.redkiln/config.yaml` wires
    // this command as that story's grain.
    crate::lint_narrative::run()?;
    // The page-need discipline, for the same reason and as one half of a pair.
    // The other half is `"standards/pages/"` on `INERT` below: that tree reaches
    // no package, so without this line a rules-only pull request would run
    // *nothing*, which is strictly worse than the correct-but-slow widening it
    // replaces. The pages tree has the same shape one step further on — `docs/`
    // is on the `xtask` arm rather than `INERT`, but a pages-only diff would
    // still never reach a page-need check without this call.
    crate::lint_pages::run(crate::lint_pages::Mode::Check)?;
    // Called rather than excused, and the decision is the one
    // `OFF_THE_STORY_GRAIN` exists to force. `.github/workflows/` reaches no
    // package at all, so a story whose diff is a new CI job — which is exactly
    // the change that adds an action on a mutable tag, or a workflow with no
    // `permissions:` key — would otherwise clear its own grain having had its
    // only deliverable read by nothing. The check opens one directory and
    // starts no process, so it costs what the neighbours above it cost.
    crate::lint_workflows::run()?;

    let members = members(&root)?;
    let changed = changed_files(&root, base)?;

    println!("\n=== affected packages ===");
    println!("{} file(s) changed against `{base}`", changed.len());

    let affected = affected_packages(&changed, &members);

    if affected.is_empty() {
        // Not a silent pass: a story confined to `spec/`, `references/` or a
        // top-level prose file genuinely has no package to compile, and saying
        // so is the difference between "nothing to do" and "the gate did not
        // look". The emptiness this reports no longer covers `docs/`: the
        // narrative tree is source for `xtask`, so a page-only change reaches
        // this branch only if the arm above stopped firing.
        println!("no package affected — nothing to compile");
        println!("\naffected gate passed");
        return Ok(());
    }

    for name in &affected {
        println!("  {name}");
    }

    // Whole-workspace, and not per package. `cargo fmt` resolves no dependencies
    // and reads no build graph, so splitting it would buy nothing and could only
    // let an unaffected package drift out of format between releases.
    run_step("formatting", "cargo", &["fmt", "--all", "--check"])?;

    // `happenstance-ladybug` is held out of the two `--all-features` steps
    // below, and it is `xtask/src/main.rs`'s four exclusions at the story grain
    // rather than a second decision (ADR-0025 §9). `--all-features` turns on
    // this workspace's only dependency that arrives as a 1.44 GB static archive
    // and needs an OpenSSL toolchain, and a command whose whole promise is
    // "only what this diff could break, in the time a story allows" cannot pay
    // that -- least of all on a diff that touched the adapter, which is exactly
    // when this command is reached.
    //
    // It is held out rather than dropped: the step below compiles it at its
    // default features, which is the configuration a consumer who did not ask
    // for LadybugDB gets, and the driver's own coverage is `cargo xtask ci`'s
    // probed conformance step. Reporting green over a package nothing compiled
    // is the failure this module's documentation is about.
    let ladybug_affected = affected.iter().any(|name| name == LADYBUG);
    let selection: Vec<String> = affected
        .iter()
        .filter(|name| name.as_str() != LADYBUG)
        .flat_map(|name| ["-p".to_owned(), name.clone()])
        .collect();

    if ladybug_affected {
        run_step(
            "clippy (happenstance-ladybug, without its driver)",
            "cargo",
            &[
                "clippy",
                "--locked",
                "-p",
                LADYBUG,
                "--all-targets",
                "--",
                "-D",
                "warnings",
            ],
        )?;
    }

    // Every affected package may have been this one, in which case the two
    // steps below would run `cargo clippy --locked --all-targets
    // --all-features` with no `-p` at all -- which is the *whole workspace*,
    // driver included. The empty check is what stops a narrowing from becoming
    // a widening.
    if selection.is_empty() {
        println!("\nno package left to compile at `--all-features`");
        println!("\naffected gate passed");
        return Ok(());
    }

    let mut clippy = vec!["clippy".to_owned(), "--locked".to_owned()];
    clippy.extend(selection.iter().cloned());
    clippy.extend(
        ["--all-targets", "--all-features", "--", "-D", "warnings"]
            .iter()
            .map(|arg| (*arg).to_owned()),
    );
    run_step("clippy (affected packages)", "cargo", &borrow(&clippy))?;

    let mut test = vec!["test".to_owned(), "--locked".to_owned()];
    test.extend(selection.iter().cloned());
    // `--show-output` for the same reason the full gate passes it: a capability
    // a fixture declines still runs as a passing test, and its one `SKIP <rule>`
    // line is suppressed by libtest without this (CF-18).
    test.extend(
        ["--all-features", "--", "--show-output"]
            .iter()
            .map(|arg| (*arg).to_owned()),
    );
    run_step("tests (affected packages)", "cargo", &borrow(&test))?;

    println!("\naffected gate passed");
    Ok(())
}

/// The packages a changed-file set can have broken, with their dependents.
///
/// Public to the crate so a test can drive the mapping without a git repository
/// or a compiler: the whole risk in this module is that this function returns
/// *too few* names, and that is not observable from a green run.
pub(crate) fn affected_packages(
    changed: &BTreeSet<String>,
    members: &[Member],
) -> BTreeSet<String> {
    let mut direct = BTreeSet::new();

    for path in changed {
        if WORKSPACE_WIDE.iter().any(|wide| path == wide) {
            return members.iter().map(|member| member.name.clone()).collect();
        }

        match members
            .iter()
            .filter(|member| path.starts_with(&format!("{}/", member.dir)))
            // The longest matching directory wins, so a member nested inside
            // another member's directory is attributed to the inner one.
            .max_by_key(|member| member.dir.len())
        {
            Some(member) => {
                direct.insert(member.name.clone());
            }
            None => {
                // Outside every member. Three prose paths are real dependencies
                // of `xtask`, whose lib target compiles them as doctests; the
                // rest — `spec/`, `.github/`, `.bklg/`, `.kb/` — reaches no
                // package. Anything unrecognised widens rather than narrows.
                if path == "README.md"
                    || path.starts_with("standards/rust/")
                    || path.starts_with("docs/")
                {
                    // All three are real dependencies of `xtask`, whose lib
                    // target compiles them as doctests. `standards/rust/` is the
                    // Rust constitution and `docs/` is the narrative tree: the
                    // examples in both are only ever compiled through that crate,
                    // so without this arm a prose-only change selects nothing and
                    // the pages are never built on the pull request that breaks
                    // them.
                    //
                    // `"docs/"` is a literal here and will be `xtask::narrative`'s
                    // pinned `TREE` constant on the other side — the harness is a
                    // *lib*-target module (`xtask/src/narrative.rs`, declared from
                    // `xtask/src/lib.rs`) and this is a *bin*-target one, so the
                    // two cannot share a private constant, and a `pub` seam across
                    // the targets would cost more than seven characters of
                    // duplication. The pair moves together; this comment is what
                    // says so, because nothing else can. The structural fix — an
                    // error by name on a missing tree — is the checker's, not this
                    // module's.
                    //
                    // What this does not verify: selection is not compilation.
                    // Naming `xtask` means the package is built and tested on
                    // this change; whether any fence inside a page was compiled
                    // is `cargo xtask narrative-doctests`' answer, and nothing
                    // here says anything about whether a page teaches.
                    direct.insert("xtask".to_owned());
                } else if !is_inert(path) {
                    return members.iter().map(|member| member.name.clone()).collect();
                }
            }
        }
    }

    close_over_dependents(direct, members)
}

/// Whether a path outside every member reaches no package at all.
///
/// The list is deliberately short and deliberately a *list*: every path not on it
/// widens the gate to the whole workspace, so forgetting to add one costs time
/// and never coverage.
///
/// **Inert to the compiler is not inert to the gate.** `spec/` holds
/// `SPECIFICATION.md` and `E2E-CASES.md`, which `spec-trace` reads on every
/// invocation of this module regardless of what changed; `experiments/` is
/// outside the workspace by construction, so nothing here compiles it and its
/// own `Cargo.toml` opens with a bare `[workspace]` table to keep it that way.
/// Both reach no package, and both are still checked.
///
/// **Two** trees are deliberately **absent**, and stating them together is what
/// stops the next contributor reading the first as the sole exception.
/// `standards/rust/` is the Rust constitution and `docs/` is the narrative tree;
/// both are caught by the arm above, which selects `xtask` because their examples
/// compile as that crate's doctests. Adding either here would silently un-compile
/// a corpus — and leaving one here *shadowed* behind that arm is the same defect
/// one reordering away, which is why `docs/` was removed rather than left in
/// place when the narrative tree landed.
///
/// `pub(crate)` rather than private: `spec_trace`'s own tests (RV-3) call this
/// directly to hold its verdict about `experiments/` to `workspace_index`'s, so
/// the two checks can be asked the same question about the same path instead of
/// one of them being reconstructed from prose.
pub(crate) fn is_inert(path: &str) -> bool {
    const INERT: &[&str] = &[
        "spec/",
        // The page-need discipline's rules tree. Nothing compiles it — it is
        // deliberately not registered with the doctest harness — so it reaches
        // no package, exactly as `spec/` does. It is still *checked*: the
        // unconditional list above runs `lint_pages` on every invocation, and
        // that pairing is the whole entry. Adding this prefix without the call
        // makes a rules-only pull request read nothing at all.
        "standards/pages/",
        "references/",
        "experiments/",
        // The standing benchmark suite, and it is here for exactly the reason
        // `experiments/` above is: `benchmarks/Cargo.toml` opens with a bare
        // `[workspace]` table, so cargo cannot reach it from the root manifest
        // and nothing in this workspace compiles it. Without this prefix a
        // change under it is not inert, and the arm below widens the gate to
        // the **whole workspace** — a full run for a diff that cannot affect a
        // single member.
        //
        // Unlike `spec/` and `standards/pages/`, this prefix is *not* half of a
        // pair: no unconditional lint runs over it, and none should. CF-34
        // (`spec/SPECIFICATION.md:8747`) rejects a benchmark result gating a
        // merge, and a `xtask` step that read this tree on every invocation is
        // one edit away from being one.
        "benchmarks/",
        // The measurement host's provisioning: shell scripts, a systemd unit
        // and the `host.env` that declares the conditions both of them read.
        //
        // Inert for a **third** reason, and the difference is worth having in
        // writing. `spec/` reaches no package; `benchmarks/` is outside the
        // workspace by construction. Neither applies here — `ops/` is not a
        // member root and never will be, because no compiler in this repository
        // reads a `.sh` or a `.service` at all. There is nothing for cargo to
        // decline to reach.
        //
        // Also *not* half of a pair, and for a sharper reason than
        // `benchmarks/`'s. A check that ran over this tree on every invocation
        // would make the gate's own greenness depend on the state of one host —
        // which is worse than the benchmark case rather than better, because a
        // machine can be wrong in ways a file cannot. `ops/host/preflight.sh`
        // asserts those conditions and is deliberately reachable from nothing
        // that can turn a merge red.
        "ops/",
        ".github/",
        ".bklg/",
        ".kb/",
        ".redkiln/",
        ".idea/",
        "CHANGELOG.md",
        "CLAUDE.md",
        "CONTRIBUTING.md",
        "RUNBOOK.md",
        "LICENSE-MIT",
        "LICENSE-APACHE",
        ".gitignore",
    ];
    INERT
        .iter()
        .any(|inert| path == *inert || path.starts_with(inert))
}

/// Every package that depends, transitively, on one of `direct`.
fn close_over_dependents(direct: BTreeSet<String>, members: &[Member]) -> BTreeSet<String> {
    let dependents = dependent_map(members);

    let mut affected = direct.clone();
    let mut frontier: Vec<String> = direct.into_iter().collect();

    while let Some(name) = frontier.pop() {
        let Some(users) = dependents.get(&name) else {
            continue;
        };
        for user in users {
            if affected.insert(user.clone()) {
                frontier.push(user.clone());
            }
        }
    }

    affected
}

/// For each package, the members whose manifest mentions it.
///
/// Textual, and over-approximating on purpose — see this module's documentation.
/// A manifest that cannot be read contributes no edges rather than failing the
/// run: [`members`] has already established that every manifest exists, so an
/// unreadable one here is a race, and the safe reading of a race is to keep the
/// package in play.
fn dependent_map(members: &[Member]) -> BTreeMap<String, Vec<String>> {
    let mut map: BTreeMap<String, Vec<String>> = BTreeMap::new();

    for member in members {
        let Ok(text) = fs::read_to_string(&member.manifest) else {
            continue;
        };
        for other in members {
            if other.name != member.name && mentions(&text, &other.name) {
                map.entry(other.name.clone())
                    .or_default()
                    .push(member.name.clone());
            }
        }
    }

    map
}

/// Every workspace member, by package name and directory.
///
/// # Errors
///
/// When a member root cannot be read, or when a manifest carries no
/// `name` key inside its `[package]` table.
fn members(root: &Path) -> Result<Vec<Member>> {
    assert_covers_manifest(root)?;

    let mut members = Vec::new();

    for entry in MEMBER_ROOTS {
        let path = root.join(entry);

        if path.join("Cargo.toml").is_file() {
            members.push(member_at(root, &path)?);
            continue;
        }

        let dir = fs::read_dir(&path)
            .with_context(|| format!("failed to read the member root `{entry}`"))?;
        for child in dir {
            let child = child.with_context(|| format!("failed to walk `{entry}`"))?;
            let child = child.path();
            if child.join("Cargo.toml").is_file() {
                members.push(member_at(root, &child)?);
            }
        }
    }

    if members.is_empty() {
        bail!("no workspace member found under {MEMBER_ROOTS:?} — is this the workspace root?");
    }

    Ok(members)
}

/// Fail when the workspace manifest declares a member root [`MEMBER_ROOTS`] does
/// not cover.
///
/// The comparison is on *roots*, not on resolved members: `crates/*` and
/// `crates` are the same statement about where to look, and requiring the two
/// lists to be spelled identically would fail on a difference that means
/// nothing.
///
/// # What it does not check
///
/// The reverse direction. A root listed here and absent from `Cargo.toml` finds
/// no manifests, contributes no members, and costs nothing — so it is not worth a
/// failure. And it reads the `members` key textually; a manifest that declares
/// members some other way (a `workspace.exclude` interaction, a path dependency
/// pulled in implicitly) is invisible to it.
///
/// # Errors
///
/// When the workspace manifest cannot be read, when it declares no `members`
/// key, or when a declared root is not covered.
fn assert_covers_manifest(root: &Path) -> Result<()> {
    let manifest = root.join("Cargo.toml");
    let text = fs::read_to_string(&manifest)
        .with_context(|| format!("failed to read {}", manifest.display()))?;

    let declared =
        declared_member_roots(&text).context("the workspace manifest declares no `members` key")?;

    let missing = uncovered(&declared);

    if !missing.is_empty() {
        bail!(
            "Cargo.toml declares member root(s) {missing:?} that `MEMBER_ROOTS` does not cover — \
             changes under them would map to no package. Add them to xtask/src/affected.rs."
        );
    }

    Ok(())
}

/// Whether `text` names the package `name`, as a whole name.
///
/// A plain substring test is wrong in this workspace specifically, and wrong in
/// the *unsafe* direction for reasoning about it. Every crate here shares the
/// prefix `happenstance`, so `text.contains("happenstance")` holds for a manifest
/// that names only `happenstance-core` — which would make every member a
/// dependent of the typed layer, and a one-line change to `crates/happenstance`
/// would rebuild and re-test the entire workspace forever.
///
/// That is still the *over*-approximating direction, so it would never have
/// missed a regression. It is fixed anyway because a gate whose cost is unrelated
/// to the change is a gate people start passing `--no-verify` around.
///
/// A hyphen counts as part of a name, which is what distinguishes
/// `happenstance-core` from `happenstance`. Comments still count as mentions:
/// that over-approximation is deliberate and is the module's stated policy.
fn mentions(text: &str, name: &str) -> bool {
    let boundary = |byte: Option<u8>| match byte {
        None => true,
        Some(byte) => !byte.is_ascii_alphanumeric() && byte != b'-' && byte != b'_',
    };

    text.match_indices(name).any(|(at, _)| {
        let before = at.checked_sub(1).map(|i| text.as_bytes()[i]);
        let after = text.as_bytes().get(at + name.len()).copied();
        boundary(before) && boundary(after)
    })
}

/// The declared roots [`MEMBER_ROOTS`] does not cover.
fn uncovered(declared: &BTreeSet<String>) -> Vec<String> {
    declared
        .iter()
        .filter(|root| !MEMBER_ROOTS.contains(&root.as_str()))
        .cloned()
        .collect()
}

/// The member roots a workspace manifest declares, with any glob suffix removed.
///
/// Accepts the key on one line or spread over several, since either is valid TOML
/// and rustfmt does not reach `Cargo.toml`.
fn declared_member_roots(manifest: &str) -> Option<BTreeSet<String>> {
    let after = manifest.split_once("members")?.1;
    let after = after.trim_start().strip_prefix('=')?;
    let open = after.find('[')?;
    let close = after.find(']')?;
    let body = after.get(open + 1..close)?;

    Some(
        body.split(',')
            .map(|entry| entry.trim().trim_matches('"').trim_matches('\''))
            .filter(|entry| !entry.is_empty())
            .map(|entry| entry.split('/').next().unwrap_or(entry).to_owned())
            .collect(),
    )
}

/// The member whose manifest sits in `dir`.
///
/// # Errors
///
/// When the manifest cannot be read, when `dir` is not under `root`, or when the
/// manifest's `[package]` table declares no `name`.
fn member_at(root: &Path, dir: &Path) -> Result<Member> {
    let manifest = dir.join("Cargo.toml");
    let text = fs::read_to_string(&manifest)
        .with_context(|| format!("failed to read {}", manifest.display()))?;

    let relative = dir
        .strip_prefix(root)
        .with_context(|| format!("{} is not under the workspace root", dir.display()))?;

    Ok(Member {
        name: package_name(&text)
            .with_context(|| format!("{} declares no package name", manifest.display()))?,
        dir: relative.to_string_lossy().replace('\\', "/"),
        manifest,
    })
}

/// The `name` of a manifest's `[package]` table.
///
/// Section-aware rather than a first-match scan: `name` is a legal key in
/// several other tables (`[[bin]]`, `[lib]`, `[[bench]]`), and a member that
/// names its binary before its package would otherwise be selected for `-p` under
/// a name cargo does not know.
fn package_name(manifest: &str) -> Option<String> {
    let mut in_package = false;

    for line in manifest.lines() {
        let line = line.trim();

        if line.starts_with('[') {
            in_package = line == "[package]";
            continue;
        }

        if !in_package {
            continue;
        }

        if let Some(value) = line.strip_prefix("name") {
            let value = value.trim_start();
            if let Some(value) = value.strip_prefix('=') {
                return Some(value.trim().trim_matches('"').to_owned());
            }
        }
    }

    None
}

/// Every file that differs from `base`, as forward-slashed repo-relative paths.
///
/// Four sources, unioned: what the branch committed since it diverged, what is
/// staged, what is modified in the working tree, and what is untracked. The
/// gate runs mid-story, where the last three routinely hold the whole change.
///
/// # Errors
///
/// When `base` does not resolve to a commit, or when git cannot be run. Both are
/// hard errors rather than an empty set: "git could not answer" and "nothing
/// changed" are the same value and opposite facts, and only one of them is safe
/// to report green.
fn changed_files(root: &Path, base: &str) -> Result<BTreeSet<String>> {
    let merge_base = git(root, &["merge-base", base, "HEAD"]).with_context(|| {
        format!("`{base}` does not resolve to a commit reachable from HEAD — pass --base <ref>")
    })?;
    let merge_base = merge_base.trim();

    let mut changed = BTreeSet::new();

    for args in [
        vec!["diff", "--name-only", merge_base, "HEAD"],
        vec!["diff", "--name-only", "--cached"],
        vec!["diff", "--name-only"],
        vec!["ls-files", "--others", "--exclude-standard"],
    ] {
        let out = git(root, &args)?;
        changed.extend(
            out.lines()
                .map(str::trim)
                .filter(|line| !line.is_empty())
                .map(|line| line.replace('\\', "/")),
        );
    }

    Ok(changed)
}

/// Run a git command in `root` and return its stdout.
///
/// # Errors
///
/// When git cannot be launched, or when it exits non-zero.
fn git(root: &Path, args: &[&str]) -> Result<String> {
    let out = Command::new("git")
        .current_dir(root)
        .args(args)
        .output()
        .with_context(|| format!("failed to launch `git {}`", args.join(" ")))?;

    if !out.status.success() {
        bail!(
            "`git {}` failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&out.stderr).trim()
        );
    }

    String::from_utf8(out.stdout).context("git printed output that is not UTF-8")
}

/// Borrow an owned argument vector as the `&str` slice `Command::args` wants.
fn borrow(args: &[String]) -> Vec<&str> {
    args.iter().map(String::as_str).collect()
}

/// Run one command, announcing it the way [`crate::run_steps`] announces a gate
/// step so the two read alike in a terminal.
///
/// # Errors
///
/// When the program cannot be launched, or when it exits non-zero.
fn run_step(name: &str, program: &str, args: &[&str]) -> Result<()> {
    println!("\n=== {name} ===");

    let status = Command::new(program)
        .args(args)
        .status()
        .with_context(|| format!("failed to launch `{program}`"))?;

    if !status.success() {
        bail!("{name} failed with {status}");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;

    /// Three synthetic members, for the cases that must not depend on the shape
    /// of the real tree. `manifest` points at nothing on purpose: these cases are
    /// about *path attribution*, and [`dependent_map`] contributing no edges
    /// keeps them independent of what the workspace happens to depend on today.
    fn members() -> Vec<Member> {
        vec![
            Member {
                name: "happenstance-core".to_owned(),
                dir: "crates/happenstance-core".to_owned(),
                manifest: PathBuf::from("/nonexistent/Cargo.toml"),
            },
            Member {
                name: "happenstance-testkit".to_owned(),
                dir: "crates/happenstance-testkit".to_owned(),
                manifest: PathBuf::from("/nonexistent/Cargo.toml"),
            },
            Member {
                name: "xtask".to_owned(),
                dir: "xtask".to_owned(),
                manifest: PathBuf::from("/nonexistent/Cargo.toml"),
            },
        ]
    }

    fn changed(paths: &[&str]) -> BTreeSet<String> {
        paths.iter().map(|path| (*path).to_owned()).collect()
    }

    #[test]
    fn a_source_change_selects_its_own_package() {
        let affected = affected_packages(&changed(&["xtask/src/main.rs"]), &members());
        assert!(affected.contains("xtask"));
    }

    #[test]
    fn the_lockfile_selects_everything() {
        let affected = affected_packages(&changed(&["Cargo.lock"]), &members());
        assert_eq!(affected.len(), members().len());
    }

    /// The failure this module exists to avoid: a path nobody taught it about
    /// resolving to *no* package, and the gate reporting green over it.
    #[test]
    fn an_unrecognised_path_widens_rather_than_narrows() {
        let affected = affected_packages(&changed(&["some/new/tree/file.rs"]), &members());
        assert_eq!(affected.len(), members().len());
    }

    /// Renamed from `a_docs_only_change_selects_nothing`, which stopped being
    /// true of `docs/` the moment the narrative tree landed there — it never
    /// asserted on a `docs/` path, so the assertion survives verbatim and only
    /// the name was false. Deleting it is forbidden: with
    /// [`an_unrecognised_path_widens_rather_than_narrows`] it is one of the two
    /// anchors of this module's widening posture.
    #[test]
    fn a_top_level_prose_file_selects_nothing() {
        let affected = affected_packages(&changed(&["RUNBOOK.md"]), &members());
        assert!(affected.is_empty());
    }

    /// The three trees that left `docs/` when it was reserved for user
    /// documentation. Each is inert for a different reason — `spec/` and
    /// `references/` reach no package, and `experiments/` is outside the
    /// workspace by construction — and each would widen the gate to everything
    /// if its prefix were dropped from the list.
    #[test]
    fn the_relocated_trees_stay_inert() {
        for path in [
            "spec/SPECIFICATION.md",
            "spec/E2E-CASES.md",
            "references/evaluation/PRESSURE-TEST.md",
            "experiments/wire-format/src/lib.rs",
        ] {
            let affected = affected_packages(&changed(&[path]), &members());
            assert!(affected.is_empty(), "{path} should reach no package");
        }
    }

    /// `README.md` is compiled as doctests of `xtask`'s lib target, so it is
    /// source for exactly one package and inert for every other.
    #[test]
    fn the_readme_selects_xtask() {
        let affected = affected_packages(&changed(&["README.md"]), &members());
        assert_eq!(affected, changed(&["xtask"]));
    }

    /// The Rust constitution's atoms are compiled the same way, and every
    /// neighbouring prose tree is inert — so without the arm this covers,
    /// editing an atom selects no package and its examples are never built on
    /// the change that breaks them. That is the whole of what makes the corpus's
    /// claim checkable.
    #[test]
    fn a_constitution_atom_selects_xtask() {
        let affected = affected_packages(
            &changed(&["standards/rust/21-send-is-not-inherited.md"]),
            &members(),
        );
        assert_eq!(affected, changed(&["xtask"]));
    }

    /// And the neighbouring prose stays inert, so the arm above is a rule about
    /// one directory rather than about every tree of markdown.
    #[test]
    fn the_constitution_arm_does_not_widen_to_all_prose() {
        let affected = affected_packages(&changed(&["references/adapter-shapes.md"]), &members());
        assert!(affected.is_empty());
    }

    /// The narrative tree is the third corpus compiled only as `xtask`'s
    /// doctests, and the one whose stories *are* prose-only: until this arm
    /// existed, the story-grain gate on the very pull request that broke a page
    /// compiled nothing and printed `affected gate passed`.
    #[test]
    fn a_narrative_page_selects_xtask() {
        let affected = affected_packages(&changed(&["docs/append-conditions.md"]), &members());
        assert_eq!(affected, changed(&["xtask"]));
    }

    /// And the same directional pair as the constitution's: one arm, one
    /// directory, and every neighbouring tree of markdown still inert.
    #[test]
    fn the_narrative_arm_does_not_widen_to_all_prose() {
        let affected = affected_packages(
            &changed(&["references/evaluation/PRESSURE-TEST.md"]),
            &members(),
        );
        assert!(affected.is_empty());
    }

    /// Asserted on the predicate rather than through [`affected_packages`],
    /// because the two directional tests above both pass while `"docs/"` sits
    /// *shadowed* on `INERT` behind an earlier `else if`. They cannot tell
    /// "removed" from "unreachable", and the next person to reorder that chain
    /// re-arms a prefix nobody meant to keep.
    #[test]
    fn the_narrative_tree_is_no_longer_inert() {
        assert!(!is_inert("docs/append-conditions.md"));
        assert!(!is_inert("docs/README.md"));
    }

    /// The page-need discipline's rules tree reaches no package, and it is the
    /// half of a pair: `affected::run` calls `lint_pages::run` unconditionally,
    /// so this prefix means "no package to build", never "nothing to check".
    /// Dropping the call and keeping this entry would make a rules-only pull
    /// request read nothing — strictly worse than the widening it replaces.
    #[test]
    fn the_rules_tree_selects_no_package() {
        for path in [
            "standards/pages/README.md",
            "standards/pages/00-one-need.md",
            "standards/pages/examples/two-needs.md",
        ] {
            let affected = affected_packages(&changed(&[path]), &members());
            assert!(affected.is_empty(), "{path} should reach no package");
        }
    }

    /// The standing benchmark suite reaches no package, and — unlike `spec/`
    /// and `standards/pages/` — nothing checks it either.
    ///
    /// Asserted on the predicate **and** through [`affected_packages`], for the
    /// reason `the_narrative_tree_is_no_longer_inert` gives one test up: the
    /// second call alone passes while the prefix sits shadowed behind an
    /// earlier arm, and could not tell "inert" from "unreachable".
    ///
    /// The wrong implementation this rejects is the absent entry, whose symptom
    /// is not a failure but a **full-workspace gate run** for a diff touching
    /// only `benchmarks/` — slow enough to be noticed, quiet enough to be
    /// blamed on the machine.
    #[test]
    fn the_benchmark_suite_selects_no_package() {
        for path in [
            "benchmarks/Cargo.toml",
            "benchmarks/src/corpus.rs",
            "benchmarks/benches/store_append.rs",
            "benchmarks/results/GRADES.md",
        ] {
            assert!(is_inert(path), "{path} should be inert");
            let affected = affected_packages(&changed(&[path]), &members());
            assert!(affected.is_empty(), "{path} should reach no package");
        }
    }

    /// The measurement host's provisioning reaches no package either, and for a
    /// reason neither of its neighbours gives.
    ///
    /// `spec/` reaches no package; `benchmarks/` is outside the workspace by
    /// construction. `ops/` is neither: it holds shell scripts, a systemd unit
    /// and an environment file, and **no compiler in this repository reads any
    /// of those file types**. There is nothing for cargo to decline to reach,
    /// which is why the prefix carries its own comment rather than sheltering
    /// under the `benchmarks/` one — a prefix whose stated argument does not
    /// cover what sits behind it is the defect this module already names twice.
    ///
    /// Dual assertion for the reason `the_benchmark_suite_selects_no_package`
    /// gives: [`affected_packages`] alone passes while a prefix sits shadowed
    /// behind an earlier arm, and cannot tell "inert" from "unreachable".
    ///
    /// The wrong implementation this rejects is the absent entry, whose symptom
    /// is a **full-workspace gate run** for a one-line edit to a shell script
    /// that no member compiles.
    #[test]
    fn the_host_provisioning_tree_selects_no_package() {
        for path in [
            "ops/host/preflight.sh",
            "ops/host/cpu-tuning.sh",
            "ops/host/host.env",
            "ops/host/happenstance-bench-tuning.service",
            "ops/host/README.md",
        ] {
            assert!(is_inert(path), "{path} should be inert");
            let affected = affected_packages(&changed(&[path]), &members());
            assert!(affected.is_empty(), "{path} should reach no package");
        }
    }

    /// And the sibling tree one directory over still selects `xtask`, so the
    /// arm at `:214-221` was not lazily broadened from `standards/rust/` to
    /// `standards/`. That widening is one keystroke and would silently
    /// un-compile the constitution.
    #[test]
    fn the_pages_prefix_does_not_swallow_the_constitution() {
        let affected = affected_packages(
            &changed(&["standards/rust/81-checks-that-cannot-be-types.md"]),
            &members(),
        );
        assert_eq!(affected, changed(&["xtask"]));
        assert!(!is_inert("standards/rust/README.md"));
    }

    #[test]
    fn member_roots_are_read_off_a_one_line_members_key() {
        let manifest = "[workspace]\nmembers = [\"crates/*\", \"examples/*\", \"xtask\"]\n";
        let roots = declared_member_roots(manifest).unwrap();
        assert_eq!(
            roots,
            ["crates", "examples", "xtask"]
                .iter()
                .map(|r| (*r).to_owned())
                .collect()
        );
    }

    #[test]
    fn member_roots_are_read_off_a_multi_line_members_key() {
        let manifest = "[workspace]\nmembers = [\n  \"crates/*\",\n  \"xtask\",\n]\n";
        let roots = declared_member_roots(manifest).unwrap();
        assert_eq!(
            roots,
            ["crates", "xtask"]
                .iter()
                .map(|r| (*r).to_owned())
                .collect()
        );
    }

    /// The wrong implementation the manifest check exists to reject: a member
    /// root added to `Cargo.toml` and not to [`MEMBER_ROOTS`], whose only symptom
    /// in production would be a gate that quietly got slower.
    #[test]
    fn an_unlisted_member_root_is_reported() {
        let manifest = "[workspace]\nmembers = [\"crates/*\", \"tools/*\"]\n";
        let roots = declared_member_roots(manifest).unwrap();
        assert_eq!(uncovered(&roots), vec!["tools".to_owned()]);
    }

    /// The defect the whole-name match exists to reject: every crate in this
    /// workspace is a `happenstance`-prefixed name, so a substring test makes the
    /// typed layer look like a dependency of everything.
    #[test]
    fn a_longer_package_name_is_not_a_mention_of_its_prefix() {
        let manifest = "happenstance-core = { workspace = true }\n";
        assert!(mentions(manifest, "happenstance-core"));
        assert!(!mentions(manifest, "happenstance"));
    }

    #[test]
    fn a_mention_is_found_at_a_line_start_and_at_end_of_file() {
        assert!(mentions("happenstance", "happenstance"));
        assert!(mentions("dep = \"happenstance\"", "happenstance"));
    }

    #[test]
    fn the_real_manifest_is_covered() {
        let root = workspace_root().unwrap();
        assert_covers_manifest(&root).unwrap();
    }

    #[test]
    fn package_name_ignores_a_bin_table() {
        let manifest = "[package]\nname = \"real\"\n\n[[bin]]\nname = \"other\"\n";
        assert_eq!(package_name(manifest).as_deref(), Some("real"));
    }

    #[test]
    fn package_name_is_not_taken_from_a_table_above_package() {
        let manifest = "[[bin]]\nname = \"other\"\n\n[package]\nname = \"real\"\n";
        assert_eq!(package_name(manifest).as_deref(), Some("real"));
    }

    /// The real workspace, so the member scan and the manifest parse are held to
    /// the tree they run against rather than to a fixture that cannot rot.
    #[test]
    fn every_workspace_member_resolves_to_a_package_name() {
        let root = workspace_root().unwrap();
        let members = super::members(&root).unwrap();

        assert!(
            members.iter().any(|m| m.name == "happenstance-core"),
            "the contract crate must be a member; found {:?}",
            members.iter().map(|m| &m.name).collect::<Vec<_>>()
        );
        assert!(members.iter().all(|m| !m.name.is_empty()));
    }

    /// A change to the contract crate must reach the testkit, or the gate would
    /// let a port change through without running the suite that judges it.
    #[test]
    fn a_contract_change_reaches_the_testkit() {
        let root = workspace_root().unwrap();
        let members = super::members(&root).unwrap();

        let affected =
            affected_packages(&changed(&["crates/happenstance-core/src/lib.rs"]), &members);

        assert!(
            affected.contains("happenstance-testkit"),
            "expected the testkit among {affected:?}"
        );
    }

    /// [`run`]'s unconditional file-reading block, as source text.
    ///
    /// Read from disk rather than reasoned about, because what is being
    /// asserted is *which calls are written there*. It is the instrument
    /// `lint_narrative` already points at this exact block
    /// (`xtask/src/lint_narrative.rs:2282`, `the_checker_joins_the_
    /// unconditional_file_reading_list`), and the two checks below are that
    /// one generalised.
    ///
    /// Scoped to the block rather than to the file, and that is load-bearing:
    /// this module's source contains its own tests, so a check for a call
    /// spelled out in a failure message would be discharged by the failure
    /// message. Both delimiters panic when they stop matching rather than
    /// returning an empty haystack every `contains` would fail against — a
    /// check that can lose its subject and stay green is the shape of defect
    /// this pair exists to reject.
    fn unconditional_block() -> String {
        let source = fs::read_to_string(workspace_root().unwrap().join("xtask/src/affected.rs"))
            .expect("this module's own source must be readable");

        let start = source
            .find("=== the file-reading checks ===")
            .expect("`run` must still announce the file-reading checks by that heading");
        let rest = &source[start..];
        let end = rest
            .find("let members = members(&root)?;")
            .expect("the file-reading block must still end where package selection begins");

        let block = rest[..end].to_owned();

        // The one widening that would defeat every `contains` below at once,
        // refused here rather than trusted to the comment above: an end
        // delimiter moved past the tests, whose failure messages spell the very
        // calls the checks look for. It is a two-word edit and it stays green.
        assert!(
            !block.contains("#[test]"),
            "the block has swallowed this module's own tests; every check over it would \
             then be discharged by its own failure messages"
        );

        block
    }

    /// The `pub(crate) fn <name>() -> Result<()>` items of a module, by name.
    fn exported_lints(source: &str) -> Vec<String> {
        source
            .lines()
            .filter_map(|line| line.strip_prefix("pub(crate) fn "))
            .filter_map(|rest| rest.split_once("() -> Result<()> {"))
            .map(|(name, _)| name.to_owned())
            .collect()
    }

    /// The check whose subject is the documents a story *edits*, on the gate
    /// that story *runs*.
    ///
    /// `stated_rule_counts` runs in the opposite direction to every other step
    /// in the gate — it holds four documents to the code rather than the code
    /// to a document — and a story that lands a conformance rule is precisely
    /// the change that leaves those four stating a count the suite no longer
    /// has. Leaving it off this list let such a story clear its own grain with
    /// the testkit's README, both `lib.rs` front pages and
    /// `happenstance-core`'s feature comment all still saying the old number.
    /// The person who breaks is the crates.io reader, and that failure had
    /// already happened three times before anything could see it.
    #[test]
    fn the_unconditional_block_runs_the_stated_rule_counts_lint() {
        assert!(
            unconditional_block().contains("crate::lints::stated_rule_counts()?;"),
            "the story-grain gate must run the rule-count check: `.redkiln/config.yaml` \
             wires this command at every advance seam, and a story that adds a \
             conformance rule is the one change it exists to catch"
        );
    }

    /// The general form of the defect above, and the reason it is a pair.
    ///
    /// The wrong implementation this rejects is the one that actually
    /// happened: a lint added to `lints.rs`, wired into `REQUIRED` so
    /// `cargo xtask ci` runs it, and never wired into the story grain — where
    /// it sat unrun for the fifteen commits that made the omission matter.
    /// The module documentation above claims `crate::lints` runs on every
    /// invocation; this is what makes that a checked claim rather than a
    /// remembered one.
    #[test]
    fn the_unconditional_block_runs_every_lint_the_module_exports() {
        let source = fs::read_to_string(workspace_root().unwrap().join("xtask/src/lints.rs"))
            .expect("`xtask/src/lints.rs` must be readable");
        let exported = exported_lints(&source);

        assert!(
            exported.iter().any(|name| name == "no_clock"),
            "the export scan no longer sees CF-33's lint, so it sees nothing and this \
             check would pass over anything; it saw {exported:?}"
        );

        let block = unconditional_block();
        let missing = exported
            .iter()
            .filter(|name| !block.contains(&format!("crate::lints::{name}()?;")))
            .collect::<Vec<_>>();

        assert!(
            missing.is_empty(),
            "`crate::lints` exports {missing:?}, which the story-grain gate never runs. \
             Either call them in `run`'s unconditional block or stop claiming, in this \
             module's own documentation, that this module's lints run whatever the diff \
             touched."
        );
    }

    // ---- the family, derived from the step table rather than from a module ---
    //
    // The check above scans `lints.rs` for exports, and this repository does not
    // add file-reading checks that way: `lint_narrative`, `lint_pages` and
    // `lint_constitution` are each their own *module*, dispatched by name and
    // named in `REQUIRED`. A fourth one, wired into the gate and never wired
    // into the story grain, is green under an export scan — `lints.rs` exports
    // nothing new — and green under every other test in this file. So the
    // subject below is the step table, which is where a check is actually added.

    /// The gate steps that run a package rather than reading a document, and
    /// why each one is not a file read.
    ///
    /// The hand-written half of the reconciliation in [`file_reading_family`].
    /// The derived half is whether the module behind the entry point ever
    /// builds a `Command`, and only the difference between the two artefacts
    /// can say which of two unrelated bugs happened: a check that started
    /// compiling something, or a name left here after the step it excused
    /// stopped doing so. That is `xtask/src/package.rs`'s `reconcile` shape
    /// (RS-81-5), one artefact over.
    const RUNS_A_PACKAGE: &[(&str, &str)] = &[
        (
            "proof-artefact",
            "asserts each phase's named tests out of `cargo test --list`, then runs them",
        ),
        (
            "wasm-conformance-enumeration",
            "enumerates each wasm32 target's rules through cargo, for that target",
        ),
        (
            "wasm-conformance",
            "executes the conformance rules on wasm32 under `wasm-bindgen-test-runner`",
        ),
        (
            "narrative-doctests",
            "compiles every fence in the narrative tree as `xtask`'s doctests",
        ),
        (
            "package-check",
            "reads `cargo package --list`, which builds each publishable artifact",
        ),
    ];

    /// The file-reading checks the story grain deliberately does not run, and
    /// the argument for each.
    ///
    /// A named list rather than an absence, and that is the whole point of it:
    /// a check added to the gate and not to [`run`]'s unconditional block has
    /// to be *decided about* — entered here with the reason, or called — rather
    /// than dropped by nobody noticing. Reversing an entry supersedes the
    /// argument it carries; it is not a tidy-up.
    const OFF_THE_STORY_GRAIN: &[(&str, &str)] = &[(
        "lint-constitution",
        "argued off this list in this module's own documentation and beside \
         `lint_narrative`'s call in `run`: the narrative tree is on the story grain \
         because a story's whole deliverable can be a page, and the constitution is \
         off it for the same argument pointing the other way",
    )];

    /// `xtask/src/<module>.rs`, or `None` when there is no such module.
    fn module_source(module: &str) -> Option<String> {
        fs::read_to_string(
            workspace_root()
                .unwrap()
                .join(format!("xtask/src/{module}.rs")),
        )
        .ok()
    }

    /// `xtask/src/main.rs`, as source text.
    fn main_source() -> String {
        fs::read_to_string(workspace_root().unwrap().join("xtask/src/main.rs"))
            .expect("`xtask/src/main.rs` must be readable")
    }

    /// The reason a hand list gives for a subcommand, if it names one.
    fn excused(list: &[(&'static str, &'static str)], subcommand: &str) -> Option<&'static str> {
        list.iter()
            .find(|(name, _)| *name == subcommand)
            .map(|(_, why)| *why)
    }

    /// The subcommand a gate step re-enters `xtask` with, if it does.
    ///
    /// `cargo run --locked --quiet -p xtask -- <subcommand>`, read off the
    /// argument vector rather than off the step's name. `cargo test --locked -p
    /// xtask --doc` names the package and no subcommand, and answers `None`: it
    /// compiles a corpus rather than running one of this binary's own entry
    /// points.
    fn xtask_subcommand(step: &crate::Step) -> Option<&'static str> {
        let at = step.args.iter().position(|arg| *arg == "--")?;
        if step.args.get(at.wrapping_sub(1)) != Some(&"xtask") {
            return None;
        }
        step.args.get(at + 1).copied()
    }

    /// Every entry point `REQUIRED` reaches by name, in table order.
    fn required_subcommands() -> Vec<&'static str> {
        crate::REQUIRED
            .iter()
            .filter_map(xtask_subcommand)
            .collect()
    }

    /// The path `main`'s dispatch calls for a subcommand — `lints::no_clock`,
    /// `lint_pages::run`.
    ///
    /// A flag-taking arm opens a nested `match`, and its `None` arm is the
    /// invocation a caller with no flag makes, which is the one the story grain
    /// would write. An arm this cannot read is reported as a problem by its
    /// callers rather than skipped: a subcommand nothing resolves is a
    /// subcommand nothing holds to anything.
    fn dispatch_target(main_rs: &str, subcommand: &str) -> Option<String> {
        let arm = main_rs
            .split_once(&format!("Some(\"{subcommand}\") => "))?
            .1;
        let body = if arm.starts_with("match ") {
            arm.split_once("None => ")?.1
        } else {
            arm
        };
        let path: String = body
            .chars()
            .take_while(|c| c.is_ascii_alphanumeric() || *c == '_' || *c == ':')
            .collect();
        path.contains("::").then_some(path)
    }

    /// The gate's file-reading family: every entry point in the step table
    /// whose module never builds a `Command`.
    ///
    /// Derived, and reconciled against [`RUNS_A_PACKAGE`] in both directions —
    /// the second return value is one problem per disagreement, naming which of
    /// the two moved.
    fn file_reading_family(
        subcommands: &[&'static str],
        main_rs: &str,
        source_of: &dyn Fn(&str) -> Option<String>,
    ) -> (Vec<&'static str>, Vec<String>) {
        let mut family = Vec::new();
        let mut problems = Vec::new();

        for subcommand in subcommands {
            let Some(target) = dispatch_target(main_rs, subcommand) else {
                problems.push(format!(
                    "`{subcommand}` is a step in `REQUIRED` and `main`'s dispatch cannot be \
                     read for it; a subcommand this check cannot resolve is one it holds to \
                     nothing"
                ));
                continue;
            };
            let module = target.split("::").next().unwrap_or(&target).to_owned();
            let Some(source) = source_of(&module) else {
                problems.push(format!(
                    "`{subcommand}` dispatches to `{target}` and `xtask/src/{module}.rs` \
                     cannot be read"
                ));
                continue;
            };

            // Derived. A module that never builds a `Command` starts no
            // process, so its whole input is files — which is exactly what
            // `print_help` calls "reads a document rather than compiling a
            // package". `lints`, `spec_trace`, `lint_narrative`, `lint_pages`
            // and `lint_constitution` hold to that; `proof`, `package` and
            // `narrative_doctests` do not.
            let spawns = source.contains("Command::new");

            match (spawns, excused(RUNS_A_PACKAGE, subcommand)) {
                (false, None) => family.push(*subcommand),
                (true, Some(_)) => {}
                (true, None) => problems.push(format!(
                    "`{subcommand}` dispatches to `{target}`, whose module starts a process, \
                     and no entry of `RUNS_A_PACKAGE` says so — either it reads documents and \
                     belongs on the family, or name it there with the reason"
                )),
                (false, Some(why)) => problems.push(format!(
                    "`RUNS_A_PACKAGE` excuses `{subcommand}` as \"{why}\", and \
                     `xtask/src/{module}.rs` starts nothing — it is a file read now, and the \
                     excuse is what has gone stale"
                )),
            }
        }

        (family, problems)
    }

    /// The family members [`run`]'s unconditional block never calls, and the
    /// exclusions that no longer name a check the gate has.
    fn unwired(family: &[&'static str], main_rs: &str, block: &str) -> Vec<String> {
        let mut problems = Vec::new();

        for subcommand in family {
            if excused(OFF_THE_STORY_GRAIN, subcommand).is_some() {
                continue;
            }
            let Some(target) = dispatch_target(main_rs, subcommand) else {
                problems.push(format!("`{subcommand}`'s dispatch arm cannot be read"));
                continue;
            };
            if !block.contains(&format!("crate::{target}(")) {
                problems.push(format!(
                    "`{subcommand}` reads documents on every `cargo xtask ci` and the \
                     story grain never runs it. `.redkiln/config.yaml` wires \
                     `cargo xtask affected` at every advance seam, so the change that \
                     breaks it is the change that clears its own grain. Call \
                     `crate::{target}` in `run`'s unconditional block, or name it in \
                     `OFF_THE_STORY_GRAIN` with the argument for leaving it out."
                ));
            }
        }

        for (subcommand, _) in OFF_THE_STORY_GRAIN {
            if !family.contains(subcommand) {
                problems.push(format!(
                    "`OFF_THE_STORY_GRAIN` excuses `{subcommand}`, which is no longer a \
                     file-reading check of the gate — a stale exclusion widens this check \
                     without saying so"
                ));
            }
        }

        problems
    }

    /// The derivation's subject: a step's argument vector, not its name.
    #[test]
    fn a_step_that_re_enters_xtask_is_derived_from_its_arguments() {
        let subcommand = crate::Step {
            name: "the zzz probe reads a document",
            program: "cargo",
            args: &[
                "run", "--locked", "--quiet", "-p", "xtask", "--", "lint-zzz",
            ],
            env: &[],
            probe: None,
        };
        assert_eq!(xtask_subcommand(&subcommand), Some("lint-zzz"));

        // The step directly below it in `REQUIRED`, which names the package and
        // no subcommand: it compiles the constitution's examples and is not an
        // entry point of this binary at all.
        let doctests = crate::Step {
            name: "the constitution's examples compile",
            program: "cargo",
            args: &["test", "--locked", "-p", "xtask", "--doc"],
            env: &[],
            probe: None,
        };
        assert_eq!(xtask_subcommand(&doctests), None);
    }

    /// Every file-reading check the gate has, on the gate a story runs.
    ///
    /// The class the export scan above could not see. Its subject is
    /// `REQUIRED` — where a check is added — rather than one module's exports,
    /// and the two hand lists are what turn "not on the story grain" from an
    /// omission into a decision with a reason attached.
    #[test]
    fn the_unconditional_block_runs_every_file_reading_check_in_the_gate() {
        // The membership of the exclusion list, pinned. `RUNS_A_PACKAGE` needs
        // no pin — a name added there is reconciled against what the module
        // does, and an excuse for a module that starts no process is reported
        // as stale — but nothing outside this line can contradict an entry
        // here. Appending one is how this check is switched off a member at a
        // time, and the review that raised it routed the question deliberately:
        // whether `lint-constitution` belongs on the story grain is a decision
        // with a written rationale, so a *second* name is a second such
        // decision and not an append nobody reads.
        assert_eq!(
            OFF_THE_STORY_GRAIN
                .iter()
                .map(|(name, _)| *name)
                .collect::<Vec<_>>(),
            ["lint-constitution"],
            "a name has joined or left the exclusion list. Supersede the argument it \
             carries — this is the line that says so — rather than editing it here to \
             match what the block happens to run."
        );

        let main_rs = main_source();
        let subcommands = required_subcommands();

        assert!(
            subcommands.contains(&"lint-rule-counts") && subcommands.contains(&"lint-pages"),
            "the step-table scan no longer sees the gate's own entry points, so it sees \
             nothing and would pass over anything; it saw {subcommands:?}"
        );

        let (family, mut problems) = file_reading_family(&subcommands, &main_rs, &module_source);

        assert!(
            family.len() >= 8,
            "the family derived to {} member(s), fewer than the gate has: {family:?}",
            family.len()
        );

        problems.extend(unwired(&family, &main_rs, &unconditional_block()));

        assert!(problems.is_empty(), "{problems:#?}");
    }

    /// The wrong implementation this pair exists to reject, frozen: a
    /// file-reading check added the way this repository actually adds them.
    ///
    /// A module `lint_zzz` that reads a document and starts nothing, a step in
    /// `REQUIRED` naming it, a dispatch arm — and no call from [`run`]. Both
    /// export-scanning checks above are green over exactly that, because
    /// `lints.rs` exports nothing new, and so is every other test in this file.
    /// It was run as a live mutation of the tree before it was frozen here.
    #[test]
    fn a_file_reading_check_added_as_a_module_is_not_silently_dropped() {
        let main_rs = format!(
            "{}\n        Some(\"lint-zzz\") => lint_zzz::run(),\n",
            main_source()
        );
        let mut subcommands = required_subcommands();
        subcommands.push("lint-zzz");

        let source_of = |module: &str| {
            if module == "lint_zzz" {
                // The probe's whole body: one file read, no `Command`.
                Some("pub(crate) fn run() { fs::read_to_string(page); }".to_owned())
            } else {
                module_source(module)
            }
        };

        let (family, problems) = file_reading_family(&subcommands, &main_rs, &source_of);

        assert!(
            problems.is_empty(),
            "the mutation must classify rather than error: {problems:#?}"
        );
        assert!(
            family.contains(&"lint-zzz"),
            "a module that reads a document and starts no process is a file-reading check"
        );

        let found = unwired(&family, &main_rs, &unconditional_block());

        assert_eq!(
            found.len(),
            1,
            "exactly the mutation and nothing else: {found:#?}"
        );
        assert!(found[0].contains("lint-zzz"), "{}", found[0]);
        assert!(found[0].contains("crate::lint_zzz::run"), "{}", found[0]);
    }

    /// `cargo xtask lints`, held to the same family.
    ///
    /// [`print_help`] calls that command *"every step that reads a document
    /// rather than compiling a package"* and prints its rows from this
    /// selection, so the sentence is true only if the selection is the family.
    /// It named nine of eleven: `spec-trace` and `lint-core-alloc-features`
    /// both read a file, compile nothing, and were reachable through
    /// `cargo xtask lints` only by typing their own names.
    #[test]
    fn cargo_xtask_lints_runs_every_file_reading_check_in_the_gate() {
        let main_rs = main_source();
        let (family, mut problems) =
            file_reading_family(&required_subcommands(), &main_rs, &module_source);

        let mut selected: Vec<&'static str> = Vec::new();
        for step in crate::lint_steps() {
            match xtask_subcommand(step) {
                Some(subcommand) => selected.push(subcommand),
                None => problems.push(format!(
                    "`cargo xtask lints` selects `{}`, which is not one of this binary's own \
                     entry points — so it compiles a package rather than reading a document",
                    step.name
                )),
            }
        }

        for subcommand in &family {
            if !selected.contains(subcommand) {
                problems.push(format!(
                    "`{subcommand}` reads a document and compiles nothing, and \
                     `cargo xtask lints` does not run it: add it to `lint_steps`, or stop \
                     calling that command every step that reads a document"
                ));
            }
        }
        for subcommand in &selected {
            if !family.contains(subcommand) {
                problems.push(format!(
                    "`cargo xtask lints` runs `{subcommand}`, which is not a file-reading \
                     check of the gate"
                ));
            }
        }

        assert!(problems.is_empty(), "{problems:#?}");
    }
}
