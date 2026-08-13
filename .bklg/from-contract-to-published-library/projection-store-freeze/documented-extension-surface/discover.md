---
item: HS-S0015
stage: discover
created: 2026-08-12T13:01:22.409Z
updated: 2026-08-12T13:01:22.409Z
template_sig: 86ce4036
rendered_sig: 24dad355
---

# Discover — DT-8's arm discharged — the suite's bar held for the author it was chosen for

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Slice one-line: DT-8's outside-author arm, discharged as chosen — if the bar is held for an author nobody here supervises, the documented pair `projection_store_conformance!` + `ProjectionProbe` is written up as an extension surface and a fixture built **from that documentation alone — not copied from `fixtures.rs`** — clears the mutant-registry exactness check and the capability-skip rule; if the narrower arm was taken, the recorded scope is shown to match what the suite actually holds implementers to | `../_storymap.md:67` | The story's shape is conditional on an upstream decision, and both arms have a deliverable |
| **AC-007** — DT-8 is resolved in `_design.md`. If the bar is held for an outside author, a documented extension surface exists and **AC-002 and AC-005 are demonstrated against a fixture written from the documentation alone** | `../project.md:201-203` | The arm is chosen in `projection-api-design-record`; discharged here (`../_storymap.md:85`) |
| `dependsOn: projection-api-design-record, buffering-conformant-variant` — the first chooses the arm; the second means the whole suite exists to be run against an outside fixture | `../_storymap.md:55,65,67,113` | "Scope depends on which DT-8 arm slice 1 recorded" (`../_storymap.md:113`) |
| UX brief **AC-U02** — the extension surface's cost must remain "one feature flag on a dependency the adapter already has, and no new edge in the graph"; if the narrower arm is taken, the recorded scope says so **where an outsider meets it** — the testkit's own crate doc — and not only in `_design.md` | `../_decomposition.md:97-108` | Both arms have a *published* obligation, not merely a recorded one |
| The coherence argument this story tests in practice: an adapter's `tests/` directory is a **different crate**, where neither a testkit trait nor the adapter's type is local, so an out-of-crate impl is rejected by the orphan rule and the adapter is forced into a non-dev dependency on `happenstance-testkit` | `spec/SPECIFICATION.md:5015-5031`; `../_decomposition.md:464-472` | This is why `ProjectionProbe` lives in `happenstance-core` (AC-A02). **Nothing inside this workspace can falsify that placement — this story is the only thing that can** |
| Architecture brief Note 9, *DT-8's blast radius*: the extension surface is the documented pair `projection_store_conformance!` + `ProjectionProbe`, and the probe's home in the contract crate "is what makes that surface cost an outside author one feature flag instead of a new dependency edge" | `../_decomposition.md:689-693`; `spec/SPECIFICATION.md:5015-5031` | The claim is precise and therefore checkable: count the edges the outside fixture adds |
| The fixture trait **must** be in `__private`, or the macro expansion cannot name it in the adapter's crate | `crates/happenstance-testkit/src/lib.rs:362-363`; `../_decomposition.md:352` | A mount point whose absence is invisible inside this workspace and fatal outside it — exactly what this story exercises |
| The checks the outside fixture must clear: AC-002's mutant-registry exactness (`mutants_fail_exactly_their_declared_rules`) and AC-005's capability-skip rule | `../project.md:183-186,194-197`; `crates/happenstance-testkit/tests/mutation_coverage.rs:2889,3184` | Not "it compiles" — it must clear the same two bars the in-tree fixtures do |
| Testing brief AC-007: **Integration**, conditional on the arm — a fixture built from the documented extension surface, "not copied from `fixtures.rs`", run against the mutant registry and the capability-skip rule, "proving the documented surface is sufficient on its own". Narrower arm: **Static**, "the recorded scope matches what the suite actually holds implementers to" | `../_decomposition.md:777` | The negation — *not copied* — is the acceptance criterion. It is also unenforceable by any tool |
| The reference implementation an author would copy from if permitted: `MemoryFixture`/`MemoryHandle`, the owned-handle pattern | `crates/happenstance-testkit/src/fixtures.rs:243-292` | Naming the file that must **not** be opened is the only way to make "written from documentation alone" operational |
| DT-8's stated failure: "a suite's own permissiveness becomes publicly scrutinised the moment outsiders make claims with it" | `../_decomposition.md:105-107` (quoting the initiative charter's DT-8) | The narrower arm is legitimate; the narrower arm *unqualified in public* is not |
| P2, the adapter author, wants "an executable definition of 'correct' they can run against their own storage system, rather than a prose specification they have to interpret", and fears "that the port quietly assumed something their storage cannot provide, discovered late" | `../_decomposition.md:57` (UX brief persona table) | This story is the only place in the project where that fear is tested rather than assumed away |

## Questions

Open questions to resolve before specifying.

