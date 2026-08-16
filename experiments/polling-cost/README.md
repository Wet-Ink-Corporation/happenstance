# The polling-cost measurement

[ES-32](../../spec/SPECIFICATION.md) forbids `EventStore` from growing a tail,
subscribe or notify method at 0.1: **consumers poll**. A deployment with N views
therefore performs N independent reads of the same log and has the same events
delivered to it N times. Its `[PROVISIONAL]` marker names a falsifier that has
never been evaluated, and says so in terms — *"the workspace has no benchmark
harness and a conformance rule cannot substitute for one, because complexity is a
benchmark and not an assertion."* This is that harness.

**The short answer.** In the arm where every view wants every event, the polling
runner delivers each event once per view: **delivery amplification of 32.00 at 32
views**, exactly the fan-out, with no economy of scale anywhere in the range. In
the arm where the views' queries do not overlap it is **1.00** at every fan-out —
the same events, delivered once. Read latency tracks the poll interval and
nothing else: the median observation lands within about 1% of the interval at
every fan-out from 1 to 32, and no backlog ever accumulated.

This directory is a reproducible experiment. It is **not** a workspace member, it
is not a `cargo xtask ci` step, it adds no dependency to any `crates/**` manifest,
and it holds no threshold. `cargo xtask ci` and `cargo xtask affected --base main`
behave identically whether or not this has ever been run.

---

## 1. What was measured, on what

### The measurement

