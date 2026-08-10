---
id: governance-weigh-the-msrv-floor
title: Weigh the MSRV floor, do not obey it
kind: governance
status: accepted
authority_tier: guideline
summary: >-
  The MSRV is 1.97.1 and is PROVISIONAL until phase 12, when first publish turns it into a
  promise to somebody else. Nothing is published, so no consumer is pinned; raising the
  floor is a trade to be recorded in an ADR, never a prohibition to be routed around. What
  stays forbidden is moving it in silence.
depends_on:
  - adr-0029-msrv-raised-to-1-97-1
related:
  - adr-0004-edition-and-msrv
source_paths:
  - CLAUDE.md
  - rust-toolchain.toml
last_reviewed: 2026-08-09
---

# Weigh the MSRV floor, do not obey it

## The rule

The floor is a cost to be weighed, not a wall. Raising it is legitimate and is done by
writing an ADR. Moving it without one is not.

## Two consequences that are easy to miss

**The `msrv` CI job currently proves nothing.** The MSRV now *equals* the pin in
`rust-toolchain.toml`, so both of that job's steps run the same compiler the gate already
runs. It is kept for the day the two diverge, and it says so in its own comments. Do not
delete it as dead weight, and do not cite it as evidence.

**The floor moved because of a dependency, not because of this workspace's code.** Five of
the five database crates here declare no `rust-version` at all, so `cargo hack
--rust-version` cannot protect a floor against them and neither can `resolver = "3"`. Only
running the compiler finds it. That is why the job runs a full test at the floor as well
as a `--no-dev-deps` check: `--no-dev-deps` is exactly the flag that hides `proptest` and
`tokio`.

## What changes at phase 12

ADR-0004 loses its `provisional` marker at first publish, and the floor stops being a
private trade and becomes a compatibility promise.
