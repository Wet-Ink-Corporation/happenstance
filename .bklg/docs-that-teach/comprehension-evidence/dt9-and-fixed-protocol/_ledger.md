---
item: "HS-S0162"
stage: implement
created: "2026-08-17T13:16:17.924Z"
updated: "2026-08-17T13:16:17.924Z"
---

# Acceptance ledger — Resolve DT-9 and fix the protocol before recruitment

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

**Evidence shape for this story.** It ships no code and no test functions. The testing brief
(`../_decomposition.md`, `## Testing brief`) sanctions `file:line` citations into the proof artifact
itself, plus static and provenance mechanisms, calling that *not a gap in rigor, the shape rigor
takes when the deliverable is a record rather than a function*. So `evidence` values here are
`.bklg/docs-that-teach/comprehension-evidence/_design.md:<line>` citations (and, for AC-009, a commit
sha with its author date), and `verifying_test` values are the real commands from the spec's
`## Tests and CI (merge gate)` table rather than test-function ids.

```yaml
- id: AC-001
  criterion: "GIVEN U2 the facilitator must choose which reader the session serves, and the three candidates want measurably different things — model a cross-entity consistency boundary correctly on the first real attempt, understand *why* the storage contract is shaped as it is, or decide inside twenty minutes whether to depend on this — WHEN a reviewer opens `_design.md` at its new DT-9 section, THEN it names exactly one chosen reader by that reader's own goal and never by bare label, and names exactly one comprehension technique from the published paraphrase / plus-minus / task-based taxonomy with the fit argued against that goal rather than asserted, so a technique mismatched to the goal verb is visibly a defect."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/comprehension-evidence/_design.md"
  verifying_test: "rg -n '^## DT-9' .bklg/docs-that-teach/comprehension-evidence/_design.md && rg -n -i 'paraphrase|plus-minus|task-based' .bklg/docs-that-teach/comprehension-evidence/_design.md; reviewer-read at .redkiln/processes/project.yaml `design` stage"

- id: AC-002
  criterion: "GIVEN U3 reading this file six months later must be able to tell a decision from a default, WHEN they read the DT-9 row, THEN it is composed into the repository's existing `## Shape decision` table primitive with all five columns filled (`Item` / `Chosen shape` / `Rejected (and why)` / `Evidence` / `Resolves`, with `Resolves` carrying `DT-9`), and the `Rejected` cell carries a distinct reason per rejected persona — including, where they were weighed, Persona 2's recruitability and the thinness of its evidence base — never one shared sentence covering both."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/comprehension-evidence/_design.md"
  verifying_test: "rg -n 'Chosen shape' .bklg/docs-that-teach/comprehension-evidence/_design.md compared against .redkiln/templates/_design.md:64; rg -n 'DT-9' over the same file"

- id: AC-003
  criterion: "GIVEN HS-P0025 must at closeout replace the blanket *none of these three personas has been directly observed* qualification with an accurate one rather than promoting three personas from one session, WHEN it lifts a sentence out of this section, THEN one sentence in that same section states which single persona this initiative will have directly observed and that the other two personas' evidence remains inferred, in a form liftable verbatim without re-derivation."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/comprehension-evidence/_design.md"
  verifying_test: "rg -n -i 'remains inferred|stays inferred' .bklg/docs-that-teach/comprehension-evidence/_design.md; reviewer confirms the other two personas are named explicitly"

- id: AC-004
  criterion: "GIVEN U1 the recruited reader must be able to start work from the scenario alone, having never seen this repository, WHEN the facilitator reads it to them at the session, THEN `_design.md` carries it as one to two sentences in the reader's own terms, naming that reader's goal and not this documentation's structure — neither so narrow that completing it proves nothing nor so esoteric that it is dismissed as an edge case."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/comprehension-evidence/_design.md"
  verifying_test: "rg -n -i 'scenario' .bklg/docs-that-teach/comprehension-evidence/_design.md locates the clause; reviewer-read cold against ../_decomposition.md UX-AC-002 for the one-to-two-sentence budget"

- id: AC-005
  criterion: "GIVEN U2 must not later report an inflated elapsed time as though it were comparable across sessions, WHEN a reviewer reads the narration clause, THEN it declares concurrent or retrospective as a stated methodological choice with its reason, records that concurrent narration inflates task time by roughly 17–20%, and states that elapsed time is therefore not a comparable metric across sessions."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/comprehension-evidence/_design.md"
  verifying_test: "rg -n -i 'concurrent|retrospective|17' .bklg/docs-that-teach/comprehension-evidence/_design.md; reviewer confirms mode, reason and the non-comparability statement are all three present"

