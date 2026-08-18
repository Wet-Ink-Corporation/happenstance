//! Generates a `0.0.0` placeholder crate for reserving a name on crates.io.
//!
//! # Why this exists as a command rather than as a note in the runbook
//!
//! Names are reserved one at a time, as each phase starts — seven more after the
//! first three — and the gap between them is measured in weeks. A procedure run
//! that rarely, from memory, is a procedure that drifts: the licence files get
//! forgotten once, and the crate that carries the project's first impression
//! ships without them, permanently, because a version can be yanked and never
//! removed.
//!
//! # Why the placeholder is standalone rather than the real crate
//!
//! Two reasons, and the second is the one that matters.
//!
//! Publishing the real crates at `0.0.0` does not work: `happenstance-testkit`
//! depends on `happenstance-core` at the workspace version, so dropping one to
//! `0.0.0` cascades into every manifest that names it, and a registry publish
//! resolves those requirements against crates.io rather than against the path.
//!
//! Publishing the real crates at their actual version is worse. `0.1.0` makes
//! the API semver-binding, and the whole plan in `RUNBOOK.md` is built
//! around freezing the contract *deliberately*, at phases 4 through 6, against
//! evidence. Reserving a name is not a reason to freeze an API, and a
//! reservation that quietly does so has cost more than it bought.
//!
//! So the placeholder shares nothing with the workspace but its metadata, says
//! plainly that it contains no functionality, and points at the specification
//! and the runbook — which is what makes it a reservation with a stated purpose
//! rather than the parked name crates.io's policy prohibits.

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};

/// The repository every placeholder points at.
const REPOSITORY: &str = "https://github.com/Wet-Ink-Corporation/happenstance";

/// A crate whose name this project intends to hold, and what it will be.
///
/// The description is the one the real crate will carry, so that the placeholder
/// and the eventual release describe the same thing to anyone browsing.
struct Reservable {
    /// The crates.io name.
    name: &'static str,
    /// `description` in the manifest, and what crates.io shows in search.
    description: &'static str,
    /// A noun phrase completing "`<name>` is …", for the README body.
    blurb: &'static str,
    /// The runbook phase that claims it. Recorded so a reservation is always
    /// traceable to the work that justifies it.
    phase: u8,
}

/// Every name this project intends to hold, and when.
///
/// Reserved one per phase rather than all at once. crates.io's policy prohibits
/// a crate that "exists only to reserve a name for a prolonged period of time …
/// without having any genuine functionality, purpose, or significant development
/// activity on the corresponding repository", and a placeholder claimed at the
/// start of the phase that builds it always has an answer to that.
///
/// It costs nothing to wait, either: crates.io has no prefix reservation, so
/// owning `happenstance` protects none of the rest. The exposure is identical
/// whenever they are claimed.
const RESERVABLE: &[Reservable] = &[
    Reservable {
        name: "happenstance",
        description: "DCB-compliant event sourcing with batteries: typed events, decision models and projection runners over a storage-agnostic contract.",
        blurb: "the crate an application programs against",
        phase: 0,
    },
    Reservable {
        name: "happenstance-core",
        description: "DCB-compliant event sourcing contract: types, storage ports, and an in-memory reference event store.",
        blurb: "the contract: value types, the storage ports, and an in-memory reference event store",
        phase: 0,
    },
    Reservable {
        name: "happenstance-testkit",
        description: "DCB conformance suite for happenstance event store adapters.",
        blurb: "the conformance suite every event store adapter is measured against",
        phase: 0,
    },
    Reservable {
        name: "happenstance-sqlite",
        description: "SQLite event store and projection store adapters for happenstance.",
        blurb: "the SQLite event store and projection store adapters",
        phase: 8,
    },
    Reservable {
        name: "happenstance-cloudflare",
        description: "Cloudflare Durable Object event store adapter for happenstance.",
        blurb: "the Cloudflare Durable Object event store adapter, for Workers",
        phase: 9,
    },
    Reservable {
        name: "happenstance-postgres",
        description: "PostgreSQL event store and projection store adapters for happenstance.",
        blurb: "the PostgreSQL event store and projection store adapters",
        phase: 10,
    },
    Reservable {
        name: "happenstance-neon",
        description: "Neon serverless PostgreSQL event store adapter for happenstance, over one-shot HTTP.",
        blurb: "the Neon adapter, reaching PostgreSQL over one-shot HTTP with no interactive transaction",
        phase: 10,
    },
    Reservable {
        name: "happenstance-ladybug",
        description: "LadybugDB graph projection store adapter for happenstance.",
        blurb: "the LadybugDB graph projection store adapter",
        phase: 11,
    },
    Reservable {
        name: "happenstance-sync",
        description: "Instance-to-instance event replication for happenstance: the peer port and its runner.",
        blurb: "the replication port — peers, ingest and the runner above them",
        phase: 13,
    },
    Reservable {
        name: "happenstance-sync-testkit",
        description: "Conformance suite for happenstance replication peers.",
        blurb: "the conformance suite every replication peer is measured against",
        phase: 13,
    },
];

