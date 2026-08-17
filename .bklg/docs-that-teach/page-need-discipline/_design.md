---
item: HS-P0021
stage: design
created: 2026-08-17
updated: 2026-08-17
---

# Interaction design — Page-Need Discipline

The resolved **form** of this project's surfaces, binding on the implementer. Behaviour lives in the
story specs; what is decided here is arrangement, hierarchy, density, transience and the three open
design tensions this project owns — **DT-2**, **DT-3**, **DT-8**.

**Two things about the medium, stated once so no section below re-litigates them.**

1. **There is no DOM, no CSS layer, no token file and no component library.** `rg` for
   `*token*.css`, `*theme*.css`, `tailwind*`, `*.tokens.*` and `*/ui/src/**` returns nothing, and
   `.redkiln/config.yaml:75-81` says so on purpose: `design.capture` is absent because "there is no
   app to screenshot". The perceptual design review is therefore a **confirmed skip**, which makes
   this file the *only* record of these choices (project.md, AC-003). Every requirement below is
   written to be checkable in text, at a terminal, or against a screenshot of a rendered markdown
   page — never by perceiving a style.
2. **The design system this project composes is textual and it already exists.** It is the
   constitution's document grammar (`standards/rust/README.md`, `standards/rust/00-prime-directives.md`)
   and the gate's diagnostic grammar (`xtask/src/lint_constitution.rs`), both *enforced* by
   `check_router` / `check_shape` / `check_rules`, which is what makes them primitives rather than
   habits. Where a section below names a primitive it names the `path:line` it was verified at.
   Nothing is invented; where the repo has a gap, the gap is named as one.

## Template sections that do not apply — public API surface

`.redkiln/templates/_design.md` is the bundled design stage repurposed for a library, and asks for
the public API surface. **This project changes no public API.** Its only Rust lives in the `xtask`
bin crate, which carries `publish = false` and is never a dependency of anything; its items are
`pub(crate)` by construction (the precedent it copies, `lint_constitution::run`, is
`pub(crate)` at `xtask/src/lint_constitution.rs:170`). So:

- **Items / Signatures / Placement / Visibility / What it costs a caller / What a user meets first /
  The doctest — N/A, no public surface.** The consts and functions this project adds are named in
  `_decomposition.md`'s architecture brief (CR-1 through CR-4) and are the implementer's to write; a
  `pub` item nobody designed cannot reach a published crate from here because no crate here is
  published.
- What replaces them is everything below: the project's four user-facing surfaces are textual, and
  the template's own warning transfers exactly — every other check in this repository is satisfied by
  a discipline that is correct and unreadable.

## Surfaces

Machine-read by the capture harness. **No capture will run** (`design.capture` absent), so this
block is an addressing manifest for a human and for the design review, not a screenshot queue. There
is no DOM, so `selector` isolates each surface by the only stable addresses this repo has: a path
glob, a line-anchored regex, a heading anchor, or a shell invocation's output stream.

```yaml
- id: page-need-declaration
  route: "<PAGE_DIR>/**/*.md — HS-P0020's pinned narrative tree; PAGE_DIR is that project's const and is unpinned in this worktree"
  selector: "the first blockquote line of the file, matching /^> \\*\\*Answers:\\*\\* `[a-z-]+` — .+\\?$/"
  states: [declared-valid, missing, two-declarations, unenumerated-token, long-question, plain-text-pager, greyscale-print]
- id: discipline-router
  route: "standards/pages/README.md"
  selector: "whole file; regions `## Precedence`, `## Start here`, and the block between `<!-- BEGIN GENERATED -->` and `<!-- END GENERATED -->`"
  states: [populated, generated-region-stale, dangling-link, narrow-80-column]
- id: rule-atom
  route: "standards/pages/NN-*.md (NN in {00,10,20,30,40})"
  selector: "`# NN — <title>` head through the final `## RP-NN-N` rule heading"
  states: [populated, over-byte-ceiling, over-rule-ceiling, missing-section, rust-tagged-fence]
- id: lint-terminal-output
  route: "$ cargo run --locked --quiet -p xtask -- lint-pages   # also reached by `cargo xtask ci` and `cargo xtask lints`"
  selector: "the step's stdout/stderr; problem lines match /^  \\S+:\\d+ — /, the success line /^  \\d+ pages?, /"
  states: [green, single-problem, many-problems, vacuous-tree, missing-tree, write-repair-offered]
- id: reviewer-procedure
  route: "standards/pages/40-reviewing-a-page.md#the-walk"
  selector: "the ordered list under `## RP-40-1`, plus the verdict table that closes it"
  states: [verdict-pass, verdict-fail-two-needs, verdict-fail-unstated, indeterminate]
```

**`standards/pages/` is pinned here.** The architecture brief (Note 9) left the directory name free
and warned it becomes a `const` two files depend on by value. This design fixes it, because a
surface with no address cannot be reviewed:

| Constant | Value | Where it is depended on |
| --- | --- | --- |
| `RULE_DIR` | `standards/pages` | the new checker module; `INERT` in `xtask/src/affected.rs:249-260` |
| `ROUTER` | `standards/pages/README.md` | the new checker's `check_router` analogue |
| task name | `lint-pages` | `main.rs` dispatch (`:689-700`), `print_help` (`:718`), `lint_steps` (`:799-808`) |
| step name | `every page declares one need` | `REQUIRED` (`main.rs:105`); `steps_named` panics on a mismatch (`:816-826`) |

Rejected homes, with the cost of each (AC-001, DR-01): **new constitution atoms under
`standards/rust/`** — the band table is a Rust namespace and `check_harness`
(`lint_constitution.rs:423-458`) would demand each new atom be registered in the doctest harness,
which is wrong for prose rules; **a `.kb/` corpus authored now** — atoms may only be authored through
`/redkiln:kb-ingest`, which runs at closeout, four projects too late, and the hand-authoring attempt
was reverted at `0269720`; **a subtree of `docs/`** — `docs/` is user documentation only
(`docs/README.md:1-6`) and is already in `INERT` (`affected.rs:250`), so a rules edit there would
select no package *and* nothing would pin it; **nothing at all** — the rule is then retrofitted to
pages already written, which is how it becomes decorative.

The rule-id prefix is **`RP-NN-N`**. `RS-` is the constitution's; `PS-` is a live
`spec/SPECIFICATION.md` clause family (verified: the specification's clause prefixes are `CF`, `ES`,
`PS`, `SY`, `VT`, `WF`), so a `PS-` rule id would read as a normative clause it is not.

## Pattern decision

### S1 — `page-need-declaration`: a first-line blockquote label (DR-05)

**Chosen.** Immediately after the page's `# Title`, separated by one blank line and followed by one
blank line, a single-line blockquote:

```text
# Append conditions

> **Answers:** `explanation` — Why does a write need to re-read what it decided on?

The first paragraph starts here.
```

The token is one member of the closed `NEEDS` set, in backticks. The clause after ` — ` is a
question in the reader's voice, ending in `?`.

**Pattern citation.** This is the repo's own `> **Load when:** …` line
(`standards/rust/00-prime-directives.md:3-6`), parsed by `load_when`
(`xtask/src/lint_constitution.rs:247-257`) and required by `check_shape` (`:486-489`). It is the only
line-level, human-visible, machine-read metadata convention in this workspace that is already
enforced. The dossier's supporting rule is the anti-pattern at
`interaction-patterns.md:436-443` — do not add an affordance the medium already renders; a blockquote
is rendered by every CommonMark renderer in the ecosystem for free.

**Rejected.**

- **YAML front matter.** The charter's top risk (project.md, Risks row 1): mdBook has no native
  per-page front matter, and a markdown file `include_str!`'d into rustdoc renders the block as
  literal body text or as a horizontal rule. It also fails UX-001 in the plain-text case — front
  matter reads as machine chrome, not as the first thing the page tells you.
- **An HTML comment (`<!-- need: explanation -->`).** Machine-readable and *invisible to the reader*,
  which fails UX-001 outright. It is the sidecar failure wearing an inline costume, and the UX
  brief's Note 4 explicitly steers the declaration to the inline half of `check_fences`' precedent
  (`lint_constitution.rs:637-643`) and not the sidecar half.
