---
id: kb-open-question-es-23-adapter-half-001
title: ES-23 carries two MUSTs on two owners; FROZEN_DOC_MUSTS has a disposition for only one of them
kind: open_question
status: accepted
authority_tier: note
summary: >-
  ES-23 is FROZEN and requires both a port-level MUST (the contract documents cancellation
  semantics in an explicit # Cancellation section) and an adapter-level MUST (each adapter states
  which of the two cancellation behaviours it provides). xtask/src/lint_narrative.rs's
  FROZEN_DOC_MUSTS array pins only obligations that fall on the contract's own documentation by
  its own derivation rule, so it carries one Pinned row for the port half and nothing for the
  adapter half — not Excluded, the disposition VT-21/VT-22/VT-24 use for wholly adapter-owned
  obligations, just absent. That absence went unnoticed from phase 8 (happenstance-sqlite
  shipping) through this lane, past two named instruments that should have caught it: ADR-0012's
  own proposed gate step (never built) and its named fallback, the phase 8-11 per-adapter review
  (which did not produce the statement). happenstance-sqlite has since discharged its half via
  kb-decision-0058's Cancellation section; happenstance-cloudflare's was landed by the lane that
  raised this question. What is open is whether FROZEN_DOC_MUSTS gains a third disposition for an
  obligation that falls on an adapter and is checked by a per-adapter test the array can only
  point at rather than run, or whether the array's scope stays as documented and the gap is
  accepted as a known limit of what xtask can see. Recommended at medium confidence: add the
  disposition, sequenced before phase 12, owned by whoever owns xtask's narrative pin.
depends_on: []
related:
  - kb-decision-0058
  - kb-decision-0012
source_paths:
  - .kb/_intake/remediation-2026-09-04-briefs/frozen-doc-musts-and-the-adapter-half.md
last_reviewed: 2026-09-07
---

# ES-23 carries two MUSTs on two owners; FROZEN_DOC_MUSTS has a disposition for only one of them

## What is true today

ES-23 is `[FROZEN]` and reads: "The port MUST document this in an explicit `# Cancellation`
section, and each adapter MUST state which of the two it does." The port half is discharged at
`crates/happenstance-core/src/store.rs`. The adapter half was discharged by neither shipping
adapter from phase 8 (`happenstance-sqlite`'s promotion) through phase 9
(`happenstance-cloudflare`'s) until the lane that raised this question landed it in both:
`kb-decision-0058` records `happenstance-sqlite`'s `# Cancellation` section, added alongside the
decision to keep the write path inline, stating today's true answer — a dropped `append` future
provably commits nothing, because `append` contains no `.await`. The Cloudflare crate's statement
landed in the same session, in its own store module with a crate-root pointer, plus a test target
per adapter that fails if the section, the pointer, or the statement's truth (no `.await` entering
`append`) goes missing.

`xtask/src/lint_narrative.rs`'s `FROZEN_DOC_MUSTS` array is what `cargo xtask ci` checks against
this class of obligation. Its derivation rule pins a candidate when the obligation falls on "the
contract's own documentation — not on an adapter's, not on a fixture's." By that rule, ES-23
appears once, `Pinned` to the port site; the adapter half falls outside scope by construction and
is not a bug in the rule as stated. VT-21, VT-22 and VT-24 — obligations wholly owned by a store —
are `Excluded`, with the reason written into the array. ES-23 is the first clause the array has
seen that splits: one obligation `Pinned`, the other neither `Pinned` nor `Excluded`, just missing
from the table entirely.

That absence survived two named instruments that should have caught it.
`references/adr/0012-append-shape-and-preconditions.md:349-364` proposes a gate check for exactly
this ("catches a missing `# Cancellation` section," with the ceiling stated in the same breath —
"cannot catch a section that lies") and records in its own words that the step "does not exist and
has not been written," naming the phase 8-11 per-adapter review as the fallback. Both were
undischarged: the step was never built, and the two reviews did not produce the statement.

## What is not decided

Whether `FROZEN_DOC_MUSTS` gains a **third disposition** — an obligation that falls on an
adapter's documentation, discharged today by a named per-crate test the array cannot run but can
point at — making ES-23 the array's first two-row clause. The argument for: a reader scanning the
array for "which `[FROZEN]` documentation MUSTs are met" currently reads one `Pinned` row against
a clause with two obligations and has no way to learn the second exists at all, let alone whether
it is met; a missing row and an `Excluded` row read identically as silence when the whole point of
`Excluded` is that it is a *stated* silence. The argument against, on file and not fully answered:
`xtask` was scoped away from reaching into adapter crates deliberately, to avoid rows that rot the
moment an adapter is renamed or a third one ships, and a disposition that only *points at* a test
in another crate is documentation wearing a gate's clothes — the decorative-check failure
`xtask/src/main.rs`'s own module documentation warns against. The partial answer offered: the
adapter half is now genuinely checked, per adapter, by the two landed test targets; what the new
disposition would buy is not verification but making the array's completeness claim honest, which
the objection's author would accept is worth something, just perhaps less than a non-checking row
costs.

## What forces it

`happenstance-postgres` and `happenstance-neon` reaching the point of shedding their skeleton
marker, each of which will owe the identical ES-23 statement with nothing in the tree pointing a
new adapter author at the obligation or the two existing test targets to copy. The two-phase gap
already measured (phase 8 to this lane) is the cost of leaving the array as documented; a third and
fourth adapter repeating it is what raises that cost from "one embarrassing gap, since closed" to
"a pattern this array reliably misses."
