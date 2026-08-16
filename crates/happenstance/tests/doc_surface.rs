//! The rendered codec surface, read off the crate's own source.
//!
//! An unstyled render — bare `pub use`s under a surviving roadmap bullet —
//! satisfies every compile assertion in this crate and fails every one below.
//! No compiler runs here; these are file reads, which is what makes them cheap
//! enough to be unconditional.

use std::path::{Path, PathBuf};

fn src() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("src")
}

fn read(name: &str) -> String {
    std::fs::read_to_string(src().join(name)).expect("the crate's own source is readable")
}

/// The `//!` body of the crate root, line by line.
fn module_doc() -> Vec<String> {
    read("lib.rs")
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim_start();
            trimmed
                .strip_prefix("//!")
                .map(|rest| rest.strip_prefix(' ').unwrap_or(rest).to_owned())
        })
        .collect()
}

fn position_of(doc: &[String], needle: &str) -> usize {
    doc.iter()
        .position(|line| line.contains(needle))
        .unwrap_or_else(|| panic!("the crate root does not render `{needle}`"))
}

// ---------------------------------------------------------------------------
// AC-009 — the roadmap bullet became the real thing, in place
// ---------------------------------------------------------------------------

#[test]
fn crate_root_renders_the_codec_surface() {
    let doc = module_doc();

    // Region 4, and the `Codec` bullet is still its first entry: the vocabulary
    // is rewritten in place, not restructured around the new item.
    let vocabulary = position_of(&doc, "# The vocabulary");
    let bullets: Vec<(usize, &String)> = doc
        .iter()
        .enumerate()
        .filter(|(at, line)| *at > vocabulary && line.starts_with("* "))
        .collect();
    let names: Vec<&str> = bullets
        .iter()
        .filter_map(|(_, line)| {
            [
                "`Codec`",
                "`DomainEvent`",
                "`DecisionModel`",
                "The command loop",
                "The typed projection runner",
            ]
            .into_iter()
            .find(|name| line.contains(name))
        })
        .collect();
    assert_eq!(
        names,
        [
            "`Codec`",
            "`DomainEvent`",
            "`DecisionModel`",
            "The command loop",
            "The typed projection runner",
        ],
        "the vocabulary bullets moved; the rewrite was not in place"
    );

    // The bullet is a link, and the link is the emphasis.
    let codec_bullet = bullets
        .iter()
        .find(|(_, line)| line.contains("`Codec`"))
        .map(|(at, _)| *at)
        .expect("the vocabulary names `Codec`");
    assert!(
        doc[codec_bullet].contains("(Codec)"),
        "the `Codec` bullet is not an intra-doc link: {}",
        doc[codec_bullet]
    );

    // No roadmap survives for what this story landed. The bullet runs until the
    // next one, and nothing in it may still call the codecs planned.
    let next = bullets
        .iter()
        .map(|(at, _)| *at)
        .find(|at| *at > codec_bullet)
        .unwrap_or(doc.len());
    let region = doc[codec_bullet..next].join(" ").to_lowercase();
    assert!(
        !region.contains("planned"),
        "the `Codec` bullet still reads as a roadmap: {region}"
    );

    // Region 5 exists, is below the vocabulary, and is a table rather than
    // prose — recessive by form as well as by position.
    let features = position_of(&doc, "# Features");
    assert!(
        features > vocabulary,
        "the Features region was hoisted above the vocabulary"
    );
    let adapters = position_of(&doc, "Adapter authors should depend on");
    assert!(
        features < adapters,
        "the Features region displaced the adapter-author pointer from last"
    );

    let table = doc[features..adapters].join("\n");
    assert!(
        table.contains("| ---"),
        "the Features region is prose, not a table: {table}"
    );
    for shipped in shipped_features() {
        assert!(
            table.contains(&format!("`{shipped}`")),
            "the Features table says nothing about `{shipped}`"
        );
    }

    // The mount itself: every codec is reachable from the crate root, beside
    // the glob that must survive.
    let root = read("lib.rs");
    assert!(
        root.contains("pub use happenstance_core::*;"),
        "the contract crate's glob re-export must survive"
    );
    for (feature, item) in [("json", "Json"), ("postcard", "Postcard"), ("cbor", "Cbor")] {
        if !shipped_features().contains(&feature.to_owned()) {
            continue;
        }
        assert!(
            root.contains(&format!("pub use codec::{item};"))
                || root.contains(&format!("{item},"))
                || root.contains(&format!("{item}}}")),
            "`{item}` is not re-exported at the crate root"
        );
    }
}

/// The codec features this build of the manifest actually ships.
fn shipped_features() -> Vec<String> {
    let manifest =
        std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml"))
            .expect("the crate's own manifest is readable");
    let mut inside = false;
    let mut out = Vec::new();
    for line in manifest.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('[') {
            inside = trimmed == "[features]";
            continue;
        }
        if !inside || trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        if let Some((name, _)) = trimmed.split_once('=') {
            let name = name.trim();
            if ["json", "postcard", "cbor"].contains(&name) {
                out.push(name.to_owned());
            }
        }
    }
    assert!(!out.is_empty(), "no codec feature is declared at all");
    out
}

// ---------------------------------------------------------------------------
// AC-010 — a gated item renders its gate
// ---------------------------------------------------------------------------

#[test]
fn every_gated_item_carries_its_badge() {
    let source = read("codec.rs");
    let lines: Vec<&str> = source.lines().collect();

    let mut gated = 0;
    for (at, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        let Some(rest) = trimmed.strip_prefix("#[cfg(feature = \"") else {
            continue;
        };
        let Some(feature) = rest.split('"').next() else {
            continue;
        };

        // Only an *item* gate renders on a page. A `#[cfg]` on a statement
        // inside a function body has no page of its own to badge.
        let mut attributes = Vec::new();
        let mut cursor = at + 1;
        while let Some(next) = lines.get(cursor) {
            let next = next.trim();
            if next.starts_with("#[") || next.starts_with("///") || next.is_empty() {
                attributes.push(next);
                cursor += 1;
                continue;
            }
            break;
        }
        let declaration = lines
            .get(cursor)
            .map(|line| line.trim())
            .unwrap_or_default();
        if !declaration.starts_with("pub ") && !declaration.starts_with("impl ") {
            continue;
        }
        gated += 1;

        let badge = format!("#[cfg_attr(docsrs, doc(cfg(feature = \"{feature}\")))]");
        assert!(
            attributes.contains(&badge.as_str()),
            "codec.rs:{}: `{feature}` gates an item that renders no badge",
            at + 1
        );
    }
    assert!(gated > 0, "no item in codec.rs is gated at all");
}

#[test]
fn new_identifiers_fit_the_item_table() {
    for name in ["Codec", "CodecError", "Json", "Postcard", "Cbor"] {
        assert!(
            name.len() <= 24,
            "`{name}` is {} characters; name and summary cannot share a row",
            name.len()
        );
    }
}
