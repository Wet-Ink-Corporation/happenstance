# Every `Serialize` impl in `happenstance-core` deep-clones the value it was handed, and `SequencedEvent` does it twice. The fix is written, measured and byte-identical — does it land, and who repoints the four citations it moves?

Short answer up front: **land it.** It is measured rather than argued — 140 heap
operations to 8 for a 64-tag `SequencedEvent` in postcard, delta zero at every tag
count in both formats — it moves no byte, no signature and no feature, and every
mirror it touches is private. It is **not** landed here for a reason with nothing
to do with its merits: it renumbers three test functions that
`spec/SPECIFICATION.md` cites by line, and repointing those citations means
editing the specification, which this lane was not permitted to do.

The prototype is on `lane/core-ports` at **`2f11eb7`**, reverted in the commit
after it. Everything below was measured on it.

**This brief did not get the author → two-critic → revision pass the original
thirteen had.** Read it with that discount.

---

## Why this is owed

AE-2. Five hand-written `Serialize` impls build an *owned* wire mirror by cloning
each field out of a value they hold by reference:

```rust
impl Serialize for Event {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        EventWire {
            event_type: self.event_type().clone(),
            data: self.data().clone(),
            tags: self.tags().clone(),
            metadata: self.metadata().cloned(),
        }
        .serialize(serializer)
    }
}
```

`SequencedEvent` then clones the whole `Event` into `SequencedEventWire`, whose
derive calls the impl above, which clones all four fields again. `QueryItem`
(`self.types().into()`, `self.tags().clone()`), `Query` (`items.to_vec()`) and
`AppendCondition` (`guard.query.clone()`) are the same shape.

The `serde` feature exists in this crate for one consumer — the crate root says
it is *"enabled by replication adapters that need one"* — and a replicated event
is precisely one that carries tags.

---

## What is true today, measured

`experiments/event-clone-allocations/`, on rustc 1.97.1 x86_64-pc-windows-msvc,
release profile. The encoder's own allocations are cancelled by a byte-identity
control: the same value is built through `Tag::new` (`Cow::Owned`) and
`Tag::from_static` (`Cow::Borrowed`), the two emit identical bytes, and the
difference is the mirror's clone and nothing else.

**AE-2's claim reproduces exactly.** `delta(Event) = t + 1`,
`delta(SequencedEvent) = 2(t + 1)`, **ratio 2.00 at 0, 1, 8, 32, 64 and 128 tags,
in postcard and in serde_json alike.** The 128 row is new: the literal table ran
to 64, VT-22's floor, and now runs to `SqliteEventStore::MAX_TAGS_PER_EVENT`, the
only documented adapter ceiling in the tree.

| value | format | tags | out bytes | today | prototype |
| --- | --- | ---: | ---: | ---: | ---: |
| `Event` | postcard | 64 | 563 | 73 | **7** |
| `SequencedEvent` | postcard | 64 | 587 | **140** | **8** |
| `Event` | postcard | 128 | 1,132 | 138 | **8** |
| `SequencedEvent` | postcard | 128 | 1,156 | **269** | **9** |
| `Event` | serde_json | 128 | 1,445 | 136 | **6** |
| `SequencedEvent` | serde_json | 128 | 1,561 | 266 | **6** |
| `QueryItem` | postcard | 64 | 516 | 139 | **8** |

At the guaranteed minimums — VT-24's 128 events by VT-22's 64 tags — one batch
encode does **16,640** transient allocations that produce no bytes, and **0** on
the prototype. (AE-2 states 16,896; it used the *clone* cost `t + 2` where the
measured *encode delta* is `t + 1`, because the `Box<[Tag]>` allocation is
present in both arms and cancels. The ratio and the argument are unaffected.)

