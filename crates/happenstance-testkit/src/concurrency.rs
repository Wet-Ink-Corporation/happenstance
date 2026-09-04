//! The parallel half of the suite: contenders on real threads, racing for the
//! same consistency boundary.
//!
//! # It is additive. It retires nothing.
//!
//! The plan this module was written from said it would retire
//! [`racing_conditional_appends_elect_one_winner`](crate::rules::racing_conditional_appends_elect_one_winner).
//! **It does not**, and the reversal is on the record rather than in a commit
//! message: `SPECIFICATION.md` §7.4 now claims that rule for ES-25, because its
//! content is the *iff* seen as two decisions taken from one snapshot, and two
//! of the three registered mutants it rejects are shapes ES-25's own `Rejects:`
//! paragraph names by hand. Three dispositions were written this phase and all
//! three were reversed — every one of them by compiling a store rather than by
//! reading a rule, which is the method §6.1 forbids for adapters and which turns
//! out to be no more reliable when it is a rule being judged.
//!
//! So the two live side by side and they answer different questions. The
//! sequential rule fixes the *semantics* a race must have: which of two
//! decisions taken from one snapshot is allowed to land. It cannot distinguish
//! an atomic check-and-write from a probe followed by an insert, because with
//! one caller at a time there is no second caller to fit between the two halves.
//! This family supplies the second caller.
//!
//! # There is no timeout, and CF-33 is why
//!
//! The runbook's work item asked for one more assertion than is written here:
//! *every task terminates within a timeout*. It is deliberately absent.
//!
//! **CF-33 is `[FROZEN]`** — no conformance rule may read a clock, measure
//! elapsed time, or assert on an operation count — and its own `Rejects:`
//! paragraph makes exactly the argument that applies to a watchdog on a racing
//! rule: a wall-clock deadline passes on the author's machine, fails on a loaded
//! CI runner, fails under a debug build, and makes the suite's verdict a
//! property of the hardware. A flaky conformance suite is worse than no
//! conformance suite, because it teaches adapter authors to re-run until green.
//! A watchdog is the most tempting clock in the whole suite precisely because it
//! looks like a safety feature rather than an assertion.
//!
//! Liveness rests on the CI job timeout instead. That is a worse *message* — a
//! hung job names no rule — and it is the trade CF-33 already made twice, at
//! `harness.rs`'s "there is deliberately no watchdog" and at ES-10's note that a
//! store which cannot answer hangs rather than failing. The runbook item has
//! been amended to record it, so the next reader does not re-add it.
//!
//! # Why the contenders are OS threads and not tasks
//!
//! A rule body cannot call `tokio::spawn`. The testkit has no runtime
//! dependency and must not acquire one: CF-20 and CF-23 exist so that the suite
//! runs on `wasm32` and under a caller-supplied harness, and a `tokio` in
//! `[dependencies]` here would end both. The runtime is the *harness's* choice,
//! and the harness is the emitter.
//!
//! [`std::thread::scope`] needs nothing from anybody. Each contender gets its
//! own thread and drives its own future with [`block_on`](crate::block_on), the
//! same park-loop the runtime-free emitter uses. That is strictly more hostile
//! than a work-stealing runtime, because preemption is the operating system's
//! and does not wait for a suspension point the store may not have — which
//! matters most for the in-memory stores whose `append` bodies contain no
//! `.await` at all.
//!
//! **The limitation this buys, named now rather than discovered at phase 10.**
//! A contender drives its future to completion on a bare thread, outside any
//! ambient reactor. A store whose futures need one — `sqlx` is the case in this
//! workspace — cannot be driven that way, and phase 10's adapter will need the
//! *spawn* to become a parameter the harness supplies, exactly as the emitter
//! already is. Nothing here is designed against that; it is simply not designed
//! for it either, because no such adapter exists yet and a seam whose only
//! implementor is imaginary is a guess.
//!
//! # The bound, and what the compiler said about each part of it
//!
//! ```text
//! F: Fixture, F::Store: EventStore + Send
//! ```
//!
//! `SPECIFICATION.md` CF-22 predicts `S: SendEventStore + Send + Sync + 'static`
//! and an `Arc<S>`. **Three of those four turned out not to be needed**, and the
//! method for finding out is `memory.rs`'s `spawns_from_generic`: remove each
//! bound in turn and ask the compiler, rather than writing down the bound that
//! sounds right.
//!
//! * **`Send` — REQUIRED, and it is the whole of what makes this family opt-in.**
//!   A contender's *handle* is moved onto its thread. Remove it and
//!   `Scope::spawn` reports its bound unsatisfied. A `!Send` adapter — the
//!   Cloudflare Durable Object is the workspace's — is excluded here and must be,
//!   which is the property ADR-0001 exists to keep optional.
//! * **`SendEventStore` — NOT REQUIRED, which is the finding.** CF-22 predicts
//!   it, the runbook's work item specifies it, and the first draft of this module
//!   was written with it. Replacing it with `EventStore + Send` compiles, every
//!   rule unchanged. The reason is worth stating because it is the same
//!   observation ADR-0001 is built on, seen from the other side: **a future only
//!   needs to be `Send` if it crosses a thread boundary, and here it does not.**
//!   The handle crosses; the future is created *on* the contender's thread by
//!   `block_on` and driven to completion there. `tokio::spawn` would need the
//!   future, because it hands a future to a runtime that may move it — which is
//!   exactly where CF-22's prediction comes from, and it is a property of the
//!   *spawner* rather than of the port.
//!
//!   So this module binds the weaker flavour, as CLAUDE.md's fourth constraint
//!   asks: the exception it grants is a rule that *genuinely* needs `Send`, and
//!   measurement says this one does not. Nothing is lost in practice —
//!   `trait_variant` emits `pub trait SendEventStore: Send`, so every adapter
//!   implementing the `Send` flavour satisfies `EventStore + Send` already, and
//!   the weaker bound additionally admits a store whose handle is `Send` while
//!   its futures are not.
//! * **`Sync` — NOT REQUIRED, because each contender owns its handle.** A handle
//!   is moved into its thread rather than shared by reference, so the obligation
//!   is `F::Store: Send` and never `&F::Store: Send`. It was measured firing when
//!   that discipline slipped: capturing the reader's handle by reference in
//!   `observe_while_writing` gives *`<F as Fixture>::Store` cannot be shared
//!   between threads safely*, and the fix is a `move` closure rather than a
//!   bound. The `Arc<S>` shape CF-22 predicts is what a *task*-based harness
//!   needs; N handles onto one store is the more faithful model of contention
//!   anyway, because that is what a connection pool hands out.
//! * **`'static` — NOT REQUIRED, and this is what scoped threads are for.**
//!   [`std::thread::scope`] guarantees every thread has joined before it returns,
//!   so a contender may borrow from the rule's own frame. `tokio::spawn` cannot
//!   promise that, which is again the spawner rather than the port. Every rule
//!   below relies on it: the condition, the events and the query are built once
//!   on the rule's stack and borrowed by all eight contenders.
//!
//! **The direction this can move is the safe one.** Should phase 10 need the
//! contenders to become tasks on a harness-supplied runtime, the bound tightens
//! to something like CF-22's prediction, and tightening a bound breaks
//! invocations. It is recorded here rather than hedged against: nothing is
//! published, the workspace's every native adapter implements `SendEventStore`
//! and so satisfies either bound, and a bound written for a spawner nobody has
//! yet is a guess with a `where` clause.
//!
//! One bound the port does **not** carry turns out to shape every rule here.
//! `EventStore::Error` has no `Send` bound (ES-6 is deferred), so a
//! `Result<SequencePosition, AppendError<S::Error>>` cannot be returned from a
//! contender's thread. Each contender therefore collapses its result to
//! an `Attempt`, which is `Send` by construction. This is the same wall
//! `spawns_from_generic` hit and solved the same way — it collapses to a
//! `usize` — and it is the line that relaxes if ES-6 is ever settled.
//!
//! # This family carries its own enumeration and its own emitter
//!
//! CF-22 requires one enumeration per rule *family*.
//! [`for_each_concurrency_rule!`](crate::for_each_concurrency_rule) is this
//! one's, beside the rules it names, for the reason `model.rs` gives: `cargo
//! xtask spec-trace` scans `suite.rs` for `pub async fn`, and a rule written
//! there needs a clause of its own.
//!
//! **Fewer wrappers make sense here than in the store families, and that is not
//! a weakening of CF-23.** The emitter is still a *parameter*; what narrows is
//! the set a racing family can be driven by — `#[wasm_bindgen_test]` has no
//! candidate, because this module does not exist on `wasm32`.
//!
//! | Emitter | Wrapper | Adapter needs |
//! |---|---|---|
//! | `__emit_concurrency_tokio` (default) | `#[tokio::test(flavor = "multi_thread")]` | `tokio` with `macros`, `rt-multi-thread` |
//! | `__emit_concurrency_blocking` | `#[test]` + `block_on` | **nothing** |
//!
//! **The table is the count**, in the crate root's shape and for its reason:
//! this paragraph opened with a number that was wrong for as long as its second
//! row has existed. `the_concurrency_page_lists_every_emitter_it_ships` holds
//! these rows to this file's `macro_rules!` definitions and refuses a spelled
//! count here; neither name is a link, for the crate root's reason. The default
//! is `multi_thread` although the parallelism is in [`std::thread::scope`],
//! because an adapter's futures may need a multi-threaded reactor even when the
//! contention does not — and the blocking emitter races exactly as hard, which
//! makes it the honest default for an adapter with **no runtime at all**.

