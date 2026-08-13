---
item: HS-S0106
stage: discover
created: 2026-08-12T13:03:27.343Z
updated: 2026-08-12T13:03:27.343Z
template_sig: 86ce4036
rendered_sig: 40ea3000
---

# Discover — The message set instantiates Envelope<T>

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Slice one-line: land the message set as new types instantiating `Envelope<T>` (never an enum around it), with the derives ADR-0027 authorised by name, `FORMAT_VERSION`'s disposition settled, and the refusal of an unknown version proven to happen **before** any message is decoded — plus the WF rules and their mutants | `_storymap.md`, *Slices* table, `wire-message-set-and-round-trip` row 1 | Types, derives, a version disposition, and rules. The envelope's own shape does not change |
| **Depends on `headline-rules-and-mutant-registry` (HS-S0105)** for the rule-plus-mutant pattern the WF rules follow | `_storymap.md`, *Slices* `depends_on` | The `Declared` table and its `expect` pins already exist by the time the wire rules land |
| **Depends on `adr-0027-merge-compensation-and-message-set` (HS-S0099)** for the message set itself and for each derive, authorised **by name** | `_storymap.md`, *Slices* `depends_on`; `_decomposition.md`, AC-A08 | *"a `#[derive]` on a public message type **is** a wire format… what travels is phase 13's"* (`crates/happenstance-sync/src/lib.rs:86-94`) |
| **AC-006** — a payload survives the boundary unchanged and replay changes nothing | `project.md`, *Acceptance criteria*, AC-006; `_storymap.md`, *Coverage* AC-006 row | This story owns the **wire** third; HS-S0107 owns the assertions and HS-S0108 the ADR-0003 lift |
| `Envelope<T>` is generic in `T` and is **not** an enum of message kinds, because `enum Message { Push(..), Pull(..) }` *"would put the message set beside the version check, and the message set is exactly what phase 13 has not designed"* | `crates/happenstance-sync/src/wire.rs:8-16` | The message set instantiates `T`. Turning the envelope into an enum inverts the decision it was built to preserve |
| The module ships exactly four things per ADR-0016 §11: `FORMAT_VERSION`, `Envelope`, its hand-written `Deserialize`, and `WireError` | `crates/happenstance-sync/src/wire.rs:1-16` | The fifth thing — the message set — is this story's, and it goes beside the envelope rather than inside it |
| **WF-8 `[FROZEN]`** — every message carries `format_version` and a receiver MUST read and check it **before any part of the message is decoded**; in a self-describing format the obligation is discharged by an explicit check in a hand-written `Deserialize` and **not** by position; a receiver that does not implement a version MUST refuse the whole message and MUST NOT attempt a partial decode | `spec/SPECIFICATION.md:2168-2216` | The derive is not an option for the decoding half. Measured: a derived `Deserialize` accepts `{"message":null,"format_version":999}` and returns `Ok` |
| WF-8's rules are `wire::rejects_an_unknown_format_version` and `wire::version_is_readable_before_the_message`, and **the replacement is a witness test, not a byte-layout test** — decode an `Envelope<Witness>` at an unknown version where `Witness`'s `Deserialize` records that it ran, and assert the count is **zero**; the derive records **one** | `spec/SPECIFICATION.md:2189-2203` | The oracle is already designed, and it is designed this way because byte framing does **not** distinguish the two implementations |
| **WF-9 `[FROZEN]`** — a bound change is not a wire change; a peer whose limit is lower MUST decode successfully, refuse with `AppendError::ExceedsStoreLimit`, and make the refusal reportable to the sync runner | `spec/SPECIFICATION.md:2219-2248` | The message set must carry enough for a limit refusal to be *reportable*. `PeerLimits` already exists (`crates/happenstance-sync/src/peer.rs:274-301`) |
| `FORMAT_VERSION` bumps whenever the **shape** of any wire type changes and MUST NOT bump on a capacity bound — conflating the two *"makes every peer in a heterogeneous deployment unreachable the moment one of them raises a limit"* | `crates/happenstance-sync/src/wire.rs:46-65`; `spec/SPECIFICATION.md:2219-2248` | Whether landing a message set is itself a shape change, and therefore whether the constant moves, is a decision this story executes from ADR-0027 |
| **WF-11 `[PROVISIONAL]`** — `Event::data` and `Event::metadata` MUST encode as standard-alphabet base64 in human-readable formats and as raw byte strings otherwise; both rules exist to reject an **inverted `is_human_readable` branch, invisible to WF-7's rules** | `spec/SPECIFICATION.md:2279-2322` | Measured: `[de ad be ef]` must render as `"3q2+7w=="` in JSON and four raw bytes in postcard; an inverted impl passes **both round trips** |
| The open question atom this story's outcome resolves: *"`FORMAT_VERSION = 1` names a message set that does not exist yet"* — the field is fully specified and tested, its **meaning** is not | `.kb/open-questions/sync-message-set-and-format-version.md` | ADR-0027 decides; `open-questions-resolved-and-indexed` (HS-S0100) records; this story is what makes the answer true in code |
| `PushBatch`, `EventGroup` and `ReplicatedEvent` carry **no** derives; they had them briefly, they were withdrawn, and phase 5 did not restore them | `crates/happenstance-sync/src/lib.rs:86-90`; `crates/happenstance-sync/Cargo.toml:18-22` | Adding one is a wire-format decision that must already be in ADR-0027 by name before this story opens the file |
| `SyncError` is **not** extended by the wire work: a version refusal is a decoding failure with its own error, `wire::WireError`, and folding it into the runner's enum *"would take a phase-13 design decision inside an encoding change"* | `crates/happenstance-sync/src/lib.rs:90-96` | `WireError` stays separate. The `SyncError` extension is ADR-0026's |
| Existing wire tests run in **both** formats because WF-8's obligation is discharged differently in each — by position in postcard, by an explicit check in `serde_json` where key order is not observable | `crates/happenstance-sync/tests/wire.rs`; `crates/happenstance-sync/Cargo.toml:30-35` | Any new message-set rule inherits the two-format discipline |
| **DR-5 / SY-35** — the suite never decodes `Event::data` or `Event::metadata`; a suite that parsed a payload would certify a peer that does | `project.md`, DR-5; `RUNBOOK.md:4576-4580`; `spec/SPECIFICATION.md:6868` | The message set must make the payload reachable *without* being reachable *through*. `ReplicatedEvent` already does (`identity.rs:165-180`) |
| **SY-30** — the push envelope makes the decomposition into independently guarded groups explicit; the receiver never re-infers it | `spec/SPECIFICATION.md:6700`; `crates/happenstance-sync/src/peer.rs:185-190` | The message set must preserve group boundaries on the wire, not flatten them for encoding convenience |
| CF-29 — one `CHANGELOG.md` entry per new rule, naming a defect | `xtask/src/lints.rs:518-530` | Each WF/SY rule landed here owes an entry. HS-S0104 made the omission fail |

