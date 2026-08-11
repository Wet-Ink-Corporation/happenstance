# Anchoring citations in a document whose targets move under it

## Where this expects to land

**Layer: none of the three scaffolded ones.** `product/` and `design/` have no purchase on a
Rust library's tooling. This is a reusable design method with measured tradeoffs, so it
belongs at the **KB root** or in a `playbooks/` layer if one is created.
**Kind: `playbook`.**

`source_paths`: `xtask/src/spec_trace.rs`, `xtask/src/lint_constitution.rs`,
`spec/SPECIFICATION.md`, `standards/rust/README.md`,
`references/evaluation/phase-4-5-reconciliation.md`,
`references/evaluation/review-citation-drift.md`, and this intake file.

## The problem

`spec/SPECIFICATION.md` carries 358 ``file:line`` citations into the source tree.
Line numbers in the source move on every commit. A checker that verifies only that the line
exists passes forever while the citations rot; a checker that pins content exactly fails on
every ordinary edit. Between those two is a narrow band, and finding it took four attempts,
each measured.

## Two properties any solution must have

- **P1 — it must detect a moved target.** A function that moves 400 lines away, with the
  citation left behind, must red the gate.
- **P2 — inserting lines above a cited item must NOT red the gate.** This is the property
  that decides the design. A doc comment growing above a function, an import added at the top
  of a file, a clause inserted into the spec — all of these shift line numbers without
  invalidating any claim. A check that fires on them produces a refresh command that becomes
  a reflex nobody reads, and a reflex that rewrites citations wholesale is strictly worse than
  no check at all, because it launders drift into the document.

**A content hash fails P2 outright** and is therefore not a candidate, however attractive its
precision. So does an exact line match. What satisfies both is a **windowed search for a
short subject string** around the cited range: it survives insertion up to the window width,
and it fails when the item genuinely moves.

## The two spellings of an anchor, and their costs

**Explicit — what `standards/rust/` does.** The Rust constitution writes the anchor into the
citation itself: `` `path:line (anchor)` ``, checked by `parse_citation` /
`check_citations` in `xtask/src/lint_constitution.rs:674-722`, with
`const ANCHOR_SLACK: usize = 10` (`:111`). An unparseable citation is a hard failure, on the
stated ground that "a citation the checker cannot read is one nothing verifies" (`:690-691`).

Its cost is honest and is the reason it was not simply copied: **every one of the 358 sites
would have to be edited to carry an anchor,** and until they were, the check would either be
partial or would fail the whole document. Explicit anchors are the right choice when a corpus
is being authored; they are an expensive retrofit onto one that already exists.

**Derived — what `spec_trace` does.** The specification has one remarkably consistent citation
idiom: a backticked identifier, then the citation, usually parenthesised.

```text
`is_violated_by` compares raw values (`append.rs:239-253`)
```

The anchor is therefore already in the prose, and deriving it costs no change at 358 sites
(`fn subject_before`, `xtask/src/spec_trace.rs:2104-2144`; the rationale is in its doc comment
at `:2083-2103`).

**Neither is universally right.** Explicit costs authoring effort and buys certainty; derived
costs nothing at the sites and buys a coverage ceiling. Which one to use is decided by whether
the corpus already exists.

## The four attempts, and the measured failure rates

**Attempt 1 — walk back up to four backticked spans to find the subject.** This over-reaches:
**118 reports out of 316 anchored.** Applying the two refinements below took it to **70 of
262**, the strict-adjacency rule to **10**, and the absent-subject discriminator to **2** —
and both of those last two were real defects.

Those first two figures were briefly recorded as though they measured the same attempt: the
in-tree doc comment attributed 70/262 to the four-span walk-back, and a draft of this file
called the disagreement unresolved and deferred it. They are two different attempts. Both
comments now carry the whole sequence (`xtask/src/spec_trace.rs`, `subject_before`).

**The transferable part is the shape rather than any of the numbers:** nearly every false
report was one sentence pattern — no backticked subject at all, as in
"the prohibition on arithmetic is already documented at `event.rs:215-217`" — where reaching
back far enough always finds *some* identifier, and it belongs to the previous sentence.

The three refinements, and then the discriminator in the section below:

**Refinement A — take only the span immediately before, and only on the citation's own line or
the one above it.** The document wraps at 80 columns and does not treat a subject and its
citation as unbreakable, so a one-line reach-back is necessary; two is already a guess.
Implemented as `if cite_line.saturating_sub(*subj_line) > 1 { return None; }`
(`xtask/src/spec_trace.rs:2116-2118`).

