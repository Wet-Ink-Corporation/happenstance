---
item: HS-S0057
stage: discover
created: 2026-08-12T13:02:19.168Z
updated: 2026-08-12T13:02:19.168Z
template_sig: 86ce4036
rendered_sig: 83ee5219
---

# Discover — CF-14 and CF-27 re-read on this runtime, and the ES-32 verdict on disk

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| The slice's one-line: write the per-clause statement of whether CF-14's and CF-27's deferrals still hold on this runtime (handing CF-27's completeness half onward), and the one-paragraph ES-32 tail-seam verdict in the runbook's ledger — recorded, not acted on — with `cargo xtask spec-trace` still green | `_storymap.md`, **Slices**, `deferral-re-reads-and-es-32-verdict` row | Three statements and a green trace. The only story in the project whose output is entirely prose |
| AC-009 (CF-14 and CF-27 re-read, with a written statement per clause; CF-27's completeness half handed on rather than answered) and AC-010 (the ES-32 verdict on disk, recorded and not acted on) | `project.md`, **Acceptance criteria** | Sole owner of both |
| `dependsOn: every-rule-under-workerd` — the re-read is an observation about what this runtime *did* under the suite, not a re-reading of documentation | `_storymap.md`, **Slices** | The edge is the entire guard against this story being writable on day one |
| CF-14 is `[DEFERRED]`, checked by `acknowledged_writes_survive_a_reopen` | `spec/SPECIFICATION.md:8725` | Durability, and its far end belongs to another project |
| CF-27 is `[DEFERRED]`, and the rules it names — `suffix_store_is_distinguishable_from_a_young_store` and `positions_are_not_reused_after_removal` — are not in `crates/happenstance-testkit/src/suite.rs` at all | `spec/SPECIFICATION.md:8738`, `:4309`, `:8045`; absent from the suite | Two halves, only one re-readable here, and neither yet written as a rule — which is what `[DEFERRED]` means and what the re-read must not paper over |
| CF-14's far end is `sqlite-durable-store`'s (HS-P0012); CF-27's completeness half is `retention-and-incomplete-logs`' (HS-P0018) | `project.md`, **Out of scope** | This story re-reads the deferrals on *this* runtime and hands the far ends on by name |
| `REOPEN`'s documentation names a Durable Object as the motivating case — storage outlives the isolate, so handle state can be discarded and the store read again, while restarting the isolate from inside a test is not possible | `crates/happenstance-testkit/src/contract.rs:163-173` | This runtime is the one CF-14's capability was written around, which is what makes the re-read evidence rather than paperwork |
| `acknowledged_writes_survive_a_reopen` is on the testkit's `MUST_SKIP` list — a rule gated on a capability a fixture may honestly decline, and therefore one that must *report* a skip rather than pass | `crates/happenstance-testkit/tests/mutation_coverage.rs:3144-3164` | A decline is a legitimate answer here, and the reason it carries is an input to this story |
| ES-32 is `[PROVISIONAL]` with no rule, and the specification's own audit calls the clause **correct as a clause** because the absence of a method is not observable by a rule, with ES-30 and ES-15 as its checkable substitutes | `spec/SPECIFICATION.md:4021`, ledger row `:8614`, audit row `:8765` | There is nothing to test. That is why the deliverable is a paragraph, and why the paragraph has to be falsifiable on its own terms |
| The ES-32 verdict is one paragraph in the runbook's ledger, recorded and explicitly not acted on; reopening the tail seam is post-0.1 and outside this initiative | `RUNBOOK.md:4273-4275`, exit criterion `:4300`; `project.md`, **Out of scope** | Scope discipline is half the acceptance criterion |
| `cargo xtask spec-trace` is a gate step precisely so markers and citations cannot rot into decoration, and CF-14, CF-27 and ES-32 all carry citations this story touches | `CLAUDE.md`, **Open questions**; `_decomposition.md`, Architecture brief Notes §7.5 | The trace keeps citations resolvable. It cannot keep them meaningful |
| No test asserts prose, so AC-010 is a review-gate item rather than a tier | `_decomposition.md`, Testing brief, **Acceptance Criteria**, AC-010 row | Stated in the brief already: this story has no automated check and must say who reads it |

## Questions

**Does CF-14's deferral still hold on this runtime?** Deferred to **spec**,
deliberately, because the answer is downstream of two things this story does not
decide: what `durable-object-host-and-fixture` declared for `REOPEN`, and what the
run then showed. What *is* settled now is the form — a written statement per
clause, naming which half this runtime answered and which it handed on, and citing
the run it was taken from rather than the clause it restates.

**CF-27.** Answered in shape: the completeness half is not answered here at any
price — it goes to `retention-and-incomplete-logs` (HS-P0018) by name. What is
re-read is narrower and is genuinely this runtime's to say: whether a Durable
Object's exclusive, non-forgetting storage changes the *premise* the deferral was
taken under, which is a different question from whether a retention-configured
store may forget.

**ES-32: does a Durable Object's storage API make a tail or subscription seam
cheap enough to reopen post-0.1?** Deferred to **spec**, with two constraints
fixed now: the answer is one paragraph, and it names the specific storage surface a
tail would be built on. Acting on it is post-0.1 and outside this initiative; a
verdict that shades into a design sketch has left scope.

**If a re-read finds a deferral no longer holds, does the marker move?** Answered:
**not here.** Moving a `[DEFERRED]` marker requires the far end, and both far ends
sit in other projects (HS-P0012 and HS-P0018). This story writes the statement and
leaves the marker; whoever owns the far end moves it, citing this statement.

**Who reads the three paragraphs, given that nothing tests prose?** Answered in
principle and settled at spec: this is a review-gate item, and the spec names the
reviewer and the falsifiability test they apply — see the mutant below.

**CF-39 / CF-40's fixture-limits ownership, and the off-tokio harness shape.** Not
this story's; they belong to `measured-store-limits`,
`adr-0023-and-atom-resolutions` (coordinated with HS-P0012) and
`every-rule-under-workerd`.

## Decision

A deferral is a decision to decide later on evidence that did not exist yet, and
this project produces exactly the evidence three of them were waiting on: CF-14's
durability question was deferred partly because no store in the workspace had
storage that outlives its execution context, CF-27's was deferred against stores
that could not distinguish a suffix from a young log, and ES-32's tail seam was
closed at 0.1 without anyone having asked what a Durable Object's storage API would
make it cost. This slice re-reads all three against what the runtime actually did
under the conformance run, writes one statement per clause, and puts the ES-32
verdict in the runbook's ledger — recorded, and not acted on. It hands CF-27's
completeness half to `retention-and-incomplete-logs` and CF-14's far end to
`sqlite-durable-store` by name rather than answering either. The spec will cover:
the three statements and, for each, the specific run output or fixture declaration
it cites; the `RUNBOOK.md` ledger row and phase-9 exit tick; the reviewer and the
falsifiability test applied to prose that no test can check; and the confirmation
that `cargo xtask spec-trace` stays green with no marker moved. Nothing `[FROZEN]`
is touched — all three clauses are `[DEFERRED]` or `[PROVISIONAL]`, and no marker
moves in this story at all.

## The wrong implementation

**The re-read that reads the old text.** Three well-written paragraphs, each
restating why its deferral was taken, sourced from `spec/SPECIFICATION.md` and the
long-form records — plus an ES-32 verdict saying a tail seam "looks feasible on
Durable Objects" without naming a single storage surface it would be built on.
Every check passes, and would have passed had the `workerd` run never happened:
`cargo xtask spec-trace` verifies that citations resolve and markers are not stale,
not that a sentence is evidence; no test asserts prose, which the testing brief
states plainly for AC-010; `redkiln validate --kb` sees nothing because nothing is
written to `.kb/` here; and a reviewer reading the diff sees careful, well-cited
paragraphs that are careful and well-cited about the wrong thing. The tell is
falsifiability: **a statement that would read identically if the run had never
happened is not a re-read.** This is the only story in the project with no
executable artefact behind it, which is precisely why it is where "recorded" can
quietly substitute for "observed" — the same substitution the whole project exists
to stop one level up, where "builds for `wasm32`" stood in for "runs on `wasm32`".

**Where the detector lives — and the honest answer is that no test can hold it.**
Not in `crates/happenstance-testkit/tests/mutation_coverage.rs`: its rows are
stores that fail named rules, and there is no rule here to fail; ES-32's own
specification audit says as much, calling the clause correct precisely because the
absence of a method is not observable by a rule
(`spec/SPECIFICATION.md:8765`). What can exist is a **citation obligation**, and
the spec must impose it: each of the three statements names the observation it was
taken from — the `REOPEN` value the fixture declared and its stated reason; the
`SKIP` or pass line `acknowledged_writes_survive_a_reopen` actually produced under
the wasm32 run; and, for ES-32, the specific `SqlStorage` surface a tail would be
built on and what it would cost. A statement that cannot name its observation is a
paraphrase and is rejected at review. `cargo xtask spec-trace` keeps the citations
resolvable (`CLAUDE.md`, **Commands**); it cannot keep them meaningful, and this
story's spec must name the reviewer who does.

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

**Box 6.** No conformance rule is added here, and no code at all. The story's
whole output is three written statements and a runbook ledger row; nothing in it
asserts a position, literal or otherwise.

**Box 7.** Nothing `[FROZEN]` is touched. CF-14 and CF-27 are `[DEFERRED]` and
ES-32 is `[PROVISIONAL]`, and **no marker is moved by this story** — moving one
requires the far end, and both far ends belong to other projects (HS-P0012 and
HS-P0018). If a re-read had found that a *frozen* clause could not hold on this
runtime, the response is a new decision atom and a re-plan written first, which is
the intake brief's own standing instruction.

**Box 8.** No conformance rule here seems wrong. The one clause that has no rule
at all — ES-32 — is correct as a clause for the reason the specification's own
audit gives, that the absence of a method is not observable by a rule, and this
story records a verdict about it rather than inventing a decorative rule to carry
it.
