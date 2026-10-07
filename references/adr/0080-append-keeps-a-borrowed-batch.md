# ADR-0080 — `append` keeps its borrowed batch, and ES-17 is frozen on the two-build measurement

- **Status:** accepted.
- **Date:** 2026-10-07
- **Phase:** 17 (the breaking window), lane L7.
- **Decided by:** the owner, in advance: *"If the evidence says freeze `&[Event]`, write the
  record, and the lane can merge."* The evidence says so under a rule written before the run.
  This record states that outcome; it takes no decision beyond it.
- **Rests on:** [ADR-0012](../../.kb/decisions/0012-append-shape-and-preconditions.md), whose
  falsifier this measurement is (`references/adr/0012-append-shape-and-preconditions.md:244-266`),
  and [ADR-0055](../../.kb/decisions/0055-append-keeps-its-borrowed-batch-at-0-2-0.md), which named
  the adapter to take it on. Both are accepted and stay accepted.
- **Supersedes:** nothing. **Closes** `kb-open-question-es-17-two-adapter-measurement-001`.
- **Evidence:** `experiments/append-batch-ownership/` — its README, `results/realistic-point.md`,
  `results/allocations.md`, `results/caller-side.md` and `results/raw/*.txt`, run on 2026-10-07 at
  base `896d48c`.

---

## 1. The question

ES-17 (`spec/SPECIFICATION.md:3563-3595`) says `EventStore::append` MUST continue to take
`events: &[Event]`. It was `[PROVISIONAL]` on one measurement: *a real adapter showing the
per-event clone is a material fraction of append cost.* ADR-0012 said what that measurement must
be (`:244-266`):

1. two builds of the same adapter, differing only in `append`'s ownership, on one harness;
2. a batch at VT-24's floor of 128, with a realistic tag count;
3. a stated commit and rejection mix;
4. the adapter shape named — one that moves the payload into an owned row it keeps, or one that
   binds from a borrow, because only the first can benefit;
5. a design for the marker's second half, *a cheap way for a caller to keep a copy for retry*,
   without which a saving in the adapter is not a saving in the system.

If it fired, the successor was `Vec<Event>` and nothing else (`:266`). ADR-0055 restated the
subject: not SQLite, which binds from the borrow and is null by construction, but
`happenstance-cloudflare`, whose `write_rows` copies type, payload and metadata into owned
`SqlValue`s per event (`crates/happenstance-cloudflare/src/event_store.rs:831-836`).

Phase 16 made ES-17 `freeze-by-17`, because the other answer changes `append` on
`happenstance-core`, the four published adapters, the testkit's own stores and the typed layer,
and only `0.4.0` can carry that.

## 2. Why the shape matters, for a reader coming from C#

`&[Event]` is a **shared borrow of a slice**. The caller keeps ownership of its `Vec<Event>`; the
adapter may read every event for the duration of the call and may not move anything out of it.
To keep an event, an adapter must `clone` it. The nearest C# picture is a `ReadOnlySpan<Event>`
argument — except that the compiler, not a convention, guarantees the callee keeps nothing.

`Vec<Event>` is a **move**. The caller gives the batch away; after the call the variable is
unusable, and the compiler refuses any later use. The adapter owns each `Event` and may call
`Event::into_parts` (`crates/happenstance-core/src/event.rs:420`) to take its fields apart without
copying them. Under `&[Event]` that method is unreachable from any `impl EventStore`, which is the
fact ES-17's second paragraph records.

What an owned batch could save depends on **`Bytes`**, the type of `data` and `metadata`
(`event.rs:323`, `:325`). A `Bytes` is a reference-counted view of a buffer, so `clone` bumps a
counter rather than copying the payload. Two details decide the numbers below:

- **Promotion.** A `Bytes` made with `Bytes::from(Vec<u8>)` starts in a unique, `Vec`-backed
  representation. Its *first* clone promotes it to the shared one, which allocates a small
  header. Later clones allocate nothing.
