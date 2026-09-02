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

| measurement | first draft | second draft | first merge | **shipped** |
| --- | --- | --- | --- | --- |
| `wc -c <atom>` (ceiling **8,192**) | **10,324** | 9,687 → 9,425 → 9,147 → 8,862 → 8,550 → 8,452 → 8,239 | 8,184 | **8,161** |
| max source column (ceiling **100**) | **101** | 92 | 92 | **91** |

The first draft failed both. Compression was applied to the prose and to none of the obligations:
the four method steps, their evidence, the five rejection families and the four stop conditions are
all still present, which is what AC-006, AC-007 and AC-009's checks confirm below.

**Red again at review, and green again after a fourth compression pass.** The adversarial slice
review found the atom in breach of AC-008's second half: it cited `standards/pages/*.md` **paths**
but no `RP-NN-N` **id**, so the clause of AC-008 that checks every cited id against the shipped rule
text for restatement had nothing to run over — and it was hiding a real restatement, RP-00-2's own
italicised reader's test reproduced verbatim at the then-`<atom>:51-52` from
`standards/pages/00-one-need.md:52-53`, plus a paraphrase of RP-00-2's imperative in the same
paragraph. Ten rule ids were added (`RP-00-1`, `RP-00-2`, `RP-10-1`, `RP-10-4`, `RP-20-1`, `RP-20-2`,
`RP-20-3`, `RP-20-4`, `RP-40-1`, `RP-40-2`), the restated sentence and the paraphrase were replaced
by citations, and a fourth compression pass paid for the added bytes — the file had 8 bytes of
headroom, so this was a re-measurement rather than a typo fix, exactly as the *Deviations* section
below warned it would have to be. Both budgets were re-run from scratch (§Composition) and the
restatement audit AC-008 asks for was run for the first time over ten real ids (§Commitment audit).

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
| `.kb/_intake/lesson-page-need-declaration-discipline.md` | **new, and the whole of the product.** 8,161 bytes. `kind: playbook`, `status: proposed`, `authority_tier: guideline`; ten required frontmatter fields; five body sections — `## The situation`, `## The method, in four steps`, `## The alternatives that lost`, `## What makes it portable`, `## Where this is the wrong instrument`, under one H1; ten `RP-NN-N` ids cited and none restated |
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
$ git diff --name-status main -- .kb/_intake
A	.kb/_intake/lesson-page-need-declaration-discipline.md

$ git diff --name-status --cached HEAD -- .kb/_intake
A	.kb/_intake/lesson-page-need-declaration-discipline.md

$ ls .kb/_intake
README.md
lesson-page-need-declaration-discipline.md
```

The spec's `main`-based command and the commit-scoped one agree, because no earlier story in this
project touched `.kb/` — see §Mechanical negatives for the same two bases run over the whole tree.
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
`**Evidence.**` clause naming a file, a rule id, a constant or a captured message:

1. declare the need in the artefact's own visible body text (`:49`);
2. spell the token from a closed, enumerated set held in one place (`:57`);
3. check the declaration mechanically, and document what the check cannot see (`:65`);
4. supply a written non-author procedure for the judgement the check cannot make (`:73`).

**The stop conditions are carried by `## Where this is the wrong instrument` (`<atom>:123`)** — the
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
| 1 | the declaration's form | `:82` | `_design.md:128-147` | **five** — front matter, an HTML comment, a filename convention, a sidecar manifest, a badge or coloured admonition |
| 2 | the need set | `:89` | `_design.md:188-202` | **three** — a four-box taxonomy adopted literally, dropping the enumeration, a persona-keyed taxonomy |
| 3 | findability | `:96` | `_design.md:231-237` | **two** — routing as an implicit byproduct, a bespoke navigation widget |
| 4 | the fold line | `:100` | `_design.md:275-280` | **two** — reviewer judgement per page, a permanent ban on collapsible content |
| 5 | the tree's home | `:107` | `_design.md:88-96` | **four** — atoms in the code-standards tree, a hand-authored KB corpus, a subtree of the user documentation, nothing at all |

