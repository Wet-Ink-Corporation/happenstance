---
item: HS-S0012
stage: discover
created: 2026-08-12T13:01:19.417Z
updated: 2026-08-12T13:01:19.417Z
template_sig: 86ce4036
rendered_sig: b6ee9221
---

# Discover — Read-your-writes, chunk-size invariance and rebuild authority

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Slice one-line: `batch_reads_reflect_pending_writes` gated on `ProjectionProbe::READS_THROUGH_BATCH` and emitted as a **reported skip** when an adapter declares `false` (CF-18), `rebuild_is_chunk_size_invariant` replaying a fixed probe sequence at chunk sizes 1, 3 and whole-log, and `rebuilding_is_distinguishable_from_live`, each with the store that fails it — including the batch `get` that answers from committed state | `../_storymap.md:64` | Three rules, three mutants, and the first *real* declined-capability instance in the suite |
| **AC-005** — a rule whose projection capability is declined is still emitted, returns a skip carrying the fixture's stated reason, and is distinguishable from a pass | `../project.md:194-197` | `projection-capability-skips` built the machinery; **this story supplies the real `READS_THROUGH_BATCH = false` instance** (`../_storymap.md:83`) |
| **AC-003** — every rule has a registered wrong implementation with stated provenance | `../project.md:187-190` | Shared with the other rule-bearing stories; exhaustiveness is the forcing function (`../_storymap.md:81`) |
| `dependsOn: projection-mutant-registry, projection-capability-skips` — the registry and exactness meta-tests, plus the skip machinery this slice is the first to exercise with a live declension | `../_storymap.md:60-61,64,111` | Independent of `reset-rules`; either order within the slice |
| `READS_THROUGH_BATCH` is a `const` gate on `ProjectionProbe`, and CF-18 requires a `false` adapter to emit the rule as a **reported skip**, not to omit it | `spec/SPECIFICATION.md:5004,5052-5074`; `../_decomposition.md:479-481` | The const was declared in `projection-probe-conformance-feature`; its `false` arm becomes observable here |
| PS-12: an adapter MUST either make reads issued through an open `Batch` reflect that batch's pending writes, or declare it cannot — `[PROVISIONAL]` | `spec/SPECIFICATION.md:5052-5055` | The rule implements a provisional clause; nothing is frozen by writing it |
| §4.11 names each rule's rejected implementation: `batch_reads_reflect_pending_writes` ← **a batch `get` answering from committed state**; `rebuild_is_chunk_size_invariant` ← a projection reading out of band; `rebuilding_is_distinguishable_from_live` ← rebuild in place with one position field | `spec/SPECIFICATION.md:5674-5676` | Three mutant shapes, pre-specified. The first is quoted verbatim in this slice's own storymap one-line |
| The rule skeleton for chunk invariance: replay a fixed probe sequence at chunk sizes 1, 3 and whole-log, **with the probe's write defined as an increment of what the batch can see**, so a store violating PS-12 diverges | `../_decomposition.md:575-577`; `spec/SPECIFICATION.md:5086-5098` | The increment is what couples PS-12 to PS-14; without it the chunk-size rule passes on a store that cannot read through its own batch |
| PS-13 and PS-14 are both `[FROZEN]` — a projection MUST NOT read its own read model other than through the batch, and a rebuild MUST produce the same read model at every chunk size | `spec/SPECIFICATION.md:5075-5077,5086-5087` | Implemented here, not amended |
| `rebuilding_is_distinguishable_from_live` is PS-24's rule, and `Checkpoint`'s third variant (`Rebuilding`) is what makes the distinction expressible at all | `spec/SPECIFICATION.md:5676`; `../_decomposition.md:588`; `spec/SPECIFICATION.md:4643-4658` | The rule asserts a **variant**, not a position — the same shape as `fresh_projection_has_no_checkpoint` |
| UX brief **AC-U10**: the skip must name the switch the author actually set — for PS-12 that is `ProjectionProbe::READS_THROUGH_BATCH`, "so a reader is sent to the constant they can change" | `../_decomposition.md:181-188`; `crates/happenstance-testkit/src/contract.rs:435-442` | The `capability` field's content is checked by AC-U09's value assertion |
| The precedent for keeping a gated rule non-vacuous: `MemoryFixture` states no CF-40 limit and `GappedPositionFixture` is what keeps the limits rule non-vacuous — "building an instrument to keep a rule non-vacuous is this repository's established move" | `crates/happenstance-testkit/src/fixtures.rs:234-241`; `../_decomposition.md:420-426` | Both arms of `READS_THROUGH_BATCH` need a fixture, or one of them is dead code |
| Architecture brief Note 10 item 1: if a projection rule is found that observes the read model **without** the probe, PS-11 is over-built and the seam should shrink — "an ADR paragraph, not a quiet deletion" | `../_decomposition.md:713-715`; `../_storymap.md:128` | The reshape trigger most likely to fire in this slice, since these are the read-side rules |
| `CLAUDE.md`: never assert literal position values; the specification permits gaps | `CLAUDE.md`, *The rule that matters*; `crates/happenstance-testkit/tests/mutation_coverage/variants.rs:116` | `rebuild_is_chunk_size_invariant` replays a sequence, which is where a contiguity assumption would hide |

## Questions

Open questions to resolve before specifying.

1. **Which fixture declares `READS_THROUGH_BATCH = false`?** Deferred to `spec`,
   with the requirement fixed here: **something must**, or the CF-18 skip arm is
   code no test enters and AC-005's real instance does not exist. The candidate
   is the buffering variant from `buffering-conformant-variant`, which is the
   natural `false` adapter — but that story is downstream, so either this slice
   builds a minimal declining instrument or the spec records the ordering.
