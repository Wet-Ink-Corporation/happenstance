//! Sixty-four connections on one file, racing for one consistency boundary, on
//! real OS threads.
//!
//! # Why this is the caller's and not the harness's
//!
//! `event_store_benchmarks!`'s contended scenario interleaves *k* append futures
//! on one thread, because the benchmark family binds `Fixture` and imposes no
//! `Send` bound — the `!Send` flavour the whole two-trait design exists for has
//! to be able to run it. `bench.rs`'s own module docs delegate the rest in
//! terms: *"an adapter that wants thread-level contention supplies it through
//! its own emitter."* This is that.
//!
//! It matters here because interleaving on one thread produces **no lock
//! contention at all**: the first future polled runs its whole synchronous body,
//! commits and returns before the second is entered, so `SQLITE_BUSY` never
//! fires and `BEGIN IMMEDIATE` is never contended. That is a correct
//! measurement of a rejection *mix* and a useless one for a busy timeout. The
//! harness figure and this one are both in `results/`, and the record says which
//! answers which question.
//!
//! # What AC-005 needs from it
//!
//! `concurrency::CONTENDERS` is 8 and both of phase 8's stated proof artefacts
//! read 64. `concurrency-family-and-contender-count` will move that constant and
//! should inherit a measured claim rather than an assumed one: whether 64
//! `rusqlite::Connection`s on one file open at all, what the race costs, and
//! what the committed/rejected/busy/failed split is. Raising the constant is
//! **not** this story's, and `crates/happenstance-testkit/**` is untouched.
//!
//! The three strategies are measured round-robin in one process, for
//! `tag_storage_probe.rs`'s reason: on this host two runs an hour apart disagree
//! by up to 45%, so a figure produced by running one arm and then another is a
//! figure about when each arm ran.

mod support;

use std::sync::Barrier;

use append_condition_probes::{
    AppendStrategy, BeginImmediateProbe, CandidateStore, ConditionalInsert, JoinTable,
    MonotonicGuard, SqliteProbeError,
};
use happenstance_core::{AppendError, Event, EventStore};
use happenstance_testkit::block_on;
use happenstance_testkit::fixtures::condition as unbounded_condition;
use happenstance_testkit::fixtures::{condition_after, query_of, tagged_event};
use support::CandidateFixture;

/// The two contender counts, measured in the same interleaved run.
///
/// 8 is what `concurrency::CONTENDERS` is today; 64 is what both of phase 8's
/// stated proof artefacts read. Measuring the pair is what turns "raising it is
/// supportable" from an assertion into a difference — a figure at 64 alone says
/// what 64 costs and not what the *raise* costs.
const COUNTS: [usize; 2] = [8, 64];

/// Events already in the log when the races start, so every probe has
/// something to walk.
const SEED: usize = 5_000;

/// The second log size the uncontended control is run at.
///
/// Ten times the first. A verdict read off one log length is a verdict about
/// that length: the arms were *predicted* to cross over as the matching set
/// grows, and the only way to say whether they do is to look at two.
const TOPPED_UP: usize = 50_000;

/// Events per seeding append.
const CHUNK: usize = 100;

/// Races per arm.
const ROUNDS: usize = 10;

/// Uncontended rounds per arm, for the sequential control below.
///
/// Far more than [`ROUNDS`] because each one is three orders of magnitude cheaper,
/// and a median over fifty samples on a host whose noise floor is ~15% is worth
/// having where a median over ten is not.
const SEQUENTIAL_ROUNDS: usize = 400;

/// The four sample vectors one arm accumulates across the rounds: accepted
/// conditional, unconditional, one-tag rejection, two-tag rejection.
///
/// A named alias rather than the bare tuple, because a four-element tuple of
/// identical vectors is exactly the shape nobody can read at the call site.
type Rounds = (Vec<u128>, Vec<u128>, Vec<u128>, Vec<u128>);

