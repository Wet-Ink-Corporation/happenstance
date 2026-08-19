---
item: HS-S0183
stage: spec
created: 2026-08-17T13:16:32.000Z
updated: 2026-08-17T13:16:32.000Z
template_sig: 87bbf1d0
rendered_sig: 23f9ed50
---

# Spec — Merge forward and record the baseline before authoring

## Scope lock

| | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/docs-that-teach/initiative.md` |
| Initiative decomposition | `.bklg/docs-that-teach/_decomposition.md` |
| Project | `.bklg/docs-that-teach/application-author-path/project.md` |
| This spec | `.bklg/docs-that-teach/application-author-path/merge-forward-preflight/spec.md` |
| Key briefs | `.bklg/docs-that-teach/application-author-path/_decomposition.md` (UX brief `:11-447`, testing brief `:448-650`) |
| Signed-off design (binding) | `.bklg/docs-that-teach/application-author-path/_design.md` |
| Grounding | `.bklg/docs-that-teach/application-author-path/_grounding.md` |
| Story map / merge order | `.bklg/docs-that-teach/application-author-path/_storymap.md` |
| Roadmap pointer | `RUNBOOK.md` — this initiative is documentation work and adds no phase; the roadmap is not amended here |

## One-line PR slice

Merge `initiative/from-contract-to-published-library` forward into
`initiative/docs-that-teach` and record the baseline, so every page in this project is
authored against the merged `crates/happenstance/src/lib.rs` rather than this worktree's
75-line copy.

## Executive summary

This PR lands **one merge commit and one baseline record**, and no teaching content at all.

The pointer is AC-014 / DR-13 (`project.md:274-277`, `:220-223`), which the decomposition
gate recorded as an operational rule rather than a preference: `crates/happenstance/src/lib.rs`
has been *replaced*, not merely diverged from, on the sibling branch
(`.bklg/docs-that-teach/_decomposition.md`, "Decisions taken at the gate"). The delta this
spec adds is that the merge's consequences were **measured**, not predicted, and the
measurement changes what the story owes.

Three things were found by running the comparison rather than asserting it, and each one
turns a piece of the signed-off design into a claim that is false after the merge:

1. **`crates/happenstance/Cargo.toml` already has the `tokio` dev-dependency** on the sibling
   (`macros`, `rt`, `rt-multi-thread`). `_design.md`'s `## Items` row declaring it `added`
   becomes a no-op, and the reason the design gave for adding it — that the crate declares no
   dev-dependencies at all, so the existing fence wraps its body in `# async fn example()` and
   never awaits it — is no longer true of the merged file.
2. **`examples/course-subscriptions/src/main.rs` now imports `happenstance`, not
   `happenstance_core`**, and its module doc runs `:1-28`, not `:1-19`. `_design.md`'s
   vocabulary-seam sentence (`## Composition`, `worked-example-handoff` item 3) exists solely
   to name an import that the merge deletes, and AC-010's `:1-19` citation points at the wrong
   span.
3. **Every line-number anchor in the planning corpus drifts.** ES-25 moves to
   `spec/SPECIFICATION.md:3698`, VT-30 to `:1816`, `const REQUIRED` to `xtask/src/main.rs:117`,
   the `"tests"` step to `:155`. Clause *ids* are stable and never renumbered
   (`spec/SPECIFICATION.md:280`); line citations are not, and this project's briefs cite lines.

So the deliverable is not "run `git merge`". It is: land the merge, prove the merged tree is
gate-green before any slice-mate inherits it, and leave behind a **baseline record** that
re-resolves every anchor the four downstream page stories will open — with each invalidated
design assumption dispositioned (fixed here, routed, or carried) rather than discovered one
story at a time.

The merge itself was dry-run in this worktree and is **textually clean**:
`git merge-tree --write-tree initiative/docs-that-teach initiative/from-contract-to-published-library`
returned a tree oid and no conflict section. Our branch has changed nothing outside `.bklg/`,
`.redkiln/telemetry/events/`, and `references/seeds/user-documentation.md` since the merge base
`ce933d8`; the sibling's 223 commits touch none of those three. The hard part of this story is
the record, not the resolution.

## Context pack

**The decision this story exists to enforce.** The initiative runs in parallel with
`initiative/from-contract-to-published-library` and merges forward before the pull request,
rather than blocking on it. That was decided at the decomposition gate, and it survived one
counter-argument: the concern that a 521-line specification divergence invalidates pinned
clause citations does not hold, because *"Clause IDs are stable and are never renumbered"*
(`spec/SPECIFICATION.md:280`). Ids are stable names; **line numbers are not**, and this
project's briefs, grounding and design cite lines throughout. One operational rule fell out and
must not be lost: **this project merges forward before it implements**
(`.bklg/docs-that-teach/_decomposition.md`, "Decisions taken at the gate").

**Why the file matters more than the branch.** `crates/happenstance/src/lib.rs` is 75 lines
here and 237 there. This branch's copy does not contain the `Tags::empty()`-twice defect that is
the initiative's headline evidence — that defect lives only on the sibling, at `:38` and `:55`
of their copy (`_grounding.md:163-170`). Authoring the opening encounter against this branch's
file means writing a page against a crate root that will not exist, and redoing it at merge.

**What this story may and may not decide.** `_design.md` is signed off and binding
(`_design.md:1039`, the approver's row). Its first stated condition *is* this story: "no page in
this project may be authored until the merge forward … is complete and recorded". This story
discharges that condition; it does **not** get to re-open the design because the merge moved the
ground under it. Where a merged fact contradicts a binding design statement, this story
**records the contradiction with a disposition and routes it** — substrate gaps to HS-P0020,
pointer and reach gaps to HS-P0023, comprehension doubts to HS-P0024, incidental bugs to the
`support` initiative per `.redkiln/config.yaml:5` — per project DoD item 9 (`project.md:300-302`).
Re-deciding DT-1/DT-4/DT-5/DT-6 here would be re-litigating a human sign-off in a preflight PR.

**The four measured contradictions, stated as facts the implementer must carry, not as work to
redo.** Each was verified against the sibling tip `3f49ec6` in this worktree:

- **The `tokio` dev-dependency is already there.** `_design.md`'s `## Items` block declares
  `happenstance [dev-dependencies] tokio` as `change: added`, and justifies it at
  `_design.md:299-306` on the grounds that "`crates/happenstance/Cargo.toml` declares **no**
  dev-dependencies at all today". True here (confirmed: the file has no `[dev-dependencies]`
  table). False on the sibling, which declares `tokio = { workspace = true, features =
  ["macros", "rt", "rt-multi-thread"] }` alongside `serde_json`, `futures-core` and a path-only
  `happenstance-testkit`. Disposition: the row becomes `unchanged`, the *reason* it was needed
  (a fence must execute, not merely type-check — AC-004, IQ-7) is untouched and still binding.
