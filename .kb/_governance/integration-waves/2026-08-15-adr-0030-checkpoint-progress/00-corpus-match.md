# Wave `2026-08-15-adr-0030-checkpoint-progress` — corpus match

What already exists in `.kb/`, what each incoming claim could have merged into, and the score
behind every disposition. Read this before `02-placement-and-adjudication.md`: the placements
there are only defensible because of what this file establishes about the corpus and about the
intake.

This is a **one-file wave** — a single staging note, eight claims — which removes the previous
three waves' hardest job (cross-file dedup) and leaves the harder one in plain sight: three of the
eight claims name a destination that is an **accepted decision atom**, and an accepted decision
atom cannot absorb anything. Two of those three are adjudicated to *no operation at all*, and the
third to a new `open_question`. `02` states each in terms rather than by omission.

## The corpus, as verified

```
ls .kb/decisions      →  20 atoms (0001–0019, 0029), every one kind: decision
ls .kb/open-questions →  19 atoms, all authority_tier: note
ls .kb/reference      →   4 · playbooks 6 · maps 3 · concepts 1 · governance 1
ls .kb/product .kb/design → READMEs only; nothing routes there this wave
                         54 atoms + 8 layer READMEs, before this wave
```

| Layer | Atoms | Bearing on this wave |
| --- | --- | --- |
| `decisions/` | **20**, ADR-0001–0019 + ADR-0029 | One atom is **added** (ADR-0030). **None is edited, none is superseded, no `status` flips** |
| `open-questions/` | 19 | **Two are resolved** (metadata flip + dated annotation, body verbatim); **one is created** |
| `maps/` | 3 | `open-questions-index` inherits two row flips; `decision-map` inherits one new row + a wave section; `domain-map` inherits one sentence |
| `reference/` | 4 | **One is added**; one (`phase-4-5-spec-reconciliation`) was considered as the merge target and declined |
| `playbooks/` | 6 | `repair-frozen-clause` and `one-decision-per-adr-title` are both cited *by name* in ADR-0030's rejected alternatives — `related` edges, not merges |
| `governance/` | 1 | `kb-governance-referent-not-reasoning-001` is the atom that makes claim 6 an open question rather than an edit. Cited, not touched |
| `concepts/` | 1 | Untouched |

For authority purposes:

