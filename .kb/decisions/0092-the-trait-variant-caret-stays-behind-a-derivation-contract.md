---
id: kb-decision-0092
title: The trait-variant caret stays, a derivation contract compiled into happenstance-core holds the shape, and ES-7 is frozen
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0092
reversibility: low
phase: 17b
supersedes: null
superseded_by: null
summary: >-
  The owner's call, 2026-10-12 (wi-727ec8, "all as recommended"). Answers
  kb-open-question-trait-variant-caret-001. SendEventStore, SendProjectionStore and the blanket
  impls that make each the bare flavour are emitted by trait_variant, which the workspace requires
  as a caret, trait-variant = "0.1.3". Cargo.lock does not travel with a published crate, so a
  consumer's resolve can compile a 0.1.z this workspace never has, and every guard on the
  derivation's shape was #[cfg(test)]. Measured: three wrong derivations, each a patched copy of
  trait-variant 0.1.3 (no blanket impl; no Send on an async fn's future; no Send on a -> impl
  Trait return), all compiled green against happenstance-core under --no-default-features, which
  is what a consumer builds. Decision: the caret stays. A derivation contract, a const _ block of
  generic functions bounded by the Send flavours in crates/happenstance-core/src/derivation.rs,
  is library code and is type-checked in every build of the crate, a consumer's included. It
  asserts both bridges, the Send supertrait on each flavour, Send on SendEventStore::append's future and SendProjectionStore::begin's,
  and Send on SendEventStore::read's stream. Against the same three wrong derivations it fails
  each with E0277 at the assertion aimed at it, on the host and on wasm32. A non-gating weekly
  floating-deps job in ci.yml runs cargo update and then the contract, the build and the tests,
  so a new release is seen there before a consumer meets it; a workflow_dispatch input makes it
  red once on purpose. ES-7 is [FROZEN], its falsifier restated to cover a consumer's unlocked
  resolve. An exact =0.1.3 pin lost: it buys the same protection with a resolver conflict for
  every consumer who meets trait-variant through another crate, and a manual bump per patch.
  Reopened by a floating-deps run or a consumer report showing a 0.1.z that fails the contract or
  makes a downstream direct impl collide.
depends_on:
  - kb-decision-0001
  - kb-decision-0008
  - kb-decision-0066
  - kb-decision-0072
  - kb-decision-wi-727ec8
related:
  - kb-open-question-trait-variant-caret-001
  - kb-decision-0001
  - kb-decision-0008
  - kb-decision-0035
  - kb-decision-0072
  - kb-decision-wi-727ec8
  - kb-decision-wi-fa5c7b
source_paths:
  - crates/happenstance-core/src/derivation.rs
  - crates/happenstance-core/src/store.rs
  - crates/happenstance-core/tests/trait_variant_pin.rs
  - .github/workflows/ci.yml
  - Cargo.toml
  - spec/SPECIFICATION.md
  - runbook/ledgers.md
last_reviewed: 2026-10-12
---

# The trait-variant caret stays, a derivation contract compiled into happenstance-core holds the shape, and ES-7 is frozen

