---
id: kb-open-question-worker-async-trait-ban-001
title: The worker dependency re-opens deny.toml's async-trait ban, and neither shape is chosen
kind: open_question
status: superseded
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
  so. Resolved 2026-09-02 by ADR-0035 (kb-decision-0035), which takes the ratify shape:
  deny.toml's wrappers list gains worker and worker-macros, with the argument written into the
  file beside the entry, amending ADR-0001's exemption set without touching ADR-0001's body. The
  two new wrappers are kept distinct from the wasm-bindgen-test entry rather than collapsed into
  it, because that one is a dev-dependency in no published artifact and worker is a normal
  dependency that ships. Sub-question 1 is answered ratify, and one wrappers list carries both
  names. Sub-question 2 is not reached and stops being this atom's, the ban no longer being red.
  Sub-question 3 is answered yes: the exemption was minted by the adapter that first needed it,
  which is the pattern kb-decision-0034 records, and no umbrella ADR over dependency exceptions
  was required. No accepted decision was edited to produce any of it.
depends_on: []
related:
  - kb-decision-0001
  - kb-decision-0029
  - kb-decision-0023
  - kb-decision-0035
source_paths:
  - .kb/_intake/0023-the-sqlstorage-mapping-and-the-off-tokio-harness.md
  - references/adr/0023-the-sqlstorage-mapping-and-the-off-tokio-harness.md
  - deny.toml
  - xtask/src/main.rs
  - .kb/_intake/0035-async-trait-through-worker.md
  - references/adr/0035-async-trait-through-worker.md
last_reviewed: 2026-09-02
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

## Resolved 2026-09-02 — status `superseded`; the ratify shape was taken

Everything above is the state of knowledge on 2026-08-20 and is left exactly as it was
written: it was true when written, and the value of the record is that it shows what was
not known on the day the gate went red. The answering atom is `kb-decision-0035`,
"`async-trait` is exempted where it is reached through `worker`"
(`.kb/decisions/0035-async-trait-through-worker.md`), whose long-form record is
`references/adr/0035-async-trait-through-worker.md`. Nothing in it reaches back into
ADR-0001: that decision is accepted, its exemption set is amended from outside, and its
body stays byte-identical — the shape `kb-decision-0029` used against `kb-decision-0004`.

**Sub-question 1 is answered `ratify`, and one `wrappers` list carries both names.**
`deny.toml`'s single `deny` entry for `async-trait` now lists three wrappers —
`wasm-bindgen-test`, `worker`, `worker-macros` (`deny.toml:74-80`) — so the second half of
the sub-question resolves the way the mechanism forces rather than the way taste would:
`wrappers` matches by crate name, `worker-macros` is a distinct node in the graph, and one
name could not have covered both. What the decision refused to do is collapse the *reason*.
`wasm-bindgen-test` is exempt because it is a dev-dependency of the conformance harnesses
appearing in no published artifact; `worker` is a normal dependency of
`happenstance-cloudflare` and does ship, and is exempt on the different argument this atom
already set out above — it uses the macro for its own `DurableObject` trait, which the
workspace implements without deriving any `happenstance` port from it. Both arguments are
written into `deny.toml` beside the entry (`deny.toml:49-73`), which is what the ratify
shape asked for, and the entry's `reason` string names both ADRs.

**Sub-question 2 is not reached, and stops being this atom's.** It was conditional on
refusing, and refusal is not what happened: `cargo deny check bans` reports `bans ok`, so
there is no red result to carve out and no named CI exception to design. HS-S0059's AC-012
became claimable as written rather than needing its acceptance sentence reworded around a
red step. Two things the decision records rather than hides survive the resolution and
belong to `kb-decision-0035` rather than here: `cargo deny` was never the load-bearing
guard — `crates/happenstance/tests/flavours.rs` is, being a compile-time obligation on
every target where the `deny` step is optional and probed — and a `wrappers` entry pins a
*name*, so a `worker` major bump that changed how it uses `async-trait` would not be
caught. That gap is a property of `wrappers`, and the `wasm-bindgen-test` entry has carried
it since the ban's first run.

**Sub-question 3 is answered yes.** The exemption was minted by the adapter that first
needed it — `happenstance-cloudflare`, at phase 9 — with the argument recorded beside the
entry it justifies, and no umbrella ADR over dependency exceptions generally was written or
required. That is the pattern `kb-decision-0034` records for the fixture contract, and it
now has an instance outside the fixture contract. What keeps it honest is what keeps the ban
real: exempting *by name* means a third route into the graph still fails the gate until
someone decides it should not.

One thing this resolution deliberately does not cover, because the staged record says so in
terms: it does not adjudicate `happenstance-cloudflare`'s three store-limit numbers.
HS-P0013's run-2 review found them read off a platform page while HS-S0055 AC-001 forbids
exactly that. No document has settled it — ADR-0023 did not and ADR-0035 does not — and it
surfaces in HS-S0055's own record rather than becoming a second question here.
