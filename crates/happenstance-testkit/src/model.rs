//! The model-based half of the suite: a store driven through a generated
//! sequence of operations, checked against a model of what the log should be.
//!
//! # What this adds that the named rules cannot
//!
//! Every rule in [`rules`](crate::rules) is a worked example: these events, this
//! query, that result. Twenty-seven of them were measured letting four plausible
//! wrong adapters through, and the reason is combinatorial rather than
//! editorial — `query × from × backwards × limit × condition × position policy`
//! is a space no hand-written example set covers, and the interactions are
//! exactly where an adapter's generated SQL goes wrong. This module generates
//! that space instead of enumerating it.
//!
//! It is **additive**, in the same sense the concurrency family is: it replaces
//! no rule. A named rule says *what* is wrong when it fails; this one says
//! *which sequence* made it wrong, minimised, and it is the only thing in the
//! suite that ever runs `read` against a store whose contents it did not choose.
//!
//! # Why the model is not agreeing with itself
//!
//! The model is built on [`Query::matches`] and
//! [`AppendCondition::is_violated_by`], and that is not circular: each is pinned
//! to an independently-written naive definition by a property test in
//! `crates/happenstance-testkit/tests/properties.rs` —
//! `query_matches_agrees_with_a_naive_definition` and
//! `is_violated_by_agrees_with_a_naive_definition` — so a model that agrees with
//! them is not agreeing with itself.
//!
//! That sentence was **false of this tree until phase 3 stage 5**, and the next
//! reader needs to be able to tell which side of that line they are on.
//! `Query::matches` was constrained only *algebraically* — order-insensitivity,
//! monotonicity, `Query::all` as the top element — and every one of those three
//! is satisfied by a function that returns `true` unconditionally.
//! `is_violated_by` had no property at all: three hand-written unit tests in
//! `happenstance-core` and nothing else. The two oracle properties were written
//! first, and deliberately so; a model whose grounding is a claim rather than a
//! test is a second implementation of the contract, and two implementations that
//! agree prove only that one author wrote both.
//!
//! The rest of the model — ordering, the inclusive `from` bound, truncation,
//! batch sequencing — is written out here rather than delegated, because those
//! are the parts the store is being *asked about*.
//!
//! # Symbolic position anchors
//!
//! A generated `ReadOptions::from(5)` is meaningless against a store that
//! assigns positions in steps of seven starting at 4096, and that store exists:
//! `GappedPositionStore` in `tests/mutation_coverage/variants.rs`. So the
//! generator never emits a position. It emits an [`Anchor`] — a *description* of
//! a position — and the runner resolves it at execution against what the store
//! has actually assigned so far.
//!
//! This is CF-6 promoted from an assertion rule to a **generation** rule. The
//! house rule says no assertion may name a literal position; a generator that
//! emitted literals would satisfy the letter of that and violate all of it, by
//! feeding the store inputs whose meaning depends on a policy the specification
//! leaves free. Resolution happens **once** per operation and the resolved value
//! is handed to both the model and the store, so the two can never disagree
//! about which position was meant — only about what the store should do with it.
//!
//! # This family carries its own enumeration, and its own emitters
//!
//! CF-22 requires exactly one enumeration *per rule family* and forbids a second
//! list of the same family. [`for_each_model_rule!`](crate::for_each_model_rule)
//! is this family's, and it lives here beside the rules it names rather than in
//! `suite.rs` — `cargo xtask spec-trace` scans `suite.rs` for `pub async fn`, so
//! a model rule written there would need a clause claiming it by name, and this
//! is one macro rather than N named rules.
//!
//! The emitters are duplicated for a smaller reason, stated so nobody
//! consolidates it by accident: an emitter is invoked as
//! `emitter!(rule_a, rule_b, …)` and the *module path* of the rules is baked
//! into its expansion, not passed. Threading a path through would change a
//! contract that three shipped emitters, three in-tree harnesses and CF-23 all
//! depend on, to save eight lines. The alternative anyone tries first —
//! re-exporting the model rules into [`crate::rules`] so `__emit_tokio` finds
//! them — is worse than duplication: it puts an unregistered name into the
//! module `no_orphan_rules` scans, in the one blind spot that test documents
//! (it cannot see a `pub use`).
//!
//! # Feature and target gating
//!
//! The whole module is behind the off-by-default `proptest` feature *and*
//! `not(target_arch = "wasm32")`, so [`event_store_model_conformance!`] does not
//! exist on the target where `proptest` does not build. An adapter that ships a
//! wasm32 harness must gate its invocation the same way this crate's own
//! `tests/memory_model_conformance.rs` does. The mandatory wasm32 step of
//! `cargo xtask ci` is what would otherwise find this.
//!
//! [`Query::matches`]: happenstance_core::Query::matches
//! [`AppendCondition::is_violated_by`]: happenstance_core::AppendCondition::is_violated_by
//! [`event_store_model_conformance!`]: crate::event_store_model_conformance

