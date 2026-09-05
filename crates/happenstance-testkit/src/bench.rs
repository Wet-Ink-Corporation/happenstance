//! The benchmark family: a fourth macro family, and deliberately not a bar.
//!
//! # A benchmark is not a conformance rule, and the enforcement is structural
//!
//! CF-34 says performance MUST be measured by a separate harness and that the
//! harness MUST NOT be part of the conformance bar. Its own `Rule:` line reads
//! *none* — the harness is not the bar, which is the clause's whole content —
//! so there was nothing to add to `suite.rs`, and adding it would have violated
//! the clause it claims to serve.
//!
//! That claim is made a property of the tree rather than a sentence in a
//! report. This module carries **its own enumeration**,
//! [`for_each_event_store_benchmark!`](crate::for_each_event_store_benchmark),
//! beside the scenarios it names, exactly as `for_each_model_rule!` and
//! `for_each_concurrency_rule!` do and for the reason `concurrency.rs` states:
//! `cargo xtask spec-trace` scans `suite.rs` for `pub async fn`, so anything
//! written there owes a clause. Nothing here is registered in
//! `for_each_event_store_rule!`, `registry::no_orphan_rules` sees the same set
//! either side of this module's arrival, and `xtask`'s `RULE_FILES` does not
//! name this file.
//!
//! **A benchmark result can never turn a merge red.** There is no threshold
//! anywhere, on any budget: the shipped emitters assert that a scenario
//! completed and that its record accounts for every attempt it made, and
//! nothing else. A wall-clock assertion passes on the author's machine, fails
//! on a loaded runner, and teaches contributors to re-run until green — which
//! is CF-33's argument for having no watchdog either.
//!
//! # The testkit never reads a clock. The caller's emitter does.
//!
//! CF-33 forbids a clock anywhere under `crates/happenstance-testkit/src`, and
//! `cargo run -p xtask -- lint-clock` enforces it by scanning every file in
//! that directory — this one included. That is not an obstacle worked around;
//! it lands CF-23 and CF-33 on the same seam. **The harness defines and drives
//! the workload and reports counts; the caller's emitter observes it and owns
//! the timer**, along with warm-up, repetition and whatever it wants to do with
//! the number.
//!
//! So the measurement dependency belongs to the caller too. `criterion`,
//! `divan` and a CSV writer are all things an adapter author puts in *their*
//! `dev-dependencies`; this crate's stay `happenstance-core` and `futures-core`,
//! which is the discipline `.kb/decisions/0010-the-suite-must-prove-itself.md`
//! records. `crates/happenstance-testkit/tests/memory_benchmarks.rs` writes such
//! an emitter, in the caller's crate, where a clock is allowed.
//!
//! # What is measured, and why it is these three
//!
//! Fixed by two consumers rather than chosen freely. `RUNBOOK.md`'s phase-8
//! work item names append throughput, conditional append under contention, and
//! replay of *N* events with and without a tag filter. ADR-0012 names this same
//! instrument as the evidence that could lift its `&[Event]` marker, and
//! specifies *"a realistic batch and rejection mix"* — so
//! [`scenarios::conditional_append_under_contention`] reports the committed and
//! rejected counts **separately**, and distinguishes a condition violation from
//! a store failure the way `concurrency::Attempt` does. A run in which every
//! contender wins is a valid measurement of the wrong thing, and the record is
//! what makes that legible instead of averaging it away.
//!
//! *n*, *k* and *N* are [`BenchmarkParams`] supplied at the call site, because
//! no constant in this crate can be right for both a `Vec` behind an `RwLock`
//! and a file under `BEGIN IMMEDIATE`.
//!
//! # Contention without a `Send` bound
//!
//! The concurrency family buys thread-level contention with
//! `F::Store: EventStore + Send`. This family does not copy that: it binds
//! [`Fixture`](crate::Fixture) and nothing more, so the shape stays honest for the `!Send`
//! flavour the whole two-trait design exists for. *k* append futures are built
//! before any of them is polled and then driven round-robin on one thread — the
//! shape the position-visibility and re-entrancy rules already use. An adapter
//! that wants thread-level contention supplies it through its own emitter.
//!
//! # Usage
//!
//! ```rust,ignore
//! happenstance_testkit::event_store_benchmarks!(MyFixture::new());
//! ```
//!
//! Behind the off-by-default `bench` feature, so `cargo test --features bench`.
//! The module is additionally absent on `wasm32-unknown-unknown`: a Cargo
//! feature is not target-scoped, so `--all-features` would otherwise reach a
//! target with no threads and no host clock.

use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};

use happenstance_core::{AppendError, SequencePosition};

/// The largest number of events one seeding append writes.
///
/// Well under `MIN_SUPPORTED_EVENTS_PER_BATCH` (128, VT-24), so a conformant
/// store never refuses a seed batch merely for its size. A fixture that states
/// a smaller ceiling narrows it further rather than being ignored.
const SEED_CHUNK: usize = 64;

