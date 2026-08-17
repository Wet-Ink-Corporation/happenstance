---
item: HS-S0139
stage: spec
created: 2026-08-17T13:16:02.729Z
updated: 2026-08-17T13:16:02.729Z
template_sig: 87bbf1d0
rendered_sig: 2a018aaa
---

# Spec — No fence opts out of the check unnoticed

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/docs-that-teach/initiative.md` |
| Initiative decomposition | `.bklg/docs-that-teach/_decomposition.md` |
| Project charter | `.bklg/docs-that-teach/checked-documentation-surface/project.md` |
| This spec | `.bklg/docs-that-teach/checked-documentation-surface/fence-discipline-and-allowance-list/spec.md` |
| Key briefs | `.bklg/docs-that-teach/checked-documentation-surface/_decomposition.md` — architecture Notes 4, 5, 10; testing brief's AC-004 paragraph; UX brief's journey row for an opted-out fence |
| Signed-off design (**binding**) | `.bklg/docs-that-teach/checked-documentation-surface/_design.md` — `## Items`, `## Signatures`, `## Composition`, `## States`, `## Anti-patterns`. Approved by the repository owner 2026-08-17 |
| Story map row + merge order | `.bklg/docs-that-teach/checked-documentation-surface/_storymap.md` (row 4, milestone `narrative-checker-discipline`) |
| Roadmap pointer | `RUNBOOK.md` is the plan of record for the workspace; this initiative's own order is `_storymap.md` `## Merge order` |

Traces to project **AC-004** (`project.md`, "No silent opt-out"). Depends on
`narrative-checker-mounted-with-pinned-path` (HS-S0138). Blocks
`hidden-content-resolution` (HS-S0140) and `documented-blind-spots-and-their-proofs`
(HS-S0145).

## One-line PR slice

Walk every fence in the tree: reject untagged fences, enumerate info-string parts
exhaustively so an unrecognised part is a hard error, reject `ignore`-class fences unless
they appear on an enumerated allowance `const`, and sweep that list in reverse so a stale
allowance is itself a problem.

## Executive summary

`narrative-checker-mounted-with-pinned-path` lands the checker: the module, the pinned
`TREE`/`HARNESS` constants, the empty-tree `bail!`, the bidirectional page↔harness
registration check, and the `REQUIRED` step, subcommand, help line and `lint_steps`
membership that make it run. **This PR adds the per-fence pass inside that module** and
nothing else in the mount: four checks plus one reverse sweep, pushing onto the same
`Vec<String>` the module already reports through one `bail!`.

The delta over the existing precedent (`check_fences`,
`xtask/src/lint_constitution.rs:601-673`) is exactly one rule and one obligation. The rule:
an `ignore`-class fence is permitted only by an entry in `IGNORE_ALLOWANCES`, not by an
`<!-- ignore: … -->` comment above it, and the list is swept in reverse so an entry naming
a fence that no longer exists is itself a problem. The obligation: because that is a second
`ignore` rule in one repository, the module's own docs must say why the narrative tree gets
the stricter shape.

Nothing here adds a gate step, a dependency, a public item or a conformance rule. What it
adds is failures the gate did not previously have.

## Context pack

The decisions this story must honour, stated as decisions. Deeper material is behind the
signposted anchors the second pass adds — do not go looking for it before it is needed.

**The allowance list is a `const`, and that is a deliberate divergence from precedent.**
`check_fences` today permits `ignore` when the line above is `<!-- ignore: <reason> -->`
(`xtask/src/lint_constitution.rs:638-643`). AC-004 asks for something different and the
architecture brief takes the decision (Note 5): a `const` of `(page-relative path,
line-or-anchor, reason)` tuples in the checker, because an inline comment is reviewable only
in the diff that introduced it while a `const` array is one place a reviewer reads in full
and a later reader audits without a `git log` — and because a stale entry in a list is
detectable, where a stale comment is not. `_design.md` `## Signatures` fixes the type and
the initial value: `const IGNORE_ALLOWANCES: &[(&str, &str, &str)] = &[]`. It starts empty.
Growing it is a review event, never a repair for a failing gate.

**The divergence is owed a sentence, in the module's own docs, in this PR.** Note 5 is
explicit: without it "the next contributor reads two different `ignore` rules in one
repository and assumes one is a mistake." Nothing accepted is contradicted —
`standards/rust/` conventions are not ADRs and `lint_constitution.rs` is not edited by this
project — so the discharge is documentation, not an amendment
(`_storymap.md`, "Stories this map deliberately does not contain": an ADR is not one of
them).

**Everything else about `check_fences` is kept, and kept for its stated reasons.** An
untagged fence is rejected because rustdoc compiles it as Rust anyway
(`lint_constitution.rs:609-619`). The info-string parts are matched **exhaustively** so an
unrecognised part is a hard error rather than a silent pass (`:621-637`) — this is the
story's named wrong implementation, from `discover.md`: *a fence walk that recognises the
info-strings it knows and silently ignores the rest, so a novel opt-out spelling passes
unnoticed.* `ignore` has already returned by the back door once in this repository's own
testkit doctests (`project.md` risk table). RS-81-2 is the constitution's form of the same
posture: a construct the scanner cannot read is an error, because the alternative failure is
that every match silently stops reporting.

