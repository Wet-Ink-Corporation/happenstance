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
| **64** | **66** | **2,001** | **1** | **1,536** | **66.0x** |

**The owned column is `t + 2` for every `t >= 1`, exactly.** One allocation for
the `EventType`'s `String`, one for the `Box<[Tag]>`, and one per tag. At `t = 0`
it is **1**, not 2: `<[T]>::to_vec()` on an empty slice allocates nothing, so the
published figure of "two allocations" is wrong in both directions — it
overcounts a tagless event by one and undercounts a 64-tag event by 64.

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
| **64** | **563 / 587** | **73** | **8** | **65** | **140** | **10** | **130** | **2.00** |

### `serde_json`

| tags | out bytes | `Event` delta | `SequencedEvent` delta | ratio |
| ---: | ---: | ---: | ---: | ---: |
| 0 | 110 / 226 | 1 | 2 | 2.00 |
| 1 | 119 / 235 | 2 | 4 | 2.00 |
| 8 | 189 / 305 | 9 | 18 | 2.00 |
| 32 | 429 / 545 | 33 | 66 | 2.00 |
| **64** | **749 / 865** | **65** | **130** | **2.00** |

**The ratio is 2.00 at every tag count in both formats.** `Serialize for
SequencedEvent` clones the whole `Event` into `SequencedEventWire`, and the
derived impl on that mirror then calls `Serialize for Event`, which builds
`EventWire` and clones all four fields again.

The absolute figure is the one worth quoting. Encoding **one** 64-tag
`SequencedEvent` to postcard costs **140 heap operations to produce 587 bytes**,
of which **130 (93%) are transient clones that produce no output**. The same
value in the borrowed regime costs 10.

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
