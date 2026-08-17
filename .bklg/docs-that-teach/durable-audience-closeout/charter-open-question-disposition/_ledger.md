---
item: HS-S0175
stage: implement
created: 2026-08-17
updated: 2026-08-17
---

# Acceptance ledger — Dispose of every charter open question, evaluator included

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Every `verifying_test` below is a **Tier 2 content review** item (`T2-01 … T2-08`, defined in
`spec.md`, `## Tests and CI (merge gate)`) paired with its Tier 1 static reader. That is not a
weakening: the testing brief makes Tier 2 the *primary* tier for both project criteria this story
traces to, because no automated reader of prose exists
(`.bklg/docs-that-teach/durable-audience-closeout/_decomposition.md`, `## Testing brief`,
"AC-### → tier mapping"). Tier 1 is never admissible as evidence on its own for any row here.

```yaml
- id: AC-001
  criterion: "GIVEN U2 opens _disposition.md having never read the discovery corpus, WHEN they check it against the charter's `## Open questions for the planning team`, THEN they find exactly one row per bullet — Q1 through Q11, keyed by charter line anchor — with no bullet folded into a neighbouring row, no bullet omitted, and no summary sentence standing in for a row, so a reviewer can walk `initiative.md:517-555` top to bottom and land on a row every time"
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/initiative.md — the appended `## Open-question disposition (closeout, HS-S0175)` section, whose row set is the eleven bullets at :513-555"
  verifying_test: "T2-01 — read .bklg/docs-that-teach/initiative.md:517-555 against .bklg/docs-that-teach/durable-audience-closeout/charter-open-question-disposition/_disposition.md one bullet at a time; Tier 1: rg -c '^\\| Q[0-9]' over _disposition.md returns 11"

- id: AC-002
  criterion: "GIVEN U2 reading the record one row at a time — in a diff, a quoted excerpt, or a screen reader — WHEN they read any single row, THEN its state is one of the four literal words `answered on the record`, `withdrawn`, `open_question atom`, `not a question`, and the row names its own Q id, charter anchor, state word and evidence, so that nothing is carried by an emoji, a tick, a colour word, a strikethrough, an empty cell, or the row's position"
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/initiative.md — the appended disposition section, and .bklg/docs-that-teach/durable-audience-closeout/charter-open-question-disposition/_disposition.md it points at"
  verifying_test: "T2-02 — the accessibility-floor read against .bklg/docs-that-teach/durable-audience-closeout/_decomposition.md AC-UX-09 and AC-UX-10; Tier 1: rg over this story's diff finds no status glyph or strikethrough"

- id: AC-003
  criterion: "GIVEN U2 wants to disagree with exactly one adjudication and not re-litigate the initiative, WHEN they read a row whose state is `answered on the record` or `withdrawn`, THEN that row names the artefact elsewhere in the initiative that actually decided it by repo-relative path — by file:line where the artefact is long — plus one sentence of what it decided, that path resolves on the merged tree, and no such row offers _disposition.md itself as its authority"
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/durable-audience-closeout/charter-open-question-disposition/_disposition.md — the evidence column of every answered or withdrawn row"
  verifying_test: "T2-03 — per answered/withdrawn row, open the cited artefact and confirm it decides the question; Tier 1: test -f over every cited path, plus rg for a row citing only _disposition.md"

- id: AC-004
  criterion: "GIVEN U1 beginning the next charter needs to know whether they inherit three personas or four, WHEN they read the evaluator disposition, THEN exactly one statement exists choosing `a persona in its own right` or `an earlier stage of the application author's journey`, its reasoning is written out rather than asserted, it agrees with reconciliation-ledger's evaluator pair row, and no second contradicting statement exists anywhere in this PR's diff"
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/durable-audience-closeout/charter-open-question-disposition/_disposition.md — the evaluator decision stated as prose above the table and as the Q8 row; surfaced at .bklg/docs-that-teach/initiative.md via the appended section"
  verifying_test: "T2-04 — read the statement against .bklg/docs-that-teach/durable-audience-closeout/reconciliation-ledger/ and .bklg/docs-that-teach/_decomposition.md:134-139; Tier 1: rg over the diff finds one coherent statement and no contradiction"