use happenstance_core::{
    AppendCondition, AppendError, Event, EventStore, Query, ReadOptions, SequencePosition,
    SequencedEvent, collect,
};
use proptest::prelude::{Strategy, prop, prop_oneof};

use crate::Fixture;
use crate::fixtures::strategies::{any_event, any_query};

// =====================================================================
// The generated vocabulary
// =====================================================================

/// A position named by *description* rather than by value.
///
/// The generator emits these; the runner resolves one into a real
/// [`SequencePosition`] against the log the store has built so far. See the
/// module documentation for why a generated literal would be meaningless.
///
/// The runbook spells the first variant `None`. It is `Unset` here so that a
/// `match` arm cannot be read as [`Option::None`] — the resolution of every
/// variant *is* an `Option`, and two `None`s in one match are one mistake away
/// from each other.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Anchor {
    /// No position at all: an unbounded read, or a condition over the whole log.
    Unset,
    /// The lowest position the store has assigned.
    First,
    /// A position in the middle of the assigned range.
    ///
    /// **It coincides with the neighbouring anchors on short logs**, and a
    /// shrunk counterexample is exactly where short logs live, so the next
    /// reader needs to know before they go looking at the wrong half of their
    /// adapter. Resolution is `positions[len / 2]`, so `Middle` *is* `Head` at
    /// `len == 2` and *is* both `First` and `Head` at `len == 1`; a strictly
    /// interior position only exists from three events up.
    ///
    /// Biasing the index down — `(len - 1) / 2` — was considered and rejected:
    /// it only moves the collapse at `len == 2` from `Head` to `First`, which is
    /// no less misleading, and it perturbs the deterministic generator that the
    /// proof artefact's per-store coverage claims are pinned against. Naming the
    /// coincidence costs nothing and is honest at every length.
    Middle,
    /// The highest position the store has assigned.
    Head,
    /// A position the store has **not** assigned, above everything it has.
    BeyondHead,
}

/// One generated operation against the store under test.
///
/// The three are the whole of the port's surface: an unconditional append, a
/// conditional one, and a read. There is deliberately no empty-batch variant —
/// ES-20's ordering of `NoEvents` against `ConditionViolated` is pinned by two
/// named rules, and generating it here would re-assert a settled thing in a test
/// whose failures are harder to read.
#[derive(Debug, Clone)]
pub enum Op {
    /// Append a batch with no condition.
    Append {
        /// The batch. Never empty.
        events: Vec<Event>,
    },
    /// Append a batch guarded by a condition whose boundary is symbolic.
    AppendConditional {
        /// The batch. Never empty.
        events: Vec<Event>,
        /// What the condition fails on.
        query: Query,
        /// The condition's exclusive `after` boundary.
        anchor: Anchor,
    },
    /// Read, with every read option in play.
    Read {
        /// What to match.
        query: Query,
        /// The inclusive `from` bound.
        from: Anchor,
        /// Whether to read newest-first.
        backwards: bool,
        /// A truncation, if any. Never zero — `ReadOptions::limit(0)` is
        /// `None` today and VT-28 owns changing that, in phase 4.
        limit: Option<usize>,
    },
}

/// Generates an [`Anchor`], weighted towards the ones with an edge in them.
fn any_anchor() -> impl Strategy<Value = Anchor> {
    prop::sample::select(vec![
        Anchor::Unset,
        Anchor::First,
        Anchor::Middle,
        Anchor::Head,
        Anchor::BeyondHead,
    ])
}

