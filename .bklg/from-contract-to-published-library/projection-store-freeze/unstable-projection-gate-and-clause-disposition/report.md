---
item: "HS-S0016"
stage: report
created: "2026-08-15"
updated: "2026-08-15"
---

# Report — The module stops lying about its own maturity

## Findings Ledger

**Verdict: nine of nine ACs satisfied. Nothing blocked, nothing deferred, nothing
stubbed.** `cargo xtask ci` green, run whole, with all four `OPTIONAL` steps
**run** and none skipped. AC-014's second arm (`_decomposition.md:307-311`) was
read as settled input and executed, not re-decided.

### AC by AC

| AC | result | what proves it | where it is mounted |
| --- | --- | --- | --- |
| **AC-001** | **Met** | `tests::the_projection_port_is_behind_an_off_by_default_feature` and `tests::the_gate_is_mounted_on_the_module_and_its_re_exports` (`xtask/src/main.rs:1003-1039`), both red before the gate and green after; workspace feature powerset exit 0 over 62 combinations | `crates/happenstance-core/Cargo.toml:68` + `crates/happenstance-core/src/lib.rs:128-130`, `:160-166` |
| **AC-002** | **Met** | Diff review recorded in `implementation-report.md` `## Changes`: the "conformance suite does not cover it yet" sentence is **replaced**, the replacement names PS-2's bar, the two testkit-side shapes, what would clear it and HS-P0016 at phase 12; the invariant section is byte-identical | `crates/happenstance-core/src/projection.rs:3-42` |
| **AC-003** | **Met** | Gate log: `documentation (no default features)`, `documentation (default features)`, `feature powerset`, `wasm32 feature powerset` — all four ran, all green. **EC-001 did not fire**, because the crate-doc link was repaired in the same commit as the gate | `crates/happenstance-core/src/lib.rs:38-45`; `conformance = ["unstable-projection"]` at `Cargo.toml:97` |
| **AC-004** | **Met** | `tests::every_crate_that_names_a_projection_item_opts_in` (red → green) and `tests::the_typed_layer_makes_no_promise_it_does_not_keep`; six manifests name the feature explicitly, `happenstance` does not; `crates/happenstance-testkit/src/contract.rs`'s `skip_line` untouched | sqlite `:37`, ladybug `:18`, postgres `:18`, neon `:18`, sync `:26`, testkit `:47` |
| **AC-005** | **Met** | `spec_trace::tests::the_projection_family_is_checked_against_its_suite` — red with the assertion message, green after the flip — plus its `SY`-still-abstains guard; `cargo xtask spec-trace` exit 0, §7.1–§7.2 regenerated with `--write` | `xtask/src/spec_trace.rs:1750-1755` |
| **AC-006** | **Met** | The thirty-eight-row disposition table in `implementation-report.md` `## Changes`, PS-1 – PS-38 with no gaps; §1.3's four figures corrected by hand at `spec/SPECIFICATION.md:219-222`; `check_stated_census` green | `spec/SPECIFICATION.md` §4 and §1.3 |
| **AC-007** | **Met** | `git diff spec/SPECIFICATION.md` changes no bolded `MUST` sentence; PS-3's discharge at `:4809-4824` carries the playbook's three components — SHOULD verbatim, named *as* a discharge, code and tests cited | `spec/SPECIFICATION.md:4809-4824` |
| **AC-008** | **Met** | `references/adr/0030-the-checkpoint-reports-the-commits-that-happened.md`; atom staged at `.kb/_intake/2026-08-15-adr-0030-checkpoint-progress.md`; number allocated at `RUNBOOK.md:299` on ADR-0029's precedent; `redkiln validate --kb` passes and no file under `.kb/decisions/` changed | `RUNBOOK.md:299` + `references/adr/` + `.kb/_intake/` |
| **AC-009** | **Met** | The reconciliation section in `implementation-report.md` `## Notes` answers all three boxes with figures — citations **359 → 376**, the clause-range disagreement reported (16 clauses) rather than reconciled — and `CHANGELOG.md:1273-1305` carries the `[Unreleased]` entry with no pass rate | `CHANGELOG.md` `[Unreleased]` + the report |

### What the story found that its own spec did not predict

1. **PS-31's second conjunct was unmet.** *"The port MUST say so"* had no
   instrument and, as it turned out, no discharge: nothing in `projection.rs`
   mentioned the outward-writing exclusion. Discharged here
   (`crates/happenstance-core/src/projection.rs:56-71`), which is what
   `RUNBOOK.md:3932-3934` assigns to this phase.
2. **`RULE_FILES` was already correct.** Only `has_suite` was stale. Half of
   AC-005's mechanism was green on arrival and is recorded as a ratchet rather
   than claimed as a repair.
3. **§4.11's *"Every one is new"* had been false since slice 3.** Sixteen of the
   seventeen rules exist; the paragraph now says which one does not and why.
4. **The `conformance` × `unstable-projection` relation had a cost the spec did
   not name.** Gating the probe on both would have turned
   `examples/outside-projection-adapter`'s deliberately one-flag manifest — the
   artefact that exists to model that exact cost — into a two-flag one. Implying
   the gate keeps HS-S0015's claim true byte for byte.

### Deferred, routed, and owned elsewhere

Nothing is deferred *within* this story's ACs. Four findings are **recorded inside
the clauses they are about and routed**, which is AC-006's fourth disposition
bucket and not a deferral of an AC:

| Finding | Strength | Owner |
| --- | --- | --- |
| PS-8 — the `MUST` binds the method's existence, the rule its behaviour | `dependent` | ADR-0017's range; a further decision |
| PS-13 — the rule asserts the converse of the `MUST` | `dependent` | ADR-0017's range; §4.11's `READS_THROUGH_BATCH` tension |
| PS-28 — *"the last good position"* is undefined | `undetermined` | resolved when the rule is written (HS-P0011) |
| PS-29 — *"without being polled for it"* reaches past *"observable through the API"* | `independent` | ADR-0019 defers the observability design to HS-P0011 |

PS-32's correction to ADR-0007's Context is **recorded as owed and deliberately
not performed**: ADR-0007 is accepted and immutable, so the correction is a
superseding atom's, and `/redkiln:kb-ingest` authors atoms. Staged in the intake
file for the next wave.

### No verdict was pronounced

Nothing in the diff says the freeze held (HS-P0015's) or that 0.1 ships behind the
feature (HS-P0016's, at phase 12). Both the module header
(`crates/happenstance-core/src/projection.rs:33-37`) and PS-3's discharge
(`spec/SPECIFICATION.md:4821-4824`) route those explicitly. No pass rate over the
mutant set appears anywhere (ADR-0010).

### Hand-off to the slice-mate

`whole-gate-run-and-proof-artefact` inherits a tree whose feature table, clause
markers and `spec-trace` configuration are final, and a measured number for the
powerset widening rather than a surprise. The whole-gate run it records must be
made **after** this commit, on a clean tree.
