---
item: HS-S0175
stage: spec
created: 2026-08-17T13:16:26.791Z
updated: 2026-08-17T13:16:26.791Z
template_sig: 87bbf1d0
rendered_sig: 90ec5f86
---

# Spec — Dispose of every charter open question, evaluator included

## Scope lock

| | Path |
| --- | --- |
| Initiative (gold source) | [`.bklg/docs-that-teach/initiative.md`](../../initiative.md) — `## Open questions for the planning team` at `:513-555` is this story's subject matter, and `## Exit criteria` at `:672-673` is the criterion it discharges |
| Initiative decomposition | [`.bklg/docs-that-teach/_decomposition.md`](../../_decomposition.md) — the evaluator resolution at `:134-139`, the merge-forward rule at `:89-103`, the parallel-and-reconcile decision at `:116-123` |
| Project | [`.bklg/docs-that-teach/durable-audience-closeout/project.md`](../project.md) — AC-017, AC-003, DR-13, DR-6 |
| This spec | `.bklg/docs-that-teach/durable-audience-closeout/charter-open-question-disposition/spec.md` |
| Key briefs | [`../_decomposition.md`](../_decomposition.md) — `## UX brief` (IQ-2 supersession annotates, IQ-3 preserved position, AC-UX-07, AC-UX-09, AC-UX-10), `## Architecture brief` (AC-A01, AC-A02, AC-A05, AC-A06, AC-A09, §5 data flow), `## Testing brief` (AC-017 is Tier 2 primary, AC-003 is Tier 2 primary) |
| Signed-off design | [`../_design.md`](../_design.md) — `hasSurface: false`, approved 2026-08-17 with no conditions. No public API, no rendered screen; the surface is the written record |
| Story map | [`../_storymap.md`](../_storymap.md) — the `closeout-adjudication` row for this slug, the M2 merge order, and the "no story authors or touches `.kb/decisions/`" note |
| Roadmap pointer | None. `RUNBOOK.md` owns no documentation phase — that absence is charter question Q3 and is dispositioned by this story, not resolved by it |

## One-line PR slice

Dispose of every question in the charter's `## Open questions for the planning team` — answered on the
record, or staged as an `open_question` atom for this project's single ingest wave — and settle the
evaluator question once, with its reasoning, marking the charter's line answered.

## Executive summary

This PR lands one written record and, conditionally, one or more staged intake documents.

**Delta against the tree as it stands.** The charter carries eleven bullets under
`## Open questions for the planning team` (`initiative.md:517-555`): nine questions and two status
statements. Nothing in the tree says what happened to any of them. `.kb/open-questions/` holds twenty
atoms, **all** about the Rust contract and the specification — not one concerns documentation, audience
or narrative structure, so no charter question has an existing atom to be merged into and the
"documentation" domain has no `##` section in `.kb/maps/open-questions-index.md` (two sections today,
`:33` and `:77`, both contract-shaped). The evaluator question (`initiative.md:538-540`) was
deliberately *deferred* to promotion by the decomposition (`:134-139`) rather than answered, so it is
open twice — once in this initiative's distillation and once in the sibling's.

This story closes that. It writes `_disposition.md` in its own folder: one row per charter bullet, a
state word from a fixed vocabulary, and for every `answered` row the artefact that *actually decided
it*, by path. It settles the evaluator question in one place with its reasoning. It appends one `##`
section to the end of `initiative.md` so the charter itself carries the disposition. Where a question
cannot honestly be answered, it stages an `open_question` document under `.kb/_intake/` in
`.kb/open-questions/README.md`'s shape and hands `audience-ingest-wave` the file name, the target
index section and the bullet text.

It writes no `.kb/` atom, edits no `.kb/map`, and touches no sibling's content — those land in M3.

## Context pack

**The record does not answer the questions; it records where the work answered them.** The charter's
own non-goal is that *"settling one in passing is a defect, not progress"* (`initiative.md:165-166`),
and the same sentence heads the open-questions section itself (`:515`). So an `answered on the record`
row is a *citation*, not an argument: it names the artefact in this initiative that made the decision —
a sibling project's `_design.md`, a shipped page, a story's report, a distillation — by path, and where
that artefact is long, by `file:line`. **A row whose only authority is this record is not answered; it
is deferred.** That single rule is what keeps this story from becoming the eleven-question think-piece
the charter forbids, and it is the plausible wrong implementation the review exists to reject.

**The one exception is the evaluator, and it is an exception on the record.** The decomposition
resolved that the *initiative* plans against three personas and explicitly **deferred the durable
question to promotion, to be settled jointly with HS-S0131's set rather than twice**
(`.bklg/docs-that-teach/_decomposition.md:134-139`). This story is that promotion moment. It decides —
`a persona in its own right` or `an earlier stage of the application author's journey` — states the
reasoning, and that decision is binding downstream: `staged-audience-payload` *transcribes* it into the
atom text and does not re-open it (`../_storymap.md`, Coverage table, AC-003 seam). The decision must be
coherent with the evaluator pair row in `reconciliation-ledger`'s record, which is why that story blocks
this one; both name the same merged sha.

