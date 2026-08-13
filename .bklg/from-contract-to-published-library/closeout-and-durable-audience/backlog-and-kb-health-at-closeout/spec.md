---
item: HS-S0133
stage: spec
created: 2026-08-12T13:48:12.563Z
updated: 2026-08-12T13:48:12.563Z
template_sig: 87bbf1d0
rendered_sig: bc66b5c9
---

# Spec — Backlog and knowledge base clean, with exactly six drift advisories

## Scope lock

| What | Path |
| ---- | ---- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` — DoD 14 and DoD 16 (`:398`, `:405`), exit criterion 7 (`:578-579`, "`redkiln validate --kb` and `redkiln doctor` are clean") |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` — the traceability matrix and the DAG that puts this project at rank 6 |
| Project | `.bklg/from-contract-to-published-library/closeout-and-durable-audience/project.md` — AC-011 (`:229-232`), AC-012 (`:233-236`), DR-11 (`:175-178`), DR-12 (`:179-183`), DoD 5 (`:259-261`) |
| This spec | `.bklg/from-contract-to-published-library/closeout-and-durable-audience/backlog-and-kb-health-at-closeout/spec.md` |
| Key briefs | `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_decomposition.md` — the one warranted `testing` brief: the AC-011 row (`:54`), the AC-012 row (`:55`), *Merge-gate commands* (`:93-107`), *Fixtures / seams* (`:109-123`); `_grounding.md:67-78` (the six-file assertion read off CI directly); `_design.md` (**no public API surface**, approved 2026-08-12) |
| Story map row | `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_storymap.md:61` (this row), `:85-88` (why health is measured last), `:24-30` (where the evidence lands), `:152-155` (merge order inside this slice) |
| This story's discovery | `.bklg/from-contract-to-published-library/closeout-and-durable-audience/backlog-and-kb-health-at-closeout/discover.md` — the signal ledger, the two answered questions, and *The wrong implementation* |
| Roadmap pointer | `RUNBOOK.md` — the plan of record; this story adds nothing to it and amends nothing in it |

## One-line PR slice

Run `redkiln validate`, `redkiln validate --kb` and `doctor --json` on the closeout tree, assert
zero problems, zero `process-drift` and a `template-drift` set equal to the six CI names, and
disposition any seventh or missing sixth with a decision — without ever running
`adopt --templates`.

## Executive summary

This PR lands **one measurement and no repair**: the three redkiln health commands run against a
pinned, residue-free checkout of the closeout tree, their raw output captured as files, the
three-part JSON assertion from CI evaluated against that output, and the verdict mounted in the
*Backlog and KB health* section of `_closeout-record.md`.

Delta against what the project already says. `project.md` DR-11 (`:175-178`) and the `testing`
brief's AC-011 row (`_decomposition.md:54`) both state the requirement — three commands, zero
`problems`, zero `process-drift`, a `template-drift` file list *equal* to the six at
`.github/workflows/ci.yml:180-185` — and the brief adds "re-run locally on the closeout tree, not
inferred from CI's last run". Neither says how a *local* run is made comparable to CI's, and that
turns out to be the whole difficulty. Three things are decided here rather than discovered during
implementation:

1. **The advisory set is a property of the CLI, not of this repository.** `ci.yml:120-125` says so
   in its own words — drift is computed "by comparing this repository's pinned `.redkiln/` against
   the BUNDLED files of whichever CLI runs" — and CI pins that CLI to `v0.18.0` (`:130`). A local
   run at any other version measures a different thing and can produce a different six. So the CLI
   version is captured as evidence beside the report (*Context pack* 2).
2. **Half of `validate --kb` goes vacuous in a pristine checkout.** `CLAUDE.md:87-91` describes the
   accepted-decision immutability check as comparing each atom against `HEAD`; in a clean clone the
   working tree *is* `HEAD`, so that half cannot fail there. The non-vacuous complement — a
   commit-range check over `.kb/decisions/` — is added here rather than left as a green step that
   proves nothing (*Context pack* 5).
3. **"`adopt --templates` was not run" is made falsifiable.** An assertion that a command was not
   typed is not evidence. What *is* evidence is that the six template files carry no modifying
   commit across the initiative's range, which is precisely the footprint running it would have
   left (*Context pack* 6).

The measured tree is also stated as a scope, not a claim about all time: three artefacts in this
project land *after* the SHA this story measures — this story's own `_ledger.md` and the two
slice-mates' folders — and the record says so and names the standing re-check rather than implying
the health verdict covers files that did not exist when it was taken (*Context pack* 7).

## Context pack

Everything below is a decision this story must honour. It is complete enough to start from; the
deeper artefacts sit behind the anchors and are opened only when a row here points at them.

**1. The check is a three-part assertion over parsed JSON, never an exit code.** This is the story's
central discipline and its named wrong implementation (`discover.md` *The wrong implementation*).
`ci.yml:147-149` records why in the repository's own voice: `doctor` "exits non-zero on a problem
and zero on a warning, so its exit code is captured rather than trusted". A green exit is
compatible with a seventh `template-drift` warning, with a `process-drift` warning, and with every
other warning kind. The three claims are: `(.problems | length) == 0`, zero warnings of kind
`process-drift`, and the sorted list of `template-drift` `.file` values **equal** to the six named
files — equality, not a superset, because an extra entry is a template someone changed without
deciding to and a missing entry is a customisation that was reverted (`ci.yml:160-171`;
`CLAUDE.md:116-121`). The exit code is *also* recorded, separately, because a disagreement between
it and the JSON verdict is itself a finding worth routing.

**2. The assertion runs at the CLI version CI pins, and the version is evidence.** `ci.yml:120-125`
is explicit that `doctor` compares this repository's pinned `.redkiln/` against the **bundled**
files of whichever CLI executes, so the expected drift set moves when redkiln releases and the
pin is what holds CI's assertion still. CI runs `v0.18.0` (`:130`). A local run must therefore
record the version it used (`redkiln --version`) alongside the report; if it is not `v0.18.0`, the
run has measured a different bundle than CI's assertion was written against and that fact belongs
in the record before the verdict does — it is not silently equivalent. It also means a *fixed* six
is not a law of nature: a redkiln release that alters a bundled template changes the expected set,
and the correct response is a decision (*Context pack* 6), never an `adopt`.

**3. The `jq` expression is copied byte-for-byte from CI, not paraphrased.** The local check and
CI's check must be capable of disagreeing only about the *tree*, never about the *question*. The
expression at `ci.yml:176-187` is transcribed verbatim — including the six literal paths in their
existing order — into this story's evidence, and `jq` resolves on this machine (`jq-1.8.2`), so
there is no reason to reach for an equivalent-looking rewrite. A rephrased assertion that happens
to pass is indistinguishable from the CI assertion until the day it is not.

**4. The run happens in a clean checkout, not in the working tree these artefacts are written in.**
`_decomposition.md:105-107` binds all four merge-gate commands to "the clean-checkout tree described
in DR-1, not against the working tree these planning artefacts are written in", and `project.md`
DR-1 (`:129-131`) says a warm working tree does not satisfy it. The slice-1 story
`clean-checkout-harness` owns that procedure and its B-7 states the consequence directly: the
checkout is evidence about **one** commit and goes stale the moment a later slice commits, so a
story needing a later tree **re-runs the harness at its own SHA and adds a harness row** rather
than citing the first one. Slice 4 committed `.kb/product/` atoms after slice 1's SHA, so this
story re-runs it. That is not duplicated work — it is the only way this story's tree contains the
atoms it is validating.

**5. In a pristine checkout, the immutability half of `validate --kb` cannot fail — so it is
complemented, not trusted.** `CLAUDE.md:87-91` describes the rule and its mechanism: an accepted
decision atom is immutable and "validation checks each one against `HEAD`". That catches an
*uncommitted* edit, which is exactly the state a fresh clone never has. Run there, the check is
satisfied by construction. This repository's own standard for that situation is unambiguous — a
rule no adapter can fail is decorative (`CLAUDE.md` *The rule that matters*) — so the claim
AC-005/AC-006 of the project actually care about is checked over the commit range instead:
`git log --diff-filter=M <initiative-start>..HEAD -- .kb/decisions/` naming every accepted atom
whose body changed, with the expectation of none. The record states which half the checkout can
carry and which half the range check carries; a green `validate --kb` alone is not offered as
evidence for immutability.

**6. A deviation is dispositioned, and `adopt --templates` is never the disposition.** If the sorted
drift set is not the six, AC-012 (`project.md:233-236`) requires a recorded **decision** — revert
the accidental change, or adopt the new state deliberately with a stated reason — and requires that
`redkiln adopt --templates` was not run in either case. `CLAUDE.md:123-128` is the standing
prohibition and names the trap: `redkiln upgrade` recommends it, and it would overwrite all six
customisations with the bundled defaults, silently, so the assertion would then fail on the
*absence* it created. Two consequences for this story. First, a seventh advisory is a *finding*
with a destination, not a defect to fix here — repairs are `findings-disposition-register`'s
routing decision (`project.md` DR-12, `:179-183`), and fixing it inside this project would fail
the project's own AC-013. Second, "we did not run it" is made falsifiable rather than asserted: the
six template files carry no modifying commit across the initiative's range, which is the footprint
an `adopt` would have left, and that range check is the evidence.

