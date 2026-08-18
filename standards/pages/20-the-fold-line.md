# 20 — The fold line

> **Load when:** about to collapse part of a page · reviewing a page that hides a claim

> **See also:** 00 (the declaration) · 30 (citations) · 40 (the reviewer's walk)

---

This band has a spirit and a letter, and it needs both. The spirit is a test a
reviewer applies to any collapsed region; the letter is a closed list of classes
that never reach the test at all, because a rule that is only a test is a rule
that drifts at the edges. This atom carries no fence and no collapsed region of
its own: an atom writing the rule against hiding may not contain the markup it
forbids.

## RP-20-1. Apply the deletion test to every collapsed region.

**Why.** A reader who never opens a disclosure has read the page without what is
inside it. The question that settles whether that matters has one yes/no answer
and needs nothing from the author.

**Do** Ask: *if the collapsed region were deleted, would the page still teach the
constraint correctly?* If the answer is yes, the region is an aside and is
eligible — go on to RP-20-3. If the answer is no, it may not be collapsed, and no
further discussion is owed.

**Not** Weighing how likely a reader is to open it, how short the page would be
without it, or how tidy the collapsed version looks. None of those is the
question, and each of them is a way of answering "no" and shipping anyway.

**Rejects.** A tutorial whose one runnable command sits inside a collapsed
"Setup" block because the page read long without it. Every reader who follows the
page top to bottom without opening the block reaches step two with nothing
installed, and the author finds out from the third person to report that the
tutorial does not work.

**Evidence.** `standards/pages/00-one-need.md:101` (band 00's never-occlusion
rule, whose mechanism question this band answers) ·
`.bklg/docs-that-teach/_discovery/distillation/interaction-patterns.md:404-410`
(the deletion test, verbatim from NN/g)

## RP-20-2. Never fold one of the five classes, and grant no exception.

**Why.** The deletion test is a judgement, and a judgement is right at the edges
until it drifts — which is how one statement of an invariant here stayed visible
while its folded twin went stale. The letter of the rule is what stops that.

**Do** Five classes may never sit behind a fold, a tab, an inactive panel or a
disclosure element, and **no reviewer may grant an exception**:

1. the page's own `> **Answers:**` declaration;
2. any sentence carrying a normative modal, or citing a `spec/SPECIFICATION.md` id;
3. any statement of an invariant, a constraint or a precondition;
4. the only occurrence of a code fence the reader is expected to run;
5. any statement of what a check does *not* verify.

The list is **closed**. An edge case grows the mechanism table in RP-20-3; it
never shrinks this list. Growing the list means disagreeing with the design's
sign-off condition 3, which is a decision a human reopens rather than an
authoring choice.

This rule binds **the author's own markup**. A wrapper the renderer supplies and
opens by default — rustdoc puts every page inside a `toggle top-doc` disclosure
and ships a `#toggle-all-docs` control that closes it — is not an authored fold,
so no page is in breach because of it. It is also not *fine*: nobody here has
watched what survives that control being used, so it is recorded as an **unmet
property** owed by the hosting decision. Both sentences ship; either alone is
wrong.

**Not** A page whose "what this does not verify" paragraph sits inside a
collapsed panel labelled *Limitations*, or whose one `MUST` is a bullet inside an
inactive tab.

**Rejects.** A checker's page whose limits are tidied into a collapsed
*Limitations* block. It reads as tidy housekeeping, it passes every review
that looks at the open page, and the reader who most needed it — the one treating
a green run as a guarantee — is exactly the reader who never opened it. They find
out when the thing the check never looked at ships.

**Evidence.** `standards/rust/81-checks-that-cannot-be-types.md:11` (a check whose
limits are undocumented is read as a guarantee — why class 5 exists) ·
`spec/SPECIFICATION.md:280` (clause ids are stable, so class 2 can be written
about ids) ·
`.bklg/docs-that-teach/_discovery/distillation/interaction-patterns.md:484-497`
(the drift this letter refuses)

## RP-20-3. Fold only with a mechanism on the permitted table, which is empty.

**Why.** Eligibility is not permission. Content that clears RP-20-1 and RP-20-2
may still only be folded by a mechanism this repository has watched behave,
because an unverified property counts as unmet rather than as probably fine.

**Do** Check the mechanism against `PERMITTED_FOLD_MECHANISMS` before folding
anything. It ships with **zero rows**, so folding is **forbidden in practice**:

| Mechanism | Accessibility tree | Keyboard | Ctrl-F | Print | Observed at |
|---|---|---|---|---|---|

A mechanism reaches that table only with a recorded observation **in this
repository** that its content (i) sits correctly in the accessibility tree,
(ii) is keyboard operable, (iii) is found by Ctrl-F and (iv) is found by print.
Four observations, all four recorded here — and upstream documentation is not an
observation, because a project's own claim about its panels is an unverified
property rather than a confirmed safe one. The first row is HS-P0020's **DT-7**
demonstration to earn.

**Not** Adding a row because a tool's own documentation says its panels are
accessible, or because the pattern is common, or because the content is short.

**Rejects.** A first row added on the strength of upstream marketing copy. The
table then reads as four checks that were performed, the next author folds
against it in good faith, and the gap surfaces when a reader using a screen
reader, a printer or Ctrl-F cannot find the content — by which time several pages
have been written against a permission nobody earned.

**Evidence.** `RUNBOOK.md:923-926` (a step that always skipped, with two
documents vouching for it) ·
`.bklg/docs-that-teach/_discovery/distillation/interaction-patterns.md:205-216`
(*"an unverified property, not a confirmed safe one"*)

## RP-20-4. Grow the mechanism table with evidence; never shrink the class list.

**Why.** The two lists have opposite openness on purpose. An edge case the letter
gets wrong is answered by earning a mechanism in public, with the four
observations attached, and never by a one-off exception granted at review.

**Do** Take an edge case to RP-20-3's table, in the open, with evidence. If the
mechanism cannot earn a row, the content is not folded — and that outcome is the
rule working rather than the rule failing.

**Not** Recording an exception in a review comment, a commit message or an
author's memory. An exception granted once is an exception the next page cites,
and by then nobody can say what it was granted for.

**Rejects.** A reviewer who lets one long output transcript be collapsed "just
this once" because it is genuinely an aside. The next page cites that review, the
one after cites the next, and the class list is hollowed out one reasonable
decision at a time — which is exactly how the invariant this band exists for
drifted in the first place.

**Evidence.** `standards/pages/README.md:17` (the tree's rank: a rule here loses
to a clause, so a rule here may not be waived to fit one) ·
`.bklg/docs-that-teach/_discovery/distillation/interaction-patterns.md:484-497`
(the tension this asymmetry settles)

## What this rule does not do

- **No gate step reads this rule.** It is applied by a reader. Nothing in
  `cargo xtask ci` counts folds, and nothing will: the markers are textual and
  the judgement is not.
- `PERMITTED_FOLD_MECHANISMS` is a table in this tree and **not a `const`** in
  `xtask/src/lint_pages.rs`. Nothing would enforce such a constant, and an
  unenforced one beside an enforced one reads as a check that exists.
- Whether a hidden branch sits inside the checked surface at all is HS-P0020's
  **DT-7** demonstration. It is not answered here, and this band is where its
  result would be recorded if it succeeds.
- A page can satisfy all five classes and still teach badly. That is **band 40**'s
  walk, and the comprehension evidence HS-P0024 owes.
