---
item: HS-S0043
stage: discover
created: 2026-08-12T13:02:05.911Z
updated: 2026-08-12T13:02:05.911Z
template_sig: 86ce4036
rendered_sig: a2f6b10b
---

# Discover — The reopen rule gets a negative control, and the durability clauses get verdicts

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Slice one-line: give `recorded_time_survives_a_reopen` the negative control it has lacked since phase 4 — a permanent registry row encoding a re-stamped `recorded_at` — and land the recorded verdicts for CF-17's rule shape, ES-35's marker and CF-14's deferral. | `_storymap.md`, *Slices* table, row `race-model-and-durability` / `reopen-negative-control-and-durability-verdicts` | Two halves: one mechanical (a mutant), one editorial (three clause verdicts). The second is the half with no gate behind it. |
| **AC-004** — this story's split is "the third proves the rule **can** fail". | `project.md`, AC-004; `_storymap.md`, *Coverage* | `schema-migration-and-identity` persists the `StoreId` and reads `recorded_at` back; `sqlite-fixture-and-whole-suite` declares `REOPEN` and takes the rules green; this story falsifies. |
| **AC-010** — the durability clauses leave with verdicts: CF-17's rule shape settled; ES-35 frozen on this adapter's evidence **or** restated as provisional with a falsifier naming what is still missing; CF-14's deferral confirmed or withdrawn with a reason. | `project.md`, AC-010 | Three verdicts, each of which may legitimately be "still open" — but none of which may be silence. |
| `dependsOn: sqlite-fixture-and-whole-suite` (HS-S0040) — the first fixture in the workspace that supplies `REOPEN` over a real durable medium, and therefore the first thing a durability verdict can be written *about*. | `_storymap.md`, *Merge order* item 3 | Without it, every reopen rule skips and ES-35's falsifier is unreachable. |
| `recorded_time_survives_a_reopen` "has no mutant reaching its headline assertion, only its setup anchor, so the sentence it exists for has no negative control until a durable adapter exists at phase 8." | `RUNBOOK.md:3205-3207` | The rule's structure makes this precise: three anchors then the headline. The anchors are "the event is readable before the reopen", "it survived the reopen at all" and "it is at the position the store assigned"; the headline is `all[0].recorded_at == recorded_at` (`crates/happenstance-testkit/src/suite.rs:2485-2521`). |
| The headline's own message: "a `RecordedAt` is persisted alongside the event, not recomputed when the store is opened. An adapter that restamps on replay hands every auditor the time of the last restart, and the one clock reading whose provenance the log itself attested is gone with no error and no symptom." | `crates/happenstance-testkit/src/suite.rs:2515-2521` | "No error and no symptom" is the definition of a defect only a negative control can find. |
| `LosingFixture` already exists and already fails `acknowledged_writes_survive_a_reopen`; `DurableFixture` is the fixture that supplies `REOPEN` at all, and keeps its `StoreId` "minted once at construction" across a reopen. | `crates/happenstance-testkit/tests/fixture_instruments.rs:64-83`, `:177-203`; `spec/SPECIFICATION.md:7578-7580` | The *losing* half of the reopen family is controlled. The *re-stamping* half is not — which is exactly the gap `RUNBOOK.md:3205-3207` names. |
| The three reopen rules are on `MUST_SKIP`: they are capability-gated on `REOPEN` and must report a skip rather than pass when it is declined — "a rule whose `require!` gate was deleted passes here in silence" is the hole CF-18 closes. | `crates/happenstance-testkit/tests/mutation_coverage.rs:3143-3163` | A negative control has to be careful not to be satisfied by a skip. |
| CF-17 `[PROVISIONAL]`: the fixture SHOULD be able to reopen — "invalidate every outstanding handle's process-level state such that a subsequent `connect()` observes only what was durably committed." Its `Rejects:` line is "nothing on its own — it is an enabling clause, and CF-14 carries the rejection." | `spec/SPECIFICATION.md:7570-7583` | "CF-17's rule shape" (AC-010) means deciding whether an enabling clause that rejects nothing on its own is the right shape now that a real durable adapter exists. |
| CF-14 `[DEFERRED]`: a rule MUST assert that an append acknowledged with `Ok` survives a reopen; it names `PRAGMA synchronous = OFF` **by name** as a wrong implementation the rule exists to reject, and records that it "landed with CF-17 rather than at phase 8". | `spec/SPECIFICATION.md:7471-7490` | This adapter is the first thing that can set that pragma, so CF-14's named rejection becomes reachable here for the first time. |
| ES-35 `[PROVISIONAL — axis: durability]`: "What is still unbuilt is the *adapter* far end — a real store that can lose a write under a real fault, rather than one instructed to. **Falsified by the first file-backed adapter.**" | `spec/SPECIFICATION.md:4142-4149`; `spec/SPECIFICATION.md:8098` | The marker text says this adapter falsifies it — but the axis has two halves and this adapter supplies only one. |
| Architecture brief §9's honest verdict: this adapter supplies the *reopen* far end and **not** the *fault* far end; `MID_BATCH_FAULT` defaults declined and nothing in this project's ACs asks for it. "This project can freeze the *reopen* half and must restate the falsifier that remains… rather than declare the axis closed." | `_decomposition.md`, *Architecture brief*, §9; `crates/happenstance-testkit/src/contract.rs:207-211` | The tempting verdict — ES-35 frozen — is the wrong one, and the brief says so before anyone reaches it. |
| If CF-17's settlement reshapes or adds a rule, `mutation_coverage::every_rule_has_a_mutant` fails until a `REGISTRY` row exists. "The row is not optional and not a follow-up." | `_decomposition.md`, *Architecture brief*, §10; `crates/happenstance-testkit/README.md:104-113`; `xtask/src/proof.rs:86` | This is the one story in the project that may add or reshape a conformance rule, so the literal-position bar is live here rather than vacuous. |
| The rule already compares against `all[0].position == position` — the position the store assigned before the reopen — not a literal. | `crates/happenstance-testkit/src/suite.rs:2509-2513` | Any reshaping must preserve that. `AUTOINCREMENT` makes gaps real and `GappedPositionStore` is the conformant store that fails a rule which forgets. |
| AC-T06: `recorded_time_survives_a_reopen` gets a negative control **before** this project claims AC-004, and the ledger cites which of the two mechanisms was used. A permanent registry row is preferred over a reverted local mutation, "because a permanent registry row is evidence a reviewer can re-run forever." | `_decomposition.md`, *Testing brief*, §5 and AC-T06 | Fixes the deliverable's form, not just its existence. |

