//! Each publishable crate's front page states the MSRV policy (#201, ADR-0067).
//!
//! ADR-0067 bounds a floor rise after `1.0.0` — a minor only, to a stable at
//! least six months old, with a `CHANGELOG.md` entry — and phase 21 asks that
//! every crate say so where a consumer reads first: the README crates.io
//! renders. This holds each README to two facts the tree knows. The number must
//! be the floor the crate's manifest resolves to, so a rise that moves
//! `Cargo.toml` and leaves a front page at the old number fails here rather than
//! on the registry. And the section must exist on every name in `PUBLISHABLE`,
//! so a crate that joins the release set without one fails too.
//!
//! # What this does not verify
//!
//! That the prose is *true*. It checks for the floor in bold, a link naming
//! ADR-0067 and the spelling `resolver = "3"` inside the section; it does not
//! read the sentence around them. A README that repeats an old floor *outside*
//! the section passes, because the section is the statement and everything
//! else is history the page may tell. A `# ` line inside a code fence in the
//! section would end it early, so the section carries no fences.
//!
//! # Why it is wired the way it is
//!
//! Through `stated_rule_counts`'s existing `stated_crate_counts()?` line, as an
//! `and_then` on the same line, for the two reasons that function's comments
//! give for every check bundled there: `lints.rs` is cited by line number, so a
//! new call line would move those citations, and a new `pub(crate)` entry point
//! there obligates `affected.rs`'s export scan. It sits beside the crate-count
//! check because it is the same direction of check over the same pages — a
//! document held to the release set.

use std::fs;
use std::io::{self, Write as _};
use std::path::Path;

use anyhow::{Context, Result, bail};

use super::publishable_from_package_rs;
use crate::spec_trace::workspace_root;

/// The heading every publishable crate's README carries.
const HEADING: &str = "## Minimum supported Rust version";

/// The decision record the section must name.
const POLICY: &str = "ADR-0067";

/// The resolver setting the promise rests on, as the section must spell it.
const RESOLVER: &str = "resolver = \"3\"";

/// The workspace manifest, whose `[workspace.package]` carries the floor.
const WORKSPACE_MANIFEST: &str = "Cargo.toml";

/// Where a manifest's `rust-version` comes from.
#[derive(Debug, PartialEq, Eq)]
enum RustVersion<'a> {
    /// `rust-version.workspace = true`.
    Workspace,
    /// `rust-version = "…"`, the quoted value.
    Literal(&'a str),
}

/// The first uncommented `rust-version` key in `manifest`, or `None`.
///
/// A line scan rather than a TOML parse, because `xtask` has no TOML parser
/// and the key is written one way in every manifest here. Lines opening in `#`
/// are skipped: the workspace manifest discusses other crates' `rust-version`
/// values in comments.
fn rust_version(manifest: &str) -> Option<RustVersion<'_>> {
    manifest.lines().map(str::trim).find_map(|line| {
        let rest = line.strip_prefix("rust-version")?;
        if let Some(inherit) = rest.trim_start().strip_prefix(".workspace") {
            let value = inherit.trim_start().strip_prefix('=')?.trim();
            return (value == "true").then_some(RustVersion::Workspace);
        }
        let value = rest.trim_start().strip_prefix('=')?.trim();
        Some(RustVersion::Literal(
            value.strip_prefix('"')?.strip_suffix('"')?,
        ))
    })
}

/// The lines of `readme` under [`HEADING`], up to the next heading of level
/// one or two, joined with newlines; `None` when there is no such heading.
fn section(readme: &str) -> Option<String> {
    let mut lines = readme.lines();
    lines.find(|line| line.trim_end() == HEADING)?;
    let body: Vec<&str> = lines
        .take_while(|line| !line.starts_with("## ") && !line.starts_with("# "))
        .collect();
    Some(body.join("\n"))
}

/// Every way `readme`, the front page of `name`, fails to state `floor` and
/// the policy that bounds it.
fn statement_problems(name: &str, floor: &str, readme: &str) -> Vec<String> {
    let Some(body) = section(readme) else {
        return vec![format!(
            "{name}'s README has no `{HEADING}` section. Every publishable crate states its \
             floor and ADR-0067's rule for raising it on its front page."
        )];
    };
    let mut problems = Vec::new();
    let bold = format!("**{floor}**");
    if !body.contains(&bold) {
        problems.push(format!(
            "{name}'s README: `{HEADING}` does not state the floor its manifest declares, \
             {bold}. A floor rise updates every front page in the same change."
        ));
    }
    if !body.contains(POLICY) {
        problems.push(format!(
            "{name}'s README: `{HEADING}` does not link {POLICY}, the rule a rise follows."
        ));
    }
    if !body.contains(RESOLVER) {
        problems.push(format!(
            "{name}'s README: `{HEADING}` does not name `{RESOLVER}`, the consumer setting \
             the promise rests on."
        ));
    }
    problems
}

