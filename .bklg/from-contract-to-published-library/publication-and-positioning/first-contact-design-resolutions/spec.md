---
item: HS-S0087
stage: spec
created: 2026-08-12T13:47:25.475Z
updated: 2026-08-12T13:47:25.475Z
template_sig: 87bbf1d0
rendered_sig: a1349968
---

# Spec — DT-1, DT-4, DT-5 and DT-6 are decided, each naming what lost

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` — BR-15, BR-17; the *Open design tensions* table at `:420` (DT-1), `:423` (DT-4), `:424` (DT-5), `:425` (DT-6); DoD 10 (`:387-389`) and DoD 16 (`:405-407`) |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md`:310-314 — the evaluator-vs-application-author flag this story's atom closes; `:115-121` — the 0017–0028 ADR allocation |
| Project | `.bklg/from-contract-to-published-library/publication-and-positioning/project.md` — DR-11 (`:202-204`), DR-13 (`:207-212`), AC-011 (`:266-269`), AC-013 (`:273-276`) |
| This spec | `.bklg/from-contract-to-published-library/publication-and-positioning/first-contact-design-resolutions/spec.md` |
| **Design (binding, and the source text)** | `.bklg/from-contract-to-published-library/publication-and-positioning/_design.md` — DT-1 `:191-245` (including the owner's amendment rider `:222-243`), DT-4 `:247-282`, DT-5 `:284-318`, DT-6 `:320-345`; sign-off `:772-789` |
| Key briefs | `.bklg/from-contract-to-published-library/publication-and-positioning/_decomposition.md` — the **UX brief**'s IQ-1 (`:232`), IQ-2 (`:241-247`), IQ-6 (`:297-304`), IQ-7 (`:305-311`), AC-UX-001/003/004/005 (`:313`, `:323`, `:330`, `:335`) and its explicit hand-off *"this brief does not decide DT-1, DT-4, DT-5 or DT-6"* (`:380-382`, `:782-784`); the **testing brief**'s AC-011 row (`:474`) which fixes the verification tier and names the wrong implementation |
| Grounding | `.bklg/from-contract-to-published-library/publication-and-positioning/_grounding.md`:153-158 — grepped `.kb` for every DT id and found nothing: this atom is new, not an update |
| Story map row | `.bklg/from-contract-to-published-library/publication-and-positioning/_storymap.md`:59; the consumed-by table at `:107`; why `release-decisions` is one ingest wave at `:73-78`; merge position at `:160` |
| Discover stage | `.bklg/from-contract-to-published-library/publication-and-positioning/first-contact-design-resolutions/discover.md` — the signal ledger (`:16-29`), the two questions deferred to this spec (`:34-35`) and the named wrong implementation (`:43`) |
| Roadmap pointer | none. `RUNBOOK.md` has no positioning phase; DT-1/4/5/6 enter the plan through the initiative charter, not the runbook — which is why `crate-set-decision` owns the only `RUNBOOK.md` edit in this slice |

## One-line PR slice

Resolve DT-1, DT-4, DT-5 and DT-6 in this project's `_design.md`, each naming the option that lost,
plus the evaluator-vs-application-author persona question the resolutions turn on
(`_decomposition.md`:310-314), and carry the settled answers into an ingested decision atom.

## Executive summary

**What already exists, and is not re-decided.** All four resolutions are written and signed off:
`_design.md` carries DT-1 at `:191-245`, DT-4 at `:247-282`, DT-5 at `:284-318` and DT-6 at
`:320-345`, each naming its rejected options with the reason each lost, each closing with a
`*Resolves:*` line, and the whole file approved by the repository owner on 2026-08-12 with **no
conditions** (`:772-789`). The persona question `_decomposition.md`:310-314 left open is resolved
inside DT-1 (`:214-220`) and then *amended in place by the owner at the spec gate* (`:222-243`).
Project **AC-011** is therefore satisfied by-existing, and this story's first job is to prove that
rather than to redo it.

**The delta this PR lands is durability.** A `_design.md` is a backlog artifact — work in motion,
under `.bklg/`, which is archived when the initiative closes. The four answers it holds are
consumed by three copy stories in the next milestone and then by every future release that touches
a published sentence, and nothing outside this initiative's folder can currently see them. After
this PR the whole first-contact positioning decision — four winners, seven named losers, and the
persona call in its amended form — is **one accepted decision atom** in `.kb/decisions/` at a
number ≥ 0030, authored through `.kb/_intake/` and `/redkiln:kb-ingest`, carrying a row on
`.kb/maps/decision-map.md` so the corpus can navigate to it.

**The risk this PR is written against is transcription loss, not disagreement.** The named wrong
implementation (`discover.md`:43) is an atom that states four winners, validates cleanly, is
correctly numbered, and quietly drops the *what lost and why* half — recording DT-5 as *"publish a
summary"* without carrying that option (b), frozen-guarantees-only, is **forbidden rather than
merely worse**, or that option (a) lost on a 200-rows-against-14-rendered-lines density argument.
Every acceptance criterion below is shaped by that: the checks count options, compare against
`_design.md`'s own wording, and require the *amended* persona consequence rather than the
superseded one.

Two stories block on this landing — `landing-copy-and-status-truth` (HS-S0092) and
`compliance-claim-and-gaps-promise` (HS-S0093) — and a third,
`guarantees-and-docs-rs-presentation`, writes into the same Guarantees block DT-4 sites its promise
in. None of them re-derives an answer.

## Context pack

Everything here is settled. This story transcribes with fidelity and proves the transcription; it
does not re-open a signed-off design. Depth stays behind the anchors table.

**1. The four answers, in the form the atom must carry them.** Each is a winner *plus* a shape —
the shape is part of the decision and collapsing it is the failure this story exists against.

- **DT-1 — the lead claim is storage-agnostic proof, option (a), with the proof stated as an act
  the reader can perform rather than as an adjective** (`_design.md`:196-200). Not "we lead with
  the contract"; the first screen says *what this is and how you can check it*.
- **DT-4 — the positions-and-gaps promise is stated in both places, option (c), in a specified
  shape** (`_design.md`:251-256): stated **in full and in the caller's register** inside the
  Guarantees block of `crates/happenstance/README.md`:41-49 and
  `crates/happenstance-core/README.md`:43-54, as one bullet of ≤ 3 rendered lines plus one nested
  line, carrying **exactly one** link, into a clause ID rather than a heading.
- **DT-5 — the maturity vocabulary is published as a summary that links the ledger, option (c), as
  a single dated census sentence that defines all four words inline** (`_design.md`:288-296),
  sited immediately after the status callout.
- **DT-6 — the peer comparison is stated, option (a)** (`_design.md`:324-326): in full in the
  existing **Prior art** slot (`README.md`:218-225) on the GitHub surface, and in a one-sentence,
  one-link reduced form on the packaged `happenstance` README. Not a new section on either.

**2. Eleven options were on the table and seven of them lost.** The initiative's DT table
enumerates them: DT-1 (a)(b)(c) at `initiative.md`:420, DT-4 (a)(b)(c) at `:423`, DT-5 (a)(b)(c) at
`:424`, DT-6 (a)(b) at `:425`. Four winners, seven losers, and **each loser has a stated reason in
`_design.md` that is stronger than "we preferred the other one"**:

| Lost | Why, in the design's own terms |
| --- | --- |
| DT-1 (b) edge story first | Narrows the perceived audience to a runtime most evaluators are not on, and it is a differentiator only *after* the contract claim is accepted. It costs nothing to demote: the constrained-runtime reader self-identifies from **one line** in Guarantees; the general evaluator cannot self-identify from an edge lead at all. **The asymmetry decides it** (`_design.md`:202-206) |
| DT-1 (c) two entry points | **Lost twice.** Structurally: crates.io renders exactly one README per crate and there is no route parameter, so "two entry points" means two crates' front pages — and ADR-0006's split is by **role**, not by runtime. And on the persona question it splits one person in half (`_design.md`:208-212) |
| DT-4 (b) specification only | Loses on IQ-1's budget — 0 hops to read a claim. A ~5,000-line specification is a context jump the one-sitting reader does not return from, and U3 is the beat all four personas share with nowhere to look (`_design.md`:258-261) |
| DT-4 (a) landing page only | A promise with no citation is exactly what two DCB-labelled stores already disagreeing in public can each write. **The citation is the difference**, and it is why BR-14 exists (`_design.md`:263-265) |
| DT-5 (a) publish the ledger | Density: 200 rows on the one screen an evaluator reads buries every other claim — twenty-five times the height of the status table, on a surface whose budget is ~14 rendered lines above the fold at 1024×768 (`_design.md`:298-299) |
| DT-5 (b) frozen guarantees only | **Forbidden, not merely worse.** Listing 139 frozen guarantees while suppressing the existence of 49 provisional ones is IQ-2's filter hiding what it filters, and the exact dishonesty BR-06 exists against (`_design.md`:301-303) |
| DT-6 (b) let the proof stand alone | Stronger than "silence reads as an oversight": the Prior art section already exists and already names both peers honestly, including the sentence that sends a reader away. Choosing (b) means **deleting existing honest copy** in a release whose thesis is that a claim ships next to what proves it (`_design.md`:328-332) |

**3. The persona question is resolved, and then amended — the atom carries the amended form.** DT-1
could not be decided without it, so `_design.md`:214-220 resolves it: **the evaluator is the first
fifteen minutes of the application author's journey, not a fifth persona**, because every property
that distinguishes Persona 4 — time-boxed, one-shot, cannot run the suite — is a property of a
*moment*, not of a person. The owner then amended the *consequence* at the spec gate
(`_design.md`:222-243): the evaluation path is promoted as **its own journey atom linked to Persona
1**, not as a journey *stage* buried inside Persona 1's journey and not as a fourth persona atom.
The reason is `closeout-and-durable-audience`'s DR-10 argument 1, which DT-1 never rebutted — the
difference is in the *mechanism* of trust-building, and a moment with its own mechanism is exactly
what a journey atom is for. The same disposition is recorded on the other side
(`../closeout-and-durable-audience/_decomposition.md`:148-179). **An atom transcribed from
`:214-220` alone would carry a superseded consequence**, which is why this is its own criterion
rather than a footnote.

**4. One atom, not four — and this spec settles it** (`discover.md`:34 deferred it here). The four
resolutions are one commitment: *what the crate says on first contact*. They interlock rather than
sit side by side — DT-1's option (c) is rejected partly on the persona call, DT-5's census sentence
is sited relative to DT-1's first screen, DT-4's promise lives in the Guarantees block that DT-1's
loser was demoted into, and DT-6's reduced form exists **because** that first screen is full. Four
atoms would each cite the other three for their own reasoning, which is precisely the near-duplicate
shape the ingest adjudication is biased against (`.kb/_intake/README.md`:8-11). It also matches the
storymap's own singular phrasing (`_storymap.md`:59) and its per-story accounting: `release-decisions`
produces **one atom per story settled**, four atoms in one wave (`:73-78`).

**5. What belongs in the atom, and what stays in `_design.md`** (`discover.md`:35 deferred this
here too). The atom is a **decision record**: what was decided, what lost, and why. The composition,
density budget, transience policy, hierarchy and AP-1…AP-15 anti-patterns stay in `_design.md` and
are cited by path, never copied — duplicating them creates the very *two copies drift* failure DT-4's
own resolution is engineered against (`_design.md`:268-270), and `.kb/decisions/README.md`:35-47 is
explicit that a decision atom holds neither the evidence nor the current truth. The corollary that
bites hardest: **the atom does not restate the four maturity counts.** It records that the census
sentence exists, is dated, defines its four words inline and takes its numbers from
`spec/SPECIFICATION.md`:219-222 via the AC-003 clause audit at the publish commit. An atom carrying
`139 / 49 / 10 / 2` becomes a second place those numbers live, which is AP-3's failure moved off the
page and into the corpus.

**6. The atom must not overstate, and must not resolve an open question in passing.** DT-4's
resolution carries a nested absence-with-a-reason: the copy must not imply ownership of
`read_from_a_gap_position`, which `.kb/open-questions/es-38-and-gap-read-rules-are-unowned.md`
records as named by two accepted decisions and owned by neither (`_design.md`:276-280). The atom
carries that qualification and links the open question as `related` —
`kb-open-question-es-38-and-gap-read-unowned-001` stays **open**. The project's own risk register
names *"an open question resolved in passing by the release rather than by decision"* as a
medium/high risk, and the open-questions README requires a question be resolved, never deleted
(`project.md`:351).

**7. The atom is ingested, never hand-written, and `/redkiln:kb-ingest` is a human handoff.** Atoms
are authored by the ingest wave from `.kb/_intake/`; hand-authoring produces the directory layout of
the process without the process and was reverted once (`0269720`, `CLAUDE.md`). The implementer's
deliverable is a **staged intake document** plus the handoff — the redkiln SDLC commands are
user-invoked and cannot be called from the implementing context. Two operational facts: the ingest
glob picks up `.kb/_intake/README.md`, which is dropped at the approval gate rather than ingested;
and a successful wave **clears** `_intake/` — a file still sitting there afterwards is a file that
run did not ingest (`.kb/_intake/README.md`:13-19).

**8. The number is ≥ 0030, and the wave is shared.** `.kb/decisions/` holds 0001–0016 and 0029;
0017–0028 are allocated to sibling projects (`_decomposition.md`:115-121), so this project's atoms
take the next free numbers above the corpus (`project.md`:207-212). `crate-set-decision` is first in
merge order and states the whole `release-decisions` allocation in its intake document so one wave
cannot collide with itself; this story requests its number **within that allocation** and treats the
id the wave actually assigns as authoritative.

**9. Nothing accepted is edited and nothing is superseded.** `redkiln validate --kb` checks every
`status: accepted` decision atom against `HEAD` (`.kb/decisions/README.md`:9-13). Nothing in `.kb/`
resolves any DT id — grepped, no hits (`_grounding.md`:153-158) — so there is nothing to merge into
and nothing to supersede. The neighbours are `related` edges, not `supersedes` edges:
`kb-decision-0006` (which crate holds the bare name — the role split DT-1 (c) collides with) and
`kb-open-question-es-38-and-gap-read-unowned-001` (the non-ownership DT-4 must not overstate).

**10. The persona on the other side never reads this atom, and meets it anyway.** Backbone activity
A1 has no user intent of its own; every story in it is a foundation consumed by A3 and A4
(`_storymap.md`:45-50). The evaluator — one sitting, time-boxed, cannot run the suite
(`../_discovery/distillation/personas-and-journeys.md`:249-313, `:333-338`) — meets this decision as
the *shape of the first screen they land on*. The named consumers are all three
`published-surface-copy` stories: DT-1 the lead claim, DT-4 the gaps site, DT-5 the maturity
treatment, DT-6 the peer statement (`_storymap.md`:107).

## Integration contract

| Field | Value |
| --- | --- |
| **Archetype** | `foundation` — real in-tree substrate (a decision atom of record) consumed inside this project by all three `published-surface-copy` stories rather than left as an unproven island (`_storymap.md`:100-108) |
| **Slice / milestone** | `release-decisions`. Slice-mates, implemented in one context and mounted as one integrated ingest wave: `crate-set-decision`, `projection-port-ship-shape`, `msrv-promise-atom` |
| **Mount point** | `.kb/maps/decision-map.md` — the corpus's composition root. The new atom gets a row there in ADR-number order under the wave's own `##` section, with no superseded row deleted (`:81-86`). This is the real mount and not a formality: an atom with no map row is an atom the corpus cannot navigate to, which for a decision whose whole purpose is to outlive `.bklg/` is the same as not landing it. The atom itself lands at `.kb/decisions/00NN-<slug>.md`, written by the wave from `.kb/_intake/` — never by hand |
| **Wires into** | `.bklg/…/publication-and-positioning/_design.md` (`:191-345`) as the **source text** the atom transcribes and cites, and the file this story must not contradict; `.kb/_intake/` as the ingest inlet (`README.md`:13-19); `.kb/decisions/README.md`:9-13 (immutability) and `:31-33` (*state the alternatives that lost*) as the shape contract; `.kb/decisions/0029-msrv-raised-to-1-97-1.md`:1-33 as the frontmatter shape of record; `.kb/open-questions/es-38-and-gap-read-rules-are-unowned.md` and `.kb/decisions/0006-bare-name-to-the-typed-layer.md` as `related` edges; `.redkiln/config.yaml`:40's story-grain `affected_gate` |
| **Renders surfaces** | **none.** `_design.md`'s `## Items` (`:49-79`) carries no item this story adds and `## Surfaces` (`:104-176`) names no surface it renders. The honest coupling is one level down and is stated rather than hidden: this atom is the **recorded reason** the seven surfaces are composed the way `_design.md` composes them, and the three stories that do render them (`landing-copy-and-status-truth`, `compliance-claim-and-gaps-promise`, `guarantees-and-docs-rs-presentation`) read it rather than re-deriving it |
| **Conformance rule(s)** | **none, and it is not adapter-observable.** No rule in `crates/happenstance-testkit/src/suite.rs` can observe a positioning decision. The observing instruments are the testing brief's tier for AC-011 — a **static presence check plus content review** (`_decomposition.md`:474) — and `redkiln validate --kb`. Stated explicitly because a story that names no rule must say which of the two it is |
| **Clause(s)** | **none amended.** No `spec/SPECIFICATION.md` clause states a positioning decision, and nothing `[FROZEN]` is touched — AC-015 and DR-15 hold trivially and the diff is expected to prove it. One clause is *referenced* by the decisions being recorded: DT-4's single link is into a clause **ID**, not a heading, precisely so `cargo xtask spec-trace` guards it (`_design.md`:273-275) — siting that link is `compliance-claim-and-gaps-promise`'s, not this story's |
| **Advances DoD scenario** | Initiative **DoD 10** — *"the published crate looks finished"* (`initiative.md`:387-389) — by putting the four decisions that determine what "finished" says on the record before any copy is written. Supports **DoD 16** — *"the audience is durable"* (`:405-407`) — by recording the evaluator disposition in its amended form where `closeout-and-durable-audience`'s promotion reads it. At project grain it advances DoD 1 (AC-011, AC-013), DoD 4 (`validate --kb` / `doctor` clean) and DoD 6 (both traced ACs covered) |

## PR boundary

```
.kb/_intake/**
.kb/decisions/**
.kb/maps/decision-map.md
.kb/_governance/integration-waves/**
.redkiln/telemetry/**
.bklg/from-contract-to-published-library/publication-and-positioning/first-contact-design-resolutions/**
```

**In this PR**

- The staged intake document under `.kb/_intake/`, and everything the `/redkiln:kb-ingest` wave
  writes from it: the atom in `.kb/decisions/`, its row on `.kb/maps/decision-map.md`, the wave
  record under `.kb/_governance/integration-waves/`, the reciprocal backlinks, and the telemetry
  that wave commits.
- The committed fidelity check under this story's folder — the eleven-option roll-call against
  `_design.md`'s own text, plus the `redkiln validate --kb` / `redkiln doctor` transcript — and this
  story's `spec.md` and `_ledger.md`.

**Explicitly not in this PR**

- **Any edit to `_design.md`.** It is signed off with no conditions (`:772-789`) and its one
  amendment was made by the repository owner at the spec gate. An implementing story that edits a
  signed-off design has changed the thing it was measured against. The edge between the two runs the
  other way: the atom's `source_paths` cites `_design.md`, not the reverse.
- **Any published-surface copy, in any file.** Not `README.md`, not `crates/*/README.md`, not
  `crates/happenstance/src/lib.rs`. The lead claim, the census sentence, the gaps bullet and the peer
  sentence are `landing-copy-and-status-truth`'s, `compliance-claim-and-gaps-promise`'s and
  `guarantees-and-docs-rs-presentation`'s. This story decides what they say; it writes none of it.
- **Any `Cargo.toml` change**, including `crates/happenstance/Cargo.toml`'s missing
  `[package.metadata.docs.rs]` block and its `description` field — both are
  `guarantees-and-docs-rs-presentation`'s (`_design.md`:531, `:464`).
- **Resolving `kb-open-question-es-38-and-gap-read-unowned-001`**, or any other open question. It is
  linked as `related` and stays open; a resolution is its own decision with its own evidence.
- **Any `RUNBOOK.md` edit.** The slice's only runbook pointer is `crate-set-decision`'s, confined to
  `:4448-4460`; `falsifier-ledger-repair` owns `:622-635`. This story enters neither.
- **The three sibling atoms of this slice** — the crate set, PS-3's ship shape, the MSRV promise.
  This story requests a number inside the allocation `crate-set-decision` states and writes none of
  their content.
- **A long-form record under `references/adr/`.** That directory holds the seventeen imported ADRs
  whose line ranges `spec/SPECIFICATION.md` cites; nothing cites a long form for this decision, and
  `_design.md` is already the long form — the atom points at it.

**Merge DoD (one line).** One accepted decision atom at a number ≥ 0030 carries all four winners in
their decided *shape*, all seven rejected options with the reason `_design.md` gives each, and the
persona call in its **amended** form; `.kb/maps/decision-map.md` has its row; `redkiln validate --kb`
and `redkiln doctor` are clean; and the diff touches no published surface, no manifest, no
`_design.md` and no open-question atom.

## Behavior and interfaces

**There is no Rust interface change and no rendered surface.** `_design.md`'s `## Items` block
(`:49-79`) carries nothing this story owns, and `project.md`'s *Out of scope* reserves any API
surface change for an upstream project and a re-plan. The interfaces this story touches are the
knowledge base's (`KbFrontmatter`, the decision-atom shape, the map) and the backlog's (the
design-stage artifact it reads and must not contradict).

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **B1 — the design-side resolutions are proven present, not assumed** | Before anything is transcribed, each of DT-1, DT-4, DT-5, DT-6 is confirmed to carry a written resolution in `_design.md` naming the option that lost and why, and each ends in a `*Resolves:*` line binding it to its project AC. This is the testing brief's own tier for AC-011 — a static presence check plus content review, not a compiled test | `_design.md`:191-245, `:247-282`, `:284-318`, `:320-345`, and the `*Resolves:*` lines at `:245`, `:282`, `:318`, `:345`; `_decomposition.md`:474 |
| **B2 — the winners are transcribed with their shape, not their headline** | The atom states each winner as decided: DT-1 (a) *with the proof as an act the reader can perform*; DT-4 (c) *in the specified shape* — full statement, caller's register, Guarantees block, ≤ 3 lines + 1 nested, exactly one link into a clause ID; DT-5 (c) *as a single dated census sentence defining all four words inline*, sited after the status callout; DT-6 (a) *in the existing Prior art slot plus a one-sentence reduced form*, not a new section | `_design.md`:196-200, `:251-256`, `:288-296`, `:324-326` |
| **B3 — the eleven options are accounted for and the seven losers keep their reasons** | The atom carries a rejected-options section per tension covering every option in `initiative.md`'s DT table: 4 winners + 7 losers, each loser with the reason at the design's own strength — DT-1 (b) the **asymmetry**, DT-1 (c) **lost twice** (one README per crate; the role split; splitting one person in half), DT-4 (b) the **0-hop budget**, DT-4 (a) **the citation is the difference**, DT-5 (a) **200 rows against a 14-line first screen**, DT-5 (b) **forbidden, not merely worse**, DT-6 (b) **it would delete existing honest copy** | `initiative.md`:420, `:423`, `:424`, `:425`; `_design.md`:202-212, `:258-265`, `:298-303`, `:328-332` |
| **B4 — the persona call is carried in its amended form** | The atom states: the evaluator is a *moment* in the application author's journey, not a fifth persona (the surviving conclusion), **and** the consequence as amended — promoted as its own **journey** atom linked to Persona 1, not a stage buried inside Persona 1's journey and not a fourth persona atom — naming the mechanism argument that carried it and the circularity argument that was discounted. Both sides of the record are cited | `_design.md`:214-220 and the rider `:222-243`; `../closeout-and-durable-audience/_decomposition.md`:148-179; `_decomposition.md`:310-314 |
| **B5 — one atom, and the granularity is stated in it** | The atom covers the whole first-contact positioning decision and says why it is one record rather than four: the four resolutions interlock (DT-1 (c)'s rejection rests on the persona call; DT-5's siting on DT-1's first screen; DT-4's slot on DT-1's demotion; DT-6's reduced form on that screen being full), and four atoms citing each other is the near-duplicate shape ingest adjudication is biased against | `.kb/_intake/README.md`:8-11; `_storymap.md`:59, `:73-78` |
| **B6 — the atom records the decision, and cites rather than copies the design detail** | Composition, transience policy, density budget, hierarchy and AP-1…AP-15 stay in `_design.md` and are referenced by path and line. The atom carries no region table, no budget table and no anti-pattern list | `.kb/decisions/README.md`:35-47; `_design.md`:268-270 (the two-copies-drift argument being honoured); `_design.md`:347-405, `:439-498`, `:641-679` as the cited-not-copied regions |
| **B7 — no maturity counts are restated** | The atom records the census sentence's *form* — dated, version-scoped, four counts, four inline definitions, one link — and states that the numbers are taken from `spec/SPECIFICATION.md`:219-222 by the AC-003 clause audit at the publish commit. It writes no count of its own. A count in the atom is a second place those numbers live, which is AP-3 relocated from the page into the corpus | `_design.md`:310-316, `:650-652` (AP-3); `spec/SPECIFICATION.md`:219-222 |
| **B8 — no overstatement, and no open question closed in passing** | The atom carries DT-4's nested absence-with-a-reason verbatim in substance: the promise must not imply ownership of `read_from_a_gap_position`. The open-question atom is a `related` edge and keeps `status: accepted` as an open question — untouched, unresolved, and not deleted | `_design.md`:276-280; `.kb/open-questions/es-38-and-gap-read-rules-are-unowned.md`:1-24; `project.md`:351 |
| **B9 — the atom reaches `.kb/` through the ingest path only** | The implementer writes one staged document at `.kb/_intake/<nnnn>-first-contact-positioning.md` and then **hands off**: the human runs `/redkiln:kb-ingest`, whose wave authors the atom, syncs the maps, wires backlinks, runs `redkiln validate --kb`, clears `_intake/` and commits on its own branch. `.kb/_intake/README.md` is dropped at the approval gate, never ingested | `CLAUDE.md` (atoms are authored by `/redkiln:kb-ingest`, not by hand; `0269720`); `.kb/_intake/README.md`:3-5, `:13-19` |
| **B10 — the frontmatter is valid `KbFrontmatter` of the decision shape** | `kind: decision`, `authority_tier: decision`, `status: accepted`, `adr_id: ADR-00NN`, `phase: 12`, `reversibility` stated, `supersedes: null`, `superseded_by: null`, `related` carrying `kb-decision-0006` and `kb-open-question-es-38-and-gap-read-unowned-001`, `source_paths` naming the intake file and `_design.md` | `.kb/decisions/0029-msrv-raised-to-1-97-1.md`:1-33 as the shape of record; `.kb/decisions/README.md`:9-13 |
| **B11 — the number is free and the wave's allocation is respected** | The intake document requests the number allocated to this story inside the `release-decisions` allocation `crate-set-decision` states — lowest free ≥ 0030, avoiding 0017–0028 and 0029. The id the wave actually assigns is authoritative and is what the ledger cites | `project.md`:207-212; `_decomposition.md`:115-121; `_storymap.md`:73-78 |
| **B12 — the corpus can find it, and nothing accepted moved** | `.kb/maps/decision-map.md` gains one row in ADR-number order under the wave's section with no superseded row deleted; `.kb/decisions/0001`…`0016` and `0029` are byte-identical; `redkiln validate --kb` and `redkiln doctor` are clean, `doctor` reporting exactly the six expected `template-drift` advisories and no `dependency-cycle` | `.kb/maps/decision-map.md`:81-86; `.kb/decisions/README.md`:9-13; `project.md`:297-298 |
| **B13 — the downstream stories can act from the atom alone** | Each of the three `published-surface-copy` stories can read its own instruction out of the atom without opening `_design.md` to *decide* anything: which claim leads and in what form, which slot the gaps promise sits in and what it may not imply, what the census sentence must contain, and where the peer statement lives in full versus reduced. `_design.md` remains the single copy of *how it is laid out* | `_storymap.md`:64-66, `:107`; `story.md` frontmatter `blocks: [HS-S0092, HS-S0093]` |

## Data and migrations

**N/A — no schema, no migration, no backfill.** This story writes no code, touches no manifest and
defines no storage. The library's own storage schemas belong to each adapter, and nothing here
reaches them.

The only durable state this story writes is **knowledge-base state**, and it has a one-way
discipline rather than a migration: the ingest wave adds one atom, adds one decision-map row, wires
reciprocal backlinks, records the wave under `.kb/_governance/integration-waves/`, and clears
`.kb/_intake/`. There is no reverse migration for an accepted decision atom — the correction path is
a **new atom carrying `supersedes`**, with the old atom's frontmatter flipped to `status: superseded`
+ `superseded_by`, and its body never reworded (`.kb/decisions/README.md`:9-13). Rollback before the
wave merges is an ordinary branch discard; afterwards, changing a first-contact answer is a
superseding atom plus a re-plan that moves `_design.md`'s resolution and the copy stories that read
it together.

One asymmetry worth naming, because it is the reason the fidelity checks are front-loaded: the
*design* side is revisable by its owner at a sign-off gate, and the *atom* side is not. A
transcription error caught after the wave is corrected by a superseding atom that must itself name
what it supersedes and why — a permanent second record of a copying mistake.

## Acceptance criteria

Each criterion is written from the intent of the person on the other side of it, and there are three
of them. **The evaluator** — Persona 4's moment, one sitting, time-boxed, cannot run the suite
(`_discovery/distillation/personas-and-journeys.md`:249-313, `:333-338`) — never opens this atom and
meets it as *the shape of the first screen they land on*. **The application author** (Persona 1) is
the same human twenty minutes later; the persona call decided here determines where their journey is
recorded in `.kb/product/`, which is `closeout-and-durable-audience`'s to execute. The **direct
reader** is the next maintainer — the three `published-surface-copy` implementers this release, and
whoever writes a published sentence in the release after `.bklg/` has been archived. The failure all
three share is the same one: an answer that survives as a headline while the reason it beat the
alternative is gone.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **GIVEN** the repository owner signed this project's `_design.md` off on 2026-08-12 with **no conditions** (`_design.md`:772-789), **WHEN** the release needs project AC-011 (*"None is left unowned at release"*, `project.md`:266-269) to be an observed fact rather than an assumption, **THEN** each of DT-1, DT-4, DT-5 and DT-6 is confirmed to carry a written resolution in `_design.md` that names its rejected option(s) **and the reason each lost**, and each closes with a `*Resolves:*` line binding it to the project ACs it discharges (`:245`, `:282`, `:318`, `:345`); **AND** that confirmation is a **dated, committed** roll-call artefact under this story's folder — the testing brief's own tier for AC-011 is *static presence check plus content review, not a compiled test* (`publication-and-positioning/_decomposition.md`:474) — so DoD item 3's *committed artefact with a date, not a remembered observation* (`project.md`, Definition of done) is satisfied; **AND** `_design.md` itself is **byte-identical** afterwards, because a story that edits the artifact it was measured against has destroyed its own evidence | the committed roll-call cites `_design.md`:191-245, `:247-282`, `:284-318`, `:320-345` and the four `*Resolves:*` lines, and records the sign-off date and the amendment date; `git diff --quiet -- .bklg/from-contract-to-published-library/publication-and-positioning/_design.md` at HEAD |
| AC-002 | **GIVEN** those four answers live only under `.bklg/`, which is archived when the initiative closes, **WHEN** a maintainer two releases later asks *what does this crate say on first contact, and why*, **THEN** **one** accepted decision atom at a number ≥ 0030 (free of the 0017–0028 sibling allocation and of 0029) carries all four winners **in their decided shape, not as headlines** — DT-1 (a) storage-agnostic proof *stated as an act the reader can perform rather than as an adjective*; DT-4 (c) both places, the promise stated **in full and in the caller's register** inside the Guarantees block, as one bullet of **≤ 3 rendered lines plus one nested line** carrying **exactly one** link **into a clause ID rather than a heading**; DT-5 (c) a **single dated census sentence of ≤ 3 rendered lines with exactly 4 counts, 4 inline definitions and 1 link**, sited immediately after the status callout; DT-6 (a) stated **in the existing Prior art slot** in full plus a **one-sentence, ≤ 2-rendered-line, one-link** reduced form on the packaged README, and **never as a feature matrix** — **AND** the atom states *why it is one record and not four* (the four interlock: DT-1 (c)'s rejection rests on the persona call, DT-5's siting on DT-1's first screen, DT-4's slot on DT-1's demotion, DT-6's reduced form on that screen being full); **AND** it is valid `KbFrontmatter` in the decision shape of record (`.kb/decisions/0029-msrv-raised-to-1-97-1.md`:1-33): `kind: decision`, `authority_tier: decision`, `status: accepted`, `adr_id`, `phase: 12`, a stated `reversibility`, `supersedes: null`, `superseded_by: null`, `related` carrying `kb-decision-0006` and `kb-open-question-es-38-and-gap-read-unowned-001`, and `source_paths` naming the intake file and `_design.md` | `redkiln validate --kb` exits 0 with the new atom present; a content check that the atom names all four winners with the shape clauses above and carries the one-record rationale; `git ls-files .kb/decisions/` shows exactly one added path whose number is ≥ 0030 and outside 0017–0028 |
| AC-003 | **GIVEN** the named wrong implementation is *an atom that states four winners, validates cleanly, is correctly numbered and quietly drops the what-lost-and-why half* (`discover.md`:43), **WHEN** the next maintainer reads the atom's rejected-options sections, **THEN** all **eleven** options the initiative's DT table put on the table (`initiative.md`:420, `:423`, `:424`, `:425`) are accounted for — 4 winners and **7 losers** — and each loser carries its reason **at the strength `_design.md` states it**, not a shorter paraphrase: DT-1 (b) lost on the **asymmetry** (the edge reader self-identifies from one Guarantees line; the general evaluator cannot self-identify from an edge lead at all); DT-1 (c) **lost twice** (crates.io renders one README per crate and there is no route parameter; ADR-0006's split is by *role*, not runtime; and it splits one person in half); DT-4 (b) lost on **IQ-1's 0-hop budget** — a ~5,000-line specification is a context jump the one-sitting reader does not return from; DT-4 (a) lost because **the citation is the difference**, since a promise with no citation is what two DCB-labelled stores already disagreeing in public can each write; DT-5 (a) lost on **density: 200 rows against a ~14-rendered-line first screen at 1024×768**, twenty-five times the height of the status table; DT-5 (b) is **forbidden, not merely worse** — listing 139 frozen guarantees while suppressing 49 provisional ones is IQ-2's filter hiding what it filters and the exact dishonesty BR-06 exists against (AP-4); DT-6 (b) lost because choosing it would mean **deleting existing honest copy**, including the sentence that sends a reader away | an option roll-call reconciling 4 + 7 = 11 against `initiative.md`:420/423/424/425; a content check that each of the seven loser entries carries its distinguishing reason token (*asymmetry*; *one README per crate* + *role*; *0 hops* / *one sitting*; *citation*; *200* + *14*; *forbidden* / *filter hiding what it filters*; *deleting existing honest copy*), each traceable to `_design.md`:202-212, `:258-265`, `:298-303`, `:328-332` |
| AC-004 | **GIVEN** `_decomposition.md`:310-314 flagged the evaluator-vs-application-author question as open and DT-1 could not be decided without it, **AND** the owner **amended the consequence in place** at the spec gate (`_design.md`:222-243), **WHEN** `closeout-and-durable-audience` later populates `.kb/product/` and reads this atom for the disposition, **THEN** the atom carries the **amended** form and only it: the evaluator is a **moment** in the application author's journey and **not a fifth persona atom** (the surviving conclusion), **and** the consequence is that the evaluation path is promoted as **its own journey atom linked to Persona 1** — explicitly *not* a journey **stage** buried inside Persona 1's journey, and *not* a fourth persona atom; **AND** the atom names the argument that carried the amendment (the difference is in the **mechanism** of trust-building — one-shot public evidence versus revisable contact with the code over weeks — which is what a journey atom is for) and the argument that was **discounted** (that folding would retroactively make DT-1's option (c) incoherent — circular, because option (c) lost partly on that same premise); **AND** both sides of the record are cited, so a reader landing on either finds the other | a content check that the atom contains the *journey atom linked to Persona 1* consequence, the mechanism argument and the discounted circularity argument; a **negative** check that the superseded consequence wording (*journey stage on the application author's atom*) appears nowhere except as the thing amended; `file:line` resolution of `_design.md`:214-220, `:222-243` and `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_decomposition.md`:148-179 |
| AC-005 | **GIVEN** AP-3 forbids two maturity counts on one surface and any count not attached to a date and a version (`_design.md`:650-652), **AND** AP-13 forbids a positions-and-gaps statement implying the library owns `read_from_a_gap_position` (`:670-672`), **AND** the project's risk register names *"an open question resolved in passing by the release rather than by decision"* as medium/high (`project.md`:351), **WHEN** the atom is read, **THEN** it records the census sentence's **form** (dated, version-scoped, four counts, four inline definitions, one link) and **contains none of the four numbers itself**, stating instead that they are taken from `spec/SPECIFICATION.md`:219-222 by the AC-003 clause audit **at the publish commit** — a count in the atom is AP-3 relocated from the page into the corpus; **AND** it carries DT-4's nested absence-with-a-reason in substance (the promise must not imply ownership of `read_from_a_gap_position`), links `kb-open-question-es-38-and-gap-read-unowned-001` as `related`, and leaves that open-question atom **open and untouched**; **AND** it **cites rather than copies** `_design.md`'s composition (`:347-437`), density budget (`:439-498`), hierarchy (`:500-616`) and AP-1…AP-15 (`:641-679`) — no region table, no budget table and no anti-pattern list in the atom, because two copies of a budget is the *two copies drift* failure DT-4's own resolution is engineered against (`_design.md`:268-270) | a content check that the atom contains no maturity count and does contain both the non-ownership qualification and the `spec/SPECIFICATION.md`:219-222 provenance sentence; `git diff --quiet -- .kb/open-questions/` and `redkiln validate --kb` showing the open-question atom byte-identical to HEAD; a structural check that no table in the atom is copied from `_design.md` |
| AC-006 | **GIVEN** atoms are authored by `/redkiln:kb-ingest` and never by hand (`CLAUDE.md`; reverted once at `0269720`), **AND** an atom with no map row is one the corpus cannot navigate to, **WHEN** the wave lands, **THEN** the atom reached `.kb/decisions/` from a staged document under `.kb/_intake/` through the ingest path — no file was written directly under `.kb/decisions/` by this story — with `.kb/_intake/` **cleared** afterwards and `.kb/_intake/README.md` dropped at the approval gate rather than ingested (`.kb/_intake/README.md`:13-19); **AND** `.kb/maps/decision-map.md` carries one row for it in ADR-number order under the wave's own `##` section with **no superseded row deleted** (`:81-86`); **AND** `.kb/decisions/0001`…`0016` and `0029` are byte-identical, `redkiln validate --kb` and `redkiln doctor` are clean with exactly six `template-drift` advisories and zero `dependency-cycle`, and the diff is bounded by the PR boundary — no published surface, no `Cargo.toml`, no `RUNBOOK.md`, no `spec/SPECIFICATION.md`, no `_design.md`; **AND** each of `landing-copy-and-status-truth` (HS-S0092), `compliance-claim-and-gaps-promise` (HS-S0093) and `guarantees-and-docs-rs-presentation` (HS-S0094) can read its own instruction out of the atom alone — which claim leads and in what form, which slot the gaps promise sits in and what it may not imply, what the census sentence must contain, and where the peer statement lives in full versus reduced — without re-deciding anything | `redkiln validate --kb && redkiln doctor` exit 0 with the six-advisory / zero-cycle shape; `git ls-files .kb/_intake/` free of this story's document with `.kb/_intake/README.md` still present; a decision-map row-count assertion (rows added only); `git diff --name-only` against the merge base a subset of the PR boundary; a read-back answering all four downstream questions from the atom's text alone |

**Coverage of the traced project ACs.** Project **AC-011** (`project.md`:266-269 — *DT-1, DT-4, DT-5
and DT-6 each have a written resolution in this project's `_design.md`, each naming the option that
lost and why; none is left unowned at release*) is carried by **AC-001** (the resolutions exist, are
proven present and carry their `*Resolves:*` bindings), **AC-003** (the *"and why"* half survives
transcription, with each of the seven losers' reasons intact) and **AC-004** (the persona question
DT-1 turns on is not left unowned either). Project **AC-013** (`project.md`:273-276 — *each states the
alternatives that lost, was authored through the ingest path from `.kb/_intake/`, and takes a number
free of the 0017–0028 allocation and of 0029*) is carried by **AC-002** (existence, number and
frontmatter shape), **AC-003** (the alternatives that lost) and **AC-006** (the ingest path, the map
row, and `validate --kb`).

## Interaction quality

This story **renders no surface**. `_design.md`'s `## Items` (`:49-79`) carries no item it adds and
`## Surfaces` (`:104-176`) names no surface it renders — the Integration contract says so, and that is
the honest reading rather than an escape hatch. The composition family therefore does not apply *as
authorship*; it applies as **constraints this decision hands forward to a design that is already
signed off**, and every one of those constraints is written into an `AC-###` row above rather than
left as a bullet here.

The state family does apply, because the UX brief states IQ-1…IQ-7 for exactly this medium — *"the
reader's 'interaction' is navigating a claim to its evidence and back inside one sitting"*
(`publication-and-positioning/_decomposition.md`:225-310). Here the navigator is the next maintainer
moving from the atom to the design to the specification, and the evaluator downstream of them.

**Nothing is asserted in this section that is not gated by a row in the table above.** This section
says which row carries which invariant, and how each is verified.

### State invariants

| Invariant (UX brief, `_decomposition.md`:225-310) | Carried by | How it is verified |
| --- | --- | --- |
| **IQ-1 — in place, not a context jump.** The decision *and* the reason each alternative lost are readable **in the atom**; `_design.md`, `initiative.md` and the sibling brief are the ≤ 1 hop, and every hop lands on a line range, never on a document root or a bare *"see the design"* | AC-002, AC-003, AC-004 | the content checks run against the atom's own text, not against what it links; every `file:line` in its rejected-options and persona sections resolves at HEAD |
| **IQ-2 — non-occlusion: a filter must not hide what it filters.** The seven rejected options stay visible **with their reasons** instead of being compressed out; DT-5 (b) — the archetypal occlusion — is recorded as *forbidden*, not as *disfavoured*, so the atom cannot be read as leaving it available | AC-003, AC-005 | the roll-call reconciles 4 + 7 = 11; the loser-reason token check includes *forbidden* / *filter hiding what it filters*; the open-question atom is left standing rather than absorbed |
| **IQ-3 — preserved position: inbound references survive.** `.kb/maps/decision-map.md` gains a row and deletes none, including superseded rows; no accepted atom is renamed, re-numbered or reworded, so every existing citation into the corpus still resolves | AC-006 | the decision-map row-count assertion (added only); `.kb/decisions/0001`…`0016` and `0029` byte-identical under `redkiln validate --kb` |
| **IQ-4 — reversibility bought before the act.** An accepted atom is irreversible in the way a publish is — the correction path is a superseding atom, never an edit (`.kb/decisions/README.md`:9-13). So the fidelity roll-call is produced and committed **before** the ingest handoff, not after it | AC-001, AC-006 | the roll-call artefact's commit predates the wave's commit; AC-001's evidence exists independently of the wave |
| **IQ-5 — reachable without running anything.** The atom's evidence is a set of committed `file:line` citations plus a committed dated roll-call — never *"re-read the design yourself"* | AC-001 | the roll-call is a file in the tree carrying the date, the four section ranges and the four `*Resolves:*` lines |
| **IQ-6 — no orphan vocabulary.** The atom defines the terms it uses where it uses them — *census sentence*, *reduced form*, *caller's register*, *clause ID* — and specifically records that DT-5's four maturity words are defined **inline in the published sentence**, since IQ-6 is why (c) beat a bare link | AC-002, AC-005 | the shape check for DT-5 includes the *4 inline definitions* requirement; the atom is read for any term used and not defined |
| **IQ-7 — truth at the commit.** Every sentence in the atom is true of the tree it describes, including its statement that `_design.md` is signed off with no conditions and amended once | AC-001, AC-006 | the roll-call records the sign-off and amendment dates read from `_design.md`:772-789 and `:222-243`; the diff is bounded and `_design.md` is unchanged |
| **Keyboard reachability, focus, scroll and selection preservation** | **not applicable — stated, not skipped** | there is no focusable surface: the artefacts are markdown atoms and a committed roll-call. The medium's real analogue is IQ-3 (does a reader keep their place in the corpus), gated by AC-006. *A skip is reported, never silent* — `.kb/decisions/0010-the-suite-must-prove-itself.md` binds that discipline in the testkit and it is honoured here |

### Composition invariants (from the signed-off `_design.md`)

This story authors none of them and must not contradict any. It **carries four forward as decided
shape**, and each is a criterion rather than a bullet:

- **Presentation exists at all.** In this medium that means the atom is a *composed* decision record
  in the shape of record (`.kb/decisions/0029-msrv-raised-to-1-97-1.md`:1-33) — real `KbFrontmatter`,
  a supersession graph, a `related` edge set, and a **row on `.kb/maps/decision-map.md`** — not prose
  in a file with a heading. **AC-002** and **AC-006** carry it. An atom with no map row passes every
  frontmatter check and is still invisible, which is this medium's exact analogue of an unstyled
  render satisfying every ARIA assertion.
- **Density budget, with its real numbers.** The atom must carry DT-4's and DT-5's shapes *as
  budgets*, because the budgets are what the winners actually won as: the census sentence at **≤ 3
  rendered lines, exactly 4 counts, 4 inline definitions, 1 link** (`_design.md`:468); the
  positions-and-gaps bullet at **≤ 3 lines + 1 nested line** (`:471`) inside a Guarantees list capped
  at **≤ 7 bullets, each ≤ 3 rendered lines, exactly 1 link per bullet** (`:470`); the packaged peer
  statement at **1 sentence, ≤ 2 rendered lines, 1 link** (`:474`) — all of it against the binding
  **~14 rendered lines** above the fold at 1024×768 (`:451-458`), which the sign-off records as
  **full at 0.2.0** (`:777-784`). **AC-002** carries these. An atom that says *"state the promise in
  the README"* without the budget hands the copy story a decision it was supposed to receive.
- **Transience policy.** DT-4's promise is *revealed on scroll* inside Guarantees with the
  non-ownership line *nested under the promise it qualifies*; DT-5's census is *revealed on scroll* at
  1024×768 and *persistent chrome* at 1440×900; DT-6's peer statement is *opened on demand* from the
  packaged surfaces and *revealed on scroll* on GitHub (`_design.md`:407-437). The atom records the
  **siting** these imply — immediately after the status callout, inside Guarantees, in the existing
  Prior art slot — and cites the policy table for the rest. **AC-002** carries the siting; **AC-005**
  forbids copying the table.
- **Hierarchy.** DT-1's winner is a statement about what ranks first on the first screen. The atom
  states it as *the lead claim* and cites `_design.md`'s composition and hierarchy sections
  (`:347-437`, `:500-616`) rather than restating the region order. **AC-002** and **AC-005**.

**Named anti-patterns.** AP-1…AP-15 (`_design.md`:641-679) attach to rendered pages this story does
not author, so most are not assertable here — stated rather than assumed. **Four are assertable,
because they are the content of the decisions being recorded**, and each sits inside a criterion:
**AP-3** (no maturity count twice, and none without a date and version) is why **AC-005** forbids
counts in the atom; **AP-4** (frozen guarantees visible with no mention of the 49 provisional) is
DT-5 (b)'s *forbidden, not merely worse* in **AC-003**; **AP-5** (never a feature matrix naming a
peer) is part of DT-6's winning shape in **AC-002**; **AP-13** (no implied ownership of
`read_from_a_gap_position`) is DT-4's nested qualification in **AC-005**. The rest — **AP-1**'s
first-screen test above all — belong to `landing-copy-and-status-truth` and its two slice-mates, and
this story's PR boundary forecloses pre-empting them.