/// Generates a batch of one to three events.
///
/// **Multi-event batches are the point.** `SharedBatchPositionStore` — one
/// position bound for every row of a multi-row insert — is invisible to every
/// rule that appends one event at a time, which was every rule in the suite
/// until CF-1 forced the pairing. A generator that only ever wrote singletons
/// would have the same blind spot.
fn any_batch() -> impl Strategy<Value = Vec<Event>> {
    prop::collection::vec(any_event(), 1..4)
}

/// Generates one [`Op`].
fn any_op() -> impl Strategy<Value = Op> {
    prop_oneof![
        3 => any_batch().prop_map(|events| Op::Append { events }),
        3 => (any_batch(), any_query(), any_anchor()).prop_map(|(events, query, anchor)| {
            Op::AppendConditional { events, query, anchor }
        }),
        // Reads are weighted highest because they are where the option
        // interactions live, and because they are the only op that cannot
        // change the state the later ops are checked against.
        4 => (any_query(), any_anchor(), prop::bool::ANY, prop::option::of(1usize..4))
            .prop_map(|(query, from, backwards, limit)| Op::Read { query, from, backwards, limit }),
    ]
}

// =====================================================================
// The model
// =====================================================================

/// What the log should be, and what the store said it is.
///
/// `expected` is the model's own belief — every event handed to a successful
/// append, in order. `known` is the store's answer to a full read, adopted only
/// **after** it has been checked against `expected`. Positions live in `known`
/// because positions are the store's to assign; the model never invents one.
///
/// # Why adopting the store's answer is not circular
///
/// It looks like it should be: `select` and `violation` both compute over
/// `known`, which came from the store. What breaks the circle is that `known` is
/// only ever adopted by [`reconcile`](Self::reconcile), which first holds it to
/// three things the store does not get a vote on — it is the same events, in the
/// same order, as `expected`; its positions strictly increase; and its last one
/// is the value `append` returned. So the *content* of the log is checked
/// against a belief the store cannot influence, and only then is the *projection*
/// of that log — which filter, which direction, which truncation — checked
/// against a computation the store also cannot influence. A store that lies
/// about its log fails the first check; a store that reports its log honestly and
/// filters it wrongly fails the second.
#[derive(Debug, Default)]
struct Model {
    /// Every event the model believes was accepted, in append order.
    expected: Vec<Event>,
    /// The store's own view, validated and adopted after each operation.
    known: Vec<SequencedEvent>,
}

impl Model {
    /// Turns an [`Anchor`] into a position, against what the store has assigned.
    ///
    /// Every branch is total, and none of them can produce a false failure: the
    /// resolved value is handed to the model and to the store, so a degenerate
    /// resolution — an empty log, an overflowing head — is merely a less
    /// interesting input rather than a disagreement.
    fn resolve(&self, anchor: Anchor) -> Option<SequencePosition> {
        let positions: Vec<SequencePosition> =
            self.known.iter().map(|event| event.position).collect();

        match anchor {
            Anchor::Unset => None,
            Anchor::First => positions.first().copied(),
            Anchor::Middle => positions.get(positions.len() / 2).copied(),
            Anchor::Head => positions.last().copied(),
            // On an empty store every position is unassigned, so `FIRST` is a
            // truthful answer to "one the store has not handed out" — and it is
            // the one that exercises a condition boundary against an empty log,
            // which ES-28 cares about. `next()` saturating is unreachable short
            // of a store that has assigned `u64::MAX`.
            Anchor::BeyondHead => Some(
                positions
                    .last()
                    .and_then(|head| head.next())
                    .unwrap_or(SequencePosition::FIRST),
            ),
        }
    }

    /// What a read of `query` under `options` should return.
    ///
    /// The three steps are separate for the reason `correct.rs` separates them:
    /// `LIMIT` applied before the filter is a real adapter bug with no in-memory
    /// analogue unless the in-memory version is written in the same order.
    fn select(&self, query: &Query, options: ReadOptions) -> Vec<SequencedEvent> {
        let matched = self
            .known
            .iter()
            .filter(|event| query.matches(event.event_type(), event.tags()));

        // `from` and `to` are both inclusive in both directions, and each bounds
        // opposite ends depending on direction: forwards `from` is a floor and
        // `to` a ceiling, backwards they swap. The model carries `to` because a
        // model that ignores an option cannot disagree with a store that
        // mishandles it — it would agree with every implementation, which is the
        // one thing a reference model must not do.
        let mut selected: Vec<SequencedEvent> = if options.backwards {
            matched
                .rev()
                .filter(|event| options.from.is_none_or(|from| event.position <= from))
                .filter(|event| options.to.is_none_or(|to| event.position >= to))
                .cloned()
                .collect()
        } else {
            matched
                .filter(|event| options.from.is_none_or(|from| event.position >= from))
                .filter(|event| options.to.is_none_or(|to| event.position <= to))
                .cloned()
                .collect()
        };

        if let Some(limit) = options.limit {
            selected.truncate(limit);
        }
        selected
    }

