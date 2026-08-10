---
id: concept-proof-artefact-vs-green-gate
title: A proof artefact, and why a green gate is not one
kind: concept
status: accepted
authority_tier: note
summary: >-
  A proof artefact is something that WOULD NOT EXIST if the design under test were wrong. A
  green gate is not: it tests the code that exists. Four of one plan revision's fifteen
  artefacts failed this test, the worst being a `todo!()` skeleton — which compiles against
  any signature, because `!` coerces to everything.
depends_on: []
related:
  - playbook-the-phase-protocol
  - governance-an-adapter-must-pass-the-suite
source_paths:
  - docs/RUNBOOK.md
  - docs/evaluation/PRESSURE-TEST.md
  - xtask/src/proof.rs
last_reviewed: 2026-08-09
---

# Proof artefact versus green gate

## The definition

An artefact proves a design only if a **wrong** design would have failed to produce it.
`cargo xtask ci` being green is a precondition for looking at a phase's exit criteria,
never one of them.

## The four that failed the test

- A skeleton of `todo!()` bodies. `todo!()` has type `!`, and `!` coerces to everything, so
  the skeleton compiles against *any* signature. "The six skeletons compile" is satisfied by
  a wrong freeze exactly as readily as by a right one.
- A Markdown document recording outcomes. It exists whatever the outcomes are.
- An assertion that did not discriminate the question its ADR asked, because both flavours'
  bare half is identical whether the second is derived or hand-written.
- An exit criterion with no artefact at all.

## The same failure one level down

A gate step can be decorative too. `cargo test` exits 0 on `running 0 tests`, so a target
truncated to its `#![cfg(…)]` attributes passes a step that a *deleted* target fails. That
is why `cargo xtask proof-artefact` asserts the named tests out of `--list` **before**
running them: a step that a deletion fails and an emptying passes is checking the filename.

## The corollary for rules

A rule that no adapter can fail is decorative. Before adding one, name the plausible wrong
implementation it rejects — and write it down.