**7. The health verdict is scoped to the SHA it was taken at, and says so.** Three artefacts in
this project land after the measured SHA by construction — this story's own `_ledger.md`, and the
folders of `findings-disposition-register` and `initiative-closeout-readiness`, the two slice-mates
that follow it (`_storymap.md:152-155`). The record therefore carries the SHA on the row, states
which artefacts are known to arrive after it, and names the `backlog` CI job on the merge commit
(`ci.yml:142-187`) as the standing re-check that covers them. The forbidden move is the opposite:
a verdict written as though it covered the finished tree. Relatedly, the two slice-mates are
*in flight* at the measured SHA — a story item legitimately sitting at an earlier stage is not a
`problem`, and if `validate` reports one for that reason the correct response is to re-run at a
later SHA and add a harness row, not to advance an item to make a check pass.

**8. Nothing is repaired here, and the boundary makes that observable.** The `testing` brief's
AC-013 row (`_decomposition.md:56`) sets the measurable form: "zero commits inside this project's
own stories touch code outside `.bklg/`/`.kb/` planning artefacts". This story writes evidence files
and record rows. It does not edit a template to make the drift set come out at six, does not touch
`.redkiln/processes/*.yaml`, and does not amend a `.kb/` atom that `validate --kb` complained about
— every one of those is a finding with an owner.

**9. This story validates whatever slice 4 actually produced, and does not re-decide it.**
`discover.md` *Questions*, first bullet, answers the obvious coupling: `validate --kb`'s
frontmatter/status conformance is the same regardless of how many atoms exist, so the amended
three-personas-plus-four-journeys shape (`_decomposition.md:150-166`, the 2026-08-12 amendment) is
not this story's to check for correctness — only for conformance. Whether the promoted set matches
the decision is AC-010's, already discharged in the brief; whether HS-P0016's DT-1 resolution is
consistent with it is a finding for `findings-disposition-register`.

**10. The persona slice this realises.** The reader is the one `_storymap.md:18-22` names for the
whole project: someone who must be able to believe the initiative closed honestly *without
re-deriving the evidence*. For this story that reader asks one question — "is the backlog and the
knowledge base actually in the state you say, or did a tool merely exit zero?" — and the answer has
to survive their scepticism about the instrument itself. That is why the parsed report, the CLI
version, the verbatim assertion and the raw transcripts are all evidence, and why a summary
sentence ("health was clean") satisfies nothing here.

**11. Nothing here designs a surface.** `_design.md` is approved with **no public API surface** —
`N/A` against every item block, sign-off recorded 2026-08-12 — so the surface invariants that
normally bind a story in this repository are vacuous and must not be ticked as though they were
met. This story adds, changes and removes zero public items and zero conformance rules.

## Integration contract

- **Archetype**: `capability` — a complete slice from "run the instrument" to "a reader can see the
  verdict and check it themselves", mounted in the project's one reader-facing artefact. Nothing is
  mocked: `_decomposition.md:121-123` is explicit that a project whose deliverable is re-observation
  "would falsify its own purpose by substituting a double for anything it claims to have re-run".
