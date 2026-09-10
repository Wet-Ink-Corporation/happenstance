//! The four-region failure message, driven by a deliberately divergent model.
//!
//! Every case here opens its subject **inside** the caught closure, so the
//! closure captures nothing, coerces to `fn()`, and satisfies `catch_unwind`'s
//! `UnwindSafe` bound without an `AssertUnwindSafe` that would have made the
//! bound mean nothing (RS-60-1).
//!
//! Nothing uses `#[should_panic]`. One case has to install a process-wide panic
//! hook to read the panic's *location*, and a `#[should_panic]` test panicking
//! concurrently would be recorded against it — so every panicking case here goes
//! through one helper that takes one lock. The helper is also strictly stronger
//! than `#[should_panic]`: it returns the message, and every assertion below is
//! about the message rather than about the fact that something failed.

#![cfg(all(feature = "memory", feature = "json"))]
#![allow(clippy::unwrap_used)]

use std::panic;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Mutex, PoisonError};

use happenstance::bytes::Bytes;
use happenstance::testing::{assert_domain_event, given};
use happenstance::{Codec, CodecError, DecisionModel, DomainEvent, EventType, Tags};
use happenstance_testkit::block_on;

// ---------------------------------------------------------------------------
// The divergent domain: a model whose EVENT_TYPES omits a type its log holds
// ---------------------------------------------------------------------------

const DEPOSITED: EventType = EventType::from_static("Deposited");
const WITHDRAWN: EventType = EventType::from_static("Withdrawn");

/// The type a *complete* domain would declare — used only to seed.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
enum Ledger {
    Deposited,
    Withdrawn,
}

impl DomainEvent for Ledger {
    const EVENT_TYPES: &'static [EventType] = &[DEPOSITED, WITHDRAWN];

    fn event_type(&self) -> EventType {
        match self {
            Self::Deposited => DEPOSITED,
            Self::Withdrawn => WITHDRAWN,
        }
    }

    fn tags(&self) -> Tags {
        scope()
    }

    fn encode<C: Codec>(&self, codec: &C) -> Result<Bytes, CodecError> {
        codec.encode(self)
    }

    fn decode<C: Codec>(codec: &C, _t: &EventType, data: &Bytes) -> Result<Self, CodecError> {
        codec.decode(data)
    }
}

/// The ADR-0020 hazard, written down: a declaration that omits a type the fold
/// needs, so the query never nominates it and the model silently folds a
/// shorter log.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
enum DepositOnly {
    Deposited,
}

impl DomainEvent for DepositOnly {
    const EVENT_TYPES: &'static [EventType] = &[DEPOSITED];

    fn event_type(&self) -> EventType {
        DEPOSITED
    }

    fn tags(&self) -> Tags {
        scope()
    }

    fn encode<C: Codec>(&self, codec: &C) -> Result<Bytes, CodecError> {
        codec.encode(self)
    }

    fn decode<C: Codec>(codec: &C, _t: &EventType, data: &Bytes) -> Result<Self, CodecError> {
        codec.decode(data)
    }
}

#[derive(Debug, Clone)]
struct Deposits {
    scope: Tags,
    seen: u32,
}

impl DecisionModel for Deposits {
    type Event = DepositOnly;

    fn scope(&self) -> &Tags {
        &self.scope
    }

    fn apply(&mut self, _event: Self::Event) {
        self.seen += 1;
    }
}

/// A type nobody's `EVENT_TYPES` will fit on one line.
const LONG: EventType = EventType::from_static(
    "WithdrawnFromAnAccountWhoseEventTypeNobodyWouldEverReasonablyChooseToWriteOutLikeThis",
);

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Verbose;

impl DomainEvent for Verbose {
    const EVENT_TYPES: &'static [EventType] = &[LONG];

    fn event_type(&self) -> EventType {
        LONG
    }

    fn tags(&self) -> Tags {
        scope()
    }

    fn encode<C: Codec>(&self, codec: &C) -> Result<Bytes, CodecError> {
        codec.encode(self)
    }

    fn decode<C: Codec>(codec: &C, _t: &EventType, data: &Bytes) -> Result<Self, CodecError> {
        codec.decode(data)
    }
}

/// A type whose value disagrees with its own declaration.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Drifted;

impl DomainEvent for Drifted {
    const EVENT_TYPES: &'static [EventType] = &[DEPOSITED];

