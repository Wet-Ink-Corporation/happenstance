---
item: HS-S0178
stage: implement
created: "2026-08-17T13:16:29.278Z"
updated: "2026-08-17T13:16:29.278Z"
---

# Acceptance ledger — Run the single closeout ingest wave into .kb/product/

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Three notes this story's ledger needs that a generic one does not:

- **AC-001's evidence is the `files` array itself, recorded verbatim** — not a description of it.
  It is the artefact AC-A02 is reviewed on (`spec.md`, `## PR boundary`, "In this PR").
- **AC-006 and AC-007 take different evidence, and neither substitutes for the other.**
  `redkiln validate --kb` skips `_`-prefixed directories, so a green run is silent about
  `.kb/_intake/` (`.kb/_intake/README.md:21-27`).
- **AC-007's evidence carries the two recorded limits beside the green result** — the validator
  does not enforce the `authority_tier` vocabulary (`.kb/README.md:31-35`) and cannot see the
  intake directory. A future reader finding a bare green row will otherwise assume it covered
  more than it did.

The landed-atom handoff AC-009 requires — id, path, `kind`, `authority_tier` for every atom this
wave landed — is written into this file as AC-009's evidence, because
`product-layer-mounting` reads it from here.

```yaml
- id: AC-001
  criterion: >-
    GIVEN U2 at the ingest approval gate, whose only cheap veto is refusing the run before it
    commits, WHEN they read the /redkiln:kb-ingest invocation this story is about to make, THEN
    they see a structured argument whose `files` field is a non-empty array of repo-relative path
    strings that names every document from all three upstream inventories — staged-audience-payload's
    `persona-*.md` and `journey-*.md`, every deferral document listed in
    charter-open-question-disposition's handoff block (zero is a valid count, and zero must be
    stated as such rather than left ambiguous), and `.kb/_intake/lesson-page-need-declaration-discipline.md`
    — and does not name `.kb/_intake/README.md`, so the README is excluded by never being named
    rather than by a glob that could be defeated.
  satisfied: false
  evidence: "" # the files array recorded verbatim, plus the pre-run inventory cross-check
  mount_point: "/redkiln:kb-ingest over .kb/_intake/ — the composition root and single writer of .kb/product/"
  verifying_test: >-
    Tier 3 — read the invocation's `files` array before the run against
    .bklg/docs-that-teach/durable-audience-closeout/staged-audience-payload/spec.md (AC-008),
    .bklg/docs-that-teach/durable-audience-closeout/charter-open-question-disposition/spec.md
    (AC-006 handoff block) and
    .bklg/docs-that-teach/page-need-discipline/playbook-atom-staged-for-ingest/spec.md;
    Tier 1 backstop after the run — `ls .kb/_intake/`

- id: AC-002
  criterion: >-
    GIVEN U1 writing "GIVEN a <persona> <context>, WHEN they <action>, THEN <outcome>" for the next
    charter and unwilling to re-run discovery, WHEN they open one landed persona atom and no other
    file, THEN it is under `.kb/product/` (flat — no subdirectory), carries `kind: concept` and
    `authority_tier: product`, composes from `.kb/_templates/atom.md`'s own skeleton — one `#`,
    then `## Context` / `## Body` / `## Consequences / links`, with the ten required frontmatter
    keys present and `summary:` a folded block carrying the argument rather than a label — and
    yields all four slots the product README fixes (goal, context, what they already do instead,
    what they are afraid of) plus its observation-status word, each stated in place, with every
    link carrying evidence or provenance and never one of the four slots.
  satisfied: false
  evidence: "" # rg output for kind/authority_tier and the required keys, plus the Tier 2 read notes
  mount_point: ".kb/product/ — the layer written only by /redkiln:kb-ingest, governed by .kb/product/README.md:6-13"
  verifying_test: >-
    Tier 1 — `rg -n "^(kind|authority_tier):" .kb/product/`, `ls .kb/product/` (no subdirectory),
    and a required-key sweep per atom against .kb/README.md:16-26; Tier 2 primary — a human read of
    each persona atom alone against .kb/product/README.md:6-13 and
    .bklg/docs-that-teach/durable-audience-closeout/_decomposition.md, "## UX brief", AC-UX-01/AC-UX-04