## Error conditions

| id | condition | required behaviour |
| --- | --- | --- |
| **EC-001** | The AC-001 roll-call finds a DT section that names a winner but **no loser**, or a `*Resolves:*` line missing | **Stop, and do not repair it here.** Writing the missing reason into `_design.md` would edit a signed-off design and would make the story's own evidence circular — the fix is an owner amendment at a gate, in the rider form `_design.md`:222-243 already demonstrates. Record the gap verbatim in the roll-call, mark AC-001 unmet, and escalate; project AC-011 is then genuinely not satisfied by-existing and the story's shape changes |
| **EC-002** | The draft atom and `_design.md` disagree — a shape clause, a rejected option's reason, or the persona consequence | **The design wins, always.** It is the signed-off source text and this story is transcription. Correct the atom, never `_design.md`. If the design looks wrong, that is an owner amendment, not an implementer's edit, and it is out of this story's scope |
| **EC-003** | The atom draft would need to state a maturity count to be readable (e.g. the census sentence's example) | Write the **form** and the provenance sentence instead (`spec/SPECIFICATION.md`:219-222, taken by the AC-003 clause audit at the publish commit). An illustrative example carrying `139 / 49 / 10 / 2` is still a second place those numbers live — AP-3 does not exempt examples |
| **EC-004** | The `/redkiln:kb-ingest` wave assigns an ADR number that collides with a slice-mate's atom (`crate-set-decision`, `projection-port-ship-shape`, `msrv-promise-atom`) | The intake documents state the whole `release-decisions` allocation up front (`_storymap.md`:73-78) precisely so one wave cannot collide with itself. If it happens anyway, the **assigned** id is authoritative; every citation — this story's `_ledger.md` above all — is updated to it, and no atom is renamed after acceptance |
| **EC-005** | After the wave, a file is still sitting in `.kb/_intake/`, or `.kb/_intake/README.md` was itself ingested | A file left behind is a file that run did not ingest (`.kb/_intake/README.md`:13-19); the README is dropped at the wave's approval gate, never ingested. Neither is repaired by hand-editing `.kb/` — re-run the wave over the remaining file |
| **EC-006** | `redkiln validate --kb` reports a diff against an accepted atom | An accepted body was edited, which is exactly what the check exists to catch (`.kb/decisions/README.md`:9-13). Revert it. If content genuinely needs correcting, that is a **new atom carrying `supersedes`**, and it is out of this story's scope |
| **EC-007** | The ingest adjudication proposes **MERGE** into an existing atom — most plausibly `0006-bare-name-to-the-typed-layer.md`, because DT-1 (c)'s rejection cites it | **Decline the merge.** 0006 is accepted and immutable; merging would edit an accepted body. The relationship is `related`, not `supersedes` and not `merge` — 0006 decided *which crate holds the bare name*, this decides *what the crates say on first contact*. `validate --kb` will catch a malformed edge but not a semantically wrong one |
| **EC-008** | Writing the atom's DT-4 section makes it tempting to state whether `read_from_a_gap_position` is owned | **Stop.** `kb-open-question-es-38-and-gap-read-unowned-001` stays open; it is a `related` edge and nothing more. Resolving it in passing is the project's own named medium/high risk (`project.md`:351), and the open-questions README requires a question be *resolved*, never deleted |
| **EC-009** | `redkiln doctor` reports a seventh `template-drift` advisory, or any `dependency-cycle` | Stop: a seventh is a template changed without a decision, and this story changes no template. **Never run `redkiln adopt --templates`** — `CLAUDE.md` forbids it, and it would overwrite all six deliberate customisations and then fail the backlog CI assertion on the absence it created |

## Non-functional

| id | requirement | why, and how it is observed |
| --- | --- | --- |
| **NF-001** | **Zero Rust, zero manifest, zero published-surface change.** No `.rs`, no `Cargo.toml`, no `Cargo.lock`, no `README.md` at any level, no `RUNBOOK.md`, no `spec/SPECIFICATION.md` in the diff | This story decides what the published sentences say and writes none of them; a diff that touches a README has taken a slice-mate's or a downstream story's work and broken the merge order the plan computed. Observed by `git diff --name-only` against the merge base being a subset of the PR boundary |
| **NF-002** | **`_design.md` is byte-identical.** The signed-off design is read-only to this story | It is the artifact this story is measured against and its one amendment was made by the owner at a gate. Observed by `git diff --quiet` on that single path, and it is a clause of AC-001 rather than only a policy |
| **NF-003** | **The story-grain gate is green, and is not trivially green.** `cargo xtask affected --base main` (`.redkiln/config.yaml`:40) passes, and it is meaningful because it runs the five file-reading lints and `spec-trace` unconditionally even when the diff touches no package | A story whose whole deliverable is markdown must still be gated on something; that unconditional half of `affected` is exactly why it exists (`publication-and-positioning/_decomposition.md`, *Merge-gate commands*) |
| **NF-004** | **Durability is one-way and stated.** After acceptance, a first-contact answer changes only by a superseding atom plus a re-plan that moves `_design.md`'s resolution and the copy stories that read it together | `.kb/decisions/README.md`:9-13; already written into *Data and migrations*, and re-checked by `validate --kb` at every later checkpoint |
| **NF-005** | **The fidelity artefact is self-dating and self-locating.** The roll-call carries the commit sha, the ISO date, and the `file:line` range of every claim it checks | DoD item 3 requires committed artefacts *with a date*, not remembered observations (`project.md`, Definition of done). A roll-call without its commit proves nothing about *this* `_design.md` |
| **NF-006** | **Reproducible offline from a clean checkout.** Every check in this story is a read of files already in the tree plus `redkiln validate --kb` / `doctor` — no network, no registry, no toolchain beyond what `affected` needs | The evidence must survive being re-run by a reviewer who did not author it; nothing here depends on crates.io being reachable, unlike this project's later stories |
| **NF-007** | **The atom is readable in one sitting.** It stays at the corpus's atom grain — roughly 100 lines, the length `CLAUDE.md` states for a decision atom — with `_design.md` as the long form | An atom that reproduces `_design.md` is not an atom; it is a second copy of a 790-line design, and the corollary that makes it wrong is the same one AC-005 gates: two copies drift and one of them is immutable |

## Implementation notes (non-prescriptive)

**Order matters more than technique here, because one of the steps is irreversible and one of the
inputs is read-only.**

1. **Read the four DT sections end to end before writing anything** — `_design.md`:191-245, `:247-282`,
   `:284-318`, `:320-345` — including the rider at `:222-243`, which is easy to skim past because it
   is a blockquote inside DT-1 and it is the single most transcription-sensitive paragraph in the file.
2. **Build the roll-call first, and build it as a table.** Eleven rows, one per option, each with:
   the DT id, the option letter, won/lost, the reason in `_design.md`'s own words, and the `file:line`
   it came from. It is AC-001's and AC-003's shared evidence, and building it before the atom means
   the atom is written *from* it rather than checked against it afterwards.
3. **Reconcile the count out loud.** `initiative.md`:420/423/424/425 declares 3 + 3 + 3 + 2 = 11.
   If the roll-call has ten rows, an option was silently dropped — which is the story's named wrong
   implementation arriving early, and it is much cheaper to catch here than in the atom.
4. **Write one staged intake document**, not an atom, at
   `.kb/_intake/<nnnn>-first-contact-positioning.md`. Hand-authoring under `.kb/decisions/` is
   prohibited and was reverted once already (`0269720`, `CLAUDE.md`). Model the body and frontmatter on
   `.kb/decisions/0029-msrv-raised-to-1-97-1.md`, the most recent atom the ingest path produced.
5. **Request the number inside the slice's stated allocation.** `crate-set-decision` is first in merge
   order and its intake document states the whole `release-decisions` allocation; this story asks for
   its own number within it and treats whatever the wave assigns as authoritative (EC-004).
6. **Then stop and hand off.** `/redkiln:kb-ingest` is user-invoked and cannot be called from the
   implementing context. The wave authors the atom, syncs `.kb/maps/decision-map.md`, wires backlinks,
   runs `redkiln validate --kb`, clears `.kb/_intake/` and commits on its own branch.
7. **Fill the ledger last**, once the assigned id exists. Everything the story can produce alone — the
   intake document and the roll-call — is committed first, so the story is never blocked on the wave
   for its *evidence*, only for its *id*.

Three things worth naming so they are not discovered late. The `related` edges point at
`kb-decision-0006` and `kb-open-question-es-38-and-gap-read-unowned-001`; a `supersedes` edge to
either would be wrong, and `validate --kb` checks that an edge is well-formed, not that it is right.
Nothing in `.kb/` currently mentions any DT id (`_grounding.md`:153-158), so the ingest
adjudication's usual MERGE-over-new preference has nothing to prefer — this is a new atom, and
EC-007 says what to do if it proposes one anyway. And the atom is the only place the *interlock*
argument for one-record-not-four will ever be written down; if it is dropped, a later reader
reasonably splits the decision and loses exactly the cross-references that make DT-1 (c)'s rejection
legible.

## Tests and CI (merge gate)

Grounded in the testing brief's own mapping (`publication-and-positioning/_decomposition.md`,
*Test mix*): project **AC-011** is *static (presence check) — confirms `_design.md` exists and each of
DT-1, DT-4, DT-5, DT-6 has a written resolution naming the option that lost — a content review, not a
compiled test* (`:474`), rejecting *a `_design.md` that states a winner without stating what lost*;
project **AC-013** is *static (process) — `redkiln validate --kb` over every atom this project
authors, each from the `/redkiln:kb-ingest` path* (`:476`), rejecting *an atom hand-written directly
under `.kb/decisions/`*.

