//! The dispatch emitter: one rule, chosen by name, run against fresh fixtures.
//!
//! CF-23 makes the emitter the adapter's to supply, and the testkit's three —
//! `emit_tokio`, `emit_blocking`, `emit_wasm` — each wrap every rule in a test
//! attribute some runner collects. Inside `workerd` the collector is vitest,
//! running in JavaScript, so what it needs from Rust is not a test function per
//! rule but **a function from a rule's name to its outcome**. `emit_dispatch`
//! is that: the same enumeration, expanded into a `match`.
//!
//! # The guard against a bespoke emitter dropping rules
//!
//! An emitter that quietly omits a rule satisfies every check that reads the
//! suite's source. This one cannot do it unseen, because the names the runner
//! iterates come from [`rule_names`], which expands the enumeration through the
//! testkit's own `__rule_names` callback rather than through this emitter. A
//! rule missing from the `match` is a rule [`run`] answers with
//! [`Dispatched::Unknown`], and the runner fails it by name.

use core::cell::Cell;

use happenstance_cloudflare::TableNamespace;
use happenstance_testkit::RuleOutcome;
use worker::State;

use crate::fixture::{HarnessFaults, WorkerdFixture};

/// CF-18's machine-checked half, which `event_store_conformance!` emits around
/// every harness and which this harness therefore runs as a rule of its own.
pub const DECLENSIONS: &str = "every_declined_capability_is_stated_by_this_fixture";

/// How many isolated fixture instances one rule may open. The suite's widest
/// rule opens two; the pool is larger so a new rule that opens three is not a
/// harness fault on the day it lands.
const INSTANCES: usize = 8;

/// Every name the runner must run, from the suite's own enumeration.
#[must_use]
pub fn rule_names() -> Vec<&'static str> {
    let mut names =
        happenstance_testkit::for_each_event_store_rule!(happenstance_testkit::__rule_names)
            .to_vec();
    names.push(DECLENSIONS);
    names
}

/// What running one rule by name came to.
#[derive(Debug, PartialEq, Eq)]
pub enum Dispatched {
    /// The rule ran and every assertion held.
    Ran,
    /// The rule needs a capability this fixture declines; the line says which
    /// and why.
    Skipped(String),
    /// No rule has this name — the signal that the dispatch table and the
    /// enumeration disagree.
    Unknown,
    /// The rule returned, but the harness failed underneath it, so its result
    /// says nothing about the adapter.
    HarnessFault(Vec<String>),
}

/// Runs the rule called `rule` inside the object whose state is `state`.
///
/// A failing assertion panics inside the rule, as it does under every emitter;
/// in `workerd` that rejects the request, which is how the runner sees it.
pub async fn run(state: &State, rule: &str) -> Dispatched {
    let faults = HarnessFaults::default();
    let pool: Result<Vec<TableNamespace>, _> = (0..INSTANCES)
        .map(|n| TableNamespace::new(&format!("f{n}")))
        .collect();
    let pool = match pool {
        Ok(pool) => pool,
        Err(err) => return Dispatched::HarnessFault(vec![format!("namespace pool: {err}")]),
    };
    let Some((spare, pool)) = pool.split_last() else {
        return Dispatched::HarnessFault(vec!["the namespace pool is empty".to_owned()]);
    };
    let opened = Cell::new(0_usize);
    let faults_ref = &faults;
    let open = async || {
        let index = opened.get();
        opened.set(index.saturating_add(1));
        let namespace = pool.get(index).unwrap_or_else(|| {
            let fault = format!("a rule opened more than {} fixtures", pool.len());
            // Logged now as well as recorded: a rule that shares the spare
            // namespace may then fail an isolation assertion and panic before the
            // record is read, and the log is all that would name the cause.
            worker::console_error!("{fault}");
            faults_ref.record(fault);
            spare
        });
        WorkerdFixture::open(state, namespace.clone(), faults_ref)
    };

    macro_rules! emit_dispatch {
        ($($name:ident),* $(,)?) => {
            match rule {
                $(
                    // VACUITY CONTROL, never merged: one rule is dropped from the
                    // dispatch table, so the runner must report `no such rule`.
                    ::core::stringify!($name)
                        if ::core::stringify!($name) != "query_item_types_are_or" => Some(
                        happenstance_testkit::__private::rules::$name(open).await,
                    ),
                )*
                DECLENSIONS => {
                    happenstance_testkit::__private::assert_declensions_are_stated(open);
                    Some(RuleOutcome::Ran)
                }
                _ => None,
            }
        };
    }

    let outcome = happenstance_testkit::for_each_event_store_rule!(emit_dispatch);

    let recorded = faults.take();
    match outcome {
        None => Dispatched::Unknown,
        Some(_) if !recorded.is_empty() => Dispatched::HarnessFault(recorded),
        Some(outcome) => outcome
            .skip_line(rule)
            .map_or(Dispatched::Ran, Dispatched::Skipped),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::{DECLENSIONS, rule_names};

    #[test]
    fn the_names_are_the_enumeration_plus_the_declension_check_once() {
        let names = rule_names();
        let enumerated =
            happenstance_testkit::for_each_event_store_rule!(happenstance_testkit::__rule_names);
        let distinct: BTreeSet<_> = names.iter().collect();
        assert_eq!(distinct.len(), names.len(), "no name appears twice");
        assert_eq!(names.len(), enumerated.len() + 1);
        assert_eq!(names.iter().filter(|name| **name == DECLENSIONS).count(), 1);
        assert!(enumerated.iter().all(|rule| names.contains(rule)));
    }
}
