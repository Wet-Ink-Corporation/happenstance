# Does `Event::metadata` get a floor of its own, a bound shared with `data`, or an explicit written statement that it is deliberately unbounded and every adapter must size around it?

Decision record: **AE-4-metadata-floor**. Source entry: `references/evaluation/review-pre-publication-2026-09-03.md:1004-1034`.

---

## Why this is owed

**Three MUSTs that do not compose.** `Event` is frozen at four fields, one of which is a payload nothing bounds; the capacity vocabulary has three names, none of which fits it; and the error channel a store would otherwise reach for is closed by a `[FROZEN]` clause.

1. **VT-1 mints the field and freezes it.** `spec/SPECIFICATION.md:594-598`:

   > An `Event` MUST carry exactly four things: an `EventType`, an opaque `data`
   > payload, a `Tags` set, and optional opaque `metadata`.

   Marked `[FROZEN]` at `spec/SPECIFICATION.md:600`.

2. **VT-21 bounds `data` and says nothing about `metadata`.** `spec/SPECIFICATION.md:1515-1518`:

   > Every store MUST accept an event whose `data` is at least
   > `MIN_SUPPORTED_EVENT_DATA_LEN` = 65,536 bytes. A store MAY accept more, MUST
   > document its actual limit, and MUST refuse an oversized payload with
   > `AppendError::ExceedsStoreLimit` rather than truncating.

   `[PROVISIONAL]` at `spec/SPECIFICATION.md:1520-1522`, with a falsifier about `data`.

3. **VT-25 closes the only channel left.** `spec/SPECIFICATION.md:1630-1633`:

   > `AppendError` MUST carry a variant `ExceedsStoreLimit { limit: StoreLimit, len:
   > usize }` […] A store MUST NOT report a capacity refusal through
   > `AppendError::Store`.

   `[FROZEN]` at `spec/SPECIFICATION.md:1635`.

So a store that cannot physically hold an event *because of its metadata* has two moves, and both are non-conformant: `AppendError::Store` is forbidden by VT-25, and `StoreLimit::EventDataLen` is a lie whose `guaranteed_minimum()` hands the caller 65,536 — a number about a different field (`crates/happenstance-core/src/limits.rs:55-56`, `:72`).

**The corpus asked the "fourth variant?" question three times and never had `metadata` in the candidate set.** `crates/happenstance-core/src/limits.rs:47-51`, `crates/happenstance-testkit/src/contract.rs:287-290`, `crates/happenstance-sqlite/src/event_store.rs:270-272`, and `references/adr/0015-validated-identifiers-and-store-limits.md:552-557` all reason about `QueryItems` and conclude — correctly — that a query-item refusal is not an append outcome. None of the four considers `metadata`, which *is* an append outcome.

**One shipping adapter has already paid for the gap in writing.** `crates/happenstance-cloudflare/src/event_store.rs:212-217` derives its `data` ceiling by listing what shares the row:

> an `event_type` of up to 255 bytes, a nullable `metadata` blob the contract
> does not bound at all, the canonical `tags` encoding […] Half the row cap
> leaves a full megabyte for all of that.

The same sentence appears at derivation time in `experiments/durable-object-limits/README.md:124`. That is an adapter buying its safety margin out of the field the contract *does* bound, because the field it could not bound had to fit somewhere — i.e. the shared-budget answer, arrived at by an adapter with no clause behind it.

> **Removed (falsified).** This paragraph previously ended *"adopted informally by
> one adapter with no clause behind it."* It is not one adapter and it is not
> informal. **The replication port implements the shared budget in shipped code.**
> `crates/happenstance-sync/src/identity.rs:193-205`:
>
> ```rust
> /// Bytes this event occupies in a peer's size budget.
> ///
> /// Only the payload and metadata are counted; the type and tags are already
> /// bounded at 255 bytes each by the contract […]
> pub fn payload_len(&self) -> usize {
>     self.event.data().len()
>         + self.event.metadata().map_or(0, happenstance_core::bytes::Bytes::len)
> }
> ```
>
> That sum is the unit `PeerLimits::admits` compares against `max_event_bytes`
> (`crates/happenstance-sync/src/peer.rs:325-330`), and `PeerLimits::minimum()`
> anchors `max_event_bytes` at `65_536` — `MIN_SUPPORTED_EVENT_DATA_LEN`'s number,
> cited as *"the contract's own floor for what a store must accept"*
> (`crates/happenstance-sync/src/peer.rs:308-314`). So the count in the tree is
> **two for the shared budget, one against**, and one of the two is a port with
> rustdoc, not a comment in a derivation.

**And the typed layer writes metadata on every append.** `crates/happenstance/src/command.rs:359`:

```rust
built
    .with_tags(event.tags())
    .with_metadata(crate::codec::frame::<C>(None)),
```

`frame`'s `application` argument exists precisely to copy caller bytes through — `crates/happenstance/src/codec.rs:253-256`: *"`application` is whatever metadata the caller wanted on the event — causation, correlation — and it is copied through after the region, untouched and never parsed."* Today every shipped call site passes `None`, so the bytes are a fixed handful; the seam is built for the case that is not.

---

## What is true today

**The field, and its constructor, admit anything.** `crates/happenstance-core/src/event.rs:321-326`:

```rust
pub struct Event {
    event_type: EventType,
    data: Bytes,
    tags: Tags,
    metadata: Option<Bytes>,
}
```

`crates/happenstance-core/src/event.rs:377-382`:

```rust
/// Attaches opaque client metadata, replacing any already set.
#[must_use]
pub fn with_metadata(mut self, metadata: impl Into<Bytes>) -> Self {
    self.metadata = Some(metadata.into());
    self
}
```