**Refinement B — decline `.md` targets entirely.** A citation into Markdown is evidence for a
*passage* — an argument, a scenario, a measurement — and the backticked word beside it is
whatever the sentence happened to be discussing, not a definition that lives at that line.
The `Citation::subject` field is documented as `None` for a `.md` target on exactly this
ground, and the doc comment records the cost of not doing it: anchoring them "produced a third
of this check's first run as false reports" (`xtask/src/spec_trace.rs:2073-2080`).

**Refinement C — search the whole cited range, not a window around its start.** A citation like
`404-427` names a span and the subject may be anywhere in it. The first run of the anchor
check searched only around the start line and reported `into_parts` missing from
`event.rs:404-427` "because the doc comment occupies the first fourteen lines of its own
citation" (`xtask/src/spec_trace.rs:2246-2250`). The window is therefore
`[line - (SLACK+1), line_end + SLACK]` (`:354-355`).

**Together with the discriminator below, these took the report count to two — and both of
those were real.** Both were citations that the sweep in commit `89bb966` had just repaired
into the wrong place, which is the best possible evidence that the check earns its keep: it
caught a defect introduced by the very pass that added it.

## The discriminator that made it work

> If the subject appears **nowhere** in the cited file, the derivation picked the wrong word.
> If it appears in the file but **far from the cited line**, that is drift. Only the second is
> worth reporting.

Implemented at `xtask/src/spec_trace.rs:349-352`, with the worked example in the comment
above it: "``limit`` cannot stand in for it because `event.rs:215-217` forbids…" derives
`limit`, which is a `query.rs` name and has no business being looked for in `event.rs`.

This is the load-bearing idea and it generalises past citations. **A heuristic that cannot
tell its own mistakes from the corpus's mistakes must decline, not guess.** The absence test
is cheap, is a property of the derivation rather than of the corpus, and converts an
unreliable heuristic into a reliable one over a smaller domain.

The same commitment shows in the smaller rejections inside `subject_before`, each of which
shrinks coverage to protect precision:

- a span that is not a bare identifier — spaces, generics, an attribute, another citation — is
  declined rather than cleaned up;
- `Type::method` anchors on the last segment, because the definition site spells `fn method`;
- a name shorter than four characters, or one starting with an upper-case letter, is declined.
  **A type name is a poor anchor even when it is the grammatical subject:**
  "``MemoryEventStore`` already behaves that way by accident of ordering (`memory.rs:372-388`)"
  cites the *behaviour*, and the type is declared four hundred lines away
  (`xtask/src/spec_trace.rs:2135-2142`).

## The cost, stated rather than hidden

Of 358 citations checked today, **69 are anchored to their subject**. The other 289 are
verified for addressing only. That ceiling is a direct consequence of declining wherever the
derivation is uncertain, and the summary line prints both numbers so the ceiling cannot be
mistaken for full coverage. The doc comment states the position plainly: "A citation with no
derivable subject is counted and not anchored, which is the honest outcome: this check
tightens the ones it can read and never invents a claim to check."

**Deriving an anchor from prose is only worth it where the derivation is certain.** Where it
is not, the explicit spelling is the answer, and paying its retrofit cost is a real option
rather than a defeat.

## Two observations worth carrying, one of them a defect

**Line-numbered citations across documents are a standing tax.** Every edit to `crates/**` or
`xtask/**` during this pass broke citations in `standards/rust/` — three separate times, each caught
by `cargo xtask lint-constitution`. The check works; the tax is real and should be priced in
before adopting line-numbered cross-references at all. An anchor-only citation with no line
number would not have this cost, at the price of ambiguity when a name occurs many times.

**The two slack constants disagree with each other, and one of them says they do not.**
`xtask/src/spec_trace.rs:381` sets `const ANCHOR_SLACK: usize = 12` and its doc comment at
`:374-380` describes it as "the same twelve `standards/rust`'s own citation lint uses". That lint's
constant is `const ANCHOR_SLACK: usize = 10` (`xtask/src/lint_constitution.rs:111`). The
values are 12 and 10. This is exactly the class of defect the pass was repairing — a sentence
about another file that is no longer true — surviving inside the repair itself, and it is
recorded here rather than fixed because this file is intake and not a patch. Either the
constants should be unified or the sentence corrected; it is a one-line repair and not an ADR.
