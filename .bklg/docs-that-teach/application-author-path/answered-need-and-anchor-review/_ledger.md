---
item: HS-S0190
stage: implement
created: "2026-08-17T13:16:37.043Z"
updated: "2026-08-17T13:16:37.043Z"
---

# Acceptance ledger — Each page's answered need and every anchor reviewed

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

**Story-specific note on evidence.** Most of this story's proof is *procedural (ledger-recorded)* —
the testing brief's fifth tier (`_decomposition.md:486`), which is proof here because no `#[test]`
carries a judgement. NF-002 binds: every mechanical row's evidence is a **pasted command output**,
and a row whose evidence reads "verified" is not satisfied. Rows recorded as *vacuous*, *blocked*,
*not yet authored* or *indeterminate* use those words (AC-008, EC-001, EC-002) and do **not** count
as satisfied.

```yaml
- id: AC-001
  criterion: >-
    GIVEN the reviewer is about to make a claim about the whole set — the thing Persona 1 actually
    depends on, per backbone activity A6 (_storymap.md:46) — and a page authored and then forgotten
    would pass by being absent rather than by being correct, WHEN the walk begins, THEN the set is
    enumerated before it is walked, from _design.md's four surface ids (:45-65) resolved to real
    paths, and cross-checked twice against sources that cannot both be stale — docs/README.md's
    narrative index rows, and `git diff main --name-only` over the five blocking stories' output —
    AND a page found by either cross-check and missing from the enumeration is recorded as a
    finding, never quietly appended to the table.
  satisfied: false
  evidence: ""
  mount_point: xtask/src/narrative.rs
  verifying_test: >-
    Mechanical — the three enumerations pasted: the four surface ids from
    .bklg/docs-that-teach/application-author-path/_design.md:45-65 resolved to paths;
    `rg -n '\]\(' docs/README.md`; `git diff main --name-only -- docs crates/happenstance/src/lib.rs`;
    both set differences computed and printed. Procedural (ledger, tier 5) — the enumeration method,
    date and walker recorded here.

- id: AC-002
  criterion: >-
    GIVEN Persona 1, whose stated fear is silent wrongness (_decomposition.md:28-30) and who must be
    able to tell what a page is for before investing in it, WHEN a named non-author executes
    HS-P0021's walk (standards/pages/40-reviewing-a-page.md#the-walk, ## RP-40-1) top to bottom from
    the rendered page alone, consulting neither the author nor the page's git history, THEN every
    surface in AC-001's enumeration carries exactly one declaration in HS-P0021's exact form —
    '> **Answers:** `token` — <question>?' as a single blockquote line, the first element under the
    page's H1 with nothing interposed (no badge row, no table of contents, no "last updated" line),
    the token one member of the closed set orientation/tutorial/how-to/explanation, the question in
    the reader's voice ending in '?' — and therefore above the first fence in reading order; AND each
    page lands on exactly one of the four verdicts pass / fail — two needs / fail — need not answered
    / indeterminate, with the per-step yes/no answers recorded rather than summarised; AND no page is
    at fail or indeterminate when this story completes.
  satisfied: false
  evidence: ""
  mount_point: xtask/src/narrative.rs
  verifying_test: >-
    Mechanical — per docs/ page: `rg -c '^> \*\*Answers:\*\*' <page>` = 1; the first 12 source lines
    captured showing H1, blank, declaration, blank, nothing interposed;
    the first fenced-block opening line located with `rg -n` and its line number shown to be greater
    than the declaration's, both numbers captured; token membership against
    .bklg/docs-that-teach/page-need-discipline/need-vocabulary-and-declaration-form/spec.md:71-82.
    Procedural (ledger, tier 5) — the non-author walk of
    standards/pages/40-reviewing-a-page.md#the-walk, four rows, each with walker identity, date,
    step-by-step yes/no answers and one verdict.

- id: AC-003
  criterion: >-
    GIVEN a later reader deciding whether to trust this record, for whom "4/4 declare one need"
    without a coverage column is the reassurance this initiative exists to refuse, WHEN the result is
    written, THEN it carries a per-page coverage table — never an average — with three columns per
    surface: the instrument that actually observed the declaration (the `every page declares one
    need` REQUIRED step, or a named person), whether that instrument could have failed on this page,
    and the verdict; AND crate-root-encounter's middle column reads no, because
    crates/happenstance/src/lib.rs is outside xtask::narrative::TREE and nothing in this repository
    reads its answered-need line, with that gap routed rather than counted as coverage; AND the
    mechanical step is run, its corpus named, and its output pasted — no second corpus is declared
    and no second checker is written.
  satisfied: false
  evidence: ""
  mount_point: xtask/src/narrative.rs
  verifying_test: >-
    Mechanical — `cargo run --locked --quiet -p xtask -- lint-pages` (the REQUIRED step
    `every page declares one need`, xtask/src/main.rs) executed and its full output pasted;
    `rg -n 'TREE' xtask/src/narrative.rs` capture; `git diff main --stat -- xtask/src` showing no new
    corpus const and no new checker. Procedural (ledger, tier 5) — the three-column per-surface
    table, with the crate-root row's `no` cell carrying its routing destination and recorded as
    agreeing with the slice-mate's inventory row for the same file
    (.bklg/docs-that-teach/application-author-path/fence-inventory-and-clause-audit/spec.md).

- id: AC-004
  criterion: >-
    GIVEN that a procedure which has never returned fail is decorative — CLAUDE.md's own corollary
    for conformance rules, applied to a written one — and that this story's deliverable is an
    absence, which can only be demonstrated by showing the instrument detects a presence, WHEN the
    calibration is run in both directions, THEN (a) HS-P0021's inert two-declaration specimen
    standards/pages/examples/two-needs.md is walked to 'fail — two needs', AND (b) a second
    declaration is temporarily injected into one page of this project's own set, the walk returns
    'fail — two needs' and — for a page inside TREE — the gate step fails naming that page by
    path:line, AND the injection is reverted and both the walk and the step return to green with the
    working tree observed clean.
  satisfied: false
  evidence: ""
  mount_point: xtask/src/narrative.rs
  verifying_test: >-
    Mechanical — four pasted outputs in order for direction (b): the injected diff;
    `cargo run --locked --quiet -p xtask -- lint-pages` failing with the offending path:line;
    `git checkout -- <page> && git status --porcelain` empty; the same step green. For direction (a):
    `rg -c '^> \*\*Answers:\*\*' standards/pages/examples/two-needs.md` = 2, and
    `ls standards/pages/*.md` not listing it. Procedural (ledger, tier 4/5) — both walks recorded
    step by step with the walker named; a pass on either is EC-006 and fails this row outright.

- id: AC-005
  criterion: >-
    GIVEN the reader arriving with the stream-per-entity prior — who asks 'which stream does this go
    in?' reflexively and will ask it again on every page until something answers it — and given that
    answering it slightly differently three times is initiative AC-08's named failure
    (initiative.md:383-385), WHEN the reviewer sweeps this project's authored spans, THEN the prior
    model is named on exactly one page: hits for aggregate, your aggregates, one stream per entity
    and which stream fall only inside docs/carry-your-invariant.md's '## Where your streams went'
    span, and nowhere on the crate root or on any step of the opening encounter.
  satisfied: false
  evidence: ""
  mount_point: xtask/src/narrative.rs
  verifying_test: >-
    Mechanical —
    `rg -ni 'aggregate|your aggregates|one stream per entity|which stream' crates/happenstance/src/lib.rs docs/first-encounter.md docs/carry-your-invariant.md docs/read-the-worked-example.md docs/README.md`
    with full line-numbered output pasted, and every hit's line number shown to fall inside the
    separately captured start/end lines of '## Where your streams went'. Procedural (ledger, tier 5)
    — anti-pattern 6 (_design.md:852-854) run in its reviewer-performable form against the rendered
    pages, so a phrase the grep did not anticipate is still caught (EC-009).

- id: AC-006
  criterion: >-
    GIVEN the reader two pages downstream who needs the anchor decision but must not be made to read
    it twice, WHEN any page relies on the DT-1 decision, THEN it links to the one recorded
    reader-facing location — docs/carry-your-invariant.md#where-your-streams-went, a real '##'
    heading whose slug is stable, human-readable and unnumbered so inserting a section breaks no
    inbound link (IQ-4) — AND that heading exists exactly once with exactly that text, AND every such
    link is proven to resolve rather than assumed to: the intra-doc link from the crate root is
    denied by RUSTDOCFLAGS=-D warnings if it does not, and no literal bracketed code-styled word
    (anti-pattern 2, the visible signature of an unresolved reference) survives on any page this
    project touches.
  satisfied: false
  evidence: ""
  mount_point: xtask/src/narrative.rs
  verifying_test: >-
    Mechanical — `rg -n '^## Where your streams went$' docs/carry-your-invariant.md` = 1;
    `rg -n 'where-your-streams-went' crates/happenstance/src/lib.rs docs/` with every relying page
    enumerated and each target resolved and pasted; the "documentation" REQUIRED step
    (xtask/src/main.rs:290) green under RUSTDOCFLAGS=-D warnings;
    `rg -n '\[`[a-z_:]+`\]' crates/happenstance/src/lib.rs` showing no unresolved pair. Procedural
    (ledger, tier 5) — the keyboard-only traverse recorded as
    'crate root -> bridge -> #where-your-streams-went', landing on the heading.

- id: AC-007
  criterion: >-
    GIVEN that a page which re-states the decision — even in agreement — has become a second recorded
    home, which is exactly the defect BR-07 exists to prevent (project.md:336), and that no command
    can tell a citation from a paraphrase, WHEN the reviewer reads each relying page, THEN they
    answer one yes/no question per page — does this page state the prior-model decision in its own
    words, or does it cite the one location? — AND the answer is recorded as a judgement with the
    sentence it turned on quoted verbatim, never as a tick; AND any page found re-arguing is repaired
    at one-line grain by replacing the restatement with a link to #where-your-streams-went, with the
    before and after both recorded.
  satisfied: false
  evidence: ""
  mount_point: xtask/src/narrative.rs
  verifying_test: >-
    Procedural (ledger, tier 5) — the only step in this story a command cannot settle. One row per
    relying page: page, verdict 'cites' or 're-argues', and the sentence quoted verbatim that the
    verdict turned on. Where a repair was made, the one-line diff is pasted and AC-005's `rg` sweep
    is re-run afterwards (a repair changes the phrase corpus). Authority: project.md:260-262,
    _decomposition.md:350-352, and the mirror commitment at
    .bklg/docs-that-teach/application-author-path/invariant-to-appendcondition-bridge/spec.md:430.

- id: AC-008
  criterion: >-
    GIVEN project DoD item 7 (project.md:296-297) asks for a recorded result and not for a green
    feeling, and given that RUNBOOK.md:920-925 is the recorded cost of a green record standing in for
    a check that never ran, WHEN the walk closes, THEN the result exists as a composed per-page table
    — surface id, path, declared token, verdict, instrument, walker identity (a named non-author),
    date, and routing destination where applicable — written into this story's _ledger.md against the
    AC ids above rather than into prose a later reader cannot audit; AND every row that is vacuous,
    blocked, not yet authored or indeterminate is recorded in those words; AND every in-tree repair
    is a single line, with anything larger stopped and routed under project DoD item 9
    (project.md:300-302); AND the working tree is clean and `cargo xtask affected --base main` is
    green at the checkpoint.
  satisfied: false
  evidence: ""
  mount_point: xtask/src/narrative.rs
  verifying_test: >-
    Mechanical — `git diff main --stat` confined to the PR boundary and showing only single-line
    hunks in the four surfaces plus at most a docs/README.md index row and an xtask/src/narrative.rs
    registration line; `git status --porcelain` empty; `cargo xtask affected --base main` green with
    output pasted (.redkiln/config.yaml:40); `redkiln verify --grain story` passing under
    require_ledger (.redkiln/config.yaml:67). Procedural (ledger, tier 5) — the result table itself,
    plus a routed-findings list naming each destination (HS-P0020, HS-P0021, HS-P0023, HS-P0024, or
    the support initiative).
```
