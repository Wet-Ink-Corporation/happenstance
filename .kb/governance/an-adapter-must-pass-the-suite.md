---
id: governance-an-adapter-must-pass-the-suite
title: An adapter that has not run the conformance suite is not an adapter
kind: governance
status: accepted
authority_tier: decision
summary: >-
  Any new event store adapter must invoke `happenstance_testkit::event_store_conformance!`
  and pass it before it is considered to exist. Compiling is not the bar. A skeleton with
  real associated types and `todo!()` bodies is an INSTRUMENT — something for the type
  checker to disagree with — and never a shipped adapter.
depends_on:
  - adr-0010-the-suite-must-prove-itself
related:
  - playbook-adding-a-conformance-rule
  - map-the-instrument-portfolio
source_paths:
  - CLAUDE.md
  - crates/happenstance-testkit/src/suite.rs
last_reviewed: 2026-08-09
---

# An adapter that has not run the suite is not an adapter

## The rule

```rust
happenstance_testkit::event_store_conformance!(MyFixture::new());
```

The expression builds a **`Fixture`**, not a store. One fixture instance is one isolated
backing store; each `connect()` on it is one handle onto that store. A fixture also
declares `SECOND_HANDLE` and `REOPEN` as `Capability` associated constants, and a rule
whose capability is declined still runs, reporting the fixture's stated reason rather than
vanishing from the binary.

If a rule seems wrong, fix the rule and explain why in the same change. Skipping it is not
available.

## Why compiling is not the bar

A skeleton whose bodies are all `todo!()` compiles against **any** signature, because
`todo!()` has type `!` and `!` coerces to everything. So "it compiles" is satisfied by a
wrong freeze exactly as readily as by a right one. This was the worst of the four
decorative proof artefacts a previous plan revision shipped.

## The reachability check

`cargo xtask lints` and `cargo xtask spec-trace` are wired as redkiln's
`verify.reachability_static`, because this rule is the library form of the failure that
tripwire exists for: the most common way for a feature to be done and still not exist is
that nothing imports it.
