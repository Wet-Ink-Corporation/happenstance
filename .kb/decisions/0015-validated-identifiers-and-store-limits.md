---
id: kb-decision-0015
title: Validated identifiers, byte equality, and the two kinds of bound
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0015
reversibility: low
phase: 4
supersedes: null
superseded_by: null
summary: >-
  One const fn walks the bytes and validates both EventType::new and EventType::from_static, so
  there is one rule rather than two; an exhaustive check over all 1,112,064 Unicode scalar
  values shows the byte walk sees the C1 controls and the seven bidirectional formatting
  controls a naive ASCII-only narrowing would have missed, because UTF-8 is self-synchronising
  and a walk that looks at a lead byte and its successor sees every multi-byte offender. Both
  EventType and Tag move from Box<str> to Cow<'static, str> and gain a const from_static, so an
  identifier written in source and baked into the binary and one that arrived from a peer at
  runtime share one type; Eq, Hash and Ord are hand-written rather than derived, because a
  derive cannot make Eq and Hash disagree with each other while a hand-written pair can, and
  writing them by hand makes the Borrow<str> consistency obligation an explicit, checked promise
  rather than an accident of today's single-field shape. Equality is byte equality over UTF-8
  with no normalisation, case folding or trimming. The load-bearing distinction is between two
  kinds of bound: a validity invariant is enforced by the constructor and re-enforced on
  deserialisation, because a violating value must be unrepresentable, while a capacity limit
  must never be enforced by a constructor or in Deserialize, because rejecting at decode
  destroys the quarantine path a peer needs — an event refused at decode has no local position,
  cannot be named, and cannot be forwarded or reported, so it silently disappears rather than
  being parked for a human. Four minimum floors become public constants in happenstance-core,
  never ceilings, all still provisional and two of the four falsifiers unreachable by any
  scheduled phase because they need a real domain event nobody has written yet.
  AppendError gains ExceedsStoreLimit with a StoreLimit enum of three variants (not four; a
  query-item ceiling is not an append outcome and routes to the ingest policy seam instead).
  CF-40 is minted so a fixture can declare its store's actual numeric limits as Option<usize>
  associated constants, because without a way to state a number the rule that checks the new
  error variant is unwritable and the frozen clause requiring it ships implemented and checked
  by nothing.
depends_on:
  - kb-decision-0003
related:
  - kb-decision-0012
  - kb-playbook-repair-frozen-clause-001
source_paths:
  - .kb/_intake/0015-validated-identifiers-and-store-limits.md
  - references/adr/0015-validated-identifiers-and-store-limits.md
  - crates/happenstance-core/src/tag.rs
  - crates/happenstance-core/src/validate.rs
  - crates/happenstance-core/src/limits.rs
  - crates/happenstance-core/src/error.rs
  - spec/SPECIFICATION.md
last_reviewed: 2026-08-10
---

# Validated identifiers, byte equality, and the two kinds of bound

## Decision

`EventType::new` and `Tag::new` share a single `const fn` byte-walk validator, and
`from_static` calls the same function — one rule rather than a pair that could quietly drift
apart. The walk rejects C0 controls and DEL directly, and reaches the C1 range (encoded as
`C2 80`–`C2 9F`) and the seven bidirectional formatting overrides (`E2 80 AA`–`E2 80 AE`,
`E2 81 A6`–`E2 81 A9`) by looking at a lead byte together with its successor, which UTF-8's
self-synchronising encoding makes sufficient. An exhaustive test over every one of the
1,112,064 Unicode scalar values, both standalone and embedded between ASCII letters, found zero
disagreements against `char::is_control` plus the closed bidirectional list — so the existing
frozen `Cc`-rejection clause stands unweakened, and `from_static` is exactly as strong as `new`
rather than a documented-weaker escape hatch.

Both `EventType` and `Tag` change their backing field from `Box<str>` to `Cow<'static, str>`
and gain a `const fn from_static`, which validates by `assert!` inside a `const` context. `Cow`
is the only representation that lets one type express both a compile-time identifier baked into
the binary and a runtime one that arrived over the wire and had to be allocated; a bare
`&'static str` can only do the first. `Eq`, `Hash` and `Ord` are hand-written rather than
derived, not because the derived versions would be wrong today — `Cow`'s own implementations
already delegate to `str`, so they would agree — but because a derive on today's single-field
shape offers no place to stand when a second field lands, and a hand-written pair is where the
`Borrow<str>` consistency promise — that a borrowed form hashes and compares identically to the
owner — becomes an explicit, assertable obligation rather than a fact nobody checks.

The central distinction the whole decision turns on: a **validity invariant** is unrepresentable
by construction, enforced by both the constructor and `Deserialize`. A **capacity limit** is the
opposite — it must never be enforced by a constructor and must never be enforced in
`Deserialize`. The reason is the quarantine path a replication peer depends on: a peer that
refuses an over-limit value at decode time cannot decode the event at all, so it cannot name it,
report which event was refused, or park it for a human to look at — it simply disappears, with
no local position and no trace on any later peer's view. A limit enforced only at the store
boundary produces a value the peer can hold, name, and quarantine.

Four floors — event data length (65,536 bytes), tags per event (64), query items (128), and
events per batch (128) — become public constants in `happenstance-core`, stated explicitly as
floors and never ceilings: a single ceiling would either straitjacket a large adapter or lie to
a small one, and it could never be revised once every consuming application depended on it. All
four stay `[PROVISIONAL]`; two of the four falsifiers cannot be reached by any scheduled phase
at all, because they require a real domain event exceeding the floor and none of the six
reference scenarios comes close — that state is recorded as "adoption-gated" rather than left to
look like a scheduled experiment.

`AppendError::ExceedsStoreLimit { limit: StoreLimit, len: usize }` lands with exactly three
`StoreLimit` variants, deliberately not four: a query-item ceiling is not an outcome `append`
can ever produce, so including it would create a variant no implementation could construct.
Checking the new variant needs a fixture that can state a real ceiling, which nothing in the
existing fixture contract could do — so CF-40 is minted, letting a `Fixture` declare
`Option<usize>` limits per dimension, defaulting to `None` and reporting through the same
declined-capability path a boolean capability already uses.

## Alternatives rejected

Ban both C1 and every Unicode format-control (`Cf`) category was rejected as unaffordable — no
general-category predicate exists in `core` without a Unicode-tables dependency far larger than
anything else in the contract crate; the seven-codepoint closed list catches the specific attack
vector at zero dependency cost, and the residual invisible-character hazard is documented rather
than hidden. Placing the fixture's new numeric-limit obligation under the existing mid-batch
fault capability was rejected because that capability is boolean by design and a limit is a
number a rule must be able to compare against.
