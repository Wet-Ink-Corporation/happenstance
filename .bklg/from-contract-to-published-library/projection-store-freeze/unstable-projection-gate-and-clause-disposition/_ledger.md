---
item: "HS-S0016"
stage: implement
created: "2026-08-12T13:46:11.103Z"
updated: "2026-08-12T13:46:11.103Z"
---

# Acceptance ledger — The module stops lying about its own maturity

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

```yaml
- id: AC-001
  criterion: "**GIVEN** an adapter author runs `cargo add happenstance-core` and reads the crate's feature table, **WHEN** they look for the projection port, **THEN** `unstable-projection` is listed there, is absent from `default = [\"std\", \"memory\"]`, and gates `pub mod projection;` and its re-exports — so a build that names `ProjectionStore` without opting in fails to compile rather than silently handing them a port nothing has ever failed. The opt-in is one documented flag and is reversible by removing it; nothing else in the crate changes shape when it is off."
  satisfied: true
  evidence: "`crates/happenstance-core/Cargo.toml:68` declares `unstable-projection = []`; `:35` keeps `default = [\"std\", \"memory\"]` without it. `crates/happenstance-core/src/lib.rs:128-130` gates `pub mod projection;` and `:160-166` the re-exports, copying the `memory` attribute pair verbatim. Tests: `tests::the_projection_port_is_behind_an_off_by_default_feature` and `tests::the_gate_is_mounted_on_the_module_and_its_re_exports` (`xtask/src/main.rs:1003-1039`), red before the gate landed and green after; `cargo hack check --workspace --feature-powerset --no-dev-deps` exit 0 (gate log, step `feature powerset`)."
  mount_point: "crates/happenstance-core/src/lib.rs:98,115 (export block :98-124) + co-mount crates/happenstance-core/Cargo.toml [features]"
  verifying_test: "cargo hack check --workspace --feature-powerset --no-dev-deps (xtask/src/main.rs:546-556) — the combination without `unstable-projection` compiles and does not expose `ProjectionStore`"

- id: AC-002
  criterion: "**GIVEN** that same author opens `projection.rs` to learn what \"provisional\" costs them, **WHEN** they read the first screen, **THEN** the module tells them the truth as of this merge: what PS-2 actually requires (two adapters at opposite ends of the batch-shape axis), what this project shipped instead (two testkit-side shapes), what would clear the bar, and who decides the 0.1 exposure — and the false sentence *\"the conformance suite does not cover it yet\"* is **replaced**, not deleted. The reason sits at the module header where the gate sent them, not three screens down."
  satisfied: true
  evidence: "`crates/happenstance-core/src/projection.rs:3-42` — the `# Status: provisional` block is replaced by `# Status: behind unstable-projection, and this is why`, which names PS-2's bar, the two testkit-side shapes this project shipped, what would clear it, and HS-P0016 at phase 12 (`RUNBOOK.md:601`). The false sentence *\"the conformance suite does not cover it yet\"* is gone and its replacement says why it was false. The invariant section that followed it is byte-identical (`git diff crates/happenstance-core/src/projection.rs` shows 39 insertions and 8 deletions, all inside the header). Before/after recorded in `implementation-report.md` (`## Changes`)."
  mount_point: "crates/happenstance-core/src/projection.rs:1-11 (module header), reachable from crates/happenstance-core/src/lib.rs:98"
  verifying_test: "Diff review recorded in implementation-report.md: crates/happenstance-core/src/projection.rs:1-11 before/after, with :13-30 byte-identical; cross-checked against spec/SPECIFICATION.md:4760-4775 and RUNBOOK.md:601"

- id: AC-003
  criterion: "**GIVEN** the whole gate runs after the module is gated, **WHEN** `cargo doc -p happenstance-core --no-default-features` and both feature powersets execute, **THEN** every one is green: the crate-doc table's `[`ProjectionStore`]` link at `lib.rs:38` is repaired the way the `memory` hazard was (not by widening the doc build), and the stated relation between `conformance` and `unstable-projection` holds for **every** combination including `conformance` without the gate. An author on any feature selection gets documentation that builds."
  satisfied: true
  evidence: "`cargo xtask ci` exit 0 with every optional step run and none skipped — `documentation (no default features)`, `documentation (default features)`, `feature powerset` and `wasm32 feature powerset` all in the step ledger (`implementation-report.md` `## Gates`). The crate-doc link at `crates/happenstance-core/src/lib.rs:38` is de-linked with the D13 reason stated at `:39-45`; the doc build was not widened. `conformance = [\"unstable-projection\"]` (`crates/happenstance-core/Cargo.toml:97`) is the stated relation and the comment at `:69-96` says why."
  mount_point: "crates/happenstance-core/src/lib.rs:38 (crate-doc table) + crates/happenstance-core/Cargo.toml [features]"
  verifying_test: "cargo xtask ci steps: no-default-features doc build (xtask/src/main.rs:494-513), workspace powerset (:546-556), wasm32 powerset (:558-591)"

- id: AC-004
  criterion: "**GIVEN** an author reading any sibling crate to learn the house pattern, **WHEN** they inspect its manifest, **THEN** every crate that names a projection item enables `unstable-projection` **explicitly** — sqlite, ladybug, postgres, neon, sync, and testkit once the suite is in it — rather than relying on workspace feature unification, which is per-build and does not hold under the `-p` doc and powerset steps. `happenstance-sqlite`'s existing `projection-store` feature *enables* the gate rather than shadowing it, and `happenstance` gains no passthrough because it re-exports no projection item. The one text surface the design signs off — the suite's declined-capability line — is unchanged."
  satisfied: true
  evidence: "`crates/happenstance-sqlite/Cargo.toml:37` (`projection-store = [\"happenstance-core/unstable-projection\"]` — enables rather than shadows), `crates/happenstance-ladybug/Cargo.toml:18`, `crates/happenstance-postgres/Cargo.toml:18`, `crates/happenstance-neon/Cargo.toml:18`, `crates/happenstance-sync/Cargo.toml:26`, `crates/happenstance-testkit/Cargo.toml:47`. `crates/happenstance/Cargo.toml` names it nowhere. Tests: `tests::every_crate_that_names_a_projection_item_opts_in` (red before, green after) and `tests::the_typed_layer_makes_no_promise_it_does_not_keep` (`xtask/src/main.rs:1042-1062`). `crates/happenstance-testkit/src/contract.rs`'s `skip_line` and its call site are untouched (`git diff` shows no change to that file)."
  mount_point: "crates/happenstance-{sqlite,ladybug,postgres,neon,sync,testkit}/Cargo.toml dependency + [features] blocks; crates/happenstance-sqlite/Cargo.toml:33"
  verifying_test: "cargo hack check --workspace --feature-powerset --no-dev-deps (xtask/src/main.rs:546-556) + per-crate `cargo doc -p`; text-surface no-diff review against crates/happenstance-testkit/src/contract.rs:500-507"

- id: AC-005
  criterion: "**GIVEN** the verdict author later asks the checker whether the projection clauses cite rules that exist, **WHEN** `cargo xtask spec-trace` runs, **THEN** it answers instead of abstaining: `has_suite` recognises the `PS-` prefix (`xtask/src/spec_trace.rs:1735-1737`), `RULE_FILES` names the projection rules file (`:71-90`), check 4 resolves every `PS` rule citation, and §7.2's regenerated region computes each `†` from `resolvable` — so the seventeen daggers on the `PS` block become a checked statement rather than an accurate one. Both comments say why the old exclusion was right and is no longer."
  satisfied: true
  evidence: "`xtask/src/spec_trace.rs:1733-1755` — `has_suite` gains the `PS-` prefix with the comment saying why the old exclusion was right and is no longer; `RULE_FILES` (`:88-93`) already named the projection rules file and its comment is unchanged. Test: `spec_trace::tests::the_projection_family_is_checked_against_its_suite`, which FAILED before the flip with the message quoted in `implementation-report.md` `## TDD Evidence`, and its guard `spec_trace::tests::the_replication_family_still_abstains_and_the_rest_do_not`. `cargo xtask spec-trace` exits 0 and reports `no problems found; §7.1–§7.2 matches the checker`; §7.2's `PS` block is regenerated by `--write`."
  mount_point: "xtask/src/spec_trace.rs:71-90 (RULE_FILES) and :1735-1737 (has_suite); documentary mount spec/SPECIFICATION.md:8507-8753 (generated region, --write only)"
  verifying_test: "cargo xtask spec-trace (mandatory gate step) exits zero; cargo test -p xtask covers the checker's own expectations"

- id: AC-006
  criterion: "**GIVEN** the verdict author needs to cite one clause rather than read thirty-seven, **WHEN** they open PS-1 – PS-37, **THEN** each carries a maturity marker and a rule citation that someone read against **today's** tree and recorded a verdict for: accurate as it stands / moved on a named accepted decision's authority / discharged / gap-and-routed. PS-31 and PS-32 are dispositioned as this phase's documentation MUSTs, with PS-32's correction to ADR-0007's Context taken as a superseding record — reasoning inside a decision that still stands is never touched. If any marker moved, §1.3's four figures are corrected **by hand** and left outside the generated markers."
  satisfied: true
  evidence: "Per-clause disposition table for PS-1 – PS-38, one row each, in `implementation-report.md` `## Changes`. PS-31 and PS-32 are dispositioned there as this phase's documentation MUSTs, with PS-32's correction taken as a superseding record rather than an edit to `.kb/decisions/0007-projection-runner-decodes.md` (unmodified — `git diff --stat` shows no file under `.kb/decisions/`). §1.3's four figures corrected by hand at `spec/SPECIFICATION.md:219-222` (200→201, 198→199, 49→50) and left outside the generated markers. `cargo xtask spec-trace` green, including `check_stated_census`."
  mount_point: "spec/SPECIFICATION.md — PS-1 – PS-37 clause bodies, §1.3 hand count at :219-222; disposition table in this story's implementation-report.md"
  verifying_test: "cargo xtask spec-trace (marker presence, falsifier length at xtask/src/spec_trace.rs:653-678, check_stated_census at :459-508) + the per-clause disposition table review covering PS-1 – PS-37 with no gaps"

- id: AC-007
  criterion: "**GIVEN** an author who read a `MUST` last month and built against it, **WHEN** they re-read it after this merge, **THEN** the sentence they built against is byte-identical: every obligation this project met is recorded in the playbook's safe form — the `MUST` stays verbatim, the discharge is named *as* a discharge so a satisfied obligation cannot be mistaken for a relaxed one, and the code and the test are cited so it is guarded rather than a claim about a moment in time. PS-3 is discharged this way: its `[PROVISIONAL]` falsifier (PS-2's bar met before 0.1) has **not** occurred, so its marker does not move; what changed is that its SHOULD now points at a feature that exists. No satisfied `MUST` is deleted."
  satisfied: true
  evidence: "No `[FROZEN]` MUST sentence changed: `git diff spec/SPECIFICATION.md` adds paragraphs beneath PS-1, PS-8, PS-13, PS-19, PS-21, PS-22, PS-28 and PS-29 and alters none of their bolded sentences. PS-3's discharge is recorded at `spec/SPECIFICATION.md:4809-4824` in the playbook's safe form — the SHOULD verbatim, the discharge named as a discharge, and the evidence cited to `crates/happenstance-core/Cargo.toml:68`, `crates/happenstance-core/src/lib.rs:130` and the two `xtask/src/main.rs` tests — with its `[PROVISIONAL]` marker unmoved because its falsifier (PS-2's bar met before 0.1) has not occurred."
  mount_point: "spec/SPECIFICATION.md — every [FROZEN] MUST sentence in PS-1 – PS-37; PS-3 at :4776-4790"
  verifying_test: "git diff over spec/SPECIFICATION.md showing zero byte changes inside any [FROZEN] MUST sentence, reviewed against .kb/playbooks/repairing-a-frozen-clause-without-amending-it.md:73-93"

- id: AC-008
  criterion: "**GIVEN** the sweep routed a residual defect to nobody — PS-1 archetypally, since the ADR queue has no group containing it (`RUNBOOK.md:296-298`) — **WHEN** this story repairs it, **THEN** the repair arrives as a **decision record**, never a line edit: the full record in `references/adr/`, its atom staged in a dated `.kb/_intake/` file whose name cannot overwrite a prior wave's audit trail, and its number *allocated* in the RUNBOOK queue on ADR-0029's `\"(unscheduled — the queue had no number for it)\"` precedent (`RUNBOOK.md:287`). No `.kb/` atom is hand-written and no accepted atom's body is edited."
  satisfied: true
  evidence: "`references/adr/0030-the-checkpoint-reports-the-commits-that-happened.md` — the full record, minting PS-38 rather than editing `[FROZEN]` PS-1. Its atom is staged at `.kb/_intake/2026-08-15-adr-0030-checkpoint-progress.md`, date-prefixed and slug-suffixed so it cannot overwrite the `2026-08-13-*` waves. The number is allocated in the queue at `RUNBOOK.md:299` on ADR-0029's `(unscheduled — the queue had no number for it)` precedent. `redkiln validate --kb` passes; `redkiln doctor` reports the six standing `template-drift` advisories and no seventh; `git diff --stat` shows no modification under `.kb/decisions/` and exactly one addition under `.kb/_intake/`."
  mount_point: "references/adr/<allocated-number>-<slug>.md + .kb/_intake/<date>-<slug>.md + the ADR queue row in RUNBOOK.md"
  verifying_test: "redkiln validate --kb (accepted-decision immutability against HEAD) && redkiln doctor (exactly six standing template-drift advisories); git diff --stat shows no modification under .kb/decisions/"

- id: AC-009
  criterion: "**GIVEN** phase 6 is exiting and the next reader must trust the phase, **WHEN** the reconciliation runs, **THEN** its three standing boxes are recorded for this clause family: every clause the phase's ADRs discharge read against the code *as it now stands*; the phase's clause range compared against the union of ADR-0017 / 0018 / 0019's ranges, with any disagreement **reported** rather than reconciled by widening an ADR; and `spec-trace`'s citation count not fallen. The `[Unreleased]` changelog entry makes `CHANGELOG.md:19-22`'s standing promise true, names what retires the exemption, and quotes no pass rate. Nothing in the diff pronounces the freeze verdict (HS-P0015) or the 0.1 exposure verdict (HS-P0016)."
  satisfied: true
  evidence: "Reconciliation section in `implementation-report.md` `## Notes` answers `RUNBOOK.md:3810-3819`'s three boxes with figures: citations 359 → 376 (not fallen), clause range compared against the union of ADR-0017/0018/0019's ranges with the disagreement reported rather than reconciled, and every clause the phase's ADRs discharge read against today's tree. `CHANGELOG.md:1273-1305` carries the `[Unreleased]` entry naming the feature, the semver exemption and what retires it, with no pass rate over the mutant set (ADR-0010). No verdict-shaped sentence: nothing in the diff says the freeze held or that 0.1 ships behind the feature; `spec/SPECIFICATION.md:4821-4824` and `crates/happenstance-core/src/projection.rs:33-37` both route those to HS-P0015 and HS-P0016."
  mount_point: "CHANGELOG.md [Unreleased] section (:24-) + the reconciliation section of this story's implementation-report.md, against RUNBOOK.md:3810-3819"
  verifying_test: "cargo xtask spec-trace citation count compared pre- and post-merge (RUNBOOK.md:3810-3819 third box) + changelog review against .kb/decisions/0010-the-suite-must-prove-itself.md and a verdict-sentence scan of the whole diff"
```
