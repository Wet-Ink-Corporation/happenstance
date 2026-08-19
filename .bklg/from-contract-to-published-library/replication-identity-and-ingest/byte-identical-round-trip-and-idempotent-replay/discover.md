---
item: HS-S0107
stage: discover
created: 2026-08-12T13:03:28.423Z
updated: 2026-08-12T13:03:28.423Z
template_sig: 86ce4036
rendered_sig: 893c3ab9
---

# Discover — A payload survives the boundary unchanged, and replay changes nothing

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Slice one-line: assert the payload `Bytes` byte-identical at the receiver across a real store boundary and replaying the same batch twice an observable no-op, with the negative control that would fail if any assertion on any path touched `Event::data` or `Event::metadata` | `_storymap.md`, *Slices* table, `wire-message-set-and-round-trip` row 2 | Two assertions and one negative control. The control is not optional decoration — DR-5 names it |
| **Depends on `message-set-on-the-envelope` (HS-S0106)**, which supplies the message set the round trip travels in and the codec whose byte-identity is in question | `_storymap.md`, *Slices* `depends_on` | The codec that could break byte-identity is introduced one story earlier. This story is where it is caught |
| **AC-006** — the payload `Bytes` are asserted byte-identical end to end across a store boundary, replaying the same batch twice is observably a no-op, and ADR-0003's `provisional` marker is lifted against the ADR's own stated lift condition — or the reason it cannot be lifted is recorded | `project.md`, *Acceptance criteria*, AC-006 | This story owns the two assertions; `adr-0003-provisional-lift` (HS-S0108) owns the marker |
| **DoD 4** — *"The proof artefact exists, and it would not exist if the design were wrong: one suite green against three peers, two structurally unlike, with a byte-identical payload round-tripped across a store boundary."* A phase is done when its proof artefact exists, not when the gate is green | `project.md`, *Definition of done* 4; `RUNBOOK.md:4597-4602` | This story is half the proof artefact. `durable-object-and-neon-peers` (HS-S0111) is the other half |
| ADR-0003's own lift condition, in the atom's Status section: *"It lifts at phase 13, when `happenstance-sync` round-trips an event between two stores without deserialising its payload."* | `.kb/decisions/0003-opaque-payloads.md:88-95` | *"That sentence **is** AC-006. Cite it; do not restate it as a new condition"* (`_decomposition.md`) |
| **DR-5** — the suite never decodes `Event::data` or `Event::metadata`; *"A suite that parses a payload would certify a peer that does, which is the exact guarantee ADR-0003 exists to buy"* | `project.md`, DR-5; `RUNBOOK.md:4576-4580`; `spec/SPECIFICATION.md:6153-6171` | The prohibition is on the **suite**, not only on the peer. This is the story that could most easily break it |
| **SY-35 `[FROZEN]`** — anything replication must reason about MUST be in the tags | `spec/SPECIFICATION.md:6868` | The structural reason the suite never needs the payload: nothing it reasons about is in there |
| **SY-11 `[FROZEN]`** — re-delivery of an event the receiver already holds MUST be a no-op: no second copy, no compensation, no error; rule `redelivery_of_an_accepted_group_is_a_no_op` | `spec/SPECIFICATION.md:6135-6152` | The replay half of AC-006 is this clause. Its `Rejects:` is an **inversion**, not a duplicate |
| **SY-12 `[FROZEN]`** — the identity a peer dedupes on MUST be reachable *without decoding* `Event::data` or `Event::metadata`; the rule *"supplies payloads and metadata that are not valid UTF-8 and not valid in any codec; a conformant peer dedupes anyway"* | `spec/SPECIFICATION.md:6153-6172` | The fixture technique is already specified: payloads no codec accepts. Reuse it here |
| **SY-19 `[FROZEN]`** — a replicated event's local position is arrival order, and there is *"no expression relating the two numbers"*; Kestrel Rotor's fourteen events sit at vessel position 38,102 and depot position 3,918,442 | `spec/SPECIFICATION.md:6409-6428` | What survives the boundary is the payload and the `EventId`, never the position. The assertion must respect that |
| **WF-11 `[PROVISIONAL]`** — the inverted `is_human_readable` branch is *"invisible to WF-7's rules"*, passing both round trips while rendering `[de ad be ef]` as `[222,173,190,239]` in JSON and as the ASCII of `"3q2+7w=="` in postcard | `spec/SPECIFICATION.md:2279-2322` | A measured, in-tree example of a codec that is not byte-identical and that every decoded-value comparison calls correct |
| `ReplicatedEvent` carries no position of its own beyond the one inside `EventId`, *"because the receiver assigns that and the sender's copy would be actively misleading"*; and *"The payload inside `event` is never decoded here"* | `crates/happenstance-sync/src/identity.rs:160-180` | The wire type is already shaped for this assertion. The test does not have to work around it |
| `Ingested { appended, skipped, last_local }` — `skipped` exists *"so a runner can tell a healthy overlap from a resume token that is not advancing"* | `crates/happenstance-sync/src/ingest.rs:178-191`; `crates/happenstance-sync/src/peer.rs:258-263` | Replay's observable no-op has a typed witness: the second pass reports `appended: 0` and a non-zero `skipped` |
| The precedent for a conformant variant that refutes an over-specified assertion behaviourally: `GappedPositionStore`, *"the store CF-6 is enforced by: a rule asserting a literal position value passes against `MemoryEventStore` and fails here"* | `crates/happenstance-testkit/tests/mutation_coverage.rs:326-338`; `spec/SPECIFICATION.md:7222-7232` (CF-5) | The negative control DR-5 asks for takes the same shape: a legal peer that a payload-touching assertion would trip over |
| The testing brief's own statement of the control: *"a suite variant (or a targeted test inside `happenstance-sync-testkit`) that would fail if any assertion touched `Event::data` or `Event::metadata` — the same shape as `GappedPositionStore` proving CF-6 by existing"* | `_decomposition.md`, testing brief *Integration* | The control is a **conformant** peer, not a mutant: it must pass everything |
| This is an integration-tier assertion by construction: it needs two real stores and a real hop between them | `_decomposition.md`, testing brief *Integration* | Not a unit test on the codec. Two stores, one peer, one boundary |
| CF-6 / DR-7 — every position assertion anchored on a value the store under test assigned | `spec/SPECIFICATION.md:7233-7249`; `project.md`, DR-7 | Both stores here assign densely from one, so a literal would pass and would be wrong |

