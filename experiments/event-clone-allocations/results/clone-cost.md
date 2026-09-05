# What one `Event` clone costs

Every row is transcribed by hand from `results/raw/clone.txt`, which is the
untouched output of `cargo test --release --test measure_clone --
--test-threads=1 --nocapture`. Conditions are in `../README.md`; the toolchain
that produced the file is the first thing in `results/raw/conditions.txt`.

`heap_ops` is `alloc` + `realloc`. `bytes` is bytes **requested**, not resident.

## Arms 1 and 2 — `event.clone()`, one event, steady state

Payload is `Bytes::from_static`, so nothing in these rows is the payload. Metadata
is `None`. "Steady state" means the second clone; see the promotion table below
for why the first can differ.

| tags | `Tags::from_pairs` (`Cow::Owned`) | | `Tag::from_static` (`Cow::Borrowed`) | | ratio |
| ---: | ---: | ---: | ---: | ---: | ---: |
| | heap ops | bytes | heap ops | bytes | |
| 0 | 1 | 17 | 0 | 0 | — |
| 1 | 3 | 48 | 1 | 24 | 3.0x |
| 8 | 10 | 265 | 1 | 192 | 10.0x |
| 32 | 34 | 1,009 | 1 | 768 | 34.0x |
| 64 | 66 | 2,001 | 1 | 1,536 | 66.0x |
| **128** | **130** | **4,041** | **1** | **3,072** | **130.0x** |

**The owned column is `t + 2` for every `t >= 1`, exactly.** One allocation for
the `EventType`'s `String`, one for the `Box<[Tag]>`, and one per tag. At `t = 0`
it is **1**, not 2: `<[T]>::to_vec()` on an empty slice allocates nothing, so the
published figure of "two allocations" is wrong in both directions — it
overcounts a tagless event by one and undercounts a 64-tag event by 64.

The bold row moved from 64 to 128 on 2026-09-04. Both are quoted and both are
someone's promise: **64** is `MIN_SUPPORTED_TAGS_PER_EVENT`, VT-22's floor, which
every conformant adapter must accept; **128** is
`SqliteEventStore::MAX_TAGS_PER_EVENT` (`crates/happenstance-sqlite/src/event_store.rs:291`),
the only documented adapter ceiling in the workspace and therefore the largest
event a caller can rely on being accepted anywhere in this tree. The cost is
linear in `t`, so one of the two numbers is half an answer.

**The borrowed column is `1` for every `t >= 1`, and it does not move.** The
boxed slice is still allocated and copied — 1,536 bytes at 64 tags, which is
*most* of the owned arm's 2,001 — but every `Cow::Borrowed` inside it clones as
two words. This is the arm a benchmark author writes without choosing to, and it
would have reported that tag count does not matter.

## The payload, which is a separate claim

`memory.rs:30-31` says "payloads are `Bytes`, so a snapshot bumps refcounts
rather than copying data". Measured, that is true of the second clone and of
every clone after it, and false of the first when the payload came from a `Vec`.

| payload | first clone | second clone | difference |
| --- | ---: | ---: | ---: |
| `Bytes::from_static`, 64 owned tags | 66 | 66 | 0 |
| `Bytes::from(Vec<u8>)`, 64 owned tags | 67 | 66 | **+1** |
| `Bytes::from_static`, 64 static tags | 1 | 1 | 0 |
| `Bytes::from(Vec<u8>)`, 64 static tags | 2 | 1 | **+1** |

`bytes` 1.x stores a `Vec`-backed payload in a *promotable* representation; the
first clone allocates a shared header and every clone after it is a refcount
bump. One allocation, once per payload, and it is real: a store that clones each
appended event once — which `MemoryEventStore::append` does — pays it on every
event it ever accepts. It is not part of `t + 2` and is reported separately
rather than folded in.

## Arms 3 and 4 — the `serde` encode path

