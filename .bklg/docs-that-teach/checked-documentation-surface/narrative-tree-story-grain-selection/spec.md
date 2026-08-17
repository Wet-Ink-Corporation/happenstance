---
item: HS-S0137
stage: spec
created: 2026-08-17T13:16:01.618Z
updated: 2026-08-17T13:16:01.618Z
template_sig: 87bbf1d0
rendered_sig: fe51924f
---

# Spec — A prose-only change selects the package that compiles it

## Scope lock

| What | Path |
| ---- | ---- |
| Initiative (gold source) | `.bklg/docs-that-teach/initiative.md` — BR-01, BR-12, DoD scenario 1 |
| Decomposition | `.bklg/docs-that-teach/_decomposition.md` — the DAG; HS-P0020 first, no inbound edge |
| Project | `.bklg/docs-that-teach/checked-documentation-surface/project.md` — DR-01, DR-10, AC-009 |
| This spec | `.bklg/docs-that-teach/checked-documentation-surface/narrative-tree-story-grain-selection/spec.md` |
| Key briefs | `.bklg/docs-that-teach/checked-documentation-surface/_decomposition.md` — architecture Note 1 **CR-5** (the landmine), Note 9 (decisions left open, with the consequence attached), testing brief AC-001/AC-009 rows |
| Signed-off design (binding) | `.bklg/docs-that-teach/checked-documentation-surface/_design.md` — **D1** (`TREE = "docs"`, the markdown *is* the render) and its "Consequences that must land in the same change" at `:190-197` |
| Grounding | `.bklg/docs-that-teach/checked-documentation-surface/_grounding.md:191-199` — no Accepted decision atom governs gate structure; the binding constraints here are conventions |
| Story map row / merge order | `.bklg/docs-that-teach/checked-documentation-surface/_storymap.md` — milestone `compiled-narrative-tree`, story 2 of 2, lands **after** `pinned-narrative-tree-and-compiling-step` in the same context |

## One-line PR slice

Extend `xtask/src/affected.rs`'s selection arm so a change confined to the narrative tree
selects the `xtask` package, with unit tests in both directions, so the story-grain gate does
not compile nothing on the very PR that broke a page.

## Executive summary

The slice-mate lands the tree at `docs/` and compiles every Rust fence in it as doctests of
the `xtask` **lib** target. That makes `docs/` source for exactly one package — and
`xtask/src/affected.rs` does not know it. Today the path falls through
`affected_packages`' outside-every-member arm (`xtask/src/affected.rs:209-225`), is matched by
`is_inert`'s `INERT` list (`:250`), and selects **nothing**: `cargo xtask affected --base main`
on a prose-only diff runs fmt, clippy and tests for zero packages and prints
`affected gate passed`.

The delta is small and entirely inside one file: one arm extended, one prefix removed from
`INERT`, three stale comments corrected, and unit tests in both directions. Its value is not
small, because `.redkiln/config.yaml:40` wires `cargo xtask affected --base {{base}}` as the
**story grain** for every subsequent story in this initiative. Until this lands, each of the
eight downstream stories' own advance gate is blind to prose-only changes — the failure is
green, silent, and structurally identical to the one `affected.rs:12-27` says the whole module
exists to avoid ("naming too few reports green over an untested regression").

This story adds **no** `Step`, no subcommand and no member of `REQUIRED`. It changes the
behaviour of an already-mounted selector.

## Context pack

The load-bearing decisions, stated as decisions. Everything deeper is an anchor.

**1. The tree is `docs/`, and that decision is already made.** `_design.md` **D1** chooses
`docs/` repurposed, with `const TREE: &str = "docs"` in `xtask/src/narrative.rs` and no second
rendered surface — "the markdown is the render." This story does **not** re-open the path
choice, the hosting shape, or DT-7. It teaches the selector the fact D1 established, and
`_design.md:190-197` names that extension as a consequence that "must land in the same change."

**2. The prefix is a literal in `affected.rs`, not a shared constant — because the two targets
do not share modules.** `xtask` has two targets. The harness is a **lib**-crate module
(`mod narrative;` at `xtask/src/lib.rs`, architecture brief CR-1), and `affected` is a
**bin**-crate module (`xtask/src/main.rs:64`, CR-2). A private `const TREE` in the lib target is
not visible to the bin target, and inventing a `pub` seam across the two to share a
seven-character string would buy less than it costs. The decision: the arm matches `"docs/"`
directly, with a comment naming `xtask::narrative::TREE` as the counterpart it must move with —
in the exact register of the existing comment at `:215-220`, which does the same job for
`standards/rust/`. State the duplication and its reason; do not hide it behind an abstraction.

**3. Remove `"docs/"` from `INERT`; do not leave it shadowed.** The new arm is evaluated before
`is_inert` in the same `else if` chain (`:214`, `:222`), so leaving `"docs/"` on the list at
`:250` would still *work* — and would leave a dead prefix that the next person to reorder that
chain re-arms. The in-repo precedent is explicit and is the same fact: `standards/rust/` is
**deliberately absent** from `INERT`, with the reason recorded at `:245-247` — "Adding it here
would silently un-compile the corpus." `docs/` now has that same property. Removing it and
extending that doc comment to say so is the deliverable; keeping it is the named wrong
implementation.

**4. Three comments become false and must be rewritten, which is deliverable rather than tidying.**
`affected.rs:210-213` says in prose that "`docs/`, `.github/`, `.bklg/`, `.kb/` — reaches no
package." `is_inert`'s doc comment (`:232-247`) explains which trees are inert and why. And the
empty-selection branch at `:136-138` justifies its own output with "a docs-only story genuinely
has no package to compile, and saying so is the difference between 'nothing to do' and 'the gate
did not look'" — which is exactly the sentence D1 falsifies: after this change a docs-only story
*does* have a package to compile, and the branch's reasoning must say that the emptiness it
reports no longer covers the narrative tree. All three are load-bearing, because this module's
judgements are recorded in comments precisely because a green run cannot show that the mapping
returned too few names.

