# Wave `2026-08-10-intake` — corpus match

What already exists in `.kb/`, what each incoming claim could have merged into, and the score
behind every disposition. Read this before `02-placement-and-adjudication.md`: the placements
there are only defensible because of what this file establishes about the corpus.

## The corpus, as verified

```
Glob .kb/**/*.md  →  .kb/README.md
                     .kb/_templates/atom.md
                     .kb/decisions/README.md      .kb/design/README.md
                     .kb/open-questions/README.md .kb/playbooks/README.md
                     .kb/product/README.md        .kb/reference/README.md
                     .kb/_intake/README.md + the seven files of this wave
```

**There are zero atoms.** Seven layer READMEs, one template, and the intake. So:

- **No accepted decision atom exists, and none can be conflicted with or superseded.** Every
  authority-rule question in this wave collapses to the same answer: nothing in `.kb/` binds
  anything yet, so no claim can drift an accepted decision, and no claim may be filed as one
  either (see `01`, and the ADR-authorship note in `02`).
- **No merge target exists.** `merge_existing` was unavailable for every claim in the wave, not
  declined. The default bias toward merge was therefore discharged *within* the wave instead:
  the question became "which of these twenty-nine claims are the same knowledge", and it is
  answered below.
- The READMEs are still authority — they are the layer contracts the pipeline routes by, and
  they scored as the highest lexical matches for several claims. A README is not an atom, so it
  is never a merge target; where a claim restates one, the disposition records the alignment
  and the atom is grounded rather than duplicated.

Two READMEs did real adjudicating work:

| README | What it settled |
| --- | --- |
| `.kb/decisions/README.md` | Already carries the repair-versus-gap test almost verbatim ("a correction is a repair if the set of implementations the decision admits is unchanged"). The frozen-clause lesson **aligns** with the corpus rather than introducing it. |
| `.kb/reference/README.md` | The pointer case ("names it and says what it establishes; it does not reproduce it") and the dating rule are exactly what the pressure-test file asks for in its own first section. Placement was over-determined. |
| `.kb/playbooks/README.md` | "A one-off … is either a `reference` atom or nothing at all" is what moved the `ANCHOR_SLACK` defect out of the anchoring playbook (CL-3) and into the census (CL-1). |

## Provenance check

Every repo path the intake cites was tested against this worktree before being carried into a
`source_paths` proposal. All resolve: `spec/SPECIFICATION.md`, `RUNBOOK.md`, `CLAUDE.md`,
`xtask/src/spec_trace.rs`, `xtask/src/lint_constitution.rs`,
`crates/happenstance-testkit/src/{concurrency,model}.rs`, `standards/rust/`,
`references/evaluation/{phase-4-5-reconciliation,review-citation-drift}.md`.

One citation had already rotted in the eleven days it existed:
`lesson-anchoring-citations-in-a-long-lived-document.md` places `const ANCHOR_SLACK: usize = 12`
at `xtask/src/spec_trace.rs:381`; it is at `:391` today (grep, 2026-08-10). The values 12 and 10
are otherwise exactly as claimed. The census atom carries the verified location, not the intake's.

## Scoring method

No deterministic engine exists in this repo, so the score is stated rather than computed: a
0–100 judgement of **subject identity** — would a reader looking for one claim expect to find
the other in the same atom — cross-checked against theme-keyword overlap from the extract
digests. Thresholds used, and applied consistently across the wave:

- **≥ 80 — same knowledge.** Collapse into one atom. Never two atoms.
- **50–79 — same subject, different method or different settling condition.** Two atoms, linked.
  The reason must be written down.
- **< 50 — related.** A `related` link and nothing else.

## Cross-file clusters

| Cluster | Claims absorbed | Files | Score | Disposition |
| --- | --- | --- | --- | --- |
| **CL-1** — the reconciliation census | pressure-test `c1`–`c7`; frozen-clause `C4`; anchoring `C3`; the shared 358/69/289 and 200/139 figures quoted by both citation lessons | 3 | **92** | one `reference` atom |
| **CL-2** — checker obligations | referent-lesson `c1`, `c2` | 1 | — | `playbook` |
| **CL-3** — the anchoring technique | anchoring `C1`, `C2` | 1 | — | `playbook` |
| **CL-4** — the ratchet | gate-lesson `C1`–`C5` | 1 | — | `playbook` |
| **CL-5** — repair versus amendment | frozen-clause `C1`, `C2`, `C3`; `C5` as links only | 1 | — | `playbook` |
| **CL-6 … CL-11** — the six gaps | gaps `gap-1` … `gap-6b` | 1 | — | six `open_question` atoms |
| **CL-12** — nothing owns the reconciliation | post-phase `claim-1` … `claim-8` | 1 | — | one `open_question` atom |

### CL-1 is the wave's one real merge, and it is a three-way one

Three files independently state the same measurement of the same document on the same day.

