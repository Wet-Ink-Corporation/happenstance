---
item: HS-P0025
stage: storymap
created: 2026-08-17T03:21:40.007Z
updated: 2026-08-17T03:21:40.007Z
template_sig: 1c63534a
rendered_sig: 33d8d5e1
---

# Story Map — The Durable, Reconciled Audience

Eleven stories in four milestones. The spine is [`project.md`](project.md)'s AC-001 … AC-018;
the slicing follows the pipeline the architecture brief fixes
([`_decomposition.md`](_decomposition.md), "## Architecture brief", §5 "Data flow, and the
orderings that are load-bearing"), whose four non-negotiable orderings — merge before
everything, ingest before validation, validation before the links, everything committed
before the final gate — are exactly the cross-milestone dependency edges below.

This project ships no Rust. Its diff is confined to `.kb/product/`, `.kb/open-questions/`,
the two files under `.kb/maps/`, `.kb/_intake/` and this project's own folder under
`.bklg/docs-that-teach/durable-audience-closeout/`, plus the merge-forward
(same brief, §1 "The seam, stated as a diff surface"). A story that proposes to edit
`crates/`, `spec/`, `xtask/`, `standards/` or `docs/` has left this project.

## Backbone

The four activities, left to right in the order a reader of the finished tree would
reconstruct them.

| | Activity | The outcome it produces | Stories |
| --- | --- | --- | --- |
| **A1** | **Assemble the tree** | One named commit that everything downstream is observed against, and a written statement that the specification's documentation MUSTs are still completely pinned | `merge-forward-baseline`, `post-merge-clause-completeness` |
| **A2** | **Adjudicate the audience** | The written decisions — every persona/journey pair adjudicated, every charter open question disposed of, every design tension accounted for | `reconciliation-ledger`, `charter-open-question-disposition`, `design-tension-audit` |
| **A3** | **Promote and mount the audience** | Durable `.kb/product/` atoms that the *next* initiative's charter author can frame acceptance criteria from, reachable from the maps, the item links and the closure | `staged-audience-payload`, `audience-ingest-wave`, `product-layer-mounting` |
| **A4** | **Re-observe the Definition of Done** | Fifteen fresh observations on the assembled tree, including a gate seen to fail and recover, and the terminal gate green on the exact tree that carries them all | `dod-scenario-ledger`, `scenario-two-fault-injection`, `terminal-gate-run` |

The audience for A3 is **not** the three documented personas — they are its subject.
Its reader is the next initiative's charter author (U1) and the closeout reviewer (U2)
([`_decomposition.md`](_decomposition.md), "## UX brief", "Who this is actually for").

## Slices

Four milestones. Each is one working, integrated surface handed to a single implementer
context; cross-milestone edges are acyclic and run strictly left to right, M1 → M2 → M3 → M4.

| Milestone | Story | Archetype | One-line slice | depends_on | traces_to |
| --------- | ----- | --------- | -------------- | ---------- | --------- |
| `merged-tree-baseline` | `merge-forward-baseline` | foundation | Merge `initiative/from-contract-to-published-library` forward into this branch and record the resulting commit sha as the single named tree every downstream record, re-check and observation cites. | — | AC-012 |
| `merged-tree-baseline` | `post-merge-clause-completeness` | capability | Re-run `cargo xtask spec-trace` on the merged tree and write the set-difference statement saying whether the sibling added a documentation MUST absent from HS-P0020's pinned clause-id set, pinning or routing any addition. | `merge-forward-baseline` | AC-013, AC-012 |
| `closeout-adjudication` | `reconciliation-ledger` | capability | Author the complete pair-by-pair reconciliation record — one row per persona and per journey on both sides, each `merged` / `superseded` / `kept distinct` with a named reason — stating in prose above the ledger which merge-order case actually held, and naming the merged sha. | `merge-forward-baseline` | AC-001, AC-002, AC-012 |
| `closeout-adjudication` | `charter-open-question-disposition` | capability | Dispose of every question in the charter's `## Open questions for the planning team` — answered on the record, or staged as an `open_question` atom for this project's single ingest wave — and settle the evaluator question once, with its reasoning, marking the charter's line answered. | `reconciliation-ledger` | AC-017, AC-003 |
| `closeout-adjudication` | `design-tension-audit` | capability | Produce the DT-1 … DT-10 audit table over the merged tree: for each tension, the owning project, its `_design.md` resolution or recorded deferral, and — where it has neither — an explicit `gap` row rather than a silent pass. | `merge-forward-baseline` | AC-018 |
| `product-layer-promotion` | `staged-audience-payload` | capability | Author the staged persona and journey documents under `.kb/_intake/` — four slots per persona, the evaluator decision transcribed, all three evidence qualifications adjacent to the claims they qualify, and the blanket "none has been directly observed" replaced by naming the persona HS-P0024's session walked. | `reconciliation-ledger`, `charter-open-question-disposition` | AC-003, AC-006, AC-007 |
| `product-layer-promotion` | `audience-ingest-wave` | capability | Run the single closeout `/redkiln:kb-ingest` wave against an explicit file list that excludes `.kb/_intake/README.md`, landing persona atoms as `kind: concept` / journey atoms as `kind: playbook` — both `authority_tier: product` — plus HS-P0021's carried playbook payload, with resolving `source_paths`, a cleared intake directory and `redkiln validate --kb` green. | `staged-audience-payload` | AC-004, AC-005, AC-008, AC-009, AC-011 |
| `product-layer-promotion` | `product-layer-mounting` | capability | Mount every landed atom at all four points — an appended `##` section in `.kb/maps/domain-map.md` with no deletion line anywhere in the diff, reciprocal `related` / `depends_on` edges, `links.kb` on HS-P0025 via `redkiln record-links --atom`, and a `## Knowledge Harvest` row — and record the two-hop reachability walk from `.kb/README.md`. | `audience-ingest-wave` | AC-010, AC-011 |
| `dod-reobservation` | `dod-scenario-ledger` | capability | Create a fresh checkout off the merged branch and re-observe all fifteen initiative DoD scenarios there, recording per scenario the observer, the checkout path and sha, and what was seen — with "inherited from a sibling" inadmissible as an outcome. | `product-layer-mounting`, `post-merge-clause-completeness` | AC-014 |
| `dod-reobservation` | `scenario-two-fault-injection` | capability | Re-observe DoD scenario 2 in both halves: break the pinned page deliberately, capture the gate's failing-by-name output verbatim as the failing half's evidence, revert, and re-run green as the recovery half — with the broken edit never a committed state of the tree. | `dod-scenario-ledger` | AC-015, AC-014 |
| `dod-reobservation` | `terminal-gate-run` | capability | Run `cargo xtask ci` — the terminal `e2e` grain — last, on the exact tree that already carries the atoms, the maps, `links.kb`, the reconciliation record, the DT audit and the fifteen-scenario ledger, and record it green. | `scenario-two-fault-injection`, `product-layer-mounting` | AC-016 |

### Why these four and not eleven

- **`merged-tree-baseline`** is one surface because the merge and the re-check are the
  same act of asserting *what tree this is*: the sha the merge produces is the sha the
  clause-completeness statement must name. Splitting them yields a merge with nothing
  observed on it — the definition of an unconsumed foundation.
- **`closeout-adjudication`** is one surface because all three stories are written records
  under `.bklg/docs-that-teach/durable-audience-closeout/`, read by the same reviewer (U2)
  in the same pass, and because the evaluator decision made in the reconciliation is the
  decision `charter-open-question-disposition` marks answered.
- **`product-layer-promotion`** is one surface because the whole wave — staged sources in,
  atoms out, staged sources removed — lands as **one commit on its own branch** and is
  reviewed by merging (`.kb/_intake/README.md`; IQ-4). Staging without ingest is not an
  atom; an ingested atom that is not mounted is "a component rendered into no tree"
  ([`_decomposition.md`](_decomposition.md), "## Architecture brief", §2). "Build it" and
  "wire it in" are the same story here by construction.
- **`dod-reobservation`** is one surface because all three stories run in the *same fresh
  checkout* off the merged branch, and the fault injection is a live mutation of that
  checkout whose revert the terminal gate run proves.

### Notes the implementer needs at the story grain

- **The single highest-probability defect is the authority tier.** Every atom an author
  will look at while composing carries `authority_tier: note` or `guideline`; the product
  layer requires `product`. Read `.kb/product/README.md:6-13`, never the neighbour
  ([`_decomposition.md`](_decomposition.md), "## UX brief", and T1 in the architecture brief).
- **`redkiln validate --kb` cannot see `.kb/_intake/`** (`.kb/_intake/README.md:21-27`), so
  AC-011 green is not evidence for AC-009. `audience-ingest-wave` owns both and must
  observe them separately.
- **The counterpart does not exist in this tree.** `.bklg/` holds `docs-that-teach` and
  `support` only ([`_grounding.md`](_grounding.md), "Reconciliation counterpart does not
  exist in this tree"), so `reconciliation-ledger`'s **default drafting case** is the unrun
  one, with the adjudication table degrading to "no counterpart staged" rows — not a
  paragraph bolted on afterwards.
- **`require_ledger: true`** (`.redkiln/config.yaml:62-67`) binds every story above, since
  each declares AC-###. That per-story `_ledger.md` is distinct from
  `dod-scenario-ledger`'s fifteen-scenario re-observation ledger; the testing brief names
  the tier, the story's ledger names the `file:line`
  ([`_decomposition.md`](_decomposition.md), "## Testing brief", Notes).
- **No story authors or touches `.kb/decisions/`.** A reconciliation finding that wants a
  decision is a routed gap written in prose in the reconciliation record
  (`.kb/governance/rewrite-the-referent-never-the-reasoning.md`; AC-A09).
- **No story edits HS-P0021's page-need discipline text.** It rides
  `audience-ingest-wave` as payload; the seam is vehicle versus content
  (`.bklg/docs-that-teach/_decomposition.md:106-114`; AC-A10).

## Coverage

Every project acceptance criterion is claimed by at least one story, and no two stories
own the same responsibility. Where an AC appears twice the responsibilities are disjoint
and named below.

| AC | Story or stories | Ownership seam where an AC is claimed twice |
| --- | --- | --- |
| AC-001 | `reconciliation-ledger` | — |
| AC-002 | `reconciliation-ledger` | — |
| AC-003 | `charter-open-question-disposition`, `staged-audience-payload` | The first *decides* the evaluator question and marks the charter's line answered; the second *transcribes* that decision and its reasoning into the atom text AC-003 requires it to appear in |
| AC-004 | `audience-ingest-wave` | — |
| AC-005 | `audience-ingest-wave` | — |
| AC-006 | `staged-audience-payload` | — |
| AC-007 | `staged-audience-payload` | — |
| AC-008 | `audience-ingest-wave` | — |
| AC-009 | `audience-ingest-wave` | — |
| AC-010 | `product-layer-mounting` | — |
| AC-011 | `audience-ingest-wave`, `product-layer-mounting` | The first proves `validate --kb` green on the atoms as landed; the second re-proves it after the reciprocal `related` edges and the map section are added, because those change the frontmatter the validator reads |
| AC-012 | `merge-forward-baseline`, `post-merge-clause-completeness`, `reconciliation-ledger` | The first *produces* the merged sha; the other two are the two records AC-012 requires to *name* it |
| AC-013 | `post-merge-clause-completeness` | — |
| AC-014 | `dod-scenario-ledger`, `scenario-two-fault-injection` | The first owns fourteen scenarios and the ledger's shape; the second owns scenario 2's two rows, which AC-015 makes a separate obligation |
| AC-015 | `scenario-two-fault-injection` | — |
| AC-016 | `terminal-gate-run` | — |
| AC-017 | `charter-open-question-disposition` | — |
| AC-018 | `design-tension-audit` | — |

Eighteen of eighteen covered; no AC is orphaned, and no story exists that traces to none.
The brief-grain criteria (`AC-UX-01 … AC-UX-12`, `AC-A01 … AC-A10`, `AC-TB-01 … AC-TB-08`)
are constraints on *how* these eleven stories are built, not additional stories: each is
discharged inside the story carrying the project AC it serves, per the tier mapping in
[`_decomposition.md`](_decomposition.md), "## Testing brief", "AC-### → tier mapping".

## Merge order

Milestone by milestone, foundation first, with the load-bearing ordering each boundary
enforces.

1. **`merged-tree-baseline`** — `merge-forward-baseline` (foundation), then
   `post-merge-clause-completeness`.
   *Enforces ordering 1: merge before everything. Nothing after this point may be observed
   against this worktree's pre-merge copy.*
2. **`closeout-adjudication`** — `reconciliation-ledger`, then
   `charter-open-question-disposition`; `design-tension-audit` in parallel with either.
   *All three name the M1 sha. The evaluator decision must exist before any atom text
   claims it.*
3. **`product-layer-promotion`** — `staged-audience-payload`, then `audience-ingest-wave`,
   then `product-layer-mounting`, committed as one wave.
   *Enforces orderings 2 and 3: ingest before validation, validation before
   `record-links --atom`.*
4. **`dod-reobservation`** — `dod-scenario-ledger`, then `scenario-two-fault-injection`,
   then `terminal-gate-run`.
   *Enforces ordering 4: everything committed before the final gate. `terminal-gate-run`
   is last in the project by construction — a gate run taken before the records are in the
   tree proves the gate, not the deliverable.*

The one foundation story, `merge-forward-baseline`, is consumed and demonstrated inside
this initiative by `post-merge-clause-completeness` in its own milestone and by every
record downstream that names its sha. It lands a real tree state, not a placeholder.
