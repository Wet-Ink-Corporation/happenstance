//! PS-26, PS-29 and PS-30, each against the shipped runner and the wrong
//! fan-out runners it rejects.
//!
//! A rule no runner can fail is decorative. Each rule here is written over a
//! [`FanRunner`], so it runs against `run_fan_out` **and** against
//! [`MiniFan`], a fan-out runner over the public API alone that carries one
//! [`Defect`] and is otherwise correct for the rule's fixtures.
//!
//! PS-30 binds a runner that catches a panic. The shipped runner catches
//! none (O1, option B), so it meets the clause by unwinding, and the rule
//! holds it to the observable any runner owes: no row the panicking view
//! wrote since its last commit, no checkpoint moved, and its write set
//! ended before anything else commits. Its three mutants are the population
//! the MUST does bind: runners that catch.

use core::num::NonZeroUsize;
use std::collections::BTreeSet;
// Test-only, and the instrument rather than an escape: the rules observe a
// panic from outside the runner (`Shipped`) and the mutants must catch one
// to be the runners PS-30 binds (`MiniFan::take`). `src/` catches nothing,
// which `projection_clauses.rs`'s tripwire holds.
use std::panic::{AssertUnwindSafe, catch_unwind};

use happenstance::{
    ApplyFailure, Authority, Checkpoint, Chunk, Delivered, DomainEvent, EventStore, Json,
    MemoryEventStore, Policy, Projection, ProjectionStore, Query, QueryItem, ReadOptions,
    SequencePosition, SequencedEvent, ViewError, checkpoint_id, collect, run_fan_out,
};
use happenstance_testkit::{BatchCall, RecordingBatch, RecordingProjectionStore, block_on};

use crate::support::{Part, Shard, ShardError, checkpoint_of, part, poisoned};

/// One view's outcome as the rule reads it: `Err` names the position the
/// view stopped at, where it names one.
type Outcome = Result<(), Option<SequencePosition>>;

/// What became of a fan-out run.
#[derive(Debug, PartialEq, Eq)]
enum Ran {
    /// It returned, with one outcome per view.
    Returned(Vec<Outcome>),
    /// A panic unwound out of it.
    Unwound,
}

/// A fan-out runner over the memory event store and [`RecordingProjectionStore`].
trait FanRunner {
    fn run(
        &self,
        events: &MemoryEventStore,
        models: &RecordingProjectionStore,
        views: &mut [Shard],
        size: NonZeroUsize,
    ) -> Ran;
}

/// The runner this crate ships.
///
/// The `AssertUnwindSafe` is honest because what the rule inspects after an
/// unwind is the stores, and each leg builds them fresh: nothing that may
/// have been left half-mutated is reused.
struct Shipped;

impl FanRunner for Shipped {
    fn run(
        &self,
        events: &MemoryEventStore,
        models: &RecordingProjectionStore,
        views: &mut [Shard],
        size: NonZeroUsize,
    ) -> Ran {
        let ran = catch_unwind(AssertUnwindSafe(|| {
            block_on(run_fan_out(events, models, views, &Json, Chunk::of(size)))
        }));
        match ran {
            Ok(fan) => {
                assert!(fan.read.is_none(), "the memory read failed");
                Ran::Returned(
                    fan.views
                        .iter()
                        .map(|outcome| outcome.as_ref().map(|_| ()).map_err(ViewError::position))
                        .collect(),
                )
            }
            Err(_unwound) => Ran::Unwound,
        }
    }
}

// ---------------------------------------------------------------------------
// The wrong runners
// ---------------------------------------------------------------------------

