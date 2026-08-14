# Wave `2026-08-13-projection-adrs` — corpus match

What already exists in `.kb/`, what each incoming claim could have merged into, and the score
behind every disposition. Read this before `02-placement-and-adjudication.md`: the placements
there are only defensible because of what this file establishes about the corpus and about the
intake.

This is the first wave to run against a **populated** decision layer. `2026-08-10-intake` left
`.kb/decisions/` empty on purpose; `2026-08-10-intake-2` filled it with seventeen atoms and
closed with the sentence *"A later wave importing ADR-0017 onwards may well need the real
thing"* — meaning `supersede` as an operation. This is that later wave. It does not need it,
and `02` says why in terms rather than by omission.

## The corpus, as verified

```
Glob .kb/**/*.md   →  36 atoms + 8 layer READMEs + .kb/README.md + _templates/atom.md
                      + .kb/_intake/README.md + the four files of this wave
ls .kb/decisions   →  17 atoms (0001–0016, 0029), all kind: decision
```

| Layer | Atoms | Bearing on this wave |
| --- | --- | --- |
| `decisions/` | **17**, ADR-0001–0016 + ADR-0029 | Four are cited by the intake; **none is edited, none is superseded** |
| `open-questions/` | 19 | **Three are merge targets**; six more are named by the intake as explicitly *not* to be touched |
| `maps/` | 3 (`domain`, `decision`, `open-questions-index`) | All three inherit work from this wave |
| `reference/` | 4 | One is the precedent the sweep's conditional atom would follow |
| `playbooks/` | 6 | `repair-frozen-clause`, `one-decision-per-adr-title` are cited by all three ADRs |
| `concepts/`, `governance/` | 1 each | Neither is touched |
| `product/`, `design/`, `narratives/`, `roadmaps/` | 0 / do not exist | Nothing routes there |

For authority purposes:

- **Seventeen accepted decision atoms now exist, and the intake touches four of them by name** —
  `kb-decision-0003` (opaque payloads), `kb-decision-0007` (the runner split and the per
  `(store, ProjectionId)` checkpoint), `kb-decision-0008` (one derivation, and the `Batch` GAT),
  `kb-decision-0013` (the global visibility invariant). Every one of the four is *cited*, and
  **none is contradicted**. The single closest call is ADR-0017's removal of the `Batch` GAT, and
  it is not a conflict at all — see "The GAT is not a conflict" below.
- **Three open questions are merge targets**, all three `authority_tier: note`, so amending them
  is permitted and no immutability rule is in play.
- **No map atom needs inventing.** All three exist and all three carry an "Adding a …" section.

## Provenance check

Each of the four intake files is a **staging note describing an op**, not the knowledge itself —
the opposite shape from the last wave, whose seventeen files were byte-identical copies of the
ADRs. Each names its long-form record, and each record was verified present:

```
references/adr/0017-what-a-projection-batch-owns.md         ✓
references/adr/0018-returning-a-projection-to-never-run.md  ✓
references/adr/0019-what-happens-when-apply-fails.md        ✓
references/evaluation/ps-clause-pairing-sweep.md            ✓  (registered in that dir's README)
```