- id: AC-003
  criterion: >-
    GIVEN U1 following a journey atom to see the moment-by-moment path, and U3 later asking what
    standing each atom carries, WHEN they list the landed set, THEN journey atoms sit under
    `.kb/product/` with `kind: playbook` and `authority_tier: product` — `kind` is not the directory
    — while HS-P0021's carried payload sits under `.kb/playbooks/` with `kind: playbook` and
    `authority_tier: guideline`, its body unedited by this story, so the wave lands two tiers
    deliberately and a uniform tier across the wave is visible as the defect it is.
  satisfied: false
  evidence: "" # rg output for both layers, plus the staged-source vs landed-body diff read
  mount_point: ".kb/product/ for the journey atoms; .kb/playbooks/ for HS-P0021's carried payload"
  verifying_test: >-
    Tier 1 — `rg -n "^(kind|authority_tier):" .kb/product/` shows no `guideline` under the product
    layer, `rg -n "^authority_tier:" .kb/playbooks/lesson-page-need-declaration-discipline.md` shows
    `guideline`, `ls .kb/playbooks/` shows no misfiled journey atom; Tier 2 — a diff read of the
    carried payload's staged source against its landed atom body (AC-A10, vehicle versus content)

- id: AC-004
  criterion: >-
    GIVEN U3 reconciling a third persona set two initiatives from now, needing to correct a claim
    without editing it and therefore needing to reach the material it was cut from, WHEN they open
    any atom this wave landed — after the same commit deleted every staged source — THEN its
    `source_paths` cites both its `.kb/_intake/…` staging path and at least one artefact under
    `.bklg/docs-that-teach/_discovery/`, and every cited path resolves in the merged tree, so the
    clearing step destroyed nothing and a charter that cited the distillation and a charter that
    cites the atom point at one audience rather than two.
  satisfied: false
  evidence: "" # the per-atom source_paths listing and the test -f sweep result over every entry
  mount_point: ".kb/product/, .kb/playbooks/ and .kb/open-questions/ — source_paths on every landed atom"
  verifying_test: >-
    Tier 1 — `rg -n "^source_paths:" -A 6` over every landed atom, then `test -f` over every entry
    (all exit zero); each atom has at least one `.kb/_intake/` entry and at least one
    `.bklg/docs-that-teach/_discovery/` entry, per the pattern at
    .kb/concepts/torn-reads-and-the-append-condition-boundary.md:23-28

- id: AC-005
  criterion: >-
    GIVEN U2 asking the one question this project's non-goal exists to make answerable — was any of
    this written by hand? — WHEN they walk the history of every `.kb/` path added in this project's
    diff, THEN each one's introducing commit is this wave's single commit, that same commit removes
    the staged source it consumed, and no `.kb/` file appears whose staged source is absent from the
    wave's history; a landed atom found wrong is corrected by fixing the staged input and re-running,
    never by editing the output.
  satisfied: false
  evidence: "" # the wave sha, git log --diff-filter=A per added path, and git show --stat output
  mount_point: "The wave's single commit on its own branch — additions and .kb/_intake/ deletions together"
  verifying_test: >-
    Tier 1 — `git log --diff-filter=A --format=%H -- <each added .kb path>` returns the wave commit,
    `git show --stat <wave sha>` shows additions and `.kb/_intake/` deletions in the same commit;
    Tier 3 — no `.kb/` path in `git diff --name-only main...HEAD` lies outside that commit's file list

- id: AC-006
  criterion: >-
    GIVEN U2 relying on the intake directory's own contract — that a file still sitting there is a
    file the run did not ingest — WHEN they list `.kb/_intake/` after the wave, THEN it contains
    `README.md` and nothing this wave consumed, `git diff -- .kb/_intake/README.md` is empty, no
    landed atom's `source_paths` names the README, and this observation is recorded separately from
    AC-007 because `redkiln validate --kb` skips every `_`-prefixed directory and its exit code is
    silent about this one.
  satisfied: false
  evidence: "" # ls output, the empty README diff, and the negative rg over the landed atoms
  mount_point: ".kb/_intake/ — the staging directory the wave clears in the same commit"
  verifying_test: >-
    Tier 1 and only Tier 1 — `ls .kb/_intake/`, `git diff -- .kb/_intake/README.md` (empty), and
    `rg -n "_intake/README" .kb/product/ .kb/playbooks/ .kb/open-questions/` (no hits).
    `redkiln validate --kb` must not be substituted (.kb/_intake/README.md:21-27)

