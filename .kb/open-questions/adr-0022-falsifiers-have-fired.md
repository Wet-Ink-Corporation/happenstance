---
id: kb-open-question-adr-0022-falsifiers-fired-001
title: Two of ADR-0022's falsifiers have fired, a third cannot fire as written, and nobody has re-opened
kind: open_question
status: accepted
authority_tier: note
summary: >-
  ADR-0022 wrote three conditions under which it would be re-opened. The 2026-09-03
  pre-publication review checked all three, two have fired and the third is unfireable as written,
  and one of the three has since been re-opened in part: section 11, by kb-decision-0065 on
  2026-09-21. The other two have not. Section 11 said re-open if any run ever reports busy > 0; busy > 0
  was observed at the shipped CONTENDERS = 64, one launch in seven, the first nonzero busy count
  anywhere in this tree, and the shape matters — busy = 1 and busy = 2, three attempts in 6,720
  across seven launches at that count, with exhausted = 0 on both rows, which is structurally zero on
  a sqlite-default handler row rather than proof that no attempt ran the 5,000 ms budget out, so the
  only defensible claim is the one the falsifier names. Section 9 said re-open if a deployment shows
  the captured tokio Handle costing something the inline path does not; the capture is unconditional and
  irreversible, which makes SqliteEventStoreError::NoRuntime unreachable for any store outliving the
  runtime it was constructed on, so the variant ADR-0022 kept in order to preserve a real meaning has
  lost it by a different route than the one that section anticipated — and the deployment its wording
  asks for still does not exist in this tree, so the twenty-line reproduction is owed before any
  remedy. Section 16's falsifier for section 8 cannot fire as written, because the shape it names as
  the re-open trigger — a GROUP BY with HAVING COUNT(DISTINCT tag) aggregate — is not what the
  adapter emits; measured, the chain that does ship loses to that aggregate in nine of nine two-tag
  cells and is beaten again by a boundary-bound chain, which is the finding the falsifier was written
  to catch and which its own wording excludes. ADR-0022 is accepted and
  immutable, so a fired falsifier cannot amend it: what had to be decided, per firing, was whether it
  is superseded, re-opened with a scoped amendment, or explicitly ratified as still correct with the
  firings recorded against it — and who takes that call. Section 11 took the scoped route and is
  answered; sections 9 and 8/16 are not, and neither has an owner. Forced by phase 12, after which
  the pragma set is a documented property of a published adapter.
depends_on: []
related:
  - kb-decision-0022
  - kb-decision-0065
  - kb-reference-busy-timeout-margin-001
  - kb-reference-busy-timeout-adapter-cap-sweep-001
  - kb-reference-append-condition-experiment-001
  - kb-open-question-es-17-two-adapter-measurement-001
  - kb-open-question-testkit-contention-tolerance-001
  - kb-governance-what-may-refute-a-finding-001
  - kb-reference-shipped-append-condition-sql-001
  - kb-reference-one-connection-latency-001
  - kb-decision-0058
  - kb-open-question-query-plan-parameter-chunking-001
source_paths:
  - .kb/_intake/2026-09-03-pre-publication-review.md
  - .kb/_intake/remediation-2026-09-04-briefs/append-condition-sql-shape.md
  - .kb/_intake/remediation-2026-09-04-briefs/transient-contention-tolerance.md
  - .kb/_intake/remediation-2026-09-04-briefs/sqlite-blocking-seam.md
  - .kb/_intake/2026-09-21-adr-0022-s11-superseded-busy-timeout-is-fifteen-seconds.md
  - references/evaluation/review-pre-publication-2026-09-03.md
  - experiments/busy-timeout-margin/
  - experiments/busy-timeout-margin/results/adapter-cap-sweep.md
  - experiments/shipped-append-condition-sql/
  - references/adr/0022-append-condition-strategy.md
  - crates/happenstance-sqlite/src/event_store.rs
  - crates/happenstance-sqlite/src/connection.rs
last_reviewed: 2026-09-21
---

# Two of ADR-0022's falsifiers have fired, a third cannot fire as written, and nobody has re-opened

## What is true today

ADR-0022 (`kb-decision-0022`) closes with a section (`references/adr/0022-append-condition-strategy.md:589`)
naming, for each of its verdicts, the condition under which a later reader should treat it as
overturned rather than affirmed — the discipline this repository already applies to ES-17
(`kb-open-question-es-17-two-adapter-measurement-001`). The 2026-09-03 pre-publication review
checked every one of them against the two experiments it ran, `busy-timeout-margin` and
`shipped-append-condition-sql`, and found three results, not zero.

