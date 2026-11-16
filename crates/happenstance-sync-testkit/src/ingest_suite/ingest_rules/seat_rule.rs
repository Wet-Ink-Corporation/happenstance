//! The seat family's merge rule, written through `happenstance-sync`'s shipped
//! [`Adjudicate`] hook, which SY-7's and SY-10's arrangements mount (#450).
//!
//! # Why the rules mount the shipped hook
//!
//! The runner's half of SY-7 and SY-10 is `happenstance-sync`'s to prove, and
//! its runner tests do, against wrong runners. What these rules hold a store
//! to is its half: it writes what the runner hands it, the compensation under
//! its own identity, and nothing of its own. They hand it what a runner
//! carrying this rule, or carrying [`Accept`](happenstance_sync::runner::Accept),
//! would hand it, so a rule here judges exactly as a runner mounting the same
//! hook does.

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use happenstance_core::{Event, EventStore, Query, ReadOptions, collect};
use happenstance_sync::runner::Adjudicate;
use happenstance_sync::{Classify, EventGroup, FailureClass};

/// One event by type and bytes: how the rule tells two claims apart.
type Fact = (String, Vec<u8>);

/// The store's read failed, so the rule could not decide.
#[derive(Debug)]
pub(super) struct Unjudged(String);

impl core::fmt::Display for Unjudged {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "the merge rule could not read the receiving store: {}",
            self.0
        )
    }
}

impl core::error::Error for Unjudged {}

/// A rule that cannot read the store it judges for cannot be retried into
/// reading it.
impl Classify for Unjudged {
    fn class(&self) -> FailureClass {
        FailureClass::Stop
    }
}

/// The hub's rule for the seat family: a group carrying a claim the family
/// `family` matches earns `reversal` where the receiving store, or a group
/// before it in the same batch, already holds another claim on the seat.
///
/// Judged by fact family (SY-35), by type, tags and bytes, never decoded, and
/// never from the group's guard, which is evidence only (SY-6).
#[derive(Debug)]
pub(super) struct SeatRule<'a, S> {
    store: &'a S,
    family: &'a Query,
    reversal: &'a Event,
}

impl<'a, S> SeatRule<'a, S> {
    /// The rule for what `store` takes, answering with `reversal`.
    pub(super) const fn new(store: &'a S, family: &'a Query, reversal: &'a Event) -> Self {
        Self {
            store,
            family,
            reversal,
        }
    }

    /// The claims in `group` the family matches, by type and bytes.
    fn claims_in(&self, group: &EventGroup) -> Vec<Fact> {
        group
            .events
            .iter()
            .filter(|event| {
                self.family
                    .matches(event.event.event_type(), event.event.tags())
            })
            .map(|event| {
                (
                    String::from(event.event.event_type().as_str()),
                    event.event.data().to_vec(),
                )
            })
            .collect()
    }
}

impl<S: EventStore> Adjudicate for SeatRule<'_, S> {
    type Error = Unjudged;

    async fn compensation(
        &self,
        group: &EventGroup,
        earlier: &[EventGroup],
    ) -> Result<Vec<Event>, Unjudged> {
        let incoming = self.claims_in(group);
        if incoming.is_empty() {
            return Ok(Vec::new());
        }
        let held = collect(self.store.read(self.family, ReadOptions::new()))
            .await
            .map_err(|error| Unjudged(format!("{error}")))?;
        let other = |fact: &Fact| !incoming.contains(fact);
        let contested = held.iter().any(|row| {
            other(&(
                String::from(row.event_type().as_str()),
                row.event.data().to_vec(),
            ))
        }) || earlier
            .iter()
            .any(|group| self.claims_in(group).iter().any(other));
        Ok(if contested {
            // The trait hands back an owned answer; the reversal's payload is
            // a reference-counted `Bytes`, so the copy is a count, not a copy.
            Vec::from([self.reversal.clone()])
        } else {
            Vec::new()
        })
    }
}
