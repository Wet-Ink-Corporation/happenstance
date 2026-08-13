---
item: "HS-S0075"
stage: implement
created: "2026-08-12"
updated: "2026-08-12"
---

# Acceptance ledger — ADR-0025: checkpoint placement, mutation vocabulary, blocking bridge

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

**Three things about this story's ledger specifically.** First, `mount_point` carries the placeholder
`0025-<slug>` because the slug should name the answer and the answer is Q1+Q2+Q3 taken together (spec
*Clarifications* item 5); the implementer replaces `<slug>` with the real one when citing evidence.
Second, this story's mount is completed by a **human-invoked** `/redkiln:kb-ingest` wave that commits
on its own worktree branch, so an intake document under `.kb/_intake/` is **never** evidence for any
row — only a `file:line` under `.kb/decisions/0025-<slug>.md`, plus the `.kb/maps/decision-map.md`
row, is (spec EC-002). Third, this story ships **no Rust**: every `verifying_test` below is a static
gate, an artefact-presence check, or a named review-tier read, because the project testing brief
records that the content check for project AC-002 "is a review-tier check, not automatable"
(`.bklg/from-contract-to-published-library/ladybug-projection-store/_decomposition.md`, Testing brief,
AC-002 row). Manufacturing a conformance rule here would be decorative.

