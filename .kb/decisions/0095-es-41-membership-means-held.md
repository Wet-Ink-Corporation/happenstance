---
id: kb-decision-0095
title: ES-41's contains_event_id means held, not visible, and a rule stages the row that tells them apart
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0095
reversibility: low
phase: 13
supersedes: null
superseded_by: null
summary: >-
  The owner's call, 2026-10-11 (wi-33ab08, option A), recorded as an ADR because it adds a
  normative sentence to [FROZEN] ES-41 without changing its marker. Where ES-10's frontier
  separates a committed row from a visible one, "holds" means held in the committed log:
  contains_event_id MUST answer true for a row the store has committed and the frontier has not
  yet passed. That is what happenstance-postgres and happenstance-neon already answer, so no
  published behaviour changes. The reason is agreement, not duplicate prevention: ES-25
  evaluates a condition over what the store holds, and VT-8's uniqueness on the origin pair is
  what ingest deduplicates against, while ingest never probes first, so neither reading could
  produce a duplicate. The proof is additive. happenstance-testkit gains
  Fixture::HELD_ABOVE_FRONTIER, defaulted to declined, Fixture::arm_held_above_frontier, defaulted
  to arming nothing, and the rule contains_event_id_reports_a_held_row_above_the_frontier, which
  checks that read does not show the staged row before it asks the probe. Two mutants pin it:
  FrontierFilteredMembershipStore, the probe with the frontier predicate copied in, and
  UnarmedFrontierFixture, the fixture that claims the capability and arms nothing. CF-18's emitted
  check rejects a fixture that declines READ_YOUR_OWN_WRITES and inherits the new default. The
  Postgres and Neon fixtures stage the row with a BEFORE INSERT trigger that lifts xact_id above
  any frontier, because holding the server-wide frontier back would stall every sibling rule.
  Reopened by a caller that needs true to mean that a read yields the event now.
depends_on:
  - kb-decision-wi-33ab08
  - kb-decision-0028
  - kb-decision-0024
related:
  - kb-decision-wi-33ab08
  - kb-decision-wi-fa5c7b
  - kb-decision-0028
  - kb-decision-0024
  - kb-decision-0071
  - kb-decision-0051
source_paths:
  - spec/SPECIFICATION.md
  - crates/happenstance-testkit/src/contract.rs
  - crates/happenstance-testkit/src/suite.rs
  - crates/happenstance-testkit/tests/mutation_coverage/mutants.rs
  - crates/happenstance-testkit/tests/mutation_coverage/variants.rs
  - crates/happenstance-testkit/tests/frontier_fixture_declension.rs
  - crates/happenstance-postgres/src/event_store.rs
  - crates/happenstance-postgres/tests/support/mod.rs
  - crates/happenstance-neon/src/event_store.rs
  - crates/happenstance-neon/tests/support/mod.rs
last_reviewed: 2026-10-12
---

# ES-41's contains_event_id means held, not visible, and a rule stages the row that tells them apart

