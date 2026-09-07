# At VT-23's floor the windowed arm stops dominating

> **Superseded by [`merge-join.md`](merge-join.md).** A fourth shape — the arms
> merged as a compound with the budget on the compound rather than on each arm,
> which is SQLite's own co-routine merge — wins every cell in both this table
> and the wide one. The pages below are kept because they are how that shape was
> found, and because each records a claim that had to be run to be refuted.

Written by hand from the `raw/wide-windowed-arms.txt` recorded at commit
`58ec849`. That file has since been **regenerated** by the same test with a
fourth shape added, so the working copy carries a later run than the three
columns below; this page's run is in git, and the current one is in
[`merge-join.md`](merge-join.md). One
500,000-event store over `Corpus::ManyBuckets`, **128 items per query** — VT-23's
floor — one 512-row page per statement, 5 rounds per cell with the leading shape
rotated, full-row projection. The returned page is compared column-by-column
across all three shapes every round before any time is recorded; it held in all
eight cells.

`MAX_QUERY_ARMS_PER_STATEMENT` is 400, so 128 items is **one** statement. This
measures arm count, not the chunk-and-merge loop.

## The headline, stated against the page it corrects

[`windowed-arms.md`](windowed-arms.md) concluded that one shape wins both ends of
the selectivity axis, so no threshold is needed. **Every cell there was a union
of one arm, and at 128 that conclusion does not hold.**

| query | cell | `in-exists` (ships) | `wrapper-exists` | `windowed-in-exists` |
| --- | --- | ---: | ---: | ---: |
| partition-128 | first page | 572,182 | **77,267** | 264,779 |
| partition-128 | mid-replay | 736,600 | **102,476** | 320,253 |
| partition-128 | late replay | 791,400 | **108,382** | 154,098 |
| partition-128 | backwards | 680,852 | **103,529** | 312,238 |
| needle-128 | first page | 24,485 | 25,898,015 | **17,375** |
| needle-128 | mid-replay | 40,875 | 24,590,183 | **16,604** |
| needle-128 | late replay | 45,794 | 15,770,962 | **12,480** |
| needle-128 | backwards | 27,744 | 23,051,502 | **13,198** |

`partition-128` — item *i* names `bucket:b{i}`, so the union is the whole log and
no arm duplicates another. `needle-128` — 127 arms naming buckets that do not
exist and one that does, matching 3,907 events.

## 1. The crossover is back, and it is a different crossover

**Against what ships, the windowed arm still wins every cell**: 2.2x–5.1x on
`partition-128`, 1.4x–3.7x on `needle-128`. That part survives the width.

**Against `wrapper-exists` it does not.** It is 1.4x–3.4x *worse* on
`partition-128` and 1,264x–1,747x *better* on `needle-128`. At one arm the
windowed shape beat `wrapper-exists` at both ends; at 128 it beats it at one.

So the prediction on which the shape was proposed — *"the matched set is bounded,
therefore the `IN` wrapper stops being bad on a broad query"* — is **half right
and the half that fails is the one the argument was about**. The bound is real:
`budget x arms` is 65,536 positions instead of 500,000, which is why the windowed
shape beats the shipped one here at all. But 65,536 positions materialised to
return 512 rows is still 128x more work than the answer needs, and
`wrapper-exists` materialises nothing.

## 2. The cost is exactly the `budget x arms` term, and it is visible

The statement itself says so — same run:

| shape | SQL bytes | bound parameters |
| --- | ---: | ---: |
| `in-exists` | 25,816 | 387 |
| `wrapper-exists` | 26,831 | 387 |
| `windowed-in-exists` | **39,128** | **771** |

Two extra parameters per arm are the window's `lo` and `hi`; the extra bytes are
128 subquery preambles and 128 `ORDER BY … LIMIT ?` tails.

The `late-replay` cell is the tell that this is the `budget x arms` term and not
per-arm parsing overhead: the windowed shape costs 154,098 µs there against
264,779–320,253 µs elsewhere on the same query, and late replay is the one cell
whose window holds fewer than `budget` rows per arm — about 390 — so the `LIMIT`
stops binding and less is materialised. The cost tracks what the arms
materialise, not how many there are.

## 3. Width makes the *old* crossover far worse than `read-path.md` measured

`wrapper-exists` at one arm was *"2.3x worse on a selective query"*. At 128 arms
it is **1,264x–1,747x worse**, and 25.9 seconds for a single page is the slowest
statement anywhere in this crate.

The mechanism is plain: it walks `event` in rowid order and evaluates a
128-branch `UNION ALL` inside `EXISTS` per row. On `partition-128` it finds a
match after about 64 branches and stops. On `needle-128` it must reject all 128
before advancing, and it advances about 128 times per row it keeps.

So width is not a small correction to [`read-path.md`](read-path.md)'s table. It
is the axis on which that page's preferred candidate becomes unusable.

## 4. What this leaves the decision

Three things, and they point the same way without settling it.

1. **`in-exists` — what ships — is not best in any of the eight cells.** It loses
   to the windowed arm everywhere here.
2. **The remaining crossover is bounded on one side and not the other.** Picking
   the windowed arm everywhere costs at most **3.4x** against the best available
   shape. Picking `wrapper-exists` everywhere costs up to **1,747x**. An
   asymmetry of that shape argues for a default rather than a threshold — the
   opposite conclusion from [`read-path.md`](read-path.md) §4, and for the
   opposite reason.
3. **A conditional rule is not ruled out, it is merely no longer forced.** If one
   is wanted, the discriminator here is not tag selectivity but *arm count times
   page budget against matched-set size*, which the adapter already has both
   halves of: `Selectivity` knows the per-tag counts and `chunks` knows the arm
   count.

None of that is this crate's to decide, and none of it is implemented. It is
recorded in `references/seeds/adr-0022-shipped-shape-drift.md`.

## What this page does not show

One machine, one run, **5 rounds** per cell rather than the narrow file's ten —
`needle-128` under `wrapper-exists` is 25 seconds a page and four cells of it at
ten rounds would take longer than the rest of `run.sh` together. The windowed
column on `partition-128` moves 264,779 → 320,253 → 154,098 → 312,238 across
cells whose materialisation differs only at `late-replay`, so treat one-cell
differences under about 1.5x as noise; the claims above are the ones that survive
that.

Two tags and one type per item throughout, one store size, one page size, one
item count. **128 is a floor, not a range** — nothing here says where between 1
and 128 arms the windowed shape stops dominating, and
[`windowed-arms.md`](windowed-arms.md) says it does dominate at 1.

Nothing here is a patch to `happenstance-sqlite`.
