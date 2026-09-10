//! The suite's proof that it discriminates: the mutant registry and its eight
//! meta-tests.
//!
//! Before this file, **three of the suite's rules had been shown to reject
//! something and the rest had not** — and all three by a hand-written
//! `#[should_panic]` driver that recorded only that *something* failed, which is
//! the shape CF-2 rejects by name and the reason the registry replaces it rather
//! than supplementing it. (ADR-0010's Context sentence says "not one", and was
//! written before stage 2 landed `fixture_instruments.rs`; the accurate version
//! is no weaker an argument.)
//!
//! The rules were written by reading the contract and asserting what it says,
//! which is how every conformance suite starts and is not sufficient: a rule that
//! asserts something no plausible implementation gets wrong passes forever,
//! certifies nothing, and is indistinguishable from a good rule until an adapter
//! with the corresponding bug passes it. A reviewer measured four plausible wrong
//! implementations passing the suite as it stood.
//! [ADR-0010](../../../.kb/decisions/0010-the-suite-must-prove-itself.md) is the
//! decision; `SPECIFICATION.md` §6.1's CF-1 – CF-6 are the clauses.
//!
//! The eight tests live in a `mod mutation_coverage` inside this file so that the
//! printed name and `cargo test mutation_coverage::every_rule_has_a_mutant` both
//! match the name the clauses cite. A clause name a reviewer cannot paste into
//! `cargo test` is a clause name that drifts.
//!
//! # Never quote a pass rate over this registry
//!
//! It is an author-chosen bug set, so the denominator is a choice: a fraction
//! says how representative the author was and reports it as though it said how
//! good the suite is. Report which defects the set covers and which axes it
//! leaves uncovered. (ADR-0010, §1; `SPECIFICATION.md` §6.1's closing note.)
//!
//! # Why the whole file is gated off `wasm32`, and what that costs
//!
//! [`std::panic::catch_unwind`] cannot catch on a target with no unwinder, and a
//! meta-test that cannot observe a panic is the whole mechanism gone. One
//! crate-level attribute rather than eighteen per-item ones;
//! `tests/local_conformance.rs`'s `mutants` module carries the same gate, so
//! this is precedent rather than novelty.
//!
//! **The cost, stated rather than discovered later:** the stores in this binary
//! are never type-checked for `wasm32`. A phase-4 signature change that only
//! breaks on that target will break `local_conformance.rs`'s wasm harness and
//! not this file, and the two then get fixed in separate sittings.

#![cfg(not(target_arch = "wasm32"))]
// Test code, per the house rule and `tests/local_conformance.rs:39`'s precedent.
#![allow(clippy::unwrap_used)]

// `#[path]` is not decoration. The root of a test target resolves `mod foo;`
// against its own *directory* — `tests/` — not against a `tests/mutation_coverage/`
// subdirectory, because the crate-root rule and the `mod.rs` rule are the same
// rule. Without these attributes `mod harness;` looks for `tests/harness.rs`,
// which would put four support files in the same directory as the test targets
// and invite cargo to compile them as targets of their own.
#[path = "mutation_coverage/correct.rs"]
mod correct;
#[path = "mutation_coverage/harness.rs"]
mod harness;
#[path = "mutation_coverage/mutants.rs"]
mod mutants;
#[path = "mutation_coverage/racers.rs"]
mod racers;
#[path = "mutation_coverage/variants.rs"]
mod variants;

use happenstance_core::{Query, ReadOptions, SequencedEvent};
use harness::{Origin, RUNTIME_PANICS, SubjectReport, Verdict};

// =====================================================================
// The registry, as data
// =====================================================================

/// What kind of store an entry describes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kind {
    /// A store with a named defect, which MUST fail exactly the rules it
    /// declares.
    Mutant,
    /// A store that is legally different from `MemoryEventStore` and MUST pass
    /// everything (CF-5).
    ConformantVariant,
    /// A store with a named defect that **no rule of the event-store family can
    /// see**, and which the *model* family therefore has to.
    ///
    /// It looks like a hole in [`Kind::Mutant`]'s contract and is the opposite:
    /// a mutant with an empty `fails` list is rejected outright, because a
    /// defect nothing catches is a defect nobody knows about. This kind is what
    /// makes "nothing in this family catches it, and here is what does" a
    /// *checked* claim.
    ///
    /// It carries **three** obligations, and the split between them is the whole
    /// design:
    ///
    /// 1. Every rule of the event-store family must pass or skip for a declared
    ///    reason — exactly as for a mutant with nothing in `fails`. That is the
    ///    claim *"no rule sees it"*, measured rather than asserted, by
    ///    `mutants_fail_exactly_their_declared_rules`.
    /// 2. It must carry a [`Witness`] in [`MODEL_ONLY_WITNESSES`]: a fixed log
    ///    and read on which its own `Defect::select` answers differently from
    ///    [`crate::correct`]. That is the claim *"it is defective at all"*, and
    ///    it holds in **every feature configuration**, which is what obligation
    ///    3 cannot do.
    /// 3. [`MODEL_COVERAGE`] must claim it [`ModelOutcome::Rejected`], and the
    ///    model must deliver it. That is the claim *"the model is what catches
    ///    it"*.
    ///
    /// Obligation 2 was not there when this kind landed, and its absence was a
    /// real hole rather than a theoretical one. An adversarial review registered
    /// [`crate::mutants::HidingPlaceStore`] — a bare `impl Defect` with no
    /// defect whatever — under this kind, satisfying 1 honestly and 3 with a
    /// lie, and `cargo test --no-default-features --test mutation_coverage`
    /// reported nine passes, because obligation 3 is behind
    /// `#[cfg(feature = "proptest")]` and `cargo hack`'s powerset builds that
    /// configuration. `Kind::Mutant` is unabusable because its bar is data in
    /// this same table; a kind that borrows a *family* for its bar can have the
    /// bar compiled out from under it.
    ///
    /// The shape it exists for is a defect that needs a *query* and a *read
    /// option* together, where the family's rules for that option all issue
    /// `Query::all()`. The suite's answer to that for `from` was CF-12's rule;
    /// where no such rule exists, the generative family is what is left, and
    /// this kind records which of the two is doing the work.
    ///
    /// # It has no members at this commit, and that is the healthy outcome
    ///
    /// `UnparenthesisedToPredicateStore` was the only store ever filed here — the
    /// `to` half of exactly the shape above, with all three `to` rules issuing
    /// `Query::all()`. `read_to_composes_with_multi_item_query` is now the rule
    /// the kind's own documentation said would end it, so the row is an ordinary
    /// [`Kind::Mutant`] with one entry in `fails`.
    ///
    /// That matters more than a tidy-up, because the gap was worse than "no rule
    /// sees it": the model family is `proptest`-gated **and**
    /// `cfg(not(target_arch = "wasm32"))`, so on the two targets that most need
    /// a generated `WHERE` clause — Cloudflare Workers and Neon — nothing in the
    /// binary caught it at all. A kind that records "the other family has this"
    /// is only as good as where the other family runs.
    ///
    /// The variant is **kept**, empty. Withdrawing it is a decision about the
    /// registry's vocabulary and belongs to an ADR pass, not to the lane that
    /// happened to empty it; and the machinery is dormant rather than wrong —
    /// [`MODEL_ONLY_WITNESSES`] is empty, the two tests over it are tied to that
    /// emptiness in both directions, and a store filed here tomorrow is held to
    /// all three obligations unchanged.
    ModelOnlyMutant,
}

/// Why a mutant's declared rule fails.
///
/// **This is not optional bookkeeping.** CF-2's `Rejects:` names the hazard in
/// terms: *a mutant which fails the right rule for the wrong reason (a panic in
/// its constructor, an unrelated regression) reads as proof.* Without this
/// field, a mutant that starts panicking on a slice index during phase 4's
/// refactor keeps reading green — it still panics, the meta-test still counts a
/// failure, and the claim "this rule catches this defect" becomes false while
/// the test stays passing.
///
/// # It is one field per mutant, not per (mutant, rule)
///
/// So a mutant that fails three rules claims the same mode for all three. That
/// is a real limitation and it is left standing deliberately: no registered
/// mutant needs the split today — every [`FailureMode::StorePanic`] row fails
/// *only* by store panic, and every [`FailureMode::Assertion`] row only by
/// assertion — and a per-rule mode would turn every `fails` entry into a tuple
/// for the sake of a case that does not exist. The first mutant that fails one
/// rule by assertion and another by a store panic cannot be expressed, and is
/// the signal to change the shape rather than to file it under the wrong mode.
///
/// [`Declared::expect`] *is* keyed by rule, and the difference is not an
/// inconsistency. That signal arrived: `PreCommitPositionStore` fails two rules
/// at two different assertions, and the response was to change the shape rather
/// than to drop the pin. This field's signal has not arrived.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FailureMode {
    /// The rule's own assertion fired.
    ///
    /// Checked **positively**, by where the panic was raised: a rule's
    /// assertions — and the `append_ok` / `read_ok` helpers that panic on a
    /// rule's behalf — all live in `happenstance-testkit/src/suite.rs`, so a
    /// panic from anywhere else is something other than the rule rejecting the
    /// store. [`RUNTIME_PANICS`] is then a second pass over the message, for the
    /// standard-library panic raised *inside* a rule body, which the location
    /// cannot distinguish.
    Assertion,
    /// The store itself panicked, and the message MUST contain this substring.
    StorePanic(&'static str),
}

/// One row of the registry: a store, and the exact claim made about it.
///
/// # Why this is a hand-written `const` and not generated
///
/// Two proposals arrive here and both must be refused.
///
/// **Generating the table from observed outcomes** turns the proof artefact into
/// a snapshot test, and a snapshot of a mutant registry asserts nothing at all:
/// every future regression is "expected", which is the exact failure mode the
/// registry exists to prevent.
///
/// **Putting the declaration on the impl as an associated const** lets a
/// reviewer edit the claim in the same keystroke as the bug it describes, and
/// the claim then stops being a check. The distance between the store and the
/// claim about it is the mechanism.
#[derive(Debug)]
struct Declared {
    /// Matches the store's own [`harness::Subject::NAME`].
    name: &'static str,
    /// Mutant or conformant variant.
    kind: Kind,
    /// The **exact** set of rules this store fails. Empty iff
    /// [`Kind::ConformantVariant`].
    fails: &'static [&'static str],
    /// CF-4: the real adapter shape or scenario that makes this store
    /// plausible. Never empty.
    provenance: &'static str,
    /// How the declared rules fail. Never consulted for a conformant variant.
    mode: FailureMode,
    /// Per-rule pins: `(rule, substring)` pairs naming the **exact assertion**
    /// this mutant is expected to trip in that rule.
    ///
    /// [`FailureMode`] says the rule rejected the store; it does not say *which*
    /// of the rule's assertions did the rejecting, and several rules carry more
    /// than one. `CachedHeadFixture`'s provenance claims "its reads are correct
    /// and only its condition probe is stale" — a future edit that made it trip
    /// the read-side assertion instead would keep the meta-test green while the
    /// mutant stopped demonstrating what it says it does.
    ///
    /// This is the coverage the deleted `#[should_panic(expected = "…")]`
    /// drivers used to carry.
    ///
    /// # Why this is keyed by rule rather than one field per mutant
    ///
    /// It was `Option<&'static str>` — one pin per mutant — until
    /// `PreCommitPositionStore` acquired a second declared failure with a
    /// different message and the pin came off rather than the shape changing.
    /// That is the wrong way round: the mutant the registry calls "the one entry
    /// whose bug is a faithful model of a real database" is exactly the one that
    /// must stay pinned, and `nothing_below_an_observed_position_appears_later`
    /// has two failure surfaces — the ES-10 ordering assertion and the closing
    /// non-vacuity anchor — so an unpinned declaration certifies the visibility
    /// obligation on the strength of a row count. Keying by rule costs one tuple
    /// per pin and removes the reason to ever drop one.
    ///
    /// A pin is optional per (mutant, rule): an empty slice pins nothing, and a
    /// rule absent from the slice is unpinned. What is *not* optional is that a
    /// pin naming a rule the mutant does not declare is an error —
    /// `mutant_registry_is_exhaustive` rejects it, because a pin on a rule that
    /// never runs is a claim nothing evaluates.
    expect: &'static [(&'static str, &'static str)],
}

