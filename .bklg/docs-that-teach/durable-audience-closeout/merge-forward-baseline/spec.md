---
item: HS-S0172
stage: spec
created: 2026-08-17T13:16:24.878Z
updated: 2026-08-17T13:16:24.878Z
template_sig: 87bbf1d0
rendered_sig: b58f0070
---

# Spec — Merge the sibling branch forward and name the baseline tree

## Scope lock

| | Path |
| --- | --- |
| Initiative charter | [`.bklg/docs-that-teach/initiative.md`](../../initiative.md) — the fifteen DoD scenarios (`## Definition of Done`), DoD-11 and DoD-15 |
| Initiative decomposition | [`.bklg/docs-that-teach/_decomposition.md`](../../_decomposition.md) — the merge-forward decision (`:89-103`), the parallel-and-reconcile decision (`:116-123`), the terminal flag (`:306-308`) |
| Project item | [`.bklg/docs-that-teach/durable-audience-closeout/project.md`](../project.md) — AC-012, DR-2, DR-11, DR-12, and the out-of-DAG merge-forward dependency |
| This spec | `.bklg/docs-that-teach/durable-audience-closeout/merge-forward-baseline/spec.md` |
| Key briefs | [`../_decomposition.md`](../_decomposition.md) — `## Architecture brief` §1 "The seam, stated as a diff surface", §5 "Data flow, and the orderings that are load-bearing" (ordering 1), AC-A06, AC-A07, AC-A09, AC-A10; `## Testing brief` tiers 1 and 4; `## UX brief` reader U3 and IQ-3 |
| Grounding | [`../_grounding.md`](../_grounding.md) — "Reconciliation counterpart does not exist in this tree", "Tensions and risks worth carrying" (T2, the clean-checkout tension) |
| Signed-off design | [`../_design.md`](../_design.md) — `hasSurface: false`, approved 2026-08-17. **No surface, no signature, no doctest.** Nothing in this story renders |
| Story map / slice | [`../_storymap.md`](../_storymap.md) — milestone `merged-tree-baseline`, "Merge order" step 1, "Why these four and not eleven" |

## One-line PR slice

Merge `initiative/from-contract-to-published-library` forward into this branch and record the
resulting commit sha as the single named tree every downstream record, re-check and observation
cites.

## Executive summary

This PR lands **one merge commit and one record of it**. It does not author a persona, adjudicate a
pair, run an ingest wave or take a gate reading as evidence — those are the four milestones after
it.

The delta it creates is a *baseline*: before this story there is no tree in which both this
initiative's documentation work and the sibling's 223 commits of contract work co-exist, so every
observation the project is required to make (AC-001, AC-013, AC-014, AC-016) would otherwise be
taken against a copy the pull request will replace. `../_decomposition.md`'s ordering 1 — "merge
before everything… every observation downstream of it is worthless without it" — is the whole of
this story's reason to exist, and the reason the story map made it the milestone's foundation
rather than a step inside `post-merge-clause-completeness`.

Two things beyond the merge itself are in the delta, and both exist because a sha alone is not
usable by the four stories downstream:

1. **A named baseline record** at `../_baseline.md`, so "the merged tree" is one citable string
   rather than four stories each re-deriving it from `git log`.
2. **A premise re-check** — the planning corpus (`../_grounding.md`, `../_decomposition.md`) states
   three facts about *this* worktree that the merge changes. They are re-observed and handed
   forward as observations. They are not adjudicated here; each names the story that owns it.

## Context pack

The load-bearing decisions this story must honour. Everything deeper is a signposted anchor.

**D1 — Merge forward; do not block, do not rebase, do not squash.** The initiative resolved to run
in parallel and merge forward *before the pull request* rather than block on the sibling: blocking
"would put an unschedulable cross-branch dependency on this initiative's terminal project"
(`../../_decomposition.md:116-123`). The merge must produce a **merge commit with two parents** —
that is what makes one sha name both histories, and what lets a later reader answer "which tree
was this observed against?" by resolving a single ref. A rebase or a squash rewrites this branch's
six commits, invalidates every `file:line` citation already written into the planning corpus, and
destroys the ancestry that proves the sibling's work is present. Fast-forward is not available in
any case (both sides have moved), but state `--no-ff` intent explicitly rather than relying on that.

**D2 — This story observes; it never adjudicates.** The merge changes the truth of three premises
the planning corpus asserts. Each is recorded as a fact with its owner named, and left there:

| Premise as written at planning time | What the merged tree makes true | Who owns the decision |
| --- | --- | --- |
| "`.kb/_intake/` today contains exactly `README.md`" (`../_decomposition.md`, T4; `../_grounding.md`, "The `_intake/` ingest glob is real and literal") | The sibling tip carries **six** staged documents beside the README (`0031-…`, `0032-…`, `0033-…`, `0034-…`, `contract-defect-log-phase-7.md`, `happenstance-macros-verdict.md`) | `audience-ingest-wave` (AC-A02, AC-009) — this story does **not** narrow, delete or ingest any of them |
| "HS-S0131 … is **not present anywhere under `.bklg/`** in this tree" (`../_grounding.md`) | The counterpart item arrives at `.bklg/from-contract-to-published-library/closeout-and-durable-audience/persona-and-journey-intake-staging/`, and its frontmatter states its `stage` and `status` | `reconciliation-ledger` (AC-002, DR-2) — this story records the item's path and its two frontmatter values verbatim, and adjudicates nothing |
| "`.kb/decisions/` holds sixteen accepted decisions" (`../_grounding.md`, Anchors) | The sibling adds `0017`–`0022` and touches the three map atoms | `design-tension-audit` and `charter-open-question-disposition` read the post-merge corpus; **AC-A09 still forbids this project authoring or editing any decision atom** — arriving-by-merge is not authoring |

**D3 — The merge is a merge, not an edit.** `../_decomposition.md` §1 grants this project exactly
one whole-tree act — "the whole tree | merge-forward of the sibling branch | `git merge`" — and
keeps `crates/`, `spec/`, `xtask/`, `standards/`, `docs/` and `.kb/` **read only** for this
project's own authorship. The check that makes that falsifiable is available because of a fact
verified at planning time: **this branch has changed no file under any of those paths since the
merge base** (`ce933d8`; this side's six commits touch only `.bklg/`,
`.redkiln/telemetry/events/`, and `references/seeds/user-documentation.md`). A conflict-free merge
must therefore leave every one of those paths **byte-identical to the sibling tip**. Any
difference is a hand edit wearing a merge's clothes, and it is directly observable with one
`git diff`. This is the AC-003 obligation below.

**D4 — Conflicts are dispositioned in writing, and "there were none" is a disposition.** At
planning time the two sides changed **zero files in common** (this side's changed-path set,
intersected with the sibling's 1,089-file set, is empty; the only near miss is `CLAUDE.md`, which
the sibling changed by +9/-1 and this branch has not touched at all). The sibling branch is live,
so that may not hold at merge time. Either way the record states it: each conflicted path, which
side's content survived, and why — or the sentence that no path conflicted. Silence is the defect
the UX brief's IQ-8 names ("no state expressed by presence, absence, ordering or omission"), and a
reviewer cannot tell a clean merge from an unexamined one without the sentence.

