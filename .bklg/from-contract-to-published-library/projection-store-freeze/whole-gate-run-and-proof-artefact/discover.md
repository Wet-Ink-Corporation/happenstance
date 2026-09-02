---
item: HS-S0017
stage: discover
created: 2026-08-12T13:01:24.044Z
updated: 2026-08-12T13:01:24.044Z
template_sig: 86ce4036
rendered_sig: 59ccd5ed
---

# Discover — One clean cargo xtask ci run, and the proof artefact recorded

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Slice one-line: one full `cargo xtask ci` on a clean checkout — every projection rule passing under the wasm32 emitter in the same run as the host emitters via the existing conformance-harness check step, the skeletons compiling, `cargo hack` over the widened feature powersets — with the proof artefact recorded: **the name of the test `CheckpointOnlyStore` fails, and both passing batch shapes** | `../_storymap.md:69` | The gate run is the precondition; the recorded artefact is the deliverable |
| **AC-016** — every projection rule runs and passes under the `wasm32` emitter in the same `cargo xtask ci` run as the host emitters, not as a separately maintained subset | `../project.md:232-234` | `projection-suite-entry-point` landed the harness file; this story verifies the whole rule set (`../_storymap.md:94`) |
| **AC-013** — the skeletons compile with no change other than the lifetime removal, **and `cargo xtask ci` is green on the workspace** | `../project.md:221-224` | `owned-batch-port-shape` owned the reviewed diff; this story owns the whole-workspace gate (`../_storymap.md:91`) |
| `dependsOn: unstable-projection-gate-and-clause-disposition, documented-extension-surface` — the last two things that change the tree before the run means anything | `../_storymap.md:67-69,114` | The gate run must be the *last* thing, or it is a gate run of an earlier project |
| **DoD 8** — "a green gate is a **precondition** for looking at items 1 and 2, never a substitute for them" | `../project.md:255-256`; `../_storymap.md:114` | The single most important constraint on this story, and the one the wrong implementation below violates |
| DoD 1 and DoD 2, the two things the artefact records: `CheckpointOnlyStore` **fails** the projection suite by name, with the failing test's name quoted in the closeout; and two structurally unlike batch shapes **pass** | `../project.md:241-245` | Named test, named fixtures. Not "the suite is green" |
| The wasm32 mechanism, already existing: `cargo check -p happenstance-testkit --tests --target wasm32-unknown-unknown` type-checks the conformance harnesses. "AC-016 costs this project a new harness file, not a new gate step — which is the whole point of citing this pattern instead of inventing a wasm-specific CI job" | `../_decomposition.md:786`; `xtask/src/main.rs` (wasm32 conformance-harness check); `crates/happenstance-testkit/tests/memory_conformance_wasm.rs` | The step is mandatory and already runs. The risk is not the step; it is what the harness contains |
| `--fast` is defined as *not* running the mandatory wasm32 steps or `spec-trace`, and two of this project's sixteen ACs are proven by exactly those two steps. "A story-grain `--fast` pass is real progress but is not evidence for AC-014 or AC-016" | `../_decomposition.md:828-853`; `CLAUDE.md`, *Commands* | This project's DoD overrides the non-terminal default. The full run is the project-boundary bar |
| What `cargo xtask ci` runs: fmt, clippy `-D warnings`, tests, four wasm32 steps, docs, `spec-trace`, a `--no-default-features` doc build of `happenstance-core`, a `cargo package --list` licence/README assertion, then `cargo hack` feature-powerset, `cargo deny`, a wasm32 feature-powerset, and a nightly `--cfg docsrs` rustdoc build | `CLAUDE.md`, *Commands*; `xtask/src/main.rs` | Both powersets now cover `conformance` and `unstable-projection`, which took core's eight combinations to thirty-two (`../_decomposition.md:483-487`) |
| `cargo-hack` and `cargo-deny` both resolve on this machine, so those steps run rather than printing `skipped` — "a step skips only when its probe fails to find the tool" | `CLAUDE.md`, *Commands* | A "green" run that skipped a step is a different claim, and the artefact must be able to tell them apart |
| Testing brief **E2E** tier: `cargo xtask ci` run whole, from a clean workspace state, is the proof artefact DoD 8 requires "as a *precondition* rather than a substitute for looking at items 1 and 2" | `../_decomposition.md:821-826` | The brief states the same constraint from the testing side, in the same words |
| Testing brief AC-004's E2E half: both fixtures' runs must happen **inside the same `cargo xtask ci` invocation**, "so the proof artefact is one gate run naming both fixtures, not two separate `cargo test` invocations a reviewer has to reconcile by hand" | `../_decomposition.md:774` | Same-run is a property of the artefact, and this story is where it is checked |
| The MSRV is the one thing the local gate does not check; CI carries a dedicated `msrv` job | `CLAUDE.md`, *Commands* | A locally green `cargo xtask ci` is not the whole bar, and the artefact should not imply it is |
| AC-013's skeleton bar is Static and compile-only, "because every affected method body is still `todo!()`" — and `happenstance-postgres`/`happenstance-ladybug`/`happenstance-sqlite` are explicitly **not** fixtures here | `../_decomposition.md:783,882-886` | "No story should attempt to flesh out a skeleton body to make it Integration-provable; that adapter's own project owns doing so" |

