# Does `EventStore::append` change ownership — take `Vec<Event>` instead of `&[Event]` — before `0.2.0` is published?

Decision record: **AE-1-append-ownership**. Audit entry: `references/evaluation/review-pre-publication-2026-09-03.md:2724-2754` (immutable evidence; read, not edited).

---

## Why this is owed

**A clause carries a falsifier, and nobody has scheduled it.** `spec/SPECIFICATION.md:3345-3353`:

> #### ES-17 — The batch is borrowed, not owned
>
> `append` MUST continue to take `events: &[Event]`.
>
> **[PROVISIONAL — falsified by a measurement on a real adapter showing the
> per-event clone is a material fraction of append cost. The named measurement is
> the SQLite adapter's multi-row insert benchmark, in the phase that builds it. A
> positive result changes the signature to take `Vec<Event>` **and** obliges the
> contract to give callers a cheap way to keep a copy for retry.]**

The ledger agrees the marker has not moved — `spec/SPECIFICATION.md:9094`:

```
| ES-17 | PROVISIONAL | `append_preserves_event_payload` | E2E-36, E2E-39 |
```

**The measurement that would decide it has no owner.** `.kb/open-questions/es-17-two-adapter-measurement-is-unscheduled.md` (kind `open_question`, status `accepted`):

> What exists right now is neither: the marker sits `[PROVISIONAL]`, the falsifier's evidence is unproduced, and no queue row or story names an owner.

Its two named triggers: whoever next proposes lifting ES-17, and phase 12 — `RUNBOOK.md:4681`, *"## Phase 12 — Publish `0.2.0`"*.

**The audit found the corpus mis-prices the thing the decision turns on.** `references/evaluation/review-pre-publication-2026-09-03.md:2740`:

> Measured, one `event.clone()`, steady state, payload `Bytes::from_static` so nothing in the row is the payload: **66 heap operations requesting 2,001 bytes** at VT-22's 64-tag floor, against **1 operation and 1,536 bytes** in the `Tag::from_static` control — **66.0x**.

That is verified below against `results/raw/`, and it is not the reason the decision is owed — it is the reason the decision cannot be taken from the corpus's own prose.

---

## What is true today

### The signature, and the doc that explains why

`crates/happenstance-core/src/store.rs:261-265`:

```rust
    async fn append(
        &self,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<Self::Error>>;
```

The obligation that makes the borrow load-bearing is on the same item, `store.rs:257-259`:

```
    /// * [`AppendError::ConditionViolated`] when the store already holds an
    ///   event matching `condition`. This is routine under contention: rebuild
    ///   the decision model and retry.
```

ES-17's own third ground, `spec/SPECIFICATION.md:3374-3377`:

> And `ConditionViolated` obliges the caller to keep its events across the call, so by-value would move the clone from the adapter's success path to the caller's every path.

### The four corpus sites that price a clone at two allocations

Three of them state the count. All three name `Box<str>`, which is the wrong type.

1. `crates/happenstance-core/src/event.rs:409-412` — rustdoc on `Event::into_parts`:

   ```
   /// reach this method at all; an owning adapter clones instead, and that
   /// clone is cheap — the expensive fields are [`Bytes`], so it bumps a
   /// refcount rather than copying the payload, leaving one `Box<str>` and one
   /// boxed tag slice.
   ```

2. `spec/SPECIFICATION.md:3370-3373` — inside ES-17's own rationale:

   ```
   The borrow wins on three grounds. `Event`'s expensive fields are `Bytes`
   (`event.rs:323`, `:325`), so a clone bumps a refcount rather than copying the
   payload; the remaining cost is one `Box<str>` and one boxed tag slice, bounded by
   the tag count.
   ```

3. `references/adr/0012-append-shape-and-preconditions.md:172-174`:

   ```
   - `Event`'s expensive fields are `Bytes` (`event.rs:185`, `:187`), which is
     refcounted, so `event.clone()` bumps a counter rather than copying a payload.
     What actually copies is one `Box<str>` for the type and one boxed tag slice.
   ```

4. `crates/happenstance-core/src/memory.rs:30-31` — the cheapness claim without a count:

   ```
   /// Cloning is cheap regardless: payloads are [`Bytes`](bytes::Bytes), so a
   /// snapshot bumps refcounts rather than copying data.
   ```

**`event.rs:411` is stale twice over, and the audit is right on both counts.** The type is wrong: `crates/happenstance-core/src/event.rs:47` is `pub struct EventType(Cow<'static, str>);` and `crates/happenstance-core/src/tag.rs:79` is `pub struct Tag(Cow<'static, str>);`, both changed by ADR-0015 (`references/adr/0015-validated-identifiers-and-store-limits.md:245`: *"Both change their backing field from `Box<str>` to `Cow<'static, str>`"*). And the count is wrong, because `crates/happenstance-core/src/tag.rs:281` is `pub struct Tags(Box<[Tag]>);` — a boxed slice of `Tag`s, each of which is its own allocation when owned. `Tags::from_pairs` routes every tag through `Tag::new` (`tag.rs:304-312` → `tag.rs:90-93`), whose accepting arm is `Ok(Self(Cow::Owned(value)))`.

`spec/SPECIFICATION.md:3372` and `references/adr/0012:174` carry the same stale type. `memory.rs:30-31` carries no count, so it is stale only against the payload finding below.

### The measured figures, verified from `results/raw/`

