# Phase 20 — Documentation that teaches

**Goal.** The work planned as `HS-I0007` ("From Accurate to Teachable") is done:
every API claim in the user documentation is checked by the gate, every page declares the one need it answers, and a reader who is not the author has used
it and been listened to.

**Why here.** 1.0 is a promise made to people who will not read the
specification. Documentation that is accurate and cannot be learned from is the
failure mode that initiative was chartered against, and it is the part of 1.0 no
type checker can see.

**This phase does not re-plan that work, and it now tracks it.**
`.bklg/docs-that-teach/` holds its charter, its projects and its specified
stories, and those `spec.md` bodies are still the plan. Its recorded stages are
not: `.bklg/` is frozen (D-2), so progress is ticked here, project by project,
and nowhere else.

At the split, three of its six projects (`HS-P0020` – `HS-P0022`) sit at review with
their stories at `report`, and three are not yet built: `HS-P0023`
reach-and-adapter-path (blocked, after a `changes-requested` slice),
`HS-P0024` comprehension-evidence and `HS-P0025` durable-audience-closeout.

**Work carried in from phase 16**

- [x] #149 · **ES-23's adapter half, for the next adapter author**
      (`.kb/open-questions/es-23-frozen-doc-musts-adapter-half.md`). All four
      published event-store adapters now carry their own `# Cancellation`
      section, so ES-23 is met; what nothing does is point a new adapter author
      at the obligation, or at the per-adapter test targets to copy. Write that
      pointer where an adapter author reads, and answer the question the atom
      still asks: whether `FROZEN_DOC_MUSTS` (`xtask/src/lint_narrative.rs`)
      gains a third disposition — an adapter-owned obligation it points at but
      cannot run — or keeps its documented scope with the gap stated. Additive
      either way; phase 21's clause audit reads ES-23 as frozen and met.
      **Done 2026-10-11:** the third disposition, `AdapterOwned`, with ES-23's
      adapter row naming the four `cancellation_statement.rs` targets; the
      pointer is in `docs/adapter-reading-order.md`, the recipe atom and
      `CONTRIBUTING.md`, and the atom is superseded.

**Work carried in from phase 21's audit**

- [ ] #188 · **The rules three `[FROZEN]` ES clauses name and nothing owned**
      (`kb-decision-0114`, proposed 2026-11-18). This phase owns them because it is
      the last phase phase 21 waits on that is not `done`, which is PS-40's
      precedent. Phase 21 settles nothing, and its audit reads a frozen clause as met.
      ES-6's `store_error_crosses_a_join_handle`, bound by ADR-0009's marker. ES-31's
      `checkpoint_lag_is_not_a_position_difference`, beside the typed runner. ES-29's
      `wire_condition_with_after_is_refused`, shared with SY-6, which needs SY-6's
      refusal built in the shape the owner picks in that record. Each rule lands with
      its wrong implementation and takes its `Scheduled` entry out of
      `xtask/src/spec_trace.rs`.

**Proof artefact.** The initiative's own Definition of Done
(`.bklg/docs-that-teach/initiative.md`), re-observed from a clean checkout, which is
what its terminal project `HS-P0025` was specified to do.

**Exit criteria**

- [ ] `HS-P0023`, `HS-P0024` and `HS-P0025` done, each against its stories'
      `spec.md` acceptance criteria, ticked in this file's session log.
- [ ] #150 · `HS-P0020` – `HS-P0022`, which sat at review when `.bklg/` froze, confirmed
      shipped or reopened here.
- [ ] The narrative tree's checks are in `cargo xtask ci`, and have been watched
      failing on a deliberately broken page.
- [ ] The specification is reconciled against this phase's changes (session
      protocol step 6), and `cargo xtask spec-trace` passes.

**Estimate.** Not yet made. Three unbuilt projects, 29 specified stories; the
first session on this phase owes a number.

**Session log**