## Questions

**Does ingest re-check the writer's asserted append conditions? — Answered
elsewhere; what this story owes is that the wire keeps the answer testable.**
SY-6 requires a guard whose `after` is `Some(_)` to be refused as ingest input,
and it is checkable *because* `Guard` is `#[non_exhaustive]` with public fields
while `AppendCondition::guards` is private and readable only through `guards()` —
*"Readable-but-not-literal-constructible is precisely what makes this
checkable"*. WF-4 is the other half: `after` is **always present** on the wire,
never elided, *"because a policy can only refuse what it can see"*
(`spec/SPECIFICATION.md:6023-6029`). So the message set must not gain a
`skip_serializing_if` on that field, and no derive authorised by ADR-0027 may
introduce one.

**Hub-and-spoke versus peer-to-peer — not this story's**, though it constrains
the message set: the messages must be role-blind, because SY-9 makes hub-ness an
edge property and one adapter type serves both roles at once
(`spec/SPECIFICATION.md:6090-6106`). A message that names a role would put the
topology into the wire format.

**What is in the message set? — Answered by ADR-0027, executed here.** The data
flow already names the candidates end to end: `PushBatch` out, `Pulled { batch,
resume }` back, `Ack { appended, skipped, confirmed }` as the confirmation, and
`PeerLimits` read before pushing (`_decomposition.md`, *Data flow*). What is a
message versus a return value, and whether `PeerLimits` is negotiated or polled,
is ADR-0027's.