- **Twenty accepted decision atoms exist and the intake touches five of them by name** —
  `kb-decision-0007` (the runner split, and the sentence PS-32 says is wrong),
  `kb-decision-0017` (the projection batch, and the range PS-8/PS-13 sit in),
  `kb-decision-0018` (reset, and PS-19's clause range), `kb-decision-0019` (apply failure, and
  PS-28/PS-29's disposition), and `kb-decision-0030` itself, which does not exist yet. Four are
  *cited*; **none is contradicted, none is edited, and none is superseded.**
- **The one candidate for `supersede` in the whole wave is ADR-0007, and this wave declines to
  perform it** — see `02`, Adjudication 3. Declining it is not silence: it becomes an atom.
- **Two open questions are resolved rather than amended**, which is a first for this corpus. Both
  are `authority_tier: note`, so the metadata flip is permitted; the precedent for the exact shape
  is `kb-open-question-projection-batch-no-apply-001`, flipped to `superseded` by the
  2026-08-13 wave with its body left verbatim.

## Provenance check

The single intake file is a **staging note describing ops**, not the knowledge itself — the same
shape as the four files of `2026-08-13-projection-adrs`. Every path it names was tested against
this worktree:

```
references/adr/0030-the-checkpoint-reports-the-commits-that-happened.md  ✓  158 lines
references/evaluation/ps-clause-pairing-sweep.md                          ✓
references/evaluation/PRESSURE-TEST.md §3.4 (:203-232)                    ✓  the compiled pump
references/adr/0007-projection-runner-decodes.md:37                       ✓  the wrong sentence
references/adr/0017-what-a-projection-batch-owns.md:356-361               ✓  correction recorded owed
spec/SPECIFICATION.md:5679 (PS-32), :5699 (Rejects)                       ✓
xtask/src/spec_trace.rs:1750 has_suite, :2363 the_projection_family_…     ✓  and :2387 SY abstains
standards/rust/70-rustdoc-obligations.md                                  ✓  (claim 7's instrument)
```

Two things the check turned up that the intake does not say, and both change a placement:

1. **`spec/SPECIFICATION.md:5714` names this very intake file** as "the staging note for the next
   wave" for PS-32. The specification is asking, in a `[FROZEN]` clause, for this wave to carry the
   correction forward. That is why claim 6 becomes an atom instead of a third `unresolved` line.
2. **`kb-decision-0007`'s atom body does not contain the wrong sentence.** The claim
   *"cannot be written against the port as it stands — in either crate"* lives at
   `references/adr/0007-…:37` — the long-form record — and the atom's own Context paraphrases the
   split without asserting it. This sharpens claim 6 considerably: what is defective is the
   *record*, and the atom that would have to be superseded to correct it carries no defect of its
   own. `02`, Adjudication 3.

## Scoring method

Unchanged from all three previous waves — a threshold that moves between waves is not a threshold.
A 0–100 judgement of **subject identity**: would a reader looking for one claim expect to find the
other in the same atom.

- **≥ 80 — same knowledge.** Collapse into one atom. Never two.
- **50–79 — same subject, different method or different settling condition.** Two atoms, linked,
  and the reason written down.
- **< 50 — related.** A `related` link and nothing else.

Three rules carried forward and one added:

- **A supersession chain is never a merge** (wave 2). Not exercised — nothing is superseded.
- **A README is not a merge target** (wave 1). Exercised twice: the open-questions README's
  *"the answer is a new atom, and the record of the question stays"* governs Ops 2 and 3, and the
  reference README's dating rule governs Op 5. Both are **obeyed**, not absorbed.
- **A decision naming a gap is not the same knowledge as the question that owns the gap** (wave 3).
  Exercised in reverse this wave: ADR-0030 *answers* two of those questions, so for the first time
  the edge is a resolution rather than a citation.
- **New this wave: an accepted decision atom is not a merge target, however high it scores.**
  Claim 5 scores 80 and 85 against `kb-decision-0017` and `kb-decision-0019` and produces **no
  operation**, because the only honest way to "merge" into an accepted decision is to discover that
  it already says the thing — which is exactly what the verification found. A merge disposition
  with an empty edit is a no-op wearing an op's clothes, and recording it as an op would put a
  false mutation in the manifest.

## Merge candidates against the existing corpus

| Existing atom | Incoming claim | Score | Disposition |
| --- | --- | --- | --- |
| `kb-open-question-ps-1-no-progress-obligation-001` | claim 2 — resolved by ADR-0030; the "clause of its own" candidate wins; the misfiled-onto-PS-23 amendment repeated | **100** | **merge_existing** — Op 2. `status` → `superseded`; body verbatim; dated annotation; `related` += `kb-decision-0030` |
| `kb-open-question-ps-19-scope-narrower-001` | claim 3 — resolved by the same record; scope kept, unseen-id half moves to PS-38's second sentence | **100** | **merge_existing** — Op 3, same shape |
| `kb-map-open-questions-index-001` | claim 4 — both rows Open → Resolved | 100 | **`mapsImpact.openQuestionIndex` on Ops 2 and 3**, not an op of its own — wave 3's convention, and the index's own *"Adding an entry"* section owns the wording, which is why the label lands as `Superseded` (`02`, Adjudication 7) |
| `kb-decision-0017` | claim 5's PS-8 and PS-13 rows | **80** | **no operation.** Verified: the atom already records both as gaps it names and repairs not. Nothing to add, and its body is immutable |
| `kb-decision-0019` | claim 5's PS-28 and PS-29 rows | **85** | **no operation**, same verification. `02`, Adjudication 2 |
| `kb-decision-0007` | claim 6 — the Context sentence PS-32 rejects | 55 | **not a merge and not a supersession this wave.** The atom is accepted and immutable, and this wave has no ADR authorising the supersession. Routed to a new `open_question` — Op 4 |
| `kb-governance-referent-not-reasoning-001` | claim 6's *"must be a superseding atom, never an edit"* | 42 | **link only.** Claim 6 is an *instance* of the governance atom's test, not a new rule; the atom already carries three worked instances and a fourth adds no method |
| `kb-reference-phase-4-5-spec-reconciliation-001` | claim 8 — `has_suite` is a per-family switch | 55 | **precedent, not a merge target** — and refused a second time, now on the reference README's dating rule. That atom is a census *at 2026-08-10*; appending a 2026-08-15 fact would break the snapshot property that makes it citable. **create_new** — Op 5 |
| `kb-open-question-provisional-falsifiers-001` | claim 8's "same defect shape one level up" | 48 | **link only.** The intake says *"the same defect shape … one level up"*, which is a resemblance, not shared knowledge: one is an unanswered question about two clause markers, the other a settled fact about a tool. `related` edge from Op 5 |
| `kb-playbook-repair-frozen-clause-001` | ADR-0030's whole rejected-alternatives section | 45 | **link only** — third wave running. ADR-0030 is the playbook's method applied, and the playbook already carries worked examples |
| `kb-playbook-one-decision-per-adr-title-001` | ADR-0030's fourth rejected alternative, cited by path in the record | 40 | **link only.** The record cites the playbook *as the bar it was judged against*; that is a citation, and the playbook gains nothing from a fourth instance |
| `kb-open-question-adr-status-vocabulary-001` | ADR-0030 is accepted-and-provisional (PS-38 is `[PROVISIONAL]`) | 85 lexical, **0 actionable** | **no change**, for the third wave running. The qualification and its falsifier go in `summary`'s first clause; the question is not answered by acting on it |

## Cross-file clusters

**None, and that is a structural fact rather than a lucky one.** One intake file cannot contain a
cross-file cluster. The dedup work that a multi-file wave spends on collapsing duplicates was spent
here instead on the *intra*-file question: whether claims 1, 2 and 3 are one atom or three.

| Cluster | Claims | Score | Disposition |
| --- | --- | --- | --- |
| **CL-1** — PS-38 and the two questions it answers | 1, 2, 3 | **88** | **three atoms, not one.** See below |
| **CL-2** — the open-questions index rows | 4 | 100 with 2 and 3 | **no atom.** Folded into Ops 2 and 3 as `mapsImpact` |
| **CL-3** — "recorded, not repaired": PS-8, PS-13, PS-28, PS-29 | 5 | 80–85 against two accepted decisions | **no atom, no op.** The knowledge has owners and they already state it; ADR-0030's own atom carries the one-line scope boundary in `## What this does not repair` |
| **CL-4** — a documentation obligation with no instrument (PS-3/PS-31/PS-36) | 7 | — | **no atom.** The source withholds mandate in terms; carried in `unresolved` so declining it stays visible |

### CL-1 is the wave's one real temptation, and it must stay three atoms

ADR-0030 exists *because of* the two questions, answers both, and its `summary` will restate the
finding each question records. On subject identity that is 88, and 88 is above the collapse
threshold. It stays three atoms on the open-questions README's instruction, which is not a
preference but the layer's contract:

```
.kb/open-questions/README.md:42-45
  When the question is answered, the answer is a new atom — and the record of the question
  stays. … Do not rewrite a question into its own answer: the value of the record is that it
  shows the state of knowledge on the day the choice was made.
```

Collapsing CL-1 would delete two records of what was *not* known on 2026-08-10 and 2026-08-13 —
including PS-1's refuted sentence, which is the corpus's only worked example of a finding surviving
the refutation of the argument that raised it. The threshold measures subject identity; the README
decides what a layer is *for*, and where they disagree the README wins. This is the same shape as
wave 3's ruling that a decision naming a gap and the question owning it stay two atoms, run
forward: now that the decision *answers* rather than *names*, the answer is one more atom, not one
fewer.

## The dedup that was refused, and the one that was not offered

| Group | Score | Ruling |
| --- | --- | --- |
| ADR-0030 vs ADR-0017/0018/0019 (all phase 6, all projection-store) | 60–70 | **Four atoms.** ADR-0030 is `unscheduled` within phase 6 on ADR-0029's precedent (`RUNBOOK.md:287`), has its own 158-line record and its own `adr_id`, and mints a clause the other three do not touch. Identical to wave 3's refusal to merge three phase-6 ADRs |
| ADR-0030's two sentences (advance; unseen id ⇒ `NeverRun`) as two clauses / two atoms | — | **Not offered, and correctly.** The record settles it at `:88-90`: *"Both sentences are one proposition … the first says a commit is visible in it; the second says nothing else is."* Splitting would produce two atoms each carrying half a proposition, and the `adr_id` would have to name one of them |
| Claim 6 (PS-32) folded into ADR-0030's atom as a *"what this does not repair"* line | 35 | **Refused.** It would file a live, unowned obligation inside a decision — the inverse of the rule wave 3 established, and barred by `.kb/decisions/README.md`. ADR-0030's record already declines PS-32 by name; the atom repeats that boundary in one sentence and the question owns the substance |