Every rejection in the atom maps to a row above, and every row maps to a rejection the design
actually took.

**Who walked it, and when.** The walk that carries this row was performed by the **adversarial slice
reviewer for `binding-beyond-this-project`** — the review pass that produced this slice's findings —
on **2026-08-18**. That actor drafted no part of the atom, of this report or of `_ledger.md`, which
is the bar AC-007's `verifying_test` states. Their finding, transcribed: *five families, each
mapping to a real design rejection at the cited ranges (`_design.md:88-95`, `:128-150`, `:188-202`,
`:231-237`, `:275-280`), no sixth family invented, none attributed a reason the design does not
state.* Two of the walker's ranges are read one boundary wider than the atom's own — the atom cites
`:88-96` where the walker wrote `:88-95`, and `:128-147` where the walker wrote `:128-150` — and the
atom's are the tighter, verified ends of the same two rejection blocks (`_design.md:96` is the
*nothing at all* clause, `:148-150` is the hosting-assumption paragraph that follows the five
rejected forms). The families are the same five either way, which is what the criterion asks.

**What this replaced, and why.** This row previously recorded a walk by the implementer, who drafted
the atom. AC-007's criterion requires a non-author, so that capture is not evidence and has been
removed from the ledger row; this paragraph is the audit trail of the state the row was in, kept per
the precedent one slice back (`reviewer-and-citation-procedures/_ledger.md:84`).

### §Commitment audit

```text
$ rg -n '\b(must|shall|MUST)\b' .kb/_intake/lesson-page-need-declaration-discipline.md
exit=1
```

**Zero hits**, so there is nothing to adjudicate line by line: the atom carries no commitment at
all, and the enforceable half of the discipline stays where the gate reads it — as `RP-NN-N` rules
in `standards/pages/`.

**The restatement audit, run for real over ten cited ids.** AC-008's second clause asks that *every
`RP-` id in the atom* be checked against the shipped `standards/pages/` rule text for restatement.
At first merge the atom cited zero ids, so that clause had nothing to run over — and it was hiding
one. The atom now cites ten, each beside the file that carries it, and each is diffed below against
the shipped rule. The discriminator is RP-40-2's own: *is this sentence's authority a visible,
resolving id, or a restatement of the rule's content?*

```text
$ rg -n 'RP-[0-9]{2}-[0-9]' .kb/_intake/lesson-page-need-declaration-discipline.md
52:than repeated here. **Evidence.** RP-00-1 (the count) and RP-00-2 (the position, and the
62:could delete; in `standards/pages/10-the-need-set.md`, RP-10-1 governs how the token is
63:spelled and RP-10-4 how the set is changed.
76:`standards/pages/40-reviewing-a-page.md`, RP-40-1 carries the walk, its four verdicts and
77:the constraints on who may run it and what they may consult; RP-40-2 the paraphrase spot
103:shipped is RP-20-1's test, RP-20-2's closed class list, RP-20-3's empty mechanism table
104:and RP-20-4's asymmetry between the two — folding forbidden in practice today, with the
```

