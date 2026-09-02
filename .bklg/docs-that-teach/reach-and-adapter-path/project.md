---
id: HS-P0023
uid: 86a957
type: project
slug: reach-and-adapter-path
title: Reach and the Adapter Path
parent: HS-I0007
initiative: docs-that-teach
project: reach-and-adapter-path
status: implementing
process: project
stage: implementation
automation: HITL
severity: null
blocked_by: []
blocks: []
terminal: false
owner: ryan-britton
created: 2026-08-17
updated: 2026-08-20T03:07:10.455Z
links:
  pr: null
  commits: []
  kb: []
gate_open: false
schema: 1
process_rev: 7289a0c4
---
# Reach and the Adapter Path

## One-line objective

Make the teaching **reachable from where each reader already stands** — one decided
pointer policy, applied at the crate's front door, at the evaluator's second
question, and at the `store.rs` line where the adapter author's compiler error
actually fires.

## How this advances the initiative

The initiative's cross-persona finding is that the answering content already exists
somewhere in this workspace and cannot be reached from where the reader is
(`.bklg/docs-that-teach/initiative.md`, "Underneath all six sits one cross-persona
finding"). Two of the three personas hit that in a place a page cannot go to them:
the evaluator hits it inside a decision, the adapter author inside a compiler error.
This project owns both, plus the front door that lets anyone find the narrative
material at all.

It is the last of the three content projects and the one that closes the loop:
HS-P0020 builds the checked surface, HS-P0021 states what a page owes, HS-P0022
authors the application author's path — and none of that is reachable until
something points at it. The decomposition of record puts reachability and the
persona-2 slice in one project because both are pointer problems governed by one
policy, DT-10 (`.bklg/docs-that-teach/_decomposition.md`, Children table and Design
tension ownership).

Three concrete, verified gaps this project closes:

- `crates/happenstance/README.md` and `crates/happenstance/src/lib.rs` — the
  evaluator's front door — point outward only to crates.io and to
  `spec/SPECIFICATION.md`. Nothing on either surface points at narrative teaching,
  which is BR-08's converged-ecosystem pattern (tokio, serde, diesel each carry one
  explicit crate-root pointer) left unimplemented here.
- `crates/happenstance-core/src/store.rs:31-45` does explain the two-flavour
  ambiguity and does show an `error[E0034]` excerpt — but a **trimmed** one. rustc's
  actual output for this collision carries two `= note:` candidate lines naming
  `TraitVariantBlanketType`, and that string appears nowhere in `store.rs`. The
  reader who pastes their real error into a search box therefore still misses the
  file they already have open, which is BR-15's gap stated precisely.
- The reasoning an adapter author needs exists — `CONTRIBUTING.md:69-95` (the
  four-step recipe), `CONTRIBUTING.md:97-140` (why a provided method is never
  `async fn`), `standards/rust/91-adapter-authoring-recipe.md`,
  `standards/rust/20-two-flavour-ports.md`,
  `crates/happenstance-core/src/memory.rs:16-71` (the runnable full-DCB-loop
  walk-through) — and is spread across four contributor-facing surfaces in no
  reading order. AC-05 is a sequencing job, not an authoring job.

## In scope (this project)

- **DT-10, resolved and recorded.** Whether the reference surface points outward
  once, per-item, or both-with-one-authoritative — decided in this project's
  `_design.md`, with the rule for where a per-item pointer is permitted and how the
  set is kept from rotting. The perceptual review is a skip (`design.capture` is
  deliberately absent from `.redkiln/config.yaml`), so the written resolution is the
  only record.
- **The front door pointer** (BR-08, AC-11, DoD-7). Installing the policy on
  `crates/happenstance/src/lib.rs`'s crate root and `crates/happenstance/README.md`,
  and observing a person reach the narrative material starting only from what a
  developer sees after `cargo add happenstance`.