- **A filename or directory convention (`explanation/append-conditions.md`).** Invisible on the
  rendered page, and it forces the taxonomy into the tree structure — the eighth anti-pattern
  (`interaction-patterns.md`, "Forcing content into a fixed number of top-level structural
  categories"), the one Diátaxis's own maintainer calls "horrible".
- **A sidecar manifest (`pages.toml`).** Two artefacts that must agree; `check_summaries`
  (`lint_constitution.rs:466-470`) already names the general shape — "a summary that does not point
  at the corpus is a second copy of it, and one of the two will be stale."
- **A badge, an icon, or a coloured admonition block.** Fails UX-002 (colour never alone) and
  fails the greyscale/plain-text-pager test.

**Hosting assumption this rests on, stated so a change visibly invalidates it (DR-05, project.md
Risks row 1).** The form assumes only that the medium renders **CommonMark blockquotes as visible
body text in document order**. It assumes *nothing* about front matter, about directory-derived
navigation, or about a renderer's own metadata layer. HS-P0020's hosting decision is open in this
worktree (its deployment brief states Option A `docs/`-repurposed and Option B a separate rendered
tree, and defers the choice to its own `_design.md`). **Both options satisfy the assumption**; a
third option that does not — a renderer that strips or relocates leading blockquotes, or one that
requires front matter — invalidates this decision and re-opens DR-05, and the checker's module docs
must say so in those words.

**Failure mode of the chosen pattern, and its mitigation.** A one-line label is trivially *gameable*:
an author declares `explanation` and writes a how-to underneath, and no parser sees it. That is not a
defect this pattern can fix — it is the exact boundary the architecture brief's scope statement draws
and the reason S5 exists. Mitigation is structural, not textual: the checker's own "what this does not
verify" section states, **first**, that it checks a need is *declared* and never that the page
*answers* it (RS-81-1, `standards/rust/81-checks-that-cannot-be-types.md:11`), and DR-07's reviewer
procedure is the named instrument for the rest.

**Resolves.** DR-05 (and, jointly with the set below, AC-006).

### S1 vocabulary — DT-2: the need set

**Chosen: option (c), adopt with a stated local extension.** Four tokens, closed:

| Token | The page's job | Success for the reader |
| --- | --- | --- |
| `orientation` | route the reader to the page that answers their question | they leave, correctly, within one screen |
| `tutorial` | carry a newcomer through one working thing, staged | they ran it and it worked |
| `how-to` | get a reader who already has a goal to that goal | the goal is achieved |
| `explanation` | build the mental model behind a behaviour | they can predict what the API does before running it |

The extension is **one subtraction and one addition**, each with a named cause:

- **`reference` is removed.** This workspace already has two reference surfaces and both are
  authoritative: rustdoc, and `spec/SPECIFICATION.md`, whose clauses are the normative voice a page
  must cite and never restate (DR-09). A `reference` bucket inside the narrative tree is either
  permanently empty — the "empty or near-empty bucket" half of the eighth anti-pattern — or it
  becomes a second specification, which is the risk the charter lists at project.md Risks row 4.
- **`orientation` is added.** DT-3, below.

**Rejected.**

- **(a) Adopt the four Diátaxis categories literally.** The evidence cuts directly against it for
  this subject: `interaction-patterns.md:498-512` records that Diátaxis's sharpest critic names
  *this project's exact kind of subject* — a dense, interrelated conceptual model — as where the
  four-box split strains, and that Diátaxis's own site is cited as failing to follow its own
  structure. Adopting the four literally would have produced the empty `reference` bucket above.
- **(b) Keep the discipline and drop the enumeration.** Unavailable, not merely unattractive: UX-002
  requires a token checkable against a set, and a lint cannot check membership in an open set. Option
  (b) makes AC-006 unenforceable and reduces the whole project to a convention — the failure mode the
  charter is built to avoid.
- **A persona-keyed taxonomy (`application-author` / `adapter-author` / `evaluator`).** Named as an
  option in the dossier's tension 3. Rejected because a page's need is a property of the page and a
  persona is a property of the reader: the adapter author reads explanations *and* how-tos, so a
  persona token would make almost every page declare two.

**Failure mode and mitigation.** The dossier's failure mode for any fixed category set is content
straining to be two things at once. Mitigation, three parts, all checkable: (1) the set is closed and
`NEEDS.len() <= 6` is a ceiling in the checker, so the answer to a straining page is never "add a
fifth token"; (2) the answer *is* splitting the page, and the reviewer procedure (S5) says so in
those words; (3) adding a member is a two-part commit — the `NEEDS` const **and** the rule atom that
justifies it — and the generated router region (`lint_constitution.rs:388-421`) makes a one-part
commit a gate failure.

**Resolves.** DT-2, AC-003, DR-03.

### S1 vocabulary — DT-3: findability

**Chosen: option (a), a first-class need (`orientation`), with a content ceiling.** The dossier's
tension 7 (`interaction-patterns.md:568-584`) records that the taxonomy is silent on routing and that
this project's evaluator persona names exactly that gap. Left implicit, a landing page is mis-slotted
and then flagged by this project's own lint as answering a second need — the charter states that
outcome in DR-04.

The stated cost of option (a) is that the discipline grows a category its source taxonomy does not
have; the mitigation is a **ceiling rule that makes the category unable to grow into a sink**:

- **RP-10-2.** An `orientation` page contains links, and at most one sentence per destination saying
  what the destination answers. It teaches nothing. The moment it explains, instructs or references,
  it is answering a second need and must be split.
- **RP-10-3.** At most one `orientation` page per directory level. Checkable by the lint (count
  `orientation` declarations per directory), and it is what stops routing pages from breeding.

**Rejected.** **(b) an implicit byproduct** — it is the option that reproduces the measured defect:
the evaluator's gap is *"good until the second question, and then nowhere to go"*, and an implicit
routing responsibility is nobody's. Also rejected: **a bespoke navigation widget** — the dossier's
seventh anti-pattern (`interaction-patterns.md:436-443`) states the evaluator's gap is a missing
*link*, not a missing *widget*, and rustdoc's per-item nav bar and mdBook's sidebar TOC already
perform the function.

**Resolves.** DT-3, AC-004, DR-04.

### S1/S3 — DT-8: the fold line

**Chosen: option (a), a stated rule — in three parts, because a one-clause rule is the version that
is wrong at the edges.**

**Part 1 — the deletion test, as the rule's spirit.** *If the collapsed region were deleted, would
the page still teach the constraint correctly?* If no, it may not be collapsed. Verbatim from
`interaction-patterns.md:404-410`, which is NN/g's own rule, and it is the form the charter's DR-08
already names.

**Part 2 — the closed never-fold list, as the rule's letter.** The test above requires judgement, and
"use good judgment" is the non-answer that let a code-layer invariant drift here once already
(`interaction-patterns.md:484-497`). So five classes may never sit behind a fold, a tab, an inactive
panel or a `<details>`, and no reviewer may grant an exception:

1. the `> **Answers:**` declaration itself (UX-003);
2. any sentence carrying a normative modal or citing a `spec/SPECIFICATION.md` clause id;
3. any statement of an invariant, constraint or precondition;
4. the only occurrence of a code fence the reader is expected to run;
5. any statement of what a check does *not* verify.

**Part 3 — the permission gate, and the honest gap.** A class not on the never-fold list is
*eligible* to be folded; it is **permitted** only if the mechanism appears on an explicit
`PERMITTED_FOLD_MECHANISMS` list in the rules tree, and a mechanism reaches that list only with a
recorded observation *in this repository* that its content is in the accessibility tree, is keyboard
operable, and is found by Ctrl-F and by print (UX-011: an unverified property counts as unmet).

**As of this design, that list is empty, and folding is therefore forbidden in practice.** This is
the substrate gap named plainly rather than designed around: no rustdoc or mdBook render is wired up
in this worktree, so no collapse mechanism can be confirmed safe, and the dossier records the
specific unknown — nothing in `mdbook-tabs`' own documentation states whether an inactive panel is in
`mdbook test`, the search index, or Ctrl-F/print, which it calls *"an unverified property, not a
confirmed safe one"* (`interaction-patterns.md:214-216`). The first entry is HS-P0020's to earn via
DT-7's demonstration; this project states the rule and does not build a fold-checker.

**Rejected.** **(b) reviewer judgement per page** — the trade-off table's own diagnosis: judgement is
right at the edges and drifts, "which is precisely how an invariant stated twice drifted here once
already". Also rejected: **forbidding collapsible content permanently** — it is the third option in
the dossier's tension 2, and it is over-broad; an exercise answer and a long output transcript are
real, and a permanent ban would have to be re-litigated the first time one appears. Part 3's empty
list achieves the same effect *today* while naming exactly what would lift it.

**Failure mode and mitigation.** The trade-off says a stated rule "will be wrong at the edges". The
mitigation is that the two lists have opposite openness: the never-fold list is **closed** (an edge
case never shrinks it) and the permitted-mechanism list is **open by amendment with evidence** (an
edge case grows it, in the rules tree, in public, with a recorded observation). An edge case is
therefore never resolved by a one-off reviewer exception, which is the drift the tension names.

**Resolves.** DT-8, AC-005, DR-08, and the rule half of BR-11.

### S2 — `discipline-router`: filter plus complete generated index

**Chosen.** The router carries, in order, a one-paragraph scope statement, a numeric band table, its
rank inside the precedence chain, an intent-keyed "Start here" table, and a **generated** complete
index between `<!-- BEGIN GENERATED -->` and `<!-- END GENERATED -->`.

**Pattern citation.** `standards/rust/README.md:1-21` (scope + band table), `:23-29` (precedence),
`:45-58` (`## Start here`, the filter), `:61-97` (`## Index`, the unfiltered whole, generated by
`--write` and checked for equality by `check_router`, `lint_constitution.rs:334-384`). This is the
one shape in the repository proven to let a reader load one of twenty-seven atoms without loading
the corpus, and its index cannot fall behind its corpus because the equality *is* the check.

**Rejected.** **A hand-maintained index** — a second copy of the truth (`check_summaries`'
statement of the general form, `:466-470`). **A filter-only router** listing "common" rules —
violates UX-004's non-occlusion invariant: a filter must not hide what it filters. **A generated
index of pages and their declared needs**, tempting because it would make the corpus visible in one
place — deferred, and deliberately: its source is HS-P0020's tree, so it would make the rules tree
change on every page addition and would couple two projects' commit cadence. The architecture brief's
Note 9 flags the same and defers to this stage; this stage declines it, and the reviewer procedure
(S5) does the same job for a human at review time.

**Resolves.** AC-001, DR-01, UX-004, UX-012.

### S2 — DR-02: rank inside the precedence chain

**Chosen.** The router states, in its own text, that the discipline sits **at the constitution-atom
tier** of the existing five-tier chain (`SPECIFICATION clause > ADR > constitution atom > CLAUDE.md /
CONTRIBUTING.md summary > references/evaluation/*`), *alongside* `standards/rust/`, on the same rank,
scoped to a different subject. It adds no tier and `standards/rust/README.md:23-29` is not edited —
the checkable form is that `git diff main -- standards/rust/README.md` is empty (AC-002).

**Rejected.** **A new sixth tier** — an initiative-level non-goal. **Leaving the rank unstated and
linking to the chain** — fails UX-005: a reader who has to open another file to work out whether a
page rule beats a clause will guess.

### S4 — `lint-terminal-output`: report-all with the repair in the message

**Chosen.** One line per problem: `{path}:{line} — {what is wrong}; {why it matters, or what to do}`.
Every problem is pushed onto one `Vec<String>` and **all** are printed before `bail!("{n} problem(s)
in {RULE_DIR}")`. A green run states what it checked. An empty tree is a `bail!`, never a pass.

**Pattern citation.** `xtask/src/lint_constitution.rs:169-198` (report-all-then-bail), `:176`
(vacuity `bail!`), `:192` (the success line), `:375-379` (the repair inside the message —
"run `cargo xtask … --write`"), `:431-441` / `:480-484` / `:497-501` (the message shape). No
alternative was seriously considered because this grammar is already the gate's, and a second
diagnostic dialect in the same `cargo xtask ci` output would be its own defect.

**Failure mode and mitigation.** The documented failure of report-all is a wall of text on a badly
broken tree. It is **not** mitigated by truncating — truncation is how a reviewer misses the third
problem and turns one cycle into two. It is mitigated by ordering (see Density budget: problems sort
by path then line, so the output is a diff-shaped worklist) and by the count in the `bail!` line, so
the reader knows before scrolling whether they are looking at three problems or thirty.

**Resolves.** AC-007, UX-007, UX-008.

### S5 — `reviewer-procedure`: an ordered walk yielding a verdict

**Chosen.** A numbered list a stranger executes top to bottom from the rendered page alone, closing
in a three-row verdict table (`pass` / `fail — two needs` / `fail — need not answered`). Each step is
a question with a yes/no answer and a stated consequence; no step says "consider" or "use judgment".

**Pattern citation.** The `Do` / `Not` / `Rejects.` grammar of a constitution rule
(`standards/rust/README.md:99-107`, worked at `00-prime-directives.md:30,54,78`) — a shape that
already forces a named wrong state rather than an impression. The dossier's contribution is negative
and load-bearing: `interaction-patterns.md`'s quiz entry rules out an author-authored comprehension
check as evidence, which is why this procedure yields a *verdict on the page's shape* and never a
claim about reader understanding.

**Rejected.** **A checklist of adjectives** ("is the page clear, focused, useful") — unfalsifiable and
author-flattering. **A quiz** — measures recall of the prose that wrote it. **Delegating to the lint**
— the lint cannot see the thing this procedure exists for.

**Resolves.** AC-009, DR-07, DoD-8.

## Composition

Regions in order, with what sits **relative to** what. Order is the only positional language a text
medium has, so it is binding.

### S1 — the governed page's head

```text
┌──────────────────────────────────────────────────────────────┐
│ # <Page title>                                    ← H1, line 1│
│                                                   ← blank     │
│ > **Answers:** `token` — <reader's question>?     ← the label │
│                                                   ← blank     │
│ <first prose paragraph>                                       │
└──────────────────────────────────────────────────────────────┘
```

Exactly one blank line above and below. **Nothing may be inserted between the H1 and the
declaration** — no badge row, no table of contents, no admonition, no "last updated" line. The
declaration's position *is* its meaning: UX-001's test is "read nothing but the region above the
first prose paragraph and name the need", and any interposed element makes that region ambiguous.

Everything else on the page sits below the first paragraph, in the page's own order. This design says
nothing about it — that is HS-P0022/HS-P0023's content, governed by the rules, not arranged here.

### S2 — the router, top to bottom

1. **`# Page standards`** — the H1.
2. **Scope paragraph** — one paragraph, and the load instruction ("load one rule, never the tree"),
   directly under the H1, before anything tabular. Mirrors `standards/rust/README.md:1-8`.
3. **Band table** — `| Band | Owns |`, five rows (`00`, `10`, `20`, `30`, `40`). Immediately after
   the scope paragraph, because it is the cheapest route: a reader who can guess the band never reads
   the rest of the page.
4. **`## Precedence`** — the rank statement (DR-02), as a blockquote. It sits *above* `## Start here`
   deliberately, inverting nothing from `standards/rust/README.md` (`:23-29` is likewise above
   `:45-58`): a reader who does not yet know whether these rules bind them should learn that before
   they are routed to one.
5. **`## Start here`** — the intent-keyed table, `| You are… | Load |`.
6. **`## Index`** — the generated region, complete, every rule listed. Last, because it is the
   fallback for the reader the filter failed, and it is the longest thing on the page.
7. **`## The shape of a rule`** — the authoring grammar, for the reader who is writing a rule rather
   than obeying one. Last of all: it serves the smallest audience.

The filter (3, 5) is always **above** the thing it filters (6). That vertical relationship is
UX-004's non-occlusion invariant expressed as composition.

### S3 — a rule atom, top to bottom

`# NN — <Title>` · `> **Load when:** …` · `> **See also:** …` · `---` · then one `## RP-NN-N.
<imperative sentence>` per rule, each followed by exactly five sections in fixed order: **Why.** ·
**Do** · **Not** · **Rejects.** · **Evidence.** Copied from
`standards/rust/00-prime-directives.md:1-8,19,30,54,78` and enforced by a `SECTIONS` analogue of
`lint_constitution.rs:67-81`.

Two deliberate differences from the constitution's atoms, both consequences of this tree not being
registered with the doctest harness (`xtask/src/lib.rs:28`, CR-0):

- **`Do` and `Not` hold markdown page fragments, not Rust.** A `rust`-tagged fence is **rejected**
  by the checker, with a message saying why — nothing compiles it, so it would be a Rust claim
  nothing checks. Untagged fences are rejected too, so a future decision to register the tree cannot
  be undermined retroactively. Fences are tagged `text` or `markdown`.
- **`Evidence.` cites `path:line` in this repository first**, then the dossier, then external URLs
  with a `*(checked …)*` stamp — the constitution's own ordering.

### S4 — one terminal run, top to bottom

```text
$ cargo xtask ci
  …
  every page declares one need
  standards/pages/20-the-fold-line.md:41 — 7 rules, and the ceiling is 6; split the atom
  <PAGE_DIR>/append-conditions.md:3 — declares `explanation` and `how-to`; a page answers one need
  <PAGE_DIR>/getting-started.md — no `> **Answers:**` line; see standards/pages/00-one-need.md
Error: 3 problem(s) in standards/pages + <PAGE_DIR>
```

Problems are printed as one block, **sorted by path then line**, with no blank lines between them and
no per-problem heading. The step's own name prints above them (the existing `Step` machinery does
this); the `bail!` count prints below. There is no summary section, no "next steps" paragraph and no
box drawing — every line the reader has to skip is a line the third problem is hiding behind.

