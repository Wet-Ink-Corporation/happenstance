---
item: HS-S0074
stage: implement
created: "2026-08-12"
updated: "2026-08-12"
---

# Acceptance ledger — Preflight the merged port, then commit the "structurally unlike" axes

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

This story ships no Rust, so every `verifying_test` below is a **command over a real path** rather
than a `#[test]` — which is what the testing brief already requires of project AC-001: *"checked by
commit order, not by content review"* (`_decomposition.md:471`). A green `cargo xtask ci` is
evidence for none of these rows (spec, NF-002).

```yaml
- id: AC-001
  criterion: "GIVEN an adapter author about to write the first Ladybug batch against a port they did not freeze, WHEN they open references/evaluation/phase-11-preflight-and-unlike-axes.md, THEN its `## Preflight finding` table gives them three named observations against the *merged* happenstance-core tree — (1) `ProjectionStore::Batch` is declared `type Batch;`, no lifetime parameter and no `where Self: 'a`; (2) the write seam exists; (3) `projection_store_conformance!` resolves from happenstance-testkit — each with the exact command that produced it, a green/red result, and the commit SHA it was run at, so the author never has to re-derive whether the port they are coding against is the port that shipped."
  satisfied: false
  evidence: "" # file:line and/or verifying test id — required once satisfied
  mount_point: "references/evaluation/README.md"
  verifying_test: "rg -n '^## Preflight finding' references/evaluation/phase-11-preflight-and-unlike-axes.md (three observation rows, each carrying a commit SHA), with the three observation commands re-run and their output pasted in .bklg/from-contract-to-published-library/ladybug-projection-store/preflight-and-unlike-axes/implementation-report.md"

- id: AC-002
  criterion: "GIVEN the same author, facing two mutually incompatible write seams — PS-11's `ProjectionProbe` in the contract crate behind `feature = \"conformance\"` versus HS-P0010's fixture-level `write_probe(&mut Batch)` plus an out-of-band `read_probe(&Store)` — WHEN they read observation 2, THEN it names which shape actually shipped, by file:line in the merged tree, and states whether that shape can express a replayable parameterised Cypher statement; AND if neither shipped, or the shipped one cannot, the document records that as *the freeze not holding* for the verdict to carry rather than as an adapter problem to route around."
  satisfied: false
  evidence: "" # file:line and/or verifying test id — required once satisfied
  mount_point: "references/evaluation/README.md"
  verifying_test: "Review-tier: observation 2 of references/evaluation/phase-11-preflight-and-unlike-axes.md read against spec/SPECIFICATION.md:4977-5005 (PS-11) and .bklg/from-contract-to-published-library/projection-store-freeze/project.md (In scope item 4); the cited file:line resolved by hand at the recorded SHA and recorded in the story's _review.md"

- id: AC-003
  criterion: "GIVEN a maintainer who needs to know whether phase 11 may proceed, WHEN any of the three observations comes back red, THEN the document still merges — carrying the red result, an escalation naming projection-store-freeze (HS-P0010) and the specific observation that failed — with the `## Axes of unlikeness` section present but explicitly left unwritten and why, and with nothing anywhere in the PR stubbing, vendoring, feature-gating or locally patching the missing port; the story reports blocked rather than green."
  satisfied: false
  evidence: "" # file:line and/or verifying test id — required once satisfied
  mount_point: "references/evaluation/README.md"
  verifying_test: "redkiln verify --grain story (PR-boundary block in spec.md contains no crates/** path) plus rg -n '^## Axes of unlikeness' references/evaluation/phase-11-preflight-and-unlike-axes.md; escalation to HS-P0010 recorded in preflight-and-unlike-axes/implementation-report.md"

