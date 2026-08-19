---
item: "HS-S0027"
stage: report
created: "2026-08-16"
updated: "2026-08-16"
---

# Report — PS-33, PS-27, PS-30 settled and PS-18 excluded, on the record

## Findings Ledger

**Outcome: ten of ten ACs satisfied. Nothing deferred, nothing blocked.**

The mount point is `spec/SPECIFICATION.md` — the verdicts land in the clause bodies of
§4.6, §4.8 and §4.9 and in §1.3's hand census, and the document is held to the tree by the
`specification traceability` REQUIRED step (`xtask/src/main.rs:315`), which runs in
`cargo xtask ci`, in `cargo xtask ci --fast`, and in the story-grain affected gate. A
second mount, not required by the spec but chosen deliberately, is
`crates/happenstance/tests/projection_clauses.rs`: five executed tests that pin each
verdict to the fact about the tree it was derived from, so the count cannot silently stop
being true.

| AC | result | proved by | notes |
| --- | --- | --- | --- |
| AC-001 | satisfied | `spec-trace` green; `projection_clauses.rs::no_checkpoint_pump_exists_in_the_contract_crate`; `redkiln validate --kb` | PS-33 is `[NON-NORMATIVE`, and its body states the verdict in **branch two**: the falsifier fired, the pump collapses upward, and the superseding ADR is staged at `.kb/_intake/0032-adr-0031-the-runner-collapses-upward.md`, cited by filename. The count is in the clause, so the reader never opens an ADR. |
| AC-002 | satisfied | `::no_skip_and_record_path_is_offered` + `spec-trace` | Exclusion branch, because the tree decided it: `run_projection` has no `on_error`, no `SkipPolicy`, no policy argument. The clause says so, names HS-P0010 as owner of both the rule and the port surface, and names the reopening artefact. The test also drives a poisoned event and asserts the checkpoint did **not** advance past it. |
| AC-003 | satisfied | `::no_fan_out_runner_catches_a_panic` + `spec-trace` | Exclusion branch with its contingency named — `experiments/polling-cost`, held outside the gate by CF-34. The clause records that the obstacle grew teeth: `Batch` is now an **owned** associated type, so a write set cannot be shared between tasks at all. |
| AC-004 | satisfied | `::reset_refusal_has_a_mechanism_and_no_adapter` + `spec-trace` | Documented exclusion naming HS-P0010. **The planned reason was stale and the grep caught it** — see *Deviations*. |
| AC-005 | satisfied | `::three_ids_are_retained_rather_than_deleted`; every citation re-checked by grep | PS-32, PS-33, PS-35 all `[NON-NORMATIVE`, headings intact. Eighteen citations across `RUNBOOK.md` and `.kb/decisions/0008-…` still resolve. §7.3's opening count, three rows and closing paragraph rewritten; §7.5's headline and PS-33 row closed. |
| AC-006 | satisfied | the census check at `xtask/src/spec_trace.rs:463-511` | 201 IDs, **196** normative, **137**/**47**/**12**/**five**. Written by hand **first**; the checker then reported four disagreements against the old figures, and agreed only after the human recomputed — the direction this check exists to enforce. |
| AC-007 | satisfied | `md5sum` before and after a second `--write`: identical | The generated region was regenerated last and never hand-edited. `spec-trace` without `--write` is green, which *is* the equality assertion. §7.3–§7.6 stay authored. |
| AC-008 | satisfied | `cargo test -p happenstance --features unstable-projection` → `running 5 tests` | Executed, non-zero, feature-gated, and inside `cargo xtask affected --base main` (**affected gate passed**). No adapter conformance rule; `crates/happenstance-testkit/src` untouched. No workspace e2e crate invented. |
| AC-009 | satisfied | CF-38 enforcement (`spec_trace.rs:663`); `spec-trace`, `affected`, `ci --fast` all green | Every new falsifier runs to several lines. No `[FROZEN]` `ES-*` clause body touched. |
| AC-010 | satisfied | the implementation report's *Defect log entries routed to HS-S0032* | Three findings recorded with clause IDs and **none fixed**: CF-36's unimplemented level-marker cross-reference; the redundant, ownerless `†` beside a `[DEFERRED]` marker plus check 4's `PS`-wide short-circuit; and the private-module glob-shadowing hazard carried forward from HS-S0026. `xtask/src` and CF-36's body are unchanged. |

### What a reviewer should look at first

1. **The red transcript in the implementation report.** All five tests failed on their
   *spec* assertion, which means the *tree* assertions before it had already passed. That
   ordering is the evidence that the counts were taken from the tree and the clauses
   written to match, rather than the reverse.
2. **PS-33's clause body.** It is the one verdict that had two admissible forms and could
   only take one. The reason branch one was impossible — no pump function exists at all —
   is stated with the two free functions the contract crate actually publishes.
3. **`.kb/_intake/0032-adr-0031-the-runner-collapses-upward.md`.** Staged, not ingested,
   and it says so in its first three lines. Nothing under `.kb/decisions/` was touched, and
   `redkiln validate --kb` passes.

### Deviations

- **PS-18's planned verdict was factually wrong and the spec told us to re-check.** The
  spec asserts its subject *does not exist*; `reset` (`crates/happenstance-core/src/projection.rs:497`),
  `ResetError::Refused` (`:287`) and a registered `refused_reset_changes_nothing`
  (`crates/happenstance-testkit/src/projection.rs:1360`, `:1909`) all landed with HS-P0010
  after the spec was written. The exclusion holds for a better reason — the mechanism and
  its instrument exist, and what is missing is an **adapter** — and the count is recorded
  as *unavailable* rather than *zero*.
- **`standards/rust/01-standard-of-evidence.md` changed by one line number.** This story's
  insertions pushed the sentence it cites from `:5787` to `:5910`, and
  `cargo xtask lint-constitution` — a REQUIRED gate step — failed on the drift. A
  one-token repair with no change of meaning, the same mechanical consequence two earlier
  commits on this branch already carry.
- **Both PS-27 and PS-30 came back as exclusions**, which is not what `_design.md`'s states
  table hoped for. The signed-off *signature* carries no failure policy and the runner has
  none, so the verdicts were written against the tree. Neither clause is withdrawn.

### Handed forward

- **HS-S0032 (`defect-log-and-macros-verdict`)** receives the three defect entries above,
  each with its clause ID, each unfixed by design.
- **HS-P0010 (`projection-store-freeze`)** is now named in three clause bodies — PS-18,
  PS-27 and PS-30 — as the owner of counts it will be able to take once a projection
  adapter ships.
- **The human** owns `/redkiln:kb-ingest` for the staged ADR. Until that wave runs,
  PS-33's clause points at a file in `.kb/_intake/`, and the test asserts that file is
  present so the citation cannot dangle.