    /// The position of the first stored event violating `condition`, if any.
    ///
    /// This is the prediction, and it is made **before** the store is called.
    /// A check made only afterwards — "the store rejected it, does the store
    /// agree it should have?" — cannot catch a store that rejects everything.
    ///
    /// # Why the *value* is discarded on a rejection
    ///
    /// [`apply`](Self::apply)'s rejection arm binds this position to `_` and
    /// never compares it against the `ConditionViolated` the store reported.
    /// That looks like coverage left on the table and it is a decision, so it is
    /// written down here rather than inferred from an underscore.
    ///
    /// `ConditionViolated::conflicting_position` is **informational** by
    /// specification (ES-25): an adapter that detects the conflict without
    /// learning which event caused it — a conditional `INSERT … WHERE NOT
    /// EXISTS`, which is the natural SQL shape — MUST be permitted to report
    /// `None`, and callers MUST NOT depend on the value. Asserting equality
    /// would therefore fail a conformant adapter, which is a worse defect than
    /// the one it would catch.
    ///
    /// The weaker check — *if* a position is reported, it is one of the
    /// violating set — was considered and not written either. It is not what the
    /// clause says, and equality with this function's answer specifically is
    /// wrong in a second way: this takes the **first** violating event in
    /// ascending order, and an adapter whose probe scans backwards or takes
    /// `max(position)` over the matching set names a different real culprit.
    /// Pinning that would freeze a position policy the specification leaves
    /// free, which is CF-6's mistake in a new place. If ES-25 is ever
    /// strengthened to require the reported position be a matching one, this is
    /// where the check goes and the value is already computed.
    fn violation(&self, condition: &AppendCondition) -> Option<SequencePosition> {
        self.known
            .iter()
            .find(|event| {
                condition.is_violated_by(event.position, event.event_type(), event.tags())
            })
            .map(|event| event.position)
    }

    /// Reads the whole log back and checks it against the model.
    ///
    /// `appended` is the position the store just returned from a successful
    /// append, or `None` when the last operation was not one.
    async fn reconcile<S: EventStore>(
        &mut self,
        store: &S,
        appended: Option<SequencePosition>,
    ) -> Result<(), String> {
        let observed = match collect(store.read(&Query::all(), ReadOptions::new())).await {
            Ok(events) => events,
            Err(err) => return Err(format!("reading the whole log failed: {err:?}")),
        };

        if observed.len() != self.expected.len() {
            return Err(format!(
                "the log holds {} event(s); {} were accepted",
                observed.len(),
                self.expected.len()
            ));
        }

        for (index, (seen, wanted)) in observed.iter().zip(&self.expected).enumerate() {
            if &seen.event != wanted {
                return Err(format!(
                    "the event at index {index} is not the one accepted there: \
                     read back {:?}, appended {wanted:?}",
                    seen.event
                ));
            }
        }

        for pair in observed.windows(2) {
            if pair[1].position <= pair[0].position {
                return Err(format!(
                    "positions must be unique and strictly increasing in \
                     assignment order, but {} follows {}",
                    pair[1].position, pair[0].position
                ));
            }
        }

        if let Some(appended) = appended {
            let last = observed.last().map(|event| event.position);
            if last != Some(appended) {
                return Err(format!(
                    "`append` returned {appended}, but the last event in the log \
                     is at {last:?}"
                ));
            }
        }

        self.known = observed;
        Ok(())
    }

