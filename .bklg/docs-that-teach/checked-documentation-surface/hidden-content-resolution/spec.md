---
item: HS-S0140
stage: spec
created: 2026-08-17T13:16:03.191Z
updated: 2026-08-17T13:16:03.191Z
template_sig: 87bbf1d0
rendered_sig: 6aa88373
---

# Spec — Hidden content is inside the check, or absent

## Scope lock

| What | Path |
| ---- | ---- |
| Initiative (gold source) | `.bklg/docs-that-teach/initiative.md` — BR-11, AC-006's parent, DoD scenario 13 (`:458-461`), DT-7 (`:491`) |
| Initiative decomposition | `.bklg/docs-that-teach/_decomposition.md` — why HS-P0020 owns DT-7 rather than a content project |
| Project | `.bklg/docs-that-teach/checked-documentation-surface/project.md` — AC-006, DR-08, DoD items 3 and 8 |
| This spec | `.bklg/docs-that-teach/checked-documentation-surface/hidden-content-resolution/spec.md` |
| Signed-off design (**binding**) | `.bklg/docs-that-teach/checked-documentation-surface/_design.md` — D2 resolves DT-7; `HIDDEN_MARKERS`, the `## States` message form, anti-pattern 1, the negative fixture. Approved 2026-08-17, no conditions |
| Key briefs | `.bklg/docs-that-teach/checked-documentation-surface/_decomposition.md` — architecture AC-006 (`:75-79`) and Note 5 (allowance-list shape), ux DT-7 (`:424-465`), testing AC-006 (`:608-620`) |
| Story map / slice | `.bklg/docs-that-teach/checked-documentation-surface/_storymap.md` — activity C, milestone `narrative-checker-discipline`, merge position 5 |
| Discover | `.bklg/docs-that-teach/checked-documentation-surface/hidden-content-resolution/discover.md` — the named wrong implementation |
| Roadmap pointer | `RUNBOOK.md:916-925` — the decorative-step precedent this whole project exists not to repeat |

## One-line PR slice

Close DT-7 in `_design.md` and enforce the answer in the same fence walk — either the markers that
produce tabs/folds/panels are rejected under the pinned tree, or a claim broken inside a non-default
panel is observed to fail the gate and the run recorded.

## Executive summary

DT-7 is **already closed**, in `_design.md` `## Pattern decision` D2, signed off by the repository
owner on 2026-08-17 with no conditions. So the first half of that sentence is a pointer, not work,
and the second half is a branch this story deliberately does **not** take: the design chose *(a)
mechanically forbidden*, which means the disjunction in project AC-006 resolves to its first arm and
the hidden-panel falsification is not run.

The delta this PR lands is that resolution made executable. `HIDDEN_MARKERS` — seven tokens,
`_design.md` `## Signatures` — is added to the bin-crate narrative checker its slice-mates built, and
scanned over every page under the pinned tree from inside the *same* fence walk that
`fence-discipline-and-allowance-list` just wrote, pushing onto the same `Vec<String>` and reported by
the same single `bail!`. It ships with the named wrong implementation the design demands (the fixture
page of `## The doctest`, wrapped in `<details>`), a case-insensitive match so the elaborate spellings
this repository has already been bitten by cannot get through, and a set-pin so shrinking the token
list fails a test rather than quietly reopening a tension a human closed.

What it does not add: an allowance list (`_design.md` `## Shape decision`, row 3 — deliberately none),
a page under `docs/`, or any claim that a green gate means a page teaches (project DoD item 8).

## Context pack

**DT-7's answer, as decided — do not re-derive it.** `_design.md` D2: scoped divergence is written as
**visible level-3 subsections under one level-2 heading** while it is ≤ 3 scopes *and* ≤ 25 rendered
lines per scope; past either bound it becomes **one page per scope**; and **hidden panels are rejected
by the checker, by file and line**. There is no third state where scoped content is present but
hidden (`_design.md` `## Transience policy`, the *Scoped subsections* row). This story implements the
third clause. It does not implement, tune or gate the threshold — the 250-line page cap and the
40-character H1 cap are review rules by decision, and only the *path*-length budget became a gate
rule (`_design.md` `## Open questions`, item 2).

**Why the falsification branch of AC-006 is the wrong half to build, stated so nobody "improves" this
back.** Project AC-006 permits (a) *if* a claim broken inside a non-default panel is observed to fail
the gate. `_design.md` D2 argument 2 rejects that route on the ground this repository already uses for
conformance rules: the observation would establish the property for one construct, one extractor and
one toolchain, nothing would notice it regressing, and the fixture page keeps passing whichever way
extraction goes — **a rule no implementation can fail**. Rejecting the markers instead yields a rule
with a named wrong implementation that fails today, deterministically, on every runner. Two further
reasons compound it: D1 removed the mechanism (no mdBook, therefore no `mdbook-tabs`), and
compilation is only one of three unverified properties — search, Ctrl-F and print stay unverified even
if extraction resolved (`_decomposition.md:441-449`; `_discovery/distillation/interaction-patterns.md:213-216`).

**No allowance list, and this is a decision rather than an omission.** `IGNORE_ALLOWANCES` exists
because the need for an uncompiled fence is real and enumerable, and a reverse sweep makes a stale
entry detectable. A hidden-panel allowance would be permission to reintroduce an *unverified
mechanism* one page at a time, and no sweep can detect that (`_design.md` D2, "No allowance list for
hidden markers"; `## Shape decision` row 3). Shrinking `HIDDEN_MARKERS` "re-opens DT-7 and requires a
new design record, not an edit" (`_design.md` `## Visibility and stability`).

**Where it mounts, and the mistake that is available.** The enforcement lands "in the same fence walk
as AC-004" (`_decomposition.md:75-79`; `_storymap.md`, why `narrative-checker-discipline` is one
surface) — the bin-crate checker module declared from `xtask/src/main.rs:64-70` alongside
`mod lint_constitution;` (`:65`). `xtask` has two targets that do not share modules: the *harness* is
`xtask/src/narrative.rs` in the **lib** target (`_design.md` `## Placement and re-export`), and a
check added there would be compiled by rustdoc and run by nothing. Adding a second walk, a second
`REQUIRED` step or a second `bail!` is equally wrong: `lint_constitution::run` accumulates every
problem and prints them all before one terminal count (`xtask/src/lint_constitution.rs:169-198`),
because "a check that stops at the first problem turns one review cycle into six"
(`_decomposition.md:218-219`).

**The output form is fixed by the design, not by taste.** One line per occurrence, on stderr, two-space
indent, `{path}:{line} — {message}`, location first because the location is the primary element
(`_design.md` `## Hierarchy`; the primitive is `xtask/src/lint_constitution.rs:605-660`). The message
is given verbatim in `_design.md` `## States`, the *Hidden marker present* row:
`` docs/<page>.md:<line> — `<details` is a hidden panel; DT-7 forbids it in {TREE} ``. No truncation,
ever — "… and N more" is anti-pattern 8.