| tier | command / path | proves |
| --- | --- | --- |
| **static (content review)** | the committed roll-call under `.bklg/from-contract-to-published-library/publication-and-positioning/first-contact-design-resolutions/`, read against `_design.md`:191-345 | AC-001, AC-003: four resolutions present with their `*Resolves:*` lines, and 4 + 7 = 11 options accounted for with each loser's reason at the design's own strength. This is the exact tier the testing brief assigns to AC-011 — no compiled test can observe a positioning decision |
| **static (content review)** | a read of the accepted atom against AC-002's shape checklist and AC-004's amended-persona checklist, including the **negative** check for the superseded *journey stage* wording | AC-002, AC-004: the winners carry their shapes and budgets, the one-record rationale is present, and the persona consequence is the amended one rather than the one it replaced |
| **static (content review)** | a read of the accepted atom for maturity counts, the non-ownership qualification, and any table copied from `_design.md` | AC-005: no count, AP-13 honoured, and the design detail cited rather than duplicated |
| **static (citation resolution)** | `file:line` resolution at HEAD of every citation in the atom's rejected-options and persona sections | AC-003, AC-004: IQ-1's ≤ 1 hop lands on a range, not a document root; a citation that does not resolve is a hop the reader does not come back from |
| **static (process)** | `redkiln validate --kb` | AC-002, AC-005, AC-006: `KbFrontmatter` conformance on the new atom, every accepted atom byte-identical to `HEAD`, and the open-question atom untouched. This is the check that catches an edited `0001`…`0016`/`0029` and the check project AC-013 is scored on |
| **static (process)** | `redkiln doctor` | AC-006, EC-009: exactly six `template-drift` advisories and zero `dependency-cycle`, per `CLAUDE.md`'s backlog-CI assertion. Required at *this* checkpoint, not only at project end |
| **static (diff)** | `git diff --name-only <merge-base>..HEAD` asserted against the PR boundary; `git diff --quiet -- .bklg/…/publication-and-positioning/_design.md`; `git diff --quiet -- .kb/open-questions/ crates/ Cargo.toml RUNBOOK.md spec/` | NF-001, NF-002, AC-001, AC-005, AC-006: the design is unchanged, no open question moved, and no published surface, manifest or specification was touched |
| **static (map)** | a row-count and ordering read of `.kb/maps/decision-map.md` before and after the wave | AC-006: one row added in ADR-number order under the wave's section, nothing deleted — the mount point actually mounted rather than assumed |
| **story-grain gate (automatic)** | `cargo xtask affected --base main` (`.redkiln/config.yaml`:40) | NF-003: fmt/clippy/tests for anything the diff could reach, plus the five file-reading lints and `spec-trace` unconditionally, so a markdown-only story is still gated on something real |
| **not run here** | `cargo xtask ci` (full), the four mandatory `wasm32` steps, `event_store_conformance!` | Stated so the absence is a decision rather than an oversight: this project is not terminal (`.redkiln/config.yaml`:55 wires `cargo xtask ci --fast` at integration grain), the full gate belongs to `publish-0-2-0`'s AC-016, and **no rule in `crates/happenstance-testkit/src/suite.rs` can observe a positioning decision** — no `EventStore` fixture is exercised by this project at all (`publication-and-positioning/_decomposition.md`:507-511) |

