---
item: "HS-S0043"
stage: report
created: "2026-08-17"
updated: "2026-08-17"
---

# Report — The reopen rule gets a negative control, and the durability clauses get verdicts

## Findings Ledger

**Seven of seven ACs satisfied. Nothing blocked, nothing deferred.** A debt open
since phase 4 is paid — `recorded_time_survives_a_reopen`'s headline assertion has
a registered subject that reaches it and fails there — and three durability
clauses leave with written verdicts at unchanged maturity levels.

**Mount point:** `crates/happenstance-testkit/tests/mutation_coverage.rs` — the
mutant harness's composition root. The control is registered in all four places
that matter: the type (`mutants.rs:3687`), `for_each_mutant!`
(`mutation_coverage.rs:2176`), the `Declared` row with its `expect` pin
(`:1234`), and the `MODEL_COVERAGE` row (`:2484`).

**Second wiring point:** `crates/happenstance-testkit/tests/mutation_coverage/correct.rs:477`
— `Log::replayed`, the replay seam a reopen-shaped fixture needs and that `new` +
`append` could not supply.

| AC | Result | What proves it |
| --- | --- | --- |
| **AC-001** — a subject fails the rule at its **headline** assertion | **Met** | `mutants_fail_exactly_their_declared_rules` green with the pin quoting `suite.rs:2517-2522`. The pin was **proved to discriminate**: swapped for `LosingFixture`'s survival-anchor string it fails, and the message it printed instead was the headline sentence verbatim |
| **AC-002** — the stamp is deterministic and monotone, never a clock | **Met** | `RecordedAt::from_millis(TEST_RECORDED_AT.as_millis() + generation)` at `mutants.rs:3726`, generation a `Cell<i64>` on the fixture. Green on three consecutive runs at 0.24/0.26/0.25 s. No `SystemTime`, `Instant` or `now()` added anywhere under `mutation_coverage/` |
| **AC-003** — one defect, one rule | **Met** | The `Declared` row lists exactly one rule, and `mutants_fail_exactly_their_declared_rules` asserts both directions — so the control passes `acknowledged_writes_survive_a_reopen` and `reopened_store_does_not_reissue_an_event_id`. Structural, not incidental: the replay keeps position, `EventId` and payload and replaces only `recorded_at` |
| **AC-004** — registered in all four places, and the prose count corrected | **Met** | All four registrations cited above; the model doc comment moves from *"The twenty it does not catch"* to *"The twenty-one"* and its reopen bullet now names both subjects. `mutant_registry_is_exhaustive`, `every_mutant_states_its_provenance`, `the_model_rule_rejects_exactly_what_it_claims` green under `--all-features` |
| **AC-005** — three written verdicts, no level moved | **Met** | ES-35's falsifier restated to the **fault** far end; CF-17's rule shape **confirmed**; CF-14's deferral **confirmed and narrowed** to HS-P0013 and HS-P0014. `spec-trace`'s census identical before and after; the three status rows unmoved; the escalation note is `_adr-queue-escalation.md` beside this file |
| **AC-006** — the adapter's reopen pass, against a rule now failable | **Met** | `cargo test -p happenstance-sqlite --test conformance -- --show-output`: 90 passed, three reopen rules `ok`, both `MID_BATCH_FAULT` rules printing a `SKIP` line with `SqliteFixture`'s own sentence |
| **AC-007** — discoverable from the record alone | **Met** | One `CHANGELOG.md` `[Unreleased] / Added` entry naming the defect in CF-29's shape. `affected gate passed`; `lints` reports `27 atoms, all consistent`; `spec-trace` `389 citations checked` — the same count as `HEAD`, verified by running it against both versions of the file |

## The finding: the debt was real, and the naive fix would have re-opened it silently

`RUNBOOK.md:3205-3207` records that `recorded_time_survives_a_reopen`'s headline
sentence has had no negative control since phase 4. It is a sharp problem rather
than a bookkeeping one: `LosingFixture`, the one registered store that fails the
rule, fails at the **first** of its three assertions — its events are gone, so
line 2515's `recorded_at` comparison never executes. The rule's reason for
existing had never been shown to bite.

The obvious control is a fixture that re-stamps on reopen. **Written the obvious
way it passes**, and that is a documented property of this binary rather than a
surprise: `correct::stamp` spends the constant `TEST_RECORDED_AT` because CF-33
forbids a clock, so a replay through the correct path lands on the value it
replaced. `GappedPositionStore`'s doc comment has said so since phase 5.

So the naive version was written first and registered as declared-to-fail:

```text
`RestampingFixture` declares that it fails `recorded_time_survives_a_reopen`,
but the rule passed. Either the rule does not catch this defect after all —
which makes it decorative for this mutant — or the declaration names the wrong
rule
```

That is EC-002 arriving exactly as the spec predicted, caught by the harness
rather than by review. The answer is a **per-fixture reopen generation** —
deterministic, so the harness stays reproducible; strictly monotone, so the new
value can never equal the old; and not a clock, so CF-33 is untouched and the row
cannot go flaky at millisecond resolution.

And a pin that is *present* is not a pin that *bites*, so it was falsified too:
swapped for `LosingFixture`'s survival-anchor string, the harness reports *"a
different assertion fired"* and quotes the headline sentence. That output is the
direct evidence that this control reaches the third assertion.

## The verdicts, and what was deliberately not done

- **ES-35 stays `[PROVISIONAL]`**, falsifier restated: *a store that loses a write
  to a fault rather than to an instruction*. The clause's claim that the axis has
  *"nothing at the other end"* was false the moment `SqliteFixture` ran the reopen
  rules over a real file, and is corrected. The still-true half — no fixture in
  the mutant binary has a medium outside the process — is kept.
- **CF-17 stays `[PROVISIONAL]`**, rule shape **confirmed**: the first durable
  file-backed adapter expressed a reopen through the contract with nothing added,
  so the pre-empted `restart` split stays unbought. The sentence naming
  `DurableFixture` as the only supplier of the capability is corrected.
- **CF-14 stays `[DEFERRED]`**, narrowed: one of the three implementations its
  experiment named has answered with the one shape and needed no grading; the
  falsifier is now the two that have not, with HS-P0013 and HS-P0014 named.
- **No level moved.** That is checked mechanically, not promised:
  `spec-trace`'s census is identical before and after
  (`137 FROZEN, 47 PROVISIONAL, 12 DEFERRED, 5 NON-NORMATIVE`) and the three
  status rows still read `PROVISIONAL`, `DEFERRED`, `PROVISIONAL`. The case for
  promotion is written up in `_adr-queue-escalation.md` with what an ADR would
  have to decide and what each is gated on. No ADR and no `.kb/decisions/` atom
  was authored.
- **`RUNBOOK.md`'s stale prose counts were left alone**, per EC-010:
  `registry_len()` is printed and deliberately not asserted, and the fifty-mutant
  figure is a historical session log.

## Self-heal, recorded because it is a gate observation and not only a fix

`cargo xtask lints` went red on two `standards/rust/` citations that no longer
landed within ten lines of their subject, because this slice moved lines in
`mutation_coverage.rs` and `concurrency.rs`. Both were re-anchored here.

The observation worth carrying forward: **`cargo xtask affected --base main` does
not run `lint-constitution`**, so it printed `affected gate passed` while
`reachability_static` — the grain `.redkiln/config.yaml:48` wires — was red. A
story that runs only the affected gate can leave the static grain broken without
seeing it. Both are green now.

## Deferred

Nothing from this story. The fault far end of ES-35 remains open **by design** and
is now named precisely rather than generally; that is the deliverable, not a gap.