**5. `a_docs_only_change_selects_nothing` does not need re-pointing — its name needs correcting.**
`_decomposition.md` Note 9 and `_design.md:194-196` both warn the test may need a path that is
still genuinely inert. Verified against the code: the test at `xtask/src/affected.rs:650-654`
asserts on **`RUNBOOK.md`**, not on any `docs/` path, so it stays green as written. What it no
longer is, is a test about `docs/`. Rename it to say what it actually asserts (a top-level
prose file reaches no package) and keep the assertion. Deleting it is forbidden — it is one of
the two anchors of the widening posture.

**6. Both directions, or the test is decorative.** `affected_packages` is `pub(crate)`
specifically so a test can drive the mapping "without a git repository or a compiler", because
"the whole risk in this module is that this function returns *too few* names, and that is not
observable from a green run" (`:183-190`). The two shapes to mirror already exist for the
constitution: `a_constitution_atom_selects_xtask` (`:688-696`) proves the arm fires, and
`the_constitution_arm_does_not_widen_to_all_prose` (`:699-701`) proves it is a rule about one
directory rather than about every tree of markdown. This story owes one of each, and must leave
`the_relocated_trees_stay_inert` (`:660-675`) and `an_unrecognised_path_widens_rather_than_narrows`
(`:645-649`) green and untouched.

**7. The mount already exists; this story changes what is mounted, not where.** `mod affected;`
(`main.rs:64`), the dispatch arm (`main.rs:653-658`), the help line (`:729`) and
`.redkiln/config.yaml:40` are all in place. RS-80-1's "add a check as a `Step` in `REQUIRED`,
and reach it by name" is the slice-mate's obligation for the *compiling* step
(`standards/rust/80-the-gate.md:11`); reaching for it here would produce a second, redundant
gate step for a selector that is not a check.

**8. The persona-journey slice, and the humane outcome.** The user is a **contributor running
the gate** and a **reviewer reading its output** (`_storymap.md`, preamble) — this project ships
no runtime surface. The outcome is not "the arm exists": it is that a contributor who edits only
a narrative page sees `affected --base main` name `xtask` and actually compile the page's fences,
and that a reviewer reading `1 file(s) changed against \`main\`` followed by a package list can
tell the gate looked at something. The failure this replaces is a gate that says
`affected gate passed` having compiled nothing.

**9. What this story must not claim.** Selection is not compilation. Naming `xtask` proves only
that the package will be built and tested; whether a fence inside a page is compiled is the
slice-mate's `#[cfg(doctest)]` harness, and whether the prose *teaches* is HS-P0024's and is
proven by nothing in this project (project DoD items 7 and 8). Say the first in the code, in the
"what this does not verify" register of `xtask/src/lint_constitution.rs:9-28`; do not say the
second anywhere.

**10. Authority.** No Accepted decision atom under `.kb/decisions/` governs gate structure,
documentation trees or fence compiling (`_grounding.md:191-199`); no ADR is written or amended
here, and no `[FROZEN]` clause is touched. The binding constraints are conventions: CLAUDE.md,
`standards/rust/80-the-gate.md` and `standards/rust/81-checks-that-cannot-be-types.md`.

## Integration contract

- **Archetype**: `capability` — user-observable on the story grain, through
  `cargo xtask affected --base main`, which is the invocation `.redkiln/config.yaml:40` runs at
  every `redkiln advance` seam.
- **Slice / milestone**: `compiled-narrative-tree`. Slice-mate:
  `pinned-narrative-tree-and-compiling-step` (HS-S0136), which lands first in the same context
  and creates `docs/`'s first registered page, `xtask/src/narrative.rs` and the `REQUIRED`
  compiling step. This story is second and last in the slice.
- **Mount point**: `xtask/src/affected.rs` — the outside-every-member arm of
  `affected_packages` (`:209-225`) and `is_inert`'s `INERT` list (`:248-260`). It is reached in
  production by `affected::run` (`:112`) via the `Some("affected")` dispatch at
  `xtask/src/main.rs:653-658`, declared at `xtask/src/main.rs:64`, and invoked by
  `.redkiln/config.yaml:40`. Nothing new is mounted; an existing mount changes behaviour.
- **Wires into**: `affected_packages`' existing contract — `changed: &BTreeSet<String>` of
  repo-relative paths and `members: &[Member]` (`:188-191`); `close_over_dependents` (called at
  `:229`, defined at `:273`), which closes the selection over dependents and needs no change
  because nothing depends on `xtask`; the empty-selection branch at `:135-142`, whose output this
  change makes unreachable for a tree-confined diff; the module's `WORKSPACE_WIDE` escape
  (`:73`, `:195-197`), which must keep precedence over
  the new arm; and, as a *fact* rather than a symbol, `xtask::narrative::TREE` in the slice-mate's
  lib-crate module (`_design.md` `## Signatures`).
- **Renders surfaces**: **none** of the five ids in `_design.md` `## Surfaces`. The output this
  story changes is the `=== affected packages ===` banner and package list at
  `xtask/src/affected.rs:130-135`, which the design deliberately does not claim as a surface —
  its two gate surfaces (`gate-narrative-compile-step`, `gate-narrative-checker-step`) belong to
  the slice-mate and to milestone 2. This story must not invent an affordance there; it changes
  which package names appear in a list that already exists.
- **Public items**: none. No item in `_design.md` `## Items` is implemented or changed by this
  story — every entry there is `xtask::spec_trace::*` or `xtask::narrative::*`. `affected_packages`
  and `is_inert` are already `pub(crate)` / private and stay so. An empty row here is correct,
  not an omission.
- **Conformance rule(s)**: none, and this is not adapter-observable. Nothing in this story
  touches a port, a value type or `crates/happenstance-testkit/`; no adapter can observe the
  story grain's package selection. Its equivalent instrument is the `#[cfg(test)]` module at
  `xtask/src/affected.rs:597`, where the wrong implementation is named by test rather than by
  mutant registry.
- **Clause(s)**: none discharged, none amended. `spec/SPECIFICATION.md` is unread by this change
  and unedited by it; the initiative is additive and says so (`project.md`, "Out of scope").
- **Advances DoD scenario**: initiative **DoD scenario 1** — "@smoke — the teaching survives a
  clean checkout … not as a separate manual step someone remembers to do"
  (`.bklg/docs-that-teach/initiative.md:412-415`), via project **AC-009**'s third and
  least-visible invocation path. Not to green on its own: scenario 1 also needs the slice-mate's
  `ci` / `ci --fast` paths.

