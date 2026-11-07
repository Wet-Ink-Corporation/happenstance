---
id: kb-decision-0101
title: The fan-out runner does not catch a panic, and ADR-0074 decision 11 binds only a runner that does
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0101
reversibility: high
phase: 18
supersedes: null
superseded_by: null
summary: >-
  Proposed on 2026-11-07 for #112. The owner chose it himself on the #112 panic card
  (2026-11-07 04:38Z, "Catch nothing"). happenstance::run_fan_out, the fan-out runner PS-30 names,
  catches no unwind: a panic in a view's decode, apply or on_error unwinds out of the call, as it
  does out of run_projection, every open write set is dropped on the way out, and the port rolls a
  dropped batch back, so no row a view wrote since its last commit survives and no checkpoint
  moves. ADR-0074 decision 11 ("catch_unwind around each poll") is read as conditional on a runner
  that catches, which this one is not. PS-30's MUST binds a runner that catches, so the shipped
  runner meets it by unwinding, and panicking_apply_rolls_back holds it, and three catching
  mutants, to the observable any runner owes. Catching later is additive: ViewError is
  #[non_exhaustive], so a Panicked variant and a catching runner can follow without a break.
depends_on:
  - kb-decision-0074
related:
  - kb-decision-0063
source_paths:
  - crates/happenstance/src/fan_out.rs
  - crates/happenstance/tests/fan_out/rules.rs
  - crates/happenstance/tests/projection_clauses.rs
  - spec/SPECIFICATION.md
last_reviewed: 2026-11-07
---

# The fan-out runner does not catch a panic, and ADR-0074 decision 11 binds only a runner that does

**Proposed on 2026-11-07 for #112.** The owner made the call himself on the #112 panic card
(2026-11-07 04:38Z): "Catch nothing". The session that wrote this record built it. Acceptance is
the owner's, and it flips `status` alone. No long-form record exists; this atom is the record.

## Context

ADR-0074 decision 11 says that if phase 18 builds the fan-out runner, `panicking_apply_rolls_back`
over an async `apply` needs `catch_unwind` around each poll, through a hand-written `poll_fn`
wrapper. #112 builds that runner. PS-30's MUST is conditional: it binds a fan-out runner that
**catches** a panic.

Catching was probed on rustc 1.97.1 with `std` alone: a panic raised on a second poll, after an
await, is caught, no `unsafe` is needed, and the enclosing future stays `Send`. What it costs is
not the mechanism:

- The temper standard treats `catch_unwind` in production code as a hard H-03 finding, and a
  panicked task as a bug to treat as fatal (ASY-04). Shipping it needs a standing exception.
- It isolates nothing on `wasm32-unknown-unknown` or under `panic = "abort"`, where a panic ends
  the process whichever runner is used, and `happenstance` builds for `wasm32`.
- Its honesty rests on a `C: RefUnwindSafe` bound, a quarantine of the panicked view, and an
  awaited rollback. The bound and the catching behaviour could not be withdrawn in 1.x.

## Decision

1. **`run_fan_out` catches nothing.** A panic in a view's decode, `apply` or `on_error` unwinds out
   of the call. Every open write set, the panicking view's and every other view's, is dropped on
   the way out, and the port requires a dropped batch to roll back. Nothing applied since any
   view's last commit survives, and no checkpoint moves.
2. **ADR-0074 decision 11 is read as conditional.** It states what a catching runner would need. It
   does not oblige the runner to catch, and no catching runner ships. ADR-0074 is not edited.
3. **PS-30 stays `[DEFERRED]`** with its rule written: `panicking_apply_rolls_back` runs against
   the shipped runner and fails three catching runners (one that carries on, one that drops the
   write set bare, one that rolls it back only after the others commit). Freezing it is #127's.
4. **The tripwire stays true.** `no_fan_out_runner_catches_a_panic` scans `fan_out.rs` beside
   `runner.rs` and `lib.rs`, and drives the fan-out through a panic.

## Alternatives

- **Catch per poll, as decision 11 describes.** Isolates a panicking view in one task on a
  catching target, at the cost of a standing H-03 exception, a `RefUnwindSafe` bound on the codec,
  and a guarantee that does not exist on two targets this workspace builds for. Left as an
  additive follow-up rather than rejected outright.
- **One task per view through `run_projection`.** The executor contains the panic, at one read per
  view. This is the safe runner E2E-28 names, and `run_fan_out`'s rustdoc points to it.

## Consequences

- One view that panics on an event stops all N in the run, and stops them again on every restart
  until it is fixed. E2E-28 names this trade: the cheap runner and the safe runner are opposites.
- `ViewError` is `#[non_exhaustive]`, so a later `Panicked` variant and a catching runner are
  additive.

## Falsifier

This record is wrong if a panic in one view of a `run_fan_out` call leaves a row that view wrote
since its last commit, or moves any view's checkpoint.
