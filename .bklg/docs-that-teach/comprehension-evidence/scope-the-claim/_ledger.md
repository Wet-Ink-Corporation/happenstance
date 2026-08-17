---
item: HS-S0169
stage: implement
created: "2026-08-17T13:16:21.956Z"
updated: "2026-08-17T13:16:21.956Z"
---

# Acceptance ledger — Scope the claim to what one session supports

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

`mount_point` throughout is the friction log landed by `friction-log-skeleton` at
`.bklg/docs-that-teach/comprehension-evidence/_friction-log.md` — `## Scope of the claim` being the
canonical string and the source of record, with `## Status`, the `## Dispositions index` preamble and
the `## Hand-off` scope slot as its in-file propagation points and
`.bklg/docs-that-teach/comprehension-evidence/project.md` → `## Companions` as the one propagation
point outside the log (`spec.md`, `## Integration contract`). Where `_design.md` fixes a different
path for the log, it substitutes verbatim, per EC-008. `verifying_test` values are real checks
against real paths rather than test-file ids — this project has no functions and no test binary, and
per `_decomposition.md`'s testing brief each check asserts on content, not merely on structural
presence.

```yaml
- id: AC-001
  criterion: "GIVEN the scope sentence was written before the session so it could not be tuned to what the reader happened to hit, WHEN U3 six months later asks whether the claim was trimmed to fit the findings, THEN `## Scope of the claim`'s body is the string the skeleton committed, character for character — and where it was genuinely defective, the correction sits **beside** the original as a dated additive amendment naming what it replaces and why, never as an in-place overwrite"
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/comprehension-evidence/_friction-log.md — `## Scope of the claim`, the canonical string and source of record"
  verifying_test: "Provenance (static): `git log -p --follow -- .bklg/docs-that-teach/comprehension-evidence/_friction-log.md` shows the `## Scope of the claim` body unchanged since `friction-log-skeleton`'s commit, or extended by an append-only amendment hunk with no deletion inside the section; that commit date precedes the session date in `## Session record`"
- id: AC-002
  criterion: "GIVEN U3 meets this evidence at whichever point they happen to land on — the board card, the `## Status` banner, the dispositions index, the hand-off slot — WHEN they read only that point and never open the chronological record, THEN the canonical string is there, byte-identical, at every one of the five enumerated locations, each rendered as a composed line in that section's existing shape rather than a paragraph dropped under a heading"
  satisfied: false
  evidence: ""
  mount_point: "All five: `.bklg/docs-that-teach/comprehension-evidence/_friction-log.md` — `## Scope of the claim`, `## Status`, the `## Dispositions index` preamble, the `## Hand-off` scope slot; and `.bklg/docs-that-teach/comprehension-evidence/project.md` — `## Companions`"
  verifying_test: "Presence + byte-identity (static): `rg -F -n \"<canonical string>\" .bklg/docs-that-teach/comprehension-evidence/_friction-log.md .bklg/docs-that-teach/comprehension-evidence/project.md` returns a hit inside each of the five named sections, and a pairwise `diff` of the five extracted spans shows zero divergence — a fixed-string search, so a paraphrase cannot satisfy it"
- id: AC-003
  criterion: "GIVEN a promotion reviewer at closeout is deciding what may honestly be written into a persona atom, WHEN they scan the log and everything that summarises it for any assertion of coverage, THEN they find none on the full Decision 4 vocabulary — not merely the word \"exhaustive\" — and every hit the sweep does return sits inside the disclaiming sentence itself"
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/comprehension-evidence/_friction-log.md (whole file) and `.bklg/docs-that-teach/comprehension-evidence/project.md` — `## Companions`"
  verifying_test: "Negative sweep (static, `rg -i`) over both files for the Decision 4 exhaustiveness vocabulary — `exhaustiv`, `comprehensive`, `complete (set|picture|coverage)`, `all the (issues|problems|stumbles)`, `every (issue|problem)`, `no (other|further|major) issues`, `representative` — run as one alternation and recorded verbatim in the PR; `rg -i \"exhaustiv\"` run separately and named as the floor per `_decomposition.md`'s testing brief AC-008 row"
