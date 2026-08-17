---
item: HS-S0181
stage: spec
created: 2026-08-17T13:16:30.754Z
updated: 2026-08-17T13:16:30.754Z
template_sig: 87bbf1d0
rendered_sig: 22a50b71
---

# Spec — Re-observe scenario 2 in both halves: fail by name, then recover

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/docs-that-teach/initiative.md` — DoD-2 at `:416-420`, the fifteen scenarios at `:406-468` |
| Initiative decomposition | `.bklg/docs-that-teach/_decomposition.md` — the merge-forward rule (`:89-103`), the terminal flag (`:306-308`) |
| Project | `.bklg/docs-that-teach/durable-audience-closeout/project.md` — AC-014 (`:203`), AC-015 (`:204`), DR-11 |
| This spec | `.bklg/docs-that-teach/durable-audience-closeout/scenario-two-fault-injection/spec.md` |
| Key briefs (sections, not files) | `.bklg/docs-that-teach/durable-audience-closeout/_decomposition.md` — `## Architecture brief` §"Scenario 2 contains a mutation" (`:642-647`), `## UX brief` AC-UX-09…AC-UX-12 (`:266-281`), `## Testing brief` AC-TB-04/05 (`:833-838`) and the fault-injection seam (`:795-802`) |
| Signed-off design | `.bklg/docs-that-teach/durable-audience-closeout/_design.md` — `hasSurface: false`, approved 2026-08-17, no conditions. This project renders **no** surface (`:10-47`, `:85-93`) |
| Story map row | `.bklg/docs-that-teach/durable-audience-closeout/_storymap.md:57` (this story), `:35` (activity A4), `:76-78` (why `dod-reobservation` is one surface) |
| Roadmap pointer | `.redkiln/config.yaml:60` — `e2e: cargo xtask ci`, the terminal grain this project alone is held to |
| The gate under observation | `.bklg/docs-that-teach/checked-documentation-surface/_design.md` — the pinned tree `const TREE = "docs"` (`:93`), `const HARNESS` (`:96`), the problem-line shape (`:37`), the checker's banner (`:761`) |

## One-line PR slice

Re-observe DoD scenario 2 in both halves: break the pinned page deliberately, capture the
gate's failing-by-name output verbatim as the failing half's evidence, revert, and re-run
green as the recovery half — with the broken edit never a committed state of the tree.

## Executive summary

`dod-scenario-ledger` (HS-S0180) creates the fresh checkout off the merged branch and lands
the fifteen-row re-observation ledger. Fourteen of those rows are a *read* — run the
scenario, write what was seen. Scenario 2 is the only row that requires **mutating the tree
under observation**, and it is therefore split out here as its own story because AC-015
makes it a separate obligation from AC-014 (`_storymap.md:127`).

The delta this PR lands, on top of the ledger the dependency created:

1. A deliberate break to one claim on one page under the pinned narrative tree, applied in
   the fresh checkout only.
2. The failing gate run's output, captured **verbatim** — including which file it actually
   named — as the failing half's evidence.
3. The revert, and the same command re-run to green as the recovery half's evidence.
4. Scenario 2's single ledger row replaced by **two** named rows, `failed by name` and
   `recovered`, each self-contained and each carrying its own evidence.
5. A recorded proof that the broken edit was never committed, so the terminal
   `cargo xtask ci` in `terminal-gate-run` (HS-S0182) is taken on a tree that does not
   contain it.

This story observes a gate that HS-P0020 `checked-documentation-surface` built. It does not
build, fix, tune or re-scope that gate. If the observation finds the gate wrong, the finding
is recorded and routed — it is not repaired here.

## Context pack

Everything below is a decision already taken elsewhere and binding on this story. Read it
before touching anything; open the anchors only where a row says to.

**D1 — This is a re-observation, not a first observation, and "inherited from a sibling" is
inadmissible.** HS-P0020's `observed-failure-falsification`
(`.bklg/docs-that-teach/checked-documentation-surface/_storymap.md:55`) already breaks a
claim, records the failure verbatim, reverts and re-runs green — *inside its own project, on
its own tree*. That run is not this row's evidence. The decomposition made this project the
terminal DoD owner: fourteen scenarios are *made true* by a sibling and all fifteen are
**re-observed here on the assembled result** (`.bklg/docs-that-teach/_decomposition.md:236-239`).
AC-014 forbids "inherited from a sibling" as an evidence value, and AC-UX-11 makes the
literal string's presence in an outcome column a review-blocking defect
(`.bklg/docs-that-teach/durable-audience-closeout/_decomposition.md:274-277`). The named
failure mode this story exists to prevent is the project risk table's *"fifteen
re-observations become fifteen ticked boxes"* (`project.md:249`).

**D2 — The failing half is the half that matters, and it is evidenced by a transcript, not
by a claim.** DoD-2 requires both halves observed and says outright that "the failing half is
the one that matters" (`.bklg/docs-that-teach/initiative.md:416-420`). AC-TB-05 tightens
this into an evidence rule: scenario 2's ledger entry carries the failing run's **captured
output**, not only the fact that it failed
(`.bklg/docs-that-teach/durable-audience-closeout/_decomposition.md:837-838`). "Recovered"
without a captured failing transcript is the exact gap AC-UX-11 already flags
(`:801-802`). A paraphrase, a summary, a screenshot of a scrollback, or a re-typed
approximation of the error line all fail this — the bytes the gate wrote are the artefact.

**D3 — "Fails by name" means the output identifies the page and the location, and what it
actually named is recorded as-seen.** The gate HS-P0020 built prints problems in the shape
`  {path}:{line} — {message}` on stderr
(`.bklg/docs-that-teach/checked-documentation-surface/_design.md:37`), under a step banner
that is the claim sentence `=== every narrative page is checked ===` (`:761`), over a tree
pinned as `const TREE: &str = "docs"` (`:93`) with the compiling harness at
`const HARNESS: &str = "xtask/src/narrative.rs"` (`:96`). HS-P0020's own falsification story
is explicitly written to record "which file it actually names"
(`.bklg/docs-that-teach/checked-documentation-surface/_storymap.md:55`) — because a fence
compiled through the harness can name the harness rather than the page. This story inherits
that honesty requirement: **record the file the gate named, verbatim; do not reword the
transcript so that it names the page.** If the gate names something other than the broken
page, the row still records what was seen and the discrepancy is written down as a routed
gap for HS-P0020, not fixed here (`project.md:102-108`, out of scope).

