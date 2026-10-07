# The decision cells: batch 128, payload ≤ 16 KiB, `Vec`-backed regime

Written by hand from `raw/cloudflare-sweep.txt` (the `batch=128 … regime=vec` rows).
Times are microseconds per append (the call and the drop of the batch), median
and interquartile range over n = 21 after 3 warm-up repetitions, wasm32 under Node
24 against the `node:sqlite` Durable Object host. Heap ops are allocations plus
reallocations per append, identical in all 21 repetitions of every row.

The rule (README, written before the run) fires in a cell only if
`median(O1) < 0.9 × median(B1)` **and** `median(O1) < Q1(B1)`.

| tags | payload | B1 median [Q1, Q3] | O1 median | O1 saving vs B1 | O1 < Q1(B1)? | fires |
| ---: | ---: | --- | ---: | ---: | :---: | :---: |
| 1 | 64 B | 3,658 [3,478, 3,886] | 3,665 | −0.2% | no | no |
| 8 | 64 B | 11,101 [10,786, 11,743] | 11,010 | 0.8% | no | no |
| 64 | 64 B | 96,041 [80,339, 108,091] | 84,949 | 11.5% | no | no |
| 1 | 1 KiB | 4,179 [4,072, 4,212] | 4,213 | −0.8% | no | no |
| 8 | 1 KiB | 12,027 [10,911, 13,412] | 11,256 | 6.4% | no | no |
| 64 | 1 KiB | 72,879 [69,235, 99,595] | 95,189 | −30.6% | no | no |
| 1 | 16 KiB | 5,569 [5,351, 6,335] | 5,404 | 3.0% | no | no |
| 8 | 16 KiB | 12,432 [12,339, 12,843] | 12,541 | −0.9% | no | no |
| 64 | 16 KiB | 101,152 [74,153, 102,563] | 98,733 | 2.4% | no | no |

**0 of 9 cells fire.**

## The same cells, all four arms (median µs)

| tags | payload | adapter | B0 | B1 | O1 |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 64 B | 3,770 | 3,735 | 3,658 | 3,665 |
| 8 | 64 B | 11,120 | 11,335 | 11,101 | 11,010 |
| 64 | 64 B | 92,791 | 96,758 | 96,041 | 84,949 |
| 1 | 1 KiB | 4,335 | 4,217 | 4,179 | 4,213 |
| 8 | 1 KiB | 11,237 | 11,326 | 12,027 | 11,256 |
| 64 | 1 KiB | 97,214 | 96,947 | 72,879 | 95,189 |
| 1 | 16 KiB | 5,633 | 5,744 | 5,569 | 5,404 |
| 8 | 16 KiB | 12,780 | 12,727 | 12,432 | 12,541 |
| 64 | 16 KiB | 73,398 | 100,160 | 101,152 | 98,733 |

## How noisy the clock is here: two controls the sweep carries for free

**The 64-tag rows are bimodal.** Every arm's samples at 64 tags cluster near
70 ms and near 97–100 ms (see any 64-tag row's Q1 and Q3), so a median can jump
between the two modes. That is why the 64-tag medians move by ±30% between arms
whose allocation counts differ by 0.6%.

**The static regime is a placebo.** There, O1's `Vec::from(Bytes)` must copy, and
O1 makes *exactly* B1's heap operations and bytes in all 36 static points (checked
row by row). Any O1-vs-B1 difference there is noise. At batch 128 it reaches
**19.6%** (64 tags, 64 B: B1 91,617 [71,234, 102,497], O1 73,681) and 12.5%
(1 tag, 256 KiB), and in neither case is O1's median below B1's Q1. That is the
case the IQR clause was written for.

## Outside the decision cells, stated rather than buried

The sweep prints the rule's verdict for all 72 points (`rule:` rows in
`raw/cloudflare-sweep.txt`), 63 of them outside the nine decision cells. Both
conditions held in **two** of those 63, both in the `Vec` regime; no static
(placebo) row fired. These are every `fires=true` row outside the region:

| batch | tags | payload | B1 median [Q1, Q3] µs | O1 median µs | saving |
| ---: | ---: | ---: | --- | ---: | ---: |
| 128 | 8 | 256 KiB | 48,823 [41,796, 50,088] | 41,570 | 14.9% |
| 1 | 8 | 64 B | 130.5 [112.8, 144.4] | 112.5 | 13.8% |

The 256 KiB cell (above the rule's 16 KiB bound, 32 MiB per batch) beside its
neighbours at batch 128:

| tags | payload | B1 median [Q1, Q3] | O1 median [Q1, Q3] | saving |
| ---: | ---: | --- | --- | ---: |
| 1 | 256 KiB | 35,846 [34,476, 41,593] | 38,760 [32,980, 40,077] | −8.1% |
| **8** | **256 KiB** | 48,823 [41,796, 50,088] | 41,570 [40,263, 48,357] | **14.9%** |
| 64 | 256 KiB | 148,481 [139,120, 179,859] | 172,156 [139,215, 178,715] | −15.9% |

Its two neighbours went the other way by similar amounts, and the static
placebo at the same payload showed 12.5% and 8.9% with identical work. The
batch-1 cell is a single event whose O1 saves exactly 2 of B1's 59 heap
operations (57 against 59) in an append of about 130 µs.

**They are noise, and a re-run says so.** An independent reviewer's re-run of
the sweep (not committed; its rows are not in `raw/`) fired at different cells:
batch 1, 1 tag, 256 KiB at 19.5%, and batch 1, 8 tags, 256 KiB at 10.4%, while
batch 128, 8 tags, 256 KiB did not fire. Cells that fire on one run and not the
next, with no change in what the arms allocate, are the clock, not the
ownership. Nothing in them changes the verdict either way, because the rule's
region was fixed before the run.