**D5 — The baseline is mounted in two places, because prose alone is not machine-readable.**
`../_baseline.md` carries it for human citation; `redkiln record-links HS-S0172 --sha <merged-sha>`
carries it on the item, which is also what `require_commit_provenance: true`
(`.redkiln/config.yaml:73`) requires of any story that changed files inside its own PR boundary.
The CLI is the single writer of item frontmatter — never hand-edit `links.commits` (CLAUDE.md,
"Where the work lives"; a `PreToolUse` hook denies it).

**D6 — No new tooling, no gate ownership, no fix-ups.** The whole gate is `terminal-gate-run`'s
(AC-016) and it must run **last**, on the tree that already carries every artefact; a gate reading
taken here "proves the gate, not the deliverable" (`../_decomposition.md`, ordering 4). What this
story does take is a **baseline health observation** — the story-grain `affected` gate that
`.redkiln/config.yaml:40` already wires (`cargo xtask affected --base {{base}}`) — recorded with
its command and outcome so that a downstream failure can be attributed to the merge or exonerated
from it. A failure it surfaces is an *incidental* defect and routes to the `support` initiative
(`.redkiln/config.yaml:5`, `support_initiative: support`); it is not repaired inside this story,
because repairing sibling contract code here is the scope breach `../_decomposition.md` AC-A10
forbids in its own words ("no sibling's content is edited by this project").

**D7 — The persona-journey slice this realises.** Not U1 the charter author — U1 never reads a
baseline. This story serves **U3, the future maintainer**: *"let me see what was decided, against
which tree, on what evidence — and correct it without editing it"* (`../_decomposition.md`,
`## UX brief`, reader table), and **U2, the closeout reviewer**, who cannot check any downstream
claim without knowing which tree it was checked on. That is the entire user-observable value of a
merge commit, and it is why the sha has to be *written down* rather than merely produced.

**D8 — What the implementer must re-observe rather than trust.** Every number in this spec was
measured against sibling tip `3f49ec6` on 2026-08-17 while that branch was actively being
implemented (223 commits ahead, ~218k insertions across 1,089 files). They are orientation, not
input: the merge is performed against whatever the tip is **at merge time**, and the baseline
record states the tip it actually merged. A spec figure copied into the record instead of a
measured one is the failure this note exists to prevent.

**D9 — No ADR governs this surface, and that is a finding.** All sixteen accepted decision atoms
govern the Rust contract; none reaches documentation, KB promotion or branch policy
(`../_decomposition.md`, `## UX brief` Notes; `.bklg/docs-that-teach/initiative.md:524-525`).
Citing one here would be manufacturing authority. What binds instead is
`.kb/governance/rewrite-the-referent-never-the-reasoning.md` (supersede, never edit), the
single-write-path rule for `.kb/`, and CLAUDE.md's rule that the CLI owns item frontmatter.

## Integration contract

- **Archetype**: `foundation` — real in-tree substrate, consumed inside this initiative by
  `post-merge-clause-completeness` in its own milestone and by every record downstream that names
  its sha (`../_storymap.md`, "Merge order"). It lands a real tree state, not a placeholder.
- **Slice / milestone**: `merged-tree-baseline`. **Slice-mate**: `post-merge-clause-completeness`
  (HS-S0173), implemented in the same context. The two are one surface because "the sha the merge
  produces is the sha the clause-completeness statement must name" (`../_storymap.md`, "Why these
  four and not eleven").
- **Mount point**: `.bklg/docs-that-teach/durable-audience-closeout/_baseline.md` — a new
  project-level companion in the folder `../_decomposition.md` §1 grants this project
  (`.bklg/docs-that-teach/durable-audience-closeout/*`). It is the render path for this
  capability: the four downstream stories that must "name the merged tree (by ref or sha)"
  (AC-012) read it rather than re-deriving it. The tree-level composition root it wires into is
  the branch tip **`initiative/docs-that-teach`** itself — the ref every later `git worktree add`
  and clean checkout (AC-014, AC-A07) is created from.
- **Wires into**:
  - `refs/heads/initiative/from-contract-to-published-library` — the merge source, checked out at
    `D:/repos/happenstance/.claude/worktrees/from-contract-to-published-library`; merged **from**,
    never checked out here.
  - `redkiln record-links HS-S0172 --sha <merged-sha>` → `links.commits` on
    `merge-forward-baseline/story.md` (CLI-written; `.redkiln/config.yaml:73`).
  - `.redkiln/config.yaml:40` `affected_gate` — the story-grain verify this story's baseline
    health line reports.
  - `.redkiln/config.yaml:5` `support_initiative: support` — the route for anything the merge
    surfaces that is not this project's.
- **Renders surfaces**: **none.** `../_design.md` records `hasSurface: false` with the design gate
  cleared by the repository owner on 2026-08-17 — this project changes no public API and renders no
  screen. There is no surface id to claim and none to contradict.
- **Conformance rule(s)**: **none, and this is not adapter-observable.** This story adds no port,
  no store and no rule to `happenstance-testkit`; it compiles nothing. Naming a rule here would be
  the decorative-rule defect CLAUDE.md warns about from the other direction — a rule invented for a
  change no adapter can fail.
- **Clause(s)**: **none discharged, none amended.** `spec/SPECIFICATION.md` diverges by 521 lines
  between the two branches and every one of those lines arrives **by merge**, unedited by this
  story. Whether the sibling *added* a documentation MUST outside HS-P0020's pinned set is
  explicitly `post-merge-clause-completeness`'s question (AC-013), not this one's. `spec-trace`
  runs over the specification; it never edits it (`../_decomposition.md` §1).
- **Advances DoD scenario**: **DoD-11** — "the frozen documentation MUSTs are still discharged…
  the specification cross-reference step passes over the tree **as it stands after** every doc
  comment this work touched" — which cannot be observed at all until the sibling's doc comments are
  in the tree. It is additionally the precondition for DoD-1 (clean checkout of the assembled
  result) and DoD-15 (the audience is durable on that same tree): all fifteen re-observations
  (AC-014) are taken on the tree this story names.

## PR boundary

**In this PR**

- One merge commit on `initiative/docs-that-teach` with two parents: this branch's tip and the
  sibling branch's tip at merge time.
- `../_baseline.md` — the baseline record: the merged sha, both parents, the merge date and
  observer, the conflict disposition, the three premise re-checks, and the baseline health line.
- This story's own folder: `spec.md`, `_ledger.md` (`require_ledger: true`,
  `.redkiln/config.yaml:62-67`) and the stage artefacts the CLI renders.
- `redkiln record-links HS-S0172 --sha <merged-sha>` (CLI-written item frontmatter).

**Explicitly not in this PR**

- Any hand edit to a file the merge brought in — `crates/`, `spec/`, `xtask/`, `standards/`,
  `docs/`, `.kb/` are read-only to this project's authorship (`../_decomposition.md` §1, AC-A10).
- Narrowing, deleting, moving or ingesting anything in `.kb/_intake/` — the merge delivers six
  sibling-staged documents there and they stay exactly as delivered; `audience-ingest-wave` owns
  the wave and its file list (AC-A02).
