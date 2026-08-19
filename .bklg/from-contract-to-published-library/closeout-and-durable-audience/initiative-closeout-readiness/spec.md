---
item: HS-S0135
stage: spec
created: 2026-08-12T13:48:14.313Z
updated: 2026-08-12T13:48:14.313Z
template_sig: 87bbf1d0
rendered_sig: "4e611637"
---

# Spec — No open child of HS-I0006, and the closeout links the promoted atoms

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` — DoD 16 (`:405-407`), exit criteria 7–8 (`:578-581`), *Referenced personas & journeys* (`:227-258`) |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` — children HS-P0010…HS-P0019 (`:25-34`), the DAG, *Decisions taken at the gate* |
| Project | `.bklg/from-contract-to-published-library/closeout-and-durable-audience/project.md` — AC-014, DR-13, DoD 7 |
| This spec | `.bklg/from-contract-to-published-library/closeout-and-durable-audience/initiative-closeout-readiness/spec.md` |
| Key briefs | `.../closeout-and-durable-audience/_decomposition.md` (the one warranted `testing` brief; AC-014 row and the amended evaluator decision), `.../_grounding.md` (what exists on disk), `.../_design.md` (**no public API surface**, approved 2026-08-12) |
| Story map row | `.../closeout-and-durable-audience/_storymap.md:63` — slice `closeout-health-and-disposition`, `depends_on` `product-atom-promotion-via-kb-ingest`, `findings-disposition-register`; `:129` — AC-014 coverage row |
| Roadmap pointer | `RUNBOOK.md` — the plan of record; this story is the last leaf of the DAG sink (rank 6) |

## One-line PR slice

Show the initiative can close: every sibling HS-P0010…HS-P0018 closed out with no open child of HS-I0006 under `redkiln status`, and the initiative's own closeout linking the promoted product atoms.

## Executive summary

This PR lands the **closeout-readiness section** of `_closeout-record.md` — the last section written into that artefact — and the one durable state change that makes DoD 16's second clause an on-disk fact: `links.kb` on the HS-I0006 item, carrying the ids of the product atoms its slice-mate promoted.

Delta against what the project already says: `project.md` AC-014 asserts the end state ("all nine sibling projects are closed out and the initiative's closeout links the promoted product atoms, evidenced by `redkiln status` showing no open child of HS-I0006"). This spec resolves the one thing that assertion cannot be taken literally on — **HS-P0019 is itself an open child while this story runs** — and converts the rest into two observations and one recorded command:

1. a **census** of every child and grandchild of HS-I0006 taken from `redkiln status --json` on the closeout tree, with the nine siblings shown at `status: done` / `stage: closeout` and this project named as the single declared residual with its remaining transitions enumerated; and
2. the **linkage**, performed by `redkiln record-links HS-I0006 --atom <ids>` over the atom ids read from `.kb/product/` on disk — dry-run first, then for real, then re-read and quoted.

The deliverable is a **verdict**, not a green light. READY and NOT-READY are both valid outcomes and both ship; what is not permitted is closing something, advancing something, or authoring an atom in order to make the verdict come out green.

## Context pack

**The decisions this story must honour, stated as decisions.** Everything deeper is a signposted anchor (second pass); nothing below needs another file to be actionable.

**1. This story proves readiness; it does not close anything.** Redkiln's CLI is the only writer of an item's system frontmatter and the orchestrating command owns every state transition (`CLAUDE.md` *Where the work lives*). `redkiln advance` and `redkiln new` are not run here, by anyone, for any reason. The stage transitions that actually close HS-I0006 — `closure` (`status_on_enter: closeout`, produces `closure.md`, `harvest_kb: true`) and `retrospective` (`status_on_enter: archived`) at `.redkiln/processes/initiative.yaml:32-41` — belong to `/redkiln:closeout`. What this story owes that command is a **preflight that will pass**: it audits the same predicates (every project at review-approved/closeout, every story at report-approved/closeout, tree clean) and records the answer where a human can read it before authorising the run.