## Questions

Open questions to resolve before specifying.

1. **Does the negative control go in `REGISTRY` or is it a reverted local
   mutation?** *Answered: `REGISTRY`.* A re-stamping store fails a named
   event-store rule — `recorded_time_survives_a_reopen` — so unlike the
   `BEGIN DEFERRED` racer it has a non-empty `fails` list and belongs in
   `crates/happenstance-testkit/tests/mutation_coverage/mutants.rs` rather than in
   `racers.rs`. Testing brief §5 prefers this form for the reason that matters: a
   reviewer can re-run it forever.
2. **How does a `mutants.rs` store express a reopen at all?** Deferred to `spec`
   and flagged as the real design problem. `mutants.rs`'s stores are `Rc`-backed
   and single-threaded; the reopen capability lives on the **fixture**, and
   `DurableFixture` (`crates/happenstance-testkit/tests/fixture_instruments.rs:75-110`)
   is the existing shape for "state that survives a reopen". The control is
   therefore a *fixture*-shaped mutant in the `DurableFixture`/`LosingFixture`
   family whose reopen re-stamps `recorded_at` from a fresh clock, with the
   registry row naming the exact rule it fails. Which file it lands in follows
   from that and is `spec`'s to fix.
3. **ES-35: freeze, or restate?** *Answered: restate as `[PROVISIONAL]` with a
   narrowed falsifier.* The clause's current falsifier — "falsified by the first
   file-backed adapter" — is satisfied for the *reopen* half and not for the
   *fault* half, and this project declines `MID_BATCH_FAULT` explicitly. Freezing
   would close an axis on evidence that does not reach it, and nothing in the gate
   would notice. The restated falsifier names what is missing: a store that loses a
   write to a **fault** rather than to an instruction.
