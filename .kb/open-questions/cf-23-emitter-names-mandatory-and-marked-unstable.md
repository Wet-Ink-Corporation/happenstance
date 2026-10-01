---
id: kb-open-question-cf-23-emitter-names-unstable-001
title: CF-23 makes naming an emitter mandatory while every emitter is doc(hidden)
kind: open_question
status: superseded
authority_tier: note
summary: >-
  CF-23 is FROZEN and requires the per-test wrapper an adapter's conformance macro invocation
  names to be supplied by the adapter, using one of happenstance-testkit's twelve __emit_*
  macros — the only concrete instances of that parameter anyone has. Every one of those macros
  carries #[doc(hidden)] and a double-underscore prefix, Rust's own declaration that an item is
  not public API and may change without notice. Both readings are published at one commit and
  are irreconcilable: a FROZEN clause requires an adapter author to write a name the crate's own
  convention says may vanish. The instrument gap makes it worse rather than better: #[doc(hidden)]
  is exactly the marker cargo-semver-checks uses to exclude an item from its diff, so the one tool
  that would catch a silent rename is switched off by the attribute the names already carry. The
  page now discloses the contradiction; no clause resolves it. Correction 2 in this corpus applies:
  0.2.0 is live, so the free-to-declare-either-way window the brief assumed has closed, and
  whichever answer is chosen now costs a decision record rather than a documentation edit.
  Resolved 2026-09-30 by kb-decision-0076, executing the policy kb-decision-0066 set: the ten
  conformance emitters lose the __ prefix and doc(hidden) and are public API (emit_tokio and its
  siblings), pinned against a committed list by CF-41's test; __emit_rule_names becomes
  __rule_names and the benchmark pair stays hidden, both outside the promise; __ now means not
  promised. Sub-question 1 is answered by a clause, CF-41, in section 6.6; sub-question 2 by the
  pinned list, which holds whether or not cargo-semver-checks can see a macro; sub-question 3 by
  leaving __rule_names out of the set.
depends_on: []
related:
  - kb-decision-0076
  - kb-decision-0066
source_paths:
  - .kb/_intake/remediation-2026-09-04-briefs/emitter-surface-stability.md
last_reviewed: 2026-09-30
---

# CF-23 makes naming an emitter mandatory while every emitter is doc(hidden)

## What is true today

CF-23 (`spec/SPECIFICATION.md:8852-8863`, `[FROZEN]`) requires that
`happenstance-testkit` "MUST NOT emit any runtime-specific attribute from its
own expansion; the per-test wrapper MUST be a parameter supplied by the
adapter." The twelve `__emit_*` macros the crate ships are the only concrete
instances of that parameter that exist, and `crates/happenstance-cloudflare`
already reaches across a crate boundary to name one:
`happenstance_testkit::event_store_conformance!(… emit =
happenstance_testkit::__emit_wasm, …)`.

Every one of those twelve macros carries, in this order, `#[doc(hidden)]`
then `#[macro_export]` then a `__`-prefixed name. `#[doc(hidden)]` plus a
double-underscore prefix is Rust's conventional way of saying "exported
because the language forces it, not because it is a promise" — the item does
not appear on docs.rs and is excluded from `cargo-semver-checks`'s diff.
Both readings are published at the same commit: CF-23 makes writing one of
these twelve names a `[FROZEN]` obligation on every adapter author, and the
naming convention on the names themselves says they may be renamed or removed
without notice. §6.6, the crate's compatibility policy, governs rule addition
(CF-29), rule meaning-change (CF-31), and the version key (CF-32); it says
nothing about the emitters.

A remediation lane closed the *documentation* half without deciding the
*policy* half: the crate's front page now names all twelve emitters (three
were named before), states plainly that they carry `#[doc(hidden)]`, and
states that this is the attribute `cargo-semver-checks` uses to exclude an
item — so the reader is told the truth, including that the one instrument
that would report a rename as breaking cannot see these names. What the page
does not do, and what no clause does, is say whether a rename is a MAJOR
event.

## What is not decided