use happenstance_core::{
    AppendError, Event, EventStore, Query, ReadOptions, SequencePosition, SequencedEvent, collect,
};

use crate::Fixture;

/// A [`Fixture`] whose handles can cross a thread boundary.
///
/// # Why the macro needs a name for this and the other two families do not
///
/// `event_store_conformance!` hoists its fixture expression behind
/// `async fn __conformance_fixture() -> impl Fixture`, so that an emitter never
/// has to name the fixture *type* — which it could not, having been handed an
/// expression. The opaque return type carries exactly the bounds written on it
/// and nothing else, so `impl Fixture` tells a caller nothing about
/// `<F as Fixture>::Store` beyond `EventStore`. That is enough for two families
/// and one short of enough for this one: the store also has to be `Send`, and
/// the compiler reports that unsatisfied at every one of the ten call sites the
/// two emitters expand to.
///
/// The fix is a trait whose *supertrait bound* carries the extra obligation, so
/// that `impl ConcurrentFixture` elaborates to both facts. The blanket
/// implementation means no adapter ever writes it: any fixture whose handle can
/// be moved to another thread already is one.
///
/// It is written `Fixture<Store: Send>` rather than
/// `Fixture where Self::Store: Send`, and the difference is not cosmetic — an
/// associated-type bound in the supertrait position is elaborated for users of
/// the opaque type, and a `where` clause on the trait is not.
pub trait ConcurrentFixture: Fixture<Store: Send> {}

impl<F> ConcurrentFixture for F
where
    F: Fixture,
    F::Store: Send,
{
}

