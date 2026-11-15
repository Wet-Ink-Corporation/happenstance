//! The report half of the rule: is what the instrument says it withholds what
//! it actually hides?
//!
//! The oracle shares no subroutine with the instrument (RS-60-4). It reads the
//! instrument through the port and takes the difference from what the inner
//! store assigned; the instrument consults its own withheld set. A wrong
//! instrument is one overridden default method of [`Defect`], selected by a
//! `PhantomData` marker (RS-60-3).

use core::cell::RefCell;
use core::marker::PhantomData;
use std::collections::BTreeSet;

use happenstance_core::{
    EventId, EventStore, MemoryStoreError, Query, ReadOptions, SequencePosition, SequencedEvent,
    collect,
};
use happenstance_testkit::fixtures::{MemoryFixture, MemoryHandle, tagged_event};
use happenstance_testkit::{Fixture, WithholdingStore, block_on};

/// One way an instrument can misstate what it withholds. The defaults are the
/// real instrument.
pub(crate) trait Defect: 'static {
    /// What the instrument reports as withheld.
    fn report(subject: &Subject<Self>) -> BTreeSet<SequencePosition>
    where
        Self: Sized,
    {
        subject.real.withheld()
    }

    /// Whether the instrument answers that it holds `id`.
    async fn contains(subject: &Subject<Self>, id: EventId) -> Result<bool, MemoryStoreError>
    where
        Self: Sized,
    {
        EventStore::contains_event_id(&subject.real, id).await
    }
}

/// The real instrument, with nothing overridden.
pub(crate) struct Correct;

impl Defect for Correct {}

/// An instrument under test: the real one, plus what a defect needs to lie.
pub(crate) struct Subject<D> {
    pub(crate) real: WithholdingStore<MemoryHandle>,
    /// A second handle onto the inner store, for the decorator that forgets to
    /// answer membership itself.
    pub(crate) inner: MemoryHandle,
    /// Every `withhold` call, in order, for the report that keeps only the last.
    pub(crate) calls: RefCell<Vec<Vec<SequencePosition>>>,
    defect: PhantomData<D>,
}

impl<D: Defect> Subject<D> {
    async fn open() -> Self {
        let memory = MemoryFixture::new();
        Self {
            real: WithholdingStore::new(memory.connect().await),
            inner: memory.connect().await,
            calls: RefCell::new(Vec::new()),
            defect: PhantomData,
        }
    }

    /// Appends one event per claim, one batch each, and returns the whole log.
    async fn seed(&self, claims: &[&str]) -> Vec<SequencedEvent> {
        for claim in claims {
            self.real
                .append(&[tagged_event("Claimed", &[("claim", claim)])], None)
                .await
                .expect("an unconditional append to a memory store lands");
        }
        collect(EventStore::read(
            &self.inner,
            &Query::all(),
            ReadOptions::new(),
        ))
        .await
        .expect("a memory store's reads do not fail")
    }

    async fn withhold(&self, positions: &[SequencePosition]) {
        self.calls.borrow_mut().push(positions.to_vec());
        self.real
            .withhold(positions)
            .await
            .expect("every position a scenario withholds was assigned");
    }
}

/// How an instrument's report and its behaviour disagree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Inaccuracy {
    /// Reported withheld, but the store never assigned it.
    ReportedButNeverAssigned(SequencePosition),
    /// Reported withheld, while `read` still yields it.
    ReportedWithheldButReadable(SequencePosition),
    /// `read` no longer yields it, but the report calls it retained.
    DroppedButReportedRetained(SequencePosition),
    /// `contains_event_id` disagrees with the report for this event.
    MembershipDisagreesWithReport {
        position: SequencePosition,
        answered: bool,
    },
    /// The instrument hid a different set from the one it was asked to hide.
    WithheldOtherThanAsked {
        asked: BTreeSet<SequencePosition>,
        hidden: BTreeSet<SequencePosition>,
    },
}

/// Which scenario caught an instrument, with the log it ran over, so a caller
/// can name the expected position by index rather than by literal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Rejection {
    pub(crate) scenario: Scenario,
    pub(crate) assigned: Vec<SequencePosition>,
    pub(crate) inaccuracy: Inaccuracy,
}

/// The four staged logs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Scenario {
    /// Six events; the first three withheld in one call.
    Suffix,
    /// Eight events; `[1]` and `[3]` in one call, `[5]` in a second. The
    /// survivors `[2]` and `[4]` sit between the holes.
    Scattered,
    /// Four events, nothing withheld.
    Nothing,
    /// Four events, all withheld.
    Everything,
}

impl Scenario {
    const ALL: [Self; 4] = [
        Self::Suffix,
        Self::Scattered,
        Self::Nothing,
        Self::Everything,
    ];