Source: `experiments/event-clone-allocations/results/raw/clone.txt`, untouched output of `cargo test --release --test measure_clone`. Toolchain in `results/raw/conditions.txt`: `rustc 1.97.1 (8bab26f4f 2026-07-14)`, `x86_64-pc-windows-msvc`. `heap_ops` = `alloc` + `realloc`.

`event.clone()`, one event, second clone ("again"), `Bytes::from_static` payload:

| tags | `from_pairs` (`Cow::Owned`) heap ops | bytes | `from_static` (`Cow::Borrowed`) heap ops | bytes | ratio |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 0 | 1 | 17 | 0 | 0 | — |
| 1 | 3 | 48 | 1 | 24 | 3.0x |
| 8 | 10 | 265 | 1 | 192 | 10.0x |
| 32 | 34 | 1,009 | 1 | 768 | 34.0x |
| **64** | **66** | **2,001** | **1** | **1,536** | **66.0x** |

The audit's arithmetic holds against the raw rows. **The owned arm is `t + 2` for every `t >= 1`** — one for the `EventType`'s `String`, one for the `Box<[Tag]>`, one per tag — **and `1` at `t = 0`**, because `<[T]>::to_vec()` on an empty slice allocates nothing. So the published "two allocations" is wrong in both directions: it overcounts a tagless event by one and undercounts a 64-tag event by 64. **The borrowed arm is flat at `1` across 1, 8, 32 and 64 tags** and does not respond to tag count at all.

64 is not an arbitrary probe point. `spec/SPECIFICATION.md:1543-1548`, VT-22:

> Every store MUST accept an event carrying at least `MIN_SUPPORTED_TAGS_PER_EVENT` = 64 tags

One thing the audit reports that is worth carrying forward because it contradicts site 4: from the same raw file, `Bytes::from(Vec<u8>)` — the shape every decoded payload has — costs **67** on its first clone against **66** on the second at 64 owned tags, and **2** against **1** at 64 static tags. `bytes` 1.x stores a `Vec`-backed payload promotably; the first clone allocates a shared header. `crates/happenstance-core/src/memory.rs:398` clones each appended event once, so it pays that once per event it ever accepts.

The corpus already had the right arithmetic once and lost it — `references/evaluation/research-rust-api-guidelines.md:103-105`:

> `Event::clone()` is not cheap. `Bytes` is a refcount bump, but `EventType(Box<str>)` is one allocation and `Tags(Box<[Tag]>)` is `n + 1` allocations because each `Tag` is its own `Box<str>`. A three-tag event costs **five allocations per event, per append**, in every adapter, forever.

### ADR-0012's falsifier, items 1 and 5

`references/adr/0012-append-shape-and-preconditions.md:244-264`:

> **What phase 8 must produce for the marker to lift.** Restating the falsifier so that a positive result cannot be manufactured:
>
> 1. Two builds of the *same* SQLite adapter differing only in `append`'s ownership, measured on the same harness.

> 5. A design for the second half of the clause's own marker, which a measurement alone does not supply: "a cheap way to keep a copy for retry". A positive result is not a licence to change one signature. `ConditionViolated` obliges the caller to still hold its events, so by-value `append` is only a net win if the contract hands back something cheaper than `Vec::clone` — and until that shape exists, a measurement that shows the adapter saving a clone has not shown the *system* saving one.

Item 4 is the one that decides whether item 1 can fire at all (`:254-257`):

> 4. A statement of which adapter shape was measured — one that moves the payload into an owned row it keeps, or one that binds parameters from a borrow. Only the first can benefit, and a measurement that does not say which it is does not settle anything.

And the successor is already fixed (`:266`): *"If that fires, the successor is `Vec<Event>` **and nothing else** — see §2."*

### Two adapters have real bodies, and they sit on opposite sides of item 4

> **Removed, and why.** This section previously read *"The only adapter with real bodies is
> the shape that cannot benefit"*, and closed with *"the one store in the workspace that does
> clone is the one ADR-0012 disqualifies"* — `MemoryEventStore`. Both sentences are
> **false** and are deleted rather than softened. `happenstance-cloudflare` has full bodies
> (no `todo!()` under `crates/happenstance-cloudflare/src/`), runs
> `event_store_conformance!` (`crates/happenstance-cloudflare/tests/durable_object_conformance.rs:64`),
> and has had its `publish = false` removed (`crates/happenstance-cloudflare/Cargo.toml:22`).
> It is neither borrow-binding SQLite nor the reference store, and it copies the payload.
> ADR-0012's census was true at phase 4 and has been stale since phase 9; the earlier draft
> repeated it instead of checking it.

#### The borrow-binding shape: `happenstance-sqlite`

`crates/happenstance-sqlite/src/event_store.rs:758-779` — `write_batch`, the multi-row insert the falsifier names:

```rust
fn write_batch(
    connection: &Connection,
    store_id: StoreId,
    events: &[Event],
    recorded_at: RecordedAt,
) -> rusqlite::Result<SequencePosition> {
    let mut positions = Vec::with_capacity(events.len());
    {
        let mut insert = connection.prepare(
            "INSERT INTO event (event_type, data, metadata, tags, recorded_at) \
             VALUES (?, ?, ?, ?, ?)",
        )?;
        for event in events {
            insert.execute(rusqlite::params![
                event.event_type().as_str(),
                &event.data()[..],
                event.metadata().map(|metadata| &metadata[..]),
                crate::row::encode_tags(event.tags()),
                recorded_at.as_millis(),
            ])?;
```

