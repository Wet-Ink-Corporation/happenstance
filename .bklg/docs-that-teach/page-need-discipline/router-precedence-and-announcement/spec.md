---
item: HS-S0147
stage: spec
created: 2026-08-17T13:16:08.169Z
updated: 2026-08-17T13:16:08.169Z
template_sig: 87bbf1d0
rendered_sig: 6013dd0d
---

# Spec — The rules tree, its router, its rank in the chain, and its announcement

## Scope lock

| Level | Path and the part that binds this story |
| --- | --- |
| Initiative (gold source) | `.bklg/docs-that-teach/initiative.md` — BR-12 (where a documentation discipline lives), AC-13, **DoD-14** (`:462-464`, "the discipline is on disk and cited"); the non-goal against extending the precedence chain |
| Initiative decomposition | `.bklg/docs-that-teach/_decomposition.md` — the BR-12 gate decision (a new tree, sibling to `standards/rust/`, pinned by path) and the HS-P0020 seam |
| Project | `.bklg/docs-that-teach/page-need-discipline/project.md` — **AC-001**, **AC-002**, **AC-011**; DR-01, DR-02, DR-10 |
| Briefs (key) | `.bklg/docs-that-teach/page-need-discipline/_decomposition.md` — Architecture brief AC-001 / AC-002 / AC-011 and Notes 1 (CR-4), 5 (the generated region), 6 (no shared abstraction), 9 (decisions left open); UX brief **UX-004**, **UX-005**, **UX-012** and Note 1's primitive table; Testing brief AC-001 / AC-002 / AC-011 |
| Signed-off design (**BINDING**) | `.bklg/docs-that-teach/page-need-discipline/_design.md` — `## Surfaces` (`discipline-router`, `rule-atom`), the pinned-constant table, `### S2 — discipline-router`, `### S2 — DR-02`, `## Composition` (router top to bottom), `## Density budget`, `## Anti-patterns` 7-9 and 13, `## Sign-off` condition 2 |
| Story map | `.bklg/docs-that-teach/page-need-discipline/_storymap.md` — slice `discipline-on-disk`; this story's row; "Why the slices fall here", first bullet |
| Grounding | `.bklg/docs-that-teach/page-need-discipline/_grounding.md` — no Accepted decision atom governs documentation trees; the binding authority here is sub-ADR |
| Roadmap pointer | **None.** `RUNBOOK.md` carries no phase for this initiative (`rg docs-that-teach RUNBOOK.md` → no matches). The roadmap of record is the story map's merge order. |
| Design mock (built, awaiting sign-off) | `.bklg/docs-that-teach/page-need-discipline/design/mock.html` — the `discipline-router` frames, including `generated-region-stale` and `dangling-link` |

## One-line PR slice

Stand the tree up sibling to `standards/rust/` with a router carrying both an
intent-keyed trigger table and a complete index, state its rank inside the five-tier
chain without editing `standards/rust/README.md:23-29`, and take its row in
`docs/README.md`'s table and its name in the gate-read-trees paragraph.

## Executive summary

**What this PR lands.** `standards/pages/README.md` — the router for the page-need
discipline — and the two edits to `docs/README.md` that make the tree reachable from
the repository's own index. After this PR a contributor who is about to write a
narrative page can start at `docs/README.md`, land on the router, read a band table
and one `## Start here` row, and open exactly one rule file; and a contributor who
wonders whether a page rule outranks a `SPECIFICATION.md` clause gets the answer on
the page they landed on rather than by opening a second file.

