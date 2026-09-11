---
id: kb-playbook-require-the-property-001
title: Requiring a mechanism rather than a property is how a defect acquires four defenders
kind: playbook
status: accepted
authority_tier: guideline
summary: >-
  An xtask guard and four test assertions all required the literal `pub use
  happenstance_core::*` glob by name rather than the property it stood for —
  that contract items resolve only through their own gates — so ten ungated
  contract names leaked under `cargo test` while every check stayed green. A
  crate root with no gate at all passed unchanged. The repair derives each
  item's gate from the contract crate's own root and rejects four wrong
  implementations by mutation. The boundary: it stops holding if the gate step
  stops compiling integration targets. A second instance, 2026-09-10, in a
  specification clause rather than a guard: PS-6's "begin MUST be neither
  async nor fallible" named a signature as proxy for the property it protected
  — a buffering adapter's begin makes no round trip, resolves at its first
  poll and never fails. ADR-0062 (kb-decision-0062) moved the signature to
  async and fallible so a live-transaction store could exist, and the brief
  records the property held by two tests instead (one over a transport that
  fails every request, one polled once with no executor), with the MUST
  rewritten to state the property. Same repair as the guard's: derive what
  the check is for, and assert that.
depends_on:
  - kb-decision-0036
related:
  - kb-playbook-assert-execution-not-discovery-001
  - kb-playbook-verify-referent-report-coverage-001
  - kb-decision-0062
source_paths:
  - .kb/_intake/remediation-2026-09-04-briefs/typed-layer-promise-guard.md
  - .kb/_intake/2026-09-10-adr-0062-the-probe-seam-moves.md
last_reviewed: 2026-09-11
---

# Requiring a mechanism rather than a property is how a defect acquires four defenders

## The claim

A check that asserts on a literal piece of source text — a specific line, a
specific token, a specific re-export spelling — is testing the mechanism a
property was implemented with, not the property itself. When the mechanism
changes while the property silently breaks, every check built that way stays
green together, because they were all reading the same wrong line.

## The measured instance

`the_typed_layer_makes_no_promise_it_does_not_keep` (`xtask/src/main.rs`) read
`crates/happenstance/src/lib.rs` and asserted on the literal `pub use runner::{`
line. The item the check was actually meant to police sat three lines below:
`pub use happenstance_core::*;` — a glob that re-exports whatever the
**compiled** contract crate happens to expose. `happenstance-core` gates its
projection items on its own `unstable-projection` feature; `happenstance-testkit`
enables that feature unconditionally and is `happenstance`'s dev-dependency, so
under a routine `cargo test`, ten contract names — `Checkpoint`, `ProjectionId`,
`ProjectionStore`, `SendProjectionStore`, `Authority`, `CommitError`,
`ResetError`, `ProjectionProbe`, a whole module, and `MemoryProjectionStore` —
resolved through `happenstance::` with the *consuming* crate's
`unstable-projection` feature switched off. The guard stayed green throughout,
because it was never reading the line where the leak lived.

Four other tests made the same mistake in the same direction, and are the part
worth carrying forward because they show the failure is a shape, not a slip.
`the_glob_reexport_survives_and_nothing_shadows_it`, `crate_root_renders_the_
codec_surface`, `crate_root_renders_the_projection_surface`, and
`no_item_shadows_a_core_name` all required the glob **by name**, because the
story's own acceptance criterion was written as a mechanism: "nothing shadows a
contract name, and `pub use happenstance_core::*;` survives." The property that
sentence was actually protecting — that the contract's paths still resolve
through the facade — was never what got checked; the literal spelling was. A
constructed counter-example confirms the gap precisely: a crate root carrying an
explicit, fully-gated `pub use happenstance_core::{Authority, Checkpoint,
ProjectionId, ProjectionStore};` with **no gate at all** passes all five
assertions unchanged, because none of them can see below the one line they were
each written to check.

## The repair

