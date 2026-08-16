//! The DSL's own unit tests.
//!
//! They live inside the module because three of them assert on things a test
//! author cannot see: the store `given` created, the typed refusal a `Decision`
//! is carrying, and the `#[must_use]` attribute's own source text. The
//! user-observable half — the four-region message, the retry demonstration and
//! the mount — is in `tests/`.

#![allow(clippy::unwrap_used)]

use core::convert::Infallible;
use std::sync::Arc;

use happenstance_core::bytes::Bytes;
use happenstance_core::{EventStore, EventType, Query, ReadOptions, Tags, collect};

use super::{DECISION_MUST_USE, Outcome, assert_domain_event, given};
use crate::codec::{Codec, CodecError, Json};
use crate::command::CommandError;
use crate::domain::{DecisionModel, DomainEvent};

// ---------------------------------------------------------------------------
// A domain small enough to reason about and large enough to disagree with
// itself
// ---------------------------------------------------------------------------

const DEPOSITED: EventType = EventType::from_static("Deposited");
const WITHDRAWN: EventType = EventType::from_static("Withdrawn");
const AUDITED: EventType = EventType::from_static("Audited");

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
enum Ledger {
    Deposited { account: String },
    Withdrawn { account: String },
}

impl Ledger {
    fn account(&self) -> &str {
        match self {
            Self::Deposited { account } | Self::Withdrawn { account } => account,
        }
    }
}

impl DomainEvent for Ledger {
    const EVENT_TYPES: &'static [EventType] = &[DEPOSITED, WITHDRAWN];

    fn event_type(&self) -> EventType {
        match self {
            Self::Deposited { .. } => DEPOSITED,
            Self::Withdrawn { .. } => WITHDRAWN,
        }
    }

    fn tags(&self) -> Tags {
        account_tags(self.account())
    }

    fn encode<C: Codec>(&self, codec: &C) -> Result<Bytes, CodecError> {
        codec.encode(self)
    }

    fn decode<C: Codec>(codec: &C, _t: &EventType, data: &Bytes) -> Result<Self, CodecError> {
        codec.decode(data)
    }
}

/// A type declaring `Deposited` whose payload no `Ledger` can be built from.
///
/// The instrument for EC-002: an event the derived query **nominates** and the
/// fold cannot decode.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Foreign {
    amount: u32,
    account: String,
}

impl DomainEvent for Foreign {
    const EVENT_TYPES: &'static [EventType] = &[DEPOSITED];

    fn event_type(&self) -> EventType {
        DEPOSITED
    }

    fn tags(&self) -> Tags {
        account_tags(&self.account)
    }

    fn encode<C: Codec>(&self, codec: &C) -> Result<Bytes, CodecError> {
        codec.encode(self)
    }

    fn decode<C: Codec>(codec: &C, _t: &EventType, data: &Bytes) -> Result<Self, CodecError> {
        codec.decode(data)
    }
}

/// A type no `Ledger` boundary ever nominates.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Audited {
    account: String,
}

impl DomainEvent for Audited {
    const EVENT_TYPES: &'static [EventType] = &[AUDITED];

    fn event_type(&self) -> EventType {
        AUDITED
    }

    fn tags(&self) -> Tags {
        account_tags(&self.account)
    }

    fn encode<C: Codec>(&self, codec: &C) -> Result<Bytes, CodecError> {
        codec.encode(self)
    }

    fn decode<C: Codec>(codec: &C, _t: &EventType, data: &Bytes) -> Result<Self, CodecError> {
        codec.decode(data)
    }
}

#[derive(Debug, Clone)]
struct Account {
    scope: Tags,
    deposits: u32,
    withdrawals: u32,
}

impl DecisionModel for Account {
    type Event = Ledger;

    fn scope(&self) -> &Tags {
        &self.scope
    }

    fn apply(&mut self, event: Self::Event) {
        match event {
            Ledger::Deposited { .. } => self.deposits += 1,
            Ledger::Withdrawn { .. } => self.withdrawals += 1,
        }
    }
}

/// The caller's own refusal, carried typed the whole way.
#[derive(Debug, PartialEq, Eq)]
struct Overdrawn {
    account: String,
}

