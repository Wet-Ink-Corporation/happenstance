---
title: Backlog adjacency — From Accurate to Teachable
kind: grounding/summary
---

# Backlog adjacency

Grounding pass for `docs-that-teach` (HS-I0007) against the rest of the backlog and
knowledge base. One finding dominates the rest: **the backlog this worktree can see
is not the whole backlog.** `docs-that-teach` was branched from `main` at `c58ba64`
/ `31174d5`, and on `main`, `.bklg/` currently contains only `support` (HS-I0005) —
confirmed by `git ls-tree -r --name-only main -- .bklg`. The initiative this
initiative's own intake brief cites by id and leans on for a scope boundary,
`from-contract-to-published-library` (HS-I0006), is real, extensively developed,
and **not merged to `main`** — it lives on its own unmerged branch
`initiative/from-contract-to-published-library`, checked out in the sibling
worktree `D:/repos/happenstance/.claude/worktrees/from-contract-to-published-library`
(`git worktree list`). Everything below that is cited from that tree is cited
read-only, from a branch this initiative does not yet depend on structurally, only
by the prose of its own brief.

## The adjacent initiative: HS-I0006, `from-contract-to-published-library`

Title *"From Contract to Published Library"*, `status: implementing`,
`stage: implementation`, ten projects
(`.bklg/from-contract-to-published-library/initiative.md`, on that branch). It is
the umbrella that grew out of `references/seeds/remaining-runway.md` — "what is
left is the half that can only be settled by contact" — and adapters, publication
and closeout are three of its ten projects. `docs-that-teach`'s own seed
(`references/seeds/user-documentation.md`, reconciled 2026-08-16 per the intake
brief) sits underneath that same umbrella in spirit but is being run as a separate
Redkiln initiative rather than an eleventh project of HS-I0006.

### The named boundary: HS-P0016, `publication-and-positioning`

Confirmed at
`.bklg/from-contract-to-published-library/publication-and-positioning/project.md`
on the sibling worktree: `id: HS-P0016`, `parent: HS-I0006`, `status: in-review`,
`stage: design`, title *"0.2.0 — where private opinions become promises."* Its
`_design.md` states its surface plainly: "this project ships almost no Rust API...
its designed surface is three rendered pages — the crates.io front page, the
docs.rs page, and the repository landing page." Ten stories under it —
`clause-maturity-audit`, `compliance-claim-and-gaps-promise`, `crate-set-decision`,
`deferred-clause-reread`, `falsifier-ledger-repair`, `first-contact-design-
resolutions`, `guarantees-and-docs-rs-presentation`, `landing-copy-and-status-
truth`, `msrv-promise-atom`, `projection-port-ship-shape`, `publish-0-2-0`,
`registry-surface-diff`, `rendered-page-preflight`, `stranger-install-smoke` —
none of which is a narrative page, a book, or a doctest; they are compliance
claims, README/landing copy truth, and the mechanics of the publish act itself.