## Questions

**Does ingest re-check the writer's asserted append conditions? — Answered
elsewhere, and this story is where the consequence of the *wrong* answer would
have shown up first.** SY-6's second fatal defect is that re-evaluation is not
idempotent: on re-delivery the origin's condition matches the receiver's own
already-accepted copy, returns `ConditionViolated`, and the receiver adjudicates
against the event it just accepted (`spec/SPECIFICATION.md:6008-6014`, E2E-33).
The replay half of this story is exactly the observation that would catch it —
provided the assertion is on **what the receiver holds**, keyed by identity, and
not on a count.

**Hub-and-spoke versus peer-to-peer — not this story's.** One boundary, two
stores, one peer handle (SY-8).

**Byte-identical *where*, exactly? — Deferred to spec, and the choice is
load-bearing.** "At the receiver" could mean the `Bytes` handed to
`IngestStore::ingest`, or the `Bytes` read back out of the receiving store
through `EventStore::read` after the ingest committed. Only the second closes the
loop, because only the second crosses the store's own persistence path — and for
`MemoryEventStore` the two are nearly the same object, which is precisely why the
choice must be made deliberately here rather than discovered when a SQLite-backed
store round-trips through a `BLOB` column.

**How is "an observable no-op" observed? — Deferred to spec, with three
candidates and a bar.** The typed witness (`Ingested { appended: 0, skipped: n }`),
the receiver's log contents keyed by `EventId`, and the receiver's head position
before and after. The bar: whichever is chosen must distinguish a no-op from a
peer that appended a duplicate *and* dropped an unrelated event, which a count
alone cannot.

**Does `Bytes`' `PartialEq` count as a byte comparison? — Answered: yes, and it
is the right one.** `bytes::Bytes` compares as a byte slice, so
`assert_eq!(sent.data, received.data)` is a byte assertion and not a structural
one. What is forbidden is not comparing `Bytes`; it is *parsing* them —
`serde_json::from_slice`, `str::from_utf8`, a domain decode — anywhere on any
path in the suite.

**Does the negative control belong in this story or in HS-S0103? — Deferred to
spec.** The fixture contract that would carry it is HS-S0103's, but the control
only has something to refute once there is a payload assertion to refute, which is
here. What is fixed is that it must be a **conformant variant** — legal, and
passing everything — in the shape CF-5 establishes, and not a mutant declared
against a rule.

## Decision

ADR-0003 made `Event::data` opaque `Bytes` and kept `serde` out of
`happenstance-core`'s default features on one claim: that a peer forwards events
without deserialising them, cannot fail to parse a payload it does not
understand, and cannot corrupt one by re-encoding it. The atom itself records
that the claim *"has not yet been exercised, because no replication code exists
yet"* and names the exercise that would settle it. As of the story before this
one, replication code exists and a message set travels — and the codec that
carries it is capable of being wrong in a way every ordinary round-trip test calls
correct, which is not speculation: WF-11 pins a measured example of an inverted
`is_human_readable` branch that passes both round trips while rendering the wrong
bytes in both formats. This story is the measurement that turns ADR-0003's claim
into evidence: a payload appended to one store, carried across a real boundary,
and asserted **byte-identical** where it lands — not equal as a decoded value, not
equal as a `String`, equal as bytes — plus a replay of the same batch that is
observably a no-op rather than merely arithmetically balanced, plus the negative
control that would fail if any assertion on any path had touched the payload. The
spec stage will cover: where "at the receiver" is measured and why; the payload
fixture, using SY-12's technique of bytes valid in no codec; the replay
observation and what it must distinguish; the no-op witness; the negative
control's shape as a conformant variant; and the evidence line for AC-006 that
`adr-0003-provisional-lift` (HS-S0108) will cite.

## The wrong implementation

