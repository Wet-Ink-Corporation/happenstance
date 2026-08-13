---
item: HS-S0093
stage: spec
created: 2026-08-12T13:47:31.155Z
updated: 2026-08-12T13:47:31.155Z
template_sig: 87bbf1d0
rendered_sig: c4518b5e
---

# Spec — The compliance claim ships with its evidence, and the gaps promise is findable

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` — **AC-08** (`:330-332`, the compliance claim is checkable rather than trusted), **AC-09** (`:333-334`, the positions-and-gaps promise is readable without reading source), **BR-14** (`:297`), DT-4 (`:423`); the risk register's *"passed against N adapters is a snapshot"* row (`:434`) |
| Project | `.bklg/from-contract-to-published-library/publication-and-positioning/project.md` — **AC-009** (`:258-261`), **AC-010** (`:262-265`), DR-9 (`:191-197`), DR-10 (`:198-201`); the Dependencies section (`:315-329`) that says *why* four adapter projects block this one |
| This spec | `.bklg/from-contract-to-published-library/publication-and-positioning/compliance-claim-and-gaps-promise/spec.md` |
| **Design (binding)** | `.bklg/from-contract-to-published-library/publication-and-positioning/_design.md` — DT-4's resolution `:247-282`, the compliance block's composition slot R6 `:362` and the Guarantees slot R9 `:365`, the per-crate deltas `:380-389`, transience `:424-428`, density `:469-471`, hierarchy `:511`, `:514`, `:520-522`, the packaging/link rules `:543-559`, the `Error` state's two held anchors `:625`, anti-patterns AP-10 `:665-666`, AP-13 `:670-671`, AP-15 `:674-675`, the doctest `:688-711`, sign-off `:772-783` |
| Key briefs | `.bklg/from-contract-to-published-library/publication-and-positioning/_decomposition.md` — **UX brief**: U2 and U3 (`:83-84`, `:91-96`), IQ-1 (`:232-239`), IQ-2 (`:241-247`), IQ-4 (`:272-288`), IQ-5 (`:289-296`), IQ-7 (`:305-311`), **AC-UX-002** (`:317-322`), **AC-UX-003** (`:323-329`), AC-UX-011 (`:367-370`); **testing brief**: the AC-009 row (`:472`) and the AC-010 row (`:473`), which fix the tier and name the wrong implementation each must reject |
| Grounding | `.bklg/from-contract-to-published-library/publication-and-positioning/_grounding.md`:27-31 (ADR-0010 is the evidentiary backbone the claim points at), `:139-147` (**no existing "DCB-compliant" string anywhere in `README.md` or any crate README** — this is new copy, not an edit) |
| Story map row | `.bklg/from-contract-to-published-library/publication-and-positioning/_storymap.md`:65; why `published-surface-copy` is one slice at `:87-90`; merge position at `:168` |
| Discover stage | `.bklg/from-contract-to-published-library/publication-and-positioning/compliance-claim-and-gaps-promise/discover.md` — the signal ledger, the **two questions it deferred to this spec** (which implementations get named; the exact non-ownership wording) and the named wrong implementation |
| Roadmap pointer | `RUNBOOK.md`:163 — phase 12's row and its proof artefact. This story writes no runbook prose; `falsifier-ledger-repair` owns `:622-635` and `crate-set-decision` owns `:4448-4460` |

> **A note on `discover.md`'s line citations.** It cites DT-4 at `_design.md`:228-261 and the density
> row at `:448`. Those were correct when it was written and are now short by the 22 lines of the
> owner's amendment rider inserted at `:222-243`. Every citation in *this* spec is against the
> current file. Where the two disagree, this spec's numbers are the live ones.

## One-line PR slice

Put the "DCB-compliant" claim on the packaged READMEs in `README.md`:227-234's claim-with-evidence
form — the implementations the suite was actually run against, the date it was run, and a reachable
link — and state, at DT-4's chosen site and within one hop of the landing entry, what the library
promises about sequence positions and gaps in plain language, without implying ownership of
`read_from_a_gap_position` that `.kb/open-questions/es-38-and-gap-read-rules-are-unowned.md` records
as owned by nobody.

## Executive summary

**What this PR lands.** Two blocks of prose on the three **packaged** crate READMEs, and nothing
else: a compliance block (`##` heading, R6) on all three, and one Guarantees bullet plus its single
nested line (R9) on `happenstance` and `happenstance-core`. Both are new copy. `_grounding.md`:139-143
verified there is **no existing "DCB-compliant" string anywhere in the tree** to redirect, and
`crates/happenstance/README.md`:41-49 / `crates/happenstance-core/README.md`:43-54 currently carry
three and four Guarantees bullets respectively with nothing about positions at all.

**The pointer.** The *what* is entirely decided upstream and is not re-opened here: DT-4's winner,
its site, its register, its length and its single-link rule are `_design.md`:247-282, signed off with
no conditions on 2026-08-12; the claim-with-evidence *form* is `README.md`:227-234; the block's slot,
budget and rank are `_design.md`:362, `:469` and `:511`. This story writes the sentences those
decisions describe.

**The delta is that two of the four parts of each block cannot be written from the design at all** —
they have to be resolved against the tree at the publish commit, and both are exactly where the
naive version goes wrong.

1. **Which implementations the claim names is not knowable from any planning artifact.** It is
   naturally read off `project.md`:315-329's Dependencies list — four sibling adapter projects — and
   that list says *which projects block this release*, not *which adapters have run the suite and
   passed*. Conflating the two produces a claim broader than what was checked, which is precisely
   the wrong implementation the testing brief names for AC-009 (`_decomposition.md`:472). This spec
   fixes the derivation rule and makes the empty case an outcome the copy must be able to state
   rather than a case that quietly gets padded.
2. **The gaps promise must stop one clause short of what a reader would expect it to say.** VT-11
   (positions unique, strictly increasing, gaps permitted) is `[FROZEN]` and safe to promise. The
   *rule* that would check a read starting at an unoccupied position —
   `read_from_a_gap_position` — is named by ADR-0011 and ADR-0013 and owned by neither
   (`.kb/open-questions/es-38-and-gap-read-rules-are-unowned.md`). A sentence that reads as claiming
   it is the wrong implementation the testing brief names for AC-010 (`_decomposition.md`:473).

**One finding this spec adds that no upstream artifact carries.** `crates/happenstance-testkit/README.md`:16-17
says **"No adapter has run this suite"** — a sentence that will be false on the tree that ships and
that sits roughly ten lines above where this story puts a block naming the adapters that did. It is
a *fifth* stale string; AC-UX-008's list (`_decomposition.md`:351-355) names four and this is not
one of them, so nothing else in the plan is obliged to fix it. It is in a file this story edits, and
leaving it produces a page that contradicts itself inside one screenful. It is claimed here.

## Context pack

Everything in this section is a decision, already made, that this story must honor. It is
self-sufficient: an implementer can write both blocks from this section alone. The deeper artifacts
stay behind the anchors table.

**1. The claim's form is fixed, and it is not a badge.** The compliance block is *one visual unit*
with four mandatory parts — claim, implementations, date, link — under its own `##` heading, sited at
R6: below the fold, above the quick start, and it is **the first heading a scrolling reader meets**
(`_design.md`:362). The order is deliberate and stated: the evaluator's sequence is *is it real → do
I try it*, not the reverse (`:369-378`). The block is **the same text on all three packaged
READMEs**, because it is the same claim (`:389`). Budget: **≤ 6 rendered lines** — claim 1,
implementations 1–2, date 1, link 1 — and the reason the ceiling is where it is: *"longer and it
stops reading as a single checkable claim"* (`:469`). Its rank is **primary for the screen it lands
in**, and its four parts are all mandatory (`:511`); at most one primary per screenful (`:520-522`).
**AP-10** is the failure: a "DCB-compliant" claim on a screen with no date and no named
implementations *in the same block* (`:665-666`).

**2. The claim must open by conceding what it cannot assert.** The DCB specification defines **no
conformance process**, so the label is industry-wide self-asserted — the evaluator has nothing
external to check it against (`../_discovery/distillation/personas-and-journeys.md`:272-275, carried
into the UX brief at `_decomposition.md`:58-61). What *is* checkable is what the suite was run
against, and why a pass from this particular suite means something: ADR-0010's discipline — every
rule carries a wrong implementation it rejects, and a skip is reported rather than silent
(`.kb/decisions/0010-the-suite-must-prove-itself.md`; `_grounding.md`:27-31). A block that asserts
the adjective and omits the concession is a claim, not evidence.

**3. Which implementations may be named, and how the set is derived.** Exactly those adapters that
had **run the conformance suite and passed** as of the stated date, taken from each sibling project's
own closed report — never from `project.md`:315-329's Dependencies list, which records *blocking*,
not *passing*. `CLAUDE.md`'s bar is the discriminator and it is quotable: *an adapter that compiles
but has not run the suite is not an adapter*. The UX brief's state vocabulary makes the same cut,
naming the live instance: `cargo package -p happenstance-sqlite --list` exits 0 today and lists seven
files (`_decomposition.md`:97-111, citing `xtask/src/package.rs:20-22`). The reference
`MemoryEventStore` is not an adapter result and does not pad the list; if it is mentioned at all it
is labelled as the reference store, not as an implementation the claim rests on.

**4. The empty and the thin cases are outcomes, not blockers to route around.** If, at the publish
commit, no sibling adapter has a closed passing report, the honest block says so — the suite exists,
the rules discriminate, and no third-party adapter has run it yet — and it still carries the date and
the link. Padding the list to make the block read better is the exact failure this story exists
against. This is the same "state the absence with its reason" rule the design applies everywhere
(`_design.md`:623, RS-40-5 at `standards/rust/40-public-surface-and-evolution.md`:212).

**5. The claim is worded so a later dated claim can supersede it.** Publication is irreversible: a
yank removes a version from the resolver and leaves the rendered page exactly as it is
(IQ-4, `_decomposition.md`:272-288). So every part is stated as a snapshot — the fact, the
implementations, the date — which is the form `README.md`:227-234 already uses for the rename
(*"was called eventum until 2026-08-05 … see ADR-0005"*): the fact plainly, the date attached, the
authority cited by path rather than asserted. The initiative's risk register is the reason
(`initiative.md`:434).

**6. The evidence link is one link, absolute, and it lands on a specific anchor.** IQ-5: no claim on
a published surface may have *"run our CI"* or *"check out the repo"* as its only evidence — the
evaluator cannot run the suite (`_decomposition.md`:289-296). IQ-1: **≤ 1 hop to the evidence, landing
on a specific anchor — never a repository root, never "see the specification"** (`:232-239`). And
every link on a packaged surface is an **absolute** URL, because a relative link resolves against
crates.io and 404s (`_design.md`:555-559, AP-6 at `:656-657`).