## Questions

Open questions to resolve before specifying.

1. **Where does the proof artefact live?** Deferred to `spec`. It must be
   citable by the closeout, which quotes the failing test's name
   (`../project.md:241-243`), and by `ladybug-projection-store` when it writes the
   freeze verdict.
2. **How is `CheckpointOnlyStore`'s failure demonstrated in a run whose exit code
   must be zero?** Deferred to `spec`, and it is a genuine mechanical question:
   the mutant is asserted to fail *by* the mutant-registry harness, so the gate is
   green precisely because the mutant failed where it was declared to. The
   artefact must quote the harness's own assertion rather than an inverted test
   run by hand.
3. **What distinguishes a step that ran from a step that skipped?** Deferred to
   `spec`. `cargo xtask ci` prints `skipped` when a tool probe fails
   (`CLAUDE.md`, *Commands*); the artefact should record which optional steps
   actually executed, because a green run with four skips is a weaker claim.
4. **Does the artefact assert anything about the MSRV?** Answered: no. That is
   CI's `msrv` job, not the local gate, and the artefact should say so rather
   than imply coverage it does not have.
5. **None beyond what the project brief already carries.** No skeleton body is
   fleshed out here (`../_decomposition.md:882-886`).

## Decision

Every claim this project makes is a claim about one run: that seventeen
projection rules pass on three runtimes, that a hostile store fails a named rule,
that two structurally unlike batch shapes are green, that the skeletons still
compile, and that the widened feature powersets hold. This slice performs that
run whole, on a clean checkout, and records what it showed — specifically the two
things DoD 1 and DoD 2 ask for, which are a **test name** and **two fixture
names**, not a colour. The spec will fix: the artefact's location and its
required contents (the failing test's name, both passing fixtures' names, which
optional gate steps ran versus skipped, and what the run does *not* cover);
verification that every projection rule — not a subset — reaches the wasm32
emitter through the single enumeration; and the ordering, since this must be the
last change in the project or it is evidence about an earlier one. It touches no
`[FROZEN]` clause and adds no rule.

## The wrong implementation

**A proof artefact that says `cargo xtask ci`: green.** It is true, it is the
hardest-won sentence in the project, and it is the substitution DoD 8 exists to
forbid in exactly these words: "a green gate is a **precondition** for looking at
items 1 and 2, never a substitute for them" (`../project.md:255-256`). A green
gate is consistent with a `CheckpointOnlyStore` that was quietly removed from the
registry, with a second batch shape that is the oracle wearing a hat, and with a
wasm harness running two rules out of seventeen. The artefact's content is
therefore specified as nouns rather than a status: the **name of the test**
`CheckpointOnlyStore` fails, and the **names of both** passing fixtures
(`../_storymap.md:69`). No tool enforces this; the review does, and only if the
spec says what the artefact must contain.

**A `projection_conformance_wasm.rs` that lists its rules by hand.** The gate's
wasm32 step is `cargo check -p happenstance-testkit --tests --target
wasm32-unknown-unknown` (`../_decomposition.md:786`) — it *type-checks* the
harnesses. A harness that invokes three rules explicitly type-checks exactly as
well as one that expands from `for_each_projection_store_rule!`, so the step is
green either way, and AC-016's "not as a separately maintained subset" is false
with nothing to say so. The structural guard was built in
`projection-suite-entry-point` — one enumeration, driving all three emitters —
and this story's job is to **verify it held**, by checking that the wasm harness
expands from the enumeration rather than from a list. That verification is the
whole of AC-016's second half, and it is the only thing standing between the
constrained-target persona and a suite that silently stopped covering them.

**A run that is green because steps skipped.** `cargo xtask ci` prints `skipped`
when a tool probe fails to find `cargo-hack`, `cargo-deny` or a nightly toolchain
(`CLAUDE.md`, *Commands*). Both tools resolve on this machine today, so the
powerset steps are the ones that would catch `conformance` being silently coupled
to `memory`, or `unstable-projection` failing to compile with the port gated off —
thirty-two combinations, up from eight (`../_decomposition.md:483-487`). An
artefact recording "green" without recording which optional steps executed is
claiming more than the run proved.

**And the one that would make the whole project's evidence retrospective:**
running the gate before `unstable-projection-gate-and-clause-disposition` or
`documented-extension-surface` merges, and recording it. The run is then evidence
about a tree that no longer exists. The dependency edges in the storymap say this
story is last (`../_storymap.md:114`); the spec must say the run happens after the
final merge, on a clean checkout, and not once during it.

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

The two judgement boxes. **Literal positions**: this story adds no conformance
rule — it runs the ones that exist and records what the run showed. Vacuously
true; if the run reveals a rule that asserts a literal position, that is a defect
reported against the story that wrote it, under the eighth box. **`[FROZEN]`
clauses**: none is touched. The clause dispositions were settled in
`unstable-projection-gate-and-clause-disposition`, which is this story's own
`dependsOn`, and any frozen-clause repair landed as a new accepted atom in slice
1.
