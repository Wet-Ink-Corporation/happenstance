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
  satisfied: true
  evidence: 'Both records exist: references/evaluation/phase-7-contract-defects.md (337 lines) and references/evaluation/phase-7-macros-verdict.md (265 lines). Each carries its date and pin in its first ten lines - `- **Date:** 2026-08-16` and `- **Pinned to:** 78a2170c1d06bad5eec34915b0b3682f524ec91f` at lines :3-4 of both - and `git cat-file -e 78a2170` resolves (it is this branch''s HEAD at the moment the records were written). references/evaluation/README.md gains a paragraph for each under its `## Later additions, which are neither` section (:165-201), each naming the lifecycle it carries, so the directory''s own three-way taxonomy still covers everything in it. `rg -n ''phase-7-contract-defects|phase-7-macros-verdict'' references/evaluation/README.md` returns four hits, two per document (the link and the body reference).'
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
  satisfied: true
  evidence: 'Structural read, re-runnable: `rg -c ''^### D-''` returns 5 and `rg -c ''^\*\*Clause:\*\*''` returns 5 - the count of `Clause:` lines equals the count of entry headings. Each of the other five field labels also returns exactly 5: `^\*\*Attempted\.\*\*`, `^\*\*Contract\.\*\*`, `^\*\*Why a defect`, `^\*\*Routing\.\*\*`, plus the id carried by the `### D-N` heading itself. Every field is composed structure under its own label, never a free-prose sentence, and the shape is declared up front in the `## How to read an entry` table (references/evaluation/phase-7-contract-defects.md:27-44) which states in terms that an entry missing the clause ID or the routing is not an entry. Density held: every entry is <= 40 lines and prose wraps at 80 columns, both checked mechanically; the single 82-column line is an unbreakable path token, exempt for the same reason a URL is.'
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
  satisfied: true
  evidence: 'Entry one is references/evaluation/phase-7-contract-defects.md:50-92. It carries D-1 from the signed-off design verbatim in substance - `happenstance-core` has no infallible `QueryItem` constructor for pre-validated inputs, so every derived query carries a `Result` unreachable for a well-formed model - names clause **VT-18** with its **[FROZEN]** marker and both citations, and cites the call site `crates/happenstance-core/src/query.rs:48-62`. Cited lines re-resolved at the pin: `sed -n ''1374p'' spec/SPECIFICATION.md` is VT-18''s heading, `sed -n ''8858p''` is its FROZEN row, and `sed -n ''48,62p'' crates/happenstance-core/src/query.rs` still shows the fallible `QueryItem::new` and its `# Errors` doc. The `**Proposed fix, subordinate to the routing.**` paragraph states plainly that the constructor *may well be the right answer* and that the objection is to taking that decision without a record. The entry is strengthened, not merely copied: `decision-model-composition` found that `Boundary`''s seal makes the error arm untestable from outside the crate, and that is folded in.'
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
  satisfied: true
  evidence: 'references/evaluation/phase-7-contract-defects.md:137-176, entry D-3. Names **CF-36** with its **[FROZEN]** marker (`spec/SPECIFICATION.md:8611-8622`, table row `:9062`) and states the contradiction concretely: CF-36''s `Rule:` line claims `cargo xtask spec-trace` cross-references each case''s level marker, and `grep -c "Level" xtask/src/spec_trace.rs` returns 0. Routed to a decision record. **Neither was edited to make it go away**: `git diff --name-only 78a2170..HEAD` for this story''s commit contains no path under `xtask/` and none under `spec/`. Cross-checked against projection-clause-verdicts/implementation-report.md:117-126, which routed it here by name.'
  mount_point: "references/evaluation/phase-7-contract-defects.md — the CF-36 entry; xtask/src/spec_trace.rs is read, never written"
  verifying_test: "rg -n 'CF-36' references/evaluation/phase-7-contract-defects.md; git diff --name-only <base>...HEAD contains neither xtask/src/spec_trace.rs nor spec/SPECIFICATION.md"