- **The worked example changed crates.** It imports `happenstance` at `:33-37` on the sibling —
  the merge already did what ADR-0006 asks (`.kb/decisions/0006-bare-name-to-the-typed-layer.md`).
  `_design.md`'s handoff surface spends one whole composition slot on a vocabulary-seam sentence
  whose premise ("the example imports `happenstance_core` directly … and that is correct for its
  own purpose") the merge deletes. Disposition: record it; the sentence's fate belongs to
  `surface-course-subscriptions` under this recorded baseline, not to this story.
- **The worked example's module doc grew.** `:1-19` becomes `:1-28`, gaining a
  `# What is not in this file, and used to be` section. AC-010 requires that doc be *surfaced*
  verbatim, and `_design.md`'s `## Placement and re-export` plans an `overview.md` extraction to
  make that true by construction. The span moved; the mechanism did not.
- **The example is no longer test-free.** `project.md:75-77` records it as carrying "no test
  target"; the sibling adds `examples/course-subscriptions/tests/runs.rs` and `tests/ui.rs` with
  a `trybuild` dev-dependency, still under `publish = false`. That bears directly on AC-005's
  premise (the testing brief's own note that `cargo run -p course-subscriptions` proves nothing
  to the gate, `_decomposition.md:568-573`), and the merged position must be recorded before
  `boundary-falsification-drill` builds a check on the old one.

**The persona-journey slice this serves.** Backbone activity A1 — *"Give me a baseline I am not
about to redo"* (`_storymap.md:41`). It is the reader's activity only indirectly: the reader
never sees this PR. What they see is that the encounter they follow was written against the
crate they installed. Every other story in this project is downstream of that, which is why both
`preflight-and-anchor` stories are `foundation` and why the slice is one slice rather than two
(`_storymap.md:66-69`).

**The gate bar, and why it applies to a merge.** This project is `terminal: false`
(`project.md:17`), so the project-scoped integration bar is `cargo xtask ci --fast`
(`.redkiln/config.yaml:55`) and the story-grain bar is `cargo xtask affected --base main`
(`:41`). A merge that brings 223 commits of source across is exactly the case where "the gate was
green before I started" stops being true silently: the next story's implementer would meet a red
baseline and attribute it to their own page. Proving green **on the merge commit** is what makes
the baseline a baseline.

**What must not happen in this PR.** No page. No fence. No edit to `crates/happenstance/src/lib.rs`
beyond what the merge itself brings, and no edit to `_design.md`. The story's entire authored
diff is its own backlog folder; everything else arrives through the merge commit's second parent.

## Integration contract

- **Archetype**: `foundation` — real in-tree substrate (the merged crate root and the recorded
  baseline) consumed by the capability slices in this same project. Not a double, not a fixme:
  `boundary-refusal-encounter` is authored against the merged `lib.rs` and
  `surface-course-subscriptions` surfaces the merged example (`_storymap.md:91-93`).
- **Slice / milestone**: `preflight-and-anchor`. Slice-mate: `tension-resolutions`
  (HS-S0184), which blocks on this story. Both are gates on the same thing — authoring — and are
  delivered together (`_storymap.md:66-69`).
- **Mount point**: `crates/happenstance/src/lib.rs` — the merged crate root. It is the render
  path for the `crate-root-encounter` surface (`_design.md:46-49`, route
  `/happenstance/index.html`, rendered by `cargo doc -p happenstance --no-deps`), and it is the
  file AC-014 names by path. The merge commit on `initiative/docs-that-teach` is the mechanism;
  this file is where the mount is observable — a reader of the worktree sees 237 lines and
  `Tags::empty()` at `:38`/`:55`, or the mount did not happen.
- **Wires into**:
  - `crates/happenstance/Cargo.toml` — the merged `[dev-dependencies]` table that makes an
    executing fence possible (`_design.md:299-306`).
  - `spec/SPECIFICATION.md` — the merged clause text every later page cites by id, never by
    prose (`:280` for id stability; ES-25 `:3698`, VT-30 `:1816`, CF-7 in the `:7266` region,
    all post-merge).
  - `xtask/src/main.rs` — the merged `REQUIRED` array (`:117`) and its `"tests"` step (`:155`),
    which is the step tier 3 rides (`_decomposition.md:545-551`).
  - `examples/course-subscriptions/src/main.rs` and its `Cargo.toml` — the merged worked example
    AC-010 surfaces.
  - `.redkiln/config.yaml` — `affected_gate` (`:41`) and `integration_scoped` (`:55`), the two
    commands redkiln runs at this story's and this project's grains whether or not anyone types
    them.
- **Public items**: **none.** `_design.md`'s `## Items` block declares that this project adds,
  changes and removes no public Rust API item (`_design.md:268`), and this story does not even
  change the rendered doc surface — it changes which copy of it is in the tree. The one `## Items`
  row this story *corrects the status of* is `happenstance [dev-dependencies] tokio`
  (`_design.md:284-288`), which the merge satisfies rather than this project adding.
- **Conformance rule(s)**: **none, and it is not adapter-observable.** No port, no store, no
  fixture and no `suite.rs` rule changes here. The merge brings the sibling's conformance work
  with it, but this story neither adds nor amends a rule, so naming one would be decorative by
  CLAUDE.md's own test. What observes this story is the gate: `cargo xtask affected --base main`
  at the checkpoint and `cargo xtask ci --fast` on the merged tree.
- **Clause(s)**: **none discharged, none amended.** The initiative is additive and discharges no
  clause (`project.md:155-157`). This story *re-resolves the citations* to ES-25
  (`spec/SPECIFICATION.md:3698`), VT-30 (`:1816`) and CF-7 (`:7266` region) against the merged
  text, and touches no clause body. Changing a `[FROZEN]` clause would take a new ADR; nothing
  here comes near one.
- **Advances DoD scenario**: initiative DoD **3** — *"the opening encounter runs and demonstrates
  a boundary"* (`initiative.md:418-422`) — which this story does not reach but makes reachable,
  by fixing the tree the encounter is authored against; and DoD **4**, its check, for the same
  reason. It also directly re-establishes DoD **1**'s precondition (`initiative.md:412-415`) —
  the full gate green on the assembled tree — because the merge is the largest single change to
  that tree this initiative will make.

## PR boundary

```
xtask/src/spec_trace.rs
standards/rust/51-features-and-no-std.md
standards/rust/52-wasm32-and-target-cfg.md
standards/rust/70-rustdoc-obligations.md
standards/rust/80-the-gate.md
crates/happenstance/tests/doc_budget.rs
.bklg/docs-that-teach/application-author-path/merge-forward-preflight/**
```

> **Amended 2026-08-18. The merge stopped being clean, and this fence is the
> paragraph below being honoured — one commit late.** The paragraph requires each
> conflict-resolution file to be named here *before* it is resolved. The merge was
> performed in `/redkiln:implement` preflight under human supervision rather than
> inside the workflow, and the six files were named **after** resolution, in the
> same session and before any of this project's authored work began. That ordering
> is worse than the paragraph asks for and is recorded rather than smoothed over:
> the point of naming first is that nobody gets to decide a file was in scope by
> having already edited it.
>
> What each is, so the scrutiny the paragraph demands can actually be applied:
>
> - **`xtask/src/spec_trace.rs`** — both branches added tests to the same
>   `mod tests`. Resolved as a **union**, 230 lines of ours plus 54 of theirs;
>   taking either side would have deleted passing tests.
> - **`standards/rust/{51,52,70,80}-*.md`** — Evidence line-number citations only.
>   Both branches inserted a `REQUIRED` step into `xtask/src/main.rs` and each
>   repointed these citations at its own numbers, so **neither side was correct**
>   against the merged file. Took theirs as the base and recomputed all ten stale
>   citations; `cargo xtask lint-constitution` reports 27 atoms, all consistent.
>   No claim, rule or example changed.
> - **`crates/happenstance/tests/doc_budget.rs`** — the only red test after the
>   merge. `read()` now normalises CRLF to LF. The test matches source text against
>   literal `\n`, and this repository runs `core.autocrlf = true` with no
>   `.gitattributes`, so a freshly *checked out* file is CRLF and the pattern never
>   matches. The sibling worktree passes only because its agents *wrote* those files
>   as LF — a fresh clone on Windows fails identically, so this is a latent defect on
>   `initiative/from-contract-to-published-library` that the merge exposed rather than
>   caused.
>
> The merge is `a5c0f30`. `cargo xtask ci --fast` and
> `cargo test --workspace --all-features --no-fail-fast` are both green on it. This
> story still owes everything else the list below names — the baseline record, the
> re-resolved anchor table, the four dispositioned design contradictions and the
> recorded gate result.

**Read the fence together with this paragraph, because a merge commit does not fit a glob.**
`redkiln verify --grain story` fails on any file changed outside the first fenced block above,
and the merge commit's second parent brings roughly 231 files across. Those files are not
*authored* changes: they arrive by parentage, not by edit, and `git diff HEAD^1 HEAD` on the
merge is empty of anything this story wrote. The fence is therefore the narrowest honestly-true
statement of what this story **authors**, and it is deliberately not widened to `**`. If the
merge stops being clean between this spec and its implementation, each conflict-resolution file
must be **named individually in this fence, in the spec, before it is resolved** — a resolution
is an authored edit and gets an authored edit's scrutiny.

**In this PR**

- One merge commit on `initiative/docs-that-teach` with
  `initiative/from-contract-to-published-library` as its second parent, recorded with its two
  parent shas.
- The baseline record — a companion under this story's own folder — carrying the re-resolved
  anchor table, the four dispositioned design contradictions, and the recorded gate result.
- A green `cargo xtask ci --fast` **on the merge commit**, recorded as observed rather than
  assumed.

**Explicitly not in this PR**

- Any page, fence, output block, mapping table or answered-need line. Authoring is gated on this
  story *and* on `tension-resolutions` (`_design.md:1033-1036`).
- Any edit to `_design.md`. Its sign-off row is a human's; a preflight PR records contradictions
  and routes them, it does not amend a signed-off design (`project.md:300-302`).
- Any edit to `crates/happenstance/src/lib.rs`, `examples/course-subscriptions/`,
  `spec/SPECIFICATION.md` or `xtask/src/` beyond what the merge brings. The crate-root rewrite is
  `boundary-refusal-encounter`'s; the `overview.md` extraction is `surface-course-subscriptions`'s.
- Repairing the stale line citations *inside* `project.md`, `_grounding.md`, `_storymap.md` and
  `_decomposition.md`. Those are Redkiln item bodies and companions belonging to a stage already
  passed; the baseline record supersedes them by pointing at the merged lines, and the closeout's
  reference reconciliation is where the pointers themselves are repaired
  (`_design.md:736-741`, HS-P0025).
- Merging anything to `main`, or waiting on the sibling's own merge to `main`. The cross-branch
  relationship is a precondition on implementation, not a DAG edge (`project.md:326-329`).

**Merge DoD one-liner** — the merge commit exists with both parents recorded,
`cargo xtask ci --fast` is green **on it**, and the baseline record re-resolves every anchor the
four page stories will open, with each contradicted design assumption carrying a disposition.

The implementer MAY also touch the composition-root / wiring files named in the Integration
contract to mount this slice; that is not scope drift. Here that permission is nearly vacuous by
construction — the mount *is* the merge — and it exists to cover the one case where it is not: a
conflict resolution inside `crates/`, `spec/` or `xtask/`, which must be named in the fence first.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **The merge lands, and it is a real merge** | `initiative/from-contract-to-published-library` becomes an ancestor of `initiative/docs-that-teach`. Merge base today is `ce933d8`; the branches stand at 6 commits (ours) against 223 (theirs). Verified by `git merge-base --is-ancestor`, not by a diffstat. | `.bklg/docs-that-teach/_decomposition.md` ("Decisions taken at the gate"); `project.md:274-277` |
| **The merge is expected clean, and a conflict is a stop-and-report** | `git merge-tree --write-tree` over the two tips returns a tree oid with no conflict section. Our side has touched only `.bklg/**`, `.redkiln/telemetry/events/ryan-britton@docs-that-teach.jsonl`, `.redkiln/telemetry/events/ryan-britton@from-contract-to-published-library@runs.jsonl` and `references/seeds/user-documentation.md` since the base; the sibling touches none of them. A conflict therefore means the world moved, and the response is to name the files in the PR boundary fence before resolving. | `.redkiln/telemetry/events/ryan-britton@docs-that-teach.jsonl`; `references/seeds/user-documentation.md` |
| **The mount is observable in the tree** | After the merge, `crates/happenstance/src/lib.rs` is 237 lines (75 before) and carries `Tags::empty()` at `:38` and `:55` — the initiative's headline evidence, which is absent from this branch entirely. Checked by reading the file, not by trusting the merge's exit code. | `crates/happenstance/src/lib.rs`; `_grounding.md:163-170` |
| **The merged tree is gate-green before any slice-mate inherits it** | `cargo xtask ci --fast` (`.redkiln/config.yaml:55`, the non-terminal project bar) run **on the merge commit**, plus `cargo xtask affected --base main` at the story grain (`:41`). A failure here is the merge's, not a later page's, and must be resolved or reported before the slice proceeds. | `.redkiln/config.yaml:41,55`; `project.md:290-292`; `xtask/src/main.rs` |
| **The baseline record exists at a stable, citable path** | A companion in this story's own folder — the only place this story is allowed to author. It is what the four page stories open instead of re-deriving line numbers, and what the reviewer walks at DoD item 8. | `project.md:298-299`; `_storymap.md:121` |
| **Anchors are re-resolved by id first, by line second** | Clause ids are stable and never renumbered, so ES-25, VT-30, ES-26, ES-27 and CF-7 stay valid names; their *lines* move. Measured pre/post: ES-25 `:3695`→`:3698` (the corpus cites `:3693`), VT-30 `:1813`→`:1816`, `const REQUIRED` `xtask/src/main.rs:105`→`:117`, `"tests"` step `:143`→`:155`, the worked example's module doc `:1-19`→`:1-28`. The record states the id, then the merged line. | `spec/SPECIFICATION.md:280`; `spec/SPECIFICATION.md`; `xtask/src/main.rs`; `examples/course-subscriptions/src/main.rs` |
| **Every design assumption the merge contradicts carries a disposition** | Four, all measured: (1) `tokio` dev-dependency already present with `macros`/`rt`/`rt-multi-thread` — `_design.md`'s `added` row becomes `unchanged`, its stated reason survives; (2) the worked example imports `happenstance`, deleting the vocabulary-seam sentence's premise; (3) its module doc is `:1-28`, not `:1-19`; (4) it now carries `tests/runs.rs` and `tests/ui.rs` under `trybuild`, against `project.md`'s "no test target". Each is recorded as *carried*, *routed* (HS-P0020 / HS-P0023 / HS-P0024 / `support`) or *fixed here* — and none is re-decided. | `_design.md:284-306`, `:497-502`, `:728-741`; `project.md:73-77`, `:300-302`; `.redkiln/config.yaml:5` |
| **No teaching content ships** | The authored diff is this story's backlog folder and nothing else. A reviewer confirms by `git diff HEAD^1 HEAD --stat` on the merge commit being empty of authored files, and by the PR boundary fence holding. | `_storymap.md:55`; `project.md:279-299` |
| **The signed-off design is consumed, never amended** | `_design.md` is binding (`:10-12`) and its approval row makes this merge its own first condition (`:1039`). This story reads it and records against it; the file is not edited. | `.bklg/docs-that-teach/application-author-path/_design.md` |
| **This story is distinct from the closeout's merge-forward story** | `.bklg/docs-that-teach/durable-audience-closeout/merge-forward-baseline/` is HS-P0025's, and it re-observes the whole initiative against whatever tree exists at closeout. This one is a *preflight* on one project's authoring, tier 0, one-time. Confusing the two would either delay authoring to closeout or skip the closeout re-observation. | `.bklg/docs-that-teach/durable-audience-closeout/merge-forward-baseline/story.md`; `_decomposition.md:535` |

**Interfaces, explicitly.** This story defines, changes and consumes **no Rust interface**. Its
"interface" is two artifacts and one guarantee: the merge commit (git), the baseline record
(markdown, in this story's folder), and the guarantee that the tree the next four stories open is
the merged one and is gate-green. Anything that looks like an API decision surfacing during the
merge is, by construction, the sibling branch's decision already taken — record it, do not
re-take it.

## Data and migrations

**N/A — no schema, no store, no data.** Nothing in this story reads or writes an event store, a
projection store or a checkpoint. `MemoryEventStore` is not constructed. No `Cargo.toml` feature
is added or removed by an authored edit (the merged `[dev-dependencies]` change arrives by
parentage, and dev-dependencies are not resolved by downstream consumers —
`_design.md:761`).

The one migration-shaped thing here is **citation migration**, and it is deliberately not a
rewrite: clause ids and rule names are stable across the merge
(`spec/SPECIFICATION.md:280`), so nothing is renamed; only line offsets move, and they are
re-resolved *in the baseline record* rather than by editing the stage artifacts that carry the
stale numbers. Repairing those pointers in place is the closeout's reference reconciliation
(`_design.md:736-741`), and doing it here would edit item bodies belonging to stages already
signed off.

## Acceptance criteria

The baseline record this story authors is
`.bklg/docs-that-teach/application-author-path/merge-forward-preflight/_baseline.md` — inside
the PR boundary fence, and the only file this story writes besides `spec.md` and `_ledger.md`.
Section names below (`§ Merge`, `§ Gate`, `§ Anchors`, `§ Dispositions`, `§ Composition
baseline`) are that file's headings, so the ledger's `verifying_test` column points at
something with a path and a heading rather than at a promise.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | GIVEN Persona 1 (the application author) will meet this library as its rendered crate root, WHEN the merge lands on `initiative/docs-that-teach`, THEN `initiative/from-contract-to-published-library` is an ancestor of the branch tip, the merge commit records both parent shas in `_baseline.md § Merge`, `crates/happenstance/src/lib.rs` is the sibling's 237-line file carrying `Tags::empty()` at `:38` and `:55`, `cargo doc -p happenstance --no-deps` renders `/happenstance/index.html` with `#main-content details.top-doc > div.docblock` present — so the page the reader will be taught from is composed and addressable at baseline, not merely merged on paper — and nothing this branch authored under `.bklg/**`, `.redkiln/telemetry/events/**` or `references/seeds/user-documentation.md` is occluded by the merge. | `git merge-base --is-ancestor initiative/from-contract-to-published-library HEAD` (exit 0) and `git rev-list --parents -n 1 HEAD` (three shas), both transcribed into `_baseline.md § Merge`; a read of `crates/happenstance/src/lib.rs` confirming line count and both `Tags::empty()` sites; `cargo doc -p happenstance --no-deps` then a grep of `target/doc/happenstance/index.html` for the selector; `git diff <pre-merge-sha> HEAD -- .bklg .redkiln/telemetry references/seeds/user-documentation.md` showing only this story's own additions. |
| AC-002 | GIVEN the next implementer in the `preflight-and-anchor` slice must be able to trust that a red gate is their own page's doing and not an inherited failure, WHEN this story's checkpoint is cut, THEN `cargo xtask ci --fast` has been run **on the merge commit itself** and recorded green against that commit's sha in `_baseline.md § Gate`, `cargo xtask affected --base main` is green at the story grain, and the baseline is reversible in one move — the merge is a single commit whose first parent is this branch's pre-merge tip, so `git revert -m 1 <sha>` restores it without unpicking 223 commits. | `cargo xtask ci --fast` (`.redkiln/config.yaml:55`) run at `HEAD` = the merge commit, its sha and result recorded in `_baseline.md § Gate`; `cargo xtask affected --base main` (`.redkiln/config.yaml:41`) at the checkpoint; `git rev-list --parents -n 1 HEAD` confirming parent order, recorded alongside. |
| AC-003 | GIVEN an author of any of the four downstream page stories needs a clause or a source line **cold**, without re-running the archaeology this story already ran, WHEN they open `_baseline.md § Anchors`, THEN every anchor those stories will open resolves from that table alone — ES-25, VT-30, ES-26, ES-27 and CF-7 stated **by clause id first and merged line second** (ids are stable and never renumbered, `spec/SPECIFICATION.md:280`; lines are not), `const REQUIRED` and the `"tests"` step in `xtask/src/main.rs`, the worked example's module-doc span, and `crates/happenstance/Cargo.toml`'s `[dev-dependencies]` table — each row carrying the command that re-derives it, each line verified against the merged tree rather than copied from the pre-merge corpus, and `cargo xtask spec-trace` green over that tree. | `cargo xtask spec-trace` (REQUIRED step) on the merge commit; every row of `_baseline.md § Anchors` re-derived by the command it names (`rg -n "^ES-25" spec/SPECIFICATION.md` and siblings) with the output line number matching the recorded one; a cold read of the table by the `tension-resolutions` implementer with no fallback to `git log`. |
| AC-004 | GIVEN the four contradictions the merge creates would otherwise be rediscovered one story at a time — each time as a surprise mid-authoring — WHEN a page author reads `_baseline.md § Dispositions`, THEN all four appear with an explicit disposition of `carried`, `routed to <item>` or `fixed here`: the `tokio` dev-dependency already present with `macros`/`rt`/`rt-multi-thread`, the worked example importing `happenstance` rather than `happenstance_core`, its module doc at `:1-28` rather than `:1-19`, and its new `tests/runs.rs` and `tests/ui.rs` under `trybuild` against `project.md:75-77`'s "no test target" — each preserving the *reason* `_design.md` gave rather than re-deciding the design (the `tokio` row's justification that a fence must execute and not merely type-check survives even though the row's status does not), each route named per project DoD item 9 (`project.md:300-302`), and `_design.md` byte-identical to its signed-off state. | Reviewer walk of `_baseline.md § Dispositions` against `_design.md:284-306`, `:497-502`, `project.md:73-77` and `.redkiln/config.yaml:5`, recorded as DoD item 8's observation; `git diff <pre-merge-sha> HEAD -- .bklg/docs-that-teach/application-author-path/_design.md` empty; each `routed to` target existing as a real item id (HS-P0020 / HS-P0023 / HS-P0024 / the `support` initiative). |
| AC-005 | GIVEN every composition number in the signed-off design was measured on the **pre-merge** render of a file the merge replaces, WHEN the baseline is recorded, THEN `_baseline.md § Composition baseline` carries the re-measured value for each composition invariant the merge could have moved — the count of literal `[bracket]` pairs rendered on the crate root (four before; anti-pattern 2 targets zero), the count of `#`-hidden lines in the crate-root fence and whether any of them carries a `Query`, `Tags`, `Guard`, `AppendCondition`, the append call or an assertion (forbidden by the transience policy, `_design.md:524`), the merged fence's widest line against the **68-column** budget and its rendered height against the **32-line** crate-root ceiling, and every `##` heading over **22 characters** — each read off `target/doc/happenstance/index.html` rather than argued from the source; and the authored diff ships **zero teaching content**: no Rust fence, no answered-need line, no mapping table, and no control outside rustdoc's own chrome (anti-pattern 8). | `cargo doc -p happenstance --no-deps` (`_design.md:1025-1027` names it as the cheapest way to re-take these measurements), then measurement off `target/doc/happenstance/index.html` with each number written into `_baseline.md § Composition baseline` next to the design's pre-merge figure; `git diff --stat HEAD^1 HEAD` on the merge showing no authored file, and the PR boundary fence holding under `redkiln verify --grain story`. |

**Coverage of the traced project AC.** AC-014 (`project.md:274-277`) has two halves — *completed
and recorded before the first page is authored*, and *the opening encounter is written against
the merged `crates/happenstance/src/lib.rs`*. AC-001 and AC-002 discharge the first half (the
merge exists, is observable, and is green). AC-003, AC-004 and AC-005 discharge the second: they
are what make "written against the merged file" mean something an author can act on rather than
a hope, by handing the four page stories re-resolved anchors, dispositioned contradictions and
re-measured composition numbers. There is no sixth criterion because there is no sixth
obligation: AC-014 is the only project AC this story traces to (`_storymap.md:121`).

## Interaction quality

This story renders **no new reader-facing surface** — it ships no page. It does, however, put
one existing surface into the state every later story inherits: `crate-root-encounter`, route
`/happenstance/index.html`, whose file the merge replaces wholesale. So both families apply, and
both are carried by rows in the table above rather than by bullets here. The list below says
*which* row carries each invariant and how it is checked. Nothing in this section is an
additional obligation; a bullet here with no AC id would be ungated and untested.

**STATE invariants.**

- **In-place, not a context jump — AC-003.** A downstream author resolving ES-25 or the
  `"tests"` step must stay in their own story: the answer is in `_baseline.md § Anchors`, in
  this project's tree, not behind `git log`, a branch checkout, or a re-read of
  `spec/SPECIFICATION.md`. Verified by the cold read named in AC-003's verification cell.
- **Non-occlusion — AC-001.** The merge brings 223 commits across and must not occlude what this
  branch authored: the planning corpus under `.bklg/**`, the two telemetry files and
  `references/seeds/user-documentation.md` survive the merge unchanged. Verified by the scoped
  `git diff` in AC-001.
- **Preserved selection / prior state — AC-004.** The human sign-off already recorded in
  `_design.md` is the "selection" this story must not clear. It is preserved byte-identically;
  contradictions are recorded beside it, never folded into it. Verified by the empty `git diff`
  on that path.
- **Reversibility — AC-002.** One merge commit, first parent this branch's pre-merge tip, both
  parents recorded, so the baseline backs out with `git revert -m 1` instead of being unpicked.
  Verified by `git rev-list --parents -n 1 HEAD` recorded in `§ Gate`.
- **Keyboard reachability — n/a, recorded rather than dropped.** This story adds no control. The
  only rendered surface at baseline is rustdoc's own chrome, whose keyboard behaviour is
  rustdoc's; `_design.md`'s anti-pattern 8 forbids this project from adding any control rustdoc
  does not already ship, and AC-005's "no control outside rustdoc's own chrome" clause is the
  check that keeps that true here.

**COMPOSITION invariants** (from `_design.md`, binding — `:10-12`, sign-off row `:1039`).

- **Presentation exists at all — AC-001.** The strongest failure available to this story is a
  merge that lands, passes every text-level assertion, and leaves a crate root that never renders
  — the documentation analogue of an unstyled DOM satisfying every attribute check. AC-001
  therefore requires the doc build to run and `#main-content details.top-doc > div.docblock`
  (`_design.md:46-49`, read out of the real rendered `index.html`) to be present on the merged
  tree. A green `git merge` is not evidence of this; only the build is.
- **Composition and placement — AC-005.** Anti-pattern 2 (a literal `[bracket]` pair where an
  intra-doc link failed to resolve) was measured at **four** on the pre-merge crate root, and
  `cargo doc` did not warn (`_design.md:886-921`, gap 6). The file was replaced, so the count is
  unknown until re-measured; recording it is what lets `boundary-refusal-encounter` be held to
  zero against a real starting number rather than an inherited one.
- **Transience — AC-005.** The design's hardest transience finding is that hidden doctest lines
  are **binary in this medium**: `#`-prefixed lines are absent from the rendered DOM entirely,
  with no hover, focus or toggle that recovers them, so they are forbidden for any `Query`,
  `Tags`, `Guard`, `AppendCondition`, the append call, or any assertion (`_design.md:524`).
  AC-005 requires the merged crate root's hidden lines to be counted and classified against that
  rule at baseline — the one place a violation could arrive silently, by parentage rather than
  by authorship.
- **Density budget, with its real numbers — AC-005.** Fence width **68 columns** hard (72 is
  where `overflow-x` engages at 1024×768 on the 696px fence); fence height **24 rendered lines**
  on a step page and **32 on `crate-root-encounter` alone**, the exemption granted at the design
  gate because the refusal program is 31 rendered lines with exactly one line eligible for hiding
  (finding F-4); `##` heading length **22 characters** before the sidebar TOC truncates with an
  ellipsis; paragraph length **435 characters** (`_design.md:532-620`). Every one of those was
  measured against a 75-line `lib.rs` that no longer exists after the merge. AC-005 re-measures
  the ones the merged file can violate today and records the delta; it does not re-decide a
  budget, which would be re-opening a signed-off design.
- **Hierarchy — AC-001.** The crate root's hierarchy at baseline is whatever the merge brought;
  this story neither restructures it nor lets it go unobserved. The rendered `details.top-doc`
  region resolving is the check that the hierarchy the design's `## Composition` addresses is
  the one actually in the tree.
- **Named anti-patterns — AC-005 (2, 8, 14, and the transience clause), AC-004 (13).** Anti-pattern
  13 forbids `use happenstance_core::` on any page this project authors and permits the other
  crate's name in exactly one prose sentence — the vocabulary seam on the handoff page. The merge
  deletes that sentence's premise by changing the worked example's own import to `happenstance`
  (ADR-0006, `.kb/decisions/0006-bare-name-to-the-typed-layer.md`), which is why AC-004 requires
  the contradiction to be *dispositioned* and routed to `surface-course-subscriptions` rather
  than resolved here. Anti-patterns 1, 3, 6, 7, 9, 10, 11, 12 and 15 concern authored page
  content and cannot fire in a PR that authors none — AC-005's "zero teaching content" clause is
  what keeps that true instead of assumed.

## Error conditions

| id | condition | required response |
| --- | --- | --- |
| EC-001 | `git merge` reports a conflict, contradicting the `merge-tree --write-tree` dry run recorded in the Executive summary. | **Stop and report.** A conflict resolution is an authored edit: name every conflicting path in this spec's PR boundary fence *before* resolving it, then resolve. Do not widen the fence to `**` and do not resolve blind. If the conflict lands inside `crates/`, `spec/` or `xtask/`, the resolution also inherits AC-002's gate obligation on the resulting commit. |
| EC-002 | `cargo xtask ci --fast` is red **on the merge commit**. | Do not proceed to `tension-resolutions` and do not author on top of a red baseline — that is the exact failure mode this story exists to prevent. Diagnose whether the failure is the merge's own (a genuine interaction between the two branches) or the sibling's pre-existing defect. Record either way in `§ Gate`; a sibling defect routes to the `support` initiative (`.redkiln/config.yaml:5`) and the slice halts loudly rather than continuing on an amber baseline. |
| EC-003 | A clause id (ES-25, VT-30, ES-26, ES-27, CF-7) does not resolve on the merged `spec/SPECIFICATION.md`. | Escalate rather than repoint. Ids are stable and never renumbered (`spec/SPECIFICATION.md:280`), so a missing id is a semantic change on the sibling, not line drift. Record the id, the last known text and the merged state in `§ Anchors`, and route to HS-P0023 as a pointer gap. Substituting a nearby clause silently would hand every downstream page a citation nobody chose. |
| EC-004 | The sibling tip has moved past `3f49ec6` between this spec and its implementation. | Re-measure all four dispositions and every anchor row against the tip actually merged, and record that sha in `§ Merge`. The four contradictions in the Context pack are stated as measurements against a named commit precisely so a moved tip invalidates the measurement rather than the story. |
| EC-005 | The merged tree contradicts a **binding** design statement in a way that cannot be carried — e.g. the merged crate-root program cannot be brought under the 32-line ceiling, or a hidden line carries a boundary construct the transience policy forbids. | Record it in `§ Composition baseline` as a design gap with its measured number, and route it to the sign-off owner as a condition on `boundary-refusal-encounter`. Do **not** edit `_design.md`, and do not quietly relax a number — the design gate rejected this project once for exactly that (`_design.md:1041-1044`). |
| EC-006 | The merge changes dependency resolution or breaks the MSRV floor (223 commits may bring new crates). | Record it; the floor is 1.97.1 and equals the toolchain pin (ADR-0029, CLAUDE.md constraint 5), so a break surfaces in CI's dedicated `msrv` job rather than in `--fast`. A floor break is the sibling's decision to record, not this story's to re-take. |
| EC-007 | `redkiln verify --grain story` fails on the ~231 files the merge's second parent brings, because a merge commit does not fit a glob. | Not a scope violation and not a licence to widen the fence. Record the tool's output, confirm `git diff HEAD^1 HEAD` is empty of authored files, and carry the discrepancy as a process observation for the closeout rather than editing the fence to make a tool quiet. |

## Non-functional

| id | requirement | why |
| --- | --- | --- |
| NF-001 | The gate result is recorded **against a sha**, not as a claim: `§ Gate` names the merge commit's sha, the exact command, and the outcome. | "The gate was green" without a commit is unfalsifiable, and this story's entire value to the next four is that it is falsifiable. |
| NF-002 | `_baseline.md` is self-sufficient cold. An implementer resolves every anchor from it without re-running git archaeology, and every row names the command that re-derives it. | The record replaces work, or it is a second thing to read. Progressive disclosure only pays if the disclosed layer is complete at the point of disclosure. |
| NF-003 | Every anchor is stated id-first, line-second, with its re-derivation command. | Line numbers rot — this story exists because they rotted once. Stating the id makes a later drift a five-second repair rather than a second archaeology. |
| NF-004 | The preflight stays tier 0 and one-time: `--fast`, not the full gate. | The project is `terminal: false` (`project.md:17`), so `integration_scoped` is the applicable bar (`.redkiln/config.yaml:55`); the whole-initiative `cargo xtask ci` re-observation is HS-P0025's at closeout, and running it twice on an unchanged tree buys nothing. |
| NF-005 | No authored change to any dependency a downstream consumer resolves. | The merged `[dev-dependencies]` arrives by parentage, and dev-dependencies are not resolved by downstreams (`_design.md:761`). This story must not turn a preflight into a semver-relevant event. |
| NF-006 | The record's own vocabulary is the taught vocabulary: `happenstance`, not `happenstance_core`, wherever it names the crate a reader installs. | ADR-0006 (`.kb/decisions/0006-bare-name-to-the-typed-layer.md`) gave the bare name to the typed layer; a baseline record that says otherwise seeds the wrong word into four downstream stories. |

## Implementation notes (non-prescriptive)

- **Order that keeps every failure attributable.** Re-run the dry run
  (`git merge-tree --write-tree initiative/docs-that-teach initiative/from-contract-to-published-library`)
  to confirm the world has not moved; capture the pre-merge sha; merge with an explicit merge
  commit; capture `git rev-list --parents -n 1 HEAD`; run the gate; *then* write the record. A
  record written before the gate runs is a prediction.
- **Measure, do not transcribe.** Every number in `§ Anchors` and `§ Composition baseline` should
  come from a command run on the merged tree in this session. The pre-merge figures in the
  Context pack are there to be *compared against*, not copied — that is the whole mistake this
  story is preventing downstream.
- **`cargo doc -p happenstance --no-deps` is the cheap instrument** for AC-005 (`_design.md:1025-1027`).
  The hidden-line audit is a diff between the fence in `crates/happenstance/src/lib.rs` and the
  fence as it appears in `target/doc/happenstance/index.html`: lines present in the source and
  absent from the DOM are the hidden ones, which is also the only way to see them, since there is
  no toggle that recovers them.
- **A disposition is three fields, not a sentence**: what the design says, what the merged tree
  says, and one of `carried` / `routed to <item>` / `fixed here`. Anything longer starts
  re-arguing the design.
- **Keep `§ Anchors` a table.** The four page stories will open it under time pressure mid-authoring;
  prose that has to be read to be searched is a context jump wearing a different hat.
- The `_baseline.md` filename is this spec's choice, not a redkiln convention — it lives in the
  story folder because that is the only place this story may author, and the leading underscore
  marks it a companion rather than a stage artifact, matching `_ledger.md` beside it.

## Tests and CI (merge gate)

Grounded in the project testing brief's five tiers (`_decomposition.md:480-486`), which this
story rides rather than extends. Tiers 2, 3 and 4 are stated as not-applicable rather than
omitted, per that brief's own rule that "n/a tiers are stated, not omitted" (`:517-518`).

| tier | command / path | proves |
| --- | --- | --- |
| 0 — preflight (this story's own tier, `_decomposition.md:535`) | `git merge-base --is-ancestor initiative/from-contract-to-published-library HEAD`; `git rev-list --parents -n 1 HEAD` | AC-001, AC-002 — the merge is real, is an ancestry change and not a diffstat, and is a single revertible commit with both parents recorded. |
| story grain (redkiln `affected_gate`) | `cargo xtask affected --base main` (`.redkiln/config.yaml:41`) | AC-002 — the story's checkpoint commit does not break what its diff could reach; runs whether or not anyone types it. |
| integration grain, non-terminal (redkiln `integration_scoped`) | `cargo xtask ci --fast` (`.redkiln/config.yaml:55`), run **at the merge commit** | AC-002 — 223 inherited commits of source are green *before* a page inherits them. This is the row that makes the baseline a baseline. |
| 1 — structural, citations | `cargo xtask spec-trace` — REQUIRED step (`xtask/src/main.rs`) | AC-003 — every clause id the record hands downstream resolves on the merged text; the mechanical half of the anchor table. |
| static reachability (redkiln `reachability_static`) | `cargo xtask lints && cargo xtask spec-trace` (`.redkiln/config.yaml`) | AC-003 — the file-reading lints and the trace both run on a story whose deliverable maps to no package, which is exactly this one. |
| 1 — structural, render | `cargo doc -p happenstance --no-deps`, plus the REQUIRED `documentation` step's `RUSTDOCFLAGS=-D warnings` build inside `--fast` | AC-001, AC-005 — the crate-root surface actually renders and its selector resolves; the composition numbers are read off the render, not argued. |
| 2 — compiled fence | **n/a.** This story authors no fence; HS-P0020's step does not exist yet (`_design.md:886-921`, gap 4) and nothing here consumes it. | — |
| 3 — executed | **n/a.** No doctest and no `#[tokio::test]` is authored. The merged example's new `tests/runs.rs` and `tests/ui.rs` are swept by the `"tests"` REQUIRED step inside `--fast` as inherited content, which AC-004 records rather than adds. | — |
| 4 — falsification drill | **n/a.** DoD item 3's drill is `boundary-falsification-drill`'s, and it depends on this story's baseline; running it here would prove nothing about a page that does not exist. | — |
| 5 — review sign-off | A reviewer walks `_baseline.md` against `_design.md` and `project.md`'s DoD item 8, recording the result | AC-004, AC-005 — dispositions are honest and complete, and no teaching content shipped. `design.capture` is deliberately absent from `.redkiln/config.yaml`, so this written record is the only record these observations will ever have. |
| ledger gate | `redkiln verify --grain story` over `_ledger.md` | all five — every AC present, satisfied, and carrying non-placeholder evidence before `implement → report`. |

## Risks and coupling (PR-scoped)

| risk | likelihood / impact | mitigation in this PR |
| --- | --- | --- |
| The sibling tip moves past `3f49ec6` between spec and implementation, invalidating the four measured dispositions. | Medium / Medium | EC-004: every measurement is stated against a named sha and re-taken at implementation; `§ Merge` records the tip actually merged. |
| A red gate arriving with the merge is attributed to a later page's author. | Low / High | AC-002 runs `--fast` on the merge commit itself and records the sha. This is the single highest-value line in the story. |
| `redkiln verify --grain story` trips on the merge's second parent (~231 files). | High / Low | EC-007 and the PR boundary paragraph: the fence states what this story *authors*; the discrepancy is recorded, not papered over by widening the fence. |
| The implementer, meeting a contradicted design statement, fixes `_design.md` "while they are in there". | Medium / High | AC-004 requires `_design.md` byte-identical and routes every contradiction. The design gate already rejected this project once for numbers being relaxed rather than re-derived (`_design.md:1041-1044`). |
| This story is confused with HS-P0025's `durable-audience-closeout/merge-forward-baseline`. | Low / Medium | The Behavior table's last row states the distinction explicitly; both stories exist and neither substitutes for the other — that one re-observes the whole initiative at closeout, this one preflights one project's authoring. |
| Stale line citations remain inside `project.md`, `_grounding.md`, `_storymap.md` and `_decomposition.md`, and a downstream author opens one of them directly instead of `_baseline.md`. | Medium / Medium | Out of scope by design (PR boundary); `§ Anchors` is the superseding table and the anchor rows below point every page story at it first. The pointers themselves are repaired by the closeout's reference reconciliation (`_design.md:736-741`, HS-P0025). |
| The merged crate root violates a density or transience budget in a way no story is scoped to fix. | Medium / Medium | AC-005 measures it and EC-005 routes it as a condition on `boundary-refusal-encounter`, whose whole job is rewriting that file — so the finding lands on the story that can act on it. |

## Dependencies

**Blocks on:** none. `depends_on: []`, matching `_storymap.md:55` (its `depends_on` cell is `—`)
and the story item's `blocked_by: []`. This is the first story in the merge order
(`_storymap.md:138`) and nothing in this project may start before it.

**Unlocks:**

- `tension-resolutions` (HS-S0184) — its slice-mate, which blocks on this story
  (`_storymap.md:56`) and is delivered with it as one integrated slice.
- `boundary-refusal-encounter` (HS-S0185) — blocks on this story directly
  (`_storymap.md:57`); it is authored against the merged `crates/happenstance/src/lib.rs` and
  consumes `§ Anchors`, `§ Dispositions` and `§ Composition baseline` in full.
- Transitively, every remaining story in this project: `boundary-falsification-drill`,
  `invariant-to-appendcondition-bridge`, `surface-course-subscriptions`,
  `fence-inventory-and-clause-audit`, `answered-need-and-anchor-review` — none may be authored
  before the merge is recorded (AC-014, and `_design.md`'s first sign-off condition, `:1039`).

**Cross-branch, and deliberately not a DAG edge:** `initiative/from-contract-to-published-library`.
It is a precondition on implementation, not a blocking Redkiln item; nothing here waits on that
branch's own merge to `main` (`project.md:326-329`).

## Anchors (progressive disclosure)

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `.bklg/docs-that-teach/application-author-path/_design.md` | The signed-off, binding design. Its `## Items` row for `tokio` (`:284-306`), its transience policy (`:507-531`), its density budget with the real numbers (`:532-620`) and its anti-patterns (`:831-885`) are the statements this story measures the merged tree against. Its sign-off row (`:1039`) makes this merge its own first condition. | Before writing `§ Dispositions` and `§ Composition baseline` — and never with an editor open on it. | AC-004, AC-005 |
| `.bklg/docs-that-teach/application-author-path/project.md` | Carries AC-014 verbatim (`:274-277`), the nine Definition-of-done items including item 8's "the merge forward is recorded" and item 9's routing rule (`:279-302`), and the "no test target" claim about the worked example (`:73-77`) that the merge falsifies. | First, before the merge — it is the source of the obligation and of the routing targets. | AC-002, AC-004 |
| `.bklg/docs-that-teach/application-author-path/_decomposition.md` | The UX brief (`:11-447`) and testing brief (`:448-650`). The tier table (`:480-486`), the AC-to-tier mapping that puts this story at "tier 0 preflight" (`:535`), and the merge-gate command list (`:543-574`) are where this spec's Tests table comes from. | Before running the gate, to confirm which command is the applicable bar and which tiers are honestly n/a. | AC-002, AC-003 |
| `.bklg/docs-that-teach/application-author-path/_storymap.md` | The backbone activity A1 (`:41`), this story's row and one-line slice (`:55`), the slice rationale (`:66-69`), the AC-to-story mapping (`:121`) and the merge order (`:138`). It is what makes the Dependencies section checkable rather than asserted. | When confirming what unlocks on completion, and before touching any slice-mate. | AC-001, AC-003 |
| `.bklg/docs-that-teach/application-author-path/_grounding.md` | The measured evidence for why the *file* matters more than the branch — the `Tags::empty()`-twice defect at `:38`/`:55` of the sibling's copy, absent from this branch (`:163-170`), and DR-13's statement of the merge-forward rule (`:23-32`). | Immediately after the merge, when verifying AC-001's read of `crates/happenstance/src/lib.rs`. | AC-001 |
| `crates/happenstance/src/lib.rs` | The mount point. 75 lines before, 237 after; the file every later page is authored against and the render source for `crate-root-encounter`. Reading it is the check that the merge happened in the tree and not only in the reflog. | Directly after the merge, before anything else is recorded. | AC-001, AC-005 |
| `examples/course-subscriptions/src/main.rs` | Two of the four dispositions live here: the import changed to `happenstance` and the module doc grew from `:1-19` to `:1-28`. AC-010's citation and `_design.md`'s vocabulary-seam sentence both point at it. | While writing `§ Dispositions`, and again by `surface-course-subscriptions` later. | AC-003, AC-004 |
| `spec/SPECIFICATION.md` | The clause corpus. `:280` states that clause ids are stable and never renumbered — the single sentence the whole anchor strategy rests on — and ES-25, VT-30, ES-26, ES-27 and CF-7 are re-resolved against the merged text. | While building `§ Anchors`; open by id search, never by scrolling to a remembered line. | AC-003 |
| `xtask/src/main.rs` | The `REQUIRED` array and the `"tests"` step, both of which move line-wise across the merge and both of which downstream stories cite. It is also the definition of what `cargo xtask ci --fast` actually runs. | While building `§ Anchors`, and when interpreting a gate failure under EC-002. | AC-002, AC-003 |
| `.redkiln/config.yaml` | `affected_gate` (`:41`), `integration_scoped` (`:55`) and `reachability_static` are the commands redkiln runs at this story's and this project's grains whether or not anyone types them; `:5` names the `support` initiative that incidental bugs route to. | Before the gate run, and again when routing anything found. | AC-002, AC-004 |
| `.kb/decisions/0006-bare-name-to-the-typed-layer.md` | The Accepted decision atom behind the taught vocabulary. The merge has already applied it to the worked example, which is *why* the vocabulary-seam sentence's premise disappears — this atom is the reason, not an incidental. | When writing the worked-example disposition, and before choosing any crate name in the record. | AC-004 |
| `.bklg/docs-that-teach/initiative.md` | The gold source. DoD scenarios 1, 3 and 4 (`:412-422`) are what this story makes reachable; the vision framing is what keeps the record from drifting into a changelog. | Once, before writing the record's opening paragraph. | AC-001, AC-002 |
| `.bklg/docs-that-teach/_decomposition.md` | "Decisions taken at the gate" is where the merge-forward operational rule was recorded, and where the counter-argument about clause citations was answered. | If anyone questions why this story exists at all, or proposes deferring the merge. | AC-001 |
| `.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md` | Persona 1's stated fear — "a mental model that looks right, compiles, runs, and is quietly wrong" (`:99-106`) — is the user intent every AC above is framed from, and the reason a stale-tree page is a real harm rather than rework. | When framing the record for a reader, and if an AC's user-intent framing is ever questioned. | AC-001, AC-005 |
| `.bklg/docs-that-teach/durable-audience-closeout/merge-forward-baseline/story.md` | The *other* merge-forward story, HS-P0025's. Reading it is how the implementer confirms they are not doing that one's job — it re-observes the whole initiative at closeout against whatever tree exists then. | Once, at the start, to fix the scope boundary between the two. | AC-005 |

## Clarifications resolved during spec

1. **The AC set is exactly the five ids the first pass decided** — AC-001 through AC-005. None
   added, none dropped. The ledger matches.
2. **The baseline record's path and shape are named here**:
   `.bklg/docs-that-teach/application-author-path/merge-forward-preflight/_baseline.md`, with
   the five sections `§ Merge`, `§ Gate`, `§ Anchors`, `§ Dispositions`, `§ Composition
   baseline`. The front half said "a companion under this story's own folder"; a ledger row
   needs a path, so one is fixed. The name is this spec's choice, not a redkiln convention.
3. **Composition invariants are re-measured, not re-decided — and that is AC-005's whole point.**
   Every number in `_design.md`'s density budget and anti-pattern list was measured against a
   75-line `lib.rs` the merge replaces. The temptation is to treat "no page authored" as "no
   composition obligation"; the opposite is true, because a composition assumption that silently
   became false by parentage is exactly the kind of defect no text-level check can see. AC-005
   forces the render to be built and looked at.
4. **Keyboard reachability is recorded as not-applicable, with its reason**, rather than dropped:
   this story adds no control, and anti-pattern 8 forbids this project from adding one. Dropping
   it silently would leave a reader of this spec unable to tell whether it was considered.
5. **Testing tiers 2, 3 and 4 are stated n/a with reasons**, per the testing brief's own rule
   (`_decomposition.md:517-518`). A merge preflight that claims a compiled-fence tier it does not
   exercise would be the decorative-check failure CLAUDE.md names.
6. **A merge conflict changes the PR boundary before it changes the tree** (EC-001). The fence
   is the narrowest honestly-true statement of what is *authored*; a resolution is an authored
   edit and must be named in the fence first. This makes explicit what the front half's PR
   boundary paragraph implied.
7. **`redkiln verify --grain story` tripping on the merge's second parent is an expected
   observation, not a scope violation** (EC-007). It is recorded for the closeout rather than
   silenced by widening the fence — a tool that cannot express "arrived by parentage" is a
   process gap, and pretending otherwise would make every future merge-bearing story widen its
   fence to `**`.
8. **Nothing here re-opens DT-1, DT-4, DT-5 or DT-6.** Where a merged fact contradicts a signed-off
   design statement, this story records and routes (AC-004, EC-005). The sign-off row is a
   human's, and this is a preflight PR.