    // The defect no `const` can see: a match arm returning a type the
    // declaration above does not carry.
    fn event_type(&self) -> EventType {
        WITHDRAWN
    }

    fn tags(&self) -> Tags {
        scope()
    }

    fn encode<C: Codec>(&self, codec: &C) -> Result<Bytes, CodecError> {
        codec.encode(self)
    }

    fn decode<C: Codec>(codec: &C, _t: &EventType, data: &Bytes) -> Result<Self, CodecError> {
        codec.decode(data)
    }
}

/// Both names ARE declared; the two sequences disagree on their ORDER.
///
/// The defect ADR-0059's positional-agreement precondition exists to reject,
/// and the reason it is a defect rather than a style: the rendered docs teach
/// `EVENT_TYPES[i]` as the way to name an event type, which is correct only
/// while index `i` means the same thing on both sides. Membership alone cannot
/// see this — every value here is declared.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Permuted;

impl DomainEvent for Permuted {
    const EVENT_TYPES: &'static [EventType] = &[DEPOSITED, WITHDRAWN];

    fn event_type(&self) -> EventType {
        WITHDRAWN
    }

    fn tags(&self) -> Tags {
        scope()
    }

    fn encode<C: Codec>(&self, codec: &C) -> Result<Bytes, CodecError> {
        codec.encode(self)
    }

    fn decode<C: Codec>(codec: &C, _t: &EventType, data: &Bytes) -> Result<Self, CodecError> {
        codec.decode(data)
    }
}

/// The caller's own refusal.
#[derive(Debug)]
struct Refused;

impl core::fmt::Display for Refused {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("the account is closed")
    }
}

impl core::error::Error for Refused {}

fn scope() -> Tags {
    Tags::from_pairs([("account", "a1")]).expect("a valid tag pair")
}

fn model() -> Deposits {
    Deposits {
        scope: scope(),
        seen: 0,
    }
}

// ---------------------------------------------------------------------------
// Catching one panic at a time, with its location
// ---------------------------------------------------------------------------

/// Serialises every panicking case in this file.
///
/// The location case installs a process-wide hook, and libtest runs tests on
/// parallel threads; without this, another case's panic is what the hook
/// records.
static PANICS: Mutex<()> = Mutex::new(());

/// Where the caught panic was raised.
static LINE: AtomicU32 = AtomicU32::new(0);

/// What one caught panic reported.
struct Caught {
    message: String,
    file: String,
    line: u32,
}

/// Runs `subject`, which must panic, and reports what it panicked with.
///
/// `fn()` rather than a closure type: a function pointer captures nothing and
/// is unconditionally `UnwindSafe`, so the bound passes honestly instead of
/// being asserted away.
fn caught(subject: fn()) -> Caught {
    let guard = PANICS.lock().unwrap_or_else(PoisonError::into_inner);

    let slot: std::sync::Arc<Mutex<Option<(String, u32)>>> = std::sync::Arc::default();
    let sink = std::sync::Arc::clone(&slot);
    let previous = panic::take_hook();
    panic::set_hook(Box::new(move |info| {
        *sink.lock().unwrap_or_else(PoisonError::into_inner) =
            info.location().map(|at| (at.file().to_owned(), at.line()));
    }));

    let payload = panic::catch_unwind(subject);

    panic::set_hook(previous);
    let recorded = slot.lock().unwrap_or_else(PoisonError::into_inner).clone();
    drop(guard);

    let payload = payload.expect_err("the subject was supposed to panic");
    let message = payload
        .downcast_ref::<String>()
        .cloned()
        .or_else(|| {
            payload
                .downcast_ref::<&str>()
                .map(|held| (*held).to_owned())
        })
        .expect("a string panic payload");
    let (file, line) = recorded.expect("the hook recorded a location");

    Caught {
        message,
        file,
        line,
    }
}

/// The subject every message case drives: a model that emits nothing while the
/// test expects one event, over a log carrying a type the model never
/// nominated.
fn divergent_assertion() {
    let decision = block_on(async {
        given(model())
            .event(Ledger::Deposited)
            .unwrap()
            .event(Ledger::Withdrawn)
            .unwrap()
            .when(|_: &Deposits| Ok::<_, core::convert::Infallible>(Vec::new()))
            .await
            .unwrap()
    });
    decision.then(&[DepositOnly::Deposited]);
}

