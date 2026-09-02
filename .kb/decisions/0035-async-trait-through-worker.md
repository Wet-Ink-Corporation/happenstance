---
id: kb-decision-0035
title: async-trait is exempted where it is reached through worker
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0035
reversibility: medium
phase: 9
supersedes: null
superseded_by: null
summary: >-
  deny.toml's async-trait ban gains two wrappers, worker and worker-macros, amending
  ADR-0001's exemption set without touching ADR-0001's body — the shape ADR-0029 used
  against ADR-0004. The ban is a proxy for a property, not an end in itself: ADR-0001
  forbids #[async_trait] because it injects + Send and forecloses the wasm32/Workers
  target, and worker does not violate that property, using the macro for its own
  DurableObject trait, which this workspace implements without deriving any
  happenstance port from it. The two new wrappers rest on a different argument from
  the existing wasm-bindgen-test entry and the distinction is kept rather than
  collapsed: that one is a dev-dependency present in no published artifact, while
  worker is a normal dependency of happenstance-cloudflare and does ship. The
  load-bearing guard was never cargo deny, which is an optional probed gate step, but
  crates/happenstance/tests/flavours.rs, which instantiates every typed-layer entry
  point against a genuinely !Send store and stops compiling the moment a Send bound
  reaches the chain; the exemption removes the legible guard on two edges and leaves
  the unskippable one on all. Exempting by name is what keeps the ban real — a third
  route into the graph still fails the gate until someone decides it should not.
  Known gap, recorded rather than hidden: a wrappers entry pins a name, so a worker
  major bump changing how it uses async-trait would not be caught, which is a
  property of wrappers and has been true of the wasm-bindgen-test entry since the
  ban's first run. Resolves kb-open-question-worker-async-trait-ban-001. Decided at
  HS-P0013's gate on 2026-08-20, after two earlier passes named both shapes and
  declined to choose.
depends_on:
  - kb-decision-0001
related:
  - kb-decision-0023
  - kb-decision-0029
  - kb-open-question-worker-async-trait-ban-001
source_paths:
  - .kb/_intake/0035-async-trait-through-worker.md
  - references/adr/0035-async-trait-through-worker.md
  - deny.toml
  - crates/happenstance/tests/flavours.rs
  - xtask/src/main.rs
last_reviewed: 2026-09-02
---

# async-trait is exempted where it is reached through worker

## Decision

`deny.toml`'s `async-trait` ban gains two wrappers — `worker` and
`worker-macros` — ratified by ADR-0035. This amends `kb-decision-0001`'s exemption
set from outside; `kb-decision-0001`'s body is not edited and stays byte-identical,
the same shape `kb-decision-0029` used to raise the MSRV against `kb-decision-0004`
without touching its text. The full transcript, the rejected alternatives and the
`cargo deny` before/after output live in `references/adr/0035-async-trait-through-worker.md`;
this atom cites it rather than restating it.

The call was made explicitly at HS-P0013's gate on 2026-08-20, after two earlier
passes named both shapes — exempt, or don't — and declined to choose: HS-S0059
refused it as outside its own PR boundary, and the identical edit had already been
reverted once as finding-deletion; the phase-9 ingest wave refused it because
ratifying an exemption to a binding constraint against a red gate with no human
present is exactly what `.kb/decisions/README.md` forbids.

## Why the exemption holds

**The ban is a proxy, not an end in itself.** `kb-decision-0001` forbids
`#[async_trait]` because the macro injects `+ Send` into every method it desugars,
which forecloses the `wasm32` / Cloudflare Workers target where a store is `!Send`
by construction. `worker` does not violate that property: it uses `async-trait` for
its own `DurableObject` trait, and this workspace implements that trait without
deriving any `happenstance` port from it. No `Send` bound reaches a `happenstance`
port through this dependency.

**The two new wrappers argue differently from the existing entry, on purpose.**
`wasm-bindgen-test` was already exempt because it is a dev-dependency present in no
published artifact — nothing ships it. `worker` is a normal dependency of
`happenstance-cloudflare` and does ship. Reusing the dev-dependency argument for a
shipping one would be reasoning about a different case with the same words, so the
two exemptions are recorded on their own grounds rather than folded together.

**The load-bearing guard was never `cargo deny`.** `cargo deny` is an optional
probed gate step — it runs when the tool resolves, and skips when it does not.
`crates/happenstance/tests/flavours.rs` is not optional: it instantiates every
typed-layer entry point against a genuinely `!Send` store and stops compiling the
moment a `Send` bound reaches the chain, on every target, in every run. Exempting
`worker` and `worker-macros` removes the legible, human-readable guard on those two
edges and leaves the unskippable compile-time one standing on all of them —
`reversibility: medium` rather than `low`, because reversing this exemption is a
one-line `deny.toml` edit but the property it protects is still checked by a second,
independent mechanism regardless of which way this decision goes.

**Exempting by name, rather than by property, is what keeps the ban real.** A third
crate reaching `async-trait` through some other route still fails `cargo deny bans`
until someone names it and argues the same case again.

## Known gap

A `wrappers` entry pins a crate name, not a property. A `worker` major version that
changed how it uses `async-trait` internally — or stopped using it — would not be
caught by this exemption; the entry would keep matching on name alone. This is not a
defect specific to this decision: the `wasm-bindgen-test` entry has carried the same
gap since the ban's first run, and it is a property of how `deny.toml`'s `wrappers`
mechanism works rather than a property of this exemption.

## What this does not decide

This does not adjudicate the three store-limit numbers HS-P0013's run-2 review found
read off a platform page, which HS-S0055 AC-001 forbids in terms; no document has
resolved that, `kb-decision-0023` did not, and this decision does not either. It
surfaces in HS-S0055's own record.

## Resolves

`kb-open-question-worker-async-trait-ban-001`, minted by wave `2026-08-20-intake-phase-9`
precisely because neither shape had been chosen yet.

## Alternatives rejected

Editing `kb-decision-0001`'s body directly to add the exemption: rejected because it
is accepted and immutable, and the only legal shapes for changing a decision's
content are a new atom or a superseding one. Deferring the call again as a third
open question: rejected because the maintainer made the call explicitly at
HS-P0013's gate, and a decision that has been taken belongs in a decision atom, not
back in the open-question layer it came from.
