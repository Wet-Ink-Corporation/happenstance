# Verification — The sequenced adapter reasoning account

The reviewer-repeatable pass this story's spec asks for (`spec.md`, `## Tests and CI`). This project
takes no testing brief by decision; its verification is a recorded observation plus an `rg` one-liner,
and both are written here rather than dressed up as a test suite. Every command below was run from the
repository root of the worktree `D:/repos/happenstance/.claude/worktrees/docs-that-teach` on branch
`initiative/docs-that-teach`.

The page under verification is **`docs/adapter-reading-order.md`**.

## EC-001 — the mount exists, so the story does not block

`spec.md` `## Error conditions` EC-001 requires this story to **block** if HS-P0020's pinned narrative
tree or its registration path has not landed. Re-verified at implementation start (`_grounding.md` says
to re-verify rather than assume, and the state had moved):

| What the spec calls it | What it is, on this branch | Where |
| --- | --- | --- |
| the pinned narrative tree | `docs/` | `xtask/src/lint_narrative.rs:239` — `pub(crate) const TREE: &str = "docs";` |
| the tree's index | `docs/README.md`, exempt from registration and from the need check | `xtask/src/lint_narrative.rs:250`; `xtask/src/lint_pages.rs:496` |
| the no-orphan mechanism | `check_registration`, bidirectional — every page registered, every registration naming a page | `xtask/src/lint_narrative.rs:536-563` |
| the registration file | `xtask/src/narrative.rs` — one `#[cfg(doctest)] mod` + `include_str!` per page | `xtask/src/lint_narrative.rs:257` — `const HARNESS` |
| the mandatory step | `every narrative page is checked`, and `every page declares one need` | `xtask/src/lint_narrative.rs:351`; `xtask/src/lint_pages.rs:188` |

HS-P0021's page-need discipline has landed too (`standards/pages/`, four rule bands plus a router), so
this page is authored **under** that rule rather than toward it. Design finding **F5** (`_design.md`) is
**closed for this surface**: `route` is bound to `docs/adapter-reading-order.md`.

## AC-001 — registered, not a loose file

The mount was proved by watching it fail first.

**Red** — the page written into `docs/` with no entry in the harness:

```console
$ cargo xtask narrative
  xtask/src/narrative.rs — does not include adapter-reading-order.md; its examples are never compiled
  xtask/src/narrative.rs — no `mod adapter_reading_order`; one module per page is what keeps a doctest failure's line number relative to the page
xtask failed: 2 problem(s) in docs
```

**Green** — after `xtask/src/narrative.rs:153-162` (`mod adapter_reading_order`, `:160-161`):

```console
$ cargo xtask narrative
  6 pages, all consistent
```

That red transcript **is** the falsification the AC asks for, run rather than asserted: delete the
registration entry and the mandatory step fails by name. The page is not under `docs/` as a loose file,
is not a `//!` module doc, and is not a section appended to `CONTRIBUTING.md`.

The tree's index carries it as well — `docs/README.md`, the `Page | Read it at` table — so a human
browsing the tree reaches it without reading `xtask/src/`.

## AC-002 — five regions in the bound order, and the reading order inside the first screen

Region order, against `_design.md` `## Composition`, `adapter-reasoning-account`:

| Region | Where it is | Lines |
| --- | --- | --- |
| 1 · H1 + answered-need | `# What to read before you write an adapter` then the declaration | `:1`, `:3` |
| 2 · the reading order | the six-entry numbered list | `:7-19` |
| 3 · the caveat, at entry 1 | `**And it is not an adapter.**` | `:28-35` |
| 4 · six positioned sections | `## 1 of 6 …` through `## 6 of 6 …` | `:21`, `:37`, `:44`, `:50`, `:58`, `:65` |
| 5 · terminal region | `## What this page does not cover` + one hop | `:71-77` |

No region is reordered and none is omitted. The reading order precedes the first section by 2 source
lines and is the page's only enumerated element:

```console
$ rg -n '^\s*[-*+] |^\s*[0-9]+\. ' docs/adapter-reading-order.md
7:1. …  9:2. …  11:3. …  13:4. …  15:5. …  18:6. …
```

