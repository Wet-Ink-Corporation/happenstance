//! The rule registry: one enumeration, many harnesses.
//!
//! # Why a callback macro rather than a list
//!
//! Rust has no reflection, so "run every rule" has to be spelled out somewhere.
//! Spelling it out *twice* — once as the rule bodies, once as a list of
//! `#[tokio::test]` wrappers — is the arrangement that rots: the two agree on
//! the day they are written and nothing checks them afterwards.
//!
//! [`for_each_event_store_rule!`](crate::for_each_event_store_rule) is the one
//! place the set is enumerated. It takes the *path of a macro* and hands it the
//! whole list, so a harness is built by writing an emitter rather than by
//! copying names. Adding a rule means touching the enumeration once; every
//! harness in the workspace picks it up.
//!
//! # Why the runtime attribute is a parameter
//!
//! `#[tokio::test]`, `#[wasm_bindgen_test]` and a plain `#[test]` around a
//! hand-rolled `block_on` are three different answers to "how is an async test
//! driven", and the testkit is in no position to know which one an adapter
//! wants. Selecting between them with `cfg` inside this crate would put the
//! list of supported runtimes *here*, where a new runtime would mean a release
//! of the testkit. So the wrapper is a parameter: the emitter is supplied by
//! the caller. The three emitters below are conveniences for the three
//! harnesses this workspace demonstrates, not the closed set — a caller can
//! pass its own, as the example on `for_each_event_store_rule!` shows.
//!
//! # The contract an emitter must satisfy
//!
//! An emitter is invoked as `emitter!(rule_a, rule_b, ...)` and expands to
//! items placed inside the harness module. That module already contains
//!
//! ```text
//! async fn __conformance_fixture() -> impl happenstance_testkit::Fixture {
//!     <fixture expr>
//! }
//! ```
//!
//! so an emitter never needs to know the fixture expression — or the fixture
//! *type*, which it could not know, since the expression is all the macro was
//! given. `macro_rules!` hygiene applies to local variables, not to items, which
//! is what lets one macro's expansion define that function and another's refer
//! to it.
//!
//! The emitter passes `__conformance_fixture` itself, as an
//! `impl AsyncFn() -> F`, so the *rule* decides how many fixture instances it
//! needs. That is the change the fixture contract bought: the old arrangement
//! called the factory once per rule, in the emitter, and a rule that wanted a
//! second isolated store had nowhere to ask for one.
//!
//! An emitter must report what a rule returns. It is not asked politely:
//! [`RuleOutcome`](crate::RuleOutcome) is `#[must_use]`, and dropping it warns
//! in a workspace whose CI denies warnings.
//!
//! [`RuleOutcome::report`](crate::RuleOutcome::report) is the stdout answer and
//! is what the tokio and blocking emitters call.
//! [`RuleOutcome::skip_line`](crate::RuleOutcome::skip_line) is the same line
//! without a sink, for a harness whose target has no stdout — which is not
//! hypothetical: it is `wasm32-unknown-unknown`, and `__emit_wasm` is why the
//! method is public.

use core::future::Future;
use core::task::{Context, Poll, Waker};
use std::sync::Arc;
use std::task::Wake;
use std::thread::Thread;

