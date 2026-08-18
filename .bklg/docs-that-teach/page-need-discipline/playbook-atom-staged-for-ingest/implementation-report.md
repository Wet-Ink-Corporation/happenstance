---
item: "HS-S0153"
stage: implement
created: "2026-08-18T00:00:00.000Z"
updated: "2026-08-18T00:00:00.000Z"
---

# Implementation Report — The playbook atom staged under .kb/_intake/, never hand-authored into .kb/

`<atom>` stands for `.kb/_intake/lesson-page-need-declaration-discipline.md` throughout.

## TDD Evidence

This story adds **no Rust**, so it carries no `#[test]` — the testing brief's own categorisation of
AC-012, not a shortcut (spec, *Tests and CI*). Its Red/Green is the command set the acceptance table
names, run before the file existed and again after, plus one authoring bar that failed twice and had
to be fixed before Green.

**Red, before the file existed.**

| AC | command | Red result |
| --- | --- | --- |
| AC-001 | `ls .kb/_intake` | `README.md` — the directory held only its own README |
| AC-001 | `git diff --name-status HEAD -- .kb/_intake` | no output — nothing staged |
| AC-010 | `rg -n "_intake/lesson-page-need" -g '!.bklg/**'` | exit 1 — nothing pointed at a path that does not exist |

**Green, after staging.** Every row of the acceptance table's mechanical half is captured in the
sections below. The two that were genuinely *red first and green after work* rather than red-by-
absence are AC-011's two budgets, and they are worth naming because they were failed twice:

| measurement | first draft | second draft | shipped |
| --- | --- | --- | --- |
| `wc -c <atom>` (ceiling **8,192**) | **10,324** | 9,687 → 9,425 → 9,147 → 8,862 → 8,550 → 8,452 → 8,239 | **8,184** |
| max source column (ceiling **100**) | **101** | 92 | **92** |

The first draft failed both. Compression was applied to the prose and to none of the obligations:
the four method steps, their evidence, the five rejection families and the four stop conditions are
all still present, which is what AC-006, AC-007 and AC-009's checks confirm below.

## Commits

| SHA | subject |
| --- | --- |
| `dc58e80` | `feat(page-need-discipline): Playbook atom staged for ingest` |

One checkpoint commit carrying `<atom>` plus this story's `_ledger.md`, `implementation-report.md`
and `report.md`. Trailer: `Story: page-need-discipline/playbook-atom-staged-for-ingest`. The
slice-mate `governed-page-cites-the-discipline` landed separately at `263dc7b`.

## Changes

| file | shape of the change |
| --- | --- |
| `.kb/_intake/lesson-page-need-declaration-discipline.md` | **new, and the whole of the product.** 8,184 bytes. `kind: playbook`, `status: proposed`, `authority_tier: guideline`; ten required frontmatter fields; six body sections — `## The situation`, `## The method, in four steps`, `## The alternatives that lost`, `## What makes it portable`, `## Where this is the wrong instrument`, under one H1 |
| `.bklg/…/playbook-atom-staged-for-ingest/_ledger.md` | eleven rows flipped to `satisfied: true` with evidence citing the sections below by name and line |
| `.bklg/…/playbook-atom-staged-for-ingest/implementation-report.md`, `report.md` | this file and the review-facing report |

Nothing else moved. `git diff HEAD --stat -- .kb ':!.kb/_intake'` is empty, `git diff HEAD --stat --
.kb/maps` is empty, and `standards/pages/**` and `xtask/**` are untouched by this story.

## Gates

| command | result |
| --- | --- |
| `cargo xtask affected --base main` | `affected gate passed` |
| `cargo xtask ci --fast` | `all required checks passed (--fast: 4 optional step(s) not run)` |
| `redkiln validate --kb` | `redkiln: validate passed.` |
| `redkiln doctor` | `doctor found no problems` plus **exactly six** template warnings (`redkiln doctor 2>&1 \| rg -c "template-drift\|differs SUBSTANTIVELY"` → `6`) |

