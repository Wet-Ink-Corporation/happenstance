---
item: HS-S0132
stage: implement
created: "2026-08-12"
updated: "2026-08-12"
---

# Acceptance ledger — The audience promoted into .kb/product/ through the ingest path

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Two notes specific to this story. This story writes **no code**, so every `verifying_test` is a
command whose verbatim output lands at a real path — the project's `testing` brief makes *static*
and *process* first-class tiers here, and calls AC-008's ingest-path check *E2E (process)*
(`_decomposition.md` *Testing brief → Notes → Test mix*). And the `mount_point` for every row is
`.kb/maps/domain-map.md`: an atom under `.kb/product/` that no index lists is reachable only by
someone who already knows its filename, which is the definition of unmounted.

```yaml
- id: AC-001
  criterion: >-
    GIVEN a planner opening the next initiative who has never read this initiative's _discovery/,
    WHEN they list .kb/product/ on the closeout tree, THEN they find seven atoms — three kind:
    concept personas and four kind: playbook journeys, every one status: accepted and
    authority_tier: product — and `redkiln validate --kb` exits zero over the corpus containing
    them, so what they inherit is a checked layer and not a folder of drafts.
  satisfied: false
  evidence: ""
  mount_point: ".kb/maps/domain-map.md"
  verifying_test: >-
    `redkiln validate --kb` on the merged closeout tree (exit code + verdict verbatim) and
    `rg -n "^(id|kind|status|authority_tier):" .kb/product`, both transcribed into
    .bklg/from-contract-to-published-library/closeout-and-durable-audience/product-atom-promotion-via-kb-ingest/_promotion-record.md
    §Validation and §Atoms

- id: AC-002
  criterion: >-
    GIVEN the repository owner's amendment of 2026-08-12, which held that the evaluator's mechanism
    of trust-building earns a first-class atom while the evaluator remains the same human as the
    application author, WHEN a reader looks for the evaluation path, THEN they find it as a journey
    atom of its own whose `related` names the application-author persona atom — not a fourth
    persona, and not a stage folded inside "Choose a contract before a database" — and the persona
    set is exactly the application author, the adapter author and the local-first / edge developer.
  satisfied: false
  evidence: ""
  mount_point: ".kb/maps/domain-map.md"
  verifying_test: >-
    Enumeration table (file → kind → the charter journey it realises, initiative.md:241-250) in
    .bklg/from-contract-to-published-library/closeout-and-durable-audience/product-atom-promotion-via-kb-ingest/_promotion-record.md
    §Set, checked against _decomposition.md *The evaluator-persona decision* amendment; plus
    `redkiln validate --kb` resolving the evaluation journey's `related` edge to the
    application-author persona id

- id: AC-003
  criterion: >-
    GIVEN a reviewer who does not trust a correctly-shaped .kb/ tree on sight, because 0269720 was
    exactly that and was reverted, WHEN they run
    `git log --diff-filter=A --format='%h %s' <base>..HEAD -- .kb/product/`, THEN every one of the
    seven atoms is attributed to a single /redkiln:kb-ingest wave commit, and no other commit in the
    range adds or modifies a file under .kb/product/ — the provenance is the deliverable, not the
    file shapes.
  satisfied: false
  evidence: ""
  mount_point: ".kb/maps/domain-map.md"
  verifying_test: >-
    `git log --diff-filter=A --format='%h %s' <base>..HEAD -- .kb/product/` and the same range
    unfiltered, with the `git merge --no-ff worktree-kb-intake-<date>` command used, all recorded in
    .bklg/from-contract-to-published-library/closeout-and-durable-audience/product-atom-promotion-via-kb-ingest/_promotion-record.md
    §Provenance; tied to the work commit by .redkiln/config.yaml:73 (require_commit_provenance)

- id: AC-004
  criterion: >-
    GIVEN the _intake contract that "a file still sitting here after a run is a file that run did not
    ingest" (.kb/_intake/README.md:13-19), WHEN the reviewer lists .kb/_intake/ after the merge, THEN
    it contains README.md and nothing else — the staging README survived the narrowed glob and was
    neither ingested as content nor deleted by the clear — and each of the seven atoms cites the
    .kb/_intake/… draft path it consumed in source_paths, so the drafts are recoverable from history
    rather than merely gone.
  satisfied: false
  evidence: ""
  mount_point: ".kb/maps/domain-map.md"
  verifying_test: >-
    `git ls-files .kb/_intake/` on the merged tree (exactly one path) and `rg -n "_intake" .kb/product`
    (a staged-source citation in all seven), with the Stage-A resolved input list pasted into
    .bklg/from-contract-to-published-library/closeout-and-durable-audience/product-atom-promotion-via-kb-ingest/_promotion-record.md
    §Wave inputs

- id: AC-005
  criterion: >-
    GIVEN a reader who reads only frontmatter — the failure mode DR-9 was written against — WHEN they
    read any one of the seven summary fields, THEN that atom tells them its evidence is secondary and
    that no persona was directly observed, and its source_paths names the discovery artefacts it rests
    on; the sentence is true of that atom (no summary asserts a four-persona set), the disclaimer is
    the same strength across all seven including the evaluation journey, and it is not in the body
    only.
  satisfied: false
  evidence: ""
  mount_point: ".kb/maps/domain-map.md"
  verifying_test: >-
    `rg -n "^summary:" -A 6 .kb/product/*.md` with all seven summaries quoted and each checked for the
    secondary-evidence statement, the not-directly-observed statement and the absence of any
    "all four personas" claim, plus `rg -n "source_paths:" -A 4 .kb/product`, in
    .bklg/from-contract-to-published-library/closeout-and-durable-audience/product-atom-promotion-via-kb-ingest/_promotion-record.md
    §Qualification

- id: AC-006
  criterion: >-
    GIVEN a reader who does not already know the filenames — the only reader an index exists for —
    WHEN they open .kb/maps/domain-map.md, THEN a new ## audience domain section lists all seven
    atoms with id and a relative link that resolves, appended rather than folded into an existing
    section per the map's own *Adding a domain* rule (:144-150), and the reciprocal `related` edges
    resolve in both directions. An atom present in .kb/product/ and absent here is
    constructed-but-unmounted and fails this AC.
  satisfied: false
  evidence: ""
  mount_point: ".kb/maps/domain-map.md"
  verifying_test: >-
    Diff of .kb/maps/domain-map.md inside the wave commit showing an appended ## section, a
    `git ls-files` link-resolution pass over that section's seven relative links, and
    `redkiln validate --kb` for the reciprocal ids — recorded in
    .bklg/from-contract-to-published-library/closeout-and-durable-audience/product-atom-promotion-via-kb-ingest/_promotion-record.md
    §Mount

- id: AC-007
  criterion: >-
    GIVEN an auditor at closeout who reads wave records to reconstruct how the corpus grew, WHEN they
    list .kb/_governance/integration-waves/, THEN a third directory under an id distinct from
    2026-08-10-intake and 2026-08-10-intake-2 holds this wave's five-file record in the shape the
    previous wave produced, and the two existing wave directories are byte-unchanged — a reused id
    would silently overwrite the audit trail AC-003 reads.
  satisfied: false
  evidence: ""
  mount_point: ".kb/maps/domain-map.md"
  verifying_test: >-
    `git diff --stat <base>..HEAD -- .kb/_governance/integration-waves/2026-08-10-intake
    .kb/_governance/integration-waves/2026-08-10-intake-2` (must be empty) and an `ls` of the new wave
    directory showing 00-corpus-match.md … 04-retrospective.md, both in
    .bklg/from-contract-to-published-library/closeout-and-durable-audience/product-atom-promotion-via-kb-ingest/_promotion-record.md
    §Wave record

- id: AC-008
  criterion: >-
    GIVEN the two prohibitions this project is most likely to breach under time pressure —
    hand-completing a partial wave, and editing an accepted decision — WHEN the reviewer reads the
    whole PR diff, THEN the only changes outside .kb/product/ and the wave record are the domain-map
    section and additive `related:` backlinks, no accepted decision atom's frozen signature (title,
    kind, summary, authority_tier, depends_on, adr_id, reversibility, phase, body) has moved, and
    every defect the wave surfaced appears in _promotion-record.md §Findings routed to
    findings-disposition-register with zero fixed here.
  satisfied: false
  evidence: ""
  mount_point: ".kb/maps/domain-map.md"
  verifying_test: >-
    `git diff <base>..HEAD -- .kb/decisions/` reviewed line by line (only `related:` entries may
    appear) and `git diff --stat <base>..HEAD` with every out-of-boundary path named and justified,
    plus the findings list with one destination per row, all in
    .bklg/from-contract-to-published-library/closeout-and-durable-audience/product-atom-promotion-via-kb-ingest/_promotion-record.md
    §Diff review and §Findings
```
