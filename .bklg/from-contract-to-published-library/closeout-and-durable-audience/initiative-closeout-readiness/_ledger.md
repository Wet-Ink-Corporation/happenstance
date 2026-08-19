---
item: "HS-S0135"
stage: implement
created: "2026-08-12"
updated: "2026-08-12"
---

# Acceptance ledger — No open child of HS-I0006, and the closeout links the promoted atoms

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
  criterion: "GIVEN the repository owner is deciding whether to authorise /redkiln:closeout on HS-I0006, WHEN they open _closeout-record.md and read the `## Closeout readiness (AC-014)` section — appended in place beneath the findings register's section, with no earlier section rewritten, reordered or truncated and no private artefact created beside the record — THEN they meet a child census of exactly ten rows, one per item with parent: HS-I0006 (HS-P0010…HS-P0019), each carrying id, slug, status, stage and path verbatim from one `redkiln status --json` run whose index.branch, index.builtAt and tree commit SHA are quoted above the table, with HS-P0010…HS-P0018 each reading status: done AND stage: closeout; a count other than ten, or a sibling at review with an approved verdict but no closeout advance, is reported as a named residual rather than rounded off."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md → `## Closeout readiness (AC-014)` → *Census provenance* + *Child census*"
  verifying_test: "`redkiln status --json` capture quoted in `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md`, row set compared against `.bklg/from-contract-to-published-library/_decomposition.md:25-34` and the closure predicate at `.redkiln/processes/project.yaml:71-80`"

- id: AC-002
  criterion: "GIVEN the same authoriser knows the closeout preflight audits story grain as well as project grain, and that a project marked closed above a story still at report is the inconsistency that makes a preflight fail after they have already said yes, WHEN they read on, THEN a *Story census* gives, per sibling project HS-P0010…HS-P0018, a count of stories at status: done / stage: closeout over that project's total, plus an itemised list of every story not there with its id, owning project and current stage — nothing netted out into a single aggregate number."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md → `## Closeout readiness (AC-014)` → *Story census*"
  verifying_test: "The same `redkiln status --json` capture filtered to type: story, tabulated in `_closeout-record.md`; terminal story stage checked against `.redkiln/processes/story.yaml:33-41`"

- id: AC-003
  criterion: "GIVEN AC-014's sentence (\"no open child of HS-I0006\") cannot be literally true while HS-P0019 — the project writing the record — is itself an open child, WHEN the authoriser looks for the catch, THEN the record has already stated it: HS-P0019 is named as open at the moment of writing, its own stories are listed with their stages, the remaining transitions are enumerated in order (this story's report → closeout, then the project's review → closeout), and the single advance after which the sentence becomes literally true is named and attributed to the orchestrating command — never performed here."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md → `## Closeout readiness (AC-014)` → *The declared residual*"
  verifying_test: "Transition order checked against `.redkiln/processes/story.yaml:33-41` and `.redkiln/processes/project.yaml:71-80`; non-performance evidenced by this story's work-commit range (recorded via `redkiln record-links --sha`) touching only the PR-boundary globs"

- id: AC-004
  criterion: "GIVEN the next initiative's planner will follow links.kb expecting an audience rather than a directory, WHEN the record's atom manifest is read, THEN it lists every file under .kb/product/ except README.md with its path and its frontmatter id, kind and authority_tier, and states observed-vs-expected as two computed counts against the amended evaluator decision — seven atoms, exactly three of them kind: concept personas (application author, adapter author, local-first/edge developer), the remaining four kind: playbook journeys including the evaluation path as a journey of its own, all authority_tier: product — and any mismatch produces a finding row naming product-atom-promotion-via-kb-ingest as owner, routed through the findings register, with zero atoms authored, edited, moved or renamed by this story."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md → `## Closeout readiness (AC-014)` → *Atom manifest*"
  verifying_test: "Directory listing and frontmatter of `.kb/product/*.md` quoted into `_closeout-record.md`, compared against `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_decomposition.md:227-242` and `.kb/product/README.md:6-9`; the no-authoring rule enforced by `redkiln verify --item HS-S0135 --grain story` (no `.kb/` path in the PR boundary)"

