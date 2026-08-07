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
mod suite;

pub use suite::rules;

/// Generates the full DCB conformance suite for an event store adapter.
///
/// Takes an expression that produces a fresh, empty store. See the [crate
/// documentation](crate) for what is checked and what the adapter must provide.
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
#[macro_export]
macro_rules! event_store_conformance {
    ($factory:expr) => {
        $crate::event_store_conformance!(mod_name = dcb_conformance, factory = $factory);
    };
    (mod_name = $mod_name:ident, factory = $factory:expr) => {
        mod $mod_name {
            #![allow(clippy::unwrap_used, unused_imports)]

            use super::*;

            macro_rules! conformance_test {
                ($name:ident) => {
                    #[tokio::test]
                    async fn $name() {
                        $crate::rules::$name(|| $factory).await;
                    }
                };
            }

            // --- Query semantics -------------------------------------------
            conformance_test!(query_all_matches_every_event);
            conformance_test!(query_item_types_are_or);
            conformance_test!(query_item_tags_are_and);
            conformance_test!(query_item_tags_match_supersets);
            conformance_test!(query_item_rejects_partial_tag_overlap);
            conformance_test!(query_item_combines_types_and_tags_with_and);
            conformance_test!(query_items_are_or);
            conformance_test!(query_matching_nothing_yields_empty);

            // --- Read options ----------------------------------------------
            conformance_test!(read_from_is_inclusive);
            conformance_test!(read_backwards_reverses_order);
            conformance_test!(read_limit_truncates);
            conformance_test!(read_backwards_from_with_limit);
            conformance_test!(read_defaults_to_ascending_order);

            // --- Sequence positions -----------------------------------------
            conformance_test!(positions_are_unique);
            conformance_test!(positions_are_strictly_monotonic);

            // --- Append -----------------------------------------------------
            conformance_test!(append_returns_last_written_position);
            conformance_test!(append_is_atomic);
            conformance_test!(append_rejects_empty_batch);
            conformance_test!(append_preserves_event_payload);

            // --- Append conditions -------------------------------------------
            conformance_test!(condition_without_after_rejects_any_match);
            conformance_test!(condition_without_after_allows_non_match);
            conformance_test!(condition_after_ignores_events_at_the_boundary);
            conformance_test!(condition_after_rejects_events_beyond_the_boundary);
            conformance_test!(condition_after_ignores_non_matching_events);
            conformance_test!(condition_rejection_leaves_store_unchanged);
            conformance_test!(condition_rejection_is_reported_as_condition_violated);

            // --- Concurrency --------------------------------------------------
            conformance_test!(racing_conditional_appends_elect_one_winner);
        }
    };
}
