---
id: HS-P0020
uid: be2c63
type: project
slug: checked-documentation-surface
title: The Checked Documentation Surface
parent: HS-I0007
initiative: docs-that-teach
project: checked-documentation-surface
status: in-review
process: project
stage: design
automation: HITL
severity: null
blocked_by: []
blocks: []
terminal: false
owner: ryan-britton
created: 2026-08-17
updated: 2026-08-17T05:56:44.748Z
links:
  pr: null
  commits: []
  kb: []
gate_open: false
schema: 1
process_rev: 7289a0c4
---
# The Checked Documentation Surface

## One-line objective

Build the substrate that makes narrative teaching prose *checkable*: a tree pinned
by path in `xtask/src/`, every Rust fence in it compiled against the real workspace
crates by a mandatory step of `cargo xtask ci`, and the check **observed failing** on
a page deliberately broken to prove it.

## How this advances the initiative

The initiative's central finding is that twenty-four gate steps exist and **none of
them reads a sentence** — the constitution supplies its own green counter-example on
purpose (`standards/rust/70-rustdoc-obligations.md`, RS-70-5, cited at
`.bklg/docs-that-teach/initiative.md` "Vision and narrative"). This project closes the
mechanical half of that gap and only the mechanical half.

It is first in merge order and depends on nothing
(`.bklg/docs-that-teach/_decomposition.md` "Dependency graph"), which is what makes it
the right owner of two things that must structurally precede everything else:

- **The machine.** Every page HS-P0021, HS-P0022 and HS-P0023 write lands in a tree
  this project pins and a check this project wires. A page authored before the check
  exists is a page the check is retro-fitted to, and a retro-fitted check is how
  `ignore`-fenced prose survives.
- **BR-10's clause-id pin.** Pinning the frozen documentation MUSTs by clause id here
  puts every project that later rewrites a `happenstance-core` doc comment
  transitively behind the pin, in the graph rather than in a note. HS-P0023 is the
  project that rewrites `crates/happenstance-core/src/store.rs`, and it sits behind
  this one.

The scope boundary that keeps this project honest is stated in the research the
initiative rests on: *"compiles" and "teaches correctly" are different properties that
only partly overlap* (`.bklg/docs-that-teach/_discovery/research/02-compiled-prose-tooling-mdbook-test-doc-comment-skeptic-doc-i.md`).
This project delivers the first of the initiative's two non-substitutable proof
artefacts. It must never be presented as the second — HS-P0024 owns that, and a green
gate here is a *precondition* for looking at the friction log, never a replacement for
it.

## In scope (this project)

- **Pinning the narrative tree by path** in `xtask/src/`, on the precedent already set
  twice: `cargo xtask spec-trace` parses `spec/` by path and `cargo xtask
  lint-constitution` reads `standards/rust/` by path, so moving either "means editing
  `xtask/src/` in the same change — which is the point of pinning them by path rather
  than by convention" (`docs/README.md:25-29`; the constants themselves are
  `ATOM_DIR`, `ROUTER` and `HARNESS` in `xtask/src/lint_constitution.rs:54-61`).
- **Choosing and building the compiled-prose mechanism.** The candidate set is closed
  and surveyed — `mdbook test`, `#[doc = include_str!]` under `#[cfg(doctest)]`,
  `doc-comment`, `skeptic`, `trybuild` — with `skeptic` dormant since 2022 and
  `trybuild` scoped to diagnostic text rather than pages
  (`_discovery/research/02-…`). The workspace already runs the second mechanism at
  scale: `xtask/src/constitution.rs` compiles all twenty-seven constitution atoms as
  doctests of a `publish = false` crate, one module per atom precisely so a failure
  names a file a reader can open (`xtask/src/constitution.rs:11-18`).
- **Wiring the build and render into `cargo xtask ci` as an ordinary REQUIRED step**,
  respecting the existing `Step`/`probe` contract: `None` means mandatory, and "the
  tool is missing" is a skip while "the tool ran and found a problem" is a failure
  (`xtask/src/main.rs:92-102`).
- **The falsification.** Breaking a page on purpose, running the gate, observing it
  fail *by name and location*, reverting, observing green. Both halves recorded; the
  failing half is the one that matters (initiative DoD-2).
- **Closing the opt-out.** An `ignore`-tagged fence opts a block out of the type
  checker rather than through it, and this repository has already found "elaborate
  spellings of `ignore`" in its own doctests (`_discovery/research/02-…`). A lint in
  `xtask/src/` in the shape of `xtask/src/lints.rs` is the existing answer to a check
  no type can express.