/// Builds the placeholder for `name` under `target/reserve/`.
///
/// # Errors
///
/// Returns an error if `name` is not one of the names this project intends to
/// hold, if the repository's licence files cannot be read, or if the generated
/// crate cannot be written.
pub(crate) fn run(name: Option<&str>) -> Result<()> {
    let Some(name) = name else {
        list();
        return Ok(());
    };

    let Some(crate_) = RESERVABLE.iter().find(|c| c.name == name) else {
        list();
        bail!(
            "`{name}` is not a name this project intends to hold — add it to RESERVABLE first, with the phase that claims it"
        );
    };

    let root = workspace_root()?;
    let out = root.join("target").join("reserve").join(crate_.name);
    if out.exists() {
        fs::remove_dir_all(&out)
            .with_context(|| format!("removing the previous {}", out.display()))?;
    }
    fs::create_dir_all(out.join("src")).with_context(|| format!("creating {}", out.display()))?;

    // Copied, not linked. Cargo packages only what is inside the crate
    // directory, which is exactly the defect (D11) that shipped licence
    // metadata with no licence text.
    for licence in ["LICENSE-MIT", "LICENSE-APACHE"] {
        let from = root.join(licence);
        fs::copy(&from, out.join(licence))
            .with_context(|| format!("copying {}", from.display()))?;
    }

    fs::write(out.join("Cargo.toml"), manifest(crate_))?;
    fs::write(out.join("README.md"), readme(crate_))?;
    fs::write(out.join("src").join("lib.rs"), lib_rs(crate_))?;

    println!("Placeholder for `{}` written to:", crate_.name);
    println!("  {}", out.display());
    println!();
    let manifest_path = out.join("Cargo.toml");
    println!("Verify, which packages and compiles but does not upload:");
    println!(
        "  cargo publish --manifest-path {} --dry-run",
        manifest_path.display()
    );
    println!();
    println!("Then claim it. This is irreversible — a version can be yanked, never removed:");
    println!(
        "  cargo publish --manifest-path {}",
        manifest_path.display()
    );
    println!();
    println!("The token lives in $CARGO_HOME/credentials.toml, put there by `cargo login`.");
    println!("Never commit it; .gitignore already covers the repository-local path.");

    Ok(())
}

/// Prints every name and the phase that claims it.
fn list() {
    println!("Names this project intends to hold, and the phase that claims each:");
    println!();
    for c in RESERVABLE {
        println!("  phase {:>2}   {}", c.phase, c.name);
    }
    println!();
    println!("Usage: cargo xtask reserve <name>");
}

/// The repository root, derived from this crate's manifest rather than from the
/// working directory, so the command behaves the same wherever it is run.
fn workspace_root() -> Result<PathBuf> {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
    manifest
        .parent()
        .map(Path::to_path_buf)
        .context("xtask must live one level below the workspace root")
}

