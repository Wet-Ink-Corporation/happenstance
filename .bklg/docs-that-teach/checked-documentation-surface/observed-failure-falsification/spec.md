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
