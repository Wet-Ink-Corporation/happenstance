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
  stops compiling integration targets.
depends_on:
  - kb-decision-0036
related:
  - kb-playbook-assert-execution-not-discovery-001
  - kb-playbook-verify-referent-report-coverage-001
source_paths:
  - .kb/_intake/remediation-2026-09-04-briefs/typed-layer-promise-guard.md
last_reviewed: 2026-09-07
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

## The general lesson

Before writing an assertion, ask: if the author fixed the underlying property by
a different mechanism than the one in front of you today, would this check still
pass? If the honest answer is "no, because I'm matching this literal spelling,"
the check is decorative the moment anyone edits the mechanism without reading the
check. Derive the expected value from the same source of truth the property
depends on — here, the contract crate's own root — rather than hand-encoding a
snapshot of it. This stops holding once the artifact the derivation reads from is
no longer built under the gate configuration the check runs in — `cargo hack
check --no-dev-deps` strips dev-dependencies and does not compile integration
test targets at all, so a check living in one only proves the property under the
configurations that do build it.
