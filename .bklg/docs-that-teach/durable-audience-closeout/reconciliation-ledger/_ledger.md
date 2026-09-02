---
item: HS-S0174
stage: implement
created: "2026-08-17T13:16:26.176Z"
updated: "2026-08-17T13:16:26.176Z"
---

# Acceptance ledger — Author the pair-by-pair audience reconciliation record

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Three notes for whoever flips these rows.

**The mount point is the same for all eight, and that is the point.** Every criterion is reachable
through the `## Companions` list in the prose body of
`.bklg/docs-that-teach/durable-audience-closeout/project.md` (currently `:291-301`), which is the
render path a closeout reviewer reaches this project's artefacts through. A record that exists at
its path but is not on that list is reachable only by knowing this story's slug — IQ-5's named
falsifier — so no row is satisfiable while the mount is missing.

**"Verifying test" here is a tier plus a command or a checklist, not a Rust test.** This project
compiles no new Rust, and `.bklg/docs-that-teach/durable-audience-closeout/_decomposition.md`'s
`## Testing brief` is explicit that Tier 2 — a human reading one claim at a time against IQ-1…IQ-8
and AC-UX-01…AC-UX-12 — is the *only* tier that can fail a plausible wrong implementation of the
content criteria. Tier 1 corroboration is named where it exists; it never substitutes for Tier 2 on
a content row (AC-TB-01).

**Evidence is a `file:line` into the record itself.** The artefact under test is
`.bklg/docs-that-teach/durable-audience-closeout/reconciliation-ledger/_reconciliation.md`; cite the
row, the sentence or the section that discharges each criterion, plus the command output where a
Tier 1 check is claimed.

```yaml
- id: AC-001
  criterion: "GIVEN U2 at the pull request with `.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md` open beside the record, WHEN they walk both sides item by item — this side's three personas (`:56`, `:148`, `:225`) and four charter journeys (`initiative.md:289-301`), and HS-S0131's staged set — THEN each one has exactly one ledger row carrying one closed-vocabulary verdict word and a reason that names its object, so U2 can disagree with exactly one adjudication without reading the discovery corpus. A pair with no row is a defect; a summary sentence never stands in for a row; a counterpart entry that cannot be read carries `not readable from this tree` rather than an invented pairing. (project AC-001, DR-1, AC-UX-03, IQ-2)"
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/durable-audience-closeout/project.md — the `## Companions` list in its prose body (currently :291-301), the render path a closeout reviewer reaches this record through"
  verifying_test: "Tier 2 content review (primary, sufficient alone per _decomposition.md `## Testing brief`, AC-001 row): the record read side by side with .bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md against AC-UX-03. Tier 1 corroboration: `rg -n \"^\\| \" .bklg/docs-that-teach/durable-audience-closeout/reconciliation-ledger/_reconciliation.md` yields >= 9 body rows and every verdict cell matches the closed vocabulary"
- id: AC-002
  criterion: "GIVEN U3 opening this record months later to correct it, WHEN they read the prose immediately above the ledger, THEN they learn which of the three merge-order cases actually held, what was looked at to determine it (`ls .bklg`, whether the sibling distillation path resolves, `ls .kb/_intake/` and `ls .kb/product/`), and the ledger beneath reads coherently under that case — including the case, live today, in which HS-S0131 has still not run and this initiative's set stands as the authored one, with the table degrading to `no counterpart staged` rows rather than a paragraph bolted on afterwards. The record never conflates \"HS-S0131 ran and staged output\" with \"the sibling's distillation became readable post-merge\". (project AC-002, DR-2, AC-UX-03, B-2)"
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/durable-audience-closeout/project.md — the `## Companions` list in its prose body (currently :291-301), the render path a closeout reviewer reaches this record through"
  verifying_test: "Tier 2 content review run parametrically per _decomposition.md `## Testing brief`, \"The unrun counterpart\": the stated case matches what `ls .bklg` shows at the moment of reading, AND the prose would still parse sensibly under the other case. Tier 1 corroboration: `ls .bklg` and `ls .kb/_intake/` on the merged tree, cross-checked against the record's stated observations"
- id: AC-003
  criterion: "GIVEN U3 wanting to re-run the observation rather than trust it, WHEN they read the record's attestation, THEN they find the merged commit as a full 40-character sha, the branch it was read on, and the command that produced it — enough to check the tree out and repeat every reading in the ledger. A ref name alone does not satisfy this; refs move, and the whole point of ordering 1 (\"merge before everything\") is that a reader can tell a stale observation from a current one. (project AC-012, AC-A06, AC-UX-12)"
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/durable-audience-closeout/project.md — the `## Companions` list in its prose body (currently :291-301), the render path a closeout reviewer reaches this record through"
  verifying_test: "Tier 1 static (primary): `rg -n \"[0-9a-f]{40}\" .bklg/docs-that-teach/durable-audience-closeout/reconciliation-ledger/_reconciliation.md` finds the sha and `git cat-file -t <sha>` resolves it to a commit on the merged branch. Tier 2 confirms the prose states what was read, on which tree, and when (_decomposition.md `## Testing brief`, AC-012 row)"
