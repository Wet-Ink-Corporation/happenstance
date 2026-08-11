---
id: kb-decision-0011
title: A read is one sample with a ceiling, and &Query stays
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0011
reversibility: low
phase: 4
supersedes: null
superseded_by: null
summary: >-
  Seven dispositions on the read side, two clauses left provisional and two clauses minted. read
  keeps query: &Query — the opaque return type captures the query's lifetime whether or not one is
  named, so an escaping stream owns its query as a parameter rather than as a local, and ES-13's
  own stated remedy is wrong on that point. The port's promise is corrected from lazy to evaluated
  against one state sampled no later than the first poll: laziness is permitted and never
  required, building a stream is not guaranteed free, and a caller must not depend on whether
  events appended between the call and the first poll appear. An adapter issuing more than one
  statement per read must capture a position ceiling no later than the first poll and bound every
  later statement by it, which discharges both the one-snapshot and the shared-snapshot clauses
  across all three store shapes. read gains no + Unpin bound, minted provisional with a hard
  phase-12 deadline because the bound is unaddable after publish. ReadOptions gains an inclusive to
  and a limit of Option<usize> so limit(0) yields nothing, and is frozen with exactly one lower
  bound: no exclusive after, ever. No extension trait and no prelude at 0.1.
depends_on:
  - kb-decision-0008
  - kb-decision-0010
related:
  - kb-decision-0001
  - kb-reference-port-traits-compiled-findings-001
source_paths:
  - .kb/_intake/0011-read-laziness-and-isolation.md
  - docs/adr/0011-read-laziness-and-isolation.md
  - crates/happenstance-core/src/store.rs
  - crates/happenstance-core/src/query.rs
  - references/evaluation/phase-4-reconciliation.md
  - spec/SPECIFICATION.md
last_reviewed: 2026-08-10
---

# A read is one sample with a ceiling, and `&Query` stays

## Context

`read` is the only port method whose shape is load-bearing for the wasm target: it returns a bare
`impl Stream`, not an `async fn`, so `#[trait_variant::make]` can mark the *stream* `Send` on the
derived flavour. This decision leaves that shape untouched and settles what it promises: when is
the store's state sampled, and do the items of one `Query` share one sample? Isolation matters
even though DCB's append condition re-checks at write time, because the caller derives the
condition's boundary *from* the read — a torn read returns a maximum position above an event it
silently missed, and the resulting condition says "reject anything after P" while the missed event
sits below P. A torn read does not turn into a rejected append; it turns into an accepted one.

Compiled experiments settled three open disputes. `&Query` compiles for every escaping case the
project's own RUNBOOK claimed was impossible, provided the query is owned as a *parameter* rather
than a named local — the local/parameter distinction that ES-13's stated remedy had gotten
backwards, since a local dies at the end of its own function and cannot outlive an escaping
stream. The port can be erased behind `dyn` today, by hand, with no `unsafe`, contradicting a
prior ADR's claim that `dynosaur` was required. And an `Unpin` bound was compiled to forbid a
generator-backed stream — the natural implementation of a chunked-cursor adapter, the one shape
with no implementation anywhere in the workspace.

## Decision

`read` keeps `query: &Query`; the clause is confirmed, not superseded. The port's promise changes
from "lazy" to "evaluated against one state sampled no later than the first poll": laziness is
permitted, never required; building an unpolled stream is not guaranteed free; and a caller must
not depend on whether events appended between the call and the first poll appear. A single
mechanism discharges both the one-snapshot and shared-snapshot isolation questions across all
three store shapes in the workspace — snapshot-under-a-lock, chunked cursor, and one buffered
response body: an adapter issuing more than one statement per `read` must capture a position
ceiling no later than the first poll and bound every later statement by it. The two isolation
clauses stay provisional, because the transport axis that would falsify them is empty at both ends
in-tree; their falsifiers are named against the phase-9 and phase-10 adapters that will exist.

`read`'s return type takes no `+ Unpin` bound — decided, not deferred, because adding a bound to
an opaque return type later is breaking. It is minted provisional with a hard phase-12 deadline,
since after publish the bound becomes permanently unaddable. `ReadOptions` gains an inclusive `to`
as the caller-side spelling of the ceiling, and `limit` becomes `Option<usize>` so `limit(0)`
yields nothing rather than being silently discarded as unlimited before any adapter sees it.
`ReadOptions::after`, an exclusive lower bound, is declined and frozen with no falsifier: two lower
bounds with opposite inclusivity on one `#[non_exhaustive]` struct is a precedence hole with no
adapter-consistent resolution, and the idiom it would replace — `checkpoint.next()` — is already
frozen and sound. No `EventStoreExt` and no prelude ship at 0.1, since an extension trait is the
one surface that never needs a freeze phase and can be added later non-breakingly.

## Consequences and alternatives rejected

Good: no adapter signature changes, and the derivation scheme this decision extends is untouched.
Bad, and the honest cost: the phase-4 proof artefact loses its stated purpose, since all four
proof cases now compile against the pre-freeze signatures and the artefact becomes a regression
pin rather than a demonstration of change. Rejected: precise capturing via `use<…>`, which requires
superseding a different frozen clause governing the derivation macro; `read(query: Query)` by
value, which supersedes the confirmed clause to buy an escape already available by parameter, and
relocates rather than removes the clone cost; and mandatory laziness or mandatory call-time
sampling, both unobservable through the port and both would exclude a real in-tree adapter shape.
