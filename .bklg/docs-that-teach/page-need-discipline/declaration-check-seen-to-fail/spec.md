---
item: HS-S0151
stage: spec
created: 2026-08-17T13:16:10.417Z
updated: 2026-08-17T13:16:10.417Z
template_sig: 87bbf1d0
rendered_sig: 5971eddc
---

# Spec — The declaration check watched failing, and recovering

## Scope lock

| | Path |
| --- | --- |
| Initiative (gold source) | [`.bklg/docs-that-teach/initiative.md`](../../initiative.md) — AC-07, DoD scenario 2, DoD-8 |
| Initiative decomposition | [`.bklg/docs-that-teach/_decomposition.md`](../../_decomposition.md) |
| Project | [`.bklg/docs-that-teach/page-need-discipline/project.md`](../project.md) — AC-007, DoD "AC-007's failure observed and recorded in the ledger" |
| This spec | `.bklg/docs-that-teach/page-need-discipline/declaration-check-seen-to-fail/spec.md` |
| Key briefs | [`../_decomposition.md`](../_decomposition.md) — Architecture brief AC-007 and Note 1 (CR-1…CR-4); UX brief UX-007, UX-008; Testing brief AC-007 and its Notes |
| Signed-off design (binding) | [`../_design.md`](../_design.md) — surface `lint-terminal-output`, the S1 declaration form, the closed `NEEDS` set, the S4 composition and density budget |
| Grounding | [`../_grounding.md`](../_grounding.md) — no Accepted decision atom governs this work |
| Roadmap pointer | [`../_storymap.md`](../_storymap.md) — milestone `page-need-gate-step`, story 6 of 8, merge order 2.2 |

## One-line PR slice

Break a governed page two ways — two needs, then an unenumerated need — watch `cargo xtask ci` fail
by file and line each time, revert, and record green plus a clean `git status`; all three captures
verbatim in the ledger.

## Executive summary

The slice-mate [`page-need-checker-mounted-in-the-gate`](../page-need-checker-mounted-in-the-gate/spec.md)
lands the checker and mounts it in `REQUIRED`. **This story is the only thing that proves the mount is
real.** It ships no Rust, no rule atom and no page: its whole deliverable is a recorded, reproducible
observation of the assembled gate failing on purpose and then recovering, captured verbatim into
`_ledger.md`.

The delta over the checker story is precise and is the reason the story map refuses to fold the two
together ([`../_storymap.md`](../_storymap.md), "Why the slices fall here", fourth bullet): the checker
story's tests call the checker's own pure functions over synthetic `&str` literals and **never invoke
`cargo xtask ci`**. A green unit test therefore says nothing about whether the step is in `REQUIRED`,
whether dispatch reaches it, whether `affected.rs` runs it on a prose-only diff, or whether a real
author breaking a real page sees a usable message. This repository has already paid once for a gate
step that was wired and could only ever print `skipped` while two documents vouched for it
(`RUNBOOK.md:918-928`). This story is the instrument that would have caught that, applied to this
project's own step before anyone trusts it.

What lands in the diff is one file — this story's `_ledger.md` — carrying three captured runs (two
failing, one recovered), their exact commands, their exit status and their provenance. The broken
pages themselves are working-tree edits that are reverted and never committed; a break that survives
into the diff is a failed acceptance criterion, not an artefact.

## Context pack

The load-bearing decisions this story must honor. Read this and you can start; the deeper artefacts
are behind the anchors, bound to the criteria they serve.

**The persona-journey slice this realizes.** S3, the lint's terminal output, read by *the author who
just broke the rule* ([`../_decomposition.md`](../_decomposition.md), UX brief, Intent table). Their
goal is stated as one sentence and it is the sentence this story tests: *learn what is wrong, where,
and how to undo it, in one run.* Every capture below is judged against that reader, not against an
exit code. The initiative's own framing is that the failing half is what matters and the reversible
half is what makes the check safe to use (UX-008) — so the recovery is a first-class observation, not
a tidy-up step.

**Decision 1 — the observation is the deliverable, and no `#[test]` may stand in for it.** The testing
brief classifies AC-007 as *end-to-end/fixture*, the one criterion in this project proven by a
recorded procedure rather than a `#[test]`, "for the identical reason HS-P0020's own AC-003 is: a unit
test that calls the checker function directly never invokes `cargo xtask ci`"
([`../_decomposition.md`](../_decomposition.md), Testing brief, AC-007). The project's Definition of
Done says the same in the imperative: *observed and recorded in the ledger, not merely asserted*
([`../project.md`](../project.md)). Adding a `#[test]` that shells out to the gate is not an
acceptable substitute either — it would be the same claim one indirection further from the human who
has to believe it.

**Decision 2 — the thing run is `cargo xtask ci`, the whole gate.** Not the checker function, not
`cargo run -p xtask -- lint-pages` alone. The step-alone invocation
(`cargo run --locked --quiet -p xtask -- lint-pages`, pinned in [`../_design.md`](../_design.md)'s
constants table) is permitted **while iterating** and is worthless as evidence, because it bypasses
exactly the thing under observation: `REQUIRED` (`xtask/src/main.rs:105`) and the dispatch, help and
`lint_steps` entries that make the step reachable by name (`xtask/src/main.rs:689-700`, `:718`,
`:799-808`, with `steps_named`'s panic at `:816-826`). The captured runs are `cargo xtask ci`.

**Decision 3 — three breaks, and each is a page a person could plausibly write.** The closed need set
is four tokens — `orientation`, `tutorial`, `how-to`, `explanation` — and `reference` was deliberately
*removed*, because rustdoc and `spec/SPECIFICATION.md` already own that surface
([`../_design.md`](../_design.md), "S1 vocabulary — DT-2"). That makes `reference` the correct
unenumerated token for the second break: it is the token an author would type in good faith, and
[`../_design.md`](../_design.md)'s anti-pattern 16 names it explicitly. A nonsense string
(`zzz`, `foo`) would test the same code path and prove less, because nobody would ever ship it.

**Decision 4 — the message shape is the acceptance bar, not the exit code.** Every problem is
`{path}:{line} — {what is wrong}; {why it matters, or what to do}`, all problems are printed before
`bail!("{n} problem(s) …")`, and a green run *says what it checked*. That grammar is the repository's,
inherited verbatim from `xtask/src/lint_constitution.rs:169-198` (report-all-then-bail), `:192` (the
success line) and `:375-379` (the repair inside the message), and [`../_design.md`](../_design.md)'s
S4 section adopts it with no alternative considered. A capture in which the gate merely goes red is
not a discharged criterion: the file must be named, the line must be named, and the reader must be
able to act without opening a second document.

**Decision 5 — report-all is observed, not assumed.** UX-007's stated test is literal and belongs
here because this is the only story that runs real pages through the real gate: *break three pages in
three different ways, run the gate once, and count three named problems*
([`../_decomposition.md`](../_decomposition.md), UX brief, UX-007). A check that stops at the first
problem turns one review cycle into six, and the only way to see that it does not is to break more
than one page at once.

**Decision 6 — a vacuous run is a failure, so the green baseline must be non-vacuous, and that is a
precondition rather than something to work around.** The checker `bail!`s when the pages tree holds no
pages, mirroring `xtask/src/lint_constitution.rs:174-176`; the UX brief names *"0 pages, all
consistent"* as the decorative-gate failure with this project's name on it
([`../_decomposition.md`](../_decomposition.md), UX brief Note 1). Two consequences bind this story.
The baseline capture must show a **non-zero page count** in the success line, or the observation has
no attributable before-state. And if HS-P0020's pinned pages tree is still empty when this story runs,
the "revert → green" half is unreachable by construction — the honest move is to **halt loudly and
record the block**, never to fabricate a green or to leave a fixture page behind so the tree stops
being empty. The testing brief permits a fixture page only as throwaway test material analogous to
`happenstance-testkit`'s `MemoryFixture` ([`../_decomposition.md`](../_decomposition.md), Testing
brief, Notes), and a throwaway that has to survive the revert is not a throwaway.

