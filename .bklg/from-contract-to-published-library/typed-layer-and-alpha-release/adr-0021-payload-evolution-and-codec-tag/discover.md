---
item: HS-S0019
stage: discover
created: 2026-08-12T13:01:39.260Z
updated: 2026-08-12T13:01:39.260Z
template_sig: 86ce4036
rendered_sig: ed07a6c1
---

# Discover — ADR-0021 — payload evolution and the codec tag's home

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| The slice: stage into `.kb/_intake/` the payload-evolution decision — whether `EventType` carries a version suffix, whether an upcaster needs a read-path hook `EventStore` does not have, and which of the two admissible homes the codec tag takes, under the constraint that **no adapter may need to understand it** — ingested in the same wave as ADR-0020 | `_storymap.md:50` (M1 row) | A record, not code. Three questions, and the third has exactly two admissible answers |
| AC-005 — `Codec` ships JSON by default with CBOR/postcard behind features, an event carries a codec tag, **and ADR-0021 states** whether `EventType` carries a version suffix and whether an upcaster needs a read-path hook | `project.md:178-180` | The version-suffix and upcaster halves are this record's alone; the feature/codec half is `codec-and-feature-forwarding`'s |
| AC-016 — both ADRs are atoms naming the alternatives that lost, `redkiln validate --kb` clean | `project.md:218-220` | Same ingest wave as ADR-0020, suffixed so it does not overwrite the first wave's audit trail |
| `depends_on`: none | `_storymap.md:50`; manifest `dependsOn: []` | M1 has no in-project input. `codec-and-feature-forwarding` (M3) consumes this record |
| The two admissible homes, and what each costs: **`Event::metadata`** (`crates/happenstance-core/src/event.rs:379`, read as `Option<&Bytes>`) is opaque to every store, which is what VT-3 requires — *"the contract layer, a store adapter, and a peer MUST NOT parse `data` or `metadata`"* (`spec/SPECIFICATION.md:629-637`) — but it is the same field an application wants for causation/correlation. **`Tags`** is queryable and validated but joins the DCB matching surface and consumes tag budget | `_decomposition.md:548-570` (architecture brief, *The codec tag has two possible homes*) | Both are legitimate; the choice is a cost comparison, and the record must state which cost was accepted |
| The constraint that binds whichever wins: **no adapter may need to understand the tag.** A design in which a store must read the tag to serve a read puts domain knowledge into the adapter population — ADR-0003's own reasoning, and the ground on which ADR-0007 rejected a decoding projection store | `_decomposition.md:565-570`; `_design.md:674-679` | This is the falsifier the record is written against |
| The public API surface is **invariant** under the choice — `Codec::TAG` is a `&'static str` either way and no signature changes — so the design did not wait on it and neither does M3's shape | `_design.md:443-445`, `_design.md:674-679` | The record can be written and ingested before any codec code exists, which is what AC-016 requires |
| `EventType::from_static` is `const` and refuses an empty or over-long value at compile time; `EVENT_TYPES` is a `const` list | `crates/happenstance-core/src/event.rs:95-115` | A version suffix would be a *string* convention inside a const list, invisible to every existing query. Consequence to state, not to assume |
| `EventStore` is `[FROZEN]` and offers `read`, `append`, `head` and `read_decision_model` — there is **no** read-path hook of any kind | `crates/happenstance-core/src/store.rs:110-159`; `project.md:132-134` | If evolution genuinely needs an upcaster hook, that is a contract defect to record under AC-012 and route to a decision record, never a line edit |
| `serde` arriving in `happenstance` is the split working, not a violation: ADR-0003 constrains `happenstance-core`, whose `serde` feature covers envelope types only | `_grounding.md:38-44`; `_decomposition.md:588-595` | The record must read the crate name carefully or it will state the constraint backwards |
| Atoms are authored by `/redkiln:kb-ingest` from `.kb/_intake/`, never by hand; an accepted atom is immutable | `_decomposition.md:643-656`; `_grounding.md:76-80` | The handoff is human-invoked and is planned as such |

## Questions

**Answered here.**

- *Does this record wait on the codec code?* No, and it must not. The signatures in
  `_design.md:439-452` are invariant under the tag's home, and AC-016 requires the record
  to precede the code it governs (`project.md:218-220`).
- *Is `serde` in `happenstance` in tension with ADR-0003?* No. ADR-0003 attaches to
  `happenstance-core`; after ADR-0006 this crate is the typed layer whose entire job is
  encoding (`_grounding.md:38-44`). Payloads stay `Bytes` at the port and `Codec` operates
  strictly above it.

**Deferred to `spec`, and it is the point of the story.**

- *Which home the codec tag takes.* Both are admissible and the record decides, with the
  cost it accepted stated. Discovery's contribution is the constraint the answer is checked
  against — no adapter may need to understand the tag — and the observation that the choice
  is invisible to the public surface, so it can be taken on its own merits rather than on
  ergonomics.
- *Whether `EventType` carries a version suffix.* Open. The consequence discovery records
  is concrete and must be argued in the spec: `EVENT_TYPES` is a `const` list of exact
  strings and every existing query names exact strings, so a `CourseDefined.v2` is a
  **different event type** to every query already written.
- *Whether an upcaster needs a read-path hook `EventStore` does not have.* Open, and it has
  a routing consequence rather than a design one: `EventStore` is `[FROZEN]`
  (`crates/happenstance-core/src/store.rs:110-159`), so "yes" is not a licence to add one —
  it is a defect entry with a clause ID, routed under AC-012.