- Any adjudication of persona or journey pairs (`reconciliation-ledger`), the clause-completeness
  set-difference statement (`post-merge-clause-completeness`), the DT audit
  (`design-tension-audit`), the fifteen re-observations (`dod-scenario-ledger`), or a
  `cargo xtask ci` reading offered as AC-016 evidence (`terminal-gate-run`).
- Any `.kb/decisions/` addition or edit, and any ADR written as a side effect (AC-A09;
  `.kb/governance/rewrite-the-referent-never-the-reasoning.md`).
- Fixing whatever the merged tree's affected gate reports — that routes to `support`
  (`.redkiln/config.yaml:5`).

**Merge DoD one-liner**: the branch tip is a two-parent merge commit whose sha is written in
`../_baseline.md` and recorded on HS-S0172, and every read-only path in it is byte-identical to
the sibling tip it merged.

```
**
```

**Why the boundary is `**`, deliberately and once.** `redkiln verify --grain story` reads the first
fenced block above and fails on any file changed outside it. This story's diff *is* the whole tree
by construction — a 1,089-file merge is the one act `../_decomposition.md` §1 grants at whole-tree
scope — so a narrow glob here would not be "honestly true"; it would be a boundary that fails on
the story's entire reason for existing. The widening is stated here rather than discovered at the
gate, per the template's own instruction. It is not a licence: the compensating control is AC-003,
which is *stricter* than a glob because it checks content rather than paths — outside this story's
two authored globs
(`.bklg/docs-that-teach/durable-audience-closeout/_baseline.md`,
`.bklg/docs-that-teach/durable-audience-closeout/merge-forward-baseline/**`), every read-only path
must be byte-identical to the sibling tip, so a hand edit anywhere in the merged 1,089 files is
caught even though the glob admits it. No other story in this project may cite this precedent;
every one of the other ten is confined to `.kb/` and this project's folder (`../_storymap.md`).

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **Preflight: the tree is committed before the merge** (AC-001) | The worktree is clean — `git status --porcelain` empty — before `git merge` runs. Planning-time reality is that it is **not**: sibling planning passes are writing spec bodies and untracked `_ledger.md` files across four projects concurrently, and `.redkiln/telemetry/events/ryan-britton@docs-that-teach.jsonl` is staged. A merge started on a dirty tree either aborts or silently mixes uncommitted planning work into the baseline commit, which makes the sha name something nobody reviewed. | `git status --porcelain`; `../_decomposition.md` §5 ordering 1 |
| **The merge produces one two-parent commit** (AC-001) | `git merge --no-ff initiative/from-contract-to-published-library` from `initiative/docs-that-teach` inside this worktree. Never `rebase`, never `squash`, never `checkout` of the sibling branch (it is checked out in its own worktree, `D:/repos/happenstance/.claude/worktrees/from-contract-to-published-library`). Afterwards `git rev-list --count HEAD^2..HEAD` resolves and the sibling tip is an ancestor of `HEAD` (`git merge-base --is-ancestor <sibling-tip> HEAD`). | `../../_decomposition.md:89-103`, `:116-123`; `git worktree list` |
| **The baseline record exists and is citable in one line** (AC-002) | `../_baseline.md` states, each as a literal string a reader can copy: the merged sha (full 40 hex), parent 1 (this branch's pre-merge tip), parent 2 (the sibling tip merged), the sibling branch name, the date, the observer, and the branch the commit sits on. Written for U3's question — "against which tree, on what evidence" (`../_decomposition.md`, `## UX brief`, reader table). Every downstream record cites this file; none re-derives the sha from `git log`. | `../_decomposition.md` §6 ("where the reconciliation record lives" — a companion under this project's folder), AC-012 |
| **The merge introduced no hand edits** (AC-003) | `git diff --name-only <sibling-tip> HEAD -- crates spec xtask standards docs examples .kb` is **empty**, and `git diff --name-only <sibling-tip> HEAD -- references` names at most `references/seeds/user-documentation.md`. This holds because this branch changed nothing under those paths since merge base `ce933d8` — verified at planning time, and re-verified at merge time before the diff is read as evidence. A non-empty result is a hand edit and fails the story. | `../_decomposition.md` §1 (the seam table), AC-A10 |
| **The conflict disposition is written, including "none"** (AC-004) | One row per conflicted path — the path, which side's content survived, and the one-sentence reason — or the explicit sentence that no path conflicted. Planning-time measurement: the two sides' changed-path sets do not intersect, so the expected outcome is no conflict; the sibling branch is live and this must be re-observed, not assumed. Where a conflict does occur in a read-only path, the sibling's content wins by default (this project authors nothing there) and the row says so. | `../_decomposition.md`, `## UX brief`, IQ-8 (no state by omission); AC-A10 |
| **The three premise re-checks are recorded as observations** (AC-005) | For each row of D2's table: the premise as the planning corpus states it (with its cite), what the merged tree shows (with the command that showed it), and the story that owns the decision. Verbatim values only — for `.kb/_intake/` the post-merge file listing; for the counterpart, its path plus its `stage:` and `status:` as they read in the merged tree. **No disposition, no narrowing, no ingest, no adjudication in this story.** | `../_grounding.md`, "Reconciliation counterpart does not exist in this tree"; `../_decomposition.md`, T3, T4; AC-002, AC-009 |
| **The baseline is recorded on the item, not only in prose** (AC-006) | `redkiln record-links HS-S0172 --sha <merged-sha>` writes `links.commits`. Hand-editing that frontmatter is denied by a `PreToolUse` hook and is the wrong record in any case. This is also what `require_commit_provenance: true` requires of a story that changed files inside its own boundary. | CLAUDE.md, "Where the work lives"; `.redkiln/config.yaml:73` |
| **Baseline health is observed and attributed, not repaired** (AC-007) | The story-grain gate `cargo xtask affected --base main` (`.redkiln/config.yaml:40`) is run on the merged tree and its command plus outcome recorded in `../_baseline.md`. Green means downstream failures cannot be blamed on the merge; a failure is recorded verbatim and routed to the `support` initiative with the routing stated in the record. It is **not** AC-016 evidence and is not offered as one — the whole gate belongs to `terminal-gate-run`, last, on the final tree. | `.redkiln/config.yaml:40`, `:5`, `:60`; `../_decomposition.md` §5 ordering 4 |
| **Interfaces** | None. No Rust is written, no `pub` item added, no signature changed, no doctest authored — `../_design.md` records `hasSurface: false`. The only "interfaces" are three CLI verbs already in the repository (`git merge`, `redkiln record-links --sha`, `cargo xtask affected`) and one new markdown file. No new tooling is invented (AC-TB-02). | `../_design.md`; `../_decomposition.md`, `## Testing brief`, AC-TB-02 |

## Data and migrations

**N/A for schema data — but the git history *is* the datastore here, and it has one migration-shaped
rule.** This project defines no database, no serialized format and no persisted type; it adds no
frontmatter key and no atom (`../_decomposition.md` §1: `.kb/` is written only by
`/redkiln:kb-ingest`, and not in this story at all).

What is irreversible is the shape of the commit graph. A merge commit is cheap to add and
expensive to un-make once four downstream records cite its sha, so the two rules below are stated
as data rules rather than style:

- **The history is append-only from here.** After the merge, this branch's six pre-merge commits
  and the sibling's 223 must both remain reachable from the tip. Rebasing, squashing or amending
  the merge afterwards changes the sha the records name and silently invalidates AC-012 for four
  stories — the same class of defect as `../_decomposition.md`'s IQ-3 ("nothing this project lands
  may move an anchor someone else already cited").
- **A correction is a new commit, never a rewritten one.** If the merge is later found wrong, the
  route is another commit and a superseding statement in `../_baseline.md` naming both shas — the
  corpus's own discipline for corrections, `.kb/governance/rewrite-the-referent-never-the-reasoning.md`,
  applied to the one artefact of this story that is not text.

No `.kb/` atom, no map file and no item frontmatter field is written by hand in this story;
`links.commits` is written by the CLI (D5) and every `.kb/` change in the diff arrived as the
sibling's, by merge.

## Acceptance criteria

Seven criteria, each stated from the intent of a reader the UX brief names
(`../_decomposition.md`, `## UX brief`, reader table): **U2**, the closeout reviewer who must be
able to disagree with exactly one thing without reading the corpus, and **U3**, the future
maintainer who must be able to ask *against which tree, on what evidence* and get an answer from
one open file. U1 is not a reader of this story's artefact and no criterion is written for them.
Tier numbers are `../_decomposition.md`, `## Testing brief`, "The test mix".

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **GIVEN** U3 asks "does this branch actually contain the sibling's contract work, or only a claim that it does?", **WHEN** they resolve the branch tip of `initiative/docs-that-teach`, **THEN** it is a commit with **two** parents — this branch's pre-merge tip and the sibling branch's tip at merge time — created by `git merge --no-ff initiative/from-contract-to-published-library` on a worktree that was clean (`git status --porcelain` empty) before the merge ran, so the sha names one tree that reviewed work produced rather than a tree with uncommitted planning mixed into it. Not a rebase, not a squash, not a checkout of the sibling. | **Tier 1 (static).** `git rev-list --count HEAD^2..HEAD` resolves non-fatally; `git merge-base --is-ancestor <parent2> HEAD` exits 0; `git cat-file -p HEAD` shows exactly two `parent` lines; both parent shas equal the two recorded in `../_baseline.md`. `git log --oneline HEAD^1` still shows this branch's six pre-merge commits unrewritten, which is what rejects the rebase and squash implementations. |
| AC-002 | **GIVEN** U3 opens exactly one file to answer "which tree was this closeout observed against?", **WHEN** they open `../_baseline.md` and read nothing else, **THEN** they get the full 40-hex merged sha, both parent shas, the sibling branch name, the branch the merge sits on, the merge date and the observer — each as a literal copyable string, none of them behind a link, a `git log` invocation, or a phrase like "see the item". Downstream records cite this file; none re-derives the sha. | **Tier 2 (content review)** against IQ-1's falsifier (`../_decomposition.md`, `## UX brief`, IQ-1) — the file is read *alone*, and any required value that reads "see …" is a defect. **Tier 1** cross-check: every sha string in `../_baseline.md` resolves (`git cat-file -e <sha>^{commit}`), and the merged sha equals `git rev-parse HEAD`. |
| AC-003 | **GIVEN** U2 must be able to trust that this project edited none of the sibling's files while claiming only to have merged them, **WHEN** they diff the merged tip against the sibling tip over the paths `../_decomposition.md` §1 marks read-only, **THEN** the result is empty: `git diff --name-only <parent2> HEAD -- crates spec xtask standards docs examples .kb` prints nothing, and `git diff --name-only <parent2> HEAD -- references` names at most `references/seeds/user-documentation.md`. Every cited `file:line` in the planning corpus therefore still means what it meant. | **Tier 1 (static)**, and it is the compensating control for the `**` PR boundary: it checks *content* rather than paths. Both commands are recorded verbatim in `../_baseline.md` with their output. A non-empty result is a hand edit and fails the story outright (AC-A10, `../_decomposition.md`). Also discharges IQ-3 and IQ-6. |
| AC-004 | **GIVEN** U2 cannot tell a clean merge from an unexamined one by looking at the tree, **WHEN** they read `../_baseline.md`'s conflict section, **THEN** they find either one row per conflicted path (the path, which side's content survived, and the one-sentence reason) or the explicit sentence that **no path conflicted** — never an absent section, an empty heading, or a summary that replaces the rows. Where a read-only path conflicted, the row states that the sibling's content won and why. | **Tier 2 (content review)** against IQ-8's bar — "no state expressed by presence, absence, ordering or omission" (`../_decomposition.md`, `## UX brief`, IQ-8) — with **Tier 1** corroboration: `git log -1 --format=%B HEAD`, and where any conflict occurred `git diff <parent2> HEAD -- <conflicted-path>` showing the stated survivor. The reviewer's falsifier is a baseline record with no conflict sentence at all. |
| AC-005 | **GIVEN** U2 and U3 need the planning corpus's three now-stale premises re-observed rather than silently outgrown, **WHEN** they read `../_baseline.md`'s premise section, **THEN** each of D2's three rows appears as premise-as-written (with its cite) → what the merged tree shows (with the command that showed it, output verbatim) → the story slug that owns the decision — and **nothing is adjudicated here**: no `.kb/_intake/` file is narrowed, deleted or ingested, no persona pair is dispositioned, no decision atom is treated as settled. | **Tier 2 (content review)**: three rows, each carrying a command and its literal output — `ls .kb/_intake/` listing the sibling-staged documents beside the README; the counterpart item's path plus its `stage:` and `status:` copied verbatim from its frontmatter in the merged tree; `ls .kb/decisions/` showing the post-merge set. **Tier 1** corroboration: `git diff --name-only <parent2> HEAD -- .kb` is empty (AC-003), which is what proves no adjudication happened in this diff. |
| AC-006 | **GIVEN** the baseline must be machine-readable and not only prose, **WHEN** `redkiln verify --grain story` reads HS-S0172 under `require_commit_provenance: true`, **THEN** `links.commits` carries the merged sha, written by `redkiln record-links HS-S0172 --sha <merged-sha>` and by nothing else — the item's YAML frontmatter is never hand-edited (a `PreToolUse` hook denies it), and the sha in `links.commits` is byte-identical to the one in `../_baseline.md`. | **Tier 1 (static).** `rg "commits" .bklg/docs-that-teach/durable-audience-closeout/merge-forward-baseline/story.md` shows the sha and it string-matches the record's; `redkiln verify --grain story` for HS-S0172 reports no missing commit provenance (`.redkiln/config.yaml:73`). |
| AC-007 | **GIVEN** a downstream story hits a failure and must know whether the merge caused it, **WHEN** they read `../_baseline.md`'s baseline-health line, **THEN** they find the exact command `cargo xtask affected --base main` (`.redkiln/config.yaml:40`), the date it ran, and its outcome — green, or the failure captured verbatim with the sentence routing it to the `support` initiative (`.redkiln/config.yaml:5`). The line explicitly states it is **not** AC-016 evidence, and nothing sibling-owned is repaired in this story. | **Tier 1** for the run itself (`cargo xtask affected --base main`, exit code recorded). **Tier 2** for the attribution: the reviewer checks the record carries the command string, the outcome, the non-AC-016 disclaimer, and — on failure — the routing sentence rather than a fix. Falsifier: a record saying "gate green" with no command and no date, or a diff containing a repair under `crates/`. |

**Coverage of the traced project AC.** `project.md:201`'s **AC-012** ("the sibling branch has been
merged forward, and both the reconciliation record and the clause-id re-check name the merged tree
(by ref or sha)") splits cleanly here: AC-001 and AC-003 discharge the *merged forward* half;
AC-002 and AC-006 produce the single named tree the other two records
(`post-merge-clause-completeness`, `reconciliation-ledger`) are required to cite (`../_storymap.md`,
Coverage, AC-012 row). AC-004, AC-005 and AC-007 are the brief-grain obligations that ride the same
act — AC-A06, AC-A10 and IQ-8 — and none of them belongs to another story.

## Interaction quality

**COMPOSITION family — declared not applicable, once, with the authority.**
`../_design.md` is signed off with **`hasSurface: false`** ("This project changes no public API…
there is no rendered surface for any of those concepts to apply to", `../_design.md`,
`## Items`), approved by the repository owner on 2026-08-17. There is no control, no chrome, no
density budget and no named visual anti-pattern for this story to honour or contradict, and
inventing one would be manufacturing a design decision the human gate never made. Presentation,
placement, transience and hierarchy therefore carry **no AC row** here, and their absence is a
recorded disposition rather than an omission.

What survives the translation is the UX brief's **text analogues**, which that brief authors
deliberately as the falsifiable form of the same invariants for a prose corpus
(`../_decomposition.md`, `## UX brief`, "Interaction-quality invariants"). Each applicable
invariant is carried by an AC row in the table above — none is left as a bullet here, because a
bullet in this section gets no ledger row and is never gated.

| Family | Invariant (source) | Carried by | How it is verified |
| --- | --- | --- | --- |
| STATE — in place, not a context jump | IQ-1: the artefact answers its question on first load; links carry provenance, never a required value | **AC-002** | Tier 2 read of `../_baseline.md` *alone*; any required value reading "see …" fails |
| STATE — non-occlusion | IQ-2: a summary may precede the rows, it may never replace one | **AC-004**, **AC-005** | Tier 2: one row per conflicted path and one row per premise; a condensing sentence in place of rows fails |
| STATE — preserved position (focus/scroll analogue) | IQ-3: nothing this project lands may move an anchor someone else already cited | **AC-003** | Tier 1: both read-only `git diff --name-only` runs are empty |
| STATE — preserved selection | IQ-6: prior citations keep meaning what they meant — the six pre-merge commits stay reachable and unrewritten | **AC-001**, **AC-003** | Tier 1: `git log --oneline HEAD^1` unrewritten; the ancestry assertion |
| STATE — reversibility | IQ-4: a correction is a new commit plus a superseding statement naming both shas, never a rewritten one (`.kb/governance/rewrite-the-referent-never-the-reasoning.md`) | **AC-001**, **AC-006** | Tier 1: a two-parent commit recorded in two places, so a later correction can name both shas rather than mutate one |
| STATE — reachable without prior knowledge (keyboard-reachability analogue) | IQ-5: a reader reaches the artefact in at most two hops, with link text naming the destination | **AC-002** | Tier 2: `project.md` → `_baseline.md` is one hop; the four downstream records link it by name |
| STATE — every state change legible at the moment of reading | IQ-8: no state expressed by presence, absence, ordering or omission | **AC-004**, **AC-005**, **AC-007** | Tier 2: the "no conflicts" sentence, the owner named on every premise row, and the health outcome with its routing are each *written words*, not inferences from an empty section |

**Why this section is not vacuous despite the empty composition family.** Every criterion above is
satisfiable by a record that is machine-correct and useless — a `_baseline.md` holding a sha and
nothing else passes every `rg` and every `git` assertion in the table. Tier 2 is what fails it, and
Tier 2's checklist is IQ-1…IQ-8 above, per **AC-TB-06** (`../_decomposition.md`,
`## Testing brief`), which forbids this story maintaining a second, divergent checklist.