**No byte moves.** Checked, not asserted: the encodings of `Event`,
`SequencedEvent`, `Query` and `AppendCondition` were dumped at `701191b` and
again on the prototype, in both formats, at one tag and at sixty-four — 4,124
bytes at the large size — and are identical in every case. The four small vectors
are now pinned permanently in
`crates/happenstance-core/tests/serialize_is_borrowing.rs`, which is landed and
green in both worlds.

---

## The question

**Does the borrowing-mirror change land, and who repoints the citations it
moves?**

### Option A — land the prototype and repoint the four citations

Cherry-pick `2f11eb7` and make four one-line edits to `spec/SPECIFICATION.md`.

- **Caller:** nothing. No signature, no feature, no byte.
- **Adapter author:** nothing. The mirrors are private, inside
  `#[cfg(feature = "serde")] mod serde_impls`.
- **Semver:** none, in either direction.
- **Cost of being wrong:** a wire-format change nobody notices. That is the real
  hazard and it is why the golden vectors were pinned *before* the fix rather
  than after: postcard writes a struct as its fields in declaration order with no
  names, so a mirror whose fields are reordered still round-trips against itself
  and decodes nothing a deployed peer wrote.

### Option B — land it and let the citations drift within tolerance

Not available. `spec-trace` allows twelve lines and the shift is thirty and
sixty-six.

### Option C — restructure the crate so the added lines sit below the cited ones

Move the borrowing mirrors into a module at the foot of each file, after
`mod tests`, so that only a `use` line is added above the anchors.

- Costs nothing at runtime and keeps the citations still.
- Rejected, and recorded because it is the tempting one: it shapes the module
  layout of a published crate around a checker's line arithmetic, which is the
  trade this repository refuses everywhere else. The citations are the thing that
  should move.

### Option D — do not land it

- **Cost:** a sync runner pushing one batch at the crate's own floors does 16,640
  transient allocations that produce no bytes, and the target it lands hardest on
  is the memory-constrained one. `happenstance-sync` does not exist yet, which is
  why nothing has noticed.

---

## Recommendation, and the strongest argument against it

**Option A.**

The strongest argument against it, in its own words: *the prototype was written
by a lane that then reverted it, so nobody has reviewed the version that would
land; a five-site rewrite of a published crate's wire path is not a thing to
cherry-pick on the strength of a green test run, and the one instrument that
would catch a reordered field is a golden vector the same lane wrote.*

That is fair, and the answer is that the golden vectors are landed and independent
of the fix: they are pinned against the bytes the **pre-fix** code produced, and
they pass today. Whoever lands the prototype runs them against it and gets the
comparison without having to trust this brief. The full 64-tag comparison is
reproducible the same way — the dump is eight lines of `println!` over the
helpers already in that file.

**Confidence: high on the measurement, high on byte-identity, medium on the
review status of the diff.**

---

## Cost of delay

None to semver — nothing here is public. The deadline is `happenstance-sync`:
the crate that would notice does not exist, which is both why the defect survived
and why fixing it now costs nobody anything.

---

## What this does not settle

- **The wasm32 residency delta.** AE-2 asks for it and this experiment cannot
  supply it: `heap_ops` is `alloc` + `realloc` and `bytes` is bytes *requested*,
  both measured on the host. The instrument AE-2 proposed — a tagged `attempt` in
  `crates/happenstance-cloudflare/tests/wf11_memory_ceiling.rs` comparing
  `peak_pages` — was not run and is still owed.
- **`Serialize for Tags` and `Serialize for Tag`.** Already allocation-free;
  untouched.
- **`tests/measure_read.rs`.** Red for an unrelated reason recorded in
  `results/read-limit.md`: H2's fix landed in `MemoryEventStore::read` and three
  assertions there tie the real store to the *before* replica. Not this lane's
  file.

---

## Handoff

**1. The change.** Cherry-pick `2f11eb7` from `lane/core-ports` — or re-derive it;
it is a borrowing mirror per site with the same `rename`, the same fields in the
same order and the same payload routing:

- `crates/happenstance-core/src/event.rs` — `EventRef<'a>` and
  `SequencedEventRef<'a>`; `payload::Encode<'a>` lifted out of
  `payload::optional` so both the `Bytes` and the `Option<Bytes>` field can use
  it. `SequencedEventRef::event` is an `EventRef`, **not** an `&Event`: `&Event`
  would forward to `Serialize for Event` and pay the copy once, which is half the
  defect.
- `crates/happenstance-core/src/query.rs` — `QueryItemRef<'a>` with
  `types: &'a [EventType]`, and `QueryRef<'a>` with `Items(&'a [QueryItem])`.
- `crates/happenstance-core/src/append.rs` — `GuardRef<'a>`, `WireRef<'a>`, and a
  `GuardsRef<'a>(&'a [Guard])` whose `Serialize` is `collect_seq` over the slice.
  `collect_seq` takes its length from the iterator's `size_hint`, which a slice
  iterator supplies exactly, so postcard writes the length prefix the `Vec` wrote.

**2. The four citations**, repointed by anchor rather than by offset. Each is the
same test function, thirty or sixty-six lines further down:

| File | Line | Reads | Becomes | Anchor |
| --- | ---: | --- | --- | --- |
| `spec/SPECIFICATION.md` | 1764 | `` `zero_limit_means_zero_events` (`query.rs:523-536`) `` | `` (`query.rs:553-566`) `` | `fn zero_limit_means_zero_events` moves 524 → 554 |
| `spec/SPECIFICATION.md` | 1801 | `` `to_is_recorded_and_independent_of_from`\n(`query.rs:538-548`) `` | `` (`query.rs:568-578`) `` | `fn to_is_recorded_and_independent_of_from` moves 539 → 569 |
| `spec/SPECIFICATION.md` | 3331 | `` (`query.rs:538-548`) `` | `` (`query.rs:568-578`) `` | same anchor, second citation of it |
| `spec/SPECIFICATION.md` | 5509 | `` `position_next_signals_overflow` (`event.rs:842-873`) `` | `` (`event.rs:908-939`) `` | `fn position_next_signals_overflow` moves 843 → 909 |

Both ends of each range shift by the same amount, because every added line is
above the range and none is inside it: `query.rs` +30, `event.rs` +66. Verify
both ends independently before believing that; `spec-trace` accepts an anchor
within twelve lines, so a range can be green while pointing at the wrong
occurrence.

**3. The guard**, which belongs in the same change and is red without it. Add to
`crates/happenstance-core/tests/serialize_is_borrowing.rs` a test that extracts
every `impl Serialize for …` body from `event.rs`, `query.rs` and `append.rs` by
brace counting and asserts none contains `.clone()`, `.cloned()`, `.to_vec()`,
`.to_owned()` or `.into()`. It reads the source, which is unusual: the mirrors
are private, the bytes do not move, and the workspace sets
`unsafe_code = "forbid"` — which `allow` cannot lift — so a counting allocator
cannot live in the gate and no behavioural test in any format can tell an owned
mirror from a borrowing one. Review was the only other instrument and it passed
this at five sites. The full text is at `701191b` in that file.

**4. The experiment.** `experiments/event-clone-allocations/tests/measure_clone.rs`
asserts `t + 1` and `2(t + 1)`; both become `0`, and the comment above them says
so. `results/clone-cost.md`'s *What the prototype costs* becomes *What it costs
now*.

## Revision record

Revision 1, authored 2026-09-04 by the `lane/core-ports` remediation lane against
finding AE-2, in the same session as the measurement it reports. No independent
critic. Nothing withdrawn.

One thing corrected during authoring rather than after: the first draft asserted
the fix had landed. It had, at `2f11eb7`, and was reverted when
`cargo xtask spec-trace` failed on four citations the lane was not permitted to
repoint — which is the whole subject of the handoff above.
