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
  not whether its condition has been met. Both markers now have named owners in the imported ADR
  corpus: ES-7's evidence is ADR-0001's lift condition, discharged by ADR-0008 and compiled in the
  port-traits findings, and VT-9 is one of ADR-0014's four provisional parts, owned by phase 9's
  Workers skeleton. Two further markers join them in 2026-09, and they sharpen the observation
  rather than repeat it, because these falsifiers never could fire rather than having already
  fired harmlessly. PS-4 is PROVISIONAL and its falsifier has two limbs; the Rust-level limb — an
  adapter whose write handle must exist before a traversal the projection's own logic depends on
  — is foreclosed by the port for every batch shape, because Projection::apply is synchronous and
  a traversal is I/O, which is a fact about the port and equally true of the SQLite and Postgres
  adapters. PS-2 is the same defect one level over and with a wider blast radius: that clause is
  FROZEN, so what cannot be met is its bar rather than a falsifier, and the live-transaction end
  of its batch-shape axis is forbidden by begin's total, synchronous, infallible signature for
  both drivers the Rule names, while thirteen PROVISIONAL clauses wait on PS-2 alone. ADR-0060
  keeps the port's gate on that ground without rewording PS-2's MUST or moving a marker. The
  transferable observation therefore gets one clause sharper: a falsifier can be decoration a
  priori and not only in retrospect, and spec-trace detects neither shape, because it checks that
  a clause's citations resolve rather than whether its stated condition is reachable.
depends_on: []
related:
  - kb-reference-phase-4-5-spec-reconciliation-001
  - kb-playbook-repair-frozen-clause-001
  - kb-open-question-post-phase-reconciliation-001
  - kb-decision-0001
  - kb-decision-0008
  - kb-decision-0014
  - kb-reference-port-traits-compiled-findings-001
  - kb-open-question-adr-status-vocabulary-001
  - kb-reference-spec-trace-has-suite-001
  - kb-open-question-no-ps-rule-name-resolved-001
  - kb-open-question-cf-17-cf-14-markers-001
  - kb-decision-0025
  - kb-decision-0060
  - kb-open-question-probe-read-through-signature-001
  - kb-open-question-one-shot-http-es-11-001
source_paths:
  - .kb/_intake/gaps-owed-a-decision.md
  - .kb/_intake/0001-async-port-flavours.md
  - .kb/_intake/0008-one-derivation-for-both-ports.md
  - .kb/_intake/0014-event-identity-and-recorded-time.md
  - .kb/_intake/2026-09-08-adr-0025-ladybug-projection-adapter.md
  - .kb/_intake/2026-09-08-ps-2-live-transaction-axis-is-forbidden-not-unbuilt.md
  - .kb/_intake/2026-09-08-adr-0060-ps-2s-axis-re-evaluated.md
  - spec/SPECIFICATION.md
  - crates/happenstance-testkit/tests/local_conformance.rs
  - crates/happenstance-core/src/memory.rs
  - references/adr/0001-async-port-flavours.md
  - references/adr/0014-event-identity-and-recorded-time.md
last_reviewed: 2026-09-09
---

# Two provisional markers whose falsifiers can no longer falsify

## What is true today

A `[PROVISIONAL]` marker in `spec/SPECIFICATION.md` carries a stated falsifier — the observation
that would demote or change the clause. Two markers name falsifiers that, on inspection of the
code the pass grounded itself in, no longer discriminate: the condition each names has already
occurred, and the clause is unharmed by it.

This atom is about `[PROVISIONAL]` markers on *specification clauses*;
`kb-open-question-adr-status-vocabulary-001` is about the frontmatter *status* of an imported ADR
that is accepted-but-provisional. Same word, two different objects — a clause marker is normative
metadata inside `SPECIFICATION.md` that lifts by a marker edit at a named phase, an ADR status is
a KB frontmatter value with only five legal spellings.

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

**The imported ADR corpus supplies the owners the paragraph above could not find, and both land on
phase 9.** For ES-7, `kb-decision-0001` was itself authored provisional on 2026-08-05 with no
`!Send` implementer in existence, and `kb-decision-0008` lifted that marker on 2026-08-06 on
exactly the evidence this atom cites: `LocalMemoryEventStore` — an `Rc<RefCell<Vec<_>>>` store —
implementing `EventStore` directly in a genuinely downstream crate alongside the blanket impl,
with no `error[E0119]`, passing all twenty-seven rules under three harnesses. Two refinements
recorded there matter to anyone restating the marker, and are compiled in
`kb-reference-port-traits-compiled-findings-001`: `RefCell` alone is `Send` — it surrenders `Sync`,
not `Send` — so the `Rc` is what does the work, and a store that were literally `RefCell<Vec<_>>`
would prove nothing. ADR-0008 also states what its lift does *not* buy: the full proof remains the
Cloudflare adapter at phase 9, because what is proved is that the design admits a `!Send`
implementer and that the suite can drive one, not that a real platform SDK fits. So the ADR's own
provisional marker is discharged while the clause's marker is not — two objects again, and the
same distinction the opening section draws.

