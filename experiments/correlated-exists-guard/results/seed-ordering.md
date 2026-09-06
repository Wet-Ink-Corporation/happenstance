# Does the correlated form make most-selective-first correct?

**Yes. The sign flips.**

Written by hand from [`raw/seed-ordering.txt`](raw/seed-ordering.txt), one
`./run.sh` on 2026-09-05. One 500,000-event store, 25 rounds per ordering,
**alternating which ordering runs first every round** so neither median is about
going second. Every round asserts the two orderings returned the identical
position before either time is recorded — ordering a conjunction cannot change
its answer, and if it ever did the faster arm would be wrong.

`CARDINALITY shard:cold=500000 row:r7=5155` — `row:r7` is 97x the more selective
tag, and is what the shipped ordering puts in the seed arm.

| shape | scenario | most-selective-first (**what ships**) | least-selective-first | most ÷ least |
| --- | --- | ---: | ---: | ---: |
| `chain-as-shipped` | accepted, at head | 978,539 | 25,817 | **37.9x** |
| `chain-as-shipped` | rejected, unbounded | 800,550 | 22,508 | **35.6x** |
| `chain-as-shipped` | rejected, mid-log | 935,364 | 21,759 | **43.0x** |
| **`chain-exists`** | accepted, at head | **28** | 59 | **0.47x** |
| **`chain-exists`** | rejected, unbounded | **26** | 57 | **0.46x** |
| **`chain-exists`** | rejected, mid-log | **29** | 58 | **0.50x** |

## What the flip means

`crates/happenstance-sqlite/src/event_store.rs:91-96`, in the crate's own
**public** module documentation:

> **`tag_cardinality` is a requirement rather than a convenience.** Multi-tag
> items must be probed most-selective-tag-first…

On the shape it shipped until this crate's findings were adopted, obeying that
requirement cost **36x–43x**. On the correlated form it now earns **2.0x–2.2x**.

[`query-plans.md`](query-plans.md) is why. The shipped chain materialises the
*chained* arm, so putting the selective tag in the seed puts the unselective one
in the materialisation — the expensive half — and the seed's selectivity buys
nothing. The correlated form makes the seed the outer loop, so a seed 97x smaller
is 97x less to walk.

So the rewrite does two things, not one. It removes the cost class, and it makes
`tag_cardinality` — a migration-1 table, an upsert inside every write
transaction, and a documented requirement — start paying for itself instead of
actively costing.

## The three sentences this bears on

* `event_store.rs:91-96` — the public requirement quoted above.
* `query_sql.rs:48-54` — *"an intersection chain seeded by the most selective
  tag, which keeps the boundary pushable … and is what `tag_cardinality` exists
  to order"*.
* ADR-0022 §8 (`references/adr/0022-append-condition-strategy.md:374-379`) —
  which makes it a migration-1 requirement.

All three were written about the `GROUP BY` form, where the question does not
arise: `tag IN (?,?)` is order-independent. They were inherited by the
intersection chain without being re-measured on it. This table says the
inheritance was wrong for the chain as shipped, and would be right for the chain
correlated.

## What this page does not show

One store, one cardinality ratio (97:1), two tags. Three tags compose two chained
`EXISTS` clauses and are not measured. A corpus where **neither** tag is
selective is measured — in [`unselective-pair.md`](unselective-pair.md) — and
there the ordering question is vacuous, because the two counts tie and a stable
sort leaves input order deciding.

The absolute microseconds here are **not comparable to**
[`guard-cost.md`](guard-cost.md)'s: that file times the whole guard path through
`ProbeStore`, this one times the prepared statement. Each is internally
consistent; do not divide one by the other.