**No `Event::clone` anywhere in the SQLite write path.** It binds parameters from the borrow. Under item 4 that is the shape that *cannot* benefit — which ADR-0012 already said in prose at `:181-187`:

> And the adapter that would benefit from ownership is a specific one: a store that **moves** the payload into an owned row type it keeps. A SQL adapter does not — it binds parameters from borrowed bytes and hands them to the driver, so it takes nothing at all from owning the batch. The reference store is the wrong instrument in the opposite direction: `memory.rs:232-235` performs the clone because an in-memory log *is* the owned row type, which is the least representative shape in the portfolio.

The reference store, still disqualified by that same passage — `crates/happenstance-core/src/memory.rs:388-399`:

```rust
        stored.extend(events.iter().enumerate().map(|(offset, event)| {
            let position = position_at(first_index + offset);
            ...
            SequencedEvent::new(
                position,
                EventId::new(self.store_id, position),
                recorded_at,
                event.clone(),
            )
        }));
```

#### The owning shape, which ADR-0012's census predates: `happenstance-cloudflare`

`crates/happenstance-cloudflare/src/event_store.rs:602-631` — `write_rows`, one `INSERT … RETURNING position` per event:

```rust
        for event in events {
            let metadata = event
                .metadata()
                .map_or(SqlValue::Null, |bytes| SqlValue::Blob(bytes.to_vec()));
            let mut cursor = self.sql.exec(
                "INSERT INTO event (event_type, data, metadata, tags, recorded_at) \
                 VALUES (?, ?, ?, ?, ?) RETURNING position",
                &[
                    SqlValue::Text(event.event_type().as_str().to_owned()),
                    SqlValue::Blob(event.data().to_vec()),
                    metadata,
                    SqlValue::Blob(encode_tags(event.tags())),
                    SqlValue::Integer(recorded_at.as_millis()),
                ],
            )?;
```

Every column is **copied into an owned `SqlValue`, per event**: `to_owned()` on the type, `to_vec()` on the payload, `to_vec()` on the metadata. It has no choice — the worker layer marshals these into JS and cannot bind a borrow. That is item 4's *"moves the payload into an owned row it keeps"* shape, in an adapter that is not the reference store.

**What by-value would buy there, stated with its condition rather than as a claim.** `Vec<u8>: From<Bytes>` is O(1) when the `Bytes` is uniquely owned and `Vec`-backed and a copy otherwise, and `Cow<'static, str>::into_owned` on an `Owned` arm is free. So an owned batch could move the payload, the metadata and the type instead of copying them — for a decoded payload, which is exactly the `Bytes::from(Vec<u8>)` shape. **Whether that is a material fraction of append cost against a `sql.exec` round trip is a measurement, not a reading.** That is the point: the saving is proportional to payload size rather than tag count, so it is on a different axis from the 66x clone finding entirely, and it is a live candidate for the falsifier's own words rather than a foregone null.

### The instrument exists. Its parameters do not cover the falsifier.

The harness landed at phase 8 — `spec/SPECIFICATION.md:8762-8765`:

> **The separation stopped being a plan at phase 8.** `event_store_benchmarks!` now exists as a fourth macro family in `crates/happenstance-testkit/src/bench.rs`, behind an off-by-default `bench` feature

and `crates/happenstance-testkit/src/bench.rs:52-58` says it was built for exactly this:

> ADR-0012 names this same instrument as the evidence that could lift its `&[Event]` marker, and specifies *"a realistic batch and rejection mix"*

Three things about it are decision-relevant, and two of them refine the audit rather than repeat it.

**(a) The regime is already the expensive one.** `crates/happenstance-testkit/src/fixtures.rs:73-75`:

```rust
pub fn tags(pairs: &[(&str, &str)]) -> Tags {
    Tags::from_pairs(pairs.iter().copied()).expect("valid tags")
}
```

`from_pairs` is the `Cow::Owned` path. `tagged_event` (`fixtures.rs:36-38`) routes through it, and `bench.rs` uses `tagged_event` exclusively. There is no `Tag::from_static` anywhere under `crates/happenstance-testkit/src/`. **The `from_static` trap the audit predicted is not the shape this tree's harness has.**

**(b) The tag count is hard-coded at one, and there is no knob.** `crates/happenstance-testkit/src/bench.rs:539-541`:

```rust
        let batch: Vec<Event> = (0..params.batch_size())
            .map(|_| tagged_event("BenchmarkAppended", &[("bench", "append")]))
            .collect();
```

and `bench.rs:106-110`:

```rust
pub struct BenchmarkParams {
    batch_size: usize,
    contenders: usize,
    replay_events: usize,
}
```

One tag. `t + 2 = 3` heap ops per event, against 66 at VT-22's floor — a 22x understatement *inside the right regime*. Falsifier item 2 asks for *"a realistic tag count per event"* (`references/adr/0012:249-251`); `BenchmarkParams` has no field that could express it.

**(c) The contention scenario shares one batch by reference, which is where the by-value cost would land.** `crates/happenstance-testkit/src/bench.rs:601` and `:608-617`:

```rust
        let contender_event = [tagged_event("BenchmarkContender", &[("bench", "contend")])];
        ...
                let events = &contender_event;
                let attempt =
                    async move { Outcome::of_append(&store.append(events, Some(condition)).await) };
```

