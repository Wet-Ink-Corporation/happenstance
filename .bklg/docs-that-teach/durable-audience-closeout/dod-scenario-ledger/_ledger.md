---
item: HS-S0180
stage: implement
created: "2026-08-17T13:16:30.285Z"
updated: "2026-08-17T13:16:30.285Z"
---

# Acceptance ledger — Re-observe the fifteen DoD scenarios from a clean checkout

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Two notes specific to this story, because its evidence shape is unusual:

- **`evidence` must cite the fifteen-scenario re-observation companion at a real `file:line`**, not
  by bare filename. That citation *is* the mount (`spec.md`, `## Integration contract`;
  `_decomposition.md:429-436`) — a record in a story directory that nothing cites is this project's
  analogue of a component rendered into no tree, and AC-007 is the row that fails if it is missing.
- **This ledger is not the fifteen-scenario ledger.** The per-story `_ledger.md` that
  `require_ledger: true` (`.redkiln/config.yaml:67`) mandates is distinct from
  `dod-scenario-ledger`'s re-observation ledger (`_storymap.md:94-98`). This file carries seven
  rows, one per `AC-###`; that one carries sixteen data rows, one per DoD scenario with scenario 2
  as two halves.

```yaml
- id: AC-001
  criterion: "GIVEN U3, a future maintainer who wants to know against which tree this initiative was called done, WHEN they open the re-observation ledger, THEN its head names a checkout that is a fresh `git clone` or a new `git worktree add` off the merged branch and is demonstrably not `.claude/worktrees/docs-that-teach`, together with that checkout's absolute path and the sha it resolved to — AND that sha is the merged sha `merge-forward-baseline` produced and this story consumed as a value rather than derived (AC-A06), AND the checkout was created only after every artefact any scenario observes was committed on that branch, so the fixture could not observe an absence the branch does not have."
  satisfied: false
  evidence: "" # file:line and/or verifying test id — required once satisfied
  mount_point: ".bklg/docs-that-teach/durable-audience-closeout/dod-scenario-ledger/_ledger.md"
  verifying_test: "Tier 4 — `git rev-parse HEAD` and `git rev-parse --show-toplevel` in the fixture, recorded verbatim in the companion's head; Tier 1 — `git cat-file -t <sha>` is `commit` and `git log --oneline -1 <sha>` matches, run from the initiative worktree; Tier 2 — `git ls-tree -r <sha> --name-only` contains each row's observed artefact"

- id: AC-002
  criterion: "GIVEN U2, the closeout reviewer, who must be able to find the one observation they disagree with without reading the discovery corpus, WHEN they read the ledger end to end, THEN all fifteen initiative scenarios are present, none merged away and none silently dropped; fourteen carry a fresh observation with a named human observer, the checkout path and sha it was taken on, the instrument used, what was seen, and an outcome stated as a literal word; scenario 2 occupies two named halves (`failed by name`, `recovered`) that exist and are visibly pending its slice-mate rather than absent — AND each row additionally names the project that made the scenario true, as provenance — AND the string `inherited from a sibling` appears nowhere in the file."
  satisfied: false
  evidence: "" # file:line and/or verifying test id — required once satisfied
  mount_point: ".bklg/docs-that-teach/durable-audience-closeout/dod-scenario-ledger/_ledger.md"
  verifying_test: "Tier 1 — `rg -c \"inherited from a sibling\" .bklg/docs-that-teach/durable-audience-closeout/dod-scenario-ledger/` returns no match, and the companion's table carries sixteen data rows; Tier 2 — reviewer read of each row against `initiative.md:413-468` and the owner column at `.bklg/docs-that-teach/_decomposition.md:219-235`"

- id: AC-003
  criterion: "GIVEN U2, who knows that a claim nothing can falsify is decorative, WHEN they read the two mutation-shaped rows this story owns — scenario 4 (removing the boundary from the example makes it fail) and scenario 13 (nothing load-bearing is hidden from the check) — THEN each carries a captured red transcript from the check run against the deliberately broken state and a green transcript from the re-run after the revert, both verbatim rather than summarised as \"failed\" and \"green\"; OR, for scenario 13 only, the initiative's second admissible arm is shown — the folded, tabbed or collapsed content in the shipped material enumerated item by item with each one's claims read — never asserted in a sentence; AND neither mutation is present in any commit on any branch, so `git status` in the fixture is clean before anything is committed anywhere."
  satisfied: false
  evidence: "" # file:line and/or verifying test id — required once satisfied
  mount_point: ".bklg/docs-that-teach/durable-audience-closeout/dod-scenario-ledger/_ledger.md"
  verifying_test: "Tier 4 — the mutation/revert pairs run in the fixture, red and green transcripts captured under the companion's transcripts heading; Tier 1 — `git status --porcelain` empty in the fixture after each pair and `git log -p` on the branch containing neither edit; Tier 2 — for the shown-disjunct arm, the enumeration checked to name N items at real `file:line`"

