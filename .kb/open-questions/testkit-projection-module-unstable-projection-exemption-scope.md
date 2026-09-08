---
id: kb-open-question-projection-module-exemption-scope-001
title: ADR-0036's unstable-projection exemption names two crates, and happenstance-testkit is not one of them
kind: open_question
status: accepted
authority_tier: note
summary: >-
  ADR-0036 keeps ProjectionStore off the 0.2.0 freeze by shipping it behind the off-by-default
  unstable-projection feature in happenstance-core and happenstance, forwarded as projection-store
  on adapters — and names only those crates. happenstance-testkit's projection module is
  unconditional (its own doc calls it out as unlike its two nearest templates) and its manifest
  turns unstable-projection on for happenstance-core unconditionally, so on the documents as
  written ProjectionFixture is not gated at all: it is ordinary, published, semver-committed API on
  a trait the port's own governing decision expects to keep moving. That gap surfaces concretely in
  the fixture-declension question this atom is a sibling of — whether a future ProjectionFixture
  capability may be required or must be defaulted turns partly on whether the trait is exempt from
  ordinary minor-bump discipline, and today it demonstrably is not. Distinct from the
  already-resolved question of which crates carry unstable-projection in their default feature set
  (happenstance-sqlite's default was corrected to ["event-store"] alone); this is about whether the
  testkit crate should be inside the exemption's scope at all, and nobody has decided it either way.
depends_on: []
related:
  - kb-decision-0036
  - kb-decision-0042
  - kb-decision-0034
source_paths:
  - .kb/_intake/remediation-2026-09-04-briefs/fixture-declension-policy.md
  - crates/happenstance-testkit/src/lib.rs
  - crates/happenstance-testkit/Cargo.toml
  - .kb/decisions/0036-the-projection-port-ships-gated.md
last_reviewed: 2026-09-07
---

# ADR-0036's unstable-projection exemption names two crates, and happenstance-testkit is not one of them

## What is true today

ADR-0036 (`.kb/decisions/0036-the-projection-port-ships-gated.md`) keeps `ProjectionStore` off the
`0.2.0` freeze by shipping it behind an off-by-default `unstable-projection` feature in
`happenstance-core` and `happenstance`, forwarded as `projection-store` on adapters, with the
semver exemption documented on the module. Read literally, the exemption's scope is those two
crates and the adapters that forward the flag — nothing in ADR-0036's text reaches
`happenstance-testkit`.

`crates/happenstance-testkit/src/lib.rs:471-476` states, in its own words, that the crate's
projection module is "Unconditional, unlike its two nearest templates" — meaning `happenstance-core`
and `happenstance`, the very crates ADR-0036 exempts. `crates/happenstance-testkit/Cargo.toml:49-55`
turns `unstable-projection` on for `happenstance-core` unconditionally, from inside the testkit's
own manifest, rather than forwarding a feature a consumer opts into. Put together: `ProjectionFixture`
and the rest of the projection conformance surface compile and ship on every build of
`happenstance-testkit`, with no feature gate a consumer could decline. On the documents as written,
`ProjectionFixture` is not exempted API — it is ordinary, published, semver-committed API, on a
trait whose governing decision (ADR-0036) explicitly expects the underlying port to keep moving
before it freezes.

This is the premise the fixture-declension question (`kb-decision-0042`) turns partly on: whether a
future `ProjectionFixture` capability may be added as a required item, or must always land defaulted,
depends in part on whether the trait sits inside a semver exemption that tolerates faster movement,
or is bound by the same minor-bump discipline as any other published trait. Today it demonstrably
sits outside the exemption ADR-0036 wrote, whatever anyone intended.

## What is not decided

Whether ADR-0036's exemption should be read as reaching `happenstance-testkit`'s projection module,
or whether the testkit crate is deliberately outside it and the movement risk is accepted as an
ordinary cost of a pre-1.0 minor bump. Widening ADR-0036's scope to cover the testkit would be a
widening of an accepted atom and would need its own superseding atom rather than a reinterpretation
— ADR-0036 is accepted and immutable, and nothing in it can be stretched by argument alone. Leaving
it as written means `ProjectionFixture` and its sibling declension policy (`kb-decision-0042`) are
governed by ordinary published-API discipline with no gating cushion, which is a stricter bar than
the port it conforms to is held to.

This is distinct from the already-settled question of which crates carry `unstable-projection` in
their *default* feature set — `happenstance-sqlite`'s default was corrected to `["event-store"]`
alone specifically because it had been forwarding the gate on, defeating off-by-default for the one
crate most consumers install. That fix does not touch `happenstance-testkit`, whose module is
unconditional rather than merely defaulted-on.

## What forces it

The next `ProjectionFixture` capability the projection port needs — `POLL_BUDGET`-shaped or
otherwise — is the first case where the answer changes what is allowed to ship. If the testkit is
inside ADR-0036's spirit even though its text does not name it, a required addition costs less than
ordinary published-API discipline would charge; if it is not, `kb-decision-0042`'s "everything
future is defaulted" rule is the only thing standing between a testkit minor and a source break in
every out-of-tree fixture. Whoever settles the fixture-declension question's next capability should
settle this one first, because it changes what that decision is allowed to cost.
