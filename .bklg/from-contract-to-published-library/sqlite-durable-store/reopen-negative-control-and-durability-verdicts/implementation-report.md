---
item: "HS-S0043"
stage: implement
created: "2026-08-17"
updated: "2026-08-17"
---

# Implementation Report — The reopen rule gets a negative control, and the durability clauses get verdicts

> **STATUS: seven of seven ACs satisfied.** `recorded_time_survives_a_reopen` has
> carried a headline sentence since phase 4 that **nothing in the workspace had
> ever reached**. `RestampingFixture` reaches it and fails there, and the pin was
> proved to discriminate rather than merely to be present.
>
> Three durability clauses leave with written verdicts at **unchanged maturity
> levels**. `spec-trace` reports the same clause census before and after —
> `137 FROZEN, 47 PROVISIONAL, 12 DEFERRED, 5 NON-NORMATIVE` — which is the
> mechanical half of "no marker moved".

## TDD Evidence

| AC | Test | Red, and for what reason | Green |
| --- | --- | --- | --- |
| **AC-001** | `mutation_coverage::mutants_fail_exactly_their_declared_rules` | **Red twice, for two different right reasons.** First as EC-002 (below). Then, deliberately, with the `expect` pin swapped for `LosingFixture`'s survival-anchor string: *"declared to fail … at "the acknowledged event must have survived the reopen at all", but a different assertion fired"* — and the message it printed instead was the headline sentence verbatim | green with the headline pin |
| **AC-002** | the same test, run three times | The naive re-stamp is the red: `with_recorded_at(correct::TEST_RECORDED_AT)` re-spends the constant and the store **passes** the rule it declares it fails | green, 0.24 s / 0.26 s / 0.25 s — identical every run |
| **AC-003** | the same test, plus `every_rule_has_a_mutant` | Goes red if the control also loses events or reissues an identity, because the test asserts a subject fails *exactly* its declared rules | green |
| **AC-004** | `mutant_registry_is_exhaustive`, `every_mutant_states_its_provenance`, `the_model_rule_rejects_exactly_what_it_claims` | `mutant_registry_is_exhaustive` is red for a type in `mutants.rs` that never reaches `for_each_mutant!` — a control nothing runs, which is the exact drift the file's own procedure says it exists to catch | green |
| **AC-005** | `cargo xtask spec-trace`, the clause-status rows | Red if a citation is dropped or a rule name renamed by a clause edit | `389 citations checked`, no problems found; the three status rows unmoved |
| **AC-006** | `cargo test -p happenstance-sqlite --test conformance -- --show-output` | Re-run *after* the control landed, which is what makes the pass a pass of a rule shown to be failable | 90 passed; both `MID_BATCH_FAULT` rules `SKIP` with the fixture's own words |
| **AC-007** | `cargo xtask affected --base main`, `cargo xtask lints && cargo xtask spec-trace` | `lints` went red on two drifted constitution citations — see *Self-heal* | all green |

### The red that mattered: EC-002, written on purpose and caught by the harness

The spec's highest-probability failure is a re-stamp that lands on the value it
replaced. `correct::stamp` spends a **constant**, `TEST_RECORDED_AT`, because
CF-33 forbids a clock — so a fixture that replays through the correct stamp is a
fixture that passes. `GappedPositionStore`'s doc comment has recorded exactly
this since phase 5.

The control was therefore written that way **first**, registered as declared to
fail, and run:

```text
test mutation_coverage::mutants_fail_exactly_their_declared_rules ... FAILED

`RestampingFixture` declares that it fails `recorded_time_survives_a_reopen`,
but the rule passed. Either the rule does not catch this defect after all —
which makes it decorative for this mutant — or the declaration names the wrong
rule
```

The fix is the generation counter: `RecordedAt::from_millis(TEST_RECORDED_AT.as_millis() + generation)`,
where `generation` is a `Cell<i64>` on the *fixture* — because reopen is a
fixture operation, so the counter that makes the stamp monotone lives where the
reopen does.

### The second red: proving the pin discriminates

A pin that is *present* is not a pin that *bites*. With it swapped for
`LosingFixture`'s survival-anchor string:

```text
`RestampingFixture` is declared to fail `recorded_time_survives_a_reopen` at
"the acknowledged event must have survived the reopen at all", but a different
assertion fired. … Message: assertion `left == right` failed: a `RecordedAt` is
persisted alongside the event, not recomputed when the store is opened. …
```

That output is the direct evidence for AC-001: the assertion that actually fires
is the **third** one, the rule's own reason for existing — the one no registered
subject had ever reached. The swap was reverted.

## Commits

- `2d08e0d` — `feat(sqlite-durable-store): The reopen negative control, and three recorded verdicts`

## Changes

