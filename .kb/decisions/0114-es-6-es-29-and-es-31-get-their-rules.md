---
id: kb-decision-0114
title: ES-6, ES-29 and ES-31 get their rules, owned by phase 20, and none is demoted
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0114
reversibility: medium
phase: 21
supersedes: null
superseded_by: null
summary: >-
  Accepted 2026-11-20 for #188, on the owner's yes to every recommendation (proposed 2026-11-18). Three frozen
  clauses name rules that nothing owns. Each is writable, so none is demoted, and phase 20, the
  last phase phase 21 waits on, owns all three.
  ES-6: store_error_crosses_a_join_handle is written as a happenstance-core test that declares
  ADR-0009's marker the way a downstream crate would, spawns append under it, and returns the
  error through the JoinHandle, which asserts on the Output as ADR-0008 requires. A
  compile_fail,E0277 probe beside SendStoreWithLocalError is its negative. "Unwritable against
  today's port" was true of a rule bound by SendEventStore, not of one bound by the marker.
  ES-31: checkpoint_lag_is_not_a_position_difference is written beside the typed runner, over a
  stride-7 GappyMemoryStore. ES-29 shares SY-6's rule, and writing it means building SY-6's
  refusal. Nothing builds it today: every adapter stores a guard that carries after as evidence.
  How a guard carrying after is refused is a fork left to the owner. The recommendation is to land
  the events and record the group with no guard, which SY-1, SY-33 and convergence all allow.
  ES-6's and ES-31's Rule lines move to name where their rules live. That is a repair by this
  record. ES-6's open question is closed.
depends_on:
  - kb-decision-0008
  - kb-decision-0009
  - kb-decision-0023
  - kb-decision-0107
related:
  - kb-open-question-es-6-unwritable-rule-001
  - kb-reference-spec-trace-unresolved-declarations-001
  - kb-playbook-repair-frozen-clause-001
source_paths:
  - xtask/src/spec_trace.rs
  - xtask/src/spec_trace/sync_family.rs
  - crates/happenstance-cloudflare/src/send_shape.rs
  - crates/happenstance-cloudflare/src/lib.rs
  - crates/happenstance-sync/src/peer.rs
  - crates/happenstance-sync/src/memory_ingest.rs
  - crates/happenstance/tests/projection_runner.rs
  - runbook/phases/20-docs-that-teach.md
  - spec/SPECIFICATION.md
last_reviewed: 2026-11-20
---

# ES-6, ES-29 and ES-31 get their rules, owned by phase 20, and none is demoted

**Accepted on 2026-11-20 for #188, as recommended; proposed on 2026-11-18.** What follows describes the tree at acceptance; the follow-ups build it. One line has landed on the proposing
branch, because it is additive: the owning-phase box in `runbook/phases/20-docs-that-teach.md`.
Nothing else under "What acceptance changes" has been applied. This atom is the whole record.

## Context

`UNRESOLVABLE_RULE_NAMES` declares three frozen clauses' rules `Scheduled` with no real owner
(`xtask/src/spec_trace.rs:1268-1285`):

| Clause | Rule | Declared owner |
| --- | --- | --- |
| ES-6 (`spec/SPECIFICATION.md:2828`, rule at `:2924`) | `store_error_crosses_a_join_handle` | "nothing, yet", because `send_shape.rs` argues it is unwritable |
| ES-29 (`:4420`, rule at `:4448`) | `wire_condition_with_after_is_refused` | "the phase that lands the wire condition" |
| ES-31 (`:4540`, rule at `:4549`) | `checkpoint_lag_is_not_a_position_difference` | "the phase that lands checkpoint lag" |

No runbook file mentions ES-29 or ES-31. Phase 21 settles nothing (`runbook/phases/21-one-point-oh.md`,
"Decisions it settles. None"). Its clause audit reads a frozen clause as met.

## ES-6, writable once the bound is right

`SendStoreWithLocalError` (`crates/happenstance-cloudflare/src/send_shape.rs:114-138`) shows that
`SendEventStore` admits a `!Send` error. So a rule bound by `SendEventStore` cannot prove that an
error crosses a `JoinHandle`, and that is all `crates/happenstance-cloudflare/src/lib.rs:371-378`
shows. ADR-0009 put the strength in a marker declared downstream,
`ThreadSafeEventStore: SendEventStore<Error: Send + Sync>` with a blanket impl
(`.kb/decisions/0009-error-send-sync.md:74-75`). A rule bound by that marker is writable today.
ADR-0008 fixes its shape: it must assert on the future's `Output`. `tokio::spawn` requires
`F::Output: Send`, so a spawned task that returns `append`'s `Result` meets that requirement by
construction.