/// How many contenders a racing rule starts.
///
/// Public because it is the number an adapter author has to size a connection
/// pool against: a fixture whose pool is smaller than this deadlocks rather than
/// failing, and the suite has no way to tell them apart (CF-33 — there is no
/// watchdog).
///
/// **It is not a tuning knob.** The rules assert *set* properties — exactly one
/// winner, k boundaries admit exactly k, all positions distinct — and every one
/// of them holds at any size above one. Moving this number therefore changes how
/// hard the operating system is asked to interleave, and nothing else. That is
/// what makes it movable at all, and it is also why moving it "to see" is the
/// wrong move: the cost is real and lands somewhere else.
///
/// # Why sixty-four, and what it costs
///
/// It was **eight** for two phases, on the argument that two threads on a
/// multi-core host frequently do not overlap and eight is enough that the
/// operating system has to preempt somewhere. That argument is still true and it
/// is not the reason this number is now 64. Two of the repository's stated proof
/// artefacts read *64 contenders* (`RUNBOOK.md:159`, `:4217-4218`), so an
/// evaluator reading the plan and an adapter author reading this line were told
/// different things — a discrepancy that had been carried for three phases, and
/// the third option, leaving it, is the one that rots.
///
/// ADR-0022 §12 measured the raise against the first file-backed adapter rather
/// than arguing it: sixty-four `rusqlite::Connection`s on one file opened on
/// every one of thirty races with no file-descriptor or connection ceiling
/// reached, exactly one winner per race at both counts, and `busy = 0` and
/// `failed = 0` throughout. **The cost is wall time and it is one order of
/// magnitude** — roughly 10x to 20x per race — which for a five-rule family is
/// the difference between a fraction of a second and a handful of seconds per
/// adapter per CI run.
///
/// The cost is also **workspace-wide**, and that is the part worth stating here
/// rather than in a commit message: this constant is what
/// `crates/happenstance-testkit/tests/memory_concurrency_conformance.rs`, the
/// five racing stores behind
/// `mutation_coverage::the_concurrency_rules_reject_exactly_what_they_claim`,
/// and every future fixture in any adapter crate run at. A fixture whose backing
/// store cannot open sixty-five handles onto one medium now deadlocks where it
/// used to pass — which is precisely what the first paragraph says this number
/// is for.
pub const CONTENDERS: usize = 64;

/// What one contender's `append` produced, collapsed to something `Send`.
///
/// The collapse is forced rather than stylistic. `EventStore::Error` carries no
/// `Send` bound — ES-6 is deferred — so `Result<_, AppendError<S::Error>>`
/// cannot leave the thread that produced it. The error is rendered to a `String`
/// inside the contender, which is the only place it is still allowed to exist.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Attempt {
    /// The append landed, at this position.
    Committed(SequencePosition),
    /// The store reported `ConditionViolated`, which is the DCB retry signal.
    Rejected,
    /// Anything else: a store error, or an empty batch the rule did not send.
    Failed(String),
}

impl Attempt {
    /// Collapses one contender's result, discarding everything that is not
    /// `Send`.
    fn of<E: core::fmt::Display>(result: Result<SequencePosition, AppendError<E>>) -> Self {
        match result {
            Ok(position) => Self::Committed(position),
            Err(AppendError::ConditionViolated(_)) => Self::Rejected,
            Err(other) => Self::Failed(format!("{other}")),
        }
    }

    /// The position, if this attempt committed.
    const fn position(&self) -> Option<SequencePosition> {
        match self {
            Self::Committed(position) => Some(*position),
            _ => None,
        }
    }
}

/// Runs one closure per contender, each on its own thread, and collects what
/// each returned.
///
/// Each contender is handed its handle **by value**, which is what keeps `Sync`
/// off the bound: a moved handle needs `F::Store: Send`, a shared one would need
/// `&F::Store: Send` and therefore `F::Store: Sync`.
///
/// A contender that panics is re-raised on this thread with
/// [`std::panic::resume_unwind`] rather than being reported as a value. That is
/// deliberate: the panic keeps its original payload, so the proof artefact's
/// panic hook still sees the message it would have seen, and a store that falls
/// over is not silently counted as a contender that lost.
///
/// # The starting gate is load-bearing, and it is not a sleep
///
/// Spawning N threads in a loop does not start them together. It was measured
/// not doing so: without the [`Barrier`](std::sync::Barrier) below, the first
/// contenders finished before the last were spawned, a store whose defect needs
/// an overlap was joined by *pairs* rather than by everybody, and the rule it is
/// supposed to fail passed about half the time. A conformance rule that finds a
/// defect on half its runs is a rule that will be re-run until it is green.
///
/// A barrier is the right instrument for that and a `sleep` is not: it
/// synchronises on the other threads rather than on a wall clock, so it is
/// exact on a sixteen-core host and on a loaded single-core runner alike, and
/// there is nothing in it for CF-33 to object to. It cannot deadlock, because
/// its party count is the number of threads [`std::thread::scope`] is about to
/// guarantee have been spawned.
fn race<S, T, B>(stores: Vec<S>, body: B) -> Vec<T>
where
    S: Send,
    T: Send,
    B: Fn(usize, S) -> T + Sync,
{
    let gate = std::sync::Barrier::new(stores.len());

    std::thread::scope(|scope| {
        let running: Vec<_> = stores
            .into_iter()
            .enumerate()
            .map(|(index, store)| {
                let body = &body;
                let gate = &gate;
                scope.spawn(move || {
                    gate.wait();
                    body(index, store)
                })
            })
            .collect();

        running
            .into_iter()
            .map(|thread| match thread.join() {
                Ok(value) => value,
                Err(payload) => std::panic::resume_unwind(payload),
            })
            .collect()
    })
}

/// The rules of the concurrency family.
///
/// Each has the same shape as an event-store rule — `async fn(open) ->
/// RuleOutcome` — and differs only in the bound on `F::Store`. None of them is
/// capability-gated, so none returns [`RuleOutcome::Skipped`](crate::RuleOutcome::Skipped);
/// a fixture that cannot hand out [`CONTENDERS`] handles onto one store already
/// fails CF-16's MUST in the event-store family.
pub mod rules {
    // Every rule panics on failure — that is what a test does, and a `# Panics`
    // section on each of them would say "panics when the adapter is
    // non-conformant" five times over.
    #![allow(clippy::missing_panics_doc)]

    use core::sync::atomic::{AtomicBool, Ordering};

    use happenstance_core::{
        Event, EventStore, Query, ReadOptions, SequencePosition, SequencedEvent,
    };

    use super::{Attempt, CONTENDERS, append_ok, connect_many, race, read_ok, sorted_positions};
    use crate::fixtures::{condition_after, event, query_of, query_of_types, tagged_event};
    use crate::{Fixture, RuleOutcome};