**The mutant this story exists to catch: `DecodedValueRoundTrip` — and it is a
mutant of the *test*, not of the peer.** Write the assertion as
`assert_eq!(sent_event, received_event)` on the decoded domain value, or as
`assert_eq!(from_utf8(&sent.data)?, from_utf8(&received.data)?)` because that
produces a readable failure message. It is green against every correct peer, it
is green against a peer whose codec is not byte-identical, and it is green
against `InvertedHumanReadable` — the measured WF-11 mutant that renders
`[de ad be ef]` as `[222,173,190,239]` in JSON and as the ASCII of `"3q2+7w=="`
in postcard while round-tripping perfectly in both
(`spec/SPECIFICATION.md:2315-2322`). It is worse than a wrong assertion: it is a
**wrong assertion that satisfies AC-006's wording**, so the story closes, ADR-0003
loses `provisional` on its strength, and the guarantee the whole crate structure
was built to buy is certified by a test that never looked at a byte. Worse still,
writing it requires *decoding the payload in the suite*, which is the thing DR-5
forbids outright — so the mutant announces itself in the diff if anyone is
looking for it, which is what the negative control is for.

**`PayloadTouchingSuite` — the control that must exist, and it is a conformant
variant rather than a mutant.** A peer that is entirely legal and stores or
forwards a payload that is valid in no codec: not UTF-8, not JSON, not postcard,
high bytes and embedded nulls — the fixture technique SY-12's rule already
prescribes (`spec/SPECIFICATION.md:6157-6161`). It must **pass every rule**. Any
assertion anywhere in the suite that decodes a payload fails against it
immediately and locally, naming itself. This is the same mechanism
`GappedPositionStore` provides for CF-6 — *"a rule asserting a literal position
value passes against `MemoryEventStore` and fails here"*
(`crates/happenstance-testkit/tests/mutation_coverage.rs:330-338`) — and it is
the only enforcement of DR-5 that does not rely on a reviewer noticing a
`from_utf8` in a diff.

**`BalancedReplayPeer` — the replay mutant a count cannot see.** On the second
delivery it appends one duplicate and silently drops one unrelated event that
arrived in the same batch. The receiver's event count after two passes equals the
count after one. `Ingested { appended: 1, skipped: n-1 }` looks like a healthy
partial overlap rather than a defect. Any assertion of the form *"the log is the
same length after the replay"* is green. What has actually happened is that a
durable fact has been lost and a duplicate created, and only an assertion over
the **set of `EventId`s the receiver holds** distinguishes the two. Its sharper
sibling is SY-11's own named target, the *inversion*: re-delivery causes the
receiver to supersede the event it previously accepted, which leaves the count
right, the identities right, and the decision wrong — *"A duplicate is visible;
an inversion looks like a decision"* (`spec/SPECIFICATION.md:6144-6152`). Both
belong in `crates/happenstance-sync-testkit/tests/`, declared against
`redelivery_of_an_accepted_group_is_a_no_op`, with `expect` pins naming the exact
assertion each should trip.

## Gate: Discover

Tick each box once the sections above satisfy it. `redkiln advance` reads these
against `.redkiln/templates/gates/discover.md` — where each box's rationale is
written — and will not leave `discover` until every one is ticked. Keep each box on
one line; the parser matches line by line.

- [x] The problem is framed in one paragraph.
- [x] Prior art and constraints are recorded in the signal ledger.
- [x] Open questions are either answered or explicitly deferred.
- [x] The next stage (spec) has a clear starting point.
- [x] The wrong implementation this work rejects is named.
- [x] No conformance rule added here asserts a literal position value.
- [x] Any `[FROZEN]` clause this touches is changed by a new ADR, written first.
- [x] If a rule here seems wrong, it is fixed and the reason given in the same change.

**Box 6, and the temptation here is specific.** The replay assertion wants to say
"the receiver's head did not move", and the cheapest spelling of that is a
literal. Both stores in this story are `MemoryEventStore`, which assigns densely
from one, so `assert_eq!(head, SequencePosition::new(3))` would be green and would
convert VT-11's `MAY` into a `MUST` (CF-6). Every position here is read back from
the store that assigned it, compared to a value read from the same store before
the replay. Separately and more importantly for this project: the *origin's*
position travels inside the `EventId` and is never asserted as a number — SY-19
is explicit that no expression relates two peers' position numbers
(`spec/SPECIFICATION.md:6421-6428`), so an assertion that compared them would be
wrong even if both were read from a store.

**Box 7.** This story edits no `[FROZEN]` clause. It exercises SY-11, SY-12,
SY-19 and SY-35 and is the first evidence for `[PROVISIONAL]` WF-11 against a real
boundary. It changes no decision atom either: `.kb/decisions/0003-opaque-payloads.md`
is `status: accepted` and therefore immutable, so the lift is a **new** atom
authored through `/redkiln:kb-ingest` in HS-S0108 and this story's diff must leave
that file untouched (`_decomposition.md`, *Tension 5*; AC-A09). If the round trip
cannot be made byte-identical, the output is a recorded reason — AC-006's second
arm — and not a weaker assertion.
