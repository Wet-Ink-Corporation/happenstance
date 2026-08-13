---
item: HS-S0106
stage: spec
created: 2026-08-12T13:47:46.126Z
updated: 2026-08-12T13:47:46.126Z
template_sig: 87bbf1d0
rendered_sig: e161fb9a
---

# Spec — The message set instantiates Envelope<T>

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` |
| Project | `.bklg/from-contract-to-published-library/replication-identity-and-ingest/project.md` |
| This spec | `.bklg/from-contract-to-published-library/replication-identity-and-ingest/message-set-on-the-envelope/spec.md` |
| This story's discover (answers carried forward, not re-litigated) | `.bklg/from-contract-to-published-library/replication-identity-and-ingest/message-set-on-the-envelope/discover.md` |
| Key briefs — architecture §7 *The wire mounts inside `Envelope<T>`, never around it*, AC-A08; testing *The test mix*, Unit tier | `.bklg/from-contract-to-published-library/replication-identity-and-ingest/_decomposition.md` |
| Story map (this row, its slice, its merge position) | `.bklg/from-contract-to-published-library/replication-identity-and-ingest/_storymap.md` |
| Signed-off design | `.bklg/from-contract-to-published-library/replication-identity-and-ingest/_design.md` — **no user-facing surface**, approved 2026-08-12. This story renders none and claims no `## Items` id. |
| Grounding (atoms + code facts, pre-verified) | `.bklg/from-contract-to-published-library/replication-identity-and-ingest/_grounding.md` |
| Roadmap pointer | `RUNBOOK.md:4586-4587` — phase 13's work item *"Envelope types on phase 5's tested wire format, with the format version first."* |

## One-line PR slice

Land the replication message set as new public types that **instantiate** `Envelope<T>` beside the
envelope rather than an enum wrapped around it, with every derive on `PushBatch`, `EventGroup` and
`ReplicatedEvent` authorised by name in ADR-0027, `FORMAT_VERSION`'s disposition executed from the
same atom, and the refusal of an unknown version re-proved against a *real* message — plus the wire
tests and the compiled wrong implementations that make each of those falsifiable in both supported
formats.

## Executive summary

`crates/happenstance-sync/src/wire.rs` ships exactly four things today —
`FORMAT_VERSION`, `Envelope<T>`, its hand-written `Deserialize`, and `WireError` — and says in its
own module documentation why there is no fifth: *"`Envelope` is generic in `T` and is **not** an
enum of message kinds… the message set is exactly what phase 13 has not designed"*
(`crates/happenstance-sync/src/wire.rs:1-16`). This PR lands the fifth thing. It adds the message
types, turns on the derives that make `PushBatch`, `EventGroup` and `ReplicatedEvent` encodable
(`crates/happenstance-sync/src/lib.rs:86-90` records that those derives were withdrawn and never
restored), and settles what `FORMAT_VERSION = 1` is version 1 *of*.

The delta is small in lines and large in commitment: **an encoding is a promise**, and this is the
commit that makes it. So the PR is weighted toward negative controls rather than toward types. Three
wrong implementations are compiled into the wire tests and each is invisible to the assertion a
first implementation would reach for — a derived `Deserialize` with a post-hoc version check, which
produces byte-identical framing; a message-set-level payload codec that inverts
`is_human_readable`, which round-trips perfectly in both formats; and a flattened push encoding,
which loses SY-30's group boundaries and still compares equal on decoded values.

Not restated here: what a peer is, what ingest promises, the merge rule, or whether replication is
whole-log or scoped. Those are ADR-0026's and ADR-0027's, they are merged before this story opens a
`.rs` file, and this story executes them.

## Context pack

The decisions this story must honor, stated as decisions. Everything deeper is a signposted anchor.

**The message set goes *beside* the envelope, never *inside* it.** `Envelope<T>` stays generic; each
message is a distinct type instantiated into it at the call site (`Envelope<PushMessage>`, not
`Envelope<Message>` where `Message` is an enum of kinds). The reason is stated at the module and it
is a deployment consequence, not a taste: enumerating the kinds inside the versioned type makes
*adding a message kind* a shape change to the envelope, so every new message forces a
`FORMAT_VERSION` bump, and every peer that has not been redeployed refuses every message from a peer
that has — total mutual unreachability, one release later
(`crates/happenstance-sync/src/wire.rs:8-16`; architecture brief §7; AC-A08). `MessageEnum` is the
named wrong implementation and it passes every test at this story's grain.

**Every derive is authorised by ADR-0027 *by name*, and a derive not named there does not land.**
`PushBatch`, `EventGroup` and `ReplicatedEvent` carry no derives today; they carried them briefly,
they were withdrawn, and phase 5 did not restore them on the explicit ground that *"a `#[derive]` on
a public message type **is** a wire format… what travels is phase 13's"*
(`crates/happenstance-sync/src/lib.rs:86-90`). This story is the phase-13 execution of that
sentence, not a second deliberation of it. If ADR-0027 names no derive for a type, that type stays
underived and the message set is built without it.

**WF-8 is `[FROZEN]` and its obligation is discharged differently in each format.** Every message
carries `format_version`, and a receiver MUST read and check it **before any part of the message is
decoded**; a receiver that does not implement a version MUST refuse the whole message and MUST NOT
attempt a partial decode (`spec/SPECIFICATION.md:2168-2216`). In a positional format the version
being declared first is what makes `postcard::take_from_bytes::<u16>` yield it; in a self-describing
format key order carries no meaning — measured, a derived `Deserialize` accepts
`{"message":null,"format_version":999}` and returns `Ok` — so there the obligation is the explicit
check in the hand-written `Deserialize`. **Do not replace that impl with a derive.** The derive has
no interception point: it reads every field into a local `Option` and constructs afterwards
(`crates/happenstance-sync/src/wire.rs:219-238`).

**The oracle for WF-8 is a witness, not a byte layout, and the specification already designed it.**
Measured: a derive plus a post-hoc check returns the *same* `Err` in both formats and produces
*byte-identical* postcard framing — `[01 07]` at version 1, `[80 03 07]` at 384, the same
`take_from_bytes::<u16>` value and the same `[07]` remainder. Neither the error nor the buffer
separates the forbidden implementation from the required one, so a rule asserting on either is
decorative. The one separating observation is whether `T::deserialize` ran at all: decode an
`Envelope<Witness>` at an unknown version where `Witness` records that it ran, and assert the count
is **zero**; the derive records **one** (`spec/SPECIFICATION.md:2189-2203`;
`crates/happenstance-sync/tests/wire.rs:1-34`). This story re-proves that property against a *real
message-set type*, because the existing tests prove it about `String`.

**The payload is carried, never re-encoded.** `Event::data` and `Event::metadata` are `Bytes`, and
`happenstance-core` already owns their encoding — base64 in human-readable formats, raw bytes
otherwise, with the two rules green today at `crates/happenstance-core/tests/wire.rs:1060` and
`:1114` (WF-11, `spec/SPECIFICATION.md:2279-2322`). The message set therefore adds **no second
payload codec**: it makes the payload reachable without being reachable *through*. A message type
with a hand-written `Serialize` that re-wraps the bytes, or a helper that hexes them "for
readability", is `InvertedHumanReadable` in a new costume — it round-trips perfectly in both formats
and every assertion on a *decoded value* is green while the bytes on the wire are wrong. This is the
trap `byte-identical-round-trip-and-idempotent-replay` (HS-S0107) inherits, so the control belongs
here, where the codec is introduced.

**Group boundaries are on the wire and explicit, and no encoding may flatten them.** SY-30: a peer's
unit of work is `[(Option<AppendCondition>, Vec<Event>)]`, not a flat event list, and a receiver that
re-infers the decomposition would publish a state the origin never had
(`spec/SPECIFICATION.md:6700-6712`; `crates/happenstance-sync/src/peer.rs:183-196`). The wrong
implementation is `FlattenedPushEncoding` — flatten `PushBatch` to a list of `ReplicatedEvent` on
the grounds that the receiver can regroup by guard. Byte counts go down, round trips compare equal,
and the structure the receiver was supposed to be *told* is gone.

**`after` is always present on the wire, and no derive may elide it.** `Guard` is `#[non_exhaustive]`
with **public** fields precisely so a replication hub can *read* `after` on a peer-supplied condition
in order to refuse it, while being unable to fabricate one
(`crates/happenstance-core/src/append.rs:120-140`; `spec/SPECIFICATION.md:6020-6029`). WF-2 forbids
`skip_serializing_if` on any wire struct (`spec/SPECIFICATION.md:1943`), and WF-4 is the other half
— *a policy can only refuse what it can see*. A `skip_serializing_if` on `after` would make SY-6's
`wire_condition_with_after_is_refused` unfalsifiable by deleting the evidence it inspects.