**D4 — The mutation must not survive, and that is an architectural constraint, not
tidiness.** The architecture brief states it directly: the failing half's evidence is a
captured transcript in the ledger, the revert is proven by the subsequent green run, and the
broken edit is **never a committed state of the tree** the final `cargo xtask ci` is taken on
(`.bklg/docs-that-teach/durable-audience-closeout/_decomposition.md:642-647`). Ordering 4 of
the same brief — everything committed before the final gate — is what makes this load-bearing:
`terminal-gate-run` (HS-S0182) runs the whole gate on the exact tree carrying every artefact,
and a surviving break would either fail that gate or, worse, be reverted in the same commit
that claims it was never there. Stage the break in the working tree, capture, revert, verify
clean, then commit only the records.

**D5 — The tree is the merged, assembled one, and it is named by sha in every row.** The
merge-forward of `initiative/from-contract-to-published-library` precedes every observation
(AC-A06, `:380-387`); the re-observation runs in a checkout created fresh off the **merged**
branch, never in `.claude/worktrees/docs-that-teach` itself (AC-A07, `:388-391`; the testing
brief's clean-checkout seam, `:789-794`). AC-UX-12 requires every observation to name the
tree it was performed against by ref or sha so a later reader can re-run it (`:278-281`).
That checkout is created by `dod-scenario-ledger`, not by this story — this story runs in it.

**D6 — Two rows, each self-contained.** AC-UX-11 fixes the shape: scenario 2 occupies **two
named halves** in the ledger — the gate `failed by name` and the gate `recovered` — each with
its own evidence (`:274-277`). AC-UX-10 requires every ledger row to stand alone: scenario id,
owning project, observer, tree ref or sha, outcome, with no row that depends on the row above
to be understood (`:270-273`). AC-TB-04 adds the checkout path and forbids the inherited-value
string (`:833-836`). AC-UX-09 is the accessibility floor and it binds this table: **no state
anywhere in this project's diff is carried by colour, emoji, glyph, strikethrough, ordering or
an empty cell** (`:266-269`) — so "the failing row is the one above the green one" is not a
record, and an empty outcome cell is not "no finding".

**D7 — The persona-journey slice.** This project's reader is not the three documented
personas; it is **the closeout reviewer (U2)** and the next initiative's charter author (U1)
(`_storymap.md:37-39`). What U2 does with these two rows is decide whether the gate was ever
actually watched failing. The whole reason the initiative wrote DoD-2 at all is the
in-house precedent at `RUNBOOK.md:918-925` — a step that was wired, vouched for by two
documents, and printed `skipped` on all three runners
(`.bklg/docs-that-teach/checked-documentation-surface/_storymap.md:36-38`). A row that says
"gate failed" without the bytes leaves U2 exactly where that precedent left the last reader.

**D8 — No Accepted decision atom governs this surface, and that is a finding rather than a
gap.** All sixteen accepted decision atoms govern the Rust contract; none binds documentation
structure, KB promotion or observation practice
(`.bklg/docs-that-teach/durable-audience-closeout/_decomposition.md:285-296`). Do not hunt for
a documentation ADR to cite, and do not write one: authoring or touching `.kb/decisions/` is
forbidden here (AC-A09, `:396-400`; `.kb/governance/rewrite-the-referent-never-the-reasoning.md`).

**D9 — This story ships no Rust and no surface.** The project's diff is confined to `.kb/`,
the two map files and this project's own backlog folder; a story proposing to edit `crates/`,
`spec/`, `xtask/`, `standards/` or `docs/` has left the project (`_storymap.md:21-23`). The
`docs/` edit this story makes is **transient by construction** — it exists in the working tree
of a throwaway checkout for the length of one gate run and appears in no diff. `_design.md`
records `hasSurface: false` with human sign-off, so there is no composition, density budget or
hierarchy to honour beyond the ledger table's own row shape (D6).

## Integration contract

| Field | Value |
| --- | --- |
| **Archetype** | `capability` — a user-observable slice; the "user" is the closeout reviewer (U2) reading the ledger |
| **Slice / milestone** | `dod-reobservation` |
| **Slice-mates** | `dod-scenario-ledger` (HS-S0180, upstream), `terminal-gate-run` (HS-S0182, downstream). All three run in the *same* fresh checkout off the merged branch, which is why they are one surface (`_storymap.md:76-78`) |
| **Mount point** | `.bklg/docs-that-teach/durable-audience-closeout/_dod-ledger.md` — the fifteen-row re-observation ledger table created by `dod-scenario-ledger`. This story does not create a parallel file: it replaces that table's single scenario-2 row with two rows. **The mount is the table, not the filename** — if the slice-mate's spec names a different file in this project's folder for the same fifteen-row table, that name wins and this story mounts there. A second scenario-2 record anywhere else in the tree is the defect this clause exists to prevent |
| **Wires into** | The ledger row contract from `dod-scenario-ledger` (observer, checkout path, tree sha, outcome, evidence) — AC-UX-10 (`_decomposition.md:270-273`), AC-TB-04 (`:833-836`); the fresh checkout that story created off the merged branch (AC-A07, `:388-391`); the gate binary `cargo xtask ci` as defined once in `xtask/src/main.rs` and wired as the terminal grain at `.redkiln/config.yaml:60`; the pinned narrative tree and checker HS-P0020 built (`.bklg/docs-that-teach/checked-documentation-surface/_design.md:93`, `:96`, `:37`) |
| **Renders surfaces** | **None.** `_design.md` records no public API surface and no rendered UI surface for this project, signed off by the repository owner on 2026-08-17 with no conditions (`_design.md:10-47`, `:85-93`). The nearest thing to a surface this story touches is the two ledger rows, whose shape is fixed by AC-UX-09/10/11 rather than by a design item |
| **Public items** | None. `_design.md`'s `## Items` block is explicitly empty (`_design.md:45-47`) |
| **Conformance rule(s)** | None, and this is not adapter-observable: this story compiles no Rust, touches no port and adds no conformance rule. The testing brief classifies AC-015 as **Tier 4 — e2e** with no other tier able to falsify it (`_decomposition.md:875`), because the only instrument that can catch a fabricated observation is a human reading a captured transcript |
| **Clause(s)** | None discharged or amended. `spec/SPECIFICATION.md` is read-only for this project (`_decomposition.md:419-425`); `cargo xtask spec-trace` runs *over* it as a step inside the gate and does not edit it |
| **Advances DoD scenario** | **DoD-2** — "a deliberately broken page fails the gate, by name" (`.bklg/docs-that-teach/initiative.md:416-420`) — to green in both halves. Contributes rows to DoD-14's ledger obligation via AC-014, and is a precondition of the terminal `cargo xtask ci` in `terminal-gate-run` being taken on a tree that carries this evidence |

This story is delivered **mounted**: its output is two rows inside the project's existing
fifteen-row ledger table plus the captured transcripts they cite. A transcript captured into
a file nothing links to, or a scenario-2 record kept in this story's folder while the
project ledger still shows one unresolved row, is the "component rendered into no tree" shape
the architecture brief names (`_decomposition.md:435`).

## PR boundary

**In this PR**

- The two scenario-2 rows in the project's DoD re-observation ledger, replacing the single
  placeholder row `dod-scenario-ledger` left for them.
- The captured evidence artefacts: the failing run's verbatim output and the recovering
  run's verbatim output, stored under this story's own folder and cited by the rows.
- The recorded proof that the broken edit was never committed — the `git status` /
  `git log` observation taken after the revert and before the commit.
- A prose note, in this story's folder, of the break that was chosen and why it is a claim
  break rather than a formatting break, and of any discrepancy between the file the gate
  named and the page that was broken (D3).
- This story's `spec.md` body, and its `_ledger.md` (`require_ledger: true`,
  `.redkiln/config.yaml:67`).