**7. DT-4's resolution, in the shape that is part of the decision.** Option (c) won — *both*, with
the promise **stated in full and in the caller's register** inside the Guarantees block of
`crates/happenstance/README.md`:41-49 and `crates/happenstance-core/README.md`:43-54, as **one bullet
of ≤ 3 rendered lines plus one nested line**, carrying **exactly one** link into `spec/SPECIFICATION.md`
(`_design.md`:251-256, `:471`). Option (b), specification-only, lost on IQ-1's 0-hop budget: a
~5,000-line specification is a context jump the one-sitting reader does not return from. Option (a),
landing-page-only, lost because *the citation is the difference* — a promise with no citation is
exactly what two DCB-labelled stores already disagreeing in public can each write (`:258-265`).
Collapsing "in the caller's register, ≤ 3 lines, one clause link" into "we mention gaps somewhere" is
losing the decision while keeping the winner.

**8. What the promise says, and the register it says it in.** The README states the **consequence for
the caller**; the specification states the clause; **neither is a paraphrase of the other**, and that
is the structural mitigation against two copies drifting (`_design.md`:268-272). The caller-facing
content is fixed: positions are **unique and strictly increasing within one store**, **gaps are
permitted**, a position **is not a count**, and **code that assumes the next event is at `n + 1` is
wrong against a conformant store**. The clause behind it is **VT-11** (`spec/SPECIFICATION.md`:1020-1027,
`[FROZEN]`). The resume idiom — `checkpoint.next()`, sound over gaps because `ReadOptions::from` is
an inclusive lower bound rather than a seek — is **VT-13**'s (`spec/SPECIFICATION.md`:1074-1085,
`[FROZEN]`), and the behaviour underneath it is **ES-9** — *`from` names a position, not an index*,
so a read at an unoccupied `p` yields the next matching event rather than erroring
(`spec/SPECIFICATION.md`:2748-2760, `[FROZEN]`). **All three clauses this story cites are frozen**,
which is what makes the promise safe to make; what is *not* frozen is that anything checks the last
of them, which is item 9.

**9. The nested line is the absence, with its reason, attached to the thing it qualifies.** The copy
must **not** imply ownership of `read_from_a_gap_position`. The open question records what is
actually true: it is ES-9's owed rule; ADR-0013 says outright that it is *"named by two documents and
owned by neither"*, and ADR-0011 listed ES-9 among the clauses it *"confirms unchanged"* without
assigning the rule an owner — *"two accepted decisions have now looked directly at this rule and
neither took it"* (`.kb/open-questions/es-38-and-gap-read-rules-are-unowned.md`). The nested line
says so and links the open question, and it is nested **because subordination is the point**: it must
not be readable as the promise (`_design.md`:428, `:471`). This is IQ-2 and RS-40-5 applied to prose
— **no surface may state an absence without a reason** (`_design.md`:276-280). **AP-13** is the
failure (`:670-671`).

**10. The open question stays open.** Linking it is not resolving it. The project's risk register
names *"an open question resolved in passing by the release rather than by decision"* as medium/high,
and the open-questions index requires a question be **resolved, never deleted** (`project.md`:351).
This story writes no `.kb/` atom and changes no atom's `status`.

**11. Two anchors are held and must survive.** `README.md`:9 → `#licence` and `README.md`:16 →
`#status` are live inbound intra-document links; at the publish commit either the anchor survives or
every inbound reference is updated in the same change (IQ-3, `_decomposition.md`:260-271;
`_design.md`:625). This story adds headings and does not rename any, so the obligation is discharged
by *not* breaking it — which is checkable, and is checked mechanically rather than read
(AC-UX-011, `_decomposition.md`:367-370).

**12. The clause link's anchor is guarded by a different instrument than the clause ID, and the
difference matters.** `cargo xtask spec-trace` guards clause **IDs** and `spec/SPECIFICATION.md`:210-211
retains even a demoted ID *"so that citations resolve"* — which is why the design says the link goes
into a clause ID rather than a heading (`_design.md`:273-275). But a GitHub/`docs.rs` anchor is slugged
from the **whole heading text**, so `#vt-11--positions-are-unique-…-and-may-have-gaps` breaks if the
*descriptive half* of the heading is reworded even though the ID survives and `spec-trace` stays
green. The gap is real and is named here rather than discovered at publish: the ID's stability is
`spec-trace`'s, the **slug's** stability is the slice's mechanical link check (AC-UX-011), and this
story's link must be registered with the latter.

**13. The Guarantees block is a shared, nearly-full budget, and this story spends one bullet of it.**
**≤ 7 bullets, each ≤ 3 rendered lines, exactly 1 link per bullet**, and **no nested bullets except
the DT-4 non-ownership line** (`_design.md`:470, `:514`). `crates/happenstance/README.md`:41-49 holds
three today; this story adds one; slice-mate `guarantees-and-docs-rs-presentation` adds three (MSRV
promise, minor-bump-is-breaking, `wasm32` self-identification). 3 + 1 + 3 = **7, exactly at the
ceiling** — which `_design.md`:595-597 already states as the honest cost. A later release wanting an
eighth demotes something per `## Density budget`. The corollary the implementer must not miss: the
nested line is the **only** permitted nested bullet in the block, and it carries the *second* link
(into the open question) — the "exactly 1 link per bullet" rule binds top-level bullets, and the
nested line's own link is what makes the absence-with-a-reason reachable at all.

**14. The first screen is full, and this story is not on it.** The owner signed off accepting that
the five first-screen regions measure ≈343 px against a 340 px budget and that the first screen is
**full at 0.2.0** (`_design.md`:772-783). Both of this story's blocks are **revealed on scroll** —
0 hops, below the fold (`:424`, `:427-428`) — so neither competes for that budget, and neither may be
promoted onto the first screen to make it more prominent. **AP-15**: more than one primary-ranked
element in a single screenful (`:674-675`).

**15. The persona slice this realizes.** Backbone activity **A3 — meet the crate on first contact**,
serving **U2** (*"show me what I would be trusting, and let me check it myself"*) and **U3**
(*"tell me what happens when a read is independent of the write that produced it"*)
(`_storymap.md`:42, `:45-50`; `_decomposition.md`:83-84). The reader is the evaluator: one sitting,
time-boxed, **cannot run the suite**, and has nothing external to check "DCB-compliant" against
(`../_discovery/distillation/personas-and-journeys.md`:249-313, `:262-266`, `:333-338`). **U3 is the
cross-persona beat — all four personas share it and none of them currently has anywhere to look**
(`_decomposition.md`:91-96), which is the whole reason BR-14 exists: two DCB-labelled stores already
disagree in public on gaps-permitted versus gapless, and a reader who never finds the statement
depends on the wrong one by accident.

## Integration contract

| Field | Value |
| --- | --- |
| **Archetype** | `capability` — a user-observable slice: an evaluator reading a published registry page meets both blocks and can act on them without running anything |
| **Slice / milestone** | `published-surface-copy`. Slice-mates, implemented in one context and mounted as one integrated surface: **`landing-copy-and-status-truth`**, **`guarantees-and-docs-rs-presentation`**. The slice is one slice because *it is one page*: a reader does not experience the compliance claim, the gaps promise and the Guarantees slot as three deliveries, and IQ-1's *0 hops to a claim, ≤ 1 hop to its evidence* budget can only be held by whoever sees the whole surface at once (`_storymap.md`:87-90) |
| **Mount point** | **`crates/happenstance/README.md`** — the packaged render source for surface `crates-io-happenstance` (`_design.md`:114-120). It is a real render path, not a formality: `readme = "README.md"` (`crates/happenstance/Cargo.toml`:12) is what puts it inside the `.crate`, `xtask/src/package.rs`:88-94's `REQUIRED_FILES` asserts its containment, and `crates/happenstance/src/lib.rs`:10's `#![cfg_attr(doctest, doc = include_str!("../README.md"))]` compiles it. Copy written only at the repo root is invisible to a registry reader, because `include_str!` cannot reach outside a package (`README.md`:131-137, `_design.md`:543-553). The same mount, one crate over: `crates/happenstance-core/README.md` and `crates/happenstance-testkit/README.md` |
| **Wires into** | `crates/happenstance/README.md`:41-49 and `crates/happenstance-core/README.md`:43-54 — the existing **Guarantees** lists DT-4's bullet joins, shared with `guarantees-and-docs-rs-presentation` inside the 7-bullet ceiling; `crates/happenstance-testkit/README.md`:9-22 — the status callout whose *"No adapter has run this suite"* sentence this story reconciles; `spec/SPECIFICATION.md`:1020-1027 (VT-11) and `:1078-1085` / `:2748` (ES-9) as the clause the single link cites and `cargo xtask spec-trace` guards; `.kb/decisions/0010-the-suite-must-prove-itself.md` as the evidentiary backbone the claim rests on; `.kb/open-questions/es-38-and-gap-read-rules-are-unowned.md` as the nested line's link target, which stays **open**; the slice's mechanical link/anchor check (AC-UX-011, `_decomposition.md`:367-370), owned by `landing-copy-and-status-truth`, which this story registers its two new links and one new anchor with; `.redkiln/config.yaml`'s story-grain `cargo xtask affected --base main` |
| **Renders surfaces** | **`crates-io-happenstance`** (primary, `_design.md`:114-120), **`crates-io-happenstance-core`** (`:122-128`), **`crates-io-happenstance-testkit`** (`:130-136`) — regions **R6** (compliance block, all three) and **R9** (Guarantees, `happenstance` and `happenstance-core` only; the testkit README has no Guarantees section and does not gain one). Not `github-landing`: the root README's four edits are `landing-copy-and-status-truth`'s (`_design.md`:399-404). Indirectly: `docs-rs-happenstance`, because `crates/happenstance/src/lib.rs`:10 compiles this README's fences as doctests — the surface is unchanged but the build must stay green |
| **Conformance rule(s)** | **none, and it is not adapter-observable.** No rule in `crates/happenstance-testkit/src/suite.rs` can observe README prose, and adding one would be decorative. The observing instruments are the testing brief's assigned tiers: **human observation on the rendered page** for AC-009 (`_decomposition.md`:472), **human observation plus AC-UX-011's mechanical link check** for AC-010 (`:473`). Stated explicitly because a story that names no rule must say which of the two it is |
| **Clause(s)** | **none amended, and none may be.** VT-11 and ES-9 are both `[FROZEN]`; this story **cites** them and changes no clause text, so AC-015 and DR-15 hold and the diff is expected to prove it (`project.md`:281-283, `:217-219`). If writing the caller-facing sentence turns out to require a clause to say something it does not, that is a blocker and a new decision atom, not an edit — `project.md`'s *Out of scope* reserves it for an upstream project and a re-plan (`:124-126`) |
| **Advances DoD scenario** | Initiative **DoD 10** — *"the published crate looks finished … the registry page carries licence, description and README as rendered, checked by looking at them"* (`initiative.md`:387-389) — by making two of the page's four load-bearing blocks exist and be true. It is the sole story for initiative **AC-08** and **AC-09** (`:330-334`) and for project **AC-009** and **AC-010**, which `_storymap.md`:132-133 records as unshared. At project grain it advances DoD 1 (both ACs observed on the published tree), DoD 3 (the compliance block is a dated artefact rather than a remembered observation) and DoD 6 |

