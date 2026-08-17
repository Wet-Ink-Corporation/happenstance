---
item: HS-S0144
stage: spec
created: 2026-08-17T13:16:05.747Z
updated: 2026-08-17T13:16:05.747Z
template_sig: 87bbf1d0
rendered_sig: "3683e651"
---

# Spec — The gate is watched failing on a page broken on purpose, then recovering

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/docs-that-teach/initiative.md` — BR-02; **DoD scenario 2** at `:416-420`, the scenario this story turns green |
| Initiative decomposition | `.bklg/docs-that-teach/_decomposition.md` — the DAG; HS-P0020 first, no inbound edge |
| Project charter | `.bklg/docs-that-teach/checked-documentation-surface/project.md` — **AC-003** (`:208-211`), DR-06, DoD items 2, 6 and 8 |
| This spec | `.bklg/docs-that-teach/checked-documentation-surface/observed-failure-falsification/spec.md` |
| Key briefs | `.bklg/docs-that-teach/checked-documentation-surface/_decomposition.md` — testing brief **AC-003 row** (`:568-584`, the procedure and its non-substitutability) and its Notes (`:676-703`, fixture ownership + the merge-gate command ladder); architecture brief **Note 3** (`:186-189`, the residual degradation to record rather than route around) and **Note 10** (`:351-376`, the limits list this story's findings feed) |
| Signed-off design (**binding**) | `.bklg/docs-that-teach/checked-documentation-surface/_design.md` — `## The doctest` (the fixture this story breaks, verbatim), `## Surfaces` (`gate-narrative-compile-step`, state `fail-broken-fence`), `## Composition` (the failure report's two-line tail), `## Mock` (the dated-instrument discipline this record inherits) |
| Story map / merge order | `.bklg/docs-that-teach/checked-documentation-surface/_storymap.md` — milestone `falsification-and-limits`, story 9 of 10, run "against the assembled gate, not a partial one" (`:165-171`); Activity E, "the one that makes the other four worth anything" (`:34-38`) |
| Grounding | `.bklg/docs-that-teach/checked-documentation-surface/_grounding.md` — no Accepted decision atom governs gate structure, documentation trees or fence compiling |
| Roadmap pointer | `RUNBOOK.md:914-928` — the decorative-step precedent: a probe-gated step printed `skipped` on all three runners while two documents vouched for it |

Authority order for anything this spec does not settle: Accepted decision atoms under `.kb/`
(none govern gate structure; two *playbook* atoms do bind method — see the Context pack) →
`CLAUDE.md` → `standards/rust/81-checks-that-cannot-be-types.md` → `_design.md`.

## One-line PR slice

Break one claim in the fixture page, run the gate, record the failure output verbatim including
which file it actually names, revert, run again, record green — both halves in this project's own
artefacts.

Mechanically: author `_falsification.md` in this story's own folder — a dated, sha-pinned record of three runs of
`cargo xtask ci` (green baseline → red on a one-line edit to `docs/append-conditions.md` → green
after `git checkout --`), carrying each run's output verbatim, the exact edit as a diff hunk, and
the four things the run **measured** rather than predicted. No source file changes; no broken page
is ever committed.

## Executive summary

Eight stories have built a machine. This one is the only story in the project that finds out
whether it works.

Everything before this point is a check asserted green: milestone 1's step compiles the fixture
page's fences, milestone 2's checker reads the tree, milestone 3's resolver reads the
specification, and every one of them has been observed passing. That is precisely the state
`RUNBOOK.md:914-928` describes — a step wired, vouched for by two documents, and printing
`skipped` on all three runners for as long as anyone cared to look. A gate that has only ever
been green is decorative (project DoD item 2), and no unit test in this project can substitute for
the observation, because AC-003 requires `cargo xtask ci` **itself** to fail and a unit test never
invokes the gate (testing brief, `_decomposition.md:581-584`).

The delta this PR lands is therefore evidence, and it is a small artifact with an unusually high
load: **`_falsification.md`**, the dated record. Four things in it are new facts rather than
restatements, and each is a measurement the machine's designers wrote down as a prediction and
nobody has yet checked:

1. **That a false claim fails at all.** The fixture's fence asserts `store.len() == 0`. Flipping
   the expected value to `1` leaves code that *compiles* and a claim that is *false*. If the gate
   catches that, the fences are **run**, not merely compiled — which nothing in the project has
   established. If it does not, the limits list gains an item and `documented-blind-spots-and-their-proofs`
   inherits it.
2. **Which file the failure actually names.** Architecture brief Note 3 predicts the harness
   (`xtask/src/narrative.rs`), not the markdown, with the module resolving the page and the line
   resolving the location. AC-003's wording — "identifies the page and the location" — is satisfied
   or not by what rustdoc actually prints, and the record must show the real output, "not what an
   idealized failure would print" (`_decomposition.md:578-581`).
3. **Which banner it fails under.** Milestone 1 placed the narrative step immediately before
   `the constitution's examples compile` because that step's argv is unfiltered
   (`xtask/src/main.rs:488-492`) and would otherwise re-attribute a broken narrative fence. That
   reasoning is asserted over an array by a unit test; here it is observed live, together with the
   counterfactual that makes it non-trivial.
4. **What the recovered run says on the way past.** Green output is the half everyone skips, and it
   is where the coverage numbers live — the narrative step's page count and the checker's
   `  {n} pages, all consistent`. A reader who is told a number can falsify it; a reader told "no
   problems found" cannot (`.kb/playbooks/verify-the-referent-and-report-coverage.md`).

What this PR is **not**: any claim that the fixture page teaches anything. It establishes that code
inside prose is not stale. Whether prose teaches is HS-P0024's friction log, is not substitutable
by anything here, and the record carries one unhedged sentence saying so (project DoD item 8).

## Context pack

Read this section whole before running anything. Every paragraph is a decision already taken by a
signed-off artifact, a house standard, or an accepted `.kb` atom; the deeper material is linked and
each link says why and when.

**This story is not permitted to be a `#[test]`.** The testing brief settles it: AC-003 is "the
only AC in this project whose proof is a recorded procedure rather than a `#[test]` function"
(`_decomposition.md:568-570`), and it is "not substitutable by a unit test that calls the checker
function directly, because AC-003 as written in `project.md` requires observing `cargo xtask ci`
itself fail" (`:581-584`). So the deliverable is an artifact, not code, and the temptation to
discharge it with an automated harness is a temptation to discharge a different obligation. What
*is* owed in exchange is that the procedure stay re-runnable rather than becoming a claim nobody
can re-verify (`_decomposition.md:684-687`) — which is why the record carries the exact commands
and the exact edit, not a prose summary of them.

**Three runs, not two, because a red run against an unknown baseline proves nothing about the
edit.** The story map's one-liner names break → red → revert → green. A green run *before* the
break is added here deliberately: without it, a red run is consistent with a tree that was already
red for an unrelated reason, and the record would attribute a failure to an edit that did not cause
it. `.kb/playbooks/landing-a-stricter-gate-without-a-red-baseline.md` is the atom on exactly this
class of mistake, one level up — its subject is landing a check against a dirty corpus, and its
transferable half is that a check's evidence is only as good as the baseline it is read against.
The baseline run is also what proves the *starting* tree clean, which AC-005 needs.

**The break is a claim falsification, not an item deletion, and the distinction is the whole
point.** Project AC-002's evidence — "removing a public item that a page calls makes the gate fail"
— is already milestone 1's, and its spec records the promise to rename
`MemoryEventStore::len` temporarily and observe the compile step fail
(`.bklg/docs-that-teach/checked-documentation-surface/pinned-narrative-tree-and-compiling-step/spec.md`,
AC-002 row). Repeating it here would buy nothing. Project AC-003 asks for something strictly
harder: "a page is deliberately edited so **one claim is no longer true of the library**"
(`project.md:208-211`), and the initiative's DoD-2 uses the same words (`initiative.md:416-420`).
The fixture's fence is:

```rust
use happenstance_core::MemoryEventStore;

let store = MemoryEventStore::new();
assert_eq!(store.len(), 0);
```

(`_design.md`, `## The doctest`; the items are real —
`crates/happenstance-core/src/memory.rs:87` and `:171`, re-exported at
`crates/happenstance-core/src/lib.rs:122`.) Changing `0` to `1` produces code that type-checks
perfectly and states something false about the library. That break discriminates between two
worlds nothing in the project has yet distinguished: fences **compiled** and fences **compiled and
run**. Pre-register both dispositions before running it, because a falsification whose outcomes are
interpreted after the fact is not one.