**Decision 7 — verbatim capture, with provenance, because a document that vouches for a check is not
evidence that the check runs.** `RUNBOOK.md:918-928` records the incident in this repository's own
words: a step printed `skipped` on all three runners while `xtask/src/main.rs` asserted the check was
real there and a later phase spent it as a proof artefact. Paraphrase is how that happens. Each
capture carries the exact command line, the exit status, the step's own name line, the problem lines
unedited, and the commit the working tree sat on — `.redkiln/config.yaml` already treats a recorded
manual verification as first-class proof (`require_ledger: true`, `:67`) and requires commit
provenance (`:73`).

**Decision 8 — nothing this story observes is fixed in this story's boundary.** If the checker names
a file but not a line, stops at the first problem, or prints a message the author cannot act on, that
is a defect in the slice-mate's deliverable. Slice-mates in `page-need-gate-step` are implemented in
one context, so the fix lands in `xtask/src/**` under
[`page-need-checker-mounted-in-the-gate`](../page-need-checker-mounted-in-the-gate/spec.md)'s boundary
and this story re-runs the observation. Recording a substandard message as "observed" is the failure
mode this story exists to prevent, one level up.

**Decision 9 — one finding is routed here, not settled here.** [`../_design.md`](../_design.md)'s Mock
section, finding 1, records that the ≤100-character problem-line budget is *not reachable* for this
tree: a realistic path plus the `see standards/pages/…` citation runs to 112 characters, and the
inherited spelling at `lint_constitution.rs:369-378` is 111 on its own. This story is where that meets
a real message on a real path. It **records the measured length** as evidence and routes the finding;
it does not re-decide the budget, the citation form or the no-wrap rule — that is a design amendment,
and the design is signed off (`_design.md`, "Sign-off").

## Integration contract

