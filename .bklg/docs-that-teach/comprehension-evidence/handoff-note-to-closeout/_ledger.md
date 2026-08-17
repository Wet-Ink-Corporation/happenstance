---
item: "HS-S0171"
stage: implement
created: "2026-08-17T13:16:23.376Z"
updated: "2026-08-17T13:16:23.376Z"
---

# Acceptance ledger — The hand-off note naming one directly observed persona

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

This story adds no `pub` item and touches no crate, so every `verifying_test` below is a real,
runnable Static-tier command or a ledger-cited Artifact-evidence read against a real path — the two
tiers `.bklg/docs-that-teach/comprehension-evidence/_decomposition.md`'s `## Testing brief` defines
for this project. Each was chosen because it rejects a named wrong implementation; the brief's
forbidden check ("a `Persona:` line exists") appears nowhere below. Most `evidence` values will be
`file:line` citations into `.bklg/docs-that-teach/comprehension-evidence/_friction-log.md` itself,
because that file *is* the proof artifact.

```yaml
- id: AC-001
  criterion: "GIVEN HS-P0025's implementer opens the log at ## Hand-off and reads nothing else, WHEN they need to write DR-5's sentence — the observation qualification is corrected, not copied — THEN the section names exactly one persona as directly observed, spelled character-for-character as one of the three ## headings in .bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md, with one sentence of what that reader was trying to accomplish; names the other two and marks each not directly observed; and, where the recruited reader fitted the chosen persona only partially, states the divergence in one sentence rather than smoothing it."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/comprehension-evidence/_friction-log.md → ## Hand-off (section 8 of 8), reached from .bklg/docs-that-teach/comprehension-evidence/project.md → ## Companions"
  verifying_test: "Static: the persona string in ## Hand-off matches one of the headings returned by rg -n '^## Persona' .bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md (lines 56, 148, 225) exactly; all three names appear in the section and exactly one carries the directly-observed marker. Plus Artifact-evidence: ledger-cited file:line at the slot."

- id: AC-002
  criterion: "GIVEN a reader of the hand-off alone asks whether a stumble still applies today, WHEN they look for what was walked and when, THEN the section carries the session date and the tree as a resolvable commit or merge id, both transcribed from the log's ## Session record and agreeing with it character-for-character — not the authoring date of the note, not a date range, not a branch name that will move."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/comprehension-evidence/_friction-log.md → ## Hand-off (section 8 of 8), reached from .bklg/docs-that-teach/comprehension-evidence/project.md → ## Companions"
  verifying_test: "Static provenance: git show <the cited tree id> resolves from this worktree, and a diff of the date and tree strings in ## Hand-off against the same fields in ## Session record of .bklg/docs-that-teach/comprehension-evidence/_friction-log.md is empty."

- id: AC-003
  criterion: "GIVEN the initiative's BR-14 forbids a claim wider than one session supports, WHEN HS-P0025 lifts this section into a promoted atom, THEN the scope sentence scope-the-claim wrote into ## Scope of the claim appears in the section as the same string, and nothing in the section asserts exhaustiveness — not by the word, and not by the softer forms ('the documentation was validated', 'readers can now follow the material')."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/comprehension-evidence/_friction-log.md → ## Hand-off (section 8 of 8), reached from .bklg/docs-that-teach/comprehension-evidence/project.md → ## Companions"
  verifying_test: "Static: rg -F '<the scope sentence>' .bklg/docs-that-teach/comprehension-evidence/_friction-log.md returns hits in both ## Scope of the claim and ## Hand-off; rg -i 'exhaustiv' returns nothing in the section outside a disclaiming sentence — the mechanism the ## Testing brief's AC-008 row specifies. Plus Artifact-evidence: a reviewer reads the section for the soft overclaims, which pass the grep and fail the criterion."

- id: AC-004
  criterion: "GIVEN the UX brief's state list admits only run and declined-with-reason, and states that 'Not mentioned' is not one of the states, WHEN a reader of the summary asks whether a second session was owed, THEN the section carries exactly one of run or 'declined — <reason>', transcribed from second-session-decision's record and agreeing with it, with zero occurrences of the token left anywhere in the section."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/comprehension-evidence/_friction-log.md → ## Hand-off (section 8 of 8), reached from .bklg/docs-that-teach/comprehension-evidence/project.md → ## Companions"
  verifying_test: "Static: rg -c 'NOT-YET-RECORDED' scoped to the ## Hand-off section of .bklg/docs-that-teach/comprehension-evidence/_friction-log.md returns 0, and the verdict line matches one of the two legal forms. Plus Artifact-evidence: ledger-cited file:line, cross-read against .bklg/docs-that-teach/comprehension-evidence/second-session-decision/spec.md's verdict vocabulary."

- id: AC-005
  criterion: "GIVEN the brief marks 'Stumble recorded, undispositioned' as legal during the session, illegal at hand-off, WHEN U2 sits down to close the record out, THEN the section states as a dated, checked fact that every severity-marked stumble carries exactly one disposition (zero and two are both failures), every routed: id resolves to a real item and every escalated: id is a DT id from .bklg/docs-that-teach/_decomposition.md's ownership table — and if any of those is false the hand-off is not written at all, rather than written with the gap mentioned in passing."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/comprehension-evidence/_friction-log.md → ## Hand-off (section 8 of 8), reached from .bklg/docs-that-teach/comprehension-evidence/project.md → ## Companions"
  verifying_test: "Static: a per-entry disposition-arm count over rg -n '^### FL-' and the Disposition: fields of .bklg/docs-that-teach/comprehension-evidence/_friction-log.md yields exactly one arm each; test -f .bklg/support/initiative.md && test -f .bklg/docs-that-teach/application-author-path/project.md && test -f .bklg/docs-that-teach/reach-and-adapter-path/project.md; rg '^\\| DT-' .bklg/docs-that-teach/_decomposition.md covers every escalated id. Plus Artifact-evidence: the dated check line, ledger-cited."

- id: AC-006
  criterion: "GIVEN a reviewer deletes the entire ## Hand-off section, WHEN they re-read the log, THEN not one stumble, severity, disposition or reason has been lost (IQ-2's deletion check) — because every claim in the section resolves to something already written elsewhere in the log; every citation the section makes into the record uses a frozen FL-### heading anchor rather than a re-description (IQ-3); the disposition of any cited stumble remains readable at that stumble, so the section is at most one optional hop (IQ-1); and any post-hand-off correction appends a dated revision line beside the original with the earlier text still legible (IQ-4)."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/comprehension-evidence/_friction-log.md → ## Hand-off (section 8 of 8), derived from its ## Chronological record, ## Session record and ## Scope of the claim"
  verifying_test: "Artifact-evidence: for each sentence in the section, the file:line elsewhere in .bklg/docs-that-teach/comprehension-evidence/_friction-log.md it resolves to, with the IQ-2 deletion check performed and its result recorded. Plus Static: every citation in the section matches an existing '^### FL-' heading and rg -n '<details>|<summary>|<script>' returns nothing. Authority: .kb/governance/rewrite-the-referent-never-the-reasoning.md."

- id: AC-007
  criterion: "GIVEN U3 reads the log as a raw git diff or through a screen reader, with every style stripped, WHEN they land on the section, THEN it is composed from this repository's existing document primitives — labelled one-line fields on the brief spine, the ## Shape decision / routing-table shapes, one-line checkboxes — as five named slots plus the precondition line, in the reading order HS-P0025 needs (persona, date, tree, scope sentence, verdict); it introduces no new heading, no fold, no widget and no script; nothing carries meaning by colour or emoji alone; and it is complete and navigable by browser find and stable heading anchors alone."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/comprehension-evidence/_friction-log.md → ## Hand-off (section 8 of 8), composed from the document primitives .bklg/docs-that-teach/comprehension-evidence/_design.md names"
  verifying_test: "Static: the section adds no '^## ' or '^### ' heading beyond the slots the skeleton fixed; every checkbox line matches a single-line '- [ ] …'; git show HEAD:.bklg/docs-that-teach/comprehension-evidence/_friction-log.md read as raw bytes carries no emoji or colour-swatch glyph in a meaning-bearing position. Plus Artifact-evidence: a reviewer maps each slot to one primitive named in .bklg/docs-that-teach/comprehension-evidence/_decomposition.md's '#### The primitive layer to compose from — do not hand-roll'."

- id: AC-008
  criterion: "GIVEN a person who has never seen this story reaches HS-P0024 from redkiln board, and separately a person reaching HS-P0025 from the same board, WHEN each looks for the evidence the closeout promotion rests on, THEN the first reaches the filled section in two hops (project card → ## Companions → the log, then the in-page ## Hand-off anchor), and the second finds a one-line pointer in .bklg/docs-that-teach/durable-audience-closeout/project.md's ## Dependencies naming the log by path and the section by heading anchor — with no frontmatter hunk in either file's diff and no file under .kb/ or .kb/_intake/ anywhere in this story's diff."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/comprehension-evidence/project.md → ## Companions (first mount, the two-hop reach), plus .bklg/docs-that-teach/durable-audience-closeout/project.md → ## Dependencies (second mount, the one-line pointer)"
  verifying_test: "Static: rg -n '_friction-log' .bklg/docs-that-teach/comprehension-evidence/project.md .bklg/docs-that-teach/durable-audience-closeout/project.md returns both the existing Companions row and the new pointer; git diff shows no hunk above either file's closing '---'; git diff --name-only returns no path under .kb/."
```
