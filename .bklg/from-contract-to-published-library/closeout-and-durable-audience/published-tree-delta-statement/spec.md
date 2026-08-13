---
item: HS-S0128
stage: spec
created: 2026-08-12T13:48:07.687Z
updated: 2026-08-12T13:48:07.687Z
template_sig: 87bbf1d0
rendered_sig: 6c3c3ff6
---

# Spec — The DoD 13 delta between the published tag and the closeout tree, stated

## Scope lock

| What | Path |
| ---- | ---- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` — DoD 13 at `:396-397`, the charter's exit criteria at `:561-582` |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` — merge order at `:178-193`, *Decisions taken at the gate* item 4 at `:239-249` |
| Project | `.bklg/from-contract-to-published-library/closeout-and-durable-audience/project.md` — DR-4 at `:140-143`, AC-004 at `:204-206`, the DoD 13 risk row at `:298` |
| This spec | `.bklg/from-contract-to-published-library/closeout-and-durable-audience/published-tree-delta-statement/spec.md` |
| Key brief | `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_decomposition.md` — the one warranted (`testing`) brief; the AC-004 row at `:47` fixes the tier as **static / process** and the procedure as `git log <published-tag>..HEAD` filtered to HS-P0017 and HS-P0018 |
| Signed-off design | `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_design.md` — **no public API surface**, N/A in every section, approved 2026-08-12. This story renders no surface and adds no item. |
| Story map row | `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_storymap.md:56` (this story), `:75-77` (why this slice is one artefact with two halves) |
| Grounding | `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_grounding.md:18-48` — which decision atoms exist and which numbers are reserved-but-unwritten |
| Roadmap pointer | `RUNBOOK.md:244-258` (the ordering constraints the DAG carries); `.redkiln/config.yaml:56-60` (the terminal `e2e` grain this project alone carries) |

## One-line PR slice

State the DoD 13 delta rather than glossing it: the published `0.2.0` tag, the commits between it and the closeout tree, the two projects that own them, and the cited gate decision that keeps the published surface unchanged across the delta.

## Executive summary

DoD 13 asks for `cargo xtask ci` green **"on the exact tree that was published"** (`.bklg/from-contract-to-published-library/initiative.md:396-397`). That phrase is false by construction the moment the plan was approved: `publication-and-positioning` (HS-P0016) ships `0.2.0` at merge position 7, and `replication-identity-and-ingest` (HS-P0017) and `retention-and-incomplete-logs` (HS-P0018) merge at positions 8 and 9, ahead of this project at 10 (`.bklg/from-contract-to-published-library/_decomposition.md:178-193`). The closeout tree therefore *cannot* be byte-identical to the published one.

This PR lands the honest statement of that gap, as a new `## DoD 13 — the published-tree delta` section appended to this project's single closeout artefact. Its slice-mate `dod-set-re-observation-record` fills the fourteen-row table for DoD 1–12 and 14–15; **DoD 13 is deliberately absent from that table** and this section is what row 13 would otherwise have been. The two are read in one sitting by one reader, which is why they are one slice (`_storymap.md:75-77`).

The delta is stated as four named things, not as a caveat sentence: the published tag with its SHA, the enumerated commit range from that tag to the SHA the gate actually ran against, the owning project for every commit in that range with no residue, and the **cited** reason the published surface is unchanged across it. The reason is cited rather than re-derived — the surface diff belongs to `retention-and-incomplete-logs` (`.bklg/from-contract-to-published-library/retention-and-incomplete-logs/_storymap.md:78`) and the constraint that made it possible belongs to the initiative gate (`_decomposition.md:239-249`). This story's contribution is that both are *pointed at from where DoD 13 is being adjudicated*, so a reader deciding whether the initiative closed honestly does not have to go and find them.

The delta between this and the naive version is the whole point: the naive version writes "the closeout tree differs slightly from the published tree, but the public API is unchanged" and cites nothing. That sentence is unfalsifiable, and it is exactly what AC-004's word "glossed" forbids.

## Context pack

Read this section and you can start. Everything deeper is a signposted anchor.

**1. This project writes no code, and this story writes no test.** The deliverable is a run plus a cited record (`_storymap.md:14-22`). The `testing` brief classes AC-004 as **static / process** tier: `git log`, `git show`, `git tag` over the closeout tree, and a checked, cited artefact — explicitly "not a test-framework check" (`.bklg/.../closeout-and-durable-audience/_decomposition.md:47`). Do not write a Rust test for this. Do not touch any crate. `_design.md` records **no public API surface** for the whole project and approves that determination; adding one here contradicts a signed-off design.

**2. The published tag is an input this story does not create.** `0.2.0` is shipped by `publication-and-positioning`'s `publish-0-2-0` story (`.bklg/from-contract-to-published-library/publication-and-positioning/publish-0-2-0/`), five projects upstream. At the time this spec is written **no git tag exists in this tree** — `git tag --list` is empty — because HS-P0016 has not run. The implementing agent resolves the tag *by looking*, and records the tag name and the commit SHA it points at. If it is absent, or if more than one plausible tag exists, that is a finding routed to `findings-disposition-register` (`_storymap.md:62`), not a number invented to make the section render.

**3. The far end of the delta is a recorded SHA, not `HEAD`.** The closeout end of the range is the commit `whole-gate-green-on-the-assembled-tree` recorded as the tree `cargo xtask ci` exited zero on (`project.md:192-195`, AC-001). Using `HEAD` at the moment of writing is wrong in a way that is invisible: the delta would then describe a tree no gate was ever run against, and DoD 13's whole subject is *the tree the gate was green on*. If the two have diverged because this story's own commits landed in between, say so and use the gate's SHA.

**4. Every commit in the range gets an owner, and the residue is the finding.** DR-4 and the brief both name HS-P0017 and HS-P0018 as the expected owners. That is the *expectation*, not the method. Enumerate the range and attribute each commit; a commit belonging to neither — a stray fix, a doc touch, a backlog commit from this project itself — is named with what it is, and if it touched anything a consumer can see it is escalated. A delta statement that silently drops the commits that did not fit its story is the gloss AC-004 forbids, wearing a table.