Infallible, no `Result`, no length check — which is correct under ADR-0015's central distinction and must stay that way whatever is decided here. `.kb/decisions/0015-validated-identifiers-and-store-limits.md:81-88`: a capacity limit *"must never be enforced by a constructor and must never be enforced in `Deserialize`"*, because an event refused at decode has no local position and cannot be quarantined. **Any option below that reaches for a constructor bound is already rejected by an accepted decision.**

**`None` and `Some(empty)` are deliberately distinct, and this is checked.** VT-1's rule list names `metadata_distinguishes_absent_from_empty` (`spec/SPECIFICATION.md:604`; the rule is at `crates/happenstance-testkit/src/suite.rs:3549`). VT-1's `Rejects:` (`spec/SPECIFICATION.md:609-612`) names the adapter that writes an empty slice as `NULL`. `happenstance-sqlite` states the obligation twice — `crates/happenstance-sqlite/src/event_store.rs:104-105` (*"`None` and `Some(<empty>)` are two values the contract keeps apart, and a store that folds them has lost one"*) and `crates/happenstance-sqlite/src/row.rs:79`. The core doctest asserts it on the wire too (`crates/happenstance-core/src/event.rs:586-589`).

Consequence for this decision: **a metadata bound cannot be expressed as "absent means zero"**. `None` is not `Some(&[])` and neither is `Some(&[0u8; 0])` folded into it, so any arithmetic over `metadata.len()` has to decide what `None` contributes. **The tree has already decided it, in code:** `map_or(0, Bytes::len)` at `crates/happenstance-sync/src/identity.rs:200-204`. That is precedent, not a clause — what is owed here is writing it down, not choosing it.

**The limits vocabulary has four floors and three variants.** `crates/happenstance-core/src/limits.rs:17-43` declares `MIN_SUPPORTED_EVENT_DATA_LEN = 65_536`, `MIN_SUPPORTED_TAGS_PER_EVENT = 64`, `MIN_SUPPORTED_QUERY_ITEMS = 128`, `MIN_SUPPORTED_EVENTS_PER_BATCH = 128`. Each carries a derivation from something observed — the 128 KiB legacy KV cap, eight tags in the richest of six scenarios, four query items in the largest decision model, and SQLite's `SQLITE_MAX_VARIABLE_NUMBER` arithmetic. There is no `MIN_SUPPORTED_METADATA_LEN`.

`crates/happenstance-core/src/limits.rs:45-61`:

```rust
/// Which capacity limit an append exceeded.
///
/// Three variants and not four: a query-item refusal is not an append outcome,
/// so a `QueryItems` variant would name a refusal no `append` could ever
/// produce. […]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum StoreLimit {
    /// One event's `data` payload was larger than the store accepts.
    EventDataLen,
    /// One event carried more tags than the store accepts.
    TagsPerEvent,
    /// The batch held more events than the store accepts in one append.
    EventsPerBatch,
}
```

**`guaranteed_minimum` is total and returns `usize`, not `Option<usize>`** (`crates/happenstance-core/src/limits.rs:69-76`). That is load-bearing for the options below: **a new variant cannot exist without a number**. Its doc (`:64-68`) says the number is how a caller tells *"this store is stricter than the contract allows" — a conformance bug —* from *"this payload was always going to be too big to replicate"*.

**Neither shipping adapter checks metadata.** `happenstance-sqlite`'s `check_ceilings` (`crates/happenstance-sqlite/src/event_store.rs:517-538`) tests `events.len()`, `event.data().len()` and `event.tags().len()`. `happenstance-cloudflare`'s (`crates/happenstance-cloudflare/src/event_store.rs:467-491`) tests the same three against `Ceilings::DECLARED` (`:234-238`: `event_data_len: 1024 * 1024`, `tags_per_event: 1024`, `events_per_batch: 1024`).

**And `happenstance-sqlite` has already written down the opposite of the shared-bound answer**, in a `pub const`'s documentation — `crates/happenstance-sqlite/src/event_store.rs:236-239`:

> The largest `data` payload this store accepts, in **bytes of
> [`Event::data`]** — not of an encoded row, and not of `data` and
> `metadata` together.

Two shipping adapters therefore disagree in prose about what the `data` ceiling means: cloudflare treats it as a row budget it carved metadata room out of, sqlite states explicitly that it is not.

**Nothing in the skeletons bounds it either**, and both Postgres-shaped ones have a `metadata bytea` column in their sketched schemas (`crates/happenstance-postgres/src/event_store.rs:14`, `crates/happenstance-neon/src/event_store.rs:43`, `:141`). `happenstance-neon` carries a *hard* 64 MiB response ceiling with no cursor fallback (`crates/happenstance-neon/src/transport.rs:49-54`), so metadata size lands on its read path as well as its write path — a second axis, and one where the wall is documented as hard rather than merely documented.

**The wire format doubles the question.** WF-11 (`spec/SPECIFICATION.md:2328-2330`): *"`Event::data` and `Event::metadata` MUST encode as standard-alphabet base64 in human-readable formats"* — so a metadata byte is ~1.33 bytes on a JSON-shaped wire, and WF-11's own `[PROVISIONAL]` falsifier is about a payload too large to buffer.

**Replication depends on metadata *not* being where identity lives, not on its size.** SY-12 is `[FROZEN]` and rejects a peer that carries identity in `Event::metadata` (`spec/SPECIFICATION.md:6558-6576`). Nothing in the *clauses* bounds its length.