**Messages are role-blind.** SY-9 is `[FROZEN]`: hub-ness is an *edge* property, and one adapter type
serves both roles at once (`spec/SPECIFICATION.md:6090-6106`). A message that names a role puts the
topology into the wire format and makes `hub-and-spoke-and-peer-to-peer-topologies` unimplementable
without a format break.

**`WireError` stays a decoding failure, and `SyncError` is not extended here.** A version refusal
happens before any exchange semantics apply; folding it into the runner's enum *"would take a
phase-13 design decision inside an encoding change"* (`crates/happenstance-sync/src/lib.rs:90-96`;
`crates/happenstance-sync/src/wire.rs:167-217`). The `SyncError` extension is ADR-0026's and lands
elsewhere. `WireError::UNSUPPORTED_FORMAT_VERSION` stays the public marker text, because it is what
keeps *"refused, park this"* distinguishable from *"malformed, drop this"* across a `serde` boundary
that erases types.

**`FORMAT_VERSION`'s disposition is executed, not decided.** The constant bumps on a **shape** change
to a wire type and MUST NOT bump on a capacity bound — conflating the two makes every peer in a
heterogeneous deployment unreachable the moment one of them raises a limit
(`crates/happenstance-sync/src/wire.rs:46-65`; WF-9, `spec/SPECIFICATION.md:2219-2248`). Whether
landing a message set is itself such a change, and whether the version is per-message or negotiated
once per connection, are ADR-0027's answers; this story writes whichever landed into the constant's
own documentation so the next reader does not have to re-derive it. Nothing is published, so the
cost of either answer is zero today and non-zero after HS-P0016 — which is the argument for making
it explicit now.

**The rule-placement split is already decided and must not be blurred.** `RULE_FILES`
(`xtask/src/spec_trace.rs:85-89`) is the set every rule in which must be claimed by a clause,
because a conformance rule is something an *adapter* must pass. `WIRE_TESTS`
(`xtask/src/spec_trace.rs:91-104`) is deliberately **outside** it, and resolution is one-way: a
clause may name a wire test, and a wire test need not be named by a clause. A claim about *how bytes
encode* is a wire test; a claim about *what a peer does* is a testkit rule. This story lands wire
tests only.

**Mount and slice.** This is the first of three stories in `wire-message-set-and-round-trip`,
implemented in one context. It mounts into the existing `wire` module, reached through
`crates/happenstance-sync/src/lib.rs:147`'s `pub mod wire;` and the crate's public re-export block at
`:154-160` — not a new module, not a parallel path. `byte-identical-round-trip-and-idempotent-replay`
(HS-S0107) consumes what this PR encodes; `adr-0003-provisional-lift` (HS-S0108) cites that round
trip as evidence.

**Who observes this.** There is no screen. The persona is the **peer adapter author** of the project's
backbone row A4 — *"Speak the wire, and keep the payload opaque"* — whose observable is a message
vocabulary they can implement `SyncPeer` against without inventing an encoding
(`_storymap.md`, **Backbone**). The **KB reader** observes it second-hand, when
`open-questions-resolved-and-indexed` (HS-S0100) records that
`.kb/open-questions/sync-message-set-and-format-version.md` is answered by something that now exists
in code.

## Integration contract

- **Archetype**: `capability`.
- **Slice / milestone**: `wire-message-set-and-round-trip`. Slice-mates, implemented in one context
  and mounted as one integrated surface: `byte-identical-round-trip-and-idempotent-replay`
  (HS-S0107, depends on this story) and `adr-0003-provisional-lift` (HS-S0108, last, because it
  cites evidence that does not exist until HS-S0107 merges). Merge order is `_storymap.md`,
  **Merge order** item 4.
- **Mount point**: `crates/happenstance-sync/src/wire.rs` — the existing `wire` module, beside
  `Envelope<T>` (`:103-143`) and its hand-written `Deserialize` (`:234-238`). It is reachable the
  moment it lands, through `crates/happenstance-sync/src/lib.rs:147` (`pub mod wire;`) and the
  public re-export block at `:154-160`, which is where any message type ADR-0027 names as part of
  the crate's public vocabulary is re-exported alongside `PushBatch`, `EventGroup` and
  `ReplicatedEvent`. No new module, no `mod` left undeclared, nothing reachable only from a test.
- **Wires into**:
  - `crates/happenstance-sync/src/peer.rs:183-196` (`PushBatch`), `:236-252` (`EventGroup` and its
    `guard`), `:254-278` (`Ack`), `:166-190` (`Pulled`), `:280-338` (`PeerLimits`) — the candidate
    vocabulary the data flow already names (`_decomposition.md`, *Data flow*). Which of these is a
    *message* and which is a return value is ADR-0027's; this story instantiates what that atom
    named and invents nothing it did not.
  - `crates/happenstance-sync/src/identity.rs:171-199` — `ReplicatedEvent`, which already carries
    `EventId`, `RecordedAt` and the opaque `Event` and is what makes the payload reachable without
    being reachable through.
  - `crates/happenstance-core` `serde` feature — `Event`, `AppendCondition`, `Guard`, `Query`,
    `SequencePosition` and the payload codec, consumed as-is
    (`crates/happenstance-sync/Cargo.toml:14-24`, which takes `happenstance-core` with
    `features = ["std", "serde"]`). No `serde` attribute in this crate overrides a core encoding.
  - `serde_json` and `postcard` as dev-dependencies (`crates/happenstance-sync/Cargo.toml:26-35`) —
    the two formats WF-8's obligation is discharged differently in, and therefore the two every new
    wire test runs in.
  - `crates/happenstance-sync/tests/wire.rs` — extended, not replaced. Its `mod wire { … }` wrapper
    is load-bearing: a rule name `cargo test --list` cannot print is a citation that resolves to
    nothing (`:25-33`).
- **Renders surfaces**: **none.** `_design.md` records no user-facing surface for this project and
  that determination is what was signed off; there are no `## Items` ids to claim. The public API
  obligation it points at — rustdoc with a compiled example on every new public item — is carried
  here as acceptance criteria rather than as a surface.
- **Conformance rule(s)**: **none added to `RULE_FILES`, deliberately.** Everything this story
  proves is a claim about how bytes encode, so it lands in `crates/happenstance-sync/tests/wire.rs`,
  which is in `WIRE_TESTS` and deliberately outside `RULE_FILES`
  (`xtask/src/spec_trace.rs:85-104`). The peer-observable rule SY-30 names —
  `push_envelope_preserves_group_boundaries (new, happenstance-sync-testkit)`,
  `spec/SPECIFICATION.md:6710-6711` — is a claim about what a *peer* does and is **not** this
  story's; folding it in here would re-open a distinction ADR-0016 §15 already settled. This story
  is nevertheless observed by the existing WF-8 rules
  (`wire::rejects_an_unknown_format_version`, `wire::version_is_readable_before_the_message`) and by
  WF-11's two core rules (`crates/happenstance-core/tests/wire.rs:1060`, `:1114`), all four of which
  must stay green over the new types.
- **Clause(s)**: **discharged, none amended.** WF-8 `[FROZEN]` (the version check now guards a real
  message set), WF-9 `[FROZEN]` (the message set carries enough for a capacity refusal to stay
  reportable without a version bump), WF-2 and WF-4 (no `skip_serializing_if`; `after` always
  present), WF-11 `[PROVISIONAL]` (tested against a real message set for the first time), SY-30
  `[PROVISIONAL]` (the encoding half), SY-9 `[FROZEN]` (messages stay role-blind). No clause text is
  edited and no `(new)` marker is dropped — the `Rejects:`-symbol audit and the marker sweep are
  `frozen-clause-repairs` (HS-S0112)'s. If WF-11's provisional marker turns out to be falsified by
  what the message set needs, that is a recorded finding routed to
  `clause-arithmetic-and-deferral-renewals` (HS-S0113), not a clause edited in passing.
- **Advances DoD scenario**: initiative **DoD 14** — *"Replication has an answer on disk"*
  (`initiative.md`, **Definition of Done** 14). This story is the code half that makes ADR-0027's
  answer about the message set true rather than merely written, and it is the prerequisite for the
  project's own DoD 4 proof artefact (a byte-identical payload round-tripped across a store
  boundary), which HS-S0107 lands. It does not by itself turn either green, and says so.

## PR boundary

**In this PR**

- The message types ADR-0027 names, declared beside `Envelope<T>` and instantiated into it at the
  call site; their `pub` / `#[non_exhaustive]` shape; their re-export.
- The derives ADR-0027 authorised on `PushBatch`, `EventGroup` and `ReplicatedEvent`, and nothing
  else.
- `FORMAT_VERSION`'s disposition written into the constant's own documentation — per-message or
  negotiated, and whether the constant moves — with the reasoning, not just the outcome.
- New wire tests in `crates/happenstance-sync/tests/wire.rs`: the WF-8 witness re-proved over a real
  message, the payload-encoding assertions over a real message, and the group-boundary encoding
  assertion.
