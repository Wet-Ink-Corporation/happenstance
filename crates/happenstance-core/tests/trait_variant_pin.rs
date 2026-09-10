//! `trait-variant` is still the release whose expansion was actually read.
//!
//! # Why this is here and not beside the position assertion it completes
//!
//! It reads the workspace `Cargo.lock`. `include_str!` resolves against the
//! including file's directory, so from `src/store.rs` that path reaches OUTSIDE
//! this package — and every file under `src/` ships in the `.crate`, where no
//! `exclude` can reach it, because the file is the library.
//!
//! Nothing caught it: `cargo package --list` never compiles, and the
//! verification build compiles the library, not its `#[cfg(test)]` blocks. The
//! tarball built cleanly while carrying a test that cannot, and whoever runs
//! `cargo test` on an unpacked crate is the one who finds out.
//!
//! `store.rs` keeps the positional half — the two attributes sitting where the
//! derivation copies them from — and points here for this half.

/// The workspace lockfile, so a positional claim can be tied to a version.
const LOCKFILE: &str = include_str!("../../../Cargo.lock");

/// The `trait-variant` release whose expansion was actually read.
///
/// Bumping this constant is not a chore. It is the signal to re-open
/// `variant.rs` and confirm the copying still happens, because nothing else in
/// this repository can see it.
const TRAIT_VARIANT_VERIFIED: &str = "0.1.3";

/// The version `Cargo.lock` resolves `trait-variant` to.
///
/// Parsed rather than pinned in the manifest: `trait-variant = "0.1.3"` is a
/// caret requirement, so `0.1.4` would resolve without the manifest changing.
/// The lockfile is what the gate builds against (`--locked`), so it is the only
/// place the *resolved* version can be read.
fn resolved_trait_variant() -> &'static str {
    LOCKFILE
        .split("name = \"trait-variant\"")
        .nth(1)
        .and_then(|rest| rest.split("version = \"").nth(1))
        .and_then(|rest| rest.split('\"').next())
        .expect("the workspace lockfile resolves the derivation's crate")
}

/// `SendEventStore` carries the search keys only because this release copies them.
///
/// `store.rs` asserts the two attributes sit immediately above the derivation.
/// That is necessary and not sufficient: the copying itself is upstream
/// behaviour, and a `trait-variant` release that stopped doing it would leave
/// that assertion green and `SendEventStore` carrying no search key at all.
/// Nothing here can observe the expansion, so the resolved version stands in for
/// it — the gate builds `--locked`, so it cannot move without someone changing
/// it deliberately, and that is the moment to re-read `variant.rs`.
#[test]
fn the_derivation_still_copies_attributes_at_the_resolved_version() {
    assert_eq!(
        resolved_trait_variant(),
        TRAIT_VARIANT_VERIFIED,
        "the two attributes `store.rs` pins reach `SendEventStore` only because \
         trait-variant {TRAIT_VARIANT_VERIFIED} rebuilds the derived trait with \
         `..tr.clone()` (`trait-variant-{TRAIT_VARIANT_VERIFIED}/src/variant.rs:115-123`), \
         copying the base trait's attributes onto it. The position asserted there \
         cannot see that, so the version stands in for it: read the new \
         `variant.rs`, confirm the attributes are still copied, then move this \
         constant"
    );
}