**Is `format_version` per-message or per-connection-negotiated? — Answered by
ADR-0027; this story implements whichever landed.** The open question atom names
the consequence of the alternative: per-connection negotiation *"would make the
field dead weight on every message and removing it a format break"*
(`.kb/open-questions/sync-message-set-and-format-version.md`). Whichever way it
goes, WF-8's before-any-decoding obligation stands and is discharged by the
hand-written `Deserialize`.

**Does landing the message set bump `FORMAT_VERSION`? — Deferred to spec, with
the test fixed.** The constant bumps on a **shape** change to a wire type
(`crates/happenstance-sync/src/wire.rs:46-58`). Adding new types that instantiate
`T` may or may not be one, and adding derives to three existing types plausibly
is. Nothing is published, so the cost of either answer is zero today and
non-zero after HS-P0016 — which is an argument for deciding it deliberately now
rather than discovering it later.

**Do the WF rules live in `happenstance-sync/tests/` or in the sync testkit? —
Deferred to spec, and the existing split is the precedent.** WF-8's and WF-11's
rules are `wire::`-qualified and already live in the sync crate's own `tests/`;
`spec-trace` keeps `WIRE_FILES` deliberately **outside** `RULE_FILES` and the
distinction is ADR-0016 §15's (`xtask/src/spec_trace.rs:91-96`). A rule about
*what a peer does* belongs in the testkit; a rule about *how bytes encode* does
not, and blurring the two would re-open a distinction already decided.

## Decision

`crates/happenstance-sync/src/wire.rs` ships a versioned envelope that is
deliberately generic in `T` and deliberately empty of message kinds, and
`FORMAT_VERSION = 1` therefore *"names a message set that does not exist yet"* —
a promise that some vocabulary is version 1 of itself, without saying what the
vocabulary contains. Three public message types sit beside it carrying no derives
at all, because the derives were withdrawn on the explicit ground that a derive on
a public message type *is* a wire format and the format that travels is phase
13's. This story lands the vocabulary: new types instantiating `Envelope<T>`,
never an enum wrapped around it, with each derive on `PushBatch`, `EventGroup` and
`ReplicatedEvent` authorised by name in ADR-0027, and the version disposition —
per-message or negotiated, and whether the constant moves — executed from the same
atom. It also lands the rules that make WF-8's before-any-decoding obligation and
WF-11's payload encoding falsifiable in **both** supported formats, each with the
wrong implementation the clause already names. The spec stage will cover: the
message types and their placement beside rather than inside the envelope; each
authorised derive and the absence of `skip_serializing_if` on `Guard::after`
(WF-4); the `FORMAT_VERSION` disposition; the witness-test shape for
`version_is_readable_before_the_message`; the two-format discipline for every new
rule; `WireError`'s scope and the fact that `SyncError` is untouched here; the
group boundaries surviving encoding (SY-30); and one `CHANGELOG.md` entry per
rule naming a defect.

## The wrong implementation

**`DerivedEnvelopeDeserialize` — and the reason WF-8's own rule is a witness test
is that this mutant is *invisible to byte inspection*.** Replace the hand-written
`Deserialize` with `#[derive(Deserialize)]` plus a post-hoc version check. It
round-trips every valid message correctly. It returns the *same* `Err` for an
unknown version in both formats. It produces **byte-identical postcard framing** —
`[01 07]` at version 1, `[80 03 07]` at 384, the same `take_from_bytes::<u16>`
value and the same `[07]` remainder — so neither the error nor the front of the
buffer distinguishes it from the correct impl
(`spec/SPECIFICATION.md:2192-2203`). What it does is decode the message body
*first* and check the version *after*, which is the partial decode WF-8 forbids:
a hostile or malformed body at an unrecognised version is fully parsed before
anyone has decided whether to speak that version. The only oracle that separates
them is a `Witness` type whose `Deserialize` records that it ran, asserting the
count is **zero**; the derive records one. It is measured, it is already
specified, and it must be compiled into the wire tests as a negative control.

