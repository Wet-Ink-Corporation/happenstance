---
item: HS-S0083
stage: spec
created: 2026-08-12T13:47:21.508Z
updated: 2026-08-12T13:47:21.508Z
template_sig: 87bbf1d0
rendered_sig: c457f2c5
---

# Spec — Merge the verdict ahead of publication, and hand over what it owes

## Scope lock

| Layer | Path | What it fixes for this story |
| --- | --- | --- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` | DoD 8 (*"the freeze verdict is written"*) and exit criterion 4 — the verdict exists **before** `0.2.0` is a promise |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` | the DAG edge `ladybug-projection-store` → `publication-and-positioning` (`:150`), and the merge order that puts ladybug at position 6, publication at 7 (`:185-186`) |
| Project charter | `.bklg/from-contract-to-published-library/ladybug-projection-store/project.md` | **AC-011** (sole owner), **DR-8**, and the out-of-scope line that keeps the `0.2.0` decision with HS-P0016 (`:127-130`) |
| Key briefs | `.bklg/from-contract-to-published-library/ladybug-projection-store/_decomposition.md` | architecture §7's three handover items (`:394-406`), M9's evidence-document lifecycle (`:211-218`), M5's packaging consequence (`:173-183`); testing brief's AC-011 row (`:481`) |
| Story map | `.bklg/from-contract-to-published-library/ladybug-projection-store/_storymap.md` | this story's row (`:49`) and slice 5's ordering clause (`:139-143`) |
| Signed-off design | `.bklg/from-contract-to-published-library/ladybug-projection-store/_design.md` | **no user-facing surface** — `## Surfaces` is `N/A` (`:40-43`) and the sign-off approves that determination (`:92-98`). This story renders no surface and must not invent one |
| This spec | `.bklg/from-contract-to-published-library/ladybug-projection-store/verdict-ordering-and-publication-handoff/spec.md` | the artefact you are reading |
| Roadmap pointer | `RUNBOOK.md` | phase 11's body (`:4393-4446`) — the licensed seam; phase 12 (`:4448-4500`); the status rows (`:162-163`) and the phase graph (`:176-190`) this story reconciles **by record, not by edit** |

## One-line PR slice

Merge the verdict to the initiative branch ahead of `publication-and-positioning`'s gate, state that
ordering explicitly rather than leaving it to the DAG, and hand HS-P0016 the build number, the docs.rs
failure and the `[package.metadata.docs.rs]` question.

## Executive summary

The slice-mate `freeze-verdict-document` writes the verdict. This PR is what makes it **arrive in time
and arrive at somebody**.

Two deltas, and both are small edits carrying a disproportionate obligation:

1. **A new dated evidence document**, `references/evaluation/phase-11-publication-handover.md`, pinned
   to the verdict's commit, that states the ordering as a claim (verdict merged at commit *X*, before
   HS-P0016 opens) and hands over the three residues phase 11 owes phase 12 — the measured `lbug` build
   number and the CI shape it bought, the docs.rs build failure now that `publish = false` is gone, and
   the `[package.metadata.docs.rs]` question, stated and *not* answered.
2. **A handover block at the end of `RUNBOOK.md` phase 11's body**, pointing at it, so a reader of the
   plan of record meets the edge rather than having to reconstruct it from the backlog.

Nothing about the freeze is re-decided, re-measured or re-argued here. Every number and every finding
is transcribed with its provenance from a sibling story that owns it.

## Context pack

Read this section before opening anything. It is the whole of what must be internalised; everything
deeper is behind a signposted anchor.

**1. The ordering is a decision, and the plan of record currently says the opposite.** `RUNBOOK.md:163`
gives phase 12 the dependencies **"7, 8"** — not 11. `RUNBOOK.md:188` draws phase 11 as one of *"three
that never rejoin — off the 0.1 path: … 6 ─▶ 11"*, and `RUNBOOK.md:243-245` lists it among the post-0.1
phases that *"parallelise freely"*. A reader who trusts that graph may open phase 12 with phase 11
unfinished, which is exactly the inversion DR-8 exists to forbid. The edge lives instead in the
initiative's own DAG (`.bklg/from-contract-to-published-library/_decomposition.md:150`,
`publication-and-positioning` `blocked_by` … `ladybug`) and in HS-P0016's charter, which lists this
project as an input it *reads* (`.bklg/from-contract-to-published-library/publication-and-positioning/project.md:327-328`).
**This story's job is to write that edge into the tree** — as a stated claim in the handover document
and a forward pointer at the end of phase 11's body — and to *record* the RUNBOOK discrepancy rather
than silently repair it. Phase 12's row, the phase graph and phase 12's body belong to HS-P0016 and to
a re-plan; the seam this project may edit is `RUNBOOK.md:4411-4444` and nothing else
(`…/ladybug-projection-store/_decomposition.md:82`, and the *"Nothing outside this list is this
project's to edit"* rule at `:67-68`).

**2. Evidence is superseded, never edited — so the handover is a new file.** `references/evaluation/`
holds *"immutable evidence… dated, pinned to a commit… must be superseded rather than edited"*
(`references/evaluation/README.md:9-13`), and M9 applies that lifecycle to phase 11's two documents
deliberately, *"because AC-001 is an ordering claim about commits and one file amended in place cannot
make it"* (`…/_decomposition.md:211-218`). The same argument applies one document later: appending a
handover section to the verdict in a second commit would reopen an artefact whose value is that it was
sealed. The governing test is `.kb/governance/rewrite-the-referent-never-the-reasoning.md` — *ask
whether the edit changes what the document asserts* — and adding a forward obligation to a sealed
verdict changes exactly that. **A third document, dated, pinned to the verdict's SHA.**

**3. The three items handed over are fixed, and one of them must stay a question.** Architecture brief
§7 states the obligation in one sentence: *"This project's obligation is to hand that project the number
and the `[package.metadata.docs.rs]` question, modelled on `crates/happenstance-core/Cargo.toml:54-56`"*
(`…/_decomposition.md:400-406`). The docs.rs failure is the third: `lbug` builds LadybugDB's C++ through
`cxx` and `cmake`, and *"docs.rs itself fails to build `lbug` 0.19.1, which is the same cost seen from
outside"* (`crates/happenstance-ladybug/src/lib.rs:24-32` today; `real-lbug-driver-swap` rewrites that
section, so cite it where it lands). Answering the `[package.metadata.docs.rs]` question here would take
a decision this project explicitly does not hold — *"whether this crate is published at `0.2.0` →
`publication-and-positioning`"* (`…/project.md:127-130`). Note also what the block on
`crates/happenstance-core/Cargo.toml:54-56` does and does not buy: `all-features = true` and
`rustdoc-args = ["--cfg", "docsrs"]` configure *rustdoc*, not the native toolchain, so it cannot by
itself turn a failing `cxx`/`cmake` build green. Hand over that constraint with the question.

