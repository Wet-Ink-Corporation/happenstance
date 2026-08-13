---
item: HS-S0027
stage: implement
created: 2026-08-12T13:46:24.399Z
updated: 2026-08-12T13:46:24.399Z
---

# Acceptance ledger — PS-33, PS-27, PS-30 settled and PS-18 excluded, on the record

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
  criterion: "GIVEN an evaluator reading `spec/SPECIFICATION.md` §4.9 to decide whether ADR-0007's runner split is a settled design or an open bet, WHEN they reach PS-33 (`:5529`), THEN the clause body states a verdict in one of exactly two admissible forms — an independent caller of the core-side checkpoint pump named by repo path, **or** a superseding ADR collapsing the pump upward staged under `.kb/_intake/` and cited by filename — and in neither form does the reader have to open an ADR to learn the outcome."
  satisfied: false
  evidence: ""
  mount_point: "spec/SPECIFICATION.md — PS-33's clause body in §4.9, held to the tree by the `specification traceability` REQUIRED step (xtask/src/main.rs:315)"
  verifying_test: "cargo xtask spec-trace (xtask/src/main.rs:315), plus redkiln validate --kb and the staged .kb/_intake/ file on the superseding-ADR branch"

- id: AC-002
  criterion: "GIVEN a contributor who needs to know whether *\"the projection runner records a skipped event\"* is a promise or a hope, WHEN they read PS-27 (`:5411`), THEN the clause names the runner path that writes a skip record into the **same** batch that advances the checkpoint past the poisoned position — or records the exclusion with the reason the count is unavailable — and the verdict matches what `crates/happenstance/src/` actually contains at the end of the slice rather than what `_design.md`'s states table hoped for."
  satisfied: false
  evidence: ""
  mount_point: "spec/SPECIFICATION.md — PS-27's clause body in §4.8, plus its rendered §7.2 row inside the generated region (:8507)"
  verifying_test: "crates/happenstance/tests/projection_clauses.rs::skip_and_record_is_atomic under `cargo test -p happenstance --features unstable-projection` (count branch); otherwise cargo xtask spec-trace with the exclusion naming its reopening artefact"

- id: AC-003
  criterion: "GIVEN the same contributor asking whether a panicking projection can corrupt a read model, WHEN they read PS-30 (`:5462`), THEN the clause names the rollback-on-panic caller by path — or records the exclusion **and** names the contingency that governs it, that the fan-out runner is benchmark-gated and the benchmark is `polling-cost-measurement`'s deliverable, deliberately outside the gate (CF-34)."
  satisfied: false
  evidence: ""
  mount_point: "spec/SPECIFICATION.md — PS-30's clause body in §4.8, plus its rendered §7.2 row inside the generated region (:8507)"
  verifying_test: "crates/happenstance/tests/projection_clauses.rs::panicking_apply_rolls_back under `cargo test -p happenstance --features unstable-projection` (count branch); otherwise cargo xtask spec-trace with the exclusion naming the benchmark"

- id: AC-004
  criterion: "GIVEN a reader who has followed a `RUNBOOK.md` citation to PS-18 (`:5200`) expecting to learn whether reset protection is required of an adapter, WHEN they read the clause, THEN they meet a **documented exclusion** that states why the count is unavailable — no projection adapter has shipped, and the clause's subject (`reset`, `ResetError::Refused`) does not exist in `crates/happenstance-core/src/projection.rs` — and names `projection-store-freeze` (HS-P0010) as the owner of the eventual count, so the reader can tell a decision from a deferral."
  satisfied: false
  evidence: ""
  mount_point: "spec/SPECIFICATION.md — PS-18's clause body in §4.6"
  verifying_test: "cargo xtask spec-trace (xtask/src/main.rs:315), with a re-run grep for `reset`/`ResetError` over crates/happenstance-core/src/projection.rs confirming the subject is still absent"

- id: AC-005
  criterion: "GIVEN anyone who has ever cited PS-32, PS-33 or PS-35 — `RUNBOOK.md:4079-4088` has, and so has §7.3 — WHEN those three leave the clause space, THEN each **ID is retained** with a `[NON-NORMATIVE]` marker the way CF-30 already is (`:209-212`), every existing citation still resolves to a row, and the authored §7.3 disposition rows (`:8769-8783`) and PS-33's §7.5 row (`:9033`) — which today say these three are kept only because *\"the work list does not exist yet\"* — are reconciled in the same edit rather than left as prose no parser will catch."
  satisfied: false
  evidence: ""
  mount_point: "spec/SPECIFICATION.md — the three retained clause IDs plus the authored §7.3 (:8769-8783) and §7.5 (:9033) rows"
  verifying_test: "cargo xtask lints && cargo xtask spec-trace (.redkiln/config.yaml:48), plus a grep over RUNBOOK.md, spec/E2E-CASES.md, .kb/** and .bklg/** for PS-32/PS-33/PS-35 showing every hit still resolves"