    /// Of `CONTENDERS` handlers that decided from one snapshot, exactly one
    /// commits.
    ///
    /// This is
    /// [`racing_conditional_appends_elect_one_winner`](crate::rules::racing_conditional_appends_elect_one_winner)'s
    /// question asked of a store that is genuinely contended rather than called
    /// twice in a row, and the difference is the whole reason this family
    /// exists. The sequential rule cannot separate an atomic check-and-write
    /// from a probe followed by an insert, because with one caller at a time the
    /// probe and the insert are adjacent and nothing can get between them.
    /// `RacingProbeStore` in the proof artefact is that store: it probes, opens
    /// a window, then inserts, and every sequential rule in the suite passes it.
    ///
    /// The condition is `after: boundary` with the boundary taken from a real
    /// setup append rather than written as a number (CF-6), and its query is
    /// tagged rather than type-only for
    /// [`two_handles_observe_each_others_appends`](crate::rules::two_handles_observe_each_others_appends)'s
    /// reason: the tag join is the first thing an adapter drops from a probe,
    /// and a type-only condition returns the identical verdict here while
    /// testing strictly less.
    pub async fn exactly_one_of_n_contenders_commits<F>(open: impl AsyncFn() -> F) -> RuleOutcome
    where
        F: Fixture,
        F::Store: EventStore + Send,
    {
        let fixture = open().await;
        let boundary = {
            let setup = fixture.connect().await;
            append_ok(&setup, &[event("CourseDefined")]).await
        };

        let query = query_of(&["StudentSubscribed"], &[("course", "c1")]);
        let guard = condition_after(query.clone(), boundary.get());
        let subscribe = tagged_event("StudentSubscribed", &[("course", "c1")]);

        let stores = connect_many(&fixture, CONTENDERS).await;
        let attempts = race(stores, |_, store| {
            crate::block_on(async {
                Attempt::of(
                    store
                        .append(core::slice::from_ref(&subscribe), Some(&guard))
                        .await,
                )
            })
        });

        let failures: Vec<&Attempt> = attempts
            .iter()
            .filter(|attempt| matches!(attempt, Attempt::Failed(_)))
            .collect();
        assert!(
            failures.is_empty(),
            "a contender must either commit or be told `ConditionViolated`; \
             anything else is a store failure under contention rather than the \
             concurrency signal. Got {failures:?}"
        );

        let committed: Vec<SequencePosition> =
            attempts.iter().filter_map(Attempt::position).collect();
        assert_eq!(
            committed.len(),
            1,
            "exactly one of {CONTENDERS} handlers that decided from the same \
             snapshot may commit — more than one means the consistency boundary \
             is not enforced under contention, none means the store rejected a \
             decision nothing had invalidated. Attempts: {attempts:?}"
        );

        let observer = fixture.connect().await;
        let landed = read_ok(&observer, &query, ReadOptions::new()).await;
        assert_eq!(
            sorted_positions(&landed),
            committed,
            "the store must hold exactly the one batch it acknowledged, at the \
             position it told the winner"
        );

        RuleOutcome::Ran
    }

    /// Contenders on *different* consistency boundaries do not conflict.
    ///
    /// `BOUNDARIES` disjoint queries, `PER_BOUNDARY` contenders each, all
    /// racing at once: every boundary must elect exactly one winner, so exactly
    /// `BOUNDARIES` batches land.
    ///
    /// # Why this is the shape of the runbook's "K seats" and not its letter
    ///
    /// The work item asks for K seats and exactly K commits — a capacity
    /// boundary, N contenders competing for K places. That formulation needs a
    /// **retry loop**: a contender that loses re-reads and tries again until the
    /// seats are full, and the loop needs a budget or it is a livelock waiting
    /// to happen. A budget is an operation count, and asserting a rule fails
    /// when the budget is exhausted is asserting on an operation count, which
    /// CF-33 `[FROZEN]` forbids for the same anti-flake reason it forbids
    /// clocks.
    ///
    /// K disjoint boundaries buy the same *arithmetic* — exactly K commits out
    /// of N attempts — with one attempt each and no loop, and they reject a
    /// defect the capacity form cannot see at all: an adapter whose concurrency
    /// control is coarser than its condition. A Postgres store under
    /// `SERIALIZABLE` that maps `40001 serialization_failure` onto
    /// `AppendError::ConditionViolated` is conformant on every sequential rule —
    /// a sequential writer never serialisation-fails — and reports contention
    /// between two commands that share nothing. `SpuriousConflictStore` in the
    /// proof artefact is that adapter.
    ///
    /// The false-negative direction is checked in the same breath: a store that
    /// let two contenders onto one boundary fails the per-boundary count.
    pub async fn k_disjoint_boundaries_admit_exactly_k_commits<F>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome
    where
        F: Fixture,
        F::Store: EventStore + Send,
    {
        /// Distinct consistency boundaries, each with its own query.
        const BOUNDARIES: usize = 4;
        /// Contenders per boundary. Two would do; three leaves a loser behind
        /// every winner even if one contender is descheduled for the whole run.
        const PER_BOUNDARY: usize = 3;

        let fixture = open().await;
        let courses: Vec<String> = (0..BOUNDARIES).map(|index| format!("c{index}")).collect();

        let boundary = {
            let setup = fixture.connect().await;
            let defined: Vec<_> = courses
                .iter()
                .map(|course| tagged_event("CourseDefined", &[("course", course.as_str())]))
                .collect();
            append_ok(&setup, &defined).await
        };

        // Built once, on this frame, and borrowed by every contender — which is
        // what `std::thread::scope` allows and `tokio::spawn` would not.
        let queries: Vec<Query> = courses
            .iter()
            .map(|course| query_of(&["StudentSubscribed"], &[("course", course.as_str())]))
            .collect();
        let guards: Vec<_> = queries
            .iter()
            .map(|query| condition_after(query.clone(), boundary.get()))
            .collect();
        let subscribes: Vec<_> = courses
            .iter()
            .map(|course| tagged_event("StudentSubscribed", &[("course", course.as_str())]))
            .collect();

        let contenders = BOUNDARIES * PER_BOUNDARY;
        let stores = connect_many(&fixture, contenders).await;
        let attempts = race(stores, |index, store| {
            let seat = index % BOUNDARIES;
            crate::block_on(async {
                Attempt::of(
                    store
                        .append(
                            core::slice::from_ref(&subscribes[seat]),
                            Some(&guards[seat]),
                        )
                        .await,
                )
            })
        });

        let failures: Vec<&Attempt> = attempts
            .iter()
            .filter(|attempt| matches!(attempt, Attempt::Failed(_)))
            .collect();
        assert!(
            failures.is_empty(),
            "a contender must either commit or be told `ConditionViolated`. Got \
             {failures:?}"
        );

        let observer = fixture.connect().await;
        for (seat, query) in queries.iter().enumerate() {
            let won: Vec<&Attempt> = attempts
                .iter()
                .enumerate()
                .filter(|(index, attempt)| {
                    index % BOUNDARIES == seat && attempt.position().is_some()
                })
                .map(|(_, attempt)| attempt)
                .collect();
            // Two assertions rather than one `assert_eq!(won.len(), 1)`, and the
            // split is what makes the two failures nameable. Zero winners and
            // two winners are opposite defects — an over-broad conflict check
            // and an under-enforced condition — and a single message describing
            // both is a message that identifies neither. The proof artefact
            // pins each store to the one it is supposed to trip, which it could
            // not do while they shared a string.
            assert!(
                !won.is_empty(),
                "boundary {seat} of {BOUNDARIES} elected no winner out of \
                 {PER_BOUNDARY} contenders. Nothing had invalidated any of \
                 them: their conditions name a query no other boundary writes \
                 to. A store that answers `ConditionViolated` here is reporting \
                 a conflict between commands that share nothing — a global \
                 version check, or a serialisation failure mapped onto the DCB \
                 retry signal. All attempts: {attempts:?}"
            );
            assert_eq!(
                won.len(),
                1,
                "boundary {seat} of {BOUNDARIES} must elect at most one winner \
                 out of {PER_BOUNDARY} contenders, and elected {}. All of them \
                 decided from the same snapshot and all of them named the same \
                 query, so the condition did not hold under contention. All \
                 attempts: {attempts:?}",
                won.len()
            );

            let landed = read_ok(&observer, query, ReadOptions::new()).await;
            assert_eq!(
                landed.len(),
                1,
                "and boundary {seat} must hold exactly the one batch it \
                 acknowledged"
            );
        }

        RuleOutcome::Ran
    }

