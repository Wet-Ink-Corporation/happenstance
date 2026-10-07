# Does `append` keep `&[Event]`? The ES-17 two-build measurement

ES-17 (`spec/SPECIFICATION.md:3563-3571`) is `[PROVISIONAL]` on one measurement:
*a real adapter showing the per-event clone is a material fraction of append
cost*. ADR-0012's falsifier (`references/adr/0012-append-shape-and-preconditions.md:244-266`)
says what that measurement must be — two builds of the same adapter differing only
in `append`'s ownership, at VT-24's batch floor with a realistic tag count, a
stated commit/reject mix, the adapter shape named, and a design for keeping a copy
for retry. ADR-0055 named the adapter: `happenstance-cloudflare`, the one shape in
the workspace that copies the payload into an owned row value. This directory is
that measurement.

## The decision rule, written before the run

Recorded here before the first measuring run, per weigh-in `wi-95d2b2`, so that the
threshold cannot be fitted to the result. Nothing below this section was known
when it was written.

**That claim is self-attested.** This directory was untracked until it was
committed, and the README's modification time was 05:39Z when the verdict was
written (on the run day), after the raw outputs' (05:26–05:31Z), because the verdict below was written into the same
file afterwards. Nothing in the repository proves the section came first. The
earliest records of the rule are outside this directory, and both precede the
first raw output by 27 minutes:

* the brief's suggested rule (`.temper/plans/p17-es17-brief.md`, "Decision rule",
  last written 04:59:41Z; `.temper/` is not committed, so it is quoted here):
  *"**Freeze `&[Event]`** if, at the realistic point (batch 128, any tag count,
  payload ≤ 16 KiB, `Vec`-backed regime), O1's saving over **B1** is under 10% of
  B1's median append time, or inside B1's IQR. Also freeze if the only gain over B0
  is one B1 already gets."*
* Weigh-In `wi-95d2b2`, defaulted in the session's Weigh-In ledger at 04:59:53Z,
  before the experiment began, as *"10% of B1 median, outside IQR, at batch 128,
  payload <= 16 KiB (two-way)"*.

The rule below is that rule with its two clauses written as inequalities and the
nine cells named; it adds no condition the two records do not state.