Every other path the plan proposes was tested against this worktree and resolves:
`references/adapter-shapes.md`, `crates/happenstance-ladybug/src/live_handle.rs`,
`spec/SPECIFICATION.md`, `.bklg/from-contract-to-published-library/**` (for
`unstable-projection-gate-and-clause-disposition`, which is a **project**, not an atom —
`grep` confirms it at `.bklg/.../buffering-conformant-variant/spec.md:126` and in the Postgres
project's spec). One path in the intake does **not** resolve and is deliberately not
manufactured: `experiments/live-handle-projection-batch/`. `ls experiments/` returns
`position-visibility`, `rustc-ice-gat-foreign-trait`, `wire-format`. The move is a code change
this wave does not make and does not record as done — the atom states the disposition ADR-0017
decided, in ADR-0017's voice, and `unresolved` carries the fact that the directory is not there
yet.

## Scoring method

Unchanged from both previous waves, deliberately — a threshold that moves between waves is not a
threshold. A 0–100 judgement of **subject identity**: would a reader looking for one claim expect
to find the other in the same atom.

- **≥ 80 — same knowledge.** Collapse into one atom. Never two.
- **50–79 — same subject, different method or different settling condition.** Two atoms, linked,
  and the reason written down.
- **< 50 — related.** A `related` link and nothing else.

Two rules carried forward and one added:

- **A supersession chain is never a merge** (wave 2's rule). Not exercised this wave.
- **A README is not a merge target** (wave 1's rule). Exercised: three intake files quote
  `.kb/open-questions/README.md`'s "do not rewrite a question into its own answer". That rule is
  *obeyed*, not absorbed into an atom.
- **New this wave: a decision atom naming a gap is not the same knowledge as the open question
  that owns the gap.** ADR-0018 names PS-19's pairing defect; `kb-open-question-ps-19-scope-narrower-001`
  owns it. Those score 85 on subject identity and are still two atoms, because one is a
  commitment's scope boundary ("this ADR repairs nothing") and the other is an unanswered
  question with sub-questions and an owner. Collapsing them would file a live question inside a
  decision, which the decisions README bars in terms.

## Merge candidates against the existing corpus

| Existing atom | Incoming claims | Score | Disposition |
| --- | --- | --- | --- |
| `kb-open-question-projection-batch-no-apply-001` | `0017-c11` (resolve, do not delete), and the whole of ADR-0017's claim 3 as its answer | **95** | **merge_existing** — Op 4. This atom's own summary says *"Owned by phase 6, which freezes ProjectionStore and carries ADR-0017 for what a projection batch owns and what vocabulary writes into it."* ADR-0017 is that decision. `status` → `superseded`; body verbatim; dated annotation. |
| `kb-open-question-ps-19-scope-narrower-001` | `0018-C2` (named, scoped out, attributed, not repaired) **and** sweep `2a`/`2b`/`2c` (sub-question 2 answered, finding reproduces) | **92** | **merge_existing** — Op 5. **The wave's one true cross-file merge**: two intake files, one destination atom, one annotation. |
| `kb-open-question-ps-1-no-progress-obligation-001` | sweep `1a`/`1b`/`1c` (sub-question 3 answered `isolated`; the body's "no clause's MUST" sentence refuted by PS-23) **and** `0019-c4`'s cross-cutting note (the defect is a habit, not a table artefact) | **90** | **merge_existing** — Op 6. Second cross-file merge. |
| `kb-decision-0008` | ADR-0017's removal of the `Batch` GAT | 70 | **link only** (`depends_on`). Not a merge and not a supersession — 0008 *delegated* this. See below. |
| `kb-decision-0007` | ADR-0018's scope half (PS-17), ADR-0019's isolation fact | 60 | **link only** (`depends_on`). Both cite it rather than re-deriving it; that citation-not-re-derivation is itself one of ADR-0018's four graded claims. |
| `kb-decision-0013` | ADR-0018's boundary-scoped checkpoint, filed not absorbed | 35 | **link only**, and `kb-open-question-global-vs-boundary-visibility-001` is left untouched, exactly as the intake instructs. |
| `kb-open-question-adr-status-vocabulary-001` | `0017-c2` (the status convention) | 85 lexical, **0 actionable** | **no change.** The intake says in terms: do not answer this question by acting on it. The convention is applied and re-stated in `02`'s standing choices; the atom is not edited. |
| `kb-reference-phase-4-5-spec-reconciliation-001` | sweep `1d` (mint a reference atom for the sweep?) | 55 | **precedent, not a merge target.** Declined this wave — `02`, Adjudication 4. |
| `kb-playbook-repair-frozen-clause-001` | five gap-namings across three files (PS-8, PS-13, PS-19, PS-28, PS-29) | 45 | **link only.** These are *instances* of the playbook's method; the playbook already carries worked examples and four more would grow it past atomicity for no new method. Identical to wave 2's ruling on the same atom. |
| `kb-open-question-projection-id-unvalidated-001`, `…-cf-40-ownership-001`, `…-post-phase-reconciliation-001`, `…-global-vs-boundary-visibility-001` | named in three "Not proposed" sections | — | **no change**, and the wave records that it was asked not to touch them rather than simply not touching them. |

### The GAT is not a conflict, and this is the wave's most important classification

ADR-0017 commits to `type Batch;` — owned, **no lifetime parameter** — which removes the GAT that
`kb-decision-0008` describes. On subject identity that scores 70 and it *looks* like the wave's
one candidate for `supersede`. It is not, and the ground is in ADR-0008's own accepted body:

```
.kb/decisions/0008-one-derivation-for-both-ports.md:80-81
  Bad and load-bearing for phase 6: a `ProjectionStore` provided method may never hold its
  batch across a suspension point, and phase 6 must decide whether the GAT survives
```

ADR-0008 records the GAT as a **cost it measured and a question it delegated to phase 6**, not as
a commitment it made. ADR-0017 is phase 6 answering it. Nothing in 0008's body becomes false, so
there is nothing to supersede and nothing to flip — the correct edge is `depends_on`, and the
correct label is `extends`. Filing this as a supersession would flip an accepted atom's status on
the strength of a question it asked, which is the inverse of the immutability rule's purpose.

## Cross-file clusters

Four intake files, thirty claims. The clusters below are where two or more **different files**
carry the same knowledge; everything not listed is single-file.

| Cluster | Claims absorbed | Files | Score | Disposition |
| --- | --- | --- | --- | --- |
| **CL-1** — PS-19's pairing defect: named by a decision, owned by a question | `0018-C2`, sweep `2a`, `2b`, `2c` | **2** | **92** | **one** amendment to `kb-open-question-ps-19-scope-narrower-001` (Op 5) |
| **CL-2** — the rule written to the clause's *intent* rather than its *sentence* | sweep `1a`'s first qualification, `0019-c4`'s closing note | **2** | **88** | **one** amendment, to `kb-open-question-ps-1-no-progress-obligation-001` (Op 6); ADR-0019's atom **links** rather than restating |
| **CL-3** — the sweep returned `isolated`, so no ADR widens its scope | `0017-c10`, `0018` §PS-19, `0019` §the-sweep's-finding, sweep `1a`/`2a` | **4** | 75 | **not one atom.** Each decision names the rows in *its own* clause range; the family verdict is owned by Op 6 and cited. See below. |
| **CL-4** — the wave-id and `status`-vocabulary orchestration notes | `0017-c0`, `0017-c2`, and the two siblings' one-line inheritance of them | 3 | 100 | **no atom at all.** Orchestration guidance for the runner; recorded in `02`'s standing choices. |
| **CL-5** — "Not proposed": no `crates/**`, no `spec/` edit, no marker moved, no `[FROZEN]` line edited | `0017-c13`, `0018-C6`, `0019-c7`, sweep `4` | **4** | 100 | **no atom.** A negative scope statement is a guardrail on the integrate pass, not knowledge. Enforced in `02` and re-checked in the wave summary. |

### CL-3 is the cluster that most wanted to be one atom, and must not be

All four documents record the same sentence — the sweep returned **isolated** over 37 clauses,
so no phase-6 ADR widens its scope. The temptation is one `reference` atom carrying the verdict
that all five destinations cite. Refused on two grounds, and the second is the one that decides
it:

1. The reference README bars *"a mirror that someone is expected to keep current"* and the sweep
   is already registered evidence at a stated commit (`2136dde`), citable by path today.
2. **What each document does with the verdict differs, and the difference is the content.**
   ADR-0017 names PS-8 and PS-13 and defers; ADR-0018 names PS-19, attributes the repair to a
   backlog project and records one *wording* ambiguity it routes rather than fixes; ADR-0019
   names PS-29 and PS-28 and distinguishes `defective` from `undetermined`. Those are three
   different scope boundaries on three different decisions. A shared atom would carry the verdict
   and none of the three uses of it, and each decision would still have to state its own — two
   copies of the cheap half and one of the expensive half.

The family verdict lands **once as an answer**: on `kb-open-question-ps-1-no-progress-obligation-001`,
whose sub-question 3 asked for exactly this scan. Everything else cites
`references/evaluation/ps-clause-pairing-sweep.md` by path and links that atom.

## The dedup that was refused: three ADRs stay three atoms

| Group | Score | Why not merged |
| --- | --- | --- |
| ADR-0017 + ADR-0018 + ADR-0019 — the phase-6 projection-port freeze | 65–72 | One phase, three RUNBOOK rows (`RUNBOOK.md:296`, `:297`, `:298`), three human sign-offs, three long-form records of 500+ lines each. Identical in shape to wave 2's refusal to merge the five phase-4 ADRs, and `adr_id` plus the supersede pair have to identify one atom. |
| ADR-0018 §1 + ADR-0017 §3 — `probe_delete_all` | 55 | ADR-0018 *uses* the probe ADR-0017 minted. That is a dependency, not shared knowledge: `depends_on: kb-decision-0017`. |
| ADR-0019 §2 + ADR-0017 §4 — `rollback` stays on the port | **80** | The closest call in the wave, and still two atoms: ADR-0017 puts `rollback` on the port *because Rust has no async `Drop`*; ADR-0019 says it **must survive the port change** *because `AssertUnwindSafe` in a fan-out runner is a lie without it*. Same method, two different obligations, taken a document apart. ADR-0019's atom states its own reason and links; it does not restate PS-7/PS-8. |

The converse refusal — splitting one ADR across two or three atoms because its halves have
different strengths — is also declined, and for ADR-0018 that is a live temptation: its own
record grades four claims at three strengths and warns against levelling them. The answer is a
strength table **inside** one atom, not four atoms. `02`, standing choice 3.