## Error conditions

| id | condition | required handling |
| --- | --- | --- |
| **EC-001** | The worktree is dirty when the merge is attempted — the realistic case, since concurrent planning passes write spec bodies and untracked `_ledger.md` files across sibling projects and `.redkiln/telemetry/events/*.jsonl` is auto-staged (`.redkiln/config.yaml:12`, `auto_stage_telemetry: true`). | **Stop; do not merge.** Commit the planning work as its own commit first, so the merge commit's first parent is a tree someone reviewed. Never `git stash` across the merge — a dropped stash silently loses planning work — and never reach for `-X` to paper over it. `../_baseline.md` records the pre-merge tip sha that resulted (AC-001, AC-002). |
| **EC-002** | `git merge` reports conflicts. Planning-time measurement says the two sides' changed-path sets do not intersect, but the sibling branch is live. | Resolve **in favour of the sibling** for any path `../_decomposition.md` §1 marks read-only — this project authors nothing there — and in favour of this branch only under `.bklg/docs-that-teach/` and `.redkiln/telemetry/`. Record every resolved path in AC-004's table with its reason. If a conflict lands in a path neither rule covers, **abort** (`git merge --abort`) and escalate; do not invent a policy inside the merge. |
| **EC-003** | `initiative/from-contract-to-published-library` does not resolve, or is checked out in another worktree. | The second is expected and fine — it lives at `D:/repos/happenstance/.claude/worktrees/from-contract-to-published-library` and is merged **from**, never checked out here; `git merge <branch>` needs no checkout. If the ref itself does not resolve, stop and re-resolve or fetch rather than merging a stale local copy, and record the tip sha actually merged (D8). |
| **EC-004** | `cargo xtask affected --base main` fails on the merged tree. | This does **not** fail the story. Capture the failing output verbatim into `../_baseline.md`'s health line, state the routing to the `support` initiative (`.redkiln/config.yaml:5`), and stop there. Repairing sibling contract code here is the AC-A10 scope breach; a "quick fix" under `crates/` also breaks AC-003 by making a read-only path differ from the sibling tip. |
| **EC-005** | The sibling tip advances between the merge and the writing of the record. | The record states the tip **as merged**, i.e. `HEAD^2`, never the branch's current tip. Derive both parents with `git rev-parse HEAD^1 HEAD^2` on the merge commit itself, so this is structurally unable to drift; do not read them from `git rev-parse <branch>`. |
| **EC-006** | `redkiln record-links HS-S0172 --sha <sha>` fails, or an agent is tempted to write `links.commits` by hand. | The CLI is the single writer; the `PreToolUse` hook denies the edit and it is the wrong record regardless (CLAUDE.md, "Where the work lives"). On CLI failure, stop and report — an unrecorded sha fails AC-006 and `require_commit_provenance` (`.redkiln/config.yaml:73`), and a hand-written one fails validation later and louder. |
| **EC-007** | The story is re-run and the merge already exists (`HEAD` is already a two-parent merge of the sibling). | Idempotent by construction: do **not** merge again and do not `--amend`. Re-derive both parents from the existing `HEAD`, re-run AC-003's two diffs and the health command, and update `../_baseline.md` in place. A second merge commit would give downstream records two candidate shas — the exact ambiguity AC-012 exists to remove. |
| **EC-008** | A file under `.kb/_intake/` looks stale, duplicated or wrong after the merge. | Leave it exactly as delivered. Narrowing, deleting or ingesting is `audience-ingest-wave`'s (AC-A02, AC-009); this story records the listing as an observation and names the owner (AC-005). Acting on it here destroys the evidence that story needs at its approval gate. |