/// Every store this binary drives.
///
/// Adding one is three edits: write the defect in `mutants.rs`, add its type to
/// [`for_each_mutant!`], add its row here. The enumeration and the claim are
/// deliberately separate — they are the two lists that drift, and
/// `mutant_registry_is_exhaustive` is what holds them together.
///
/// # What this set covers, and what it does not
///
/// **Scope: the event-store port only.** Every store here implements
/// `EventStore`, and every rule it is driven through comes from
/// `for_each_event_store_rule!`. The projection store port owes its own CF-1 –
/// CF-5 obligation and now discharges it in a registry of its own,
/// `tests/projection_mutation_coverage.rs` — same six `Declared` fields, same
/// semantics, its own rule universe taken from `for_each_projection_store_rule!`.
/// CF-5's projection half is still open there and is named in that file rather
/// than tracked here. Nothing below says anything about either.
///
/// ADR-0010 requires both halves of that sentence and forbids the third thing
/// anyone would write instead. **Never quote a pass rate over this table.** The
/// denominator is an author's choice, so a fraction says how representative the
/// author was while reading as though it said how good the suite is.
///
/// *Covered:* the SQL query builder in all six of the ways it is joined wrongly
/// (a type list conjoined, a tag set disjoined, tags compared for equality, the
/// two clauses of an item disjoined, items conjoined, the type clause dropped
/// when interning misses); a query's items interned by their type list, so a
/// second item with the same types is dropped; the tag side table joined with
/// `INNER JOIN` and again without `DISTINCT`; a multi-item query unioned per item
/// and never merge-sorted; ordering by a covering index; `from` read as an
/// `OFFSET`; `backwards` lost between two structs; an upper bound ignored, read
/// as exclusive, and never swapped under `backwards`; a budget of zero read as no
/// budget, a budget written per query item and never re-applied to the union,
/// `LIMIT n + 1`, `LIMIT` pushed into the scan ahead of the filter, and a budget
/// spent everywhere except the branch that serves a forward cursor;
/// `WHERE a OR b AND position >= ?` without parentheses; a paging window anchored on a `max(position)` that is `NULL` on
/// an empty store; one position bound for a whole batch; `RETURNING` read from
/// the wrong end and read without checking there is a row; probe-then-insert
/// outside a transaction; an empty batch treated as a no-op, and an empty batch
/// answered by the condition check instead of by the argument check; a column
/// missing from the `INSERT` list; every one of the four ways `after` is
/// mishandled (defaulted, ignored, made inclusive, read as an offset), plus
/// `after` validated against the store's own head, a probe that ANDs two
/// uncorrelated predicates, a probe that compares tags for equality, a probe
/// that drops the tag join entirely, an aggregate probe that reads SQL's
/// three-valued `NULL` as a rejection, and a violation surfaced as a driver
/// error. Three ways a `head` is wrong: a `NULL` aggregate spent on a
/// position that is not one, a head scoped to the read path's own tag join,
/// and a head cached per handle. Three more value edges: a normalising
/// collation, a tag index shaped as a key-to-value map, and an identifier
/// trimmed on the way in. Seven store-assigned facts got wrong: an identity
/// synthesised from a row's ordinal, one derived from the payload, one
/// minted per event, one bound once per statement, one materialised as a
/// queryable tag, a time read from the connection's clock, and a membership
/// test that drops the incarnation from its `WHERE` clause. On the fixture side: a stale per-handle head, a write acknowledged
/// before it is durable, and two fixture instances that are one store. On the
/// isolation axis, a read that takes a fresh sample per page. On an
/// axis of its own, a position allocated *before* the transaction that publishes
/// the row commits. And, on re-entrancy, a `read` stream that keeps a borrow of
/// the store alive and an `append` that holds an exclusive one across an
/// `.await`.
///
/// One caution about reading the table as a map. A mutant that makes reads return
/// *nothing* — `InnerJoinTagStore` is the one — trips many rules at their setup
/// **anchors** rather than at the property the rule is named for:
/// `two_fixture_instances_observe_none_of_each_others_appends` fails at "the
/// append must actually have landed", `read_from_is_inclusive` and
/// `query_items_are_or` at their `all.len()` guards. Those rows evidence the
/// anchor, not the headline property, and they are inflation rather than
/// vacuity.
///
/// This paragraph used to close by saying that no rule in the table is covered
/// by that mutant alone. That was false, and it was the one sentence a reviewer
/// uses to stop checking: `untagged_events_match_query_all` appears in exactly
/// one `fails` list in the whole event-store registry, and it is this one. The
/// intent survives where the sentence did not — for that rule
/// `InnerJoinTagStore` is the headline mutant rather than an inflation row,
/// because the rule body carries a single assertion and so has no anchor to
/// fail at. That is a property of the rule today rather than of the registry,
/// which is why it is now pinned rather than restated: the `expect` field names
/// the assertion, and `the_shotgun_mutants_sole_coverage_is_pinned` requires a
/// pin wherever this mutant is a rule's only evidence. The day that rule
/// acquires a second assertion the pin fails, rather than the discount quietly
/// becoming wrong while the paragraph tells the auditor not to look. Elsewhere
/// the `expect` field is what makes the distinction visible where it has been
/// written down.
///
/// *Not covered, and each of these is a real axis rather than an oversight:*
///
/// * **Sequence-allocation strategies observable only across a reopen.**
///   `COUNT(*) + 1` is the runbook's example and it is deliberately **not** a
///   mutant here: `MemoryEventStore` already allocates that way
///   (`position_at(stored.len() + offset)`) and is conformant. In an append-only
///   log with no deletions and no pre-existing gaps, `COUNT(*) + 1` and
///   `MAX(position) + 1` are the same function, and no fixture that starts empty
///   can separate them. The reopen is not the observation; the *pre-existing
///   gap* is, and gaps only arise from rows a fixture cannot create.
/// * **Per-handle position caches.** `CachedMaxPositionStore` — each handle
///   caching `max(position)` when it opens and allocating from the cache, so two
///   handles assign the same position — is the observable sibling of the entry
///   above and is a genuine defect. No rule that exists today can see it: the
///   only rule where two handles both reach `append`
///   (`two_handles_observe_each_others_appends`) has the second one *rejected*,
///   so no second position is ever allocated. **Fifteen rules landed at stage 4
///   and none of them produced that shape**, which is the useful part of the
///   report: the rule this mutant needs is the multi-connection form of
///   `positions_are_unique`, no clause names one, and writing a rule whose only
///   justification is that a mutant needs somewhere to live is the tail wagging
///   the dog — ADR-0010 says so in as many words. It lands when a clause asks
///   for it.
/// * **A read that tears between two *statements* of one query rather than
///   between two pages.** `RefetchingPagedStore` re-samples per page, which is
///   what rejects both ES-11's and ES-12's rules; the per-`QueryItem` shape
///   ES-12's `Rejects:` names — one SQL statement per item, unioned
///   client-side — is the same defect at a different granularity and is not
///   separately registered. It would need a stream whose poll schedule is one
///   statement per item, and it would fail exactly the rules
///   `RefetchingPagedStore` already fails, so it would buy a shape in the
///   catalogue and no coverage. ADR-0011 §3's finding is that ES-12 is
///   discharged by ES-11's ceiling rather than by a second mechanism, and this
///   table is the same finding seen from the instrument side.
/// * **A defect that is a deadlock rather than a panic.** `BorrowHoldingStore`
///   and `AwaitAcrossBorrowStore` are registered now, and both are `RefCell`
///   stores that answer a conflicting borrow immediately. The adapters they
///   model — a pooled SQL store holding one connection — answer the same
///   question by *waiting*, and `harness.rs` explains why there is deliberately
///   no watchdog to turn that into a message. The re-entrancy rules are written
///   so that a store which cannot answer is separated from one that answers
///   wrongly; what nothing here can do is make the first case fail *quickly*.
/// * **Real faults.** Nothing here injects a failure *between two rows of a
///   batch*, so `append_is_atomic_under_a_mid_batch_fault` is still owed —
///   a rolled-back batch and a batch written before the decision is made are
///   different defects, and only `WriteThenCheckStore` is the second one.
///   ADR-0010's amendment about `append_is_atomic` was itself amended on the
///   strength of that store: the rule is **retained**, because the batch does
///   reach the write path there, and ES-18 now claims both names.
///   More broadly, **no fixture in this binary has a medium outside the
///   process**: every "durable" store is a `Vec` behind an `Rc`, so
///   `LosingFixture` models acknowledge-before-commit rather than testing it, and
///   a store that survives `reopen` here has survived a pointer swap. §6.5
///   carries that limitation, and phase 8 is where a real medium arrives.
/// * **Concurrency in earnest** — *closed at stage 5, and the entry is kept
///   because it says where the answer went.* Every mutant in **this** table is
///   still driven on one thread by `block_on`, and still must be: they are
///   `Rc`-backed, which is what keeps them exercising the flavour ADR-0001
///   exists for. What was missing was a store whose defect appears only under
///   genuine *parallelism* — a lost update between two OS threads — and that is
///   now [`RACERS`](super::RACERS), a second table of six `Arc`/`Mutex` stores
///   driven through `event_store_concurrency_conformance!`. It is a separate
///   table rather than six more rows here for a mechanical reason: every store
///   in it fails **no** rule of the event-store family, and
///   `mutant_registry_is_exhaustive` rejects a row with an empty `fails` list.
///   The claim that the port "deliberately carries no `Send` bound" and
///   therefore cannot express the defect was the part that was wrong: the port
///   carries *two* flavours, and the concurrency family binds the other one.
const REGISTRY: &[Declared] = &[
    // --- Conformant variants (CF-5) -----------------------------------
    Declared {
        name: "GappedPositionStore",
        kind: Kind::ConformantVariant,
        fails: &[],
        provenance: "positions allocated from a Postgres sequence with CACHE 7, from a \
             transaction id, or with a shard id in the low bits — all three are \
             legal, none of them is dense, and none of them starts at 1. This is \
             the store CF-6 is enforced by: a rule asserting a literal position \
             value passes against MemoryEventStore and fails here.",
        mode: FailureMode::Assertion,
        expect: &[],
    },
    Declared {
        name: "PagedStreamStore",
        kind: Kind::ConformantVariant,
        fails: &[],
        provenance: "any store with a network under it: happenstance-neon fetches over \
             one-shot HTTP and a Durable Object's SqlStorage answers \
             asynchronously, so a read stream that is ready on first poll is a \
             property of in-memory adapters rather than of the contract. It is \
             also the workspace's only exercise of `block_on`'s parking path.",
        mode: FailureMode::Assertion,
        expect: &[],
    },
    // --- Query semantics: the read filter ------------------------------
    Declared {
        name: "InnerJoinTagStore",
        kind: Kind::Mutant,
        fails: &[
            "two_fixture_instances_observe_none_of_each_others_appends",
            "query_all_matches_every_event",
            "query_item_types_are_or",
            "query_items_are_or",
            "untagged_events_match_query_all",
            // Since the rule gained its non-vacuity anchor: this store drops
            // every untagged event from every read, so the anchor — a query for
            // the type just appended must select it — is what it fails. Another
            // inflation row, and the reason the anchor was owed: without it a
            // store that returns nothing at all passed the rule.
            "query_matching_nothing_yields_empty",
            "duplicate_items_do_not_duplicate_events",
            "query_item_order_does_not_change_the_result_set",
            "read_from_is_inclusive",
            "read_backwards_reverses_order",
            "read_limit_truncates",
            "read_backwards_from_with_limit",
            "read_defaults_to_ascending_order",
            "read_limit_applies_after_filtering",
            "read_backwards_limit_applies_after_filtering",
            "read_from_composes_with_multi_item_query",
            // Slice F: this store drops every untagged event from every read,
            // and that rule seeds an untagged event on purpose — so it fails at
            // the anchor rather than at the head relation. Inflation, on the same
            // reading the table doc comment already gives for this store.
            "head_is_the_highest_visible_position",
            "positions_are_strictly_monotonic",
            "append_returns_last_written_position",
            "append_is_atomic",
            "condition_rejection_leaves_store_unchanged",
            "a_live_read_stream_does_not_block_an_append",
        ],
        provenance: "the two-table shape at `crates/happenstance-sqlite/src/event_store.rs`, \
             where tags are rows in a side table. `INNER` is the default anyone \
             writes first, and it is invisible until an untagged event is \
             written.",
        mode: FailureMode::Assertion,
        // The one rule in the whole event-store registry this store is the
        // *only* evidence for, so it is the one row here that may not be read
        // as anchor inflation. Pinned to the head relation itself: the rule
        // carries a single assertion today, and this is what makes a second
        // one visible rather than silently converting the mutant's only
        // registered evidence into an anchor failure.
        expect: &[(
            "untagged_events_match_query_all",
            "must still be matched by Query::all()",
        )],
    },
    Declared {
        name: "TypesAreAndStore",
        kind: Kind::Mutant,
        fails: &["query_item_types_are_or"],
        provenance: "the clause list joined with the separator used *between* items. One \
             `join` helper, two call sites, and the inner one wants OR while the \
             outer one wants OR of ANDs.",
        mode: FailureMode::Assertion,
        expect: &[],
    },
    Declared {
        name: "TagsAreOrStore",
        kind: Kind::Mutant,
        fails: &[
            "query_item_tags_are_and",
            "query_item_rejects_partial_tag_overlap",
        ],
        provenance: "`WHERE (k=? AND v=?) OR (k=? AND v=?)` with no \
             `GROUP BY … HAVING COUNT(*) = n` — the commonest way to get \"all of \
             these tags\" wrong in SQL, because the per-tag predicate is right and \
             only the aggregation is missing.",
        mode: FailureMode::Assertion,
        expect: &[],
    },
    Declared {
        name: "ExactTagMatchReadStore",
        kind: Kind::Mutant,
        fails: &[
            "query_item_tags_match_supersets",
            // Slice F: VT-17's rule writes three tags on one event and queries
            // with one of them, which is a superset probe wearing a different
            // clause's name. Inflation — the defect is the one above, met by an
            // arrangement written for something else.
            "tags_may_repeat_a_key",
        ],
        provenance: "tags serialised to one canonical column and compared with `=`, which \
             is what an adapter does to avoid a side table and a join. Every \
             exact-match rule still passes, which is what makes it survivable.",
        mode: FailureMode::Assertion,
        expect: &[],
    },
    Declared {
        name: "ClauseJoinerStore",
        kind: Kind::Mutant,
        fails: &["query_item_combines_types_and_tags_with_and"],
        provenance: "the clause vector `join(\" OR \")`ed at both levels. Harmless whenever \
             an item carries one constraint, which is why it survives every other \
             rule in the suite and why it needs a rule with both.",
        mode: FailureMode::Assertion,
        expect: &[],
    },
    Declared {
        name: "ItemsAreAndStore",
        kind: Kind::Mutant,
        fails: &[
            "query_items_are_or",
            "query_item_order_does_not_change_the_result_set",
            "read_from_composes_with_multi_item_query",
            // Not inflation: VT-23's rule is a 128-item query with one matching
            // item, which is the OR defect at a hundredfold scale. Every
            // multi-item rule in the suite catches this store, and a rule that
            // did not would be one whose items all match.
            "store_evaluates_a_query_at_the_guaranteed_minimum_item_count",
            // The three multi-item rules slice F added, and the same reading:
            // each fails at its arrangement anchor, because no event matches
            // both items under an AND. Inflation rather than coverage — no rule
            // among them is covered by this store alone.
            "limit_applies_across_items_not_per_item",
            "query_union_is_item_concatenation",
            "query_items_share_one_snapshot",
            // And L1-1's rule for the same reason: its two items match three
            // events each and nothing matches both, so the budgeted page comes
            // back empty and this store fails at the arrangement anchor rather
            // than at the budget.
            "read_from_composes_with_limit",
            // And the `to` twin of CF-12's rule, for the same reason once more:
            // its tag item and its type item share no event, so the window comes
            // back empty and this store fails at the arrangement anchor rather
            // than at the bound.
            "read_to_composes_with_multi_item_query",
        ],
        provenance: "the same joiner bug one level up: the item list assembled with the \
             separator that belongs inside an item.",
        mode: FailureMode::Assertion,
        expect: &[],
    },
    Declared {
        name: "UninternedTypeStore",
        kind: Kind::Mutant,
        fails: &[
            "query_matching_nothing_yields_empty",
            "condition_without_after_allows_non_match",
            "condition_after_ignores_non_matching_events",
        ],
        provenance: "interning is the standard way to avoid storing a type string per row. \
             An unknown type yields an empty id list, `type_id IN ()` is a syntax \
             error, and the clause is dropped rather than the query being refused \
             — so the read widens instead of narrowing. One query builder serves \
             both `read` and the condition probe, which is why it shows up on \
             both.",
        mode: FailureMode::Assertion,
        // The widening is the whole defect, and the rule that catches it is the
        // only place an author meets it. `read_ok` already owns the error half
        // one layer down, so the assertion in
        // `query_matching_nothing_yields_empty` can only ever fire on a store
        // that returned *events* — this pin is what holds its message to that.
        expect: &[(
            "query_matching_nothing_yields_empty",
            "must yield no events",
        )],
    },
    Declared {
        name: "TagJoinFanOutStore",
        kind: Kind::Mutant,
        fails: &[
            "duplicate_items_do_not_duplicate_events",
            // The same defect at the tag-count edge, and unavoidable rather than
            // incidental: VT-22's rule reads a sixty-four-tag event, and the only
            // two ways to read it are `Query::all()`, where this store fans it out
            // sixty-four times, and a tag-constrained query, where
            // `ExactTagMatchReadStore` drops it instead. One of the two has to be
            // declared; this is the one whose failure is the loudest and whose
            // provenance names the same schema.
            "store_accepts_the_guaranteed_minimum_tag_count",
            // And unavoidable for the same reason one clause over: VT-17's rule
            // is about two values under one key, so its event carries three tags
            // by construction and this store returns three copies of it. There is
            // no seeding that both exercises a repeated key and gives this store
            // one tag to fan out on.
            "tags_may_repeat_a_key",
        ],
        provenance: "`InnerJoinTagStore`'s sibling in the same two-table schema \
             (`crates/happenstance-sqlite/src/event_store.rs`): the join that reaches the \
             tag rows has no `DISTINCT`, so an event carrying three tags comes back three \
             times. The tag-AND path has to be written as `GROUP BY … HAVING COUNT(*) = n` \
             and the grouping collapses the duplicates for free, which is why the defect \
             shows only where no item constrains tags — `Query::all()`, the query every \
             projection runner starts from.",
        mode: FailureMode::Assertion,
        // The rule reads the multi-tagged event two ways — through `Query::all()`
        // and through a two-item query — and the fan-out is only visible on the
        // first. Pinned so it stays the one that fires.
        expect: &[(
            "duplicate_items_do_not_duplicate_events",
            "must be yielded once, not once per",
        )],
    },
    Declared {
        name: "ItemOrderedUnionStore",
        kind: Kind::Mutant,
        fails: &[
            "query_item_order_does_not_change_the_result_set",
            "read_from_composes_with_multi_item_query",
        ],
        provenance: "one statement per `QueryItem`, unioned client-side and deduplicated but \
             never merge-sorted — the same shape ES-12 rejects for an unrelated reason, and \
             the natural one for an adapter with no server-side cursor. Note what it is not: \
             ES-15's own `Rejects:` names an adapter that sorts and deduplicates a query's \
             *items*, and no order-invariance rule can catch that, because sorting them is \
             precisely what makes their order stop mattering. `ItemDedupByTypeStore` below \
             is that adapter, and `query_union_is_item_concatenation` is the match-set rule \
             that finally sees it.",
        mode: FailureMode::Assertion,
        expect: &[],
    },
    Declared {
        name: "ItemDedupByTypeStore",
        kind: Kind::Mutant,
        fails: &["query_union_is_item_concatenation"],
        provenance: "the optimisation ES-15's `Rejects:` names and that nothing caught until \
             VT-31's rule landed. `QueryItem::new` already sorts and deduplicates \
             *types*, so grouping the items by their type set and emitting one \
             `type_id IN (...)` clause per distinct set looks like the same move — \
             and the tag half of every swallowed item goes with it, so the query \
             selects a SMALLER set than the caller asked for. \
             `query_item_order_does_not_change_the_result_set` cannot see it: \
             deduplicating the items is precisely what makes their order stop \
             mattering.",
        mode: FailureMode::Assertion,
        expect: &[(
            "query_union_is_item_concatenation",
            "must be the union of their match sets",
        )],
    },
    // --- Read options ---------------------------------------------------
    Declared {
        name: "SortByEventTypeStore",
        kind: Kind::Mutant,
        fails: &[
            "query_all_matches_every_event",
            "query_items_are_or",
            "read_backwards_reverses_order",
            "read_limit_truncates",
            "read_defaults_to_ascending_order",
            "read_from_composes_with_multi_item_query",
            "positions_are_strictly_monotonic",
        ],
        provenance: "a covering index on `(type, position)` is the first index anyone adds, \
             the planner will use it, and an index scan with no outer sort \
             returns its own order. Position is still the tiebreak, so the result \
             looks perfectly ordered — by the wrong key.",
        mode: FailureMode::Assertion,
        expect: &[],
    },
    Declared {
        name: "FromIsAnOffsetStore",
        kind: Kind::Mutant,
        fails: &[
            "read_from_is_inclusive",
            "read_backwards_from_with_limit",
            "read_from_composes_with_multi_item_query",
            // Skipping by a COUNT slides the closed window down by the lower
            // bound's numeric value, and answers a `from` above the head by
            // skipping past every row rather than by yielding what is below it.
            // The two `to`-only rules leave `from` unset, so it passes those.
            "read_from_and_to_bound_a_closed_window",
            "read_from_a_gap_position",
            // The budgeted resume too, one composition over: an `OFFSET` of the
            // cursor's numeric value starts the page at the wrong event, so what
            // comes back is the wrong three events rather than too many.
            "read_from_composes_with_limit",
        ],
        provenance: "a position anchor read as an index. `OFFSET` is the parameter already \
             in the paging query, and a `u64` position slots into it without \
             complaint.",
        mode: FailureMode::Assertion,
        expect: &[],
    },
    Declared {
        name: "BackwardsIgnoredStore",
        kind: Kind::Mutant,
        fails: &[
            "read_backwards_reverses_order",
            "read_backwards_from_with_limit",
            "read_backwards_limit_applies_after_filtering",
            "read_from_composes_with_multi_item_query",
            // Every rule that reads backwards and asserts on the result. Forced
            // forwards, the `to` rule returns the three oldest rather than the
            // three newest, the budget rule's mirror the first four rather than
            // the last four, and the gap rule's backwards read turns
            // `from(beyond)` into a floor and comes back empty.
            // `read_limit_zero_yields_nothing`'s backwards read is expected
            // empty either way, so it passes that one.
            "read_to_under_backwards_bounds_the_older_end",
            "limit_applies_across_items_not_per_item",
            "read_from_a_gap_position",
        ],
        provenance: "the direction is a `bool` in one struct and a hard-coded `ASC` in the \
             string another struct builds, so it reaches the query builder and \
             never reaches the SQL.",
        mode: FailureMode::Assertion,
        expect: &[],
    },
    Declared {
        name: "FetchOneExtraStore",
        kind: Kind::Mutant,
        fails: &[
            "read_limit_truncates",
            "read_backwards_from_with_limit",
            "read_limit_applies_after_filtering",
            "read_backwards_limit_applies_after_filtering",
            "read_from_composes_with_multi_item_query",
            // `LIMIT n + 1` with n = 0 returns one event where none was asked
            // for, and with n = 4 returns five. The zero rule is worth two
            // rejecters: reading zero as unlimited and being off by one are
            // different defects, and only this one shows the budget is present
            // but wrong.
            "read_limit_zero_yields_nothing",
            "limit_applies_across_items_not_per_item",
            // Off by one on the budgeted resume as well. Inflation on the axis
            // `read_limit_truncates` already owns: L1-1's rule is about a budget
            // that is never spent, not about one spent a row late.
            "read_from_composes_with_limit",
            // And off by one inside a window, for the same reason once more: a
            // budget spent a row late is spent a row late under every bound.
            // What that rule owns alone is the budget dropped entirely because
            // a bound was present.
            "read_to_composes_with_limit",
        ],
        provenance: "ubiquitous: every cursor-paging implementation fetches `LIMIT n + 1` to \
             answer \"is there more\", and most of them trim. This one forgot, \
             which is a one-line omission in the mapping rather than in the query.",
        mode: FailureMode::Assertion,
        expect: &[],
    },
    Declared {
        name: "ToBoundIgnoredStore",
        kind: Kind::Mutant,
        fails: &[
            "read_to_is_inclusive",
            "read_from_and_to_bound_a_closed_window",
            "read_to_under_backwards_bounds_the_older_end",
            // And the multi-item rule, which is not inflation but the honest
            // shape of an ignored field: a bound nobody applies is wrong under
            // every query. What that rule owns alone is the store that applies
            // it to one disjunct.
            "read_to_composes_with_multi_item_query",
        ],
        provenance: "the shape every `#[non_exhaustive]` options struct invites, and the one \
             ES-16's `Rejects:` names first: an adapter written before `to` existed \
             copies the fields it recognises into its own query-builder struct, \
             compiles unchanged when a field is added because `ReadOptions` is \
             passed by value, and silently returns everything above the bound. A \
             backfill worker given [1, H] reads to the end of the log and the tail \
             worker beside it processes the overlap twice, with no error anywhere.",
        mode: FailureMode::Assertion,
        expect: &[],
    },
    Declared {
        name: "ToIsExclusiveStore",
        kind: Kind::Mutant,
        fails: &[
            "read_to_is_inclusive",
            "read_from_and_to_bound_a_closed_window",
            "read_to_under_backwards_bounds_the_older_end",
            // For `ToBoundIgnoredStore`'s reason: an off-by-one on the bound is
            // an off-by-one under every query shape, and the window this rule
            // reads ends on an event the store drops.
            "read_to_composes_with_multi_item_query",
        ],
        provenance: "the other half of ES-16's `Rejects:`. `WHERE position < ?` is the \
             defensible reading of an upper bound in half the APIs anyone has used, \
             and it costs one event per chunk boundary — invisible until the chunks \
             are reassembled, and then a projection missing one event per page for \
             the whole backfill.",
        mode: FailureMode::Assertion,
        expect: &[],
    },
    Declared {
        name: "BackwardsToIsAnUpperBoundStore",
        kind: Kind::Mutant,
        fails: &["read_to_under_backwards_bounds_the_older_end"],
        provenance: "`WHERE position <= ?` copied verbatim into the backwards branch — the \
             same mistake ES-8's `Rejects:` already describes for `from`, one bound \
             over. It is CORRECT reading forwards, which is what makes it \
             survivable: every forward `to` rule passes and a backwards read comes \
             back with the oldest events instead of the newest.",
        mode: FailureMode::Assertion,
        // The one row in the table that pins its rule alone, and it is why that
        // rule exists rather than being a third assertion in
        // `read_to_is_inclusive`.
        expect: &[(
            "read_to_under_backwards_bounds_the_older_end",
            "`to` is the STOPPING bound",
        )],
    },
    Declared {
        name: "LimitZeroIsUnlimitedStore",
        kind: Kind::Mutant,
        fails: &["read_limit_zero_yields_nothing"],
        provenance: "the DCB reference implementation's `if (limit)` guard, which is a \
             coherent reading of `0` in a language where `0` is falsy. Ported to a \
             language where it is not, it is `NonZeroUsize::new(limit)` — which is \
             what `happenstance-core` itself stored until phase 4, so this is the \
             crate's own former behaviour rather than an invented one. The caller \
             it breaks is the one who computed the zero: `.limit(budget - fetched)` \
             at parity reads the entire log.",
        mode: FailureMode::Assertion,
        expect: &[("read_limit_zero_yields_nothing", "must yield nothing")],
    },
    Declared {
        name: "ForwardPagingBudgetStore",
        kind: Kind::Mutant,
        fails: &["read_from_composes_with_limit"],
        provenance: "a forward read that branches on `from` and never threads `limit` into the \
             resume branch — the shape that arises when the cursor is added to a paging \
             query written before it, which is the order every SQL adapter in this \
             workspace will be written in. The backwards branch is left correct, so \
             `read_backwards_from_with_limit` passes. The caller it breaks is a projection \
             resuming from its checkpoint with a budget: it asks for five hundred events \
             and is handed the whole stream, with no error anywhere.",
        mode: FailureMode::Assertion,
        expect: &[],
    },
    Declared {
        name: "WindowedPagingBudgetStore",
        kind: Kind::Mutant,
        // One entry, and one is the whole claim. This store passed all
        // ninety-two rules that preceded `read_to_composes_with_limit`,
        // measured with an empty list rather than argued: the bound's own rules
        // issue no budget, the budget's own rules issue no bound, and a budget
        // smaller than the window is the one arrangement in which neither can
        // stand in for the other.
        fails: &["read_to_composes_with_limit"],
        provenance: "`ForwardPagingBudgetStore`'s defect one read option over. A closed window is              a different statement from a page — `BETWEEN ? AND ?` rather than `LIMIT ?`              — and the budget was threaded into the one that had a paging clause              already. The argument that writes it is that the window is the bound that              matters and the budget is redundant, which is true exactly when the budget              is larger than the window and false on every call a backfill worker makes              but its last. It hands back the whole window to a caller who asked for a              page, with no error anywhere.",
        mode: FailureMode::Assertion,
        expect: &[(
            "read_to_composes_with_limit",
            "the smaller of them is what the caller gets",
        )],
    },
    Declared {
        name: "LimitBeforeFilterStore",
        kind: Kind::Mutant,
        fails: &[
            "read_limit_applies_after_filtering",
            "read_backwards_limit_applies_after_filtering",
            "read_from_composes_with_multi_item_query",
        ],
        provenance: "**a reviewer measured this passing the suite as it stood.** `SELECT … \
             FROM events WHERE position >= ? ORDER BY position LIMIT n`, with the query's \
             own predicate applied in the application to the rows that come back. The tag \
             join is the expensive half, so moving it out of the statement reads as an \
             optimisation rather than a semantic change — and the read returns fewer than \
             *n* matches, sometimes none, while a caller reading \"no more events\" stops.",
        mode: FailureMode::Assertion,
        expect: &[],
    },
    Declared {
        name: "LimitPerItemStore",
        kind: Kind::Mutant,
        fails: &[
            "limit_applies_across_items_not_per_item",
            // Not inflation: that rule's third read is `backwards().limit(2)`
            // over a two-item query, which is this defect exactly. It fails
            // there at the `limit` assertion rather than at the precedence one,
            // which is what `UnparenthesisedPredicateStore`'s row protects.
            "read_from_composes_with_multi_item_query",
            // Not inflation either: L1-1's rule pages a window that STRADDLES
            // the two items' blocks, which is the one shape a per-item budget
            // cannot produce. It spends three rows on each item and returns five.
            "read_from_composes_with_limit",
        ],
        provenance: "an adapter that cannot express a disjunction in one statement emits one \
             per `QueryItem`, and the row budget goes onto each of them because \
             that is where the paging clause is written. The union is then \
             merge-sorted correctly, so every ordering rule passes, and a caller \
             who asked for four events gets `n x items`. ES-14's `Rejects:` names \
             it, and it is the same adapter shape ES-12 rejects failing for an \
             independent reason.",
        mode: FailureMode::Assertion,
        expect: &[(
            "limit_applies_across_items_not_per_item",
            "not four per item",
        )],
    },
    Declared {
        name: "UnparenthesisedPredicateStore",
        kind: Kind::Mutant,
        fails: &[
            "read_from_composes_with_multi_item_query",
            // And L1-1's rule, which is the same predicate with a budget on it:
            // the cursor binds to the `Right` item alone, the whole `Left` block
            // survives it, and the first three rows of the page are three events
            // the caller had already checkpointed.
            "read_from_composes_with_limit",
            // And the `to` twin's rule, because this store dangles **both**
            // bounds off the last disjunct rather than only the upper one. That
            // is what keeps it distinct from `UnparenthesisedToPredicateStore`
            // in the other direction: this one is strictly the worse store, and
            // a strictly worse store failing a superset of the rules is the
            // registry working.
            "read_to_composes_with_multi_item_query",
        ],
        provenance: "`WHERE a OR b AND position >= ?` — the textbook operator-precedence bug, \
             reached by string-concatenating a cursor clause onto a disjunction someone else \
             built. It is invisible to a single-item query, which is every read-option rule \
             the suite had, and it re-delivers already-checkpointed events to a resuming \
             projection forever with no error anywhere.",
        mode: FailureMode::Assertion,
        // The rule makes three assertions — `from` bounding every item, the
        // `backwards` ordering of the merged set, `limit` truncating it — and
        // eight other mutants fail it for unrelated reasons. This is the one
        // that names the precedence bug.
        expect: &[(
            "read_from_composes_with_multi_item_query",
            "must bound EVERY item of the query",
        )],
    },
    Declared {
        name: "UnparenthesisedToPredicateStore",
        kind: Kind::Mutant,
        // One entry, and the entry is this row's whole content. It was
        // `Kind::ModelOnlyMutant` with an empty list, because the three `to`
        // rules all issued `Query::all()` and with no items there is nothing
        // for the `OR` to bind wrongly across — so the only thing in the tree that
        // could see this store was the model family, which is `proptest`-gated
        // and `cfg(not(target_arch = "wasm32"))`. A Cloudflare or Neon adapter
        // carrying this precedence bug passed every rule it actually runs.
        // `read_to_composes_with_multi_item_query` is the rule that closes it:
        // CF-12's shape, one bound over.
        fails: &["read_to_composes_with_multi_item_query"],
        provenance: "`WHERE a OR b AND position <= ?` — `UnparenthesisedPredicateStore` one bound \
             over, and reached the same way: the window's top appended to a `WHERE` string that \
             already carries a disjunction someone else built. The caller it breaks is a bounded \
             backfill worker owning `[1, H]` while a tail worker owns everything above it: the \
             backfill reads past its own window and re-delivers events the tail worker has \
             already processed, with no error anywhere.",
        mode: FailureMode::Assertion,
        // The rule makes one assertion, and other mutants fail it from other
        // directions — an ignored bound, an exclusive one. This is the row that
        // names the precedence bug.
        expect: &[(
            "read_to_composes_with_multi_item_query",
            "must bound EVERY item of the query",
        )],
    },
    Declared {
        name: "NullHeadPagingStore",
        kind: Kind::Mutant,
        fails: &[
            "two_fixture_instances_observe_none_of_each_others_appends",
            "reading_an_empty_store_yields_nothing",
        ],
        provenance: "ES-11 *prescribes* this shape for a paginating adapter — capture the head \
             at the first poll and bound every page by `position <= H` — so it is the \
             recommended implementation with one column type wrong. `max()` over no rows is \
             `NULL`, and `query_scalar!` decoding it into an `i64` is a decode error rather \
             than a zero — which is why it is modelled as an `Err` item on the stream and \
             not as a panic: the rule must reject it, not survive it. It fails on a store \
             whose only fault is being new, which is the state every adapter is in on its \
             first run.",
        mode: FailureMode::Assertion,
        // Pinned to the *error* surface rather than to either emptiness
        // assertion, because that is honestly what this store demonstrates:
        // ES-9's "and MUST NOT error" half. The "yields the wrong events" half
        // has no plausible saboteur — nobody ships a store that invents rows
        // for a log nothing has been written to — and ES-9's clause says so
        // rather than leaving the asymmetry to be rediscovered.
        expect: &[(
            "reading_an_empty_store_yields_nothing",
            "read should succeed",
        )],
    },
    // --- Append ---------------------------------------------------------
    Declared {
        name: "SharedBatchPositionStore",
        kind: Kind::Mutant,
        fails: &[
            "read_from_is_inclusive",
            "read_backwards_from_with_limit",
            "read_defaults_to_ascending_order",
            "positions_are_unique",
            "positions_are_strictly_monotonic",
            "event_ids_are_unique_within_a_store",
            "appending_equal_events_yields_two_events",
            // The same defect seen as ordering rather than as duplication: one
            // position bound for every row of a multi-row insert is not
            // *ascending*, and "strictly" is the word in ES-19 that rejects it.
            "batch_positions_follow_slice_order",
        ],
        provenance: "`INSERT … VALUES (?1,…), (?1,…)` — correct when `?1` is `nextval()` and \
             wrong the moment the value is precomputed in Rust and bound once. \
             The single-event appends every other rule makes cannot see it. One \
             position per statement is also one *identity* per statement, since a \
             correct store mints `(own StoreId, the position it assigned)`.",
        mode: FailureMode::Assertion,
        // The two identity rules are inflation rather than coverage and the pins
        // say so: this store fails `appending_equal_events_yields_two_events` at
        // the *positions* assertion, one line above the identity one the rule is
        // named for, and it reaches `event_ids_are_unique_within_a_store`'s
        // conclusion only because a shared position is a shared identity.
        // `SharedBatchIdentityStore` is the entry that covers the identity column
        // with the positions left correct.
        expect: &[
            (
                "appending_equal_events_yields_two_events",
                "they must occupy two distinct positions",
            ),
            (
                "event_ids_are_unique_within_a_store",
                "a store holds at most one event per `EventId`",
            ),
        ],
    },
    Declared {
        name: "ReturnsFirstOfBatchStore",
        kind: Kind::Mutant,
        fails: &["append_returns_last_written_position"],
        provenance: "`INSERT … RETURNING position` plus `fetch_one`. Postgres returns rows \
             in insertion order and `fetch_one` takes the first, so the bug is \
             invisible for every single-event batch — which is most of them.",
        mode: FailureMode::Assertion,
        expect: &[],
    },
    Declared {
        name: "ReverseOrderBatchStore",
        kind: Kind::Mutant,
        fails: &[
            // Found by running. That rule seeds `[event("A"), event("B")]` as one
            // batch and anchors on reading back `["A", "B"]`, so a store that
            // writes a batch backwards fails at the anchor rather than at the
            // property. Inflation rather than coverage — the defect is
            // `batch_positions_follow_slice_order`'s, met by an arrangement
            // written for something else.
            "condition_rejection_leaves_store_unchanged",
            // The rule this store is for: ES-19's slice-order sentence, which
            // `append_returns_last_written_position` structurally cannot see.
            "batch_positions_follow_slice_order",
            // Inflation, and intrinsic rather than sloppy: a batch written
            // backwards is a batch read back backwards, so every rule that
            // appends more than one event in one call and cares which one is
            // where fails too. Eight of them do. The registry's own caution about
            // reading this table as a map applies here more than anywhere except
            // `InnerJoinTagStore`: these eight evidence the reordering, and the
            // first entry is the only one that evidences the *clause*.
            "query_all_matches_every_event",
            "query_item_types_are_or",
            "query_item_combines_types_and_tags_with_and",
            "query_items_are_or",
            "read_backwards_reverses_order",
            "read_limit_truncates",
            "read_backwards_from_with_limit",
            "store_accepts_the_guaranteed_minimum_batch_size",
        ],
        provenance: "a multi-row `INSERT` assembled by pushing rows onto a stack and draining \
             it, or one that groups the batch by event type to bind one interned type id per \
             group rather than one per row — the first optimisation anybody makes to a bulk \
             insert, and one where grouping is also reordering. It returns the MAXIMUM \
             position, which is what makes it survivable: on a quiescent store that is the \
             last row in the log whatever order the rows went in, so \
             `append_returns_last_written_position` is satisfied. ES-19's own `Rejects:` \
             names the shape as \"an adapter whose bulk insert returns generated keys in \
             unspecified order\".",
        mode: FailureMode::Assertion,
        expect: &[(
            "batch_positions_follow_slice_order",
            "MUST be assigned in SLICE order",
        )],
    },
    Declared {
        name: "WriteThenCheckStore",
        kind: Kind::Mutant,
        fails: &[
            // Both follow from the one defect below. Writing before deciding puts
            // the batch into the set its own probe reads, so a condition matching
            // the batch's own events self-rejects (ES-21) — and the reissue
            // guarantee's FIRST attempt is refused rather than its second, which
            // is where ADR-0012 §6 predicted the failure. The prediction was made
            // by reading the store rather than by running it; ES-18's own history
            // is the warning about exactly that.
            "batch_is_not_evaluated_against_its_own_condition",
            "reissued_conditional_batch_lands_once",
            "append_is_atomic",
            "condition_rejection_leaves_store_unchanged",
            "racing_conditional_appends_elect_one_winner",
            "interleaved_appends_on_one_handle_elect_one_winner",
        ],
        provenance: "a reviewer measured probe-then-insert outside the transaction passing \
             the suite as it stood. This is its non-concurrent form: autocommit \
             plus a separate probe, so the rows are durable before the answer is \
             known and `Err` is the only thing left to return.",
        mode: FailureMode::Assertion,
        expect: &[],
    },
    Declared {
        name: "EmptyBatchIsANoOpStore",
        kind: Kind::Mutant,
        fails: &[
            "append_rejects_empty_batch",
            "empty_batch_is_refused_before_the_condition_is_evaluated",
        ],
        provenance: "an insert of zero rows is a successful no-op in every driver. Refusing \
             it is a decision the adapter has to *make*, and an adapter that \
             passes the slice straight through has not made it.",
        mode: FailureMode::Assertion,
        expect: &[],
    },
    Declared {
        name: "EmptyBatchPanicsStore",
        kind: Kind::Mutant,
        fails: &[
            "append_rejects_empty_batch",
            "empty_batch_is_refused_before_the_condition_is_evaluated",
        ],
        provenance: "the same missing guard reached from the other side: the returned \
             position is read off the last row of the `RETURNING` set, and an \
             empty set is treated as unreachable. `MemoryEventStore`'s own append \
             is this code plus the guard.",
        mode: FailureMode::StorePanic("INSERT … RETURNING produced no rows"),
        expect: &[],
    },
    Declared {
        name: "DropsMetadataStore",
        kind: Kind::Mutant,
        fails: &[
            "append_preserves_event_payload",
            // The stronger form of `MetadataConflatingStore`'s defect: a store
            // that never writes the column cannot distinguish `None` from
            // `Some(<empty>)` either. Both rows are wanted — the pair is what
            // shows the new rule is about the *distinction* rather than about
            // metadata surviving at all, which the round trip already covers.
            "metadata_distinguishes_absent_from_empty",
        ],
        provenance: "the column was added after the first `INSERT` statement was written, \
             and the statement is the one place a new column has to be repeated. \
             Nothing reads metadata back except a round-trip test.",
        mode: FailureMode::Assertion,
        expect: &[],
    },
    Declared {
        name: "ViolationAsStoreErrorStore",
        kind: Kind::Mutant,
        fails: &[
            "two_handles_observe_each_others_appends",
            "condition_without_after_rejects_any_match",
            "condition_after_rejects_events_beyond_the_boundary",
            "condition_matches_on_tags",
            "condition_matching_every_tag_rejects_the_append",
            "condition_rejection_is_reported_as_condition_violated",
            "racing_conditional_appends_elect_one_winner",
            "interleaved_appends_on_one_handle_elect_one_winner",
        ],
        provenance: "the DCB uniqueness shape implemented as a partial unique index: the \
             database reports `23505 unique_violation` and the adapter passes the \
             driver error straight through. Callers distinguish *retry* from \
             *something broke* on this alone.",
        mode: FailureMode::Assertion,
        expect: &[],
    },
    Declared {
        name: "FirstTagOnlyConditionStore",
        kind: Kind::Mutant,
        fails: &["condition_matches_on_every_tag_in_an_item"],
        provenance: "the shape a conditional-write KV store reaches for by construction:              a condition has to become a KEY there, and a key is one value, so the              adapter keys on `tags[0]` and every other tag narrows nothing. The              boundary the caller drew is then enforced strictly WIDER than they drew              it, and a store that refuses too much looks correct until throughput              matters. ES-27 names the gap in its own text.",
        mode: FailureMode::Assertion,
        expect: &[],
    },
    Declared {
        name: "ConditionBeforeEmptinessStore",
        kind: Kind::Mutant,
        fails: &["empty_batch_is_refused_before_the_condition_is_evaluated"],
        provenance: "**the reference implementation, until this rule landed.** D8: \
             `MemoryEventStore` evaluated the condition before checking that the batch was \
             non-empty, `LocalMemoryEventStore` copied the ordering deliberately and said so \
             in a comment, and this file's own correct core copied it a third time. For \
             every non-empty batch the two orders are indistinguishable, which is how it \
             survived in three places at once; what it costs is `append(&[], Some(&c))` \
             answering `NoEvents` or `ConditionViolated` depending on store contents, and a \
             caller branching on `is_condition_violated()` retrying an empty batch forever.",
        mode: FailureMode::Assertion,
        expect: &[],
    },
    Declared {
        name: "AfterValidatedAgainstHeadStore",
        kind: Kind::Mutant,
        fails: &["condition_after_beyond_head_admits_the_append"],
        provenance: "input validation on an opaque ordering key: an `after` above the store's \
             own head cannot name anything this store issued, so refusing it reads like \
             defensive programming. It is fatal to a peer resuming after a gap, and on any \
             store that allocates in steps every position between two assigned ones is one \
             it never assigned.",
        mode: FailureMode::Assertion,
        expect: &[],
    },
    // --- Append conditions ----------------------------------------------
    Declared {
        name: "ExactTagMatchConditionStore",
        kind: Kind::Mutant,
        fails: &["condition_matches_on_tags"],
        provenance: "`ExactTagMatchReadStore`'s sibling one code path over, and the pairing is \
             CF-9's own argument: the read path and the condition probe are different code. \
             Tags serialised to one canonical column and compared with `=` is what an \
             adapter does to avoid the join, and the probe is where the join costs most — so \
             it is the half that gets the shortcut. A stored event carrying the condition's \
             tags plus one more then does not violate, and the uniqueness guard stops \
             guarding for exactly the entities with the most history.",
        mode: FailureMode::Assertion,
        expect: &[],
    },
    Declared {
        name: "TagBlindConditionStore",
        kind: Kind::Mutant,
        fails: &[
            "condition_matches_on_every_tag_in_an_item",
            "condition_with_an_unheld_tag_does_not_reject",
        ],
        provenance: "**the canonical DCB uniqueness shape, wrong in the direction that looks \
             safe.** The probe drops the tag join and matches on type alone. The join is the \
             expensive half and tags live in a second table in the planned SQLite schema \
             (`crates/happenstance-sqlite/src/event_store.rs`), so dropping it is the natural \
             first cut; the result over-rejects, which reads as caution. Such an adapter \
             rejects every command touching any entity of that type while passing every \
             other rule — a total-availability failure certified as conformant.",
        mode: FailureMode::Assertion,
        expect: &[],
    },
    Declared {
        name: "NullAggregateProbeStore",
        kind: Kind::Mutant,
        fails: &["condition_against_an_empty_store_admits_the_append"],
        provenance: "`SELECT max(position) … > ?` is `NULL > ?` on an empty table, and SQL's \
             three-valued result is *unknown* rather than false — which a `WHERE` discards \
             and a wrapping `NOT` turns into a match, depending on how the predicate is \
             nested. Every other condition rule seeds the store first, so the case this \
             breaks is every adapter's very first conditional append.",
        mode: FailureMode::Assertion,
        expect: &[],
    },
    Declared {
        name: "UncorrelatedProbeStore",
        kind: Kind::Mutant,
        fails: &[
            "condition_after_beyond_the_last_matching_position_admits_the_append",
            // Slice F: the defect this row already states, met in two new
            // places. In the reissue rule the first attempt's own row sits above
            // the boundary and refuses the second; in the guards rule the busy
            // fragment's event sits above the quiet guard's boundary.
            "reissued_batch_conditioned_on_other_events_lands_twice",
            "condition_guards_carry_independent_boundaries",
        ],
        provenance: "the probe written as two subqueries — *does any event match the query* \
             and *is the head above `after`* — ANDed together, which is the obvious \
             decomposition because each half is a statement someone can read. It is correct \
             on every case the rest of the suite exercises, because in all of them the \
             matching event is also the latest one. Where it is wrong is the steady state of \
             a quiet entity in a busy store, and the deployment reads it as contention.",
        mode: FailureMode::Assertion,
        expect: &[],
    },
    Declared {
        name: "AfterDefaultsToFirstStore",
        kind: Kind::Mutant,
        fails: &[
            "condition_matching_every_tag_rejects_the_append",
            "two_handles_observe_each_others_appends",
            "append_is_atomic",
            "condition_without_after_rejects_any_match",
            "condition_matches_on_tags",
            // Both liveness mirrors probe with no `after`, and the event each
            // must find is the first in the log — the one position this defect
            // hides. Anchor coverage rather than headline: neither rule is
            // about `after`, and this store's fault is.
            "condition_with_an_unheld_tag_does_not_reject",
            "condition_against_an_empty_store_admits_the_append",
            "condition_rejection_leaves_store_unchanged",
            "condition_rejection_is_reported_as_condition_violated",
        ],
        provenance: "`condition.after.unwrap_or(FIRST)` — the `Option` collapsed at the \
             boundary because the SQL wanted a value. Every conditional append \
             that names an explicit anchor still behaves, so the defect only \
             shows against the very first event in the log.",
        mode: FailureMode::Assertion,
        expect: &[],
    },
    Declared {
        name: "ExistenceProbeStore",
        kind: Kind::Mutant,
        fails: &[
            "condition_matches_on_every_tag_in_an_item",
            "condition_without_after_allows_non_match",
            "condition_after_ignores_non_matching_events",
            "condition_with_an_unheld_tag_does_not_reject",
            "condition_after_beyond_the_last_matching_position_admits_the_append",
            // Slice F: the defect this row already states, met in two new
            // places. In the reissue rule the first attempt's own row sits above
            // the boundary and refuses the second; in the guards rule the busy
            // fragment's event sits above the quiet guard's boundary.
            "reissued_batch_conditioned_on_other_events_lands_twice",
            "condition_guards_carry_independent_boundaries",
            // And a third: that rule seeds a row above the boundary and then
            // conditions on the batch's own events, so a probe that asks only
            // "does anything exist above `after`" refuses the FIRST attempt.
            // Same defect, and the same reading ADR-0012 §6 got wrong about
            // `WriteThenCheckStore` — the refusal lands on the first attempt
            // rather than on the reissue.
            "reissued_conditional_batch_lands_once",
        ],
        provenance: "`SELECT EXISTS(SELECT 1 FROM events WHERE position > ?)` — the fast \
             path someone adds when the join is the expensive half, and it is \
             right for every condition whose query happens to match everything \
             after the anchor.",
        mode: FailureMode::Assertion,
        expect: &[],
    },
    Declared {
        name: "AfterIsInclusiveStore",
        kind: Kind::Mutant,
        fails: &[
            "condition_after_ignores_events_at_the_boundary",
            "condition_after_beyond_the_last_matching_position_admits_the_append",
        ],
        provenance: "the textbook off-by-one: `after` is exclusive and `from` is inclusive, \
             and one comparison serves both in the first draft.",
        mode: FailureMode::Assertion,
        expect: &[],
    },
    Declared {
        name: "AfterIsAnOffsetStore",
        kind: Kind::Mutant,
        fails: &[
            "condition_after_rejects_events_beyond_the_boundary",
            "racing_conditional_appends_elect_one_winner",
            "interleaved_appends_on_one_handle_elect_one_winner",
        ],
        provenance: "the sibling of `FromIsAnOffsetStore`, one parameter over: a position \
             anchor spent as a row count, in the one place where spending it \
             wrongly admits a write instead of hiding a read.",
        mode: FailureMode::Assertion,
        expect: &[],
    },
    // --- Fixture-level defects ------------------------------------------
    Declared {
        name: "CachedHeadFixture",
        kind: Kind::Mutant,
        fails: &["two_handles_observe_each_others_appends"],
        provenance: "a `max(position)` fast path cached per session — the first of the three \
             adapters the rule names, alongside a per-connection repeatable-read \
             snapshot and an advisory lock scoped to one pool member. Its reads \
             are correct and only its condition probe is stale, which is the \
             realistic shape: the fast path exists because the probe is the \
             expensive half.",
        mode: FailureMode::Assertion,
        // Its reads are correct; only its condition probe is stale, and this is
        // which of the rule's two assertions says so.
        expect: &[(
            "two_handles_observe_each_others_appends",
            "must see the first handle's committed append",
        )],
    },
    Declared {
        name: "LastWrittenHeadStore",
        kind: Kind::Mutant,
        fails: &["head_advances_across_two_handles"],
        provenance: "a handle that answers `head()` from the position its own last `append` \
             returned — a field, a session variable, or Postgres' own `currval()`, which \
             is documented to be session-scoped and is therefore the version of this \
             defect the database hands you ready-made. ES-30's first rejected \
             implementation in terms: `a cached last-written position, which is stale \
             the moment a second handle writes`. It is correct against a single handle, \
             which is every test anybody writes before they have a pool, and it is the \
             second of the two ways one cached head is spent — `CachedHeadFixture` \
             spends it on the condition probe and answers `head` from the log, which is \
             why these are two rows and not one widened one.",
        mode: FailureMode::Assertion,
        // The head relation rather than the ES-33/ES-34 anchor: this store's reads
        // are `correct::select` verbatim, so the observer does see both of the
        // writer's appends. A failure at the anchor would mean the row had stopped
        // evidencing ES-30 and started evidencing ES-34, which `CachedHeadFixture`
        // already owns.
        expect: &[(
            "head_advances_across_two_handles",
            "must be the highest position currently visible",
        )],
    },
    Declared {
        name: "LosingFixture",
        kind: Kind::Mutant,
        fails: &[
            "acknowledged_writes_survive_a_reopen",
            "recorded_time_survives_a_reopen",
            "reopened_store_does_not_reissue_an_event_id",
        ],
        provenance: "an append acknowledged before its COMMIT returns: a pooled store \
             answering on a `spawn_blocking` join, a Durable Object relying on \
             output-gate semantics it does not have, any connection setup \
             carrying `PRAGMA synchronous = OFF`. Read from the identity side it \
             is also the restored-backup shape VT-6 names: the incarnation \
             survives the reopen and the position counter does not, so the next \
             append reissues a pair.",
        mode: FailureMode::Assertion,
        // Three rules, one defect, three different assertions — which is why each
        // is pinned. The first two fail at a *survival* anchor: the events are
        // gone, not merely their positions or their stamps. The third is the
        // interesting one and is not an anchor at all: this store reaches the end
        // of the rule and mints, for a genuinely new event, an identity it has
        // already issued to a different one. That is what a restored backup looks
        // like from the inside, and it is the harm VT-6 spends a clause on.
        expect: &[
            (
                "acknowledged_writes_survive_a_reopen",
                "every event of an acknowledged append must survive a reopen",
            ),
            (
                "recorded_time_survives_a_reopen",
                "the acknowledged event must have survived the reopen at all",
            ),
            (
                "reopened_store_does_not_reissue_an_event_id",
                "must never issue an `(StoreId, SequencePosition)` pair it",
            ),
        ],
    },
    Declared {
        name: "RestampingFixture",
        kind: Kind::Mutant,
        // **One** rule, and the count is the content. `LosingFixture` above
        // fails all three reopen rules and fails `recorded_time_survives_a_reopen`
        // at its *survival* anchor — its events are gone, so the stamp comparison
        // is never reached. Since phase 4 that has left the rule's headline
        // sentence with no negative control at all (`RUNBOOK.md:3205-3207`). This
        // row is that control: it must reach the third assertion and fail there,
        // and it must pass the two survival rules beside it, or it is a second
        // `LosingFixture` wearing a different name.
        fails: &["recorded_time_survives_a_reopen"],
        provenance: "a migration that stores payload, type and tags and no `recorded_at` \
             column, so `open` reconstructs the log by replaying rows and stamping \
             them at open time. It is the first schema somebody writes — the \
             column reads like metadata until an auditor has to answer *when did \
             this happen* out of the log rather than out of a backup — and it is \
             the exact mirror of what a correct migration does: persist the \
             column and read it back, never re-derive it. Every event still \
             returns, at its own position, under its own identity; the one clock \
             reading whose provenance the log itself attested is what is gone.",
        mode: FailureMode::Assertion,
        // The pin is the whole point of this row. Quoting the *headline* message
        // is what makes a failure at the survival anchor — which is where the
        // only other candidate for this rule dies — get reported as the wrong
        // failure rather than counted as a pass.
        expect: &[(
            "recorded_time_survives_a_reopen",
            "a `RecordedAt` is persisted alongside the event, not recomputed when",
        )],
    },
    // --- Head -------------------------------------------------------------
    Declared {
        name: "EmptyHeadIsFirstStore",
        kind: Kind::Mutant,
        fails: &["head_of_an_empty_store_is_none"],
        provenance: "`SELECT IFNULL(MAX(position), 0)`, because `MAX` over no rows is `NULL` and \
             the driver's scalar decode wants a column type that can hold what comes \
             back — then `SequencePosition::new(value).unwrap_or(SequencePosition::FIRST)`, \
             because the newtype forbids zero and this workspace's own house rule \
             forbids `unwrap` in library code. Two reasonable local decisions produce a \
             store that reports a position no event occupies on the one state every \
             adapter is in on its first run. ES-11 prescribes anchoring a paginating \
             read on `head()` at the first poll and ES-31 makes `caught up?` a \
             comparison against it, so the first event ever appended is the one a fresh \
             runner skips.",
        mode: FailureMode::Assertion,
        // The empty-store assertion rather than the rule's anchor. This store's
        // non-empty answer is `correct::head_of` verbatim, so a future edit that
        // tripped the anchor instead would be a different defect wearing this row.
        expect: &[(
            "head_of_an_empty_store_is_none",
            "has no highest visible position",
        )],
    },
    Declared {
        name: "DefaultQueryHeadStore",
        kind: Kind::Mutant,
        fails: &["head_is_the_highest_visible_position"],
        provenance: "`SELECT max(e.position) FROM event e JOIN tag t ON t.event_id = e.id` — the \
             head statement written against the same joined view the read path is built \
             around, in the same two-table schema `InnerJoinTagStore` and \
             `TagJoinFanOutStore` model from the read side. There is one view in the \
             adapter and reusing it is the obvious move; an untagged row has nothing on \
             the other side of the join, so the store's head stops at the highest \
             *tagged* position. It is ES-30's second rejected implementation — a head \
             reporting the highest position matching some default query rather than the \
             store's head — and it is invisible to any rule that appends one uniform \
             batch, which is why the rule seeds an event the narrower query cannot match.",
        mode: FailureMode::Assertion,
        // The head relation rather than the anchor: this store's *reads* are
        // `correct::select` verbatim, so the untagged event does come back and the
        // anchor holds. Tripping the anchor would mean the store had acquired a
        // read-side defect it does not declare.
        expect: &[(
            "head_is_the_highest_visible_position",
            "must be the highest position currently visible",
        )],
    },
    // --- Identity, recorded time and membership -------------------------
    Declared {
        name: "NormalisingTagStore",
        kind: Kind::Mutant,
        fails: &["tags_differing_only_by_unicode_normalisation_are_distinct"],
        provenance: "`CREATE COLLATION … (provider = icu, deterministic = false)` on the tag \
             column — one line, and exactly what an author reaches for when a search \
             stops matching an accented word — or a normaliser called in the row mapper \
             because `tags should be canonical`. Either rewrites a caller's identifier \
             on the way in, so the tag read back is not the tag written, ADR-0003's \
             byte-for-byte forwarding promise is broken, and the store's index disagrees \
             with every external system holding the original string. A macOS client \
             writes `café` in NFD and a Linux client writes it in NFC; under this store \
             they stop being two consistency boundaries with no visible cue anywhere. \
             VT-15's own `Rejects:` names it.",
        mode: FailureMode::Assertion,
        // The selective read, not the round trip: both rows survive a normalising
        // column, and what is lost is that they are distinguishable. A pin on the
        // round trip would belong to `TrimmingIdentifierStore`.
        expect: &[(
            "tags_differing_only_by_unicode_normalisation_are_distinct",
            "must select the event written with it and no other",
        )],
    },
    Declared {
        name: "KeyedTagMapStore",
        kind: Kind::Mutant,
        fails: &["tags_may_repeat_a_key"],
        provenance: "tags stored as a map: a `JSONB` object, a `HashMap<String, String>` column, \
             or a side table under `UNIQUE (event_id, key)` written with `ON CONFLICT \
             (event_id, key) DO UPDATE`. All three are natural schemas for something \
             the contract itself invites you to read as a pair — `Tag::key` and \
             `Tag::value` are public — and all three keep one value per key. \
             `Tags::from_pairs([(\"tenant\", \"a\"), (\"tenant\", \"b\")])` is a *two*-element \
             set, because deduplication is on the whole `key:value` string \
             (`tag.rs:258-265`). VT-17's own `Rejects:` names it, and on Wattline's \
             4,200-tenant shared log it is a cross-tenant correctness failure produced \
             entirely by an indexing choice.",
        mode: FailureMode::Assertion,
        // The round trip, which is where the dropped tag first becomes visible;
        // the two queries below it would fail too, one rule-iteration later.
        expect: &[("tags_may_repeat_a_key", "must round-trip with both of them")],
    },
    Declared {
        name: "TrimmingIdentifierStore",
        kind: Kind::Mutant,
        fails: &["append_preserves_event_type_and_tags_byte_for_byte"],
        provenance: "`TRIM()` in the insert statement, or `value.trim()` in the row mapper, \
             because a trailing space in an identifier `must be a typo`. It is \
             `NarrowIdentifierColumnStore`'s and `Latin1IdentifierStore`'s third sibling \
             — one column, three ways for it to rewrite what it was given — and the one \
             with the worst consequence, because it moves the decision about what an \
             identifier *is* from the application into the store. Kestrel Rotor \
             replicated `turbine:HW2-A14 ` with a trailing space; `Tag::new` accepts it, \
             `contains_all` is a strict merge-scan on equality (`tag.rs:231-245`) that \
             does not match the unpadded tag, and a lot-recall query silently missed a \
             turbine. VT-15 makes that the application's bug to prevent; an adapter that \
             `fixes` it produces a different wrong answer and hides the first one.",
        mode: FailureMode::Assertion,
        expect: &[(
            "append_preserves_event_type_and_tags_byte_for_byte",
            "stores a value that is not the one the caller wrote",
        )],
    },
    Declared {
        name: "RowOrdinalIdentityStore",
        kind: Kind::Mutant,
        fails: &["append_stamps_identity_and_time"],
        provenance: "an adapter with no identity column, synthesising an `EventId` in its \
             row mapper from the row's ordinal in the result set — an \
             `.enumerate()` inside `rows.map(…)`, which is what is written when \
             `EventId` arrives on `SequencedEvent` after the table exists and \
             nobody wants a migration. Under `Query::all()` on a dense store the \
             synthesised value is right, so every rule that reads the whole log \
             back agrees with it.",
        mode: FailureMode::Assertion,
        // The rule's third assertion, not either anchor: this store returns
        // every event and returns them correctly, and is wrong only about what
        // one of them is *called* when reached by a narrow query.
        expect: &[(
            "append_stamps_identity_and_time",
            "reaching one event two ways must produce one value",
        )],
    },
    Declared {
        name: "ReadTimeClockStore",
        kind: Kind::Mutant,
        fails: &[
            "append_stamps_identity_and_time",
            "append_stamps_a_recorded_time",
            // Inflation rather than coverage, and worth stating because it was
            // found by running rather than by reading. That rule compares two
            // whole `SequencedEvent` lists for equality — ES-18's word is
            // "byte-identical" — rather than going through `snapshot_of`, so a
            // store whose reads are not repeatable fails it whatever the append
            // did. `append_stamps_a_recorded_time` is the rule that owns this
            // defect; the rejection here is a consequence of the rule being
            // stronger than its name, not a second defect.
            //
            // `snapshot_of`'s own doc comment anticipated the opposite outcome:
            // it excludes `recorded_at` on the grounds that the field "would make
            // two reads of an unchanged store differ". For a *conformant* store
            // it does not — the value is persisted — so the stricter comparison
            // is sound, and this row is the evidence.
            "condition_rejection_leaves_store_unchanged",
        ],
        provenance: "an adapter with no `recorded_at` column, filling the field in its row \
             mapper from the connection's clock. `RecordedAt` arrives on the port \
             after the schema is written, the field has to be given something, \
             and `now()` is the value already in scope. Correct in every other \
             respect and invisible to any rule that reads once.",
        mode: FailureMode::Assertion,
        expect: &[
            (
                "append_stamps_a_recorded_time",
                "fixed at append and reported unchanged afterwards",
            ),
            (
                "append_stamps_identity_and_time",
                "reaching one event two ways must produce one value",
            ),
            (
                "condition_rejection_leaves_store_unchanged",
                "must leave the store byte-identical",
            ),
        ],
    },
    Declared {
        name: "ContentHashIdentityStore",
        kind: Kind::Mutant,
        fails: &[
            "appending_equal_events_yields_two_events",
            "append_stamps_a_local_event_id",
        ],
        provenance: "content-addressed identity, which is what anyone reaching for idempotent \
             ingest proposes first and is exactly what VT-8 forbids by making \
             uniqueness the store's obligation rather than the caller's. Its \
             identities are unique across distinct events, stable across a \
             reopen and never reissued, so nothing but a repeated payload sees \
             it.",
        mode: FailureMode::Assertion,
        expect: &[
            (
                "appending_equal_events_yields_two_events",
                "carry two distinct identities",
            ),
            (
                "append_stamps_a_local_event_id",
                "`id.position()` and `position` are the same number here",
            ),
        ],
    },
    Declared {
        name: "PerEventStoreIdStore",
        kind: Kind::Mutant,
        fails: &["append_stamps_a_local_event_id"],
        provenance: "an adapter reading VT-6 as \"mint often, never reissue\" and minting a \
             fresh incarnation per *append* rather than per open. It satisfies \
             the clause's literal MUST — no pair is ever issued twice — and is \
             the defect the replaced `store_id_is_stable_across_reopen` used to \
             catch as a side effect of demanding stability.",
        mode: FailureMode::Assertion,
        expect: &[(
            "append_stamps_a_local_event_id",
            "every event a store accepts in one open carries that store's own",
        )],
    },
    Declared {
        name: "SharedBatchIdentityStore",
        kind: Kind::Mutant,
        fails: &[
            "event_ids_are_unique_within_a_store",
            "append_stamps_a_local_event_id",
            "appending_equal_events_yields_two_events",
        ],
        provenance: "`INSERT … RETURNING event_id` read once and applied to every row — the \
             off-by-a-loop `SharedBatchPositionStore` models on the position \
             column, arriving on the identity column where no rule about \
             positions can see it. The positions are all correct here, so both \
             position rules pass and the store holds two events it cannot tell \
             apart.",
        mode: FailureMode::Assertion,
        expect: &[
            (
                "event_ids_are_unique_within_a_store",
                "a store holds at most one event per `EventId`",
            ),
            (
                "append_stamps_a_local_event_id",
                "`id.position()` and `position` are the same number here",
            ),
            (
                "appending_equal_events_yields_two_events",
                "carry two distinct identities",
            ),
        ],
    },
    Declared {
        name: "IdentityMatchableAsTagStore",
        kind: Kind::Mutant,
        fails: &["event_id_is_not_matchable_by_query"],
        provenance: "the tag-materialised identity VT-7 rejects by name, written the way an \
             adapter would actually write it: the extra tag row is the \
             *adapter's*, not the caller's, so `Event::tags()` round-trips \
             untouched and every payload-fidelity rule still passes. It is what \
             an adapter reaches for to answer `contains_event_id` out of the tag \
             index it already has.",
        mode: FailureMode::Assertion,
        // The probe loop, not the tags assertion: this store's whole premise is
        // that the event's own tags are unchanged.
        expect: &[(
            "event_id_is_not_matchable_by_query",
            "Identity is answered by a dedicated port operation",
        )],
    },
    Declared {
        name: "PositionOnlyMembershipStore",
        kind: Kind::Mutant,
        fails: &["contains_event_id_reports_membership"],
        provenance: "`SELECT 1 FROM events WHERE position = ?` — the natural query for an \
             adapter whose events table has a position column and no origin \
             columns yet, which is every adapter before it implements ingest. It \
             passes every single-store rule in the suite, because a store that \
             has ingested nothing only ever holds its own incarnation.",
        mode: FailureMode::Assertion,
        // The second loop. The first — that it reports what it does hold — is a
        // question this store answers correctly, which is what makes it
        // survivable.
        expect: &[(
            "contains_event_id_reports_membership",
            "which names another incarnation at a position this store happens to have",
        )],
    },
    // --- Position visibility --------------------------------------------
    Declared {
        name: "PollPaddedPositionStore",
        kind: Kind::Mutant,
        // Declared as failing exactly what `PreCommitPositionStore` fails,
        // because its DEFECT is exactly `PreCommitPositionStore`'s: the same
        // sequence advanced outside the same transaction, publishing the same
        // rows late. The only difference is that its `append` needs one more
        // poll. If this claim is wrong, the run says so -- which is the entire
        // point of the instrument.
        fails: &[
            "interleaved_appends_on_one_handle_elect_one_winner",
            "nothing_below_an_observed_position_appears_later",
        ],
        provenance: "The bounding instrument `spec/SPECIFICATION.md` names under ES-10 and              assigns to phase 10: a poll-padding decorator over `PreCommitPositionStore`,              calibrated against a real adapter rather than chosen by an author.              ADR-0013 deferred the calibration because \"an author choosing n is the              reference-store failure mode with one more step\", and phase 10 supplied the              adapter: `happenstance-postgres`'s `append` measures at 3 polls against a live              server (`crates/happenstance-postgres/tests/poll_shape.rs`), and the unpadded              mutant needs 2, so the padding is 1. The rule polls A, B, B, A -- two polls per              future -- so this store's window sits one poll outside the schedule while its              defect is untouched.",
        mode: FailureMode::Assertion,
        expect: &[],
    },
    Declared {
        name: "PreCommitPositionStore",
        kind: Kind::Mutant,
        fails: &[
            "interleaved_appends_on_one_handle_elect_one_winner",
            "nothing_below_an_observed_position_appears_later",
        ],
        provenance: "`nextval()` outside the transaction — **what Postgres does by default**. A \
             writer takes its number, does its work and publishes its row at commit, so a \
             transaction that started earlier and commits later becomes visible *below* a \
             position a reader has already observed. It is the one entry in this table \
             whose bug is a faithful model of a real database rather than an \
             implementation slip: every other mutant here is somebody's mistake, and this \
             one is somebody's database. `happenstance-postgres` sits in the workspace to \
             occupy this end of the position-allocation axis, and §6.5's portfolio table \
             is where the adapter half is still owed.",
        mode: FailureMode::Assertion,
        // Both rules pinned, and the visibility one is why `expect` is keyed by
        // rule at all. This pin came off when the re-entrancy rule landed and
        // the mutant acquired a second declared failure with a different
        // message; the field's shape changed rather than the claim being
        // dropped. Unpinned, a future edit that stopped this store publishing
        // rows at all — or that collapsed `YieldOnce`'s window — would fail
        // `nothing_below_an_observed_position_appears_later` at its closing
        // `seen.len() == 2` anchor, in `suite.rs`, satisfying every check above
        // while the position-visibility claim evaporated. That is CF-2's
        // wrong-reason hazard inside the right rule.
        expect: &[
            (
                "nothing_below_an_observed_position_appears_later",
                "visible underneath it",
            ),
            (
                "interleaved_appends_on_one_handle_elect_one_winner",
                "exactly one must win",
            ),
        ],
    },
    Declared {
        name: "SharedBackingFixture",
        kind: Kind::Mutant,
        fails: &["two_fixture_instances_observe_none_of_each_others_appends"],
        provenance: "a file-backed fixture that points every instance at one temporary path \
             — CF-15's own example. Under the old `Fn() -> S` factory it produced \
             cross-test contamination surfacing as some unrelated rule failing \
             intermittently.",
        mode: FailureMode::Assertion,
        expect: &[],
    },
    // --- Re-entrancy ------------------------------------------------------
    Declared {
        name: "BorrowHoldingStore",
        kind: Kind::Mutant,
        fails: &[
            "a_live_read_stream_does_not_block_an_append",
            // The same defect reached by two more rules rather than three
            // defects: both isolation rules hold a live read stream across an
            // `append`, which is precisely what this store's stream forbids.
            "read_result_is_stable_under_concurrent_append",
            "query_items_share_one_snapshot",
        ],
        provenance: "a stream that yields rows from a live cursor rather than from a \
             snapshot: rusqlite streaming from an open statement, an `Rc`-shared cursor, a \
             pooled adapter that checks a connection out in `read` and returns it in `Drop`. \
             RPITIT lets an implementer return a stream borrowing from `&self`, so the port \
             cannot forbid the shape and only a rule catches it. **A `RefCell` answers a \
             conflicting borrow immediately; a pooled adapter with one connection answers by \
             waiting** — the same defect is a deadlock there, and a hung conformance run \
             names no rule at all, which is why the instrument is a `RefCell`.",
        mode: FailureMode::StorePanic("already borrowed"),
        expect: &[],
    },
    Declared {
        name: "RefetchingPagedStore",
        kind: Kind::Mutant,
        fails: &[
            "read_result_is_stable_under_concurrent_append",
            "query_items_share_one_snapshot",
        ],
        provenance: "`happenstance-neon` reaches Postgres over one-shot HTTP: one buffered \
             JSON document per round trip under a 64 MiB cap, no cursor, and no way \
             to hold a statement open across polls. Above the cap the only way to \
             answer at all is an independent \
             `WHERE position > $last ORDER BY position LIMIT n` per chunk, and each \
             of those is a fresh snapshot. ES-11's prescribed fix is a position \
             ceiling captured no later than the first poll; this is that adapter \
             with the ceiling missing, which is one line rather than a redesign. \
             The page is modelled at one event because a page size is a deployment \
             constant and only the number of events needed to OBSERVE the defect \
             depends on it.",
        mode: FailureMode::Assertion,
        expect: &[(
            "query_items_share_one_snapshot",
            "must be evaluated against the same sample",
        )],
    },
    Declared {
        name: "SwallowedReadFaultStore",
        kind: Kind::Mutant,
        fails: &["arming_a_read_fault_makes_the_stream_yield_an_error"],
        provenance: "`let Ok(page) = fetch().await else { return Poll::Ready(None) };` \
             — a failed page fetch reported as the end of the stream. It is the most \
             natural way to get a fallible fetch past a `poll_next` that must return a \
             value, and both adapters that will need one are already in the tree: \
             `happenstance-cloudflare` over `SqlStorage` and `happenstance-neon` over \
             one-shot HTTP, neither of which can hold a cursor open across polls. The port \
             expresses the failure per item and this store declines to use it. What the \
             consumer sees is a short, SUCCESSFUL read: a projection runner applies two \
             events of five, commits the checkpoint at the truncation point, and the rest \
             are never applied, with `Ok` everywhere and no error to log. Who finds out is \
             whoever reconciles the read model against the log, months later.",
        mode: FailureMode::Assertion,
        expect: &[(
            "arming_a_read_fault_makes_the_stream_yield_an_error",
            "MUST cause the next read's stream to yield an `Err` ITEM",
        )],
    },
    Declared {
        name: "AwaitAcrossBorrowStore",
        kind: Kind::Mutant,
        fails: &[
            "interleaved_appends_on_one_handle_elect_one_winner",
            "nothing_below_an_observed_position_appears_later",
        ],
        // Not `"already borrowed"`, and the reason is `RefCell`'s two messages
        // rather than a weakening: a conflicting `borrow_mut` says "already
        // borrowed" and a conflicting `borrow` says "already mutably borrowed",
        // and this store reaches both — the second `append`'s exclusive borrow
        // under the re-entrancy rule, and the visibility rule's *read* while an
        // append is suspended holding one. `mode` is one field per mutant, so
        // the shared substring is what it can carry; `expect` below is keyed by
        // rule and restores the deleted `#[should_panic]` drivers' precision.
        mode: FailureMode::StorePanic("borrowed"),
        provenance: "the Durable Object shape: take the store, `SqlStorage::exec(..).await`, \
             write. Correct for as long as nothing re-enters the store, which is every \
             sequential test anyone writes. `clippy::await_holding_refcell_ref` catches it \
             statically and fires under this workspace's own `-D warnings`, but it is \
             `warn`-by-default, so a downstream adapter on stock settings gets a warning it \
             can ignore — and the lint does not see `BorrowHoldingStore` at all.",
        expect: &[
            (
                "interleaved_appends_on_one_handle_elect_one_winner",
                "already borrowed",
            ),
            (
                "nothing_below_an_observed_position_appears_later",
                "already mutably borrowed",
            ),
        ],
    },
    Declared {
        name: "PerRowConditionStore",
        kind: Kind::Mutant,
        fails: &["batch_is_not_evaluated_against_its_own_condition"],
        provenance: "the per-row conditional `INSERT … SELECT … WHERE NOT EXISTS`, which the \
             decision ledger carries as a live candidate for the append-condition SQL \
             strategy (`RUNBOOK.md:64`) and which is the only shape `happenstance-neon` can \
             express at all, having no interactive transaction. Carried per row the guard \
             travels with every statement, so the second row is checked against a store that \
             already holds the first — and on the canonical DCB uniqueness shape, where the \
             condition names the very type being written, it self-rejects. It ROLLS BACK, \
             which is the point of the shape rather than a detail: a store that self-rejected \
             and kept its rows would fail `append_is_atomic` instead, and the registry could \
             not then say which of the two defects that rule catches.",
        mode: FailureMode::Assertion,
        expect: &[(
            "batch_is_not_evaluated_against_its_own_condition",
            "a batch can never conflict with itself",
        )],
    },
    Declared {
        name: "PayloadDedupStore",
        kind: Kind::Mutant,
        fails: &[
            // The two rules this store is for: ES-24's stated limits, both of
            // which it strengthens.
            "reissued_unconditional_batch_lands_twice",
            "reissued_batch_conditioned_on_other_events_lands_twice",
            // Inflation, and intrinsic: the suite appends byte-identical events
            // in five places for reasons that have nothing to do with reissue —
            // `seed_hits_between_misses`' three `event("Hit")`s, two
            // `event("Blocker")`s, two `event("Alpha")`s — and a
            // content-addressed store collapses every one of them. That is what a
            // content-addressed store IS; there is no narrower version of this
            // defect.
            "read_limit_applies_after_filtering",
            "read_backwards_limit_applies_after_filtering",
            "read_from_composes_with_multi_item_query",
            "condition_after_beyond_head_admits_the_append",
            "condition_after_beyond_the_last_matching_position_admits_the_append",
        ],
        provenance: "`INSERT … ON CONFLICT (content_hash) DO NOTHING` against a unique index \
             on a hash of `(event_type, tags, data)` — what an adapter built to be safe under \
             at-least-once ingest reaches for, and what a content-addressed store gets for \
             free. It would ship as a feature. Nothing about it looks wrong: no data is lost, \
             no error is hidden, and ES-24's DOCUMENTED at-most-once guarantee still holds. \
             What it does is strengthen the two shapes the clause explicitly disclaims, which \
             is worse than not providing the guarantee at all — callers write retry loops \
             against the adapter they tested on, and the same loop double-charges against the \
             next one.",
        mode: FailureMode::Assertion,
        // Pinned to the two rules the provenance claims it demonstrates, so that
        // the five incidental failures cannot quietly become the whole of what
        // this row evidences.
        expect: &[
            (
                "reissued_unconditional_batch_lands_twice",
                "MUST append a second copy",
            ),
            (
                "reissued_batch_conditioned_on_other_events_lands_twice",
                // The count, not the acceptance sentence above it. This store
                // ACCEPTS the reissue — dedup happens inside the write, not at
                // the condition — so the first assertion passes and the row
                // count is where the collapse becomes visible. Pinned to what
                // actually fires rather than to what reads best.
                "and both copies must be in the store",
            ),
        ],
    },
    Declared {
        name: "MinCollapseStore",
        kind: Kind::Mutant,
        fails: &["condition_guards_carry_independent_boundaries"],
        provenance: "the application-side workaround E2E-05 names, promoted into an adapter: \
             four reads produce four boundaries, one `WHERE position > ?` takes one number, \
             and the safe-looking choice is the smallest. It is SOUND — it never admits an \
             append it should have refused — and it is a liveness failure, which is the \
             harder defect to see: the quiet fragment's stale boundary governs the busy one, \
             so a consistency boundary that never conflicted starts refusing and the \
             deployment reads the rejection rate as physics. It passes the ENTIRE existing \
             `condition_after_*` family, because every rule in it carries a single guard and \
             the minimum over one boundary is that boundary.",
        mode: FailureMode::Assertion,
        expect: &[(
            "condition_guards_carry_independent_boundaries",
            "evaluated against its OWN boundary",
        )],
    },
    Declared {
        name: "SingleGuardFastPathStore",
        kind: Kind::Mutant,
        fails: &[
            "condition_with_one_guard_behaves_as_today",
            // Intrinsic, and the reason `notes.md` records this rule as adding no
            // UNIQUE coverage: every existing append-condition rule carries one
            // guard, so a store wrong on the one-guard path is wrong in all of
            // them. What none of them can see, and what its own rule is for, is
            // that the OTHER path is right — which is the difference between this
            // store and `ExistenceProbeStore`.
            "condition_after_ignores_events_at_the_boundary",
            "condition_after_beyond_head_admits_the_append",
            "condition_after_beyond_the_last_matching_position_admits_the_append",
            "reissued_batch_conditioned_on_other_events_lands_twice",
        ],
        provenance: "`MinCollapseStore`'s mirror image, and the regression VT-30's refactor \
             actually invites. An adapter generalising to N guards keeps a `guards.len() == 1` \
             fast path, because one guard is the overwhelmingly common case and a `UNION` per \
             guard is pure overhead there — and the fast path is the OLD statement, the \
             uniqueness probe written before `after` existed, kept because it was already \
             working. The general path is new and was reviewed; the fast path is old and was \
             not. The result is a store with two answers to one question, and which answer a \
             caller gets depends on how many fragments its decision model happened to read.",
        mode: FailureMode::Assertion,
        expect: &[(
            "condition_with_one_guard_behaves_as_today",
            "boundary ABOVE every matching event",
        )],
    },
    Declared {
        name: "NoTransactionStore",
        kind: Kind::Mutant,
        fails: &["append_is_atomic_under_a_mid_batch_fault"],
        provenance: "`for event in batch { conn.execute(INSERT, …)? }` with the `BEGIN` \
             forgotten — the same shape `RowAtATimeStore` models in the concurrency \
             table, met from the other side. There every row eventually lands and \
             only a *reader* can see the gap; here a row does not land, and rollback \
             is the only thing that could have saved the store. It is correct under \
             `append_is_atomic`, whose rejection comes through the condition path and \
             arms nothing, which is exactly why ES-18 needs both rules.",
        mode: FailureMode::Assertion,
        expect: &[("append_is_atomic_under_a_mid_batch_fault", "byte-identical")],
    },
    Declared {
        name: "YieldingRowAtATimeStore",
        kind: Kind::Mutant,
        fails: &["dropped_append_future_leaves_no_partial_batch"],
        provenance: "`for event in batch { conn.execute(INSERT, …).await? }` — one statement \
             per row, awaited, with the `BEGIN` forgotten. It is `NoTransactionStore` on the \
             other axis and `racers::RowAtATimeStore` on a third, and the three must not be \
             merged: that one needs a fault ARMED and answers `Err` over a partial log, the \
             racer's rows all land in the end and what is wrong is the window a reader sees \
             through, and this one needs no fault at all — the caller simply stops polling. At \
             the edge that is the NORMAL termination path: a client disconnect, a CPU limit, a \
             Durable Object eviction, a pod eviction. The future is destroyed at whatever \
             suspension point it had reached, the rows already written stay written, and there \
             is no `Result` anywhere for anybody to read.",
        mode: FailureMode::Assertion,
        expect: &[(
            "dropped_append_future_leaves_no_partial_batch",
            "fully applied or unchanged",
        )],
    },
    Declared {
        name: "NoopFaultFixture",
        kind: Kind::Mutant,
        fails: &["arming_a_mid_batch_fault_makes_the_append_fail"],
        provenance: "a fixture that declares `MID_BATCH_FAULT` supported and overrides \
             `arm_mid_batch_fault` with an EMPTY BODY, over a completely correct store. Not a \
             forgotten override — that reaches the trait's provided body, which panics and \
             names this exact mistake, and is a different outcome. The author who writes this \
             one is the author of a real adapter who intends to wire a trigger up later, \
             declares the capability while stubbing the method, and gets a green suite in \
             which `append_is_atomic_under_a_mid_batch_fault` appears to have been exercised. \
             Nothing in the build tells them otherwise until CF-39: no fault, `Ok`, every row \
             present, all-or-nothing satisfied.",
        mode: FailureMode::Assertion,
        expect: &[(
            "arming_a_mid_batch_fault_makes_the_append_fail",
            "cause the write of the k-th event to fail",
        )],
    },
    // --- Value edges ---------------------------------------------------
    //
    // Ten stores that are correct for every value the rest of this binary
    // writes. That is what makes them worth having and it is also what makes
    // them cheap to declare: each fails exactly one rule, because the value it
    // is wrong about is written by exactly one.
    //
    // Three of them refuse — `EmptyPayloadIsNullStore`, `PayloadCeilingStore`,
    // `BatchParameterCeilingStore` — and a refusal is caught by `append_ok`, at
    // the rule's *setup*, before the rule body runs. That is a real rejection and
    // the pins below name the exact driver error each produces rather than
    // `append_ok`'s generic message. But it is only half of what each rule
    // asserts: the other half is the round trip, and until stage 6's review
    // nothing in this table reached it. `TruncatingPayloadStore` and
    // `ChunkLosingBatchStore` are the quiet halves of the two ceilings — the same
    // column, the same limit, the engine configured to clip instead of to refuse
    // — so the assertion each rule is *named* for now has an implementation that
    // fails it.
    //
    // `append_preserves_an_empty_payload` has no such sibling and does not need
    // one: `Event::data` is a `Bytes` and never an `Option<Bytes>`, so a store
    // cannot conflate empty with absent on that column the way
    // `MetadataConflatingStore` does on the nullable one, and truncating an empty
    // payload is a no-op. Refusing is the only observable defect, which is why it
    // is the only one compiled.
    Declared {
        name: "EmptyPayloadIsNullStore",
        kind: Kind::Mutant,
        fails: &["append_preserves_an_empty_payload"],
        provenance: "one row-mapper helper — `(!blob.is_empty()).then_some(blob)` — bound to \
             both blob columns, written for `metadata` where the option is genuine. \
             `data` is `NOT NULL`, so a legal event with a zero-length payload meets \
             `NOT NULL constraint failed: event.data`. Nothing in the workspace \
             writes an empty payload today, which is exactly why the mapping looks \
             total: `append_preserves_event_payload` builds a thirteen-byte one.",
        mode: FailureMode::Assertion,
        // Pinned to the driver error rather than to `append_ok`'s generic
        // "unconditional append should succeed": the helper's message is the same
        // for every store that refuses anything, so pinning it would certify
        // "this rule's setup failed" where every sibling row certifies which
        // defect fired. `NotNullViolation` is this store's and no other's.
        expect: &[("append_preserves_an_empty_payload", "NotNullViolation")],
    },
    Declared {
        name: "MetadataConflatingStore",
        kind: Kind::Mutant,
        fails: &["metadata_distinguishes_absent_from_empty"],
        provenance: "the nullable half of the same helper: `metadata` is an `Option<Bytes>` \
             already, so `filter(|m| !m.is_empty())` reads as tidying rather than as \
             losing information — and a driver that maps a zero-length `BLOB` to \
             `NULL` does it without being asked. VT-1 makes `None` and `Some(&[])` \
             two values; this store makes them one.",
        mode: FailureMode::Assertion,
        // Pinned to the *presence* assertion rather than to the zero-length one:
        // this store returns `None` where `Some(<empty>)` was written, which is
        // the first of the two, and a future edit that made it return
        // `Some(b"\0")` instead would be a different defect wearing this row.
        expect: &[(
            "metadata_distinguishes_absent_from_empty",
            "must read back with metadata present",
        )],
    },
    Declared {
        name: "NarrowIdentifierColumnStore",
        kind: Kind::Mutant,
        fails: &["store_accepts_a_max_length_identifier"],
        provenance: "`VARCHAR(64)` for an event type, picked after looking at a domain whose \
             longest type is thirty characters. VT-20 keeps `MAX_EVENT_TYPE_LEN` and \
             `MAX_TAG_LEN` at 255 bytes precisely because they are the width an \
             adapter declares against (`crates/happenstance-core/src/tag.rs:10-14`), and \
             MySQL's non-strict `sql_mode` stores the over-length value truncated \
             with a warning nobody reads.",
        mode: FailureMode::Assertion,
        expect: &[("store_accepts_a_max_length_identifier", "round-trip")],
    },
    Declared {
        name: "Latin1IdentifierStore",
        kind: Kind::Mutant,
        fails: &[
            "store_accepts_non_ascii_identifiers",
            // Not incidental. VT-14's boundary rule uses a multi-byte identifier
            // *at* the limit — which is the case a byte-counting column is most
            // likely to be wrong about — so a store that cannot hold a
            // non-ASCII codepoint fails both rules, for the same reason and at
            // two different edges.
            "store_accepts_a_max_length_identifier",
            // A third edge of the same column, and unavoidable: VT-15's
            // normalisation rule is about two Unicode normal forms, so both of
            // its tags are non-ASCII by construction and a latin-1 column cannot
            // hold either. `NormalisingTagStore` is the entry that covers that
            // rule for the reason it is named for.
            "tags_differing_only_by_unicode_normalisation_are_distinct",
        ],
        provenance: "`VARCHAR(255) CHARACTER SET latin1`, SQL Server's non-`N` `VARCHAR`, or a \
             `CHECK` written against an `[[:ascii:]]` class. The first two do not \
             refuse: the driver transcodes and every codepoint outside the target \
             charset becomes `?`. The crate's own docs invite it — four sites say \
             `EventType` rejects \"ASCII control characters\" when `char::is_control` \
             is Unicode `Cc`, so an adapter author reading them concludes the value \
             space is ASCII.",
        mode: FailureMode::Assertion,
        expect: &[(
            "store_accepts_non_ascii_identifiers",
            "must match itself, and only itself",
        )],
    },
    Declared {
        name: "PayloadCeilingStore",
        kind: Kind::Mutant,
        fails: &[
            "store_accepts_the_guaranteed_minimum_payload",
            // CF-40 gave this store a stated ceiling, so VT-25's rule can reach it:
            // it refuses, but through `AppendError::Store` — the channel VT-25
            // replaced.
            "append_reports_exceeded_store_limits",
        ],
        provenance: "an undocumented row-size ceiling: a payload held inline in a fixed-width \
             column, or a KV backend with a per-value cap the adapter never states. \
             VT-21 permits a store to accept less than it would like — what it \
             forbids is doing so silently and undeclared, and the refusal arriving \
             at write time after the caller has taken its side effects.",
        mode: FailureMode::Assertion,
        expect: &[(
            "store_accepts_the_guaranteed_minimum_payload",
            "ValueTooLarge",
        )],
    },
    Declared {
        name: "TruncatingPayloadStore",
        kind: Kind::Mutant,
        fails: &[
            "store_accepts_the_guaranteed_minimum_payload",
            // CF-40, and the other half of VT-25's MUST: it does not refuse at all.
            // Same column and same number as `PayloadCeilingStore`, so both
            // halves are checked at one boundary.
            "append_reports_exceeded_store_limits",
        ],
        provenance: "the same column as `PayloadCeilingStore` with `MySQL`'s `sql_mode` left \
             non-strict: over-length data is stored clipped and a warning is \
             raised into a log nobody reads. It is the pairing \
             `NarrowIdentifierColumnStore` models one column over, and it is the \
             half of VT-21's MUST — \"refuse rather than truncating\" — that no \
             registered store rejected until stage 6's review, because a refusal \
             is caught at the rule's setup and never reaches the round trip.",
        mode: FailureMode::Assertion,
        // The round-trip half, explicitly: this store's whole content is that it
        // reaches the property and fails *there*.
        expect: &[(
            "store_accepts_the_guaranteed_minimum_payload",
            "a store that truncates rather than refusing",
        )],
    },
    Declared {
        name: "PackedTagColumnStore",
        kind: Kind::Mutant,
        fails: &["store_accepts_the_guaranteed_minimum_tag_count"],
        provenance: "the schema that avoids a side table and a join: the canonical tag list \
             joined into one `VARCHAR(255)`. It is correct for every event anybody \
             looked at — the richest in the six scenarios is Wattline's \
             `SessionStarted` at eight tags — and VT-22 makes sixty-four a floor. \
             What falls off the end is not reported.",
        mode: FailureMode::Assertion,
        expect: &[(
            "store_accepts_the_guaranteed_minimum_tag_count",
            "packs them into one column",
        )],
    },
    Declared {
        name: "ChunkedQueryStore",
        kind: Kind::Mutant,
        fails: &["store_evaluates_a_query_at_the_guaranteed_minimum_item_count"],
        provenance: "one bound parameter per query item meets `SQLITE_MAX_VARIABLE_NUMBER`, or \
             Postgres' 65,535-parameter cap, and chunking is the fix everybody \
             reaches for. Unioning the chunks is a second edit and this is the \
             store where it was not made, so the answer comes back wrong rather \
             than as an error — which is the half VT-23 is about, since it \
             explicitly permits a store to *refuse* a larger query.",
        mode: FailureMode::Assertion,
        // The only value-edge row that carried no pin, and the one that needed it
        // most: this rule raises two assertions in `suite.rs` before the one that
        // matters — `append_ok`'s setup and the 128-item non-vacuity anchor — and
        // `FailureMode::Assertion` only requires the panic to have come from that
        // file. Without the pin the row certified "something in this rule
        // panicked".
        expect: &[(
            "store_evaluates_a_query_at_the_guaranteed_minimum_item_count",
            "only its first chunk",
        )],
    },
    Declared {
        name: "BatchParameterCeilingStore",
        kind: Kind::Mutant,
        fails: &[
            "store_accepts_the_guaranteed_minimum_batch_size",
            // CF-40: the driver's parameter ceiling, stated as the batch ceiling it
            // actually is, and still refused through `AppendError::Store`.
            "append_reports_exceeded_store_limits",
        ],
        provenance: "a multi-row `INSERT` binding one parameter set per event and per tag, \
             meeting its driver's ceiling at write time — after the caller has made \
             its decision and taken its side effects. A hundred events is inside \
             every driver's limit, which is why nobody meets this until a batch \
             gets big; Kestrel Rotor's 1,840-event ingest is the shape that does.",
        mode: FailureMode::Assertion,
        expect: &[(
            "store_accepts_the_guaranteed_minimum_batch_size",
            "TooManyParameters",
        )],
    },
    Declared {
        name: "ChunkLosingBatchStore",
        kind: Kind::Mutant,
        fails: &[
            "store_accepts_the_guaranteed_minimum_batch_size",
            // CF-40. `BatchParameterCeilingStore` with the fix applied wrongly: it
            // clamps rather than refusing, so nothing is reported at all.
            "append_reports_exceeded_store_limits",
        ],
        provenance: "the fix applied after meeting `BatchParameterCeilingStore` in production, \
             applied wrongly: `&events[..CEILING]` where `events.chunks(CEILING)` \
             was meant. Clamping is one character from chunking and needs no loop, \
             no transaction around the loop, and no decision about which chunk's \
             position to return. `Ok` comes back carrying a real position — the \
             last row actually written — so nothing downstream has anything to \
             notice.",
        mode: FailureMode::Assertion,
        expect: &[(
            "store_accepts_the_guaranteed_minimum_batch_size",
            "MUST accept in one append",
        )],
    },
];