## PR boundary

```
crates/happenstance/README.md
crates/happenstance-core/README.md
crates/happenstance-testkit/README.md
.bklg/from-contract-to-published-library/publication-and-positioning/compliance-claim-and-gaps-promise/**
```

**In this PR**

- The **compliance block** on all three packaged READMEs — same text on each — carrying claim,
  implementations, date and exactly one absolute evidence link, sited at R6 above the quick start.
- The **positions-and-gaps bullet plus its single nested non-ownership line** in the Guarantees block
  of `crates/happenstance/README.md` and `crates/happenstance-core/README.md`.
- The **reconciliation of `crates/happenstance-testkit/README.md`:16-17**'s *"No adapter has run this
  suite"* sentence, so the callout and the block ten lines below it do not contradict each other.
- The committed derivation record under this story's folder: which sibling reports were read, which
  implementations the set resolved to, the date, which rung of the evidence-link ladder was taken and
  why, and the two new links plus one new anchor handed to the slice's mechanical check — plus this
  story's `spec.md` and `_ledger.md`.
- The implementer **may** touch the mount/wiring files named in the Integration contract to mount
  this slice — that is not scope drift. Here that means the three READMEs themselves; no manifest and
  no source file needs to change for the mount to exist, because `readme = "README.md"` and
  `crates/happenstance/src/lib.rs`:10 already wire them.

**Explicitly not in this PR**

- **Any edit to `_design.md`.** Signed off with no conditions on 2026-08-12 (`:772-783`). An
  implementing story that edits the design has changed the thing it was measured against.
- **The repo-root `README.md`.** Its four edits — the callout, the status table, the Quick start
  version copy and the Prior art date — are `landing-copy-and-status-truth`'s (`_design.md`:399-404).
  The compliance block does **not** get a root copy: R6 is specified for the three packaged surfaces
  (`:389`), and a second copy is a second thing to keep true on a page that can never be edited.
- **The maturity census sentence (DT-5), the lead claim (DT-1) and the peer statement (DT-6)** —
  `landing-copy-and-status-truth`'s, even though the census lands on the same READMEs. **The four
  maturity counts do not appear in either of this story's blocks**; AP-3 forbids two counts on one
  surface (`_design.md`:650-651) and the numbers live in exactly one sentence per page.
- **The MSRV promise, the minor-bump-is-breaking line and the `wasm32` self-identification line** —
  `guarantees-and-docs-rs-presentation`'s, in the same Guarantees block. This story writes one bullet
  of the seven and leaves the other three unwritten rather than sketching them.
- **The quick-start fence and its doctest** — `guarantees-and-docs-rs-presentation`'s
  (`_design.md`:532, `:681-711`). Note the seam and do not silently cross it: the fence's one comment
  is *DT-4's promise stated where a copier will see it* (`:702`, `:709-711`), so its **content** is
  this story's and its **file location** is theirs. It is a one-line reduction of the Guarantees
  bullet, never a paraphrase — same divergence mitigation as DT-4's own (`:268-272`). Agree the exact
  string across the slice; do not write two.
- **`crates/happenstance/Cargo.toml`**, including the missing `[package.metadata.docs.rs]` block and
  the `description` field — both `guarantees-and-docs-rs-presentation`'s (`_design.md`:531, `:464`).
- **Any `.kb/` write.** No decision atom, no open-question edit, no map row. `first-contact-design-resolutions`
  owns the atom that records DT-4; this story consumes it. `kb-open-question-es-38-and-gap-read-unowned-001`
  is linked and stays open.
- **Any `spec/SPECIFICATION.md` edit**, and any `RUNBOOK.md` edit.
- **Building the mechanical link checker.** AC-UX-011's instrument is `landing-copy-and-status-truth`'s
  (`_storymap.md`:64); this story registers its links with it and fails if it is absent rather than
  building a second one.
- **The rendered-page read.** Reading the *published* crates.io page against the accessibility floor
  is `rendered-page-preflight`'s (`_storymap.md`:67), which blocks on this story landing.

**Merge DoD (one line).** All three packaged READMEs carry the same ≤ 6-line compliance block with a
date, a named set derived from closed sibling reports rather than from the Dependencies list, and one
absolute link landing on a specific anchor; `happenstance` and `happenstance-core` carry DT-4's
≤ 3-line promise with its one clause link and its one nested non-ownership line; the testkit callout
no longer contradicts the block; the Guarantees list is within budget and the doctest still compiles;
and the diff touches no `.kb/` atom, no clause, no manifest, no root README and no `_design.md`.

## Behavior and interfaces

**There is no Rust interface change.** `_design.md`'s `## Items` block (`:49-79`) carries nothing this
story owns, and `project.md`'s *Out of scope* reserves any API surface change for an upstream project
and a re-plan (`:124-126`). The interfaces here are the **rendered packaged surface** — Markdown
rendered by crates.io, whose theme, type scale and link resolution we do not own — and the
**packaging boundary**, which behaves like coherence: a path reaching outside the package does not
exist inside the `.crate` (`_design.md`:543-553).

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **B1 — the compliance block exists on all three packaged READMEs, as one visual unit** | A `##`-headed block at R6 — below the fold, **above** the quick start, the first heading a scrolling reader meets — carrying four mandatory parts: the claim, the implementations, the date, one link. **≤ 6 rendered lines** (claim 1, implementations 1–2, date 1, link 1). Same text on all three, because it is the same claim. Primary rank for the screen it lands in, and the only primary in it | `_design.md`:362, `:369-378`, `:389`, `:469`, `:511`, `:520-522`; AP-10 `:665-666`, AP-15 `:674-675`; `_decomposition.md`:317-322 |
| **B2 — the claim concedes the label is self-asserted, then states what is checkable** | The DCB specification defines no conformance process, so "DCB-compliant" is self-asserted everywhere it appears — including here. What is checkable is which implementations ran `happenstance-testkit`'s rules and passed, and why a pass from this suite discriminates: every rule carries a wrong implementation it rejects and a skip is reported, never silent. A block that asserts the adjective and omits the concession is a claim, not evidence | `../_discovery/distillation/personas-and-journeys.md`:272-275; `_decomposition.md`:63-66; `.kb/decisions/0010-the-suite-must-prove-itself.md`; `_grounding.md`:27-31 |
| **B3 — the named set is derived from closed sibling reports, never from the Dependencies list** | Exactly the adapters that had **run the suite and passed** as of the stated date. `project.md`:315-329 records which projects *block* this release, which is a different set and the one a naive implementation reaches for. The reference `MemoryEventStore` is not an adapter result; if named at all it is labelled the reference store. The derivation — which report, read on which date, resolving to which set — is committed under this story's folder | `_decomposition.md`:472 (the named wrong implementation); `project.md`:315-329; `CLAUDE.md` (*an adapter that compiles but has not run the suite is not an adapter*); `_decomposition.md`:97-111; `xtask/src/package.rs`:20-22 |
| **B4 — the empty and thin cases are statable, not paddable** | If no sibling adapter has a closed passing report at the publish commit, the block says exactly that — the suite exists, the rules discriminate, no third-party adapter has run it yet — and still carries the date and the link. Padding to make the block read better is the failure this story exists against. This is the same absence-with-a-reason rule the design applies to every state | `_design.md`:623; `standards/rust/40-public-surface-and-evolution.md`:212; `_decomposition.md`:241-247 (IQ-2) |
| **B5 — the claim is a dated snapshot, supersedable rather than editable** | Fact, implementations and date stated so a later dated claim replaces this one — because publication is irreversible and a yank leaves the rendered page exactly as it is. The form is `README.md`:227-234's: the fact plainly, the date attached, the authority cited by path rather than asserted | `_decomposition.md`:272-288 (IQ-4); `README.md`:227-234; `initiative.md`:434; `standards/rust/51-features-and-no-std.md`:226-231 |
| **B6 — exactly one evidence link, absolute, landing on a specific anchor, reachable without running anything** | No *"run our CI"*, no *"check out the repo"*, no repository root, no bare *"see the specification"*. Absolute URL, because a relative link on a packaged surface resolves against crates.io and 404s. **Ladder, in order, take the highest rung available at the publish commit and record which:** (1) the committed conformance-run report that names the implementations and the date; (2) the sibling project's committed report for the single adapter named; (3) `.kb/decisions/0010-the-suite-must-prove-itself.md` — the provenance discipline that makes a pass mean something — with the implementations and date carried in the block itself, which AP-10 requires regardless. Which rung was taken and why is recorded, not silently chosen: ADR-0010's own *a skip is reported, never silent* applied to the link | `_decomposition.md`:289-296 (IQ-5), `:232-239` (IQ-1); `_design.md`:425, `:555-559`, AP-6 `:656-657`; `.kb/decisions/0010-the-suite-must-prove-itself.md` |
| **B7 — the testkit callout stops contradicting the block** | `crates/happenstance-testkit/README.md`:16-17 states **"No adapter has run this suite"**, roughly ten lines above where B1 puts the block. It is a fifth stale string — AC-UX-008's list names four and this is not one of them — and it is in a file this story edits. Reconciled to the published tree, keeping the callout's existing shape: what changed, and what is still early. If B4's empty case holds, the sentence is *true* and stays; the block and the callout must agree either way | `crates/happenstance-testkit/README.md`:9-22; `_decomposition.md`:351-355 (the four named strings), `:305-311` (IQ-7); `_design.md`:386-389 |
| **B8 — DT-4's promise lands in the Guarantees block, in the caller's register** | One bullet, **≤ 3 rendered lines**, in `crates/happenstance/README.md`:41-49 and `crates/happenstance-core/README.md`:43-54. Content: positions are unique and strictly increasing within one store; **gaps are permitted**; a position is not a count; **code that assumes the next event is at `n + 1` is wrong against a conformant store**; resume from `checkpoint.next()`, an inclusive lower bound rather than a seek. The README states the *consequence for the caller*, the specification states the clause, and **neither is a paraphrase of the other** — that is the structural mitigation against two copies drifting. Not on the testkit README, which has no Guarantees section and does not gain one | `_design.md`:251-256, `:268-272`, `:365`, `:427`, `:471`; `spec/SPECIFICATION.md`:1020-1027 (VT-11), `:1074-1085` (VT-13), `:2748-2760` (ES-9); `_decomposition.md`:323-329 |
| **B9 — exactly one clause link, into a clause ID, and its slug is registered with the link check** | The bullet carries one link into `spec/SPECIFICATION.md`, at VT-11. `cargo xtask spec-trace` guards the clause **ID** and `spec/SPECIFICATION.md`:210-211 retains even a demoted ID *"so that citations resolve"*. But the rendered anchor is slugged from the **whole heading text**, so rewording the descriptive half breaks the link while `spec-trace` stays green. The ID's stability is `spec-trace`'s; the **slug's** stability is AC-UX-011's mechanical check, and this link is registered with it | `_design.md`:273-275; `spec/SPECIFICATION.md`:210-211, `:1020`; `_decomposition.md`:367-370; `xtask/src/spec_trace.rs` |
| **B10 — the nested non-ownership line, subordinate, with its reason and its own link** | Exactly one nested line under the bullet — the only nested bullet permitted anywhere in Guarantees. It states that no conformance rule yet checks a read starting at a position no event occupies, that the rule is named by two accepted decisions and owned by neither, and links the open question. Nested **because subordination is the point**: it must not be readable as the promise. The whole promise nowhere implies ownership of `read_from_a_gap_position`. Draft register, to be tuned to budget: *"Not yet checked: a read that starts at a position no event occupies. The behaviour is frozen; the rule that would check it is named by two accepted decisions and owned by neither — [written down rather than left to be found](…)."* | `_design.md`:276-280, `:428`, `:471`, `:514`, AP-13 `:670-671`; `.kb/open-questions/es-38-and-gap-read-rules-are-unowned.md`; `_decomposition.md`:473, `:241-247` |
| **B11 — the open question is linked, not resolved** | `kb-open-question-es-38-and-gap-read-unowned-001` keeps `status` and body unchanged. This story writes nothing under `.kb/`. Resolving it is its own decision with its own evidence, and the project's risk register names resolving one in passing as a medium/high risk | `project.md`:351; `.kb/open-questions/es-38-and-gap-read-rules-are-unowned.md`; `.kb/maps/open-questions-index.md` |
| **B12 — the Guarantees budget holds and the two blocks stay below the fold** | Guarantees: ≤ 7 bullets, each ≤ 3 rendered lines, exactly 1 link per top-level bullet, one nested exception. This story spends 1 of the 7; `guarantees-and-docs-rs-presentation` spends 3; 3 existing + 1 + 3 = 7, exactly at the ceiling. Both of this story's blocks are **revealed on scroll** — 0 hops, below the fold — and neither may be promoted onto a first screen the owner signed off as **full at 0.2.0** | `_design.md`:470, `:514`, `:595-597`, `:424`, `:427-428`, `:772-783`; `crates/happenstance/README.md`:41-49; `crates/happenstance-core/README.md`:43-54 |
| **B13 — the packaged surface stays compilable and the held anchors survive** | `crates/happenstance/src/lib.rs`:10 compiles this README, so a malformed fence or a broken `include_str!` path is a failing `cargo test`, not a cosmetic defect. This story adds no fence and renames no heading, so `README.md`:9 → `#licence` and `README.md`:16 → `#status` survive by construction — and that is checked mechanically rather than asserted. The story-grain gate is `cargo xtask affected --base main`, which runs the five file-reading lints and `spec-trace` unconditionally | `crates/happenstance/src/lib.rs`:1-10; `_design.md`:625; `_decomposition.md`:260-271, `:367-370`, `:490-497`; `xtask/src/main.rs` |

