---
item: HS-S0064
stage: spec
created: 2026-08-12T13:47:03.102Z
updated: 2026-08-12T13:47:03.102Z
template_sig: 87bbf1d0
rendered_sig: 61b84b23
---

# Spec — The naive-arm control and the Postgres mutant column

## Scope lock

| What | Path |
| --- | --- |
| Initiative | `.bklg/from-contract-to-published-library/initiative.md` — DoD 5 (`:373`), DoD 6 (`:375`) |
| Project | `.bklg/from-contract-to-published-library/postgres-and-neon-stores/project.md` — AC-002, AC-004, DR-1, DR-9 |
| This spec | `.bklg/from-contract-to-published-library/postgres-and-neon-stores/postgres-rule-controls/spec.md` |
| Key briefs | `.../postgres-and-neon-stores/_decomposition.md` — *Architecture brief* §9.1 (the seam), §10 (the owed decorator); *Testing brief* Notes §2 (the control), §4 (the rule-function seam), §6 (gating) |
| Signed-off design | `.../postgres-and-neon-stores/_design.md` — **no user-facing surface**, approved 2026-08-12. This story renders none. |
| Story map row | `.../postgres-and-neon-stores/_storymap.md` — *Slices*, `postgres-rule-controls`; *Coverage*, AC-002 and AC-004 rows |
| Discovery | `.../postgres-rule-controls/discover.md` — the signal ledger, the four answered questions, and the named wrong implementation |
| Roadmap pointer | `RUNBOOK.md:687` — the instrument-portfolio row that has read "**Adapter at phase 10**" since phase 3; `spec/SPECIFICATION.md:2845-2850` — the clause that names the decorator and owes it to this phase |

## One-line PR slice

The CF-13 rule is shown to *reject* a naive `nextval()` arm of the same adapter,
and the rule set is driven directly from the Postgres side as the mutant
control — so "`PostgresEventStore` passed" means the adapter had to work for it.

## Executive summary

`postgres-append-and-frontier-head` (HS-S0062) lands a real `append` and a
frontier `head`, and turns `event_store_conformance!` and
`event_store_model_conformance!` green against a live Postgres. This PR is the
**delta that makes that greenness mean something**: two controls that answer
one question — *would this rule have noticed?*

1. **The adapter-shaped control.** Build a naive `nextval()` arm of
   `PostgresEventStore` — no visibility machinery — and show
   `nothing_below_an_observed_position_appears_later` (CF-13) **fails** it, by
   name, before the shipped mechanism is credited with passing it. Both columns
   come out of the same seam: `happenstance_testkit::rules::<name>` driven from
   `crates/happenstance-postgres/tests/`, inside the live Postgres job.
2. **The rule-strength control.** Build the poll-padding decorator over
   `PreCommitPositionStore` that `spec/SPECIFICATION.md:2848-2850` names and
   owes to *this phase*, calibrated against the poll count HS-S0062's real
   `append` actually needs — or record in `.kb/_intake/` why a real multi-poll
   `append` did not need it, for ADR-0024 to carry.

No `[FROZEN]` clause changes, and no ADR is owed by this story. ES-10's own
clause pre-authorises the only change that can fall out of control 2: "if it
fires, **the rule changes and this clause does not**."

## Context pack

Load-bearing decisions this story must honour. Everything deeper is a
signposted anchor — do not go looking for it until an AC sends you.

**A rule no adapter can fail is decorative, and this project is about to claim
one was hard-won.** `CLAUDE.md`'s conformance-suite corollary is explicit: name
a plausible wrong implementation and write it down. For CF-13 the *fixture*
instrument already exists — `PreCommitPositionStore`, whose defect is the
sequence "advanced *outside* the transaction that will publish the row"
(`crates/happenstance-testkit/tests/mutation_coverage/mutants.rs:3752-3758`).
What has never existed is the **adapter**-shaped copy of that defect, which the
instrument-portfolio table has been listing as "Adapter at phase 10" since
phase 3 (`RUNBOOK.md:687`). This story is that column.

**The naive arm is a throwaway, not a second permanent fixture.** A
feature-gated code path in `happenstance-postgres`, exercised once and recorded
— *not* a deliberately-broken `Fixture` kept alive in the tree. The testkit
already keeps one canonical wrong implementation per axis, and duplicating that
pattern for Postgres without a stated reason is exactly the undecided addition
the house rule warns against (*Testing brief* Notes §2). If the arm survives
merge as a maintained fixture, this story has shipped the wrong thing.

**The control mounts on the Postgres side, and that is a recorded seam
decision, not a preference.** AC-004's literal wording — "the phase-3 mutant
harness runs with `PostgresEventStore` in the pass column" — reads as a
`REGISTRY` row in `crates/happenstance-testkit/tests/mutation_coverage.rs:324`.
It compiles, and it is wrong twice over: it makes `happenstance-testkit`'s test
binary depend on an adapter (the adapter-on-adapter direction `CLAUDE.md`
forbids), and it puts a live server inside `cargo test --workspace`, which DR-9
forbids. The seam that works is the public rule functions —
`happenstance_testkit::rules::<name>(open: impl AsyncFn() -> F) -> RuleOutcome`
(`crates/happenstance-testkit/src/lib.rs:189`, `src/suite.rs:89`) — driven from
`crates/happenstance-postgres/tests/`, inside the live job. **Record the
decision.** If the implementer concludes after all that the registry entry is
right, that is a change to a dependency rule `CLAUDE.md` states, and it needs a
written decision — never a `Cargo.toml` edit (*Architecture brief* §9.1).

**Assert which rule failed, not that something failed.** The naive arm will
also be missing an `xid8` column and a frontier predicate; a decode panic or a
missing column masquerading as a visibility finding is the weak control that
records the right verdict for the wrong reason. The in-tree pattern is
`catch_unwind` over a **non-capturing** probe
(`crates/happenstance-testkit/tests/mutation_coverage/harness.rs:325-340`,
`:454`) — `Probe::run` is a function pointer precisely so the harness needs no
`AssertUnwindSafe` anywhere, and the first refactor anyone attempts (hoisting a
fixture out and capturing `&Fixture`) is what breaks that. Do not.

**The rule may be unable to fail here at all, and that is the second control.**
`spec/SPECIFICATION.md:2845-2850`: the rule polls two `append` futures A, B, B,
A; `Fixture` cannot express a poll budget; against a store whose `append` needs
three polls "the interleaving window never opens where the rule looks and the
rule cannot fail". A real Postgres `append` — acquire a connection, `BEGIN`,
execute, `COMMIT` — is exactly that multi-poll shape. So the naive arm *passing*
is the expected outcome of a naive run, and recording it as "the mechanism was
unnecessary" is the failure this story exists to prevent
(`discover.md`, *The wrong implementation*). The bounding instrument is named:
a poll-padding decorator over `PreCommitPositionStore`, which pads `append` to
*n* `Pending` returns while keeping the known defect, and the observation is
whether the rule still rejects it
(`.kb/open-questions/poll-count-bounds-the-visibility-rule.md`).

