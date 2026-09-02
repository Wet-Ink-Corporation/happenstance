---
item: HS-S0174
stage: spec
created: 2026-08-17T13:16:26.176Z
updated: 2026-08-17T13:16:26.176Z
template_sig: 87bbf1d0
rendered_sig: 48a7563b
---

# Spec — Author the pair-by-pair audience reconciliation record

## Scope lock

| | Path |
| --- | --- |
| Initiative (gold source) | [`.bklg/docs-that-teach/initiative.md`](../../initiative.md) — BR-13, DoD-15, `## Referenced personas & journeys` (`:275-311`), `## Open questions for the planning team` (`:513-556`) |
| Initiative decomposition | [`.bklg/docs-that-teach/_decomposition.md`](../../_decomposition.md) — the merge-forward rule (`:89-103`), persona reconciliation run-parallel-and-reconcile (`:116-123`), the evaluator deferral (`:134-139`) |
| Project | [`.bklg/docs-that-teach/durable-audience-closeout/project.md`](../project.md) — DR-1, DR-2, AC-001, AC-002, AC-012 |
| This spec | `.bklg/docs-that-teach/durable-audience-closeout/reconciliation-ledger/spec.md` |
| Key briefs | [`../_decomposition.md`](../_decomposition.md) — `## UX brief` (readers U1/U2/U3, the state vocabulary, IQ-1…IQ-8, AC-UX-03/09/10/12), `## Architecture brief` (§1 diff surface, §2 mount points, §5 ordering 1, §6 "where the reconciliation record lives"), `## Testing brief` (Tier 2 is primary for AC-001/AC-002; the unrun-counterpart parametric read) |
| Signed-off design | [`../_design.md`](../_design.md) — `hasSurface: false`, approved 2026-08-17 with no conditions. No API surface, no rendered surface, no design tension owned |
| Grounding | [`../_grounding.md`](../_grounding.md) — "Reconciliation counterpart does not exist in this tree"; no Accepted ADR reaches this surface |
| Story map / roadmap | [`../_storymap.md`](../_storymap.md) — milestone `closeout-adjudication`, merge order 2, "The counterpart does not exist in this tree" |
| Source of truth for the pairs | [`../../_discovery/distillation/personas-and-journeys.md`](../../_discovery/distillation/personas-and-journeys.md) — three personas (`:56`, `:148`, `:225`), the risks (`:317-359`), the open questions (`:361-397`) |

## One-line PR slice

Author the complete pair-by-pair audience reconciliation record — one row per persona and per journey on both sides, each `merged` / `superseded` / `kept distinct` with a named reason — stating in prose above the ledger which merge-order case actually held, and naming the merged sha.

## Executive summary

This PR lands one new file — the reconciliation record — plus the one line in
[`../project.md`](../project.md)'s `## Companions` list that makes it reachable, and nothing else.

The delta against what already exists: today the audience lives in exactly one place, this
initiative's own discovery distillation, and the charter says so in as many words — the product
layer is *"structurally present and functionally empty"* and the personas are *"flagged for
promotion at closeout"* ([`../../initiative.md`](../../initiative.md), `:275-311`). The sibling
initiative authored its own overlapping set independently
([`../../_discovery/distillation/personas-and-journeys.md`](../../_discovery/distillation/personas-and-journeys.md), `:343-359`).
Nobody has ever written down which of those two authorings survives, pair by pair.

This story writes that down, once, against the merged tree that
`merge-forward-baseline` (HS-S0172) produced. It is the *adjudication*, not the promotion: no
`.kb/` file is touched here. Its output is consumed twice inside its own milestone —
`charter-open-question-disposition` marks the charter's evaluator question answered against the
reasoning recorded here, and, one milestone later, `staged-audience-payload` transcribes the
surviving pairs and their qualifications into the staged atom text. Getting it wrong is not
recoverable cheaply: an unstated single-sided authoring is *"the charter's named failure mode
wearing a reconciliation's clothes"* ([`../project.md`](../project.md), risk table).

## Context pack

The load-bearing decisions this story must honor. Everything deeper is a signposted anchor below.

**1. Reconciliation is an adjudication with a row per pair, not an assertion.** DR-1 fixes the
unit as the *pair* and the verdict vocabulary as exactly three words — `merged` (naming the
surviving item), `superseded` (naming the superseded source), `kept distinct` (naming the
distinguishing goal or fear). Silence on a pair is a defect, not a tacit pass. The reason the
unit is the pair rather than the set is U2, the closeout reviewer, whose stated need is *"show me
every adjudication you made, including the ones that changed nothing, so I can disagree with
exactly one of them without reading the discovery corpus"* ([`../_decomposition.md`](../_decomposition.md),
`## UX brief`, reader table). A summary sentence may precede the ledger; it may never replace a
row (IQ-2, *"the ledger, not the summary"*).

**2. The merge-order case is stated in prose above the ledger, and the record reads coherently
under whichever case actually held.** This is AC-002, and it is not a footnote to be bolted on if
the merge turns out empty — the architecture brief's T3 requires the unrun case to be the
record's **default shape**, with the adjudication table degrading to "no counterpart staged" rows.
Do not blur two different facts: whether HS-S0131 *ran and staged output*, and whether the
sibling's own *distillation document* became readable when the branch merged forward. The second
can be true while the first is false, and the record must say which it found by naming what it
looked at (see `## Behavior and interfaces`, B-2).

**3. Every observation names the merged tree by sha.** Ordering 1 of the architecture brief's data
flow is *merge before everything*: reconciliation performed against this worktree's pre-merge copy
measures a stale tree ([`../../_decomposition.md`](../../_decomposition.md), `:89-103`). AC-012 requires
this record to be one of the two artefacts that *name* the merged tree; AC-UX-12 requires it in a
form a later reader can re-run from. The sha is produced upstream by `merge-forward-baseline`
(HS-S0172) — this story does not perform the merge and must not run one.

**4. This story authors no `.kb/` file and no decision.** `.kb/` atoms reach the corpus only
through `/redkiln:kb-ingest` from `.kb/_intake/`, which is a *later* milestone's work
(`staged-audience-payload` → `audience-ingest-wave`); hand-authoring the directory layout of the
process without the process was reverted once already at `0269720`. And AC-A09 forbids adding to
or modifying `.kb/decisions/` at all: a reconciliation finding that wants a decision is written
down here as a **routed gap in prose**. The governing rule is
[`.kb/governance/rewrite-the-referent-never-the-reasoning.md`](../../../../.kb/governance/rewrite-the-referent-never-the-reasoning.md)
— a correction to a standing decision is a new atom that supersedes, never an edit — and it cuts
both ways: it also forbids inventing a decision as closeout exhaust.

**5. Do not go hunting for an ADR to cite.** None of the sixteen accepted decision atoms reaches
this surface; the charter states it outright, *"none of the seventeen decisions concerns
documentation"* ([`../../initiative.md`](../../initiative.md), `:524-525`). Citing one here would
be manufacturing authority, which is the same defect as manufacturing a persona. What binds is the
governance atom above, the single-write-path rule, and the project's own DR-1/DR-2.

