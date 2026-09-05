//! The manifest is part of what this crate falsifies, so something has to read
//! it.
//!
//! The crate's own opening paragraph claims a geometry in which *"the orphan
//! rule, the feature flags and the dependency graph all behave the way they
//! would for a stranger"*. Two of those three were falsified by the compiler
//! the moment this crate was written: the orphan rule answers `error[E0117]`
//! from `tests/`, and a dev-dependency on the testkit does not exist for the
//! lib build. The third was falsified by nobody. **A manifest is not compiled
//! against a document**, so the one artefact in this repository whose job is to
//! follow the published recipe was free to write a different manifest than the
//! recipe specifies, and did — for two months, green.
//!
//! This target is the missing half. It reads the recipe out of the rendered
//! source it is published from and asserts that this crate's manifest carries
//! the line the recipe prescribes.
//!
//! # Why it is here and not in `xtask`
//!
//! Because the claim is *this crate's*. `xtask` owns the checks that hold the
//! whole workspace to a shape; this one holds a single crate to a sentence it
//! wrote about itself, and a workspace-wide lint would have to invent the rule
//! that every crate implementing `ProjectionProbe` forwards the feature —
//! which is true of `happenstance-sqlite` and is not something this lane is
//! entitled to legislate. The narrower check is the honest one.
//!
//! # What it does not check, stated because a green here is read as coverage
//!
//! * **The `[dependencies]` half of the recipe.** This bullet used to say the
//!   recipe's fence wrote `happenstance-core = "…"` with no features named, so
//!   that asserting its dependency line here *"would pin a line that is
//!   wrong"*. That stopped being true at `48d4cd5`: the fence now writes
//!   `features = ["unstable-projection"]`, and
//!   `crates/happenstance-core/tests/projection_recipe.rs` holds it there
//!   against the gates `lib.rs` actually carries. The half this target still
//!   does not check is the same one for a different reason — the requirement is
//!   the port's own to state and its instrument lives beside the port, not in
//!   an example.
//! * **That the manifest is the manifest a stranger writes.** It carries a
//!   `path` this crate needs and a stranger does not, and the version
//!   requirement moves with the workspace. Both are documented departures in
//!   `Cargo.toml`'s own comments, and neither is mechanical.
//! * **That the feature does anything.** `cargo test --workspace
//!   --all-features` is what compiles the gated impls and runs the suite
//!   through them. This target only asserts the flag is declared and forwards.

use std::fs;
use std::path::{Path, PathBuf};

/// The forwarding line the recipe prescribes, verbatim.
///
/// One `const` read by both assertions below, so the check cannot pass by
/// comparing this crate's manifest against a string that is no longer in the
/// recipe. If the port renames the feature, both halves go red together and the
/// failure names the recipe rather than this file.
const FORWARDING: &str = r#"conformance = ["happenstance-core/conformance"]"#;

/// Where the recipe is published from, relative to this crate.
///
/// A path out of the workspace rather than a copy, deliberately: a copy is the
/// two-documents-one-truth failure this whole target exists to close, one level
/// up. The crate is `publish = false` and stays that way, so reaching across
/// the workspace costs nothing a consumer ever pays.
const RECIPE: &str = "../../crates/happenstance-core/src/projection.rs";

fn workspace_file(rel: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(rel)
}

fn read(rel: &str) -> String {
    let path = workspace_file(rel);
    fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("{} is part of this check: {error}", path.display()))
}

/// The recipe still says what this crate is being held to.
///
/// Asserted first, and separately, because the interesting failure is not
/// "this crate drifted" but "the recipe moved and nothing noticed". A single
/// combined assertion would report both as the same problem and send the reader
/// to the wrong file.
#[test]
fn the_published_recipe_still_prescribes_the_forwarding_feature() {
    let recipe = read(RECIPE);

    assert!(
        recipe.contains(FORWARDING),
        "the projection port's rustdoc no longer publishes `{FORWARDING}`. Either \
         the recipe changed and this crate has to follow it, or the feature was \
         renamed and this file is the second place that has to say so — it is \
         not a licence to delete the check"
    );
}

/// This crate's manifest carries the line the recipe prescribes.
///
/// **The wrong implementation this rejects is the one that shipped.** Until
/// this test existed the manifest wrote `features = ["conformance"]` inside
/// `[dependencies]` and declared no features of its own, so `unstable-projection`
/// — which `conformance` implies — was on unconditionally for anything that
/// copied it. That is exactly the hazard
/// `.kb/open-questions/projection-store-in-adapter-default-features.md` records
/// as accepted and open for two other crates, published from the crate whose
/// job is to be the reference a stranger copies.
#[test]
fn this_crates_manifest_writes_the_manifest_the_recipe_specifies() {
    let manifest = read("Cargo.toml");

    assert!(
        manifest.contains("[features]"),
        "the recipe prescribes a `[features]` table and this manifest has none. \
         A crate that turns `conformance` on inside `[dependencies]` publishes \
         `unstable-projection` to every application in its graph, and this crate \
         is the one an outside author is pointed at"
    );
    assert!(
        manifest.contains(FORWARDING),
        "this manifest does not carry `{FORWARDING}`, which is the line the \
         projection port's own rustdoc tells an adapter author to write"
    );
}

/// The feature is forwarded rather than enabled, which is the whole distinction.
///
/// A manifest can satisfy the two assertions above and still enable
/// `conformance` on the dependency directly, at which point the `[features]`
/// table is decorative and the graph is exactly as wide as before. This is the
/// assertion that tells those two states apart.
#[test]
fn the_dependency_does_not_turn_conformance_on_by_itself() {
    let manifest = read("Cargo.toml");

    // Comments are stripped before the split, and that is not tidiness: the
    // `[features]` table's own rationale comment sits *above* the header and
    // names `#[cfg(feature = "conformance")]`, so a naive split on the header
    // puts that sentence in the dependency half and this assertion fails on
    // prose. The first version of this test did exactly that.
    let uncommented: String = manifest
        .lines()
        .filter(|line| !line.trim_start().starts_with('#'))
        .collect::<Vec<_>>()
        .join("\n");
    let dependencies = uncommented
        .split("[features]")
        .next()
        .expect("split always yields at least one part");

    assert!(
        !dependencies.contains("\"conformance\""),
        "`conformance` is enabled on the dependency itself, so the `[features]` \
         table forwards a flag that was already on. The point of the recipe is \
         that an adapter's consumer chooses"
    );
}
