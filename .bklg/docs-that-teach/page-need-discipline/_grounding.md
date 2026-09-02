# Grounding — Page-Need Discipline (HS-P0021)

Written for `/redkiln:plan`'s briefs stage. Every claim below was verified against
this worktree directly (line numbers re-checked, not copied from another project's
grounding uncritically), not assumed from the charter or the decomposition.

`project.md` for this item is already unusually thorough — it restates most of the
decomposition's reasoning and cites many of the same anchors this file does. This
note exists to (a) independently re-verify the load-bearing citations rather than
trust them, (b) supply the primary evidence text for DT-2/DT-3/DT-8 that `project.md`
references but does not quote, and (c) state plainly what constrains this project and
what does not.

## Accepted decision atoms constraining this project

**None, directly.** All seventeen atoms under `.kb/decisions/` were read by title:

`0001` async port flavours · `0002` crate naming · `0003` opaque payloads · `0004`
edition and MSRV · `0005` rename to happenstance · `0006` bare name to the typed
layer · `0007` projection runner decodes · `0008` one derivation for both ports ·
`0009` error Send/Sync · `0010` the suite must prove itself · `0011` read laziness
and isolation · `0012` append shape and preconditions · `0013` position assignment
and visibility · `0014` event identity and recorded time · `0015` validated
identifiers and store limits · `0016` the wire format · `0029` MSRV raised to
1.97.1.