## PR boundary

```
xtask/src/affected.rs
.bklg/docs-that-teach/checked-documentation-surface/narrative-tree-story-grain-selection/**
```

**In this PR**

- The selection arm at `xtask/src/affected.rs:214` extended so a path under the narrative tree
  selects `xtask`, with the comment stating why and naming its lib-target counterpart.
- `"docs/"` removed from `INERT` (`:250`), and `is_inert`'s doc comment extended with the reason
  it is now deliberately absent, in the shape of the `standards/rust/` paragraph at `:245-247`.
- The three comments falsified by the change corrected: `:210-213`'s list of trees that reach no
  package, `is_inert`'s doc comment, and the empty-selection branch's justification at `:136-138`.
- Unit tests in both directions, plus the rename of `a_docs_only_change_selects_nothing` to what
  it actually asserts.
- The recorded observation of `cargo xtask affected --base main` selecting `xtask` on a
  tree-confined change, in this story's own folder.

**Explicitly not in this PR**

- The tree, its first page, `xtask/src/narrative.rs`, the `mod narrative;` line and the
  `REQUIRED` compiling step — slice-mate `pinned-narrative-tree-and-compiling-step`.
- The `docs/README.md:25-29` rewrite that lists the trees the gate reads by path. It is the same
  slice's obligation (`_design.md:194-197`) and belongs with the story that creates the tree;
  this story must not contradict it.
- Any new `Step`, subcommand, `lint_steps` member or `probe`. Also: adding the
  milestone-2 checker to `affected::run`'s unconditional list (`:120-125`) — that decision is
  `_decomposition.md` Note 9's and belongs to `narrative-checker-mounted-with-pinned-path`,
  which is the story that creates the checker.
- Any teaching page, any change under `crates/`, and any edit to `spec/SPECIFICATION.md`.

**Merge DoD**: `cargo test -p xtask` green with the new bidirectional tests present,
`cargo xtask affected --base main` observed selecting `xtask` on a tree-confined change with the
output recorded, and `cargo xtask ci --fast` green — the bar `.redkiln/config.yaml` sets for a
non-terminal project (`project.md` DoD-6).

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| -------------------- | ------- | ------------- |
| A path under the narrative tree selects `xtask` | `affected_packages` inserts `"xtask"` into `direct` for any changed path beginning with the tree prefix, before the `is_inert` branch is reached. Same mechanism, same reason and same comment register as `standards/rust/`: the tree's fences are compiled only as that crate's doctests | `xtask/src/affected.rs:209-225` (the arm), `:214-221` (the precedent and its stated reason) |
| The prefix is a literal, not a cross-target import | The counterpart const lives in the lib target (`xtask::narrative::TREE`); `affected` is a bin-crate module, so the two cannot share it. The comment names the counterpart so the pair moves together | `xtask/src/main.rs:64` (bin module list), `_decomposition.md` Note 1 CR-1/CR-2, `_design.md` `## Signatures` |
| `"docs/"` leaves `INERT` rather than being shadowed | A prefix left on the list but unreachable is re-armed by the next reordering of the `else if` chain. `standards/rust/`'s deliberate absence, with its reason in the doc comment, is the shape to copy | `xtask/src/affected.rs:250` (removed), `:245-247` (the precedent), `:222` (the chain) |
| The three comments this change falsifies are rewritten | The tree list at `:210-213`, `is_inert`'s doc comment, and the empty-selection branch's justification ("a docs-only story genuinely has no package to compile"). In this module the recorded reasoning *is* the safeguard — a wrong mapping is invisible from a green run | `xtask/src/affected.rs:210-213`, `:232-247`, `:135-142` |
| Neighbouring prose stays inert | `spec/`, `references/`, `experiments/`, `.bklg/`, `.kb/`, `.redkiln/`, `.github/` and the top-level `*.md` files keep reaching no package. The arm is a rule about one directory | `xtask/src/affected.rs:248-262`, tests at `:660-675` and `:698-702` |
| The widening posture is unchanged | An unrecognised path still returns every member; `WORKSPACE_WIDE` still short-circuits before the arm. Every judgement stays settled in the direction of *more* packages | `xtask/src/affected.rs:12-27`, `:195-197`, `:222-223`, test at `:645-649` |
| Both directions are unit-tested without git or a compiler | `affected_packages` is `pub(crate)` for exactly this; the two shapes to mirror are `a_constitution_atom_selects_xtask` and `the_constitution_arm_does_not_widen_to_all_prose` | `xtask/src/affected.rs:183-190`, `:688-696`, `:699-701` |
| The misnamed existing test is corrected, not deleted | `a_docs_only_change_selects_nothing` asserts on `RUNBOOK.md`; the assertion survives, the name must stop claiming to be about `docs/` | `xtask/src/affected.rs:650-654` |
| The new arm states what selection does not prove | Selection means the package is built and tested — not that any fence compiled, and not that the prose teaches. Recorded in the arm's own comment, in the register every file-reading check here uses | `xtask/src/lint_constitution.rs:9-28`, `standards/rust/81-checks-that-cannot-be-types.md:11`, `project.md` DoD items 7 and 8 |
| Observable on the real story-grain path | `cargo xtask affected --base main` prints `=== affected packages ===` and the selected list, then runs fmt, clippy `-D warnings` and `cargo test --locked` for that set. On a tree-confined change the list contains `xtask` and is no longer empty | `xtask/src/affected.rs:112-177`, `.redkiln/config.yaml:28-40` |
| No new gate wiring | No `Step`, no `REQUIRED` member, no subcommand, no `probe`. The mount is already in place and `steps_named`'s panic-on-unknown-name is not engaged | `xtask/src/main.rs:64`, `:653-658`, `:729`; `standards/rust/80-the-gate.md:11` |

## Data and migrations

**N/A — no persisted state, no schema, no serialised format.** The change is confined to two
compile-time string tables in one bin-crate module: the selection arm's prefix literal
(`xtask/src/affected.rs:214`) and the `INERT` array (`:249-260`). `xtask` is `publish = false`,
so removing `"docs/"` from `INERT` is not a semver event and needs no deprecation arm — the same
reasoning CLAUDE.md gives for the testkit's `factory =` spelling being gone with no deprecated
arm. There is no migration for existing checkouts: the next `cargo xtask affected` run simply
selects one more package than the previous one did, which is the intended and only observable
difference. No feature, no `cfg`, no target and no MSRV interaction
(`_design.md:536`: "MSRV, wasm32, features: unaffected").