Six matches, all in region 2. No bullet list anywhere on the page.

**First-screen occupancy, measured rather than asserted.** `_design.md` `## Density budget` defines the
first screen as ≈ 27 rendered lines at 1024x768 and budgets H1 + need (3) + list (≤ 12) = **≤ 15**. The
rendered length of each element with markdown markers stripped, and the line count it takes at both a
96-column and a 105-column content box (the mock measured a 936px box, which is ~95–105 characters of
this tree's prose):

| Element | Rendered characters | Lines @96 | Lines @105 |
| --- | --- | --- | --- |
| H1 | 42 | 1 | 1 |
| declaration | 90 | 1 | 1 |
| lead-in | 83 | 1 | 1 |
| entry 1 | 176 | 2 | 2 |
| entry 2 | 173 | 2 | 2 |
| entry 3 | 162 | 2 | 2 |
| entry 4 | 159 | 2 | 2 |
| entry 5 | 148 | 2 | 2 |
| entry 6 | 143 | 2 | 2 |
| **total** | | **15** | **15** |

15 ≤ 15, against a first screen of ≈ 27, leaving ~12 rendered lines of headroom before
`## 1 of 6` is reached. No entry was dropped or folded to fit, and no entry exceeds two rendered lines.
Checked against `design/mock.html#all`, the `adapter-reasoning-account` / `first-screen-reading-order`
frame at 1024x768: same H1-then-need-then-`<ol>`-then-first-`h2` composition, same six entries, same
"≤ 2 rendered lines each" shape.

## AC-003 — the six sources, in the decided order, cited and not copied

```console
$ rg -n --no-heading -o "crates/happenstance-core/src/memory\.rs|standards/rust/91-adapter-authoring-recipe\.md|CONTRIBUTING\.md|standards/rust/20-two-flavour-ports\.md|standards/rust/25-what-removes-send-and-sync\.md" docs/adapter-reading-order.md
7:crates/happenstance-core/src/memory.rs          ← reading order, entry 1
9:standards/rust/91-adapter-authoring-recipe.md   ← entry 2
11:CONTRIBUTING.md                                 ← entry 3
13:standards/rust/20-two-flavour-ports.md          ← entry 4
16:standards/rust/25-what-removes-send-and-sync.md ← entry 5
18:CONTRIBUTING.md                                 ← entry 6
24:crates/happenstance-core/src/memory.rs          ← section 1 of 6
34:standards/rust/91-adapter-authoring-recipe.md   ← the AC-005 pairing, inside the caveat
39:standards/rust/91-adapter-authoring-recipe.md   ← section 2 of 6
46:CONTRIBUTING.md                                 ← section 3 of 6
52:standards/rust/20-two-flavour-ports.md          ← section 4 of 6
61:standards/rust/25-what-removes-send-and-sync.md ← section 5 of 6
67:CONTRIBUTING.md                                 ← section 6 of 6
```

Document order holds in both bands. Two occurrences are *more* than one per path and both are required
rather than accidental: `CONTRIBUTING.md` is **two** of the six sources by the spec's own list (two
different headings), and the occurrence at `:34` is the pairing AC-005 mandates inside the caveat.

**Anchored named-subject + path, and every subject resolves.** Line ranges appear nowhere on the page as
a handle; each citation names a subject string that `rg` finds in the cited file today:

| Entry | Subject string on the page | Resolves |
| --- | --- | --- |
| 1 | `"The reference implementation"` | `rg -c "The reference implementation" crates/happenstance-core/src/memory.rs` → **1** (`:16`) |
| 2 | `RS-91-1..4`, `PgStore` | `rg -c "PgStore" standards/rust/91-adapter-authoring-recipe.md` → **6** (`:55`, `:57`, …) |
| 3 | `"Writing an adapter"` | `rg -c "Writing an adapter" CONTRIBUTING.md` → **1** (`:69`) |
| 4 | `RS-20-1` | `rg -c "RS-20-1" standards/rust/20-two-flavour-ports.md` → **1** |
| 5 | `RS-25-1` | `rg -c "RS-25-1" standards/rust/25-what-removes-send-and-sync.md` → **1** |
| 6 | `"Adding a method to a port"` | `rg -n "Adding a method to a port" CONTRIBUTING.md` → `97:## Adding a method to a port` |

