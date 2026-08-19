---
item: "HS-S0129"
stage: implement
created: "2026-08-12"
updated: "2026-08-12"
---

# Acceptance ledger — An accepted atom per settled answer, naming the alternatives that lost

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
    GIVEN a reader who knows only that the initiative "settled some things", WHEN they open the
    decision-atom audit in `_closeout-record.md`, THEN they find exactly one row for every ADR
    number in the union of the queue's `0008`-`0028` (`RUNBOOK.md:284-307`) and every `NNNN-*.md`
    on disk under `.kb/decisions/` — so a number that exists on disk but was never queued
    (ADR-0029, which the queue itself marks "(unscheduled — the queue had no number for it)") has
    a row, and a number the queue reserved but nobody wrote has a row too; no number appears twice
    and none is absent
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md § Decision-atom audit (AC-005) → audit table"
  verifying_test: "Union-completeness check — `ls .kb/decisions/*.md` × the queue rows at `RUNBOOK.md:284-307`, both listings quoted in evidence; set difference in either direction fails (the Static tier the testing brief names at `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_decomposition.md:48`)"

- id: AC-002
  criterion: >-
    GIVEN a reader deciding whether an answer actually stands, WHEN they read a row, THEN the row
    carries the four columns the testing brief fixed (`_decomposition.md:135-137` — Settled
    question / Atom / Status / Alternatives it records as rejected) plus the two additive ones
    (Evidence, Finding), and its Status is the value read from that atom's frontmatter on the
    audited tree, never assumed from the atom's existence — a `superseded` atom is reported as
    superseded with its `superseded_by`, not omitted and not silently counted as an answer
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md § Decision-atom audit (AC-005) → audit table"
  verifying_test: "Status-provenance check — every row's `status:`/`superseded_by:` cell matches the atom's own frontmatter line cited `file:line`, cross-read against `.kb/maps/decision-map.md`; `redkiln validate --kb` exits zero on the audited tree"

- id: AC-003
  criterion: >-
    GIVEN the reader who meets the same fork later and needs to know it was a fork
    (`.kb/decisions/README.md:31-33`), WHEN they read the Alternatives cell, THEN it names the
    options that lost as quoted from the atom's body and cites the `file:line` where the body
    states them — and an atom that is `accepted` but whose body records no rejected alternative
    yields an empty alternatives cell plus a finding id, never a blank cell, never a
    plausible-sounding alternative inferred from the atom's title, and never a repair
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md § Decision-atom audit (AC-005) → audit table, Alternatives + Evidence columns"
  verifying_test: "Citation-resolution sweep — every Evidence `file:line` in the table is opened and the named alternative is present at that location; every empty Alternatives cell carries a non-empty Finding (the check the wrong implementation at `.bklg/from-contract-to-published-library/closeout-and-durable-audience/decision-atom-audit-table/discover.md:38-40` fails)"

- id: AC-004
  criterion: >-
    GIVEN a reader who counts `0017`-`0028` missing from `.kb/decisions/` and wants to know whether
    that is a gap, WHEN they read the reserved-number disposition list, THEN every number unwritten
    at execution time carries either a stated reason it was not needed — following the ADR-0009
    precedent that "a reserved number that stays empty … is the cost of [the question] being
    genuinely open, not a scheduling defect" (`RUNBOOK.md:276-283`) — or, where its owning sibling
    project is closed out and the question it was reserved for was nonetheless answered somewhere
    with no atom, a finding id; and never a blank
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md § Decision-atom audit (AC-005) → reserved-number disposition"
  verifying_test: "Reserved-number disposition sweep — each unwritten number's row cites the owning sibling and the state that makes the emptiness correct (mapping read from `.bklg/from-contract-to-published-library/closeout-and-durable-audience/project.md:148-152` and `_decomposition.md:125-131`), or names a finding; neither = fail"

- id: AC-005
  criterion: >-
    GIVEN a reader who wants to know whether the audit is complete rather than merely internally
    consistent, WHEN they read the coverage statement, THEN they find a computation, not a claim —
    the union of what the written atoms actually discharge set against a re-derived list of what
    this initiative settled, drawn from all four enumerations (the queue `RUNBOOK.md:284-307`; the
    `.kb/decisions/` listing; the six consumed open questions at `project.md:153-161` cross-read
    against `.kb/maps/open-questions-index.md`; and DT-1…DT-8 at `initiative.md:420-427`) — with
    every settled question that has no atom appearing as a row with no atom plus a finding
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md § Decision-atom audit (AC-005) → coverage computation"
  verifying_test: "Coverage computation check — the section shows the four input lists and the set difference between them, with a per-DT line for DT-1…DT-8 stating where each was resolved and whether that resolution is a commitment `.kb/decisions/README.md:25-29` puts in the decisions layer; a completeness claim with no shown difference fails (the `RUNBOOK.md:309-320` precedent)"

- id: AC-006
  criterion: >-
    GIVEN the author of the slice-mate `open-question-preservation-audit`, working in the same
    context, WHEN they need "what this initiative settled" to check each consumed open-question
    atom, THEN they cite a named subsection with stable per-item labels in `_closeout-record.md`
    rather than re-deriving the list or lifting it out of the first table column — the enumeration
    is an artefact with an address, not a working list held in the auditor's head
    (`_storymap.md:58,78-80`)
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md § Decision-atom audit (AC-005) → settled-question enumeration (SQ-##)"
  verifying_test: "Consumer-citability check — the enumeration exists as its own heading with one labelled item per settled question, and `open-question-preservation-audit`'s record cites those `SQ-##` labels verbatim; a first-column-only enumeration fails"

- id: AC-007
  criterion: >-
    GIVEN a reader who opens the project's one closeout artefact, WHEN they reach the decision-atom
    audit, THEN they meet it composed in place in `_closeout-record.md` — a `## Decision-atom audit
    (AC-005)` section carrying its four subsections in the order enumeration → table →
    reserved-number disposition → coverage-and-findings, the table rendered with its full six
    columns, no cell reading "I checked" or "yes", and the prior sections slice 1 and slices 1-2
    wrote left byte-identical — rather than as a loose file in the story folder that the record
    merely mentions (`_storymap.md:26-30`)
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md § Decision-atom audit (AC-005) — the whole section, appended in place"
  verifying_test: "Composition + non-occlusion check — `git diff -- .bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md` over the story's commit range shows additions only with every pre-existing line unchanged; the appended section carries four subsections in the fixed order, a six-column table of no fewer than 29 rows, and a `file:line` in every claim cell"

- id: AC-008
  criterion: >-
    GIVEN the author of `findings-disposition-register` (AC-013), WHEN they pick this story's
    findings up, THEN each is listed with enough detail to route (what was expected, what was
    observed, the owning sibling or [FROZEN] clause it touches) and the audited surface is provably
    unmutated — `git diff --stat -- .kb/` over this story's commit range is empty, no atom body was
    edited, no status flipped, no missing ADR written, and `redkiln validate --kb` exits zero on the
    audited tree with its output cited
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md § Decision-atom audit (AC-005) → findings list (F-##), handed to findings-disposition-register"
  verifying_test: "Read-only + routability check — `git diff --stat -- .kb/` empty over the commit range; one findings entry per finding id referenced in the section, each with expected/observed/owner; `redkiln validate --kb` exit code and output recorded here (`.redkiln/config.yaml:67,73`)"
```