**4. Transcribe; never re-measure and never re-open.** The number is
`cold-build-cost-and-ci-shape`'s (project AC-009, measured per architecture §7's protocol: a cold build
with a fresh `CARGO_TARGET_DIR` on each matrix runner plus a warm figure, each stamped with toolchain
and machine — `…/_decomposition.md:394-399`). The findings — T1, the GAT/ICE, `WriteTransactionInUse` —
are `freeze-verdict-document`'s (AC-007, AC-008). A handover that re-derives either produces a second
number that can disagree with the first, which is how an evidence corpus starts contradicting itself.
Every item in the handover carries the path and commit of the record it came from.

**5. Removing `publish = false` changed the docs.rs note into a consequence, and HS-P0016 will meet it
at a specific place.** `package-completeness-and-name-claim` drops `publish = false` from
`crates/happenstance-ladybug/Cargo.toml:12` and adds the crate to `xtask/src/package.rs`'s `PUBLISHABLE`
(M5, `…/_decomposition.md:173-183`). HS-P0016's working default is that `0.2.0` ships **exactly three**
crates — `happenstance-core`, `happenstance`, `happenstance-testkit` — and its `crate-set-decision`
story asserts that *"`xtask/src/package.rs`'s derived set and its `PUBLISHABLE` intention list agree
with no reconciliation failure"*
(`.bklg/from-contract-to-published-library/publication-and-positioning/_storymap.md:56`). So a
publishable-in-manifest crate that is not in the decided set is a fact that lands on a named story, and
the handover is where it is named — not discovered by a reconciliation failure in someone else's PR.

**6. The receiving items are real backlog stories; name them by slug.** An obligation handed to a
project is an obligation handed to nobody. The candidates, each verified in
`.bklg/from-contract-to-published-library/publication-and-positioning/_storymap.md`: `crate-set-decision`
(`:56`, the published set and the `PUBLISHABLE` reconciliation), `guarantees-and-docs-rs-presentation`
(`:66`, which already owns adding a `[package.metadata.docs.rs]` block and *"all three crates build docs
under all features"*), `rendered-page-preflight` (`:67`, reading the rendered docs.rs page before the
irreversible act) and `publish-0-2-0` (`:68`, the whole-gate run on the literal publish commit — the run
that would pay the `lbug` build cost if the crate set widened). `projection-port-ship-shape` (`:57`)
already reads this project's report for PS-3 and needs nothing new from this story.

**7. The persona slice.** The initiative has no `.kb/product/` atoms yet, so the journeys are carried in
`.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md` (the charter
says so, and says why inventing one would be the hand-authoring reverted at `0269720` —
`initiative.md:227-240`). The journey this story protects is **"Decide in one sitting"** — the
evaluator's bounded look at public evidence, ending in adopt or decline for a stated reason. A red
docs.rs page on a published crate, or a freeze verdict that arrives after the release it should have
informed, damages that journey in a way no gate step can detect afterwards, because a crates.io release
cannot be edited (`RUNBOOK.md:4481-4484`). The ordering *is* the user-facing behaviour here.

## Integration contract

- **Archetype**: `capability`. It lands an artefact a human reads and an obligation another project
  acts on — not substrate for a later slice.
- **Slice / milestone**: `freeze-verdict`. Slice-mates: **`freeze-verdict-document`** (the verdict
  itself; AC-007, AC-008). The two are implemented in one context and merged in that order
  (`…/_storymap.md:139-143`). De-confliction is explicit: the slice-mate writes the verdict document
  **and ticks phase 11's remaining exit-criteria boxes** (`…/_storymap.md:118-121`); this story writes a
  *new* document and *appends* the handover block. Neither edits the other's text.
- **Mount point**: **`RUNBOOK.md` — phase 11's body, within the licensed seam `RUNBOOK.md:4411-4444`.**
  A handover block at the end of phase 11 (after its exit criteria, before the `---` that opens phase
  12) stating the ordering and linking the handover document. RUNBOOK.md is the plan of record and *"how
  far it has got"* (`CLAUDE.md`, repository map); a handover that exists only under
  `references/evaluation/` is reachable from nothing a reader of the plan opens next.
- **Wires into**:
  - `references/evaluation/README.md` (`:9-13`) — the lifecycle the new document must obey: dated,
    pinned to a commit, superseded rather than edited.
  - The slice-mate's verdict document under `references/evaluation/` — cited by **path and commit
    SHA**, never edited, and never quoted at length.
  - `cold-build-cost-and-ci-shape`'s recorded number and CI decision in `RUNBOOK.md`'s phase 11 body —
    transcribed with its provenance.
  - `crates/happenstance-ladybug/Cargo.toml` and `crates/happenstance-core/Cargo.toml:54-56` — **read**,
    as the model for the question handed over. This story writes neither.
  - `.bklg/from-contract-to-published-library/publication-and-positioning/_storymap.md` — **read**, to
    name the receiving story slugs correctly.
- **Renders surfaces**: **none.** `_design.md` records `N/A — no user-facing surface` and that
  determination is what was signed off (`…/_design.md:40-43`, `:92-98`). No public Rust item is added,
  changed or removed; no doctest is owed.
- **Conformance rule(s)**: **none, and it is not adapter-observable.** This story changes no port, no
  fixture and no rule, so naming one would be decorative. Its instruments are `git log` commit order
  and the whole gate, per the testing brief's AC-011 row: *"a human-checked ordering claim"*, not a
  CI-enforced one (`…/_decomposition.md:481`).
- **Clause(s)**: **none amended.** `spec/SPECIFICATION.md` is not touched — no clause text, no marker,
  no citation. AC-008's *"nothing `[FROZEN]` moved"* obligation still applies to this PR as a standing
  check (`…/project.md:210-213`).
- **Advances DoD scenario**: **DoD 8** — *"After the unlike batch shape exists, a written verdict states
  whether the `ProjectionStore` freeze held, either way, with what it was checked against"*
  (`initiative.md:380-382`) — this story closes its *merged, and merged in time* half, which is
  initiative exit criterion 4 (`initiative.md:571-572`). It also guards **DoD 9–12** (the publication
  scenarios) by making phase 12 open onto a settled verdict rather than a pending one.

## PR boundary

**In this PR**

- `references/evaluation/phase-11-publication-handover.md` — new, dated, pinned to the verdict's commit
  SHA, carrying the ordering claim, the three handover items with provenance, and the receiving story
  slugs.
- `RUNBOOK.md` phase 11's body only (`:4411-4444`) — a handover block stating the ordering to phase 12
  and linking the document.
- This story's own backlog folder — the ledger and, later, its report.

**Explicitly not in this PR**

- **The verdict's content.** Held / did not hold, the rules run, the PS clause ids, the T1, GAT/ICE and
  `WriteTransactionInUse` findings, and the "did not hold" routing are `freeze-verdict-document`'s
  (AC-007, AC-008). Cite them; do not restate or revise them.
- **Editing the verdict document, or the axes document.** They are sealed evidence
  (`references/evaluation/README.md:9-13`).