| | |
| --- | --- |
| **Slice / milestone** | `page-need-gate-step` |
| **Slice-mates** | `page-need-checker-mounted-in-the-gate` (implemented first, in the same context) |
| **Archetype** | `capability` — a user-observable slice: the author who broke a rule meets the message that tells them how to undo it |
| **Mount point** | `xtask/src/main.rs` — the `REQUIRED` array at `:105`, the gate's composition root, reached through `cargo xtask ci` (`run_ci`, `:828-831`). This story adds no entry to it; it is the story that **proves the entry is live end to end**, which no test in the workspace does |
| **Wires into** | `xtask/src/main.rs:689-700` (dispatch), `:718` (`print_help`), `:799-808` (`lint_steps`), `:816-826` (`steps_named`'s panic on a name absent from `REQUIRED`) — the by-name routes used while iterating; the slice-mate's checker module and its `pub(crate) fn run(mode: Mode) -> Result<()>`, shaped after `xtask/src/lint_constitution.rs:170`; HS-P0020's `pub(crate)` pages-tree constant, which is the tree the broken page lives in (Architecture brief Note 2 — one constant, never two); `xtask/src/affected.rs:118-125` (the unconditional file-reading list) and `INERT` at `:249-260`; git itself (`git checkout --`, `git status --porcelain`, `git rev-parse HEAD`) as the reversibility instrument |
| **Design-system primitives consumed** | The gate's diagnostic grammar as [`../_design.md`](../_design.md) fixes it for S4: `{path}:{line} — {what is wrong}; {why it matters or what to do}`, problems sorted by path then line with no blank lines between them, the `bail!` count below, the success line above zero problems, and the repair **inside** the problem line rather than in a footer. Also S1's declaration form — `> **Answers:** \`token\` — question?` as the first block after the H1 — which is what a break perturbs |
| **Renders surfaces** | `lint-terminal-output` — observed in states `green`, `single-problem` and `many-problems`, and (only if the precondition fails) `vacuous-tree`. The story **changes** no surface; it is the only story that puts three of that surface's declared states in front of a human. The `page-need-declaration` surface's `two-declarations`, `unenumerated-token` and `missing` states are the *inputs* it induces, transiently |
| **Advances DoD scenario** | Initiative **DoD scenario 2** — *"@smoke — a deliberately broken page fails the gate, by name … the edit is then reverted and the gate returns to green. Both halves are observed; the failing half is the one that matters"* ([`../../initiative.md`](../../initiative.md)). This story moves the **page-need half** of that scenario to green; the "a claim is no longer true of the library" half is HS-P0020's `checked-documentation-surface`. It also underwrites DoD-8's premise that the singular-need property is enforced rather than asserted |
| **Note on the affected gate** | This story's committed diff lives entirely under `.bklg/`, which is on `INERT` (`xtask/src/affected.rs:249-260`), so `cargo xtask affected --base main` selects **no package** and runs only the unconditional file-reading list at `:118-125`. Because the slice-mate adds the new checker to that list (Architecture brief, CR-4), the story-grain gate is itself an incidental second observation that a prose-only pull request does not read nothing |

**Delivered mounted, not as an isolated component.** There is no component here to isolate: the
deliverable *is* the integrated run. If the slice-mate's step is not in `REQUIRED` when this story
executes, `cargo xtask ci` will not exercise it and the observation is void — which is precisely why
`depends_on` names that story and the story map puts it at merge position 2.1.

## PR boundary

**In this PR**

- `_ledger.md` in this story's own directory, carrying the three captures verbatim with commands,
  exit status, step-name lines, problem lines and commit provenance.
- The story's implementation report and any companion notes in the same directory.
- Nothing else. The breaks are working-tree edits made, observed and reverted inside the run.

```
.bklg/docs-that-teach/page-need-discipline/page-need-checker-mounted-in-the-gate/implementation-report.md
.bklg/docs-that-teach/page-need-discipline/declaration-check-seen-to-fail/**
```

> **Amended 2026-08-18, after implementation.** The slice-mate's report path was
> added because this story's fence admitted only its own directory, and `d7d27c5`
> corrected eight lines of
> `page-need-checker-mounted-in-the-gate/implementation-report.md`.
>
> That is this story's whole job reaching one file upstream. It breaks the gate,
> watches it fail, and reverts — so it is the first thing in the project to
> observe the checker from outside, and what it observed contradicted eight lines
> the checker story had already written about its own behaviour. Correcting them
> in place is better than recording a contradiction and leaving it, and the eight
> lines are named in this story's `_ledger.md`. No code, page or rule was touched:
> `d7d27c5` is `.bklg/` and telemetry only, which is why `git diff main -- xtask`
> is empty for this story and the two red captures it records were reverted in
> full.

**Explicitly not in this PR**

- The checker module, its `const`s, its tests and its five mount points — `page-need-checker-mounted-in-the-gate`
  (`xtask/src/**`). A defect this observation surfaces is fixed under **that** story's boundary in the
  same slice context, which is the slice working as designed, not scope drift.
- Any page authored, kept or repaired in HS-P0020's pinned pages tree. This project's tree is the
  *rules*; that project's is the *pages* ([`../project.md`](../project.md), "Out of scope").
- Any rule atom, router edit or `docs/README.md` row — `discipline-on-disk`'s slice.
- Any committed broken page, anywhere. A break that reaches a commit is a failed criterion.
- Any amendment to [`../_design.md`](../_design.md)'s density budget or citation form on the strength
  of the 100-character finding. The measurement is recorded; the amendment is not this story's.

**Merge DoD one-liner.** `cargo xtask ci` green on a clean tree, `git status --porcelain` empty, and
`_ledger.md` carrying three verbatim captures — two red naming file and line, one green naming a
non-zero page count — each with its command, exit status and commit.

## Behavior and interfaces

The implementer executes a procedure and records it. Every row below is something a reader of the
ledger can check without re-running anything.

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **Preflight: a green, non-vacuous baseline** | Before any edit: clean worktree, `git rev-parse HEAD` recorded, `cargo xtask ci` run and captured green. The page-need step's success line must show a **non-zero page count** (`  {n} pages, {m} rules, all consistent`). A baseline captured over an empty tree attributes nothing. | `xtask/src/lint_constitution.rs:192`; [`../_design.md`](../_design.md), States table, S4 "Empty" row; [`../_decomposition.md`](../_decomposition.md), Testing brief AC-006 |
| **Halt rule on a vacuous pages tree** | If the pinned pages tree holds no pages, the step `bail!`s and there is no green baseline to return to. The story **halts and records the block against its `depends_on`** — it does not leave a fixture page behind to make the tree non-empty, and it does not report a green it did not see. | `xtask/src/lint_constitution.rs:174-176`; [`../_decomposition.md`](../_decomposition.md), UX brief Note 1 and Testing brief Notes ("throwaway … analogous to `MemoryFixture`") |
| **Break A — two declarations on one page** | A single governed page is edited so two `> **Answers:**` lines are present (or one line naming two tokens). Expected: `cargo xtask ci` exits non-zero; one problem line naming that file and **the line number of the offending declaration**; the message says a page answers one need. | [`../_design.md`](../_design.md), States table, S1 "Error" row; [`../_decomposition.md`](../_decomposition.md), Testing brief AC-007 (1); UX brief UX-001 |
| **Break B — an unenumerated need** | Separately, on a clean tree, a page is edited to declare `` `reference` `` — the token removed from the set on purpose, and the one an author would type in good faith. Expected: fails, naming file, line and the offending token. | [`../_design.md`](../_design.md), "S1 vocabulary — DT-2" and anti-pattern 16; [`../_decomposition.md`](../_decomposition.md), Testing brief AC-007 (2) |
| **Break C — no declaration at all** | Used only in the report-all run: a page's declaration line is deleted. Expected: a problem naming that page (the file-level case, no line number, per the design's S4 sample). | [`../_design.md`](../_design.md), Composition, S4 sample output (third problem line); States table, S1 "Empty" row |
| **One run, every problem** | With A, B and C applied simultaneously, **one** `cargo xtask ci` invocation reports **three** problems, sorted by path then line, with no truncation, no "and others", and a `bail!` line carrying the count. | [`../_decomposition.md`](../_decomposition.md), UX brief UX-007; `xtask/src/lint_constitution.rs:169-198`; [`../_design.md`](../_design.md), anti-pattern 10 |
| **The observed command is the whole gate** | Captures are `cargo xtask ci`. `cargo run --locked --quiet -p xtask -- lint-pages` and `cargo xtask lints` are permitted while iterating and are recorded as such if recorded at all — they do not discharge anything, because they bypass `REQUIRED`. | [`../_decomposition.md`](../_decomposition.md), Testing brief AC-007 and its Merge-gate commands block; `xtask/src/main.rs:105`, `:799-808`, `:816-826`; `CLAUDE.md`, "Commands" |
| **Recovery, in both directions** | Each break is undone with `git checkout -- <page>`; the gate is re-run and captured **green**; `git status --porcelain` is captured **empty**. No cache to clear, no generated file left dirty, no manual step. If the router's generated region moved, `--write` residue counts as residue. | [`../_decomposition.md`](../_decomposition.md), UX brief UX-008; [`../_design.md`](../_design.md), S2 generated region and its `--write` repair |
| **Capture format** | Per capture: the exact command line, the exit status, the step's own name line, the problem lines **unedited**, and `git rev-parse HEAD`. Verbatim, in a fenced block, never paraphrased and never re-flowed — line length is itself evidence. | `RUNBOOK.md:918-928`; `.redkiln/config.yaml:67,73` (`require_ledger`, `require_commit_provenance`) |
| **Measured, routed, not settled** | The longest problem line's character count is recorded against [`../_design.md`](../_design.md)'s ≤100-character budget, which that file's own finding 1 predicts is unreachable. The number goes in the ledger and the finding is routed; the budget is not amended here. | [`../_design.md`](../_design.md), "Mock", finding 1; `xtask/src/lint_constitution.rs:375-380` (the inherited message spelling it measures against) |
| **Interfaces used, none built** | The step name `every page declares one need` and the task name `lint-pages` are consumed **by value** and are the slice-mate's to define; `steps_named` panics at build time on a mismatch, which is the intended failure rather than a silent omission. This story adds no `const`, no function and no test. | [`../_design.md`](../_design.md), constants table; `xtask/src/main.rs:816-826`; [`../_storymap.md`](../_storymap.md), "not a checkbox on the checker" |