/// The workload sizes a benchmark run uses, chosen by the caller.
///
/// Three numbers, one per scenario, and they are parameters rather than
/// constants for a reason the first two adapters make concrete: the batch size,
/// contender count and replay length that say something useful about a `Vec`
/// behind an `RwLock` are not the ones that say something useful about a file
/// under `BEGIN IMMEDIATE`. A constant chosen here would be wrong for at least
/// one of them and unarguable for both.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BenchmarkParams {
    batch_size: usize,
    contenders: usize,
    replay_events: usize,
}

impl BenchmarkParams {
    /// The deliberately tiny budget the one-argument macro arm uses.
    ///
    /// Sized to live inside the gate's ordinary
    /// `cargo test --workspace --all-features` step rather than to measure
    /// anything: a real number comes from a caller who passed their own
    /// `params`. Keeping the mounted budget small is what stops an executed
    /// benchmark from becoming a benchmark that gates.
    pub const SMOKE: Self = Self {
        batch_size: 8,
        contenders: 4,
        replay_events: 16,
    };

    /// Builds a parameter set, refusing a degenerate one by name.
    ///
    /// # Panics
    ///
    /// Panics if any of `batch_size`, `contenders` or `replay_events` is zero,
    /// naming the offending parameter. The refusal is here, before any store is
    /// reached, because an empty batch is already refused by the contract — and
    /// *before* the condition is evaluated — so letting one through would turn
    /// a caller's typo into a benchmark of the error path reported as
    /// throughput. Zero contenders measures no contention, and zero events to
    /// replay measures no replay.
    #[must_use]
    pub const fn new(batch_size: usize, contenders: usize, replay_events: usize) -> Self {
        assert!(
            batch_size > 0,
            "`batch_size` must be at least one: an empty batch is refused by the \
             contract before the condition is evaluated, so a benchmark of one \
             measures the error path and calls it throughput"
        );
        assert!(
            contenders > 0,
            "`contenders` must be at least one: a contended append with no \
             contenders measures no contention"
        );
        assert!(
            replay_events > 0,
            "`replay_events` must be at least one: a replay of nothing measures \
             nothing"
        );

        Self {
            batch_size,
            contenders,
            replay_events,
        }
    }

    /// How many events one append writes — the *n* of the throughput scenario.
    #[must_use]
    pub const fn batch_size(self) -> usize {
        self.batch_size
    }

    /// How many appends race for one consistency boundary — the *k* of the
    /// contended scenario.
    #[must_use]
    pub const fn contenders(self) -> usize {
        self.contenders
    }

    /// How many events are seeded before replay — the *N* of the replay
    /// scenario.
    #[must_use]
    pub const fn replay_events(self) -> usize {
        self.replay_events
    }
}

/// What one attempt within a pass produced.
///
/// The four cases are kept apart for ADR-0012's reason: a run whose contenders
/// never collide, a run refused by a stated ceiling and a run in which the
/// store broke are three different measurements, and collapsing any two of them
/// would let a broken store report a perfect rejection mix.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Outcome {
    /// The work completed — an append landed, or a read returned.
    Committed,
    /// The store answered `ConditionViolated`, which is the DCB retry signal.
    Rejected,
    /// A limit the fixture states, or the store reported, refused the work.
    Refused,
    /// Anything else: a store error, or an empty batch.
    Failed,
}

impl Outcome {
    /// Classifies an append error, keeping the three refusal kinds apart.
    ///
    /// The wildcard is required rather than lazy: `AppendError` is
    /// `#[non_exhaustive]`, so a variant added upstream must land somewhere —
    /// and `Failed` is the honest home for one this crate has never heard of.
    /// Folding it into `Rejected` would let a future error shape inflate the
    /// rejection mix ADR-0012 asked for.
    const fn of_error<E>(error: &AppendError<E>) -> Self {
        match error {
            AppendError::ConditionViolated(_) => Self::Rejected,
            AppendError::ExceedsStoreLimit { .. } => Self::Refused,
            _ => Self::Failed,
        }
    }

    /// Classifies a whole append result.
    fn of_append<E>(result: &Result<SequencePosition, AppendError<E>>) -> Self {
        match result {
            Ok(_) => Self::Committed,
            Err(error) => Self::of_error(error),
        }
    }

    /// Whether this attempt did the work it was asked to do.
    const fn is_committed(self) -> bool {
        matches!(self, Self::Committed)
    }
}

/// One measured pass within a scenario.
///
/// A scenario reports one pass per thing it did that a decision could be made
/// about — the seeding append, the contended append, each of the two replays —
/// so a reader can tell "the workload ran and this is what it cost" from "the
/// workload never happened".
///
/// The counters are deliberately *counts* and not durations. Nothing under
/// `crates/happenstance-testkit/src` may read a clock (CF-33), so the elapsed
/// time of a pass is the caller's emitter's to produce and to report beside
/// this record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BenchmarkPass {
    label: &'static str,
    attempts: usize,
    committed: usize,
    rejected: usize,
    refused: usize,
    failed: usize,
    events: usize,
}