**Where the decorator lives, and what happens if it fires.** Beside the store it
wraps, in `crates/happenstance-testkit/tests/mutation_coverage/mutants.rs`: it
needs no server, stays single-threaded (`Rc`/`RefCell`), and preserves the
`UnwindSafe` probe shape the harness depends on. If the padded store passes —
the rule fails to reject a store with a known defect — then
`nothing_below_an_observed_position_appears_later` changes, in the same change,
with its reason given and a `CHANGELOG.md` entry naming the defect it detects
(CF-29's lint sweeps all three rule files, `xtask/src/lints.rs:525`,
`xtask/src/spec_trace.rs:85-89`). **ES-10's clause is not touched, and no ADR is
owed** — the clause settles that in advance. The rule's *name* must not change
either: `spec-trace` check 6 requires every rule in `RULE_FILES` to be claimed
by a clause, so a rename is a specification edit this story is not taking.

**The calibration number comes from the real adapter, not from taste.** The
open question's sub-question 3 asks exactly this: does *n* come from
`happenstance-postgres`'s actual `append` once phase 10 builds it, or from a
synthetic worst case? HS-S0062 has built it by the time this story starts —
that is what the `depends_on` edge buys — so *n* is measured off the shipped
`append`, and "an author choosing *n*" is the reference-store failure mode with
one more step.

**Nothing here may run in the default gate.** DR-9 and AC-011: `cargo xtask ci`
on a clean checkout with no Docker and no credentials exits zero. The Postgres
control is `#[ignore]`d / env-gated / `required-features`-gated at
**whole-invocation** grain; DR-5's one hard constraint is that gating may never
be a `#[cfg]` hiding a conformance rule out of a macro's expansion. The
decorator is the opposite case: it needs no server at all and therefore runs in
the ordinary `cargo test --workspace --all-features` alongside the rest of the
mutant harness.

**No literal position values.** `cargo xtask lint-position-literals` scans only
`suite.rs`, `model.rs` and `concurrency.rs` (`xtask/src/spec_trace.rs:85-89`),
so neither this story's control in `crates/happenstance-postgres/tests/` nor the
decorator under `crates/happenstance-testkit/tests/` is swept. CF-6 holds here
**by discipline**: compare against the positions the store actually assigned.

**The persona slice.** The reader served is the adapter author about to trust
this workspace's compliance claim, and the auditor of
`publication-and-positioning` (HS-P0016) reading it later. What they meet is not
a screen: it is a live-job log with a named rule failing against the naive arm
and passing against the shipped one, and a mutant harness that says out loud how
strong the rule was at the poll shape a real database has.

## Integration contract

- **Archetype**: `capability` — the deliverable is a run that reports, and a
  recorded verdict a future reader is bound by.
- **Slice / milestone**: `postgres-live-suite`. Slice-mates, implemented in one
  context and mounted as one surface: `postgres-schema-and-live-fixture`,
  `postgres-append-and-frontier-head`, `postgres-concurrency-family`.
- **Mount point**: `crates/happenstance-postgres/tests/rule_controls.rs` — a new
  integration-test target in the adapter's own `tests/` directory (the crate has
  none today), driving `happenstance_testkit::rules::*` directly and executed by
  the live Postgres CI job. Secondary mount, for the rule-strength half:
  `crates/happenstance-testkit/tests/mutation_coverage/mutants.rs` (the
  decorator) plus its `REGISTRY` row in
  `crates/happenstance-testkit/tests/mutation_coverage.rs:324` and its entry in
  the `for_each_mutant!` list (`:2056`).
- **Wires into**:
  - `happenstance_testkit::rules` — the public rule functions
    (`crates/happenstance-testkit/src/lib.rs:189`, `src/suite.rs:89`), returning
    `RuleOutcome` and panicking on failure.
  - `happenstance_testkit::{Fixture, RuleOutcome}`
    (`crates/happenstance-testkit/src/contract.rs:120-353`) — the control opens
    fixtures HS-S0060 authored (`PostgresFixture`); it authors none.
  - `PreCommitPositionStore` and the probe/`Verdict` machinery
    (`.../mutation_coverage/mutants.rs:3752`,
    `.../mutation_coverage/harness.rs:325-340`, `:454`) — consumed and wrapped,
    never rebuilt and never "fixed".
  - `impl SendEventStore for PostgresEventStore`
    (`crates/happenstance-postgres/src/event_store.rs:121-175`) — the shipped
    arm under test, and the feature-gated naive arm beside it.
  - The live Postgres CI job authored by `postgres-schema-and-live-fixture`
    (`.github/workflows/ci.yml`), whose
    `cargo test -p happenstance-postgres --all-features -- --ignored --show-output`
    invocation sweeps this target up with **no workflow edit** — consumed, not
    changed.
- **Renders surfaces**: **none.** `_design.md` records no user-facing surface
  for this project, approved at the `/redkiln:plan` design gate.
- **Public items**: none. `_design.md`'s `## Items` block is `N/A`. Everything
  this story adds is `pub(crate)`, a `#[test]`, or a `dev`/feature-gated path;
  no item joins `happenstance-postgres`'s or `happenstance-testkit`'s public
  API, and none may.
- **Conformance rule(s)**: `nothing_below_an_observed_position_appears_later`
  (CF-13, `crates/happenstance-testkit/src/suite.rs:5880`) — observed here as
  the *subject*, not the instrument: this story measures the rule rather than
  being measured by it. The rule may change (schedule, not name) if the
  decorator fires; the change carries its reason and a `CHANGELOG.md` entry.
- **Clause(s)**: discharges the phase-10 obligation recorded at
  `spec/SPECIFICATION.md:2845-2850` and supplies the adapter-level evidence
  under **ES-10**. **No clause text changes** — ES-10 is `[FROZEN]` and its own
  paragraph forecloses a clause change here.
- **Advances DoD scenario**: initiative **DoD 5** — "a store that does not
  serialise its writers passes the suite, with the position-visibility cost
  measured rather than estimated" (`initiative.md:373`). This story is what
  makes "passes" a claim rather than a coincidence, and it hands
  `adr-0024-position-visibility-mechanism` (HS-S0065) the control it cites.

## PR boundary

**In this PR**

- The feature-gated naive `nextval()` arm of `PostgresEventStore`
  (`position` allocated by the sequence, `head` as `max(position)`, no
  visibility predicate) and the `Cargo.toml` feature that selects it.
- `crates/happenstance-postgres/tests/rule_controls.rs`: the control that drives
  `happenstance_testkit::rules::*` against both arms, asserts the naive arm
  fails CF-13 **by name**, and records a per-rule outcome column for
  `PostgresEventStore`.
- The poll-padding decorator over `PreCommitPositionStore` in
  `crates/happenstance-testkit/tests/mutation_coverage/mutants.rs`, its
  `REGISTRY` row and `for_each_mutant!` entry — **or** the written excuse staged
  in `.kb/_intake/` for ADR-0024 to carry.
- If and only if the decorator fires: the schedule change to
  `nothing_below_an_observed_position_appears_later` in
  `crates/happenstance-testkit/src/suite.rs`, with its reason in the same change
  and a `CHANGELOG.md` entry naming the defect.
- The §9.1 seam decision, written into this story's own folder (`_ledger.md`)
  and, where it is load-bearing for ADR-0024, staged in `.kb/_intake/`.

**Explicitly not in this PR**

- The append path, the frontier `head`, the schema, `PostgresFixture` or the
  live CI job — all `postgres-schema-and-live-fixture` / HS-S0062's, consumed
  here.
