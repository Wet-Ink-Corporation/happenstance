# Appending under a condition

> **Answers:** `explanation` — Why does a write re-read what it decided on?
> **Answers:** `how-to` — How do I append under a condition?

**This page is deliberately wrong.** It is the worked example band 40's walk is
calibrated against, and it exists so that a procedure which has never returned
`fail` cannot be mistaken for a procedure that works. It declares two needs,
which `standards/pages/00-one-need.md` forbids, and both declarations are visible
in the head, which is what makes the verdict `fail — two needs` rather than
`fail — need not answered`.

It is deliberately inert. It sits under `examples/`, outside the band namespace,
so a corpus reader taking top-level `NN-slug.md` files never sees it and it can
stay permanently broken without ever turning a gate red. Nothing links to it
except band 40.

## Why a write re-reads

A conditional append states what the writer decided against, and the store
re-reads that condition before it commits. What that buys the writer is the
ability to decide against a world they have actually seen rather than against one
they assumed.

## How to append under a condition

Build the condition from the query the decision was made against and the position
the reader had reached, then hand both to the store with the events. If a
matching event has arrived in between, the append is rejected and the caller
decides again.

---

**What the walk should find.** Step 1 answers `no`: two `> **Answers:**` lines sit
between the H1 and the first paragraph. A second need is visible in the head, so
the verdict is `fail — two needs`. The remedy the verdict names is to split the
page — the explanation and the how-to are two pages, each declaring one need —
and not to delete one declaration and leave the body serving both.
