---
item: "HS-S0002"
stage: report
created: "2026-08-13"
updated: "2026-08-13"
---

# Report — ADR-0017, ADR-0018 and ADR-0019 accepted before the port changes

## Findings Ledger

> **COMPLETE. Ten of ten ACs satisfied.** Eight of them required three **accepted
> `.kb/decisions/` atoms**, which only a **human-invoked `/redkiln:kb-ingest`
> wave** may author. That wave ran: `2026-08-13-projection-adrs`, at **`493a194`**,
> merged at **`d05d2b3`**, both ancestors of HEAD. It was not run from inside the
> implementation — AC-007 requires it to *be* a wave, and this spec instructs
> against it — so the story reached its checkpoint with two ACs green and eight
> blocked on one named dependency, and the wave discharged exactly that
> dependency and nothing else. The port did not move in the meantime:
> `crates/happenstance-core/src/projection.rs` is the same blob at `main`, at
> `493a194` and at HEAD.

| Finding | Evidence | Follow-up |
| ------- | -------- | --------- |
| **Three long-form records exist, and they were the substance the wave ingested.** `references/adr/0017-what-a-projection-batch-owns.md`, `…/0018-returning-a-projection-to-never-run.md`, `…/0019-what-happens-when-apply-fails.md`. Each carries the runbook's question verbatim, a strength grading of its halves, quoted evidence, the decision, the alternatives that lost with reasons, the provisional halves with falsifier **and** phase, and what is deferred and to whom. | `scratchpad/adr-checks.sh`: 35/35 pass; red baseline at story 1's checkpoint, 19 failures | Done: the wave folded each into a 116–125-line atom, and the records stay in `references/adr/` holding the transcripts a summary cannot. |
| **AC-004 satisfied — evidence quoted, absences quoted honestly.** ADR-0017 carries six `references/adapter-shapes.md:<range>` citations. ADR-0018 and ADR-0019 quote the **stated absence** from §5 — *"Durability \| Nothing. Every body is `todo!()` \| Phase 8"* (`:286-303`) — because no skeleton exercised `reset` or an apply failure, and each says in its own text why a fabricated diagnostic would be a defect (`:29-33`). Zero diagnostics appear that are absent from the referent. | `_ledger.md` AC-004 evidence; `adr-checks.sh` stated-absence assertions | None. |
| **AC-006 satisfied — no title carries a strong decision and a weak one undifferentiated.** ADR-0017 opens with a three-row strength table; ADR-0018 with a four-row table across three strengths and the statement that levelling them is `one-decision-per-adr-title`'s recorded failure; ADR-0019 puts its *Out of scope* paragraph first and defers three items to HS-P0011 **by name**. Every provisional half names a falsifier and a phase. | `_ledger.md` AC-006 evidence; `.kb/playbooks/one-decision-per-adr-title.md` | None. |
| **The sweep was consumed, not re-derived.** Verdict received as **isolated**; no record's scope was widened and **no re-plan is raised**. ADR-0017 names PS-8 and PS-13; ADR-0018 names PS-19; ADR-0019 names PS-29 and records PS-28 as `undetermined`. Each states the repair/gap test and defers to `unstable-projection-gate-and-clause-disposition`. **Nothing `[FROZEN]` is line-edited.** | `references/evaluation/ps-clause-pairing-sweep.md`; each record's scope section | The frozen-clause repairs land at slice 8 on the sweep's evidence. |
| **`LiveHandleProjectionStore` has a named disposition: moved to `experiments/live-handle-projection-batch/`, not deleted.** It cannot compile against the port once `type Batch;` lands; deleting it would leave PS-5 with no surviving counter-case, and deleting the only compiled evidence against a decision is how a port gets frozen against its own hypothesis. | `references/adr/0017-…md` §4; `crates/happenstance-ladybug/src/live_handle.rs` | Executed by `owned-batch-port-shape`, inside project AC-013's *"no change other than the removal of the batch's lifetime parameter"*. |
| **AC-001, AC-002, AC-003, AC-005 — the atom half is now the green half too.** Every substantive claim each criterion demands survived the fold from a 1,000-line record into a ~120-line atom: `E0195` + the `DefId::expect_local` ICE as PS-5's ground rather than `Send` (`0017-…:65-74`); the PS-15 caveat and its run-time discharge (`:76-80`); PS-9/PS-11 as one split-by-consumer decision (`:82-88`); PS-17 cited to ADR-0007 rather than re-derived (`0018-…:70-73`, with `depends_on: kb-decision-0007`); refusal's losing alternative named (`:75-80`); `(None, true)` (`:82-86`); the port growing nothing for apply failure (`0019-…:59-61`); `PumpError` / `SkipAndRecord` / observability deferred to HS-P0011 by name (`:89-96`). Each criterion is phrased *"WHEN they read `.kb/decisions/00NN-*.md`"*, and now they can. | `_ledger.md` AC-001/002/003/005 evidence, each cited to atom line ranges; `redkiln validate --kb` green | None. |
| **AC-007, AC-008 (second half), AC-009 — the wave operations, executed as pre-specified.** The wave record is the five-file set under `.kb/_governance/integration-waves/2026-08-13-projection-adrs/`; `.kb/_intake/` is down to its `README.md`, dropped at the approval gate rather than ingested (`03-integration-summary.md:105-106`); `kb-open-question-projection-batch-no-apply-001` moved to `superseded` with reciprocal `related` edges and its body verbatim; the three rows are at `.kb/maps/decision-map.md:91-93`. The wave adjudicated from the intake documents rather than inventing — `02-placement-and-adjudication.md` is 640 lines of that. | `.kb/_governance/integration-waves/2026-08-13-projection-adrs/**` | None. |
| **Two places the wave exceeded a row's `verifying_test`, stated rather than glossed.** AC-007's check said `adr-status-vocabulary-exceeds-the-schema.md` would be *unchanged*, and AC-008's said the answered question's diff would touch *frontmatter only*. Neither is literally true: the wave's reciprocal-backlink phase adds `related` ids and bumps `last_reviewed` on four open questions, and the answered question gains an appended dated section. **No criterion's substance moves** — no body line is edited anywhere, no `status` moves except the one the criterion asks for, and no question is answered. | `git diff main...HEAD -- .kb/open-questions/`: every `-` line is inside frontmatter | Recorded in `_ledger.md` on both rows and in the implementation report. |
| **The wave reached three atoms outside this story's PR boundary, and it was right to.** `.kb/playbooks/one-decision-per-adr-title.md`, `…/repairing-a-frozen-clause-without-amending-it.md` and `.kb/reference/port-traits-compiled-findings.md` gained `related` backlinks and a `last_reviewed` bump — frontmatter only, no body line touched. A `related` edge that resolves one way is a dangling link, and `validate --kb` checks both ends. The boundary block was **not** widened after the fact; the exception is recorded instead. | `git diff main...HEAD -- .kb/playbooks/ .kb/reference/` | If a later wave's backlink phase reaches into a **body**, that is a finding against the wave. |
| **AC-010 has its sha.** The atoms were committed at `493a194` on a tree where `crates/happenstance-core/src/projection.rs` is byte-identical to `main` (blob `fc398b7`, the same at `main`, `493a194` and HEAD); `git diff --stat main...493a194 -- crates/ spec/ RUNBOOK.md` is empty; and `redkiln validate --kb` is green over that tree's KB — the `.kb` tree object `1dd6f45` is shared by `493a194`, `d05d2b3` and HEAD, so the run is a run over the wave commit's inputs and not a later one's. | `_ledger.md` AC-010; implementation report, *The wave, and the sha AC-010 asks for* | `owned-batch-port-shape` may now start; the ordering it was blocked on is on the record. |

