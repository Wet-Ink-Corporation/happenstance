---
id: kb-decision-0059
title: The rendered surface stops indexing EVENT_TYPES by position, and a lint keeps it that way
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0059
reversibility: medium
phase: 12
supersedes: null
superseded_by: null
summary: >-
  Every rendered doctest and example lifts the already-compiling named-const plus
  membership-guarded-decode pattern instead of subscripting EVENT_TYPES, and a
  repo-local xtask lint holds the line. The audit's census was wrong — 17 of 22
  non-doctest impls already used the reorder-immune pattern — but what is
  categorical is the copy-pasteable surface: all seven rendered doctests and all
  five examples indexed by position. assert_domain_event gaining a positional-
  agreement precondition is a breaking change to a published function and is owed
  before 0.2.0; the additive sibling is the fallback if that deadline passes.
depends_on:
  - kb-decision-0020
  - kb-decision-0032
related:
  - kb-open-question-event-type-positional-mapping-001
  - kb-governance-what-may-refute-a-finding-001
  - kb-open-question-d-1-no-total-path-001
source_paths:
  - .kb/_intake/remediation-2026-09-04-briefs/domain-event-guard-and-decode.md
  - .kb/_intake/ratifications-2026-09-06-pre-publication.md
last_reviewed: 2026-09-07
---

# The rendered surface stops indexing EVENT_TYPES by position, and a lint keeps it that way

## Context

`DomainEvent::event_type()` is returned by value precisely because a `&'static
EventType` would force implementations to index `EVENT_TYPES` by position — the
trait's own rationale (`crates/happenstance/src/domain.rs:70-75`) names indexing
as the hazard it exists to remove. Every rendered doctest and every `examples/`
impl then indexed `EVENT_TYPES` by position anyway. Reordering the array —
alphabetising it, say — silently mislabels every affected event on disk while
`commit` returns `Ok`: `Boundary::query` derives one `QueryItem` over the whole
declared set regardless of order, `decode` ignores the envelope's `event_type` at
every copy-pasted site, and the worked example's own transcript test asserts
membership only. `assert_domain_event`, the guard ADR-0020 names as the
mitigation, reads only `event_type()` per element with no cross-element or
positional check, so it cannot see a reorder — only a value absent from
`EVENT_TYPES` entirely, which is the narrower residual ADR-0020 actually states.

A second hazard rides beside it: every rendered `decode` implementation discards
its `event_type` parameter, including the crate root's first program
(`lib.rs:44-45`). That throws away the exact mechanism ADR-0032's carried-forward
Decision 3 relies on to earn "no read-path hook is needed" — decode trying the
current payload shape and falling back to an older one keyed on the envelope's
own type.

## What the corrected census changed

The audit's raw `grep -c` counts overstated the problem: of 22 real (non-doctest)
`impl DomainEvent` blocks, 17 already use named `const EventType` items —
`crates/happenstance/src/testing/tests.rs:27-53` is the crate's own house style,
and it is immune to a reorder by construction. What *is* categorical, and is what
this decision addresses, is the surface consumers actually copy: all seven
rendered doctests and all five `examples/` impls index by position, and eight
copy-pasteable `decode` sites discard the type parameter. The tree already
contains proven, compiling remedies for both — the named-const pattern above, and
the membership-guarded `decode` at `crates/happenstance/tests/projection_runner.rs:75-86`.

## Decision

Rewrite every rendered doctest and both worked examples to the named-const
pattern, lift the membership-guarded `decode` into the crate root's first program
and both examples, and add an `xtask` lint step forbidding `EVENT_TYPES[` in any
`///`, `//!`, or `examples/` line. This is a documentation fix with no semver
class — no signature moves and no existing test starts failing — and it is
strictly upstream of any run-time guard: fixing what the docs teach is cheaper
and more durable than detecting the consequence of teaching it wrong. It does
**not** make `decode` envelope-*dispatched* — serde's variant identifier remains
the live discriminator, and the rename hazard (renaming a Rust variant
permanently orphans already-written payloads) survives this decision intact.

`assert_domain_event` additionally gains a positional-agreement precondition —
`every_variant[i].event_type() == EVENT_TYPES[i]` — **before `0.2.0`**, because
`happenstance` `0.2.0-alpha.1` is already published and pre-release is the only
licence this repository will ever have to change a published function's
behaviour for free. The measured in-tree cost is zero broken calls: the guard has
two real call sites and one doctest call, all three positionally correct today.
The break, if taken, lands as a red test in a consumer's `happenstance::testing`
usage rather than as a production failure. This B-strengthening carries a real
cost the taught-shape fix does not: the check has no reflection over enum
variants, so its diagnostic cannot distinguish "the impl permuted" from "the
caller listed variants in a different order," and the published doc states no
ordering precondition today. If the pre-`0.2.0` window closes before this lands,
the fallback is an additive sibling entry point (`assert_domain_event_in_order`
or similar) carrying the precondition from its own first day, accepting that the
canonical name then keeps the weaker check permanently — the same
two-constructors-enforcing-different-rules shape ADR-0020 already names as a
defect, now applied to assertions rather than constructors.

## Consequences

Neither ADR-0020 nor ADR-0032 is superseded: ADR-0020's membership residual
stands as stated, and this decision extends it to a hazard ADR-0020 never priced
rather than contradicting it; ADR-0032's Decision 3 is corroborated, not revised.
The specification is silent here by design — `DomainEvent`, `EVENT_TYPES`, and
`assert_domain_event` sit entirely in the typed layer, outside the clause space
`cargo xtask spec-trace` governs, so this decision's compliance is checked only by
the doctests, the examples, and the new lint. Left open: whether `decode` should
become genuinely envelope-dispatched (a materially larger question about what
`DomainEvent` is), and whether `assert_domain_event` should also exercise
`tags()`, `encode`, or `decode`.