## Data and migrations

**N/A — there is no schema, no store and no persisted state.** This story adds no type, no field, no
feature flag and no manifest key; it changes three Markdown files that are rendered by third parties.

The reason to say so rather than omit the section is that this medium has the one property a
migration story exists to manage, in a harsher form: **there is no rollback.** `cargo yank` removes a
version from resolution and leaves the rendered crates.io and docs.rs pages exactly as they are
(`standards/rust/51-features-and-no-std.md`:226-231; `_storymap.md`:91-96). A wrong implementation
list or an overstated gaps promise published at `0.2.0` is permanent for that version number.

So the migration posture is **forward-only, by superseding dated claims**, and it is bought before
the act rather than after it (IQ-4, `_decomposition.md`:272-288):

- Every claim is worded as a dated snapshot, so the next release replaces it with a later dated claim
  rather than needing a silent rewrite (B5).
- The reversibility is spent upstream of publication: `rendered-page-preflight` reads the rendered
  pages before the irreversible act and blocks on this story landing (`_storymap.md`:67, `:91-96`).
- The one piece of state that *is* carried forward is a committed artefact rather than a memory: the
  derivation record under this story's folder — reports read, set resolved, date, evidence-link rung
  — which is what a future release re-derives against instead of re-reading the page.

## Acceptance criteria

Every criterion is written from the reader's intent, not from the edit. The reader is **Persona 4,
the evaluator** — one sitting, time-boxed, **cannot run the conformance suite**, and has nothing
external to check "DCB-compliant" against (`../_discovery/distillation/personas-and-journeys.md`:249-313,
`:262-266`, `:333-338`) — except where a criterion serves the **maintainer at the release gate**, who
is the only person who can make the claim false. **U2** is *"show me what I would be trusting, and let
me check it myself"*; **U3** is *"tell me what happens when a read is independent of the write that
produced it"*, and it is the beat all four personas share (`_decomposition.md`:83-84, `:91-96`).

Verification tiers are the testing brief's, unchanged: **human observation on the rendered page** for
the AC-009 family and **human observation plus AC-UX-011's mechanical link check** for the AC-010
family (`_decomposition.md`:472-473). "Rendered observation" here means the **pre-publish** render —
the packaged Markdown as GitHub and a local `cargo package` extract render it — recorded dated at
`.bklg/from-contract-to-published-library/publication-and-positioning/compliance-claim-and-gaps-promise/_rendered-check.md`.
Reading the **published** crates.io page is `rendered-page-preflight`'s and is not re-done here.