4. **CF-14: confirm the deferral or withdraw it?** Deferred to `spec` with the new
   evidence named: this adapter is the first thing in the workspace that can
   actually set `PRAGMA synchronous = OFF`, which is the wrong implementation
   CF-14 names by name. Whether that makes the clause's rejection *reachable*
   enough to withdraw the deferral is a judgement the spec must make explicitly
   and record either way. A confirmed deferral with a reason satisfies AC-010; a
   silence does not.
5. **CF-17's rule shape.** Deferred to `spec`. The live question is whether an
   enabling clause whose `Rejects:` line reads "nothing on its own" is still the
   right shape now that a real durable adapter exists to reject something. If the
   answer reshapes or adds a rule, this story also owes the `REGISTRY` row that
   `every_rule_has_a_mutant` requires, in the same change.
6. **Does anything here amend a `[FROZEN]` clause?** Answered: **no.** ES-35,
   CF-17 and CF-40 are `[PROVISIONAL]`; CF-14 is `[DEFERRED]`. If the spec's
   verdict on CF-17 would require touching a frozen clause, that is a new ADR
   written first and a re-plan — `project.md`'s *Out of scope* is explicit.
7. **The append-condition SQL strategy.** Untouched. The `synchronous` and journal
   settings this story reads verdicts about are ADR-0022's, and are consumed here
   rather than re-decided.

## Decision

`recorded_time_survives_a_reopen` has been passing since phase 4 against nothing
that could fail its headline assertion, which by this repository's own standard
makes it decorative — and the three durability clauses that describe the axis
carry markers written when no file-backed adapter existed. This slice closes both.
The spec will cover: a permanent negative control for the re-stamping defect — a
fixture-shaped mutant whose reopen recomputes `recorded_at` from a fresh clock,
with a `REGISTRY` row in `crates/happenstance-testkit/tests/mutation_coverage/`
naming `recorded_time_survives_a_reopen` as the rule it fails and this adapter as
its provenance — plus recorded verdicts for CF-17's rule shape, ES-35's marker
(restated `[PROVISIONAL]` with the falsifier narrowed to a store that loses a
write to a **fault** rather than to an instruction, since this adapter supplies
the reopen far end and explicitly declines `MID_BATCH_FAULT`), and CF-14's
deferral, confirmed or withdrawn **with a reason** now that a store exists that
can set the pragma CF-14 names.

This is the one story in the project that may reshape or add a conformance rule,
so the literal-position bar is live rather than vacuous: any rule this story
touches compares against positions the store actually assigned — as the existing
rule already does at `crates/happenstance-testkit/src/suite.rs:2509-2513` — never
against a literal, because `AUTOINCREMENT` leaves gaps and `GappedPositionStore`
is a conformant store that fails a rule which forgets. And if CF-17's settlement
adds or reshapes a rule, the `REGISTRY` row lands in the same change, because
`mutation_coverage::every_rule_has_a_mutant` (`xtask/src/proof.rs:86`) fails until
it does. No `[FROZEN]` clause is amended here; if the CF-17 verdict turns out to
require one, that is a new ADR written first, not an edit.

## The wrong implementation

**The mutant, and it is the one the workspace has been unable to express for four
phases: a store whose reopen re-stamps `recorded_at` from the current clock.**
Concretely — `recorded_at` is not persisted as a column at all, or is persisted
and then overwritten on read with `now()`. Every existing check passes. The event
survives the reopen, so anchor one and anchor two hold. Its position is unchanged,
so anchor three holds. `acknowledged_writes_survive_a_reopen` is green.
`reopened_store_does_not_reissue_an_event_id` is green, because the `StoreId` is
persisted independently. `cargo xtask ci --fast` is green, all four macros are
green, and every auditor who later asks when an event happened is handed the time
of the last restart — "with no error and no symptom", which is the rule's own
message (`crates/happenstance-testkit/src/suite.rs:2515-2521`).