**Section 11 fired.** The pragma section's falsifier reads: *"Re-open the busy timeout if any run
ever reports `busy > 0`, which would mean five seconds stopped being generous"* (`0022:614-616`).
At the shipped `CONTENDERS = 64`, launched seven times under SQLite's own busy handler, one launch
produced `busy > 0` — the first nonzero busy count anywhere in this tree. Counted from the raw rather
than a summary: each launch is 3 rules × 5 rounds × 64 = 960 attempts, so the incident is **three
attempts in 6,720**, 0.045%, all inside one launch, and of the 44 rows at `contenders=64` in
`results/raw/` exactly two carry a nonzero `busy`.

The shape of the firing matters, and **both readings that have been offered of it over-read the
counters, in opposite directions.** The summary page's *"three contenders across two rules exhausted
the full 5,000 ms"* is wrong: `exhausted` reads `0` on both rows. But the replacement inference —
`exhausted=0` therefore *no attempt ran the budget out* — is also more than the field can carry, and
this atom asserted it until 2026-09-07. `exhausted` is incremented only inside the experiment's own
counting handler (`experiments/busy-timeout-margin/src/busy.rs:262`), and these rows carry
`busy_handler=sqlite-default`, which installs SQLite's handler instead (`src/busy.rs:100-104`): on
such a row `exhausted` is **structurally zero**, as are `wait_ms_max`, `retries` and `lock_events`.
The defensible sentence is the one the falsifier itself names — **`busy > 0` at the shipped
`CONTENDERS = 64`** — and the two rows' `race_us_max`, both just past the budget, corroborate
exhaustion without proving it. Nothing that follows turns on which it was. The margin claim
(1.31x–1.38x on the plateau, every `wait_ms` figure a lower bound) is unaffected; what broke is the
premise the falsifier was testing, not the number it was protecting.

**Section 9 fired, by a different route than it anticipated.** Its falsifier: *"Re-open it if a
deployment shows the captured `Handle` costing something the inline path does not"* (`0022:609-610`).
The captured `Handle` is unconditional and irreversible — no constructor, setter or builder can
clear or replace it — so a store built inside one Tokio runtime and served from another gets
`append` and `head` calls that run inline and succeed, and every `read` call that hangs or yields
one cancelled-task item and terminates. `SqliteEventStoreError::NoRuntime`, the variant ADR-0022
kept specifically because this design choice preserved its meaning, is unreachable for such a store.
The cost is real, as the falsifier anticipated; it is a correctness gap rather than the performance
delta the falsifier's wording pictured.

**But the falsifier says *"a deployment shows"*, and no deployment has.** Nothing in `experiments/`
builds a store inside `Runtime::new().block_on(…)`, drops the runtime, and observes `read`; the
`Worker(task was cancelled)` outcome above is reasoned from tokio's semantics rather than reproduced
in this tree. This repository's own bar — name a plausible wrong implementation and write it — puts
the reproduction before the remedy, and it is roughly twenty lines. Until it exists this firing is a
hazard rather than a fact, and an escape hatch built on it — an explicit-`Handle` constructor, or one
that declines to capture — risks being a door onto a room nobody has entered. What does *not* fire
this falsifier: the write path's inline blocking seam widens section 9's **scope** rather than
disputing its **verdict**, and is settled separately in `kb-decision-0058`.

**Section 16's own falsifier for section 8 cannot fire as written.** It reads: *"Re-open it if a
future SQLite pushes predicates through an aggregate"* (`0022:606-607`) — naming a future compiler
change as the trigger. But the aggregate in question, `GROUP BY position HAVING COUNT(DISTINCT tag)
= n`, is not a future capability; it is the form every one of ADR-0022's own published figures was
measured against, and the adapter that ships emits a different shape, a correlated intersection
chain, instead. Measured against the aggregate for the first time, the shipped chain loses in nine
of nine two-tag cells, at 1.54x–1.86x warm and 11.8x cold, and it is beaten again — six cells worse,
three inside band — by a fourth arm that binds the boundary into every chained subquery. The shipped
shape is dominated by both alternatives everywhere it was measured; the figures and their caveats are
in `kb-reference-shipped-append-condition-sql-001`. The condition the falsifier was written to detect
has already happened, on the code that shipped — its wording just points at the wrong mechanism to
notice it.

Two consequences follow, and neither is settled here. Section 8 item 1 makes
most-selective-tag-first probing a **requirement** and `tag_cardinality` a requirement with it; on
the shape that ships that ordering is measured 38.1x–44.0x *backwards* across two runs, because the
chained subquery is uncorrelated and the shipped sort materialises the larger set. And ES-27's
`Rejects:` prose (`spec/SPECIFICATION.md:3902-3905`) quotes the aggregate's "roughly 200x" to
justify the chain — repairing that number is a **`[FROZEN]` clause edit**, which CLAUDE.md routes
through a new decision rather than a documentation sweep, and the honest repair states the
order-of-magnitude gap rather than pinning another warm-cache one-host multiple that will rot the
same way.

## What is not decided