**6. The record is a project artefact and never becomes an atom.** The architecture brief is
explicit that the temptation is to promote it wholesale: *"It is not durable knowledge about an
audience; it is a project artefact. Its **outcome** travels into the atoms' bodies and
`source_paths`; the record itself stays under
`.bklg/docs-that-teach/durable-audience-closeout/`"* ([`../_decomposition.md`](../_decomposition.md),
`## Architecture brief`, §4). `.kb/product/README.md`'s "what does not belong here" says the same
from the other side: requirements, scope and acceptance criteria stay in `.bklg/`.

**7. Every state is a word, and load-bearing verdicts also exist as sentences.** The accessibility
floor binds *today*, in markdown: no tick, no emoji, no colour word, no strikethrough-as-status,
no "a blank cell means fine", no state inferable only from a row's absence — the corpus precedent
is `.kb/maps/open-questions-index.md`, which spells `Open` / `Withdrawn` / `Superseded` as literal
words, first. And *"a verdict that exists only as a table cell is a verdict that will be lost in a
quote or a diff"*: where a reconciliation outcome or a routed gap is load-bearing, state it in a
sentence as well ([`../_decomposition.md`](../_decomposition.md), `## UX brief`, "Accessibility floor").

**8. Supersession annotates, it never deletes.** A superseded persona is *named in the record*,
not erased from it, and the discovery distillation this record adjudicates is cited, never
deleted or edited — so that a charter which already cited the distillation and a charter that
later cites the promoted atom are pointed at the same audience rather than two (IQ-6, "preserved
selection"). Nothing this story lands may move an anchor someone else already cited (IQ-3).

**9. The persona-journey slice this realizes.** The reader here is not any of the three documented
personas — they are the *subject* of this record, never its audience. U2 (the closeout reviewer)
reads it to find the one adjudication they disagree with; U3 (a future maintainer reconciling a
third persona set) reads it to see *what was decided, against which tree, on what evidence* and to
correct it without editing it; U1 (the next initiative's charter author) never opens this file at
all — they meet its outcome later, in an atom.

**10. What the two sides actually contain.** This side: three personas — the application author
(`:56`), the adapter author (`:148`), the evaluator (`:225`) — and the four journeys the charter
names at [`../../initiative.md`](../../initiative.md) `:289-301` (*the first fifteen minutes*,
*model my invariant in your words*, *walk the adapter path, not just the recipe*, *survive the
second question*). The other side: HS-S0131 is specced to stage **four** personas, of which only
two are named from this tree — an "application author" and an "adapter author" that overlap
Personas 1 and 2 *"almost exactly"* (`:343-359`). Its remaining two and its journeys are not
readable here. An unreadable counterpart entry is recorded as unreadable; it is never guessed at,
because inventing personas is the single failure mode this whole project exists to prevent
([`../project.md`](../project.md), "Out of scope").

## Integration contract

- **Archetype**: `capability`.
- **Slice / milestone**: `closeout-adjudication`. Slice-mates implemented in the same context:
  `charter-open-question-disposition`, `design-tension-audit`.
- **Mount point**: `.bklg/docs-that-teach/durable-audience-closeout/project.md` — the
  `## Companions` list in its body (currently `:291-301`). That list is the render path a closeout
  reviewer reaches this project's artefacts through; a record not on it is reachable only by
  knowing this story's slug, which is IQ-5's named falsifier. The project item's YAML frontmatter
  is CLI-owned and is not touched — only the prose body beneath the closing `---`.
- **Record path**: `.bklg/docs-that-teach/durable-audience-closeout/reconciliation-ledger/_reconciliation.md`.
  Fixed here because [`../_decomposition.md`](../_decomposition.md) `## Architecture brief` §6 left it
  deliberately unprescribed ("a companion under this project's folder, or the body of `closure.md`")
  and `closure.md` does not exist until the project's closeout stage renders it. A companion in this
  story's own folder keeps the record inside the story's PR boundary and inside the project folder,
  which satisfies both halves of that sentence.
- **Wires into**:
  - upstream — `merge-forward-baseline` (HS-S0172), which supplies the merged commit sha and the
    merged tree everything below is read on;
  - source material — [`../../_discovery/distillation/personas-and-journeys.md`](../../_discovery/distillation/personas-and-journeys.md)
    (this side's three personas and their risks/overlap notes) and
    [`../../initiative.md`](../../initiative.md) `:289-301` (this side's four journeys);
  - counterpart material — whatever the merged tree actually exposes under `.bklg/` and
    `.kb/_intake/`, enumerated rather than assumed (B-2 below);
  - downstream — `charter-open-question-disposition` (HS-S0175) consumes the evaluator adjudication
    recorded here; `staged-audience-payload` (HS-S0177) transcribes the surviving pairs, the
    supersession statements and the HS-S0131 overlap qualification into staged atom text;
  - "design-system primitives" in this repository's text-corpus sense: the state-word vocabulary and
    the append-only map/index formats catalogued at [`../_decomposition.md`](../_decomposition.md),
    `## UX brief`, "Design-system primitives". No new format is invented here.
- **Renders surfaces**: **none**. [`../_design.md`](../_design.md) records `hasSurface: false` with an
  empty `## Items` block, signed off 2026-08-17 with no conditions, so there is no surface id to
  claim. The binding written description of what this artefact must be is instead the UX brief's
  artefact **3 — the reconciliation record** ([`../_decomposition.md`](../_decomposition.md),
  `## UX brief`, Intent) together with AC-UX-03, AC-UX-09, AC-UX-10 and AC-UX-12. This story
  implements that description; it does not re-decide it.
- **Conformance rule(s) / clause(s)**: none, and not adapter-observable. This story ships no Rust,
  invokes no port, and its diff never reaches `crates/`, `spec/`, `xtask/` or `standards/`
  ([`../_decomposition.md`](../_decomposition.md), `## Architecture brief`, §1). No
  `SPECIFICATION.md` clause is discharged or amended; the clause-completeness work is
  `post-merge-clause-completeness`'s, in a different milestone.
- **Advances DoD scenario**: **DoD-15** — *"The audience is durable and reconciled"*
  ([`../../initiative.md`](../../initiative.md), `:465-468`). This story lands the *reconciled*
  half's evidence: the written adjudication that the promoted set is reconciled with the separately
  staged one rather than duplicating it. The *durable* half (atoms under `.kb/product/` passing
  `redkiln validate --kb`, linked from the closeout) is the `product-layer-promotion` milestone's.

## PR boundary

**In this PR**

- `_reconciliation.md` in this story's folder: the prose merge-order statement, the pair ledger,
  the routed-gap section, and the sha attestation.
- `_ledger.md` in this story's folder (`require_ledger: true`, `.redkiln/config.yaml:62-67`), one
  row per AC-### below with a `file:line` citation.
- One appended bullet in [`../project.md`](../project.md)'s `## Companions` list naming the record —
  the mount, and explicitly not scope drift.

**Explicitly not in this PR**

- The merge itself. `merge-forward-baseline` (HS-S0172) performs it; this story reads its result.
- Any file under `.kb/` — no atom, no map edit, no `.kb/_intake/` staging, and above all no
  addition to or modification of `.kb/decisions/` (AC-A09).
- Any file under `crates/`, `spec/`, `xtask/`, `standards/`, `docs/`, `examples/` or `references/`.
- Marking the charter's evaluator open question answered, and editing
  [`../../initiative.md`](../../initiative.md) at all — that is `charter-open-question-disposition`'s
  (AC-017), consuming the reasoning recorded here.
- Transcribing anything into atom text, and authoring `source_paths` — `staged-audience-payload`'s.
- The DT-1 … DT-10 audit table — `design-tension-audit`'s, same milestone, different artefact.
- Editing HS-P0021's page-need discipline text, which rides a later wave as payload.

**Merge DoD**: the record exists at the path above, names the merged sha, states the merge-order
case it found in prose above a ledger with no unaddressed pair, is reachable from
[`../project.md`](../project.md)'s `## Companions`, and `cargo xtask affected --base main` is green
(a no-op affected set is the expected and correct result for a markdown-only diff).

```
.bklg/docs-that-teach/durable-audience-closeout/reconciliation-ledger/**
.bklg/docs-that-teach/durable-audience-closeout/project.md
```

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **B-1 — The record is a single file at a fixed path** | `_reconciliation.md` in this story's folder, with one `#` title, `##` sections, no skipped heading level. Sections in this order: the merge-order statement (prose), the pair ledger (table), the routed gaps, the sha attestation. Prose first is not cosmetic — AC-UX-03 requires the case to be readable *before* the table it governs. | [`../_decomposition.md`](../_decomposition.md) `## Architecture brief` §6; `## UX brief` "Accessibility floor" (heading structure) |
| **B-2 — The merge-order case is *determined*, then stated** | Determine by looking, on the merged tree, and record what was looked at: (a) `ls .bklg` — which initiatives are present; (b) whether `.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md` resolves; (c) `ls .kb/_intake/` and `ls .kb/product/` — whether HS-S0131 actually staged or landed anything. Three admissible cases: **counterpart ran** (adjudicate against its staged/landed output); **counterpart unrun but its distillation is readable post-merge** (adjudicate against four *unstaged sketches*, saying so — they are not atoms); **neither readable** (this initiative's set stands as the authored one, stated plainly). The default drafting case is the last one. | [`../_grounding.md`](../_grounding.md) "Reconciliation counterpart does not exist in this tree"; [`../_decomposition.md`](../_decomposition.md) T3; [`../../_discovery/distillation/personas-and-journeys.md`](../../_discovery/distillation/personas-and-journeys.md) `:353-356` (the sibling distillation's own path) |
| **B-3 — The ledger's unit is the pair, and both sides are enumerated** | This side: three personas (`:56`, `:148`, `:225`) and the four charter journeys ([`../../initiative.md`](../../initiative.md) `:289-301`) — seven rows minimum. Other side: HS-S0131's four staged personas, of which only two are named from this tree. Every entry on both sides gets a row; a counterpart entry that cannot be read is recorded as **not readable from this tree**, never invented and never silently omitted. | [`../project.md`](../project.md) DR-1, AC-001; [`../_decomposition.md`](../_decomposition.md) AC-UX-03; personas distillation `:343-359` |
| **B-4 — Verdict vocabulary is closed and each verdict names its object** | Exactly `merged` (names the surviving item), `superseded` (names the superseded source), `kept distinct` (names the distinguishing goal or fear), plus `no counterpart staged` and `not readable from this tree` for the degraded cases. No fourth improvised verdict, no empty cell, no glyph. | [`../project.md`](../project.md) DR-1; [`../_decomposition.md`](../_decomposition.md) `## UX brief`, "The states this surface has" |
| **B-5 — Rows are self-contained and linearly readable** | Each row names its side, its item, its counterpart, its verdict word, its reason and the path the item was read from. No row depends on the row above it to be understood, and no verdict is expressed by ordering, grouping or omission. | [`../_decomposition.md`](../_decomposition.md) AC-UX-10, AC-UX-09, IQ-8 |
| **B-6 — Load-bearing verdicts are restated as sentences** | Every `superseded`, every `kept distinct`, and every routed gap also appears as a sentence outside the table, so it survives a quote or a diff hunk. `merged` and no-change rows may live in the table alone. | [`../_decomposition.md`](../_decomposition.md) `## UX brief`, "Plain-text equivalence for load-bearing claims" |
| **B-7 — The evaluator pair carries a reasoned adjudication, marked as the handoff** | Persona 3's row is the one the decomposition deferred to promotion *"jointly with HS-S0131's set rather than twice"*. Its reason states which way it resolves — a persona in its own right, or an earlier stage of the application author's journey — and *why*, in prose, naming both the differing verbs and the shared fear the distillation records. The record marks it explicitly as the input `charter-open-question-disposition` marks answered; it does not itself edit the charter. | [`../../_decomposition.md`](../../_decomposition.md) `:134-139`; [`../../_discovery/distillation/personas-and-journeys.md`](../../_discovery/distillation/personas-and-journeys.md) `:370-376`; [`../_storymap.md`](../_storymap.md) "Why these four and not eleven" |
| **B-8 — Findings that want a decision become routed gaps, in prose** | A dedicated `## Routed gaps` section. Each gap states what was found, why it is not settled here, and where it goes (a later initiative, the `support` initiative per `.redkiln/config.yaml:5`, or a charter open question). The diff contains no addition to and no modification of `.kb/decisions/`. If there are no gaps, the section says so in a sentence rather than being omitted. | [`../_decomposition.md`](../_decomposition.md) AC-A09; [`.kb/governance/rewrite-the-referent-never-the-reasoning.md`](../../../../.kb/governance/rewrite-the-referent-never-the-reasoning.md) |
| **B-9 — The sha attestation is re-runnable** | The record names the merged commit as a full 40-character sha plus the branch it was read on, and states the command that produced it. Prose form: what was read, on which tree, when. A ref name alone is not sufficient — refs move, and U3's stated need is to re-run the observation. | [`../project.md`](../project.md) AC-012; [`../_decomposition.md`](../_decomposition.md) AC-A06, AC-UX-12 |
| **B-10 — The three carried qualifications are named as handoff obligations, not discharged here** | The record names all three — per-persona observation status, the adapter author's "no third-party adapter exists to read" resting on internal audit alone (`:336-342`), and the HS-S0131 overlap (`:343-359`) — and states that they travel into atom bodies. The overlap qualification is this record's own output and must be stated in its own terms. Discharging them inside atoms is `staged-audience-payload`'s (project AC-006/AC-007), not this story's. | [`../project.md`](../project.md) DR-4, and its `## Coverage` seam for AC-006/AC-007; [`../_decomposition.md`](../_decomposition.md) IQ-7 |
| **B-11 — Nothing already cited moves** | The discovery distillation is cited, never edited or deleted; no existing `##` section of [`../project.md`](../project.md) is reflowed; the only change to it is an appended bullet. `git diff` over `project.md` shows no deletion line. | [`../_decomposition.md`](../_decomposition.md) IQ-3, IQ-6, AC-A05 |
| **B-12 — Verification is a human read against an existing checklist** | Tier 2 (content review) is primary for AC-001 and AC-002; Tier 1 contributes only `ls .bklg` and an `rg` for the sha string. The checklist is the UX brief's IQ-1…IQ-8 and AC-UX-01…AC-UX-12 — no second, divergent checklist is authored here. | [`../_decomposition.md`](../_decomposition.md) `## Testing brief`, "AC-### → tier mapping" (AC-001, AC-002, AC-012 rows), AC-TB-01, AC-TB-06 |

## Data and migrations

**N/A.** This story adds no schema, no persisted state, no migration and no frontmatter.

The one shape worth stating so the second pass and the reviewer agree on it: `_reconciliation.md`
is plain markdown with **no YAML frontmatter** — it is a project artefact under `.bklg/`, not a
`.kb/` atom, so `KbFrontmatter` does not apply to it and inventing keys on it would be the
hand-rolling the UX brief forbids ([`../_decomposition.md`](../_decomposition.md), `## UX brief`,
"Hand-rolling, explicitly forbidden here"). `redkiln validate --kb` never reads this file; the only
machine that reads anything in this diff is `redkiln verify --grain story`, over `_ledger.md` and
the PR boundary block above.

The ledger table's columns are fixed by B-3 through B-5 — side, item, counterpart, verdict, reason,
source path — and are a presentation contract for U2 rather than data. No column may be replaced by
a glyph, and no column may be dropped for a row that has nothing interesting to say in it.

## Acceptance criteria

Framed from the intent of the three readers the UX brief names — U1 (the next initiative's charter
author), U2 (the closeout reviewer at the ingest gate and the pull request), U3 (a future maintainer
reconciling a third persona set) — because the three *documented* personas are this record's subject
and never its audience ([`../_decomposition.md`](../_decomposition.md), `## UX brief`, "Who this is
actually for"). Verification is by the tier the testing brief already assigned to the project AC each
row serves; this story authors no second, divergent checklist (AC-TB-06).

| ID | Criterion | Verification |
| --- | --- | --- |
| **AC-001** | **GIVEN** U2 at the pull request with [`../../_discovery/distillation/personas-and-journeys.md`](../../_discovery/distillation/personas-and-journeys.md) open beside the record, **WHEN** they walk both sides item by item — this side's three personas (`:56`, `:148`, `:225`) and four charter journeys ([`../../initiative.md`](../../initiative.md) `:289-301`), and HS-S0131's staged set — **THEN** each one has exactly one ledger row carrying one closed-vocabulary verdict word and a reason that names its object, so U2 can disagree with exactly one adjudication without reading the discovery corpus. A pair with no row is a defect; a summary sentence never stands in for a row; a counterpart entry that cannot be read carries `not readable from this tree` rather than an invented pairing. (project AC-001, DR-1, AC-UX-03, IQ-2) | Tier 2 content review, primary and sufficient on its own ([`../_decomposition.md`](../_decomposition.md), `## Testing brief`, AC-### → tier mapping, AC-001 row): the reviewer reads the distillation and the record side by side against AC-UX-03. Tier 1 corroboration: `rg -n "^\| " .bklg/docs-that-teach/durable-audience-closeout/reconciliation-ledger/_reconciliation.md` yields at least nine body rows (seven this side, at least the two counterpart entries named from this tree), and `rg -n "merged\|superseded\|kept distinct\|no counterpart staged\|not readable from this tree"` accounts for every row's verdict cell. |
| **AC-002** | **GIVEN** U3 opening this record months later to correct it, **WHEN** they read the prose immediately above the ledger, **THEN** they learn which of the three merge-order cases actually held, *what was looked at* to determine it (`ls .bklg`, whether the sibling distillation path resolves, `ls .kb/_intake/` and `ls .kb/product/`), and the ledger beneath reads coherently under that case — including the case, live today, in which HS-S0131 has still not run and this initiative's set stands as the authored one, with the table degrading to `no counterpart staged` rows rather than a paragraph bolted on afterwards. The record never conflates "HS-S0131 ran and staged output" with "the sibling's distillation became readable post-merge". (project AC-002, DR-2, AC-UX-03, B-2) | Tier 2 content review, primary, run **parametrically** per [`../_decomposition.md`](../_decomposition.md) `## Testing brief`, "The unrun counterpart": verify the stated case matches what `ls .bklg` shows at the moment of reading, **and** that the prose would still parse sensibly under the other case. Tier 1 corroboration: `ls .bklg` and `ls .kb/_intake/` on the merged tree, cross-checked against the record's stated observations. |
| **AC-003** | **GIVEN** U3 wanting to re-run the observation rather than trust it, **WHEN** they read the record's attestation, **THEN** they find the merged commit as a full 40-character sha, the branch it was read on, and the command that produced it — enough to check the tree out and repeat every reading in the ledger. A ref name alone does not satisfy this; refs move, and the whole point of ordering 1 ("merge before everything") is that a reader can tell a stale observation from a current one. (project AC-012, AC-A06, AC-UX-12) | Tier 1 static, primary: `rg -n "[0-9a-f]{40}" .bklg/docs-that-teach/durable-audience-closeout/reconciliation-ledger/_reconciliation.md` finds the sha, and `git cat-file -t <sha>` resolves it to a commit on the merged branch. Tier 2 confirms the prose states *what* was read, *on which tree*, and *when* — not merely that a sha exists ([`../_decomposition.md`](../_decomposition.md), `## Testing brief`, AC-012 row). |
| **AC-004** | **GIVEN** the two downstream implementers who consume this record — `charter-open-question-disposition` (the evaluator decision) and `staged-audience-payload` (the surviving pairs and their caveats) — **WHEN** they open it as their only input, **THEN** they find the evaluator pair adjudicated *with reasoning*, resolved one way (a persona in its own right, or an earlier stage of the application author's journey) and naming both the differing verbs and the shared fear the distillation records; and they find all three carried qualifications stated as handoff obligations — per-persona observation status, the adapter author's internal-audit-only basis (`:336-342`), and the HS-S0131 overlap (`:343-359`), the last of which is this record's own output. The record marks these as inputs to those stories and discharges neither: it does not edit the charter and it writes no atom text. (project AC-001; DR-4; IQ-7; [`../_storymap.md`](../_storymap.md) `## Coverage`, AC-003 and AC-006/AC-007 seams) | Tier 2 content review, primary: read the evaluator row and its adjacent sentence against [`../../_decomposition.md`](../../_decomposition.md) `:134-139` and the distillation's own open question (`:370-376`); confirm each of the three qualifications appears. Tier 1 corroboration: `git diff --name-only` shows no change to `.bklg/docs-that-teach/initiative.md` and no file under `.kb/`. |
| **AC-005** | **GIVEN** U2 asking "did this closeout quietly invent a decision?", **WHEN** they read the `## Routed gaps` section, **THEN** every finding that wanted a decision is written down there in prose with what was found, why it is not settled here, and where it goes (a later initiative, the `support` initiative per `.redkiln/config.yaml:5`, or a charter open question) — and the diff contains no addition to and no modification of `.kb/decisions/`. If there are no gaps, the section says so in a sentence rather than being absent, because an absent section and a section reporting nothing are indistinguishable to a reader and only one of them is evidence. (AC-A09; [`.kb/governance/rewrite-the-referent-never-the-reasoning.md`](../../../../.kb/governance/rewrite-the-referent-never-the-reasoning.md); [`../project.md`](../project.md), "Out of scope", writing an ADR as a side effect) | Tier 1 static, sufficient for the prohibition: `git diff --name-only` contains no path under `.kb/`, and `redkiln validate --kb`'s accepted-decision immutability check against `HEAD` fails structurally on any touch ([`../_decomposition.md`](../_decomposition.md), `## Testing brief`, Notes). Tier 2 for the section's presence and its routing: read `## Routed gaps` against this criterion. |
| **AC-006** | **GIVEN** a closeout reviewer who has never heard of this story's slug, **WHEN** they open [`../project.md`](../project.md) and read its `## Companions` list, **THEN** the reconciliation record is named there with link text that names the destination, reachable in one hop — and `git diff` over `project.md` shows an appended bullet and **no deletion line anywhere**, so every `file:line` anyone already cited into that file still points where it pointed. A record reachable only by knowing this story's slug is IQ-5's named falsifier. (IQ-3, IQ-5, IQ-6, AC-A05, AC-UX-06 applied to this story's one mount) | Tier 3 mount-point walk, primary: open `project.md`, follow the `## Companions` link, land on the record. Tier 1 corroboration: `git diff -- .bklg/docs-that-teach/durable-audience-closeout/project.md` contains no line beginning `-` outside the diff header, and `redkiln verify --grain story` reads this story's `_ledger.md` and the PR boundary block (`.redkiln/config.yaml:40`, `:62-67`). |
| **AC-007** | **GIVEN** any reader consuming this record one row at a time — a screen reader, a `git diff` hunk, a quotation in a review comment — **WHEN** they meet a single row or a single sentence out of context, **THEN** they lose nothing: every state is a literal word (no tick, no emoji, no colour word, no strikethrough-as-status, no blank cell meaning "fine", no state inferable only from a row's absence); every row names its side, item, counterpart, verdict, reason and source path without depending on the row above; and every `superseded`, every `kept distinct` and every routed gap also exists as a sentence outside the table. The corpus precedent is [`.kb/maps/open-questions-index.md`](../../../../.kb/maps/open-questions-index.md), which spells `Open` / `Withdrawn` / `Superseded` as words, first. (AC-UX-09, AC-UX-10, IQ-8, the accessibility floor's "colour, glyph and position never alone") | Tier 2 content review against AC-UX-09 and AC-UX-10, primary. Tier 1 corroboration: `rg -n "✅\|❌\|🟢\|~~" .bklg/docs-that-teach/durable-audience-closeout/reconciliation-ledger/_reconciliation.md` returns nothing, and `rg -n "^\|\s*\|"` finds no empty leading cell. |
| **AC-008** | **GIVEN** U2 reading this record beside the corpus's other written records, **WHEN** they take in its shape before reading a word of its content, **THEN** it composes from the corpus's existing primitives and nothing invented: one `#` title and `##` sections with no skipped level; exactly the four sections in the fixed order merge-order statement → pair ledger → routed gaps → sha attestation, with the prose case stated *above* the table it governs; exactly the six columns of B-5 and at least nine body rows; exactly the five admissible verdict words; the ledger persistent on the page rather than revealed, collapsed or summarised behind a lead sentence; and **no YAML frontmatter, no bespoke per-row table, no "Persona card" vocabulary, no new map format** — the record is a project artefact under `.bklg/`, not an atom, and hand-rolling beside the primitive layer is exactly what [`../_decomposition.md`](../_decomposition.md) `## UX brief`, "Hand-rolling, explicitly forbidden here" refuses. (AC-UX-03 placement clause; the a11y floor's heading-structure and linear-readability clauses; `## Architecture brief` §4, "the record itself stays under `.bklg/`") | Tier 2 content review, primary, as a structural read against the numbers above. Tier 1 corroboration: `rg -n "^#" _reconciliation.md` yields one `#` and exactly four `##` in the stated order; the file's first line is not `---`; the ledger's header row has six cells. |

## Interaction quality

RFC §6.7/D6. Two families, and neither is a prose checklist: **every invariant that applies is carried
by an AC-### row in the table above**, because `redkiln verify` extracts criteria by matching a leading
`| AC-001 |` cell or a `- AC-001:` bullet — an invariant left as a bullet here would get no ledger row,
never be gated and never be checked. This section names only which id carries what, and how it is seen.

**A note on where the composition family comes from.** [`../_design.md`](../_design.md) records
`hasSurface: false` with an empty `## Items` block, signed off 2026-08-17 with no conditions, so there
is no approved screen composition for this story to implement or contradict. That does *not* void the
composition family — it relocates it. The binding written description of what this artefact must look
like is the UX brief's artefact **3 — the reconciliation record** together with its design-system
primitives table, its accessibility floor and AC-UX-03/09/10/12 ([`../_decomposition.md`](../_decomposition.md),
`## UX brief`). In a text corpus the primitive layer is the atom template, the state-word vocabulary and
the two map formats, and "compose, do not hand-roll" is testable against them rather than metaphorical.

**State invariants**

| Invariant | Carried by | How it is seen |
| --- | --- | --- |
| **In place, not a context jump** — the record answers *which case held, which pairs survived, on what evidence* on first load; links carry provenance, never a verdict or a reason | AC-001, AC-002 | Tier 2: a row whose reason reads "see the distillation" fails, exactly as IQ-1's falsifier does for an atom |
| **Non-occlusion — a summary must not hide what it summarises** | AC-001 (the ledger, not the summary), AC-005 (an empty gaps section still speaks), AC-008 (the ledger is persistent, never collapsed or revealed) | Tier 2 row-by-row read; Tier 1 row count |
| **Preserved position / preserved selection** — nothing already cited moves, and prior citations keep meaning what they meant | AC-006 | Tier 1: `git diff` over `project.md` carries no deletion line; the discovery distillation is cited, never edited |
| **Reversibility** — a later correction routes as *supersede*, never *edit*; the observation can be re-run rather than re-trusted | AC-003 (re-runnable attestation), AC-005 (a finding routes as a gap, never as an edit to an accepted decision) | Tier 1 sha resolution; Tier 2 read of the routing sentences |
| **Reachable without prior knowledge** (the keyboard-reachability analogue) | AC-006 | Tier 3 one-hop walk from `project.md`'s `## Companions` |

**Composition invariants**

| Invariant | Carried by | How it is seen |
| --- | --- | --- |
| **Presentation exists at all** — the record is a composed document, not a dumped table: a stated case, a ledger, routed gaps, an attestation | AC-008 | Tier 2 structural read: four `##` sections in the fixed order |
| **Composition and placement** — the merge-order case sits *above* the ledger it governs (AC-UX-03), the sentences sit *outside* the table | AC-008, AC-007 | Tier 2; a case stated below the table fails AC-UX-03 as written |
| **Transience** — the ledger is persistent chrome, never revealed-on-demand or summarised away; the routed-gaps section is persistent even when empty | AC-008, AC-005 | Tier 2 |
| **Density budget, with its numbers** — six columns; ≥ 9 body rows (3 personas + 4 journeys this side, ≥ 2 counterpart entries); 5 admissible verdict words; 1 `#`; 4 `##`; 0 glyphs; 0 frontmatter lines | AC-008, AC-001, AC-007 | Tier 1 counts, Tier 2 judgement on whether a row earns its cells |
| **Hierarchy** — one `#`, sections at `##`, no skipped level, no heading carrying state | AC-008 | Tier 1 `rg -n "^#"` |
| **Named anti-patterns** — a glyph, tick, colour word, strikethrough or empty cell as status; a state expressed by ordering or omission; invented frontmatter; a bespoke per-row table; promoting the record wholesale as a `.kb/` atom | AC-007 (status by glyph or omission), AC-008 (invented shape), AC-005 (the atom/decision temptation) | Tier 1 `rg` for the glyph set and for a leading `---`; Tier 2 for the rest |

## Error conditions

| ID | Condition | Required behaviour |
| --- | --- | --- |
| **EC-001** | The merged sha cannot be determined — `merge-forward-baseline` (HS-S0172) has not landed, or the branch in this worktree carries no merge commit from `initiative/from-contract-to-published-library` | **Halt and report.** Do not perform the merge (out of scope, PR boundary) and do not write a placeholder, a short sha or "the current branch". Ordering 1 makes every observation downstream of the merge worthless without it ([`../../_decomposition.md`](../../_decomposition.md) `:89-103`). The story blocks on its declared dependency. |
| **EC-002** | A counterpart entry is *named* from this tree but its content is not readable — e.g. HS-S0131's two unnamed personas, or its journeys | Record the row with `not readable from this tree`, naming what was looked at and where. Never infer the entry's content from its name. Inventing personas is the single failure mode this project exists to prevent ([`../project.md`](../project.md), "Out of scope"). |
| **EC-003** | A reconciliation finding appears to require a standing decision | Write it as a routed gap under `## Routed gaps` (AC-005). Do not author, amend or supersede a decision atom — an accepted decision is immutable and supersession is a deliberate act, not closeout exhaust ([`.kb/governance/rewrite-the-referent-never-the-reasoning.md`](../../../../.kb/governance/rewrite-the-referent-never-the-reasoning.md)). |
| **EC-004** | The evaluator pair resists a clean resolution | State a resolution *and its reasoning* anyway, marked as the input `charter-open-question-disposition` consumes. `TBD`, `open`, or a row deferring to a later story is not admissible — the decomposition already deferred this question once, to *this* point (`:134-139`), and deferring it again leaves it open in two distillations, which AC-003 exists to prevent. |
| **EC-005** | A slice-mate (`charter-open-question-disposition`, `design-tension-audit`) has already appended to `project.md`'s `## Companions` list, or does so concurrently in the same implementer context | Append; never reflow, re-order or re-wrap the existing bullets. Resolve any conflict by re-appending this story's single bullet on top of theirs. A reflow that shifts a cited `file:line` fails AC-006 even when the rendered list looks identical. |
| **EC-006** | The record's stated merge-order case and the tree disagree at review time (the sibling landed between authoring and review) | Re-determine by looking (B-2), rewrite the prose statement and the affected rows, and re-run AC-002's parametric read. The record states what it found, at the sha it names; a stale case silently left in place is the unstated single-sided authoring the project's risk table names. |

## Non-functional

| ID | Requirement | Why it is not a footnote |
| --- | --- | --- |
| **NF-001** | **Self-sufficiency for U2.** A reviewer can evaluate any single row without opening the discovery corpus, while every row still cites the path it was read from so they may. | This is the whole reason the unit is the pair rather than the set ([`../_decomposition.md`](../_decomposition.md), `## UX brief`, reader table, U2). |
| **NF-002** | **Re-runnability for U3.** Every observation in the record names the tree it was performed against and the command that produced the naming, so the reading can be repeated rather than trusted. | AC-UX-12; the difference between evidence and assertion. |
| **NF-003** | **Diff hygiene.** The change is markdown only and maps to no workspace package; `cargo xtask affected --base main` (`.redkiln/config.yaml:40`) is expected green with an empty affected set, and that no-op result is the correct outcome, not a skipped check — the command still runs the five file-reading lints and `spec-trace` unconditionally (`:36-39`). | A markdown-only story that reports a *compiled* package has touched something it should not have. |
| **NF-004** | **Correction discipline.** Where this record is later found wrong, the stated route is a superseding correction with its own sha attestation, never an edit that silently rewrites an adjudicated row. | [`.kb/governance/rewrite-the-referent-never-the-reasoning.md`](../../../../.kb/governance/rewrite-the-referent-never-the-reasoning.md); AC-UX-12's second clause. |
| **NF-005** | **No new checklist.** Verification runs against IQ-1…IQ-8 and AC-UX-01…AC-UX-12 as they already stand; this story adds no parallel review list for the same artefact. | AC-TB-06 — one place a reviewer needs open, or the checklist stops being read. |

## Implementation notes (non-prescriptive)

- **Determine before you draft.** The merge-order case is an observation, not an assumption. Run the
  four looks in B-2 and paste what they returned into the prose statement. The `_grounding.md` and
  `_storymap.md` both say the *unrun* case is the one most likely to hold — that makes it the default
  drafting shape, not a foregone conclusion.
- **Draft the ledger before the prose.** The prose statement has to be true of the table beneath it;
  writing it first tends to produce a case the rows then quietly contradict.
- **Enumerate this side mechanically.** Three personas at `:56`, `:148`, `:225` and four journeys at
  [`../../initiative.md`](../../initiative.md) `:289-301`. Seven rows before a counterpart is
  considered. Then add one row per counterpart entry the tree exposes, readable or not.
- **The sentences are not decoration.** Write the `superseded` / `kept distinct` / routed-gap sentences
  as you write the rows, not as a pass afterwards — a pass afterwards is how a row and its sentence
  drift apart, which is worse than having neither.
- **Do not run `git merge`.** If the sha is missing, that is EC-001.
- **Coordinate the `## Companions` append with the slice-mates.** All three `closeout-adjudication`
  stories mount into the same list in one implementer context. One appended bullet each, no reflow.
- **Resist two temptations by name**: promoting the record as an atom (it is a project artefact —
  `## Architecture brief` §4), and citing a decision atom for authority (none reaches this surface —
  Context pack 5).

## Tests and CI (merge gate)

Grounded in [`../_decomposition.md`](../_decomposition.md), `## Testing brief`. This story's ACs sit at
Tier 2 primary for the content criteria (AC-001, AC-002, AC-004, AC-007, AC-008), Tier 1 primary for the
shape criteria (AC-003, AC-005) and Tier 3 for the mount (AC-006) — which is exactly what AC-TB-01
requires, since Tier 1 alone can never falsify a *content* criterion.

| Tier | Command / path | Proves |
| --- | --- | --- |
| **1 — static / shape** | `ls .bklg` and `ls .kb/_intake/` on the merged tree | The merge-order case the record states matches what the tree shows (AC-002 corroboration) |
| **1 — static / shape** | `rg -n "[0-9a-f]{40}" .bklg/docs-that-teach/durable-audience-closeout/reconciliation-ledger/_reconciliation.md`, then `git cat-file -t <sha>` | The attestation names a real, resolving merged commit (AC-003) |
| **1 — static / shape** | `git diff --name-only` over the PR boundary | No path under `.kb/` — in particular none under `.kb/decisions/` — and no path under `crates/`, `spec/`, `xtask/` or `standards/` (AC-005) |
| **1 — static / shape** | `git diff -- .bklg/docs-that-teach/durable-audience-closeout/project.md` | The `## Companions` change is an appended bullet with no deletion line, so no cited `file:line` moved (AC-006) |
| **1 — static / shape** | `rg -n "✅\|❌\|🟢\|~~" _reconciliation.md`; `rg -n "^#" _reconciliation.md` | No glyph carries state; one `#`, four `##`, no skipped level, no leading `---` (AC-007, AC-008) |
| **2 — content review** (the "unit" analogue; a human, one claim at a time) | The record read against [`../_decomposition.md`](../_decomposition.md)'s IQ-1…IQ-8 and AC-UX-03, AC-UX-09, AC-UX-10, AC-UX-12, side by side with [`../../_discovery/distillation/personas-and-journeys.md`](../../_discovery/distillation/personas-and-journeys.md) | Every pair has a row; the case reads coherently and would still parse under the other case; the evaluator adjudication carries reasoning; the three qualifications are present; no summary replaces a row (AC-001, AC-002, AC-004, AC-007, AC-008). This is the only tier that can fail a plausible wrong implementation of these — a complete-looking ledger with one silently missing pair passes every machine check in this table |
| **3 — integration / mount** | Open [`../project.md`](../project.md), follow the `## Companions` entry to the record | The artefact is reachable by a reviewer who does not know this story's slug (AC-006) |
| **story gate** | `cargo xtask affected --base main` (`.redkiln/config.yaml:40`) | Nothing outside markdown moved: an empty affected set plus green file-reading lints and `spec-trace`. A non-empty package set here means the diff left this story's boundary (NF-003) |
| **story gate** | `redkiln verify --grain story` over this story's `_ledger.md` and the PR boundary block (`.redkiln/config.yaml:62-67`, `:69-73`) | Every AC-### above carries a `satisfied: true` row with real cited evidence, and no file changed outside the declared boundary |

Explicitly **not** run by this story: `cargo xtask ci` (the terminal `e2e` grain, `.redkiln/config.yaml:60`)
is `terminal-gate-run`'s and must be taken *last*, on the tree that already carries every artefact —
a run taken here would prove the gate, not the deliverable ([`../_decomposition.md`](../_decomposition.md),
`## Architecture brief`, ordering 4). `redkiln validate --kb` is likewise not this story's bar: it never
reads anything in this diff.

## Risks and coupling (PR-scoped)

| Risk | Coupling | Mitigation in this PR |
| --- | --- | --- |
| The record becomes a single-sided authoring wearing a reconciliation's clothes | Directly the charter's named failure mode ([`../project.md`](../project.md), risk table, first row) | AC-002 forces the case to be *determined and stated*; AC-001 forces a row even for `no counterpart staged`. The unrun case is the default shape, not an apology appended to a shape built for the other case |
| A counterpart persona gets inferred from its name because a blank row looks unfinished | HS-S0131's two unnamed personas | EC-002 and AC-001's `not readable from this tree` verdict word make "unreadable" a legitimate, visible outcome rather than a gap someone feels pressure to fill |
| A verdict lives only in a table cell and is lost in the review quote that decides it | U2's whole reading mode | AC-007's plain-text-equivalence clause: every `superseded`, `kept distinct` and routed gap is also a sentence |
| The evaluator adjudication is deferred *again*, this time to `staged-audience-payload` | `charter-open-question-disposition` (HS-S0175) consumes this decision; `staged-audience-payload` (HS-S0177) transcribes it | EC-004 makes `TBD` inadmissible. The storymap's `## Coverage` seam is explicit: this record and its consumer *decide*, the payload story only *transcribes* |
| A reconciliation finding turns into an ADR as closeout exhaust | `.kb/decisions/` immutability | AC-005 routes it as prose; AC-A09 and `validate --kb`'s `HEAD` check make a touch fail structurally |
| Three slice-mates edit the same `## Companions` list in one context and someone reflows it | `charter-open-question-disposition`, `design-tension-audit` | EC-005; AC-006's no-deletion-line check catches a reflow even when the rendered list looks unchanged |
| The record is promoted wholesale into `.kb/product/` because it is the best-written artefact in the folder | `staged-audience-payload`, `audience-ingest-wave` | Context pack 6 and AC-008: the record is a project artefact; only its *outcome* travels, into atom bodies and `source_paths` |
| The sha is recorded as a ref or a short sha because it is shorter | `merge-forward-baseline` upstream; `post-merge-clause-completeness` alongside | AC-003 requires 40 characters plus branch plus producing command, and `git cat-file -t` is the check |

## Dependencies

**Blocks on** — `merge-forward-baseline` (HS-S0172, milestone `merged-tree-baseline`). It produces the
merged commit this record must name and read on. This is ordering 1 of the architecture brief's data
flow: everything here observed against this worktree's pre-merge copy measures a stale tree
([`../../_decomposition.md`](../../_decomposition.md) `:89-103`). EC-001 governs the case where it has
not landed.

**Unlocks** —

- `charter-open-question-disposition` (HS-S0175), same milestone, immediately downstream: it marks the
  charter's evaluator open question answered *against the reasoning recorded here* ([`../_storymap.md`](../_storymap.md),
  `## Merge order`, milestone 2: "the evaluator decision must exist before any atom text claims it").
- `staged-audience-payload` (HS-S0177), milestone `product-layer-promotion`: it transcribes the surviving
  pairs, the supersession statements and the HS-S0131 overlap qualification into staged atom text.

**Not a dependency in either direction** — `design-tension-audit` (HS-S0176) is a slice-mate implemented
in the same context and mounted into the same `## Companions` list, but it consumes nothing from this
record and produces nothing this record needs; the storymap runs it in parallel with either sibling.

## Anchors (progressive disclosure)

Every path below resolves in this worktree. Link, open at the stated moment, and do not paste in bulk —
the Context pack above already carries the decisions; these carry the detail behind them.

| Anchor (real path) | Why it is load-bearing | When to open | Serves |
| --- | --- | --- | --- |
| `.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md` | The authoritative text of this side: the three personas at `:56`, `:148`, `:225`; the risks at `:317-359` carrying the observation, adapter-audit and HS-S0131-overlap qualifications; the open questions at `:361-397` including the evaluator's | Before writing the first ledger row, and again when drafting the evaluator row's reasoning | AC-001, AC-004 |
| `.bklg/docs-that-teach/initiative.md` | The four journeys this side enumerates (`:289-301`) and the three qualifications the charter says must not be dropped (`:303-311`); DoD-15's literal text (`:465-468`) | While enumerating the journey rows, and before claiming the qualifications are complete | AC-001, AC-004 |
| `.bklg/docs-that-teach/durable-audience-closeout/_decomposition.md` | The binding written description of this artefact: `## UX brief` artefact 3, the state vocabulary, the accessibility floor, IQ-1…IQ-8, AC-UX-03/09/10/12, and `## Testing brief`'s tier mapping and parametric-read instruction | Open the UX brief before drafting the table's shape; open the testing brief before writing any ledger evidence | AC-007, AC-008, AC-002 |
| `.bklg/docs-that-teach/durable-audience-closeout/project.md` | DR-1's verdict vocabulary and DR-2's either-order rule; project AC-001/AC-002/AC-012 verbatim; and the `## Companions` list at `:291-301` that is this story's mount | Read DR-1/DR-2 before drafting; open `## Companions` when mounting | AC-001, AC-006 |
| `.bklg/docs-that-teach/durable-audience-closeout/_grounding.md` | "Reconciliation counterpart does not exist in this tree" — the verified-by-listing basis for the default drafting case, and the note that nothing observed in this planning worktree counts | Before stating the merge-order case, to see what was already looked at and by what method | AC-002 |
| `.bklg/docs-that-teach/durable-audience-closeout/_storymap.md` | The slice's merge order and the explicit instruction that the unrun case is the record's default shape "not a paragraph bolted on afterwards"; the `## Coverage` seams that fix what this story decides versus what downstream stories transcribe | When the unrun case tempts a shortcut, and when deciding how much of the evaluator question to settle here | AC-002, AC-004 |
| `.bklg/docs-that-teach/_decomposition.md` | The initiative-level merge-forward rule (`:89-103`), the run-parallel-and-reconcile resolution (`:116-123`) and the evaluator deferral to promotion (`:134-139`) — the three decisions that make this story exist in this shape | Before EC-001 or EC-004 has to be applied | AC-002, AC-004 |
| `.bklg/docs-that-teach/durable-audience-closeout/merge-forward-baseline/spec.md` | The upstream story that produces the merged sha; its boundary is where the merge is performed and this one's is not | When determining the sha, and if EC-001 fires | AC-003 |
| `.kb/governance/rewrite-the-referent-never-the-reasoning.md` | The governing rule for corrections: a change to a standing decision is a new atom that supersedes, never an edit — and, read the other way, the reason a closeout may not mint a decision as exhaust | Before writing anything into `## Routed gaps`, and before any impulse to "just record this as an ADR" | AC-005 |
| `.kb/product/README.md` | What must not travel into an atom (`:26-43`) — the boundary that keeps this record a project artefact — and the persona vocabulary the downstream payload story will need the qualifications phrased for | When drafting the handoff sentences, and if promotion of the record itself is ever proposed | AC-004 |
| `.kb/maps/open-questions-index.md` | The corpus precedent for state-as-a-word: `Open` / `Withdrawn` / `Superseded` spelled literally and first (`:136-143`), and supersession annotating rather than deleting (`:11-13`) | When choosing how a verdict cell is written | AC-007 |
| `.redkiln/config.yaml` | `affected_gate` (`:40`), `require_ledger` (`:62-67`) and `require_commit_provenance` (`:69-73`) — the three keys that decide what `redkiln verify --grain story` actually enforces on this PR; `:5` names where a routed incidental bug goes | Before running the story gate, and when routing a gap to the `support` initiative | AC-005, AC-006 |

## Clarifications resolved during spec

1. **The AC set is eight, not the seven the first pass carried.** AC-008 was added to carry the
   composition family — structure, placement, transience, the density numbers, the hierarchy and the
   named anti-patterns — rather than overloading AC-007, which carries the accessibility floor and the
   plain-text-equivalence clause. RFC §6.7/D6 requires every applicable composition invariant to be a
   *table row*, and one row whose evidence had to cite six unrelated structural facts would have been a
   row nothing could honestly falsify. The ledger enumerates AC-001 … AC-008 to match.
2. **The composition family binds even though `_design.md` records `hasSurface: false`.** The signed-off
   design declares no surface and therefore fixes no composition; the binding description is the UX
   brief's artefact 3 plus its design-system primitives table and accessibility floor. This spec
   implements that description and does not re-decide it. Recorded explicitly so a reviewer does not
   read `hasSurface: false` as licence to skip presentation entirely.
3. **"Real-path test" here means a tier plus a command, not a Rust test.** This project compiles no new
   Rust and the testing brief is explicit that Tier 2 — a human reading one claim at a time against
   IQ-1…IQ-8 and AC-UX-01…AC-UX-12 — is the *only* tier that can fail a plausible wrong implementation
   of AC-001, AC-002 and AC-004. `verifying_test` in `_ledger.md` therefore names the tier and the
   checklist path or command; the evidence column names the `file:line`, which is exactly the division
   the testing brief's Notes fix.
4. **The record's path is fixed by this spec, and the architecture brief permitted either.** §6 left it
   between "a companion under this project's folder" and "the body of `closure.md`"; `closure.md` does
   not exist until the project's closeout stage renders it, so a companion in this story's own folder is
   the only option that keeps the record inside this story's PR boundary. Recorded in the Integration
   contract rather than left to the implementer.
5. **`cargo xtask ci` is deliberately absent from this story's gate.** It is the terminal `e2e` grain and
   belongs to `terminal-gate-run`, last in the project. The story-grain gate here is
   `cargo xtask affected --base main`, whose correct result on a markdown-only diff is an empty affected
   set — stated so that a green no-op is not mistaken for a check that failed to run.
6. **No decision atom is cited anywhere in this spec, and that is the finding.** None of the sixteen
   accepted decisions reaches this surface ([`../../initiative.md`](../../initiative.md) `:524-525`);
   the authorities that do bind are the governance atom, the single-write-path rule and the project's own
   DR-1/DR-2. Citing an ADR here would have been manufacturing authority.
