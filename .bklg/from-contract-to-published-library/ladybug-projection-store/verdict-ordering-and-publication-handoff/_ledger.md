---
item: HS-S0083
stage: implement
created: 2026-08-12T13:47:21.508Z
updated: 2026-08-12T13:47:21.508Z
---

# Acceptance ledger — Merge the verdict ahead of publication, and hand over what it owes

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Every `verifying_test` below names a **command plus an artefact path**, not a `#[test]` id, and that
is deliberate: this story ships no code, and the project's testing brief classifies its AC as
**process** — "not a CI-enforced check (no such gate exists in `.redkiln/config.yaml`) — a
human-checked ordering claim" (`…/ladybug-projection-store/_decomposition.md:481`). Inventing a test
function name to fill the column would be a decorative rule. Evidence must therefore cite the
commit SHAs, the `git` output, and the `file:line` of the merged handover document — not a green
tick alone.

The `mount_point` is the same for every row, because this story has exactly one: the handover block
inside `RUNBOOK.md` phase 11's licensed seam (`RUNBOOK.md:4411-4444`), which is what makes
`references/evaluation/phase-11-publication-handover.md` reachable from the plan of record.

```yaml
- id: AC-001
  criterion: "GIVEN the backbone-E reviewer, who must believe that the freeze verdict was heard before 0.2.0 became irreversible rather than filed after it, WHEN they look for the ordering, THEN they find it asserted in a new dated document references/evaluation/phase-11-publication-handover.md that names the verdict's own commit SHA, carries its own ISO date, and states in terms that publication-and-positioning (HS-P0016) opens after it — citing the initiative DAG edge as the authority. The claim is made by a third file, never by an amendment to the sealed verdict, because a verdict that grew a forward obligation in a second commit is no longer the artefact whose value was that it was sealed"
  satisfied: false
  evidence: ""
  mount_point: "RUNBOOK.md phase 11 handover block (within the licensed seam RUNBOOK.md:4411-4444), linking references/evaluation/phase-11-publication-handover.md"
  verifying_test: "`git log --oneline -- references/evaluation/` (verdict introduced once, handover a later separate file) plus `git log --oneline` showing both ahead of the first commit touching .bklg/from-contract-to-published-library/publication-and-positioning/** outside planning; reviewed against references/evaluation/README.md:9-13"
- id: AC-002
  criterion: "GIVEN a reader of the plan of record — someone who opens RUNBOOK.md to find out how far phase 11 got and what comes next, and who will not go spelunking in .bklg/ — WHEN they reach the end of phase 11's body, THEN they meet a handover block that states the ordering to phase 12 and links references/evaluation/phase-11-publication-handover.md by relative path, so the edge is reachable from the document a reader opens next rather than only from the backlog. The block sits inside the licensed seam and touches nothing else in it: phase 11's exit-criteria boxes are the slice-mate's to tick, and this story ticks none and rewords none"
  satisfied: false
  evidence: ""
  mount_point: "RUNBOOK.md phase 11 handover block (within the licensed seam RUNBOOK.md:4411-4444), linking references/evaluation/phase-11-publication-handover.md"
  verifying_test: "`git diff --name-only <merge-base>..HEAD` (exactly RUNBOOK.md, references/evaluation/phase-11-publication-handover.md, this story's backlog folder) and `git diff -- RUNBOOK.md` read with line numbers: every changed line inside RUNBOOK.md:4411-4444 and no `- [ ]`/`- [x]` box changed state; relative link resolves and `cargo xtask ci`'s docs step stays green"
- id: AC-003
  criterion: "GIVEN the HS-P0016 implementer deciding whether publish-0-2-0's whole-gate run on the literal publish commit can afford happenstance-ladybug, WHEN they read handover item 1, THEN they get the measured cold and warm lbug build figures per matrix runner with toolchain and machine stamped, plus the CI shape that number bought and the files that carry it (xtask/src/main.rs, .github/workflows/ci.yml, RUNBOOK.md's phase 11 body) — each transcribed from cold-build-cost-and-ci-shape's record with that record's path and commit, so the number in the handover and the number in the record cannot diverge"
  satisfied: false
  evidence: ""
  mount_point: "RUNBOOK.md phase 11 handover block (within the licensed seam RUNBOOK.md:4411-4444), linking references/evaluation/phase-11-publication-handover.md"
  verifying_test: "Review of references/evaluation/phase-11-publication-handover.md handover item 1 against RUNBOOK.md:4420-4425, :4437 and the cold-build-cost-and-ci-shape (AC-009) record — figures must agree digit for digit and each carry a path:line or commit citation; `git diff --name-only` shows no change to xtask/src/main.rs or .github/workflows/ci.yml"
- id: AC-004
  criterion: "GIVEN Persona 4 landing on a registry page during their one bounded look, WHEN HS-P0016 decides what 0.2.0 ships, THEN they have already been told — in handover item 2 — that docs.rs cannot build lbug 0.19.1, and that with publish = false removed this is no longer a note but a consequence: a published happenstance-ladybug would carry a red docs page against phase 12's own proof artefact, on a release that cannot be edited afterwards. The item also flags that xtask/src/package.rs's PUBLISHABLE now carries a crate outside HS-P0016's three-crate working default, so crate-set-decision meets that fact by name rather than by a reconciliation failure inside its own PR"
  satisfied: false
  evidence: ""
  mount_point: "RUNBOOK.md phase 11 handover block (within the licensed seam RUNBOOK.md:4411-4444), linking references/evaluation/phase-11-publication-handover.md"
  verifying_test: "Review of handover item 2 against crates/happenstance-ladybug/src/lib.rs's driver-absent section at the line range it occupies AT THIS COMMIT (re-grepped after real-lbug-driver-swap, not the pre-swap :24-32), xtask/src/package.rs's PUBLISHABLE set, and crates/happenstance-ladybug/Cargo.toml with publish = false gone; `git diff --name-only` shows none of those three files modified by this PR"
- id: AC-005
  criterion: "GIVEN the HS-P0016 implementer of guarantees-and-docs-rs-presentation, who owns adding a [package.metadata.docs.rs] block, WHEN they read handover item 3, THEN they receive a question with its options and its constraint, not an answer: modelled on and citing crates/happenstance-core/Cargo.toml:54-56, and stating that all-features = true plus rustdoc-args = [\"--cfg\", \"docsrs\"] configures rustdoc and not the cxx / cmake native toolchain, so the block cannot by itself turn a failing native build green. No answer is given and no block is added to crates/happenstance-ladybug/Cargo.toml, because whether this crate ships at 0.2.0 is a decision this project explicitly does not hold"
  satisfied: false
  evidence: ""
  mount_point: "RUNBOOK.md phase 11 handover block (within the licensed seam RUNBOOK.md:4411-4444), linking references/evaluation/phase-11-publication-handover.md"
  verifying_test: "Review that handover item 3 is phrased as an open question and states the rustdoc-not-toolchain constraint, citing crates/happenstance-core/Cargo.toml:54-56 and .bklg/from-contract-to-published-library/ladybug-projection-store/project.md:127-130; `git diff -- crates/happenstance-ladybug/Cargo.toml` is empty in this PR"
- id: AC-006
  criterion: "GIVEN that an obligation handed to a project is an obligation handed to nobody, WHEN the HS-P0016 implementer opens whichever story they are actually working, THEN each handover item names its receiving story by slug — crate-set-decision, guarantees-and-docs-rs-presentation, rendered-page-preflight, publish-0-2-0 — and every slug cited was verified to exist before it was written down, so a typo cannot silently route an obligation into a void"
  satisfied: false
  evidence: ""
  mount_point: "RUNBOOK.md phase 11 handover block (within the licensed seam RUNBOOK.md:4411-4444), linking references/evaluation/phase-11-publication-handover.md"
  verifying_test: "`ls .bklg/from-contract-to-published-library/publication-and-positioning/` resolves all four slugs as real directories, and each matches a row at that project's _storymap.md:56, :66, :67, :68; the reviewer re-runs the listing rather than trusting the prose"
- id: AC-007
  criterion: "GIVEN a reader who trusts RUNBOOK.md's own graph — which today gives phase 12 the dependencies \"7, 8\" and draws phase 11 as one of three that never rejoin — WHEN they read the handover document, THEN the discrepancy is recorded as a finding for HS-P0016 and for a re-plan, with the exact locations, and is not repaired here: phase 12's dependency row, the phase graph and phase 12's body are outside this project's licensed seam, and no .bklg/** frontmatter (including blocks / blocked_by) is touched, because the CLI is the only writer of it"
  satisfied: false
  evidence: ""
  mount_point: "RUNBOOK.md phase 11 handover block (within the licensed seam RUNBOOK.md:4411-4444), linking references/evaluation/phase-11-publication-handover.md"
  verifying_test: "The merged references/evaluation/phase-11-publication-handover.md contains a finding naming RUNBOOK.md:163, :176-190 and :243-245; `git diff -- RUNBOOK.md` shows no change outside :4411-4444; `git diff --name-only` contains no .bklg/** path other than this story's own folder and no frontmatter hunk; `redkiln doctor` clean with exactly six template-drift advisories"
- id: AC-008
  criterion: "GIVEN the backbone-E reviewer asking the one question that would void the whole artefact — did this document invent anything? — WHEN they audit the diff, THEN every number, finding, verdict phrase and PS disposition in it is quoted or cited from the sibling record that owns it with path and commit, and none originates here; AND the tree is still green on the whole gate, with spec/SPECIFICATION.md untouched so no [FROZEN] clause text, marker or citation could have moved"
  satisfied: false
  evidence: ""
  mount_point: "RUNBOOK.md phase 11 handover block (within the licensed seam RUNBOOK.md:4411-4444), linking references/evaluation/phase-11-publication-handover.md"
  verifying_test: "`cargo xtask ci` green on the merge commit (the whole gate, not --fast, including `cargo xtask spec-trace`); `git diff -- spec/SPECIFICATION.md` empty; `git diff --name-only` contains no path under .kb/, crates/, xtask/ or any Cargo.toml; `redkiln validate --kb` clean; review confirms every assertion in the handover document carries an outbound citation"
```