**Nothing here is observed against this worktree's pre-merge copy.** AC-A06 is unconditional: the
merge-forward of `initiative/from-contract-to-published-library` precedes this record, and this record
names the merged commit sha (`../_decomposition.md`, `## Architecture brief`, AC-A06 and §5 ordering 1).
That is not ceremony for this story — charter bullet Q10 (`initiative.md:544-552`) is a standing
instruction to **re-verify three specific paths against the merged tree**
(`examples/outside-projection-adapter/`, `references/seeds/measured-not-claimed.md`, and
`publication-and-positioning`'s asserted `_design.md`/HS-S0131 stage), none of which resolves in this
tree today. Disposing of Q10 means running that check on the merged tree and recording each path
present or absent. A Q10 row that says "verified" without the merged sha beside it has verified nothing.

**Deferral has exactly one shape, and this story does not perform it.** A question that cannot be
answered becomes a document staged in `.kb/_intake/`, in the shape `.kb/open-questions/README.md`
fixes — *what is true today / what is not decided / what forces it*, plus ordered sub-questions, every
claim grounded by a path that goes into `source_paths`. Staged material **is not an atom** and is not
held to `KbFrontmatter` (`.kb/_intake/README.md:7-11`). The atom is written by `/redkiln:kb-ingest`,
the corpus's only writer of `.kb/` (AC-A01; precedent `0269720`), which runs in M3 as
`audience-ingest-wave`. This story therefore **cannot** land the index bullet AC-017 names — the atom
it would point at does not exist until the wave runs. What this story owes instead is a handoff block:
per deferred question, the staged file name, the target `##` section in
`.kb/maps/open-questions-index.md`, and the bullet text in that index's fixed shape — status word
first, then the link, then the id, then one sentence (`.kb/maps/open-questions-index.md:136-143`;
AC-UX-07). Documentation has no section in that index today, so a **new** `##` section is the correct
move and the index's own rule permits it (`:137-139`); the UX brief's prohibition is on a new section
when the domain *already has one*.

**Append; never edit, never delete.** The charter annotation is a single appended `##` section at the
**end of `initiative.md`**, not an edit to the bullets at `:517-555`. Two reasons, both load-bearing.
IQ-2: *"supersession annotates, it does not delete"* — a disposed question stays legible as a question,
the way `.kb/maps/open-questions-index.md:11-13` keeps a withdrawn question listed. IQ-3: *"nothing this
project lands may move an anchor someone else already cited"* — `initiative.md` is cited by `file:line`
from `project.md`, both `_decomposition.md` files and the briefs (`:538-540`, `:524-525`, `:275-288`,
`:416-420`), and appending at EOF shifts none of them, where an in-place annotation would shift every
line below the insertion. The falsifier is mechanical: `git diff .bklg/docs-that-teach/initiative.md`
containing any `-` line.

**This is markdown prose, and the CLI still owns the frontmatter.** `initiative.md` is a `.bklg/` item:
its YAML frontmatter is written only by `redkiln`, a `PreToolUse` hook denies the edit, and the body
beneath the closing `---` is free (CLAUDE.md, "Where the work lives"). Appending a `##` section to the
body is a body edit. It does not pre-empt `/redkiln:closeout`'s reconciliation step, which redirects a
*different* section — `## Referenced personas & journeys` — once the atoms exist
(`../_decomposition.md`, `## Architecture brief`, mount point 6).

**No Accepted decision atom governs this surface, and that is a finding rather than a gap.** All sixteen
accepted decisions govern the Rust contract; the charter says so at `:524-525`, and the UX brief's Notes
say it again. Do not go hunting for a documentation ADR to cite. What binds here is
`.kb/governance/rewrite-the-referent-never-the-reasoning.md` (correct by superseding, never by editing),
the single-write-path rule, and AC-A09: **this story's diff contains no addition to and no modification
of `.kb/decisions/`.** A disposition that concludes a question wants an ADR writes that down as a routed
gap in prose and stops there — an ADR authored as a closeout by-product is the practice the corpus was
built to refuse (`project.md`, "Out of scope").

**Every state is a word.** No emoji, no tick, no colour, no strikethrough, no "an empty cell means
fine", no state inferable from a row's absence (AC-UX-09, and the accessibility floor's
colour-never-alone line, which binds *today* in markdown). Every row is self-contained: the question id,
its charter line anchor, its state word, its evidence, and its merged sha where the evidence needed one
— no row that only makes sense read after the one above (AC-UX-10).

**The persona-journey slice.** The reader is **U2, the closeout reviewer** — *"show me every adjudication
you made, including the ones that changed nothing, so I can disagree with exactly one of them without
reading the discovery corpus"* — and secondarily **U1, the next initiative's charter author**, who needs
the evaluator decision to know whether they inherit three personas or four (`../_decomposition.md`,
`## UX brief`, "Who this is actually for"). Neither the application author nor the adapter author nor the
evaluator ever reads this file; the evaluator is its *subject*, never its audience.

## Integration contract

- **Archetype**: `capability` — a written record a real reader reaches from the charter, not substrate
  for a later story.
- **Slice / milestone**: `closeout-adjudication`. Slice-mates: **`reconciliation-ledger`** (blocks this
  story; its evaluator pair row must agree with this story's decision) and **`design-tension-audit`**
  (parallel). All three are written records under
  `.bklg/docs-that-teach/durable-audience-closeout/`, read by U2 in one pass, and all three name the
  same M1 merged sha (`../_storymap.md`, "Why these four and not eleven"; "Merge order", step 2).
- **Mount point**: **`.bklg/docs-that-teach/initiative.md`** — one `##` section appended at end of file,
  `## Open-question disposition (closeout, HS-S0175)`, carrying one row per charter bullet. This is the
  render path: the charter is where a reader meets the questions, so it is where they must meet the
  answers. Body prose only; frontmatter is the CLI's (CLAUDE.md).
- **Wires into**:
  - `.bklg/docs-that-teach/initiative.md:513-555` — the eleven bullets, read as the row set. Q-numbering
    is fixed by line anchor, not by re-ordering.
  - `.bklg/docs-that-teach/durable-audience-closeout/reconciliation-ledger/` — the blocking sibling. The
    evaluator pair adjudication and this story's evaluator decision are one decision stated in two
    records; a contradiction between them is a review-blocking defect.
  - `.kb/open-questions/README.md` — the *what is true today / what is not decided / what forces it*
    shape every staged deferral document takes.
  - `.kb/_intake/` — the staging directory. Files land here; nothing here is an atom
    (`.kb/_intake/README.md:7-11`).
  - `.kb/maps/open-questions-index.md:136-143` — the bullet shape this story authors as text and
    `audience-ingest-wave`'s map-sync step lands.
  - `.kb/governance/rewrite-the-referent-never-the-reasoning.md` — the correction route this record
    states for itself: supersede, never edit.
- **Design-system primitives consumed** (`../_decomposition.md`, `## UX brief`, "Design-system
  primitives"): the open-question body shape, the index bullet shape, the status-word vocabulary
  (`Open` / `Withdrawn` / `Superseded`), and the append-only map discipline. Nothing new is invented —
  no bespoke table format, no per-question frontmatter key, no status glyph.
- **Renders surfaces**: **none** as `_design.md` defines a surface — that file records `hasSurface:
  false` and no `## Items` block, so there is no `path` id for this story to claim. The surface it does
  render is the UX brief's artefact (2), *the navigation surface*: the charter's appended disposition
  section, plus the index-bullet text handed to M3.
- **Conformance rule(s)**: **none, and this is not adapter-observable.** This story compiles no Rust and
  touches nothing under `crates/`; there is no rule id in `suite.rs` that could observe a markdown
  record, and inventing one would be a rule no adapter can fail (CLAUDE.md, "The rule that matters",
  first corollary). Verification is Tier 2 — content review against a named checklist — which the
  testing brief makes AC-017's and AC-003's *primary* tier precisely because no automated reader exists
  (`../_decomposition.md`, `## Testing brief`, "AC-### → tier mapping").
- **Clause(s)**: **none.** `spec/SPECIFICATION.md` is read-only for this project and untouched by this
  story; no clause is discharged or amended, and no `[FROZEN]` clause is approached.
- **Advances DoD scenario**: **DoD-15** — *"the audience is durable and reconciled"*
  (`initiative.md:465-468`). The evaluator decision fixes whether the promoted set is three persona
  atoms or four, which is the "reconciled rather than duplicating" half of that scenario;
  `staged-audience-payload` cannot author the atoms until this decision exists (`../_storymap.md`,
  "Merge order", step 2). It also discharges the initiative exit criterion at `initiative.md:672-673`,
  which is an exit criterion rather than a numbered DoD scenario — stated as such rather than folded in.

Delivered **mounted**: the record exists *and* the charter carries the appended section pointing at it.
A `_disposition.md` that no reader reaches from the charter is the same defect as an atom nothing links
to, and this project's architecture brief names that failure first (`../_decomposition.md`,
`## Architecture brief`, "Intent").

## PR boundary

```
.bklg/docs-that-teach/durable-audience-closeout/charter-open-question-disposition/**
.bklg/docs-that-teach/initiative.md
.kb/_intake/**
```

**In this PR**

- `.../charter-open-question-disposition/_disposition.md` — the disposition record: the eleven-row
  table, the evaluator decision with its reasoning, the Q10 re-verification results, the M3 handoff
  block, and the routed-gap prose for anything that wants a decision atom.
- One appended `##` section at the end of `.bklg/docs-that-teach/initiative.md`.
- Zero or more staged deferral documents under `.kb/_intake/`, one per question that could not be
  answered, in `.kb/open-questions/README.md`'s shape.
- This story's own `_ledger.md` and stage artifacts (`require_ledger: true`,
  `.redkiln/config.yaml:67`).