impl BenchmarkPass {
    /// An empty pass under `label`.
    const fn new(label: &'static str) -> Self {
        Self {
            label,
            attempts: 0,
            committed: 0,
            rejected: 0,
            refused: 0,
            failed: 0,
            events: 0,
        }
    }

    /// Records one attempt and the events it moved.
    fn count(&mut self, outcome: Outcome, events: usize) {
        self.attempts += 1;
        match outcome {
            Outcome::Committed => self.committed += 1,
            Outcome::Rejected => self.rejected += 1,
            Outcome::Refused => self.refused += 1,
            Outcome::Failed => self.failed += 1,
        }
        self.events += events;
    }

    /// What this pass measured — `"append"`, `"seed"`, `"contend"`,
    /// `"replay-all"` or `"replay-tagged"`.
    #[must_use]
    pub const fn label(&self) -> &'static str {
        self.label
    }

    /// How many operations this pass attempted.
    #[must_use]
    pub const fn attempts(&self) -> usize {
        self.attempts
    }

    /// How many of them did the work they were asked to do.
    #[must_use]
    pub const fn committed(&self) -> usize {
        self.committed
    }

    /// How many lost a race, as `ConditionViolated`.
    ///
    /// The number ADR-0012 asked this instrument for: a contended run whose
    /// rejection count is zero measured no contention, whatever its timing
    /// said.
    #[must_use]
    pub const fn rejected(&self) -> usize {
        self.rejected
    }

    /// How many met a stated store limit.
    ///
    /// Limits are declared facts about a fixture rather than trades, so a
    /// benchmark meeting one is a legitimate outcome that has to be legible in
    /// the output — never counted as completed work, and never a panic.
    #[must_use]
    pub const fn refused(&self) -> usize {
        self.refused
    }

    /// How many failed for a transport or store reason.
    ///
    /// Kept apart from [`rejected`](Self::rejected) for the reason
    /// `concurrency::Attempt` keeps them apart: collapsing the two would let a
    /// broken store report a perfect rejection mix.
    #[must_use]
    pub const fn failed(&self) -> usize {
        self.failed
    }

    /// How many events this pass wrote or read.
    #[must_use]
    pub const fn events(&self) -> usize {
        self.events
    }

    /// Whether every attempt is accounted for by exactly one counter.
    #[must_use]
    pub const fn is_well_formed(&self) -> bool {
        self.attempts == self.committed + self.rejected + self.refused + self.failed
    }
}

/// What one benchmark scenario produced.
///
/// `#[must_use]` for `RuleOutcome`'s reason: a harness that ran a workload and
/// dropped the answer measured nothing, and the workspace denies warnings, so
/// an emitter that forgets to report is a red build rather than a quiet one.
#[derive(Debug, Clone, PartialEq, Eq)]
#[must_use]
pub struct BenchmarkRecord {
    scenario: &'static str,
    owed: &'static [&'static str],
    passes: Vec<BenchmarkPass>,
}

impl BenchmarkRecord {
    /// An empty record for `scenario`, naming the passes that scenario owes.
    ///
    /// `owed` is not a free-standing declaration, and it is worth saying why
    /// rather than trusting it: a claim that can be satisfied without doing the
    /// work it names is not an obligation. This one is pinned from **both**
    /// sides. [`push`](Self::push) panics on a label that is not owed, so a
    /// scenario cannot under-declare — every label its ordinary path produces
    /// is forced into this list by the first run. [`report`](Self::report)
    /// panics on an owed label that never arrived, so it cannot over-declare
    /// either. What is left is exactly the set of passes the scenario produces
    /// when nothing went wrong, which is what *completed* has to mean here.
    const fn new(scenario: &'static str, owed: &'static [&'static str]) -> Self {
        Self {
            scenario,
            owed,
            passes: Vec::new(),
        }
    }

    /// Adds a completed pass.
    ///
    /// # Panics
    ///
    /// Panics if `pass` carries a label the scenario did not declare in
    /// [`new`](Self::new). That is a defect in the harness rather than a
    /// verdict on the adapter, and it is the half of the tie that stops `owed`
    /// from being narrowed to whatever a scenario happens to reach on its
    /// unhappy path.
    fn push(&mut self, pass: BenchmarkPass) {
        assert!(
            self.owed.contains(&pass.label),
            "{}: pushed a `{}` pass the scenario does not declare. Its owed \
             labels are {:?}, and a pass outside them is a record that no \
             longer says what completion means for this scenario.",
            self.scenario,
            pass.label,
            self.owed
        );
        self.passes.push(pass);
    }

    /// Which scenario produced this record.
    #[must_use]
    pub const fn scenario(&self) -> &'static str {
        self.scenario
    }

    /// Every pass this scenario reported, in the order it ran them.
    #[must_use]
    pub fn passes(&self) -> &[BenchmarkPass] {
        &self.passes
    }

    /// The pass under `label`, if the scenario reported one.
    #[must_use]
    pub fn pass(&self, label: &str) -> Option<&BenchmarkPass> {
        self.passes.iter().find(|pass| pass.label == label)
    }

