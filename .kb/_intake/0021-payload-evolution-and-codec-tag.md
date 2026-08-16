# Staged: ADR-0021 — payload evolution and the codec tag's home

**Staged 2026-08-15** for the next `/redkiln:kb-ingest` wave. **Not an atom.**
Long-form record:
[`references/adr/0021-payload-evolution-and-codec-tag.md`](../../references/adr/0021-payload-evolution-and-codec-tag.md).

Stage in **one** wave with `0020-fold-query-agreement.md`; the wave-id suffix note
and the *do not sweep `README.md` into the glob* note in that document apply here
unchanged and are not repeated.

---

## The op: create one `decision` atom

`.kb/decisions/0021-payload-evolution-and-codec-tag.md`. Unlike its slice-mate this
record **takes** its decisions rather than transcribing a signed-off design row:
`_design.md:674-679` hands all three to it in terms, because the public surface is
invariant under them.

## Proposed frontmatter

The ingest run authors the frontmatter; this is the proposal it authors from.

```yaml
id: kb-decision-0021
title: >-
  The codec tag lives in Event::metadata, event types do not carry versions, and upcasting
  happens at decode
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0021
reversibility: low
phase: 7
supersedes: null
superseded_by: null
summary: >-
  Three answers, none of them taken at the design gate, because the public surface is invariant
  under all three - Codec::TAG is a &'static str either way. The codec tag lives in
  Event::metadata, inside a versioned framing region the typed layer owns and no store parses,
  with everything after it the application's own causation and correlation metadata, carried
  through unmodified. The decision turns on VT-3's second half rather than its first: anything a
  store, a peer, a conformance rule or a query must see belongs in EventType or Tags, and
  nothing below the port needs to see the codec tag because decoding is strictly above it. The
  falsifier is name the adapter change this choice forces, applied to both candidates: metadata
  forces none, while Tags obliges every adapter to store, index, match on and budget the tag,
  spending one of the 64 tags every store must accept on every event forever, and makes
  re-encoding change which QueryItems an event satisfies - so a consistency boundary silently
  changes shape as a consequence of a storage-format migration that was supposed to be
  invisible. EventType carries no version suffix: matching is exact equality, so CourseDefined.v2
  is invisible to every query already written and the first upcast makes existing decision models
  read an empty log with their append conditions matching nothing; an event type is a stable
  identity and the payload is what evolves. No read-path hook is needed, and the strategy that
  earns it is decode-time tolerance, which DomainEvent::decode already permits because it
  receives the event type and the raw bytes; its falsifier is an upcast needing information from
  outside the event being decoded, and if that is ever reached the route is a defect entry with a
  clause ID under AC-012, never a line edit of the frozen EventStore. An event with no framing
  region decodes with the codec in hand - Json under commit, C under commit_with - and not
  UnknownTag, because refusing it would make every event written before the typed layer existed
  unreadable with no legal repair, and because that fallback is the mechanism any future
  re-siting would reuse. UnknownTag is left meaning exactly one thing: a tag was written and this
  build cannot honour it. Rejected: the tag in Tags; either-home-is-fine; happenstance owning the
  whole metadata field; a serde-encoded framing region; a bare version suffix; a version suffix
  with a query-naming rule; an unearned no-hook answer; adding a hook to EventStore; UnknownTag
  for untagged events; answering in ADR-0016's terms. Reversibility is low, not high: reversing
  it after 0.2.0-alpha.1 means re-siting a tag on events already written into real stores, which
  VT-3 forbids any adapter from doing on the user's behalf.
depends_on:
  - kb-decision-0003
  - kb-decision-0006
related:
  - kb-decision-0007
  - kb-decision-0016
  - kb-open-question-human-readable-encoding-limits-001
source_paths:
  - .kb/_intake/0021-payload-evolution-and-codec-tag.md
  - references/adr/0021-payload-evolution-and-codec-tag.md
  - .bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_design.md
  - crates/happenstance-core/src/event.rs
  - crates/happenstance-core/src/store.rs
  - spec/SPECIFICATION.md
last_reviewed: 2026-08-15
```

Every `source_paths` entry resolves as staged.
`kb-open-question-human-readable-encoding-limits-001` is cited as `related` **only**,
to mark a seam: its subject is wire-format encoding on a memory-limited peer, it is
phase 9's, and it is not resolved, edited or deleted by this wave.

## The problem, against this tree

