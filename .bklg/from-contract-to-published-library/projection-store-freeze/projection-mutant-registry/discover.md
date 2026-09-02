---
item: HS-S0009
stage: discover
created: 2026-08-12T13:01:16.508Z
updated: 2026-08-12T13:01:16.508Z
template_sig: 86ce4036
rendered_sig: a3f6912a
---

# Discover — CheckpointOnlyStore fails the suite by name

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Slice one-line: `CheckpointOnlyStore` fails the suite by name — a projection `REGISTRY: &[Declared]` with the same shape as the event-store one, the three exactness meta-tests, non-empty provenance per entry, ADR-0010's no-pass-rate warning in the module doc, and the stale "neither a suite nor a mutant yet" scope note replaced | `../_storymap.md:61` | The registry is the deliverable; `CheckpointOnlyStore` is the first entry in it and the phase's whole bar |
| **AC-002** — `CheckpointOnlyStore` — a store that commits the checkpoint and silently drops the read-model write — **fails at least one named rule**, and **passes every rule it does not declare**, under the mutant registry's exactness meta-tests | `../project.md:183-186` | Two directions. The second is what `mutants_fail_exactly_their_declared_rules` exists for and is the harder half |
| **AC-003** — every projection rule has a registered wrong implementation that fails it, with stated provenance naming the real implementation mistake it models; **no pass rate is quoted** over the mutant set | `../project.md:187-190`; `.kb/decisions/0010-the-suite-must-prove-itself.md` | This story lands the registry and the meta-tests; the three rule stories each carry their own mutants, which the exhaustiveness meta-test forces (`../_storymap.md:81`) |
| `dependsOn: projection-suite-entry-point` — supplies the fixture trait, the enumeration and the two baseline rules the first mutant must fail | `../_storymap.md:59,61,110` | `CheckpointOnlyStore`'s declared failure is `commit_is_atomic_with_the_read_model`, which lands in that story |
| PS-2 names the mutant, its home and its assertion **verbatim**: "a testkit-internal hostile store, `CheckpointOnlyStore`, in `crates/happenstance-testkit/tests/`, asserted to fail `commit_is_atomic_with_the_read_model`" | `spec/SPECIFICATION.md:4760-4767` | Nothing here is a naming choice. PS-2 is `[FROZEN]` and this is its rule |
| The RUNBOOK states the bar in one sentence: **if it passes, the port is not frozen** | `RUNBOOK.md:3890-3892`; `../project.md:89-93`; `../_grounding.md:186-191` | DoD 1's first half, and the failing test's name is quoted in the closeout (`../project.md:241-243`) |
| The registry shape to mirror: `struct Declared` pairing each mutant with the **exact** set of rules it fails, a `provenance` string naming the real adapter mistake it models (never empty), and an optional per-rule `expect` pin | `crates/happenstance-testkit/tests/mutation_coverage.rs:141-186`; `../_grounding.md:106-124` | Architecture brief AC-A07: same `Declared` shape, same three meta-tests, same no-pass-rate warning (`../_decomposition.md:320-325`) |
| The three meta-tests to sibling: `every_rule_has_a_mutant` (CF-1), `mutant_registry_is_exhaustive` (CF-2, which also requires at least one conformant variant), `mutants_fail_exactly_their_declared_rules` (CF-3) | `crates/happenstance-testkit/tests/mutation_coverage.rs:2734,2754,2889` | "Exactness is what makes the registry a map from rules to the bugs they catch" |
| The mutants module's own doc already names the failure mode: a mutant broken in more ways than it claims "satisfies `every_rule_has_a_mutant` mechanically and proves nothing" | `crates/happenstance-testkit/tests/mutation_coverage/mutants.rs:5-18,71` | The wrong implementation below is written down in the tree, by the people who hit it |
| ADR-0010's no-pass-rate rule: "the denominator is an author's choice, so a fraction says how representative the author was while reading as though it said how good the suite is" | `.kb/decisions/0010-the-suite-must-prove-itself.md`; `crates/happenstance-testkit/tests/mutation_coverage.rs:26-31,203-206`; `../_grounding.md:125-132` | The projection registry's module doc must carry the same warning. Not negotiable at the projection port merely because its suite is new (`../_decomposition.md:398-401`) |
| The stale scope note to replace: "the projection store port owes its own CF-1 – CF-5 obligation and has neither a suite nor a mutant yet" | `crates/happenstance-testkit/tests/mutation_coverage.rs:197-206`; `../_decomposition.md:323-325` | A sentence that becomes false the moment this story lands, and nothing would catch it staying |
| Architecture brief Note 9, *One mutant binary or two*: `mutation_coverage.rs` is 3,527 lines and its `Subject: Fixture` harness and `all_rules()` helper are event-store-bound. A sibling binary is the low-risk read; one binary keeps one place to look. "Either way the three meta-tests must exist for the projection family" | `../_decomposition.md:677-683`; `crates/happenstance-testkit/tests/mutation_coverage/harness.rs:60-63` | A recorded decision at `spec`, with the invariant fixed regardless of which is chosen |
| Testing brief AC-002: **Integration** (the mutant is registered as a fixture and run against the full suite, so the failure is observed by actually calling `begin`/write/`commit` and reading back) + **Unit** (the exactness sibling asserts both directions programmatically, "so the exactness claim is checked once, not eyeballed per run") | `../_decomposition.md:772-773` | The mutant must run, not be inspected |
| §4.11 requires **three** hostile stores, not one: `CheckpointOnlyStore`, `TruncatingResetStore` (a `reset` that clears every id) and `ValidatingCommitStore` (rejects a position the batch did not write) | `spec/SPECIFICATION.md:5678-5685`; `../_decomposition.md:684-688` | `CheckpointOnlyStore` is the *bar*; it is not the whole obligation, because AC-003 requires a mutant per rule. The other two land with their rules |