- id: AC-005
  criterion: >-
    GIVEN a reader who cannot tell a story that found nothing from a story whose finding was
    dropped, WHEN they read the log's reconciliation table, THEN every M2–M6 story that
    declared a routing into this log appears as a row with an explicit disposition — an entry
    id, or "found none" stated as a claim by that story — and no declared routing is absent
  satisfied: true
  evidence: 'references/evaluation/phase-7-contract-defects.md:283-303, the `## Reconciliation` table. The sweep was re-run at the pinned commit rather than copied from the spec''s planning-time list - `rg -l ''defect log|AC-012'' .bklg/from-contract-to-published-library/typed-layer-and-alpha-release/*/spec.md` returned fifteen files - and the table carries **fourteen** rows, one per story, each with an explicit disposition: an entry id (D-1 … D-5) or *found none* stated as that story''s own claim with the report line that states it. Five of the fourteen are `found none` and each cites where the story said so, e.g. misbehaving-testkit-stores at implementation-report.md:162-163 and codec-and-feature-forwarding at report.md:61. `compile-fail-proof-artefact` is listed with `declares no routing` so the sweep''s own coverage is checkable. No declared routing is absent.'
  mount_point: "references/evaluation/phase-7-contract-defects.md — the reconciliation table"
  verifying_test: "rg -n 'defect log|phase-7-contract-defects' .bklg/from-contract-to-published-library/typed-layer-and-alpha-release/*/spec.md — every slug returned must appear as a row in the reconciliation table"

- id: AC-006
  criterion: >-
    GIVEN a bug found mid-implementation that bears on no clause, WHEN the implementer
    classifies it, THEN the classification is made at the moment of the finding, the log
    records the finding, its support classification and the fact that it was handed to the
    `support` initiative — and this PR neither invents a clause ID to promote it into an entry
    nor writes anything under `.bklg/support/**`
  satisfied: true
  evidence: 'references/evaluation/phase-7-contract-defects.md:247-281, the `## Findings that are **not** entries` section - present rather than omitted, and it is not empty. **N-2** is the support-bound finding: `read_through` is dead code in eight `wasm32` feature combinations (crates/happenstance-core/src/projection_memory.rs:233), it bears on no clause, and it is classified **support** in the entry itself, citing `.redkiln/config.yaml:5`''s `support_initiative: support` and project.md:270. **No clause ID was invented to promote it into an entry** - the section says so in terms. **N-1** is the other half of the same discipline in the other direction: a finding that looks like a defect and is not, because ES-6 [FROZEN] and ADR-0009 already answer it, recorded so its absence from the entries is not read as a gap. Nothing was written under `.bklg/support/`: `git diff --name-only` for this commit contains no such path, and the log states that the `redkiln new` hand-off is owed.'
  mount_point: "references/evaluation/phase-7-contract-defects.md — the support-bound findings section; .bklg/support/ is the declared hand-off destination and stays outside this PR"
  verifying_test: "the log carries a support-bound section (empty is stated, not omitted); git diff --name-only <base>...HEAD returns no path under .bklg/support/"

