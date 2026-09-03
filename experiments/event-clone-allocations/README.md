# `experiments/event-clone-allocations`

What one `Event` clone costs, measured, in **both** tag regimes — and what
`MemoryEventStore::read` pays for a `limit(1)` read on a store it has to scan.

This is not a crate anybody depends on. It is **not a workspace member** (its
`Cargo.toml` carries an empty `[workspace]` table, the same trick
`experiments/append-condition/` and `experiments/wire-format/` use), it appears
in no `verify:` command and no `cargo xtask ci` step, and it adds no dependency
to any workspace manifest — `Cargo.lock` at the repository root is untouched.

## Why it exists

Four places in this repository state the cost of cloning an `Event` as **two**
heap allocations:

| site | the sentence |
| --- | --- |
| `crates/happenstance-core/src/event.rs:404-413` | "…leaving one `Box<str>` and one boxed tag slice." |
| `crates/happenstance-core/src/memory.rs:30-31` | "Cloning is cheap regardless: payloads are `Bytes`, so a snapshot bumps refcounts rather than copying data." |
| `spec/SPECIFICATION.md:3370-3373` (ES-17, `[PROVISIONAL]`) | "…the remaining cost is one `Box<str>` and one boxed tag slice, bounded by the tag count." |
| `references/adr/0012-append-shape-and-preconditions.md:173` | "`Bytes` are refcounted, so `event.clone()` bumps a counter rather than copying a payload." |

ES-17 is the clause that decides whether `EventStore::append` takes `&[Event]` or
`Vec<Event>`, and it is `[PROVISIONAL]` pending a measurement. The hazard the
review named is specific: a benchmark author building the harness the natural way
— tags from `Tag::from_static` constants, because that is what a test fixture
holds — measures a regime in which the clone costs **one** allocation no matter
how many tags an event carries, reports that the clone is immaterial, and freezes
`append`'s ownership on a number that no application will ever see.

So this experiment runs **both regimes side by side**. The `from_static` arm is
the point of the experiment, not an aside: it is reported beside the owned arm
rather than instead of it, precisely because it is the arm that would have been
written by accident.

## Conditions

Every figure is printed by the code that produced it, beside the settings it was
produced under. `results/raw/conditions.txt` is `rustc --version --verbose` read
back at the top of the same run that produced every other file in `results/raw/`
— not transcribed from this table.

| | |
| --- | --- |
| Machine | 13th Gen Intel Core i9-13905H, 20 logical cores, 31.7 GB RAM |
| OS | Windows 11 Home 10.0.26200 |
| Filesystem | NTFS, local NVMe (irrelevant here — nothing touches a disk) |
| Toolchain | `rustc 1.97.1 (8bab26f4f 2026-07-14)`, LLVM 22.1.6 |
| Host / target | `x86_64-pc-windows-msvc` |
| Build | `--release` for every timed run (`opt-level = 3`, `lto = false`, `codegen-units = 16`, stated in `Cargo.toml`); the conformance suite runs at `dev` |
| Allocator | `System`, wrapped by `counting::Counting` — **see the caveats** |
| Tree | `56ef6c5` |
| Command | `./run.sh` |
| Wall clock | about a minute, of which the million-event case is roughly seven seconds |

There is no pragma to read back off a live connection here — nothing in this
experiment opens a connection to anything. The equivalent obligation, and the
place a figure could be silently produced under the wrong settings, is the
`--release` flag and the allocator; both are read back in the output, the first
by the profile the numbers were produced under and the second by
`tests/measure_clone.rs`'s control on a single `Tag::clone`, which must be `1`
and `0` and would not be if the counter were miscounting.

## Conformance first, measurement second

**An arm that is fast and wrong wins every benchmark.** `run.sh` runs
`tests/arms_are_equivalent.rs` before it times anything, and it establishes four
things:

1. The two tag regimes build the **same event** — same type string, same 64 tag
   strings, same payload, `==` — at 0, 1, 8 and 64 tags, on both payload shapes.
2. They encode to **byte-identical** output, in `serde_json` and in `postcard`,
   for `Event` and for `SequencedEvent`. This is the control that stops the cheap
   arm winning by encoding less: without it, "the static arm made fewer
   allocations" would be equally consistent with "the static arm dropped the
   tags".
3. Both regimes **round-trip** in both formats, the postcard half **framed** with
   a sentinel — the same two-format matrix and the same framing trick
   `crates/happenstance-core/tests/wire.rs` uses, and for the reason its header
   gives: a field-count desynchronisation surfaces as a tidy parse error unless
   there are bytes after the value.
4. The read-path replicas are **faithful**. `readpath::read_before` is a
   transcription of `memory.rs:302-333`; it is compared against the real
   `MemoryEventStore::read` over the same `SequencedEvent` values across 160
   query/option combinations on a deliberately non-uniform store, and
   `read_after` is compared against it over the same grid. A replica that had
   drifted fails there rather than producing a number about code that does not
   exist.

## Why the instrument lives here and cannot live in `crates/`

`counting::Counting` is a `#[global_allocator]`, and it could not be a workspace
member for two independent reasons:

* `GlobalAlloc` cannot be implemented without `unsafe impl`, and the workspace
  root sets `unsafe_code = "forbid"`. A `deny` could be waived with an `expect`;
  **`forbid` cannot be overridden from inside the crate at all.**
* A global allocator is a per-binary singleton. Declaring one in
  `happenstance-core` would install it into every test binary in the workspace
  that links the crate.

And CF-34 applies as it does to every experiment: performance is measured by a
separate harness which is not part of the conformance bar. `run.sh` must never
become a gate step — a benchmark that can turn a merge red teaches people to
re-run until green.

## Findings