**2. "No open child" is not literally observable from inside the terminal project — say so rather than round it off.** HS-P0019 is a child of HS-I0006 and is `in-review`/`design` while this story executes; its own stories are still advancing. A record that prints "zero open children" while the project writing it is open is the failure mode this story exists to avoid. The honest predicate this story observes is: **the nine siblings HS-P0010…HS-P0018 are each at `status: done` / `stage: closeout`, and HS-P0019 is the sole residual**, named, with the exact remaining transitions listed (its remaining stories → `report` → `closeout`; then the project's `review` → `closeout`, `.redkiln/processes/project.yaml:71-80` and `.redkiln/processes/story.yaml:33-41`). AC-014's full sentence becomes true one advance after this story merges, and the record says which advance.

**3. The linkage mechanism is `redkiln record-links`, and it is a provenance write, not a transition.** `redkiln record-links <id> --atom <ids>` records the `.kb` atom ids produced for an item; values are **appended and de-duplicated**, so the later `/redkiln:closeout` run re-recording them is a no-op rather than a conflict. This is the only sanctioned way to fill `links.kb` on `.bklg/from-contract-to-published-library/initiative.md`, whose frontmatter today reads `kb: []` (`initiative.md:21-24`). Running it here — after a `--dry-run` that proves the ids resolve and writes nothing — is what turns DoD 16's "and the initiative's closeout links them" from a promise deferred to a command outside the backlog into a fact this story can be held to.

**4. The promoted atom set is counted from disk, never from memory, and never authored here.** The amended evaluator decision (`.../closeout-and-durable-audience/_decomposition.md:150-242`) governs: **three** persona atoms (application author, adapter author, local-first/edge developer) plus **four** journey atoms — the three matching journeys and the evaluation path as a first-class journey of its own linked to the application-author persona. Seven atoms; exactly three of them personas. This story enumerates what `product-atom-promotion-via-kb-ingest` actually landed under `.kb/product/` and links that. A set that does not match seven-and-three is a **finding routed to `findings-disposition-register`**, not something this story corrects — and under no circumstance is a missing atom hand-written, which is precisely what commit `0269720` was reverted for ("the directory layout of the process without the process"; `project.md` DR-8, risk table row 4).

**5. No story in this project fixes what it finds.** `_storymap.md:92-94` (*Grain notes*): "A story that repairs a defect it found has failed its own acceptance criterion." If a sibling project is not closed, if a story is stuck at `report`, if an atom id will not resolve — the verdict is NOT-READY with the residual named and its owner named, and the residual routes through the slice-mate register per `project.md` DR-12 (`support` → `.redkiln/config.yaml:5`; a cross-project interaction → a new item against the owning sibling; anything touching a `[FROZEN]` clause → a decision atom plus a re-plan).

**6. The evidence converges on one artefact.** `_storymap.md:25-30`: the stories that must be read together append to `_closeout-record.md` in the project directory — "fourteen ledger entries in fourteen places is the failure mode the charter's DoD preamble is written against." That file is stood up by `clean-checkout-harness` (slice 1, foundation) and is this story's mount point. It is **appended to, never created here**: if it is absent, that is a missing dependency and a loud halt, not a licence to write a private artefact beside it.

**7. The seam with the two slice-mates is sharp, and crossing it duplicates work that is already owned.** `backlog-and-kb-health-at-closeout` owns *health* — `redkiln validate`, `validate --kb`, `doctor --json`, and the exactly-six `template-drift` set asserted at `.github/workflows/ci.yml:177-186` (AC-011/AC-012). `findings-disposition-register` owns *routing* (AC-013). This story owns *closure state and linkage* (AC-014) and cites the other two rather than re-running them. It never runs `redkiln adopt --templates` (`CLAUDE.md`).

**8. The persona-journey slice this realises is the inheritance moment.** `.kb/product/README.md:15-21` states that where an initiative's discovery produced personas not yet promoted, "`/redkiln:closeout` performs that promotion, so the next initiative inherits them instead of inventing a fresh set from the same evidence." The reader served by this story is the **next initiative's planner**, who opens the closed HS-I0006 item, follows `links.kb`, and arrives at an adjudicated audience — with the secondary-evidence qualification attached (`initiative.md:256-258`) — instead of re-deriving four personas from the same download counts and issue threads. An atom that exists but that the closed initiative does not point at leaves that reader exactly where they started.

**9. There is no surface.** `_design.md` records **no public API surface** for this project, approved 2026-08-12, with `design.capture` a declared skip (`.redkiln/config.yaml:75-82` — "there is no app to screenshot"). This story adds, changes and removes zero items in any crate. It writes no Rust, so `CLAUDE.md`'s binding constraints are not exercised by it — and it must not become the place someone quietly exercises them.

## Integration contract

**Archetype**: `capability` — the terminal user-observable slice of the terminal project. Its user is a reader, and what they observe is a verdict they can act on.

**Slice / milestone**: `closeout-health-and-disposition`. Slice-mates, implemented in one context and mounted as one integrated surface: `backlog-and-kb-health-at-closeout`, `findings-disposition-register`, `initiative-closeout-readiness` (this story). Within the slice the order is fixed by `_storymap.md:152-155`: health → findings → readiness.

**Mount point**: `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md` — the project's single composition root for closeout evidence, stood up by `clean-checkout-harness` and appended to by every story in the project (`_storymap.md:25-30`). This story appends the final section, `## Closeout readiness (AC-014)`, positioned after the findings register's section so a reader meets the verdict last. Delivery is **mounted**: a readiness verdict that lives only in this story's `_ledger.md` has not been delivered.

**Wires into** (real sibling contracts consumed, by path):

- `redkiln status --json` — the census source. Emits a flat `items[]` array carrying `id`, `type`, `parent`, `status`, `stage`, `path`, `terminal`, `blockedBy`, plus an `index` object whose `branch` field records which tree the state was resolved on. Plain `--json`, **not** `--all-branches`: the question is whether the assembled closeout tree can close, not whether some branch somewhere holds a closed copy.
- `redkiln record-links <id> [--atom <ids>] [--dry-run]` — the linkage writer, and the only sanctioned writer of `links.kb`.
- `.bklg/from-contract-to-published-library/initiative.md` — the HS-I0006 item whose `links.kb` this story fills (`:21-24`, today `kb: []`).
- `.kb/product/` — the atoms promoted by `product-atom-promotion-via-kb-ingest`, read for their `id` frontmatter field (shape as at `.kb/decisions/0006-bare-name-to-the-typed-layer.md:1-8`; kinds and tier per `.kb/product/README.md:6-9`).
- `.redkiln/processes/initiative.yaml:32-41`, `project.yaml:71-80`, `story.yaml:33-41` — the stage/status definitions the closure predicates are written against, so "closed out" means what the process pack says it means rather than what a reader assumes.
- The two slice-mates' sections of `_closeout-record.md` — cited, not re-derived.

**Renders surfaces**: **none.** `_design.md` declares no public API surface for this project and no item ids exist in its `## Items` block (`_design.md:41-45`). Nothing here is rendered to a screen and no Rust signature changes.

**Conformance rule(s)**: none, and the reason is structural — this story touches no port, no adapter and no crate, so there is nothing for `happenstance_testkit` to observe. Adding a conformance rule for a backlog-state predicate would be a rule no adapter can fail (`CLAUDE.md` *The rule that matters*, first corollary).

**Clause(s)**: none. No `spec/SPECIFICATION.md` clause is discharged or amended, and no `[FROZEN]` clause is touched.

**Advances DoD scenario**: initiative **DoD 16** — "The audience is durable. Persona and journey atoms exist under `.kb/product/` with valid frontmatter and pass validation, **and the initiative's closeout links them**" (`initiative.md:405-407`). The first clause is the slice-mate `product-atom-promotion-via-kb-ingest`'s; **this story is the linkage clause**. It also carries project **DoD 7** (`project.md:264-265`) and initiative **exit criterion 8** (`initiative.md:580-581`), and reports — without owning — exit criterion 7's second half (`validate --kb` and `doctor` clean, owned by `backlog-and-kb-health-at-closeout`).

## PR boundary

`redkiln verify --grain story` reads the first fenced block under this heading and fails on any file changed outside it.

```
.bklg/from-contract-to-published-library/closeout-and-durable-audience/initiative-closeout-readiness/**
.bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md
.bklg/from-contract-to-published-library/initiative.md
```

**In this PR**

- The `## Closeout readiness (AC-014)` section appended to `_closeout-record.md`: the child census, the story-grain census, the residual declaration, the atom link manifest, the exit-criteria answer sheet, and the READY / NOT-READY verdict.
- The one durable state change: `links.kb` on `.bklg/from-contract-to-published-library/initiative.md`, written **only** by `redkiln record-links HS-I0006 --atom <ids>`. The third glob above is the deliberate widening this requires — narrower than the file, since only the CLI writes the four lines it touches, and it is named here rather than discovered at verify time.
- This story's own `_ledger.md` and `implementation-report.md` under its item folder.

**Explicitly not in this PR**

- Any `redkiln advance` or `redkiln new`, on any item, including this story and this project. The command owns every transition.
- Authoring, editing or moving any `.kb/` atom — including a missing product atom. That is the slice-mate's, through the ingest path (DR-8).
- `redkiln validate`, `validate --kb`, `doctor --json` assertions and the six-file `template-drift` set — `backlog-and-kb-health-at-closeout` (AC-011/AC-012). Cited here, not re-run as evidence.
- Routing decisions for findings — `findings-disposition-register` (AC-013). This story hands it residuals; it does not choose their destinations.
- Anything `/redkiln:closeout` does: the KB harvest, the `_archive/<slug>/` reference reconciliation, `_retrospective.md`, `closure.md`, and the release plan.
- `redkiln adopt --templates`. Never (`CLAUDE.md`).
- Any change to a crate, `spec/SPECIFICATION.md`, `xtask/`, or CI.

**Merge DoD**: `_closeout-record.md` carries a readiness section whose census was taken by `redkiln status --json` on the closeout tree, whose verdict is READY or NOT-READY with every residual named and owned, and `redkiln record-links HS-I0006 --atom <ids>` has run so the initiative item's `links.kb` lists the promoted product atoms — with no item advanced and nothing fixed.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **The census is taken on the closeout tree, once, and its provenance is recorded** | `redkiln status --json` is run on the same checkout AC-001 was proven on (the harness's residue-free tree), not on a warm planning worktree. The record quotes the run's `index.branch`, `index.builtAt` and the tree's commit SHA, so a reader can tell *which* tree was censused. `--all-branches` is not used: a sibling shown closed on another branch is not a sibling closed on the tree that closes. | `.../closeout-and-durable-audience/_closeout-record.md` (readiness section, *Census provenance*); the harness's section for the SHA |
| **Ten rows, one per child of HS-I0006** | The census table lists every item with `parent: HS-I0006` — HS-P0010…HS-P0019 — with `id`, `slug`, `status`, `stage` and `path` taken verbatim from the JSON. Ten rows, matching `_decomposition.md:25-34`'s children table; a count other than ten is itself a finding (an unrecorded project, or a project moved out of the initiative). | `.bklg/from-contract-to-published-library/_decomposition.md:25-34`; `redkiln status --json` output captured in the record |
| **The sibling closure predicate is the process pack's, not a paraphrase** | Each of HS-P0010…HS-P0018 must read `status: done` **and** `stage: closeout` — the terminal project stage at `.redkiln/processes/project.yaml:76-80`. A project at `review` with an approved verdict but no closeout advance is **not** closed and is reported as a residual. | `.redkiln/processes/project.yaml:71-80` |
| **The story-grain census runs too, because the closeout preflight audits it** | Every item with `type: story` under HS-P0010…HS-P0018 must read `status: done` / `stage: closeout` (`.redkiln/processes/story.yaml:38-41`). Reported as a count per project plus an itemised list of every story not there, with its id, project and current stage. A project marked closed above a story still at `report` is exactly the inconsistency this row exists to catch. | `.redkiln/processes/story.yaml:33-41`; the record's *Story census* table |
| **HS-P0019's own residual is declared, never netted out** | The record states plainly that HS-P0019 is open at the moment of writing, lists its own stories with their stages, and enumerates the remaining transitions in order: this story's `report` (human verdict) → `closeout`, then the project's `review` → `closeout`. It names the advance after which AC-014's full sentence becomes true, and attributes that advance to the orchestrating command. | The record's *The declared residual* subsection |
| **The atom set is read from disk and checked against the amended decision** | Every atom under `.kb/product/` (excluding `README.md`) is listed with its file path, frontmatter `id`, `kind` and `authority_tier`. The expected shape is three `kind: concept` personas and four `kind: playbook` journeys, all `authority_tier: product` (`.kb/product/README.md:6-9`; amended DR-10). The record states observed-vs-expected as two counts, computed, not asserted. | `.kb/product/`; `.../closeout-and-durable-audience/_decomposition.md:150-242`; `.kb/product/README.md:6-9` |
| **A mismatched atom set is routed, not repaired** | Missing, extra or mis-kinded atoms produce a finding row handed to `findings-disposition-register` with the owning story named (`product-atom-promotion-via-kb-ingest`), and the verdict goes NOT-READY. No atom is hand-authored, moved or edited here under any circumstance. | `_storymap.md:92-97`; `project.md` DR-8 and risk table row 4; the findings register's section of the record |
| **The linkage is proved before it is performed** | `redkiln record-links HS-I0006 --atom <ids> --dry-run` runs first and its output is captured: it validates and reports, writing nothing. Only on a clean dry run does the real invocation follow. The ids are comma-separated in one flag — repeated `--atom` flags are last-wins under strict `parseArgs` and would silently record a single value. | `redkiln record-links --help`; the record's *Link manifest* |
| **The linkage is verified by re-reading the item, not by trusting the exit code** | After the real run, `.bklg/from-contract-to-published-library/initiative.md`'s `links.kb` list is re-read and quoted verbatim into the record, and its members are compared to the atom-set table above. Re-running is safe: values are appended and de-duplicated, so the later `/redkiln:closeout` run recording the same ids is a no-op. | `.bklg/from-contract-to-published-library/initiative.md:21-24`; `redkiln record-links --help` |
| **The exit-criteria answer sheet says which halves this story owns** | One row each for initiative exit criteria 7 and 8 and for DoD 16. DoD 16 clause 1 → `product-atom-promotion-via-kb-ingest`; DoD 16 clause 2 → this story. Exit criterion 7's `validate --kb` / `doctor` half → `backlog-and-kb-health-at-closeout`. Exit criterion 8's "whole gate green" half → `whole-gate-green-on-the-assembled-tree` and `published-tree-delta-statement`; its "harvested rather than left in the backlog" half → `/redkiln:closeout`'s `closure` and `retrospective` stages, out of scope here. Each row cites the artefact, never a memory. | `initiative.md:405-407`, `:578-581`; `.redkiln/processes/initiative.yaml:32-41` |
| **The verdict is a first-class outcome with two legal values** | The section ends with `READY` or `NOT-READY`, a one-line reason, and — when NOT-READY — a residual table with three columns — the residual, the item that owns it, and where it is routed. NOT-READY ships and merges. Lowering a predicate, closing a sibling, or authoring an atom to reach READY is a failed story, not a completed one. | The record's *Verdict*; `_storymap.md:92-94` |
| **The non-actions are recorded as observations, not promised in prose** | The record states that no `redkiln advance`, `redkiln new` or `redkiln adopt --templates` was run in this story, evidenced by the story's commit range touching only the three globs in the PR boundary — the same commit-provenance mechanism `.redkiln/config.yaml:73` already requires. | `.redkiln/config.yaml:69-73`; this story's `_ledger.md` and recorded work SHAs |
| **The seam with the slice-mates is cited, not re-run** | The readiness section links the health section (validate/doctor/drift, AC-011/AC-012) and the findings section (AC-013) rather than repeating their commands. Re-running `doctor --json` here would produce a second, separately-timed answer to a question another story already owns — the fourteen-entries-in-fourteen-places failure mode at one remove. | `_storymap.md:25-30`; `.github/workflows/ci.yml:177-186` (the six-file set the health story asserts) |

