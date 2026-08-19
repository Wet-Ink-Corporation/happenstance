---
item: HS-S0011
stage: discover
created: 2026-08-12T13:01:18.472Z
updated: 2026-08-12T13:01:18.472Z
template_sig: 86ce4036
rendered_sig: 421c333f
---

# Discover — reset is one unit of work, scoped, refusable, and not commit-at-FIRST

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Slice one-line: `reset` is provably one unit of work, scoped and refusable — `reset_clears_rows_and_checkpoint_together`, `reset_is_scoped_to_one_projection`, `refused_reset_changes_nothing`, `fresh_projection_has_no_checkpoint`, and `reset_is_not_commit_at_first` rejecting `commit(empty, id, FIRST)` as a substitute — with `TruncatingResetStore` and the commit-at-first substitute registered as the stores that fail them | `../_storymap.md:63` | Five rules; two mutants are named in the one-line itself, and the fifth rule is the one this project exists to make impossible to skip |
| **AC-011** — `reset` clears rows and checkpoint in one unit of work, is scoped to one `(store, ProjectionId)`, and is refusable; a rule rejects `commit(empty, id, FIRST)` as a substitute, **which silently skips event 1** | `../project.md:215-217` | Sole owner (`../_storymap.md:89`). The parenthetical is the defect, and it is a position bug |
| **AC-003** — every rule has a registered wrong implementation with stated provenance | `../project.md:187-190` | Shared across the four rule-bearing stories; the exhaustiveness meta-test is the forcing function (`../_storymap.md:81`) |
| `dependsOn: projection-mutant-registry, projection-capability-skips` — the first supplies `REGISTRY` and the three exactness meta-tests; the second supplies the declined-capability machinery `refused_reset_changes_nothing` needs to express "this store protects this id" | `../_storymap.md:60-61,63,111` | The refusability rule is the reason the capability edge exists: Architecture brief Note 5 names "a way to say 'this store protects this id from reset'" as a projection-specific capability need (`../_decomposition.md:526-531`) |
| The substitute is named and explicitly wrong, with a count attached: `commit(empty, id, FIRST)` "silently skips event 1 — the substitute all six scenarios reached for and all six got wrong" | `RUNBOOK.md:3904-3907`; `../_grounding.md:197-201` | Six out of six. That is the reason this needs a *named rejecting rule* rather than a paragraph of documentation, and it is the provenance string for the mutant |
| §4.11 names each rule's rejected implementation: `reset_clears_rows_and_checkpoint_together` ← the two-statement runbook procedure; `reset_is_scoped_to_one_projection` ← a checkpoint-table truncate; `reset_is_not_commit_at_first` ← `checkpoint.unwrap_or(FIRST)` with an inclusive `from`; `refused_reset_changes_nothing` ← a refusal reported as success; `fresh_projection_has_no_checkpoint` ← a store that reports `Live { through: FIRST }` for an id it has never seen | `spec/SPECIFICATION.md:5660,5670-5673` | Five mutant shapes, pre-specified. The spec stage names and registers them |
| `TruncatingResetStore` is one of §4.11's three named hostile stores — "a `reset` that clears every id" | `spec/SPECIFICATION.md:5680-5682`; `../_decomposition.md:684-688` | It is `reset_is_scoped_to_one_projection`'s mutant, and it belongs to this slice |
| `probe_delete_all` exists so `reset` (PS-16) is checkable **without the suite knowing what a read model is** | `spec/SPECIFICATION.md:5027`; `../_decomposition.md:476-478` | The seam these rules are written through; landed in `projection-probe-conformance-feature` |
| PS-18: an adapter MUST be able to refuse a reset; `ResetError::Refused` carries no payload, and whether it gains the store's stated reason is `_design.md`'s recorded decision | `spec/SPECIFICATION.md:5200-5217`; `../_decomposition.md:148-157` (AC-U07) | The rule is `refused_reset_changes_nothing`; the payload question is answered upstream and consumed here |
| **PS-19 is `[FROZEN]` and its scope is narrower than the rule the §4.11 table assigns it**: the clause is scoped *after a successful reset*, while `fresh_projection_has_no_checkpoint` asks about an id never seen — "the store that satisfies one and fails the other is the natural implementation rather than a contrivance" | `spec/SPECIFICATION.md:5218-5249`; `../_decomposition.md:644-647`; `.kb/open-questions/ps-19-scope-narrower-than-its-rule.md` | This slice is where the PS-19 defect becomes executable, so the repair atom must already be accepted. See the gate note |
| PS-17 is also `[FROZEN]`, and ADR-0007 already fixes checkpoints per `(store, ProjectionId)` | `spec/SPECIFICATION.md:5188-5190`; `.kb/decisions/0007-projection-runner-decodes.md` | Implemented, not amended |
| Testing brief AC-011: **Integration** (reset, then assert both the read model and the checkpoint are absent for that id and unaffected for a sibling id) + **Unit** (`commit(empty, id, FIRST)` as a hostile mutant's declared failure mode must fail the reset-equivalence rule **by name** — "this is what turns RUNBOOK's observation into an enforced rejection instead of a warning in prose") | `../_decomposition.md:781` | Two tiers, and the Unit half is the named-rejection requirement |
| `Checkpoint` is the three-variant enum — `NeverRun` / `Live` / `Rebuilding` — precisely because the tuple `(Option<SequencePosition>, bool)` can spell `(None, true)`: "authoritative, never run — which means nothing" | `spec/SPECIFICATION.md:4643-4658`; `../_decomposition.md:113-120` (AC-U03) | `fresh_projection_has_no_checkpoint` asserts a **variant**, and that is what makes it different from asserting a position |

## Questions

Open questions to resolve before specifying.

1. **Which capability expresses refusability, and is it a MUST?** Answered
   upstream — the capability set is `_design.md`'s call
   (`../_decomposition.md:526-534`), delivered by `projection-api-design-record`
   and consumed by `projection-capability-skips`. This story uses it; if the
   fixture declines it, `refused_reset_changes_nothing` is emitted as a reported
   skip carrying the fixture's reason, never omitted.
2. **Does `ResetError::Refused` carry the store's reason?** Answered upstream in
   `_design.md` (AC-U07). Either answer is workable for the rule, which asserts
   that *nothing changed*, not why.
3. **What is the commit-at-first mutant called, and is it a new store or an arm
   of `ValidatingCommitStore`?** Deferred to `spec`. Architecture brief Note 9
   floats "`ValidatingCommitStore` or a sibling"
   (`../_decomposition.md:781`); either is legal, and the registry's exactness
   meta-test constrains the choice more than taste does.
4. **Does `fresh_projection_has_no_checkpoint` land here or with the PS-19
   repair?** Answered: here, and only after the repair atom is accepted. The
   ordering is stated in the gate note below rather than left implicit.
5. **None beyond what the project brief already carries.** ADR-0013's globally
   frozen visibility invariant is not touched: nothing in this slice makes a
   checkpoint boundary-scoped
   (`.kb/open-questions/global-versus-per-boundary-visibility-invariant.md`).

## Decision

`reset` does not exist on the port today, and the substitute every deployment
reaches for — commit an empty batch at the first position — is wrong in a way
that is invisible until a rebuilt read model is missing exactly one event. Six
scenarios out of six reached for it. This slice makes that substitution a **named
failing rule** rather than a warning in prose, alongside the four rules that make
`reset`'s other guarantees observable: one unit of work covering rows and
checkpoint together, scoping to a single `(store, ProjectionId)`, refusability
without side effects, and a fresh projection reporting `Checkpoint::NeverRun`
rather than a position. The spec will fix: each rule's text and assertion, with
`fresh_projection_has_no_checkpoint` asserting the `NeverRun` **variant** rather
than any position value; `TruncatingResetStore`'s implementation and registration
against `reset_is_scoped_to_one_projection`; the commit-at-first mutant's
identifier, its declared failure and its provenance (the RUNBOOK's six-for-six
observation); the sibling-id assertion that makes the scoping claim real; and how
`refused_reset_changes_nothing` behaves when the fixture declines the capability.
**It depends on an ADR that must already be accepted**: PS-19's `MUST` is scoped
to the state after a reset while `fresh_projection_has_no_checkpoint` asks about
an id never seen, and PS-19 is `[FROZEN]` — the repair atom from
`projection-decision-atoms` (slice 1) is what makes this rule legitimate rather
than a widening of a frozen clause by test.

## The wrong implementation

**A store whose `reset(batch, id)` is `commit(empty_batch, id, FIRST)`.** This is
the mutant, it is named by AC-011 in those words, and it is what six independent
deployment scenarios actually reached for. It compiles. It returns `Ok`. The
checkpoint moves, so any rule that asserts "the checkpoint changed" passes. Even a
rule that asserts "the checkpoint is no longer where it was" passes. The read
model is untouched and the replay that follows begins *after* the first position,
so event 1 is silently skipped and the rebuilt read model is quietly wrong
forever. It must live in `crates/happenstance-testkit/tests/mutation_coverage/`
as a registered mutant whose declared failure is `reset_is_not_commit_at_first`,
with `RUNBOOK.md:3904-3907`'s six-for-six observation as its provenance —
otherwise the rule is a rule nobody has checked.

**And the rule written to catch it, written wrong:** asserting that after a reset
the checkpoint equals `FIRST`. That is a literal position assertion *and* it is
the exact wrong answer — it is the value the defective store writes. The correct
assertion is on the `Checkpoint` variant: `NeverRun`, which the three-variant enum
exists to make distinguishable from `Live { through: FIRST }`
(`spec/SPECIFICATION.md:4643-4658`). §4.11 says the same thing from the other
side: `fresh_projection_has_no_checkpoint` rejects "a store that reports `Live {
through: FIRST }` for an id it has never seen" (`spec/SPECIFICATION.md:5660`). A
rule that compares positions here cannot tell the two apart, and both mutants walk
free.

**`reset_is_scoped_to_one_projection` run against one projection.** The rule
resets `id`, asserts `id`'s rows and checkpoint are gone, and passes — against
`TruncatingResetStore`, whose `reset` clears **every** id
(`spec/SPECIFICATION.md:5681`). The scoping claim is only observable through a
*sibling* id that must survive, which is why the testing brief specifies exactly
that ("unaffected for a sibling id", `../_decomposition.md:781`). Without the
sibling, `TruncatingResetStore` is a registered mutant that fails nothing, and
`mutant_registry_is_exhaustive` rejects a row whose `fails` list is empty
(`crates/happenstance-testkit/tests/mutation_coverage/racers.rs:12`) — so the
weak rule does not merely under-check, it breaks the registry, which is the
better outcome and worth stating so nobody "fixes" it by trimming the row.

**`refused_reset_changes_nothing` that only checks the return value.** A refusal
reported as success is §4.11's named rejection for this rule
(`spec/SPECIFICATION.md:5673`), and the mirror-image defect — a refusal reported
correctly, after the rows have already gone — passes any rule that stops at
`assert!(matches!(err, ResetError::Refused))`. "Changes nothing" is the operative
half: the rows and the checkpoint must both be exactly as they were, observed
through a fresh handle.

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

The two judgement boxes, and both are load-bearing in this slice. **Literal
positions**: the natural way to write `fresh_projection_has_no_checkpoint` and
`reset_is_not_commit_at_first` is to compare a checkpoint against `FIRST`, and
that is *both* a literal-position assertion and the precise value the defective
store writes. The commitment carried into `spec` is that both rules assert on the
`Checkpoint` **variant** — `NeverRun` versus `Live { .. }` — and that every other
rule here compares against positions the rule itself supplied to `commit`, never
against a literal or an assumption of contiguity. **`[FROZEN]` clauses**: this
slice touches three — PS-17, PS-19 and PS-20
(`spec/SPECIFICATION.md:5190,5221,5253`) — and PS-19 is the one with a real
defect: its `MUST` is scoped to the state after a successful reset, while
`fresh_projection_has_no_checkpoint` asks about an id never seen. No clause text
is edited here. The repair is the **new accepted atom** written in
`projection-decision-atoms` (ADR-0018's range, PS-16 – PS-20), which lands in
slice 1 and therefore strictly before this slice
(`../_storymap.md:107,111`); if the sweep found the defect systematic, that atom's
scope is larger and this story waits on the re-plan rather than proceeding.
