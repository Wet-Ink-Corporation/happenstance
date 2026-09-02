---
id: kb-open-question-worker-async-trait-ban-001
title: The worker dependency re-opens deny.toml's async-trait ban, and neither shape is chosen
kind: open_question
status: accepted
authority_tier: note
summary: >-
  Taking the real worker crate in place of the hand-written stand-in was reversed on a
  measurement rather than on taste, and its price is recorded rather than hidden: the graph
  gains about 40 crates, cargo deny check licenses advisories stays green, and cargo deny check
  bans is red. deny.toml bans async-trait under ADR-0001, which chose trait_variant over it
  because #[async_trait] injects + Send and forecloses wasm32; worker 0.8.5 and worker-macros
  depend on async-trait unconditionally. What is true today is that deny.toml's wrappers list
  names exactly one crate, wasm-bindgen-test, reached only as a dev-dependency of the
  conformance harnesses and present in no published artifact — and the file's own comment states
  that a second route into the graph is a new wrapper the list does not carry and that the check
  fails until someone decides it should. Taking worker as a production dependency is that second
  route. What the red ban does not mean, since it invites the wrong inference: worker uses
  #[async_trait] for its own DurableObject trait, no happenstance port gains a Send bound from
  it, and happenstance-core still declares each port once without a Send bound and lets
  trait_variant derive the second flavour. What is not decided is which of two shapes is taken —
  ratify a wrappers entry naming worker and worker-macros with the argument written into
  deny.toml beside it, or refuse and let the ban stay red with the exception recorded. Either is
  a decision; leaving it undecided while the gate is red is not. Forced now: publish-ready-crate
  cannot claim a green gate while the ban is red, and its AC-012 and its project's DoD both say
  so.
depends_on: []
related:
  - kb-decision-0001
  - kb-decision-0029
  - kb-decision-0023
source_paths:
  - .kb/_intake/0023-the-sqlstorage-mapping-and-the-off-tokio-harness.md
  - references/adr/0023-the-sqlstorage-mapping-and-the-off-tokio-harness.md
  - deny.toml
  - xtask/src/main.rs
last_reviewed: 2026-08-20
---

# The worker dependency re-opens deny.toml's async-trait ban, and neither shape is chosen

## What is true today

`happenstance-cloudflare` took a dependency on the real `worker` crate (0.8.5) in place
of the hand-written stand-in it carried for two phases, and ADR-0023 records this as a
reversal made on measurement rather than on taste — the stand-in's four modelled
properties (synchronous `exec`, a non-snapshot cursor, `!Send`/`!Sync` bindings, a
re-entrant single-threaded object) needed checking against real bindings, and only the
real crate supplies them. The reversal's price is recorded rather than hidden:
`cargo tree` shows the graph gains roughly 40 crates, `cargo deny check licenses
advisories` stays green, and `cargo deny check bans` turns **red**.

The cause is `deny.toml`'s ban on `async-trait`, put there under ADR-0001, which chose
`trait_variant` precisely because `#[async_trait]` injects a `+ Send` bound and
forecloses the `wasm32` target this crate exists to support. `worker` 0.8.5 and
`worker-macros` depend on `async-trait` unconditionally — not behind a feature flag —
because `worker` uses it for its own `DurableObject` trait.

`deny.toml`'s `wrappers` list, the mechanism that permits a named crate to carry a
banned dependency without failing the check, names exactly one entry: `wasm-bindgen-test`,
reached only as a dev-dependency of the conformance harnesses and present in no
published artifact. The file's own comment already states that a second route into the
graph is a new wrapper the list does not carry, and that the check fails until someone
decides it should. Taking `worker` as a **production** dependency of
`happenstance-cloudflare` is that second route, and it is not covered.

What the red ban does **not** mean, since a red check invites the wrong inference: no
`happenstance` port gains a `Send` bound from this. `worker`'s use of `#[async_trait]`
is confined to its own `DurableObject` trait, which this crate implements but does not
re-export as a bound on anything `happenstance-core` declares. `happenstance-core`
still declares `EventStore` once, without `Send`, and lets `trait_variant` derive the
`SendEventStore` flavour — ADR-0001's design is untouched by this dependency.

## What is not decided

Which of two available shapes is taken. **Ratify**: add a `wrappers` entry naming
`worker` and `worker-macros`, with the argument above — that no port gains a bound
from it — written into `deny.toml` beside the entry, so the exemption carries its own
justification where the next reader will find it. **Refuse**: leave the ban red, with
the exception recorded here and in the crate's own documentation, accepting that
`cargo deny check bans` cannot be green for this workspace as long as `worker` is a
dependency. Either is a decision the wave can point to; leaving the ban red with
nothing written down is not — it reads as an oversight rather than a considered choice.

## What forces it

`publish-ready-crate` (or whichever story carries `happenstance-cloudflare` toward its
own AC-012) cannot claim a green gate while `cargo deny check bans` is red, and the
project's Definition of Done says the same. `cargo xtask ci` runs `cargo deny` whenever
the tool resolves on the machine, which it does here, so this is not a step that can
quietly skip — a tool that runs and finds a problem always fails the gate.

## Ordered sub-questions

1. Ratify or refuse — and if ratify, does the `wrappers` entry cover `worker-macros`
   separately or is one entry sufficient for both?
2. If refused, does the red `cargo deny check bans` result get carved out as a named,
   documented exception in the CI configuration itself, or does the gate stay
   genuinely red for this crate until the dependency is removed or `async-trait`
   is dropped upstream?
3. Does this choice, once made, become a precedent the fixture-contract-ownership
   pattern in `kb-decision-0034` would recognise — a decision minted by the adapter
   that first needs it, rather than requiring a new umbrella ADR over dependency
   exceptions generally?