- **Re-measuring the build, or re-deciding the CI shape** — `cold-build-cost-and-ci-shape` (AC-009).
- **Answering the `[package.metadata.docs.rs]` question, or adding the block to
  `crates/happenstance-ladybug/Cargo.toml`.** It is HS-P0016's
  (`…/project.md:127-130`; receiving story `guarantees-and-docs-rs-presentation`).
- **Deciding whether `happenstance-ladybug` is in the `0.2.0` crate set** — `crate-set-decision`.
- **Editing `RUNBOOK.md:163`'s dependency cell, `:176-190`'s phase graph, or phase 12's body.** The
  discrepancy is *recorded* in the handover document as a finding for HS-P0016, not repaired here; the
  seam this project may edit stops at `:4444` (`…/_decomposition.md:82`, `:67-68`).
- **Any `.bklg/**` frontmatter, including `blocks` / `blocked_by`.** The CLI is the only writer and a
  `PreToolUse` hook denies the edit (`CLAUDE.md`, *Where the work lives*). The DAG edge already exists;
  this story states it in prose, it does not re-encode it.
- **Any Rust source, any `Cargo.toml`, `spec/SPECIFICATION.md`, `xtask/**` and `.kb/**`.**
- **Incidental defects found in passing** → the `support` initiative (`.redkiln/config.yaml:5`).

**Merge DoD (one line).** The handover document and the RUNBOOK block are merged to the initiative
branch, `cargo xtask ci` is green on the merge commit, and `git log` shows both this PR's commit and the
verdict's ahead of the first commit of any `publication-and-positioning` story.

```
references/evaluation/phase-11-publication-handover.md
RUNBOOK.md
.bklg/from-contract-to-published-library/ladybug-projection-store/verdict-ordering-and-publication-handoff/**
```

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| A **new** handover document exists under `references/evaluation/` | `phase-11-publication-handover.md`: dated (ISO date in the body), pinned to the verdict's commit SHA, written in the genre of `phase-4-reconciliation.md`. It is a third file, not a section appended to the verdict — the verdict is sealed the moment it is committed | `references/evaluation/README.md:9-13`; `…/ladybug-projection-store/_decomposition.md:211-218`; `.kb/governance/rewrite-the-referent-never-the-reasoning.md` |
| The ordering is **stated**, not inferred | The document says, in terms: this verdict merged at commit *X* on *date*, and `publication-and-positioning` (HS-P0016) opens after it. It cites the initiative DAG edge as the authority and names both commits | `.bklg/from-contract-to-published-library/_decomposition.md:150`, `:185-186`; `…/ladybug-projection-store/project.md` DR-8, AC-011 |
| The plan-of-record discrepancy is **recorded, not repaired** | `RUNBOOK.md:163` gives phase 12 deps "7, 8" and `:188` draws `6 ─▶ 11` as never rejoining. The handover names this as a finding for HS-P0016 and for a re-plan, and changes neither | `RUNBOOK.md:163`, `:176-190`, `:243-245`; seam list `…/_decomposition.md:82`, `:67-68` |
| Handover item 1 — the build number | The cold and warm `lbug` build figures per matrix runner, each with toolchain and machine, **transcribed** from `cold-build-cost-and-ci-shape`'s record with its path and commit; plus the CI shape chosen (`--exclude` + dedicated job, or accepted in the three-OS matrix) and the failure mode its rationale addressed | `…/ladybug-projection-store/_decomposition.md:375-399`; `RUNBOOK.md:4420-4425`, `:4437`; `xtask/src/main.rs`, `.github/workflows/ci.yml` named as the files that carry the shape |
| Handover item 2 — the docs.rs failure, and what `publish = false`'s removal did to it | docs.rs cannot build `lbug` 0.19.1. With `publish = false` gone the crate is publishable-in-manifest, so the note is now a consequence: a published `happenstance-ladybug` would carry a red docs page against phase 12's proof artefact and DoD 10. The document also flags that `PUBLISHABLE` now carries a crate outside the three-crate working default, which is what `crate-set-decision` asserts against | `crates/happenstance-ladybug/src/lib.rs:24-32` (as rewritten by `real-lbug-driver-swap`); `…/_decomposition.md:173-183`, `:400-406`; `RUNBOOK.md:4481-4484`; `initiative.md:387-389`; `…/publication-and-positioning/_storymap.md:56` |
| Handover item 3 — the `[package.metadata.docs.rs]` question, left open | Stated as a question with its options and the constraint that `all-features` + `--cfg docsrs` configures rustdoc, not the C++ toolchain, so it cannot on its own fix a failing native build. Modelled on, and citing, `crates/happenstance-core/Cargo.toml:54-56`. **No answer, no block added to this crate's manifest** | `…/_decomposition.md:400-406`; `…/ladybug-projection-store/project.md:127-130`; receiving story `…/publication-and-positioning/_storymap.md:66` |
| Every handover item names its receiving story by slug | `crate-set-decision`, `guarantees-and-docs-rs-presentation`, `rendered-page-preflight`, `publish-0-2-0` — each verified to exist before it is cited | `.bklg/from-contract-to-published-library/publication-and-positioning/_storymap.md:56`, `:66`, `:67`, `:68`; item dirs under `…/publication-and-positioning/` |
| `RUNBOOK.md` phase 11 gains a handover block, and nothing else | Inside `:4411-4444`, after the exit criteria: the ordering statement and a relative link to the handover document. The exit-criteria boxes themselves are the slice-mate's to tick; this story ticks none and rewords none | `…/_decomposition.md:82`; `…/_storymap.md:118-121`; `RUNBOOK.md:4432-4444` |
| Nothing is re-derived | No number, finding, verdict phrase or PS disposition originates in this PR. Each is quoted or cited from the sibling record that owns it, with path and commit | `…/ladybug-projection-store/project.md` AC-007, AC-009; testing brief `…/_decomposition.md:477`, `:479`, `:481` |
| The gate stays green and nothing frozen moves | `cargo xtask ci` — the whole gate, not `--fast` — on the merge commit, including `cargo xtask spec-trace`; `spec/SPECIFICATION.md` untouched, so no citation can drift from this PR | `CLAUDE.md`, *Commands*; `…/_decomposition.md:517-539`; `…/ladybug-projection-store/project.md:210-213` |

## Data and migrations

**N/A.** This story adds no schema, no stored data, no runtime state and no code: it writes one markdown
evidence document and one markdown block in `RUNBOOK.md`. There is no database, no serialized format and
therefore nothing to migrate.

The one persistence-shaped rule that *does* apply is the evidence lifecycle, and it is a rule about
files rather than data: documents under `references/evaluation/` are dated, pinned to a commit and
**superseded rather than edited** (`references/evaluation/README.md:9-13`). If any statement in this
story's handover document is later found wrong, the repair is a new dated document that supersedes it —
not an in-place correction — on the same test the KB applies to a signed decision: ask whether the edit
changes what the document asserts (`.kb/governance/rewrite-the-referent-never-the-reasoning.md`).

## Acceptance criteria

