---
item: HS-S0125
stage: spec
created: 2026-08-12T13:48:04.895Z
updated: 2026-08-12T13:48:04.895Z
template_sig: 87bbf1d0
rendered_sig: 5bcccca3
---

# Spec — The clean-checkout seam and the closeout record it feeds

## Scope lock

| What | Path |
| ---- | ---- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` — *Definition of Done* preamble (`:356`, "from a clean checkout") and DoD 13 (`:396`) |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` — traceability matrix, DAG, *Decisions taken at the gate* |
| Project | `.bklg/from-contract-to-published-library/closeout-and-durable-audience/project.md` — AC-001 (`:192`), DR-1 (`:129`), DR-2 (`:132`) |
| This spec | `.bklg/from-contract-to-published-library/closeout-and-durable-audience/clean-checkout-harness/spec.md` |
| Key briefs | `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_decomposition.md` (the one warranted `testing` brief — *Fixtures / seams* at `:116`, *Merge-gate commands*); `_grounding.md` (the gate's real step list, the six-file drift assertion, the corrected clause-figure range); `_design.md` (**no public API surface**, approved 2026-08-12) |
| Story map row | `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_storymap.md:53` (this row), `:24-30` (why the evidence converges on one artefact), `:140-143` (merge order — this story is the DAG root) |
| This story's discovery | `.bklg/from-contract-to-published-library/closeout-and-durable-audience/clean-checkout-harness/discover.md` — the two questions deferred here, and the wrong implementation |
| Roadmap pointer | `RUNBOOK.md` — the plan of record; this story adds nothing to it and amends nothing in it |

## One-line PR slice

Construct the clean-checkout seam the gate must run in — a fresh checkout with no untracked or
ignored residue and no path dependency standing in for a published version — and stand up
`_closeout-record.md` as the artefact every later story appends cited evidence to.

## Executive summary

This PR lands **two things and no code**: a pinned, residue-free checkout of the assembled library
with its provenance captured as files, and the empty-but-shaped `_closeout-record.md` that the rest
of this project writes into.

Delta against what the project already says. `project.md` DR-1 (`:129-131`) states the requirement
— "no untracked or ignored residue and no path dependency standing in for a published version. A
warm working tree does not satisfy this" — and the `testing` brief (`_decomposition.md:116-120`)
names two acceptable constructions, "a fresh `git worktree add` or clone", without choosing. This
spec **chooses the clone, and says what the worktree would have hidden** (see *Context pack*,
decision 1). It also settles the second thing nobody has settled: `_storymap.md:24-30` names
`_closeout-record.md` as the convergence artefact but does not say what is in it, and an artefact
whose shape is invented by whichever story reaches it first is fourteen ledger entries in fourteen
places wearing one filename.

Two consequences that are easy to miss and are decided here rather than discovered later. The
checkout is **pinned to a commit SHA**, so it is evidence about one tree and goes stale the moment a
later slice commits — any story needing a later tree re-runs this harness at its own SHA and adds a
row, rather than quietly citing the first one (*Behavior*, B-7). And the `skipped:` accounting
AC-001 demands exists **only on the gate's stdout** (`xtask/src/main.rs:876`), so capturing the
transcript is a harness obligation, not something the slice-mate can reconstruct afterwards
(*Behavior*, B-5).

## Context pack

Everything below is a decision this story must honour. It is complete enough to start from; the
deeper artefacts sit behind the anchors and are opened only when a row here points at them.

**1. The isolation mechanism is a fresh `git clone` from `origin` at an explicit SHA, checked out
detached — not a `git worktree add`.** Both satisfy the letter of `_decomposition.md:116-120`. The
clone is chosen because a worktree shares the primary repository's *common* `.git` directory: a
`.git/info/exclude` line, a local `core.excludesFile`, or a hook can silence `git status` in the
worktree while no committed file records that it happened, and the residue check is the entire point
of this story. A clone gets a fresh `.git` — no `info/exclude`, no local config, no hooks — and, by
fetching from `origin`, additionally proves the closeout commit is reachable from a pushed ref
rather than existing only on this machine, which is what "the assembled tree" has to mean to anyone
auditing this later. The cost is real and is accepted deliberately: a full network fetch and a cold
`target/`. The cold build is not overhead, it is the half of DR-1 that says "stale build artefacts",
and a shared build cache would defeat it.

**2. The gate this seam hosts is `cargo xtask ci`, whole.** `.redkiln/config.yaml:60` reserves
`verify.e2e: cargo xtask ci` for the terminal project; `:55`'s `cargo xtask ci --fast` is the bar
every sibling already met and re-running it here would prove nothing this project exists to prove
(DR-2). This story does not run the gate — its slice-mate does — but the harness is built to that
shape: twenty `REQUIRED` steps and four `OPTIONAL` ones (`xtask/src/main.rs:105`, `:535`), the
`OPTIONAL` four each behind a probe (`cargo hack --version` twice, `cargo deny --version`,
`cargo +nightly --version` at `:556`, `:593`, `:600`, `:635`).

**3. Skips are a property of the host, not of the checkout — so the host is captured too.** When a
probe fails, `run_steps` prints a `skipped:` line naming the probe command that did not succeed, and
continues to the next step (`xtask/src/main.rs:873-878`). Nothing else records it. AC-001 requires
every `skipped` step to be listed "with the absent tool that caused it", which is only answerable if
(a) the run's stdout is
captured to a file and (b) the probe results were recorded *before* the run, so a `skipped` line is
attributable rather than merely observed. Both are this story's, because the slice-mate cannot
reconstruct either after the fact.

**4. "No path dependency standing in for a published version" has a precise reading here, and it is
not the workspace's own path deps.** `Cargo.toml:24-26` declares `happenstance-core`,
`happenstance` and `happenstance-testkit` with both `version` and `path` — that is ordinary
workspace practice and is not what DR-1 forbids. What DR-1 forbids is a `[patch]` section, a
`paths` override, a `source.replace-with` in a cargo config, or a vendored directory silently
supplying what the registry is supposed to. A fresh clone rules out the *ambient* forms (nothing
outside the repository travels), and `--locked` — which every dependency-resolving step in the gate
passes (`xtask/src/main.rs:52-54`) — rules out a resolution that differs from the committed
`Cargo.lock`. The check this story owns is therefore: the in-tree forms are absent, and `Cargo.lock`
is unchanged after the run.

**5. This project owns no code, and that constrains where the harness may live.** `project.md`
*Out of scope* is explicit, and the `testing` brief's AC-013 row makes it observable: "zero commits
inside this project's own stories touch code outside `.bklg/`/`.kb/` planning artefacts." So the
harness is **a recorded procedure plus captured evidence files**, not a script committed under
`xtask/` or `scripts/`. A helper script would be the fastest thing to write and would fail this
project's own acceptance criterion.

**6. The checkout is read-only evidence.** No story commits into it, and no story's `.kb/` or
`.bklg/` writes happen there — those belong to the working tree on the initiative branch. Two
reasons: writing into it destroys the residue property the clone was made to demonstrate, and a
commit made in a detached clone is reachable from nothing and would be lost. The one thing the gate
is *allowed* to create there is `target/`, which `.gitignore` already ignores; that is why the
residue assertion is taken **before** the run and a narrower tracked-file assertion is taken after.

**7. `_closeout-record.md` is one artefact with a fixed shape, stood up empty here.**
`_storymap.md:24-30` makes the convergence the point of AC-003's word "set". This story creates it
with its sections, its column contracts and a coverage line, and writes exactly one row of real
content — its own. Every later story appends; none re-shapes it.

**8. The record's citations must verify the referent, not merely the address, and must report their
own coverage.** This is not invented here: it is the accepted playbook
`.kb/playbooks/verify-the-referent-and-report-coverage.md` ("that a `file:line` resolves says
nothing about whether the attributed content is there... a check that does not state what fraction
of the corpus it parsed is indistinguishable from one that sees all of it"), grounded in a parser
that checked 84 of 338 citations while printing "no problems found". Its consequence for the record
is concrete: every evidence cell names a path **and** the short subject string a reader should find
there, and the record carries a coverage line stating how many of its rows are filled against how
many are owed. The companion playbook
`.kb/playbooks/anchoring-citations-in-a-long-lived-document.md` is why the record prefers a path plus
a subject string over a bare `file:line` — the targets move.

**9. The persona slice this realises.** The reader served here is the one `_storymap.md` names for
the whole project: someone who must be able to believe the initiative closed honestly *without
re-deriving the evidence*. For this story specifically that reader asks two questions — "where did
this run, and how do you know nothing was left in the tree?" and "where do I read all of it
together?" — and this story is the only place either is answerable. The evaluator/adapter/author
personas being promoted in slice 4 are not this story's; it authors no `.kb/product/` atom
(`discover.md` *Questions*, third bullet).

**10. Nothing here designs a surface.** `_design.md` is approved with **no public API surface** —
"N/A" against every item block, sign-off recorded 2026-08-12. This story adds, changes and removes
zero public items, so the surface invariants that normally bind a story in this repository are
vacuous here and must not be ticked as though they were met.

## Integration contract

- **Archetype**: `foundation` — real in-tree substrate (a pinned checkout with captured provenance,
  and the record artefact) consumed inside this same slice by
  `whole-gate-green-on-the-assembled-tree`. Never a double: `_decomposition.md:120` is explicit that
  "nothing is mocked. A project whose entire deliverable is re-observation would falsify its own
  purpose by substituting a double for anything it claims to have re-run."
- **Slice / milestone**: `assembled-tree-gate`. Slice-mate: `whole-gate-green-on-the-assembled-tree`
  (the run inside this checkout). The two are implemented in one context and land as one integrated
  surface — `_storymap.md` *Why the slices are cut here*: splitting "make a clean checkout" from
  "run the gate in it" would be exactly the build-it/wire-it-in split the slicing rule forbids.
- **Mount point**:
  `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md`. For a
  project whose only deliverable is verification and recording, this file *is* the composition root
  and the render path: it is the one place a reader meets the project's output, and
  `_storymap.md:24-30` names it as where the DoD set, the delta, the audit tables and the findings
  converge. This story creates it, mounts its own harness row in it, and the slice-mate mounts the
  gate run in it in the same slice — so the seam is demonstrated through the mount point, not beside
  it.
- **Wires into** (real siblings and contracts consumed, by path):
  - `xtask/src/main.rs` — the gate, defined once: the `REQUIRED`/`OPTIONAL` split (`:105`, `:535`),
    the four probes (`:556`, `:593`, `:600`, `:635`), the `skipped:` print (`:876`), the four
    `wasm32` steps (`:203`, `:231`, `:252`, `:271`), the `--locked` invariant (`:52-54`).
  - `.redkiln/config.yaml:60` (`verify.e2e`) and `:55` (`integration_scoped`) — which grain this
    checkout exists to host, and which one it must not substitute.
  - `.redkiln/config.yaml:67` / `:73` — `require_ledger` and `require_commit_provenance`: this
    story's own `_ledger.md` must cite the isolation command and the residue output as artefacts.
  - `.gitignore` — the operative definition of "ignored residue" (`target/`, `.claude/`,
    `.redkiln-cli/`, `mutants.out*`, `.mcp.json`, `.idea/`).
  - `rust-toolchain.toml` (pins 1.97.1 and the `wasm32-unknown-unknown` target),
    `.cargo/config.toml` (the `xtask` alias, without which `cargo xtask ci` does not exist) and
    `Cargo.lock` — all tracked, so all three travel into the clone; a clean checkout that lacked any
    of them would fail for the wrong reason.
  - `.kb/playbooks/verify-the-referent-and-report-coverage.md` and
    `.kb/playbooks/anchoring-citations-in-a-long-lived-document.md` — the citation contract the
    record's columns encode.
- **Public items**: none. `_design.md` records no public surface for this project and this story
  implements no `## Items` entry, because the block is `N/A` by an approved determination rather
  than by omission.
- **Conformance rule(s)**: none, and this is **not adapter-observable**. The story adds no rule to
  `crates/happenstance-testkit/src/suite.rs` and changes no port, so there is no adapter that could
  fail differently because of it. Naming a rule here would be decorative in exactly the sense
  `CLAUDE.md` forbids — a rule no adapter can fail.
- **Clause(s)**: none discharged, none amended. No `[FROZEN]` clause of `spec/SPECIFICATION.md` is
  touched, so no new ADR is owed. The gate step that would notice otherwise (`cargo xtask
  spec-trace`) runs inside the slice-mate's run, not here.
- **Advances DoD scenario**: **DoD 13** ("The gate is green on the assembled whole",
  `initiative.md:396`) — this story supplies the tree the claim is about. It also makes the
  *preamble* to all sixteen scenarios (`initiative.md:356`, "run and observed to pass on the
  assembled library, from a clean checkout") true rather than aspirational, which is why a failure
  here is a finding about the whole DoD list and not about one scenario.

## PR boundary

`redkiln verify --grain story` reads the first fenced block under this heading and fails on any file
changed outside it. The set below is narrow on purpose: this project owns no code
(`project.md` *Out of scope*), and the `testing` brief's AC-013 row makes "zero commits... touch code
outside `.bklg/`/`.kb/` planning artefacts" an observable criterion. A harness script under `xtask/`
or `scripts/` would be the natural thing to reach for and would breach this boundary — the harness is
a recorded procedure plus captured evidence, per *Context pack* decision 5.

```
.bklg/from-contract-to-published-library/closeout-and-durable-audience/clean-checkout-harness/**
.bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md
```

**In this PR**

- The isolation procedure, run for real: a fresh clone from `origin`, detached at the closeout SHA,
  in a directory outside every existing checkout of this repository.
- The captured evidence, committed under the story folder: the isolation transcript, the
  `git status --porcelain --ignored` output (empty), the resolved SHA and commit date, the
  in-tree override scan, and the host tool-probe manifest.
- `_closeout-record.md`, created with its sections, column contracts and coverage line, carrying its
  own harness row and empty placeholders for every row the later slices owe.
- This story's `_ledger.md` (second pass), citing those artefacts per `.redkiln/config.yaml:67`.

**Explicitly not in this PR**

- Running `cargo xtask ci` and recording its exit code, SHA and `skipped` accounting — that is
  `whole-gate-green-on-the-assembled-tree`, the other half of project AC-001, and all of AC-002.
- Any `.kb/` atom, staged or promoted — slice `durable-audience`.
- Any `redkiln validate` / `doctor` health assertion — `backlog-and-kb-health-at-closeout`.
- The DoD 13 delta, the published `0.2.0` tag, or any statement about the published surface —
  `published-tree-delta-statement`. No tag exists in this tree yet, and creating one is HS-P0016's.
- Fixing anything the isolation turns up. A residue class nobody expected is a finding, routed by
  `findings-disposition-register` (AC-013); repairing it here would fail the project's own criterion.
- Any change to `xtask/`, any crate, `spec/SPECIFICATION.md`, `.github/workflows/ci.yml` or
  `.redkiln/templates/`.

**Merge DoD one-liner.** A reader who has never seen this machine can name the exact commit the
closeout gate was run against, see for themselves that the tree it ran in held nothing but that
commit, and open one file that tells them where every remaining piece of closeout evidence will be.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **B-1 · The checkout is a fresh clone from `origin`, detached at a named SHA** | `git clone <origin-url> <dir>` into a directory that is not inside `D:/repos/happenstance` nor any of its worktrees, then `git -C <dir> checkout --detach <SHA>` where `<SHA>` is the closeout commit on `initiative/from-contract-to-published-library`. Detached HEAD so the measured tree is unambiguous; cloned from `origin` so the SHA is proven reachable from a pushed ref. `git worktree add` is rejected: it shares the common `.git`, so `info/exclude`, hooks and local config travel with it and can silence the residue check (*Context pack* 1). | `git remote -v` resolves `origin` → `https://github.com/Wet-Ink-Corporation/happenstance.git`; procedure and its transcript captured under the story folder |
| **B-2 · The residue assertion is taken before anything is built** | Inside the clone: `git status --porcelain --ignored` produces **empty** output, and `git rev-parse HEAD` matches `<SHA>`. `--ignored` is load-bearing — the plain form would pass with a `target/` present, and `target/` is the stale-artefact half of DR-1. Captured as a file, not read from scrollback: an empty output is only evidence if the command that produced it is on record. | `project.md:129-131` (DR-1); `.gitignore` defines what `--ignored` covers |
| **B-3 · No in-tree override stands in for a published version** | Scan the clone for the forms DR-1 forbids: a `[patch...]` section in any `Cargo.toml`, a `paths = [...]` or `[source.*] replace-with` in any cargo config, or a vendored source directory. Zero hits expected today (`grep -rn "^\[patch" --include=Cargo.toml` is empty in this tree). The workspace's own `version`+`path` dependencies at `Cargo.toml:24-26` are **not** hits and are recorded as such, so the next reader does not mistake ordinary workspace practice for the thing DR-1 rules out. | `Cargo.toml:24-26`; `project.md:129-131` |
| **B-4 · The lockfile is the resolution, before and after** | `Cargo.lock` is tracked and travels with the clone; every dependency-resolving step in the gate passes `--locked` (`xtask/src/main.rs:52-54`), so a resolution differing from the committed lock is a hard failure rather than a silent update. The harness records the lock's blob hash before the run so the post-run comparison in B-6 is against a recorded value. | `xtask/src/main.rs:52-54`; `git ls-files Cargo.lock` |
| **B-5 · The run is captured where `skipped` can still be read** | The gate prints a `skipped:` line naming the failed probe to stdout, and records it nowhere else (`xtask/src/main.rs:873-878`). The harness therefore fixes the transcript path the slice-mate must tee into, under this story's folder, and — before the run — captures a **probe manifest**: the exit status of `cargo hack --version`, `cargo deny --version` and `cargo +nightly --version` with `RUSTUP_AUTO_INSTALL=0`, the same spelling `is_available` uses (`xtask/src/main.rs:894-908`). A `skipped` line is then attributable to a named absent tool rather than merely observed. `CLAUDE.md` records that `cargo-hack` and `cargo-deny` both resolve on this machine, so a skip of either is itself a finding. | `xtask/src/main.rs:556`, `:593`, `:600`, `:635`, `:873-908`; `project.md:192-195` (AC-001's skip clause) |
| **B-6 · The checkout is read-only evidence** | Nothing is committed into the clone and no `.kb/` or `.bklg/` write happens there; those belong to the working tree on the initiative branch. After the slice-mate's run, `git status --porcelain` (tracked files, without `--ignored`) is **empty** — `target/` is expected and ignored, a modified tracked file is not. This is a live check, not a formality: `cargo xtask spec-trace` can rewrite §7.1–§7.2 of `spec/SPECIFICATION.md`, and the gate step deliberately invokes it **without** `--write` (`xtask/src/main.rs:315-327`). A dirty tracked file after the gate means a step mutated the tree, which is a finding to route. | `xtask/src/main.rs:315-327`; `xtask/src/spec_trace.rs` (`--write` is opt-in) |
| **B-7 · The harness pins one SHA, and says so** | The clone is evidence about one commit. Later slices commit into the working tree (`.kb/product/` atoms, audit tables, the findings register), so by slice 5 the pinned tree is stale by construction. Any story needing a later tree **re-runs this procedure at its own SHA and adds a row** to the record's harness table; citing the first row for a later observation is the failure this contract exists to make impossible. The record therefore carries the observed SHA per row, not once per document. | `_storymap.md` *Merge order* (slices 4–5 write to `.kb/`); `.redkiln/config.yaml:73` (`require_commit_provenance`) |
| **B-8 · `_closeout-record.md` is created with a fixed shape** | Sections, in order: *Harness* (one row per pinned checkout: SHA, commit date, isolation command, residue-output path, probe-manifest path); *The whole gate* (AC-001/AC-002 — the slice-mate's); *DoD 1–12, 14–15* (fourteen rows, AC-003); *The DoD 13 delta* (AC-004); *Decision-atom audit* (AC-005) and *Open-question preservation* (AC-006); *Backlog and KB health* (AC-011/AC-012); *Findings* (AC-013 — with a destination column, never a fix column); *Closeout readiness* (AC-014). Every section is present and empty-with-placeholders on creation, so a missing row is visibly owed rather than invisibly absent. | `_storymap.md:24-30`; `project.md` AC-003…AC-014 |
| **B-9 · Every row's evidence cell names a referent, not just an address** | Column contract for every evidence cell: the artefact path, the short subject string a reader should find at it, and the SHA it was observed on. Straight from the accepted playbook — an address that resolves says nothing about whether the attributed content is there — and from its companion on why a bare `file:line` rots. The record also carries a **coverage line**: rows filled / rows owed, so a half-populated record cannot read as a complete one. | `.kb/playbooks/verify-the-referent-and-report-coverage.md`; `.kb/playbooks/anchoring-citations-in-a-long-lived-document.md` |
| **B-10 · A sibling's `_ledger.md` may be the pointer, never the evidence** | Stated once, in the record's own preamble, because it is the rule every later row is written against (DR-3; `_decomposition.md` AC-003 row). The record's shape enforces it structurally: the evidence cell asks for an artefact observed on *this* tree with its SHA, and a `_ledger.md` path from a sibling project carries a different SHA. | `project.md` DR-3; `.redkiln/config.yaml:67` |
| **B-11 · No public interface changes** | This story exposes no Rust item, no CLI flag and no config key. `_design.md` records **no public API surface** for the project, approved 2026-08-12; the surface invariants that normally bind a story here are vacuous and are not ticked. The only "interface" produced is the record's column contract in B-9, which binds later stories rather than callers. | `_design.md` (*Items*, *Signatures*, *Sign-off*) |

## Data and migrations

**N/A — no schema, no store, no migration.** This story writes no persistent data structure that any
program reads. Two near-misses, both deliberately not migrations:

- **`_closeout-record.md`** is markdown read by humans and by `redkiln verify`'s ledger/AC extraction
  only through the *stories'* `_ledger.md` files, not directly. Its "schema" is the column contract
  in B-9, versioned by nothing and changed by editing this spec, not by a migration step. It is
  created new in this PR, so there is no prior shape to migrate from.
- **The clone** holds no state of its own beyond the checkout and a build `target/`. It is discarded
  after the closeout observations are captured; the durable residue is the evidence files committed
  under this story's folder, which is why B-2 and B-5 insist those outputs are captured to files
  rather than observed in a terminal.

No `.kb/` atom is authored or amended here (that is DR-8's path, in slice `durable-audience`), so no
`KbFrontmatter` shape is introduced or changed.

## Acceptance criteria

The reader every criterion is written for is the one `_storymap.md:18-22` names for the whole
project: **someone who must be able to believe the initiative closed honestly without re-deriving
the evidence** — and who, per the initiative's own *Decide in one sitting* journey
(`initiative.md:249-250`) and the persona behind it (`../_discovery/distillation/personas-and-journeys.md:249-262`,
*Persona 4 — the evaluator, pre-adoption*: "decide, in a bounded amount of research time"), gets one
bounded pass over public artefacts and cannot re-run anything themselves. The same reader is what
the slice-mate's criteria are written for, deliberately: the seam and the run are one surface.
Each criterion below is that reader crossing the whole stack: a question they arrive
with, the artefact they open, and what they must find there. A criterion satisfied by "the command
was run" and not by "the reader can see it was run, where, and on what" has not been met.

Evidence file paths below are fixed by this spec so that later stories cite a stable address; they
all sit under
`.bklg/from-contract-to-published-library/closeout-and-durable-audience/clean-checkout-harness/_evidence/`,
abbreviated `_evidence/` in the table.

| id | criterion | verification |
| --- | --- | --- |
| **AC-001** | **The reader can name the exact tree the closeout gate ran on, and reproduce it.** GIVEN an auditor who has never had access to this machine and cannot re-run the suite, WHEN they open the *Harness* row of `_closeout-record.md`, THEN they find the full isolation command — `git clone https://github.com/Wet-Ink-Corporation/happenstance.git <dir>` into a directory outside `D:/repos/happenstance` and every worktree of it, followed by `git -C <dir> checkout --detach <SHA>` — with the resolved SHA and its commit date, AND running those two commands themselves yields a tree whose `git rev-parse HEAD` equals the recorded SHA. A `git worktree add` in place of the clone fails this criterion even though it produces a checkout, because the tree it produces shares the primary `.git` (B-1). | `_evidence/clone-transcript.txt` (the clone + detach transcript, captured whole) and `_evidence/head-sha.txt` (`git rev-parse HEAD`, `git log -1 --format=%cI`, and `git branch -r --contains <SHA>` proving the SHA is reachable from a pushed ref). Reviewed against the *Harness* row of `_closeout-record.md`, which must repeat the same SHA. |
| **AC-002** | **The reader can see for themselves that nothing was left in the tree.** GIVEN the auditor asking "how do you know a scratch file or a stale `target/` was not part of what you measured", WHEN they open the residue evidence, THEN they find `git status --porcelain --ignored` producing **empty** output in the clone, captured to a file, and taken **before** anything was built — ordered ahead of the gate transcript, not reconstructed after it. The `--ignored` flag is part of the criterion: the plain form passes with a populated `target/`, which is the stale-artefact half of DR-1 (`project.md:129-131`). | `_evidence/residue-before.txt` — zero bytes, with the command line recorded in `_evidence/clone-transcript.txt` immediately after the detach and before any `cargo` invocation. An empty file with no recorded command does not satisfy this: an empty output is evidence only when the command that produced it is on record (B-2). |
| **AC-003** | **The reader can believe the gate measured the real dependency graph.** GIVEN the auditor asking "was a local copy standing in for a published crate", WHEN they open the override scan, THEN they find zero hits for every form DR-1 forbids — a `[patch…]` section in any `Cargo.toml`, a `paths = [...]` or `[source.*] replace-with` in any cargo config, a vendored source directory — AND they find the workspace's own `version`+`path` dependencies at `Cargo.toml:24-26` listed explicitly as **non-hits with the reason**, so the next reader does not mistake ordinary workspace practice for the thing DR-1 rules out. `Cargo.lock`'s blob hash is recorded before the run so the post-run comparison has a recorded value to compare against. | `_evidence/override-scan.txt` (the four scans and their zero-hit results, plus the recorded non-hit explanation) and `_evidence/lock-hash.txt` (`git hash-object Cargo.lock` in the clone). Cross-checked against `xtask/src/main.rs:52-54`, which is why a differing resolution is a hard failure rather than a silent update (B-3, B-4). |
| **AC-004** | **The reader can tell what the gate did *not* do, and why.** GIVEN the auditor asking "twenty-four steps, four of them optional — which ran", WHEN they read the harness row, THEN they find a probe manifest captured **before** the run recording the exit status of `cargo hack --version`, `cargo deny --version` and `cargo +nightly --version` under `RUSTUP_AUTO_INSTALL=0` — the same spelling `is_available` uses (`xtask/src/main.rs:894-908`) — AND the transcript path the run must tee into, fixed here, so every `skipped:` line the gate prints to stdout and records nowhere else (`xtask/src/main.rs:873-878`) survives the session. A skip is then attributable to a named absent tool rather than merely observed, which is what AC-001 of `project.md:192-195` asks for. | `_evidence/probe-manifest.txt` (three probes, exit status each, with the host's OS and toolchain from `rustc -Vv`), and `_evidence/gate-transcript.txt` declared as the tee target in the *Harness* row before the slice-mate runs. `CLAUDE.md` records that `cargo-hack` and `cargo-deny` both resolve on this machine, so a non-zero probe for either is itself a finding (EC-4). |
| **AC-005** | **The reader has one place to read all of it together.** GIVEN the auditor who must not be sent to fourteen ledger entries in fourteen places — the failure mode `_storymap.md:24-30` writes AC-003's word "set" against — WHEN they open `_closeout-record.md`, THEN it exists with all nine sections in the fixed order of B-8, every owed row present as a **visible placeholder naming the AC and the story that owes it**, a stated column contract requiring artefact path *plus* the subject string a reader should find there *plus* the SHA it was observed on, a preamble stating that a sibling's `_ledger.md` may be the pointer and never the evidence, and a **coverage line** giving rows filled against rows owed. | Structural review of `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md` against B-8/B-9/B-10: nine sections present in order; the *DoD 1–12, 14–15* section carries exactly fourteen rows; the *Findings* section carries a destination column and **no** fix column; the coverage line is present and reads `1 / N` with this story's own harness row the only one filled. Contract grounded in `.kb/playbooks/verify-the-referent-and-report-coverage.md`. |
| **AC-006** | **The reader can trust that no later claim was smuggled in on this observation.** GIVEN the auditor reading a row about a tree that moved after this harness ran — slices 4 and 5 commit into `.kb/` by construction — WHEN they check its provenance, THEN every row carries the SHA it was observed on rather than the document carrying one SHA, the record's preamble states the rule that a story needing a later tree **re-runs this procedure at its own SHA and adds a harness row** rather than citing this one, and the checkout's read-only property is evidenced by a post-run tracked-file check whose non-empty result is declared a finding to route rather than something to repair. | `_evidence/residue-after.txt` — `git status --porcelain` (tracked files, deliberately **without** `--ignored`, because `target/` is expected) captured in the same slice after the gate run, empty. Plus the *Harness* table's per-row SHA column and the preamble rule, reviewed in `_closeout-record.md`. `cargo xtask spec-trace` can rewrite `spec/SPECIFICATION.md` and the gate deliberately invokes it without `--write` (`xtask/src/main.rs:315-327`), so a dirty tracked file here is a real signal, not a formality. |

Coverage of the traced project AC: **`project.md` AC-001** ("the whole gate is green on the
assembled tree from a clean checkout") is split along the seam `_storymap.md:116` names. This story
owns the *checkout* half — AC-001 through AC-004 and AC-006 together are what makes "clean checkout"
a checked claim rather than an adjective — and AC-005 stands up the artefact the run's half is
recorded in. The green run itself, its exit code and its SHA are
`whole-gate-green-on-the-assembled-tree`'s, in this same slice.

## Interaction quality

This story renders **no screen and no public API**. `_design.md` is approved with *no public API
surface* — "N/A" against every item block, sign-off recorded 2026-08-12 — so the project's
signed-off design contributes **no composition invariants to inherit**, and inventing some here
would be fabricating a design a human never approved. What this story *does* render is a document a
reader meets: `_closeout-record.md` is the mount point, and the invariants below are the ones that
make it legible rather than merely present. Every one of them is already carried by an AC row in
the table above; none is a prose-only bullet, because `redkiln verify` extracts criteria from table
cells and a bullet here would never be gated.

**State family.**

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **In-place, not context-jump** — later stories *append* to the one record; no story creates a second record file or re-shapes a section it did not create | AC-005, AC-006 | The nine sections and their order are fixed at creation and reviewed against B-8; a second `*-record.md` under the project directory is a breach of the PR boundary of whichever story wrote it |
| **Non-occlusion** — an owed row is a visible placeholder naming its AC and owner, never an absent line, and the coverage line prevents a half-populated record reading as complete | AC-005 | Structural review: rows owed = 14 DoD rows + the sections' declared rows; coverage line present and arithmetic-checkable |
| **Stable anchors** (the document analogue of preserved focus/scroll/selection) — a citation written against a section survives every later append, because headings and row order are fixed at creation and rows are added at the end of their section | AC-005 | Reviewed against `.kb/playbooks/anchoring-citations-in-a-long-lived-document.md`, which is why the column contract asks for a path plus a subject string rather than a bare `file:line` |
| **Reversibility** — nothing is committed into the clone, the clone is discardable, and every observation is re-creatable by re-running the recorded command at the recorded SHA | AC-001, AC-006 | `_evidence/residue-after.txt` empty for tracked files; the *Harness* row carries the command and the SHA, so the re-run is a copy-paste |
| **Reachable without a tool** (the analogue of keyboard reachability) — every artefact is a plain UTF-8 file readable in any editor, and every command in the record is copy-pasteable as written | AC-001, AC-004 | `_evidence/*.txt` are plain text; no captured output is a binary log or a screenshot |

**Composition family.** Taken from the record's own contract and the two accepted playbooks, since
`_design.md` declares none:

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **Presentation exists at all** — the record is a composed document with sections, a stated column contract and a coverage line, not a bare list of paths appended by whoever arrived first | AC-005 | Structural review against B-8/B-9 |
| **Placement** — the record sits in the *project* directory, not inside any story folder, because it is read across stories; the raw evidence sits inside *this* story's folder, because it is this story's provenance | AC-005, and the PR boundary block above | The two paths in the PR boundary fenced block are exactly these two locations |
| **Transience** — the record is persistent chrome (it outlives every story and is the project's deliverable); the clone and its `target/` are transient and are discarded; the `_evidence/*.txt` captures are persistent precisely because the thing they describe is not | AC-002, AC-006 | *Data and migrations* above states this; the evidence files are committed, the clone is not |
| **Density budget, with its real numbers** — one record file; nine sections; the *DoD* section exactly fourteen rows (DoD 1–12, 14–15); the *Harness* table five columns (SHA, commit date, isolation command, residue-output path, probe-manifest path) plus the per-row SHA already among them; one coverage line. A tenth section or a fifteenth DoD row means someone re-shaped the artefact instead of appending to it | AC-005 | Counted in structural review |
| **Hierarchy** — the harness row comes first because every other section's rows are claims *about the tree it pins*; findings come last because they are produced by all of the others | AC-005, AC-006 | Section order in B-8 |
| **Named anti-patterns** — (a) running the gate in the warm working tree these planning artefacts were authored in and calling it a clean checkout (`discover.md` *The wrong implementation*); (b) an evidence cell carrying a bare `file:line` with no subject string; (c) a sibling's `_ledger.md` cited as evidence rather than as a pointer; (d) a *Findings* section with a fix column; (e) a second record file | AC-001 (a), AC-005 (b, d, e), AC-006 (c) | Each is a review check with a named artefact to look at; (a) is caught by `_evidence/head-sha.txt` showing a detached HEAD in a directory outside every existing checkout |

## Error conditions

| id | Condition | Required response |
| --- | --- | --- |
| **EC-1** | `git status --porcelain --ignored` is **non-empty** in the fresh clone | Halt the slice and record it. A file present in a fresh clone is either tracked-and-modified (impossible immediately after checkout, so: a `.gitattributes`/EOL effect worth naming) or committed-and-ignored (a real contradiction). **Do not delete anything** — deleting the residue destroys the evidence and repairs a defect this project is forbidden to repair (`project.md` AC-013). Route it through `findings-disposition-register`. |
| **EC-2** | The clone succeeds but the closeout SHA is not reachable from any remote ref | The commit exists only on this machine, so "the assembled tree" is unauditable. Push the branch and re-pin, or pin to an already-pushed SHA. **Never** fall back to `git worktree add` or to cloning the local path to work around it — both defeat B-1's reason for cloning from `origin`. |
| **EC-3** | The override scan finds a `[patch]`, a `paths` override, a `replace-with` or a vendored directory | DR-1 is violated on the tree. Record the hit with its file and line in `_evidence/override-scan.txt`, mark AC-003 unsatisfied, and route the finding. Editing the manifest to remove it is out of this project's scope (`project.md` *Out of scope*) and would be a code change outside this story's PR boundary. |
| **EC-4** | A probe `CLAUDE.md` says resolves on this machine (`cargo hack`, `cargo deny`) exits non-zero in the manifest | The gate would print `skipped:` for a step that ought to run, silently weakening AC-001's claim. The **host** is not the tree, so repairing the host tooling is permitted and is not a scope breach — but the manifest must record both states, before and after, so the reader can see what was fixed. A run whose transcript shows either step skipped is a finding, not a green gate. |
| **EC-5** | After the slice's gate run, `git status --porcelain` shows a modified tracked file | A gate step mutated the tree. `cargo xtask spec-trace` is the likeliest candidate and the gate invokes it deliberately without `--write` (`xtask/src/main.rs:315-327`), so this is a real signal. Capture the file list into `_evidence/residue-after.txt`, do **not** commit it into the clone, and route the finding. |
| **EC-6** | The clone or the cold build exhausts disk, or the network fetch fails partway | Retry the clone into a fresh directory rather than resuming into a partial one — a half-populated clone is exactly the residue state this story exists to rule out. Never reuse a shared `CARGO_TARGET_DIR` to save time: the cold `target/` is the stale-artefact half of DR-1, not overhead (NF-1). |
| **EC-7** | `_closeout-record.md` already exists when this story runs (a re-run of the slice at a later SHA) | **Append a harness row; never re-create or re-shape the file.** Re-creating it destroys every row a later story already filled, and re-shaping it breaks the citations written against its sections (`.kb/playbooks/anchoring-citations-in-a-long-lived-document.md`). The record's own preamble states this rule so a later story meets it without reading this spec. |
| **EC-8** | The clone lands on a path long enough to trip Windows' path limit, and a build step fails for that reason | Recognised and rejected as a finding: this is a property of the *host*, not of the tree. Re-clone into a short root and re-run. A gate failure recorded without noticing this would be a false finding attributed to the library. |

## Non-functional

| id | Requirement | Why, and how it is checked |
| --- | --- | --- |
| **NF-1** | **The cost is paid, not optimised away.** A full network clone and a cold `target/`, with the whole gate — including two feature powersets, `cargo deny` and a nightly rustdoc build — run on top of it. No shared build cache, no `--fast`, no reused clone. | The cold build is the DR-1 requirement, not overhead (`project.md:129-131`); `--fast` is the bar every sibling already met (DR-2, `.redkiln/config.yaml:55`). Elapsed wall-clock is recorded in the harness row so the next closeout can budget from a measurement rather than a guess. |
| **NF-2** | **Auditability over convenience.** Every empty result is captured to a file with its command on record; nothing load-bearing is read from terminal scrollback. | An empty output observed and not captured is indistinguishable from a command never run — the same failure the coverage rule in `.kb/playbooks/verify-the-referent-and-report-coverage.md` was written against. |
| **NF-3** | **The durable footprint is the evidence, not the checkout.** The clone is discardable; the committed residue is `_evidence/*.txt` plus the record. Total committed bytes are small and plain text. | *Data and migrations* above. A story that keeps the clone around as "the artefact" has stored something nobody can review. |
| **NF-4** | **No credential travels into the evidence.** The remote is public HTTPS (`https://github.com/Wet-Ink-Corporation/happenstance.git`); if any captured transcript contains a credential-bearing URL or token, it is scrubbed before commit and the scrub is noted. | The transcripts are committed to a repository intended to be published. |
| **NF-5** | **Zero code, zero runtime cost.** This story changes no crate, so `cargo xtask affected --base main` names no affected package and the library's compile time, binary size and MSRV are untouched. | `.redkiln/config.yaml`'s `affected_gate`; `project.md` *Out of scope*; the `testing` brief's AC-013 row ("zero commits inside this project's own stories touch code outside `.bklg/`/`.kb/` planning artefacts"). |

## Implementation notes (non-prescriptive)

Not requirements — the shape the author of this spec expects, offered so the implementer can
disagree with something concrete.

- **Order matters more than tooling.** Clone → detach → residue check → override scan → lock hash →
  probe manifest → (slice-mate: gate run, teed) → post-run tracked-file check. Every step before
  the run must be capturable *only* before it; taking the residue check after the build makes it
  unfalsifiable, and taking the probe manifest after makes a `skipped:` line unattributable.
- **A shell transcript is fine; a script is not.** `script`, `tee`, or simply redirecting each
  command's output into its own file all satisfy the evidence contract. Committing a
  `scripts/clean-checkout.sh` does not — *Context pack* decision 5 and the PR boundary rule it out,
  and it is the fastest wrong thing to reach for.
- **Pick the clone directory outside every existing checkout.** Something like `C:\hs-closeout\` on
  this machine: short (EC-8), and demonstrably not inside `D:/repos/happenstance` or its
  `.claude/worktrees/` (B-1). Record the absolute path in the transcript so the reader can see the
  containment claim for themselves.
- **The override scan is four greps, and the fourth is a directory test.** `^\[patch` across every
  `Cargo.toml`; `^paths\s*=` and `replace-with` across every `.cargo/config.toml` and
  `~/.cargo/config.toml`-equivalent *inside the clone* (an ambient user-level config is ruled out by
  nothing but this scan being run in a fresh clone, so say which forms were checked where); and the
  absence of a `vendor/` directory. Record the commands, not just the verdict.
- **Write `_closeout-record.md` from the section list in B-8 verbatim.** The temptation is to write
  only the sections this slice can fill and let later stories add theirs. That is exactly the shape
  AC-005's non-occlusion invariant forbids: an owed row must be visible as owed.
- **The coverage line is arithmetic, not prose.** `rows filled / rows owed` with both integers
  stated, so a reader who does nothing but read that line still learns the record is incomplete.
- **Two file-name conventions to keep.** Evidence under `_evidence/` with the names this spec fixes
  (later stories cite these addresses); the record at the project directory root as
  `_closeout-record.md`, matching `_storymap.md:24-30`.

## Tests and CI (merge gate)

The `testing` brief (`_decomposition.md` *Test mix, summarised by tier*) is explicit that **static**
and **process** are load-bearing tiers here, because this project audits rather than builds. There
is no new unit test to write and writing one would mean this story had grown code it is forbidden
to have.

| Tier | Command / path | Proves |
| --- | --- | --- |
| **Process** | `git clone https://github.com/Wet-Ink-Corporation/happenstance.git <dir>` then `git -C <dir> checkout --detach <SHA>`, captured to `_evidence/clone-transcript.txt` | AC-001 — the isolation actually happened, from `origin`, into a directory outside every existing checkout |
| **Static** | `git -C <dir> rev-parse HEAD`, `git -C <dir> log -1 --format=%cI`, `git -C <dir> branch -r --contains <SHA>` → `_evidence/head-sha.txt` | AC-001 — the SHA is exact, dated, and reachable from a pushed ref |
| **Static** | `git -C <dir> status --porcelain --ignored` → `_evidence/residue-before.txt` (empty), taken before any `cargo` invocation | AC-002 — no untracked and no ignored residue, including `target/` |
| **Static** | The four override scans over the clone → `_evidence/override-scan.txt` (zero hits, plus the recorded `Cargo.toml:24-26` non-hit explanation) | AC-003 — no `[patch]`, no `paths`, no `replace-with`, no vendor directory |
| **Static** | `git -C <dir> hash-object Cargo.lock` → `_evidence/lock-hash.txt`; re-run after the slice's gate | AC-003 — the committed resolution is what was measured, backed by `--locked` at `xtask/src/main.rs:52-54` |
| **Process** | `cargo hack --version`, `cargo deny --version`, `cargo +nightly --version` each under `RUSTUP_AUTO_INSTALL=0`, plus `rustc -Vv` → `_evidence/probe-manifest.txt` | AC-004 — every future `skipped:` line is attributable to a named absent tool (`xtask/src/main.rs:894-908`) |
| **Static** | Structural review of `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md` against B-8/B-9/B-10 — nine sections in order, fourteen DoD rows, destination-not-fix column, coverage line, column contract, preamble rules | AC-005 — the convergence artefact exists with its shape fixed, and every owed row is visible as owed |
| **Static / process** | `git -C <dir> status --porcelain` after the slice's gate run → `_evidence/residue-after.txt` (empty), read together with the *Harness* table's per-row SHA column | AC-006 — the checkout stayed read-only, and evidence is pinned per row rather than per document |
| **E2E (slice-mate, same slice)** | `cargo xtask ci` inside `<dir>`, teed to `_evidence/gate-transcript.txt` — the terminal grain at `.redkiln/config.yaml:60`, never `--fast` | Consumes AC-001…AC-004; supplies the `skipped:` lines AC-004's manifest makes attributable. Owned by `whole-gate-green-on-the-assembled-tree`, listed here because this story fixes the transcript path it must use |
| **Merge gate (story grain)** | `redkiln verify --grain story` — reads `_ledger.md`, the PR-boundary fenced block, and commit provenance (`.redkiln/config.yaml:67`, `:73`) | Every AC has a ledger row with cited evidence; no file changed outside the two-line boundary |
| **Merge gate (affected)** | `cargo xtask affected --base main` | NF-5 — no crate is affected, which is the observable form of "this project owns no code" (`project.md` *Out of scope*) |

The four merge-gate commands the `testing` brief reserves for this project
(`cargo xtask ci`, `redkiln validate`, `redkiln validate --kb`, `redkiln doctor --json`) are **not**
this story's bar. The first belongs to the slice-mate; the last three belong to
`backlog-and-kb-health-at-closeout` in slice 5. Running them here would not be wrong, but citing
them as this story's evidence would be — this story's claim is about the *tree*, not about what
passed in it.

## Risks and coupling (PR-scoped)

| Risk / coupling | Note |
| --- | --- |
| **The warm-tree shortcut** | Named in `discover.md` *The wrong implementation*: running the gate in the very worktree these planning artefacts were authored in and reporting it as "the assembled tree, clean checkout". It satisfies "the gate passed" literally and constructs nothing. It is the single most likely way this story is done wrongly, because it is faster by roughly the cost of a full clone and a cold build. Caught by AC-001's containment evidence and AC-002's `--ignored` capture. |
| **Coupling to the slice-mate is tight and deliberate** | AC-004's transcript path and AC-006's post-run check are only completable once `whole-gate-green-on-the-assembled-tree` has run. That is why `_storymap.md` *Why the slices are cut here* keeps the seam and the run in one slice — the two land as one integrated surface, in one context, in this order. Do not attempt to close this story's ledger before the slice's run has happened. |
| **Every later story cites this SHA** | If the pin is wrong, mis-transcribed or unpushed, five downstream stories inherit a bad citation. Cheap mitigation: AC-001 requires `branch -r --contains`, so an unpushed SHA fails here rather than in slice 5. |
| **Staleness by construction** | Slices 4 and 5 commit into `.kb/` and `.bklg/`, so this pin is stale by the time the findings register is written. B-7 makes re-running the harness at a later SHA the required response and reusing this row the forbidden one — but nothing mechanical enforces it, so the record's preamble carries the rule where a later author will read it. |
| **Pressure to fix what the harness finds** | The whole project's standing risk (`project.md` risk table) lands here first, because this is the first tree where residue is visible. EC-1 and EC-3 both say record-and-route; a story that tidies the tree it was measuring has destroyed its own evidence and failed AC-013 on behalf of a sibling. |
| **Cost, and the temptation to share a build cache** | The clone plus cold build is the expensive part of the whole project. A shared `CARGO_TARGET_DIR` would cut it and would silently delete the stale-artefact half of DR-1 (NF-1, EC-6). |
| **Host tooling drift** | `CLAUDE.md` records that `cargo-hack` and `cargo-deny` resolve on this machine. If that has changed, the gate's optional steps skip and AC-001's "green" claim quietly covers less. EC-4 makes that a finding rather than a footnote. |
| **The record's shape is a contract with five future stories** | `dod-set-re-observation-record`, `published-tree-delta-statement`, `decision-atom-audit-table`, `open-question-preservation-audit` and `findings-disposition-register` all append to it. Getting the section list wrong here means either a later story re-shapes it (breaking citations) or writes beside it (recreating the fourteen-places failure mode). B-8 is therefore stated as a fixed list, not as a suggestion. |
| **Windows specifics** | Long paths (EC-8) and EOL normalisation on checkout are both capable of producing a false residue signal or a false build failure. Both are host properties; recognising them as such is part of not attributing a false finding to the library. |

## Dependencies

**Blocks on:** nothing. `depends_on: []` — this story is the DAG root of the project
(`_storymap.md:140-143`: "First because everything downstream cites the run it produces, and because
a gate failure here is the finding the whole project exists to surface"). At the *project* level
HS-P0019 depends on all nine siblings having merged, but that is the project's edge, not this
story's, and it is discharged before any story here runs.

**Unlocks:**

- `whole-gate-green-on-the-assembled-tree` — same slice, immediately; it runs inside this checkout
  and records its result in this record (`_storymap.md:54`).
- `dod-set-re-observation-record` and `published-tree-delta-statement` — slice 2; both write rows
  into `_closeout-record.md` under the column contract fixed here.
- `decision-atom-audit-table` and `open-question-preservation-audit` — slice 3; same record, same
  contract. No DAG edge to this story, but no place to write without it.
- `findings-disposition-register` — slice 5; its *Findings* section, with a destination column and
  no fix column, is created here.

## Anchors (progressive disclosure)

Read the *Context pack* first; open a row below only when the AC it serves is the one being worked
on. Nothing here is optional detail — it is deferred detail.

| Anchor | Why it is load-bearing | When to open | Serves |
| --- | --- | --- | --- |
| `.bklg/from-contract-to-published-library/closeout-and-durable-audience/project.md` | DR-1 (`:129-131`) is the literal requirement this story constructs, and AC-001 (`:192-195`) is the traced project criterion including its skip clause. The *Out of scope* section is what forbids a harness script. | Before writing the isolation procedure, and again before deciding where any file lands | AC-001, AC-002, AC-003, AC-004 |
| `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_decomposition.md` | The one warranted `testing` brief: *Fixtures / seams* (`:109-123`) names the clean checkout as the one seam this project must construct and says nothing is mocked; the AC-001 row gives the tier; the AC-013 row is the "zero commits touch code" criterion this story's PR boundary is written against. | Before choosing between clone and worktree, and before adding any file outside `.bklg/` | AC-001, AC-002, AC-003 |
| `xtask/src/main.rs` | The gate defined once. The module doc (`:44-54`) names the four tool-gated steps and the `--locked` invariant; `REQUIRED`/`OPTIONAL` at `:105`/`:535`; the probes at `:556`, `:593`, `:600`, `:635`; the `skipped:` print at `:873-878` and `is_available` at `:894-908`, including why `RUSTUP_AUTO_INSTALL=0` is what makes a probe a probe; `spec-trace` invoked without `--write` at `:315-327`. | Before writing the probe manifest (AC-004) and before interpreting the post-run tracked-file check (AC-006) | AC-003, AC-004, AC-006 |
| `.kb/playbooks/verify-the-referent-and-report-coverage.md` | The accepted playbook behind the record's column contract and coverage line: an address that resolves says nothing about whether the attributed content is there, and a check that does not state what fraction it parsed is indistinguishable from one that sees everything. Grounded in a parser that checked 84 of 338 citations while printing "no problems found". | While writing `_closeout-record.md`'s preamble and column contract | AC-005 |
| `.kb/playbooks/anchoring-citations-in-a-long-lived-document.md` | Why the record asks for a path *plus* a subject string rather than a bare `file:line` — the targets move, and this record is written against a tree five later stories will change. Also why EC-7 forbids re-shaping the file. | While fixing the evidence-cell shape, and again if tempted to renumber or reorder sections | AC-005, AC-006 |
| `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_storymap.md` | `:24-30` names `_closeout-record.md` as the convergence artefact and states the fourteen-places failure mode AC-003's word "set" is written against; `:53-54` are this story's row and its slice-mate's; `:116` is the AC-001 split; `:140-143` is the merge order. | Before creating the record, and before assuming what a later story will append | AC-005, AC-006 |
| `.bklg/from-contract-to-published-library/closeout-and-durable-audience/clean-checkout-harness/discover.md` | *The wrong implementation* names the warm-tree shortcut precisely, including why it makes the slice-mate's citation worthless; *Questions* records the two things deferred to this spec and now decided. | First, before any command is run — it is the shortest statement of what failure looks like | AC-001, AC-002 |
| `.redkiln/config.yaml` | `:55` vs `:60` — `--fast` is every sibling's bar and `cargo xtask ci` is this project's alone; `:67` `require_ledger` and `:73` `require_commit_provenance` are why this story's `_ledger.md` must cite the isolation command and the residue output as artefacts; the `affected_gate` entry is NF-5's check. | Before filling `_ledger.md`, and if anyone proposes running `--fast` | AC-004, AC-006 |
| `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_grounding.md` | `:54-66` is the authoritative current gate step list and ordering, more granular than `CLAUDE.md`'s summary, and is the citation AC-001's skip clause should be checked against. `:67-78` is the six-file drift assertion (slice 5's, not this story's — open it to confirm it is *not* owed here). | Before writing the probe manifest, if the four tool-gated steps need naming exactly | AC-004 |
| `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_design.md` | The approved *no public API surface* determination, sign-off 2026-08-12. Load-bearing negatively: it is why the composition family above inherits nothing and why no surface invariant may be ticked as met. | If any reviewer asks why this spec declares no design invariants | AC-005 |
| `.bklg/from-contract-to-published-library/initiative.md` | `:356-358` is the Definition-of-Done preamble ("run and observed to pass on the assembled library, from a clean checkout") that this story makes true rather than aspirational; `:396-397` is DoD 13; `:249-250` is the *Decide in one sitting* journey the acceptance criteria are framed from. | When framing or reviewing the acceptance criteria's reader | AC-001, AC-005 |
| `.gitignore` | The operative definition of "ignored residue" — `target`, `debug`, `**/mutants.out*/`, `.idea/`, `.cargo/credentials{,.toml}`, `.mcp.json`, `.redkiln-cli/`, `.claude/`. Without it, `--ignored` returning empty is a claim about an unstated set. Its `.claude/` comment (`:47-57`) is also the repository's own statement of why a worktree-shaped tree is not a clean one. | While interpreting `_evidence/residue-before.txt`, and if EC-1 fires | AC-002 |
| `Cargo.toml` | `:24-26` — the workspace's own `version`+`path` dependencies, which AC-003 requires be recorded as **non-hits with the reason**. The single most likely misreading of DR-1 is that these are what it forbids. | While writing the override scan's recorded explanation | AC-003 |
| `.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md` | `:249-262` — *Persona 4, the evaluator (pre-adoption)*: the reader every criterion here is framed from, who decides "in a bounded amount of research time" and cannot run the suite. Load-bearing because it is why the criteria are written as *what a reader can see* rather than as *what was run*. | Before reviewing or re-wording any acceptance criterion | AC-001, AC-005, AC-006 |

## Clarifications resolved during spec

1. **Which isolation mechanism** — deferred by `discover.md` *Questions*, first bullet, because
   `_decomposition.md:116-120` named "a fresh `git worktree add` or clone" without choosing.
   **Resolved: a fresh `git clone` from `origin`, checked out detached.** The worktree shares the
   primary repository's common `.git`, so `info/exclude`, a local `core.excludesFile` or a hook can
   silence `git status` with nothing committed recording that it happened — and the residue check is
   the entire point. The clone additionally proves the SHA is reachable from a pushed ref. Cost
   accepted: a full fetch and a cold `target/` (*Context pack* 1, NF-1).
2. **Whether a path dependency exists to rule out** — deferred by `discover.md` *Questions*, second
   bullet. **Resolved as a scoped scan rather than a resolution audit.** The workspace's own
   `version`+`path` dependencies (`Cargo.toml:24-26`) are ordinary practice and are *not* what DR-1
   forbids; what is forbidden is `[patch]`, a `paths` override, `source.replace-with` or a vendored
   directory. The fresh clone rules out the ambient forms and `--locked` rules out a divergent
   resolution; AC-003 owns the in-tree scan and requires the non-hits be recorded with their reason
   (B-3).
3. **What is in `_closeout-record.md`** — `_storymap.md:24-30` named the artefact without saying what
   it contains, which would have left its shape to whichever story reached it first. **Resolved: nine
   sections in a fixed order, a column contract of path + subject string + SHA, a destination column
   in *Findings* and no fix column, and an arithmetic coverage line** (B-8, B-9, AC-005).
4. **Evidence file names are fixed by this spec**, not left to the implementer, because five later
   stories will cite them and a renamed artefact is a dead citation
   (`.kb/playbooks/anchoring-citations-in-a-long-lived-document.md`). They live under this story's
   `_evidence/` directory, inside the PR boundary.
5. **AC count unchanged.** The six ids the first pass enumerated (AC-001…AC-006) are exactly the six
   written here; none was added or dropped. The interaction-quality invariants were mapped onto
   existing rows rather than given rows of their own, because every one of them is a property of the
   checkout (AC-001, AC-002, AC-006) or of the record's shape (AC-005) that those criteria already
   assert — a seventh row would have restated one of them.
6. **The post-run checks (AC-006's `residue-after`, AC-003's second lock hash) complete only after
   the slice-mate's run.** That is not a hidden dependency: the slice is implemented in one context
   with this story first (`_storymap.md` *Why the slices are cut here*), so both files are written
   before either story's ledger closes. Stated explicitly so nobody flips AC-006 early on the
   strength of the contract being *recorded* rather than *observed*.
7. **Not decided here, deliberately**: the evaluator-persona question (DR-10 — settled in
   `_decomposition.md` and executed in slice 4) and the DoD 13 delta (DR-4 —
   `published-tree-delta-statement`). `discover.md` *Questions*, third bullet, already flagged both
   as absent by design; they are named again here so a reader does not read their absence as an
   omission.