**5. The published-surface claim is cited, never asserted, and never re-derived here.** Two upstream artefacts carry it and both are load-bearing:

  - **The gate decision (why it is true by design).** `_decomposition.md:239-249` item 4 constrains `retention-and-incomplete-logs`'s answer to "what needs no published-surface change", on the explicit reasoning that ES-39/ES-40 are `EventStore` clauses landing after `0.2.0`, so a real decision there would be a change to a published port — under 0.x a minor bump, meaning a `0.3.0` this initiative's exit criteria do not contemplate. The runbook already blesses an explicit written refusal as a legitimate answer, so the constraint cost nothing the charter wanted.
  - **The surface diff (whether it came out true).** `retention-and-incomplete-logs`'s `surface-diff-and-the-ac-012-escalation` story runs the comparison against the `0.2.0` registry baseline and enumerates every additive item (`.../retention-and-incomplete-logs/_storymap.md:78`). `replication-identity-and-ingest`'s `memory-store-ingest-seam` likewise commits to an **inherent** `&self` method with the `EventStore` trait signature byte-identical before and after (`.../replication-identity-and-ingest/_storymap.md:58`).

  This story cites both. It does not re-run a surface diff, and it does not re-litigate the version class — that is HS-P0016's and HS-P0018's work, explicitly out of this project's scope (`project.md:108-111`).

**6. If the surface *did* change, this story reports and routes; it never absorbs.** `project.md:298` states it plainly: "If that constraint was broken upstream, this project reports it — it does not absorb a `0.3.0` the exit criteria do not contemplate." AC-013 makes routing the deliverable, and this story is a named input to `findings-disposition-register` (`_storymap.md:62`). A published-surface change discovered here is escalated with the surface named and the version consequence stated; no fix, no re-version, no edit to a sibling's record.

**7. DoD 13 is not marked green on a byte-identity claim.** The record states what DoD 13's literal phrase cannot mean on this tree, and what was observed *in its place*: the whole gate green on the assembled whole from a clean checkout, with the delta from the published tag bounded and attributed. Writing "DoD 13 ✅" beside a tree that is not the published tree, without that qualification, is the specific dishonesty this story exists to prevent — and it is the one the charter's DoD preamble (`initiative.md:356-358`) is written against.

**8. The reader this is for.** Not a persona from the product layer — this project's audience is the reader of a *closed initiative* who must be able to believe it closed honestly without re-deriving the evidence (`_storymap.md:14-22`). Their journey through this story is: they reach DoD 13 in the re-observation record, find it does not have a row like the other fourteen, follow one pointer to this section, and in one screen learn the tag, the range, the owners, and where the surface claim was actually proved. If they have to open a sibling project's backlog to find out whether the published API moved, this story failed.

## Integration contract

- **Archetype**: `capability` — a reader-observable slice: a question a reader of the closed initiative asks, answered end to end in the artefact they are already reading (`story.md` frontmatter `archetype: capability`).
- **Slice / milestone**: `dod-re-observation`. Slice-mate: **`dod-set-re-observation-record`** (HS-S0127), which is also this story's `depends_on`. The two are implemented in one context and land as one integrated surface: one artefact with two halves, the fourteen-row table and the DoD 13 delta that qualifies the phrase the table is answering (`_storymap.md:75-77`).
- **Mount point**: **`.bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md`** — this project's single closeout artefact, stood up by `clean-checkout-harness` and appended to by every later story (`_storymap.md:26-30`). This story appends the `## DoD 13 — the published-tree delta` section to it. This is the composition root for this project: fourteen ledger entries in fourteen places is the failure mode the charter's DoD preamble is written against, and a delta statement that lives only in this story's own folder is exactly that failure at n=1. **Mounted means: the section is in `_closeout-record.md`, and the slice-mate's DoD 13 position points at it by heading.**
- **Wires into** (real, existing or in-slice contracts — never a double):
  - `.bklg/.../closeout-and-durable-audience/_closeout-record.md` — the artefact and its heading conventions, from `clean-checkout-harness`.
  - `whole-gate-green-on-the-assembled-tree`'s recorded commit SHA and exit code — the far end of the delta range (`project.md:192-195`).
  - `dod-set-re-observation-record`'s fourteen-row table — the near neighbour; its DoD 13 position is the pointer into this section.
  - `.bklg/.../publication-and-positioning/publish-0-2-0/` — the story that creates the `0.2.0` tag this delta starts from.
  - `.bklg/.../retention-and-incomplete-logs/_storymap.md:78` — the surface-diff record and its AC-012 escalation path.
  - `.bklg/.../replication-identity-and-ingest/_storymap.md:58` — the inherent-method commitment keeping the `EventStore` trait signature byte-identical.
  - `.bklg/from-contract-to-published-library/_decomposition.md:239-249` — gate decision item 4, the reason the constraint exists.
  - `findings-disposition-register` — the downstream consumer (`story.md` frontmatter `blocks: [HS-S0134]`); this story hands it any escalation it raises.
  - Git itself: `git tag`, `git log <tag>..<sha>`, `git show`. No tooling is built.
- **Renders surfaces**: **none.** `.bklg/.../closeout-and-durable-audience/_design.md` records N/A in every section — no public API item is added, changed or removed by this project, and the determination itself is what was signed off. This story therefore claims no `## Items` path and introduces none.
- **Conformance rule(s)**: none, and it is not adapter-observable. This story executes no library code and changes no port; there is nothing in `crates/happenstance-testkit/src/suite.rs` that could observe it. Adding a rule here would be decorative by CLAUDE.md's own test — no adapter could fail it.
- **Clause(s)**: none discharged or amended. `spec/SPECIFICATION.md` is not touched. If the delta reveals that a `[FROZEN]` clause moved after publication, that is an escalation routed under AC-006, not an edit made here — changing a frozen clause takes a new ADR (`CLAUDE.md`; `.kb/decisions/README.md`).
- **Advances DoD scenario**: **initiative DoD 13** (`.bklg/from-contract-to-published-library/initiative.md:396-397`) — the only scenario this story is on, and the one whose literal phrasing it exists to qualify. Project-grain: AC-004 and DoD item 2 (`project.md:251-253`). It contributes evidence to, but does not own, AC-013.

## PR boundary

The narrowest set that is honestly true. This story writes backlog markdown and nothing else — no crate, no `spec/`, no `.kb/` atom. `redkiln verify --grain story` reads the first fenced block below and fails on any file changed outside it.

```
.bklg/from-contract-to-published-library/closeout-and-durable-audience/published-tree-delta-statement/**
.bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md
```

**In this PR**

- The `## DoD 13 — the published-tree delta` section appended to `_closeout-record.md`: the tag and its SHA, the enumerated commit range with an owner per commit, the cited published-surface reason, and the explicit statement of what DoD 13's literal phrase cannot mean here and what was observed instead.
- This story's `_ledger.md`, one row per AC-###, each citing the artefact and the commit that produced it (`.redkiln/config.yaml:67`, `:73`).
- Any escalation this story raises, written where `findings-disposition-register` will pick it up — stated as a finding with its owning sibling named, never as a fix.

**Explicitly not in this PR**