- id: AC-004
  criterion: "GIVEN exactly one reader walked the material exactly once, WHEN any summary of this evidence describes that walk, THEN it speaks of that one reader and never of \"users\" or \"readers\" in general, presents counts as counts and never as a rate or a percentage, and issues no verdict on the documentation as a whole"
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/comprehension-evidence/_friction-log.md — `## Status`, the `## Dispositions index` preamble, `## Scope of the claim`, the `## Hand-off` scope slot; and `.bklg/docs-that-teach/comprehension-evidence/project.md` — `## Companions`"
  verifying_test: "Negative sweep (static, `rg -i`) for the plural slide (`users`, `readers`, `typical (reader|user|developer)`, `most (readers|users)`, `readers (generally|tend|will)`), the rate (`%`, `success rate`, `pass rate`) and the verdict (`validated`, `broadly usable`, `tested well`, `signed off as usable`), each surviving hit adjudicated against the disclaiming sentence and ledger-cited; a phrase inside a reader's own quoted words in an `FL-###` entry is out of scope per EC-005"
- id: AC-005
  criterion: "GIVEN HS-P0025 must decide whether this evidence licenses anything at all, WHEN they read the scope statement at any of the five locations, THEN the positive half is there with its basis — each recorded stumble is a real stumble a real reader hit, traceable to an `FL-###` id, a named tree SHA and a dated session, already worth fixing without needing to recur — so the scoping reads as a **licence with a boundary**, not as \"weak evidence, discount it\""
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/comprehension-evidence/_friction-log.md — `## Scope of the claim`, carried to the other four locations"
  verifying_test: "Artifact-evidence (ledger `file:line` at the sentence) plus static presence of the traceability triple reachable from the claim — an `FL-###` reference, the tree identifier from `## Session record`, and the session date — checked against `.bklg/docs-that-teach/comprehension-evidence/_grounding.md:72-76` (Nielsen & Landauer 1993)"
- id: AC-006
  criterion: "GIVEN the session recorded **ineligible-but-used-anyway**, or ended in `Kind: abandonment`, WHEN U3 reads any single one of the five locations and no other, THEN that qualification is stated there too — the artefact is unmet, or the walk was partial and stopped where it stopped — and GIVEN neither occurred, no location invents a qualification that the session did not record"
  satisfied: false
  evidence: ""
  mount_point: "All five locations; the source state is `.bklg/docs-that-teach/comprehension-evidence/_friction-log.md` — `## Session record` and `## Chronological record`"
  verifying_test: "Conditional inheritance (static): if `rg -n \"ineligible-but-used-anyway\" .bklg/docs-that-teach/comprehension-evidence/_friction-log.md` hits inside `## Session record`, the unmet statement is present at each of the five locations; if `rg -n \"Kind: abandonment\"` hits, the partial-walk statement is present at each; both directions ledger-cited, per `session-run-against-pinned-tree`'s EC-002"
- id: AC-007
  criterion: "GIVEN a reviewer deletes every derived copy of the claim from the log and the project card, WHEN they re-read what is left, THEN the claim is still stated in full at `## Scope of the claim` and nothing has been lost — because each copy named that section as its source of record before it was deleted — and GIVEN HS-P0025 wants the string, they lift it in one hop as plain text: browser find and stable heading anchors, no fold, no widget, no script, no rendering step"
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/comprehension-evidence/_friction-log.md — `## Scope of the claim` as the source of record, with the four derived copies labelled as copies"
  verifying_test: "Deletion check (static), stated verbatim in `_decomposition.md`'s IQ-2: in a scratch copy delete the `## Status` claim line, the index preamble line, the hand-off slot line and the `## Companions` row, then confirm `## Scope of the claim` still carries the whole claim including its positive half and any inherited qualification; plus `rg -n \"<details>|<summary>|<script>\" <log>` returns nothing and `rg -n \"^## \" <log>` returns the skeleton's eight headings unchanged and in order"
```
