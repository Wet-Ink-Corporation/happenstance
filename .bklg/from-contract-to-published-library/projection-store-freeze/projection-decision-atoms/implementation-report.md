---
item: "HS-S0002"
stage: implement
created: "2026-08-13"
updated: "2026-08-13"
---

# Implementation Report — ADR-0017, ADR-0018 and ADR-0019 accepted before the port changes

> **STATUS: BLOCKED, and partially delivered.** Eight of the ten acceptance
> criteria require three **accepted `.kb/decisions/` atoms**. This repository
> reserves atom authorship to `/redkiln:kb-ingest`, a **human-invoked** wave —
> `CLAUDE.md` (*Where the work lives*) and this story's own AC-007 both say so,
> and this spec's *Implementation notes* instruct explicitly: *"Do not run
> `/redkiln:kb-ingest` from inside the implementation. It is user-invoked; plan it
> as a handoff at the wave gate."* Hand-writing the atoms would violate AC-007 in
> the act of appearing to satisfy AC-001 – AC-003, and is exactly the move that
> was reverted at `0269720`.
>
> What is delivered is everything the wave needs and everything this story owns:
> **three long-form records** under `references/adr/` and **three staged intake
> documents** under `.kb/_intake/`. Two ACs are satisfied by those records alone
> and are flipped with cited evidence; **eight are not, and remain
> `satisfied: false`.** The exact missing dependency is named under **Notes**.

## TDD Evidence