// ---------------------------------------------------------------------------
// AC-002 — four labelled regions, in order, at the caller's own line
// ---------------------------------------------------------------------------

#[test]
fn four_regions_render_in_order() {
    let message = caught(divergent_assertion).message;

    let at = |label: &str| {
        message
            .find(label)
            .unwrap_or_else(|| panic!("`{label}` is missing from:\n{message}"))
    };

    assert!(
        message.starts_with("assertion failed: the decision emitted different events"),
        "the header sentence is missing:\n{message}"
    );
    // Byte offsets, not four `contains` calls: four labels in the wrong order
    // satisfy `contains` perfectly.
    assert!(at("expected:") < at("actual:"), "{message}");
    assert!(
        at("actual:") < at("selected by the model's query:"),
        "{message}"
    );
    assert!(
        at("selected by the model's query:") < at("seeded but NOT selected:"),
        "{message}"
    );
    assert!(
        at("seeded but NOT selected:") < at("derived query:"),
        "the derived-query line is last: {message}"
    );
}

#[test]
fn panic_location_is_the_callers_line() {
    let caught = caught(|| {
        // Opened inside, so nothing is captured.
        let decision = block_on(async {
            given(model())
                .event(Ledger::Deposited)
                .unwrap()
                .when(|_: &Deposits| Ok::<_, core::convert::Infallible>(Vec::new()))
                .await
                .unwrap()
        });
        LINE.store(line!() + 1, Ordering::Relaxed);
        decision.then(&[DepositOnly::Deposited]);
    });

    assert!(
        caught.file.ends_with("dsl_failure_message.rs"),
        "the panic named the DSL's own file rather than the caller's: {}",
        caught.file
    );
    assert_eq!(
        caught.line,
        LINE.load(Ordering::Relaxed),
        "`#[track_caller]` and the `panic!` have come apart: the location is no \
         longer the assertion the author wrote"
    );
}

// ---------------------------------------------------------------------------
// AC-003 — the filter cannot hide what it filtered
// ---------------------------------------------------------------------------

#[test]
fn region_four_names_the_seeded_but_unselected_event() {
    let message = caught(divergent_assertion).message;

    let (head, region_four) = message
        .split_once("seeded but NOT selected:")
        .expect("region four");
    let selected = head
        .split_once("selected by the model's query:")
        .expect("region three")
        .1;

    // The oracle is spelled from what this test seeded — a `Withdrawn` the
    // model's EVENT_TYPES omits — never by asking the DSL what it filtered.
    assert!(
        region_four.contains("Withdrawn"),
        "the event the query dropped is the entire diagnosis and it is not on \
         the screen:\n{message}"
    );
    assert!(
        !selected.contains("Withdrawn"),
        "an event the query never nominated is reported as selected:\n{message}"
    );
    assert!(
        selected.contains("Deposited"),
        "the event the query did nominate is missing:\n{message}"
    );

    let closing = message.lines().last().expect("a closing line");
    assert!(
        closing.starts_with("derived query:") && closing.contains("Deposited"),
        "the last line must name the query that did the filtering: {closing}"
    );
}

// ---------------------------------------------------------------------------
// AC-004 — the degenerate case keeps its region
// ---------------------------------------------------------------------------

#[test]
fn empty_seed_keeps_region_four_and_says_so() {
    let message = caught(|| {
        let decision = block_on(async {
            given(model())
                .when(|_: &Deposits| {
                    Ok::<_, core::convert::Infallible>(vec![DepositOnly::Deposited])
                })
                .await
                .unwrap()
        });
        decision.then(&[]);
    })
    .message;

    assert!(
        message.contains("seeded but NOT selected:"),
        "the region disappeared, which reads as 'the DSL has nothing to say \
         about the filter':\n{message}"
    );
    assert!(
        message.contains("(nothing was seeded)"),
        "the region must say why it is empty:\n{message}"
    );
}

#[tokio::test]
async fn empty_selection_is_not_an_error() {
    given(model())
        .when(|held: &Deposits| {
            assert_eq!(
                held.seen, 0,
                "a query matching nothing must never `apply` the model"
            );
            Ok::<_, core::convert::Infallible>(Vec::new())
        })
        .await
        .expect("an empty selection is a well-formed test, not a failure")
        .then(&[]);
}

