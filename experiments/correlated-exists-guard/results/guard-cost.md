# Guard cost: six shapes, four scenarios, three log sizes

Written by hand from [`raw/guard-cost.txt`](raw/guard-cost.txt), one `./run.sh`
on 2026-09-05. Conditions: [`../README.md#conditions`](../README.md#conditions).

`guard_us_median` — the guard statement inside `BEGIN IMMEDIATE`, with the
transaction's own cost (`txn_only_us_median`) subtracted. Rounds: 120 at 50,000,
30 at 500,000, 15 at 10^6. **Shapes rotate within each round**, so no shape is
permanently first and none of the differences below is an artefact of going
second.

## The single-tag control

Every shape, every size, every scenario: **8–10 µs**, indistinguishable.

That is the control that makes the rest of this page mean something. The
single-tag fast path emits no chained subquery at all, so all six shapes emit the
same SQL and must — and do — cost the same. A table where they differed would be
measuring the harness.

## Two tags, µs

| size | scenario | `chain-as-shipped` | `chain-bounded-all-arms` | `grouped-adr0022` | **`chain-exists`** | **`chain-exists-bounded-seed`** |
| ---: | --- | ---: | ---: | ---: | ---: | ---: |
| 50,000 | accepted, at head | 17,477 | 90 | 9,953 | **91** | **32** |
| 50,000 | rejected, mid-log | 17,181 | 8,665 | 9,861 | **92** | **32** |
| 50,000 | rejected, unbounded | 17,592 | 17,708 | 10,092 | **93** | **33** |
| 500,000 | accepted, at head | 210,143 | 116 | 136,987 | **125** | **35** |
| 500,000 | rejected, mid-log | 213,548 | 106,989 | 135,150 | **127** | **38** |
| 500,000 | rejected, unbounded | 210,312 | 216,712 | 136,615 | **130** | **37** |
| 10^6 | accepted, at head | 579,883 | 137 | 331,397 | **129** | **33** |
| 10^6 | rejected, mid-log | 580,703 | 277,154 | 353,897 | **151** | **37** |
| 10^6 | rejected, unbounded | 549,047 | 561,838 | 332,810 | **169** | **65** |

`chain-bounded-seed` is omitted from the table: it sits within a few percent of
`chain-as-shipped` in all nine cells — its query plan is byte-for-byte the
shipped one — and a column that never moves is a column a reader has to skip. It
is in [`raw/guard-cost.txt`](raw/guard-cost.txt) in full.

## 1. `chain-exists` wins every cell, and the margin grows with the log

| size | vs `chain-as-shipped` |
| ---: | --- |
| 50,000 | 189x – 192x |
| 500,000 | 1,617x – 1,681x |
| 10^6 | 3,248x – 4,495x |

The margin grows because the shipped chain grows and `chain-exists` does not:
**91 → 125 → 129 µs** across a 20x log growth, against 17,477 → 210,143 →
579,883. See [`../README.md`](../README.md#what-this-does-not-show) item 2 —
near-flat *over the range measured* is what this establishes, not a scaling law,
and [`unselective-pair.md`](unselective-pair.md) finds a reason to be careful
about why it is flat.

## 2. The boundary push is a conditional repair; this is not

`chain-bounded-all-arms` was the best remediation the sibling experiment found,
and it is excellent **when the boundary is high**: 137 µs at 10^6 with the guard
anchored at head, a 4,200x improvement on shipped. Read down its column instead
of across:

| 10^6, `chain-bounded-all-arms` | µs |
| --- | ---: |
| boundary at head | 137 |
| boundary mid-log | 277,154 |
| **no boundary** | **561,838** |

With nothing to push there is nothing to shrink, and the shape collapses back
onto the shipped chain — here slightly past it. `chain-exists` reads 129 / 151 /
**169** across the same three: a 1.3x spread against a 4,100x one.
[`unselective-pair.md`](unselective-pair.md) finds the same collapse on a corpus
where no tag is selective, at 1.13 seconds.

That matters because the unbounded guard is not an edge case. It is
`AppendCondition::new(query)` with no anchor: the *"this must not exist anywhere
in the log"* shape — a unique name, an idempotency key, a first write against a
boundary. `examples/handles-and-quotas/` is built on it.

## 3. The two remediations compose

`chain-exists-bounded-seed` is 2–4x cheaper again (32–65 µs) and flat across all
nine cells. With the materialisation already gone, a boundary on the seed arm is
a seek into the interior of one contiguous `(tag, position)` range, and it is the
only cost left to remove.

## 4. Both remediated chains beat the aggregate ADR-0022 decided on

`grouped-adr0022` is stable — 331,397 to 353,897 µs at 10^6, barely moving with
the boundary, because `GROUP BY` is an optimisation barrier and nothing can be
pushed through it. It beats the *shipped* chain by 1.5x–1.8x, which is the
sibling experiment's finding reproduced. It loses to `chain-exists` by
2,000x–2,600x.

So the chain is repairable, and repaired it is far better than the aggregate.
That is evidence for the chain's shape — but not a licence to close ADR-0022 §16
from here; see [`../README.md`](../README.md#what-this-does-not-show) item 6.

## What this page does not show

One machine, one run per cell, 15 rounds at the largest size. The claims are
ratios between shapes measured in the same round-robin, never absolute
throughput, and never a third significant figure. The single-tag control above
is what distinguishes *"these shapes differ"* from *"this harness is noisy"*; it
came out flat.