None governs documentation trees, CI/gate structure, or narrative-content
conventions. This project's binding constraints are therefore **conventions**
(`CLAUDE.md`, `standards/rust/README.md`'s precedence block, `docs/README.md`), not
ADRs, and this project does not need to write or amend an ADR to do its work —
nothing in scope here touches a `[FROZEN]` `spec/SPECIFICATION.md` clause or
contradicts an accepted decision. This matches the independent finding already on
record in `.bklg/docs-that-teach/checked-documentation-surface/_grounding.md`
("No tension with an Accepted ADR was found"), and is worth re-confirming rather
than inheriting, since a false negative here would be easy to miss.

One `.kb/governance/` atom is relevant though not a `decision`: **the immutability
rule for `.kb/decisions/`** (`.kb/decisions/README.md:9-13`) — "An accepted decision
is never edited... To correct one, write a new atom carrying `supersedes`." This is
what makes BR-12's "stage a playbook, don't hand-author into `.kb/`" the only live
option, together with `.kb/_intake/README.md`'s statement that nothing staged there
is held to `KbFrontmatter` (raw material only) and `.kb/playbooks/README.md`'s
explicit exclusion: *"A commitment... is a `decision` atom... filing it here strips
it of the immutability that makes it enforceable"* (`.kb/playbooks/README.md:29-31`).
That is the textual basis for `project.md`'s DR-11: the playbook atom records
*method*, and the binding `must` stays in the gate-read tree.

## The load-bearing prior art: `standards/rust/` is the template to imitate, not merely cite

Re-verified directly, independent of `checked-documentation-surface`'s grounding:

- **Precedence block**: `standards/rust/README.md:23-29` —
  ```
  ## Precedence

  > **SPECIFICATION clause > ADR > constitution atom > `CLAUDE.md` /
  > `CONTRIBUTING.md` summary > `references/evaluation/*`.**
  ```
  Confirmed at exactly those line numbers by direct grep
  (`## Precedence` at :23, the block quote opening at :25). **AC-002's literal text**
  ("the router states its rank relative to `standards/rust/README.md:23-29`") cites
  this correctly — do not let a later edit to the router's own file shift this
  without re-checking.
- **Router shape to imitate**: `standards/rust/README.md:1-21` — a one-paragraph
  statement of scope, "**Load one to three atoms, never the corpus**," a trigger
  table by numeric band (`00`–`01` prime directives, `10`–`13` types, ... `90`–`92`
  skeletons), each atom a separate file. This is the direct template for AC-001's
  router — a reader should be able to find one rule from a table, not read a
  concatenated corpus.
- **Rule-atom shape to imitate**: `standards/rust/00-prime-directives.md:1-16` — each
  atom opens with a `> **Load when:**` trigger line and a `> **See also:**`
  cross-reference line. Each numbered rule that follows (`RS-00-1` at `:19`, with
  its **Do** at `:30`, **Not** at `:54`, **Rejects** at `:78`) states **Why**, a
  compiling example under **Do**, and a named wrong implementation that could
  plausibly ship under **Not**/**Rejects**. This project's rules do not compile
  Rust — they govern prose — but the "Do / Rejects, with a named wrong
  implementation" shape is exactly what AC-008 asks for, and `standards/rust/`'s own
  discipline is the house precedent for it, not an import from outside the
  repository.
- **Pinned-by-path constants**: `xtask/src/lint_constitution.rs:55,58,61` —
  `const ATOM_DIR: &str = "standards/rust"`, `const ROUTER: &str =
  "standards/rust/README.md"`, `const HARNESS: &str = "xtask/src/constitution.rs"`
  — confirmed at those exact lines. `docs/README.md:25-28` states the general rule:
  *"Two of those are read by the gate rather than only by people... Moving either
  tree means editing `xtask/src/` in the same change — which is the point of pinning
  them by path rather than by convention."* This is BR-12's textual authority and
  the direct precedent this project's own architecture brief should cite for "a tree
  the gate reads is pinned by path."
- **No-orphan-page check, already solved once**: `xtask/src/lint_constitution.rs:423`
  — `fn check_harness(root: &Path, atoms: &[Atom], problems: &mut Vec<String>)` —
  confirmed to exist at that line. This is the reusable shape for a check that
  cross-verifies a directory listing against a second source of truth (here: pages
  against declared needs) in both directions.
- **Fence/opt-out discipline, a weaker precedent than this project needs**:
  `xtask/src/lint_constitution.rs:601` — `fn check_fences(atom: &Atom, ...)` —
  confirmed to exist at that line; it forbids an untagged fence and requires an
  `ignore` fence to carry a `<!-- ignore: <reason> -->` comment. Not directly reused
  by this project (that is HS-P0020's citation-resolution and fence-compiling
  territory per the non-goals below), but the same "a reason must be stated inline,
  where the check can read it" shape is the right model for how a page's declared
  need should be spelled — machine-readable in the same document a human reads,
  not in a sidecar file that can drift.

**Consequence for the architecture brief**: this project's new tree is not a
metaphorical sibling to `standards/rust/` — it should reuse the same mechanism
(pinned-path constants in a new `xtask/src/` module, a router with a trigger table,
one rule-atom per file) rather than inventing a different shape for the same job.
Any deviation should be named as a deliberate choice, not a default.

## DT-2, DT-3, DT-8 — primary evidence, quoted directly

`project.md` cites `_discovery/distillation/interaction-patterns.md` by tension
number without quoting; the actual text is the evidentiary base the design stage
argues from, so it is reproduced here rather than re-paraphrased a third time.

**DT-2 = interaction-patterns.md tension 3** (`:498-512`), *"Whether Diátaxis's
four-category taxonomy is adopted as literal structure, or only its 'one need per
page' discipline is kept"*:

> this project's three audiences don't sort cleanly into
> tutorial/how-to/reference/explanation, and Diátaxis's sharpest critic names this
> project's exact kind of subject — a dense, interrelated conceptual model, not a
> simple tool — as where the four-box split strains.

Three named options: adopt the four categories literally; adopt only the
one-need-per-page discipline without four named categories; design a taxonomy keyed
to the three personas instead of reader-intent quadrants. The tension text states
plainly this is *"a primary input to that decision, not a call made here"* — i.e.
the design stage owns the resolution, not this grounding pass.

The companion anti-pattern (`interaction-patterns.md:442-447`) supplies the
strongest quotable line for whichever option is rejected: *"Diátaxis's own
maintainer calls this 'horrible,' and its own site is cited by its users as failing
to follow its own structure... does forcing a page into a category bucket produce
an empty or near-empty bucket, or a page straining to be two things at once."*

**DT-3 = interaction-patterns.md tension 7** (`:568-584`), *"Findability/landing-page
structure: is it a need outside whatever page taxonomy gets chosen, and where does
it live"*:

> Diátaxis's taxonomy classifies content but is silent on how a reader gets routed
> to the right document in the first place — an index or landing page is neither a
> tutorial, how-to, reference, nor explanation in the strict sense. This project's
> own evaluator persona names exactly this gap ("good until the second question,
> and then nowhere to go").

Three named options: a dedicated structural need with its own page(s); fold routing
into whichever category each taxonomy treats as entry point; leave it entirely to
the two-surface split's crate-root pointer (interaction-patterns' tension 8, owned
by `reach-and-adapter-path`'s DT-10, not this project). Explicitly coupled to DT-2:
*"whichever page-need taxonomy tension 3 resolves to, a deliberate decision on
whether a landing/index page is a first-class category of its own."*

**DT-8 = interaction-patterns.md tension 2** (`:484-497`), *"Where the line sits
between 'aside' and 'load-bearing' for collapsible content"*:

> NN/g's own rule — never hide essential information behind a fold — collides
> directly with this project's own precedent of a constraint that drifted out of
> sync once only one of its two statements stayed visible.

Three named options: ban collapsible content for anything stating an invariant,
constraint or `MUST`, restricting it to exercises/asides only; allow it broadly and
rely on review discipline; forbid the pattern in this corpus entirely. Explicitly
states the non-answer this project must not give: *"'use good judgment' is the same
non-answer that let the code-layer invariant drift in the first place."*

The matching anti-pattern (`interaction-patterns.md:404-410`) supplies the checkable
test `project.md`'s DR-08 already adopts near-verbatim: *"if the collapsed section
were deleted, would the page still teach the constraint correctly? If not, it should
not be collapsed."*

The first anti-pattern in the list (`interaction-patterns.md:397-403`), *"A page
answering more than one named need,"* is DT-2/BR-04's own root evidence, citing both
Diátaxis and Mark Baker's "Every Page is Page One" independently: a page that
accumulates every discovered gap becomes *"an open-ended sink."* This is the
citation `project.md`'s "How this advances the initiative" section already draws on
— confirmed to exist at that anchor.

## The `.kb/playbooks/` shape this project's staged atom must match (AC-012)

Six existing playbook atoms were read for shape and frontmatter convention (all
under `.kb/playbooks/`, listed by `ls`): `anchoring-citations-in-a-long-lived-document.md`,
`landing-a-stricter-gate-without-a-red-baseline.md`, `one-decision-per-adr-title.md`,
`repairing-a-frozen-clause-without-amending-it.md`,
`testing-interleavings-with-cold-futures.md`, `verify-the-referent-and-report-coverage.md`.
`.kb/playbooks/README.md:7-19` states the shape required: a **method** with a stated
claim and the conditions under which it stops holding, a **procedure** with an order
that matters, or a **technique** arrived at by discarding named alternatives — and
explicitly: *"A playbook that lists only the approach that won reads as arbitrary."*
This project's staged atom (the discipline itself, summarized) should be written to
this bar, not as a summary of the rules — DT-2/DT-3/DT-8's rejected options, named
above, are exactly the material that satisfies README's "discarding cheaper
alternatives, with those alternatives named."

`.kb/_intake/README.md:7-8` confirms the mechanism: files dropped in
`.kb/_intake/*.md` are **not** held to `KbFrontmatter` at staging time — they are
"raw material." AC-012's requirement that the staged atom carry "valid KB
frontmatter" is therefore a **project-imposed stricter bar** than the ingest
mechanism requires (the ingest process would tolerate a looser draft and adjudicate
it), not a mechanical validation this project can lean on `redkiln validate --kb` to
enforce — `_intake/` is skipped by that validator by design
(`.kb/_intake/README.md:21-25`: *"`redkiln validate --kb` skips any directory whose
name begins with `_`"*). So the "valid frontmatter" bar in AC-012 must be
self-verified against `.kb/README.md`'s field table (`id`, `title`, `kind: playbook`,
`status`, `authority_tier: guideline`, `summary`, `depends_on`/`related`,
`source_paths`, `last_reviewed`) at authoring time, not proven by a green gate step.

## `.kb/maps/domain-map.md` — confirmed: documentation has no functional home yet

`.kb/maps/domain-map.md:35` states plainly: *"This is the map's first wave. One
domain exists so far"* — "Specification governance & conformance." No documentation
or narrative-content domain section exists. `.kb/maps/README.md:34-36` states the
mechanism by which one would be added: *"a new canonical concept or domain area gets
an entry on the domain map"* during `/redkiln:kb-ingest`'s Maps phase — i.e. this is
an ingest-time addition, owned by HS-P0025's closeout ingest, not something this
project edits directly (this project stages material into `.kb/_intake/`, per
AC-012, and does not touch `.kb/maps/` itself). Note this is a narrower claim than
`project.md`'s risk-table phrasing ("a new subject area is an appended section,
never an edit") — the README states *how a new domain gets added*, not an explicit
prohibition on editing an existing domain's content; the two are consistent in
effect but the latter is this project's own inference, not a quoted rule.

## The decorative-gate-step precedent (cited by DR-06/AC-008's reasoning)

`RUNBOOK.md:920-925` — the CI nightly-toolchain incident: *"nothing in `.github/`
installed nightly, so it printed `skipped` on all three runners while
`xtask/src/main.rs` asserted 'CI installs nightly, so the check is real there'...
A step that always skips is the decorative-rule failure applied to tooling, and it
had two documents vouching for it."* A second, distinct incident in the same
paragraph block, `RUNBOOK.md:931-935`: the `documentation` gate step *"had been
printing 'generated 3 warnings' and exiting 0 for as long as it had existed"*
because rustdoc does not read the ambient `RUSTFLAGS`. Both are confirmed at those
line numbers by direct read, not paraphrase. This is the textual basis this
project's own `CLAUDE.md`-derived corollary already states generally ("A rule that
no adapter can fail is decorative") applied specifically to a gate *step* rather
than a conformance *rule* — the same failure mode, one level up, and AC-007's
"observed to fail, then observed to recover" requirement exists specifically so this
project's own lint cannot join that list unnoticed.

## Clause-id stability (DR-09's textual basis)

`spec/SPECIFICATION.md:280` — confirmed: *"Clause IDs are stable and are never
renumbered."* This is the citation-survives-refactor guarantee DR-09 depends on
(a page's citation of a clause id is not invalidated by the sibling branch's
521-line divergence, since ids are names, not line references).

## Non-goals and the precedence-chain boundary, re-confirmed

`standards/rust/README.md:23-29`'s five-tier precedence block was read directly
(not just grepped for existence) and contains no reference to any documentation or
narrative-content tier — confirming there is nothing in the current text this
project's tree would need to avoid contradicting by omission. AC-002's bar ("a diff
over the branch shows that precedence block unedited") is therefore a check this
project can satisfy by never touching `standards/rust/README.md` at all, which is
the simplest way to discharge it: this project's new tree states its own rank
*inside* the existing chain — most plausibly slotting at or near "constitution atom"
tier, since it is a project convention of the same kind, not a `SPECIFICATION.md`
clause, an ADR, or `CLAUDE.md` prose — without editing the cited file.

## Anchors this project's briefs should cite directly

- `standards/rust/README.md:23-29` — the precedence block, AC-002's anchor
- `standards/rust/README.md:1-21` — the router/trigger-table shape to imitate
- `standards/rust/00-prime-directives.md:1-16,19,30,54,78` — the rule-atom shape
  (Do/Not/Rejects, named wrong implementation) to imitate for AC-008
- `xtask/src/lint_constitution.rs:55,58,61` — pinned-path constants, the template
  for this project's own `ATOM_DIR`/`ROUTER`-equivalent constants
- `xtask/src/lint_constitution.rs:423` (`check_harness`) — bidirectional
  page-vs-declared-need cross-check template
- `xtask/src/lint_constitution.rs:601` (`check_fences`) — the weaker existing
  opt-out precedent, useful as a contrast case
- `docs/README.md:25-28` — why a gate-read tree is pinned by path
- `.kb/decisions/README.md:9-13` — the immutability rule that makes staging (not
  hand-authoring) the only option for AC-012
- `.kb/playbooks/README.md:7-19,29-31` — what the staged atom must contain, and why
  a commitment does not belong in a playbook
- `.kb/_intake/README.md:7-8,21-25` — staged files are not `KbFrontmatter`-checked;
  `redkiln validate --kb` skips `_`-prefixed directories
- `.kb/maps/domain-map.md:35` and `.kb/maps/README.md:34-36` — no documentation
  domain exists yet; a new one is added at ingest time, not by this project directly
- `.bklg/docs-that-teach/_discovery/distillation/interaction-patterns.md:397-403`
  (anti-pattern 1), `:404-410` (anti-pattern 2, the DT-8 checkable test),
  `:484-497` (tension 2 = DT-8), `:498-512` (tension 3 = DT-2), `:568-584`
  (tension 7 = DT-3) — the primary evidence for the three tensions this project
  must resolve
- `RUNBOOK.md:920-925,931-935` — the decorative-gate-step precedent
- `spec/SPECIFICATION.md:280` — clause ids are stable and never renumbered

## Tensions / risks to flag in the briefs

1. **No Accepted decision atom constrains this project.** Confirmed independently
   by title-scan of all seventeen. The binding authority is `CLAUDE.md` +
   `standards/rust/README.md`'s precedence chain + `docs/README.md`'s pin-by-path
   convention, all sub-ADR tier. Nothing here should be escalated into a new ADR;
   doing so would itself be a scope violation of the initiative-level non-goal
   against extending the precedence chain.
2. **AC-012's "valid KB frontmatter" bar is not gate-checkable.** `_intake/` is
   deliberately exempt from `redkiln validate --kb`. This project's Definition of
   Done line "`redkiln validate --kb` green: nothing has been hand-authored into
   `.kb/`" only proves the negative (nothing landed in the checked tree); it does
   not prove the staged atom's frontmatter is actually valid. The ledger evidence
   for AC-012 needs a manual field-by-field check against `.kb/README.md`'s table,
   not a green CLI run standing in for it.
3. **DT-2/DT-3/DT-8's resolution should cite the interaction-patterns.md tension
   text directly in `_design.md`**, not merely name the tension number — the design
   stage's own bar (per `.kb/playbooks/README.md`'s "discarding cheaper
   alternatives, with those alternatives named") is exactly the three-option
   structure the research file already states; re-deriving it from scratch would
   discard free, already-cited work.
4. **The domain-map claim in `project.md`'s risk table is slightly stronger than
   what `.kb/maps/README.md` actually states.** Not a defect — the practical effect
   is the same — but the architecture/testing briefs should not cite it as a quoted
   rule; it is this project's own reasonable inference from the addition mechanism.
