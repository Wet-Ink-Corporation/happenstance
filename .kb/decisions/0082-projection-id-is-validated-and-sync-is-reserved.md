---
id: kb-decision-0082
title: ProjectionId is validated — VT-14's set, a 255-byte bound, and happenstance/ and sync/ reserved
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0082
reversibility: low
phase: 17
supersedes: null
superseded_by: null
summary: >-
  Answers kb-open-question-projection-id-unvalidated-001 and closes it, on the owner's default
  (refuse the full ADR-0015 set, with a generic reserved prefix and SY-31's sync/) and the two
  Weigh-In calls wi-2155ac and wi-279dbb. D1: ProjectionId::new returns Result<ProjectionId,
  InvalidProjectionId>, and no infallible way in remains: no From<&str>, no From<String>, no hidden
  unchecked constructor. A const from_static enforces the same rules through the same private
  validator and panics at a free const as a compile error. D2: the rules are VT-14's (empty, more
  than MAX_PROJECTION_ID_LEN = 255 bytes, a Cc character or one of the explicit bidirectional
  controls, reported in that order with the character class decided left to right), then the
  reserved prefixes happenstance/ and sync/, matched as exact bytes with no folding or trimming.
  An accepted id is byte-identical to its input. D3: ProjectionId::sync_watermark(StoreId) is the
  only constructor of a sync/ id, rendering sync/ and the peer's 32 lowercase hex digits; the
  format is frozen. D4: InvalidProjectionId is a new non_exhaustive error type beside the port's
  own errors, with From<Infallible>. D5: phase 18's derived checkpoint id is bound by the same
  255 bytes, so a derived-id runner accepts a projection name of at most 238 bytes and refuses a
  longer one at derivation, never truncating or hashing it; its separator is one printable ASCII
  byte occurring in no reserved prefix, recommended @; and it is built through ProjectionId::new,
  because the proof's premise is a name new accepts, which a valid sync_watermark id is not. D6: VT-35 is minted FROZEN; PS-39, the
  store's half (a checkpoint is keyed on the id's exact bytes), is minted PROVISIONAL with
  freeze-by-17b, falsified by a backing store whose key column cannot hold a 255-byte UTF-8 id
  byte-faithfully. Partly supersedes ADR-0015 at section 10 only, where it declined to validate
  ProjectionId; kb-decision-0015 stays accepted. A published break in happenstance-core and
  happenstance, paid in the 0.4.0 window.
depends_on:
  - kb-decision-0015
  - kb-decision-0066
related:
  - kb-open-question-projection-id-unvalidated-001
  - kb-decision-0015
  - kb-decision-0074
  - kb-decision-0063
source_paths:
  - crates/happenstance-core/src/projection.rs
  - spec/SPECIFICATION.md
  - crates/happenstance-testkit/src/projection.rs
last_reviewed: 2026-10-07
---

# ProjectionId is validated — VT-14's set, a 255-byte bound, and happenstance/ and sync/ reserved

The long-form record, with the call-site census, the D5 proof written out and the mutant table, is
[`references/adr/0082-projection-id-is-validated-and-sync-is-reserved.md`](../../references/adr/0082-projection-id-is-validated-and-sync-is-reserved.md).

## Context

ADR-0015 validated every identifier in `happenstance-core` except one. At §10 it looked at
`ProjectionId` and declined, for two reasons. Both are gone.

1. **There was no instrument.** `ProjectionStore` had no conformance suite, so a validating
   constructor would have been checked by nothing. The suite now has seventeen rules and a mutant
   registry, ADR-0063 froze the port, and a store-side rule can be written and failed.
2. **A second constructor would have been worse than none.** A fallible `parse` beside an
   infallible `new` reproduces defect D2, the VT-16 / VT-26 hole: two doors with different rules,
   and an invalid value reachable through the weaker one. That reason still binds, so the answer
   is not a second door but replacing the only one.

The open question named its forcing event as *the first projection store whose backing table
rejects a key the constructor accepts*. It had already fired: Postgres and Neon `text` refuse
U+0000 at `commit`, and a Postgres btree entry tops out near 2,704 bytes.

## Decision

**D1 — `new` is fallible, and it is the only checking door.** `ProjectionId::new` returns
`Result<ProjectionId, InvalidProjectionId>`. `TryFrom<&str>`, `TryFrom<String>` and `FromStr`
forward to it. There is no `From<&str>` or `From<String>`: an infallible conversion would reopen
D2. `from_static(&'static str)` is `const` and calls the same private validator; a refusal is a
panic, which at a free `const` is a compile error.

**D2 — what is refused.** In this order: an empty value; one longer than
`MAX_PROJECTION_ID_LEN` (255) bytes; a character in Unicode `Cc` or one of the explicit
bidirectional controls (U+202A–U+202E, U+2066–U+2069), whichever comes first reading left to
right, because VT-14's walk is one pass; and a value beginning with a reserved prefix. The reserved
set is `["happenstance/", "sync/"]` (`wi-2155ac`), matched as an exact byte prefix: no case
folding, no trimming, no normalisation. `Cf` in general is accepted. An accepted value is kept
byte for byte.

`sync/` belongs to SY-31's replication watermark. `happenstance/` is held for ids this library may
mint later, so that minting one is not a break. Widening the list later refuses ids that are valid
today, which is why the generic namespace is reserved now.