/// A deterministic disagreement between a store's own read path and
/// [`crate::correct`]'s — the bar [`Kind::ModelOnlyMutant`] is held to in
/// **every** feature configuration.
///
/// # Why the kind needed one
///
/// [`Kind::Mutant`] is unabusable because its bar is *data in the same table*: a
/// non-empty `fails` list, checked wherever the registry compiles at all. The
/// model-only kind was landed with a bar that borrowed another family instead —
/// `MODEL_COVERAGE` must claim `Rejected` — and a family can be compiled out.
/// An adversarial review demonstrated it: `HidingPlaceStore`, a store with no
/// defect whatever, was registered under this kind with an empty `fails` list
/// and a `MODEL_COVERAGE` row claiming `Rejected`, and
/// `cargo test --no-default-features --test mutation_coverage` reported nine
/// passes. `cargo hack`'s feature powerset builds that configuration, so it was
/// reachable in the real gate.
///
/// This table is the repair, and it is deliberately not a second claim. A claim
/// can be written down falsely; this holds a *scenario* and a **function
/// pointer to the mutant's own `Defect::select`**, and
/// `every_model_only_mutant_demonstrates_its_defect` does the comparing. To
/// satisfy it a store has to actually answer a read differently from the
/// reference implementation, in a run that needs no generator, no runtime and no
/// optional dependency.
///
/// # Why it is not extended to every mutant
///
/// Because for every other kind the rule *is* the witness, and a second one
/// would be a snapshot of what the rule already asserts —
/// `mutants_fail_exactly_their_declared_rules` drives each declared failure and
/// `Declared::expect` pins which assertion fired. The witness exists only where
/// no rule of this family can see the defect, which is exactly the hole the kind
/// names.
/// A `Defect::select`, as a value.
///
/// Named for `clippy::type_complexity`, and it earns the name anyway: it is the
/// signature every mutant's read path has, and writing it once says that the
/// witness holds *the store's own function* rather than a copy of it.
type SelectFn =
    fn(&[SequencedEvent], &Query, ReadOptions) -> Result<Vec<SequencedEvent>, correct::LogError>;