/// What one contender's attempt collapsed to.
///
/// `Rejected` and `Busy` are kept apart deliberately, and the distinction is the
/// whole reason this file can say anything about a busy timeout: a
/// `ConditionViolated` is the DCB retry signal and a *correct* outcome for a
/// loser, while `SQLITE_BUSY` is the adapter running out of patience and would
/// reach a conformance rule as `Attempt::Failed` — a red rule that is not about
/// the adapter's logic.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Outcome {
    Committed,
    Rejected,
    Busy,
    Failed,
}

/// What one race produced.
#[derive(Debug, Default)]
struct Tally {
    committed: usize,
    rejected: usize,
    busy: usize,
    failed: usize,
}

impl Tally {
    fn count(&mut self, outcome: Outcome) {
        match outcome {
            Outcome::Committed => self.committed += 1,
            Outcome::Rejected => self.rejected += 1,
            Outcome::Busy => self.busy += 1,
            Outcome::Failed => self.failed += 1,
        }
    }

    fn total(&self) -> usize {
        self.committed + self.rejected + self.busy + self.failed
    }
}

/// Classifies one contender's result without letting a non-`Send` error escape
/// its thread.
fn classify(
    result: Result<happenstance_core::SequencePosition, AppendError<SqliteProbeError>>,
) -> Outcome {
    match result {
        Ok(_) => Outcome::Committed,
        Err(AppendError::ConditionViolated(_)) => Outcome::Rejected,
        Err(AppendError::Store(SqliteProbeError::Sqlite(rusqlite::Error::SqliteFailure(
            error,
            _,
        )))) if error.code == rusqlite::ErrorCode::DatabaseBusy
            || error.code == rusqlite::ErrorCode::DatabaseLocked =>
        {
            Outcome::Busy
        }
        Err(_) => Outcome::Failed,
    }
}

/// Seeds one arm's store.
fn seeded_store<A: AppendStrategy>(
    fixture: &CandidateFixture<A, JoinTable>,
) -> CandidateStore<A, JoinTable> {
    let store = fixture.open();
    fill(&store, 0, SEED);
    store
}

/// Appends events until the log holds `to` of them.
fn fill<A: AppendStrategy>(store: &CandidateStore<A, JoinTable>, from: usize, to: usize) {
    let mut written = from;
    while written < to {
        let take = CHUNK.min(to - written);
        let batch: Vec<Event> = (0..take)
            .map(|offset| {
                let row = format!("r{}", (written + offset) % 97);
                tagged_event("Seeded", &[("shard", "cold"), ("row", &row)])
            })
            .collect();
        block_on(store.append(&batch, None)).expect("seeding must succeed");
        written += take;
    }
}