/// Every publishable crate's README states its floor and ADR-0067's policy.
///
/// # Errors
///
/// Returns an error if a manifest or README cannot be read, if the workspace
/// manifest declares no literal `rust-version`, if a publishable crate's
/// manifest declares none, or listing every problem [`statement_problems`]
/// finds.
pub(super) fn front_pages_state_the_policy() -> Result<()> {
    let root = workspace_root()?;
    let workspace = read(&root, WORKSPACE_MANIFEST)?;
    let Some(RustVersion::Literal(floor)) = rust_version(&workspace) else {
        bail!("{WORKSPACE_MANIFEST} declares no `rust-version = \"…\"` for its members to inherit");
    };

    let names = publishable_from_package_rs(&root)?;
    let mut problems = Vec::new();
    for name in &names {
        let manifest_path = format!("crates/{name}/Cargo.toml");
        let manifest = read(&root, &manifest_path)?;
        let crate_floor = match rust_version(&manifest) {
            Some(RustVersion::Workspace) => floor,
            Some(RustVersion::Literal(own)) => own,
            None => {
                problems.push(format!("{manifest_path} declares no `rust-version`."));
                continue;
            }
        };
        let readme = read(&root, &format!("crates/{name}/README.md"))?;
        problems.extend(statement_problems(name, crate_floor, &readme));
    }

    if !problems.is_empty() {
        bail!(
            "{} MSRV statement problem(s):\n  {}",
            problems.len(),
            problems.join("\n  ")
        );
    }
    writeln!(
        io::stdout().lock(),
        "front_pages_state_the_policy: {} publishable README(s) state their floor and {POLICY}",
        names.len()
    )?;
    Ok(())
}

/// `rel`, read relative to `root`.
fn read(root: &Path, rel: &str) -> Result<String> {
    fs::read_to_string(root.join(rel)).with_context(|| format!("reading {rel}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The section as every publishable README writes it, with a heading on
    /// each side so the section's edges are exercised.
    const GOOD: &str = "\
# c

## Use

Rust 1.0.

## Minimum supported Rust version

Rust **1.97.1**. After `1.0.0` it rises only as
[ADR-0067](https://example.invalid/0067) bounds it. Under `resolver = \"3\"`
Cargo prefers a compatible version.

## Licence

MIT OR Apache-2.0.
";

    #[test]
    fn a_conforming_front_page_passes() {
        assert_eq!(
            statement_problems("c", "1.97.1", GOOD),
            Vec::<String>::new()
        );
    }

    /// Rejects a check over a fixed list of READMEs: a crate that joins the
    /// release set with no section is reported, not skipped.
    #[test]
    fn a_front_page_without_the_section_is_refused() {
        let readme = "# c\n\n## Use\n\nRust **1.97.1**, ADR-0067, resolver = \"3\".\n";
        let problems = statement_problems("c", "1.97.1", readme);
        assert_eq!(problems.len(), 1, "{problems:?}");
        assert!(problems[0].contains("has no"), "{problems:?}");
    }

    /// Rejects a check that compares the README against a number written into
    /// the check: the floor comes from the manifest, so a rise there fails
    /// every front page still stating the old one.
    #[test]
    fn a_front_page_left_at_the_old_floor_is_refused() {
        let problems = statement_problems("c", "1.98.0", GOOD);
        assert_eq!(problems.len(), 1, "{problems:?}");
        assert!(problems[0].contains("**1.98.0**"), "{problems:?}");
    }

    /// Rejects a whole-file grep: the floor under another heading is not the
    /// statement, and neither is a link that only the next section carries.
    #[test]
    fn what_sits_outside_the_section_does_not_count() {
        let readme =
            GOOD.replace("Rust **1.97.1**. After", "Rust 1.97.1. After") + "\nRust **1.97.1**.\n";
        let problems = statement_problems("c", "1.97.1", &readme);
        assert_eq!(problems.len(), 1, "{problems:?}");
        assert!(
            problems[0].contains("does not state the floor"),
            "{problems:?}"
        );
    }

    /// Rejects a section that states a number and nothing else: the policy and
    /// the resolver sentence are each required on their own.
    #[test]
    fn the_policy_and_the_resolver_are_each_required() {
        let no_policy = GOOD.replace("ADR-0067", "ADR-0029");
        let no_resolver = GOOD.replace("`resolver = \"3\"`", "the new resolver");
        assert_eq!(statement_problems("c", "1.97.1", &no_policy).len(), 1);
        assert_eq!(statement_problems("c", "1.97.1", &no_resolver).len(), 1);
    }

    #[test]
    fn rust_version_reads_a_literal_and_skips_comments() {
        let manifest = "[workspace.package]\n# rust-version = \"1.75\" elsewhere\n\
                        rust-version = \"1.97.1\"\n";
        assert_eq!(rust_version(manifest), Some(RustVersion::Literal("1.97.1")));
    }

    #[test]
    fn rust_version_reads_workspace_inheritance() {
        let manifest = "[package]\nname = \"c\"\nrust-version.workspace = true\n";
        assert_eq!(rust_version(manifest), Some(RustVersion::Workspace));
    }

    #[test]
    fn rust_version_is_none_when_undeclared() {
        assert_eq!(rust_version("[package]\nname = \"c\"\n"), None);
    }
}