- id: AC-007
  criterion: >-
    GIVEN U1 needing the frontmatter contract to hold before they cite an atom, and U2 needing to
    know exactly what a green run did and did not prove, WHEN `redkiln validate --kb` is run on the
    tree carrying the landed atoms, THEN it exits zero, and the ledger records beside that result
    the two limits it does not cover — it does not enforce the `authority_tier` vocabulary
    (`authority_tier` is `z.string().min(1)`), so AC-007 green is not evidence for AC-002 or AC-003;
    and it cannot see `.kb/_intake/`, so it is not evidence for AC-006.
  satisfied: false
  evidence: "" # the captured command output and exit code, plus the two recorded limits
  mount_point: "redkiln validate --kb over .kb/ — the only automated reader of the landed layer"
  verifying_test: >-
    Tier 1 — `redkiln validate --kb` exit code zero with captured output, paired with
    `redkiln doctor` still reporting exactly six template-drift advisories; scoped to the atoms as
    landed only, per .bklg/docs-that-teach/durable-audience-closeout/_storymap.md, "## Coverage",
    AC-011 row

- id: AC-008
  criterion: >-
    GIVEN U3, and anyone who has already cited a `file:line` in either map, needing the anchor they
    cited to still mean what it meant, WHEN they diff the two map files and the decisions layer
    after the wave, THEN whatever the run's map-sync wrote to `.kb/maps/domain-map.md` or
    `.kb/maps/open-questions-index.md` is an appended `##` section or appended bullets in each map's
    own fixed shape — domain-map entries grouped by kind under bold labels with each atom's id
    beside its link; index bullets with the status word first, then the link, then the id, then one
    sentence — with no deletion line anywhere in either diff, and `.kb/decisions/` shows no line at
    all, added or modified.
  satisfied: false
  evidence: "" # the two empty deletion-line greps and the empty decisions name-only diff
  mount_point: ".kb/maps/domain-map.md and .kb/maps/open-questions-index.md (append-only); .kb/decisions/ (untouched)"
  verifying_test: >-
    Tier 1 — `git diff -- .kb/maps/domain-map.md .kb/maps/open-questions-index.md | rg "^-[^-]"`
    returns nothing and `git diff --name-only -- .kb/decisions/` returns nothing; Tier 2 — the
    appended content read for shape against .kb/maps/domain-map.md:144-150 and
    .kb/maps/open-questions-index.md:136-143

- id: AC-009
  criterion: >-
    GIVEN the implementer of product-layer-mounting, who must run
    `redkiln record-links HS-P0025 --atom`, write the domain-map entries and fill the
    `## Knowledge Harvest` rows without re-deriving the landed set by listing a directory, WHEN they
    open this story's `_ledger.md`, THEN they find the complete handoff — for every atom the wave
    landed, its id, its path, its `kind` and its `authority_tier` — with every one of those states
    written as a literal word, no emoji, tick, colour word, strikethrough, empty cell or ordering
    carrying meaning anywhere in this story's diff, and every link naming its destination rather
    than "here" or "see above".
  satisfied: false
  evidence: "" # the landed-atom handoff list itself, plus the negative glyph sweep
  mount_point: "This _ledger.md, consumed by .bklg/docs-that-teach/durable-audience-closeout/product-layer-mounting/"
  verifying_test: >-
    Tier 2 primary — read the handoff list against `ls .kb/product/ .kb/playbooks/ .kb/open-questions/`
    (a landed atom absent from the list is an unmounted atom); Tier 1 backstop — a glyph/strikethrough
    sweep over .bklg/docs-that-teach/durable-audience-closeout/audience-ingest-wave/ and .kb/product/
    returns nothing, per .bklg/docs-that-teach/durable-audience-closeout/_decomposition.md,
    "## UX brief", AC-UX-09
```
