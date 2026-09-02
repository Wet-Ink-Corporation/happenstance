---
item: "HS-S0002"
stage: implement
created: "2026-08-13"
updated: "2026-08-13"
---

# Implementation Report — ADR-0017, ADR-0018 and ADR-0019 accepted before the port changes

> **STATUS: COMPLETE. Ten of ten.** This report was first written at the
> implementation checkpoint, when eight of the ten criteria were blocked on the
> one thing an implementation agent is not permitted to do — author a
> `.kb/decisions/` atom. That block is gone: the **human-invoked
> `/redkiln:kb-ingest` wave ran**, as `2026-08-13-projection-adrs`, at
> **`493a194`**, and merged into the initiative branch at **`d05d2b3`**. Both
> commits are ancestors of HEAD.
>
> What this story delivered on its own account is unchanged and is what the wave
> consumed: **three long-form records** under `references/adr/` and **three staged
> intake documents** under `.kb/_intake/`, now folded into
> `.kb/decisions/0017-…`, `0018-…` and `0019-…` — `status: accepted`, phase 6.
> The eight rows are flipped in `_ledger.md` against artefacts that exist, with
> the two places the wave's behaviour exceeded a row's `verifying_test` written
> out rather than glossed (AC-007 and AC-008, both in the reciprocal-backlink
> phase). The section *The wave, and the sha AC-010 asks for* records the
> ordering evidence; *What the wave touched outside this story's PR boundary*
> records the one boundary exception.

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
| AC-001 | the *record* half — `E0195`, `DefId::expect_local`, `live_handle.rs`, `LiveHandleProjectionStore`, `ForeignBatch`, `ProjectionProbe` all named in ADR-0017 | **red** six `FAIL`s → **green** six `PASS`es. **AC now `true`:** the same six claims are in the atom, at `.kb/decisions/0017-…:65-95`. |
| AC-002 | the *record* half — ADR-0018 cites `0007-projection-runner-decodes` for PS-17, names the typed-layer-only alternative, names `(None, true)`, and defers PS-19 to `unstable-projection-gate-and-clause-disposition` | **red** four `FAIL`s → **green** four `PASS`es. **AC now `true`:** all four in the atom (`:70-73`, `:75-80`, `:82-86`, `:113-116`), with `depends_on: kb-decision-0007`. |
| AC-003 | the *record* half — ADR-0019 names `HS-P0011`, `PumpError`, `SkipAndRecord`, `AssertUnwindSafe`, and designs no runner error type | **red** four `FAIL`s → **green** four `PASS`es. **AC now `true`:** the atom's *Deferred, by name, to the typed-layer phase (HS-P0011)* section is `:89-96`, and no runner error type is designed anywhere in it. |
| AC-005 | every record carries a non-empty `## Alternatives rejected` section | **red** three `FAIL`s → **green** three `PASS`es. **AC now `true`:** all three *atoms* carry one too — `0017:106-116`, `0018:97-104`, `0019:106-114` — with a reason per entry. |
| AC-007 | the wave ran, `.kb/_intake/` is clear of this wave's documents, a non-colliding directory exists under `.kb/_governance/integration-waves/` | **red** at the checkpoint → **green** at `493a194`. Wave id `2026-08-13-projection-adrs` (no collision), the five-file wave record present, `.kb/_intake/` down to its `README.md`, which `03-integration-summary.md:105-106` records as dropped at the approval gate. |
| AC-008 | `git diff --diff-filter=D -- .kb/open-questions/` empty; the answered question's `status` flipped with `related` edges and its body verbatim | both halves **green**. No open question deleted; `projection-store-batch-has-no-apply-seam.md` moves to `superseded` with reciprocal `related` edges, and every `-` line in its diff is inside the frontmatter — the 2026-08-10 body is verbatim beneath a purely appended dated section. |
| AC-009 | three rows in `.kb/maps/decision-map.md`; the resolved question annotated in `.kb/maps/open-questions-index.md` | **green.** Rows at `decision-map.md:91-93`; the resolved question still listed and annotated at `open-questions-index.md:99-105`. |
| AC-010 | `git diff --stat main...HEAD -- crates/ spec/ RUNBOOK.md` empty | **green**, and now with the sha the criterion asks for: `493a194`. See *The wave, and the sha AC-010 asks for*. |

Check script: `scratchpad/adr-checks.sh`, run as `adr-checks.sh <rev>` for the red
baseline and with no argument for green. Kept out of the repository: this story's
PR boundary is `references/adr/**`, `.kb/**` and its own backlog folder, and a
shell script is none of them.

## Commits

