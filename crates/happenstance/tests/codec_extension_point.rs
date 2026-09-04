//! The invitation to write a codec, and the limit that travels with it.
//!
//! `Codec` is **not sealed**, and its own page says so in as many words: *"a
//! codec of your own is a legitimate thing to write, which is why this is a
//! trait rather than an enum of the three below."* Writing one works. Reading
//! one back does not, past the moment a second codec enters the picture:
//! `decode_event` resolves a *foreign* tag through a fixed chain of the three
//! built-ins, and a tag that answers to none of them is `CodecError::UnknownTag`
//! with no registration seam anywhere to change that.
//!
//! For the three built-ins that refusal is temporary and means what
//! `UnknownTag`'s own page says it means — *a tag was written and this build
//! cannot honour it* — because turning the feature on makes the build able to.
//! For a codec of your own it is permanent: no feature exists to turn on, so no
//! build can ever become able to. An application that runs on its own codec for
//! months and then adopts `Json` gets `UnknownTag` from `Boundary::absorb` on
//! every historical event, which is an empty fold and an append condition
//! matching nothing.
//!
//! Whether that is repaired — a defaulted resolution method on `Codec`, a
//! registry, or sealing the trait and withdrawing the invitation — is a
//! decision with its own ADR number, and this file does not take it. What it
//! holds is the part that is true whichever way that goes: **the invitation and
//! its limit must travel together.** An extension point documented as open, on
//! a page that never says what the extension cannot do, is how a reader finds
//! out from their own production log.

use std::path::{Path, PathBuf};

fn src() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("src")
}

/// Read a source file with its line endings normalised to `\n`.
///
/// Same reason `tests/doc_budget.rs` does it: this repository is developed on
/// Windows with `core.autocrlf = true`, so a freshly checked out file is CRLF
/// and every pattern below would miss.
fn read(name: &str) -> String {
    std::fs::read_to_string(src().join(name))
        .unwrap_or_else(|error| panic!("{name}: {error}"))
        .replace("\r\n", "\n")
}

/// The `///` block immediately above `item`, with the markers stripped.
///
/// Hard-errors when the item is not found, rather than returning an empty
/// block: an empty block satisfies every `contains` below by vacuum, which is
/// the shape this repository has now met four times.
fn doc_block_above(source: &str, item: &str) -> String {
    let at = source
        .find(item)
        .unwrap_or_else(|| panic!("`{item}` is not in the source any more"));
    // Back up to the start of the item's own line. Without this an *indented*
    // item leaves its leading whitespace as a final partial line, which trims
    // to empty and ends the walk before it starts — and an empty block
    // satisfies every `contains` below.
    let at = source[..at].rfind('\n').map_or(0, |newline| newline + 1);
    let mut block: Vec<&str> = Vec::new();
    for line in source[..at].lines().rev() {
        let trimmed = line.trim_start();
        if let Some(body) = trimmed.strip_prefix("/// ") {
            block.push(body);
        } else if trimmed == "///" {
            block.push("");
        } else if !trimmed.starts_with("#[") && !trimmed.starts_with("//") {
            // An attribute or a plain comment may sit between the block and the
            // item — `#[error(…)]` does, on every `CodecError` variant — so
            // walking past those is what finds the block at all.
            break;
        }
    }
    assert!(!block.is_empty(), "`{item}` carries no doc block at all");
    block.reverse();
    block.join("\n")
}

/// The heading the limit lives under, on every page that carries it.
const HEADING: &str = "# Reading a tag this build did not write";

/// The trait that invites the extension states what the extension cannot do.
///
/// The wrong implementation this rejects is not hypothetical and is not a
/// deletion: it is the page as it shipped, where the invitation stands alone.
/// It also rejects the likelier future edit — someone rewording the "not
/// sealed" paragraph and carrying the invitation across without the limit.
#[test]
fn the_unsealed_trait_states_what_a_foreign_tag_costs() {
    let source = read("codec.rs");
    let block = doc_block_above(&source, "pub trait Codec {");

    let invitation = block
        .find("not sealed")
        .expect("the trait's page no longer says it is unsealed — if the trait was sealed, this test is what should have been deleted with it");
    let limit = block.find(HEADING).unwrap_or_else(|| {
        panic!(
            "`Codec`'s page invites a codec of your own and never says what \
             happens when one of its tags is read back. Add a `{HEADING}` \
             section. The page as written is the defect."
        )
    });

    assert!(
        invitation < limit,
        "the limit is stated above the invitation it limits, so a reader meets \
         the caveat before the offer"
    );

    let section = &block[limit..];
    for required in ["UnknownTag", "no build", "Json", "Postcard", "Cbor"] {
        assert!(
            section.contains(required),
            "the `{HEADING}` section never mentions `{required}`: a reader \
             cannot tell which tags resolve, or how permanently one does not"
        );
    }
}

/// The error the limit produces says which of its two meanings applies.
///
/// `UnknownTag` is one variant covering two conditions that differ in whether
/// they can ever be repaired: a built-in codec behind a feature that is off,
/// and a codec that is in no build's feature table at all. A caller writing a
/// recovery path needs to know which one they have.
#[test]
fn the_refusal_distinguishes_a_feature_from_a_dead_end() {
    let source = read("codec.rs");
    let block = doc_block_above(&source, "UnknownTag {");

    assert!(
        block.contains("no build"),
        "`CodecError::UnknownTag`'s page says a tag \"was written and this \
         build cannot honour it\" and stops there. For a codec outside this \
         crate no build can, ever, and the page has to say which case a \
         reader is in"
    );
}

/// The public door that takes a codec of your own carries the same warning.
///
/// `commit_with` is titled *"The command loop, with a codec of your own"*, so
/// it is the page a reader arrives at holding exactly the codec this limit is
/// about. Naming the section rather than restating it, because two copies of a
/// caveat is one that goes stale.
#[test]
fn the_door_for_a_codec_of_your_own_points_at_the_limit() {
    let source = read("command.rs");
    let block = doc_block_above(&source, "pub async fn commit_with<");

    assert!(
        block.contains("Reading a tag this build did not write"),
        "`commit_with`'s page offers a codec of your own and never points at \
         what reading one back costs"
    );
}