## Data and migrations

**N/A.** This story introduces no persistent data, no schema, no store and no migration. It has no
runtime: it makes transient edits to files already tracked by git, observes a process, reverts the
edits, and writes prose. The only durable state it produces is captured text in `_ledger.md`, and the
only state-restoration mechanism it relies on is git's own — `git checkout --`, verified by an empty
`git status --porcelain`, which is the story's reversibility criterion rather than a data concern.

## Acceptance criteria

Eight criteria. Each is framed from the reader whose journey it belongs to — **S3, the author who
just broke the rule** ([`../_decomposition.md`](../_decomposition.md), UX brief, Intent table), and
behind them the repository owner who has to decide whether a green gate means anything. Every
verification is a *recorded procedure*, because that is what the testing brief classifies AC-007 as
and the project's Definition of Done demands in the imperative: *observed and recorded in the ledger,
not merely asserted* ([`../project.md`](../project.md)).

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **GIVEN** the slice-mate's step is in `REQUIRED` and the pinned pages tree carries at least one governed page, **WHEN** a maintainer who is about to trust this gate runs `cargo xtask ci` on a clean tree *before touching anything*, **THEN** the page-need step prints its own name and a success line carrying a **non-zero** page count (`{n} pages, {m} rules, all consistent`), the gate exits `0`, and that run plus `git rev-parse HEAD` is captured as the attributable before-state. A green captured over an empty tree is not a baseline — it is the `0 pages, all consistent` failure with this project's name on it. | **Procedural (ledger-recorded).** Capture 0 in `_ledger.md`: command line, exit status, step-name line, success line with its count, `git rev-parse HEAD`. Read against `xtask/src/lint_constitution.rs:192` (success line) and `:174-176` (vacuity `bail!`). If the count is zero the story halts under EC-001 rather than recording a green. |
| AC-002 | **GIVEN** an author mid-edit who has left a second `> **Answers:**` line on a governed page — the plausible accident, not a synthetic one — **WHEN** they run `cargo xtask ci`, **THEN** the gate exits non-zero and prints a problem line that names **that file and the line number of the offending declaration**, says the page answers more than one need, and tells them what to do — so they can repair it without opening a second document. A red gate that names the file but not the line does not discharge this. | **Procedural (ledger-recorded).** Capture 1: apply Break A to one page, run the whole gate, record the command, non-zero exit, the unedited problem line(s) and the head sha. Judged against [`../_design.md`](../_design.md), States table S1 "Error" row, and its S4 sample output; the `path:line` prefix is asserted by reading the captured line, not by re-running. Defect routes under EC-004. |
| AC-003 | **GIVEN** an author who, in good faith, declares `` `reference` `` — the token DT-2 *removed* on purpose because rustdoc and `spec/SPECIFICATION.md` already own that surface — **WHEN** they run `cargo xtask ci` on an otherwise clean tree, **THEN** the gate fails naming the file, the line, **and the offending token**, and points at where the closed set is written, so the author learns the set exists rather than guessing a fifth word. | **Procedural (ledger-recorded).** Capture 2: revert Break A first (clean tree confirmed by `git status --porcelain`), apply Break B, run the whole gate, record verbatim. Judged against [`../_design.md`](../_design.md), "S1 vocabulary — DT-2" and anti-pattern 16; testing brief AC-007 (2). |
| AC-004 | **GIVEN** a review cycle in which three governed pages are wrong in three different ways at once — two declarations, an unenumerated token, and no declaration at all — **WHEN** the author runs `cargo xtask ci` **once**, **THEN** all **three** problems print in that single run, as one block sorted by path then line with no blank lines and no per-problem heading, with **no** truncation, no ellipsis and no "and others", and a `bail!` line carrying the count `3` — so one review cycle stays one cycle instead of becoming three. | **Procedural (ledger-recorded).** Capture 3: revert to clean, apply Breaks A, B and C simultaneously, one invocation, record the whole block unedited. Count the problem lines in the capture and compare to the `bail!` count. This is UX-007's own stated test ([`../_decomposition.md`](../_decomposition.md), UX brief UX-007) and [`../_design.md`](../_design.md) anti-pattern 10. |
| AC-005 | **GIVEN** an author who has seen the gate go red and now wants their tree back, **WHEN** they run `git checkout -- <page>` for each break and re-run `cargo xtask ci`, **THEN** the gate returns **green** and `git status --porcelain` prints **nothing** — no cache to clear, no generated router region left dirty, no `--write` residue, no manual step. Every state the author can enter, they can leave, in one documented command. | **Procedural (ledger-recorded).** Capture 4 (and the interstitial reverts between captures 1–3): the `git checkout` command, the re-run's exit `0` and success line, and the *empty* output of `git status --porcelain` shown as an empty fenced block rather than described. UX-008's own test ([`../_decomposition.md`](../_decomposition.md), UX brief UX-008); residue routes under EC-005. |
| AC-006 | **GIVEN** a reader of this repository six months from now who has only the ledger and no ability to re-run anything, **WHEN** they open `_ledger.md`, **THEN** they find every capture **verbatim** — exact command line, exit status, the step's own name line, the problem lines unedited and un-re-flowed, and `git rev-parse HEAD` for each — and the story's committed diff contains **no broken page anywhere**, only files under this story's own directory. A document that *vouches* for a check is not evidence the check runs; `RUNBOOK.md:918-928` is this repository's own receipt for that. | **Procedural + gate-state.** Read `_ledger.md` against the capture-format row in "Behavior and interfaces"; confirm each capture carries all five elements. Then `git status --porcelain` empty and the story's diff confined to `.bklg/docs-that-teach/page-need-discipline/declaration-check-seen-to-fail/**`. Provenance is required by `.redkiln/config.yaml:67` (`require_ledger`) and `:73` (`require_commit_provenance`). |
| AC-007 | **GIVEN** the S3 reader scanning a column of failures, **WHEN** they read any captured problem line, **THEN** it is one line beginning `{path}:{line} — `, then what is wrong, then why it matters or what to do, with the **repair inside the line** and never in a footer paragraph or a "next steps" section — location first, remedy third, never removed. The longest captured line's character count is measured and recorded against [`../_design.md`](../_design.md)'s ≤ 100-character budget, whose own finding 1 predicts 112 for a realistic path plus citation and 111 for the inherited spelling at `lint_constitution.rs:369-378`. The measurement is evidence; the budget is **not** amended here. | **Procedural, measured.** For every problem line in captures 1–3: assert the `path:line — ` prefix by reading it (the file-level form with no line number is permitted only for the missing-declaration case, per the design's S4 sample third line), assert no footer repair appears anywhere in the capture, and record `awk '{ print length }'` over the captured block with the maximum called out. Judged against `xtask/src/lint_constitution.rs:169-198`, `:375-379` and [`../_design.md`](../_design.md) anti-pattern 12 + Hierarchy S4 row. Overrun is **routed**, not fixed (Decision 9). |
| AC-008 | **GIVEN** the same reader watching a gate that passes, **WHEN** they look at capture 0 and capture 4, **THEN** the step is **visible** — its own name line and a success line with a count are present — and **nothing else is**: no per-file progress, no spinner, no box drawing, no summary section and no "next steps" paragraph in any capture. A green run that printed nothing is indistinguishable from a step that did not run, which is the `RUNBOOK.md:920-925` incident in one sentence; and every decorative line is a line the third problem hides behind. | **Procedural.** Read captures 0 and 4 for the step name + success line (anti-pattern 11), and read all five captures for the absence of progress output, spinners, box drawing and summary paragraphs ([`../_design.md`](../_design.md), Transience policy, "S4 per-file progress / spinner — not rendered at all"; Composition, S4). Compare against the `green`, `single-problem` and `many-problems` frames in [`../design/mock.html`](../design/mock.html). |

**Coverage of the traced project AC.** All eight discharge halves of **AC-007** ([`../project.md`](../project.md)) and nothing else: AC-002 and AC-003 are its two named failing halves, AC-005 is its recovery half, AC-001 makes both attributable, AC-004 is UX-007's report-all obligation observed on real pages for the first time, AC-006 is the "recorded in the ledger, not merely asserted" clause, and AC-007/AC-008 are the composition invariants without which a red gate is merely red rather than *usable*. The initiative's **DoD scenario 2** page-need half moves to green when all eight are satisfied.

## Interaction quality

The surface is `lint-terminal-output` ([`../_design.md`](../_design.md), Surfaces). This story renders
no new surface and changes none — it is the only story that puts three of that surface's declared
states (`green`, `single-problem`, `many-problems`) in front of a human. **Every invariant below is
carried by an AC-### row in the table above**; this section only says which row carries which, and how
each is checked. A capture that satisfies the exit code and fails an invariant here is a failed
criterion routed to the slice-mate under EC-004, never a discharged one.

**State invariants.**

| Invariant | Carried by | How it is checked |
| --- | --- | --- |
| **In place, not a context jump** | AC-002, AC-003, AC-007 | The repair sits *inside* the problem line, so the author acts without opening a second document. This is the measured defect from `personas-and-journeys.md:174-181` — an explanation that existed in three files, none of them the one the reader was looking at — applied to a terminal |
| **Non-occlusion** | AC-004, AC-008 | Every problem prints, every run: no truncation marker hides the third one (anti-pattern 10), and no progress output, summary or box drawing pushes it off-screen (Transience policy, S4) |
| **Preserved focus / scroll / selection** | AC-008 | The terminal analogue is stated rather than skipped: the step must not clear the screen, rewrite lines in place, or animate. Scrollback *is* the reader's state here, and a spinner is the only thing that could destroy it — which is why S4's progress output is the one control the design deliberately removed |
| **Reversibility** | AC-005 | `git checkout --` → green → `git status --porcelain` empty, captured. Every state the author can enter, they can leave, with no residue and no manual step (UX-008) |
| **Keyboard reachability** | AC-001, AC-005 | Trivially satisfied and stated so it is not silently assumed: every observation is a shell invocation and every repair is a shell command; there is no pointer target anywhere in this surface |

**Composition invariants**, taken from the signed-off [`../_design.md`](../_design.md) and binding.

| Invariant | Carried by | How it is checked |
| --- | --- | --- |
| **Presentation exists at all** | AC-001, AC-008 | The step is *named* in every capture and a passing run prints a success line with a count. An exit code with no output is the bare-markup equivalent here, and it is precisely what `RUNBOOK.md:918-928` recorded going unnoticed (anti-pattern 11) |
| **Composition / placement** | AC-004 | One block; problems sorted by **path then line**; no blank lines between them; no per-problem heading; the step's name above, the `bail!` count below (Composition, "S4 — one terminal run, top to bottom") |
| **Transience** | AC-007, AC-008 | Persistent chrome: every problem line, the success line, and the repair **inside** the problem line. Not rendered at all: per-file progress and spinners. There is no fourth class, and a capture that shows one is a fail |
| **Density budget, with its real numbers** | AC-004, AC-007 | Problem lines per run: **unbounded, never truncated**. Problem line width: **≤ 100 characters** measured and recorded — with the design's own finding 1 predicting **112** for `<PAGE_DIR>/guide/first-projection.md:7` plus `see standards/pages/20-the-fold-line.md`, and **111** for the inherited spelling at `lint_constitution.rs:369-378`. The number is recorded; the budget is not re-decided (Decision 9) |
| **Hierarchy** | AC-007 | Left-to-right within one line: `path:line` primary, what is wrong secondary, why-it-matters/repair recessive. Recessive means *read third*, never *removed* — a repair moved to a footer inverts the hierarchy and fails anti-pattern 12 |
| **Named anti-patterns** | AC-002 (4), AC-003 (16), AC-004 (10), AC-007 (12), AC-008 (11) | Each of the five S4/S1 anti-patterns this story can observe is bound to exactly one criterion. Anti-patterns 1–3, 5–9 and 13–15 belong to surfaces this story does not touch and are named here only so the omission is deliberate |

## Error conditions

| id | condition | required behaviour |
| --- | --- | --- |
| EC-001 | **The pinned pages tree exists but holds no governed page.** The step `bail!`s (mirroring `xtask/src/lint_constitution.rs:174-176`) and there is no non-vacuous green to return to, so AC-001 and AC-005 are unreachable by construction | **Halt loudly and record the block against `depends_on`.** Do not fabricate a green. Do **not** author a page into HS-P0020's tree to make it non-empty — that crosses this project's scope boundary ([`../project.md`](../project.md), "Out of scope") and a fixture that must survive the revert is not a throwaway ([`../_decomposition.md`](../_decomposition.md), Testing brief, Notes) |
| EC-002 | **The pinned pages tree is missing entirely** (path moved or never created). The step's `.with_context()` names the pinned path and the run **fails**; it does not skip | Distinct from EC-001 and captured as such if it occurs. A skip here would be the decorative-step failure a second time. Record the context message verbatim and halt against the slice-mate |
| EC-003 | **The step is not in `REQUIRED`** when this story executes — or `steps_named` panics because a name diverged (`xtask/src/main.rs:816-826`) | The observation is **void**: `cargo xtask ci` would not exercise the checker and a green capture would prove the opposite of what it appears to. Halt against `page-need-checker-mounted-in-the-gate` and record why, rather than substituting `cargo run -p xtask -- lint-pages` |
| EC-004 | **The captured message is substandard** — names a file but not a line, stops at the first problem, puts the repair in a footer, or is unactionable without a second document | Not this story's to fix and not this story's to excuse. The defect lands in `xtask/src/**` under the slice-mate's boundary, in the same slice context, and this story **re-runs the whole observation** from capture 0. Recording it as "observed" is the exact failure this story exists to prevent (Decision 8) |
| EC-005 | **The revert leaves residue** — a dirty generated router region, a `--write` artefact, or any non-empty `git status --porcelain` after `git checkout --` | AC-005 fails. Capture the residue verbatim, name the file, and route it to the slice-mate; do not `git checkout` a wider path to make the status clean, because that hides the defect rather than observing it |
| EC-006 | **A break reaches a commit.** A broken page appears in `git diff main` at any point after the run | A failed criterion, not an artefact. Revert, re-verify with `git status --porcelain`, and re-capture. The ledger is never edited to describe a tree state that was not the one observed |
| EC-007 | **The gate fails for a reason unrelated to the break** (an unrelated red step in `REQUIRED`) | The capture is not evidence for AC-002/AC-003: a non-zero exit attributable to `formatting` or `clippy` proves nothing about the page-need step. Capture the *step-name line* and the problem lines, confirm the failing step is the page-need one, and clear the unrelated failure before re-running |

## Non-functional

| id | requirement | why it binds here |
| --- | --- | --- |
| NF-001 | **A second person can reproduce every capture from `_ledger.md` alone** — no tribal knowledge, no "run it and see". Each capture carries the exact command line and the head sha, so the run can be reconstructed on that commit | This is what makes a recorded procedure first-class proof under `.redkiln/config.yaml:67,73` rather than a note someone wrote |
| NF-002 | **Zero lines of Rust.** No `const`, no function, no `#[test]`, no dependency added to `xtask/Cargo.toml`. The step name and task name are consumed **by value** and are the slice-mate's to define | The story map is explicit that this is "not a checkbox on the checker" ([`../_storymap.md`](../_storymap.md)); a `#[test]` that shells out to the gate would be the same claim one indirection further from the human who must believe it (Decision 1) |
| NF-003 | **Captures are colour-free plain text.** Run with colour disabled (`CARGO_TERM_COLOR=never` or `--color=never`) so the ledger's evidence carries no ANSI escapes and reads identically in a pager, a diff and a terminal | UX-002's colour-never-alone floor applied to the evidence itself: a capture whose meaning depends on escape codes is unreadable in the medium the ledger is read in |
| NF-004 | **The committed diff selects no package.** `.bklg/` is on `INERT` (`xtask/src/affected.rs:249-260`), so `cargo xtask affected --base main` runs only the unconditional file-reading list (`:118-125`) and selects nothing to build | Stated so the story-grain gate's *narrowness* is expected rather than alarming — and because the slice-mate adds the new checker to that unconditional list, the affected run is an incidental second observation that a prose-only pull request does not read nothing |
| NF-005 | **Iteration cost is bounded.** `cargo run --locked --quiet -p xtask -- lint-pages` while iterating; the whole gate for the five captures of record. Roughly five full `cargo xtask ci` runs is the story's budget | Decision 2: the step-alone invocation is permitted while iterating and is worthless as evidence, because it bypasses `REQUIRED` |
| NF-006 | **`redkiln doctor` still reports exactly six `template-drift` advisories** after this story lands | The project DoD asserts it and the `backlog` CI job enforces it; a story that only writes markdown should not perturb it, and confirming that is cheaper than diagnosing it later |

## Implementation notes (non-prescriptive)

The implementer owns the sequence; what follows is the shape that avoids the known traps, not a script.

**Order that keeps each capture attributable.** Capture 0 (green baseline, non-zero page count) →
Break A alone → capture 1 → revert → Break B alone → capture 2 → revert → Breaks A + B + C together
→ capture 3 → revert all → capture 4 (green + empty `git status --porcelain`). Breaks A and B are
captured *alone* deliberately: a red run with three simultaneous breaks cannot show that either one
would have failed the gate on its own, and AC-002/AC-003 are each about one plausible author mistake.

**Choosing the pages to break.** Prefer real pages in the pinned tree, so the paths in the captured
messages are realistic — path length is exactly what AC-007's measurement is about, and a break on
`a.md` would flatter the budget. If the tree's contents make one page the only candidate, break the
same page in sequence rather than inventing pages; three distinct paths are only required for the
report-all run (AC-004), and a directory with fewer than three pages is EC-001 territory in miniature
— record the constraint rather than manufacturing pages around it.

**Capturing.** Redirect combined streams (`2>&1`) into a file per capture and paste that file into the
ledger unmodified — no re-wrapping, no trimming of leading spaces, no "…" for the middle of the gate's
output. The problem block and the step-name line must both survive; other steps' output may be elided
**only** with an explicit marker saying so, and never inside the page-need step's own block. Measure
with `awk '{ print length }'` over the captured problem lines and record the maximum next to the
budget it is compared against.

**Reverting.** `git checkout -- <page>` per page, then `git status --porcelain` with no arguments —
the un-narrowed form is the point, because a residue in the router's generated region would not show
up under a path-scoped status. If the generated region did move, `cargo xtask lint-pages --write` is
the slice-mate's repair, and needing it is EC-005, not a step in the happy path.

**Comparing against the design.** [`../_design.md`](../_design.md)'s S4 block and
[`../design/mock.html`](../design/mock.html)'s labelled frames are the reference the captures are read
against. Open the mock's `green`, `single-problem` and `many-problems` frames beside the real captures
once, at the end — the mock is a specimen composed before any feature code existed, so a divergence is
a question about which one is wrong, not automatically a defect in the code.

**When something is wrong.** Stop and route it (Decision 8, EC-004). The slice is implemented in one
context precisely so that a defect this observation surfaces can be fixed under
[`page-need-checker-mounted-in-the-gate`](../page-need-checker-mounted-in-the-gate/spec.md)'s boundary
in the same pass; what is not available is recording a substandard message as observed.

## Tests and CI (merge gate)

Grounded in [`../_decomposition.md`](../_decomposition.md)'s Testing brief — which classifies AC-007
as **end-to-end/fixture** and adds **procedural (ledger-recorded)** as a fifth tier because
`.redkiln/config.yaml`'s `require_ledger: true` already treats a recorded manual verification as
first-class proof.

| tier | command / path | proves |
| --- | --- | --- |
| **end-to-end / fixture** | `cargo xtask ci` — captures 1, 2 and 3, each on a deliberately broken tree | AC-002, AC-003, AC-004. The whole gate, reached through `REQUIRED` (`xtask/src/main.rs:105`) and `run_ci` (`:828-831`) — the only invocation that proves the mount is live |
| **end-to-end / fixture** | `cargo xtask ci` — captures 0 and 4, on a clean tree | AC-001, AC-005, AC-008. Non-zero page count before; green with no residue after |
| **gate-state** | `git status --porcelain` (un-narrowed), `git rev-parse HEAD`, `git diff main --stat` | AC-005, AC-006. The reversibility instrument and the proof that no broken page reached the diff |
| **procedural, measured** | `awk '{ print length }'` over the captured problem block; visual read against [`../_design.md`](../_design.md) S4 and anti-patterns 10, 11, 12, 16 | AC-007, AC-008. The composition invariants an exit code cannot see |
| **procedural (ledger)** | `.bklg/docs-that-teach/page-need-discipline/declaration-check-seen-to-fail/_ledger.md` | AC-006. Every row cites a real capture; `redkiln verify --grain story` blocks `implement → report` until each is `satisfied: true` with non-placeholder evidence |
| **gate-integration (story grain)** | `cargo xtask affected --base main` | NF-004. On this story's `.bklg/`-only diff it selects no package and runs the unconditional file-reading list at `xtask/src/affected.rs:118-125` — an incidental second observation that a prose-only PR does not read nothing |
| **integration (project bar)** | `cargo xtask ci --fast` | The non-terminal project's DoD bar (`.redkiln/config.yaml`, `verify.integration_scoped`). `--fast` runs `REQUIRED` only, which *includes* the new step — but the captures of record are the full `cargo xtask ci`, per Decision 2 |
| **backlog hygiene** | `redkiln validate --kb && redkiln doctor` | NF-006. Exactly six `template-drift` advisories; nothing hand-authored into `.kb/` by this story |
| **explicitly not owed here** | `cargo test -p xtask` | Named so nobody spends it as evidence: those are the slice-mate's unit tests over synthetic `&str` literals, and a green run of them says nothing about whether the step is in `REQUIRED` ([`../_decomposition.md`](../_decomposition.md), Testing brief AC-007) |

## Risks and coupling (PR-scoped)

| Risk | L / I | Coupling and mitigation |
| --- | --- | --- |
| **The pinned pages tree is empty when this story runs** — HS-P0020 has not merged content and this project is second in merge order | High / High | The story's top risk and the reason EC-001 is written before the happy path. Mitigation is procedural and unglamorous: halt, record the block against `depends_on`, and do not author pages into another project's tree. The tempting workaround — a fixture page left behind so the tree stops being empty — converts a recorded block into a silent scope breach |
| **The slice-mate's message is substandard and the fix reopens `xtask/src/**`** | Medium / Medium | By design, not drift: slice-mates share one implementation context, so EC-004's fix lands under the slice-mate's boundary and this story re-runs. The risk is *scope creep in the other direction* — this story quietly editing `xtask/src/` to make its own capture look right. NF-002 (zero lines of Rust) is the tripwire |
| **A break survives into a commit** | Low / High | EC-006. `git status --porcelain` after every revert, and `git diff main --stat` before the work commit. A committed broken page would also poison the slice-mate's own green baseline |
| **Paraphrase at ledger-writing time** | Medium / High | The failure `RUNBOOK.md:918-928` records, one level up: two documents vouched for a step that only ever printed `skipped`. Mitigation is mechanical — capture to a file with `2>&1`, paste the file, never retype. Line length is itself evidence (AC-007), so re-flowing destroys the measurement |
| **The ≤ 100-character budget is breached and someone amends the design to match** | High / Medium | Decision 9 and the PR boundary forbid it. The design is signed off with four carried conditions; a density amendment is a design change with its own gate, not a consequence of a measurement. Record the number, route the finding |
| **An unrelated red step makes the capture ambiguous** | Medium / Medium | EC-007. Every capture records the *step-name line*, so a reader can see which step failed; a capture that only shows a non-zero exit is not evidence for AC-002 or AC-003 |
| **Gate runtime discourages re-running after a route** | Medium / Low | Five full gate runs is the budget (NF-005). Iterate with the step alone, then re-capture the full gate — the temptation to keep a stale capture after a fix is the one shortcut that would invalidate the whole story |

## Dependencies

**Blocks on** — must be merged before this story can execute at all:

- `page-need-checker-mounted-in-the-gate` — the checker and its five mount points, including the
  `REQUIRED` entry at `xtask/src/main.rs:105`. Without it, `cargo xtask ci` does not exercise the step
  and every capture is void (EC-003). This is the story's only `depends_on` edge, and the story map
  places it at merge position 2.1 to this story's 2.2 ([`../_storymap.md`](../_storymap.md), "Merge
  order").

**Unlocks:**

- `playbook-atom-staged-for-ingest` — names this story in its own `depends_on`, because the playbook
  atom records the conditions under which the discipline stops holding, and those are only true once
  the discipline has been watched working ([`../_storymap.md`](../_storymap.md), Slices table).
- The project's Definition of Done clause *"AC-007's failure observed and recorded in the ledger, not
  merely asserted"* ([`../project.md`](../project.md)), and through it the page-need half of the
  initiative's **DoD scenario 2**.

**Not a dependency, deliberately:** HS-P0020's `checked-documentation-surface` owns the *other* half
of DoD scenario 2 (a claim that is no longer true of the library). This story does not wait on it,
except in the practical sense that EC-001 depends on that project's pages tree carrying content.

## Anchors (progressive disclosure)

Load-bearing depth, deferred rather than pasted. Each row says why it is load-bearing and the moment
to open it; every path was confirmed present in this worktree.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| [`.bklg/docs-that-teach/page-need-discipline/_design.md`](../_design.md) | The signed-off S4 composition, the transience policy, the density budget with its real numbers, the sixteen anti-patterns and finding 1's 112/111 prediction. It is the bar the captures are read against and this story may not amend it | Before capture 1, and again beside the finished captures when judging AC-007/AC-008 | AC-004, AC-007, AC-008 |
| [`.bklg/docs-that-teach/page-need-discipline/_decomposition.md`](../_decomposition.md) | Testing brief AC-007 states the three-step procedure verbatim; UX-007 states the three-breaks-one-run test; UX-008 states the reversibility test; UX brief Note 1 names the vacuous-run failure | At the start, before writing any capture plan | AC-002, AC-003, AC-004, AC-005 |
| `xtask/src/main.rs` | `REQUIRED` at `:105`, dispatch at `:689-700`, `print_help` at `:718`, `lint_steps` at `:799-808`, `steps_named`'s panic at `:816-826`, `run_ci` at `:828-831` — the mount this story exists to prove is live, and the thing EC-003 checks before anything else | At preflight, before capture 0, to confirm the step name is present in `REQUIRED` | AC-001 |
| `xtask/src/lint_constitution.rs` | The inherited diagnostic grammar the captured messages are judged against: report-all-then-bail at `:169-198`, the vacuity `bail!` at `:174-176`, the success line at `:192`, the repair-inside-the-message at `:375-379` | When judging a captured problem line, and when deciding whether a vacuous run is EC-001 | AC-007 |
| `RUNBOOK.md` | `:918-928` is this repository's own record of a gate step that printed `skipped` on all three runners while two documents vouched for it. It is the reason capture is verbatim and the reason a silent green fails AC-008 | Before writing `_ledger.md`, so the capture discipline is motivated rather than mechanical | AC-006 |
| `.redkiln/config.yaml` | `require_ledger: true` at `:67` and `require_commit_provenance: true` at `:73` are what make a recorded procedure first-class proof — and what the story fails if a capture lacks a command or a sha | When authoring the ledger rows and recording the work commit | AC-006 |
| [`.bklg/docs-that-teach/page-need-discipline/page-need-checker-mounted-in-the-gate/spec.md`](../page-need-checker-mounted-in-the-gate/spec.md) | The slice-mate's PR boundary — where every defect this observation surfaces is fixed, and the definition of the step name and task name this story consumes by value | The moment a capture is substandard (EC-004), before touching anything | AC-002, AC-003, AC-007 |
| [`.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md`](../../_discovery/distillation/personas-and-journeys.md) | `:174-181` is the measured in-place failure — an `E0034` explanation that existed in three documents, none of them the file the reader was looking at. It is the standard for "actionable without opening a second document" | When judging whether a captured message is usable, not merely correct | AC-002, AC-003 |
| [`.bklg/docs-that-teach/page-need-discipline/design/mock.html`](../design/mock.html) | The labelled `green`, `single-problem` and `many-problems` frames, with density chips computed from the specimen's own bytes — the only pre-existing picture of what these captures should look like | At the end, once, beside the finished captures | AC-008 |
| `xtask/src/affected.rs` | `:118-125` is the unconditional file-reading list the slice-mate adds the checker to; `:249-260` is `INERT`, which is why this story's `.bklg/`-only diff selects no package | When running the story-grain gate and reading its narrow output | AC-006 |
| [`.bklg/docs-that-teach/page-need-discipline/project.md`](../project.md) | AC-007's own wording ("naming the file and the line"; "both halves are observed") and the DoD clause that forbids asserting it | At the start, and again when flipping the last ledger row | AC-001, AC-006 |
| [`.bklg/docs-that-teach/initiative.md`](../../initiative.md) | DoD scenario 2's exact wording — *"the failing half is the one that matters"* — which is what this story moves halfway to green | When writing the implementation report's outcome | AC-005 |

## Clarifications resolved during spec

1. **Eight criteria, not six.** The first pass planned AC-001 through AC-006. Two more were added
   here — **AC-007** (message grammar, hierarchy and the measured line length) and **AC-008** (the run
   is visible and carries no noise) — because RFC §6.7/D6 requires every applicable composition
   invariant to be a *row in the acceptance table*, not a prose bullet: a bullet gets no ledger row,
   is never gated and is never tested. Packing "the repair is inside the line", "the step names
   itself", "no spinner" and "≤ 100 characters, measured" into AC-002's cell would have produced one
   mushy row nobody could flip honestly. The ledger carries all eight.
2. **The one-line slice names two breaks; the spec observes three.** Break C (a page with no
   declaration at all) is used **only** in the report-all run (AC-004), because UX-007's stated test
   is *three pages, three ways, one run*. It is not a third standalone red capture, and the PR slice
   line is unchanged.
3. **A missing-declaration problem may have no line number.** [`../_design.md`](../_design.md)'s own
   S4 sample prints the third problem as `<PAGE_DIR>/getting-started.md — no > **Answers:** line`,
   file-level with no `:line`, and the mock's finding 4 notes the S4 selector regex is narrower than
   its own precedent's messages. So AC-007 requires `path:line — ` for the two-declaration and
   unenumerated cases and permits the file-level form **only** for the missing-declaration case. This
   is a reading of the signed-off design, not an amendment to it.
4. **The vacuous-tree question is a precondition, not a workaround.** Resolved as EC-001: halt and
   record the block. Rejected: authoring a page into HS-P0020's tree (crosses the project's scope
   boundary), and leaving a "throwaway" fixture behind (a throwaway that must survive the revert is
   not a throwaway — [`../_decomposition.md`](../_decomposition.md), Testing brief, Notes).
