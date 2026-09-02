---
title: Vocabulary and conventions — From Accurate to Teachable
kind: grounding/summary
item: HS-I0007
slug: docs-that-teach
---

# Vocabulary and conventions

Grounding pass for `docs-that-teach` (HS-I0007). This repo has **no
`.kb/03-reference/glossary/` and no taxonomy directory** — that path in the
task brief does not exist here (`find .kb -iname "*glossary*"` returns
nothing; `.kb/` is laid out as `decisions/ concepts/ governance/ maps/
open-questions/ playbooks/ product/ design/ reference/`, per
`.kb/README.md:1-90`). House vocabulary has to be read off the atom schema,
the maps, and the layer READMEs instead of a dedicated glossary. What follows
is that vocabulary, plus the code and ADR anchors this initiative will need
to cite rather than restate.

## House vocabulary this initiative must use, not invent

**Atom, kind, authority_tier.** A unit of KB knowledge is an *atom*: markdown
with `KbFrontmatter` validated by `redkiln validate --kb`. `kind` is one of
`narrative · concept · decision · reference · playbook · map · roadmap ·
open_question · governance` (`.kb/README.md:20`). `authority_tier` is a
free-string convention, not an enum, and this repo's own atoms currently use
`decision`, `guideline`, `note`, `product`, `design` (`.kb/README.md:31-36`,
`.kb/product/README.md:6-9`, `.kb/design/README.md:1-2`). Whatever this
initiative asks a documentation-standard atom to be — the open question
itself, in the intake brief, is *where a documentation standard lives if one
is written* — it inherits this vocabulary rather than inventing a new one.