- The three compiled wrong implementations — `DerivedEnvelopeDeserialize`, the message-set-level
  inverted/re-wrapping payload codec, and `FlattenedPushEncoding` — each locally declared in the
  test file, in the same shape the existing derived-envelope control already uses.
- `CHANGELOG.md` entries naming the defect each new wire test detects.
- Deleting the crate prose that this PR makes untrue — but only after moving each *finding* it holds
  into the ADR that consumed it (`_decomposition.md`, architecture brief **Notes**, *One thing to
  carry forward that no AC captures*).
- Mounting is *in* scope by definition: the types land in the module already declared at
  `crates/happenstance-sync/src/lib.rs:147` and re-exported at `:154-160`. Touching those wiring
  lines to mount this slice is not scope drift.

**Explicitly not in this PR**

- The byte-identity and idempotent-replay assertions across a real store boundary, and the DR-5
  negative control for them — `byte-identical-round-trip-and-idempotent-replay` (HS-S0107).
- ADR-0003's `provisional` lift — `adr-0003-provisional-lift` (HS-S0108), through
  `/redkiln:kb-ingest` only.
- Authoring or editing **any** `.kb/` atom, including ADR-0027 itself and the open-question
  resolution this story's outcome makes true — `adr-0027-merge-compensation-and-message-set`
  (HS-S0099) and `open-questions-resolved-and-indexed` (HS-S0100). Hand-authoring an atom was
  reverted once already (`0269720`).
- Any edit to `spec/SPECIFICATION.md`, including dropping a `(new)` marker —
  `frozen-clause-repairs` (HS-S0112).
- Extending `SyncError`, or folding `WireError` into it — ADR-0026's, elsewhere.
- Any testkit rule, mutant-registry entry, or `RULE_FILES` / `TESTKIT_SRC` change —
  `sync-testkit-crate-and-rule-registry` (HS-S0101), `gate-mounts-for-the-sync-suite` (HS-S0104) and
  `headline-rules-and-mutant-registry` (HS-S0105), all of which merge before this story.
- Real `SyncPeer` / `IngestStore` bodies, the runner, and removing the crate's scoped
  `#![allow(clippy::todo)]` — `ingest-store-and-memory-peer-round-trip` (HS-S0098) and
  `send-free-sync-runner` (HS-S0109).
- Any change to `Envelope<T>`'s own shape, its `Serialize` derive, or its hand-written
  `Deserialize`'s structure. Adding a field to the envelope is a shape change and would bump the
  format version for a reason this story does not have.

**Merge DoD one-liner.** `cargo xtask affected --base main` is green, `cargo test -p
happenstance-sync --all-features` passes in both formats, `cargo xtask spec-trace` is green and
reports no new orphan, and `git diff` shows no `.kb/` and no `spec/SPECIFICATION.md` change.

The narrowest honest path set — `redkiln verify --grain story` reads the first fenced block below:

```
crates/happenstance-sync/src/wire.rs
crates/happenstance-sync/src/peer.rs
crates/happenstance-sync/src/identity.rs
crates/happenstance-sync/src/lib.rs
crates/happenstance-sync/Cargo.toml
crates/happenstance-sync/tests/wire.rs
CHANGELOG.md
.bklg/from-contract-to-published-library/replication-identity-and-ingest/message-set-on-the-envelope/**
```

`peer.rs` and `identity.rs` are in the set only for the derives ADR-0027 authorised and the rustdoc
that explains them; their *bodies* and their `todo!()`s belong to HS-S0098 and HS-S0109, and a diff
that fills one here has taken another story's work. `Cargo.toml` is in the set only if ADR-0027's
message set genuinely needs a dependency line this crate does not have — the expectation is that it
does not, because the payload codec is already core's.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| The message set is types *in* `Envelope<T>`, never an enum *around* it | Each message ADR-0027 names is its own `pub` type, wrapped with `Envelope::new(message)` at the call site. `Envelope` keeps exactly its two fields and its `Serialize` derive. There is no `enum Message` anywhere in the crate | `crates/happenstance-sync/src/wire.rs:8-16`, `:103-121`; `_decomposition.md` architecture brief §7 and AC-A08 |
| Derives are executed from ADR-0027, not chosen here | `PushBatch`, `EventGroup` and `ReplicatedEvent` gain exactly the derives the atom names, by name. A type the atom does not name stays underived and the message set is built without it | `crates/happenstance-sync/src/lib.rs:86-90`; `_storymap.md` `adr-0027-…` row (*"the message set plus the derives on `PushBatch`/`EventGroup`/`ReplicatedEvent` authorised **by name**"*) |
| No `skip_serializing_if`, anywhere, and `Guard::after` is always present | WF-2 forbids the attribute on any wire struct; WF-4 requires `after` on the wire always, because a policy can only refuse what it can see. Eliding it would silently disarm SY-6's refusal | `spec/SPECIFICATION.md:1943`, `:2027`, `:6020-6029`; `crates/happenstance-core/src/append.rs:120-140` |
| An unknown version is refused before a real message decodes | The existing hand-written `Deserialize` already does this; this story re-proves it with the *message set's own type* in `T` position rather than with `String`. The refusal carries `WireError::UNSUPPORTED_FORMAT_VERSION` in `serde_json`; in postcard the text is dropped by the format and the typed answer comes from `check_format_version` on a `take_from_bytes::<u16>` peek | `crates/happenstance-sync/src/wire.rs:145-217`, `:234-308`; `spec/SPECIFICATION.md:2168-2216` |
| The WF-8 oracle is a witness count, not a byte comparison | Decode `Envelope<Witness>` at an unknown version where `Witness`'s `Deserialize` records that it ran; assert **zero** through the real impl and **one** through a locally-declared derived envelope decoding the *same bytes*. Byte framing is identical between the two, so any assertion on bytes or on the returned `Err` is decorative | `spec/SPECIFICATION.md:2189-2203`; `crates/happenstance-sync/tests/wire.rs:1-33` |
| `DerivedEnvelopeDeserialize` is compiled, not described | The forbidden implementation — `#[derive(Deserialize)]` plus a post-hoc version check — exists in the test file as a negative control and is asserted to record one witness call where the real impl records zero. Without it the witness test is a rule no implementation can fail | `discover.md`, *The wrong implementation*; `crates/happenstance-sync/src/wire.rs:219-233` |
| The message set adds no second payload codec | `Event::data` / `Event::metadata` travel through `happenstance-core`'s own `serde` impls: base64 in human-readable formats, raw bytes otherwise. No `serde` attribute, helper or hand-written impl in this crate touches them. Asserted at the message-set level in both formats — `[de ad be ef]` renders as `"3q2+7w=="` in JSON and four raw bytes in postcard | WF-11, `spec/SPECIFICATION.md:2279-2322`; `crates/happenstance-core/tests/wire.rs:1060`, `:1114`; `.kb/decisions/0003-opaque-payloads.md` |
| An inverted / re-wrapping payload codec is compiled as a control | A message-level codec that swaps the `is_human_readable` branch (or hexes the payload "for readability") round-trips perfectly in both formats, so every assertion on a decoded value passes. The control asserts on the *rendered bytes*, which is the only thing that separates them | `spec/SPECIFICATION.md:2315-2322`; `discover.md`, *`InvertedHumanReadable`* |
| Group boundaries survive encoding | A `PushBatch` of *n* groups encodes and decodes as *n* groups with each group's `guard` attached to its own events. The receiver is told the decomposition; it never re-infers one | SY-30, `spec/SPECIFICATION.md:6700-6712`; `crates/happenstance-sync/src/peer.rs:183-196` |
| `FlattenedPushEncoding` is compiled as a control | Encoding `PushBatch` as a flat `Vec<ReplicatedEvent>` compares equal on every decoded value once the structure is gone, and is smaller. The control asserts the encoded form retains the grouping — a claim no round-trip assertion can make | `discover.md`, *And the quiet one*; `crates/happenstance-sync/src/peer.rs:186-190` |
| Messages are role-blind | No message type, field or variant names "hub", "spoke", "primary" or "replica". Hub-ness is an edge property and one adapter type serves both roles at once; a role in the wire format would make HS-S0110 a format break | SY-9 `[FROZEN]`, `spec/SPECIFICATION.md:6090-6106`; `crates/happenstance-sync/src/lib.rs:52-66` |
| `FORMAT_VERSION`'s disposition is written down where the constant is | Whether the version is per-message or negotiated once per connection, and whether landing the message set is itself a shape change that moves the constant, is executed from ADR-0027 and recorded in the constant's own rustdoc — including the reasoning, so the next reader does not re-derive it. A capacity bound still never bumps it | `crates/happenstance-sync/src/wire.rs:46-66`; WF-9, `spec/SPECIFICATION.md:2219-2248`; `.kb/open-questions/sync-message-set-and-format-version.md` |
| `WireError` and `SyncError` are untouched in shape | A version refusal stays a decoding failure with its own error; `SyncError` gains nothing here. `WireError::UNSUPPORTED_FORMAT_VERSION` stays public and stays the marker text that keeps "refused, park this" distinguishable from "malformed, drop this" | `crates/happenstance-sync/src/wire.rs:167-217`; `crates/happenstance-sync/src/lib.rs:90-96` |
| Every new wire test runs in both supported formats | WF-8's obligation is discharged by position in postcard and by an explicit check in `serde_json`; a test in one format proves half a clause. `postcard` and `serde_json` are already dev-dependencies for exactly this reason | `crates/happenstance-sync/Cargo.toml:26-35`; `crates/happenstance-sync/src/wire.rs:18-37` |
| New tests live in `mod wire { … }` and stay outside `RULE_FILES` | The wrapper is what makes `cargo test --list` print the qualified name a citation resolves to. `WIRE_TESTS` is deliberately outside `RULE_FILES` because a round trip of this crate's own encoding is not an adapter obligation; resolution is one-way | `crates/happenstance-sync/tests/wire.rs:25-33`; `xtask/src/spec_trace.rs:85-104` |
| Each new wire test earns a `CHANGELOG.md` entry naming a defect | CF-29's lint reads `RULE_FILES` only, so it does not mechanically reach a wire test. The convention is honoured anyway, and the fact that the lint cannot enforce it here is stated rather than relied on | `xtask/src/lints.rs:510-535`; CF-29 |
| No new test asserts a literal position or reads a clock | Positions reaching the wire — inside an `EventId`, and inside `Guard::after` — are values a store assigned or fixtures constructed, compared for identity or round-trip equality. `RecordedAt` values are constructed, never `now()`. A position on the wire *is* a naked integer, so a literal here would read as innocuous and would encode the belief that positions are portable | `discover.md`, **Box 6**; CF-6 / CF-33; `spec/SPECIFICATION.md:6006-6014`; `CLAUDE.md`, *The rule that matters* |
| Every new public item carries rustdoc with a compiled example | Repository-wide obligation, and the only check that the vocabulary is usable rather than merely correct. Fallible public functions carry an `# Errors` section naming the conditions, not the error type | `standards/rust/70-rustdoc-obligations.md`; `_design.md`, *Public API surface note* |
| Visibility and stability are deliberate | New message types are `pub` and `#[non_exhaustive]` where the shape may grow, matching `PushBatch` (`peer.rs:192-196`) and `WireError` (`wire.rs:175-177`). Nothing becomes `pub` that ADR-0027 did not name — an item that turned `pub` during implementation is a semver promise nobody made, and `publish = false` today does not make it free after HS-P0016 | `crates/happenstance-sync/Cargo.toml:12`; `crates/happenstance-sync/src/peer.rs:191-196`; `crates/happenstance-sync/src/wire.rs:175-194` |