| id | shipped rule text (the imperative it is filed under) | what the atom says beside the id | verdict |
| --- | --- | --- | --- |
| `RP-00-1` | *Declare exactly one need on every governed page.* (`00-one-need.md:14`) | "RP-00-1 (the count)" | **cite** — names the subject, reproduces no clause |
| `RP-00-2` | *Put the declaration immediately after the H1 and interpose nothing.* (`:50`), whose **Why.** carries the italicised reader's test at `:52-53` | "RP-00-2 (the position, and the reader's test it protects)" | **cite** — the test is pointed at, not reproduced |
| `RP-10-1` | *Declare a token from the set above, spelled exactly.* (`10-the-need-set.md:49`) | "RP-10-1 governs how the token is spelled" | **cite** — the exactness rule (no trimming, no case folding, no aliasing) appears nowhere in the atom |
| `RP-10-4` | *Change the set in one commit, never in two.* (`:149`) | "RP-10-4 how the set is changed" | **cite** — the two-part-commit procedure is not stated here |
| `RP-20-1` | *Apply the deletion test to every collapsed region.* (`20-the-fold-line.md:16`) | "RP-20-1's test" | **cite** — the test's own question is not written out |
| `RP-20-2` | *Never fold one of the five classes, and grant no exception.* (`:42`) | "RP-20-2's closed class list" | **cite** — the five classes are not enumerated in the atom |
| `RP-20-3` | *Fold only with a mechanism on the permitted table, which is empty.* (`:87`) | "RP-20-3's empty mechanism table" | **cite** — the four earning observations are not restated |
| `RP-20-4` | *Grow the mechanism table with evidence; never shrink the class list.* (`:121`) | "RP-20-4's asymmetry between the two" | **cite** — a label for the rule, not its content |
| `RP-40-1` | *Reach a verdict on a page you did not write, from the page alone.* (`40-reviewing-a-page.md:14`) | "RP-40-1 carries the walk, its four verdicts and the constraints on who may run it and what they may consult" | **cite** — the six walk steps, the consequence mapping and the four verdict names are all absent from the atom |
| `RP-40-2` | *Spot-check the set for paraphrase, sentence by sentence.* (`:68`) | "RP-40-2 the paraphrase spot check" | **cite** — the procedure's name, not its text |

**The restatement this audit was written to catch, and how it was repaired.** At first merge the
atom's step 1 read *"the test is read nothing but the region above the first prose paragraph and
name the need"* — RP-00-2's own italicised sentence from `standards/pages/00-one-need.md:52-53`,
reproduced verbatim and attributed to a file path rather than to a rule id. The same paragraph
paraphrased RP-00-2's imperative as "nothing interposed" against the rule's "interpose nothing".
Both are gone: the sentence is replaced by the `RP-00-2` citation above, and the paragraph now says
only that "the count and the position are both load-bearing, and both are cited rather than repeated
here" (`<atom>:51-52`). `rg -n 'read nothing but the region' <atom>` → exit 1.

The remaining shared vocabulary between the atom and the tree is the **subject being pointed at** —
the word *token*, the word *need*, the word *walk* — which is the same discriminator the slice-mate
applied to its own citations (`governed-page-cites-the-discipline/_ledger.md` AC-003): the only
shared token may be the thing named, never the rule's reasoning.

Anti-pattern 15 sweep, the same one the rules tree is held to:

```text
$ rg -in 'consider|use judgement|use judgment|as appropriate|if it seems' <atom>
exit=1
```

No hedge, so no procedure step hands back the judgement it exists to replace.

### §Portability

The atom's subject is the **shape**, and happenstance is the cited instance. `## What makes it
portable` (`<atom>:113`) states the transferable half as *"a mechanical check that verifies a
declaration, a written non-author procedure for the part it cannot verify, and the check's own
documentation naming the seam between them"*, and names **one recurrence outside this initiative**:
RS-81-1 (`standards/rust/81-checks-that-cannot-be-types.md:11`), which states the pairing as a rule
for **any** check, together with `kb-playbook-verify-referent-report-coverage-001`, where a check
verified an address and reported its coverage instead of what it could not see. Neither is about
documentation.

**Who walked it, and when.** The portability test of `.kb/playbooks/README.md:33-35` was performed
by the **adversarial slice reviewer for `binding-beyond-this-project`** — the review pass that
produced this slice's findings — on **2026-08-18**. That actor drafted no part of the atom, of this
report or of `_ledger.md`, which is the non-author bar AC-009's `verifying_test` states, and the
identity is recorded here because AC-009 asks for it by name.

**Verdict: PASSES.** Transcribed from the walker: *the subject is the shape, happenstance is the
cited instance, and RS-81-1 (`standards/rust/81-checks-that-cannot-be-types.md:11`) is a genuine
recurrence on a subject that is not documentation.* The method reads as instructions for any corpus
of prose artefacts; the need tokens are named as an instance rather than as the subject; no step
depends on this repository's vocabulary.

**What this replaced, and why.** This row previously recorded a portability verdict reached by the
implementer, who drafted the atom. AC-009's criterion requires a non-author and requires the
walker's identity in the ledger, so that capture is not evidence and has been removed from the
ledger row; this paragraph is the audit trail of the state the row was in, kept per the precedent
one slice back (`reviewer-and-citation-procedures/_ledger.md:84`).

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
80:## The alternatives that lost
113:## What makes it portable
123:## Where this is the wrong instrument
```