Eight criteria. The observer is named in each, because this story's whole product is a statement
made *to* somebody. Three actors recur: the **backbone-E reviewer** — *"the reviewer who will be
asked to believe the answer"* (`…/_storymap.md:14-16`, activity **E**); the **HS-P0016
implementer**, who picks up `publication-and-positioning` and needs the residue of phase 11
without archaeology; and **Persona 4, the evaluator (pre-adoption)**, who gets one bounded look at
what was published and *"does not get a second pass"*
(`.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md:249-316`).
Every criterion crosses the full stack this story has — a sibling record → a transcription with
provenance → a merged, dated document → a pointer from the plan of record a reader actually opens.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **GIVEN** the backbone-E reviewer, who must believe that the freeze verdict was heard *before* `0.2.0` became irreversible rather than filed after it, **WHEN** they look for the ordering, **THEN** they find it asserted in a new dated document `references/evaluation/phase-11-publication-handover.md` that names the verdict's own commit SHA, carries its own ISO date, and states in terms that `publication-and-positioning` (HS-P0016) opens after it — citing the initiative DAG edge as the authority. The claim is made by a **third file**, never by an amendment to the sealed verdict, because a verdict that grew a forward obligation in a second commit is no longer the artefact whose value was that it was sealed | Review of `references/evaluation/phase-11-publication-handover.md` against `references/evaluation/README.md:9-13` (dated, pinned to a commit, superseded not edited); mechanically, `git log --oneline -- references/evaluation/` shows the verdict file introduced in one commit and never re-touched, and this document introduced in a later one; `git log --oneline` shows both ahead of the first commit touching `.bklg/from-contract-to-published-library/publication-and-positioning/**` outside planning |
| AC-002 | **GIVEN** a reader of the plan of record — someone who opens `RUNBOOK.md` to find out how far phase 11 got and what comes next, and who will *not* go spelunking in `.bklg/` — **WHEN** they reach the end of phase 11's body, **THEN** they meet a handover block that states the ordering to phase 12 and links `references/evaluation/phase-11-publication-handover.md` by relative path, so the edge is reachable from the document a reader opens next rather than only from the backlog. The block sits inside the licensed seam and touches nothing else in it: phase 11's exit-criteria boxes are the slice-mate's to tick, and this story ticks none and rewords none | `git diff --name-only` against the merge base lists `RUNBOOK.md` and `references/evaluation/phase-11-publication-handover.md` and nothing else outside this story's backlog folder; `git diff -- RUNBOOK.md` confirms every changed line falls within `RUNBOOK.md:4411-4444` and that no `- [ ]` / `- [x]` box changed state in this PR; the relative link resolves, checked by hand and by `cargo xtask ci`'s docs step staying green |
| AC-003 | **GIVEN** the HS-P0016 implementer deciding whether `publish-0-2-0`'s whole-gate run on the literal publish commit can afford `happenstance-ladybug`, **WHEN** they read handover item 1, **THEN** they get the *measured* cold and warm `lbug` build figures per matrix runner with toolchain and machine stamped, plus the CI shape that number bought and the files that carry it (`xtask/src/main.rs`, `.github/workflows/ci.yml`, `RUNBOOK.md`'s phase 11 body) — each transcribed from `cold-build-cost-and-ci-shape`'s record with that record's path and commit, so the number in the handover and the number in the record cannot diverge | Review that every figure in handover item 1 carries a `path:line` or commit citation to the AC-009 record; a diff of the transcribed figures against `RUNBOOK.md:4420-4425`, `:4437` and the AC-009 story's report, which must agree digit for digit; `git diff` shows this PR added no measurement of its own and touched no CI or `xtask` file |
| AC-004 | **GIVEN** Persona 4 landing on a registry page during their one bounded look, **WHEN** HS-P0016 decides what `0.2.0` ships, **THEN** they have already been told — in handover item 2 — that docs.rs cannot build `lbug` 0.19.1, and that with `publish = false` removed this is no longer a note but a **consequence**: a published `happenstance-ladybug` would carry a red docs page against phase 12's own proof artefact, on a release that cannot be edited afterwards. The item also flags that `xtask/src/package.rs`'s `PUBLISHABLE` now carries a crate outside HS-P0016's three-crate working default, so `crate-set-decision` meets that fact by name rather than by a reconciliation failure inside its own PR | Review of handover item 2 against `crates/happenstance-ladybug/src/lib.rs`'s driver-absent section **as rewritten by `real-lbug-driver-swap`** (cited at the range it actually occupies at this commit, not the pre-swap `:24-32`), `xtask/src/package.rs`'s `PUBLISHABLE` set, and `crates/happenstance-ladybug/Cargo.toml` with `publish = false` gone; `git diff` shows this PR modified none of those three files |
| AC-005 | **GIVEN** the HS-P0016 implementer of `guarantees-and-docs-rs-presentation`, who owns adding a `[package.metadata.docs.rs]` block, **WHEN** they read handover item 3, **THEN** they receive a **question with its options and its constraint**, not an answer: modelled on and citing `crates/happenstance-core/Cargo.toml:54-56`, and stating that `all-features = true` plus `rustdoc-args = ["--cfg", "docsrs"]` configures *rustdoc* and not the `cxx` / `cmake` native toolchain, so the block cannot by itself turn a failing native build green. No answer is given and no block is added to `crates/happenstance-ladybug/Cargo.toml`, because whether this crate ships at `0.2.0` is a decision this project explicitly does not hold | Review that handover item 3 is phrased as an open question and states the rustdoc-not-toolchain constraint; `git diff -- crates/happenstance-ladybug/Cargo.toml` is empty in this PR; the item cites `crates/happenstance-core/Cargo.toml:54-56` and `…/ladybug-projection-store/project.md:127-130` |
| AC-006 | **GIVEN** that an obligation handed to a *project* is an obligation handed to nobody, **WHEN** the HS-P0016 implementer opens whichever story they are actually working, **THEN** each handover item names its receiving story **by slug** — `crate-set-decision`, `guarantees-and-docs-rs-presentation`, `rendered-page-preflight`, `publish-0-2-0` — and every slug cited was verified to exist before it was written down, so a typo cannot silently route an obligation into a void | Each cited slug resolves to a real directory under `.bklg/from-contract-to-published-library/publication-and-positioning/` **and** to a row at that project's `_storymap.md:56`, `:66`, `:67`, `:68`; the reviewer re-runs the directory listing rather than trusting the prose |
| AC-007 | **GIVEN** a reader who trusts `RUNBOOK.md`'s own graph — which today gives phase 12 the dependencies "7, 8" and draws phase 11 as one of three that *"never rejoin"* — **WHEN** they read the handover document, **THEN** the discrepancy is **recorded as a finding** for HS-P0016 and for a re-plan, with the exact locations, and is **not repaired here**: phase 12's dependency row, the phase graph and phase 12's body are outside this project's licensed seam, and no `.bklg/**` frontmatter (including `blocks` / `blocked_by`) is touched, because the CLI is the only writer of it | The handover document contains a finding naming `RUNBOOK.md:163`, `:176-190` and `:243-245`; `git diff -- RUNBOOK.md` shows no change outside `:4411-4444`; `git diff --name-only` contains no `.bklg/**` path other than this story's own folder and no frontmatter hunk in any item; `redkiln doctor` stays clean with exactly the six expected `template-drift` advisories |
| AC-008 | **GIVEN** the backbone-E reviewer asking the one question that would void the whole artefact — *"did this document invent anything?"* — **WHEN** they audit the diff, **THEN** every number, finding, verdict phrase and PS disposition in it is quoted or cited from the sibling record that owns it with path and commit, and none originates here; **AND** the tree is still green on the whole gate, with `spec/SPECIFICATION.md` untouched so no `[FROZEN]` clause text, marker or citation could have moved | `cargo xtask ci` green on the merge commit — the whole gate, not `--fast` — including `cargo xtask spec-trace`; `git diff -- spec/SPECIFICATION.md` empty; `git diff --name-only` contains no path under `.kb/`, `crates/`, `xtask/` or any `Cargo.toml`; `redkiln validate --kb` clean; reviewer confirms every assertion in the handover document carries an outbound citation |