**D3 — `sync_watermark(StoreId)` is the only way to a `sync/` id** (`wi-279dbb`). It renders
`sync/` followed by the peer's `StoreId` `Display`, 32 lowercase hex digits, 37 bytes in all, and
is infallible because its output is closed. The format is part of VT-35 and frozen: changing it
orphans every persisted watermark. A restored peer re-mints its `StoreId` and gets a new
watermark, which is right, because a new incarnation is new history.

**D4 — a new error type.** `InvalidProjectionId` is `#[non_exhaustive]` with `Empty`,
`TooLong { len }`, `ControlCharacter`, `BidirectionalControl` and `Reserved { prefix }`, and lives
in `projection.rs` beside `CommitError` and `ResetError`. It implements `From<Infallible>`, so
phase 18 can accept `impl TryInto<ProjectionId, Error: Into<InvalidProjectionId>>` with no second
error type. No variant is added to the private `validate::Refusal`, which is `EventType`'s and
`Tag`'s too.

**D5 — phase 18's derived id.** Recorded here because the bound is a published constant fixed
now; phase 18 implements it.

- **One bound.** `MAX_PROJECTION_ID_LEN = 255` applies to every `ProjectionId`, derived or not.
- **The derived id** is `name ++ SEP ++ 16 lowercase hex`. `name` is the projection's own
  `ProjectionId` (today `Projection::id()`). The suffix is exactly 17 bytes. Phase 18 MUST build it
  by passing the concatenation to `ProjectionId::new` and surfacing a refusal as its typed error:
  being a valid `ProjectionId` is not enough, because a `sync_watermark` id is valid and begins
  with `sync/`.
- **The cap rule.** A derived-id runner accepts a name of at most `MAX_PROJECTION_ID_LEN - 17` =
  **238 bytes**. It refuses a longer one with a typed error at the point of derivation. It never
  truncates the name and never hashes it. How the refusal surfaces (a fallible `checkpoint_id`, or
  a name type with its own bound) is phase 18's choice. No second public constant is minted now; a
  `MAX_PROJECTION_NAME_LEN` would be phase 18's additive change.
- **The separator rule.** `SEP` is one printable ASCII byte that occurs in no `RESERVED` prefix,
  so it is not `/` and not any letter of `sync` or `happenstance`. **Proof:** if
  `name ++ SEP ++ hex` began with a reserved prefix `p`, either `p` is a prefix of `name`, which is
  impossible because `name` is accepted by `ProjectionId::new`, or `p` contains `SEP`, which the
  rule excludes. Deriving through `new` is what makes that premise checked rather than assumed. Recommended:
  `@`, read as "at digest". `#` lost because it is a URL fragment delimiter and
  `tickets-over-http` exposes ids over HTTP. `:` lost because it is the tag `key:value`
  convention, so mixing it into ids invites confusion. Phase 18 pins the byte with its golden
  values; changing it later forces a rebuild everywhere.
- **Validity.** `name` is accepted by `ProjectionId::new`, `SEP` is printable ASCII, and the hex
  is ASCII, so the derived id passes VT-35 whenever `len(name) ≤ 238`.

**D6 — maturity.** VT-35 is `[FROZEN]` by this record. PS-39, the store's half, is
`[PROVISIONAL]` with freeze-by-17b: it is green against memory, SQLite and a live Postgres (both
projection stores) at phase 17, and freezes once live Neon has run it green. It is falsified by a
backing store whose key column cannot hold a `MAX_PROJECTION_ID_LEN`-byte UTF-8 key
byte-faithfully and cannot be configured to.

## Consequences

- **A published break.** `ProjectionId::new`'s return type changes in `happenstance-core` and in
  `happenstance`, which re-exports it. Paid in the `0.4.0` window, with a CHANGELOG entry.
- **Stranded rows.** A checkpoint row written at 0.3.x under an id that is now invalid (empty,
  over 255 bytes, a control character, or a `sync/` or `happenstance/` prefix) can no longer be
  named. It stays in the table, unreachable. No automated migration ships; the CHANGELOG gives the
  operator's `UPDATE`. Postgres and Neon could never have stored a NUL id in any case. Whether to
  ship a one-shot migration instead is an owner decision recorded as open in the lane's report.
- **The testkit's minor.** PS-39's rule `projection_ids_round_trip_by_bytes` joins the projection
  family, which can turn a passing adapter red (CF-32), so it rides the testkit's `0.4.0`.
- **A comment becomes true.** The Postgres and Neon migrations say *the contract validates the
  identifier*. They are published SQL and are left byte-identical; the sentence is now accurate.

## Rejected alternatives

- **A hidden `new_reserved` or `new_unchecked`.** VT-32's `Rejects:` names exactly that shape. A
  `#[doc(hidden)]` function is still callable, so the reservation would be advisory.
- **Reserve only `happenstance/` and move the watermark to `happenstance/sync/<peer>`.** SY-31 is
  provisional and could have moved, but the owner's default names `sync/`.
- **A variant on `validate::Refusal`.** It would make `EventType` and `Tag` change for a rule that
  is not theirs.
- **A lower bound.** 255 matches `MAX_EVENT_TYPE_LEN` and `MAX_TAG_LEN`, and D5 shows phase 18's
  derived id fits under it with a 238-byte name.
- **Case-insensitive matching of the prefixes.** VT-15 compares identifiers by bytes. `Sync/x` is a
  different id from `sync/x`, and refusing it would be a second, unstated equality.

## Supersession

Partly supersedes ADR-0015 at **§10 only**, where it declined to validate `ProjectionId`. Nothing
else in ADR-0015 is touched, so `kb-decision-0015` stays `accepted` and its decision-map row gains
the annotation, the ADR-0076 → ADR-0066 shape.
