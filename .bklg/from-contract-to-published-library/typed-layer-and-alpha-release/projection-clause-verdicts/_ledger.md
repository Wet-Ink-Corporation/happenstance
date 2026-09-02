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
  satisfied: true
  evidence: >-
    spec/SPECIFICATION.md PS-33 — marker moved from `[DEFERRED]` to `[NON-NORMATIVE`, and the clause
    body carries the verdict in admissible form **two**: the falsifier fired and the pump collapses
    upward, with the superseding decision staged by filename at
    `.kb/_intake/0032-adr-0031-the-runner-collapses-upward.md`. The count is stated in the clause
    itself — `happenstance-core` publishes exactly two free functions, `collect` (store.rs:285) and
    `read_decision_model` (:321), and neither is a pump — so the reader learns the outcome without
    opening an ADR. Verified: `cargo xtask spec-trace` green (`traceability: no problems found`);
    `redkiln validate --kb` passes; the staged file is present. Held to the tree by
    crates/happenstance/tests/projection_clauses.rs::no_checkpoint_pump_exists_in_the_contract_crate,
    which asserts the free-function set, the absence of `pump` in the port, the marker, and the
    existence of the file the clause cites.
  mount_point: "spec/SPECIFICATION.md — PS-33's clause body in §4.9, held to the tree by the `specification traceability` REQUIRED step (xtask/src/main.rs:315)"
  verifying_test: "cargo xtask spec-trace (xtask/src/main.rs:315), plus redkiln validate --kb and the staged .kb/_intake/ file on the superseding-ADR branch"

- id: AC-002
  criterion: "GIVEN a contributor who needs to know whether *\"the projection runner records a skipped event\"* is a promise or a hope, WHEN they read PS-27 (`:5411`), THEN the clause names the runner path that writes a skip record into the **same** batch that advances the checkpoint past the poisoned position — or records the exclusion with the reason the count is unavailable — and the verdict matches what `crates/happenstance/src/` actually contains at the end of the slice rather than what `_design.md`'s states table hoped for."
  satisfied: true
  evidence: >-
    spec/SPECIFICATION.md PS-27 — the **exclusion branch**, because the tree decided it:
    `happenstance::run_projection` (crates/happenstance/src/runner.rs:401) offers no `on_error`, no
    `SkipPolicy` and no policy argument of any kind, so there is no path that could write a skip
    record. The clause now says so, states that this is its own *Rejects* read one layer up rather
    than an omission, names `projection-store-freeze` (HS-P0010) as the owner of both the suite rule
    and the port surface, and names the artefact that reopens it (a per-projection failure policy on
    `Projection`). Verified:
    crates/happenstance/tests/projection_clauses.rs::no_skip_and_record_path_is_offered — asserts
    the three absences over comment-stripped source AND drives a poisoned event through the real
    runner, asserting the checkpoint did **not** advance past it and the following event was never
    applied. `cargo test -p happenstance --features unstable-projection` runs 5 tests in that file,
    non-zero. `cargo xtask spec-trace` green.
  mount_point: "spec/SPECIFICATION.md — PS-27's clause body in §4.8, plus its rendered §7.2 row inside the generated region (:8507)"
  verifying_test: "crates/happenstance/tests/projection_clauses.rs::skip_and_record_is_atomic under `cargo test -p happenstance --features unstable-projection` (count branch); otherwise cargo xtask spec-trace with the exclusion naming its reopening artefact"

- id: AC-003
  criterion: "GIVEN the same contributor asking whether a panicking projection can corrupt a read model, WHEN they read PS-30 (`:5462`), THEN the clause names the rollback-on-panic caller by path — or records the exclusion **and** names the contingency that governs it, that the fan-out runner is benchmark-gated and the benchmark is `polling-cost-measurement`'s deliverable, deliberately outside the gate (CF-34)."
  satisfied: true
  evidence: >-
    spec/SPECIFICATION.md PS-30 — the **exclusion branch**, with its contingency named. The fan-out
    runner is not built, and the clause now records why the obstacle is stronger than the clause
    anticipated: the port's `Batch` became a plain **owned** associated type
    (crates/happenstance-core/src/projection.rs:436), so a write set cannot be shared between tasks
    at all and a `tokio::spawn` would additionally demand `Send`. The contingency is named as
    `experiments/polling-cost`, the slice-mate's deliverable, held outside the gate by CF-34.
    Verified: crates/happenstance/tests/projection_clauses.rs::no_fan_out_runner_catches_a_panic —
    asserts no `catch_unwind`, `AssertUnwindSafe` or `spawn(` in the typed layer's comment-stripped
    source, AND that a panicking `apply` really does escape `run_projection` (caught by the test)
    leaving the store empty and the checkpoint at `NeverRun`. `cargo xtask spec-trace` green.
  mount_point: "spec/SPECIFICATION.md — PS-30's clause body in §4.8, plus its rendered §7.2 row inside the generated region (:8507)"
  verifying_test: "crates/happenstance/tests/projection_clauses.rs::panicking_apply_rolls_back under `cargo test -p happenstance --features unstable-projection` (count branch); otherwise cargo xtask spec-trace with the exclusion naming the benchmark"