**The realistic point** is: batch **128** (VT-24's floor), payload **64 B, 1 KiB or
16 KiB**, the **`Vec`-backed** payload regime (`Bytes::from(Vec<u8>)`, unique, so
an owned arm can move it), and **any** of the three tag counts 1, 8 and 64 — nine
cells in all.

**Freeze `&[Event]`** unless, in **at least one** of those nine cells, **both** of
these hold for the median wall time of one append (the call and the drop of the
batch, batch built outside the timed region):

1. **O1** (owned) beats **B1** (borrowed, one copy) by **more than 10% of B1's
   median**: `median(O1) < 0.9 × median(B1)`; **and**
2. O1's median lies **outside B1's interquartile range**: `median(O1) < Q1(B1)`.

Two refinements, also fixed in advance:

* **O1 is compared with B1, never only with B0.** B0 is the adapter as shipped;
  B1 is borrowed, one Rust copy per payload (the `to_binding` clone in
  `crates/happenstance-cloudflare/src/sql_storage.rs:81-82` removed, which needs no
  signature change). A gain O1 shows over B0 that B1 also shows is an internal
  Cloudflare fix, recorded as a follow-up, and is not evidence about `append`'s
  signature.

  *Corrected after the run, on review:* this bullet called B1 "the best a borrowed
  batch can do", and that was false. `worker` 0.8.5's `SqlStorage::exec_raw(&self,
  query: &str, bindings: impl Into<Option<Vec<JsValue>>>)` (`sql.rs:220`) lets a
  borrowed batch bind `Uint8Array::from(&[u8])` and `JsValue::from_str(&str)`
  straight from the borrow, with no Rust copy of a payload, a type or a tag —
  fewer than O1, which still copies the type and the tags. The correction makes
  the freeze more robust, not less: O1 was held to a borrowed arm that was not
  the strongest one available, and still did not clear it.
* **Firing the rule is necessary, not sufficient, for a change.** ADR-0012 item 5
  still has to be met — a cheap way for a caller to keep a copy for retry — and the
  successor is `Vec<Event>` and nothing else (ADR-0012 `:266`). The caller-side
  figures below are the evidence for item 5 either way.

Allocation counts are the primary *descriptive* metric because they are
deterministic; the rule is on time because "a material fraction of append cost" is
a statement about cost, and an allocation that is cheap relative to a JS boundary
crossing is not material however many of it there are.

*Everything above this line was written before the first measuring run, by this
file's own account (self-attested, as said above), except the self-attestation
note and the marked correction, both added on review; everything below was
written after it.*

## The verdict the rule gives

**Freeze `&[Event]`.** In none of the nine decision cells did O1 beat B1 by
more than 10% with its median below B1's first quartile
([`results/realistic-point.md`](results/realistic-point.md)). The largest
apparent saving in the region, 11.5% (64 tags, 64 B), had O1's median inside
B1's IQR. The 64-tag samples are bimodal in every arm, and a reviewer's re-run
fired at different out-of-region cells than this one, so a gap of that size is
within what the clock moves here. (The static regime, where O1 and B1 make equal
allocation counts, shows gaps up to 19.6%; it is an allocation-count control,
not identical work, so it is consistent with that reading rather than proof of
it — corrected on review.)

The deterministic numbers say the same thing more plainly
([`results/allocations.md`](results/allocations.md)):

* Owning the batch saves **exactly 2 heap ops per event** (data and metadata) in
  the `Vec` regime, and **none** in the static regime: 12.5%, 3.9% and 0.6% of
  B1's heap ops at 1, 8 and 64 tags. It also saves the payload bytes; O1's
  requested bytes do not depend on payload size.
* Removing the `to_binding` clone (B0 → B1) saves **27–29%** of the adapter's
  heap ops and **25–50%** of its requested bytes at batch 128, depending on
  payload and tag count, **with no signature change**. It does not separate from
  noise in time either.
* An append here costs `batch × (1 + t) + 1` statements, each a JS crossing and a
  `prepare`. That cost, not the allocator, dominates.

And the caller-side figures ([`results/caller-side.md`](results/caller-side.md))
cut against item 5. A raw caller that resends under an owned `append` pays a
clone on every attempt, refusals included: **+90–95% heap ops** over borrowed on
Cloudflare. The clone also keeps the refcount above one, so O1's one saving turns
back into a copy. A typed-layer caller, which rebuilds every attempt, saves 2 ops
per *committed* event and nothing on a refusal: 0.2–3.8% of the simplified run
measured, which builds a fixed batch and fences with one statement. The real
command loop also reads, decodes, decides and encodes per attempt, so that share
is an upper bound.

**Recommended follow-up, not this measurement's to make:** bind through
`SqlStorage::exec_raw` (`worker-0.8.5/src/sql.rs:220`) inside
`happenstance-cloudflare`, handing it `Uint8Array::from(&[u8])` and
`JsValue::from_str(&str)` built from the borrowed batch. That removes every Rust
copy of a bound value — more than B1's consuming bind removes, and more than O1
does — with no public-API change and no signature change. B1, the smaller step
through the published `exec`, is the fallback. Either is an internal finding,
recorded as the rule requires and not as evidence about `append`'s signature;
neither `exec_raw` arm nor its allocation count was measured here.

## What was measured, and how

| Arm | What it is | Where |
| --- | --- | --- |
| adapter | the real `CloudflareEventStore::append` | `happenstance-cloudflare` at the base commit |
| **B0** | `write_rows` + `write_tag_rows` + stamp, verbatim, through the published `SqlStorage::exec` | `src/cloudflare.rs` |
| **B1** | the same statements, bound without the `to_binding` clone; borrowed batch, one copy per payload | `src/cloudflare.rs`, `src/cloudflare/binding.rs` |
| **O1** | `append_owned(Vec<Event>, …)`; `into_parts`, then `Vec::from(Bytes)` moved into the binding | `src/cloudflare.rs` |

Replica arms, not a feature-gated copy of the adapter (weigh-in `wi-8b2786`), so
nothing under `crates/` changed. `event_store_benchmarks!` could not have driven
this: it is compiled out on `wasm32` (`crates/happenstance-testkit/src/lib.rs:438`).

**Conformance before timing** (`tests/cloudflare.rs`, `tests/host.rs`). Each test
names the wrong implementation it rejects, and each of the four calibration
claims was shown to fail against a deliberately broken arm before it was trusted:
an O1 that drops metadata, an O1 whose "move" copies, a B1 that skips the
cursor's bookkeeping, and a B0 that grows a `Vec` the adapter pre-sizes.

* every arm's rows read back through the **real** `CloudflareEventStore::read`
  equal the batch as whole `Event`s, in both regimes;
* adapter and B0 allocate identically (heap ops and bytes), at every tag count,
  in both regimes, and at all 72 sweep points;
* B0 − B1 is exactly the binding clones, `batch × (4 + 2t) + 1`, and their bytes;
* O1 − B1 is exactly `2 × batch` and the moved bytes in the `Vec` regime, and 0
  in the static regime;
* `Vec::from(Bytes::from(vec))` keeps the pointer and allocates nothing, on the
  host and on wasm32.

**Sweep.** Batch 1, 16, 128 × tags 1, 8, 64 (the boundary tag included; built by
`Tags::from_pairs`, never `from_static`) × payload 64 B, 1 KiB, 16 KiB, 256 KiB ×
payload regime `Vec`-backed, static. Every event carries 32 bytes of metadata.
For each point there is one fresh object and 3 warm-up + 21 measured repetitions.
The four arms run interleaved, in an order rotated by the repetition. Each
repetition builds a fresh batch outside the timed region and times the append
plus the drop of the batch. Rows are deleted between repetitions, outside the
region.

**Contention.** k = 8, one boundary tag, 36 attempts / 8 commits / 28
rejections per run (`src/contention.rs`), batch 128, 1 KiB, `Vec` regime, tags 1,
8, 64; five caller behaviours; 2 warm-up + 21 measured runs.

**Instrument.** `event-clone-allocations`'s counting `#[global_allocator]`,
linked by path (`Cargo.toml` explains why it is borrowed, not copied); this crate
writes no `unsafe`. Time is `performance.now()` through `js_sys::Reflect` on
wasm32 and `Instant` on the host. Quartiles are nearest-rank.

