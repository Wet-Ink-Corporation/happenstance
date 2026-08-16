//! The manifest is part of the surface, so it is read like one.
//!
//! A feature that exists in code but not in `Cargo.toml` is unmounted, and a
//! feature that subtracts is a difference nobody discovers until somebody
//! else's build turns it on. Neither is visible to a compiler; both are visible
//! here.

use std::path::Path;

/// This crate's own manifest.
const MANIFEST: &str = include_str!("../Cargo.toml");

/// The named section's lines, comments and blanks dropped.
fn section(manifest: &str, heading: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut inside = false;
    for line in manifest.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('[') {
            inside = trimmed == heading;
            continue;
        }
        if !inside || trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        out.push(trimmed.to_owned());
    }
    assert!(!out.is_empty(), "the manifest has no `{heading}` section");
    out
}

/// `name = [...]` for one feature, or `None` when the feature does not exist.
fn feature(name: &str) -> Option<String> {
    section(MANIFEST, "[features]")
        .into_iter()
        .find_map(|line| {
            let (key, value) = line.split_once('=')?;
            (key.trim() == name).then(|| value.trim().to_owned())
        })
}

fn feature_or_panic(name: &str) -> String {
    feature(name).unwrap_or_else(|| panic!("the manifest declares no `{name}` feature"))
}

// ---------------------------------------------------------------------------
// AC-003 — every switch means what it means to the contract crate
// ---------------------------------------------------------------------------