The `docs-that-teach` intake brief's own non-goal line is accurate against this:
*"the published surface — README landing copy, status-truth vocabulary, the
maturity census, peer positioning, docs.rs manifest metadata and rendered-page
preflight — is owned by project `publication-and-positioning`... against a
`_design.md` signed off with no conditions on 2026-08-12."* That sign-off date and
the "no conditions" claim both check out against the file read directly. The
boundary this suggests, restated in problem-space terms rather than solutioneering:
`publication-and-positioning` owns *whether what the surface says is true*;
`docs-that-teach` owns *whether a reader can be taught*. Where the two would
collide is the README — `publication-and-positioning`'s `landing-copy-and-status-
truth` story already claims the workspace README's truth-telling, so any narrative
rewrite of README prose this initiative proposes is a shared-file risk even where
the *purpose* split is clean. That risk is not new — it is the ordinary hazard of
two initiatives both wanting a well-worded pen near the same file — but it is worth
naming as a NON-GOAL candidate: **`docs-that-teach` should not restate or re-decide
README landing copy / status-truth vocabulary**, only cite or extend it narratively
where `publication-and-positioning` has already settled the words.

### The named dependency: HS-S0131, `persona-and-journey-intake-staging`

Confirmed at
`.bklg/from-contract-to-published-library/closeout-and-durable-audience/persona-and-journey-intake-staging/story.md`:
`id: HS-S0131`, `parent: HS-P0019` (project `closeout-and-durable-audience`, itself
confirmed `id: HS-P0019`, `parent: HS-I0006`, `status: in-review`, `stage: design`),
`status: ready`, `stage: plan`, `blocks: [HS-S0132]`. Title: *"Four personas and
their journeys staged in `.kb/_intake/`, qualification in frontmatter."* This is
the story the intake brief flags as the real dependency risk — *"two initiatives
independently inventing personas is the failure mode."* Two things follow from
reading it directly rather than from the brief's paraphrase:

1. **It has not run.** `stage: plan` means its spec exists but nothing has been
   implemented; no personas are staged in `.kb/_intake/` yet, and none exist in
   `.kb/product/` (confirmed empty but for a README — `.kb/product/README.md`,
   `.kb/design/README.md`). `docs-that-teach`'s open question — "who supplies the
   audience model" — currently has **no answer available to consume**, staged or
   otherwise, from either the current worktree or the sibling one.
2. **It is on the unmerged branch.** Even once HS-S0131 runs and personas land in
   `.kb/_intake/`, and even once `kb-ingest` promotes them into `.kb/product/`
   atoms, that will happen on `initiative/from-contract-to-published-library` (or
   whatever descendant branch closes it out) and will not exist on `main` — and
   therefore not in a `docs-that-teach` worktree branched before that merge — until
   someone merges it forward. This is a sequencing dependency in substance, not
   just in the brief's prose: **`docs-that-teach` cannot honestly claim an audience
   model until either HS-S0131 has run and merged to `main`, or this initiative
   stages its own equivalent persona work** — and the intake brief already forecloses
   the latter as a failure mode. The open question needs a decision — consume after
   merge (this initiative blocks on HS-S0131's completion and a merge to `main`) or
   supersede (HS-S0131 is dropped and `docs-that-teach` owns persona staging) —
   and it is not decidable from this initiative's own tree; it needs the human or a
   cross-initiative coordination step, because the two initiatives' branches
   currently cannot see each other's state through Redkiln tooling (`redkiln
   status` in this worktree reports only 2 initiatives — `docs-that-teach` and
   `support` — not `from-contract-to-published-library`, because it is not on this
   branch).

### Precedent already in the tree: HS-P0011, `typed-layer-and-alpha-release`

Confirmed at
`.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/project.md`:
`id: HS-P0011`, `status: in-review`, `stage: review`, title *"The typed layer, the
worked example, and 0.2.0-alpha.1."* This is the project that built the worked
example and published `0.2.0-alpha.1` — the release the `docs-that-teach` intake
brief's Problem section uses as its central piece of evidence (the `Tags::empty()`
double-call in the opening program). It is not a boundary or a dependency so much
as **the origin of the evidence this initiative is built on**: the alpha it
critiques was shipped by a sibling project of the same unmerged initiative, still
sitting at `stage: review` rather than closed. Worth naming because a reader of
`docs-that-teach`'s brief alone would not know the alpha's own project had not yet
finished its review — the documentation gap this initiative names may still be
moving under it.

## `support` (HS-I0005)

The only other initiative visible on `main` / in this worktree. `status:
exploring`, `stage: intake`, title "Support" — a generic reactive-fix container
(the `forge-fix` lane's parent), not a body of user-facing or narrative work.
Confirmed at `.bklg/support/initiative.md`. No overlap in subject matter; noted
only because it is the entirety of what `redkiln status` in this worktree reports
alongside `docs-that-teach` (`0/0 stories done-or-in-review across 2 initiative(s)`).

## Knowledge-base grounding for the open questions

- **"Where a documentation standard lives, if one is written"** — confirmed open.
  `.kb/maps/domain-map.md` lists exactly two domains as of its last wave
  (*Specification governance & conformance*; *Contract ports, conformance, and the
  ADR corpus*) and neither is documentation-shaped. `standards/rust/` (the Rust
  constitution CLAUDE.md points to) is a sibling tree to `.kb/`, not part of it, and
  the brief already confirms twenty-eight atoms there hold no narrative-structure
  rule. There is no existing `.kb/` atom, playbook, or governance record this
  initiative would collide with or need to supersede on that question — it is a
  genuinely open field, not a contested one.
- **"Who supplies the audience model"** — see HS-S0131 above; this is the one place
  where the KB grounding and the backlog grounding are the same finding. `.kb/
  product/` and `.kb/design/` hold READMEs only (confirmed by direct listing), so
  there is no persona atom anywhere in the tree, staged branch or not, for this
  initiative to consume today.
- **RUNBOOK phase sequencing** — `RUNBOOK.md` (this worktree) confirms phase 12 is
  the publish act the intake brief calls "the irreversible act" (`RUNBOOK.md:393`,
  ADR-0004 loses `provisional` at phase 12) and that adapter phases 8–11 are where
  the sibling initiative's adapter projects (`sqlite-durable-store`,
  `cloudflare-durable-object-store`, `postgres-and-neon-stores`,
  `ladybug-projection-store`, `replication-identity-and-ingest`,
  `retention-and-incomplete-logs`) sit. `docs-that-teach` names no phase for itself
  in its brief (an open question), but its fuse — "phase 12 is the irreversible
  act" — is stated correctly against the plan of record.
- **`references/seeds/measured-not-claimed.md`** — the intake brief's non-goals
  section cites this seed as the home for benchmarks/performance reporting, folded
  out of scope. It does not exist in this worktree
  (`references/seeds/` here holds only `remaining-runway.md` and
  `user-documentation.md`). This may be a seed staged on another branch (the same
  pattern as HS-I0006) or a forward reference not yet materialized; either way, the
  exclusion the brief states is sound regardless of where the seed file itself
  currently lives — benchmarks are out of scope here on the brief's own terms, not
  contingent on that file's presence.

## Suggested NON-GOALS for distillation, restated from what was actually verified

1. **Do not restate or re-decide README landing copy or status-truth vocabulary** —
   `publication-and-positioning`'s `landing-copy-and-status-truth` story owns those
   words; a narrative page may cite or link them but should not fork a second
   version of what the README already says.
2. **Do not invent a second persona/audience-model source** — HS-S0131 is already
   specced for exactly this staging, on `HS-P0019`/`HS-I0006`. Distillation owes a
   real decision (consume-after-merge vs. supersede), not a default.
3. **Do not treat `0.2.0-alpha.1`'s adequacy as this initiative's to judge** —
   `typed-layer-and-alpha-release` (HS-P0011) is still `stage: review`; the alpha's
   own project has not closed. `docs-that-teach` uses it as evidence of a
   documentation gap, which is fair, but fixing what the alpha *contains* (versus
   how it is *taught*) is not this initiative's boundary to redraw.
4. **Do not claim a documentation-standard `.kb/` domain is contested** — it is not;
   the field is empty, confirmed above, so this is a placement decision for
   distillation rather than a negotiation with another initiative.

## Files cited

- `.bklg/docs-that-teach/_intake-brief.md` (this worktree)
- `references/seeds/remaining-runway.md`, `references/seeds/user-documentation.md`
  (this worktree)
- `.kb/maps/domain-map.md`, `.kb/product/README.md`, `.kb/design/README.md`
  (this worktree)
- `RUNBOOK.md` (this worktree, phase 12 / phase 8-11 sequencing)
- `.bklg/support/initiative.md` (this worktree)
- On the sibling worktree
  `D:/repos/happenstance/.claude/worktrees/from-contract-to-published-library`
  (branch `initiative/from-contract-to-published-library`, read-only — not merged
  to `main`, not visible from this worktree's own `.bklg/`):
  - `.bklg/from-contract-to-published-library/initiative.md` (HS-I0006)
  - `.bklg/from-contract-to-published-library/publication-and-positioning/project.md`
    and `_design.md` (HS-P0016)
  - `.bklg/from-contract-to-published-library/closeout-and-durable-audience/project.md`
    (HS-P0019) and `.../persona-and-journey-intake-staging/story.md` (HS-S0131)
  - `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/project.md`
    (HS-P0011)