- id: AC-007
  criterion: >-
    GIVEN the one commit that would destroy this story's reason to exist — absorbing a defect
    with a four-line convenience edit and going green — WHEN the story's gate runs, THEN it
    fails on the paths in the diff, before anyone has to notice what the edit meant: no file
    under `crates/**` and no file under `spec/**` is changed by this PR
  satisfied: true
  evidence: 'The absorption mutant fails **on the paths in the diff**, before anyone reads the edit. `git diff --name-only` for this story''s checkpoint lists exactly: RUNBOOK.md, references/evaluation/README.md, references/evaluation/phase-7-contract-defects.md, references/evaluation/phase-7-macros-verdict.md, .kb/_intake/contract-defect-log-phase-7.md, .kb/_intake/happenstance-macros-verdict.md, and this story''s own `.bklg` folder. Filtered: `| rg ''^(crates|spec)/''` returns **nothing**, and `| rg ''\.rs$|Cargo\.toml$''` returns **nothing** (NF-001). The temptation was real and named - D-1''s proposed fix is a four-line `QueryItem::from_validated` that would have gone green - and it is recorded inside the entry instead of applied.'
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
  satisfied: true
  evidence: 'references/evaluation/phase-7-macros-verdict.md, written in evidence order (NF-006): the prediction (:24-50), the counting method (:54-101), the count (:105-160), the verdict (:164 onward). The classification is published **line range by line range** - 29 rows over examples/course-subscriptions/src/main.rs at the pin - and the totals are derived by summing the rows rather than asserted beside them. **Partition check, re-derived mechanically from the published table rather than trusted**: the ranges are contiguous from 1 with no gap or overlap, every row''s stated `n` equals `end - start + 1`, and the sum is 532, which equals `wc -l` on the subject. Buckets: ceremony 40, domain 249, neither 158, contested 85. Both extremes reported (EC-007): 125:249 = 0.50:1 and 40:334 = 0.12:1, **both out** against the 1:1 threshold, so the contested block is a footnote rather than the decision. Both altitudes are labelled and reported side by side at :247-258 - the doctest''s 26:11 = 2.36:1 as the **prior**, the example''s as the **criterion** - and the design''s 2.4:1 prediction is named **contradicted**, with the reason the two disagree (the `DomainEvent` impl is a fixed cost; the domain is not). Substrate validity checked before counting per EC-005: `rg -c ''macro_rules!''` on the example returns 0.'
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
  satisfied: true
  evidence: 'Three hunks in RUNBOOK.md and nothing else. (1) The decision-table row, located by its `Is happenstance-macros in scope for 0.1` cell rather than by number, at RUNBOOK.md:525 - no longer `open`, now carrying the verdict, the two integers and the citation. (2) The exit box, located by its sentence, at RUNBOOK.md:4079-4083 - now `[x]` and naming the record. (3) The phase-7 session log, located by its `**Session log**` heading, at RUNBOOK.md:4109 - previously empty, now one dated entry citing references/evaluation/phase-7-macros-verdict.md and references/evaluation/phase-7-contract-defects.md by path. `rg -n ''phase-7-macros-verdict'' RUNBOOK.md` returns the session-log entry. `git diff --stat RUNBOOK.md` shows one file and only those three regions; every other phase-7 exit box is byte-identical. **Citation-drift sweep run (EC-008 / the review-citation-drift.md 1 failure class):** `rg -no ''RUNBOOK\.md:[0-9]+'' .kb spec references standards docs CLAUDE.md` - the highest line any durable-tree citation names is **3953**, and the first of my three edits is at 4079, so no citation in the swept set drifted. The `:525` edit is one line replaced by one line and shifts nothing. Recorded honestly: citations under `.bklg/` from OTHER projects'' planning artefacts (from 4104 upward) do shift by the session log''s insertion; they are work-in-motion artefacts outside this story''s PR fence, and re-anchoring them would fail the fence it is here to respect.'
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
  satisfied: true
  evidence: 'Both staged: `.kb/_intake/contract-defect-log-phase-7.md` (134 lines) and `.kb/_intake/happenstance-macros-verdict.md` (117 lines), under names distinct from M1''s wave, which is still present and untouched (`0031-adr-0021-serde-attribution-correction.md`, `0032-adr-0031-the-runner-collapses-upward.md`) - EC-009 honoured, nothing overwritten and nothing cleared. Each carries a **proposed** frontmatter block, explicitly labelled `# PROPOSED - for the ingest wave to adjudicate, not to copy verbatim`, with `source_paths` naming its long-form record: `rg -n ''source_paths'' .kb/_intake/*.md` shows references/evaluation/phase-7-contract-defects.md and references/evaluation/phase-7-macros-verdict.md respectively. Nothing was authored under `.kb/decisions/`, `.kb/open-questions/` or `.kb/maps/` - the diff contains no such path - and no ingest was run. The macros document opens with `## The one thing the wave must not do`: **do not edit ADR-0020''s atom**, because it is accepted and therefore immutable and its own spec assigned this verdict elsewhere. `redkiln validate --kb` reports `validate passed`; `redkiln doctor` reports exactly the six expected template-drift advisories and no new class.'
  mount_point: ".kb/_intake/ — the KB's only input path to /redkiln:kb-ingest and this story's primary mount point"
  verifying_test: "test -f .kb/_intake/contract-defect-log-phase-7.md && test -f .kb/_intake/happenstance-macros-verdict.md; rg -n 'source_paths' .kb/_intake/*.md names both references/evaluation/ records; git diff --name-only <base>...HEAD | rg '^\\.kb/(decisions|open-questions|maps)/' returns nothing; redkiln validate --kb && redkiln doctor stay clean"
```