- **The evaluator's second question** (BR-16, AC-06, DoD-9). Naming the plausible
  second questions in advance — research 03's three stall points are the candidate
  set — and making each one reachable by a link from the page that answered the
  first, without adding a navigation widget the medium already provides.
- **The adapter author's error site** (BR-15, AC-04, DoD-10). Bringing the
  explanation into `crates/happenstance-core/src/store.rs` in the form rustc
  actually emits, including the candidate notes, and pointing from there to the
  reasoning account.
- **The adapter reasoning account** (AC-05). A narrative page that *surfaces and
  sequences* the existing `MemoryEventStore` walk-through and the two-flavour
  reasoning with connective tissue — the scope resolved from the charter at the
  decomposition gate, not a new chapter-length account
  (`.bklg/docs-that-teach/_decomposition.md`, "AC-05's scope").
- **Keeping the doc-comment rewrite honest.** Consuming HS-P0020's pinned clause-id
  set before touching any `happenstance-core` doc comment, and running
  `cargo xtask spec-trace` over the result
  (`.kb/governance/rewrite-the-referent-never-the-reasoning.md`).
- **Obeying, not authoring, the discipline.** Every page this project adds carries
  its single named answered-need under HS-P0021's rule and compiles under HS-P0020's
  gate step.

## Out of scope (this project)

| Excluded | Owned by |
| --- | --- |
| The pinned narrative tree, the gate step that builds and compiles it, the deliberately-broken-page falsification, the hosting shape (docs.rs-only vs. a separate rendered surface), DT-7's hidden-panel decision, and BR-10's clause-id pin itself | HS-P0020 `checked-documentation-surface` |
| The page-need discipline and where it lives (DT-2, DT-3, DT-8), the one-need-per-page review pass, and the "no page is a second specification" spot check | HS-P0021 `page-need-discipline` |
| The opening encounter, the conceptual bridge, which prior mental model is anchored against, diagrams and the wrong-model contrast (DT-1, DT-4, DT-5, DT-6) | HS-P0022 `application-author-path` |
| Recruiting and running the non-author non-insider reader, the friction log's shape, and dispositioning what it finds — including any verdict that the surfaced adapter account does not carry an adapter author | HS-P0024 `comprehension-evidence` |
| Promoting personas into `.kb/product/`, reconciling with HS-S0131, and re-observing all fifteen initiative Definition-of-Done scenarios | HS-P0025 `durable-audience-closeout` |
| Whether the README's *claims* are true — landing copy, status-truth vocabulary, the maturity census, peer positioning, registry metadata | `publication-and-positioning` (HS-P0016), on the unmerged `initiative/from-contract-to-published-library` branch. **The seam is purpose, not paragraph**: this project adds a pointer to `crates/happenstance/README.md` and changes no claim on it. That project's resolved design gate is not reopened. |
| Automated checking of the three observed walks | Nobody, deliberately. DoD-7, 9 and 10 are observed walks, which is why this project takes no `testing` brief (`.bklg/docs-that-teach/_decomposition.md`, Warranted briefs). |

## Derived requirements

Expanded from the initiative requirements this project owns (BR-08, BR-15, BR-16)
and the criteria traced to it (AC-04, AC-05, AC-06, AC-11; DoD-7, 9, 10; DT-10).

- **DR-1 — One pointer policy, decided before any pointer is installed.** DT-10's
  three options are weighed and one is chosen in `_design.md`. A per-item pointer,
  if permitted, is permitted by a stated rule rather than case by case, because the
  evidence names the cost precisely: per-item pointers "become a second navigation
  surface to keep true" (`.bklg/docs-that-teach/initiative.md`, DT-10 row).
- **DR-2 — The front door carries the pointer the ecosystem converged on.** The
  crate root of `happenstance` states, in one line, that guide-level material exists
  and where — the tokio/serde/diesel shape recorded in
  `_discovery/distillation/interaction-patterns.md` ("Two-surface split"). The
  crate's README carries the same pointer, since a crates.io reader never sees the
  rustdoc.