Under `Vec<Event>` every contender would need its own copy. The caller-side cost the borrow avoids is structurally invisible in the harness as shipped.

### Why the proposed instrument cannot live in `crates/`

`Cargo.toml:160`:

```toml
unsafe_code = "forbid"
```

`GlobalAlloc` requires `unsafe impl`, and `forbid` at the workspace root cannot be lifted from inside a member crate. That is why the measurement lives in `experiments/event-clone-allocations/`, which is outside the workspace — `Cargo.toml:3`, `members = ["crates/*", "examples/*", "xtask"]`. It also cannot be a gate step under CF-34 (`spec/SPECIFICATION.md:8747-8748`): *"Performance MUST be measured by a separate harness, and that harness MUST NOT be part of the conformance bar."* The audit's reading here is correct and I found nothing that softens it.

### Governing decision atom

`.kb/decisions/0012-append-shape-and-preconditions.md` — `id: kb-decision-0012`, `kind: decision`, `status: accepted`, `authority_tier: decision`, `adr_id: ADR-0012`, `reversibility: low`, `phase: 4`, `supersedes: null`, `superseded_by: null`. Its holding:

> `EventStore::append` keeps `events: &[Event]`. […] The clause carrying `&[Event]` stays `[PROVISIONAL]` rather than lifting to `[FROZEN]`, because the evidence its own marker asks for […] does not exist yet

**An accepted atom is immutable.** Any answer to this question that changes the signature, or that lifts the marker, or that restates the falsifier, is a **new atom superseding `kb-decision-0012`** — never an edit to it. The long form at `references/adr/0012-append-shape-and-preconditions.md` (1,100+ lines) stays cited by `file:line`; `spec/SPECIFICATION.md` cites ranges in it and `cargo xtask spec-trace` will notice if they move.

The open-question atom `kb-open-question-es-17-two-adapter-measurement-001` (status `accepted`) is the second record in play. It records the *absence of a schedule*; it does not decide anything, and the audit explicitly adds an obligation to it rather than settling it.

### Publication state, stated precisely

`Cargo.toml:15` is `version = "0.2.0-alpha.1"`, and `Cargo.toml:6-11` says why:

> A **pre-release**, and it is the number `happenstance` and `happenstance-core` both inherit. […] `-alpha.1` because the API is expected to keep moving until the stable `0.2.0`, and a stable number would make "we can't change that now" available as an argument against the rest of the runbook.

