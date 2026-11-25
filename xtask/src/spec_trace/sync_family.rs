//! What check 4 knows about the replication family (#458).
//!
//! Check 4 reads the seven sync-testkit families for an `SY` clause, as it reads
//! `suite.rs` and its neighbours for the contract's. This module holds the two
//! things that follow from that: where the message sends a reader when an `SY`
//! name resolves against none of them, and the `SY` names nothing can resolve,
//! each with the reason.
//!
//! # Why a module of its own
//!
//! The table is [`UNRESOLVABLE_RULE_NAMES`]'s sibling, not a second kind of
//! thing, and every rule written there applies here: reconciled in all three
//! directions on every run, so it can only shrink. It lives in its own file
//! because `.kb` atoms cite `spec_trace.rs` by line, and twenty entries
//! written into the parent's table would move every line those atoms cite
//! below it.

use super::{RULE_FILES, UNRESOLVABLE_RULE_NAMES, Unresolvable};

/// The prefix every sync-testkit family in [`RULE_FILES`] shares.
const SYNC_TESTKIT: &str = "crates/happenstance-sync-testkit/";

/// The sync-testkit families check 4 resolved an `SY` name against, joined for
/// the message it prints when the name is in none of them.
///
/// Derived from [`RULE_FILES`] rather than listed again, so that a family joining
/// the sweep joins the message in the same change.
pub(super) fn suites() -> String {
    RULE_FILES
        .iter()
        .copied()
        .filter(|file| file.starts_with(SYNC_TESTKIT))
        .collect::<Vec<_>>()
        .join(" or ")
}

/// The table a declaration for `clause` belongs in, as the remediation messages
/// name it: an `SY` clause's goes here, every other clause's in the parent's.
///
/// `every_sy_declaration_is_in_the_sync_table` refuses an `SY` entry in the
/// parent table, so a message that sent an `SY` name there would send its reader
/// into a second failure.
pub(super) fn table_for(clause: &str) -> &'static str {
    if clause.starts_with("SY-") {
        "sync_family::SYNC_UNRESOLVABLE_RULE_NAMES"
    } else {
        "UNRESOLVABLE_RULE_NAMES"
    }
}

/// Every declaration check 4 consults: the contract families' table, then this
/// module's.
pub(super) fn all_declarations()
-> impl Iterator<Item = &'static (&'static str, &'static str, Unresolvable)> {
    UNRESOLVABLE_RULE_NAMES
        .iter()
        .chain(&SYNC_UNRESOLVABLE_RULE_NAMES)
}

/// The owner of the seven `[FROZEN]` clauses whose named rule nobody wrote. A
/// keyword search over every `pub async fn` in the seven families found a
/// differently named rule enforcing an eighth, SY-3's, and ADR-0108 repointed
/// SY-3's `Rule:` line to it. #458 asks for each of the rest to be written or
/// for its `Rule:` line to be repointed, and a repoint of a `[FROZEN]` line is
/// a record's.
const OWED_BY_458: &str = "#458's second half: written into `happenstance-sync-testkit`, or the \
     `[FROZEN]` Rule line repointed by a record";

