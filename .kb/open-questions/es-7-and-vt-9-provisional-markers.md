---
id: kb-open-question-provisional-falsifiers-001
title: Two provisional markers whose falsifiers can no longer falsify
kind: open_question
status: accepted
authority_tier: note
summary: >-
  ES-7 and VT-9 are PROVISIONAL and each names a falsifier that no longer discriminates. ES-7's
  falsifier is error[E0119] on a downstream direct impl, and its own named instrument —
  LocalMemoryEventStore in happenstance-testkit — compiles natively and on wasm32 with no such
  error. VT-9's falsifier is a target that cannot supply a wall clock at append time, and one is
  already in the build: on wasm32-unknown-unknown there is no clock and every event is stamped
  from_millis(0), yet VT-9's MUST is satisfied there because its rules assert presence and
  stability of a recorded time and never recency. Moving a maturity marker is an ADR's act, so
  both are recorded rather than moved. The transferable observation, worth keeping however these
  are settled: a falsifier that has already occurred without changing anything is a marker that
  has quietly become decoration, and spec-trace cannot detect it — it sees that a marker exists,
  not whether its condition has been met.
depends_on: []
related:
  - kb-reference-phase-4-5-spec-reconciliation-001
  - kb-playbook-repair-frozen-clause-001
  - kb-open-question-post-phase-reconciliation-001
source_paths:
  - .kb/_intake/gaps-owed-a-decision.md
  - spec/SPECIFICATION.md
  - crates/happenstance-testkit/tests/local_conformance.rs
  - crates/happenstance-core/src/memory.rs
last_reviewed: 2026-08-10
---

# Two provisional markers whose falsifiers can no longer falsify

## What is true today

A `[PROVISIONAL]` marker in `spec/SPECIFICATION.md` carries a stated falsifier — the observation
that would demote or change the clause. Two markers name falsifiers that, on inspection of the
code the pass grounded itself in, no longer discriminate: the condition each names has already
occurred, and the clause is unharmed by it.

**ES-7** (`spec/SPECIFICATION.md:2685`) says a downstream crate may implement the bare
`EventStore` flavour directly without colliding with the blanket impl `trait_variant` emits. Its
marker reads: `[PROVISIONAL — falsified by error[E0119]: conflicting implementations on a
downstream impl EventStore for LocalType. The named test is the !Send reference store, which
lives in happenstance-testkit — a genuinely downstream crate — and which is also ADR-0001's own
lift condition.]` The named instrument is `LocalMemoryEventStore`
(`crates/happenstance-testkit/tests/local_conformance.rs:198`), whose own comment states: "This
is the ES-7 evidence: a direct `impl EventStore for` a local type in a downstream crate,
alongside the blanket `impl<T: SendEventStore> EventStore for T`… No `error[E0119]`." The
instrument compiles natively and on `wasm32` with no collision. A falsifier its own named
instrument cannot produce is not doing work.

**VT-9** (`spec/SPECIFICATION.md:903`) carries the marker: `[PROVISIONAL — falsified by a target
that cannot supply a wall clock at append time; a Cloudflare Durable Object returning a frozen
clock between I/O operations is the candidate, and the Workers skeleton is the instrument]`.
`crates/happenstance-core/src/memory.rs:263-274` shows such a target already in the build: on
`wasm32-unknown-unknown` there is no clock, `wall_clock_millis()` returns `None`, and every event
is stamped `RecordedAt::from_millis(0)`. Its own doc comment names the clause directly: "VT-9's
own falsifier names 'a target that cannot supply a wall clock at append time', and this is one."
Yet VT-9's MUST is satisfied there — its two rules assert that a recorded time is *present* and
*stable*, never that it is *recent*, and CF-33 independently forbids checking a value for
plausibility against the harness's clock. The falsifier condition is satisfiable without
falsifying the clause.

## What is not decided

Whether ES-7 and VT-9 lift to `[FROZEN]`, or whether each falsifier is restated so it would
actually discriminate. Moving a maturity marker changes what a downstream consumer is entitled to
rely on, which is why it is an ADR's act rather than an edit — the same repair-versus-amendment
test that governs `[FROZEN]` clauses applies here to the marker itself.

For VT-9, the honest restatement is probably not "does a clockless target exist" — one already
does, harmlessly — but something about what a clockless target is *obliged* to do (e.g., a bound
on drift or ordering once a real network clock is reintroduced). For ES-7, the restatement would
need to name a collision surface the blanket impl does not already cover, since the direct-impl
surface it names is now demonstrated clean on the target that motivated the marker.

## What forces it

Neither is on a named phase or ADR queue entry the way the PS-layer gaps are (see
`kb-open-question-ps-1-no-progress-obligation-001` and
`kb-open-question-ps-19-scope-narrower-001`), which is itself part of what is open: each is
entangled with a schedule that belongs to something else. ES-7 is entangled with ADR-0001's own
lift condition — the marker and the ADR's condition name the same instrument, so settling one is
close to settling the other. VT-9 is entangled with the Cloudflare Workers adapter's schedule:
today's clockless build is `happenstance-core`'s own `wasm32` target, not yet the Durable Object
adapter the marker was written against, so the marker may still have discriminating work left to
do once that adapter exists — which argues for restating rather than lifting.

## The transferable observation

Worth keeping independent of how these two are settled: **a falsifier that has already occurred
without changing anything is a marker that has quietly become decoration.** `cargo xtask
spec-trace` cannot detect this class of drift — it can see that a marker exists and cite it, but
it has no way to evaluate whether the condition the marker names has been met by the code it is
checking against. That is a reading task, not a mechanisable one, which is the same shape of gap
`kb-open-question-post-phase-reconciliation-001` describes for the reconciliation exit criterion
more generally: some obligations in this specification's maintenance can only be discharged by a
human rereading a clause against the tree as it now stands.

## Owner

Unassigned.