/// The **uncontended** cost of each strategy's guard, which is the case an
/// adapter runs on almost every write.
///
/// The contended figures below say what happens when sixty-four writers collide;
/// this says what the strategy costs when nobody does, and the two are different
/// questions. `conditional - unconditional` isolates the guard from the
/// transaction commit it sits inside — without the subtraction the figure is a
/// commit with a probe somewhere in it, and a reader cannot tell which of the
/// two moved.
fn sequential<A: AppendStrategy>(
    store: &CandidateStore<A, JoinTable>,
    conditional: &mut Vec<u128>,
    unconditional: &mut Vec<u128>,
    rejected: &mut Vec<u128>,
    rejected_multi: &mut Vec<u128>,
    round: usize,
) {
    let head = block_on(store.head())
        .expect("head must succeed")
        .expect("the store is not empty");
    let boundary = query_of(&["Seeded"], &[("row", "r7")]);
    let condition = condition_after(boundary, head.get());
    let row = format!("r{}", round % 97);
    let event = tagged_event("Seeded", &[("shard", "cold"), ("row", &row)]);

    let started = std::time::Instant::now();
    block_on(store.append(core::slice::from_ref(&event), Some(&condition)))
        .expect("the guard is anchored at head, so nothing can be after it");
    conditional.push(started.elapsed().as_micros());

    let started = std::time::Instant::now();
    block_on(store.append(core::slice::from_ref(&event), None))
        .expect("an unconditional append must succeed");
    unconditional.push(started.elapsed().as_micros());

    // The **rejection** path, and it is the one that separates the arms. An
    // unbounded guard over a query the log already satisfies is violated every
    // time, so the transaction rolls back and no commit happens — which removes
    // the one cost that dominates and swamps the two guards being compared. It
    // is also the path where the arms structurally differ: `EXISTS` and the
    // conditional `INSERT` both answer a boolean and need a *second* query to
    // name the conflicting position, while `max(position)` answers both halves
    // at once.
    //
    // It is a real path, not a synthetic one: this is exactly what a DCB
    // command loop runs when it loses a race and has to re-decide.
    let doomed = unbounded_condition(query_of(&["Seeded"], &[("row", "r7")]));
    let started = std::time::Instant::now();
    let outcome = block_on(store.append(core::slice::from_ref(&event), Some(&doomed)));
    rejected.push(started.elapsed().as_micros());
    assert!(
        matches!(outcome, Err(AppendError::ConditionViolated(_))),
        "the rejection control must actually be rejected, or it is timing the \
         wrong path: {outcome:?}"
    );

    // The same rejection path against a **two-tag** boundary, which is where the
    // arms were predicted to cross over. A single-tag item takes the join
    // table's fast path, and `max(position)` over a `(tag, position)` range is
    // then a seek to the end of it — an O(1) answer that flatters the monotonic
    // guard. Two tags force the general `GROUP BY … HAVING COUNT(DISTINCT tag)`
    // form, whose result is not an index range, so `max()` has to materialise
    // every matching position while `EXISTS` may still stop at the first.
    //
    // A boundary constrained by two tags is an ordinary DCB shape, not a corner:
    // "this course, this student" is two.
    let doomed_multi =
        unbounded_condition(query_of(&["Seeded"], &[("shard", "cold"), ("row", "r7")]));
    let started = std::time::Instant::now();
    let outcome = block_on(store.append(core::slice::from_ref(&event), Some(&doomed_multi)));
    rejected_multi.push(started.elapsed().as_micros());
    assert!(
        matches!(outcome, Err(AppendError::ConditionViolated(_))),
        "the multi-tag rejection control must actually be rejected: {outcome:?}"
    );
}

/// Runs one race and returns its wall time in microseconds and its tally.
fn race<A: AppendStrategy>(
    fixture: &CandidateFixture<A, JoinTable>,
    contenders: usize,
    round: usize,
) -> (u128, Tally) {
    // One connection per contender, opened before the race so that the timed
    // region is the contention rather than sixty-four file opens.
    let handles: Vec<CandidateStore<A, JoinTable>> =
        (0..contenders).map(|_| fixture.open()).collect();

    // Everybody decides from one snapshot: the guard is anchored at the head as
    // it stands now, so exactly one contender may commit and the rest must learn
    // they lost.
    let head = block_on(handles[0].head())
        .expect("head must succeed")
        .expect("the store is not empty");
    let boundary = query_of(&["Contended"], &[("race", "one")]);
    let condition = condition_after(boundary, head.get());
    let event = tagged_event("Contended", &[("race", "one")]);

    let gate = Barrier::new(contenders);
    let started = std::time::Instant::now();

    // A barrier rather than a sleep, for the reason `concurrency::race` records:
    // it synchronises on the other threads rather than on a wall clock, so it is
    // exact on a sixteen-core host and on a loaded single-core runner alike, and
    // spawning N threads in a loop does not start them together.
    let outcomes: Vec<Outcome> = std::thread::scope(|scope| {
        let running: Vec<_> = handles
            .iter()
            .map(|store| {
                let gate = &gate;
                let condition = &condition;
                let event = &event;
                scope.spawn(move || {
                    gate.wait();
                    classify(block_on(
                        store.append(core::slice::from_ref(event), Some(condition)),
                    ))
                })
            })
            .collect();

        running
            .into_iter()
            .map(|thread| match thread.join() {
                Ok(outcome) => outcome,
                Err(payload) => std::panic::resume_unwind(payload),
            })
            .collect()
    });

    let elapsed = started.elapsed().as_micros();
    let mut tally = Tally::default();
    for outcome in outcomes {
        tally.count(outcome);
    }

    assert_eq!(
        tally.total(),
        contenders,
        "round {round}: every contender must be accounted for"
    );
    (elapsed, tally)
}