## Data and migrations

**No schema, database or file-format migration.** This story writes no Rust, no SQL and no migration script; the project it belongs to owns no code at all (`project.md` *Out of scope*: "Design of anything"; `_design.md`: no public API surface).

One durable state change exists and is worth naming precisely, because it is the only frontmatter this story causes to change:

| Field | Item | Before | After | Written by | Reversal |
| --- | --- | --- | --- | --- | --- |
| `links.kb` | HS-I0006 (`.bklg/from-contract-to-published-library/initiative.md:21-24`) | `kb: []` | the ids of the promoted `.kb/product/` atoms | `redkiln record-links HS-I0006 --atom <ids>` — never a hand edit; a `PreToolUse` hook denies the edit (`CLAUDE.md`) | `redkiln record-links HS-I0006 --atom <ids> --remove` |

Three properties of that write matter to the implementer:

- **Append-and-dedupe.** Re-running with the same ids adds nothing, so the later `/redkiln:closeout` run recording the same atoms cannot conflict with this one and needs no coordination.
- **Not a transition.** `record-links` records provenance; it does not touch `status`, `stage` or `updated`-driven lifecycle, and it is therefore compatible with this story's prohibition on `advance`.
- **Dry-runnable.** `--dry-run` validates and reports without writing, which is what makes "prove the ids resolve before mutating the initiative item" a real step rather than a hope.