**The fixture is somebody else's artifact and is not this story's to redesign.** `_design.md`
`## The doctest` writes the page out in full and names it "the literal artifact
`pinned-narrative-tree-and-compiling-step` and `observed-failure-falsification` build against".
Milestone 1's spec also flags a residual on it — the sentence describes a *boundary* property while
`ES-40`'s normative text is about *completeness* (`spec/SPECIFICATION.md:4351`; the boundary
property is argued at `:4293` as the joint consequence of ES-38 and ES-40). That residual is
recorded, not fixed here: the citation resolves, so nothing fails, and any correction is an
amendment to `_design.md` by its owner. This story breaks the page's **assertion**, which is
independent of the citation question.

**The edit is transient and nothing broken is ever committed.** The break is applied to the working
tree, observed, and reverted with `git checkout -- docs/append-conditions.md`. The committed diff
must contain no change under `docs/` and none under `crates/`. This is not tidiness: a committed
broken fixture would make the gate red for every later story in the initiative, and the project's
own risk table forbids this project from touching the corpus at all
(`project.md:294`). It also keeps the story's PR boundary honest —
`redkiln verify --grain story` computes touched files from git, so a reverted file is simply not a
changed file.

**The record is a dated instrument and must never be edited to agree with a later tree.** This is
the discipline `_design.md` already applies to its own mock: "`design/mock.html` deliberately still
shows the pre-correction figures, and must not be 'fixed' … it is the dated instrument that
*disproved* those numbers … Editing the mock to agree with the corrected design would delete the
evidence" (`_design.md`, `## Mock`). The same rule binds `_falsification.md`: it carries the
`git rev-parse HEAD` of the tree it was run against and the date, and when a later tree makes its
output stale the answer is a **second dated section from a second run**, never an edit to the
first. The in-house shape for a verbatim machine transcript with the mechanism named beside it is
`experiments/rustc-ice-gat-foreign-trait/README.md:1-22` — a ` ```text ` fence holding real output,
followed by the paragraph that explains what in it is load-bearing. Copy that shape; the record
lives under `.bklg/`, outside the pinned tree, so the narrative tree's fence discipline does not
reach it and a ` ```text ` fence here is not the AC-004 back door it would be under `docs/`.

**The failing run's log is truncated at the narrative step, and that is a property to state rather
than a defect to hide.** `run_steps` prints the banner, spawns each step as its own process and
`bail!`s on the first non-zero status (`xtask/src/main.rs:862-892`, the bail at `:887`). So the red
run proves the narrative step fails and proves **nothing** about every step after it; only the
recovered run carries a whole-gate green. The failure's tail is two lines, both real primitives and
neither invented: `eprintln!("\nxtask failed: {err:#}")` from the child at
`xtask/src/main.rs:709-715`, then the parent's `bail!("{} failed with {status}", step.name)` at
`:887` (`_design.md`, `## Composition`, and its mock finding 2, which corrected an earlier invented
line). Record both, on stderr, alongside the stdout banner at `:864`.

**Banner attribution is measurable here, and the counterfactual is one command.** The narrative
step's argv filters doctests to `narrative::`; the constitution's step at
`xtask/src/main.rs:480-492` runs `cargo test --locked -p xtask --doc` **unfiltered** and therefore
also compiles the narrative fences. Ordering — not filtering — is what keeps a broken narrative
fence attributed to the narrative banner, and milestone 1's spec explains why filtering the
constitution step in compensation is forbidden (its argv also carries the repository README's
doctest, `xtask/src/lib.rs:21`, which matches neither filter and would drop out of the gate). So
while the page is broken, run the constitution step's exact argv directly and observe that it fails
too. That single observation converts an asserted rationale into a measured one and is the live
form of milestone 1's `EC-005` wrong implementation.

**Every number the green run prints is part of the evidence, because a check that does not state
its coverage is indistinguishable from one that sees everything.**
`.kb/playbooks/verify-the-referent-and-report-coverage.md` is the accepted atom, and it is grounded
in this repository's own measurement: a citation parser checked 84 of 338 citations while printing
"no problems found". Its transferable half is one line of output — the coverage figure — because "a
reader who is told a number can falsify it. A reader who is told 'no problems found' cannot." The
recovered run's record therefore captures the narrative step's enumerated page count and the
checker's one-line `  {n} pages, all consistent` (the primitive at
`xtask/src/lint_constitution.rs:192`, per `_design.md`'s verified primitive table), and the record
states what those numbers were **at that sha** so a later reader can see if the corpus grew and the
number did not.

**RS-81-1 is why this story exists at all, and its second half is the one that constrains the
record.** "Prove the check's blind spot in its own tests, then state it in its own documentation"
(`standards/rust/81-checks-that-cannot-be-types.md:10`). RS-81-4 is the same argument for output:
"a zero exit status is evidence of nothing" (`:264`). This story is the project applying both to
itself. The consequence for the record is that a finding which *contradicts* a prediction in the
architecture brief is the most valuable thing the run can produce and must be written up as a
finding rather than smoothed over — `documented-blind-spots-and-their-proofs` (HS-S0145) is
`blocks:` this story precisely so it can consume them (`story.md`, `blocks: [HS-S0145]`).

**The persona-journey slice.** The user is a **contributor running the gate** and a **reviewer
reading its output** — this project ships no runtime surface (`_storymap.md`, preamble). The journey
this story realizes is the one nobody has walked: a contributor edits a claim on a page so it is no
longer true, runs the gate, and finds out. What they must meet is a banner naming the narrative
tree, a location they can act on, and — crucially — no ambiguity about whether the gate was even
looking. The reviewer's half is the recorded log itself: after this story, "the gate checks the
docs" is a sentence backed by a transcript rather than by a wiring diagram.