    /// Concurrent unconditional appends never share a position.
    ///
    /// [`positions_are_unique`](crate::rules::positions_are_unique) reads a
    /// quiescent store back after a single sequential writer, so it measures
    /// *assignment* and says nothing about assignment under contention. The
    /// defect it cannot see is a store that reads its head, suspends, and then
    /// allocates from the value it read — the in-process form of `SELECT
    /// max(position)` outside the transaction that will use it, and the shape
    /// `RacingSequenceStore` models.
    ///
    /// The batch is two events rather than one on purpose: a store that binds
    /// one position for a whole multi-row insert is invisible to a
    /// singleton-only writer, which is what `SharedBatchPositionStore` taught
    /// the sequential rules at stage 3.
    pub async fn positions_are_unique_under_concurrent_appends<F>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome
    where
        F: Fixture,
        F::Store: EventStore + Send,
    {
        /// Events per contender.
        const BATCH: usize = 2;

        let fixture = open().await;
        let batch = [event("Appended"), event("Appended")];

        let stores = connect_many(&fixture, CONTENDERS).await;
        let attempts = race(stores, |_, store| {
            crate::block_on(async { Attempt::of(store.append(&batch, None).await) })
        });

        let committed: Vec<SequencePosition> =
            attempts.iter().filter_map(Attempt::position).collect();
        assert_eq!(
            committed.len(),
            CONTENDERS,
            "an unconditional append has nothing to be rejected by, so every \
             contender must commit. Attempts: {attempts:?}"
        );

        // Nothing here asserts that the *returned* positions are distinct, and
        // the omission is deliberate rather than an oversight. It was written,
        // it failed `GlobalHeadStore`, and it was removed on that evidence: a
        // store which commits correctly and then answers with the table's head
        // hands two callers the same number without ever having assigned it
        // twice. That defect is
        // [`append_returns_the_callers_own_last_position`]'s, and asserting it
        // here would give one defect two rules to fail and leave a reader of
        // this rule's message — "the sequence was read across a suspension" —
        // looking for a bug in the wrong half of the store.
        let observer = fixture.connect().await;
        let all = read_ok(&observer, &Query::all(), ReadOptions::new()).await;
        assert_eq!(
            all.len(),
            CONTENDERS * BATCH,
            "every event of every contender's batch must be in the store"
        );

        let mut positions = sorted_positions(&all);
        let total = positions.len();
        positions.dedup();
        assert_eq!(
            positions.len(),
            total,
            "no two events anywhere in the store may share a position"
        );

        RuleOutcome::Ran
    }

    /// `append` returns the caller's **own** last position, not the store's
    /// head.
    ///
    /// Sequentially the two are the same value, which is why no rule in the
    /// event-store family can tell them apart: a single writer's last event
    /// *is* the head. Under contention they diverge, and the divergence is a
    /// real adapter shape — `INSERT …; SELECT max(position) FROM events` instead
    /// of `INSERT … RETURNING position`, which is what an adapter writes when
    /// its driver does not support `RETURNING` on a multi-row insert.
    ///
    /// The consequence is not cosmetic. A caller checkpoints a projection or
    /// builds the next `AppendCondition::after` from this value, so a head
    /// borrowed from somebody else's transaction silently skips every event
    /// between the caller's own last one and it.
    pub async fn append_returns_the_callers_own_last_position<F>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome
    where
        F: Fixture,
        F::Store: EventStore + Send,
    {
        /// Events per contender. Must be at least two, or "the caller's last"
        /// and "the caller's only" are the same event.
        const BATCH: usize = 2;

        let fixture = open().await;
        let writers: Vec<String> = (0..CONTENDERS).map(|index| format!("w{index}")).collect();
        let batches: Vec<Vec<_>> = writers
            .iter()
            .map(|writer| {
                (0..BATCH)
                    .map(|_| tagged_event("Written", &[("writer", writer.as_str())]))
                    .collect()
            })
            .collect();

        let stores = connect_many(&fixture, CONTENDERS).await;
        let attempts = race(stores, |index, store| {
            crate::block_on(async { Attempt::of(store.append(&batches[index], None).await) })
        });

        let observer = fixture.connect().await;
        for (index, writer) in writers.iter().enumerate() {
            let Some(returned) = attempts[index].position() else {
                panic!("contender {index} did not commit an unconditional append: {attempts:?}");
            };

            let mine = read_ok(
                &observer,
                &query_of(&["Written"], &[("writer", writer.as_str())]),
                ReadOptions::new(),
            )
            .await;
            assert_eq!(
                mine.len(),
                BATCH,
                "contender {index}'s batch must be in the store whole"
            );

            let last = sorted_positions(&mine).pop();
            assert_eq!(
                last,
                Some(returned),
                "`append` must return the position of the caller's own last \
                 event. Contender {index} was told {returned:?} and its own \
                 events are at {:?} — a store that answers with its head hands \
                 the caller a checkpoint that skips somebody else's batch",
                sorted_positions(&mine)
            );
        }

        RuleOutcome::Ran
    }

