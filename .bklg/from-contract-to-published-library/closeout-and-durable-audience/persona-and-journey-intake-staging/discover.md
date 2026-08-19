---
item: HS-S0131
stage: discover
created: 2026-08-12T13:03:56.999Z
updated: 2026-08-12T13:03:56.999Z
template_sig: 86ce4036
rendered_sig: a4003ec0
---

# Discover — Four personas and their journeys staged in .kb/_intake/, qualification in frontmatter

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Slice one-liner: "Stage the four personas and their journeys in `.kb/_intake/` under a distinct wave id, each draft carrying the secondary-evidence sentence in `summary` and its discovery artefacts in `source_paths`, matching the four-persona set the testing brief already decided." | `_storymap.md:59` | The storymap itself asserts "four personas" as settled — but see Questions below; this is exactly the point that needs re-checking, not re-asserting. |
| AC-010 | `project.md:225-228` | "This project's `testing` brief records whether the evaluator is its own persona or a stage of the application author's journey, with the reasoning, and the set of promoted atoms matches that decision." |
| DR-10 — the evaluator question is decided first | `project.md:171-174` | The decision must be settled *before* any atom is authored — "carried unresolved from the charter and from `_decomposition.md` *Carried into the briefs*." |
| The testing brief's own DR-10 decision | `_decomposition.md:148-199` (this project's own briefs file) | **Decides: promote Persona 4 as its own persona atom, not folded into Persona 1** — four personas, four journeys. Reasoning cites `personas-and-journeys.md`'s cross-persona tensions, the intake brief's evaluator-facing framing, and DT-1's option (c). |
| The open question this turns on, verbatim | `../_discovery/distillation/personas-and-journeys.md:373-377` | "Should Persona 4 (the evaluator) be promoted as its own persona at closeout, or folded into Persona 1 as an earlier stage of the same journey? ... they may not warrant separate acceptance criteria." Filed as genuinely open for "the planning team." |
| `publication-and-positioning`'s signed-off DT-1 resolution | `.bklg/from-contract-to-published-library/publication-and-positioning/_design.md:214-222` | **Resolves the opposite way**: "the evaluator is the first fifteen minutes of the application author's journey, not a fifth persona... Consequence, and it is not this project's to execute: `closeout-and-durable-audience` (BR-16) promotes this as a **journey stage** on the application author's atom, not as a fourth persona atom." Names this project by slug as the executor of *its* resolution. |
| The sign-off's own flagged risk | `publication-and-positioning/_design.md:764-767` | The approver was explicitly told this call "is what makes DT-1's option (c) incoherent, and reversing it reopens DT-1" — i.e., the design's author already knew this decision was load-bearing and contestable, and flagged it for pushback at sign-off. |
| Persona 4's evidence quality | `../_discovery/distillation/personas-and-journeys.md:349-355` | Persona 4 is "the two least-tested against real evidence... inferred from research framing rather than from a named individual's stated experience anywhere in the source material." Weighs toward folding, not promoting, all else equal. |
| `dependsOn: []` | `_storymap.md:149-151` (merge order 4) | "Independent of slices 1–3; must precede slice 5 because the new atoms are inputs to `redkiln validate --kb`." No upstream story dependency, but see the cross-project dependency this signal ledger surfaces. |

## Questions

