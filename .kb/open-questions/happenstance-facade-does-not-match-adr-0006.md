---
id: kb-open-question-facade-does-not-match-adr-0006-001
title: happenstance neither re-exports the contract crate nor feature-gates the adapters, as ADR-0006 said it would
kind: open_question
status: accepted
authority_tier: note
summary: >-
  kb-decision-0006 is accepted and holds that happenstance re-exports the contract and
  feature-gates the adapters; today it does neither in that stated form. crates/happenstance/src/lib.rs:242
  is pub use happenstance_core::*, a glob of items rather than a re-export of the crate - there is
  no happenstance::happenstance_core path, though the glob does carry core's own re-exports through,
  so happenstance::bytes::Bytes already resolves. And crates/happenstance/Cargo.toml carries no
  dependency on any adapter crate at all, so there is nothing to feature-gate. Neither gap is a
  regression from a decision that changed its mind; ADR-0006 is accepted and immutable, and no
  later record revisits either sentence. What is not decided is whether the two sentences describe
  a state this workspace still intends to reach, whether they were aspirational when written and
  never meant as a near-term commitment, or whether the facade's actual shape - a glob re-export
  and no adapter dependency - is the right one and ADR-0006's prose should be corrected by a
  superseding record. Forced by the first time an application tries to write generic code against
  both happenstance::Event and an adapter crate and cannot name happenstance_core without adding
  it to its own manifest, or by whichever phase next touches the typed layer's public surface.
depends_on: []
related:
  - kb-decision-0006
source_paths:
  - .kb/_intake/remediation-2026-09-04-briefs/adapter-driver-reexport-policy.md
  - .kb/decisions/0006-bare-name-to-the-typed-layer.md
  - crates/happenstance/src/lib.rs
  - crates/happenstance/Cargo.toml
last_reviewed: 2026-09-07
---

# `happenstance` neither re-exports the contract crate nor feature-gates the adapters, as ADR-0006 said it would

## What is true today

`kb-decision-0006` is accepted, immutable, and its summary states plainly what
the typed layer is supposed to do: `happenstance` "holds `Codec`,
`DomainEvent`, `DecisionModel` and the command loop, **re-exports the
contract**, **feature-gates the adapters**, and is what an application `cargo
add`s." That sentence is not incidental prose — it is the allocation the whole
ADR argues for, the reason the bare name went to the typed layer rather than
staying on the contract crate.

Checked against the tree, neither half holds in the form stated.

**The re-export is a glob of items, not the crate.** `grep -rn "pub use
happenstance_core"` over the workspace returns exactly one non-test hit:

```rust
pub use happenstance_core::*;
```
— `crates/happenstance/src/lib.rs:242`

`pub use happenstance_core;` — the form that would give a caller
`happenstance::happenstance_core::SomeType` and let them write generic code
against the contract crate's own path — appears in no crate. Four tests pin
the glob's exact text (`crates/happenstance/tests/doc_surface.rs:133,326`,
`doc_budget.rs:260`, `docs_composition.rs:197`), so this is not an oversight
nobody has looked at; it is the surface as deliberately shipped. One
consequence is already load-bearing: the glob carries core's own crate
re-exports through, so `happenstance::bytes::Bytes` resolves today and is used
across the test suite. What the glob does not give is a path to the crate
itself — a consumer who wants to write `fn ingest<S: EventStore>(store: S)`
over both `happenstance::Event` and an adapter's store type still has to add
`happenstance-core` to their own manifest and choose a version, which is
exactly the friction "re-exports the contract" reads as promising to remove.

**There is no adapter dependency to feature-gate.** `crates/happenstance/Cargo.toml`
declares no dependency on `happenstance-sqlite`, `happenstance-cloudflare`, or
any other adapter crate. "Feature-gates the adapters" describes a shape where
`happenstance` is the single crate an application adds and turns adapters on
or off with feature flags; today an application adds `happenstance` for the
typed layer and each adapter crate separately, with no feature relationship
between them at all.

## What is not decided

Whether ADR-0006's two sentences describe a destination this workspace still
intends to reach — in which case this is a scheduling gap, not a contradiction
— or whether they were the ADR's aspirational framing of what a typed facade
*could* eventually do, never meant as a phase-0 commitment, and the shipped
shape (a glob re-export, no adapter coupling) is the intended one. If the
latter, ADR-0006's summary is wrong about the present tense and needs a
superseding record to say so — an accepted decision atom cannot be edited to
match the code; the code, or a new atom, has to catch up to one or the other.

## What forces it

The first time an application genuinely needs both `happenstance`'s typed
surface and an adapter's store type in one generic function and discovers it
must add `happenstance-core` itself with no help from either crate's
manifest — which is the same underlying friction
`kb-open-question-adapter-version-lockstep-and-cf-32-001`'s sibling brief
raises from the adapter side. Or whichever phase next touches
`crates/happenstance/src/lib.rs`'s public surface or its `Cargo.toml`, since
either change is the natural moment to decide whether ADR-0006's prose is
still the target.

## Ordered sub-questions

1. Is `pub use happenstance_core::*;` considered a satisfying implementation
   of "re-exports the contract," or does the ADR's language mean the crate
   path specifically?
2. Was "feature-gates the adapters" ever scheduled against a phase, or is it
   still an open design choice for how `happenstance` should relate to the
   adapter crates?
3. If the shipped shape is correct and ADR-0006's prose is the thing that is
   wrong, who writes the superseding record, and does it touch only the
   summary or the allocation itself?
