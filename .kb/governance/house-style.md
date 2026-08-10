---
id: governance-house-style
title: House style — what the lints do not say
kind: governance
status: accepted
authority_tier: guideline
summary: >-
  No `unwrap`/`expect` in library code and no `anyhow` in library crates; every public item
  documented and every fallible public function carrying an `# Errors` section;
  `#[non_exhaustive]` on public types that will grow; comments explain WHY; a runnable
  doctest is preferred to a described one, because a doctest cannot rot.
depends_on: []
related:
  - governance-an-adapter-must-pass-the-suite
source_paths:
  - CLAUDE.md
  - clippy.toml
last_reviewed: 2026-08-09
---

# House style

## The rules

- No `unwrap` / `expect` in library code. Tests and fixtures may, with
  `#![allow(clippy::unwrap_used)]` scoped to the test module.
- No `anyhow` in library crates. `xtask` and examples may use it.
- Every public item documented; `missing_docs` is a warning and CI denies warnings. Every
  fallible public function needs an `# Errors` section.
- `#[non_exhaustive]` on public structs and enums that will grow.
- Comments explain *why*, not *what*. Prefer one comment naming the constraint over three
  narrating the code.
- Doctests are documentation that cannot rot — prefer a runnable example to a described
  one. Feature-gated examples belong in the feature-gated module, so they are only
  compiled when the feature is on.

## The audience this is written for

The repository owner has 25+ years in distributed systems, databases and enterprise
architecture, and is fluent in C#, Python and JavaScript — and is **new to idiomatic
Rust**. So: do not explain event sourcing, CQRS, consistency boundaries or concurrency
control. **Do** explain Rust-specific reasoning — why a newtype instead of a type alias,
why `NonZeroU64` earns its keep, what coherence is refusing to allow, why a lint is on,
why a lifetime is where it is. When a construct is unusual, say what the alternative was
and why it lost.