**Accepted by the owner on 2026-10-12** (`wi-727ec8`: "Keep the caret, with derivation contract
and weekly floating-deps job", taken "all as recommended"). No long-form record exists; this atom
is the record. It answers
[`kb-open-question-trait-variant-caret-001`](../open-questions/trait-variant-caret-resolves-past-the-locked-gate.md).

## Context

ADR-0001 derives the `Send` flavour of each port with `#[trait_variant::make]`, and ADR-0008 makes
that one derivation serve both ports. So `SendEventStore`, `SendProjectionStore`, and the blanket
impls `impl<T: SendEventStore> EventStore for T` and its projection twin are not written in this
repository. The proc macro emits them. CLAUDE.md's binding constraint 4 ("bind `EventStore`, not
`SendEventStore`, in generic code") rests on the first blanket impl, and binding constraint 3 rests
on the macro adding `Send` to `read`'s top-level stream.

The workspace requires `trait-variant = "0.1.3"`, a caret, so any `0.1.z` with `z >= 3` resolves
without the manifest changing. The gate builds `--locked`, so in this workspace the version cannot
move without a deliberate change. A consumer of a published crate has no lock. Their resolve takes
the newest compatible release, and every guard on the derivation's shape was `#[cfg(test)]`, which
a consumer's build never compiles.

ADR-0066's phase-16 pass classified the question as additive, and ADR-0072 moved ES-7 from
`freeze-by-17` to `freeze-by-17b` on that basis, naming an exact pin as the one answer that would
return it to phase 17. The owner took the caret.

## The exposure, measured

Three wrong derivations, each a copy of `trait-variant-0.1.3` from the registry with one edit to
`src/variant.rs`, applied with `cargo check -p happenstance-core --no-default-features --config
'patch.crates-io.trait-variant.path="…"'`:

- **W1**, the bridge gone: `make` emits no blanket impl.
- **W2**, `Send` gone from futures: an `async fn` becomes `-> impl Future<Output = …>` with no
  variant bound.
- **W3**, `Send` gone from `-> impl Trait` returns: `read`'s stream keeps only its own bounds.

Against the tree before this change, all three compiled green. A consumer resolving any of them
would have built `happenstance-core` without a diagnostic, and met the break later in their own
generic code, as an error about their own bounds.

## Decision

1. **The caret stays.** `trait-variant = "0.1.3"` is unchanged in the root `Cargo.toml`.
2. **A derivation contract is compiled into the library.** `crates/happenstance-core/src/derivation.rs`
   holds one `const _: () = { … };` of generic functions bounded by the `Send` flavours. Each
   obligation is discharged at the definition, from the bound alone (constitution RS-61-1), so
   nothing is instantiated, no concrete store or feature is needed, and auto-trait leakage from a
   concrete type cannot answer for the trait. It asserts:
   - `S: SendEventStore` implies `S: EventStore`, and `S: SendProjectionStore` implies
     `S: ProjectionStore` (the two bridges);
   - `SendEventStore::append`'s future is `Send`, and `SendProjectionStore::begin`'s;
   - `SendEventStore::read`'s stream is `Send` at the top level;
   - each `Send` flavour is itself `Send`, the supertrait `trait_variant` emits.

   The functions are never called, and their names start with `_`, so `dead_code` does not report
   them. That is the language's spelling for a deliberately unused item, not a lint attribute.
   Against W1, W2 and W3 with the contract in place:
   - W1: `error[E0277]: the trait bound S: EventStore is not satisfied` at `derivation.rs:58`, and
     `S: ProjectionStore` at `:86`. The same two on `wasm32-unknown-unknown`.
   - W2: `E0277`, `append`'s `impl Future<…>` "cannot be sent between threads safely" at `:68`,
     and `begin`'s at `:93`.
   - W3: `E0277`, `read`'s `impl Stream<…>` at `:78`, and nothing else.

   Each wrong derivation fails exactly the assertions aimed at it.
3. **A non-gating weekly `floating-deps` job.** A sibling job in `.github/workflows/ci.yml`, as
   `advisories` and `minimal-versions` are, on the same Tuesday `schedule` and on
   `workflow_dispatch`. It runs `cargo update`, which discards the lock and resolves every
   requirement to its newest version within the 1.97.1 floor (`resolver = "3"`). It then checks the
   contract alone, on the host and on wasm32, so a shape change is the first error in the log,
   and then builds and tests the workspace. It is not a gate step and not a required check: its
   answer moves without a commit, so a red run is work for a pull request of its own.
   `tests/trait_variant_pin.rs` reads the lockfile the job wrote, so the week a newer
   `trait-variant` resolves, that test goes red there whether or not the contract holds. That is
   the signal to read the new expansion and move the pin.
4. **Watching it fail once.** A boolean `workflow_dispatch` input, `floating_deps_prove_red`,
   false by default and read by this job alone, applies W1 to the resolved `trait-variant` for
   one run. A `grep` asserts the edit happened, so a moved upstream line fails the step rather
   than letting a green run claim the contract was tested. It can only run after a push, so the
   run is owed: dispatch it once, link it from #88 and phase 17b's session log, and leave the
   input at its default.
5. **ES-7 is `[FROZEN]`.** Its falsifier was `error[E0119]` on a downstream direct impl, and the
   `!Send` reference store in `happenstance-testkit` has compiled beside the blanket impl since
   phase 1. Restated to cover a consumer's unlocked resolve, it is a `trait-variant` release under
   which a downstream direct impl collides with the emitted one, or the derived flavour stops
   being the bare one. The contract fails the second in the consumer's own build. The first is
   coherence, which no assertion in the defining crate can state, and the `floating-deps` job is
   what sees it, because the job's tests compile that downstream store.
6. **The unscoped sentence is corrected.** `store.rs`'s claim that the version "cannot move"
   because the gate builds `--locked` now says it holds for this workspace's builds only, and
   names what covers a consumer's.

## Consequences

- A consumer whose resolve picks a `trait-variant` that breaks the shape gets an `E0277` inside
  `happenstance-core`, at a line whose doc comment names the property that moved, instead of a
  failure in their own code.
- The contract covers types only. That the derived trait still copies the base trait's attributes,
  which carries the two `#[doc(alias)]` search keys onto `SendEventStore`, is documentation.
  `missing_docs` guards it in this workspace and `tests/trait_variant_pin.rs` ties it to the
  locked version, and neither runs in a consumer's build. A consumer is exposed to a missing
  search key, never to a type error.
- The contract does not reject the `async fn read` refactor binding constraint 3 forbids. That is
  a change to this crate's source, and `spawns_from_generic` in `memory.rs` still rejects it.
- `happenstance-core/src/lib.rs` gains one private `mod derivation;` and no public item. Nothing
  here is semver-visible.

## What reopens it

A `floating-deps` run, or a consumer report, showing a `0.1.z` this workspace has not pinned
under which the contract fails or a downstream direct `impl EventStore` collides. The answer is
then an exact pin or a bump, and an exact pin taken after `1.0.0` is a resolver promise that needs
its own record.

## Alternatives that lost

- **An exact `=0.1.3` pin.** Closes the exposure by refusing to resolve anything else. It lost on
  cost: every consumer who also meets `trait-variant` through another crate at another `0.1.z`
  gets a hard resolver conflict, and every upstream patch release needs a bump here before anyone
  can take it. It also protects nothing the contract does not.
- **Correct the `store.rs` sentence and nothing else.** Honest, and leaves a consumer's build as
  exposed as the measurement above shows.
- **The contract without the job.** It fails a consumer's build, which is the right place to fail
  and the wrong place to find out first. The job is what makes this workspace the first to see it.