| id | criterion | verification |
| --- | --- | --- |
| **AC-001** | **GIVEN** an evaluator has landed on the rendered crates.io page for any of `happenstance`, `happenstance-core` or `happenstance-testkit` and read the first screen, **WHEN** they scroll past the fold, **THEN** the first `##` heading they meet is the compliance block, sited **above** the quick start, presented as one composed visual unit — `##` heading, the claim as the block's first sentence, the date on its own line, one link — carrying **all four** mandatory parts (claim, implementations, date, link) in **≤ 6 rendered lines**, and the block is **byte-identical on all three** packaged READMEs because it is one claim. *(project AC-009; U2; hierarchy `_design.md`:511; AP-10 `:665-666`)* | Dated rendered observation at `…/compliance-claim-and-gaps-promise/_rendered-check.md` recording heading level, part ordering, the block's position relative to the quick start and the measured rendered-line count on each of the three surfaces; a byte-identity assertion across `crates/happenstance/README.md`, `crates/happenstance-core/README.md` and `crates/happenstance-testkit/README.md` registered with the slice's mechanical check (AC-UX-011); `cargo xtask affected --base main` |
| **AC-002** | **GIVEN** an evaluator who knows the DCB specification defines no conformance process and therefore has nothing external to check the adjective against, **WHEN** they read the block's first two sentences, **THEN** the block **concedes that "DCB-compliant" is self-asserted — here as everywhere** — and then states what *is* checkable: which implementations ran `happenstance-testkit`'s rules and passed, and why a pass from this suite discriminates (every rule carries a wrong implementation it rejects, and a declined capability still runs the rule and reports its stated reason rather than vanishing). The evaluator can restate **what was checked**, not merely that something was. *(project AC-009; U2; ADR-0010)* | Dated rendered observation, recorded as a restatement test: the observer writes, from the block alone, what was checked and by what — and that record is the evidence. Cross-read against `.kb/decisions/0010-the-suite-must-prove-itself.md` for any claim the ADR does not support; `cargo xtask affected --base main` |
| **AC-003** | **GIVEN** a maintainer at the publish commit with four sibling adapter projects listed as blockers, **WHEN** they write the implementations line, **THEN** every name in it is an adapter whose **own sibling project report is closed and records a conformance-suite pass** at or before the stated date — derived from those reports, **never** from `project.md`:315-329's Dependencies list, which records blocking and not passing; `MemoryEventStore` appears only if labelled the **reference** store and never as an implementation the claim rests on; **no maturity count appears in the block** (AP-3, `_design.md`:650-651); **and if no sibling has a closed passing report the block says exactly that** — the suite exists, its rules discriminate, no third-party adapter has run it yet — with the date and the link still attached, rather than being padded, softened or omitted. *(project AC-009; the testing brief's named wrong implementation, `_decomposition.md`:472)* | The committed derivation record at `…/compliance-claim-and-gaps-promise/_evidence-derivation.md` naming each sibling report read, its path, its closed state, the date it was read and the set it resolved to — reviewed against `project.md`:315-329 for **non**-equality where the two sets differ; dated rendered observation of the block against that record; `cargo xtask affected --base main` |
| **AC-004** | **GIVEN** publication is the one irreversible act here — `cargo yank` removes a version from resolution and leaves the rendered page exactly as it is — **WHEN** a later release passes the suite against more adapters, **THEN** this version's block is **superseded by a later dated claim rather than needing a silent rewrite**: the fact stated plainly, the implementations stated as of a date, the date attached, the authority cited by path rather than asserted, and **no timeless, forward-looking or aspirational phrasing** anywhere in the block. *(project AC-009; IQ-4 `_decomposition.md`:272-288; `README.md`:227-234's form; `initiative.md`:434)* | Dated rendered observation reading the block for tense and for date attachment, diffed in form against `README.md`:227-234 (the rename precedent); a written statement in `_evidence-derivation.md` of what a later release's block would replace and what it would leave standing; `cargo xtask affected --base main` |
| **AC-005** | **GIVEN** an evaluator who cannot run the suite and will not clone the repository, **WHEN** they follow the block's evidence, **THEN** **exactly one** link leaves the block; it is an **absolute** URL; its text is meaningful standing alone (never "here", "this", or a bare URL); and it lands in **one hop on a specific anchor** in a reachable committed artefact — never a repository root, never a bare *"see the specification"*, never *"run our CI"* or *"check out the repo"*. **The rung of the evidence ladder taken is recorded with its reason** — (1) a committed conformance-run report naming implementations and date, (2) the single named adapter's sibling report, (3) `.kb/decisions/0010-the-suite-must-prove-itself.md` — taking the highest rung available, because a skip is reported and never silent. *(project AC-009; IQ-1 `_decomposition.md`:232-239, IQ-5 `:289-296`; AP-6 `_design.md`:656-657)* | The slice's mechanical link/anchor check (AC-UX-011, owned by `landing-copy-and-status-truth`) resolving the link and asserting it is absolute and lands on a fragment, not a root; the ladder rung and its reason recorded in `_evidence-derivation.md`; dated rendered observation of the hop count and the link text; `cargo xtask affected --base main` |
| **AC-006** | **GIVEN** a reader on the `happenstance-testkit` crates.io page, **WHEN** they read the status callout and then scroll roughly ten lines to the compliance block, **THEN** the two **agree**: `crates/happenstance-testkit/README.md`:16-17's *"No adapter has run this suite"* is reconciled to the tree that ships, keeping the callout's existing what-changed / what-is-still-early shape — **or**, if AC-003's empty case holds, the sentence is true, stands unchanged, and the block agrees with it. No sentence on the page contradicts another sentence on the same page. *(project AC-009; IQ-7 `_decomposition.md`:305-311; the fifth stale string, outside AC-UX-008's list of four at `:351-355`)* | Dated rendered observation of `crates/happenstance-testkit/README.md`:9-22 and the block together in one screenful, recorded as an explicit agree/contradict verdict; a diff review asserting the callout's shape (blockquote, bold lead phrase) survives; `cargo xtask affected --base main` |
| **AC-007** | **GIVEN** a newcomer of **any** of the four personas asking U3 — what happens when a read is independent of the write that produced it — and none of them currently has anywhere to look, **WHEN** they scroll to the Guarantees block of the rendered `happenstance` or `happenstance-core` page (**0 hops** from the landing entry), **THEN** one bullet of **≤ 3 rendered lines** tells them, in plain language and **without reading source**, that positions are **unique and strictly increasing within one store**, that **gaps are permitted**, that a position **is not a count**, that **code assuming the next event is at `n + 1` is wrong against a conformant store**, and that resume is `checkpoint.next()` — stated as the **consequence for the caller**, in the caller's register, and **not as a paraphrase of the clause**. `happenstance-testkit` gains no Guarantees section. *(project AC-010; DT-4's resolution `_design.md`:247-282; AC-UX-003 `_decomposition.md`:323-329)* | Dated rendered observation reading the bullet against U3's question and recording whether it answers it without a hop; a register check asserting the bullet is not a phrase-level paraphrase of `spec/SPECIFICATION.md`:1020-1027; rendered-line count against the ≤ 3 budget; `cargo xtask affected --base main` |
| **AC-008** | **GIVEN** the citation is the *only* thing separating this promise from what two DCB-labelled stores already disagreeing in public can each write, **WHEN** the reader takes the one hop, **THEN** the bullet carries **exactly one** link, absolute, into **VT-11's clause ID** in `spec/SPECIFICATION.md` (not a heading, not the file root), and it **resolves to a live rendered anchor** — **and that anchor's slug is registered with the slice's mechanical link check**, because `cargo xtask spec-trace` guards the clause **ID** while the rendered anchor is slugged from the whole heading text and can die while the gate stays green. *(project AC-010; `_design.md`:273-275; `spec/SPECIFICATION.md`:210-211; AC-UX-011 `_decomposition.md`:367-370)* | The slice's mechanical link/anchor check (AC-UX-011) resolving the fragment against the rendered `spec/SPECIFICATION.md`, with the ID-versus-slug distinction recorded as the reason it is registered there and not left to `spec-trace`; `cargo xtask spec-trace` green (the ID side); `cargo xtask affected --base main`, which runs `spec-trace` unconditionally (`.redkiln/config.yaml`:36-40) |
| **AC-009** | **GIVEN** `read_from_a_gap_position` is named by ADR-0011 and ADR-0013 and taken by neither — *"two accepted decisions have now looked directly at this rule and neither took it"* — **WHEN** a reader reads the promise, **THEN** **nothing in it can be read as claiming the library owns that rule**; a **single nested line** — the only nested bullet permitted anywhere in Guarantees — states the absence **and its reason, attached to the thing it qualifies**, is structurally subordinate so it cannot be read as the promise, and carries its own link to `.kb/open-questions/es-38-and-gap-read-rules-are-unowned.md`. **The open question stays open**: its `status`, its body and `.kb/maps/open-questions-index.md` are unchanged, and the diff contains **no `.kb/` write at all**. *(project AC-010; the testing brief's named wrong implementation, `_decomposition.md`:473; AP-13 `_design.md`:670-671; `project.md`:351)* | Dated rendered observation reading the promise **specifically for any phrase that could be read as claiming the unowned rule**, recorded as an explicit verdict listing the phrases considered; the slice's link check resolving the nested line's open-question link; `git diff --stat` asserting **zero** paths under `.kb/`; `cargo xtask affected --base main` |
| **AC-010** | **GIVEN** the owner signed off a first screen measured at ≈343 px against a 340 px budget and declared it **full at 0.2.0**, **WHEN** the page is read at **1024×768** (14 rendered lines above the fold — the design case, not the degraded one), **THEN** both of this story's blocks are **revealed on scroll, below the fold, 0 hops**, and **neither is promoted onto the first screen** to make it more prominent; the compliance block is **≤ 6 rendered lines** (claim 1, implementations 1–2, date 1, link 1) and is the **only primary-ranked element in its screenful**; and the Guarantees list holds **≤ 7 bullets, each ≤ 3 rendered lines, exactly 1 link per top-level bullet**, of which **this story spends exactly one**, plus its **single** nested exception. *(project AC-009 and AC-010; `_design.md`:424, `:427-428`, `:469-471`, `:514`, `:595-597`, `:772-783`; AP-15 `:674-675`)* | Dated rendered observation at 1024×768 **and** 1440×900 recording, per surface, the fold position, the measured rendered-line count of each block, the Guarantees bullet count and per-bullet link count, and the number of primary-ranked elements in the block's screenful; a slice-level reconciliation with `guarantees-and-docs-rs-presentation` confirming 3 existing + 1 + 3 = 7; `cargo xtask affected --base main` |
| **AC-011** | **GIVEN** `crates/happenstance/src/lib.rs`:10 compiles this README as the crate's own doctest and two live intra-document anchors are already held, **WHEN** the story-grain gate runs on the diff, **THEN** the packaged surface is still a working artefact: the README doctests compile, `cargo package -p happenstance --list` still contains the README and both licence files, `README.md`:9 → `#licence` and `README.md`:16 → `#status` still resolve and **no heading this story adds collides with or shadows either**, and **no raw HTML, inline style, JavaScript, colour, meaning-bearing image or animated media** enters any packaged surface. *(project AC-009 and AC-010; IQ-3 `_decomposition.md`:260-271; `_design.md`:625; AP-7 `:658-659`)* | `cargo test -p happenstance --doc` (the README is compiled via `crates/happenstance/src/lib.rs`:10); `cargo package -p happenstance --list` against `xtask/src/package.rs`:88-94's `REQUIRED_FILES`; the slice's mechanical anchor check (AC-UX-011) resolving both held anchors; `cargo xtask affected --base main`, which is the story-grain gate `.redkiln/config.yaml`:40 wires |

**Coverage of the traced project ACs.** **AC-009** (*the compliance claim is checkable rather than
trusted*, `project.md`:258-261) is carried by AC-001 through AC-006, with AC-010 and AC-011 holding
its presentation and its containment. **AC-010** (*the positions-and-gaps promise is readable without
reading source and does not overstate*, `:262-265`) is carried by AC-007 through AC-009, with AC-010
and AC-011 doing the same. Neither project AC is shared with another story
(`_storymap.md`:132-133), so no criterion here may be left to a sibling.

## Interaction quality

RFC §6.7/D6, in this medium. This story renders three surfaces, so the project's signed-off
`_design.md` is **binding** on its composition and this section re-decides none of it. **Every
invariant below is already an `AC-###` row in the table above** — this section says *which row carries
it* and how it is verified, because `redkiln verify` extracts ACs from table cells and bullets in the
acceptance-criteria section only, and an invariant living as prose here would never be gated.

**The medium's translation, stated once.** The reader's "interaction" is *navigating a claim to its
evidence and back inside one sitting* (`_decomposition.md`:229-231). So *in place* means **on the page
they landed on**; *context jump* means **a hop**; *scroll is not a hop*; *focus and scroll position*
survive as **inbound anchors that still resolve**; and *reversibility* is bought **before publication**
because after it there is none.

### State invariants

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **In place, not a context jump** — 0 hops to *read* a claim, ≤ 1 hop to reach its evidence, and that hop lands on a **specific anchor**, never a repository root and never a bare *"see the specification"* (IQ-1, `_decomposition.md`:232-239) | **AC-005** (the compliance evidence, 1 hop, specific anchor), **AC-007** (the promise itself readable at 0 hops), **AC-008** (the clause hop lands on VT-11's ID) | The slice's mechanical link check resolves each link and asserts a fragment is present; the dated rendered observation records the **hop count** the reader actually spends per claim |
| **Non-occlusion — a filter must not hide what it filters; no surface may state an absence without a reason** (IQ-2, `:241-247`; RS-40-5, `standards/rust/40-public-surface-and-evolution.md`:212) | **AC-003** (the empty case is *stated*, with its reason, not omitted or padded), **AC-009** (the unowned rule is named and reasoned at the point it qualifies, never left off the page) | Rendered observation recorded as an explicit verdict, not a glance: for AC-003 the observer states what the block says when the set is empty; for AC-009 the observer lists the phrases considered for over-claim |
| **Preserved position — inbound anchors survive, or every inbound reference moves in the same change** (IQ-3, `:260-271`; the `Error` state's two held anchors, `_design.md`:625) | **AC-011** | Mechanical, not read: AC-UX-011's anchor check resolves `README.md`:9 → `#licence` and `README.md`:16 → `#status`. An error here renders as **nothing visible**, which is exactly why it is not verified by looking |
| **Reversibility, bought before the irreversible act** — a yank removes a version from the resolver and leaves the rendered page exactly as it is, so every claim is worded to be **superseded by a later dated claim** (IQ-4, `:272-288`; `standards/rust/51-features-and-no-std.md`:226-231) | **AC-004** | Tense-and-date read recorded in `_rendered-check.md`, plus a written statement in `_evidence-derivation.md` of what a later release's block would replace and what it would leave standing |
| **Reachable without running anything** — the evaluator cannot run the suite; no claim may have *"run our CI"* or *"check out the repo"* as its only evidence (IQ-5, `:289-296`) | **AC-002** (what is checkable is *stated*, not deferred to a run), **AC-005** (the evidence is a reachable committed artefact) | The ladder rung and its reason recorded in `_evidence-derivation.md`; the link check proves reachability rather than plausibility |
| **Reachability of the text itself** (this medium's keyboard-reachability analogue) — plain-text legibility is the **floor**, not the fallback: link text meaningful standing alone, no meaning carried by colour or glyph alone, the page correct with images blocked (`_decomposition.md`:214-224; the *Images blocked* and *Theme* states, `_design.md`:629-631) | **AC-005** (link text), **AC-001** (the block's four parts are text, not a badge), **AC-011** (no raw HTML, no colour, no meaning-bearing image) | Rendered observation with images blocked and in both themes, recorded dated; a diff review for raw HTML, inline style and script |
| **Truth at the publish commit** — no sentence on a published surface may be false on the tree that shipped, and no sentence may contradict another on the same page (IQ-7, `:305-311`) | **AC-003** (the named set is the tree's, not the plan's), **AC-006** (the testkit callout and the block agree) | The derivation record is the AC-003 instrument; AC-006 is an explicit one-screenful agree/contradict verdict |

### Composition invariants

Taken from the signed-off `_design.md`. This story renders **R6** on three surfaces and **R9** on two.

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **Presentation exists at all** — the compliance block is a **composed unit**, not bare prose: its own `##` heading, the claim as the block's first sentence, the date on its own line, one link, all four parts in the same block. A sentence containing the word "DCB-compliant" dropped into an existing paragraph fails this even though every string is present (AP-10, `_design.md`:665-666) | **AC-001** | The rendered observation records **heading level, part presence and part order** as separate findings — a checklist of four, not a yes/no |
| **Composition and placement** — R6 sits **below the fold, above the quick start**, and is **the first heading a scrolling reader meets**; R9's bullet sits inside the existing Guarantees list on `happenstance` and `happenstance-core` only. The order is the evaluator's sequence, *is it real → do I try it* (`_design.md`:362, `:369-378`, `:365`) | **AC-001** (R6's site and order), **AC-007** (R9's site, and that the testkit gains no Guarantees section) | Rendered observation records the block's position **relative to the quick start** on each of the three surfaces, not merely that it exists |
| **Transience** — both blocks are **revealed on scroll** (0 hops, below the fold); the **evidence** is **opened on demand** (exactly 1 hop, specific anchor); the **non-ownership line** is revealed **nested under the promise it qualifies**, never on a separate page (`_design.md`:424-428) | **AC-010** (both blocks revealed, neither promoted), **AC-005** (evidence opened on demand), **AC-009** (the absence travels with the thing it qualifies) | Fold position measured at 1024×768 **and** 1440×900 and recorded per surface; hop count recorded per claim |
| **Density budget, with its real numbers** — compliance block **≤ 6 rendered lines** (claim 1, implementations 1–2, date 1, link 1); Guarantees **≤ 7 bullets, each ≤ 3 rendered lines, exactly 1 link per top-level bullet**; the positions-and-gaps bullet **≤ 3 lines + 1 nested line**; a rendered line is 16px text at ~1.5 line-height ≈ **24 CSS px**, blank lines counted; the 1024×768 first screen is **14 rendered lines** and is the binding constraint (`_design.md`:469-471, `:514`, `:452-462`) | **AC-010**, with **AC-001** and **AC-007** carrying their own per-block ceilings | Counted and recorded, never estimated: `_rendered-check.md` carries the measured line count per block per viewport and the Guarantees bullet and link tally. A missed budget is a **finding for this story**, not a silent pass (`_design.md`:441-447) |
| **Hierarchy** — R6 is **primary for the screen it lands in**, carried by `##` + claim-first + date-on-its-own-line, and it is the **only** block on the page whose four parts are all mandatory; R9 is **secondary**, a flat bulleted list with **no nested bullets except this story's non-ownership line**, whose subordination *is* the point; at most **one** primary-ranked element per screenful (`_design.md`:511, `:514`, `:520-522`) | **AC-001** (R6's rank and its carriers), **AC-009** (the nested line's subordination), **AC-010** (one primary per screenful) | Rendered observation counts primary-ranked elements in the block's screenful and asserts the nested line renders **as a nested bullet**, not as a second top-level one |
| **Named anti-patterns** — **AP-10** (a "DCB-compliant" claim on a screen with no date and no named implementations *in the same block*), **AP-13** (a positions-and-gaps statement implying ownership of `read_from_a_gap_position`, or any absence stated without a reason next to it), **AP-6** (a relative link or a 404 on a packaged surface; corollary, a renamed `#status` or `#licence`), **AP-3** (two maturity counts on one surface), **AP-15** (more than one primary per screenful), **AP-7** (raw HTML, style, JS, colour, meaning-bearing image, animated media) (`_design.md`:645-675) | AP-10 → **AC-001**; AP-13 → **AC-009**; AP-6 → **AC-005**, **AC-011**; AP-3 → **AC-003**; AP-15 → **AC-010**; AP-7 → **AC-011** | Each anti-pattern is phrased to be checkable against a **screenshot by someone who cannot read the code** (`_design.md`:642-644), so each is a line item in `_rendered-check.md` with a per-surface verdict |

**Why an unstyled render would pass everything else.** There is no data attribute and no ARIA node on
this medium to assert against; the whole surface *is* composition. A diff that adds every required
string to the bottom of each README satisfies "the claim is present", "the date is present", "the link
resolves" and "the promise mentions gaps" — and fails **AC-001**, **AC-007**, **AC-009** and **AC-010**,
which are precisely the rows that make placement, rank, subordination and budget blocking rather than
advisory.

## Error conditions

| id | Condition | Required behaviour |
| --- | --- | --- |
| **EC-001** | **No sibling adapter has a closed passing report at the publish commit.** The four adapter projects block this release; blocking is not passing | **Not a blocker, and not a reason to route around the block.** The empty case ships: the suite exists, its rules discriminate, no third-party adapter has run it yet — with the date and the link still attached (AC-003, B4). Padding the list is the failure this story exists against |
| **EC-002** | **A sibling report records a partial run** — some rules passed, some capabilities declined by the fixture | Name what ran and what was declined, **carrying the fixture's stated reason**, never rounding up to "passed". This is ADR-0010's own discipline — a declined capability still runs the rule and reports its reason rather than vanishing (`.kb/decisions/0010-the-suite-must-prove-itself.md`) — applied to the claim instead of to the test output |
| **EC-003** | **No committed conformance-run report exists**, so the evidence ladder's rungs 1 and 2 are both unavailable | Descend to rung 3 — `.kb/decisions/0010-the-suite-must-prove-itself.md` — with the implementations and the date carried **in the block itself**, which AP-10 requires whichever rung the link takes. Record the rung and *why* it was the highest available in `_evidence-derivation.md`. A silently chosen rung is the same defect as a silent skip |
| **EC-004** | **VT-11's heading is reworded upstream**, so the rendered anchor slug dies while the clause ID survives and `cargo xtask spec-trace` stays green | Caught by the slice's link check, not by the gate (AC-008). **Fix the link, never the clause.** VT-11 is `[FROZEN]`; editing it to restore a slug is exactly the failure DR-15 and project AC-015 exist against (`project.md`:281-283) |
| **EC-005** | **The slice's mechanical link/anchor check (AC-UX-011) does not exist yet** when this story implements — it is `landing-copy-and-status-truth`'s instrument (`_storymap.md`:64) | **Halt and report the missing dependency loudly.** Do not build a second checker, and do not substitute reading for it: IQ-3's failure mode renders as **nothing visible**, which is why the design specifies it is checked mechanically. The two are slice-mates in one context, so the correct resolution is ordering within the slice, not a workaround |
| **EC-006** | **The Guarantees list is already at 7 bullets** when this story's bullet arrives, because the slice-mate landed its three first | Coordinate inside the slice; if something must yield, demote per `_design.md`'s `## Density budget` yield order and **state the demotion**. Never delete an existing bullet silently, and never nest a bullet to dodge the count — the nested line is the **one** permitted exception and it is spoken for (`_design.md`:514, `:595-597`) |
| **EC-007** | **Writing the caller-facing sentence appears to require a `[FROZEN]` clause to say something it does not say** | **Blocker, and a new decision atom — not an edit.** VT-11, VT-13 and ES-9 are all frozen; `project.md`:124-126 reserves any such change for an upstream project and a re-plan. Stop and report rather than softening the sentence until it fits |
| **EC-008** | **The testkit callout and the compliance block disagree** at the publish commit | **The callout's truth wins.** Correct the block; never soften the callout to match a claim. If EC-001's empty case holds, `crates/happenstance-testkit/README.md`:16-17 is true and stands unchanged (AC-006) |
| **EC-009** | **A sibling report names an adapter that is not in this release's crate set** (`crate-set-decision` ships exactly three crates) | The claim is about **what ran the suite**, not about what ships. Name it, and say what it is — an adapter outside this release — rather than dropping it or implying it is packaged here. Silence would understate the evidence; implication would overstate the package |

## Non-functional

| id | Requirement | Why, and where it comes from |
| --- | --- | --- |
| **NF-001** | **Forward-only. There is no rollback for this medium.** `cargo yank` removes a version from resolution and leaves the rendered crates.io and docs.rs pages exactly as they are. A wrong implementation list or an overstated promise published at `0.2.0` is permanent for that version number | `standards/rust/51-features-and-no-std.md`:226-231; IQ-4 (`_decomposition.md`:272-288); `_storymap.md`:91-96. Bought before the act by AC-004's dated-snapshot wording and by `rendered-page-preflight` reading the pages first |
| **NF-002** | **Plain-text legibility is the floor.** The evaluator's decision must be reachable from the text alone: correct with images blocked, correct in light, dark and ayu, no meaning in colour or glyph alone, every fence languaged, every link meaningful standing alone | `_decomposition.md`:214-224; the *Images blocked* and *Theme* states (`_design.md`:629-631). We set no colour, so both themes are legible **only** if we add nothing |
| **NF-003** | **Containment: the copy must live inside the package.** `include_str!` resolves against the file tree and a path reaching outside the package does not exist inside the `.crate`. Copy written only at the repo root is invisible to a registry reader | `README.md`:131-137; `_design.md`:543-553; `xtask/src/package.rs`:88-94's `REQUIRED_FILES`. Containment is **not** presentation — `xtask/src/package.rs`:4-18 says so explicitly, which is why AC-001 and AC-010 are observed rather than asserted |
| **NF-004** | **No gate cost, and no gate regression.** This story adds no compile-time dependency, no feature and no test binary; the story-grain gate's runtime is unchanged. The docs build and the README doctest stay green | `.redkiln/config.yaml`:40; `crates/happenstance/src/lib.rs`:10 |
| **NF-005** | **One full copy per register, and exactly one.** The promise exists in full in the caller's register (the READMEs) and in full as a clause (`spec/SPECIFICATION.md`); the quick-start fence's one comment is a **one-line reduction** of the bullet, agreed once across the slice. Two paraphrases of one promise is the divergence failure DT-4 named and mitigated structurally | `_design.md`:268-272, `:702`, `:709-711`. The maintenance cost of "both" is what DT-4 paid for deliberately; a third uncoordinated copy spends it twice |
| **NF-006** | **Immutability of the knowledge base.** No accepted decision atom is edited, no open question's `status` moves, no map row is written. The diff contains zero paths under `.kb/` | `CLAUDE.md`'s accepted-decision immutability rule; `project.md`:351; `redkiln validate --kb` checks accepted atoms against `HEAD` |

## Implementation notes (non-prescriptive)

Hints, not instructions. The sections above are the contract; how the sentences get written is the
implementer's.

- **Derive before you draft.** The implementation list is an input, not a phrasing choice. Read each
  sibling adapter project's report first, write `_evidence-derivation.md`, *then* write the block. In
  the other order the sentence exists before the facts do and it will bend them — which is the exact
  wrong implementation `_decomposition.md`:472 names.
- **Count rendered lines by counting, not by eye.** 16px text at ~1.5 line-height ≈ 24 CSS px per
  rendered line, blank lines included, at ~95 characters per line in the crates.io content column
  (`_design.md`:452-456). A block that is 6 source lines can be 8 rendered ones at 1024×768.
- **Write the block once and copy it three times, then diff the three.** AC-001's byte-identity is
  cheaper to hold by construction than to restore after three independent edits, and it is what makes
  "it is the same claim" true rather than asserted (`_design.md`:389).
- **The nested line's register is worth drafting twice.** The front half's draft is deliberately in
  the *repository's* voice — *"the rule that would check it is named by two accepted decisions and
  owned by neither"* — and a reader who has never seen an ADR needs it in theirs. Tune to budget, keep
  the two facts (nothing checks it yet; that is recorded rather than forgotten) and keep the link.
- **Absolute URLs follow the tree's existing base.** `crates/happenstance/README.md`:47-48 already uses
  `https://github.com/Wet-Ink-Corporation/happenstance/blob/main/…`; match it rather than inventing a
  second form. The root README's links are repo-relative and must **not** be copied across the
  boundary (`_design.md`:555-559).
- **Anchor slugs are lowercased, punctuation-stripped and hyphenated from the whole heading text.**
  Derive the VT-11 fragment from the current heading at `spec/SPECIFICATION.md`:1020 rather than
  guessing it, and hand it to the slice's link check the moment you write it (AC-008, EC-004).
- **Reconcile the testkit callout by editing the sentence, not the shape.** `crates/happenstance-testkit/README.md`:9-22
  already states both what changed and what is still early, and that shape is why the design demotes
  the census below the fold on that surface (`_design.md`:386-389). Keep the blockquote and the bold
  lead phrase.
- **Talk to the slice-mates before spending the seventh bullet.** `guarantees-and-docs-rs-presentation`
  spends three of the seven and owns the quick-start fence whose comment reduces this story's bullet.
  Both seams are one conversation, not two merge conflicts (EC-006, NF-005).
- **If a criterion here reads as wrong once you are in the file, fix it and say why in the same
  change** — `CLAUDE.md`'s standing rule. Do not satisfy the letter of a criterion you believe is
  mis-specified.

## Tests and CI (merge gate)

Grounded in the project testing brief (`_decomposition.md`:440-500). This story adds **no conformance
rule and no new gate step**: no rule in `crates/happenstance-testkit/src/suite.rs` can observe README
prose, and one that could would be decorative — `CLAUDE.md`'s *"a rule that no adapter can fail is
decorative"* applied to the instrument rather than to the port. The brief assigns AC-009 **human
observation on the rendered page** and AC-010 **human observation plus AC-UX-011's mechanical link
check** (`:472-473`); this table is those two tiers made concrete, plus the standing gate this diff
must not break.

| tier | command / path | proves |
| --- | --- | --- |
| **story gate (automatic)** | `cargo xtask affected --base main` — wired at `.redkiln/config.yaml`:40 | The diff maps to the packages it touches and their dependents, and the **five file-reading lints and `spec-trace` run unconditionally** — which is what gates a story whose whole deliverable is Markdown. Nothing here is exempt from fmt, clippy `-D warnings` and the tests for the affected set |
| **compile (packaged surface)** | `cargo test -p happenstance --doc` | `crates/happenstance/src/lib.rs`:10's `#![cfg_attr(doctest, doc = include_str!("../README.md"))]` compiles this README, so a malformed fence, a broken relative path or an accidental fence edit is a **failing test**, not a cosmetic defect (AC-011) |
| **containment (static)** | `cargo package -p happenstance --list` against `xtask/src/package.rs`:88-94's `REQUIRED_FILES`; run for all three packaged crates | The edited README is **inside the `.crate`** and both licence files travel with it. Containment only — `xtask/src/package.rs`:4-18 is explicit that this cannot stand in for presentation (AC-011, NF-003) |
| **clause citation (static)** | `cargo xtask spec-trace` (`xtask/src/spec_trace.rs`), run unconditionally by the story gate | VT-11's clause **ID** still resolves. It does **not** prove the rendered anchor slug survives — that gap is AC-008's whole point and is why the slug is registered elsewhere |
| **link and anchor (mechanical)** | The slice's mechanical link/anchor check — AC-UX-011, owned by `landing-copy-and-status-truth` (`_decomposition.md`:367-370, `_storymap.md`:64). This story **registers** three new links (the evidence link, the clause link, the nested line's open-question link) and its new `##` anchor, and asserts both held anchors still resolve | AC-005, AC-008, AC-009, AC-011. Checked mechanically because an `Error` state on this medium renders as **nothing visible** (`_design.md`:625). Absent instrument → **EC-005**, halt and report |
| **rendered observation (human, dated)** | `.bklg/from-contract-to-published-library/publication-and-positioning/compliance-claim-and-gaps-promise/_rendered-check.md` — the **pre-publish** render at **1024×768 and 1440×900**, images blocked and both themes, per surface, with measured rendered-line counts and a per-anti-pattern verdict | AC-001, AC-002, AC-004, AC-006, AC-007, AC-009, AC-010. The only instrument that can perceive composition; every other check here is satisfied by a correct, unreadable page |
| **derivation record (committed artefact)** | `.bklg/from-contract-to-published-library/publication-and-positioning/compliance-claim-and-gaps-promise/_evidence-derivation.md` — sibling reports read with paths and dates, the resolved set, the evidence-ladder rung and why it was the highest available | AC-003, AC-004, AC-005, and EC-002/EC-003's recording obligations. It is what a future release re-derives against instead of re-reading the page |
| **KB immutability (static)** | `git diff --stat` asserting zero paths under `.kb/`; `redkiln validate --kb` still clean | AC-009's *the open question stays open*, and NF-006 |
| **integration grain (slice, automatic)** | `cargo xtask ci --fast` — `.redkiln/config.yaml`:55 | The non-terminal project's integration bar: everything in the mandatory gate except the two feature powersets, `cargo deny` and the nightly `--cfg docsrs` build, **including all four `wasm32` steps**. Run over the whole `published-surface-copy` slice, not per story |
| **not run here** | `cargo xtask ci` (full) on the literal publish commit; the read of the **published** crates.io and docs.rs pages | `publish-0-2-0`'s (project AC-016) and `rendered-page-preflight`'s (project AC-007) respectively. Naming them keeps this story from claiming a bar it does not clear |

## Risks and coupling (PR-scoped)

| Risk | Likelihood × impact | Mitigation, in this PR |
| --- | --- | --- |
| **The implementation list is written from the Dependencies list.** Four sibling projects are named as blockers in `project.md`:315-329, and that list is the nearest thing to hand when the sentence needs a set | High × High — it is the testing brief's own named wrong implementation, and it produces a **published, permanent** claim broader than what was checked | AC-003 makes the derivation a committed artefact with a named source per name; `_evidence-derivation.md` must record the two sets **and where they differ**. The derivation precedes the drafting |
| **The gaps promise reads one clause too far.** *"Reads work correctly at a gap position"* is the natural next sentence and it claims an unowned rule | High × High — AP-13, and the promise is permanent for the version | AC-009 requires an explicit recorded verdict listing the phrases considered, not a glance; the nested line ships in the same bullet, subordinate, with its own link |
| **The block lands as a sentence rather than as a block.** Every required string present, no composed unit — passes a string search, fails the design | Medium × High — it is the failure an unstyled render always survives | AC-001 checks heading level, part presence, part order and position relative to the quick start as **four separate findings**; AP-10 is a line item in `_rendered-check.md` |
| **Guarantees hits the ceiling in the wrong order.** 3 existing + 1 (this story) + 3 (`guarantees-and-docs-rs-presentation`) = exactly 7, with no headroom | Medium × Medium — a merge conflict at best, a silent demotion at worst | EC-006; the slice is implemented in **one context** precisely so the seventh bullet is a conversation and not a race (`_storymap.md`:87-90) |
| **The clause anchor dies quietly.** `spec-trace` guards the ID; the rendered slug comes from the whole heading | Medium × Medium — the link 404s on a page that cannot be edited | AC-008 registers the **slug** with AC-UX-011's check and names the gap in the spec rather than leaving it to be discovered at publish; EC-004 forbids the frozen-clause "fix" |
| **The two copies of the promise diverge** — the README bullet, the clause, and the quick-start fence's one-line comment | Medium × Medium — divergence is the failure mode DT-4 accepted when it chose *both* | NF-005: different registers by design, one full copy each, the fence's comment agreed once across the slice as a **reduction** rather than a paraphrase |
| **AC-UX-011's checker is not built yet.** It is a slice-mate's deliverable and this story depends on it for four ACs | Medium × High — the temptation is to substitute reading, which cannot see a dead anchor | EC-005: halt and report. Never a second checker; the gate is *defined once* in `xtask/src/main.rs` |
| **The testkit contradiction is missed** because no upstream artifact lists it — AC-UX-008 names four stale strings and this is a fifth | Medium × Medium — a page that contradicts itself inside one screenful, on a published surface | AC-006 makes it a criterion of this story rather than an observation in its executive summary. It is in a file this story already edits |
| **An open question gets resolved in passing** by prose that reads as settling it | Low × High — the project's own risk register rates it medium/high, and the open-questions index requires resolution, never deletion | AC-009 asserts a **zero-path `.kb/` diff** mechanically; the nested line links the question rather than answering it |

**Coupling, stated plainly.** This story is **tightly coupled to its two slice-mates by file** —
`guarantees-and-docs-rs-presentation` edits the same Guarantees list and owns the fence whose comment
reduces this story's bullet; `landing-copy-and-status-truth` owns the link checker this story
registers with and edits the same three READMEs elsewhere. It is **coupled to four sibling adapter
projects by fact**, not by file: their reports determine AC-003's set, and nothing in this PR can
compensate for a set that is empty (EC-001). It is **coupled to nothing in the Rust surface at all** —
no crate's API, no feature, no manifest.

## Dependencies

**Blocks on**

- **`first-contact-design-resolutions`** — the only hard blocker, and it is a *content* dependency
  rather than a build one. DT-4's resolution (option (c), the site, the register, the ≤ 3 lines plus
  one nested line, the exactly-one clause link) is the input AC-007, AC-008 and AC-009 are written
  against; without it this story would be re-deciding the design instead of implementing it
  (`_storymap.md`:65; `_design.md`:247-282, signed off 2026-08-12 with no conditions at `:772-783`).

**Slice-mates — implemented in one context, mounted as one surface (`published-surface-copy`)**

- **`landing-copy-and-status-truth`** — owns AC-UX-011's mechanical link/anchor check, which four of
  this story's criteria are verified by (AC-005, AC-008, AC-009, AC-011). Its absence is **EC-005**,
  a halt. It also owns the root `README.md`'s four edits and the four stale strings, which this story
  must not touch.
- **`guarantees-and-docs-rs-presentation`** — shares the 7-bullet Guarantees ceiling (it spends three,
  this story spends one) and owns the quick-start fence whose single comment is a one-line reduction
  of this story's bullet. **NF-005** and **EC-006** are the two seams.

**Unlocks**

- **`rendered-page-preflight`** — reads the *rendered* pages against the accessibility floor before the
  irreversible act, and lists this story as a blocker (`_storymap.md`:67). It cannot read a compliance
  block that does not exist.
- **`publish-0-2-0`** transitively, through the preflight — and with it the whole `the-release-event`
  slice, including `stranger-install-smoke`.

**Depends on, upstream of the project** — the four sibling adapter projects named at
`project.md`:315-329 supply AC-003's facts. They block the *release*, not this PR: if none has closed
with a pass, EC-001's empty case ships and this story is still done.

## Anchors (progressive disclosure)

Every path below exists in the tree at spec time. Open them **when the row says**, not on first load —
the `## Context pack` is the must-read core and is self-sufficient for drafting both blocks.

| anchor (real path) | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `.bklg/from-contract-to-published-library/publication-and-positioning/_design.md` | The **binding** signed-off design. DT-4's full resolution with the two options that lost and the three structural divergence mitigations (`:247-282`); R6 and R9's slots (`:362`, `:365`); transience (`:424-428`); the density table with its per-item numbers (`:469-471`); hierarchy (`:511`, `:514`, `:520-522`); the packaging and link rules (`:543-559`); the `Error` state's two held anchors (`:625`); AP-10, AP-13, AP-15 (`:665-675`) | **Before writing either block** — `:247-282` for the promise's shape, `:441-471` for the budgets you will be measured against | AC-001, AC-007, AC-009, AC-010 |
| `.bklg/from-contract-to-published-library/publication-and-positioning/_decomposition.md` | The UX brief's seven interaction-quality invariants in full (`:229-311`) and the eleven AC-UX rows (`:315-372`) — AC-UX-002 and AC-UX-003 are this story's, AC-UX-011 is the instrument it registers with. The testing brief's AC-009 and AC-010 rows (`:472-473`) fix the tier and name the wrong implementation each must reject | **Before writing the acceptance evidence**, and again when deciding what `_rendered-check.md` must record | AC-001, AC-005, AC-008, AC-011 |
| `.kb/decisions/0010-the-suite-must-prove-itself.md` | The evidentiary backbone the whole claim rests on, and the only artifact that makes *"passed"* mean something to a reader who cannot run the suite. Also the rung-3 link target if the ladder descends that far, and the source of EC-002's *report the decline, never round up* | **When drafting AC-002's second sentence**, and again at the evidence-ladder decision | AC-002, AC-005 |
| `.kb/open-questions/es-38-and-gap-read-rules-are-unowned.md` | The nested line's link target and the *exact current text* the non-ownership sentence must be drafted against rather than paraphrased from memory — *"two accepted decisions have now looked directly at this rule and neither took it"*. It stays **open**; linking is not resolving | **Immediately before drafting the nested line.** Read it; do not summarise it from this spec | AC-009 |
| `spec/SPECIFICATION.md` | VT-11 at `:1020-1027` (`[FROZEN]`) is the clause the one link cites and the source of the caller-facing facts; VT-13 at `:1074-1085` carries the `checkpoint.next()` resume idiom; ES-9 at `:2748-2760` is *`from` names a position, not an index* — the behaviour the unowned rule would check. `:210-211` is the ID-retention precedent AC-008's slug caveat turns on | **When drafting the promise bullet**, and again to derive the anchor fragment from the live heading text rather than guessing it | AC-007, AC-008 |
| `crates/happenstance/README.md` | The mount point. The Guarantees list this story's bullet joins is `:41-49` (three bullets today); `:47-48` shows the absolute-URL base to match; `:30-39` is the fence that is **not** this story's | **Before the first edit**, to see the existing budget and link form | AC-001, AC-007, AC-010, AC-011 |
| `crates/happenstance-core/README.md` | The second Guarantees mount, `:43-54` (four bullets today) — a different count from `happenstance`, which the 7-bullet arithmetic must respect **per surface** rather than once | **When placing the promise bullet on the second surface** | AC-007, AC-010 |
| `crates/happenstance-testkit/README.md` | `:9-22` is the status callout whose `:16-17` sentence — *"No adapter has run this suite"* — this story reconciles, and whose blockquote-plus-bold-lead shape must survive the edit. It is the fifth stale string, named by no upstream artifact | **When making the AC-006 edit**, and again during the rendered observation, which must see it and the block in one screenful | AC-006 |
| `README.md` | `:227-234` is the **claim-with-evidence precedent** the block's form copies — the fact plainly, the date attached, the authority cited by path. `:9` and `:16` are the two held inbound anchors. `:131-137` explains why root-only copy is invisible to a registry reader | **When drafting the block's tense and date form** (AC-004), and when checking the held anchors | AC-004, AC-011 |
| `.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md` | The evaluator persona in full (`:249-313`), why they **cannot run the suite** (`:262-266`), the one-sitting time box (`:333-338`), and that U3 is the beat all four personas share with nowhere to look. It is what makes IQ-1's hop budget a constraint rather than a preference | **When the wording of a criterion feels over-cautious** — the persona is the argument for the caution | AC-002, AC-005, AC-007 |
| `.bklg/from-contract-to-published-library/publication-and-positioning/project.md` | `:258-265` are the two traced project ACs verbatim; `:315-329` is the Dependencies list AC-003 must **not** derive from; `:351` is the risk-register row on resolving an open question in passing; `:124-126` is the *Out of scope* that makes EC-007 a blocker | **When resolving the implementation set** (to see what it is not), and when tempted by EC-007 | AC-003, AC-009 |
| `.bklg/from-contract-to-published-library/publication-and-positioning/compliance-claim-and-gaps-promise/discover.md` | The story's own signal ledger, the **two questions it deferred to this spec** (which implementations get named; the exact non-ownership wording) and the named wrong implementation in the discover stage's own words. Its line citations predate the design's amendment rider — see the note under `## Scope lock` | **At the start**, to see what was already decided and what was deliberately left | AC-003, AC-009 |
| `standards/rust/40-public-surface-and-evolution.md` | RS-40-5 at `:212` — a declined capability's constructor **rejects an empty reason**. It is the mechanism IQ-2 borrows, and the reason *"no surface may state an absence without a reason"* is a rule here rather than a preference | **When drafting the empty case (AC-003) and the nested line (AC-009)** | AC-003, AC-009 |
| `standards/rust/51-features-and-no-std.md` | `:226-231` — what `cargo yank` does and does not do. It is the whole basis of NF-001 and of AC-004's supersedable wording | **Once, before writing any dated claim**, if the irreversibility is not already felt | AC-004 |
| `xtask/src/package.rs` | `:88-94`'s `REQUIRED_FILES` is the containment assertion the packaged README must satisfy; `:4-18` states explicitly that containment **is not presentation**, which is why AC-001 and AC-010 are observed rather than asserted | **When verifying AC-011**, and when tempted to treat `cargo package --list` as proof the page is right | AC-011 |
| `xtask/src/spec_trace.rs` | The instrument that guards clause **IDs**. Reading it is how you see that it never looks at a rendered anchor slug — the gap AC-008 registers elsewhere | **Only if AC-008's ID-versus-slug distinction is not yet believed** | AC-008 |
| `crates/happenstance/src/lib.rs` | `:10`'s `#![cfg_attr(doctest, doc = include_str!("../README.md"))]` is why a README edit can fail `cargo test`; `:73`'s `#![doc(html_no_source)]` is an existing deliberate choice **not to flip in passing** | **Before editing the README**, to know what compiles it | AC-011 |
| `.redkiln/config.yaml` | `:40` is the story-grain gate command, and `:36-40`'s comment explains why the five file-reading lints and `spec-trace` run unconditionally — which is what gates a Markdown-only diff at all; `:55` is the slice's integration bar | **When wiring the merge gate**, or when wondering what checks a prose-only story | AC-011 |

## Clarifications resolved during spec

1. **The AC set is exactly the eleven the first pass enumerated** — AC-001 through AC-011 — and none
   was added or dropped. They partition the front half's thirteen behaviours: B1→AC-001, B2→AC-002,
   B3 and B4→AC-003, B5→AC-004, B6→AC-005, B7→AC-006, B8→AC-007, B9→AC-008, B10 and B11→AC-009,
   B12→AC-010, B13→AC-011. B4's empty case rides with B3 because they are one sentence written under
   two conditions; B11's *the open question stays open* rides with B10 because the same nested line
   both links the question and must not resolve it.
2. **`discover.md`'s first deferred question is answered by a rule, not by a list.** *Which
   implementations does the claim name* cannot be answered at plan time — the four sibling projects
   are still in flight. AC-003 fixes the **derivation rule** (closed sibling reports recording a pass,
   never the Dependencies list) and makes the **empty outcome statable** (EC-001), so the story is
   implementable regardless of what the tree looks like at the publish commit. Deriving a list here
   would have been the wrong implementation wearing planning clothes.
3. **`discover.md`'s second deferred question is answered as a register plus two facts, not as a
   fixed string.** The nested line must carry *nothing checks it yet* and *that is recorded rather
   than forgotten*, drafted against the open question's current text and tuned to the one-nested-line
   budget. B10 carries a draft in the repository's voice; the implementer is expected to move it into
   the caller's. Fixing an exact sentence here would over-specify prose the design deliberately left
   to the writer.
4. **The "human observation" tier is given a committed artefact.** `_decomposition.md`:472-473 assigns
   rendered observation, and a tier with no output is not a check. `_rendered-check.md` and
   `_evidence-derivation.md` are named as the two artefacts this story commits, both under the story's
   own folder, both inside the PR boundary the front half drew. Neither exists yet, which is why
   neither appears in the anchors table.
5. **The rendered read here is the pre-publish one.** `rendered-page-preflight` owns reading the
   *published* crates.io and docs.rs pages (`_storymap.md`:67) and blocks on this story. Saying so
   keeps this story from claiming a bar it cannot clear, and keeps the preflight from assuming this
   one already did it.
6. **AC-UX-011's checker is treated as a hard input, not as something this story may build.** Four
   criteria are verified by it and it belongs to a slice-mate. EC-005 makes its absence a **halt and
   report** rather than an improvisation — building a second one would put two link checkers in a
   repository whose whole gate is *defined once in `xtask/src/main.rs`*.
7. **The ID-versus-slug gap is named here rather than discovered at publish.** No upstream artifact
   carries it: `_design.md`:273-275 chose a clause ID over a heading precisely because `spec-trace`
   guards IDs, which is correct about the *gate* and silent about the *rendered anchor*. AC-008 splits
   the obligation across the two instruments explicitly.
8. **`crates/happenstance-testkit/README.md`:16-17 is claimed by this story** although AC-UX-008's
   list names only four stale strings and this is a fifth (`_decomposition.md`:351-355). Nothing else
   in the plan is obliged to fix it, it sits ten lines above where this story puts a block that
   contradicts it, and it is in a file this story already edits. AC-006 is the criterion; leaving it
   for a sibling would have been the cheapest way to ship a self-contradicting page.
9. **EC-009 was added during this pass.** A sibling report may name an adapter outside the
   three-crate release set `crate-set-decision` fixes. The claim is about what ran the suite, not
   about what ships — so such an adapter is named and labelled rather than dropped (which would
   understate the evidence) or listed unqualified (which would imply it is packaged here).
10. **No `_design.md` line citation in this half was taken from `discover.md`.** Every number is
    against the current file, per the note under `## Scope lock`; where `discover.md` says `:228-261`
    for DT-4 and `:448` for the density row, this spec says `:247-282` and `:469` respectively.