- id: AC-004
  criterion: "GIVEN a reader who has followed a `RUNBOOK.md` citation to PS-18 (`:5200`) expecting to learn whether reset protection is required of an adapter, WHEN they read the clause, THEN they meet a **documented exclusion** that states why the count is unavailable — no projection adapter has shipped, and the clause's subject (`reset`, `ResetError::Refused`) does not exist in `crates/happenstance-core/src/projection.rs` — and names `projection-store-freeze` (HS-P0010) as the owner of the eventual count, so the reader can tell a decision from a deferral."
  satisfied: true
  evidence: >-
    spec/SPECIFICATION.md PS-18 — a documented exclusion, `[DEFERRED`, naming
    `projection-store-freeze` (HS-P0010) as the owner of the eventual count. **The grep was re-run
    rather than assumed, and it changed the verdict's reason:** the clause's subject now EXISTS —
    `reset` at crates/happenstance-core/src/projection.rs:497 and `ResetError::Refused` at :287 —
    and `refused_reset_changes_nothing` is a registered projection-suite rule
    (crates/happenstance-testkit/src/projection.rs:1360, :1909). What is absent is an adapter: no
    projection adapter has shipped, and the one fixture in the tree declines `RESET_REFUSAL` with
    the store's own reason (crates/happenstance-testkit/src/fixtures.rs:446). The clause states all
    of that, so the count is unavailable rather than zero. Verified:
    crates/happenstance/tests/projection_clauses.rs::reset_refusal_has_a_mechanism_and_no_adapter;
    `cargo xtask spec-trace` green.
  mount_point: "spec/SPECIFICATION.md — PS-18's clause body in §4.6"
  verifying_test: "cargo xtask spec-trace (xtask/src/main.rs:315), with a re-run grep for `reset`/`ResetError` over crates/happenstance-core/src/projection.rs confirming the subject is still absent"

- id: AC-005
  criterion: "GIVEN anyone who has ever cited PS-32, PS-33 or PS-35 — `RUNBOOK.md:4079-4088` has, and so has §7.3 — WHEN those three leave the clause space, THEN each **ID is retained** with a `[NON-NORMATIVE]` marker the way CF-30 already is (`:209-212`), every existing citation still resolves to a row, and the authored §7.3 disposition rows (`:8769-8783`) and PS-33's §7.5 row (`:9033`) — which today say these three are kept only because *\"the work list does not exist yet\"* — are reconciled in the same edit rather than left as prose no parser will catch."
  satisfied: true
  evidence: >-
    spec/SPECIFICATION.md — PS-32, PS-33 and PS-35 each carry `[NON-NORMATIVE` with a stated reason,
    their bold `**PS-nn —` headings intact so every citation resolves, exactly as CF-30 does. Every
    existing citation was re-checked and still lands: RUNBOOK.md:112, :409, :414, :438, :473, :552,
    :553, :1106, :1263, :1299, :1306, :3930, :3983, :4065, :4082, :4087-4088, and
    .kb/decisions/0008-one-derivation-for-both-ports.md:14, :45. The authored §7.3 disposition rows
    were rewritten from *should move* to *moved, and here is what happened*, its opening count
    corrected from eleven-plus-one to eight-plus-four, and its closing paragraph rewritten to say
    the work list now exists; §7.5's PS-33 row is closed with its reason and its headline corrected
    from *two more* to *one more*. Verified:
    crates/happenstance/tests/projection_clauses.rs::three_ids_are_retained_rather_than_deleted;
    `cargo xtask spec-trace` green; `cargo xtask lint-constitution` green.
  mount_point: "spec/SPECIFICATION.md — the three retained clause IDs plus the authored §7.3 (:8769-8783) and §7.5 (:9033) rows"
  verifying_test: "cargo xtask lints && cargo xtask spec-trace (.redkiln/config.yaml:48), plus a grep over RUNBOOK.md, spec/E2E-CASES.md, .kb/** and .bklg/** for PS-32/PS-33/PS-35 showing every hit still resolves"

