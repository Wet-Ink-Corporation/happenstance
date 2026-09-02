# Grounding — The Durable, Reconciled Audience (HS-P0025)

Companion to `_intake-brief.md` (an unfilled template — see note below) and
`_decomposition.md` (the real source of this project's mandate). Written for the
briefs/design/story-map stages that follow. Every path below was checked against
this worktree; none is invented.

## Note on `_intake-brief.md`

`.bklg/docs-that-teach/durable-audience-closeout/_intake-brief.md` still carries the
unfilled `_intake-brief` template — every section is the placeholder sentence, not
project content (`item: HS-P0025`, `stage: intake`, checked-off gate boxes with
templated prose). It is not a usable source. `project.md` (already advanced to
`stage: storymap`) and `_decomposition.md` carry the actual objective, ACs, scope,
dependencies and brief tags, and this note treats those two as the gold source
throughout, matching what `project.md` itself already does.

## What kind of project this is, and what that means for "Accepted decisions"

This project touches no Rust contract code. Its diff is confined to `.kb/product/`,
`.kb/maps/domain-map.md`, `.kb/maps/open-questions-index.md`, possibly
`.kb/open-questions/`, plus the merge-forward of the sibling branch and a full
`cargo xtask ci` run. None of the sixteen **accepted** ADRs (`0001`–`0016`, `0029`;
`0002` is `status: superseded` per `.kb/decisions/0002-crate-naming.md:5`) binds
persona content, KB promotion, or documentation structure — CLAUDE.md says so
directly: *"none of the seventeen decisions concerns documentation"*
(`.bklg/docs-that-teach/initiative.md:524-525`). The constraints that actually bind
this project are **process and governance atoms**, not the ADR corpus, and the two
sibling design tensions it might have touched (documentation structure, MSRV) are
owned elsewhere in the decomposition, not here.

So "what constrains HS-P0025" is answered by:

1. `.kb/governance/rewrite-the-referent-never-the-reasoning.md`
   (`kb-governance-referent-not-reasoning-001`, `status: accepted`,
   `authority_tier: guideline`) — the discrimination the corpus uses for any edit
   near a decision: a rename may be rewritten in place, reasoning inside a
   standing decision is never touched, and a superseded decision's body stays
   factually intact. Binds this project's "no ADR as a side effect of promotion"
   non-goal directly — if reconciliation surfaces something that looks like a
   decision, the corpus's own rule is that a correction to a standing decision is
   a **new atom that supersedes**, never an edit, and inventing one as an
   incidental output of a closeout is exactly the kind of unearned authorship the
   atom exists to name.
2. The **single-write-path rule**: `.kb/` atoms are authored only by
   `/redkiln:kb-ingest` from `.kb/_intake/`, never by hand. Confirmed both by
   CLAUDE.md's "Where the work lives" section and by the commit history itself —
   `0269720` ("Revert the hand-authored backlog and knowledge base") exists in
   this worktree's log (`git log --oneline --all`) and is the concrete precedent
   the project's out-of-scope list cites. This is the load-bearing constraint on
   AC-008/AC-009/DR-7/DR-8: every persona/journey atom this project lands must
   pass through an ingest wave, and the wave must clear its own staged payload
   without ingesting `.kb/_intake/README.md` as if it were content.
3. **The project process itself** (`.redkiln/processes/project.yaml:76-80`):
   the `closeout` stage is the only one carrying `harvest_kb: true`, which is the
   mechanical reason promotion is a closeout-shaped obligation and cannot happen
   earlier in this project's own stages.
4. **The gate wiring** (`.redkiln/config.yaml:60`, `e2e: "cargo xtask ci"` —
   verified present at that line) is what makes this project's terminal
   Definition-of-Done obligation (AC-016) a real, already-wired command rather
   than a new one this project would have to invent.

## Existing patterns this project must follow

**KB atom shape for the product layer.** `.kb/product/README.md:6-13` fixes the
two kinds this project must emit: persona atoms are `kind: concept`, journey
atoms `kind: playbook`, both `authority_tier: product`. That is a **different**
`authority_tier` from every existing atom sampled below (`note`, `guideline`) —
worth flagging explicitly to whoever authors the intake payload, since it is easy
to default to the tier the nearby examples use rather than the tier the README
that governs this specific layer actually names.

Two existing atoms show the concrete shape a well-formed atom takes and are the
right models to imitate (frontmatter: `id`, `title`, `kind`, `status`,
`authority_tier`, `summary`, `depends_on`, `related`, `source_paths`):
- `.kb/concepts/torn-reads-and-the-append-condition-boundary.md`
  (`kb-concept-torn-read-append-boundary-001`) — a `concept` atom, the same
  `kind` a persona atom will use.
- `.kb/playbooks/one-decision-per-adr-title.md`
  (`kb-playbook-one-decision-per-adr-title-001`) — a `playbook` atom, the same
  `kind` a journey atom will use.

Both cite `source_paths` back to the intake document and (where relevant) the
long-form record — the same pattern DR-4/AC-005 require here: every promoted
atom's `source_paths` must cite a real path under
`.bklg/docs-that-teach/_discovery/`.

**Map maintenance is append-only.** `.kb/maps/domain-map.md:144-150` ("Adding a
domain" — the file is 150 lines total) states the rule directly: append a new
`##` section, never edit an existing one, because a domain is a subject area
that should outlive the ingest wave that first populated it. The file today has
exactly the sections CLAUDE.md describes (governance, playbooks, open questions
for the existing two domains); documentation/product has no section yet, which
is what DR-9/AC-010 require this project to add.

`.kb/maps/open-questions-index.md` shows the parallel pattern for open
questions: append a bullet under the domain section the question belongs to
(its own "Adding an entry" section), state status first (`Open` /
`Withdrawn` / `Superseded`), then the atom id, then one sentence — grounding
lives in the atom, not the index. This is the pattern DR-13/AC-017 must follow
for any charter open question that becomes an `open_question` atom rather than
being answered on the record.

**Open-question atom shape**, per `.kb/open-questions/README.md`: state what is
true today, what is not decided, and what forces it; ground every claim with
`source_paths` the way a `reference` atom would. Relevant here because at least
one of the initiative's five open questions (the evaluator-persona question,
AC-003/DR-6) is explicitly this project's to close — it may resolve as a
decision recorded in the promoted atoms rather than as a new open-question atom,
per the charter's own framing (`initiative.md:538-540`).

**The `_intake/` ingest glob is real and literal.** `.kb/_intake/README.md:5-7`
states the default input is `.kb/_intake/*.md`, and the directory today contains
only `README.md` — confirming DR-8's risk is live, not theoretical: an
unfiltered ingest run over that glob would attempt to ingest the README itself
unless it is excluded at the ingest invocation (per the project's own
"drop it at the approval gate" risk note, and consistent with the maintainer's
own recorded lesson that this exact glob sweeps the README in).

## Reconciliation counterpart does not exist in this tree

Confirmed directly rather than only asserted: this worktree's `.bklg/` contains
exactly two initiatives, `docs-that-teach` and `support`
(`ls .bklg`) — no `HS-P0019`, no `HS-S0131`, no
`from-contract-to-published-library` directory anywhere under `.bklg/`. The only
place those ids appear in this tree is as **prose references** inside this
initiative's own discovery and planning documents (personas-and-journeys.md,
initiative.md, `_decomposition.md`), not as items `redkiln` can see. That
directly corroborates `_decomposition.md:116-123`'s claim ("HS-S0131 ... is
still `stage: plan`, `status: ready` — unrun, on the unmerged branch... There is
nothing to consume today") and grounds DR-2 / AC-002's requirement that the
reconciliation record must state, honestly, which of the two cases actually
held at the time this project ran.