#[derive(Debug)]
struct Witness {
    /// The `REGISTRY` row this witness answers for.
    name: &'static str,
    /// The mutant's own read path, as a function pointer.
    ///
    /// `<T as Defect>::select` rather than a re-implementation, so the thing
    /// measured is the store itself and a witness cannot drift away from the
    /// defect it demonstrates.
    select: SelectFn,
    /// The log to read, the query, and the options — all fixed.
    scenario: fn() -> (Vec<SequencedEvent>, Query, ReadOptions),
}

/// One row per [`Kind::ModelOnlyMutant`] in [`REGISTRY`], and no others.
///
/// **Empty since `read_to_composes_with_multi_item_query` landed**, and the
/// emptiness is a measured claim rather than an oversight.
/// `UnparenthesisedToPredicateStore` was the kind's only member; the rule that
/// closed CF-12's gap for `to` sees it, so it is an ordinary [`Kind::Mutant`]
/// with one entry in `fails` and its witness would now be an orphan — which
/// `every_model_only_mutant_demonstrates_its_defect`'s second loop rejects by
/// name.
///
/// The **kind** is deliberately left standing. Whether a kind with no members
/// should be withdrawn (as `Kind::StatedOnlyDefect` was, for a different and
/// worse reason — its bar was unsound) is a decision, and a decision belongs in
/// an ADR pass rather than in the lane that emptied it. What holds the emptiness
/// honest meanwhile is the pair of `is_empty` assertions on the two tests below:
/// a table with rows and no members is caught, and so is a member with no row.
const MODEL_ONLY_WITNESSES: &[Witness] = &[];