/// One defect per wrong runner, otherwise correct (CF-3).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Defect {
    /// No defect: catches a panic, rolls the write set back at once, and
    /// stops the view. The positive control, so a rule that every catching
    /// runner fails cannot pass for strict.
    Absent,
    /// Catches a panic, keeps the write set the view panicked in, and
    /// carries on with the view as if nothing happened.
    CarriesOn,
    /// Catches a panic, stops the view, and drops its write set bare.
    DropsInsteadOfRollingBack,
    /// Catches a panic, stops the view, and rolls its write set back only
    /// once every other view has committed.
    RollsBackAtTheEnd,
    /// Asks the first view's `on_error` about every view's failure.
    AsksTheFirstViewForEveryone,
    /// Stops every view when any one of them halts.
    StopsEveryView,
}

/// Whether one of a wrong runner's views is still taking events.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Lane {
    Running,
    Stopped(Option<SequencePosition>),
}

/// A view's open write set, and the last position it took.
struct Open {
    batch: RecordingBatch,
    taken: usize,
    last: SequencePosition,
}

/// A fan-out runner over the public API alone, carrying one [`Defect`].
///
/// It collects the log, which the shipped runner never does, because the
/// rule's fixtures are three events long and the defect is the subject.
struct MiniFan(Defect);

/// What taking one event came to.
enum Took {
    Taken,
    Halted,
    Panicked,
}

impl MiniFan {
    /// Decodes and applies one event, offering a failure to `ask`, and
    /// catching a panic per call.
    fn take(
        view: &mut Shard,
        ask: Option<&mut Shard>,
        event: &SequencedEvent,
        batch: &mut RecordingBatch,
    ) -> Took {
        let id = event.id;
        // Catching is this mutant's subject: PS-30 binds a runner that
        // catches, so the wrong runners must catch to be wrong in its terms.
        // `AssertUnwindSafe` holds because a panicked view is never offered
        // another event (every defect but `CarriesOn`, whose carrying on is
        // the defect), and its write set is what the rule inspects.
        let caught = catch_unwind(AssertUnwindSafe(|| {
            block_on(async {
                match Part::decode(&Json, event.event.event_type(), event.event.data()) {
                    Ok(part) => match view.apply(Delivered::new(id, part), batch).await {
                        Ok(()) => Took::Taken,
                        Err(source) => {
                            let failure = ApplyFailure::apply(id, &source);
                            Self::offer(view, ask, &failure, batch).await
                        }
                    },
                    Err(source) => {
                        let failure = ApplyFailure::decode(id, &source);
                        Self::offer(view, ask, &failure, batch).await
                    }
                }
            })
        }));
        caught.unwrap_or(Took::Panicked)
    }

    /// Asks `ask`, or the view itself when there is no `ask`, what to do.
    async fn offer(
        view: &mut Shard,
        ask: Option<&mut Shard>,
        failure: &ApplyFailure<'_, ShardError>,
        batch: &mut RecordingBatch,
    ) -> Took {
        let policy = match ask {
            Some(first) => first.on_error(failure, batch).await,
            None => view.on_error(failure, batch).await,
        };
        match policy {
            Ok(Policy::Skip) => Took::Taken,
            // `Halt`, a failing policy, and whatever `Policy` grows.
            _ => Took::Halted,
        }
    }

    fn commit(models: &RecordingProjectionStore, view: &Shard, open: Open) {
        let key = checkpoint_id(view).expect("a derivable id");
        block_on(models.commit(open.batch, &key, open.last, Authority::Live))
            .expect("the watched store commits");
    }
}