## Data and migrations

**No database and no schema migration.** `happenstance-sync` owns no storage; the stores this crate
replicates between are `happenstance-core`'s and the adapter crates', and none of their schemas is
touched here.

What *is* a migration surface is the **encoding**, and it is governed by one number.

| Surface | What this PR does to it | Migration rule |
| --- | --- | --- |
| `Envelope<T>`'s own shape (`format_version`, `message`) | **Unchanged.** Two fields, `format_version` declared first, `Serialize` derived and `Deserialize` hand-written | A field added, removed, retyped or moved here is a shape change and bumps `FORMAT_VERSION` (`crates/happenstance-sync/src/wire.rs:46-58`) |
| The message set instantiated into `T` | **Introduced.** New types, and derives on three existing ones | Whether introducing the vocabulary bumps the constant is ADR-0027's answer, executed here and documented at the constant. Nothing is published, so both answers cost zero today and are not free after HS-P0016 (`crates/happenstance-sync/Cargo.toml:12`) |
| The payload bytes inside `Event` | **Passed through.** No codec added, no attribute overriding core's | Payload encoding belongs to `happenstance-core`'s `serde` feature (`crates/happenstance-core/Cargo.toml:49`); a change there is WF-11's, not this crate's |
| Capacity bounds (`PeerLimits`, `StoreLimit`) | **Untouched.** The message set only has to keep a limit refusal *reportable* | A bound change MUST NOT change the encoding and MUST NOT bump the version; the deployment mechanism is refuse-and-park, not renegotiation (WF-9, `spec/SPECIFICATION.md:2219-2248`) |

**There is no backward-compatibility burden to discharge and that is a fact with an expiry date.**
Both sync crates are `publish = false` and this project does not claim their names
(`_decomposition.md`, architecture brief *AC-015 — the disposition on claiming the two crate names*),
so no peer anywhere speaks version 1 of anything yet and a version bump is a one-line edit with no
deployment consequence. After `publication-and-positioning` (HS-P0016) ships, the same edit makes
every un-redeployed peer refuse every message from a redeployed one. That asymmetry is the reason
the disposition is written into the constant now rather than discovered later.

## Acceptance criteria

The persona throughout is **the adapter author** — Persona 2 of the initiative's distillation
(`.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md:114`),
walking the journey *Learn when you are finished* (`initiative.md`, **Referenced personas &
journeys**), and standing in this project's backbone as row A4, *"Speak the wire, and keep the
payload opaque"* (`_storymap.md`, **Backbone**). Their goal is not "types exist"; it is *I can
implement `SyncPeer` against a vocabulary I did not have to invent, and the suite will tell me when I
have got it wrong.* Two ACs are written from the second persona this touches — the **local-first /
edge developer** (`personas-and-journeys.md:182`), whose peers are heterogeneous and un-redeployed by
default, which is what makes the version disposition their problem rather than a housekeeping detail.

