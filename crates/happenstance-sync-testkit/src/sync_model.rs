//! The model family: seeded peer topologies, driven through push and pull from
//! one seed, and checked at quiescence against a model of their own.
//!
//! # What this adds that the worked examples cannot
//!
//! Every rule in [`rules`](crate::rules) is a worked example against one peer.
//! The clauses this family checks — every peer ends up holding the union
//! (SY-4, SY-24), holds it once (SY-11), and forwards in arrival order (SY-25) —
//! are statements about a *set* of peers under a runner, and the interesting
//! failures are interleavings: a lost ack, a page delivered twice, a page that
//! lands after a later one, a partition that heals. This family generates
//! those. Each case is a topology of three to six peers — A—B—C chains
//! included, where the ends never meet — and up to thirty-two steps of writes,
//! exchanges, partitions and heals, each exchange carrying a fault. After the
//! last step every link heals and the harness sweeps until nothing moves; then
//! each peer's whole log is pulled and judged.
//!
//! # How it holds several peers, and SY-8
//!
//! The family opens **one fixture per node**, so every node is an isolated
//! peer, and plays the runner itself: per edge, a resume token each way and at
//! most one page in flight. Each peer is still driven only through its own
//! port, one relationship at a time, which is what SY-8 asks of the port. What
//! SY-8's `Rule:` line says of the whole testkit — every rule takes exactly
//! one peer handle — is true of [`rules`](crate::rules) and not of this family,
//! and SY-24's own `Rule:` line already names a three-peer test. The tension is
//! recorded in `.kb/open-questions/sy-8-rule-line-versus-multi-peer-rules.md`
//! rather than settled here.
//!
//! # Deterministic, and a failure prints its seed
//!
//! A master RNG with a fixed seed draws one `u64` per case, and that seed alone
//! builds the case. So a peer either passes or it does not, the same on every
//! machine, and a failure names its seed in hex beside the smallest scenario it
//! shrank to. The case count is `proptest`'s `Config::default()`'s, so
//! `PROPTEST_CASES` buys more coverage. Before any generated case, three
//! scenarios pinned by value run first: a seed's meaning moves whenever the
//! generator does, and these must not.
//!
//! # Bounded
//!
//! No clock (CF-33). Every loop is counted: steps, sweeps, pulls and shrink
//! moves. A peer that never settles fails the rule rather than hanging it
//! (H-13).
//!
//! # Not on wasm32
//!
//! `proptest` does not build there, so the module, its macros and its
//! dependency are absent on that target whatever the feature says.

mod oracle;
mod regressions;
mod scenario;
mod simulate;

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use proptest::prelude::{RngCore, Strategy};
use proptest::strategy::ValueTree;
use proptest::test_runner::{Config, RngAlgorithm, TestRng, TestRunner};

use happenstance_testkit::RuleOutcome;

use crate::{SyncFixture, Violation};
use oracle::{Check, Failure, judge};
use regressions::regressions;
use scenario::{Scenario, any_scenario, seed_bytes};
use simulate::simulate;

/// How many times the shrinker may move before it reports the smallest
/// counterexample it has. A count rather than `proptest`'s
/// `max_shrink_time`, because CF-33 forbids a rule reading a clock.
const MAX_SHRINK_MOVES: usize = 1024;

/// The model rules. See the [module documentation](crate::sync_model) for
/// what the family does, and each rule for what it rejects.
///
/// Each rule takes `impl AsyncFn() -> F`, as the worked-example rules do, and
/// calls it once per node of every scenario: each call must be a fresh,
/// isolated peer.
pub mod rules {
    use happenstance_testkit::RuleOutcome;

    use super::{Check, drive};
    use crate::{SyncFixture, Violation};

