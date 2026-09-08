---
id: kb-reference-spec-trace-unresolved-declarations-001
title: Forty-five rule-name declarations across twenty-eight clauses resolve against nothing, in three kinds
kind: reference
status: accepted
authority_tier: note
summary: >-
  With spec-trace's prose guard retired, check 4 resolves every rule name a clause cites against
  the real specification and finds 45 unresolvable across 28 clauses: 26 Elsewhere(path) — a real
  test in a file no resolution source reads — 4 NotARuleName harvested out of prose, and 15
  Scheduled(who). Also measured: the bare dagger convention fires on zero clauses; only the
  backticked `new` term does the PS family's work.
depends_on: []
related:
  - kb-open-question-dagger-convention-vs-maturity-markers-001
  - kb-reference-spec-trace-has-suite-001
  - kb-open-question-es-6-unwritable-rule-001
  - kb-open-question-cf-36-unperformed-cross-reference-001
  - kb-playbook-ratchet-gate-landing-001
source_paths:
  - .kb/_intake/remediation-2026-09-04-briefs/prose-guard-retired-and-what-it-owes.md
last_reviewed: 2026-09-07
---

# Forty-five rule-name declarations resolve against nothing, in three kinds

## What this is a pointer to

`xtask/src/spec_trace.rs`'s `rules_of` carried a loop guard — `c.schedules_new || !has_suite(&c.id)`
— that a lane retired while making `has_suite` admit the `PS` family. Retiring the guard makes
check 4 resolve every rule name a `spec/SPECIFICATION.md` clause cites, rather than skipping
clauses the guard judged not-yet-real. This atom is the census that retirement surfaced, taken in
this working tree at `9b06836`. It is a measurement, not a verdict on any clause; the residual
question about the dagger convention is `kb-open-question-dagger-convention-vs-maturity-markers-001`.

## What the guard's two terms were doing, measured by instrumenting `rules_of` over the real specification

| Term in the guard | Clauses it fired on (all families) |
|---|---|
| `(new)` | 9 |
| a bare `†` in the `Rule:` line | **0** |
| a leading `new ` | **0** |
| backticked `` `new` `` | 10 |
| `unit test` | 6 |
| `compile test` | 5 |
| `meta-test` | 8 (all `CF`, none carrying a suite) |

The dagger fires on nothing: the file carries 59 daggers and every one sits inside §7's generated
table or the prose describing the convention itself, never inside a `Rule:` line the guard reads.
The term actually doing the `PS` family's work is the backticked `` `new` ``, at 10 hits. Dropping
the dagger produces zero newly-failing `PS` clauses, independently reproducing
`experiments/gate-vacuity/results/raw/spec-trace-drop-dagger.txt`.

## The census retirement exposed

| Kind | Count | What it means |
|---|---|---|
| `Elsewhere(path)` | **26** | A real test exists, in a file no resolution source reads; the path is opened on every run and the identifier must still be there. |
| `NotARuleName` | **4** | `trait_variant` (ES-2), `spec_trace` and `compile_fail` (WF-12), `compile_fail` (VT-32) — harvested out of prose by `backticked_idents`. |
| `Scheduled(who)` | **15** | Nothing has written it yet. |

The 26 `Elsewhere` entries are the number worth having. Twelve `[FROZEN]` clauses — VT-13, VT-18,
VT-20, VT-26, VT-32, VT-33, WF-12, ES-2, ES-3, ES-4, ES-5 and (via VT-10) the foreign-identity
target — stand on tests `collect_rules` has never read, because they live in `happenstance-core`'s
`src/` and `tests/`, or in the testkit's `tests/` and `lib.rs`. Before this change, nothing in the
workspace would have noticed one of those tests being deleted; it does now, but by a path sitting
in a table rather than a resolved reference, which is a weaker instrument.

Of the 15 `Scheduled` entries, nine are the replication and projection families waiting on suites
that do not exist yet. One is not a schedule at all: ES-6 (`store_error_crosses_a_join_handle`)
is argued, in `crates/happenstance-cloudflare/src/send_shape.rs`, to be **unwritable against
today's port** — a finding rather than something merely not yet built. See
`kb-open-question-es-6-unwritable-rule-001`.

## Why `resolvable` was not widened to close the 26

Widening the resolution source to include `happenstance-core/src/*.rs`'s `#[test]` items would
turn the 26 `Elsewhere` declarations into 26 resolved names — a strictly stronger instrument. It
was not taken in the same change because `resolvable` also feeds §7.2's generated table in
`spec/SPECIFICATION.md`, so widening it moves that committed, regenerated region, and the file is
held by another lane. Recorded here as a clean, mechanical follow-up rather than performed.

## Conditions

Measured in this working tree at `9b06836`, by instrumenting `rules_of` and printing per-term
firings and per-clause resolution outcomes over the real `spec/SPECIFICATION.md`.