/// Hands the complete event store rule set to `$callback`.
///
/// `$callback` is the path of a macro that accepts a comma-separated list of
/// identifiers. It is invoked exactly once, with every rule name in
/// [`rules`](crate::rules).
///
/// This is the *only* place the rule set is written down. Anything that needs
/// to iterate the rules — a test harness, a documentation table, the
/// `no_orphan_rules` meta-test — is built by invoking this macro.
///
/// # Examples
///
/// A caller-supplied emitter, which is how a runtime this crate has never
/// heard of gets a harness. The callback path is substituted verbatim, so a
/// bare name resolves in *your* scope:
///
/// ```
/// macro_rules! rule_names {
///     ($($name:ident),* $(,)?) => { [ $( stringify!($name) ),* ] };
/// }
///
/// let names = happenstance_testkit::for_each_event_store_rule!(rule_names);
/// assert!(names.contains(&"append_is_atomic"));
/// assert!(names.contains(&"racing_conditional_appends_elect_one_winner"));
/// ```
#[macro_export]
macro_rules! for_each_event_store_rule {
    // The callback is captured as raw token trees rather than as `$cb:path`.
    // A `path` fragment is a parsed AST node, and once parsed it can no longer
    // sit in callee position inside an expression: rustc reports "macro
    // expansion ignores `!` and any tokens following". It works in item
    // position and nowhere else, which would quietly forbid
    // `let names = for_each_event_store_rule!(...)` — exactly what the
    // meta-test below needs.
    ($($callback:tt)+) => {
        $($callback)+! {
            // --- The fixture contract --------------------------------------
            two_fixture_instances_observe_none_of_each_others_appends,
            two_handles_observe_each_others_appends,
            acknowledged_writes_survive_a_reopen,

            // --- Query semantics -------------------------------------------
            query_all_matches_every_event,
            query_item_types_are_or,
            query_item_tags_are_and,
            query_item_tags_match_supersets,
            query_item_rejects_partial_tag_overlap,
            query_item_combines_types_and_tags_with_and,
            query_items_are_or,
            untagged_events_match_query_all,
            duplicate_items_do_not_duplicate_events,
            query_item_order_does_not_change_the_result_set,
            query_matching_nothing_yields_empty,
            query_union_is_item_concatenation,

            // --- Read options ----------------------------------------------
            read_from_is_inclusive,
            read_backwards_reverses_order,
            read_limit_truncates,
            read_backwards_from_with_limit,
            read_defaults_to_ascending_order,
            reading_an_empty_store_yields_nothing,
            read_limit_applies_after_filtering,
            read_backwards_limit_applies_after_filtering,
            read_from_composes_with_multi_item_query,
            read_from_composes_with_limit,
            read_to_is_inclusive,
            read_from_and_to_bound_a_closed_window,
            read_to_under_backwards_bounds_the_older_end,
            read_to_composes_with_multi_item_query,
            read_limit_zero_yields_nothing,
            limit_applies_across_items_not_per_item,
            read_to_composes_with_limit,
            read_from_a_gap_position,

            // --- Sequence positions ----------------------------------------
            positions_are_unique,
            positions_are_strictly_monotonic,

            // --- Head -----------------------------------------------------
            head_of_an_empty_store_is_none,
            head_is_the_highest_visible_position,
            head_advances_across_two_handles,

            // --- Identity, recorded time and membership --------------------
            append_stamps_identity_and_time,
            append_stamps_a_local_event_id,
            event_ids_are_unique_within_a_store,
            appending_equal_events_yields_two_events,
            event_id_is_not_matchable_by_query,
            reopened_store_does_not_reissue_an_event_id,
            append_stamps_a_recorded_time,
            recorded_time_survives_a_reopen,
            contains_event_id_reports_membership,

            // --- Append ----------------------------------------------------
            append_returns_last_written_position,
            append_is_atomic,
            append_is_atomic_under_a_mid_batch_fault,
            arming_a_mid_batch_fault_makes_the_append_fail,
            append_rejects_empty_batch,
            empty_batch_is_refused_before_the_condition_is_evaluated,
            append_preserves_event_payload,
            batch_positions_follow_slice_order,
            batch_is_not_evaluated_against_its_own_condition,
            dropped_append_future_leaves_no_partial_batch,
            reissued_conditional_batch_lands_once,
            reissued_unconditional_batch_lands_twice,
            reissued_batch_conditioned_on_other_events_lands_twice,

            // --- Value edges -----------------------------------------------
            append_preserves_an_empty_payload,
            metadata_distinguishes_absent_from_empty,
            store_accepts_a_max_length_identifier,
            store_accepts_non_ascii_identifiers,
            store_accepts_the_guaranteed_minimum_payload,
            store_accepts_the_guaranteed_minimum_tag_count,
            store_evaluates_a_query_at_the_guaranteed_minimum_item_count,
            store_accepts_the_guaranteed_minimum_batch_size,
            tags_differing_only_by_unicode_normalisation_are_distinct,
            tags_may_repeat_a_key,
            append_preserves_event_type_and_tags_byte_for_byte,
            append_reports_exceeded_store_limits,

            // --- Append conditions -----------------------------------------
            condition_without_after_rejects_any_match,
            condition_without_after_allows_non_match,
            condition_after_ignores_events_at_the_boundary,
            condition_after_rejects_events_beyond_the_boundary,
            condition_after_ignores_non_matching_events,
            condition_matches_on_tags,
            condition_with_an_unheld_tag_does_not_reject,
            condition_against_an_empty_store_admits_the_append,
            condition_after_beyond_head_admits_the_append,
            condition_after_beyond_the_last_matching_position_admits_the_append,
            condition_rejection_leaves_store_unchanged,
            condition_rejection_is_reported_as_condition_violated,
            condition_guards_carry_independent_boundaries,
            condition_with_one_guard_behaves_as_today,

            // --- Concurrency -----------------------------------------------
            racing_conditional_appends_elect_one_winner,

            // --- Re-entrancy -----------------------------------------------
            interleaved_appends_on_one_handle_elect_one_winner,
            a_live_read_stream_does_not_block_an_append,

            // --- Read isolation ---------------------------------------------
            read_result_is_stable_under_concurrent_append,
            query_items_share_one_snapshot,

            // --- The read path's error arm ----------------------------------
            arming_a_read_fault_makes_the_stream_yield_an_error,

            // --- Position visibility ---------------------------------------
            nothing_below_an_observed_position_appears_later,
        }
    };
}