Every criterion is falsifiable by a named test. Where a criterion's whole point is that a *plausible
wrong implementation* passes the obvious assertion, the verification is the compiled control, not the
happy path — per `CLAUDE.md`, *a rule that no adapter can fail is decorative*.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **GIVEN** an adapter author reading `happenstance-sync`'s public surface to find out what a message *is*, **WHEN** they look at the message set this PR lands, **THEN** each message is its own `pub` type wrapped at the call site as `Envelope::new(message)`, `Envelope<T>` still carries exactly its two fields in declaration order, and no `enum Message { … }` of kinds exists anywhere in the crate — so adding a message kind later is not a shape change to the versioned type and cannot force a `FORMAT_VERSION` bump on a peer that has not been redeployed | `wire::envelope_shape_is_unchanged_by_the_message_set` in `crates/happenstance-sync/tests/wire.rs` — serialise `Envelope::new(<a real message-set value>)` and assert the JSON object has exactly the keys `format_version` and `message` in that order, and that postcard framing is still `<version varint><message>`. `MessageEnum` is the wrong implementation this rejects (`discover.md`, *The wrong implementation*) |
| AC-002 | **GIVEN** an adapter author who must serialise a batch to send it, **WHEN** they call `serde_json::to_string(&push_batch)` or `postcard::to_stdvec(&push_batch)`, **THEN** it compiles and round-trips — because `PushBatch`, `EventGroup` and `ReplicatedEvent` carry exactly the derives ADR-0027 names by name and no others, and any type the atom does not name is still underived and the message set is built without it | `wire::authorised_derives_round_trip_in_both_formats` in `crates/happenstance-sync/tests/wire.rs`, plus ledger evidence citing the ADR-0027 atom `file:line` for **each** derive landed. A derive with no citation is a wire-format decision taken in this story, which `crates/happenstance-sync/src/lib.rs:86-90` forbids |
| AC-003 | **GIVEN** an adapter author whose peer receives a message from a peer one release ahead, **WHEN** the envelope carries a version this build does not implement, **THEN** the message's own `Deserialize` is **never called** — proved over a *real* message-set type, in both `serde_json` and `postcard`, with the forbidden implementation compiled beside it and observed to call it once | `wire::version_is_readable_before_the_message` in `crates/happenstance-sync/tests/wire.rs` (existing rule, extended from `String` to a message-set type): assert the witness count is **zero** through the real `Envelope`, and **one** through the locally-declared `DerivedEnvelopeDeserialize` decoding the *same bytes*. Byte framing is identical between the two (`spec/SPECIFICATION.md:2189-2203`), so this is the only oracle that separates them |
| AC-004 | **GIVEN** an operator triaging a peer that has started refusing traffic, **WHEN** the refusal reaches them, **THEN** *"refused, park this"* is distinguishable from *"malformed, drop this"*: the `serde_json` error carries `WireError::UNSUPPORTED_FORMAT_VERSION` verbatim and the postcard path answers through `check_format_version` on a `take_from_bytes::<u16>` peek — and `SyncError` gained nothing, because a version refusal is a decoding failure and folding it into the runner's enum would take a phase-13 design decision inside an encoding change | `wire::rejects_an_unknown_format_version` in `crates/happenstance-sync/tests/wire.rs` (existing rule, extended to a message-set type in `T` position); plus `cargo test -p happenstance-sync --all-features` showing `WireError`'s and `SyncError`'s variants unchanged (`crates/happenstance-sync/src/lib.rs:90-96`) |
| AC-005 | **GIVEN** an application author who put encrypted bytes in `Event::data` and needs them to arrive as those bytes, **WHEN** a message carrying that event is encoded, **THEN** the payload is rendered by `happenstance-core`'s own codec and nothing else: `[de ad be ef]` appears as the JSON string `"3q2+7w=="` and as four raw bytes in postcard, with no `serde` attribute, helper or hand-written impl in this crate touching either field | `wire::payload_bytes_render_through_the_core_codec` in `crates/happenstance-sync/tests/wire.rs` — assert on the **rendered bytes** in both formats, not on a decoded value. WF-11 (`spec/SPECIFICATION.md:2279-2322`) and its two core rules at `crates/happenstance-core/tests/wire.rs:1060` and `:1114` must stay green over the new types |
| AC-006 | **GIVEN** the same author, **WHEN** somebody later "improves readability" by hexing the payload or swaps the `is_human_readable` branch, **THEN** a test fails by name — because the inverted / re-wrapping codec is compiled into the wire tests as a control and asserted against the rendered bytes, which is the only observation that separates it from the correct impl (it round-trips perfectly in both formats and every assertion on a decoded value is green) | `wire::an_inverted_payload_codec_is_caught_on_the_rendered_bytes` in `crates/happenstance-sync/tests/wire.rs` — the control renders `[222,173,190,239]` in JSON and the ASCII of `"3q2+7w=="` in postcard (`spec/SPECIFICATION.md:2315-2322`) and the assertion rejects it |
| AC-007 | **GIVEN** an adapter author whose origin store appended three independently guarded groups, **WHEN** the receiver decodes the push, **THEN** it is *told* three groups with each `guard` attached to its own events — it never re-infers a decomposition, because a receiver that regrouped would publish a state the origin never had | `wire::push_batch_preserves_group_boundaries_on_the_wire` in `crates/happenstance-sync/tests/wire.rs` — encode a `PushBatch` of *n* groups with distinguishable guards (at least one `Some`, one `None`), decode, and assert group count and per-group membership. SY-30, `spec/SPECIFICATION.md:6700-6712`; `crates/happenstance-sync/src/peer.rs:183-196` |
| AC-008 | **GIVEN** a future contributor who notices the nesting level costs bytes, **WHEN** they flatten `PushBatch` to a list of `ReplicatedEvent` "because the receiver can regroup by guard", **THEN** a test fails by name — because `FlattenedPushEncoding` is compiled as a control and the assertion is on the encoded *form* retaining the grouping, which no round-trip-on-values assertion can make | `wire::a_flattened_push_encoding_loses_the_group_boundary` in `crates/happenstance-sync/tests/wire.rs` — the control round-trips equal on decoded values and is smaller, and is rejected on structure (`discover.md`, *And the quiet one*) |
| AC-009 | **GIVEN** a replication hub that must refuse a peer-supplied condition whose `after` is `Some(_)` (SY-6), **WHEN** a wire-carried `AppendCondition` reaches it, **THEN** `after` is there to be read — always present on the wire, never elided, with no `skip_serializing_if` on any wire struct in this crate — because a policy can only refuse what it can see, and eliding the field would make `wire_condition_with_after_is_refused` unfalsifiable by deleting the evidence it inspects | `wire::guard_after_is_always_present_on_the_wire` in `crates/happenstance-sync/tests/wire.rs` — encode a group whose guard has `after: None` **and** one with `after: Some(_)`, and assert the key is present in both renderings. WF-2 `spec/SPECIFICATION.md:1943`, WF-4 `:6020-6029`; `crates/happenstance-core/src/append.rs:120-140` |
| AC-010 | **GIVEN** the local-first / edge developer who will wire the same adapter type as a hub on one edge and a spoke on another, **WHEN** they read the message set, **THEN** no type, field or variant names a role — no `hub`, `spoke`, `primary` or `replica` — because hub-ness is an edge property (SY-9 `[FROZEN]`) and a role in the wire format would make `hub-and-spoke-and-peer-to-peer-topologies` (HS-S0110) a format break rather than a wiring | `wire::messages_name_no_role` in `crates/happenstance-sync/tests/wire.rs` — an `include_str!` scan of `../src/wire.rs` and `../src/peer.rs` asserting the role vocabulary appears in no identifier (prose and rustdoc excluded by construction: the scan matches declarations, not comments). `spec/SPECIFICATION.md:6090-6106` |
| AC-011 | **GIVEN** the local-first / edge developer running two peers they cannot redeploy simultaneously, **WHEN** one of them raises a capacity bound, **THEN** nothing on the wire changes and the two stay mutually reachable — the over-capacity value decodes to `Ok` and is refused by the store that cannot hold it — and **WHEN** they ask what `FORMAT_VERSION = 1` is version 1 *of*, the constant's own rustdoc answers: whether the version is per-message or negotiated once per connection, whether landing the message set moved the constant, and the reasoning, executed from ADR-0027 rather than re-derived by the reader | `wire::a_capacity_bound_does_not_bump_the_format_version` in `crates/happenstance-sync/tests/wire.rs` (a value beyond `PeerLimits` decodes `Ok` at the supported version), plus the compiled doctest on `FORMAT_VERSION` in `crates/happenstance-sync/src/wire.rs:46-66` run by `cargo test -p happenstance-sync --all-features --doc`. WF-9, `spec/SPECIFICATION.md:2219-2248` |
| AC-012 | **GIVEN** an adapter author meeting this vocabulary for the first time, **WHEN** they open its docs, **THEN** every new public item carries rustdoc with a **compiled example** (and an `# Errors` section naming the conditions, not the error type, on anything fallible), nothing became `pub` that ADR-0027 did not name, every new wire test lists under `wire::` so a citation resolves, each earns a `CHANGELOG.md` entry naming the defect it detects, and no new test asserts a literal position value or reads a clock | `cargo test -p happenstance-sync --all-features --doc` (examples compile and run); `cargo test -p happenstance-sync --all-features -- --list` (every new name prints as `wire::…`); `cargo xtask lints` and `cargo xtask spec-trace` green with no new orphan; `git diff CHANGELOG.md` showing one entry per new test. `standards/rust/70-rustdoc-obligations.md`; `xtask/src/lints.rs:510-535`; `xtask/src/spec_trace.rs:85-104` |

**Coverage of the traced project AC.** All twelve serve **AC-006** — *"A payload survives the
boundary unchanged, and replay changes nothing"* (`project.md`, **Acceptance criteria**, AC-006).
This story owns the **wire** third of it: AC-005 and AC-006 above are the byte-level half of "the
payload survives", AC-003/AC-004 are what make the boundary refuse rather than half-decode, and
AC-001/AC-007/AC-011 are what keep the vocabulary extensible enough for the round trip to still be
running a release later. The *assertions across a real store boundary* and the replay no-op are
`byte-identical-round-trip-and-idempotent-replay` (HS-S0107); the marker lift is
`adr-0003-provisional-lift` (HS-S0108). No project AC other than AC-006 is claimed here.

## Interaction quality

**STATE invariants: not applicable, and the determination is signed off rather than assumed.**
`_design.md` records **no user-facing surface** for this project — no route, no DOM node, no TUI
pane, nothing a screenshot could be taken of — and that no-surface determination is itself what the
repository owner approved on 2026-08-12 (`_design.md`, **Sign-off**). This story renders none, claims
no `## Items` id, and `design.capture` is a declared skip repository-wide (`CLAUDE.md`, *Where the
work lives*). In-place-vs-context-jump, occlusion, focus/scroll/selection preservation, reversibility
and keyboard reachability have no referent here; asserting them would be theatre.

**COMPOSITION invariants: the medium is the public API, and the family transfers with teeth.** The
signed-off design does not leave this blank — it names the obligation and says where it is
discharged: *"the public API surface is the only 'surface' here… it is repository-wide obligated to
carry rustdoc"* (`_design.md`, **Public API surface note**; `initiative.md:196-197`). Read against a
library, the composition family says: *presentation exists at all* means every public item is
composed for a reader rather than emitted as bare markup — a type with a signature and no worked
example is the API-surface equivalent of an unstyled render, correct against every structural
assertion and unusable. That invariant is **AC-012**, and it is a table row precisely because
`redkiln verify` extracts ACs from table cells and bullets, so a composition invariant left as prose
here would never be gated and never tested.

Which AC carries each transferred invariant, and how each is verified:

| Composition invariant (design → API medium) | Carried by | Verified by |
| --- | --- | --- |
| **Presentation exists at all** — every new public item is composed for a reader: rustdoc plus a compiled example, `# Errors` naming conditions on anything fallible | AC-012 | `cargo test -p happenstance-sync --all-features --doc`; `standards/rust/70-rustdoc-obligations.md` |
| **Composition and placement** — the message set sits *beside* `Envelope<T>` in the module already declared and re-exported, not in a parallel path reachable only from a test | AC-001, and the Integration contract's mount point | `wire::envelope_shape_is_unchanged_by_the_message_set`; `crates/happenstance-sync/src/lib.rs:147`, `:154-160` |
| **Transience** — what is permanent public vocabulary versus what stays internal: nothing becomes `pub` that ADR-0027 did not name, and `#[non_exhaustive]` marks the shapes that may grow | AC-012 (visibility half), AC-002 (derive half) | Ledger evidence citing ADR-0027 per item; `crates/happenstance-sync/src/peer.rs:191-196` as the existing pattern |
| **Density budget, with its real numbers** — `Envelope<T>` stays at **exactly two** fields; a `PushBatch` of *n* groups decodes as *n* groups; the payload is copied **zero** additional times and encoded by **one** codec | AC-001, AC-007, AC-005 | `wire::envelope_shape_is_unchanged_by_the_message_set`; `wire::push_batch_preserves_group_boundaries_on_the_wire`; `wire::payload_bytes_render_through_the_core_codec` |
| **Hierarchy** — the version is read before anything else, and the group boundary is above the event | AC-003, AC-007 | `wire::version_is_readable_before_the_message`; `wire::push_batch_preserves_group_boundaries_on_the_wire` |
| **Named anti-patterns** — `MessageEnum`, `DerivedEnvelopeDeserialize`, `InvertedHumanReadable`, `FlattenedPushEncoding` (`discover.md`, *The wrong implementation*) | AC-001, AC-003, AC-006, AC-008 | Each is a **compiled** control in `crates/happenstance-sync/tests/wire.rs`, not a described one; three of the four pass every assertion a first implementation would reach for |

## Error conditions

| id | Condition | Required behaviour | Evidence |
| --- | --- | --- | --- |
| EC-001 | An envelope arrives at a `format_version` this build does not implement | Refuse the **whole** message. No part of `T` is decoded — not opportunistically, not "to log what it was". The error carries `WireError::UNSUPPORTED_FORMAT_VERSION` where the format can carry text | WF-8 `[FROZEN]`, `spec/SPECIFICATION.md:2168-2216`; AC-003, AC-004 |
| EC-002 | The body is malformed at a version this build *does* implement | An ordinary `serde` decoding error, textually distinct from EC-001's marker, so a runner can drop rather than park. `WireError` gains no variant for this | `crates/happenstance-sync/src/wire.rs:167-217`; AC-004 |
| EC-003 | A value exceeds a receiver's capacity bound | Decode succeeds. The refusal happens at the store, with an error naming it, and stays reportable to the runner. The version does **not** move and the encoding does **not** change | WF-9, `spec/SPECIFICATION.md:2219-2248`; AC-011 |
| EC-004 | ADR-0027 names no derive for a type the message set appears to need one on | **Stop and raise it.** The type stays underived and the message set is built without it, or the story blocks on an amendment to the atom. A derive chosen during implementation is a wire format chosen during implementation | `crates/happenstance-sync/src/lib.rs:86-90`; AC-002 |
| EC-005 | The format cannot carry the marker text (postcard drops strings from the error) | The typed answer comes from `check_format_version` on a `take_from_bytes::<u16>` peek, not from string matching. A test that only asserts the marker proves half a clause | `crates/happenstance-sync/src/wire.rs:145-217`; AC-004 |
| EC-006 | A wire-carried `AppendCondition` has `after: Some(_)` | Not this story's refusal — SY-6's, in the sync testkit. This story's obligation is that the field is **on the wire to be seen**; a `skip_serializing_if` here would disarm that rule silently and green | `spec/SPECIFICATION.md:6020-6029`; AC-009 |

## Non-functional

| id | Requirement | Why it is not decorative | Evidence |
| --- | --- | --- | --- |
| NF-001 | No new runtime dependency. `crates/happenstance-sync/Cargo.toml` is expected to be **unchanged**; if ADR-0027's message set genuinely needs a line this crate lacks, that is a finding to state, not a quiet addition | The payload codec is already `happenstance-core`'s; a new dependency here is the first symptom of a second codec being written | `crates/happenstance-sync/Cargo.toml:14-35`; AC-005 |
| NF-002 | The payload is moved, never re-encoded: no additional copy of `Event::data` / `Event::metadata` on the encode path | This is what makes HS-S0107's byte-identity assertion possible at all, and what DR-5 protects — *"a suite that parses a payload would certify a peer that does"* | `project.md`, DR-5; `spec/SPECIFICATION.md:6868` |
| NF-003 | Every new wire test runs in **both** `serde_json` and `postcard` | WF-8's obligation is discharged by position in one and by an explicit check in the other; a test in a single format proves half a clause and leaves the other half free to regress | `crates/happenstance-sync/src/wire.rs:18-37`; `crates/happenstance-sync/Cargo.toml:26-35` |
| NF-004 | The crate still builds for `wasm32-unknown-unknown`, with no `Send` bound introduced anywhere in the message set | Binding constraint 1. The sync `wasm32` steps exist from `gate-mounts-for-the-sync-suite` (HS-S0104) and are the standing guard | `CLAUDE.md`, **Binding constraints** 1; `_decomposition.md`, architecture brief §6 |
| NF-005 | Both sync crates stay `publish = false`; no crate name is claimed by this story | AC-015's disposition, and the reason a version bump costs nothing today. The `cargo package --list` assertion inside `cargo xtask ci` is what notices | `crates/happenstance-sync/Cargo.toml:12`; `_decomposition.md`, testing brief, static tier |
| NF-006 | `cargo xtask ci --fast` is this project's ceiling; the full `cargo xtask ci` belongs to the terminal project | DoD 1, and `.redkiln/config.yaml`'s `integration_scoped` wiring — running the whole gate here would not be wrong, but claiming it as this story's bar would misreport the project's grain | `_decomposition.md`, testing brief, **Merge-gate commands** |

## Implementation notes (non-prescriptive)

Nothing below is binding. It is the order that avoids the two expensive mistakes this story is
positioned to make — writing a wire format that ADR-0027 did not authorise, and writing tests that
pass against the implementation the clause forbids.

**Read ADR-0027 before opening a `.rs` file, and transcribe rather than infer.** The atom is merged
by `adr-0027-merge-compensation-and-message-set` (HS-S0099) before this story starts. Make a literal
list first: which types are messages, which are return values, which derives are authorised on
`PushBatch` / `EventGroup` / `ReplicatedEvent` by name, and what the `FORMAT_VERSION` disposition is.
Anything on that list without a `file:line` behind it is a decision this story is not allowed to
take (EC-004). That list is also the ledger's evidence for AC-002.

**Extend `crates/happenstance-sync/tests/wire.rs`; do not start a second file.** The existing module
already carries the witness pattern (`Witness`, `foreign_json`, `foreign_postcard`,
`UNSUPPORTED_VERSION` written as an offset from `FORMAT_VERSION` rather than as a literal) and the
locally-declared derived envelope. Reuse those, and note the deliberate detail at `:47-49`: the
unsupported version is `FORMAT_VERSION.wrapping_add(1)` precisely so that a bump does not silently
turn the "unknown" version into the supported one and invert both tests. If AC-011's disposition
moves the constant, that idiom is what keeps the existing tests honest — and the doctest at
`crates/happenstance-sync/src/wire.rs:84` that asserts `{"format_version":1,…}` is what will need
updating with it.

**Write each control the way the existing one is written** — locally declared inside the test module,
in the same shape as the derived envelope already there, decoding the *same bytes* as the real path
in the same test so a reader sees the discrimination instead of taking it on trust
(`crates/happenstance-sync/tests/wire.rs:18-24`). Three of the four named anti-patterns pass every
assertion a first implementation reaches for; a control that is described in a comment instead of
compiled is exactly the decorative rule `CLAUDE.md` forbids.

**Assert on rendered bytes wherever the mutant round-trips.** For AC-005/AC-006 and AC-007/AC-008 the
decoded value is *identical* under the wrong implementation. The habit to build is: encode, assert on
the bytes, then decode and assert on the value — in that order, so the byte assertion cannot be
quietly dropped as redundant.

**Leave `peer.rs` and `identity.rs` bodies alone.** They are in the path set for their derives and
their rustdoc only. A `todo!()` filled here has taken HS-S0098's or HS-S0109's work, and the crate's
scoped `#![allow(clippy::todo)]` stays until `send-free-sync-runner` removes it.

