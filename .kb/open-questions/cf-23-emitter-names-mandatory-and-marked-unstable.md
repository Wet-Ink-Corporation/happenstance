---
id: kb-open-question-cf-23-emitter-names-unstable-001
title: CF-23 makes naming an emitter mandatory while every emitter is doc(hidden)
kind: open_question
status: accepted
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
depends_on: []
related: []
source_paths:
  - .kb/_intake/remediation-2026-09-04-briefs/emitter-surface-stability.md
last_reviewed: 2026-09-07
---

# CF-23 makes naming an emitter mandatory while every emitter is doc(hidden)

## What is true today

CF-23 (`spec/SPECIFICATION.md:8474-8485`, `[FROZEN]`) requires that
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