    /// Whether every pass accounts for every attempt it made.
    ///
    /// One of the two things the shipped emitters assert, and the weaker one.
    /// It is a well-formedness check on the *record* — never a judgement on a
    /// number, which is what keeps a benchmark from failing a merge — and it
    /// says nothing at all about which passes ran, because every counter in a
    /// record built by an aborted scenario adds up just as well as one in a
    /// complete run. [`is_complete`](Self::is_complete) is the other half, and
    /// [`report`](Self::report) asserts both.
    #[must_use]
    pub fn is_well_formed(&self) -> bool {
        !self.passes.is_empty() && self.passes.iter().all(BenchmarkPass::is_well_formed)
    }

    /// Whether every pass this scenario owes actually ran.
    ///
    /// The distinction this preserves is the one the module documentation says
    /// the record exists for: *contention produced no rejections* and
    /// *contention never happened* are different measurements, and only the
    /// first is a measurement. A scenario that returned early leaves the pass
    /// that carried its whole point **absent** rather than zero, and an
    /// absence is exactly what a `BENCH` line compared across two releases
    /// cannot recover afterwards.
    ///
    /// Still not a judgement on a number: an incomplete run is not a slow one,
    /// and no threshold exists here at any budget (CF-34).
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.missing_passes().is_empty()
    }

    /// Which owed passes never arrived, in the order the scenario declared them.
    fn missing_passes(&self) -> Vec<&'static str> {
        self.owed
            .iter()
            .copied()
            .filter(|label| self.pass(label).is_none())
            .collect()
    }

    /// Every counter, on one line.
    ///
    /// One line and not more, because the gate's test step runs
    /// `-- --show-output` so that a declined capability's `SKIP <rule>: …` line
    /// reaches a human, and a chatty benchmark would bury exactly the output
    /// that step exists to surface.
    #[must_use]
    pub fn summary(&self) -> String {
        self.passes
            .iter()
            .map(|pass| {
                format!(
                    "{} attempts={} committed={} rejected={} refused={} failed={} events={}",
                    pass.label,
                    pass.attempts,
                    pass.committed,
                    pass.rejected,
                    pass.refused,
                    pass.failed,
                    pass.events
                )
            })
            .collect::<Vec<_>>()
            .join("; ")
    }

    /// Reports this record to stdout, under the name of the scenario that
    /// produced it.
    ///
    /// One line. It asserts that the scenario completed and that the record is
    /// well-formed — and nothing about how long anything took, at any budget.
    ///
    /// Both halves are checked here, and they are two assertions rather than
    /// one because they fail for unrelated reasons and a reader has to be able
    /// to tell them apart: a malformed record is a counter that was not
    /// incremented, an incomplete one is a pass that never ran.
    ///
    /// # Panics
    ///
    /// Panics if a pass does not account for every attempt it made, and
    /// separately if a pass the scenario owes is absent. Either is a defect in
    /// the harness or a store that fell over before the measurement began —
    /// never a verdict on how fast anything was.
    pub fn report(self, scenario: &str) {
        assert!(
            self.is_well_formed(),
            "{scenario}: every pass must report at least one attempt and account \
             for each of them as committed, rejected, refused or failed. Got {self:?}"
        );
        let missing = self.missing_passes();
        assert!(
            missing.is_empty(),
            "{scenario}: did not complete — the {missing:?} pass(es) it owes are \
             absent rather than zero, so the run measured something other than \
             what this scenario names. This is not a threshold and not a \
             timing: a pass that never ran is not a slow pass. Got {self:?}"
        );
        println!("BENCH {scenario}: {}", self.summary());
    }
}

/// One contender's append, boxed so that *k* of them can be held together.
type ContenderFuture<'a> = Pin<Box<dyn Future<Output = Outcome> + 'a>>;

/// Drives every future to completion, polling them round-robin on one thread.
///
/// This is how contention is reached without a `Send` bound. Every future is
/// built before any of them is polled — an `async fn` body runs nothing until
/// its first `poll` — so all *k* appends are genuinely in flight against one
/// backing store, with no executor, no thread and no bound the `!Send` flavour
/// cannot satisfy.
///
/// Against a store that serialises its writers the first future polled commits
/// and the rest learn that they lost, which is the rejection mix ADR-0012 asked
/// for. Against a store with real I/O under it a future returns `Pending`, the
/// loop moves on to the next, and the interleaving is genuine rather than
/// nominal.
struct Interleaved<'a> {
    pending: Vec<Option<ContenderFuture<'a>>>,
    done: Vec<Option<Outcome>>,
}

impl<'a> Interleaved<'a> {
    /// Takes ownership of the futures to drive.
    fn new(futures: Vec<ContenderFuture<'a>>) -> Self {
        let done = std::vec![None; futures.len()];
        Self {
            pending: futures.into_iter().map(Some).collect(),
            done,
        }
    }
}