## Acceptance criteria

The persona is the one `_storymap.md:12-13` fixes for this whole project: a **contributor
running the gate** and a **reviewer reading its output**. There is no runtime surface, so
every criterion below is a goal one of those two people has, crossing from the diff they made
to the output they read.

| id | criterion | verification |
| -- | --------- | ------------ |
| AC-001 | **GIVEN** a contributor whose whole change is one page under the narrative tree (`docs/…`, `_design.md` D1), **WHEN** they run the story-grain gate — `cargo xtask affected --base main`, the command `.redkiln/config.yaml:40` runs at every advance seam — **THEN** the mapping names `xtask`, the package whose lib target compiles that page's fences, so the gate builds and tests the one package the change could have broken. No new subcommand, flag or prompt is introduced: the contributor reaches this through the invocation already in `print_help` (`xtask/src/main.rs:729`), and the same result is reachable in CI where there is no terminal at all. | Unit test `a_narrative_page_selects_xtask` in `xtask/src/affected.rs`'s `mod tests`, in the shape of `a_constitution_atom_selects_xtask` (`:688-696`): `affected_packages(&changed(&["docs/<page>.md"]), &members())` equals `{"xtask"}` exactly — driven without a git repository or a compiler, which is what `affected_packages` is `pub(crate)` for (`:183-190`). Run by `cargo test -p xtask`. |
| AC-002 | **GIVEN** a contributor editing prose that is *not* in the narrative tree — `references/`, `spec/`, `experiments/`, `.bklg/`, `.kb/`, or a top-level file such as `RUNBOOK.md` — **WHEN** they run the same command, **THEN** no package is selected, because the new arm is a rule about one directory and not about every tree of markdown; and **GIVEN** a path this module has never heard of, the selection still widens to every member rather than narrowing to nothing, so the module's stated posture (`:12-27`, "naming too few reports green over an untested regression") survives the change. The existing test that used to stand for `docs/` is **renamed to what it actually asserts**, not deleted — it is one of the two anchors of that posture. | Unit test `the_narrative_arm_does_not_widen_to_all_prose` (mirroring `:699-701`) asserting a `references/` path selects nothing; `a_docs_only_change_selects_nothing` renamed to `a_top_level_prose_file_selects_nothing` with its `RUNBOOK.md` assertion unchanged (`:650-654`); and `the_relocated_trees_stay_inert` (`:660-675`), `an_unrecognised_path_widens_rather_than_narrows` (`:645-649`), `the_readme_selects_xtask` (`:676-680`) and `the_lockfile_selects_everything` (`:636-640`) all still green and unedited. `cargo test -p xtask`. |
| AC-003 | **GIVEN** a reviewer reading `is_inert` six months from now to decide whether a tree is checked, **WHEN** they look for the narrative tree, **THEN** they find `"docs/"` **absent from `INERT`** — removed, not left shadowed behind an earlier `else if` — with the doc comment stating that it is deliberately absent and why, in the same shape as the `standards/rust/` paragraph at `:245-247` ("Adding it here would silently un-compile the corpus"). A prefix left on the list but unreachable is a prefix the next person to reorder that chain re-arms; that is this AC's named wrong implementation. | Unit test `the_narrative_tree_is_no_longer_inert` asserting `is_inert("docs/<page>.md")` is `false` — a direct assertion on the predicate, so the guarantee does not depend on the order of the `else if` chain at `:214-224` and a later reordering fails a test rather than silently un-compiling the tree. `cargo test -p xtask`. |
| AC-004 | **GIVEN** a reviewer who cannot run the gate and must judge the mapping by reading it — the only instrument available, since a wrong mapping returns *too few* names and that is invisible from a green run (`:183-190`) — **WHEN** they read the module, **THEN** every comment the change falsified says something true: the tree list at `:210-213` no longer claims `docs/` reaches no package; `is_inert`'s doc comment (`:232-247`) covers the new case; the empty-selection branch's justification at `:136-138` no longer rests on "a docs-only story genuinely has no package to compile"; and the new arm states, in the "what this does not verify" register of `xtask/src/lint_constitution.rs:9-28`, that **selection is not compilation** — naming `xtask` means the package is built and tested, not that any fence compiled. Nothing added anywhere claims the surface proves a page teaches (`project.md` DoD items 7 and 8). | Review against the four sites, with `rg -n '"docs/"' xtask/src/affected.rs` returning the new arm's literal and no `INERT` entry as the mechanical half. No `#[test]` can assert a comment is true, which is why RS-81-1 splits the obligation ("prove the blind spot in its own tests, then state it in its own documentation" — `standards/rust/81-checks-that-cannot-be-types.md:11`); the tests are AC-001–AC-003 and this row is the statement. DoD-7's two commands must stay green: `cargo xtask lint-constitution` and `cargo test -p xtask --doc`. |
| AC-005 | **GIVEN** a reviewer reading a CI log for a prose-only pull request, **WHEN** `cargo xtask affected --base main` runs against a change confined to the narrative tree, **THEN** the recorded output shows `=== affected packages ===`, the `{n} file(s) changed against \`main\`` line, and `xtask` on its own two-space-indented line — **and does not show** `no package affected — nothing to compile` — followed by the fmt, clippy and test steps actually running and `affected gate passed` as the single closing summary line. The block's composition is the one that already exists at `:130-146`: nothing is added to it and nothing is hidden by the addition. Per `_design.md` `## Transience policy`, the banner is persistent chrome and the summary is one line; per `## Density budget`, the terminal surface is 80 columns and no list is ever truncated; per `## Hierarchy`, distinction is carried by position and indent only — no colour, weight, tick, badge or "verified" mark, and no progress ticker or per-path line (`## States`, Loading). The evidence is the parsed output naming the package, because a zero exit status is evidence of nothing (RS-81-4). | Gate-integration, run and recorded: `cargo xtask affected --base main` on a tree-confined working tree, transcript captured verbatim into this story's own folder as `_observed-affected-run.md`, showing the banner, the count line, the `xtask` line, the absence of the empty-selection line, and the step banners that follow. Plus `cargo xtask ci --fast` green (`project.md` DoD-6). This is AC-009's third and least-visible invocation path (`_decomposition.md` testing brief, AC-009 item 3). |