    /// SY-24 and SY-4: after every link heals and the peers settle, every
    /// peer holds every event written anywhere, byte for byte — including
    /// peers that never met the event's origin.
    ///
    /// A missing event is classified by what the model saw: never offered to
    /// that peer is SY-24 (not forwarded); offered and acknowledged as
    /// appended is SY-4 (quarantined); offered and acknowledged as already
    /// held when it never was is SY-1.
    ///
    /// # Rejects
    ///
    /// A peer that holds guarded events back for review and acknowledges them
    /// as appended: the quarantine design SY-4 names. `tests/wrong_topologies.rs`
    /// carries it, and only a generated case can catch it, so it is also the
    /// end-to-end check that a failure prints its seed. And a peer that dedupes
    /// by the highest position seen per origin, which drops an event that
    /// arrives after a later one from the same origin; that is caught by the
    /// pinned regression `delayed_page_lands_after_a_later_one`. And a peer
    /// whose caught-up resume token points one group past the end of its log,
    /// so the next group it takes in is never forwarded along a lane that had
    /// caught up: SY-24 itself.
    ///
    /// # Errors
    ///
    /// A [`Violation`] of SY-4, SY-1 or SY-24 for a peer missing an event, and
    /// of SY-24 for a peer that refuses a push or a pull or never settles.
    pub async fn transitive_convergence_over_a_partial_mesh<F: SyncFixture>(
        open: impl AsyncFn() -> F,
    ) -> Result<RuleOutcome, Violation> {
        drive(&open, Check::Union).await
    }

    /// SY-11: re-delivering an event a peer already holds is a no-op. Every
    /// push is acknowledged with exactly as many appended events as the peer
    /// had never been offered, and exactly as many skipped as it had; and no
    /// peer holds any event twice.
    ///
    /// # Rejects
    ///
    /// A peer that dedupes per batch rather than per event: a page half of
    /// which it already holds (SF-03) is applied whole, and the half it held is
    /// appended a second time. The pinned regression `sf03_partial_overlap`
    /// builds that page. And the position-prefix deduper above, whose
    /// acknowledgement calls a never-held event skipped.
    /// `tests/wrong_topologies.rs` carries both.
    ///
    /// # Errors
    ///
    /// A [`Violation`] of SY-11 for a miscounted acknowledgement or a doubled
    /// event, or for a peer that refuses a push or a pull or never settles.
    pub async fn redelivery_of_an_accepted_group_is_a_no_op<F: SyncFixture>(
        open: impl AsyncFn() -> F,
    ) -> Result<RuleOutcome, Violation> {
        drive(&open, Check::ExactlyOnce).await
    }

    /// SY-25: a peer hands back what it holds in the order it arrived. The
    /// first occurrence of each event in a peer's log follows the order the
    /// model saw it first offered.
    ///
    /// # Rejects
    ///
    /// A peer that re-sorts what it forwards by `EventId`, which looks tidy
    /// and makes a downstream scalar cursor skip an old event that arrived
    /// late. `tests/wrong_topologies.rs` carries it.
    ///
    /// # Errors
    ///
    /// A [`Violation`] of SY-25 for a peer whose log is out of arrival order,
    /// or that refuses a push or a pull or never settles.
    pub async fn forwarding_order_is_arrival_order<F: SyncFixture>(
        open: impl AsyncFn() -> F,
    ) -> Result<RuleOutcome, Violation> {
        drive(&open, Check::ArrivalOrder).await
    }
}

/// Every regression, then every generated case, judged by `check`.
async fn drive<F, O>(open: &O, check: Check) -> Result<RuleOutcome, Violation>
where
    F: SyncFixture,
    O: AsyncFn() -> F,
{
    for regression in regressions().map_err(|message| Violation::new(check.clause(), message))? {
        if let Some(failure) = replay(open, &regression.scenario, check).await {
            return Err(Violation::new(
                failure.clause,
                format!(
                    "regression scenario `{}`: {}",
                    regression.name, failure.message
                ),
            ));
        }
    }

    // One config, read once, so `PROPTEST_CASES` is consulted once. Each runner
    // stores its own copy, which is why it is cloned rather than borrowed.
    let config = Config::default();
    let cases = config.cases;
    let mut master = TestRunner::new_with_rng(
        config.clone(),
        TestRng::deterministic_rng(RngAlgorithm::ChaCha),
    );
    let strategy = any_scenario();
    for case in 0..cases {
        let seed = master.rng().next_u64();
        let mut runner = TestRunner::new_with_rng(
            config.clone(),
            TestRng::from_seed(RngAlgorithm::ChaCha, &seed_bytes(seed)),
        );
        let mut tree = strategy.new_tree(&mut runner).map_err(|reason| {
            Violation::new(
                check.clause(),
                format!("the generator rejected seed {seed:#018x}, and it has no filter: {reason}"),
            )
        })?;
        let Some(failure) = replay(open, &tree.current(), check).await else {
            continue;
        };
        let (smallest, failure) = shrink(open, &mut tree, check, failure).await;
        return Err(Violation::new(
            failure.clause,
            describe(seed, case, cases, &smallest, &failure.message),
        ));
    }
    Ok(RuleOutcome::Ran)
}