**Explicitly not in this PR**

- **Any file under `.kb/` that is not in `.kb/_intake/`.** No atom, no map edit. `.kb/product/`,
  `.kb/open-questions/`, `.kb/maps/domain-map.md` and `.kb/maps/open-questions-index.md` are written by
  the ingest wave in M3 (AC-A01; `../_decomposition.md`, `## Architecture brief`, §1 diff-surface table).
- **`.kb/decisions/`, in any form** — no addition, no modification (AC-A09).
- **The persona and journey atom text.** `staged-audience-payload` transcribes the evaluator decision;
  this story does not draft atom bodies (`../_storymap.md`, Coverage, AC-003 seam).
- **The reconciliation record and the DT-1…DT-10 audit** — slice-mates, their own stories.
- **The merge-forward itself** — `merge-forward-baseline` (M1) produces the sha this story names.
- **Answering a question the initiative did not actually answer.** A question without a deciding
  artefact is staged, not argued (`initiative.md:165-166`).
- **`crates/`, `spec/`, `xtask/`, `standards/`, `docs/`, `examples/`** — read-only for this project.

The implementer **may** touch the wiring file named in the Integration contract —
`.bklg/docs-that-teach/initiative.md` — to mount the appended disposition section. That is the mount, not
scope drift; it is append-only, it is body prose, and the frontmatter stays the CLI's. It is the one path
in the boundary above that sits outside this project's folder, and it is there deliberately: AC-003's
"the initiative charter's corresponding open question is marked answered" cannot be discharged anywhere
else.

**Merge DoD**: the charter carries an appended disposition section whose every row resolves to a real
artefact or a staged deferral, the evaluator is decided once with reasoning agreeing with the
reconciliation record, `git diff` on `initiative.md` shows no deletion line, and `cargo xtask affected
--base main` is green.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **Eleven rows, no silence** | `_disposition.md` carries one row per bullet in the charter's open-questions section, keyed **Q1…Q11** by charter line anchor: Q1 `:517-520` chapters · Q2 `:521-525` where a documentation discipline lives · Q3 `:526-527` RUNBOOK phase · Q4 `:528-530` documentation MUSTs by clause id · Q5 `:531-533` how the staged persona work reconciles · Q6 `:534-535` which opportunities are in the first pass · Q7 `:536-537` whether the adapter path waits · Q8 `:538-540` the evaluator · Q9 `:541-543` the "no replacement noun" vocabulary · Q10 `:544-552` cross-branch verification · Q11 `:553-555` discovery coverage. A missing row is a defect; so is a merged row covering two bullets | `.bklg/docs-that-teach/initiative.md:513-555`; `project.md` AC-017, DR-13 |
| **A fixed state vocabulary, written as words** | Exactly one of `answered on the record` · `withdrawn` · `open_question atom` · `not a question` per row. Q11 is the only bullet expected to take `not a question`; Q10 is a status bullet that still carries a live obligation and therefore takes a real state. No glyph, tick, colour, strikethrough or empty cell anywhere in the record | `../_decomposition.md` `## UX brief`, "Accessibility floor" and AC-UX-09; `.kb/maps/open-questions-index.md:136-143` |
| **`answered` and `withdrawn` cite the artefact that decided it** | Each such row names the artefact **by repo-relative path** — and by `file:line` where the artefact is long — that actually made the decision, plus one sentence of what it decided. The record is never its own authority: a row citing only `_disposition.md` is mis-stated and must be re-disposed as `open_question atom` | `initiative.md:165-166` and `:515`; `../_decomposition.md` `## Testing brief`, AC-017 row (Tier 2 primary) |
| **The evaluator is decided once, with reasoning** | One statement, in `_disposition.md`, choosing `a persona in its own right` **or** `an earlier stage of the application author's journey`, with the reasoning written out, agreeing with `reconciliation-ledger`'s evaluator pair row. `rg` over this PR's diff finds no second, contradicting statement. Downstream, `staged-audience-payload` transcribes it; this story does not draft the atom | `.bklg/docs-that-teach/_decomposition.md:134-139`; `project.md` AC-003, DR-6; `../_storymap.md` Coverage, AC-003 seam |
| **Deferral is staged, never hand-authored** | Every `open_question atom` row has a matching document under `.kb/_intake/`, shaped *what is true today / what is not decided / what forces it* with ordered sub-questions and a grounded path behind every claim. No `.kb/open-questions/*.md` file appears in this story's diff | `.kb/open-questions/README.md`; `.kb/_intake/README.md:7-11`; AC-A01 |
| **The M3 handoff is explicit, not implied** | For each deferred question the record states: the staged file name, the target `##` section in `.kb/maps/open-questions-index.md` (a **new** section, since documentation has none — the index permits this at `:137-139`), and the bullet text in index shape — status word first, then the link, then the id, then one sentence. `audience-ingest-wave` narrows its invocation to include these files and to exclude `.kb/_intake/README.md` (AC-A02) | `.kb/maps/open-questions-index.md:136-143`; `../_decomposition.md` `## Architecture brief`, §1 writer table and mount point 2; AC-UX-07 |
| **The charter is marked answered by appending** | One `##` section at **end of** `.bklg/docs-that-teach/initiative.md`, one row per Q, each with its state word and where the answer lives. The bullets at `:517-555` are not rewritten, not annotated in place, not deleted. `git diff` on that file shows **no `-` line** | IQ-2 and IQ-3 (`../_decomposition.md` `## UX brief`); `.kb/maps/open-questions-index.md:11-13`; AC-UX-06 as the pattern |
| **Q10's three claims are re-verified on the merged tree** | `examples/outside-projection-adapter/`, `references/seeds/measured-not-claimed.md`, and the asserted `publication-and-positioning` `_design.md` / HS-S0131 stage are each checked on the **merged** tree and recorded present or absent, with the merged sha named beside the result. None resolves in this worktree today, which is why the check is post-merge or worthless | `initiative.md:544-552`; `../_grounding.md:120` "Reconciliation counterpart does not exist in this tree"; AC-A06 |
| **A gap that wants a decision is routed, not decided** | If a disposition concludes a question warrants an ADR, the record writes it as a routed gap in prose and names the route. The diff adds nothing to and modifies nothing in `.kb/decisions/` — structurally caught by `redkiln validate --kb`'s accepted-decision immutability check against `HEAD` | AC-A09; `.kb/governance/rewrite-the-referent-never-the-reasoning.md`; `project.md` "Out of scope" |
| **Rows are self-contained** | Every row names its own Q id, charter anchor, state word and evidence; no row depends on the row above to be understood, and no state is carried by ordering or by absence | AC-UX-10, IQ-8 (`../_decomposition.md` `## UX brief`) |
| **Verification tier** | Tier 2 — content review against IQ-1…IQ-8 and AC-UX-01…AC-UX-12, with Tier 1 as a secondary reader only (`rg` for a contradicting evaluator statement; `git diff` for deletion lines; `test -f` over cited paths). Tier 1 is never cited alone for AC-017 or AC-003 | `../_decomposition.md` `## Testing brief`, "AC-### → tier mapping" and AC-TB-01, AC-TB-06 |