**Explicitly not in this PR**

- The fresh checkout and the other fourteen ledger rows — `dod-scenario-ledger` (HS-S0180).
- The terminal `cargo xtask ci` run and its green record — `terminal-gate-run` (HS-S0182).
  The green re-run this story takes is the **recovery half's** evidence for one scenario; it
  is not AC-016's evidence and must not be presented as it (`_decomposition.md:830-832`).
- Any change to the narrative tree, the checker, the harness, the gate step, its message
  format or the allowance list. HS-P0020 owns all of it (`project.md:106-108`); a gate
  defect found here is a routed gap.
- Any permanent edit under `docs/`, `crates/`, `spec/`, `xtask/` or `standards/`
  (`_storymap.md:21-23`).
- Any `.kb/decisions/` atom (AC-A09) and any ADR written as a side effect
  (`project.md:124-127`).
- Incidental bugs found while observing — they route to the `support` initiative
  (`.redkiln/config.yaml:5`).

**Merge DoD one-liner.** Both halves of scenario 2 are in the project ledger as two
self-contained rows naming the observer, the checkout path and the merged sha, each citing
its own verbatim transcript; the working tree is clean of the break; and no commit in this
story's history contains the broken page.

The implementer MAY also touch the ledger file named in the Integration contract in order to
mount these rows — that is the mount, not scope drift.

```
.bklg/docs-that-teach/durable-audience-closeout/scenario-two-fault-injection/**
.bklg/docs-that-teach/durable-audience-closeout/_dod-ledger.md
```

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **B1 — Run in the slice's fresh checkout, never in the planning worktree** | The observation is taken in the checkout `dod-scenario-ledger` created off the **merged** branch. Record its path and the tree sha before the first command. Tier 4 does not start until that checkout exists | `.bklg/docs-that-teach/durable-audience-closeout/_decomposition.md:388-391` (AC-A07), `:789-794` (clean-checkout seam) |
| **B2 — Break one claim, on one page, under the pinned tree** | The edit must make a *claim* about the library untrue — a fence that no longer compiles against it, or a cited clause id that no longer resolves — not a typo, a broken link or a formatting change. One page, one claim, so the failing output is attributable. The pinned tree is `docs` (`const TREE`) and the compiling harness is `xtask/src/narrative.rs` (`const HARNESS`) | `.bklg/docs-that-teach/initiative.md:416-420`; `.bklg/docs-that-teach/checked-documentation-surface/_design.md:93`, `:96` |
| **B3 — Run the gate and capture the failure verbatim** | Run the gate command and capture its output byte-for-byte, including the step banner, the problem line(s) and the terminal failure. Expected shape is `  {path}:{line} — {message}` on stderr under `=== every narrative page is checked ===`, but the *expectation is not the evidence* — what the run printed is | `.bklg/docs-that-teach/checked-documentation-surface/_design.md:37`, `:761`; `_decomposition.md:837-838` (AC-TB-05) |
| **B4 — Record the file the gate actually named** | If the named file is the broken page, say so. If it is the harness, or a different page, the transcript still stands as evidence and the discrepancy is written down as a routed gap for HS-P0020 — the transcript is never edited to agree with the expectation | `.bklg/docs-that-teach/checked-documentation-surface/_storymap.md:55`; `project.md:102-108` |
| **B5 — Revert, and prove the revert** | Restore the page to its committed content, then observe the working tree is clean of the break before anything is committed. The revert is proven by the subsequent green run, and the cleanliness observation is what makes "never a committed state" checkable rather than asserted | `.bklg/docs-that-teach/durable-audience-closeout/_decomposition.md:642-647` |
| **B6 — Re-run the same command and capture green** | The recovery half uses the *same* command as B3 on the reverted tree. A different command, a narrower grain, or a `--fast` run is not a recovery observation of the same thing that failed | `.bklg/docs-that-teach/initiative.md:416-420`; `.redkiln/config.yaml:60` |
| **B7 — Two ledger rows, both self-contained** | Scenario 2's single row becomes two, outcomes `failed by name` and `recovered`. Each carries: scenario id, owning project (HS-P0020), observer, checkout path, tree sha, what was seen, and a citation to its own transcript. Neither row depends on the other to be understood | `.bklg/docs-that-teach/durable-audience-closeout/_decomposition.md:270-277` (AC-UX-10, AC-UX-11), `:833-836` (AC-TB-04) |
| **B8 — No inherited evidence, anywhere** | The string "inherited from a sibling" appears in no outcome cell. HS-P0020's own falsification run may be *cited as context*; it may not be the value of either row's evidence | `project.md:203`; `_decomposition.md:274-277`, `:833-836` |
| **B9 — Accessibility floor on the rows** | No state carried by colour, emoji, glyph, strikethrough, ordering or an empty cell. Every outcome is a word; every "not applicable" is written out | `.bklg/docs-that-teach/durable-audience-closeout/_decomposition.md:266-269` (AC-UX-09) |
| **B10 — The tree is named by sha in both rows** | Both rows name the merged tree by ref or sha, so a later reader can re-run the observation. This is the same rule the post-merge re-checks carry | `.bklg/docs-that-teach/durable-audience-closeout/_decomposition.md:278-281` (AC-UX-12) |
| **B11 — Commit provenance** | `require_commit_provenance: true` binds: the work commit that lands these rows is recorded via `redkiln record-links --sha`. The `redkiln` CLI remains the single writer of item frontmatter — never hand-edit `links` | `.redkiln/config.yaml:69-73`; `CLAUDE.md`, "Where the work lives" |
| **B12 — Sequencing** | Strictly after `dod-scenario-ledger` (the checkout and the table must exist) and strictly before `terminal-gate-run` (which must be taken on a tree already carrying these rows and free of the break) | `_storymap.md:156-160`; `_decomposition.md:642-647` |

**Interfaces consumed, none authored.** This story defines no function, no trait, no type and
no CLI verb. Its interfaces are: the gate command wired at `.redkiln/config.yaml:60` and
defined once in `xtask/src/main.rs`; the ledger row columns fixed by AC-UX-10/AC-TB-04; and
`redkiln record-links --sha` for provenance. Every command it runs already exists in this
tree — AC-TB-02 forbids invented tooling
(`.bklg/docs-that-teach/durable-audience-closeout/_decomposition.md:827-829`).