**The negative fixture cannot live under the pinned tree, and that is the load-bearing detail.**
`_design.md` `## The doctest` assigns this story the negative fixture: the fixture page with a
`<details>` wrapper around the scope band, which must fail the checker by file and line, "and without
it the rule is decorative." A file carrying that wrapper committed under `docs/` would fail
`cargo xtask ci` forever — it is the one wrong implementation that cannot be a real page. It is
therefore *test material*, as a page-shaped `&str` in the checker's own `#[cfg(test)] mod tests`,
which is exactly how `lint_constitution`'s tests are written today (string inputs to pure functions;
`xtask/src/lint_constitution.rs:826-876`) and the only shape available without a new dev-dependency —
`xtask` has no `tests/` directory and no `tempfile`, and DR-12's standing trade at
`xtask/Cargo.toml:16-21` is why widening that graph needs an argument. The consequence for the
implementation is a *shape* obligation, not a style one: the marker scan must be a pure function over
`(page path, page text)` so the fixture can be fed to it, in the same way the page→module derivation
must be pure so both directions of the orphan check agree (`_decomposition.md`, Note 2).

**The spellings the check must survive.** HTML tag names are case-insensitive, so a case-sensitive
`contains` accepts `<Details>` and `<DETAILS open>` — the same class of hole as the "elaborate
spellings of `ignore`" this repository already found in its own testkit doctests (`project.md`,
"Closing the opt-out"). The match is therefore ASCII-case-insensitive, and the scan is **line-based
over the whole page including fenced blocks**: a marker quoted inside a `text` fence still renders as
a page telling a reader to fold something, and there is no allowance path to exempt it. The checker's
own module docs are outside `TREE`, so the rule can be documented in prose that names the tokens
without tripping itself.

**Whose journey this protects.** The reader harmed by a hidden panel is persona 2, the adapter author,
whose whole goal is understanding *why* the port is shaped as it is
(`_discovery/distillation/personas-and-journeys.md:148-166`) and for whom the per-adapter fanout is
the natural tab strip. The in-house precedent is not hypothetical: a reviewer added a sixth event
type, updated a fold, and forgot the query it should have stayed in sync with — an invariant stated
once visibly and once behind a fold, with nothing catching the drift
(`_discovery/distillation/interaction-patterns.md:244-252`). The contributor at the gate is the
person the message form above is designed for.

**The seam this story must not cross.** DT-8 — where the line falls between a safe aside and a
load-bearing constraint — is HS-P0021's (`initiative.md:492`). A blanket marker ban makes DT-8 moot
*inside* the pinned tree only; if a genuinely non-normative disclosure use ever appears, HS-P0021
petitions with an allowance shaped like `IGNORE_ALLOWANCES`, in its own change, with its own
falsification. **Do not build that allowance now, and do not leave a hook for it**
(`_design.md` D2, "What this does not decide"; `## Open questions` item 3). And per project DoD item
8, nothing this story writes — message, module doc or ledger — may assert that the surface proves a
page teaches.

## Integration contract

- **Archetype**: `capability` — observable end to end through `cargo xtask ci`, `cargo xtask ci --fast`
  and `cargo xtask narrative`.
- **Slice / milestone**: `narrative-checker-discipline`. Slice-mates, implemented in this one context
  and mounted as one surface: `narrative-checker-mounted-with-pinned-path` (the module, the pinned
  constants, the vacuity guard, the `REQUIRED` step) and `fence-discipline-and-allowance-list` (the
  fence walk this story extends). This story is **third** in the slice's merge order (`_storymap.md`,
  "Merge order" 2.5) and is `blocked_by: HS-S0139`.
- **Mount point**: `xtask/src/main.rs` — the bin-crate narrative checker module declared at `:64-70`
  alongside `mod lint_constitution;`, reached from its `REQUIRED` entry, its dispatch arm and its
  `lint_steps()` membership (`xtask/src/main.rs:799-812`; `steps_named` panics on a name absent from
  `REQUIRED`, `:816-826`, which makes a half-mounted step a build-time bug). This story's marker scan
  is called from that module's existing fence walk and reported by that module's single `bail!` — it
  adds no step, no subcommand and no banner of its own.