    fn claims(self) -> &'static [&'static str] {
        match self {
            Self::Suffix => &["a", "b", "a", "b", "a", "b"],
            Self::Scattered => &["a", "b", "a", "b", "a", "b", "a", "b"],
            Self::Nothing | Self::Everything => &["a", "b", "a", "b"],
        }
    }

    /// The withhold calls, as indexes into the assigned log.
    fn calls(self) -> &'static [&'static [usize]] {
        match self {
            Self::Suffix => &[&[0, 1, 2]],
            Self::Scattered => &[&[1, 3], &[5]],
            Self::Nothing => &[],
            Self::Everything => &[&[0, 1, 2, 3]],
        }
    }
}

/// Runs every scenario against `D`, returning the first inaccuracy found.
pub(crate) fn report_is_accurate<D: Defect>() -> Result<(), Rejection> {
    Scenario::ALL
        .into_iter()
        .try_for_each(|scenario| block_on(stage::<D>(scenario)))
}

async fn stage<D: Defect>(scenario: Scenario) -> Result<(), Rejection> {
    let subject = Subject::<D>::open().await;
    let assigned = subject.seed(scenario.claims()).await;
    let positions: Vec<SequencePosition> = assigned.iter().map(|event| event.position).collect();

    let mut asked = BTreeSet::new();
    for call in scenario.calls() {
        let chosen: Vec<SequencePosition> = call
            .iter()
            .map(|index| {
                *positions
                    .get(*index)
                    .expect("the scenario seeded this many")
            })
            .collect();
        asked.extend(chosen.iter().copied());
        subject.withhold(&chosen).await;
    }

    check(&subject, &assigned, &asked)
        .await
        .map_err(|inaccuracy| Rejection {
            scenario,
            assigned: positions,
            inaccuracy,
        })
}

/// The oracle. `assigned` is the inner store's whole log, read before anything
/// was withheld; `asked` is the union of every position passed to `withhold`.
pub(crate) async fn check<D: Defect>(
    subject: &Subject<D>,
    assigned: &[SequencedEvent],
    asked: &BTreeSet<SequencePosition>,
) -> Result<(), Inaccuracy> {
    let readable: BTreeSet<SequencePosition> = collect(EventStore::read(
        &subject.real,
        &Query::all(),
        ReadOptions::new(),
    ))
    .await
    .expect("a memory store's reads do not fail")
    .iter()
    .map(|event| event.position)
    .collect();
    let all: BTreeSet<SequencePosition> = assigned.iter().map(|event| event.position).collect();
    let hidden: BTreeSet<SequencePosition> = all.difference(&readable).copied().collect();
    let report = D::report(subject);

    if let Some(&position) = report.iter().find(|p| !all.contains(p)) {
        return Err(Inaccuracy::ReportedButNeverAssigned(position));
    }
    if let Some(&position) = report.iter().find(|p| !hidden.contains(p)) {
        return Err(Inaccuracy::ReportedWithheldButReadable(position));
    }
    if let Some(&position) = hidden.iter().find(|p| !report.contains(p)) {
        return Err(Inaccuracy::DroppedButReportedRetained(position));
    }
    for event in assigned {
        let answered = D::contains(subject, event.id)
            .await
            .expect("a memory store's membership does not fail");
        if answered == report.contains(&event.position) {
            return Err(Inaccuracy::MembershipDisagreesWithReport {
                position: event.position,
                answered,
            });
        }
    }
    if hidden != *asked {
        return Err(Inaccuracy::WithheldOtherThanAsked {
            asked: asked.clone(),
            hidden,
        });
    }
    Ok(())
}

/// Any log of up to twelve events, any withheld mask, any split of the mask
/// over up to three calls: the real instrument's report is accurate.
#[cfg(feature = "proptest")]
mod properties {
    use std::collections::BTreeSet;

    use happenstance_testkit::block_on;
    use proptest::collection::vec;
    use proptest::prelude::*;

    use super::{Correct, Subject, check};

    /// A mask over `n` events, and for each event the call (of three) that
    /// would withhold it.
    fn masks() -> impl Strategy<Value = Vec<(bool, usize)>> {
        (1usize..=12).prop_flat_map(|n| vec((any::<bool>(), 0usize..3), n))
    }

    proptest! {
        #[test]
        fn any_withheld_set_split_over_any_calls_is_reported_accurately(mask in masks()) {
            let verdict = block_on(async {
                let subject = Subject::<Correct>::open().await;
                let claims: Vec<&str> = mask.iter().map(|_| "p").collect();
                let assigned = subject.seed(&claims).await;

                let mut asked = BTreeSet::new();
                for call in 0..3 {
                    let chosen: Vec<_> = assigned
                        .iter()
                        .zip(&mask)
                        .filter(|(_, (withheld, at))| *withheld && *at == call)
                        .map(|(event, _)| event.position)
                        .collect();
                    if !chosen.is_empty() {
                        asked.extend(chosen.iter().copied());
                        subject.withhold(&chosen).await;
                    }
                }
                check(&subject, &assigned, &asked).await
            });
            prop_assert_eq!(verdict, Ok(()));
        }
    }
}