- **Recovery.** `Vec::from(Bytes)` on a unique, `Vec`-backed `Bytes` hands the original
  allocation back with no copy (measured: same pointer, zero allocations, on the host and on
  wasm32). If the refcount is above one — because anybody cloned it — it must copy.

The rest of an `Event` is not refcounted. `EventType` and each `Tag` are `Cow<'static, str>`, and
`Tags` is a `Box<[Tag]>`, so a clone of an event built at run time costs `t + 2` heap operations
(the type, the boxed slice, one per tag). Core has no `EventType → String` or `Tag → String`
conversion, so an owned batch cannot move those into a row either; every arm copies them.

And `ConditionViolated` returns no batch. Under `Vec<Event>` a caller that wants to try again
with the same events must clone *before* every attempt, because a refusal consumes the batch and
gives nothing back. That clone also raises the payload's refcount above one, so the adapter's
`Vec::from(Bytes)` copies after all.

The two other shapes stay foreclosed by ADR-0012 (`:93-160`), and this record does not reopen
them. An owned `EventBatch` newtype is forbidden by VT-24. `impl IntoIterator<Item = Event>` is
a generic method: Rust compiles one copy per iterator type, and a `dyn` vtable holds one function
pointer per method, so there is no single address to store and the trait stops being
dyn-compatible (`error[E0191]` under `dynosaur`). The choice was only ever `&[Event]` or
`Vec<Event>`.

## 3. What was measured

`experiments/append-batch-ownership/`, an out-of-workspace crate. Four arms, interleaved in an
order rotated per repetition:

| Arm | What it is |
| --- | --- |
| adapter | the real `CloudflareEventStore::append`, at the base commit |
| **B0** | `write_rows` + `write_tag_rows` + the stamp, verbatim, through the published `SqlStorage::exec` |
| **B1** | the same statements bound without `SqlValue::to_binding`'s clone (`crates/happenstance-cloudflare/src/sql_storage.rs:81-82`); borrowed, one Rust copy per payload, and not the strongest borrowed shape (§4) |
| **O1** | `append_owned(Vec<Event>, …)`: `into_parts`, then `Vec::from(Bytes)` moved into the binding |

**The sweep:** batch 1, 16, 128 × tags 1, 8, 64 × payload 64 B, 1 KiB, 16 KiB, 256 KiB × payload
regime `Vec`-backed or static: 72 points, each on a fresh object, 3 warm-up and 21 measured
repetitions, each building its batch outside the timed region and timing the append plus the
batch's drop. wasm32 under Node 24, against `happenstance_cloudflare::host::DurableObjectHost`
(`node:sqlite` in memory, reached through `worker` 0.8.5's real bindings). Every event carries 32
bytes of metadata, and tags are built by `Tags::from_pairs`, never `from_static`.

**The contention scenario** (ADR-0012 item 3): k = 8 contenders on one boundary tag, 36 attempts,
8 commits and 28 rejections per run, asserted identical in every repetition; batch 128, 1 KiB,
tags 1, 8 and 64; five caller behaviours (§5).

**Conformance before timing.** Every arm's rows, read back through the real
`CloudflareEventStore::read`, equal the batch as whole `Event`s. The adapter and B0 allocate
identically, heap operations and bytes, at all 72 points. B0 − B1 is exactly the binding clones,
`batch × (4 + 2t) + 1`. O1 − B1 is exactly `2 × batch` in the `Vec` regime and 0 in the static one.
Each of those four claims was shown to fail against a deliberately broken arm before it was
trusted.

## 4. The rule, written before the run

Recorded in the experiment's README before the first measuring run (Weigh-In `wi-95d2b2`), so the
threshold could not be fitted to the result.

**That precedence is self-attested.** The experiment directory was untracked until it was
committed, and the README's modification time (05:39Z) is after the raw outputs' (05:26–05:31Z),
because the verdict was written into the same file afterwards; nothing in the repository proves
the rule came first. The earliest records of it are outside the directory, and both are 27
minutes older than the first raw output. One is the lane brief's suggested rule
(`.temper/plans/p17-es17-brief.md`, "Decision rule", last written 04:59:41Z, not committed):
*"Freeze `&[Event]` if, at the realistic point (batch 128, any tag count, payload ≤ 16 KiB,
`Vec`-backed regime), O1's saving over B1 is under 10% of B1's median append time, or inside B1's
IQR."* The other is Weigh-In `wi-95d2b2`, defaulted in the session's Weigh-In ledger at 04:59:53Z,
before the experiment began, as *"10% of B1 median, outside IQR, at batch 128, payload <= 16 KiB
(two-way)"*. The README quotes both.

