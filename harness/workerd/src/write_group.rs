//! The write-group family on `workerd` (#420, SY-39): every rule of
//! `for_each_write_group_rule!`, run against `CloudflareIngestStore` inside the
//! object, through the routes `/write-group-rules` and
//! `/write-group-rule/<name>`.
//!
//! The rules take an `IngestFixture` whose store also reads its write groups,
//! so they run on the ingest family's [`WorkerdIngestFixture`] through the
//! ingest dispatcher's opener (its `run_family` with `Family::WriteGroup`): one
//! fixture per instance over its own namespace, the store built by the
//! published `namespaced_from_object_state`, and each ingest one real
//! `ctx.storage.transactionSync`. What this adds on the runtime is the
//! adapter's own write path: the migration's `PRAGMA table_info` probe and its
//! `ALTER TABLE`s, which the Node shim's SQLite answers without the object's
//! authoriser, and the group read's one cursor.
//!
//! # The guard against a bespoke emitter dropping rules
//!
//! [`crate::ingest::dispatch`]'s: [`rule_names`] expands the enumeration through
//! the testkit's `__write_group_rule_names` callback, not through the emitter,
//! so a rule missing from the dispatch `match` answers
//! [`Dispatched::Unknown`], and the runner fails it by name.
//!
//! # No declension check
//!
//! `write_group_store_conformance!` emits none: no write-group rule is
//! `require!`-gated on a capability, so there is nothing for a fixture to
//! decline. Every rule must answer `ran`.
//!
//! [`WorkerdIngestFixture`]: crate::ingest::fixture::WorkerdIngestFixture

use happenstance_cloudflare::worker::State;
use happenstance_cloudflare::worker::worker_sys::DurableObjectState;

use crate::dispatch::Dispatched;
use crate::ingest::dispatch::{Family, run_family};

/// Every name the runner must run, from the write-group family's own
/// enumeration.
#[must_use]
pub fn rule_names() -> Vec<&'static str> {
    happenstance_sync_testkit::for_each_write_group_rule!(
        happenstance_sync_testkit::__write_group_rule_names
    )
    .to_vec()
}

/// Runs the write-group rule called `rule` inside the object whose state is
/// `state`, and whose raw state is `object_state` (the same JavaScript object),
/// as [`crate::ingest::dispatch::run`] runs an ingest rule.
pub async fn run(state: &State, object_state: &DurableObjectState, rule: &str) -> Dispatched {
    run_family(state, object_state, Family::WriteGroup, rule).await
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::rule_names;

    /// The names are the enumeration, in its order, each once: the runner
    /// names one object per rule from them.
    #[test]
    fn the_names_are_the_write_group_enumeration_each_once() {
        let names = rule_names();
        let enumerated = happenstance_sync_testkit::for_each_write_group_rule!(
            happenstance_sync_testkit::__write_group_rule_names
        );
        let distinct: BTreeSet<_> = names.iter().collect();
        assert_eq!(distinct.len(), names.len(), "no name appears twice");
        assert_eq!(names, enumerated);
    }
}