ADR-0022 is `accepted` and immutable under KB authority rule 1: a fired falsifier is evidence, not
license to edit the decision's body, and recording "falsifier fired" as a status line inside it
would be exactly that edit, made worse — a later reader could no longer tell whether the decision
was signed with that knowledge or acquired it afterward. What is not decided is which of three
moves resolves it: supersede ADR-0022 with a new decision atom carrying `supersedes: [kb-decision-0022]`;
re-open it with a scoped amendment addressing only the pragma and runtime-seam sections; or ratify
it explicitly as still correct, with the firings recorded from outside as accepted costs — the third
is defensible, since section 11 fired with a broken premise rather than a broken margin (`busy > 0`,
with `exhausted` uninformative on those rows). A fourth reading has since been proposed for section 8
specifically: supersede it **in part**, items 1 and 2 only, leaving sections 4, 6, 7, 9, 10, 11 and 15
untouched — which is narrower than superseding ADR-0022 and wider than ratifying it, and shows that
"which of three moves" was itself too coarse a question. Nobody has taken that call, and the routing
question — who owns the
testkit-facing contention-tolerance question section 11's firing raises, versus who owns section 9's
runtime-seam correction — is likewise open.

## What forces it

Phase 12, when the pragma set (`synchronous`, the busy timeout, the runtime-seam choice) becomes a
documented property of a published adapter rather than an internal decision this workspace can
revisit freely. Superseding, re-opening or ratifying ADR-0022 before then is materially cheaper than
after: a downstream crate that has pinned against the current behavior is not yet a stakeholder.

## Partly answered 2026-09-21 — §11 is decided by the fourth move; §9 and §8/§16 stand

**One of the three findings is now settled, and by the move this atom itself proposed.** The
paragraph above offers a fourth reading — *supersede it in part* — and offers it for section 8,
listing section 11 among the sections it would leave untouched. `kb-decision-0065` applies exactly
that move to section 11 instead: it supersedes ADR-0022 at section 11 only, says in terms that
sections 4, 6, 7, 9, 10, 12 and 15 stand, and therefore carries `supersedes: null` while
`kb-decision-0022` stays `accepted` — the same shape `kb-decision-0007` used against ADR-0006. The
irony is worth leaving on the record rather than tidying away: the sentence that found the right
instrument pointed it at the wrong section, because it was written about section 8.

`crates/happenstance-sqlite/src/connection.rs:112`'s `BUSY_TIMEOUT_MS` is now `15_000`, decided on a
measurement of *this adapter's own* concurrency target rather than the experiment's candidate —
`kb-reference-busy-timeout-adapter-cap-sweep-001`, which closes the first caveat the margin page
still carries. At `--test-threads=1`, 5,000 ms went red in 7 launches of 8 and 15,000 ms in 0 of 16.
Note what that does to the *finding* recorded above: nothing. The firing stands exactly as written,
`exhausted` reading included — what changed is that the falsifier has been answered rather than
re-argued, and the replacement falsifier now names an instrument that can fire it, which the
original never had for as long as it existed.

**The routing question is half answered.** This atom asked who owns the testkit-facing
contention-tolerance question section 11's firing raises, versus who owns section 9's runtime-seam
correction. The first half has an owner: `kb-open-question-testkit-contention-tolerance-001`, and
the instrument it was blocked on now exists — `FaultyStore::contend_next`
(`crates/happenstance-testkit/src/faulty.rs:333`), with
`crates/happenstance-testkit/tests/contended_store_instruments.rs` showing both affected rules
reject a merely-contended store. Raising the cap lowers the rate at which a contended store is
mistaken for a broken one; it cannot remove it, because CF-33 is `[FROZEN]` and denies a rule the
clock that would tell them apart. The second half is still unowned.

**Section 9 stands, and it outranks section 11 now that section 11 is settled.** It is a
correctness gap — a store outliving the runtime it captured — rather than a liveness cap, and it is
still owed the twenty-line reproduction its own falsifier's *"a deployment shows"* wording demands.
Nothing in this wave built that deployment, and the ordering above holds: reproduction before
remedy.

**Sections 8 and 16 stand, and the repair is a decision rather than a documentation sweep.** The
falsifier still cannot fire as written, and the repair path still runs through a `[FROZEN]` clause:
ES-27's `Rejects:` prose quoting the aggregate's "roughly 200x". That citation has drifted — it now
reads at `spec/SPECIFICATION.md:4061-4066`, with the figure itself on `:4065`, not the
`:3902-3905` the paragraph above cites. Repoint by the anchor rather than the offset, which moves
every wave.

So this atom is **amended, not withdrawn**: `status` stays `accepted`, because two of its three
findings are exactly as open as they were. Its title's final clause — *"and nobody has re-opened"* —
is now false for one of the three, and is left standing on purpose under this layer's own rule that
a question is not rewritten into its own answer. A reader who wants the current state reads this
section; a reader who wants what was known on 2026-09-07 reads the ones above it.