- id: AC-005
  criterion: "GIVEN a charter question that no artefact in this initiative actually decided, WHEN U2 reads its row, THEN the row states `open_question atom`, a matching document exists under .kb/_intake/ in the what-is-true-today / what-is-not-decided / what-forces-it shape with a grounded path behind every claim, the question is not argued to a conclusion in the record instead, and where a disposition concludes a decision atom is warranted it is written as a routed gap in prose — with this PR's diff adding nothing to and modifying nothing in .kb/decisions/ and creating no file under .kb/open-questions/"
  satisfied: false
  evidence: ""
  mount_point: ".kb/_intake/ — the staging directory consumed by the single M3 /redkiln:kb-ingest wave; the record's routed-gap prose in _disposition.md for anything that wants a decision"
  verifying_test: "T2-05 — read each staged .kb/_intake/ document against .kb/open-questions/README.md; Tier 1: git diff --name-only shows no path under .kb/decisions/ or .kb/open-questions/, and redkiln validate --kb exits zero"

- id: AC-006
  criterion: "GIVEN the M3 implementer who will run audience-ingest-wave and has only this record, WHEN they open its handoff block, THEN for each deferred question they get the staged file name, the target `##` section in .kb/maps/open-questions-index.md — a new section, since documentation has none today and the index permits starting one only in that case — and the bullet text in the index's own shape, status word first, then the link, then the atom id, then one sentence, plus the instruction to narrow the wave's invocation so .kb/_intake/README.md is excluded"
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/durable-audience-closeout/charter-open-question-disposition/_disposition.md — the M3 handoff block, consumed by audience-ingest-wave; its target is .kb/maps/open-questions-index.md"
  verifying_test: "T2-06 — read the handoff block against .kb/maps/open-questions-index.md:136-143 and confirm the targeted section does not already exist; Tier 1: rg '^## ' .kb/maps/open-questions-index.md shows the two existing sections untouched"

- id: AC-007
  criterion: "GIVEN U2 meets these questions in the charter and not in this project's folder, WHEN they open .bklg/docs-that-teach/initiative.md, THEN one `##` section appended at end of file carries one row per Q with its state word and where the answer lives, so the charter itself marks each question answered; git diff on that file contains no deletion line; and every file:line citation of the charter already made from project.md, both _decomposition.md files and the briefs still resolves to the same text"
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/initiative.md — one `## Open-question disposition (closeout, HS-S0175)` section appended at end of file, body prose beneath the closing frontmatter delimiter"
  verifying_test: "T2-07 — read the appended section as a reader arriving at the charter cold, one hop to the record; Tier 1: git diff .bklg/docs-that-teach/initiative.md contains no deletion line, and spot-checks of initiative.md:165-166, :275-288, :416-420, :524-525, :538-540 still land on the cited text"

- id: AC-008
  criterion: "GIVEN Q10 is a standing instruction to re-verify three specific claims rather than a question to answer, WHEN U2 reads its row, THEN each of examples/outside-projection-adapter/, references/seeds/measured-not-claimed.md and publication-and-positioning's asserted _design.md / HS-S0131 stage is recorded present or absent as checked on the merged tree, with the merged commit sha named beside the result, and no Q10 result is recorded against this worktree's pre-merge copy"
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/durable-audience-closeout/charter-open-question-disposition/_disposition.md — the Q10 row and its three recorded results, each naming the merged sha produced by merge-forward-baseline"
  verifying_test: "T2-08 — read the three results against the sha named and confirm it is merge-forward-baseline's; Tier 1: git cat-file -e <sha> resolves and git ls-tree <sha> over the three Q10 paths agrees with the recorded present/absent"
```
