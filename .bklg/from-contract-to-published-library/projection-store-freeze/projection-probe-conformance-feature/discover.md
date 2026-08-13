---
item: HS-S0005
stage: discover
created: 2026-08-12T13:01:13.187Z
updated: 2026-08-12T13:01:13.187Z
template_sig: 86ce4036
rendered_sig: 7fca2299
---

# Discover — ProjectionProbe behind happenstance-core's conformance feature

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Slice one-line: add `ProjectionProbe` (`READS_THROUGH_BATCH`, `probe_write`, `probe_delete_all`, `probe_read`, `probe_read_through`) to `happenstance-core` behind a new `conformance` feature — mounted in the export block with `#[cfg_attr(docsrs, doc(cfg(…)))]` and in `Cargo.toml`'s feature table — with a round-trip test through nothing but the trait, and the host and wasm32 feature-powersets green over the widened combination set | `../_storymap.md:57` | Three deliverables: the trait, both mount points, and the powerset that proves the new feature is not silently coupled to `memory` |
| **AC-009** — generic suite code can write into a batch and observe the resulting read model out of band (`write_probe` / `read_probe`) without reaching into an adapter's internals | `../project.md:209-211` | This story owns the **seam half**; `owned-batch-port-shape` owned the signature half (`../_storymap.md:87`) |
| `dependsOn: owned-batch-port-shape` — supplies `type Batch;` with no lifetime, which is what lets a probe method take `&mut Self::Batch` without a GAT anywhere | `../_storymap.md:56-57,108` | The probe is unwritable against the borrowing GAT; that is the ordering's whole content |
| Today there is **no `apply` seam**: generic code holding a `P::Batch` can pass it to `commit` or `rollback` and nothing else | `crates/happenstance-core/src/projection.rs:92-99,126-138`; `.kb/open-questions/projection-store-batch-has-no-apply-seam.md`; `../_grounding.md:27-37` | Without the probe, three of the projection rules cannot observe the read model at all and the suite degenerates to a checkpoint test a broken store passes |
| Architecture brief **AC-A02**: the write seam is `ProjectionProbe` in **`happenstance-core`** behind a new `conformance` feature, exactly as the specification gives it, *including the coherence argument*. "Any deviation is an ADR-0017 clause, not an implementation shortcut" | `../_decomposition.md:299-302`; `spec/SPECIFICATION.md:4977-5031` | The placement is settled upstream and is not this story's to re-open. What this story owes is executing it and keeping the argument intact |
| The coherence argument, already written: an adapter's `tests/` directory is a **different crate**, where neither a testkit trait nor the adapter's type is local, so the impl is rejected by the orphan rule and the adapter is forced into a non-dev dependency on `happenstance-testkit` | `spec/SPECIFICATION.md:5015-5031`; `../_decomposition.md:464-472` | "This is the part most likely to be rediscovered expensively; it is already discovered." It is also the reason the wrong implementation below is invisible inside this workspace |
| `probe_delete_all` exists so `reset` (PS-16) is checkable without the suite knowing what a read model is — "not optional garnish" | `spec/SPECIFICATION.md:5027`; `../_decomposition.md:476-478` | A four-method probe that drops `probe_delete_all` makes `reset-rules` unwritable two slices later |
| `READS_THROUGH_BATCH` is a `const` gate, and CF-18 requires a `false` adapter to emit the rule as a **reported skip**, not to omit it | `spec/SPECIFICATION.md:5004,5052-5074`; `../_decomposition.md:479-481` | The const is declared here; the skip behaviour it drives is `read-through-and-rebuild-rules`' rule and `projection-capability-skips`' machinery |
| Adding `conformance` widens two gate steps — the workspace feature powerset and the **wasm32** powerset, which names `happenstance-core` explicitly. Core has three features today; `conformance` plus a possible `unstable-projection` takes eight combinations to thirty-two, and **each must compile, including `conformance` without `memory`** | `../_decomposition.md:483-487`; `xtask/src/main.rs` (`hack` and wasm32 powerset steps) | The powerset is the only automated check that the new feature is independently coherent, and it is the one the wrong implementation below trips |
| Rustdoc hazard already paid for once: an intra-doc link into a module absent on some documented configuration is a hard rustdoc error, and the testkit spells two module names plainly for exactly that reason | `crates/happenstance-testkit/src/lib.rs:112-132`; `../_decomposition.md:694-699` | `ProjectionProbe` behind `conformance` is a new instance of the same hazard; the gate's `--no-default-features` doc build of `happenstance-core` is where it fires |
| ADR-0001 and ADR-0008 both bite: the probe is bare-flavour only, and a provided body cannot hold the `Batch` GAT across a suspension point — one of ADR-0017's inputs | `../_decomposition.md:374-390`; `.kb/decisions/0008-one-derivation-for-both-ports.md` | The probe's method shapes are constrained by the derivation scheme, not chosen freely |
| Testing brief AC-009: **Static** (`type Batch;` with no lifetime is enforced at every call site by `cargo check`) plus **Unit** — a test calling `probe_write` then `probe_read` through nothing but the trait, "proving the seam is generic rather than merely present" | `../_decomposition.md:779` | The round-trip test is the story's own falsifier and must be written against the trait, not against `MemoryProjectionStore`'s inherent methods |
| Architecture brief Note 10 item 1: if a projection rule is found that observes the read model **without** the probe, PS-11 is over-built and the seam should shrink — "a finding worth an ADR paragraph, not a quiet deletion" | `../_decomposition.md:713-715` | A named reshape trigger for this story specifically |