## Risks and coupling (PR-scoped)

| risk | likelihood / impact | mitigation, in this PR |
| --- | --- | --- |
| **Transcription loss — the atom states four winners and drops what lost** (`discover.md`:43) | this is the whole risk the story exists against | AC-003 is a ledger row requiring an 11-option roll-call with each loser's distinguishing reason token; a `_ledger.md` row cannot be flipped on prose, and the roll-call is built *before* the atom so the atom is written from it |
| **The superseded persona consequence is copied.** `_design.md`:214-220 reads as a complete resolution; the rider that overrules its consequence is a blockquote twenty lines further down | medium / high. It would put a wrong instruction into an immutable atom, and `closeout-and-durable-audience` reads exactly this | AC-004 requires the amended form *and* carries a **negative** check for the *journey stage* wording; both sides of the record are cited (`_design.md`:222-243 and `closeout-and-durable-audience/_decomposition.md`:148-179), so a reader landing on either finds the other |
| **A maturity count leaks into the atom** — the census sentence is hard to describe without quoting it | medium / medium. AP-3 relocated from the page into the corpus, and the corpus copy cannot be edited when the numbers change | AC-005 forbids all four numbers and requires the provenance sentence instead; EC-003 closes the *"but it's only an example"* route explicitly |
| **An open question is resolved in passing** — DT-4's section is *about* an unowned behaviour | medium / high; it is the project's own named risk (`project.md`:351) | AC-005 requires the open-question atom byte-identical and the edge to be `related`; EC-008 states the stop rule; `git diff --quiet -- .kb/open-questions/` is a gate command |
| **Number collision inside a shared ingest wave** — four atoms, one wave | medium / low | `crate-set-decision` states the whole allocation and is first in merge order (`_storymap.md`:73-78); EC-004 makes the wave's assigned id authoritative and the ledger cites it rather than a guessed number |
| **Scope creep into published copy.** The atom has to *describe* sentences that are one keystroke from being written | medium / medium | An explicit PR-boundary exclusion naming `README.md`, `crates/*/README.md` and `crates/happenstance/src/lib.rs`; NF-001 asserts it on the diff; the composition section restates that AP-1 and the first-screen budget are `landing-copy-and-status-truth`'s to hold |
| **Editing `_design.md` "just to fix a typo"** | low / high. It is the artifact this story is measured against, and its one amendment was an owner act at a gate | NF-002 and AC-001 both assert byte-identity; EC-001 routes a genuine gap to an owner amendment rather than an implementer edit |
| **Two stories block on this and the atom's id does not exist until the wave runs** — HS-S0092 and HS-S0093 | high / low | It is a *reading* dependency, not a code one: both need the four answers stated, which AC-002 delivers, and AC-006 requires the atom to answer their four questions unaided. Everything the story can produce alone is committed before the handoff |
| **The wave lands on its own branch, outside this story's control** | medium / medium | Implementation note 7 sequences the ledger's evidence after the handoff; AC-001's roll-call is deliberately wave-independent so the story always has evidence for at least project AC-011 |
| **`guarantees-and-docs-rs-presentation` (HS-S0094) writes into the same Guarantees block DT-4 sites its promise in**, without a declared dependency edge on this story (`_storymap.md`, slices table) | medium / medium | Stated rather than hidden: this story writes no README text at all, so there is no file-level conflict. The coupling is semantic — the ≤ 7 bullet / 1 link-per-bullet budget (`_design.md`:470) has to absorb DT-4's bullet *and* the MSRV line *and* the `wasm32` line — and AC-002 records that budget in the atom precisely so the two copy stories are working from the same ceiling |

