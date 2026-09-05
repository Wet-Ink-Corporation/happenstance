# What `read(limit = 1)` costs on `MemoryEventStore`

Transcribed by hand from `results/raw/read.txt`, the untouched output of
`cargo test --release --test measure_read -- --test-threads=1 --nocapture`.
Conditions in `../README.md`.

Every store below is filled through the **owned** tag regime
(`Tags::from_pairs`), because that is the regime an application is in. A store
filled through `Tag::from_static` would make every clone cost one allocation
regardless of tag count, and the whole table would flatten — which is exactly the
measurement this experiment exists to refuse.

`before` is `readpath::read_before`, a transcription of `memory.rs:302-333` at
`56ef6c5`. `after` is the same function with `.take(limit)` moved above
`.cloned()`. `tests/arms_are_equivalent.rs` proves, over 160 query/option
combinations against a mixed store, that `before` returns what the **real**
`MemoryEventStore::read` returns and that `after` returns what `before` returns.

> **`tests/measure_read.rs` is red as of 2026-09-04, and the reason is that the
> fix landed.** Three of its assertions tie the *real* store to the **before**
> arm — `real_limited.heap_ops().abs_diff(before.heap_ops()) < 64` is the
> sharpest — and `MemoryEventStore::read` now costs 6 heap ops at `limit=1`
> against the before arm's 40,013 on a 10,000-event store. It is the **after**
> arm the real store now matches. The rows above are therefore a record of the
> decision, not a live comparison, and `run.sh` exits non-zero at this step.
>
> Not repaired here, and deliberately: this file is H2's, the change that
> falsified it is not this lane's, and rewriting an assertion to agree with a
> store whose behaviour someone else moved today is exactly how a measurement
> stops being one. It is recorded so that the next person to run `run.sh` reads
> this before reading an exit code. Arms 3 and 4 — which is what AE-2 is about,
> and what `clone-cost.md` reports — ran green in the same invocation and print
> `all encode-cost assertions hold`.

## The sweep

| store | tags/event | arm | heap ops | bytes requested | median wall clock (min..max of 5) |
| ---: | ---: | --- | ---: | ---: | ---: |
| 10 | 2 | real store, `limit=1` | 44 | 3,670 | 2.4 µs (2.1..3.5) |
| | | replica **before** | 43 | 3,094 | 2.0 µs (1.9..2.1) |
| | | replica **after** | **5** | **655** | **100 ns** (100..200) |
| 10,000 | 2 | real store, `limit=1` | 40,014 | 3,149,872 | 3.13 ms (2.90..3.18) |
| | | replica **before** | 40,013 | 3,149,296 | 3.10 ms (3.03..3.37) |
| | | replica **after** | **5** | **655** | **300 ns** (100..300) |
| 1,000,000 | 2 | real store, `limit=1` | 4,000,020 | 229,995,520 | 383 ms (330..412) |
| | | replica **before** | 4,000,019 | 229,994,944 | 311 ms (268..342) |
| | | replica **after** | **5** | **655** | **2.0 µs** (1.9..2.2) |
| 10,000 | **64** | real store, `limit=1` | 660,014 | 22,369,872 | 40.0 ms (34.5..48.3) |
| | | replica **before** | 660,013 | 22,369,296 | 34.2 ms (33.9..44.5) |
| | | replica **after** | **67** | **2,577** | **1.8 µs** (1.7..1.9) |

At a million events and two owned tags each, a `limit(1)` read performs **four
million heap allocations and requests 230 MB** to return **one event**. The same
read after the change performs **five allocations and requests 655 bytes**. The
ratio on allocations is 800,004:1; on wall clock, 311 ms against 2.0 µs.

**Quote the allocation columns, not the timing column.** The counts are
deterministic — identical to the digit across four separate `./run.sh`
invocations of this crate — while the wall-clock medians moved by up to 40%
between runs on this host, which is a property of measuring on a shared
developer laptop rather than of the code. `experiments/append-condition/`'s
README records the same thing about the same machine.

## The control that matters more than the ratio

| store | `limit=1` heap ops | `limit=None` heap ops | ratio |
| ---: | ---: | ---: | ---: |
| 10 | 44 | 46 | 0.9565 |
| 10,000 | 40,014 | 40,026 | 0.9997 |
| 1,000,000 | 4,000,020 | 4,000,038 | **1.0000** |
| 10,000 @ 64 tags | 660,014 | 660,026 | **1.0000** |

**`limit` buys the caller nothing.** Asking for one event out of a million costs
what asking for all million costs, to four significant figures. The only
difference the two columns show is in `deallocs`: the `limit=1` row frees
3,999,997 of the four million allocations before returning, because `truncate`
drops the tail it had just finished cloning. The work was done and then thrown
away, which is what makes this a defect rather than a cost.

## Two controls on the instrument

**The fix must be free, and only free, when there is no limit.** At `limit=None`
and 10,000 events, `before` and `after` cost identically — 40,013 heap ops,
3,149,296 bytes, both arms. A change that were cheaper here as well would be a
change that had altered the semantics.

**Which fully-matching query asked does not matter.** `Query::all()`
short-circuits `matches` (`query.rs:221`); the tag query used throughout runs the
comparison in full. Both cost 40,014 heap ops at 10,000 events and `limit=1`, so
the figures above are about the clone rather than about the filter.

## How the two findings compose

The per-event clone cost is `t + 2` (see `clone-cost.md`), and this path pays it
once per **matched** event rather than once per **returned** event. They
multiply. The 10,000-event row at VT-22's 64-tag floor is the product: 660,013
heap operations and 22 MB requested for a single-event read, on a store holding
about 27 MB.