**When deleting the crate prose this PR makes untrue, move the finding first.** The module
documentation at `crates/happenstance-sync/src/wire.rs:1-16` and the note at
`crates/happenstance-sync/src/lib.rs:86-96` hold *reasoning*, not just status. Each sentence deleted
should already exist inside ADR-0027 or inside a rustdoc paragraph on the item it constrains
(`_decomposition.md`, architecture brief **Notes**, *One thing to carry forward that no AC
captures*).

**A last check before calling it done.** Ask of each new test: *name the implementation this fails
against.* If the answer is "none I can write", the test is decorative and the control is missing.

## Tests and CI (merge gate)

Tiers per `_decomposition.md`, **Testing brief** → *The test mix, tier by tier*. This story is
**Static + Unit only** — AC-006's integration tier (two real stores, a real hop) is
`byte-identical-round-trip-and-idempotent-replay` (HS-S0107)'s and is deliberately absent here.

| Tier | Command / path | Proves |
| --- | --- | --- |
| Static | `cargo xtask affected --base main` | The story grain: only what this diff could break. The `verify:` block wires it (`.redkiln/config.yaml`), so it runs whether or not anyone types it |
| Static | `cargo fmt --check` and `cargo clippy --workspace --all-targets -- -D warnings` | Applies to the new public items the moment they exist; `missing_docs`-class findings are the first signal AC-012 is not met |
| Static | `cargo xtask spec-trace` | Every `WF`/`SY` clause's `Rule:` still resolves and no rule is orphaned. Named because this story lands wire tests **outside** `RULE_FILES` on purpose, and `spec-trace` is what would notice if one drifted into it (`xtask/src/spec_trace.rs:85-104`) |
| Static | `cargo xtask lints` | The five-lint story-grain gate `reachability_static` wires. Carries CF-6's position-literal check and CF-29's changelog-per-rule check — the latter reads `RULE_FILES` only and therefore cannot mechanically reach a wire test, which is why AC-012 states the convention is honoured by hand and says so out loud (`xtask/src/lints.rs:510-535`) |
| Static | `cargo xtask wasm` | The sync `wasm32` steps HS-S0104 added still pass: no `Send` bound and no host-only dependency entered through the message set (NF-004) |
| Unit | `cargo test -p happenstance-sync --all-features` | AC-001 through AC-011's wire tests, in both formats, with all four controls compiled. This is the command the ledger cites for every one of them |
| Unit | `cargo test -p happenstance-sync --all-features --doc` | AC-012's compiled examples, and AC-011's `FORMAT_VERSION` disposition doctest. A rustdoc example that does not compile is documentation of a shape that does not exist |
| Unit | `cargo test -p happenstance-sync --all-features -- --list` | Every new test prints as `wire::<name>`, so a citation to it resolves. The `mod wire { … }` wrapper is load-bearing, not cosmetic (`crates/happenstance-sync/tests/wire.rs:25-33`) |
| Unit (regression) | `cargo test -p happenstance-core --all-features` | WF-11's two existing rules (`crates/happenstance-core/tests/wire.rs:1060`, `:1114`) still green — the message set must not have introduced a payload codec that shadows core's |
| Unit (regression) | `cargo test -p happenstance-sync-testkit --all-features` | The rules HS-S0105 landed are unaffected. This story adds no testkit rule and must not perturb one |
| Integration | **Deliberately none in this PR.** | AC-006's byte-identity and idempotent-replay assertions need two real stores and a real hop; they are HS-S0107's, in the same slice and the same context (`_decomposition.md`, testing brief, integration tier) |
| Ceiling | `cargo xtask ci --fast` | `integration_scoped` — this project's ceiling per DoD 1. Green here proves the gate passed; it never on its own proves which assertion ran, which is why the ledger cites the specific test per AC (`_decomposition.md`, **Merge-gate commands**) |
| Merge gate | `git diff --stat` over `.kb/` and `spec/SPECIFICATION.md` | **Empty.** Both are other stories' (HS-S0099/HS-S0100 and HS-S0112). A non-empty diff here is scope theft, not a bonus |

## Risks and coupling (PR-scoped)

| Risk | Likelihood / impact | Handling inside this PR |
| --- | --- | --- |
| **ADR-0027 lands thinner than this story assumes** — it names the merge rule and scope but leaves a derive or a message boundary unstated | Medium / High | This is EC-004 and it is a **stop**, not a judgement call. The message set is built without the unnamed piece, or the story blocks on an amendment to the atom. The one thing that must not happen is a derive appearing in a diff with no citation; the ledger's AC-002 row is where that becomes visible |
| **The version disposition moves `FORMAT_VERSION` and breaks existing green tests** | Medium / Low | Known and cheap. The doctest at `crates/happenstance-sync/src/wire.rs:84` asserts `{"format_version":1,"message":"hello"}` literally and would need updating; the test module's `UNSUPPORTED_VERSION = FORMAT_VERSION.wrapping_add(1)` idiom (`tests/wire.rs:47-49`) is already bump-safe by construction. Nothing is published, so the deployment cost is zero today (`crates/happenstance-sync/Cargo.toml:12`) |
| **A control is described instead of compiled** — the failure mode this PR is most likely to have and least likely to notice | Medium / High | Three of the four anti-patterns pass every assertion a first implementation writes. AC-003, AC-006 and AC-008 each name their control explicitly, and the review question is fixed: *name the implementation this test fails against* |
| **A second payload codec enters through a "readability" helper** | Low / High | AC-005 and AC-006 assert on rendered bytes rather than decoded values. NF-001 makes a new dependency line in `Cargo.toml` the visible early symptom |
| **Scope drift into `peer.rs` / `identity.rs` bodies** — both files are in the path set and both are full of `todo!()` | Medium / Medium | The PR boundary says derives and rustdoc only; the bodies are HS-S0098's and HS-S0109's. The crate's scoped `#![allow(clippy::todo)]` stays, which is itself the tell that this story did not remove one |
| **Coupling to HS-S0107, in the same context** | High / Low by design | Slice-mates are implemented in one context, in the merge order at `_storymap.md`, **Merge order** item 4. The coupling is real and the sequence handles it: this PR encodes, HS-S0107 asserts across a boundary, HS-S0108 cites the result. Encoding and asserting must not be collapsed into one commit — the ADR-0003 lift depends on the round trip being evidence produced by a test, not by the same change that wrote the codec |
| **A role name arrives through ADR-0027's own vocabulary** | Low / High | AC-010's scan is mechanical rather than a review habit. If the atom itself names a role in a *message*, that is a conflict with `[FROZEN]` SY-9 to raise before implementing, not to encode |

## Dependencies

**Blocks on** — both merged before this story opens a `.rs` file:

- `adr-0027-merge-compensation-and-message-set` (HS-S0099) — the message set itself, every derive
  authorised **by name**, and the `FORMAT_VERSION` disposition. Without it this story has nothing to
  execute and would be *deciding* a wire format, which `crates/happenstance-sync/src/lib.rs:86-90`
  forbids. It arrives through `.kb/_intake/` and `/redkiln:kb-ingest`, never hand-authored.
- `headline-rules-and-mutant-registry` (HS-S0105) — the rule-plus-compiled-mutant pattern and the
  `Declared` table with its `expect` pins, established against `MemorySyncPeer` first. This story
  follows that pattern for wire tests rather than inventing a second one; it adds no registry entry,
  because a wire test is not an adapter obligation (`xtask/src/spec_trace.rs:85-104`).