`contract_surface.rs` derives each item's required gate from `happenstance-
core`'s own crate root — the source of truth the property is actually about —
compares it against an explicit, hand-written, `#[cfg]`-carrying list in
`happenstance`'s crate root, and rejects the property's violation regardless of
mechanism. It is verified against four wrong implementations, each demonstrated
by mutation rather than argued: restoring the glob; the explicit list with one
item's gate silently dropped; an ungated contract item silently missing from the
facade entirely; and a contract item mounted under a plausible-looking but wrong
gate. Each wrong implementation is caught by exactly one of the four checks, and
none catches another's defect — the four together, not any one alone, cover the
property.

## A second instance, in a clause rather than a guard

The 2026-09-10 brief for ADR-0062 ([kb-decision-0062](../decisions/0062-the-probe-seam-moves-and-the-far-end-is-built.md))
records the same shape one layer up, in the specification itself rather than in
a check over it. PS-6 read *"`begin` MUST be neither `async` nor fallible"* and
gave as its rule *"the signature; no runtime rule. Enforced by the compiler on
every implementer."* What that clause was protecting was never the signature.
It was a discipline about buffering adapters: a `begin` that makes no network
round trip, resolves at its first poll, and never fails — the discipline that
stops a Neon adapter spending a one-shot HTTP request on a `BEGIN` it does not
need. The signature was the cheapest mechanism that implied the property, and
the clause was written as the mechanism.

The cost came due exactly as this playbook predicts. ADR-0060 found that a
live-transaction store — `sqlx`, whose `BEGIN` *is* "something reserved from the
server before the first write" — could not exist behind a synchronous,
infallible `begin`. PS-6's own falsifier had fired, and nobody had said so,
because a clause stated as a signature has no falsifier that a signature can
fail: the compiler enforced the mechanism on every implementer while the
property the mechanism stood for was never examined once. When ADR-0062 moved
`begin` to `async fn begin(&self) -> Result<Self::Batch, Self::Error>`, the
mechanism could no longer carry the property at all — an `async` signature is
what permits the expensive `begin` the clause existed to forbid.

The repair is the one the guard got. The brief records the property held by two
tests where the signature no longer can: `begin_makes_no_round_trip`, in the
Neon adapter over a transport that fails every request, so a `begin` that
touched the network would surface as an error; and
`begin_resolves_at_its_first_poll_without_a_runtime`, in the contract crate,
polling the future once with no executor, so a `begin` that needed a second
poll would never resolve. And the MUST is rewritten to say what those tests
check — that a buffering adapter's `begin` resolves at its first poll and never
fails — rather than the signature that used to stand in for it. Each test names
the wrong implementation it rejects, which the mechanism-shaped clause never
could.

Two things the second instance adds to the first. A clause can be a
mechanism-shaped check as readily as a test can, and a `[PROVISIONAL]` marker
does not protect it: the falsifier PS-6 carried was correct and fired anyway,
unnoticed, because the clause's own rule said the compiler was the check.
And the moment a mechanism has to move for a reason unrelated to the property
— here, so the far end of PS-2 could be built — is the moment the property
must already be written down somewhere else, or it leaves with the mechanism.

## The general lesson

Before writing an assertion, ask: if the author fixed the underlying property by
a different mechanism than the one in front of you today, would this check still
pass? If the honest answer is "no, because I'm matching this literal spelling,"
the check is decorative the moment anyone edits the mechanism without reading the
check. Derive the expected value from the same source of truth the property
depends on — here, the contract crate's own root — rather than hand-encoding a
snapshot of it. The same question applies to a specification clause: if the
MUST names a signature, a lint or a line, ask what behaviour that mechanism was
chosen to guarantee, and write *that* as the rule, with a test that a wrong
implementation can fail. This stops holding once the artifact the derivation
reads from is no longer built under the gate configuration the check runs in —
`cargo hack check --no-dev-deps` strips dev-dependencies and does not compile
integration test targets at all, so a check living in one only proves the
property under the configurations that do build it.