**The realistic point** is batch 128, payload 64 B, 1 KiB or 16 KiB, the `Vec`-backed regime, and
any of tags 1, 8 and 64: nine cells. **Freeze `&[Event]`** unless in at least one of them both

1. `median(O1) < 0.9 × median(B1)`, and
2. `median(O1) < Q1(B1)`

hold for the time of one append. **O1 is compared with B1, never only with B0**: a gain O1 shows
over B0 that B1 shows too is an internal Cloudflare fix, not evidence about the signature. And
firing the rule was to be necessary, not sufficient: item 5 would still have to be met.

**B1 is not the strongest borrowed arm,** though the README's pre-run text called it "the best a
borrowed batch can do"; the README now marks that as corrected on review. B1 is borrowed with one
Rust copy per payload. `worker` 0.8.5 also publishes `SqlStorage::exec_raw(&self, query: &str,
bindings: impl Into<Option<Vec<JsValue>>>)` (`sql.rs:220`), through which a borrowed batch binds
`Uint8Array::from(&[u8])` and `JsValue::from_str(&str)` with no Rust copy of a payload, a type or
a tag — fewer than O1, which still copies the type and the tags. The correction cuts one way: O1
was judged against a borrowed arm weaker than one that exists, and still did not clear it, so the
freeze is more robust for it, not less. `exec_raw` was not measured as an arm (§8).

## 5. What it found

**0 of 9 cells fire.**

| tags | payload | B1 median [Q1, Q3] µs | O1 median µs | O1 saving vs B1 | O1 < Q1(B1)? |
| ---: | ---: | --- | ---: | ---: | :---: |
| 1 | 64 B | 3,658 [3,478, 3,886] | 3,665 | −0.2% | no |
| 8 | 64 B | 11,101 [10,786, 11,743] | 11,010 | 0.8% | no |
| 64 | 64 B | 96,041 [80,339, 108,091] | 84,949 | 11.5% | no |
| 1 | 1 KiB | 4,179 [4,072, 4,212] | 4,213 | −0.8% | no |
| 8 | 1 KiB | 12,027 [10,911, 13,412] | 11,256 | 6.4% | no |
| 64 | 1 KiB | 72,879 [69,235, 99,595] | 95,189 | −30.6% | no |
| 1 | 16 KiB | 5,569 [5,351, 6,335] | 5,404 | 3.0% | no |
| 8 | 16 KiB | 12,432 [12,339, 12,843] | 12,541 | −0.9% | no |
| 64 | 16 KiB | 101,152 [74,153, 102,563] | 98,733 | 2.4% | no |

The largest apparent saving, 11.5%, had O1's median inside B1's interquartile range. Two controls
the sweep carries say why that clause was needed. The 64-tag rows are bimodal in every arm (near
70 ms and near 97–100 ms), so a median can jump between modes. And the static regime is a placebo:
there O1 makes exactly B1's heap operations and bytes at all 36 points, yet its median moved by up
to 19.6% (64 tags, 64 B), never below B1's first quartile.

**The deterministic numbers say the same thing more plainly.** Heap operations per append, batch
128, identical in all 21 repetitions of every row:

