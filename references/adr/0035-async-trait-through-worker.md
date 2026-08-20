# ADR-0035: `async-trait` is exempted where it is reached through `worker`

- **Status:** accepted
- **Date:** 2026-08-20
- **Amends:** [ADR-0001](0001-async-port-flavours.md), which bans `#[async_trait]`
  from this workspace. ADR-0001's reasoning is untouched and its body stays
  verbatim; what changes is the **exemption set** its `deny.toml` guard carries.
- **Resolves:** `kb-open-question-worker-async-trait-ban-001`, minted by the
  phase-9 ingest wave (`.kb/open-questions/deny-bans-red-on-the-worker-dependency.md`)
  precisely because neither shape had been chosen.

## Context

Phase 9 replaced the hand-written Durable Object stand-in with the real `worker`
crate (HS-S0049, `310a4c8`). That is the whole point of the project: an adapter
that is run rather than asserted. It also turned `cargo deny check bans` red, and
the failure is not a false positive:

```text
error[banned]: crate 'async-trait = 0.1.91' is explicitly banned
  ├ async-trait v0.1.91
    ├── wasm-bindgen-test v0.3.76        ← already exempted
    ├── worker v0.8.5
    │   └── happenstance-cloudflare v0.2.0-alpha.1
    └── worker-macros v0.8.5
        └── worker v0.8.5 (*)
```

Two new edges, both arriving with `worker`. `deny.toml`'s own comment predicted
this case and refused to pre-authorise it: *"a second route into the graph — a
manifest that names the crate itself, or any other crate that pulls it in — is a
new wrapper this list does not carry, and the check fails until someone decides
it should."* This is that decision.

## The argument that does not work, and it must be said first

The existing `wasm-bindgen-test` exemption rests on a fact that **does not carry
over**. `deny.toml` justifies it as *"a dev-dependency of the conformance
harnesses [that] appears in no published artifact."* `worker` is a **normal**
dependency of `happenstance-cloudflare` — it ships. Reusing the old sentence for
the new wrapper would be reasoning about a different situation with the same
words, so the exemption needs its own argument or it does not get one.

## The argument that does work

**ADR-0001 bans `#[async_trait]` to keep `wasm32` reachable**, because the
attribute injects `+ Send` into every method it desugars and a Durable Object's
store is `!Send` by construction. The ban is a proxy for that property, not an
end in itself.

`worker` uses `async-trait` **for its own traits**, which this workspace neither
implements nor derives ports from. No `happenstance` port, and no method on one,
passes through the macro. The property ADR-0001 protects is untouched: the
`wasm32` target still builds, and `crates/happenstance-cloudflare` is the crate
that proves it, since it is the one taking the dependency.

That claim is checkable rather than asserted, and by something stronger than
`cargo deny`. `crates/happenstance/tests/flavours.rs` instantiates every
typed-layer entry point against a store that is genuinely `!Send`, and it stops
compiling the moment a `Send` bound reaches the chain. `deny.toml` says so in its
own words — *"the unskippable one is `crates/happenstance/tests/flavours.rs`"* —
and it is unskippable in exactly the sense `cargo deny` is not: it is a
compile-time obligation, not an optional probed gate step.

So the exemption removes the legible guard on two edges while leaving the load-
bearing one in place on all of them.

## What was rejected

**Keeping the ban absolute.** Refusing the exemption means `worker` cannot be
depended on, which reopens HS-S0049 and returns the crate to the stand-in the
project exists to replace. That trades a real adapter for a clean `deny.toml`
line, and ADR-0001 was never about the line.

**Rewording AC-012 and initiative DoD 13** to *"green at every required step"*,
leaving `cargo deny check bans` red and disclosed. This is accurate about what is
measured — `cargo xtask ci` is green at every REQUIRED step, including all seven
`wasm_steps()` names and `spec-trace`, and red only at the OPTIONAL *licences and
advisories* step — but it moves an acceptance sentence onto the artefact that
shipped. That is the edit `af9eb10` reverted at HS-P0013's run-2 gate, and the
distinction recorded there holds: widening a **path fence** a correct
implementation cannot satisfy is licensed; moving an **acceptance sentence** is
not.

**A blanket allow, or dropping the ban.** Rejected for the reason `deny.toml`
already gives: exempting by name is what keeps the ban real. A third route into
the graph must still fail until someone decides it should not.

## Decision

Add `worker` and `worker-macros` to the `wrappers` list of the `async-trait` ban
in `deny.toml`. Do not broaden the ban's shape, do not remove it, and do not
touch the `wasm-bindgen-test` entry.

Record on the entry that the two new wrappers rest on a **different** argument
from the first, so a later reader does not collapse them.

## Consequences

- `cargo deny check bans` passes, so `cargo xtask ci` is green at every step and
  HS-S0059's AC-012 can be claimed honestly rather than reworded.
- The ban's *legible* guard no longer covers the `worker` subtree. `flavours.rs`
  still does, on every target, without a probe.
- A future crate reaching `async-trait` by any third route fails the gate, which
  is the behaviour that makes this exemption meaningful rather than decorative.
- `worker 0.8.5` is pinned by what it drags. A major bump that changed how
  `worker` uses `async-trait` would not be caught by this list, because the
  wrapper name is unchanged. That is a known gap, and it is the same gap the
  `wasm-bindgen-test` entry has carried since the ban's first run.

## What this does not decide

**The three store-limit numbers.** HS-P0013's run-2 review found them read off a
platform page while HS-S0055 AC-001 says *"never read off a platform page"*. No
staged document has adjudicated it, ADR-0023 did not, and neither does this. It
is still owed.

**Whether `pub mod host` survives publication.** It is `#[doc(hidden)]` and every
item behind it panics on a real Workers isolate.