/// Emits one `#[tokio::test]` per rule.
///
/// The caller's crate needs `tokio` with `macros` and `rt` in its
/// `dev-dependencies`; the attribute resolves in the caller's scope, not here.
#[doc(hidden)]
#[macro_export]
macro_rules! __emit_tokio {
    ($($name:ident),* $(,)?) => {
        $(
            #[tokio::test]
            async fn $name() {
                $crate::__private::rules::$name(__conformance_fixture)
                    .await
                    .report(::core::stringify!($name));
            }
        )*
    };
}

/// Emits one plain `#[test]` per rule, driven by [`block_on`](crate::block_on).
///
/// No runtime, no dependency, one thread. This is the harness that proves the
/// suite never quietly needs `tokio` — and, because `block_on` imposes no
/// `Send` bound, that it works against a `!Send` adapter.
#[doc(hidden)]
#[macro_export]
macro_rules! __emit_blocking {
    ($($name:ident),* $(,)?) => {
        $(
            #[test]
            fn $name() {
                $crate::__private::block_on($crate::__private::rules::$name(__conformance_fixture))
                    .report(::core::stringify!($name));
            }
        )*
    };
}

/// Emits one `#[wasm_bindgen_test]` per rule.
///
/// The caller's crate needs `wasm-bindgen-test` in its `dev-dependencies`,
/// gated on `cfg(target_arch = "wasm32")`; as with the tokio emitter the
/// attribute resolves in the caller's scope.
///
/// This is the one emitter that does not call
/// [`RuleOutcome::report`](crate::RuleOutcome::report), and the reason is the
/// target rather than taste: `println!` writes nowhere on
/// `wasm32-unknown-unknown`, so a skip reported through it leaves no record on
/// the only target CF-20 and CF-23 exist for. `console_log!` is what the
/// runner captures, and resolving it through the caller's `wasm_bindgen_test` —
/// which the `#[wasm_bindgen_test]` attribute above already requires — keeps the
/// testkit itself free of a `wasm-bindgen` dependency.
#[doc(hidden)]
#[macro_export]
macro_rules! __emit_wasm {
    ($($name:ident),* $(,)?) => {
        $(
            #[::wasm_bindgen_test::wasm_bindgen_test]
            async fn $name() {
                let __outcome = $crate::__private::rules::$name(__conformance_fixture).await;
                if let Some(__line) = __outcome.skip_line(::core::stringify!($name)) {
                    ::wasm_bindgen_test::console_log!("{}", __line);
                }
            }
        )*
    };
}

/// Expands to a `[&str; N]` of the rule names, for meta-tests and docs.
#[doc(hidden)]
#[macro_export]
macro_rules! __emit_rule_names {
    ($($name:ident),* $(,)?) => { [ $( ::core::stringify!($name) ),* ] };
}

/// Wakes the thread parked inside [`block_on`].
///
/// `std::task::Wake` exists precisely so a waker can be built from safe code;
/// the alternative — hand-writing a `RawWakerVTable` — needs `unsafe`, which
/// this workspace forbids outright.
struct ParkWaker(Thread);

impl Wake for ParkWaker {
    fn wake(self: Arc<Self>) {
        self.0.unpark();
    }
}

/// Drives a future to completion on the current thread.
///
/// This exists so the testkit can demonstrate a harness that depends on no
/// async runtime at all, which is what keeps the conformance suite from
/// quietly acquiring a `tokio` requirement. It is deliberately not bounded on
/// `Send`: the rules are written against
/// [`EventStore`](happenstance_core::EventStore), the `!Send` flavour, and a
/// `block_on` that demanded `Send` would exclude exactly the adapters that
/// flavour exists for.
///
/// # Panics
///
/// Not itself. A failing rule panics inside the polled future and that panic
/// propagates, which is what makes the enclosing `#[test]` fail.
///
/// # Examples
///
/// ```
/// assert_eq!(happenstance_testkit::block_on(async { 1 + 1 }), 2);
/// ```
pub fn block_on<F: Future>(future: F) -> F::Output {
    let waker = Waker::from(Arc::new(ParkWaker(std::thread::current())));
    let mut context = Context::from_waker(&waker);
    let mut future = Box::pin(future);

    loop {
        match future.as_mut().poll(&mut context) {
            Poll::Ready(value) => return value,
            // Parking rather than spinning: a rule that awaits real I/O still
            // makes progress, and a rule that never wakes hangs visibly rather
            // than burning a core.
            Poll::Pending => std::thread::park(),
        }
    }
}

