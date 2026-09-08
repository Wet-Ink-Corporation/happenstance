---
id: kb-open-question-adapter-version-lockstep-001
title: Whether an adapter's version implies a happenstance-core version has no answer, and the easy answer collides with CF-32
kind: open_question
status: accepted
authority_tier: note
summary: >-
  A consumer on happenstance = "0.3" and happenstance-sqlite = "0.2" can hold two Event types that
  print identically, because nothing states whether the five publishable crates' version numbers
  move together or what an adapter's number implies about the happenstance-core it was built
  against. The brief that raised this filed a lockstep sentence as part of its recommendation and
  then withdrew it on inspection, because CF-32 is FROZEN and requires happenstance-testkit to
  carry its own independent version key for a stated reason - a shared number makes a patch release
  of the contract force every adapter to re-run a conformance bar that did not change, and a rule
  bump on the testkit drags happenstance-core to a number with no earlier release to diff against.
  happenstance-testkit is one of the five crates in xtask's PUBLISHABLE set, so "all five move
  together" is a published promise that contradicts an accepted, machine-checked clause - and
  because CF-32's guard is a manifest check, a lockstep promise written in prose would reinstate
  the coupling in the one medium that check cannot see. What is not decided is what, short of full
  lockstep, tells a consumer which happenstance-core an adapter's version implies. What forces it
  is the next time a consumer reports the two-Event confusion, or phase 12's publish of 0.2.0,
  after which any answer is a decision about already-published numbers rather than about numbers
  still in a pre-release window.
depends_on: []
related:
  - kb-open-question-stale-0-0-0-name-reservations-001
  - kb-decision-0044
  - kb-open-question-testkit-contention-tolerance-001
source_paths:
  - .kb/_intake/remediation-2026-09-04-briefs/adapter-driver-reexport-policy.md
  - spec/SPECIFICATION.md
  - xtask/src/package.rs
  - Cargo.toml
last_reviewed: 2026-09-07
---

# Whether an adapter's version implies a `happenstance-core` version has no answer, and the easy answer collides with CF-32

## What is true today

Every publishable crate in this workspace pins `happenstance-core` at one value
through workspace inheritance — `Cargo.toml:39-41` gives
`happenstance-core = { version = "0.2.0-alpha.1", … }` and the same number to
`happenstance` — with one deliberate exception: `happenstance-testkit` carries
its own `version` key rather than `version.workspace = true`
(`crates/happenstance-testkit/Cargo.toml:21`). That exception is not an
oversight. It is **CF-32**, `[FROZEN]` (`spec/SPECIFICATION.md:8872-8890`),
enforced by a `cargo xtask ci` manifest check that reads the `[package]` table
and fails on an absent `version` key or one that mentions `workspace`. The
clause states its reason at length: under a shared key, adding a conformance
rule bumps the testkit's minor and drags `happenstance-core` to the same
number, republishing an unchanged contract with no earlier version for a
semver-checking tool to diff against; a patch release of the contract likewise
forces every adapter to re-run a bar that did not change. The two numbers mean
different things — the contract's is a promise about types, the testkit's is a
promise about the bar — and CF-32 keeps them independent on purpose.

A remediation brief on adapter driver re-exports (`adapter-driver-reexport-policy.md`)
raised a real and separate consumer-facing hole while investigating a different
question: a consumer who writes `happenstance = "0.3"` and
`happenstance-sqlite = "0.2"` can resolve two `Event` types that print
identically, because nothing states what an adapter's version number implies
about the `happenstance-core` it was built against. The brief's first draft of
Option D tried to close that hole with one prose sentence per publishable
crate, asserting that all five move together — which `Cargo.toml:39-41`
already makes true *in fact*, at the values pinned today, and which the brief
proposed to make true *as a promise*. On inspection that sentence is not
available: `happenstance-testkit` is one of the five crates in
`xtask/src/package.rs`'s `PUBLISHABLE` set (`:86`), so "all five move
together" contradicts CF-32 directly, and it does so in the one place CF-32's
manifest check cannot see it — prose in five READMEs, not a `[package]` table.
The clause would stay satisfied while the documentation promised the opposite.
The brief withdrew the sentence rather than publish a promise its own gate
would not catch a violation of, and its recommendation now covers only the
re-export half of the original question.

## What is not decided

Whether, and how, a consumer should be told what `happenstance-core` version
an adapter's release was built against — short of the lockstep promise CF-32
forecloses. Two sub-questions the withdrawn clause had conflated, now kept
separate: whether the *testkit* moves with the contract (CF-32 already answers
this, no), and whether an *adapter*'s number implies a contract version, which
is the real hole and which CF-32 does not touch either way. No mechanism in
the tree states or checks a weaker relationship — a documented minimum
`happenstance-core` per adapter release, a compatibility table in each
adapter's README, or something else. Resolving this means either amending or
superseding CF-32 with a new record narrow enough to state what an adapter may
promise without reinstating the coupling the clause exists to prevent, or
finding an answer that needs no change to CF-32 at all.

## What forces it

The next consumer report of two `Event` types resolving to different concrete
types under compatible-looking version requirements, or phase 12's publication
of `0.2.0` (`RUNBOOK.md:4681`), after which any change to what a version number
promises is a decision about numbers already on the registry rather than about
numbers still inside a pre-release window that resolves to nothing by default.

## Ordered sub-questions

1. Does the hole need a `[FROZEN]`-clause-level answer at all, or does a
   convention documented in each adapter's README (with no manifest check)
   discharge it well enough?
2. If a clause is wanted, does it amend CF-32 or sit beside it as an
   independent rule about adapter-to-contract version communication?
3. What is the actual mechanism — a compatibility table, a minimum-version
   dependency declaration, a doctest — and does it need a new `cargo xtask ci`
   step the way CF-32 has one?