For VT-9, `kb-decision-0014` names it as one of four provisional parts and restates the falsifier
unchanged: a target that cannot supply a wall clock at append time, with the Durable Object as
candidate, the Workers skeleton as instrument, and phase 9 as the first observation. Its stated
reason for phase 9 is the one this atom contradicts — *"nothing before it can pose the question,
because every store in the tree runs on a host with `SystemTime`."* The `wasm32` build of
`happenstance-core` is a store in the tree that does not, and it poses the question today,
harmlessly. That is not a defect in ADR-0014, whose restatement is aimed at a Durable Object's
*frozen* clock rather than an absent one, but it is the sharpest available argument that the
falsifier as written under-specifies its own condition. ADR-0014 also forbids, in the contract
itself, the rule that would have made VT-9 discriminate for free: no rule may compare two recorded
times or compare one against a position. That is why presence and stability are all VT-9's two
rules can assert, and it is a deliberate consequence rather than an oversight.

## Two more, and these never *could* fire

Phases 10b and 11 added two instances, and they are not repetitions of the two above. ES-7's and
VT-9's conditions had already occurred harmlessly; these two name conditions that nothing the port
admits can meet at all.

**PS-4** (`spec/SPECIFICATION.md:5148-5157`) is `[PROVISIONAL]` and its falsifier has two limbs.
The Cypher-level limb — graph mutations that cannot be expressed as a replayable statement list —
did not fire against LadybugDB, which is an ordinary negative result: a transaction gives
read-your-own-writes within itself, so a deferred write set answers it. The Rust-level limb — a
write handle that must exist before a traversal the projection's own logic depends on — cannot
fire against any adapter, because `Projection::apply` is synchronous and a traversal is I/O, so no
batch shape can carry the dependency the limb describes. `kb-decision-0025` reports this, having
been the phase that owned the clause, and states its scope precisely: it is a finding about the
*port*, equally true of the SQLite and Postgres adapters, rather than something LadybugDB happened
to demonstrate. Half of PS-4's falsifier is therefore unreachable by construction, and the half
that remains is the one already discharged.

**PS-2** is the same defect one level over, and it is the one with consequences. That clause is
`[FROZEN]`, so what has become unmeetable is its *bar* rather than a falsifier — the shape is
identical, the vocabulary is not. The bar wants two adapters at opposite ends of the batch-shape
axis. The live-transaction end is refuted for both drivers the Rule names, by two different
mechanisms: `sqlx`'s `Transaction` cannot be produced by a `begin` that is total, synchronous and
infallible, and `rusqlite`'s `Transaction<'_>` is `!Send` and so costs the `SendProjectionStore`
impl. `kb-decision-0060` records that the drivers are not the deeper problem — a store whose batch
genuinely is a live transaction *can* implement the port, and must then declare
`READS_THROUGH_BATCH = false`, so it reports the same capability profile as a buffering one and
the suite cannot tell the two ends apart; that seam is
`kb-open-question-probe-read-through-signature-001`. ADR-0060 keeps the port's gate on exactly
that ground, without rewording PS-2's MUST or moving PS-3's marker, and proposes the probe
signature change without making it. Thirteen `[PROVISIONAL]` clauses are gated on PS-2 alone, and
a provisional group waiting on an adapter nobody can build waits forever — which is why this
instance is the one that most needed writing down rather than leaving as a schedule slipping.

## The transferable observation

Worth keeping independent of how these two are settled: **a falsifier that has already occurred
without changing anything is a marker that has quietly become decoration.** `cargo xtask
spec-trace` cannot detect this class of drift — it can see that a marker exists and cite it, but
it has no way to evaluate whether the condition the marker names has been met by the code it is
checking against. That is a reading task, not a mechanisable one, which is the same shape of gap
`kb-open-question-post-phase-reconciliation-001` describes for the reconciliation exit criterion
more generally: some obligations in this specification's maintenance can only be discharged by a
human rereading a clause against the tree as it now stands.

**And 2026-09 adds one clause to it: a falsifier can be decoration *a priori*, not only in
retrospect.** ES-7 and VT-9 are markers whose condition was met without effect; PS-4's Rust-level
limb and PS-2's live-transaction end are conditions the port's own signatures forbid being met.
The second kind is worse in one specific way — waiting is a rational response to the first and
never terminates for the second — and `spec-trace` misses both for the same reason: it checks that
a clause's citations resolve, never whether the condition the clause states is reachable. Deciding
reachability means reading a falsifier against a signature, which is the same human reading task,
now owed at authoring time as well as at reconciliation time.

## Owner

Unassigned as a *marker-editing* task. The evidence is owned: ES-7's by ADR-0001 and ADR-0008,
VT-9's by ADR-0014. What no ADR owns is the act of restating either falsifier so that it would
discriminate again, and phase 9 is where both first have the instrument to justify it.

The 2026-09 pair are owned the same way and no further: PS-4's evidence by `kb-decision-0025` and
PS-2's by `kb-decision-0060`, both of which deliberately stop short of touching a marker or a
Rule. Restating PS-4's Rust-level limb so it could fire, and deciding between rewording PS-2's
axis, moving the port's `begin`/probe seam, and recording the bar as unmeetable in its stated
terms, is PS-2's owner's call and is unassigned.
