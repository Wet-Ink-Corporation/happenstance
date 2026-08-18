# 10 — The need set

> **Load when:** choosing which need a page declares · a page that fits no token

> **See also:** 00 (the declaration) · 20 (the fold line) · 40 (the reviewer's walk)

---

Four tokens, and the set is closed. A page declares one of them, spelled exactly
as it is spelled here.

| Token | The page's job | Success for the reader |
| --- | --- | --- |
| `orientation` | route the reader to the page that answers their question | they leave, correctly, within one screen |
| `tutorial` | carry a newcomer through one working thing, staged | they ran it and it worked |
| `how-to` | get a reader who already has a goal to that goal | the goal is achieved |
| `explanation` | build the mental model behind a behaviour | they can predict what the API does before running it |

The same four tokens are enumerated once in Rust, at `xtask/src/lint_pages.rs`,
so this table and the membership test cannot disagree by accident. The third
column is prose and is deliberately not a field of that constant: both fields
that exist have a machine that reads them, and a third that nothing reads would
be dead code wearing a doc comment.

**`reference` is not a member, and its absence is a subtraction with a cause.**
This workspace already has two authoritative reference surfaces — rustdoc, and
`spec/SPECIFICATION.md`, whose clauses a page cites and never restates (band 30).
A `reference` bucket inside a narrative tree is therefore either permanently empty
or it becomes a second specification, which is the outcome this discipline exists
to refuse.

**`orientation` is an addition, and its cause is a measured gap.** The taxonomy
this set starts from is silent about routing, and the gap this repository
actually measured is an evaluator who found the material good until the second
question and then had nowhere to go. Left implicit, a landing page is filed under
some other token and is then flagged by RP-00-1 for answering a second need.
RP-10-2 and RP-10-3 are what stop the new category from becoming a sink.

**The three shapes that lost.** Adopting the four Diátaxis categories literally
lost because the evidence names this project's own kind of subject — a dense,
interrelated conceptual model — as where the four-box split strains, and it would
have produced the empty `reference` bucket above. Keeping the discipline and
dropping the enumeration lost because a check cannot test membership in
an open set, which reduces the whole rule to a convention. A persona-keyed
taxonomy lost because a need is a property of the page and a persona is a
property of the reader: an adapter author reads explanations and how-tos, so
almost every page would declare two.

## RP-10-1. Declare a token from the set above, spelled exactly.

**Why.** The token is compared exactly: no trimming, no case folding, no aliasing
and no plural form. `Explanation`, `how_to` and `explanation ` are all not
members, because a set that can only be nearly matched is a set nothing enforces.

**Do**

```text
> **Answers:** `how-to` — How do I append under a condition?
```

**Not**

```text
> **Answers:** `reference` — What fields does an append condition carry?
```

**Rejects.** A page declaring `reference`, or `Explanation`, or `guide`. It reads
as conformant to anyone skimming the head, and the author learns otherwise only
when the token meets the set — by which time the page has been written to a shape
the set does not have, so the remedy is a rewrite rather than a rename.

**Evidence.** `xtask/src/lint_pages.rs:85` (the one enumeration, and the doc
comment naming everything that must agree with it) ·
`standards/rust/README.md:99-107` (the atom shape this is written in) ·
`.bklg/docs-that-teach/page-need-discipline/_design.md` (S1 vocabulary — DT-2)

## RP-10-2. Keep an `orientation` page to links and one sentence each.

**Why.** `orientation` is the token this set adds to its source taxonomy, and a
routing category with no ceiling is exactly the open-ended sink the discipline
exists to refuse. The ceiling is what stops it becoming one.

**Do**

```text
# Where to start

> **Answers:** `orientation` — Which page answers my question?

- [Append conditions](append-conditions.md) — why a write re-reads what it
  decided on.
- [Text fences](text-fences.md) — how a fence the compiler never sees stays
  honest.
```

An `orientation` page carries links, and at most one sentence per destination
saying what that destination answers. It teaches nothing.

**Not**

```text
# Where to start

> **Answers:** `orientation` — Which page answers my question?

An append condition is a query plus a position. The store re-reads the query
before it commits, because the decision the writer made was made against what
it read …
```

The moment it explains, instructs or references, it is answering a second need
and must be split.

**Rejects.** An index page that grew three paragraphs of background so a reader
would not have to click. It is the friendliest possible version of the failure:
it reads well, it still declares one token so RP-00-1 is satisfied, and it is
caught only when a reviewer asks what a reader wanting that background would have
searched for and finds it filed under routing.

**Evidence.** `standards/rust/README.md:45-58` (a filter that routes and teaches
nothing) · `.bklg/docs-that-teach/page-need-discipline/_design.md` (S1
vocabulary — DT-3) ·
`.bklg/docs-that-teach/_discovery/distillation/interaction-patterns.md` (tension
7, routing)

## RP-10-3. Carry at most one `orientation` page per directory level.

**Why.** Two routing pages at one directory level are two answers to the same
question, and the reader who lands on the second has no way to learn that the
first exists.

**Do** One `orientation` page per directory level and no more. A second one is
merged into the first, or the level is split into two directories so that each
routing page owns one.

**Not** A `README.md` and an `index.md` at the same directory level, each
declaring `orientation` and each listing a different half of the tree.

**Rejects.** A tree whose routing pages breed one per subject rather than one per
level. No single page is wrong; the reader is simply routed into whichever half
of the tree the page they landed on happens to cover, and the half they wanted is
reachable only from the page they did not open.

**Evidence.** `standards/rust/README.md:61-97` (one complete index per tree,
generated so it cannot fall behind) ·
`.bklg/docs-that-teach/page-need-discipline/_design.md` (S1 vocabulary — DT-3,
RP-10-3)

## RP-10-4. Change the set in one commit, never in two.

**Why.** The tokens exist twice on purpose — once as a table a human argues with,
once as a constant a machine compares against — and the only thing keeping two
lists in agreement is that they move together.

**Do** Changing the set is a **two-part commit**: the `NEEDS` constant in
`xtask/src/lint_pages.rs` and the table in this atom, in the same change, with
the rule that justifies the new member written before anything declares it.

**Not** A one-part commit that adds a token to the constant and leaves the table
alone, or the reverse. A one-part commit is precisely the half-landed state a
single enumeration exists to foreclose.

**Rejects.** A seventh token added to the constant because some page strained
against all six. The set is closed at six by an assertion the compiler checks, so
the straining page is answering more than one need and the remedy is to split it
— a contributor who reads that ceiling as an obstacle rather than as a diagnosis
spends an afternoon on the wrong repair.

**Evidence.** `xtask/src/lint_pages.rs:110` (the ceiling, checked at
`cargo check` rather than at `cargo test`) · `standards/rust/README.md:99-107`
(the atom shape) · `.bklg/docs-that-teach/page-need-discipline/_design.md` (DT-2,
"Failure mode and mitigation")
