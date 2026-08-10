---
id: playbook-adding-a-conformance-rule
title: "Playbook: adding a conformance rule"
kind: playbook
status: accepted
authority_tier: guideline
summary: >-
  A rule that no adapter can fail is decorative. Before adding one, name a plausible wrong
  implementation it rejects and write that implementation into the testkit's own `tests/`
  if it is not already there. Never assert on literal position values; the specification
  permits gaps, and asserting on one converts a MAY into a MUST with no ADR behind it.
depends_on:
  - adr-0010-the-suite-must-prove-itself
related:
  - governance-an-adapter-must-pass-the-suite
  - adr-0013-position-assignment-and-visibility
source_paths:
  - CLAUDE.md
  - crates/happenstance-testkit/tests/mutation_coverage.rs
last_reviewed: 2026-08-09
---

# Adding a conformance rule

## The steps

1. **Name the wrong implementation the rule rejects.** If you cannot, the rule is
   decorative and does not belong in the suite.
2. **Write that wrong implementation into the testkit's own `tests/`**, unless one is
   already registered. The mutant registry is the proof: every rule has a mutant that
   fails it, and every mutant fails exactly its declared rules.
3. **Add the changelog entry naming the defect the rule detects** (CF-29). This is a gate
   step, not a courtesy.
4. **Cite the rule from its clause** in `SPECIFICATION.md`, and let `cargo xtask
   spec-trace` confirm the citation resolves.

## The two prohibitions

**No literal position values** (CF-6). The specification permits gaps (VT-11), so a rule
asserting `[1, 2, 3]` converts a `MAY` into a `MUST` without an ADR. Compare against the
positions the store actually assigned. The grep in `cargo xtask lints` is the cheap second
line; the real enforcement is `GappedPositionStore` in the registry.

**No clock** (CF-33). A conformance rule does not read time. The lint is scoped to
`crates/happenstance-testkit/src` and deliberately not the whole crate: `tests/` is where
the concurrency racers live, and a racer may legitimately need to synchronise.

## Why the lints are greps, and what that costs

Each checks something no type can express. Each states, in its own documentation, what it
does *not* verify — because a check whose limits are undocumented is read as a guarantee.
The scanner blanks comments and string contents so that prose about a construct does not
read as the construct; it is not a lexer, and it fails loudly on the four constructs that
would defeat it rather than silently ceasing to report.
