---
item: HS-S0010
stage: discover
created: 2026-08-12T13:01:17.526Z
updated: 2026-08-12T13:01:17.526Z
template_sig: 86ce4036
rendered_sig: 0c09fbc5
---

# Discover — Commit, rollback and dropped-batch rules, each with the store that fails it

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Slice one-line: the commit-side rules and the store that fails each — `failed_commit_leaves_both_unchanged`, `rollback_leaves_both_unchanged`, `dropped_batch_leaves_store_usable` (drop bare, then open and commit a second batch), `commit_rejects_a_foreign_batch` via the per-instance stamp, `commit_accepts_a_position_the_batch_did_not_write`, `commit_rejects_a_regressing_position` and `distinct_projections_advance_independently`, each registered with its mutant and **never asserting a literal position** | `../_storymap.md:62` | Seven rules, seven mutants, and the literal-position warning is in the slice's own one-line |
| **AC-010** — a batch dropped without `commit` or `rollback` rolls back **and** leaves the store usable, proved by a rule that rejects a store which loses its connection and answers `Busy` forever | `../project.md:212-214`; `RUNBOOK.md:3893-3897` | Sole owner (`../_storymap.md:88`). The "and" is the whole clause: "the rule is not 'rolls back', it is 'and the store remains usable'" |
| **AC-003** — every rule has a registered wrong implementation that fails it, with stated provenance | `../project.md:187-190` | Shared with the registry story and the two reset/rebuild stories; the exhaustiveness meta-test is what forces each rule story to carry its own mutants (`../_storymap.md:81`) |
| `dependsOn: projection-mutant-registry` — supplies `REGISTRY: &[Declared]`, the three exactness meta-tests and the `CheckpointOnlyStore` precedent every new mutant is registered beside | `../_storymap.md:61-62,110` | A rule added here without a registered mutant fails `every_rule_has_a_mutant` immediately. That is the intended pressure |
| §4.11 already names, per rule, the implementation it rejects: `failed_commit_leaves_both_unchanged` ← a partial apply that reports failure; `rollback_leaves_both_unchanged` ← a rollback that only discards the buffer; `dropped_batch_leaves_store_usable` ← a pooled connection `Drop` never returns; `distinct_projections_advance_independently` ← a single-row checkpoint table; `commit_accepts_a_position_the_batch_did_not_write` ← an adapter that validates the position; `commit_rejects_a_regressing_position` ← unconditional `UPDATE checkpoint SET position = ?`; `commit_rejects_a_foreign_batch` ← every adapter writable today | `spec/SPECIFICATION.md:5663-5669` | Seven mutants are already specified as *shapes*. The spec stage names and registers them; it does not invent the defects |
| `ValidatingCommitStore` is one of §4.11's three named hostile stores — "rejects a position the batch did not write" | `spec/SPECIFICATION.md:5680-5683`; `../_decomposition.md:684-688` | It belongs to this slice, and it is the mutant for `commit_accepts_a_position_the_batch_did_not_write` |
| AC-010's provenance is a real reviewer's probe, not a hypothetical: "a dropped batch permanently losing the connection, after which the store returned `Busy` forever" | `../_grounding.md:192-196`; `RUNBOOK.md:3893-3897` | Cite it rather than restate it more weakly. It is also the provenance string for this rule's mutant |
| The rule skeleton for the drop/rollback family: substitute `rollback(batch)` or a bare `drop(batch)` for the commit, **then open and commit a second batch** | `../_decomposition.md:569-575`; `spec/SPECIFICATION.md:4898-4910` | The second batch is the entire difference between a rule that catches the `Busy`-forever store and one that does not |
| PS-15 stays `[PROVISIONAL]` and is discharged at run time: `begin` stamps an identity minted per store instance and `commit` compares. "With an owned batch that stamp is a field and the check is an integer comparison" | `spec/SPECIFICATION.md:5126-5155`; `../_decomposition.md:629-637` | Dropping the lifetime did **not** make the foreign-batch hazard unrepresentable; only a generative brand would, and it was compiled and refuted |
| `commit_rejects_a_foreign_batch` needs two *isolated* stores — two `open()` calls — and explicitly does **not** need a second handle, which is why rules take `impl AsyncFn() -> F` rather than a made fixture | `spec/SPECIFICATION.md:5126-5155`; `../_decomposition.md:541-544`; `crates/happenstance-testkit/src/registry.rs:45-49` | The fixture shape landed in `projection-suite-entry-point` exists for this rule specifically |
| Testing brief AC-010: **Integration**, not Unit — "'stays usable' is a claim about the store's subsequent behaviour, which a structural assertion cannot see" — and "a hostile mutant (a store whose `Drop` leaks the connection) is what makes this rule non-decorative" | `../_decomposition.md:780` | The mutant for AC-010 is named by shape but **not by identifier** anywhere in the briefs; naming it is a spec-stage deliverable |
| `CLAUDE.md`: never assert on literal position values — the specification permits gaps and a conformant adapter may leave them; `GappedPositionStore` exists to convict a rule that forgets | `CLAUDE.md`, *The rule that matters*; `crates/happenstance-testkit/tests/mutation_coverage/variants.rs:116`; `../_decomposition.md:705-707` | Two rules here are *about* positions (`commit_rejects_a_regressing_position`, `commit_accepts_a_position_the_batch_did_not_write`), so this is live rather than ceremonial |
| PS-7 and PS-8 are both `[FROZEN]`; PS-8 records that `rollback` stays on the port even though a buffered batch could drop, partly to keep `AssertUnwindSafe` honest | `spec/SPECIFICATION.md:4900,4913,4911-4922` | Implemented here, not amended |