impl FanRunner for MiniFan {
    fn run(
        &self,
        events: &MemoryEventStore,
        models: &RecordingProjectionStore,
        views: &mut [Shard],
        size: NonZeroUsize,
    ) -> Ran {
        let queries: Vec<Query> = views
            .iter()
            .map(|view| {
                let item = QueryItem::new(Part::EVENT_TYPES.iter().cloned(), view.scope().clone())
                    .expect("a derivable query");
                Query::from_item(item)
            })
            .collect();
        for view in views.iter() {
            let at = block_on(checkpoint_of(models, view));
            assert_eq!(at, Checkpoint::NeverRun, "the rule's fixtures start fresh");
        }
        let log = block_on(collect(EventStore::read(
            events,
            &Query::all(),
            ReadOptions::new(),
        )))
        .expect("the memory log reads");

        let mut fan = Fan {
            lanes: vec![Lane::Running; views.len()],
            open: views.iter().map(|_| None).collect(),
            parked: Vec::new(),
        };
        for event in &log {
            for (at, query) in queries.iter().enumerate() {
                if fan.lanes[at] == Lane::Running
                    && query.matches(event.event.event_type(), event.event.tags())
                {
                    self.step(&mut fan, models, views, at, event, size);
                }
            }
        }
        fan.finish(models, views)
    }
}

/// A wrong runner's views in flight: which still run, what each holds open,
/// and the write sets a [`Defect::RollsBackAtTheEnd`] runner has parked.
struct Fan {
    lanes: Vec<Lane>,
    open: Vec<Option<Open>>,
    parked: Vec<RecordingBatch>,
}

impl Fan {
    /// Commits every open write set, then ends the parked ones.
    fn finish(mut self, models: &RecordingProjectionStore, views: &[Shard]) -> Ran {
        for (held, view) in self.open.iter_mut().zip(views) {
            if let Some(held) = held.take() {
                MiniFan::commit(models, view, held);
            }
        }
        for batch in self.parked {
            block_on(models.rollback(batch)).expect("a buffer drops");
        }
        Ran::Returned(
            self.lanes
                .iter()
                .map(|lane| match lane {
                    Lane::Running => Ok(()),
                    Lane::Stopped(at) => Err(*at),
                })
                .collect(),
        )
    }
}

impl MiniFan {
    /// Offers one event to view `at`, which runs and whose query matches.
    fn step(
        &self,
        fan: &mut Fan,
        models: &RecordingProjectionStore,
        views: &mut [Shard],
        at: usize,
        event: &SequencedEvent,
        size: NonZeroUsize,
    ) {
        let (mut batch, taken) = match fan.open[at].take() {
            Some(held) => (held.batch, held.taken),
            None => (block_on(models.begin()).expect("a buffer opens"), 0),
        };
        let took = self.take_at(views, at, event, &mut batch);
        let held = Open {
            batch,
            taken: taken + 1,
            last: event.position,
        };
        match took {
            Took::Taken if held.taken == size.get() => Self::commit(models, &views[at], held),
            Took::Taken => fan.open[at] = Some(held),
            Took::Halted => self.halt(fan, models, at, held),
            Took::Panicked => self.unwound(fan, models, at, held),
        }
    }

    /// Takes the event into view `at`, asking the first view's policy for
    /// it under [`Defect::AsksTheFirstViewForEveryone`].
    fn take_at(
        &self,
        views: &mut [Shard],
        at: usize,
        event: &SequencedEvent,
        batch: &mut RecordingBatch,
    ) -> Took {
        let (first, rest) = views.split_first_mut().expect("at least one view");
        match at.checked_sub(1) {
            None => Self::take(first, None, event, batch),
            Some(below) => {
                let ask = (self.0 == Defect::AsksTheFirstViewForEveryone).then_some(first);
                Self::take(&mut rest[below], ask, event, batch)
            }
        }
    }

    /// View `at` halted on `held.last`: it stops, and under
    /// [`Defect::StopsEveryView`] every other view stops with it.
    fn halt(&self, fan: &mut Fan, models: &RecordingProjectionStore, at: usize, held: Open) {
        let stop = Lane::Stopped(Some(held.last));
        block_on(models.rollback(held.batch)).expect("a buffer drops");
        fan.lanes[at] = stop;
        if self.0 == Defect::StopsEveryView {
            for (lane, other) in fan.lanes.iter_mut().zip(fan.open.iter_mut()) {
                if let Some(other) = other.take() {
                    block_on(models.rollback(other.batch)).expect("a buffer drops");
                }
                *lane = stop;
            }
        }
    }