The only assertion that catches it is the headline equality, and
`RUNBOOK.md:3205-3207` records that **nothing in the workspace has ever reached
it** — every mutant that touches this rule fails at the setup anchor instead, so
the rule has been green for four phases without once being tested. `LosingFixture`
fails the *survival* anchor; `DurableFixture` passes everything. There is no
in-between store, and this project is the first that can build one, because you
need a durable medium before "persisted versus recomputed" is a distinction that
exists.

**Where it must live:** `crates/happenstance-testkit/tests/mutation_coverage/`,
as a permanent `REGISTRY` row rather than a temporary local mutation. It qualifies
for `REGISTRY` — unlike the `BEGIN DEFERRED` racer, which fails no event-store
rule and therefore belongs in `RACERS` (`racers.rs:10-18`) — because it fails one
named rule exactly. `every_mutant_states_its_provenance`
(`crates/happenstance-testkit/tests/mutation_coverage.rs:3059-3068`) requires the
row to name "the adapter shape or the scenario that makes it plausible", and here
the provenance is unusually strong: it is the shape `happenstance-sqlite` would
have had if `recorded_at` had been left out of migration 1, which is a live
possibility in this very project. Testing brief §5 requires the AC-004 ledger
entry to cite which mechanism was used and where; the registry row is the one a
reviewer can re-run.

**Two traps for the control itself.** First, a mutant fixture that declines
`REOPEN` fails nothing — the three reopen rules are on `MUST_SKIP`
(`crates/happenstance-testkit/tests/mutation_coverage.rs:3157-3163`) and a skip is
a legitimate outcome — so the control must declare `REOPEN: SUPPORTED` and be
wrong *inside* a working reopen. Second, `mutants_fail_exactly_their_declared_rules`
checks both directions, so a mutant that also loses the event would be declared
against two rules and would duplicate `LosingFixture` rather than covering the gap.
It must be wrong in exactly one way.

**The editorial mutant, which no test can catch: ES-35 marked `[FROZEN]` on this
adapter's evidence.** The clause's own falsifier reads "falsified by the first
file-backed adapter", this *is* the first file-backed adapter, and freezing it is
therefore the reading the text invites. `cargo xtask spec-trace` would stay green —
it checks that clauses name rules that exist and that citations resolve, not
whether a marker is *earned* — and the citation count would not fall. What is lost
is the distinction the durability axis is built on: `RUNBOOK.md:692` and
`spec/SPECIFICATION.md:8098` both say the far end is a store that loses a write to
a **fault** rather than to an instruction, and this project declines
`MID_BATCH_FAULT` explicitly with a stated reason. Freezing would record a far end
as filled by an adapter that never reached it — the same inversion of CF-26 the
handle-multiplicity story guards against, in the other axis. The control is
architecture brief §9's pre-written verdict and this story's obligation to restate
the falsifier rather than delete it.

## Gate: Discover

Tick each box once the sections above satisfy it. `redkiln advance` reads these
against `.redkiln/templates/gates/discover.md` — where each box's rationale is
written — and will not leave `discover` until every one is ticked. Keep each box on
one line; the parser matches line by line.

- [x] The problem is framed in one paragraph.
- [x] Prior art and constraints are recorded in the signal ledger.
- [x] Open questions are either answered or explicitly deferred.
- [x] The next stage (spec) has a clear starting point.
- [x] The wrong implementation this work rejects is named.
- [x] No conformance rule added here asserts a literal position value.
- [x] Any `[FROZEN]` clause this touches is changed by a new ADR, written first.
- [x] If a rule here seems wrong, it is fixed and the reason given in the same change.