No other item's frontmatter is written. The `.kb/` tree is read-only to this story.

## Acceptance criteria

Framed from the **reader whose goal crosses the whole stack**, not from the command that
produces the artefact. Two readers are served, and both are named upstream rather than
invented here:

- the **repository owner as closeout authoriser**, who must decide whether
  `/redkiln:closeout` can be run on HS-I0006 without failing its own preflight
  (`.redkiln/processes/initiative.yaml:32-41`); and
- the **next initiative's planner**, who inherits an adjudicated audience by following the
  closed initiative's `links.kb` instead of re-deriving four personas from the same
  download counts and issue threads (`.kb/product/README.md:15-24`;
  `initiative.md:252-258`).

"Verification" means a cited, re-derivable observation — this project owns no code, and its
own testing brief makes **static** and **process** first-class tiers for exactly that reason
(`.../closeout-and-durable-audience/_decomposition.md:36-40`, AC-014 row at `:57`). Every
row below is proven by an artefact a reader can open plus the captured output of a command
that was actually run on the closeout tree.

| id | criterion | verification |
| --- | --- | --- |
| **AC-001** | **GIVEN** the repository owner is deciding whether to authorise `/redkiln:closeout` on HS-I0006, **WHEN** they open `_closeout-record.md` and read the `## Closeout readiness (AC-014)` section — **appended in place** beneath the findings register's section, with no earlier section rewritten, reordered or truncated and no private artefact created beside the record — **THEN** they meet a child census of **exactly ten rows**, one per item with `parent: HS-I0006` (HS-P0010…HS-P0019), each carrying `id`, `slug`, `status`, `stage` and `path` **verbatim** from one `redkiln status --json` run whose `index.branch`, `index.builtAt` and tree commit SHA are quoted above the table, with HS-P0010…HS-P0018 each reading `status: done` **and** `stage: closeout`; a count other than ten, or a sibling at `review` with an approved verdict but no closeout advance, is reported as a named residual rather than rounded off. | `_closeout-record.md` → *Census provenance* + *Child census*, with the raw `redkiln status --json` capture; row set compared against `.bklg/from-contract-to-published-library/_decomposition.md:25-34`; the closure predicate compared against `.redkiln/processes/project.yaml:71-80`; `--all-branches` absent from the recorded invocation |
| **AC-002** | **GIVEN** the same authoriser knows the closeout preflight audits **story** grain as well as project grain, and that a project marked closed above a story still at `report` is the inconsistency that makes a preflight fail after they have already said yes, **WHEN** they read on, **THEN** a *Story census* gives, per sibling project HS-P0010…HS-P0018, a count of stories at `status: done` / `stage: closeout` over that project's total, **plus** an itemised list of every story not there with its id, owning project and current stage — nothing netted out into a single aggregate number. | `_closeout-record.md` → *Story census*; the same JSON capture filtered to `type: story`; the terminal story stage at `.redkiln/processes/story.yaml:33-41` |
| **AC-003** | **GIVEN** AC-014's sentence ("no open child of HS-I0006") cannot be literally true while HS-P0019 — the project writing the record — is itself an open child, **WHEN** the authoriser looks for the catch, **THEN** the record has already stated it: HS-P0019 is named as open at the moment of writing, its own stories are listed with their stages, the remaining transitions are enumerated **in order** (this story's `report` → `closeout`, then the project's `review` → `closeout`), and the single advance after which the sentence becomes literally true is named and attributed to the orchestrating command — never performed here. | `_closeout-record.md` → *The declared residual*; the transition order checked against `.redkiln/processes/story.yaml:33-41` and `.redkiln/processes/project.yaml:71-80`; the non-performance evidenced by this story's work-commit range touching only the PR-boundary globs |
| **AC-004** | **GIVEN** the next initiative's planner will follow `links.kb` expecting an audience rather than a directory, **WHEN** the record's atom manifest is read, **THEN** it lists **every** file under `.kb/product/` except `README.md` with its path and its frontmatter `id`, `kind` and `authority_tier`, and states observed-vs-expected as two **computed** counts against the amended evaluator decision — seven atoms, exactly three of them `kind: concept` personas (application author, adapter author, local-first/edge developer), the remaining four `kind: playbook` journeys including the evaluation path as a journey of its own, all `authority_tier: product` — and **any** mismatch produces a finding row naming `product-atom-promotion-via-kb-ingest` as owner, routed through the findings register, with **zero** atoms authored, edited, moved or renamed by this story. | Directory listing of `.kb/product/` plus each atom's frontmatter, quoted into `_closeout-record.md` → *Atom manifest*; expectation at `.../closeout-and-durable-audience/_decomposition.md:227-242`; kinds and tier at `.kb/product/README.md:6-9`; the no-authoring rule enforced by the PR boundary (which contains no `.kb/` path) and `_storymap.md:95-96` |
| **AC-005** | **GIVEN** DoD 16's second clause is a promise until `links.kb` on the initiative item is non-empty, and that the item's frontmatter may only be written by the CLI, **WHEN** the linkage is performed, **THEN** `redkiln record-links HS-I0006 --atom <comma-separated ids> --dry-run` ran **first** with its output captured and clean (writing nothing), the real invocation followed **only** on that clean dry run, `.bklg/from-contract-to-published-library/initiative.md`'s `links.kb` was **re-read after the write** and quoted verbatim into the record with its members matched one-for-one against AC-004's manifest — never inferred from an exit code — and the record names `redkiln record-links HS-I0006 --atom <ids> --remove` as the reversal, so the one durable state change this story causes is undoable without a hand edit. | `_closeout-record.md` → *Link manifest*: the dry-run transcript, the real-run transcript, and the post-write quotation of `initiative.md:21-24`; flag semantics (comma-separated single `--atom`, append-and-dedupe, `--dry-run` writes nothing) per `redkiln record-links --help`; hand edits blocked by the `PreToolUse` hook (`CLAUDE.md`) |
| **AC-006** | **GIVEN** the authoriser must know which halves of the initiative's exit criteria this story can speak for and which belong to a sibling or to the closeout command itself, **WHEN** they read the answer sheet, **THEN** three rows — DoD 16, exit criterion 7, exit criterion 8 — each split the criterion into its clauses and name an owner per clause (DoD 16 clause 1 → `product-atom-promotion-via-kb-ingest`, clause 2 → this story; exit criterion 7's `validate --kb` / `doctor` half → `backlog-and-kb-health-at-closeout`; exit criterion 8's whole-gate half → `whole-gate-green-on-the-assembled-tree` and `published-tree-delta-statement`, its harvest half → `/redkiln:closeout`'s `closure` and `retrospective` stages), every cell citing an artefact path, and the slice-mates' commands **linked rather than re-run** so no question gets a second, separately-timed answer. | `_closeout-record.md` → *Exit-criteria answer sheet*; criteria text at `initiative.md:405-407` and `:578-581`; stage ownership at `.redkiln/processes/initiative.yaml:32-41`; the six-file `template-drift` set at `.github/workflows/ci.yml:177-186` cited to the health story rather than re-asserted; slice convergence per `_storymap.md:25-30` |
| **AC-007** | **GIVEN** a verdict that can only come out green is not a verdict, **WHEN** the authoriser reaches the end of the record — this section **last**, after every other story's — **THEN** they find `READY` or `NOT-READY`, a one-line reason, and when NOT-READY a residual table of exactly three columns (the residual, the item that owns it, where it is routed per DR-12: `support`, a new item against the owning sibling, or a decision atom plus a re-plan); **and** the record states as observations, not promises, that no `redkiln advance`, no `redkiln new` and no `redkiln adopt --templates` was run, evidenced by the story's work-commit range touching only the three PR-boundary globs. A NOT-READY verdict ships and merges unchanged; lowering a predicate, closing a sibling or authoring an atom to reach READY is a failed story. | `_closeout-record.md` → *Verdict*; destinations against `project.md` DR-12 and `.redkiln/config.yaml:5`; commit range against the PR boundary via `.redkiln/config.yaml:69-73` (`require_commit_provenance`) and this story's `_ledger.md`; `redkiln verify --item HS-S0135 --grain story` exits zero (`.redkiln/processes/story.yaml:32`) |

