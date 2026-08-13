---
item: "HS-S0084"
stage: implement
created: "2026-08-12"
updated: "2026-08-12"
---

# Acceptance ledger — The crate set for 0.2.0 is a recorded decision, not a default

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

This story lands **no Rust and no manifest change**, so no compiler stands behind most of these rows.
`verifying_test` therefore names the real command, the real gate step or the real diff assertion that
decides the row — the testing brief's process/static tiers for project AC-001 and AC-013
(`_decomposition.md`, Testing brief, *Test mix* rows for AC-001 and AC-013). Two `mount_point`s recur
because the story has two mounts: `xtask/src/package.rs` (the `PUBLISHABLE` constant, reached through
the mandatory `package-check` step) and the knowledge tree (`.kb/decisions/` plus its row in
`.kb/maps/decision-map.md`). An atom with no map row is an atom the corpus cannot see, which is why
the map — not the atom's own path — is the mount for the rows that prove reachability.

`00NN` stands for the ADR number the `/redkiln:kb-ingest` wave actually assigns (≥ 0030, free of the
0017–0028 sibling allocation and of 0029). The implementer replaces it with the assigned id when
citing evidence; see EC-004 in the spec.

```yaml
- id: AC-001
  criterion: "GIVEN the repository owner has decided once that `0.2.0` ships exactly `happenstance-core`, `happenstance` and `happenstance-testkit`, WHEN they (or any later reader) open `.kb/decisions/`, THEN an accepted decision atom at a number >= 0030 states that set — exactly three names, listed — in the decision-atom shape of record (`.kb/decisions/0029-msrv-raised-to-1-97-1.md`:1-33): valid `KbFrontmatter` with `kind: decision`, `authority_tier: decision`, `status: accepted`, `supersedes: null`, `superseded_by: null`, a `related` edge to `kb-decision-0006` and `source_paths` naming the intake file, `xtask/src/package.rs` and `RUNBOOK.md`; AND the atom defines the vocabulary it uses — intention list (`PUBLISHABLE`) and derived set (`cargo metadata`) — where it uses them rather than sending the reader elsewhere for the definition; AND the set is stated as three so the signed-off `<= 3 entries` disambiguation-triad budget (`_design.md`:474, region R3 at `:359`) is satisfiable by the downstream copy story without re-deciding anything"
  satisfied: false
  evidence: ""
  mount_point: ".kb/decisions/00NN-the-0-2-0-crate-set.md (the atom the ingest wave authors), reached from .kb/maps/decision-map.md"
  verifying_test: "redkiln validate --kb (exit 0, new atom present); content check of .kb/decisions/00NN-*.md for the three crate names, the six frontmatter fields and both inline definitions; git ls-files .kb/decisions/ showing exactly one added path whose number is >= 0030 and outside 0017-0028"

- id: AC-002
  criterion: "GIVEN `RUNBOOK.md`:4450-4451 states the phase-12 goal as four crates including `happenstance-sqlite`, WHEN the owner or a future planner reads the new atom, THEN an Alternatives rejected section quotes that four-crate goal verbatim, names it as the source being overridden, and carries its three costs unchanged from the deployment brief (`publication-and-positioning/_decomposition.md`:585-608) — the intention-list change, the `LICENSE-MIT` / `LICENSE-APACHE` / `README.md` plus status-row plus docs.rs scope a fourth crate owes (`xtask/src/package.rs`:88-94), and the phase-12 section's independently-known staleness — AND the cost is readable in the atom with the runbook citation as the only hop needed to check it (IQ-1's 0-hops-to-a-claim, <= 1-hop-to-evidence budget, `publication-and-positioning/_decomposition.md`:229-237)"
  satisfied: false
  evidence: ""
  mount_point: ".kb/decisions/00NN-the-0-2-0-crate-set.md — the Alternatives rejected section"
  verifying_test: "content check that the atom contains the verbatim four-crate goal string from RUNBOOK.md:4450-4451, the three named rejection grounds, and the sentence naming RUNBOOK.md as overridden; file:line resolution of every citation in that section against the tree at HEAD"

- id: AC-003
  criterion: "GIVEN the atom claims the intention list and the manifests agree, WHEN the owner needs that claim to be checked rather than trusted, THEN `cargo xtask package-check` has been run at the decision commit and its stdout is a committed artefact under this story's folder, carrying the commit sha, the date, the literal command line, the exit status, `reconcile`'s success line `publishable set agrees with the manifests: happenstance-core, happenstance, happenstance-testkit` (`xtask/src/package.rs`:213-217) and the three per-crate lines naming `LICENSE-MIT`, `LICENSE-APACHE` and `README.md`; AND the atom cites that artefact rather than restating `PUBLISHABLE`'s contents — no claim on it has run-the-gate-yourself as its only evidence (IQ-5)"
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/package.rs — PUBLISHABLE (:86) reached through the mandatory package-check step at xtask/src/main.rs:515-532, dispatched at :680"
  verifying_test: "cargo run -p xtask -- package-check (exit 0) with its output committed under .bklg/from-contract-to-published-library/publication-and-positioning/crate-set-decision/; cargo test -p xtask package::tests (xtask/src/package.rs:408-457) green"

- id: AC-004
  criterion: "GIVEN the named wrong implementation is an atom that reads correctly while nobody ran the check (`discover.md`:38-40), WHEN the check is seeded with the exact drift it exists to catch — `publish = false` deleted from `crates/happenstance-sqlite/Cargo.toml`:12 in the working tree only — THEN `cargo xtask package-check` fails, and its message names the direction as promoted and tells the reader to copy the three required files or restore the missing `publish = false` (`xtask/src/package.rs`:186-196); AND the seeded edit is reverted and appears in no commit; AND if the unseeded run had failed in either direction, the story stops and records it rather than editing `PUBLISHABLE` or a manifest to make the gate green (reversibility is bought before the irreversible act, not after it — IQ-4)"
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/package.rs — reconcile (:172-218), the promoted-direction message at :186-196"
  verifying_test: "seeded-failure transcript committed alongside AC-003's passing capture, showing a non-zero exit and the word promoted; git diff --quiet -- crates/ and git diff --quiet -- xtask/ at HEAD proving the seed was reverted"

- id: AC-005
  criterion: "GIVEN an accepted decision atom is immutable and the corpus is only navigable through its map, WHEN the wave that authors this atom lands, THEN `.kb/maps/decision-map.md` carries a row for it in ADR-number order under a new wave section with no superseded row deleted (`.kb/maps/decision-map.md`:81-86); AND `.kb/decisions/0001`…`0016` and `0029` are byte-identical to HEAD~, `xtask/src/package.rs` and every `Cargo.toml` are unmodified, and the diff contains only paths inside the PR boundary; AND `redkiln validate --kb` and `redkiln doctor` are clean, `doctor` reporting exactly the six expected `template-drift` advisories and no `dependency-cycle`; AND every sentence in the atom is true of the tree at that commit (IQ-7)"
  satisfied: false
  evidence: ""
  mount_point: ".kb/maps/decision-map.md — the new wave section's row for 00NN"
  verifying_test: "redkiln validate --kb && redkiln doctor (exit 0; six template-drift advisories, zero dependency-cycle); git diff --stat against the merge base bounded by the spec's PR boundary; cargo xtask affected --base main (.redkiln/config.yaml:40) green"

- id: AC-006
  criterion: "GIVEN the plan of record still states a four-crate goal, WHEN a reader arrives at `RUNBOOK.md`'s phase-12 Goal paragraph, THEN they meet a single added pointer line naming the atom as the source in force for the crate set — the stale sentence is annotated, not deleted (IQ-2's annotate, never subtract), no heading is renamed and no anchor moves (IQ-3), and no other region of `RUNBOOK.md` — the provisional falsifier ledger at `:622-635` above all — is entered; AND the three stories that block on this one (`registry-surface-diff`, `landing-copy-and-status-truth`, `publish-0-2-0`, `_storymap.md`:105) can quote the set from the atom without re-deriving it, which is what keeps the evaluator's first contact three pages and a three-answer triad rather than four"
  satisfied: false
  evidence: ""
  mount_point: "RUNBOOK.md — the phase-12 Goal paragraph at :4448-4460"
  verifying_test: "git diff RUNBOOK.md showing added lines only inside :4448-4460, no removed line and no changed heading; heading-anchor check that every anchor RUNBOOK.md exposed before the edit still exists; the atom's decision sentence quotable verbatim by the three named consumers"
```