**Options.**

- **A. A test in `happenstance-core` with a local marker (chosen).** The test lives at
  `crates/happenstance-core/tests/thread_safe_marker.rs`. It declares the marker as a downstream
  crate would, spawns `append` on a `MemoryEventStore` inside `tokio::spawn`, and awaits the
  `JoinHandle` for the `AppendError`. The negative is a `compile_fail,E0277` doctest in
  `send_shape.rs`: the same marker refuses `SendStoreWithLocalError`. That is the wrong store ES-6's
  `Rejects:` already names. A marker written without the `Error` bound, which is ADR-0008's
  decorative shape, makes that doctest compile, and the doctest fails. This adds no public item.
- B. An opt-in testkit group, `event_store_thread_safe_conformance!`, with the marker shipped
  publicly in `happenstance-testkit`. This is ADR-0009's original shape. It adds public API (H-16),
  and it proves nothing more: the property is type-level, so one compiling instance is the whole
  proof.
- C. Demote the Rule to "none". The port's lack of a bound is already pinned:
  `SendStoreWithLocalError` stops compiling if `Error` gains one. But the clause's second half, that
  the strength is a marker, would then have no instrument.

## ES-31, writable beside the typed runner

ES-31 binds "generic code determining whether a consumer is current". In this workspace that code
is `run_projection` and `run_fan_out`. Each treats a run that finds nothing as caught up, and
neither subtracts positions. §4.11 already places the rule beside the typed runners as
integration-level, not in the adapter suite (`SPECIFICATION.md:6904-6912`, CF-36). ES-31's own
Rule line says "in the projection suite", which contradicts that. `tolerates_gapped_positions`
(`crates/happenstance/tests/projection_runner.rs:597-628`) is already halfway there. It resumes
across a stride-7 gap and asserts `through == Some(third)`.

**Options.**

- **A. Write the named rule in `projection_runner.rs` (chosen).** Over
  `GappyMemoryStore::with_stride(7)`: seed, run to completion, append exactly one event, and run
  again. Assert three things: `applied == 1`, the checkpoint equals `head()`, and a third run is
  idle (`through: None, applied: 0`). The numeric distance is 7 and the backlog is one. A runner
  that sized its work as `head - checkpoint` would read seven events of outstanding work and either
  fail to finish or report a backlog that does not exist.
- B. Repoint ES-31's Rule line at `tolerates_gapped_positions`. Cheaper, but that rule never checks
  `checkpoint == head` or the idle rerun, which are the two facts ES-31 names.
- C. Demote. ES-31's MUST also binds caller code no rule can see, but the in-tree runners are
  generic code it binds, and they can be checked.

## ES-29, which needs SY-6's refusal built

ES-29's MUST is that a store does not evaluate a foreign `after`. That is met by construction:
ingest evaluates nothing (SY-1, held by `ingest_never_rejects`). Its Rule line names SY-6's rule,
and SY-6 (`SPECIFICATION.md:7194`, rule at `:7203`) is `[FROZEN]` with an unbuilt half: "a
condition carrying a position-relative boundary … MUST be refused as ingest input". Nothing
refuses one today:

- `EventGroup::guard` travels on the wire. Its rustdoc still says "Phase 13 builds the refusal"
  (`crates/happenstance-sync/src/peer.rs:227-244`).
- The crate root says "Nothing here enforces it yet" (`crates/happenstance-sync/src/lib.rs:121`).
- Every ingest stores the guard verbatim as evidence.

Local appends record no guard (`crates/happenstance-sync/src/memory_ingest.rs:425-431`), so only a
wrong or foreign peer can send one. No rule in the sync testkit builds a guard with `after`. The
ingest-side declaration is `xtask/src/spec_trace/sync_family.rs:86-92`, owed by #458's second half.

**The fork, which the owner settled on 2026-11-20 at (i)** (Decision 4 records it):

- **(i) Land the events and refuse the guard (recommended).** The group is recorded with no guard,
  so `read_groups` hands back `None`. This meets SY-1, because every event lands, and SY-33,
  because nothing is refused on condition-level grounds. The edge keeps converging. Its costs: the
  sender's bug is silent unless `Ingested` gains a count (additive, since it is
  `#[non_exhaustive]`), and SY-30's "the same guards" needs the qualifier *position-free*. That
  qualifier is prose by this record, because no store SY-30 admits today relays a guard carrying
  `after`.
