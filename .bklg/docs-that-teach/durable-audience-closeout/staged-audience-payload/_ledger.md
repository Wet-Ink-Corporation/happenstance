---
item: HS-S0177
stage: implement
created: 2026-08-17
updated: 2026-08-17
---

# Acceptance ledger — Stage the persona and journey documents for the closeout wave

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Six of these ten rows are Tier 2 primary — a human reading one claim at a time against
`_decomposition.md`'s `## UX brief` checklist. For those rows the evidence must be a `file:line` in
the staged document where the criterion is met, not a command's exit code: a command that cannot
fail the criterion is not evidence for it (`_decomposition.md`, `## Testing brief`, AC-TB-01).
`redkiln validate --kb` is not admissible evidence for any row here — it skips `_`-prefixed
directories and cannot see `.kb/_intake/` at all (`.kb/_intake/README.md:21-27`).

```yaml
- id: AC-001
  criterion: "GIVEN U1, who has never opened this initiative's discovery corpus, WHEN they open one staged persona document under .kb/_intake/persona-<slug>.md and read nothing else, THEN that file states, as prose in itself, the persona's goal, their context, what they already do instead, and what they are afraid of — four slots, none of them discharged by a link — so U1 can write a GIVEN a <persona> <context>, WHEN they <action>, THEN <outcome> criterion from that one file."
  satisfied: false
  evidence: ""
  mount_point: ".kb/_intake/ — the input directory /redkiln:kb-ingest reads, consumed by audience-ingest-wave's narrowed invocation"
  verifying_test: "Tier 2 content review, one document at a time, against .bklg/docs-that-teach/durable-audience-closeout/_decomposition.md '## UX brief' IQ-1 and AC-UX-01; Tier 1 backstop: rg -n \"already do instead\" .kb/_intake/persona-*.md and rg -n \"afraid of\" .kb/_intake/persona-*.md hit every persona file"
- id: AC-002
  criterion: "GIVEN U1 choosing which journey a new initiative will improve, WHEN they list .kb/_intake/journey-*.md, THEN there are exactly four documents — The first fifteen minutes, Model my invariant in your words, Walk the adapter path not just the recipe, Survive the second question — and each reads as a moment-by-moment path through a task, names in its own text the persona it belongs to, and contains no screen, control, route or interaction-pattern name, so a later reader can tell whether a build improved it rather than whether a surface moved."
  satisfied: false
  evidence: ""
  mount_point: ".kb/_intake/ — journey-*.md, whose named persona is what product-layer-mounting writes the reciprocal related edge from"
  verifying_test: "Tier 2 content review against .kb/product/README.md:33-36 and _decomposition.md '## Architecture brief' §4 (Journey body); Tier 1 backstop: ls .kb/_intake/journey-*.md returns exactly four paths and rg -n \"persona\" .kb/_intake/journey-*.md hits every one; titles cross-read against .bklg/docs-that-teach/initiative.md '## Referenced personas & journeys'"
- id: AC-003
  criterion: "GIVEN U2 at the ingest approval gate asking whether the evaluator was settled or quietly left open in two distillations, WHEN they read the staged payload end to end, THEN the evaluator decision appears exactly once, in the document the decision itself makes it belong to, carrying the reasoning transcribed from charter-open-question-disposition rather than re-argued here — and the persona-document count that follows from it (three if a persona in its own right, two if an earlier stage of the application author's journey) is what is actually on disk."
  satisfied: false
  evidence: ""
  mount_point: ".kb/_intake/ — the single persona document the decision lands in; the count of persona-*.md files is the decision's observable consequence"
  verifying_test: "Tier 2 content review of the whole staged set plus .bklg/docs-that-teach/durable-audience-closeout/charter-open-question-disposition/spec.md, against project AC-003; Tier 1 backstop: rg -n -i \"evaluator\" .kb/_intake/persona-*.md .kb/_intake/journey-*.md yields exactly one passage stating the decision and no second contradicting statement; the answered question is .bklg/docs-that-teach/initiative.md:538-540"
- id: AC-004
  criterion: "GIVEN U1 deciding which claims they may frame a criterion from and which they must re-check first, WHEN they read any staged persona document, THEN that document carries an observation-status word — directly observed or still inferred — and exactly one document across the set carries directly observed, with the session date and the tree ref lifted from HS-P0024's hand-off note and a sentence saying the identification was decided at DT-9 rather than assumed; the blanket \"none has been directly observed\" appears nowhere."
  satisfied: false
  evidence: ""
  mount_point: ".kb/_intake/persona-*.md — the observation-status sentence the ingest carries into each promoted atom's body"
  verifying_test: "Tier 2 content review against _decomposition.md '## UX brief' AC-UX-04 and project AC-007; Tier 1 backstop: rg -n \"directly observed\" .kb/_intake/persona-*.md hits exactly one file, rg -n \"still inferred\" .kb/_intake/persona-*.md hits every other, rg -n \"none has been directly observed\" .kb/_intake/ returns nothing; date and tree ref matched verbatim against .bklg/docs-that-teach/comprehension-evidence/handoff-note-to-closeout/spec.md"
- id: AC-005
  criterion: "GIVEN U2 scanning the four slots of a persona document and stopping there, WHEN they reach the claim each qualification qualifies, THEN the qualification is already in front of them — the observation status beside the persona's evidence, 'rests on the seed's internal audit alone, corroborated by no research digest' beside the adapter author's 'no third-party adapter exists to read', and the HS-S0131 overlap beside the statement of where this audience came from — and no staged document collects any of the three in a trailing caveats block a scanning reader never reaches."
  satisfied: false
  evidence: ""
  mount_point: ".kb/_intake/persona-*.md — adjacency inside the body the ingest lifts into the atom's summary and body"
  verifying_test: "Tier 2 content review, the dedicated adjacency pass, against _decomposition.md '## UX brief' IQ-7 and AC-UX-04 and .kb/product/README.md:23-24 and :26-31; Tier 1 backstop: rg -n -i \"^#+ .*caveat\" .kb/_intake/ returns nothing, rg -n \"internal audit\" .kb/_intake/persona-*.md and rg -n \"HS-S0131\" .kb/_intake/ each hit"
- id: AC-006
  criterion: "GIVEN U3 asking a year from now whether this audience was reconciled or merely authored, WHEN they read the staged document that states where this audience came from, THEN it states the merge-order case reconciliation-ledger actually recorded — with the counterpart-never-ran case as the payload's default written shape, not a paragraph bolted on — names the merged tree by the sha that record names, and does not re-read or re-adjudicate the sibling branch here; and the reconciliation record itself is nowhere staged as an atom, only its outcome."
  satisfied: false
  evidence: ""
  mount_point: ".kb/_intake/ — the provenance passage in whichever persona document carries the audience's origin"
  verifying_test: "Tier 2 parametric content review per _decomposition.md '## Testing brief' ('The unrun counterpart'): the stated case matches what ls .bklg shows AND the prose parses sensibly under the other case; Tier 1 backstop: the sha in the staged text matches .bklg/docs-that-teach/durable-audience-closeout/reconciliation-ledger/spec.md's record and no staged file is a copy of that record"
- id: AC-007
  criterion: "GIVEN U2 opening any staged document expecting the corpus's shared shape rather than this story's invention, WHEN they read it, THEN it composes the repository's own primitives — the # title plus the atom template's ## Context / ## Body / ## Consequences / links skeleton, and the product README's exact four-slot vocabulary — and hand-rolls none of the alternatives the UX brief forbids: no 'Persona card', no 'TL;DR', no bespoke per-document table, no frontmatter key absent from .kb/_templates/atom.md, no id/kind/authority_tier claiming an atom identity the ingest has not minted, and no subdirectory under .kb/_intake/."
  satisfied: false
  evidence: ""
  mount_point: ".kb/_intake/ — flat, README untouched; the skeleton the ingest cuts each atom from"
  verifying_test: "Tier 2 content review against _decomposition.md '## UX brief' ('Design-system primitives — compose these, do not hand-roll' and its forbidden list) and .kb/_templates/atom.md; Tier 1 backstop: rg -n \"^## \" .kb/_intake/persona-*.md .kb/_intake/journey-*.md shows only the three skeleton headings, rg -n \"^(id|kind|authority_tier|last_reviewed):\" over the same files returns nothing, ls -d .kb/_intake/*/ returns nothing"
- id: AC-008
  criterion: "GIVEN the next story's implementer needing an invocation that provably cannot sweep .kb/_intake/README.md into the wave, WHEN they enumerate what this story staged, THEN every file is .kb/_intake/persona-<slug>.md or .kb/_intake/journey-<slug>.md — a prefix set disjoint from README.md and from a sibling's staged open_question document — and .kb/_intake/README.md is byte-identical to its pre-PR state, so the one act review cannot cheaply undo is prevented at invocation rather than repaired after the commit."
  satisfied: false
  evidence: ""
  mount_point: ".kb/_intake/ — the file list consumed by .bklg/docs-that-teach/durable-audience-closeout/audience-ingest-wave/spec.md's narrowed invocation (its AC-A02 obligation)"
  verifying_test: "Tier 1: ls .kb/_intake/ shows only README.md plus this story's prefixed files and any sibling's own, and git diff -- .kb/_intake/README.md is empty; Tier 3 mount-point check: the file list here is exactly what audience-ingest-wave/spec.md enumerates, per _decomposition.md '## Architecture brief' AC-A02 and T4"
- id: AC-009
  criterion: "GIVEN a maintainer reading a landed atom after the ingest has cleared .kb/_intake/, WHEN they follow the evidence the atom inherited, THEN it resolves — because every staged document named its _discovery/ artefacts inline at the claims they support, every named path resolved at staging time, and no staged document cited .kb/_intake/README.md as evidence; a charter that already cited the distillation and a charter that cites the promoted atom are pointed at the same audience rather than two."
  satisfied: false
  evidence: ""
  mount_point: ".kb/_intake/ — the inline evidence the ingest lifts into each atom's source_paths alongside the .kb/_intake/... staging path"
  verifying_test: "Tier 1: test -f over every path cited in every staged document, all pass; rg -n \"_intake/README\" .kb/_intake/persona-*.md .kb/_intake/journey-*.md returns nothing; Tier 2: the cited artefact supports the claim beside it, against _decomposition.md '## UX brief' IQ-6 and AC-UX-05 and the verified discovery set in '## Architecture brief' §4"
- id: AC-010
  criterion: "GIVEN a reader consuming this payload one line at a time — in a diff, a quote, or a screen reader — WHEN they encounter any state this story expresses, THEN it is a literal word: no emoji, tick, colour word, strikethrough, empty cell, ordering or absence carries meaning anywhere in this story's diff; each document has one # heading, sections at ##, no skipped level, and link text naming its destination rather than 'here' or 'see above'; and nothing outside this story's own folder and its new .kb/_intake/ files is modified, so no file:line anyone already cited moves."
  satisfied: false
  evidence: ""
  mount_point: ".kb/_intake/ plus this story's own folder under .bklg/docs-that-teach/durable-audience-closeout/staged-audience-payload/ — the whole fenced PR boundary"
  verifying_test: "Tier 1: git diff --stat lists only new .kb/_intake/persona-*.md and journey-*.md files and this story's own folder, and git diff shows no '-' line outside them; Tier 2 against _decomposition.md '## UX brief' '### Accessibility floor', AC-UX-09, IQ-3 and IQ-8, with the corpus precedent at .kb/maps/open-questions-index.md:136-143"
```
