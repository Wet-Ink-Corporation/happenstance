# What each arm allocates, per append

Written by hand from `raw/cloudflare-sweep.txt` and `raw/conformance-cloudflare.txt`.
Batch 128, `Vec`-backed regime. Heap ops are allocations plus reallocations;
bytes are bytes *requested* (a lower bound on resident growth). Every count was
identical in all 21 repetitions of its row, at all 72 sweep points.

## Heap ops per append (independent of payload size)

| tags | adapter | B0 | B1 | O1 (`Vec`) | O1 (static) | B1 − O1 | O1 saving as % of B1 |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 2,825 | 2,825 | 2,056 | 1,800 | 2,056 | 256 | 12.5% |
| 8 | 9,097 | 9,097 | 6,536 | 6,280 | 6,536 | 256 | 3.9% |
| 64 | 59,273 | 59,273 | 42,376 | 42,120 | 42,376 | 256 | 0.6% |

* **adapter = B0, exactly**, at every one of the 72 points (heap ops *and* bytes):
  the replica is the adapter's write path. `b0_allocates_exactly_what_the_adapter_allocates`
  holds this on every run; it went red when B0's positions `Vec` lost its
  `with_capacity` (two reallocations).
* **B0 − B1 = 128 × (4 + 2t) + 1** — the `to_binding` clones of every `Text` and
  `Blob` bound: four per event row (type, data, metadata, tag blob), two per tag
  row (tag, type), one for the stamp. That is 769, 2,561 and 16,897: **27–29%** of
  the adapter's heap ops, removable **without any signature change**.
* **B1 − O1 = 2 × 128 = 256** in the `Vec` regime — one buffer each for data and
  metadata per event, the only things an owned batch can move — and **0** in the
  static regime, where `Vec::from(Bytes)` copies. The event type and the tags are
  copied by every arm: core has no `EventType → String` or `Tag → String`, and the
  type is written once per tag row besides.

## Bytes requested per append

| tags | payload | adapter / B0 | B1 | O1 (`Vec`) |
| ---: | ---: | ---: | ---: | ---: |
| 1 | 64 B | 75,362 | 55,378 | 43,090 |
| 1 | 1 KiB | 321,122 | 178,258 | 43,090 |
| 1 | 16 KiB | 4,253,282 | 2,144,338 | 43,090 |
| 8 | 1 KiB | 451,938 | 275,922 | 140,754 |
| 64 | 1 KiB | 1,498,466 | 1,057,234 | 922,066 |
| 64 | 16 KiB | 5,430,626 | 3,023,314 | 922,066 |

O1's bytes do not depend on the payload at all: the payload is moved, so it is
never requested again on the Rust side. B1 requests each payload once, B0 twice.
Neither figure counts `worker`'s copy into a JS `Uint8Array`
(`worker-0.8.5/src/sql.rs:126-130`), which every arm pays and no signature removes.

## And yet: time

B1 makes 27–29% fewer heap operations than B0 and requests 25–50% fewer bytes,
depending on payload and tag count (25.4% at 64 tags and 64 B, 49.6% at one tag
and 16 KiB, 50.0% at 256 KiB; every batch-128 row of `raw/cloudflare-sweep.txt`),
and its median append time is within a few percent of B0's at every decision
cell (`realistic-point.md`). O1's further 256 operations do not show at all. On
this host an append's cost is the statements, `128 × (1 + t) + 1` of them, each a
JS boundary crossing plus a `prepare` in `node:sqlite`; the allocator is not where
the time goes.