/// Hands every registered store **type** to `$callback`.
///
/// Mirrors `happenstance_testkit::for_each_event_store_rule!`, and for the same
/// reason: the set of stores is the second list in this phase that can silently
/// drift. The callback is captured as raw token trees rather than as
/// `$cb:path` — a parsed `path` fragment cannot sit in callee position inside an
/// expression, which would forbid `let reports = for_each_mutant!(…)`.
///
/// It lives here rather than in `harness.rs` because it and [`REGISTRY`] are the
/// pair that must agree; separating them is how they stop agreeing.
macro_rules! for_each_mutant {
    ($($callback:tt)+) => {
        $($callback)+! {
            crate::variants::GappedPositionFixture,
            crate::variants::PagedStreamFixture,

            crate::mutants::MutantFixture<crate::mutants::InnerJoinTagStore>,
            crate::mutants::MutantFixture<crate::mutants::TypesAreAndStore>,
            crate::mutants::MutantFixture<crate::mutants::TagsAreOrStore>,
            crate::mutants::MutantFixture<crate::mutants::ExactTagMatchReadStore>,
            crate::mutants::MutantFixture<crate::mutants::ClauseJoinerStore>,
            crate::mutants::MutantFixture<crate::mutants::ItemsAreAndStore>,
            crate::mutants::MutantFixture<crate::mutants::UninternedTypeStore>,

            crate::mutants::MutantFixture<crate::mutants::SortByEventTypeStore>,
            crate::mutants::MutantFixture<crate::mutants::FromIsAnOffsetStore>,
            crate::mutants::MutantFixture<crate::mutants::BackwardsIgnoredStore>,
            crate::mutants::MutantFixture<crate::mutants::FetchOneExtraStore>,
            crate::mutants::MutantFixture<crate::mutants::EmptyHeadIsFirstStore>,
            crate::mutants::MutantFixture<crate::mutants::DefaultQueryHeadStore>,
            crate::mutants::MutantFixture<crate::mutants::ToBoundIgnoredStore>,
            crate::mutants::MutantFixture<crate::mutants::ToIsExclusiveStore>,
            crate::mutants::MutantFixture<crate::mutants::BackwardsToIsAnUpperBoundStore>,
            crate::mutants::MutantFixture<crate::mutants::LimitZeroIsUnlimitedStore>,
            crate::mutants::MutantFixture<crate::mutants::ForwardPagingBudgetStore>,
            crate::mutants::MutantFixture<crate::mutants::WindowedPagingBudgetStore>,

            crate::mutants::MutantFixture<crate::mutants::SharedBatchPositionStore>,
            crate::mutants::MutantFixture<crate::mutants::ReturnsFirstOfBatchStore>,
            crate::mutants::MutantFixture<crate::mutants::ReverseOrderBatchStore>,
            crate::mutants::MutantFixture<crate::mutants::WriteThenCheckStore>,
            crate::mutants::MutantFixture<crate::mutants::PerRowConditionStore>,
            crate::mutants::MutantFixture<crate::mutants::PayloadDedupStore>,
            crate::mutants::MutantFixture<crate::mutants::EmptyBatchIsANoOpStore>,
            crate::mutants::MutantFixture<crate::mutants::EmptyBatchPanicsStore>,
            crate::mutants::MutantFixture<crate::mutants::DropsMetadataStore>,
            crate::mutants::MutantFixture<crate::mutants::ViolationAsStoreErrorStore>,
            crate::mutants::MutantFixture<crate::mutants::FirstTagOnlyConditionStore>,

            crate::mutants::MutantFixture<crate::mutants::AfterDefaultsToFirstStore>,
            crate::mutants::MutantFixture<crate::mutants::ExistenceProbeStore>,
            crate::mutants::MutantFixture<crate::mutants::AfterIsInclusiveStore>,
            crate::mutants::MutantFixture<crate::mutants::AfterIsAnOffsetStore>,

            crate::mutants::MutantFixture<crate::mutants::TagJoinFanOutStore>,
            crate::mutants::MutantFixture<crate::mutants::ItemOrderedUnionStore>,
            crate::mutants::MutantFixture<crate::mutants::LimitBeforeFilterStore>,
            crate::mutants::MutantFixture<crate::mutants::LimitPerItemStore>,
            crate::mutants::MutantFixture<crate::mutants::ItemDedupByTypeStore>,
            crate::mutants::MutantFixture<crate::mutants::UnparenthesisedPredicateStore>,
            crate::mutants::MutantFixture<crate::mutants::UnparenthesisedToPredicateStore>,
            crate::mutants::MutantFixture<crate::mutants::NullHeadPagingStore>,
            crate::mutants::MutantFixture<crate::mutants::ConditionBeforeEmptinessStore>,
            crate::mutants::MutantFixture<crate::mutants::AfterValidatedAgainstHeadStore>,
            crate::mutants::MutantFixture<crate::mutants::ExactTagMatchConditionStore>,
            crate::mutants::MutantFixture<crate::mutants::TagBlindConditionStore>,
            crate::mutants::MutantFixture<crate::mutants::NullAggregateProbeStore>,
            crate::mutants::MutantFixture<crate::mutants::UncorrelatedProbeStore>,
            crate::mutants::MutantFixture<crate::mutants::MinCollapseStore>,
            crate::mutants::MutantFixture<crate::mutants::SingleGuardFastPathStore>,

            crate::mutants::NoTransactionFixture,
            crate::mutants::YieldingRowAtATimeFixture,
            crate::mutants::NoopFaultFixture,

            crate::mutants::MutantFixture<crate::mutants::EmptyPayloadIsNullStore>,
            crate::mutants::MutantFixture<crate::mutants::MetadataConflatingStore>,
            crate::mutants::MutantFixture<crate::mutants::NarrowIdentifierColumnStore>,
            crate::mutants::MutantFixture<crate::mutants::Latin1IdentifierStore>,
            crate::mutants::MutantFixture<crate::mutants::PayloadCeilingStore>,
            crate::mutants::MutantFixture<crate::mutants::TruncatingPayloadStore>,
            crate::mutants::MutantFixture<crate::mutants::PackedTagColumnStore>,
            crate::mutants::MutantFixture<crate::mutants::ChunkedQueryStore>,
            crate::mutants::MutantFixture<crate::mutants::BatchParameterCeilingStore>,
            crate::mutants::MutantFixture<crate::mutants::ChunkLosingBatchStore>,
            crate::mutants::MutantFixture<crate::mutants::NormalisingTagStore>,
            crate::mutants::MutantFixture<crate::mutants::KeyedTagMapStore>,
            crate::mutants::MutantFixture<crate::mutants::TrimmingIdentifierStore>,

            crate::mutants::MutantFixture<crate::mutants::RowOrdinalIdentityStore>,
            crate::mutants::MutantFixture<crate::mutants::ReadTimeClockStore>,
            crate::mutants::MutantFixture<crate::mutants::ContentHashIdentityStore>,
            crate::mutants::MutantFixture<crate::mutants::PerEventStoreIdStore>,
            crate::mutants::MutantFixture<crate::mutants::SharedBatchIdentityStore>,
            crate::mutants::MutantFixture<crate::mutants::IdentityMatchableAsTagStore>,
            crate::mutants::MutantFixture<crate::mutants::PositionOnlyMembershipStore>,

            crate::mutants::CachedHeadFixture,
            crate::mutants::LastWrittenHeadFixture,
            crate::mutants::LosingFixture,
            crate::mutants::RestampingFixture,
            crate::mutants::SharedBackingFixture,
            crate::mutants::PreCommitPositionFixture,
            crate::mutants::PollPaddedPositionFixture,
            crate::mutants::BorrowHoldingFixture,
            crate::mutants::RefetchingPagedFixture,
            crate::mutants::SwallowedReadFaultFixture,
            crate::mutants::AwaitAcrossBorrowFixture,
        }
    };
}

/// Drives every registered store through every registered rule.
fn reports() -> Vec<SubjectReport> {
    macro_rules! run_each {
        ($($subject:ty),* $(,)?) => {
            ::std::vec![ $( crate::harness::run_subject::<$subject>() ),* ]
        };
    }

    for_each_mutant!(run_each)
}

/// The `NAME` of every registered store, from the store types themselves.
fn registered_names() -> Vec<&'static str> {
    macro_rules! names {
        ($($subject:ty),* $(,)?) => {
            ::std::vec![ $( <$subject as crate::harness::Subject>::NAME ),* ]
        };
    }

    for_each_mutant!(names)
}

/// `(name, SECOND_HANDLE)` for every registered store.
fn registered_second_handle() -> Vec<(&'static str, happenstance_testkit::Capability)> {
    macro_rules! capabilities {
        ($($subject:ty),* $(,)?) => {
            ::std::vec![ $( (
                <$subject as crate::harness::Subject>::NAME,
                <$subject as happenstance_testkit::Fixture>::SECOND_HANDLE,
            ) ),* ]
        };
    }

    for_each_mutant!(capabilities)
}

/// Every rule name, from the suite's own single enumeration.
///
/// This is the universe every membership check in this file is resolved
/// against. There is no second list of rule names anywhere here, on purpose.
fn all_rules() -> Vec<&'static str> {
    happenstance_testkit::for_each_event_store_rule!(happenstance_testkit::__emit_rule_names)
        .to_vec()
}

/// Every projection rule name, from the projection family's own single
/// enumeration.
///
/// [`all_rules`] one enumeration over, and the same discipline: the projection
/// meta-tests resolve every membership check against this and never against a
/// list of their own, so a rule added to the family arrives in them without an
/// edit here.
fn all_projection_rules() -> Vec<&'static str> {
    happenstance_testkit::for_each_projection_store_rule!(happenstance_testkit::__emit_rule_names)
        .to_vec()
}

/// The registry row for `name`, if there is one.
fn declared(name: &str) -> Option<&'static Declared> {
    REGISTRY.iter().find(|entry| entry.name == name)
}

/// What the model family made of one store.
///
/// Three outcomes for the same reason [`Verdict`] has three: a store that falls
/// over is not a store the model caught, and counting it as one would let a
/// mutant whose modelled defect had been deleted keep reading as evidence.
#[cfg(feature = "proptest")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ModelOutcome {
    /// Every generated sequence agreed with the model.
    Agreed,
    /// The model found a sequence the store answered wrongly.
    Rejected,
    /// The store panicked somewhere that is not the model's own assertion.
    FellOver,
}

/// Classifies one model-rule [`Verdict`].
#[cfg(feature = "proptest")]
fn model_outcome(verdict: &Verdict) -> ModelOutcome {
    match verdict {
        Verdict::Passed => ModelOutcome::Agreed,
        // No model rule is capability-gated, so a skip would mean the family
        // grew a `require!` nobody accounted for. Classed as `FellOver` rather
        // than silently as agreement, because it is the one outcome that means
        // *nothing ran*.
        Verdict::Skipped { .. } => ModelOutcome::FellOver,
        Verdict::Panicked { message, origin } => {
            let from_the_model = origin.as_ref().is_some_and(Origin::is_a_model_rule_body)
                && !RUNTIME_PANICS.iter().any(|needle| message.contains(needle));
            if from_the_model {
                ModelOutcome::Rejected
            } else {
                ModelOutcome::FellOver
            }
        }
    }
}

/// Drives every registered store through every rule of the model family.
#[cfg(feature = "proptest")]
fn model_reports() -> Vec<(&'static str, ModelOutcome, String)> {
    macro_rules! run_each {
        ($($subject:ty),* $(,)?) => {
            ::std::vec![ $(
                crate::harness::model_probes::<$subject>()
                    .into_iter()
                    .map(|probe| {
                        let verdict = crate::harness::run_probe(probe);
                        (
                            <$subject as crate::harness::Subject>::NAME,
                            model_outcome(&verdict),
                            verdict.describe(),
                        )
                    })
                    .collect::<::std::vec::Vec<_>>()
            ),* ]
        };
    }

    let per_subject: Vec<Vec<_>> = for_each_mutant!(run_each);
    per_subject.into_iter().flatten().collect()
}

/// What the model family does to every store in this binary, measured.
///
/// # Why this is exhaustive rather than a list of successes
///
/// `event_store_model_conformance!` is one macro rather than N named rules, so
/// it has no [`REGISTRY`] row and CF-1 does not reach it. What CF-1's *argument*
/// reaches it with is unchanged: a model-based test that no store fails is
/// decorative in exactly the way ADR-0010 describes, and it is better to
/// discover that here than to ship it. A table of the stores it catches would
/// answer that and nothing else. A table of **every** store answers the question
/// that actually matters — *what is this test blind to* — and it answers it in a
/// form that goes red when the answer changes.
///
/// # What it does not catch is a handful of shapes, not a list
///
/// **Count the table rather than this sentence** — and something now does.
/// Forty-one rows below are marked [`ModelOutcome::Agreed`] at this commit; two
/// of them are the conformant controls and *must* be, which leaves
/// **thirty-nine misses**. `the_model_coverage_heading_counts_the_table` asserts both numbers
/// against the table, and the count of controls it subtracts is derived from
/// [`REGISTRY`]'s [`Kind::ConformantVariant`] rows rather than written a second
/// time.
///
/// That check is here because the sentence has now drifted twice, at two
/// scales. This heading said *twenty-one* and the paragraph under it said
/// *twenty-three rows* for several phases while the table itself said forty and
/// forty-two — the drift the pre-publication review found wherever a number was
/// written beside the list it describes. It was corrected to thirty-nine and
/// thirty-seven, and was stale again by one inside the day, because a mutant
/// landed and nothing counted. It went stale a third way at phase 10's merge,
/// where neither branch was wrong on its own — one added a row, the other
/// corrected the sentence for its own additions, and `git` took both with no
/// conflict. **A corrected number is not a fix for a number nothing counts**,
/// and a number two authors correct separately is the same failure wearing a
/// second hat. The shapes below are still the shapes; they no longer
/// enumerate every name, and the honest instrument is the assertion.
///
/// Every miss carries a defect the model **cannot express**, and the boundary is
/// sharp enough to state in one line: the model drives *one handle*, on *one
/// fixture*, through a *strictly sequential* stream of *non-empty* batches of
/// *typical* values, and it never reopens and never arms a fixture.
///
/// **Three names left this list at phase 12**, and they are worth naming because
/// what moved was the *generator* rather than any store. `ToBoundIgnoredStore`,
/// `ToIsExclusiveStore` and `BackwardsToIsAnUpperBoundStore` were misses for one
/// reason: [`Op`]`::Read` carried four of the five read options, `to` was the
/// missing one, and `Model::select`'s two `to` branches were therefore dead
/// code. All three are rejected now, and a fourth store —
/// `UnparenthesisedToPredicateStore` — was written to be caught by nothing else,
/// which is what `Kind::ModelOnlyMutant` recorded until
/// `read_to_composes_with_multi_item_query` landed and made it an ordinary
/// mutant — the row below stays `Rejected` and the model stays what proves it,
/// but it is no longer the *only* thing that does. `LimitZeroIsUnlimitedStore` is
/// the read option still on this list, and it is a *value* boundary rather than
/// a missing field: `Op::Read` generates `Option<usize>` over `1..4` and never
/// proposes the zero, which is the exclusion the ten value edges sit behind.
///
/// * `EmptyBatchIsANoOpStore`, `EmptyBatchPanicsStore`,
///   `ConditionBeforeEmptinessStore` — the empty batch, which [`Op`] does not
///   generate. Deliberately: ES-20's ordering of `NoEvents` against
///   `ConditionViolated` is pinned by two named rules, and generating it here
///   would re-assert a settled thing in a test whose failures are harder to
///   read.
/// * `CachedHeadFixture`, `SharedBackingFixture` — defects that need a second
///   handle or a second fixture instance.
/// * `LosingFixture`, `RestampingFixture` — defects that are only visible across
///   a reopen. Two names rather than one since phase 8, and the pair is worth
///   reading together: the model misses the second for a *sharper* reason than
///   the first. `LosingFixture` loses every event, which a model that reopened
///   would notice immediately; `RestampingFixture` loses one field of one
///   store-assigned fact, so even a model that reopened would have to be
///   comparing `recorded_at` across the boundary to see it.
/// * `PreCommitPositionStore`, `BorrowHoldingStore`, `AwaitAcrossBorrowStore` —
///   defects whose content is a *window*: two futures overlapping on one handle.
///   The model awaits each operation to completion before starting the next, so
///   there is no window for them to be wrong in.
/// * `NoTransactionStore` — a defect that only exists once a fault is **armed**,
///   and arming is a call on the *fixture*. An `Op` that armed one would be
///   generating a fixture call rather than a store operation, which is not what
///   [`Op`] is. Unarmed, this store is an ordinary correct one.
/// * The ten value edges, from `EmptyPayloadIsNullStore` to
///   `ChunkLosingBatchStore` — one shape rather than ten. Every one of them is
///   wrong only at a boundary `fixtures::strategies::any_event` does not reach:
///   it generates a small non-empty payload, metadata that is `None` or
///   non-empty, up to six tags from a five-symbol alphabet, and queries of up to
///   three items. Two of those exclusions are documented on the generator itself
///   — the empty payload and `Some(<empty>)` metadata were kept out because the
///   clause that owned the question had not been written. It has now, and the
///   rules that own it are named rather than generated.
///
/// That is the shape of the argument for the concurrency family, arriving from
/// the other direction: three of the window-shaped six are exactly what a
/// concurrent harness is for, and the model demonstrates by construction that a
/// sequential generator cannot reach them however many cases it runs. The value
/// edges make the same argument for *named* rules: a generator tuned to typical
/// values will never propose 65,536 bytes, and one tuned to propose it would
/// spend most of its cases there.
///
/// [`Op`]: happenstance_testkit::model::Op
#[cfg(feature = "proptest")]
const MODEL_COVERAGE: &[(&str, ModelOutcome)] = &[
    // The conformant controls. CF-5's argument applies to this rule with more
    // force than to any other, because the model *generates* position anchors:
    // `GappedPositionStore` assigns positions in steps of seven from 4096, and
    // it is the whole reason the generator emits an `Anchor` rather than a
    // number. If it ever appears in the rejected column, the finding is about
    // the model.
    ("GappedPositionStore", ModelOutcome::Agreed),
    ("PagedStreamStore", ModelOutcome::Agreed),
    // Query semantics.
    ("InnerJoinTagStore", ModelOutcome::Rejected),
    ("TypesAreAndStore", ModelOutcome::Rejected),
    ("TagsAreOrStore", ModelOutcome::Rejected),
    ("ExactTagMatchReadStore", ModelOutcome::Rejected),
    ("ClauseJoinerStore", ModelOutcome::Rejected),
    ("ItemsAreAndStore", ModelOutcome::Rejected),
    ("UninternedTypeStore", ModelOutcome::Rejected),
    // Read options.
    ("SortByEventTypeStore", ModelOutcome::Rejected),
    ("FromIsAnOffsetStore", ModelOutcome::Rejected),
    ("BackwardsIgnoredStore", ModelOutcome::Rejected),
    ("FetchOneExtraStore", ModelOutcome::Rejected),
    // The head. `Op` generates reads and appends; nothing in the model calls
    // `head()` at all, so a defect that is only visible through it cannot be
    // reached however many cases run. That is the same boundary the ten value
    // edges sit on, one method over.
    ("EmptyHeadIsFirstStore", ModelOutcome::Agreed),
    ("DefaultQueryHeadStore", ModelOutcome::Agreed),
    // The upper bound, and all three moved from `Agreed` to `Rejected` at phase
    // 12 without any of them changing. What changed is the generator: `Op::Read`
    // now carries a `to`, so `Model::select`'s two `to` branches execute. They
    // sat under the *head* comment above only because that is where the table
    // put them, and they were never about `head`.
    ("ToBoundIgnoredStore", ModelOutcome::Rejected),
    ("ToIsExclusiveStore", ModelOutcome::Rejected),
    ("BackwardsToIsAnUpperBoundStore", ModelOutcome::Rejected),
    // The `to` twin of the precedence bug. This row was once the whole of what
    // caught it — `Kind::ModelOnlyMutant` — and it is now a second instrument
    // beside `read_to_composes_with_multi_item_query`, which is the arrangement
    // every other row here has.
    ("UnparenthesisedToPredicateStore", ModelOutcome::Rejected),
    // A *value* boundary rather than a missing field: `Op::Read`'s limit is
    // `Option<usize>` over `1..4` and never proposes the zero this store
    // mishandles.
    ("LimitZeroIsUnlimitedStore", ModelOutcome::Agreed),
    ("ForwardPagingBudgetStore", ModelOutcome::Rejected),
    ("WindowedPagingBudgetStore", ModelOutcome::Rejected),
    ("LimitPerItemStore", ModelOutcome::Rejected),
    ("ItemDedupByTypeStore", ModelOutcome::Rejected),
    // Append.
    ("SharedBatchPositionStore", ModelOutcome::Rejected),
    ("ReturnsFirstOfBatchStore", ModelOutcome::Rejected),
    ("WriteThenCheckStore", ModelOutcome::Rejected),
    ("ReverseOrderBatchStore", ModelOutcome::Rejected),
    ("PerRowConditionStore", ModelOutcome::Rejected),
    ("PayloadDedupStore", ModelOutcome::Rejected),
    ("EmptyBatchIsANoOpStore", ModelOutcome::Agreed),
    ("EmptyBatchPanicsStore", ModelOutcome::Agreed),
    ("DropsMetadataStore", ModelOutcome::Rejected),
    ("ViolationAsStoreErrorStore", ModelOutcome::Rejected),
    ("FirstTagOnlyConditionStore", ModelOutcome::Rejected),
    // The condition probe.
    ("AfterDefaultsToFirstStore", ModelOutcome::Rejected),
    ("ExistenceProbeStore", ModelOutcome::Rejected),
    ("AfterIsInclusiveStore", ModelOutcome::Rejected),
    ("AfterIsAnOffsetStore", ModelOutcome::Rejected),
    // The measured gaps of §6.2.
    ("TagJoinFanOutStore", ModelOutcome::Rejected),
    ("ItemOrderedUnionStore", ModelOutcome::Rejected),
    ("LimitBeforeFilterStore", ModelOutcome::Rejected),
    ("UnparenthesisedPredicateStore", ModelOutcome::Rejected),
    ("NullHeadPagingStore", ModelOutcome::Rejected),
    ("ConditionBeforeEmptinessStore", ModelOutcome::Agreed),
    ("AfterValidatedAgainstHeadStore", ModelOutcome::Rejected),
    ("ExactTagMatchConditionStore", ModelOutcome::Rejected),
    ("TagBlindConditionStore", ModelOutcome::Rejected),
    ("NullAggregateProbeStore", ModelOutcome::Rejected),
    ("UncorrelatedProbeStore", ModelOutcome::Rejected),
    ("MinCollapseStore", ModelOutcome::Agreed),
    ("SingleGuardFastPathStore", ModelOutcome::Rejected),
    // The fixture-armed fault, and the ten value edges. Both arguments live in
    // this table's doc comment rather than here, so that the count and the reason
    // cannot drift apart the way they did between stage 5 and stage 6.
    ("NoTransactionStore", ModelOutcome::Agreed),
    ("YieldingRowAtATimeStore", ModelOutcome::Agreed),
    ("NoopFaultFixture", ModelOutcome::Agreed),
    ("EmptyPayloadIsNullStore", ModelOutcome::Agreed),
    ("MetadataConflatingStore", ModelOutcome::Agreed),
    ("NarrowIdentifierColumnStore", ModelOutcome::Agreed),
    ("Latin1IdentifierStore", ModelOutcome::Agreed),
    ("PayloadCeilingStore", ModelOutcome::Agreed),
    ("TruncatingPayloadStore", ModelOutcome::Agreed),
    ("PackedTagColumnStore", ModelOutcome::Agreed),
    ("ChunkedQueryStore", ModelOutcome::Agreed),
    ("BatchParameterCeilingStore", ModelOutcome::Agreed),
    ("ChunkLosingBatchStore", ModelOutcome::Agreed),
    ("NormalisingTagStore", ModelOutcome::Agreed),
    ("KeyedTagMapStore", ModelOutcome::Agreed),
    ("TrimmingIdentifierStore", ModelOutcome::Agreed),
    // Identity, recorded time and membership, and the split here was found by
    // running rather than by reading. `Journal::reconcile` does only compare
    // `seen.event` and the positions — so a defect visible only at append is
    // invisible to it — but `Op::Read` compares two whole `SequencedEvent` lists,
    // the observed one against `known`, which is the *store's own* answer to the
    // last `Query::all()`. So the model catches exactly the identity and time
    // defects that make two differently-shaped reads disagree, and nothing else:
    // a defect that is stable across reads (a content hash, a per-event
    // incarnation, one identity per batch) is consistent with itself and
    // survives, and no generated `Op` calls `contains_event_id` at all.
    ("RowOrdinalIdentityStore", ModelOutcome::Rejected),
    ("ReadTimeClockStore", ModelOutcome::Rejected),
    ("ContentHashIdentityStore", ModelOutcome::Agreed),
    ("PerEventStoreIdStore", ModelOutcome::Agreed),
    ("SharedBatchIdentityStore", ModelOutcome::Agreed),
    ("IdentityMatchableAsTagStore", ModelOutcome::Agreed),
    ("PositionOnlyMembershipStore", ModelOutcome::Agreed),
    // The fixture-level and window-shaped defects.
    ("CachedHeadFixture", ModelOutcome::Agreed),
    ("LastWrittenHeadStore", ModelOutcome::Agreed),
    ("LosingFixture", ModelOutcome::Agreed),
    ("RestampingFixture", ModelOutcome::Agreed),
    ("SharedBackingFixture", ModelOutcome::Agreed),
    ("PreCommitPositionStore", ModelOutcome::Agreed),
    ("PollPaddedPositionStore", ModelOutcome::Agreed),
    ("BorrowHoldingStore", ModelOutcome::Agreed),
    ("RefetchingPagedStore", ModelOutcome::Agreed),
    // Agreed, and it has to be: unarmed, this store is completely conformant,
    // and nothing in the model family arms a read fault. The row is the
    // measurement that the generative family is not what catches it.
    ("SwallowedReadFaultStore", ModelOutcome::Agreed),
    ("AwaitAcrossBorrowStore", ModelOutcome::Agreed),
];

// =====================================================================
// The racing registry
// =====================================================================

/// What the concurrency family made of one racing store.
///
/// Three outcomes for [`Verdict`]'s reason, and the third one earns its keep
/// harder here than anywhere else: a contender panics on *its own thread*, and
/// the location the hook records lands in that thread's local where nothing
/// reads it. What crosses back is a payload with no origin, so a store that
/// falls over inside a contender arrives as [`Self::FellOver`] rather than as a
/// rejection. That is the safe direction — a store that panicked must never read
/// as a store a rule caught — and it means a `FellOver` row is a defect in the
/// instrument rather than an ambiguity to be explained away.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RacerOutcome {
    /// Every assertion in the rule held.
    Passed,
    /// The rule's own assertion rejected the store.
    Rejected,
    /// The store panicked somewhere that is not a rule's assertion.
    FellOver,
}

/// One row of the racing registry.
///
/// Deliberately **not** a [`Declared`]: `fails` there is checked against
/// `for_each_event_store_rule!`, and every store here fails none of those rules —
/// which is the entire point of them. Sharing the shape would mean loosening the
/// check that a mutant cannot land before the rule that catches it.
#[derive(Debug)]
struct Racer {
    /// Matches the store's own [`harness::Subject::NAME`].
    name: &'static str,
    /// Whether this row is a defect or the family's conformant control.
    ///
    /// [`Declared::kind`]'s twin, and it is a field rather than a reading of
    /// `fails` for the reason `fails.is_empty()` is *not* the same question. An
    /// empty `fails` list is also what a disarmed mutant looks like, and this
    /// family documents its own rendezvous flakiness at length — so emptying a
    /// flaky row's list is the cheaper repair than fixing a rendezvous, and a
    /// control derived from emptiness would be manufactured by exactly that
    /// edit.
    kind: RacerKind,
    /// The **exact** set of concurrency rules this store fails. Empty iff this
    /// row is [`RacerKind::ConformantControl`].
    fails: &'static [&'static str],
    /// CF-4's obligation, one family over: the real adapter shape that makes
    /// this store plausible. Never empty.
    provenance: &'static str,
    /// Per-rule pins: `(rule, substring)` naming the **exact assertion** this
    /// store is supposed to trip.
    ///
    /// [`Declared::expect`]'s twin, and this family needs it more rather than
    /// less. Two of the five rules carry several assertions, and two different
    /// stores fail
    /// `k_disjoint_boundaries_admit_exactly_k_commits` for *opposite* reasons —
    /// `RacingProbeStore` elects too many winners, `GlobalVersionStore` elects
    /// none. Without the pin, either could quietly start failing the other's
    /// assertion and the table would stay green while both provenance
    /// paragraphs became fiction.
    ///
    /// A pin naming a rule the store does not declare is an error: a pin on a
    /// rule that never fails is a claim nothing evaluates.
    expect: &'static [(&'static str, &'static str)],
}

/// What a [`Racer`] row claims about its store.
///
/// [`Kind`]'s counterpart for the concurrency family. The event-store family
/// and the projection family both carry one and both assert that a conformant
/// member is registered; this family did not, and its control was held by an
/// empty list and a paragraph.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RacerKind {
    /// A store with a named defect, which MUST fail exactly the rules it
    /// declares.
    Racing,
    /// A store that is legally different from `MemoryEventStore` and MUST pass
    /// every rule of this family (CF-5).
    ConformantControl,
}