## Tensions and risks worth carrying into the design/briefs stages

- **Authority-tier mismatch is an easy silent defect.** Every sampled atom in
  the corpus uses `authority_tier: note` or `guideline`; the product layer's own
  README requires `product`. A briefs/design author who pattern-matches on the
  nearest example rather than reading `.kb/product/README.md:6-13` directly will
  ship the wrong tier, and nothing except `redkiln validate --kb` (AC-011) and a
  human reviewer would catch it — it is exactly the kind of defect this
  grounding note exists to head off.
- **`redkiln validate --kb` is not exercised by this grounding pass** — it
  requires a real ingest wave to run against, which is implementation, not
  planning. AC-011's bar is real but unverifiable from a planning-grain read; the
  briefs stage should say so rather than imply it was checked here.
- **The "clean checkout" requirement (AC-014) and this worktree are in tension
  by construction.** This project's own planning work is being done inside a
  dedicated worktree, and its terminal DoD re-observation is explicitly required
  to run "from a clean checkout" on the **merged** tree — not this worktree as
  it stands mid-planning. The design/testing briefs should treat "clean
  checkout" as a literal instruction to the implementation stage (a fresh clone
  or `git worktree add` off the merged branch), not as already satisfied by the
  worktree this planning pass is running in.
- **No Accepted ADR is at risk of being violated by this project's normal
  operation**, because its diff surface (`.kb/product/`, `.kb/maps/`,
  `.kb/_intake/`, the merge-forward, the gate run) does not touch
  `happenstance-core`, `happenstance`, or any adapter crate. The one way this
  project *could* create ADR-shaped tension is exactly the failure mode its own
  non-goals name: writing an ADR as a side effect of reconciliation. The
  governance atom above is the citation that makes that non-goal load-bearing
  rather than a stylistic preference.

## Anchors (verified present in this worktree)

- `.kb/decisions/0002-crate-naming.md:5` — the one `status: superseded` decision;
  all others sampled (`0001,0003,0004,0005,0006,0007,0008,0009,0010,0011,0012,
0013,0014,0015,0016,0029`) are `status: accepted`.
- `.kb/governance/rewrite-the-referent-never-the-reasoning.md`
- `.kb/product/README.md:6-13`, `:23-31`
- `.kb/design/README.md`
- `.kb/maps/domain-map.md:144-150`
- `.kb/maps/open-questions-index.md`
- `.kb/open-questions/README.md`
- `.kb/_intake/README.md:5-7`, `:14-19`
- `.kb/concepts/torn-reads-and-the-append-condition-boundary.md`
- `.kb/playbooks/one-decision-per-adr-title.md`
- `.redkiln/processes/project.yaml:76-80`
- `.redkiln/config.yaml:60`
- `spec/SPECIFICATION.md:280` — clause ids stable, never renumbered
- `xtask/src/main.rs:324` — `"spec-trace"` as a named CI step
- `.bklg/docs-that-teach/_decomposition.md:89-123`, `:236-239`, `:306-308`
- `.bklg/docs-that-teach/initiative.md:513-556`
- `.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md`
  (all sections, especially `## Risks` and `## Open questions for the planning
  team`)
- Commit `0269720` — "Revert the hand-authored backlog and knowledge base"
  (`git log --oneline --all`)
