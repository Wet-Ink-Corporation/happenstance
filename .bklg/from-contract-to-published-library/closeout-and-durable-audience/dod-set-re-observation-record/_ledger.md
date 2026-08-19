---
item: "HS-S0127"
stage: implement
created: "2026-08-12"
updated: "2026-08-12"
---

# Acceptance ledger — DoD 1–12 and 14–15 re-observed as a set on this tree

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

```yaml
- id: AC-001
  criterion: "GIVEN a reader who opens the project's single closeout artefact expecting one place where the closeout evidence lives, WHEN they scroll past the Gate run section whole-gate-green-on-the-assembled-tree wrote, THEN they meet exactly one `## DoD re-observation (1–12, 14–15)` section inside that same file, no rival dod-re-observation.md or per-scenario file exists beside it in the project directory, and every line written before it is byte-identical — the section is appended, and nothing earlier is reflowed, renumbered or restructured."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md"
  verifying_test: "`rg -c '^## DoD re-observation' .bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md` == 1; `git diff <slice-1-sha>..HEAD -- .bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md` shows zero deleted lines; project-directory listing shows no new *.md beside the record — captured in .bklg/from-contract-to-published-library/closeout-and-durable-audience/dod-set-re-observation-record/evidence/record-shape-checks.md"

- id: AC-002
  criterion: "GIVEN the same reader counting what was promised against what was observed, WHEN they read the section, THEN it presents a markdown table — not a bullet list and not prose — of exactly fourteen data rows under an eight-column header, whose DoD column is exactly the set {1,2,3,4,5,6,7,8,9,10,11,12,14,15}; and immediately beneath it a note names DoD 13 as published-tree-delta-statement's and DoD 16 as product-atom-promotion-via-kb-ingest's, so fourteen reads as scope rather than as two forgotten scenarios."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md"
  verifying_test: "Row/column extraction over the section (14 data rows, header cell count == 8, DoD id-set equality) plus slug presence and directory existence for both excluded-scenario owners under .bklg/from-contract-to-published-library/closeout-and-durable-audience/ — captured in .bklg/from-contract-to-published-library/closeout-and-durable-audience/dod-set-re-observation-record/evidence/record-shape-checks.md"

- id: AC-003
  criterion: "GIVEN a reader who has been told these were re-observed and wants to know what that word bought, WHEN they read any row, THEN the row's Mode cell carries a token from the three-mode legend printed in the section itself (re-run, conditional re-run, re-inspect) — DoD 10 alone carrying two tokens for its two halves; every re-inspect row states in the row why no command re-derives the claim here (a documentary verdict, or a property of the published artefact rather than of this tree) and confirms the claim still holds at this SHA; and every conditional re-run row that could not execute names the absent environment in DR-2's absent-tool shape, so no row degrades silently into a re-read."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md"
  verifying_test: "Per-row mode-token membership check; re-inspection set equals {DoD 8, DoD 10 registry half, DoD 11} with a reason clause on each; DoD 5 and DoD 6 each show captured run output or a named absent environment — read against .bklg/from-contract-to-published-library/closeout-and-durable-audience/project.md:131-138 and captured in .bklg/from-contract-to-published-library/closeout-and-durable-audience/dod-set-re-observation-record/evidence/mode-audit.md"

- id: AC-004
  criterion: "GIVEN a reviewer applying discover.md:38's own test — diffing the fourteen artefact citations against the sibling ledgers they resemble — WHEN they check each row, THEN pointer and evidence sit in different columns (Pointer (sibling ledger) versus Artefact produced here), no path appears in both, no cell in Artefact produced here is any _ledger.md, every one of the fourteen artefact paths resolves on this tree, and each was produced or captured by this story's observation run rather than pre-dating it — so a row whose only citation is the producing project's memory cannot exist."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md"
  verifying_test: "Per row: artefact path exists, does not match *_ledger.md, and is either under .bklg/from-contract-to-published-library/closeout-and-durable-audience/dod-set-re-observation-record/evidence/ or appears in this story's commit range via `git log --diff-filter=AM --format=%H -- <path>`; empty set intersection of the two columns — captured in .bklg/from-contract-to-published-library/closeout-and-durable-audience/dod-set-re-observation-record/evidence/pointer-vs-evidence.md"

- id: AC-005
  criterion: "GIVEN a reader asking the question the word set exists to answer — were these fourteen observed on one tree? — WHEN they read the section header, THEN it carries one commit SHA, the observation date and a citation of the slice-1 `cargo xtask ci` run; that SHA equals the one whole-gate-green-on-the-assembled-tree recorded; every row is stated as observed at that SHA; and no row's Command run here cell contains a `cargo xtask ci` invocation at all — the rows the gate re-proves (DoD 4 in part, 7, 10 in part, 12) cite that run's step output instead of spawning a second run at a second SHA, and --fast appears nowhere in the section."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md"
  verifying_test: "Header SHA compared against the SHA recorded by .bklg/from-contract-to-published-library/closeout-and-durable-audience/whole-gate-green-on-the-assembled-tree/_ledger.md and its gate-run record; `git rev-parse HEAD` at observation recorded beside it; `rg 'cargo xtask ci'` matches citation prose only; `rg -- '--fast'` over the section returns nothing — captured in .bklg/from-contract-to-published-library/closeout-and-durable-audience/dod-set-re-observation-record/evidence/sha-anchor.md"

- id: AC-006
  criterion: "GIVEN the pressure project.md's risk table names (Pressure to fix what re-observation finds), WHEN a scenario does not come back green, THEN that row's Outcome names the failure plainly, a findings list beneath the table gives that finding a proposed destination drawn from DR-12's three (support per .redkiln/config.yaml:5, a new item against the named owning sibling, or a decision atom plus a re-plan where a [FROZEN] clause is touched), the disposition itself is left to findings-disposition-register, and this story's commits touch no file outside _closeout-record.md and its own story directory — zero repairs, and an all-green table states no findings explicitly rather than omitting the list."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md"
  verifying_test: "`git diff --name-only <base>..HEAD` is a subset of the PR-boundary fence (empty intersection with crates/, xtask/, spec/, examples/, .kb/); every non-green Outcome cell has a findings entry naming one of DR-12's three destinations, or the literal no-findings statement is present — captured in .bklg/from-contract-to-published-library/closeout-and-durable-audience/dod-set-re-observation-record/evidence/findings-and-boundary.md"

- id: AC-007
  criterion: "GIVEN the one-shot evaluator/closeout reader who cannot ask the author what a row meant, WHEN they pick any of the fourteen rows and try to follow it end to end, THEN the Command run here cell is a runnable invocation naming a real target on this tree, the Artefact produced here cell resolves to a file they can open, the Owning project cell resolves to a real sibling project directory, no cell exceeds roughly 200 characters (longer material moves to a numbered note beneath the table), and any captured transcript of 20 lines or more lives in a linked file under this story's directory rather than inline — so the table is readable in one sitting and no row requires asking its author what it meant."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md"
  verifying_test: "Follow-through pass over all fourteen rows: package names checked against `cargo metadata --no-deps --format-version 1`, xtask subcommands against the dispatch match at xtask/src/main.rs:671-681, every artefact path opened, every owning-project directory confirmed under .bklg/from-contract-to-published-library/, plus a cell-length and transcript-inlining scan — captured in .bklg/from-contract-to-published-library/closeout-and-durable-audience/dod-set-re-observation-record/evidence/follow-through.md"
```