**Coverage of project AC-011.** AC-001 is the ordering claim itself and AC-002 is the place a
reader meets it; AC-011's second half — *"the design brief states that ordering explicitly rather
than leaving it to the DAG"* (`…/ladybug-projection-store/project.md:222-224`) — is discharged by
AC-001 and AC-002 together (see *Clarifications*, item 2). AC-003 through AC-006 are the handover
half the architecture brief attaches to this story (`…/_decomposition.md:394-406`). AC-007 and
AC-008 are the boundary: what this story must *not* do, made checkable.

## Interaction quality

**This story renders no surface, and that determination is what was signed off.** `_design.md`
records `N/A — no user-facing surface` in `## Surfaces` and in every block beneath it, approved by
the repository owner on 2026-08-12 at the `/redkiln:plan` design sign-off gate, with
`design.capture` a **declared** skip rather than a silent pass
(`…/ladybug-projection-store/_design.md:40-43`, `:92-98`; `CLAUDE.md`, *Where the work lives*).
There is therefore no composition, transience, density budget or hierarchy in the UI sense, and
none is invented here.

The invariants are not vacuous, though, because this story's medium is a *document* and a document
has the same failure modes one level over. The mapping below records that, so the silence is a
mapping rather than an omission. **Every invariant that applies is carried by an AC row in the
table above** — this section only says which row carries it and how it is checked. Nothing here is
a floating bullet, because a bullet in this section gets no ledger row, is never gated and is never
tested.

**STATE family.**

| invariant | its form here | carried by | verified by |
| --- | --- | --- | --- |
| In-place, not context-jump | The ordering is met where the reader already is — the end of phase 11 in the plan of record — not by requiring a jump into `.bklg/` to reconstruct a DAG edge | AC-002 | `git diff -- RUNBOOK.md` confined to `:4411-4444`; the relative link resolves |
| Non-occlusion | Nothing in the handover hides behind a summary: each item carries the path and commit of the record it came from, so the reader can always reach the primary source in one hop | AC-003, AC-008 | every figure and finding carries an outbound citation, checked in review |
| Preserved state | The sealed artefacts stay sealed — the verdict and the axes document are cited, never edited, never re-opened — and the slice-mate's exit-criteria boxes keep their state | AC-001, AC-002 | `git log` shows the verdict file untouched after its introducing commit; no box changed state |
| Reversibility | If a statement here is later found wrong, the repair is a **new dated document that supersedes it**, never an in-place correction — the same test the KB applies to a signed decision | AC-001 | `references/evaluation/README.md:9-13`; `.kb/governance/rewrite-the-referent-never-the-reasoning.md` |
| Keyboard reachability | The library analogue is *reachability without privileged context*: a reader with only the repository — no session history, no backlog tooling — gets from `RUNBOOK.md` to the handover document to each cited record | AC-002, AC-006 | relative link resolves; every cited story slug resolves to a real directory |

**COMPOSITION family — `N/A` by sign-off, with one analogue that is not optional.** *"Presentation
exists at all"* has a real form for an evidence document, and it is the whole of this story: a
handover that says only *"phase 11 hands phase 12 some open questions"* is the bare-markup failure
mode — it satisfies every structural check and transfers nothing. The composed form is what AC-003,
AC-004 and AC-005 assert: a **number** with its toolchain and machine, a **named failure** with what
`publish = false`'s removal did to it, and a **question** with its options and its constraint — each
addressed to a named receiving story (AC-006). Placement, transience, density budget and hierarchy
have no analogue in a three-item handover and are not invented. The design's named anti-patterns are
`N/A`; this story's operative anti-patterns are the four the PR boundary names — answering the
docs.rs question, re-deriving the number, amending the sealed verdict, repairing the RUNBOOK graph —
and all four are carried as AC rows (AC-005, AC-003 with AC-008, AC-001, AC-007) rather than as prose.

## Error conditions

| id | condition | required behaviour | evidence |
| --- | --- | --- | --- |
| EC-001 | The slice-mate `freeze-verdict-document` has not merged when this story is picked up | **Halt loudly.** This story cannot pin a commit SHA that does not exist and must not invent a placeholder, a "TBD", or a claim about a verdict it has not read. The slice merges in a stated order and this story is second | `…/ladybug-projection-store/_storymap.md:139-143`; story `blocked_by: HS-S0082` |
| EC-002 | `cold-build-cost-and-ci-shape` recorded no number, or recorded it in a form this story cannot cite by path and commit | The handover **states the gap as a gap** and names AC-009 as its owner. It never estimates, never re-runs the build to fill the hole, and never writes a figure whose provenance it cannot give — a second number that can disagree with the first is how an evidence corpus starts contradicting itself | `…/ladybug-projection-store/_decomposition.md:394-399`; project AC-009 |
| EC-003 | The verdict says *"did not hold"* | Nothing about this story changes. Backbone activity **E** is unconditional by construction, so the handover is unconditional too; it *gains* urgency, not scope. The ordering claim is the same claim, and routing a "did not hold" to a decision atom and a re-plan stays `freeze-verdict-document`'s (AC-008). Do not re-argue the verdict in the handover | `…/ladybug-projection-store/_storymap.md:26-30`; `…/ladybug-projection-store/project.md`, AC-007, AC-008 |
| EC-004 | A cited receiving story slug does not resolve — renamed, dropped, or mistyped | Fix the citation against `…/publication-and-positioning/_storymap.md` before merge. If the receiving story genuinely does not exist, name the **project** *and* say so explicitly as a gap for a re-plan; never leave a slug that looks resolvable and is not, because that failure is silent at exactly the moment it matters | `…/publication-and-positioning/_storymap.md:56`, `:66`, `:67`, `:68`; item directories under `…/publication-and-positioning/` |
| EC-005 | The temptation to repair `RUNBOOK.md:163` or `:176-190` while the file is open | **Out of bounds in both directions.** The seam this project may edit stops at `:4444`, and *"nothing outside this list is this project's to edit"*. The discrepancy is a recorded finding (AC-007). A repair here would also silently contradict phase 12's own stated reasoning that a dependency row can carry ordering intent rather than technical necessity, without a re-plan having decided it | `…/ladybug-projection-store/_decomposition.md:67-68`, `:82`; `RUNBOOK.md:4526-4532` |
| EC-006 | An incidental defect is found while transcribing — a wrong figure in the AC-009 record, a stale citation in the verdict | Record it and route it; do not repair it inside a sealed artefact. A wrong figure in merged evidence is superseded by a new dated document, not edited; a defect outside this project's boundary goes to the `support` initiative | `references/evaluation/README.md:9-13`; `.redkiln/config.yaml:5` |
| EC-007 | `publication-and-positioning` has already opened its gate by the time this story is ready | The ordering claim **cannot be made true retroactively** and must not be written as though it were. State what actually happened, with commits and dates, and raise the inversion as a finding against initiative exit criterion 4 — that inversion is precisely what DR-8 exists to make visible rather than to hide | `…/ladybug-projection-store/project.md:166-170`, `:295-298`; `initiative.md:571-572` |

