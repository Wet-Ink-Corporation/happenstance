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
//! the API semver-binding, and the whole plan in `docs/RUNBOOK.md` is built
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
[`docs/architecture/SPECIFICATION.md`]({REPOSITORY}/blob/main/docs/architecture/SPECIFICATION.md),
where every clause carries what would falsify it, and the plan to finish it is in
[`docs/RUNBOOK.md`]({REPOSITORY}/blob/main/docs/RUNBOOK.md).

The first functional release will be `0.1.0-alpha.1`. Until then there is nothing
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
//! will be `0.1.0-alpha.1`; this version exists so that the name matches the
//! project already using it in public, and it deliberately offers nothing to
//! depend on.

#![forbid(unsafe_code)]

/// What this version is, in a form `cargo doc` will show.
pub const STATUS: &str = "placeholder: no functionality at 0.0.0";
"#,
        name = c.name,
    )
}
