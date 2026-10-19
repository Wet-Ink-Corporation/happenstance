---
id: kb-decision-0096
title: VT-30 is frozen, after_every_guard names the blanket setter's scope, and typestate is rejected
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0096
reversibility: low
phase: 17b
supersedes: null
superseded_by: null
summary: >-
  The owner's call, 2026-10-12 (wi-378078, "All as recommended"). ADR-0054 is carried out as
  written. AppendCondition gains after_every_guard and after_every_guard_opt, and after and
  after_opt are deprecated since 0.4.2 and forward to them. The old spellings are kept, with the
  same behaviour, through 1.x. The two pin tests ADR-0054 names are in append.rs's unit tests, and
  and_guard's and the blanket setters' doc blocks state the erasure and the order that causes it.
  Every in-tree call site is migrated, and one unit test keeps the deprecated spellings exercised.
  The builder-state option, a typestate that makes the hazardous order a compile error, is
  rejected. EventStore::append takes the erased condition type, so the blanket setter would
  return on the conversion, and an additive builder stays available after 1.0 if ever needed.
  VT-30 is FROZEN. Limb 1 of its falsifier does not fire: happenstance-neon pushes a multi-guard
  condition into one statement with no self-join per guard. Limb 2, the min() collapse measured
  on a Wattline-shaped workload, is retired by this record and not by a benchmark scenario. It
  could fire only as a major. This amends one sentence of ADR-0054, which said the alias would be
  renamed or deleted when VT-30's marker lifts. It is kept instead, because deleting it is a break.
depends_on:
  - kb-decision-0054
  - kb-decision-0066
  - kb-decision-0072
  - kb-decision-wi-378078
related:
  - kb-decision-0012
  - kb-decision-0054
  - kb-decision-0075
  - kb-decision-wi-378078
  - kb-open-question-vt-30-marker-stale-unscheduled-001
source_paths:
  - crates/happenstance-core/src/append.rs
  - crates/happenstance-neon/src/event_store.rs
  - spec/SPECIFICATION.md
  - runbook/ledgers.md
last_reviewed: 2026-10-12
---

# VT-30 is frozen, after_every_guard names the blanket setter's scope, and typestate is rejected