The last two prove **AC-010 only**. `redkiln validate --kb` skips every `_`-prefixed directory by
design (`.kb/_intake/README.md:21-27`), so it is silent about `<atom>`'s own frontmatter and is not
cited anywhere under AC-002, AC-003 or AC-005 (EC-005).

## Notes

### §Mount

```text
$ git diff --name-status --cached HEAD -- .kb/_intake
A	.kb/_intake/lesson-page-need-declaration-discipline.md

$ ls .kb/_intake
README.md
lesson-page-need-declaration-discipline.md
```

Exactly one `A` line, exactly two files. **The staged file, named in full for HS-P0025's approval
gate: `.kb/_intake/lesson-page-need-declaration-discipline.md`.** The other file in the glob is
`.kb/_intake/README.md`, which is deliberately not an atom and is the one the ingest operator drops
at the gate. The payload is registered in no manifest, index or config: `.kb/_intake/*.md` is the
default glob and needs none (`.kb/_intake/README.md:3-5`).

### §Frontmatter walk

Performed by hand against `.kb/README.md:16-26`, because nothing mechanical reads this directory.
Ten rows, each citing the staged file's own line.

| # | field | `<atom>` line | value, and why it is well-formed |
| --- | --- | --- | --- |
| 1 | `id` | `:2` | `kb-playbook-declared-page-need-001` — the corpus's `kb-playbook-<slug>-001` shape. A **proposal**: the wave may merge this atom into an existing one under a different id, and nothing downstream depends on the id surviving |
| 2 | `title` | `:3` | `Making the need a page answers a declared, singular, checkable property` — a title, not a restatement of the summary |
| 3 | `kind` | `:4` | `playbook` — fixed by DR-11 and by `.kb/playbooks/README.md:3-5`. The atom prescribes a method rather than describing a mechanism, which is what keeps it out of `concept` |
| 4 | `status` | `:5` | `proposed`. **Not `accepted`** — acceptance is conferred by the ingest wave that adjudicates the atom against the corpus, and asserting it in the one directory nothing validates would claim a state no gate granted. **Not `draft`** — draft invites the wave to treat the file as raw material and re-derive its claims at full cost, which is exactly the cost this atom is authored to avoid |
| 5 | `authority_tier` | `:6` | `guideline` — the layer's tier (`.kb/playbooks/README.md:3-5`). The schema is `z.string().min(1)` (`.kb/README.md:31-35`), so this is a convention nothing enforces, which is why it is walked here |
| 6 | `summary` | `:7-17` | a folded block (`>-`), four sentences, adjudicable without opening the body — see §Summary adjudication |
| 7 | `depends_on` | `:18-19` | one id, `kb-playbook-verify-referent-report-coverage-001` — the direct ancestor of "the check sees a declaration, never an answer" |
| 8 | `related` | `:20-23` | three ids: `kb-playbook-ratchet-gate-landing-001`, `kb-playbook-anchoring-citations-001`, `kb-governance-referent-not-reasoning-001` |
| 9 | `source_paths` | `:24-32` | eight paths, every one resolving — see §Resolution transcript |
| 10 | `last_reviewed` | `:33` | `2026-08-18` — the date the atom was written **against the shipped tree**, which is this story's merge date and not the design's sign-off date |

No row of this walk cites `redkiln validate --kb`.

### §Summary adjudication

The four elements, quoted out of `<atom>:7-17` and nothing else:

- **the problem** — *"A prose tree teaches unevenly and nothing sees it: the need each page answers
  is implicit and therefore arguable."*
- **the properties that decide the design** — *"one need per page, declared in the page's own
  visible body text, spelled from a closed enumerated set, checked mechanically — then the check
  documents the judgement it cannot make, and a written non-author walk supplies it."*