// ---------------------------------------------------------------------------
// AC-005 — the density budget, and which region yields
// ---------------------------------------------------------------------------

#[test]
fn overflow_truncates_selected_first_and_the_diagnosis_last() {
    let message = caught(|| {
        let decision = block_on(async {
            let mut seeded = given(model());
            for _ in 0..20 {
                seeded = seeded.event(Ledger::Deposited).unwrap();
            }
            seeded
                .event(Ledger::Withdrawn)
                .unwrap()
                .when(|_: &Deposits| Ok::<_, core::convert::Infallible>(Vec::new()))
                .await
                .unwrap()
        });
        decision.then(&[DepositOnly::Deposited]);
    })
    .message;

    // Twenty selected, eight printed: the oracle is this test's own arithmetic.
    assert!(
        message.contains("… and 12 more"),
        "the selected region did not truncate:\n{message}"
    );

    let region_four = message
        .split_once("seeded but NOT selected:")
        .expect("region four")
        .1;
    assert!(
        !region_four.contains("… and"),
        "region four carries the diagnosis and truncates last:\n{region_four}"
    );
    assert!(region_four.contains("Withdrawn"), "{region_four}");

    for line in message.lines() {
        assert!(
            line.chars().count() <= 80,
            "over an 80-column terminal ({}): {line}",
            line.chars().count()
        );
    }
}

#[test]
fn long_event_type_wraps_without_truncating_type_or_position() {
    let message = caught(|| {
        let decision = block_on(async {
            given(model())
                .event(Ledger::Deposited)
                .unwrap()
                .event(Verbose)
                .unwrap()
                .when(|_: &Deposits| Ok::<_, core::convert::Infallible>(Vec::new()))
                .await
                .unwrap()
        });
        decision.then(&[DepositOnly::Deposited]);
    })
    .message;

    for line in message.lines() {
        assert!(
            line.chars().count() <= 80,
            "over an 80-column terminal ({}): {line}",
            line.chars().count()
        );
    }

    let squashed: String = message
        .chars()
        .filter(|held| !held.is_whitespace())
        .collect();
    assert!(
        squashed.contains(LONG.as_str()),
        "the event type was truncated to fit rather than wrapped:\n{message}"
    );

    let region_four = message
        .split_once("seeded but NOT selected:")
        .expect("region four")
        .1;
    for line in region_four
        .lines()
        .filter(|line| line.contains("not selected"))
    {
        assert!(
            line.starts_with("  not selected"),
            "the marker column moved: {line}"
        );
    }
}

// ---------------------------------------------------------------------------
// EC-006 and AC-007's residual
// ---------------------------------------------------------------------------

#[test]
fn then_on_a_refused_decision_panics_naming_the_refusal() {
    let message = caught(|| {
        let decision = block_on(async {
            given(model())
                .when(|_: &Deposits| Err::<Vec<DepositOnly>, _>(Refused))
                .await
                .unwrap()
        });
        // A refusal read as "emitted nothing" would pass here, silently.
        decision.then(&[]);
    })
    .message;

    assert!(
        message.contains("refused"),
        "a refusal compared against an empty slice passed:\n{message}"
    );
    assert!(
        message.contains("the account is closed"),
        "the refusal must be named, not categorised:\n{message}"
    );
}

#[test]
fn assert_domain_event_rejects_variants_permuted_against_event_types() {
    // Index 0 carries `Withdrawn` while EVENT_TYPES declares `Deposited`
    // there. Both names are declared, so the membership half passes and only
    // ADR-0059's positional precondition can see it.
    let message = caught(|| assert_domain_event(&[Permuted])).message;

    assert!(
        message.contains("index 0"),
        "the disagreeing index must be named:
{message}"
    );
    assert!(
        message.contains("ORDER disagreement"),
        "it must say this is an order problem, not a drifted name:
{message}"
    );
    assert!(
        message.contains("cannot tell you which side moved"),
        "the diagnostic's own limit must be stated, per ADR-0059:
{message}"
    );
}

#[test]
fn assert_domain_event_rejects_a_variant_outside_event_types() {
    let message = caught(|| assert_domain_event(&[Drifted])).message;

    assert!(
        message.contains("Withdrawn"),
        "the offending event type must be named:\n{message}"
    );
    assert!(
        message.contains("EVENT_TYPES"),
        "the declaration it is missing from must be named:\n{message}"
    );
}