| File | Shape of the change |
| --- | --- |
| `crates/happenstance-testkit/tests/mutation_coverage/mutants.rs` | **`RestampingFixture`**, longhand, placed directly after `LosingFixture` so the two read as opposites — one loses everything, one loses exactly one field. Its doc comment carries the CF-4 provenance, the one-sentence reason it is not a `Defect` step, and the whole generation-counter argument. The module doc's longhand list moves from eight stores to nine |
| `crates/happenstance-testkit/tests/mutation_coverage/correct.rs` | **`Log::replayed(allocate, events)`** — the replay seam. `new` + `append` cannot express a reopen: `append` re-allocates positions and re-spends `stamp`, which is the opposite of both halves of a replay |
| `crates/happenstance-testkit/tests/mutation_coverage.rs` | The four registrations — `for_each_mutant!`, the `Declared` row with its headline pin, the `MODEL_COVERAGE` row — plus the model doc comment's arithmetic (twenty → **twenty-one** misses, twenty-two → twenty-three `Agreed` rows) and its reopen bullet, which now names both subjects and says why the model misses the second for a *sharper* reason than the first |
| `spec/SPECIFICATION.md` | ES-35's marker and closing paragraph, CF-14's marker and its "what stayed deferred" paragraph, CF-17's marker and its `Rule:` line, and the Durability row of the maturity portfolio table. **Four edits, each inside its own paragraph**, at unchanged levels |
| `CHANGELOG.md` | One `[Unreleased] / Added` entry naming the defect the control encodes, in CF-29's shape |
| `standards/rust/23-streams-and-state-machines.md`, `standards/rust/41-declarative-macros.md` | Two drifted `file:line` citations re-anchored — see *Self-heal* |

## Gates

| Command | Result |
| --- | --- |
| `cargo test -p happenstance-testkit --all-features --test mutation_coverage` | **10 passed; 0 failed**, 2.28 s |
| the same, `mutants_fail_exactly_their_declared_rules` ×3 | ok / ok / ok — 0.24 s, 0.26 s, 0.25 s |
| `cargo test -p happenstance-sqlite --test conformance -- --show-output` | **90 passed; 0 failed**; three reopen rules `ok`, two `MID_BATCH_FAULT` rules `SKIP` with the fixture's own reason |
| `cargo xtask lints` | `27 atoms, all consistent`; every stated rule count matches |
| `cargo xtask spec-trace` | `389 citations checked`, no problems found — the **same** count as `HEAD`, verified by running it against both versions of the file |
| `cargo xtask affected --base main` | **`affected gate passed`** |
| `cargo fmt --all -- --check` | clean |

## Notes

**Self-heal: two constitution citations, and why they were this slice's to fix.**
`cargo xtask lints` went red on
`standards/rust/23-streams-and-state-machines.md:191` and
`standards/rust/41-declarative-macros.md:197`, both `file:line` citations that no
longer landed within ten lines of their subject — because this slice's edits
moved lines in `mutation_coverage.rs` (the new `Declared` row) and in
`concurrency.rs` (the rewritten `CONTENDERS` doc comment). Both were re-anchored
in this change. Re-anchoring is the maintenance that lint exists to force: a
citation that points at the wrong place is worse than none, and leaving it would
hand the next story a red gate it did not cause. `standards/rust/` was outside
every story's declared PR boundary when this was written; after review it is an
entry in **this** story's boundary block, admitting citation re-anchoring only,
on the terms `benchmark-harness` set at `e020276` — the initiative's precedent
for a compelled excursion of exactly this class. The excursion is therefore
admitted rather than merely disclosed.

**And a third check that reports green without looking**, found while proving the
widening works: `redkiln verify --item HS-S0043 --grain story` prints
`[skip] boundary — no boundary declared`. The block is declared; the parser reads
the first fenced block under `## PR boundary` and stops at the next heading, and
this spec's block sits under the `### Merge DoD` subheading. Moving it up makes
`boundary` and `provenance` both run and both fail while `links.commits` is
empty — the boundary check then diffs the whole initiative branch — and
`redkiln record-links` is the orchestrating command's write, not an
implementer's. Recorded in the spec beside the block and routed, not worked
around.

Worth recording separately: **`cargo xtask affected --base main` does not run
`lint-constitution`**, so it reported `affected gate passed` while
`reachability_static` — `cargo xtask lints && cargo xtask spec-trace`, the grain
`.redkiln/config.yaml:48` wires — was red. Both grains are green now.

**The verdicts were written last, against evidence.** The spec's implementation
notes ask for exactly that, and the order was kept: the control landed, then
AC-006's SQLite run was observed, then the three clauses were edited. Writing
CF-17's *"the rule shape is confirmed"* before running the adapter's reopen rules
would have recorded a prediction as a verdict.

**No maturity marker moved, and the check is mechanical rather than a promise.**
`spec-trace`'s clause census is byte-identical before and after —
`201 clauses (137 FROZEN, 47 PROVISIONAL, 12 DEFERRED, 5 NON-NORMATIVE)` — and the
three status rows still read `PROVISIONAL`, `DEFERRED`, `PROVISIONAL`. The case
for promoting ES-35 and CF-17, and what an ADR would have to decide, is in
`_adr-queue-escalation.md` beside this file. Authoring an ADR here is forbidden by
the PR boundary and by `CLAUDE.md`'s two-places rule.

**`RUNBOOK.md`'s stale prose counts were left alone**, per EC-010: `registry_len()`
is printed by the gate and deliberately not asserted, and the fifty-mutant figure
at `RUNBOOK.md:1762` is a historical session log. The new count is stated where
this change's own record lands — the gate prints `79 registry rows` for the
event-store registry — rather than chased through the tree.