/// Runs one scenario and judges it. `None` if it passed.
async fn replay<F, O>(open: &O, scenario: &Scenario, check: Check) -> Option<Failure>
where
    F: SyncFixture,
    O: AsyncFn() -> F,
{
    match simulate(open, scenario).await {
        Ok(trace) => judge(check, &trace).err(),
        Err(message) => Some(Failure::new(check.clause(), message)),
    }
}

/// Minimises a failing scenario: `happenstance-testkit`'s model loop, which is
/// `TestRunner::run_one`'s minus the synchronous closure.
async fn shrink<F, O, T>(
    open: &O,
    tree: &mut T,
    check: Check,
    failure: Failure,
) -> (Scenario, Failure)
where
    F: SyncFixture,
    O: AsyncFn() -> F,
    T: ValueTree<Value = Scenario>,
{
    let mut smallest = (tree.current(), failure);
    let mut failing = true;
    for _ in 0..MAX_SHRINK_MOVES {
        let moved = if failing {
            tree.simplify()
        } else {
            tree.complicate()
        };
        if !moved {
            break;
        }
        let scenario = tree.current();
        match replay(open, &scenario, check).await {
            Some(failure) => {
                smallest = (scenario, failure);
                failing = true;
            }
            None => failing = false,
        }
    }
    smallest
}

/// The failure message: the seed in hex, the case, the smallest scenario, and
/// what the peers did.
///
/// `case` is zero-based and printed from one. `case < cases`, so the
/// saturation never bites; it is there so the addition cannot overflow.
fn describe(seed: u64, case: u32, cases: u32, scenario: &Scenario, message: &str) -> String {
    let case = case.saturating_add(1);
    let topology = scenario.topology();
    let steps: Vec<String> = scenario
        .steps()
        .iter()
        .enumerate()
        .map(|(index, step)| format!("  {index}. {step:?}"))
        .collect();
    format!(
        "seed {seed:#018x} (case {case} of {cases}); smallest failing scenario ({} steps, {} \
         peers, edges {:?}):\n{}\n{message}",
        steps.len(),
        topology.peers(),
        topology.edges(),
        steps.join("\n"),
    )
}

/// Hands the complete model rule set to `$callback`.
///
/// This family's single enumeration (CF-22), beside the rules it names.
///
/// # Examples
///
/// Compiled only with the `proptest` feature, because the module defining the
/// macro is.
///
/// ```
/// macro_rules! rule_names {
///     ($($name:ident),* $(,)?) => { [ $( stringify!($name) ),* ] };
/// }
///
/// let names = happenstance_sync_testkit::for_each_sync_model_rule!(rule_names);
/// assert!(names.contains(&"forwarding_order_is_arrival_order"));
/// ```
#[macro_export]
macro_rules! for_each_sync_model_rule {
    ($($callback:tt)+) => {
        $($callback)+! {
            // --- Every peer holds the union (SY-24, SY-4) --------------------
            transitive_convergence_over_a_partial_mesh,

            // --- Redelivery is a no-op (SY-11) ------------------------------
            redelivery_of_an_accepted_group_is_a_no_op,

            // --- Forwarding order is arrival order (SY-25) ------------------
            forwarding_order_is_arrival_order,
        }
    };
}

/// Emits one `#[tokio::test]` per model rule.
///
/// The caller's crate needs `tokio` with `macros` and `rt` in its
/// `dev-dependencies`. The default emitter of
/// [`sync_model_conformance!`](crate::sync_model_conformance).
#[macro_export]
macro_rules! emit_sync_model_tokio {
    ($($name:ident),* $(,)?) => {
        $(
            #[tokio::test]
            async fn $name() -> ::core::result::Result<(), $crate::Violation> {
                $crate::__private::model_rules::$name(__conformance_fixture)
                    .await?
                    .report(::core::stringify!($name));
                Ok(())
            }
        )*
    };
}