- **Slice / milestone**: `closeout-health-and-disposition`. Slice-mates, implemented in one context
  and landing as one integrated surface: `findings-disposition-register` (which consumes this
  story's findings among others) and `initiative-closeout-readiness`. This story is first inside the
  slice (`_storymap.md:152-155`) because the other two read what it measures.
- **Mount point**:
  `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md` — its
  **Backlog and KB health** section, created empty-with-placeholders by `clean-checkout-harness`
  (that story's B-8 names the section and binds it to AC-011/AC-012). For a project whose only
  deliverable is verification and recording, this file is the composition root and the render path:
  it is the one place a reader meets the project's output (`_storymap.md:24-30`). This story fills
  that section and adds its own **Harness** row for the SHA it measured; it does not re-shape the
  document.
- **Wires into** (real siblings and contracts consumed, by path):
  - `.github/workflows/ci.yml:142-187` — the `backlog` job: the command order (`:144-150`), the
    exit-code caveat (`:147-149`), the verbatim `jq` expression (`:176-187`) and the six literal
    paths (`:180-185`). This story's local check is the same question asked of a different tree.
  - `.github/workflows/ci.yml:120-132` — the CLI pin (`ref: v0.18.0`) and the recorded reason
    pinning is load-bearing for the drift set.
  - `.redkiln/templates/` — the six deliberately customised files: `_design.md`, `_intake-brief.md`,
    `discover.md`, `gates/discover.md`, `gates/intake.md`, `spec.md`. All six exist in this tree.
  - `.redkiln/processes/` — the pinned pack (`bug`, `incident`, `initiative`, `project`,
    `project-lite`, `story`, `story-lite`). Never edited here; zero `process-drift` is the claim.
  - `.redkiln/config.yaml:5` (`support_initiative`, where an incidental defect routes), `:12`
    (`auto_stage_telemetry`, why a redkiln run can leave a tracked-path footprint in a checkout),
    `:67` / `:73` (`require_ledger`, `require_commit_provenance` — this story's own `_ledger.md`
    must cite the captured report and the harness row).
  - `.kb/` in full — `decisions/`, `open-questions/`, `maps/`, `playbooks/`, `governance/`,
    `concepts/`, and `product/` as slice 4 leaves it. `validate --kb` reads all of it;
    `.kb/decisions/README.md` is the immutability rule the range check complements.
  - `clean-checkout-harness`'s recorded procedure and its `_evidence/` layout — re-run at this
    story's SHA, producing a second harness row rather than a second convention.
  - `.kb/playbooks/verify-the-referent-and-report-coverage.md` — the citation contract the record's
    evidence cells encode: name the referent, not merely the address, and report coverage.
- **Renders surfaces**: **none.** `_design.md`'s `## Items` block is `N/A` by an approved
  determination rather than by omission, so there is no surface id for this story to claim. What it
  renders is a document section, and its legibility invariants are carried as AC rows, not as prose.
- **Public items**: none. Zero Rust items, zero CLI flags, zero config keys.
- **Conformance rule(s)**: none, and this is **not adapter-observable**. No rule is added to
  `crates/happenstance-testkit/src/suite.rs` and no port changes, so no adapter could fail
  differently because of this story. Naming one would be decorative in exactly the sense
  `CLAUDE.md` forbids.
- **Clause(s)**: none discharged, none amended. No `[FROZEN]` clause of `spec/SPECIFICATION.md` is
  touched, so no new ADR is owed. `cargo xtask spec-trace` runs inside
  `whole-gate-green-on-the-assembled-tree`'s gate run, not here.
- **Advances DoD scenario**: **DoD 16** (`initiative.md:405-407`, "Persona and journey atoms exist
  under `.kb/product/` with valid frontmatter and pass validation") — this story is where "pass
  validation" is *observed* on the assembled tree rather than assumed from the ingest run. It also
  carries the `redkiln validate --kb` half of **DoD 14** (`:398-401`) and is the whole of the
  tooling half of **exit criterion 7** (`:578-579`, "`redkiln validate --kb` and `redkiln doctor`
  are clean"), which is why the verdict is mounted where `initiative-closeout-readiness` can cite it.

## PR boundary

`redkiln verify --grain story` reads the first fenced block under this heading and fails on any file
changed outside it. The set is narrow on purpose: this project owns no code (`project.md` *Out of
scope*), and the `testing` brief's AC-013 row (`_decomposition.md:56`) makes "zero commits… touch
code outside `.bklg/`/`.kb/` planning artefacts" an observable criterion. The tempting breaches are
specific and named: editing a `.redkiln/templates/` file so the drift set comes out at six, and
amending a `.kb/` atom that `validate --kb` complained about. Both are findings with owners.

```
.bklg/from-contract-to-published-library/closeout-and-durable-audience/backlog-and-kb-health-at-closeout/**
.bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md
```

**In this PR**

- A clean checkout re-run at this story's SHA, following `clean-checkout-harness`'s recorded
  procedure, with its residue evidence captured — because slice 4's `.kb/product/` atoms postdate
  slice 1's pinned SHA.
- The three commands run for real in that checkout — `redkiln validate`, `redkiln validate --kb`,
  `redkiln doctor --json` — with raw stdout/stderr, the captured `doctor.json`, the separately
  recorded exit codes, and the `redkiln --version` output committed under this story's `_evidence/`.
- The verbatim `jq` expression from `ci.yml:176-187` evaluated against that `doctor.json`, with its
  own exit status captured.
- The commit-range complements: the `.kb/decisions/` modification check (*Context pack* 5) and the
  `.redkiln/templates/` no-modifying-commit check (*Context pack* 6).
- The **Backlog and KB health** section of `_closeout-record.md` filled, plus this story's own
  **Harness** row carrying the measured SHA, and the stated scope caveat for artefacts landing after
  it.
- Where the drift set deviates: the recorded disposition — revert, or a deliberate adopt with a
  stated reason — and the finding handed to `findings-disposition-register`.
- This story's `_ledger.md` (second pass), citing those artefacts per `.redkiln/config.yaml:67`.

**Explicitly not in this PR**

- Fixing anything the run reports. A `problem`, a `process-drift` warning, a seventh advisory or a
  `validate --kb` failure is routed, never repaired here (`project.md` DR-12; AC-013 belongs to
  `findings-disposition-register`).
- Any edit to `.redkiln/templates/`, `.redkiln/processes/`, `.redkiln/config.yaml` or
  `.github/workflows/ci.yml`. Changing the assertion to match the tree is the inverse of this story.
- **`redkiln adopt --templates`**, in any form, for any reason (`CLAUDE.md:123-128`).
- Authoring, amending or hand-writing any `.kb/` atom — including a product atom `validate --kb`
  rejects. Promotion is `product-atom-promotion-via-kb-ingest`'s, through the ingest path (DR-8).
- Any `redkiln advance` / `redkiln new` / item-frontmatter edit. The command owns every transition.
- The findings register itself, and the `redkiln status` no-open-child check — the two slice-mates.
- The gate run, the DoD table, the DoD 13 delta and the two audit tables — earlier slices, cited
  from the record rather than re-derived.

**Merge DoD one-liner.** A reader who distrusts the instrument can open one section, see the exact
CLI version and SHA the three commands ran at, read the raw report, re-evaluate CI's own assertion
against it themselves, and reach the same verdict — or see the deviation named with a decision
beside it.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **B-1 · The tree is a clean checkout pinned at this story's own SHA** | `clean-checkout-harness`'s procedure is re-run — a fresh clone from `origin`, detached at the closeout SHA current when this story runs — because slice 4 committed `.kb/product/` atoms after slice 1's pinned SHA and a health check that cannot see them is measuring the wrong tree. A second **Harness** row is added; slice 1's row is not overwritten or re-cited. Residue evidence (`git status --porcelain --ignored`, empty) is taken before any command runs. | `_decomposition.md:105-107` (all four commands run against the clean-checkout tree); `project.md:129-131` (DR-1); `clean-checkout-harness/spec.md` B-2, B-7 |
| **B-2 · Three commands, in CI's order, each captured whole** | `redkiln validate`, then `redkiln validate --kb`, then `redkiln doctor --json > doctor.json`, matching `ci.yml:144-150`. Every invocation's stdout, stderr and exit status is captured to a file — `doctor`'s with `|| true` so the report survives a red run, exactly as CI does it, because the assertions need the report even when it is red. Nothing is read from scrollback. | `.github/workflows/ci.yml:144-151`; `_decomposition.md:96-101` (the merge-gate command block) |
| **B-3 · The verdict comes from the parsed report; the exit code is recorded beside it, never in place of it** | The three claims are evaluated by the `jq` expression transcribed verbatim from `ci.yml:176-187`: zero `.problems`, zero `template`-independent `process-drift` warnings, and `[.warnings[] \| select(.kind == "template-drift") \| .file] \| sort` **equal** to the six literal paths. `doctor`'s own exit code is captured separately. A disagreement between the two — exit zero with a failing assertion, or the reverse — is recorded as a finding, since it would mean CI and this run are asking different questions. This is the named wrong implementation inverted. | `discover.md` *The wrong implementation*; `.github/workflows/ci.yml:147-149`, `:176-187` |
| **B-4 · Equality, not superset, and the six are named in the record** | The expected set is `.redkiln/templates/_design.md`, `_intake-brief.md`, `discover.md`, `gates/discover.md`, `gates/intake.md`, `spec.md` — all six present in this tree today. The record repeats the six by name so a reader can compare without opening CI, and states that the test is equality: an extra entry is a template changed without a decision, a missing one is a customisation reverted. No count is written in prose beside the list — `ci.yml:168-171` records why a number beside its own list drifts the first time the list moves. | `.github/workflows/ci.yml:160-171`, `:180-185`; `CLAUDE.md:116-121`; `.redkiln/templates/` (all six on disk) |
| **B-5 · The CLI version is evidence, because the drift set is a property of the CLI** | `redkiln --version` is captured in the same session as the run. CI pins `v0.18.0` (`ci.yml:130`) precisely because drift compares this repository's `.redkiln/` against the **bundled** files of whichever CLI executes. If the local version is not `v0.18.0`, the record states the version and states that the comparison is against a different bundle *before* stating any verdict. The local CLI resolves to `0.18.0` today, so the expected case is agreement — an unexpected version is a finding, not a footnote. | `.github/workflows/ci.yml:120-125`, `:130`; `redkiln --version` capture under `_evidence/` |
| **B-6 · `validate --kb` is complemented by a commit-range immutability check** | Run in a pristine checkout, the accepted-decision immutability half of `validate --kb` compares each atom against `HEAD` and the working tree *is* `HEAD`, so it cannot fail there (`CLAUDE.md:87-91`). The non-vacuous claim is checked over the range: `git log --diff-filter=M <initiative-start-sha>..HEAD -- .kb/decisions/`, expected to name no commit that modifies an atom whose status was `accepted`; a supersession (a **new** atom plus a status flip on the old one) is the permitted shape and is recorded as such. The record states which half each check carries. | `CLAUDE.md:87-91`; `.kb/decisions/README.md`; `.kb/governance/rewrite-the-referent-never-the-reasoning.md` (which edits are repairs and which are reversals) |
| **B-7 · "`adopt --templates` was not run" is proven by footprint, not asserted** | `git log --oneline <initiative-start-sha>..HEAD -- .redkiln/templates/` is expected to name no commit modifying any of the six, which is the footprint an `adopt` would have left — it overwrites all six with the bundled defaults (`CLAUDE.md:123-128`). Combined with B-4's equality result, a reader can conclude the prohibition held without taking anyone's word for it. A modifying commit that is *not* an adopt (a deliberate customisation change) is a legitimate outcome and takes B-8's disposition path. | `CLAUDE.md:123-128`; `.github/workflows/ci.yml:164-166` (a missing entry is what `adopt` produces) |
| **B-8 · A deviation is dispositioned in the record and routed, never repaired** | If the sorted set is not the six, the record states which file is extra or missing, what changed it (from the range check in B-7), and the decision: **revert** the accidental change, or **adopt deliberately** with a stated reason — and confirms `adopt --templates` was not run either way. The change itself is not made here: the disposition is a decision plus a routed item, per `project.md` DR-12 (`:179-183`) and `.redkiln/config.yaml:5` for an incidental defect. The same routing applies to any `problem`, any `process-drift` warning, and any `validate --kb` failure. | `project.md:233-236` (AC-012); `project.md:179-183` (DR-12); `.redkiln/config.yaml:5` |
| **B-9 · Warning kinds other than the two named are ignored, deliberately and visibly** | CI ignores every other warning kind on purpose: `harvest-unrecorded` "and friends fire in normal operation, and a `warnings.length > 0` guard would red this branch the day it landed" (`ci.yml:173-175`). This run inherits that scope exactly rather than inventing a stricter one. The record lists the other warning kinds the report contained, unasserted, so a reader can see what was consciously not claimed — the difference between "no other warnings" and "no other warnings were asserted on". | `.github/workflows/ci.yml:173-175` |
| **B-10 · The verdict carries its SHA and states what it does not cover** | Three artefacts land after the measured SHA by construction: this story's `_ledger.md` and the two slice-mates' folders (`_storymap.md:152-155`). The record's row carries the SHA it was observed at, names those artefacts as arriving later, and names the `backlog` CI job on the merge commit (`ci.yml:142-187`) as the standing re-check. A slice-mate story legitimately mid-stage at that SHA is not a `problem`; if `validate` reports one for that reason, the answer is a later SHA and a new harness row, never advancing an item to make a check pass. | `_storymap.md:152-155`; `.redkiln/config.yaml:73` (`require_commit_provenance`); `clean-checkout-harness/spec.md` B-7 |
| **B-11 · Every evidence cell names a referent and the section reports its coverage** | Column contract inherited from the record's own shape: artefact path, the short subject string a reader should find there, and the SHA it was observed on — an address that resolves says nothing about whether the attributed content is there. The section states rows filled against rows owed, so a partially populated health verdict cannot read as a complete one. | `.kb/playbooks/verify-the-referent-and-report-coverage.md`; `.kb/playbooks/anchoring-citations-in-a-long-lived-document.md`; `clean-checkout-harness/spec.md` B-9 |
| **B-12 · The run's own footprint in the checkout is recorded, not tidied away** | `.redkiln/config.yaml:12` sets `auto_stage_telemetry: true` and `:10` puts telemetry under the tracked path `.redkiln/telemetry/events/`, keyed per worktree — so a redkiln run inside a differently-named clone can leave a tracked-path change there. Whatever the three commands leave behind (a telemetry file, a rebuilt `.redkiln/index/`, which `.redkiln/.gitignore:2` ignores) is recorded in the harness row rather than deleted to make a post-run status check look clean. Deleting it would be manufacturing the evidence the residue check exists to collect. | `.redkiln/config.yaml:10-12`; `.redkiln/.gitignore`; `clean-checkout-harness/spec.md` B-6 |
| **B-13 · No public interface changes** | No Rust item, no CLI flag, no config key, no conformance rule, no clause. `_design.md` records **no public API surface** for this project, approved 2026-08-12, so the surface invariants that normally bind a story here are vacuous and are not ticked. The only "interface" produced is the record section's shape, which binds readers rather than callers. | `_design.md` (*Items*, *Signatures*, *Sign-off*) |

## Data and migrations

**N/A — no schema, no store, no migration.** This story writes no persistent structure any program
reads, and creates no data format. Three near-misses, each deliberately not a migration:

- **`doctor.json`** is a captured report, not a schema this repository owns. Its shape —
  `.problems`, `.warnings[].kind`, `.warnings[].file` — is defined by the redkiln CLI at the pinned
  version and consumed here exactly as `ci.yml:176-187` consumes it. This story does not parse it
  into a new structure, does not normalise it, and does not define a format for it; it stores the
  bytes and evaluates CI's own expression against them. A shape change is a CLI upgrade and belongs
  to the commit that raises the pin (`ci.yml:124-125`), not here.
- **`_closeout-record.md`** is markdown read by humans, created and shaped by
  `clean-checkout-harness` (its B-8). This story fills an existing section and appends a harness
  row; it does not alter the column contract, so there is no prior shape to migrate from and no new
  one introduced.
- **`.kb/` atoms** are read, never written. `KbFrontmatter` is validated here and neither introduced
  nor changed — authoring runs through `/redkiln:kb-ingest` in the preceding story (DR-8), and
  amending an atom this run rejects is explicitly out of boundary.

No item frontmatter is written either: the redkiln CLI is the single writer of `id`, `stage`,
`status`, `updated` and `links` (`CLAUDE.md:111-114`), and this story runs no `advance` and no `new`.

## Acceptance criteria

The reader every criterion is written for is the one `_storymap.md:18-22` names for the whole
project — **someone who must be able to believe the initiative closed honestly without re-deriving
the evidence** — and specifically, in this slice, the evaluator of
`../_discovery/distillation/personas-and-journeys.md` (*Persona 4 — the evaluator, pre-adoption*),
who gets one bounded pass over public artefacts, cannot run the suite, and is exactly the reader the
initiative's *Decide in one sitting* journey (`initiative.md:249-250`) is written for. For this
story that reader arrives with one question: **"is the backlog and the knowledge base actually in
the state you say, or did a tool merely exit zero?"** Every criterion below is that reader crossing
the whole stack — the question, the artefact they open, and what must be there. A criterion
satisfied by "the command was run and passed" and not by "the reader can see *what was asked*, *of
which tree*, *at which CLI version*, and re-evaluate it themselves" has not been met.

Evidence file names are **fixed by this spec**, not left to the implementer: two slice-mates cite
them and a renamed artefact is a dead citation
(`.kb/playbooks/anchoring-citations-in-a-long-lived-document.md`). All sit under
`.bklg/from-contract-to-published-library/closeout-and-durable-audience/backlog-and-kb-health-at-closeout/_evidence/`,
abbreviated `_evidence/` below.

| id | criterion | verification |
| --- | --- | --- |
| **AC-001** | **The reader can name the tree and the instrument, and get the same answer themselves.** GIVEN the evaluator asking "clean on *what*, measured with *what*", WHEN they open the **Backlog and KB health** section of `_closeout-record.md`, THEN they find (a) the SHA the three commands ran at and the isolation command that produced that checkout, added as this story's own **Harness** row rather than citing slice 1's — because slice 4 committed `.kb/product/` atoms after slice 1's pin, so slice 1's tree does not contain the atoms being validated (B-1); (b) the `redkiln --version` output captured in the same session, with an explicit statement of whether it equals CI's pinned `v0.18.0` (`ci.yml:130`) and, if not, that the comparison is against a *different bundled template set* stated **before** any verdict (B-5); and (c) `git status --porcelain --ignored` empty in that checkout, captured before any command ran. Re-running the recorded isolation command at the recorded SHA yields a tree whose `git rev-parse HEAD` equals it. | `_evidence/checkout-transcript.txt` (clone + `checkout --detach <SHA>`, captured whole, per `clean-checkout-harness` B-2), `_evidence/residue-before.txt` (zero bytes, command on record), `_evidence/cli-version.txt` (`redkiln --version`), read against this story's **Harness** row in `_closeout-record.md`. A version recorded without the equals/not-equals statement does not satisfy this: the number is not the claim, the comparison is. |
| **AC-002** | **The reader can see that the verdict came from the report, not from an exit code.** GIVEN the evaluator who knows `doctor` "exits non-zero on a problem and zero on a warning" (`ci.yml:147-149`) and therefore distrusts a green tick, WHEN they open the assertion evidence, THEN they find the three commands run in CI's order — `redkiln validate`, `redkiln validate --kb`, `redkiln doctor --json > doctor.json` with `\|\| true` — each invocation's stdout, stderr **and exit status** captured to a file; the full `doctor.json` committed as bytes; and the `jq` expression **transcribed byte-for-byte from `ci.yml:176-187`** (not paraphrased, not re-spelled) evaluated against those bytes with its own exit status recorded. The exit code appears in the record *beside* the parsed verdict and never in place of it, and a disagreement between the two is recorded as a finding rather than reconciled. This is the story's named wrong implementation (`discover.md` *The wrong implementation*) stated as its inverse. | `_evidence/validate.txt`, `_evidence/validate-kb.txt` (stdout/stderr/exit each), `_evidence/doctor.json` (verbatim report), `_evidence/doctor-exit.txt` (the exit status alone), `_evidence/assertion.txt` (the `jq` expression as pasted, plus its exit status). A byte-comparison of the expression in `_evidence/assertion.txt` against `.github/workflows/ci.yml:176-187` — an equivalent-looking rewrite fails this criterion even when it returns the same answer (B-2, B-3). |
| **AC-003** | **The reader can check the six for themselves without opening CI.** GIVEN the evaluator asking "six of *what*, and six out of how many", WHEN they read the health section, THEN they find the six expected paths written out by name — `.redkiln/templates/_design.md`, `_intake-brief.md`, `discover.md`, `gates/discover.md`, `gates/intake.md`, `spec.md` — the sorted `template-drift` `.file` list the report actually contained, and the statement that the test is **equality, not superset**, with the reason on both sides: an extra entry is a template changed without a decision, a missing entry is a customisation reverted (`ci.yml:160-171`; `CLAUDE.md:116-121`). No count is written in prose beside the list — `ci.yml:168-171` records that a number beside its own list drifts the first time the list moves, having itself said "the four files" while the list held six. | `_evidence/assertion.txt` (the equality clause and its result) read against `_evidence/doctor.json`'s `template-drift` entries, and the six named paths in `_closeout-record.md`'s health section compared to `.github/workflows/ci.yml:180-185`. All six files exist in `.redkiln/templates/` in this tree; a passing assertion against a report listing five is a contradiction to route, not to reconcile (B-4). |
| **AC-004** | **The reader can tell what was deliberately *not* claimed.** GIVEN the evaluator asking "you asserted on two warning kinds — what else was in that report", WHEN they read the health section, THEN they find every other `warnings[].kind` the report contained listed **unasserted**, with the inherited scope stated in the repository's own terms: `harvest-unrecorded` "and friends fire in normal operation, and a `warnings.length > 0` guard would red this branch the day it landed" (`ci.yml:173-175`). The difference the criterion protects is between "there were no other warnings" and "no other warnings were asserted on" — the first is a claim this run cannot support and must not appear. Inventing a stricter local check than CI's also fails this: the local run and CI must be capable of disagreeing about the tree, never about the question. | `_evidence/warning-kinds.txt` (`jq -r '[.warnings[].kind] \| group_by(.) \| map({kind: .[0], count: length})'` or equivalent enumeration over the same `doctor.json`), reproduced as an unasserted list in `_closeout-record.md` with the `ci.yml:173-175` reason quoted (B-9). |
| **AC-005** | **The reader is not offered a vacuous check as evidence.** GIVEN the evaluator who reads that accepted decision atoms are immutable and that "validation checks each one against `HEAD`" (`CLAUDE.md:87-91`), and who then notices the run happened in a **pristine checkout where the working tree is `HEAD`**, WHEN they look for what actually backs the immutability claim, THEN they find the record stating which half `validate --kb` can carry there (frontmatter/status conformance, including on slice 4's new `.kb/product/` atoms) and which half it cannot, plus the non-vacuous complement over the commit range — `git log --diff-filter=M <initiative-start-sha>..HEAD -- .kb/decisions/`, expected to name no commit modifying the body of an atom whose status was `accepted`, with a supersession (a **new** atom plus a status flip on the old one) recorded as the permitted shape when one appears. A green `validate --kb` offered alone as evidence for immutability fails this criterion — that is a rule no run in this checkout could fail, which is decorative in exactly the sense `CLAUDE.md` *The rule that matters* forbids. | `_evidence/validate-kb.txt` (the conformance half, exit zero) and `_evidence/decisions-range.txt` (the range check with its full command line and output, empty or with each hit classified as supersession vs. body edit), read against `.kb/decisions/README.md` and `.kb/governance/rewrite-the-referent-never-the-reasoning.md` for which edits are reversals (B-6). |
| **AC-006** | **The reader can verify the prohibition held, and see any deviation dispositioned rather than tidied.** GIVEN the evaluator who has read `CLAUDE.md:123-128` ("**Never run `redkiln adopt --templates`**" — `redkiln upgrade` recommends it and it would overwrite all six customisations with the bundled defaults, silently), and who knows that "we did not run it" is an assertion rather than evidence, WHEN they check, THEN they find `git log --oneline <initiative-start-sha>..HEAD -- .redkiln/templates/` naming **no commit modifying any of the six** — the exact footprint an adopt would have left — which together with AC-003's equality result lets them conclude the prohibition held without taking anyone's word for it. AND, where the sorted set is *not* the six, they find the deviation dispositioned in the record: which file is extra or missing, which commit changed it (from the same range check), and the decision — **revert** the accidental change, or **adopt deliberately with a stated reason** — with confirmation that `adopt --templates` was not run either way, and the finding handed to `findings-disposition-register` with a destination. The change itself is not made here; a template edited so the assertion comes out at six fails this criterion and AC-013 on the project's behalf. | `_evidence/templates-range.txt` (the range check over `.redkiln/templates/`, with each hit — if any — classified as adopt-footprint vs. deliberate customisation), plus the disposition rows in `_closeout-record.md`'s health and **Findings** sections (destination column, no fix column) checked against `project.md:233-236` (AC-012), `project.md:179-183` (DR-12) and `.redkiln/config.yaml:5`. Where the set *is* the six, the criterion is met by the recorded no-deviation statement plus the empty range check — not by silence (B-7, B-8). |
| **AC-007** | **The reader can tell exactly how far the verdict reaches.** GIVEN the evaluator who notices that three artefacts in this project land *after* the SHA this story measured — this story's own `_ledger.md` and the folders of `findings-disposition-register` and `initiative-closeout-readiness` (`_storymap.md:152-155`) — WHEN they read the health verdict, THEN it carries the SHA on the row rather than the document carrying one, names those artefacts as arriving later, and names the `backlog` CI job on the merge commit (`ci.yml:142-187`) as the standing re-check that covers them; every evidence cell names the artefact path **plus the subject string a reader should find there** plus the SHA it was observed on; the section states rows filled against rows owed; and whatever footprint the run itself left in the checkout (a telemetry file under the tracked `.redkiln/telemetry/events/` path, given `auto_stage_telemetry: true` at `.redkiln/config.yaml:12`, or a rebuilt `.redkiln/index/` that `.redkiln/.gitignore` ignores) is **recorded, not deleted**. A verdict written as though it covered the finished tree fails this; so does a post-run status check made clean by tidying, which manufactures the evidence the residue check exists to collect. | `_evidence/residue-after.txt` (`git status --porcelain` in the checkout after the three commands, with any entry named and explained rather than removed), read together with the per-row SHA, the coverage line and the scope caveat in `_closeout-record.md`'s health section; contract checked against `.kb/playbooks/verify-the-referent-and-report-coverage.md` and `.kb/playbooks/anchoring-citations-in-a-long-lived-document.md` (B-10, B-11, B-12). |

**Coverage of the traced project ACs.** `project.md` **AC-011** ("backlog and knowledge base are
clean, with exactly six drift advisories") is carried by AC-001 (which tree, which instrument),
AC-002 (the parsed three-part assertion rather than an exit code), AC-003 (equality against the six
named files), AC-004 (the scope not claimed) and AC-005 (the `validate --kb` half made non-vacuous).
`project.md` **AC-012** ("a seventh advisory, or a missing sixth, is dispositioned rather than
accepted") is carried whole by AC-006, including its "`adopt --templates` is not run in either case"
clause, made falsifiable by footprint. AC-007 carries the legibility both project ACs depend on —
a verdict whose reach is unstated is not a checkable claim. No project AC is orphaned, and this
story claims no other: AC-013's routing register and AC-014's `redkiln status` check are the two
slice-mates'.

## Interaction quality

This story renders **no screen and no public API**. `_design.md` is approved with *no public API
surface* — `N/A` against every item block, sign-off recorded 2026-08-12 by the repository owner —
so the project's signed-off design contributes **no composition invariants to inherit**, and
inventing some here would be fabricating a design a human never approved. What this story *does*
render is a document section a reader meets: the **Backlog and KB health** section of
`_closeout-record.md`, whose shape was fixed by `clean-checkout-harness` (its B-8) and which this
story fills rather than re-shapes.

Every invariant below is already carried by an AC row in the table above. None is a prose-only
bullet, deliberately: `redkiln verify` extracts criteria by matching a leading `| AC-001 |` table
cell or an `- AC-001:` bullet, so an invariant stated only here would get no ledger row, would never
be gated and would never be tested.

**State family.**

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **In-place, not context-jump** — this story fills an existing section and appends one **Harness** row; it creates no second record file, renames no heading and re-orders no section | AC-001, AC-007 | The PR boundary's fenced block admits exactly this story's folder and `_closeout-record.md`; a structural diff showing a changed heading or a new `*-record.md` is a breach |
| **Non-occlusion** — the health section's owed rows stay visible as owed until filled, the unasserted warning kinds are shown rather than elided, and the coverage line stops a partially populated verdict from reading as a complete one | AC-004, AC-007 | Structural review: coverage line present and arithmetic-checkable; `_evidence/warning-kinds.txt` reproduced in the section |
| **Stable anchors** (the document analogue of preserved focus/scroll/selection) — the two slice-mates cite this section and these evidence file names; both survive later appends because rows are added at the end of their section and the fixed `_evidence/` names are part of this spec | AC-002, AC-006 | `.kb/playbooks/anchoring-citations-in-a-long-lived-document.md`; the evidence names are enumerated in the *Acceptance criteria* preamble, not chosen at implementation time |
| **Reversibility** — nothing is committed into the measured checkout, the checkout is discardable, and every observation is re-creatable by re-running the recorded command at the recorded SHA with the recorded CLI version | AC-001, AC-002 | `_evidence/residue-after.txt`; the **Harness** row carries the isolation command, the SHA and the version, so the re-run is a copy-paste |
| **Reachable without a tool** (the analogue of keyboard reachability) — every artefact is plain UTF-8 readable in any editor; `doctor.json` is committed as text a reader can pipe through their own `jq`, not as a summarised table | AC-002, AC-003 | `_evidence/*` are plain text; no captured output is a screenshot or a binary log |

**Composition family.** `_design.md` declares none, so these come from the record's own contract
(`clean-checkout-harness` B-8/B-9) and the two accepted playbooks — not invented here.

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **Presentation exists at all** — the health verdict is a composed section with a stated column contract, named files, a scope caveat and a coverage line; not a one-line "validate and doctor: clean" appended by whoever ran the commands | AC-003, AC-007 | Structural review against `clean-checkout-harness` B-8/B-9 |
| **Placement** — the verdict lands in the *project*-level `_closeout-record.md` because two slice-mates read it across stories; the raw transcripts land inside *this* story's `_evidence/` because they are this story's provenance | AC-001, AC-002 | The two paths in the PR boundary fenced block are exactly these two locations |
| **Transience** — the record section and the `_evidence/` captures are persistent (committed); the checkout, its `.redkiln/index/` and any local `target/` are transient and discarded. The one exception is deliberate: the run's own tracked-path footprint is *recorded* before the checkout is discarded, because deleting it would manufacture the residue result | AC-007 | *Data and migrations* above; `_evidence/residue-after.txt` names what was left rather than showing an empty file achieved by tidying |
| **Density budget, with its real numbers** — one section; **six** named template paths in the expected set and no prose count beside them; **three** assertion clauses, each with its own recorded result; **three** command captures plus one report plus one version capture; **two** commit-range complements; **one** coverage line; **one** added **Harness** row. A seventh named path without a disposition row, or a fourth assertion clause, means the local check stopped being CI's check | AC-002, AC-003, AC-006 | Counted in structural review against `.github/workflows/ci.yml:176-187` |
| **Hierarchy** — the tree and the instrument (SHA, CLI version) are stated **before** the verdict; the verdict before the unasserted remainder; the disposition, where one exists, immediately beside the deviation it dispositions rather than in a distant footnote | AC-001, AC-004, AC-006 | Section order review; AC-001 explicitly requires a non-`v0.18.0` version be stated *before* any verdict |
| **Named anti-patterns** — (a) reporting AC-011 satisfied on a green exit code (`discover.md` *The wrong implementation*); (b) a paraphrased `jq` expression that "checks the same thing"; (c) a superset reading of the six; (d) editing a `.redkiln/templates/` file so the assertion comes out at six; (e) running `redkiln adopt --templates` in any form; (f) a verdict written as though it covered artefacts that land after the measured SHA; (g) deleting the run's own footprint to make a post-run status check look clean | AC-002 (a, b), AC-003 (c), AC-006 (d, e), AC-007 (f, g) | Each is a review check with a named artefact: (a) `_evidence/doctor-exit.txt` recorded *beside* `_evidence/assertion.txt`; (b) byte-comparison against `ci.yml:176-187`; (d)/(e) `_evidence/templates-range.txt` plus the PR boundary |

## Error conditions

| id | Condition | Required response |
| --- | --- | --- |
| **EC-1** | `doctor --json` exits **zero** but the `jq` assertion fails (or the reverse) | Record both, in that order, and treat the disagreement itself as a finding. It means CI's tick and this run's verdict are answering different questions, which is the precise thing `ci.yml:147-149` warns about. Do **not** pick the friendlier of the two. Route it (`project.md` DR-12). |
| **EC-2** | `doctor --json` writes nothing, or writes non-JSON, into `doctor.json` | The report is the evidence; without it there is no assertion to evaluate. Capture stderr, record the exit status, and stop — do not substitute the human-readable `doctor` output and parse it by eye. A hand-summarised report is not `doctor.json`, and every downstream citation would be to a paraphrase. |
| **EC-3** | The sorted `template-drift` set contains a **seventh** file | AC-006's disposition path: name the file, find the commit that changed it via `_evidence/templates-range.txt`, and record the decision — revert (as a routed item against the owning story), or adopt deliberately with a stated reason. **Do not edit the template here.** That would fix the assertion by changing the tree, which is the inverse of this story and fails `project.md` AC-013 on the project's behalf. |
| **EC-4** | One of the six is **missing** from the set | The stronger signal of the two: a missing entry is what `adopt --templates` produces (`ci.yml:164-166`; `CLAUDE.md:123-128`). Check `_evidence/templates-range.txt` first — a commit touching all six at once is the adopt footprint and is a finding of a different severity than a single reverted customisation. Record which, disposition per AC-006, route. Never re-apply the customisation here. |
| **EC-5** | `redkiln --version` is not `v0.18.0` | Not a blocker and not a footnote. Record the version, state in the record that the drift comparison ran against a **different bundled template set** than CI's assertion was written against, and place that sentence *before* the verdict (AC-001). If the six then differ, the version is the first hypothesis and the disposition says so — a redkiln release that alters a bundled template legitimately changes the expected set, and the response is a decision, never an `adopt`. |
| **EC-6** | `redkiln validate` reports a `problem` that is a slice-mate story legitimately sitting at an earlier stage | Not a defect and not something to make go away. The two slice-mates are in flight at the measured SHA by construction (*Context pack* 7). Re-run the harness at a later SHA and add a **new** harness row; **never** run `redkiln advance` to make a check pass — the command owns every transition, and advancing an item to satisfy an assertion falsifies both. |
| **EC-7** | `redkiln validate --kb` rejects an atom slice 4 promoted | Record it, mark the affected AC unsatisfied, and route it to `product-atom-promotion-via-kb-ingest`'s owner. **Do not hand-edit the atom** — promotion runs through `/redkiln:kb-ingest` (DR-8), and hand-authoring is the constraint the reverted commit `0269720` exists to remember. Amending an accepted decision atom is doubly forbidden: supersede, never edit (`.kb/decisions/README.md`). |
| **EC-8** | The `.kb/decisions/` range check names a commit that modified an accepted atom's body | The immutability claim is false on this tree, and `validate --kb` in a pristine checkout could not have caught it (AC-005). Classify the hit: a supersession (new atom + status flip on the old one) is the permitted shape; a body edit is a finding with a destination, and `.kb/governance/rewrite-the-referent-never-the-reasoning.md` is what separates the two. Record the classification with the commit, do not revert it here. |
| **EC-9** | The clean checkout cannot be produced at this story's SHA (unpushed commit, network failure, disk) | Halt rather than fall back to the warm working tree these planning artefacts are written in. `_decomposition.md:105-107` binds the commands to the clean-checkout tree, and a run in the authoring worktree measures a tree containing uncommitted planning artefacts — the exact failure `clean-checkout-harness` EC-2 rules out. Push and re-pin, or re-clone. |
| **EC-10** | The three commands leave a modified tracked file in the checkout (a telemetry event under `.redkiln/telemetry/events/`) | Expected and permitted: `auto_stage_telemetry: true` (`.redkiln/config.yaml:12`) with telemetry on a tracked path. Record the file in `_evidence/residue-after.txt` and in the harness row. Do **not** delete it, do not `git checkout --` it, and do not commit it into the checkout — the footprint is the observation (AC-007). |
| **EC-11** | `jq` is unavailable, or the transcribed expression fails to parse | Stop and fix the transcription, not the expression. `CLAUDE.md`-adjacent grounding records `jq` resolving on this machine (`jq-1.8.2`); a parse failure means the copy was lossy. Re-copy from `.github/workflows/ci.yml:176-187`. Reaching for an "equivalent" one-liner because the paste broke is anti-pattern (b) arriving by accident. |

## Non-functional

| id | Requirement | Why, and how it is checked |
| --- | --- | --- |
| **NF-1** | **The instrument is pinned and recorded, not assumed.** The CLI version travels with the evidence; the assertion travels byte-for-byte from CI. | The drift set is a property of the CLI's bundled files, not of this repository (`ci.yml:120-125`), so an unrecorded version makes the verdict unreproducible. AC-001, AC-002. |
| **NF-2** | **Auditability over convenience.** Every command's stdout, stderr and exit status is captured to a file; nothing load-bearing is read from terminal scrollback, including the empty results. | An empty output observed and not captured is indistinguishable from a command never run — the failure `.kb/playbooks/verify-the-referent-and-report-coverage.md` was written against. AC-002, AC-005, AC-006. |
| **NF-3** | **Cheap by construction.** No build, no `cargo` invocation, no network beyond the clone slice 1's procedure already performs. The three redkiln commands plus four `git log`/`status` invocations. | The expensive tree-assembly cost was paid by `clean-checkout-harness` and `whole-gate-green-on-the-assembled-tree`; this story re-runs only the checkout procedure, not the gate. Recording elapsed time in the harness row is enough. |
| **NF-4** | **Zero code, zero runtime cost.** No crate changes, so `cargo xtask affected --base main` names no affected package and the library's compile time, binary size and MSRV are untouched. | `project.md` *Out of scope*; the `testing` brief's AC-013 row (`_decomposition.md:56`), "zero commits inside this project's own stories touch code outside `.bklg/`/`.kb/` planning artefacts". |
| **NF-5** | **No credential travels into the evidence.** The transcripts are committed to a repository intended to be published; if any capture contains a token or a credential-bearing URL it is scrubbed before commit and the scrub is noted. | `doctor --json` and `validate` output paths and item ids, not secrets — but the checkout transcript carries a remote URL, and CI's own checkout uses `secrets.REDKILN_TOKEN` (`ci.yml:131`), so the habit is worth keeping. |
| **NF-6** | **The verdict is legible in one bounded pass.** The health section reads top to bottom — tree, instrument, verdict, unasserted remainder, disposition, scope — without requiring the reader to open CI or a sibling's ledger to know what was asked. | The evaluator persona gets one bounded pass and cannot re-run anything (`../_discovery/distillation/personas-and-journeys.md`, *Persona 4*). A section that is correct but requires three other files to interpret has failed the reader it is written for. |

## Implementation notes (non-prescriptive)

Not requirements — the shape the author of this spec expects, offered so the implementer can
disagree with something concrete.

- **Order matters, and some of it is only capturable in one order.** Re-run the checkout procedure →
  `residue-before` → `redkiln --version` → `validate` → `validate --kb` → `doctor --json` → the `jq`
  assertion → `warning-kinds` → the two commit-range checks → `residue-after`. The residue check
  taken after the commands is unfalsifiable; the version captured afterwards is a different session's
  claim.
- **Copy the `jq` expression, do not retype it.** Open `.github/workflows/ci.yml`, select lines
  176–187, paste. Then paste the same bytes into `_evidence/assertion.txt` next to the result. The
  temptation to "clean it up" into a single line is exactly anti-pattern (b) — and the six literal
  paths must keep CI's existing order in the pasted copy even though the comparison sorts.
- **`|| true` is part of the command, not sloppiness.** CI writes it (`ci.yml:150`) because the
  assertions need the report even when it is red. A run that lets a non-zero `doctor` abort the
  script has thrown away the evidence.
- **The two range checks need an `<initiative-start-sha>` and it should be resolved once.** Whatever
  commit `decision-atom-audit-table` and `open-question-preservation-audit` used for their own range
  diffs is the one to reuse — three different start points across one project's audits is a
  discrepancy a reader will find. Record the resolved SHA and how it was derived in
  `_evidence/decisions-range.txt`'s header.
- **Classify range-check hits, do not just count them.** `--diff-filter=M` over `.kb/decisions/`
  will also catch a legitimate status flip on a superseded atom. The output is only evidence once
  each hit carries "supersession" or "body edit" beside it, with the atom named.
- **Write the section for someone who will not open CI.** The six paths by name, the three claims in
  words, the result of each. A reader who has to fetch `ci.yml` to know what "the assertion" was has
  been handed a pointer where evidence was owed.
- **A transcript is fine; a script is not.** Redirecting each command into its own file, or `tee`,
  both satisfy the evidence contract. Committing a `scripts/health-check.sh` does not — it is code
  outside `.bklg/`, which the PR boundary and the `testing` brief's AC-013 row rule out, and it is
  the fastest wrong thing to reach for on the third re-run.

## Tests and CI (merge gate)

The `testing` brief (`_decomposition.md` *Test mix, summarised by tier*, `:63-73`) puts every one of
these three commands in the **static** tier and is explicit that static and process are the
load-bearing tiers for this project, because it audits rather than builds. There is no unit test to
write here, and writing one would mean this story had grown code it is forbidden to have
(`project.md` *Out of scope*).

| Tier | Command / path | Proves |
| --- | --- | --- |
| **Process** | `clean-checkout-harness`'s recorded isolation procedure, re-run at this story's SHA → `_evidence/checkout-transcript.txt`, `_evidence/residue-before.txt` (empty, command on record) | AC-001 — the tree measured contains slice 4's `.kb/product/` atoms and carries no residue; a second **Harness** row rather than a re-cited first one |
| **Static** | `redkiln --version` → `_evidence/cli-version.txt`, compared against `ci.yml:130` (`ref: v0.18.0`) | AC-001 — the bundled template set the drift comparison ran against is named, and the equals/not-equals statement precedes the verdict |
| **Static** | `redkiln validate` (stdout, stderr, exit) → `_evidence/validate.txt` | AC-002 — the backlog half of DR-11; a `problem` here is routed, and EC-6 covers the in-flight slice-mate case |
| **Static** | `redkiln validate --kb` (stdout, stderr, exit) → `_evidence/validate-kb.txt` | AC-002, AC-005 — `KbFrontmatter` conformance across `.kb/`, including slice 4's promoted atoms (DoD 16's "pass validation" half); explicitly **not** offered as evidence for immutability |
| **Static** | `redkiln doctor --json > doctor.json \|\| true` → `_evidence/doctor.json`, with the exit status captured separately to `_evidence/doctor-exit.txt` | AC-002 — the report survives a red run, and the exit code is recorded beside the verdict rather than standing in for it (`ci.yml:147-150`) |
| **Static** | The `jq` expression transcribed byte-for-byte from `.github/workflows/ci.yml:176-187`, evaluated against `_evidence/doctor.json`, with its exit status → `_evidence/assertion.txt` | AC-002, AC-003 — zero `.problems`, zero `process-drift`, `template-drift` file list **equal** to the six named paths. The same question CI asks, of a different tree |
| **Static** | An enumeration of `[.warnings[].kind]` over the same report → `_evidence/warning-kinds.txt` | AC-004 — the warning kinds present but deliberately unasserted, inheriting CI's scope exactly (`ci.yml:173-175`) |
| **Static / process** | `git log --diff-filter=M <initiative-start-sha>..HEAD -- .kb/decisions/`, each hit classified → `_evidence/decisions-range.txt` | AC-005 — the non-vacuous complement to the immutability half that a pristine checkout cannot fail (`CLAUDE.md:87-91`) |
| **Static / process** | `git log --oneline <initiative-start-sha>..HEAD -- .redkiln/templates/` → `_evidence/templates-range.txt` | AC-006 — "`adopt --templates` was not run" as a footprint check rather than an assertion (`CLAUDE.md:123-128`) |
| **Static** | `git status --porcelain` in the checkout after the three commands → `_evidence/residue-after.txt`, entries named and explained | AC-007 — the run's own tracked-path footprint recorded rather than tidied (`.redkiln/config.yaml:10-12`) |
| **Static** | Structural review of `_closeout-record.md`'s **Backlog and KB health** section: six paths named, three claims with results, unasserted kinds listed, per-row SHA, scope caveat, coverage line, disposition rows where owed | AC-003, AC-004, AC-006, AC-007 — the verdict is legible and its reach is stated; contract from `clean-checkout-harness` B-8/B-9 and `.kb/playbooks/verify-the-referent-and-report-coverage.md` |
| **Merge gate (story grain)** | `redkiln verify --grain story` — reads `_ledger.md`, the PR-boundary fenced block and commit provenance (`.redkiln/config.yaml:67`, `:73`) | Every AC has a ledger row with cited evidence, and no file changed outside the two-line boundary |
| **Merge gate (affected)** | `cargo xtask affected --base main` | NF-4 — no crate is affected, the observable form of "this project owns no code" |
| **Merge gate (CI, on the merge commit)** | The `backlog` job, `.github/workflows/ci.yml:142-187` | The standing re-check AC-007 names: it re-asks the same three claims on the merged tree, covering the artefacts that landed after this story's measured SHA |

The fourth merge-gate command the `testing` brief reserves for this project — `cargo xtask ci`
(`_decomposition.md:96-101`) — is **not** this story's bar. It belongs to
`whole-gate-green-on-the-assembled-tree` in slice 1, and citing its result as this story's evidence
would confuse a claim about the *library* with a claim about the *backlog and knowledge base*.

## Risks and coupling (PR-scoped)

| Risk / coupling | Note |
| --- | --- |
| **The green-exit shortcut** | `discover.md` *The wrong implementation*, and the single most likely way this story is done wrongly: run `redkiln doctor`, see exit zero, tick AC-011. It is faster by the cost of capturing and parsing a report, and it is compatible with a seventh advisory, a `process-drift` warning and every other warning kind. Caught by AC-002 requiring `_evidence/doctor.json` plus the transcribed expression, with the exit code recorded separately. |
| **The paraphrased assertion** | Subtler and nearly as likely: re-spelling the `jq` expression because the paste broke or "reads better". A rewritten assertion that happens to pass is indistinguishable from CI's until the day it is not — the local run and CI must be able to disagree about the *tree*, never about the *question*. Caught by the byte-comparison in AC-002 and EC-11. |
| **Pressure to fix what the run finds** | The whole project's standing risk (`project.md` risk table) lands here in its most tempting form, because every deviation this story can surface has a one-line repair: edit a template, revert a customisation, hand-fix an atom's frontmatter, advance a story item. Each is a boundary breach *and* fails AC-013 on `findings-disposition-register`'s behalf. EC-3, EC-4, EC-6 and EC-7 all say record-and-route. |
| **Staleness by construction, in both directions** | This story's tree is stale before the project ends (two slice-mates commit after it) and would be wrong if taken too early (slice 4's atoms must be in it). AC-001 handles the early half by re-running the harness; AC-007 handles the late half by stating the scope and naming the CI re-check. Nothing mechanical enforces either, which is why both are AC rows rather than notes. |
| **Coupling to slice 4 is a real ordering edge, not a courtesy** | `depends_on: product-atom-promotion-via-kb-ingest`. Run before those atoms exist and `validate --kb` passes over a `.kb/` that lacks the very atoms DoD 16 asks about — a green result that proves nothing (`_storymap.md:85-88`). Cheap check: `.kb/product/` in the measured checkout contains more than `README.md`. |
| **Coupling to the slice-mates is tight and directional** | `findings-disposition-register` consumes this story's findings (`_storymap.md:62` lists it as a `depends_on`), and `initiative-closeout-readiness` cites this verdict for exit criterion 7. Both are in the same slice and implemented in the same context, in this order. A finding recorded here without a destination is not this story's failure to fix — it is the register's input, and it must be shaped as one. |
| **Version drift in the instrument** | The expected six is a property of the pinned CLI's bundled files. A local CLI ahead of `v0.18.0` can legitimately produce a different set, and the response is a decision (EC-5), never an `adopt` and never a quiet re-pin. The corresponding repository-side move — raising the `ref` in `ci.yml` — is out of this PR's boundary entirely. |
| **The vacuity trap** | `validate --kb` in a pristine checkout satisfies its immutability half by construction. The risk is not that it fails; it is that it *passes* and gets cited for a claim it never tested. AC-005 exists because this repository's own standard ("a rule that no adapter can fail is decorative") applies to its own tooling, and nothing but this criterion applies it here. |

## Dependencies

**Blocks on:**

- `product-atom-promotion-via-kb-ingest` (slice 4, `durable-audience`) — the only `depends_on`
  edge, and a real one. The promoted `.kb/product/` atoms are **inputs** to `redkiln validate --kb`
  (`_storymap.md:85-88`: "Health must be measured *after* the new product atoms exist"), and DoD
  16's "pass validation" clause is observed here rather than assumed from the ingest run. Running
  before it lands produces a green result over a `.kb/` that does not contain what the verdict
  claims to cover.

Two further orderings are real but are *not* this story's `depends_on` edges, and are recorded so
nobody adds them: `clean-checkout-harness` (slice 1) supplies the procedure and the record's shape,
but this story re-runs the procedure at its own SHA rather than consuming slice 1's tree; and the
project itself depends on all nine sibling projects having merged, which is HS-P0019's edge,
discharged before any story here runs.

**Unlocks:**

- `findings-disposition-register` — same slice, immediately next (`_storymap.md:62` names this story
  among its `depends_on`). Every deviation, disagreement or rejected atom this run surfaces arrives
  there with a destination.
- `initiative-closeout-readiness` — same slice, last. Exit criterion 7 (`initiative.md:578-579`,
  "`redkiln validate --kb` and `redkiln doctor` are clean") is cited from this story's verdict rather
  than re-derived, which is why the verdict is mounted in `_closeout-record.md` and not left in this
  story's ledger.

## Anchors (progressive disclosure)

Read the *Context pack* first — it is complete enough to start from. Open a row below only when the
AC it serves is the one being worked. Nothing here is optional detail; it is deferred detail.

| Anchor | Why it is load-bearing | When to open | Serves |
| --- | --- | --- | --- |
| `.github/workflows/ci.yml` | The literal source of everything this story asserts: the command order and the `\|\| true` (`:144-151`), the exit-code caveat in the repository's own voice (`:147-149`), the three claims and why the third is equality (`:153-171`), the ignored-kinds scope (`:173-175`), the `jq` expression to transcribe byte-for-byte and the six literal paths (`:176-187`), and the CLI pin with the reason pinning is load-bearing (`:120-132`). | Before running anything — the expression is copied from here, not written | AC-002, AC-003, AC-004 |
| `.bklg/from-contract-to-published-library/closeout-and-durable-audience/project.md` | AC-011 (`:229-232`) and AC-012 (`:233-236`) are the traced criteria; DR-11 (`:175-178`) states the requirement at the project's own level; DR-12 (`:179-183`) is the routing rule that makes every deviation a finding rather than a repair; *Out of scope* is what forbids a script or a template edit. | Before writing the disposition procedure, and again before touching any file outside the boundary | AC-002, AC-003, AC-006 |
| `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_decomposition.md` | The one warranted `testing` brief: the AC-011 row (`:54`) and AC-012 row (`:55`) give the tier and the "re-run locally on the closeout tree, not inferred from CI's last run" instruction; *Merge-gate commands* (`:93-107`) binds all four commands to the clean-checkout tree; *Fixtures / seams* (`:109-123`) states nothing is mocked; the AC-013 row (`:56`) is the "zero commits touch code" criterion the PR boundary is written against. | Before deciding which tree to run in, and before adding any file outside `.bklg/` | AC-001, AC-002, AC-006 |
| `.bklg/from-contract-to-published-library/closeout-and-durable-audience/clean-checkout-harness/spec.md` | The procedure this story re-runs and the record shape it fills: B-1/B-2 (clone from `origin`, detach, residue captured before anything else), B-6 (the run's own footprint), B-7 (the rule that a later tree means a **new** harness row, never a re-cited one), B-8/B-9 (the record's nine sections, its column contract and coverage line). | Before re-running the checkout, and before writing into `_closeout-record.md` | AC-001, AC-007 |
| `.redkiln/config.yaml` | `:5` `support_initiative` — where an incidental defect routes; `:10-12` `auto_stage_telemetry` on a tracked telemetry path — why a redkiln run can legitimately leave a tracked-file footprint (EC-10); `:67` `require_ledger` and `:73` `require_commit_provenance` — why this story's `_ledger.md` must cite the captured report and the harness row. | While interpreting `residue-after`, and before filling `_ledger.md` | AC-006, AC-007 |
| `.kb/playbooks/verify-the-referent-and-report-coverage.md` | The accepted playbook behind the record's column contract and coverage line: an address that resolves says nothing about whether the attributed content is there, and a check that does not state what fraction it parsed is indistinguishable from one that sees everything. It is also the general form of AC-005's argument. | While writing the health section's evidence cells and coverage line | AC-005, AC-007 |
| `.kb/playbooks/anchoring-citations-in-a-long-lived-document.md` | Why the evidence cells carry a path *plus* a subject string rather than a bare `file:line`, and why the `_evidence/` file names are fixed by this spec: two slice-mates cite them into a tree that keeps moving. | While fixing evidence-cell shape, and if tempted to rename a capture | AC-002, AC-007 |
| `.kb/decisions/README.md` | The immutability rule the range check complements — accepted atoms are immutable, supersede rather than edit — and the shape a legitimate supersession takes, which is what EC-8's classification depends on. | While classifying hits in `_evidence/decisions-range.txt` | AC-005 |
| `.kb/governance/rewrite-the-referent-never-the-reasoning.md` | The accepted governance atom that separates a repair from a reversal in `.kb/`. Without it, "a commit modified an atom" is a bare fact rather than a classified finding. | Only if the `.kb/decisions/` range check returns a hit | AC-005 |
| `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_grounding.md` | `:67-78` reads the six-file assertion off CI directly and states in the project's own words that the check is equality "not just same-length", plus why the step's comments justify AC-012's "dispositioned rather than accepted" language. Useful as the second, independent transcription to check the first against. | Before writing the six names into the record | AC-003, AC-006 |
| `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_storymap.md` | `:61` is this story's row; `:85-88` is why health is measured last and why the slice-4 edge is real; `:24-30` is the convergence artefact and the fourteen-places failure mode; `:152-155` is the merge order inside this slice and therefore which artefacts land after the measured SHA. | Before assuming what a slice-mate will append, and while writing AC-007's scope caveat | AC-001, AC-007 |
| `.bklg/from-contract-to-published-library/closeout-and-durable-audience/backlog-and-kb-health-at-closeout/discover.md` | *The wrong implementation* is the shortest statement of what failure looks like here; *Questions* records the two things answered before spec (the atom-count coupling, and that AC-012 already specifies the deviation response). | First, before any command is run | AC-002, AC-006 |
| `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_design.md` | The approved *no public API surface* determination, sign-off 2026-08-12. Load-bearing **negatively**: it is why the composition family inherits nothing and why no surface invariant may be ticked as met. | If a reviewer asks why this spec declares no design invariants | AC-007 |
| `.bklg/from-contract-to-published-library/initiative.md` | DoD 14 (`:398`) and DoD 16 (`:405`) are the scenarios this story advances, and exit criterion 7 (`:578-579`) is the clause `initiative-closeout-readiness` will cite from this verdict; `:249-250` is the *Decide in one sitting* journey the acceptance criteria are framed from. | When framing or reviewing the acceptance criteria's reader, and before writing the DoD claim into the record | AC-001, AC-005 |
| `.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md` | *Persona 4 — the evaluator, pre-adoption*: the reader every criterion here is framed from, who decides in a bounded amount of research time and cannot re-run anything. Load-bearing because it is why the criteria are written as *what a reader can see* rather than *what was run*. | Before reviewing or re-wording any acceptance criterion | AC-001, AC-004 |
| `.kb/product/README.md` | `:15-24` — persona = `concept`, journey = `playbook`, both `authority_tier: product`, and closeout performs the promotion. It is the shape `validate --kb` is being asked to conform-check in this run, and the reason a rejected atom routes to slice 4 rather than being edited here (EC-7). | Only if `validate --kb` rejects something under `.kb/product/` | AC-005 |

## Clarifications resolved during spec

1. **Which tree the three commands run in** — `_decomposition.md:105-107` binds all four merge-gate
   commands to "the clean-checkout tree described in DR-1", and slice 1 built exactly one such tree,
   pinned at its own SHA. **Resolved: the harness procedure is re-run at this story's SHA and a
   second Harness row is added.** Slice 4 commits `.kb/product/` atoms after slice 1's pin, so slice
   1's tree does not contain the atoms this story is validating; `clean-checkout-harness` B-7 already
   states that a story needing a later tree re-runs and adds a row rather than citing the first
   (*Context pack* 4, AC-001).
2. **How a local run is made comparable to CI's** — neither `project.md` DR-11 nor the `testing`
   brief says. **Resolved as three commitments:** the `jq` expression is transcribed byte-for-byte
   rather than paraphrased; the CLI version is captured as evidence because the drift set is a
   property of the CLI's bundled files (`ci.yml:120-125`); and a version other than CI's pinned
   `v0.18.0` is stated *before* the verdict rather than treated as silently equivalent (AC-001,
   AC-002, EC-5).
3. **What `validate --kb` actually proves in a pristine checkout** — nothing upstream noticed that
   half of it goes vacuous there. **Resolved: the immutability half is complemented by a commit-range
   check over `.kb/decisions/`, and the record states which half each check carries.** This is the
   repository's own "a rule that no adapter can fail is decorative" standard applied to its own
   tooling; a green `validate --kb` is not offered alone as evidence for immutability (AC-005).
4. **How "`adopt --templates` was not run" becomes evidence** — AC-012 requires the confirmation but
   an assertion that a command was not typed is unfalsifiable. **Resolved: a footprint check.** The
   six template files carry no modifying commit across the initiative's range, which is precisely
   what an adopt would have left, since it overwrites all six with the bundled defaults (AC-006,
   `CLAUDE.md:123-128`).
5. **Which warning kinds are asserted on** — CI ignores every kind but the two, deliberately
   (`ci.yml:173-175`). **Resolved: this run inherits that scope exactly and lists the other kinds
   unasserted**, so a reader can see the difference between "there were no other warnings" and "no
   other warnings were asserted on". Inventing a stricter local check would make the local run and
   CI disagree about the question rather than about the tree (AC-004).
6. **What a `problem` about an in-flight slice-mate means** — `findings-disposition-register` and
   `initiative-closeout-readiness` are legitimately mid-stage at the measured SHA. **Resolved: not a
   defect.** The response is a later SHA and a new harness row; running `redkiln advance` to make a
   check pass is forbidden — the command owns every transition, and an item advanced to satisfy an
   assertion falsifies both the item and the check (EC-6, *Context pack* 7).
7. **AC count unchanged.** The seven ids the first pass enumerated (AC-001…AC-007) are exactly the
   seven written here; none was added or dropped. The interaction-quality invariants were mapped onto
   existing rows rather than given rows of their own, because each is a property of the tree and
   instrument (AC-001), the assertion (AC-002, AC-003), the stated scope (AC-004, AC-007) or the
   disposition (AC-006) that those criteria already assert — an eighth row would have restated one
   of them.
8. **Not decided here, deliberately**: whether slice 4's three-personas-plus-four-journeys shape is
   the *right* set (AC-010's, already discharged in `_decomposition.md`'s amended DR-10; this story
   checks conformance, not correctness — *Context pack* 9); whether HS-P0016's recorded DT-1
   resolution is consistent with it (a finding for `findings-disposition-register`); and the
   destination of any finding this run produces (the register's, by construction — this story shapes
   findings as inputs, it does not route them).
