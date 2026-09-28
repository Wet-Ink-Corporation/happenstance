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

**Proof artefact.** The initiative's own Definition of Done
(`.bklg/docs-that-teach/initiative.md`), re-observed from a clean checkout, which is
what its terminal project `HS-P0025` was specified to do.

**Exit criteria**

- [ ] `HS-P0023`, `HS-P0024` and `HS-P0025` done, each against its stories'
      `spec.md` acceptance criteria, ticked in this file's session log.
- [ ] `HS-P0020` – `HS-P0022`, which sat at review when `.bklg/` froze, confirmed
      shipped or reopened here.
- [ ] The narrative tree's checks are in `cargo xtask ci`, and have been watched
      failing on a deliberately broken page.

**Estimate.** Not yet made. Three unbuilt projects, 29 specified stories; the
first session on this phase owes a number.

**Session log**