/// Emits one plain `#[test]` per model rule, driven by
/// [`happenstance_testkit::block_on`]: no runtime and no `Send` bound.
#[macro_export]
macro_rules! emit_sync_model_blocking {
    ($($name:ident),* $(,)?) => {
        $(
            #[test]
            fn $name() -> ::core::result::Result<(), $crate::Violation> {
                $crate::__private::block_on($crate::__private::model_rules::$name(__conformance_fixture))?
                    .report(::core::stringify!($name));
                Ok(())
            }
        )*
    };
}

/// Generates the model family for a peer adapter.
///
/// Takes the same fixture expression
/// [`sync_peer_conformance!`](crate::sync_peer_conformance) does. Each rule
/// calls it once per node, so it must build a fresh, isolated peer every time.
///
/// It exists behind this crate's `proptest` feature and not on
/// `wasm32-unknown-unknown`; gate the invocation accordingly.
///
/// # Examples
///
/// ```
/// # macro_rules! ignore { ($($t:tt)*) => {} }
/// # ignore! {
/// happenstance_sync_testkit::sync_model_conformance!(
///     happenstance_sync_testkit::MemorySyncFixture::new()
/// );
/// # }
/// ```
#[macro_export]
macro_rules! sync_model_conformance {
    (mod_name = $mod_name:ident, emit = $emit:path, fixture = $fixture:expr) => {
        mod $mod_name {
            async fn __conformance_fixture() -> impl $crate::__private::SyncFixture {
                $fixture
            }

            $crate::for_each_sync_model_rule!($emit);
        }
    };
    (mod_name = $mod_name:ident, fixture = $fixture:expr) => {
        $crate::sync_model_conformance!(
            mod_name = $mod_name,
            emit = $crate::emit_sync_model_tokio,
            fixture = $fixture
        );
    };
    ($fixture:expr) => {
        $crate::sync_model_conformance!(
            mod_name = sync_model_conformance,
            emit = $crate::emit_sync_model_tokio,
            fixture = $fixture
        );
    };
}

#[cfg(test)]
mod tests {
    use alloc::vec::Vec;

    use super::describe;
    use super::regressions::regressions;

    /// Every `pub async fn` in this file's `rules` module.
    fn declared_rules() -> Vec<&'static str> {
        const SOURCE: &str = include_str!("sync_model.rs");
        const PREFIX: &str = "    pub async fn ";
        SOURCE
            .lines()
            .filter_map(|line| line.strip_prefix(PREFIX))
            .filter_map(|rest| {
                rest.split(|c: char| !c.is_alphanumeric() && c != '_')
                    .next()
            })
            .collect()
    }

    /// No rule exists without appearing in the enumeration, and none is
    /// enumerated without existing.
    #[test]
    fn no_orphan_sync_model_rules() {
        let registered = crate::for_each_sync_model_rule!(crate::__sync_rule_names);
        let mut declared = declared_rules();
        assert!(!declared.is_empty(), "the source scan matched no rule");
        let mut listed = registered.to_vec();
        declared.sort_unstable();
        listed.sort_unstable();
        assert_eq!(declared, listed, "declared against enumerated");
    }

    #[test]
    fn describe_names_the_seed_in_hex() {
        let all = regressions().expect("the regressions are valid");
        let scenario = &all.first().expect("there is a regression").scenario;
        let text = describe(0xdead_beef, 7, 256, scenario, "what the peers did");
        assert!(
            text.starts_with("seed 0x00000000deadbeef (case 8 of 256)"),
            "{text}"
        );
        assert!(text.contains("3 peers"), "{text}");
        assert!(text.ends_with("what the peers did"), "{text}");
    }

    /// The case is counted from one, as "of N" reads, and the last one a
    /// `u32` can name does not overflow.
    #[test]
    fn describe_counts_cases_from_one() {
        let all = regressions().expect("the regressions are valid");
        let scenario = &all.first().expect("there is a regression").scenario;
        let first = describe(1, 0, 256, scenario, "");
        assert!(first.contains("(case 1 of 256)"), "{first}");
        let last = describe(1, u32::MAX, u32::MAX, scenario, "");
        assert!(
            last.contains(&format!("(case {0} of {0})", u32::MAX)),
            "{last}"
        );
    }
}