- id: AC-005
  criterion: "GIVEN DoD 16's second clause is a promise until links.kb on the initiative item is non-empty, and that the item's frontmatter may only be written by the CLI, WHEN the linkage is performed, THEN `redkiln record-links HS-I0006 --atom <comma-separated ids> --dry-run` ran first with its output captured and clean (writing nothing), the real invocation followed only on that clean dry run, .bklg/from-contract-to-published-library/initiative.md's links.kb was re-read after the write and quoted verbatim into the record with its members matched one-for-one against AC-004's manifest — never inferred from an exit code — and the record names `redkiln record-links HS-I0006 --atom <ids> --remove` as the reversal, so the one durable state change this story causes is undoable without a hand edit."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md → `## Closeout readiness (AC-014)` → *Link manifest*; and `links.kb` on `.bklg/from-contract-to-published-library/initiative.md:21-24`"
  verifying_test: "Dry-run and real-run transcripts of `redkiln record-links HS-I0006 --atom <ids>` captured in `_closeout-record.md`, followed by a post-write re-read of `.bklg/from-contract-to-published-library/initiative.md:21-24` quoted verbatim and matched against the atom manifest; flag semantics per `redkiln record-links --help`"

- id: AC-006
  criterion: "GIVEN the authoriser must know which halves of the initiative's exit criteria this story can speak for and which belong to a sibling or to the closeout command itself, WHEN they read the answer sheet, THEN three rows — DoD 16, exit criterion 7, exit criterion 8 — each split the criterion into its clauses and name an owner per clause (DoD 16 clause 1 → product-atom-promotion-via-kb-ingest, clause 2 → this story; exit criterion 7's validate --kb / doctor half → backlog-and-kb-health-at-closeout; exit criterion 8's whole-gate half → whole-gate-green-on-the-assembled-tree and published-tree-delta-statement, its harvest half → /redkiln:closeout's closure and retrospective stages), every cell citing an artefact path, and the slice-mates' commands linked rather than re-run so no question gets a second, separately-timed answer."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md → `## Closeout readiness (AC-014)` → *Exit-criteria answer sheet*"
  verifying_test: "Answer sheet reviewed against `.bklg/from-contract-to-published-library/initiative.md:405-407` and `:578-581`, stage ownership at `.redkiln/processes/initiative.yaml:32-41`, and the six-file template-drift set at `.github/workflows/ci.yml:177-186` cited to `backlog-and-kb-health-at-closeout` rather than re-asserted"

- id: AC-007
  criterion: "GIVEN a verdict that can only come out green is not a verdict, WHEN the authoriser reaches the end of the record — this section last, after every other story's — THEN they find READY or NOT-READY, a one-line reason, and when NOT-READY a residual table of exactly three columns (the residual, the item that owns it, where it is routed per DR-12: support, a new item against the owning sibling, or a decision atom plus a re-plan); and the record states as observations, not promises, that no `redkiln advance`, no `redkiln new` and no `redkiln adopt --templates` was run, evidenced by the story's work-commit range touching only the three PR-boundary globs. A NOT-READY verdict ships and merges unchanged; lowering a predicate, closing a sibling or authoring an atom to reach READY is a failed story."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md → `## Closeout readiness (AC-014)` → *Verdict*"
  verifying_test: "Verdict and residual table reviewed against `.bklg/from-contract-to-published-library/closeout-and-durable-audience/project.md:179-183` (DR-12) and `.redkiln/config.yaml:5`; `redkiln verify --item HS-S0135 --grain story` exits zero over the PR boundary and commit provenance (`.redkiln/config.yaml:69-73`, `.redkiln/processes/story.yaml:32`)"
```
