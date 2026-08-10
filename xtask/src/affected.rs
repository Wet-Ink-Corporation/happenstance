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
pub(crate) fn run(base: Option<&str>) -> Result<()> {
    let base = base.unwrap_or("main");
    let root = workspace_root()?;

    // First, and unconditionally. See the module documentation: the packages
    // these read are not the packages the diff touched.
    println!("\n=== the file-reading checks ===");
    crate::spec_trace::retired_rules()?;
    crate::lints::no_clock()?;
    crate::lints::no_position_literals()?;
    crate::lints::changelog_names_every_rule()?;
    crate::lints::testkit_version()?;
    crate::lints::core_alloc_features()?;
    crate::spec_trace::run(crate::spec_trace::Mode::Check)?;

    let members = members(&root)?;
    let changed = changed_files(&root, base)?;

    println!("\n=== affected packages ===");
    println!("{} file(s) changed against `{base}`", changed.len());

    let affected = affected_packages(&changed, &members);

    if affected.is_empty() {
        // Not a silent pass: a docs-only story genuinely has no package to
        // compile, and saying so is the difference between "nothing to do" and
        // "the gate did not look".
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

    let selection: Vec<String> = affected
        .iter()
        .flat_map(|name| ["-p".to_owned(), name.clone()])
        .collect();

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
                // Outside every member. `README.md` is a real dependency of
                // `xtask`, whose lib target compiles it as doctests; the rest —
                // `docs/`, `.github/`, `.bklg/`, `.kb/` — reaches no package.
                // Anything unrecognised widens rather than narrows.
                if path == "README.md" {
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
/// and never coverage. `docs/` is inert to the compiler and **not** to the gate —
/// `SPECIFICATION.md` is read by `spec-trace`, which this module runs on every
/// invocation regardless of what changed.
fn is_inert(path: &str) -> bool {
    const INERT: &[&str] = &[
        "docs/",
        ".github/",
        ".bklg/",
        ".kb/",
        ".redkiln/",
        ".idea/",
        "CHANGELOG.md",
        "CLAUDE.md",
        "CONTRIBUTING.md",
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

    #[test]
    fn a_docs_only_change_selects_nothing() {
        let affected = affected_packages(&changed(&["docs/RUNBOOK.md"]), &members());
        assert!(affected.is_empty());
    }

    /// `README.md` is compiled as doctests of `xtask`'s lib target, so it is
    /// source for exactly one package and inert for every other.
    #[test]
    fn the_readme_selects_xtask() {
        let affected = affected_packages(&changed(&["README.md"]), &members());
        assert_eq!(affected, changed(&["xtask"]));
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
}