**EC-006, checked explicitly.** The sixth source's heading is `Adding a method to a port`:

```console
$ rg -n "Provided methods" docs/adapter-reading-order.md CONTRIBUTING.md
(no matches)
```

**Nothing is copied.** The page carries **zero** fenced blocks — `rg -n '^```' docs/adapter-reading-order.md`
returns nothing — so `memory.rs`'s full-DCB-loop doctest is cited and never pasted (EC-007), and no
`ignore`-class fence exists to escape the mandatory compile step.

## AC-004 — every section heading carries its position

```console
$ rg -n '^#+ ' docs/adapter-reading-order.md
1:# What to read before you write an adapter
21:## 1 of 6 — the runnable loop, and what it is not
37:## 2 of 6 — the recipe, with a non-`memory` example
44:## 3 of 6 — the same four steps in contributor voice
50:## 4 of 6 — why there are two flavours
58:## 5 of 6 — what removes `Send`, and what removes `Sync`
65:## 6 of 6 — why a provided method is never `async fn`
71:## What this page does not cover
```

Six positioned headings, six markers, one per heading, and the numbering agrees entry-for-entry with the
reading order at `:7-19`. The eighth heading is the terminal region and names no source, so it is not a
section heading — `rg -c '^## [1-6] of 6 — '` returns **6** and `rg -c '^## '` returns **7**. Anti-pattern
17's wrong implementation — a heading that reads as a bare source name — is absent. Falsification: a
reader landing on `#3-of-6` from a search result reads "3 of 6" in the heading they landed on.

## AC-005 — the caveat, in position at entry 1, and paired

The caveat's first sentence is `**And it is not an adapter.**` at `:28`. Section 2's heading is at `:37`,
so the caveat sits **inside entry 1's block and before the first heading of section 2**. It is not a
closing note and not a footnote.

It defers to `memory.rs`'s own three-reasons list (`crates/happenstance-core/src/memory.rs:16-25`) rather
than paraphrasing it — the oracle the conformance suite is validated against, the thing that makes the
crate's examples runnable, the store application code is written against before a real one exists — and
it names the storage-shape axis in the same paragraph: `PgStore` in
`standards/rust/91-adapter-authoring-recipe.md` (`:34`, assigns positions outside the transaction) and
`references/adapter-shapes.md` (`:35`). Two implementations at opposite ends of the axis, in the same
breath, which is what UX-AC-07 and `project.md`'s risk table (final row) ask for.

## AC-006 — cites, never restates

**Clause ids.** Two, both resolving, both checked mechanically: the narrative step parses every
citation-shaped token on the page and fails on one the specification does not define
(`xtask/src/lint_narrative.rs:1197`, `check_citations`).

| Id | Where | Resolves |
| --- | --- | --- |
| `VT-11` | `:32`, the caveat's "one storage shape, and not a required one" | `spec/SPECIFICATION.md:1050` — positions may have gaps |
| `CF-15` | `:42`, entry 2's step 4 | `spec/SPECIFICATION.md:7949` — the fixture is a trait |

`cargo xtask narrative` → `6 pages, all consistent` is that check passing. No clause is restated: neither
sentence carries a `MUST`, and each is the RP-30-2 shape — a short statement of the behaviour with the id
beside it, not a faithful paraphrase of the clause's normative text.

**Decision atoms.** ADR-0001 (`.kb/decisions/0001-async-port-flavours.md`) at `:53` and ADR-0008
(`.kb/decisions/0008-one-derivation-for-both-ports.md`) at `:69`. Both reached through the supersession
graph in `.kb/maps/decision-map.md` (`:68` and `:75`): each row reads `status: accepted`, supersession
column `—`. No superseded atom is cited (EC-008 does not fire).

**Not a third statement.** Section 4 names where the rule lives and says in its own words that the page
states neither the rule nor the decision because those two documents already do (`:55-56`). Section 5
routes to `25-what-removes-send-and-sync.md` and states no bound-removal rule of its own. The precedence
chain at `standards/rust/README.md:25-29` is obeyed and never mentioned, let alone extended.