**But the sync port already counts it, and counts it in the shared-budget shape.** `ReplicatedEvent::payload_len` is `data.len() + metadata.map_or(0, Bytes::len)` (`crates/happenstance-sync/src/identity.rs:193-205`), its rustdoc says *"Only the payload and metadata are counted"*, and that sum is what `PeerLimits::admits` tests against `max_event_bytes` per event and `max_batch_bytes` per batch (`crates/happenstance-sync/src/peer.rs:322-331`). `PeerLimits::minimum()` sets `max_event_bytes: 65_536` and names it *"the contract's own floor for what a store must accept"* (`crates/happenstance-sync/src/peer.rs:306-315`).

> **Removed (falsified).** This paragraph previously closed: *"The floors' stated
> purpose at `crates/happenstance-core/src/limits.rs:8-10` — 'gives the sync layer
> something to compare a peer's declared limit against **before** it starts
> pushing' — currently has no term for metadata at all."* That is false. The sync
> layer has exactly that term and has had it since the port was written; the brief
> quoted `limits.rs`'s aspiration and never checked whether `happenstance-sync`
> delivered on it. It did. This was the **sole discriminator** the Recommendation
> used to prefer Option 1 over Option 2, and its removal is what flipped the
> recommendation — see the Revision record.

**The specification's numbering.** Highest `VT-` in `spec/SPECIFICATION.md` is **VT-34**; highest `CF-` is **CF-40**. A new clause is VT-35 (and possibly a CF-41 if the fixture gains a constant).

---

## Options

Throughout: none of these may put a bound in a constructor or in `Deserialize` — ADR-0015 forecloses that (`.kb/decisions/0015-validated-identifiers-and-store-limits.md:81-88`). Every option below is a *store-boundary* bound.

### Option 1 — A fourth floor of its own

`MIN_SUPPORTED_METADATA_LEN`, `StoreLimit::MetadataLen`, `Fixture::MAX_METADATA_LEN: Option<usize> = None`, and two rules: a floor rule (`store_accepts_the_guaranteed_minimum_metadata`) and a fourth arm in `append_reports_exceeded_store_limits`. This is the audit's proposed remediation (`review-pre-publication-2026-09-03.md:1030`).

- **Costs a caller:** essentially nothing new to *write*; one more `StoreLimit` variant to handle if they match exhaustively — but `StoreLimit` is already `#[non_exhaustive]` (`limits.rs:53`) so a wildcard arm is already mandatory. It gives them something real: a metadata refusal they can distinguish and park.
- **Costs an adapter author:** one more comparison in `check_ceilings` (four lines in each of two adapters, mechanically identical to the existing three), one more `pub const`, one more fixture constant, and a *documentation* obligation matching VT-21/22/24's *"MUST document its actual limit"*.
- **Semver:** additive on `happenstance-core` — `StoreLimit` is `#[non_exhaustive]`, `guaranteed_minimum` returns `usize` so no signature moves, and `Fixture`'s ceiling constants are defaulted (`crates/happenstance-testkit/src/contract.rs:264`, `:273`, `:290`), so a fourth is additive on `happenstance-testkit`. **The conformance-rule half splits, and the audit does not split it:** the *ceiling* arm is gated on `Option<usize>` defaulting to `None`, so it **skips** for an adapter that declares nothing (`crates/happenstance-testkit/src/suite.rs:4285-4296`); the *floor* rule is ungated and runs against every adapter unconditionally, exactly like `store_accepts_the_guaranteed_minimum_payload` (`crates/happenstance-testkit/src/suite.rs:3775`, registered at `crates/happenstance-testkit/src/registry.rs:180`). So it is only the floor rule that turns a stranger's green suite red on `cargo update`.
- **It is not four lines in two adapters — it changes `PeerLimits`.** A separate floor `N` is **incommensurable with `payload_len`**. Under Option 1 a conformant-minimum event is 65,536 bytes of `data` *plus* `N` bytes of metadata, whose `payload_len()` is `65_536 + N`; `PeerLimits::minimum().admits` compares that against `max_event_bytes: 65_536` and returns `false` (`crates/happenstance-sync/src/peer.rs:312`, `:325-330`). So Option 1 declares an event every store must accept that the contract's own minimum peer must reject. Either `PeerLimits::minimum()` moves to `65_536 + N`, or `payload_len` stops summing the two, or the sync port grows a second term. This cost was **absent from the brief's first pass** and it is the largest single item in Option 1's bill.
- **Forecloses:** the shared-budget reading. Once `MetadataLen` names its own refusal and its own floor, an adapter that refuses a *combined* size has no honest variant to report it through — it would have to name one field for a two-field overflow. The brief originally called that "the same defect this decision exists to close, one level down"; against a tree in which **every** store adapter is row-oriented, and in which the wall is the row rather than the field, that reads at least as easily as an argument *for* the shared budget as against it.
- **The soft spot:** the number. Every other floor in `limits.rs` is derived from an observation. The only metadata any code in this workspace writes is `codec::frame::<C>(None)` — a fixed framing prefix plus a codec tag plus one byte (`crates/happenstance/src/codec.rs:258-269`) — so the evidence base for a metadata floor is *tens of bytes*, and any larger number is an assertion about applications nobody has written. A `[PROVISIONAL]` marker with an adoption-gated falsifier is the honest shape, and it is a shape ADR-0015 already used twice (`references/adr/0015-validated-identifiers-and-store-limits.md:520-527`).

### Option 2 — A refusal channel with no guaranteed capacity