- **The registration hole.** `cfg(doctest)` means a renamed or deleted atom is
  invisible to every step except `cargo test`, which is exactly why
  `lint-constitution` reads the directory instead of trusting the compiler
  (`xtask/src/constitution.rs:20-25`). A narrative page that exists but is wired to
  nothing must fail, not pass silently.
- **The mechanical citation check.** A page's normative claims cite
  `spec/SPECIFICATION.md` clause ids; ids "are stable and are never renumbered"
  (`spec/SPECIFICATION.md:280`), so resolution is mechanically checkable in the shape
  `xtask/src/spec_trace.rs` already checks the specification's own cross-references.
  This project owns *the citation resolving*; HS-P0021 owns *the page deferring rather
  than restating*.
- **DT-7 — hidden panels.** Whether adapter- or feature-scoped content may use tabs or
  folds, settled by the same falsification standard: break the code inside a
  non-default panel and see whether the gate catches it. Nothing in `mdbook-tabs`'
  documentation states whether an inactive panel is inside `mdbook test`
  (`_discovery/distillation/interaction-patterns.md`, tension 1).
- **BR-10 — the clause-id pin.** Enumerating the frozen documentation MUSTs in exactly
  one place, by id, before any `happenstance-core` doc comment is touched, and
  re-running `cargo xtask spec-trace` over the result.
- **Hosting and render shape** — docs.rs-only versus a separate rendered narrative
  surface — because it is inseparable from which tree is pinned and what the gate
  builds (`_decomposition.md` "Hosting shape"). This is the only project in the
  initiative that earns a `deployment` brief.
- **Stating the limits in the step's own documentation**, as every other file-reading
  check in `xtask/src/` does under a "What this does not verify" heading
  (`xtask/src/lint_constitution.rs:9-28`).

## Out of scope (this project)

| Excluded | Owner |
| --- | --- |
| The one-need-per-page rule, where the discipline lands as prose, and DT-2/DT-3/DT-8 | HS-P0021 `page-need-discipline` |
| Whether a page *defers* to a clause rather than restating it (the reviewer half of BR-09/AC-12) | HS-P0021 |
| Any teaching content: the opening encounter, the conceptual bridge, DT-1/DT-4/DT-5/DT-6, the diagram question | HS-P0022 `application-author-path` |
| The pointer policy (DT-10), the crate's front door, the evaluator's second question, and the `crates/happenstance-core/src/store.rs` doc-comment rewrite | HS-P0023 `reach-and-adapter-path` |
| The friction log, its shape, its recruitment and its dispositions | HS-P0024 `comprehension-evidence` |
| Persona/journey promotion into `.kb/product/`, and re-observing all fifteen DoD scenarios on the assembled tree | HS-P0025 `durable-audience-closeout` |
| Whether the published surface's *claims* are true — README landing copy, status vocabulary, registry metadata | `publication-and-positioning` (HS-P0016), on the unmerged sibling branch; its design gate is not reopened |
| Amending, discharging or restating any `SPECIFICATION.md` clause; extending the precedence chain in `standards/rust/README.md:23-29` | Nobody — the initiative is additive and says so |
| Incidental bugs found in passing | the `support` initiative, per `.redkiln/config.yaml` |

Two seams are worth restating because they are the ones most likely to be crossed by
accident. **This project owns the machine; HS-P0021 owns the rule.** And **this project
proves code inside prose is not stale; it proves nothing about whether the prose
teaches** — overclaiming that is the initiative's top-ranked risk.

## Derived requirements

Expanded from the initiative requirements the decomposition assigns to HS-P0020:
BR-01, BR-02, BR-10, and the machine halves of BR-09, BR-11 and BR-12.

- **DR-01 (BR-12).** The narrative tree's path is a constant in `xtask/src/`, not a
  convention. Moving or renaming the tree without editing `xtask/src/` fails the gate.
- **DR-02 (BR-01).** Every Rust fence in that tree is compiled against the real
  workspace crates — not against a standalone copy. mdBook's Playground button is
  explicitly not this: it "demonstrates a different, unconnected copy of the code, not
  the one the gate checked" (`_discovery/distillation/interaction-patterns.md`).
- **DR-03 (BR-01).** The compiling step is a member of `REQUIRED` in
  `xtask/src/main.rs`, with `probe: None`, so a missing tool is a hard error rather
  than a `skipped` line. `RUNBOOK.md:918-925` is the precedent: a step behind a probe
  that nothing installed printed `skipped` on all three runners while two documents
  vouched for it.