Whether `happenstance-testkit` **supports** the twelve emitter names as
stable public surface (a §6.6 clause naming a rename MAJOR, paid for by
`emitter_surface.rs` pinning the names against a committed list, since
`#[doc(hidden)]` itself gives no mechanical enforcement) or **declares them
unstable** (a §6.6 sentence saying they may change in any release, leaning on
CF-30's existing advice to pin the crate exactly — which costs nothing today
and becomes a breaking announcement the day it is written, because a name
shipped twice without a disclaimer is a name people have relied on). A third
option — leaving §6.6 silent and relying on the front page's disclosure — is
what is landed today, and it is the option every accepted playbook in this
corpus about `[FROZEN]` clauses treats as a defect rather than a resting
state.

## What forces it

`0.2.0` is not a free moment for this decision the way it would have been
before publication: `happenstance-testkit` is live at `0.2.0-alpha.1`, so
whichever way this is settled needs a decision record rather than a
documentation edit, and the cost asymmetry runs one direction — declaring the
names unstable is free now and a breaking announcement after the next
release that ships them undisclosed, while declaring them stable forecloses
renaming the two names the brief itself flags as wrong
(`__emit_rule_names`, which wraps no test, and the `__emit_benchmark_*` pair).
The `!Send` Workers author, who has no `#[tokio::test]` arm available, is the
population for whom this is not a convenience but the only door — the
strongest reason to resolve it before a second adapter of that shape exists.

## Ordered sub-questions

1. Does §6.6 acquire a clause at all, or a non-normative paragraph — that is
   the specification pass's call, not this question's.
2. If declared unstable, does `emitter_surface.rs` gain the pin-the-list
   instrument regardless, so a rename is at least a loud diff even though
   `cargo-semver-checks` cannot see it?
3. Does `__emit_rule_names` belong in whichever set is chosen, given it wraps
   no test and is reachable only by the same route as the other twelve?

## Phase 16 — 2026-09-29

**Kept by phase 16, which decides the policy in `kb-decision-0066`. The renames it implies land
in phase 17** (`runbook/phases/17-breaking-window.md`). The names are CF-23's only concrete
parameters, and Cloudflare, having no `#[tokio::test]`, can name only `__emit_wasm`
(`crates/happenstance-testkit/src/lib.rs:75-86`). So whatever 1.0 says about them binds every
adapter author. `cargo-semver-checks` cannot see a `#[doc(hidden)]` macro name. That is why
`__emit_rule_names` and the `__emit_benchmark_*` pair, which this atom flags as misnamed, are
renamed in the `0.4.0` window or never.

Sub-questions 2 and 3, the pin-the-list instrument and `__emit_rule_names`' membership, go with
the renames. **Owner now: phase 17**, to execute the policy. The atom closes when the §6.6 text
and the renames have both landed.

## Closed — 2026-09-30

**Superseded by `kb-decision-0076`** (phase 17, lane L4), which carries the renames ADR-0066's
policy implied. The two readings this atom found published at one commit no longer coexist:

- **The promised names are no longer hidden.** `emit_tokio`, `emit_blocking`, `emit_wasm`,
  `emit_projection_{tokio,blocking,wasm}`, `emit_model_{tokio,blocking}` and
  `emit_concurrency_{tokio,blocking}` render on docs.rs, and `cargo-semver-checks` can see them.
- **§6.6 has a clause.** CF-41 `[FROZEN]` states the set, makes a rename or removal a testkit
  major, and makes `__` mean *not promised*. Its rule,
  `the_promised_emitters_are_exactly_the_pinned_list` in
  `crates/happenstance-testkit/tests/emitter_surface.rs`, compares against a committed list,
  with two negative controls built from the `0.3.2` shape.
- **The names this atom flagged as misnamed are settled.** `__emit_rule_names` is `__rule_names`,
  hidden and outside the set. `__emit_benchmark_tokio` and `__emit_benchmark_blocking` keep their
  names and stay hidden, because CF-34 says a benchmark is not the bar.

The removed names were hidden, so the `0.4.0` semver trace needs a hand-written row for them.