One story checkpoint, no fixups, plus the wave that consumed its output. The
checkpoint's own sha cannot be written into a file the commit contains; it is
recorded on the item through `redkiln record-links`. The wave's sha can be, and
is, because it landed afterwards.

| SHA | Subject |
| --- | ------- |
| *on the item's `links.commits`* | `feat(projection-store-freeze): The three projection decision atoms` |
| `493a194` | `kb(2026-08-13-projection-adrs): ingest the three projection decision atoms` |
| `d05d2b3` | `Merge the 2026-08-13-projection-adrs wave into the initiative` |

The checkpoint's parent is story 1's checkpoint,
`feat(projection-store-freeze): Sweep the PS clause range for the pairing defect`,
which is what makes the sweep's finding an **input** to these records rather than
a claim about them.

## Changes

| Path | Shape of the change |
| --- | --- |
| `references/adr/0017-what-a-projection-batch-owns.md` | **New.** The long-form record for PS-4 – PS-15: the runbook's question, a strength table over its three halves, four quoted transcripts, the split-by-consumer decision (`type Batch;` + no write vocabulary + `ProjectionProbe` behind `conformance`), the drop/rollback decision, `LiveHandleProjectionStore`'s disposition, consequences, and seven rejected alternatives with reasons. |
| `references/adr/0018-returning-a-projection-to-never-run.md` | **New.** PS-16 – PS-20: four decisions across three strengths, the stated-absence transcript, the night-desk incident the atomicity decision models, PS-17 cited to ADR-0007 rather than re-derived, refusal as mechanism-with-policy-elsewhere, the three-variant `Checkpoint`, PS-19's defect named and deferred, six rejected alternatives. |
| `references/adr/0019-what-happens-when-apply-fails.md` | **New.** PS-26 – PS-30: *Out of scope* written first, the three deferrals to HS-P0011 by name in a table, the decision that the port grows nothing, the skip primitive that already exists, why `rollback` survives, PS-29's defect named and deferred, PS-28 recorded `undetermined`, six rejected alternatives. |
| `.kb/_intake/2026-08-13-adr-0017-projection-batch.md` | **New, staged; consumed by the wave.** The wave's input for ADR-0017's atom: frontmatter instructions (invent no keys; `status: accepted` with the qualification **and its falsifier** in `summary`'s first clause), the claims, the rejected alternatives, the provisional halves with falsifier + phase, the **resolve-not-delete** protocol for `kb-open-question-projection-batch-no-apply-001`, the map rows, and an explicit *Not proposed* list. Carried the wave-id warning. |
| `.kb/_intake/2026-08-13-adr-0018-reset.md` | **New, staged; consumed.** Same shape for ADR-0018. Resolves no open question. |
| `.kb/_intake/2026-08-13-adr-0019-apply-failure.md` | **New, staged; consumed.** Same shape for ADR-0019. Resolves no open question. |
| `.kb/decisions/0017-…`, `0018-…`, `0019-…` | **New, by the wave at `493a194`** — not by this story's checkpoint, which is the whole point of AC-007. 125, 116 and 125 lines; `status: accepted`, phase 6, `supersedes`/`superseded_by` `null`. |
| `.kb/open-questions/**`, `.kb/maps/**`, `.kb/_governance/integration-waves/2026-08-13-projection-adrs/**` | **By the wave.** One `status` flip with reciprocal edges and four backlink-only amendments; three map atoms; the five-file wave record. |
| `.bklg/.../projection-decision-atoms/**` | `_ledger.md` (**ten** rows `satisfied: true` with cited evidence — two at the checkpoint, eight after the wave; no criterion and no `verifying_test` touched), this report, and `report.md`. |

Not touched by this story or its wave, and asserted by the gate: `crates/**`,
`spec/SPECIFICATION.md`, `RUNBOOK.md`.

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

### The wave, and the sha AC-010 asks for

**The blocking dependency is discharged.** The wave that was missing at the
checkpoint — one **human-invoked `/redkiln:kb-ingest`**, fed from the three
`.kb/_intake/2026-08-13-adr-00{17,18,19}-*.md` documents this story staged — ran
as `2026-08-13-projection-adrs` and landed six operations: three atoms created,
three open questions amended, one of them to `superseded`.

AC-010 does not ask whether the atoms exist. It asks for a **commit** at which
they existed on a tree the port had not moved on, and for that commit to be
written down. It is:

| Fact AC-010 asks for | Value | How it was taken |
| --- | --- | --- |
| The commit the atoms were accepted at | **`493a194`** | `kb(2026-08-13-projection-adrs): ingest the three projection decision atoms` |
| Merged into the initiative branch at | **`d05d2b3`** | both are ancestors of HEAD |
| `crates/happenstance-core/src/projection.rs` byte-identical to `main` **on that tree** | **yes** — blob `fc398b7f3e01995f3d2cd4ffe14d1b99700f2b7c` at `main`, at `493a194` and at HEAD | `git rev-parse main:… 493a194:… HEAD:…` returns one sha three times |
| No diff under `crates/**`, `spec/**`, `RUNBOOK.md` | **empty**, at the wave commit and three commits later | `git diff --stat main...493a194 -- crates/ spec/ RUNBOOK.md` and `main...HEAD` both print nothing |
| `redkiln validate --kb` green **on that tree**, not a later one | **`validate passed`** | the `.kb` tree object is `1dd6f4539f1c5bc272f44c0285a53996fdac59e5` at `493a194`, at `d05d2b3` and at the pre-fix HEAD `a3b9a33` — one identical tree, so the run is a run over `493a194`'s KB |

The last row is the one worth stating carefully, because "green on that tree" is
the whole content of the criterion and a green run at a *later* commit would not
be it. The three commits share a `.kb` tree object, so they are the same inputs
by construction rather than by inspection; that is a stronger check than
re-running the tool at a detached checkout, and it does not require one.

`owned-batch-port-shape` (HS-S0003) has not started. The port could not have moved
first, which is what project AC-008 wanted the ordering to prove.

**Why the wave was not run from inside the implementation.** Three independent
reasons, any one sufficient, and all three still the right answer in hindsight:

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

### What the wave touched outside this story's PR boundary

Stated because the boundary block in `spec.md` does not name it, and an
unexplained file in a diff is indistinguishable from scope creep.

The wave's **reciprocal-backlink phase** amended three atoms outside the fenced
list — `.kb/playbooks/one-decision-per-adr-title.md`,
`.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md` and
`.kb/reference/port-traits-compiled-findings.md`. Every one of the three is
**frontmatter only**: `related` gains the new `kb-decision-00{17,18,19}` ids and
`last_reviewed` moves to 2026-08-13. `git diff main...HEAD -- .kb/playbooks/
.kb/reference/` shows six added `related` lines, three changed `last_reviewed`
lines and nothing else; no body line in any of the three is touched, and no
claim in any of them changes.

**Why this is the ingest's contract rather than drift.** A `related` edge that
resolves in one direction only is a dangling link — `redkiln validate --kb`
checks both ends, and the three atoms are precisely the ones this story's own
boundary block lists under *Wires into* as the guidelines these decisions are
written under. Writing an outbound edge to them and declining to write the
inbound one would leave the KB in the state the check exists to reject. The
boundary block enumerates what **this story authors**; the wave's backlink phase
writes wherever the graph says it must, and that is a property of `kb-ingest`,
not of this story's scope.

**Which arm was taken, and why.** The boundary block was **not** widened after
the fact to `.kb/playbooks/**` and `.kb/reference/**`. Editing a story's own
boundary to match what its diff turned out to contain is how a boundary stops
being a constraint; the exception is recorded here instead, where a reviewer
reading the diff will look. If a later story finds the same phase reaching
further — into a body rather than into frontmatter — that is a finding against
the wave, not an omission in this record.

### The six open questions this story deliberately does not answer

Reproduced here because the spec requires it and because an unexplained omission
is indistinguishable from an oversight. **None of the six is answered**, every one
keeps `status: accepted` and its owner, and
`git diff --diff-filter=D main...HEAD -- .kb/open-questions/` is empty — nothing
was deleted.

Four of them are no longer byte-identical to `main`, and the difference is worth
being exact about rather than rounding to "untouched". `cf-40-fixture-limits-ownership.md`
and `projection-id-is-unvalidated.md` are byte-identical. `adr-status-vocabulary-…`
and `global-versus-per-boundary-…` changed in **frontmatter only** — `related`
gains the new decision ids, `last_reviewed` moves to 2026-08-13. `ps-1-…` and
`ps-19-…` additionally carry an **appended** dated section recording what the
sweep found, with their prior bodies byte-for-byte above it and every `-` line in
their diffs inside the frontmatter. An appended amendment that answers no
sub-question and moves no `status` is the KB recording an input, not this story
settling a question in passing.

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
sub-questions 1, 2 and 4. It moved to `status: superseded` in the wave, keeping
its body verbatim under an appended *Answered 2026-08-13* section.
**Sub-question 3** — whether closing it retroactively validates ADR-0006's
encoding-versus-orchestration discriminator — stays open with HS-P0011, and the
record, the atom (`0017-…:120-123`) and the question itself (`:129-133`) all say
so.

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
