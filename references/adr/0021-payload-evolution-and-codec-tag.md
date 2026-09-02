# ADR-0021: The codec tag lives in `Event::metadata`, event types do not carry versions, and upcasting happens at decode

**Status:** accepted · **Phase:** 7 · **Date:** 2026-08-15 ·
**Reversibility:** low · **Amends:** nothing · **Supersedes:** nothing ·
**Related:** ADR-0003, ADR-0007, ADR-0016

`RUNBOOK.md:301` holds this slot, and phrases it as four questions in one sentence:

> **0021** | 7 | How does a payload's shape evolve — codec tag, versioned event
> types, upcasting, and does the read path need a hook it does not have?

Three answers, and none of them was taken at the design gate — `_design.md:674-679`
hands all three to this record in terms, because the public surface is invariant
under them and the design did not wait. So this record **takes** decisions rather
than transcribing them, which is why every one is stated as a commitment and every
loser is named with the wrong implementation it admits.

1. **The codec tag lives in `Event::metadata`**, inside a framing region the typed
   layer owns and no store ever parses.
2. **`EventType` does not carry a version suffix.** An event type is a stable
   identity; the payload is what evolves.
3. **No read-path hook is needed**, and the strategy that makes that true is
   **decode-time tolerance** in `DomainEvent::decode`.

Plus the rule the three imply and M3 would otherwise have to invent: **an event
with no framing region decodes with the codec in hand.**

---

## Context

### A typed layer has an evolution problem from its first commit

`happenstance-core` carries opaque `Bytes` and has no evolution problem at all —
bytes never need reinterpreting, which is exactly what ADR-0003 bought. The
**typed** layer inherits one immediately: a decoded
`Enrolment::Defined { capacity: u32 }` breaks the moment a field is added, and a
store that holds two encodings at once needs a way to tell them apart **that no
adapter is allowed to understand.**

Project AC-005 already requires the tag to exist (`project.md:177-180`). This record
decides where it lives, what evolution strategy it belongs to, and what a reader
does when it is absent.

### The constraint that binds every answer, and its falsifier

> **No adapter may need to understand the tag.**
> (`_decomposition.md:565-570`; `_design.md:678-679`)

Stated as a preference that is unfalsifiable, so it is stated here as a **check**:

> **Name the adapter change this choice forces.**

