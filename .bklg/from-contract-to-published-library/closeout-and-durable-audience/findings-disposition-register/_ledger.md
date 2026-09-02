---
item: "HS-S0134"
stage: implement
created: "2026-08-12"
updated: "2026-08-12"
---

# Acceptance ledger — Every finding routed to an owner, none absorbed here

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
  criterion: >-
    GIVEN a reader who has just read five earlier sections of `_closeout-record.md` and wants to
    know whether the findings register covers all of them, WHEN they read the input sweep at the
    head of `## Findings and disposition (AC-013)`, THEN they find one line per input source — the
    six stories at `_storymap.md:62`, each named with the section or file that was read and the
    count of finding entries taken from it — and the register table below contains exactly one row
    per entry in that union, in both directions: an upstream entry with no row is an omission and a
    row naming no source entry is an invention. The sweep reports its own coverage rather than
    asserting completeness (`.kb/playbooks/verify-the-referent-and-report-coverage.md`)
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md § Findings and disposition (AC-013) → input sweep"
  verifying_test: "Bidirectional union check — each of the six sources named at .bklg/from-contract-to-published-library/closeout-and-durable-audience/_storymap.md:62 has its finding entries listed by id and set against the register's Surfaced by column, with the set difference shown in both directions and empty; a completeness claim with no per-source count fails (the coverage-reporting rule at .kb/playbooks/verify-the-referent-and-report-coverage.md)"

- id: AC-002
  criterion: >-
    GIVEN the future owner of a finding, arriving at a row with no memory of the project, WHEN they
    read it, THEN every one of the seven cells is filled — Register id / Finding (what was expected,
    what was observed) / Surfaced by (source story + that source's own finding id) / Lane (exactly
    one of DR-12's three, `project.md:179-183`) / Destination / State / Evidence (`file:line` into
    the source section) — and the row states the test by which its lane was chosen, so the routing
    is checkable rather than asserted. "TBD", "noted", "see above" and an empty cell each fail the
    row
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md § Findings and disposition (AC-013) → register table"
  verifying_test: "Row-shape and lane-test check plus citation-resolution sweep — zero empty or placeholder cells; every Lane cell carries one of DR-12's three literal lanes (.bklg/from-contract-to-published-library/closeout-and-durable-audience/project.md:179-183) plus its one-clause test; every Evidence file:line is opened and the described finding is present at that location, per .kb/playbooks/verify-the-referent-and-report-coverage.md"

- id: AC-003
  criterion: >-
    GIVEN the same owner, who now wants to act, WHEN they read Destination and State, THEN either
    the state is `opened` and the item id cited resolves to a file that exists on the tree, or the
    state is `pending` and the row carries the literal `redkiln` invocation that opens it with its
    parent named plus the human owner who will run it — and a lane-1 row names `support` / HS-I0005
    as it actually exists (`.bklg/support/initiative.md`; `.redkiln/config.yaml:5`), never a
    `support` project that is not there, while a lane-3 row names the clause id or accepted atom
    implicated, the commitment that would move, the owner of the new atom and that a re-plan is
    owed, and authors nothing under `.kb/`
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md § Findings and disposition (AC-013) → register table, Destination + State columns"
  verifying_test: "Destination-resolution sweep — every `opened` id resolved to an existing file cited by path; every `pending` row carrying a runnable redkiln invocation and a named owner; `git diff --stat -- .kb/ .bklg/support/ .bklg/from-contract-to-published-library/publication-and-positioning/` over this story's commit range is empty (.redkiln/config.yaml:5; .kb/decisions/README.md:7-23)"

- id: AC-004
  criterion: >-
    GIVEN a reader who cannot distinguish "nothing was found" from "nobody looked", WHEN they read
    the risk-shape sweep, THEN they find one line for each of the ten rows of `project.md:293-306` —
    the pre-registered list of shapes a finding was expected to take — each marked explicitly
    surfaced (with its register id) or did not surface (with the evidence that was checked and
    where), including the rows expected to be quiet (the `!Send` flavour, a seventh `template-drift`
    advisory) and DoD 13's delta, which appears as a finding only in the negative case its owner
    already bounded (`../published-tree-delta-statement/spec.md:215`)
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md § Findings and disposition (AC-013) → risk-shape sweep"
  verifying_test: "Pre-registered ten-row sweep against .bklg/from-contract-to-published-library/closeout-and-durable-audience/project.md:293-306 — row count equals ten, each mapping to a distinct risk row; every 'did not surface' line cites what was read (a section of _closeout-record.md, a command output, or a path); an uncited 'did not surface' fails"