**Accepted by the owner on 2026-10-11** as option A of `wi-33ab08` ("go with your
recommendations"), and cleared for an unattended build on 2026-10-12 by `wi-fa5c7b`. The issue is
#109. No long-form record exists; this atom is the record. It is `door:one-way`, so the owner merges
the change that lands it.

## Context

ES-41 requires `EventStore::contains_event_id`, reporting whether **this** store holds an event with
a given identity. ADR-0028 froze it at phase 17 and left one reading open
(`.kb/decisions/0028-what-a-store-may-forget.md:186-193`): on a store where ES-10's frontier can
separate a committed row from a visible one, does "holds" mean held in the committed log, or
visible to `read`?

Two adapters have such a row. `happenstance-postgres` buys ES-10 with `xid8` and
`pg_snapshot_xmin` (ADR-0024), and `happenstance-neon` runs the same predicate over HTTP. A row is
visible once its writer's `xact_id` falls below `pg_snapshot_xmin(pg_current_snapshot())`. Between
commit and that moment, the row is committed, held and invisible. Both adapters' probes omit the
predicate, so both answer `true` for it, and both recorded the answer as unsettled.

No rule could tell the readings apart. `contains_event_id_reports_membership` probes rows it has
just read back, which are visible by construction. So a probe that carried the frontier predicate
passed the whole suite.

## Decision

1. **ES-41 reads "held".** Where ES-10's frontier separates a committed row from a visible one,
   `contains_event_id` MUST answer `true` for a row the store has committed and the frontier has not
   yet passed. The sentence is added to ES-41 as a clarification cited to this ADR. The clause stays
   `[FROZEN]`, and §7.1's counts do not move.
2. **The reason is agreement.** Membership agrees with the two other places the store answers about
   what it holds. ES-25 evaluates an append condition over what the store holds, not over what a
   read shows: filtering a committed row out of a condition would admit an append the consistency
   boundary forbids. VT-8's uniqueness on the origin pair refuses a second copy of a held event,
   and ingest deduplicates against it. A "visible" probe would say `false` about a row that both
   of those treat as present.

   The reason is **not** duplicate prevention, and an earlier draft of ES-41's freeze was withdrawn
   for claiming it was (`runbook/phases/17-breaking-window.md:355-359`). Ingest deduplicates
   inside the write and never probes first (`crates/happenstance-sync/src/ingest.rs:97-101`), so
   neither reading could produce a duplicate.
3. **No published behaviour changes.** Postgres and Neon already answer "held". Their notes at the
   method now cite this ADR in place of "recorded, not settled".
4. **The proof is a rule behind a defaulted capability**, which is the shape option A named.
   `happenstance-testkit` gains:
   - `Fixture::HELD_ABOVE_FRONTIER`, defaulted to a declension whose reason is the public constant
     `UNSTATED_HELD_ABOVE_FRONTIER`;
   - `Fixture::arm_held_above_frontier`, whose provided body arms nothing;
   - `contains_event_id_reports_a_held_row_above_the_frontier`. It appends an anchor and reads it
     back to learn the store's incarnation, arms the fixture, and appends again. It then requires
     that `read` not show the second event (the staging check) and that the probe answer
     `Ok(true)` for it. Positions are the ones `append` returned, never literals.

   The row cannot be staged through the port. Every store publishes what it commits sooner or
   later, and `[FROZEN]` CF-33 forbids a rule that waits or races for the gap. So the staging is
   the adapter's, as a mid-batch fault is.
5. **Two wrong implementations pin it**, registered in the testkit's mutant registry:
   - `FrontierFilteredMembershipStore`: the probe with the read path's frontier predicate copied
     in. It fails exactly the new rule, at the probe assertion. Its conformant twin,
     `HeldAboveFrontierStore`, differs from it in one function and passes everything.
   - `UnarmedFrontierFixture`: a fixture that declares the capability and leaves
     `arm_held_above_frontier` to the provided body. It fails exactly the new rule, at the staging
     check. That check is why the provided body may arm nothing rather than panic: a forgotten
     override fails in the adapter's own CI either way.
6. **CF-18 holds the default to the stores it is false about.** The default reason says the store
   has no frontier. That is a fact for most stores, so a fixture with no frontier may leave it.
   A fixture that declines `READ_YOUR_OWN_WRITES` has said its reads trail its commits, so for that
   fixture the sentence is false. `every_declined_capability_is_stated_by_this_fixture` therefore
   rejects such a fixture unless it states `HELD_ABOVE_FRONTIER` itself.
   `tests/frontier_fixture_declension.rs` holds all three cases.
7. **Postgres and Neon stage the row by lifting it, not by holding the frontier back.** Their
   fixtures support the capability with a `BEFORE INSERT` trigger that sets the new row's `xact_id`
   to `i64::MAX`. The append runs unchanged, and the read path's own predicate keeps the row out of
   `read` and `head`. Holding the frontier back would need an open transaction with an assigned
   `xid`. The frontier is server-wide, so for as long as that hold lasted it would stall every
   sibling rule's reads in the binary. That is CF-33's flake, induced on purpose. Neon has a second
   reason: one-shot HTTP has no transaction to keep open.

Items 4 to 7 are this change's design rather than the owner's words. Option A named a capability
defaulted to declined, a rule and a mutant. The rest is how this change built them, and the owner
reviews it at merge.

## Consequences

- A frontier adapter whose probe carries the predicate now fails a named rule, and its fixture
  cannot pass by silence: CF-18 rejects the inherited declension once `READ_YOUR_OWN_WRITES` is
  declined.
- Every adapter with no frontier reports one more skip, with the testkit's reason, and nothing
  else changes for it. Adding a rule is a testkit minor that can turn a passing adapter's CI red,
  as the CHANGELOG says. Here only a fixture that declines `READ_YOUR_OWN_WRITES` and stays silent
  goes red.
- The Postgres and Neon rule runs need a live server or endpoint, and this change compiles them
  without running them. Their first live CI run is the first execution of the trigger.
- The probe can briefly say `true` while `read` does not yet yield the event. The disagreement
  resolves as the frontier advances, and it is the cost option A accepted.

## What reopens it

A caller that needs `true` to mean that a read yields the event now (`wi-33ab08`'s flip
condition). Changing the reading after `1.0.0` would change a `[FROZEN]` clause's answer on two
published adapters, and it needs its own record.

## Alternatives that lost

- **Visible (option B).** Carry the frontier predicate into the probe on Postgres and Neon. The
  probe would then disagree with ES-25 and with VT-8's dedup. It would also change behaviour on two
  published adapters.
- **Leave it unsettled (option C).** `1.0.0` would ship an unpinned reading on a frozen method.
- **Hold the frontier back in the fixture.** It is faithful to how a row ends up above the frontier
  in production. It is also server-wide, so every sibling rule's reads would lag, and Neon cannot
  do it at all.
- **A defaulted capability that every fixture must state.** It would turn every adapter with no
  frontier red for repeating a sentence that is true of it. The coupling to
  `READ_YOUR_OWN_WRITES` catches exactly the fixtures for which the sentence is false.
- **A provided `arm_held_above_frontier` that panics**, the shape of `reopen` and
  `arm_read_fault`. The staging check already fails a forgotten override in the adapter's own CI,
  so a panic would add no protection, only one more panic in a provided body.
