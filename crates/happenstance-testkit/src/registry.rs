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
//! fn __conformance_store() -> impl happenstance_core::EventStore { <factory expr> }
//! ```
//!
//! so an emitter never needs to know the factory expression; it names
//! `__conformance_store` and each rule calls it once per test. `macro_rules!`
//! hygiene applies to local variables, not to items, which is what lets one
//! macro's expansion define that function and another's refer to it.

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
            // --- Query semantics -------------------------------------------
            query_all_matches_every_event,
            query_item_types_are_or,
            query_item_tags_are_and,
            query_item_tags_match_supersets,
            query_item_rejects_partial_tag_overlap,
            query_item_combines_types_and_tags_with_and,
            query_items_are_or,
            query_matching_nothing_yields_empty,

            // --- Read options ----------------------------------------------
            read_from_is_inclusive,
            read_backwards_reverses_order,
            read_limit_truncates,
            read_backwards_from_with_limit,
            read_defaults_to_ascending_order,

            // --- Sequence positions ----------------------------------------
            positions_are_unique,
            positions_are_strictly_monotonic,

            // --- Append ----------------------------------------------------
            append_returns_last_written_position,
            append_is_atomic,
            append_rejects_empty_batch,
            append_preserves_event_payload,

            // --- Append conditions -----------------------------------------
            condition_without_after_rejects_any_match,
            condition_without_after_allows_non_match,
            condition_after_ignores_events_at_the_boundary,
            condition_after_rejects_events_beyond_the_boundary,
            condition_after_ignores_non_matching_events,
            condition_rejection_leaves_store_unchanged,
            condition_rejection_is_reported_as_condition_violated,

            // --- Concurrency -----------------------------------------------
            racing_conditional_appends_elect_one_winner,
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
                $crate::rules::$name(__conformance_store).await;
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
                $crate::block_on($crate::rules::$name(__conformance_store));
            }
        )*
    };
}

/// Emits one `#[wasm_bindgen_test]` per rule.
///
/// The caller's crate needs `wasm-bindgen-test` in its `dev-dependencies`,
/// gated on `cfg(target_arch = "wasm32")`; as with the tokio emitter the
/// attribute resolves in the caller's scope.
#[doc(hidden)]
#[macro_export]
macro_rules! __emit_wasm {
    ($($name:ident),* $(,)?) => {
        $(
            #[::wasm_bindgen_test::wasm_bindgen_test]
            async fn $name() {
                $crate::rules::$name(__conformance_store).await;
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
/// expands to `$crate::rules::$name`, so a name in the enumeration with no rule
/// behind it is `error[E0425]` in any harness. The *orphan* direction — a rule
/// with no registration — is checked here by parsing the module's own source,
/// which `include_str!` bakes into the binary, so the test needs neither a
/// filesystem nor a particular working directory.
///
/// It catches the failure CF-24 describes: a 28th `pub async fn` lands in
/// `suite.rs` and the enumeration is not updated. It does **not** catch a rule
/// introduced into `rules` by a macro expansion, by a `pub use` re-export, or
/// from a `#[path]`-included file — none of those are visible to a textual
/// scan. The scan is fail-loud rather than fail-open: were it ever to match
/// nothing, the second assertion reports all 27 registered rules as missing.
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
