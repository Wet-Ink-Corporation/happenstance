---
item: "HS-S0002"
stage: report
created: "2026-08-13"
updated: "2026-08-13"
---

# Report — ADR-0017, ADR-0018 and ADR-0019 accepted before the port changes

## Findings Ledger

> **BLOCKED.** Two of ten ACs satisfied. Eight require three **accepted
> `.kb/decisions/` atoms**, which only a **human-invoked `/redkiln:kb-ingest`
> wave** may author. The wave's inputs are complete and staged; the wave has not
> run and was not run from inside the implementation, because AC-007 requires it
> to *be* a wave and because this spec instructs against it.

| Finding | Evidence | Follow-up |
| ------- | -------- | --------- |
| **Three long-form records exist, and they are the substance the wave ingests.** `references/adr/0017-what-a-projection-batch-owns.md`, `…/0018-returning-a-projection-to-never-run.md`, `…/0019-what-happens-when-apply-fails.md`. Each carries the runbook's question verbatim, a strength grading of its halves, quoted evidence, the decision, the alternatives that lost with reasons, the provisional halves with falsifier **and** phase, and what is deferred and to whom. | `scratchpad/adr-checks.sh`: 35/35 pass; red baseline at story 1's checkpoint, 19 failures | The wave folds each into a ~100-line atom; the records stay in `references/adr/` and hold the transcripts a summary cannot. |
| **AC-004 satisfied — evidence quoted, absences quoted honestly.** ADR-0017 carries six `references/adapter-shapes.md:<range>` citations. ADR-0018 and ADR-0019 quote the **stated absence** from §5 — *"Durability \| Nothing. Every body is `todo!()` \| Phase 8"* (`:286-303`) — because no skeleton exercised `reset` or an apply failure, and each says in its own text why a fabricated diagnostic would be a defect (`:29-33`). Zero diagnostics appear that are absent from the referent. | `_ledger.md` AC-004 evidence; `adr-checks.sh` stated-absence assertions | None. |
| **AC-006 satisfied — no title carries a strong decision and a weak one undifferentiated.** ADR-0017 opens with a three-row strength table; ADR-0018 with a four-row table across three strengths and the statement that levelling them is `one-decision-per-adr-title`'s recorded failure; ADR-0019 puts its *Out of scope* paragraph first and defers three items to HS-P0011 **by name**. Every provisional half names a falsifier and a phase. | `_ledger.md` AC-006 evidence; `.kb/playbooks/one-decision-per-adr-title.md` | None. |
| **The sweep was consumed, not re-derived.** Verdict received as **isolated**; no record's scope was widened and **no re-plan is raised**. ADR-0017 names PS-8 and PS-13; ADR-0018 names PS-19; ADR-0019 names PS-29 and records PS-28 as `undetermined`. Each states the repair/gap test and defers to `unstable-projection-gate-and-clause-disposition`. **Nothing `[FROZEN]` is line-edited.** | `references/evaluation/ps-clause-pairing-sweep.md`; each record's scope section | The frozen-clause repairs land at slice 8 on the sweep's evidence. |
| **`LiveHandleProjectionStore` has a named disposition: moved to `experiments/live-handle-projection-batch/`, not deleted.** It cannot compile against the port once `type Batch;` lands; deleting it would leave PS-5 with no surviving counter-case, and deleting the only compiled evidence against a decision is how a port gets frozen against its own hypothesis. | `references/adr/0017-…md` §4; `crates/happenstance-ladybug/src/live_handle.rs` | Executed by `owned-batch-port-shape`, inside project AC-013's *"no change other than the removal of the batch's lifetime parameter"*. |
| **AC-001, AC-002, AC-003, AC-005 — record half green, atom half blocked.** Every substantive claim each criterion demands is present and check-asserted **in the record**: `E0195` + the `DefId::expect_local` ICE as PS-5's ground rather than `Send`; the PS-15 caveat; PS-9/PS-11 as one split-by-consumer decision; PS-17 cited to ADR-0007 rather than re-derived; refusal's losing alternative named; `(None, true)`; the port growing nothing for apply failure; `PumpError` / `SkipAndRecord` / observability deferred to HS-P0011. But each criterion is phrased *"WHEN they read `.kb/decisions/00NN-*.md`"*, and no atom exists. | `adr-checks.sh` AC-001/002/003 record assertions, all green; `_ledger.md` rows left `satisfied: false` | Satisfied by the wave, with no further authoring. |
| **AC-007, AC-008 (second half), AC-009 — blocked outright.** These *are* wave operations: the wave record under `.kb/_governance/integration-waves/`, an emptied `_intake`, the `open_question` status flip with reciprocal `related` edges and its body verbatim, and the three rows in `.kb/maps/decision-map.md`. All are pre-specified in the intake documents so the wave adjudicates rather than invents. | `.kb/_intake/2026-08-13-adr-00{17,18,19}-*.md` | Run the wave with id `2026-08-13-projection-adrs` (free; `2026-08-10-intake` and `-2` are taken). Drop `.kb/_intake/README.md` at the approval gate. |
| **AC-008's first half is green and stays green.** `git diff --diff-filter=D -- .kb/open-questions/` is empty; the six deliberately-unanswered questions are named with owners in the implementation report and all six files are untouched; `kb-open-question-adr-status-vocabulary-001` is followed as a convention and **not answered**. | `adr-checks.sh` `AC-008 no open question deleted`; the six-row owner table | None. |
| **AC-010's guarantee is intact even though its row is `false`.** `crates/happenstance-core/src/projection.rs` is byte-identical to `main`; `git diff --stat main...HEAD -- crates/ spec/ RUNBOOK.md` is empty; `owned-batch-port-shape` is blocked on this story, so the port cannot move first. What is missing is only a commit at which `validate --kb` was green over three accepted atoms. | `adr-checks.sh` AC-010 assertions, all green | The wave still lands the atoms before any port change. The ordering is not at risk; only its citation is. |