## Non-functional

| id | requirement | why it is stated |
| --- | --- | --- |
| **NF-001** | **History is append-only from this commit forward.** No rebase, squash, amend or force-push touches this branch after the merge sha is written into `../_baseline.md` or `links.commits`. | Four downstream records cite the sha; a rewrite invalidates AC-012 for all of them silently, with no failing command anywhere. This is the `## Data and migrations` rule restated as a standing obligation. |
| **NF-002** | **Every sha in the record is the full 40 hex characters**, and shas appear as bare literals a reader can copy and `git cat-file` without editing — no abbreviation, no prose paraphrase, no "the merge of yesterday". | U3's question is answered by *resolving a ref*. An abbreviated sha is ambiguous across a merge of this size and is the kind of near-miss no automated check in this repository would catch. |
| **NF-003** | **No new tooling is introduced.** The story uses `git`, `redkiln record-links` and `cargo xtask affected` — all three already resolve in this tree (`xtask/src/main.rs`, `.redkiln/config.yaml`, CLAUDE.md "Commands"). | **AC-TB-02** (`../_decomposition.md`, `## Testing brief`) forbids invented tooling, and a bespoke script for a one-shot merge is a maintenance liability no later reader can run. |
| **NF-004** | **The `**` PR-boundary widening is precedent for this story only.** No other story in this project may cite it; the other ten stay confined to `.kb/` and this project's folder (`../_storymap.md`). | A boundary that widens once by argument widens forever by citation. Recorded here so the next reviewer meets the fence rather than the exception. |
| **NF-005** | **The record is self-contained.** `../_baseline.md` depends on no external service, no CI artefact URL and no ephemeral console scrollback; everything a reader needs is in the file or re-derivable from the repository. | The closeout is read by U3 at an unknown future date. An evidence line pointing at a CI run that has been garbage-collected is the same defect as no evidence. |
| **NF-006** | **The merged tree carries the sibling's work without this story reading it.** The story asserts *byte-identity* of sibling content, never its correctness. | Reviewing the sibling's files here would be both unbounded and outside scope (AC-A10). AC-003 is deliberately a cheap total check rather than an expensive partial one. |