**Mount point (made):** `.kb/maps/decision-map.md:70-93` — a new dated section and
three rows, one per ADR, each with the atom link, `accepted`, phase 6 and `—` in
the supersession column. A library has no render tree; the corpus's composition
root is its index, and an atom absent from the map is a `lib.rs` export block's
missing `pub use`. The rows were written out in the intake documents so the wave's
Maps phase placed them rather than inferring them. The Status column reads
`accepted`, not "accepted (provisional)": that string is in no `KbFrontmatter`
enum, and printing it here would be the map answering
`kb-open-question-adr-status-vocabulary-001` by acting on it — the section
preamble says so, and sends the reader to each atom's own `## Provisional`.

**Deferred (by design, to HS-P0011):** `PumpError<E::Error, P::Error, A>` and its
three type parameters; the `SkipAndRecord` policy vocabulary; the supervisor's
observability half. Each is named in ADR-0019's *Out of scope* table with the
specification line that states it. An ADR that designed any of them would be
reversed by the project that owns them.

## Acceptance

| AC | Status | Verification as run |
| --- | --- | --- |
| AC-001 | **satisfied** | `.kb/decisions/0017-what-a-projection-batch-owns.md` — `E0195` and the `DefId::expect_local` ICE as the ground, with `references/adapter-shapes.md:186-194` and `:307-365` cited by line (`:65-70`); the `Send` argument stated as refuted by LadybugDB's `Send + Sync` `Connection` (`:71-74`); PS-15 left provisional and discharged at run time through `CommitError::ForeignBatch` (`:76-80`); PS-9/PS-11 as one split-by-consumer decision (`:82-88`); `LiveHandleProjectionStore` moved to `experiments/`, not deleted (`:92-95`). |
| AC-002 | **satisfied** | `.kb/decisions/0018-…` separates four halves of three strengths and says it is not levelling them; `reset(batch, id)` as one unit with the two-connections incident it models (`:60-68`); PS-17 cited to ADR-0007 and applied to removal, `depends_on: kb-decision-0007` (`:70-73`); refusal as mechanism with policy in the domain, the typed-layer-only alternative named as what lost (`:75-80`); `(None, true)` (`:82-86`); PS-19 named as a gap and not repaired (`:113-116`). |
| AC-003 | **satisfied** | `.kb/decisions/0019-…` — *the port grows nothing*, in the title and at `:59-61`; the skip primitive shown to already exist (`:63-70`); `rollback` surviving on PS-30's `AssertUnwindSafe` (`:72-77`); policy per projection, not per runner (`:83-87`); `PumpError`, `SkipAndRecord` and the supervisor's observability half deferred to HS-P0011 by name (`:89-96`). No runner error type is designed. |
| AC-004 | **satisfied** | Six / two / two `adapter-shapes.md:<range>` citations; both stated-absence quotes present; eight ranges re-anchored to their subject text before commit. |
| AC-005 | **satisfied** | All three *atoms* carry a non-empty `## Alternatives rejected` with a reason per entry — `0017:106-116` (supertrait with `put`/`get`; probe in the testkit; keeping the GAT; the `Send` argument; a generative brand; the receiver-lifetime batch), `0018:97-104` (two-statement reset; adapter-initiated clear; refusal as typed-layer-only; re-deriving PS-17), `0019:106-114`. The `(Option<SequencePosition>, bool)` checkpoint is named with its reason at `0018:82-86`. |
| AC-006 | **satisfied** | Strength tables in all three records; falsifier + phase on every provisional half; the sweep quoted in each record's scope section; `[FROZEN]` repairs deferred, not absorbed; no re-plan raised. |
| AC-007 | **satisfied** | The wave ran: `2026-08-13-projection-adrs` at `493a194`, no id collision, five-file wave record, `.kb/_intake/` cleared of all four staged documents with `README.md` dropped at the approval gate rather than ingested. Frontmatter invents no keys; all three carry `status: accepted`, `supersedes`/`superseded_by` `null`, and the provisional qualification **with its falsifier** in the first clause of `summary`. `redkiln validate --kb`, `redkiln validate` pass; `redkiln doctor` unchanged. One deviation from the row's check, recorded: the vocabulary open question is not byte-unchanged — it gained three `related` backlinks and a `last_reviewed` bump, body and `status` untouched. |
| AC-008 | **satisfied** | Nothing deleted; `projection-store-batch-has-no-apply-seam.md` moved to `superseded` with reciprocal `related` edges in both directions and its 2026-08-10 body verbatim beneath an appended dated section; sub-question 3 stays open with HS-P0011 in all three places. The six deliberately-unanswered questions are named with owners in the implementation report and **none is answered** — two byte-identical, two frontmatter-only, two carrying an appended amendment that settles nothing. |
| AC-009 | **satisfied** | Three rows at `.kb/maps/decision-map.md:91-93` with `accepted`, phase 6 and an empty supersession column; the resolved question still listed at `.kb/maps/open-questions-index.md:99-105`, marked **Superseded** and annotated with its answer rather than removed; `validate --kb` resolves the reciprocal edges both ways. |
| AC-010 | **satisfied** | The atoms are committed at `493a194`, on a tree whose `crates/happenstance-core/src/projection.rs` is the same blob as `main`'s (`fc398b7`), with `git diff --stat main...493a194 -- crates/ spec/ RUNBOOK.md` empty and `redkiln validate --kb` green over that tree's KB (`.kb` tree `1dd6f45`, shared with `d05d2b3` and HEAD). `owned-batch-port-shape` had not started, so the port could not have moved first. |