**Traceability.** Project **AC-014** ("all nine sibling projects are closed out and the
initiative's closeout links the promoted product atoms, evidenced by `redkiln status`
showing no open child of HS-I0006", `project.md:241-243`) is covered whole: its census half
by AC-001/AC-002/AC-003, its linkage half by AC-004/AC-005, its "can the initiative close"
framing by AC-006/AC-007. Project **DoD 7** (`project.md:264-265`) and initiative **exit
criterion 8** (`initiative.md:580-581`) are the same coverage read at their own grain. No
other project AC is claimed here.

## Interaction quality

This story renders **no screen and no public API item** — the project's signed-off
`_design.md` answers N/A to every composition prompt (`_design.md:41-91`) and
`design.capture` is a *declared* skip, not a silent one (`.redkiln/config.yaml:75-82`). That
does not make this section vacuous: the deliverable is a document a human reads under time
pressure to make an irreversible decision, and the invariants below are what keep it
legible. **Every one of them is written into an AC row above** — `redkiln verify` extracts
ACs by matching a leading `| AC-001 |` cell, so an invariant that lived only as a bullet
here would never be gated. This section says which row carries which invariant and how it is
checked.

**State invariants**

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **In place, not a context jump** — the readiness section is *appended* to the existing `_closeout-record.md`; no new sibling artefact, no re-cut document | AC-001 ("appended in place… no private artefact created beside the record") | `git diff` on the record shows additions only below the findings section; the PR boundary permits exactly one path under the project directory |
| **Non-occlusion** — the health and findings sections stay whole and readable; the readiness section links them rather than restating or overwriting them | AC-001, AC-006 | No earlier heading is modified in the diff; the answer sheet's cells are links/citations, not copied command output |
| **Preserved position** — a reader who bookmarked or linked an earlier heading lands where they did before | AC-001 ("no earlier section rewritten, reordered or truncated") | Heading text and order above the appended section are byte-identical pre/post |
| **Reversibility** — the one durable write is undoable, and nothing irreversible happens before it is proved | AC-005 (dry-run first; `--remove` named as the reversal), AC-004 (no `.kb/` write exists to reverse) | The dry-run transcript precedes the real-run transcript in the record; `--remove` is named in the record, not only in this spec |
| **Reachable without special tooling** — every claim is a literal, greppable string: ids, paths, frontmatter values, quoted JSON fields. No image, no binary, no collapsed detail, nothing that requires the CLI to re-run to be read | AC-001 (verbatim JSON fields), AC-004 (paths + frontmatter ids), AC-005 (verbatim quotation of `links.kb`) | `rg 'HS-P001' _closeout-record.md` returns the census rows; the atom ids in the record match `rg '^id:' .kb/product/*.md` |

**Composition invariants**

The rendered-surface family from `_design.md` is **empty by sign-off, not by omission**:
there is no screen, no widget and no API item for hierarchy, transience or density to attach
to (`_design.md:10-12`, `:41-45`, approved 2026-08-12). What survives is the *document*
composition the Integration contract already fixes, and it is likewise carried by AC rows:

| Invariant | Real numbers / shape | Carried by |
| --- | --- | --- |
| **Presentation exists at all** — every claim arrives as a table with named columns and cited paths, never as a pasted command dump or an attached `status.json` standing in for a reading | Census, story census, atom manifest and answer sheet are all tables; raw captures are quoted *beneath* the table they support, not instead of it | AC-001, AC-002, AC-004, AC-006 |
| **Placement** — the section sits after the findings register's section and is the last section in the record, so a reader meets the verdict having already met its inputs | one `##` section, five `###` subsections in fixed order: *Census provenance* → *Child census* / *Story census* → *The declared residual* → *Atom manifest* / *Link manifest* → *Exit-criteria answer sheet* → *Verdict* | AC-001, AC-007 |
| **Transience** — nothing is revealed on demand or left in a terminal; every transcript the verdict rests on is persistent in the record | dry-run and real-run transcripts, the JSON capture's provenance triple, and the post-write `links.kb` quotation are all in-document | AC-001, AC-005 |
| **Density budget** — fixed, computable row counts, so an over-long section is a defect rather than a style choice | **10** child rows; **9** story-census rows plus an itemised exception list; **7** expected atom rows; **3** answer-sheet rows; **1** verdict line plus a 3-column residual table | AC-001, AC-002, AC-004, AC-006, AC-007 |
| **Hierarchy** — heading depth composes into the record's existing outline rather than competing with it (`##` for the section, `###` for its parts, matching the sections the slice-mates appended) | as above | AC-001 |
| **Named anti-patterns** — the ones this story can actually commit: repairing a defect it found (`_storymap.md:92-94`), hand-authoring a `.kb/` atom (`project.md` DR-8; `0269720`), absorbing rather than routing a finding (DR-12), and netting HS-P0019's own openness out of the census | — | AC-003, AC-004, AC-007 |

## Error conditions

| id | condition | required behaviour |
| --- | --- | --- |
| **EC-001** | `_closeout-record.md` is absent when this story starts | **Loud halt**, reported as a missing dependency on `clean-checkout-harness`. Do not create the record, do not write a private artefact beside it, do not proceed (`_storymap.md:25-30`) |
| **EC-002** | `redkiln status --json` exits non-zero, or its `index` is stale relative to the tree | Re-resolve on the closeout tree and re-run; record the failure and the fix. **Never** fall back to parsing the human-readable output, and **never** substitute `--all-branches` — a sibling closed on another branch is not a sibling closed on the tree that closes (`redkiln status --help`) |
| **EC-003** | The census returns a child count other than ten | Record every row observed, state the delta against `.bklg/from-contract-to-published-library/_decomposition.md:25-34`, raise a finding (an unrecorded project, or one moved out of the initiative) and go **NOT-READY**. Do not create or re-parent an item |
| **EC-004** | A sibling HS-P0010…HS-P0018 is at `review` with an approved verdict but has not advanced to `closeout` | Report it as a residual with its id and current stage, name the missing advance and its owner, go **NOT-READY**. Running the advance is out of scope for every actor in this story |
| **EC-005** | `.kb/product/` is empty, or the atom set is not seven atoms with exactly three `kind: concept` personas | Emit the manifest as observed, state both computed counts, hand a finding to `findings-disposition-register` naming `product-atom-promotion-via-kb-ingest` as owner, go **NOT-READY**. Under no circumstance author, edit or move an atom (`project.md` DR-8) |
| **EC-006** | `record-links … --dry-run` reports an id that does not resolve | **Do not run the real invocation.** Capture the dry-run output, raise a finding against the promoting story, go **NOT-READY**. `links.kb` stays `[]` and DoD 16 clause 2 stays open, honestly |
| **EC-007** | The real `record-links` run exits zero but the re-read of `initiative.md` shows `links.kb` unchanged or partial | Treat the exit code as unproven: record both the transcript and the re-read, raise a tooling finding routed to `support` (`.redkiln/config.yaml:5`), go **NOT-READY**. Do not hand-edit the frontmatter to "correct" it — the `PreToolUse` hook denies it and the denial is the design |
| **EC-008** | A residual could be cleared by one small advance, edit or atom, and clearing it would make the verdict READY | The verdict goes **NOT-READY** and ships. This is the named risk (`project.md` risk table, "Pressure to fix what re-observation finds"; `_storymap.md:92-94`): a story that repairs what it found has failed its own acceptance criterion |
| **EC-009** | The slice-mates' sections are missing or contradict this census (e.g. the health story reported a seventh drift advisory) | Cite the contradiction, do not re-run their commands to adjudicate it, and route it as a finding. Re-running `doctor --json` here would produce a second, separately-timed answer to a question another story owns |

## Non-functional

| id | requirement | why / how observed |
| --- | --- | --- |
| **NF-001** | **Zero lifecycle mutation.** No `redkiln advance`, no `redkiln new`, no `redkiln adopt --templates`, and no hand edit of any item's system frontmatter. The only frontmatter change in the PR is `links.kb` on HS-I0006, written by `record-links` | `CLAUDE.md` *Where the work lives* (the CLI is the single writer); observable as a diff constrained to the PR boundary and recorded per AC-007 |
| **NF-002** | **Re-derivability.** Every number in the section is recomputable by a reader from the captured JSON and a listing of `.kb/product/` — counts are stated *with* the inputs they were computed from, never asserted | `.../closeout-and-durable-audience/_decomposition.md:36-40` (static/process are real tiers only if their evidence is checkable) |
| **NF-003** | **Idempotence of the one write.** Re-running `record-links` with the same ids records nothing new, so the later `/redkiln:closeout` run recording the same atoms is a no-op and needs no coordination with this story | `redkiln record-links --help` — "values are appended and de-duplicated" |
| **NF-004** | **Boundary containment.** `redkiln verify --item HS-S0135 --grain story` exits zero: the ledger carries a row per AC with real evidence, commit provenance is recorded, and no file outside the three PR-boundary globs changed | `.redkiln/config.yaml:62-73`; `.redkiln/processes/story.yaml:32` |
| **NF-005** | **Cost.** This story runs read-only backlog and filesystem commands only. It does **not** re-run `cargo xtask ci` — that is `whole-gate-green-on-the-assembled-tree`'s, and a second run here would be a second, separately-timed answer to an owned question | `.redkiln/config.yaml:56-60`; `_storymap.md:54` |
| **NF-006** | **Durability of the inheritance.** After merge, a reader who opens the HS-I0006 item and follows `links.kb` reaches the promoted atoms without opening this backlog folder at all — the point of the linkage is that the record is not the only path to the audience | `.kb/product/README.md:15-24`; `initiative.md:252-258` |

## Implementation notes (non-prescriptive)

Shape suggestions, not instructions — the ACs are the contract.

- **Order of operations that avoids rework.** Confirm `_closeout-record.md` exists (EC-001)
  → take the census in **one** `redkiln status --json` run and save the raw capture → derive
  the child and story tables from that saved capture, never from a second run → list
  `.kb/product/` → dry-run the linkage → real run → re-read `initiative.md` → write the
  section → decide the verdict last, from what the section actually says.
- **One capture, one provenance triple.** Two runs produce two `index.builtAt` values and a
  reader cannot tell which table came from which. Capture once (e.g. into the story folder
  or quoted inline) and derive everything from it.
- **`--atom` takes one comma-separated value.** Repeated `--atom` flags are last-wins under
  strict `parseArgs` and would silently record a single id — the help text says so
  explicitly, and it is the easiest way to produce a green run that linked one atom out of
  seven.
- **Read the atom ids from the atoms, not from the slice-mate's report.** The `id`
  frontmatter field is the thing `record-links` records; a title or filename is not it.
- **Append, do not rewrite.** A text append keeps the "no earlier section modified" invariant
  true by construction; regenerating the whole record from a template does not.
- **The verdict is written after the tables, not before them.** Deciding READY first and then
  assembling evidence is how EC-008 happens.
- **Nothing here writes Rust,** so `CLAUDE.md`'s binding constraints are untouched — and this
  must not become the place someone quietly exercises them. If a crate file appears in the
  diff, the PR boundary is wrong or the story has drifted.

## Tests and CI (merge gate)

Tiers as the project's testing brief defines them: *static* reads source/config/frontmatter
without executing the code under test; *process* checks the commit graph or backlog state.
AC-014's assigned tier there is **process** (`.../closeout-and-durable-audience/_decomposition.md:57`).

| tier | command / path | proves |
| --- | --- | --- |
| Process | `redkiln status --json` (one run, on the closeout tree, capture retained) | AC-001, AC-002, AC-003 — the child census, the story census and the declared residual, with `index.branch` / `index.builtAt` provenance |
| Static | Listing + frontmatter of `.kb/product/*.md` compared against `.../closeout-and-durable-audience/_decomposition.md:227-242` and `.kb/product/README.md:6-9` | AC-004 — seven atoms, exactly three personas, all `authority_tier: product`, computed not asserted |
| Process | `redkiln record-links HS-I0006 --atom <ids> --dry-run`, then the real run, then a re-read of `.bklg/from-contract-to-published-library/initiative.md:21-24` | AC-005 — the linkage is proved before it is performed and verified after, not trusted from an exit code |
| Static | `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md` — the `## Closeout readiness (AC-014)` section reviewed against `initiative.md:405-407`, `:578-581` and `.redkiln/processes/initiative.yaml:32-41` | AC-006 — the answer sheet's clause-by-clause ownership; AC-007 — the verdict's shape and the recorded non-actions |
| Gate (blocking) | `redkiln verify --item HS-S0135 --grain story` (`.redkiln/processes/story.yaml:32`) | The ledger carries one satisfied, evidence-bearing row per AC-###; commit provenance recorded; no file changed outside the PR boundary |
| Gate (story grain) | `cargo xtask affected --base main` (`.redkiln/config.yaml:40`) | This story maps to **no workspace package**, which is the expected and correct outcome; the command still runs the five file-reading lints and `spec-trace` unconditionally, so a stray edit to a crate or to `spec/SPECIFICATION.md` cannot slip through green (`.redkiln/config.yaml:36-39`) |
| Cited, **not** re-run | `redkiln validate`, `redkiln validate --kb`, `redkiln doctor --json`, and the six-file `template-drift` assertion at `.github/workflows/ci.yml:177-186` | Owned by `backlog-and-kb-health-at-closeout` (AC-011/AC-012). Linked from the answer sheet; re-running them here would be the fourteen-entries-in-fourteen-places failure mode at one remove |
| Cited, **not** re-run | `cargo xtask ci` (`.redkiln/config.yaml:60`, the terminal grain) | Owned by `whole-gate-green-on-the-assembled-tree` (AC-001/AC-002 of the project). This story's exit-criterion-8 row cites that run's SHA and verdict |

The project-grain integration gate that runs `cargo xtask ci` fires at the **project's**
`implementation → review` seam, not at this story's. Nothing in this story's own merge gate
compiles anything, and that is the correct shape for a story whose whole deliverable is a
cited observation.

## Risks and coupling (PR-scoped)

| Risk | Why it bites here | Containment |
| --- | --- | --- |
| **Pressure to close the last sibling** | One `redkiln advance` away from a clean census is the most tempting single command in the initiative, and it would make AC-014 read true | NF-001 and EC-004/EC-008: the advance belongs to the orchestrating command; NOT-READY ships. The absence is evidenced by the commit range, not asserted |
| **Pressure to hand-write a missing atom** | If `.kb/product/` is short an atom, one file makes the manifest balance — and reproduces exactly what `0269720` was reverted for | EC-005; the PR boundary contains no `.kb/` path, so the write would fail `redkiln verify` before review |
| **`--all-branches` "fixes" the census** | A sibling shown closed on some branch would clear a residual instantly | AC-001 records the invocation verbatim; EC-002 forbids the substitution and says why |
| **Repeated `--atom` flags** | Last-wins under strict `parseArgs`: a green run that recorded one id out of seven, with no error anywhere | AC-005's post-write re-read compares members one-for-one against AC-004's manifest, which is what catches it |
| **Census staleness across the slice** | The health story's `doctor` run and this story's `status` run happen at different moments; a project could advance between them | AC-001 quotes `index.builtAt` and the tree SHA, so a reader can date the census rather than assume it |
| **The self-reference paradox is quietly rounded off** | "Nine of ten closed" reads like a rounding error and is the single most likely thing to be glossed | AC-003 makes the declaration a first-class subsection, with the remaining transitions enumerated and the enabling advance named |
| **Coupling to `/redkiln:closeout`'s own `record-links`** | The closeout command will record atom links too; two writers of the same field | NF-003: append-and-dedupe makes the second run a no-op. No coordination, no lock, no ordering constraint |
| **Coupling to the slice-mates' sections** | This section cites two sections that must already exist; if the slice order slips, the citations dangle | The slice order is fixed at `_storymap.md:152-155` (health → findings → readiness) and EC-009 says what to do if a cited section is missing: cite the gap, route it, do not adjudicate it |

## Dependencies

**Blocks on** (must merge first; both are slice-mates in `closeout-health-and-disposition`,
and the order within the slice is fixed at `_storymap.md:152-155`):

- `product-atom-promotion-via-kb-ingest` — supplies the atoms under `.kb/product/` that
  AC-004 enumerates and AC-005 links. Without it there is nothing to record and DoD 16
  clause 2 has no referent.
- `findings-disposition-register` — supplies the routing mechanism AC-004, AC-007 and
  EC-003…EC-007 hand residuals to. Without it a NOT-READY verdict would name residuals with
  nowhere to send them, which is how a finding gets absorbed.

Transitively, through `findings-disposition-register`'s own `depends_on`
(`_storymap.md:62`): `whole-gate-green-on-the-assembled-tree`,
`dod-set-re-observation-record`, `published-tree-delta-statement`,
`decision-atom-audit-table`, `open-question-preservation-audit` and
`backlog-and-kb-health-at-closeout` — every story whose findings this verdict must be able
to see.

**Unlocks:**

- HS-P0019's own `implementation → review → closeout` sequence — this is the last leaf of the
  project (`_storymap.md:152-155`), so the project's review gate reads a complete record.
- `/redkiln:closeout` on HS-I0006 — its preflight audits the same predicates this story
  observed, and its `closure` and `retrospective` stages (`.redkiln/processes/initiative.yaml:32-41`)
  are what actually close and archive the initiative.
- Initiative **exit criterion 8** and **DoD 16** clause 2 (`initiative.md:580-581`,
  `:405-407`) — and, downstream of the linkage, the next initiative's planner inheriting the
  audience instead of re-deriving it.

## Anchors (progressive disclosure)

Load-bearing depth, deferred rather than pasted. Each row says why the artefact is
load-bearing and the moment to open it; every path was checked to exist on the closeout
tree.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `.redkiln/processes/project.yaml` | Defines the terminal project stage (`review` → `closeout`, `status_on_enter: done`) at `:71-80`. "Closed out" must mean what the process pack says, not what a reader assumes — this is the exact line a sibling at `review` with an approved verdict fails against | Before writing the child census predicate — i.e. before the first census row is typed | AC-001 |
| `.redkiln/processes/story.yaml` | The story lifecycle and its terminal `closeout` stage (`:33-41`), plus the blocking `redkiln verify --grain story` gate at `:32` that this story's own ledger must satisfy | Before the story-grain census, and again before flipping any ledger row | AC-002 |
| `.redkiln/processes/initiative.yaml` | `closure` (`status_on_enter: closeout`, `harvest_kb: true`) and `retrospective` (`status_on_enter: archived`) at `:32-41` — the two stages this story must **not** perform and must attribute correctly in the answer sheet | When writing the declared residual and the exit-criterion-8 row | AC-003, AC-006 |
| `.bklg/from-contract-to-published-library/_decomposition.md` | The children table at `:25-34` is the independent count the ten-row census is checked against; a census that agrees with itself proves nothing | Immediately after capturing `redkiln status --json`, to compare row sets | AC-001 |
| `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_decomposition.md` | Carries the **amended** evaluator decision at `:150-242` — seven atoms, exactly three personas, the evaluation path as a journey of its own. The superseded text is preserved above the amendment, so reading only the first paragraph gives the wrong expected count | Before computing observed-vs-expected in the atom manifest; read `:227-242` (the amended consequence), not the preserved original | AC-004 |
| `.kb/product/README.md` | `:6-9` fixes persona = `concept`, journey = `playbook`, both `authority_tier: product`; `:15-24` states that closeout performs the promotion so the next initiative inherits the audience — the persona-journey slice this story's linkage actually serves | While validating each atom's `kind`/`authority_tier`, and when writing why the linkage matters | AC-004, AC-005 |
| `.bklg/from-contract-to-published-library/initiative.md` | The item whose `links.kb` is written (`:21-24`, today `kb: []`), DoD 16 at `:405-407`, exit criteria 7–8 at `:578-581`, and the *Referenced personas & journeys* section at `:227-258` that names the four journeys and the secondary-evidence qualification travelling with them | Read `:21-24` before and after the `record-links` run; read `:405-407` and `:578-581` when building the answer sheet | AC-005, AC-006 |
| `.bklg/from-contract-to-published-library/closeout-and-durable-audience/project.md` | AC-014's exact wording (`:241-243`), DR-12's three routing destinations (`:179-183`), DR-13 (`:184-186`), DoD 7 (`:264-265`) and the risk table row on pressure to fix what re-observation finds | When choosing a destination for any residual, and before writing the verdict | AC-004, AC-007 |
| `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_storymap.md` | `:25-30` (why all evidence converges on one artefact), `:92-97` (no story fixes what it found; no story hand-authors an atom), `:152-155` (the fixed order within this slice) | Before appending to the record, and any time a residual looks cheap to fix | AC-003, AC-004, AC-007 |
| `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_design.md` | The signed-off determination that this project has **no public API surface** (`:10-12`, `:41-45`, approved 2026-08-12) — the reason the rendered-surface composition family is empty by sign-off rather than by omission | Before writing anything that looks like a surface, or if a reviewer asks where the composition invariants went | AC-001, AC-007 |
| `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_grounding.md` | What actually exists on disk at planning time, including the state of `.kb/product/` and the reserved-but-unwritten ADR numbers — the baseline any "missing" claim must be checked against before it becomes a finding | When the atom manifest or a census row looks wrong, before raising a finding | AC-004 |
| `.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md` | The discovery source the promoted atoms carry forward — four personas, four journeys, each resting on secondary evidence. It is what makes the inheritance meaningful rather than a directory of files | When judging whether the manifest's seven atoms correspond to the audience this initiative actually served | AC-004 |
| `.redkiln/config.yaml` | `:5` (`support_initiative`, where incidental findings route), `:40` (the story-grain gate), `:56-60` (the terminal grain owned by a slice-mate, not re-run here), `:62-73` (`require_ledger`, `require_commit_provenance`), `:75-82` (the declared `design.capture` skip) | When routing a finding, and when assembling the ledger and commit provenance | AC-006, AC-007 |
| `.github/workflows/ci.yml` | `:177-186` is the exact six-file `template-drift` set the health story asserts — cited so the answer sheet can attribute exit criterion 7's half without re-running `doctor --json` | Only when writing the exit-criterion-7 row; never as a licence to re-run the assertion | AC-006 |
| `CLAUDE.md` | The CLI is the single writer of item system frontmatter and a `PreToolUse` hook denies hand edits; `redkiln adopt --templates` is never run; the six deliberate template customisations | Before touching anything that looks like frontmatter, and before any "just fix it" impulse | AC-005, AC-007 |
| `RUNBOOK.md` | The plan of record — where this initiative's phases sit and what closeout is expected to have produced; orientation for a reader who arrives at the record without the backlog context | Optional orientation before starting; not needed to satisfy any AC | AC-006 |

## Clarifications resolved during spec

1. **AC-014 is not literally observable from inside the terminal project, and the spec says
   so rather than rounding it off.** HS-P0019 is a child of HS-I0006 and is open while this
   story runs. The observable predicate is "HS-P0010…HS-P0018 at `done`/`closeout`, HS-P0019
   the sole declared residual, with the enabling advance named" (AC-003). AC-014's full
   sentence becomes true one advance after this story merges, and that advance belongs to
   `/redkiln:closeout`.
2. **The atom count is seven, of which three are personas** — not four personas. The
   evaluator decision was **amended on 2026-08-12** by the repository owner
   (`.../closeout-and-durable-audience/_decomposition.md:150-242`): the evaluation path is a
   first-class *journey* atom linked to the application-author persona, not a fourth
   persona. `_storymap.md:100-106`'s *Grain note* still reads "four personas", which is
   downstream of the superseded text; the amendment governs. That inconsistency is itself a
   **finding to route**, not something this story edits.
3. **The linkage is performed here, not deferred to `/redkiln:closeout`.** DoD 16's second
   clause would otherwise be a promise about a command outside the backlog. `record-links`
   appends and de-duplicates, so performing it now cannot conflict with the closeout run
   doing it again — which is what makes the early write safe rather than presumptuous.
4. **The PR boundary is widened by one path beyond the story folder and the record**, to
   `.bklg/from-contract-to-published-library/initiative.md`, because `record-links` writes
   `links.kb` there. It is named in the boundary rather than discovered at verify time; only
   the CLI writes those lines.
5. **No AC was added or dropped.** The seven ids the front half decided (AC-001…AC-007) are
   exactly the ids enumerated above and exactly the rows in `_ledger.md`. The twelve rows of
   *Behavior and interfaces* map onto them: census provenance / ten rows / sibling predicate
   → AC-001; story census → AC-002; declared residual → AC-003; atom set from disk +
   mismatch routed → AC-004; linkage proved + verified → AC-005; answer sheet + seam cited
   → AC-006; verdict + recorded non-actions → AC-007.
6. **The composition family of the interaction-quality section is empty by sign-off.**
   `_design.md` records no public API surface and `design.capture` is a declared skip. The
   invariants that survive are document-composition ones with real numbers, and they are
   carried as AC rows rather than as prose bullets, because `redkiln verify` only extracts
   table cells and `- AC-###:` bullets.
7. **`redkiln status` is run without `--all-branches`.** The question is whether the
   assembled closeout tree can close, not whether some branch somewhere holds a closed copy
   of an item.