- Choosing the ES-10 mechanism, or writing ADR-0024. This story supplies the
  control that decision cites; HS-S0065 decides
  (`_decomposition.md` *Architecture brief* §3, tension 1 — ADR-0024 is a
  deliverable, not prior art).
- Any edit to `spec/SPECIFICATION.md`, any `[FROZEN]` clause, or any rule
  **rename**. The clause settles the rule-versus-clause question in advance.
- Registering `PostgresEventStore` in `mutation_coverage.rs`'s `REGISTRY`, or
  any `happenstance-testkit → happenstance-postgres` dependency edge.
- The Neon negative control (`ProbeThenWriteStore`) — it reuses this story's
  seam, and that reuse is the only coupling;
  `neon-conflicting-position-verdict` (HS-S0070) owns it.
- Keeping a second, deliberately-broken Postgres `Fixture` alive in the tree.

**Merge DoD**: the naive arm is on record as *failing* the named rule and the
shipped arm as passing it, both from the same seam; the decorator is built and
calibrated or its absence is written down for ADR-0024; `cargo xtask ci --fast`
is green on a machine with no Docker and no credentials, and the live Postgres
job is green on the same tree.

The implementer MAY touch the wiring files named in the Integration contract to
mount this slice; that is not scope drift.

```
crates/happenstance-postgres/src/**
crates/happenstance-postgres/tests/**
crates/happenstance-postgres/Cargo.toml
crates/happenstance-testkit/tests/mutation_coverage.rs
crates/happenstance-testkit/tests/mutation_coverage/**
crates/happenstance-testkit/src/suite.rs
CHANGELOG.md
.kb/_intake/**
.bklg/from-contract-to-published-library/postgres-and-neon-stores/postgres-rule-controls/**
```

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| The naive arm exists, and only under a feature | `position` from `nextval()` / `bigserial`, plain `INSERT … RETURNING position`, `head` as `SELECT max(position)`, no `xid8` column read and no frontier predicate. Selected by an off-by-default Cargo feature (or a `cfg` chosen in its place); never the default build, never a `Fixture` impl that outlives this PR. | `crates/happenstance-postgres/src/event_store.rs:26-70` (the three candidate mechanisms and what discarding them costs); `_decomposition.md` *Testing brief* Notes §2 |
| The control drives public rule functions, not the registry | `happenstance_testkit::rules::<name>(open: impl AsyncFn() -> F) -> RuleOutcome` invoked from `crates/happenstance-postgres/tests/`. The testkit gains no dependency on any adapter and no live server enters `cargo test --workspace`. | `crates/happenstance-testkit/src/lib.rs:189`; `src/suite.rs:89`; `_decomposition.md` *Architecture brief* §9.1 |
| Failure is asserted **by name** | `catch_unwind` over a non-capturing probe, matching on the rule that panicked, so a decode panic or a missing column cannot be recorded as a visibility finding. `Probe::run` stays a function pointer; no `AssertUnwindSafe` appears anywhere. | `crates/happenstance-testkit/tests/mutation_coverage/harness.rs:325-340`, `:454` |
| Two columns, one instrument | The naive arm's `fail` and the shipped arm's `pass` for CF-13 are produced by the same call site in the same run, so the comparison is not across two harnesses. `RuleOutcome::Skipped` is a third, distinct outcome and is never read as a pass. | `crates/happenstance-testkit/src/contract.rs` (`RuleOutcome`); `crates/happenstance-testkit/src/suite.rs:5880` |
| The seam decision is recorded, not merely taken | The choice of the rule-function seam over a `REGISTRY` row is written down with its two structural reasons. Taking the registry route instead requires a written decision, never a manifest edit. | `_decomposition.md` *Architecture brief* §9.1; `crates/happenstance-testkit/tests/mutation_coverage.rs:324`, `:2056` |
| The poll-padding decorator wraps, and does not repair | It inserts *n* `Pending` returns into `PreCommitPositionStore::append` and keeps the defect intact — the sequence still advances outside the publishing transaction. Single-threaded `Rc`/`RefCell`, no server, no clock. | `crates/happenstance-testkit/tests/mutation_coverage/mutants.rs:3752-3758`; `.kb/open-questions/poll-count-bounds-the-visibility-rule.md` |
| *n* is measured off the shipped `append` | The calibration target is the poll count HS-S0062's real `append` needs (connection acquire → `BEGIN` → execute → `COMMIT`), not a number an author picked. Record how it was counted. | `.kb/open-questions/poll-count-bounds-the-visibility-rule.md` (sub-question 3); `spec/SPECIFICATION.md:2845-2848` |
| A decorator that fires changes the rule, never the clause | If the padded store passes, `nothing_below_an_observed_position_appears_later`'s schedule changes in the same commit, with the reason given and a `CHANGELOG.md` entry naming the defect. ES-10 is untouched, no ADR is owed, and the rule keeps its name. | `spec/SPECIFICATION.md:2848-2850`; `xtask/src/lints.rs:525` (`changelog_names_every_rule`); `xtask/src/spec_trace.rs:85-89` (`RULE_FILES`, check 6) |
| The registry stays exhaustive and honest | A new mutant needs a `REGISTRY` row with a **non-empty** `fails` list — `mutant_registry_is_exhaustive` rejects an empty one — and a matching `for_each_mutant!` entry. If the decorator's whole point is that the rule does *not* reject it, that verdict is the finding, and the row must state the truth rather than be made to pass. | `crates/happenstance-testkit/tests/mutation_coverage.rs:318-330`, `:2056` |
| Gating is whole-invocation | `#[ignore]`, a `required-features` gate, or an env read — the implementer's call. Never a `#[cfg]` hiding a rule out of a macro expansion (DR-5). `cargo test --workspace --all-features` exits zero with no server reachable. | `_decomposition.md` *Architecture brief* §7, *Testing brief* Notes §6; `.redkiln/config.yaml` (`integration_scoped: cargo xtask ci --fast`) |
| The live job picks it up with no workflow edit | The Postgres job already runs `cargo test -p happenstance-postgres --all-features -- --ignored --show-output`; an `#[ignore]`d target in the crate is swept up by it. If the chosen gating mechanism is not `--ignored`, the job's invocation is HS-S0060's to amend and this story states the requirement rather than editing the workflow. | `_decomposition.md` *Testing brief* Notes §6 (project-level merge-gate commands) |
| No literal position values anywhere added | Neither added location is scanned by `cargo xtask lint-position-literals`, so the constraint is stated and held by discipline: compare against positions the store actually assigned. | `xtask/src/lints.rs:628-640`; `xtask/src/spec_trace.rs:85-89`; `CLAUDE.md` (CF-6 corollary) |
| Nothing becomes public API | Everything added is `pub(crate)`, a `#[test]`, or feature-gated. `_design.md` declares no items; an item that became `pub` here is a semver promise nobody made. | `.../postgres-and-neon-stores/_design.md` (`## Items`, `## Visibility and stability` — both `N/A`) |

## Data and migrations

**No migration is authored or altered here.** Migration 1 — identity, time, tag
storage and the mechanism's column — belongs to
`postgres-schema-and-live-fixture`, and this story consumes it
(`_storymap.md`, *Slices*).

Two data-shaped facts the naive arm forces, and the constraint on both:

1. **The naive arm needs `position` allocated by a sequence.** Migration 1
   deliberately does **not** make `position` a `bigserial`
   (`crates/happenstance-postgres/src/event_store.rs:22-24`), so the arm either
   (a) reads its positions from a sequence created alongside the table under its
   own feature-gated DDL, or (b) runs against a variant schema applied only to
   the throwaway schema/database the fixture hands it. Either is acceptable.
   What is **not** acceptable is a second file under the crate's migration
   source: a maintained migration for a deliberately-broken arm is precisely the
   "permanent second fixture" *Testing brief* Notes §2 refuses.
2. **The mechanism's column is left unwritten, not dropped.** The arm skips the
   `xid8` write and the frontier predicate; it does not need the column removed.
   Keeping the shipped schema and changing only the SQL keeps the two arms
   comparable — the difference the control attributes a failure to is the
   *allocation and visibility logic*, not the table.

**Isolation.** Whichever of schema-per-fixture-instance or
database-per-fixture-instance HS-S0060 chose (`_decomposition.md` *Architecture
brief* §9.3 leaves it open) is inherited unchanged. The naive arm must run in
its own isolated backing store: one fixture instance is one isolated store, and
an arm that shares a schema with the shipped arm can pass or fail for reasons
neither arm owns.

**The decorator half touches no data at all.** It is in-memory, single-threaded
and serverless by construction, which is why it lives beside
`PreCommitPositionStore` rather than beside the adapter.

## Acceptance criteria