Transitively: `adr-0026-peer-ingest-and-transport` (HS-S0098's predecessor) precedes ADR-0027, and
`gate-mounts-for-the-sync-suite` (HS-S0104) precedes HS-S0105 — neither is a direct edge here.

**Unlocks**:

- `byte-identical-round-trip-and-idempotent-replay` (HS-S0107) — consumes what this PR encodes;
  its byte-identity assertion is over these types. Same slice, next in merge order.
- `adr-0003-provisional-lift` (HS-S0108) — cites HS-S0107's round trip, so it is unlocked one hop
  later. Last in the slice on purpose: it cites evidence that does not exist until HS-S0107 merges.
- `durable-object-and-neon-peers` (HS-S0111) — names `message-set-on-the-envelope` in its own
  `depends_on` (`_storymap.md`, *Slices*), because a peer implemented against no vocabulary would be
  implemented against an invented one.

## Anchors (progressive disclosure)

Open these when the bound AC says to, not before. Each is linked rather than pasted; the Context pack
above already carries the decisions, and these carry the depth behind them.

| Anchor | Why it is load-bearing | When to open | Serves |
| --- | --- | --- | --- |
| `crates/happenstance-sync/src/wire.rs` | The mount point, and the only file that states *why* the envelope is generic rather than an enum (`:1-16`), what bumps `FORMAT_VERSION` and what must never bump it (`:46-65`), and how the hand-written `Deserialize` intercepts the version before `T` (`:219-308`) | First, before writing any line of the message set | AC-001, AC-011 |
| `crates/happenstance-sync/tests/wire.rs` | The existing witness pattern in full: `Witness`, the foreign encoders, `UNSUPPORTED_VERSION` as an offset rather than a literal, the locally-declared derived envelope, and the `mod wire` wrapper that makes rule names resolve | Before writing the first new test — copy this shape rather than invent one | AC-003, AC-004, AC-012 |
| `spec/SPECIFICATION.md` | WF-8 `[FROZEN]` at `:2168-2216` with the measured reason the oracle is a witness at `:2189-2203`; WF-9 at `:2219-2248`; WF-11 with the `[de ad be ef]` / `"3q2+7w=="` pin at `:2279-2322`; WF-2 at `:1943`; SY-9 at `:6090-6106`; SY-30 at `:6700-6712` | Open the specific range named by the AC you are implementing; do not read the file | AC-003, AC-005, AC-007, AC-010, AC-011 |
| `crates/happenstance-sync/src/peer.rs` | The candidate vocabulary as it exists today: `PushBatch` (`:183-196`), `EventGroup` and its `guard` (`:236-252`), `Ack` (`:254-278`), `Pulled` (`:166-190`), `PeerLimits` (`:280-338`) — and the `#[non_exhaustive]` pattern to match at `:191-196` | Before adding a derive or a `pub` item — check what already exists rather than declaring a parallel type | AC-002, AC-007, AC-009 |
| `crates/happenstance-sync/src/identity.rs` | `ReplicatedEvent` at `:171-199` — already carries `EventId`, `RecordedAt` and the opaque `Event`, which is what makes the payload reachable *without* being reachable *through* (DR-5) | Before implementing AC-005; it is the type the payload assertions run over | AC-005, AC-002 |
| `crates/happenstance-core/tests/wire.rs` | The two green WF-11 rules at `:1060` and `:1114` — the exact assertion shape for base64-in-human-readable and raw-bytes-otherwise, written once and not to be re-derived | Before implementing AC-005 and AC-006 | AC-005, AC-006 |
| `crates/happenstance-core/src/append.rs` | `Guard` at `:120-140`: `#[non_exhaustive]` with **public** fields, readable-but-not-literal-constructible, which is exactly what makes SY-6's refusal checkable and what a `skip_serializing_if` would destroy | Before implementing AC-009 | AC-009 |
| `crates/happenstance-sync/src/lib.rs` | `:86-96` — why the three derives were withdrawn and never restored, and why `SyncError` is not extended by wire work. `:147` and `:154-160` are the mount wiring | Before AC-002 (the derive rule) and before touching error types for AC-004 | AC-002, AC-004 |
| `xtask/src/spec_trace.rs` | `RULE_FILES` at `:85-89` and `WIRE_TESTS` at `:91-104` — the one-way resolution that keeps a wire test out of the adapter-obligation set. Blurring it re-opens a distinction ADR-0016 §15 settled | Before deciding where a new test lives; and again if `spec-trace` reports an orphan | AC-012 |
| `xtask/src/lints.rs` | `:510-535` — CF-29's changelog-per-rule lint reads `RULE_FILES` only, so it cannot mechanically reach a wire test. This is why AC-012 honours the convention by hand and states the gap | Before writing the `CHANGELOG.md` entries | AC-012 |
| `standards/rust/70-rustdoc-obligations.md` | The rustdoc bar in full, with the mechanism attached: compiled examples, `# Errors` naming conditions rather than types | Before writing rustdoc on any new public item | AC-012 |
| `standards/rust/60-what-a-test-must-prove.md` | RS-60-3 (one defect is one overridden default, selected by a marker) and RS-60-4 (spell the oracle in a direction sharing no subroutine with the implementation) — directly why the controls must not reuse the real path's own codec | Before writing the first negative control | AC-003, AC-006, AC-008 |
| `.kb/open-questions/sync-message-set-and-format-version.md` | The open question this story's outcome makes answerable, including the consequence of per-connection negotiation (*"would make the field dead weight on every message and removing it a format break"*) | Before writing the `FORMAT_VERSION` disposition into the constant's rustdoc | AC-011 |
| `.kb/decisions/0003-opaque-payloads.md` | The accepted, still-`provisional` atom whose marker HS-S0108 lifts using HS-S0107's round trip. It is why the payload may not be re-encoded here — and it is **read-only**: an accepted atom's body is immutable | Before implementing AC-005; and re-read before any temptation to "just adjust" the atom | AC-005, AC-006 |
| `crates/happenstance-testkit/tests/mutation_coverage.rs` | The `Declared` table shape at `:140-175` and the conformant-variant control at `:326-338` (`GappedPositionStore`) — the repository's established idiom for "a control that exists to be tripped over" | If a control's shape is unclear; it is the precedent, not a requirement to register one here | AC-006, AC-008 |
| `.bklg/from-contract-to-published-library/replication-identity-and-ingest/_decomposition.md` | Architecture brief §7 (*The wire mounts inside `Envelope<T>`, never around it*) and AC-A08; testing brief *The test mix, tier by tier*, unit tier, and **Merge-gate commands** | When the mount shape or the exact merge-gate command list is in question | AC-001, AC-012 |
| `.bklg/from-contract-to-published-library/replication-identity-and-ingest/message-set-on-the-envelope/discover.md` | The four named wrong implementations with their measurements — `DerivedEnvelopeDeserialize`'s byte-identical framing, `MessageEnum`'s one-release-later failure, `InvertedHumanReadable`'s perfect round trip, `FlattenedPushEncoding`'s smaller bytes | Before writing each control; it is the source the Context pack distilled | AC-001, AC-003, AC-006, AC-008 |
| `.bklg/from-contract-to-published-library/replication-identity-and-ingest/_grounding.md` | The project's pre-verified atom-and-code facts, so a claim can be re-checked without re-deriving it from the tree | If any citation in this spec looks stale against `HEAD` | AC-002 |
| `.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md` | Persona 2 (`:114`, the adapter author) and Persona 3 (`:182`, the local-first / edge developer) in full, including the qualification that all four rest on secondary evidence | If an acceptance criterion's framing needs to be checked against who it is for | AC-010, AC-011, AC-012 |

## Clarifications resolved during spec

**The twelve AC ids are exactly those the first pass enumerated.** None was added and none dropped.
Two consolidations were made *within* that set, and both are recorded here rather than left as
silent structure:

1. **The WF-8 witness and its control are one AC (AC-003), not two.** The existing test already
   asserts both directions in a single body — zero calls through the real `Envelope`, one through a
   locally-declared derived envelope decoding the same bytes
   (`crates/happenstance-sync/tests/wire.rs:18-24`). Splitting them would have produced two ledger
   rows citing one test, which is a weaker gate than one row that fails if either direction is
   missing. The payload and group-boundary controls stayed separate (AC-006, AC-008) because each is
   a distinct compiled implementation whose *absence* is exactly the silent failure worth gating.
2. **The hygiene family is one AC (AC-012).** Rustdoc with a compiled example, deliberate visibility,
   the `CHANGELOG.md` entry per new test, `wire::`-qualified naming, and the no-literal-position /
   no-clock discipline are all "the record is honest and the vocabulary is usable". They are gated
   together because each is verified by a command rather than by a test body, and because splitting
   them would have pushed a genuine deliverable — the `FORMAT_VERSION` disposition — out of the
   twelve.

**The design surface question is settled, not skipped.** `_design.md` exists, was verified with
`test -f`, and records **no user-facing surface** — a determination that was itself signed off
(2026-08-12). So this spec's *Interaction quality* section transfers the composition family to the
public-API medium the design explicitly points at (`_design.md`, **Public API surface note**) rather
than declaring the section inapplicable. The state family genuinely has no referent and says so.

**Where a rule this story proves would live is decided, and the decision is not this story's to
re-take.** Wire tests go in `crates/happenstance-sync/tests/wire.rs` (`WIRE_TESTS`), never in
`RULE_FILES`. SY-30's peer-observable rule — `push_envelope_preserves_group_boundaries (new,
happenstance-sync-testkit)`, `spec/SPECIFICATION.md:6710-6711` — is a claim about what a *peer* does
and is not landed here. AC-007 proves the *encoding* half only, and the distinction is ADR-0016
§15's.

**Three things deferred by name rather than resolved.** (a) Whether landing the message set moves
`FORMAT_VERSION` is ADR-0027's and is *executed* by AC-011, not decided here — the spec states the
consequence either way and the bump-safe test idiom that survives both. (b) Whether `PeerLimits` is
negotiated or polled is ADR-0027's; AC-011 only requires that a capacity refusal stays reportable
without a version move. (c) If WF-11's `[PROVISIONAL]` marker turns out to be falsified by what the
message set needs, that is a recorded finding routed to `clause-arithmetic-and-deferral-renewals`
(HS-S0113) — never a clause edited in passing, and never a `[FROZEN]` clause edited at all.