- **the options ruled out** — *"Front matter, an HTML comment, a filename convention, a sidecar
  manifest and a badge all lost as declaration forms; an open set lost because no check tests
  membership in one; a persona taxonomy lost because a persona is a property of the reader."*
- **the discriminator** — *"a check sees a declaration and never an answer, so any form that hides
  it from the reader or makes it unenumerable hands the discipline back to judgement."*

**Sentence count: 4.** Side by side with the two corpus exemplars —
`.kb/playbooks/landing-a-stricter-gate-without-a-red-baseline.md:7-15` runs **5** sentences and
`.kb/playbooks/anchoring-citations-in-a-long-lived-document.md:7-15` runs **4**, and both are folded
blocks that state problem, deciding properties, ruled-out options and discriminator in that order.
This summary is the same shape at the same length. It is not a one-line label, and it does not
restate the title.

### §Resolution transcript

`source_paths`, one line each (`ls -1` over all eight returned all eight with no error):

```text
standards/pages/README.md
standards/pages/00-one-need.md
standards/pages/10-the-need-set.md
standards/pages/20-the-fold-line.md
standards/pages/40-reviewing-a-page.md
xtask/src/lint_pages.rs
.bklg/docs-that-teach/page-need-discipline/_design.md
.bklg/docs-that-teach/_discovery/distillation/interaction-patterns.md
```

`depends_on` / `related` ids, `rg -n "^id: <id>" .kb`, each a single hit in the permanent layers:

```text
.kb/playbooks/verify-the-referent-and-report-coverage.md:2:id: kb-playbook-verify-referent-report-coverage-001
.kb/playbooks/landing-a-stricter-gate-without-a-red-baseline.md:2:id: kb-playbook-ratchet-gate-landing-001
.kb/playbooks/anchoring-citations-in-a-long-lived-document.md:2:id: kb-playbook-anchoring-citations-001
.kb/governance/rewrite-the-referent-never-the-reasoning.md:2:id: kb-governance-referent-not-reasoning-001
```

The sweep also matched `.kb/_governance/integration-waves/**`, which are the wave records that
created those atoms; the canonical hit for each id is the atom file above.

### §Non-occlusion

```text
$ rg -n '<details>|<summary>|<!--' .kb/_intake/lesson-page-need-declaration-discipline.md
exit=1
```

No fold markup, no tab strip, no HTML comment. Nothing in the atom is revealed on hover or opened on
demand; every load-bearing region is persistent chrome, readable top to bottom in a GitHub blob view
with nothing clicked.

### §Method

The four steps are `<atom>:49-78`, each opening with a bold run-in numeral and each carrying its own
`**Evidence.**` clause naming a file, a constant or a captured message:

1. declare the need in the artefact's own visible body text (`:49`);
2. spell the token from a closed, enumerated set held in one place (`:57`);
3. check the declaration mechanically, and document what the check cannot see (`:64`);
4. supply a written non-author procedure for the judgement the check cannot make (`:72`).

**The stop conditions are carried by `## Where this is the wrong instrument` (`<atom>:121`)** — the
corpus's own heading for the obligation
(`.kb/playbooks/landing-a-stricter-gate-without-a-red-baseline.md:98`). Two of its four conditions,
quoted verbatim:

- *"**A medium that does not render the declaration in document order.** A renderer that strips
  leading blockquotes, relocates them into a metadata layer, or demands front matter invalidates the
  form, and the decision re-opens rather than quietly contradicting itself."*
- *"**A subject whose reference surface is not already owned elsewhere.** The set here subtracts a
  `reference` token because two reference surfaces already existed; where they do not, the
  subtraction is wrong and the set is a different set."*

The other two are *a corpus small enough that the router costs more than it saves* and *a corpus
nobody may edit*.

### §Rejection cross-walk

Five families, each compressed and cited rather than re-litigated. No sixth family appears.