    /// View `at` panicked on `held.last`, and this runner caught it.
    fn unwound(&self, fan: &mut Fan, models: &RecordingProjectionStore, at: usize, held: Open) {
        let stop = Lane::Stopped(Some(held.last));
        match self.0 {
            Defect::CarriesOn => fan.open[at] = Some(held),
            Defect::DropsInsteadOfRollingBack => {
                drop(held);
                fan.lanes[at] = stop;
            }
            Defect::RollsBackAtTheEnd => {
                fan.parked.push(held.batch);
                fan.lanes[at] = stop;
            }
            // The right thing, which is to roll back at once.
            Defect::Absent | Defect::AsksTheFirstViewForEveryone | Defect::StopsEveryView => {
                block_on(models.rollback(held.batch)).expect("a buffer drops");
                fan.lanes[at] = stop;
            }
        }
    }
}

/// The legs a rule failed, read off its `Err` message, each once, in order.
fn legs(result: Result<(), String>) -> Vec<char> {
    let Err(message) = result else {
        return Vec::new();
    };
    let legs: BTreeSet<char> = message
        .lines()
        .filter_map(|line| line.strip_prefix("leg "))
        .filter_map(|rest| rest.chars().next())
        .collect();
    assert!(!legs.is_empty(), "the rule failed unparseably: {message}");
    legs.into_iter().collect()
}

fn chunk_of(n: usize) -> NonZeroUsize {
    NonZeroUsize::new(n).expect("a non-zero chunk")
}

// ---------------------------------------------------------------------------
// PS-30 — `panicking_apply_rolls_back`
// ---------------------------------------------------------------------------

/// PS-30's rule, against any runner, swept over chunk 1 to 3. `Err` names
/// every leg that failed, at every chunk it failed at.
///
/// Views `a`, `boom` and `c` over `[VIN1, BOOM, VIN2]`; `boom` writes its
/// row for `BOOM`, logs a `Marked` with its write set open, and panics.
///
/// - **A**: no row `boom` wrote since its last commit survives.
/// - **B**: `boom`'s checkpoint is its last commit before the panic.
/// - **C**: the write set `boom` panicked in ends, by a rollback or a bare
///   drop, before anything logged after the panic commits.
/// - **D**: if the run returned, that write set ended in a rollback rather
///   than a bare drop.
fn panicking_apply_holds(runner: &impl FanRunner) -> Result<(), String> {
    let mut failed = Vec::new();
    for size in 1..=3 {
        let events = MemoryEventStore::new();
        block_on(EventStore::append(
            &events,
            &[part("VIN1"), part("BOOM"), part("VIN2")],
            None,
        ))
        .expect("the fixture seeds");
        let read = events.snapshot();
        let models = RecordingProjectionStore::new();
        let mut views = [
            Shard::new("a"),
            Shard::new("boom").panicking_on("BOOM"),
            Shard::new("c"),
        ];
        let ran = runner.run(&events, &models, &mut views, chunk_of(size));

        let committed_before = (size == 1).then(|| read[0].position);
        let rows: Vec<String> = models
            .snapshot()
            .into_keys()
            .filter(|key| key.starts_with("boom:"))
            .collect();
        let want_rows: Vec<String> = if size == 1 {
            vec!["boom:VIN1".to_owned()]
        } else {
            Vec::new()
        };
        if rows != want_rows {
            failed.push(format!(
                "leg A at chunk {size}: boom's rows are {rows:?}; expected {want_rows:?}"
            ));
        }

        let at = block_on(checkpoint_of(&models, &views[1]));
        let want =
            committed_before.map_or(Checkpoint::NeverRun, |through| Checkpoint::Live { through });
        if at != want {
            failed.push(format!(
                "leg B at chunk {size}: boom's checkpoint is {at:?}; expected {want:?}"
            ));
        }

        let calls = models.calls();
        let Some(panicked) = calls.iter().position(|c| matches!(c, BatchCall::Marked(_))) else {
            failed.push(format!(
                "leg A at chunk {size}: boom never panicked: {calls:?}"
            ));
            continue;
        };
        let serial = calls[panicked].serial();
        let after = &calls[panicked..];
        let ended = after.iter().position(|c| c.serial() == serial && c.ends());
        let commit = after.iter().position(|c| matches!(c, BatchCall::Commit(_)));
        let in_time = match (ended, commit) {
            (Some(ended), Some(commit)) => {
                ended < commit && !matches!(after[ended], BatchCall::Commit(_))
            }
            (Some(ended), None) => !matches!(after[ended], BatchCall::Commit(_)),
            (None, _) => false,
        };
        if !in_time {
            failed.push(format!(
                "leg C at chunk {size}: write set {serial} did not end before the next \
                 commit: {calls:?}"
            ));
        }

        if matches!(ran, Ran::Returned(_))
            && ended.map(|ended| after[ended]) != Some(BatchCall::Rollback(serial))
        {
            failed.push(format!(
                "leg D at chunk {size}: the run returned and write set {serial} was not \
                 rolled back: {calls:?}"
            ));
        }
    }
    if failed.is_empty() {
        Ok(())
    } else {
        Err(failed.join("\n"))
    }
}