## Questions

Open questions to resolve before specifying.

1. **Does the `conformance` feature imply `memory`?** Answered: no, and the
   powerset is what enforces it — `conformance` without `memory` is one of the
   thirty-two combinations that must compile (`../_decomposition.md:483-487`).
   The probe is a trait over an adapter's own store; it has no dependency on the
   reference implementation.
2. **Does the probe need a `Send` flavour?** Answered by the specification: bare
   flavour only (`spec/SPECIFICATION.md:4998-5031`). Suite code binds the weaker
   requirement, which is the same rule `CLAUDE.md` states for `EventStore`.
3. **What are the probe's key and value types?** Deferred to `spec`. The
   specification gives the trait verbatim and the spec stage should take it
   rather than re-derive it; where the two disagree the specification wins
   (`../_decomposition.md:277-279`).
4. **Is `ProjectionProbe` a supertrait of `ProjectionStore`, or a separate impl?**
   Deferred to `spec`, with one constraint recorded: PS-11 says an adapter MUST
   implement the probe (`spec/SPECIFICATION.md:4977`), and whichever spelling is
   chosen must not make `happenstance-core` compile differently with the feature
   off — that is what the powerset checks.
5. **None beyond these.** The placement question (core versus testkit) is *not*
   open — it is Architecture brief AC-A02 and the specification's own coherence
   argument.

## Decision

Three of the projection suite's rules have to write a row and read it back, and
today generic code holding an adapter's batch can do neither: it can `commit` the
batch or `rollback` it, and nothing else. Without a write seam the whole suite
collapses into a checkpoint test that a store which silently discards every
read-model write passes — which is to say, into exactly the decorative suite this
project exists to avoid. This slice lands `ProjectionProbe` in `happenstance-core`
behind a new `conformance` feature, mounted in both places a library item is
either reachable or invisible, and proves it round-trips through the trait alone.
The spec will fix: the trait's five members, deferring to
`spec/SPECIFICATION.md:4998-5031` verbatim; the feature's declaration and its
independence from `memory`; the export block's `#[cfg(feature = "conformance")]`
plus `#[cfg_attr(docsrs, doc(cfg(…)))]` pattern, copied from the existing
`memory` gating; the defensive module spelling that keeps a `--no-default-features`
doc build green; and the round-trip test, written through `ProjectionProbe`'s
methods only. It touches no `[FROZEN]` clause: PS-11 and PS-12 are both
`[PROVISIONAL]` (`spec/SPECIFICATION.md:4980,5055`) and this story implements
them rather than amending them.