## Non-functional

| id | requirement | why it binds here |
| --- | --- | --- |
| NF-001 | **Zero code, zero dependency, zero gate-time cost.** Two markdown files change. No `Cargo.toml`, no `xtask/**` step array, no CI job, no new tool | The gate must stay a clean signal for the slice-mate's verdict; a story whose merge changes the gate's own shape cannot also be the one certifying the gate was green (`…/ladybug-projection-store/_decomposition.md`, *Merge-gate commands*) |
| NF-002 | **The whole gate, not `--fast`.** `cargo xtask ci` on the merge commit, per this project's stated bar — a subset already met by every sibling proves nothing new, and this project's dependency graph is the one place wasm32, `cargo-hack`, package-completeness and `spec-trace` meet `lbug` | `CLAUDE.md`, *Commands*; `…/ladybug-projection-store/_decomposition.md`, *Merge-gate commands* |
| NF-003 | **Durable without session context.** The handover must be readable in full by someone who was not in the room: no "as discussed", no unresolved pronoun, no reference to a chat. Every claim carries a path, and every path was checked to resolve | `references/evaluation/README.md`'s standard for kept evidence; DR-9's re-takeable-snapshot discipline (`…/ladybug-projection-store/project.md:171-174`) |
| NF-004 | **Immutability posture stated in the document itself.** The handover names its own supersession rule, so a future reader knows the repair path before they need it | `references/evaluation/README.md:9-13`; `.kb/governance/rewrite-the-referent-never-the-reasoning.md` |
| NF-005 | **Genre consistency.** Written in the register of the existing reconciliation documents — dated heading, commit pin, numbered findings with citations — so it reads as the same kind of artefact rather than a memo that happens to live in the same folder | `references/evaluation/phase-4-reconciliation.md`; `references/evaluation/phase-4-5-reconciliation.md` |
| NF-006 | **No frontmatter, anywhere.** Neither file this PR writes carries Redkiln or KB frontmatter, and no `.bklg/**` item frontmatter is edited: `references/evaluation/` is not a KB tree, and this story mints no atom | `CLAUDE.md`, *Where the work lives* — the CLI is the only writer, and a `PreToolUse` hook denies the edit |

## Implementation notes (non-prescriptive)

Not instructions — the few places this is easy to get subtly wrong.

**Write the document last, and write it from the tree.** Every input it transcribes already exists
in the repository when it is written: the verdict's SHA from `git log`, the build figures from the
AC-009 record and `RUNBOOK.md`'s phase 11 body, the docs.rs note from
`crates/happenstance-ladybug/src/lib.rs` as `real-lbug-driver-swap` left it. Open each and copy
from it. Nothing here should be written from memory of a sibling story's output, because the whole
value of the artefact is that its statements are re-checkable against the things they cite.

**Pin the verdict's SHA, not this PR's.** The ordering claim is about *the verdict's* commit
relative to publication's first commit. This document's own commit is later by construction and is
not the interesting one. A path alone cannot make an ordering claim.

**Re-derive the `lib.rs` line range before citing it.** The front half deliberately says to cite it
*where it lands*: `:24-32` is the pre-swap range and will almost certainly have moved. Re-grep for
the docs.rs sentence and cite the range it occupies at this commit. A citation that was true two
stories ago is exactly the drift `spec-trace` catches for `SPECIFICATION.md` and nothing catches
here.

**The RUNBOOK block wants to be short.** Its job is to make the edge *reachable*, not to duplicate
the handover. Three or four lines: phase 12 opens after this verdict, here is the handover document,
here are the three things it carries. Duplicating the content creates a second copy that can drift
from the first — the same failure the transcription discipline exists to avoid one level down.

**Keep the recorded discrepancy factual and non-prescriptive.** Say where `RUNBOOK.md` disagrees,
say that the initiative's DAG is the authority for this initiative's scheduling, and stop.
Recommending an edit to phase 12's dependency row is not this story's to recommend; that is a
re-plan's call, and phase 12's own text already argues a dependency row can encode ordering intent
rather than technical necessity (`RUNBOOK.md:4526-4532`), which a hasty repair could contradict.

**Verify the four receiving slugs by listing the directory, not by grepping prose.** A slug in a
story-map row is a claim; a directory that exists is a fact.

## Tests and CI (merge gate)

The testing brief classifies project AC-011 as **process**, proven by commit order, and says plainly
that it is *"not a CI-enforced check (no such gate exists in `.redkiln/config.yaml`) — a
human-checked ordering claim"* (`…/ladybug-projection-store/_decomposition.md:481`). That is not a
licence for the gate to be optional: the gate proves *nothing was broken*, the human checks prove
*the claim*, and both are required.