`happenstance-core` carries opaque `Bytes` and has no evolution problem. The typed
layer has one from its first commit: a decoded
`Enrolment::Defined { capacity: u32 }` breaks when a field is added, and a store
holding two encodings at once needs a way to tell them apart **that no adapter is
allowed to understand**. Project AC-005 requires the tag to exist
(`project.md:177-180`); this record decides where it lives.

## VT-3 read in both directions — the atom must carry this, it is what decides

`spec/SPECIFICATION.md:631-633` (`[FROZEN]`) has two halves. The first — *stores,
peers and the contract layer MUST NOT parse `data` or `metadata`* — licences
nothing on its own. The second decides the siting: *anything a store, a peer, a
conformance rule or a query **must** see MUST be carried in `EventType` or `Tags`*.
So the prior question is **does anything below the port need to see the codec
tag?** It does not: decoding is strictly above the port (ADR-0007), and
`QueryItem::matches` looks at `event_type` and `tags` only
(`crates/happenstance-core/src/query.rs:112-115`). Had the answer been yes, VT-3
would have **required** `Tags` and the constraint *no adapter may need to understand
the tag* would already be violated. VT-3's `Rejects:` line — an ingest path writing
origin identity into `metadata` (`:643-646`) — is the mirror case, not a
counter-example: replication identity **is** something a peer must see.

## The claims the atom carries — three decisions and the rule they imply

1. **The codec tag lives in `Event::metadata`** (`with_metadata` at
   `crates/happenstance-core/src/event.rs:379`, read at `:400`), inside a
   **versioned framing region** the typed layer owns. Everything after it is the
   application's, carried through unmodified and never parsed by `happenstance`. The
   framing region must be readable without knowing the payload codec, must add no
   dependency (so it works under `--no-default-features` and on `wasm32`), and must be
   distinguishable from metadata that is entirely the application's. **Its exact bytes
   are M3's**, not this record's.
2. **`EventType` carries no version suffix.** Matching is exact equality
   (`query.rs:113`), so `"CourseDefined.v2"` is invisible to every query already
   written — including `examples/course-subscriptions/src/main.rs:117` — and the first
   upcast makes existing decision models read an empty log: capacity `None`, the
   handler bailing *"course does not exist"* (`main.rs:158-160`), the append condition
   matching nothing. An event type is a **stable identity**; the payload is what
   evolves. A genuinely new fact takes a **new type name**, and every query that must
   see it names it deliberately.
3. **No read-path hook is needed**, earned by naming the strategy: **decode-time
   tolerance**. `DomainEvent::decode(codec, event_type, data) -> Result<Self,
   CodecError>` (`_design.md:396-408`) receives the type and the raw bytes, so an
   older payload shape is handled inside the typed layer with no port change.
   `EventStore` is `[FROZEN]` with four required methods and no hook
   (`crates/happenstance-core/src/store.rs:119`, `:213`, `:248`, `:268`).
4. **The rule this implies, so M3 does not invent one:** *an event whose metadata
   carries no framing region decodes with the codec in hand* — `Json` under `commit`,
   `C` under `commit_with::<C>` — **and not `CodecError::UnknownTag`**. Refusing would
   make every event written before the typed layer existed unreadable, with no legal
   repair: no adapter may rewrite stored events (VT-3) and no backfill may run through
   a hook that does not exist. `UnknownTag` (`_design.md:461-462`) is left meaning
   exactly one thing: *a tag was written and this build cannot honour it.*

## The falsifier, applied mechanically to both homes

*Name the adapter change this choice forces.*

| Obligation | `Event::metadata` | `Tags` |
| --- | --- | --- |
| store it | already stored | stored as a tag |
| index it | **none** | **yes**, in every adapter's tag index |
| match on it | **none** (VT-3) | **yes**, unavoidably — matching reads tags |
| budget it | **none** | **yes** — one of the 64 in `crates/happenstance-core/src/limits.rs:27`, on every event, forever |
| migrate it | **none** | **yes, silently** — re-encoding changes the tag set, which changes which `QueryItem`s the event satisfies, which changes which decision models read it |

`Tags` forces a change in **every** adapter; `metadata` forces none. Not by making
one adapter parse the tag — none would — but by obliging all of them to store, index,
match on and budget it.

## The accepted cost of `Event::metadata`, discharged

`metadata` is the field an application wants for causation and correlation. The
coexistence rule is the two-region split above: `happenstance` reads and writes only
its framing region and carries the rest verbatim. **The price, stated:** an
application reading `event.metadata()` at the *contract* level gets the framing bytes
too and must skip them — one caller shape paying, instead of every adapter.