Project **AC-009** is the only traced project criterion, and its share here — "`cargo xtask
affected --base main` after a tree-only change selects a run that includes the new step"
(`_decomposition.md:650-657`) — is covered by AC-001 (the mapping) and AC-005 (the observed
run). The `ci` and `ci --fast` halves of AC-009 belong to the slice-mate
(`_storymap.md:121`).

## Interaction quality

This story renders **none** of the five surfaces in `_design.md` `## Surfaces`. It changes
which package names appear in an output block the design deliberately does not claim
(`xtask/src/affected.rs:130-146`). The invariants below therefore apply in the terminal's
medium, and every one of them is carried by an AC row above — this section only says which
row carries which, and how it is checked. Nothing here adds a criterion.

**State family**

| Invariant | Carried by | How it is verified |
| --------- | ---------- | ------------------ |
| In-place, not a context jump | AC-001, AC-005 | The behaviour change is observable inside a command the contributor already runs. No subcommand, no flag, no `Step`, no `REQUIRED` member and no `probe` is added (`xtask/src/main.rs:64`, `:653-658`, `:729`); AC-005's transcript shows the same invocation the gate ran before. |
| Non-occlusion | AC-005 | `xtask` is inserted into the existing sorted `BTreeSet` and printed as one more line in the existing loop (`:144-146`). The `=== the file-reading checks ===` block, the changed-file count, and the fmt/clippy/test step banners all still print, in the same order. The transcript is the check. |
| Preserved reading position | AC-005 | The terminal analogue of preserved focus/scroll: the block's first two lines stay `=== affected packages ===` and `{n} file(s) changed against \`main\``, so a reviewer scanning a long CI log finds the same landmarks in the same place. Asserted against the recorded transcript, not against an exit code (RS-81-4). |
| Reversibility | AC-004, AC-005 | The selector is read-only — it maps strings to package names and writes nothing to the tree, so a run leaves no state to undo. This story adds no write, and AC-004's comment obligations are what stop a future reader assuming otherwise. |
| Keyboard reachability | AC-001 | The entire surface is a typed command, already documented in `print_help` (`xtask/src/main.rs:729`). No interactivity, no TTY dependence and no mouse affordance is introduced, which is why it works unchanged in CI. |

**Composition family** — from `_design.md`, which is binding here even though this story
renders no surface, because the terminal-surface rows constrain the block it prints into.

| Invariant | Carried by | How it is verified |
| --------- | ---------- | ------------------ |
| Presentation exists at all | AC-005 | The package name appears inside the composed block — banner, count line, two-space indent — not as a bare `println!("{affected:?}")` or a debug dump appended to the run. An unstyled equivalent would satisfy "the arm fires" perfectly and is exactly what this row rejects. |
| Composition and placement | AC-005 | The block at `:130-146` *is* the composition; the story adds no seventh surface to `_design.md` `## Surfaces` and no affordance to this one. The two gate surfaces the design does claim (`gate-narrative-compile-step`, `gate-narrative-checker-step`) belong to the slice-mate and to milestone 2. |
| Transience | AC-005 | `_design.md` `## Transience policy`: a step banner is **persistent chrome**, a success summary is **persistent chrome, one line**, and a probe skip line is **never present**. What changes is which *state* renders — `no package affected — nothing to compile` is a state, not chrome, and it must stop appearing for a tree-confined diff. No new chrome, and per `## States` (Loading) **no** spinner, dot ticker or per-path progress line. |
| Density budget, with its numbers | AC-005 | Terminal surface: **80 columns**, **success output 1 line**, **problem list unbounded — never truncated** (`_design.md` `## Density budget`, `### The terminal surface`). The addition is exactly one line, `  xtask`; the longest package name in this workspace is `happenstance-cloudflare` at 23 characters, so two-space-indented package lines stay inside 80 columns with no wrapping and no elision. |
| Hierarchy | AC-005 | `_design.md` `## Hierarchy`: nothing is carried by colour, weight or size — only position, heading level and adjacency. The package list stays a plain two-space-indented list under its banner; no ANSI colour, no bold, no emoji, no glyph. |
| Named anti-patterns | AC-005, AC-004 | #8, **a truncated list** — the package list is never summarised as a count or capped with "… and N more". #9, **a badge, tick, shield or "verified" mark** — none is added to the output (AC-005) and none is claimed in prose (AC-004, DoD item 8). #7's `path:line`-first rule does not apply: this block emits no problem lines, and the story must not invent any. |

## Error conditions

| id | condition | required behaviour |
| -- | --------- | ------------------ |
| EC-001 | `--base` names a ref that does not resolve to a commit | Unchanged and must stay a hard error: "a gate that cannot tell what changed must not report green" (`xtask/src/affected.rs:106-111`). This story touches nothing on that path and must not add a fallback. |
| EC-002 | A tree-confined path is presented while the tree does not exist on disk — for example this arm lands before its slice-mate, or a page is deleted | Selection stays **purely path-based**. `affected_packages` takes `&BTreeSet<String>` of paths and touches no filesystem (`:188-191`); adding an existence probe would make the mapping depend on working-tree state and could return *too few* names, which is the one error this module is not allowed to make. A deleted page still selects `xtask`, which is correct — the harness referencing it must be rebuilt. |
| EC-003 | A path that merely *looks* like the tree — a top-level file literally named `docs`, or a sibling directory such as `docs-notes/` | Neither matches the `"docs/"` prefix, so both fall through to `is_inert`, find no entry, and **widen to the whole workspace** (`:222-223`). That is the safe direction and must not be "fixed" into a narrower match. Prefix comparison is on the raw string, never on `std::path` components. |
| EC-004 | A non-markdown file under the tree — `docs/img/diagram.png`, say | Still selects `xtask`. Filtering the arm by file extension would be a narrowing refinement, and the module's whole posture forbids narrowing (`:12-27`). Cost of the over-selection is one `cargo test -p xtask`; cost of the alternative is a green gate over an untested change. |
| EC-005 | A diff containing both a workspace-wide file and a tree-confined page — `Cargo.lock` plus `docs/<page>.md` | `WORKSPACE_WIDE` still short-circuits and returns every member before the arm is consulted (`:195-197`). The new arm must be added *inside* the existing `None` branch and must not be hoisted above that early return. Covered by `the_lockfile_selects_everything` staying green (`:636-640`). |
| EC-006 | The tree is later moved or renamed without this arm being edited | Out of scope here by design: that failure is AC-001 of the project, owned by `narrative-checker-mounted-with-pinned-path`, which errors by name on a missing tree. This story's mitigation is the comment naming `xtask::narrative::TREE` as the counterpart the literal must move with (Context pack 2). |

