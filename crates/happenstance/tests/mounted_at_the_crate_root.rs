//! The mount, and the density budget the rendered page is held to.
//!
//! A `testing` module that compiles but is never declared in `lib.rs` is not
//! delivered — it is reachable from this crate's own sources and from nowhere
//! else. Naming the three items through `happenstance::testing::…` is what makes
//! that a test failure rather than a discovery on docs.rs.

#![cfg(all(feature = "memory", feature = "json"))]

use happenstance::bytes::Bytes;
use happenstance::testing::{Decision, Given, assert_domain_event, given};
use happenstance::{Codec, CodecError, DecisionModel, DomainEvent, EventType, Tags};

/// Every item the design's `## Items` block lists, named through the crate root.
#[test]
fn testing_is_reachable_by_its_public_path() {
    // Named as values and as types, so neither a missing `pub` nor a missing
    // `pub mod` can pass. `Given` is `given`'s return type and the design's item
    // table omits it; it is named here because it cannot be private.
    let _: Given<Pinged> = given(Pinged {
        scope: Tags::empty(),
    });
    let _: fn(&[Ping]) = assert_domain_event::<Ping>;
    assert!(core::any::type_name::<Decision<Ping>>().contains("Decision"));
}

/// The smallest domain a boundary can be built from.
#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
enum Ping {
    Sent,
}

impl DomainEvent for Ping {
    const EVENT_TYPES: &'static [EventType] = &[EventType::from_static("PingSent")];

    fn event_type(&self) -> EventType {
        Self::EVENT_TYPES[0].clone()
    }

    fn tags(&self) -> Tags {
        Tags::empty()
    }

    fn encode<C: Codec>(&self, codec: &C) -> Result<Bytes, CodecError> {
        codec.encode(self)
    }

    fn decode<C: Codec>(codec: &C, _t: &EventType, data: &Bytes) -> Result<Self, CodecError> {
        codec.decode(data)
    }
}

#[derive(Debug, Clone)]
struct Pinged {
    scope: Tags,
}

impl DecisionModel for Pinged {
    type Event = Ping;

    fn scope(&self) -> &Tags {
        &self.scope
    }

    fn apply(&mut self, _event: Self::Event) {}
}

/// Anti-pattern 3, checked against the source rather than against a screenshot.
///
/// rustdoc gives a `pre` block `overflow-x: auto`, so an over-wide line in a doc
/// fence does not wrap — it **scrolls**, and a first example a reader has to
/// scroll horizontally is the most avoidable failure this page has. A test can
/// see it; a green assertion about a `data-*` attribute cannot.
#[test]
fn every_doc_fence_line_fits_the_column_budget() {
    for (file, source) in [
        ("src/testing/mod.rs", include_str!("../src/testing/mod.rs")),
        (
            "src/testing/render.rs",
            include_str!("../src/testing/render.rs"),
        ),
    ] {
        let mut inside = false;
        for (number, line) in source.lines().enumerate() {
            let trimmed = line.trim_start();
            let Some(body) = trimmed
                .strip_prefix("//! ")
                .or_else(|| trimmed.strip_prefix("/// "))
                .or_else(|| trimmed.strip_prefix("//!"))
                .or_else(|| trimmed.strip_prefix("///"))
            else {
                continue;
            };

            if body.trim_start().starts_with("```") {
                inside = !inside;
                continue;
            }
            if inside {
                assert!(
                    body.chars().count() <= 72,
                    "{file}:{}: a doc fence line is {} columns and will scroll \
                     at 1024px: {body}",
                    number + 1,
                    body.chars().count()
                );
            }
        }
        assert!(!inside, "{file}: an unclosed doc fence");
    }
}

/// The crate-root page points at the module, in the region the design fixes.
///
/// Read off the source, because the alternative is reading it off docs.rs after
/// a release. The link target is spelled conditionally in `lib.rs` so that it
/// resolves with the features off as well; what this asserts is that the region
/// is there at all.
#[test]
fn the_crate_root_page_points_at_the_testing_module() {
    let root = include_str!("../src/lib.rs");

    assert!(
        root.contains("//! # Testing without a database"),
        "the landing page has no region pointing at `happenstance::testing`"
    );
    assert!(
        root.contains("[testing-module]"),
        "the region does not link anywhere"
    );

    let region = root
        .find("//! # Testing without a database")
        .expect("the region");
    let features = root.find("//! # Features").expect("the features region");
    let adapters = root
        .find("//! Adapter authors should depend on")
        .expect("the adapter pointer");

    assert!(
        features < region && region < adapters,
        "the design fixes this region between Features and the adapter pointer"
    );
}