Full tables in `results/clone-cost.md` and `results/read-limit.md`, every figure a
row in `results/raw/` from one `./run.sh`. In one paragraph each:

**The clone costs `t + 2`, and the published figure is wrong in both
directions.** At VT-22's 64-tag floor, in the canonical `Tags::from_pairs`
regime, one `event.clone()` performs **66 heap allocations** requesting 2,001
bytes — one for the `EventType`'s `String`, one for the `Box<[Tag]>`, and one per
tag. The relation is exact at 1, 8, 32 and 64 tags. At **zero** tags it is
**one**, not two, because `<[T]>::to_vec()` on an empty slice allocates nothing —
so the "two allocations" figure overcounts a tagless event by one and undercounts
a 64-tag event by sixty-four.

**The trap arm behaves exactly as predicted, and it is the one a benchmark would
have used.** In the `Tag::from_static` regime the same clone is **one**
allocation at 1, 8, 32 and 64 tags — flat, because every `Cow::Borrowed` clones
as two words. A harness built that way reports a 66x-cheaper clone that does not
respond to tag count at all, and everything ES-17 would conclude from it would be
about a regime no application is in.

**The `serde` encode path doubles it, exactly, at every tag count in both
formats.** With the encoder's own allocations cancelled by the byte-identity
control, the owned-minus-static delta is `t + 1` for `Serialize for Event` and
`2(t + 1)` for `Serialize for SequencedEvent` — ratio **2.00** at 0, 1, 8, 32 and
64 tags, in `postcard` and in `serde_json` alike. In absolute terms: encoding one
64-tag `SequencedEvent` to postcard costs **140 heap operations to produce 587
bytes**, of which **130 (93%) are transient clones that produce no output.**

**`limit` buys the caller nothing on the read path.** A `limit(1)` read against a
fully-matching query on a store of a million two-tag events performs **4,000,020
heap allocations, requests 230 MB, and takes about 310 ms** to return one event —
which is `1.0000x` what reading all one million costs. With `.take(limit)` above
`.cloned()` the same read is **5 allocations, 655 bytes, 2 µs**. At the 64-tag
floor and only ten thousand events it is 660,013 allocations and 22 MB against 67
and 2,577 bytes. The allocation counts are identical to the digit across four
separate runs; the timings moved by up to 40% and are the weaker half of the
row.

**One thing the finding did not claim, found on the way.** `Bytes::clone` is a
refcount bump *from the second clone onwards*. A payload built with
`Bytes::from(Vec<u8>)` — the shape every decoded or generated payload has —
starts in `bytes`' promotable representation, and its **first** clone allocates a
shared header. One extra allocation, once per payload, paid by any store that
clones each appended event once. `Bytes::from_static` never pays it.

**And one correction to H3's own instrument.** `Cow<'static, str>` is **24
bytes**, not the 32 the finding reasoned: `String`'s capacity field carries the
discriminant, so the niche it assumed absent is present. A
`tests/layout_budget.rs` written from the finding's reasoned numbers would have
failed on the first `cargo test`. Measured: `Tag` 24, `EventType` 24, `Tags` 16,
`Event` 104, `SequencedEvent` 144, `QueryItem` 32.

## Caveats — what these numbers do not show

* **They are allocation *counts* and *requested* bytes, not time or resident
  memory.** `bytes` is the sum of `layout.size()`; the system allocator's real
  reservation, its bookkeeping and its fragmentation are all outside the
  instrument. Nothing here says how expensive an allocation *is* on this machine
  or on any other.
* **The wall-clock column was produced with the counting allocator installed**,
  which adds four relaxed atomic RMWs to every heap operation. That inflates
  allocation-heavy arms specifically, so the timing ratios in
  `results/read-limit.md` are an **upper bound** on the speed-up, and the
  allocation counts — which the shim cannot bias, because it counts rather than
  causes — are the figures to quote.
* **A million-event `MemoryEventStore` is not a production workload.** The store
  documents itself as "not built for scale" two lines above the sentence under
  review, and nobody is entitled to be surprised that it is slow. What the number
  answers is narrower and it is the thing that was actually claimed: what the
  snapshot costs, and whether `limit` reduces it.
* **`read_after` is not the shipped code.** It is one plausible spelling of H2's
  fix, checked for equal output over 160 combinations on one mixed store. That is
  evidence the change is behaviour-preserving on that grid; it is not a proof, and
  the conformance suite — which this crate does not run against a modified core,
  because it may not modify core — is where the real bar sits.
* **Windows, MSVC, one machine, one toolchain.** Allocation *counts* should be
  identical on any 64-bit target with the same `bytes`, `serde` and `postcard`
  versions, because they are a property of the code rather than of the allocator.
  The `size_of` figures are for `x86_64`; a 32-bit target's are different by
  construction. The timings are this laptop's.
* **The four documentation sites are quoted, not corrected.** This crate may not
  edit `crates/`, `spec/`, `.kb/` or `docs/`, so nothing here changes what those
  four sentences say. The measurement is what a correction would have to be
  written against.

## Layout

```
Cargo.toml                       empty [workspace]; happenstance-core by path, serde on
run.sh                           re-derives results/raw/ from a clean checkout. NOT a gate step.
src/counting.rs                  the counting #[global_allocator] and `measure`
src/arms.rs                      the two tag regimes, and the events they build
src/readpath.rs                  memory.rs:302-333 transcribed, and the same with `.take` moved
tests/arms_are_equivalent.rs     CONFORMANCE. runs first; nothing is timed until it passes
tests/measure_clone.rs           arms 1-4, plus the size_of table and the Tag control
tests/measure_read.rs            arms 5-6, plus the limit=None and Query::all controls
results/clone-cost.md            hand-written from results/raw/clone.txt
results/read-limit.md            hand-written from results/raw/read.txt
results/raw/                     untouched command output
```