| # | family | `<atom>` line | `_design.md` cite | how many, and the reason each lost |
| --- | --- | --- | --- | --- |
| 1 | the declaration's form | `:81` | `_design.md:128-147` | **five** — front matter, an HTML comment, a filename convention, a sidecar manifest, a badge or coloured admonition |
| 2 | the need set | `:88` | `_design.md:188-202` | **three** — a four-box taxonomy adopted literally, dropping the enumeration, a persona-keyed taxonomy |
| 3 | findability | `:95` | `_design.md:231-237` | **two** — routing as an implicit byproduct, a bespoke navigation widget |
| 4 | the fold line | `:99` | `_design.md:275-280` | **two** — reviewer judgement per page, a permanent ban on collapsible content |
| 5 | the tree's home | `:105` | `_design.md:88-96` | **four** — atoms in the code-standards tree, a hand-authored KB corpus, a subtree of the user documentation, nothing at all |

Every rejection in the atom maps to a row above, and every row maps to a rejection the design
actually took. **Residual:** this cross-walk was performed by the implementer, who drafted the atom.
A non-author walk is owed at this story's review and integration stages, and this row is not a
licence to skip it.

### §Commitment audit

```text
$ rg -n '\b(must|shall|MUST)\b' .kb/_intake/lesson-page-need-declaration-discipline.md
exit=1
```

**Zero hits**, so there is nothing to adjudicate line by line: the atom carries no commitment at
all, and the enforceable half of the discipline stays where the gate reads it — as `RP-NN-N` rules
in `standards/pages/`. The atom names no `RP-` id and therefore restates none; it cites the *atoms*
that carry them (`standards/pages/00-one-need.md`, `40-reviewing-a-page.md`) and the checker module,
which is the cite-never-restate discipline turned on this story's own output.

Anti-pattern 15 sweep, the same one the rules tree is held to:

```text
$ rg -in 'consider|use judgement|use judgment|as appropriate|if it seems' <atom>
exit=1
```

No hedge, so no procedure step hands back the judgement it exists to replace.

### §Portability

The atom's subject is the **shape**, and happenstance is the cited instance. `## What makes it
portable` (`<atom>:111`) states the transferable half as *"a mechanical check that verifies a
declaration, a written non-author procedure for the part it cannot verify, and the check's own
documentation naming the seam between them"*, and names **one recurrence outside this initiative**:
RS-81-1 (`standards/rust/81-checks-that-cannot-be-types.md:11`), which states the pairing as a rule
for **any** check, together with `kb-playbook-verify-referent-report-coverage-001`, where a check
verified an address and reported its coverage rather than asserting what it could not see. Neither
is about documentation.

**Verdict on `.kb/playbooks/README.md:33-35`'s test — does the claim survive being read by someone
working on a different part of the system?** Yes: the method reads as instructions for any corpus of
prose artefacts, the tokens are named as an instance rather than as the subject, and no step depends
on this repository's vocabulary. **Walker: the implementer, who drafted the atom, and the verdict is
labelled as such.** A genuine non-author portability walk is owed at this story's review and
integration stages; recording an author-run verdict is what makes that obligation visible rather
than assumed.

### §Divergence

The atom was written against the **shipped** `standards/pages/` corpus and `xtask/src/lint_pages.rs`
at this story's HEAD, then checked against `_design.md`. Three divergences, recorded rather than
smoothed over (`.kb/governance/rewrite-the-referent-never-the-reasoning.md`):

1. **The permitted-fold-mechanism list is a table, not a `const`.** `_design.md` discusses
   `PERMITTED_FOLD_MECHANISMS` as a named list; the shipped tree carries it as a `| Mechanism |`
   table in `standards/pages/20-the-fold-line.md` with zero data rows, and `xtask/src/lint_pages.rs`
   deliberately holds no such constant — a test asserts its **absence**, because an unenforced const
   beside an enforced one reads as a check that exists. The atom describes the table.
2. **The router's link check is `lint_pages.rs`'s own copy, not `lint_constitution.rs`'s.**
   `_design.md` cites `check_router` at `lint_constitution.rs:334-384` as the pattern; the shipped
   checker is a deliberate copy inside `xtask/src/lint_pages.rs` sharing no code with it (RS-81-3).
   The atom cites the shipped module.
3. **The governed set excludes `README.md`, which `_design.md` does not discuss.** Observed by the
   slice-mate `governed-page-cites-the-discipline`: `xtask/src/lint_pages.rs:496` excludes
   `README.md` from the walk, so a tree's own index is not a governed page. The atom does not claim
   otherwise — its method step 1 speaks of artefacts, not of every markdown file in a directory —
   and the fact is recorded here so a later reader of both documents is not surprised by it.

Nothing in the atom contradicts the signed-off design or the shipped tree.

### §Composition

Heading inventory (`rg -n '^#{1,2} ' <atom>`):

```text
36:# Making the need a page answers a declared, singular, checkable property
38:## The situation
47:## The method, in four steps
79:## The alternatives that lost
111:## What makes it portable
121:## Where this is the wrong instrument
```

Section order: `summary` before the body; the method before its rejected alternatives; the stop
conditions last — the corpus's own placement
(`.kb/playbooks/landing-a-stricter-gate-without-a-red-baseline.md:32-98`). Hierarchy is carried by
position, heading level, the `**Bold.**` run-in marker and monospace `path:line` citations, exactly
as the corpus does it.

Budgets:

```text
$ wc -c .kb/_intake/lesson-page-need-declaration-discipline.md
8184