### S5 — the reviewer procedure, in the atom that carries it

`## RP-40-1.` heading · a one-paragraph statement of who performs it (**not** the author) and what
they may consult (the rendered page, the router, `spec/SPECIFICATION.md`; **not** the author, and not
the page's git history) · the ordered walk · the verdict table · a `**Rejects.**` naming the review
that reaches "looks fine" without executing the walk.

## Transience policy

The three transience classes map onto a text medium exactly, and the mapping is stated so the
implementer cannot silently pick a fourth: **persistent chrome** = rendered in the document by
default, in every state; **revealed on hover/focus** = rendered by the medium's own affordance
without an author decision (rustdoc's heading anchor links, a `title` attribute); **opened on
demand** = behind a fold, a tab, an inactive panel, or a link to another page.

| Control | Class | Why |
| --- | --- | --- |
| S1 declaration line | **persistent chrome** | UX-001/UX-003. Its whole function is being read before the page is; it is never-fold class 1 |
| S1 need token | **persistent chrome**, and never abbreviated | UX-002: the word carries the meaning, not a colour or an icon |
| S2 scope paragraph + band table | **persistent chrome** | the cheapest route to one rule; a collapsed band table is a router that does not route |
| S2 precedence blockquote | **persistent chrome** | UX-005: a reader must not click to learn whether the rules bind them |
| S2 `## Start here` table | **persistent chrome** | the filter |
| S2 generated `## Index` | **persistent chrome**, always complete | UX-004: hiding the filtered set behind a disclosure is exactly the occlusion the invariant forbids. It is also the longest region, and it is *still* not folded — length is not a reason |
| S2 links from index to atoms | **opened on demand** | this is the one intended context switch on the page, and the point of UX-012: the reader loads one rule, not the tree |
| S3 rule `Why.`/`Do`/`Not`/`Rejects.`/`Evidence.` | **persistent chrome** | `Rejects.` names a wrong state that could ship; never-fold class 3 |
| S3 long `Evidence.` URL lists | **persistent chrome** | eligible to fold under Part 2, forbidden by Part 3's empty mechanism list. Revisit only when that list has an entry |
| S3 heading anchor links | **revealed on hover/focus** | the medium's own, not authored here; explicitly *not* an authored affordance (`interaction-patterns.md:436-443`) |
| S4 problem lines | **persistent chrome** | all of them, every run; report-all is the invariant |
| S4 success line | **persistent chrome** | a green run that prints nothing is indistinguishable from a step that did not run — the `RUNBOOK.md:920-925` incident in one sentence |
| S4 repair instruction | **persistent chrome, inside the problem line** | not a footer, not a separate paragraph: the reader who scrolled away from problem 1 must still see its repair |
| S4 per-file progress / spinner | **not rendered at all** | this is a file read over tens of files; progress output would be the only moving thing in a gate whose entire value is a legible diff of what failed |
| S5 walk steps | **persistent chrome** | a procedure whose steps are behind a disclosure is a procedure a stranger abandons |
| S5 verdict table | **persistent chrome** | it is the output of the procedure |
| S5 worked example of a failing page | **opened on demand** — a link to a fixture, not an inline fold | it is long, it is genuinely an aside, and a link is the one "opened on demand" mechanism whose accessibility this repository does not have to verify |

**Nothing in this project is persistent because nobody decided otherwise.** The two controls that
were argued and *stayed* persistent are S2's generated index (length is not a reason to fold the
thing a filter filters) and S3's `Evidence.` lists (eligible, but Part 3 has no permitted mechanism).
The one control deliberately removed is S4's progress output.

## Density budget

**How the browser viewports map onto a text medium, stated as an assumption rather than asserted as a
measurement.** No render exists in this worktree, so pixel budgets cannot be verified and are not
claimed. What *is* measurable, today, with `awk`, is the source column count and the byte size — so
the budget is expressed there, and the two viewports are given as the two rendering contexts the
column budget must survive.

| | 1440×900 | 1024×768 |
| --- | --- | --- |
| Assumed rendering context | a wide markdown render (mdBook/GitHub/rustdoc) beside an editor; a terminal at ≥ 120 columns | a single markdown column, or a terminal at 80 columns |
| S1 declaration | **1 rendered line** | **≤ 2 rendered lines**, and within the first two block elements of the page, so it is above the fold on any viewport ≥ 480px tall |
| S2 band + `Start here` tables | both fully visible without horizontal scroll | tables may wrap; **no cell is abbreviated or truncated** |
| S4 problem line | 1 terminal line, no wrap | **≤ 100 characters, so it does not wrap at 100 columns**; if it must wrap, it wraps *after* `path:line —`, never inside it |

**Source-measurable ceilings, all enforced by the checker.** Measured against the existing corpus so
they are calibrated, not guessed: `standards/rust/README.md`'s longest non-table line is **93**
columns; the largest existing atom is **15,616** bytes against a `MAX_ATOM_BYTES` of **16,384**
(`lint_constitution.rs:95`).

| Budget | Value | Reason |
| --- | --- | --- |
| Declaration line, total | **≤ 96 characters** (hard); ≤ 80 target | 80 is the plain-text pager and `git diff` case; 96 matches the prose wrap the constitution already holds |
| Declaration's question clause | **≥ 65 characters available** with the longest token | `> **Answers:** ` (15) + `` `explanation` `` (13) + ` — ` (3) = 31 of the 96 |
| Need tokens in `NEEDS` | **≤ 6**, four today | a closed set that grows past six is bucket proliferation — the failure DT-2 chose (c) to avoid |
| Rules per rule-atom | **6** | identical to `MAX_RULES_PER_ATOM` (`:88`) on purpose: two trees teaching two different numbers for the same idea is its own defect |
| Bytes per rule-atom | **16,384** | identical to `MAX_ATOM_BYTES` (`:95`), same reason, and the message carries the same explanation — "an agent loading this pays for all of it" (`:503-508`) |
| Router, total | **≤ 8,192 bytes** — half an atom's ceiling | new, and this tree's own: the router is the one file *every* page author loads, and a router at atom scale is a corpus |
| Router `Start here` rows | **≤ 12** | past a dozen, a filter is a second index and the reader reads both |
| Prose line width, outside tables | **≤ 96 columns** | measured ceiling of the existing corpus is 93; tables exempt (the corpus already runs to 124 there) |
| Problem lines per run | **unbounded — never truncated** | truncation is how the third problem is missed |

**What yields first when a budget is exceeded**, per surface — stated because a budget without a
yield order is decided by whichever element shrinks:

- **S1.** The *question clause* is shortened. The token never is: it is the primary label and it is
  never abbreviated, never initialised, never dropped. If the question cannot be said in the
  remaining characters, the page is answering more than one need — the overflow is the diagnosis.
- **S2.** The `## Index` never yields; it is generated and complete by construction. The `Start
  here` table yields first (merge rows), then the scope paragraph (shorten), then the band table
  (never — it is the numeric namespace). A router that outgrows 8,192 bytes means the tree needs a
  band, not that the router needs trimming.
- **S3.** `Evidence.` yields first (drop external URLs, keep repo `path:line`), then `Why.`
  (≤ 3 sentences, as the constitution's own shape requires). `Not` and `Rejects.` never yield: they
  are the only sections that name a wrong state, and an atom that drops them is decorative.
- **S4.** Nothing yields. Both the problem count and the per-problem repair stay. If a message
  cannot fit in 100 characters, the *explanation half* moves into the rule atom and the message
  cites it by path — the location and the repair pointer are never what is cut.
- **S5.** The worked example yields (it is a link already). The walk's steps never do.

**Minimum legible size for the primary label.** The primary label is the need token. It renders at
**body-text size in whatever the medium's body size is** — never inside `<small>`, `<sub>`, a caption
element or a footnote, and never smaller than the paragraph beneath it. The checkable form, which
needs no render: the declaration is a blockquote containing inline code and normal body text, and it
uses no size-affecting markup at all. In a plain-text pager it is the same size as everything else,
which is the floor this rule guarantees on every medium at once.

## Hierarchy

Per region: what is primary, secondary, recessive — and **what carries the distinction**. In a text
medium the carriers are position, heading level, block type (blockquote / table / fence), the
bold-marker convention (`**Why.**`), and monospace. **Colour is never a carrier** (UX-002); the
repository already practises this — the constitution distinguishes right from wrong with the literal
words **Do** and **Not** (`standards/rust/README.md:99-107`).

| Region | Primary | Secondary | Recessive | Carried by |
| --- | --- | --- | --- | --- |
| S1 page head | the need **token** | the reader's question | the page title | position (line 3 of 5), inline-code monospace on the token, the bold `**Answers:**` marker |
| S2 router | the band table | `## Start here`, `## Precedence` | `## Index`, `## The shape of a rule` | vertical order; the index is recessive by being last and generated, not by being smaller |
| S3 rule atom | the `## RP-NN-N.` imperative sentence | `Do` / `Not` | `Why.`, `Rejects.`, `Evidence.` | heading level (`##` vs bold run-in markers); the imperative *is* the rule, the sections are its support |
| S4 terminal | `path:line` | what is wrong | why it matters / the repair | left-to-right order within one line — the reader scanning a column of problems reads locations first |
| S5 procedure | the ordered steps | the verdict table | who may perform it, what they may consult | numbered-list structure vs prose |

One inversion worth naming: in S4 the **repair** is recessive within the line but is never *removed*
(Transience policy). Recessive means "read third", not "read never".

## States

Six states per surface. "Loading" has no analogue on a static page and is stated as such rather than
left blank.

| State | S1 declaration | S2 router | S3 rule atom | S4 terminal | S5 procedure |
| --- | --- | --- | --- | --- | --- |
| **Empty** | a page with no `> **Answers:**` line → the lint reports `{path} — no `> **Answers:**` line`, naming the rule atom to read | a tree with no rule atoms → **`bail!`**, never "0 rules, all consistent" (`:176`) | an atom with no `## RP-` rule → problem, mirroring `:494-495` | zero problems → the success line, `  {n} pages, {m} rules, all consistent` | no pages to review → the procedure states the walk is vacuous and the reviewer records that, rather than recording a pass |
| **Loading** | N/A — static text | N/A | N/A | the step prints its name before it reads; no spinner, no per-file progress | N/A |
| **Error** | two declarations, or an unenumerated token → one problem per page, each with the line number *of the offending declaration* | the generated region disagrees with `NEEDS` → problem naming **which member moved** (RS-81-5, `81-checks-that-cannot-be-types.md:335`) and the repair `cargo xtask lint-pages --write` | a `rust`-tagged fence → rejected, with the reason (nothing compiles it) | the tree is missing → `.with_context()` naming the pinned path (`:212`); the run fails, it does not skip | the walk cannot be completed from the page alone → verdict `indeterminate`, which is recorded as a **defect in the page**, not in the procedure |
| **Overflow** | question clause over budget → the page is splitting; see Density budget's yield order | over 12 `Start here` rows → merge rows; index unaffected | over 6 rules or 16,384 bytes → split the atom, with the byte count in the message | thirty problems → all thirty print, sorted by path then line, count in the `bail!` | more pages than one sitting → the walk is per-page and resumable; the ledger records which pages were walked, never "the set" |
| **Long label** | a token is never long — the set is closed and the longest is `explanation` (11 chars), which is why a closed set is also a density decision | a long atom title wraps inside the table cell; **never truncated with an ellipsis** | a long `## RP-NN-N.` sentence wraps at 96 columns | a long path pushes the message past 100 chars → the explanation moves to the rule atom and the message cites it | — |
| **Narrow viewport** | ≤ 2 rendered lines at 1024×768; still the first thing after the H1 | tables wrap; no cell abbreviated; horizontal scroll is acceptable, hidden content is not | body text wraps at the source's 96 columns | 80-column terminal → the line wraps *after* `path:line —`; the location stays on the first line | — |

## Anti-patterns

Each is checkable against a screenshot of a rendered page or a terminal, by someone who cannot read
the code.

1. **A page whose first visible content after the title is anything other than the `**Answers:**`
   line.** A badge row, a table of contents, an admonition, an "updated on" line — any of them
   between the H1 and the declaration is a fail.
2. **A need shown as a colour, an icon, a coloured pill, or an emoji with no word.** Screenshot in
   greyscale: if you cannot read the need, it fails.
3. **The declaration rendered smaller than the paragraph under it**, or inside a caption, footnote or
   `<small>`.
4. **Two need words visible in the declaration region** — including "explanation and how-to",
   "mostly reference", or a slash.
5. **A declaration, an invariant, a `MUST`, a clause citation, or a "what this does not verify"
   statement that is not visible until something is clicked.** Screenshot the page as it first
   loads: if any of the five never-fold classes is missing from it, it fails.
6. **Any `<details>`, tab strip or accordion on a governed page at all**, while
   `PERMITTED_FOLD_MECHANISMS` is empty. There is no exception, and "it's only an aside" is the
   exception the rule exists to refuse.
7. **A router whose index is behind a "show all rules" toggle, a fold, or a second page.** The filter
   and the whole must be on one screenshot's page.
8. **A router that lists only some rules**, or an index whose row count disagrees with the number of
   files in the tree.
9. **A bespoke navigation widget** — a breadcrumb bar, a sidebar rail, a custom search box — added on
   top of what rustdoc or mdBook already renders.
10. **A terminal screenshot showing one problem and the word "and others", "…", or a truncation
    marker.** Every problem, every run.
11. **A green run that printed nothing.** A screenshot of a passing gate must show the step's name
    and a count.
12. **A failure message with no `path:line` at its start**, or whose repair instruction is in a
    footer paragraph rather than in the line itself.
13. **A `rust`-tagged code fence anywhere in `standards/pages/`.** Nothing compiles it; it is a Rust
    claim wearing syntax highlighting.
14. **A rule atom with no `Not` or no `Rejects.` section.** Screenshot the atom: if it never shows a
    wrong page, it is decorative.
15. **A reviewer procedure step containing "consider", "use judgement", "as appropriate", or "if it
    seems".** Every step is a question with a yes/no answer.
16. **A `reference` declaration on any page**, or a fifth token that is not in the router's generated
    index.

## Mock

| Mock | Path | Viewports | Themes | Status |
| --- | --- | --- | --- | --- |
| Static sign-off contact sheet — 5 surfaces × 26 states × 2 viewports = 52 labelled frames | [`design/mock.html`](design/mock.html) | 1440×900, 1024×768 | light | built, awaiting sign-off |

**Self-contained.** No network fetch of any kind: no font, no stylesheet, no image, no script.
Every `url()` in it is an inline `data:image/svg+xml`. Open it from disk or serve it; it renders the
same.

**What it is composed from, and the correction it forced.** This section previously said the mock
should be three hand-written textual artefacts and that no image was required, on the stated premise
that this repository has no CSS to compose against. The first half stands and the second half was
wrong. The tracked tree has no authored CSS — verified again, `rg --files -g '*.css' -g '*.scss'
-g '*.tokens.*' -g 'tailwind*'` returns nothing — but the gate's own `documentation` step **emits
one**: `target/doc/static.files/rustdoc-17e0aaed.css`, 69,637 bytes, with a real light-theme custom
property block (`--main-background-color`, `--main-color`, `--code-block-background-color`,
`--border-color`, `--headings-border-bottom-color`, `--link-color`, `--font-family`,
`--docblock-indent`, `--desktop-sidebar-width`) and real class contracts (`.docblock`,
`.docblock table`, `.example-wrap`, `pre.rust.rust-example-rendered`, `a.doc-anchor`,
`details.toggle.top-doc`, `main`, `.width-limiter`).

So every markdown frame is painted by **160 of rustdoc's own top-level rules, declarations
verbatim**, selectors re-scoped to `.rd` so they cannot repaint the sheet around them, with no
layout override authored at all; and the markup mirrors
`target/doc/happenstance/index.html` element for element. Four omissions are documented in a comment
at the top of the mock's stylesheet: `@font-face` (would fetch woff2), `@media` (rustdoc's
breakpoints key off the real browser width, not a 1024px box inside a wider window),
renderer chrome (sidebar, topbar, search), and the `dark`/`ayu` theme blocks — the last of which,
left in, would have won on source order and rendered every frame labelled *light* in ayu dark.
That is the mock earning its place before a single line of feature code exists.

Everything not under `.rd` is viewer chrome invented for the contact sheet, is labelled as such in
the same comment, and carries no surface content: strip the whole stylesheet and every frame still
says what it says, because every frame is markdown or terminal text.

**Honest density is measured, not asserted.** Each frame's chips are computed from the specimen's
own bytes at build time — declaration length, widest prose line, atom bytes, rule count, router
bytes, `Start here` rows, walk steps, hedging words, longest problem line — and go amber when the
specimen breaches a budget this file sets. Several do deliberately: a state named
`over-rule-ceiling` that stayed under the ceiling would prove nothing.

**Five findings** are recorded in the mock's closing panel and are the substance of the sign-off
conversation. Four bear directly on decisions above and are named here so this file does not
disagree with its own mock:

1. **The 100-character problem-line budget is not reachable for this tree.** After the Density
   budget's own yield remedy is applied, `<PAGE_DIR>/guide/first-projection.md:7` plus
   `see standards/pages/20-the-fold-line.md` is still 112 characters, and the inherited spelling at
   `lint_constitution.rs:369-378` is 111 on its own. The budget, the citation form, or the no-wrap
   rule has to give.
2. **rustdoc wraps every page in `<details class="toggle top-doc" open>`** and ships a
   `#toggle-all-docs` control that closes it. Under rustdoc hosting the declaration — never-fold
   class 1 — is already inside a disclosure mechanism nobody has observed. DT-8 Part 3 owes a
   sentence on whether a renderer-supplied, open-by-default wrapper counts as a *mechanism*.
3. **The verdict table is specified twice and differently** — three rows in Composition, four states
   in the States block. The mock draws four, because the States block is its addressing manifest.
4. **The S4 selector regex is narrower than its own precedent's messages**, which address a whole
   file with no line number in four of six cases.

The declaration form itself survived the one renderer that exists: rustdoc's stylesheet contains
**zero** `blockquote` rules, so the `> **Answers:**` line wears the user agent's default indent and
nothing else — no left rule, no tint, no icon — and still reads, because the meaning is carried by
the bold marker, the monospace token and the position. DR-05's hosting assumption is discharged
against a real render rather than assumed.

**Reference captures for the design review.** `design.capture` is absent from
`.redkiln/config.yaml`, so the perceptual review is a confirmed skip and **nothing will produce
these automatically**. They are named here so that a reviewer who chooses to capture them by hand,
or a later phase that wires a renderer up, has one agreed set of paths:

| Surface | 1440×900 | 1024×768 |
| --- | --- | --- |
| `page-need-declaration` | `design/reference/page-need-declaration@1440x900.png` | `design/reference/page-need-declaration@1024x768.png` |
| `discipline-router` | `design/reference/discipline-router@1440x900.png` | `design/reference/discipline-router@1024x768.png` |
| `rule-atom` | `design/reference/rule-atom@1440x900.png` | `design/reference/rule-atom@1024x768.png` |
| `lint-terminal-output` | `design/reference/lint-terminal-output@1440x900.png` | `design/reference/lint-terminal-output@1024x768.png` |
| `reviewer-procedure` | `design/reference/reviewer-procedure@1440x900.png` | `design/reference/reviewer-procedure@1024x768.png` |

The original note still holds for the *content*: a screenshot of markdown is a worse artefact than
the markdown, and `mock.html` is the reviewable artefact. The capture paths exist so that a review
which does look at pixels has somewhere agreed to put them, not because an image is required.

## Sign-off

**Approved by Ryan Britton (repository owner), 2026-08-17, with no conditions beyond the four
recorded below**, which are carried as stated rather than waived. The verifier found no gaps,
no dead anchors and no unstyled findings on this project.

Recorded via `redkiln advance HS-P0021 --verdict approved --stay --apply`, which clears the
review gate without moving the project off `design`.

The three tensions this project owns are settled: **DT-2** as a closed four-token needs set
(`orientation`, `tutorial`, `how-to`, `explanation`) — dropping `reference`, which rustdoc and
`spec/SPECIFICATION.md` already own and which would otherwise become a second specification;
**DT-3** making findability a first-class need with two ceiling rules so the new category
cannot become a sink; and **DT-8** as a closed never-fold list of five classes behind a
permission gate that ships empty.

Conditions that must be attached at sign-off, because each is a decision this stage took under a
named gap:

1. **DR-05's hosting assumption** — the declaration form assumes only that the medium renders
   CommonMark blockquotes as visible body text in document order. HS-P0020's hosting decision is
   still open; if it lands on a shape that violates that assumption, this decision is invalidated and
   DR-05 re-opens (project.md, Risks row 1).
2. **`standards/pages/` is pinned by this file**, and pinning it costs three edits in
   `xtask/src/main.rs`, one `INERT` entry in `xtask/src/affected.rs:249-260`, and the two `affected`
   tests the architecture brief's CR-4 requires. Renaming it later is not a rename.
3. **`PERMITTED_FOLD_MECHANISMS` ships empty**, which means folding is forbidden in practice until
   HS-P0020's DT-7 demonstration earns the first entry. If that is not the intent, DT-8's Part 3 is
   what to disagree with.
4. **No pixel budget is claimed.** The density budget binds source columns and bytes, which are
   measurable today; the viewport rows are stated as assumptions about a render that does not exist
   in this worktree yet.