## AC-007 — exactly one answered need, declared

```console
$ rg -n '^> \*\*Answers:\*\*' docs/adapter-reading-order.md
3:> **Answers:** `explanation` — In what order do I read what already exists, to build an adapter?
```

Verbatim, and singular. Ninety-six characters, at RP-00-2's cap. It sits immediately after the H1 with
nothing interposed, in a blockquote that is rendered in every state and every medium — no fold, no
"About this page" panel.

**Red, before the declaration existed:**

```console
$ cargo xtask lint-pages
  docs/adapter-reading-order.md — no `> **Answers:**` line; see standards/pages/00-one-need.md
xtask failed: 1 problem(s) in standards/pages + docs
```

**Green:** `6 pages, 16 rules, all consistent`. `check_declarations`
(`xtask/src/lint_pages.rs:630`) is what fails on zero, two, malformed or unenumerated declarations, so
"the page carries no second need" is a mechanical claim on this branch rather than a review promise.

**The token, and why it is `explanation` and not `orientation` — the one place this story deviates from
the design's *wording* while keeping its substance.** `_design.md` region 1 states the need as *"in what
order do I read what already exists, to build an adapter"*, which is routing-shaped, and HS-P0021 had not
landed when that was signed off. It has now, and it closes the set at four tokens
(`xtask/src/lint_pages.rs:119-135`). `orientation` is unavailable twice over, and neither reason is a
preference:

1. **RP-10-3 — at most one `orientation` page per directory level**, enforced by
   `check_orientation_ceiling` (`xtask/src/lint_pages.rs:685`). `docs/read-the-worked-example.md:3`
   already holds that slot for `docs/`. A second one is a gate failure, by file and line.
2. **RP-10-2 — an `orientation` page carries links and at most one sentence per destination, and it
   teaches nothing.** `_design.md`'s own density budget for this surface allows ≤ 5 sentences / ~120
   words of connective tissue per source and *requires* the `MemoryEventStore` caveat, which teaches. A
   page written to the signed-off budget is not an `orientation` page under the landed rule.

`explanation` is the honest token: the page's own journey state is Persona 2 at **B3**, "building a model
of what an adapter is shaped like", which is `explanation`'s job as band 10 words it. The *need sentence*
is unchanged — it is the declaration's question, verbatim — so AC-007's substance holds and only the
vocabulary the design could not have known about has moved. Recorded here rather than settled silently.

## AC-008 — volume, measured

```console
$ wc -w docs/adapter-reading-order.md
704 docs/adapter-reading-order.md
```

**704 words** against a ≤ 900 target and a 1,200 hard cap, and above the ~350 floor
(`_design.md` `## Density budget`, "`adapter-reasoning-account`, whole page"). EC-003 does not fire;
nothing was trimmed from the sequence and no citation or caveat yielded.

Per-region, by `awk` over the heading bands:

| Region | Words | Ceiling |
| --- | --- | --- |
| head + reading order | 187 | — |
| 1 of 6 — connective tissue (`:24-26`) | 43 | ~120 |
| 1 of 6 — the caveat (`:28-35`) | 111 | region 3; never yields |
| 2 of 6 | 69 | ~120 |
| 3 of 6 | 54 | ~120 |
| 4 of 6 | 59 | ~120 |
| 5 of 6 | 67 | ~120 |
| 6 of 6 | 52 | ~120 |
| terminal region | 70 | — |

No source's connective tissue exceeds five sentences or ~120 words. The caveat is counted separately and
deliberately: `_design.md` makes it region 3 and states that connective tissue is the only thing that ever
yields — the caveat and the citations never do.

## AC-009 — the terminal region: limits, then exactly one hop

```console
$ rg -n '\]\(' docs/adapter-reading-order.md
77:what it decided on, and that is [why an append re-reads its condition](append-conditions.md).
```

**One outbound link on the whole page**, and it is in the terminal region. Everything else — the six
sources, the two decision atoms, `references/adapter-shapes.md` — is the named-but-unlinked
cross-reference form, so the "one onward hop, and only one" claim is not a count of links in a region but
a count of links on the page.

- **Limits first.** `:73-74` states what the page does not cover: whether this port is right for your
  store, your append-condition strategy, and that it restates none of the six sources.