> **Removed, and why.** This section previously read *"Nothing is on crates.io at that
> number"*, and construed the audit's *"live at `0.2.0-alpha.1`"* as *"the version the
> workspace declares, not a release anyone can depend on"*. Both are **false** and are
> deleted, not rewritten around. They rested on two repository sentences that resolve but
> say something else: `RUNBOOK.md:1093-1094` is a **dated 2026-08-06 changelog line** about
> the `0.0.0` name reservations, superseded ten days later; `RUNBOOK.md:4186` (*"Nothing is
> published, so nothing downstream broke"*) sits in the **phase-4 narrative** justifying the
> `0.1.0` → `0.2.0` bump, and is followed at `RUNBOOK.md:4197` by *"## Release:
> `0.2.0-alpha.1`, here"*. The audit had already warned against exactly this, at
> `references/evaluation/review-pre-publication-2026-09-03.md:57-58`: *"already on crates.io
> at `0.2.0-alpha.1` — **verified against the registry, not read off the README**."* This is
> the ADR-0029 defect in miniature — a repository sentence that was true when written,
> relied on as a present fact.

**`happenstance`, `happenstance-core` and `happenstance-testkit` are published at `0.2.0-alpha.1`.** The audit verified it against the registry (`:57-58`), and the repository's own record agrees — `CHANGELOG.md:306-308`:

```
## [0.2.0-alpha.1] — 2026-08-16

**The first published release, and it is a pre-release on purpose.**
```

So `EventStore::append`'s `&[Event]` (`crates/happenstance-core/src/store.rs:261-265`) is **inside a published crate**, and has been since 2026-08-16. The clock on this question started then, not at phase 12.

**Why the window is nonetheless still cheap, stated for the right reason.** Not because nothing is published — it is. Because **`0.2.0-alpha.1` is a pre-release, and no `^0.2` requirement resolves to a pre-release** (`references/evaluation/review-pre-publication-2026-09-03.md:60-61`: *"so no `^0.2` requirement resolves to it. That makes a breaking change to the three published crates **nearly free today and permanent the day `0.2.0` ships**"*). A dependent has to name the pre-release explicitly to be pinned to this signature. That is a narrower and more perishable protection than "nothing is published", and it expires at phase 12's publish of `0.2.0` (`RUNBOOK.md:4681-4684`) exactly as the deadline claim said.

---

## Options

### Option A — Keep `&[Event]`; leave ES-17 `[PROVISIONAL]`; schedule the two-build measurement, **against `happenstance-cloudflare`**, as ADR-0012 items 1 and 4 jointly specify

> **Removed, and why.** The earlier draft costed this option as *"a second SQLite build
> against a `Vec<Event>` port cannot be driven by `event_store_benchmarks!` at all … either a
> second trait or a forked testkit"*, and forecast its result as a null. Both are deleted.
> The forked-testkit claim is falsified by `experiments/append-condition/`, which drives
> `happenstance_testkit::event_store_benchmarks!` **verbatim** (`tests/measure.rs:1`) over
> **five variant stores** built on one schema and varied along a local axis
> (`src/strategy.rs`, `src/tags.rs`), from a crate **outside the workspace**
> (`Cargo.toml:3`, `members = ["crates/*", "examples/*", "xtask"]`) — the by-value port is a
> local trait in such a crate, not a fork of anything in `crates/`. The null forecast is
> falsified by the Cloudflare adapter above.

**Costs a caller:** nothing today. The uncertainty persists past `0.2.0` unless the measurement lands first.
**Costs an adapter author:** nothing to the signature. The work is an experiment crate on the `experiments/append-condition/` pattern: two builds of the same store differing only in `append`'s ownership, the by-value one against a locally-declared port, both driven by `event_store_benchmarks!` for the borrowed arm and by this crate's own emitter otherwise. The two testkit refinements the audit's finding implies — a tag-count field on `BenchmarkParams` (`bench.rs:106-110`) and an `append_throughput` that stops hard-coding one tag (`:540`) — remain worth having, but Option A does not depend on them landing in `crates/` first.
**Semver class:** `none` today; defers the class to whenever the measurement reports.
**Forecloses:** nothing. It buys the option and pays for the instrument.

**What the evidence says about its expected outcome:** it depends entirely on which adapter is measured, which is what item 4 exists to force. Against **SQLite** the adapter-side delta is expected to be a null, and that is readable off `event_store.rs:770-779` without running anything: it binds every column from the borrow. Against **`happenstance-cloudflare`** it is not a null by inspection either way — `write_rows` copies payload, metadata and type into owned `SqlValue`s per event, so by-value could move them, and the size of that against a `sql.exec` round trip is unknown. Scheduling this option against SQLite would produce the null; scheduling it against Cloudflare is a real question. **The saving on that axis scales with payload size, not tag count**, so it is a different quantity from the 66x clone finding rather than a larger version of it.

### Option B — Keep `&[Event]`; correct the four documentation sites; restate ES-17's falsifier so it names an instrument that could exist; leave the marker where the human puts it

**Costs a caller:** nothing. No signature moves.
**Costs an adapter author:** nothing. Three of the four sites are rustdoc and record (`event.rs:409-412`, `memory.rs:30-31`, `references/adr/0012:172-174`); the fourth is a clause's rationale (`spec/SPECIFICATION.md:3370-3373`) and is a specification change routed to whoever holds ES-17.
**Semver class:** `none`. Documentation and a marker's falsifier text; no public item changes.
**Forecloses:** the "`Box<str>`, two allocations" framing, permanently, in all four places. It does not foreclose `Vec<Event>` — it makes the case for it *harder to manufacture*, which is what ADR-0012 says a restated falsifier is for (`:244-245`).

**What it must add, from the audit and from this grounding:** the falsifier must name (i) the adapter shape measured — item 4 already asks; (ii) the **tag regime** the fixtures are in, because the answer differs by 66x at the specification's own floor; and (iii) the **tag count**, because the shipped harness is in the right regime at the wrong count and would have reported a 3-allocation clone as if it were the 66-allocation one.

### Option C — Change `append` to `Vec<Event>` at `0.2.0`, plus the retry affordance item 5 obliges

**Costs a caller:** the events, at the call site. `ConditionViolated` is *"routine under contention"* (`store.rs:258`), so every retrying caller clones its batch before every attempt — moving a clone off the adapter's success path onto the caller's every path (`spec/SPECIFICATION.md:3374-3377`). ADR-0012 spells the vocabulary out at `:284-286`: *"`Vec<Event>` transfers ownership: the caller's variable is gone at the call site, which is why `ConditionViolated` would then force every caller to clone *before* appending in order to be able to retry."*
**Costs an adapter author:** every `impl EventStore` in the workspace changes signature — `happenstance-core`'s `MemoryEventStore`, `happenstance-sqlite`, `happenstance-cloudflare`, four skeletons, the testkit's own wrong-adapter fixtures, and `examples/outside-projection-adapter`. `happenstance-sqlite` gains nothing in exchange: it never owned an `Event` to begin with. `happenstance-cloudflare` is the one that could gain, by the amount Option A would measure.
**Semver class:** **breaking** on `happenstance-core`'s `EventStore`/`SendEventStore`, plus **additive** for the retry shape item 5 requires — so **mixed**.

> **Removed, and why.** This line previously ended *"Nothing on crates.io depends on it today
> (`RUNBOOK.md:4186`), so the break is in-tree."* That is **false** and is deleted:
> `happenstance-core` is published at `0.2.0-alpha.1` (`CHANGELOG.md:306`), so this is a
> breaking change **to a published crate**, not a tree-wide rename. It is cheap for a
> narrower reason — no `^0.2` requirement resolves to a pre-release — and only a dependent
> who named `0.2.0-alpha.1` explicitly is broken by it. See *Publication state* above.
**Forecloses:** the free window. Once `0.2.0` is on crates.io, reversing it is a second breaking change against real dependents.

**What it lacks:** item 5's *"cheap way to keep a copy for retry"* has no design anywhere in the tree. I grepped `crates/` for `Arc<[Event]>`, `Arc<Vec<Event>>` and `EventBatch`: the only hits are two comments saying an `EventBatch` newtype is forbidden (`crates/happenstance-testkit/src/contract.rs:282`, `crates/happenstance-testkit/src/suite.rs:3927`). Item 5 is explicit that a measurement alone does not supply it and that *"a measurement that shows the adapter saving a clone has not shown the **system** saving one"*.

### Option D — Lift ES-17 to `[FROZEN]` now, keeping `&[Event]`, on the structural argument

**Costs a caller:** nothing.
**Costs an adapter author:** nothing.
**Semver class:** `none` for code; a maturity marker moves.
**Forecloses:** `Vec<Event>` permanently, absent a new ADR. `spec/SPECIFICATION.md` and CLAUDE.md agree: changing a `[FROZEN]` clause requires a new ADR, not an edit.

**The repository's own standing objection to it**, from `.kb/decisions/0012`'s *Alternatives rejected*:

> Lifting the borrow clause to frozen on its existing three grounds was rejected because those grounds were already in the clause; lifting on no new evidence moves a maturity marker because a phase wanted it moved, which is the one thing a marker must never do.

The audit's measurement is new evidence about the *clone's price*, not about *append cost*, so it is not obviously the evidence that objection was waiting for.

---

## Recommendation

**Option B *and* Option A: take B's corrections now, and schedule A against `happenstance-cloudflare` rather than against SQLite.** Do not change `append`'s ownership at `0.2.0`; correct the four sites; restate the falsifier so it names its adapter shape, its tag regime and its tag count; leave the marker at `[PROVISIONAL]`; and name an owner for a two-build measurement whose subject is the adapter that actually copies the payload.

> **This flipped, and what flipped it.** The earlier draft recommended *"Option B, and
> explicitly not Option D"*, and declined Option A on two claims that are now deleted as
> false: that the measurement would *"produce a result the code already discloses"*, and that
> it would have to be paid for *"in the one place the repository is most protective of — the
> testkit"*. The first was true of SQLite and only SQLite; `happenstance-cloudflare` has real
> bodies, runs the conformance suite, is publishable, and copies payload, type and metadata
> into owned `SqlValue`s per event. The second is falsified by
> `experiments/append-condition/`, which already drives `event_store_benchmarks!` verbatim
> over five variant stores from outside the workspace. With both reasons gone, nothing was
> left declining Option A, so it is no longer declined. **B is unchanged; A moves from
> rejected to recommended-with-a-named-subject.**

**Why it still beats Option C.** Option C's falsifier has not fired: no measurement exists on any adapter, including the Cloudflare one that could produce a non-null. And Option C cannot be taken as a whole today regardless of what a measurement says, because item 5's retry affordance does not exist and nothing in the tree sketches it — *"a measurement that shows the adapter saving a clone has not shown the **system** saving one"* (`references/adr/0012:262-264`). Note also what the falsifier as written now aims at the wrong target: it names *the SQLite adapter's multi-row insert benchmark* (`spec/SPECIFICATION.md:3350-3351`), and SQLite is the shape item 4 excludes. Restating that is part of Option B's work, and it is now the highest-value part of it.

**What Option A is worth, now that it is not a null.** The residual value is on two axes, not one. The **caller** side, which the harness structurally hides (`bench.rs:601`). And the **payload** side on Cloudflare, which is on a different axis from the 66x clone finding entirely and which no reading of the code settles. Option B's restatement is what keeps A honest when it runs: without (ii) and (iii) the harness would run in the right regime at one tag and report a 3-allocation clone as the answer to a 66-allocation question.

**Why not Option D.** The measurement is genuinely absent, and Option D converts its absence into a permanent commitment. `.kb/decisions/0012` already refused that move once, by name.

**The strongest argument against Option B, in its own words.** From `.kb/decisions/0012-append-shape-and-preconditions.md`:

> A phase that decides a measurement question without the measurement has not decided it, it has guessed and frozen the guess.

The earlier draft declined a measurement on the strength of reading `write_batch`. That is reasoning about cost from the shape of code — and this review has already demonstrated, at 66x, that reasoning about allocation cost from the shape of types failed in this exact corpus, in four places, for months. A recommendation built on the same method has no standing to say the method is reliable this time. **That objection landed**: reading the code is precisely how this brief missed `happenstance-cloudflare`, and the flip above is its consequence. The honest form of the recommendation is therefore: *correct what was measured, restate what would falsify, schedule the measurement against the shape that could fail it, and do not claim the restatement is a substitute for the measurement.* It is not.

**The strongest surviving argument against the recommendation, verbatim from the critique that raised it.** It does not change the recommendation — B survives — but it names a fact this brief got wrong and a reason it stated wrongly, and both are load-bearing for anyone who quotes the *Cost of delay* section:

> The brief overturns a registry-verified audit fact with two repo sentences that resolve but say something else. RUNBOOK.md:1093 ("reserved at 0.0.0") is a dated 2026-08-06 changelog line, superseded ten days later. RUNBOOK.md:4186 ("Nothing is published") sits in the phase-4 narrative, followed at :4197 by "Release: 0.2.0-alpha.1, here" — it justifies the version bump, not today's registry. The audit warned at :58: verified against the registry, not read off the repo. Materially, append's `&[Event]` is inside published core 0.2.0-alpha.1 (store.rs:213-217): Option C breaks a published crate and the clock started 2026-08-16, not phase 12. That points the way B already does, so B survives — but "Free now" and Option C's semver line are priced on a false fact, and the real reason the window stays cheap goes unstated: no ^0.2 resolves to a pre-release.

**And the one that flipped it, verbatim:**

> Option A was declined on two wrong claims. (1) "The code already discloses the result." It discloses a null for SQLite only. CloudflareEventStore::write_rows copies the whole payload, type and metadata into owned SqlValues per event — the worker layer marshals into JS and cannot bind a borrow — so it is exactly item 4's owning shape, is not MemoryEventStore, and by-value append could move Bytes/EventType instead. That saving scales with payload size, not tag count, dwarfing the 66x finding, and is a live candidate for "a material fraction of append cost". ADR-0012's census was true at phase 4, stale since phase 9; the brief repeats it. (2) "A second build needs a forked testkit." experiments/append-condition drives event_store_benchmarks! verbatim over five variant stores generic over a local strategy axis. Take B's corrections; schedule A against Cloudflare, not SQLite.

**What this recommendation does not cover.** Whether the marker ever lifts, and by whom, is a marker decision with its own norm attached (`.kb/decisions/0012`, *Alternatives rejected*). I am not recommending a marker move in either direction, because the evidence supports "do not change the signature" and does not support "the question is closed."

---

## Cost of delay

**Nearly free now — and the clock started on 2026-08-16, not at phase 12.**

> **Removed, and why.** This section previously opened *"Free now"* and led with *"Nothing is
> on crates.io at `0.2.0-alpha.1`. The three principal names sit at `0.0.0`."* That bullet is
> **deleted as false**, and the heading is corrected with it. The three crates were published
> at `0.2.0-alpha.1` on 2026-08-16 (`CHANGELOG.md:306`, and the audit's registry check at
> `references/evaluation/review-pre-publication-2026-09-03.md:57-58`), so *"free"* was priced
> on a fact that stopped being true two and a half weeks before this brief was written.

Nearly free today, for a narrower and more perishable reason than the one this brief first gave:

- **No `^0.2` requirement resolves to a pre-release.** `0.2.0-alpha.1` is opt-in by exact mention, so a breaking change to `EventStore::append` reaches only a dependent who named the alpha — *"nearly free today and permanent the day `0.2.0` ships"* (`references/evaluation/review-pre-publication-2026-09-03.md:60-61`). That is the real protection, and it is not the same as *"nothing is published"*.
- The alpha exists precisely to keep that window open — `Cargo.toml:8-11`: *"a stable number would make 'we can't change that now' available as an argument against the rest of the runbook."* `CHANGELOG.md:308-309` says the same from the release side: *"The API is expected to move until the stable `0.2.0`."*
- Option B costs nothing on any axis: no public item moves, no adapter recompiles, no gate step changes. Option A costs an experiment crate outside the workspace and no published surface at all.

Not free after phase 12 (`RUNBOOK.md:4681`, *"Publish `0.2.0`"*). After that, Option C is a breaking change against dependents who did not opt in, and the open-question atom already names phase 12 as its second trigger: *"first publish is the point at which an unresolved signature stops being free to change."* The correction to note is that the *first* publish has already happened; what phase 12 removes is the pre-release exemption, not the first exposure.

**One honest tension.** `RUNBOOK.md:4690-4691` says publication *"is not 'publishing freezes the public API'. The API was frozen in phases 4 – 6, on evidence, which is what makes publishing safe."* ES-17 is one of the clauses that was deliberately *not* frozen in phase 4. So for this clause specifically, publication really does close the window the runbook says publication does not close. That is not a contradiction in the runbook — it is the residue of a deferral the runbook accounted for — but it means "the API was frozen in phases 4-6" cannot be quoted as reassurance here.

**The asymmetry that makes this bearable.** Option B's corrections are free and permanent whichever way the ownership question eventually goes: `Box<str>` is wrong in three places today regardless, and a falsifier that names its tag regime is better in both outcomes.

---

## What this does not settle

- **Whether ES-17 lifts to `[FROZEN]`.** Deliberately left open. It is a marker decision, ADR-0012 refused it once on a stated norm, and nothing found here discharges that norm.
- **Who takes the two-build measurement, and when.** `.kb/open-questions/es-17-two-adapter-measurement-is-unscheduled.md` stays open, and this brief now recommends *scheduling* it (Option A) rather than deferring it, but names no owner and no phase — that is the atom's choice to record. What this brief does settle is its **subject**: `happenstance-cloudflare`, not SQLite.

  > **Removed, and why.** This bullet previously offered *"a fourth reason it is hard to
  > satisfy — the only adapter with real bodies is the shape item 4 excludes."* Deleted as
  > false; the Cloudflare adapter is the counter-example, and the measurement is easier to
  > satisfy than this brief claimed, not harder.
- **What "a cheap way to keep a copy for retry" would be.** Item 5's obligation has no candidate shape in the tree. If Option C is ever revisited, that design is a prerequisite and not an afterthought.
- **Whether `references/adr/` records may be corrected in place.** `references/adr/0012:174` carries the same stale `Box<str>`. CLAUDE.md makes accepted *atoms* immutable and describes `references/adr/` as the full original records kept for citation; it does not say whether a record may be amended. `spec/SPECIFICATION.md` cites line ranges into it and `cargo xtask spec-trace` gates those, so any correction there has a mechanical consequence. Unresolved here.
- **Whether the testkit's benchmark harness should gain a tag-count parameter.** It is the concrete enabling change behind both Option A and Option B's restatement (`bench.rs:106-110`, `:540`, `:601`), but adding it is testkit surface work with its own review, not part of answering the ownership question.
- **The payload-promotion cost.** `Bytes::from(Vec<u8>)`'s first clone allocates a shared header — one extra allocation per event for any store that clones each appended event once, which `memory.rs:398` does. It contradicts `memory.rs:30-31` and is not part of `t + 2`. It is a documentation correction on a fourth site, and it bears on the reference store rather than on `append`'s signature.
- **`Serialize for SequencedEvent`'s double clone.** `clone-cost.md` records a flat 2.00x across every tag count and both formats — 130 of 140 heap ops transient at 64 tags. It is a real finding in the same experiment and it belongs to the wire layer, not to ES-17.

---

## Revision record

Two critiques declined the first draft's recommendation. Both were checked against the working
tree before anything was changed, and **both were upheld**. Nothing outside this file was
touched; the audit at `references/evaluation/review-pre-publication-2026-09-03.md` remains
immutable evidence, read and not edited.

### 1. `happenstance-cloudflare` was missed, and it is item 4's benefiting shape

**Falsified premise:** *"The only adapter with real bodies is the shape that cannot benefit"*,
and *"the only store that moves the payload into an owned row is `MemoryEventStore`, which
ADR-0012 disqualifies by name."*

**Verified in the tree:** `crates/happenstance-cloudflare/src/` contains no `todo!()`;
`crates/happenstance-cloudflare/tests/durable_object_conformance.rs:64` invokes
`event_store_conformance!`; `Cargo.toml:22` records that `publish = false` is gone; and
`write_rows` (`src/event_store.rs:602-631`) builds `SqlValue::Text(… .to_owned())`,
`SqlValue::Blob(event.data().to_vec())` and `SqlValue::Blob(metadata.to_vec())` **per event**,
because the worker layer marshals into JS and cannot bind a borrow. ADR-0012's two-shape census
was written at phase 4 and this adapter got real bodies at phase 9; the first draft repeated the
census instead of re-running it.

**Also falsified:** *"a second SQLite build … cannot be driven by `event_store_benchmarks!` at
all … either a second trait or a forked testkit."* `experiments/append-condition/tests/measure.rs:1`
drives `event_store_benchmarks!` **verbatim** over five variant stores
(`src/strategy.rs`, `src/tags.rs`) from a crate outside the workspace (`Cargo.toml:3`).

**Changed:**
- §*The only adapter with real bodies…* renamed to *Two adapters have real bodies, and they sit
  on opposite sides of item 4*; the false sentences **deleted** with a note saying so; the
  Cloudflare write path added, with the by-value saving stated **as a conditional worth
  measuring**, not as a claim.
- Option A rewritten: the forked-testkit cost and the null forecast **deleted**; its subject
  named as `happenstance-cloudflare`.
- Option C's adapter-cost line corrected (`happenstance-cloudflare` is an `impl` that changes,
  and the only one that gains).