## Implementation notes (non-prescriptive)

Shape, not a script — the implementer owns the sequence.

- **Commit, then merge, then record.** The three phases are separable, and the record is written
  *from the merge commit* rather than from the branch refs (EC-005). Deriving both parents with
  `git rev-parse HEAD^1 HEAD^2` afterwards is the cheapest way to make the record structurally
  unable to disagree with the commit graph.
- **Write `../_baseline.md` as a record, not a narrative.** Four downstream stories read it to
  extract one string. A short block of literal values (merged sha, parent 1, parent 2, source
  branch, target branch, date, observer) followed by the conflict disposition, the three premise
  rows and the health line serves that better than paragraphs. `../_design.md` imposes no template,
  so the shape is yours — but IQ-1's "one open file, no link-following" is binding whatever you
  choose.
- **Run AC-003's two diffs immediately after the merge, before anything else touches the tree.**
  They are only meaningful against a tree nothing has been hand-edited into. Paste their output —
  including the empty output — into the record rather than asserting the result in prose.
- **Treat the slice-mate as one context.** `post-merge-clause-completeness` (HS-S0173) is
  implemented alongside this story and its `spec-trace` statement must name this story's sha
  (`../_storymap.md`, "Why these four and not eleven"). Finishing the merge and handing the sha
  over conversationally is how the two drift; the record is the hand-off.
- **`--no-ff` is stated intent even though fast-forward is unavailable.** Both sides have moved, so
  git would create a merge commit anyway. Writing the flag makes the intent survive a future reader
  who reaches a state where fast-forward *would* be possible.
- **Expect `redkiln doctor` to keep reporting exactly six `template-drift` advisories.** A seventh
  or a missing one after the merge is a finding worth naming in the health line, not something to
  silence (`../_decomposition.md`, `## Testing brief`, merge-gate step 2; CLAUDE.md).
- **Never run `redkiln adopt --templates`** (CLAUDE.md) — it would overwrite the six deliberate
  customisations, and post-merge is exactly when `redkiln upgrade`'s recommendation to run it is
  most tempting.

## Tests and CI (merge gate)

Tiers are `../_decomposition.md`, `## Testing brief`, "The test mix". This story compiles no Rust
and adds no Rust test (**AC-TB-08**); the Rust-grain steps below function as a regression net
proving `crates/`, `examples/`, `spec/` and `standards/` stayed untouched, not as tests of new
behaviour.

| tier | command / path | proves |
| --- | --- | --- |
| **1 — static** | `git cat-file -p HEAD` (exactly two `parent` lines); `git merge-base --is-ancestor <parent2> HEAD`; `git log --oneline HEAD^1` | **AC-001** — a two-parent merge commit exists, the sibling tip is an ancestor, and this branch's six pre-merge commits were not rewritten. Rejects the named wrong implementations: `rebase`, `squash`, `cherry-pick`. |
| **1 — static** | `git diff --name-only <parent2> HEAD -- crates spec xtask standards docs examples .kb` (empty); `git diff --name-only <parent2> HEAD -- references` (at most `references/seeds/user-documentation.md`) | **AC-003**, and with it IQ-3 / IQ-6. The compensating control for the `**` PR boundary — it checks content, not paths, so a hand edit anywhere in the merged tree fails even though the glob admits it. |
| **1 — static** | `git cat-file -e <sha>^{commit}` for every sha in `.bklg/docs-that-teach/durable-audience-closeout/_baseline.md`; string-compare against `git rev-parse HEAD HEAD^1 HEAD^2` | **AC-002** (the record's shas are real and are *these* shas) and half of **AC-006** (the recorded sha matches the item's). |
| **1 — static** | `rg "commits" .bklg/docs-that-teach/durable-audience-closeout/merge-forward-baseline/story.md`; `redkiln verify --grain story` for HS-S0172 | **AC-006** — `links.commits` is populated by the CLI, satisfying `require_commit_provenance: true` (`.redkiln/config.yaml:73`) and `require_ledger: true` (`:67`). |
| **1 — static** | `ls .kb/_intake/`; `ls .kb/decisions/`; `rg "^(stage\|status):"` over the counterpart item in the merged tree | **AC-005** — the three premise re-checks are recorded from live observation, each with the command that produced it. |
| **2 — content review** (human, one claim at a time) | `.bklg/docs-that-teach/durable-audience-closeout/_baseline.md` read **alone**, against the checklist IQ-1, IQ-2, IQ-3, IQ-8 in `../_decomposition.md`, `## UX brief` — no second checklist, per **AC-TB-06** | **AC-002**, **AC-004**, **AC-005**, **AC-007** — the tier the testing brief names as the only one that can fail a machine-correct but unusable record. The wrong implementation it rejects: a `_baseline.md` carrying only a sha, with no conflict sentence, no owner on any premise row and no health command. |
| **3 — integration / mount-point** | Manual walk: `.bklg/docs-that-teach/durable-audience-closeout/project.md` → `../_baseline.md` → the four downstream records that must cite the sha (`post-merge-clause-completeness`, `reconciliation-ledger`, `design-tension-audit`, `dod-scenario-ledger`) | **AC-002**'s reachability half (IQ-5 — two hops, link text naming the destination). Within this PR only the first hop exists; the downstream hops are checked when those stories land. |
| **3 — story grain (already wired)** | `cargo xtask affected --base main` — the `affected_gate` (`.redkiln/config.yaml:40`), which runs whether or not anyone types it | **AC-007** — the baseline health observation, recorded with its command and outcome. Green exonerates the merge from downstream failures; a failure routes to `support` (`:5`) and is **not** repaired here (EC-004). |
| **4 — e2e / whole tree** | `cargo xtask ci` — **deliberately not run in this story** | Nothing here, on purpose. The terminal gate is `terminal-gate-run`'s (AC-016), last, on the tree that already carries every artefact; a reading taken now "proves the gate, not the deliverable" (`../_decomposition.md`, ordering 4) and is inadmissible as AC-016 evidence per **AC-TB-03**. The row exists so its absence reads as a decision rather than an oversight. |

