---
id: kb-concept-mutation-kind-tethered-bar-001
title: A mutation kind needs a bar tethered to its subject
kind: concept
status: accepted
authority_tier: note
summary: >-
  Kind::StatedOnlyDefect was withdrawn because its bar — an untethered fn() ->
  String observed/control pair plus a certifies-list check — is satisfiable by a
  defect-free subject whose two pointers are string literals, and adding the
  positive control the review demanded does not close it. Witness escapes this
  because it holds the store's own function; there is no analogue for a
  fixture-level defect because Fixture is not dyn-compatible. What replaced the
  kind is a named test that states rather than falsely checks.
depends_on:
  - kb-decision-0010
related:
  - kb-decision-0042
  - kb-open-question-model-only-kind-memberless-001
  - kb-open-question-cf-17-cf-14-markers-001
source_paths:
  - .kb/_intake/remediation-2026-09-04-briefs/stated-only-defects-and-the-reopen-must.md
last_reviewed: 2026-09-07
---

# A mutation kind needs a bar tethered to its subject

## The claim

A mutation-testing kind is only as sound as the *tether* between the pointers it
asks an author to supply and the subject those pointers are supposed to describe.
`Kind::StatedOnlyDefect` was invented to record a defect that cannot be driven —
`REOPEN`'s over-claiming hazard has no port-observable consequence, unlike
`MID_BATCH_FAULT`, whose armed fault makes the append itself answer `Err`. The bar
written for it asked for three things: an empty, measured `fails` list; a
`STATED_ONLY_DEFECTS` row whose `observed` and `control` — both plain `fn() ->
String` — answer one fixed scenario differently; and an assertion that the row's
`certifies` list still passes. None of the three is behind a `cfg`, so the kind
looked unspoofable in the way `Kind::ModelOnlyMutant`'s `cfg`-gated third
obligation was not.

It was spoofable anyway, by a different route. `observed` and `control` carry no
type-level tie to any store, fixture, or `Defect`: the whole kind is satisfied by
a subject with no defect at all, twice over — once by pointing `control` at a
defective third store while `observed` names an honest one, and once in the
degenerate case where both pointers are string literals that touch nothing. The
repair a reviewer reached for first — add the positive control
`Kind::ModelOnlyMutant` already has — does not close it either: that control only
catches the *degenerate* spoof, because a literal happens to differ from the
honest answer, and pointing the spoof row's `control` at the honest fixture while
keeping `observed` as a literal still passes both the scenario check and the
positive control.

## Why `Witness` escapes this and a fixture-level defect cannot

`Witness` ties its two functions to `<T as Defect>::select`, the store's own
function — the tether is the trait's associated function, not a bare pointer, so
a spoof would have to supply an actual value of the actual type. There is no
equivalent for a fixture-level defect: `Fixture` returns `impl Trait` in its own
trait methods (RPITIT), which makes it not `dyn`-compatible, so a fixture cannot
be held as data the way a `Defect` implementor's function can. A kind whose
soundness depends on tethering the observed/control pair to the subject has
nothing to tether to when the subject is a fixture, which is exactly the case
`Kind::StatedOnlyDefect` existed for.

## What replaced it

The kind was withdrawn rather than repaired. What is left is a single named test,
`reopen_over_claiming_is_undetectable_and_this_is_the_record`, driving
`NoopReopenFixture` — kept deliberately outside both `for_each_mutant!` and the
fixture registry — through every rule and asserting three things that *can*
honestly be checked: it fails none of them; it converts three durability rules
from reported skips into passes while an honest twin one line apart still reports
them as skips; and it answers the sharpest proposed scenario identically to an
honest, durable-shaped fixture (verified: `happenstance-sqlite`'s own `reopen`
keeps a caller-held connection live against the same file, landing on the liar's
side of that scenario alongside it — proof the scenario separates styles of
`reopen`, not honest from defective). The test states the defect is undetected
rather than pretending an untethered pair detects it.

## The general lesson

Before inventing a mutation kind for a defect that resists driving, ask what the
observed/control pair is tied to. If the answer is "nothing but a promise the
author will fill it in honestly," the kind is unsound regardless of what `cfg`
gates or positive controls are added around it — those harden the wrong seam. A
kind's bar is only as strong as the type system's ability to make the honest
pointers the only pointers that compile.