**Copy the shape; do not refactor `lint_constitution.rs` to share it.** RS-81-3 scopes a
scanner to the directory whose behaviour it constrains, and a shared abstraction makes one
error message answer two questions (`_storymap.md`, "A refactor of `lint_constitution.rs`
… " under stories deliberately not contained). The two `ignore` rules are *supposed* to
differ, so a shared helper would have to be parameterised by the difference — which is the
whole rule.

**The reverse sweep is the half that earns the list.** `check_harness` documents the same
asymmetry it exists for (`lint_constitution.rs:445-446`): the forward direction catches a
page nobody registered; the reverse catches a registration nobody deleted. Here the forward
direction catches a fence nobody approved; the reverse catches an approval for a fence that
is gone. A checker that never sweeps is the named wrong implementation for that check, and
the testing brief says so (`_decomposition.md`, testing brief, AC-004 item (d)).

**Four backticks are not a fence.** The precedent parser steps over an entire quoted block
because the corpus quotes fenced templates inside one, and reading the inner fence as an
example would compile prose as Rust (`lint_constitution.rs:293-307`, with the regression
test `four_backtick_fences_are_not_examples` at `:862-868`). This is not a nicety here: the
fixture page this project's gate compiles is itself written out inside a four-backtick
`markdown` block in `_design.md` `## The doctest`. A walk without that behaviour flags quoted material and
teaches contributors that the checker cries wolf.

**One pass, one accumulator, one report — and this story does not get a step.**
`_design.md` `## Composition` fixes the output: the banner
`=== every narrative page is checked ===`, then every problem in source order as
`  {path}:{line} — {message}` on stderr with a two-space indent, then
`bail!("{n} problem(s) in {TREE}")` last, because the count is the only line that stays in
view when the list is long. `lint_constitution::run` is the working shape
(`:169-198`); a check that stops at the first problem turns one review cycle into six
(`_decomposition.md:218-219`). Anti-pattern 8 forbids `… and N more`. Adding a *second*
step or banner for fence discipline would make one name answer two questions
(architecture brief Note 3) — the mount is the step story HS-S0138 already wired.

**Leave room for the slice-mate in the same walk.** `hidden-content-resolution` adds
`HIDDEN_MARKERS` to *this* fence walk, by design (`_design.md` D2: "in the same fence walk
as AC-004"; `_storymap.md`, why `narrative-checker-discipline` is one surface). Structure the
pass so that lands as another check over the same parsed fences, not a second traversal of
the tree.

**The back door stays open, and this PR is where that is written down.** A Rust example
deliberately tagged `text` is neither compiled nor flagged: the allowance list narrows that
hole and does not close it (`_decomposition.md`, Note 10 item 5). `documented-blind-spots-and-their-proofs`
owns the *contents* of the limits section; every story owes the section's existence for what
it adds, and it goes **first** in the module docs, not last, because a check whose limits are
undocumented is read as a guarantee (`lint_constitution.rs:9-13`; project DoD item 7;
RS-81-1).

**Zero new dependencies.** DR-12's standing trade at `xtask/Cargo.toml:16-21` keeps drivers
out of xtask's dev graph because every `cargo xtask ci` would build them. A fence walk is a
line scan; it needs nothing.

**The persona-journey slice.** The user is a contributor running the gate and a reviewer
reading its output (`_storymap.md` preamble). The journey row this story realises is
*meeting an opted-out fence*: the contributor's outcome is to **know that an uncompiled
block is uncompiled** — either it is rejected by file and line, or it appears on a list a
human approved with a reason attached (`_decomposition.md:471`). Neither branch is silence.

**And the standing boundary.** Nothing in this story asserts anywhere that a checked fence
means the page teaches (project DoD item 8). This walk proves that code inside prose was
offered to the compiler. That is all it proves.

## Integration contract

- **Archetype**: `capability` — observable end to end through `cargo xtask ci`,
  `cargo xtask ci --fast`, `cargo xtask affected --base main` and the `narrative`
  subcommand.
- **Slice / milestone**: `narrative-checker-discipline`. Slice-mates, implemented in one
  context and mounted as one surface: `narrative-checker-mounted-with-pinned-path`
  (lands first, creates the module) and `hidden-content-resolution` (lands after, extends
  this walk).
- **Mount point**: the bin-crate narrative checker module's `run()` fence walk — the module
  `narrative-checker-mounted-with-pinned-path` creates and declares from
  `xtask/src/main.rs:64-70`, beside `mod lint_constitution;` (`xtask/src/main.rs:65`), and
  which `cargo xtask narrative` dispatches (`_design.md` `## Surfaces`,
  `gate-narrative-checker-step`; `## Placement and re-export`). This story adds checks
  *inside* that module's existing pass and adds no module, step or subcommand of its own.
  **This is not delivered as an isolated function**: the acceptance evidence is problems
  appearing in that step's output under the pinned tree, with the unit tests as the grain
  beneath it.
- **Wires into**:
  - `xtask/src/lint_constitution.rs:601-673` — `check_fences`, the shape being copied, and
    `:288-330` — the `fences()` parser, including the four-backtick behaviour and the
    `preceding` line the divergent rule stops needing.
  - `xtask/src/lint_constitution.rs:169-198` — the accumulate-all / report-all / count-last
    contract the new problems join.
  - `xtask/src/lint_constitution.rs:423-458` — `check_harness`, the reverse-sweep precedent
    (`:445-446` states the asymmetry).
  - `TREE` and `IGNORE_ALLOWANCES` from `_design.md` `## Signatures`; `TREE` is `"docs"`.
  - `xtask/src/main.rs:105` (`REQUIRED`), `:799-826` (`lint_steps`, `steps_named`) and
    `xtask/src/affected.rs:116-125` (the unconditionally-run file-reading lints) — consumed,
    not modified: HS-S0138 owns those edits, and this story's checks reach all three
    invocation paths through them.
- **Renders surfaces** (from `_design.md` `## Surfaces`): `gate-narrative-checker-step` —
  states `fail-one`, `fail-many` and `pass` (the one-line success summary). It renders no
  markdown surface and adds no page. `fail-hidden-marker` belongs to
  `hidden-content-resolution`, in this same walk.
- **Public items** (`_design.md` `## Items`): `xtask::narrative::IGNORE_ALLOWANCES` — added
  here, private `const`, no semver promise, grows only by review
  (`## Visibility and stability`). `TREE` is consumed, not added.
- **Conformance rule(s)**: **none, and deliberately.** This behaviour is not
  adapter-observable: it constrains a documentation tree, not a store. No rule id in
  `crates/happenstance-testkit/src/suite.rs` changes, no port changes, and the testkit's
  changelog obligation (CF-29) is not engaged because no rule is added.
- **Clause(s)**: none. `spec/SPECIFICATION.md` is not read, amended or discharged by this
  story, and no `[FROZEN]` clause is touched. Clause-id resolution is the
  `specification-pin` milestone's (`narrative-citation-resolution`).
- **Advances DoD scenario**: initiative **DoD-2** — *a deliberately broken page fails the
  gate, by name*. This story does not perform that observation (`observed-failure-falsification`
  does, and `project.md` AC-003 owns it); it is what makes the observation mean something,
  because a tree in which any fence may opt out silently is a tree where DoD-2 can be
  satisfied and evaded in the same commit.

## PR boundary

```
xtask/src/**
.bklg/docs-that-teach/checked-documentation-surface/fence-discipline-and-allowance-list/**
```

**In this PR**

- The per-fence walk inside the checker module: untagged rejection, exhaustive info-string
  enumeration, the `IGNORE_ALLOWANCES` gate, and the reverse sweep over that list.
- `IGNORE_ALLOWANCES` itself, empty, with the tuple meaning documented on the `const`.
- The fence parser this walk needs, including the four-backtick step-over, with its
  regression test.
- The module-doc additions this story owes: the deliberate-divergence sentence, and the
  `text`-tag limit under the module's "What this does not verify" heading.
- Unit tests in the module's own `#[cfg(test)] mod tests`, in the shape
  `lint_constitution.rs:827-878` already uses.

**Explicitly not in this PR**

- The checker module's creation, its constants other than `IGNORE_ALLOWANCES`, the
  empty-tree `bail!`, the page↔harness registration check, and the `REQUIRED` step,
  dispatch arm, help line, `lint_steps` membership and `affected` unconditional-list entry
  — all `narrative-checker-mounted-with-pinned-path`'s (HS-S0138).
- `HIDDEN_MARKERS` and DT-7's enforcement — `hidden-content-resolution` (HS-S0140), landing
  in this same walk afterwards.
- Any edit to `xtask/src/lint_constitution.rs`, and any refactor to share code with it.
- Any narrative page, fixture or corpus content under `docs/`; any `docs/README.md` row.
- Clause-id resolution, the frozen-MUST pin, the `affected` selection arm, and the recorded
  falsification runs.
- The contents of the limits list (six items) — `documented-blind-spots-and-their-proofs`.

**Merge DoD**: `cargo xtask ci --fast` green, the new unit tests failing before the walk
exists and passing after, and the fence checks visible in
`cargo xtask narrative`'s output on a page carrying each defect — not only in a test.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| Untagged fence is rejected | A fence whose info string is empty is a problem: rustdoc compiles it as Rust regardless, so silence here means prose is compiled by accident or a Rust example is compiled without anyone deciding it should be. Message keeps the existing wording — *an untagged fence is compiled as Rust; tag it `rust` or `text`* | `xtask/src/lint_constitution.rs:609-619` |
| Info-string parts are matched exhaustively | Split the info string after the leading `rust`, then match every remaining part against a closed set; a part that matches nothing is a problem naming the whole info string. No wildcard arm that accepts. This is what makes a novel opt-out spelling a failure rather than a pass | `xtask/src/lint_constitution.rs:621-637`; `standards/rust/81-checks-that-cannot-be-types.md:95-104` (RS-81-2) |
| `ignore`-class fence needs an allowance entry | An `ignore` part is permitted only when `IGNORE_ALLOWANCES` carries an entry matching that fence's page path and its line-or-anchor. Otherwise a problem naming file and line. The `<!-- ignore: … -->` comment form is **not** accepted under the narrative tree | `_decomposition.md` architecture Note 5; `_design.md` `## Signatures` |
| The allowance list is swept in reverse | Every entry is matched against the fences actually present; an entry naming a fence that no longer exists is itself a problem, so the list cannot accumulate permission for code nobody has | `xtask/src/lint_constitution.rs:445-446`; `_decomposition.md` testing brief, AC-004 item (d) |
| The allowance tuple carries a reason, and the reason is prose | `(page-relative path, line-or-anchor, reason)`. The checker does not interpret the reason; it exists so a reviewer reading the `const` in full knows what was approved and why. An entry with an empty reason is a problem | `_design.md` `## Signatures`; `_decomposition.md` Note 5 |
| Error-code rules are carried over unchanged | An error code on a fence that is not `compile_fail` is a problem; a `compile_fail` naming a code the page's prose never mentions is a problem, because rustdoc accepts a `compile_fail` whose code never matches | `xtask/src/lint_constitution.rs:644-660` |
| Four-backtick blocks are stepped over entirely | A four-backtick block quotes fenced material for display; its inner fences are not examples and must not be flagged. The design's own fixture page is written inside one | `xtask/src/lint_constitution.rs:293-307`, `:862-868` |
| Problems accumulate; nothing short-circuits | Every fence in every page is walked before anything is reported. Problems print in source order (path, then line), two-space indent, on stderr; the count and the tree name come last via `bail!` | `xtask/src/lint_constitution.rs:169-198`; `_design.md` `## Composition` |
| No truncation of the problem list | Forty problems print as forty lines. No `… and N more` | `_design.md` `## Anti-patterns` item 8, `## States` (overflow) |
| Success stays one line | The step's green output remains the single summary line the module already prints; this story adds no per-page or per-fence chatter on the happy path | `_design.md` `## Transience policy`, `## States` (loading) |
| Location prefix budget is respected, not re-implemented | Problem lines are `{path}:{line} — {message}` with the location first, so a long message may soft-wrap but never pushes the location off the first row. The path-length gate rule itself belongs to the pinning check | `_design.md` `## Density budget` (terminal surface), `## Hierarchy` |
| The divergence is documented where it can be read | The module's `//!` docs state that the narrative tree uses an enumerated allowance list rather than `lint_constitution`'s comment form, and why | `_decomposition.md` Note 5; `xtask/src/lint_constitution.rs:9-44` for the shape |
| The `text` back door is stated as a limit | Under the module's "What this does not verify" heading, placed first: a Rust example deliberately tagged `text` is neither compiled nor flagged; this list narrows that hole and does not close it | `_decomposition.md` Note 10 item 5; `standards/rust/81-checks-that-cannot-be-types.md:11` (RS-81-1) |
| No new dependency, no new step, no public API | The walk is a line scan over already-read files inside an existing step. `xtask` stays `publish = false`; nothing becomes `pub` | `xtask/Cargo.toml:16-21`; `_design.md` `## Visibility and stability`, `## What it costs a caller` |

## Data and migrations

**N/A.** This story adds no persistent data, no schema, no serialised format and no
migration. The only new state is a compile-time `const` array in a `publish = false` binary
crate (`IGNORE_ALLOWANCES`, initially empty), which has no on-disk representation and no
consumer outside the module that declares it. No store, no projection, no checkpoint and no
event payload is touched, so the append-condition and position-visibility invariants are
untouched by construction.

## Acceptance criteria

Seven criteria. The first five are the walk; the last two are the surface it renders and the
record it owes. Every one is framed from the intent of a real person in this initiative's
audience — the **contributor running the gate** and the **reviewer reading its output**
(`_storymap.md` preamble), standing in for the **application author** whose trust in a fenced
example is the thing being protected (`.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md`,
`## Persona 1 — The application author`; `_decomposition.md` UX brief, "Reader states this
project must make true", row *Meeting an opted-out fence*). Each row is one line on purpose:
`redkiln verify` extracts an AC by matching the leading `| AC-001 |` cell.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | GIVEN an application author reading a fenced example and trusting it still compiles against the crate they installed, WHEN a contributor adds a fence with an empty info string to a page under `TREE`, THEN `cargo xtask narrative` reports `{TREE}/{page}:{line} — an untagged fence is compiled as Rust; tag it \`rust\` or \`text\`` and the gate fails — because rustdoc compiles an untagged fence as Rust regardless, so silence here means either prose is compiled by accident or Rust is compiled that nobody decided to check. | Unit test `an_untagged_fence_is_rejected` in the checker module's `#[cfg(test)] mod tests` (shape: `xtask/src/lint_constitution.rs:827-878`), asserting on the message text and the reported line; plus the same defect observed in `cargo xtask narrative`'s output on a fixture page, not only in a test. |
| AC-002 | GIVEN a reviewer who cannot be expected to know every spelling rustdoc accepts, WHEN a fence's info string carries any part the walk does not enumerate — `ignore_me`, `norun`, a future rustdoc attribute, or an error code on a fence that is not `compile_fail` — THEN the walk names the whole info string as unrecognised and the gate fails, because the closed match has no accepting wildcard arm; a novel opt-out spelling is therefore a hard error and never a silent pass. | Unit tests `an_unrecognised_info_string_part_is_a_problem` and `an_error_code_without_compile_fail_is_a_problem`, each constructed from a plausible novel spelling rather than a known one (RS-81-2's posture, `standards/rust/81-checks-that-cannot-be-types.md:95-104`); a reviewer check that the `match` carries no arm that accepts an unknown part. |
| AC-003 | GIVEN a reviewer who must be able to read, in one place and in full, every block on the narrative tree that no compiler read, WHEN a fence carries an `ignore`-class part and no `IGNORE_ALLOWANCES` entry names it, THEN it is a problem naming file and line and an `<!-- ignore: … -->` comment above it does not rescue it; and WHEN an entry does name that fence and carries a non-empty reason, THEN the fence passes and the walk reports nothing else about the page. | Unit tests `an_unlisted_ignore_fence_is_rejected` (naming file and line), `a_comment_above_an_ignore_fence_does_not_permit_it` — the named wrong implementation from `_decomposition.md` testing brief item (b) — `a_listed_ignore_fence_passes`, and `an_allowance_with_an_empty_reason_is_a_problem`. |
| AC-004 | GIVEN a reviewer six months later auditing what the tree is still allowed to skip, WHEN an `IGNORE_ALLOWANCES` entry names a page or fence that no longer exists, or names a fence that is no longer `ignore`-class, THEN that entry is itself a problem naming the entry, so the list cannot accumulate standing permission for code nobody has. | Unit tests `a_stale_allowance_is_a_problem` and `an_allowance_for_a_fence_that_no_longer_opts_out_is_a_problem`, mirroring the reverse half of `check_harness` (`xtask/src/lint_constitution.rs:445-446`, which states the asymmetry) and discharging `_decomposition.md` testing brief item (d). |
| AC-005 | GIVEN a contributor who will keep reading the gate's output only while it is right about their tree, WHEN a page displays fenced material inside a four-backtick block — the shape a page teaching how to tag a fence must use — THEN the walk steps over the entire block and reports nothing for the fences inside it; and every problem it does report names the page's own path and the line of the offending fence, so the reader opens the file the message names. | Unit test `four_backtick_fences_are_not_examples`, carried over from `xtask/src/lint_constitution.rs:862-868` with the narrative tree's paths, plus `a_problem_names_the_page_and_the_fence_line` asserting the location prefix is the page path and not the harness. |
| AC-006 | GIVEN a contributor reading a CI log after the step failed, WHEN the walk finds n problems anywhere in the tree, THEN all n print — never truncated, never `… and N more`, never a bare non-zero exit — under the step's banner on stderr in source order (path, then line) as `  {path}:{line} — {message}` with the location first and inside the ≤ 48-character prefix budget, and the count last as `bail!("{n} problem(s) in {TREE}")`; the output is append-only plain text with no screen clear, no cursor rewrite, no pager and no colour-only or TTY-only distinction, so the log reads identically to the terminal and the banner and earlier steps stay on screen; and the walk writes no file, so repairing the fence or adding an allowance returns the same command to green with nothing to clear. | Unit tests `problems_are_reported_in_source_order` and `every_problem_is_reported_not_the_first` over a multi-defect fixture; `cargo xtask narrative` run on a fixture page carrying three distinct defects, with its output compared against `_design.md` `## Composition`'s block; a reviewer check that the walk contains no ANSI escape, no `std::fs::write`, and no early `return` on the first problem. |
| AC-007 | GIVEN a contributor who must not read this green step as more than it is, WHEN the walk finds nothing, THEN the step adds no per-page or per-fence output — one summary line, no spinner, no progress ticker — and WHEN that contributor opens the checker module, THEN its `//!` docs state, *before* the checks, that a Rust example deliberately tagged `text` is neither compiled nor flagged and that this list narrows that hole rather than closing it, and that the narrative tree deliberately diverges from `lint_constitution`'s `<!-- ignore: … -->` form together with why; and nothing this story adds claims anywhere that a checked fence means the page teaches. | Unit test `the_module_docs_state_the_text_limit_and_the_divergence`, reading the checker module's own source with `include_str!` (the technique `xtask/src/constitution.rs` already uses to pull text into the crate) and asserting both sentences are present and precede the first check; observed one-line green output from `cargo xtask narrative`; reviewer check against project DoD item 8 (`project.md:260`). |

Project **AC-004** ("No silent opt-out", `project.md:212-214`) is discharged by AC-001 through
AC-005 together: AC-001 and AC-002 close the untagged and novel-spelling routes, AC-003 is the
enumerated allowance list the project AC names in its own text, AC-004 is what keeps that list
honest, and AC-005 is what stops the check being abandoned for crying wolf. No other project AC
is traced here; AC-006 and AC-007 discharge the two obligations `_storymap.md` `## Coverage`
assigns to *every* story rather than to one — DoD item 7's "what this does not verify" section
for what this story adds, and DoD item 8's standing prohibition.

## Interaction quality

The surface this story renders is `gate-narrative-checker-step` (`_design.md` `## Surfaces`),
in the states `fail-one`, `fail-many` and `pass`. Its composition is already approved and is
**not re-decided here**. Every invariant below is carried by an AC row in the table above —
this section says which row carries which and how it is verified, and adds nothing that is not
gated there.

**State invariants.** The medium is a terminal and a CI log, so each invariant has a terminal
form, and the terminal form is the one that is checked.

| Invariant | Terminal form | Carried by | How it is verified |
| --- | --- | --- | --- |
| In-place, not a context jump | The problems appear in the failing step's own output stream. There is no artifact to open, no second log to fetch, no `--verbose` re-run required to see which fence failed. | AC-006 | `cargo xtask narrative` on a three-defect fixture: the three problems are in the same output as the banner. |
| Non-occlusion | Output is append-only. No screen clear, no cursor movement, no in-place progress rewrite — the banner, the earlier steps and the reader's scrollback all survive the failure. | AC-006 | Reviewer check for ANSI escapes and carriage returns; the log and the interactive run compared. |
| Preserved focus / scroll / selection | The scrollback *is* the reader's scroll position, and nothing rewrites it. A reader who scrolled up to problem 1 while problem 40 printed does not lose their place. | AC-006 | Same as non-occlusion: an append-only stream cannot move the viewport. |
| Reversibility | The walk is read-only. It writes no file and keeps no cache, so tagging the fence or adding the allowance entry and re-running the same command is the whole undo. | AC-006 | Reviewer check that the walk performs no write; the fix-and-rerun cycle exercised while iterating. |
| Keyboard reachability | Every problem is reachable without an interactive gesture: plain text on stderr, no pager, no prompt, no TTY requirement, no colour-only distinction. Piping to a file loses nothing. | AC-006 | `cargo xtask narrative 2> file` carries the same bytes the terminal showed. |

**Composition invariants**, from the signed-off `_design.md`. Real numbers, and its named
anti-patterns.

| Invariant | The design's requirement | Carried by | How it is verified |
| --- | --- | --- | --- |
| Presentation exists at all | A problem is a composed line — two-space indent, `{path}:{line}`, the em dash separator, then the message (`## Composition`; the primitive at `_design.md:37`). Never a `Debug` dump, never a bare non-zero exit (`## States`, Error row). | AC-006 | Output compared against `_design.md` `## Composition`'s block, line shape included. |
| Composition and placement | Banner, then every problem in source order, then the count. The count carries the tree name because it is the line that stays in view (`## Composition`, third rule). | AC-006 | `problems_are_reported_in_source_order`; the `bail!` string asserted. |
| Transience | Problem lines are **opened on demand, by failing** — they exist only in the failure state. The success summary is **persistent chrome, one line**. No spinner, no dot ticker, no per-page progress (`## Transience policy`; `## States`, Loading row: "**Do not add**"). | AC-007 | Green run of `cargo xtask narrative` produces exactly one added line; reviewer check for progress output. |
| Density budget | 80-column log line; location prefix **≤ 48 characters** including `:{line}` and inclusive of the 16-character doctest prefix, i.e. ≤ 32 characters repo-relative (`## Density budget`, terminal table, and mock finding 3's disposition); problem list **unbounded**; success output **1 line**. | AC-006, AC-007 | The location prefix asserted in `a_problem_names_the_page_and_the_fence_line`; the unbounded list in `every_problem_is_reported_not_the_first`. |
| Hierarchy | `{path}:{line}` is primary, carried by being first on the line; the message is secondary, after the em dash; the indent, banner and count are recessive (`## Hierarchy`, third block). | AC-005, AC-006 | The message format asserted character-for-character in the unit tests. |
| Anti-pattern 5 | No fence with no language tag, and none visibly marked `ignore` with no allowance entry. | AC-001, AC-003 | The two rejection tests. |
| Anti-pattern 7 | No gate failure whose first visual row does not begin with `path:line`. | AC-006 | Output shape assertion; the location is first by construction. |
| Anti-pattern 8 | No truncated problem list — no `… and N more`. | AC-006 | `every_problem_is_reported_not_the_first`. |
| Anti-pattern 9 | No badge, tick, shield or "verified" mark asserting the documentation is checked for correctness or comprehension. | AC-007 | The docs test plus the DoD item 8 reviewer check. |

Two composition decisions are **consumed, not made here**: the banner text
`=== every narrative page is checked ===` (mock finding 6's disposition) and the one-line green
summary are `narrative-checker-mounted-with-pinned-path`'s to emit. This story adds problems to
that surface; it does not restyle it. The `fail-hidden-marker` state of the same surface belongs
to `hidden-content-resolution`.

## Error conditions

| id | Condition | Required behaviour |
| --- | --- | --- |
| EC-001 | A page under `TREE` cannot be read — permissions, or bytes that are not UTF-8. | Propagate with `.with_context(|| format!("reading {path}"))` and fail the step. Never treat an unreadable page as a page with zero fences: that is the silent-pass shape `xtask/src/lint_constitution.rs:29-44` argues against in the citation parser's own words — "a citation this parser declines to read is a citation nothing verifies" — and the same reasoning transfers to a fence. |
| EC-002 | A fence is opened and never closed before end of file. | A problem naming the page and the opening line. The precedent parser drops an unpaired opener on the floor (`lint_constitution.rs:288-330`: the final `open` is never pushed), which means its info string is never examined — an opt-out route this story would otherwise inherit. Fix it in the copy; do not fix it in `lint_constitution.rs`. |
| EC-003 | An `IGNORE_ALLOWANCES` entry is malformed: an empty reason, a path that is not under `TREE`, or a line-or-anchor that matches no possible fence position. | A problem naming the entry by its tuple contents, reported from the reverse sweep. A malformed entry must never be silently inert — an inert entry looks like a granted permission to the next reader. |
| EC-004 | Two entries name the same fence. | A problem naming both. Otherwise deleting one leaves the other silently authorising the fence, and the reverse sweep reports neither as stale. |
| EC-005 | A line-keyed entry now names a fence that exists but is not `ignore`-class. | A problem: the permission is stale in the way most likely to occur, because inserting a paragraph above a fence shifts every line-keyed entry below it. This is the case that makes the anchor form the recommended one. |
| EC-006 | A page's markdown is pathological — nested backticks, a fence marker inside an indented block, CRLF line endings. | The walk reports a problem or reports nothing, but never panics. No `unwrap`, no `expect`, no slicing by byte offset that can split a char boundary; `#![allow(clippy::unwrap_used)]` stays scoped to `#[cfg(test)]` as at `lint_constitution.rs:829`. A gate that panics is a gate whose output nobody can read. |

## Non-functional

| id | Requirement | Why, and where it is fixed |
| --- | --- | --- |
| NF-001 | Zero new dependencies; `xtask`'s dependency and dev-dependency graph is byte-identical after this PR. | DR-12's standing trade at `xtask/Cargo.toml:16-21` — every `cargo xtask ci` builds this crate's graph. A line scan needs nothing (`_design.md` `## What it costs a caller`). |
| NF-002 | No second traversal of the tree. The walk consumes the page text the module already read, and adds no `read_dir`. | `_design.md` `## What it costs a caller`: the whole added cost is a `read_dir` and a line scan, the class `xtask/src/affected.rs:28-36` argues finishes inside the time cargo takes to decide `xtask` is up to date. It is also what leaves room for `HIDDEN_MARKERS` in the same pass. |
| NF-003 | Deterministic order and byte-identical reruns: problems sorted by page path then line, independent of filesystem iteration order. | Source order is a design requirement (`_design.md` `## Composition`), and a CI log that reorders between runs cannot be diffed. |
| NF-004 | No `unwrap`, `expect` or panic in the walk; scoped `allow`s only inside `#[cfg(test)]`, each with a `reason`. | House style, and the precedent's own scoping at `xtask/src/lint_constitution.rs:829`. |
| NF-005 | Clippy clean under `-D warnings` with the workspace's pedantic set, and `cargo fmt` clean. | `cargo xtask ci`'s first two steps; the gate is the bar, not a follow-up. |
| NF-006 | MSRV, `wasm32`, features and public API unaffected: no `cfg`, no feature gate, nothing becomes `pub`, `xtask` stays `publish = false`. | `_design.md` `## Visibility and stability` and `## What it costs a caller`. ADR-0001's `Send` question and ADR-0029's floor are untouched by construction — there is no trait, no future and no dependency here. |
| NF-007 | Path comparison is separator-normalised: an allowance entry written `docs/adapters/sqlite.md` matches the same page discovered on Windows. | This repository is developed on Windows and `lint_constitution.rs` composes its locations with `/` literals throughout (`:607`). An allowance list that silently matches nothing on one platform is a list that grants permission on one runner and denies it on another. |

## Implementation notes (non-prescriptive)

- **Shape the walk as a pure function over `(page path, page text) -> Vec<String>`.** The
  module's `run()` then folds it over the pages it already read. This is what makes
  `hidden-content-resolution` an added check inside one loop rather than a second traversal,
  and what lets every unit test above be a string slice instead of a temp directory.
- **Copy `fences()` rather than call it.** RS-81-3 scopes a scanner to the tree it constrains
  (`standards/rust/81-checks-that-cannot-be-types.md`), and the two `ignore` rules are
  *supposed* to differ — a shared helper would have to be parameterised by exactly the
  difference this story exists to introduce. The copy can drop the `preceding` field the
  precedent keeps for the comment rule (`lint_constitution.rs:161-162`); if the module story
  already carried it, delete it here rather than leaving a field clippy will call dead.
- **Keep the info-string match closed.** The precedent's `_ => ok = false` arm
  (`lint_constitution.rs:632`) is the whole mechanism of AC-002. A `_ => {}` there is the named
  wrong implementation, and it is one character away.
- **Match an allowance once and record that it was used.** A `Vec<bool>` (or a `BTreeSet` of
  indices) filled during the forward pass gives the reverse sweep for free at the end of the
  same walk, and gives EC-004's duplicate detection as a by-product.
- **Prefer the anchor form when documenting the tuple.** Both halves of "line-or-anchor" must
  work, but the `const`'s own comment should say that a line number goes stale on any
  insertion above the fence and an anchor does not — the reverse sweep will report the drift
  either way, and an author who reads that comment first will not have to learn it from a
  failure.
- **Write the module-doc sentences as prose a contributor reads, not as a checklist entry.**
  `xtask/src/lint_constitution.rs:9-44` and `xtask/src/constitution.rs:1-44` are the register:
  each limit states the mechanism, not just its name.
- **`IGNORE_ALLOWANCES` ships empty and stays empty in this PR.** If a fence in a fixture page
  needs an entry to make the tests meaningful, construct it in the test, not in the `const`.

## Tests and CI (merge gate)

Tiers are the testing brief's four (`_decomposition.md` `## Testing brief`, Acceptance
Criteria preamble), and the commands are its own merge-gate list in the order a contributor
runs them, narrowest first.

| tier | command / path | proves |
| --- | --- | --- |
| static | `cargo test -p xtask` → `xtask/src/lint_narrative.rs`, its `#[cfg(test)] mod tests` | AC-001 through AC-005 and AC-007's docs half: `an_untagged_fence_is_rejected`, `an_unrecognised_info_string_part_is_a_problem`, `an_error_code_without_compile_fail_is_a_problem`, `an_unlisted_ignore_fence_is_rejected`, `a_comment_above_an_ignore_fence_does_not_permit_it`, `a_listed_ignore_fence_passes`, `an_allowance_with_an_empty_reason_is_a_problem`, `a_stale_allowance_is_a_problem`, `an_allowance_for_a_fence_that_no_longer_opts_out_is_a_problem`, `four_backtick_fences_are_not_examples`, `a_problem_names_the_page_and_the_fence_line`, `the_module_docs_state_the_text_limit_and_the_divergence` |
| static | `cargo test -p xtask` → `xtask/src/lint_narrative.rs`, the same module | AC-006's ordering and completeness, and the error conditions: `problems_are_reported_in_source_order`, `every_problem_is_reported_not_the_first`, `an_unterminated_fence_is_a_problem` (EC-002), `a_duplicate_allowance_is_a_problem` (EC-004), `a_malformed_allowance_is_a_problem` (EC-003) |
| gate-integration | `cargo xtask narrative` on a fixture page carrying an untagged fence, an unlisted `ignore`, and a stale allowance | The Merge DoD's own bar: the checks are visible in the step's real output, in `_design.md` `## Composition`'s shape, not only in a test. This is the run that proves the surface, and the one AC-006 is scored against |
| gate-integration | `cargo xtask narrative` with the tree clean | AC-007's happy path: exactly one added line, no per-page chatter |
| static (regression) | `cargo xtask lint-constitution` | This story did not touch or regress the existing checker — `_decomposition.md` Note 8's "what must not move", and the reviewer's evidence that no shared refactor happened |
| compile | `cargo test --locked -p xtask --doc` | The lib-side harness and every fixture page still compile; this story changed the bin crate only |
| gate-integration | `cargo xtask affected --base main` | The walk is reached on a prose-only change, through `affected::run`'s unconditional file-reading lints (`xtask/src/affected.rs:116-125`) that HS-S0138 wired — the path `.redkiln/config.yaml` uses as this initiative's story grain |
| gate-integration | `cargo xtask ci --fast` | **The merge bar for this non-terminal story** (`project.md` DoD item 6, CLAUDE.md "Commands"). `run_fast` runs the whole `REQUIRED` array unfiltered, so a `probe: None` step cannot hide from it |
| gate-integration | `cargo xtask ci` | Run before the project is called done, not per story (DoD item 6) |

**Red before green.** Every unit test above must be observed failing before the walk exists —
the testing brief's whole posture is that a check nobody watched fail is decorative
(`_decomposition.md` `## Testing brief`, Intent). AC-003's `a_listed_ignore_fence_passes` is
the exception worth naming: it goes green trivially before the `ignore` rule exists, so it is
only meaningful paired with `an_unlisted_ignore_fence_is_rejected` in the same commit.

## Risks and coupling (PR-scoped)

| Risk | Blast radius, and what contains it |
| --- | --- |
| **HS-S0138's parse shape is not the one this walk needs.** If the module landed without a fence parser, or with one that keeps `preceding` for a rule this story deletes, the first commit here is a parser change rather than a check. | Contained by the slice: both stories are implemented in one context, in the order `_storymap.md` `## Merge order` fixes. The mitigation is to read the module as it actually landed before writing a line, not to assume the precedent's struct. |
| **A shared abstraction with `lint_constitution.rs` looks like an obvious cleanup.** It is forbidden, and the reason is not tidiness: the two `ignore` rules differ on purpose, so the helper would be parameterised by the difference. | `_storymap.md`, "Stories this map deliberately does not contain"; `_decomposition.md` Note 8; RS-81-3. A reviewer rejecting the refactor is the control. |
| **`IGNORE_ALLOWANCES` becomes the repair for a failing gate.** The list's whole value is that it is small and read in full; the first entry added to make CI green rather than by decision destroys that. | The reason field, the reverse sweep, and the `const`'s own comment. Worth stating in the PR description: this list ships empty and an addition is a review event (`_design.md` `## Visibility and stability`). |
| **Line-keyed allowances go stale on unrelated edits**, producing failures that look like the checker being wrong. | EC-005 makes the failure specific rather than mysterious, and the `const` comment recommends the anchor form. Residual: an author who ignores both gets one confusing failure, once. |
| **A false positive teaches contributors that the checker cries wolf.** The four-backtick case is the known one; a fence marker inside an indented block is the next candidate. | AC-005 and EC-006. This is why the four-backtick behaviour is in this PR rather than deferred: the design's own fixture is written inside such a block (`_design.md` `## The doctest`). |
| **The `text` back door stays open.** Nothing here stops a Rust example being tagged `text` and skipping the compiler entirely. | Not contained — narrowed and *documented*. AC-007 owes the sentence; `documented-blind-spots-and-their-proofs` (HS-S0145) owns walking a `text`-tagged broken fixture through the gate to prove the limit is real rather than asserted. |
| **`hidden-content-resolution` has to edit this same function.** A walk written as one monolithic closure makes its story a rewrite of this one. | NF-002 and the implementation note on shaping the walk as a pure per-page function. `_design.md` D2 fixes the enforcement site as "the same fence walk as AC-004", so the coupling is intended and only its shape is at risk. |
| **A green gate is read as evidence that the pages teach.** The initiative's top-ranked risk, and this story adds a check that looks reassuring. | AC-007's docs half and `project.md` DoD item 8. Nothing in this PR's prose, messages or module docs may say or imply it; HS-P0024's friction log is not substitutable. |
| **Cross-branch file collision.** `initiative/from-contract-to-published-library` is unmerged and diverges in `spec/SPECIFICATION.md` by 521 lines. | Not a risk for this story: it reads no specification and touches no shared file. `xtask/src/main.rs` is HS-S0138's edit, not this one's. |

## Dependencies

**Blocks on** — `narrative-checker-mounted-with-pinned-path` (HS-S0138). It creates the module
this story writes inside, the `TREE` constant it walks, the accumulate-all/`bail!` reporting
contract its problems join, and the `REQUIRED` step, subcommand, help line, `lint_steps`
membership and `affected` entry that make every one of those problems reachable. Nothing here
is observable without it, and this story adds none of them.

**Unlocks** —

- `hidden-content-resolution` (HS-S0140), directly: `HIDDEN_MARKERS` lands as another check
  over the fences this walk parses (`_design.md` D2; `_storymap.md` `## Merge order` step 5).
- `documented-blind-spots-and-their-proofs` (HS-S0145), directly: its limits list is complete
  only once every check that has a limit exists, and the `text`-tag limit this story states is
  one of its six.
- `observed-failure-falsification` (HS-S0144), indirectly: it observes the assembled gate
  failing, and a tree where any fence may opt out silently is a tree where DoD-2 could be
  satisfied and evaded in the same commit.

No story in the `specification-pin` milestone depends on this one, and this story depends on
nothing in it.

## Anchors (progressive disclosure)

Open these at the moment named, not before. The Context pack above is the must-read; everything
below is depth deferred on purpose.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `xtask/src/lint_constitution.rs` (`:601-673`) | `check_fences` is the shape being copied, message wording included. Lines `:609-619` are AC-001's message, `:621-637` are the exhaustive match AC-002 preserves, `:638-643` are the comment rule AC-003 replaces, `:644-660` are the error-code rules carried over unchanged. | First, before writing any check — read the whole function once so the divergence is a deliberate single edit rather than a rewrite. | AC-002 |
| `xtask/src/lint_constitution.rs` (`:288-330`) | The `fences()` parser: the four-backtick step-over with its stated reason, and the unpaired-opener hole EC-002 closes in the copy. | When copying the parser, before AC-005's test. | AC-005 |
| `xtask/src/lint_constitution.rs` (`:423-458`, asymmetry at `:445-446`) | `check_harness`'s reverse half — the precedent that a forward check without a reverse sweep lets a registration outlive what it registered. AC-004 is the same argument about allowances. | Immediately before implementing the reverse sweep. | AC-004 |
| `xtask/src/lint_constitution.rs` (`:169-198`) | The accumulate-all / report-all / count-last contract, and the one-line green summary at `:192`. The new problems join this exactly; nothing about it is restyled. | Before wiring problems into the module's reporting. | AC-006 |
| `xtask/src/lint_constitution.rs` (`:827-878`) | The house test shape for a bin-crate lint, including the scoped `unwrap_used` allow at `:829` and `four_backtick_fences_are_not_examples` at `:862-868` — the test AC-005 carries over. | When writing the first unit test. | AC-005 |
| `.bklg/docs-that-teach/checked-documentation-surface/_design.md` (`## Signatures`, `## Composition`, `## Transience policy`, `## Density budget`, `## Hierarchy`, `## States`, `## Anti-patterns`) | **Binding, signed off 2026-08-17.** It fixes `IGNORE_ALLOWANCES`'s type and empty initial value, the banner text, the problem-line shape, the 48-character prefix budget inclusive of the doctest prefix, the one-line green summary, and the anti-patterns AC-006 and AC-007 are scored against. | Before writing any output string, and again before claiming AC-006 or AC-007. | AC-006 |
| `.bklg/docs-that-teach/checked-documentation-surface/_decomposition.md` (Note 5, and Note 10 item 5) | Note 5 is where the enumerated-list decision was actually taken, with both reasons and the obligation to document the divergence; Note 10 item 5 is the `text`-tag limit AC-007 must state. | Note 5 before the allowance rule; Note 10 before writing the module docs. | AC-003 |
| `.bklg/docs-that-teach/checked-documentation-surface/_decomposition.md` (`## Testing brief`, AC-004 paragraph, items (a)–(d)) | The four required unit tests and, for (b) and (d), the named wrong implementation each rejects. A test that rejects nothing plausible is decorative. | Before writing the test list; it is the checklist AC-001–AC-004 are scored against. | AC-004 |
| `standards/rust/81-checks-that-cannot-be-types.md` (RS-81-1 at `:11`, RS-81-2 at `:95-104`) | RS-81-2 is the constitution's form of the exhaustive-match posture — a construct the scanner cannot read is an error, because the alternative is that every match silently stops reporting. RS-81-1 is why the limit is proven and then stated. | RS-81-2 when writing the `match`; RS-81-1 when writing the module docs. | AC-002 |
| `.bklg/docs-that-teach/checked-documentation-surface/narrative-checker-mounted-with-pinned-path/spec.md` | The dependency's own contract: what the module, its constants and its reporting actually look like when this story starts. Read the module as it landed, not the precedent it was copied from. | First thing, before the parser decision. | AC-001 |
| `xtask/src/constitution.rs` (`:1-44`) | The worked example of a limits section that documents an untestable gap honestly, and the `include_str!` technique AC-007's docs test uses. | When writing the module docs and their test. | AC-007 |
| `.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md` (`## Persona 1 — The application author`) | The reader whose trust in a fenced example is what these checks protect; the source the ACs are framed from, since `.kb/product/` holds no persona atom yet. | If an AC's intent needs re-grounding during implementation or review. | AC-001 |
| `xtask/Cargo.toml` (`:16-21`) | DR-12's standing trade: why no dependency is added to `xtask`, stated in the file itself. | Only if a crate looks like the easy way to parse markdown. | NF-001 |

## Clarifications resolved during spec

1. **Seven ACs, exactly the ids the front half declared, and how they were allotted.** Five
   checks and two surface obligations: AC-001 untagged, AC-002 exhaustive info string, AC-003
   the allowance gate in both directions (unlisted rejected, listed passes), AC-004 the reverse
   sweep, AC-005 no false positive and the right location, AC-006 the failure surface, AC-007
   the green surface and the record. Nothing was added or dropped.
2. **The four-backtick behaviour got its own id rather than being folded into AC-001.** It is a
   *false-positive* criterion, and the design's own fixture depends on it (`_design.md`
   `## The doctest` is written inside a four-backtick block). Folding it into a
   true-positive AC would have left the repository's cheapest way to discredit the whole check
   ungated.
3. **The `compile_fail` error-code rules are inside AC-002, not an AC of their own.** They are
   part of the same closed match (`lint_constitution.rs:644-660`), no project AC names them,
   and splitting them would produce a row whose only wrong implementation is a subset of
   AC-002's.
4. **The unterminated-fence hole is an error condition, not an acceptance criterion.** The
   precedent parser drops an unpaired opener, so its info string is never examined — an
   inherited opt-out route. It is EC-002 because it is a property of the copied parser rather
   than a behaviour project AC-004 asks for, and it is fixed **in the copy only**: `_storymap.md`
   and `_decomposition.md` Note 8 both forbid editing `lint_constitution.rs` here.
5. **The documentation obligation is gated, not left to review.** `_storymap.md` `## Coverage`
   assigns DoD item 7 to every story for what that story adds, and an ungated obligation on
   the last story in a slice is one that lands last or not at all. AC-007 verifies it by
   reading the module's own source with `include_str!`, the technique `xtask/src/constitution.rs`
   already uses — a bin-crate `//!` doc is not compiled by `cargo test --doc`, so a doctest
   could not have carried it.
6. **Both halves of "line-or-anchor" are supported, and the anchor form is recommended in the
   `const`'s own comment.** `_design.md` `## Signatures` fixes the tuple as
   `(page path, line-or-anchor, reason)` without choosing between them; this spec resolves that
   the checker accepts either, that ambiguity and duplication are error conditions (EC-003,
   EC-004), and that line-keyed drift is reported specifically (EC-005) rather than as a bare
   staleness message.
7. **Separator normalisation is decided here (NF-007) rather than discovered.** The design's
   paths are `/`-shaped and this repository is developed on Windows; an allowance list that
   matches nothing on one platform grants permission on one runner and denies it on another.
8. **No ADR, and that is a verified finding rather than an omission.** `_storymap.md`,
   "Stories this map deliberately does not contain", records that no Accepted decision atom
   under `.kb/decisions/` governs gate structure, documentation trees or fence compiling, and
   that this divergence is from in-repo *precedent* — a convention, discharged by a sentence in
   the new module's docs. Writing one here would also violate this repository's standing rule
   that an ADR is never a side effect of implementation work.
9. **`IGNORE_ALLOWANCES` ships empty.** Every allowance-path test constructs its own list. A
   `const` that ships with an entry ships with a precedent for adding the second.
10. **The module's file name is `xtask/src/lint_narrative.rs`, taken from the dependency rather
    than chosen here.** `narrative-checker-mounted-with-pinned-path`'s spec fixes it (module
    `lint_narrative`, following `lint_constitution`'s precedent) and records why it cannot be
    `xtask/src/narrative.rs`: that path is `_design.md` `## Signatures`'s `HARNESS`, the lib-side
    file this module reads as *text*. Every test path in this spec and its ledger names the
    checker file on that basis; if the dependency lands under another name, the ledger's
    `verifying_test` paths move with it and nothing else in this spec changes.