    /// A reader running against a live writer never sees part of a batch.
    ///
    /// ES-18's atomicity from the only side that can observe it. A sequential
    /// rule can assert that a *rejected* batch left nothing behind
    /// ([`append_is_atomic`](crate::rules::append_is_atomic)); it cannot assert
    /// that an *accepted* batch was never visible half-written, because by the
    /// time a sequential reader looks, the append has returned. The shape this
    /// rejects is a row-at-a-time insert loop outside a transaction, which is
    /// what an adapter writes when its driver has no multi-row insert and it
    /// forgets the `BEGIN`.
    ///
    /// # Two halves, and only one of them is deterministic
    ///
    /// The **post-hoc** half is: once every writer has finished, every batch is
    /// present and complete. That holds on every run and rejects a store that
    /// permanently loses part of a batch.
    ///
    /// The **live** half is a sampling test, and saying so is better than
    /// implying otherwise. The reader loops until the writers signal completion
    /// and checks every batch it can see; a store that is *never* caught
    /// mid-batch passes. It cannot flake in the direction that matters — a
    /// conformant store has no partial state to be caught in, so this half never
    /// fails a correct adapter — and the loop is bounded by the writers rather
    /// than by a count or a clock, so there is nothing here for CF-33 to object
    /// to. What it can do is fail to catch a genuine defect on an unlucky run,
    /// which is why the proof artefact's `RowAtATimeStore` does not rely on luck:
    /// it holds each partial batch open until the reader has completed a read
    /// that started after the partial write, using an atomic counter and
    /// `std::thread::yield_now`, so the rejection is a rendezvous rather than a
    /// race.
    ///
    /// That rendezvous needs the reader to *exist* before the first writer runs,
    /// and a spawned thread is not a scheduled one. `observe_while_writing`
    /// below therefore completes one read on the rule's own thread before the
    /// scope opens — spelled plainly rather than as an intra-doc link, because
    /// it is private and linking to it from a public item is a rustdoc warning
    /// the gate denies. The comment there records what its absence was measured
    /// costing.
    pub async fn a_concurrent_reader_never_sees_a_partial_batch<F>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome
    where
        F: Fixture,
        F::Store: EventStore + Send,
    {
        /// Concurrent writers. One reader runs alongside them.
        const WRITERS: usize = 4;
        /// Batches each writer appends, one after another.
        const ROUNDS: usize = 4;
        /// Events per batch. Three, so that "some but not all" has two ways to
        /// happen rather than one.
        const BATCH: usize = 3;

        let fixture = open().await;

        // One event *type* per batch, so a reader can group by it with no tag
        // parsing. The type is the batch's identity; `BATCH` copies of it are
        // one unit of work.
        let types: Vec<String> = (0..WRITERS)
            .flat_map(|writer| (0..ROUNDS).map(move |round| format!("Batch{writer}x{round}")))
            .collect();
        let batches: Vec<Vec<_>> = types
            .iter()
            .map(|name| (0..BATCH).map(|_| event(name)).collect())
            .collect();

        let mut stores = connect_many(&fixture, WRITERS + 1).await;
        // The reader's handle is the odd one out, so it comes off the end and
        // the writers keep dense indices.
        let Some(reading) = stores.pop() else {
            panic!("connect_many returned nothing for {WRITERS} writers and a reader");
        };

        let (committed, partial) =
            observe_while_writing(stores, reading, &batches, &types, ROUNDS, BATCH);
        let failures: Vec<&Attempt> = committed
            .iter()
            .filter(|attempt| attempt.position().is_none())
            .collect();
        assert!(
            failures.is_empty(),
            "an unconditional append has nothing to be rejected by. Got \
             {failures:?}"
        );

        assert!(
            partial.is_empty(),
            "a reader observed {} batch(es) part-written: {partial:?}. Either \
             every event of a batch is visible or none is (ES-18), and a reader \
             that can see half of one can build a decision model from a command \
             that never completed",
            partial.len()
        );

        // The post-hoc half. This one is deterministic.
        let observer = fixture.connect().await;
        let all = read_ok(&observer, &Query::all(), ReadOptions::new()).await;
        assert_eq!(
            all.len(),
            WRITERS * ROUNDS * BATCH,
            "every event of every acknowledged batch must still be there once \
             the writers have finished"
        );
        for name in &types {
            let found = read_ok(
                &observer,
                &query_of_types(&[name.as_str()]),
                ReadOptions::new(),
            )
            .await;
            assert_eq!(
                found.len(),
                BATCH,
                "batch `{name}` was acknowledged and must be present whole"
            );
        }

        RuleOutcome::Ran
    }