#[test]
fn panicking_apply_rolls_back() {
    assert_eq!(panicking_apply_holds(&Shipped), Ok(()));
}

#[test]
fn every_rule_accepts_a_correct_catching_runner() {
    assert_eq!(panicking_apply_holds(&MiniFan(Defect::Absent)), Ok(()));
    assert_eq!(
        per_projection_policy_holds(&MiniFan(Defect::Absent)),
        Ok(())
    );
    assert_eq!(one_poisoned_view_holds(&MiniFan(Defect::Absent)), Ok(()));
}

#[test]
fn panicking_apply_rolls_back_rejects_a_runner_that_carries_on() {
    // The row survives, the checkpoint moves past the panic, and the write
    // set is committed after the panic rather than ended, so it was never
    // rolled back either.
    assert_eq!(
        legs(panicking_apply_holds(&MiniFan(Defect::CarriesOn))),
        ['A', 'B', 'C', 'D']
    );
}

#[test]
fn panicking_apply_rolls_back_rejects_a_dropped_batch() {
    // The port rolls a dropped batch back, so nothing durable is wrong; the
    // runner returned without the rollback the clause requires.
    assert_eq!(
        legs(panicking_apply_holds(&MiniFan(
            Defect::DropsInsteadOfRollingBack
        ))),
        ['D']
    );
}

#[test]
fn panicking_apply_rolls_back_rejects_a_late_rollback() {
    // Rolled back, but only after the other views committed past the panic.
    assert_eq!(
        legs(panicking_apply_holds(&MiniFan(Defect::RollsBackAtTheEnd))),
        ['C']
    );
}

// ---------------------------------------------------------------------------
// PS-26 — `failure_policy_is_per_projection`
// ---------------------------------------------------------------------------

/// PS-26's rule: two views in one run, one halting and one skipping, fed
/// one failing event. The first stops there; the second advances past it.
fn per_projection_policy_holds(runner: &impl FanRunner) -> Result<(), String> {
    let events = MemoryEventStore::new();
    block_on(EventStore::append(
        &events,
        &[part("VIN1"), poisoned(), part("VIN2")],
        None,
    ))
    .expect("the fixture seeds");
    let read = events.snapshot();
    let models = RecordingProjectionStore::new();
    let mut views = [Shard::new("ledger"), Shard::new("board").skipping()];

    let ran = runner.run(&events, &models, &mut views, chunk_of(1));

    let mut failed = Vec::new();
    let want = Ran::Returned(vec![Err(Some(read[1].position)), Ok(())]);
    if ran != want {
        failed.push(format!("leg A: the run was {ran:?}; expected {want:?}"));
    }
    let ledger = block_on(checkpoint_of(&models, &views[0]));
    let board = block_on(checkpoint_of(&models, &views[1]));
    let record = models.get(&views[1].skip_row(read[1].id));
    let at = |n: usize| Checkpoint::Live {
        through: read[n].position,
    };
    if ledger != at(0) || board != at(2) || record.is_none() {
        failed.push(format!(
            "leg B: ledger at {ledger:?}, board at {board:?}, record {record:?}; expected \
             the ledger at the first event, the board at the last, and the record"
        ));
    }
    if failed.is_empty() {
        Ok(())
    } else {
        Err(failed.join("\n"))
    }
}