| tags | adapter = B0 | B1 | O1 (`Vec`) | B1 − O1 | O1's saving as % of B1 |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 2,825 | 2,056 | 1,800 | 256 | 12.5% |
| 8 | 9,097 | 6,536 | 6,280 | 256 | 3.9% |
| 64 | 59,273 | 42,376 | 42,120 | 256 | 0.6% |

Owning the batch saves **exactly 2 heap operations per event** — one buffer each for data and
metadata, the only things it can move — and none in the static regime. It also saves the payload
bytes: O1's requested bytes do not depend on payload size at all. But an append here costs
`batch × (1 + t) + 1` statements, each a JS boundary crossing and a `prepare`, and that, not the
allocator, is where the time goes.

**Item 5 cuts the other way.** Heap operations per contended run (k = 8, 36 attempts):

| tags | B1, resend one `&[Event]` | O1, clone before every attempt | B1, rebuild per attempt | O1, rebuild per attempt |
| ---: | ---: | ---: | ---: | ---: |
| 1 | 16,817 | 32,725 | 53,897 | 51,849 |
| 8 | 52,657 | 100,821 | 191,113 | 189,065 |
| 64 | 339,377 | 645,589 | 1,265,801 | 1,263,753 |

A raw caller that resends under an owned `append` pays **+90–95% heap operations** over the
borrowed shape, and more bytes (1,890,736 against 1,437,616 at one tag), because its clone keeps
the refcount above one and O1's one saving turns back into a copy — paid on all 36 attempts,
including the 28 that write nothing. A caller that rebuilds its batch per attempt saves exactly
2,048 operations (8 commits × 128 events × 2 buffers), 0.2–3.8% of the run, and nothing on a
refusal. No caller's median time separated from its counterpart's. On the reference store, where
the clone *is* the write path and which ADR-0012 rules out as unrepresentative, the raw owned
caller was 4.2–6.6× slower and the rebuilding owned caller 2–11% faster: the ceiling on what
ownership could buy, and a ceiling the representative adapter does not approach.

So none of items 1 to 5 points at `Vec<Event>`, and item 5 points away from it.

## 6. What ADR-0012 and ADR-0055 could not say

Both records are accepted and immutable. Four things they state, or assume, are not so, and this
record is where that is written.

**(a) The harness ADR-0055 named cannot drive this adapter.** ADR-0055 scheduled the two builds
"both driven through `happenstance_testkit::event_store_benchmarks!`". That macro's module is
compiled only under `#[cfg(all(feature = "bench", not(target_arch = "wasm32")))]`
(`crates/happenstance-testkit/src/lib.rs:438`), and `happenstance-cloudflare` runs on wasm32. The
measurement therefore used **calibrated replica arms** (Weigh-In `wi-8b2786`) rather than a
feature-gated copy of the adapter, so nothing under `crates/` changed. B0 is the shipped write
path, and the claim that it is was checked rather than assumed: adapter and B0 allocate
identically, operations and bytes, at **all 72** sweep points, and the test that holds it went red
when B0 lost one `with_capacity`.

**(b) Postgres copies too.** ADR-0055's census added `happenstance-cloudflare` to ADR-0012's
memory-only list and stopped. `happenstance-postgres`'s `insert_batch` also copies into owned
values per event: the type by `.to_owned()`, data and metadata by `.to_vec()`, and each tag into a
`Vec<String>` (`crates/happenstance-postgres/src/event_store.rs:1025-1030`). It was not measured
(§8). Its data and metadata are what an owned batch could move there; whether sqlx 0.8's
`QueryBuilder<'args>` can bind them borrowed instead, with no signature change, is unchecked.