**Not blocked.** Neither `trybuild` nor `MemoryProjectionStore` is an input to this story.

## Decision

The problem this slice solves is that a typed layer which encodes payloads has, from its
first commit, an evolution problem the contract crate deliberately does not have: opaque
`Bytes` never need to be reinterpreted, but a decoded `Enrolment::Defined { capacity: u32 }`
does the moment a field is added, and a store that holds two encodings at once needs a way
to tell them apart that no adapter is allowed to understand. ADR-0021 is the record that
settles three coupled answers before any codec code exists — where the codec tag lives
(`Event::metadata` or `Tags`, with the accepted cost stated), whether `EventType` carries a
version suffix, and whether an upcaster needs a read-path hook the frozen `EventStore` does
not offer. The spec for this story covers the intake document's content: each of the three
questions with its alternatives and the reason the loser lost; the invariant that binds
whichever tag home wins — *no adapter may need to understand the tag* — stated as the
record's own falsifier; the observation that the public surface is invariant under the
choice, so it is not being taken for ergonomic reasons; and, if the upcaster answer is
"yes", a defect entry naming the frozen clause and routing it to a decision record rather
than proposing an amendment. It also covers the handoff: the file lands in `.kb/_intake/`
for the same wave as ADR-0020, with its wave id suffixed so it does not overwrite the first
wave's audit trail.

## The wrong implementation

**The mutant: ADR-0021 sites the codec tag in `Tags`.**

It reads well and it passes everything. `Tags::from_pairs([("codec", "json")])` validates
(`crates/happenstance-core/src/tag.rs:304-310`), the tag is queryable, no adapter changes,
every conformance rule passes untouched, `cargo xtask ci` is green, `cargo xtask spec-trace`
is unaffected, and per-codec round-trip tests all pass. The record is accepted and
immutable.

It is wrong because a tag is not metadata — it is **part of the DCB matching surface**. Two
consequences, both silent:

1. *A re-encoding changes which queries select the event.* The entire purpose of the tag is
   to let one store hold more than one encoding so a migration is possible. Re-encoding an
   event changes its tag set, which changes the `QueryItem`s it satisfies, which changes
   which decision models read it. A consistency boundary silently changes shape as a
   consequence of a storage-format migration that was supposed to be invisible.
2. *Every adapter's tag index now indexes it.* That is precisely the constraint the record
   is written against — *no adapter may need to understand the tag*
   (`_decomposition.md:565-570`) — violated not by an adapter parsing it, but by every
   adapter being obliged to store, index and match on it. It also consumes tag budget
   (`crates/happenstance-core/src/limits.rs`) for every event ever written.

Nothing mechanical rejects this. The instrument that does is the record's own falsifier,
stated as a check rather than a preference: *name the adapter change this choice forces.*
`Tags` forces one in every adapter; `Event::metadata` forces none, because VT-3 already
forbids a store from parsing it (`spec/SPECIFICATION.md:629-637`).

**A second mutant: the record answers "yes, `EventType` carries a version suffix" and stops
there.** `CourseDefined.v2` is a valid `EventType` — `from_static` accepts it
(`crates/happenstance-core/src/event.rs:95-115`) — so it compiles, round-trips and is
indistinguishable from a correct decision at review time. And every `Query` already written
names `CourseDefined` exactly, including the one at
`examples/course-subscriptions/src/main.rs:117`, so the first upcast makes every existing
decision model read an empty log: capacity resolves to `None`, the handler bails with
*"course does not exist"*, and the append condition that was supposed to protect the
boundary now matches nothing at all. A version suffix without a matching rule for how
queries name versions is not an evolution strategy; it is a way to make old events
unreachable in one commit.

**A third mutant, particular to this story's third question: the record answers "no
read-path hook is needed" without checking.** `EventStore` has no such hook and is
`[FROZEN]`, so "no" is the answer that requires no work and leaves no trace — and if it is
wrong it is discovered by an application in production, which is P1's stated fear
(`_decomposition.md:45-49`). The honest form of "no" names the upcasting strategy that makes
it true (decode-time tolerance in `DomainEvent::decode`, which the signature already
permits — `_design.md:404-408`); the honest form of "yes" is a defect entry with a clause ID
under AC-012, not an amendment.

## Gate: Discover

Tick each box once the sections above satisfy it. `redkiln advance` reads these
against `.redkiln/templates/gates/discover.md` — where each box's rationale is
written — and will not leave `discover` until every one is ticked. Keep each box on
one line; the parser matches line by line.

The two judgement boxes. **Literal positions:** this story adds no conformance rule and no
test; it produces an intake document. Vacuously true, ticked on that basis. **Frozen
clauses:** the record touches two — VT-3, whose *"a store adapter MUST NOT parse `data` or
`metadata`"* is the constraint the tag-home choice is checked against, and `EventStore`'s
`[FROZEN]` read surface, which the upcaster question runs into. Neither is amended: VT-3 is
*obeyed*, and if the upcaster answer turns out to be "a hook is needed", the record produces
a defect entry with a clause ID routed to a decision record, which is exactly this box's
rule rather than an exception to it.

- [x] The problem is framed in one paragraph.
- [x] Prior art and constraints are recorded in the signal ledger.
- [x] Open questions are either answered or explicitly deferred.
- [x] The next stage (spec) has a clear starting point.
- [x] The wrong implementation this work rejects is named.
- [x] No conformance rule added here asserts a literal position value.
- [x] Any `[FROZEN]` clause this touches is changed by a new ADR, written first.
- [x] If a rule here seems wrong, it is fixed and the reason given in the same change.