## Rejected alternatives the atom must name, each with what it admits

- **the tag in `Tags`** — validates, is queryable, changes no adapter, greens the
  whole gate, and lets a re-encoding change a consistency boundary's shape;
- **"either home is fine"** — two admissible constructions of one value, which is
  `crates/happenstance-core/src/projection.rs:152-154`'s defect: *"two constructors
  enforcing different rules is the defect that makes an invalid value reachable
  through the weaker one"*;
- **`happenstance` owning the whole of `metadata`** — takes away the field an
  application wants, with no replacement in the contract;
- **a `serde`-encoded framing region** — unreadable under `--no-default-features`,
  and a `postcard`-only build could not read a `json`-only build's tag;
- **a bare version suffix on `EventType`** — old events unreachable in one commit;
- **a version suffix *with* a query-naming rule** — admissible, rejected on cost:
  every query enumerating every live version forever, duplicating what `decode` does;
- **"no hook is needed", asserted** — rejected as a *form* of answer; the conclusion
  stands only with its strategy and its falsifier attached;
- **adding a hook to `EventStore`** — amends a `[FROZEN]` trait for a concern above
  the port;
- **`UnknownTag` for untagged events** — unreadable legacy logs with no legal repair;
- **answering in ADR-0016's terms** — a second competing answer in a corpus that
  already has one.

## The seam against ADR-0016

**ADR-0016 owns how the metadata bytes cross a peer boundary; ADR-0021 owns what
those bytes mean to the typed layer.** ADR-0016 moves them and never reads them;
this record reads them and never moves them, so a replicated event carries its
framing region along as opaque base64/raw bytes and `happenstance-sync` never learns
it exists. Cited as `related: kb-decision-0016`. **What does not transfer is its
`reversibility: high`**, which rests on that format being private and unpublished.

## `serde`, in the right direction

ADR-0003 constrains **`happenstance-core`**, whose `serde` feature covers envelope
types only; after ADR-0006 `happenstance` is the typed layer whose job is encoding,
so `serde` there is the split working. Payloads stay `Bytes` at the port and
encoding never routes through `happenstance-core/serde` — and the framing region is
not `serde`-encoded at all, which is what keeps the contract crate's
`--no-default-features` doc build and the `wasm32` steps honest.

## Consequences the atom carries

M3 inherits three answers and a rule rather than a gap. **No conformance rule, and
the absence is the decision's own content**: a testkit rule that could observe the
codec tag would prove it is visible below the port, which falsifies the decision
rather than verifying it. Event-type strings become part of the public commitment
surface, because Decision 2 makes them un-renameable. **Reversibility is `low`**:
reversing after `0.2.0-alpha.1` means re-siting a tag on events already in users'
stores, and the only legal route is application-level re-emission, which mints new
identities (VT-5) and leaves the old events in place forever. Claim 4's fallback is
what makes any future re-siting *additive* rather than impossible, which is why it is
part of this decision and not M3's.

## One citation checked and found imprecise

`EventStore` requires `read`, `append`, `head` and `contains_event_id`
(`crates/happenstance-core/src/store.rs:119`, `:213`, `:248`, `:268`);
`read_decision_model` is a **free function** at `:321`, not a trait method, though
`_design.md`, `_decomposition.md` and this story's spec all recite it as one.
Decision 3 is unaffected — neither spelling contains a hook — and the atom uses the
correct enumeration. A **repair**, not an amendment: the admitted-implementation set
is unchanged.

## Open questions

**None is resolved by this atom.**
`kb-open-question-human-readable-encoding-limits-001` is cited as `related` to mark
a seam and is left exactly as it is; it is phase 9's and its subject is wire-format
encoding on a memory-limited peer. Nothing under `.kb/open-questions/` is deleted.

## Map rows

- `.kb/maps/decision-map.md` — one row: ADR-0021, atom link, title, `accepted`,
  phase 7, supersession `—`.
- `.kb/maps/domain-map.md` — an entry under the typed layer, beside ADR-0020's.

## Not proposed

No change under `crates/**`, `examples/**`, `xtask/**` or `spec/SPECIFICATION.md`;
no maturity marker moved; **no amendment to VT-3 or to any `ES-*` clause**; no hook
added to `EventStore`; no supersession of ADR-0016 or of any other atom; no
`reference` atom asserted — whether the long record also warrants one is the wave's
adjudication, and the corpus's precedent (`ADR-0029`) discharges it with
`source_paths` alone.