    /// Runs `writers` in parallel with one reader, and returns what each saw.
    ///
    /// Extracted from the rule above rather than inlined, and not only because
    /// `clippy::too_many_lines` said so: the shape here is the one thing in this
    /// family that is *not* symmetric — one role reads, the others write — and
    /// it is easier to check that the reader is stopped by the writers rather
    /// than by anything else when the two are the whole of one function.
    fn observe_while_writing<S: EventStore + Send>(
        writers: Vec<S>,
        reading: S,
        batches: &[Vec<Event>],
        types: &[String],
        rounds: usize,
        batch: usize,
    ) -> (Vec<Attempt>, Vec<String>) {
        let done = AtomicBool::new(false);

        // One read on *this* thread before any writer is spawned, and it is
        // load-bearing rather than a warm-up.
        //
        // `Scope::spawn` starts a thread; it does not schedule one. The writer
        // phase below is a few dozen appends of in-memory work with no I/O in
        // it, so on a host with no spare core — an oversubscribed CI runner is
        // the case measured — every writer can finish before the reader's thread
        // is scheduled even once. The live half then samples a store that is
        // already quiescent and degenerates silently into the post-hoc half,
        // which is the worst failure an instrument has: it keeps passing.
        //
        // This read establishes a happens-before that no scheduler can take
        // away — a read of this store has completed before the first append is
        // issued — so a store that synchronises against its readers has one to
        // find. The proof artefact's `RowAtATimeStore` is exactly that store,
        // and without this line it was measured losing its rejection on ten of
        // twenty-four oversubscribed runs.
        //
        // The result is dropped rather than asserted on. Nothing has been
        // appended yet, so there is no batch it could have seen part of, and a
        // store whose reads fail outright is reported by the reader thread —
        // which calls the same function and folds the failure into `partial`.
        // Asserting here would put a second, differently-worded failure in front
        // of that one.
        drop(incomplete_batches(&reading, types, batch));

        std::thread::scope(|scope| {
            // The reader's handle is *moved* into its thread, so the bound stays
            // `S: Send`. Capturing `&reading` instead would demand `S: Sync` —
            // exactly the bound this family was written to do without — and the
            // compiler says so in terms: *cannot be shared between threads
            // safely*. `done` and `types` are shared, so they are rebound as
            // references before the `move` closure can swallow them whole.
            let flag = &done;
            let names = types;
            let reader = scope.spawn(move || {
                let mut partial: Vec<String> = Vec::new();
                // Bounded by the writers rather than by a count or a clock: the
                // flag is set once every writer has joined, and the final pass
                // below runs after it, so the loop cannot spin forever and
                // cannot exit before the writers have started.
                while !flag.load(Ordering::Acquire) {
                    partial.extend(incomplete_batches(&reading, names, batch));
                }
                partial.extend(incomplete_batches(&reading, names, batch));
                partial
            });

            let writing: Vec<_> = writers
                .into_iter()
                .enumerate()
                .map(|(writer, store)| {
                    scope.spawn(move || {
                        crate::block_on(async {
                            let mut attempts = Vec::with_capacity(rounds);
                            for round in 0..rounds {
                                let index = writer * rounds + round;
                                attempts
                                    .push(Attempt::of(store.append(&batches[index], None).await));
                            }
                            attempts
                        })
                    })
                })
                .collect();

            // Every writer is joined and its result *held* before anything is
            // propagated, because the order here is the difference between a
            // named failure and a hung CI job.
            //
            // The obvious spelling — `resume_unwind` inside the join loop — is
            // a deadlock. The reader's only exit is `done`, and `resume_unwind`
            // would leave the loop before `done` is set; `std::thread::scope`
            // then waits for every scoped thread *even while unwinding*, so the
            // reader spins forever on a flag nobody will ever set and the scope
            // never returns. There is deliberately no watchdog anywhere in this
            // family (CF-33), so that converts a store panicking under
            // contention — precisely the class of store this family is pointed
            // at — into a silent timeout naming no rule.
            //
            // Setting the flag first costs one `Vec` and keeps the payload
            // intact, which is what `race`'s note says the `resume_unwind` is
            // for: the proof artefact's panic hook still sees the original
            // message and its original location.
            let joined: Vec<std::thread::Result<Vec<Attempt>>> = writing
                .into_iter()
                .map(std::thread::ScopedJoinHandle::join)
                .collect();
            done.store(true, Ordering::Release);
            let observed = reader.join();

            let mut committed = Vec::new();
            for writer in joined {
                match writer {
                    Ok(attempts) => committed.extend(attempts),
                    Err(payload) => std::panic::resume_unwind(payload),
                }
            }

            match observed {
                Ok(partial) => (committed, partial),
                Err(payload) => std::panic::resume_unwind(payload),
            }
        })
    }

    /// The names of every batch the store is currently showing incompletely.
    ///
    /// Runs on the reader's thread, so it must return something `Send`: the
    /// batch names, not the events, and certainly not a `Result` carrying
    /// `S::Error`. A read that fails is reported as a sighting of its own, which
    /// is the honest answer — a reader that cannot read while a writer is
    /// working is a defect this rule is entitled to name.
    fn incomplete_batches<S: EventStore>(store: &S, types: &[String], batch: usize) -> Vec<String> {
        let seen: Vec<SequencedEvent> = match crate::block_on(happenstance_core::collect(
            store.read(&Query::all(), ReadOptions::new()),
        )) {
            Ok(events) => events,
            Err(err) => return std::vec![format!("a concurrent read failed: {err}")],
        };

        types
            .iter()
            .filter(|name| {
                let count = seen
                    .iter()
                    .filter(|event| event.event_type().as_str() == name.as_str())
                    .count();
                count != 0 && count != batch
            })
            .cloned()
            .collect()
    }
}

// ---------------------------------------------------------------------
// Helpers shared by the rules
//
// Bounded on `EventStore` rather than on `Fixture`, for `suite.rs`'s reason:
// they operate on a handle, and a handle is a store. `Send` is absent here and
// present on the rules, which is exactly the seam: these run on the rule's own
// thread and never cross one.
// ---------------------------------------------------------------------

/// Opens `count` handles onto one backing store.
///
/// Sequentially, on the rule's own thread. A fixture whose `connect` is itself
/// contended is not what any rule here is testing, and opening the handles in
/// parallel would put a second variable into every failure message.
async fn connect_many<F: Fixture>(fixture: &F, count: usize) -> Vec<F::Store> {
    let mut stores = Vec::with_capacity(count);
    for _ in 0..count {
        stores.push(fixture.connect().await);
    }
    stores
}

/// Appends and unwraps, failing the test with context on error.
async fn append_ok<S: EventStore>(store: &S, events: &[Event]) -> SequencePosition {
    match store.append(events, None).await {
        Ok(position) => position,
        Err(err) => panic!("unconditional append should succeed, got {err:?}"),
    }
}