- id: AC-006
  criterion: "GIVEN U3 opens the eventual log on a `git diff` view that has no colour at all, or through a screen reader, WHEN they reach a severity mark, THEN the scale named in `_design.md` is a named, published scale (Nielsen's 0–4 is the supplied candidate; the choice is this file's), every mark on it is a text token — the word, or the number on that scale — with no emoji or colour swatch carrying the meaning alone, and the clause states the mark is applied inline as the log is written and never retrofitted."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/comprehension-evidence/_design.md"
  verifying_test: "rg -n -i 'severity' .bklg/docs-that-teach/comprehension-evidence/_design.md finds the named scale and its levels; the clause read as plain text with styling stripped carries no emoji and no colour swatch (../_decomposition.md `#### The accessibility floor` bullet 1; UX-AC-003)"

- id: AC-007
  criterion: "GIVEN a reviewer in HS-S0164 must check a real candidate's declaration without asking the reader anything further, WHEN they hold that declaration against this file, THEN `_design.md` lists the disqualifying criteria as checkable statements about artefacts — having authored the material under test; having read `crates/happenstance-core/`'s source, `references/adr/`, or `references/evaluation/` — never a judgement call, and states that if an ineligible reader is used anyway the artefact is unmet and the log records that, which is never a reason to redefine the bar."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/comprehension-evidence/_design.md"
  verifying_test: "rg -n 'happenstance-core|references/adr|references/evaluation' .bklg/docs-that-teach/comprehension-evidence/_design.md && test -d crates/happenstance-core && test -d references/adr && test -d references/evaluation"

- id: AC-008
  criterion: "GIVEN HS-S0163 must build a log scaffold it did not invent, and sibling projects will later cite heading anchors and stumble ids into that log, WHEN its implementer opens `_design.md`, THEN the file names the log's repo-relative file path and fixes its heading vocabulary (scenario; logger identity, context and date; chronological record; disposition; scope sentence; hand-off) as stable anchors, so a citation made into a heading or a stumble id still resolves after the log is finalised, and the log so described is navigable by browser find alone — no widget, script or rendering tool required to read it."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/comprehension-evidence/_design.md"
  verifying_test: "rg -n 'friction' .bklg/docs-that-teach/comprehension-evidence/_design.md yields a concrete repo-relative path and the six heading names; HS-S0163's ledger must be able to cite those exact lines as its input"

- id: AC-009
  criterion: "GIVEN project AC-002 is a provenance claim — the protocol could not have been written to fit what the reader did — and not a quality claim, WHEN a reviewer runs `git log --format=%aI -- .bklg/docs-that-teach/comprehension-evidence/_design.md`, THEN a single commit carries all five elements (persona, scenario, narration mode, severity scale, disqualifying criteria) and its author date precedes both the session date the log will record and any contact with any candidate; no candidate is approached, contacted or screened in this PR."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/comprehension-evidence/_design.md"
  verifying_test: "git log --format=%aI -- .bklg/docs-that-teach/comprehension-evidence/_design.md (one date, one commit) and git diff --name-only main..HEAD (no recruitment artefact, no file outside the PR boundary)"

- id: AC-010
  criterion: "GIVEN the human who approved `_design.md` on 2026-08-17 approved a file with no persona in it, and rewriting that approval to read as though it had covered the persona would convert a true claim into a false one, WHEN they review this amendment at the `design` gate, THEN the existing sign-off paragraph is unchanged, the protocol is appended as new sections beneath the template's existing heading spine with its own sign-off line below the original, `hasSurface: false` and the `# no items` fenced block are untouched, the addition is composed only from the repository's existing document primitives — the `## Shape decision` table, the `## Sign-off` section, the Intent/AC/Notes spine, the two-column routing table, the one-line checkbox — with no fold, no widget and no new format invented, and every checklist box added stays on one line."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/comprehension-evidence/_design.md"
  verifying_test: "git diff -- .bklg/docs-that-teach/comprehension-evidence/_design.md shows no removed line in the sign-off paragraph or the `# no items` block; rg -n 'details>|summary>' returns nothing; rg -n '\\- \\[ \\]' shows one box per line; .redkiln/processes/project.yaml `design` gate verdict approved"
```