## Data and migrations

**N/A — no schema, no store, no migration.** This story compiles no Rust, opens no database, and adds no
crate; `_design.md` records `hasSurface: false` and no public item.

Two schema-adjacent obligations exist and are named so they are not mistaken for absent:

- **`KbFrontmatter` does not apply to anything this story writes.** Staged material in `.kb/_intake/` is
  explicitly *not* an atom and is not held to the schema (`.kb/_intake/README.md:7-11`), and
  `redkiln validate --kb` skips every `_`-prefixed directory (`:21-27`). The atom that will be held to
  the schema is written by `/redkiln:kb-ingest` in M3, not here — which is exactly why a green
  `validate --kb` on this story's tree is **not** evidence about the staged documents' quality.
- **`.bklg/` item frontmatter is the CLI's.** The append to `initiative.md` is body prose beneath the
  closing `---`; `id`, `stage`, `status`, `updated` and `links` are untouched, and a `PreToolUse` hook
  denies the edit if attempted (CLAUDE.md, "Where the work lives"). `links.kb` on HS-P0025 is written by
  `redkiln record-links --atom` in M3, after ingest — never by hand and never here.

**Reversibility.** Everything this story lands is append-only text on the initiative branch: the record,
the charter section, the staged documents. Reverting the commit restores the tree exactly. A disposition
later found wrong is corrected the corpus's way — a new record superseding this one, with this one left
standing as the account of what was known on the day — never by editing this record after the fact
(`.kb/governance/rewrite-the-referent-never-the-reasoning.md`; IQ-4).

## Acceptance criteria

Eight criteria. Each is framed from the reader the UX brief names — **U2**, the closeout reviewer who
must be able to disagree with exactly one adjudication without reading the discovery corpus, and **U1**,
the next initiative's charter author who needs the evaluator decision to know what audience they inherit
(`../_decomposition.md`, `## UX brief`, "Who this is actually for"). No criterion is a bare capability:
each crosses from the charter bullet, through the record, to the reader acting on it.

Verification is **Tier 2 — content review** as primary throughout, because the testing brief makes Tier 2
primary for both project criteria this story traces to and states plainly that no automated reader of
prose exists (`../_decomposition.md`, `## Testing brief`, "AC-### → tier mapping", AC-017 and AC-003
rows). The Tier-2 checklist items are `T2-01 … T2-08`, defined in `## Tests and CI (merge gate)` below;
the Tier-1 commands beside them are secondary readers and are never cited alone.

| id | criterion | verification |
| --- | --- | --- |
| **AC-001** | GIVEN U2 opens `_disposition.md` having never read the discovery corpus, WHEN they check it against the charter's `## Open questions for the planning team`, THEN they find exactly one row per bullet — Q1 through Q11, keyed by charter line anchor — with no bullet folded into a neighbouring row, no bullet omitted, and no summary sentence standing in for a row, so a reviewer can walk `initiative.md:517-555` top to bottom and land on a row every time | **T2-01** (primary): read `initiative.md:517-555` and the record side by side, one bullet at a time; **Tier 1**: `rg -c "^\| Q[0-9]" .bklg/docs-that-teach/durable-audience-closeout/charter-open-question-disposition/_disposition.md` returns `11`. Serves project AC-017, DR-13; AC-UX-03's ledger discipline |
| **AC-002** | GIVEN U2 reading the record one row at a time — in a diff, a quoted excerpt, or a screen reader — WHEN they read any single row, THEN its state is one of the four literal words `answered on the record`, `withdrawn`, `open_question atom`, `not a question`, and the row names its own Q id, charter anchor, state word and evidence, so that nothing is carried by an emoji, a tick, a colour word, a strikethrough, an empty cell, or the row's position | **T2-02** (primary): the accessibility-floor read — every state a word, every row self-contained; **Tier 1**: `rg -n "✅|❌|🟢|🔴|~~" ` over this story's diff returns nothing. Serves AC-UX-09, AC-UX-10, IQ-8 |
| **AC-003** | GIVEN U2 wants to disagree with exactly one adjudication and not re-litigate the initiative, WHEN they read a row whose state is `answered on the record` or `withdrawn`, THEN that row names the artefact **elsewhere in the initiative** that actually decided it by repo-relative path — by `file:line` where the artefact is long — plus one sentence of what it decided, that path resolves on the merged tree, and no such row offers `_disposition.md` itself as its authority | **T2-03** (primary): per `answered`/`withdrawn` row, open the cited artefact and confirm it decides the question; **Tier 1**: `test -f` over every cited path, and `rg` for a row whose only citation is `_disposition.md`. Serves project AC-017; `initiative.md:165-166`, `:515` |
| **AC-004** | GIVEN U1 beginning the next charter needs to know whether they inherit three personas or four, WHEN they read the evaluator disposition, THEN exactly one statement exists choosing `a persona in its own right` **or** `an earlier stage of the application author's journey`, its reasoning is written out rather than asserted, it agrees with `reconciliation-ledger`'s evaluator pair row, and no second contradicting statement exists anywhere in this PR's diff | **T2-04** (primary): read the statement against `reconciliation-ledger`'s record and `.bklg/docs-that-teach/_decomposition.md:134-139`; **Tier 1**: `rg -n "persona in its own right|earlier stage of the application author" ` over the diff returns one coherent statement. Serves project AC-003, DR-6; AC-UX-03's state vocabulary |
| **AC-005** | GIVEN a charter question that no artefact in this initiative actually decided, WHEN U2 reads its row, THEN the row states `open_question atom`, a matching document exists under `.kb/_intake/` in the *what is true today / what is not decided / what forces it* shape with a grounded path behind every claim, the question is **not** argued to a conclusion in the record instead, and where a disposition concludes a decision atom is warranted it is written as a routed gap in prose — with this PR's diff adding nothing to and modifying nothing in `.kb/decisions/` and creating no file under `.kb/open-questions/` | **T2-05** (primary): read each staged document against `.kb/open-questions/README.md`'s shape; **Tier 1**: `git diff --name-only` shows no path under `.kb/decisions/` or `.kb/open-questions/`, and `redkiln validate --kb` exits zero (its accepted-decision immutability check against `HEAD`). Serves project AC-017; AC-A01, AC-A09 |
| **AC-006** | GIVEN the M3 implementer who will run `audience-ingest-wave` and has only this record, WHEN they open its handoff block, THEN for each deferred question they get the staged file name, the target `##` section in `.kb/maps/open-questions-index.md` — a **new** section, since documentation has none today and the index permits starting one only in that case — and the bullet text in the index's own shape, status word first, then the link, then the atom id, then one sentence, plus the instruction to narrow the wave's invocation so `.kb/_intake/README.md` is excluded | **T2-06** (primary): read the handoff block against `.kb/maps/open-questions-index.md:136-143` and confirm the section it targets does not already exist; **Tier 1**: `rg -n "^## " .kb/maps/open-questions-index.md` shows the two existing sections are untouched. Serves project AC-017; AC-UX-07, AC-A02 |
| **AC-007** | GIVEN U2 meets these questions in the charter and not in this project's folder, WHEN they open `.bklg/docs-that-teach/initiative.md`, THEN one `##` section appended at **end of file** carries one row per Q with its state word and where the answer lives, so the charter itself marks each question answered; `git diff` on that file contains no `-` line; and every `file:line` citation of the charter already made from `project.md`, both `_decomposition.md` files and the briefs still resolves to the same text | **T2-07** (primary): read the appended section as a reader arriving at the charter cold — one hop to the record; **Tier 1**: `git diff .bklg/docs-that-teach/initiative.md` contains no line beginning `-` outside the diff header, and spot-checks of `initiative.md:165-166`, `:275-288`, `:416-420`, `:524-525`, `:538-540` still land on the cited text. Serves project AC-017 and AC-003's "charter's corresponding open question is marked answered"; IQ-2, IQ-3, AC-UX-06 |
| **AC-008** | GIVEN Q10 is a standing instruction to re-verify three specific claims rather than a question to answer, WHEN U2 reads its row, THEN each of `examples/outside-projection-adapter/`, `references/seeds/measured-not-claimed.md` and `publication-and-positioning`'s asserted `_design.md` / HS-S0131 stage is recorded **present** or **absent** as checked on the merged tree, with the merged commit sha named beside the result, and no Q10 result is recorded against this worktree's pre-merge copy | **T2-08** (primary): read the three results against the sha named, and confirm the sha is the one `merge-forward-baseline` recorded; **Tier 1**: `git cat-file -e <sha>` resolves, and `git ls-tree <sha> -- examples/outside-projection-adapter references/seeds/measured-not-claimed.md` agrees with the recorded present/absent. Serves project AC-017; AC-A06, AC-UX-12 |