- Any change to a crate, to `spec/SPECIFICATION.md`, to `CHANGELOG.md`, or to any `.kb/` atom.
- Re-running the public-surface comparison or `cargo-semver-checks` — that is `retention-and-incomplete-logs`'s (`.../retention-and-incomplete-logs/_storymap.md:78`).
- Re-stating or revising the registry positioning, the MSRV promise or the compliance claim — `publication-and-positioning` (HS-P0016), out of scope at `project.md:108-111`.
- The fourteen-row DoD 1–12/14–15 table — the slice-mate's, and duplicating it here splits the artefact the slice exists to keep whole.
- Fixing anything the delta surfaces, including a published-surface change. Route it.
- Re-running `cargo xtask ci`. The gate run and its SHA are `whole-gate-green-on-the-assembled-tree`'s; this story consumes the SHA it recorded.

**Merge DoD one-liner**: `_closeout-record.md` carries a DoD 13 delta section in which the tag, every commit in the range, every commit's owner and the cited surface reason are all present and checkable by a reader who opens nothing else, and the slice-mate's DoD 13 position points at it.

The implementer may also touch `_closeout-record.md` — the mount point named above — to append that section; that is the mounting, not scope drift.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **Resolve the published tag** | Find the annotated tag `publication-and-positioning` created for `0.2.0` and record **both** its name and the commit SHA it points at. Do not assume the tag string — read it. `git tag --list` is empty in the planning tree, so the tag is a real upstream artefact this story looks up, never a literal copied from a spec. | `.bklg/.../publication-and-positioning/publish-0-2-0/`; `.bklg/.../_decomposition.md:186` (position 7 ships `0.2.0`) |
| **Tag absent or ambiguous → finding, not invention** | If no tag resolves, or several plausibly do, the section records that fact with what was searched, and an escalation is raised naming HS-P0016 as owner. The delta section still lands, stating what it could not resolve. A placeholder SHA is a fabricated record. | `project.md:237-240` (AC-013, routing is the deliverable); `.redkiln/config.yaml:5` (`support_initiative`) |
| **Fix the closeout end of the range to the gate's SHA** | The far end is the commit `whole-gate-green-on-the-assembled-tree` recorded as the tree `cargo xtask ci` exited zero on — not `HEAD` at authoring time. If they differ, both are named and the divergence explained. | `project.md:192-195` (AC-001 records the SHA); `_storymap.md:54` |
| **Enumerate the commit range** | `git log --oneline <tag>..<gate-sha>` (or equivalent), listed in full or summarised by count with the full list in this story's folder if it is long. The count is stated either way, so a reader can tell whether the table is complete. | `.bklg/.../closeout-and-durable-audience/_decomposition.md:47` (the brief's stated procedure) |
| **Attribute every commit to an owning project** | Each commit maps to `replication-identity-and-ingest` (HS-P0017), `retention-and-incomplete-logs` (HS-P0018), or **something else, named**. The expected two owners come from DR-4; they are the hypothesis, not the filter. Zero unattributed commits. | `project.md:140-143` (DR-4); `.bklg/.../_decomposition.md:178-193` (merge order 7 → 8 → 9 → 10) |
| **Classify residue** | A commit owned by neither expected project is classified: backlog/planning-only, docs-only, or code. A code commit outside HS-P0017/HS-P0018 in this range is escalated with its owning sibling named. | `project.md:237-240`; `_storymap.md:62` |
| **State the published-surface reason, cited** | Two citations, both to artefacts produced elsewhere: the gate decision that constrained retention's answer to what needs no published-surface change, and the surface-diff record that shows how it came out. Neither is re-derived here. | `.bklg/.../_decomposition.md:239-249`; `.../retention-and-incomplete-logs/_storymap.md:78`; `.../replication-identity-and-ingest/_storymap.md:58` |
| **Surface changed → escalate, never absorb** | If the cited evidence shows a published-surface change (a `0.3.0` the exit criteria do not contemplate), the section names the surface, the version consequence and the owning sibling, and hands it to `findings-disposition-register`. No fix, no re-version, no edit to a sibling's record, no `[FROZEN]` clause touched. | `project.md:298` (risk row); `project.md:179-183` (DR-12); `.kb/decisions/README.md` (supersede, never edit) |
| **Qualify DoD 13 explicitly** | The section states in its own words that "the exact tree that was published" is not satisfiable on this tree and why, and states what was observed instead: `cargo xtask ci` green on the assembled whole from a clean checkout, with the delta from the tag bounded and attributed. DoD 13 is never marked green on an unqualified byte-identity claim. | `initiative.md:396-397` (the clause); `initiative.md:356-358` (the DoD preamble); `project.md:77-81` (*The honest caveat on DoD 13*) |
| **Mount beside the fourteen-row table** | The section lives in `_closeout-record.md` under a stable heading, and the slice-mate's table names that heading at the DoD 13 position so a reader following the table lands here. Delivered mounted: a delta statement reachable only from this story's own folder has not been delivered. | `_storymap.md:26-30` (convergence on one artefact); `_storymap.md:75-77` (one artefact, two halves) |
| **Interfaces consumed** | Read-only: `git tag`, `git log`, `git show` against the closeout tree; the sibling backlog artefacts named above, by path. **Interfaces produced**: one markdown section at a stable heading in `_closeout-record.md`, plus this story's `_ledger.md`. No code, no CLI, no schema. | `.redkiln/config.yaml:67`, `:73` |
| **No `redkiln advance` / `new` here** | The implementing agent authors bodies; every stage transition and item creation belongs to the driving command. Item frontmatter is never hand-edited. | `CLAUDE.md` (*the CLI is the only writer of an item's system frontmatter*) |

## Data and migrations

**N/A.** This story adds, changes and removes no schema, no table, no on-disk format and no serialised type. It writes one markdown section into an existing backlog artefact and reads the git commit graph.

Two adjacent things it is worth being explicit are *not* migrations, because both look like data changes from a distance:

- **The `.kb/` knowledge base is untouched.** No atom is authored, amended or superseded by this story. If the delta produces something durable enough to belong in `.kb/`, it arrives through `/redkiln:kb-ingest` at initiative closeout, never hand-written here (`project.md:162-165`, DR-8; the reverted commit `0269720`).
- **The published crate versions are untouched.** No `Cargo.toml` version is edited and no tag is created or moved. The `0.2.0` tag is read; a version consequence discovered in the delta is *reported* with its owner, and re-versioning is HS-P0016's, out of scope at `project.md:108-111`.

## Acceptance criteria

Framed from the reader this project exists for: the one-shot reader of a *closed* initiative,
who must be able to believe it closed honestly without re-deriving the evidence
(`_storymap.md:14-22`). That reader's mechanism is Persona 4's — "time-boxed and one-shot",
"whatever they spend reading a registry page and a README once"
(`.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md:249-256`,
*Cross-persona tensions*) — applied to the closeout record instead of the registry page. They do
not get a second pass, so anything they must open a sibling project's backlog to learn, they will
not learn.

This story is **static / process** tier throughout (`.bklg/.../closeout-and-durable-audience/_decomposition.md:47`:
"Not a test-framework check — a checked, cited artefact"). "Verification" below therefore means a
command a reviewer re-runs on the closeout tree and compares against the recorded transcript, not a
`#[test]`. Every transcript lands in this story's own companion,
`.bklg/.../published-tree-delta-statement/_delta-log.md`, which the PR boundary's first glob already
covers.

| id | criterion | verification |
| --- | --- | --- |
| **AC-001** | **GIVEN** a reader who has just been told the initiative shipped `0.2.0`, **WHEN** they read the near end of the delta, **THEN** they find a tag they can check out for themselves: the tag string exactly as it exists in this repository and the commit SHA it points at, both *read from the tree* rather than copied from a spec — and if no tag resolves, or several plausibly do, the section says so, states what was searched, and raises a finding naming HS-P0016 as owner instead of rendering a plausible SHA. | `git tag --list` and `git rev-parse <tag>^{commit}` (plus `git show <tag>` for an annotated tag's message) re-run on the closeout tree by a reviewer and compared against the transcript in `.bklg/.../published-tree-delta-statement/_delta-log.md`. There are no tags in this repository at planning time, so a tag string that appears without a transcript is a fabrication (`.bklg/.../publication-and-positioning/publish-0-2-0/spec.md:267`). |
| **AC-002** | **GIVEN** a reader who wants to know *which* tree the gate was green on, **WHEN** they read the far end of the delta, **THEN** it is the commit SHA `whole-gate-green-on-the-assembled-tree` recorded as the tree `cargo xtask ci` exited zero on — never `HEAD` at authoring time — and the section states that SHA, whether the tag is an ancestor of it, and the **count** of commits in the range, so the reader can tell whether the table below it is complete rather than curated. | `git merge-base --is-ancestor <tag> <gate-sha>` (exit code recorded) and `git rev-list --count <tag>..<gate-sha>` compared against the count stated in the section and against the SHA recorded by AC-001 of `project.md:192-195`. A stated count that does not equal `rev-list --count` fails. |
| **AC-003** | **GIVEN** a reader who was told "two projects landed after publication", **WHEN** they read the commit table, **THEN** every commit in the range carries an owner — `replication-identity-and-ingest` (HS-P0017), `retention-and-incomplete-logs` (HS-P0018), or **something else, named** — with zero unattributed rows and zero commits omitted; residue is classified as backlog/planning-only, docs-only, or code; and a **code** commit owned by neither expected project is escalated with its owning sibling named. DR-4's two owners are the hypothesis being tested, never the filter applied. | `git log --oneline <tag>..<gate-sha>` re-run and matched row-for-row against the section's table (row count equals AC-002's stated count); `git show --stat <sha>` for every commit classified as residue, checked against the classification given. Any row whose owner cell is blank, "misc", or absent fails. |
| **AC-004** | **GIVEN** a reader deciding whether the API they may already have built against moved under them, **WHEN** they read the delta's published-surface claim, **THEN** it is carried by two citations to work done elsewhere and is checkable in one hop each: the gate decision constraining retention's answer to what needs no published-surface change (`.bklg/from-contract-to-published-library/_decomposition.md:239-249`), and the surface-diff record showing how that came out (`.bklg/.../retention-and-incomplete-logs/_storymap.md:78`), with the byte-identical `EventStore` signature commitment beside it (`.bklg/.../replication-identity-and-ingest/_storymap.md:58`). No surface diff, `cargo-semver-checks` run or version class is re-derived here. | Citation resolution: each cited path opens on the closeout tree and the cited lines say what the section claims they say (a reviewer reads all three). Plus a negative check — `git diff --stat <tag>..<gate-sha> -- crates/` appearing in this story's own commits, or any `cargo-semver-checks` invocation, is a scope failure, because that work belongs to HS-P0018. |
| **AC-005** | **GIVEN** a reader who has read DoD 13's actual words, "on the exact tree that was published" (`initiative.md:396-397`), **WHEN** they reach DoD 13 in the closeout record, **THEN** they find that phrase explicitly qualified in the record's own words: that byte-identity is not satisfiable on this tree and *why* (merge positions 7 → 8 → 9 → 10, `_decomposition.md:178-193`), and what was observed **in its place** — `cargo xtask ci` green on the assembled whole from a clean checkout, with the delta from the tag bounded and attributed. Nowhere in the artefact is DoD 13 marked green on an unqualified byte-identity claim. | Read of `_closeout-record.md`: the qualifying statement is present in the DoD 13 section, and a search of the whole artefact finds no DoD 13 pass mark that is not adjacent to it. The section is self-contained on this point — a reader who opens nothing else can state what DoD 13 could and could not mean here (`initiative.md:354-358`, the DoD preamble; `project.md:298`). |
| **AC-006** | **GIVEN** the same reader working down the fourteen-row DoD table, **WHEN** they reach the DoD 13 position, **THEN** one pointer takes them to `## DoD 13 — the published-tree delta` **in the same `_closeout-record.md`** — appended, with no earlier section re-worded, re-ordered, truncated or displaced — and every escalation the delta raised is written where `findings-disposition-register` (HS-S0134) picks it up, with **zero** defects fixed inside this story. | Read of `_closeout-record.md` for the exact heading; `git diff` of this story's commits over `_closeout-record.md` shows additions only below existing content, no deletions or modifications to prior sections; the slice-mate's DoD 13 position names that heading; and `git diff --stat` for this story's commits touches nothing outside the two PR-boundary globs (`_storymap.md:26-30`, `:62`, `:75-77`; `project.md:237-240`). |

Coverage: this story traces to project **AC-004** alone (`project.md:204-206`, `_storymap.md:119`).
AC-001…AC-003 carry its four named things (the tag, the commits, the responsible projects);
AC-004 carries its "reason the published surface is unchanged"; AC-005 carries the word
"not glossed"; AC-006 makes the whole thing reachable and keeps the finding routed rather than
absorbed. It contributes evidence to project AC-013 without owning it.

## Interaction quality

This story renders **no** screen and no public API item. The signed-off design
(`.bklg/.../closeout-and-durable-audience/_design.md`) records N/A in every section and its
sign-off is the *no-surface determination itself*, approved 2026-08-12. So the composition family
below is not skipped — it is discharged against the one surface this story does compose: a section
of a markdown record that a human reads once, top to bottom, with no second pass. The invariants
are real and blocking; they are simply documentary rather than visual.

Every invariant here is carried by an `AC-###` **row in the table above**. Nothing in this section
is a free-floating bullet, because a bullet here would get no ledger row and never be gated.

**State family.**

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **In place, not a context jump** — the delta lands in the artefact the reader is already in, not in this story's folder with a link out. A reader who must open a sibling project's backlog to learn whether the published API moved has been context-jumped, and the story failed (`_storymap.md:26-30`). | AC-006 (mount), AC-004 (the claim travels as a citation the reader can *choose* to follow, with the answer stated inline) | heading present in `_closeout-record.md`; the surface claim is legible without opening the two cited artefacts |
| **Non-occlusion** — appending DoD 13 does not displace, truncate or re-word the fourteen-row table or any earlier story's section of the record. | AC-006 | `git diff` over `_closeout-record.md` shows additions only; zero deletions or modifications above the new heading |
| **Preserved position** — earlier stories' evidence keeps its heading text and its order, so a citation written against the record before this story still resolves after it. | AC-006 | same diff; every pre-existing heading string is unchanged |
| **Reversibility** — every act in this story is an ordinary commit that reverts cleanly. Nothing irreversible is performed: no tag created, moved or deleted; no version edited; no sibling's record amended; no `.kb/` atom written (an accepted atom cannot be edited at all — `.kb/decisions/README.md`). | AC-001 (tag is *read*), AC-006 (no fixes, no sibling edits), NF-001 | `git tag --list` identical before and after this story's commits; PR-boundary check |
| **Reachable in one hop** — the keyboard-reachability analogue for a document: from the DoD 13 position in the table, exactly one pointer reaches the section; from the section, exactly one hop reaches each piece of upstream evidence. No chain of three. | AC-006, AC-004 | the slice-mate's DoD 13 position names the heading; each citation is a path (and line range) that opens directly on the claim |

**Composition family** (from `_design.md`'s no-surface determination, applied to the documentary
surface this story does compose).

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **Presentation exists at all** — the delta is *composed*, not asserted. It is a named section with a stable heading, a tag/SHA pair, an enumerated commit table with a stated count, and two citations. Bare markup here is the single sentence "the trees differ slightly but the public API is unchanged", which satisfies every structural check and tells the reader nothing checkable. | AC-001, AC-002, AC-003, AC-004 | each of the four named things is separately present and separately re-runnable |
| **Placement** — under one `##` heading in `_closeout-record.md`, immediately readable after the fourteen-row table, because the delta exists to qualify the phrase that table is answering (`_storymap.md:75-77`). | AC-006 | heading position relative to the slice-mate's table |
| **Transience** — persistent, not revealed and not opened on demand. The tag, the count, the owners and the two citations are *always visible* in the record; only the full per-commit enumeration may be deferred to `_delta-log.md`, and only above the density budget below, and only with the count stated in place so nothing is hidden silently. | AC-002 (count stated), AC-003 (owners visible), AC-004 (citations visible) | a section that defers the count, the owners or the citations to a companion file fails |
| **Density budget (real numbers)** — the section is **one screen**: a target of ≤ 60 lines in `_closeout-record.md`. The commit table renders **in full up to 25 rows**; above that it renders the count, the per-project subtotals and the residue rows in full, with the complete enumeration in `_delta-log.md`. The fourteen-row table stays **fourteen** rows — DoD 13 is not added to it as a fifteenth. Four named things, no fifth. *(These numbers are set by this spec, from the front half's one-screen goal in Context pack item 8; no upstream artefact states them.)* | AC-003 (the overflow rule is the enumeration's, and the subtotals plus residue are never elided), AC-006 (fourteen rows stay fourteen) | line count of the appended section; row count of the slice-mate's table |
| **Hierarchy** — the qualification of DoD 13 leads; the tag and range follow; the attribution table follows that; the surface citation closes. The reader's first question ("can I believe the ✅?") is answered before the evidence that supports it, because a one-shot reader who stops early must still leave with the honest answer. | AC-005 (the qualification is the section's lead, not a footnote) | read order of the section |
| **Named anti-patterns** — three, all fatal: (a) the unfalsifiable gloss, "the closeout tree differs slightly from the published tree, but the public API is unchanged", citing nothing; (b) a table that silently drops the commits that did not fit the story's expected two owners; (c) `DoD 13 ✅` beside a tree that is not the published tree, with the qualification a paragraph away or absent. `_design.md`'s own *Anti-patterns* section is N/A for API items and remains so — these are this surface's. | (a) → AC-004, (b) → AC-003, (c) → AC-005 | the three checks in the AC table's verification column |

## Error conditions

| id | condition | required behaviour |
| --- | --- | --- |
| **EC-001** | No tag resolves for `0.2.0` on the closeout tree — HS-P0016 published without tagging, or under a name nobody predicted. | The section still lands. It records what was searched (`git tag --list`, and `git log --grep` over the publish commit), states that the near end is unresolved, and raises a finding naming HS-P0016 as owner, routed to `findings-disposition-register`. No SHA is invented and no "approximate" starting commit is substituted (`project.md:237-240`). |
| **EC-002** | Several tags plausibly denote the release (`0.2.0` and `v0.2.0`; or a tag that was moved). | Record every candidate with its SHA, state which was used as the near end and why, and raise a finding. `publish-0-2-0/spec.md:267` says there is no tag convention in this repository to inherit and the chosen one is recorded in `_release-log.md` — so read that log rather than guessing, and if it disagrees with the tags on disk, that disagreement *is* the finding. |
| **EC-003** | `HEAD` at authoring time has moved past the gate's recorded SHA — typically because this slice's own backlog commits landed in between. | Name both SHAs, state the divergence and its cause, and use the **gate's** SHA as the far end. Using `HEAD` would describe a tree no gate was ever run against, which is the failure DoD 13's whole subject forbids (Context pack item 3; `project.md:192-195`). |
| **EC-004** | The range is empty — the tag's SHA *is* the gate's SHA. | Then DoD 13's literal phrase **is** satisfiable, and the record says exactly that instead of manufacturing a qualification. The tag, the SHA and the zero count still land, and AC-005's statement becomes "byte-identity holds, and here is the evidence" rather than an excuse. A qualification written against an empty delta is its own dishonesty. |
| **EC-005** | A commit in the range belongs to neither HS-P0017 nor HS-P0018 **and** touches `crates/`. | Escalate. Name the commit, the crate and the paths it touched, and the sibling that owns it; state whether it touched a `pub` item. No fix, no revert, no re-version (`project.md:298`; DR-12 at `project.md:179-183`). |
| **EC-006** | The cited evidence shows the published surface **did** change — a `0.3.0` this initiative's exit criteria do not contemplate. | The section names the surface, the version consequence and the owning sibling, and hands the escalation to `findings-disposition-register`. This project reports; it does not absorb. Nothing in `spec/SPECIFICATION.md` is edited, and a `[FROZEN]` clause implicated by the change takes a new ADR and a re-plan, never an edit here (`CLAUDE.md`; `.kb/decisions/README.md`). |
| **EC-007** | The surface-diff record AC-004 must cite does not exist, or `surface-diff-and-the-ac-012-escalation` did not run. | The citation cannot be made and is not faked. Record the absence as a finding against HS-P0018 and state plainly that the published-surface claim is **unsupported on this tree**. Substituting this story's own diff is the scope failure AC-004's negative check exists to catch. |
| **EC-008** | `_closeout-record.md` does not exist — `clean-checkout-harness` has not landed. | Halt and report a dependency failure. Do not create a parallel artefact in this story's folder: an artefact per story is precisely the "fourteen ledger entries in fourteen places" failure the convergence exists to prevent (`_storymap.md:26-30`). |
| **EC-009** | The tag's commit is not an ancestor of the gate SHA — history was rebased or rewritten between publish and closeout. | Record the `git merge-base --is-ancestor` exit code, state that `<tag>..<gate-sha>` is therefore not a linear delta, give both `A..B` and `A...B` counts, and escalate. A two-dot range over rewritten history quietly under-reports, which reads as a smaller delta than there is. |
| **EC-010** | The enumeration exceeds the 25-row budget. | The count, the per-project subtotals and every residue row stay in `_closeout-record.md`; the full list moves to `_delta-log.md` with the section pointing at it. Truncating the table without stating the count is anti-pattern (b). |

## Non-functional

| id | requirement | how it is held |
| --- | --- | --- |
| **NF-001** | **Read-only against git and the registry.** No tag is created, moved or deleted; no `Cargo.toml` version is edited; no crate, `spec/`, `CHANGELOG.md` or `.kb/` file is touched; no network call is made. | `git tag --list` identical before and after; the PR boundary's two globs; `cargo xtask affected --base main` (`.redkiln/config.yaml:40`) maps this diff to no package and still runs the five file-reading lints and `spec-trace`, so a backlog-only story is not green by compiling nothing. |
| **NF-002** | **Every number is reproducible.** Each figure in the section is accompanied — in `_delta-log.md` if not inline — by the command that produced it, so a reviewer can re-derive it rather than trust it. This is DR-3's rule ("a scenario marked from the memory of the project that produced it does not count") applied to a delta instead of a scenario. | reviewer re-run of AC-001…AC-003's commands |
| **NF-003** | **Self-contained on one read.** Tag, SHA, count, owners, the two citations and the qualification are all present in `_closeout-record.md` itself. Only the overflow enumeration may live elsewhere (EC-010). | AC-005 / AC-006 verification; the density budget |
| **NF-004** | **Idempotent.** Re-running the whole procedure on the same tree reproduces the same section, modulo nothing. A second run that yields a different range means the far end was `HEAD` (EC-003). | re-run comparison |
| **NF-005** | **No stage transitions, no frontmatter edits.** The implementing agent authors bodies only; `redkiln advance` / `redkiln new` belong to the driving command, and item system frontmatter is written by the CLI alone (`CLAUDE.md`; a `PreToolUse` hook denies the edit). | hook; `redkiln validate` |
| **NF-006** | **Wall-clock: minutes, not the gate's hours.** This story runs `git` and reads markdown. It must not re-run `cargo xtask ci` (`.redkiln/config.yaml:60`) — that run belongs to `whole-gate-green-on-the-assembled-tree` and re-running it here would produce a *second* SHA and a second answer to the question DoD 13 asks once. | PR boundary; AC-002 consumes the recorded SHA |

## Implementation notes (non-prescriptive)

Not a prescription — the shape that has already survived the constraints above.

- **Resolve the two ends first, and write them down before anything else.** `git tag --list`,
  then `git rev-parse <tag>^{commit}`, then the gate SHA from `_closeout-record.md`'s AC-001
  entry. If either end is unresolved, stop and write EC-001/EC-008 rather than proceeding with a
  guess — everything downstream is a function of these two strings.
- **Attribute with `git log`, not with memory.** `git log --oneline --no-merges <tag>..<gate-sha>`
  for the enumeration; then, per commit, whichever of `git show --stat`, the commit trailer, or the
  `.bklg/` path it touched actually identifies the owner. Attributing by *subject line prefix* alone
  is how residue gets misfiled into the expected two owners, which is exactly anti-pattern (b).
- **Compute the residue set explicitly.** The set difference between "all commits in range" and
  "commits attributed to HS-P0017 ∪ HS-P0018" is a number worth writing down even when it is zero,
  because a stated zero is checkable and an unstated one is indistinguishable from a filter.
- **Write the qualification before the evidence.** The one-shot reader who stops after two
  paragraphs must still leave with the honest answer (hierarchy invariant). Drafting the evidence
  first tends to produce a section whose lead is a commit table and whose conclusion is buried.
- **Keep the two halves of the slice in one editing session.** The slice-mate's DoD 13 position and
  this heading are a matched pair; writing the pointer in a different sitting from the target is how
  the pointer ends up naming a heading that was later renamed.
- **When something is wrong, the deliverable is the sentence that says so.** This project's own risk
  table calls it the project "most tempted to absorb a fix" (`project.md:298`, and the row below it).
  The escalation text — surface named, version consequence stated, owning sibling named — *is* the
  work product, not a note attached to a fix.

## Tests and CI (merge gate)

Grounded in the testing brief's tier assignment for AC-004: **static / process**, "not a
test-framework check — a checked, cited artefact"
(`.bklg/.../closeout-and-durable-audience/_decomposition.md:47`). No Rust test is written and no
crate is compiled by this story's own change.

| tier | command / path | proves |
| --- | --- | --- |
| Static / process | `git tag --list`; `git rev-parse <tag>^{commit}`; `git show <tag>` — transcripts in `.bklg/.../published-tree-delta-statement/_delta-log.md`, re-run by the reviewer | **AC-001** — the near end is a real tag on this tree, not a string from a spec |
| Static / process | `git merge-base --is-ancestor <tag> <gate-sha>`; `git rev-list --count <tag>..<gate-sha>` against the count stated in the section and the SHA at `project.md:192-195` | **AC-002** — the far end is the tree the gate was green on, and the enumeration is complete |
| Static / process | `git log --oneline --no-merges <tag>..<gate-sha>` matched row-for-row against the section's table; `git show --stat <sha>` per residue commit | **AC-003** — every commit has an owner and the residue is classified, not dropped |
| Static | citation resolution over `.bklg/from-contract-to-published-library/_decomposition.md:239-249`, `.bklg/.../retention-and-incomplete-logs/_storymap.md:78`, `.bklg/.../replication-identity-and-ingest/_storymap.md:58`; plus the negative check that no `cargo-semver-checks` run or `crates/` diff appears in this story's commits | **AC-004** — the surface claim is cited from where it was proved, and not re-derived here |
| Static | read of `.bklg/.../closeout-and-durable-audience/_closeout-record.md` for the qualification, and a search of the whole artefact for an unqualified DoD 13 pass mark | **AC-005** — DoD 13 is qualified, never marked green on byte-identity |
| Static | read of `_closeout-record.md` for the `## DoD 13 — the published-tree delta` heading; `git diff` over that file showing additions only; the slice-mate's DoD 13 position naming the heading; `git diff --stat` confined to the two PR-boundary globs | **AC-006** — mounted where the reader already is, occluding nothing, with findings routed and nothing fixed |
| Story merge gate | `cargo xtask affected --base main` (`.redkiln/config.yaml:40`) | the diff maps to no workspace package; the five file-reading lints and `cargo xtask spec-trace` still run, so this story cannot pass by compiling nothing (`.redkiln/config.yaml:36-39`) |
| Story merge gate | `redkiln verify --grain story` over `.bklg/.../published-tree-delta-statement/_ledger.md` (`.redkiln/config.yaml:67`, `:73`) | every AC-### above carries cited, non-placeholder evidence and a recorded work commit |
| Backlog / KB health | `redkiln validate` and `redkiln doctor` | the appended markdown breaks no item and adds no seventh `template-drift` advisory (`.github/workflows/ci.yml:142-187`) |
| **Not run here** | `cargo xtask ci` (`.redkiln/config.yaml:60`) | consumed from `whole-gate-green-on-the-assembled-tree`, never re-run: NF-006, and a second run would produce a second SHA for a question DoD 13 asks once |

## Risks and coupling (PR-scoped)

| risk | why it bites here | containment |
| --- | --- | --- |
| **The tag does not exist yet.** `git tag --list` is empty in the planning tree; the whole near end of this story is an artefact HS-P0016 creates five projects upstream. | A story written against an assumed tag string renders confidently and wrongly. | AC-001 makes *reading* the tag the criterion; EC-001/EC-002 make its absence or ambiguity a finding with a stated owner. |
| **`HEAD` drift inside this very slice.** This story's own commits, and the slice-mate's, land after the gate SHA. | Using `HEAD` silently widens the range and describes a tree the gate never saw — an error that is invisible in the rendered output. | AC-002 fixes the far end to the recorded SHA; EC-003 requires the divergence to be named. |
| **The pull to re-derive the surface diff.** It looks cheap, and it is the most satisfying part of the question. | Two projects would then answer the same question, and can disagree; the version class is HS-P0016's and HS-P0018's, out of scope at `project.md:108-111`. | AC-004's negative check; the PR boundary's "explicitly not in this PR" list. |
| **The pull to fix residue.** A stray code commit in the range is a one-line finding and, often, a one-line fix. | This project's own risk table names it the project most tempted to absorb a fix; a story that repairs what it found has failed its own criterion (`_storymap.md:92-96`). | AC-006's zero-fixes clause; EC-005/EC-006 make the escalation the deliverable. |
| **Slice-mate coupling in both directions.** AC-006 needs the slice-mate's table to point at this heading; the slice-mate needs this section to exist for its DoD 13 position to mean anything. | Landing the two in separate sittings orphans one end of the pointer. | The slice is implemented in one context by construction (`_storymap.md:75-77`); the heading string is fixed in this spec so both halves name the same literal. |
| **A long delta.** If HS-P0017 and HS-P0018 were busy, the range may be large enough that a full table swamps the record. | The tempting response is a summarised table — which is the gloss AC-004 forbids, wearing a table (Context pack item 4). | The density budget: 25 rows, then subtotals plus residue in place and the full list in `_delta-log.md`, count always stated (EC-010). |
| **Downstream blast radius is one story.** `findings-disposition-register` (HS-S0134) consumes whatever this raises. | An escalation written vaguely here becomes an unroutable row there. | Every escalation names the surface, the version consequence and the owning sibling (EC-005, EC-006). |

## Dependencies

**Blocks on**

- **`dod-set-re-observation-record`** (HS-S0127) — the slice-mate and this story's only
  `blocked_by` edge (`story.md` frontmatter). Its fourteen-row table is the surface this section
  qualifies, and its DoD 13 position is the pointer AC-006 requires. Transitively it carries
  `whole-gate-green-on-the-assembled-tree` (the gate SHA that is this delta's far end) and
  `clean-checkout-harness` (which stands up `_closeout-record.md`).

**Unlocks**

- **`findings-disposition-register`** (HS-S0134) — this story is one of its six named inputs
  (`_storymap.md:62`; `story.md` frontmatter `blocks`). Every escalation raised here is routed
  there, never fixed here.

**Consumes, without an edge** (upstream projects, complete before this project's rank-6 merge
position): `publication-and-positioning` (HS-P0016) for the tag;
`replication-identity-and-ingest` (HS-P0017) and `retention-and-incomplete-logs` (HS-P0018) for
the commits in the range and for the surface-diff record AC-004 cites.

## Anchors (progressive disclosure)

Load-bearing depth, deferred rather than pasted. The Context pack above is sufficient to start;
open these at the moments named.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_decomposition.md` (the testing brief, AC-004 row at `:47`) | Fixes this story's tier as **static / process** and states the procedure verbatim: `git log <published-tag>..HEAD` filtered to HS-P0017/HS-P0018, "not a test-framework check — a checked, cited artefact". Reading it prevents the single most expensive wrong turn: writing a Rust test. | First, before anything else. | AC-001, AC-002, AC-003 |
| `.bklg/from-contract-to-published-library/publication-and-positioning/publish-0-2-0/spec.md` (`:267` the tag row, `:290` the `_release-log.md` ordering, `:286-288` the version root) | The only place that says what the tag *is*: annotated, on the publish commit, created after the crates are live, with no convention in this repository to inherit and the chosen name recorded in `_release-log.md`. Resolving the near end without this invites guessing `v0.2.0`. | Before resolving the tag; again if EC-002 fires. | AC-001 |
| `.bklg/from-contract-to-published-library/closeout-and-durable-audience/project.md` (`:139-143` DR-4, `:192-195` AC-001's recorded SHA, `:204-206` AC-004, `:237-240` AC-013, `:298` the DoD 13 risk row) | Carries the two owners as a hypothesis, the exact provenance of the far-end SHA, and the sentence that makes reporting rather than absorbing binding: "this project reports it — it does not absorb a `0.3.0` the exit criteria do not contemplate." | Before enumerating the range; again the moment anything looks broken. | AC-002, AC-003, AC-006 |
| `.bklg/from-contract-to-published-library/_decomposition.md` (`:178-193` merge order, `:239-249` *Decisions taken at the gate* item 4) | Item 4 is the *reason* the published surface should be unchanged — ES-39/ES-40 are `EventStore` clauses landing after `0.2.0`, so a real decision there would be a published-port change, a `0.3.0` outside the exit criteria; the runbook already blesses a written refusal, so the constraint cost nothing. The merge order is why byte-identity is impossible. | Before writing the surface claim, and before writing the DoD 13 qualification. | AC-004, AC-005 |
| `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/_storymap.md:78` | `surface-diff-and-the-ac-012-escalation` — where the surface comparison against the `0.2.0` registry baseline is actually run and every additive item enumerated, plus the escalation path if the honest answer needed a port surface. This is the citation, and the reason this story does not re-run the diff. | When writing AC-004's citation; and if EC-006 or EC-007 fires. | AC-004 |
| `.bklg/from-contract-to-published-library/replication-identity-and-ingest/_storymap.md:58` | `memory-store-ingest-seam` commits to an **inherent** `&self` operation with the `EventStore` trait signature byte-identical before and after — the second half of why HS-P0017's commits do not move the published surface. | Alongside the anchor above, when writing AC-004. | AC-004 |
| `.bklg/from-contract-to-published-library/initiative.md` (`:354-358` the DoD preamble, `:394-397` DoD 13) | The exact words being qualified — "on the exact tree that was published" — and the preamble insisting each DoD item is "run and observed to pass on the assembled library, from a clean checkout", with a green gate a precondition and never a substitute. AC-005 must quote the clause, not paraphrase it. | When drafting the qualification. | AC-005 |
| `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_storymap.md` (`:26-30` convergence on one artefact, `:56` this story's row, `:62` the findings consumer, `:75-77` one artefact two halves, `:92-96` "no story fixes anything") | Why the mount point is the shared record rather than this story's folder, and why routing rather than fixing is a slicing decision rather than a preference. | Before appending to `_closeout-record.md`; and before writing any escalation. | AC-006 |
| `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_design.md` | The signed-off no-surface determination, approved 2026-08-12, N/A in every section. It is the standing refusal that keeps this story from inventing an API item, a CLI flag or a tool where a markdown section is what was designed. | If any part of the work starts to look like code. | AC-005, AC-006 |
| `.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md:249-256` (Persona 4, and *Cross-persona tensions* at `:333-336`) | The reader the acceptance criteria are framed from: time-boxed, one-shot, no second pass. It is the evidence behind the one-screen budget and the "one hop" reachability invariant. | When judging whether the section is self-contained enough. | AC-005, AC-006 |
| `.redkiln/config.yaml` (`:40` the story grain, `:60` the terminal grain, `:67` `require_ledger`, `:73` `require_commit_provenance`, `:5` `support_initiative`) | The gate wiring this story is actually held to, including why a backlog-only diff is still gated (lints and `spec-trace` run unconditionally) and where incidental defects route. | When filling `_ledger.md` and before claiming the story is done. | AC-006 |
| `.kb/decisions/README.md` | Accepted atoms are immutable — supersede, never edit. The standing reason an escalation discovered here becomes a new atom and a re-plan rather than a correction to an existing record. | Only if EC-006 fires. | AC-004 |
| `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_grounding.md:18-48` | Seventeen atoms exist (`0001`–`0016`, `0029`); `0017`–`0028` are reserved and unwritten at planning time. Prevents citing an ADR number that does not exist when naming the decision behind an escalation. | Before citing any ADR number in this story. | AC-004 |

## Clarifications resolved during spec

1. **The six AC-### ids from the front half are unchanged.** AC-001…AC-006 as enumerated there;
   nothing added, nothing dropped. The mapping is: AC-001/AC-002/AC-003 carry project AC-004's
   "tag / commits / projects responsible", AC-004 carries its "reason the published surface is
   unchanged", AC-005 carries "not glossed", AC-006 carries delivery-mounted and finding-routed.
2. **AC-006 carries two clauses on purpose.** The front half already bound escalation routing to
   AC-006 ("that is an escalation routed under AC-006", *Integration contract*). Mounting is joined
   to it rather than given a seventh id because both are properties of the section's *edges* — the
   artefact it lands in and the register it hands findings to — and a seventh id would have
   contradicted the front half's own binding.
3. **A companion transcript file is introduced: `_delta-log.md`** in this story's own folder. The
   front half already contemplated it ("the full list in this story's folder if it is long") and the
   PR boundary's first glob already covers it. It holds the re-runnable command transcripts (NF-002)
   and the overflow enumeration (EC-010); it is never where the tag, the count, the owners or the
   citations live (transience invariant).
4. **The density numbers are set here, not inherited.** ≤ 60 lines for the section, 25 rows before
   the enumeration overflows, four named things, fourteen rows staying fourteen. No upstream artefact
   states them; they are derived from the front half's one-screen goal (Context pack item 8) and are
   labelled as this spec's own decision so a later reader does not go looking for a source.
5. **The composition family is answered, not skipped.** `_design.md` records N/A in every section
   and its sign-off is the no-surface determination itself. Rather than treat that as an exemption,
   the composition invariants are applied to the one surface this story does compose — a section of
   a markdown record read once, top to bottom — and each is carried by an AC row. An "unstyled
   render" here is the unfalsifiable gloss sentence, and anti-patterns (a)–(c) are what make it fail.
6. **The persona framing is explicit.** This project's audience is the reader of a closed
   initiative, which is not a product persona (front half, Context pack item 8). The criteria are
   nonetheless framed from a real referenced persona's *mechanism* — Persona 4's one-shot,
   time-boxed reading — because that is what makes "one hop" and "one screen" blocking rather than
   stylistic.
7. **EC-004 was added deliberately.** If the delta turns out to be empty, the honest record says
   byte-identity holds; it does not manufacture a qualification for a gap that is not there. A spec
   that only handles the non-empty case would push the implementer toward writing the caveat
   regardless of the evidence.
