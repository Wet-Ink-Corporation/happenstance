---
id: kb-decision-0070
title: The runner takes a named Chunk, and gets no observation seam at 1.0
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0070
reversibility: medium
phase: 16
supersedes: null
superseded_by: null
summary: >-
  run_projection's chunk parameter stops being a bare NonZeroUsize and becomes a named Chunk type:
  #[non_exhaustive], private representation, built through named constructors, Chunk::of(NonZeroUsize)
  first, the shape Retry already has in the same crate. Decided now because the parameter sits in
  run_projection's signature, which is free to change only while happenstance's unstable-projection
  gates the runner; phase 18 builds Chunk before it lifts that gate, and the fan-out runner phase 18
  also builds (PS-30) takes the same type, so the crate has one chunk vocabulary rather than two.
  Whether run_projection takes impl Into<Chunk> with From<NonZeroUsize>, which would keep today's
  call sites compiling, is left to phase 18's compile. No Default: every chunk size produces the same
  read model (PS-14), so a default hides only a cost, and the measurement that would choose it —
  wall time, peak memory and commits at {1, 8, 64, 1024, 65536} against a real store — does not
  exist; adding Default later is additive. No observation seam at 1.0: Progressed is already
  #[non_exhaustive] and can grow fields, a run_projection_observed entry point taking a callback is
  additive in any later minor, and tracing is declined at this layer, which keeps the
  specification's "logs nothing" sentence true. This departs from the open question's own
  recommendations (a default after measurement, and the observed entry point at the freeze) on one
  argument: both are additive, and the only thing a freeze forces is the half that is not. Closes
  kb-open-question-projection-runner-chunk-observation-001.
depends_on:
  - kb-decision-0063
  - kb-decision-0036
related:
  - kb-decision-0066
  - kb-decision-0007
  - kb-open-question-projection-runner-chunk-observation-001
  - kb-open-question-apply-synchronous-live-store-001
source_paths:
  - crates/happenstance/src/runner.rs
  - crates/happenstance/src/command.rs
  - experiments/polling-cost/README.md
  - runbook/phases/18-typed-runner.md
  - spec/SPECIFICATION.md
last_reviewed: 2026-09-29
---

# The runner takes a named Chunk, and gets no observation seam at 1.0

## Why this is decided now and not at phase 18

`run_projection`'s signature is `(events, models, projection, codec, chunk: NonZeroUsize)`
(`crates/happenstance/src/runner.rs:514-520`, `chunk` at `:519`). While `happenstance`'s
`unstable-projection` gates the runner, that signature carries no semver promise, so changing it is
free. The moment phase 18 lifts the gate, it is published surface, and replacing `NonZeroUsize` with
anything else is a major version. The observation seam is the opposite case: every form of it is
additive. So the parameter type is decided here, before the lift, and the seam is deliberately left
for later.

## Decision

**1. `chunk` becomes a named `Chunk`.** `#[non_exhaustive]`, private representation, built through
named constructors, with `Chunk::of(NonZeroUsize)` first. This mirrors `Retry` in the same crate
(`crates/happenstance/src/command.rs:43-62`: `#[non_exhaustive]`, a private `NonZeroU32`,
`Retry::attempts` and `Retry::once`).

The Rust reason it is worth a type, when `NonZeroUsize` already rules out zero, is room to grow. A
parameter typed `NonZeroUsize` can only ever mean "this many events". A struct whose field is
private can later hold a private enum and gain a `Chunk::adaptive()` constructor without any caller
noticing, because no caller could name or match the representation in the first place. The
honest counter-argument is that today `Chunk` adds no invariant over `NonZeroUsize`, which makes it
ceremony, unlike `Retry`, which has two construction shapes from the start. It loses because the
ceremony costs one name at a call site, and the alternative is paid in a major version the first
time the parameter needs to mean anything else.

**2. The conversion is phase 18's to compile.** Taking `chunk: impl Into<Chunk>` with
`impl From<NonZeroUsize> for Chunk` would keep every existing call site compiling unchanged. Taking
`chunk: Chunk` is plainer, and it makes each call site spell out what it chose. Which is better
depends on how inference behaves at the real call sites, and that is a compile, not an argument.
Phase 18 chooses and records the choice in its session log.

**3. No `Default` until it is measured.** `Retry` refuses a default because a hidden default would
hide a *semantic* worst case, namely how many times a command re-runs. That argument does not
carry over here: PS-14 (`spec/SPECIFICATION.md:5514`) requires the same read model at every chunk
size, so a default for `Chunk` would hide only a cost. A cost is exactly what a measurement should
choose, and the measurement does not exist. `experiments/polling-cost` fixes the chunk at 1024
(`experiments/polling-cost/README.md:37`), and `examples/rebuilding-read-models` runs two sizes to
check PS-14 equality, not to compare cost. Adding `impl Default for Chunk` later is a minor change,
so it waits for the numbers: wall time, peak resident memory and commit count at
{1, 8, 64, 1024, 65536} against a real projection store.

**4. One chunk vocabulary.** The fan-out runner phase 18 builds (PS-30, E2E-32) takes the same
`Chunk`. If the single-view and fan-out runners took two differently shaped chunk parameters, that
difference would be a second vocabulary to freeze.

**5. No observation seam at 1.0.** The checkpoint is the API. `Progressed` is already
`#[non_exhaustive]` (`runner.rs:102-126`, the attribute at `:114`), so it can grow fields without
a break. A `run_projection_observed` entry point taking a callback, the plain-door/full-door shape
`commit`/`commit_with` already uses, is additive in any later minor. `tracing` spans are declined
at this layer. They would be the workspace's first `tracing` dependency, which is a larger decision
than the runner's own surface, and they would falsify the specification's sentence that the runner
"writes nothing anywhere, logs nothing" (`spec/SPECIFICATION.md:5959`).

## Where this departs from the open question

The open question recommended a named type **with** a default after measurement (its 1C) and the
observed entry point at the freeze (its 2B). This record takes 1B and 2A. Nothing in the open
question's own analysis is overturned. Its strongest counters were "a flat curve makes 1C
ceremony" and "no operator has asked for 2B", and both still stand unrefuted. The difference is the
rule that decides the timing: a freeze forces only the half that cannot be added later. The type
cannot, so it is decided. The default and the seam can, so each waits for the evidence the
question itself said it lacked.

## Alternatives rejected

- **1A: keep `NonZeroUsize`.** This freezes the parameter at "a count" for the whole of 1.x, and
  it leaves two caller-supplied bounds in one crate treated in opposite ways.
- **1C now: a default without the measurement.** This would choose a performance number by taste
  and publish it as the value every caller who didn't choose gets.
- **2B now: the observed entry point before anyone needs it.** It is additive, so shipping it now
  buys nothing. It would double the page to maintain, and it would bring in a synchronous callback
  inside an async loop, a foot-gun for any caller who does I/O in it, before a reported case exists
  to design against.
- **2C: `tracing`.** Declined at this layer, for the reasons in point 5.

## Falsifier

Reopen point 3 when the chunk-size measurement exists: a curve that is not flat is the case for a
default, and it arrives additively. Reopen point 5 when an operator reports needing progress from a
run in flight that polling a second handle's checkpoint cannot give them. If phase 18's compile
finds that `Chunk` cannot be taken by both runners without a second type, this record is wrong
about point 4 and phase 18 says so before it lifts the gate, not after.