$ awk '{if(length($0)>m)m=length($0)}END{print m}' <atom>
92
```

**8,184 ≤ 8,192** and **92 ≤ 100**, against a measured corpus of 5,161–7,137 bytes
(largest: `.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md` at 7,137) and a measured
corpus column maximum of exactly 100.

```text
$ rg -n '^> \*\*Answers:\*\*' <atom>
exit=1
```

**No governed-page declaration**, deliberately: this is a `.kb` atom, outside both pinned trees, and
a declaration here would claim a governance the checker does not extend to it. Greyscale: the atom's
only distinction carriers are heading level, bold run-in markers and monospace — no colour, icon,
emoji or badge appears anywhere in it, so nothing is lost in greyscale or in print.

### §Mechanical negatives

```text
$ git diff HEAD --stat -- .kb ':!.kb/_intake'
exit=0, no output

$ git diff HEAD --stat -- .kb/maps
exit=0, no output

$ rg -n "_intake/lesson-page-need" -g '!.bklg/**'
exit=1

$ redkiln validate --kb
redkiln: validate passed.

$ redkiln doctor 2>&1 | rg -c "template-drift|differs SUBSTANTIVELY"
6
```

Nothing landed in the checked knowledge base ahead of the wave; `.kb/maps/domain-map.md` is
untouched; no durable tree links a path that a successful ingest deletes. **These two green CLI runs
are AC-010's evidence and are not cited for AC-002, AC-003 or AC-005** — the validator is blind to
`_`-prefixed directories by design.

### Deviations from the plan

**One, and it is a size overrun fixed rather than waived.** The first draft was 10,324 bytes against
AC-011's 8,192 ceiling and 101 columns against its 100. The design's yield order for a page over
budget is *the overflow is the diagnosis — the page is answering a second need* — which does not
apply here: the atom's subject is single and the overflow was verbosity. Three compression passes
removed prose and no obligation. Both budgets are now met with 8 bytes of headroom, which is worth
saying plainly: a later edit to this file has almost no room and is a re-measurement, not a typo fix.

**`git diff main` is not the base for this story's negatives.** `main` predates this project's
merged dependency stories, so `git diff main -- .kb ':!.kb/_intake'` would report other stories'
work. The story-scoped diff against the slice base is the one that can prove this story's promise,
and it is what is captured above.