## The wrong implementation

**`ProjectionProbe` defined in `happenstance-testkit` instead of
`happenstance-core`.** This is the one that matters, because *nothing in this
workspace can fail it*. Every fixture here lives in a crate that already depends
on the testkit; the trait is local, the impls are local, the orphan rule never
fires, `cargo xtask ci` is green in every step including both powersets, and the
seam works perfectly for as long as the only adapter authors are us. It breaks
for the first author outside this repository: their adapter's `tests/` directory
is a different crate, where neither a testkit trait nor their own type is local,
so `impl ProjectionProbe for MyStore` is rejected by the orphan rule and the only
way out is a **non-dev dependency** on `happenstance-testkit`
(`spec/SPECIFICATION.md:5015-5031`). The workspace-internal check that convicts it
does not exist and cannot: the falsifier is AC-007's outside-author fixture, built
from documentation alone in `documented-extension-surface` (HS-S0015), which is
this story's real mutant and sits five slices later. The spec should say so, so
that the placement is not re-litigated on grounds of diff size in the meantime.

**A `conformance` feature that only compiles alongside `memory`.** Trivially
arrived at — the round-trip test needs a store, `MemoryProjectionStore` is the
store, and an `#[cfg(feature = "conformance")]` module that reaches into the
`memory` module compiles fine on default features and on `--all-features`. It is
caught, but only by a step people skip locally: the feature powerset over
`happenstance-core`, host and wasm32
(`../_decomposition.md:483-487`). Making it a *stated* deliverable of this story
rather than a consequence of the gate is the difference between finding it here
and finding it in `whole-gate-run-and-proof-artefact`.

**A four-method probe.** Dropping `probe_delete_all` because nothing calls it yet
is locally reasonable — the suite at the end of this slice has no `reset` rules —
and it makes `reset_clears_rows_and_checkpoint_together` unwritable two slices
later without the suite knowing what a read model is
(`spec/SPECIFICATION.md:5027`). Nothing fails now; the cost lands on `reset-rules`.

**And the doc-only one:** an intra-doc link from the port's rustdoc into the
`conformance`-gated module. `cargo doc` is green with default features and on
`--all-features`, and the gate's `--no-default-features` doc build of
`happenstance-core` turns a broken link into a hard error — a hazard this
workspace has already paid for once and documented at
`crates/happenstance-testkit/src/lib.rs:112-132`.

## Gate: Discover

Tick each box once the sections above satisfy it. `redkiln advance` reads these
against `.redkiln/templates/gates/discover.md` — where each box's rationale is
written — and will not leave `discover` until every one is ticked. Keep each box on
one line; the parser matches line by line.

- [x] The problem is framed in one paragraph.
- [x] Prior art and constraints are recorded in the signal ledger.
- [x] Open questions are either answered or explicitly deferred.
- [x] The next stage (spec) has a clear starting point.
- [x] The wrong implementation this work rejects is named.
- [x] No conformance rule added here asserts a literal position value.
- [x] Any `[FROZEN]` clause this touches is changed by a new ADR, written first.
- [x] If a rule here seems wrong, it is fixed and the reason given in the same change.

The two judgement boxes. **Literal positions**: this story adds a trait and one
round-trip test, not a conformance rule; the round-trip asserts a probe value,
never a position. Vacuously true. **`[FROZEN]` clauses**: none. PS-11 and PS-12,
the two clauses this seam serves, are both `[PROVISIONAL]`
(`spec/SPECIFICATION.md:4980,5055`), and this story implements them rather than
amending their text.