## Non-functional

| id | requirement | why / evidence |
| -- | ----------- | -------------- |
| NF-001 | No new dependency, feature, `cfg`, target or MSRV interaction | `_design.md:536` ("MSRV, wasm32, features: unaffected"). The change is two compile-time string tables in one bin-crate module. |
| NF-002 | Gate cost bounded by one package | `xtask` has no dependents, so `close_over_dependents` (`xtask/src/affected.rs:229`, `:273`) adds nothing to the selection. A prose-only diff goes from compiling zero packages to compiling one — `cargo test -p xtask` — which is the cost `_design.md:528-530` already accounts for. |
| NF-003 | Path comparison stays forward-slashed and platform-independent | The changed set is git's output, which uses forward slashes on Windows too (`xtask/src/affected.rs:87-91`). The arm must compare against `"docs/"` as a string, never build a separator from `std::path`. |
| NF-004 | `cargo fmt --all --check` and clippy `-D warnings` clean; no `unwrap`/`expect` added outside `mod tests` | `cargo xtask ci` runs both (CLAUDE.md, "Commands"); the test module's `#![allow(clippy::unwrap_used)]` at `:598` already covers the tests. |
| NF-005 | The module's register is preserved | Every judgement in this file is recorded in prose because a green run cannot show a too-narrow mapping. New comments match the existing voice at `:210-221` and `:245-247` — stating the duplication and its reason rather than hiding it behind an abstraction — and stay inside the 90-column prose wrap the file already keeps. |

## Implementation notes (non-prescriptive)

