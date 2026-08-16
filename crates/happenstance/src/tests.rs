//! Crate-internal tests for the typed vocabulary.
//!
//! The fixture domain here is deliberately **not** the `Seat` domain the
//! doctests use: a change to one must not quietly repair the other.

use happenstance_core::bytes::Bytes;
use happenstance_core::{
    Event, EventId, EventType, InvalidQuery, RecordedAt, SequencePosition, SequencedEvent, StoreId,
    Tags,
};
use serde::{Deserialize, Serialize};

use crate::boundary::{derive_query, nominates};
use crate::{Boundary, Codec, CodecError, DecisionModel, DomainEvent};

// ---------------------------------------------------------------------------
// The fixture domain
// ---------------------------------------------------------------------------

/// Three declared event types, **two** variants. The third is the residual the
/// compiler cannot check, and it is what makes a genuine `EVENT_TYPES`/fold
/// disagreement reachable in a test.
const ISSUED: EventType = EventType::from_static("TicketIssued");
const VOIDED: EventType = EventType::from_static("TicketVoided");
const AUDITED: EventType = EventType::from_static("TicketAudited");

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
enum Ticket {
    Issued { holder: String },
    Voided,
}

impl DomainEvent for Ticket {
    const EVENT_TYPES: &'static [EventType] = &[ISSUED, VOIDED, AUDITED];

    fn event_type(&self) -> EventType {
        match self {
            Self::Issued { .. } => ISSUED,
            Self::Voided => VOIDED,
        }
    }

    fn tags(&self) -> Tags {
        Tags::empty()
    }

    fn encode<C: Codec>(&self, codec: &C) -> Result<Bytes, CodecError> {
        codec.encode(self)
    }

