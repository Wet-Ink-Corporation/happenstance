# ADR-0038: `async-trait` is exempted where it is reached through `testcontainers`

- **Status:** accepted
- **Date:** 2026-09-06
- **Amends:** [ADR-0001](0001-async-port-flavours.md), which bans `#[async_trait]`
  from this workspace, and [ADR-0035](0035-async-trait-through-worker.md), which
  first widened its exemption set. Neither body changes; what changes is the
  **exemption set** the `deny.toml` guard carries.
- **Owner:** phase 10, `postgres-and-neon-stores` (HS-P0014), story
  `postgres-schema-and-live-fixture` (HS-S0061).

## Context

Phase 10 gives `happenstance-postgres` a live server to be measured against.
`RUNBOOK.md:4350-4362` names the mechanism: *"`testcontainers` for the fixture,
pinned to a specific Postgres minor, in its own CI job."* Adding it turned
`cargo deny check bans` red, and the failure is not a false positive:

```text
error[banned]: crate 'async-trait = 0.1.91' is explicitly banned
  in async-trait v0.1.91 (proc-macro)
    testcontainers v0.28.0
      (dev) happenstance-postgres v0.2.0-alpha.1
    tonic v0.14.6
      bollard v0.21.1
        testcontainers v0.28.0
      bollard-buildkit-proto v0.8.1
      tonic-prost v0.14.6
    worker v0.8.5                     <- already exempted, ADR-0035
    worker-macros v0.8.5              <- already exempted, ADR-0035
```

Two new edges, both arriving with `testcontainers`: one direct, and one through
`tonic`, which is how `bollard` — the standard Docker client for Rust — speaks
gRPC to buildkit. `deny.toml`'s own comment refused to pre-authorise this case in
exactly the words ADR-0035 was written against: *"A third route still fails until
someone decides it should not."* This is that decision.

## The argument, and it is the first one this time

ADR-0035 had to open by discarding the `wasm-bindgen-test` justification, because
`worker` **ships** — it is a normal dependency of `happenstance-cloudflare` — and
reusing "appears in no published artifact" for it would have been false. That
inversion does not repeat here. The original argument is the one that applies,
and it applies more cleanly than it does to the crate it was written for:

- `testcontainers` is a `[dev-dependencies]` of `happenstance-postgres` **alone**.
- Cargo strips dev-dependencies from a published manifest, so no consumer of
  `happenstance-postgres` resolves it, links it, or inherits an MSRV floor from
  it.
- `cargo hack check --no-dev-deps --rust-version` — the `msrv` job's first half,
  which is what a *consumer* sees — cannot see it at all.
- Nothing in this workspace implements a `testcontainers` or a `tonic` trait, so
  no `happenstance` port passes through the macro. Not one method signature in
  the contract, an adapter, or the typed layer is desugared by it.

So the property ADR-0001 exists to protect — a `!Send` store staying reachable on
`wasm32`, because a Durable Object's storage is `!Send` by construction — is
untouched in the only way that matters: no port acquired a `+ Send` bound.

That property also has a guard that is not this one.
`crates/happenstance/tests/flavours.rs` instantiates every typed-layer entry point
against a store that is genuinely `!Send`, on every target, and stops compiling
the moment a `Send` bound reaches the chain. `cargo deny` is an OPTIONAL, probed
gate step and is therefore unenforced on a machine without the tool; `flavours.rs`
is unskippable. The ban is the *legible* guard, and this ADR widens the legible
one by two names while leaving the unskippable one exactly where it was.

## Why `tonic` is named separately

`wrappers` matches **direct** parents. Listing only `testcontainers` would leave
`tonic`'s own edge to `async-trait` unmatched and the check still red, and
listing `bollard` instead would be wrong in the other direction — `tonic` is
`async-trait`'s direct parent, `bollard` is one further out.

The two are not collapsed into a broader allowance because a wrapper entry is a
statement about a *route*, not about a crate's character. `tonic` is a general
gRPC framework that a shipping crate could plausibly reach one day; naming it
here admits it from anywhere, which is a real cost, accepted knowingly and
recorded here so that a future reader can price it rather than discover it. The
mitigation is under *Consequences*.

## What was rejected

**Dropping `testcontainers` for an environment-supplied URL plus a GitHub Actions
`services:` container.** Genuinely viable, and it was priced: zero new
dependencies, no ban pressure, no 236-node dev graph, and the same shape the Neon
fixture must use regardless — which is the case `event_store_conformance!` taking
an *expression* was written for. It loses on two counts. It deviates from the
runbook's named plan of record, which would owe its own record; and it moves the
image pin out of Rust and into YAML, where a developer running the suite locally
no longer gets the pinned minor by construction but by remembering. NF-003 exists
because *"an unpinned image makes a red job unattributable to a change in this
repository"*, and a pin a human has to reproduce by hand is weaker than one the
fixture asserts.

**Widening the ban to a warning.** It would turn every future route green,
including from a crate that ships, which is precisely the failure mode
`deny.toml`'s comment was written to prevent.

**Vendoring or forking a container harness to drop the `async-trait` edge.** The
cost is permanent and the benefit is a lint result.

## Decision

Add `testcontainers` and `tonic` to the `async-trait` ban's `wrappers` list in
`deny.toml`, with the argument recorded at the entry rather than only here.

## Consequences

- `cargo deny check bans` passes, so `cargo xtask ci` is green at every step.
- The ban's legible guard no longer covers the `testcontainers` or `tonic`
  subtrees. `flavours.rs` still covers what ADR-0001 protects, on every target,
  without a probe.
- A crate reaching `async-trait` by any **fourth** route fails the gate, which is
  the behaviour that keeps this exemption meaningful rather than decorative.
- `tonic` is now admissible from anywhere in the graph, including from a crate
  that ships. This is the one real weakening in this record. The compensating
  observation is that a shipping crate gaining `tonic` is a large, visible
  manifest change, and `cargo hack --no-dev-deps` would show it moving into the
  published graph where `testcontainers` never appears.
- `testcontainers 0.28` is pinned by what it drags, and a major bump that changed
  how it reaches `async-trait` would not be caught, because the wrapper name is
  unchanged. That is a known gap, identical to the one the `wasm-bindgen-test`
  entry has carried since the ban's first run and the one ADR-0035 recorded for
  `worker`.

## What this does not decide

**The position-visibility mechanism.** ADR-0024 is a deliverable of the
`position-visibility-decision` slice and nothing here touches it. Migration 1
carries no `xid8` column and `position` carries no sequence default precisely so
that this story cannot answer that question by accident.

**Whether the default gate should ever grow a live-infrastructure step.** It must
not — DR-9 and AC-011 — and this record does not reopen it. `testcontainers`
enters as a dev-dependency whose tests are `#[ignore]`d; the container starts in
its own CI job, never in `cargo xtask ci`.

**Whether `cargo deny`'s probed-optional status is acceptable.** ADR-0035 named
it as a known weakness and so does this. Making the supply-chain gate mandatory
is a question for `the-gate`, not for an adapter.
