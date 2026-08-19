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
  satisfied: true
  evidence: >-
    Enumerated before walked, from _design.md:45-65's four surface ids resolved to
    paths, not from a directory listing: crates/happenstance/src/lib.rs, docs/first-encounter.md,
    docs/carry-your-invariant.md, docs/read-the-worked-example.md — all four exist, so EC-001 did not
    fire. Cross-check 1, pasted at _walk.md § 1: `git grep -n '](' -- docs/README.md` returns the
    narrative index rows :19 append-conditions.md, :20 first-encounter.md, :21 text-fences.md.
    Cross-check 2: `git diff main --name-only -- docs crates/happenstance/src/lib.rs` returns seven
    paths. BOTH set differences computed and printed. In the index and not in the enumeration:
    append-conditions.md and text-fences.md — not a finding, they are HS-P0020's own pages added by
    7020c4c and 5ecce36. In the enumeration and NOT IN THE INDEX: docs/carry-your-invariant.md and
    docs/read-the-worked-example.md — recorded as finding W-1 and routed to HS-P0023, never quietly
    appended. No fifth page of this project's set was found by either cross-check. Method, date
    (2026-08-19) and walker recorded at _walk.md § 1 and § 2.
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
  satisfied: true
  evidence: >-
    _walk.md § 3, four pages walked top to bottom against
    standards/pages/40-reviewing-a-page.md's six steps, answers recorded per step rather than
    summarised. Mechanical half pasted: `git grep -c '^> \*\*Answers:\*\*'` returns exactly 1 for
    each of the three docs pages and 1 for the crate root; the head region of all four captured
    showing H1 (or the crate summary), blank, declaration, blank, with NOTHING interposed; declaration
    line against first-fence line — lib.rs :21 vs :26, first-encounter :3 vs :16,
    carry-your-invariant :3 vs :60, read-the-worked-example :3 vs no fence (recorded as vacuously
    above, in that word). Tokens checked against the closed set at
    standards/pages/10-the-need-set.md:12-17: tutorial, tutorial, explanation, orientation — all four
    members, spelled exactly. VERDICTS: four `pass`, one per page, exactly one of the four available;
    NO page is at `fail` or `indeterminate`. The crate root's `pass` carries two scopings stated in
    full at _walk.md § 3 (EC-005's missing H1 anchor, and RP-40-1 step 3's corpus against a rustdoc
    reference surface band 10 excludes by name at standards/pages/10-the-need-set.md:25-31), both
    routed to HS-P0021 as W-2 and W-3 so a reviewer can overturn the judgement from the evidence.
    Walker: this story's implementation context, non-author of all four surfaces (EC-008 clear);
    sources consulted and not consulted recorded at _walk.md § 2.
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
  satisfied: true
  evidence: >-
    _walk.md § 4 carries a three-column table, one row per surface, never averaged:
    the instrument that observed the declaration, whether that instrument COULD HAVE FAILED on this
    page, and the verdict. crate-root-encounter's middle column reads NO — the file is outside
    TREE = "docs" (xtask/src/lint_narrative.rs:239) and `every page declares one need` never opens it
    — with the gap routed to HS-P0021 as W-4 rather than counted as coverage; the story-specific test
    at xtask/tests/first_encounter.rs:184 is named there and is explicitly not the discipline's
    instrument. The mechanical step was RUN and its output pasted:
    `cargo run --locked --quiet -p xtask -- lint-pages` -> `5 pages, 16 rules, all consistent` — five,
    because docs/ holds six markdown files and the index is excluded by
    xtask/src/lint_narrative.rs:250, which is the coverage column stated as a count. The corpus is
    SHOWN not assumed: `git grep -n 'const TREE' -- xtask/src/lint_narrative.rs` -> :239, and
    `git grep -n 'lint_narrative::TREE' -- xtask/src/lint_pages.rs` -> :379, :467, :519, with
    xtask/src/lint_pages.rs:443-444 stating no second constant names it. NO PAGE_DIR exists, no second
    corpus and no second checker were written: `git diff 17f14b5 -- xtask/ standards/ spec/` is EMPTY,
    which is the boundary proof scoped to this story rather than to the branch. The crate-root row was
    cross-read against the slice-mate's row for the same file and the two AGREE —
    fence-inventory-and-clause-audit/_inventory.md § Routing R-2 and R-5 reach `no` for that file's
    fences and citations for the same structural reason.
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
  satisfied: true
  evidence: >-
    _walk.md § 5, both directions. (a) The inert specimen:
    `git grep -c '^> \*\*Answers:\*\*' -- standards/pages/examples/two-needs.md` -> 2, and
    `ls standards/pages/*.md` does NOT list it (six files, none of them the specimen), proving it inert
    per standards/pages/examples/two-needs.md:13-16. Walked: step 1 answers `no`, two declarations
    both visible in the head, VERDICT `fail — two needs`, matching what the specimen's own :34-38 says
    the walk should find. EC-006 did not fire. (b) A live page of this project's set — the bridge,
    chosen over the handoff because the handoff carries a second instrument in
    examples/course-subscriptions/tests/reach.rs and the drill could not then attribute the failure.
    Four pasted outputs in order: the injected diff adding
    `> **Answers:** \`how-to\` — How do I build an AppendCondition from my own rule?` at :4;
    `cargo run --locked --quiet -p xtask -- lint-pages` FAILING with
    `docs/carry-your-invariant.md:4 — declares \`explanation\` and \`how-to\`; a page answers one
    need` and the same failure under `cargo xtask lints`; the revert shown by
    `git hash-object docs/carry-your-invariant.md` -> b63697589f05c4a1663ef3754a5d96c59b4b6298,
    identical to pre-injection, with `git status --porcelain` EMPTY; and the step green again at
    `5 pages, 16 rules, all consistent`. The walk over the injected page also returned
    `fail — two needs`. Both instruments were shown detecting a presence, which is what makes the
    report of an absence mean anything.
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
  satisfied: true
  evidence: >-
    _walk.md § 6. Full output pasted with line numbers for all four phrases over the
    five files AC-005 names: `git grep -ni "aggregate"` -> docs/carry-your-invariant.md:22 only;
    `git grep -ni "one stream per entity"` -> :20 only; `git grep -ni "which stream"` -> :19 only;
    `your aggregates` is the same :22 hit. ZERO hits on crates/happenstance/src/lib.rs, ZERO on
    docs/first-encounter.md, ZERO on docs/read-the-worked-example.md, ZERO on docs/README.md. Every
    hit falls inside the `## Where your streams went` span, captured separately: the section opens at
    docs/carry-your-invariant.md:17 and the next `##` opens at :30, so the span is :17-29 and the hits
    sit at :19, :20 and :22. Anti-pattern 6 (_design.md:888-890) was also run in its
    reviewer-performable form against the rendered pages so the check survives a phrase the grep did
    not anticipate: neither the crate root nor any step of the opening encounter asks the reader to
    think in streams or entities at all. EC-009 did not fire. The three `aggregate` occurrences in
    examples/course-subscriptions/src/overview.md are outside AC-005's stated corpus and already owned
    by tension-resolutions/_resolutions.md:407; docs/read-the-worked-example.md:10-12 names that seam
    and sends the reader to the anchor.
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
  satisfied: true
  evidence: >-
    _walk.md § 6. `git grep -c "Where your streams went" -- docs/carry-your-invariant.md`
    -> 1, and `git grep -n` places it at :17 as `## Where your streams went`, exactly that text, exactly
    once (note: the working copy is CRLF, so a `$`-anchored pattern matches nothing — the command is
    written without one). Every inbound reference enumerated by
    `git grep -n "where-your-streams-went" -- crates/happenstance/src/lib.rs docs examples standards`:
    docs/read-the-worked-example.md:12 is the ONE relying page and it links the one recorded location;
    the target file exists and the fragment is the slug of the heading at :17; it is also asserted
    mechanically by examples/course-subscriptions/tests/reach.rs:42, which is green. The crate root and
    the opening encounter rely on the decision NOT AT ALL — zero prior-model phrases — so no link is
    owed, which is HS-S0185's own conditional at boundary-refusal-encounter/spec.md:418. Anti-pattern 2:
    `git grep -nE '\[\`[a-z_:]+\`\]' -- crates/happenstance/src/lib.rs` returns three hits and all
    three carry a target (two inline, one reference-style resolved at :161-168), so none is an
    unresolved pair; the authoritative read is off the render, where
    fence-inventory-and-clause-audit/_inventory.md records ZERO literal [bracket] pairs in the crate
    root's rendered div.docblock against the four _design.md:951-956 measured pre-merge. The
    `documentation` REQUIRED step (xtask/src/main.rs:344) under RUSTDOCFLAGS=-D warnings is green inside
    `cargo xtask ci --fast`. EC-004 fired and its disposition is the one already recorded: the heading
    measures 23 characters, and tension-resolutions/_resolutions.md:338-343 scoped the 22-character
    budget to crate-root-encounter alone, so it does not bind this markdown surface — measured,
    recorded, NOT renamed. The keyboard-only traverse `crate root -> bridge -> #where-your-streams-went`
    was attempted and DOES NOT COMPLETE: docs/first-encounter.md carries no link to the bridge and
    docs/README.md carries no row for it. That is recorded verbatim as finding W-1 and routed to
    HS-P0023; it is a defect of REACH, and the four THEN clauses of this criterion — one relying page
    links, the heading exists exactly once, the link resolves, no literal bracket survives — hold
    independently of it.
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
  satisfied: true
  evidence: >-
    _walk.md § 7, a table with one row per page carrying the verdict and the sentence
    the verdict turned on QUOTED VERBATIM, never a tick. One relying page:
    docs/read-the-worked-example.md, verdict CITES, on the sentence at :10-12 — "Its own explanation
    names the model you arrived with, in another voice and correctly for its purpose; the single place
    that model is answered is [where your streams went](carry-your-invariant.md#where-your-streams-went)."
    The judgement is stated so it can be disagreed with: the sentence names that a prior model is
    present downstream, disclaims correcting it there, and points, with the link as its last element;
    it does not say what the prior model is, does not say why it does not apply, and repeats no part of
    `## Where your streams went`. A reader following it instead of the bridge would learn only where to
    go. crates/happenstance/src/lib.rs and docs/first-encounter.md are recorded as DOES NOT RELY (zero
    prior-model phrases, _design.md:116-118 binds them to name none); docs/carry-your-invariant.md IS
    the anchor. NO page was found re-arguing, so no repair was made and AC-005's sweep did not have to
    be re-run against a changed phrase corpus.
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
  satisfied: true
  evidence: >-
    The composed result table is at _ledger.md `## Recorded result` below this block —
    eight columns per surface: id, path, declared token, verdict, instrument, walker, date and routing
    destination — with the working shown at _walk.md. Every row that is vacuous or not applicable is
    recorded IN THOSE WORDS rather than dropped: read-the-worked-example's fence-position cell reads
    `vacuously above — the page carries no fence`, and its and the crate root's normative-sentence
    cells read `vacuously yes`. No row reads blocked, not yet authored or indeterminate, because none
    is: EC-001 clear (all four surfaces exist), EC-002 clear (HS-P0021's atoms, walk and checker are
    all present), EC-003 clear (the corpus constant is present and single; the module path deviation is
    recorded as W-5 rather than halted on), EC-008 clear (a non-author walker was available for all
    four). SIX findings, W-1..W-6, each with a named destination and none absorbed: W-1 reach ->
    HS-P0023, W-2 and W-3 rule/form gaps -> HS-P0021, W-4 corpus coverage -> HS-P0021, W-5 a citation
    correction -> HS-P0021, W-6 a stale consumption-map row -> HS-P0025's reference reconciliation.
    EVERY in-tree repair is a single line because NONE was made: this story's own diff touches no
    surface at all. `git diff 17f14b5 --stat` is confined to this story's own backlog folder;
    `git status --porcelain` is EMPTY with docs/carry-your-invariant.md byte-identical to its
    pre-injection hash b63697589f05c4a1663ef3754a5d96c59b4b6298; `git diff 17f14b5 -- xtask/ standards/
    spec/` is EMPTY (NF-001, NF-006). `cargo xtask affected --base main` -> `affected gate passed`;
    `cargo xtask ci --fast` -> `all required checks passed (--fast: 4 optional step(s) not run)`.
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

## Recorded result

The artifact project DoD item 7 (`project.md:296-297`) asks for, composed as a table with named
columns rather than as prose a later reader cannot audit. The working — the step-by-step walk
answers, the pasted command outputs, the calibration transcripts and the anchor sweep — is at
`_walk.md` beside this file.

**Walked 2026-08-19** by this story's implementation context, which authored **none** of the four
surfaces (`_walk.md § 2` names what it was and was not permitted to read). The tier-5 human
sign-off is the project's review gate; this table is what it reads.

| surface id | path | declared token | verdict | instrument that observed the declaration | could it have failed? | walker | date | routing |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `crate-root-encounter` | `crates/happenstance/src/lib.rs` | `tutorial` | **`pass`**, with two scopings stated (`_walk.md § 3`) | **a named person** — the walker below | **no** — outside `TREE`; `every page declares one need` never opens it | non-author, `_walk.md § 2` | 2026-08-19 | **W-2, W-3, W-4 → HS-P0021** |
| `opening-encounter` | `docs/first-encounter.md` | `tutorial` | **`pass`** | `every page declares one need` (`cargo run --locked --quiet -p xtask -- lint-pages`) | **yes** | non-author, `_walk.md § 2` | 2026-08-19 | — |
| `conceptual-bridge` | `docs/carry-your-invariant.md` | `explanation` | **`pass`** | the same step, and **only** that step | **yes** — observed failing, `_walk.md § 5(b)` | non-author, `_walk.md § 2` | 2026-08-19 | **W-1 → HS-P0023** (reach, not the declaration) |
| `worked-example-handoff` | `docs/read-the-worked-example.md` | `orientation` | **`pass`** | the same step, plus `examples/course-subscriptions/tests/reach.rs` | **yes** | non-author, `_walk.md § 2` | 2026-08-19 | **W-1 → HS-P0023** (reach, not the declaration) |

**Four surfaces, four verdicts, four `pass`. No row is at `fail`, `indeterminate`, `blocked`,
`vacuous` or `not yet authored`** — and each of those five words was checked for rather than
assumed away: EC-001, EC-002, EC-003, EC-006 and EC-008 all cleared, with the checks recorded at
`_walk.md § 8`.

**The check was demonstrated able to return `fail`, twice** (`_walk.md § 5`): once on HS-P0021's
inert specimen, and once on a live page of this project's own set, where the `every page declares
one need` step failed by `path:line` and the tree was then observed clean and byte-identical.
Without that, four rows of `pass` would be an impression.

### Findings, each with a destination

| id | finding | destination |
| --- | --- | --- |
| **W-1** | `docs/carry-your-invariant.md` and `docs/read-the-worked-example.md` are an isolated two-page component: absent from `docs/README.md`'s narrative index, linked by neither the crate root nor the opening encounter, and linking only to each other. The keyboard traverse does not complete | **HS-P0023** `reach-and-adapter-path` (DT-10) |
| **W-2** | HS-P0021's declaration form has no defined anchor on a rustdoc crate root — RP-00-2 anchors to a markdown `# Title` and there is none | **HS-P0021** `page-need-discipline` |
| **W-3** | RP-40-1 states no behaviour for a page outside the governed tree; step 3's corpus had to be scoped, and the scoping is disclosed in full so it can be overturned | **HS-P0021** `page-need-discipline` |
| **W-4** | Nothing in the repository reads the crate root's answered-need line | **HS-P0021** `page-need-discipline` |
| **W-5** | The corpus constant is `xtask::lint_narrative::TREE`, not `xtask::narrative::TREE` as HS-P0021's spec names it. Present and single, so EC-003 records rather than halts | **HS-P0021** `page-need-discipline` |
| **W-6** | `tension-resolutions/_resolutions.md:404` predicts a crate-root link to the anchor that the merged tree correctly does not carry, because HS-S0185's obligation is conditional on relying on the decision | **HS-P0025** `durable-audience-closeout`, reference reconciliation |

**Nothing absorbed, and no repair made.** This story's own diff touches none of the four surfaces:
every finding is a pointer-policy, rule or record question owned elsewhere, and each names the item
that owns it.