    fn decode<C: Codec>(
        codec: &C,
        event_type: &EventType,
        data: &Bytes,
    ) -> Result<Self, CodecError> {
        if event_type == &AUDITED || !Self::EVENT_TYPES.contains(event_type) {
            return Err(CodecError::UnknownEventType {
                event_type: event_type.clone(),
            });
        }
        codec.decode(data)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Gate {
    scope: Tags,
    issued: u32,
    voided: u32,
}

impl Gate {
    fn for_show(show: &str) -> Self {
        Self {
            scope: Tags::from_pairs([("show", show)]).expect("a valid tag pair"),
            issued: 0,
            voided: 0,
        }
    }
}

impl DecisionModel for Gate {
    type Event = Ticket;

    fn scope(&self) -> &Tags {
        &self.scope
    }

    // No `_ =>` arm: adding a variant to `Ticket` is a compile error here.
    fn apply(&mut self, event: Self::Event) {
        match event {
            Ticket::Issued { .. } => self.issued += 1,
            Ticket::Voided => self.voided += 1,
        }
    }
}

/// A real codec, so an encode/decode failure carries a real typed source.
struct Json;

impl Codec for Json {
    const TAG: &'static str = "json";

    fn encode<T: Serialize>(&self, value: &T) -> Result<Bytes, CodecError> {
        serde_json::to_vec(value)
            .map(Bytes::from)
            .map_err(|e| CodecError::Encode(Box::new(e)))
    }

    fn decode<T: serde::de::DeserializeOwned>(&self, data: &[u8]) -> Result<T, CodecError> {
        serde_json::from_slice(data).map_err(|e| CodecError::Decode(Box::new(e)))
    }
}

fn sequenced(position: u64, event: Event) -> SequencedEvent {
    let position = SequencePosition::new(position).expect("a non-zero position");
    SequencedEvent::new(
        position,
        EventId::new(StoreId::from_bytes([7; 16]), position),
        RecordedAt::from_millis(1_700_000_000_000),
        event,
    )
}

fn read_event(position: u64, ticket: &Ticket, show: &str) -> SequencedEvent {
    let payload = ticket.encode(&Json).expect("the fixture encodes");
    let event = Event::new(ticket.event_type(), payload)
        .expect("a valid event type")
        .with_tags(Tags::from_pairs([("show", show)]).expect("a valid tag pair"));
    sequenced(position, event)
}

// ---------------------------------------------------------------------------
// AC-001 — the fold is exhaustive, the scope is held, `Clone` not `Default`
// ---------------------------------------------------------------------------

#[test]
fn fold_is_exhaustive_over_domain_enum() {
    let mut gate = Gate::for_show("s1");
    gate.apply(Ticket::Issued {
        holder: "h1".into(),
    });
    gate.apply(Ticket::Voided);

    // Both arms are reachable and both are the model's own. The exhaustiveness
    // itself is the compiler's: `apply` above has no `_ =>` arm, so a third
    // `Ticket` variant is `error[E0004]` in this file.
    assert_eq!((gate.issued, gate.voided), (1, 1));
}

#[test]
fn scope_returns_the_models_own_tags() {
    let gate = Gate::for_show("s1");
    let expected = Tags::from_pairs([("show", "s1")]).expect("a valid tag pair");

    assert_eq!(gate.scope(), &expected);
    // Read twice: validation happened in the constructor, not here.
    assert_eq!(gate.scope(), gate.scope());
}

#[test]
fn decision_model_is_clone_not_default() {
    fn assert_clone<M: DecisionModel + Clone>() {}
    assert_clone::<Gate>();

    let pristine = Gate::for_show("s1");
    let mut folded = pristine.clone();
    folded.apply(Ticket::Voided);

    assert_eq!(pristine.voided, 0, "the clone taken first stays pristine");
    assert_eq!(folded.voided, 1);
}

// ---------------------------------------------------------------------------
// AC-002 — the query is derived, and it is an ordinary value
// ---------------------------------------------------------------------------

#[test]
fn query_is_derived_from_event_types_and_scope() {
    let gate = Gate::for_show("s1");
    let query = gate
        .query()
        .expect("a well-formed boundary derives a query");

    let items = query.items().expect("the derivation constrains something");
    assert_eq!(items.len(), 1);

    // The oracle is built from what the model itself declared, never from a
    // literal list: `EVENT_TYPES` sorted and deduplicated is what a query item
    // holds.
    let mut declared: Vec<EventType> = <Gate as DecisionModel>::Event::EVENT_TYPES.to_vec();
    declared.sort_unstable();
    declared.dedup();

    assert_eq!(items[0].types(), declared.as_slice());
    assert_eq!(items[0].tags(), gate.scope());
}

#[test]
fn query_is_a_value_a_test_can_assert_on() {
    let gate = Gate::for_show("s1");
    let query = gate
        .query()
        .expect("a well-formed boundary derives a query");

    // Bound, printable, and inspectable outside any read.
    assert!(!query.is_all(), "a derived query is not `Query::all()`");
    assert!(format!("{query:?}").contains("TicketIssued"));

    let inside = read_event(1, &Ticket::Voided, "s1");
    let outside = read_event(2, &Ticket::Voided, "s2");
    assert!(query.matches(inside.event.event_type(), inside.event.tags()));
    assert!(!query.matches(outside.event.event_type(), outside.event.tags()));
}

// ---------------------------------------------------------------------------
// AC-003 — the refusal is stated out loud, never widened
// ---------------------------------------------------------------------------

#[test]
fn unconstrained_boundary_is_an_error_not_query_all() {
    // A boundary that constrains neither types nor tags. The `query()` route to
    // it is closed at compile time — an empty `EVENT_TYPES` does not compile —
    // so the arm is asserted at the derivation `query()` delegates to.
    let refused = derive_query(&[], &Tags::empty());
    assert!(
        matches!(refused, Err(InvalidQuery::UnconstrainedItem)),
        "an unconstrained boundary is refused, not widened: {refused:?}"
    );

    // And the derivation never answers with `Query::all()` for a well-formed
    // model either, which is the same failure wearing the other face.
    let derived = Gate::for_show("s1")
        .query()
        .expect("a well-formed boundary derives a query");
    assert!(!derived.is_all());
}

// ---------------------------------------------------------------------------
// AC-004 — absorb routes by nomination, and codec failures carry a source
// ---------------------------------------------------------------------------

#[test]
fn absorb_skips_an_unnominated_event() {
    let mut gate = Gate::for_show("s1");

    // Right type, wrong scope: the query never nominated it.
    let other_show = read_event(1, &Ticket::Voided, "s2");
    gate.absorb(&other_show, &Json)
        .expect("an unnominated event is skipped, not an error");

    // Wrong type entirely, right scope.
    let unrelated = Event::new("SomethingElse", Bytes::from_static(b"not json"))
        .expect("a valid event type")
        .with_tags(Tags::from_pairs([("show", "s1")]).expect("a valid tag pair"));
    gate.absorb(&sequenced(2, unrelated), &Json)
        .expect("an unnominated event is skipped, not an error");

    assert_eq!((gate.issued, gate.voided), (0, 0));
}

#[test]
fn absorb_returns_unknown_event_type_when_a_nominated_event_cannot_be_decoded() {
    let mut gate = Gate::for_show("s1");

    // `TicketAudited` is declared in `EVENT_TYPES` and has no fold arm, so the
    // query nominates it and the domain type cannot decode it. That is a real
    // disagreement, and it is loud.
    let audited = Event::new(AUDITED, Bytes::from_static(b"{}"))
        .expect("a valid event type")
        .with_tags(Tags::from_pairs([("show", "s1")]).expect("a valid tag pair"));

    let refused = gate.absorb(&sequenced(3, audited), &Json);
    match refused {
        Err(CodecError::UnknownEventType { event_type }) => assert_eq!(event_type, AUDITED),
        other => panic!("expected UnknownEventType, got {other:?}"),
    }
    assert_eq!((gate.issued, gate.voided), (0, 0));
}

#[test]
fn absorb_applies_a_decoded_event_to_the_fold() {
    let mut gate = Gate::for_show("s1");

    gate.absorb(
        &read_event(
            1,
            &Ticket::Issued {
                holder: "h1".into(),
            },
            "s1",
        ),
        &Json,
    )
    .expect("a nominated, decodable event folds");
    gate.absorb(&read_event(2, &Ticket::Voided, "s1"), &Json)
        .expect("a nominated, decodable event folds");

    assert_eq!((gate.issued, gate.voided), (1, 1));
}

#[test]
fn codec_error_carries_a_typed_source() {
    fn assert_source<E: core::error::Error>(_: &E) {}

    let failure = Json
        .decode::<Ticket>(b"not json at all")
        .expect_err("the bytes are not a Ticket");
    assert_source(&failure);

    let source = core::error::Error::source(&failure).expect("a typed source, never a String");
    assert!(
        source.downcast_ref::<serde_json::Error>().is_some(),
        "the codec's own error survives the chain"
    );
}

#[test]
fn nomination_agrees_with_the_derived_query() {
    let gate = Gate::for_show("s1");
    let query = gate
        .query()
        .expect("a well-formed boundary derives a query");
    let types = <Gate as DecisionModel>::Event::EVENT_TYPES;

    for candidate in [
        read_event(1, &Ticket::Voided, "s1"),
        read_event(2, &Ticket::Voided, "s2"),
    ] {
        let event = &candidate.event;
        assert_eq!(
            nominates(types, gate.scope(), event),
            query.matches(event.event_type(), event.tags()),
            "the fold and the query select the same events"
        );
    }
}

// ---------------------------------------------------------------------------
// AC-005 — the mount, and the contract's names
// ---------------------------------------------------------------------------

/// Nothing is imported into this module on purpose: both spellings have to
/// stay fully qualified for the witnesses below to say anything at all.
mod shadowing {
    // Type-equality witnesses: these bodies compile only if the name reached
    // through `happenstance` is the very item `happenstance_core` exports. A
    // second `Query` of our own would make each one `error[E0308]`.
    fn same_query(q: happenstance_core::Query) -> crate::Query {
        q
    }
    fn same_tags(t: happenstance_core::Tags) -> crate::Tags {
        t
    }
    fn same_event(e: happenstance_core::Event) -> crate::Event {
        e
    }
    fn core_store_bound<S: happenstance_core::EventStore>() {}
    fn facade_store_bound<S: crate::EventStore>() {
        core_store_bound::<S>();
    }

    #[test]
    fn contract_names_are_not_shadowed() {
        let _ = same_tags as fn(happenstance_core::Tags) -> crate::Tags;
        let _ = same_event as fn(happenstance_core::Event) -> crate::Event;
        let _ = facade_store_bound::<happenstance_core::MemoryEventStore> as fn();

        assert!(same_query(happenstance_core::Query::all()).is_all());
    }
}

// ---------------------------------------------------------------------------
// AC-008 — the residual agreement, and purity
// ---------------------------------------------------------------------------

#[test]
fn every_variant_event_type_is_declared() {
    let variants = [
        Ticket::Issued {
            holder: "h1".into(),
        },
        Ticket::Voided,
    ];

    for variant in &variants {
        assert!(
            Ticket::EVENT_TYPES.contains(&variant.event_type()),
            "{} is returned by a variant and not declared",
            variant.event_type()
        );
    }

    // The residual runs the other way and is not compiler-checkable: `AUDITED`
    // is declared and no variant returns it. That is why `decode` refuses it
    // rather than pretending the disagreement is not there.
    assert!(
        !variants.iter().any(|v| v.event_type() == AUDITED),
        "the declared-but-unfolded type is the residual this test records"
    );
}

#[test]
fn nothing_in_the_vocabulary_touches_a_store() {
    // Define, fold, derive and encode — with no runtime, no store constructed,
    // and nothing awaited. Every item this crate adds is callable from here.
    let mut gate = Gate::for_show("s1");
    let query = gate
        .query()
        .expect("a well-formed boundary derives a query");
    assert!(!query.is_all());

    let issued = Ticket::Issued {
        holder: "h1".into(),
    };
    let payload = issued.encode(&Json).expect("the fixture encodes");
    let round_tripped = Ticket::decode(&Json, &issued.event_type(), &payload)
        .expect("what this codec wrote, it reads");
    assert_eq!(round_tripped, issued);

    gate.absorb(&read_event(1, &issued, "s1"), &Json)
        .expect("a nominated, decodable event folds");
    assert_eq!(gate.issued, 1);
}

// ---------------------------------------------------------------------------
// AC-007 — the density budget, read off this crate's own sources
// ---------------------------------------------------------------------------

/// Doc-comment prose, in columns. Lines carrying a URL are exempt: a link
/// target cannot be wrapped, and the existing crate root already carries two.
const PROSE_COLUMNS: usize = 80;
/// Code inside a doc fence, in columns. This is the budget that stops the
/// fence acquiring a horizontal scrollbar at 1024px.
const FENCE_COLUMNS: usize = 72;
/// The crate-root module doc, in lines.
const MODULE_DOC_LINES: usize = 130;
/// The first sentence of any doc comment, in characters.
const FIRST_SENTENCE: usize = 80;
/// A public identifier this story adds, in characters.
const IDENTIFIER: usize = 24;
/// Hidden lines in the crate-root fence, and they may only be harness.
const HIDDEN_LINES: usize = 2;
/// Prose lines above the crate-root fence.
const LINES_TO_FIRST_FENCE: usize = 12;
/// Visible lines in the crate-root fence.
///
/// The signed-off design budgets 35 and its own mock measured the program it
/// budgeted at **81** (`_design.md`, `## Mock`, finding 1), accepted at
/// sign-off as DT-2's measurement rather than as a defect to fix. The
/// vocabulary program held here is 44, and the shortfall against 35 is
/// recorded as defect **D-2** rather than closed by hiding lines behind `# `.
const FENCE_LINES: usize = 44;
/// The number the design asked for, kept in the source so the gap is visible
/// at the place a future implementer would try to widen the ceiling.
const FENCE_LINES_DESIGNED: usize = 35;

fn sources() -> Vec<(String, String)> {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut out = Vec::new();
    for entry in std::fs::read_dir(&dir).expect("the crate's own src/ is readable") {
        let path = entry.expect("a readable directory entry").path();
        // This file is `#[cfg(test)]`: nothing in it renders on a page, so it
        // is not what the budget is about.
        if path.file_name().is_some_and(|n| n == "tests.rs") {
            continue;
        }
        if path.extension().is_some_and(|e| e == "rs") {
            let name = path
                .file_name()
                .expect("a named file")
                .to_string_lossy()
                .into_owned();
            out.push((name, std::fs::read_to_string(&path).expect("valid utf-8")));
        }
    }
    assert!(out.len() >= 4, "the vocabulary's modules were not found");
    out
}

/// The doc-comment body of a line, if it is one: `//!` and `///` alike.
fn doc_body(line: &str) -> Option<&str> {
    let trimmed = line.trim_start();
    for marker in ["//!", "///"] {
        if let Some(rest) = trimmed.strip_prefix(marker) {
            return Some(rest.strip_prefix(' ').unwrap_or(rest));
        }
    }
    None
}

#[test]
fn doc_density_budget_holds() {
    let mut checked = 0_usize;

    for (name, source) in sources() {
        let mut in_fence = false;
        let mut awaiting_first_sentence = true;
        let mut sentence = String::new();

        for (index, line) in source.lines().enumerate() {
            let at = format!("{name}:{}", index + 1);
            let Some(body) = doc_body(line) else {
                awaiting_first_sentence = true;
                sentence.clear();
                continue;
            };
            checked += 1;

            if body.trim_start().starts_with("```") {
                in_fence = !in_fence;
                continue;
            }

            if in_fence {
                assert!(
                    body.chars().count() <= FENCE_COLUMNS,
                    "{at}: a doc fence line is {} columns, over the {FENCE_COLUMNS}-column \
                     budget, so the fence scrolls at 1024px",
                    body.chars().count()
                );
                continue;
            }

            assert!(
                line.chars().count() <= PROSE_COLUMNS || line.contains("http"),
                "{at}: doc prose is {} columns, over the {PROSE_COLUMNS}-column budget",
                line.chars().count()
            );

            if awaiting_first_sentence && !body.trim().is_empty() {
                if !sentence.is_empty() {
                    sentence.push(' ');
                }
                sentence.push_str(body.trim());
                if let Some(end) = sentence.find(". ").or_else(|| {
                    sentence
                        .ends_with('.')
                        .then(|| sentence.len().saturating_sub(1))
                }) {
                    assert!(
                        end < FIRST_SENTENCE,
                        "{at}: a first sentence is {} characters, over the \
                         {FIRST_SENTENCE}-character item-table budget",
                        end + 1
                    );
                    awaiting_first_sentence = false;
                    sentence.clear();
                }
            }
        }
        assert!(!in_fence, "{name}: an unterminated doc fence");
    }

    assert!(
        checked > 200,
        "the budget read too few doc lines to be real"
    );
}

#[test]
fn public_identifiers_fit_the_item_table() {
    for (name, source) in sources() {
        for (index, line) in source.lines().enumerate() {
            let trimmed = line.trim_start();
            for keyword in ["pub trait ", "pub enum ", "pub struct ", "pub fn "] {
                let Some(rest) = trimmed.strip_prefix(keyword) else {
                    continue;
                };
                let identifier: String = rest
                    .chars()
                    .take_while(|c| c.is_alphanumeric() || *c == '_')
                    .collect();
                assert!(
                    identifier.chars().count() <= IDENTIFIER,
                    "{name}:{}: `{identifier}` is over the {IDENTIFIER}-character budget",
                    index + 1
                );
            }
        }
    }
}

#[test]
fn the_crate_root_page_fits_above_the_fold() {
    let source = include_str!("lib.rs");

    let doc: Vec<&str> = source
        .lines()
        .filter(|line| line.trim_start().starts_with("//!"))
        .collect();
    assert!(
        doc.len() <= MODULE_DOC_LINES,
        "the crate-root module doc is {} lines, over {MODULE_DOC_LINES}",
        doc.len()
    );

    let opens = doc
        .iter()
        .position(|line| doc_body(line).is_some_and(|b| b.starts_with("```")))
        .expect("the crate root carries a fence");
    assert!(
        opens <= LINES_TO_FIRST_FENCE,
        "the first fence starts {opens} prose lines down, over {LINES_TO_FIRST_FENCE}"
    );

    let closes = doc
        .iter()
        .skip(opens + 1)
        .position(|line| doc_body(line).is_some_and(|b| b.starts_with("```")))
        .expect("the fence is terminated")
        + opens
        + 1;

    let fence: Vec<&str> = doc[opens + 1..closes]
        .iter()
        .filter_map(|line| doc_body(line))
        .collect();
    // rustdoc hides a line spelled `# …`, and only that: `#[derive(…)]` starts
    // with a hash and renders in full.
    let hidden: Vec<&&str> = fence
        .iter()
        .filter(|body| body.starts_with("# ") || body.trim_end() == "#")
        .collect();

    assert!(
        hidden.len() <= HIDDEN_LINES,
        "{} hidden lines in the first fence, over {HIDDEN_LINES}",
        hidden.len()
    );
    for body in &hidden {
        assert!(
            body.contains("Ok::<") || body.contains("tokio"),
            "a hidden line is API, not harness: {body}"
        );
    }

    let visible = fence.len() - hidden.len();
    assert!(
        visible <= FENCE_LINES,
        "the first fence is {visible} visible lines, over the {FENCE_LINES} it holds \
         today (the design asked for {FENCE_LINES_DESIGNED}; see defect D-2)"
    );
}
