---
item: HS-S0082
stage: implement
created: "2026-08-12"
updated: "2026-08-12"
---

# Acceptance ledger — The freeze verdict, dated and either way

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
  criterion: >-
    **GIVEN** the projection suite has been run against `LadybugFixture` and the result is known, **WHEN** the evaluator opens `references/evaluation/` at the tip of the initiative branch, **THEN** `phase-11-freeze-verdict.md` is present with all seven fixed headings from *Behavior and interfaces* — `## Verdict`, `## Rules run`, `## Clause ledger`, `## Findings the suite cannot emit`, `## What this was checked against`, `## If it did not hold` — and `## If it did not hold` is **answered** on a green run ("not triggered; the routing that would have applied is …") rather than deleted or left empty, **SO THAT** the artefact's existence and shape carry no information about which way the run went, which is the whole of its evidentiary value.
  satisfied: false
  evidence: ""
  mount_point: "references/evaluation/README.md"
  verifying_test: "Static/process, reviewed not scripted (_decomposition.md:477): all seven headings present in references/evaluation/phase-11-freeze-verdict.md and `## If it did not hold` non-empty, in the reviewed PR diff; `git log --follow` shows the shape predates the first conformance run"

- id: AC-002
  criterion: >-
    **GIVEN** the evaluator will not clone the repository, **WHEN** they read the verdict's front matter and its `## Verdict` line, **THEN** they find all five DR-4 fields — which implementation (`LadybugProjectionStore`, crate version, `lbug` version), which rules (the `## Rules run` table), which PS clause ids (the `## Clause ledger`), at which commit (a **full SHA**, never a branch name or an abbreviation), on which date — plus the axes document's own SHA and the CI shape the run happened under, with the cold-build **number cited to `RUNBOOK.md`'s phase 11 body rather than copied**; and the `## Verdict` heading is followed by exactly one line reading exactly `held` or exactly `did not hold`, **SO THAT** the verdict can be judged, and its ordering against the axes seen, without running `git log` or trusting a paraphrase.
  satisfied: false
  evidence: ""
  mount_point: "references/evaluation/README.md"
  verifying_test: "Static: the five DR-4 fields (project.md:149-151) greppable by name in references/evaluation/phase-11-freeze-verdict.md; `git cat-file -e <run-sha>` and `git cat-file -e <axes-sha>` both resolve; `## Verdict` body matches one of two literals (RUNBOOK.md:4435-4436)"

- id: AC-003
  criterion: >-
    **GIVEN** the adapter author knows a freeze fails quietly — as a rule reported as a fixture limitation, not as a red test — **WHEN** they read `## Rules run`, **THEN** the table carries **one body row per rule `projection-store-freeze` registered**, with its row count reconciled against that project's registration list at the run commit rather than counted by eye, and each row's outcome drawn from a closed vocabulary of three — `passed`, `declined — <the fixture's stated reason, quoted from the `--show-output` `SKIP` line>`, `failed` — with a `declined` row carrying a paraphrased, generic or empty reason failing the criterion, **SO THAT** a rule that declined is impossible to confuse with a rule that passed and impossible to omit.
  satisfied: false
  evidence: ""
  mount_point: "references/evaluation/README.md"
  verifying_test: "Reconciliation (_decomposition.md:474): `## Rules run` body-row count diffed against projection-store-freeze's registration list at the run commit; each `declined` reason matched verbatim against the captured `--show-output` transcript (xtask/src/main.rs:131-151; target held to the gate by xtask/src/proof.rs:133)"

- id: AC-004
  criterion: >-
    **GIVEN** the reviewer must tell a data point from a decision, **WHEN** they read `## Clause ledger` and `## What this was checked against`, **THEN** nine clause ids appear — PS-2 as the bar, plus PS-4, PS-5, PS-6, PS-9, PS-11, PS-12, PS-15 and PS-34 — each with what the run showed **and its owner beside it** (`projection-store-freeze` for all nine), PS-3 appears as a tenth row explicitly labelled *data point* owned by `publication-and-positioning`, and PS-2's requirement is **quoted** — `CheckpointOnlyStore` failed, plus two adapters at opposite ends of the batch-shape axis passed — and answered by naming, against the axes document, **which two adapters and on which axis they are unlike**, **SO THAT** "the freeze held" is falsifiable against the clause that set the bar rather than against a green run.
  satisfied: false
  evidence: ""
  mount_point: "references/evaluation/README.md"
  verifying_test: "Static: all ten ids present with an owner column; PS-2's quoted text matched against spec/SPECIFICATION.md:4760-4774; owners checked against RUNBOOK.md:598, :601, :602; the unlike-axis claim resolves to a named axis in references/evaluation/phase-11-preflight-and-unlike-axes.md"