/// Every store the concurrency family is driven against here.
///
/// # What this table is for
///
/// `event_store_concurrency_conformance!` is five rules that pass against
/// `MemoryEventStore`, and a suite that has only ever been run against a store
/// which serialises its writers has demonstrated nothing about contention. CF-1's
/// argument — a rule no implementation can fail is decorative — is sharper for
/// this family than for any other, because a racing rule is the easiest kind to
/// write in a form that cannot fail: start some threads, assert something true of
/// the final state, and the test is green whatever the store did in between.
///
/// So each rule below names the store it rejects, each store is wrong in exactly
/// one way, and each way is one a real adapter is wrong in. Both directions are
/// checked, as CF-3 asks: a store must fail every rule it declares and pass every
/// rule it does not.
///
/// # What it does not cover
///
/// * **A real medium.** These are `Arc`/`Mutex` stores in one process. Nothing
///   here crosses a connection, a socket or a transaction manager, so an
///   adapter's *actual* isolation level is untested until phase 8 and phase 10.
/// * **A store that does not serialise its writers.** `LockedStore` is the
///   control and it holds one mutex, which is the shape of every adapter in this
///   workspace bar one. `happenstance-postgres` is the instrument at the other
///   end of that axis and it is still a skeleton; until it runs this family, the
///   rules have only been passed by stores that made passing them easy.
/// * **Liveness.** There is no watchdog, by CF-33 and by
///   `harness.rs`'s reasoning: a store that deadlocks hangs the binary, and the
///   CI job timeout is the only thing that notices.
const RACERS: &[Racer] = &[
    Racer {
        kind: RacerKind::ConformantControl,
        name: "LockedStore",
        fails: &[],
        provenance: "the conformant control: one mutex held across the whole append, which is \
             the shape of rusqlite behind a connection, of a Durable Object, and of \
             `MemoryEventStore` itself",
        expect: &[],
    },
    // **There is no `REGISTRY` row for this defect and there cannot be one.**
    // `mutant_registry_is_exhaustive` rejects a `Declared` whose `fails` list is
    // empty, and a probe-then-insert store fails *no* sequential rule — that is
    // the entire content of the defect. A concurrency rule's wrong store belongs
    // here, in `RACERS`, and `crates/happenstance-testkit/README.md:187-190` says
    // so in prose where the harness says it in code
    // (`mutant_registry_is_exhaustive`, below). Recorded on the row rather than
    // in a commit message because "add a `BEGIN DEFERRED` row to `REGISTRY`" is
    // an instruction that has been written down once already and will read as an
    // unfilled gap to the next person who looks for it.
    Racer {
        kind: RacerKind::Racing,
        name: "RacingProbeStore",
        fails: &[
            "exactly_one_of_n_contenders_commits",
            "k_disjoint_boundaries_admit_exactly_k_commits",
        ],
        provenance: "`BEGIN DEFERRED` — or no `BEGIN` at all — then `SELECT 1 FROM events \
             WHERE …`, then `INSERT`, with nothing holding a write lock across the \
             two and no `SERIALIZABLE` under them. `BEGIN DEFERRED` is the SQLite \
             spelling and it is the default one: rusqlite's `Connection::transaction` \
             opens deferred, so the read lock is taken at the probe and promoted at \
             the insert, and between those two SQLite lets another connection \
             commit. The autocommit form is the same defect with the driver's \
             convenience API — one statement per call — supplying the gap instead. \
             It is `WriteThenCheckStore` with the two halves the other way round, \
             which is what makes it invisible to `append_is_atomic`",
        // Both pins name the *too many winners* side. That is the whole content
        // of this defect: every contender's probe was answered before any of
        // them acted on it, so the answers were all stale together.
        expect: &[
            ("exactly_one_of_n_contenders_commits", "may commit"),
            (
                "k_disjoint_boundaries_admit_exactly_k_commits",
                "must elect at most one winner",
            ),
        ],
    },
    Racer {
        kind: RacerKind::Racing,
        name: "GlobalVersionStore",
        fails: &["k_disjoint_boundaries_admit_exactly_k_commits"],
        provenance: "optimistic concurrency control on a single version number: a Durable \
             Object with one `version` key, `UPDATE … WHERE version = ?`, or a \
             `SERIALIZABLE` adapter mapping `40001 serialization_failure` onto \
             `ConditionViolated`. Every command in a busy store is told it is \
             contending with every other",
        // The opposite side of the same rule, which is why the pin exists: this
        // store elects *no* winner on the boundaries it never touched.
        expect: &[(
            "k_disjoint_boundaries_admit_exactly_k_commits",
            "elected no winner",
        )],
    },
    Racer {
        kind: RacerKind::Racing,
        name: "RacingSequenceStore",
        fails: &["positions_are_unique_under_concurrent_appends"],
        provenance: "`SELECT max(position) FROM events` before `BEGIN`, which is the \
             in-process form of the position-allocation hazard `SPECIFICATION.md` \
             §6.5 owes Postgres a measurement for",
        // Pinned to the *store-side* assertion rather than to the count of
        // committed appends: what is wrong here is the numbers the store
        // assigned, not how many callers it let through.
        expect: &[(
            "positions_are_unique_under_concurrent_appends",
            "may share a position",
        )],
    },
    Racer {
        kind: RacerKind::Racing,
        name: "GlobalHeadStore",
        fails: &["append_returns_the_callers_own_last_position"],
        provenance: "`INSERT …;` then `SELECT max(position) FROM events`, two statements with \
             no transaction around them — what an adapter writes when its driver \
             cannot give it `RETURNING` on a multi-row insert. The caller \
             checkpoints a projection at somebody else's position",
        expect: &[(
            "append_returns_the_callers_own_last_position",
            "must return the position of the caller's own last event",
        )],
    },
    Racer {
        kind: RacerKind::Racing,
        name: "RowAtATimeStore",
        fails: &["a_concurrent_reader_never_sees_a_partial_batch"],
        provenance: "`for event in batch { conn.execute(INSERT, …)? }` with the `BEGIN` \
             forgotten — every row lands, so the store is correct at rest and wrong \
             for exactly as long as the loop runs",
        // The *live* half of the rule, not the post-hoc one. Every row does
        // land here, so a pin on the closing count would be a pin on an
        // assertion this store passes.
        expect: &[(
            "a_concurrent_reader_never_sees_a_partial_batch",
            "part-written",
        )],
    },
];

/// Hands every racing fixture type to `$callback`.
///
/// [`RACERS`]' twin, kept separate for `for_each_mutant!`'s reason: the type
/// list and the claim about it are the two lists that drift.
macro_rules! for_each_racer {
    ($($callback:tt)+) => {
        $($callback)+! {
            crate::racers::LockedFixture,
            crate::racers::RacingProbeFixture,
            crate::racers::GlobalVersionFixture,
            crate::racers::RacingSequenceFixture,
            crate::racers::GlobalHeadFixture,
            crate::racers::RowAtATimeFixture,
        }
    };
}

/// Classifies one concurrency-rule [`Verdict`].
fn racer_outcome(verdict: &Verdict) -> RacerOutcome {
    match verdict {
        Verdict::Passed => RacerOutcome::Passed,
        // No concurrency rule is capability-gated, so a skip would mean the
        // family grew a `require!` nobody accounted for.
        Verdict::Skipped { .. } => RacerOutcome::FellOver,
        Verdict::Panicked { message, origin } => {
            let from_a_rule = origin
                .as_ref()
                .is_some_and(Origin::is_a_concurrency_rule_body)
                && !RUNTIME_PANICS.iter().any(|needle| message.contains(needle));
            if from_a_rule {
                RacerOutcome::Rejected
            } else {
                RacerOutcome::FellOver
            }
        }
    }
}

/// Every racing store's verdict on every concurrency rule.
fn racer_reports() -> Vec<(&'static str, &'static str, RacerOutcome, String)> {
    macro_rules! run_each {
        ($($subject:ty),* $(,)?) => {
            ::std::vec![ $(
                crate::harness::concurrency_probes::<$subject>()
                    .into_iter()
                    .map(|probe| {
                        let verdict = crate::harness::run_probe(probe);
                        (
                            <$subject as crate::harness::Subject>::NAME,
                            probe.name,
                            racer_outcome(&verdict),
                            verdict.describe(),
                        )
                    })
                    .collect::<::std::vec::Vec<_>>()
            ),* ]
        };
    }

    let per_subject: Vec<Vec<_>> = for_each_racer!(run_each);
    per_subject.into_iter().flatten().collect()
}

/// The `NAME` of every racing store, from the types themselves.
fn racer_names() -> Vec<&'static str> {
    macro_rules! names {
        ($($subject:ty),* $(,)?) => {
            ::std::vec![ $( <$subject as crate::harness::Subject>::NAME ),* ]
        };
    }

    for_each_racer!(names)
}

/// Every concurrency rule name, from that family's own single enumeration.
fn all_concurrency_rules() -> Vec<&'static str> {
    happenstance_testkit::for_each_concurrency_rule!(happenstance_testkit::__emit_rule_names)
        .to_vec()
}

// =====================================================================
// The meta-tests
// =====================================================================

/// CF-1 – CF-6 and CF-18.
///
/// Wrapped in a module whose name matches the clauses' `Rule:` citations, so
/// that `cargo test mutation_coverage::every_rule_has_a_mutant` resolves.
mod mutation_coverage {
    use super::{
        Declared, FailureMode, Kind, MODEL_ONLY_WITNESSES, Origin, RACERS, REGISTRY,
        RUNTIME_PANICS, RacerKind, RacerOutcome, Verdict, all_concurrency_rules,
        all_projection_rules, all_rules, declared, racer_names, racer_reports, registered_names,
        registered_second_handle, reports,
    };
    #[cfg(feature = "proptest")]
    use super::{MODEL_COVERAGE, ModelOutcome, model_reports};
    use happenstance_testkit::fixtures::MemoryProjectionFixture;

    use crate::harness::{run_projection_subject, run_subject};
    use crate::variants::{DecliningFixture, DecliningProjectionFixture, GappedPositionFixture};