impl Future for Interleaved<'_> {
    type Output = Vec<Outcome>;

    fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        // `get_mut` rather than an unsafe projection: every field is `Unpin`,
        // and this workspace forbids `unsafe` outright.
        let this = self.get_mut();
        let mut outstanding = false;

        for (index, slot) in this.pending.iter_mut().enumerate() {
            if let Some(future) = slot.as_mut() {
                match future.as_mut().poll(context) {
                    Poll::Ready(outcome) => {
                        this.done[index] = Some(outcome);
                        *slot = None;
                    }
                    Poll::Pending => outstanding = true,
                }
            }
        }

        if outstanding {
            return Poll::Pending;
        }

        // Every slot is `Some` on this path, so flattening preserves both the
        // count and the order; it is written this way rather than with an
        // unwrap because the workspace denies those and a panic here would be
        // a benchmark failing a build.
        Poll::Ready(this.done.iter().copied().flatten().collect())
    }
}

/// The three scenarios, in the order the runbook names them.
///
/// Each has the same shape as a conformance rule — `async fn(open, params)` —
/// and differs in what it returns: a [`BenchmarkRecord`] rather than a verdict,
/// because a benchmark has no verdict to give.
pub mod scenarios {
    use happenstance_core::{Event, EventStore, Query, ReadOptions, collect};

    use super::{
        BenchmarkParams, BenchmarkPass, BenchmarkRecord, ContenderFuture, Interleaved, Outcome,
        SEED_CHUNK,
    };
    use crate::Fixture;
    use crate::fixtures::{condition_after, query_of, query_tagged, tagged_event};

    /// (a) Append throughput: one batch of *n* events, unconditionally.
    ///
    /// Repetition and warm-up are the caller's emitter's, not the harness's —
    /// so one invocation is one batch, and a runner that wants a median over
    /// twenty runs the scenario twenty times around its own timer.
    ///
    /// A fixture that states a `MAX_EVENTS_PER_BATCH` below *n* has the append
    /// reported as a **refusal** rather than as work: the ceiling is a declared
    /// fact about that store, and a benchmark that met one and reported zero
    /// events as a success would be measuring the refusal path.
    pub async fn append_throughput<F: Fixture>(
        open: impl AsyncFn() -> F,
        params: BenchmarkParams,
    ) -> BenchmarkRecord {
        let fixture = open().await;
        let store = fixture.connect().await;

        let batch: Vec<Event> = (0..params.batch_size())
            .map(|_| tagged_event("BenchmarkAppended", &[("bench", "append")]))
            .collect();

        let mut pass = BenchmarkPass::new("append");
        if F::MAX_EVENTS_PER_BATCH.is_some_and(|ceiling| ceiling < params.batch_size()) {
            pass.count(Outcome::Refused, 0);
        } else {
            let outcome = Outcome::of_append(&store.append(&batch, None).await);
            let landed = if outcome.is_committed() {
                params.batch_size()
            } else {
                0
            };
            pass.count(outcome, landed);
        }

        // One pass, and the declaration is what makes "it ran" checkable: a
        // scenario that produced no `append` pass did not measure an append,
        // however well-formed the record it returned.
        let mut record = BenchmarkRecord::new("append_throughput", &["append"]);
        record.push(pass);
        record
    }

    /// (b) Conditional append under *k* contenders, reporting the split.
    ///
    /// One boundary event is seeded, then *k* handles each attempt a
    /// conditional append against the same boundary. Every one of the *k* is
    /// accounted for as committed, rejected, refused or failed, which is what
    /// makes a run where nobody collided legible as such rather than averaged
    /// away — ADR-0012 asked this instrument for a *rejection mix*, and a
    /// rejection count of zero is a measurement of the wrong thing however fast
    /// it was.
    ///
    /// The *k* futures are built before any is polled and then driven
    /// round-robin on one thread, so this imposes no `Send` bound. An adapter
    /// that wants OS-thread contention supplies it from its own emitter.
    pub async fn conditional_append_under_contention<F: Fixture>(
        open: impl AsyncFn() -> F,
        params: BenchmarkParams,
    ) -> BenchmarkRecord {
        let fixture = open().await;
        // Both passes are owed. The `contend` one is the entire measurement,
        // and the early return below is exactly the path that used to drop it
        // silently.
        let mut record =
            BenchmarkRecord::new("conditional_append_under_contention", &["seed", "contend"]);

        let opener = fixture.connect().await;
        let boundary_event = [tagged_event("BenchmarkBoundary", &[("bench", "contend")])];
        let planted = opener.append(&boundary_event, None).await;

        let mut seed_pass = BenchmarkPass::new("seed");
        let outcome = Outcome::of_append(&planted);
        seed_pass.count(outcome, usize::from(outcome.is_committed()));
        record.push(seed_pass);

        let Ok(boundary) = planted else {
            // A seed that never landed leaves no boundary to race for. The
            // record says so — one pass, no contended pass — rather than
            // reporting a contended run that did not happen.
            return record;
        };

        let condition = condition_after(
            query_of(&["BenchmarkContender"], &[("bench", "contend")]),
            boundary.get(),
        );
        let contender_event = [tagged_event("BenchmarkContender", &[("bench", "contend")])];

        let mut stores = Vec::with_capacity(params.contenders());
        for _ in 0..params.contenders() {
            stores.push(fixture.connect().await);
        }

        let futures: Vec<ContenderFuture<'_>> = stores
            .iter()
            .map(|store| {
                let condition = &condition;
                let events = &contender_event;
                let attempt =
                    async move { Outcome::of_append(&store.append(events, Some(condition)).await) };
                Box::pin(attempt) as ContenderFuture<'_>
            })
            .collect();

        let mut pass = BenchmarkPass::new("contend");
        for outcome in Interleaved::new(futures).await {
            pass.count(outcome, usize::from(outcome.is_committed()));
        }
        record.push(pass);

        record
    }