- id: AC-006
  criterion: "GIVEN the evaluator who trusts §1.3's census as the one count a human computed by reading the document (`:219-222`), WHEN the four verdicts and three retirements land, THEN the census is **re-derived by hand** — total, each maturity count, the \"of which N are normative\" figure, and the document's own subtraction — and the checker agrees with the human rather than the human copying the checker."
  satisfied: true
  evidence: >-
    spec/SPECIFICATION.md §1.3 — the census re-derived by hand before the generator was run, and the
    checker asked to disagree: 201 clause IDs of which **196** are normative, **137** `[FROZEN]`,
    **47** `[PROVISIONAL]`, **12** `[DEFERRED]`, **five** `[NON-NORMATIVE]`, with the parenthetical
    naming all five. The document's own subtraction holds: 201 − 5 = 196. The hand count was written
    first and `cargo xtask spec-trace` then reported four disagreements (139/50/10/2 against
    137/47/12/5) — the checker agreeing with the human only after the human recomputed, which is the
    direction this check exists to enforce. Verified: the census check at
    xtask/src/spec_trace.rs:463-511 passes; `cargo xtask spec-trace` green.
  mount_point: "spec/SPECIFICATION.md §1.3 — the hand-written census at :219-222"
  verifying_test: "cargo xtask spec-trace — the census check at xtask/src/spec_trace.rs:447-509 compares each stated figure against a fresh parse and fails on disagreement"

- id: AC-007
  criterion: "GIVEN a maintainer who regenerates the traceability tables on a later phase, WHEN they run `cargo xtask spec-trace --write` on the committed tree, THEN the §7.1/§7.2 region between the `<!-- BEGIN GENERATED -->` markers (`:8507`) is **byte-identical** to what is committed — this story hand-edited nothing inside the markers — while §7.3–§7.6 stay authored (`:8462-8468`) and carry the reconciled rows from AC-005."
  satisfied: true
  evidence: >-
    The generated §7.1/§7.2 region was regenerated with `cargo xtask spec-trace --write` and never
    hand-edited: every clause change was made in §4's clause bodies and §1.3, and the write step was
    run last. Idempotency proved by hash rather than by eye — `md5sum spec/SPECIFICATION.md` before
    and after a second `--write` returned the identical `a1d10293441988df566fae021bd87512`, so the
    committed region is byte-identical to a fresh computation. `cargo xtask spec-trace` without
    `--write` is green, which IS the equality assertion (xtask/src/spec_trace.rs:23-37). §7.3-§7.6
    stay authored and carry AC-005's reconciled rows.
  mount_point: "spec/SPECIFICATION.md — the generated §7.1/§7.2 region (:8507), regenerated by cargo xtask spec-trace --write"
  verifying_test: "cargo xtask spec-trace without --write (the equality assertion, xtask/src/spec_trace.rs:23-37), then cargo xtask spec-trace --write followed by an empty git status --porcelain"

- id: AC-008
  criterion: "GIVEN a reader who meets a verdict that says *\"this is called\"*, WHEN they look for the evidence, THEN they find an **executed** integration test under `crates/happenstance/tests/`, feature-gated `unstable-projection` per `_design.md:746`, that runs in the story-grain affected gate — not an adapter conformance rule, which CF-36 (`:8297-8298`, `[FROZEN]`) forbids for a clause backed only by integration-level cases, and not a \"workspace e2e crate\", which does not exist and whose creation is a runbook-grain scope change."
  satisfied: true
  evidence: >-
    crates/happenstance/tests/projection_clauses.rs — **five executed integration tests**,
    feature-gated on `unstable-projection` (plus `memory` and `json`, which the fixtures need),
    under `cargo test -p happenstance --features unstable-projection`: `running 5 tests / test
    result: ok. 5 passed`, non-zero and not `running 0 tests`. They run in the story-grain affected
    gate: `cargo xtask affected --base main` reports **affected gate passed**. No adapter
    conformance rule was added — CF-36 forbids it for a clause backed only by integration-level
    cases — and `git status --porcelain -- crates/happenstance-testkit/src` is empty. No workspace
    e2e crate was created; the root `Cargo.toml` `members` list is untouched.
  mount_point: "crates/happenstance/tests/projection_clauses.rs — a new integration-test target for crates/happenstance, gated #![cfg(feature = \"unstable-projection\")], run by cargo xtask affected --base main (.redkiln/config.yaml:40)"
  verifying_test: "cargo test -p happenstance --features unstable-projection over crates/happenstance/tests/projection_clauses.rs (non-zero test count asserted), with crates/happenstance-testkit/src/suite.rs unchanged in the diff"