    /// Runs one operation and checks everything it is allowed to check.
    async fn apply<S: EventStore>(&mut self, store: &S, op: &Op) -> Result<(), String> {
        match op {
            Op::Append { events } => {
                let written = match store.append(events, None).await {
                    Ok(position) => position,
                    Err(err) => {
                        return Err(format!("an unconditional append was refused: {err:?}"));
                    }
                };
                self.expected.extend(events.iter().cloned());
                self.reconcile(store, Some(written)).await
            }

            Op::AppendConditional {
                events,
                query,
                anchor,
            } => {
                let condition =
                    AppendCondition::new(query.clone()).after_opt(self.resolve(*anchor));
                let predicted = self.violation(&condition);

                let result = store.append(events, Some(&condition)).await;

                match (predicted, result) {
                    // Rejected, and the model agrees a stored event violates it.
                    // Nothing may have changed: `reconcile` is what checks that,
                    // because `expected` was not extended.
                    (Some(_), Err(AppendError::ConditionViolated(_))) => {
                        self.reconcile(store, None).await
                    }
                    (None, Ok(written)) => {
                        self.expected.extend(events.iter().cloned());
                        self.reconcile(store, Some(written)).await
                    }
                    (Some(conflict), Ok(written)) => Err(format!(
                        "the append was accepted at {written}, but the event at \
                         {conflict} matches {condition:?}"
                    )),
                    (None, Err(AppendError::ConditionViolated(violated))) => Err(format!(
                        "the append was rejected as {violated:?}, but no stored \
                         event matches {condition:?}"
                    )),
                    (_, Err(other)) => {
                        Err(format!("a conditional append failed outright: {other:?}"))
                    }
                }
            }

            Op::Read {
                query,
                from,
                backwards,
                limit,
            } => {
                let mut options = ReadOptions::new();
                if let Some(from) = self.resolve(*from) {
                    options = options.from(from);
                }
                if *backwards {
                    options = options.backwards();
                }
                if let Some(limit) = *limit {
                    options = options.limit(limit);
                }

                let observed = match collect(store.read(query, options)).await {
                    Ok(events) => events,
                    Err(err) => return Err(format!("the read failed: {err:?}")),
                };
                let wanted = self.select(query, options);

                if observed == wanted {
                    Ok(())
                } else {
                    Err(format!(
                        "read({query:?}, {options:?}) returned {}, and the model \
                         expected {}",
                        render(&observed),
                        render(&wanted)
                    ))
                }
            }
        }
    }
}

/// A read result, as `position:type` pairs — enough to read a diff by eye
/// without printing every payload.
fn render(events: &[SequencedEvent]) -> String {
    let rendered: Vec<String> = events
        .iter()
        .map(|event| format!("{}:{}", event.position, event.event_type().as_str()))
        .collect();
    format!("[{}]", rendered.join(", "))
}

// =====================================================================
// The runner
// =====================================================================

/// The longest sequence generated. Failures shrink far below it.
const MAX_OPS: usize = 8;

/// How many times the shrinker may move before it reports the smallest
/// counterexample it has.
///
/// A bound rather than a deadline, deliberately: CF-33 forbids a conformance
/// rule reading a clock, and `proptest`'s own `max_shrink_time` is exactly that.
/// A count is reproducible on a loaded runner; a millisecond budget is not.
const MAX_SHRINK_MOVES: usize = 1024;

/// How many times a rejected generation is retried before the case is dropped.
///
/// `any_query_item` filters the one corner that is unrepresentable — an item
/// constraining neither types nor tags — so rejections are rare and local.
const MAX_GENERATION_RETRIES: usize = 8;

/// Runs one operation sequence against a fresh fixture.
///
/// Returns the first disagreement, or `None` if the store agreed throughout.
/// It returns rather than panics because the shrinker has to run it hundreds of
/// times.
async fn replay<F, O>(open: &O, ops: &[Op]) -> Option<String>
where
    F: Fixture,
    O: AsyncFn() -> F,
{
    let fixture = open().await;
    let store = fixture.connect().await;
    let mut model = Model::default();

    for (index, op) in ops.iter().enumerate() {
        if let Err(failure) = model.apply(&store, op).await {
            return Some(format!("at op {index}, {op:?}: {failure}"));
        }
    }

    None
}

/// The conformance rules of the model family.
pub mod rules {
    // As in `suite.rs`: every rule panics when the adapter is non-conformant,
    // and a `# Panics` section saying so on each of them says nothing.
    #![allow(clippy::missing_panics_doc)]

