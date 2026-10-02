---
id: kb-open-question-adapter-version-lockstep-001
title: Whether an adapter's version implies a happenstance-core version has no answer, and the easy answer collides with CF-32
kind: open_question
status: superseded
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
  Resolved 2026-09-29 by kb-decision-0066, on the owner's decision. All crates release in lockstep at
  1.0.0 and version independently after that. Each adapter declares the minimum core it needs
  (happenstance-core = "1.N"), and a minimal-versions CI job, which is phase 17's work, keeps those
  declared lower bounds honest. Each adapter's README states "conformant" as "passes
  happenstance-testkit X.Y". CF-32 stands untouched. CF-29 and CF-31 are kept: a new rule that
  detects violation of an already-frozen clause is a testkit minor, and a new requirement is a clause
  change, an ADR and a major. An adapter's major follows the breaking version of any driver it
  re-exports (ADR-0044).
depends_on: []
related:
  - kb-open-question-stale-0-0-0-name-reservations-001
  - kb-decision-0044
  - kb-open-question-testkit-contention-tolerance-001
  - kb-decision-0066
  - kb-decision-0057
source_paths:
  - .kb/_intake/remediation-2026-09-04-briefs/adapter-driver-reexport-policy.md
  - spec/SPECIFICATION.md
  - xtask/src/package.rs
  - Cargo.toml
  - crates/happenstance-testkit/Cargo.toml
  - CHANGELOG.md
last_reviewed: 2026-09-29
---

# Whether an adapter's version implies a `happenstance-core` version has no answer, and the easy answer collides with CF-32

## What is true today

Every publishable crate in this workspace pins `happenstance-core` at one value
through workspace inheritance — `Cargo.toml:39-41` gives
`happenstance-core = { version = "0.2.0-alpha.1", … }` and the same number to
`happenstance` — with one deliberate exception: `happenstance-testkit` carries
its own `version` key rather than `version.workspace = true`
(`crates/happenstance-testkit/Cargo.toml:21`). That exception is not an
oversight. It is **CF-32**, `[FROZEN]` (`spec/SPECIFICATION.md:9267-9285`),
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

## Closed — 2026-09-29

`kb-decision-0066`, the 1.0 charter, answers this on the owner's decision of 2026-09-29. What tells
a consumer which `happenstance-core` an adapter implies is the manifest, not prose. Each adapter
declares the lowest core it works with as an ordinary dependency requirement
(`happenstance-core = "1.N"`), and Cargo's resolver enforces it. A minimal-versions CI job, added
by phase 17, resolves every declared lower bound and builds against it, so a requirement that
claims more than the adapter needs fails in CI rather than on a consumer's machine. Each adapter
already re-exports the core it was built against — `pub use happenstance_core;` at
`crates/happenstance-sqlite/src/lib.rs:148`, `crates/happenstance-postgres/src/lib.rs:230` and
`crates/happenstance-neon/src/lib.rs:173`, and `pub use {happenstance_core, worker};` at
`crates/happenstance-cloudflare/src/lib.rs:589` — so a consumer who reaches core through the adapter
cannot end up holding the second `Event` this atom opens with.

The sub-questions:

1. **Both, and neither alone.** The version relationship lives in a manifest fact with a CI check,
   and the conformance claim lives in a README sentence ("passes `happenstance-testkit` X.Y"). No
   new `[FROZEN]` clause is needed. This avoids the failure that got the lockstep sentence
   withdrawn, which was prose that no check can see.
2. **Beside CF-32, not amending it.** The 1.0.0 lockstep is a single release, not a promise that the
   numbers move together afterwards, so it does not bring back the coupling CF-32 exists to prevent.
   The testkit keeps its own `version` key (`crates/happenstance-testkit/Cargo.toml:36`), and after
   1.0 it moves on its own schedule. What a testkit number *means*, now that a rule can turn a
   passing adapter red, is settled by keeping CF-29 and CF-31. A rule that detects violation of a
   clause already `[FROZEN]` is a testkit minor. A new requirement needs a clause change, which
   means an ADR and a major. `CHANGELOG.md:20-25` already tells consumers to pin the testkit
   exactly.
3. **A minimum-version dependency declaration, checked by a minimal-versions job.** The release
   tooling that bumps the numbers (`release-plz` or `cargo-release`) is chosen at phase 21.

**Two limits, named in the charter rather than left to be found.** An adapter that re-exports its
driver under ADR-0044 takes a major whenever that driver breaks, whatever the core did.
`happenstance-core`'s own public dependency `futures-core 0.3` is still `0.x`, and the charter
carries that as an accepted risk. **Owed and not done here:** ADR-0057 dropped the testkit's version
key from the root manifest, but `Cargo.toml:62` still carried `version = "0.3.2"` on the testkit's
workspace dependency. Phase 17 executed it in its first breaking change (2026-09-30): the key is
gone, and `cargo xtask package-check` refuses a publishable crate whose testkit line has one. Closed by hand in phase 16. No accepted decision was
edited.