> **These tables are the state of the tree, and a prototype fix is measured
> against them below.** Every row here is `happenstance-core` as it ships: each
> `Serialize` impl builds an *owned* wire mirror. A borrowing mirror on the
> `Serialize` side — serde's own idiom, keeping the owned one for `Deserialize` —
> takes every delta to **zero**, and is measured in
> *[What the prototype costs](#what-the-prototype-costs)*. It is **not landed**;
> that section says why.


`delta` is the owned row minus the static row at the same tag count and format.
`tests/arms_are_equivalent.rs` proves the two rows emit **byte-identical**
output in both formats at every tag count, so the encoder's own allocations
cancel and the delta is the wire mirror's clone and nothing else.

### `postcard`

| tags | out bytes | `Event` owned | `Event` static | delta | `SequencedEvent` owned | `SequencedEvent` static | delta | ratio |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 0 | 51 / 75 | 5 | 4 | 1 | 7 | 5 | 2 | 2.00 |
| 1 | 59 / 83 | 7 | 5 | 2 | 11 | 7 | 4 | 2.00 |
| 8 | 115 / 139 | 15 | 6 | 9 | 26 | 8 | 18 | 2.00 |
| 32 | 307 / 331 | 41 | 8 | 33 | 75 | 9 | 66 | 2.00 |
| 64 | 563 / 587 | 73 | 8 | 65 | 140 | 10 | 130 | 2.00 |
| **128** | **1,132 / 1,156** | **138** | **9** | **129** | **269** | **11** | **258** | **2.00** |

### `serde_json`

| tags | out bytes | `Event` delta | `SequencedEvent` delta | ratio |
| ---: | ---: | ---: | ---: | ---: |
| 0 | 110 / 226 | 1 | 2 | 2.00 |
| 1 | 119 / 235 | 2 | 4 | 2.00 |
| 8 | 189 / 305 | 9 | 18 | 2.00 |
| 32 | 429 / 545 | 33 | 66 | 2.00 |
| 64 | 749 / 865 | 65 | 130 | 2.00 |
| **128** | **1,445 / 1,561** | **129** | **258** | **2.00** |

**The ratio is 2.00 at every tag count in both formats**, now including 128.
`Serialize for SequencedEvent` clones the whole `Event` into
`SequencedEventWire`, and the derived impl on that mirror then calls `Serialize
for Event`, which builds `EventWire` and clones all four fields again.
`delta(Event)` is `t + 1` and `delta(SequencedEvent)` is `2(t + 1)`, and the
assertions at the foot of `tests/measure_clone.rs` are written as those formulas
rather than as the numbers, so a change to `Tag`, `Tags`, `EventType` or the
mirrors turns the file red rather than silently moving a published figure.

The absolute figure is the one worth quoting. Encoding **one** 64-tag
`SequencedEvent` to postcard costs **140 heap operations to produce 587 bytes**,
of which **130 (93%) are transient clones that produce no output**. The same
value in the borrowed regime costs 10. At the documented ceiling of 128 tags it
is **269 heap operations to produce 1,156 bytes, of which 258 (96%) produce no
output**, against 11 borrowed.

### What that costs at the guaranteed minimums

The floors are the point: they are what a peer may send without asking, so they
are the batch a sync runner must be able to encode.

| | events | tags | transient heap ops on encode | postcard output |
| --- | ---: | ---: | ---: | ---: |
| VT-24 × VT-22 floors | 128 | 64 | **16,640** | 75,136 B |
| SQLite's declared ceilings | 256 | 128 | **66,048** | 295,936 B |

`2 × events × (tags + 1)`, from the measured `delta(SequencedEvent) = 2(t + 1)`.
Nothing in those two columns is resident memory: `heap_ops` is `alloc` +
`realloc` and `bytes` is bytes *requested*. The wasm32 residency delta the
finding also asks about is still unmeasured, and this experiment cannot measure
it — it runs on the host.

**One correction to the finding that prompted this row.** AE-2 computes the floor
case as `2 × 128 × 66 = 16,896`, using the *clone* cost `t + 2`. The measured
*encode delta* is `t + 1` — the `Box<[Tag]>` allocation is present in both arms
and cancels — so the figure is **16,640**, 256 lower. The ratio, the linearity
and the argument are unaffected.

## What the prototype costs

Same harness, same conditions, same values, with a **borrowing** mirror at each
of the five `Serialize` sites — `Event`, `SequencedEvent`, `QueryItem`, `Query`,
`AppendCondition` — keeping the owned mirror for `Deserialize`, which is serde's
own idiom for the asymmetry. Written and measured at **`2f11eb7`** on
`lane/core-ports`, then reverted.

`tests/arms_are_equivalent.rs` proves the two regimes still emit byte-identical
output, and the encodings of four values were compared against the bytes the
previous commit produced — the four small vectors now pinned in
`crates/happenstance-core/tests/serialize_is_borrowing.rs`, and the full 64-tag
values, all 4,124 bytes of them. **No byte moved**, in either format.

**Why it is not landed.** It adds 66 lines to `event.rs` and 30 to `query.rs`
above each file's `mod tests`, which renumbers three test functions that
`spec/SPECIFICATION.md` cites by line — `zero_limit_means_zero_events`,
`to_is_recorded_and_independent_of_from`, `position_next_signals_overflow` —
past `spec-trace`'s twelve-line tolerance, and repointing those four citations
means editing the specification. The proposal and the four repoints are in
`.kb/_intake/remediation-2026-09-04-briefs/serialize-borrows-what-it-writes.md`.

| value | format | tags | out bytes | heap ops today | heap ops on the prototype |
| --- | --- | ---: | ---: | ---: | ---: |
| `Event` | postcard | 64 | 563 | 73 | **7** |
| `SequencedEvent` | postcard | 64 | 587 | 140 | **8** |
| `Event` | postcard | 128 | 1,132 | 138 | **8** |
| `SequencedEvent` | postcard | 128 | 1,156 | 269 | **9** |
| `Event` | serde_json | 128 | 1,445 | 136 | **6** |
| `SequencedEvent` | serde_json | 128 | 1,561 | 266 | **6** |
| `QueryItem` | postcard | 64 | 516 | 139 | **8** |

What remains on the prototype is the encoder's own: one allocation for the
output buffer and the reallocations it performs as it grows. Identical in both
tag regimes — which is what `delta = 0` says — and scaling with the *output*
rather than with the value, which is the only thing an encode should cost.

`tests/measure_clone.rs` still asserts `t + 1` and `2(t + 1)`, because that is
what the tree does. On the prototype both become `0`, and there is no term left
to write.

### The guaranteed minimums, both ways

| | events | tags | transient heap ops today | on the prototype |
| --- | ---: | ---: | ---: | ---: |
| VT-24 × VT-22 floors | 128 | 64 | 16,640 | **0** |
| SQLite's declared ceilings | 256 | 128 | 66,048 | **0** |

Zero is the *transient* count — the allocations that produce no output. Each
encode still allocates its own output buffer and grows it, and that cost is
unchanged, because the bytes are unchanged.

`Serialize for Query` is the same pattern one file over (`query.rs:380`,
`items.to_vec()`): one 64-tag `QueryItem` costs 139 heap ops owned against 11
borrowed, for 516 bytes of output.

## The control on the unit

| | heap ops |
| --- | ---: |
| `Tag::new("k00:v00").clone()` | 1 |
| `Tag::from_static("k00:v00").clone()` | 0 |

## `size_of`, measured

H3 asks for a layout budget and states, correctly, that its own figures were
reasoned rather than compiled. These are compiled, on the toolchain in
`../README.md`, target `x86_64-pc-windows-msvc`.

| type | `size_of` | `align_of` | reasoned in H3 |
| --- | ---: | ---: | --- |
| `Tag` | **24** | 8 | 32 (`Cow` "with no available niche") |
| `EventType` | **24** | 8 | 32 |
| `Tags` | 16 | 8 | 16 |
| `Event` | **104** | 8 | — |
| `SequencedEvent` | **144** | 8 | — |
| `QueryItem` | 32 | 8 | — |

`Cow<'static, str>` is **24 bytes, not 32**: `String`'s capacity field carries the
discriminant, so the niche H3 assumed absent is present. Any `tests/layout_budget.rs`
written from the finding's reasoned numbers would have failed on the first
`cargo test`. That is the finding's own instrument correcting the finding, which
is the point of building it.

Context, unchanged from what the crate already asserts at `event.rs:970-976`:
`SequencePosition` and `Option<SequencePosition>` are both 8, and `Bytes` and
`Option<Bytes>` are both 32.