- The smallest correct shape is one extra disjunct on the condition already at `:214`
  (`path == "README.md" || path.starts_with("standards/rust/") || path.starts_with("docs/")`),
  with the arm's comment extended to cover the third case. Splitting it into a separate `else
  if` branch would work identically and reads worse: the three paths are selected for one
  reason, and one comment should state it once.
- Land the slice-mate first. Nothing in AC-001–AC-004 depends on the tree existing (EC-002),
  but AC-005's transcript is only meaningful once there is a page to change and a harness to
  compile it, and the observation is the point.
- Order the new tests next to the constitution pair they mirror (`:682-702`) rather than at
  the end of `mod tests`. Adjacency is what makes the "one of each direction" obligation
  visible to the next reader; the file's own comment at `:696-698` is written to be read as
  the second half of a pair.
- Extend `is_inert`'s doc comment rather than starting a new paragraph elsewhere: it now has
  **two** deliberately-absent trees, and stating them together prevents the next contributor
  reading the `standards/rust/` paragraph as the sole exception.
- Do **not** add the milestone-2 checker to `affected::run`'s unconditional list (`:120-125`)
  while you are in this file. That is a live open decision with a recommendation attached
  (`_decomposition.md:337-343`) and it belongs to the story that creates the checker.
- Leave `docs/README.md` alone. Its `:25-29` paragraph becomes false when the tree lands, and
  fixing it is the slice-mate's obligation (`_design.md:193-198`); two stories editing the
  same paragraph in the same slice is a conflict for nothing.

## Tests and CI (merge gate)

Tiers are the four `_decomposition.md:538-546` defines. Commands are that brief's own
merge-gate list (`:692-699`), narrowed to the ones this story is answerable for.

| tier | command / path | proves |
| ---- | -------------- | ------ |
| static (unit) | `cargo test -p xtask` → `xtask/src/affected.rs` `mod tests` :: `a_narrative_page_selects_xtask` | AC-001 — a tree-confined path maps to exactly `{"xtask"}`, driven without git or a compiler. |
| static (unit) | `cargo test -p xtask` → `the_narrative_arm_does_not_widen_to_all_prose`, `a_top_level_prose_file_selects_nothing` (renamed from `a_docs_only_change_selects_nothing`, `:650-654`) | AC-002 — the arm is a rule about one directory; the renamed test's `RUNBOOK.md` assertion survives verbatim. |
| static (unit, regression) | `cargo test -p xtask` → `the_relocated_trees_stay_inert` (`:660-675`), `an_unrecognised_path_widens_rather_than_narrows` (`:645-649`), `the_readme_selects_xtask` (`:676-680`), `a_constitution_atom_selects_xtask` (`:688-696`), `the_lockfile_selects_everything` (`:636-640`) | AC-002, EC-005 — the widening posture and `WORKSPACE_WIDE`'s precedence are unchanged; the neighbouring arms still fire. Green and unedited. |
| static (unit) | `cargo test -p xtask` → `the_narrative_tree_is_no_longer_inert` | AC-003 — `is_inert` answers `false` for the tree directly, so the guarantee does not rest on the order of the `else if` chain. |
| static (review, named command) | `rg -n '"docs/"' xtask/src/affected.rs` | AC-003, AC-004 — one hit, in the arm; no `INERT` entry left shadowed. |
| static (existing gate steps) | `cargo xtask lint-constitution`; `cargo test -p xtask --doc` | AC-004 / `project.md` DoD-7 — the existing checker did not regress and every example in the constitution still compiles. Note 8's requirement (`_decomposition.md:695`). |
| gate-integration (recorded) | `cargo xtask affected --base main`, transcript captured to `.bklg/docs-that-teach/checked-documentation-surface/narrative-tree-story-grain-selection/_observed-affected-run.md` | AC-005 and project AC-009's third invocation path — the output *names* `xtask` and omits the empty-selection line. Per RS-81-4 the evidence is the parsed output, not the exit status (`standards/rust/81-checks-that-cannot-be-types.md:264`). |
| gate-integration | `cargo xtask ci --fast` | The merge bar for a non-terminal project (`project.md` DoD-6, `.redkiln/config.yaml`). Also proves this change added no `Step` that `steps_named` would panic on. |
| gate-integration (project close, not story close) | `cargo xtask ci` | The full gate, run before the project is called done (DoD-6). Not this story's bar. |

Not owed here, and named so nobody adds it: no conformance rule (nothing touches a port, a
value type or `crates/happenstance-testkit/`), no doctest of its own (the change is in the
**bin** target, whose comments rustdoc never compiles), and no new integration harness — the
testing brief is explicit that CR-5's obligation is "proven by a unit test in `affected.rs` …
not by a new integration harness" (`_decomposition.md:556-559`).

## Risks and coupling (PR-scoped)

| Risk | Likelihood / impact | Mitigation in this PR |
| ---- | ------------------- | --------------------- |
| The `"docs/"` literal and `xtask::narrative::TREE` drift apart | Medium / High — a moved tree silently un-selects, which is the exact failure this story fixes | The arm's comment names its counterpart explicitly (Context pack 2), and AC-003's test pins `is_inert`'s answer. The structural fix — an error by name on a missing tree — is the project's AC-001 and belongs to `narrative-checker-mounted-with-pinned-path`. Stated, not hidden. |
| The `else if` chain is reordered later and `is_inert` reclaims the tree | Low / High | `docs/` is *removed* from `INERT` rather than shadowed (AC-003), and `the_narrative_tree_is_no_longer_inert` asserts the predicate directly, so a reordering fails a test instead of un-compiling the tree. |
| Someone re-adds `"docs/"` to `INERT` while adding a genuinely inert sibling | Low / High | The doc comment now records **two** deliberately-absent trees with the reason attached, in the shape that has already survived one such edit for `standards/rust/` (`:245-247`). |
| Landing before the slice-mate | Medium / Low | Harmless mechanically (EC-002: selection is path-based), but AC-005's observation is unobtainable, so the story cannot close. The merge order is fixed by `_storymap.md:147-149` and by `blocked_by: HS-S0136`. |
| Scope creep into the slice-mate's or milestone 2's files | Medium / Medium | The PR boundary is two paths. `docs/README.md`, `xtask/src/narrative.rs`, `xtask/src/lib.rs`, `REQUIRED` and `affected::run`'s unconditional list are all named as out (see "Explicitly not in this PR"). |
| Over-selection annoys someone into narrowing the arm | Low / High | EC-003 and EC-004 state the two narrowing refinements that will look tempting (extension filter, tighter prefix match) and forbid both, with the module's own reasoning cited (`:12-27`). |
| The recorded transcript rots against a future output change | Low / Low | It is dated evidence of one observation, not a golden file, and no test asserts against it. AC-005's mechanical successor is `cargo xtask ci --fast` continuing to pass. |

## Dependencies

**Blocks on** — `pinned-narrative-tree-and-compiling-step` (HS-S0136). It creates the tree at
`docs/`, its first registered page, `xtask/src/narrative.rs`, the `mod narrative;` line in
`xtask/src/lib.rs` and the `REQUIRED` compiling step. Without it there is no page to change,
no harness to compile it, and AC-005 has nothing to observe. Matches `story.md`'s
`blocked_by`.

**Unlocks** — no story declares a formal `depends_on` this one (`_storymap.md:45-56`), and
that understates it. Materially it unlocks the *gate* of every story that follows: until this
lands, each of the eight downstream stories runs its own advance-seam check
(`.redkiln/config.yaml:40`) blind to prose-only changes, so a story whose whole deliverable
is under the tree compiles nothing and reports `affected gate passed`. That is why the merge
order puts it second in the first milestone rather than last in the project
(`_storymap.md:147-149`).

**Same-slice, no edge** — it shares milestone `compiled-narrative-tree` with HS-S0136 and is
implemented in the same context. The two touch disjoint files.

## Anchors (progressive disclosure)

Everything load-bearing that is not already distilled above. Open by need, not in order.

| anchor | why it is load-bearing | when to open | serves |
| ------ | ---------------------- | ------------ | ------ |
| `xtask/src/affected.rs` | The mount point and the whole delta. `:12-27` is the posture every judgement here is settled against, `:209-225` the arm, `:232-270` `is_inert` and its two precedents, `:130-146` the output block, `:596-702` the test module and the pair to mirror. | First, before writing any code. Read `:12-27` and `:180-270` in full; the file's comments are the safeguard, not commentary. | AC-001, AC-002, AC-003, AC-004, AC-005 |
| `.bklg/docs-that-teach/checked-documentation-surface/_design.md` | Binding. **D1** (`:165-198`) settles the tree at `docs/` and names this extension as a consequence that must land in the same change (`:193-198`); `## Transience policy` (`:354-374`), `## Density budget` `### The terminal surface` (`:420-428`), `## Hierarchy` (`:453-460`), `## States` (`:462-472`) and `## Anti-patterns` (`:571-597`) are the composition rules AC-005 is held to. | Before AC-005 — both before running the observation and before writing anything into the output block. | AC-001, AC-005 |
| `.bklg/docs-that-teach/checked-documentation-surface/_decomposition.md` | Architecture Note 1 **CR-5** (`:129-141`) is the landmine, in full, including the two test shapes it requires; Note 9 (`:328-343`) records the decisions left open and what stays out; the testing brief (`:514-546`, AC-001 at `:548-559`, AC-009 at `:643-657`, merge-gate list at `:692-699`) fixes the tiers and commands. | Note 1 CR-5 before AC-001; the testing brief before writing the tests; Note 9 before touching anything else in the file. | AC-001, AC-002, AC-003, AC-005 |
| `.bklg/docs-that-teach/checked-documentation-surface/project.md` | AC-009 verbatim (`:227-229`) — the only traced criterion — plus DoD-6 (the `ci --fast` bar) and DoD items 7 and 8 (`:256-260`), which AC-004 discharges and which forbid claiming the surface proves teaching. | Before AC-004 and before writing the merge-DoD claim. | AC-004, AC-005 |
| `.bklg/docs-that-teach/checked-documentation-surface/_storymap.md` | The persona framing every AC is written from (`:12-24`), the slice row and its one-line slice (`:48`), and the merge-order paragraph that says *why* this lands second in the same context (`:147-149`). | Before writing or reviewing acceptance criteria; again if the merge order is questioned. | AC-001, AC-005 |
| `.bklg/docs-that-teach/initiative.md` | DoD scenario 1 (`:412-415`) — the clean-checkout scenario this story advances by one of three invocation paths — and the referenced personas and journeys (`:275-311`), which is also the record that no `authority_tier: product` atom exists to cite. | When justifying why this story exists at all, or when tempted to write a persona claim not carried by that section. | AC-005 |
| `standards/rust/81-checks-that-cannot-be-types.md` | RS-81-1 (`:11`) splits AC-004's obligation into a test plus a statement; RS-81-4 (`:264`) is why AC-005's evidence is the parsed output and not a zero exit status; RS-81-3 (`:209`) is why the arm scopes to one directory rather than generalising over prose. | Before AC-004 and before capturing AC-005's transcript. Pull these three rules only — not the corpus. | AC-004, AC-005 |
| `standards/rust/80-the-gate.md` | RS-80-1 (`:11`) is the rule that would demand a `Step` in `REQUIRED` reached by name. It is the slice-mate's obligation, not this story's, and knowing that is what stops a redundant second gate step for a selector that is not a check. | Only if you are about to add a `Step`, a subcommand or a `REQUIRED` member — the answer is that this story adds none. | AC-005 |
| `xtask/src/lint_constitution.rs` | `:9-28` is the exact register AC-004's "what selection does not prove" comment must be written in — a file-reading check stating its own limits first, because an undocumented limit is read as a guarantee. `:169-198` is the accumulate-all output shape the milestone-2 checker inherits, which is *not* this story's. | Immediately before writing the arm's comment. Read `:9-28` only. | AC-004 |
| `xtask/src/main.rs` | The mount that already exists: `mod affected;` (`:64`), the `Some("affected")` dispatch (`:653-658`) and the `print_help` entry (`:725-732`). Confirms the story changes what is mounted, not where — and that `print_help` needs no edit. | Before claiming the mount, and before adding any wiring. | AC-001, AC-005 |
| `.redkiln/config.yaml` | `verify.affected_gate` at `:40`, with the comment block at `:28-39` explaining why the story grain re-derives the file list from git rather than taking `{{changed}}`. This is what makes the story's value structural rather than cosmetic: it is the command every downstream story's advance seam runs. | When judging blast radius, and when writing the transcript for AC-005 — run the command in the form the config runs it. | AC-005 |
| `.bklg/docs-that-teach/checked-documentation-surface/_grounding.md` | `:191-199` records that **no** Accepted decision atom under `.kb/decisions/` governs gate structure, documentation trees or fence compiling. It is the authority check: the binding constraints here are conventions, so no ADR is written, amended or cited. | If you are about to cite or write an ADR for anything in this story. | AC-004 |
| `docs/README.md` | `:25-29` states the pin-by-path rule and lists the trees the gate reads. It becomes false when the tree lands and is the **slice-mate's** edit (`_design.md:193-198`). Read so this story does not contradict it and does not edit it. | Before touching anything outside `xtask/src/affected.rs`. | AC-004 |