/// Reads and unwraps, failing the test with context on error.
async fn read_ok<S: EventStore>(
    store: &S,
    query: &Query,
    options: ReadOptions,
) -> Vec<SequencedEvent> {
    match collect(store.read(query, options)).await {
        Ok(events) => events,
        Err(err) => panic!("read should succeed, got {err:?}"),
    }
}

/// The positions of a read result, ascending.
///
/// Sorted rather than taken in read order, because every assertion built on it
/// is about the *set* the store assigned. `read` already returns ascending
/// order and a rule that relied on that would be asserting ES-8 by accident,
/// under a name that says nothing about it.
fn sorted_positions(events: &[SequencedEvent]) -> Vec<SequencePosition> {
    let mut positions: Vec<SequencePosition> = events.iter().map(|event| event.position).collect();
    positions.sort_unstable();
    positions
}

// =====================================================================
// The enumeration, the emitter, and the macro
// =====================================================================

/// Hands the complete concurrency rule set to `$callback`.
///
/// This family's single enumeration (CF-22). It lives here beside the rules it
/// names for `model.rs`'s reason: `cargo xtask spec-trace` reads `suite.rs`, and
/// a rule written there would need a clause of its own.
///
/// # Examples
///
/// ```
/// macro_rules! rule_names {
///     ($($name:ident),* $(,)?) => { [ $( stringify!($name) ),* ] };
/// }
///
/// let names = happenstance_testkit::for_each_concurrency_rule!(rule_names);
/// assert!(names.contains(&"exactly_one_of_n_contenders_commits"));
/// ```
#[macro_export]
macro_rules! for_each_concurrency_rule {
    // Raw token trees rather than `$cb:path`, for the reason
    // `for_each_event_store_rule!` records: a parsed `path` fragment cannot sit
    // in callee position inside an expression, which would forbid the
    // `let names = …` form the example above depends on.
    ($($callback:tt)+) => {
        $($callback)+! {
            exactly_one_of_n_contenders_commits,
            k_disjoint_boundaries_admit_exactly_k_commits,
            positions_are_unique_under_concurrent_appends,
            append_returns_the_callers_own_last_position,
            a_concurrent_reader_never_sees_a_partial_batch,
        }
    };
}

/// Emits one `#[tokio::test(flavor = "multi_thread")]` per concurrency rule.
///
/// The multi-thread flavour is not what makes the contenders parallel — that is
/// [`std::thread::scope`], inside the rules — but an adapter whose own futures
/// need a reactor under them will want one, and a `current_thread` runtime whose
/// only thread is blocked inside a scope cannot drive anything else.
///
/// The caller's crate needs `tokio` with `macros`, `rt` and `rt-multi-thread`.
#[doc(hidden)]
#[macro_export]
macro_rules! __emit_concurrency_tokio {
    ($($name:ident),* $(,)?) => {
        $(
            #[tokio::test(flavor = "multi_thread")]
            async fn $name() {
                $crate::concurrency::rules::$name(__conformance_fixture)
                    .await
                    .report(::core::stringify!($name));
            }
        )*
    };
}

/// Emits one plain `#[test]` per concurrency rule, driven by
/// [`block_on`](crate::block_on).
///
/// Shipped because CF-23's content is that the wrapper is a *parameter*, and a
/// family with exactly one emitter reads as a family that forgot. It is also the
/// honest default for an adapter with no runtime at all: the parallelism is in
/// [`std::thread::scope`], so this emitter races exactly as hard as the tokio
/// one.
#[doc(hidden)]
#[macro_export]
macro_rules! __emit_concurrency_blocking {
    ($($name:ident),* $(,)?) => {
        $(
            #[test]
            fn $name() {
                $crate::block_on($crate::concurrency::rules::$name(__conformance_fixture))
                    .report(::core::stringify!($name));
            }
        )*
    };
}

/// Generates the concurrency conformance suite for an event store adapter.
///
/// **Opt-in.** An adapter invokes this separately from
/// [`event_store_conformance!`](crate::event_store_conformance), and an adapter
/// whose store is `!Send` — the Cloudflare Durable Object is the workspace's —
/// cannot invoke it and must not be expected to. The bound is
/// `F::Store: EventStore + Send`: the *handle* has to be movable between
/// threads, which is precisely the property ADR-0001 exists to keep optional.
///
/// It does not exist on `wasm32-unknown-unknown`, which has no threads to race
/// on. Gate the invocation as this crate's own
/// `tests/memory_concurrency_conformance.rs` does.
///
/// # Examples
///
/// ```
/// # macro_rules! ignore { ($($t:tt)*) => {} }
/// # ignore! {
/// use happenstance_testkit::fixtures::MemoryFixture;
///
/// happenstance_testkit::event_store_concurrency_conformance!(MemoryFixture::new());
/// # }
/// ```
#[macro_export]
macro_rules! event_store_concurrency_conformance {
    (mod_name = $mod_name:ident, emit = $emit:path, fixture = $fixture:expr) => {
        mod $mod_name {
            #![allow(clippy::unwrap_used, unused_imports)]

            use super::*;

            // Named exactly as `event_store_conformance!`'s is, so a
            // caller-supplied emitter — CF-23's extension point — drives any of
            // the three families without knowing which one it was handed.
            //
            // `ConcurrentFixture` rather than `Fixture`, and that trait exists
            // for this line: an opaque return type carries only the bounds
            // written on it, so `impl Fixture` would leave every rule's
            // `F::Store: Send` unprovable at the call site.
            async fn __conformance_fixture() -> impl $crate::concurrency::ConcurrentFixture {
                $fixture
            }

            $crate::for_each_concurrency_rule!($emit);
        }
    };
    (mod_name = $mod_name:ident, fixture = $fixture:expr) => {
        $crate::event_store_concurrency_conformance!(
            mod_name = $mod_name,
            emit = $crate::__emit_concurrency_tokio,
            fixture = $fixture
        );
    };
    ($fixture:expr) => {
        $crate::event_store_concurrency_conformance!(
            mod_name = dcb_concurrency_conformance,
            emit = $crate::__emit_concurrency_tokio,
            fixture = $fixture
        );
    };
}