- **DR-04 (BR-01).** No fence in the tree may opt out of the check unnoticed. An
  `ignore`-class fence is either rejected by a lint naming file and line, or permitted
  only through an enumerated, reviewed allowance list that the same lint enforces.
- **DR-05 (BR-01).** A page present in the tree but not reachable by the checking
  mechanism fails the gate. The `cfg(doctest)` invisibility hole is a known property of
  the pattern this workspace already uses, and `lint-constitution` already answers it
  by reading the directory.
- **DR-06 (BR-02).** The falsification is executed and recorded: a page is edited so
  one of its claims is no longer true of the library, the gate fails naming the page
  and location, the edit is reverted, and the gate returns green.
- **DR-07 (BR-09, machine half).** Every clause id a narrative page cites resolves
  against `spec/SPECIFICATION.md`; a dangling citation fails the gate. Ids are stable
  names, not line references (`spec/SPECIFICATION.md:280`), so this survives the
  521-line divergence on the sibling branch noted in `_decomposition.md`.
- **DR-08 (BR-11, machine half + DT-7).** Either no shipped content uses hidden,
  tabbed or collapsed panels, or a deliberately broken claim *inside a hidden panel* is
  observed to fail the gate. Assertion is not acceptable; the demonstration is.
- **DR-09 (BR-10).** The frozen documentation MUSTs are enumerated by clause id in
  exactly one place in the tree, before any `happenstance-core` doc comment is touched
  by any project, and `cargo xtask spec-trace` passes over the result. Any later touch
  of such a comment is governed by
  `.kb/governance/rewrite-the-referent-never-the-reasoning.md`: rewriting the referent
  is permitted, rewriting the reasoning is not.
- **DR-10 (BR-01 + hosting).** Whatever renders the narrative surface renders inside
  `cargo xtask ci` from a clean checkout, with no manual step anyone has to remember.