fn manifest(c: &Reservable) -> String {
    format!(
        r#"[package]
name = "{name}"
version = "0.0.0"
description = "{description}"
edition = "2021"
rust-version = "1.85"
license = "MIT OR Apache-2.0"
repository = "{REPOSITORY}"
authors = ["Ryan Britton"]
keywords = ["event-sourcing", "dcb", "cqrs", "event-store"]
categories = ["database", "data-structures"]
readme = "README.md"

# Generated under target/, which is inside the workspace directory, so Cargo
# would otherwise refuse to build it as a non-member. An empty table makes it
# its own workspace — which is also the honest description: it shares nothing
# with this repository but its metadata.
[workspace]

[dependencies]
"#,
        name = c.name,
        description = c.description,
    )
}

fn readme(c: &Reservable) -> String {
    format!(
        r"# {name}

**This `0.0.0` is a placeholder. It contains no functionality.**

`{name}` is {blurb}, part of
[happenstance]({REPOSITORY}) — an opinionated, storage-agnostic event sourcing
library for Rust built on the
[Dynamic Consistency Boundary specification](https://dcb.events/specification/).

The work is public and active. The design is specified clause by clause in
[`spec/SPECIFICATION.md`]({REPOSITORY}/blob/main/spec/SPECIFICATION.md),
where every clause carries what would falsify it, and the plan to finish it is in
[`RUNBOOK.md`]({REPOSITORY}/blob/main/RUNBOOK.md).

The first functional release will be `0.2.0-alpha.1`. Until then there is nothing
here worth depending on, and this version says so rather than pretending
otherwise.

## Licence

MIT OR Apache-2.0.
",
        name = c.name,
        blurb = c.blurb,
    )
}

fn lib_rs(c: &Reservable) -> String {
    format!(
        r#"//! Placeholder. `{name}` has no functionality at `0.0.0`.
//!
//! See <{REPOSITORY}> for the design and the plan. The first functional release
//! will be `0.2.0-alpha.1`; this version exists so that the name matches the
//! project already using it in public, and it deliberately offers nothing to
//! depend on.

#![forbid(unsafe_code)]

/// What this version is, in a form `cargo doc` will show.
pub const STATUS: &str = "placeholder: no functionality at 0.0.0";
"#,
        name = c.name,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The version this project's first functional release will actually carry.
    ///
    /// It is `0.2.0-alpha.1` rather than `0.1.0-alpha.1` because `0.1.0` was
    /// spent: the workspace already carries `0.2.0-alpha.1` in its own manifests.
    /// The placeholder is published text and a published version can be yanked
    /// but never removed, so the claim is reconciled *before* an upload rather
    /// than corrected after one.
    const FIRST_FUNCTIONAL_RELEASE: &str = "0.2.0-alpha.1";

    /// The version claim the templates used to make, kept here as the string a
    /// regression would reintroduce.
    const SUPERSEDED_RELEASE_CLAIM: &str = "0.1.0-alpha.1";

    /// Every placeholder this command can generate names the release this
    /// project will actually ship.
    ///
    /// Both templates are checked, not one: the README is what crates.io renders
    /// and `lib.rs` is what docs.rs renders, and a reader who is deciding when to
    /// depend on this crate may meet either.
    #[test]
    fn every_placeholder_names_the_release_this_project_will_ship() {
        for c in RESERVABLE {
            for (surface, rendered) in [("README.md", readme(c)), ("src/lib.rs", lib_rs(c))] {
                assert!(
                    !rendered.contains(SUPERSEDED_RELEASE_CLAIM),
                    "the {} placeholder's {surface} still promises {SUPERSEDED_RELEASE_CLAIM}, \
                     which this initiative superseded with {FIRST_FUNCTIONAL_RELEASE}",
                    c.name
                );
                assert!(
                    rendered.contains(FIRST_FUNCTIONAL_RELEASE),
                    "the {} placeholder's {surface} names no first functional release; it must \
                     name {FIRST_FUNCTIONAL_RELEASE}",
                    c.name
                );
            }
        }
    }

    /// The workspace crate directory for `name`, if this workspace has one.
    fn crate_dir(name: &str) -> Option<PathBuf> {
        let dir = workspace_root().ok()?.join("crates").join(name);
        dir.is_dir().then_some(dir)
    }

    /// The value of a top-level `key = "…"` in a manifest, if it has one.
    fn manifest_string(manifest: &str, key: &str) -> Option<String> {
        manifest
            .lines()
            .find_map(|line| line.strip_prefix(&format!("{key} = \"")))
            .and_then(|rest| rest.strip_suffix('"'))
            .map(str::to_owned)
    }

    /// A crate that has a README has begun facing a registry, and everything the
    /// placeholder carries it must carry too.
    ///
    /// The trigger is `README.md`, because that is the file whose presence says
    /// somebody has started writing the crate's front page. From that point the
    /// three facts crates.io renders — the search line, the licence chips and the
    /// page itself — must all be backed: the description must be the one the
    /// placeholder published, so the reservation and the release describe the
    /// same thing, and both licence texts must sit **inside** the package
    /// directory, because Cargo will not follow a path outside it and does not
    /// warn when the `license` field is backed by nothing.
    ///
    /// That last half is D11, quoted at `package.rs:1-22`: three crates whose
    /// metadata promised two licences neither of which shipped, passing every
    /// check anyone had. This is the check that would have caught it, expressed
    /// where the copying is already done and already commented.
    #[test]
    fn a_crate_with_a_readme_carries_what_the_placeholder_carries() {
        let root = workspace_root().expect("the workspace root is derivable from this manifest");

        for c in RESERVABLE {
            let Some(dir) = crate_dir(c.name) else {
                continue;
            };
            if !dir.join("README.md").is_file() {
                continue;
            }

            let manifest = fs::read_to_string(dir.join("Cargo.toml"))
                .unwrap_or_else(|e| panic!("reading {}'s manifest: {e}", c.name));
            assert_eq!(
                manifest_string(&manifest, "description").as_deref(),
                Some(c.description),
                "{}'s manifest describes it differently from the placeholder this command \
                 publishes under the same name; the reservation and the release must describe \
                 the same thing",
                c.name
            );

            for licence in ["LICENSE-MIT", "LICENSE-APACHE"] {
                let inside = dir.join(licence);
                assert!(
                    inside.is_file(),
                    "{}/{licence} is missing: the manifest offers a choice of two licences and \
                     the package would carry neither",
                    c.name
                );
                assert_eq!(
                    fs::read(&inside).expect("the crate's licence text is readable"),
                    fs::read(root.join(licence)).expect("the root licence text is readable"),
                    "{}/{licence} is not byte-identical to the workspace root's; divergent \
                     licence text across one workspace is a legal defect, not a formatting nit",
                    c.name
                );
            }
        }
    }

    /// A README fence is a promise that the example compiles, unless the crate
    /// compiles its own README as a doctest.
    ///
    /// `crates/happenstance/src/lib.rs` takes the first route — `include_str!`
    /// under `cfg(doctest)` — so the unqualified Rust fences in its README are
    /// checked by `cargo test`. A crate that does not do that has nothing
    /// compiling its README, and an unqualified fence there is an example
    /// advertised and never built. The house answer is `rust,ignore`, chosen
    /// deliberately rather than by omission.
    #[test]
    fn no_readme_advertises_an_example_nothing_compiles() {
        for c in RESERVABLE {
            let Some(dir) = crate_dir(c.name) else {
                continue;
            };
            let Ok(readme) = fs::read_to_string(dir.join("README.md")) else {
                continue;
            };
            let doctested = fs::read_to_string(dir.join("src").join("lib.rs"))
                .is_ok_and(|lib| lib.contains("include_str!(\"../README.md\")"));
            if doctested {
                continue;
            }

            for (number, line) in readme.lines().enumerate() {
                assert!(
                    line.trim_end() != "```rust",
                    "{}/README.md:{} opens a bare ```rust fence, but nothing compiles this \
                     README; mark it `rust,ignore` or make the crate doctest its own README",
                    c.name,
                    number + 1
                );
            }
        }
    }
}
