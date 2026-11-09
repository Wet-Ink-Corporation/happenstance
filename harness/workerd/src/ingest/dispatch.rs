//! The ingest family's dispatcher: one rule, chosen by name, run against
//! fresh [`WorkerdIngestFixture`]s.
//!
//! [`crate::dispatch`] one family over, with three differences. The opener
//! builds ingest fixtures over the same namespace pool; the emitter calls the
//! testkit's **public** `ingest_rules`, which answer a [`Violation`] as a value
//! rather than panicking, so a red rule is [`Dispatched::Violated`] and never a
//! trap; and the declension check is the ingest family's own.
//!
//! The write-group family (#420) runs through the same opener, because its
//! rules take the same `IngestFixture`: [`crate::write_group`] lists its names
//! and calls this module's crate-private `run_family` with
//! `Family::WriteGroup`. So does the sync family (#130), with
//! `Family::SyncPeer`: [`crate::sync_peer`] lists its names, and each peer
//! fixture a rule opens wraps one ingest fixture from this opener.
//!
//! # The guard against a bespoke emitter dropping rules
//!
//! The same as the event-store family's: [`rule_names`] expands the
//! enumeration through the testkit's `__ingest_rule_names` callback, not
//! through this emitter, so a rule missing from the `match` is one [`run`]
//! answers [`Dispatched::Unknown`], and the runner fails it by name.
//!
//! [`Violation`]: happenstance_sync_testkit::Violation

use core::cell::Cell;

use happenstance_cloudflare::worker::State;
use happenstance_cloudflare::worker::worker_sys::DurableObjectState;
use happenstance_testkit::RuleOutcome;

use crate::dispatch::{Dispatched, namespace_pool};
use crate::fixture::HarnessFaults;
use crate::ingest::fixture::WorkerdIngestFixture;
use crate::sync_peer::fixture::WorkerdSyncFixture;

/// CF-18's machine-checked half for the ingest family, which
/// `ingest_store_conformance!` emits around every harness.
///
/// The same text as [`crate::dispatch::DECLENSIONS`], because both macros name
/// their check alike. That is why the ingest routes are `/ingest-rules` and
/// `/ingest-rule/<name>`: one listing would carry the name twice, and
/// `/rule/<it>` would be ambiguous.
pub const DECLENSIONS: &str = "every_declined_capability_is_stated_by_this_fixture";

/// Every name the runner must run, from the ingest family's own enumeration.
#[must_use]
pub fn rule_names() -> Vec<&'static str> {
    let mut names = happenstance_sync_testkit::for_each_ingest_store_rule!(
        happenstance_sync_testkit::__ingest_rule_names
    )
    .to_vec();
    names.push(DECLENSIONS);
    names
}

/// Runs the ingest rule called `rule` inside the object whose state is
/// `state`, and whose raw state is `object_state` (the same JavaScript object).
///
/// A **pre-flight** fixture is opened on the pool's spare namespace before any
/// rule runs. If it cannot be opened, nothing runs and the answer is a harness
/// fault. A later `open` that fails is recorded, and the rule is handed the
/// pre-flight fixture instead, because the opener cannot fail by signature; the
/// recorded fault then outranks whatever the rule answered.
pub async fn run(state: &State, object_state: &DurableObjectState, rule: &str) -> Dispatched {
    run_family(state, object_state, Family::Ingest, rule).await
}

/// The families that run on [`WorkerdIngestFixture`]s: the fixture trait both
/// take is `IngestFixture`, so they share this dispatcher's opener.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Family {
    /// `for_each_ingest_store_rule!` and CF-18's declension check.
    Ingest,
    /// `for_each_write_group_rule!` (#420), which emits no declension check:
    /// see [`crate::write_group`].
    WriteGroup,
    /// `for_each_sync_peer_rule!` (#130), each rule handed
    /// [`WorkerdSyncFixture`]s built over this opener's ingest fixtures, and
    /// no declension check either: see [`crate::sync_peer`].
    SyncPeer,
}