    /// (c) Replay of *N* events, once unfiltered and once behind a tag filter.
    ///
    /// Both passes are reported, because the pair is the measurement: an
    /// unfiltered replay says what the read path costs per event, and a
    /// filtered one says what the adapter's index — or its absence — costs on
    /// top. One event in three carries the filtered tag, so the filtered pass
    /// selects a proper subset rather than everything or nothing.
    ///
    /// Seeding is chunked, so a fixture stating a batch ceiling is honoured
    /// rather than driven into a refusal that would measure the error path.
    pub async fn replay_with_and_without_a_tag_filter<F: Fixture>(
        open: impl AsyncFn() -> F,
        params: BenchmarkParams,
    ) -> BenchmarkRecord {
        let fixture = open().await;
        let store = fixture.connect().await;
        // The pair is the measurement, so both replays are owed alongside the
        // seed: a filtered replay with nothing to compare it against says
        // nothing about what the adapter's index costs.
        let mut record = BenchmarkRecord::new(
            "replay_with_and_without_a_tag_filter",
            &["seed", "replay-all", "replay-tagged"],
        );

        let chunk_size = F::MAX_EVENTS_PER_BATCH
            .map_or(SEED_CHUNK, |ceiling| ceiling.min(SEED_CHUNK))
            .max(1);

        let mut seed_pass = BenchmarkPass::new("seed");
        let mut written = 0usize;
        while written < params.replay_events() {
            let take = chunk_size.min(params.replay_events() - written);
            let chunk: Vec<Event> = (0..take)
                .map(|offset| seed_event(written + offset))
                .collect();

            let outcome = Outcome::of_append(&store.append(&chunk, None).await);
            let landed = if outcome.is_committed() { take } else { 0 };
            seed_pass.count(outcome, landed);
            written += landed;

            if !outcome.is_committed() {
                break;
            }
        }
        record.push(seed_pass);

        record.push(replay(&store, "replay-all", &Query::all()).await);
        record.push(replay(&store, "replay-tagged", &query_tagged(&[("shard", "hot")])).await);

        record
    }

    /// One seeded event; every third carries the tag the filtered pass selects.
    fn seed_event(index: usize) -> Event {
        if index.is_multiple_of(3) {
            tagged_event(
                "BenchmarkReplayed",
                &[("bench", "replay"), ("shard", "hot")],
            )
        } else {
            tagged_event("BenchmarkReplayed", &[("bench", "replay")])
        }
    }

    /// Reads `query` to exhaustion and reports how many events it matched.
    ///
    /// A read that fails is a `failed` attempt rather than a panic: a store
    /// that cannot serve a replay is a measurement, and the record is where it
    /// belongs.
    async fn replay<S: EventStore>(store: &S, label: &'static str, query: &Query) -> BenchmarkPass {
        let mut pass = BenchmarkPass::new(label);
        match collect(store.read(query, ReadOptions::new())).await {
            Ok(events) => pass.count(Outcome::Committed, events.len()),
            Err(_) => pass.count(Outcome::Failed, 0),
        }
        pass
    }
}

// =====================================================================
// The enumeration, the emitters, and the macro
// =====================================================================

/// Hands the complete benchmark scenario set to `$callback`.
///
/// This family's single enumeration, beside the scenarios it names. It is
/// **not** `for_each_event_store_rule!` and it deliberately shares nothing with
/// it: a benchmark is not a conformance rule (CF-34), so the rule-name list
/// this crate publishes is byte-for-byte what it was before this module
/// existed.
///
/// # Examples
///
/// ```
/// macro_rules! scenario_names {
///     ($($name:ident),* $(,)?) => { [ $( stringify!($name) ),* ] };
/// }
///
/// let names = happenstance_testkit::for_each_event_store_benchmark!(scenario_names);
/// assert!(names.contains(&"append_throughput"));
/// assert_eq!(names.len(), 3);
/// ```
#[macro_export]
macro_rules! for_each_event_store_benchmark {
    // Raw token trees rather than `$cb:path`, for the reason
    // `for_each_event_store_rule!` records: a parsed `path` fragment cannot sit
    // in callee position inside an expression, which would forbid the
    // `let names = …` form the example above depends on.
    ($($callback:tt)+) => {
        $($callback)+! {
            append_throughput,
            conditional_append_under_contention,
            replay_with_and_without_a_tag_filter,
        }
    };
}

