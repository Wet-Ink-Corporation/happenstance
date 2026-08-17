---
item: HS-S0148
stage: spec
created: 2026-08-17T13:16:08.737Z
updated: 2026-08-17T13:16:08.737Z
template_sig: 87bbf1d0
rendered_sig: 7b7cd1a1
---

# Spec — DT-8 resolved as a rule a reviewer can apply without the author

## Scope lock

| Level | Path and the part that binds this story |
| --- | --- |
| Initiative (gold source) | `.bklg/docs-that-teach/initiative.md` — BR-11's rule half; **DoD-13** (`:458-462`, "Nothing load-bearing is hidden from the check … **or** no such content carries a load-bearing claim") and DoD-14; the non-goal against extending the precedence chain |
| Initiative decomposition | `.bklg/docs-that-teach/_decomposition.md` — the BR-11 split: **this project states what may never be folded; HS-P0020 demonstrates whether a fold is checked** |
| Project | `.bklg/docs-that-teach/page-need-discipline/project.md` — **AC-005**; **DR-08**; the risk row "the discipline becomes a second specification"; the non-goal on DT-7 |
| Briefs (key) | `.bklg/docs-that-teach/page-need-discipline/_decomposition.md` — Architecture brief Note 3 items 3 and 5, Note 4 (no `rust` fence), Note 6 (no shared abstraction); **UX brief UX-003, UX-011** and Note 3 ("the accessibility floor, and why it is written as a burden of proof" — *"do not implement a fold-checker here"*); Testing brief AC-003/004/005 (procedural tier) and its closing paragraph (*"this brief states only what may never be folded, never whether a fold is itself checked"*) |
| Signed-off design (**BINDING**) | `.bklg/docs-that-teach/page-need-discipline/_design.md` — **`### S1/S3 — DT-8: the fold line`** (Parts 1, 2, 3, and its "Failure mode and mitigation"), `## Surfaces` (`rule-atom`, and the pinned `RULE_DIR` table), `## Composition` S3, `## Density budget`, `## Transience policy` rows S3, `## Anti-patterns` 5, 6, 13, 14, `## Mock` **finding 2**, `## Sign-off` condition 3 |
| Story map | `.bklg/docs-that-teach/page-need-discipline/_storymap.md` — slice `discipline-on-disk`; this story's row; Coverage (`AC-005 … sole owner`); merge order 1.3 |
| Grounding | `.bklg/docs-that-teach/page-need-discipline/_grounding.md` — "DT-2, DT-3, DT-8 — primary evidence, quoted directly"; **no Accepted decision atom constrains this project and none may be written for it** |
| Primary evidence | `.bklg/docs-that-teach/_discovery/distillation/interaction-patterns.md:404-410` (the deletion test), `:484-497` (tension 2 = DT-8, and the "use good judgment" non-answer), `:205-216` (the `mdbook-tabs` unknown — *"an unverified property, not a confirmed safe one"*), `:220-225` (the gate step that looked wired and wasn't) |
| Roadmap pointer | **None.** `RUNBOOK.md` carries no phase for this initiative. The roadmap of record is `_storymap.md`'s merge order; this story is 1.3, after the router and independent of `reviewer-and-citation-procedures`. |
| Design mock (built, awaiting sign-off) | `.bklg/docs-that-teach/page-need-discipline/design/mock.html` — the `rule-atom` frames (`populated`, `over-rule-ceiling`, `over-byte-ceiling`, `missing-section`, `rust-tagged-fence`) and the closing findings panel |

## One-line PR slice

Resolve DT-8 and write it as a rule a reviewer applies without the author — *if
the collapsed section were deleted, would the page still teach the constraint
correctly?* — covering the declaration itself, which may never be occluded.

## Executive summary

**What this PR lands.** `standards/pages/20-the-fold-line.md` — band 20 of the
rules tree — plus the two edits to `standards/pages/README.md` that mount it: one
row inside the generated `## Index` region and one row in the `## Start here`
filter. After this PR a reviewer holding a page that puts a `MUST`, an invariant,
a runnable fence, a "what this does not verify" sentence, or the page's own
`> **Answers:**` declaration behind a `<details>`, a tab or an accordion can reach
a **verdict** from the rule's own text, without asking the author what they
intended and without a byte count that does not exist.

**Pointer, not restatement.** DT-8 is **already resolved**, in three parts, in the
signed-off `_design.md`; its three rejected options are already written down with
the cost of each. This story does not re-decide it and does not re-argue it. What
this PR converts is *a design decision* into *an applied rule* in the tree's own
`## RP-NN-N.` grammar — the imperative sentence, the five fixed sections, and the
named wrong page that could plausibly ship.

**The delta worth reading twice.** Four things are decided *here* rather than left
to the implementer, and each survives beyond this PR:

1. **`PERMITTED_FOLD_MECHANISMS` is a table in the atom, not a `const` in
   `xtask/src/lint_pages.rs`.** A `const` implies a checker; there is none, and
   the brief forbids building one here.
2. **The list ships empty, so folding is forbidden in practice** — a decision
   (`_design.md` sign-off condition 3), not an omission, and the atom must say so
   in those words and name what would lift it.
3. **The mock's finding 2 is paid in this PR.** rustdoc wraps every page in
   `<details class="toggle top-doc" open>` and ships `#toggle-all-docs`; DT-8
   Part 3 owes a sentence on whether a renderer-supplied, open-by-default wrapper
   is a *mechanism*. It is answered here, and answered so that UX-011 still holds.
4. **The two lists have opposite openness on purpose** — never-fold closed, mechanisms
   open-by-evidence — and the atom states the amendment procedure for each, because
   the failure this rule exists to prevent is an edge case resolved by a one-off
   reviewer exception.

## Context pack

The decisions this story must honor, stated inline. Everything deeper is a
signposted anchor in the second half of this spec — link, do not paste.

**DT-8 is resolved and this story is downstream of the resolution.** `_design.md`
chose **option (a), a stated rule, in three parts**, and recorded why a one-clause
rule is the version that is wrong at the edges. Part 1 is the **deletion test** as
the rule's *spirit* — *if the collapsed region were deleted, would the page still
teach the constraint correctly?* — taken verbatim from NN/g via
`interaction-patterns.md:404-410`, which is the form `project.md`'s DR-08 already
names. Part 2 is the **closed never-fold list** as the rule's *letter*, five
classes, **no reviewer may grant an exception**. Part 3 is the **permission gate**:
a class not on the never-fold list is *eligible* to be folded and is **permitted**
only if the mechanism appears on `PERMITTED_FOLD_MECHANISMS`. The two rejected
options and why each lost are already on record — **(b) reviewer judgement per
page**, which is the drift the tension names in its own words (*"'use good
judgment' is the same non-answer that let the code-layer invariant drift in the
first place"*, `interaction-patterns.md:484-497`), and **forbidding collapsible
content permanently**, which is over-broad because an exercise answer and a long
output transcript are real. Do not re-derive either into the atom's prose; cite the
decision and spend the atom's bytes on the *applied* rule.

**The five never-fold classes are fixed, and class 1 discharges a deferral a
slice-mate already wrote.** They are: (1) the page's own `> **Answers:**`
declaration; (2) any sentence carrying a normative modal or citing a
`spec/SPECIFICATION.md` clause id; (3) any statement of an invariant, constraint or
precondition; (4) the only occurrence of a code fence the reader is expected to
run; (5) any statement of what a check does *not* verify. Class 1 is not
decorative bookkeeping: `need-vocabulary-and-declaration-form` authored
`standards/pages/00-one-need.md` with the never-occluded rule **and the mechanism
question explicitly deferred to band 20** (that story's PR boundary says so). Band
20 is where the deferral is paid, so the atom's `> **See also:**` names band 00 —
**by number, not by link**, which is the constitution's own spelling
(`standards/rust/00-prime-directives.md:7-9`) and the spelling the slice-mates
adopted. Class 5 is the one that closes a loop with the repository's own gate
discipline: RS-81-1 requires a check's blind spot to be stated in its own
documentation because *"a check whose limits are undocumented is read as a
guarantee"* (`standards/rust/81-checks-that-cannot-be-types.md:11`), and a blind
spot behind a disclosure is an undocumented blind spot wearing a costume.

**`PERMITTED_FOLD_MECHANISMS` lives in the rules tree, as a table in this atom —
not as a Rust `const`.** `_design.md` says "an explicit `PERMITTED_FOLD_MECHANISMS`
list **in the rules tree**". The tempting alternative — a `const
PERMITTED_FOLD_MECHANISMS: &[&str]` beside `NEEDS` in `xtask/src/lint_pages.rs` —
is rejected for a reason that is structural rather than stylistic: a `const` in the
checker module is read as a thing the checker enforces, and **nothing enforces it**.
The architecture brief states the limit as item 3 of the checker's own "what this
does not verify" — *"the fold rule (DT-8) is enforced only to the extent the chosen
markers are textual"* — and the UX brief closes Note 3 with the seam in one
sentence: *"this project states the rule; HS-P0020 owns the demonstration … Do not
implement a fold-checker here."* An unenforced `const` next to an enforced one is
the decorative-gate-step failure (`RUNBOOK.md:920-925`, `:931-935`) recreated at the
vocabulary layer. The `NEEDS` precedent does not transfer: `NEEDS` has two named
machine consumers this initiative builds (`_storymap.md`, coverage of AC-003/AC-004);
`PERMITTED_FOLD_MECHANISMS` has none and will not until HS-P0020's DT-7
demonstration exists.

**The list ships empty, and the atom must say what that means and what lifts it.**
`_design.md` states it plainly — *"As of this design, that list is empty, and
folding is therefore forbidden in practice"* — and sign-off condition 3 carries it
as an attached condition rather than a waiver: *"If that is not the intent, DT-8's
Part 3 is what to disagree with."* A mechanism reaches the list only with a
**recorded observation in this repository** that its content (i) sits correctly in
the accessibility tree, (ii) is keyboard operable, (iii) is found by Ctrl-F, and
(iv) is found by print. That burden of proof is UX-011's second sentence — **an
unverified property counts as unmet** — and it is earned, not borrowed: nothing in
`mdbook-tabs`' own documentation states whether an inactive panel is in
`mdbook test`, the search index, or Ctrl-F/print, which the dossier calls *"an
unverified property, not a confirmed safe one"* (`interaction-patterns.md:205-216`),
and this workspace *has already shipped* a gate step that looked wired and was not
(`:220-225`; the incident at `RUNBOOK.md:931-935`). So the atom states the four
conditions as an entry procedure, names the first entry as HS-P0020's DT-7
demonstration to earn, and states that upstream documentation is not an observation.

**Mock finding 2 is this story's to pay, and the answer must not quietly break
UX-011.** rustdoc wraps every page in `<details class="toggle top-doc" open>` and
ships a `#toggle-all-docs` control that closes it (`_design.md`, `## Mock`,
finding 2), so under rustdoc hosting the declaration — never-fold class 1 — already
sits inside a disclosure nobody has observed. The decision this story writes down:
**RP-20-2 binds the author's own markup**, so a renderer-supplied, open-by-default
wrapper is not an authored fold and does not put a page in breach; **and** because
the reader can collapse it and nobody in this repository has checked what survives
that, the wrapper is recorded in the atom as a **named unmet property owed by
HS-P0020's hosting decision**, not as a silent assumption. Both halves are required.
Saying only the first turns "we did not check" into "it is fine", which is exactly
the move UX-011 exists to refuse; saying only the second makes every rustdoc-hosted
page in the repository retroactively non-conformant to a rule about author choices.

**The atom is a rule atom, and the grammar is already enforced elsewhere.**
`# 20 — The fold line` · a one-source-line `> **Load when:**` · `> **See also:**`
naming sibling bands by number · `---` · then one `## RP-20-N. <imperative
sentence>` per rule, each followed by exactly five sections in fixed order —
**Why.** · **Do** · **Not** · **Rejects.** · **Evidence.**
(`standards/rust/00-prime-directives.md:1-9,19,30,54,78`;
`standards/rust/README.md:99-107`; the marker list at
`xtask/src/lint_constitution.rs:67-81`). **`Load when:` occupies exactly one source
line** — the parser reads only the first line of the block and silently drops
continuations (`xtask/src/lint_constitution.rs:247-257`), visible today as
mid-phrase index rows in the constitution's own router
(`standards/rust/README.md:95-96`, "asked to 'wire up' a"), and
`router-precedence-and-announcement` states the constraint in
`## The shape of a rule` precisely so the slice-mates author against it. Ceilings,
all from `_design.md`'s Density budget and calibrated against the existing corpus:
**≤ 6 rules** and **≤ 16,384 bytes** per atom (identical to `MAX_RULES_PER_ATOM`
and `MAX_ATOM_BYTES`, `xtask/src/lint_constitution.rs:88,95`, on purpose — two
trees teaching two different numbers for the same idea is its own defect), prose
**≤ 96 columns** outside tables, and every `**Rejects.**` clearing **120
characters** (`MIN_REJECTS_CHARS`, `:82`). Two deliberate differences from the
constitution's atoms, both consequences of this tree not being registered with the
doctest harness (`xtask/src/lib.rs:24-28` registers only `mod constitution`):
**`Do` and `Not` hold markdown page fragments, not Rust**, fences tagged `text` or
`markdown` — a `rust`-tagged fence is a Rust claim nothing compiles (`_design.md`
anti-pattern 13) and an untagged fence is rejected too, so a future decision to
register the tree cannot be undermined retroactively; and **`Evidence.` cites
`path:line` in this repository first**, then the dossier, then external URLs with a
`*(checked …)*` stamp.

**Mounting is a router edit, and the router's index has a byte-exact contract.** An
atom that exists and is not in the router is a rule nobody can find — the same
half-mount the story map refuses in its first "Why the slices fall here" bullet.
So this PR appends **one row inside the generated `## Index` region** of
`standards/pages/README.md` and **one row to `## Start here`**. The index row is not
free-form: `router-precedence-and-announcement` fixed the shape one slice before the
generator exists, to the format `generated_region` builds at
`xtask/src/lint_constitution.rs:400-420` — a markdown link to the atom file, the
atom's first `Load when` source line, and its comma-separated rule ids, with every
interior `|` escaped and no blank line inside the markers. Get it wrong and
`page-need-checker-mounted-in-the-gate`'s first `--write` produces a diff nobody
intended, which is that story's AC and this story's obligation. `## Start here` has
a **12-row ceiling**; adding a row must not breach it, and the router's own
**8,192-byte** ceiling must still hold after the edit.

**No Accepted decision atom constrains this story, and none may be written for
it.** All seventeen under `.kb/decisions/` were read by title at grounding; none
governs documentation trees, fold mechanics or narrative conventions. The binding
authority is sub-ADR: `CLAUDE.md`, `standards/rust/README.md:23-29`'s precedence
chain, and `_design.md`. Writing an ADR here would itself breach the
initiative-level non-goal against extending that chain
(`_grounding.md`, "Tensions / risks to flag in the briefs", item 1).

**The persona-journey slice this realizes.** One reader, and the whole rule is
shaped around what they are *not* allowed to need.

- **The non-author reviewer, holding a page they did not write.** Project AC-005's
  own wording is the bar: the rule must be *"in a form a reviewer applies to a page
  without consulting the author"*, and UX-009 sharpens it — *performable by a
  stranger, in reading order, with no access to the author, no repository
  archaeology and no mouse*, yielding **a verdict, not an impression**. Every rule
  in this atom is therefore a question with a yes/no answer; the words "consider",
  "use judgement", "as appropriate" and "if it seems" are the named failure
  (`_design.md` anti-pattern 15), and they are the failure because they are the
  exact register of the non-answer the tension quotes.
- **The author who folded one of two statements of the same invariant.** DT-8's
  evidence is not hypothetical for this repository: the tension records *"this
  project's own precedent of a constraint that drifted out of sync once only one of
  its two statements stayed visible"*. The rule's job is to make that page fail
  review by *class membership*, before anyone has to remember which copy is stale.

**What this story is forbidden from doing, stated as decisions.** No fold-checker,
no marker scanner, no `xtask/src/**` edit of any kind — the enforcement seam is
HS-P0020's DT-7 and the checker story's, and the UX brief says *do not implement a
fold-checker here* in those words. No re-argument of DT-8's rejected options —
`_design.md` owns them and duplicating them into the atom spends the byte ceiling
on prose already written down. No new never-fold class invented at authoring time:
the list is **closed at five**, and an edge case grows the *mechanism* list, never
shrinks the class list. No `rust`-tagged or untagged fence. No `<details>`, tab
strip or accordion inside this atom itself — an atom about folding that folds is
`_design.md` anti-pattern 6 self-applied. And no restatement of a
`spec/SPECIFICATION.md` clause: class 2 names clause citations as never-foldable
and must do so **by citing**, because the discipline's own rule against becoming a
second specification (DR-09, band 30) applies to the band that writes about
citations first.

## Integration contract

**Slice / milestone.** `discipline-on-disk`. Slice-mates, implemented in one
context and mounted as one surface: `need-vocabulary-and-declaration-form` (bands
00/10, the `NEEDS` const, the path pins), `router-precedence-and-announcement`
(this story's dependency — the router, the band table, the generated region's
shape), `reviewer-and-citation-procedures` (bands 30/40, and the walk that *cites*
this rule).

**Archetype.** `capability` — a user-observable slice. The observable user is a
non-author reviewer who reaches a verdict on a folded claim from the tree alone.

**Mount point.** **`standards/pages/README.md`** — the router created by this
story's dependency, and the only composition root a rule atom has in this tree. Two
regions, both required:

- the generated `## Index` region between `<!-- BEGIN GENERATED -->` and
  `<!-- END GENERATED -->` — one new row for `20-the-fold-line.md`, in the byte-exact
  format `xtask/src/lint_constitution.rs:400-420` builds;
- the `## Start here` filter — one new row keyed to the reviewer's intent, inside
  the 12-row ceiling.

A rule atom with no index row is reachable only by `ls`, which is the failure
UX-004 forbids from the other direction (a filter that hides what it filters) and
`_design.md` anti-pattern 8 forbids by counting (an index whose row count disagrees
with the file count).

**Wires into** (real siblings, by path):

| Consumed | Path | What this story takes from it |
| --- | --- | --- |
| the router and its two mount regions (**dependency**) | `standards/pages/README.md` — created by `router-precedence-and-announcement` | the `## Index` markers, the row format, the `## Start here` table and its 12-row ceiling, the band-20 row of the band table this atom fills |
| the generated-region contract | `xtask/src/lint_constitution.rs:400-420`, `:388-397` | the exact row shape the checker's `--write` will regenerate to — link cell, first `Load when` line, comma-separated rule ids |
| the `Load when` parser's limit | `xtask/src/lint_constitution.rs:247-257` | why this atom's `Load when:` is one source line, and why a continuation would truncate its index row |
| the atom grammar and its ceilings | `standards/rust/00-prime-directives.md:1-9,19,30,54,78`; `standards/rust/README.md:99-107`; `xtask/src/lint_constitution.rs:67-81,82,88,95` | the head, the five fixed sections, `MIN_REJECTS_CHARS`, `MAX_RULES_PER_ATOM`, `MAX_ATOM_BYTES` |
| band 00's deferral (**slice-mate**) | `standards/pages/00-one-need.md` | the never-occluded rule whose *mechanism* question band 20 answers; class 1 of the never-fold list |
| the path pins (**slice-mate**) | `xtask/src/lint_pages.rs` — `RULE_DIR = "standards/pages"` | the address this atom is written to; not edited here |
| RS-81-1, for class 5 | `standards/rust/81-checks-that-cannot-be-types.md:11` | *"a check whose limits are undocumented is read as a guarantee"* — the reason a "what this does not verify" statement may never be folded |
| the accessibility burden of proof | `_decomposition.md`, UX brief UX-011 and Note 3 | the four recorded observations a mechanism needs, and *unverified counts as unmet* |

**Design-system primitives consumed.** Textual, from the UX brief's Note 1 table
and enforced today for the sibling tree: `# NN — Title`; the
`> **Load when:**` / `> **See also:**` head; `---`; `## RP-NN-N. <imperative
sentence>`; the five fixed sections **Why.** · **Do** · **Not** · **Rejects.** ·
**Evidence.**; the rules-and-bytes ceilings; the generated-region markers; the
`| You are… | Load |` intent-keyed row. The rule-id prefix is **`RP-`** — `RS-` is
the constitution's and `PS-` is a live `spec/SPECIFICATION.md` clause family
(`_design.md`, `## Surfaces`).

**Renders surfaces** (ids from `_design.md`'s `## Surfaces`):

- **`rule-atom`** — one instance, `standards/pages/20-the-fold-line.md`, in state
  `populated`, inside every Density-budget ceiling, and explicitly **not** in states
  `over-rule-ceiling`, `over-byte-ceiling`, `missing-section` or `rust-tagged-fence`.
- **`discipline-router`** — *changed, not created*: two rows added, state stays
  `populated`, and the `generated-region-stale` and `dangling-link` states must not
  be entered by this PR's edit.
- **`page-need-declaration`** — **constrained normatively, rendered nowhere here.**
  RP-20-2 class 1 governs it; its route is HS-P0020's unpinned `PAGE_DIR`.
- **`lint-terminal-output`** and **`reviewer-procedure`** — not this story's.

**Conformance rule(s).** **None, and this is not adapter-observable.** No port, no
crate, no feature is in this diff; `happenstance-testkit`'s suite observes stores,
and a markdown rule atom is invisible to it. The instruments that *do* observe this
story arrive later and are named so the absence is scheduled rather than silent:
the checker's `check_router` analogue (index-row equality and link resolution) and
its shape checks land with `page-need-checker-mounted-in-the-gate`; the fold rule's
*content* is observed by a human running band 40's walk
(`reviewer-and-citation-procedures`), and by HS-P0020's DT-7 demonstration if a
mechanism ever earns a place on the permitted list. A story that changes a port and
names no rule is a port change nothing can fail; this story changes no port.

**Clause(s).** **None discharged, none amended.** This atom *cites* the existence
of clause ids as a never-fold class and restates no clause;
`cargo xtask spec-trace` remains the only writer of `spec/SPECIFICATION.md`'s
generated sections and must still be green at merge. Clause ids are stable names
and are never renumbered (`spec/SPECIFICATION.md:280`), which is why class 2 can be
written about ids at all.

**Advances DoD scenario.** **DoD-13** — *"Nothing load-bearing is hidden from the
check … **or** no such content carries a load-bearing claim"*
(`.bklg/docs-that-teach/initiative.md:458-462`). This story is what makes the
**second disjunct** the initiative's default outcome by rule rather than by luck:
with `PERMITTED_FOLD_MECHANISMS` empty and the never-fold list closed, no shipped
page may carry a load-bearing claim behind a fold at all. The first disjunct — a
hidden branch demonstrated to be inside the checked surface — stays HS-P0020's
DT-7, and this atom is where its demonstration would be recorded if it succeeds.
Secondarily **DoD-14**, by adding the band the discipline's own account was
missing.

## PR boundary

**In this PR**

- `standards/pages/20-the-fold-line.md` — new. Band 20: the atom head, then the
  rules of DT-8's three parts, the closed never-fold list, the
  `PERMITTED_FOLD_MECHANISMS` table (shipping empty, with its entry procedure), the
  renderer-supplied-wrapper sentence, the amendment asymmetry, and the atom's own
  "what this rule does not do" statement.
- `standards/pages/README.md` — **two rows only**: one inside the generated
  `## Index` region, one in `## Start here`. No other region of that file is
  touched.
- `.bklg/docs-that-teach/page-need-discipline/fold-line-rule/**` — this spec, the
  `_ledger.md` the second pass defines, and this story's own stage artifacts.

The implementer **may** touch the composition-root file named in the Integration
contract to mount this slice; that is not scope drift. Here the composition root
*is* `standards/pages/README.md`, and it is already in the list above.

**Explicitly not in this PR**

- **`xtask/src/**` — nothing.** No `PERMITTED_FOLD_MECHANISMS` const, no marker
  scanner, no fold check, no `NEEDS` edit, no `main.rs` mount, no `affected.rs`
  entry. The checker's mounts belong to `page-need-checker-mounted-in-the-gate`;
  the vocabulary module belongs to `need-vocabulary-and-declaration-form`.
- **A fold-checker of any kind**, and the demonstration that a hidden branch sits
  inside the checked surface — HS-P0020's DT-7, initiative DoD-13's first disjunct
  (`_decomposition.md`, UX brief Note 3, *"Do not implement a fold-checker here"*).
- **Bands 00 and 10** (`standards/pages/00-one-need.md`,
  `standards/pages/10-the-need-set.md`), and **bands 30 and 40** — slice-mates'.
  This story edits neither, and states class 2's citation rule as a *cross-reference
  to band 30*, not as a second copy of it.
- **`standards/rust/**` — nothing.** `git diff main -- standards/rust/README.md`
  stays empty; project AC-002 is discharged architecturally by never opening that
  tree for writing.
- **Any `docs/README.md` edit** — the announcement and its
  `[PROVISIONAL — settles at …]` marker are
  `router-precedence-and-announcement`'s, already merged by the time this lands.
- **Any page in HS-P0020's narrative tree**, and any link from one — that is
  `governed-page-cites-the-discipline`.
- **`.kb/**` — nothing.** The playbook atom is staged under `.kb/_intake/` by
  `playbook-atom-staged-for-ingest`; DT-8's rejected options are that atom's
  material, not this one's.
- **`.redkiln/templates/**` — nothing**, so `redkiln doctor` still reports exactly
  six `template-drift` advisories, and `redkiln adopt --templates` is never run.
- **`_design.md`** — not edited. If DT-8's resolution is wrong, the disagreement is
  raised against sign-off condition 3, not patched in place.
- Any `spec/SPECIFICATION.md` clause, any ADR, any crate under `crates/`.

```
standards/pages/20-the-fold-line.md
standards/pages/README.md
.bklg/docs-that-teach/page-need-discipline/fold-line-rule/**
```

**Merge DoD, one line.** `standards/pages/20-the-fold-line.md` exists, is inside
every ceiling, carries the deletion test, the five closed classes, the empty
permitted-mechanism table with its entry procedure and the renderer-wrapper
sentence; the router indexes it with a resolving link and a row byte-identical to
the generator's format; `git diff main -- standards/rust/README.md`,
`git diff main -- xtask` and `git diff main -- .kb` are all empty; and
`cargo xtask ci --fast`, `cargo xtask lints`, `cargo xtask spec-trace` and
`cargo xtask affected --base main` are green — the last still widened to the whole
workspace, as `router-precedence-and-announcement` recorded and did not fix.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **The atom exists at its pinned address** | `standards/pages/20-the-fold-line.md`, band 20 of the five-band namespace the router fixes, under `RULE_DIR = standards/pages`. The filename is already cited by the design and by a slice-mate's PR boundary, so it is depended on **by value** before it exists. | `_design.md` `## Surfaces` and the pinned-constant table; `_design.md` `### S4` (the specimen problem line naming `standards/pages/20-the-fold-line.md`); `.bklg/docs-that-teach/page-need-discipline/need-vocabulary-and-declaration-form/spec.md` PR boundary |
| **The head grammar, with `Load when:` on one source line** | `# 20 — The fold line` · `> **Load when:** …` **one source line, no continuation** · `> **See also:** 00 (the declaration) · 30 (citations) · 40 (the reviewer's walk)`, bands **by number, not by link** · `---`. The one-line constraint exists because the generator reads only the first line of the block and silently drops the rest, producing the mid-phrase index rows visible in the constitution's own router today. | `standards/rust/00-prime-directives.md:1-9`; `xtask/src/lint_constitution.rs:247-257`; `standards/rust/README.md:95-96` (the truncation it produces); the router's `## The shape of a rule` |
| **Rule 1 — the deletion test, as the rule's spirit** | One `## RP-20-N.` imperative rule carrying the test verbatim: *if the collapsed region were deleted, would the page still teach the constraint correctly?* If no, it may not be collapsed. Stated as a question with a yes/no answer and a stated consequence — never "consider", "use judgement", "as appropriate" or "if it seems". | `_design.md` `### S1/S3 — DT-8`, Part 1; `.bklg/docs-that-teach/_discovery/distillation/interaction-patterns.md:404-410`; `project.md` DR-08; `_design.md` anti-pattern 15 |
| **Rule 2 — the closed never-fold list, as the rule's letter** | Five classes, enumerated, and **no reviewer may grant an exception**: (1) the `> **Answers:**` declaration; (2) any sentence carrying a normative modal or citing a `spec/SPECIFICATION.md` clause id; (3) any statement of an invariant, constraint or precondition; (4) the only occurrence of a code fence the reader is expected to run; (5) any statement of what a check does *not* verify. The list is **closed**: an edge case never shrinks it. | `_design.md` `### S1/S3 — DT-8`, Part 2 and "Failure mode and mitigation"; UX-003 (`_decomposition.md`, UX brief); `standards/rust/81-checks-that-cannot-be-types.md:11` (why class 5 exists) |
| **Class 1 pays band 00's deferral** | `standards/pages/00-one-need.md` states the declaration is never occluded and defers *which mechanisms count* to band 20. This atom answers it, and the `See also` line makes the pairing findable from either side. | `.bklg/docs-that-teach/page-need-discipline/need-vocabulary-and-declaration-form/spec.md` PR boundary (band 00 "with the mechanism question deferred to band 20"); `_design.md` `## Transience policy`, S1 declaration row |
| **Rule 3 — the permission gate, and its empty table** | A class not on the never-fold list is *eligible*; it is **permitted** only if the mechanism is on `PERMITTED_FOLD_MECHANISMS`. That table is authored **in this atom** and **ships with zero rows**, and the atom states the consequence in those words — *folding is forbidden in practice* — plus what lifts it: HS-P0020's DT-7 demonstration earning the first entry. | `_design.md` `### S1/S3 — DT-8`, Part 3, and `## Sign-off` condition 3 |
| **The entry procedure is four recorded observations, in this repository** | A mechanism reaches the table only with a recorded observation **here** that its content (i) sits correctly in the accessibility tree, (ii) is keyboard operable, (iii) is found by Ctrl-F, (iv) is found by print. **Upstream documentation is not an observation**, and an unverified property counts as unmet. | UX-011 and `_decomposition.md` UX brief Note 3; `interaction-patterns.md:205-216` (*"an unverified property, not a confirmed safe one"*), `:220-225`; `RUNBOOK.md:931-935` |
| **`PERMITTED_FOLD_MECHANISMS` is textual, not a `const`** | It is a table in the rules tree. A `const` beside `NEEDS` in `xtask/src/lint_pages.rs` is rejected: nothing enforces it, and an unenforced const beside an enforced one reads as a check that exists. The atom says which instrument does apply — band 40's reviewer walk, and HS-P0020's DT-7 for the demonstration half. | `_design.md` Part 3 ("a list **in the rules tree**"); `_decomposition.md` Architecture brief Note 3 item 3 and Note 7 item 3; UX brief Note 3 (*"Do not implement a fold-checker here"*); `RUNBOOK.md:920-925` |
| **The renderer-supplied wrapper is answered, in both halves** | RP-20-2 binds **the author's own markup**, so rustdoc's `<details class="toggle top-doc" open>` does not put a page in breach — **and** because `#toggle-all-docs` lets the reader close it and nothing here has observed what survives that, the wrapper is recorded in the atom as a **named unmet property owed by HS-P0020's hosting decision**. Both sentences ship; either alone is wrong. | `_design.md` `## Mock`, finding 2; UX-011's *unverified counts as unmet*; `_design.md` DR-05's hosting assumption and `## Sign-off` condition 1 |
| **The amendment asymmetry is stated, so an edge case never becomes an exception** | The never-fold list is **closed** and changing it takes disagreeing with `_design.md`'s sign-off condition 3; the permitted-mechanism table is **open by amendment with evidence**, in the tree, in public, with the four observations recorded. The failure this forecloses is the one the tension names: judgement that is right at the edges and then drifts. | `_design.md` `### S1/S3 — DT-8`, "Failure mode and mitigation"; `interaction-patterns.md:484-497` |
| **The atom states what this rule does not do, and does so unfolded** | A closing statement, itself never-fold class 5: this rule is applied by a reader, not by a byte count; no gate step reads it; whether a hidden branch is inside the checked surface is HS-P0020's DT-7 and not answered here; and a page can satisfy every class and still teach badly, which is band 40's walk and HS-P0024's evidence. | `standards/rust/81-checks-that-cannot-be-types.md:11` (RS-81-1); `_decomposition.md` Architecture brief Note 7 items 3 and 5; Testing brief closing paragraph |
| **`Not` and `Rejects.` name a wrong page that could plausibly ship** | Each rule carries a named wrong page in `Not` and a `**Rejects.**` clearing 120 characters that says **who is misled and when they find out** — e.g. a page whose "what this does not verify" paragraph sits in a `<details>` labelled "Limitations", which reads as tidy and is exactly RS-81-1's failure re-created. A rule with no wrong state is decorative (`_design.md` anti-pattern 14). | `standards/rust/README.md:99-107`; `xtask/src/lint_constitution.rs:82` (`MIN_REJECTS_CHARS`); `CLAUDE.md` (the decorative-rule corollary); `_design.md` anti-pattern 14 |
| **Fences are `text` or `markdown`; the atom folds nothing itself** | No `rust`-tagged fence (nothing compiles this tree — `xtask/src/lib.rs:24-28` registers only `mod constitution`) and no untagged fence. No `<details>`, tab strip or accordion anywhere in the atom: an atom about folding that folds is anti-pattern 6 applied to itself. | `_decomposition.md` Architecture brief Note 4; `_design.md` `## Composition` S3, anti-patterns 6 and 13 |
| **Every ceiling is measured, not asserted** | ≤ **6** rules; ≤ **16,384** bytes; prose ≤ **96** columns outside tables; every `**Rejects.**` ≥ **120** characters. The first two are identical to `MAX_RULES_PER_ATOM` and `MAX_ATOM_BYTES` on purpose — two trees teaching two different numbers for the same idea is its own defect. Yield order if a budget bites: `Evidence.` first (drop external URLs, keep repo `path:line`), then `Why.` to ≤ 3 sentences; `Not` and `Rejects.` never yield. | `_design.md` `## Density budget` and its per-surface yield order; `xtask/src/lint_constitution.rs:82,88,95` |
| **The atom is mounted: one index row, byte-exact** | One row appended inside `standards/pages/README.md`'s `<!-- BEGIN GENERATED -->` / `<!-- END GENERATED -->` region, in the format `generated_region` builds — a markdown link to `20-the-fold-line.md`, the atom's first `Load when` source line, and its comma-separated rule ids — every interior `\|` escaped, no blank line inside the markers, rows in filename order. The checker story's first `--write` must produce **no diff**. | `xtask/src/lint_constitution.rs:400-420`, `:388-397`; `router-precedence-and-announcement`'s spec, "The region's shape is a forward contract on the checker story" |
| **The atom is mounted: one `## Start here` row** | One row in the router's intent-keyed `| You are… | Load |` table, keyed to the reviewer's and author's real moment ("about to put something behind a `<details>`, a tab or a collapsed panel" / "reviewing a page that hides a claim"), inside the **12-row** ceiling, and the router still ≤ **8,192 bytes** after both edits. | `standards/rust/README.md:45-59`; `_design.md` `## Density budget` (Start here ≤ 12 rows; router ≤ 8,192 bytes); UX-012 |
| **Nothing else moves** | No `xtask` source, no `standards/rust/` file, no `docs/README.md` edit, no `.kb/` file, no `.redkiln/templates/` file, no narrative page, no other region of the router. `redkiln doctor` still reports exactly six `template-drift` advisories. | `CLAUDE.md`; `project.md` Definition of done; `_decomposition.md` Architecture brief Note 8 |

## Data and migrations

**N/A — no data and no migration.** This story adds one markdown file and appends
two rows to another. It touches no schema, no serialised format, no wire envelope,
no stored state and no compiled code: `standards/pages/` is deliberately absent
from the doctest harness (`xtask/src/lib.rs:24-28`), and nothing in the diff is
read by `rustc`.

Three adjacent obligations are the closest thing this story has to a migration.
None is a script, and each is stated so a later change cannot silently invalidate
it:

- **The generated index row is a format contract, not stored data.** Nothing
  migrates it. `page-need-checker-mounted-in-the-gate`'s first `--write` against
  the router must produce no diff, which makes the row this PR appends a forward
  obligation inherited from `router-precedence-and-announcement` rather than a
  free-form edit.
- **`PERMITTED_FOLD_MECHANISMS` ships empty and is an append-only table with a
  stated entry cost.** Adding the first row is a change to what the discipline
  permits, and it carries four recorded observations with it; removing rows is not
  contemplated, because a mechanism that was observed safe does not become unsafe
  by omission. Should the table ever hold a row, the atom's "folding is forbidden
  in practice" sentence becomes false and must move in the same commit — the same
  `[PROVISIONAL — settles at …]` discipline the router applied to
  `docs/README.md:25-29`, applied here to a sentence rather than a pin.
- **The never-fold list is closed at five and is *not* append-only.** Growing it is
  a disagreement with `_design.md` sign-off condition 3, which means re-opening the
  design, not editing the atom. This asymmetry is the whole mitigation and it is
  the one thing a future contributor is most likely to get backwards.
## Acceptance criteria

Nine criteria. Each is framed from **one reader's intent crossing the whole stack** — from
the router, into band 20, to a verdict on a page the reader did not write — because that
traverse is the product here. A rule that is correct and unfindable, or findable and
un-appliable, satisfies nothing.

The personas are the initiative's, cited not invented: the **non-author reviewer** holding
a page they did not write (project AC-005's own words — *"in a form a reviewer applies to a
page without consulting the author"*; UX-009's sharpening — *performable by a stranger, in
reading order, with no access to the author, no repository archaeology and no mouse*), the
**author** about to put something behind a `<details>`, and the **adapter author** whose
measured defect is an answer that existed three documents from where they were standing
(`.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md:174-181`).

Every verification is runnable from the worktree root in `git bash`, or is a named
procedural walk whose *output* — never a claim about it — enters `_ledger.md`. This PR
touches no `xtask/src/**`, so it can carry no `#[test]`; where an AC's permanent regression
assertion lands in the next slice, the verification names both the check that runs **now**
and the forward test that inherits it (see "Clarifications resolved during spec", item 2).

| id | criterion | verification |
| --- | --- | --- |
| **AC-001** | **GIVEN** the non-author reviewer, holding a page whose "what this does not verify" paragraph sits inside a `<details>` labelled *Limitations*, and who cannot ask the author what they intended, **WHEN** they load `standards/pages/20-the-fold-line.md` — one hop from the router, `# 20 — The fold line`, a one-source-line `> **Load when:**`, a `> **See also:**` naming sibling bands **by number**, then `---` — and apply **RP-20-1**, **THEN** the atom hands them the deletion test verbatim (*if the collapsed region were deleted, would the page still teach the constraint correctly?*) as a question with a **yes/no** answer and a stated consequence, and they reach a **verdict**, not an impression — with no sentence anywhere in the atom reading "consider", "use judgement", "as appropriate" or "if it seems". | *Mechanical, output captured:* `test -f standards/pages/20-the-fold-line.md`; `rg -n '^# 20 — The fold line$'` → one hit; `rg -n -A1 'Load when:' standards/pages/20-the-fold-line.md` → the following source line does not begin with `>`; `rg -n 'See also' …` → names `00`, `30`, `40` as numbers, not links; `rg -n 'would the page still teach the constraint correctly' …` → ≥ 1; `rg -in 'consider\|use judge?ment\|as appropriate\|if it seems' standards/pages/20-the-fold-line.md` → **no matches**. *Procedural (ledger):* a person who did not author this atom walks a specimen page carrying the `<details>`-wrapped limitations paragraph and records the verdict, the rule id they reached it by, and that they consulted no author. Bars: `_design.md` `### S1/S3 — DT-8` Part 1 and anti-pattern 15; `interaction-patterns.md:404-410`. |
| **AC-002** | **GIVEN** the same reviewer facing an edge case the deletion test alone would leave to judgement — the drift the tension names in its own words — **WHEN** they read **RP-20-2**, **THEN** they find **five** enumerated never-fold classes: the page's own `> **Answers:**` declaration; any sentence carrying a normative modal or citing a `spec/SPECIFICATION.md` clause id; any statement of an invariant, constraint or precondition; the only occurrence of a code fence the reader is expected to run; any statement of what a check does *not* verify — **AND** the atom states in its own text that **no reviewer may grant an exception** and that the list is **closed**, so an edge case grows the *mechanism* list and never shrinks the class list, growing the class list being a disagreement with `_design.md` sign-off condition 3 rather than an authoring choice. | *Mechanical:* the five classes are an enumerated list of exactly five items under `## RP-20-2.` — `awk '/^## RP-20-2\./,/^## /' standards/pages/20-the-fold-line.md \| grep -c '^[0-9]\.'` → `5`; `rg -n 'no reviewer may grant an exception' …` → ≥ 1; `rg -n 'closed' …` → the closedness sentence present and naming sign-off condition 3 as the amendment path. *Procedural (ledger):* the reviewer classifies three specimen fragments (a `MUST` sentence, a clause citation, a runnable fence) by class number, unaided, and each maps to exactly one class. Bars: `_design.md` Part 2 and "Failure mode and mitigation"; UX-003; class 5's reason at `standards/rust/81-checks-that-cannot-be-types.md:11`. |
| **AC-003** | **GIVEN** the page author who has just read `standards/pages/00-one-need.md` and found the declaration's never-occluded rule with *which mechanisms count* explicitly deferred, **WHEN** they follow the deferral, **THEN** band 20 pays it — class 1 of RP-20-2 **is** the declaration — and the pairing is findable from either side, because band 20's `> **See also:**` names `00` by number, as the constitution spells it, **AND** `standards/pages/00-one-need.md` is not edited by this PR to say so. | *Mechanical, both directions:* `rg -n 'See also' standards/pages/20-the-fold-line.md` names `00`; `rg -n '20' standards/pages/00-one-need.md` (the slice-mate's own `See also`, already merged) → present; `git diff main -- standards/pages/00-one-need.md standards/pages/10-the-need-set.md` → **empty**. *Procedural (ledger):* the round trip walked once in each direction, keyboard only, one hop each way, recorded. Precedent for by-number spelling: `standards/rust/00-prime-directives.md:7-9`. Source of the deferral: the slice-mate's PR boundary (`.bklg/docs-that-teach/page-need-discipline/need-vocabulary-and-declaration-form/spec.md:246-247`). |
| **AC-004** | **GIVEN** the author who has cleared RP-20-2 — their content is on no never-fold class and is therefore *eligible* — **WHEN** they read **RP-20-3**, **THEN** they learn eligibility is not permission: a fold is permitted only if its mechanism appears on `PERMITTED_FOLD_MECHANISMS`, a table **in this atom**, which **ships with zero data rows** — and the atom says in those words that **folding is therefore forbidden in practice**, names what would lift it (HS-P0020's DT-7 demonstration earning the first entry), and so the author reaches "not yet, and here is who can change that" instead of a silence they resolve in their own favour. | *Mechanical:* the table exists with a header and a separator and **no data row** — `awk '/PERMITTED_FOLD_MECHANISMS/,/^$/' standards/pages/20-the-fold-line.md \| grep -c '^\|'` → `2`; `rg -n 'forbidden in practice' …` → ≥ 1; `rg -n 'DT-7' …` → ≥ 1. *Procedural (ledger):* an author who did not write the atom states, from RP-20-3 alone, (a) whether they may use a `<details>` today, (b) what would have to happen first, (c) who owns it — all three recorded. Bar: `_design.md` Part 3 and `## Sign-off` condition 3. |
| **AC-005** | **GIVEN** a future contributor who has read that `mdbook-tabs` documents its panels as accessible and wants to add the first mechanism row on that basis, **WHEN** they read the entry procedure in **RP-20-3**, **THEN** it holds them to **four recorded observations in this repository** — the content (i) sits correctly in the accessibility tree, (ii) is keyboard operable, (iii) is found by Ctrl-F, (iv) is found by print — and states that **upstream documentation is not an observation** and that an **unverified property counts as unmet**, so the row is not added and the honest gap stays named rather than closed by assurance. | *Mechanical:* `rg -n 'accessibility tree' …`, `rg -n 'keyboard' …`, `rg -n 'Ctrl-F\|find' …`, `rg -n 'print' …` → all four present inside the `## RP-20-3.` block; `rg -n 'unverified' …` → the *counts as unmet* sentence present; `rg -n 'upstream' …` → the not-an-observation sentence present. *Procedural (ledger):* the contributor scenario walked as a dry run — the walker states what they would have to produce and confirms none of it exists in this worktree. Bars: UX-011 and `_decomposition.md` UX brief Note 3; the evidence it is earned from, `interaction-patterns.md:205-216` and `:220-225`, and the incident at `RUNBOOK.md:931-935`. |
| **AC-006** | **GIVEN** a reader of a rustdoc-hosted page, where rustdoc has wrapped the whole document in `<details class="toggle top-doc" open>` and ships `#toggle-all-docs` to close it — so never-fold class 1 already sits inside a disclosure nobody has observed — **WHEN** they consult band 20, **THEN** it answers in **both halves**: RP-20-2 binds **the author's own markup**, so a renderer-supplied, open-by-default wrapper does not put a page in breach; **AND** because the reader can close it and nothing here has checked what survives that, the wrapper is recorded in the atom as a **named unmet property owed by HS-P0020's hosting decision** — so no page is retroactively non-conformant and no unchecked property is quietly promoted to "fine". | *Mechanical:* `rg -n 'toggle.top-doc\|toggle-all-docs' standards/pages/20-the-fold-line.md` → ≥ 1; both sentences present and adjacent — the author's-markup scope sentence and the unmet-property sentence — confirmed by reading the block and pasting it into `_ledger.md`. *Procedural (ledger):* a reader who knows only mock finding 2 reads the block and answers "is a rustdoc-hosted page in breach today?" (**no**) and "is the wrapper's behaviour known?" (**no, and it is owed by HS-P0020**); both answers recorded. Bars: `_design.md` `## Mock` finding 2, `### S1 —` DR-05's hosting assumption, `## Sign-off` condition 1; UX-011. |
| **AC-007** | **GIVEN** a contributor who assumes a rule written into a gate-read repository is a rule something checks, **WHEN** they reach band 20's closing "what this rule does not do" statement — itself unfolded, because it is never-fold class 5 applied to the atom that wrote the class — **THEN** they learn that this rule is applied by a **reader** and by **no gate step**; that `PERMITTED_FOLD_MECHANISMS` is a table in the rules tree and deliberately **not** a `const` in `xtask/src/lint_pages.rs`, because an unenforced const beside an enforced one reads as a check that exists; that whether a hidden branch sits inside the checked surface is HS-P0020's DT-7 and is not answered here; and that a page can satisfy every class and still teach badly, which is band 40's walk and HS-P0024's evidence. | *Mechanical, and this is the negative half that matters:* `git diff main -- xtask` → **empty**; `rg -n 'PERMITTED_FOLD_MECHANISMS' xtask/` → **no matches**; `rg -n 'does not' standards/pages/20-the-fold-line.md` → the closing statement present, with all four limits named. *Procedural (ledger):* a reader who has not seen this spec reads only the closing statement and states the four limits in their own words. Bars: RS-81-1 (`standards/rust/81-checks-that-cannot-be-types.md:11`) — *"a check whose limits are undocumented is read as a guarantee"*; `_decomposition.md` Architecture brief Note 7 items 3 and 5; UX brief Note 3's *"Do not implement a fold-checker here"*. |
| **AC-008** | **GIVEN** any of those readers, on a rendered markdown page **or** in a plain-text pager, **WHEN** the atom first loads with nothing clicked, **THEN** every rule is real **composed presentation in this repository's own textual grammar** — a `## RP-20-N.` imperative sentence followed by exactly five sections in fixed order (**Why.** · **Do** · **Not** · **Rejects.** · **Evidence.**), each `Not` naming a wrong page that could plausibly ship and each `**Rejects.**` clearing **120 characters** by saying who is misled and when they find out — **AND** the atom is inside every ceiling (≤ **6** rules, ≤ **16,384** bytes, prose ≤ **96** columns outside tables), every fence is tagged `text` or `markdown` (never `rust`, never untagged), and the atom contains **no `<details>`, tab strip or accordion of its own**, because an atom about folding that folds is anti-pattern 6 self-applied. | *Mechanical, every number a command whose output is pasted into `_ledger.md`:* `grep -c '^## RP-20-' standards/pages/20-the-fold-line.md` → ≤ `6`; `wc -c < standards/pages/20-the-fold-line.md` → ≤ `16384`; `awk '!/^\|/ && length > 96 {print FNR": "length}' …` → no output; each rule block contains the five markers in order (`awk` per block, captured); each `**Rejects.**` body ≥ 120 chars (`awk` length, captured per rule); `rg -n '^```rust' standards/pages/` → no matches; `rg -n '^```$' standards/pages/` → no matches; `rg -n '<details\|<summary\|role="tab"\|\{\{#tab\|<small>\|<sub>\|<img' standards/pages/` → no matches. *Procedural (ledger):* read the atom through `less` with no renderer and confirm no meaning is lost and no wrong page is missing. Bars: `standards/rust/README.md:99-107`; `xtask/src/lint_constitution.rs:67-81,82,88,95`; `_design.md` `## Composition` S3, `## Density budget`, `## Hierarchy`, anti-patterns 6, 13, 14. |
| **AC-009** | **GIVEN** the next page author who reaches this tree from a stated intent — *about to put something behind a `<details>`, a tab or a collapsed panel* — and the implementer of `page-need-checker-mounted-in-the-gate` one slice later, **WHEN** the first opens `standards/pages/README.md` and the second runs `--write` against it for the first time, **THEN** the author finds **one** `## Start here` row keyed to that intent (table still ≤ **12** rows, router still ≤ **8,192** bytes) that routes them to exactly one file, **AND** the generated `## Index` region carries **one** row for `20-the-fold-line.md` — a markdown link, the atom's first `Load when` source line, its comma-separated rule ids, interior `\|` escaped, no blank line inside the markers — **byte-identical** to what the generator emits, so the `--write` produces **no diff**. | *Mechanical:* `rg -n '20-the-fold-line' standards/pages/README.md` → exactly **2** hits, one inside `sed -n '/BEGIN GENERATED/,/END GENERATED/p'` and one inside the `## Start here` block; the link target passes `test -f standards/pages/20-the-fold-line.md`; in-region row count still equals `ls standards/pages/*.md \| grep -v README \| wc -l`; `awk '/^## Start here/,/^## Index/' standards/pages/README.md \| grep -c '^\|'` → ≤ `14`; `wc -c < standards/pages/README.md` → ≤ `8192`; the index row's trigger cell is not mid-phrase. *Procedural (ledger):* the row-by-row derivation against `generated_region` (`xtask/src/lint_constitution.rs:400-420`), and the one-hop keyboard traverse *intent → row → this file → stop*. *Forward:* the no-diff `--write` recorded in `page-need-checker-mounted-in-the-gate`'s own ledger, inherited from `router-precedence-and-announcement`'s AC-004. |

**Project-AC coverage.** Project **AC-005** — *"DT-8 is resolved as a stated rule … which
classes of content may never sit behind a fold, tab or collapsed panel, in a form a reviewer
applies to a page without consulting the author"* — is discharged by **AC-001** (the form a
reviewer applies), **AC-002** (which classes, closed and exception-free), **AC-004** and
**AC-005** (the permission gate that makes the rule bite today, and the cost of loosening
it), **AC-006** (the renderer-supplied edge, answered rather than left), and **AC-007** (the
rule's own stated limits, so it is not read as a check). **AC-003**, **AC-008** and
**AC-009** are what keep it *reachable and applied* rather than merely written — the
half-mount `_storymap.md` refuses in its first "Why the slices fall here" bullet. This story
is AC-005's **sole owner** (`_storymap.md`, Coverage); it traces to no other project AC, and
DR-08 rides along with it.

## Interaction quality

RFC §6.7/D6. Every invariant that applies is an **AC row above**, never a bullet here: this
section says *which id carries which invariant* and how each is verified, so nothing gateable
lives only in this section. The medium is text, so each invariant is restated in the form it
takes with no DOM — that translation is `_design.md`'s (`## Transience policy`, first
paragraph), not invented here.

**State family.**

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **In place, not a context jump** — the verdict is reached from the atom the reader already opened, with no author consulted and no repository archaeology | **AC-001** (the deletion test and its yes/no consequence in the atom's own text), **AC-007** (the blind-spot statement in the atom, not in a separate note) | AC-001's procedural non-author walk plus the hedging-word grep; AC-007's read-back. The forbidden shape is the initiative's one *measured* defect — an answer three documents from where the reader was standing (`personas-and-journeys.md:174-181`) |
| **One intended context switch, and the reader chooses it** | **AC-009** (one `## Start here` row → this one file → stop), **AC-003** (the band 00 ↔ band 20 round trip, one hop each way) | the recorded traverses. `_design.md` `## Transience policy` classes index→atom links as the tree's *only* opened-on-demand control |
| **Non-occlusion — nothing load-bearing is hidden, and the rule that says so does not hide** | **AC-002** (the rule itself, five closed classes), **AC-008** (the atom contains no `<details>`, tab or accordion of its own — anti-pattern 6 self-applied), **AC-009** (an atom absent from the index is occluded by omission) | AC-008's absence greps; AC-009's row-count and two-hit checks; AC-002's enumerated-list count. This is the invariant the *whole story* is about, applied to its own artifact first |
| **Preserved focus, scroll and selection** | **AC-008** (discharged by **absence**: this PR authors no disclosure and no script, so nothing exists that *can* move focus or scroll), **AC-006** (the one disclosure that does exist is the renderer's, and it is recorded as an unmet property rather than assumed safe) | AC-008's greps keep the absence true; AC-006's both-halves check keeps the exception honest. UX brief Note 2 item 3 names the `mdbook-tabs` unknown this refuses to inherit |
| **Reversibility** | **AC-009** (the generated region's only writer is `--write`, and reverting this PR leaves no cache, no generated artifact and no dirty file), **AC-008** (nothing to close, because nothing opens) | `git checkout -- standards/pages && git status` → clean, captured. UX-008's own procedure shape |
| **Keyboard reachability** | **AC-009** (the intent→row→file traverse run keyboard-only; plain markdown links), **AC-003** (the round trip likewise), **AC-008** (no widget that could need a pointer) | the recorded traverses plus the absence greps. UX-006's test wording; the tab-triad cost UX brief Note 2 item 5 records is avoided by *narrowing what is permitted*, which is exactly what AC-004's empty table does |

**Composition family** — taken from the signed-off
`.bklg/docs-that-teach/page-need-discipline/_design.md`, **binding** on this story because it
renders the `rule-atom` surface and changes the `discipline-router` surface. This story
implements that design and does not re-decide it.

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **Presentation exists at all** — every rule is composed in the repo's own grammar (head, `---`, `## RP-20-N.` imperative, five fixed sections, `Not` and `Rejects.` naming a wrong page), not prose that happens to contain the words | **AC-008**, with **AC-001** carrying the head grammar specifically | AC-008's per-block five-marker capture and the 120-character `Rejects.` measurement; AC-001's head greps. `_design.md` `## Composition` S3; `standards/rust/00-prime-directives.md:19,30,54,78` |
| **Composition and placement** — the atom top to bottom in the binding order; the rules in DT-8's own part order (spirit, letter, gate) | **AC-001** (head → `---`), **AC-002**, **AC-004** (RP-20-1 → RP-20-2 → RP-20-3) | `grep -n '^## RP-20-'` prints the three ids in ascending order with the deletion test first; compared against `_design.md` `### S1/S3 — DT-8` Parts 1-3 |
| **Transience** — every region of the atom is **persistent chrome**; `Evidence.` lists are *eligible* to fold and forbidden anyway, because Part 3's list is empty | **AC-008**, **AC-004** | `_design.md` `## Transience policy` rows S3 — the `Evidence.` row states the eligibility and the prohibition together; AC-008's absence greps are what make the prohibition observable |
| **Density budget, with its real numbers** — ≤ 6 rules, ≤ 16,384 bytes, ≤ 96 columns outside tables, `Rejects.` ≥ 120 characters; router ≤ 8,192 bytes and ≤ 12 `Start here` rows; and the stated yield order (`Evidence.` first, then `Why.` to ≤ 3 sentences; `Not` and `Rejects.` **never** yield) | **AC-008** (the atom), **AC-009** (the router after the edit) | the `wc` / `awk` / `grep` measurements in both ACs, each **output** pasted into `_ledger.md` (NF-004). `_design.md` `## Density budget` and its per-surface yield order; the numbers are `MAX_RULES_PER_ATOM` and `MAX_ATOM_BYTES` verbatim (`xtask/src/lint_constitution.rs:88,95`) |
| **Hierarchy** — the `## RP-20-N.` imperative sentence primary; `Do` / `Not` secondary; `Why.`, `Rejects.`, `Evidence.` recessive — carried by heading level and the bold run-in convention, **never** by colour, icon or size | **AC-008** | AC-008's five-marker order capture plus the `<small>`/`<sub>`/`<img>` absence grep. `_design.md` `## Hierarchy` row S3. Recessive means "read third", not "read never" — which is why `Rejects.` still carries a 120-character floor |
| **Named anti-patterns refused** — 5 (a never-fold class invisible until clicked), 6 (any `<details>`/tab/accordion at all while `PERMITTED_FOLD_MECHANISMS` is empty), 13 (a `rust`-tagged fence anywhere in `standards/pages/`), 14 (a rule atom with no `Not` or no `Rejects.`), 15 (a hedging step: "consider", "use judgement", "as appropriate", "if it seems") | 5 → **AC-002**; 6 and 13 → **AC-008**; 14 → **AC-008**; 15 → **AC-001** | the greps and per-block measurements named in each AC. Anti-pattern 15's grep is the one that keeps AC-005 (project) honest: the register of a hedge is the register of the non-answer the tension quotes |

**Why this matters here specifically.** An unstyled, uncomposed render satisfies almost every
structural assertion a spec like this can make — a file containing the words "never fold" and
five bullet points would pass a naive presence check. The composition rows are what fail it:
a rule with four sections instead of five, a `**Rejects.**` of eleven words, a seventh rule, a
`rust`-tagged fence, an atom over 16,384 bytes, or a `## Start here` row that routes to two
files each fail a **named measurement with a real number**. There is no perceptual review to
catch what those miss (`design.capture` is absent from `.redkiln/config.yaml:75-81` and the
skip is deliberate), so `_design.md` plus these rows are the only instrument this story has.

## Error conditions

| id | condition | required handling |
| --- | --- | --- |
| **EC-001** | `standards/pages/README.md` does not exist, or exists without its `<!-- BEGIN GENERATED -->` / `<!-- END GENERATED -->` markers and its `## Start here` table, when this atom is authored. | **Stop.** The dependency `router-precedence-and-announcement` has not landed, and this story's mount point is its output. Authoring the atom without mounting it produces the half-mount `_storymap.md` refuses — a rule reachable only by `ls`. Do not create the router here: its shape, its precedence statement and its band table are that story's ACs, and a second author's version of it would be a competing composition root. |
| **EC-002** | The atom's `> **Load when:**` wraps onto a second source line. | **The index row silently truncates.** `load_when` reads only the first line of the block and drops continuations (`xtask/src/lint_constitution.rs:247-257`); the visible cost is the mid-phrase rows in the constitution's own router today (`standards/rust/README.md:95-96`). Shorten the trigger phrase; never continue it. This is `router-precedence-and-announcement`'s AC-005 obligation landing on the first atom written after it. |
| **EC-003** | An implementer adds `const PERMITTED_FOLD_MECHANISMS: &[&str]` to `xtask/src/lint_pages.rs`, or any fold-marker scan, "so it is enforced". | **AC-007 fails outright, and `git diff main -- xtask` is the gate.** Nothing reads that const, and an unenforced const beside an enforced one is the decorative-gate-step failure (`RUNBOOK.md:920-925`, `:931-935`) recreated at the vocabulary layer. The enforcement seam is HS-P0020's DT-7 and the checker story's; the UX brief says *"Do not implement a fold-checker here"* in those words (`_decomposition.md`, UX brief Note 3). |
| **EC-004** | Someone proposes shipping the first `PERMITTED_FOLD_MECHANISMS` row in this PR — `<details>` "because rustdoc already uses it", or `mdbook-tabs` "because its docs say it is accessible". | **Refuse; the table ships empty.** The entry cost is four *recorded observations in this repository* (AC-005), none of which exist in a worktree with no wired-up render. Upstream documentation is not an observation, and an unverified property counts as unmet (UX-011). If the empty list is wrong, the disagreement is with `_design.md` `## Sign-off` condition 3, not with this PR. |
| **EC-005** | An edge case during authoring — an exercise answer, a long output transcript — pressures either list: a sixth never-fold class, or a one-off exception for this page. | **The asymmetry is the answer, and it runs one way only.** An edge case grows the *mechanism* list, with evidence, in public; it never shrinks or grows the class list. Growing the class list re-opens the design. Granting a one-off exception is the exact drift the tension names — *"'use good judgment' is the same non-answer that let the code-layer invariant drift in the first place"* (`interaction-patterns.md:484-497`). |
| **EC-006** | The renderer-wrapper question (AC-006) is answered with **one** half — either "rustdoc's wrapper is fine" or "every rustdoc-hosted page is in breach". | **Both sentences ship or neither is correct.** The first alone converts *we did not check* into *it is fine*, which is the move UX-011 exists to refuse; the second alone makes every rustdoc-hosted page in the repository retroactively non-conformant to a rule about **author** choices. `_design.md` `## Mock` finding 2 is the source and it asks for a sentence on the *mechanism* question, not a verdict on the repository. |
| **EC-007** | The atom approaches 16,384 bytes or 6 rules while being written — most likely by re-arguing DT-8's rejected options inside it. | **Apply the stated yield order, do not invent one:** `Evidence.` yields first (drop external URLs, keep repo `path:line`), then `Why.` shortens to ≤ 3 sentences. `Not` and `Rejects.` **never** yield — an atom that drops them is decorative (anti-pattern 14). The first thing to cut is any restatement of `_design.md`'s rejected options: they are already written down, and the playbook atom (`playbook-atom-staged-for-ingest`) is where they belong durably. |
| **EC-008** | `## Start here` is already at 12 rows when this story's row is added, or the router exceeds 8,192 bytes after the two edits. | Apply the router's own yield order (`_design.md` `## Density budget`, S2): merge `Start here` rows, then shorten the scope paragraph. The band table and the `## Index` never yield. Do **not** solve it by omitting this story's `Start here` row — the index row alone leaves the author with no intent-keyed route, which is the filter failing the reader it exists for. |
| **EC-009** | A fence in `Do` or `Not` is tagged `rust`, or left untagged. | **Rejected, both.** Nothing compiles this tree — `xtask/src/lib.rs:24-28` registers only `mod constitution` — so a `rust` fence is a Rust claim nothing checks (anti-pattern 13), and an untagged fence would let a future decision to register the tree be undermined retroactively (`_decomposition.md`, Architecture brief Note 4). Tag every fence `text` or `markdown`. |
| **EC-010** | `cargo xtask affected --base main` selects every package on this prose-only PR, and it is read as a failure or "fixed" by adding `standards/pages` to `INERT`. | **Expected, and correct at this merge.** Record the widening as observed; do not touch `xtask/src/affected.rs`. The `INERT` entry alone makes a prose-only PR read **nothing** — the half-mount the story map names — and it lands paired with the checker's unconditional-list entry in the next slice (`_decomposition.md`, Architecture brief Note 1, CR-4). `router-precedence-and-announcement` recorded the same widening and deliberately did not fix it. |
| **EC-011** | A rule's `**Rejects.**` comes in under 120 characters because the wrong page is described rather than named. | The floor is a *length calibrated against the corpus* precisely because "this would be confusing" is the failure it forbids (`xtask/src/lint_constitution.rs:82` and its doc comment). Rewrite it to name **who is misled and when they find out** — e.g. the reviewer who reads a `<details>` labelled *Limitations* as tidy, and discovers the blind spot only when the check they trusted did not catch what it never looked at. |

## Non-functional

| id | requirement | why, and how it is observed |
| --- | --- | --- |
| **NF-001** | **No dependency changes and no Rust.** Every workspace manifest untouched; nothing added for measurement (the budgets are `wc`, `awk`, `grep`, `rg`, all present). | `git diff main -- '**/Cargo.toml' Cargo.lock xtask` → empty. The testing brief is explicit that no `tempfile` — or any — dependency is added for this project, and this story adds no Rust at all. |
| **NF-002** | **LF line endings and UTF-8, no BOM**, on both files. | This is a Windows checkout, and the router's generated region is compared for **string equality** by the checker one slice later: a CRLF interior line is not equal to an LF one, so a CRLF region turns the first `--write` into a whole-region diff (EC-003 by another route, inherited from `router-precedence-and-announcement`'s NF-002). Observe with `file standards/pages/20-the-fold-line.md` and `git diff --check`. |
| **NF-003** | **Non-ASCII characters match the corpus's existing set** — the em dash `—` and the middle dot `·` only, and only where the corpus already uses them. No smart quotes, no non-breaking spaces, no ellipsis character. | Two files in one tree rendering the same idiom two ways is a second grammar, in a tree whose subject is that a contributor should not have to guess which of three spellings is the mistake (`_decomposition.md`, UX brief Note 4). |
| **NF-004** | **Every budget number in this spec is reproducible from a named command**, and the command's **output** — not a claim about it — is what enters `_ledger.md`. | `_design.md` `## Sign-off` condition 4: no pixel budget is claimed, only source columns and bytes, "which are measurable today". A ledger row reading "checked, within budget" without the number is not evidence, and this project's whole thesis is that a document vouching for a check is not the check. |
| **NF-005** | **Gate wall-clock may regress only by the affected-gate widening.** No step is added, no step is slowed, no new file is read by any existing step. | `cargo xtask affected --base main` compiles and tests the whole workspace (EC-010); `cargo xtask ci --fast` is unchanged in shape because no step reads `standards/pages/` yet — which is itself the fact AC-007 and the router's own blind-spot section state out loud. |
| **NF-006** | **`redkiln doctor` still reports exactly six `template-drift` advisories.** | `.redkiln/templates/**` is out of scope (PR boundary) and the `backlog` CI job asserts the set is exactly those six (`CLAUDE.md`). A seventh or a fifth means a template moved in this diff; `redkiln adopt --templates` is never run. |
| **NF-007** | **`cargo xtask spec-trace` stays green and `spec/SPECIFICATION.md` is unmodified.** | Nothing here writes, restates or renumbers a clause; RP-20-2 class 2 is *about* clause ids and must discharge itself by **citing**, never by copying (DR-09, band 30). `git diff main -- spec/` → empty. Clause ids are stable names and are never renumbered (`spec/SPECIFICATION.md:280`), which is what makes a rule about ids durable. |
| **NF-008** | **Nothing is written into `.kb/`.** | `git diff main -- .kb` → empty. Atoms are authored only through `/redkiln:kb-ingest`, and the hand-authoring attempt was reverted at `0269720` (`CLAUDE.md`). DT-8's rejected options are `playbook-atom-staged-for-ingest`'s material, staged under `.kb/_intake/` by that story — not this one's, and not `.kb/`'s. |
| **NF-009** | **The atom is loadable in isolation.** A reader who opens `standards/pages/20-the-fold-line.md` and nothing else can apply all three rules; no rule's meaning depends on having read a sibling band. | UX-012 — *a reader loads one rule, not the corpus* — and the byte ceiling that enforces it. The `> **See also:**` line points *outward* for context; it may never be load-bearing for a verdict. Observed by the AC-001 procedural walk, which is run against the atom alone. |

## Implementation notes (non-prescriptive)

Not instructions — the shape the front half's decisions imply, offered so the implementer
spends judgement on the prose rather than on rediscovering the constraints.

- **Read one atom end to end before writing a line.** `standards/rust/00-prime-directives.md`
  is the worked five-section shape at `:19,30,54,78`, and its head at `:1-9` is the grammar
  AC-001 checks. Copy the *shape*; copy none of the Rust content, and do not copy the
  mid-phrase index rows the precedent tolerates — AC-009 exists because this tree must not.
- **Three rules, in DT-8's own part order.** `RP-20-1` the deletion test (spirit), `RP-20-2`
  the closed five-class list (letter), `RP-20-3` the permission gate and its empty table.
  The front half already refers to *"RP-20-2 class 1"* and *"RP-20-2 binds the author's own
  markup"*, so the numbering is fixed by prior citation, not by preference. Three of six
  rules leaves real headroom; spend it on `Not` and `Rejects.`, not on a fourth rule.
- **The renderer-wrapper sentences and the amendment asymmetry are carried *inside* those
  three rules**, not promoted to rules of their own. A "rule" whose `Not` names no wrong page
  is decorative (anti-pattern 14), and neither of those two forbids anything on its own — the
  wrapper sentence scopes RP-20-2, and the asymmetry states how each list changes.
- **Write the closing "what this rule does not do" statement early, while the limits are
  fresh and before the temptation to soften them.** Place it after the final rule's
  `**Evidence.**`; if it takes a heading, a plain `## ` heading is safe — `rules()` counts
  only headings prefixed `## RP-` and ends the previous rule's body at the next `## `
  (`xtask/src/lint_constitution.rs:258-284`, and the test that pins the behaviour at `:871`).
- **Author the router's two rows last, from the file that exists.** Read the atom's own first
  `> **Load when:**` line and its `## RP-20-N.` ids, then build the index row by hand the way
  `generated_region` builds it (`xtask/src/lint_constitution.rs:400-420`). Deriving it from
  the file is what makes AC-009's no-diff obligation reachable rather than lucky.
- **The `## Start here` row is written in that table's voice.** Every row is a phrase a
  contributor would actually think — *"about to put something behind a `<details>`, a tab or
  a collapsed panel"* — not a description of a deliverable. A row that reads like a project
  artifact is a row the reader skips.
- **Quote the primary evidence, do not paraphrase it.** The deletion test is verbatim from
  `interaction-patterns.md:404-410`; the *"use good judgment"* diagnosis is verbatim from
  `:484-497`; the *"an unverified property, not a confirmed safe one"* line is verbatim from
  `:205-216`. Paraphrase is how a rule drifts from the evidence that justifies it — the exact
  failure mode this band is being written about.
- **Two spellings are pins, typed by hand once and copied thereafter:**
  `standards/pages/20-the-fold-line.md` and `PERMITTED_FOLD_MECHANISMS`. The first is already
  depended on by value by `_design.md` `### S4` and by a slice-mate's PR boundary; the second
  is a name the playbook atom and HS-P0020's DT-7 will both cite.
- **Do not reach for a shared helper, and do not open `xtask/`.** After the second time you
  write a path relative to `standards/pages`, the instinct is to factor something out with
  `lint_constitution`. RS-81-3 scopes a scanner to the directory whose behaviour it
  constrains (`_decomposition.md`, Architecture brief Note 6) — and in any case this PR
  contains no Rust and `git diff main -- xtask` empty is a merge condition.
- **Run the measurements as you go and paste their output into the ledger.** Nine ACs,
  roughly twenty commands; collecting them at the end is how they get paraphrased (NF-004).

## Tests and CI (merge gate)

Tiers as the project's testing brief defines them
(`.bklg/docs-that-teach/page-need-discipline/_decomposition.md`, Testing brief, "Acceptance
Criteria"): **static** (`#[cfg(test)]`), **gate-integration**, **end-to-end/fixture**,
**procedural (ledger-recorded)**. That brief places AC-005 — this story's whole traced
project AC — in the **procedural** tier explicitly (*"the resolutions themselves, with
rejected options named, are `_design.md`'s prose and are cited in the ledger, not
code-tested"*), and closes by stating that it covers *"only what may never be folded, never
whether a fold is itself checked"*. This table is weighted accordingly, and says where the
static obligations land instead of dropping them.

| tier | command / path | proves |
| --- | --- | --- |
| **static** | *(none in this PR — by construction)* | This PR touches no `xtask/src/**`, so it can carry no `#[test]`. The permanent regression assertions its ACs imply are named here and inherited by `page-need-checker-mounted-in-the-gate`: the atom-shape checks (five sections in order, `Rejects.` ≥ 120 chars, ≤ 6 rules, ≤ 16,384 bytes — the `check_shape`/`check_rules` analogues), the `rust`-tagged/untagged fence rejection (Architecture brief Note 4), and the link + index-equality halves of `check_router` (`xtask/src/lint_constitution.rs:334-384`). Recorded as a forward obligation, not skipped. |
| **gate-integration** | `cargo xtask lints` | The file-reading lint family is green and **unchanged** — `lint-constitution` still passes over `standards/rust/`, proving this story perturbed neither the tree it cites nor the harness (`verify.reachability_static`, project.md DoD). |
| **gate-integration** | `cargo xtask spec-trace` | `spec/SPECIFICATION.md`'s markers and citations still resolve (NF-007). Load-bearing precisely because this story's deliverable is a file the compiler never reads, and because RP-20-2 class 2 is *about* clause ids. |
| **gate-integration** | `cargo xtask ci --fast` | The bar a non-terminal project is held to (`.redkiln/config.yaml`, `verify.integration_scoped`; project.md DoD). Green at merge. |
| **gate-integration** | `cargo xtask ci` | The merge gate of record (`CLAUDE.md`, "Commands"). Run before the story is called done. |
| **gate-integration** | `cargo xtask affected --base main` | The story grain (`verify.affected_gate`). Expected to widen to the whole workspace and to be **green**; the widening is recorded as observed, per EC-010 (`xtask/src/affected.rs:209-224`). |
| **gate-state** | `git diff main -- xtask` | **AC-007's negative half.** Empty. No `PERMITTED_FOLD_MECHANISMS` const, no marker scanner, no fold check, no mount. Nothing inside this diff can prove a *different* tree was never touched except the diff itself. |
| **gate-state** | `git diff main -- standards/rust .kb spec .redkiln/templates docs/README.md '**/Cargo.toml'` | The PR boundary and NF-001/NF-006/NF-007/NF-008 in one command. Empty. The `standards/rust` half is project AC-002 discharged architecturally — by never opening that tree for writing. |
| **gate-state** | `git diff main -- standards/pages/00-one-need.md standards/pages/10-the-need-set.md` | **AC-003's negative half.** Empty: band 20 pays band 00's deferral by *answering it in band 20*, not by editing band 00. |
| **mechanical (procedural, captured)** | the ~20 `test -f` / `rg` / `grep` / `awk` / `wc` / `sed` invocations named in the acceptance table | **AC-001 … AC-009.** Reproducible, and each one's **output** — never a claim about it — enters `_ledger.md` (NF-004). These are the checks that run *now*, in place of the static tier that cannot exist in this PR. |
| **procedural (ledger-recorded)** | the non-author verdict walk over a `<details>`-wrapped limitations paragraph (AC-001); the three-fragment class-membership classification (AC-002); the band 00 ↔ 20 round trip (AC-003); the "may I fold today, and what would change that" read-back (AC-004); the entry-procedure dry run (AC-005); the rustdoc-wrapper two-answer read-back (AC-006); the four-limits read-back (AC-007); the plain-pager read (AC-008); the intent→row→file traverse (AC-009) | The judgements no command carries — and the ones project AC-005 is actually made of, since its bar is *a reviewer applies it without the author*. `.redkiln/config.yaml`'s `require_ledger: true` treats a recorded manual verification as first-class proof, which is why the testing brief made this a named tier rather than an excuse. Each entry records **who walked it** (not the author of the atom) and **what verdict they reached**. |
| **procedural (reversibility)** | `git checkout -- standards/pages && git status` | Nothing this PR adds leaves residue: no cache, no generated artifact, no dirty file. UX-008's procedure applied to a prose PR. |
| **end-to-end/fixture** | *(none — deferred, by design)* | Breaking a page and watching the gate name the file and line is `declaration-check-seen-to-fail`'s whole story, and no gate step reads `standards/pages/` yet. Nothing here pretends to that evidence — and saying so is AC-007's own subject matter. |

**Merge gate, one line.** `cargo xtask ci` green; `cargo xtask affected --base main` green
(widened, recorded); all three `git diff` boundary checks empty; every mechanical measurement
captured with its output; every procedural walk recorded in `_ledger.md` with the walker
named and the verdict cited.

## Risks and coupling (PR-scoped)

| Risk | Coupling it runs through | Mitigation in this PR |
| --- | --- | --- |
| **The atom becomes a second copy of `_design.md`.** DT-8's three rejected options are vivid and well argued, and the instinct is to re-tell them in the rule. Doing so spends the byte ceiling on prose already written down and creates two accounts that will drift. | `_design.md` `### S1/S3 — DT-8` → the atom; and `playbook-atom-staged-for-ingest`, whose material this is | EC-007 makes the rejected options the **first** thing cut when the budget bites; the Context pack states the boundary as a decision (*cite the decision, spend the atom's bytes on the applied rule*); `Evidence.` cites `_design.md` by path rather than quoting it. |
| **The unenforced-const temptation.** `NEEDS` will be a `const` in `xtask/src/lint_pages.rs` one slice later, and `PERMITTED_FOLD_MECHANISMS` reads like its sibling. It is not: `NEEDS` has two machine consumers this initiative builds; this list has none. | `xtask/src/lint_pages.rs` (slice-mate's) ← the temptation; `RUNBOOK.md:920-925`, `:931-935` (what it costs) | EC-003 names the failure; AC-007 makes `git diff main -- xtask` empty a merge condition and `rg PERMITTED_FOLD_MECHANISMS xtask/` a no-match check; the Context pack states why the `NEEDS` precedent does not transfer. |
| **The empty permitted list is read as an oversight and quietly filled.** An empty table in a rules tree looks unfinished; the first contributor with an accessibility claim from upstream docs will want to fix it. | `_design.md` `## Sign-off` condition 3 → the atom → HS-P0020's DT-7 | AC-004 requires the atom to say *folding is forbidden in practice* **in those words** and to name what lifts it; AC-005 makes the entry cost four recorded observations *in this repository*; EC-004 states the refusal and where a disagreement goes instead. |
| **The renderer-wrapper answer lands as one half.** It is genuinely easier to write either half alone, and each alone is wrong in a different direction. | `_design.md` `## Mock` finding 2; UX-011; HS-P0020's hosting decision (open) | AC-006 makes **both** sentences the criterion and EC-006 states each single-half failure by name. The unmet property is recorded as *owed by HS-P0020's hosting decision*, so the debt has an owner rather than a hope. |
| **`Load when:` wraps, and the index row this PR commits truncates mid-phrase.** The precedent's own router shows the defect today, which makes it easy to reproduce by copying. | `xtask/src/lint_constitution.rs:247-257` → the row → `page-need-checker-mounted-in-the-gate`'s first `--write` | EC-002 states it; AC-001 checks the source line directly (`rg -A1`); AC-009 checks the *consequence* — the committed trigger cell is not mid-phrase. `router-precedence-and-announcement` put the constraint in the router's `## The shape of a rule` one slice earlier for exactly this atom. |
| **The index row's bytes disagree with the generator, and the surprise surfaces in someone else's PR.** | this story → `page-need-checker-mounted-in-the-gate` (a coupling by **value**: bytes, not an interface) | AC-009 pins the row to `generated_region`'s format by line (`xtask/src/lint_constitution.rs:400-420`) and requires a row-by-row derivation record in the ledger, which that story's implementer can read. NF-002 covers the CRLF route to the same failure. |
| **A slice-mate or reviewer proposes a sixth never-fold class during authoring.** It will look like diligence. | `_design.md` `## Sign-off` condition 3 → the closed list | EC-005 states the asymmetry as the answer and routes the disagreement to the design stage. The atom itself carries the amendment procedure for both lists, so the next person meets the rule rather than the argument. |
| **The affected gate widens and someone "fixes" it with a lone `INERT` entry.** | `xtask/src/affected.rs:209-224`, `:249-266`, `:116-125` | EC-010 names the correct treatment as a **pair** of edits owed by the next slice, and `git diff main -- xtask` empty makes a stray edit a merge blocker. Stated in the Context pack, EC-010 and NF-005 — three times, because `router-precedence-and-announcement` recorded the same widening and it will look like a regression twice in a row. |
| **The atom folds something.** A long `Evidence.` list or a table of the five classes invites a `<details>`. | `_design.md` anti-pattern 6, `## Transience policy` row S3 | AC-008's absence greps run over the whole tree, and the `Evidence.` transience row states the eligibility and the prohibition together so the temptation is met by an already-decided answer rather than by judgement. |

## Dependencies

**Blocks on** (must be merged before this story's mount can be authored):

- **`router-precedence-and-announcement`** — creates `standards/pages/README.md`, the only
  composition root a rule atom has in this tree. It fixes the two mount regions (the
  `<!-- BEGIN GENERATED -->` / `<!-- END GENERATED -->` index and the `## Start here` filter
  with its 12-row ceiling), the band table whose band-20 row this atom fills, the router's
  8,192-byte ceiling, and — decisively for AC-001 and AC-009 — `## The shape of a rule`,
  including the constraint that `Load when:` occupies exactly one source line. Without it,
  this story has an atom and no way for a reader to find it (EC-001).

Transitively, through that story: **`need-vocabulary-and-declaration-form`**, which lands
`standards/pages/00-one-need.md` with the never-occluded rule and the mechanism question
deferred to band 20. AC-003 pays that deferral, so band 00 must exist to be paid; it is not a
direct `depends_on` edge because the story map routes it through the router, and the storymap's
edge set is the merge order (`_storymap.md`, "Merge order", 1.1 → 1.2 → 1.3).

**Unlocks** (each `depends_on` this story, per `_storymap.md` and this item's `blocks`):

- **`page-need-checker-mounted-in-the-gate`** (HS-S0150) — needs a **non-empty** rules tree
  with more than one atom, or its own vacuity guard fails it correctly on the first run. It
  inherits this PR's index row as bytes its `--write` must reproduce exactly, and it inherits
  the *absence* this story insisted on: no fold check, and a "what this does not verify"
  section that must name the fold rule's textual limit as item 3.
- **`playbook-atom-staged-for-ingest`** (HS-S0153) — the playbook records the method, the
  rejected alternatives and the conditions under which the discipline stops holding. DT-8's
  rejected options and the empty permitted list are among its material, and it is last in
  merge order because those are only true once the discipline has stopped moving.

**Slice-mates** (implemented in one context and mounted as one surface — the
`discipline-on-disk` milestone): `need-vocabulary-and-declaration-form`,
`router-precedence-and-announcement`, `reviewer-and-citation-procedures`. The last is
**independent of this story** — either order — and it is the one that *cites* this rule from
band 40's reviewer walk, which is the only instrument that observes RP-20-1's content at all.

## Anchors (progressive disclosure)

Load-bearing depth, deferred rather than pasted. Each row says **why** it is load-bearing and
**when** to open it, and is bound to the AC it serves. Every path was confirmed to exist in
this worktree; `standards/pages/README.md` and `standards/pages/00-one-need.md` are
deliberately absent from this table because they do not exist yet — they are the dependency's
output and this story's mount point, named in the Integration contract instead.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `.bklg/docs-that-teach/page-need-discipline/_design.md` | **Binding, signed off.** `### S1/S3 — DT-8` carries the three parts, the two rejected options and the failure-mode asymmetry this atom converts into rules; `## Surfaces` pins `rule-atom` and `RULE_DIR`; `## Composition` S3 fixes the atom's top-to-bottom order; `## Transience policy` rows S3 decide what may fold; `## Density budget` carries every number and the yield order; `## Anti-patterns` 5, 6, 13, 14, 15 are the named wrong states; `## Mock` finding 2 is AC-006's whole subject; `## Sign-off` condition 3 is the amendment path for the closed list. This story implements it and may not contradict it. | **Before writing the first rule**, and again before each measurement and before answering the renderer-wrapper question. | AC-001, AC-002, AC-004, AC-006, AC-008 |
| `.bklg/docs-that-teach/_discovery/distillation/interaction-patterns.md` | The primary evidence, to be **quoted, not paraphrased**: `:404-410` the deletion test verbatim; `:484-497` tension 2 and the *"use good judgment"* non-answer with this repository's own drift precedent; `:205-216` the `mdbook-tabs` unknown — *"an unverified property, not a confirmed safe one"*; `:220-225` the gate step that looked wired and was not; `:436-443` the anti-pattern against adding an affordance the medium already renders. | **When writing RP-20-1's rule sentence and RP-20-3's entry procedure**, with the file open — the wording is the deliverable. | AC-001, AC-002, AC-005 |
| `standards/rust/00-prime-directives.md` | `:1-9` is the atom head grammar AC-001 checks — `# NN — Title`, the one-line `> **Load when:**`, the `> **See also:**` naming siblings **by number**. `:19,30,54,78` is the worked five-section rule shape (`Why.` · `Do` · `Not` · `Rejects.` · `Evidence.`) in the only form the gate holds up today. | **First**, read one whole rule before drafting; re-open at `:7-9` when writing `See also`. | AC-001, AC-003, AC-008 |
| `xtask/src/lint_constitution.rs` | The grammar as **code**, which is what makes it a system rather than a habit: `:67-81` the five section markers in order, `:82` `MIN_REJECTS_CHARS` and the doc comment explaining why it is a length, `:88,95` the rule and byte ceilings this tree copies on purpose, `:247-257` the `Load when` first-line-only parser (EC-002's mechanism), `:258-284` how a rule's body ends at the next `## ` (why a closing section is safe), `:400-420` the exact index-row bytes, `:334-384` index equality and the `--write` repair. | **Before writing the atom head** (`:247-257`), **before the closing statement** (`:258-284`), and **immediately before authoring the index row** (`:400-420`). | AC-001, AC-008, AC-009 |
| `standards/rust/81-checks-that-cannot-be-types.md` | `:11` is RS-81-1 — *"a check whose limits are undocumented is read as a guarantee"* — which is simultaneously the **reason never-fold class 5 exists** (a blind spot behind a disclosure is an undocumented blind spot wearing a costume) and the **bar the atom's own closing statement must clear**. One citation, two obligations. | **Before writing class 5**, and again before the closing "what this rule does not do" statement. | AC-002, AC-007 |
| `.bklg/docs-that-teach/page-need-discipline/_decomposition.md` | The three briefs. **Architecture**: Note 3 items 3 and 5 (the checker's stated limits, one of which is the fold rule's), Note 4 (no `rust` fence, and no untagged one), Note 6 (no shared abstraction — RS-81-3), Note 8 (what must not move). **UX**: UX-003, UX-009, UX-011, UX-012, Note 2's five interaction invariants, and Note 3's *"Do not implement a fold-checker here"*. **Testing**: the five tiers, AC-005's placement in the procedural tier, and the closing paragraph that scopes this story away from DT-7. | **UX Note 3 before RP-20-3's entry procedure; Architecture Note 4 before any fence; the Testing brief before filling the ledger.** | AC-005, AC-007, AC-008 |
| `.bklg/docs-that-teach/page-need-discipline/router-precedence-and-announcement/spec.md` | The dependency's own contract, and the reason AC-009 is mechanically checkable: its **AC-004** fixes the generated region's header, separator and row format byte-for-byte against the precedent, and its **AC-005** fixes the one-source-line `Load when:` constraint in `## The shape of a rule`. Reading it is how this atom inherits a shape rather than inventing a compatible-looking one. | **Immediately before authoring either router row.** | AC-009, AC-001 |
| `.bklg/docs-that-teach/page-need-discipline/need-vocabulary-and-declaration-form/spec.md` | `:246-247` is the deferral in the slice-mate's own words — band 00 states the declaration is never occluded and defers *which mechanisms count* to band 20 — and `:300` and `:354` are the ACs that made band 00 point here by number without settling DT-8 in passing. AC-003 is the payment of exactly that debt. | **Before writing `> **See also:**` and never-fold class 1.** | AC-003, AC-002 |
| `.bklg/docs-that-teach/page-need-discipline/design/mock.html` | The built contact sheet: the `rule-atom` frames (`populated`, `over-rule-ceiling`, `over-byte-ceiling`, `missing-section`, `rust-tagged-fence`) with density chips computed from each specimen's own bytes, and the closing findings panel whose **finding 2** is AC-006's entire subject. It shows what "over ceiling" looks like rather than describing it. | **When a measurement comes out close to a ceiling**, and **before answering the rustdoc-wrapper question**. | AC-006, AC-008 |
| `.bklg/docs-that-teach/page-need-discipline/project.md` | `:221-224` is project AC-005 in the charter's own words — the bar this story is graded against, including *"in a form a reviewer applies to a page without consulting the author"* — and `:246-270` is the Definition of done this story's gate commands come from. The authority for what "traces to" means here. | **When filling the ledger**, to confirm AC-005 is discharged rather than merely mentioned. | AC-001, AC-002 |
| `.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md` | `:174-181` is this initiative's one **measured** defect — the `E0034` explanation that existed in three contributor-facing documents, none of them the file the reader was looking at. A rule that exists and is unreachable is that defect wearing this story's name, which is why mounting is an AC and not a formality; `:225-281` and `:305-309` are the evaluator who gets one reading session and no second attempt. | **Before writing the `## Start here` row's wording** — write it in a reader's voice, not a deliverable's. | AC-009 |
| `RUNBOOK.md` | `:920-925` and `:931-935` are the two incidents this rule's shape is calibrated against: a gate step that printed warnings and exited 0 because nothing read its output. They are why an unenforced `const` is rejected (EC-003) and why *unverified counts as unmet* is earned rather than borrowed (AC-005). | **If the "why not just add the const" question comes up**, and before writing the entry procedure. | AC-005, AC-007 |
| `.bklg/docs-that-teach/page-need-discipline/_grounding.md` | Records that **no Accepted decision atom under `.kb/decisions/` constrains this project and none may be written for it** — all seventeen were read by title — and that the binding authority is sub-ADR. It also flags the initiative-level non-goal against extending the precedence chain, which is why an ADR here would itself be a breach. | **If the temptation to write an ADR appears** — it will, because DT-8 is a real decision with rejected options. | AC-007 |
| `.bklg/docs-that-teach/page-need-discipline/_storymap.md` | The slice boundaries, the Coverage table (`AC-005 … sole owner`), the merge order that makes this story 1.3, and the first "Why the slices fall here" bullet — the half-mount this story's AC-009 exists to refuse. | **If the scope of a rule is questioned**, or before deciding whether something belongs to this story or a slice-mate. | AC-009, AC-002 |

## Clarifications resolved during spec

1. **The AC set is exactly the nine the front half enumerated** — AC-001 through AC-009, none
   added, none dropped. `_ledger.md` carries nine rows with the same ids and criteria verbatim.
2. **The static test tier is empty in this PR, and that is a decision rather than an
   omission.** The testing brief's static assertions for a rule atom's shape (five sections,
   `Rejects.` length, the ceilings, the fence rejection) and for the router (index equality,
   link resolution) are `#[cfg(test)]` tests inside a module that does not exist until
   `page-need-checker-mounted-in-the-gate` — and this story's PR boundary forbids touching
   `xtask/src/**`, with `git diff main -- xtask` empty as a merge condition. Each such
   assertion is named in the Tests and CI table's `static` row as an obligation that story
   inherits, and each AC's *present-tense* verification is the mechanical `rg`/`awk`/`wc`
   check named in its row. This is the "procedural (ledger-recorded)" tier the testing brief
   created for exactly this shape, plus a stated forward obligation — not an unverified AC.
3. **Three rules, numbered RP-20-1, RP-20-2, RP-20-3, in DT-8's own part order.** The front
   half already cites *"RP-20-2 class 1"* and *"RP-20-2 binds the author's own markup"*, so
   the numbering was fixed by prior citation and is recorded here rather than left to the
   implementer. The renderer-wrapper sentences and the amendment asymmetry are carried
   **inside** those rules, not promoted to rules of their own: a rule whose `Not` names no
   wrong page is decorative (anti-pattern 14), and three of six rules leaves the headroom the
   `Not` and `Rejects.` sections need.
4. **The closing "what this rule does not do" statement stays at the close, and may take a
   plain `## ` heading.** RS-81-1's *"stated first"* applies to a **checker's own module
   documentation** (`xtask/src/lint_constitution.rs:9-28`); this atom is prose, not a check,
   and the front half placed the statement at the close. That placement stands. The mechanical
   question it raised — whether a non-rule `## ` section breaks the shape parser — was
   checked: `rules()` counts only headings prefixed `## RP-`/`## RS-` and ends a rule's body
   at the next `## ` of any kind (`xtask/src/lint_constitution.rs:258-284`, pinned by the test
   at `:871`), so a trailing section neither becomes a seventh rule nor swallows the sixth.
5. **`PERMITTED_FOLD_MECHANISMS` is a table in the atom and never a Rust `const`** — decided
   in the front half, restated here because it is the single most likely "improvement" a
   reviewer will suggest. The rejected alternative and its cost are recorded (EC-003), and the
   check that catches it is mechanical (`rg -n 'PERMITTED_FOLD_MECHANISMS' xtask/` → no
   matches), not a convention.
6. **Reversibility and preserved focus are discharged by absence, not by a mechanism.** UX
   brief Note 2's third and fourth invariants presuppose something that opens. This PR authors
   no disclosure and no script, so both are satisfied by AC-008's absence greps plus the
   `git checkout --` residue check — stated explicitly so a later reader does not read the
   thin rows as an oversight. The one disclosure in the picture is the renderer's, and AC-006
   is where it is accounted for.
7. **No ADR is written, and none may be.** `_grounding.md` records that no Accepted decision
   atom governs documentation trees, fold mechanics or narrative conventions, and that writing
   one here would breach the initiative's non-goal against extending the precedence chain. The
   binding authority is sub-ADR: `CLAUDE.md`, `standards/rust/README.md:23-29`, and the
   signed-off `_design.md`. `RUNBOOK.md` carries no phase for this initiative, so an ADR is
   not this story's to write in any case.
8. **The `cargo xtask affected --base main` widening is a green observation, not a
   regression** — stated in the Context pack, EC-010 and NF-005. It is repeated because
   `router-precedence-and-announcement` recorded the same widening one slice earlier and the
   second occurrence is exactly when someone "fixes" it in the way that makes a prose-only PR
   read nothing.
9. **`reviewer-and-citation-procedures` is a slice-mate, not a dependency, and the direction
   of citation runs from band 40 to band 20.** Band 40's walk *cites* RP-20-1 as a step; band
   20 does not depend on band 40 existing, and it must not restate band 30's citation rule —
   class 2 names clause citations as never-foldable **by citing band 30**, because the
   discipline's own rule against becoming a second specification applies first to the band
   that writes about citations.