/// Every `pub async fn` declared at the top level of `suite.rs`'s `rules`
/// module.
///
/// rustfmt pins the shape of those declarations, which is what makes a textual
/// scan defensible here: a rule is always `    pub async fn <name>` at one
/// level of indentation inside `pub mod rules`.
#[cfg(test)]
fn declared_rules() -> Vec<&'static str> {
    const SOURCE: &str = include_str!("suite.rs");
    const PREFIX: &str = "    pub async fn ";

    SOURCE
        .lines()
        .filter_map(|line| line.strip_prefix(PREFIX))
        .map(|rest| {
            rest.split(|c: char| !c.is_alphanumeric() && c != '_')
                .next()
                .unwrap_or("")
        })
        .collect()
}

/// CF-24: no rule may exist in [`rules`](crate::rules) without appearing in
/// [`for_each_event_store_rule!`](crate::for_each_event_store_rule).
///
/// # What this checks, and what it cannot
///
/// Rust has no reflection, so there is no way to ask the compiler for "the
/// public items of `rules`" at run time. One direction is free: every emitter
/// expands to `$crate::__private::rules::$name`, so a name in the enumeration with no rule
/// behind it is `error[E0425]` in any harness. The *orphan* direction — a rule
/// with no registration — is checked here by parsing the module's own source,
/// which `include_str!` bakes into the binary, so the test needs neither a
/// filesystem nor a particular working directory.
///
/// It catches the failure CF-24 describes: a 47th `pub async fn` lands in
/// `suite.rs` and the enumeration is not updated. It does **not** catch a rule
/// introduced into `rules` by a macro expansion, by a `pub use` re-export, or
/// from a `#[path]`-included file — none of those are visible to a textual
/// scan. The scan is fail-loud rather than fail-open: were it ever to match
/// nothing, the second assertion reports every registered rule as missing.
///
/// One thing it deliberately does not match, and it is worth knowing before
/// adding another: a helper shared by two rules is spelled `    async fn`
/// without `pub` — `seed_hits_between_misses` is the first — so the prefix that
/// selects rules also excludes it. A helper made public would be reported as an
/// orphan, which is the right answer rather than a false positive.
///
/// The scan survived the fixture contract untouched, which was worth checking
/// rather than assuming: rules gained a generic parameter and a return type, but
/// the prefix it matches — `    pub async fn ` at one level of indentation
/// inside `pub mod rules` — is the part that did not move.
///
/// A strictly stronger guard exists and was compiled before this one was
/// chosen: move the bodies into a private `mod rules_impl` and generate
/// `pub mod rules` as `pub use` re-exports through a fourth emitter, at which
/// point an unregistered rule is a `pub` item in a private module and
/// `unreachable_pub` — already `warn` in the workspace lints, already
/// `-D warnings` in CI — makes it a build failure. It is not landed here
/// because it trades a legible failure message for a compiler error at a
/// distance, and because it turns `rules` from a module that reads
/// top-to-bottom into a macro-generated re-export list. Phase 3 owns the
/// suite's own proof obligation (CF-1 – CF-29) and is the right place to
/// revisit it.
#[cfg(test)]
#[test]
fn no_orphan_rules() {
    let registered = crate::for_each_event_store_rule!(crate::__emit_rule_names);
    let declared = declared_rules();

    let orphans: Vec<_> = declared
        .iter()
        .filter(|name| !registered.contains(*name))
        .collect();
    assert!(
        orphans.is_empty(),
        "these rules exist in `rules` but are absent from \
         `for_each_event_store_rule!`, so no harness runs them: {orphans:?}"
    );

    let missing: Vec<_> = registered
        .iter()
        .filter(|name| !declared.contains(*name))
        .collect();
    assert!(
        missing.is_empty(),
        "these rules are registered but were not found in `suite.rs` — either \
         they moved, or the source scan no longer matches the way they are \
         written: {missing:?}"
    );
}