/// Emits one `#[tokio::test]` per benchmark scenario.
///
/// The default, and the honest smoke shape: it asserts that the scenario
/// completed and that its record accounts for every attempt, then prints one
/// line. It reads no clock, so it imposes no measurement dependency — an
/// emitter that wants a duration is the caller's to write, and
/// `tests/memory_benchmarks.rs` writes one.
///
/// The caller's crate needs `tokio` with `macros` and `rt`.
#[doc(hidden)]
#[macro_export]
macro_rules! __emit_benchmark_tokio {
    ($($name:ident),* $(,)?) => {
        $(
            #[tokio::test]
            async fn $name() {
                $crate::bench::scenarios::$name(__benchmark_fixture, __benchmark_params())
                    .await
                    .report(::core::stringify!($name));
            }
        )*
    };
}

/// Emits one plain `#[test]` per benchmark scenario, driven by
/// [`block_on`](crate::block_on).
///
/// No runtime, no dependency, one thread — and the contended scenario is just
/// as contended, because the interleaving is in the scenario rather than in the
/// runtime.
#[doc(hidden)]
#[macro_export]
macro_rules! __emit_benchmark_blocking {
    ($($name:ident),* $(,)?) => {
        $(
            #[test]
            fn $name() {
                $crate::block_on(
                    $crate::bench::scenarios::$name(__benchmark_fixture, __benchmark_params())
                )
                .report(::core::stringify!($name));
            }
        )*
    };
}

/// Generates the benchmark harness for an event store adapter.
///
/// **Not conformance.** Nothing this expands to is a conformance rule, no
/// adapter's bar moves because it exists, and no result it produces can fail a
/// merge (CF-34). It is inherited exactly as conformance is — one line against
/// the fixture you already wrote — and it imposes no measurement dependency on
/// you (CF-23): the per-scenario wrapper, and with it the clock, is the `emit`
/// parameter.
///
/// Behind the off-by-default `bench` feature, and absent on
/// `wasm32-unknown-unknown`. Gate your invocation as this crate's own
/// `tests/memory_benchmarks.rs` does.
///
/// # Examples
///
/// ```
/// # macro_rules! ignore { ($($t:tt)*) => {} }
/// # ignore! {
/// use happenstance_testkit::fixtures::MemoryFixture;
///
/// happenstance_testkit::event_store_benchmarks!(MemoryFixture::new());
/// # }
/// ```
///
/// Your own workload sizes, and your own emitter — which is where a timer, a
/// warm-up, a repetition count and a `criterion` dependency all belong:
///
/// ```
/// # macro_rules! ignore { ($($t:tt)*) => {} }
/// # ignore! {
/// happenstance_testkit::event_store_benchmarks!(
///     mod_name = sqlite_benchmarks,
///     emit = my_timed_emitter,
///     params = happenstance_testkit::bench::BenchmarkParams::new(256, 64, 100_000),
///     fixture = SqliteFixture::new()
/// );
/// # }
/// ```
#[macro_export]
macro_rules! event_store_benchmarks {
    // The general form. Listed first so that arm matching never has to back out
    // of a shorter arm to reach it.
    (
        mod_name = $mod_name:ident,
        emit = $emit:path,
        params = $params:expr,
        fixture = $fixture:expr
    ) => {
        mod $mod_name {
            #![allow(clippy::unwrap_used, unused_imports)]

            use super::*;

            // Named for `event_store_conformance!`'s `__conformance_fixture`
            // and hoisted for its reason: an emitter never has to name the
            // fixture's *type*, which it could not know, since an expression is
            // all this macro was handed. `impl Fixture` and not a concrete
            // store, and no `Send` bound anywhere — the benchmark family stays
            // usable by the `!Send` flavour the two-trait design exists for.
            async fn __benchmark_fixture() -> impl $crate::__private::Fixture {
                $fixture
            }

            // The workload sizes, hoisted the same way and for the same reason.
            // They are the caller's, because no constant here can be right for
            // both an in-process `Vec` and a file under a write lock.
            fn __benchmark_params() -> $crate::bench::BenchmarkParams {
                $params
            }

            // `$emit` is `$crate::`-qualified by the caller when it is one of
            // this crate's; a bare name is substituted verbatim and resolves in
            // the *caller's* crate, which is what makes a third-party emitter
            // writable at all — and what makes a misspelt one a compile error
            // naming the missing item there rather than a silent fallback.
            $crate::for_each_event_store_benchmark!($emit);
        }
    };
    (mod_name = $mod_name:ident, emit = $emit:path, fixture = $fixture:expr) => {
        $crate::event_store_benchmarks!(
            mod_name = $mod_name,
            emit = $emit,
            params = $crate::bench::BenchmarkParams::SMOKE,
            fixture = $fixture
        );
    };
    (mod_name = $mod_name:ident, params = $params:expr, fixture = $fixture:expr) => {
        $crate::event_store_benchmarks!(
            mod_name = $mod_name,
            emit = $crate::__emit_benchmark_tokio,
            params = $params,
            fixture = $fixture
        );
    };
    (mod_name = $mod_name:ident, fixture = $fixture:expr) => {
        $crate::event_store_benchmarks!(
            mod_name = $mod_name,
            emit = $crate::__emit_benchmark_tokio,
            params = $crate::bench::BenchmarkParams::SMOKE,
            fixture = $fixture
        );
    };
    ($fixture:expr) => {
        $crate::event_store_benchmarks!(
            mod_name = dcb_benchmarks,
            emit = $crate::__emit_benchmark_tokio,
            params = $crate::bench::BenchmarkParams::SMOKE,
            fixture = $fixture
        );
    };
}