5. **`cargo xtask ci --fast` does exercise the step**, since `--fast` runs `REQUIRED` only. It is
   therefore a legitimate iteration bar and is the project's own interim DoD command — but the
   captures of record remain the full `cargo xtask ci`, per Decision 2, so nothing turns on whether
   `OPTIONAL` steps ran.
6. **The ≤ 100-character budget is measured and routed, never amended.** [`../_design.md`](../_design.md)
   predicts 112 and 111 against a 100 budget in its own finding 1. AC-007 records the real number on
   real paths; the amendment — to the budget, the citation form, or the no-wrap rule — belongs to a
   design change with its own gate, and the design carries a human sign-off dated 2026-08-17.
7. **A `#[test]` that shells out to `cargo xtask ci` was considered and rejected** as a substitute for
   the recorded procedure. It would satisfy a runner and not a reader: the same claim one indirection
   further from the human who has to believe it, and it would let a green suite stand in for the
   observation the project DoD demands be *recorded* (Decision 1).
8. **Colour is disabled in captures (NF-003).** Not stated in any brief; resolved here because the
   ledger's evidence is read in a pager and a diff, and ANSI escapes would make the captured problem
   lines unreadable in exactly the medium that matters — and would corrupt the `awk` length
   measurement AC-007 depends on.