That is ADR-0003's own reasoning — payloads stay opaque so no adapter acquires
domain knowledge — and the ground on which ADR-0007 rejected a decoding projection
store: decoding is the typed layer's, and a component below the port that must
decode has been handed knowledge the port exists to withhold
(`.kb/decisions/0007-projection-runner-decodes.md`, `summary`: the pump *"never
decodes"*).

### VT-3, read in both directions — because only one of them decides anything

The clause is `[FROZEN]` at `spec/SPECIFICATION.md:629-645`, and it has two halves:

> The contract layer, a store adapter, and a peer MUST NOT parse `data` or
> `metadata`. **Any value that a store, a peer, a conformance rule or a query must
> be able to see MUST be carried in the `EventType` or in `Tags`.**
> — `:631-633`

The obvious reading — *stores must not parse `metadata`, therefore `metadata` is
safe* — uses only the first half, and the first half on its own would licence
putting anything anywhere. **The second half is the one that decides the siting**,
and it decides it by asking a prior question:

> **Does anything below the port need to see the codec tag?**

It does not. Decoding is strictly above the port (ADR-0007). A store serves bytes;
a peer moves bytes; `QueryItem::matches` looks at `event_type` and `tags` and
nothing else (`crates/happenstance-core/src/query.rs:112-115`); no conformance rule
can observe an encoding it is forbidden to parse. So VT-3 does **not** require
`Tags` — and if the honest answer to that question had been *yes*, VT-3 would have
**required** `Tags`, and the constraint *no adapter may need to understand the tag*
would already have been violated by the requirement itself. The clause and the
constraint agree, and they agree for the same reason.

**The `Rejects:` line is the mirror case, not a counter-example.** VT-3 rejects
*"an ingest path that writes the origin's identity into `Event::metadata`"*
(`:643-646`). That is a value a **peer must see** — replication identity is exactly
the thing below the port that has to reason — so the clause sends it to
`EventType`/`Tags`. The codec tag is the opposite: a value nothing below the port
may see. Same clause, opposite answers, and the discriminator is the question
above.

---

## Decision 1 — the codec tag lives in `Event::metadata`

`Event::with_metadata` (`crates/happenstance-core/src/event.rs:379`) and
`Event::metadata() -> Option<&Bytes>` (`:400`) are the affordance. The typed layer
writes the tag into a **framing region** at a fixed, self-describing position in
those bytes; everything outside the framing region is the application's, verbatim,
and `happenstance` neither reads nor rewrites it.

### The falsifier, applied mechanically to both candidates

One row per obligation the choice puts on the adapter population.

| Adapter obligation | `Event::metadata` | `Tags` |
| --- | --- | --- |
| **Store it** | already stored. `metadata` is `Option<Bytes>` on every `Event` and every adapter persists it byte-for-byte | stored — but as a *tag*, in whatever table or index the adapter uses for tags |
| **Index it** | **none.** No adapter indexes `metadata`, and nothing queries on it | **yes.** Matching reads tags, so the tag joins whatever structure serves matching in every adapter |
| **Match on it** | **none.** VT-3 forbids a store parsing `metadata` | **yes, unavoidably.** `QueryItem::matches` looks at `event_type` and `tags` (`query.rs:112-115`). A tag participates in query semantics whether or not anyone wanted it to |
| **Budget it** | **none.** It rides in the payload-class budget the store already accepts | **yes.** One of the 64 tags every store must accept (`MIN_SUPPORTED_TAGS_PER_EVENT`, `crates/happenstance-core/src/limits.rs:27`) is spent on every event, forever |
| **Migrate it** | **none** | **yes, and silently** — see below |
| **Parse it** | forbidden below the port, and nothing below the port needs to | required, in the only sense that counts: matching *is* reading it |

**`Tags` forces a change in every adapter. `Event::metadata` forces none.** Not
because one adapter would have to parse the tag — none would — but because *all* of
them would be obliged to store, index, match on and budget it. The falsifier is not
symmetric, and that asymmetry is the whole argument.

### The migration row is the one that corrupts

The tag exists so one store can hold **more than one encoding** — that is its only
purpose. So re-encoding is not an exotic event; it is the event the tag was
invented for. If the tag is a `Tag`:

> re-encoding an event changes its tag set → which changes the `QueryItem`s it
> satisfies → which changes which decision models read it.

A **consistency boundary silently changes shape as a consequence of a
storage-format migration that was supposed to be invisible.** Nothing rejects it:
`Tags::from_pairs([("codec", "json")])` validates
(`crates/happenstance-core/src/tag.rs:304-312`), the whole gate is green, and every
per-codec round-trip test passes. The only instrument that catches it is the
falsifier above.

### The accepted cost, discharged rather than waved through

`metadata` is the field an application wants for causation and correlation. Siting
a typed-layer-private structure inside a public opaque field is a real cost, and
here is how the two coexist.

**`Event::metadata` is split into two regions.** A leading, **versioned framing
region** owned by `happenstance`, and everything after it, owned by the
application. The rules:

1. **`happenstance` reads and writes only the framing region.** The application's
   bytes are carried through unmodified, in whatever encoding the application chose.
   `happenstance` never parses them and has no opinion about them.
2. **The framing region is self-describing and readable without knowing the payload
   codec.** It must be, because reading it is *how* the payload codec is
   discovered. It therefore cannot be encoded by a `Codec`, and must not depend on
   any codec feature — a build with only `postcard` enabled must still read a tag
   written by a build with only `json`.
3. **The framing region adds no dependency.** It must be readable with no default
   features and on `wasm32`, so it is not `serde`-encoded and is not JSON — see
   *Which instrument covers rule 3* below for the gate step that actually proves it.
4. **The framing region is versioned**, so the framing itself can evolve without
   re-siting the tag — which, per *Reversibility* below, is the expensive move.
5. **Metadata with no framing region is untagged**, and Decision 4 says what a
   reader does with it. This is what keeps every event written before the typed
   layer existed readable.

**The cost, stated plainly:** an application that reads `event.metadata()` at the
*contract* level — below `happenstance` — gets the framing bytes as well as its
own, and must skip them. That is the price of a private structure in a public
opaque field, it is paid by exactly one caller shape, and the alternative was
paying it in every adapter instead.

**The exact bytes of the framing region are not decided here.** They are
`codec-and-feature-forwarding`'s (M3), bounded by rules 2–5 above. This record
**sites** the tag; it does not format it.

---

## Decision 2 — `EventType` does not carry a version suffix

**An event type is a stable identity. The payload is what evolves.**

`EventType::from_static` is `const` and would accept `"CourseDefined.v2"` happily
(`crates/happenstance-core/src/event.rs:108-122` — the validator refuses empty,
over-long, control and bidirectional-control strings, and a dot is none of those).
`EVENT_TYPES` is a `const` list of **exact strings**. So the naive "yes" compiles,
round-trips, and is indistinguishable from a correct decision at review time.

It is wrong, and the worked example shows exactly how. Every `Query` already
written names its types **exactly** — `examples/course-subscriptions/src/main.rs:117`:

```rust
QueryItem::new(
    [COURSE_DEFINED, STUDENT_SUBSCRIBED, STUDENT_UNSUBSCRIBED],
    Tags::from_pairs([("course", course)])?,
)?,
```

with `const COURSE_DEFINED: &str = "CourseDefined";` at `:29`. Matching is
`self.types.binary_search(event_type).is_ok()` (`query.rs:113`) — exact equality,
no prefix rule, no normalisation. So the first event written as
`"CourseDefined.v2"` is invisible to that query, and to every query already written
against the old name. In the worked example the consequence is concrete and silent:
`capacity` resolves to `None`, the handler bails with *"course does not exist"*
(`main.rs:158-160`), and the append condition that was supposed to protect the
consistency boundary matches nothing at all.

**A version suffix without a rule for how a query names versions is not an
evolution strategy; it is a way to make old events unreachable in one commit.** The
rule such a suffix would need — every query enumerating every live version of every
type it selects, forever — is more ceremony than the thing it buys, on a project
already carrying a 2.4:1 ceremony ratio (ADR-0020).

**What carries evolution instead.** The payload, decoded tolerantly (Decision 3),
identified by the codec tag (Decision 1).

**The corollary that keeps this honest.** If a change is a genuinely *new fact*
rather than a new shape of an existing one, it takes a **new event type name** —
and then every query that must see it names it, deliberately. The visible cost is
the point: it makes the widening of a consistency boundary a decision somebody
takes rather than a consequence of a serialiser change.

---

## Decision 3 — no read-path hook is needed, and here is the strategy that makes it true

`EventStore` is `[FROZEN]` (`crates/happenstance-core/src/store.rs:93-269`) and has
exactly four required methods: `read` `:119`, `append` `:213`, `head` `:248`,
`contains_event_id` `:268`. There is no hook of any kind, and
`read_decision_model` — often recited as a fifth method — is a **free function** at
`:321`, not part of the trait. Either way: nowhere to register an upcaster.

"No hook is needed" is therefore the answer that requires no work and leaves no
trace, and if it is wrong it is discovered by an application in production, which is
P1's stated fear (`_decomposition.md:45-49`). So it is **earned** here, by naming the
strategy:

**Decode-time tolerance.** The signed-off surface already permits it
(`_design.md:396-408`):

```rust
fn decode<C: Codec>(
    codec: &C,
    event_type: &EventType,
    data: &bytes::Bytes,
) -> Result<Self, CodecError>;
```

`decode` receives the event type **and** the raw bytes and returns a
`Result<Self, CodecError>`. An implementation may therefore try the current payload
shape, fall back to an older one, and construct the current variant from it —
entirely inside the typed layer, with no port change, no backfill, and no adapter
that knows anything. The upcast is a `match` inside somebody's `decode`, which is
the same place the domain already lives.

**The falsifier, stated so this answer can be shown wrong.** Decode-time tolerance
sees exactly one event's bytes. **If an upcast requires information from outside the
event being decoded** — a field reconstructed from a sibling event, or from a read
model — this answer is wrong. That case is not hypothetical in general; it is simply
not reachable from anything this project builds, and no scenario in
`references/scenarios/` needs it.

**And if it is ever reached, the route is fixed and it is not a hook.** A
cross-event upcast is a **defect entry naming the clause ID, routed to a decision
record** under project AC-012 (`project.md:204-206`) — never a line edit of the
frozen trait, never a proposed amendment to an `ES-*` clause, and never a variant
added to `EventStore` (`AC-A02`). That is the rule, not an exception to it.

---

## Decision 4 — an event with no framing region decodes with the codec in hand

The tag exists so one store can hold two encodings, which makes this a migration
decision by construction — and every event already written carries **no tag at
all**. The rule, in one sentence M3 can implement:

> **An event whose `metadata` carries no framing region decodes with the codec the
> caller is already holding** — `Json` under `commit`, `C` under
> `commit_with::<C>` — **and not `CodecError::UnknownTag`.**

Why not `UnknownTag`. Refusing an untagged event would make the typed layer unable
to read any log written before it existed, including the one
`examples/course-subscriptions` writes today. The only repair would be rewriting
stored events, and **no adapter may rewrite or reinterpret them** (VT-3), while
**no backfill may run through a hook `EventStore` does not have** (`store.rs`, above).
So refusal is not a strict-and-safe choice here; it is an unimplementable one.

Why the codec in hand rather than a hard-coded `Json`. An absent tag means *no
claim was made about this event's encoding*. The caller's own codec is the only
information available, and it is usually right — the common case is `commit`, whose
codec is `Json`. When it is wrong, the failure is a `CodecError::Decode` carrying
the underlying serialiser's error, which is a better diagnostic than a category.

**What this buys `UnknownTag`.** `CodecError::UnknownTag { tag }`
(`_design.md:461-462`) is left with exactly one meaning: *a tag was written, and
this build cannot honour it* — an event written by a build with the `cbor` feature,
read by a build without it. That is a real, reachable failure with an actionable
message, rather than a variant that also fires on every legacy event.

Neither half of this rule needs an adapter to change, and neither needs a hook.

---

## Alternatives rejected

| Rejected | The wrong implementation it admits |
| --- | --- |
| **The tag in `Tags`** | It validates, it is queryable, no adapter changes, the whole gate is green and every round-trip test passes — and re-encoding an event changes which `QueryItem`s it satisfies, so a consistency boundary changes shape as a consequence of a storage-format migration. It also obliges every adapter to store, index, match on and budget it, which is *no adapter may need to understand the tag* violated by the population rather than by any one member |
| **"Either home is fine"** | Two admissible constructions of the same value, which is the defect the contract crate names about `ProjectionId`: *"two constructors enforcing different rules is the defect that makes an invalid value reachable through the weaker one"* (`crates/happenstance-core/src/projection.rs:152-154`). Two homes means a reader must handle both, forever, and the weaker one wins by being easier |
| **`happenstance` owning the whole of `Event::metadata`** | Removes the field an application wants for causation and correlation, with no replacement anywhere in the contract. It buys a simpler framing rule by taking away a public affordance |
| **A `serde`-encoded framing region** | Unreadable under `--no-default-features`, and a build with only `postcard` could not read a tag written by a build with only `json` — the tag would need a codec to say which codec to use |
| **A version suffix on `EventType`** | `"CourseDefined.v2"` compiles, validates and round-trips, and is invisible to every query already written, because matching is exact equality (`query.rs:113`). The first upcast makes existing decision models read an empty log and their append conditions match nothing |
| **A version suffix *with* a query-naming rule** | Admissible in principle, and rejected on cost: every query must enumerate every live version of every type it selects, forever, and forgetting one is the same silent failure with an extra step. It also duplicates what `decode` already does |
| **"No hook is needed", asserted** | The answer that requires no work and leaves no trace. If it is wrong it is discovered in production by an application that has already built on it — P1's stated fear. Rejected as a *form of answer*, not as a conclusion: the conclusion stands, with the strategy and the falsifier attached |
| **Adding a read-path hook to `EventStore`** | Amends a `[FROZEN]` trait to solve a problem `DomainEvent::decode` already solves, and does it by giving every adapter a surface it must implement in order to serve a concern above the port |
| **`CodecError::UnknownTag` for untagged events** | Makes every event written before the typed layer existed unreadable, with no legal repair: no adapter may rewrite stored events (VT-3) and no backfill may run through a hook that does not exist |
| **Answering this question in ADR-0016's terms** | Puts a second, competing answer to *"how is a payload encoded"* into a corpus that already has one, and the immutability rule makes that expensive to unwind. See the seam below |

**One home, stated once.** The codec tag lives in `Event::metadata`. There is no
second admissible home, no configuration switch, and no "adapters that prefer tags
may…". Two homes would be the `ProjectionId` defect at a larger scale, because the
weaker one is reachable through an ordinary `with_tags` call that nothing refuses.

---

## The seam against ADR-0016, stated so the corpus does not acquire two answers

ADR-0016 settled the **replication wire format** — private to happenstance, no
compatibility obligation to the DCB reference, `Event::data` and `Event::metadata`
as standard base64 in human-readable formats and raw bytes in binary ones
(`.kb/decisions/0016-the-wire-format.md`).

The two decisions never collide, and the sentence that separates them is:

> **ADR-0016 owns how the metadata bytes cross a peer boundary. ADR-0021 owns what
> those bytes mean to the typed layer.**

ADR-0016 moves them and never reads them; this record reads them and never moves
them. A replicated event therefore carries its framing region along untouched, as
opaque bytes, which is precisely what VT-3 requires of a peer and what makes the
tag survive replication without `happenstance-sync` knowing it exists.

One thing that does **not** transfer across the seam is ADR-0016's
`reversibility: high`. That value is justified there by the format being private
and unpublished, with nothing to break. See *Reversibility* below.

`.kb/open-questions/human-readable-payload-encoding-on-a-constrained-peer.md`
(`kb-open-question-human-readable-encoding-limits-001`) is the nearest neighbouring
question and is **not** settled here: its subject is wire-format encoding on a
memory-limited peer, and it is phase 9's. It is cited to mark the seam and left
exactly as it is.

---

## The `serde` boundary, in the right direction

ADR-0003's prohibition attaches to **`happenstance-core`**, whose `serde` feature
covers envelope types only. After ADR-0006 gave the bare name to the typed layer,
`happenstance` is the crate whose entire job is encoding — so `serde` arriving
there is the split working, not a violation, and forbidding it there would forbid
the thing the split exists to allow (`CLAUDE.md`, binding constraint 2, which warns
in terms that this constraint *"says the opposite of what it used to"*).

The line that stays true either way: **payloads remain `Bytes` at the port**, and
`Codec` operates strictly above it. Encoding never routes through
`happenstance-core/serde`. Decision 1's rule 3 is the same principle applied to the
framing region: it is not `serde`-encoded at all, so the contract crate's
`--no-default-features` doc build and the `wasm32` steps stay honest.

---

## Consequences

**M3 inherits three answers and a rule, not a gap.** `codec-and-feature-forwarding`
writes the tag where this record sites it, formats the framing region within the
bounds of Decision 1's rules 2–5, and implements Decision 4's untagged rule rather
than inventing one under time pressure.

**No conformance rule, and the absence is this decision's own content.** The point
of siting the tag where VT-3 keeps it opaque is that **no store can see it**, so no
adapter could fail a rule about it and such a rule would be decorative
(`CLAUDE.md`, *A rule that no adapter can fail is decorative*). A testkit rule that
*could* observe the codec tag would prove the tag is visible below the port — which
falsifies this decision rather than verifying it. The instruments are
`redkiln validate --kb` and M3's per-codec round-trip tests.

**Event types become part of the public commitment surface.** Decision 2 makes an
`EventType` string a name that queries and consistency boundaries are written
against and that therefore cannot be renamed. That is a heavier promise than a
serialisation format, and it is the promise being made deliberately.

**The alpha ships whatever this chooses.** Initiative DoD 9 (*a stranger can install
it*) inherits these answers unchanged, and every event a user writes with
`0.2.0-alpha.1` carries the framing region this decision sites.

### Reversibility: **low**

This is the least reversible decision in the project and it looks like the most
reversible, because no signature moves either way — `Codec::TAG` is a
`&'static str` under both homes (`_design.md:441-445`), which is what let the design
proceed without it.

Reversing it after `0.2.0-alpha.1` means **re-siting a tag on events already
written into real stores**, and no adapter may rewrite them (VT-3) and no backfill
may run through a hook `EventStore` does not have. The only legal route is
application-level re-emission, which mints new events with new identities (VT-5:
identity is the store-assigned `(StoreId, SequencePosition)` pair), so the old
events do not go away — they stay, in the old siting, forever.

**Which is why Decision 4 is load-bearing beyond its own question.** An event
without the framing region falls back to the codec in hand, so a future re-siting is
*additive*: a new framing region, with the old one still readable. That fallback is
the mechanism any future supersession of this decision would have to reuse, and it
exists now rather than being invented then.

`0016`'s `high` does not transfer. Its format is private, unpublished and owes
compatibility to nobody. This one is inherited by every store a user writes into.

---

## What this decision does not decide

- **The codec trait, its features, and the framing region's exact bytes.** M3's
  (`codec-and-feature-forwarding`). This record sites the tag and bounds the
  format; it does not write it.
- **Whether a CBOR codec ships.** A new dependency decision with licence, `no_std`
  and wasm consequences of its own (`_decomposition.md:596-605`).
- **ADR-0020's subject matter** — fold/query agreement, `Boundary::query`,
  `DecisionModel::scope`. Staged in the same wave.
- **Anything in `spec/SPECIFICATION.md`.** VT-3 is *obeyed* and cited; no clause is
  amended, no maturity marker moves, and the frozen `EventStore` surface is
  untouched.
- **`kb-open-question-human-readable-encoding-limits-001`**, which is phase 9's and
  is about a different question.

---

## A citation checked and found imprecise

`_design.md`, this story's spec and `_decomposition.md` all describe `EventStore` as
offering *"`read`, `append`, `head` and `read_decision_model`"*. The trait
(`crates/happenstance-core/src/store.rs:93-269`) requires `read` `:119`, `append`
`:213`, `head` `:248` and `contains_event_id` `:268`; `read_decision_model` is a free
function at `:321`. Decision 3's conclusion is unaffected — **neither** spelling of
the surface contains a hook — and the correct enumeration is what this record uses.
It is a **repair** by `.kb/decisions/README.md`'s mechanical test: the set of
implementations admitted is unchanged.

---

## Provenance

Taken at phase 7, **before** the code it governs exists — project AC-016's whole
content — and recorded here in the two forms the corpus keeps on purpose. Where this
record and `spec/SPECIFICATION.md` ever disagree, the specification wins: an ADR is
history, and is never updated to match the code.

---

## Which instrument covers rule 3

Decision 1's rule 3 says the framing region must be readable with no default features
and on `wasm32`. An earlier draft of that rule cited `--no-default-features` and
`wasm32` as gate steps of `cargo xtask ci` and left it there. That is true of the gate
and **false of this crate**, so the instrument is named here instead.

Both `--no-default-features` steps in `cargo xtask ci` are scoped `-p happenstance-core`:
the `wasm32` build at `xtask/src/main.rs:216-223` and the rustdoc build at `:532-544`.
The framing region is written by `happenstance`, one crate above — neither step ever
builds it. What actually covers this crate is `cargo hack check --workspace
--feature-powerset --no-dev-deps` (`:605-615`), which is workspace-wide and so compiles
`happenstance` with the empty feature set, together with the `wasm32` feature powerset
(`:623-652`). Both sit in `OPTIONAL` behind a `cargo hack --version` probe; the tool
resolves on this machine, so they run rather than print `skipped` — but a machine
without `cargo-hack` narrows this coverage to nothing, which is the honest statement of
the guarantee and the reason for naming the step rather than the gate.

The rule itself is unchanged. This is a **repair** by `.kb/decisions/README.md`'s
mechanical test: the set of implementations admitted is identical.

---

## A second citation checked — and found correct

A review pass proposed widening `crates/happenstance-core/src/projection.rs:152-154`,
cited by this record and by ADR-0020 for the two-constructor sentence, to `:152-155` on
the grounds that the quoted clause runs one line further. **It does not.** Verified
against `HEAD` and against ingest wave `a28322b`:

```
152:    /// fallible `parse` beside this constructor would be worse than either
153:    /// choice: two constructors enforcing different rules is the defect that
154:    /// makes an invalid value reachable through the weaker one.
155:    pub fn new(value: impl Into<String>) -> Self {
```

`:152-154` covers the quoted prose exactly; line 155 is the function signature.
Widening the range would pull code into a citation that quotes a doc comment. The
repair these records already performed — from `:47-61`, correct at planning commit
`ae77ac4` and stale thereafter, to `:152-154` — stands as made. Recorded so the finding
is disposed of rather than re-raised and applied.