impl core::fmt::Display for Overdrawn {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "account {} is overdrawn", self.account)
    }
}

impl core::error::Error for Overdrawn {}

fn account_tags(account: &str) -> Tags {
    Tags::from_pairs([("account", account)]).expect("a valid tag pair")
}

fn account(name: &str) -> Account {
    Account {
        scope: account_tags(name),
        deposits: 0,
        withdrawals: 0,
    }
}

fn deposit(name: &str) -> Ledger {
    Ledger::Deposited {
        account: name.to_owned(),
    }
}

fn withdrawal(name: &str) -> Ledger {
    Ledger::Withdrawn {
        account: name.to_owned(),
    }
}

// ---------------------------------------------------------------------------
// AC-001 — the shape a test author writes
// ---------------------------------------------------------------------------

#[tokio::test]
async fn seeded_then_decided_asserts_emitted_events() {
    given(account("a1"))
        .event(deposit("a1"))
        .unwrap()
        .event(withdrawal("a1"))
        .unwrap()
        .when(|held: &Account| {
            assert_eq!(held.deposits, 1, "the fold ran over the seeded log");
            assert_eq!(held.withdrawals, 1);
            Ok::<_, Infallible>(vec![deposit("a1")])
        })
        .await
        .unwrap()
        .then(&[deposit("a1")]);
}

/// A tuple is a boundary, so the same call shape composes.
#[tokio::test]
async fn composed_boundary_folds_both_models() {
    given((account("a1"), account("a2")))
        .event(deposit("a1"))
        .unwrap()
        .event(deposit("a2"))
        .unwrap()
        .when(|(first, second): &(Account, Account)| {
            // Each member folded only what its own query nominated. Handing
            // every arriving event to every member would make both read 2.
            assert_eq!(first.deposits, 1, "the a1 member folded a2's event");
            assert_eq!(second.deposits, 1, "the a2 member folded a1's event");
            Ok::<_, Infallible>(vec![deposit("a1")])
        })
        .await
        .unwrap()
        .then(&[deposit("a1")]);
}

// ---------------------------------------------------------------------------
// AC-006 — the refusal path
// ---------------------------------------------------------------------------

#[tokio::test]
async fn refusal_is_asserted_through_then_refused() {
    let decision = given(account("a1"))
        .event(deposit("a1"))
        .unwrap()
        .when(|_: &Account| {
            Err::<Vec<Ledger>, _>(Overdrawn {
                account: "a1".to_owned(),
            })
        })
        .await
        .unwrap();

    // The caller's own type survived the round trip: it is a value to downcast,
    // not a rendered string.
    match &decision.outcome {
        Outcome::Refused(refusal) => {
            let held = refusal
                .downcast_ref::<Overdrawn>()
                .expect("the refusal is still an Overdrawn");
            assert_eq!(held.account, "a1");
        }
        other @ Outcome::Emitted(_) => panic!("the refusal was not carried: {other:?}"),
    }

    decision.then_refused();
}

/// A refusal is not a store failure, and does not travel as one.
#[tokio::test]
async fn refusal_does_not_route_through_when_err() {
    let outcome = given(account("a1"))
        .when(|_: &Account| {
            Err::<Vec<Ledger>, _>(Overdrawn {
                account: "a1".to_owned(),
            })
        })
        .await;

    assert!(
        outcome.is_ok(),
        "`when`'s Err arm is store and codec failure only"
    );
    outcome.unwrap().then_refused();
}

// ---------------------------------------------------------------------------
// AC-007 — seeding is the mapping production uses
// ---------------------------------------------------------------------------