- **Is the evaluator its own persona, or the first fifteen minutes of the application author's journey?** This is the live tension the task brief flagged explicitly, and the signal ledger above shows it is not hypothetical: two signed-off artefacts disagree. This project's own testing brief (`_decomposition.md:150-152`, authored at this project's `briefs` stage) decided **four personas, evaluator promoted separately**. The sibling project `publication-and-positioning`, whose `_design.md` was signed off by the repository owner on 2026-08-12 at its own design gate, decided the opposite and named *this* project as the executor of that opposite decision: "`closeout-and-durable-audience` (BR-16) promotes this as a journey stage... not as a fourth persona atom" (`publication-and-positioning/_design.md:220-222`).

  **Answered, for the purpose of this story's spec: consume `publication-and-positioning`'s resolution.** Three reasons. First, it is the *later* signed-off artefact and the one that explicitly names this project's execution as its consequence — the testing brief's DR-10 reasoning never cites or rebuts it (its citations are `personas-and-journeys.md` and the intake brief, not the sibling's design), so it is not a considered disagreement, it is two projects independently answering the same open question from `personas-and-journeys.md:373-377` without either seeing the other's answer. Second, `publication-and-positioning`'s own DT-1 resolution is now load-bearing on a *published* surface — the crate split by role (ADR-0006), the disambiguation triad, the registry copy — reversing it after that project has closed would mean re-opening DT-1 downstream of publication, which is a materially larger cost than re-deriving three atoms instead of four here. Third, the testing brief's own DR-12/risk-table language already anticipated this exact possibility and named its handling: "whether HS-P0016's recorded DT-1 resolution is consistent with this decision is a **finding to route**, not something this brief re-litigates" (`_decomposition.md:181-185`; `project.md` risk table, "The evaluator decision has downstream reach"). Routing, not silent deference, is what the brief itself asked for — and "route" means record it as a finding, which this story's spec must do explicitly, not fold into ordinary staging as if uncontested.

  **Consequence for this story — UPDATED 2026-08-12 by the repository owner at the `/redkiln:plan` spec stage, superseding the paragraph above.** The conflict was escalated rather than left to this story's own deference, and it was settled as a **synthesis of both artefacts, not a win for either**: stage **three** persona atoms (Application author, Adapter author, Local-first/edge developer) plus their journeys, **and one further journey atom for the evaluation path**, linked to the Application-author persona and carrying the same secondary-evidence qualification as its siblings. Not a fourth *persona* (DR-10 overruled on that), and not a *stage buried inside* Persona 1's journey either (`publication-and-positioning`'s original consequence wording, amended). The reason DR-10 was not simply discarded: its argument 1 — that the difference is in the *mechanism* of trust-building, one-shot public evidence versus revisable contact with the code — was never rebutted by the sibling design, and a moment with its own mechanism is exactly what a **journey** atom is for. Both artefacts have been amended in place to say so (`../_decomposition.md` DR-10; `../../publication-and-positioning/_design.md` DT-1 rider), each preserving its rejected reasoning. The testing brief's DR-10 decision itself is not edited (nothing under `.bklg/` needs `.kb/`'s immutability discipline, but a signed-off brief still is not silently rewritten by a downstream story) — instead this discrepancy is recorded as a finding for `findings-disposition-register` to route, and AC-010 is satisfied by *this* story's own reasoning (recorded here and carried into spec) superseding the brief's stale one, exactly as the brief's own DR-12 language contemplated.

- **Does folding the evaluator into the application author's journey change what "matching the four-persona set the testing brief already decided" (`_storymap.md:59`) means for this story's slice description?** Yes — the storymap's one-liner is itself downstream of the now-superseded DR-10 text and needs the same correction; that is not this story's artefact to edit, but its spec should note the storymap line is stale for the same reason.
- The DoD 13 caveat does not touch this story.

## Decision

The problem this slice solves is turning discovery's *inferred* audience (`personas-and-journeys.md`, all secondary evidence, explicitly not directly observed) into staged material a human can approve at the intake gate before `/redkiln:kb-ingest` promotes it — never hand-authored directly into `.kb/product/`, which is the discipline commit `0269720` exists to remember. What the spec will cover: the concrete intake files for three personas and their journeys (not four, per the Questions resolution above), each carrying the secondary-evidence sentence in its `summary` field per DR-9, `source_paths` naming the discovery artefacts, and — new, because of the resolution above — **one further journey atom for the evaluation path**, linked to the Application-author persona, carrying the same secondary-evidence sentence, and cross-referencing both amended artefacts (`../_decomposition.md` DR-10 and `../../publication-and-positioning/_design.md`'s DT-1 rider) as the citation for why the evaluator is a journey of its own and not a fourth persona. The spec also records the DT-1/DR-10 discrepancy as a named finding, with a pointer for `findings-disposition-register` to pick up.

## The wrong implementation

Staging four persona intake files exactly as `_decomposition.md`'s DR-10 text describes, because that is what this project's own signed-off testing brief says and a story should not second-guess its own brief. This satisfies AC-010's literal first clause — "this project's testing brief records [a decision]... before authoring" — and would pass a reviewer who checks only that a decision was made and atoms match it. It fails the acceptance criterion's actual purpose and produces a `.kb/product/` layer inconsistent with a sibling's already-published, already-signed-off resolution of the *same* open question, discovered only because this is — per the whole premise of this project — the first tree where a cross-project interaction becomes visible (`project.md`, "Overview"; DR-3's rationale generalizes here even though DR-3 itself is about DoD scenarios). Promoting four personas into `.kb/product/` when `publication-and-positioning/_design.md` already tells a reader "not as a fourth persona atom" creates a durable knowledge-base artifact that contradicts a decision the next initiative will read as settled — exactly the failure mode DR-12's "routing, never absorbing" language exists to prevent, except inverted: here the risk is not fixing a code defect but *silently entrenching* an inconsistency because "the brief already said so." What catches it: cross-checking the persona count and the DT-1 resolution text against every sibling's signed-off `_design.md` before staging, not just against this project's own brief — which is precisely the check this discover pass performed and the brief's own authoring process skipped.

## Gate: Discover

Tick each box once the sections above satisfy it. `redkiln advance` reads these
against `.redkiln/templates/gates/discover.md` — where each box's rationale is
written — and will not leave `discover` until every one is ticked. Keep each box on
one line; the parser matches line by line.

- [x] The problem is framed in one paragraph.
- [x] Prior art and constraints are recorded in the signal ledger.
- [x] Open questions are either answered or explicitly deferred.
- [x] The next stage (spec) has a clear starting point.
- [x] The wrong implementation this work rejects is named.
- [x] No conformance rule added here asserts a literal position value.
- [x] Any `[FROZEN]` clause this touches is changed by a new ADR, written first.
- [x] If a rule here seems wrong, it is fixed and the reason given in the same change.