## Data and migrations

**N/A — no data and no migration.** This project ships no Rust, no schema and no store
(`_storymap.md:21-23`), and `_design.md` records no public API surface and no rendered
surface (`_design.md:10-47`). Nothing here reads or writes an event store, a projection
store or any persisted state; `crates/` is read-only for the whole project
(`_decomposition.md:419-425`).

The only state this story mutates is a **transient working-tree edit** to one page under
`docs/` in a throwaway checkout, and the whole point of B5 is that it is reverted before any
commit and therefore never enters version history. There is no forward migration because
there is nothing persisted to migrate, and no rollback step beyond the revert itself — which
is not a rollback but the second half of the observation.

**Evidence artefact paths are fixed here, not left to taste.** The ledger rows, this story's
`_ledger.md` and the tests below all cite the same three files, so they are named once:

| Artefact | Path | Holds |
| --- | --- | --- |
| Failing transcript | `.bklg/docs-that-teach/durable-audience-closeout/scenario-two-fault-injection/_evidence-failing.md` | The failing run's output, verbatim, in one fenced block, with the command line and the tree sha above it |
| Recovery transcript | `.bklg/docs-that-teach/durable-audience-closeout/scenario-two-fault-injection/_evidence-recovered.md` | The green re-run's output, verbatim, same shape |
| Observation note | `.bklg/docs-that-teach/durable-audience-closeout/scenario-two-fault-injection/_break-note.md` | What was broken and why it is a claim break; the file the gate actually named; the post-revert cleanliness observation; any routed gap |

The filenames are normative so citations are stable; their *content* is the implementer's.
Nothing else in the story's folder is evidence, and a transcript pasted only into a commit
message or a chat transcript is not one of these files.

## Acceptance criteria

Seven criteria. Each is written from the intent of the **closeout reviewer (U2)** — the reader
who has to decide whether this gate was ever actually watched failing — and each crosses the
whole slice: the checkout, the command, the transcript, and the row a later reader lands on
(`_storymap.md:37-39`; `_decomposition.md`, "## UX brief", "Who this is actually for").
Verification names the tier from the testing brief's mapping (`_decomposition.md:853-878`),
because this project has no Rust test to point at and AC-015 is Tier 4 by that table's own
row (`:875`).

| id | criterion | verification |
| --- | --- | --- |
| **AC-001** | GIVEN the closeout reviewer needs to re-run this observation on the same tree the closeout was taken on, WHEN scenario 2's re-observation begins, THEN it runs inside the fresh checkout `dod-scenario-ledger` created off the **merged** branch — never `.claude/worktrees/docs-that-teach` — and that checkout's path and the merged tree sha are recorded before the first command is issued and repeated in both ledger rows. | Tier 4 precondition, then Tier 1 static: `rg -n "<merged-sha>" .bklg/docs-that-teach/durable-audience-closeout/_dod-ledger.md` returns both scenario-2 rows, and the sha resolves with `git rev-parse`. Sha and checkout path also appear at the head of `_evidence-failing.md` and `_evidence-recovered.md`. |
| **AC-002** | GIVEN a reviewer who must be able to tell a real falsification from a typo, WHEN the page under the pinned narrative tree is broken, THEN exactly one claim on exactly one page is made untrue of the library — a fence that no longer compiles against it, or a cited clause id that no longer resolves — with the break, the page and the reason it is a claim break rather than a formatting break written down in `_break-note.md`. | Tier 2 content review of `_break-note.md` against B2 and the pinned constants `const TREE: &str = "docs"` and `const HARNESS: &str = "xtask/src/narrative.rs"` (`.bklg/docs-that-teach/checked-documentation-surface/_design.md:93`, `:96`). A note describing a broken link, a typo or a reflow fails this row. |
| **AC-003** | GIVEN a reviewer who was burned by a step that was vouched for by two documents and printed `skipped` on all three runners (`RUNBOOK.md:918-925`), WHEN the gate is run against the broken tree, THEN the failing half's evidence is the run's output captured **byte-for-byte** into `_evidence-failing.md` — banner, problem line(s) and terminal failure — and `_break-note.md` records the file the gate actually named, even where that is the harness rather than the page, with no word of the transcript reworded to agree with the expectation. | Tier 4 e2e — the run itself — read as Tier 2 content review: `_evidence-failing.md` contains a fenced verbatim block whose problem line matches the shape `  {path}:{line} — {message}` under `=== every narrative page is checked ===` (`.bklg/docs-that-teach/checked-documentation-surface/_design.md:37`, `:761`), and the named file in the transcript equals the file named in `_break-note.md`. A paraphrase, a summary, a screenshot or a re-typed error line fails. |
| **AC-004** | GIVEN the terminal `cargo xtask ci` in `terminal-gate-run` must be taken on a tree that does not contain the break, WHEN the failing transcript has been captured, THEN the page is restored to its committed content, the working tree is observed clean of the break before anything is committed, and no commit in this story's history contains the broken page. | Tier 1 static: `git status --porcelain` recorded in `_break-note.md` showing the page absent from the change set, plus `git log -p -- <broken page>` and `git log -S "<the broken claim string>"` over this story's commit range returning nothing. Falsifier: a commit that adds the break and a later one that removes it. |
| **AC-005** | GIVEN a reviewer who must see the same instrument recover, not a cheaper one pass, WHEN the revert is in place, THEN the **same** command run in AC-003 is re-run on the reverted tree and its green output is captured verbatim into `_evidence-recovered.md` as the recovery half's evidence — not `--fast`, not a narrower grain, not a different command. | Tier 4 e2e. Tier 2 diff of the command line recorded at the head of `_evidence-recovered.md` against the one in `_evidence-failing.md`: they must be character-identical, and both must resolve to a real command in this tree (`.redkiln/config.yaml:60`; `xtask/src/main.rs`) per AC-TB-02 (`_decomposition.md:827-829`). |
| **AC-006** | GIVEN a reviewer reading the DoD ledger one row at a time, WHEN they reach scenario 2, THEN they find **two** named rows — outcome `failed by name` and outcome `recovered` — mounted in the project's fifteen-row re-observation ledger in place of its single placeholder row, each carrying scenario id, owning project (HS-P0020), observer, checkout path, tree sha, what was seen and a link to its own transcript, each understandable without reading the other, and neither row's outcome column containing the string "inherited from a sibling". | Tier 3 mount-point walk of `.bklg/docs-that-teach/durable-audience-closeout/_dod-ledger.md`: scenario 2 has two rows and no unresolved placeholder, and no second scenario-2 record exists elsewhere (`rg -n "scenario 2" .bklg/docs-that-teach/durable-audience-closeout`). Tier 1: `rg -n "inherited from a sibling" .bklg/docs-that-teach/durable-audience-closeout` returns nothing. Tier 2: each row read in isolation against AC-UX-10 (`_decomposition.md:270-273`) and AC-UX-11 (`:274-277`). |
| **AC-007** | GIVEN a reader consuming the ledger as plain text — in a diff, a quote, a terminal or a screen reader — WHEN they read either scenario-2 row, THEN every state it expresses is a written word: no outcome carried by colour, emoji, glyph, strikethrough, row ordering or an empty cell, no "the failing row is the one above the green one", every not-applicable written out, and each row's load-bearing verdict also stated in a sentence outside the table. | Tier 2 content review against AC-UX-09 (`_decomposition.md:266-269`) and IQ-8 (`:221-224`), plus the plain-text-equivalence rule (`:142-145`). Tier 1: `rg -n "[✅❌✓✗~]" ` over the two rows returns nothing, and no cell in either row is empty. |