**Pointer, not restatement.** The *rules* are not this story's content: the closed
need set and the declaration form arrive with `need-vocabulary-and-declaration-form`
(this story's only dependency), the fold line with `fold-line-rule`, the citation
rule and the reviewer walk with `reviewer-and-citation-procedures`. This story lands
the **navigation, the rank and the announcement** — the three things that turn a
directory of markdown into a tree a reader arrives at on purpose.

**The delta worth reading twice.** Three decisions in this PR are load-bearing beyond
it, and each is stated as a decision in the Context pack rather than left to the
implementer:

1. The `## Index` is a **generated region** whose exact byte shape is fixed *here*,
   one slice before the checker that regenerates it exists. Get the shape wrong and
   the checker story's first `--write` produces a diff nobody intended.
2. `docs/README.md:25-29` claims which trees the gate reads. At this merge no gate
   step reads `standards/pages/`, so naming it there unqualified would author a false
   sentence into the file whose whole subject is that pins are honest.
3. `standards/pages/` is unrecognised by `xtask/src/affected.rs`, so this PR's
   `affected` run widens to the whole workspace. That is correct-and-slow, and it is
   **not** fixed here — the fix is a pair of edits that only makes sense with the
   checker.

## Context pack

The decisions this story must honor, stated inline. Everything deeper is a signposted
anchor in the second half of this spec — link, do not paste.

**The home was decided at decomposition, and the design pinned its name.** The
discipline lives in a **new tree, `standards/pages/`, sibling to `standards/rust/`**,
inside the existing precedence chain without extending it. `_design.md`'s
pinned-constant table fixes `RULE_DIR = standards/pages` and
`ROUTER = standards/pages/README.md`; the architecture brief's Note 9 had left the
directory name free and warned that it becomes a `const` two files depend on **by
value**, and `_design.md` sign-off condition 2 records the cost of renaming it later
("renaming it later is not a rename"). Use those two spellings exactly. The four
rejected homes and the cost of each are already written down in `_design.md` — do not
re-argue them, and do not re-derive them into the router's prose either.

**No Accepted decision atom constrains this story, and none may be written for it.**
All seventeen atoms under `.kb/decisions/` were read by title at grounding; none
governs documentation trees, gate structure or narrative conventions. The binding
authority is sub-ADR: `CLAUDE.md`, the precedence block at
`standards/rust/README.md:23-29`, and the pin-by-path convention stated in prose at
`docs/README.md:25-29`. Writing an ADR here would itself breach the initiative's
non-goal against extending the chain.

**The rank is decided: the constitution-atom tier, alongside `standards/rust/`.** The
router states, in its own text, that the discipline sits at the **constitution-atom
tier** of `SPECIFICATION clause > ADR > constitution atom > CLAUDE.md / CONTRIBUTING.md
summary > references/evaluation/*` — on the same rank as `standards/rust/`, scoped to a
different subject. It adds no tier. Two rejected options and why they lost: a **sixth
tier** is an initiative-level non-goal; **leaving the rank unstated and linking to the
chain** fails UX-005, because a reader who must open another file to learn whether a
page rule beats a clause will guess instead. The checkable consequence is that
`standards/rust/README.md` is never opened for writing — `git diff main --
standards/rust/README.md` is empty at merge.

**The router filters without hiding what it filters.** Both halves are mandatory and
on one page: an intent-keyed `## Start here` table (the filter) *and* a complete
`## Index` of every rule (the thing filtered), the index present and complete whether
or not the filter matched. `standards/rust/README.md` is the working proof at
twenty-seven-atom scale — `:45-59` is the filter, `:61-97` is the unfiltered whole.
A filter-only router violates UX-004; a "show all rules" toggle over the index is
anti-pattern 7 in `_design.md`; an index that lists only some rules is anti-pattern 8.
Length is explicitly **not** a reason to fold the index (`_design.md`, Transience
policy, S2 generated `## Index`).

**The index is generated, and this story fixes the shape one slice before the
generator exists.** The mechanism is `check_router`'s: a region between
`<!-- BEGIN GENERATED -->` and `<!-- END GENERATED -->`, compared for **equality**
against what the corpus says, with the repair printed in the failure message
(`xtask/src/lint_constitution.rs:332-385`, markers parsed at `:388-397`, rows built at
`:400-420`). For `standards/rust/` that region is a convenience over a corpus the
compiler also reads; **here it is the only mechanism preventing the enumerated need
set from existing twice** (architecture brief, Note 5), which is why a hand-maintained
index is rejected — `check_summaries` states the general form of that failure at
`lint_constitution.rs:466-470`: *"a summary that does not point at the corpus is a
second copy of it, and one of the two will be stale."* The consequence for **this**
story is a seam obligation, not a nicety: the checker lands in the next slice
(`page-need-checker-mounted-in-the-gate`), so the region authored here must be
byte-identical to what that checker's `generated_region` analogue will emit, and the
first `--write` run against this router must produce **no diff**. This story therefore
*fixes the contract* — a header row, a separator row, and one row per rule atom
carrying three cells in this order: a markdown link to the atom file, the atom's
`Load when` line, and its comma-separated rule ids, exactly as
`lint_constitution.rs:400-420` builds them — and the checker story implements to it.

**One mechanical trap in that generator, and it changes how the atoms are written.**
`load_when` reads only the **first source line** of a `> **Load when:**` block
(`lint_constitution.rs:247-255`); continuation lines on subsequent `>` lines are
silently dropped, which is why `standards/rust/README.md:70-71`'s index rows end
mid-phrase ("reaching for a"). The constitution tolerates that because its atoms are
found by band. This tree must not: the router's `## The shape of a rule` section states
that a rule atom's `Load when:` is **one source line**, so the generated row is a whole
trigger phrase rather than a truncated fragment. This is a rule about the corpus that
only the router can state, and it is stated here because the slice-mates author their
atoms against it.

**`docs/README.md` is the mount point, and one of its two edits is a claim about the
gate.** The table at `:12-24` answers "Looking for / It is at" and takes a new row —
that sentence is true the moment it is written. The paragraph at `:25-29` is different:
it says *"Two of those are read by the gate rather than only by people"*, names
`cargo xtask spec-trace` and `cargo xtask lint-constitution`, and closes with the rule
this whole project rests on — *"Moving either tree means editing `xtask/src/` in the
same change — which is the point of pinning them by path rather than by convention."*
At **this** story's merge nothing reads `standards/pages/`. So the third tree is named
there **with a maturity marker in the repository's own idiom** —
`[PROVISIONAL — settles at …]`, the form `standards/rust/README.md:109-111` already
uses — naming `page-need-checker-mounted-in-the-gate` as what lifts it, and that story
removes the marker in the commit that makes the sentence true. The alternative was
deferring the whole paragraph edit to the checker story; it lost because the row and
the paragraph answer different questions, and a reader of `:25-29` would otherwise
believe the pin list is complete while a third pinned tree sat one directory away. The
governance test for touching text that already discharges something applies and is
satisfied in this shape: the referent is rewritten, the reasoning is not
(`.kb/governance/rewrite-the-referent-never-the-reasoning.md`).

**What checks this tree at this merge: nothing — and the router says so, first.**
RS-81-1 (`standards/rust/81-checks-that-cannot-be-types.md:11`) requires a check's
blind spot to be stated in its own documentation because *"a check whose limits are
undocumented is read as a guarantee"*; the same failure applies one level up to a tree
whose reader assumes the gate is watching it. `standards/rust/README.md:114-127` is the
in-repo shape to copy (`## What the gate checks, and what it does not`). The router's
analogue states at this merge that **no gate step reads `standards/pages/` yet**, names
the story that adds one, and names what will still not be checked after that (whether a
page *answers* its declared need — DR-07's reviewer procedure is the instrument).

**Corrected 2026-08-18 — this paragraph, and AC-006's THEN, were overtaken inside their
own slice.** The clause *"no gate step reads `standards/pages/` yet"* was true of the
merge this spec was written against and false by the time the slice landed: `xtask` is a
workspace member, `cargo xtask ci`'s mandatory `tests` step is
`cargo test --locked --workspace --all-features`, and `xtask/src/lint_pages.rs`'s test
module reads every atom in this tree by name — a dangling router link, a missing
`**Rejects.**`, an over-96-column prose line or a `rust`-tagged fence turns the gate red
today. What is actually missing is the **dedicated** `lint-pages` step and the corpus
reader that walks the directory rather than a hand-written list of filenames, both
`page-need-checker-mounted-in-the-gate`'s. The router says that instead, and
`::router_states_what_checks_this_tree_and_what_does_not` pins the corrected sentence and
rejects the old one. RS-81-1's bar is unchanged and is the reason for the correction:
state the blind spot first, and state it accurately.

**The affected-gate consequence, accepted rather than fixed.**
`affected_packages` has an arm for prose outside every workspace member: `README.md`
and `standards/rust/` select `xtask` because their examples compile as that crate's
doctests, and **anything unrecognised widens to the whole workspace**
(`xtask/src/affected.rs:209-224`). `standards/pages/` is not on `INERT`
(`:249-266`) and does not match the literal `standards/rust/` prefix, so this PR's
`cargo xtask affected --base main` compiles and tests every package. That is safe and
slow. The correct treatment is a **pair** of edits — the new prefix on `INERT` *and*
the checker on the unconditional file-reading list at `:116-125` — and the second half
cannot exist before the checker does (architecture brief, Note 1, CR-4). Adding the
`INERT` entry alone here would make a prose-only pull request read **nothing**, which
is the half-mount the story map names explicitly. So: do not touch
`xtask/src/affected.rs` in this PR; record the widening as observed.

**The persona-journey slice this realizes.** Two readers, and the second is why the
announcement is in this story rather than a later one.

- **The next page author, and the reviewer** (surfaces S2 in the UX brief) want *the
  one rule that governs the thing they are about to write, without reading the corpus*.
  UX-012 is their bar: name a task, follow one trigger row, read one file, stop. The
  ceiling that makes it hold is enforced rather than aspirational — six rules and
  16,384 bytes per atom in the precedent (`lint_constitution.rs:88,95`), and
  `_design.md` sets the router's own ceiling at **8,192 bytes**, half an atom, because
  the router is the one file *every* page author loads and a router at atom scale is a
  corpus.
- **The reader who arrives from the repository's index** is the measured defect one
  level up. The adapter author's `E0034` explanation exists — in `RUNBOOK.md`, an ADR
  and evaluation documents, *"all contributor-facing, none of them
  `crates/happenstance-core/src/store.rs`, the file the reader is looking at at the
  moment they need it"*
  (`.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md:174-181`).
  A rules tree that exists and is unreachable is that defect wearing this story's name,
  which is exactly why the story map keeps the `docs/README.md` announcement inside
  the `discipline-on-disk` slice instead of deferring it.

**What this story is forbidden from doing, stated as decisions.** No new
abstraction shared with `lint_constitution` (architecture brief, Note 6 — RS-81-3
scopes a scanner to the directory whose behaviour it constrains). No `rust`-tagged
fence anywhere in `standards/pages/`: nothing in the workspace compiles this tree
(`xtask/src/lib.rs:28` registers only `mod constitution`), so a `rust` fence is a Rust
claim nothing checks — `_design.md` anti-pattern 13, and the inversion is explained in
architecture brief Note 4. No bespoke navigation widget: the evaluator's gap is *a
missing link, not a missing widget* (`_design.md` anti-pattern 9). No page-need index
generated into the router — `_design.md` declines it deliberately, because its source
is HS-P0020's tree and adopting it would couple two projects' commit cadence.

## Integration contract

**Slice / milestone.** `discipline-on-disk`. Slice-mates, implemented in one context
and mounted as one surface: `need-vocabulary-and-declaration-form` (this story's
dependency), `fold-line-rule`, `reviewer-and-citation-procedures`.

**Archetype.** `capability` — a user-observable slice. The observable user is a
contributor navigating from the repository's index to one rule.

**Mount point.** **`docs/README.md`** — the repository's index, and the only
composition root a documentation tree has here. Two regions, both required:

- the "Looking for / It is at" table, `docs/README.md:12-24` — one new row;
- the gate-read-trees paragraph, `docs/README.md:25-29` — the third tree named, with
  the `[PROVISIONAL — settles at …]` marker described in the Context pack.

A tree with no row in that table is reachable only by someone who already knows it
exists, which is the failure `_storymap.md` names in its first "Why the slices fall
here" bullet.

**Renders surfaces.** `discipline-router` (`_design.md` `## Surfaces`, route
`standards/pages/README.md`) — this story creates it, in the region order
`_design.md` `## Composition` fixes, and is accountable for its `populated`,
`dangling-link` and `narrow-80-column` states. `rule-atom` — **not** rendered here
beyond the band namespace and the authoring grammar the router states; its instances
are the slice-mates'.

**Wires into** (real siblings, by path):

| Consumed | Path | What this story takes from it |
| --- | --- | --- |
| the router shape | `standards/rust/README.md:1-8,10-21,23-29,45-59,61-97,99-112,114-127` | scope paragraph, band table, precedence block, `## Start here`, generated `## Index`, shape-of-a-rule, and the "what the gate checks and what it does not" section |
| the atom head grammar | `standards/rust/00-prime-directives.md:3-9` | the `> **Load when:**` / `> **See also:**` pair the index rows are built from |
| the generated-region contract | `xtask/src/lint_constitution.rs:332-385,388-397,400-420` | the exact markers, the equality semantics, the row format, and the `--write` repair message |
| the `Load when` parser's limit | `xtask/src/lint_constitution.rs:247-255` | why a rule atom's `Load when:` must be one source line |
| the precedence chain | `standards/rust/README.md:23-29` | the five tiers, cited and **not edited** |
| the pin-by-path convention | `docs/README.md:25-29` | the sentence this story extends, and the discipline it states |
| the closed need set (**dependency**) | `need-vocabulary-and-declaration-form` — the `NEEDS` const in the new `xtask/src/` module, and the band-`00` rule atom | the tree is non-empty when the index is authored, and the index's first rows are real |

**Design-system primitives consumed.** Textual, and all enforced today, from the UX
brief's Note 1 table: the one-paragraph scope plus "load one rule, never the tree"
instruction; the band table as a stable numeric namespace; the precedence blockquote;
the intent-keyed `You are… / Load` table; the generated region; `# NN — Title`;
`> **Load when:**` / `> **See also:**`; `## RP-NN-N. <imperative sentence>`; the five
fixed sections **Why.** · **Do** · **Not** · **Rejects.** · **Evidence.**; the
rules-and-bytes ceilings. The rule-id prefix is **`RP-`** — `RS-` is the
constitution's and `PS-` is a live `spec/SPECIFICATION.md` clause family
(`_design.md`, `## Surfaces`).

**Band namespace this story fixes** (five bands, `NN` in `{00,10,20,30,40}` per
`_design.md` `## Surfaces`), with the slice-mate that fills each:

| Band | Owns | Filled by |
| --- | --- | --- |
| `00` | one need per page, and the declaration form | `need-vocabulary-and-declaration-form` (`_design.md` cites `standards/pages/00-one-need.md`) |
| `10` | the closed need set, and the `orientation` ceilings RP-10-2 / RP-10-3 | `need-vocabulary-and-declaration-form` (`_design.md`, DT-3) |
| `20` | the fold line — the deletion test, the never-fold list, the permission gate | `fold-line-rule` (`_design.md` cites `standards/pages/20-the-fold-line.md`) |
| `30` | citations: a page cites a clause id and never restates it | `reviewer-and-citation-procedures` — **the one band `_design.md` names no file for**; assigned here so band `40`'s walk has a rule to cite. Folding it into `20` was rejected: the fold line and the citation rule are applied by different readers at different moments |
| `40` | reviewing a page — the non-author walk, and the paraphrase spot check | `reviewer-and-citation-procedures` (`_design.md` cites `standards/pages/40-reviewing-a-page.md#the-walk`, `## RP-40-1`) |

**Conformance rule(s).** None, and this is not adapter-observable. No port, no crate,
no feature is in this diff; `happenstance-testkit`'s suite observes stores, and a
documentation tree is invisible to it. The instruments that *do* observe this story are
`xtask`-side and arrive in the next slice: the checker's `check_router` analogue (link
resolution and region equality) and its two `docs/README.md` structural assertions
(Testing brief, AC-011). A story that changes a port and names no rule is a port change
nothing can fail; this story changes no port.

**Clause(s).** None discharged, none amended. Nothing here writes, restates or
re-numbers a `spec/SPECIFICATION.md` clause — the discipline requires pages to *cite*,
and `cargo xtask spec-trace` remains the only writer of that file's generated sections
(architecture brief, Note 8). `cargo xtask spec-trace` must still be green at merge.

**Advances DoD scenario.** **DoD-14** — *"The discipline is on disk and cited"*
(`.bklg/docs-that-teach/initiative.md:462-464`). This story moves the first two thirds
to green: the account exists in its decided home, and it is reachable from the
repository's index. The remaining third — *at least one page cites it as the reason it
is shaped as it is* — is `governed-page-cites-the-discipline`, in the
`binding-beyond-this-project` slice.

## PR boundary

**In this PR**

- `standards/pages/README.md` — the router, in the region order `_design.md`
  `## Composition` fixes, with a populated `<!-- BEGIN GENERATED -->` /
  `<!-- END GENERATED -->` region and a `## What checks this tree, and what does not`
  section.
- `docs/README.md` — one new table row (`:12-24`) and the third tree named in the
  gate-read paragraph (`:25-29`) with its `[PROVISIONAL — settles at …]` marker.
- `.bklg/docs-that-teach/page-need-discipline/router-precedence-and-announcement/**` —
  this spec, the `_ledger.md` the second pass defines, and the story's own stage
  artifacts.

The implementer **may** also touch the composition-root files named in the Integration
contract to mount this slice; that is not scope drift. Here the composition root *is*
`docs/README.md`, and it is already in the list above.

**Explicitly not in this PR**

- **`standards/rust/**` — nothing.** AC-002 is discharged architecturally by never
  opening that tree for writing; `git diff main -- standards/rust/README.md` is empty.
- **`xtask/src/**` — nothing.** The path constants, the checker, the `REQUIRED` step,
  the dispatch arm, `print_help`, `lint_steps` and the `affected.rs` `INERT` pair all
  belong to `page-need-checker-mounted-in-the-gate` (architecture brief, Note 1,
  CR-1 through CR-4). Adding the `INERT` entry alone would make a prose-only pull
  request read nothing.
- **The rules themselves.** Bands `00`/`10` are the dependency's, `20` is
  `fold-line-rule`'s, `30`/`40` are `reviewer-and-citation-procedures`'. This story
  authors the band table, not the bands.
- **Any page in HS-P0020's narrative tree**, and any link from one. That is
  `governed-page-cites-the-discipline`.
- **`.kb/**` — nothing.** The playbook atom is staged under `.kb/_intake/` by
  `playbook-atom-staged-for-ingest`; `.kb/maps/domain-map.md` is not edited by this
  project at all.
- **`.redkiln/templates/**` — nothing**, so `redkiln doctor` still reports exactly six
  `template-drift` advisories, and `redkiln adopt --templates` is never run.
- A page-need index generated into the router (deferred by `_design.md`), a
  fold-checker (HS-P0020's DT-7 demonstration), and any shared abstraction with
  `lint_constitution` (architecture brief, Note 6).

```
standards/pages/**
docs/README.md
.bklg/docs-that-teach/page-need-discipline/router-precedence-and-announcement/**
```

**Merge DoD, one line.** `standards/pages/README.md` exists with a complete generated
index and a stated rank; `docs/README.md` reaches it in one hop and does not lie about
who reads it; `git diff main -- standards/rust/README.md` and `git diff main -- xtask`
are both empty; `cargo xtask ci --fast`, `cargo xtask lints`, `cargo xtask spec-trace`
and `cargo xtask affected --base main` are green, the last having widened to the whole
workspace by design.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **The tree exists at its pinned name** | A new directory `standards/pages/`, sibling to `standards/rust/`, holding `README.md` (the router) plus the rule atoms its slice-mates author. The two spellings are fixed and depended on by value: `RULE_DIR = standards/pages`, `ROUTER = standards/pages/README.md`. Renaming later is not a rename. | `.bklg/docs-that-teach/page-need-discipline/_design.md` — pinned-constant table, `## Sign-off` condition 2; `xtask/src/lint_constitution.rs:55,58` for the `const` shape it will take next slice |
| **Region order, top to bottom, is binding** | `# Page standards` → one scope paragraph carrying "load one rule, never the tree" → the band table → `## Precedence` → `## Start here` → the generated `## Index` → `## The shape of a rule` → `## What checks this tree, and what does not`. Order is the only positional language a text medium has: the filter (band table, `## Start here`) is always **above** the thing it filters (`## Index`). | `_design.md` `## Composition`, "S2 — the router, top to bottom"; the same relative order at `standards/rust/README.md:1-21,23-29,45-59,61-97,99-112,114-127` |
| **The filter routes to one rule** | `## Start here` is an intent-keyed table whose two columns are **You are…** and **Load**, at most **12** rows, each row naming one band or one atom. A reader names a task, follows one row, reads one file, stops. | `standards/rust/README.md:45-59`; UX brief UX-012; `_design.md` `## Density budget` (`Start here` rows ≤ 12) |
| **The band table is the numeric namespace** | Five rows — `00`, `10`, `20`, `30`, `40` — each naming what the band owns, sitting immediately after the scope paragraph because it is the cheapest route: a reader who can guess the band never reads the rest of the page. It never yields to a budget. | `_design.md` `## Composition` (region 3), `## Density budget` (yield order, S2); `standards/rust/README.md:10-21` |
| **The precedence rank is answered in place** | `## Precedence` is a blockquote stating that the discipline sits at the **constitution-atom tier** of `SPECIFICATION clause > ADR > constitution atom > CLAUDE.md / CONTRIBUTING.md summary > references/evaluation/*`, alongside `standards/rust/` on the same rank, scoped to a different subject — adding no tier. It sits **above** `## Start here`: a reader who does not yet know whether these rules bind them learns that before being routed to one. | `_design.md` `### S2 — DR-02`; `standards/rust/README.md:23-29`; UX brief UX-005; project `project.md` AC-002 |
| **`standards/rust/README.md` is not opened for writing** | The chain is cited, never edited. The only code in the workspace that writes that file is `check_router`'s `Mode::Write` arm, scoped to its own `ROUTER` const and untouched here. | `xtask/src/lint_constitution.rs:334-385`; architecture brief AC-002 |
| **The index is complete and generated-shaped** | A `<!-- BEGIN GENERATED -->` / `<!-- END GENERATED -->` region containing a header row, a separator row, and exactly one row per rule atom in the tree (`README.md` excluded). Each row carries three cells in this order: a markdown link to `NN-slug.md`, the atom's first `Load when` source line, and its comma-separated rule ids (`RP-NN-1, RP-NN-2, …`). Every rule in the tree appears; a rule no `## Start here` row points at is still reachable by reading the router top to bottom. | `xtask/src/lint_constitution.rs:400-420` (row construction), `:388-397` (marker parsing), `:217` (`README.md` excluded from the corpus); UX brief UX-004; `_design.md` anti-patterns 7-8 |
| **The region's shape is a forward contract on the checker story** | The checker in `page-need-checker-mounted-in-the-gate` compares this region for **equality** and offers `--write` as the repair. Its first `--write` against this router must produce **no diff**: header spelling, separator spelling, column order, link form and `|`-escaping are decided here, in this PR, and that story implements to them. | `xtask/src/lint_constitution.rs:358-383` (equality, and the `--write` repair message at `:376-379`), `:400-420`; architecture brief Note 5 |
| **Every `.md` link in the router resolves** | The link half of `check_router` is copied verbatim next slice and is what discharges half of AC-011's reachability, so the router must not ship a dangling link. Links are relative to the tree, as the constitution's are. | `xtask/src/lint_constitution.rs:343-356`; architecture brief Note 5; `_design.md` `## Surfaces`, `discipline-router` state `dangling-link` |
| **`Load when:` is one source line, and the router says why** | `## The shape of a rule` states the atom head grammar (`# NN — Title`, `> **Load when:**`, `> **See also:**`, `---`, then `## RP-NN-N.` with five fixed sections) **and** the constraint that `Load when:` occupies a single source line, because the generator reads only the first line of the block and silently drops continuations — visible today as mid-phrase index rows in the constitution. | `xtask/src/lint_constitution.rs:247-255` (parser), `standards/rust/README.md:70-71` (the truncation it produces), `standards/rust/00-prime-directives.md:3-9` (the grammar), `_design.md` `## Composition` S3 |
| **The router states its own blind spots, first** | `## What checks this tree, and what does not` says, at this merge: **no gate step reads `standards/pages/` yet**, names `page-need-checker-mounted-in-the-gate` as the step that adds one, and names what will still be unchecked afterwards — that a page *declares* a need is checkable, that a page *answers* it is not, and the reviewer walk in band `40` is the instrument for the rest. | `standards/rust/81-checks-that-cannot-be-types.md:11` (RS-81-1); `standards/rust/README.md:114-127` (the section to model); architecture brief Note 7, items 1 and 5 |
| **No `rust`-tagged and no untagged fence** | Fences in this tree are tagged `text` or `markdown`. Nothing registers `standards/pages/` with the doctest harness, so a `rust` fence is a Rust claim nothing compiles; untagged is rejected too, so a future decision to register the tree cannot be undermined retroactively. The router itself must model this. | `xtask/src/lib.rs:28` (only `mod constitution` is registered); architecture brief Note 4; `_design.md` anti-pattern 13 |
| **`docs/README.md` gains a row that answers a reader's question** | One row in the `:12-24` table, in that table's own voice — a "Looking for" phrase a contributor would actually think ("What makes a narrative page teach, and the rules a page author follows") against a link to `../standards/pages/README.md` labelled `standards/pages/`, in the same relative-link and inline-code form as the rows around it. | `docs/README.md:12-24`; architecture brief AC-011; project `project.md` AC-011 |
| **The gate-read paragraph is extended without becoming false** | `docs/README.md:25-29` currently says *"Two of those are read by the gate…"* and names two commands. It is rewritten to name the third tree with an explicit `[PROVISIONAL — settles at `page-need-checker-mounted-in-the-gate`]` marker, so the sentence is true at this merge, and the marker names its own removal. The closing sentence — moving a tree means editing `xtask/src/` in the same change — is preserved verbatim in meaning. | `docs/README.md:25-29`; `standards/rust/README.md:109-111` (the marker idiom); `.kb/governance/rewrite-the-referent-never-the-reasoning.md`; architecture brief AC-011 ("*or that paragraph becomes false in the same commit that makes it so*") |
| **Budgets are measured, not asserted** | Router ≤ **8,192 bytes** (half an atom's ceiling, because the router is the one file every page author loads); prose lines outside tables ≤ **96 columns** (the existing corpus's measured maximum is 93); `## Start here` ≤ 12 rows. Yield order when a budget bites: `Start here` rows merge first, then the scope paragraph shortens; the band table and the `## Index` never yield. | `_design.md` `## Density budget` and its yield order; `xtask/src/lint_constitution.rs:88,95` (the precedent ceilings and why bytes not lines) |
| **The affected gate widens, and that is recorded not fixed** | `standards/pages/` is unrecognised by `affected_packages` and absent from `INERT`, so a prose-only change here selects **every** package. Correct and slow. The fix is the `INERT` entry paired with the checker on the unconditional file-reading list, and the pair lands with the checker story. `xtask/src/affected.rs` is not touched here. | `xtask/src/affected.rs:209-224`, `:249-266` (`INERT`), `:116-125` (the unconditional list); architecture brief Note 1, CR-4 |
| **Nothing else moves** | No `xtask` source, no `standards/rust/` file, no `.kb/` file, no `.redkiln/templates/` file, no narrative page. `redkiln doctor` still reports exactly six `template-drift` advisories. | `CLAUDE.md`; architecture brief Note 8; project `project.md` Definition of done |

## Data and migrations

**N/A — no data and no migration.** This story adds two markdown artifacts and edits
one, and touches no schema, no serialised format, no wire envelope and no stored
state. Nothing in the diff is compiled: `standards/pages/` is deliberately not
registered with the doctest harness (`xtask/src/lib.rs:28`), and `docs/` reaches no
workspace package (`xtask/src/affected.rs:249-266`, `INERT`).

Two adjacent obligations that are *not* migrations but are the closest thing this
story has to one, both already stated above and both owned by the next slice rather
than by a script:

- **The generated region is a format contract, not stored data.** Nothing migrates it;
  the checker story's first `--write` must simply produce no diff against what this PR
  commits.
- **The `[PROVISIONAL — settles at …]` marker in `docs/README.md:25-29` is a dated
  claim with a named retirement**, not a TODO. `page-need-checker-mounted-in-the-gate`
  removes it in the commit that makes the sentence true; leaving it after that step
  merges is a defect in *that* story, and its spec inherits the obligation from here.

## Acceptance criteria

Nine criteria. Each is framed from a **reader's intent crossing the whole stack** — from
the repository index, through the router, to one rule file — because that traverse is the
product here; a router that is correct and unreachable satisfies nothing. The personas are
the initiative's: the **next page author** and the **non-author reviewer**
(`.bklg/docs-that-teach/page-need-discipline/_decomposition.md`, UX brief, S2), and the
**reader who arrives from the repository's index**, whose journey carries this
initiative's one measured defect
(`.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md:174-181`).

Every verification is runnable from the worktree root in `git bash`. Where a criterion's
**permanent** regression test lands in the next slice (it cannot land here — this PR
touches no `xtask/src/**`), the verification names both the mechanical check that runs
*now* and the forward test that inherits it; see "Clarifications resolved during spec".

| id | criterion | verification |
| --- | --- | --- |
| **AC-001** | **GIVEN** the next page author, about to write a narrative page and unwilling to read a corpus to start, **WHEN** they open `standards/pages/README.md` from a stated intent, **THEN** the page opens with `# Page standards`, one scope paragraph carrying the instruction to load one rule and never the tree, and the five-row band table (`00`, `10`, `20`, `30`, `40`) — and they name their task in one `## Start here` row, open exactly **one** rule file, and stop, without ever loading a second. | *Procedural (ledger).* The UX-012 walk, run once per band that exists at this merge, each recorded as `task → row → file opened`, and each ending at one file. *Mechanical, all captured verbatim:* `test -f standards/pages/README.md`; `rg -n '^# Page standards$' standards/pages/README.md` → one hit; `rg -c 'load one rule, never the tree' standards/pages/README.md` → ≥ 1; `awk '/^\| Band \|/,/^$/' standards/pages/README.md \| tail -n +3 \| wc -l` → `5`. Precedent for the shape: `standards/rust/README.md:1-21,45-59`. |
| **AC-002** | **GIVEN** the non-author reviewer, who has found a page rule and a `spec/SPECIFICATION.md` clause that appear to disagree and does **not** know which wins, **WHEN** they read `## Precedence` on the page they already landed on, **THEN** they learn the discipline sits at the **constitution-atom tier** of the existing five-tier chain, alongside `standards/rust/` and scoped to a different subject, adding no tier — **AND** they reach that answer without opening a second file, **AND** the chain's own statement was never edited to say it. | *Procedural / gate-state.* `git diff main -- standards/rust/README.md` → **empty**, output pasted into `_ledger.md` (the testing brief's own shape: nothing inside this diff can prove a *different* file was untouched except the diff itself). *Procedural (ledger):* a person who did not author the router answers "does a page rule beat a clause?" from the router's text alone, verdict recorded. *Mechanical:* `rg -n 'constitution.atom tier' standards/pages/README.md` → ≥ 1, and `rg -n 'SPECIFICATION clause' standards/pages/README.md` → ≥ 1. *Forward:* the `check_shape`-style structural-presence assertion in `page-need-checker-mounted-in-the-gate` (Testing brief, AC-002, "Static, secondary"). |
| **AC-003** | **GIVEN** a reader whose intent matches **no** `## Start here` row — the case a filter cannot serve — **WHEN** they keep reading the router top to bottom, **THEN** every rule in the tree is still listed, in a complete `## Index` between `<!-- BEGIN GENERATED -->` and `<!-- END GENERATED -->`, one row per rule atom with a resolving link, its trigger phrase and its rule ids; the index is **on the same page as the filter**, not behind a toggle, a fold or a second page, and it is complete whether or not the filter matched. | *Mechanical.* In-region row count equals the corpus: `sed -n '/BEGIN GENERATED/,/END GENERATED/p' standards/pages/README.md \| grep -c '^\|'` minus 2 equals `ls standards/pages/*.md \| grep -v README \| wc -l`. Every in-region link resolves: each `NN-slug.md` target passes `test -f standards/pages/<target>`. `rg -n 'show all|<details>|<summary>' standards/pages/README.md` → no matches. *Procedural (ledger):* name one rule no `## Start here` row points at and reach it by reading top to bottom. *Forward:* the link half of `check_router` reused verbatim (`xtask/src/lint_constitution.rs:343-356`; Testing brief, AC-011 "Static"). |
| **AC-004** | **GIVEN** the implementer of `page-need-checker-mounted-in-the-gate`, one slice later, who runs `--write` against this router for the first time, **WHEN** the generator emits the region from the corpus, **THEN** the output is **byte-identical** to what this PR committed and the run produces **no diff** — so the reader never sees a router whose index silently disagrees with its own tree, and the repair path (`--write`) is the only writer of that region. | *Mechanical, and this is the decisive one:* the header and separator lines are the precedent's verbatim, so `diff <(sed -n '/BEGIN GENERATED/,/END GENERATED/p' standards/rust/README.md \| sed -n '2,3p') <(sed -n '/BEGIN GENERATED/,/END GENERATED/p' standards/pages/README.md \| sed -n '2,3p')` → **empty**. *Procedural (ledger):* a row-by-row derivation table mapping each committed row to `generated_region`'s format at `xtask/src/lint_constitution.rs:400-420` — link form `[`NN-slug.md`](NN-slug.md)`, an empty trigger rendered `—`, every interior `\|` escaped, rows joined with `\n` and no blank line inside the markers. *Forward:* `page-need-checker-mounted-in-the-gate`'s first `--write` run, recorded in **that** story's ledger as producing no diff. |
| **AC-005** | **GIVEN** a slice-mate authoring a rule atom into this tree, **WHEN** they read `## The shape of a rule` before writing the atom head, **THEN** they learn the grammar (`# NN — Title`, `> **Load when:**`, `> **See also:**`, `---`, then `## RP-NN-N.` with the five fixed sections) **and** the constraint that `Load when:` occupies exactly **one source line**, with the reason attached — so the reader of the generated index gets a whole trigger phrase instead of the mid-phrase fragment the constitution's own index shows today. | *Mechanical.* `rg -n '^## The shape of a rule$' standards/pages/README.md` → one hit. Every atom in the tree conforms: for each `> **Load when:**` match, the following source line does not begin with `>` — `rg -n -A1 'Load when:' standards/pages/` reviewed and captured. Cross-check against the parser that motivates it (`xtask/src/lint_constitution.rs:247-255`) and the truncation it produces (`standards/rust/README.md:70-71`). *Consequence check:* no in-region trigger cell ends mid-phrase, confirmed by reading the region. |
| **AC-006** | **GIVEN** a contributor who assumes that because the repository has a gate, the gate is watching this tree, **WHEN** they read the router, **THEN** `## What checks this tree, and what does not` tells them, **at this merge**, that **no** gate step reads `standards/pages/` yet, names `page-need-checker-mounted-in-the-gate` as the step that adds one, and names what will still be unchecked afterwards — that a page *declares* a need is checkable, that it *answers* one is not, and band `40`'s reviewer walk is the instrument for the rest. | *Mechanical.* `rg -n '^## What checks this tree, and what does not$' standards/pages/README.md` → one hit; `rg -n 'page-need-checker-mounted-in-the-gate' standards/pages/README.md` → ≥ 1. *Procedural (ledger):* a reader who has not seen this spec reads only that section and states, in their own words, the three facts above; recorded. Bar and precedent: RS-81-1 (`standards/rust/81-checks-that-cannot-be-types.md:11`), section modelled on `standards/rust/README.md:114-127`. |
| **AC-007** | **GIVEN** the reader who arrives at the repository's own index — the persona whose measured defect is an answer that existed three documents from where they were standing — **WHEN** they scan `docs/README.md`'s "Looking for / It is at" table, **THEN** one row states a question they would actually ask and links to `standards/pages/README.md`, reachable in **one** navigation step, keyboard only; **AND** the gate-read-trees paragraph names the third tree carrying `[PROVISIONAL — settles at page-need-checker-mounted-in-the-gate]`, so a reader of that paragraph is told the truth at this merge rather than promised a check that does not exist. | *Mechanical.* `rg -n 'standards/pages' docs/README.md` → ≥ 2 hits, one inside the `:12-24` table and one inside the `:25-29` paragraph; the row's link target passes `test -f standards/pages/README.md`; `rg -n 'PROVISIONAL — settles at' docs/README.md` → one hit naming `page-need-checker-mounted-in-the-gate`. *Procedural (ledger):* the one-hop keyboard traverse from `docs/README.md` to the router, and the closing sentence about editing `xtask/src/` in the same change confirmed present and unweakened. *Forward:* the static "names three trees, not two" count assertion in `page-need-checker-mounted-in-the-gate` (Testing brief, AC-011). |
| **AC-008** | **GIVEN** any of the three readers, on a rendered markdown page **or** in a plain-text pager, **WHEN** the router first loads with nothing clicked, **THEN** every region is present as **composed presentation from this repository's own textual grammar** — H1, scope paragraph, band table, precedence blockquote, intent-keyed table, marker-delimited generated region, headed sections — in the binding top-to-bottom order, with the filter above the thing it filters; **AND** nothing is behind a fold, a `<details>`, a tab or an inactive panel, no bespoke navigation widget is added on top of what the renderer already gives, and no region's meaning is carried by colour, an icon or size. | *Mechanical.* Heading order captured: `rg -n '^#{1,2} ' standards/pages/README.md` prints exactly `# Page standards`, `## Precedence`, `## Start here`, `## Index`, `## The shape of a rule`, `## What checks this tree, and what does not`, in that order, with the band table between the scope paragraph and `## Precedence`. `rg -n '<details>|<summary>|role="tab"|\{\{#tab' standards/pages/` → no matches. `rg -n '<small>|<sub>|<sup>|<nav>|<img' standards/pages/` → no matches. *Procedural (ledger):* read the file through `less` (no renderer) and confirm each region is legible and that no meaning is lost; checked against `_design.md` `## Anti-patterns` 5, 6, 7, 9 and `## Transience policy` rows S2. |
| **AC-009** | **GIVEN** the page author who loads this router on **every** page they write — so its cost is paid on every task in the project — **WHEN** the router is measured rather than asserted, **THEN** it is ≤ **8,192 bytes**, every prose line outside a table is ≤ **96 columns**, `## Start here` carries ≤ **12** rows, every fence is tagged `text` or `markdown` (never `rust`, never untagged), and every `.md` link in the file resolves — and if a budget bites, the stated yield order is what gives: `Start here` rows merge, then the scope paragraph shortens; the band table and the `## Index` never yield. | *Mechanical, every number a command whose output is pasted into `_ledger.md`.* `wc -c < standards/pages/README.md` → ≤ `8192`. `awk '!/^\|/ && length > 96 {print FILENAME":"FNR": "length}' standards/pages/README.md` → no output. `awk '/^## Start here/,/^## Index/' standards/pages/README.md \| grep -c '^\|'` → ≤ `14` (12 rows plus header and separator). `rg -n '^```rust' standards/pages/` → no matches; `rg -n '^```$' standards/pages/` → no matches. Link resolution as AC-003. Calibration: the existing corpus's longest non-table line is 93 columns and `MAX_ATOM_BYTES` is 16,384 (`xtask/src/lint_constitution.rs:95`), so the router's ceiling is half an atom by decision, not by accident. |

**Project-AC coverage.** AC-001 (project) ← AC-001, AC-003, AC-004, AC-005, AC-009 (the
tree exists, its router routes to one rule, and its index cannot fall behind it).
AC-002 (project) ← AC-002. AC-011 (project, this story's half — *announced in the
repository's index*) ← AC-007, with AC-006 and AC-008 keeping the announcement honest and
reachable. The other half of project AC-011 — a governed page naming the discipline in
place — is `governed-page-cites-the-discipline`'s, per
`.bklg/docs-that-teach/page-need-discipline/_storymap.md`'s Coverage table.

## Interaction quality

RFC §6.7/D6. Every invariant that applies is an **AC row above**, not a bullet here: this
section says *which id carries which invariant* and how each is verified, so nothing in
this section is separately gateable and nothing gateable is only in this section. The
medium is text, so each invariant is restated in the form it takes with no DOM — that
translation is `_design.md`'s (`## Transience policy`, first paragraph), not invented here.

**State family.**

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **In place, not a context jump** — the answer is on the page the reader landed on | **AC-002** (the precedence rank answered in the router's own text, no second file), **AC-006** (the blind-spot statement in place, not in a separate note) | the procedural read in AC-002 and AC-006; `rg` for the rank sentence. The forbidden shape is "leaving the rank unstated and linking to the chain" — UX-005's named failure |
| **One intended context switch, and the reader chooses it** | **AC-001** (one `## Start here` row → one file), **AC-007** (one hop from `docs/README.md`) | the recorded walks. `_design.md` `## Transience policy` classes the index→atom links as the page's *only* opened-on-demand control |
| **Non-occlusion — a filter must not hide what it filters** | **AC-003** (complete index on the same page as the filter, present whether or not the filter matched), reinforced by **AC-008** (no toggle, no fold, no second page) | the row-count-equals-corpus check in AC-003 and the `<details>`/`show all` greps in AC-003 and AC-008. Anti-patterns 7 and 8 |
| **Preserved focus, scroll and selection** | **AC-008** | this repository's text medium has no authored disclosure and no authored script, so there is nothing that *can* move focus or scroll; AC-008's greps are what keep it that way. The invariant is discharged by **removing the mechanism**, which is the cheapest way to satisfy it (UX brief, Note 2, item 5) |
| **Reversibility** | **AC-004** (the generated region's only writer is `--write`, and the repair is named in the failure message the checker will print), **AC-008** (nothing to close, because nothing opens) | AC-004's derivation record plus the forward no-diff run. Reverting this PR leaves no cache, no generated artifact and no dirty file — `git status` clean after `git checkout -- standards/pages docs/README.md`, captured in the ledger |
| **Keyboard reachability** | **AC-007** (the one-hop traverse is run keyboard-only), **AC-003** (index links are plain markdown links), **AC-008** (no widget that could need a pointer) | the recorded traverse; the absence greps. UX-006's own test wording |

**Composition family** — taken from the signed-off
`.bklg/docs-that-teach/page-need-discipline/_design.md`, which is **binding** on this
story because it renders the `discipline-router` surface. This story does not re-decide
any of it.

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **Presentation exists at all** — every region is real composed presentation in the repo's own grammar (band table, blockquote, intent-keyed table, marker-delimited region, headed sections), not bare prose that happens to contain the words | **AC-008**, with **AC-001** and **AC-003** carrying the two tabular regions specifically | the heading-order capture in AC-008; the band-table row count in AC-001; the region checks in AC-003. `_design.md` `## Composition`, "S2 — the router, top to bottom" |
| **Composition and placement** — regions in the binding order, the filter always **above** the thing it filters | **AC-008** | AC-008's ordered heading capture, compared against `_design.md` `## Composition` items 1-7 and the same relative order at `standards/rust/README.md:1-21,23-29,45-59,61-97,99-112,114-127` |
| **Transience** — every router region is **persistent chrome**; the only opened-on-demand control is a link to an atom | **AC-008**, **AC-003** | `_design.md` `## Transience policy`, rows S2. The index is explicitly still persistent *because* it is the longest region — "length is not a reason" |
| **Density budget, with its real numbers** — 8,192 bytes; 96 columns outside tables; ≤ 12 `Start here` rows; and the stated yield order | **AC-009** | the four `wc`/`awk`/`grep` measurements in AC-009, each output pasted into the ledger. `_design.md` `## Density budget` and its per-surface yield order |
| **Hierarchy** — the band table primary; `## Start here` and `## Precedence` secondary; `## Index` and `## The shape of a rule` recessive — carried by **vertical order**, never by colour or size | **AC-008** | AC-008's order capture plus the `<small>`/`<sub>`/`<img>` absence grep. `_design.md` `## Hierarchy`, row S2: "the index is recessive by being last and generated, not by being smaller" |
| **Named anti-patterns refused** — 5 (a never-fold class invisible until clicked), 6 (any `<details>`/tab/accordion at all while `PERMITTED_FOLD_MECHANISMS` is empty), 7 (index behind a "show all rules" toggle), 8 (an index listing only some rules, or whose row count disagrees with the file count), 9 (a bespoke navigation widget), 13 (a `rust`-tagged fence anywhere in `standards/pages/`) | 5, 6, 9 → **AC-008**; 7, 8 → **AC-003**; 13 → **AC-009** | the greps named in each AC. Anti-pattern 8 is the one with a *counting* test, which is why AC-003's verification compares the in-region row count against `ls standards/pages/*.md` rather than eyeballing completeness |

**Why this matters here specifically.** An unstyled render satisfies every structural
assertion in this spec — a file containing the six headings in the right order and nothing
else would pass most of the greps above. The composition rows are what fail it: a band
table with fewer than five rows, an index whose row count disagrees with the tree, a
router over 8,192 bytes, or a heading order that puts `## Index` above `## Start here`
each fail a named measurement. There is no perceptual review to catch what those miss
(`design.capture` is absent from `.redkiln/config.yaml`), so `_design.md` plus these rows
are the only instrument.

## Error conditions

| id | condition | required handling |
| --- | --- | --- |
| **EC-001** | `standards/pages/` holds **no rule atom** when the index is authored — the dependency has not landed, or landed without its band-`00`/`10` files. | **Stop; do not author an empty region.** An index with a header, a separator and zero rows is the vacuity failure this project's own checker is built to `bail!` on (`xtask/src/lint_constitution.rs:176`), authored by hand one slice early. The dependency `need-vocabulary-and-declaration-form` is a hard blocker for the `## Index` region specifically; the rest of the router may be drafted, but the PR does not merge with an empty region. |
| **EC-002** | A `## Start here` row, or an `## Index` row, points at a rule file that does not exist yet (a band a slice-mate has not filed). | **A dangling link may not ship** — it is the `dangling-link` state `_design.md` names for this surface, and it fails the link half of `check_router` the moment that arrives. Either the row is dropped until its file exists, or the file lands in the same slice. Do not link a planned file. |
| **EC-003** | The committed generated region disagrees with `generated_region`'s format — a different header word, a `\|`-unescaped trigger phrase, a `-------` separator instead of `---`, a link written as a bare path, or a blank line inside the markers. | The checker story's first `--write` then rewrites the region and produces a diff nobody intended, and the reviewer of *that* PR cannot tell an intentional change from a formatting drift. AC-004's `diff` of the first two in-region lines against `standards/rust/README.md`'s is the guard; run it before opening the PR, not after. |
| **EC-004** | `docs/README.md:25-29` names the third tree **without** the `[PROVISIONAL — settles at …]` marker, or with a marker naming no story. | A false sentence in the file whose subject is that pins are honest, in a project whose whole thesis is that a check with undocumented limits reads as a guarantee. The marker is mandatory at this merge and its bracket must name `page-need-checker-mounted-in-the-gate` verbatim, because that is the string the removing commit greps for. |
| **EC-005** | Someone proposes renaming `standards/pages` during or after this story. | It is not a rename: `_design.md` `## Sign-off` condition 2 records that the name is depended on **by value** in three `xtask/src/main.rs` edits, one `INERT` entry, and two `affected` tests, none of which exist yet. Changing it here means changing `_design.md`, which is signed off — so it is a design-stage decision reopened by a human, not an implementer's choice. |
| **EC-006** | `standards/rust/README.md` is opened for writing — to add a cross-link, a "see also", or a mention of the sibling tree. | **AC-002 fails outright.** The rank is stated in the *new* router and cited from it; the chain's own file is read-only for this story. The reciprocal link, if it is ever wanted, is a decision for a later story with its own ADR-shaped argument, because it makes the constitution's file depend on this tree's existence. |
| **EC-007** | `cargo xtask affected --base main` selects every package on this prose-only PR, and it is read as a failure or "fixed" by adding `standards/pages` to `INERT`. | **Expected, and correct.** Record the widening as observed; do not touch `xtask/src/affected.rs`. Adding the `INERT` entry alone makes a prose-only PR read **nothing**, which is the half-mount `_storymap.md` names; the entry lands paired with the checker's unconditional-list entry in the next slice (architecture brief, Note 1, CR-4). |
| **EC-008** | The router grows past 8,192 bytes while being written. | Apply the stated yield order, not judgement: merge `## Start here` rows, then shorten the scope paragraph. The band table and the `## Index` never yield. If it still does not fit, the diagnosis is that **the tree needs a band**, not that the router needs trimming (`_design.md` `## Density budget`, S2). |

## Non-functional

| id | requirement | why, and how it is observed |
| --- | --- | --- |
| **NF-001** | **No dependency changes.** `xtask/Cargo.toml` and every workspace manifest are untouched; nothing is added for measurement (the budgets are `wc`/`awk`/`rg`, all present). | `git diff main -- '**/Cargo.toml' Cargo.lock` → empty. The testing brief is explicit that no `tempfile` (or any) dependency is added for this project, and this story adds no Rust at all. |
| **NF-002** | **LF line endings and UTF-8, no BOM**, on both files. | This is a Windows checkout. `check_router` compares the region for string equality after `.trim()` on the whole block only — a CRLF interior line is *not* equal to an LF one, so a CRLF region would make the checker's first `--write` produce a whole-region diff (EC-003 by another route). Observe with `file standards/pages/README.md` and `git diff --check`. |
| **NF-003** | **Non-ASCII characters match the corpus's existing set** — the em dash `—`, the middle dot `·`, and the box-drawing characters only where the corpus already uses them. No smart quotes, no non-breaking spaces. | Two files that render the same idiom two ways is a second grammar. `rg -n '[""'"'"''…\xa0]' standards/pages/README.md docs/README.md` → no matches beyond what `main` already carries. |
| **NF-004** | **Every budget number in this spec is reproducible from a named command**, and the command's output — not a claim about it — is what enters the ledger. | `_design.md` `## Sign-off` condition 4: no pixel budget is claimed, only source columns and bytes, "which are measurable today". A ledger row citing "checked, within budget" without the number is not evidence. |
| **NF-005** | **Gate wall-clock is allowed to regress on this PR, and only by the affected-gate widening.** No step is added, no step is slowed. | `cargo xtask affected --base main` compiles and tests the whole workspace (EC-007); `cargo xtask ci --fast` is unchanged in shape because no new step exists yet. Record both durations so the checker story's PR has a baseline to compare its `INERT` pair against. |
| **NF-006** | **`redkiln doctor` still reports exactly six `template-drift` advisories.** | `.redkiln/templates/**` is out of scope (PR boundary), and the `backlog` CI job asserts the set is exactly those six (`CLAUDE.md`). A seventh or a fifth means a template moved in this diff. |
| **NF-007** | **`cargo xtask spec-trace` stays green and `spec/SPECIFICATION.md` is unmodified.** | Nothing here writes, restates or renumbers a clause; the discipline requires pages to *cite*. `git diff main -- spec/` → empty. |

## Implementation notes (non-prescriptive)

Not instructions — the shape the front half's decisions imply, offered so the implementer
spends their judgement on the prose rather than on rediscovering the constraints.

- **Read the precedent as a whole file before writing a line of the router.** Open
  `standards/rust/README.md` end to end once. It is the only working proof in this
  repository that a reader can load one of twenty-seven atoms without loading the corpus,
  and almost every decision this story has to make has already been made there in a form
  the gate holds up. Copy the *shape*; do not copy the Rust content, and do not copy the
  mid-phrase index rows (AC-005 exists because those are a defect the precedent tolerates
  and this tree must not).
- **Author the region last, from the files that exist.** The `## Index` is derived. Write
  the atoms' heads (the dependency's, already merged), then read their first
  `> **Load when:**` line and their `## RP-NN-N.` ids, then build the rows. Deriving the
  region by hand from the corpus is the same operation the generator performs, which is
  what makes AC-004's no-diff obligation reachable rather than lucky.
- **A useful order of work:** scope paragraph and band table → `## Precedence` →
  `## What checks this tree, and what does not` (write it *early*, while the blind spots
  are fresh and before the temptation to soften them) → `## The shape of a rule` →
  `## Start here` → the `## Index` → then measure, then `docs/README.md`.
- **The `docs/README.md` row is written in that table's voice, not this spec's.** Every
  existing row is a phrase a contributor would actually think, against an inline-code
  label and a relative link. Match the surrounding rows' form exactly; a row that reads
  like a project deliverable is a row a reader skips.
- **The gate-read paragraph is a rewrite of two sentences, not an appended one.** It
  currently opens "Two of those are read by the gate rather than only by people" — that
  count is the thing that changes, and the closing sentence about editing `xtask/src/` in
  the same change is the reasoning, which stays. Rewrite the referent, never the reasoning
  (`.kb/governance/rewrite-the-referent-never-the-reasoning.md`).
- **Two spellings are constants, typed by hand exactly once each and then copied:**
  `standards/pages` and `standards/pages/README.md`. A typo in either is not a typo — it
  is a pin that will not match the `const` the checker story writes.
- **Where a fence is needed, tag it `text` or `markdown`.** The router will want at least
  one, to show the atom head grammar in `## The shape of a rule`. An untagged fence is
  rejected for the same reason a `rust` one is: it leaves the door open for a future
  decision to register this tree with the doctest harness to be undermined retroactively.
- **Do not build the shared helper you will want.** After the second time you write a
  path relative to `standards/pages`, the instinct is to factor something out with
  `lint_constitution`. RS-81-3 scopes a scanner to the directory whose behaviour it
  constrains (architecture brief, Note 6), and in any case this PR contains no Rust.
- **Run the measurements before opening the PR, and paste their output into the ledger as
  you go.** Nine ACs, roughly fifteen commands; collecting them at the end is how one gets
  paraphrased.

## Tests and CI (merge gate)

Tiers as the project's testing brief defines them
(`.bklg/docs-that-teach/page-need-discipline/_decomposition.md`, Testing brief,
"Acceptance Criteria"): **static** (`#[cfg(test)]`), **gate-integration**,
**end-to-end/fixture**, **procedural (ledger-recorded)**. This story is deliberately
weighted to the last two — see the clarification below on why the static tier is empty
here and where it lands.

| tier | command / path | proves |
| --- | --- | --- |
| **static** | *(none in this PR — by construction)* | The permanent regression assertions this story's ACs imply are named and inherited by `page-need-checker-mounted-in-the-gate`: the router's precedence-sentence presence (AC-002), the link-resolution half of `check_router` (AC-003), and the `docs/README.md` row-plus-three-trees count (AC-007). This PR touches no `xtask/src/**`, so it can carry no `#[test]`; the obligation is recorded here and in the next story's spec rather than dropped. |
| **gate-integration** | `cargo xtask lints` | The file-reading lint family is green and **unchanged** — `lint-constitution` still passes over `standards/rust/`, proving this story did not perturb the tree it cites (`verify.reachability_static`, project.md DoD). |
| **gate-integration** | `cargo xtask spec-trace` | `spec/SPECIFICATION.md`'s markers and citations still resolve; NF-007. Load-bearing here precisely because this project's deliverable is files the compiler never reads. |
| **gate-integration** | `cargo xtask ci --fast` | The bar a non-terminal project is held to (`.redkiln/config.yaml`, `verify.integration_scoped`; project.md DoD). Green at merge. |
| **gate-integration** | `cargo xtask ci` | The merge gate of record (`CLAUDE.md`, "Commands"). Run before the story is called done. |
| **gate-integration** | `cargo xtask affected --base main` | The story grain (`verify.affected_gate`). Expected to widen to the whole workspace and to be **green**; the widening is recorded as observed, per EC-007 (`xtask/src/affected.rs:209-224`). |
| **gate-state** | `git diff main -- standards/rust/README.md` | **AC-002.** Empty. The testing brief's own reasoning: nothing inside this diff can prove a different file was never touched except the diff. |
| **gate-state** | `git diff main -- xtask spec .kb .redkiln/templates '**/Cargo.toml'` | The PR boundary and NF-001/NF-006/NF-007 in one command. Empty. |
| **mechanical (procedural, captured)** | the fifteen `wc` / `awk` / `rg` / `sed` / `diff` / `test -f` invocations named in the acceptance table | **AC-001, AC-003, AC-004, AC-005, AC-006, AC-007, AC-008, AC-009.** Reproducible, and each one's *output* — never a claim about it — enters `_ledger.md` (NF-004). |
| **procedural (ledger-recorded)** | the UX-012 walk (AC-001); the non-author precedence read (AC-002); the unpointed-rule traverse (AC-003); the region derivation table (AC-004); the blind-spot read-back (AC-006); the one-hop keyboard traverse (AC-007); the plain-pager read (AC-008) | The judgements no command carries. `.redkiln/config.yaml`'s `require_ledger: true` already treats a recorded manual verification as first-class proof, which is why the testing brief made this a named tier rather than an excuse. |
| **procedural (reversibility)** | `git checkout -- standards/pages docs/README.md && git status` | Nothing this PR adds leaves residue: no cache, no generated artifact, no dirty file. The reversibility invariant, discharged the same way UX-008 specifies for the checker's own failure. |
| **end-to-end/fixture** | *(none — deferred, by design)* | Breaking a page and watching the gate name the file and line is `declaration-check-seen-to-fail`'s whole story, and it cannot run before a gate step exists to be watched failing. Nothing in this story pretends to that evidence. |

**Merge gate, one line.** `cargo xtask ci` green; `cargo xtask affected --base main` green
(widened); both `git diff` boundary checks empty; every mechanical measurement captured;
every procedural walk recorded in `_ledger.md` with cited evidence.

## Risks and coupling (PR-scoped)

| Risk | Coupling it runs through | Mitigation in this PR |
| --- | --- | --- |
| **The generated region's shape is decided here and implemented one slice later.** If the two disagree, the checker's first `--write` produces a surprise diff and the reviewer of *that* PR cannot separate drift from intent. | `page-need-checker-mounted-in-the-gate` ← this story, by **value** (bytes, not an interface) | AC-004 makes the header and separator a `diff` against the precedent's own region — a mechanical equality, not a description. The row format is pinned to `xtask/src/lint_constitution.rs:400-420` by line, and the derivation table is a ledger artifact the next story can read. |
| **`standards/pages` is a name two future files depend on by value, and it is being fixed by a prose PR.** | `_design.md` pins it; three `main.rs` edits, one `INERT` entry and two `affected` tests will consume it | EC-005: renaming is a signed-off design decision reopened by a human, not an implementer's choice. Typed once, copied thereafter (Implementation notes). |
| **The `[PROVISIONAL — settles at …]` marker outlives the thing it hedges.** A marker nobody removes is the decorative-documentation failure this initiative exists to refuse, in this initiative's own file. | `docs/README.md:25-29` ← `page-need-checker-mounted-in-the-gate` | The bracket names the removing story verbatim, so it is greppable; the obligation is stated in this spec's Data and migrations section as inherited by that story, not merely hoped for. |
| **The affected gate widens to the whole workspace and someone "fixes" it.** | `xtask/src/affected.rs:209-224`, `:249-266`, `:116-125` | EC-007 states the correct treatment as a *pair* of edits, names why the `INERT` half alone is worse than the widening, and puts `xtask/src/**` out of scope in the PR boundary. The gate-state `git diff` command makes a stray edit a merge blocker. |
| **The dependency's atoms arrive thin, and the index is authored over one or two files.** A five-band table above a two-row index reads as an unfinished tree. | `need-vocabulary-and-declaration-form` → this story | EC-001 forbids an empty region. A *sparse* one is acceptable and expected — the band table is the namespace, and `_design.md` fixes five bands whose owners are named in the Integration contract's band table, so a reader can see which are filled and which are coming. Do not link an unwritten file (EC-002). |
| **The router drifts toward being a corpus.** Every reviewer of a router wants to add one more paragraph, and this file is loaded on every page-writing task in the project. | `_design.md` `## Density budget`; UX-012 | AC-009's 8,192-byte ceiling is measured, not asserted, and the yield order is stated (EC-008) so the answer to overflow is never "trim the index". |
| **Slice-mates author atoms whose `Load when:` wraps**, and the generated index then carries truncated fragments — the precedent's own defect, reproduced. | this story's `## The shape of a rule` → `fold-line-rule`, `reviewer-and-citation-procedures` | AC-005 puts the one-source-line constraint in the router *and* verifies it across the tree at this merge, so the rule is stated before the atoms that must obey it are written — which is the reason this story precedes them in merge order. |
| **Someone adds a reciprocal link from `standards/rust/README.md`.** It is a small, plausible, friendly edit that fails AC-002. | `standards/rust/README.md:23-29` | EC-006 names it as the forbidden shape and the gate-state `git diff` catches it. |

## Dependencies

**Blocks on** (must be merged before this story's `## Index` can be authored):

- **`need-vocabulary-and-declaration-form`** — the closed need set landed once, as the
  `NEEDS` const in the new `xtask/src/` module *and* as the band-`00`/`10` rule atoms.
  This story's index is generated from those atoms, so without them the region is either
  empty (EC-001) or a set of dangling links (EC-002). This is the project's only
  foundation story and it is consumed here first
  (`.bklg/docs-that-teach/page-need-discipline/_storymap.md`, "Why the slices fall here",
  second bullet).

**Unlocks** (each `depends_on` this story, per the story map):

- **`fold-line-rule`** — band `20`. Needs the band namespace, the atom grammar
  (`## The shape of a rule`, including AC-005's one-source-line constraint) and a router
  to be indexed in.
- **`reviewer-and-citation-procedures`** — bands `30` and `40`, for the same three
  reasons. Band `30` is the one band `_design.md` names no file for; this story's band
  table assigns it, so that story inherits an owner rather than a gap.
- **`page-need-checker-mounted-in-the-gate`** — needs a **non-empty** rules tree (or its
  own vacuity guard fails it correctly on the first run) and inherits three obligations
  from this PR: the generated region's byte shape (AC-004), the `docs/README.md` marker's
  removal, and the `xtask/src/affected.rs` `INERT`-plus-unconditional-list pair.
- **`governed-page-cites-the-discipline`** — needs a router at a stable path to link to,
  and completes the other half of project AC-011.

**Slice-mates** (implemented in one context, mounted as one surface):
`need-vocabulary-and-declaration-form`, `fold-line-rule`,
`reviewer-and-citation-procedures` — the `discipline-on-disk` milestone.

## Anchors (progressive disclosure)

Load-bearing depth, deferred rather than pasted. Each row says **why** it is load-bearing
and **when** to open it, and is bound to the AC it serves. Every path was confirmed to
exist in this worktree.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `.bklg/docs-that-teach/page-need-discipline/_design.md` | **Binding, signed off.** Carries the pinned-constant table, `### S2 — discipline-router`, `### S2 — DR-02`, `## Composition`'s seven ordered regions, the `## Transience policy` S2 rows, the `## Density budget` with its yield order, `## Hierarchy`, and the sixteen `## Anti-patterns`. This story implements it and may not contradict it. | **Before writing the first line of the router**, and again before each measurement. | AC-008, AC-009, AC-002, AC-003 |
| `standards/rust/README.md` | The only working proof at twenty-seven-atom scale of a filter that does not hide what it filters: `:1-8` scope, `:10-21` band table, `:23-29` the precedence chain to cite and never edit, `:45-59` the filter, `:61-97` the generated whole, `:99-112` shape-of-an-atom plus the `[PROVISIONAL — settles at …]` idiom at `:109-111`, `:114-127` the blind-spot section to model. | **First**, read end to end, before drafting. Re-open at `:109-111` when writing the `docs/README.md` marker and at `:114-127` when writing the blind-spot section. | AC-001, AC-002, AC-006 |
| `xtask/src/lint_constitution.rs` | The generated-region contract in code: markers parsed at `:388-397`, equality and the `--write` repair at `:358-383`, the exact row bytes at `:400-420`, the link check at `:343-356`, the `Load when` first-line-only parser at `:247-255`, the ceilings at `:88,95`, the vacuity `bail!` at `:176`. This story authors bytes another story's code will compare for equality. | **Immediately before authoring the `## Index` region**, and again before running AC-004's `diff`. | AC-003, AC-004, AC-005, AC-009 |
| `standards/rust/00-prime-directives.md` | `:3-9` is the atom head grammar the index rows are built from — the `> **Load when:**` / `> **See also:**` pair — and the worked five-section rule shape the router's `## The shape of a rule` describes. | **When writing `## The shape of a rule`.** | AC-005 |
| `standards/rust/81-checks-that-cannot-be-types.md` | `:11` is RS-81-1: a check's blind spot must be stated in its own documentation, because "a check whose limits are undocumented is read as a guarantee". This story applies it one level up, to a tree nothing checks yet. | **Before writing `## What checks this tree, and what does not`** — and write that section early, per the Implementation notes. | AC-006 |
| `docs/README.md` | The mount point, and the file whose two regions this PR edits: the `:12-24` "Looking for / It is at" table (voice and link form to match) and the `:25-29` gate-read paragraph (the count that changes, and the closing sentence that must not). | **When making the two edits**, after the router exists and its path is final. | AC-007 |
| `.kb/governance/rewrite-the-referent-never-the-reasoning.md` | The governance test for touching text that already discharges something. `docs/README.md:25-29` discharges the pin-by-path discipline; this PR rewrites its referent (two trees → three, marked) and must leave its reasoning intact. | **Before editing `docs/README.md:25-29`**, if there is any temptation to reword the closing sentence. | AC-007 |
| `.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md` | `:174-181` is this initiative's one **measured** defect: the `E0034` explanation that existed in three contributor-facing documents, none of them the file the reader was looking at. A rules tree that exists and is unreachable is that defect wearing this story's name — which is why the announcement is in this slice. | **Before deciding the `docs/README.md` row's wording**, to write it in a reader's voice rather than a deliverable's. | AC-007 |
| `.bklg/docs-that-teach/page-need-discipline/_decomposition.md` | The three briefs. Architecture: AC-001/AC-002/AC-011 and Notes 1 (CR-4), 4 (the fence inversion), 5 (the generated region), 6 (no shared abstraction), 7 (what the blind-spot section carries), 9 (what was left open). UX: UX-004, UX-005, UX-012, Note 1's primitive table, Note 2's five invariants. Testing: the five tiers and the per-AC test mix. | **Architecture Note 5 before the index; UX Note 1 before composing any region; the Testing brief before filling the ledger.** | AC-001, AC-003, AC-005, AC-009 |
| `.bklg/docs-that-teach/page-need-discipline/design/mock.html` | The built contact sheet: the `discipline-router` frames including `generated-region-stale`, `dangling-link` and `narrow-80-column`, with density chips computed from each specimen's own bytes. It shows what "populated" and "over budget" actually look like rather than describing them. | **When a budget measurement comes out close to a ceiling**, and when deciding what `dangling-link` must never ship as. | AC-008, AC-009, AC-003 |
| `xtask/src/affected.rs` | `:209-224` is the arm that widens an unrecognised prose path to the whole workspace, `:249-266` is `INERT`, `:116-125` is the unconditional file-reading list. Reading all three is what makes the widening legible as correct-and-slow rather than as a bug to fix in this PR. | **Before running `cargo xtask affected --base main`** and recording its result. | AC-001 |
| `.bklg/docs-that-teach/page-need-discipline/project.md` | `:206-211` is project AC-001 and AC-002 in the charter's own words, and `:239-241` is AC-011; `:246-259` is the Definition of done this story's gate commands come from. The authority for what "traces to" means here. | **When filling the ledger**, to confirm each project AC is discharged by a story AC and not merely mentioned. | AC-002, AC-007 |
| `.bklg/docs-that-teach/page-need-discipline/_storymap.md` | The slice boundaries and the Coverage table: which half of project AC-011 is this story's and which is `governed-page-cites-the-discipline`'s, and why the `docs/README.md` announcement is in this slice rather than a later one. | **If the announcement's scope is questioned**, or before deciding whether a rule belongs to this story or a slice-mate. | AC-007, AC-001 |

## Clarifications resolved during spec

1. **The AC set is exactly the nine the front half enumerated** — AC-001 through AC-009,
   none added, none dropped. The ledger carries nine rows.
2. **The static test tier is empty in this PR, and that is a decision rather than an
   omission.** The testing brief specifies static assertions for AC-002 ("a
   `check_shape`-style test asserts the router's own text contains a stated precedence
   sentence") and for AC-011 (`docs/README.md`'s table carries a row, and the paragraph
   names three trees). Both are `#[cfg(test)]` tests inside a module that does not exist
   until `page-need-checker-mounted-in-the-gate`, and this story's PR boundary forbids
   touching `xtask/src/**` — with `git diff main -- xtask` empty as a merge condition. The
   resolution: each such assertion is named in the Tests and CI table's `static` row as an
   obligation the checker story inherits, and its *present-tense* verification is the
   mechanical `rg`/`sed`/`diff` check named in the AC. This is the "procedural
   (ledger-recorded)" tier the testing brief created for exactly this shape, plus a stated
   forward obligation, rather than an unverified AC.
3. **The generated region's header and separator are the precedent's verbatim** —
   `| Atom | Load when | Rules |` and `|---|---|---|`, exactly as
   `xtask/src/lint_constitution.rs:401-404` emits them. This was open: the front half
   fixed the *column order and cell contents* but not the header word. Deciding it as a
   copy makes AC-004 mechanically checkable by `diff` against the live constitution
   region, and makes the checker story's `generated_region` analogue a copy rather than a
   variant. "Atom" reads correctly here because `_design.md` names the surface `rule-atom`.
4. **Band `30` is assigned to `reviewer-and-citation-procedures`.** `_design.md` fixes
   five bands and names a file for four of them; band `30` (citations) had no owner. The
   front half assigned it and this half keeps the assignment, with the rejected
   alternative recorded: folding citations into band `20` lost because the fold line and
   the citation rule are applied by different readers at different moments.
5. **Reversibility is discharged by absence, not by a mechanism.** UX brief Note 2's third
   and fourth invariants (preserved focus/scroll, reversibility) assume something that
   opens. This story authors no disclosure and no script, so the invariant is satisfied by
   AC-008's absence greps plus the `git checkout --` residue check — stated explicitly so
   a later reader does not read the empty rows as an oversight.
6. **The one-hop keyboard traverse (AC-007) is verified against the markdown source and a
   rendered page if one is available, and against the source alone if not.** No renderer
   is wired up in this worktree, and `_design.md` `## Sign-off` condition 4 forbids
   claiming a pixel budget. Plain markdown links are keyboard reachable in every renderer
   in the ecosystem and in a pager; the recorded evidence says which medium was used.
7. **No ADR is written, and none may be.** Grounding confirmed no Accepted decision atom
   under `.kb/decisions/` governs documentation trees, gate structure or narrative
   conventions; the binding authority here is sub-ADR (`CLAUDE.md`,
   `standards/rust/README.md:23-29`, `docs/README.md:25-29`). Writing one would itself
   breach the initiative's non-goal against extending the precedence chain — and it is not
   this story's to write in any case (`RUNBOOK.md` carries no phase for this initiative).
8. **`cargo xtask affected --base main` widening to the whole workspace is recorded as a
   green observation, not a regression.** Stated three times on purpose — Context pack,
   EC-007, NF-005 — because it is the single most likely thing for a reviewer or a future
   implementer to "fix" in the way that makes a prose-only PR read nothing.