| tier | command / path | proves |
| --- | --- | --- |
| Process — ordering | `git log --oneline` over `references/evaluation/`, this PR's commits, and the first commit touching `.bklg/from-contract-to-published-library/publication-and-positioning/**` outside planning | AC-001. The verdict's commit and this one both precede publication's gate opening — the literal form of project AC-011 (`…/_decomposition.md:481`) |
| Process — immutability | `git log --oneline -- references/evaluation/<verdict>.md` shows exactly one commit; this handover is a separate, later file | AC-001. The verdict was sealed and stayed sealed (`references/evaluation/README.md:9-13`) |
| Static — diff boundary | `git diff --name-only <merge-base>..HEAD` | AC-002, AC-007, AC-008. Exactly `RUNBOOK.md`, `references/evaluation/phase-11-publication-handover.md` and this story's backlog folder — nothing under `crates/`, `xtask/`, `spec/` or `.kb/`, and no other `.bklg/**` path |
| Static — seam | `git diff -- RUNBOOK.md`, read with line numbers | AC-002, AC-007. Every changed line inside `RUNBOOK.md:4411-4444`; no exit-criteria box changed state; `:163`, `:176-190` and phase 12's body untouched |
| Static — frozen surface | `git diff -- spec/SPECIFICATION.md` empty; `cargo xtask spec-trace` | AC-008. No clause text, maturity marker or citation moved (`…/ladybug-projection-store/project.md:210-213`) |
| Static — backlog and KB health | `redkiln validate --kb && redkiln doctor` | AC-007. No frontmatter hand-edited, no accepted atom body changed, and the six expected `template-drift` advisories are still exactly six (`CLAUDE.md`, *Where the work lives*) |
| Integration — the whole gate | `cargo xtask ci` on the merge commit | AC-008, NF-002. fmt, clippy `-D warnings`, tests, four wasm32 steps, docs, `spec-trace`, the `--no-default-features` doc build and `cargo package --list`, plus `cargo-hack` and `cargo-deny`, which resolve on this machine and therefore run rather than skip (`CLAUDE.md`, *Commands*) |
| Integration — story grain | `cargo xtask affected --base main` | NF-001. The affected set contains no Rust package, which is itself the evidence this story shipped no code (`CLAUDE.md`, *Commands*; `.redkiln/config.yaml`'s `verify:` block) |
| Review — transcription audit | Read `references/evaluation/phase-11-publication-handover.md` against the AC-009 record, the verdict, `xtask/src/package.rs` and `crates/happenstance-core/Cargo.toml:54-56` | AC-003, AC-004, AC-005, AC-008. Every figure and finding matches its source digit for digit and carries a path plus commit; handover item 3 is a question, not an answer |
| Review — slug resolution | `ls .bklg/from-contract-to-published-library/publication-and-positioning/` plus the rows at that project's `_storymap.md:56`, `:66`, `:67`, `:68` | AC-006. Each of the four cited receiving stories exists |

`cargo xtask ci --fast` is explicitly **not** this story's bar, for the reason its own project gives:
a subset already met by every sibling proves nothing new.

## Risks and coupling (PR-scoped)

| risk | how it shows up | containment |
| --- | --- | --- |
| **The ordering is asserted and then inverted anyway.** Schedule pressure opens HS-P0016 early and the document's claim quietly becomes false | Nobody notices, because no CI step checks it — the failure DR-8 names, and the one that breaks initiative exit criterion 4 | Checked by a human at *this* story's merge (AC-001) and again at project closeout, and stated in `RUNBOOK.md` where a phase-12 reader meets it (AC-002). EC-007 says what to do if it has already happened: record it, do not launder it |
| **Transcription drift.** A figure or finding is copied slightly wrong and the handover now disagrees with the record it claims to quote | Two numbers for one measurement; a downstream decision made on the wrong one | Every transcribed item carries the path and commit of its source (AC-003, AC-008), so a disagreement is discoverable in one hop rather than being an unattributed claim |
| **Citation rot in `crates/happenstance-ladybug/src/lib.rs`.** `real-lbug-driver-swap` rewrites the cited section and a stale `:24-32` survives into merged evidence | A reader follows the citation and lands on unrelated text; nothing in the gate catches it, because `spec-trace` covers `SPECIFICATION.md` only | Re-grep and re-cite at merge time (*Implementation notes*); AC-004's verification names the check explicitly |
| **Scope creep into HS-P0016's decisions.** Answering the docs.rs question, or nominating a crate set, because both feel one sentence away | This project takes a decision it does not hold, and HS-P0016 inherits an answer instead of a question | AC-005 and the PR boundary make "left open" a checkable property of the artefact rather than an intention |
| **Repairing the RUNBOOK graph in passing** | An edit outside the licensed seam that no gate rejects, contradicting phase 12's own reasoning about dependency rows | EC-005 plus AC-007's `git diff` check bound the diff to `:4411-4444` |
| **Slice-mate collision on `RUNBOOK.md`.** Both stories in slice `freeze-verdict` touch phase 11's body from one context | A merge conflict, or worse, one story silently reverting the other's ticked box | De-confliction is explicit in the *Integration contract*: the slice-mate ticks the exit-criteria boxes, this story appends a block after them, and the slice merges in stated order (`…/_storymap.md:118-121`, `:139-143`) |
| **The coupling must stay one-directional.** `publication-and-positioning` reads this project; this project must not start depending on HS-P0016's decisions | A circular dependency between two projects, discovered when neither can merge first | This story writes only outbound statements. `projection-port-ship-shape` (`…/publication-and-positioning/_storymap.md:57`) already reads this project's report for PS-3 and needs nothing new from here |

## Dependencies

**Blocks on**

- **`freeze-verdict-document`** (HS-S0082, slice-mate, merged first) — it writes the verdict this
  story pins a SHA to and hands over. Without it there is nothing to order and nothing to cite;
  EC-001 says halt rather than placeholder. It also ticks phase 11's remaining exit-criteria boxes,
  which is why this story appends *after* them and edits none
  (`…/ladybug-projection-store/_storymap.md:118-121`, `:139-143`).

Transitively, through the slice-mate: `cold-build-cost-and-ci-shape` (the measured number, AC-009),
`package-completeness-and-name-claim` (the `publish = false` removal that turns the docs.rs note
into a consequence, AC-010) and `read-your-own-writes-projection`. This story cites their outputs; it
does not depend on them directly, because the slice-mate already does (`…/_storymap.md`, *Merge
order*, items 4–5).

**Unlocks**

- **`publication-and-positioning` (HS-P0016)** as a whole — this is the last story of the last slice
  of `ladybug-projection-store`, and the project `blocks` publication in the initiative DAG
  (`.bklg/from-contract-to-published-library/_decomposition.md:150`, merge order `:185-186`). Named
  receiving stories: `crate-set-decision` (the published set and the `PUBLISHABLE` reconciliation),
  `guarantees-and-docs-rs-presentation` (the `[package.metadata.docs.rs]` block),
  `rendered-page-preflight` (reading the rendered page before the irreversible act) and
  `publish-0-2-0` (the whole-gate run on the literal publish commit).
- **This project's closeout** — initiative DoD 8's *merged, and merged in time* half, which is
  initiative exit criterion 4 (`initiative.md:571-572`).

No story inside `ladybug-projection-store` blocks on this one; it is terminal for the project.

## Anchors (progressive disclosure)

Everything above is sufficient to start. These are the deeper artefacts — open each at the moment
named, and link rather than paste.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `references/evaluation/README.md` | The lifecycle the new document must obey — immutable, dated, pinned to a commit, superseded rather than edited. It is *why* the handover is a third file instead of a section appended to the verdict | Before writing the document's heading, and again before any later correction | AC-001 |
| `references/evaluation/phase-4-reconciliation.md` | The genre model: how a reconciliation document in this repository states a claim, pins a commit and numbers its findings. Match its register, not its content | While drafting, to fix the shape before the words | AC-001 |
| `.kb/governance/rewrite-the-referent-never-the-reasoning.md` | The governing test for whether an edit is permissible — ask whether it changes what the document asserts. Adding a forward obligation to a sealed verdict changes exactly that | If tempted to amend the verdict instead of writing a new file, and before any in-place correction | AC-001 |
| `RUNBOOK.md` | Carries both the licensed seam (`:4411-4444`) this story writes into and the discrepancy it records (`:163`, `:176-190`, `:243-245`), plus phase 12's own argument that a dependency row can encode ordering intent (`:4526-4532`) | Immediately before editing phase 11's body, and again when wording the recorded finding | AC-002, AC-007 |
| `.bklg/from-contract-to-published-library/ladybug-projection-store/_decomposition.md` | Architecture §7's three handover items (`:394-406`), M9's evidence-document lifecycle (`:211-218`), M5's packaging consequence (`:173-183`), the seam list and the *"nothing outside this list"* rule (`:67-68`, `:82`), and the testing brief's AC-011 row (`:481`) | Before drafting each handover item, and before touching any file, to confirm it is inside the seam | AC-003, AC-004, AC-005, AC-007 |
| `.bklg/from-contract-to-published-library/_decomposition.md` | The initiative DAG edge `ladybug-projection-store` → `publication-and-positioning` (`:150`) and the merge order placing ladybug at 6 and publication at 7 (`:185-186`) — the authority the ordering claim cites | While writing the ordering statement, to cite the edge rather than assert it | AC-001 |
| `.bklg/from-contract-to-published-library/ladybug-projection-store/project.md` | AC-011's exact wording (`:222-224`), DR-8 (`:166-170`), the one-directional-and-timed coupling note (`:295-298`), and the out-of-scope line keeping the `0.2.0` decision with HS-P0016 (`:127-130`) | Before writing the ordering claim, and before deciding whether any item may be answered here | AC-001, AC-005, AC-007 |
| `.bklg/from-contract-to-published-library/publication-and-positioning/_storymap.md` | The four receiving story rows — `crate-set-decision` (`:56`), `guarantees-and-docs-rs-presentation` (`:66`), `rendered-page-preflight` (`:67`), `publish-0-2-0` (`:68`) — including the `PUBLISHABLE`-reconciliation assertion AC-004's flag lands on | When routing each handover item to a receiving story, and to verify each slug resolves | AC-006, AC-004 |
| `.bklg/from-contract-to-published-library/publication-and-positioning/project.md` | HS-P0016's charter lists this project as an input it *reads* (`:327-328`), confirming the coupling is one-directional and that a handover is what that project expects to find | Once, when checking that nothing here creates a reverse dependency | AC-006 |
| `crates/happenstance-core/Cargo.toml` | The `[package.metadata.docs.rs]` block at `:54-56` that handover item 3 is modelled on and cites, and the source of the constraint that it configures rustdoc rather than the native toolchain | While writing handover item 3 | AC-005 |
| `crates/happenstance-ladybug/src/lib.rs` | Carries the docs.rs / `lbug` 0.19.1 build-failure statement. `real-lbug-driver-swap` rewrites this section, so the line range must be re-derived at this commit rather than copied from the front half | Immediately before citing the docs.rs failure — re-grep, then cite | AC-004 |
| `xtask/src/package.rs` | The `PUBLISHABLE` set. After `package-completeness-and-name-claim` it carries `happenstance-ladybug`, which is the fact that lands on `crate-set-decision` | While writing handover item 2's `PUBLISHABLE` flag — read the set, do not assume it | AC-004 |
| `.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md` | Persona 4, the evaluator (`:249-316`): the reader who gets one bounded look and *"does not get a second pass"*. The reason a red docs page and a late verdict are user-facing harms rather than bookkeeping | When framing why the handover items matter, if the reasoning starts to feel procedural | AC-001, AC-004 |
| `.bklg/from-contract-to-published-library/ladybug-projection-store/_design.md` | The signed-off `N/A — no user-facing surface` determination (`:40-43`) and its approval (`:92-98`) — binding, and the reason no surface may be invented here | Once, if any part of this story starts to look like it renders something | AC-002 |
| `.bklg/from-contract-to-published-library/ladybug-projection-store/_storymap.md` | Slice 5's stated merge order (`:139-143`), the slice-mate de-confliction (`:118-121`), and backbone activity E's unconditionality (`:26-30`) | Before starting, to confirm the slice-mate has merged; and again if the verdict says *"did not hold"* | AC-002, AC-007 |
| `.bklg/from-contract-to-published-library/initiative.md` | DoD 8 (`:380-382`) and exit criterion 4 (`:571-572`) — the initiative-level obligations this story closes — plus the framing that the journeys live in the discovery distillation (`:227-240`) | Once, when writing the document's opening statement of what it discharges | AC-001 |
| `.github/workflows/ci.yml` | Together with `xtask/src/main.rs`, the file that carries whatever CI shape AC-009 chose — the thing handover item 1 must name accurately and must not change | While transcribing handover item 1's CI-shape sentence | AC-003 |

## Clarifications resolved during spec

1. **The eight AC ids are exactly those the front half decided** — AC-001 through AC-008. None added,
   none dropped. The behaviour table's ten rows collapse to eight because "a new document exists" and
   "the ordering is stated" are one criterion (AC-001: a claim with no artefact is nothing, and an
   artefact with no claim is a memo), and "nothing is re-derived" and "the gate stays green" are one
   criterion (AC-008: both are audits of the diff rather than of the prose).

2. **Project AC-011 says *"the design brief states that ordering explicitly"*.** `_design.md` records
   no surface, so there is no design-brief prose to carry it. Resolved in favour of the *intent* —
   the ordering is stated explicitly rather than inferred from the DAG — discharged by AC-001 (in the
   handover document) and AC-002 (in the plan of record). The brief is not retro-fitted with a
   surface it does not have, and the sign-off is not re-opened.

3. **The verification tier for a process AC.** The testing brief is explicit that AC-011 is not
   CI-enforced (`…/_decomposition.md:481`), so this spec's `verification` column and the ledger's
   `verifying_test` entries name real commands and real artefact paths — `git log`,
   `git diff --name-only`, `cargo xtask ci`, `redkiln validate --kb` — rather than test-function ids
   that would not exist. Inventing a `#[test]` name to satisfy a column shape would be the
   decorative-rule failure `CLAUDE.md` warns about, one level up from conformance.

4. **`crates/happenstance-ladybug/src/lib.rs:24-32` is deliberately treated as movable.** The front
   half flags it; this half turns it into a verification obligation (AC-004) and an anchor
   instruction, because a merged evidence document with a stale citation is exactly the drift this
   repository built `spec-trace` to prevent everywhere it could.

5. **What happens if the ordering has already been inverted** was unstated, and is now EC-007: the
   claim is not written as if it were true. Recording an inversion is what makes DR-8 an instrument
   rather than a wish.

6. **No `.kb/` atom is minted here.** The RUNBOOK discrepancy, and any *"did not hold"* routing, are
   findings recorded in evidence and routed to their owners; atoms are authored by
   `/redkiln:kb-ingest` from `.kb/_intake/`, never by hand (`CLAUDE.md`, *Where the work lives*).