/// Measures one arm across every round.
fn measure<A: AppendStrategy>(
    fixture: &CandidateFixture<A, JoinTable>,
    contenders: usize,
    samples: &mut Vec<u128>,
    totals: &mut Tally,
) {
    for round in 0..ROUNDS {
        let (elapsed, tally) = race(fixture, contenders, round);
        samples.push(elapsed);
        totals.committed += tally.committed;
        totals.rejected += tally.rejected;
        totals.busy += tally.busy;
        totals.failed += tally.failed;
    }
}

fn report(arm: &str, contenders: usize, samples: &mut [u128], totals: &Tally, conditions: &str) {
    samples.sort_unstable();
    println!(
        "CONTEND\t{arm}\tcontenders={contenders}\trace_us_median={}\t\
         race_us_min={}\trace_us_max={}\tcommitted={}\trejected={}\tbusy={}\t\
         failed={}\trounds={ROUNDS}\tseeded={SEED}\t{conditions}",
        samples[samples.len() / 2],
        samples[0],
        samples[samples.len() - 1],
        totals.committed,
        totals.rejected,
        totals.busy,
        totals.failed,
    );
}

#[test]
fn the_three_strategies_at_sixty_four_connections() {
    let probe_fixture = CandidateFixture::<BeginImmediateProbe, JoinTable>::new();
    let insert_fixture = CandidateFixture::<ConditionalInsert, JoinTable>::new();
    let guard_fixture = CandidateFixture::<MonotonicGuard, JoinTable>::new();

    // Seeding also proves the sixty-four-connection open works before anything
    // is timed: `race` opens them per round, and a file-descriptor or
    // connection ceiling would surface as a panic in `fixture.open()` naming the
    // platform — which is the EC-007 finding, not a number to work around by
    // quietly measuring at thirty-two.
    let probe = seeded_store(&probe_fixture);
    drop(seeded_store(&insert_fixture));
    drop(seeded_store(&guard_fixture));

    let conditions = probe.durability().conditions();

    let insert = insert_fixture.open();
    let guard = guard_fixture.open();

    // The uncontended control first, while the three logs are the same length.
    uncontended_control(&probe, &insert, &guard, SEED, &conditions);

    for contenders in COUNTS {
        let mut probe_samples = Vec::new();
        let mut insert_samples = Vec::new();
        let mut guard_samples = Vec::new();
        let mut probe_totals = Tally::default();
        let mut insert_totals = Tally::default();
        let mut guard_totals = Tally::default();

        measure(
            &probe_fixture,
            contenders,
            &mut probe_samples,
            &mut probe_totals,
        );
        measure(
            &insert_fixture,
            contenders,
            &mut insert_samples,
            &mut insert_totals,
        );
        measure(
            &guard_fixture,
            contenders,
            &mut guard_samples,
            &mut guard_totals,
        );

        report(
            "begin-immediate-probe",
            contenders,
            &mut probe_samples,
            &probe_totals,
            &conditions,
        );
        report(
            "conditional-insert",
            contenders,
            &mut insert_samples,
            &insert_totals,
            &conditions,
        );
        report(
            "monotonic-guard",
            contenders,
            &mut guard_samples,
            &guard_totals,
            &conditions,
        );

        for (arm, totals) in [
            ("begin-immediate-probe", &probe_totals),
            ("conditional-insert", &insert_totals),
            ("monotonic-guard", &guard_totals),
        ] {
            assert_eq!(
                totals.committed, ROUNDS,
                "{arm} at {contenders}: exactly one contender per round may \
                 commit — {} in {ROUNDS} rounds means the strategy is not atomic",
                totals.committed
            );
            assert_eq!(
                totals.total(),
                ROUNDS * contenders,
                "{arm} at {contenders}: every contender must be accounted for"
            );
        }
    }

    // And again at ten times the log. The arms were predicted to cross over as
    // the matching set grows — `max(position)` cannot stop where `EXISTS` can —
    // so a verdict read off one length would be a verdict about that length.
    for store in [
        &probe as &dyn LogFill,
        &insert as &dyn LogFill,
        &guard as &dyn LogFill,
    ] {
        store.top_up();
    }
    uncontended_control(&probe, &insert, &guard, TOPPED_UP, &conditions);
}