**`MessageEnum` — turning `Envelope<T>` into `enum Message { Push(..), Pull(..)
}`.** Every test passes, the encoding is arguably tidier, and a single enum reads
as a cleaner API than a generic wrapper. It puts the message set beside the
version check, which is the exact coupling the generic envelope exists to prevent
(`crates/happenstance-sync/src/wire.rs:8-16`): once the kinds are enumerated
inside the versioned type, adding a message kind is a shape change to the
envelope, so every new message forces a `FORMAT_VERSION` bump and every peer that
has not been redeployed refuses every message from a peer that has. The failure
is not visible at this story's grain at all — it appears one release later, in a
heterogeneous deployment, as total mutual unreachability.

**`InvertedHumanReadable` — the payload mutant that passes both round trips.**
Swap the `is_human_readable` branch so `Event::data` renders as raw bytes in JSON
and as base64 ASCII in postcard. Encode-then-decode is a perfect identity in
both formats, so **every round-trip assertion is green**, and any test that
compares *decoded values* — `assert_eq!(sent.event, received.event)` — cannot see
it. WF-11 names it explicitly as *"invisible to WF-7's rules"* and pins the
measurement: `[de ad be ef]` must render as the JSON string `"3q2+7w=="` and as
four raw bytes in postcard, and the inverted implementation renders
`[222,173,190,239]` and the ASCII of `"3q2+7w=="` respectively
(`spec/SPECIFICATION.md:2315-2322`). This is the general form of the trap AC-006
is written against, arriving one story early: **a payload round trip asserted on a
decoded value rather than on bytes hides a codec that is not byte-identical.**
`byte-identical-round-trip-and-idempotent-replay` (HS-S0107) inherits the
consequence; this story is where the codec that could break it is introduced, so
the negative controls belong here.

**And the quiet one: `FlattenedPushEncoding`.** Encode `PushBatch` as a flat list
of `ReplicatedEvent` because groups add a nesting level and the receiver "can
regroup by guard". Round trips are equal, byte counts go down, and the group
boundary is gone — so the receiver re-infers a decomposition instead of being
told one, which SY-30 forbids and which no assertion over decoded *values* can
detect once the encoding has already lost the structure
(`crates/happenstance-sync/src/peer.rs:185-190`).

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

**Box 6.** The rules added here assert on encodings, not on positions. Where a
`SequencePosition` appears — inside an `EventId` on `ReplicatedEvent`, and inside
`Guard::after` on a wire-carried condition — it is constructed from a value the
*origin* store assigned and is compared for identity or round-trip equality,
never asserted against a literal. This matters more than usual here: a position
on the wire *is* a naked integer (`spec/SPECIFICATION.md:6008-6014`), so a
literal in an encoding test would read as innocuous and would silently encode the
belief that positions are portable. `lint-position-literals` covers the sync
testkit from HS-S0104; the wire tests are covered by the same discipline whether
or not the lint's file list reaches them, and spec must say which.

**Box 7.** This story edits no `[FROZEN]` clause. It implements WF-8, WF-9 and
SY-30 and it is the first code to test `[PROVISIONAL]` WF-11 against a real
message set. Every wire-format decision it makes — the derives, the version
disposition — is authorised by ADR-0027, which is written first and is this
story's direct `depends_on` edge; a derive not named in that atom does not land
here. If WF-11's provisional marker turns out to be falsified by what the message
set needs, that is a recorded finding routed to
`clause-arithmetic-and-deferral-renewals` (HS-S0113), not a clause edited in
passing.