/// [`run`]'s body, for the rule called `rule` of `family`.
pub(crate) async fn run_family(
    state: &State,
    object_state: &DurableObjectState,
    family: Family,
    rule: &str,
) -> Dispatched {
    let faults = HarnessFaults::default();
    let pool = match namespace_pool() {
        Ok(pool) => pool,
        Err(err) => return Dispatched::HarnessFault(vec![format!("namespace pool: {err}")]),
    };
    let Some((spare, pool)) = pool.split_last() else {
        return Dispatched::HarnessFault(vec!["the namespace pool is empty".to_owned()]);
    };
    // The fixture stores its namespace, so it gets its own copy (H-08).
    let preflight = match WorkerdIngestFixture::open(state, object_state, spare.clone()) {
        Ok(fixture) => fixture,
        Err(err) => {
            return Dispatched::HarnessFault(vec![format!("the pre-flight fixture: {err}")]);
        }
    };
    let opened = Cell::new(0_usize);
    let faults_ref = &faults;
    let preflight_ref = &preflight;
    let fallback = |fault: String| {
        // Logged as well as recorded, for `crate::dispatch::run`'s reason.
        worker::console_error!("{fault}");
        faults_ref.record(fault);
        // A handle onto the pre-flight store, which exists: `Clone` aliases.
        preflight_ref.clone()
    };
    let open = async || {
        let index = opened.get();
        opened.set(index.saturating_add(1));
        let Some(namespace) = pool.get(index) else {
            return fallback(format!("a rule opened more than {} fixtures", pool.len()));
        };
        WorkerdIngestFixture::open(state, object_state, namespace.clone())
            .unwrap_or_else(|err| fallback(err.to_string()))
    };

    macro_rules! emit_ingest_dispatch {
        ($($name:ident),* $(,)?) => {
            match rule {
                $(
                    ::core::stringify!($name) => Some(
                        happenstance_sync_testkit::ingest_rules::$name(open).await,
                    ),
                )*
                DECLENSIONS => Some(
                    happenstance_sync_testkit::ingest_declensions_are_stated(open)
                        .map(|()| RuleOutcome::Ran),
                ),
                _ => None,
            }
        };
    }

    // The write-group family's twin: the testkit's public `write_group_rules`,
    // which answer a violation as a value too, and no declension check.
    macro_rules! emit_write_group_dispatch {
        ($($name:ident),* $(,)?) => {
            match rule {
                $(
                    ::core::stringify!($name) => Some(
                        happenstance_sync_testkit::write_group_rules::$name(open).await,
                    ),
                )*
                _ => None,
            }
        };
    }

    // The sync family's twin: the testkit's public `rules`, which answer a
    // violation as a value too, each handed a peer fixture over an ingest
    // fixture from the same opener, so a failed `open` is recorded the same way.
    let open_peer = async || WorkerdSyncFixture::new(open().await);
    macro_rules! emit_sync_peer_dispatch {
        ($($name:ident),* $(,)?) => {
            match rule {
                $(
                    ::core::stringify!($name) => Some(
                        happenstance_sync_testkit::rules::$name(open_peer).await,
                    ),
                )*
                _ => None,
            }
        };
    }

    let outcome = match family {
        Family::Ingest => {
            happenstance_sync_testkit::for_each_ingest_store_rule!(emit_ingest_dispatch)
        }
        Family::WriteGroup => {
            happenstance_sync_testkit::for_each_write_group_rule!(emit_write_group_dispatch)
        }
        Family::SyncPeer => {
            happenstance_sync_testkit::for_each_sync_peer_rule!(emit_sync_peer_dispatch)
        }
    };

    let recorded = faults.take();
    match outcome {
        None => Dispatched::Unknown,
        // A verdict over a broken fixture says nothing about the adapter.
        Some(_) if !recorded.is_empty() => Dispatched::HarnessFault(recorded),
        Some(Err(violation)) => Dispatched::Violated(violation.to_string()),
        Some(Ok(outcome)) => outcome
            .skip_line(rule)
            .map_or(Dispatched::Ran, Dispatched::Skipped),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::{DECLENSIONS, rule_names};

    #[test]
    fn the_names_are_the_ingest_enumeration_plus_the_declension_check_once() {
        let names = rule_names();
        let enumerated = happenstance_sync_testkit::for_each_ingest_store_rule!(
            happenstance_sync_testkit::__ingest_rule_names
        );
        let distinct: BTreeSet<_> = names.iter().collect();
        assert_eq!(distinct.len(), names.len(), "no name appears twice");
        assert_eq!(names.len(), enumerated.len() + 1);
        assert_eq!(names.iter().filter(|name| **name == DECLENSIONS).count(), 1);
        assert!(enumerated.iter().all(|rule| names.contains(rule)));
    }
}