1. **Which arm?** Answered upstream in `_design.md` by
   `projection-api-design-record` (AC-007, `../project.md:201-203`). This story's
   spec must branch on it explicitly rather than assuming the outside-author arm,
   because the narrower arm has a real deliverable too — the published scope
   statement in the testkit's crate doc.
2. **Where does an "outside" fixture live, given there is no outside crate?**
   Deferred to `spec`. It must at minimum be a crate where neither
   `happenstance-testkit`'s traits nor the fixture's own store type would be
   local under the orphan rule if the probe lived in the testkit — otherwise the
   exercise proves nothing about the placement it exists to validate.
3. **How is "written from the documentation alone" made checkable?** Deferred to
   `spec`, and it is this story's hardest question. No tool can enforce it. The
   available levers are procedural: name the files that must not be opened
   (`crates/happenstance-testkit/src/fixtures.rs` foremost), and record every
   point at which the documentation was found insufficient — because those gaps
   are the actual output of the exercise.
4. **What counts as "no new edge in the graph"?** Answered: the outside fixture's
   manifest gains a feature flag on a dependency it already has, and no new
   dependency (`../_decomposition.md:689-693`). That is countable, and the spec
   should require the manifest diff as the evidence.
5. **None beyond what the project brief already carries.**

## Decision

The suite's whole promise is to an adapter author, and every fixture that has run
it so far was written by the people who wrote the suite — which means the claim
that the documented surface is sufficient has never been tested. This slice
discharges DT-8's chosen arm. If the bar is held for an outside author, the
extension surface — `projection_store_conformance!` plus `ProjectionProbe`, and
the `__private` exports the macro expansion needs — is written up as
documentation, and a fixture is built from that documentation alone, in a
position where the orphan rule applies as it would to a stranger, and made to
clear AC-002's exactness meta-test and AC-005's capability-skip rule. If the
narrower arm was recorded, the deliverable is the published scope statement in the
testkit's own crate doc plus the demonstration that it matches what the suite
actually enforces. The spec will fix: the branch on the recorded arm; where the
outside fixture lives and why that location makes the orphan-rule exercise real;
the ban on copying from `crates/happenstance-testkit/src/fixtures.rs`, with the
gaps found recorded as the exercise's primary output; the manifest-diff evidence
for the one-feature-flag-no-new-edge claim; and, for the narrower arm, where the
scope statement is published. No `[FROZEN]` clause is touched.

## The wrong implementation

**A fixture written "from the documentation" with
`crates/happenstance-testkit/src/fixtures.rs` open in the next tab.** It compiles,
it clears the exactness meta-test, it clears the capability-skip rule, AC-007
reads as demonstrated, and the exercise has proved exactly nothing — because the
question was never "can a fixture be written" but "can it be written from what we
published". No tool distinguishes the two: the resulting code is the same code.
The storymap says "not copied from `fixtures.rs`" in the story's own one-line
(`../_storymap.md:67`) and the testing brief repeats it
(`../_decomposition.md:777`), which is the strongest form the constraint can take
— procedural. The spec must therefore make the exercise's *output* the record of
where the documentation ran out, not the fixture itself. A fixture that was built
with no gaps found is either a triumph or a copy, and only the record tells you
which.

**And the placement failure this story exists to convict.** If
`ProjectionProbe` had been defined in `happenstance-testkit` rather than
`happenstance-core` — the wrong implementation named in
`projection-probe-conformance-feature` — every check in this workspace would still
be green, because every fixture here lives in a crate that already depends on the
testkit. The first thing that fails is *this* story: the outside fixture's `impl
ProjectionProbe for TheirStore` sits in a crate where neither the trait nor the
type is local, the orphan rule rejects it, and the only escape is a **non-dev
dependency** on `happenstance-testkit`
(`spec/SPECIFICATION.md:5015-5031`). That is the mutant for AC-A02, it lives here
and nowhere else, and it is the reason this story cannot be dropped as
documentation polish. The same applies to the fixture trait's `__private` export
(`crates/happenstance-testkit/src/lib.rs:362-363`): omit it and every in-workspace
harness still compiles.

**The narrower arm's version, and it is quieter:** recording "the suite's bar is
held for adapters written in this workspace" in `_design.md` and nowhere else.
`_design.md` is a backlog artefact; the person it protects is reading
`happenstance-testkit`'s crate documentation on docs.rs. The suite is then
permissive in fact and unqualified in public, which is the failure DT-8 names
(`../_decomposition.md:105-107`). The check is AC-U02's placement requirement —
the scope statement lands where an outsider meets it — and the instrument is the
design review, since nothing compiles a crate doc's honesty.

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

The two judgement boxes. **Literal positions**: this story adds documentation and
one fixture, and runs rules that already exist — it adds no conformance rule.
Vacuously true. **`[FROZEN]` clauses**: none is touched; the surface here is the
testkit's public API and its documentation. The eighth box is ticked with intent:
this is the story most likely to *find* a rule or a documented surface wrong,
because it is the first time either is used by someone without the authors'
context — and the discipline recorded above is that every such gap is written
down as the exercise's output rather than patched around.