- **Never a list.** `rg -n "See also|Next steps|Further reading"` returns nothing. Anti-patterns 3 and 5
  are absent.
- **Link text**: "why an append re-reads its condition" — **six words**, a self-describing noun phrase
  naming what the destination answers. Not "here", "this", "docs", "read more".
- **Not a bare URL**: `rg -n "https?://"` returns nothing.
- **The rung, and its guard.** Rung 2 of the href ladder — *an in-tree markdown link inside the pinned
  documentation tree* (`xtask/src/pointers.rs:157-163`, `PointerForm::PinnedTreeMarkdown`). Guarded by
  `check_registration` (`xtask/src/lint_narrative.rs:536`): `docs/append-conditions.md` cannot be deleted
  or renamed without `mod append_conditions` naming no page, or `include_str!` failing to resolve, in the
  mandatory `every narrative page is checked` step. Rung 1 was unavailable (the destination is a page,
  not an item); rung 3 was not needed.
- **No register row, and why.** The register in `xtask/src/pointers.rs` counts pointers installed under
  rule 2 — *a secondary pointer at an item*, on the reference surface, admitted by gates (i)–(iv). This
  hop is neither rule 1's front door nor rule 2's pointer at an item: it is an ordinary in-tree link
  between two pages of one tree, already guarded by that tree's own machinery. Filing a row for every
  in-tree link would spend the cap of 8 (`MAX_ROWS_AT_PROJECT_CLOSE`) on links the register was not built
  to track. NF-005 allows *at most* one row for this story; it takes zero, and says so here rather than
  leaving the absence to be discovered.

## AC-010 — surface hygiene, and B1 independence

**(a) Composed only from the enumerated primitives.**

```console
$ rg -n "<details|<summary|<div|<table|style=|\{\{#tab|admonish|<!-- tab" docs/adapter-reading-order.md
(no matches)
$ rg -n '^```' docs/adapter-reading-order.md
(no matches)
```

The page is a markdown H1, seven `##` headings with no skipped level (`#` → `##`, and no `###`), one
numbered list, one markdown link, and prose. No raw HTML, no inline `style=`, no `<details>`, tab,
accordion or fold. All six sources are visible rather than revealed — this project installs **zero**
revealed or opened-on-demand content, which is `_design.md` `## Transience policy`'s row for exactly this
surface. Nothing animates. Nothing carries meaning in colour or position alone: the reading order's
meaning is carried by its enumeration and by each entry's "why it is at that position" clause, and each
section's position is carried by the words "N of 6" in the heading rather than by where the heading sits.
The one hop is a plain markdown link, so there is no bespoke control to be unreachable by keyboard;
keyboard-only pass recorded as *nothing to reach but links, reached by <kbd>Tab</kbd>*. The *observed*
walk stays `error-site-walk-record`'s, deliberately a different pair of hands.

At 1024x768 nothing this page adds is clipped rather than reflowed: no fixed width, no table, no column
layout, and no fence (so anti-pattern 16's sanctioned horizontal-scrollbar exception — the `error[E0034]`
block — belongs to the slice-mate and appears nowhere here). No colour is authored, so NF-004's theme
independence is met negatively and permanently.

**(b) B1 independence — executed, not asserted.**

```console
$ rg -n "adapter-reading-order" crates/
(no matches)
```

At this story's commit nothing in `crates/` names the page, so deleting it is a no-op for a reader stuck
at `crates/happenstance-core/src/store.rs`. Read with the page absent, `## Import one flavour, not both`
(`store.rs:31-45`) still states the cause, shows the `error[E0034]` block, gives the import rule
("Import only the one you are binding on") and the fully-qualified escape hatch
(`SendEventStore::read(&store, &query, options)`). A reader who never follows any hop is correctly
unstuck. **Re-run after the slice-mate installs the pointer** — that is the version of this check that
matters, and it is recorded in `store-error-site-rewrite/_verification.md`.

## Gate

```console
$ cargo xtask narrative        → 6 pages, all consistent
$ cargo xtask lint-pages       → 6 pages, 16 rules, all consistent
$ cargo xtask affected --base main → affected gate passed
```
