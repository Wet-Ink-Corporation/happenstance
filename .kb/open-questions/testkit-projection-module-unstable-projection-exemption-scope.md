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
  2026-09-11: ADR-0063 (kb-decision-0063) freezes the port and retires the exemption this question
  is scoped to, so the premise dissolves — ProjectionFixture is a committed surface on a frozen
  trait, which is the ordinary case rather than the anomaly this atom described. The brief records a
  third xtask test holding that no in-tree crate still forwards the retired feature; at 86a410c
  crates/happenstance-testkit/Cargo.toml still forwards it, so the atom is annotated rather than
  closed until the lane's manifest is readable.
depends_on: []
related:
  - kb-decision-0036
  - kb-decision-0042
  - kb-decision-0034
  - kb-decision-0063
source_paths:
  - .kb/_intake/remediation-2026-09-04-briefs/fixture-declension-policy.md
  - crates/happenstance-testkit/src/lib.rs
  - crates/happenstance-testkit/Cargo.toml
  - .kb/decisions/0036-the-projection-port-ships-gated.md
  - .kb/_intake/2026-09-11-adr-0063-the-projection-port-is-frozen.md
last_reviewed: 2026-09-11
---

# ADR-0036's unstable-projection exemption names two crates, and happenstance-testkit is not one of them

## What was true when this was written (2026-09-07)

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
or is bound by the same minor-bump discipline as any other published trait. On 2026-09-07 it
demonstrably sat outside the exemption ADR-0036 wrote, whatever anyone intended.

## What ADR-0063 does to the premise (2026-09-11)

ADR-0063 (`kb-decision-0063`) freezes the port. `happenstance-core`'s `projection` module and its
re-exports become unconditional; a signature change on `ProjectionStore`, its value types or
`ProjectionProbe` is a breaking change with a decision record behind it, which is what *frozen*
already means for `EventStore`. The `unstable-projection` feature name stays on `happenstance-core`
because removing a Cargo feature is itself breaking and a `0.2.0` manifest names it — but it is
`= []`, gating nothing. ADR-0036 is discharged rather than superseded: its reason (gate the port
until PS-2's bar is met) was right when written, and ADR-0062 met the bar.

That removes the thing this question was about. The question asked whether `happenstance-testkit`
should be *inside* ADR-0036's exemption; ADR-0063 retires the exemption from the port it covered, so
there is no longer an exemption for the testkit to be inside or outside of. `ProjectionFixture` is a
committed, published surface over a frozen trait. The atom above described that as the anomaly —
the one crate held to a stricter bar than the port it conforms to — and under ADR-0063 it is the
ordinary case: every crate in the projection family is now held to the same bar, and the
fixture-declension policy (`kb-decision-0042`) governs `ProjectionFixture` on the same footing as
any other published trait, with no gating cushion to argue about.

The testkit's *manifest* forward is a separate, mechanical residue. The brief records that the two
`xtask` tests which held the gate on are inverted rather than deleted, and that a third test holds
that no in-tree crate still forwards the retired feature — needed because a forward of an empty
feature compiles silently. `crates/happenstance-testkit/Cargo.toml:70` is exactly such a forward,
and it is the line this atom's summary cited as evidence. Whether the lane removes it, and whether
the manifest comment at `Cargo.toml:20-28` that still describes the family as "fenced rather than
hidden" with "a written semver exemption" is rewritten, is what the third test decides. The
`lib.rs:471-476` comment stays true under ADR-0063 for a reason unrelated to the exemption: the
module is unconditional so the wasm32 `--tests` step cannot pass while proving nothing.

## What is not decided

Nothing about the exemption's scope, any longer — that is ADR-0063's to have dissolved. What is
still open is narrower and verifiable: at this worktree's `HEAD` (`86a410c`) the long form
`references/adr/0063-the-projection-port-is-frozen.md` is absent, `crates/happenstance-core/Cargo.toml:110`
still reads `conformance = ["unstable-projection"]`, and `crates/happenstance-testkit/Cargo.toml:70`
still forwards the feature. The brief describes a third `xtask` test that would make that last line
fail the gate; whether it does, and what the testkit's manifest and its header comment say after
the lane lands, cannot be read from here. This atom therefore stays open as an annotation rather
than being closed on the brief's word — the same discipline `kb-decision-0063`'s own provenance
note applies to itself.

## What closes it

A `HEAD` at which `crates/happenstance-testkit/Cargo.toml` no longer forwards `unstable-projection`
(or forwards it with a comment saying why an empty feature is deliberately named), the third `xtask`
test exists and is green, and the manifest header no longer describes a semver exemption on the
projection family. At that point the question has no subject and the atom should be closed with a
pointer to `kb-decision-0063`. Reopened only if ADR-0063 itself is reopened — by a breaking change
to `ProjectionStore` that an adapter the suite passes turns out to need — because that would
re-raise, in a new form, whether the testkit's fixture trait moves with the port or ahead of it.