- (ii) Refuse the call with a new `IngestError` arm of class `Stop`, as a malformed envelope
  (SY-33 permits that refusal). It is loud at the first hop. But it wedges an edge over a defect
  that lives only in evidence, and SY-1's body names SY-38 and SY-40 as its only exceptions. So
  SY-1 would need a third exception, which is a frozen-clause amendment, and it is one-way.

Under either branch the rule is one rule, `wire_condition_with_after_is_refused`, in the ingest
family. It ingests a group whose guard carries an `after` that *would* reject if evaluated in the
receiver's numbering, then asserts the branch's outcome. Under (i): the events land, nothing is
compensated, and `read_groups` gives no guard. Under (ii): the arm, and nothing written. Two wrong
stores go in `tests/wrong_ingest_stores.rs`. `EvaluatesForeignAfterStore` evaluates the guard.
`KeepsPositionRelativeGuardStore` is today's behaviour, and it fails under (i) only. All four
adapters and `MemoryIngestStore` change, which is a behaviour narrowing for the next minor.

## Decision

1. **None of the three is demoted.** Each rule is writable and has a named wrong implementation.
2. **Phase 20 owns all three.** It is the last phase phase 21 waits on that is not `done`, which is
   PS-40's precedent in `runbook/ledgers.md`. The box is under "Work carried in" as #188.
3. **ES-6: option A. ES-31: option A.**
4. **ES-29: the shared rule with SY-6, on branch (i), which the owner took on 2026-11-20.**
5. **ES-6's open question is closed.** Sub-question 1 is answered by Decision 3. On sub-questions
   2 and 3: an `UNRESOLVABLE_RULE_NAMES` entry must name an owner a reader can find, either a phase
   or an issue. That is enforced by review here and not by a lint, because a lint over free text
   would be decorative.

## What acceptance changes

**`spec/SPECIFICATION.md`.**

- **ES-6's Rule line:** "in a rule group whose bound is the marker rather than `SendEventStore`"
  becomes "in `crates/happenstance-core/tests/thread_safe_marker.rs`, bound by ADR-0009's marker
  declared as a downstream crate declares it, with its negative a `compile_fail` probe in
  `send_shape.rs`". `(new)` comes off. The §5 "Blocked on machinery" paragraph (`:5140-5143`) is
  discharged as the ES-18 entry was.
- **ES-31's Rule line:** "in the projection suite" becomes "beside the typed runner, in
  `crates/happenstance/tests/projection_runner.rs` (§4.11)". `(new)` comes off, and §4.11's
  "the last unwritten" sentence is updated.
- **ES-29 and SY-6:** `(new)` comes off both Rule lines. The "does not exist" clause about the
  sync testkit at `:4448-4450`, which says the crate does not exist, and in §5's "Blocked on a decision" (`:5157-5158`) goes. Under (i),
  SY-6 gains one sentence, *refused means not kept: the events land and the group is recorded with
  no guard*, and SY-30 gains *position-free*. Under (ii), SY-1 is amended to name SY-6 as an
  exception.
- **§7.2:** regenerated, so the three `†` marks go.

**`xtask/src/spec_trace.rs`.** Each `Scheduled` entry at `:1268-1285` goes as its rule lands. A
name that resolves is deleted; one that resolves only `Elsewhere` becomes `Elsewhere(path)`.
Until each rule lands, its reason names "phase 20 (#188)". The edit stays line-neutral, as the
comments below `:1286` require. `sync_family.rs`'s SY-6 entry goes with ES-29's.

**Code.**

- `peer.rs`'s "Phase 13 builds the refusal" and `lib.rs:121` are corrected when the refusal lands.
- `crates/happenstance-cloudflare/src/lib.rs:371-378` and `send_shape.rs:94-98` stop calling the
  rule unwritable, and say instead that it is bound by the marker.

**`.kb/open-questions/es-6-names-an-unwritable-rule.md`** is closed, citing this record.

## Costs

About 80 lines for ES-6, about 60 for ES-31, and for ES-29 about 350 across `happenstance-sync`,
four adapters, the sync testkit and its wrong stores. ES-29 changes no port signature on branch
(i). Branch (ii) adds an `IngestError` arm, which is additive.

## Door

ES-6 and ES-31 are two-way: tests, plus Rule-line repairs that admit the same set. ES-29 on (i) is
two-way in code, but it makes SY-6's meaning concrete in a frozen clause, so reversing it needs a
record. On (ii) it is one-way, because it amends SY-1.
