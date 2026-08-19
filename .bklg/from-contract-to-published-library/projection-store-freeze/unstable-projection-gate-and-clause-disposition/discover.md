---
item: HS-S0016
stage: discover
created: 2026-08-12T13:01:23.349Z
updated: 2026-08-12T13:01:23.349Z
template_sig: 86ce4036
rendered_sig: 9ceaacc6
---

# Discover — The module stops lying about its own maturity

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Slice one-line: the module stops lying about its own maturity — `projection.rs`'s provisional block is replaced by an `unstable-projection` gate with a stated reason (AC-014's second arm, per AC-A04), every PS-1 – PS-37 clause carries an accurate maturity marker and rule citation, the sweep's frozen-clause repair lands as a new decision atom rather than a line edit, and `cargo xtask spec-trace` is green | `../_storymap.md:68` | Four deliverables: the gate, the markers, the repair's landing, and a green cross-reference check |
| **AC-014** — `projection.rs` no longer describes itself as provisional, **or** the module is behind `unstable-projection` and says why; every PS-1 – PS-37 clause carries an accurate maturity marker and `cargo xtask spec-trace` is green | `../project.md:225-228` | Sole owner (`../_storymap.md:92`). Two arms, and which one is taken is decided upstream |
| `dependsOn: ps-clause-pairing-sweep, ps3-batch-shape-finding` — the first supplies the per-clause finding that scopes the marker work and the repair; the second supplies the evidence about whether the two shapes disagreed | `../_storymap.md:53,66,68,114` | The dependency on the sweep spans the whole project: slice 1 to slice 8. That is deliberate — the sweep is the input, the disposition is the discharge |
| Architecture brief **AC-A04**: the design states in writing that PS-2's bar is **not** met by anything this project can build alone, and takes AC-014's **second** arm — `unstable-projection` with a stated reason — "rather than deleting the provisional marker" | `../_decomposition.md:307-311,449-456` | The arm is pre-selected by a recorded architectural decision. This story executes it and does not re-open it |
| The text being replaced, verbatim: "**Status: provisional** … this port is **not yet frozen** … the conformance suite does not cover it yet — and a port without a conformance suite is a guess. It will be settled in the pass that lands the first real projection adapter" | `crates/happenstance-core/src/projection.rs:1-11` | Half of it becomes false when the suite lands (there is a suite now) and half stays true (PS-2's bar is unmet). The replacement must be accurate about both |
| PS-3 is the idiom being adopted: "Until PS-2's bar is met the port SHOULD ship behind an off-by-default `unstable-projection` feature, with a documented exemption from semver" — the `tokio_unstable` pattern | `spec/SPECIFICATION.md:4776-4790`; `RUNBOOK.md:3924-3928` | The gate is a clause's discharge, not an invention. "It decouples publishing `EventStore` from settling `ProjectionStore`" |
| PS-2 is `[FROZEN]` and its bar is two **adapters** at opposite ends of the batch-shape axis — not two testkit instruments | `spec/SPECIFICATION.md:4760-4775` | The stated reason on the gate is exactly this. The far end is downstream: Ladybug's graph batch and Neon's no-connection transport (`../project.md:301-307`) |
| The frozen-clause repair discipline: a new decision atom, "**never a line edit**", per both open-question atoms and `CLAUDE.md` | `../project.md:292-300`; `CLAUDE.md`, *Open questions, deliberately unresolved*; `.kb/open-questions/ps-1-states-no-progress-obligation.md` | The repair atom is written in `projection-decision-atoms` (slice 1); this story lands its consequences in the clause table |
| Testing brief AC-014: **Static** — `cargo xtask spec-trace` over the clause table, "exactly the check CLAUDE.md names for this class of claim… maturity markers 'cannot rot into decoration' because spec-trace is a gate step". A `[FROZEN]` clause changing maturity without a new ADR is what spec-trace plus `redkiln validate --kb` are jointly positioned to catch | `../_decomposition.md:784` | Two gate steps, and neither reads a doc comment's honesty — which is where the wrong implementation lives |
| Adding `unstable-projection` widens the feature powersets again: core's combinations go to thirty-two counting `conformance`, host and wasm32 | `../_decomposition.md:483-487,349`; `xtask/src/main.rs` (`hack` and wasm32 powerset steps) | Every combination must compile, including the port gated off entirely |
| Scoping decision AC-014 audits: "whether all seventeen adapter rules land here, or a named subset with the remainder carrying an accurate maturity marker" | `../_decomposition.md:672-676` | If a rule did not land, its clause's marker must say so. This is the audit, and it is the reason this story sits after every rule story |
| Flagged but **not settled here**: nothing owns the post-phase reconciliation, and this project's exit is what forces it. "AC-014 covers only this project's own clauses" | `../project.md:330-332`; `.kb/open-questions/nothing-owns-the-post-phase-reconciliation.md` | A boundary this story must not quietly cross by tidying clauses it does not own |
| Reshape trigger from slice 1: if the sweep returned "systematic", the repair is larger than one ADR and the project's scope is wrong — a re-plan | `../_storymap.md:126`; `../_decomposition.md:724-727` | This story is where a swallowed "systematic" finding would surface as ten quietly widened clauses |

## Questions

Open questions to resolve before specifying.

1. **Which AC-014 arm?** Answered upstream: the second, per Architecture brief
   AC-A04 (`../_decomposition.md:307-311`). The first arm — deleting "provisional"
   — is available only if PS-2's bar is met, and it is not.
2. **What exactly does `unstable-projection` gate — the module, or the
   re-exports?** Deferred to `spec`. The constraint recorded: whatever is gated,
   the feature powersets must stay green in both directions, including the
   configuration where the port is absent entirely, and no intra-doc link may
   point into a gated module (`crates/happenstance-testkit/src/lib.rs:112-132`).
3. **Does the semver exemption get documented, and where?** Deferred to `spec`.
   PS-3 asks for "a documented exemption from semver"
   (`spec/SPECIFICATION.md:4776-4777`); the natural home is the module doc and
   the crate's feature table, and it is a doc obligation rather than an adapter
   obligation.
4. **Which clauses does this story's marker audit cover?** Answered: PS-1 –
   PS-37 and no further. The post-phase reconciliation across other clause
   families is explicitly unowned and not absorbed here
   (`../project.md:330-332`).
5. **What if a rule from §4.11 did not land?** Answered: its clause carries an
   accurate marker naming what is missing, rather than a marker implying it is
   covered. That is the audit AC-014 performs
   (`../_decomposition.md:672-676`).

## Decision

`crates/happenstance-core/src/projection.rs` opens by saying the port has no
conformance suite and will be settled when the first real projection adapter
lands. By the time this story runs, the first half is false — there is a suite,
and a hostile store fails it by name — while the underlying caution is still
correct, because PS-2's bar is two *adapters* at opposite ends of the batch-shape
axis and both ends are downstream. A module that says the wrong true thing is
worse than one that says nothing. This slice replaces the provisional block with
AC-014's second arm: the port ships behind an off-by-default
`unstable-projection` feature carrying a stated reason and PS-3's documented
semver exemption, and every PS-1 – PS-37 clause is audited so its maturity marker
and rule citation say what is actually true after this project's work — including
for any rule that did not land. The spec will fix: the new module doc's text and
what it claims; what `unstable-projection` gates and the powerset combinations
that must stay green; where the semver exemption is documented; the per-clause
marker audit, driven by the sweep's finding and by which rules actually shipped;
and how the frozen-clause repair from `projection-decision-atoms` is reflected in
the clause table. **No `[FROZEN]` clause text is edited here.** Where PS-1, PS-19
or a sweep-identified sibling changes, the change is the consequence of the new
accepted atom written in slice 1, and `redkiln validate --kb` plus
`cargo xtask spec-trace` are jointly the check.

## The wrong implementation

**Deleting the "Status: provisional" block because the suite is green.** This is
AC-014's first arm taken on the wrong evidence, and it is the single most likely
thing to happen at this point in the project: seventeen rules pass, two batch
shapes pass, `CheckpointOnlyStore` fails by name, and the paragraph saying "the
conformance suite does not cover it yet" is now plainly false. Removing it makes
`cargo xtask ci` green — it was green before — and makes `cargo xtask spec-trace`
green, because spec-trace checks citations and markers, not whether a doc comment
is honest. The port is then advertised as settled while PS-2, a `[FROZEN]`
clause, still requires two adapters at opposite ends of the axis and explicitly
**Rejects** the monoculture this project can produce alone
(`spec/SPECIFICATION.md:4760-4775`). Nothing in the tree convicts it. The guard is
AC-A04's pre-recorded choice of the second arm
(`../_decomposition.md:307-311,449-456`), and the spec must require the gate's
stated reason to name PS-2's unmet bar specifically rather than gesture at
caution.

**Editing a `[FROZEN]` clause's text so the markers line up.** The mechanical
temptation of a marker audit: PS-1's `MUST` does not entail
`commit_advances_the_checkpoint`, the table now cites a rule that exists, and one
extra sentence in the clause would make everything consistent. `spec-trace` goes
green — greener, since the citation now resolves cleanly — and a frozen clause
has been changed by an edit, which `CLAUDE.md` forbids in those words and which
`redkiln validate --kb` cannot see because no atom moved. The repair is the
accepted atom from `projection-decision-atoms`, and this story's job is to
reflect its consequences, not to author them late.

**A marker audit that marks an unlanded rule as covered.** If the scoping
decision was "a named subset of §4.11's seventeen"
(`../_decomposition.md:672-676`), then some clause is left with no rule — and the
cheapest way to a green `spec-trace` is to cite the rule that was *planned*.
Nothing catches a citation to a rule that exists but does not test that clause;
spec-trace checks that the reference resolves. The audit's output must therefore
be per-clause and explicit about absence, which is the same discipline
`ps-clause-pairing-sweep` established for the entailment question.

**And the one that surfaces here having been swallowed earlier:** ten frozen
clauses quietly widened under cover of a marker-disposition story, because the
sweep said "systematic" and nobody escalated. The escalation route is named
(`../_storymap.md:126`), the budget assumption is named
(`../_decomposition.md:659-661`), and this is the last point at which reporting it
is cheaper than living with it.

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

The two judgement boxes, and the seventh is the whole subject of this story.
**Literal positions**: no conformance rule is added — the deliverables are a
feature gate, a module doc and a clause-table audit. Vacuously true.
**`[FROZEN]` clauses**: this story reads every frozen `PS` clause and edits none.
PS-2 is cited as the reason for the gate; PS-1, PS-19 and any sweep-identified
sibling have their disposition recorded as the consequence of the **new accepted
atom** written in `projection-decision-atoms` in slice 1 — written first, by four
slices — and the clause bodies themselves are not touched. The wrong
implementation section names the line edit explicitly because `spec-trace` would
go green on it.
