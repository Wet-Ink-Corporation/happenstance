// The README's code blocks are compiled as doctests. `cfg(doctest)` keeps the
// prose out of the rendered documentation — it would otherwise appear twice, once
// here and once in the module docs below — while still type-checking every
// example. A README example that does not compile is worse than no example: it
// is the first thing a reader tries, and the first impression the crate makes.
// (D10)
#![cfg_attr(doctest, doc = include_str!("../README.md"))]
//! The DCB conformance suite for happenstance event store adapters.
//!
//! "Storage agnostic" is a claim about behaviour, and a claim about behaviour
//! is worth exactly as much as the test that checks it. This crate is that
//! test. An adapter in this workspace is not considered to exist until it
//! invokes [`event_store_conformance!`] and passes.
//!
//! # Usage
//!
//! In your adapter's `tests/` directory or a `#[cfg(test)]` module:
//!
//! ```
//! # #[cfg(feature = "doctest-only")]
//! happenstance_testkit::event_store_conformance!(MemoryEventStore::new());
//! ```
//!
//! The macro takes an expression that builds a **fresh, empty** store and
//! expands to one `#[tokio::test]` per rule, so a failure names the rule that
//! broke rather than reporting "conformance failed". The expression is
//! re-evaluated for every test; a file-backed adapter should point it at a
//! temporary directory.
//!
//! Your crate needs `tokio` with the `macros` and `rt` features in
//! `dev-dependencies`.
//!
//! # Choosing a harness
//!
//! `#[tokio::test]` is a default, not a requirement. The per-test wrapper is a
//! parameter — an *emitter* macro — because the testkit is in no position to
//! know which runtime an adapter is tested on, and a `cfg` ladder in here would
//! mean every new runtime needs a testkit release. Three emitters ship, and all
//! three are demonstrated in this crate's `tests/`:
//!
//! | Emitter | Wrapper | Adapter needs |
//! |---|---|---|
//! | `__emit_tokio` (default) | `#[tokio::test]` | `tokio` with `macros`, `rt` |
//! | `__emit_blocking` | `#[test]` + [`block_on`] | nothing |
//! | `__emit_wasm` | `#[wasm_bindgen_test]` | `wasm-bindgen-test` |
//!
//! ```
//! # macro_rules! ignore { ($($t:tt)*) => {} }
//! # ignore! {
//! happenstance_testkit::event_store_conformance!(
//!     mod_name = dcb_conformance_blocking,
//!     emit = happenstance_testkit::__emit_blocking,
//!     factory = MyStore::new()
//! );
//! # }
//! ```
//!
//! A runtime none of those cover needs no change here: write a `macro_rules!`
//! that accepts a comma-separated list of identifiers and hand it to
//! [`for_each_event_store_rule!`] yourself.
//!
//! # Where the rule set lives
//!
//! In exactly one place: [`for_each_event_store_rule!`]. Every harness, and the
//! `no_orphan_rules` meta-test, is built by invoking it. A rule that exists in
//! [`rules`] without appearing there is a rule nothing runs, which is the
//! failure the meta-test is for.
//!
//! # What is checked
//!
//! Every rule traces to a MUST in the [specification][spec], plus the
//! properties an adapter can plausibly get wrong:
//!
//! | Area | Rules |
//! |---|---|
//! | Query semantics | types OR within an item; tags AND within an item; items OR across a query; `Query::all`; supersets match, partial overlaps do not |
//! | Read options | `from` is inclusive; `backwards` reverses order; `limit` truncates; the three compose |
//! | Positions | unique; strictly monotonic; gaps permitted |
//! | Append | atomic; a rejection changes nothing; an empty batch is refused; the returned position is the last written |
//! | Append conditions | the full matrix, including the exact `after` boundary |
//! | Concurrency | racing appends with overlapping conditions — exactly one wins |
//!
//! [spec]: https://dcb.events/specification/
//!
//! # Which flavour to test
//!
//! The macro binds on [`EventStore`](happenstance_core::EventStore), the flavour
//! with no `Send` bound, so it accepts both kinds of adapter. If your adapter
//! implements [`SendEventStore`](happenstance_core::SendEventStore) — as every
//! native one should — you get that bound checked for free, because
//! `SendEventStore` implies `EventStore`.

#![doc(html_no_source)]

pub mod fixtures;
mod registry;
mod suite;

pub use registry::block_on;
pub use suite::rules;

/// Generates the full DCB conformance suite for an event store adapter.
///
/// Takes an expression that produces a fresh, empty store. See the [crate
/// documentation](crate) for what is checked and what the adapter must provide.
///
/// The rule list is not written here; it comes from
/// [`for_each_event_store_rule!`](crate::for_each_event_store_rule), which is
/// the only place it is written at all.
///
/// # Examples
///
/// ```
/// # macro_rules! ignore { ($($t:tt)*) => {} }
/// # ignore! {
/// use happenstance_core::MemoryEventStore;
///
/// happenstance_testkit::event_store_conformance!(MemoryEventStore::new());
/// # }
/// ```
///
/// Choosing a different harness — the emitter is a parameter, so the testkit
/// never decides which async runtime an adapter is tested on:
///
/// ```
/// # macro_rules! ignore { ($($t:tt)*) => {} }
/// # ignore! {
/// happenstance_testkit::event_store_conformance!(
///     mod_name = blocking_conformance,
///     emit = happenstance_testkit::__emit_blocking,
///     factory = MemoryEventStore::new()
/// );
/// # }
/// ```
#[macro_export]
macro_rules! event_store_conformance {
    // The general form. Listed first so that arm matching never has to back out
    // of `factory = $factory:expr` to reach it.
    (mod_name = $mod_name:ident, emit = $emit:path, factory = $factory:expr) => {
        mod $mod_name {
            #![allow(clippy::unwrap_used, unused_imports)]

            use super::*;

            // The factory, hoisted behind a function so that emitters need to
            // know nothing about it. `impl Trait` keeps the adapter's concrete
            // type opaque without naming it, and re-evaluates the expression on
            // every call — a file-backed adapter still gets a fresh store per
            // rule.
            fn __conformance_store() -> impl $crate::__private::EventStore {
                $factory
            }

            // `$emit` is `$crate::`-qualified by the caller. A bare name here
            // would be substituted verbatim and resolve in the *adapter's*
            // crate, where the testkit's emitters do not exist.
            $crate::for_each_event_store_rule!($emit);
        }
    };
    (mod_name = $mod_name:ident, factory = $factory:expr) => {
        $crate::event_store_conformance!(
            mod_name = $mod_name,
            emit = $crate::__emit_tokio,
            factory = $factory
        );
    };
    ($factory:expr) => {
        $crate::event_store_conformance!(
            mod_name = dcb_conformance,
            emit = $crate::__emit_tokio,
            factory = $factory
        );
    };
}

/// Re-exports the macro expansions need to name, so an adapter is not required
/// to have `happenstance-core` in scope under that exact name.
#[doc(hidden)]
pub mod __private {
    pub use happenstance_core::EventStore;
}