| | |
|---|---|
| Headline | **delivery amplification 32.00** at 32 overlapping views; **1.00** disjoint |
| Runner | `happenstance::run_projection`, behind `unstable-projection`, called as published |
| Event store | `happenstance_core::MemoryProjectionStore`'s sibling, `MemoryEventStore` — an in-process `Vec` |
| Projection store | `happenstance_core::MemoryProjectionStore` (HS-P0010's), behind `memory` |
| Codec | `Json` |
| Chunk | 1024 events per `begin`/`commit` pair |
| Repeats | 2 per cell, raw rows committed |
| Seed | `0x5011C057`, written into every record |

### The machine, and the revision it measured

| | |
|---|---|
| Toolchain | `rustc 1.97.1 (8bab26f4f 2026-07-14)`, `x86_64-pc-windows-msvc`, LLVM 22.1.6 |
| Profile | `release` |
| Measured revision | `55a2370`, clean under `crates/` and the workspace manifests |
| Host | Intel64 Family 6 Model 186 (13th-gen mobile i9 class), 20 logical CPUs, Windows |
| RAM | not captured — the manifest records `0`, which means *unknown*, not *none* |

### The axes

| Axis | Values | Swept in |
|---|---|---|
| Fan-out (views) | 1, 2, 4, 8, 16, 32 | both phases |
| Log size (events) | 1 000, 10 000, 100 000 | amplification |
| Poll interval | 10 ms, 100 ms, 1 000 ms | staleness |
| Selectivity arm | *overlapping* (every view selects the whole log — ES-32's worst case) and *disjoint* (each view selects ≈1/N) | both phases |

**The trim, and its reason.** The declared grid is a full cross product of all
four axes. This pass runs it as **two phases** instead: amplification over
fan-out × log size × arm, staleness over fan-out × poll interval × arm at a fixed
1 000-event log. No axis value is dropped and neither arm is dropped — what is
dropped is the cross product between *log size* and *poll interval*, and the
reason is that neither quantity moves with the other axis. Replay amplification
does not move with the poll interval, because the runner resumes past its
checkpoint and an idle poll delivers nothing; staleness does not move with the
log size once the views have caught up. The full product would have cost the
staleness phase a factor of three in wall-clock — every 1 000 ms cell is paid in
real seconds — to record the same figures three times.

## 2. The numbers

**Delivery amplification** is the headline and it is a **count**, not a duration:
the events the store yielded across all views, divided by the distinct events
those deliveries carried. It is dimensionless and machine-independent, and it is
the quantity ES-32 is really about — a tail seam buys back exactly the redundant
deliveries.

*Overlapping arm — every view's query selects the whole log.*

| fan-out | log | delivered | distinct | amplification | reads |
|---|---|---|---|---|---|
| 1 | 100 000 | 100 000 | 100 000 | **1.00** | 2 |
| 2 | 100 000 | 200 000 | 100 000 | **2.00** | 4 |
| 4 | 100 000 | 400 000 | 100 000 | **4.00** | 8 |
| 8 | 100 000 | 800 000 | 100 000 | **8.00** | 16 |
| 16 | 100 000 | 1 600 000 | 100 000 | **16.00** | 32 |
| 32 | 100 000 | 3 200 000 | 100 000 | **32.00** | 64 |

The 1 000- and 10 000-event logs give the same ratios at the same fan-outs — the
amplification is a property of the fan-out, not of the log — and every row is in
`results/pass-001/records.ndjson`. … and 30 more amplification rows are in the
corpus rather than in this table.

*Disjoint arm — each view selects about one Nth.*

| fan-out | log | delivered | distinct | amplification | reads |
|---|---|---|---|---|---|
| 1 | 100 000 | 100 000 | 100 000 | **1.00** | 2 |
| 8 | 100 000 | 100 000 | 100 000 | **1.00** | 16 |
| 32 | 100 000 | 100 000 | 100 000 | **1.00** | 64 |

**Read count grows either way.** Note the last column: the number of `read` calls
is 2N in both arms — one delivering read per view plus one that confirms there is
nothing new. Selectivity buys back the *deliveries*; it buys back none of the
*reads*, and on a store where a read is a round trip rather than a `Vec` scan it
is the reads that will dominate. That distinction is the single most important
thing this measurement does not settle.

**Timings, secondary and machine-attached.** The 32-view, 100 000-event
overlapping cell — 3.2 million deliveries — took a mean of **6.3 s**, against
**138 ms** for the same log at one view. These figures belong to the host in §1
and to nothing else; they are recorded because they bound the shape of the
experiment, not because they are the result.

## 3. Staleness

Reported as two observed quantities, and **never** as `head() - checkpoint`.
Positions are an opaque ordering key and the specification permits gaps, so the
difference between two of them counts nothing; against a dense store it happens
to be right, which is exactly what would let it survive review.

- **Observation latency** — nanoseconds from an event's append instant to the
  instant an observing view's checkpoint reached or passed it.
- **Backlog at a poll boundary** — events appended and not yet observed, counted
  by *comparing* positions.

| poll interval | fan-out | p50 | p95 | max backlog |
|---|---|---|---|---|
| 10 ms | 1 | 10.7 ms | 12.0 ms | 0 |
| 10 ms | 32 | 12.0 ms | 12.9 ms | 0 |
| 100 ms | 1 | 100.8 ms | 101.9 ms | 0 |
| 100 ms | 32 | 102.5 ms | 104.8 ms | 0 |
| 1 000 ms | 1 | 1 000.6 ms | 1 001.1 ms | 0 |
| 1 000 ms | 32 | 1 008.5 ms | 1 013.6 ms | 0 |

… and 30 more staleness rows, both arms, in the corpus.

Latency is the poll interval and nothing else, at every fan-out this pass
reached, and no backlog ever accumulated: the deployment kept up. That is a
statement about **this** deployment — one process, an in-process `Vec`, four
appends per cell — and §4 is where it stops being one about any other.

## 4. What this does not prove

**This is a floor.** It is the first number that exists where there were only
estimates, and it bounds the cost from below. It does not reach either term of
ES-32's own falsifier, and both gaps are named here rather than left for a reader
to notice.

**"on a real deployment".** There is none. All six adapter crates carry
`publish = false` and not one has run the conformance suite; the store measured
here is an in-process `Vec` behind a `RwLock`, in one process, with no network,
no storage latency, no connection pool and no second machine. Every figure in §2
that involves time is therefore a lower bound by an unknown factor, and the read
count — 2N, in both arms — is the figure most likely to change character
entirely: on a store where a read is a round trip, N idle polls per interval is a
cost this harness charges nothing for. Reaching that term needs a durable
adapter, which is `sqlite-durable-store`'s (HS-P0012) to build.

**"their staleness budget".** Nobody has stated one. §3 reports what was
observed and judges none of it, because a budget this repository has never
written down cannot be met or missed. A reader who has a budget can compare it to
§3; this document will not do it for them.

**And two smaller ones.** The projections are deliberately near-zero-cost — one
counter and one row per commit — because the quantity under study is the
redundant *read* fan-out and read-model work would dilute it; a deployment whose
`apply` does real work will see a smaller amplification of *total* cost from the
same amplification of deliveries. And the amplification figures in §2 are exact
integers because the arms are exact: every overlapping view selects every event,
every disjoint view selects a disjoint shard. A real deployment's views overlap
partially, and this pass measures the two ends rather than the middle.

## 5. The verdict

At 32 views over one log, where every view wants every event, the polling runner
delivers each event **32 times** and issues **64 reads** where a tail seam would
deliver it once. Where the views do not overlap, the amplification is **1.00**
and the read count is still 2N. Observation latency is the poll interval.

That is the measurement. What it bears on is ES-32's `[PROVISIONAL]` marker,
whose falsifier this pass supplies evidence toward and does not settle: the
marker is not moved here and `spec/SPECIFICATION.md` is not edited by this
directory.

## 6. How to re-run it

```
bash experiments/polling-cost/run.sh
```

A Rust toolchain and nothing else. It prints the toolchain, the measured
revision and the host, builds `--release`, runs all 144 cells, and writes
`results/<tag>/records.ndjson` — one manifest line carrying the environment, then
one line per cell-repeat, conforming to `schema/`.

- `HS_TAG=pass-002 bash …/run.sh` writes a second pass **beside** the first. A
  tag that already exists is refused by name; nothing is ever edited in place, so
  undoing a re-run is deleting one directory.
- `HS_CELLS=6 bash …/run.sh` runs the first six cells, for a smoke check.

The whole sweep takes a few minutes, most of it paid by the 1 000 ms staleness
cells in real seconds and by the 32-view 100 000-event overlapping cell in
deliveries.

### Layout

| Path | What it is |
|---|---|
| `src/lib.rs` | the sweep, the measured projection, the record contract |
| `src/counting.rs` | the instrument: an `EventStore` that counts what it hands out |
| `src/observer.rs` | staleness, by comparison rather than subtraction |
| `src/progress.rs` | plain-line console output, legible piped to a file |
| `src/validate.rs` | the small checker `tests/schema.rs` holds the corpus to |
| `schema/` | the record and manifest schemas the corpus conforms to |
| `results/pass-001/` | the committed pass |
| `tests/` | tests **of the harness**, never of a cost |

`cargo test --manifest-path experiments/polling-cost/Cargo.toml` runs those
tests. They are not in the gate and must not be added to it: CF-34 says
performance is measured by a separate harness and that the harness is not the
bar.