- **DR-3 — Every pointer resolves in the rendered artefact, not only in source.**
  The gate already denies `broken_intra_doc_links` and runs three rustdoc builds
  (`CLAUDE.md`, Commands); a pointer out of rustdoc into the narrative surface is
  outside that check unless HS-P0020's step covers it, so this project states which
  mechanism checks each pointer and installs no pointer that nothing checks.
- **DR-4 — The second questions are named before they are answered.** At least two
  are drawn from the field's independently observed stall points — dynamism mistaken
  for chaos, modelling-versus-routing, and no replacement noun for what an Aggregate
  named (`_discovery/distillation/personas-and-journeys.md`, Persona 3, journey step
  2) — and each is walked to a page that answers it.
- **DR-5 — No bespoke navigation widget.** rustdoc renders a per-item nav bar and
  any narrative surface renders a TOC; the dossier's anti-pattern list is explicit
  that the evaluator's gap "is evidence of a missing *link*, not a missing *widget*"
  (`_discovery/distillation/interaction-patterns.md`, Anti-patterns). Exhausting the
  built-in surface — intra-doc links, `#[doc(alias)]`, rustdoc search — precedes
  proposing anything else, and a rejection is recorded with its reason.
- **DR-6 — The error text in `store.rs` is rustc's, not a paraphrase of it.** The
  excerpt at `crates/happenstance-core/src/store.rs:36-41` is trimmed of the
  candidate `= note:` lines, so the exact string a confused reader searches
  (`TraitVariantBlanketType`) is absent from the file they have open. The rewrite
  restores it. The excerpt stays in a ```` ```text ```` fence, so nothing compiles it
  — that limit is stated on the page rather than left implied.
- **DR-7 — The error site points at the reasoning, and the reasoning is sequenced,
  not rewritten.** The account orders what already exists —
  `crates/happenstance-core/src/memory.rs:16-71`, `CONTRIBUTING.md:69-95` and
  `:97-140`, `standards/rust/91-adapter-authoring-recipe.md`,
  `standards/rust/20-two-flavour-ports.md`,
  `standards/rust/25-what-removes-send-and-sync.md` — and cites
  `.kb/decisions/0001-async-port-flavours.md` and
  `.kb/decisions/0008-one-derivation-for-both-ports.md` for *why* the two-flavour
  split exists rather than restating them.
- **DR-8 — No page becomes a second specification.** A normative claim is a citation
  into `spec/SPECIFICATION.md` that resolves; the precedence chain in
  `standards/rust/README.md:25-29` is obeyed and not extended (BR-09/AC-12 are
  HS-P0021's and HS-P0020's to check, but this project's own pages must satisfy
  them).
- **DR-9 — The clause pin is consulted before the doc comment is touched.** This is
  the project the decomposition names as the one that rewrites `happenstance-core`
  doc comments, sequenced behind HS-P0020's pin for exactly that reason
  (`.bklg/docs-that-teach/_decomposition.md`, "BR-10 was placed inside
  `checked-documentation-surface`").
- **DR-10 — Every observed walk produces a dated record.** A walk that happened and
  was not written down is indistinguishable from one that did not. The record states
  who walked it and that they were not the person who installed the pointer; it does
  **not** claim non-insider status — that is HS-P0024's instrument and this project
  may not stand in for it (BR-14's scoping discipline applied locally).

## Acceptance criteria

Project-grain and testable. This is the spine the story map must cover.

- **AC-001 — DT-10 is resolved in writing.** `_design.md` names the chosen pointer
  policy, the two rejected options and why, and the rule governing where a per-item
  pointer may exist. Verifiable: the design gate is signed off with the tension named
  by id.
- **AC-002 — The crate's front door points outward.** `crates/happenstance/src/lib.rs`
  and `crates/happenstance/README.md` each carry the pointer the policy prescribes,
  and `cargo xtask ci` is green with them in place (the README is compiled as a
  doctest under `#![cfg_attr(doctest, doc = include_str!("../README.md"))]`, so a
  malformed pointer is a build failure, not a review miss).