Section order: `summary` before the body; the method before its rejected alternatives; the stop
conditions last — the corpus's own placement
(`.kb/playbooks/landing-a-stricter-gate-without-a-red-baseline.md:32-98`). Hierarchy is carried by
position, heading level, the `**Bold.**` run-in marker and monospace `path:line` citations, exactly
as the corpus does it.

Budgets:

Re-measured from scratch after the review pass added the ten rule ids and removed the restatement —
not carried forward from the first merge, because the file had 8 bytes of headroom and every added
citation had to be paid for in prose:

```text
$ wc -c .kb/_intake/lesson-page-need-declaration-discipline.md
8161

$ awk '{if(length($0)>m)m=length($0)}END{print m}' <atom>
91
```

**8,161 ≤ 8,192** and **91 ≤ 100**, against a measured corpus of 5,161–7,137 bytes
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

**The `main`-based commands the spec names were also run, and they pass.** The base note below
explains why the commit-scoped diff is the stronger evidence for a *story*-scoped promise, but the
substitution turned out to be unnecessary for `.kb`, because no earlier story in this project touched
that tree at all:

```text
$ git diff --stat main -- .kb ':!.kb/_intake'
exit=0, no output

$ git diff --stat main -- .kb/maps
exit=0, no output

$ git diff --name-status main -- .kb/_intake
A	.kb/_intake/lesson-page-need-declaration-discipline.md

$ ls .kb/_intake
README.md
lesson-page-need-declaration-discipline.md
```

Both bases agree, so AC-001's and AC-010's `verifying_test` fields name the `main`-based commands
that were actually run, with the commit-scoped pair recorded beside them (NF-004: the recorded
command matches the command run).

Nothing landed in the checked knowledge base ahead of the wave; `.kb/maps/domain-map.md` is
untouched; no durable tree links a path that a successful ingest deletes. **These two green CLI runs
are AC-010's evidence and are not cited for AC-002, AC-003 or AC-005** — the validator is blind to
`_`-prefixed directories by design.

### Deviations from the plan

**One, and it is a size overrun fixed rather than waived.** The first draft was 10,324 bytes against
AC-011's 8,192 ceiling and 101 columns against its 100. The design's yield order for a page over
budget is *the overflow is the diagnosis — the page is answering a second need* — which does not
apply here: the atom's subject is single and the overflow was verbosity. Three compression passes
removed prose and no obligation, landing at 8,184 bytes — **8 bytes of headroom**, which this
section warned was a re-measurement rather than a typo fix for whoever edited next.

**And whoever edited next was the review pass, which is exactly the cost that warning priced.**
Adding ten `RP-NN-N` citations and removing RP-00-2's restated sentence needed a fourth compression
pass over the situation, both procedural steps, three rejection families and the portability section;
the shipped file is **8,161 bytes / 91 columns**, re-measured from scratch in §Composition rather
than adjusted arithmetically. Headroom is now 31 bytes, which is still small enough that the same
warning stands.

**`git diff main` is not the base this story's negatives were captured against.** `main` predates
this project's merged dependency stories, so a `main`-based diff can report other stories' work; the
commit-scoped diff is what can prove *this* story's promise. For `.kb`, however, both bases were run
and both are empty — no earlier story in this project touched the knowledge base — so AC-001's and
AC-010's `verifying_test` fields name the `main`-based commands, which are the ones the spec asks for
and the ones that were run (§Mechanical negatives).