## Questions

Open questions to resolve before specifying.

1. **What is the drop-mutant called?** Deferred to `spec` — the briefs name its
   *shape* ("a store whose `Drop` leaks the connection",
   `../_decomposition.md:780`; "a pooled connection `Drop` never returns",
   `spec/SPECIFICATION.md:5665`) but no identifier. It needs one, registered in
   the projection `REGISTRY` with the RUNBOOK probe as its provenance. This is
   the one mutant in this slice that the testing brief does not hand over
   pre-named.
2. **Does `failed_commit_leaves_both_unchanged` require a fault-injection
   capability?** Deferred to `spec`. The event-store fixture has
   `MID_BATCH_FAULT`, defaulted-declined
   (`crates/happenstance-testkit/src/contract.rs:207`); whether the projection
   fixture needs an analogue is `projection-api-design-record`'s capability-set
   answer, and if the answer is yes, the rule is emitted as a reported skip
   rather than omitted.
3. **Where does the PS-15 stamp live?** Deferred to `spec`, with the constraint
   already recorded in `owned-batch-port-shape`: a field on the owned batch,
   minted per store instance, compared as an integer.
4. **Do all seven rules land in this slice, or a named subset?** Deferred to
   `spec` and audited by AC-014 (`../_decomposition.md:672-676`). Any rule not
   landed must leave its clause carrying an accurate maturity marker rather than
   an empty falsifier.
5. **None beyond what the project brief already carries.**

## Decision

Seven of the projection port's rules are about what happens at and around
`commit`, and all seven currently have the same status: the clause exists, the
rule does not, and no store in the tree can fail anything. This slice writes them
and — the half that makes them real — registers a wrong implementation for each,
drawn from the shape §4.11 already names as the thing that rule rejects. The
centrepiece is `dropped_batch_leaves_store_usable`, whose bar is deliberately
stronger than "rolls back": the rule drops a batch bare, then opens and commits a
**second** batch, because the defect that motivated the clause was a pooled
connection whose `Drop` returned to nothing and left the store answering `Busy`
forever. The spec will fix: each rule's text and the exact assertion it makes;
each mutant's identifier, defect and provenance string, including a name for the
drop mutant that the briefs leave unnamed; `ValidatingCommitStore`'s registration
against `commit_accepts_a_position_the_batch_did_not_write`; the per-instance
stamp `commit_rejects_a_foreign_batch` compares, and the two-`open()` shape that
rule needs; and the standing requirement that no rule here compares a checkpoint
against a literal. It edits no `[FROZEN]` clause: PS-7 and PS-8 are frozen and
this slice implements them.