- id: AC-009
  criterion: "GIVEN the gate itself as the reader of last resort, WHEN this edit lands, THEN no marker is left softened without replacement text — a `[PROVISIONAL]`/`[DEFERRED]` falsifier under 12 characters is a build failure under CF-38 (`:213-217`) — no `[FROZEN]` `ES-*` clause is amended, and `cargo xtask spec-trace` plus the story-grain affected gate are green on the committed tree, so the verdicts are held to the tree by a step that runs whether or not anyone types it."
  satisfied: true
  evidence: >-
    No marker was softened without replacement text: every new `[DEFERRED —` falsifier runs to
    several lines, far over CF-38's 12-character floor, and the CF-38 enforcement at
    xtask/src/spec_trace.rs:663-673 passes. No `[FROZEN]` `ES-*` clause body was touched — `git diff
    main -- spec/SPECIFICATION.md` shows changes only in §1.3, PS-18, PS-27, PS-30, PS-32, PS-33,
    PS-35, §7.1/§7.2 (generated), §7.3 and §7.5. Green on the committed tree: `cargo xtask
    spec-trace` (traceability: no problems found), `cargo xtask affected --base main` (affected gate
    passed), and `cargo xtask ci --fast` (all required checks passed).
  mount_point: "xtask/src/main.rs:315 — the `specification traceability` REQUIRED step, which runs in cargo xtask ci, ci --fast and the story-grain affected gate"
  verifying_test: "cargo xtask spec-trace (CF-38 enforcement at xtask/src/spec_trace.rs:668-680), cargo xtask affected --base main (.redkiln/config.yaml:40) and cargo xtask ci --fast (.redkiln/config.yaml:55), with the diff touching no ES-* clause body"

- id: AC-010
  criterion: "GIVEN the project exists partly to find out what using a `[FROZEN]` contract reveals (BR-01), WHEN this pass finds that CF-36 names `cargo xtask spec-trace` as *\"cross-referencing each case's level marker\"* while the checker reads no level marker at all, THEN that finding is **recorded with its clause ID and routed** to `defect-log-and-macros-verdict` — along with anything else the pass turns up — and neither `xtask/src/spec_trace.rs` nor the frozen clause is edited to make it go away."
  satisfied: true
  evidence: >-
    Three findings recorded in this story's implementation report under *Defect log entries routed
    to HS-S0032*, each with its clause ID and none of them fixed: **CF-36** — its `Rule:` line names
    `cargo xtask spec-trace` *"cross-referencing each case's level marker"*, and `grep -c Level
    xtask/src/spec_trace.rs` returns **0**, so a `[FROZEN]` clause cites a check that is not
    implemented; **PS-27/PS-30's `†`** — the generated §7.2 renders *must be written* against rules
    the clause bodies now say deliberately do not exist, so the dagger and the marker state the same
    fact twice and the dagger names no owner; and **the hidden-glob module hazard** carried forward
    from HS-S0026. `git status --porcelain -- xtask/src` is empty and CF-36's clause body at
    spec/SPECIFICATION.md:8611-8622 is unchanged in the diff.
  mount_point: ".bklg/from-contract-to-published-library/typed-layer-and-alpha-release/projection-clause-verdicts/ — the defect entry handed to defect-log-and-macros-verdict, which stages it for /redkiln:kb-ingest"
  verifying_test: "Reviewed at the gate against project.md AC-012 and *Out of scope*: the entry names CF-36 by clause ID, and the diff shows xtask/src/spec_trace.rs and CF-36's clause body unchanged"
```