- id: AC-005
  criterion: >-
    **GIVEN** a green suite is structurally incapable of emitting a finding about the port, **WHEN** the adapter author reads `## Findings the suite cannot emit`, **THEN** T1 is stated — the freeze **deleted the counter-example that was evidence for freezing**: `LiveHandleProjectionStore` binds a genuinely borrowed handle with no lifetime-free spelling and cannot exist under a frozen `type Batch;`, and its borrowed GAT had compiled and **not** failed the `Send` flavour across a real `tokio::spawn`, a result that cuts against §4.2 — together with the fact that both transcripts survive outside the crate and that the escalation already went upstream at HS-S0074 rather than being re-raised here; and the GAT/ICE finding is recorded on whichever branch merged (survived: the port is implementable only by stores outliving every batch lifetime and the failure mode is a **compiler panic, not a diagnostic**; retired: recorded as *observed*, noting that binding an owned type to the GAT bought none of that relief), **SO THAT** the freeze's own cost is on the record where nothing else would put it.
  satisfied: false
  evidence: ""
  mount_point: "references/evaluation/README.md"
  verifying_test: "Review against the surviving transcripts (reviewed, not scripted — _decomposition.md:477): references/adapter-shapes.md:169-195 and :186-191, crates/happenstance-ladybug/src/live_handle.rs:38-66; AC-A04's bar at _decomposition.md:51-54 with T1 at :314-331 and T2 at :332-359"

- id: AC-006
  criterion: >-
    **GIVEN** the reviewer will treat any unanswered question as an answer of "we did not look", **WHEN** they read the remaining `## Findings` rows, **THEN** four questions are **answered rather than reported**: `WriteTransactionInUse` under a concurrency-shaped rule is either a **declared capability limit** (the suite is right; this adapter's concurrency envelope is narrower than a SQL adapter's) or a **rule defect** routed to HS-P0010 by name — never "retry until green", and if no concurrency-shaped rule met it, that absence is itself stated as a finding about the suite's reach; read-your-own-writes is written in **PS-12's own vocabulary** (reads through an open `Batch` reflect that batch's pending writes, *or* no read path is exposed on `Batch` at all) and never as "a read path answering from committed state"; ADR-0025 Q3's blocking bridge is reported as having let every conformance emitter (tokio, blocking, wasm) run the adapter as built, or as having narrowed the harness; and E2E-19's and E2E-24's third-shape halves carry a stated status — writable now, or not, with the reason, **SO THAT** the next project inherits answers instead of discovering the questions late.
  satisfied: false
  evidence: ""
  mount_point: "references/evaluation/README.md"
  verifying_test: "Review (reviewed, not scripted — _decomposition.md:477) cross-checked against crates/happenstance-ladybug/src/projection_store.rs:204-213 and :49-64, spec/SPECIFICATION.md:5052-5075 (PS-12's two permitted shapes and its forbidden one), RUNBOOK.md:4440 and spec/E2E-CASES.md; the excluded answer is fixed in advance by project.md:278-283"

- id: AC-007
  criterion: >-
    **GIVEN** an unclassified markdown file in `references/evaluation/` is exactly the file a later contributor edits in place, and **GIVEN** the repository reads how far it has got from `RUNBOOK.md`, **WHEN** this PR merges, **THEN** `references/evaluation/README.md` classifies `phase-11-freeze-verdict.md` **by name** under the immutable-evidence lifecycle with one line saying what it is — making explicit that a correction is a **new dated, commit-pinned document naming the one it supersedes**, never an in-place amendment — and `RUNBOOK.md:4432-4438`'s phase 11 exit-criteria boxes are ticked **in place**, each against the merged story and commit that discharged it, with any box that cannot be ticked left unticked and its reason written beside it, **SO THAT** the document is evidence rather than a file, and phase 11 is closable by someone who did not do the work.
  satisfied: false
  evidence: ""
  mount_point: "references/evaluation/README.md (classification, :1-13 and :77-85) and RUNBOOK.md:4432-4438 (phase 11 exit criteria, ticked in place per _decomposition.md:82)"
  verifying_test: "Static: the filename appears in references/evaluation/README.md under the lifecycle at :1-13, :77-85; the RUNBOOK.md:4432-4438 block's boxes ticked or reasoned in the reviewed diff, with every ticked box's commit an ancestor of HEAD (project.md Definition of done, items 4 and 6)"

- id: AC-008
  criterion: >-
    **GIVEN** the response to bad news must be a new atom and never an edit, **WHEN** the reviewer diffs this PR and, on a *did not hold* verdict, follows the routing, **THEN** no path under `spec/` appears in the diff at all, `cargo xtask spec-trace` is green, and `git diff <project-start-commit>..HEAD -- spec/SPECIFICATION.md` shows no `[FROZEN]` clause marker or sentence differing from its state at the project's start; and a *did not hold* stages exactly **one note under `.kb/_intake/`** carrying what did not hold, which clause, which alternative the superseding ADR must weigh, and the meta-finding that the two-implementation freeze rule should have been three — with **nothing written under `.kb/decisions/` by hand**, no accepted atom edited, no clause marker moved and the re-plan left as backlog work, **SO THAT** a freeze that did not hold is heard before publication opens without the record it disagrees with being quietly rewritten.
  satisfied: false
  evidence: ""
  mount_point: "references/evaluation/README.md (the verdict's classification) and .kb/_intake/ (the did-not-hold routing note, per _decomposition.md:204-209)"
  verifying_test: "Gate: `cargo xtask spec-trace` (xtask/src/spec_trace.rs:291-372, inside `cargo xtask ci`); static: `git diff --name-only` on this PR contains no `spec/` and no `.kb/decisions/` path; reviewed `git diff <project-start-commit>..HEAD -- spec/SPECIFICATION.md` (_decomposition.md:478); `redkiln validate --kb && redkiln doctor` clean with the intake note staged"
```