| Pair | Score | Evidence |
| --- | --- | --- |
| pressure-test `c5` ("200 clauses (139 FROZEN, …), 358 citations checked (69 anchored …)") vs frozen-clause `C4` ("200 numbered clauses … 139 of the 200 are `[FROZEN]` today") | **95** | Identical numbers, identical instrument (`cargo run -p xtask -- spec-trace`), identical date. The extract digest proposed a second atom at `.kb/reference/specification-frozen-clause-census.md` for `C4`; that would have been the exact failure the reference README names — "two copies of a measurement is two things to update and one that quietly goes stale". **Rejected.** |
| pressure-test `c5` vs anchoring `C1`/referent `c1` ("69 of 358 … 81% verified for addressing only") | **88** | Same three numbers. The playbooks need them as evidence, not as their own claim, so the census owns them and both playbooks cite the atom. |
| anchoring `C3` (`ANCHOR_SLACK` 12 vs 10) vs CL-1 | **62** | Not the same claim, but a dated single-file fact from the same pass, and explicitly "a one-line repair and not an ADR". The playbooks README bars a one-off from a playbook; the reference README accepts a dated defect at a commit. Folded into the census as a residual-defect line, and the anchoring playbook links rather than restates. |

Everything else in the pressure-test file is pointer material (`c1`, `c2`, `c7`) or class-count
material (`c3`, `c4`, `c6`) with no second home in the wave.

### CL-2 versus CL-3 — the closest call that was *not* merged

| Pair | Score | Verdict |
| --- | --- | --- |
| referent-lesson vs anchoring-lesson | **58** | **Two atoms.** |

They share a subject (the same checker, the same 358 citations, the same two commits) and the
first even points at the second. But they answer different questions and stop holding under
different conditions:

- **CL-2** is about *scope legibility* — print the denominator, name the population you declined,
  suspect a check that has never failed. It generalises to any cross-reference checker (links,
  IDs, schema `$ref`s, traceability) and is falsified by nothing in the anchoring mechanism.
- **CL-3** is about *one mechanism* — P1/P2, the windowed subject search, explicit versus derived
  anchors, four measured attempts, the decline-don't-guess discriminator. It stops holding when
  the corpus is being authored rather than retrofitted, which has nothing to do with coverage
  reporting.

Merging them would produce a ~1,600-word atom with two claim-sets, against the ~300–900-word
atomicity bar, and would bury the portable half inside the mechanism half. They are linked
instead: CL-3 `depends_on` CL-2, because the anchoring mechanism is how CL-2's first obligation
is discharged here.

### CL-11 — 6a and 6b kept together

| Pair | Score | Verdict |
| --- | --- | --- |
| gap 6a (ES-7) vs gap 6b (VT-9) | **74** | **One atom**, per the intake's own "six atoms" instruction, which permits but does not require the split. |

Different clauses, different entanglements (ADR-0001's lift condition; the Workers schedule) —
but one question shape, one settling move ("lift to `[FROZEN]`, or restate the falsifier so it
discriminates"), and one transferable observation the intake asks be kept on both if split. Below
80, above 50: two atoms would be defensible, one atom is better, because a reader who fixes the
falsifier-hygiene problem fixes both at once. Recorded so a later wave can split it cheaply.

### The dedup that was refused

`gaps-owed-a-decision.md` opens with **"this file must yield six atoms, not one. Do not merge
them."** Scored on subject identity the six run 15–40 against each other (shared provenance,
nothing else): gaps 3 and 4 are the closest at **40** — both PS-layer, both owned by phase 6,
both "the MUST is narrower than the rule table assigns it", and the intake itself says whoever
takes one should check the other. Still below threshold: they are settled by different evidence
about different clauses. Two atoms, cross-linked, with the systematic-§4.11 suspicion written on
both.

### Cross-file `related` links, scored but not merged

| Pair | Score | Why linked only |
| --- | --- | --- |
| gap 6 (CL-11) ↔ post-phase reconciliation (CL-12) | 38 | The intake connects them explicitly: a falsifier that has already occurred is undetectable by a gate step and wants a phase-exit reading task. Different questions; CL-12 links outward. |
| post-phase reconciliation sub-question 2 ↔ ratchet playbook (CL-4) | 35 | Both turn on the self-referential-count failure mode at `xtask/src/spec_trace.rs:1965-1967`. CL-4 states the rule; CL-12 is bound by it. |
| gaps 1 and 2 ↔ ratchet playbook (CL-4) | 45 | Those two gaps *are* the two `UNCLAIMED_PENDING_ADR` entries the ratchet holds. The playbook is the mechanism; the gaps are the owed decisions. Merging would file a live question in a guideline atom. |
| frozen-clause playbook (CL-5) ↔ gaps 3, 4, 6 | 42 | CL-5 uses PS-1, PS-19, ES-7 and VT-9 as worked examples of "a gap, not a repair". The examples stay; the questions live in `open-questions/`, per the decisions README's "a decision not yet taken" rule applied one layer over. |