## Knowledge Harvest

- **The three decisions themselves**, now folded in: ADR-0017 (batch ownership and
  the split-by-consumer seam), ADR-0018 (reset scope, atomicity and refusal),
  ADR-0019 (the port grows nothing for apply failure) — `.kb/decisions/0017-…`,
  `0018-…`, `0019-…`, accepted at `493a194`. Their long-form records stay in
  `references/adr/`, which is the two-places-on-purpose split `CLAUDE.md`
  describes: link the atom, cite the record by `file:line`.
- **A `playbook` candidate: an ADR whose evidence is an absence.** Two of these
  three had no compiler transcript, because every skeleton body is `todo!()`. The
  transferable move is to *quote the stated absence* from the evidence base rather
  than fabricate a diagnostic or omit the section — and the reason is already
  written down in `references/adapter-shapes.md:29-33`: a table showing only
  `error[E….]` ranks the least compatible adapter as the most compatible.
- **A `concept` candidate: grading a conjunction before deciding it.** All three
  runbook questions were conjunctions; the strength table each record opens with
  (settled by being made / settled elsewhere and cited / provisional with falsifier
  and phase) is the operational form of `one-decision-per-adr-title`'s
  discriminator, and it made ADR-0018's PS-17 half a citation instead of a second
  derivation.
- **A process finding worth keeping:** an implementation story whose deliverable is
  a `.kb/` atom cannot complete inside an implementation agent, because atom
  authorship is reserved to a human-invoked wave. The story's own AC-007 states the
  constraint and its ACs then depend on the wave's output. Whether such a story
  should be planned as two — *records staged* and *wave applied* — is a candidate
  open question for closeout, not settled here. What this run adds to it: the
  story reached `report` **before** the wave ran, so the report and the ledger
  described a tree that stopped being true three commits later and had to be
  re-derived against `HEAD`. A story whose acceptance depends on a later human
  gate needs its ledger re-run *at* that gate, and nothing in the pipeline
  currently schedules that.
- **A second one, from the ledger itself:** two of this story's `verifying_test`
  fields encoded *"file unchanged"* / *"frontmatter only"* as proxies for
  *"nothing was settled in passing"*. The `kb-ingest` reciprocal-backlink phase
  breaks both proxies while satisfying every criterion, because a backlink is a
  frontmatter write to a file the story never names. A check written as a
  file-level diff assertion over `.kb/` will keep tripping on that; the property
  worth asserting is *no body line changed and no `status` moved*, which is
  checkable and is what was checked here.