#[tokio::test]
async fn seeded_bytes_are_the_bytes_commit_writes() {
    let value = deposit("a1");
    let seeded = given(account("a1")).event(value.clone()).unwrap();
    let store = Arc::clone(&seeded.store);

    seeded
        .when(|_: &Account| Ok::<_, Infallible>(Vec::new()))
        .await
        .unwrap()
        .then(&[]);

    let held = collect(store.read(&Query::all(), ReadOptions::new()))
        .await
        .unwrap();
    let written = held.first().expect("one seeded event");

    // Compared against the codec's own output rather than a literal: a literal
    // would pin the encoding rather than the agreement between the two paths.
    assert_eq!(written.event.data(), &Json.encode(&value).unwrap());
    assert_eq!(
        written.event.metadata(),
        Some(&crate::codec::frame::<Json>(None)),
        "the framing region `commit` writes is missing from the seeded event"
    );
    assert_eq!(written.event.event_type(), &DEPOSITED);
    assert_eq!(written.event.tags(), &account_tags("a1"));
}

#[tokio::test]
async fn nominated_but_undecodable_event_is_an_error() {
    let outcome = given(account("a1"))
        .event(Foreign {
            amount: 3,
            account: "a1".to_owned(),
        })
        .unwrap()
        .when(|_: &Account| Ok::<_, Infallible>(Vec::new()))
        .await;

    match outcome {
        Err(CommandError::Decode { source, .. }) => {
            assert!(
                matches!(source, CodecError::Decode(_)),
                "a nominated event the fold cannot decode is a real disagreement \
                 between EVENT_TYPES and the fold, got {source:?}"
            );
        }
        other => panic!("the disagreement was swallowed: {other:?}"),
    }
}

#[tokio::test]
async fn non_nominated_type_is_skipped_not_an_error() {
    given(account("a1"))
        .event(Audited {
            account: "a1".to_owned(),
        })
        .unwrap()
        .event(deposit("a1"))
        .unwrap()
        .when(|held: &Account| {
            assert_eq!(held.deposits, 1);
            assert_eq!(
                held.withdrawals, 0,
                "an event the query never nominated reached the fold"
            );
            Ok::<_, Infallible>(Vec::new())
        })
        .await
        .unwrap()
        .then(&[]);
}

// ---------------------------------------------------------------------------
// AC-008 — everything before the append is pure
// ---------------------------------------------------------------------------

#[tokio::test]
async fn dropping_a_decision_appends_nothing() {
    let seeded = given(account("a1")).event(deposit("a1")).unwrap();
    let store = Arc::clone(&seeded.store);

    let decision = seeded
        .when(|_: &Account| Ok::<_, Infallible>(vec![deposit("a1"), withdrawal("a1")]))
        .await
        .unwrap();

    let before = collect(store.read(&Query::all(), ReadOptions::new()))
        .await
        .unwrap();
    drop(decision);
    let after = collect(store.read(&Query::all(), ReadOptions::new()))
        .await
        .unwrap();

    assert_eq!(before.len(), 1, "only the seeded event is in the store");
    assert_eq!(
        before.iter().map(|e| e.position).collect::<Vec<_>>(),
        after.iter().map(|e| e.position).collect::<Vec<_>>(),
        "the decision's two events were appended; the append is the one \
         irreversible act and a Decision has not performed it"
    );
}

/// The `#[must_use]` text is the one the design authored, character for
/// character.
///
/// The attribute takes a literal, so the sentence is written twice; this reads
/// the source back and compares them, which is the only way a `const` can guard
/// an attribute.
#[test]
fn must_use_message_is_verbatim() {
    const OPENS: &str = "#[must_use = \"a Decision";

    let source = include_str!("mod.rs");
    let start = source.find(OPENS).expect("the attribute is still there") + "#[must_use = \"".len();
    let rest = &source[start..];
    let end = rest.find("\"]").expect("the literal is still closed");

    // A `\` at end of line swallows the newline and the indentation after it.
    let written: String = rest[..end]
        .split('\\')
        .enumerate()
        .map(|(index, part)| {
            if index == 0 {
                part.to_owned()
            } else {
                part.trim_start().to_owned()
            }
        })
        .collect();

    assert_eq!(written, DECISION_MUST_USE);
    assert!(
        written.contains("dropping it discards the events the decision produced"),
        "the lint's own sentence is what a caller reads: {written}"
    );
}

/// The residual `assert_domain_event` measures, over a type that agrees.
#[test]
fn assert_domain_event_accepts_every_declared_variant() {
    assert_domain_event(&[deposit("a1"), withdrawal("a1")]);
}