**(c) ADR-0055's correction was half done.** Its body says the four sites stating a clone's cost
as "one `Box<str>` and one boxed tag slice" were all corrected. Two were, both in
`happenstance-core` (`event.rs` and `memory.rs`); the specification and the ADR-0012 long form
were not (`experiments/event-clone-allocations/README.md:19-26`, "Two of the four are now
corrected"). This record's change to ES-17 corrects the specification's (§7). The ADR-0012 long
form stays as written, because the policy on correcting `references/adr/` in place is still the
open question `kb-open-question-references-adr-correction-policy-001` names.

**(d) ES-17's third ground binds fewer callers than it says.** The ground reads: `ConditionViolated`
obliges the caller to keep its events, so by-value moves the clone onto the caller's every path.
The typed layer's command loop does not keep its events. It re-reads, re-decides and re-encodes a
new batch on every attempt (`crates/happenstance/src/command.rs:427`; the batch is built at
`:468` and used once at `:473`), so an owned `append` would cost it no copy. The ground binds
only a **raw-port caller that resends one batch**, and the contention table above shows what it
costs that caller. The freeze does not rest on the ground holding everywhere: the typed caller's
saving under `Vec<Event>` was 2 operations per committed event and no measurable time.

## 7. Decision

1. **`EventStore::append` keeps `events: &[Event]`, and ES-17 is `[FROZEN]`.** From `1.0.0` a
   change to it is a major release of `happenstance-core` and of every adapter.
2. **The marker's falsifier text is replaced** by the measurement's outcome and its reopening
   conditions (§9). The SQLite multi-row insert benchmark, which ADR-0055 showed names the one
   shape that cannot fire it, is gone from the clause.
3. **The clause's stale prose is corrected, line for line.** The clone cost reads `t + 2` heap
   operations rather than "one `Box<str>` and one boxed tag slice"; the clone it cites is at
   `crates/happenstance-core/src/memory.rs:402-413` (was `:388-400`), the early return at
   `:386-397` (was `:377-383`); and the third ground is stated for the raw caller, with the typed
   loop's per-attempt rebuild cited.
4. **No API changes.** No rule changes either: `append_preserves_event_payload` passes under
   either shape and stays ES-17's rule, because ownership is a cost question a conformance rule
   cannot see (ADR-0012, "No rule in the suite fails if the ownership changes").
5. **ES-7 is not frozen here.** The 1.0 dispositions say ES-7 is frozen "in the record that
   answers `trait-variant-caret-resolves-past-the-locked-gate` and ES-17's ownership". ES-7 is
   `freeze-by-17b` since ADR-0072 and its own falsifier is the `trait-variant` caret question,
   which this measurement does not touch. It stays with that record.

## 8. What was not measured

- **`workerd` or a deployed Durable Object.** The host is Node's SQLite. The statement cost there
  is not `workerd`'s, and the ratio of statement cost to allocation cost is what the verdict rests
  on. `harness/workerd` could carry the same arms and has not.
- **Postgres**, the second owning adapter ((b) above). It was optional and not taken.
- **SQLite and Neon**, null by construction: SQLite binds from the borrow, and Neon's base64
  encoding into its JSON body is a copy under either shape.
- **The adapter's own `evaluate`.** The contention scenario's condition is a one-tag fence, the
  same single statement in every arm.
- **The compensating discard** on a failed batch; no measured batch fails.
- **Anything under the clock's noise floor.** The placebo moves medians by up to about 20%, so
  effects below that are visible only in the allocation counts, which is why those are reported.
- **An `exec_raw` arm** (§4): the strongest borrowed shape, binding from the borrow with no Rust
  copy. Named as the follow-up (§10), not measured.
- **Cells outside the decision region as decision points.** The sweep prints the rule for all 72
  points; 63 lie outside the nine cells, and both conditions held in two of them, both `Vec`
  regime, no static placebo row among them. At batch 128, 8 tags, 256 KiB: B1 48,823
  [41,796, 50,088] µs against O1 41,570, a 14.9% saving, with its neighbours going the other way
  by similar amounts (−8.1% at one tag, −15.9% at 64) and the static placebo at the same payload
  moving 12.5% and 8.9% on identical work. At batch 1, 8 tags, 64 B: B1 130.5 [112.8, 144.4] µs
  against O1 112.5, a 13.8% saving on a single event whose arms differ by 2 of 59 heap operations.
  An independent reviewer's re-run of the sweep (its rows are not committed) fired at different
  cells — batch 1, 256 KiB, at 1 tag (19.5%) and at 8 tags (10.4%) — and not at batch 128, 8 tags,
  256 KiB. Cells that fire on one run and not the next, with identical allocation counts, are the
  clock. They are stated rather than buried, and the 256 KiB cells are the first thing a
  reopening would repeat (§9).

## 9. Falsifiers — what reopens it

ES-17 is frozen, so any of these reopens it as a decision record and, after `1.0.0`, as a major:

- **A `workerd` or deployed-object run of the same four arms** in which, at a realistic point,
  O1 beats B1 under the same pre-registered rule. That is the one condition under which the
  allocation the arms differ by could dominate the statements they share.
- **A realistic point with a large payload.** A domain whose ordinary events carry 256 KiB or more,
  and a repeat of the 256 KiB / 8-tag cell with enough repetitions that the rule fires again at
  the same cell, and at its neighbours.
- **An adapter shape where allocation is the write path**: a store that keeps the payload whole in
  owned memory, without a statement or a boundary crossing per row. The reference store is that
  shape and is ruled out; a shipped adapter of the same shape would not be.
- **Item 5 met.** If `AppendError::ConditionViolated` ever hands the batch back, or the contract
  gains another way to retry without a clone, the raw caller's +90–95% disappears and only the
  adapter's side is left to weigh.
- **Core gaining `EventType` and `Tag` conversions into `String`.** O1 could then move the type
  and the tags, not only two buffers, and its saving would scale with the tag count rather than
  stay at two operations per event.

## 10. A follow-up this record does not decide

**`happenstance-cloudflare` can drop every Rust copy of a bound value without any signature
change.** The stronger step is `SqlStorage::exec_raw` (`worker-0.8.5/src/sql.rs:220`): the adapter
builds `Uint8Array::from(&[u8])` and `JsValue::from_str(&str)` from the borrowed batch and binds
those, so no payload, type or tag is copied on the Rust side — fewer copies than O1 makes, from a
borrow. It was not measured. The smaller step, measured as B1, keeps the published `exec`:
B0 → B1 is a consuming bind inside the adapter — `SqlValue::to_binding` clones every `Text` and
`Blob` it binds, and nothing reads the `SqlValue` after. Removing that clone saves **27–29%** of
the adapter's heap operations (769, 2,561 and 16,897 per 128-event append at 1, 8 and 64 tags)
and **25–50%** of its requested bytes at batch 128, depending on payload and tag count. It did not
separate from noise in time either, so it is a memory finding, not a latency one. Either step is
internal, needs no record of this kind, and is left to a lane that changes the adapter. The
equivalent check for Postgres ((b)) is the same kind of follow-up.

## 11. Alternatives

- **`Vec<Event>` in `0.4.0`.** Rejected on §5: the rule did not fire, the adapter's saving was two
  operations per event, and a raw retrying caller would pay roughly double. ADR-0012 already
  priced the change at a new piece of public API besides the signature.
- **Renewing ES-17 `[PROVISIONAL]` past 1.0** against the `workerd` measurement. Rejected: the
  measurement ADR-0012 asked for was taken and did not fire, and a marker kept open after its
  own falsifier ran is the decoration the specification forbids. The `workerd` run is a reopening
  condition instead (§9), at the cost of a major if it fires.
- **Waiting for the `workerd` arms before deciding.** Rejected on the owner's pre-authorisation
  and on the window: a `Vec<Event>` change can only ship in `0.4.0`, and the gap between B1 and
  O1 that a different host would have to open is a gap the allocation counts bound.