**Accepted by the owner on 2026-10-12** (`wi-378078`, "All as recommended", for #85). No
long-form record exists; this atom is the record.

## Context

VT-30 says an `AppendCondition` is a non-empty sequence of guards, each with its own boundary,
and that the blanket setter applies its boundary to every guard. ADR-0054 kept that behaviour,
because VT-30 mandates it. It asked for two pin tests, a sentence in two doc blocks, and a
spelling that says what the setter does, with the old one deprecated. None of that had landed.

VT-30's marker had two limbs. It was falsified if no adapter could push a multi-guard condition
into one statement without one self-join per guard, or if the `min()` collapse measured as
immaterial on a Wattline-shaped workload. It named the Postgres adapter and the benchmark harness
as the instruments, and called both unbuilt. ADR-0066 gave the clause `freeze-by-17`, with
"ADR-0054's alias and builder-state decided in one pass; limb 2 retired by a record or by a
multi-guard bench scenario". ADR-0072 moved the row to `freeze-by-17b`, because the answer is
additive. `kb-open-question-vt-30-marker-stale-unscheduled-001` recorded that the marker was
stale and that no phase scheduled lifting it.

## Decision

1. **The scope-carrying spelling.** `AppendCondition::after_every_guard(position)` and
   `after_every_guard_opt(position)` set the boundary of every guard already present. This is
   the behaviour `after` and `after_opt` always had. The new methods sit where the old ones did
   in `crates/happenstance-core/src/append.rs`. `after_every_guard_opt` now writes each guard's
   `after` in place instead of rebuilding the boxed slice. The result is the same, with one
   allocation fewer.
2. **The old spelling is deprecated and kept through 1.x.** `after` and `after_opt` carry
   `#[deprecated(since = "0.4.2")]`, and each note names its replacement and says the old name is
   kept through 1.x. They forward to the new methods and behave the same. They sit in a second
   `impl` block after `serde_impls`, so no line a citation names moved. Removing them is a major
   release, so they cannot go before 2.0.
3. **The pins ADR-0054 names.** Three tests are in `append.rs`'s `mod tests`:
   - `after_every_guard_after_and_guard_overwrites_every_boundary` covers the hazardous order.
     `new(q1).and_guard(q2, Some(p2)).after_every_guard(p1)` leaves both guards at `p1`. It
     rejects the fill-only-`None` implementation, which would leave `[p1, p2]`.
   - `after_every_guard_opt_before_and_guard_keeps_boundaries_independent` covers the safe order.
   - `deprecated_after_and_after_opt_still_overwrite_every_guard` pins the deprecated spellings on
     the hazardous order, under a scoped `#[expect(deprecated)]`. It is the only use of them left
     in the tree.
4. **The doc sentences.** `and_guard`'s doc says that a later blanket setter erases the `after`
   it set, and to call the setter first. `after_every_guard_opt`'s and `after_opt`'s docs say
   that, called after `and_guard`, they erase its boundary and widen what the condition
   tolerates. `after_every_guard`'s doc says it erases any boundary `and_guard` set.
5. **Every in-tree call site is migrated** in the same change. This covers the crates, the
   doctests, the READMEs, the guide under `docs/`, the constitution's RS-13 example and the
   experiments. Because the gate runs `-D warnings`, a call site left on the old name fails the
   build. `xtask/tests/first_encounter.rs`'s two substring checks follow the guide to
   `after_every_guard_opt(upto)`.
6. **Builder-state (typestate) is rejected.** `EventStore::append` takes `Option<&AppendCondition>`
   on a frozen port. A typestate `AppendCondition<S>` would either change that signature, which is
   a major break, or convert into the erased type. The blanket setter would then be reachable
   again after the conversion, unless it were removed from the erased type, and that removal is
   the break the deprecation route avoids. The hazard is one wrong order of two builder calls. It
   is now named in both doc blocks, pinned by two tests, and warned on through the old spelling.
   ADR-0054 kept this option open for one possible use: a replication ingest policy that wants to
   re-blanket a condition it has already assembled. `happenstance-sync` has not been written, and
   a separate builder type could be added after 1.0 without breaking anything. So rejecting
   typestate now rules nothing out in 1.x.
7. **VT-30 is `[FROZEN]`.** The clause now names `after_every_guard` and
   `after_every_guard_opt`, and keeps `after` and `after_opt` as deprecated spellings of them.
   - **Limb 1 does not fire.** `happenstance-neon` pushes a multi-guard condition into one
     statement with no self-join per guard. Its conditional insert carries
     `WHERE NOT EXISTS (SELECT 1 FROM event WHERE (g₁ AND position > p₁) OR (g₂ AND position > p₂) …)`
     (`crates/happenstance-neon/src/event_store.rs:530-549`). Its conflict probe is one
     `UNION ALL` statement over the guards (`:465-491`). Both run in the live conformance suite,
     which includes `condition_guards_carry_independent_boundaries`. `happenstance-postgres`,
     `happenstance-sqlite` and `happenstance-cloudflare` issue one `SELECT max(position)` per
     guard. That is a choice, not an inability.
   - **Limb 2 is retired by this record**, as the owner chose, and no `fragmented_boundary`
     benchmark scenario is built. The limb asked whether the guard sequence earns its keep. The
     argument against the `min()` collapse is a construction, not a measurement. When
     `p_quiet < p_busy`, the collapse treats every busy-fragment event in `(p_quiet, p_busy]` as a
     conflict. Showing that this is immaterial on one workload would not show it is immaterial on
     another. If the limb did fire, the remedy would be to withdraw `Guard` and `and_guard` and
     return to a single boundary. Both are published, WF-4's wire form carries `guards`, and
     withdrawing them is a major release. A falsifier that can fire only as a major is restated
     as a post-1.0 reopening. That is ADR-0066's route, and ADR-0075 took it for PS-15.

## The sentence of ADR-0054 this amends

ADR-0054 said the alias would be "renamed or deleted" in the pass that lifts VT-30's marker. This
is that pass, and the alias stays. Deleting `after` and `after_opt` would break a published
crate. Phase 17b lands only additive answers, and the owner's call keeps them through 1.x. This
does not supersede ADR-0054. The rest of it is carried out as written. Only that one clause is
replaced, by decision 2 above.

## Consequences

- From `1.0.0`, three things need a major release: a blanket setter that fills only unset
  boundaries, removing either spelling, and withdrawing the guard sequence.
- A downstream call to `after` or `after_opt` compiles with a deprecation warning that names the
  replacement. A crate built with `-D warnings` turns that warning into an error, which is the
  point. The fix is the rename, and behaviour does not change.
- The ledger's VT-30 provisional row is struck as frozen at 17b. Its `freeze-by-17b` disposition
  is removed, and `kb-open-question-vt-30-marker-stale-unscheduled-001` is resolved by this record.

## What reopens it

After `1.0.0`, any of these reopens it, and only as a major with its own record: a workload where
per-guard boundaries cost more than they save, an adapter that cannot evaluate guards
independently, or a replication ingest policy that needs to re-blanket an assembled condition.

## Alternatives that lost

- **A multi-guard benchmark scenario for limb 2.** This would measure a falsifier whose firing
  could not be acted on within 1.x. The owner chose to retire it by record.
- **Deleting `after` and `after_opt` in this pass**, as ADR-0054 anticipated. It lost because it
  is a break, and 17b sends breaks back to phase 17.
- **Typestate.** It lost for the reasons in decision 6.