**Mount point (specified, not yet made):** `.kb/maps/decision-map.md` — three rows,
one per ADR, with status, phase 6 and an empty supersession column. A library has
no render tree; the corpus's composition root is its index, and an atom absent from
the map is a `lib.rs` export block's missing `pub use`. The rows are written out in
the intake documents so the wave's Maps phase places them rather than inferring
them.

**Deferred (by design, to HS-P0011):** `PumpError<E::Error, P::Error, A>` and its
three type parameters; the `SkipAndRecord` policy vocabulary; the supervisor's
observability half. Each is named in ADR-0019's *Out of scope* table with the
specification line that states it. An ADR that designed any of them would be
reversed by the project that owns them.

## Acceptance

| AC | Status | Verification as run |
| --- | --- | --- |
| AC-001 | **not satisfied** | Record half green (`adr-checks.sh`: `E0195`, `DefId::expect_local`, `live_handle.rs`, `LiveHandleProjectionStore`, `ForeignBatch`, `ProjectionProbe` all present in ADR-0017). Atom half blocked: `.kb/decisions/0017-*.md` does not exist. |
| AC-002 | **not satisfied** | Record half green (ADR-0018 cites `0007-projection-runner-decodes` for PS-17, names the typed-layer-only alternative, names `(None, true)`, defers PS-19 by name). Atom half blocked. |
| AC-003 | **not satisfied** | Record half green (ADR-0019 names `HS-P0011`, `PumpError`, `SkipAndRecord`, `AssertUnwindSafe`; designs no runner error type). Atom half blocked. |
| AC-004 | **satisfied** | Six / two / two `adapter-shapes.md:<range>` citations; both stated-absence quotes present; eight ranges re-anchored to their subject text before commit. |
| AC-005 | **not satisfied** | Every record carries a non-empty `## Alternatives rejected` with a reason per entry (`adr-checks.sh`, three passes). The criterion is about the three *atoms*. |
| AC-006 | **satisfied** | Strength tables in all three records; falsifier + phase on every provisional half; the sweep quoted in each record's scope section; `[FROZEN]` repairs deferred, not absorbed; no re-plan raised. |
| AC-007 | **not satisfied — blocked** | No `/redkiln:kb-ingest` wave has run. `.kb/_intake/` holds this story's three staged documents plus the pre-existing README. No new directory under `.kb/_governance/integration-waves/`. `redkiln validate --kb` and `redkiln validate` pass; `redkiln doctor` unchanged. |
| AC-008 | **not satisfied** | First half green: no open question deleted, six named with owners and untouched. Second half blocked: the `status` flip and reciprocal `related` edges are wave ops. |
| AC-009 | **not satisfied — blocked** | Map rows are the wave's Maps phase. Specified in the intake documents. |
| AC-010 | **not satisfied** | Its diff assertions are all green (`crates/`, `spec/`, `RUNBOOK.md` empty), but the criterion requires the three atoms to be committed on that tree. The ordering guarantee itself is intact. |

## Knowledge Harvest

- **The three decisions themselves**, once the wave folds them in: ADR-0017 (batch
  ownership and the split-by-consumer seam), ADR-0018 (reset scope, atomicity and
  refusal), ADR-0019 (the port grows nothing for apply failure). Their long-form
  records are already in `references/adr/`; the atoms are the wave's output.
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
  open question for closeout, not settled here.