`StoreLimit::MetadataLen` and `Fixture::MAX_METADATA_LEN`, but **no floor**: `guaranteed_minimum()` returns `0` for the new variant, and a clause states that the contract guarantees no metadata capacity — a store may refuse any non-empty metadata, and must say so with this variant rather than through `AppendError::Store`.

- **Costs a caller:** the ability to plan. A caller cannot ask "how much metadata may I attach and still replicate?" and get a useful answer; `guaranteed_minimum()` returning 0 means the branch its own doc describes — *"this store is stricter than the contract allows" — a conformance bug* (`limits.rs:66-67`) — becomes unreachable for this variant.
- **Costs an adapter author:** the same four lines as Option 1, minus the floor obligation. Strictly cheaper: no adapter has to guarantee anything it has not measured.
- **Semver:** additive, and *more* additive than Option 1 in practice — there is no ungated floor rule, so the only new rule is the gated ceiling arm, which skips by default. A third-party adapter on `cargo update` sees a *skip*, not a red.
- **Forecloses:** raising the guarantee later is not free in spirit even though it is free in types. Publishing "0 bytes guaranteed" and later publishing "N bytes guaranteed" is additive to the compiler but is a new MUST on every existing adapter, and it lands as an ungated floor rule at exactly the moment third-party adapters exist. It also leaves `limits.rs` with a variant whose `guaranteed_minimum()` is a number that means "no number", which is the kind of decorative value the corpus is otherwise careful about.

### Option 3 — One shared payload budget

VT-21's floor and every adapter's declared ceiling apply to `data.len() + metadata.map_or(0, Bytes::len)`. No new variant, no new floor; `StoreLimit::EventDataLen`'s documentation is rewritten to name the event's payload bytes rather than the `data` field.

- **Costs a caller:** a coupling they did not have. A caller whose payload sits near a store's ceiling now has its accepted size depend on a metadata blob the *typed layer* attaches on their behalf (`crates/happenstance/src/command.rs:359`), which is a payload budget that moves when a codec tag changes length.
- **Costs an adapter author:** the least code of any option — one addition in an existing comparison — but it makes every already-published number mean something new.
- **Semver:** type-additive, **behaviourally breaking**, and breaking against text already in the tree. `crates/happenstance-sqlite/src/event_store.rs:236-239` says in a `pub const`'s rustdoc that the number is *"not of `data` and `metadata` together"*, and `StoreLimit::EventDataLen`'s own doc in `happenstance-core` says *"One event's `data` payload"* (`crates/happenstance-core/src/limits.rs:102-103`). Adopting this option makes both false and makes an append that succeeds today fail tomorrow (1,048,576 bytes of `data` plus any metadata). **Of those two, only the second is published** — see the corrected pricing below and in Cost of delay.

  > **Removed (falsified).** This bullet previously ended *"Free now because nothing
  > is published; not free after 0.1."* `happenstance-core`, `happenstance` and
  > `happenstance-testkit` have been live on crates.io at **`0.2.0-alpha.1` since
  > 2026-08-16** (`448e1ac`; transcripts in
  > `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/publish-0-2-0-alpha-1/_release-log.md` §5).
  > There is no 0.1 release — 0.1.0 became 0.2.0. What actually keeps this cheap is
  > narrower and needs stating: `0.2.0-alpha.1` is a **pre-release**, and a bare
  > `"0.2"` requirement never selects one (release log §5.3), so no consumer can be
  > resolved onto the published text by accident. **The deadline is 0.2.0 stable,
  > not "first publish".**