- id: AC-006
  criterion: "GIVEN the evaluator who trusts §1.3's census as the one count a human computed by reading the document (`:219-222`), WHEN the four verdicts and three retirements land, THEN the census is **re-derived by hand** — total, each maturity count, the \"of which N are normative\" figure, and the document's own subtraction — and the checker agrees with the human rather than the human copying the checker."
  satisfied: false
  evidence: ""
  mount_point: "spec/SPECIFICATION.md §1.3 — the hand-written census at :219-222"
  verifying_test: "cargo xtask spec-trace — the census check at xtask/src/spec_trace.rs:447-509 compares each stated figure against a fresh parse and fails on disagreement"

- id: AC-007
  criterion: "GIVEN a maintainer who regenerates the traceability tables on a later phase, WHEN they run `cargo xtask spec-trace --write` on the committed tree, THEN the §7.1/§7.2 region between the `<!-- BEGIN GENERATED -->` markers (`:8507`) is **byte-identical** to what is committed — this story hand-edited nothing inside the markers — while §7.3–§7.6 stay authored (`:8462-8468`) and carry the reconciled rows from AC-005."
  satisfied: false
  evidence: ""
  mount_point: "spec/SPECIFICATION.md — the generated §7.1/§7.2 region (:8507), regenerated by cargo xtask spec-trace --write"
  verifying_test: "cargo xtask spec-trace without --write (the equality assertion, xtask/src/spec_trace.rs:23-37), then cargo xtask spec-trace --write followed by an empty git status --porcelain"

- id: AC-008
  criterion: "GIVEN a reader who meets a verdict that says *\"this is called\"*, WHEN they look for the evidence, THEN they find an **executed** integration test under `crates/happenstance/tests/`, feature-gated `unstable-projection` per `_design.md:746`, that runs in the story-grain affected gate — not an adapter conformance rule, which CF-36 (`:8297-8298`, `[FROZEN]`) forbids for a clause backed only by integration-level cases, and not a \"workspace e2e crate\", which does not exist and whose creation is a runbook-grain scope change."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/tests/projection_clauses.rs — a new integration-test target for crates/happenstance, gated #![cfg(feature = \"unstable-projection\")], run by cargo xtask affected --base main (.redkiln/config.yaml:40)"
  verifying_test: "cargo test -p happenstance --features unstable-projection over crates/happenstance/tests/projection_clauses.rs (non-zero test count asserted), with crates/happenstance-testkit/src/suite.rs unchanged in the diff"

- id: AC-009
  criterion: "GIVEN the gate itself as the reader of last resort, WHEN this edit lands, THEN no marker is left softened without replacement text — a `[PROVISIONAL]`/`[DEFERRED]` falsifier under 12 characters is a build failure under CF-38 (`:213-217`) — no `[FROZEN]` `ES-*` clause is amended, and `cargo xtask spec-trace` plus the story-grain affected gate are green on the committed tree, so the verdicts are held to the tree by a step that runs whether or not anyone types it."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs:315 — the `specification traceability` REQUIRED step, which runs in cargo xtask ci, ci --fast and the story-grain affected gate"
  verifying_test: "cargo xtask spec-trace (CF-38 enforcement at xtask/src/spec_trace.rs:668-680), cargo xtask affected --base main (.redkiln/config.yaml:40) and cargo xtask ci --fast (.redkiln/config.yaml:55), with the diff touching no ES-* clause body"

- id: AC-010
  criterion: "GIVEN the project exists partly to find out what using a `[FROZEN]` contract reveals (BR-01), WHEN this pass finds that CF-36 names `cargo xtask spec-trace` as *\"cross-referencing each case's level marker\"* while the checker reads no level marker at all, THEN that finding is **recorded with its clause ID and routed** to `defect-log-and-macros-verdict` — along with anything else the pass turns up — and neither `xtask/src/spec_trace.rs` nor the frozen clause is edited to make it go away."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/typed-layer-and-alpha-release/projection-clause-verdicts/ — the defect entry handed to defect-log-and-macros-verdict, which stages it for /redkiln:kb-ingest"
  verifying_test: "Reviewed at the gate against project.md AC-012 and *Out of scope*: the entry names CF-36 by clause ID, and the diff shows xtask/src/spec_trace.rs and CF-36's clause body unchanged"
```