- id: AC-005
  criterion: >-
    GIVEN the reader for whom "zero were fixed" must mean more than "no `.rs` file changed", WHEN
    they read the self-audit entry, THEN every decision taken inside this project's own stories that
    settled a disagreement owned elsewhere has a row, and the evaluator/DT-1 case in particular is
    observed on both sides at register time — this project's amendment
    (`../_decomposition.md:148-180`, `:227-242`) and `publication-and-positioning`'s own DT-1 rider
    (`../../publication-and-positioning/_design.md:221-243`) — recorded either as reconciled at
    planning, no item owed, with both citations, or as a lane-2 finding against HS-P0016. The row
    exists in either case; its absence, or a quiet edit to either side to make the question go away,
    is the failure
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md § Findings and disposition (AC-013) → self-audit"
  verifying_test: "Both-sides observation check — the operative sentence quoted with file:line from .bklg/from-contract-to-published-library/closeout-and-durable-audience/_decomposition.md:148-242 and from .bklg/from-contract-to-published-library/publication-and-positioning/_design.md:221-243, plus a stated verdict; `git diff --name-only` over the story's commit range contains neither path (the test at .kb/governance/rewrite-the-referent-never-the-reasoning.md; the wrong implementation at findings-disposition-register/discover.md:37-39)"

- id: AC-006
  criterion: >-
    GIVEN a reader who distrusts the claim rather than the author, WHEN they read the zero-fix
    proof, THEN they find the commit range covering all eleven HS-P0019 stories stated as full
    40-character SHAs, the literal command quoted, its output quoted rather than summarised, and the
    allow-list stated before the result — `.bklg/**`; the `.kb/_intake/` → `.kb/product/` and
    `.kb/maps/` writes `/redkiln:kb-ingest` legitimately produced (`project.md:215-221`); and the
    `.redkiln/telemetry/events/` footprint `auto_stage_telemetry` leaves
    (`.redkiln/config.yaml:12`) — such that they can re-run it themselves and get the same answer.
    Any path under `crates/`, `xtask/`, `spec/`, `standards/`, `examples/`, `references/` or
    `.github/`, and any hand-authored `.kb/` write outside the ingest path, is a fix whatever its
    commit message says
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md § Findings and disposition (AC-013) → zero-fix proof"
  verifying_test: "Re-derivable diff check — `git diff --name-only <project-base-sha>..<closeout-head-sha>` run at the stated 40-character SHAs, every output path classified against the stated allow-list by prefix, plus `git log --diff-filter=A --format=%H -- .kb/product/ .kb/maps/` cross-read against product-atom-promotion-via-kb-ingest's recorded ingest commit; an unstated allow-list, an abbreviated SHA, a summarised output or a story-only range fails (project.md:237-240, :215-221)"

- id: AC-007
  criterion: >-
    GIVEN a reader who opens the project's one closeout artefact, WHEN they reach the findings
    register, THEN they meet it composed in place in `_closeout-record.md` — a `## Findings and
    disposition (AC-013)` section carrying five subsections in the fixed order input sweep →
    register table → risk-shape sweep → self-audit → zero-fix proof, the table rendered with all
    seven columns, every sibling section already in that file left byte-identical, and — where
    nothing surfaced — an explicit swept-zero sentence naming the six sources read, the ten risk
    rows checked and the clean diff, rather than an absent table
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md § Findings and disposition (AC-013) — the whole section, appended in place"
  verifying_test: "Composition + non-occlusion check — `git diff -- .bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md` over the story's commit range shows additions only with every pre-existing line unchanged; the appended section carries five subsections in the fixed order, a seven-column table, and no cell reading 'I checked' or 'yes' (_storymap.md:24-30)"
```