2. **Does `MemoryProjectionStore` declare `true`?** Deferred to `spec`. If both
   fixtures in the tree declare the same value, one arm of the rule is untested
   whichever value it is.
3. **What is the fixed probe sequence, and why 1, 3 and whole-log?** Deferred to
   `spec`. The three sizes come from the architecture brief
   (`../_decomposition.md:575-577`); the sequence's content must make a PS-12
   violation *diverge* rather than merely differ, which is the increment
   requirement.
4. **Does `rebuilding_is_distinguishable_from_live` need a rebuild driver?**
   Deferred to `spec`. §4.11 places the six runner-dependent rules in the
   workspace e2e crate under CF-36 (`spec/SPECIFICATION.md:5693-5703`); this rule
   is an adapter-level one, so it must be expressible through `commit`'s
   `Authority` argument alone.
5. **None beyond what the project brief already carries.**

## Decision

Three of the port's guarantees are about *reading*: whether a batch can see its
own pending writes, whether a rebuild produces the same read model however it is
chunked, and whether a store rebuilding a projection is distinguishable from one
serving it live. All three are unobservable today and two of them are frozen
clauses with no rule. This slice writes them, registers the store that fails each,
and — the part that makes AC-005 real rather than machinery — supplies the first
genuine declined capability in the suite, `READS_THROUGH_BATCH = false`, emitted
as a reported skip naming the constant the author can change. The spec will fix:
each rule's text; the fixed probe sequence and the increment that makes a PS-12
violation diverge at different chunk sizes; which fixture declares each arm of
`READS_THROUGH_BATCH`, so neither arm is dead; the three mutants' identifiers and
provenance, taken from §4.11's `Rejects` column; and the assertion shape for
`rebuilding_is_distinguishable_from_live`, which compares `Checkpoint` variants
rather than positions. It edits no `[FROZEN]` clause: PS-13 and PS-14 are frozen
and this slice implements the rules they state.

## The wrong implementation

**A store whose batch `get` answers from committed state.** §4.11 names it
verbatim as what `batch_reads_reflect_pending_writes` rejects
(`spec/SPECIFICATION.md:5674`), and it is the natural first cut: reads go to the
same connection as everything else, and the pending write set is a buffer nobody
consults. It passes every other rule in the suite — commits are atomic, rollbacks
are clean, resets are scoped — and it fails only this one, which is exactly what
makes it a good registry entry. It must exist in
`crates/happenstance-testkit/tests/mutation_coverage/` with
`batch_reads_reflect_pending_writes` as its declared failure.

**The one that makes that mutant unreachable, and is the real hazard here:** an
adapter — ours or anyone's — that declares `READS_THROUGH_BATCH = false` because
the rule is inconvenient. CF-18 is satisfied: the rule is still emitted, the skip
is reported, the reason is printed, nothing vanishes from the binary
(`spec/SPECIFICATION.md:5052-5074`). And if **every** fixture in the tree declares
`false`, the rule body is code no test ever executes, the mutant that fails it is
never run, and the registry's exactness meta-test is asserting over a rule that
cannot fail. That is `CLAUDE.md`'s first corollary — a rule no adapter can fail is
decorative — arriving through the skip path rather than through weak assertions.
The guard is structural and has a precedent: both arms need a fixture, the way
`MemoryFixture` states no CF-40 limit while `GappedPositionFixture` keeps the
limits rule non-vacuous (`crates/happenstance-testkit/src/fixtures.rs:234-241`).
The spec must name which fixture holds which arm.

**`rebuild_is_chunk_size_invariant` with a probe write that is a plain `set`.**
Replay the same sequence at chunk sizes 1, 3 and whole-log, assert the read models
match — and they match on a store that cannot read through its own batch, because
a last-writer-wins `set` is chunk-insensitive by construction. The rule passes,
PS-13 and PS-14 are both "covered", and the coupling the architecture brief
specifies is gone: the probe's write must be **an increment of what the batch can
see**, so a store violating PS-12 produces different totals at different chunk
sizes (`../_decomposition.md:575-577`). This is the rule that most looks correct
while checking nothing.

**And the position-shaped trap:** writing chunk invariance as an assertion over
the checkpoint values reached at each chunk size, or over a contiguous replay
sequence 1..n. The specification permits gaps and a conformant adapter may leave
them (`CLAUDE.md`, *The rule that matters*;
`crates/happenstance-testkit/tests/mutation_coverage/variants.rs:116`). The
invariant is over the **read model**, compared between runs — never over the
positions used to get there.

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

The two judgement boxes. **Literal positions**: `rebuild_is_chunk_size_invariant`
replays a sequence three times, which is where a contiguity assumption would
hide. The commitment carried into `spec` is that the invariant is asserted over
the resulting **read model**, compared run against run, and never over the
positions used to reach it or over a checkpoint compared to a literal;
`rebuilding_is_distinguishable_from_live` likewise asserts a `Checkpoint`
variant rather than a position. **`[FROZEN]` clauses**: PS-13 and PS-14 are
frozen (`spec/SPECIFICATION.md:5077,5087`) and this slice implements the rules
they state without editing either; PS-12 and PS-24's rule surface is
`[PROVISIONAL]`. Any pairing defect in PS-13/PS-14 found by
`ps-clause-pairing-sweep` was repaired by the new atom accepted in
`projection-decision-atoms`, in slice 1.