/// Tops one arm's log up to [`TOPPED_UP`], whatever its strategy.
///
/// A one-method object-safe trait rather than three calls, because the three
/// stores have three different types and the only thing wanted from them here
/// is identical.
trait LogFill {
    fn top_up(&self);
}

impl<A: AppendStrategy> LogFill for CandidateStore<A, JoinTable> {
    fn top_up(&self) {
        let held = block_on(self.head())
            .expect("head must succeed")
            .expect("the store is not empty");
        let held = usize::try_from(held.get()).unwrap_or(usize::MAX);
        fill(self, held, TOPPED_UP);
    }
}

/// The uncontended control: what each strategy's guard costs when nobody is
/// contending, at a stated log size.
fn uncontended_control(
    probe: &CandidateStore<BeginImmediateProbe, JoinTable>,
    insert: &CandidateStore<ConditionalInsert, JoinTable>,
    guard: &CandidateStore<MonotonicGuard, JoinTable>,
    seeded: usize,
    conditions: &str,
) {
    let mut samples: [Rounds; 3] = Default::default();

    for round in 0..SEQUENTIAL_ROUNDS {
        sequential(
            probe,
            &mut samples[0].0,
            &mut samples[0].1,
            &mut samples[0].2,
            &mut samples[0].3,
            round,
        );
        sequential(
            insert,
            &mut samples[1].0,
            &mut samples[1].1,
            &mut samples[1].2,
            &mut samples[1].3,
            round,
        );
        sequential(
            guard,
            &mut samples[2].0,
            &mut samples[2].1,
            &mut samples[2].2,
            &mut samples[2].3,
            round,
        );
    }

    for (arm, (conditional, unconditional, rejected, rejected_multi)) in [
        "begin-immediate-probe",
        "conditional-insert",
        "monotonic-guard",
    ]
    .into_iter()
    .zip(samples.iter_mut())
    {
        conditional.sort_unstable();
        unconditional.sort_unstable();
        rejected.sort_unstable();
        rejected_multi.sort_unstable();
        let with = conditional[conditional.len() / 2];
        let without = unconditional[unconditional.len() / 2];
        println!(
            "SEQUENTIAL\t{arm}\tconditional_append_us={with}\t\
             unconditional_append_us={without}\tguard_cost_us={}\t\
             rejected_1tag_us={}\trejected_1tag_p10_us={}\t\
             rejected_2tag_us={}\trejected_2tag_p10_us={}\t\
             rounds={SEQUENTIAL_ROUNDS}\tseeded={seeded}\t{conditions}",
            with.saturating_sub(without),
            rejected[rejected.len() / 2],
            rejected[rejected.len() / 10],
            rejected_multi[rejected_multi.len() / 2],
            rejected_multi[rejected_multi.len() / 10],
        );
    }
}