- id: AC-004
  criterion: "GIVEN U2 reading the seven scenarios that have no command at all — 5, 6, 8, 9, 10, 12 and 14, plus the walk-shaped 3 and 7 — and knowing this story's own named wrong implementation is \"a ledger asserting fifteen scenarios pass, with evidence for the cheap ones only\", WHEN they read those rows, THEN every one names the instrument it used from the fixed vocabulary — command run, mutation, artefact read, reader walk — and carries evidence of that instrument's shape: a walk records its hops in order with each hop's landing point at a real file:line in the fixture, and a read cites the artefact at a real file:line in the fixture; AND no row's evidence is a sentence of confidence, a restatement of the scenario, or a citation of the owning project's own prior proof."
  satisfied: false
  evidence: "" # file:line and/or verifying test id — required once satisfied
  mount_point: ".bklg/docs-that-teach/durable-audience-closeout/dod-scenario-ledger/_ledger.md"
  verifying_test: "Tier 2 — reviewer read row by row against the instrument column, per `_decomposition.md:704-711` and the UX checklist at `:266-281`; Tier 1 — every file:line cited by a non-command row resolves in the fixture at the named sha (`test -f` plus a line-count bound); Tier 4 — the walks performed in the fixture"

- id: AC-005
  criterion: "GIVEN U2 checking the three rows whose evidence comes from somewhere other than this story's own reading — scenario 1 (the gate), scenario 11 (the sibling statement) and scenario 15 (this project's own output) — WHEN they read them, THEN scenario 1 carries both of DoD-1's claims: a `cargo xtask ci` run taken in the fixture with its result recorded, and the narrative build step located inside `xtask/src/main.rs`'s REQUIRED list on the merged tree, cited at the line it actually occupies with the search that found it recorded — and that run is explicitly not offered as AC-016's evidence, which is `terminal-gate-run`'s; scenario 11 cites `post-merge-clause-completeness`'s completeness statement at a real file:line, re-deriving no clause-id set and paraphrasing no verdict; and scenario 15 records `redkiln validate --kb` run in the fixture with its exit and output, plus the four mount points `product-layer-mounting` landed — the appended `##` in `.kb/maps/domain-map.md`, the reciprocal `related` / `depends_on` edges, `links.kb` on HS-P0025 and the `## Knowledge Harvest` row — read where they landed rather than from memory."
  satisfied: false
  evidence: "" # file:line and/or verifying test id — required once satisfied
  mount_point: ".bklg/docs-that-teach/durable-audience-closeout/dod-scenario-ledger/_ledger.md"
  verifying_test: "Tier 4 — `cargo xtask ci` (`.redkiln/config.yaml:60`) and `redkiln validate --kb` both run in the fixture, transcripts captured; Tier 3 — the four-mount-point walk from `.kb/README.md` and back from HS-P0025's `links.kb`, recorded in scenario 15's row; Tier 1 — `xtask/src/main.rs:105`'s REQUIRED list read at the merged sha, and scenario 11's cited file:line resolving inside `.bklg/docs-that-teach/durable-audience-closeout/post-merge-clause-completeness/`"

- id: AC-006
  criterion: "GIVEN U2 quoting one row of this ledger into a review comment, and U3 reading one row two years later with no other file open, WHEN either reads a single row in isolation, THEN it loses nothing: it names its scenario id, its owning project, its observer, its checkout and sha, its instrument, what was seen and its outcome, and depends on no row above it; AND every state is a literal word — no tick, emoji, colour, glyph, strikethrough, ordering or empty cell carries meaning anywhere in the file; AND the load-bearing verdict — whether the assembled tree meets the initiative's Definition of Done — is stated as a sentence in prose as well as carried by the table; AND the file is composed from the corpus's own primitives — one `#`, sections at `##`, no skipped level, every link naming its destination rather than \"here\" or \"see above\", transcripts in fenced blocks under their own heading rather than interleaved with verdicts — with no invented status vocabulary, no bespoke per-artefact table format and no new heading grammar."
  satisfied: false
  evidence: "" # file:line and/or verifying test id — required once satisfied
  mount_point: ".bklg/docs-that-teach/durable-audience-closeout/dod-scenario-ledger/_ledger.md"
  verifying_test: "Tier 2 — the presentation review of the rendered markdown against the a11y floor (`_decomposition.md:115-145`), AC-UX-09/10 (`:266-273`) and the density budget in `spec.md`, `## Interaction quality`; Tier 1 — `rg` over the companion for emoji, `~~` and `[ ]`/`[x]` used as status, plus a heading-level pass (exactly one `#`, no skipped level) and a link-text pass"

- id: AC-007
  criterion: "GIVEN the three downstream consumers this ledger exists for — `scenario-two-fault-injection`, which must fill scenario 2's two halves into a shape it did not design; `terminal-gate-run`, which runs the gate on the tree that already carries this file; and the initiative closeout, whose exit criterion is that every scenario was observed on the assembled result — WHEN any of them reaches for this ledger, THEN it is mounted: this story's `_ledger.md` cites the companion per criterion with a real file:line rather than a bare filename, so the record is reachable and not merely present; scenario 2's two slots are named and structurally identical to the fourteen owned rows, so the slice-mate fills rather than redesigns; every row is citable without paraphrase; AND where a re-observation came back red, the row records the finding with its owning project or its route to the `support` initiative and the finding is not fixed here; AND `git diff --name-only` lists no path outside `.bklg/docs-that-teach/durable-audience-closeout/dod-scenario-ledger/**`."
  satisfied: false
  evidence: "" # file:line and/or verifying test id — required once satisfied
  mount_point: ".bklg/docs-that-teach/durable-audience-closeout/dod-scenario-ledger/_ledger.md"
  verifying_test: "Tier 3 — the mount-point walk: open this `_ledger.md`, follow each row's evidence file:line, land on the claimed sentence or row in the companion, then confirm scenario 2's two slots carry the same columns as row 1; Tier 1 — `git diff --name-only` against the merge base lists only paths under the story glob, `git diff` shows no `-` line in any pre-existing file, and `cargo xtask affected --base main` (`.redkiln/config.yaml:40`) is green"
```