    use proptest::prelude::Strategy;
    use proptest::strategy::ValueTree;
    use proptest::test_runner::{Config, RngAlgorithm, TestRng, TestRunner};

    use super::{MAX_GENERATION_RETRIES, MAX_OPS, MAX_SHRINK_MOVES, Op, any_op, replay};
    use crate::{Fixture, RuleOutcome};

    /// A generated sequence of appends, conditional appends and reads leaves the
    /// store in the state the model predicts, and every read along the way
    /// returns exactly what the model says it should.
    ///
    /// # Why this is not a `proptest!` block
    ///
    /// `proptest!` generates a `#[test] fn` whose body is **synchronous**, so a
    /// rule written that way would have to choose a `block_on` inside the
    /// testkit — which is precisely what CF-23 forbids, one level down from the
    /// runtime attribute it forbids by name. Driving the runner by hand keeps
    /// this rule the same shape as every other one, `async fn(open) ->
    /// RuleOutcome`, so the adapter's own emitter decides how it is driven.
    ///
    /// The loop below is `TestRunner::run_one`'s, minus the parts that need a
    /// synchronous closure: generate, and on failure alternate `simplify` and
    /// `complicate` until neither moves.
    ///
    /// # Why the RNG is deterministic
    ///
    /// A conformance rule that fails one run in twenty is worse than no rule:
    /// an adapter author reruns CI until it is green, and the suite has taught
    /// them to. The seed is fixed, so a store either passes this rule or it does
    /// not, and a failure reproduces on the reviewer's machine. Coverage is
    /// bought with `PROPTEST_CASES` — the case count is `Config::default()`'s,
    /// which reads that variable — rather than with a different sample each run.
    ///
    /// Failure persistence is never engaged, because `TestRunner::run` is what
    /// consults it and this rule does not call it. An adapter running the suite
    /// gets no `.proptest-regressions` file appearing in its tree.
    pub async fn ops_agree_with_the_model<F: Fixture>(open: impl AsyncFn() -> F) -> RuleOutcome {
        let strategy = proptest::collection::vec(any_op(), 1..=MAX_OPS);
        let config = Config::default();
        let cases = config.cases;
        let mut runner =
            TestRunner::new_with_rng(config, TestRng::deterministic_rng(RngAlgorithm::ChaCha));

        for _ in 0..cases {
            let Some(mut tree) = new_tree(&strategy, &mut runner) else {
                continue;
            };

            let Some(failure) = replay(&open, &tree.current()).await else {
                continue;
            };

            let (ops, failure) = shrink(&open, &mut tree, failure).await;
            panic!(
                "the store disagreed with the model.\n\
                 \n\
                 smallest failing sequence ({} op(s)):\n{}\n\
                 \n\
                 {failure}",
                ops.len(),
                numbered(&ops),
            );
        }

        RuleOutcome::Ran
    }

    /// Generates one value tree, retrying the filter rejections
    /// `any_query_item` can produce.
    fn new_tree<S: Strategy>(strategy: &S, runner: &mut TestRunner) -> Option<S::Tree> {
        (0..MAX_GENERATION_RETRIES).find_map(|_| strategy.new_tree(runner).ok())
    }

    /// Minimises a failing sequence, returning the smallest one still failing
    /// and the failure it produced.
    async fn shrink<F, O, T>(open: &O, tree: &mut T, failure: String) -> (Vec<Op>, String)
    where
        F: Fixture,
        O: AsyncFn() -> F,
        T: ValueTree<Value = Vec<Op>>,
    {
        let mut smallest = (tree.current(), failure);
        let mut failing = true;

        for _ in 0..MAX_SHRINK_MOVES {
            // Still failing: try something smaller. Passing: the last step went
            // too far, so back off. When neither moves, this is the floor.
            let moved = if failing {
                tree.simplify()
            } else {
                tree.complicate()
            };
            if !moved {
                break;
            }

            let ops = tree.current();
            match replay(open, &ops).await {
                Some(failure) => {
                    smallest = (ops, failure);
                    failing = true;
                }
                None => failing = false,
            }
        }

        smallest
    }

