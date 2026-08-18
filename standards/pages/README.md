# Page standards

What makes a narrative page teach, as rules a page author and a non-author
reviewer can both apply. Five bands, a handful of rules each; every rule names a
wrong page that could ship. The instruction is the same one the sibling tree
gives: **load one rule, never the tree** — find yours in the band table, or guess
the filename from the band, because the numbering is stable.

| Band | Owns |
|---|---|
| `00` | one need per page, and the declaration's grammar and position |
| `10` | the closed need set, and the two `orientation` ceilings |
| `20` | the fold line — what may never sit behind a fold, tab or panel |
| `30` | citations — a page cites a clause id and never restates it |
| `40` | reviewing a page — the non-author walk, and the paraphrase spot check |

## Precedence

> These rules sit at the **constitution-atom tier** of the existing chain —
> **SPECIFICATION clause > ADR > constitution atom > `CLAUDE.md` /
> `CONTRIBUTING.md` summary > `references/evaluation/*`** — alongside
> [`standards/rust/`](../rust/README.md), on the same rank, scoped to a
> different subject. This tree adds no tier, and the chain's own statement is
> cited from here and never edited.

So a page rule loses to a `spec/SPECIFICATION.md` clause and to an ADR, and beats
a summary in `CLAUDE.md`. Where a rule here and a clause disagree, the clause
wins and the rule is wrong — which is why band `30` requires a page to cite a
clause rather than restate it.

## Start here

| You are… | Load |
|---|---|
| writing any narrative page | `00`, then `10` |
| unsure which need a page answers | `10` |
| writing an index, a landing page or a table of contents | `10` |
| about to put something behind a fold, a tab or a collapsed panel | `20` |
| writing a sentence that sounds normative | `30` |
| reviewing a page you did not write | `40`, then `00` |
| adding or editing a rule in this tree | `00`, then this router |

## Index

Generated from the atoms and checked for equality by the step that reads this
tree. The equality is the check; the generator is not.

<!-- BEGIN GENERATED -->
| Atom | Load when | Rules |
|---|---|---|
| [`00-one-need.md`](00-one-need.md) | starting a narrative page · a page that seems to answer two questions | RP-00-1, RP-00-2, RP-00-3 |
| [`10-the-need-set.md`](10-the-need-set.md) | choosing which need a page declares · a page that fits no token | RP-10-1, RP-10-2, RP-10-3, RP-10-4 |
| [`20-the-fold-line.md`](20-the-fold-line.md) | about to collapse part of a page · reviewing a page that hides a claim | RP-20-1, RP-20-2, RP-20-3, RP-20-4 |
| [`30-citing-the-specification.md`](30-citing-the-specification.md) | writing a sentence that sounds normative · reaching for a `MUST` | RP-30-1, RP-30-2, RP-30-3 |
| [`40-reviewing-a-page.md`](40-reviewing-a-page.md) | reviewing a page you did not write · sweeping a set for paraphrase | RP-40-1, RP-40-2 |
<!-- END GENERATED -->

## The shape of a rule

An atom opens `# NN — Title`, then `> **Load when:** …`, then
`> **See also:** …` naming sibling bands by number rather than by link, then a
`---`. Each rule is `## RP-<band>-<n>.` followed by an imperative sentence and
five sections in a fixed order: **Why.** (the mechanism, at most three
sentences) · **Do** (a page fragment) · **Not** (the wrong page) · **Rejects.**
(a wrong page that could ship, and who finds out when) · **Evidence.**
(repository `path:line` first, then the discovery dossier, then external URLs
with a `*(checked …)*` stamp).

`Load when:` occupies **one source line**. The generator that builds the index
above reads only the first line of that block and drops any continuation
silently, which is why the sibling tree's index carries trigger cells that stop
mid-phrase. Six rules and 16,384 bytes per atom, as next door; this router is
capped at half an atom, 8,192 bytes, because it is the one file every page author
loads and a router at atom scale is a corpus. Fences here are tagged `text` or
`markdown` — never `rust` and never untagged, because nothing in the workspace
compiles this tree.

## What checks this tree, and what does not

No **dedicated** gate step reads this tree yet: the `lint-pages` step, and the
corpus reader that walks this directory rather than a list of filenames, are
`page-need-checker-mounted-in-the-gate`'s. What already reads it is the gate's
mandatory `tests` step — `cargo test --locked --workspace --all-features`, which
includes `xtask`, whose `xtask/src/lint_pages.rs` names every atom here and reads
it. A dangling link in the index, a rule missing its `**Rejects.**`, a prose line
past 96 columns, a `rust`-tagged fence, a router over 8,192 bytes and a token
that disagrees with the `NEEDS` constant all turn `cargo xtask ci` red today.

The gap the checker closes is *which* files are read: the list of atoms is
hand-written in that module, so an atom nobody adds to it is read by nothing and
an empty tree would pass. A test module reading five named files is not a corpus
reader, and only the second makes a green run a statement about the tree.

After it lands, one thing still will not be checked, and it is the one that
matters most: a check can see that a page **declares** a need, never that the
page **answers** it. Band `40`'s non-author walk is the instrument for that, and
band `30`'s spot check is the instrument for a page that cites a clause id
correctly and restates its content in the paragraph underneath.