    /// Every rule the registry claims a mutant for.
    fn covered() -> Vec<&'static str> {
        REGISTRY
            .iter()
            .filter(|entry| entry.kind == Kind::Mutant)
            .flat_map(|entry| entry.fails.iter().copied())
            .collect()
    }

    /// CF-1. Every rule is paired with at least one mutant that fails it.
    ///
    /// This is what makes a decorative rule *unwriteable* rather than merely
    /// discouraged: adding a rule fails this test until its wrong implementation
    /// is named. There is no exemption list and there must never be one — a
    /// tracker with entries is CF-1 being *observed* rather than enforced, and
    /// the whole argument of ADR-0010 is that observation does not scale.
    ///
    /// If a rule ever turns up that is clearly right and has no plausible
    /// failing implementation, ADR-0010 is explicit about the honest response:
    /// record that in the clause and **retire the rule**, rather than inventing
    /// a saboteur to satisfy this test — which `every_mutant_states_its_provenance`
    /// would reject anyway.
    #[test]
    fn every_rule_has_a_mutant() {
        let rules = all_rules();
        let covered = covered();

        let decorative: Vec<_> = rules
            .iter()
            .filter(|name| !covered.contains(*name))
            .collect();
        assert!(
            decorative.is_empty(),
            "these rules have no mutant: no store in this binary can fail them, \
             so they certify nothing. Write the wrong implementation into \
             `tests/mutation_coverage/mutants.rs` and declare it in `REGISTRY` \
             (ADR-0010 §1): {decorative:?}"
        );
    }

    /// Every surface that promises a printed skip names the flag that prints it.
    ///
    /// L1-3. CF-18 requires a rule whose capability is unmet to be emitted as a
    /// test that **reports** the skip with the fixture's stated reason, and the
    /// clause's own `Rejects:` turns on that landing *in the adapter's CI log
    /// where a reviewer and a user of the adapter can both see it*. A rule that
    /// skips is a test that **passes**, and libtest discards a passing test's
    /// stdout. `RuleOutcome::report`'s own documentation measured this and says
    /// so exactly; three reader-facing surfaces said the opposite, and they are
    /// the only three a stranger reads.
    ///
    /// Measured against a fixture already in the tree, before this test was
    /// written: `cargo test -p happenstance-sqlite --test conformance` prints
    /// **zero** `SKIP` lines, and the same command with `-- --show-output`
    /// prints **three**. Exactly one CI in the world passes that flag.
    ///
    /// **Rejects: a promise about an adapter author's CI log that is true only
    /// in this repository.**
    ///
    /// # What this does not verify
    ///
    /// That the caveat is *correct*, only that it is present near the promise.
    /// It reads three files by path and would not notice a fourth surface
    /// making the same claim. Its trigger is the narrow one — printing within
    /// 120 characters of the words *stated reason* — so a promise phrased
    /// without them escapes, and the wider trigger this started with fired on
    /// `RuleOutcome`'s own prose about `#[must_use]`, which is a false positive
    /// rather than a find. Its window is a character count rather than a
    /// paragraph, so a promise and its caveat separated by more than 900
    /// characters read as a violation here even where a human would not.
    ///
    /// The mechanism half of the finding — whether CF-18's obligation should be
    /// discharged by something a stranger's default `cargo test` can observe,
    /// or whether the clause is narrowed to what libtest permits — is a
    /// decision this test does not take and must not be read as taking. CF-18
    /// is `[FROZEN]`; the decision is an ADR's, and the argument is staged in
    /// `.kb/_intake/remediation-2026-09-04-briefs/`.
    #[test]
    fn a_promised_skip_line_names_the_flag_it_needs() {
        /// The reader-facing surfaces, by the path a citation would use.
        const SURFACES: &[(&str, &str)] = &[
            (
                "crates/happenstance-testkit/README.md",
                include_str!("../README.md"),
            ),
            (
                "crates/happenstance-testkit/src/lib.rs",
                include_str!("../src/lib.rs"),
            ),
            (
                "crates/happenstance-testkit/src/contract.rs",
                include_str!("../src/contract.rs"),
            ),
        ];

        for (name, text) in SURFACES {
            let flat: Vec<char> = text
                .replace("//!", " ")
                .replace("///", " ")
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ")
                .chars()
                .collect();
            let flat: String = flat.into_iter().collect();
            let chars: Vec<char> = flat.chars().collect();

            for (index, _) in flat.match_indices("print") {
                let at = flat[..index].chars().count();
                let near: String = chars[at.saturating_sub(120)..(at + 120).min(chars.len())]
                    .iter()
                    .collect();
                // Only the sentences that make the promise. "print" appears in
                // this crate about plenty that is not a skip line, and the
                // trigger is deliberately the narrow one: a sentence that puts
                // *the fixture's stated reason* next to printing it.
                if !near.contains("stated reason") {
                    continue;
                }
                let window: String = chars[at..(at + 900).min(chars.len())].iter().collect();
                assert!(
                    window.contains("--show-output"),
                    "{name} promises a printed skip and does not name \
                     `--show-output` within the same passage. A skipped rule is \
                     a test that passes, and libtest discards a passing test's \
                     stdout, so this sentence is true in this repository and \
                     nowhere else. The passage: {window:.400}"
                );
            }
        }
    }

    /// The `MODEL_COVERAGE` heading's counted claim, made falsifiable.
    ///
    /// The heading states how many rows are `Agreed` and how many of those are
    /// misses. Nothing checked either number: the meta-test below walks the
    /// table row by row and asserts that each row's claimed outcome equals the
    /// observed one, which is a statement about the *rows* and says nothing
    /// about the sentence over them. So the sentence drifted — by nineteen, for
    /// several phases, and again by one within a day of being corrected.
    ///
    /// **Rejects: a counted claim over a machine-readable table with nothing
    /// counting the table.**
    #[test]
    #[cfg(feature = "proptest")]
    fn the_model_coverage_heading_counts_the_table() {
        let agreed = MODEL_COVERAGE
            .iter()
            .filter(|(_, outcome)| *outcome == ModelOutcome::Agreed)
            .count();

        // Derived from `REGISTRY` rather than written here: the rows that
        // *must* be `Agreed` are exactly the conformant variants, and a second
        // list of their names is a second thing to keep in step.
        let controls = REGISTRY
            .iter()
            .filter(|entry| entry.kind == Kind::ConformantVariant)
            .count();

        assert_eq!(
            agreed, 41,
            "`MODEL_COVERAGE`'s heading says forty-one rows are `Agreed` and the \
             table holds {agreed}. The heading is prose and this is what \
             makes it a claim, so correct the two together"
        );
        assert_eq!(
            agreed - controls,
            39,
            "`MODEL_COVERAGE`'s heading says thirty-nine misses and the table \
             holds {}. Every one of them is a defect the model cannot \
             express, and the heading's shape bullets are what say why",
            agreed - controls
        );

        // And the sentence itself, read out of this file. The two assertions
        // above pin the *table*; without this one the heading and the numbers
        // asserted here could still disagree, which is the shape the finding
        // was about — a claim beside a list, with nothing joining them. There
        // is no type that says "this doc comment states this count", so the
        // check reads the source (RS-81-1).
        //
        // # What this does not verify
        //
        // Only the two sentences it names, and only their numerals spelled as
        // words. A heading rewritten in different words fails loudly here
        // rather than passing silently, which is the right way round; what it
        // cannot see is whether the shape bullets beneath still describe the
        // misses they claim to. That is what `MODEL_ONLY_WITNESSES` and the
        // per-row assertions in `the_model_rule_rejects_exactly_what_it_claims`
        // are for.
        // Normalised first: a doc comment is wrapped, so the sentence being
        // looked for is split across `///` lines and no substring search over
        // the raw file can find it.
        let source: String = include_str!("mutation_coverage.rs")
            .replace("///", " ")
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        let rows = spelled(agreed);
        let misses = spelled(agreed - controls);

        let counted = format!("{}{} rows below are", rows[..1].to_uppercase(), &rows[1..]);
        assert!(
            source.contains(&counted),
            "the `MODEL_COVERAGE` heading must say {counted:?}, and it does not. \
             The table holds {agreed} `Agreed` rows; correct the sentence."
        );
        assert!(
            source.contains(&format!("**{misses} misses**")),
            "the `MODEL_COVERAGE` heading must say \"**{misses} misses**\", and \
             it does not. The table holds {agreed} `Agreed` rows of which \
             {controls} are conformant controls."
        );
    }

    /// A count spelled the way this file's prose spells it, for 0..=99.
    ///
    /// Written out rather than pulled in: the numbers a coverage heading states
    /// are the one thing a reader compares across releases, and a dependency
    /// added so that a test can say "thirty-eight" would be a dependency in the
    /// conformance crate's own test binary for the sake of one string.
    #[cfg(feature = "proptest")]
    fn spelled(n: usize) -> String {
        const UNITS: [&str; 20] = [
            "zero",
            "one",
            "two",
            "three",
            "four",
            "five",
            "six",
            "seven",
            "eight",
            "nine",
            "ten",
            "eleven",
            "twelve",
            "thirteen",
            "fourteen",
            "fifteen",
            "sixteen",
            "seventeen",
            "eighteen",
            "nineteen",
        ];
        const TENS: [&str; 10] = [
            "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
        ];

        assert!(
            n < 100,
            "this file's prose spells no number above ninety-nine"
        );
        if n < 20 {
            return UNITS[n].to_owned();
        }
        let (tens, unit) = (n / 10, n % 10);
        if unit == 0 {
            TENS[tens].to_owned()
        } else {
            format!("{}-{}", TENS[tens], UNITS[unit])
        }
    }

    /// The `REGISTRY` caution's discount, made falsifiable.
    ///
    /// The paragraph over the table tells a reviewer that the shotgun mutant's
    /// long `fails` list is *inflation rather than vacuity* — many of its rows
    /// evidence a rule's setup anchor rather than the property the rule is
    /// named for — and closes with the reassurance that no rule is covered by
    /// that mutant alone. That closing clause is what makes the discount safe
    /// to apply, and it is the one part of the paragraph nothing evaluates.
    ///
    /// Where the shotgun mutant *is* a rule's only evidence, the discount has
    /// to stop, and what stops it is an `expect` pin naming the assertion the
    /// mutant is supposed to trip. Without one, the day that rule acquires a
    /// second assertion its only registered evidence may be an anchor failure,
    /// CF-1's obligation is discharged in name only — and the paragraph told
    /// the auditor not to look.
    ///
    /// **Rejects: a reassurance aimed at exactly the audit it makes
    /// unnecessary.**
    #[test]
    fn the_shotgun_mutants_sole_coverage_is_pinned() {
        // Derived, then checked against the paragraph. The store whose rows a
        // reader is told to discount is the one that trips the most rules, so
        // the test follows the table; the equality below is what keeps the
        // paragraph and the table talking about the same store.
        let mut by_breadth: Vec<&Declared> = REGISTRY
            .iter()
            .filter(|entry| entry.kind == Kind::Mutant)
            .collect();
        by_breadth.sort_by_key(|entry| core::cmp::Reverse(entry.fails.len()));
        let shotgun = by_breadth[0];
        assert!(
            shotgun.fails.len() > by_breadth[1].fails.len(),
            "two mutants tie for the broadest `fails` list, so the caution over              `REGISTRY` no longer names a single store"
        );
        assert_eq!(
            shotgun.name, "InnerJoinTagStore",
            "the caution over `REGISTRY` names `InnerJoinTagStore` as the \
             mutant whose rows may be read as anchor inflation, and the \
             broadest mutant is now `{}` — so the paragraph is about a \
             different store than the table is",
            shotgun.name
        );

        let unpinned: Vec<&str> = all_rules()
            .into_iter()
            .filter(|rule| {
                let evidence: Vec<&str> = REGISTRY
                    .iter()
                    .filter(|entry| entry.kind == Kind::Mutant && entry.fails.contains(rule))
                    .map(|entry| entry.name)
                    .collect();
                evidence == [shotgun.name]
            })
            .filter(|rule| !shotgun.expect.iter().any(|(pinned, _)| pinned == rule))
            .collect();

        assert!(
            unpinned.is_empty(),
            "`{}` is the only evidence for {unpinned:?}, and its row pins no \
             assertion there. The caution over `REGISTRY` says its rows may \
             be discounted as anchor inflation; that discount is only safe \
             where the mutant is not the sole evidence, so a rule it alone \
             covers owes an `expect` pin naming the assertion it trips",
            shotgun.name
        );
    }

    /// CF-2. The registry and the store enumeration agree, and every name in it
    /// is real.
    #[test]
    fn mutant_registry_is_exhaustive() {
        let rules = all_rules();
        let registered = registered_names();

        for name in &registered {
            assert!(
                declared(name).is_some(),
                "`{name}` is enumerated in `for_each_mutant!` but has no \
                 `REGISTRY` row, so it is driven and nothing is claimed about it"
            );
        }

        for entry in REGISTRY {
            assert!(
                registered.contains(&entry.name),
                "`{}` has a `REGISTRY` row but is absent from \
                 `for_each_mutant!`, so its claim is never checked against a \
                 running store",
                entry.name
            );
        }

        let mut seen: Vec<&str> = REGISTRY.iter().map(|entry| entry.name).collect();
        let total = seen.len();
        seen.sort_unstable();
        seen.dedup();
        assert_eq!(
            seen.len(),
            total,
            "two `REGISTRY` rows share a name, so one of them is unreachable by \
             every lookup in this file"
        );

        // The same check on the other list. Both sides can duplicate and the two
        // duplications fail differently: a repeated `REGISTRY` row is shadowed by
        // `declared`'s `find`, while a store type repeated in `for_each_mutant!`
        // is *driven twice* and silently doubles its contribution to every
        // aggregate built from `reports()`.
        let mut driven = registered.clone();
        let driven_total = driven.len();
        driven.sort_unstable();
        driven.dedup();
        assert_eq!(
            driven.len(),
            driven_total,
            "a store type appears twice in `for_each_mutant!`, so it is driven \
             through every rule twice and counted twice"
        );

        // The typo catcher, and it is worth saying why it earns its own
        // assertion. A misspelled rule name in a `fails` list is invisible to
        // the other two meta-tests: `every_rule_has_a_mutant` only fires if the
        // *real* rule has no other mutant, and
        // `mutants_fail_exactly_their_declared_rules` would report it as
        // "declares a rule it does not fail" — true, and pointing at the wrong
        // thing. The universe comes from `__emit_rule_names`, so there is no
        // second list of rule names to keep in step.
        for entry in REGISTRY {
            for rule in entry.fails {
                assert!(
                    rules.contains(rule),
                    "`{}` declares `{rule}`, which is not a rule. Check the \
                     spelling against `for_each_event_store_rule!`",
                    entry.name
                );
            }

            // A pin on a rule the mutant does not declare never executes: the
            // pin check runs inside `assert_declared_failure`, which is only
            // reached for a declared rule. An unreachable pin reads as coverage
            // and is not, which is the same vacuity `expect` exists to prevent.
            for (pinned, _) in entry.expect {
                assert!(
                    entry.fails.contains(pinned),
                    "`{}` pins an assertion for `{pinned}`, which it does not \
                     declare as a failure — so the pin is never evaluated. \
                     Either add the rule to `fails` or drop the pin",
                    entry.name
                );
            }

            assert_kind_agrees_with_the_failure_list(entry);
        }

        assert!(
            REGISTRY
                .iter()
                .any(|entry| entry.kind == Kind::ConformantVariant),
            "no conformant variant is registered, so \
             `conformant_variants_pass_everything` asserts over nothing — which \
             is the vacuity of CF-5 reintroduced one level up"
        );
    }

    /// Each [`Kind`]'s claim about what `fails` may hold — and, for
    /// [`Kind::ModelOnlyMutant`], about what catches the store instead.
    ///
    /// Split out of `mutant_registry_is_exhaustive` rather than inlined,
    /// because the three arms are one question — *does this row's kind agree
    /// with the rest of the row* — and the enclosing test is a list of
    /// unrelated ones.
    fn assert_kind_agrees_with_the_failure_list(entry: &Declared) {
        match entry.kind {
            Kind::Mutant => assert!(
                !entry.fails.is_empty(),
                "`{}` is a mutant that fails nothing, which is a conformant \
                 store filed under the wrong kind",
                entry.name
            ),
            Kind::ModelOnlyMutant => {
                assert!(
                    entry.fails.is_empty(),
                    "`{}` is filed as caught only by the model family and \
                     declares a rule of the event-store family that it fails, \
                     so it is an ordinary mutant. Change the kind rather than \
                     the list",
                    entry.name
                );
                // Obligation two, and it is feature-**independent**, which is
                // the whole repair. The kind shipped with only the `cfg`-gated
                // obligation below, and an adversarial review registered
                // `HidingPlaceStore` — a store with no defect at all — under
                // this kind, with an empty `fails` list and a `MODEL_COVERAGE`
                // row claiming `Rejected`, and watched
                // `--no-default-features` report nine passes. `Kind::Mutant` is
                // unabusable because its bar is data in this same table;
                // borrowing a family for a bar means the bar can be compiled
                // out, and `cargo hack`'s powerset compiles exactly that.
                //
                // Presence here is only half of it: a row can be added as
                // easily as a claim. `every_model_only_mutant_demonstrates_its_defect`
                // is the other half — it runs the witness and compares the
                // store's own answer against `crate::correct`'s.
                assert!(
                    MODEL_ONLY_WITNESSES
                        .iter()
                        .any(|witness| witness.name == entry.name),
                    "`{}` is filed as caught only by the model family and carries no \
                     `MODEL_ONLY_WITNESSES` row, so nothing in this build has shown it is \
                     defective at all. Add a fixed scenario on which its own `Defect::select` \
                     disagrees with `crate::correct`",
                    entry.name
                );

                // Obligation three, and the one that needs the model family
                // itself: not merely *a* defect, but one this binary's model
                // actually rejects. It can only be checked where that family is
                // compiled, which is why obligation two above exists and does
                // not depend on it.
                #[cfg(feature = "proptest")]
                {
                    let claimed = MODEL_COVERAGE
                        .iter()
                        .find(|(row, _)| *row == entry.name)
                        .map(|(_, outcome)| *outcome);
                    assert_eq!(
                        claimed,
                        Some(ModelOutcome::Rejected),
                        "`{}` is filed as caught only by the model family, and \
                         `MODEL_COVERAGE` claims {claimed:?} rather than \
                         `Rejected` — so nothing in this binary catches it and \
                         the kind is a place to hide it",
                        entry.name
                    );
                }
            }
            Kind::ConformantVariant => assert!(
                entry.fails.is_empty(),
                "`{}` is a conformant variant that declares failures; a \
                 variant that fails a rule is either a mutant or evidence the \
                 rule is over-specified (CF-6)",
                entry.name
            ),
        }
    }

    /// Every [`Kind::ModelOnlyMutant`] really is defective, measured rather
    /// than claimed — in **every** feature configuration.
    ///
    /// The kind's other two obligations are both statements *about* a store: its
    /// `fails` list is empty, and `MODEL_COVERAGE` says the model rejects it.
    /// The first is true of a correct store as well, and the second is behind
    /// `proptest`. Between them they left the configuration `cargo hack` builds
    /// with no bar at all, and an adversarial review walked `HidingPlaceStore`
    /// straight through it.
    ///
    /// This runs the store's own [`crate::mutants::Defect::select`] against a
    /// fixed log and read, and requires it to answer differently from
    /// [`crate::correct::select`]. No generator, no runtime, no feature — the
    /// same shape as the experiment crate's `defect_is_real.rs`, which exists
    /// for the same reason one family over.
    ///
    /// Both directions, because an orphan witness is a scenario nobody runs
    /// against anything and reads as coverage.
    #[test]
    fn every_model_only_mutant_demonstrates_its_defect() {
        for entry in REGISTRY
            .iter()
            .filter(|entry| entry.kind == Kind::ModelOnlyMutant)
        {
            let witness = MODEL_ONLY_WITNESSES
                .iter()
                .find(|witness| witness.name == entry.name)
                .unwrap_or_else(|| {
                    panic!(
                        "`{}` is a `ModelOnlyMutant` with no witness; \
                         `mutant_registry_is_exhaustive` owns this failure",
                        entry.name
                    )
                });

            let (log, query, options) = (witness.scenario)();
            let observed = (witness.select)(&log, &query, options);
            let correct = crate::correct::select(&log, &query, options);

            let disagrees = match &observed {
                Ok(events) => events != &correct,
                // A read that fails where the correct one succeeds is a
                // disagreement too, and the sharpest kind.
                Err(_) => true,
            };
            assert!(
                disagrees,
                "`{}` is filed as a defect only the model family can see, and its own witness \
                 does not show a defect at all: reading {query:?} under {options:?} it answered \
                 exactly what `crate::correct` answers, {correct:?}. Either the witness is the \
                 wrong scenario or the store has stopped being wrong",
                entry.name
            );
        }

        for witness in MODEL_ONLY_WITNESSES {
            let owner = declared(witness.name);
            assert!(
                owner.is_some_and(|entry| entry.kind == Kind::ModelOnlyMutant),
                "`{}` has a `MODEL_ONLY_WITNESSES` row and is not a registered \
                 `ModelOnlyMutant`, so the scenario is never evaluated against anything and \
                 reads as coverage",
                witness.name
            );
        }
    }

    /// The positive control on the bar above: a store with **no defect** cannot
    /// satisfy it.
    ///
    /// Without this, `every_model_only_mutant_demonstrates_its_defect` is
    /// satisfied by a comparison that always reports a disagreement — the same
    /// vacuity `conformant_variants_pass_everything` exists to rule out one
    /// table over. [`crate::mutants::HidingPlaceStore`] is the review's own
    /// refutation, kept in the tree and driven through **every** scenario the
    /// witness table holds; it agrees with `crate::correct` on all of them, so
    /// no witness for it could be written.
    ///
    /// What this does and does not prove: it does not prove that *no* scenario
    /// anywhere would separate a defect-free store from the reference
    /// implementation — nothing could, and the two are the same function. It
    /// proves the comparison is a real one, over the exact inputs the bar
    /// accepts today.
    ///
    /// # Why the guard is an equivalence and not `!is_empty()`
    ///
    /// It was `assert!(!MODEL_ONLY_WITNESSES.is_empty())`, whose whole content
    /// was *this control must actually run*. That is right while the kind has a
    /// member and wrong the moment it does not: with no
    /// [`Kind::ModelOnlyMutant`] registered, the bar this controls
    /// (`every_model_only_mutant_demonstrates_its_defect`) iterates over nothing
    /// too, so a control that ran would be controlling nothing — and a bare
    /// `!is_empty()` would demand a witness for a store that must not have one.
    ///
    /// The equivalence says the same thing in both states: **the witness table
    /// is empty exactly when the kind has no members.** A row without a member
    /// is an orphan scenario, which is coverage theatre; a member without a row
    /// is a defect nothing has shown to be a defect, which is the hole
    /// `HidingPlaceStore` walked through. Both directions stay checked, and the
    /// day the kind is used again this control is non-vacuous with it.
    #[test]
    fn the_model_only_bar_rejects_a_store_with_no_defect() {
        assert_eq!(
            MODEL_ONLY_WITNESSES.is_empty(),
            !REGISTRY
                .iter()
                .any(|entry| entry.kind == Kind::ModelOnlyMutant),
            "the witness table and the `ModelOnlyMutant` rows in `REGISTRY` must be empty or              non-empty together: {} witness row(s) against {} registered member(s). A row with              no member is a scenario nobody evaluates; a member with no row is a defect nothing              has shown to be one",
            MODEL_ONLY_WITNESSES.len(),
            REGISTRY
                .iter()
                .filter(|entry| entry.kind == Kind::ModelOnlyMutant)
                .count(),
        );

        for witness in MODEL_ONLY_WITNESSES {
            let (log, query, options) = (witness.scenario)();
            let hiding = <crate::mutants::HidingPlaceStore as crate::mutants::Defect>::select(
                &log, &query, options,
            )
            .expect("the correct read path is infallible");
            assert_eq!(
                hiding,
                crate::correct::select(&log, &query, options),
                "`HidingPlaceStore` overrides no step of `Defect` and must therefore answer \
                 every read exactly as `crate::correct` does. It did not, on `{}`'s scenario — \
                 which means the witness comparison is measuring something other than the \
                 defect, and every row it passes is suspect",
                witness.name
            );
        }
    }

    /// The rules a fixture buys by lying about `REOPEN` — the ones it converts
    /// from reported skips into passes.
    ///
    /// The whole of this suite's durability certification, and the reason the
    /// incentive runs backwards. Written out rather than derived, so that a rule
    /// leaving the set is an edit somebody makes on purpose; every name is
    /// checked against `for_each_event_store_rule!` below, so a typo is a
    /// failure rather than a silently empty claim.
    const CERTIFIED_BY_OVER_CLAIMING_REOPEN: &[&str] = &[
        "acknowledged_writes_survive_a_reopen",
        "reopened_store_does_not_reissue_an_event_id",
        "recorded_time_survives_a_reopen",
    ];

    /// CF-17's hazard, recorded as what it is: **nothing detects it**, and the
    /// two consequences that *are* measurable are measured here.
    ///
    /// # Why this is a test and not a `REGISTRY` row
    ///
    /// It was a row. `NoopReopenFixture` — `REOPEN: SUPPORTED`, an empty
    /// `reopen`, over a completely correct but entirely volatile store — cannot
    /// be a [`Kind::Mutant`], because it fails nothing and that assertion is
    /// right and was not weakened. It was given a fourth kind whose obligation
    /// was a **scenario separating it from an honest fixture**, and an
    /// adversarial review showed that obligation to be wrong twice over: the
    /// row's two `fn() -> String` pointers had no tie to the store named in it,
    /// so two string literals satisfied the bar; and the scenario that was
    /// written separated two *styles of `reopen` implementation* rather than
    /// honest from defective.
    ///
    /// The second finding is the one that killed the kind rather than the row.
    /// [`crate::mutants::ClosingFixture`] and
    /// [`crate::mutants::LiveHandleReopenFixture`] are **both honest**, and they
    /// answer the stale-handle scenario differently; the liar answers with the
    /// second. `LiveHandleReopenFixture` is `SqliteFixture` in miniature — the
    /// workspace's only real durable adapter fixture — so a rule written from
    /// that partition rejects `happenstance-sqlite`. The audit said this in
    /// advance: *a correct `reopen` over a durable medium and an empty one over
    /// a `Vec` produce byte-identical observations through `EventStore`*. If
    /// that is true then **no scenario can demonstrate this defect**, and an
    /// obligation demanding one is unsatisfiable in principle. The honest move
    /// was to withdraw the obligation, not to keep hunting for a scenario.
    ///
    /// # What is left, and it is not nothing
    ///
    /// Three assertions, all measured rather than asserted:
    ///
    /// 1. The liar **fails no rule**, driven through every one of them. That is
    ///    the claim "nothing sees it", and it goes red the day something does.
    /// 2. It converts [`CERTIFIED_BY_OVER_CLAIMING_REOPEN`] from reported skips
    ///    into passes, while its honest twin — the same store, one line apart,
    ///    declining `REOPEN` — reports them as skips. That is the incentive
    ///    inversion, in-tree and reproducible, rather than a score quoted from
    ///    an experiment nobody in review ran.
    /// 3. Two honest fixtures land on opposite sides of the sharpest partition
    ///    anyone has proposed, with the liar on one of them. That is the
    ///    standing falsifier for every reopen rule that will ever be proposed,
    ///    and it is why CF-17's obligation is a MUST no rule enforces.
    ///
    /// What is **not** here is a check that the defect is real. Nothing in this
    /// workspace can supply one, and a test that appeared to would be worse than
    /// this comment.
    #[test]
    fn reopen_over_claiming_is_undetectable_and_this_is_the_record() {
        let rules = all_rules();
        for rule in CERTIFIED_BY_OVER_CLAIMING_REOPEN {
            assert!(
                rules.contains(rule),
                "`{rule}` is not a rule of this family. Check the spelling against \
                 `for_each_event_store_rule!` — a claim naming a rule that does not exist is \
                 evaluated against nothing"
            );
        }

        // 1 — the liar fails nothing, over every rule there is.
        let liar = run_subject::<crate::mutants::NoopReopenFixture>();
        let rejected: Vec<&str> = liar
            .outcomes
            .iter()
            .filter(|(_, verdict)| matches!(verdict, Verdict::Panicked { .. }))
            .map(|(rule, _)| *rule)
            .collect();
        assert!(
            rejected.is_empty(),
            "`NoopReopenFixture` declares `REOPEN` supported over a store that reopens nothing, \
             and this record says no rule of this family can reject it. These did: {rejected:?}. \
             That is good news and it means the record is out of date — promote the fixture to a \
             `Kind::Mutant` row declaring exactly these rules, and delete this test rather than \
             relaxing it"
        );

        // 2 — the incentive inversion: the same store, one line apart.
        let honest = run_subject::<crate::mutants::HonestVolatileFixture>();
        for rule in CERTIFIED_BY_OVER_CLAIMING_REOPEN {
            let lied = liar
                .verdict(rule)
                .unwrap_or_else(|| panic!("`{rule}` was not driven against the liar"));
            assert!(
                matches!(lied, Verdict::Passed),
                "`{rule}` is claimed as one an over-claiming fixture converts into a pass, and \
                 against `NoopReopenFixture` it {}",
                lied.describe()
            );

            let told_the_truth = honest
                .verdict(rule)
                .unwrap_or_else(|| panic!("`{rule}` was not driven against the honest twin"));
            assert!(
                matches!(told_the_truth, Verdict::Skipped { .. }),
                "`{rule}` must be a reported *skip* against a fixture that honestly declines \
                 `REOPEN`, or there is no inversion to report: the liar and the honest twin are \
                 the same store one line apart. It {}",
                told_the_truth.describe()
            );
        }

        // 3 — the falsifier. Two honest fixtures, opposite answers, and the liar
        //     sitting with one of them.
        let replaying = crate::mutants::closing_reopen_observed();
        let live_handle = crate::mutants::live_handle_reopen_observed();
        let lying = crate::mutants::noop_reopen_observed();
        assert_ne!(
            replaying, live_handle,
            "the two honest reopen styles must answer the stale-handle scenario differently, or \
             this assertion has stopped being the falsifier it exists to be. `ClosingFixture` \
             replaces the live log; `LiveHandleReopenFixture` does not, because reopening a file \
             does not replace the file"
        );
        assert_eq!(
            lying, live_handle,
            "`NoopReopenFixture` must answer exactly as an HONEST durable-shaped fixture does. \
             That equality is the whole reason no rule may be written here: \
             `LiveHandleReopenFixture` is `SqliteFixture` in miniature, and a rule that rejected \
             the liar on this observation would reject the workspace's only durable adapter with \
             it. If this has stopped holding, something has become observable and CF-17 may be \
             enforceable after all — which is a finding, not a test to adjust"
        );
    }

    /// CF-3. Both directions: a mutant fails every rule it declares, and every
    /// rule it does not declare either **passes** or **skips for a reason the
    /// fixture stated in advance**.
    ///
    /// The second half is the load-bearing one. A mutant that fails everything
    /// proves nothing about the rule it was written for; it proves the store is
    /// broken. Exactness is what makes the registry a map from rules to the bugs
    /// they catch.
    ///
    /// # Why the second half is not simply "passes"
    ///
    /// Because a third of this table cannot. Every `MutantFixture<_>` declines
    /// `REOPEN` — a `Vec` behind an `Rc` has no durable medium — so
    /// `acknowledged_writes_survive_a_reopen` *skips* against all of them. Those
    /// cells are blank, honestly and unfixably, and the earlier spelling of this
    /// test read a blank cell as a pass: it asserted only that the verdict was
    /// not a panic, so a rule that never executed a line counted as evidence the
    /// mutant was correct there.
    ///
    /// The fix is not to demand a pass, it is to make the hole a **checked**
    /// consequence of something the fixture declared. A skip must carry a
    /// `(capability, reason)` pair the fixture itself declines; a skip arriving
    /// from anywhere else means a rule stopped running and nothing noticed.
    /// Between that and `assert_declared_failure`'s treatment of the declared
    /// side — where a skip is a hole in the map and never a pass — a `require!`
    /// gate added to a rule by accident has no green path through this test.
    #[test]
    fn mutants_fail_exactly_their_declared_rules() {
        for report in reports() {
            let Some(entry) = declared(report.name) else {
                continue; // `mutant_registry_is_exhaustive` owns this failure.
            };
            // `Kind::ModelOnlyMutant` runs through this loop too, and with an
            // empty `fails` list every rule takes the undeclared arm — which is
            // exactly the claim that kind makes: *no rule of this family sees
            // it*. Filing it as a mutant caught elsewhere would be worth nothing
            // if the "elsewhere" were assumed rather than measured here.
            if entry.kind == Kind::ConformantVariant {
                continue;
            }

            for (rule, verdict) in &report.outcomes {
                if entry.fails.contains(rule) {
                    assert_declared_failure(entry, rule, verdict);
                } else {
                    assert_undeclared_outcome(entry, &report.declines, rule, verdict);
                }
            }
        }
    }

    /// The undeclared half of CF-3: a pass, or a skip the fixture accounted for.
    fn assert_undeclared_outcome(
        entry: &Declared,
        declines: &[(&'static str, &'static str)],
        rule: &str,
        verdict: &Verdict,
    ) {
        match verdict {
            Verdict::Passed => {}
            Verdict::Skipped { capability, reason } => assert!(
                declines.contains(&(*capability, *reason)),
                "`{}` skipped `{rule}` citing `{capability}`, which its fixture \
                 does not decline — so this rule stopped running for a reason \
                 nothing in the registry accounts for, and the cell that reads \
                 \"passes every rule it does not declare\" is empty. Declines: \
                 {declines:?}",
                entry.name
            ),
            Verdict::Panicked { .. } => panic!(
                "`{}` failed `{rule}`, which it does not declare. Either the \
                 mutant is broken in more ways than it claims — the commonest \
                 way a mutant set decays — or the declaration is short a line. \
                 Saw: {}",
                entry.name,
                verdict.describe()
            ),
        }
    }

    /// The declared-failure half of CF-3, including CF-2's *wrong reason*
    /// hazard.
    fn assert_declared_failure(entry: &Declared, rule: &str, verdict: &Verdict) {
        let Verdict::Panicked { message, origin } = verdict else {
            // Three outcomes, three readings, and conflating the last two is
            // exactly what breaks CF-3. A rule that *passed* means the defect is
            // invisible to it and the declaration is wrong. A rule that
            // *skipped* never executed a line, so it is neither a pass nor a
            // failure — it is a hole in the map, and reading it as a pass is how
            // a mutant whose fixture declines `REOPEN` comes to look like proof
            // about `acknowledged_writes_survive_a_reopen`.
            panic!(
                "`{}` declares that it fails `{rule}`, but the rule {}. {}",
                entry.name,
                verdict.describe(),
                match verdict {
                    Verdict::Skipped { .. } =>
                        "A declared failure that never ran is a hole in the map, \
                         not a pass: either the mutant's fixture must supply the \
                         capability, or the declaration belongs on a different \
                         rule",
                    _ =>
                        "Either the rule does not catch this defect after all — \
                          which makes it decorative for this mutant — or the \
                          declaration names the wrong rule",
                }
            );
        };

        match entry.mode {
            FailureMode::Assertion => {
                // The positive half, and the one that carries the claim. Every
                // rule assertion in the suite is raised in `suite.rs`; anything
                // raised elsewhere is the store, the fixture contract, or the
                // testkit falling over, and none of those is the rule rejecting
                // this defect. Measured: deleting `LosingFixture`'s `reopen`
                // override removes its modelled defect outright, and the panic
                // that follows comes from `Fixture::reopen`'s provided body in
                // `contract.rs` — which the substring denylist below passes
                // silently, leaving the whole durability axis vacuous and green.
                assert!(
                    origin.as_ref().is_some_and(Origin::is_a_rule_body),
                    "`{}` is declared to fail `{rule}` by the rule's own \
                     assertion, but the panic was not raised in the suite. A \
                     mutant that fails the right rule for the wrong reason reads \
                     as proof and is not one — and a mutant whose modelled defect \
                     has been deleted fails this way, from the fixture contract's \
                     own \"not implemented\" body. Saw: {}",
                    entry.name,
                    verdict.describe()
                );

                let runtime = RUNTIME_PANICS
                    .iter()
                    .find(|needle| message.contains(**needle));
                assert!(
                    runtime.is_none(),
                    "`{}` is declared to fail `{rule}` by the rule's own \
                     assertion, but the rule fell over instead ({:?} in the \
                     message, raised at its own line). Message: {message}",
                    entry.name,
                    runtime.copied().unwrap_or_default()
                );
            }
            FailureMode::StorePanic(expected) => {
                // The mirror of the `Assertion` arm's positive check, and it was
                // missing until stage 5. Without it `StorePanic` is a bare
                // substring test: any panic anywhere whose message contains the
                // needle certifies the row — including one of `suite.rs`'s own
                // assertions, which is precisely the substitution CF-2 forbids.
                // It bites hardest on the two rows that replaced the deleted
                // `#[should_panic]` drivers, whose needles ("already borrowed",
                // "already mutably borrowed") are themselves on `RUNTIME_PANICS`
                // — the file already classifies those strings as "the store fell
                // over", so a rule asserting one would be the wrong reason
                // wearing the right words.
                assert!(
                    origin.as_ref().is_none_or(|at| !at.is_a_rule_body()),
                    "`{}` is declared to fail `{rule}` by the store panicking, \
                     but the panic was raised in the suite — so a rule assertion \
                     is standing in for the store falling over. Saw: {}",
                    entry.name,
                    verdict.describe()
                );

                assert!(
                    message.contains(expected),
                    "`{}` is declared to fail `{rule}` by panicking with \
                     {expected:?}, but the message was: {message}",
                    entry.name
                );
            }
        }

        // Applies under both modes: `mode` says *how* the rule rejected the
        // store, `expect` says *which* of the rule's assertions — or which of a
        // `RefCell`'s two borrow messages — did it.
        if let Some((_, expected)) = entry.expect.iter().find(|(pinned, _)| *pinned == rule) {
            assert!(
                message.contains(expected),
                "`{}` is declared to fail `{rule}` at {expected:?}, but a \
                 different assertion fired. The mutant still fails the rule, so \
                 nothing above catches this — and what its provenance claims it \
                 demonstrates is no longer what it demonstrates. Message: \
                 {message}",
                entry.name
            );
        }
    }

    /// CF-4. Every registered store names the real shape that makes it
    /// plausible.
    ///
    /// This is what rejects the saboteur — `struct AlwaysWrong` — which
    /// satisfies CF-1 mechanically and proves nothing, because no author would
    /// have written it. The mutant that earns its place is the one someone would
    /// ship.
    ///
    /// Conformant variants are held to it too: a variant owes an account of
    /// *why it is legally different*, or it is just a second copy of the
    /// reference store.
    #[test]
    fn every_mutant_states_its_provenance() {
        for entry in REGISTRY {
            assert!(
                !entry.provenance.trim().is_empty(),
                "`{}` states no provenance. Name the adapter shape or the \
                 scenario that makes it plausible; where the provenance is \"a \
                 reviewer measured this passing the suite\", say so",
                entry.name
            );
        }
    }

    /// CF-5 and CF-6. The positive control.
    ///
    /// Without it, every test above is satisfied by a harness that reports
    /// failure unconditionally — which is the same vacuity in a new place.
    ///
    /// Two assertions, and the second is the one that keeps the control honest
    /// as the suite grows: it is not enough that no variant failed, every rule
    /// must have *executed* against at least one of them. A control that skipped
    /// half the suite is a control over half the suite.
    #[test]
    fn conformant_variants_pass_everything() {
        let variants: Vec<_> = reports()
            .into_iter()
            .filter(|report| {
                declared(report.name).is_some_and(|entry| entry.kind == Kind::ConformantVariant)
            })
            .collect();

        assert!(
            !variants.is_empty(),
            "no conformant variant ran; see `mutant_registry_is_exhaustive`"
        );

        for report in &variants {
            for (rule, verdict) in &report.outcomes {
                assert!(
                    !matches!(verdict, Verdict::Panicked { .. }),
                    "`{}` is conformant — it differs from `MemoryEventStore` \
                     only in ways the specification permits — and `{rule}` \
                     rejected it. **This is a finding about the rule, not about \
                     the store** (CF-6): the rule asserts something the \
                     specification allows an adapter to do differently, most \
                     likely a literal position value or an assumption that \
                     positions are dense. Fix the rule. Saw: {}",
                    report.name,
                    verdict.describe()
                );
            }
        }

        for rule in all_rules() {
            assert!(
                variants
                    .iter()
                    .any(|report| matches!(report.verdict(rule), Some(Verdict::Passed))),
                "`{rule}` never ran against a conformant variant — every variant \
                 skipped it — so nothing here says the rule accepts a legal \
                 store"
            );
        }
    }

    /// The rules that must *reject* a fixture declining a capability rather than
    /// skip it, which is what makes the MUST enforced rather than merely stated.
    ///
    /// A slice rather than a constant, and the change was forced by a rule rather
    /// than chosen: slice F's `head_advances_across_two_handles` is the second
    /// rule to spell `must!(F: SECOND_HANDLE)`, and a singleton here made the
    /// second one read as "a rule that ignored its `require!` gate". Every entry
    /// is a rule using `must!` rather than `require!`; a rule that gains a `must!`
    /// and is not added here fails `capability_skips_are_reported` with a message
    /// pointing at the rule, which is the right way round.
    ///
    /// At module scope rather than inside the test for a reason that is only
    /// arithmetic: with [`MUST_SKIP`] beside it the body crossed
    /// `clippy::too_many_lines`. Nothing else moved with them — in particular they
    /// are still not declared beside their use, which is what
    /// `clippy::items_after_statements` was asking for.
    const MUST_REJECT: &[&str] = &[
        "two_handles_observe_each_others_appends",
        "head_advances_across_two_handles",
    ];

    /// [`MUST_REJECT`]'s mirror: every rule that *is* gated on a capability a
    /// fixture may honestly decline, and therefore must report a skip rather than
    /// pass.
    ///
    /// A slice for the same reason, and it was a singleton — naming
    /// `acknowledged_writes_survive_a_reopen` as "the suite's one genuinely
    /// optional rule" — until phase 4 landed four more. Two rules on `REOPEN` and
    /// two on `MID_BATCH_FAULT` were then reaching this test as rules nothing
    /// checked had run at all, which is the hole CF-18 exists to close: a rule
    /// whose `require!` gate was deleted passes here in silence.
    /// `append_reports_exceeded_store_limits` is on the list too, and its gate is
    /// CF-40's three `Option<usize>` ceilings rather than a `Capability` — the
    /// reporting obligation is identical either way.
    const MUST_SKIP: &[&str] = &[
        "acknowledged_writes_survive_a_reopen",
        "reopened_store_does_not_reissue_an_event_id",
        "recorded_time_survives_a_reopen",
        "append_is_atomic_under_a_mid_batch_fault",
        "arming_a_mid_batch_fault_makes_the_append_fail",
        "append_reports_exceeded_store_limits",
    ];

    /// CF-18. A capability-gated rule is still emitted, still answered, and
    /// reports the fixture's own reason.
    ///
    /// # Which half of CF-18 this proves, and which it cannot
    ///
    /// libtest exposes nothing programmatically: a `#[test]` cannot ask how many
    /// tests the binary holds, cannot enumerate them, and cannot read another
    /// test's stdout. So this test drives the enumeration itself and asserts on
    /// [`Verdict`] **values**.
    ///
    /// That proves the *answering* half — every registered rule produces an
    /// outcome against a fixture that declines everything, the gated ones carry
    /// the fixture's own words, and none of them silently passes. It does **not**
    /// prove the *emission* half — that each of those rules appears as a test in
    /// a harness's binary — which is only checkable from outside the process,
    /// via `cargo test -- --list`. That belongs to an `xtask` step, not to a
    /// `#[test]`.
    #[test]
    fn capability_skips_are_reported() {
        let rules = all_rules();

        // ---- The MUST, and where it is really enforced ----------------------
        //
        // `SECOND_HANDLE` is spelled as a capability so a rule needing it has
        // somewhere to ask, but declining it is not a trade — it is a fixture
        // that does not meet CF-16 — and the *type* cannot tell the two apart,
        // because both are `Capability`. That gap is closed by `must!` in
        // `suite.rs`, inside `two_handles_observe_each_others_appends`, and it
        // has to be closed there: this file's meta-tests never run in an
        // adapter's CI, which is precisely where the fixture declining a MUST
        // lives. The demonstration is a few lines below, over `DecliningFixture`.
        //
        // **This loop is a guard on future registrations and nothing more, and
        // it is worth saying so rather than letting it read as a demonstrated
        // rejection.** Every store in `for_each_mutant!` hard-codes
        // `Capability::SUPPORTED`, so no registered store can fire it today; the
        // one fixture in this binary that declines the MUST is deliberately
        // outside its universe (see `DecliningFixture`). It fires the day someone
        // registers an instrument that declines it, which is the day a mutant
        // would otherwise start skipping rules it is supposed to fail.
        for (name, capability) in registered_second_handle() {
            assert!(
                capability.is_supported(),
                "`{name}` is registered in `for_each_mutant!` and declines \
                 `SECOND_HANDLE`, which is a MUST rather than a trade: every \
                 rule that needs two handles will now panic against it, so its \
                 registry row cannot mean what it says. Reason given: {:?}",
                capability.reason()
            );
        }

        // ---- A fixture that declines everything still answers everything ----
        let declining = run_subject::<DecliningFixture>();

        let answered: Vec<&str> = declining.outcomes.iter().map(|(rule, _)| *rule).collect();
        assert_eq!(
            answered.len(),
            rules.len(),
            "a fixture declining every capability must still produce an outcome \
             for every registered rule; `#[cfg]`-ing one out would make it \
             indistinguishable in CI output from a rule that passed"
        );
        for rule in &rules {
            assert!(
                answered.contains(rule),
                "`{rule}` produced no outcome against a fixture that declines \
                 everything"
            );
        }

        for (rule, verdict) in &declining.outcomes {
            match verdict {
                Verdict::Passed => {}
                Verdict::Skipped { reason, .. } => assert!(
                    *reason == DecliningFixture::SECOND_HANDLE_REASON
                        || *reason == DecliningFixture::REOPEN_REASON
                        || *reason == DecliningFixture::MID_BATCH_FAULT_REASON
                        || *reason == DecliningFixture::READ_FAULT_REASON
                        // CF-40: a fixture with no ceiling reports the testkit's
                        // reason rather than its own, because the sentence is the
                        // same for every store that has none. See
                        // `contract::NO_CEILING_REASON`.
                        || *reason == happenstance_testkit::NO_CEILING_REASON,
                    "`{rule}` skipped with a reason the fixture never gave: \
                     {reason:?}. The reason is the only record of the trade, so \
                     it has to be the adapter's own words"
                ),
                Verdict::Panicked { message, .. } => {
                    assert!(
                        MUST_REJECT.contains(rule),
                        "`{rule}` panicked against a fixture that is correct in \
                         every respect except its declared capabilities. If it \
                         opened a second handle, it ignored its `require!` gate. \
                         Message: {message}"
                    );
                    assert!(
                        message.contains(DecliningFixture::SECOND_HANDLE_REASON),
                        "`{rule}` must reject a fixture declining the \
                         `SECOND_HANDLE` MUST *and carry the fixture's own stated \
                         reason*, so the failing build says why. Message: \
                         {message}"
                    );
                }
            }
        }

        // The demonstrated half: `DecliningFixture` is the wrong implementation
        // that `must!` rejects, so the MUST is not an assertion nothing can fire.
        // Every rule in the list, not just the first: a second `must!` that
        // quietly skipped would be a MUST enforced on one rule and stated on the
        // other.
        for rule in MUST_REJECT {
            assert!(
                matches!(declining.verdict(rule), Some(Verdict::Panicked { .. })),
                "`{rule}` must *fail* a fixture that declines `SECOND_HANDLE`, \
                 not skip it. A skip there is the outcome `contract.rs` names as \
                 the thing to prevent: a green suite plus one SKIP line for an \
                 adapter nothing reached through two connections. Saw: {:?}",
                declining.verdict(rule)
            );
        }

        let skipped = declining.skipped();
        for rule in MUST_SKIP {
            assert!(
                skipped.contains(rule),
                "`{rule}` is gated on a capability this fixture declines, so it \
                 must report a skip rather than pass. Without at least one such \
                 rule the skip machinery is untested and CF-18 is a claim about \
                 code nothing executes — and a rule that lost its gate passes \
                 here silently, which is the same vacuity one level up. Saw \
                 {skipped:?}"
            );
        }
        for rule in MUST_REJECT {
            assert!(
                !skipped.contains(rule),
                "`{rule}` reported a skip, so `must!` has been downgraded back \
                 to `require!` and the MUST is unenforced again"
            );
        }

        // ---- And a fully capable fixture skips nothing ---------------------
        let capable = run_subject::<GappedPositionFixture>();
        assert!(
            capable.skipped().is_empty(),
            "`{}` supports every capability, so nothing may be skipped against \
             it — a skip here means a rule's `require!` gate reads the wrong \
             const: {:?}",
            capable.name,
            capable.skipped()
        );
    }

    /// The projection family's [`MUST_REJECT`]: every projection rule that
    /// spells `must!` rather than `require!`.
    ///
    /// Thirteen of the family's seventeen rules are on it, and every one belongs
    /// there rather than being gated with `require!`, because every one reads
    /// the read model or the checkpoint back through a **fresh handle**. A
    /// projection fixture that cannot open a second handle cannot observe PS-1 —
    /// the coupling the whole port exists for — at all, so declining it is a
    /// fixture that does not meet the contract rather than a trade the suite may
    /// record and move past.
    ///
    /// Two rules on it *also* spell a `require!` — `failed_commit_leaves_both_unchanged`
    /// on `COMMIT_FAULT`, and `refused_reset_changes_nothing` on
    /// `RESET_REFUSAL`, which are the family's two declinable gates. Both are
    /// here rather than in [`PROJECTION_MUST_SKIP`] because both spell the
    /// `must!` **first**, deliberately: a fixture declining both is failing the
    /// contract, and reporting that as a skip would file a broken fixture under
    /// a trade it was entitled to make. The skip path is therefore demonstrated
    /// against the *reference* fixture, which supports `SECOND_HANDLE` and
    /// declines both of the others — see the tail of
    /// [`projection_capability_skips_are_reported`].
    ///
    /// **The four absentees are absent on purpose, and the absence is the
    /// interesting part.** `commit_rejects_a_foreign_batch` wants two *isolated
    /// stores*, not two handles onto one, and two `open()` calls on the
    /// `impl AsyncFn() -> F` every rule is handed already produce them (PS-15
    /// says so in as many words). `batch_reads_reflect_pending_writes` reads
    /// through the *same* handle that owns the batch, by construction, and
    /// `rebuild_is_chunk_size_invariant` opens a fixture per run and connects
    /// once to each. The fourth is `fresh_projection_has_no_checkpoint`, the
    /// rule that landed last: it reads one checkpoint through one handle and
    /// writes nothing at all, so the capabilities a fixture might decline are
    /// not the ones it spends. All four therefore spell no `must!`, run against
    /// a fixture declining everything, and **pass** — which is what this list
    /// asserts by omission, through the `Verdict::Passed` arm of
    /// [`assert_projection_declension`]. A rule added to this list "for safety"
    /// would make that assertion unreachable.
    ///
    /// A slice rather than a constant for [`MUST_REJECT`]'s reason.
    const PROJECTION_MUST_REJECT: &[&str] = &[
        "commit_advances_the_checkpoint",
        "commit_is_atomic_with_the_read_model",
        "failed_commit_leaves_both_unchanged",
        "rollback_leaves_both_unchanged",
        "dropped_batch_leaves_store_usable",
        "commit_accepts_a_position_the_batch_did_not_write",
        "commit_rejects_a_regressing_position",
        "distinct_projections_advance_independently",
        "reset_clears_rows_and_checkpoint_together",
        "reset_is_scoped_to_one_projection",
        "refused_reset_changes_nothing",
        "reset_is_not_commit_at_first",
        "rebuilding_is_distinguishable_from_live",
    ];

    /// [`PROJECTION_MUST_REJECT`]'s mirror **against this instrument**, and it
    /// is empty for a reason that is now about ordering rather than about
    /// absence.
    ///
    /// Two projection rules spell `require!` —
    /// `failed_commit_leaves_both_unchanged`, gated on `COMMIT_FAULT`, and
    /// `refused_reset_changes_nothing`, gated on `RESET_REFUSAL` — and neither
    /// skips here, because both spell `must!(F: SECOND_HANDLE)` first and this
    /// instrument declines that too. So against a fixture that declines
    /// everything they panic, which is the correct answer and why they are in
    /// [`PROJECTION_MUST_REJECT`] instead.
    ///
    /// **The skip arm is therefore demonstrated at the other end of this test,
    /// against the reference fixture**, on real values: `MemoryProjectionFixture`
    /// supports `SECOND_HANDLE`, declines `COMMIT_FAULT` and `RESET_REFUSAL`
    /// with its own stated reasons, and reports exactly two skips. That is a
    /// change from the state this list was written in, when nothing in the
    /// workspace could demonstrate a projection skip at all.
    ///
    /// This list stays, and stays empty, as the guard on the *next* gate. It was
    /// written expecting `RESET_REFUSAL`'s rule to be the first entry, and that
    /// prediction was wrong for a reason worth keeping: the rule that arrived
    /// spells the MUST first, so it rejects this instrument rather than skipping
    /// against it. The next candidate is a rule gated on a declinable capability
    /// **without** a `must!` above it — `ProjectionProbe::READS_THROUGH_BATCH`'s
    /// `batch_reads_reflect_pending_writes` (`read-through-and-rebuild-rules`)
    /// is the nearest — which starts skipping here and fails the two-direction
    /// check below until it is listed.
    const PROJECTION_MUST_SKIP: &[&str] = &[];

    /// Every outcome against the declining instrument is one that instrument's
    /// own declarations explain.
    ///
    /// A free function rather than a block inside
    /// [`projection_capability_skips_are_reported`] for the reason
    /// [`assert_undeclared_outcome`] is one: with it inlined the test body
    /// crossed `clippy::too_many_lines`, and a lint suppression there would be
    /// the wrong trade — the arms below are the content of CF-18 and each of
    /// them names what a green build would otherwise have hidden.
    fn assert_projection_declension(rule: &str, verdict: &Verdict) {
        match verdict {
            Verdict::Passed => assert!(
                !PROJECTION_MUST_REJECT.contains(&rule),
                "`{rule}` is listed in `PROJECTION_MUST_REJECT` and passed \
                 against a fixture declining `SECOND_HANDLE`, so its `must!` \
                 gate has been deleted or reads the wrong const"
            ),
            Verdict::Skipped { reason, .. } => {
                assert!(
                    PROJECTION_MUST_SKIP.contains(&rule),
                    "`{rule}` reported a skip and is not in \
                     `PROJECTION_MUST_SKIP`. A rule that gains a `require!` gate \
                     has to be listed there, or the list rots into decoration as \
                     later stories add rules"
                );
                assert!(
                    *reason == DecliningProjectionFixture::SECOND_HANDLE_REASON
                        || *reason == DecliningProjectionFixture::RESET_REFUSAL_REASON
                        || *reason == DecliningProjectionFixture::COMMIT_FAULT_REASON,
                    "`{rule}` skipped with a reason the fixture never gave: \
                     {reason:?}. The reason is the only record of the trade, so \
                     it has to be the adapter's own words"
                );
            }
            Verdict::Panicked { message, .. } => {
                assert!(
                    PROJECTION_MUST_REJECT.contains(&rule),
                    "`{rule}` panicked against a fixture that is correct in every \
                     respect except its declared capabilities. If it opened a \
                     second handle, it ignored its gate. Message: {message}"
                );
                assert!(
                    message.contains(DecliningProjectionFixture::SECOND_HANDLE_REASON),
                    "`{rule}` must reject a projection fixture declining the \
                     `SECOND_HANDLE` MUST *and carry the fixture's own stated \
                     reason*, so the failing build says why. Message: {message}"
                );
            }
        }
    }

    /// CF-18, on the projection family. A capability-gated rule is still
    /// emitted, still answered, and reports the fixture's own reason.
    ///
    /// The projection sibling of [`capability_skips_are_reported`], and it
    /// proves and fails to prove exactly the same halves: libtest exposes
    /// nothing programmatically, so this drives the enumeration itself and
    /// asserts on [`Verdict`] **values**. That covers the *answering* half —
    /// every registered projection rule produces an outcome against a fixture
    /// that declines everything, and none of them silently passes. It does not
    /// cover the *emission* half, which is only checkable from outside the
    /// process via `cargo test -- --list`.
    #[test]
    fn projection_capability_skips_are_reported() {
        let rules = all_projection_rules();

        // ---- A fixture that declines everything still answers everything ----
        let declining = run_projection_subject::<DecliningProjectionFixture>();

        let answered: Vec<&str> = declining.outcomes.iter().map(|(rule, _)| *rule).collect();
        assert_eq!(
            answered.len(),
            rules.len(),
            "a projection fixture declining every capability must still produce \
             an outcome for every registered rule; `#[cfg]`-ing one out would \
             make it indistinguishable in CI output from a rule that passed"
        );
        for rule in &rules {
            assert!(
                answered.contains(rule),
                "`{rule}` produced no outcome against a projection fixture that \
                 declines everything"
            );
        }

        // ---- Every outcome is one this fixture's own declarations explain ----
        for (rule, verdict) in &declining.outcomes {
            assert_projection_declension(rule, verdict);
        }

        // The demonstrated half. Every rule in the list, not just the first: a
        // second `must!` that quietly skipped would be a MUST enforced on one
        // rule and stated on the other.
        for rule in PROJECTION_MUST_REJECT {
            assert!(
                matches!(declining.verdict(rule), Some(Verdict::Panicked { .. })),
                "`{rule}` must *fail* a projection fixture that declines \
                 `SECOND_HANDLE`, not skip it. A skip there is the outcome CF-18 \
                 names as the thing to prevent: a green suite plus one SKIP line \
                 for an adapter nothing reached through two connections, and for \
                 a projection that means PS-1 was never observed at all. Saw: \
                 {:?}",
                declining.verdict(rule)
            );
        }

        let skipped = declining.skipped();
        for rule in PROJECTION_MUST_SKIP {
            assert!(
                skipped.contains(rule),
                "`{rule}` is gated on a capability this fixture declines, so it \
                 must report a skip rather than pass. Saw {skipped:?}"
            );
        }
        for rule in PROJECTION_MUST_REJECT {
            assert!(
                !skipped.contains(rule),
                "`{rule}` reported a skip, so `must!` has been downgraded back \
                 to `require!` and the MUST is unenforced again"
            );
        }

        // ---- And the reference fixture skips exactly what it declines ------
        assert_reference_projection_declensions();
    }

    /// The skip arm of [`projection_capability_skips_are_reported`],
    /// demonstrated against the reference fixture on real values.
    ///
    /// A free function rather than the tail of that test for
    /// [`assert_projection_declension`]'s reason, and it crossed
    /// `clippy::too_many_lines` at the same place: the second declinable
    /// capability's rule arrived and the body grew past a hundred lines. A lint
    /// suppression there would be the wrong trade — every assertion below names
    /// something a green build would otherwise have hidden.
    fn assert_reference_projection_declensions() {
        // An *equality* rather than an emptiness check on purpose. "Skips
        // nothing" was the assertion while every capability a rule read was
        // supported; relaxing it to "may skip" the moment a rule acquired a gate
        // would have turned it into a check that passes however many rules
        // quietly stop running. So the set is pinned: exactly the rules gated on
        // a capability this fixture declines, and nothing else.
        let capable = run_projection_subject::<MemoryProjectionFixture>();
        assert_eq!(
            capable.skipped(),
            vec![
                "failed_commit_leaves_both_unchanged",
                "refused_reset_changes_nothing",
            ],
            "`{}` supports every capability the registered rules ask for except \
             `COMMIT_FAULT`, which the reference store cannot offer — it applies \
             both halves of a commit under one write lock — and `RESET_REFUSAL`, \
             which it cannot offer either, because it holds no protection policy. \
             So exactly two rules may skip against it, in enumeration order. A \
             rule that joined this set means a gate reads the wrong const or a \
             capability was declined to turn a red build green; a rule that left \
             it means the gate went away",
            capable.name
        );

        // And each skip carries the capability an author can change and the
        // fixture's own words, rather than a testkit paraphrase.
        for (rule, capability, stated) in [
            (
                "failed_commit_leaves_both_unchanged",
                "COMMIT_FAULT",
                <MemoryProjectionFixture as happenstance_testkit::ProjectionFixture>::COMMIT_FAULT
                    .reason()
                    .expect("the reference fixture declines COMMIT_FAULT"),
            ),
            (
                "refused_reset_changes_nothing",
                "RESET_REFUSAL",
                <MemoryProjectionFixture as happenstance_testkit::ProjectionFixture>::RESET_REFUSAL
                    .reason()
                    .expect("the reference fixture declines RESET_REFUSAL"),
            ),
        ] {
            assert_eq!(
                capable.verdict(rule).map(Verdict::describe),
                Some(
                    Verdict::Skipped {
                        capability,
                        reason: stated,
                    }
                    .describe()
                ),
                "the skip must name the associated const an adapter author can \
                 actually change and carry the fixture's own stated reason — \
                 compared against the fixture's own `const`, never against a \
                 literal repeated here, which is what would let the report carry \
                 someone else's sentence while this test stayed green"
            );
        }

        for (rule, verdict) in &capable.outcomes {
            assert!(
                matches!(verdict, Verdict::Passed | Verdict::Skipped { .. }),
                "`{rule}` did not pass against the reference projection fixture, \
                 which is the oracle: {}",
                verdict.describe()
            );
        }
    }

    /// Every projection capability reason is written by whoever DT-3 said writes
    /// it, and is not empty.
    ///
    /// The projection family's capability set is
    /// `SECOND_HANDLE`, `RESET_REFUSAL` and `COMMIT_FAULT` on the fixture, all
    /// three with **fixture-written** reasons, plus `READS_THROUGH_BATCH` on
    /// `ProjectionProbe` rather than on the fixture — which is why no assertion
    /// here mentions it. The family therefore adds **no** testkit-written
    /// reason: there is no projection counterpart to `NO_CEILING_REASON`,
    /// because the projection port declares no numeric limits and never reaches
    /// the surface that constant exists for. `COMMIT_FAULT` is the amendment
    /// DT-3 took on 2026-08-14, and it arrived under the same one policy — it is
    /// required rather than defaulted precisely so no testkit sentence has to
    /// stand in for a store's own account of why it cannot fail a commit.
    ///
    /// What this can and cannot check. `Capability::declined("")` is rejected by
    /// a `const fn` `assert!`, but on an *associated* const that fires at
    /// **codegen** — so `cargo test` catches it and `cargo check` and
    /// `cargo clippy` do not. This test runs in a built binary, which is
    /// precisely why it is a `#[test]` rather than a review note: it forces
    /// every constant below to be evaluated.
    #[test]
    fn projection_capability_reasons_are_authored_once() {
        let declining = run_projection_subject::<DecliningProjectionFixture>();
        let capable = run_projection_subject::<MemoryProjectionFixture>();

        for report in [&declining, &capable] {
            for (capability, reason) in &report.declines {
                assert!(
                    !reason.trim().is_empty(),
                    "`{}` declines `{capability}` with an empty reason, and the \
                     reason is the only record of the trade",
                    report.name
                );
            }
        }

        // The fixture's own words reach the report, rather than a copy of them.
        // Compared against the fixture's `const` — never against a literal
        // repeated here, which is what would let the report carry someone
        // else's sentence while this test stayed green.
        assert_eq!(
            declining.declines,
            vec![
                (
                    "SECOND_HANDLE",
                    DecliningProjectionFixture::SECOND_HANDLE_REASON
                ),
                (
                    "RESET_REFUSAL",
                    DecliningProjectionFixture::RESET_REFUSAL_REASON
                ),
                (
                    "COMMIT_FAULT",
                    DecliningProjectionFixture::COMMIT_FAULT_REASON
                ),
            ],
            "the declining instrument's reported declensions must be its own \
             three constants, in the order `projection_declines` lists them"
        );

        // The reference fixture declines exactly two things and supports the
        // MUST. A `RESET_REFUSAL` that started reading as supported would mean
        // `MemoryProjectionStore` had grown a protection policy, and a
        // `COMMIT_FAULT` that did would mean it had grown fault injection —
        // either is a change to the oracle rather than to this test.
        let names: Vec<&str> = capable
            .declines
            .iter()
            .map(|(capability, _)| *capability)
            .collect();
        assert_eq!(
            names,
            ["RESET_REFUSAL", "COMMIT_FAULT"],
            "`MemoryProjectionFixture` must support `SECOND_HANDLE` — it is a \
             MUST — and decline `RESET_REFUSAL`, because the store under it has \
             no protection policy and cannot honestly claim one, and \
             `COMMIT_FAULT`, because that store applies both halves of a commit \
             under one write lock and has no write that can be made to fail"
        );
    }

    /// CF-1's *argument*, applied to `event_store_model_conformance!`.
    ///
    /// The model family has no [`REGISTRY`](super::REGISTRY) row and cannot have
    /// one: the registry is keyed on named suite rules, and this is one macro.
    /// What it can have — and what this test is — is the same obligation
    /// discharged in the same binary against the same wrong stores.
    /// [`MODEL_COVERAGE`](super::MODEL_COVERAGE) is the claim; this drives it.
    ///
    /// Both directions are asserted, for the reason CF-3 gives: a table of the
    /// stores the model catches would go green against a model that catches
    /// everything, including the two conformant variants it must not.
    #[test]
    #[cfg(feature = "proptest")]
    fn the_model_rule_rejects_exactly_what_it_claims() {
        let observed = model_reports();

        assert_eq!(
            observed.len(),
            MODEL_COVERAGE.len(),
            "`MODEL_COVERAGE` claims {} store(s) and {} were driven, so the \
             table and `for_each_mutant!` disagree about the universe",
            MODEL_COVERAGE.len(),
            observed.len()
        );

        for (name, outcome, description) in &observed {
            let Some((_, claimed)) = MODEL_COVERAGE.iter().find(|(row, _)| row == name) else {
                panic!(
                    "`{name}` is registered in `for_each_mutant!` but absent \
                     from `MODEL_COVERAGE`, so the model family is driven \
                     against it and nothing is claimed about the answer"
                );
            };

            assert_ne!(
                *outcome,
                ModelOutcome::FellOver,
                "the model rule ran `{name}` into a panic that is not its own \
                 assertion, so whatever this row demonstrates, it is not the \
                 model catching a defect (CF-2's `Rejects:`, one family over). \
                 Saw: {description}"
            );

            assert_eq!(
                outcome, claimed,
                "`{name}`: `MODEL_COVERAGE` claims {claimed:?} and the model \
                 answered {outcome:?}.\n\n\
                 If the store moved from `Agreed` to `Rejected`, the model got \
                 stronger and the table should say so. If it moved the other \
                 way, the model got weaker and the table is the only thing that \
                 noticed. If it is a **conformant variant** that moved, the \
                 finding is about the model rather than the store — most likely \
                 an assumption that positions are dense (CF-6), which is what \
                 the symbolic `Anchor` exists to prevent.\n\n\
                 Saw: {description}"
            );
        }

        // The positive control on the whole table. Without it every assertion
        // above is satisfied by a `MODEL_COVERAGE` in which nothing is claimed
        // to be rejected and a model rule whose body is `RuleOutcome::Ran`.
        assert!(
            observed
                .iter()
                .any(|(_, outcome, _)| *outcome == ModelOutcome::Rejected),
            "the model rule rejected nothing in this binary, so it certifies \
             nothing — which is precisely what ADR-0010 calls a decorative rule"
        );
    }

    /// The static half of `the_concurrency_rules_reject_exactly_what_they_claim`:
    /// what the table says, before any store is driven.
    ///
    /// Split out because the test crossed `clippy::too_many_lines`, and split
    /// *here* rather than anywhere else because this is the seam: everything
    /// above reads `RACERS` and `for_each_racer!`, everything below reads the
    /// answers a running store gave.
    fn racer_table_agrees_with_the_enumeration(rules: &[&str], names: &[&str]) {
        // The two lists that drift, held together exactly as
        // `mutant_registry_is_exhaustive` holds `for_each_mutant!` and
        // `REGISTRY`.
        for name in names {
            let Some(row) = RACERS.iter().find(|row| row.name == *name) else {
                panic!(
                    "`{name}` is enumerated in `for_each_racer!` but has no \
                     `RACERS` row, so it is driven and nothing is claimed about \
                     the answer"
                );
            };
            assert!(
                !row.provenance.is_empty(),
                "`{name}` must name the adapter shape that makes it plausible \
                 (CF-4's obligation, one family over)"
            );
            for rule in row.fails {
                assert!(
                    rules.contains(rule),
                    "`{name}` claims to fail `{rule}`, which is not a rule of \
                     the concurrency family: {rules:?}"
                );
            }
            for (rule, _) in row.expect {
                assert!(
                    row.fails.contains(rule),
                    "`{name}` pins an assertion in `{rule}`, which it does not \
                     declare that it fails — a pin on a rule that never rejects \
                     is a claim nothing evaluates"
                );
            }

            // The two halves of the kind, in both directions. A control that
            // declares a failure is not a control, and a racing store that
            // declares none is a defect nothing catches — which is the state
            // `mutant_registry_is_exhaustive` rejects one family over, and the
            // state an author reaches by emptying a flaky row's list rather
            // than repairing its rendezvous.
            assert_eq!(
                row.fails.is_empty(),
                row.kind == RacerKind::ConformantControl,
                "`{name}` is registered as {:?} with {} declared failure(s). A \
                 conformant control declares none, and a racing store that \
                 declares none has had its defect disarmed rather than fixed",
                row.kind,
                row.fails.len()
            );
        }
        for row in RACERS {
            assert!(
                names.contains(&row.name),
                "`{}` has a `RACERS` row but is absent from `for_each_racer!`, \
                 so its claim is never checked against a store",
                row.name
            );
        }
    }

    /// CF-1 and CF-3, applied to `event_store_concurrency_conformance!`.
    ///
    /// The concurrency family has no [`REGISTRY`](super::REGISTRY) row and
    /// cannot have one: every store it drives fails *no* event-store rule, and
    /// `mutant_registry_is_exhaustive` rejects a row with an empty `fails` list.
    /// [`RACERS`](super::RACERS) is its registry, and this is the meta-test that
    /// makes it a check rather than a list.
    ///
    /// # Why this one matters more than it looks
    ///
    /// A racing rule is the easiest kind to write in a form that cannot fail:
    /// start some threads, assert something true of the *final* state, and the
    /// test is green whatever happened in between. Every rule in this family was
    /// written against a store that breaks it, and this test is what keeps that
    /// true — including through the refactor that quietly turns a rendezvous
    /// into a race and a rejection into a coin toss. A store that used to be
    /// rejected and now passes shows up here, by name.
    #[test]
    fn the_concurrency_rules_reject_exactly_what_they_claim() {
        let rules = all_concurrency_rules();
        let names = racer_names();

        racer_table_agrees_with_the_enumeration(&rules, &names);
        // Both directions, per (store, rule).
        for (store, rule, outcome, description) in racer_reports() {
            let Some(row) = RACERS.iter().find(|row| row.name == store) else {
                unreachable!("checked above")
            };
            let claimed = row.fails.contains(&rule);

            assert_ne!(
                outcome,
                RacerOutcome::FellOver,
                "`{store}` did not answer `{rule}`: it panicked somewhere that \
                 is not a rule's own assertion, so whatever this row \
                 demonstrates it is not a rule catching a defect (CF-2's \
                 `Rejects:`). A contender that panics arrives here with no \
                 origin, because the panic hook records into the *contender's* \
                 thread-local — so this failure most often means a racing store \
                 fell over rather than losing. Saw: {description}"
            );

            let rejected = outcome == RacerOutcome::Rejected;

            // Only when it *was* rejected: a pin describes which assertion did
            // the rejecting, so checking it against a store that passed reports
            // the wrong failure for the right problem — measured, on the run
            // that first caught `GlobalVersionStore` passing.
            if let Some((_, needle)) = row
                .expect
                .iter()
                .find(|(pinned, _)| *pinned == rule)
                .filter(|_| rejected)
            {
                assert!(
                    description.contains(needle),
                    "`{store}` was rejected by `{rule}`, but not at the \
                     assertion `RACERS` pins it to. Expected the message to \
                     contain {needle:?}.\n\n\
                     Two stores fail this family's rules for opposite reasons — \
                     too many winners and none at all — so a store that starts \
                     tripping the other assertion is still red-for-a-reason and \
                     is no longer evidence for the reason its provenance \
                     claims.\n\n\
                     Saw: {description}"
                );
            }

            assert_eq!(
                rejected,
                claimed,
                "`{store}` against `{rule}`: `RACERS` claims it {} and it {}.\n\n\
                 A store that stopped being rejected is the failure this test \
                 exists for — most likely a rendezvous that stopped meeting, \
                 which turns the rejection into a race the rule usually loses. \
                 A store that started being rejected means the rule reaches \
                 further than the table says, which is good news the table has \
                 to be told about.\n\n\
                 Saw: {description}",
                if claimed { "fails" } else { "passes" },
                if rejected { "was rejected" } else { "passed" },
            );
        }

        // The positive control on the whole table: every rule must be rejected
        // by something. This is CF-1 for a family whose rules are macro-emitted
        // rather than registry-keyed, and without it the table is satisfied by
        // one in which nothing fails anything.
        for rule in &rules {
            assert!(
                RACERS.iter().any(|row| row.fails.contains(rule)),
                "no racing store fails `{rule}`, so it is a rule nothing has \
                 ever been shown to break — which is what ADR-0010 calls \
                 decorative"
            );
        }

        // The other direction, and the one this family had only in prose. The
        // event-store family asserts it over `REGISTRY` and the projection
        // family over its own table; here the control was an empty `fails` list
        // and a provenance paragraph, so deleting `LockedStore` outright left
        // every assertion above holding and the gate green — measured, not
        // argued.
        assert!(
            RACERS
                .iter()
                .any(|row| row.kind == RacerKind::ConformantControl),
            "no conformant control is registered for the concurrency family, so \
             its five rules have only ever been passed by stores registered as \
             defective — which is the vacuity of CF-5 reintroduced in the one \
             family whose rules are macro-emitted and have no `REGISTRY` row to \
             fall back on"
        );
    }
}