Coverage of the traced project ACs: **AC-015** (`project.md:204`) is carried by AC-002, AC-003,
AC-004 and AC-005 — the break, the failing record, the revert, the recovery — because AC-015's
own words are "the failing half is recorded, not just the recovery". **AC-014** (`project.md:203`)
is carried by AC-001, AC-006 and AC-007 for scenario 2's share of the ledger: the clean
checkout, the self-contained rows with named observer and tree, and the inadmissibility of
"inherited from a sibling". The other fourteen scenarios are `dod-scenario-ledger`'s
(`_storymap.md:127`).

## Interaction quality

This project renders **no surface**: `_design.md` records `hasSurface: false`, an empty
`## Items` block and `N/A` under every composition heading including `## Anti-patterns`,
signed off by the repository owner on 2026-08-17 with no conditions (`_design.md:10-47`,
`:77-79`, `:85-93`). There is therefore no component library, no token set, no density number
and no chrome decision for this story to honour or contradict — and inventing one would be
the bespoke authoring the UX brief forbids (`_decomposition.md:298-304`).

What replaces them is real and binding: the UX brief's eight interaction-quality invariants
are written as the *text* analogues of the state family (`_decomposition.md:147-224`), and
AC-UX-09…AC-UX-12 are the composition rules for the one artefact this story composes — a
ledger row. Every invariant that applies is carried by an **AC-### row in the table above**;
this section only says which, and how each is caught.

**State family.**

| Invariant (text analogue) | Carried by | Caught by |
| --- | --- | --- |
| **In place, not a context jump** (IQ-1, `:152-157`) | AC-006 | Each row answers *which scenario, run by whom, on what tree, with what outcome* on first read; the link carries the transcript, never one of those four facts. Falsifier: a row whose outcome reads "see the transcript". |
| **Non-occlusion — a summary must not hide what it summarises** (IQ-2, `:159-168`) | AC-006 | Two rows, not one row saying "failed then recovered". The failing half is the one that matters (`initiative.md:416-420`); collapsing it into the recovery row is the occlusion this project's risk table names as "fifteen ticked boxes" (`project.md:249`). |
| **Preserved position** (IQ-3, `:180-187`) | AC-006 | Scenario 2's placeholder row is replaced **in place**, keeping the other fourteen rows' order and any `file:line` a sibling already cited. No renumbering, no reflow of the surrounding table, no `-` line outside the scenario-2 region in `git diff` of the ledger. |
| **Reversibility** (IQ-4, `:189-199`) | AC-004 | The break is reversible by construction and is reverted before any commit; the observation is not undone by the revert but completed by it. Falsifier: a commit pair that adds then removes the break. |
| **Reachable without prior knowledge** (IQ-5, `:201-205`) | AC-006 | A reader starting at the ledger reaches each transcript in one hop, with link text naming the destination ("failing run transcript", not "here"). Neither transcript is findable only by knowing this story's slug. |
| **Preserved selection — prior citations keep meaning** (IQ-6, `:207-212`) | AC-003 | The transcript is the bytes the gate wrote. Rewording it so it names the page a reader expects changes what the citation means, which is the same defect as editing an accepted atom. |
| **The qualification travels with the claim** (IQ-7, `:214-219`) | AC-003 | Where the gate named the harness rather than the page, that discrepancy sits **beside** the transcript citation in `_break-note.md`, not in a trailing caveats block a reviewer scanning outcomes never reaches. |
| **Legible at the moment of reading** (IQ-8, `:221-224`) | AC-007 | No state by presence, absence, ordering or omission. |

**Composition family**, taken from the only composition rules a human has signed off for this
artefact — the ledger row shape in the UX brief, since `_design.md` declares none:

| Composition rule | Real numbers / shape | Carried by |
| --- | --- | --- |
| **Presentation exists at all** — the row is composed from the ledger's existing column vocabulary, not a bespoke shape bolted on | The columns `dod-scenario-ledger` fixed: scenario id, owning project, observer, checkout path, tree sha, outcome, evidence (AC-UX-10, `:270-273`; AC-TB-04, `:833-836`) | AC-006 |
| **Placement** | Inside the fifteen-row table, at scenario 2's existing position — not appended at the end, not in a second table, not in this story's folder alone | AC-006 |
| **Transience** — what is persistent, what is revealed, what is opened on demand | Persistent: the two rows and their outcome words. Opened on demand: the transcripts, one hop away. Never transient: the failing run's bytes, which is why they are a committed file rather than scrollback | AC-003, AC-005, AC-006 |
| **Density budget** | Exactly **two** rows for scenario 2 — not one, not three. Seven fields per row, all populated, none empty. Exactly **one** fenced verbatim block per transcript file, and **zero** occurrences of the string "inherited from a sibling" in the whole project folder | AC-006, AC-007 |
| **Hierarchy** | Outcome word first in the reader's eye, then the tree it was observed on, then the evidence link; the `failed by name` row precedes the `recovered` row in reading order **without** that ordering carrying any state (AC-007 is what stops the ordering becoming the record) | AC-006, AC-007 |
| **Named anti-patterns** | `_design.md` names none (no surface). The binding anti-patterns are the UX brief's own falsifiers: an outcome expressed by a glyph or an empty cell (AC-UX-09), a summary line standing in for a row (IQ-2), a row that needs the row above (AC-UX-10), and "recovered" with no captured failing transcript — explicitly a **review-blocking defect** (`_decomposition.md:801-802`) | AC-006, AC-007, AC-003 |

An unstyled render satisfies every shape assertion here, which is exactly why AC-007 exists:
the machine tiers can confirm seven populated cells and still pass a table whose real verdict
is a green tick and a red cross.

## Error conditions