**Ledger.** Every row above resolves to an entry in
`.bklg/docs-that-teach/durable-audience-closeout/merge-forward-baseline/_ledger.md`, one per
AC-###, flipped only with a cited `file:line` or a captured command outcome (`require_ledger:
true`, `.redkiln/config.yaml:67`).

## Risks and coupling (PR-scoped)

| risk | why it bites here | control |
| --- | --- | --- |
| **The sibling branch is live and was being implemented while this spec was written.** Every figure in the Context pack — 223 commits, ~218k insertions, 1,089 files, tip `3f49ec6`, six intake documents — was measured on 2026-08-17 and may be stale at merge time. | Copying a spec figure into `../_baseline.md` instead of a measured one produces a record that is confidently wrong, which is worse than no record. | **D8** makes re-observation an explicit obligation; AC-002 and AC-005 require the commands' *output*, never the spec's numbers. |
| **The `**` PR boundary admits the entire tree.** `redkiln verify --grain story` cannot fail this story on a stray edit. | A hand edit inside a merge this size is invisible to review by volume alone. | **AC-003** replaces the path check with a byte-identity check against the sibling tip — strictly stronger. **NF-004** confines the precedent to this story. |
| **The merge is the one irreversible act in the project.** Once four records cite the sha, un-making it is expensive. | Rebasing or amending "to tidy up" afterwards silently breaks AC-012 for four stories, with nothing failing. | **NF-001** and the append-only rule in `## Data and migrations`; corrections are new commits plus a superseding statement naming both shas. |
| **Slice coupling with `post-merge-clause-completeness` (HS-S0173).** They are implemented in one context and the second must name the first's sha. | If the merge slips or is re-done, the clause-completeness statement cites a tree that no longer exists. | The record is the hand-off, not conversation (Implementation notes); **EC-007** forbids a second merge commit. |
| **Downstream coupling is total — ten stories are observed against this tree.** | Any defect in the merge is inherited by every record after it, and `dod-scenario-ledger` re-observes fifteen scenarios on it. | Ordering 1 (`../_decomposition.md` §5) is why this story is the project's foundation; **AC-007** exists precisely so a later failure can be attributed to, or exonerated from, the merge. |
| **Gate noise from sibling contract work.** The merged tree may fail `cargo xtask affected` for reasons entirely owned by the sibling. | The temptation to fix it here is strong, and it is the AC-A10 breach — which also breaks AC-003. | **EC-004** and **AC-007**: capture, route to `support` (`.redkiln/config.yaml:5`), do not repair. |
| **`.kb/_intake/` arrives non-empty and looks like work to do.** | Sibling-staged documents land where the planning corpus said only a README lived; narrowing or ingesting them here destroys `audience-ingest-wave`'s approval-gate evidence. | **EC-008**, **AC-005**, and the PR boundary's explicit exclusion. |

## Dependencies

**Blocks on:** *none.* `depends_on: []` — this is the project's single `foundation` story and step
1 of merge order (`../_storymap.md`, "Merge order"). Its only external precondition is that the
sibling branch ref resolves (EC-003), which is not a backlog dependency: the initiative decided
explicitly to run in parallel and merge forward rather than block on it
(`../../_decomposition.md:116-123`).

**Unlocks:**

| story slug | how it consumes this story |
| --- | --- |
| `post-merge-clause-completeness` | Slice-mate, same milestone. Re-runs `spec-trace` on the merged tree and names this sha in its set-difference statement (AC-013, AC-012). |
| `reconciliation-ledger` | Names this sha as the tree the adjudication was performed against (AC-002, AC-012), and adjudicates the counterpart this story only *observed* (D2, row 2). |
| `design-tension-audit` | Reads the post-merge corpus for the DT-1…DT-10 audit (AC-018). |
| `charter-open-question-disposition` | Transitively — depends on `reconciliation-ledger`, which depends on this. |
| `staged-audience-payload`, `audience-ingest-wave`, `product-layer-mounting` | The ingest wave runs on the merged tree, and `audience-ingest-wave` owns the `.kb/_intake/` documents this story recorded and left alone (AC-A02, AC-009). |
| `dod-scenario-ledger`, `scenario-two-fault-injection` | The clean checkout AC-A07 requires is created **off this branch tip** (`git worktree add` / `git clone`); the fifteen re-observations are taken on this tree (AC-014). |
| `terminal-gate-run` | `cargo xtask ci` runs last on the descendant of this commit that carries every artefact (AC-016). |

## Anchors (progressive disclosure)

Deferred depth, not optional depth. Each row says why the artefact is load-bearing, the moment to
open it, and the criterion it serves. Read at that moment; do not preload the corpus.