## Questions

Open questions to resolve before specifying.

1. **One mutant binary or two?** Deferred to `spec` as a recorded decision
   (Architecture brief Note 9, `../_decomposition.md:677-683`). The invariant is
   fixed here: the three meta-tests exist for the projection family whichever
   way it goes, and the `Declared` shape is not forked.
2. **Does the projection harness reuse `Subject: Fixture + Sized`?** Deferred to
   `spec`. The existing convention is at
   `crates/happenstance-testkit/tests/mutation_coverage/harness.rs:60-63` and is
   event-store-bound by its associated type, so a projection analogue is likely
   — but it is a mirror, not a share, for the same reason the fixture trait was.
3. **How is "passes every rule it does not declare" evaluated while the rule set
   is still growing?** Answered: it is re-asserted as later stories add rules,
   which is the storymap's own note (`../_storymap.md:80`). That is a property of
   the meta-test, not a phased weakening of it.
4. **What is `CheckpointOnlyStore`'s provenance string?** Deferred to `spec`, but
   the content is already available: PS-1's **Rejects** clause names "an adapter
   that writes read-model rows on one connection and the checkpoint on another,
   which is the natural shape for any store whose read model lives somewhere
   other than its checkpoint table" (`spec/SPECIFICATION.md:4756-4759`).
5. **None beyond these.** ADR-0010 is accepted and binding; nothing here reopens
   it.

## Decision

A conformance suite with no store that fails it is a suite nobody has checked,
and the projection family currently has zero — the event-store registry's own
scope note says so in the present tense. This slice builds the projection mutant
registry: a `REGISTRY: &[Declared]` carrying the same shape as the event-store
one, the three exactness meta-tests that make it a map from rules to the bugs
they catch, non-empty provenance on every entry, ADR-0010's no-pass-rate warning
in the module doc, the stale scope note replaced — and its first entry,
`CheckpointOnlyStore`, which commits the checkpoint and silently discards the
read-model write, registered as failing `commit_is_atomic_with_the_read_model`
and asserted to pass every rule it does not declare. The spec will fix: one
binary or two, with the reason; the `Declared` fields and the meta-tests' text;
`CheckpointOnlyStore`'s implementation, its declared `fails` set and its
provenance, drawn from PS-1's own **Rejects** clause; and the module doc's
warning. It edits no `[FROZEN]` clause — PS-2 is frozen and this story is the
execution of its stated rule, not an amendment to it.

## The wrong implementation

**A `CheckpointOnlyStore` whose `commit` returns `Err`.** It fails
`commit_is_atomic_with_the_read_model`, which is what AC-002 literally asks for.
It also fails `commit_advances_the_checkpoint`, and every other rule that commits
anything, and it satisfies `every_rule_has_a_mutant` for all of them at once — a
single store "covering" the whole suite. The registry then says nothing about
which rule catches which bug, because one broken store fails everything and the
map from rules to bugs is a constant function. The testkit's own mutants module
warns about exactly this in its first eighteen lines: such a store "satisfies
`every_rule_has_a_mutant` mechanically and proves nothing"
(`crates/happenstance-testkit/tests/mutation_coverage/mutants.rs:5-18`). The check
that convicts it is `mutants_fail_exactly_their_declared_rules`
(`mutation_coverage.rs:2889`), and it only convicts it if the projection family
has its own copy — which is this story's actual deliverable.

**The version that survives that meta-test, and is still wrong:** a registry
whose `fails` lists are written by *running* each mutant and recording whatever
it failed. Every meta-test passes by construction — the declared set and the
observed set are the same set, because one was copied from the other — and the
registry has stopped being a claim about what each rule catches and become a
transcript of what happened. Nothing mechanical can distinguish the two: the
guard is that `fails` is written from the mutant's **intended** defect first, and
a disagreement with the run is a finding to investigate rather than a line to
correct. The `provenance` field is what makes that checkable by a human, which is
why AC-003 requires it non-empty and why `Declared`'s own doc comment already does
(`mutation_coverage.rs:141-151`).

**And the one no test can ever catch:** a module doc that reports "17 of 17
projection rules have a registered mutant". It is true, it is encouraging, it is
a pass rate over a set whose denominator the author chose — "a fraction says how
representative the author was while reading as though it said how good the suite
is" (ADR-0010, restated at `mutation_coverage.rs:26-31`). The testing brief is
explicit that this is a review-time check on the doc comment and not a runtime
assertion, "because 'no pass rate is quoted' is a property of what the code does
**not** print, and there is nothing to assert against an absence except reading
it" (`../_decomposition.md:773`).

**Finally, the sentence that silently rots:** leaving
`mutation_coverage.rs:197-206`'s scope note saying the projection port "has
neither a suite nor a mutant yet". It compiles, it is a comment, and it is false
the moment this story merges. Replacing it is a stated deliverable
(Architecture brief AC-A07) precisely because nothing else would.

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

The two judgement boxes. **Literal positions**: this story adds a registry, three
meta-tests and one hostile store — no conformance rule. `CheckpointOnlyStore`
itself must not assume anything about the positions handed to `commit`; it
records the checkpoint it is given and drops the write set, which is the whole
defect. Vacuously true at the rule grain. **`[FROZEN]` clauses**: PS-1 and PS-2
are both frozen (`spec/SPECIFICATION.md:4735,4763`) and neither is edited. PS-2
*names* this mutant, its directory and its asserted failure, so this story is the
clause's discharge; PS-1's `MUST`-versus-rule gap was repaired by a new atom in
`projection-decision-atoms`, four stories earlier.
