---
item: HS-S0032
stage: implement
created: 2026-08-12T13:46:29.793Z
updated: 2026-08-12T13:46:29.793Z
---

# Acceptance ledger — The contract defect log, and the happenstance-macros verdict

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

A note this story needs more than most: **both of its criteria are "Record, not a test"**
(`_decomposition.md:790-791`), so every gate step in this repository stays green on an empty
story. Exactly one AC — AC-007 — is caught by a machine, and it is caught by path on the diff.
This ledger is therefore the primary instrument, not a formality: the evidence cited in each row
is what stands between a real record and a document that merely exists.

```yaml
- id: AC-001
  criterion: >-
    GIVEN an adapter author about to pin `happenstance-core`, WHEN they open
    `references/evaluation/` to find out what the contract's first consumer discovered, THEN
    `phase-7-contract-defects.md` and `phase-7-macros-verdict.md` are there, each carrying its
    date and the commit it was written against, and `references/evaluation/README.md` gains a
    row for each under "Later additions, which are neither" — so the directory's own lifecycle
    taxonomy still covers everything in it rather than silently going false
  satisfied: false
  evidence: ""
  mount_point: "references/evaluation/ — the durable long-form home named in the Integration contract's Wires into"
  verifying_test: "test -f references/evaluation/phase-7-contract-defects.md && test -f references/evaluation/phase-7-macros-verdict.md && rg -n 'phase-7-contract-defects|phase-7-macros-verdict' references/evaluation/README.md"

- id: AC-002
  criterion: >-
    GIVEN P1 reading one entry to decide whether the defect touches the program they already
    shipped, WHEN they read any entry in the defect log, THEN it presents six labelled fields —
    id, clause ID with its maturity marker, what was attempted with the call site as
    `path:line`, what the contract did instead, why this is a contract defect and not a misuse,
    and the routing — as composed structure under its own headings, never a free-prose
    sentence; an entry missing the clause ID or the routing is not an entry and does not merge
  satisfied: false
  evidence: ""
  mount_point: "references/evaluation/phase-7-contract-defects.md — the entry shape itself, staged onward at .kb/_intake/contract-defect-log-phase-7.md"
  verifying_test: "rg -c '^Clause:' references/evaluation/phase-7-contract-defects.md equals the entry-heading count; closeout-gate review against project.md:243-244 (DoD 9)"

- id: AC-003
  criterion: >-
    GIVEN a reader who wants to know what a good entry looks like before writing their own,
    WHEN they read entry one, THEN it is D-1 verbatim from the signed-off design —
    "`happenstance-core` has no infallible `QueryItem` constructor for pre-validated inputs;
    every derived query therefore carries a `Result` that is unreachable for well-formed
    models" — naming clause VT-18 and its [FROZEN] marker, citing the call site
    `crates/happenstance-core/src/query.rs:48-62`, routed to a decision record, and stating
    plainly that the constructor may well be the right answer and the objection is to taking
    that decision without a record
  satisfied: false
  evidence: ""
  mount_point: "references/evaluation/phase-7-contract-defects.md — entry one"
  verifying_test: "rg -n 'VT-18' references/evaluation/phase-7-contract-defects.md; text comparison against .bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_design.md:652-672; sed -n '1371p;8544p' spec/SPECIFICATION.md still shows VT-18 and its [FROZEN] marker"

- id: AC-004
  criterion: >-
    GIVEN the upstream slice-mate that routed a finding here by name rather than fixing it,
    WHEN a reader looks for CF-36 in the log, THEN it is an entry with its clause ID, the
    contradiction stated concretely (CF-36 says `cargo xtask spec-trace` cross-references each
    case's level marker; `xtask/src/spec_trace.rs` reads no level marker), and its routing —
    and neither `xtask/src/spec_trace.rs` nor the frozen clause was edited to make the finding
    go away
  satisfied: false
  evidence: ""
  mount_point: "references/evaluation/phase-7-contract-defects.md — the CF-36 entry; xtask/src/spec_trace.rs is read, never written"
  verifying_test: "rg -n 'CF-36' references/evaluation/phase-7-contract-defects.md; git diff --name-only <base>...HEAD contains neither xtask/src/spec_trace.rs nor spec/SPECIFICATION.md"

- id: AC-005
  criterion: >-
    GIVEN a reader who cannot tell a story that found nothing from a story whose finding was
    dropped, WHEN they read the log's reconciliation table, THEN every M2–M6 story that
    declared a routing into this log appears as a row with an explicit disposition — an entry
    id, or "found none" stated as a claim by that story — and no declared routing is absent
  satisfied: false
  evidence: ""
  mount_point: "references/evaluation/phase-7-contract-defects.md — the reconciliation table"
  verifying_test: "rg -n 'defect log|phase-7-contract-defects' .bklg/from-contract-to-published-library/typed-layer-and-alpha-release/*/spec.md — every slug returned must appear as a row in the reconciliation table"

- id: AC-006
  criterion: >-
    GIVEN a bug found mid-implementation that bears on no clause, WHEN the implementer
    classifies it, THEN the classification is made at the moment of the finding, the log
    records the finding, its support classification and the fact that it was handed to the
    `support` initiative — and this PR neither invents a clause ID to promote it into an entry
    nor writes anything under `.bklg/support/**`
  satisfied: false
  evidence: ""
  mount_point: "references/evaluation/phase-7-contract-defects.md — the support-bound findings section; .bklg/support/ is the declared hand-off destination and stays outside this PR"
  verifying_test: "the log carries a support-bound section (empty is stated, not omitted); git diff --name-only <base>...HEAD returns no path under .bklg/support/"

- id: AC-007
  criterion: >-
    GIVEN the one commit that would destroy this story's reason to exist — absorbing a defect
    with a four-line convenience edit and going green — WHEN the story's gate runs, THEN it
    fails on the paths in the diff, before anyone has to notice what the edit meant: no file
    under `crates/**` and no file under `spec/**` is changed by this PR
  satisfied: false
  evidence: ""
  mount_point: "the PR-boundary fence in this story's spec.md, read by redkiln verify --grain story"
  verifying_test: "redkiln verify --grain story; independently git diff --name-only <base>...HEAD | rg '^(crates|spec)/' returns nothing"

- id: AC-008
  criterion: >-
    GIVEN a reader who wants to disagree with the macros verdict rather than take it on trust,
    WHEN they read `phase-7-macros-verdict.md`, THEN they find the counting method stated, a
    classification published line range by line range over
    `examples/course-subscriptions/src/main.rs` as M6 leaves it — ceremony (what a derive would
    emit: `EVENT_TYPES`, `event_type()`, `tags()`, encode/decode plumbing, the
    `assert_domain_event` residual) versus domain (variants and payloads, model state,
    `apply`'s arms, `scope`, the decision body) versus neither (`main`'s I/O, transcript
    printing, store construction, imports) — the ranges partitioning the file's visible source
    lines with no overlap and no unclassified line, both altitudes reported (the doctest's
    11:26 as the recorded prior, the example's count as the criterion, each labelled), and the
    verdict stated as in or out against the 1:1 threshold with the design's 2.4:1 prediction
    named as confirmed or contradicted
  satisfied: false
  evidence: ""
  mount_point: "references/evaluation/phase-7-macros-verdict.md, staged onward at .kb/_intake/happenstance-macros-verdict.md; measured over examples/course-subscriptions/src/main.rs (read, never written)"
  verifying_test: "partition check — the published ranges sum to wc -l examples/course-subscriptions/src/main.rs less the 'neither' bucket at the pinned commit, with no overlap and no unclassified line; a second reader re-running the stated method reaches the same two integers"

- id: AC-009
  criterion: >-
    GIVEN a maintainer reading the plan of record to find out whether phase 7's macros question
    was answered, WHEN they open `RUNBOOK.md`, THEN the verdict is where the runbook itself
    says it is written — one phase-7 session-log entry citing
    `references/evaluation/phase-7-macros-verdict.md` by path, the macros exit box ticked, and
    the decision-table row moved off `open` to the verdict — the edit touching nothing else in
    phase 7, and every `file:line` citation into `RUNBOOK.md` from elsewhere in the repository
    still resolving to the text it named
  satisfied: false
  evidence: ""
  mount_point: "RUNBOOK.md — the second mount named in the Integration contract: phase 7's session log, the macros exit box, and the decision-table row"
  verifying_test: "rg -n 'phase-7-macros-verdict' RUNBOOK.md; git diff --stat RUNBOOK.md shows only three hunks; citation sweep rg -no 'RUNBOOK\\.md:[0-9]+' .kb spec references standards docs CLAUDE.md with each cited line re-read for the text it was cited for"

- id: AC-010
  criterion: >-
    GIVEN the human who will run `/redkiln:kb-ingest` next, WHEN they look at `.kb/_intake/`,
    THEN both records are staged there under names distinct from M1's wave —
    `contract-defect-log-phase-7.md` and `happenstance-macros-verdict.md` — each carrying
    proposed frontmatter for the wave to author from and `source_paths` naming its long-form
    record, and this PR has authored nothing under `.kb/decisions/**`, `.kb/open-questions/**`
    or `.kb/maps/**` and has run no ingest
  satisfied: false
  evidence: ""
  mount_point: ".kb/_intake/ — the KB's only input path to /redkiln:kb-ingest and this story's primary mount point"
  verifying_test: "test -f .kb/_intake/contract-defect-log-phase-7.md && test -f .kb/_intake/happenstance-macros-verdict.md; rg -n 'source_paths' .kb/_intake/*.md names both references/evaluation/ records; git diff --name-only <base>...HEAD | rg '^\\.kb/(decisions|open-questions|maps)/' returns nothing; redkiln validate --kb && redkiln doctor stay clean"
```