- id: AC-004
  criterion: "GIVEN an adapter author whose very next command is `cargo build -p happenstance-ladybug`, WHEN they read the finding, THEN they find T1 raised as its own named item — that `LiveHandleProjectionStore` binds `type Batch<'a> = GraphWriteHandle<'a>` with no lifetime-free spelling for a `'static` store, that HS-P0010's \"compiles with no change other than the lifetime parameter's removal\" claim is therefore false for one of this crate's two impls, that its retirement belongs to real-lbug-driver-swap and not to this story, and that the instrument's transcripts survive its deletion at references/adapter-shapes.md:169-195 — so they hit the compile error already knowing it is expected, whose it is, and what it costs."
  satisfied: false
  evidence: "" # file:line and/or verifying test id — required once satisfied
  mount_point: "references/evaluation/README.md"
  verifying_test: "rg -n 'LiveHandleProjectionStore' references/evaluation/phase-11-preflight-and-unlike-axes.md returns the finding, and review confirms it cites crates/happenstance-ladybug/src/live_handle.rs:177-180 and references/adapter-shapes.md:169-195; the PR touches no crates/** file"

- id: AC-005
  criterion: "GIVEN an evaluator with one sitting and no access to this backlog, WHEN they read `## Axes of unlikeness`, THEN every row is a *contrast* and not a description: three populated columns — Ladybug's property, the property every shape that froze the port has instead, and what a conformance rule would have to do to tell them apart — covering at minimum the four axes AC-001 of the project enumerates (no transaction handle type in the driver at all; a deferred, owned, `Send + 'static` write set where nothing executes until `commit`; a synchronous driver behind an `async` port; single-writer concurrency where two racing commits are routine), so the evaluator can say what the eventual verdict could have come out as, not merely what it did."
  satisfied: false
  evidence: "" # file:line and/or verifying test id — required once satisfied
  mount_point: "references/evaluation/README.md"
  verifying_test: "Structural: the `## Axes of unlikeness` table in references/evaluation/phase-11-preflight-and-unlike-axes.md has >= 4 body rows and no empty third column, cross-checked against crates/happenstance-ladybug/src/projection_store.rs:1-64, :39-47, :204-213 and spec/SPECIFICATION.md:4760-4774 (PS-2); adversarial read against discover.md:99-107 recorded in the story's _review.md"

- id: AC-006
  criterion: "GIVEN the implementer of HS-S0078's negative control, who will later have to recognise a `GraphWriteSet` of `MERGE (r:Row {k: $k}) SET r.v = $v` statements as a key-value row store wearing a Cypher accent, WHEN they consult this document, THEN it has already decided the Cypher-versus-SQL mutation-vocabulary axis explicitly — either carrying it as a row (\"the batch expresses relationships and traversal, not rows\") with a third column naming what a rule would have to do, or carrying a written omission with its reason — so the mutant is caught by a document that existed first rather than by hindsight. Silence on this axis fails the criterion."
  satisfied: false
  evidence: "" # file:line and/or verifying test id — required once satisfied
  mount_point: "references/evaluation/README.md"
  verifying_test: "rg -n -i 'relationship|traversal|vocabulary' references/evaluation/phase-11-preflight-and-unlike-axes.md returns either the axis row or the recorded omission; review against discover.md:109-123 confirms exactly one of the two is present and reasoned"

- id: AC-007
  criterion: "GIVEN a reviewer of the eventual freeze verdict who must decide whether it is evidence or rationalisation, WHEN they run `git log` over the repository, THEN the axes document is the sole content of one commit — together with the references/evaluation/README.md line that classifies it by name under the immutable-evidence lifecycle (dated, commit-pinned, superseded rather than edited) — and that commit precedes every commit touching a `projection_store_conformance!` invocation against a Ladybug fixture, so the definition provably could not have been chosen to match the result."
  satisfied: false
  evidence: "" # file:line and/or verifying test id — required once satisfied
  mount_point: "references/evaluation/README.md"
  verifying_test: "git log --diff-filter=A --format=%H -- references/evaluation/phase-11-preflight-and-unlike-axes.md compared with git log -S 'projection_store_conformance' --format=%H -- crates/happenstance-ladybug/, plus git show --stat <axes-sha> (two files only) and rg -n 'phase-11-preflight-and-unlike-axes' references/evaluation/README.md; both SHAs and their order recorded in preflight-and-unlike-axes/implementation-report.md"
```
