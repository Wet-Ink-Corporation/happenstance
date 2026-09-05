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

/// One line, whitespace collapsed.
///
/// Every phrase below is prose a human wrote and rustfmt does not touch, so
/// where it wraps is an accident of the column budget: `"no build"` sat across
/// a line break on this file's first green run and a line-wise `contains` read
/// that as absent. Flattening makes the assertion about the sentence rather
/// than about the wrap.
fn flat(block: &str) -> String {
    block.split_whitespace().collect::<Vec<_>>().join(" ")
}

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

    let section = flat(&block[limit..]);
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
        flat(&block).contains("no build"),
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
        flat(&block).contains("Reading a tag this build did not write"),
        "`commit_with`'s page offers a codec of your own and never points at \
         what reading one back costs"
    );
}

// ---------------------------------------------------------------------------
// The behaviour the pages above now describe
// ---------------------------------------------------------------------------

#[cfg(all(feature = "memory", feature = "json"))]
mod behaviour {
    use happenstance::bytes::Bytes;
    use happenstance::{
        Boundary, Codec, CodecError, DecisionModel, DomainEvent, EventType, Json, MemoryEventStore,
        Retry, Tags, commit_with, read_decision_model,
    };
    use serde::{Deserialize, Serialize};

    /// A codec of your own, written the way the invitation says one may be.
    ///
    /// A legitimate implementor: in the build, in scope, holding a tag no other
    /// codec claims. That is the whole point — nothing about it is malformed,
    /// and it is still unreachable to a reader holding anything else.
    ///
    /// JSON underneath deliberately. If the bytes were exotic the refusal below
    /// could be read as *the payload was unreadable*; they are not, and they are
    /// not what refuses.
    struct Runic;

    impl Codec for Runic {
        const TAG: &'static str = "runic";

        fn encode<T: Serialize>(&self, value: &T) -> Result<Bytes, CodecError> {
            serde_json::to_vec(value)
                .map(Bytes::from)
                .map_err(|error| CodecError::Encode(Box::new(error)))
        }

        fn decode<T: serde::de::DeserializeOwned>(&self, data: &[u8]) -> Result<T, CodecError> {
            serde_json::from_slice(data).map_err(|error| CodecError::Decode(Box::new(error)))
        }
    }

    const CUT: EventType = EventType::from_static("RuneCut");

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    enum Rune {
        Cut,
    }

    impl DomainEvent for Rune {
        const EVENT_TYPES: &'static [EventType] = &[CUT];

        fn event_type(&self) -> EventType {
            CUT
        }

        fn tags(&self) -> Tags {
            ward_tags()
        }

        fn encode<C: Codec>(&self, codec: &C) -> Result<Bytes, CodecError> {
            codec.encode(self)
        }

        fn decode<C: Codec>(
            codec: &C,
            event_type: &EventType,
            data: &Bytes,
        ) -> Result<Self, CodecError> {
            if !Self::EVENT_TYPES.contains(event_type) {
                return Err(CodecError::UnknownEventType {
                    event_type: event_type.clone(),
                });
            }
            codec.decode(data)
        }
    }

    fn ward_tags() -> Tags {
        Tags::from_pairs([("ward", "w1")]).expect("a valid tag pair")
    }

    #[derive(Debug, Clone)]
    struct Ward {
        scope: Tags,
        cuts: u32,
    }

    impl Ward {
        fn new() -> Self {
            Self {
                scope: ward_tags(),
                cuts: 0,
            }
        }
    }

    impl DecisionModel for Ward {
        type Event = Rune;

        fn scope(&self) -> &Tags {
            &self.scope
        }

        fn apply(&mut self, _event: Self::Event) {
            self.cuts += 1;
        }
    }

    /// A tag written by a codec outside this crate is unreadable by any other.
    ///
    /// **This test passes on the commit that introduced it.** It characterises
    /// today's behaviour rather than repairing it: the repair is a decision with
    /// its own ADR number, and this lane does not take it. What the test buys is
    /// that the refusal has a name, a call site and an assertion, so whichever
    /// way that decision goes there is one place that has to change and says so.
    ///
    /// It is not decorative, because it fails under every option on the table: a
    /// resolution seam on `Codec`, a registry, or sealing the trait — the last
    /// of which stops `Runic` compiling at all. The wrong implementation it
    /// therefore rejects is a *silent* repair: a seam added while `Codec`'s page
    /// still tells a reader the limit stands.
    #[tokio::test]
    async fn a_codec_of_your_own_writes_a_tag_no_other_codec_can_read() {
        // Written through the crate's own writer, so the framing region is the
        // one `commit_with` produces rather than one this test spelled out.
        let store = MemoryEventStore::new();
        let written = commit_with(&store, Ward::new(), &Runic, Retry::once(), |_: &Ward| {
            Ok::<_, core::convert::Infallible>(vec![Rune::Cut])
        })
        .await
        .expect("the command loop writes under a codec of your own");
        assert_eq!(written.attempts, 1);

        let mut ward = Ward::new();
        let query = ward.query().expect("a constrained boundary");
        let (events, _anchor) = read_decision_model(&store, &query)
            .await
            .expect("the memory store reads");
        assert_eq!(events.len(), 1, "the loop wrote one event");

        // The codec reads back the tag it wrote. Writing is open.
        ward.absorb(&events[0], &Runic)
            .expect("a codec reads the tag it wrote");

        // Anything else does not, and no feature of this crate changes that.
        let refusal = Ward::new()
            .absorb(&events[0], &Json)
            .expect_err("a foreign tag is refused");
        assert!(
            matches!(&refusal, CodecError::UnknownTag { tag } if &**tag == "runic"),
            "expected `UnknownTag` naming the codec's own tag, got {refusal:?}"
        );
    }
}