- id: AC-004
  criterion: "GIVEN the two downstream implementers who consume this record — `charter-open-question-disposition` (the evaluator decision) and `staged-audience-payload` (the surviving pairs and their caveats) — WHEN they open it as their only input, THEN they find the evaluator pair adjudicated with reasoning, resolved one way (a persona in its own right, or an earlier stage of the application author's journey) and naming both the differing verbs and the shared fear the distillation records; and they find all three carried qualifications stated as handoff obligations — per-persona observation status, the adapter author's internal-audit-only basis (`:336-342`), and the HS-S0131 overlap (`:343-359`), the last of which is this record's own output. The record marks these as inputs to those stories and discharges neither: it does not edit the charter and it writes no atom text. (project AC-001; DR-4; IQ-7; _storymap.md `## Coverage`, AC-003 and AC-006/AC-007 seams)"
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/durable-audience-closeout/project.md — the `## Companions` list in its prose body (currently :291-301), the render path a closeout reviewer reaches this record through"
  verifying_test: "Tier 2 content review (primary): the evaluator row and its adjacent sentence read against .bklg/docs-that-teach/_decomposition.md:134-139 and the distillation's own open question (:370-376); each of the three qualifications present. Tier 1 corroboration: `git diff --name-only` shows no change to .bklg/docs-that-teach/initiative.md and no file under .kb/"
- id: AC-005
  criterion: "GIVEN U2 asking \"did this closeout quietly invent a decision?\", WHEN they read the `## Routed gaps` section, THEN every finding that wanted a decision is written down there in prose with what was found, why it is not settled here, and where it goes (a later initiative, the `support` initiative per `.redkiln/config.yaml:5`, or a charter open question) — and the diff contains no addition to and no modification of `.kb/decisions/`. If there are no gaps, the section says so in a sentence rather than being absent, because an absent section and a section reporting nothing are indistinguishable to a reader and only one of them is evidence. (AC-A09; .kb/governance/rewrite-the-referent-never-the-reasoning.md; project.md \"Out of scope\", writing an ADR as a side effect)"
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/durable-audience-closeout/project.md — the `## Companions` list in its prose body (currently :291-301), the render path a closeout reviewer reaches this record through"
  verifying_test: "Tier 1 static (sufficient for the prohibition): `git diff --name-only` contains no path under .kb/, and `redkiln validate --kb`'s accepted-decision immutability check against HEAD fails structurally on any touch (_decomposition.md `## Testing brief`, Notes). Tier 2 for the section's presence and its routing sentences"
- id: AC-006
  criterion: "GIVEN a closeout reviewer who has never heard of this story's slug, WHEN they open `project.md` and read its `## Companions` list, THEN the reconciliation record is named there with link text that names the destination, reachable in one hop — and `git diff` over `project.md` shows an appended bullet and no deletion line anywhere, so every `file:line` anyone already cited into that file still points where it pointed. A record reachable only by knowing this story's slug is IQ-5's named falsifier. (IQ-3, IQ-5, IQ-6, AC-A05, AC-UX-06 applied to this story's one mount)"
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/durable-audience-closeout/project.md — the `## Companions` list in its prose body (currently :291-301), the render path a closeout reviewer reaches this record through"
  verifying_test: "Tier 3 mount-point walk (primary): open project.md, follow the `## Companions` link, land on the record. Tier 1 corroboration: `git diff -- .bklg/docs-that-teach/durable-audience-closeout/project.md` contains no line beginning `-` outside the diff header; `redkiln verify --grain story` reads this story's _ledger.md and PR boundary (.redkiln/config.yaml:40, :62-67)"
- id: AC-007
  criterion: "GIVEN any reader consuming this record one row at a time — a screen reader, a `git diff` hunk, a quotation in a review comment — WHEN they meet a single row or a single sentence out of context, THEN they lose nothing: every state is a literal word (no tick, no emoji, no colour word, no strikethrough-as-status, no blank cell meaning \"fine\", no state inferable only from a row's absence); every row names its side, item, counterpart, verdict, reason and source path without depending on the row above; and every `superseded`, every `kept distinct` and every routed gap also exists as a sentence outside the table. The corpus precedent is .kb/maps/open-questions-index.md, which spells `Open` / `Withdrawn` / `Superseded` as words, first. (AC-UX-09, AC-UX-10, IQ-8, the accessibility floor's \"colour, glyph and position never alone\")"
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/durable-audience-closeout/project.md — the `## Companions` list in its prose body (currently :291-301), the render path a closeout reviewer reaches this record through"
  verifying_test: "Tier 2 content review against AC-UX-09 and AC-UX-10 (primary). Tier 1 corroboration: an `rg` for the glyph set (tick, cross, coloured circle, `~~`) over _reconciliation.md returns nothing, and no ledger row has an empty leading cell"
- id: AC-008
  criterion: "GIVEN U2 reading this record beside the corpus's other written records, WHEN they take in its shape before reading a word of its content, THEN it composes from the corpus's existing primitives and nothing invented: one `#` title and `##` sections with no skipped level; exactly the four sections in the fixed order merge-order statement -> pair ledger -> routed gaps -> sha attestation, with the prose case stated above the table it governs; exactly the six columns of B-5 and at least nine body rows; exactly the five admissible verdict words; the ledger persistent on the page rather than revealed, collapsed or summarised behind a lead sentence; and no YAML frontmatter, no bespoke per-row table, no \"Persona card\" vocabulary, no new map format — the record is a project artefact under `.bklg/`, not an atom, and hand-rolling beside the primitive layer is exactly what _decomposition.md `## UX brief`, \"Hand-rolling, explicitly forbidden here\" refuses. (AC-UX-03 placement clause; the a11y floor's heading-structure and linear-readability clauses; `## Architecture brief` §4, \"the record itself stays under `.bklg/`\")"
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/durable-audience-closeout/project.md — the `## Companions` list in its prose body (currently :291-301), the render path a closeout reviewer reaches this record through"
  verifying_test: "Tier 2 content review as a structural read against the stated numbers (primary). Tier 1 corroboration: `rg -n \"^#\" _reconciliation.md` yields one `#` and exactly four `##` in the stated order; the file's first line is not `---`; the ledger's header row has six cells"
```
