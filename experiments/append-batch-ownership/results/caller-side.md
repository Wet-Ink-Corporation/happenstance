# The caller-side axis: what a retrying writer pays under each shape

Written by hand from `raw/host.txt` and `raw/cloudflare-contention.txt`. ADR-0012
item 5: by-value `append` is only a net win if the caller can keep a copy for retry
more cheaply than the adapter saved.

**The schedule** (`src/contention.rs`): k = 8 contenders, one boundary tag, every
contender decides at the round's boundary, the first attempt lands and the rest are
refused. **36 attempts, 8 commits, 28 rejections** per run, identical in every
repetition (asserted). Batch 128, payload 1 KiB, `Vec` regime.

## Five callers

| caller | what it does between attempts |
| --- | --- |
| `B0Resend` | the adapter's path; resends the same `&[Event]` |
| `B1Resend` | raw port, borrowed; resends the same `&[Event]` |
| `O1ClonePerAttempt` | raw port, owned; `batch.clone()` before every attempt, because the call consumes it and a refusal gives nothing back |
| `B1Rebuild` | typed-layer shape (`crates/happenstance/src/command.rs`, a new batch every attempt), borrowed |
| `O1Rebuild` | typed-layer shape, owned |

## Cloudflare (wasm32), heap ops per whole contended run

| tags | B0Resend | B1Resend | O1ClonePerAttempt | B1Rebuild | O1Rebuild |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 23,005 | 16,817 | 32,725 | 53,897 | 51,849 |
| 8 | 73,181 | 52,657 | 100,821 | 191,113 | 189,065 |
| 64 | 474,589 | 339,377 | 645,589 | 1,265,801 | 1,263,753 |

* **Raw retry, owned: +90–95% heap ops over borrowed** (`O1ClonePerAttempt` vs
  `B1Resend`), and more bytes (1,890,736 against 1,437,616 at one tag), because
  the clone keeps the payload's refcount above one, so the owned arm's
  `Vec::from(Bytes)` **copies anyway**. The one saving O1 has is gone, and the
  clone is paid on all 36 attempts, including the 28 that write nothing.
* **Typed-layer retry: owned saves exactly 2,048 heap ops** (`B1Rebuild` −
  `O1Rebuild`) = 8 commits × 128 events × 2 buffers. Refused attempts write
  nothing in either shape, so they save nothing. Against a run of 52–1,266
  thousand heap ops that is 0.2–3.8% — **of this simplified run**, not of a
  measured typed-layer retry. `B1Rebuild` and `O1Rebuild` build a fixed batch
  each attempt and fence with one statement; the real typed command loop also
  reads, decodes, decides and encodes on every attempt. The 2,048-op difference
  isolates the append shape, so it stands; the percentage is an **upper bound**
  on its share, because the real loop does more work per attempt and the same
  2,048 ops are a smaller share of it.

Median wall time per contended run (µs) [Q1, Q3]:

| tags | B1Resend | O1ClonePerAttempt | B1Rebuild | O1Rebuild |
| ---: | --- | --- | --- | --- |
| 1 | 33,092 [31,036, 51,370] | 32,047 [31,340, 48,596] | 37,296 [35,519, 60,657] | 36,484 [34,859, 62,151] |
| 8 | 126,220 [123,699, 132,921] | 123,204 [96,379, 129,846] | 137,388 [131,771, 144,273] | 133,990 [103,224, 138,918] |
| 64 | 880,486 [807,795, 950,863] | 885,409 [841,336, 926,284] | 926,490 [834,553, 1,006,679] | 936,951 [852,646, 975,692] |

No caller's median is separable from its counterpart's here; the heap-op columns
are the result.

## Host, on the reference store (`MemoryEventStore` and its replicas)

What one copy of a 128-event batch costs (`raw/host.txt`, "caller-side prices"):

| tags | first clone, `Vec` | later clone | build a batch (typed retry) |
| ---: | ---: | ---: | ---: |
| 1 | 641 ops, 19.5 µs | 385 ops, 12.6 µs | 1,030 ops, 143.9 µs |
| 8 | 1,537 ops, 42.6 µs | 1,281 ops, 35.4 µs | 3,846 ops, 256.2 µs |
| 64 | 8,705 ops, 300.4 µs | 8,449 ops, 312.3 µs | 25,734 ops, 1,362.6 µs |

A clone is `t + 2` heap ops per event, plus one per `Vec`-backed buffer on its
**first** clone, which promotes it to the shared representation: `t + 4` here,
with data and metadata. That promotion is how `tests/host.rs` first went red.

The same k = 8 schedule on the reference store, median µs per run:

| tags | RealResend | BorrowedResend | OwnedClonePerAttempt | BorrowedRebuild | OwnedRebuild |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 194.4 | 195.7 | 830.2 | 5,437.0 | 5,302.8 |
| 8 | 392.5 | 391.6 | 2,218.3 | 10,043.0 | 9,358.1 |
| 64 | 2,059.5 | 2,065.1 | 13,614.8 | 53,524.4 | 47,874.3 |

On the one store where the clone *is* the write path, the raw-retry owned caller
is **4.2–6.6× slower** than the borrowed one, and the typed-layer owned caller
is 2–11% faster. This is the ceiling on what ownership could buy, measured on
the store ADR-0012 rules out as unrepresentative.