- **Forecloses:** an adapter for which the two fields genuinely live in different places with different limits — a store with `data` in object storage and `metadata` in a row, say. It also cannot express the neon read-path axis, where the 64 MiB response ceiling (`crates/happenstance-neon/src/transport.rs:49-54`) is over the *whole result set* rather than one event.
- **In its favour, and this was under-weighed on the first pass:** it is what `happenstance-cloudflare` is already doing (`crates/happenstance-cloudflare/src/event_store.rs:212-217`) **and what `happenstance-sync` already does in code** (`identity.rs:193-205`, `peer.rs:322-331`, anchored at `MIN_SUPPORTED_EVENT_DATA_LEN`'s own 65,536). It is the only option that matches how a row-oriented store actually fails — the wall is the row, not the field — and it is the only option that needs **no** change to `PeerLimits`, because `PeerLimits` is already written in its arithmetic.

### Option 4 — A written statement that metadata is deliberately unbounded

A new VT clause saying so, plus an obligation that every adapter document how it sizes its *other* declared ceilings to leave metadata room. No change to `limits.rs`, no new variant, no new rule (or one documentation-shaped rule at most).

- **Costs a caller:** nothing to write, and no way to be told. The event that will never fit still arrives as `AppendError::Store` or as a mislabelled `EventDataLen`, so the E2E-42 disappearance (`spec/E2E-CASES.md:1105-1125` — *"a peer that **quarantines** rather than appends: the quarantined event has no local position, is never forwarded, and disappears permanently"*) stays reachable through the channel VT-25 was written to close.
- **Costs an adapter author:** a paragraph, and an impossible sizing exercise. You cannot size around an unbounded field. Cloudflare's 1 MiB of slack (`event_store.rs:234-236`) is defeated by 1 MiB of metadata beside a 1 MiB payload, and `check_ceilings` at `crates/happenstance-cloudflare/src/event_store.rs:467-491` passes that batch straight through.
- **Semver:** none. It is a specification and documentation change only.
- **Forecloses:** nothing structurally — it is the "decide later" option, and every other option remains open after it. What it costs is that VT-25's `[FROZEN]` MUST goes on being unsatisfiable for one field, in writing, with a clause acknowledging it.

---

## Recommendation

**FLIPPED. Option 2 — a refusal channel with no guaranteed capacity — replacing this brief's original recommendation of Option 1.**

The first pass recommended Option 1 (a fourth floor of its own) and conceded in the same paragraph that *"if the decider does not share that judgement, Option 2 is the right answer and this brief does not argue otherwise."* The judgement in question rested on one claim, and the claim was false. See the Revision record.

Why Option 2:

- **Against Option 1 (a floor of its own):** the sole discriminator is gone. The brief argued a floor was owed because the sync layer had *"no term for the field the typed layer writes on every append."* It has one — `ReplicatedEvent::payload_len` — and has had one all along. With the discriminator removed, Option 1 is Option 2 plus a number nobody measured. And Option 1 now carries a cost the first pass missed: a floor `N` is incommensurable with `payload_len`, so a conformant-minimum event fails `PeerLimits::minimum().admits` and the sync port has to change (`crates/happenstance-sync/src/peer.rs:312`, `:325-330`). Option 2 forces no such change, because it guarantees no capacity to be incommensurable with.
- **Against Option 4 (stated unbounded):** unchanged and untouched by either critique. Option 4 is the only option that leaves a `[FROZEN]` MUST unsatisfiable by construction. VT-25 says a store MUST NOT report a capacity refusal through `AppendError::Store` and there is no other variant that fits; "size around it" is not an instruction an adapter can follow against an unbounded field, and cloudflare's margin shows what following it looks like — a guess written into a different constant's derivation. Option 2 closes that hole with the same variant Option 1 would have used.
- **Against Option 3 (shared budget): this brief no longer claims the evidence discriminates.** The original anti-Option-3 case had two legs and both were damaged. The prose-conflict leg pointed at `happenstance-sqlite`'s `pub const` and called it *"published-shaped"* — `happenstance-sqlite` is **not published**; the genuinely published text Option 3 would falsify is `StoreLimit::EventDataLen`'s own doc in `happenstance-core` (`crates/happenstance-core/src/limits.rs:102-103`), and that is live only at a pre-release no `^0.2` resolves to. The "cannot name which field overflowed" leg cuts both ways once you notice every store adapter in the tree is row-oriented. **Option 3 is a live contender and this brief cannot rank it against Option 2 on evidence.** Option 2 is preferred here on one narrow ground only: it is the option that changes nothing already written, so it can be taken without re-deciding what `payload_len`, VT-21 and `EventDataLen`'s doc mean. That is a preference for reversibility, not a finding.

**Split the landing — and Option 2 makes the split trivial.** Option 2's whole surface is skip-by-default: the variant, `Fixture::MAX_METADATA_LEN` and the fourth arm of `append_reports_exceeded_store_limits` are gated on `Option<usize>` constants defaulting to `None` (`crates/happenstance-testkit/src/suite.rs:4285-4296`). There is no ungated floor rule, so there is no piece that turns a stranger's green suite red on `cargo update`. The whole of Option 2 lands whenever.

**The strongest argument against this recommendation, in its own words:**

> Option 3 is dismissed as one adapter's informal habit. But the replication port
> implements it too: identity.rs:193-205 sizes a replicated event as data + metadata,
> rustdoc saying only those two count; PeerLimits::max_event_bytes is checked against
> that sum, anchored at 65,536 = MIN_SUPPORTED_EVENT_DATA_LEN. So it is 2-1 for the
> shared budget with sync on that side, and the tree already answers the brief's own
> open item ("what does None contribute?" - 0, in code). Option 1 is mispriced: a
> separate floor N is incommensurable with payload_len, so a conformant-minimum event
> (65,536 data + N metadata) fails PeerLimits::minimum().admits - Option 1 forces
> PeerLimits to change, not "four lines in two adapters". Every adapter here is
> row-oriented, so naming one field for a row overflow is the defect Option 1 claims to
> close. Minimum flip: Option 2, by the brief's own concession.

The first two sentences of that are why the recommendation flipped. The last sentence is why it flipped only as far as Option 2: the critique's own stated minimum is Option 2, and it argues Option 3 is *better supported*, not that Option 3 is settled. This brief takes the minimum and hands the 2-vs-3 choice to the decider with the evidence on the table.

**And the second objection, in its own words, which does not change the recommendation but does change what the decider is deciding under:**

> The operative half of the recommendation — the landing split and the whole
> Cost-of-Delay section — is calibrated to a pre-publication window that closed 18 days
> before the brief was written […] The correct restatement inverts it: the release
> boundary is already behind us, and what keeps the ungated floor rule cheap today is
> not that nothing is published but that 0.2.0-alpha.1 is a pre-release no ^0.2
> resolves to — a fact the brief never states and the audit makes its spine. The
> deadline is 0.2.0 stable. The anti-Option-3 headline also inverts publication:
> sqlite's pub const is called "published-shaped" (:183) though sqlite is only at
> 0.0.0, while core's genuinely published StoreLimit doc is priced free. Option 1 vs 2
> is untouched.

Recorded verbatim including one detail this brief could not confirm: `happenstance-sqlite` inherits `version.workspace = true` and so reads `0.2.0-alpha.1` in the tree, not `0.0.0`. Its substance is unaffected — sqlite was not in the publish wave (`_release-log.md`: *"Crates | `happenstance-core`, `happenstance`, `happenstance-testkit`"*), so its rustdoc is unpublished either way.

**What would decide 2 vs 3:** whether the workspace intends `payload_len`'s arithmetic to *be* the contract's answer or to be an artefact of the sync port. That is a question for whoever owns ADR-0026/0027, and it can be asked without a measurement. **What would still recover Option 1** is what the first pass named: one real application or `references/scenarios/` entry attaching a metadata blob whose size can be quoted, or a measured metadata wall on any target — plus, now, an answer for `PeerLimits::minimum()`.

**Retired with the flip:** the original "strongest argument against", which said Option 1 minted an unmeasured number into a module where every number is derived from an observation, and concluded *"Option 2 closes the same `[FROZEN]` hole, admits the same refusals, and asserts nothing it cannot show."* It is retired because it is no longer an argument *against* the recommendation — it is now one of the reasons for it.

---

## Correction to the audit entry

**The Routing field's "it cannot be an amendment" is contradicted by repository precedent.** AE-4 says (`review-pre-publication-2026-09-03.md:1032`): *"ADR-0015 is `accepted` […] and therefore immutable, so this needs a superseding atom."* Immutability is right; the inference is not the only one available. The tree carries **two** patterns for correcting an accepted decision, and they are used for different situations:

- **Supersession**, when the earlier record's reasoning is wrong — `.kb/decisions/0032-adr-0021-serde-attribution-correction.md:10-11` carries `supersedes: [kb-decision-0021]`, and its summary calls itself *"A repair of ADR-0021, not an amendment."*
- **Amendment without supersession**, when the earlier reasoning is what the new decision *acted on* — `.kb/decisions/0029-msrv-raised-to-1-97-1.md:10` carries `supersedes: null` with `depends_on: [kb-decision-0004]`, and its summary states: *"This amends ADR-0004 rather than superseding it - that decision's body stays verbatim, because its reasoning is what this one acted on."*

ADR-0015's "three variants, not four" reasoning is **not wrong**. Its argument — that a query-item refusal is not an append outcome and would mint a variant no `append` could construct (`references/adr/0015-validated-identifiers-and-store-limits.md:552-557`) — is correct on its own terms and survives this decision intact. What is missing is a candidate it never enumerated. That is the ADR-0029 shape, not the ADR-0032 shape.

**So: the atom this would supersede is, on the evidence, none.** The likely correct frontmatter is `supersedes: null`, `depends_on: [kb-decision-0015]`, with the amendment stated in the summary — and superseding `kb-decision-0015` wholesale would additionally mark the identifier-validation, byte-equality and `Cow<'static, str>` halves of that atom superseded, which nothing here touches. The human should confirm the shape, but the audit's phrasing forecloses an option the repository uses.

**Two smaller corrections:**

- **The conformance-rule cost is one rule, not two.** AE-4's Semver paragraph says *"a new rule is a testkit minor that every adapter takes on its next `cargo update`, and it turns a green suite red for a store that was conformant the day before."* That is true of the **floor** rule and false of the **ceiling** arm: `append_reports_exceeded_store_limits` gates on `Option<usize>` constants that default to `None` and returns `RuleOutcome::Skipped` when a store declares nothing (`crates/happenstance-testkit/src/suite.rs:4285-4296`). Only the ungated floor rule carries the involuntary-red cost. This materially changes the deadline argument: half the remediation is free at any time.
- **The failure the entry predicts is derived from documentation, not measured.** AE-4 leans on the Durable Object's *"documented 2 MiB row cap"* as *"a hard wall the adapter does not own"*. `crates/happenstance-cloudflare/src/event_store.rs:180-184` records that on the executing runtime *"no per-value refusal is observable at 8 MiB of payload, 8,192 tags or a 4,096-event batch"* and that *"the physical wall was never located."* The defect stands — a documented cap is a contractual wall regardless, and neon's 64 MiB response ceiling is documented as hard — but the concrete overflow AE-4 describes has not been observed in this workspace, and the entry does not say so. If the decider wants the urgency argument to carry its own weight, that is the experiment to run first.
- **The RUNBOOK ADR-queue number is not free the way the queue says.** `RUNBOOK.md:262` records *"0008–0028 are free"*; `.kb/decisions/` now holds 0029 through 0036. The next free number is **0037**, and the queue line is stale.

---

## Cost of delay

**Rewritten. The section this replaces was calibrated to a pre-publication window that closed on 2026-08-16, eighteen days before this brief was written.**

> **Removed (falsified).** Three claims went with it: *"the ungated floor rule is
> cheap now and permanently expensive **after first publish**"*, *"that is the real
> deadline and it is the release boundary"*, and *"Option 3 is free now and closed
> after 0.1."* The release boundary is **behind us**. `happenstance-core`,
> `happenstance` and `happenstance-testkit` went live on crates.io at
> `0.2.0-alpha.1` on 2026-08-16 (`448e1ac`), and there was never a 0.1 release —
> 0.1.0 became 0.2.0. Every "before publish / after publish" framing below is
> restated against the boundary that is actually still ahead.

**What actually keeps the remaining work cheap.** `0.2.0-alpha.1` is a **pre-release**, and cargo will not select a pre-release for a bare `"0.2"` requirement — a consumer has to write `happenstance = "0.2.0-alpha.1"` explicitly (release log §5.3, and the README's `## Stability` section says the same). So the published crates exist, but nothing resolves onto them by accident and nothing is pinned to them by a range. **The deadline is `0.2.0` stable**, which is when a bare `"0.2"` starts resolving and the published text becomes a promise.

- **The mechanism is free forever.** `StoreLimit` is `#[non_exhaustive]` (`crates/happenstance-core/src/limits.rs:53`), `guaranteed_minimum` returns `usize` rather than `Option<usize>` (`:70`) so a new variant needs a number but not a signature change, and `Fixture`'s three ceiling constants are defaulted (`crates/happenstance-testkit/src/contract.rs:264`, `:273`, `:290`) so a fourth is additive. This is *additive by luck rather than by design* — had any one of the three gone the other way the field would already be unreachable, because these crates are live. The luck held, so the variant is as cheap now as it was in July.
- **The recommended option (2) has no deadline at all.** Its entire surface is gated on defaulted `Option<usize>` constants and skips for an adapter that declares nothing. There is no ungated rule in it, so there is no "green suite goes red on `cargo update`" cost to beat a date on. This is the second reason to prefer it over Option 1 and it did not exist in the first pass, which was pricing Option 1's ungated floor rule against a deadline that had already passed.
- **An ungated floor rule — Option 1's, or a later raise from Option 2's zero — is priced against `0.2.0` stable, not against "publish".** Today the suite has one third-party-shaped consumer in the tree (`examples/outside-projection-adapter/`, and it is a projection adapter), and a published pre-release nobody resolves to by default. Once `0.2.0` ships, a new ungated rule is a testkit minor every adapter anywhere takes on `cargo update` and goes red for a store that was conformant the day before.
- **Option 3 is still cheaper before `0.2.0` than after, but it was never free.** It rewrites what `StoreLimit::EventDataLen`'s **published** doc means (`crates/happenstance-core/src/limits.rs:102-103`) and changes what an already-accepted append does. Choosing Option 1 or 2 forecloses it; so does `0.2.0`. If the shared-budget reading has appeal — and after the sync evidence above it has more than this brief first gave it — the cheap moment is now and it is a shorter runway than the first pass implied.

**And there is a hard gate.** Phase 12's exit criterion (`RUNBOOK.md:4723-4728`) requires that every `[PROVISIONAL]` clause published at the stable release either has a scheduled falsifier or sits behind an unstable feature. VT-21 is `[PROVISIONAL]` and its falsifier is about `data` (`spec/SPECIFICATION.md:1520-1522`). A metadata decision that mints a new `[PROVISIONAL]` clause inherits that criterion and must state its falsifier's reachability the way `references/adr/0015-validated-identifiers-and-store-limits.md:511-527` did — including saying "adoption-gated" out loud if that is what it is, because CF-38 forbids an empty falsifier and the corpus has already been caught once by a marker nobody could check.

---

## What this does not settle

- **The number — now moot under the recommendation, and still open behind it.** Under Option 2 there is no `MIN_SUPPORTED_METADATA_LEN` to value: `guaranteed_minimum()` returns `0` and the brief asserts nothing. If the decider takes Option 1 instead, the number is unchosen and there is no observation in the workspace to derive one from — the only writer is `codec::frame` at tens of bytes (`crates/happenstance/src/codec.rs:258-269`) — **and** it must be reconciled with `PeerLimits::minimum().max_event_bytes`, which is 65,536 and compares against `data + metadata`.
- **Whether `payload_len`'s arithmetic is the contract's answer or the sync port's own.** This is the live 2-vs-3 question and nothing here settles it. `crates/happenstance-sync/src/identity.rs:193-205` sums `data` and `metadata` and no clause says it may; if that sum is *right*, Option 3 is already half-implemented, and if it is a local convenience, it is a port that quietly narrows what the contract guarantees. Neither reading is written down anywhere.
- **Whether `happenstance-cloudflare`'s declared `event_data_len` moves.** If a metadata floor or ceiling lands, cloudflare's 1 MiB is derived from an arithmetic that now has a fourth term (`crates/happenstance-cloudflare/src/event_store.rs:212-217`), and the experiment that produced it would have to be re-run. `experiments/durable-object-limits/README.md` names re-running as a loud failure rather than a silent one, so the gate would say — but the number is not decided here.
- **Whether `happenstance-sqlite` declares a metadata ceiling at all.** Its own limit is near SQLite's, so it may honestly declare `None` and skip the ceiling arm; that is an adapter decision, not a contract one.
- **The wire and replication half.** WF-11's base64 expansion (`spec/SPECIFICATION.md:2328-2330`) and neon's hard 64 MiB response cap (`crates/happenstance-neon/src/transport.rs:49-54`) both make metadata size a *read-path* and *transport* question, and no `StoreLimit` variant addresses a read.

  > **Removed (falsified).** This item previously closed *"Whether the sync port
  > needs its own term for a peer's metadata capacity belongs to ADR-0026/0027 […]
  > not here."* The sync port **has** a term — metadata is inside `payload_len` and
  > therefore inside `max_event_bytes` (`crates/happenstance-sync/src/peer.rs:287`,
  > `:325-330`). What belongs to ADR-0026/0027 is the opposite question: whether
  > that term should stay a *combined* one once the contract decides, and what
  > `PeerLimits::minimum()` becomes if the contract mints a separate metadata floor.
- **`None` versus `Some(&[])` in the arithmetic — narrowed to ratification.** The brief originally listed this as an open choice. It is not a choice: `crates/happenstance-sync/src/identity.rs:200-204` already contributes `0` for `None`, in shipped code. What is still owed is a clause saying so, because VT-1 and `metadata_distinguishes_absent_from_empty` make them two values rather than one (`spec/SPECIFICATION.md:604`, `609-612`) and nothing normative currently blesses the sync port's reading.
- **Whether VT-21's marker moves.** This decision may mint a new clause without touching VT-21's `[PROVISIONAL]` status; ADR-0015's own rule applies — *"Neither ADR may lift the other's marker in passing"* (`references/adr/0015-validated-identifiers-and-store-limits.md:531-533`).
- **The ADR's number and the RUNBOOK row.** AE-4 is right that `RUNBOOK.md`'s ADR queue has no row for this. It needs one, at 0037 or later, with an owner and a phase.

---

## Revision record

**Revision 2, 2026-09-03.** Two critiques were filed against revision 1 and **both falsified a premise**. The recommendation flipped from **Option 1 to Option 2**. Neither critique's finding is disputed; both were checked against the tree before folding them in.

### Critique A — the sync layer already has a metadata term (recommendation-flipping)

**The falsified premise.** Revision 1's Recommendation gave exactly one reason to prefer Option 1 over Option 2: with `guaranteed_minimum() == 0` *"there is nothing to compare, and the replication planning that motivated the whole `limits` module has no term for the field the typed layer writes on every append."* That is false, and it was falsifiable by opening one file. `crates/happenstance-sync/src/identity.rs:193-205` defines `ReplicatedEvent::payload_len` as `data.len() + metadata.map_or(0, Bytes::len)` with rustdoc stating *"Only the payload and metadata are counted"*; `PeerLimits::admits` compares that sum against `max_event_bytes` (`crates/happenstance-sync/src/peer.rs:322-331`); `PeerLimits::minimum()` anchors it at `65_536`, `MIN_SUPPORTED_EVENT_DATA_LEN`'s own number (`:306-315`). Revision 1 quoted `limits.rs:8-10`'s aspiration and never checked whether `happenstance-sync` delivered on it.

**What was removed, not rewritten around:**

1. The "no term for metadata at all" sentence in *What is true today*, with a marked removal note in its place and the shipped arithmetic stated instead.
2. "Adopted informally by one adapter with no clause behind it" in *Why this is owed* — it is two implementations, one of them a port with rustdoc.
3. The 1-vs-2 discriminator in the Recommendation, and with it the recommendation of Option 1.
4. "`None` contributes 0 … must be *written*, not assumed" as an open choice in two places — it is already written, in `identity.rs:200-204`. Narrowed to "owed a clause", not "owed a decision".
5. "Whether the sync port needs its own term … belongs to ADR-0026/0027" in *What this does not settle*.

**What was added.** Option 1's unpriced cost: a separate floor `N` is incommensurable with `payload_len`, so a conformant-minimum event (65,536 `data` + `N` metadata) fails `PeerLimits::minimum().admits`. Option 1 therefore forces a change to `PeerLimits`, not "four lines in two adapters". Option 3's "in its favour" bullet gained the sync evidence and the observation that Option 3 alone requires no `PeerLimits` change.

**How far it flipped, and why not further.** The critique argues the tree is **2-1 for the shared budget** and that Option 3 is the better-supported reading; it names **Option 2 as the minimum flip, by the brief's own concession**. This revision takes the minimum. Revision 1's claim that *"the evidence discriminates cleanly against Options 3 and 4"* is **withdrawn as to Option 3** — the brief now states that it cannot rank 2 against 3 on evidence, and prefers 2 only on reversibility. Option 4's rejection is untouched by either critique and stands.

### Critique B — the publication premise (does not flip, but recalibrates)

**The falsified premise.** Revision 1 asserted at three places that nothing was published: *"Free now because nothing is published; not free after 0.1"*, *"Option 3 is free now and closed after 0.1"*, and *"After publish, every adapter anywhere takes a new ungated rule … That is the real deadline and it is the release boundary."* `happenstance-core`, `happenstance` and `happenstance-testkit` have been live on crates.io at `0.2.0-alpha.1` **since 2026-08-16** (`448e1ac`; transcripts in `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/publish-0-2-0-alpha-1/_release-log.md` §5). There is no 0.1 release; 0.1.0 became 0.2.0. Revision 1 reintroduced the exact premise its own source audit files as defect N-2 against ADR-0029.

**What was removed and replaced.** The whole *Cost of delay* section was rewritten with a marked removal note naming the three claims. The correct spine is narrower: `0.2.0-alpha.1` is a **pre-release** and a bare `"0.2"` requirement never selects one, so nothing resolves onto the published text by accident. **The deadline is `0.2.0` stable, not "first publish".** Option 3's semver bullet lost the same sentence.

**The inverted pricing, corrected.** Revision 1's headline argument against Option 3 called `happenstance-sqlite`'s `pub const` *"published-shaped"* while pricing `happenstance-core`'s genuinely published `StoreLimit::EventDataLen` doc at nothing. Exactly backwards: `happenstance-sqlite` was not in the publish wave. The anti-Option-3 prose-conflict argument now points at `crates/happenstance-core/src/limits.rs:102-103`, where it belongs, and is qualified by the pre-release fact.

**One correction to the critique itself, recorded rather than silently dropped.** Critique B states *"sqlite is only at 0.0.0"*. `crates/happenstance-sqlite/Cargo.toml` uses `version.workspace = true`, so it reads `0.2.0-alpha.1` in the tree. The substance is unaffected — sqlite is unpublished either way, which is the load-bearing half — and the critique is quoted verbatim under Recommendation with this noted beneath it.

**Untouched by critique B, and the critique says so:** the Option 1 versus Option 2 comparison. The flip is critique A's alone.

### Not changed

The *Options* framing, the *Correction to the audit entry* section (the ADR-0029-shaped amendment argument, the one-rule-not-two rule-cost split, the unmeasured-Durable-Object-wall correction, and the stale `RUNBOOK.md:262` ADR-queue number), and the rejection of Options 1-4's constructor-bound variants under ADR-0015. No repository file was modified; this brief is the only file this revision touched.