    /// The op sequence, one per line, for a panic message a human reads once.
    fn numbered(ops: &[Op]) -> String {
        ops.iter()
            .enumerate()
            .map(|(index, op)| format!("  {index}. {op:?}"))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

// =====================================================================
// The enumeration, the emitters, and the macro
// =====================================================================

/// Hands the complete model rule set to `$callback`.
///
/// The model family's single enumeration (CF-22), and the reason it is here
/// rather than in `registry.rs` is the same reason the rules are here rather
/// than in `suite.rs`: `cargo xtask spec-trace` reads `suite.rs` and would
/// report a rule written there as needing a clause of its own.
///
/// There is one rule in it today. The macro exists anyway, because the property
/// CF-22 asks for is *one list per family* whether the family has one rule or
/// ten — and because the second model rule is otherwise landed by copying a name
/// into however many harnesses exist by then.
///
/// # Examples
///
/// This example is only compiled when the `proptest` feature is on, because the
/// module defining the macro is — which is the same reason it needs no `cfg` of
/// its own.
///
/// ```
/// macro_rules! rule_names {
///     ($($name:ident),* $(,)?) => { [ $( stringify!($name) ),* ] };
/// }
///
/// let names = happenstance_testkit::for_each_model_rule!(rule_names);
/// assert!(names.contains(&"ops_agree_with_the_model"));
/// ```
#[macro_export]
macro_rules! for_each_model_rule {
    // Raw token trees rather than `$cb:path`, for the reason
    // `for_each_event_store_rule!` records: a parsed `path` fragment cannot sit
    // in callee position inside an expression, which would forbid the
    // `let names = …` form the example above depends on.
    ($($callback:tt)+) => {
        $($callback)+! {
            ops_agree_with_the_model,
        }
    };
}

/// Emits one `#[tokio::test]` per model rule.
///
/// `__emit_tokio`'s twin, differing only in the module the rules are looked up
/// in. See this module's documentation for why that is duplicated rather than
/// parameterised.
#[doc(hidden)]
#[macro_export]
macro_rules! __emit_model_tokio {
    ($($name:ident),* $(,)?) => {
        $(
            #[tokio::test]
            async fn $name() {
                $crate::model::rules::$name(__conformance_fixture)
                    .await
                    .report(::core::stringify!($name));
            }
        )*
    };
}

/// Emits one plain `#[test]` per model rule, driven by
/// [`block_on`](crate::block_on).
#[doc(hidden)]
#[macro_export]
macro_rules! __emit_model_blocking {
    ($($name:ident),* $(,)?) => {
        $(
            #[test]
            fn $name() {
                $crate::block_on($crate::model::rules::$name(__conformance_fixture))
                    .report(::core::stringify!($name));
            }
        )*
    };
}

/// Generates the model-based conformance suite for an event store adapter.
///
/// Takes the same fixture expression [`event_store_conformance!`] does, and
/// emits the same shape of test — one per rule of the model family, through an
/// emitter the caller may replace.
///
/// It exists behind the testkit's `proptest` feature and does not exist on
/// `wasm32-unknown-unknown`. Gate the invocation accordingly.
///
/// # Examples
///
/// ```
/// # macro_rules! ignore { ($($t:tt)*) => {} }
/// # ignore! {
/// use happenstance_testkit::fixtures::MemoryFixture;
///
/// happenstance_testkit::event_store_model_conformance!(MemoryFixture::new());
/// # }
/// ```
///
/// [`event_store_conformance!`]: crate::event_store_conformance
#[macro_export]
macro_rules! event_store_model_conformance {
    (mod_name = $mod_name:ident, emit = $emit:path, fixture = $fixture:expr) => {
        mod $mod_name {
            #![allow(clippy::unwrap_used, unused_imports)]

            use super::*;

            // Named exactly as `event_store_conformance!`'s is, so that a
            // caller-supplied emitter — CF-23's extension point — drives either
            // family without knowing which one it was handed.
            async fn __conformance_fixture() -> impl $crate::__private::Fixture {
                $fixture
            }

            $crate::for_each_model_rule!($emit);
        }
    };
    (mod_name = $mod_name:ident, fixture = $fixture:expr) => {
        $crate::event_store_model_conformance!(
            mod_name = $mod_name,
            emit = $crate::__emit_model_tokio,
            fixture = $fixture
        );
    };
    ($fixture:expr) => {
        $crate::event_store_model_conformance!(
            mod_name = dcb_model_conformance,
            emit = $crate::__emit_model_tokio,
            fixture = $fixture
        );
    };
}