- **DR-11 (honesty obligation, house practice).** The new step's own documentation
  states what it does *not* verify — at minimum: that it cannot detect code that still
  compiles while no longer demonstrating the surrounding claim, and that `cargo test
  --doc` drops `RUSTDOCFLAGS` (rust-lang/cargo#13697, still-open upstream
  rust-lang/rust#67533), so the enforced lint level inside a fence is narrower than the
  workspace `[lints]` table. `xtask/src/main.rs:284-302` and
  `xtask/src/constitution.rs:26-36` already carry this argument for the existing
  rustdoc steps.
- **DR-12 (dependency hygiene).** Any dependency the mechanism adds is justified
  against the standing decision already recorded in `xtask/Cargo.toml:16-21`, which
  deliberately excludes `rusqlite` and `sqlx` from xtask's dev graph because every
  `cargo xtask ci` would then build them.

## Acceptance criteria

Project-grain and testable. These ids are the spine the story map must cover.

- **AC-001 — The tree is pinned, not conventional.** The narrative tree's path appears
  as a constant in `xtask/src/`. Moving or renaming the tree without editing
  `xtask/src/` in the same change fails `cargo xtask ci` with a message naming the
  expected path.
- **AC-002 — Every fence is compiled against the real crates, mandatorily.** A
  `REQUIRED` step with `probe: None` compiles every Rust fence in the tree against the
  actual workspace crates. Removing a public item that a page calls makes the gate
  fail.
- **AC-003 — The check has been seen to fail.** A page is deliberately edited so one
  claim is no longer true of the library; `cargo xtask ci` **fails** and identifies the
  page and the location; the edit is reverted and the gate returns green. Both
  observations are recorded in the project's own artefacts.
- **AC-004 — No silent opt-out.** Adding an `ignore`-class fence (or any equivalent
  spelling) to a narrative page fails the gate with the file and line named, unless the
  fence is on an enumerated allowance list that the same check reads.
- **AC-005 — No orphan pages.** A markdown page added to the pinned tree but not
  reachable by the compiling mechanism fails the gate, rather than shipping unchecked.
- **AC-006 — Hidden content is inside the check, or absent.** Either the design record
  forbids hidden/tabbed/folded content for anything load-bearing, or a claim broken
  *inside* a non-default panel is observed to fail the gate. DT-7 is resolved in
  `_design.md` either way.
- **AC-007 — Citations resolve.** A narrative page citing a `SPECIFICATION.md` clause
  id that does not exist fails the gate. A page citing one that does exist passes.
- **AC-008 — The frozen documentation MUSTs are pinned.** The exact set is enumerated
  by clause id in exactly one file in the tree, `cargo xtask spec-trace` passes over
  the tree as it stands, and the enumeration is in place before any project rewrites a
  `happenstance-core` doc comment.
- **AC-009 — Clean checkout, no manual step.** From a fresh clone with no local state,
  `cargo xtask ci` runs green and the narrative material builds and renders as part of
  it. The hosting/render shape is recorded in `_design.md`.
- **AC-010 — The limits are on the record.** The new step's own documentation states
  what it does not verify, naming at least the compiles-but-no-longer-demonstrates
  blind spot and the `RUSTDOCFLAGS` gap, so no downstream project can read the green
  step as evidence of teachability.

Initiative-grain coverage discharged by these: BR-01 (AC-002, AC-004, AC-005), BR-02
(AC-003), BR-09 machine half (AC-007), BR-10 (AC-008), BR-11 machine half (AC-006),
BR-12 pinning half (AC-001); AC-03 and the machine half of AC-12; DoD scenarios 1
(AC-009), 2 (AC-003), 11 (AC-008), 13 (AC-006); DT-7 (AC-006).

## Definition of done (boundary-level)

1. `cargo xtask ci` is green from a clean checkout, with the narrative build and render
   inside it as an ordinary step — not a separate thing someone remembers to run.
2. The deliberately-broken-page falsification has been **run and observed to fail**,
   then reverted and observed green, with both halves written down. A gate that has
   only ever been green is decorative.
3. If any shipped content is hidden behind a fold, tab or panel, the same falsification
   has been run *inside* a hidden branch. Otherwise the design record states that no
   such content carries a load-bearing claim.
4. `_design.md` records DT-7's resolution and the hosting/render shape, with the
   alternatives that lost. `design.capture` is deliberately absent from
   `.redkiln/config.yaml`, so the perceptual review is a skip and the written record is
   the *only* record.
5. The frozen documentation MUSTs are enumerated by clause id in one place, and
   `cargo xtask spec-trace` passes over the tree as it stands.
6. `cargo xtask ci --fast` is the bar for this non-terminal project per
   `.redkiln/config.yaml`; the full gate is run before the project is called done.
7. Every new check in `xtask/src/` carries a "what this does not verify" section, and
   `cargo xtask lint-constitution` plus `cargo test -p xtask --doc` still pass.
8. Nothing in this project asserts, anywhere, that the surface proves a page teaches.

## Dependencies

**Depends on:** nothing. HS-P0020 is first in merge order and has no inbound edge
(`.bklg/docs-that-teach/_decomposition.md` "Dependency graph").

**Unlocks:** HS-P0021 `page-need-discipline`, HS-P0022 `application-author-path`,
HS-P0023 `reach-and-adapter-path` directly; HS-P0024 `comprehension-evidence` and
HS-P0025 `durable-audience-closeout` transitively.

**Parallelism available:** HS-P0021's design stage runs alongside this project's
implementation. Nothing here waits on any sibling.

**Cross-branch note, not a dependency:** `initiative/from-contract-to-published-library`
is unmerged and diverges from `spec/SPECIFICATION.md` here by 521 lines. The
decomposition's gate decision is to run in parallel and merge forward before the pull
request; clause ids are stable names rather than line references
(`spec/SPECIFICATION.md:280`), so the pin in AC-008 survives the divergence. The
residual risk — that the sibling *adds* documentation MUSTs — is re-checked by HS-P0025
at closeout.

## Risks and coupling notes

| Risk | Note |
| --- | --- |
| A green gate is read as evidence of teachability | The single largest risk in the initiative and the one this project is most able to cause. Mitigated by AC-010 and by DoD item 8; the friction log (HS-P0024) is not substitutable |
| The step is wired and never fails | `RUNBOOK.md:918-925` is the in-house precedent — a probe-gated step printed `skipped` on all three runners while two documents vouched for it. AC-003 is the answer, and `probe: None` is the structural half |
| `cargo test --doc` drops `RUSTDOCFLAGS` | Upstream and still open (rust-lang/cargo#13697 → rust-lang/rust#67533). Switching between `mdbook test` and `include_str!` does not fix it, because both route through rustdoc's doctest machinery. Must be *recorded*, not routed around |
| Degraded diagnostics under `include_str!` | A failing fence inside an included markdown file is reported against the *including* item's file and line, not the markdown's. Detection is fine; the pointer is degraded — and AC-003 requires the gate to identify "the page and the location", so this is a real constraint on the mechanism choice, not a footnote |
| A new tool widens the dev graph | `xtask/Cargo.toml:16-21` already records the standing trade for `rusqlite`/`sqlx`. An `mdbook` binary dependency is a *probe* shape by default, which AC-002 forbids — resolving that tension is design work, not implementation work |
| `ignore` returns by the back door | Already found once in this repository's own testkit doctests. AC-004 is a grep-class check, and `xtask/src/lints.rs` is the precedent for why a grep is in a Rust gate |
| Hidden panels are assumed safe | Nothing in `mdbook-tabs`' own documentation states whether an inactive panel is inside `mdbook test`. Unverified, not verified-safe |
| File collision with HS-P0016 on the sibling branch | `docs/README.md` and the crate-root docs are the likely shared files. The seam is purpose, not paragraph: that project asks whether a claim is true, this one wires a check. Merge forward before the PR |
| Scope creep into page content | Every teaching page belongs to HS-P0021/22/23. This project may write *fixture* pages for the falsification; it may not write the corpus |
| The `page-need` metadata shape leaks in | If HS-P0021's discipline needs a machine-checkable per-page field, the *check* may land here later. It is not in this project's scope, and adding it early would give this project a second job — the exact defect BR-04 exists to make visible |

## Context anchors

Initiative and decomposition:

- [`../initiative.md`](../initiative.md) — the charter: BR-01, BR-02, BR-09 to BR-12,
  AC-03, AC-12, DoD 1/2/11/13, DT-7
- [`../_decomposition.md`](../_decomposition.md) — why this cut, the DAG, and the gate
  decisions on the sibling branch and on where the discipline lives
- `.bklg/docs-that-teach/_discovery/research/02-compiled-prose-tooling-mdbook-test-doc-comment-skeptic-doc-i.md`
  — the five-tool survey, what each is blind to, and the upstream `RUSTDOCFLAGS` gap
- `.bklg/docs-that-teach/_discovery/distillation/interaction-patterns.md` — DT-7's
  evidence (tension 1), the hidden-content anti-patterns, and the checkable test for
  each

Knowledge base:

- `.kb/governance/rewrite-the-referent-never-the-reasoning.md` — the test for any touch
  of a doc comment that already discharges a frozen clause
- `.kb/concepts/torn-reads-and-the-append-condition-boundary.md` — an existing correct
  account of the mechanism the teaching pages will describe; cited here only so this
  project's fixture pages have something true to cite

Gate and standards:

- `xtask/src/main.rs:72-103` — the `Step` contract, and why `probe: None` is the
  difference between "the tool is missing" and "the tool found a problem"
- `xtask/src/main.rs:284-302` — the documentation step, and the `RUSTDOCFLAGS` comment
  recording the "printed warnings and exited 0" incident
- `xtask/src/constitution.rs` — the working precedent: markdown compiled as doctests of
  a `publish = false` crate, one module per file, with the `cfg(doctest)` invisibility
  hole named at lines 20-25
- `xtask/src/lint_constitution.rs:54-61` — `ATOM_DIR`, `ROUTER`, `HARNESS`: a tree
  pinned by path, in code
- `xtask/src/lint_constitution.rs:9-28` — the "what this does not verify" shape every
  new check owes
- `xtask/src/lints.rs` and `xtask/src/spec_trace.rs` — why a grep is in a Rust gate, and
  the citation-resolution machinery to model AC-007 on
- `xtask/Cargo.toml:16-21` — why happenstance crates are xtask dev-dependencies, and
  which drivers are deliberately absent
- `docs/README.md:25-29` — the statement of why the gate reads trees by path
- `standards/rust/70-rustdoc-obligations.md` — the only constitution atom on
  documentation, and the source of the green-but-vacuous counter-example
- `standards/rust/README.md:23-29` — the precedence chain this work sits inside and does
  not extend
- `spec/SPECIFICATION.md:280` — clause ids are stable and never renumbered
- `RUNBOOK.md:918-925` — the decorative-step precedent this project exists not to repeat

## Companions

- [`_decomposition.md`](_decomposition.md) — this project's briefs (architecture, ux,
  testing, deployment)
- [`_storymap.md`](_storymap.md) — the vertical-slice story map
- [`_intake-brief.md`](_intake-brief.md) — this project's intake artifact
- [`../initiative.md`](../initiative.md) — the initiative charter
- [`../_plan.md`](../_plan.md) — the initiative planning rollup