Nothing executable changes, so no `#[test]` can observe this story — the Testing
brief types project AC-008 as **Static**, *"a process gate on commit sequence as
much as a schema check"*, and this story's `_ledger.md` says in its own preamble
that inventing a test file to fill the column would be the decorative rule
`CLAUDE.md` forbids. The instrument is therefore a static check set over the
artefacts, and it was written first and run against the **baseline commit**
(story 1's checkpoint) before any record existed.

Red baseline: `adr-checks.sh HEAD` — nineteen failures, every one asserting a
missing artefact or a missing claim inside one, none a typo or a parse error.
Green: `adr-checks.sh` against the working tree — 35/35 pass.

| AC | Test | Red → Green |
| -- | ---- | ----------- |
| AC-004 | each record carries ≥1 `references/adapter-shapes.md:<range>` citation; ADR-0018 and ADR-0019 additionally quote the stated absence `Every body is todo!()` | **red** `FAIL …0017 exists`, `FAIL …0018 exists`, `FAIL …0019 exists`, `FAIL AC-004 …0018 quotes the stated absence`, `FAIL AC-004 …0019 quotes the stated absence` → **green** 6 / 2 / 2 citations respectively, both stated-absence quotes present. **Satisfied.** |
| AC-006 | each record carries a strength table, a falsifier **and** a phase on every provisional half, and quotes the sweep in its scope section | **red** `FAIL AC-006 … names falsifier + phase` ×3, `FAIL AC-006 … consumes the sweep` ×3 → **green** all six pass. **Satisfied.** |
| AC-001 | the *record* half only — `E0195`, `DefId::expect_local`, `live_handle.rs`, `LiveHandleProjectionStore`, `ForeignBatch`, `ProjectionProbe` all named in ADR-0017 | **red** six `FAIL`s → **green** six `PASS`es. **AC still `false`:** its criterion is about `.kb/decisions/0017-*.md`, which the wave has not produced. |
| AC-002 | the *record* half only — ADR-0018 cites `0007-projection-runner-decodes` for PS-17, names the typed-layer-only alternative, names `(None, true)`, and defers PS-19 to `unstable-projection-gate-and-clause-disposition` | **red** four `FAIL`s → **green** four `PASS`es. **AC still `false`:** atom absent. |
| AC-003 | the *record* half only — ADR-0019 names `HS-P0011`, `PumpError`, `SkipAndRecord`, `AssertUnwindSafe`, and designs no runner error type | **red** four `FAIL`s → **green** four `PASS`es. **AC still `false`:** atom absent. |
| AC-005 | every record carries a non-empty `## Alternatives rejected` section | **red** three `FAIL`s → **green** three `PASS`es. **AC still `false`:** its criterion is about the three *atoms*. |
| AC-007 | the wave ran, `.kb/_intake/` is clear of this wave's documents, a non-colliding directory exists under `.kb/_governance/integration-waves/` | **red** and **still red**. No wave has run. **Blocked.** |
| AC-008 | `git diff --diff-filter=D -- .kb/open-questions/` empty; the answered question's `status` flipped with `related` edges and its body verbatim | first half **green** (`PASS AC-008 no open question deleted`); second half **blocked** — the flip is a wave op. The six-question owner table is reproduced below, and all six files are untouched. |
| AC-009 | three rows in `.kb/maps/decision-map.md`; the resolved question annotated in `.kb/maps/open-questions-index.md` | **blocked** — map rows are written by the wave's Maps phase. The rows are specified in the intake documents. |
| AC-010 | `git diff --stat main...HEAD -- crates/ spec/ RUNBOOK.md` empty | **green** all three (`PASS AC-010 no diff under crates/`, `spec/`, `RUNBOOK.md`). **AC still `false`:** the criterion is *"the three atoms are committed on a tree where `projection.rs` is byte-identical"*, and the atoms are not committed. The ordering guarantee it protects is **intact** — see Notes. |

Check script: `scratchpad/adr-checks.sh`, run as `adr-checks.sh <rev>` for the red
baseline and with no argument for green. Kept out of the repository: this story's
PR boundary is `references/adr/**`, `.kb/**` and its own backlog folder, and a
shell script is none of them.

## Commits

One checkpoint commit, no fixups. The sha cannot be written into a file the commit
contains; it is recorded on the item through `redkiln record-links` and returned
in the slice digest.

| SHA | Subject |
| --- | ------- |
| *on the item's `links.commits`* | `feat(projection-store-freeze): The three projection decision atoms` |

Its parent is story 1's checkpoint,
`feat(projection-store-freeze): Sweep the PS clause range for the pairing defect`,
which is what makes the sweep's finding an **input** to these records rather than
a claim about them.

## Changes

| Path | Shape of the change |
| --- | --- |
| `references/adr/0017-what-a-projection-batch-owns.md` | **New.** The long-form record for PS-4 – PS-15: the runbook's question, a strength table over its three halves, four quoted transcripts, the split-by-consumer decision (`type Batch;` + no write vocabulary + `ProjectionProbe` behind `conformance`), the drop/rollback decision, `LiveHandleProjectionStore`'s disposition, consequences, and seven rejected alternatives with reasons. |
| `references/adr/0018-returning-a-projection-to-never-run.md` | **New.** PS-16 – PS-20: four decisions across three strengths, the stated-absence transcript, the night-desk incident the atomicity decision models, PS-17 cited to ADR-0007 rather than re-derived, refusal as mechanism-with-policy-elsewhere, the three-variant `Checkpoint`, PS-19's defect named and deferred, six rejected alternatives. |
| `references/adr/0019-what-happens-when-apply-fails.md` | **New.** PS-26 – PS-30: *Out of scope* written first, the three deferrals to HS-P0011 by name in a table, the decision that the port grows nothing, the skip primitive that already exists, why `rollback` survives, PS-29's defect named and deferred, PS-28 recorded `undetermined`, six rejected alternatives. |
| `.kb/_intake/2026-08-13-adr-0017-projection-batch.md` | **New, staged only.** The wave's input for ADR-0017's atom: frontmatter instructions (invent no keys; `status: accepted` with the qualification **and its falsifier** in `summary`'s first clause), the claims, the rejected alternatives, the provisional halves with falsifier + phase, the **resolve-not-delete** protocol for `kb-open-question-projection-batch-no-apply-001`, the map rows, and an explicit *Not proposed* list. Carries the wave-id warning. |
| `.kb/_intake/2026-08-13-adr-0018-reset.md` | **New, staged only.** Same shape for ADR-0018. Resolves no open question. |
| `.kb/_intake/2026-08-13-adr-0019-apply-failure.md` | **New, staged only.** Same shape for ADR-0019. Resolves no open question. |
| `.bklg/.../projection-decision-atoms/**` | `_ledger.md` (**two** rows flipped `false` → `true` with evidence; eight left `false`, no criterion touched), this report, and `report.md`. |

Not touched, and asserted by the gate: `crates/**`, `spec/SPECIFICATION.md`,
`RUNBOOK.md`, and everything under `.kb/` outside `_intake/`.

## Gates

| Command | Result |
| --- | --- |
| `scratchpad/adr-checks.sh HEAD` (red baseline) | 19 `FAIL`, exit 1 |
| `scratchpad/adr-checks.sh` (green) | 35/35 `PASS`, exit 0 |
| `redkiln validate --kb` | `validate passed` |
| `redkiln validate` | `validate passed` |
| `redkiln doctor` | exactly the six expected `template-drift` advisories, plus the nine pre-existing foundation-story-not-consumed advisories the base branch already emits; unchanged |
| `cargo fmt --all --check` | exit 0 |
| `cargo xtask affected --base main` | *no package affected — nothing to compile*; **affected gate passed** |
| `cargo xtask ci` | **all checks passed** (run once for the slice, over a Rust tree none of these three stories touches) |

`cargo xtask ci` is recorded as a **non-regression check only**. It compiles a
tree this story does not touch; listing it as evidence for anything this story
claims would be a false positive by construction.

## Notes

### The blocking dependency, exactly

**Missing:** one **human-invoked `/redkiln:kb-ingest` wave**, fed from the three
`.kb/_intake/2026-08-13-adr-00{17,18,19}-*.md` documents this commit stages.

Nothing else is missing. The wave's inputs are complete, its adjudication is
pre-stated in each intake document (frontmatter conventions, claims, rejected
alternatives, provisional halves with falsifiers and phases, map rows, and an
explicit list of what is *not* proposed), and the wave-id collision hazard is
named — `.kb/_governance/integration-waves/` already holds `2026-08-10-intake`
and `2026-08-10-intake-2`, and `2026-08-13-projection-adrs` is free.

**Why it was not run here, rather than "not attempted".** Three independent
reasons, any one sufficient:

1. **AC-007 requires it to be a wave.** The criterion is not *"three atoms exist"*
   — it is that they *"arrive as the output of one human-invoked
   `/redkiln:kb-ingest` wave fed from `.kb/_intake/`"*. An implementation that
   hand-wrote the atoms would fail AC-007 in the act of appearing to satisfy
   AC-001 – AC-003. That is gaming, and it is the specific gaming this repository
   has already caught once: hand-writing atoms *"produces the directory layout of
   the process without the process"*, reverted at `0269720`.
2. **This spec instructs it.** *Implementation notes*: *"Do not run
   `/redkiln:kb-ingest` from inside the implementation. It is user-invoked; plan
   it as a handoff at the wave gate."*
3. **The wave carries a human approval gate** — the point at which
   `.kb/_intake/README.md` is dropped rather than ingested as an atom (EC-003),
   and at which the wave id is checked for collision (EC-002). An implementation
   agent running it would consume that gate rather than reach it.

### What is *not* at risk while this is blocked

**The ordering guarantee AC-010 exists to protect is intact.** Its whole point is
that a decision accepted *after* the port changed proves nothing about which one
was allowed to constrain the other. On this branch `crates/happenstance-core/src/projection.rs`
is byte-identical to `main`, and `owned-batch-port-shape` (HS-S0003) is blocked on
this story, so the port cannot move first. The wave running later still lands the
atoms before any port change; what is lost is only the ability to *cite* a
commit at which `validate --kb` was green over three accepted atoms.

### The six open questions this story deliberately does not answer

Reproduced here because the spec requires it and because an unexplained omission
is indistinguishable from an oversight. **All six files are untouched**, and
`git diff --diff-filter=D -- .kb/open-questions/` is empty.

| Atom | Why not here | Owner |
| --- | --- | --- |
| `kb-open-question-ps-1-no-progress-obligation-001` | frozen-clause repair, scoped by the sweep | `unstable-projection-gate-and-clause-disposition` |
| `kb-open-question-ps-19-scope-narrower-001` | same shape, same discipline; ADR-0018 names it and defers | `unstable-projection-gate-and-clause-disposition` |
| `kb-open-question-cf-40-ownership-001` | DT-3's authoritative-source question; cited, never re-litigated | `projection-api-design-record` (project AC-006) |
| `kb-open-question-projection-id-unvalidated-001` | `ProjectionId::new` stays infallible; not hardened as a side effect of freezing around it | Architecture brief AC-A09 — outside this project |
| `kb-open-question-global-vs-boundary-visibility-001` | a boundary-scoped projection checkpoint reopens ADR-0013's globally frozen invariant — **stopped and filed, not absorbed**; it surfaced while drafting ADR-0018 §2 and is recorded there | its own decision, if the design drifts there |
| `kb-open-question-post-phase-reconciliation-001` | flagged, not settled; AC-014 covers only this project's own clauses | unassigned |
| `kb-open-question-adr-status-vocabulary-001` | **followed as a convention, not answered** — `status: accepted` with the qualification and its falsifier in `summary`'s first clause; each intake document says so and instructs the wave not to act on it | unassigned |

One is answered, by ADR-0017: `kb-open-question-projection-batch-no-apply-001`,
sub-questions 1, 2 and 4. **Sub-question 3** — whether closing it retroactively
validates ADR-0006's encoding-versus-orchestration discriminator — stays open with
HS-P0011, and both the record and the intake document say so.

### The sweep's verdict as received, and what it did to scope

`references/evaluation/ps-clause-pairing-sweep.md` returned **isolated**: three
`independent` same-shape defects against a threshold of five, and two of those
inside §4.11's table against a threshold of three. **No re-plan is raised**, and no
record's scope was widened. Each record consumed its share:

- **ADR-0017** names **PS-8** and **PS-13** as defective, states the repair/gap
  test that makes both a decision's rather than an edit's, and defers.
- **ADR-0018** names **PS-19**, which was the trap the spec warned about, and
  defers. It also routes PS-18's *"testkit fixture store"* wording ambiguity to
  `reset-rules` and `projection-capability-skips` rather than fixing it.
- **ADR-0019** names **PS-29** — the sweep's *third independent* defect, and the
  one that turned the verdict's reading from *"§4.11's table was populated
  carelessly"* into *"the rule was written to the clause's intent rather than to
  its sentence"* — and records **PS-28** as `undetermined` rather than resolving
  it.

### Two judgement calls worth flagging to review

**`LiveHandleProjectionStore` is dispositioned as *moved to
`experiments/live-handle-projection-batch/`*.** AC-001 requires the disposition to
be *named*, not to take a particular value, and this spec is explicit that it does
not pre-empt the choice. The reasoning is in ADR-0017 §4: it cannot compile
against the port once `type Batch;` lands, deleting it would leave PS-5 with no
surviving counter-case, and `experiments/` is already the tree's home for a
reproducible measurement that is not in the gate. Execution is
`owned-batch-port-shape`'s, inside project AC-013's bound.

**Eight citations were re-anchored before commit.** Ranges written from memory
resolved to the right document and the wrong lines; each was re-read against the
working tree and corrected to the range whose subject text is quoted beside it —
`.kb/playbooks/anchoring-citations-in-a-long-lived-document.md`'s *verify the
referent, not the address*, applied by hand at this document count. That is NF-003
working rather than a defect, but it is recorded because the failure mode is
silent.