**What must not move.** No source file is edited in a way that survives the commit — no
`xtask/src/**`, no `crates/**`, no `spec/SPECIFICATION.md`, no `standards/rust/**`. No teaching page
is authored (the corpus is HS-P0021/22/23's, `project.md:294`). No `CHANGELOG.md` entry is owed:
this story adds no conformance rule. CLAUDE.md's five binding constraints are untouched by
construction — nothing here touches a port, an `async fn`, a `Send` bound, `serde` or a feature. And
nothing in the record may assert that the surface proves a page teaches (project DoD item 8;
`_design.md` anti-pattern 9 — no badge, tick or "verified" mark).

## Integration contract

- **Archetype**: `capability` — user-observable end to end, where the user is a contributor running
  the gate and the observation *is* the deliverable. Not `foundation`: nothing here is substrate
  another story consumes at compile time, but HS-S0145 consumes its findings as input.
- **Slice / milestone**: `falsification-and-limits`. Slice-mate implemented in the same context and
  mounted as one surface: **`documented-blind-spots-and-their-proofs`** (HS-S0145), which puts the
  six-item "What this does not verify" list first in the new modules' docs, re-runs the
  `RUSTDOCFLAGS` probe, and walks a ` ```text `-tagged broken fixture through the gate. This story
  is **first** of the two in merge order (`_storymap.md`, "Merge order" 4.9-4.10) because its
  measurements are three of that story's inputs: which file the failure names, whether a fence is
  run or only compiled, and what the green run's coverage numbers are.
- **Mount point**: **`xtask/src/main.rs`** — the `REQUIRED` array at `:105`, exercised **as a whole**
  through `cargo xtask ci`. This story is the one that *reads* that composition root rather than
  adding to it: it adds no `Step`, no `mod`, no dispatch arm and no help line, and its entire claim
  is about what that array does when a page under `docs/` is wrong. The second, artifact-side mount
  is `.bklg/docs-that-teach/checked-documentation-surface/observed-failure-falsification/_falsification.md`,
  which is mounted into the project's evidence chain by being cited from this story's `_ledger.md`
  and from HS-S0145's spec — an evidence artifact nothing cites is the documentation analogue of a
  constructed-but-unmounted component.
- **Wires into**:
  - `xtask/src/main.rs:862-892` — `run_steps`: the banner at `:864`, the per-step spawn, and the
    first-failure `bail!` at `:887`. This is the code whose behaviour the record transcribes.
  - `xtask/src/main.rs:709-715` — the child process's `xtask failed: {err:#}` tail on stderr.
  - `xtask/src/main.rs:480-492` — `the constitution's examples compile`, unfiltered, the
    counterfactual the attribution observation runs directly.
  - `xtask/src/main.rs:72-103` — the `Step` contract and the normative `probe` doc comment at
    `:89-102`: `probe: None` is why no run in this record may contain a `skipped` line for either
    narrative step.
  - `docs/append-conditions.md` — milestone 1's fixture page, the artifact broken and restored.
  - `xtask/src/narrative.rs` — milestone 1's harness, the file rustdoc is predicted to name.
  - `xtask/src/lint_narrative.rs` — milestone 2's checker, whose green one-liner is part of the
    recovered run's record.
  - `crates/happenstance-core/src/memory.rs:87,171` and `crates/happenstance-core/src/lib.rs:122` —
    the real public items the broken assertion is about. Read, never edited.
  - `.redkiln/config.yaml`, `verify:` block — `e2e: "cargo xtask ci"` is the invocation AC-003 names
    and this story runs; `integration_scoped: "cargo xtask ci --fast"` is the per-story bar;
    `require_ledger: true` is why `_ledger.md` must cite `_falsification.md` per criterion.
- **Renders surfaces** (ids from `_design.md` `## Surfaces`): **`gate-narrative-compile-step`** in
  its **`fail-broken-fence`** state — the only story in the project that renders it, and the state
  the whole design was drawn to make legible — and in its `pass` state on the recovered run.
  **`gate-narrative-checker-step`** in its `pass` state, for its coverage line. Not rendered here:
  `fail-removed-item` (milestone 1's, via the temporary `len` rename), `fail-hidden-marker`
  (`hidden-content-resolution`'s), `fail-many` (claimed by nobody in this project — say so rather
  than implying coverage), `narrative-page` / `narrative-scoped-page` / `narrative-tree-index`
  (transiently broken and restored, so unchanged as shipped), `rustdoc-reference-surface`
  (untouched by this project).
- **Public items** (`_design.md` `## Items`): **none.** No item is added, changed or removed in any
  crate, publishable or not. The design's `## Items` block lists `clause_ids` and the checker's four
  constants; every one belongs to an earlier story. This story's addition to the project is an
  artifact, and the honest statement of that is this row saying "none" rather than borrowing another
  story's items.
- **Conformance rule(s)**: **none, and this is not adapter-observable.** Nothing here touches
  `crates/happenstance-testkit/`, a port, a value type or a fixture; the instrument is the gate's own
  output. Stated explicitly because a story that changes a port and names no rule is a port change
  nothing can fail — this changes no port. The house rules it *is* held to are **RS-81-1**
  (`standards/rust/81-checks-that-cannot-be-types.md:10` — prove the blind spot, then state it) and
  **RS-81-4** (`:264` — a zero exit status is evidence of nothing).
- **Clause(s)**: none discharged, none amended, none restated. `spec/SPECIFICATION.md` is read only
  as the referent of the fixture page's `ES-40` citation (`:4351`), and only to record the residual
  milestone 1 flagged. No `[FROZEN]` clause is touched, so no ADR is owed — consistent with
  `_grounding.md` and with the story map's "Stories this map deliberately does not contain: an ADR".
- **Advances DoD scenario**: initiative **DoD-2** — *"@smoke — a deliberately broken page fails the
  gate, by name. … The edit is then reverted and the gate returns to green. Both halves are
  observed; the failing half is the one that matters"* (`initiative.md:416-420`). This story is the
  **only** thing in the initiative that moves DoD-2, and it moves it from unobserved to green. It
  also hardens **DoD-1** (already green after milestone 1) by proving the green was not vacuous, and
  it supplies three inputs to **DoD-13**'s honesty half via HS-S0145.

**Delivered mounted, not as a component.** The merge bar is a committed `_falsification.md`
containing three real transcripts from three real runs of the assembled gate on a named sha, with a
clean tree at HEAD — not a described procedure, not a plan to run it, and not a unit test that
exercises the checker in isolation.

## PR boundary

```
.bklg/docs-that-teach/checked-documentation-surface/observed-failure-falsification/**
docs/append-conditions.md
```

The second entry is listed because the story genuinely edits that file, and the boundary should
say so rather than relying on the edit being invisible. It must appear in the final diff as
**unchanged**: `redkiln verify --grain story` computes touched files from git, so a reverted file is
not a changed file — and if it *is* changed, the boundary passing is the wrong outcome, which is
why AC-005 asserts the cleanliness directly instead of leaning on the boundary check.

**In this PR**

- `.../observed-failure-falsification/_falsification.md` — the dated record. Sections: the sha and
  date; the pre-registered predictions and the disposition of each outcome; the exact edit as a
  unified diff hunk; three verbatim transcripts (baseline green, red, recovered green) in ` ```text `
  fences in the shape of `experiments/rustc-ice-gat-foreign-trait/README.md:1-22`; the four
  measurements with what each turned out to be; the attribution counterfactual; the findings routed
  to HS-S0145; and one unhedged sentence saying the observation is silent about whether the page
  teaches.
- `.../observed-failure-falsification/_ledger.md` and the implementation record — one row per
  `AC-###`, each citing a section of `_falsification.md` rather than restating it
  (`.redkiln/config.yaml`, `require_ledger: true`).
- `docs/append-conditions.md` — edited and reverted within the run. Net zero.

**Explicitly not in this PR**

- **Any source change.** No `xtask/src/**`, no `crates/**`, no `Cargo.toml`, no `Cargo.lock`. If the
  run reveals a defect in the machine, the finding is recorded and routed — to HS-S0145 for a
  limits-list item, to the owning story's review, or to the `support` initiative for an incidental
  bug (`.redkiln/config.yaml`, and `project.md`'s out-of-scope table) — and is **not** fixed here.
  A story that repairs the thing it was written to test cannot report on it.
- **A `#[test]` that automates the falsification.** Forbidden by the testing brief
  (`_decomposition.md:568-570,581-584`), because it would discharge a different obligation than the
  one AC-003 states.
- **The six-item limits list, the `RUSTDOCFLAGS` re-probe, and the ` ```text `-fence blind-spot
  walk** — all `documented-blind-spots-and-their-proofs`' (HS-S0145). This story produces inputs to
  them and must not pre-empt their contents.
- **The `fail-removed-item` observation** (renaming `MemoryEventStore::len`) — milestone 1's, already
  promised in its AC-002 row.
- **The `fail-hidden-marker` observation** — `hidden-content-resolution`'s.
- **Any correction to `_design.md`'s fixture**, including the `ES-40` citation residual. Recorded,
  not edited; amendment belongs to the design record's owner.
- **Any teaching page, any `.kb/` atom, any ADR.** Knowledge harvesting from this project happens at
  initiative closeout, not in a story.

**Merge DoD**: `_falsification.md` exists at the story's own path, carries three verbatim
transcripts from `cargo xtask ci` on a named sha, shows the red run failing under
`=== the narrative tree's examples compile ===` with its two-line stderr tail, shows the recovered
run green with the narrative step's coverage numbers, `git status --porcelain` is empty and
`git diff --stat HEAD~1 -- docs/ crates/ xtask/` is empty, and `cargo xtask ci --fast` is green on
the merged tree.

## Behavior and interfaces

Each row opens with the `AC-###` it will be enumerated as. "Evidence path" is the file the
behaviour is constrained by, copied from, or observable in.

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **(AC-001)** A page broken so one claim is false makes the assembled gate fail, and the failure is transcribed verbatim | Procedure, in order, each command's full output captured: (1) `git rev-parse HEAD` and `git status --porcelain` — record the sha, prove the tree clean; (2) `cargo xtask ci` — **green baseline**, without which the red run attributes nothing; (3) apply the one-line edit to `docs/append-conditions.md`, `assert_eq!(store.len(), 0)` → `assert_eq!(store.len(), 1)`, recorded as a unified diff hunk so the run is reproducible byte-for-byte; (4) `cargo xtask ci` — **must fail**. Transcribe stdout and stderr verbatim, including the banner (`xtask/src/main.rs:864`) and the two-line tail (`:709-715` then `:887`). The log is truncated at the failing step by design — `run_steps` bails on the first non-zero status — and the record says so, because a red `ci` log proves nothing about the steps after it | `project.md:208-211` (AC-003); `initiative.md:416-420` (DoD-2); `_decomposition.md:568-584`; `xtask/src/main.rs:862-892,709-715` |
| **(AC-002)** The failure is attributed to the narrative banner, and the counterfactual shows why ordering rather than filtering is what does it | With the page still broken: (a) confirm the failing step's banner is `=== the narrative tree's examples compile ===` and that `=== the constitution's examples compile ===` never printed; (b) run the constitution step's exact argv directly — `cargo test --locked -p xtask --doc` with `RUSTDOCFLAGS=-D warnings` — and observe it **also** fails, because it is unfiltered (`xtask/src/main.rs:480-492`). (b) is what makes (a) non-trivial: it demonstrates that step ordering, not the `narrative::` filter, is the mechanism, which is milestone 1's `EC-005` observed live instead of asserted over an array. Do **not** "fix" the overlap by filtering the constitution step — its argv also carries the repository README's doctest (`xtask/src/lib.rs:21`), which matches neither filter | `xtask/src/main.rs:480-492,862-892`; `pinned-narrative-tree-and-compiling-step/spec.md`, EC-005 and its AC-004 row |
| **(AC-003)** What the failure actually identifies is recorded as measured, and every divergence from the predicted shape is a finding routed to HS-S0145 | Record, from the real output: the **file** rustdoc names; the **doctest name** including the module; the **line number**; and whether that line, counted in `docs/append-conditions.md`, is the broken assertion. Architecture brief Note 3 predicts file = `xtask/src/narrative.rs` (harness, not markdown), module = page, line = relative to the page. Also record whether the failure is a **compile** error or an **assertion panic** — an assertion panic proves the fences are *run*, not merely compiled, which nothing in this project has yet established, and a pass on this break would prove the opposite. Pre-register both dispositions **before** step (4) of AC-001, so the interpretation is not chosen after the fact. Contradictions are the most valuable output here and are written up, not smoothed | `_decomposition.md:186-189` (Note 3), `:351-376` (Note 10); `standards/rust/81-checks-that-cannot-be-types.md:10` (RS-81-1) |
| **(AC-004)** Reverting restores green, and the recovered run's coverage numbers are part of the record | `git checkout -- docs/append-conditions.md`, then `cargo xtask ci` — **must be green**, and this is the only run in the record that says anything about the whole gate. Capture the narrative compile step's enumerated page count and the checker's `  {n} pages, all consistent` line (the primitive at `xtask/src/lint_constitution.rs:192`), and state what those numbers were at the recorded sha, so a later reader can tell whether the corpus grew while the count did not. Also confirm neither narrative step printed a `skipped` line — `probe: None` makes that structurally impossible (`xtask/src/main.rs:89-102`), and its appearance would be the `RUNBOOK.md:914-928` failure reproduced | `.kb/playbooks/verify-the-referent-and-report-coverage.md`; `_design.md` `## Transience policy` (success summary = one line); `xtask/src/main.rs:72-103` |
| **(AC-005)** The falsification leaves no residue: no broken page is committed and no source file changes | After the recovered run: `git status --porcelain` is empty, and the story's commit diff contains no change under `docs/`, `crates/`, `xtask/`, `spec/` or `standards/`. The only committed paths are under this story's own backlog folder. A committed broken fixture would turn the gate red for every later story in the initiative and would put this project inside the corpus it is forbidden to write (`project.md:294`) | `project.md:294`; `.redkiln/config.yaml`, `verify:` block (the story-grain boundary check reads git) |
| **(AC-006)** The record is a dated, sha-pinned instrument, re-runnable from what it contains, and is never edited to agree with a later tree | `_falsification.md` carries: the date, the `git rev-parse HEAD` sha (which must still resolve — `git cat-file -e <sha>`), the toolchain in use, the exact command sequence, and the edit as a diff hunk. A later tree that makes the transcripts stale gets a **second dated section from a second run**; the first is never rewritten. This is the discipline `_design.md` already applies to its own mock, in its own words: the instrument that disproved a number is only legible against the frame that produced it, and editing it "would delete the evidence". The transcript shape — a ` ```text ` fence of real output followed by the paragraph naming what in it is load-bearing — is `experiments/rustc-ice-gat-foreign-trait/README.md:1-22` | `_design.md` `## Mock` (the dated-instrument paragraph); `experiments/rustc-ice-gat-foreign-trait/README.md:1-22` |
| **(AC-007)** The record claims exactly what was observed and no more — in particular, nothing about teaching | One unhedged sentence stating that the observation establishes that code inside prose still compiles and, if AC-003 measured an assertion panic, still holds — and is **silent** about whether the page teaches; HS-P0024's friction log is not substitutable. No badge, tick or "verified" mark anywhere in the artifact (`_design.md` anti-pattern 9). The record also states its own three limits explicitly: the red `ci` log is truncated at the failing step and proves nothing about later steps; one fixture page is not the corpus; and a single observation on one toolchain and one runner is not a guarantee about others | `project.md:260` (DoD item 8); `initiative.md:429-436` (DoD 5/6, the instrument that *is* about comprehension); `_design.md` `## What a user meets first`, `## Anti-patterns` 9 |

**One residual carried forward, not resolved here.** Milestone 1 flagged that the fixture sentence
describes a boundary property while `ES-40`'s normative text is about completeness
(`spec/SPECIFICATION.md:4351`, with the boundary property argued at `:4293`). This story breaks the
fence's **assertion**, which is independent of that sentence, so the falsification is unaffected
either way. Record the observation in `_falsification.md` and leave the amendment to `_design.md`'s
owner — a silent edit here would change the artifact two other stories are pinned to.

## Data and migrations

**N/A — no data store, no schema, no persisted state, no backfill, and no source change at all.**
This story's only committed artifacts are markdown files under
`.bklg/docs-that-teach/checked-documentation-surface/observed-failure-falsification/`. The
deployment brief already establishes that "deployment" for this project means the pinned tree and
its render existing on every clean checkout with no manual step (`_decomposition.md:729-742`); this
story neither adds to that nor changes it.

Three migration-shaped facts that are not migrations but would be missed if left unstated:

- **The working-tree edit is the only mutation, and it is transient by contract.** There is no
  fixture directory to provision, no snapshot to bless and no generated artifact to check in. The
  "migration" is `git checkout --`, and AC-005 is the assertion that it ran.
- **Rollback is `git revert` of one commit and is safe in isolation** — unusually for this project.
  Milestone 1's spec records an ordering hazard for *its* rollback (reverting the pinned tree after
  the corpus lands leaves real pages unchecked); no such hazard exists here, because reverting this
  story deletes only evidence. What it costs is that DoD-2 goes back to unobserved and HS-S0145
  loses three of its inputs, so the revert is cheap mechanically and expensive in the record.
- **The record's own durability is the thing with a lifecycle.** Its transcripts go stale as the
  tree moves — new pages, a renamed harness, a changed step name. Staleness is handled by appending
  a dated re-run, never by editing (AC-006). The one thing that would make the record *wrong* rather
  than merely dated is an edit to `docs/append-conditions.md` that removes the assertion the record
  says it broke; that is a review obligation on whoever edits the fixture, and it is deliberately
  **not** a gate rule, because a check in `xtask/src/` that reads `.bklg/` would couple the gate to
  the backlog and is unprecedented in this repository.

## Acceptance criteria

The personas are the two this project ships for — a **contributor running the gate** and a
**reviewer reading its output** (`_storymap.md`, preamble). They are the machine-facing half of the
initiative's audience, carried from
`.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md`; the journey being realized
is the one nobody in this initiative has walked end to end, *a contributor breaks a claim and finds
out*. Every criterion below is that journey crossing the whole stack — a page under `docs/`, the
harness in `xtask/src/`, the `REQUIRED` array, and the log the contributor actually reads — not a
capability asserted about a function.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **GIVEN** a contributor who has just edited a narrative page so one of its claims is no longer true of the library, and a tree whose gate was observed **green immediately beforehand** at a recorded sha, **WHEN** they run `cargo xtask ci`, **THEN** the gate exits non-zero, and all three runs — green baseline, red, recovered green — are transcribed **verbatim** into `_falsification.md` together with the exact edit as a unified diff hunk, so the failure is attributable to that edit and to nothing else in the tree. | Recorded procedure, not a `#[test]` — mandated by the testing brief (`_decomposition.md:568-570,581-584`). Evidence: `.bklg/docs-that-teach/checked-documentation-surface/observed-failure-falsification/_falsification.md` §§ *Baseline*, *The edit*, *Run 2 — red*. Commands, in order: `git rev-parse HEAD`, `git status --porcelain`, `cargo xtask ci`, apply hunk, `cargo xtask ci`. Reviewer check: run 2's transcript is non-zero-exit and run 1's is green, at the same sha. |
| AC-002 | **GIVEN** the same broken page and a contributor who must work out **which half of the machine** is complaining, **WHEN** they read the failing run, **THEN** the failing banner is `=== the narrative tree's examples compile ===` and `=== the constitution's examples compile ===` never printed; **AND WHEN** the constitution step's own argv is run directly while the page is still broken, **THEN** it fails too — recorded as the counterfactual that shows **step ordering**, not the `narrative::` filter, is what preserves attribution. | Gate-integration, observed live. Evidence: `_falsification.md` § *Attribution and its counterfactual*, carrying (a) the banner sequence from run 2's stdout and (b) the direct invocation `RUSTDOCFLAGS=-D warnings cargo test --locked -p xtask --doc` with its output. Mechanism at `xtask/src/main.rs:480-492`; this is milestone 1's `EC-005` (`pinned-narrative-tree-and-compiling-step/spec.md`) observed instead of asserted over an array. |
| AC-003 | **GIVEN** a reviewer who must decide whether the recorded failure is **actionable** — can they open a file and fix it? — **WHEN** they read the record, **THEN** it states **as measured**: the file rustdoc names, the doctest name including its module, the line number, whether that line locates the broken assertion inside `docs/append-conditions.md`, and whether the failure was a **compile error** or an **assertion panic**; **AND** both dispositions were pre-registered before the red run, so every divergence from the architecture brief's Note 3 prediction is written up as a finding routed to HS-S0145 rather than smoothed over. | Recorded measurement + pre-registration. Evidence: `_falsification.md` § *Predictions, pre-registered* (written and committed to the record **before** run 2) and § *What the failure actually identified*. Cross-check: `cargo test --locked -p xtask --doc -- --list` for the doctest's registered name. Held to RS-81-1 (`standards/rust/81-checks-that-cannot-be-types.md:10`) — a contradiction is the highest-value output, not a defect in the record. |
| AC-004 | **GIVEN** a contributor who has reverted the break and needs to know the tree is genuinely restored — and a reviewer who needs to know the green means something — **WHEN** `cargo xtask ci` is run again, **THEN** the **whole** gate is green, and the record carries the narrative compile step's enumerated page count and the checker's `  {n} pages, all consistent` line with the numbers **as they stood at the recorded sha**, and confirms neither narrative step printed a `skipped:` line. | Gate-integration. Evidence: `_falsification.md` § *Run 3 — recovered*, quoting both coverage figures verbatim. Commands: `git checkout -- docs/append-conditions.md`, `cargo xtask ci`. Rules: the coverage obligation is `.kb/playbooks/verify-the-referent-and-report-coverage.md`; a `skipped:` line is structurally impossible under `probe: None` (`xtask/src/main.rs:89-102`) and its appearance reproduces `RUNBOOK.md:914-928`, so it is a halt (EC-006), not a note. |
| AC-005 | **GIVEN** every later story in this initiative, each of which runs this same gate, **WHEN** this story's PR is merged, **THEN** it carries **no broken page and no source change**: `git status --porcelain` is empty after the recovered run, and the commit diff contains no path under `docs/`, `crates/`, `xtask/`, `spec/` or `standards/` — the only committed paths are inside this story's own backlog folder. | Git-observable, asserted directly rather than inferred from the boundary check. Evidence: `_falsification.md` § *Residue*, plus `git status --porcelain` (empty) and `git diff --stat HEAD~1 -- docs/ crates/ xtask/ spec/ standards/` (empty) on the merged tree. Rationale: `project.md:294` forbids this project from writing the corpus; a committed broken fixture turns the gate red for everything downstream. |
| AC-006 | **GIVEN** a contributor reading this record a year later against a tree that has moved, **WHEN** they try to re-run it, **THEN** `_falsification.md` gives them everything they need without asking anyone: the date, a `git rev-parse HEAD` sha that still resolves, the toolchain, the full command sequence and the edit as a diff hunk; **AND** a transcript that a later tree has made stale is answered by a **second dated section from a second run**, never by editing the first. | Re-runnability + dated-instrument discipline. Evidence: `_falsification.md` § *Provenance* — date, sha, `rustc -Vv`, `cargo -V`, platform. Checks: `git cat-file -e <sha>` resolves; `git apply --check` accepts the recorded hunk against that sha. Shape copied from `experiments/rustc-ice-gat-foreign-trait/README.md:1-22`; discipline from `_design.md` `## Mock` ("editing the mock to agree with the corrected design would delete the evidence"). |
| AC-007 | **GIVEN** the initiative's top-ranked risk — a green gate read as evidence that the documentation **teaches** — **WHEN** anyone reads `_falsification.md` end to end, **THEN** it carries one unhedged sentence saying the observation is silent about whether the page teaches and names HS-P0024's friction log as the non-substitutable instrument; **AND** it states its own three limits (the red log is truncated at the failing step; one fixture page is not the corpus; one toolchain on one runner is not a guarantee); **AND** no badge, tick, shield or "verified" mark appears anywhere in it. | Document review against a named bar. Evidence: `_falsification.md` § *What this does not establish*. Bars: `project.md` DoD item 8; `initiative.md:429-436` (DoD 5/6, the instrument that *is* about comprehension); `_design.md` `## Anti-patterns` item 9. Mechanical half: `rg -n "verified|✅|badge|shield" _falsification.md` returns nothing outside this criterion's own quotation. |
| AC-008 | **GIVEN** a contributor meeting the gate's failure for the first time, who should meet a **composed report** — a banner naming the step, a body, and a tail that says which step failed — rather than a bare non-zero exit, **WHEN** the red and recovered runs are recorded, **THEN** the record checks the real output element by element against `_design.md` `## Composition` and `## Hierarchy`: banner first on stdout (`main.rs:864`), the report body, the two-line stderr tail (`xtask failed: {err:#}` at `:709-715`, then `{step} failed with {status}` at `:887`), the recovered run's success summary as exactly one line, and problem lines present **only** in a failure state; **AND** every element the design predicts that the real output does not produce — in particular anything rustdoc's own doctest report owns rather than this repository — is recorded as a **named divergence** routed to HS-S0145, never quietly normalised. | Composition fidelity, measured on the transcripts AC-001 and AC-004 captured. Evidence: `_falsification.md` § *Composition, checked against the design*, one row per predicted element with `observed` / `diverged` and the quoted line. This story is the **only** one that renders `gate-narrative-compile-step` in `fail-broken-fence`, so this is the one place the design's composition claims meet real bytes instead of a unit test over a string. A divergence recorded is a pass; a divergence unrecorded is the failure. |
| AC-009 | **GIVEN** the same contributor reading that failure in an **80-column CI log**, where a location pushed off the first visual row is a location they will not act on, **WHEN** the transcripts are measured, **THEN** the record reports: each line's width against the 80-column budget; the location prefix against **≤ 48 characters inclusive of the 16-character `xtask\src\../../` doctest prefix** (i.e. ≤ 32 characters repo-relative); the green run's narrative output as exactly one summary line; and confirms nothing was truncated or elided — no `… and N more` anywhere in the gate's output, and the transcripts themselves complete rather than abridged. | Density fidelity, measured with real numbers from `_design.md` `## Density budget` (the 48/32 split is the design gate's disposition of mock finding 3; the one-line success summary is the primitive at `xtask/src/lint_constitution.rs:192`). Evidence: `_falsification.md` § *Density, measured* — a small table of measured widths beside the budget, and the statement that the fixture page's own path fits. A budget the real path **exceeds** is recorded as a finding, not silently rounded. |

Every row above serves project **AC-003** (`project.md:208-211`), which is the whole of this story's
`traces_to`; AC-001, AC-003 and AC-004 are the three halves that AC-003's own sentence names
(fails · identifies the page and the location · reverts to green), AC-002 and AC-008/AC-009 are what
make "identifies" checkable rather than a matter of opinion, and AC-005/AC-006/AC-007 are what stop
the record decaying into a claim.

## Interaction quality

This story does not *author* a surface; it is the only story in the project that **observes two of
them live** (`_design.md` `## Surfaces`: `gate-narrative-compile-step` in `fail-broken-fence` and
`pass`, `gate-narrative-checker-step` in `pass`). That inverts the usual obligation without weakening
it: every composition claim the signed-off design makes about the gate's output is, everywhere else
in this project, asserted over a string in a unit test — here it meets real bytes. An unstyled
render passes every structural assertion; the invariants below are what fail it.

Each invariant is carried by an **`AC-###` row in the table above**, never by a bullet here.

**State invariants**

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **In place, not a context jump** — the contributor stays in their own working tree and their own terminal; there is no second render, no tool to install, no separate checkout, and one file changes | AC-001, AC-005 | The recorded command sequence contains only `git` and `cargo xtask ci`; the diff hunk touches one file |
| **Non-occlusion** — the red run's log is truncated at the failing step by `run_steps`' `bail!` (`main.rs:887`), so it says nothing about later steps, and the record **states that** instead of letting a red log imply whole-gate coverage | AC-001, AC-008 | `_falsification.md` § *Run 2 — red* carries the truncation statement adjacent to the transcript |
| **Preserved position** (terminal analogue of focus/scroll) — the location is first on its visual row, so soft-wrap at 80 columns can never push it out of sight | AC-003, AC-009 | Measured line widths and the quoted first row of the failure |
| **Reversibility** — one `git checkout --` returns the tree, and the gate, to exactly the state the baseline run recorded | AC-004, AC-005 | Run 3 green at the same sha; `git status --porcelain` empty |
| **Reachability without prerequisites** (terminal analogue of keyboard reachability) — every step in the record runs on a clean checkout with no tool to install: `probe: None`, so no `skipped:` line can appear | AC-004 | Absence of `skipped:` for either narrative step in all three transcripts; contract at `xtask/src/main.rs:89-102` |

**Composition invariants**, taken from the signed-off `_design.md`

| Invariant | Design source | Carried by |
| --- | --- | --- |
| **Presentation exists at all** — the failure is a composed report (banner → body → two-line tail), not a bare non-zero exit | `## Composition`; `## States`, Error row ("never a bare non-zero exit") | AC-008 |
| **Composition and placement** — two steps, two banners, so the banner alone tells a reader which half failed; the narrative step ordered before the constitution's | `## Composition` rule 1; `## Shape decision`, "Step count" | AC-002, AC-008 |
| **Transience** — the step banner is persistent chrome; problem lines are *opened on demand, by failing*; the success summary is persistent chrome at **one line**; the probe skip line is **never present** | `## Transience policy` | AC-004, AC-008 |
| **Density budget, with its real numbers** — 80-column line width; location prefix ≤ 48 characters **including** the 16-character doctest prefix (≤ 32 repo-relative); success output 1 line; problem list unbounded and never truncated | `## Density budget`, terminal table; mock finding 3's disposition | AC-009 |
| **Hierarchy** — `{path}:{line}` primary by being first on the line; message secondary; banner, indent and count recessive | `## Hierarchy`, `gate-narrative-checker-step` | AC-003, AC-009 |
| **Anti-pattern 7** — a gate failure whose first visual row does not begin with `path:line` | `## Anti-patterns` 7 | AC-003, AC-009 |
| **Anti-pattern 8** — a truncated problem list, any `… and N more` | `## Anti-patterns` 8 | AC-009 |
| **Anti-pattern 9** — a badge, tick, shield or "verified" mark asserting the documentation is checked | `## Anti-patterns` 9 | AC-007 |

**Anti-patterns this story cannot render, and says so rather than implying coverage**: 1–6 and 10–11
belong to the markdown surfaces and to the checker's failure states, which are
`hidden-content-resolution`'s and `fence-discipline-and-allowance-list`'s. The `fail-many`,
`fail-one`, `fail-empty-tree`, `fail-hidden-marker` and `fail-long-path` states are **not** rendered
here (`_design.md` `## Surfaces`), and `_falsification.md` names them as unrendered rather than
leaving a reader to assume the record covered them.

**One honest asymmetry, pre-registered.** Anti-pattern 7 is a rule this repository can enforce on
*its own* output. The compile step's body is **rustdoc's** doctest report, which this project does
not control, and mock finding 3 already observed its name shape
(`xtask\src\../../<page> - narrative::<mod> (line N)`). If the real first visual row does not begin
with a `path:line` this repository chose, that is a **measured limit of the mechanism**, recorded
under AC-008 and routed to HS-S0145 — it is not a defect to fix here and not a rule to quietly drop.
That is architecture brief Note 3's "residual degradation to record rather than route around",
applied to the design's own anti-pattern list.

## Error conditions

| id | Condition | Required behaviour |
| --- | --- | --- |
| EC-001 | **The baseline run (run 1) is red.** | **Halt.** Do not apply the break. A red run against an unknown baseline attributes a failure to an edit that may not have caused it, which is the class `.kb/playbooks/landing-a-stricter-gate-without-a-red-baseline.md` is about. Record the baseline failure, route it to the owning story (or to the `support` initiative if incidental), and re-run the whole procedure once the tree is green. AC-001 is not satisfiable without run 1. |
| EC-002 | **The break does not fail the gate** — the false assertion compiles and the step passes. | **Do not redefine the break.** This is a *measurement*, and the most valuable one the run can produce: it means the fences are compiled but not **run**, so no false-but-compiling claim can ever be caught. Record it under AC-003 with both pre-registered dispositions visible, route it to HS-S0145 as a limits-list item **and** to `pinned-narrative-tree-and-compiling-step`'s review as a machine defect. Project AC-003 is then **not** met and the story does not close green — substituting a weaker break (deleting an item) would discharge project AC-002 while quietly leaving AC-003 unproven. |
| EC-003 | **An earlier `REQUIRED` step fails on run 2**, so the narrative step never executes. | The red run proves nothing about the narrative step — `run_steps` bails on the first non-zero status (`main.rs:887`). Treat as EC-001: the transcript is not evidence for AC-001. Re-establish a clean baseline and re-run. Never present such a transcript as the red run. |
| EC-004 | **`git checkout --` does not restore green** (run 3 is red). | **Halt, and do not commit.** AC-005 fails by definition. Diagnose before recording: a second stray edit, a `Cargo.lock` touched by a run, or a genuine flake. If it is a flake, that is itself a finding about the gate's determinism and belongs in the record with the evidence, not smoothed away by re-running until green. |
| EC-005 | **The failure is attributed to `=== the constitution's examples compile ===`** rather than the narrative banner. | Step ordering in `REQUIRED` is wrong — this is milestone 1's `EC-005` realized. Record the observed banner sequence verbatim under AC-002, route to `pinned-narrative-tree-and-compiling-step`. **Do not** fix it here, and specifically do not "fix" it by filtering the constitution step: its argv also carries the repository README's doctest (`xtask/src/lib.rs:21`), which matches neither filter and would drop out of the gate entirely. |
| EC-006 | **A `skipped:` line appears for either narrative step** in any of the three runs. | `RUNBOOK.md:914-928` reproduced inside the very project written to prevent it. **Halt.** The step is probe-gated, which DR-03 and `probe: None` forbid. Record verbatim, route to the owning story. A green run containing a `skipped:` line must never be recorded as satisfying AC-004. |
| EC-007 | **The recorded sha does not resolve** for a later reader — a rebase, a squash, or a shallow clone. | Verify at authoring time with `git cat-file -e <sha>` and record the branch it was on. If the sha is later unreachable, the answer is a **second dated run** (AC-006), never a retro-edited sha: a sha silently swapped for a newer one makes every transcript beneath it a claim about a tree nobody can inspect. |
| EC-008 | **The broken page reaches a commit.** | The most damaging outcome available to this story: the gate goes red for every later story in the initiative and this project lands inside the corpus it is forbidden to write (`project.md:294`). AC-005 asserts the negative directly for this reason. Detection: `git diff --stat HEAD~1 -- docs/` non-empty. Remedy: amend or revert before merge; never "fix forward" by editing the fixture. |
| EC-009 | **A defect is found in the machine and someone fixes it in this PR.** | Forbidden. A story that repairs the thing it was written to test cannot report on it, and its transcripts would describe a tree that never existed on any branch. Record, route (HS-S0145 / owning story / `support`), and leave the tree alone — see the PR boundary's "Explicitly not in this PR". |

## Non-functional

| id | Requirement | Why, and how it is judged |
| --- | --- | --- |
| NF-001 | **Re-runnable by a stranger from the record alone.** | The record's whole value is that it can be re-executed rather than believed (`_decomposition.md:684-687`). Judged by: the command sequence is complete and copy-pasteable, the edit is a unified diff hunk `git apply --check` accepts at the recorded sha, and no step reads "then observe that…" without saying what was typed. |
| NF-002 | **Three full `cargo xtask ci` runs, not `--fast` and not the narrow doctest command.** | Project AC-003 names `cargo xtask ci` itself; `--fast` drops `OPTIONAL` steps and would leave a reader unable to tell whether a later step also failed. `cargo test --locked -p xtask --doc` is legitimate *while iterating* and for AC-002's counterfactual, and is explicitly not the evidence for AC-001. The record states the wall-clock cost of each run so the next person can plan for it. |
| NF-003 | **Environment is recorded, and the observation is scoped to it.** | `rustc -Vv`, `cargo -V`, OS and shell go in § *Provenance*. This matters more than usual here: mock finding 3 observed a Windows-shaped doctest name (`xtask\src\../../`), so a transcript that does not say which platform produced it invites a reader on another platform to treat a path separator as a defect. One runner is not a guarantee about others — AC-007's third limit. |
| NF-004 | **Zero cost added to the gate and zero new dependency.** | This story adds no `Step`, no `mod`, no crate. `cargo xtask ci` after this PR must take the same time it took before it, because nothing was added to it — the only durable artifact is markdown under `.bklg/`. |
| NF-005 | **No coupling from the gate to the backlog.** | Nothing in `xtask/src/` may be taught to read `.bklg/`. The record's freshness is a human review obligation on whoever next edits `docs/append-conditions.md`, deliberately not a gate rule — a check in the gate that reads the backlog would be unprecedented in this repository and would make the backlog load-bearing for a clean checkout. |
| NF-006 | **Transcripts are complete, never abridged.** | The design forbids the gate truncating its own output (anti-pattern 8); a record that elides the boring middle of a transcript commits the same defect one level up, and elides exactly the region where an unexpected `skipped:` line would sit. If a transcript is genuinely enormous, the answer is a full transcript with the load-bearing lines called out beneath it — the shape at `experiments/rustc-ice-gat-foreign-trait/README.md:1-22`. |
| NF-007 | **Findings are written to be consumed, not admired.** | HS-S0145 reads three measurements out of this record (which file the failure names; run-vs-compile; the green run's coverage numbers). Judged by: each is a single stated fact with its evidence line quoted, findable under its own heading — not a conclusion a reader has to reconstruct from a transcript. A vague finding here becomes an invented one there. |

## Implementation notes (non-prescriptive)

Shape, not prescription — the ACs are the contract.

- **Write the predictions section first and commit it before run 2.** Pre-registration is what makes
  AC-003 a falsification rather than a narrative. Two lines is enough per prediction: what the
  architecture brief expects, and what each outcome would mean.
- **Order matters more than tooling.** `git rev-parse HEAD` → `git status --porcelain` → run 1 →
  apply hunk → run 2 → the AC-002 counterfactual (page still broken) → `git checkout --` → run 3 →
  `git status --porcelain`. The counterfactual has to happen while the tree is still broken; doing it
  after the revert measures nothing.
- **Capture stdout and stderr and say which is which.** The banner is stdout (`main.rs:864`) and both
  tail lines are stderr (`:712`, `:887`); a combined capture is fine and is what a contributor
  actually sees, but the record should label the stream for the four lines AC-008 checks. On
  PowerShell, `2>&1` before a redirect is the difference between a transcript with a tail and one
  without.
- **Do not `git stash` the break.** `git checkout -- docs/append-conditions.md` is the documented
  revert and is what AC-005 asserts ran; a stash leaves state behind that `git status --porcelain`
  will not show.
- **Copy the transcript shape rather than inventing one**: a ` ```text ` fence holding real output,
  then a paragraph naming what in it is load-bearing
  (`experiments/rustc-ice-gat-foreign-trait/README.md:1-22`). The record lives under `.bklg/`, outside
  the pinned tree, so a ` ```text ` fence here is not the AC-004 back door it would be under `docs/`.
- **Measure the density numbers off the captured text, not by eye.** Line widths and the location
  prefix are countable; the design's numbers are specific for the same reason.
- **If a finding contradicts a brief, quote both.** The prediction, then the observation, then one
  sentence on which is now true. Smoothing a contradiction is the one editorial move this story
  cannot make and still be worth running.

## Tests and CI (merge gate)

The ladder is the testing brief's own, narrowest to widest (`_decomposition.md:676-703`), with the
tier vocabulary it defines: **static**, **compile**, **gate-integration**, **end-to-end/fixture**.

| Tier | Command / path | Proves |
| --- | --- | --- |
| end-to-end/fixture | `cargo xtask ci` × 3 (green → red → green), transcribed into `.bklg/docs-that-teach/checked-documentation-surface/observed-failure-falsification/_falsification.md` | AC-001, AC-004, AC-008, AC-009 — the whole of project AC-003. The **only** admissible evidence for it: a unit test never invokes the gate (`_decomposition.md:581-584`) |
| compile | `RUSTDOCFLAGS=-D warnings cargo test --locked -p xtask --doc`, run directly while the page is broken | AC-002's counterfactual — the constitution step's unfiltered argv also fails, so **ordering** is what preserves attribution (`xtask/src/main.rs:480-492`) |
| compile | `cargo test --locked -p xtask --doc -- --list` | AC-003 — the doctest's registered name, module and line, read from the harness rather than inferred |
| static | `cargo test -p xtask` | Regression floor: this story changes no source, so the `xtask` unit suite must be **identical** to its state on `main`. A change here means the PR boundary leaked |
| static | `cargo xtask lint-constitution` | Note 8's obligation — the pre-existing checker is not regressed by anything in this slice |
| gate-integration | `cargo xtask affected --base main` (`.redkiln/config.yaml`, `affected_gate`) | The story-grain bar redkiln actually runs. A backlog-only diff maps to no package, which is precisely why the file-reading lints and `spec-trace` run unconditionally in that command |
| gate-integration | `cargo xtask ci --fast` (`.redkiln/config.yaml`, `integration_scoped`) | The merge bar for this non-terminal project (`project.md` DoD item 6) — green on the merged tree, after the revert |
| git-observable | `git status --porcelain` (empty) · `git diff --stat HEAD~1 -- docs/ crates/ xtask/ spec/ standards/` (empty) | AC-005 — no residue, no broken page committed, no source change |
| git-observable | `git cat-file -e <recorded sha>` · `git apply --check <recorded hunk>` | AC-006 — the record is re-runnable rather than merely dated |
| document review | `_falsification.md` read end to end against `project.md` DoD item 8 and `_design.md` `## Anti-patterns` 9 | AC-007 — the record claims what was observed and no more |
| ledger | `redkiln verify --grain story` reading `_ledger.md` (`.redkiln/config.yaml`, `require_ledger: true`) | Every AC-### has a row, satisfied, citing a section of `_falsification.md` rather than restating it |

**Not in the ladder, deliberately**: any new `#[test]`. Adding one would discharge a different
obligation than the one project AC-003 states, and the testing brief forecloses it in as many words.

## Risks and coupling (PR-scoped)

| Risk | Coupling | Mitigation in this PR |
| --- | --- | --- |
| **The runs happen against a partial gate** — one dependency merged, not both | Both `pinned-narrative-tree-and-compiling-step` and `narrative-checker-mounted-with-pinned-path` must be on the branch, or run 3's coverage line does not exist and run 2 renders a surface a contributor will never see | The story map is explicit: run "against the assembled gate, not a partial one" (`_storymap.md:165-171`). Run 1 is also the check: if the checker's banner is absent from the baseline, stop |
| **Someone fixes what the run reveals** | Every finding is HS-S0145's input; a fixed defect leaves that story documenting a limit that no longer exists, or worse, inventing one | EC-009 forbids it; the PR boundary lists source trees as out; AC-005's diff assertion catches it mechanically |
| **The red run is mistaken for whole-gate evidence** | `run_steps` bails at the first failure, so run 2 is silent about every later step | AC-001 requires the truncation to be stated beside the transcript; AC-004 makes run 3 the only whole-gate claim |
| **The record rots into a claim nobody can re-verify** | The corpus grows under HS-P0021/22/23; page counts and paths move | AC-006's provenance block plus the append-a-dated-section rule; NF-005 keeps this a review obligation rather than a gate rule that would couple `xtask` to `.bklg/` |
| **Platform-shaped output read as a defect** | Mock finding 3's doctest name is Windows-shaped | NF-003 records the platform beside the transcript; AC-009 measures the prefix including the 16-character doctest prefix, which is the platform-independent part of the budget |
| **Cost pressure substitutes `--fast` for the third run** | `--fast` drops `OPTIONAL`, so a green would not be the gate project AC-003 names | NF-002 states the requirement and the cost, so the trade is refused explicitly rather than made silently under time pressure |
| **HS-S0145 starts before the findings are legible** | It is `blocks:`-ed on this story precisely to consume three measurements | NF-007 requires each finding under its own heading with its evidence line quoted; the slice is implemented in one context, this story first (`_storymap.md`, merge order 4.9-4.10) |
| **The fixture's `ES-40` residual gets "tidied" in passing** | Two other stories are pinned to that page's exact bytes | Recorded, not edited — the PR boundary lists any correction to `_design.md`'s fixture as out of scope; the break is to the fence's assertion, which is independent of the citation |

## Dependencies

**Blocks on** (both must be merged before the first run — the gate must be assembled, not partial):

- `pinned-narrative-tree-and-compiling-step` (HS-S0136) — creates `docs/append-conditions.md`, the
  fixture this story breaks, and `xtask/src/narrative.rs`, the harness rustdoc is predicted to name.
  Without it there is no page to falsify and no compile step to fail.
- `narrative-checker-mounted-with-pinned-path` (HS-S0138) — creates the second `REQUIRED` step whose
  green one-liner is AC-004's coverage evidence, and whose separate banner is what makes AC-002's
  attribution question answerable at all.

**Unlocks**:

- `documented-blind-spots-and-their-proofs` (HS-S0145) — same slice, implemented immediately after
  this story in the same context, consuming three of its measurements: which file the failure names,
  whether a fence is run or only compiled, and the green run's coverage numbers.

**Not a dependency, stated because it looks like one**: `fence-discipline-and-allowance-list`,
`hidden-content-resolution`, `spec-trace-clause-id-accessor`, `narrative-citation-resolution` and
`frozen-documentation-must-pin` all land before this story in the project's merge order, but this
story reads none of their behaviour. They are in the tree when it runs, which is why run 3 is a
whole-gate green rather than a claim about two steps.

## Anchors (progressive disclosure)

Linked, not pasted. Every path was confirmed to exist at spec time.

| Anchor | Why it is load-bearing | When to open | Serves |
| --- | --- | --- | --- |
| `.bklg/docs-that-teach/checked-documentation-surface/_design.md` | Carries the fixture page **verbatim** (`## The doctest`) — the exact bytes the break is applied to — plus `## Composition`'s failure report, `## Density budget`'s numbers, `## Hierarchy`, `## Surfaces`' state ids and `## Mock`'s dated-instrument discipline. Binding: this story renders two of its surfaces and must not re-decide any of it | Before applying the break (for the fence's exact text) and again before writing the composition and density sections of the record | AC-001, AC-006, AC-008, AC-009 |
| `.bklg/docs-that-teach/checked-documentation-surface/_decomposition.md` | The testing brief's AC-003 row (`:568-584`) is the procedure and the non-substitutability rule; the Notes (`:676-703`) hold fixture ownership and the merge-gate command ladder; architecture Note 3 (`:186-189`) is the residual-degradation prediction AC-003 measures, Note 10 (`:351-376`) the limits list the findings feed | Before writing the procedure, and again when a measurement contradicts a prediction | AC-001, AC-003 |
| `.bklg/docs-that-teach/checked-documentation-surface/project.md` | AC-003's exact wording (`:208-211`) is what the record is judged against; DoD items 2 and 8 set the "decorative gate" and "never claims teaching" bars; the risk table (`:294`) forbids touching the corpus | At the start, and once more before the record's closing section is written | AC-005, AC-007 |
| `.bklg/docs-that-teach/initiative.md` | DoD scenario 2 (`:416-420`) is the initiative-grain sentence this story turns green, and its "the failing half is the one that matters" is the framing the record inherits | Once, at the start — it is the reason the story exists | AC-001 |
| `xtask/src/main.rs` | The mount point. `REQUIRED` at `:105`; `run_steps`' banner at `:864` and first-failure `bail!` at `:887`; the child's `xtask failed:` tail at `:709-715`; the unfiltered constitution step at `:480-492`; the normative `probe` doc comment at `:89-102` | Before run 2, to know exactly which lines the transcript should contain and which are impossible | AC-002, AC-004, AC-008 |
| `.kb/playbooks/verify-the-referent-and-report-coverage.md` | The accepted atom behind AC-004's coverage requirement, grounded in this repository's own 84-of-338 measurement: a reader told a number can falsify it; one told "no problems found" cannot | While recording run 3 — it is what makes the green run worth transcribing | AC-004 |
| `.kb/playbooks/landing-a-stricter-gate-without-a-red-baseline.md` | The atom on the class of mistake EC-001 guards: a check's evidence is only as good as the baseline it is read against | Before run 1, and immediately if run 1 is red | AC-001 |
| `standards/rust/81-checks-that-cannot-be-types.md` | RS-81-1 at `:10` ("prove the check's blind spot in its own tests, then state it in its own documentation") is why this story exists; RS-81-4 at `:264` ("a zero exit status is evidence of nothing") is why the green run is transcribed rather than asserted | Before writing the findings and the limits section | AC-003, AC-007 |
| `experiments/rustc-ice-gat-foreign-trait/README.md` | Lines 1-22 are the in-house shape for a verbatim machine transcript with the mechanism named beside it — copy the shape rather than inventing one | When laying out `_falsification.md`'s first transcript | AC-006 |
| `.bklg/docs-that-teach/checked-documentation-surface/pinned-narrative-tree-and-compiling-step/spec.md` | The dependency that creates the fixture and the compile step, and the owner of `EC-005` (banner attribution) and of the `fail-removed-item` observation this story must **not** repeat | Before run 2, to confirm the step's name and ordering, and whenever routing a finding about the compile step | AC-002 |
| `.bklg/docs-that-teach/checked-documentation-surface/narrative-checker-mounted-with-pinned-path/spec.md` | The dependency that creates the checker step whose one-line success summary is AC-004's coverage evidence and whose separate banner makes attribution answerable | While recording run 3's coverage numbers | AC-004 |
| `RUNBOOK.md` | `:914-928` is the in-house precedent — a probe-gated step printing `skipped` on all three runners while two documents vouched for it. EC-006 is that failure reproduced, and it is a halt | If any transcript contains a `skipped:` line for either narrative step | AC-004 |
| `.redkiln/config.yaml` | The `verify:` block wires `affected_gate`, `integration_scoped: cargo xtask ci --fast` and `e2e: cargo xtask ci`, and `require_ledger: true` is why `_ledger.md` must cite the record per criterion | When assembling the merge-gate ladder and filling the ledger | AC-005 |
| `.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md` | The initiative's personas and journeys are carried here, not in `.kb/product/` (which holds no persona atom). The two this story serves — contributor running the gate, reviewer reading its output — and the qualification that none has been directly observed | Once, when framing the record's opening paragraph for a reader who is not the author | AC-007 |
| `.bklg/docs-that-teach/checked-documentation-surface/_storymap.md` | `:165-171` is the requirement to run against the assembled gate; Activity E (`:34-38`) is why this story is the one that makes the other four worth anything; the merge order fixes this story before HS-S0145 | Before scheduling the runs, to confirm both dependencies are on the branch | AC-001 |
| `xtask/src/lint_constitution.rs` | `:192` is the success-summary primitive (`  {n} atoms, all consistent`, two-space indent) the checker's `  {n} pages, all consistent` line is modelled on, and `:169-198` is the accumulate-all-problems shape the design's composition rules cite | When measuring run 3's success line against the one-line budget | AC-009 |

## Clarifications resolved during spec

1. **Two acceptance criteria were added to the front half's seven: AC-008 and AC-009.** The first
   pass enumerated AC-001…AC-007 in `## Behavior and interfaces`, and those seven cover the
   procedure, the measurement, the residue and the honesty bar — but none of them made the
   *composition* and *density* invariants gateable. This story renders `gate-narrative-compile-step`
   in `fail-broken-fence`, the only story in the project that does, so it is the one place
   `_design.md`'s composition claims meet real output rather than a unit test over a string; an
   invariant left as a prose bullet would carry no ledger row and would never be checked. AC-008
   (composition and hierarchy fidelity, with divergences named and routed) and AC-009 (the density
   budget measured with its real numbers) are therefore rows in the table and rows in `_ledger.md`.
   Nothing in the front half is contradicted: both are observations of the same three runs AC-001 and
   AC-004 already require, and neither adds a source change.
2. **"Verifying test" for an AC whose proof is a recorded procedure.** The testing brief forbids a
   `#[test]` here, so each ledger row's `verifying_test` names the **command plus the section of
   `_falsification.md` that holds its transcript**, which is the artifact a reviewer re-runs. This is
   deliberate rather than a placeholder; a row naming a `#[test]` would be a row naming the wrong
   obligation.
3. **All three runs are the full `cargo xtask ci`.** `--fast` is the project's interim merge bar and
   `cargo test --locked -p xtask --doc` is the iteration loop, but project AC-003 names the full
   gate. Recorded as NF-002 so the substitution is refused explicitly rather than made under time
   pressure.
4. **What happens if the break does not fail** is settled in advance as EC-002 rather than left to
   the implementer's judgement: it is recorded as a measurement and routed, the story does **not**
   close green, and the break is **not** silently swapped for a weaker one (deleting an item), which
   would discharge project AC-002 while leaving AC-003 unproven.
5. **Anti-pattern 7 is measured, not assumed, on the compile surface.** The failure body is rustdoc's
   report, which this repository does not control. If its first visual row does not begin with a
   `path:line` this project chose, that is recorded under AC-008 as a limit of the mechanism and
   routed to HS-S0145 — neither fixed here nor quietly dropped from the design's list.
6. **The fixture page is milestone 1's and is permanently retained.** The testing brief leaves
   deletion-vs-retention to the implementer; retention is chosen here because AC-006 requires the
   procedure to stay re-runnable, and because `_design.md` names the page as the literal artifact two
   stories build against.
7. **Platform is part of the record.** Mock finding 3 observed a Windows-shaped doctest name, so
   NF-003 requires the platform and toolchain beside the transcripts; without it a path separator
   reads as a defect to the next person on another OS.