#[test]
fn failure_policy_is_per_projection() {
    assert_eq!(per_projection_policy_holds(&Shipped), Ok(()));
}

#[test]
fn failure_policy_is_per_projection_rejects_one_policy_for_every_view() {
    assert_eq!(
        legs(per_projection_policy_holds(&MiniFan(
            Defect::AsksTheFirstViewForEveryone
        ))),
        ['A', 'B']
    );
}

// ---------------------------------------------------------------------------
// PS-29 — `one_poisoned_projection_does_not_stall_the_others`
// ---------------------------------------------------------------------------

/// PS-29's rule: twenty views over one log, one refusing an event the other
/// nineteen take. The nineteen reach the end, and the one's failure is in the
/// value the run returned, so nothing has to be polled or drained for it.
fn one_poisoned_view_holds(runner: &impl FanRunner) -> Result<(), String> {
    const NAMES: [&str; 20] = [
        "v00", "v01", "v02", "v03", "v04", "v05", "v06", "v07", "v08", "v09", "v10", "v11", "v12",
        "v13", "v14", "v15", "v16", "v17", "v18", "v19",
    ];
    const POISONED: usize = 7;

    let events = MemoryEventStore::new();
    block_on(EventStore::append(
        &events,
        &[part("VIN1"), part("POISON"), part("VIN2")],
        None,
    ))
    .expect("the fixture seeds");
    let read = events.snapshot();
    let models = RecordingProjectionStore::new();
    let mut views: Vec<Shard> = NAMES
        .iter()
        .enumerate()
        .map(|(at, name)| {
            let view = Shard::new(name);
            if at == POISONED {
                view.refusing("POISON")
            } else {
                view
            }
        })
        .collect();

    let ran = runner.run(&events, &models, &mut views, chunk_of(1));

    let mut failed = Vec::new();
    let want: Vec<Outcome> = (0..NAMES.len())
        .map(|at| {
            if at == POISONED {
                Err(Some(read[1].position))
            } else {
                Ok(())
            }
        })
        .collect();
    if ran != Ran::Returned(want) {
        failed.push(format!(
            "leg A: the run was {ran:?}; expected nineteen Ok and the poisoned view's stop"
        ));
    }
    let end = Checkpoint::Live {
        through: read[2].position,
    };
    let stalled: Vec<&str> = views
        .iter()
        .enumerate()
        .filter(|(at, view)| *at != POISONED && block_on(checkpoint_of(&models, view)) != end)
        .map(|(_, view)| view.name)
        .collect();
    if !stalled.is_empty() {
        failed.push(format!(
            "leg B: {stalled:?} did not reach the end of the log"
        ));
    }
    if failed.is_empty() {
        Ok(())
    } else {
        Err(failed.join("\n"))
    }
}

#[test]
fn one_poisoned_projection_does_not_stall_the_others() {
    assert_eq!(one_poisoned_view_holds(&Shipped), Ok(()));
}

#[test]
fn one_poisoned_projection_does_not_stall_the_others_rejects_a_shared_halt() {
    assert_eq!(
        legs(one_poisoned_view_holds(&MiniFan(Defect::StopsEveryView))),
        ['A', 'B']
    );
}