- **AC-003 — Every installed pointer resolves, and something checks it.** A single
  enumerated list of the outward pointers this project installs exists, and for each
  one the mechanism that would catch it rotting is named. No pointer is installed
  whose only guard is memory.
- **AC-004 — The front-door walk is observed and recorded** (DoD-7). Starting only
  from what a developer sees after `cargo add happenstance`, a person who did not
  install the pointer reaches the narrative material. Dated record of what they
  opened, in order.
- **AC-005 — Two named second questions are walked to an answer** (DoD-9). At least
  two, drawn from the stall points in DR-4, are each followed from the
  first-question page to a page that answers them — no dead ends, and without
  leaving surfaces a Rust developer already reads. Recorded as a path per question.
- **AC-006 — `store.rs` carries rustc's own error output.** The module doc contains
  the `error[E0034]` block including its candidate `= note:` lines, so
  `TraitVariantBlanketType` — currently absent from the file — appears at the site
  where the error fires, and the fence's uncompiled status is stated on the page.
- **AC-007 — The error site points at the reasoning account,** and the account is
  reachable from `store.rs` in one hop under the AC-001 policy.
- **AC-008 — The adapter reasoning account exists as sequencing, not volume.** A
  narrative page orders the existing explanations named in DR-7 into the sequence
  someone building an adapter needs, cites each rather than copying it, and states
  which need it answers under HS-P0021's discipline.
- **AC-009 — The error walk is observed and recorded** (DoD-10). Someone reproduces
  the E0034 collision and reaches the explanation from the file and the message in
  front of them and nothing else. Dated record; the observer is not the author of
  the rewrite.
- **AC-010 — The doc-comment rewrite leaves every frozen documentation MUST
  discharged.** HS-P0020's pinned clause-id set is consulted before the first edit to
  `crates/happenstance-core/src/store.rs`, and `cargo xtask spec-trace` passes over
  the tree afterwards.
- **AC-011 — No navigation widget was added, and the rejection is on record.** The
  built-in surface (intra-doc links, `#[doc(alias)]`, rustdoc search, the narrative
  surface's own TOC) is what carries the reach; any widget considered is recorded
  with the reason it was declined.
- **AC-012 — This project's pages obey the discipline and the specification.** Each
  page carries one named answered-need, every normative claim is a resolving citation
  into `spec/SPECIFICATION.md`, and no page restates a clause.

## Definition of done (boundary-level)

This project is **non-terminal**: it is held to a project-scoped integration bar,
not to the initiative's end-to-end (`.bklg/docs-that-teach/_decomposition.md`,
Available parallelism).

1. `cargo xtask ci --fast` is green on the merged result — the bar
   `.redkiln/config.yaml` wires for a non-terminal project (`CLAUDE.md`, Commands).
2. `cargo xtask spec-trace` passes over the tree as it stands after the
   `store.rs` rewrite, with the pinned clause-id set cited in the story that made
   the edit.
3. `_design.md` resolves DT-10 by id and is signed off; the perceptual review is
   recorded as a skip with its standing reason, not silently absent.
4. The three walks (AC-004, AC-005, AC-009) each have a dated record in this
   project's artefacts, each naming its walker and their relationship to the work.
5. Every page and doc comment this project touched is inside HS-P0020's checked
   surface — no fence opts out except the `text` error excerpt, which says so.
6. The pointer inventory of AC-003 exists in one place and every entry names its
   guard.
7. Nothing in `crates/happenstance/README.md` changed except the addition of the
   pointer — the HS-P0016 seam is demonstrable by diff, not asserted.

## Dependencies

**Depends on** (`.bklg/docs-that-teach/_decomposition.md`, Dependency graph):

- **HS-P0020 `checked-documentation-surface`** — supplies the pinned tree, the gate
  step that compiles narrative fences, the hosting shape this project points *at*,
  and BR-10's clause-id pin that DR-9/AC-010 consume.