## Clarifications resolved during spec

1. **The AC set is exactly the five ids the first pass declared**, and the interaction-quality
   invariants are folded into them rather than added as new rows — AC-005 carries the whole
   composition family and the terminal state family, AC-001 carries keyboard reachability and
   the in-place requirement. `redkiln verify` extracts ACs from table cells, so an invariant
   stated only as a bullet would never be gated; putting them inside AC-005's criterion keeps
   the count honest and keeps them checkable. Nothing was added and nothing dropped.
2. **The misnamed test is renamed, not re-pointed.** `_decomposition.md:335-336` and
   `_design.md:194-196` both anticipated `a_docs_only_change_selects_nothing` needing "a path
   that is still genuinely inert". Read against the code (`xtask/src/affected.rs:650-654`) it
   already asserts on `RUNBOOK.md`, so the assertion needs nothing; only the name is false.
   AC-002 renames it to `a_top_level_prose_file_selects_nothing` and forbids deleting it.
3. **A third test was added beyond CR-5's two.** CR-5 requires one test per direction. AC-003
   adds `the_narrative_tree_is_no_longer_inert`, asserting on `is_inert` directly, because the
   two directional tests both pass while `"docs/"` sits shadowed on `INERT` — they cannot
   distinguish "removed" from "unreachable", which is the failure mode Context pack 3 names.
4. **The prefix stays a literal in `affected.rs`.** The two `xtask` targets cannot share a
   private `const` (`xtask/src/main.rs:64` is the bin crate; the harness is a lib-crate
   module), and introducing a `pub` seam to share seven characters costs more than the
   duplication. The duplication is stated in a comment, in the register `:215-220` already
   uses for `standards/rust/`, rather than abstracted away.
5. **`docs/README.md:25-29` is deliberately not touched here.** Both this story and the
   slice-mate falsify it; `_design.md:193-198` assigns it to the change that creates the tree.
   Two stories editing one paragraph in one slice is a conflict for no gain.
6. **No new gate step, and that is a decision rather than an omission.** RS-80-1 would demand
   a `Step` in `REQUIRED` reached by name for a *check*; a package selector is not a check, and
   the mount (`xtask/src/main.rs:64`, `:653-658`, `:729`) is already complete. Adding one would
   produce a second, redundant gate entry.
7. **Whether the milestone-2 checker joins `affected::run`'s unconditional list is left open.**
   `_decomposition.md:337-343` carries the recommendation and the ambiguous precedent; the
   decision belongs to `narrative-checker-mounted-with-pinned-path`, which creates the checker.
   Touching `:120-125` here would settle another story's question in passing.
8. **AC-005's evidence is a transcript, not a golden file.** RS-81-4 requires the output be
   parsed and the names asserted; it does not make the transcript a fixture. Nothing in the
   test suite compares against `_observed-affected-run.md`, so it cannot rot into a failing
   test — it is dated evidence that the observation was made, per DoD-2's standard that a gate
   which has only ever been green is decorative.