```yaml
- id: AC-001
  criterion: "GIVEN an adapter author about to write `commit`, wondering whether the checkpoint may live in a sidecar file next to the graph, WHEN they open ADR-0025's Q1 section, THEN they find the checkpoint placed **in the graph as a node property** with the reason stated as PS-1 — it is what keeps the checkpoint write inside the same `BEGIN TRANSACTION` as the read-model write — and the **sidecar named as the loser, losing on that invariant rather than on preference**, with the concrete `MERGE (c:ProjectionCheckpoint {id: $id}) SET c.position = $position` statement quoted from the instrument that already wrote it."
  satisfied: false
  evidence: ""
  mount_point: ".kb/maps/decision-map.md — the knowledge base's decision index, reached through .kb/_intake/ and a /redkiln:kb-ingest wave that writes .kb/decisions/0025-<slug>.md"
  verifying_test: "static: `redkiln validate --kb` over .kb/decisions/0025-<slug>.md; review-tier: human read of the record's Q1 section against crates/happenstance-ladybug/src/lib.rs:53-58, crates/happenstance-ladybug/src/live_handle.rs:203-217 and crates/happenstance-core/src/projection.rs:13-30 (PS-1)"

- id: AC-002
  criterion: "GIVEN the same author filling in `checkpoint` and tempted to collapse a corrupt stored `INT64` into `Ok(None)` because it is one line shorter, WHEN they read Q1's residue, THEN both narrowing directions survive as **required** behaviour — `MalformedCheckpoint` and `PositionOutOfRange` — with the stated consequence that collapsing the corrupt case \"would silently replay a projection from the beginning\"; AND the checkpoint's schema shape (one node per `ProjectionId`, or one node with a property per id) is decided **for this adapter** and **reported to PS-23 as a data point**, with PS-23's clause text and marker untouched."
  satisfied: false
  evidence: ""
  mount_point: ".kb/maps/decision-map.md — the knowledge base's decision index, reached through .kb/_intake/ and a /redkiln:kb-ingest wave that writes .kb/decisions/0025-<slug>.md"
  verifying_test: "static: `redkiln validate --kb`; `cargo xtask spec-trace` green with `git diff -- spec/SPECIFICATION.md` empty; review-tier: human read against crates/happenstance-ladybug/src/projection_store.rs:175-202 and spec/SPECIFICATION.md:5317 (PS-23) / :4814"

- id: AC-003
  criterion: "GIVEN an adapter author who must express a graph mutation and cannot tell whether the port wants a builder or a string, WHEN they read Q2, THEN the vocabulary is **raw parameterised Cypher** carried as `GraphStatement` with **parameters beside the text, never interpolated into it**, and the reason is stated mechanically — replay goes through `prepare`/`execute` and a stringified statement cannot be prepared once and executed many times; AND the record states whether that vocabulary is still callable from ADR-0007's indicative `Projection::apply(&mut …::Batch<'_>)` signature, so that an incompatibility becomes a named bill for `typed-layer-and-alpha-release` rather than a surprise."
  satisfied: false
  evidence: ""
  mount_point: ".kb/maps/decision-map.md — the knowledge base's decision index, reached through .kb/_intake/ and a /redkiln:kb-ingest wave that writes .kb/decisions/0025-<slug>.md"
  verifying_test: "static: `redkiln validate --kb`; review-tier: human read of the record's Q2 section against crates/happenstance-ladybug/src/projection_store.rs:70-103 and .kb/decisions/0007-projection-runner-decodes.md"

- id: AC-004
  criterion: "GIVEN a future reader who must re-derive why the port has no typed write vocabulary without re-running the exploration, WHEN they read Q2's alternatives, THEN **more than one loser is named and priced** — the typed builder with what it buys *and* what it costs a projection needing Cypher the builder does not model, and the three apply-seam shapes (a bound on `Batch`, a method on `ProjectionStore`, a second associated type) taken from sub-question 1 — with ADR-0008's finding cited as **ruling shapes out before they are weighed**; AND `.kb/open-questions/projection-store-batch-has-no-apply-seam.md` is **consumed as evidence and left unresolved**, its `status` untouched and its resolution still `projection-store-freeze`'s (DR-7)."
  satisfied: false
  evidence: ""
  mount_point: ".kb/maps/decision-map.md — the knowledge base's decision index, reached through .kb/_intake/ and a /redkiln:kb-ingest wave that writes .kb/decisions/0025-<slug>.md"
  verifying_test: "static: `redkiln validate --kb`, including the accepted-decision immutability check against HEAD (catches an edit to .kb/decisions/0008-one-derivation-for-both-ports.md or to the open question's frontmatter); review-tier: human read against .kb/open-questions/projection-store-batch-has-no-apply-seam.md (Ordered sub-questions, 1)"

- id: AC-005
  criterion: "GIVEN an application author who must know, before choosing this adapter, what a synchronous graph engine costs them inside an `async` runtime, WHEN they read Q3, THEN **one** bridge is chosen from the three real candidates — a runtime-gated `spawn_blocking` feature, blocking the executor thread and documenting it, or a blocking-only adapter — **each of the other two carrying what it would have cost**; AND the record states that whatever bridges the two is a public, documented property of the adapter, not a private helper, because \"if the port cannot express it, that is the freeze not holding\"."
  satisfied: false
  evidence: ""
  mount_point: ".kb/maps/decision-map.md — the knowledge base's decision index, reached through .kb/_intake/ and a /redkiln:kb-ingest wave that writes .kb/decisions/0025-<slug>.md"
  verifying_test: "static: `redkiln validate --kb`; review-tier: human read of the record's Q3 section against crates/happenstance-ladybug/src/projection_store.rs:39-47 and the ladybug-projection-store _intake-brief.md Constraints section"

- id: AC-006
  criterion: "GIVEN a reviewer checking that Q3's field was framed honestly rather than narrowed to reach a conclusion, WHEN they read the record, THEN `#[async_trait]` is excluded **outright as a constraint** (it injects `+ Send` and makes the wasm32 target impossible, ADR-0001), \"the suite needs tokio\" is **retired as an argument** because `__emit_blocking` is a `#[test]` + `block_on` emitter needing nothing from the adapter, the existing tokio **dev**-dependency is stated **not** to be a precedent for a normal one, and the `Send` flavour is recorded as **already settled on evidence** (`lbug`'s `Database` and `Connection` are `Send + Sync`) with BR-12's `!Send` proof left to `cloudflare-durable-object-store`."
  satisfied: false
  evidence: ""
  mount_point: ".kb/maps/decision-map.md — the knowledge base's decision index, reached through .kb/_intake/ and a /redkiln:kb-ingest wave that writes .kb/decisions/0025-<slug>.md"
  verifying_test: "static: `redkiln validate --kb`; `cargo xtask affected --base main` green with no crates/** path in the diff (nothing acquired tokio as a normal dependency); review-tier: human read against .kb/decisions/0001-async-port-flavours.md, crates/happenstance-testkit/src/lib.rs:60-67, crates/happenstance-ladybug/Cargo.toml and crates/happenstance-ladybug/src/projection_store.rs:252-257"

- id: AC-007
  criterion: "GIVEN a reader in a year who cannot tell whether this record still applies to the port they are holding, WHEN they open it, THEN it **names the `ProjectionStore` shape it was written against** — whether `type Batch;` (no lifetime), the write seam and `projection_store_conformance!` had actually merged — **and the commit it read them at**, so the reasoning can be re-checked rather than assumed; AND if the upstream preflight came back red, this story did not start and the finding was surfaced to `projection-store-freeze` (HS-P0010) instead of being worked around locally."
  satisfied: false
  evidence: ""
  mount_point: ".kb/maps/decision-map.md — the knowledge base's decision index, reached through .kb/_intake/ and a /redkiln:kb-ingest wave that writes .kb/decisions/0025-<slug>.md"
  verifying_test: "process: predecessor HS-S0074 (preflight-and-unlike-axes) merged green per story.md `blocked_by`; review-tier: the SHA named in references/adr/0025-<slug>.md resolves with `git show`, and crates/happenstance-core/src/projection.rs at that SHA carries the shape the record names"

- id: AC-008
  criterion: "GIVEN an adapter author arriving cold who does not know the filename, WHEN the merge lands, THEN the decision has **mounted at the knowledge base's composition root** — an accepted atom at `.kb/decisions/0025-<slug>.md` with valid `KbFrontmatter` (`kind: decision`, `status: accepted`, `adr_id`, `phase`, `source_paths` citing both inputs), **authored by `/redkiln:kb-ingest` and never by hand**, linking a **retained** long-form record at `references/adr/0025-<slug>.md` in the house shape, carrying its row in `.kb/maps/decision-map.md` in the same column shape every other row uses, with `.kb/_intake/` emptied by the wave, the intake `README.md` dropped at the wave's approval gate, and a wave id that does not overwrite `2026-08-10-intake` or `2026-08-10-intake-2`."
  satisfied: false
  evidence: ""
  mount_point: ".kb/maps/decision-map.md — the knowledge base's decision index, reached through .kb/_intake/ and a /redkiln:kb-ingest wave that writes .kb/decisions/0025-<slug>.md"
  verifying_test: "static: `redkiln validate --kb` (KbFrontmatter conformance and accepted-decision immutability against HEAD) and `redkiln doctor` clean at exactly six template-drift advisories; artefact presence: `test -f references/adr/0025-<slug>.md`, the .kb/maps/decision-map.md row resolving, .kb/_intake/ empty after the wave, a new non-colliding directory under .kb/_governance/integration-waves/"

- id: AC-009
  criterion: "GIVEN the reviewer who must confirm this record changed the plan of record without breaking the specification's citations, WHEN they read the diff, THEN `RUNBOOK.md:304` and `RUNBOOK.md:497` say *written* in the idiom the settled rows already use and `RUNBOOK.md:4415-4419`'s ADR-0025 box is ticked `- [x]`, **all three edited in place with a net line-count change of zero**; AND `spec/SPECIFICATION.md` is byte-identical, no `[FROZEN]` clause marker or text moved, and no phase-11 checkbox belonging to a sibling story was ticked."
  satisfied: false
  evidence: ""
  mount_point: "RUNBOOK.md — the plan of record (secondary mount): the ADR queue row at :304, the Projections open-questions row at :497, and phase 11's ADR-0025 checkbox at :4415-4419"
  verifying_test: "static: `cargo xtask spec-trace` green AND `git diff --numstat -- RUNBOOK.md` showing equal insertions and deletions (spec-trace alone is insufficient — ANCHOR_SLACK is 12 lines, xtask/src/spec_trace.rs:391) AND `git diff -- spec/SPECIFICATION.md` empty; review-tier: the rewritten rows read against the settled idiom at RUNBOOK.md:288 and :485, and against _storymap.md Coverage for sibling-owned boxes"
```