- **HS-P0021 `page-need-discipline`** — supplies the rule every page this project
  adds must satisfy (AC-012). A page authored before the rule exists is a page the
  rule is retro-fitted to.
- **HS-P0022 `application-author-path`** — supplies the narrative material AC-011
  reaches and the pages DoD-9's second questions land on. Reversing this edge is the
  only cycle available in the graph.

**Unlocks**: HS-P0024 `comprehension-evidence`, which needs the assembled surface —
a friction log run against a half-assembled surface measures the assembly, not the
teaching.

**Internal ordering worth carrying into the story map.** The adapter half (AC-006,
AC-007, AC-008, AC-009, AC-010 — the `store.rs` error site and the surfaced
reasoning account) does not depend on HS-P0022's content at all and can proceed as
soon as AC-001's pointer policy is settled. Only the reach half (AC-002, AC-004,
AC-005) waits on HS-P0022's pages existing. The decomposition names this explicitly
and leaves the expression of it to `_storymap.md`.

## Risks and coupling notes

| Risk / coupling | Note |
| --- | --- |
| **`crates/happenstance/src/lib.rs` diverges hard on the unmerged sibling branch** | 75 lines here, 237 there, with the `Tags::empty()` defect intact in their version. The decomposition's operational rule — merge forward before implementing — was written for HS-P0022 and applies verbatim to AC-002, because this project edits the same file's crate root. Authoring the pointer against this branch's stale copy means redoing it at merge. |
| **Shared file with HS-P0016 `publication-and-positioning`** | `crates/happenstance/README.md` is edited by both initiatives. The seam is purpose: they own the claims, this project owns one pointer. DoD-7 makes the seam checkable by diff. The claims about that project (a `_design.md` signed off, a fifty-one-frame mock) are **asserted and unverifiable from this worktree** and must be re-verified against the merged tree before any sequencing is bound to them. |
| **A per-item pointer set becomes a second navigation surface** | This is DT-10's stated trade, not a discovered problem. AC-003's enumerated inventory plus a named guard per entry is the mitigation; if the policy resolves to per-item pointers without a guard mechanism, the policy is wrong. |
| **The `store.rs` rewrite silently un-discharges a frozen clause** | Mitigated structurally: this project sits behind HS-P0020's pin in the DAG rather than behind a note. If HS-P0020 lands without an enumerated set, AC-010 cannot be met and this project blocks rather than proceeding on judgement (`.kb/governance/rewrite-the-referent-never-the-reasoning.md`). |
| **The restored E0034 excerpt is text, so nothing checks it against rustc** | A known, accepted limit. Compiler diagnostic wording changes between toolchains, and a ```` ```text ```` fence is invisible to every gate step. The mitigation is honesty on the page plus keeping the excerpt to the stable parts (the error code and the candidate item names), not a pretence that it is checked. Deviation from nothing — `standards/rust/62-doctests-and-harnesses.md` governs compiled fences; a diagnostic transcript is not one. |
| **AC-005's second questions are chosen by the author** | The evaluator has not been observed. Research 03's three stall points are the least author-biased candidate set available, and the honest claim is "two plausible second questions were walked", never "the evaluator's real second question is answered". HS-P0024 is the instrument that can falsify the choice; this project must not overclaim in its place (BR-14's discipline). |
| **AC-005 and AC-004 walkers are non-authors, not non-insiders** | Everyone available to this project is an insider. The published friction-log method is explicit that insider knowledge routes a logger around rough spots. These walks are therefore evidence that the path exists, not evidence that a stranger finds it — the second claim is HS-P0024's alone. |
| **AC-008 may turn out to be insufficient** | The decomposition resolved AC-05 to surfacing and sequencing on the charter's "volume is not the measure" non-goal. If the friction log shows the surfaced version does not carry an adapter author, a new account is owed and is dispositioned through HS-P0024 — not absorbed here. |
| **`MemoryEventStore` is the walk-through's subject and is not an adapter** | It is the reference implementation and the conformance suite's oracle (`crates/happenstance-core/src/memory.rs:16-31`). Sequencing it as "the adapter walk-through" risks teaching that a `RwLock<Vec<_>>` is the shape an adapter takes — the exact monoculture `CLAUDE.md` warns about under "A port is only as well-designed as the *spread* of what implements it". The account must say what `MemoryEventStore` is not. |

## Context anchors

Initiative and planning:

- [`../initiative.md`](../initiative.md) — BR-08, BR-15, BR-16; AC-04, AC-05, AC-06,
  AC-11; DoD-7, 9, 10; DT-10
- [`../_decomposition.md`](../_decomposition.md) — the DAG, the AC-05 scope
  resolution, the warranted-brief tags, the merge-forward rule
- `.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md` —
  Persona 2 (the adapter author) and Persona 3 (the evaluator), with the measured
  gaps this project closes
- `.bklg/docs-that-teach/_discovery/distillation/interaction-patterns.md` — the
  two-surface split, the built-in rustdoc surface, and the anti-pattern that rules
  out a bespoke navigation widget

Knowledge base:

- `.kb/decisions/0001-async-port-flavours.md` — why there are two port flavours; the
  decision the E0034 collision is a consequence of
- `.kb/decisions/0008-one-derivation-for-both-ports.md` — one derivation scheme, and
  what a provided body owes
- `.kb/governance/rewrite-the-referent-never-the-reasoning.md` — the test applied to
  every `store.rs` doc-comment edit
- `.kb/maps/decision-map.md` — the supersession graph, so the account cites a
  standing decision rather than a superseded one

Standards and specification:

- `standards/rust/20-two-flavour-ports.md`,
  `standards/rust/25-what-removes-send-and-sync.md`,
  `standards/rust/91-adapter-authoring-recipe.md` — the reference the account
  sequences and does not restate
- `standards/rust/70-rustdoc-obligations.md` — the only constitution atom on
  documentation, and the source of the green-but-useless counter-example
- `standards/rust/README.md:25-29` — the precedence chain this work sits inside and
  does not extend
- `spec/SPECIFICATION.md` — the normative voice every page cites

Code and existing surfaces:

- `crates/happenstance-core/src/store.rs:31-45` — the trimmed E0034 excerpt, and the
  file the reader has open when it fires
- `crates/happenstance-core/src/memory.rs:16-71` — the runnable full-DCB-loop
  walk-through the reasoning account surfaces
- `crates/happenstance/src/lib.rs:11-71` — the crate root that points at crates.io
  and the specification and at no narrative material
- `crates/happenstance/README.md` — the evaluator's front door, and the file shared
  with HS-P0016
- `CONTRIBUTING.md:69-95`, `CONTRIBUTING.md:97-140` — the four-step recipe and the
  provided-method reasoning, both contributor-facing today
- `docs/README.md:25-29` — why a tree the gate reads is pinned by path

## Companions

- [`_intake-brief.md`](_intake-brief.md) — this project's intake artifact
- [`_storymap.md`](_storymap.md) — the vertical-slice story map, authored by the
  briefs workflow
- [`../_decomposition.md`](../_decomposition.md) — the initiative decomposition of
  record, including this project's dependency edges and its warranted briefs
  (`architecture`, `ux`; no `testing`, no `deployment`)
- [`../initiative.md`](../initiative.md) — the initiative charter
- [`../_plan.md`](../_plan.md) — the initiative planning rollup
- [the observed error-site walk record](error-site-walk-record/walk-record.md) — the dated,
  keyboard-only walk this project's AC-009 asks for, and Definition-of-done item 4's evidence for
  it: the `error[E0034]` collision reproduced first-hand at the pinned toolchain, `store.rs`
  observed to unblock the reader in place before any hop, and the adapter reasoning account reached
  in one hop, with the walker named and the claim bounded to non-authors rather than non-insiders