/// Every `(clause, rule)` pair an `SY` clause states and no resolution source
/// can find, with the reason.
///
/// The count in the type is written twice on purpose, as the parent table's is:
/// a change to it is a change a reviewer should be made to see.
pub(super) const SYNC_UNRESOLVABLE_RULE_NAMES: [(&str, &str, Unresolvable); 19] = [
    // ---- Scheduled: nothing has written them -----------------------------
    // SY-3's `compensation_is_idempotent_under_redelivery` left this table at
    // ADR-0108, which repointed SY-3's line to SY-12's
    // `dedupe_reaches_identity_without_decoding`: it delivers the same batch
    // twice and refuses a second compensation, SY-3's stated assertion.
    // SY-6's `wire_condition_with_after_is_refused` left at #536 (ADR-0114
    // (i)), in the write-group family, where check 4 resolves it.
    (
        "SY-9",
        "one_adapter_serves_both_roles",
        Unresolvable::Scheduled(OWED_BY_458),
    ),
    (
        "SY-13",
        "identity_survives_a_tag_reordering_peer",
        Unresolvable::Scheduled(OWED_BY_458),
    ),
    (
        "SY-16",
        "resume_survives_a_dropped_peer_handle",
        Unresolvable::Scheduled(OWED_BY_458),
    ),
    (
        "SY-19",
        "two_peers_disagree_on_total_order",
        Unresolvable::Scheduled(OWED_BY_458),
    ),
    (
        "SY-26",
        "causality_is_expressible_without_a_contract_change",
        Unresolvable::Scheduled(OWED_BY_458),
    ),
    (
        // `[PROVISIONAL]`, renewed past 1.0 by ADR-0027 and confirmed by
        // ADR-0107: no 1.0 port method takes a peer-supplied `Query`, so there
        // is nothing for the rule to call until the proposal puts one there.
        "SY-29",
        "ingest_refuses_an_unbounded_peer_query",
        Unresolvable::Scheduled(
            "the first scoped-replication proposal, which SY-29's own marker says puts a \
             `Query` on the port and builds this rule with its ADR",
        ),
    ),
    // SY-33's `refusal_is_never_condition_derived` left this table when #465
    // wrote it into the sync peer family, where check 4 resolves it.
    (
        "SY-35",
        "the_sync_suite_never_decodes",
        Unresolvable::Scheduled(OWED_BY_458),
    ),
    // ---- Elsewhere: the test exists, in a file no family is ---------------
    (
        // Moved here from the parent table, where it was declared for the
        // other half before check 4 read `SY` at all: SY-21 is `[FROZEN]` on
        // it, and deleting it must fail the gate.
        "SY-21",
        "apply_is_handed_the_origin_identity_not_the_local_position",
        Unresolvable::Elsewhere("crates/happenstance/tests/projection_runner.rs"),
    ),
    (
        // SY-34's line since ADR-0108 (#465): the refusal is the runner's to
        // survive, and the store is never told of one, so the test holds the
        // shipped `Edge` in `happenstance-sync`'s own runner tests.
        "SY-34",
        "a_refused_push_keeps_the_log_and_the_watermark_and_records_the_class",
        Unresolvable::Elsewhere("crates/happenstance-sync/tests/runner/main.rs"),
    ),
    (
        // The same assertion held against a wrong runner that rolls its own
        // events back on a refused push, ADR-0108's open point C (#487).
        "SY-34",
        "a_refused_push_keeps_the_log_rejects_a_runner_that_rolls_back_on_refusal",
        Unresolvable::Elsewhere("crates/happenstance-sync/tests/runner/wrong_runners.rs"),
    ),
    (
        // A request count inside one `ingest` call, which no port method sees
        // and CF-33 leaves to the adapter, so only Neon's own crate can pin it.
        "SY-14",
        "rotors_ingest_is_one_request_per_push_however_much_is_held",
        Unresolvable::Elsewhere("crates/happenstance-neon/src/event_store/ingest/tests/bulk.rs"),
    ),
    (
        "SY-14",
        "rotors_bulk_ingest_through_the_live_neon_peer_is_bounded",
        Unresolvable::Elsewhere("crates/happenstance-neon/src/event_store/ingest/tests/bulk.rs"),
    ),
    (
        // SY-38 on the Durable Object, run under `wasm_bindgen_test` in that
        // adapter's own crate.
        "SY-38",
        "an_unheld_own_identity_stops_the_ingest_and_writes_nothing",
        Unresolvable::Elsewhere(
            "crates/happenstance-cloudflare/src/event_store/ingest/tests/claims.rs",
        ),
    ),
    (
        "SY-38",
        "a_claim_the_calls_own_compensation_would_hide_is_refused",
        Unresolvable::Elsewhere(
            "crates/happenstance-cloudflare/src/event_store/ingest/tests/claims.rs",
        ),
    ),
    (
        "SY-38",
        "local_appends_are_not_wedged_after_a_refused_claim",
        Unresolvable::Elsewhere(
            "crates/happenstance-cloudflare/src/event_store/ingest/tests/claims.rs",
        ),
    ),
    (
        "SY-38",
        "a_claim_lands_as_foreign_after_the_store_is_reminted",
        Unresolvable::Elsewhere(
            "crates/happenstance-cloudflare/src/event_store/ingest/tests/claims.rs",
        ),
    ),
    // ---- NotARuleName: `PeerLimits`' fields, which the sentence needed ----
    ("SY-18", "max_batch_events", Unresolvable::NotARuleName),
    ("SY-18", "max_batch_bytes", Unresolvable::NotARuleName),
    ("SY-18", "max_event_bytes", Unresolvable::NotARuleName),
];
