---
id: kb-decision-0016
title: The wire format is happenstance's own, and an unknown version is refused before the message is read
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0016
reversibility: high
phase: 5
supersedes: null
superseded_by: null
summary: >-
  The format is private to happenstance with no compatibility obligation to the DCB reference,
  which publishes no wire format at all, and that is what makes every reversal in it free
  rather than breaking. Presence obligations bind the encoder only, never the decoder: an
  Option field missing on the wire still decodes to None through serde's ordinary
  missing_field mechanism with no attribute needed, and that asymmetry is kept because it fails
  closed — a missing after checks the whole log rather than silently narrowing it. Query
  becomes an externally tagged two-variant enum (All, Items) rather than riding Option's
  transparent encoding, because Some(Query::all()) and None both serialize to the same bytes
  under Option's transparent treatment and the two are indistinguishable on the wire today,
  measured. Two frozen field names are corrected to track types the code had already moved to:
  the append condition's boundary field is spelled after inside Guard rather than
  fail_if_events_match, and EventId's store field is spelled store rather than origin,
  mirroring the accessor an earlier decision already gave it. happenstance-sync gains a
  minimal generic envelope, Envelope<T> { format_version, message }, with a hand-written
  Deserialize rather than a derived one, because serde's derive reads every field into a local
  before constructing the struct and offers no hook to refuse an unknown format version before
  the message itself is decoded — measured directly: a derived envelope with a version check
  added after decoding is indistinguishable from the hand-written one by every rule tried
  except a witness type that records whether its own Deserialize ran at all. base64 is added
  as an optional dependency reachable only through the existing serde feature, at zero new
  crates otherwise, encoding Event::data and Event::metadata as standard base64 in
  human-readable formats and raw bytes otherwise. ReadOptions loses Serialize and Deserialize,
  enforced by a const-evaluation type-level assertion rather than by a compile_fail doctest,
  because the doctest was measured to pass under a misspelled type name, a misspelled trait,
  and a wrong crate path alike — three false negatives out of four spellings tried against the
  same claim.
depends_on:
  - kb-decision-0003
  - kb-decision-0012
related:
  - kb-decision-0014
  - kb-reference-wire-format-measurements-001
source_paths:
  - .kb/_intake/0016-the-wire-format.md
  - references/adr/0016-the-wire-format.md
  - crates/happenstance-sync/src/wire.rs
  - crates/happenstance-core/src/event.rs
  - experiments/wire-format/
  - spec/SPECIFICATION.md
last_reviewed: 2026-08-10
---

# The wire format is happenstance's own, and an unknown version is refused before the message is read

## Decision

The wire format belongs to happenstance alone. The DCB reference specification it otherwise
tracks publishes no wire format of its own, so nothing here is a compatibility break with an
external protocol — every correction this decision makes is free precisely because there was
never an interoperability promise to keep. That framing settles a scope question the format's
original clause left open and lets every other correction below be made without a version bump
being owed to anyone outside this workspace.

Two frozen clauses named fields the code had already stopped using, because two earlier phase-4
decisions moved the underlying types without a wire decision tracking the move: the append
condition's boundary is now `after` inside a `Guard { query, after }` value rather than the
`fail_if_events_match` name a frozen clause still froze, and `EventId`'s origin field is `store`
in the code and in its own accessor, not the `origin` a frozen clause names. Both are corrected
by naming the noun the code already carries, not by re-arguing the decision that moved it.

`Query` moves from `Option<Query>`-style encoding to an externally tagged two-variant enum,
`All` and `Items`, reversing what an existing frozen clause's wording implied. The reversal is
forced by a measured defect: because serde's `Option` encoding is transparent — `Some(T)`
produces exactly the bytes `T` would produce on its own — `Some(Query::all())` and
`None::<Query>` serialize to identical bytes, so a `Some(All)` value silently fails to survive a
round trip. The externally tagged encoding is distinguishable from `None` in both JSON and
postcard, measured directly against both formats.

Every presence obligation this decision states binds the **encoder only**. An `Option`-typed
field left absent by a foreign or older encoder still decodes to `None` through serde's default
`missing_field` handling, with no attribute required on either side — closing that gap on the
decoder side would need a hand-written visitor at every optional field, for no format any
current implementation actually produces. The asymmetry is kept because it fails in the safe
direction: a missing `after` decodes to `None`, and `None` checks the entire log rather than a
narrowed slice, so the failure mode is a spurious rejection rather than a spurious acceptance.

`happenstance-sync` gains a minimal generic envelope, `Envelope<T> { format_version: u16,
message: T }`, carrying a **hand-written** `Deserialize` rather than a derived one. The reason
is structural rather than stylistic: a derived `Deserialize` visitor reads every field into a
local before the struct is ever constructed, so a derived envelope necessarily decodes the
message before anything can examine the version — precisely the partial decode the clause this
implements forbids. Measured directly: a derived envelope with a version check bolted on
afterward is byte-identical on the wire to the hand-written one and passes every rule tried
against it except one — a witness type whose own `Deserialize` records whether it ran, which
fires once under the derive and zero times under the hand-written form. `SyncError` is not
extended for this; a version refusal is a decoding failure raised through the envelope's own
error type, not a runner failure.

`base64` is added as an optional dependency of `happenstance-core`, reachable only through the
existing `serde` feature and adding no new crate to any build that does not already enable it.
`Event::data` and `Event::metadata` encode as standard-alphabet base64 in human-readable formats
and as raw bytes otherwise, branching on serde's `is_human_readable`.

`ReadOptions` loses `Serialize`/`Deserialize` outright — a store-local position and traversal
options a peer has no business setting do not belong on the wire — but the instrument that was
supposed to guard the property is replaced. A `compile_fail` doctest was measured against four
deliberately broken spellings of the same assertion and passed three of them, including a
misspelled type name and a wrong crate path, because a `compile_fail` fence only checks that
compilation failed for *some* reason. The replacement is a `const`-evaluation type-level
assertion using an inherent-versus-trait associated-constant resolution trick, verified to
produce a genuine build failure — not a green test — under every one of the same broken
spellings.

## Provisional

The human-readable payload encoding stays open in its buffering half: base64's `no_std`
question closed at measurement, but whether a peer can encode a large payload without buffering
the whole thing is a property of serde's data model rather than of base64 specifically, and its
falsifier belongs to the phase-9 Workers peer, which has the tightest memory budget in the
workspace.

## Alternatives rejected

Per-type versioning inside each message type was rejected as this decision's own clause already
argues: it costs bytes on every event in the log and still cannot express a change to the
*relationship* between two types, which is exactly the shape of the change that moved the
append condition's boundary field in the first place.