#[test]
fn features_forward_and_only_add() {
    // Every feature this crate shared with the contract crate before this
    // story still forwards to it, unchanged in meaning.
    for (name, forwarded) in [
        ("std", "happenstance-core/std"),
        ("serde", "happenstance-core/serde"),
        ("memory", "happenstance-core/memory"),
    ] {
        let value = feature_or_panic(name);
        assert!(
            value.contains(forwarded),
            "`{name}` no longer forwards `{forwarded}`: {value}"
        );
    }

    // `serde` names the contract crate's wire-format derives and nothing else.
    // Repurposing it into a switch over this crate's own `serde` dependency
    // would change what an existing flag means, which is the one thing a
    // feature may never do.
    assert_eq!(
        feature_or_panic("serde"),
        r#"["happenstance-core/serde"]"#,
        "`serde` grew a second meaning"
    );

    // JSON is on by default, so a first program names a domain before it names
    // an encoding.
    let default = feature_or_panic("default");
    for expected in ["\"std\"", "\"memory\"", "\"json\""] {
        assert!(
            default.contains(expected),
            "`default` no longer contains {expected}: {default}"
        );
    }

    // No feature subtracts. A feature whose name spells a removal is switched
    // by a crate you did not write, and `--all-features` always turns it on.
    for line in section(MANIFEST, "[features]") {
        let name = line.split('=').next().unwrap_or_default().trim().to_owned();
        assert!(
            !name.starts_with("no-") && !name.starts_with("no_") && !name.contains("without"),
            "`{name}` reads as a subtractive feature"
        );
    }

    // Every optional dependency is reached with `dep:`. A bare `crate/feature`
    // inside a feature silently enables the dependency as a side effect.
    for crate_name in ["serde_json", "postcard", "ciborium"] {
        for line in section(MANIFEST, "[features]") {
            let bare = format!("\"{crate_name}/");
            assert!(
                !line.contains(&bare),
                "`{line}` reaches `{crate_name}` without `dep:` or `?/`"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// AC-008 — CBOR ships on a licence verdict, or is declined with the reason
// ---------------------------------------------------------------------------

#[test]
fn cbor_is_shipped_or_declined_with_a_reason() {
    if let Some(value) = feature("cbor") {
        assert!(
            value.contains("dep:"),
            "`cbor` does not name its optional dependency with `dep:`: {value}"
        );
        let dependencies = section(MANIFEST, "[dependencies]");
        assert!(
            dependencies
                .iter()
                .any(|line| line.starts_with("ciborium") && line.contains("optional = true")),
            "`cbor` names a dependency the manifest does not declare as optional"
        );
    } else {
        // Declining is a passing outcome — but only with the reason written
        // down where the next reader will look for it.
        let declined = MANIFEST
            .lines()
            .any(|line| line.contains('#') && line.contains("cbor") && line.contains("deny"));
        assert!(
            declined,
            "no `cbor` feature and no recorded reason beside the feature block"
        );
    }

    // Either way, the allowlist was read, never widened.
    let deny = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../deny.toml")
        .canonicalize()
        .expect("the workspace's deny.toml is where it has always been");
    let deny = std::fs::read_to_string(deny).expect("deny.toml is readable");
    assert!(
        !deny.contains("CDLA") && !deny.contains("GPL"),
        "the licence allowlist was widened to make a crate fit"
    );
}

// ---------------------------------------------------------------------------
// AC-010 — the gate badge exists before there is anything to badge
// ---------------------------------------------------------------------------

#[test]
fn docs_rs_metadata_is_declared() {
    let docs = section(MANIFEST, "[package.metadata.docs.rs]");
    assert!(
        docs.iter().any(|line| line == "all-features = true"),
        "docs.rs would render only the default features"
    );
    assert!(
        docs.iter()
            .any(|line| line.contains("--cfg") && line.contains("docsrs")),
        "docs.rs would render no gate badge at all"
    );

    let root = std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("src/lib.rs"))
        .expect("the crate root is readable");
    assert!(
        root.contains("#![cfg_attr(docsrs, feature(doc_cfg))]"),
        "`--cfg docsrs` turns nothing on without the crate-root attribute"
    );
}

// ---------------------------------------------------------------------------
// AC-008 — the instability is something a reader typed, not something they hit
// ---------------------------------------------------------------------------

#[test]
fn unstable_projection_is_declared_off_by_default() {
    let value = feature_or_panic("unstable-projection");

    // Off by default. The whole point of the name is that a reader performs an
    // act that spells the instability before the item is in their build.
    let default = feature_or_panic("default");
    assert!(
        !default.contains("unstable-projection"),
        "`unstable-projection` joined the defaults: {default}"
    );

    // It forwards to the contract crate's own gate, or says in the manifest why
    // it does not. Either way the decision is written down where the next
    // reader looks for it.
    let forwards = value.contains("happenstance-core/unstable-projection");
    let stated = MANIFEST
        .lines()
        .any(|line| line.trim_start().starts_with('#') && line.contains("unstable-projection"));
    assert!(
        forwards && stated,
        "`unstable-projection` neither forwards to the contract crate nor \
         states in a comment why it does not: {value}"
    );

    // It only adds. A feature that names a removal is one `--all-features`
    // always turns on.
    assert!(
        !value.contains("no-") && !value.contains("without"),
        "`unstable-projection` reads as a subtractive feature: {value}"
    );

    // And the crate really gates something on it — a feature in the manifest
    // that gates no item is a promise nothing keeps.
    let root = std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("src/lib.rs"))
        .expect("the crate root is readable");
    assert!(
        root.contains("#[cfg(feature = \"unstable-projection\")]"),
        "no item at the crate root is gated on `unstable-projection`"
    );
}

// ---------------------------------------------------------------------------
// The workspace pin, and the header a shipped codec moves a crate out of
// ---------------------------------------------------------------------------

#[test]
fn a_shipped_codec_is_not_a_dev_only_dependency() {
    let workspace =
        std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../Cargo.toml"))
            .expect("the workspace manifest is readable");
    let dev_only = workspace
        .find("# --- dev / tooling only ---")
        .expect("the workspace manifest still separates dev-only pins");

    for (crate_name, gate) in [
        ("serde_json", "json"),
        ("postcard", "postcard"),
        ("ciborium", "cbor"),
    ] {
        if feature(gate).is_none() {
            continue;
        }
        let at = workspace
            .find(&format!("\n{crate_name} = "))
            .unwrap_or_else(|| {
                panic!("`{gate}` ships but `{crate_name}` is not pinned in the workspace")
            });
        assert!(
            at < dev_only,
            "`{crate_name}` backs a shipped codec and is still pinned under `dev / tooling only`"
        );
    }
}
