---
item: HS-P0013
stage: intake
created: 2026-08-12T03:23:11.061Z
updated: 2026-08-12T03:23:11.061Z
template_sig: ab516678
rendered_sig: f7d75479
---

# Intake Brief — The edge store, run rather than asserted

## Problem

The `!Send` flavour is carried at real cost. It is why ports are defined once
without a `Send` bound and `trait_variant` derives the `Send` flavour, why
`#[async_trait]` is forbidden outright, and why `EventStore::read` returns the
stream at the top level and is not `async` (ADR-0001, ADR-0008). All of that is
paid so a store can live in a single-threaded edge runtime — and no store ever
has. `cargo xtask ci` builds `happenstance-cloudflare` for `wasm32`; a build is not
a run. Work that quietly drops this has changed the product
(`references/seeds/remaining-runway.md:88-91`).

## Desired Outcome

Every conformance rule runs and passes under `workerd` on `wasm32`, executed by
`cargo xtask ci` **in the same run as the rest of the gate** rather than as a
separately maintained subset (AC-07, DoD 4). The adapter's error type carries a
real `worker::Error` and is shown either to carry what the caller needs or
demonstrably not to. A declined capability reports the fixture's stated reason and
still appears in the output — this project is the first to exercise that policy
against a runtime that genuinely cannot do some of what the suite asks.

## Constraints

- **No upstream blockers.** This project is a DAG root and can start on day one
  alongside `projection-store-freeze`. It blocks `publication-and-positioning`.
- **Never introduce `#[async_trait]`** — it injects `+ Send` and makes this target
  impossible. This project is the reason that constraint exists.
- **`EventStore::read` returns the stream at the top level and is not `async`.**
  Nesting it inside a future silently drops `+ Send` on the `Send` flavour and
  defeats the two-trait design. Two tests in `memory.rs` assert this and it takes
  both.
- **Bind `EventStore`, not `SendEventStore`**, in generic code.
- A separately maintained wasm32 subset of the suite is explicitly **not** an
  acceptable outcome — AC-07 says *the same run*.
- **Non-goals**, each naming its owner: the projection-suite capability-reporting
  *policy* → `projection-store-freeze` (this project is its first real exerciser,
  not its author); reopening the tail seam → post-0.1, outside this initiative;
  publishing `happenstance-cloudflare` → `publication-and-positioning`.

## Open Questions

- **ADR-0023** — the `SqlStorage` mapping, and the shape of an off-tokio conformance
  harness. Nothing in the workspace runs a conformance suite off tokio today.
- **CF-39 / CF-40** — the fixture-limits ownership question. This is BR-13's
  exercise: a declined guarantee must be reported *with the fixture's stated
  reason*, never by silent absence. Carried in from
  `.kb/open-questions/cf-40-fixture-limits-ownership.md`.
- **ES-32** — whether a Durable Object's storage API makes a tail seam cheap enough
  to reopen post-0.1. The verdict is **recorded, not acted on** inside this
  initiative.
- **WF-11's falsifier** — a peer that cannot buffer a payload through a
  human-readable encoder. This runtime is where that falsifier is tested or shown
  not to bite.
- Whether a `workerd` runner can be made to exist in CI at acceptable cost. If it
  cannot, DoD 4 is not observable at all and that is a blocking finding, not a
  degradation.

## Proof artefact

**Every conformance rule green under `workerd` on `wasm32`, inside the same
`cargo xtask ci` invocation as the rest of the gate.** This would not exist if the
design were wrong: the two-trait `trait_variant` design, the non-`async` `read`,
and the whole `#[async_trait]` prohibition are load-bearing only if a real
single-threaded runtime can host a store, and a build of the crate proves none of
it. A design that had quietly lost `!Send` would still build for `wasm32` and would
fail the moment a rule actually ran.

## Clauses

- **CF-39** `[PROVISIONAL]`, **CF-40** `[PROVISIONAL]` — discharged here; CF-40's
  ownership is an open atom this project resolves.
- **CF-14** `[DEFERRED]`, **CF-27** `[DEFERRED]` — CF-27's completeness half belongs
  to `retention-and-incomplete-logs`; CF-14's far end is `sqlite-durable-store`'s.
  Both are re-read here for whether the deferral still holds on this runtime.
- **ES-7, ES-17** — confirmed by a real `worker::Error`-carrying error type.
- **ES-32** — verdict recorded, not acted on.
- **WF-11** `[PROVISIONAL]` — its falsifier tested here.
- Settled by **ADR-0023**.
- Nothing `[FROZEN]` is amended. If this runtime cannot satisfy a frozen clause,
  that is a new decision atom and a re-plan, not a line edit.

## Gate: Intake

Tick each box once the sections above satisfy it. `redkiln advance` reads these
against `.redkiln/templates/gates/intake.md` — where each box's rationale is
written — and will not leave `intake` until every one is ticked. Keep each box on
one line; the parser matches line by line.

- [x] The problem and desired outcome are stated.
- [x] Constraints and non-goals are recorded.
- [x] Open questions are captured for distillation.
- [x] The proof artefact is named, and it would not exist if the design were wrong.
- [x] For a port freeze: the axis it is most likely to be wrong about is named, and something in the workspace sits at the other end of it.
- [x] The clauses this work discharges or amends are listed by id.
- [x] Where this brief and `SPECIFICATION.md` disagree, the specification wins.
