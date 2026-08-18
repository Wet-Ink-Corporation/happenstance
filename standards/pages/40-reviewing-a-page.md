# 40 — Reviewing a page

> **Load when:** reviewing a page you did not write · sweeping a set for paraphrase

> **See also:** 00 (the declaration) · 10 (the need set) · 30 (citations)

---

Two procedures a byte count cannot replace. The first reaches a verdict on one
page; the second sweeps a set for the failure band 30 forbids and nothing
mechanical sees. Both are performed by a person, and both end in something
written down.

## RP-40-1. Reach a verdict on a page you did not write, from the page alone.

**Why.** The bar is a check a reviewer can actually perform rather than one that
depends on the author's memory. A walk whose steps are questions has that
property; an impression does not, and two readers holding impressions disagree
without either being wrong.

**Do** The walk is performed by a reader who is **not the author**. They may
consult the rendered page, this tree's router, and `spec/SPECIFICATION.md`. They
may **not** consult the author, and they may not read the page's git history: a
verdict that needs either is a verdict about the author rather than about the
page. The worked example this walk is calibrated against is
[a specimen carrying two needs](examples/two-needs.md).

### The walk

1. Does exactly one `> **Answers:**` line sit between the H1 and the first paragraph?
2. Is its token one of band 10's four, spelled exactly?
3. Does every section of the page serve the question that declaration names?
4. Is every load-bearing claim visible with nothing opened?
5. Does every normative sentence cite a clause id rather than state the rule itself?
6. Could a stranger name the page's need from the head alone, without the body?

A `no` at step 1 or 2 is `fail — two needs` when a second need is visible in the
head, and `fail — need not answered` otherwise. A `no` at step 3 or 6 is
`fail — need not answered`. A `no` at step 4 is a band 20 failure and is recorded
as `fail — need not answered`, because a claim nobody sees is a need nobody met;
a `no` at step 5 is a band 30 failure and is recorded the same way. A step that
cannot be answered from the page alone ends the walk at `indeterminate`.

| Verdict | It means | What happens next |
|---|---|---|
| `pass` | every step answered as the walk requires | record the page, the walker and the date |
| `fail — two needs` | the page answers more than one question | split the page, one need each |
| `fail — need not answered` | the declared need is not the one the body serves | rewrite the body, or change the declaration and split what is left |
| `indeterminate` | the walk could not be completed from the page alone | a defect in the page, not in the procedure; record it as one and the page is rewritten |

When the governed set is empty, the sweep is recorded as **vacuous** — in that
word, and never as a pass. An empty walk proves nothing, and a record that reads
green for a check that never ran is a failure this repository has already paid
for once.

**Not** Reading the page and writing "looks fine". That is an impression, it is
unfalsifiable, and it flatters whichever author asked for it.

**Rejects.** A review that reaches "looks fine" without executing the walk. It
costs nothing, it satisfies every process that counts reviews rather than
verdicts, and the page it approved fails the first reader who arrives with the
second question — by which time the review is old enough that nobody rereads it.

**Evidence.** `standards/pages/00-one-need.md:14` (the rule step 1 tests) ·
`standards/rust/README.md:99-107` (`Do` / `Not` / `Rejects.` — a shape that
forces a named wrong state rather than an impression)

## RP-40-2. Spot-check the set for paraphrase, sentence by sentence.

**Why.** Band 30's rule is the one nothing mechanical reaches: a page can cite a
resolving clause id and restate its content in the next paragraph. Only a reader
comparing the sentence to the clause sees that, so the sweep is written down as a
procedure rather than assumed.

**Do** Walk the corpus file by file — `standards/pages/*.md` today, and every
governed page once one exists. For each sentence that states a rule, ask whether
its authority is a **visible, resolving clause id** or a **restatement** of clause
content. Record one verdict per file, with the number of normative sentences
examined, so a later reader can tell a thorough sweep from a glance.

**Not** Recording a verdict for "the set". A per-set verdict hides which file was
read carefully and which was skipped, and it cannot be resumed by a second person
on a second day.

**Rejects.** A sweep recorded as "no paraphrase found" with no file list and no
sentence count. It is indistinguishable from a sweep nobody ran, it is the exact
shape of the decorative record this repository has already shipped once, and the
paraphrase it missed surfaces when the clause it copied is amended.

**Evidence.** `standards/pages/30-citing-the-specification.md:9` (the blind spot
this procedure is the instrument for) · `spec/SPECIFICATION.md:280` (why a cited
id survives an amendment and a paraphrase does not)