#[cfg(test)]
mod tests {
    use super::{BenchmarkParams, BenchmarkPass, BenchmarkRecord, Outcome};

    /// A pass whose counters do not add up is not well-formed, which is the one
    /// thing the shipped emitters assert.
    #[test]
    fn a_pass_accounts_for_every_attempt() {
        let mut pass = BenchmarkPass::new("contend");
        pass.count(Outcome::Committed, 1);
        pass.count(Outcome::Rejected, 0);
        pass.count(Outcome::Failed, 0);

        assert!(pass.is_well_formed());
        assert_eq!(pass.attempts(), 3);
        assert_eq!(pass.committed(), 1);
        assert_eq!(pass.rejected(), 1);
        assert_eq!(pass.failed(), 1);
        assert_eq!(pass.refused(), 0);
        assert_eq!(pass.events(), 1);
    }

    /// An empty record is not well-formed: a scenario that reported nothing did
    /// not run, and reporting it as fine is the failure mode a smoke harness is
    /// most likely to have.
    #[test]
    fn an_empty_record_is_not_well_formed() {
        let record = BenchmarkRecord::new("append_throughput", &["append"]);
        assert!(!record.is_well_formed());
    }

    /// The summary is one line whatever the pass count.
    #[test]
    fn a_summary_is_one_line() {
        // The pair is the measurement, so both replays are owed alongside the
        // seed: a filtered replay with nothing to compare it against says
        // nothing about what the adapter's index costs.
        let mut record = BenchmarkRecord::new(
            "replay_with_and_without_a_tag_filter",
            &["seed", "replay-all", "replay-tagged"],
        );
        let mut first = BenchmarkPass::new("replay-all");
        first.count(Outcome::Committed, 12);
        let mut second = BenchmarkPass::new("replay-tagged");
        second.count(Outcome::Committed, 4);
        record.push(first);
        record.push(second);

        let summary = record.summary();
        assert!(!summary.contains('\n'));
        assert!(summary.contains("replay-all"));
        assert!(summary.contains("replay-tagged"));
        assert_eq!(
            record.pass("replay-tagged").map(BenchmarkPass::events),
            Some(4)
        );
        assert_eq!(record.pass("nothing"), None);
    }

    /// A refusal is not a rejection and neither is a failure: the three stay
    /// apart, because collapsing them would let a broken store report a perfect
    /// rejection mix.
    #[test]
    fn the_three_refusal_kinds_stay_apart() {
        let violated: Result<
            happenstance_core::SequencePosition,
            happenstance_core::AppendError<core::convert::Infallible>,
        > = Err(happenstance_core::AppendError::ExceedsStoreLimit {
            limit: happenstance_core::StoreLimit::EventsPerBatch,
            len: 9,
        });
        assert_eq!(Outcome::of_append(&violated), Outcome::Refused);

        let empty: Result<
            happenstance_core::SequencePosition,
            happenstance_core::AppendError<core::convert::Infallible>,
        > = Err(happenstance_core::AppendError::NoEvents);
        assert_eq!(Outcome::of_append(&empty), Outcome::Failed);
    }

    /// A record missing a pass its scenario owes is not complete, however well
    /// its counters add up.
    ///
    /// This is the pair the shipped emitters check, and the reason they are two
    /// assertions: the record below is well-formed and did not happen.
    #[test]
    fn an_incomplete_record_is_well_formed_all_the_same() {
        let mut record =
            BenchmarkRecord::new("conditional_append_under_contention", &["seed", "contend"]);
        let mut seed = BenchmarkPass::new("seed");
        seed.count(Outcome::Rejected, 0);
        record.push(seed);

        assert!(record.is_well_formed());
        assert!(!record.is_complete());
        assert_eq!(record.missing_passes(), vec!["contend"]);
    }

    /// A scenario cannot narrow its own obligation to whatever it happens to
    /// reach: pushing an undeclared label aborts.
    ///
    /// This is the half of the tie that makes the declaration mean something.
    /// Without it a scenario could declare only the passes it always produces
    /// — `["seed"]` here — and `is_complete` would be as vacuous as the check
    /// it replaced.
    #[test]
    #[should_panic(expected = "does not declare")]
    fn a_pass_the_scenario_did_not_declare_is_refused() {
        let mut record = BenchmarkRecord::new("conditional_append_under_contention", &["seed"]);
        let mut contended = BenchmarkPass::new("contend");
        contended.count(Outcome::Committed, 1);
        record.push(contended);
    }

    /// EC-003: every degenerate parameter is refused by name.
    #[test]
    #[should_panic(expected = "batch_size")]
    fn a_zero_batch_size_is_refused() {
        let _ = BenchmarkParams::new(0, 1, 1);
    }
}