## The wrong implementation

**`dropped_batch_leaves_store_usable` written as "drop the batch, then assert the
read model and checkpoint are unchanged".** It is the obvious rule, it is a fair
reading of PS-7's first conjunct, it passes against `MemoryProjectionStore`, and
it passes against the store the clause was written for — a pooled connection
whose `Drop` returns the handle to nothing, after which every subsequent call
answers `Busy` forever (`RUNBOOK.md:3893-3897`). Nothing was written, so nothing
changed, so the rule is green while the store is permanently unusable. The fix is
one line of rule text: after the drop, **open a second batch and commit it**
(`../_decomposition.md:569-575`). The mutant that convicts the weak version is the
store whose `Drop` leaks the connection — currently named only by shape in the
briefs, and therefore something this slice must name, implement in
`crates/happenstance-testkit/tests/mutation_coverage/` and register in the
projection `REGISTRY` with the reviewer's probe as its provenance. Without that
mutant the strong version and the weak version are indistinguishable to CI.

**`commit_rejects_a_regressing_position` written with literal positions.** Commit
at 1, commit at 2, assert the checkpoint is 2, commit at 1 again, assert it is
still 2. It passes against every fixture in this workspace and it encodes an
assumption the specification explicitly denies: positions may have gaps, and a
conformant adapter may leave them (`CLAUDE.md`, *The rule that matters*). The
event-store side keeps `GappedPositionStore` in the tree — positions in steps of
seven — precisely to convict a rule that forgot
(`crates/happenstance-testkit/tests/mutation_coverage/variants.rs:116`). Here the
positions are caller-supplied, which makes the trap *easier* to fall into rather
than harder: the rule looks correct because it chose the numbers itself. The
requirement is that every assertion compares against the exact `SequencePosition`
value the rule handed to `commit`, and that the rule still passes when those
values are drawn from a gapped sequence.

**`commit_accepts_a_position_the_batch_did_not_write` implemented as a store, not
as a rule.** The subtle one: this rule's job is to reject an adapter that
*validates* the position — `ValidatingCommitStore`, §4.11's own named hostile
store (`spec/SPECIFICATION.md:5682`). A rule that merely commits at a position the
batch did not write and asserts `Ok` is correct; a rule that asserts on the
checkpoint value afterwards without also driving the validating store is a rule
`ValidatingCommitStore` might still pass, and the registry's exactness meta-test
would then record a mutant that fails nothing — caught by
`mutant_registry_is_exhaustive`, which rejects a row whose `fails` list is empty
(`crates/happenstance-testkit/tests/mutation_coverage/racers.rs:12`), but only if
the mutant is registered here rather than deferred.

**And the one that quietly passes forever:**
`distinct_projections_advance_independently` run against a single
`ProjectionId`. The rule compiles, passes, and
never touches the defect it names — a single-row checkpoint table
(`spec/SPECIFICATION.md:5666`). Its mutant is a store with exactly one checkpoint
slot, and if that store is not in the registry, no one finds out.

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

The two judgement boxes, both live here. **Literal positions**: this is the slice
where the box has the most content — two of its seven rules are about position
semantics. The commitment recorded above and carried into `spec` is that every
assertion compares against the exact `SequencePosition` the rule supplied to
`commit`, never against a literal or against an assumption of contiguity, and
that each rule must still pass when its positions come from a gapped sequence.
The wrong implementation section names the failure explicitly so the spec cannot
lose it. **`[FROZEN]` clauses**: PS-7 and PS-8 are frozen
(`spec/SPECIFICATION.md:4900,4913`) and this slice implements the rules they
state without editing either. PS-1's pairing defect and any sibling the sweep
found were repaired by a new accepted atom in `projection-decision-atoms`, in
slice 1.