## Conditions

| | |
| --- | --- |
| Machine | 4 vCPU (`nproc`) `Intel(R) Xeon(R) Processor`, MemTotal 16,480,968 kB (`free -g`: 15 GiB), a shared container: other builds may have run beside it |
| OS | Linux 6.18.44-fc-v70 x86_64 |
| Toolchain | rustc 1.97.1 (8bab26f4f 2026-07-14), LLVM 22.1.6 |
| Profile | `--release`: `opt-level = 3`, `lto = false`, `codegen-units = 16`, `debug = false` (`Cargo.toml`) |
| wasm32 | `wasm32-unknown-unknown`, `wasm-bindgen-test-runner` 0.2.126, Node v24.21.0 |
| Host for the object | `happenstance_cloudflare::host::DurableObjectHost` — `node:sqlite` in memory, reached through `worker`'s real bindings. **Not workerd**, and not a deployed object |
| Locked | `bytes` 1.12.1, `worker` 0.8.5, `wasm-bindgen` 0.2.126, `js-sys` 0.3.103, `wasm-bindgen-test` 0.3.76 |
| Base | `896d48c` |
| Run | 2026-10-07, `results/raw/conditions.txt` |

## What this does not measure

* **workerd or a deployed Durable Object.** The host is Node's SQLite. The
  statement cost there is not workerd's, and the ratio of statement cost to
  allocation cost is what the verdict rests on. `harness/workerd` could carry
  these arms; it has not.
* **Postgres**, the second owning adapter (`insert_batch`'s `to_owned`/`to_vec`
  at `crates/happenstance-postgres/src/event_store.rs:1028-1030`). It was optional
  and was not taken. Whether sqlx 0.8's `QueryBuilder<'args>` can bind those
  columns borrowed, an internal fix with no signature change, is unchecked.
* **SQLite and Neon**, which are null by construction: SQLite binds from the
  borrow, and Neon's base64 encoding is a copy either way (`p17-es17-brief.md`,
  "Adapter census").
* **The adapter's own `evaluate`.** The contention scenario's condition is a
  one-tag `Fence`, the same single statement in every arm.
* **The compensating discard** on a failed batch; no measured batch fails.
* **A clock finer than the noise.** At 64 tags every arm's samples are bimodal
  (near 70 ms and near 97–100 ms), and a re-run moves which out-of-region cells
  fire; effects of ~20% or less are not resolved in time and are visible only in
  the allocation counts. (The static regime's gaps of up to 19.6% are an
  allocation-count control, not identical work, so they do not set that floor
  by themselves.)

## Running it

```console
$ cd experiments/append-batch-ownership
$ export NVM_DIR=/opt/nvm; . "$NVM_DIR/nvm.sh"; nvm use 24
$ ./run.sh
```

`run.sh` needs `wasm-bindgen-test-runner` 0.2.126
(`cargo install wasm-bindgen-cli --version 0.2.126 --locked`) and Node 24; its
header says why. It writes `results/raw/*.txt`, and the tables in `results/*.md`
are written by hand from those rows. About six minutes on the machine above. Not
a gate step (CF-34), and the empty `[workspace]` table keeps it out of
`cargo xtask ci`'s reach.

A plain `cargo test` passes on the host. The counting allocator's counters are
process-global, so every counting test takes `region::hold()` (`src/region.rs`)
first and measures through the guard it returns; before that lock, three of five
exact-count assertions in `tests/host.rs` failed under the default parallel
harness. `run.sh` still passes `--test-threads=1`, because the lock keeps tests
out of each other's regions but not the harness's own threads. On wasm32 there is
one thread and the question does not arise.

The `region::hold` lock and the F8 bytes assertion in the host tests were added
after the recorded run, so the `results/raw` conformance logs (05:26Z) predate
both. The measured code paths are unchanged: `Region::measure` delegates to
`counting::measure`, and `run.sh` used `--test-threads=1`.
