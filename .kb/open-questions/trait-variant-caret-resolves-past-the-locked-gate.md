---
id: kb-open-question-trait-variant-caret-001
title: The two-flavour port derivation is emitted by a caret-pinned proc macro that only --locked can see
kind: open_question
status: accepted
authority_tier: note
summary: >-
  Binding constraint 4 (bind EventStore, not SendEventStore, in generic code) rests on a blanket
  impl that this repository does not write — trait_variant emits SendEventStore, SendProjectionStore
  and the flavour-bridging impl at compile time, and the manifest requirement is a caret,
  trait-variant = "0.1.3", so 0.1.4 resolves without the manifest changing. Every existing guard on
  the derivation's shape is #[cfg(test)], so it only ever observes the version the workspace's own
  Cargo.lock has resolved; the gate runs --locked precisely so that version cannot move without a
  deliberate change inside this tree. But Cargo.lock does not travel to a consumer of a published
  library crate, and three of these crates are already on crates.io at 0.2.0-alpha.1 — so a
  downstream consumer's resolve can already float to a shape this repository has never compiled
  against, today, not at some future release. A companion, uncontested finding rides along: a
  sentence in happenstance-core's store.rs claims the gate's assertion protects the derivation
  generally, when it is true only of this workspace's own --locked builds and false of what a
  consumer's resolve can see. Neither the version-pinning question nor a floating-resolve CI check
  was decided or built here — both were judged outside this lane's writable surface and are routed
  to whoever owns ADR-0001 and ADR-0035, the two decisions the two-flavour derivation belongs to.
depends_on: []
related:
  - kb-decision-0001
  - kb-decision-0035
source_paths:
  - .kb/_intake/remediation-2026-09-04-briefs/trait-variant-caret-and-the-consumer-resolve.md
  - crates/happenstance-core/src/store.rs
  - Cargo.toml
  - xtask/src/main.rs
  - .github/workflows/ci.yml
last_reviewed: 2026-09-07
---

# The two-flavour port derivation is emitted by a caret-pinned proc macro that only --locked can see

## What is true today

`SendEventStore`, `SendProjectionStore`, and the blanket impl that CLAUDE.md's binding constraint 4
rests on ("bind `EventStore`, not `SendEventStore`, in generic code") are not written in this
repository — they are emitted at compile time by the `trait_variant` proc macro, per ADR-0001's
two-trait design for keeping the `wasm32` / Cloudflare Workers target reachable without
`#[async_trait]`'s injected `+ Send`. The manifest requirement, `Cargo.toml:130`, is a caret:
`trait-variant = "0.1.3"`. A caret at `0.1.x` resolves any `0.1.z` with `z >= 3`, so `0.1.4` would
resolve with no manifest change and no review of what it emits.

Measured in this working tree: `xtask/src/main.rs` runs the whole gate `--locked` (32 occurrences of
the flag), and `.github/workflows/ci.yml`'s only unlocked-resolve trigger is a weekly `schedule:` run
that drives `advisories` and similar jobs, none of which re-resolves the dependency graph fresh.
Every existing guard on the derivation's shape — the tests asserting the blanket impl and the `Send`
flavour's properties — is `#[cfg(test)]`, so every one of them runs against, and can only ever
observe, whatever version `Cargo.lock` already pins. The `--locked` gate exists precisely so that
version cannot move without someone in this tree deliberately changing it.

That protection stops at the repository boundary. `Cargo.lock` is not published with a library
crate and does not constrain a downstream consumer's own resolve. Three of these crates are already
on crates.io at `0.2.0-alpha.1`, so a consumer's build can resolve `trait-variant` to a version this
workspace has never compiled against — not hypothetically, but as a live possibility today, because
nothing about the caret or the gate changes at any future release; it is already true.

A smaller, uncontested finding travels with this: `crates/happenstance-core/src/store.rs` states
that the derivation's shape is "asserted" so a version cannot move without someone changing it
deliberately — true of this workspace's own `--locked` gate, and silently false of a consumer's
unlocked resolve, because the sentence carries no scope qualifier saying which build it describes.

## What is not decided

Whether the caret narrows to an exact `trait-variant = "=0.1.3"` requirement, which trades version
flexibility (every patch release needs a manual bump, and a hard version conflict becomes possible
for any consumer who also depends on `trait-variant` at a different version through another crate)
for eliminating the exposure entirely; whether the exposure is instead covered by a derivation
contract compiled into `happenstance-core`'s own library code — not `#[cfg(test)]` — so a shape
change fails at a *consumer's* build with a message naming the cause; or whether the caret stays and
only the `store.rs` sentence is corrected to state its actual scope. A second, independent question
is whether a non-blocking `floating-deps` CI job should join the existing weekly `schedule:` trigger,
resolving without the lock file to observe (without gating on) a version this workspace has not yet
adopted. Neither question was settled: the crate sources are held by another lane for the duration
of the remediation this question came out of, and CI workflow ownership sits outside `xtask/`, which
is this lane's writable surface. Both are routed to whoever owns ADR-0001 and ADR-0035 — the two
decisions the two-flavour derivation itself belongs to — because `standards/rust/50-dependency-
hygiene.md` is where a policy on proc-macro crates whose expansion is public API would need to live,
and it does not yet say anything about this class of dependency.

## What forces it

Whenever `trait-variant` next publishes `0.1.4` or later — an event entirely outside this
repository's control, and unpriced by any option here, because a shape change in a version nobody
has published cannot be tested by anything in the tree today. The `store.rs` sentence correction is
free at any time and should not wait for that event.