| id | Condition | Required behaviour |
| --- | --- | --- |
| **EC-001** | The gate fails, but names the **harness** (`xtask/src/narrative.rs`) or a different page rather than the broken page. | Record what was seen. The transcript is never edited to agree with the expectation (AC-003). The row's outcome states the file that was actually named, DoD-2's "by name" claim is **not** recorded as fully green, and the discrepancy is written into `_break-note.md` as a routed gap for HS-P0020 (`project.md:102-108`). HS-P0020's own falsification story anticipates this exact case (`.bklg/docs-that-teach/checked-documentation-surface/_storymap.md:55`). |
| **EC-002** | The gate **passes** with the break in place. | Do not escalate the break until something fails — that manufactures the observation. Stop, restore the page, record the outcome as `did not fail` with the transcript of the passing run, and route it to HS-P0020 as a blind spot. This is a finding about the gate, and DoD-2 is not green. The slice halts loudly rather than proceeding to `terminal-gate-run` with a fabricated row. |
| **EC-003** | The gate is already red on the unbroken checkout. | The observation is void: a failure that predates the break proves nothing about the break. Record the pre-existing failure, halt, and route it — the baseline must be green before AC-002's edit is made, and AC-005's recovery run is meaningless without it. |
| **EC-004** | The gate fails at an earlier step (fmt, clippy, tests, wasm32) and never reaches the narrative check. | That failure is not scenario 2's evidence. Capture the full output, name the step that failed in `_break-note.md`, resolve the earlier failure or route it, and re-run. A transcript that never contains `=== every narrative page is checked ===` does not satisfy AC-003. |
| **EC-005** | The break is found in the working tree, the index or a commit after AC-004's checkpoint. | Blocking. `terminal-gate-run` must not start. Restore, re-observe cleanliness, and if a commit contains it, the story's history is rewritten or the commit reverted **before** the row is claimed — because the row claims it was never committed (`_decomposition.md:642-647`). |
| **EC-006** | The fresh checkout does not exist, or the observation was taken in `.claude/worktrees/docs-that-teach`. | Blocking, and a dependency failure rather than a defect here: `dod-scenario-ledger` owns the checkout (AC-A07, `_decomposition.md:388-391`). Tier 4 does not start until it exists (`:789-794`). Nothing observed in the planning worktree counts (`:306-310`). |
| **EC-007** | The captured evidence is a paraphrase, a truncation, a screenshot, or scrollback that has scrolled. | Not evidence. AC-TB-05 requires the failing run's **captured output** (`:837-838`); re-run and capture properly. Redirecting both streams to a file at the moment of the run is the only reliable route — a terminal buffer is not an artefact. |
| **EC-008** | The ledger file created by `dod-scenario-ledger` is named something other than `_dod-ledger.md`. | Not an error: the mount is the table, not the filename (Integration contract). Mount into the file the slice-mate actually created and correct the path in this spec's Integration contract and in `_ledger.md`. Creating a parallel file **is** the error. |
| **EC-009** | Observing the gate reveals an unrelated defect. | Record it, route it to the `support` initiative (`.redkiln/config.yaml:5`), and do not repair it here — HS-P0020 owns the gate and this story owns only the observation (PR boundary). |

## Non-functional

| id | Requirement | Why it binds |
| --- | --- | --- |
| **NF-001** | The observation costs **two full `cargo xtask ci` runs**, not two `--fast` runs. Budget for the whole gate twice — fmt, clippy, workspace tests, four wasm32 steps, docs, `spec-trace`, plus the tool-dependent steps — and do not substitute a narrower grain to save time. | `.redkiln/config.yaml:60` makes `cargo xtask ci` this project's terminal grain, and AC-005 requires the recovery run to be the same command that failed. `integration_scoped` (`--fast`) is explicitly the bar for **non**-terminal projects (`_decomposition.md:764-768`). |
| **NF-002** | Transcripts are byte-faithful and readable as plain text: no ANSI colour escapes left as the only carrier of a state word, no line wrapping introduced by the capture, one fenced block per file. | AC-UX-09's colour-never-alone floor applies to the evidence as well as the rows (`_decomposition.md:266-269`); a transcript whose failure is legible only in a colour terminal fails the same rule the ledger does. |
| **NF-003** | Both rows and both transcripts name the tree by sha and the checkout by path, so the observation is re-runnable by a later reader without asking anyone. | AC-UX-12 (`_decomposition.md:278-281`); AC-A06/AC-A07 (`:380-391`). This is the property that separates a record from a claim. |
| **NF-004** | Every command run is one that already exists in this tree — `cargo xtask ci`, `git`, `redkiln record-links`. No script, wrapper, alias or helper is invented for this observation. | AC-TB-02 (`_decomposition.md:827-829`); `xtask/src/main.rs` defines the gate once and CI runs exactly it (CLAUDE.md, "Commands"). |
| **NF-005** | The committed diff stays inside the PR boundary: this story's folder plus the project ledger. The `docs/` edit exists only in the working tree of a throwaway checkout and appears in no diff. | `_storymap.md:21-23`; AC-004. A `docs/` line in `git diff` at merge is a scope escape, not a detail. |
| **NF-006** | The work commit is recorded through `redkiln record-links --sha`; item frontmatter is never hand-edited. | `require_commit_provenance: true` (`.redkiln/config.yaml:73`); the single-writer rule (CLAUDE.md, "Where the work lives"). |

## Implementation notes (non-prescriptive)

Shape, not instructions — the implementer owns the sequence as long as AC-004 holds.

- **Order that keeps AC-004 free.** Confirm the checkout and record the sha → run the gate
  once on the unbroken tree if a green baseline is not already recorded by
  `dod-scenario-ledger` (EC-003) → break → capture → revert → observe cleanliness → capture
  the green run → write the two rows and the note → commit. The break is never staged with
  `git add`, so no accidental `commit -a` can carry it.
- **Choosing the break.** The strongest claim break is one the checker was built to catch:
  a fenced example under the pinned tree that no longer compiles against the library, or a
  cited clause id edited to one that does not resolve. Both are single-line edits with a
  single-line revert, which is what keeps AC-004 cheap. A typo, a broken link or a heading
  change is not a claim (AC-002).
- **Capture, don't scrape.** Redirect the run's stdout and stderr to a file at the moment of
  the run and paste that file's contents into the transcript artefact. Scrollback is where
  EC-007 comes from.
- **The two rows can be drafted before the runs**, but the outcome and evidence cells stay
  empty until the transcripts exist — and an empty cell must never be committed, because
  AC-007 makes an empty cell a state expressed by omission.
- **Expect the harness case.** HS-P0020 wrote its own falsification story specifically to
  record which file the gate names, because a fence compiled through
  `xtask/src/narrative.rs` may attribute the failure there. If that happens, EC-001 is the
  path — it is a normal outcome of an honest observation, not a failure of this story.
- **This story is where the initiative's own precedent gets tested.** `RUNBOOK.md:918-925`
  is the in-house case of a step that was wired, documented twice and silently skipped. The
  two artefacts this story produces are the reason that cannot happen to DoD-2.