**Clause, ADR, atom, `CLAUDE.md`/`CONTRIBUTING.md`, `references/evaluation/*`
— five tiers, one precedence chain, stated once.**
`standards/rust/README.md:23-29` states it exactly: *"SPECIFICATION clause >
ADR > constitution atom > CLAUDE.md/CONTRIBUTING.md summary >
references/evaluation/*."* The intake brief's own constraint — *"the
precedence chain in `standards/rust/README.md` is not extended by this
work"* (`_intake-brief.md:64`) — means any documentation-standard artefact
this initiative produces sits **below** that chain, citing a clause or an
atom rather than restating one. The seed sharpens the metaphor further: *"the
book slots below the constitution, not beside the specification"*
(`references/seeds/user-documentation.md:184`).

**Maturity marker.** `SPECIFICATION.md` clauses each carry one of `[FROZEN] ·
[PROVISIONAL] · [DEFERRED]`, or are demoted to non-normative prose (CLAUDE.md
"Open questions" section; corroborated by RUNBOOK.md's clause-count passages
at `RUNBOOK.md:538,580`). A `[FROZEN]` clause is changed only by a new ADR,
never an edit (CLAUDE.md). This is the vocabulary a guide must defer to when
it disagrees with the spec — the brief is explicit that *"where any page and
`spec/SPECIFICATION.md` disagree, the specification wins"*
(`_intake-brief.md:62-63`).

**Constitution atom / RS-\<band\>-\<n\> rule.** `standards/rust/` is "the Rust
constitution," 27 atoms numbered in bands (`standards/rust/README.md:1-22`),
each rule shaped **Why / Do / Not / Rejects / Evidence**
(`standards/rust/README.md:99-108`). The one atom whose subject is
documentation is `standards/rust/70-rustdoc-obligations.md`, five rules
(RS-70-1 through RS-70-5) covering `[lints] workspace = true`
(RS-70-1, `70-rustdoc-obligations.md:12`), feature-gated intra-doc links
(RS-70-2, `:93`), `doc-valid-idents` (RS-70-3, `:152`), the `docsrs`/`doc_cfg`
gate (RS-70-4, `:195`), and *"name the alternative that lost, in the doc
comment, once"* (RS-70-5, `:243`). The seed's own diagnosis — *"Returns the
head position. # Errors: Returns an error on failure." satisfies every lint
in the workspace* — is RS-70-5's own **Not** example, verbatim
(`70-rustdoc-obligations.md:283-300` vs. `user-documentation.md:58-59`). This
initiative should cite RS-70-5 by id rather than re-describing the failure
mode it already names.

**Gate, gate step.** `cargo xtask ci` is "the gate" (CLAUDE.md, "Commands").
A gate step "skips" when its probe fails to find the tool, and "a step that
always skips is the decorative-rule failure applied to tooling"
(`RUNBOOK.md:920-924`, an exact precedent for what proof artefact #1 in the
intake brief is asking for: a documentation gate that has only ever been
green is decorative — CLAUDE.md's own corollary, *"A rule that no adapter can
fail is decorative,"* is the same shape one layer down).

**Referent vs. reasoning.** `.kb/governance/rewrite-the-referent-never-the-reasoning.md`
gives the house test for whether an edit to a settled document is a permitted
repair or a forbidden reversal: *"ask whether the edit changes what the
document asserts, not whether it changes the document"*
(`rewrite-the-referent-never-the-reasoning.md:7-19`). Directly applicable if
this initiative ever needs to touch a doc comment already discharging a
`[FROZEN]` clause — the brief's constraint that such rewrites "leave them
discharged" (`_intake-brief.md:67-68`) is this same discrimination.

**Proof artefact.** Used identically across the brief, the seed and
`spec-trace`'s own history: an artefact that *"would not exist if the design
were wrong"* (`_intake-brief.md:68-69,114-135`; `user-documentation.md:188-191`).
Not "a test passes" — the standard is a deliberately-broken instance that
*fails*, the same proof the spec tracer was built to give
(CLAUDE.md's `spec-trace` description; `RUNBOOK.md:106-109` on the orphaned
rule it once was).

**Friction log.** The seed's term for the second proof artefact: a
non-author reader's dated record of *what they got stuck on, not whether
they liked it* (`_intake-brief.md:127-132`). The house precedent already
exists — `examples/outside-projection-adapter/`, built "from the rendered
documentation alone" (`user-documentation.md:212`, table row) — and is worth
citing as the shape this initiative's own friction log should take, not
re-derived from scratch.

**Persona / journey, and where they live.** `.kb/product/README.md:6-9`
fixes the vocabulary: a **persona** is a `concept` atom (`authority_tier:
product`); a **journey** is a `playbook` atom, same tier — "the moment-by-moment
path a persona takes through a task, written so you can tell whether a build
improved it." An unresearched persona is explicitly disallowed from that
layer: *"A persona nobody researched is a stock photo with a name"*
(`product/README.md:23-24`, quoted again in the seed at
`user-documentation.md:131-132`). `.kb/design/README.md` is the sibling layer
for **resolved interaction-pattern decisions** (`concept` atoms,
`authority_tier: design`) — pattern chosen, alternative rejected, failure
mode mitigated, fit conditions (`design/README.md:6-23`) — and is explicitly
**not** where a persona's journey belongs (`product/README.md:33-36`). If
this initiative's audience-model open question resolves to authoring
personas, they are `concept`/`product` atoms in `.kb/product/`, not a new
layer.

**Domain map, decision map, open-questions index — and how a new domain
gets added.** `.kb/maps/README.md:8-17` names exactly three map atoms plus a
future fourth (`dependency-map`); `.kb/maps/domain-map.md:144-150` says a new
subject area is a new `##` section appended, never an edit to an existing
one, with entries grouped by kind and cited by atom id. Two domains exist
today — "Specification governance & conformance" and "Contract ports,
conformance, and the ADR corpus" (`domain-map.md:37-142`) — and neither is
documentation. This initiative, if it lands KB atoms, adds a **third domain
section** rather than folding into either existing one; the intake brief's
own reading agrees (*"the domain map has two domains and neither is
documentation,"* `_intake-brief.md` quoting `user-documentation.md:127`).

## Candidate context anchors

**Constitution / gate.**
- `standards/rust/70-rustdoc-obligations.md` (RS-70-1..5) — the only
  constitution atom on documentation; RS-70-5 is the load-bearing citation
  for "prose that satisfies every lint and teaches nothing."
- `standards/rust/62-doctests-and-harnesses.md` — its companion on making an
  example prove something (named at `README.md`'s Start-here table,
  `standards/rust/README.md:56`, and by the seed's supporting-material table,
  `user-documentation.md:206`).
- `standards/rust/91-adapter-authoring-recipe.md` — "the best evidence of
  what this repository's documentation looks like when it is good"
  (`user-documentation.md:207`).
- `standards/rust/README.md:23-29` — the five-tier precedence chain a
  documentation standard must sit below.

**Specification / plan.**
- `spec/SPECIFICATION.md` — maturity-marker vocabulary at §1.3; clauses this
  initiative must preserve rather than rewrite: `:1196`, `:1285` (tag
  equality), `:1487`, `:1517`, `:1560` (per-store limits above
  `MIN_SUPPORTED_*`), `:4353` (`AppendCondition` is a claim about one store's
  log), and `:1079`, `:3757`, `:4109`, `:4146`, `:5602` (all listed in
  `_intake-brief.md:149-152` as the starting, not final, set).
- `RUNBOOK.md:4448-4484` — Phase 12 ("Publish `0.2.0`"), the phase whose only
  documentation criterion is docs.rs rendering, not teaching; `:920-924` and
  `:1057-1062` — the decorative-gate-step precedent (nightly docs.rs step
  silently skipped, "generated 3 warnings" exiting 0) this initiative's proof
  artefact #1 must not repeat.
- `docs/README.md` — the 36-line signpost already reserving this tree for
  "someone *using* the library," and its admission test (last paragraph).

**Code.**
- `crates/happenstance/src/lib.rs:24-62` (on
  `initiative/from-contract-to-published-library`, not `main` — see Tensions)
  — the opening example calling `Tags::empty()` twice, the seed's sharpest
  single piece of evidence.
- `crates/happenstance-core/src/store.rs` — the `E0034` /
  `TraitVariantBlanketType` gap the seed names as documented everywhere
  except the file a reader is looking at (`user-documentation.md:92-98`).
- `examples/course-subscriptions/` (`main.rs:1-19` for the module doc,
  `Cargo.toml:8` for `publish = false`) — the one worked example, unpublished.
- `examples/outside-projection-adapter/` — the existing comprehension-instrument
  precedent, on `initiative/from-contract-to-published-library`, not `main`.

**Decisions.** `.kb/decisions/0006-bare-name-to-the-typed-layer.md` and
`.kb/decisions/0007-projection-runner-decodes.md` govern which crate
(`happenstance-core` vs. `happenstance`) a piece of teaching is actually
describing — load-bearing if this initiative's chapters explain the
contract/typed-layer split. `.kb/governance/rewrite-the-referent-never-the-reasoning.md`
governs any touch to a doc comment already discharging a `[FROZEN]` clause.

## Alignments

- The vision's four asks — compiled prose, a non-author reader, a named
  page-need, a rendered result met where a Rust developer looks — match this
  repo's existing standard of evidence exactly: `standards/rust/01-standard-of-evidence.md`-style
  reasoning (compiled example over prose) is the same discipline CLAUDE.md
  states for the whole workspace ("the compiled example beats the prose,"
  `standards/rust/README.md:27-28`).
- "Which need a page answers is a decision with a name" mirrors the existing
  layer-README discipline throughout `.kb/` — every layer README states
  "what belongs here" and "what does not" as a matched pair
  (`.kb/product/README.md`, `.kb/design/README.md`,
  `.kb/open-questions/README.md`), so a documentation taxonomy that follows
  the same shape would read as native rather than imported.
- The scope exclusion against `publication-and-positioning` (HS-P0016) is
  consistent with how this KB already separates *product* claims (what the
  surface says) from *design* decisions (what a pattern is for) — two
  layers, two authority tiers, deliberately not merged
  (`.kb/product/README.md` vs. `.kb/design/README.md`).

## Tensions

- **HS-P0016 (`publication-and-positioning`) and HS-S0131
  (`persona-and-journey-intake-staging`, under HS-P0019) do not exist as
  files anywhere in this worktree.** `.bklg/` here contains exactly two
  items — `docs-that-teach` and `support`
  (`find .bklg -maxdepth 2 -type d` → `.bklg/docs-that-teach`,
  `.bklg/support`); `.bklg/_archive/` does not exist; a repo-wide search for
  `HS-P0016` and `HS-S0131` matches only the intake brief itself and the
  seed. Both are asserted with specific claims — a `_design.md` "signed off
  with no conditions on 2026-08-12," a "fifty-one-frame mock," a story
  "specced... at `stage: plan`" — that cannot be verified against this tree.
  Distillation should treat the dependency on HS-S0131's persona staging, and
  the scope boundary against HS-P0016, as **asserted but unverifiable here**
  rather than as grounded fact, and should say so rather than silently
  trusting the brief's description.
- Several of the seed's strongest citations (`crates/happenstance/src/lib.rs:24-62`,
  `examples/outside-projection-adapter/`) are explicitly on
  `initiative/from-contract-to-published-library`, not `main`
  (`user-documentation.md:66-68,212`). This worktree's branch is
  `initiative/docs-that-teach`, branched from `main` at commit `31174d5`; a
  `git log` shows no merge of the published-library initiative onto this
  branch's ancestry. Whether those files are actually present here needs
  checking by whichever stage next reads them — this pass did not open them,
  since the seed itself flags them as living on a different branch and this
  is a vocabulary/anchors pass, not a code audit.
- No `.kb/reference/` atom or playbook currently addresses documentation
  practice at all — `.kb/reference/README.md`'s four atoms are all about
  specification reconciliation, port compilation, position visibility and
  wire-format measurement (`ls .kb/reference`), confirming the seed's own
  claim that *"across all twenty-seven constitution atoms there is no rule
  about narrative structure, audience, worked-example design or diagrams"*
  (`user-documentation.md:236-238`) extends to the rest of `.kb/` as well —
  there is no prior art inside this KB to reuse, only the shape of other
  layers to imitate.

## Related initiatives

- `.bklg/docs-that-teach/initiative.md` — this initiative's own charter
  (not yet authored past intake at time of this pass).
- `.bklg/support/initiative.md` — the only other live initiative in this
  worktree; not inspected in depth here since it did not surface in any
  documentation-relevant search, but worth a glance before distillation
  finalizes scope, on the chance it owns adjacent backlog grooming.
