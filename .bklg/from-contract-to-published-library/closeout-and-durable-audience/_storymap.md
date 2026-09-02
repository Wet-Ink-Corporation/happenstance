---
item: HS-P0019
stage: storymap
created: 2026-08-12T03:30:23.655Z
updated: 2026-08-12T03:30:23.655Z
template_sig: 1c63534a
rendered_sig: bd428068
---

# Story Map — The whole gate on the assembled library, and a durable audience

Eleven stories over five slices, covering [`project.md`](project.md) **AC-001…AC-014**.

This project writes no production code ([`project.md`](project.md) *Out of scope*;
[`_decomposition.md`](_decomposition.md) *Intent*: "it verifies and records; it designs
nothing"). Every story's deliverable is therefore a **run plus a cited record**, and the
"user" of each slice is the reader who must be able to believe the initiative closed
honestly without re-deriving the evidence. The vertical-slice test still applies and is
what keeps this from degenerating into a checklist: a story is done when a command was
run on *this* tree, its output was observed, and a reader can follow the citation back
to the artefact — never when a box was ticked from the memory of the project that
produced it (DR-3).

**Where the evidence lands.** Each story carries its own `_ledger.md`, which
`.redkiln/config.yaml:67` (`require_ledger: true`) makes mandatory and cited, and
`:73` (`require_commit_provenance: true`) ties to commits. The stories that must be
read *together* — the DoD set, the delta, the audit tables, the findings — converge on
one companion artefact, `_closeout-record.md`, in this project's directory. That
convergence is the point of AC-003's word "set": fourteen ledger entries in fourteen
places is the failure mode the charter's DoD preamble is written against.

## Backbone

The user activities, left to right, in the order a reader of the closed initiative
encounters them.

| # | Activity | The outcome a reader gets | Slice that delivers it |
| - | -------- | ------------------------- | ---------------------- |
| A | **Assemble the library and run the whole gate on it** | One green `cargo xtask ci` on a checkout with no residue, with its SHA and every `skipped` step accounted for — including the four `wasm32` steps that stand guard on ADR-0001 | `assembled-tree-gate` |
| B | **Re-observe the promised scenarios as a set** | DoD 1–12 and 14–15 in one record, each with the command run here and the artefact it produced; and the honest statement of how the closeout tree differs from the published one | `dod-re-observation` |
| C | **Check that every answer is on disk, and nothing was deleted** | An atom per settled question naming what lost, and an open-questions directory that only grew | `answers-audit` |
| D | **Inherit the audience instead of re-deriving it** | Four persona atoms and their journey atoms under `.kb/product/`, arrived through the ingest path, each saying in its own `summary` that its evidence is secondary | `durable-audience` |
| E | **Close the initiative without absorbing what it found** | Clean `validate` / `doctor` with exactly six drift advisories, every finding routed to an owner, and no open child of HS-I0006 | `closeout-health-and-disposition` |

## Slices

Stories sharing a Milestone are implemented in one context and land as one integrated
surface. Cross-milestone `depends_on` edges are acyclic; the two `foundation` stories
sit ahead of the capability story in their own slice that consumes them.

| Milestone | Story | Archetype | One-line slice | depends_on | traces_to |
| --------- | ----- | --------- | -------------- | ---------- | --------- |
| `assembled-tree-gate` | `clean-checkout-harness` | foundation | Construct the clean-checkout seam the gate must run in — a fresh checkout with no untracked or ignored residue and no path dependency standing in for a published version — and stand up `_closeout-record.md` as the artefact every later story appends cited evidence to. | — | AC-001 |
| `assembled-tree-gate` | `whole-gate-green-on-the-assembled-tree` | capability | Run `cargo xtask ci` — the terminal grain at `.redkiln/config.yaml:60`, never `--fast` — to completion in that checkout, and record the exit code, the commit SHA, the four `wasm32` step outcomes, and the absent tool behind every step that printed `skipped`. | `clean-checkout-harness` | AC-001, AC-002 |
| `dod-re-observation` | `dod-set-re-observation-record` | capability | Re-run or re-inspect DoD 1–12 and 14–15 on this tree and record all fourteen in one table, each with the command and the artefact path — a sibling's `_ledger.md` may point at what to re-run, never stand in as the evidence. | `whole-gate-green-on-the-assembled-tree` | AC-003 |
| `dod-re-observation` | `published-tree-delta-statement` | capability | State the DoD 13 delta rather than glossing it: the published `0.2.0` tag, the commits between it and the closeout tree, the two projects that own them, and the cited gate decision that keeps the published surface unchanged across the delta. | `dod-set-re-observation-record` | AC-004 |
| `answers-audit` | `decision-atom-audit-table` | capability | Produce the audit table mapping every question this initiative settled to an accepted atom under `.kb/decisions/` and the alternatives that atom records as rejected, with a stated reason for each reserved ADR number nobody needed to write. | — | AC-005 |
| `answers-audit` | `open-question-preservation-audit` | capability | Prove no open-question atom was deleted — a zero-deletion diff of `.kb/open-questions/` across the initiative — and that each consumed atom carries a resolution-bearing status and an annotated bullet in the index. | `decision-atom-audit-table` | AC-006 |
| `durable-audience` | `persona-and-journey-intake-staging` | foundation | Stage three personas and four journeys in `.kb/_intake/` under a distinct wave id, each draft carrying the secondary-evidence sentence in `summary` and its discovery artefacts in `source_paths`. *(Corrected 2026-08-12: this row previously said "the four personas… matching the four-persona set the testing brief already decided", which was downstream of DR-10's superseded text. The evaluator is a journey of its own linked to the Application-author persona, not a fourth persona — see DR-10's amendment in `_decomposition.md`.)* | — | AC-010 |
| `durable-audience` | `product-atom-promotion-via-kb-ingest` | capability | Promote that staging into `.kb/product/` **through `/redkiln:kb-ingest`**, leaving the intake consumed and cleared, a `concept`/`playbook` atom per persona/journey at `authority_tier: product`, and `redkiln validate --kb` green on them. | `persona-and-journey-intake-staging` | AC-007, AC-008, AC-009 |
| `closeout-health-and-disposition` | `backlog-and-kb-health-at-closeout` | capability | Run `redkiln validate`, `redkiln validate --kb` and `doctor --json` on the closeout tree, assert zero problems, zero `process-drift` and a `template-drift` set equal to the six CI names, and disposition any seventh or missing sixth with a decision — without ever running `adopt --templates`. | `product-atom-promotion-via-kb-ingest` | AC-011, AC-012 |
| `closeout-health-and-disposition` | `findings-disposition-register` | capability | Route every finding the re-observation surfaced — `support`, a new item against the owning sibling, or a decision atom plus a re-plan — and show zero were fixed inside this project. | `whole-gate-green-on-the-assembled-tree`, `dod-set-re-observation-record`, `published-tree-delta-statement`, `decision-atom-audit-table`, `open-question-preservation-audit`, `backlog-and-kb-health-at-closeout` | AC-013 |
| `closeout-health-and-disposition` | `initiative-closeout-readiness` | capability | Show the initiative can close: every sibling HS-P0010…HS-P0018 closed out with no open child of HS-I0006 under `redkiln status`, and the initiative's own closeout linking the promoted product atoms. | `product-atom-promotion-via-kb-ingest`, `findings-disposition-register` | AC-014 |

### Why the slices are cut here

- **`assembled-tree-gate`** keeps the seam and the run together. Splitting "make a clean
  checkout" from "run the gate in it" would be exactly the build-it/wire-it-in split the
  slicing rule forbids: the checkout is not observable until something runs in it, and the
  gate result is not trustworthy until it ran there. `_decomposition.md` *Fixtures /
  seams* calls the clean checkout "the one seam this project must construct
  deliberately... and it is the thing AC-001 is actually checking for" — which is what
  makes it a `foundation` story rather than a setup step, and it is consumed and
  demonstrated by its own slice-mate.
- **`dod-re-observation`** is one artefact with two halves. The fourteen-row table and the
  DoD 13 delta statement are read in the same sitting by the same reader; the delta exists
  precisely to qualify a phrase in the DoD preamble the table is answering.
- **`answers-audit`** is the BR-15 audit half, whole. Both stories are static/process
  checks over `.kb/`, they share the same "what did this initiative settle" enumeration,
  and the second is meaningless without the first's list.
- **`durable-audience`** is the promotion, whole. The staging is `foundation` — real
  in-tree material under `.kb/_intake/`, never a hand-written atom — and it is consumed
  in the same slice by the ingest run. Holding the two apart is how `0269720` happened:
  "the directory layout of the process without the process."
- **`closeout-health-and-disposition`** is the terminal sequence. Health must be measured
  *after* the new product atoms exist (they are inputs to `validate --kb`), findings can
  only be routed once every other slice has produced its findings, and the initiative can
  only be declared closable once both have landed.

### Grain notes carried into the stories

- **No story fixes anything.** AC-013 makes routing the deliverable
  ([`project.md`](project.md) risk table, "Pressure to fix what re-observation finds").
  A story that repairs a defect it found has failed its own acceptance criterion.
- **No story hand-authors a `.kb/` atom.** DR-8 and AC-008 make the *provenance*
  observable, not just the result.
- **No story substitutes `--fast`.** DR-2; `.redkiln/config.yaml:55` is the bar every
  sibling already met, and re-running it here would prove nothing this project exists
  to prove.
- **AC-010 is already discharged in the brief** ([`_decomposition.md`](_decomposition.md)
  *The evaluator-persona decision*): the evaluator is its own persona, so the promoted set
  is **four** personas plus their journeys. `persona-and-journey-intake-staging` carries
  AC-010 because it is where that decision becomes observable in the staged set — a
  three-persona staging is wrong against the brief. Whether HS-P0016's recorded DT-1
  resolution is consistent with the decision is a **finding to route** in
  `findings-disposition-register`, not a re-litigation.

## Coverage

Every project AC-### is owned by exactly one story, except AC-001, which the foundation
story and its consumer split along a real seam (the residue-free checkout vs. the green
run inside it). No AC is orphaned and no two stories own the same responsibility.

| AC | Story | Slice |
| -- | ----- | ----- |
| AC-001 whole gate green from a clean checkout | `clean-checkout-harness` (the checkout), `whole-gate-green-on-the-assembled-tree` (the run) | `assembled-tree-gate` |
| AC-002 four `wasm32` steps green in that run | `whole-gate-green-on-the-assembled-tree` | `assembled-tree-gate` |
| AC-003 DoD 1–12, 14–15 re-observed as a set | `dod-set-re-observation-record` | `dod-re-observation` |
| AC-004 DoD 13 delta stated, not glossed | `published-tree-delta-statement` | `dod-re-observation` |
| AC-005 every answer has an atom naming what lost | `decision-atom-audit-table` | `answers-audit` |
| AC-006 no open-question atom deleted | `open-question-preservation-audit` | `answers-audit` |
| AC-007 audience exists as durable atoms | `product-atom-promotion-via-kb-ingest` | `durable-audience` |
| AC-008 atoms produced by the ingest path | `product-atom-promotion-via-kb-ingest` | `durable-audience` |
| AC-009 secondary-evidence qualification unmissable | `product-atom-promotion-via-kb-ingest` | `durable-audience` |
| AC-010 evaluator question decided before authoring | `persona-and-journey-intake-staging` | `durable-audience` |
| AC-011 backlog/KB clean, exactly six advisories | `backlog-and-kb-health-at-closeout` | `closeout-health-and-disposition` |
| AC-012 a seventh/missing sixth is dispositioned | `backlog-and-kb-health-at-closeout` | `closeout-health-and-disposition` |
| AC-013 findings routed, not absorbed | `findings-disposition-register` | `closeout-health-and-disposition` |
| AC-014 the initiative can be closed | `initiative-closeout-readiness` | `closeout-health-and-disposition` |

Fourteen ACs, fourteen rows, eleven stories. The DoD scenarios at
[`project.md`](project.md) *Definition of done* map through: DoD 1 → AC-001/AC-002,
DoD 2 → AC-003/AC-004, DoD 3 → AC-005/AC-006, DoD 4 → AC-007…AC-009, DoD 5 →
AC-011/AC-012, DoD 6 → AC-013, DoD 7 → AC-014.

## Merge order

Slice by slice, foundation stories before the capability that consumes them.

1. **`assembled-tree-gate`** — `clean-checkout-harness` (foundation) →
   `whole-gate-green-on-the-assembled-tree`. First because everything downstream cites the
   run it produces, and because a gate failure here is the finding the whole project exists
   to surface.
2. **`dod-re-observation`** — `dod-set-re-observation-record` →
   `published-tree-delta-statement`. Consumes slice 1's run for the scenarios the gate
   itself re-proves.
3. **`answers-audit`** — `decision-atom-audit-table` → `open-question-preservation-audit`.
   Independent of slices 1–2 in principle; sequenced here so its findings reach slice 5.
4. **`durable-audience`** — `persona-and-journey-intake-staging` (foundation) →
   `product-atom-promotion-via-kb-ingest`. Independent of slices 1–3; must precede slice 5
   because the new atoms are inputs to `redkiln validate --kb`.
5. **`closeout-health-and-disposition`** — `backlog-and-kb-health-at-closeout` →
   `findings-disposition-register` → `initiative-closeout-readiness`. Last by
   construction: it measures the tree every prior slice contributed to and routes what
   they found.

Slices 3 and 4 have no edge between them and could be swapped or run in either order; the
sequence above is the one that keeps the two `.kb/`-writing slices adjacent, so a single
`validate --kb` at the head of slice 5 covers both.

## Anchors

- [`project.md`](project.md) — AC-001…AC-014, DR-1…DR-13, the risk table
- [`_intake-brief.md`](_intake-brief.md) — the approved problem, constraints and proof artefact
- [`_decomposition.md`](_decomposition.md) — the testing brief: tier per AC, merge-gate commands, fixtures/seams, and the evaluator-persona decision
- [`../initiative.md`](../initiative.md) — BR-15, BR-16, AC-15, DoD 13/16, exit criteria 7–8
- [`../_decomposition.md`](../_decomposition.md) — the traceability matrix, the DAG, *Decisions taken at the gate* item 4
- [`../_discovery/distillation/personas-and-journeys.md`](../_discovery/distillation/personas-and-journeys.md) — discovery's four personas, of which three are promoted as persona atoms in slice 4, the fourth (the evaluator) becoming a journey atom instead
- `.kb/product/README.md:15-24` — persona = `concept`, journey = `playbook`, both `authority_tier: product`; closeout performs the promotion
- `.kb/maps/open-questions-index.md:11-13` — a resolved question stays listed and annotated, never removed
- `.kb/decisions/README.md` — accepted atoms are immutable; supersede, never edit
- `.redkiln/config.yaml:56-60` — `e2e: cargo xtask ci`, the terminal grain this project alone carries; `:55` is the `--fast` bar it must not substitute
- `.redkiln/config.yaml:62-73` — `require_ledger`, `require_commit_provenance`; `:5` — `support_initiative`, where findings route
- `.github/workflows/ci.yml:142-187` — the `backlog` job and the exact six-file `template-drift` assertion
- `xtask/src/main.rs` — the gate, defined once
- `RUNBOOK.md:262-284` — the ADR queue slice 3 audits
- `CLAUDE.md` — binding constraints, the six deliberate customisations, and the `adopt --templates` prohibition