## Tests and CI (merge gate)

Grounded in the testing brief's four tiers (`_decomposition.md:704-711`), its merge-gate
command order (`:734-762`) and its AC-###→tier mapping, which classifies both AC-014 and
AC-015 as **Tier 4 with no secondary tier** (`:874-875`) — because the only instrument that
can catch a fabricated observation is a human reading a captured transcript.

| tier | command / path | proves |
| --- | --- | --- |
| **Story grain (always)** | `cargo xtask affected --base main` (`.redkiln/config.yaml:40`) | The diff maps to no workspace package, so this runs the five file-reading lints and `spec-trace` unconditionally — the guard that a documentation-shaped story is not "green by compiling nothing" (`.redkiln/config.yaml:36-39`). Proves NF-005: nothing under `crates/`, `spec/`, `xtask/` or `standards/` was touched. |
| **1 — static / shape** | `rg -n "inherited from a sibling" .bklg/docs-that-teach/durable-audience-closeout` → no output; `rg -n "<merged-sha>" .bklg/docs-that-teach/durable-audience-closeout/_dod-ledger.md` → both rows; `test -f` over `_evidence-failing.md`, `_evidence-recovered.md`, `_break-note.md`; `git status --porcelain`; `git log -p -- <broken page>` over this story's range | AC-001, AC-004, AC-006 (the forbidden-string half), AC-007 (empty-cell and glyph scans) |
| **2 — content review** (the "unit" analogue, run against the UX brief's own checklist per AC-TB-06, `:840-842`) | A human reading `_break-note.md`, `_evidence-failing.md` and each of the two rows **in isolation**, against IQ-1…IQ-8 (`:147-224`) and AC-UX-09/10/11 (`:266-277`) | AC-002, AC-003, AC-006 (self-containment), AC-007. This is the only tier that can reject a well-shaped row whose transcript is a paraphrase |
| **3 — integration / mount-point** | `cargo xtask lints && cargo xtask spec-trace` (`reachability_static`, `.redkiln/config.yaml:48`), plus a manual walk: open `.bklg/docs-that-teach/durable-audience-closeout/_dod-ledger.md`, find scenario 2, follow each row's evidence link | AC-006 — the rows are mounted in the real ledger, no placeholder row survives, no second scenario-2 record exists, and both transcripts are reachable in one hop |
| **4 — e2e / whole-tree** | The two runs of `cargo xtask ci` (`.redkiln/config.yaml:60`) that **are** the observation: the first on the broken tree, expected to fail; the second on the reverted tree, expected green. Their outputs are the artefacts, not a side effect | AC-003, AC-005 — and AC-002 negatively, since a break the gate cannot see fails EC-002 rather than passing quietly |
| **Ledger gate** | `redkiln verify --grain story` over `.bklg/docs-that-teach/durable-audience-closeout/scenario-two-fault-injection/_ledger.md` (`require_ledger: true`, `.redkiln/config.yaml:67`) | Every AC-001…AC-007 row is present, `satisfied: true`, and cites non-placeholder evidence — blocking `implement → report` otherwise |
| **Provenance** | `redkiln record-links HS-S0181 --sha <commit>` (`require_commit_provenance: true`, `.redkiln/config.yaml:73`) | The work commit is on the item; frontmatter written only by the CLI |

**Not this story's gate.** The green run in AC-005 is the recovery half's evidence for one
scenario. It is **not** AC-016's evidence and must not be presented as it: AC-TB-03 requires
the AC-016 run to be taken last, on the tree already carrying the atoms, maps, `links.kb`,
the reconciliation record and the full ledger (`_decomposition.md:830-832`). That run is
`terminal-gate-run`'s (HS-S0182).

## Risks and coupling (PR-scoped)

| Risk | Shape it takes here | Mitigation in this spec |
| --- | --- | --- |
| **The break survives into history** | A `commit -a` sweeps the broken page in, or the revert lands in a later commit — so the record "never a committed state" is false in the same PR that asserts it | AC-004 is a first-class criterion with a `git log -S` falsifier, not a tidiness note; EC-005 makes it blocking; the implementation note keeps the break unstaged (`_decomposition.md:642-647`) |
| **The row becomes a tick** | "Scenario 2 — recovered ✅" with no failing transcript: the project risk table's named failure mode (`project.md:249`) | AC-003 and AC-006 split the row in two and require the bytes; "recovered" without a captured failing transcript is already a review-blocking defect (`_decomposition.md:801-802`) |
| **Sibling evidence gets reused** | HS-P0020's `observed-failure-falsification` already did this once, on its own tree; citing it is one keystroke cheaper than running it | AC-006 forbids the string outright; D1 states the run may be cited as context and never as the value; Tier 1 greps the whole project folder |
| **Coupling to `dod-scenario-ledger`'s row schema** | The seven columns and the ledger's filename are decided by the upstream story, mid-slice | The Integration contract makes the **table** the mount and names the filename as a default with an explicit fallback (EC-008); both stories are implemented in one context by construction (`_storymap.md:76-78`) |
| **Coupling to HS-P0020's message format** | The problem-line shape, the banner text and the pinned constants are HS-P0020's to change; a change would invalidate the expectation in AC-003's verification | The expectation is stated as a *shape to compare against*, never as the evidence (B3). A mismatch is EC-001 — recorded and routed, not repaired here |
| **The gate is slow, and the temptation is `--fast`** | Two whole-gate runs on a fresh checkout, cold caches | NF-001 states the cost up front and AC-005 makes command identity checkable by diffing two recorded command lines |
| **Blocking `terminal-gate-run`** | HS-S0182 cannot start until this story's rows are committed and the tree is clean | The dependency is declared both ways below; EC-005 and EC-002 halt loudly rather than letting the terminal gate proceed on a compromised tree |
| **Scope creep into the gate** | Observing a defect and fixing it in the same pass feels efficient and destroys the observation | PR boundary and EC-009: findings route to HS-P0020 or the `support` initiative (`.redkiln/config.yaml:5`) |

## Dependencies

**Blocks on**

- `dod-scenario-ledger` (HS-S0180) — creates the fresh checkout off the merged branch, lands
  the fifteen-row ledger and fixes the row schema this story writes into. Strictly upstream:
  Tier 4 does not start until that checkout exists (`_decomposition.md:789-794`;
  `_storymap.md:156-160`).

**Unlocks**

- `terminal-gate-run` (HS-S0182) — must be taken on a tree that already carries these two
  rows and their transcripts and is free of the break (ordering 4, `_decomposition.md:642-647`;
  AC-TB-03, `:830-832`). It is last in the project by construction.

Transitively, the whole `dod-reobservation` milestone sits behind `product-layer-mounting`
and `post-merge-clause-completeness` (`_storymap.md:56`), but those are `dod-scenario-ledger`'s
edges, not this story's.

## Anchors (progressive disclosure)

Linked, not pasted. The Context pack above is sufficient to start; open these at the moment
named.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `.bklg/docs-that-teach/durable-audience-closeout/_decomposition.md` | Carries all four briefs: the fault-injection seam (`:795-802`), the clean-checkout seam (`:789-794`), the mutation ordering (`:642-647`), AC-UX-09…AC-UX-12 (`:266-281`), AC-TB-04/05 (`:833-839`) and the tier mapping (`:853-878`). Every normative claim in this spec resolves here | Before AC-002 — read `:795-802` and `:642-647` together before making the edit | AC-002, AC-003, AC-004, AC-006, AC-007 |
| `.bklg/docs-that-teach/checked-documentation-surface/_design.md` | The gate under observation: the pinned tree `const TREE = "docs"` (`:93`), the harness `const HARNESS` (`:96`), the problem-line shape (`:37`) and the banner claim sentence (`:761`). Tells you what the failing output should look like — and therefore when EC-001 has fired | Before running the gate for AC-003, to know what to compare the transcript against | AC-002, AC-003 |
| `.bklg/docs-that-teach/checked-documentation-surface/_storymap.md` | Row `:55` is HS-P0020's own falsification story, written to record "which file it actually names". It is the precedent for EC-001 and the run that must **not** be reused as this story's evidence | When the gate names something unexpected, and again before writing either row's evidence cell | AC-003, AC-006 |
| `.bklg/docs-that-teach/durable-audience-closeout/dod-scenario-ledger/spec.md` | The upstream story's spec fixes the checkout, the ledger filename and the seven-column row schema this story writes into. It is the authority when EC-008 fires | Before writing the two rows — read its Integration contract for the real ledger path | AC-001, AC-006 |
| `.bklg/docs-that-teach/initiative.md` | DoD-2 in the initiative's own words at `:415-419` — "both halves are observed; the failing half is the one that matters". The clause this story is measured against, and the one a reviewer will quote | At the start, and again when writing the outcome words | AC-003, AC-005 |
| `.bklg/docs-that-teach/durable-audience-closeout/project.md` | AC-014 (`:203`), AC-015 (`:204`), DR-11 (`:169`) and the risk row that names this story's failure mode, "fifteen re-observations become fifteen ticked boxes" (`:249`) | Before drafting the rows, to check the wording of the obligation rather than this spec's paraphrase | AC-004, AC-006, AC-007 |
| `.bklg/docs-that-teach/durable-audience-closeout/_design.md` | The signed-off design gate for this project: `hasSurface: false`, empty `## Items`, `N/A` under every composition heading, approved 2026-08-17 with no conditions (`:10-47`, `:85-93`) | Only if a composition question arises — it exists to close the question, not to answer it | AC-006, AC-007 |
| `.redkiln/config.yaml` | The grains this story is actually held to: `affected_gate` (`:40`), `reachability_static` (`:48`), `e2e: cargo xtask ci` (`:60`), `require_ledger` (`:67`), `require_commit_provenance` (`:73`) | Before the first gate run, and again before the work commit | AC-005, AC-004 |
| `xtask/src/main.rs` | Defines `cargo xtask ci` once — the exact step list, and therefore which step a failure came from when EC-004 fires | Only when the failing transcript names a step other than the narrative check | AC-003 |
| `RUNBOOK.md` | `:918-925` is the in-house precedent that made DoD-2 exist: a step wired, vouched for by two documents, printing `skipped` on all three runners. It is why a transcript rather than a claim is required | When tempted to record "gate failed" without the bytes | AC-003 |
| `.kb/governance/rewrite-the-referent-never-the-reasoning.md` | The governance atom behind D8 and IQ-4: a record found wrong is superseded, never edited. Binds the transcript as much as an atom | If the urge to "clean up" a transcript or an already-written row appears | AC-003 |
| `.bklg/docs-that-teach/durable-audience-closeout/_grounding.md` | The tensions carried into this project, including T2 — that this planning worktree cannot supply the clean checkout | If EC-006 fires and the checkout's provenance is in doubt | AC-001 |

## Clarifications resolved during spec

1. **The seven AC ids are unchanged from the front half.** AC-001…AC-007 as enumerated there;
   none added, none dropped. `_ledger.md` carries exactly these seven.
2. **AC-004 was kept separate from AC-002 rather than folded in.** "Break a claim" and "the
   break never enters history" are falsified by different instruments — a content review and
   a `git log -S` — and the architecture brief treats the second as an ordering constraint on
   the whole slice (`_decomposition.md:642-647`), not as hygiene attached to the first.
3. **"Fails by name" is recorded, not asserted.** Where the gate names the harness or another
   page, EC-001 fires: the transcript stands, the row records the file actually named, and
   DoD-2's "by name" half is flagged as a routed gap for HS-P0020 rather than claimed green.
   The alternative — reword until it matches — was rejected because it is the exact defect
   `RUNBOOK.md:918-925` is the precedent for.
4. **The green re-run is scenario 2's recovery evidence and nothing else.** It is explicitly
   not AC-016's, which AC-TB-03 requires to be the last run on the fully assembled tree
   (`_decomposition.md:830-832`). Two runs of the same command with different meanings is a
   real trap and is named in the PR boundary as well as here.
5. **The mount is the ledger table, not the filename.** `_dod-ledger.md` is this spec's
   default because no artefact in the tree fixes the name yet; EC-008 makes the slice-mate's
   actual filename authoritative and forbids a parallel file. Both stories land in one
   implementer context, so the divergence is detectable in the same pass.
6. **Evidence artefact filenames were promoted to normative.** `_evidence-failing.md`,
   `_evidence-recovered.md` and `_break-note.md` are fixed so the rows, this spec and
   `_ledger.md` cite stable paths; leaving them to taste would have produced a ledger row
   citing a file the gate's own verifier cannot resolve.
7. **No design composition was re-decided.** `_design.md` declares `hasSurface: false` with
   sign-off and no conditions, so the Interaction quality section draws its composition
   family from AC-UX-09…AC-UX-11 — the only row-shape rules a human has approved for this
   artefact — rather than inventing a density budget or an anti-pattern list this project
   never had.
8. **No decision atom is cited or written.** None of the seventeen governs documentation
   structure or observation practice (`_decomposition.md:285-296`); D8 records that as a
   finding. Authoring one here is forbidden by AC-A09 and by the standing rule that an ADR is
   never a side effect.
9. **DoD-2's exact line range is `initiative.md:415-419`**, verified by reading the file. The
   Scope lock and Context pack above cite `:416-420`, one line low at both ends; the clause
   text is identical either way and the front half is left as written rather than edited, but
   the range in the Anchors table is the checked one. Cite `:415-419` in the ledger rows.