Each criterion is written from the reader it serves. Two personas carry this
story: **the adapter author** (`_discovery/distillation/personas-and-journeys.md`
§Persona 2, journey *Learn when you are finished* — "from a signature that
type-checks to a suite that says pass or fail and names why") and **the
evaluator** (§Persona 4, journey *Decide in one sitting* — a bounded look at
public evidence). Neither is served by a green tick; both are served by knowing
what the tick had to survive.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **GIVEN** an adapter author who has just read that `PostgresEventStore` passes CF-13 and wants to know what a *failing* Postgres store would look like, **WHEN** they read the merged tree, **THEN** a naive arm of the same adapter exists — `position` allocated by a sequence, plain `INSERT … RETURNING position`, `head` as `SELECT max(position)`, no `xid8` read and no frontier predicate — reachable only through an off-by-default Cargo feature (or a `cfg` chosen in its place), **AND** no deliberately-broken Postgres `Fixture` survives the merge as a maintained instrument: the default build, `cargo test --workspace --all-features` and the live job's shipped-arm run are all unchanged by its existence. | `crates/happenstance-postgres/tests/rule_controls.rs::naive_arm_is_reachable_only_under_its_feature` (compiles and runs only with the feature on; the default `cargo test -p happenstance-postgres` build does not contain the arm), plus `cargo xtask ci`'s `cargo hack --feature-powerset` step (`xtask/src/main.rs`) proving the new feature composes with every existing one. |
| AC-002 | **GIVEN** an adapter author who trusts this workspace's dependency rule ("no adapter may depend on another adapter", `CLAUDE.md`) and the promise that `cargo test --workspace` needs no server, **WHEN** the mutant control is added, **THEN** it is driven from `crates/happenstance-postgres/tests/rule_controls.rs` through the public `happenstance_testkit::rules::<name>(open: impl AsyncFn() -> F) -> RuleOutcome` seam, **AND** `happenstance-testkit` gains no dependency on any adapter and `mutation_coverage.rs`'s `REGISTRY` gains no row for a live store — with the §9.1 seam choice taken deliberately, not by default. | `cargo test --workspace --all-features` green on a machine with **no** server reachable, including `crates/happenstance-testkit/tests/mutation_coverage.rs::mutant_registry_is_exhaustive` — which would have to be re-satisfied had a live store entered the registry; plus `cargo tree -p happenstance-testkit -e normal,dev` showing no `happenstance-postgres` edge. |
| AC-003 | **GIVEN** the adapter author's question "would this rule have noticed?", **WHEN** the naive arm is driven through `rules::nothing_below_an_observed_position_appears_later` in the live Postgres job, **THEN** the run records a failure attributed to **that rule by name** — captured by `catch_unwind` over a non-capturing probe — **AND** a decode error, a missing column, a schema fault or a connection failure is reported as itself and can never be recorded as a visibility finding. | `crates/happenstance-postgres/tests/rule_controls.rs::naive_arm_fails_nothing_below_an_observed_position_appears_later`, run inside the live Postgres CI job. The test asserts on the captured panic's rule identity, not merely on the fact of a panic; a fault injected in the fixture's setup path makes it fail rather than pass. |
| AC-004 | **GIVEN** an evaluator reading the Postgres job's log to decide in one sitting whether "passes the suite" is a claim or a coincidence, **WHEN** the job runs, **THEN** one invocation emits a per-rule outcome column for `PostgresEventStore` in which every rule the seam drives reports `pass`, `fail`, or `RuleOutcome::Skipped` carrying the fixture's own stated reason — **AND** CF-13's `fail` for the naive arm and `pass` for the shipped arm are produced by the **same call site in the same run**, so the comparison is not across two harnesses, **AND** a `Skipped` is never rendered or read as a pass. | `crates/happenstance-postgres/tests/rule_controls.rs::postgres_rule_outcome_column`, run with `cargo test -p happenstance-postgres --all-features -- --ignored --show-output` in the live job; the assertion distinguishes all three `RuleOutcome` variants and fails if any outcome is unclassified. |
| AC-005 | **GIVEN** an adapter author who needs to know whether CF-13 can fail *at all* against a store whose `append` takes several polls — connection acquire, `BEGIN`, execute, `COMMIT` — **WHEN** the mutant harness runs with no server present, **THEN** a poll-padding decorator over `PreCommitPositionStore` pads `append` to *n* `Pending` returns while leaving the defect intact (the sequence still advances outside the publishing transaction), with *n* **measured off HS-S0062's shipped `append`** and the counting method written down, and it carries a `REGISTRY` row and a `for_each_mutant!` entry stating its true verdict — **OR** `.kb/_intake/` carries the written reason it was not built, addressed to ADR-0024, naming what a real multi-poll `append` demonstrated instead. | `cargo test -p happenstance-testkit --all-features` — `crates/happenstance-testkit/tests/mutation_coverage.rs`'s `for_each_mutant!` sweep plus `mutant_registry_is_exhaustive` — with the decorator registered; **or**, on the excuse branch, the staged `.kb/_intake/` file cited as evidence in `_ledger.md` and reviewed at the story gate. |
| AC-006 | **GIVEN** ES-10's own clause pre-authorising a rule change and forbidding a clause change, **WHEN** the padded store **passes** (the rule cannot detect a known defect at that poll shape), **THEN** `nothing_below_an_observed_position_appears_later`'s **schedule** changes in the same commit with its reason stated in its rustdoc and a `CHANGELOG.md` entry naming the defect it now detects, its **name** unchanged, `spec/SPECIFICATION.md` unedited and ES-10 still `[FROZEN]` and untouched; **AND** if the padded store is rejected, that observation is recorded instead and no rule changes. | `cargo xtask lint-changelog` (`xtask/src/lints.rs:525`, `changelog_names_every_rule` over `RULE_FILES`) and `cargo xtask spec-trace` check 6 (`xtask/src/spec_trace.rs:85-89`) — both already `REQUIRED` in `cargo xtask ci`; plus `git diff --stat spec/SPECIFICATION.md` being empty on the merge commit. |
| AC-007 | **GIVEN** a contributor on a clean checkout with no Docker and no credentials (DR-9, project AC-011), **WHEN** they run `cargo xtask ci --fast`, **THEN** it exits zero: every live-requiring test this story adds is gated at **whole-invocation** grain — `#[ignore]`, a `required-features` gate, or an env read — never a `#[cfg]` hiding a conformance rule out of a macro's expansion (DR-5), **AND** the serverless half (the decorator and its registry row) runs in the ordinary `cargo test --workspace --all-features`, **AND** the live job sweeps the new target up with **no** edit to `.github/workflows/ci.yml`. | `cargo xtask ci --fast` and `cargo xtask affected --base <base>` on a machine with no Docker and no credentials; the live job's existing `cargo test -p happenstance-postgres --all-features -- --ignored --show-output` invocation running the new target unchanged. |
| AC-008 | **GIVEN** the author of ADR-0024 (`adr-0024-position-visibility-mechanism`, HS-S0065) and, later, the auditor of `publication-and-positioning`, both of whom must be able to cite *why* "the adapter had to work to pass CF-13" is true, **WHEN** they open this story's folder, **THEN** they find the §9.1 seam decision written down with both of its structural reasons, the decorator's calibration (*n*, how it was counted) and its verdict, and whatever of that is load-bearing for ADR-0024 staged in `.kb/_intake/` for the runbook's ADR pass — **AND** no ADR is authored here as a side effect. | `_ledger.md` evidence rows citing the recorded decision and calibration; `.kb/_intake/**` staged where load-bearing; `redkiln validate --kb && redkiln doctor` green (no hand-written decision atom appears under `.kb/decisions/`). |

Coverage of the traced project ACs: **AC-002** ("the visibility rule is one the
adapter had to work to pass … a naive `nextval()` implementation of the same
adapter fails it") is carried by AC-001, AC-003, AC-005 and AC-006 — the arm, its
named failure, and the proof the rule could have failed at all. **AC-004** ("the
phase-3 mutant harness runs with `PostgresEventStore` in the pass column") is
carried by AC-002, AC-004 and AC-007 — the seam that mounts the column without
inverting a dependency or dragging a server into the default gate.

## Interaction quality

This story renders **no user-facing surface**. The signed-off design
(`.../postgres-and-neon-stores/_design.md`, approved by the repository owner on
2026-08-12) records `N/A — no user-facing surface` under `## Surfaces`,
`## Items`, `## The states the API must express` and `## Anti-patterns`, and
`design.capture` is deliberately absent from `.redkiln/config.yaml`, which makes
the perceptual review a declared skip rather than a silent pass. **No COMPOSITION
invariant binds this story, and none is owed an AC row** — inventing a density
budget, a hierarchy or a transience policy here would contradict a design a human
has already signed off.

What this story *does* produce that a human reads is the live job's log and the
mutant harness's output. The STATE-family invariants of RFC §6.7 apply to that
output, and each one that applies is carried by an **AC row in the table above**,
never by a bullet here:

| Invariant (STATE family) | Carried by | How it is verified |
| --- | --- | --- |
| **Non-occlusion** — the finding a reader came for is not covered by an unrelated failure. A decode panic, a missing column or a connection fault must not present as a visibility finding. | **AC-003** | `catch_unwind` over a non-capturing probe, asserting the *rule identity* of the panic (`crates/happenstance-postgres/tests/rule_controls.rs::naive_arm_fails_nothing_below_an_observed_position_appears_later`). |
| **In-place comparison, not a context jump** — the two columns a reader compares are produced by one call site in one run, not by two harnesses a reader must reconcile by hand. | **AC-004** | `postgres_rule_outcome_column` emits both arms' CF-13 outcome from the same seam in the same invocation. |
| **Preserved distinctions** — the three outcomes stay three. `RuleOutcome::Skipped` is not collapsed into `pass` on the way to the log. | **AC-004** | The assertion classifies all three variants and fails on an unclassified outcome; `--show-output` carries the fixture's stated reason through to the log. |
| **Reversibility** — the deliberately-wrong thing is opt-in and leaves no residue. Turning the feature off restores exactly the shipped adapter; nothing broken is maintained past this PR. | **AC-001** | Default build contains no arm; `cargo hack --feature-powerset` proves every combination still compiles. |
| **Reachable without privileged setup** — the equivalent of keyboard reachability for a non-visual surface: the serverless half is reachable by any contributor with no Docker and no credentials, and the live half fails loudly rather than quietly skipping. | **AC-007** (and EC-001) | `cargo xtask ci --fast` on a bare machine; whole-invocation gating, never a `#[cfg]` inside a macro expansion. |
| **Focus / scroll / selection preservation, keyboard navigation, revealed-vs-persistent chrome, density budget, hierarchy, named anti-patterns** | **N/A** | No surface exists to hold them (`_design.md`, `## Surfaces`). |

## Error conditions

| id | condition | required behaviour |
| --- | --- | --- |
| EC-001 | The live Postgres job runs with no server reachable, or with credentials that do not resolve. | The gated target **fails loudly**. It must not report success by having quietly run nothing: an `#[ignore]`d target that the live job forgot to un-ignore, or an env gate that silently returns, is the failure DR-5 forbids in its other form. The live job asserts that the control actually executed. |
| EC-002 | The naive arm panics for a reason that is not the visibility defect — a missing `xid8` column, a decode failure, a schema mismatch, a pool timeout. | Reported as itself, with the underlying error surfaced. AC-003's assertion is on the rule identity, so this path makes the control **fail**, never pass. Fix the arm's schema or SQL; do not widen the assertion to "something failed". |
| EC-003 | The naive arm **passes** CF-13. | This is the expected outcome of a naive run against a multi-poll `append` (`spec/SPECIFICATION.md:2844-2848`) and is **not** licence to conclude the mechanism was unnecessary. It is a rule-strength finding: it escalates to AC-005's decorator, and if the decorator confirms the rule is blind at that poll shape, to AC-006's rule change. Recording "the naive arm passed, so the mechanism is not needed" is the named wrong outcome this story exists to prevent (`discover.md`, *The wrong implementation*). |
| EC-004 | The decorator's honest verdict is that **no** rule rejects it, leaving an empty `fails` list. | `mutant_registry_is_exhaustive` (`crates/happenstance-testkit/tests/mutation_coverage.rs:2754`) rejects an empty `fails` list, and that rejection is the finding — not an obstacle to route around. The row must state the truth; the response is AC-006's rule change, in the same commit, with the reason given. Fabricating a `fails` entry to make the row pass is a defect, not a fix. |
| EC-005 | The naive arm and the shipped arm share a schema or database. | Rejected. One fixture instance is one isolated backing store (`CLAUDE.md`, the fixture rule); an arm sharing a schema can pass or fail for reasons neither arm owns, and the control's attribution is then worthless. Each arm runs in its own isolated schema/database, inheriting whichever isolation HS-S0060 chose. |
| EC-006 | The new Cargo feature breaks a feature combination (e.g. combined with `wasm32`-relevant or `sqlx` feature sets). | Caught by `cargo xtask ci`'s `cargo hack --feature-powerset` step, which is exactly the step that catches a feature that only builds in isolation. Fix the feature's gating; do not exclude it from the powerset. |
| EC-007 | The implementer concludes the `REGISTRY` route is right after all. | Permitted **only** as a written decision recorded in this story's folder and, if load-bearing, staged in `.kb/_intake/` — never as a `Cargo.toml` edit. It changes a dependency rule `CLAUDE.md` states (`_decomposition.md` *Architecture brief* §9.1), and it must also answer DR-9. |

## Non-functional

| id | requirement | why |
| --- | --- | --- |
| NF-001 | **Nothing added joins a public API.** Everything is `pub(crate)`, a `#[test]`, or feature-gated. No item is added to `happenstance-postgres`'s or `happenstance-testkit`'s public surface. | `_design.md`'s `## Items` and `## Visibility and stability` are both `N/A`. An item that became `pub` here is a semver promise nobody made, in a project whose sibling story removes `publish = false`. |
| NF-002 | **No literal position values** in either added location. Compare against the positions the store actually assigned. | `cargo xtask lint-position-literals` scans only `suite.rs`, `model.rs` and `concurrency.rs` (`xtask/src/spec_trace.rs:85-89`), so neither the control nor the decorator is swept. CF-6 holds here by discipline, and the specification permits gaps. |
| NF-003 | **The decorator stays inside the harness's shape.** Single-threaded `Rc`/`RefCell`, no server, no clock, no `tokio`; `Probe::run` stays a function pointer and no `AssertUnwindSafe` appears anywhere. | `crates/happenstance-testkit/tests/mutation_coverage/harness.rs:325-340`, `:454`. Hoisting a fixture out and capturing `&Fixture` is the first refactor anyone attempts and is exactly what breaks the `UnwindSafe` probe. |
| NF-004 | **No `#[async_trait]`, and generic code binds `EventStore`, not `SendEventStore`.** Import only one of the two names per module. | `CLAUDE.md` binding constraints 1 and 4 (ADR-0001). The control lives in a Postgres-only target, but the habit is the constraint — and `happenstance-testkit`'s rule functions are generic over `Fixture`. |
| NF-005 | **The live job's wall time stays bounded.** The control adds a small, fixed number of extra fixture instances (one isolated store per arm), not a second full suite run per rule. | DR-9's spirit: a live job that doubles in cost is the one that gets quietly made non-blocking, which the project's risk table names. |
| NF-006 | **No new dependency on `happenstance-testkit`, and the MSRV is unchanged at 1.97.1.** | `CLAUDE.md`, ADR-0029. The testkit is a must-not-change seam for this project (`_decomposition.md` *Testing brief*, preamble); a dependency added for a control is a contract change smuggled in as a test. |
| NF-007 | **Determinism.** The decorator is deterministic by construction. The live control is not permitted to be flaky: if the naive arm's verdict varies run to run, that variance is itself the finding and is recorded, never retried away. | A flaky control is a decorative control, which is the failure mode this whole story is built against. |

## Implementation notes (non-prescriptive)

- **Order that keeps the two halves independent.** The decorator (AC-005/AC-006)
  needs no server and can be built and run first, on any machine, before the
  live job is even reachable. Doing it first also tells you early whether CF-13
  has teeth at a multi-poll shape, which changes how you read the naive arm's
  verdict. The naive arm (AC-001/AC-003/AC-004) then runs against the real
  fixture HS-S0060 authored.
- **Counting *n*.** The honest count is the number of times HS-S0062's real
  `append` future returns `Pending` before completion under the same runtime the
  rule uses. Instrumenting the future once (a counting wrapper in a throwaway
  test) is cheaper and far more defensible than reasoning about `sqlx`'s
  internals — and *how* it was counted is part of AC-008's record.
- **Spelling the naive arm.** A Cargo feature is the obvious lever, but a `cfg`
  set only by the test target is equally acceptable; the constraint is that the
  arm is off by default and does not survive as a maintained fixture. Whichever
  is chosen, the arm's schema need is a variant applied to the throwaway
  schema/database the fixture hands it, not a second file under the crate's
  migration source.
- **Asserting by name.** The in-tree pattern is already written down at
  `crates/happenstance-testkit/tests/mutation_coverage/harness.rs:325-340` and
  `:454`. Read it before inventing a second one; its function-pointer probe is
  the reason the harness needs no `AssertUnwindSafe`.
- **If the decorator fires.** The change is to the rule's *schedule* — how it
  interleaves the two `append` futures — not to its name and not to ES-10. A
  `CHANGELOG.md` entry naming the defect the rule now detects is mandatory and
  linted (`cargo xtask lint-changelog`), and `spec-trace` check 6 will reject a
  rename.
- **What to stage for ADR-0024.** HS-S0065 will cite this story. The useful
  staging is short: the naive arm's verdict, the decorator's verdict, *n* and how
  it was counted, and the seam decision's two structural reasons. Do not write
  the ADR — per this repository's practice, ADR authorship belongs to the
  runbook's ADR pass, not to a side effect of an implementation story.

## Tests and CI (merge gate)

Grounded in `_decomposition.md` *Testing brief* Notes §6 (the tree-local and
project-level merge-gate command lists) and `.redkiln/config.yaml`'s `verify:`
block.

| Tier | Command / path | Proves |
| --- | --- | --- |
| Story grain (`verify.affected_gate`) | `cargo xtask affected --base <base>` | Only what this diff could break, run on every commit: the two touched crates plus `xtask`. The story's own bar. |
| Integration grain (`verify.integration_scoped`) | `cargo xtask ci --fast` | The non-terminal project bar (`.redkiln/config.yaml`). Includes `cargo test --workspace --all-features`, which must exit zero **with no Docker and no credentials** — AC-007. |
| Unit / mutant (serverless) | `cargo test -p happenstance-testkit --all-features` → `crates/happenstance-testkit/tests/mutation_coverage.rs` (`for_each_mutant!` sweep, `mutant_registry_is_exhaustive` at `:2754`) | AC-005: the poll-padding decorator is registered, its `fails` list is non-empty-or-the-finding, and its verdict is recorded honestly. AC-002: no live store entered the registry, because this runs with no server. |
| Live conformance (new CI job, `testcontainers`, pinned Postgres minor) | `cargo test -p happenstance-postgres --all-features -- --ignored --show-output` → `crates/happenstance-postgres/tests/rule_controls.rs` | AC-003 (named failure of the naive arm), AC-004 (the per-rule outcome column with all three `RuleOutcome` variants distinguished), AC-001 (the arm builds and runs only under its feature). The job is HS-S0060's; this story adds a target it already sweeps up. |
| Feature hygiene | `cargo xtask ci`'s `cargo hack --feature-powerset` step (`xtask/src/main.rs`) | AC-001 / EC-006: the new feature composes with every existing one and does not only build in isolation. |
| Static — rule/spec integrity | `cargo xtask lint-changelog`; `cargo xtask spec-trace` | AC-006: if the rule changed, `CHANGELOG.md` names the defect it now detects (`xtask/src/lints.rs:525`) and check 6 still finds every rule in `RULE_FILES` claimed by a clause (`xtask/src/spec_trace.rs:85-89`) — which is what rejects a rename. |
| Static — position literals | `cargo xtask lint-position-literals` (informational here) | NF-002 is **not** covered by this lint at either added location; the lint's scope (`suite.rs`, `model.rs`, `concurrency.rs`) is stated so no one mistakes a green run for coverage. Held by review. |
| Static — API surface | `cargo doc --workspace --all-features --no-deps` (already `REQUIRED`) and `cargo xtask package-check` | NF-001: nothing added appears in either crate's public documentation. |
| Process | `redkiln validate --kb && redkiln doctor` | AC-008: the record validates, and no hand-written decision atom appeared under `.kb/decisions/`. |
| Whole gate before "done" | `cargo xtask ci` | `CLAUDE.md`'s standing instruction. Green on a machine with no Docker and no credentials **and** the live Postgres job green on the same tree is this story's merge DoD. |

## Risks and coupling (PR-scoped)

| Risk | Likelihood / impact | Mitigation, in this PR |
| --- | --- | --- |
| **The naive arm passes and is read as vindication.** The rule polls A, B, B, A; a real `append` needs several polls; the window never opens where the rule looks. | **High** / **High** — this is the expected outcome, not the unlikely one | EC-003 makes it a rule-strength finding by construction, and AC-005's decorator is what turns "it passed" into a measured statement about the rule rather than about the adapter. The story is not done on a passing naive arm alone. |
| **The failure is recorded for the wrong reason** — a missing column or a decode panic banked as a visibility finding. | Medium / High | AC-003 asserts the rule's identity, not the fact of a panic, using the in-tree `catch_unwind`-over-a-function-pointer pattern. EC-002 makes the mis-attributed case fail. |
| **The `REGISTRY` shortcut.** Adding `PostgresEventStore` to `mutation_coverage.rs`'s `REGISTRY` reads as the literal discharge of project AC-004 and *compiles*. | Medium / High | AC-002 states the seam, `_decomposition.md` §9.1 gives both structural reasons, and EC-007 makes the alternative a written decision rather than a manifest edit. The two failures it causes — a testkit→adapter edge and a live server inside `cargo test --workspace` — are each independently caught by AC-002's verification. |
| **The naive arm survives as a maintained fixture.** | Medium / Medium | AC-001 and the *Testing brief* Notes §2 refusal; the arm is off by default and carries no `Fixture` impl, and the data section forbids a second migration file for it. |
| **A live job that flakes gets made non-blocking.** | Medium / High | NF-007 (variance is the finding, not a retry) and the project's own risk row: a flaky live job is fixed or reported, never made non-blocking without a recorded decision. |
| **Coupling to HS-S0062 (`postgres-append-and-frontier-head`).** *n* is measured off its shipped `append`, and the shipped arm's `pass` column is its `append` passing. If HS-S0062's `append` changes shape after this lands, *n* is stale. | Medium / Low | AC-008 records *how* *n* was counted, so a later change re-runs a documented procedure rather than re-deriving one. Both stories are slice-mates in `postgres-live-suite` and land in one context. |
| **Coupling to HS-S0070 (`neon-conflicting-position-verdict`).** It reuses this story's public-rule-function seam for `ProbeThenWriteStore`'s negative test. | Low / Medium | The seam is `happenstance-testkit`'s existing public API, not something this story invents; the only thing HS-S0070 inherits is the *pattern*, recorded under AC-008. No shared helper crate is created here — if one is wanted later, `_decomposition.md` *Testing brief* Notes §4 already flags it as a decision to record. |
| **Coupling to HS-S0065 (ADR-0024).** It cites this story's controls. | Low / High if missed | AC-008 stages the citable facts in `.kb/_intake/`; this story writes no ADR, so the two cannot deadlock on each other. |
| **A rule change slips past review.** | Low / High | `cargo xtask lint-changelog` and `spec-trace` check 6 are both `REQUIRED` gate steps; a rule change without a `CHANGELOG.md` entry, or a rename, fails the gate. |

## Dependencies

**Blocks on** (must be merged first):

- `postgres-append-and-frontier-head` (HS-S0062) — supplies the real `append`
  and frontier `head` the shipped arm's `pass` column is *about*, the multi-poll
  `append` *n* is measured off, and the two green conformance families this story
  turns into a meaningful claim. Transitively supplies
  `postgres-schema-and-live-fixture`'s migration, `PostgresFixture` and the live
  Postgres CI job, all consumed unchanged.

**Unlocks**:

- `adr-0024-position-visibility-mechanism` (HS-S0065) — declares this story in its
  `depends_on` (`_storymap.md`, *Slices*). It cites the naive arm's verdict and the
  decorator's calibration as the evidence that the chosen mechanism was not free.
- `neon-conflicting-position-verdict` (HS-S0070) — reuses this story's
  public-rule-function seam for `ProbeThenWriteStore`'s negative test
  (`_decomposition.md` *Testing brief* Notes §4). A pattern dependency, not a
  declared `depends_on`.
- `far-end-discharge-record` (HS-S0074) — reads the ES-10 adapter-level evidence
  this story produces when it records what is discharged, still exposed, or
  amended.

**Slice-mates** (`postgres-live-suite`, implemented in one context, mounted as one
surface): `postgres-schema-and-live-fixture`, `postgres-append-and-frontier-head`,
`postgres-concurrency-family`.

## Anchors (progressive disclosure)

Do not read these up front. Each row says when it becomes load-bearing.

| Anchor | Why it is load-bearing | When to open | Serves |
| --- | --- | --- | --- |
| `crates/happenstance-postgres/src/event_store.rs` | Lines 22-70 state that migration 1 deliberately does **not** make `position` a `bigserial`, and lay out the three candidate visibility mechanisms with what discarding each costs. The naive arm is defined by subtraction from this. Lines 121-175 are the `impl SendEventStore` the shipped arm lives in. | Before writing the naive arm. | AC-001 |
| `crates/happenstance-testkit/src/lib.rs` | Line 189, `pub use suite::rules;` — the single line that makes the whole §9.1 seam possible, and the proof that driving the rule set from an adapter's `tests/` needs no new API. | Before wiring `rule_controls.rs`; cite it if anyone proposes the `REGISTRY` route. | AC-002 |
| `crates/happenstance-testkit/src/suite.rs` | Line 89 is the `rules` module's shape; line 5880 is `nothing_below_an_observed_position_appears_later` itself, whose rustdoc already names `PreCommitPositionStore` as its model and describes the A, B, B, A polling that AC-006 may have to change. | Before running the rule against either arm; again if the decorator fires and the schedule must change. | AC-003, AC-004, AC-006 |
| `crates/happenstance-testkit/src/contract.rs` | Lines 120-353 define `Fixture`, `Capability` and `RuleOutcome` — including the fact that `Skipped` carries the fixture's stated reason, which is what AC-004's third column reports. | When building the outcome column and deciding how to render a decline. | AC-004 |
| `crates/happenstance-testkit/tests/mutation_coverage/mutants.rs` | Lines 3752-3758 are `PreCommitPositionStore` and, in a comment, THE DEFECT stated exactly: "the sequence, shared by every handle and advanced *outside* the transaction that will publish the row." The decorator wraps this store; the naive arm is its adapter-shaped copy. | Before writing either the decorator or the naive arm's SQL. | AC-001, AC-005 |
| `crates/happenstance-testkit/tests/mutation_coverage/harness.rs` | Lines 325-340 and 454 are the in-tree `catch_unwind`-over-a-non-capturing-probe pattern, and the reason `Probe::run` is a function pointer rather than a closure. Copying this is how AC-003 asserts by name without an `AssertUnwindSafe` anywhere. | Before writing the "this rule failed" assertion. | AC-003, NF-003 |
| `crates/happenstance-testkit/tests/mutation_coverage.rs` | Line 318-330 is `REGISTRY` and the note that `mutant_registry_is_exhaustive` rejects an empty `fails` list; `:2056` is `for_each_mutant!`; `:2754` is the exhaustiveness test itself. A new mutant needs a row in both, and EC-004's honest-verdict case is decided here. | When registering the decorator — and immediately if its `fails` list would be empty. | AC-005, EC-004 |
| `spec/SPECIFICATION.md` | Lines 2844-2850 are the entire second control in five sentences: the A, B, B, A poll schedule, `Fixture`'s inability to express a poll budget, the three-poll blindness, the poll-padding decorator named and **owed by phase 10**, and the pre-authorisation "if it fires, the rule changes and this clause does not." | Before deciding whether the decorator is in scope, and before any response to a firing decorator. | AC-005, AC-006 |
| `.kb/open-questions/poll-count-bounds-the-visibility-rule.md` | The open question in full, including sub-question 3 — whether *n* comes from `happenstance-postgres`'s real `append` or from a synthetic worst case. This story answers it with a measurement, which is why the `depends_on` edge exists. | Before choosing *n*. | AC-005, AC-008 |
| `.bklg/from-contract-to-published-library/postgres-and-neon-stores/_decomposition.md` | *Architecture brief* §9.1 (the seam and the "record it as a decision" instruction), §10 (the owed decorator), §7 (gating left to the implementer with one hard constraint); *Testing brief* Notes §2 (the throwaway-arm rule), §4 (the rule-function seam and the shared-helper decision), §6 (the exact merge-gate command lists). | §9.1 and Notes §2 before writing anything; Notes §6 before choosing the gating mechanism. | AC-001, AC-002, AC-005, AC-007 |
| `.bklg/from-contract-to-published-library/postgres-and-neon-stores/postgres-rule-controls/discover.md` | The signal ledger, the four answered questions (seam, decorator scope, decorator home, whether a rule change needs an ADR — it does not) and *The wrong implementation* in full: the naive arm that passes, and why that is the expected outcome. | First, if any of this spec's decisions feel arbitrary. | AC-003, AC-005, EC-003 |
| `.bklg/from-contract-to-published-library/postgres-and-neon-stores/project.md` | AC-002 and AC-004 in their own words, and DR-1, DR-5, DR-7, DR-9 — the design rules that forbid a silent skip, a `#[cfg]`-ed-out test, a side-effect ADR, and a Docker-dependent default gate. | When judging whether a shortcut is permitted. | AC-002, AC-007, AC-008 |
| `.bklg/from-contract-to-published-library/postgres-and-neon-stores/_design.md` | The signed-off design: no user-facing surface, `## Items` and `## Anti-patterns` both `N/A`, approved 2026-08-12. It is the citation for why this story owes no composition invariant and adds no public item. | If anyone asks for a composition AC or proposes a public API addition. | NF-001 |
| `xtask/src/lints.rs` | Line 525, `changelog_names_every_rule` — what a rule change owes `CHANGELOG.md`. Line 628-640, `no_position_literals` — and, by its `RULE_FILES` scope, what it does **not** cover here. | Only if the decorator fires (line 525), or when tempted to trust the position-literal lint (line 628). | AC-006, NF-002 |
| `xtask/src/spec_trace.rs` | Lines 85-89, `RULE_FILES`, feeding both the position-literal scan and check 6's requirement that every rule be claimed by a clause — which is what makes a rule *rename* a specification edit. | Before changing anything about the rule, including its name. | AC-006, NF-002 |
| `RUNBOOK.md` | Line 687, the instrument-portfolio row that has read "fixture, phase 3 … **Adapter at phase 10**" since phase 3. This story is that column, and the row is where a reader checks whether it was ever filled in. | When writing AC-008's record, so the record lands where the portfolio table points. | AC-008 |
| `CHANGELOG.md` | The file `cargo xtask lint-changelog` reads; a rule change without an entry naming the defect fails the gate. | Only on the firing branch of AC-006. | AC-006 |
| `.github/workflows/ci.yml` | The live Postgres job HS-S0060 authors, whose `cargo test -p happenstance-postgres --all-features -- --ignored --show-output` invocation sweeps this story's target up. Consumed, not edited — if the chosen gating mechanism is not `--ignored`, that is HS-S0060's amendment, not this story's. | Before choosing the gating mechanism; to confirm no edit is needed. | AC-007 |
| `.redkiln/config.yaml` | The `verify:` block wiring `cargo xtask affected` to the story grain and `cargo xtask ci --fast` to the non-terminal integration grain — the commands that run whether or not anyone types them. | Before assuming a local green means the story grain passed. | AC-007 |

## Clarifications resolved during spec

1. **The AC set is exactly the eight the front half enumerated** — AC-001
   through AC-008, unchanged. Nothing was added or dropped, and `_ledger.md`
   carries one row per id.
2. **Project AC-004's literal wording is discharged by the seam, not by the
   registry.** "The phase-3 mutant harness runs with `PostgresEventStore` in the
   pass column" is satisfied by AC-002 + AC-004: the *rule set* is driven from the
   Postgres side and a per-rule outcome column is emitted. A `REGISTRY` row is
   explicitly out of scope (EC-007), and taking it anyway requires a written
   decision.
3. **"Hard-won" is evidenced by two controls, and either one alone is
   insufficient.** A failing naive arm without the decorator leaves open whether
   the rule can fail at a real poll shape; a firing decorator without the naive arm
   leaves the adapter-shaped defect unmodelled. AC-003 and AC-005 are therefore
   both required, and AC-005 admits the written-excuse branch only because
   `spec/SPECIFICATION.md:2848-2850` and `_decomposition.md` §10 name it.
4. **A passing naive arm is a pass of the story, not a failure of it —
   provided it is recorded as a rule-strength finding.** EC-003 states this
   explicitly, because the tempting reading ("the mechanism was unnecessary") is
   the named wrong implementation.
5. **No composition invariants are owed.** `_design.md` records no user-facing
   surface and its `## Anti-patterns` block is `N/A`. The *Interaction quality*
   section translates only the STATE family onto this story's real output — the
   live-job log and the harness verdict — and every applicable invariant is carried
   by an AC row in the table, not by a bullet.
6. **No ADR is authored here, and none is owed.** ES-10's own clause
   pre-authorises the only change that can fall out of this work (the rule, not the
   clause). What ADR-0024 needs is *staged* under `.kb/_intake/` per AC-008;
   authorship belongs to the runbook's ADR pass, which is what
   `adr-0024-position-visibility-mechanism` (HS-S0065) is.
7. **NF-002 is stated, not gated.** `cargo xtask lint-position-literals` does not
   scan either location this story adds to. That is recorded plainly in the *Tests
   and CI* table so a green lint run is never mistaken for coverage.
8. **The live job is consumed, not amended.** If the gating mechanism the
   implementer chooses is not `--ignored`, this spec's requirement stands — the
   target must be picked up by the live job — but the workflow edit belongs to
   `postgres-schema-and-live-fixture`, and this story states the requirement rather
   than making the edit (AC-007).