- **Wires into**:
  - `xtask/src/lint_constitution.rs:601-673` — `check_fences`, the fence-walk shape the slice-mate
    copied and this story extends; `:169-198` for the accumulate-all / count-last contract;
    `:605-660` for the `{path}:{line} — {message}` primitive.
  - `xtask/src/lint_constitution.rs:54-61` — `ATOM_DIR` / `ROUTER` / `HARNESS`, the pinned-path
    precedent `TREE` follows.
  - `xtask/src/lint_constitution.rs:826-876` — the test shape: pure functions fed string inputs, no
    fixture directory, no `tempfile`.
  - `xtask/src/main.rs:105` (`REQUIRED`), `:462-492` (the two constitution steps whose naming
    convention the checker's banner follows), `:799-812` (`lint_steps`).
  - `docs/` — read only. `TREE` is `docs/` (`_design.md` `## Signatures`); this story adds nothing to
    it.
  - No new dependency, no feature, no `cfg`, no port, no `Send` bound (`_design.md`
    `## What it costs a caller`).
- **Public items** (`_design.md` `## Items`): `HIDDEN_MARKERS` — added, private `const`, default
  feature, no semver promise, "shrinking it re-opens DT-7". This story claims that item and no other;
  `TREE`, `HARNESS` and `IGNORE_ALLOWANCES` belong to its slice-mates.
- **Renders surfaces**: `gate-narrative-checker-step` — specifically its `fail-hidden-marker` state,
  and the `## States` *Hidden marker present* row. No markdown surface is rendered or changed by this
  story; `narrative-page`'s hidden-panel prohibition is what this story makes true of it, expressed
  entirely through the gate's output.
- **Conformance rule(s)**: none, and this is not adapter-observable. Nothing here touches a port, a
  store, a value type or the testkit, so no rule in `crates/happenstance-testkit/src/suite.rs` can
  observe it; the equivalent obligation — a check that can actually fail, with a named wrong
  implementation written down — is discharged by the negative fixture in AC-002 and the set-pin in
  AC-004 (`CLAUDE.md`, "A rule that no adapter can fail is decorative", applied one level up as
  `_design.md` D2 argument 2 does).
- **Clause(s)**: none. This story discharges and amends no `SPECIFICATION.md` clause and touches no
  `[FROZEN]` text, so no ADR is owed; the grounding pass confirmed no Accepted decision atom under
  `.kb/decisions/` governs gate structure, documentation trees or fence discipline (`_storymap.md`,
  "Stories this map deliberately does not contain").
- **Advances DoD scenario**: initiative DoD scenario **13** — "Nothing load-bearing is hidden from the
  check" (`initiative.md:458-461`) — to green by its second arm: no shipped content carries a
  load-bearing claim behind a fold, because no shipped content may carry a fold at all. It also
  discharges project DoD item 3's *otherwise* clause and DR-08.

## PR boundary

**In this PR**

- `HIDDEN_MARKERS` as a documented private `const` in the bin-crate checker module, with the seven
  tokens `_design.md` `## Signatures` enumerates and a doc comment naming DT-7, citing `_design.md`,
  and saying that shrinking the set requires a new design record.
- The marker scan itself: a pure function over `(page path, page text)`, ASCII-case-insensitive,
  line-based over the whole page, one problem per occurrence, pushed onto the walk's existing
  `Vec<String>`.
- The call site inside the existing fence walk, so the new problems interleave in source order with
  the untagged-fence and unlisted-`ignore` problems and are counted by the same `bail!`.
- Unit tests: the negative fixture page as a `&str` constant, the same page without the wrapper, the
  case and spelling variants, and the set-pin whose failure names which token moved.
- The "What this does not verify" additions for this check, in the module docs, first rather than last
  (`xtask/src/lint_constitution.rs:9-28`; RS-81-1).
- This story's own ledger and report under its backlog folder.

**Explicitly not in this PR**

- Any file under `docs/`. The negative fixture is test material precisely because committing a
  `<details>` page under the pinned tree would fail the gate permanently.
- An allowance list, an escape hatch, an environment variable or a `#[cfg]` that turns the marker scan
  off — `_design.md` `## Shape decision` row 3.
- The hidden-panel falsification procedure (project AC-006's second arm) and any recorded gate run —
  the design took the forbid branch; the observed-failure procedure is
  `observed-failure-falsification`'s, on the fixture page's *fence*, not on a panel.
- The fence walk itself, the pinned constants, the vacuity guard, the `REQUIRED` step, the
  subcommand, the banner, `docs/README.md`'s rows and `affected.rs`'s selection arm — slice-mates'.
- The threshold enforcement (≤ 3 scopes × ≤ 25 lines), the page-length and H1 budgets — review rules
  by decision (`_design.md` `## Open questions` item 2), and gating them would take HS-P0021's job.
- DT-8's aside/constraint line, any teaching page, and `mdbook`, a theme, a stylesheet or a `book/`
  directory (anti-patterns 10 and 11).

```
xtask/src/*.rs
.bklg/docs-that-teach/checked-documentation-surface/hidden-content-resolution/**
```

**Merge DoD one-liner** — `cargo xtask ci --fast` is green, the negative fixture fails the checker by
file and line while the same page without its wrapper passes, and `docs/` is unchanged.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| DT-7 resolves to *forbidden*, and this story does not re-decide it | Visible level-3 subsections inline while ≤ 3 scopes × ≤ 25 rendered lines; one page per scope past either bound; hidden panels rejected by the checker. Signed off 2026-08-17, no conditions | `_design.md` `## Pattern decision` D2, `## Sign-off` |
| The token set | Seven tokens, exactly as designed: `<details`, `<summary`, `{{#tabs`, `{{#tab `, `{{#endtabs`, ` ```admonish `, `<!-- tab`. Private `const`, no feature gate | `_design.md` `## Signatures`, `## Items`, `## Visibility and stability` |
| Scope of the scan | Every page under `TREE` (`docs/`), whole file, line by line, including fenced blocks and including `docs/README.md`. No file-level, page-level or fence-level exemption exists | `_design.md` `## States` (*Hidden marker present*), `## Shape decision` row 3 |
| Matching | ASCII-case-insensitive, so `<Details>` and `<DETAILS open>` are rejected. The wrong implementation is a case-sensitive `contains` — the same hole as the elaborate spellings of `ignore` already found here | `project.md`, "Closing the opt-out"; `xtask/src/lint_constitution.rs:638-643` |
| Report form | One stderr line per occurrence, two-space indent, `{path}:{line} — {message}`, location first; message as designed: `` `<details` is a hidden panel; DT-7 forbids it in docs ``. Never truncated | `_design.md` `## States`, `## Hierarchy`, anti-pattern 8; `xtask/src/lint_constitution.rs:605-660` |
| Accumulation | Pushed onto the fence walk's existing `Vec<String>`; every problem printed in source order (path then line); one terminal `bail!("{n} problem(s) in {TREE}")`. No new step, banner, subcommand or early return | `xtask/src/lint_constitution.rs:169-198`; `_decomposition.md:218-219`; `_design.md` `## Composition` |
| Interface shape | A pure function over `(page path, page text)` returning/pushing problems, so the negative fixture is exercised without writing a file into the pinned tree | `_decomposition.md`, Note 2; `xtask/src/lint_constitution.rs:826-876` |
| Named wrong implementation | The `## The doctest` fixture page with a `<details>` wrapper around its scope band: rejected by file and line. The same page unwrapped: clean. Without this pair the rule is decorative | `_design.md` `## The doctest` (closing paragraph); `CLAUDE.md`, "A rule that no adapter can fail is decorative" |
| Set-pin | The derived token set is checked against a hand-written count/list and the failure names *which token moved*, so shrinking `HIDDEN_MARKERS` fails a test instead of silently reopening DT-7 | `standards/rust/81-checks-that-cannot-be-types.md:335-341` (RS-81-5); `_design.md` `## Visibility and stability` |
| Mounting | Reached from `cargo xtask ci`, `cargo xtask ci --fast` (`REQUIRED` runs unfiltered), `cargo xtask narrative` and `cargo xtask affected --base main` after a `docs/`-only change | `xtask/src/main.rs:105`, `:816-826`, `:853-860`; `_decomposition.md` testing brief AC-009 |
| Stated limits | Added to the module's "What this does not verify" section, first: a marker spelling absent from the set, disclosure produced outside the pinned tree, and that the scan reads *source* — it cannot know what a renderer does with it | `xtask/src/lint_constitution.rs:9-28`; `standards/rust/81-checks-that-cannot-be-types.md` (RS-81-1) |
| No allowance path, no hook for one | The absence is the decision. A future non-normative use petitions in its own change with its own falsification | `_design.md` D2, `## Shape decision` row 3, `## Open questions` item 3 |
| No teaching claim | No message, doc line or artefact in this story asserts the surface proves a page teaches | `project.md` DoD item 8 |

## Data and migrations

**N/A.** This story adds a `const` and a source scan to a `publish = false` build tool. There is no
schema, no persisted state, no serialised envelope and no stored artefact: the checker reads markdown
under `docs/` and writes only to stdout/stderr and its exit status. Nothing in `crates/` is touched,
so there is no public API change, no feature, no `cfg`, no MSRV movement and no wasm32 consequence
(`_design.md` `## What it costs a caller`). The one thing that behaves like a migration is editorial
rather than mechanical and is out of scope here: existing prose elsewhere in the repository that uses
`<details>` is untouched, because the scan is scoped to `TREE` by decision — the same scoping rule
RS-81-3 states, that a scanner is scoped to the directory whose behaviour it constrains
(`_storymap.md`, "Stories this map deliberately does not contain").

## Acceptance criteria

Six criteria. All six trace to project **AC-006** — this story owns it outright
(`_storymap.md`, "Coverage") — and together they discharge project DoD item 3's *otherwise*
clause, DR-08's first arm and initiative DoD scenario 13 (`initiative.md:458-461`). AC-005 and
AC-006 additionally carry the project-wide obligations DoD items 7 and 8 that the story map
says every story owes for whatever it adds (`_storymap.md`, "Coverage", closing bullets).

Each criterion is written from the reader or contributor whose goal it protects, because a
marker ban stated as a capability ("the checker rejects `<details`") is the kind of sentence
that survives a refactor into decoration. `{checker}` below is `xtask/src/lint_narrative.rs`,
the module `narrative-checker-mounted-with-pinned-path` creates
(`narrative-checker-mounted-with-pinned-path/spec.md:70`); `{fence walk}` is that module's
`check_fences` analogue, which `fence-discipline-and-allowance-list` writes
(`fence-discipline-and-allowance-list/spec.md:46`, `:172`).

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **GIVEN** persona 2, the adapter author, whose goal is understanding *why* the port is shaped as it is and who therefore meets the per-adapter fanout as the natural tab strip (`personas-and-journeys.md:148-166`), **WHEN** any contributor writes scoped divergence into a page under `docs/` using a disclosure marker — a fold, a tab directive or a collapsed admonition — **THEN** `cargo xtask narrative` and `cargo xtask ci` fail, naming the file and the line, from inside the fence walk that already exists rather than from a step of this story's own; and **THEN** the contributor's remaining problems on other pages are reported in the same run, because the marker problem is pushed onto that walk's existing `Vec<String>` and counted by its single `bail!` | Unit: `{checker}`'s `#[cfg(test)] mod tests` — one test feeding a page containing each of the seven tokens and asserting one problem per occurrence with the `{path}:{line}` prefix; one test asserting a marker problem and an untagged-fence problem from the same page both appear in one returned list, in source order. Gate: `cargo xtask narrative` and `cargo xtask ci --fast` exit 0 over the real tree (`docs/README.md` plus the fixture page), proving the check is mounted and not vacuous |
| AC-002 | **GIVEN** the named wrong implementation this story exists to reject — "a `<details>` block whose inner claim is broken and the gate stays green" (`hidden-content-resolution/discover.md:55`) — **WHEN** the `_design.md` `## The doctest` fixture page is wrapped in a disclosure block around its scope band, **THEN** the checker reports it by file and line, and **WHEN** that wrapper alone is removed, **THEN** the same page produces no problem at all. Both halves are required: without the failing half the rule is decorative (`_design.md` `## The doctest`, closing paragraph; CLAUDE.md, "A rule that no adapter can fail is decorative"), and without the clean half the rule cannot be distinguished from one that rejects every page | Unit: `{checker}`'s tests — the wrapped fixture page as a `&str` constant asserted to yield exactly two problems (the `<details` and `<summary` lines) each naming its own line; the identical page minus the wrapper asserted to yield zero. The fixture is test material, never a file under `docs/`, because a committed wrapper would fail `cargo xtask ci` forever (`_decomposition.md`, "Fixture pages live inside this project, not in the corpus") |
| AC-003 | **GIVEN** that this repository has already been bitten by "elaborate spellings" of an opt-out getting past a naive match (`project.md`, and the risk row "`ignore` returns by the back door"), **WHEN** an author reaches for a disclosure marker in any spelling a renderer accepts — `<Details>`, `<DETAILS open>`, `{{#tabs`, a `{{#tab ` directive, `{{#endtabs`, an ```` ```admonish ```` fence or an `<!-- tab` comment — or hides one inside a fenced block or inside `docs/README.md` itself, **THEN** every one is a problem, because HTML tag names are case-insensitive, a marker quoted in a fence still renders as a page telling a reader to fold something, and there is no file-level, page-level or fence-level exemption anywhere in the design (`_design.md` `## Shape decision` row 3) | Unit: `{checker}`'s tests — a case-variant table (`<details`, `<Details`, `<DETAILS`, `{{#TABS`) asserted rejected; one test asserting a marker inside a ```` ```text ```` fence is still reported, whose wrong implementation is a fence-aware scan that skips fenced bodies; one test asserting `docs/README.md`'s real current text is clean, so the pre-existing index passes unchanged. Gate: `cargo xtask narrative` green over the tree as it stands |
| AC-004 | **GIVEN** a future maintainer who finds one token inconvenient and deletes it, **WHEN** `HIDDEN_MARKERS` is shrunk, re-ordered or has a token upper-cased, **THEN** `cargo test -p xtask` fails with a message naming *which* token moved rather than only that two numbers disagree — so DT-7, which a human closed on 2026-08-17 with no conditions (`_design.md` `## Sign-off`), cannot be reopened by an edit to a `const`, only by a new design record (`_design.md` `## Visibility and stability`) | Unit: `{checker}`'s tests — the RS-81-5 pin (`standards/rust/81-checks-that-cannot-be-types.md:335-341`): the derived set compared against a hand-written list of seven, the failure naming the differing token in both directions (present-but-unexpected, expected-but-absent); plus an assertion that every token is already ASCII-lowercase, which is what makes the case-insensitive comparison in AC-003 correct rather than accidentally correct |
| AC-005 | **GIVEN** the contributor at the gate, whose whole task on a red run is deciding where to look first, **WHEN** a run reports hidden-marker problems — one, or forty — **THEN** each is one stderr line composed from the existing problem-line primitive: two-space indent, `{path}:{line}` first, an em dash, then `` `<token>` is a hidden panel; DT-7 forbids it in docs ``; the list is in source order, path then line, interleaved with the walk's other problem kinds; it is **never** truncated with "… and N more"; the single terminal `bail!("{n} problem(s) in docs")` is the last line; and no banner, step, subcommand, spinner, progress line or success chatter of this story's own is added anywhere. **AND** the module's rustdoc leads with what this check does not verify, before what it does | Unit: `{checker}`'s tests — an exact-string assertion on one composed problem line (the `_design.md` `## States` message form, verbatim); a 40-occurrence page asserted to yield 40 lines with no elision token; an ordering assertion over a two-page input. Static: `cargo xtask lint-constitution` and `cargo test -p xtask --doc` still pass, proving the added module docs are well-formed and the pre-existing checker did not regress. Review: `REQUIRED` gains no entry and `main.rs`'s dispatch gains no arm in this story's diff |
| AC-006 | **GIVEN** persona 1 and persona 3, who will one day read this material and must not be told a green gate means a page teaches (`project.md` DoD item 8), and **GIVEN** HS-P0021, which owns DT-8's aside/constraint line (`initiative.md:492`), **WHEN** this story's diff is reviewed, **THEN** no message, doc comment, ledger row or report line asserts that the surface proves comprehension, no badge or "verified" mark exists (`_design.md` anti-pattern 9), and **THEN** no allowance list, environment variable, `#[cfg]`, feature or commented-out hook exists by which a hidden marker could be permitted — the absence is the decision, and a future non-normative use petitions in its own change with its own falsification (`_design.md` D2, `## Open questions` item 3) | Review + static: a `rg` over this story's diff for `allow`/`skip`/`SKIP`/`cfg(`/`env::var` inside `{checker}` returning nothing new; a `rg` for `verified`/`badge`/`proves` in the added prose returning nothing that asserts teachability. Unit: one test asserting that no input — not `docs/README.md`, not a page with a comment claiming an exemption, not a page naming an allowance — makes a marker pass, which is the executable form of "there is no allowance path" |

## Interaction quality

RFC §6.7/D6. This story renders one surface — `gate-narrative-checker-step`, specifically its
`fail-hidden-marker` state (`_design.md` `## Surfaces`) — so the composition family below is
taken from the signed-off `_design.md` rather than re-decided, and every invariant that applies
is already an **AC-### row in the table above**. This section only says which row carries which
invariant, and how each is verified. Nothing here is a new obligation.

The medium is a terminal, not a DOM, so the state family is translated once and explicitly
rather than silently dropped: "in place" means *inside the step already running*, "focus and
scroll" means *the order the contributor's eye and editor traverse*, and "keyboard
reachability" means *reachable by a typed command with no interactive prompt and no TTY
dependence*. The translation is not a weakening — `_design.md` `## Hierarchy` makes exactly
this argument, that position, heading level and adjacency are the only three levers plain
markdown and a terminal both have.

**STATE invariants**

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **In place, not a context jump** — the marker report appears inside the existing narrative step's output, under the banner the contributor is already reading. A second step, a second banner or a second subcommand would make one red run answer from two places | **AC-001**, **AC-005** | AC-001's "from inside the fence walk that already exists"; AC-005's review clause that `REQUIRED` and the dispatch gain nothing in this diff (`xtask/src/main.rs:105`, `:799-812`) |
| **Non-occlusion** — a marker problem never hides another problem, and is never hidden by one. Every problem in the run prints, in source order, before the count | **AC-001**, **AC-005** | AC-001's mixed-problem-kind ordering test; AC-005's 40-occurrence no-elision test. The contract is `xtask/src/lint_constitution.rs:169-198`; the reason is `_decomposition.md:218-219` |
| **Preserved traversal order** — path then line, so the list reads in the same order as the tree the contributor is about to edit; their place is not shuffled between runs | **AC-005** | AC-005's ordering assertion over a two-page input |
| **Reversibility** — removing the marker returns the page to clean, in one step, with nothing left behind. The wrapped/unwrapped fixture pair *is* the round trip | **AC-002** | AC-002's two-sided unit test. This is the same observed-fail-then-observed-green discipline DoD item 2 imposes elsewhere, applied at unit grain because the panel branch was not taken |
| **Reachability without an interactive step** — every path to this check is a typed command: `cargo xtask narrative`, `cargo xtask ci`, `cargo xtask ci --fast` (which runs `REQUIRED` unfiltered, `xtask/src/main.rs:853-860`). No prompt, no TTY requirement, no `probe:` that can decline | **AC-001** | AC-001's gate verification on `narrative` and `ci --fast`. `probe: None` is structural, not stylistic (`project.md:153` DR-03; `_design.md` `## Transience policy`, last row) |

**COMPOSITION invariants** — from `_design.md`, binding

| Invariant | The design's number | Carried by | How it is verified |
| --- | --- | --- | --- |
| **Presentation exists at all** — the problem is a *composed* line built from the verified primitive `  {path}:{line} — {message}` (`xtask/src/lint_constitution.rs:605-660`), not a bare `println!` of a token name. An uncomposed report satisfies "the check fires" perfectly and is still the wrong surface | `## Composition`; opening primitive table | **AC-005** | AC-005's exact-string assertion on one composed line, against the `## States` message form verbatim |
| **Composition and placement** — banner, then problem lines, then the count. The count is last and carries the directory, because it is the only line that scrolls into view when the list is long | `## Composition`, three composition rules | **AC-005** | AC-005's exact-string and ordering assertions; the terminal `bail!` shape at `xtask/src/lint_constitution.rs:199` |
| **Transience** — problem lines are *opened on demand, by failing*: they exist only in the failure state. The banner is persistent chrome and is also the loading state, so **no** spinner, dot ticker or per-page progress line is added. Success stays at one line | `## Transience policy`; `## States`, *Loading* | **AC-005** | AC-005's "no banner, step, subcommand, spinner, progress line or success chatter of this story's own", verified by review of the diff and by the green-run output over the real tree |
| **Density budget, with its real numbers** — 80-column line; location prefix ≤ **48** characters including `:{line}`, i.e. ≤ **32** characters of repo-relative path once the 16-character `xtask\src\../../` doctest prefix is counted; success output **1** line; problem list **unbounded, never truncated** | `## Density budget`, the terminal-surface table | **AC-005** | AC-005's no-elision test and its exact-line assertion. The path-length half is enforced by the slice-mate's pinning check (`_design.md` `## States`, *Long label*); this story must not emit a message long enough to push `{path}:{line}` off the first visual row, which cannot happen while the location is first |
| **Hierarchy** — primary is `{path}:{line}`, carried by being first and by the em dash separator; secondary is the message; recessive are the two-space indent, the banner and the count | `## Hierarchy`, `gate-narrative-checker-step` | **AC-005** | AC-005's exact-string assertion, which pins all three positions in one comparison |
| **Anti-pattern 1** — a disclosure triangle, tab strip or collapsed callout anywhere on a page under `docs/`: "if a screenshot shows something a reader must click to read, DT-7 has been reversed without a design record" | anti-pattern 1 | **AC-001**, **AC-002**, **AC-003** | The whole point of the check. AC-003 is the row that closes the spelling variants, which is where a screenshot-only rule would leak |
| **Anti-pattern 7** — a gate failure whose first visual row does not begin with `path:line` | anti-pattern 7 | **AC-005** | AC-005's exact-string assertion |
| **Anti-pattern 8** — a truncated problem list; any "… and N more" | anti-pattern 8 | **AC-005** | AC-005's 40-occurrence test asserting 40 lines and no elision token |
| **Anti-pattern 9** — a badge, tick, shield or "verified" mark asserting the documentation is checked for correctness or comprehension | anti-pattern 9 | **AC-006** | AC-006's `rg` over the added prose |

**Not this story's to render, stated so nothing is quietly claimed.** `narrative-page`,
`narrative-scoped-page` and `narrative-tree-index` are markdown surfaces this story does not
author or change; the threshold that governs their scope bands (≤ 3 scopes × ≤ 25 rendered
lines) is a **review** rule by decision, and gating it would take HS-P0021's job (`_design.md`
`## Open questions` item 2). `fail-empty-tree` and `fail-long-path` are the slice-mate's states.
`design.capture` is undeclared, so no perceptual review will ever look at any of this — which
is precisely why the composition rows above are ACs with string assertions behind them rather
than prose a reviewer is trusted to notice (`_design.md` opening, "this file is the only record
these decisions will ever have").

## Error conditions

| id | condition | required behaviour | the wrong implementation it forbids |
| --- | --- | --- | --- |
| **EC-001** | A page under `docs/` contains a hidden-content marker | A problem line, a non-zero exit, and the marker counted in the terminal `bail!`. Never a warning, never a `skipped` line, never a note on stdout that a green run buries | The `RUNBOOK.md:916-925` shape: a documentation step that printed warnings and exited 0 while two documents vouched for it. This story's whole reason for existing is not to reproduce it one layer up |
| **EC-002** | A page under `docs/` cannot be read (permissions, invalid UTF-8, a broken link) | A hard error that names the path and fails the run — `?`-propagated, not swallowed. A page the checker could not read is a page the checker did not check | A `filter_map(Result::ok)` or `unwrap_or_default()` that turns an unreadable page into a clean one. `xtask/src/lint_constitution.rs:29-44` states this posture for citations and is the precedent to copy |
| **EC-003** | A marker appears inside a fenced block, inside an HTML comment, or inside a line that also carries prose explaining it | Still a problem, at that line, with no exemption path. The scan is line-based over the whole page | A fence-aware or comment-aware scan that skips those regions. It is the most plausible "improvement" available and it re-opens DT-7 for anyone who can type three backticks |
| **EC-004** | Two different markers occur on one line, or one marker occurs twice | One problem per occurrence, each naming the token it matched, so the count in the `bail!` is the number of things to fix rather than the number of lines to visit | A `lines().any(...)` short-circuit that reports one problem per page, which makes a six-fold page look like a one-fold page |
| **EC-005** | The tree is missing, or exists and holds no pages | **Not this story's arm.** The vacuity guard `bail!`s before any check runs (`_design.md` `## States`, *Empty*; `xtask/src/lint_constitution.rs:175-177`), and this story adds no second arm and no "0 markers, clean" line that could be printed over an empty tree | A marker scan that returns `Ok(())` on an empty iterator and lets a vacuous green through. The guard is `narrative-checker-mounted-with-pinned-path`'s; the failure mode is shared, so it is named here rather than assumed |

## Non-functional

| id | requirement | why, and where it is checked |
| --- | --- | --- |
| **NF-001** | **Zero new dependencies.** No crate is added to `xtask/Cargo.toml`, in either graph | DR-12's standing trade at `xtask/Cargo.toml:16-21` deliberately keeps `rusqlite` and `sqlx` out of xtask's dev graph because every `cargo xtask ci` would build them. `_design.md` `## What it costs a caller` states the project-wide budget as zero, and a marker scan is `str` work |
| **NF-002** | **No second traversal.** The scan runs over page text the fence walk has already read; it adds no `read_dir`, no second file read and no second pass over the tree | `_design.md` `## What it costs a caller`; the cost class `xtask/src/affected.rs:28-36` already argues finishes inside the time cargo takes to decide `xtask` is up to date |
| **NF-003** | **No panic path.** No `unwrap`, `expect`, indexing that can panic, or `unreachable!` in the scan. Problems accumulate; failure is a `bail!` at the end | The workspace `[lints]` table and `standards/rust/00-prime-directives.md`; `cargo clippy -D warnings` inside `cargo xtask ci`. Test code carries the same scoped `allow` with the same `reason` as `xtask/src/lint_constitution.rs:829` |
| **NF-004** | **Byte-identical output on all three runners.** Paths in messages are composed with `/` separators regardless of host, so the message form is the same on Windows and Linux | The existing primitive builds `{ATOM_DIR}/{file}` this way (`xtask/src/lint_constitution.rs:606-607`); `observed-failure-falsification` records verbatim output, and a platform-dependent separator would make that record wrong on two of three runners |
| **NF-005** | **No consequence for consumers.** No public API, no feature, no `cfg`, no target, no MSRV movement, no wasm32 effect, no `Send` bound. `xtask` is `publish = false` and never in anyone's dependency graph | `_design.md` `## Visibility and stability`, `## What it costs a caller`. ADR-0001 is untouched by construction |
| **NF-006** | **The check does not trip itself, and its docs survive the existing gates.** The module documents the tokens in prose; the module lives in `xtask/src/`, outside `TREE`, so naming them is safe. `cargo xtask lint-constitution` and `cargo test -p xtask --doc` still pass | Project DoD item 7; `_storymap.md` "Coverage", DoD-7 bullet — each story owes the "what this does not verify" section's *existence* for whatever it adds, while `documented-blind-spots-and-their-proofs` owns its contents |
| **NF-007** | **Determinism.** Same tree in, same problem list out, in the same order, with no dependence on filesystem iteration order beyond the sort the walk already applies | AC-005's ordering assertion; `xtask/src/lint_constitution.rs:169-198`'s accumulate-then-report shape |

## Implementation notes (non-prescriptive)

These are the shapes that were already reasoned about while writing the sections above. None is
binding except where it restates a decision already cited.

- **One pure function, called once.** Something of the shape
  `fn check_hidden_markers(page: &str, text: &str, problems: &mut Vec<String>)`, called from the
  per-page body of the `check_fences` analogue `fence-discipline-and-allowance-list` adds. Purity
  is a *shape* obligation rather than a taste one: the negative fixture is a page-shaped `&str`
  and cannot be a file under `docs/`, so anything that only accepts a path is untestable here
  (`_decomposition.md`, Note 2, makes the same argument about the page→module derivation).
- **The token literals, verbatim from `_design.md` `## Signatures`** — `"<details"`,
  `"<summary"`, `"{{#tabs"`, `"{{#tab "`, `"{{#endtabs"`, ``"```admonish"``, `"<!-- tab"`. The
  trailing space in `"{{#tab "` is load-bearing: without it the token shadows `"{{#tabs"` and a
  single `{{#tabs}}` line reports twice. All seven are already lowercase, which AC-004's pin
  asserts so that the case-folding below stays correct after an edit.
- **Case folding.** `line.to_ascii_lowercase()` once per line, then `match_indices` per token,
  is enough and is UTF-8-safe: `to_ascii_lowercase` leaves non-ASCII bytes alone, so byte
  offsets in the lowered copy still line up with the original. The message reports the token as
  authored (`` `<details` ``), not as matched, which is what `_design.md` `## States` shows.
- **Line numbers are 1-based**, matching every other problem line in the repository
  (`xtask/src/lint_constitution.rs:605-660`). The column is not reported, because the design's
  location form is `{path}:{line}` and adding a column would change the primary element's shape.
- **The negative fixture.** Take `_design.md` `## The doctest`'s page verbatim — including
  `use happenstance_core::MemoryEventStore;`, which is the *corrected* spelling (mock finding 1;
  `memory` is private at `crates/happenstance-core/src/lib.rs:103`, re-exported at `:122`) — and
  hold it as one `&str` constant, with a second constant adding the wrapper. Deriving the wrapped
  form from the clean one at runtime is better than two hand-copied constants that can drift.
- **Where the "what this does not verify" lines go.** First in the module's docs, not last
  (`xtask/src/lint_constitution.rs:9-28`; RS-81-1). Three limits belong to this check: a marker
  spelling absent from the set; disclosure produced outside the pinned tree; and that the scan
  reads *source* and cannot know what a renderer does with it.
- **What not to reach for.** No refactor of `xtask/src/lint_constitution.rs` or
  `xtask/src/constitution.rs` to share a scanner — RS-81-3 scopes a scanner to the directory
  whose behaviour it constrains, and a shared abstraction makes one error message answer two
  questions (`_storymap.md`, "A refactor of `lint_constitution.rs` …"). No regex crate. No
  second walk. No early return.

## Tests and CI (merge gate)

Grounded in `_decomposition.md`'s testing brief — AC-006's row ("*Static* if `_design.md`
forbids hidden/tabbed content outright — a lint asserting the forbidden markers do not appear
anywhere under the pinned tree, tested the same way as AC-004"), and its merge-gate command
list, narrowest to widest.

| tier | command / path | proves |
| --- | --- | --- |
| unit | `cargo test -p xtask` → `xtask/src/lint_narrative.rs` `#[cfg(test)] mod tests` | AC-001 (one problem per occurrence; marker and fence problems in one list, source order), AC-002 (wrapped fixture fails by file and line; unwrapped is clean), AC-003 (case variants; marker inside a `text` fence; `docs/README.md`'s real text clean), AC-004 (the RS-81-5 set-pin naming which token moved; all tokens lowercase), AC-005 (exact composed line; 40 occurrences → 40 lines, no elision; ordering), AC-006 (no input makes a marker pass) |
| static — existing gates not regressed | `cargo xtask lint-constitution` | NF-006, AC-005's docs half: the pre-existing constitution checker still passes and the new module's added docs did not break the corpus checks (`_storymap.md`, Note 8's reason for not sharing code) |
| doc | `cargo test --locked -p xtask --doc` | NF-006: the module docs, including the "what this does not verify" section this story adds to, compile. This is also the compile mechanism the fixture page rides on (`_decomposition.md` merge-gate list, line 1) |
| gate — step reachable, unfiltered | `cargo xtask ci --fast` | AC-001's mount half: the narrative step is in `REQUIRED` with `probe: None`, so `run_fast` runs it unfiltered (`xtask/src/main.rs:853-860`), and the marker scan runs inside it over the real tree with exit 0. This is the bar for a non-terminal story (`project.md` DoD item 6, `.redkiln/config.yaml`) |
| gate — story grain | `cargo xtask affected --base main` | That a `docs/`-only change still selects a run containing this check. The selector arm is `narrative-tree-story-grain-selection`'s; this story consumes it, and a gap here is invisible until a later story's own gate compiles nothing (`_decomposition.md` testing brief, AC-009 item 3) |
| gate — full | `cargo xtask ci` | The merge gate of record (CLAUDE.md, "Commands"): fmt, clippy `-D warnings` (NF-003), tests, wasm32, docs, `spec-trace`, packaging. Run before the project is called done (`project.md` DoD item 6) |
| review — not automatable | diff read against AC-006 | No allowance path, no `cfg`/env escape, no badge or teachability claim. `rg` is the instrument; the judgement is a human's, and `_design.md` says so — `design.capture` is undeclared, so no perceptual review exists to catch it later |

**No conformance rule is added.** Nothing here touches a port, a store, a value type or the
testkit, so no rule in `crates/happenstance-testkit/src/suite.rs` can observe it. The equivalent
obligation — a check with a named wrong implementation that fails today — is discharged by
AC-002 and AC-004.

## Risks and coupling (PR-scoped)

| Risk | Note, and what absorbs it |
| --- | --- |
| **Three stories edit one function.** This story, `fence-discipline-and-allowance-list` and `narrative-checker-mounted-with-pinned-path` all write into `xtask/src/lint_narrative.rs`'s fence walk | Absorbed by the slice: all three are implemented in one context, strictly sequentially, this one **third** (`_storymap.md`, "Merge order" 2.5). Implementing it in a separate context would edit one function twice — the explicit reason the story map put it in this milestone |
| **The rule gets "improved" into the falsification branch.** A later reader sees project AC-006's disjunction, notices the second arm is untried, and builds it | Absorbed by the Context pack, which states the rejection inline with its three reasons, and by AC-004's pin. `_design.md` D2 argument 2 is the load-bearing one: the observation would establish the property for one construct and one extractor, and nothing would notice it regressing |
| **The whole-page scan rejects a page that legitimately needs to name a token.** A future page under `docs/` documenting this very rule cannot quote `<details` | Stated, not hidden. The checker's own module docs are outside `TREE`, so the rule documents itself safely. A page that genuinely needs the token petitions through HS-P0021 with an allowance in its own change and its own falsification (`_design.md` `## Open questions` item 3) — **and this PR leaves no hook for it** |
| **DT-8 coupling.** A blanket ban makes DT-8 moot inside the pinned tree only; HS-P0021 still owns the aside/constraint line everywhere else | Stated in the Context pack and in AC-006. The seam is that this story owns the machine inside `TREE`; HS-P0021 owns the rule (`initiative.md:492`; `_design.md` D2, "What this does not decide") |
| **`docs/README.md` is already in the tree.** It is scanned from the first run, and if it ever gains a fold the gate breaks | Desired, and verified now: the file as it stands carries no marker (AC-003's test pins that). The index gains rows from milestone 1, and those rows are a two-column table, not a nested disclosure (`_design.md` `## Composition`, `narrative-tree-index`) |
| **The fixture page's own compile.** The negative fixture derives from `## The doctest`, which the mock found did not compile before correction | Absorbed by using the corrected spelling (`happenstance_core::MemoryEventStore`) and by the fixture being a `&str` in this story's tests — it is never registered with the harness, so it is not compiled by `cargo test --doc` and cannot break AC-002's compile step. The *positive* fixture page is `pinned-narrative-tree-and-compiling-step`'s and is the one that compiles |
| **Cross-branch merge.** `initiative/from-contract-to-published-library` is unmerged and diverges on `spec/SPECIFICATION.md` by 521 lines | Not a coupling for this story: it touches no clause, no `SPECIFICATION.md` text and no `crates/` file. The residual is a possible conflict in `xtask/src/main.rs`'s module list, which is one line (`project.md`, "Cross-branch note") |
| **A green run is read as evidence.** The project's largest risk, and this story adds a check that passes on a tree with no teaching pages in it | Absorbed by AC-006 and by NF-006's "what this does not verify" obligation. The scan's silence means no page under `docs/` carries a fold — nothing more, and the module docs must say so in those words |

## Dependencies

**Blocks on** — `depends_on` exactly as the story map records it, and as the item's real
`blocked_by` link carries it:

- **`fence-discipline-and-allowance-list`** (HS-S0139). Direct and hard: this story's marker
  scan is *called from* the fence walk that story writes, pushes onto the same `Vec<String>` and
  is reported by the same `bail!`. Without it there is no walk to extend, and adding a second
  one is the named wrong move (`_decomposition.md:218-219`).
- Transitively, through that edge: **`narrative-checker-mounted-with-pinned-path`** (the module,
  `TREE`, the vacuity guard, the `REQUIRED` step, the banner) and, before it,
  **`pinned-narrative-tree-and-compiling-step`** (the tree and the fixture page this story's
  negative fixture derives from). Both are earlier in the same project's merge order and neither
  is restated as a direct edge, because the story map's order already guarantees them
  (`_storymap.md`, "Merge order" 1.1, 2.3, 2.4).

**Unlocks** — honestly, **no story declares a `blocked_by` on this one.**
`observed-failure-falsification` and `documented-blind-spots-and-their-proofs` depend on the
fence walk and the other checks, not on the marker scan (`_storymap.md`, milestone 4 rows). What
this story unlocks is not a story but a **closure**: it completes milestone
`narrative-checker-discipline`, discharges project AC-006 and DR-08, and turns initiative DoD
scenario 13 green by its second arm. Downstream, it is what lets HS-P0021 treat DT-8 as a rule
about surfaces *outside* `docs/` rather than a rule it must also enforce inside it.

## Anchors (progressive disclosure)

Link, do not paste. Every path below was confirmed present in this worktree.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `.bklg/docs-that-teach/checked-documentation-surface/_design.md` — `## Pattern decision` D2 and `## Sign-off` | The signed-off resolution of DT-7 with its three rejection arguments and its threshold. This story implements clause three and re-decides none of it; the sign-off row records that a human read D2 as written | First, before writing any code — and again the moment anyone proposes building AC-006's second arm | AC-001 |
| `.bklg/docs-that-teach/checked-documentation-surface/_design.md` — `## Signatures` and `## Items` | The seven token literals verbatim, and `HIDDEN_MARKERS`' visibility, feature and semver row. Copying the tokens from anywhere else is how the trailing space in `{{#tab ` gets lost | When declaring the `const` | AC-004 |
| `.bklg/docs-that-teach/checked-documentation-surface/_design.md` — `## States`, `## Composition`, `## Hierarchy`, `## Density budget`, `## Anti-patterns` | The message text verbatim, the failure report's assembled shape, the primary/secondary/recessive split, the 80-column and ≤ 48-character budgets, and anti-patterns 1/7/8/9. These are the composition ACs' source of truth | Before writing the exact-string assertion in AC-005's test | AC-005 |
| `.bklg/docs-that-teach/checked-documentation-surface/_design.md` — `## The doctest` | The fixture page in full, already corrected for the mock's finding 1, and the sentence assigning the negative fixture to this story with "without it the rule is decorative" | When building the fixture constants | AC-002 |
| `xtask/src/lint_constitution.rs:601-673` (`check_fences`) | The fence-walk shape the slice-mate copied and this story extends: per-fence iteration, `problems.push`, no early return | While writing the call site | AC-001 |
| `xtask/src/lint_constitution.rs:605-660` and `:169-199` | The `  {path}:{line} — {message}` primitive with its `/` separator, and the accumulate-all / print-all / count-last contract ending in `bail!` | While composing the message and again while asserting on it | AC-005 |
| `xtask/src/lint_constitution.rs:827-878` (`#[cfg(test)] mod tests`) | The house test shape this story must follow: pure functions fed `&str` inputs, no fixture directory, no `tempfile`, and the scoped `allow(clippy::unwrap_used)` with its `reason` | Before writing the first test | AC-002 |
| `xtask/src/main.rs:64-70`, `:105`, `:799-826`, `:853-860` | The mount point: the bin-crate module list, `REQUIRED`, `lint_steps`, `steps_named`'s panic, and `run_fast`'s unfiltered pass. Read it to confirm this story adds **nothing** here | When verifying the diff against AC-001's "no new step" clause | AC-001 |
| `.bklg/docs-that-teach/checked-documentation-surface/_decomposition.md` — architecture AC-006 (`:75-79`), Note 5, ux DT-7 (`:424-465`), testing AC-006 (`:608-620`) | The enforcement site ("the same fence walk as AC-004"), why no allowance list is symmetric with `IGNORE_ALLOWANCES`, the evidence table whose one *Unverified* row settles DT-7, and the test shape for the forbidden branch | Before AC-003's tests, and whenever the "why not tabs" question is reopened | AC-003 |
| `.bklg/docs-that-teach/checked-documentation-surface/project.md` — AC-006, DR-08, DoD items 3/7/8 | The project-grain criterion this story traces to, the "assertion is not acceptable; the demonstration is" wording, and the two project-wide obligations every story owes | At the start, and again at the ledger flip | AC-006 |
| `standards/rust/81-checks-that-cannot-be-types.md` — RS-81-1 and RS-81-5 (`:335-341`) | State the limit then execute it; and derive the fact, keep the hand-written intention, make the failure say **which one moved**. AC-004 is RS-81-5 applied to a token set | While writing the set-pin and the "does not verify" section | AC-004 |
| `.bklg/docs-that-teach/_discovery/distillation/interaction-patterns.md:213-216`, `:218-225`, `:244-252` | The single *Unverified* claim the whole decision rests on, the honest statement that the fanout genuinely suits tabs, and the in-house fold-drift precedent (a sixth event type, a fold updated, a query forgotten) | When someone argues folds are safe, or when writing the module docs' rationale line | AC-001 |
| `.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md:148-166`, `:207-215` | Persona 2's goal — understanding *why* the port is shaped as it is — and the journey gap this protects. AC-001's GIVEN comes from here and should not be paraphrased into "a developer" | When writing or reviewing AC-001's framing | AC-001 |
| `.bklg/docs-that-teach/initiative.md:455-461`, `:491-492` | DoD scenario 13 in full (the arm this story satisfies), DT-7's statement, and DT-8's assignment to HS-P0021 — the seam this story must not cross | At the ledger flip, and before touching anything that looks like DT-8 | AC-006 |
| `docs/README.md` | The one file already under `TREE`. AC-003 pins that it is clean today, and `:25-29` is the paragraph milestone 1 must update — read it to confirm this story leaves the tree unchanged | Before running `cargo xtask narrative` for the first time | AC-003 |
| `.bklg/docs-that-teach/checked-documentation-surface/_storymap.md` — activity C, the `narrative-checker-discipline` rationale, "Merge order", "Coverage" | Why the three checker stories are one surface and one context, this story's position (fifth overall, third in the slice), and the coverage row that makes AC-006 this story's alone | Before starting, to confirm the slice-mates have landed | AC-001 |
| `xtask/Cargo.toml:16-21` | DR-12's standing dependency trade, written as a comment in the manifest. NF-001's zero-new-dependency budget is this line, not a preference | If any part of the implementation starts wanting a crate | AC-005 |
| `.bklg/docs-that-teach/checked-documentation-surface/hidden-content-resolution/discover.md:55` | The named wrong implementation in one sentence: a `<details>` block whose inner claim is broken and the gate stays green | When phrasing AC-002's test names | AC-002 |
| `RUNBOOK.md:916-925` | The in-house precedent: a documentation step that printed `skipped` on all three runners while two documents vouched for it. It is the reason EC-001 forbids a warning-shaped outcome | Whenever a softer failure mode is proposed | AC-001 |
| `.bklg/docs-that-teach/checked-documentation-surface/design/mock.html` | The dated instrument that disproved four figures in `_design.md` and **deliberately still shows the pre-correction numbers**. Read it to understand why the design and the mock disagree; do not "fix" it | Only if a density or banner figure looks wrong — the design wins, the mock explains | AC-005 |

## Clarifications resolved during spec

1. **The AC ids are exactly the six the front half enumerated** — AC-001 through AC-006. None
   was added or dropped. The ledger carries the same six.
2. **AC-006 (project) resolves to its *first* arm, and the second arm is not built.** The
   disjunction in `project.md` AC-006 and DR-08 is closed by `_design.md` D2, signed off
   2026-08-17 with no conditions. Every "or observed inside a hidden panel" clause in the briefs
   is therefore a branch not taken, and the spec says so in three places on purpose — Context
   pack, PR boundary, Risks — because the tempting failure is a later reader building it.
3. **`observed-failure-falsification` is not this story's dependency and does not cover it.**
   That story breaks a claim inside a *fence* on the positive fixture page; this story's
   observation is the negative fixture at unit grain. The two are different instruments aimed at
   different escapes, and neither substitutes for the other.
4. **The checker module's filename is `xtask/src/lint_narrative.rs`**, not invented here: it is
   fixed by the slice-mate's spec (`narrative-checker-mounted-with-pinned-path/spec.md:70`) and
   cannot be `xtask/src/narrative.rs`, which `_design.md` `## Signatures` reserves for the
   lib-target harness. The tests this spec names live in that file's `#[cfg(test)] mod tests`.
5. **The negative fixture is test material and no file lands under `docs/`.** A committed page
   carrying the wrapper would fail `cargo xtask ci` forever. `_decomposition.md`'s note "Fixture
   pages live inside this project, not in the corpus" leaves *retention* to the implementer; here
   the question does not arise, because the fixture never becomes a file.
6. **The scan includes fenced blocks and `docs/README.md`, and this was a decision.** A
   fence-aware scan is the most plausible refinement and is refused (EC-003): a marker quoted in
   a fence still renders as an instruction to fold, and there is no allowance path to exempt it.
   The checker's own module docs are outside `TREE`, so the rule can document itself.
7. **The one-problem-per-occurrence rule was made explicit** (EC-004). `_design.md` `## States`
   says "one line per occurrence"; the spec spells out that two markers on one line are two
   problems, so the `bail!` count is the number of things to fix.
8. **The threshold (≤ 3 scopes × ≤ 25 rendered lines), the 250-line page cap and the
   40-character H1 cap are not gated by this story.** They are review rules by decision
   (`_design.md` `## Open questions` item 2); the only density rule that became a gate rule is
   the path-length budget, and that belongs to the slice-mate's pinning check.
9. **The state-family interaction invariants were translated to a terminal medium explicitly**
   rather than dropped as inapplicable. "In place" is *inside the running step*; "focus and
   scroll" is *source order*; "keyboard reachability" is *a typed command with no prompt and no
   probe that can decline*. Each translation carries the AC that verifies it, so none of the
   five state invariants is unowned.
10. **Nothing in this story is owed an ADR.** The grounding pass found no Accepted decision atom
    under `.kb/decisions/` governing gate structure, documentation trees or fence discipline
    (`_storymap.md`, "Stories this map deliberately does not contain"), and no `[FROZEN]` clause
    is touched. `_design.md` is the design record DT-7's resolution requires, and it already
    exists and is signed off.