- **The recommendation flipped**: from *"Option B, and explicitly not Option D"*, which declined
  Option A, to *"Option B **and** Option A, scheduled against Cloudflare."* Both reasons for
  declining A were the falsified claims; with them gone nothing declined it. **B itself did not
  change.**
- The *What this does not settle* bullet that called the measurement harder to satisfy
  **deleted**; it is easier than the draft claimed.

### 2. The three crates are published; "nothing is on crates.io" was false

**Falsified premise:** *"Nothing is on crates.io at that number"*; *"the audit's 'live at
`0.2.0-alpha.1`' means the version the workspace declares"*; and Option C's *"the break is
in-tree."*

**Verified in the tree:** `CHANGELOG.md:306-308` — *"## [0.2.0-alpha.1] — 2026-08-16 … The first
published release"*. The two sentences the draft relied on do resolve and do say something else:
`RUNBOOK.md:1093-1094` is inside a **dated 2026-08-06** entry about the `0.0.0` name
reservations, and `RUNBOOK.md:4186` sits in the phase-4 narrative justifying the `0.1.0` →
`0.2.0` bump, eleven lines above `RUNBOOK.md:4197`, *"## Release: `0.2.0-alpha.1`, here"*. The
audit had flagged the trap in advance at `:57-58` (*"verified against the registry, not read off
the README"*).

**Changed:**
- §*Publication state, stated precisely* rewritten; the false claims **deleted** with a note
  naming both mis-readings.
- Option C's semver line: *"the break is in-tree"* **deleted**; it is a breaking change to a
  published crate.
- §*Cost of delay*: the *"Nothing is on crates.io"* bullet **deleted** and the heading changed
  from *"Free now"* to *"Nearly free now"*. The window's real basis — **no `^0.2` requirement
  resolves to a pre-release** — is now stated, having been absent from the draft entirely.
- The critique is quoted **verbatim** under *Recommendation* as the strongest surviving argument
  against, because it does not flip the recommendation: B survives, and the direction it points
  is the one B already pointed.

**One citation note.** That critique cites `append` at `store.rs:213-217`; in this working tree
it is `crates/happenstance-core/src/store.rs:261-265`. Both name the same item — the difference
is the published `0.2.0-alpha.1` source against `HEAD` — and the substance (the signature is
inside a published crate) is unaffected.

### What did not change

Options B and D, the measured figures and their verification against
`experiments/event-clone-allocations/results/raw/`, the four stale documentation sites, the
`unsafe_code = "forbid"` reasoning, the immutability of `kb-decision-0012`, and the standing
objection from `.kb/decisions/0012` that a phase deciding a measurement question without the
measurement has guessed. That objection is now recorded as having **landed** — reading code
rather than measuring it is exactly how the first draft missed the Cloudflare adapter.