**Traceability.** Project **AC-017** (*every charter open question answered on the record or an
`open_question` atom, with an index bullet, none silently dropped*) is carried by AC-001, AC-002, AC-003,
AC-005, AC-006, AC-007 and AC-008 — the index bullet itself is handed to M3 as text by AC-006, because
the atom it points at does not exist until `audience-ingest-wave` runs and a bullet pointing at nothing
is worse than no bullet. Project **AC-003** (*the evaluator decided in one place with reasoning, and the
charter's open question marked answered*) is carried by AC-004 for the decision and AC-007 for the
charter marking; the atom-text half of AC-003 is `staged-audience-payload`'s, per the story map's
ownership seam.

## Interaction quality

This story renders no screen — `../_design.md` records `hasSurface: false` and carries no `## Items`
block — so the composition family below binds against the corpus's **real** primitive layer rather than
a CSS one: the atom template, the frontmatter vocabulary, the two map formats and the status-word
vocabulary, which the UX brief argues are a design system with a validator behind them and not an
analogy stretched thin (`../_decomposition.md`, `## UX brief`, "Design-system primitives"). Every
invariant below is carried by a row in the table above; none of them lives only as a bullet here.

**State invariants**

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **In place, not a context jump** (IQ-1) — a row answers *which question, what state, decided where* on first load; a link carries provenance, never the answer itself | AC-003 | T2-03: open the record alone; a row that requires the discovery corpus to be understood fails |
| **Non-occlusion — a summary must not hide what it summarises** (IQ-2) | AC-001 | T2-01: eleven bullets, eleven rows; a summary sentence may precede the table and may never replace a row |
| **Supersession annotates, it does not delete** (IQ-2) | AC-007 | T2-07 / Tier 1: the charter's bullets stay legible as questions; the diff carries no `-` line |
| **Preserved position** (IQ-3) — nothing moves an anchor someone else already cited | AC-007 | Tier 1: the append is at EOF, and the five spot-checked `file:line` citations still resolve |
| **Reversibility** (IQ-4) — the correction route is *supersede*, never *edit*, and the whole change reverts by reverting one commit | AC-005 | T2-05: the record states its own correction route; the routed-gap prose is the only outlet for "this wants a decision" |
| **Preserved selection** (IQ-6) — one decision, not two audiences | AC-004 | T2-04: the evaluator statement agrees with `reconciliation-ledger`; a contradiction is review-blocking |
| **Reachable without prior knowledge** (IQ-5) — a reader who has never heard of this initiative reaches the record from where they met the questions | AC-007 | T2-07: `initiative.md` → the appended section → the record, one hop, with link text naming the destination |
| **Every state legible at the moment of reading** (IQ-8) — no state by presence, absence, ordering or omission | AC-002 | T2-02: read any single row out of context and lose nothing |

**Composition invariants** — from the primitives the UX brief fixes, and from its named anti-patterns.

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **Presentation exists at all** — the record is a composed artefact in the corpus's own shapes (one `#`, sections at `##`, no skipped level, a table with named columns), not a bag of bullets | AC-002 | T2-02: heading structure and the fixed column set are read as composition, not as markup |
| **Placement** — the disposition record lives under this story's folder and the *navigation* for it lives in the charter; neither is duplicated in the other | AC-007 | T2-07: the charter section points at the record; it does not restate the evidence column |
| **Transience** — persistent chrome is the charter's appended section (always visible to a charter reader); the record is opened on demand; the M3 handoff block is revealed inside the record and is not hoisted into the charter | AC-006, AC-007 | T2-06, T2-07: the handoff block is for the ingest implementer, not for U2 at the charter |
| **Density budget, with its numbers** — eleven rows, one per charter bullet; five cells per row (Q id, charter anchor, state word, evidence, sha where the evidence needed one); **one** sentence of what the cited artefact decided; index bullet text in exactly four parts — status word, link, id, one sentence | AC-001, AC-002, AC-006 | T2-01, T2-02, T2-06: a row that grows into a paragraph, or an index bullet that repeats the atom's grounding, is over budget |
| **Hierarchy** — the evaluator decision is stated as prose above the table as well as inside it, because a verdict that exists only as a table cell is lost in a quote or a diff | AC-004 | T2-04: the plain-text-equivalence line of the accessibility floor |
| **Link purpose from link text alone** (WCAG 2.4.4) — every link names the atom, path or section it leads to; never "here", "this", "see above" | AC-003 | T2-03: read the link text with the surrounding sentence removed |
| **Anti-pattern: status by glyph, colour, strikethrough or empty cell** | AC-002 | Tier 1 `rg` for glyphs; T2-02 for the empty-cell case, which no `rg` can catch |
| **Anti-pattern: a bespoke table format or an invented frontmatter key** | AC-002, AC-006 | T2-02, T2-06: the index bullet is the index's shape; the staged document is the open-question README's shape; nothing new is coined |
| **Anti-pattern: a new `##` in `open-questions-index.md` when the domain already has a section** | AC-006 | T2-06: the exemption applies *only* because documentation has no section today; the two existing sections stay untouched |
| **Anti-pattern: an atom hand-written into `.kb/`, whatever its shape** | AC-005 | Tier 1: no path under `.kb/open-questions/` or `.kb/decisions/` in the diff |

## Error conditions

| id | condition | required behaviour |
| --- | --- | --- |
| **EC-001** | A charter bullet has no artefact in the initiative that decided it | The row takes `open_question atom` and a staged document is written. Arguing the question to a conclusion inside the record is the failure mode the charter forbids at `initiative.md:165-166`, and it is what T2-03 exists to reject |
| **EC-002** | `reconciliation-ledger` has not landed, or its evaluator pair row is absent, or it contradicts the decision this story would make | Stop. The evaluator decision is not authored against an absent or disagreeing sibling record; `depends_on` exists for exactly this. A contradiction discovered late is resolved by amending **both** records in one change, never by leaving the older one standing |
| **EC-003** | The merge-forward has not happened, or `merge-forward-baseline` recorded no sha | Q10's row cannot be filled. Record the blocker and halt rather than write "verified" against this worktree — AC-A06 is unconditional and a Q10 result without a sha beside it has verified nothing |
| **EC-004** | A path cited as evidence for an `answered` or `withdrawn` row does not resolve on the merged tree | The row is mis-stated. Either correct the path, or re-dispose the row as `open_question atom`. A dangling citation is worse than an honest deferral, because it reads as settled |
| **EC-005** | The charter append lands anywhere but end of file, or `git diff` on `initiative.md` shows a `-` line | Revert the edit and re-append at EOF. An in-place annotation shifts every line below it and breaks the `file:line` citations named in AC-007 |
| **EC-006** | A disposition concludes that a question warrants a decision atom | Write it as a routed gap in prose, naming the route, and stop. An ADR authored as a closeout by-product is the practice the corpus was built to refuse (`project.md`, "Out of scope"; AC-A09) |
| **EC-007** | A staged file name under `.kb/_intake/` collides with one `staged-audience-payload` will write | Rename with a distinguishing prefix before staging. Both stories feed the same single wave; a collision silently overwrites another story's staged source and loses its audit trail |
| **EC-008** | `redkiln validate --kb` fails on this story's tree | Do not work around it. Its accepted-decision immutability check against `HEAD` is the structural tripwire for AC-A09 (T5, `../_decomposition.md`, `## Architecture brief`); a failure here means `.kb/decisions/` was touched |

## Non-functional

| id | requirement | how it is held |
| --- | --- | --- |
| **NF-001** | **Append-only against shared files.** The diff on `.bklg/docs-that-teach/initiative.md` contains zero deletion lines, and no pre-existing section of it is reflowed | `git diff` on that path; AC-007 |
| **NF-002** | **Boundary containment.** No file outside the three paths in `## PR boundary` is touched — in particular nothing under `crates/`, `spec/`, `xtask/`, `standards/`, `docs/`, `examples/`, and nothing under `.kb/` that is not in `.kb/_intake/` | `git diff --name-only` read against the boundary block; AC-005 |
| **NF-003** | **Self-containment.** The record is usable from one open file: a reviewer needs the charter and the record, and no third artefact, to check any row's *state*; the artefact a row cites is opened only to check that row's *evidence* | T2-03; IQ-1 |
| **NF-004** | **Story-grain gate stays green.** `cargo xtask affected --base main` is green. This story changes no Rust, so the expectation is a fast, empty-affected run — a failure here means the boundary was breached | `.redkiln/config.yaml`, the story grain; NF-002 |
| **NF-005** | **Durability of the record.** Every claim that a later reader must be able to re-run names the tree it was taken against by sha, so the record stays checkable after the branch is merged and the worktree is gone | AC-008; AC-UX-12 |
| **NF-006** | **No advisory drift.** `redkiln doctor` still reports exactly the six permanent `template-drift` advisories CLAUDE.md names. A seventh or a missing one is flagged, never silenced | `redkiln validate --kb && redkiln doctor`; CLAUDE.md, "Where the work lives" |

## Implementation notes (non-prescriptive)

Deliberately not fixed here — record whichever way they go rather than inheriting a guess.

- **Where the record lives is fixed; how it is shaped inside is not.** `_disposition.md` under this
  story's folder is the mount the Integration contract names. Whether the eleven rows are one table or
  a table plus per-question prose beneath it is the implementer's, subject to the density budget above
  and to the plain-text-equivalence rule for the evaluator verdict.
- **How many staged deferral documents.** One per deferred question is the obvious shape and keeps the
  index bullets one-to-one with the atoms M3 will land. A single document covering two genuinely
  entangled questions is defensible if the record says why; the architecture brief leaves the count to
  the implementer (`../_decomposition.md`, `## Architecture brief`, §6).
- **What the new `##` section in `open-questions-index.md` is called.** The handoff block proposes the
  name; `audience-ingest-wave` lands it. That it is a *new* section is fixed by the index's own rule and
  by documentation having none today; the wording is a judgement call.
- **The order the rows are read in.** Q1…Q11 by charter line anchor is the safe default because it lets
  a reviewer walk the charter and the record in lockstep. Grouping by state word would read better and
  is the wrong trade here: AC-001's falsifier is positional.
- **The evaluator decision itself is genuinely open** and is this story's to make. Both answers are
  defensible; what is not defensible is making it twice, making it silently, or deferring it again —
  the decomposition already deferred it once *to this moment* (`.bklg/docs-that-teach/_decomposition.md:134-139`).
  Write the reasoning as an argument a reader can disagree with, not as an assertion.
- **Sequencing inside the slice.** `reconciliation-ledger` first (its evaluator pair row is an input),
  then this story, with `design-tension-audit` in parallel with either. All three name the same M1 sha
  (`../_storymap.md`, "Merge order", step 2).

## Tests and CI (merge gate)

The Tier-2 checklist is the UX brief's own — IQ-1 … IQ-8 and AC-UX-01 … AC-UX-12 — not a second list
invented here (AC-TB-06). `T2-01 … T2-08` below are the eight reads of that checklist this story's ACs
bind to, so a reviewer has one place open rather than two.

| tier | command / path | proves |
| --- | --- | --- |
| **2 — content review (primary)** | **T2-01** — read `.bklg/docs-that-teach/initiative.md:517-555` against `.../charter-open-question-disposition/_disposition.md`, one bullet at a time | Eleven bullets, eleven rows, none folded or omitted, no summary standing in for a row (AC-001; AC-UX-03) |
| **2 — content review (primary)** | **T2-02** — the accessibility-floor read of the record and the charter append | Every state is a literal word; no glyph, colour, strikethrough, empty cell or positional state; every row self-contained (AC-002; AC-UX-09, AC-UX-10) |
| **2 — content review (primary)** | **T2-03** — per `answered`/`withdrawn` row, open the cited artefact | The cited artefact actually decided the question, in one sentence, and the record is never its own authority (AC-003) |
| **2 — content review (primary)** | **T2-04** — read the evaluator statement against `.bklg/docs-that-teach/durable-audience-closeout/reconciliation-ledger/` and `.bklg/docs-that-teach/_decomposition.md:134-139` | One decision, with reasoning, agreeing with the sibling record (AC-004; project AC-003, DR-6) |
| **2 — content review (primary)** | **T2-05** — read each staged `.kb/_intake/` document against `.kb/open-questions/README.md` | The deferral is staged in the corpus's shape with grounded claims, and nothing was argued to a conclusion instead (AC-005) |
| **2 — content review (primary)** | **T2-06** — read the handoff block against `.kb/maps/open-questions-index.md:136-143` | File name, target section and bullet text are all present and in the index's shape; the narrowing instruction excludes the intake README (AC-006; AC-UX-07, AC-A02) |
| **2 — content review (primary)** | **T2-07** — read the appended charter section as a reader arriving cold | The charter marks each question answered and reaches the record in one hop (AC-007; IQ-2, IQ-3, IQ-5) |
| **2 — content review (primary)** | **T2-08** — read Q10's three results against the named sha | Each path recorded present or absent on the *merged* tree, sha beside the result (AC-008; AC-A06) |
| **1 — static** | `git diff .bklg/docs-that-teach/initiative.md` | No `-` line: the charter was appended to, not edited (AC-007; NF-001) |
| **1 — static** | `git diff --name-only` read against `## PR boundary` | No path under `.kb/decisions/`, `.kb/open-questions/`, `crates/`, `spec/`, `xtask/`, `standards/`, `docs/`, `examples/` (AC-005; NF-002) |
| **1 — static** | `test -f` over every path cited as evidence in the record | Every citation resolves on the merged tree (AC-003; EC-004) |
| **1 — static** | `rg` for a second, contradicting evaluator statement across this PR's diff | The decision exists once (AC-004) |
| **1 — static** | `rg` for status glyphs across this PR's diff | No emoji, tick or strikethrough carries a state (AC-002) |
| **1 — static** | `git cat-file -e <sha>` and `git ls-tree <sha> -- <the three Q10 paths>` | The named sha is real and the recorded present/absent agrees with the tree (AC-008) |
| **1 — static** | `redkiln validate --kb` | Exits zero, and its accepted-decision immutability check against `HEAD` confirms `.kb/decisions/` is untouched (AC-005; EC-008). **Not** evidence about the staged documents — it cannot see `_`-prefixed directories (`.kb/_intake/README.md:21-27`) |
| **1 — static** | `redkiln doctor` | Exactly six `template-drift` advisories, no seventh, none missing (NF-006) |
| **story grain** | `cargo xtask affected --base main` | Green. No Rust changed, so a failure means the PR boundary was breached (NF-004) |
| **4 — e2e (not this story's)** | `cargo xtask ci` | Deferred to `terminal-gate-run`, which runs it last on the tree carrying every artefact. A gate run taken here would prove the gate, not the deliverable (`../_decomposition.md`, `## Architecture brief`, ordering 4) |

## Risks and coupling (PR-scoped)

| risk | likelihood / impact | how this PR contains it |
| --- | --- | --- |
| **The record becomes the eleven-question think-piece the charter forbids** | Medium / High | AC-003 makes citation the only admissible form of "answered": a row whose only authority is this record must be re-disposed as `open_question atom`. This is the single most likely wrong implementation and the review's first check |
| **The evaluator decision is made here and contradicted in `reconciliation-ledger`** | Medium / High | `depends_on: reconciliation-ledger` sequences them, EC-002 halts on disagreement, and AC-004 requires agreement as an observable, not as an intention. Both records name the same merged sha |
| **The charter append shifts a cited `file:line`** | Low / High | Append at EOF only; AC-007's Tier-1 check is a diff with no `-` line, plus spot-checks of the five citation sites the briefs already made |
| **A deferral is written straight into `.kb/open-questions/` because it "obviously belongs there"** | Medium / High | AC-005 forbids it and Tier 1 catches it in `git diff --name-only`. Staged material is not an atom; `/redkiln:kb-ingest` is the only writer, and hand-writing atoms is the practice `0269720` reverted |
| **Q10 is marked verified against this worktree** | Medium / High | AC-008 requires the sha beside each result and EC-003 halts if there is none. None of Q10's three paths resolves in this tree today, so a "verified" row without a sha is provably false |
| **AC-017's literal index bullet is read as landable here** | Medium / Medium | It is not: the atom does not exist until M3. AC-006 converts the obligation into a handoff of exact text, and the Traceability note under the AC table says so in one place so a reviewer does not read AC-017 as unmet |
| **A closeout by-product ADR** | Low / High | AC-005 routes it as prose and `redkiln validate --kb` fails structurally on any touch of `.kb/decisions/` (EC-008) |
| **Coupling to `staged-audience-payload` (M3)** | — | One-directional and by text: it *transcribes* this story's evaluator decision into atom bodies and does not re-open it (`../_storymap.md`, Coverage, AC-003 seam). This story drafts no atom body |
| **Coupling to `audience-ingest-wave` (M3)** | — | By file: the staged documents and the handoff block are its inputs, and it must narrow its invocation to exclude `.kb/_intake/README.md` (AC-A02, T4) |

## Dependencies

**Blocks on** — `reconciliation-ledger`. Two reasons, and only the second is about sequencing: its
evaluator pair adjudication is an *input* to AC-004's decision, and both records must name the same M1
merged sha. Transitively this story also stands downstream of `merge-forward-baseline`, which produces
that sha — AC-008 and EC-003 are unsatisfiable without it.

**Unlocks** — `staged-audience-payload`, which transcribes the evaluator decision into the persona and
journey atom text (`../_storymap.md`, "Merge order", step 2; the story item records `blocks: HS-S0177`).
`audience-ingest-wave` consumes this story's staged deferral documents and its handoff block in the same
M3 wave.

**Parallel, not dependent** — `design-tension-audit`, the third `closeout-adjudication` story. It reads
the same merged tree and is read by the same reviewer in the same pass, but neither story is an input to
the other.

## Anchors (progressive disclosure)

Link, do not paste. Each row says why the artefact is load-bearing and the moment to open it; nothing
below is required reading before the first line of the record is written except the first two rows.

| anchor | why it is load-bearing | when to open it | serves |
| --- | --- | --- | --- |
| `.bklg/docs-that-teach/initiative.md` | The eleven bullets at `:513-555` **are** the row set, and `:672-673` is the exit criterion this story discharges. It is also the mount point — the appended `##` section lands at its end | First, before drafting any row; again at the end, to append the section | AC-001, AC-007 |
| `.bklg/docs-that-teach/_decomposition.md` | `:134-139` is the record of the evaluator question being *deferred to promotion* rather than answered — the reason this story is the moment it gets decided, and the reason a third deferral is not available | Before writing the evaluator decision, not before drafting the table | AC-004 |
| `.bklg/docs-that-teach/durable-audience-closeout/reconciliation-ledger/spec.md` | The blocking sibling's spec fixes the shape of its evaluator pair row; this story's decision must agree with what lands there, and disagreement is review-blocking | Immediately before AC-004's statement is written, and again at review | AC-004 |
| `.bklg/docs-that-teach/durable-audience-closeout/_decomposition.md` | Carries the Tier-2 checklist in full — IQ-1 … IQ-8 and AC-UX-01 … AC-UX-12 — plus the tier map that makes Tier 2 primary for both traced project ACs. The `## UX brief` "Design-system primitives" table is the composition vocabulary this record must compose from | Open at the review pass; skim the primitives table before choosing the record's shape | AC-002 |
| `.bklg/docs-that-teach/durable-audience-closeout/project.md` | AC-017 and AC-003 verbatim, and DR-13 / DR-6 behind them. The spec paraphrases; the project item is the bar | Before writing the ledger rows, to confirm the traced criteria are stated as the project states them | AC-003 |
| `.bklg/docs-that-teach/durable-audience-closeout/_storymap.md` | The AC-003 ownership seam — this story *decides*, `staged-audience-payload` *transcribes* — and the M2 merge order. Prevents this story drifting into drafting atom bodies | Before writing anything that begins to look like atom text | AC-004 |
| `.bklg/docs-that-teach/durable-audience-closeout/_grounding.md` | Records what does and does not resolve in this tree, including the counterpart's absence. Q10's three paths were checked here and none resolved — which is the whole reason Q10's re-check must be post-merge | Before filling Q10's row, to know what "absent" already meant pre-merge | AC-008 |
| `.kb/open-questions/README.md` | Fixes the staged deferral document's shape — what is true today / what is not decided / what forces it — with grounded `source_paths`. Writing a deferral in any other shape makes work for the ingest wave | Only when a row resolves to `open_question atom`, immediately before staging it | AC-005 |
| `.kb/_intake/README.md` | States that staged material is **not** an atom and is not held to `KbFrontmatter` (`:7-11`), that `validate --kb` cannot see the directory (`:21-27`), and that the default glob is `.kb/_intake/*.md` (`:5-7`) — the glob risk the handoff block must narrow | Before writing the first file into `.kb/_intake/`, and again when writing the handoff block | AC-005, AC-006 |
| `.kb/maps/open-questions-index.md` | `:136-143` is the "Adding an entry" rule — status word first, then link, then id, then one sentence, and a new `##` section only when the domain has none. `:11-13` is the corpus precedent that a withdrawn question stays listed rather than removed, which is IQ-2's authority | When writing the handoff block's bullet text; the `:11-13` half when justifying the append-only charter annotation | AC-006 |
| `.kb/governance/rewrite-the-referent-never-the-reasoning.md` | The accepted governance atom behind "correct by superseding, never by editing". It is what makes the routed-gap discipline load-bearing rather than stylistic, and it cuts both ways — no editing a decision, and no inventing one as closeout exhaust | When a disposition concludes a question wants a decision atom, and when stating the record's own correction route | AC-005 |

## Clarifications resolved during spec

- **Eight AC ids, exactly as the front half enumerated.** AC-001 … AC-008; none added, none dropped. The
  eleven behaviour rows in `## Behavior and interfaces` map onto eight criteria because three pairs
  collapse: the state vocabulary and the self-contained-row rule are one reader-facing invariant
  (AC-002); deferral-is-staged and gaps-are-routed are one discipline with one falsifier in
  `git diff --name-only` (AC-005); and the verification-tier row is not a criterion but the verification
  column of every other row.
- **AC-017's index bullet is discharged as a handoff, not as a landed bullet.** The project criterion
  says "with a bullet appended to `.kb/maps/open-questions-index.md`". That bullet cannot land in this
  PR: `/redkiln:kb-ingest` is the corpus's only writer of `.kb/` (AC-A01) and the atom it would link to
  does not exist until `audience-ingest-wave` runs in M3. AC-006 therefore requires the exact bullet
  text, its target section and the staged file name — and the story map's coverage table already treats
  AC-017 as landing across the M2/M3 boundary. A bullet linking to a non-existent atom would satisfy the
  letter of AC-017 and break IQ-5 outright.
- **Q10 is a status bullet that still carries a live obligation, so it gets a real state.** It reads as
  a note rather than a question, and the tempting disposition is `not a question`. It is not: it ends
  with a standing instruction to re-verify three paths against the merged tree. AC-008 makes that
  instruction the row's content. Q11, by contrast, genuinely is a status statement and is the one bullet
  expected to take `not a question`.
- **The evaluator decision itself is left to implementation, deliberately.** This spec fixes that it is
  made once, here, with reasoning, agreeing with `reconciliation-ledger` — and does not pre-empt which
  way it goes. Deciding it in the spec would be settling it in a planning artifact that no reviewer
  reads as the record, which is the same defect as settling it in passing.
- **No conformance rule and no specification clause.** Confirmed rather than assumed: this story
  compiles no Rust and touches nothing under `crates/` or `spec/`, so there is no rule id in the
  testkit that could observe it and inventing one would be a rule no adapter can fail. Stated in the
  Integration contract so that a reviewer does not read the absence as an omission.
- **`_design.md` binds this story by declaring no surface.** It records `hasSurface: false` with no
  `## Items` block, approved with no conditions. The composition family in `## Interaction quality` is
  therefore bound to the corpus's primitive layer — the atom template, the two map formats, the
  status-word vocabulary — which is what the UX brief argues is this repository's real design system.
  The section is not skipped, because an unstyled record satisfies every path-resolution check perfectly
  and still fails the reader.