## Dependencies

**Blocks on:** *nothing* — `depends_on: []`, matching this story's row in `_storymap.md`:59 and
`story.md`'s `blocked_by: []`. It is a foundation, and it is first-equal in the `release-decisions`
slice: it produces a fact other stories read rather than a capability a user meets. `crate-set-decision`
is ahead of it in merge order and states the wave's number allocation, but that is a slice ordering
convention (EC-004), **not** a dependency — this story's content does not read the crate set.

**Unlocks** (by story slug, each with what it actually consumes):

| story slug | what it reads from this one |
| --- | --- |
| `landing-copy-and-status-truth` (HS-S0092) | DT-1's lead claim in its decided form, DT-5's maturity treatment (the census sentence's contents and siting) and DT-6's peer statement, full form and reduced (`_storymap.md`:107; `story.md` `blocks:`) |
| `compliance-claim-and-gaps-promise` (HS-S0093) | DT-4's site, register, budget, single clause-ID link, and the non-ownership qualification it may not overstate past (`_storymap.md`:107; `story.md` `blocks:`) |
| `guarantees-and-docs-rs-presentation` (HS-S0094) | not a declared edge, but it writes into the same Guarantees block DT-4 sites its promise in, and shares its ≤ 7-bullet / 1-link-per-bullet budget (`_design.md`:470). It reads the atom for the ceiling, not for permission |
| `closeout-and-durable-audience` (sibling project) | the persona disposition in its amended form — the evaluation path promoted as its own journey atom linked to Persona 1 — which is what it executes into `.kb/product/` (`closeout-and-durable-audience/_decomposition.md`:148-179) |

## Anchors (progressive disclosure)

Open these when the row's *when* says to, not before. Everything load-bearing enough to be an
obligation is already stated above; these carry the depth, the exact wording and the two sides of the
amended record.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `.bklg/from-contract-to-published-library/publication-and-positioning/_design.md` | **The source text.** DT-1 `:191-245` (with the owner's rider `:222-243`), DT-4 `:247-282`, DT-5 `:284-318`, DT-6 `:320-345`, the four `*Resolves:*` lines, the transience table `:407-437`, the per-item density budgets `:439-498` (census `:468`, Guarantees `:470`, gaps bullet `:471`, peer statement `:474`, the 14-line first screen `:451-458`), AP-1…AP-15 `:641-679`, and the sign-off `:772-789` | **First, before writing anything**, and again for every shape clause. Read the rider in full — it is a blockquote inside DT-1 and skimming it is how the superseded consequence gets transcribed | AC-001, AC-002, AC-003, AC-004, AC-005 |
| `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_decomposition.md` | The **other side** of the amended persona record: DR-10's disposition at `:148-179` states what survived (argument 1, the mechanism difference), what did not (that it makes the evaluator a different person) and that argument 3 was discounted as circular | When writing the atom's persona section, so both citations are present and a reader landing on either side finds the other | AC-004 |
| `.bklg/from-contract-to-published-library/initiative.md` | The DT table is the **authoritative option enumeration** the roll-call reconciles against: DT-1 `:420`, DT-4 `:423`, DT-5 `:424`, DT-6 `:425` — 3 + 3 + 3 + 2 = 11. Also BR-15, BR-17, DoD 10 `:387-389` and DoD 16 `:405-407` | Before building the roll-call, to fix the option count independently of `_design.md`'s prose | AC-003 |
| `.bklg/from-contract-to-published-library/publication-and-positioning/_decomposition.md` | The briefs. UX brief IQ-1…IQ-7 `:225-310` (the state invariants this section maps), AC-UX-001/003/004/005 `:313`, `:323`, `:330`, `:335`, and the explicit hand-off *"this brief does not decide DT-1, DT-4, DT-5 or DT-6"*; the testing brief's AC-011 row `:474` and AC-013 row `:476`, which fix the verification tier and name the wrong implementation | Open the testing brief rows before choosing how to evidence AC-001; open the IQ block when checking that a claim in the atom is readable without a hop | AC-001, AC-003 |
| `.bklg/from-contract-to-published-library/publication-and-positioning/first-contact-design-resolutions/discover.md` | `:43` states the named wrong implementation in full — the atom that validates, is numbered correctly and drops the *what lost* half — and `:16-29` is the signal ledger behind every claim in the context pack; `:34-35` are the two questions this spec was asked to settle | **First**, if you are tempted to treat the roll-call as a formality | AC-003 |
| `.kb/decisions/0029-msrv-raised-to-1-97-1.md` | The **shape of record** — the most recent atom the ingest path produced. `:1-33` carries the frontmatter fields AC-002 enumerates, and the body demonstrates how an amendment states what it does and does not reverse, which is the same move AC-004 needs | Before writing the intake document, as the template for its body and frontmatter | AC-002, AC-004 |
| `.kb/decisions/README.md` | `:9-13` is the immutability rule `validate --kb` enforces against `HEAD`; `:31-33` requires an atom to *state the alternatives that lost*; `:35-47` says a decision atom holds neither the evidence nor the current truth — the boundary AC-005 gates | Before the ingest handoff, and again if `validate --kb` reports anything | AC-002, AC-003, AC-005 |
| `.kb/_intake/README.md` | The ingest inlet's operating notes: `:8-11` the adjudication's bias against near-duplicate atoms (the one-record argument), `:13-19` that the wave clears `_intake/` and that a file left behind is a file that run did not ingest, and that the README is dropped at the approval gate | When staging the intake document, and when checking the wave landed cleanly | AC-002, AC-006 |
| `.kb/maps/decision-map.md` | **The mount point.** `:81-86` is *Adding a row* — ADR-number order, a new `##` section per wave, never delete a superseded row. An atom with no row here is invisible to the corpus | After the wave, when verifying the map sync rather than assuming it | AC-006 |
| `.kb/open-questions/es-38-and-gap-read-rules-are-unowned.md` | The question DT-4 must not answer in passing: named by two accepted decisions and owned by neither. It is the `related` edge, and it stays open | When writing the atom's DT-4 section, and again when asserting the open-questions tree is untouched | AC-005 |
| `.kb/decisions/0006-bare-name-to-the-typed-layer.md` | The nearest neighbour in the corpus — it split the crates by **role**, which is precisely the structural half of why DT-1 (c) lost. Adjacent, not superseded: the edge is `related` | When writing DT-1 (c)'s rejection and the atom's link graph | AC-003, EC-007 |
| `.bklg/from-contract-to-published-library/publication-and-positioning/project.md` | AC-011 `:266-269` and AC-013 `:273-276` verbatim, DR-11 `:202-204`, DR-13's numbering constraint `:207-212`, the risk register's *open question resolved in passing* row `:351`, and the boundary-level Definition of done | At the start, to confirm the two traced ACs, and at the end when filling the ledger | AC-001, AC-002, AC-005, AC-006 |
| `.bklg/from-contract-to-published-library/publication-and-positioning/_storymap.md` | `:59` this story's row; `:73-78` why `release-decisions` is one slice and **one ingest wave** producing one atom per story; `:107` the named consumers and what each takes; `:160` merge position | When stating the one-record rationale and the number allocation, and when checking the atom is quotable by its consumers | AC-002, AC-006 |
| `.bklg/from-contract-to-published-library/publication-and-positioning/crate-set-decision/spec.md` | The slice-mate that is first in merge order and whose intake document states the whole `release-decisions` ADR-number allocation; its EC-004 is the collision rule this story shares | When requesting a number, and if the wave assigns something unexpected | AC-002 |
| `.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md` | Persona 4 `:249-313`, the one-sitting constraint `:333-338`, U3's *nowhere to look* `:339-345`, Persona 4's thin evidence `:349-355` and the persona question itself `:373-377` — the human on the other side of every one of these four decisions, who never reads the atom | When writing the atom's framing sentence, and when the *why does a paperwork story have a persona* question comes up | AC-001, AC-004 |
| `spec/SPECIFICATION.md` | `:219-222` is where the four maturity counts actually live — the provenance AC-005 requires the atom to cite **instead of** restating; `:213-217` is the provisional-marker-without-a-falsifier clause DT-5 (b) offends | When writing DT-5's section, at the moment the numbers feel necessary | AC-005 |
| `.redkiln/config.yaml` | `:40` the story-grain `affected_gate` that fires automatically at this story's transition; `:55` the non-terminal integration gate this project runs instead of the full one | Before claiming the gate is green, to know which command redkiln will actually have run | NF-003 |

## Clarifications resolved during spec

1. **The AC set is exactly the six the first pass decided** — AC-001…AC-006 — with none added or
   dropped. Each of B1…B13 in *Behavior and interfaces* lands in one of them: B1 → AC-001;
   B2, B5, B10, B11 → AC-002; B3 → AC-003; B4 → AC-004; B6, B7, B8 → AC-005; B9, B12, B13 → AC-006.
2. **`discover.md`:34 — one atom or four — is settled as one**, and the reason is written into the
   atom itself rather than only into this spec (AC-002). The four resolutions interlock: DT-1 (c)'s
   rejection rests on the persona call, DT-5's siting on DT-1's first screen, DT-4's slot on DT-1's
   loser being demoted into Guarantees, and DT-6's reduced form on that first screen being full. Four
   atoms would each cite the other three for their own reasoning — the near-duplicate shape the
   ingest adjudication is biased against (`.kb/_intake/README.md`:8-11) — and it matches
   `_storymap.md`:59's singular phrasing and `:73-78`'s one-atom-per-story accounting.
3. **`discover.md`:35 — what belongs in the atom — is settled as *the decision, never the layout*.**
   Composition, transience, density and AP-1…AP-15 stay in `_design.md` and are cited by path and
   line. AC-005 gates the prohibition. The one deliberate exception is the **budgets attached to
   DT-4's and DT-5's winners**, which AC-002 requires *in* the atom, because those budgets are what
   the options actually won as — DT-5 (c) is not "publish a summary", it is "≤ 3 rendered lines,
   4 counts, 4 inline definitions, 1 link". Dropping the budget is the same failure as dropping the
   loser, one level down.
4. **AC-001 is a criterion rather than an assumption, even though project AC-011 is satisfied
   by-existing.** The temptation was to write *"AC-011 already holds"* and move on. It loses on DoD
   item 3: an unrecorded reading is a remembered observation, and the roll-call is also AC-003's
   input, so building it costs nothing extra and makes the by-existing claim checkable by a reviewer
   who was not there.
5. **The persona call was split out of AC-002 into its own criterion (AC-004)** because the risk is
   not that it is omitted but that the **superseded** version is transcribed — the rider overruling
   it sits twenty lines below a resolution that reads as complete. That needs a negative check, and a
   negative check inside a clause about four winners would not survive review.
6. **The composition family is constraint, not authorship — and it is not declared inapplicable.**
   Four composition invariants (presentation-exists-at-all, the density budget with its real numbers,
   the transience-implied siting, and hierarchy) are carried by AC-002, AC-005 and AC-006 rather than
   left as prose, because an atom that hands a copy story a headline without its budget has handed
   forward a decision the copy story was supposed to receive.
7. **The keyboard/focus/scroll/selection invariants are recorded as not applicable rather than
   silently omitted**, with the medium's real analogue named and gated instead (IQ-3, inbound
   citations surviving, at AC-006). That is the same *a skip is reported, never silent* discipline
   `.kb/decisions/0010-the-suite-must-prove-itself.md` binds in the testkit.
8. **The atom's ADR number is deliberately not fixed in this spec.** The intake document requests the
   number allocated inside `crate-set-decision`'s stated `release-decisions` allocation — lowest free
   ≥ 0030, avoiding 0017–0028 and 0029 — and the wave assigns the real id, which EC-004 makes
   authoritative. Writing a number here that the wave then declined would leave a citation nobody
   could resolve.
9. **`_design.md` is read-only to this story, and that is asserted on the diff rather than merely
   stated.** NF-002 and a clause of AC-001 both require byte-identity. The correction path for a
   genuine gap is an owner amendment at a gate (EC-001), in the rider form the file already
   demonstrates — not an implementer edit to the artifact the story is measured against.