| anchor | why it is load-bearing | when to open | serves AC-### |
| --- | --- | --- | --- |
| `.bklg/docs-that-teach/_decomposition.md` | The initiative-level merge-forward decision (`:89-103`), the parallel-and-reconcile rationale (`:116-123`) and the terminal-project flag (`:306-308`). The only place the *reason* the merge is a story rather than a chore is written down. | Before running `git merge` — specifically if you are tempted to rebase, squash, or block on the sibling. | AC-001 |
| `.bklg/docs-that-teach/durable-audience-closeout/_decomposition.md` | `## Architecture brief` §1 is the seam table that makes `crates/`, `spec/`, `xtask/`, `standards/`, `docs/`, `.kb/` read-only and grants exactly one whole-tree act; §5 orderings 1 and 4 fix merge-before-everything and gate-last; AC-A06/A07/A09/A10 are the architecture-grain bars. | §1 and AC-A10 immediately before writing AC-003's diffs; §5 ordering 4 before being tempted to run the whole gate. | AC-003 |
| `.bklg/docs-that-teach/durable-audience-closeout/_decomposition.md` | `## UX brief`, "Interaction-quality invariants" — IQ-1…IQ-8 with their falsifiers, and the U1/U2/U3 reader table this spec's criteria are framed from. This *is* the Tier 2 checklist; AC-TB-06 forbids a second one. | Open it while writing `../_baseline.md`, and again at review — it is the review checklist, not background reading. | AC-002, AC-004 |
| `.bklg/docs-that-teach/durable-audience-closeout/_decomposition.md` | `## Testing brief` — the four-tier mix and what each tier can and cannot see, the AC→tier mapping including AC-012's row, and AC-TB-02/03/06/08 governing tooling, gate ordering, checklist reuse and the "no new Rust tests" claim. | Before writing any ledger `verifying_test`, and before offering any command as evidence. | AC-007 |
| `.bklg/docs-that-teach/durable-audience-closeout/_grounding.md` | The measured premises this story re-checks: "Reconciliation counterpart does not exist in this tree", the sixteen-decision count, the literal `_intake/` glob, and T2 — the clean-checkout tension that makes this worktree unusable for Tier 4. | At the moment you write AC-005's three premise rows; each must quote this file's claim as the "premise as written". | AC-005 |
| `.bklg/docs-that-teach/durable-audience-closeout/project.md` | AC-012's exact wording (`:201`), plus DR-2 (reconciliation works in either merge order), DR-11 and DR-12 — the decision records this story's sha is the precondition for. | When checking that AC-001/AC-002/AC-006 together discharge both halves of AC-012 rather than one. | AC-002, AC-006 |
| `.bklg/docs-that-teach/durable-audience-closeout/_storymap.md` | "Merge order" step 1, the Coverage table's AC-012 row (which of three stories owns which half), and "Why these four and not eleven" — the reason the slice-mate must name this sha. | Before starting the slice, and again before handing off to `post-merge-clause-completeness`. | AC-002 |
| `.bklg/docs-that-teach/durable-audience-closeout/_design.md` | The signed-off design, `hasSurface: false`, approved 2026-08-17. It is the authority for the empty COMPOSITION family in `## Interaction quality` — the absence of a presentation AC is a human decision, not an omission. | Only if you are about to add a presentation, layout or density criterion. Read it, then do not. | AC-002 |
| `.bklg/docs-that-teach/initiative.md` | `## Definition of Done` — DoD-11 (the specification cross-reference over the tree *as it stands after* every doc comment) and DoD-15, plus the fifteen scenarios `dod-scenario-ledger` re-observes on this tree. Explains why the merge is the precondition for the whole DoD. | When writing the "advances DoD scenario" line in the baseline record, or when someone asks why a merge deserves a story. | AC-001 |
| `.redkiln/config.yaml` | The four lines that actually gate this story: `affected_gate` (`:40`), `support_initiative` (`:5`), `require_ledger` (`:67`), `require_commit_provenance` (`:73`) — plus the commented rationale that an undeclared command is a *trivial pass*. | Before running the health command and before writing the ledger. | AC-006, AC-007 |
| `.kb/governance/rewrite-the-referent-never-the-reasoning.md` | The corpus's correction discipline — supersede, never edit — applied here to the one artefact of this story that is not text, the commit graph. It is what makes NF-001 a rule rather than a preference, and it is U3's grounding in the UX brief's reader table. | If the merge is later found wrong, or if anyone proposes amending or rebasing after the sha is recorded. | AC-001 |
| `.kb/_intake/README.md` | The literal ingest glob (`:5-7`), "a successful ingest clears this directory" (`:14-19`), and the fact that `redkiln validate --kb` cannot see this directory (`:21-27`). Explains why documents the merge delivers must be left exactly as delivered. | The moment `ls .kb/_intake/` returns more than the README and it looks like work to do. | AC-005 |
| `CLAUDE.md` | "Where the work lives" — the CLI is the single writer of item system frontmatter (the `PreToolUse` hook), the six permanent `template-drift` advisories, and the standing prohibition on `redkiln adopt --templates`. "Commands" resolves `cargo xtask affected`. | Before running `redkiln record-links`, and before reacting to anything `redkiln doctor` reports post-merge. | AC-006, AC-007 |
| `xtask/src/main.rs` | The single definition of the gate and of `affected` — the proof that `cargo xtask affected --base main` is a real command in this tree rather than invented tooling (**AC-TB-02**), and the reference for what `affected` actually selects. | If the health command errors, or if you need to know what "affected" resolved to before recording its outcome. | AC-007 |

## Clarifications resolved during spec

1. **The AC set is exactly the seven the first pass enumerated** — AC-001 … AC-007. None added,
   none dropped, none renumbered. `_ledger.md` matches this set row for row.
2. **The composition family of `## Interaction quality` is empty by design authority, not by
   omission.** `../_design.md` records `hasSurface: false` with a human sign-off on 2026-08-17, so
   presentation, density, transience, hierarchy and named visual anti-patterns have no referent
   here. Rather than skip the section, the UX brief's text analogues (IQ-1…IQ-8) are mapped onto the
   AC rows that carry them, and none is left as a prose bullet — a bullet gets no ledger row and is
   therefore never gated.
3. **"Test" here means tier, not `#[test]`.** This story compiles nothing, and **AC-TB-08** forbids
   implying new Rust coverage exists or is required. Every `verification` cell names a real command,
   a real file read, or the Tier 2 human checklist the testing brief already owns — never a
   fictional test path. That is the same reasoning CLAUDE.md applies to decorative conformance
   rules, from the documentation side.
4. **`cargo xtask ci` appears in the merge-gate table as a row that is explicitly not run.** Leaving
   it out would read as an oversight; **AC-TB-03** makes an early reading inadmissible as AC-016
   evidence, so the row records the decision instead.
5. **The health observation is deliberately the story grain, not the project grain.** The testing
   brief notes that `affected_gate` "is the *story*-grain check any story this project's story map
   produces still runs", and that this project is exempted from `integration_scoped` in favour of the
   whole gate at the end. AC-007 therefore claims `cargo xtask affected --base main` and nothing
   stronger.
6. **`--base main` rather than a merge-base ref.** `.redkiln/config.yaml:40` substitutes the diff
   base, and this story's PR base is `main`. On a tree that has just absorbed the sibling's commits
   this selects a wide package set — which is the point: a narrow selection would prove nothing
   about the merge.
7. **EC-007 (re-run idempotency) was added during this pass**, because the merge is the one act in
   the project that is destructive to repeat. It creates no new AC: AC-001's "one two-parent commit"
   already fails a tree carrying two candidate merge shas.
8. **No ADR is cited as governing authority, and that is a finding rather than a gap in the
   research.** D9 records it once; the anchors table carries `.kb/governance/…`, `.redkiln/config.yaml`
   and CLAUDE.md instead. Citing a Rust-contract decision atom for a branch-policy question would
   manufacture authority the corpus does not have.
9. **AC-003's `references/` carve-out is deliberate and narrow.** This branch's six pre-merge commits
   touched `references/seeds/user-documentation.md`, so a blanket "`references/` is byte-identical"
   assertion would fail for a legitimate reason. The criterion names the single permitted path rather
   than dropping the directory from the check.
