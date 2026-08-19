---
title: Vocabulary and conventions
kind: grounding/summary
---

# Vocabulary and conventions — From Contract to Published Library

Grounding pass for `HS-I0006`. Scope: the house vocabulary this initiative's
charter and downstream artifacts should read as native, and the real `.kb` /
code paths worth citing as Context anchors. No architecture, no design, no
"we should build" — vocabulary and pointers only.

**A note on the tasking's own paths.** The tasking pointed at
`.kb/03-reference/glossary/` and `.bklg/_archive/`. Neither exists in this
tree. The corpus's actual layout — confirmed against `CLAUDE.md`'s repository
map and a directory listing — is unnumbered: `.kb/reference/` (not
`03-reference`), no dedicated `glossary/` subfolder, and no `.bklg/_archive/`
yet (only two initiatives exist so far: `from-contract-to-published-library`
itself and `support`, both still at `stage: intake`, neither with a
`_archive` predecessor). There is also no `./scripts/kb-dotmd.sh` in this
repo. This document was built from `rg` over `.kb/` and `.bklg/`, plus direct
reads of the map and README atoms that stand in for a glossary — see
"Where the vocabulary lives" below.

## House vocabulary the charter should use

**Atom, kind, authority_tier.** The unit of the knowledge base is an "atom":
markdown with `KbFrontmatter` that `redkiln validate --kb` checks. Every atom
declares a `kind` (`decision`, `concept`, `playbook`, `reference`,
`open_question`, `map`) and an `authority_tier` (`decision`, `guideline`,
`note`, `product`). These six kinds are load-bearing distinctions, not loose
synonyms — each has a README stating what belongs and what doesn't:
`.kb/decisions/README.md`, `.kb/concepts/README.md`, `.kb/governance/README.md`,
`.kb/reference/README.md`, `.kb/open-questions/README.md`, `.kb/maps/README.md`.
A charter that calls a measurement a "decision" or a procedure a "concept" is
using the vocabulary wrong in a way the corpus itself would reject.

**Decision atom, and "supersede, never edit."** `.kb/decisions/` holds
`decision` atoms, each carrying `adr_id` (`ADR-0001` …), `reversibility`, and
the owning `phase`. **An accepted decision atom is immutable** — correcting one
means authoring a new atom with `supersedes: [<old id>]` and flipping the old
atom's `status` to `superseded`, never rewording its body
(`.kb/decisions/README.md:9-18`). The corpus also distinguishes a **repair**
from an **amendment**: mechanically, "a correction is a repair if the set of
implementations the decision admits is unchanged. Otherwise it is a gap, and a
gap is a new decision's" (`.kb/decisions/README.md:20-23`). ADR-0029 is the
worked example already in the tree — it *amends* ADR-0004 rather than editing
it (`CLAUDE.md`'s binding-constraints §5; `.kb/decisions/0029-msrv-raised-to-1-97-1.md`).

**Concept, playbook, reference — the other three durable kinds.** A
`concept` atom explains a mechanism assumed by more than one other atom and
commits to nothing (`.kb/concepts/README.md:8-14`). A `playbook` atom is a
transferable procedure — "what to do" (`.kb/concepts/README.md:26-27`). A
`reference` atom is a dated measurement, census, or pointer, and carries the
rule that "every atom here is a statement about a moment" — undated, it "reads
as current" and nothing detects that it has stopped being true
(`.kb/reference/README.md:19-23`). None of these three binds the project the
way a decision does.

**Open question, not a task.** `.kb/open-questions/` holds `open_question`
atoms recording a deliberately unsettled question — "what is not decided," not
a to-do. The KB "records what is known and what is not known, never what is
queued" (`.kb/open-questions/README.md:37-38`); a task belongs in `.bklg/`
instead. This initiative's own brief carries seven such open questions in a
list, and they should stay framed that way — as questions with a forcing
event and an owning phase, not as backlog items in disguise
(`.bklg/from-contract-to-published-library/_intake-brief.md:89-127`).

**Map atom.** `.kb/maps/` holds navigation only — "a map binds nothing and
asserts nothing new" (`.kb/maps/README.md:5-7`). Cite `decision-map.md` and
`domain-map.md`, don't restate their contents.

**Clause and its four maturity markers.** `spec/SPECIFICATION.md` uses
"clause" for its 200 numbered, ID'd requirements, each with exactly one marker:
`[FROZEN]` (settled; a new ADR to change, not an edit), `[PROVISIONAL —
<falsifier>]` (binding until a named artefact/measurement/deployment falsifies
it), `[DEFERRED — <experiment, owning phase>]` (deliberately open), and
`[NON-NORMATIVE]` (drafted as a clause, demoted to prose because no violating
implementation could be named) (`spec/SPECIFICATION.md:170-217`). A marker
"binds the decision. Whether it also binds the *release* is a separate
question, answered per port" — `EventStore` clauses are semver-binding at 0.1,
`ProjectionStore` clauses are design-binding only until PS-2's gate is cleared,
`SyncPeer` clauses bind design alone (`spec/SPECIFICATION.md:179-192`). The
charter's "Clauses" vocabulary (PS-\*, SY-\*, VT-\*, ES-\*, CF-\*, WF-\*) is
this same ID scheme and should be cited exactly, never paraphrased into a new
label.

**Falsifier.** The specific word the spec uses for what discharges a
`[PROVISIONAL]` marker: "a `[PROVISIONAL]` or `[DEFERRED]` marker with an
empty falsifier or experiment is forbidden" (`spec/SPECIFICATION.md:213-217`).
Prefer "falsifier" over looser words like "evidence" or "proof" when the
charter is talking about what would overturn a provisional clause specifically.

**Port, adapter, conformance suite, fixture, capability.** `happenstance-core`
defines "ports" (`EventStore`, `ProjectionStore`, and — as a port crate in its
own right — `SyncPeer`/`happenstance-sync`). A crate that implements a port is
an "adapter," and per the binding rule in `CLAUDE.md`, "an adapter that
compiles but has not run the conformance suite is not an adapter." The suite
is invoked as `happenstance_testkit::event_store_conformance!(MyFixture::new())`
— note it is called with a **`Fixture`**, not a store: "one fixture instance is
one isolated backing store; each `connect()` on it is one handle onto that
store" (`CLAUDE.md`, "The rule that matters"). A fixture declares
`SECOND_HANDLE` and `REOPEN` as `Capability` associated constants; a declined
capability still runs its rule, reporting the fixture's stated reason rather
than vanishing. `happenstance_testkit::fixtures::MemoryFixture` is named as
the reference implementation. Use "fixture," "handle," and "capability" in
that precise sense rather than as generic testing words.

**Skeleton (🔩).** A crate with "real associated types and `todo!()` bodies,
`publish = false`," scoped `#![allow(clippy::todo)]` — "an instrument first
and a target second," and explicitly "not an adapter" until it clears the
suite (`CLAUDE.md` repository map). Six exist: `happenstance-sqlite`,
`happenstance-cloudflare`, `happenstance-ladybug`, `happenstance-postgres`,
`happenstance-neon`, `happenstance-sync`.

**Typed layer, facade, worked example.** `happenstance` (the bare crate name)
is the "typed layer" — per ADR-0006, "the crate most people will `cargo add`,"
today "a five-line facade over the contract." `course-subscriptions` under
`examples/` is "the canonical DCB worked example" (`CLAUDE.md` repository
map). `spec/SPECIFICATION.md` and `RUNBOOK.md` both use "the worked example" as
a fixed term for this crate, not a generic phrase — e.g.
`spec/SPECIFICATION.md:132` ("the worked example is this document's own crate
names") and `RUNBOOK.md:3971` (Phase 7's heading, "The typed layer and the
worked example").

**Phase.** `RUNBOOK.md` numbers its remaining work as phases 0–14 with a
dependency graph (`RUNBOOK.md:713` onward, e.g. `## Phase 7 — The typed layer
and the worked example` at `RUNBOOK.md:3971`, `## Phase 12 — Publish 0.2.0` at
`RUNBOOK.md:4448`). "Phase" is RUNBOOK's word for its own sequencing, distinct
from a Redkiln "project" or "story" — the intake brief is explicit that how
phases map onto projects/stories is left to `/redkiln:plan`, not decided in
grounding (`.bklg/from-contract-to-published-library/_intake-brief.md:84-87`).

**Gate.** Overloaded on purpose across three layers, and the charter should
disambiguate which one it means: (1) `cargo xtask ci`, "the whole gate," run
locally (`CLAUDE.md`, Commands); (2) a `spec/SPECIFICATION.md` clause's own
gate, e.g. "PS-2 is a single gate under thirteen rows"
(`.bklg/from-contract-to-published-library/_intake-brief.md:21-22`); (3) a
Redkiln stage gate — e.g. the Intake gate this very brief satisfied
(`.bklg/from-contract-to-published-library/_intake-brief.md:212-227`), backed
by `.redkiln/templates/gates/*` checklists that CLAUDE.md notes are
deliberately customised, one of six intentional `template-drift` advisories.

**Initiative / project / story, and stage names.** Confirmed via the
`redkiln:conventions` skill: initiative decomposes into projects, a project
into stories, and a story is "the unit that walks the process engine (discover
→ spec → plan → implement → report → closeout)." This initiative's own item is
`HS-I0006`, currently `stage: intake`
(`.bklg/from-contract-to-published-library/initiative.md:1-13`); the sibling
`support` initiative (`HS-I0005`) is the only other item in the tree and is
also still at `stage: intake`
(`.bklg/support/initiative.md:1-13`) — there is no closed or archived
initiative yet to draw prior-art vocabulary from.

**Single-write-path rule.** "The CLI is the only writer" of an item's system
frontmatter fields (`id`, `stage`, `status`, `updated`, `links`, …) — drive
every transition through `redkiln <command>`; the prose body is free to edit
by hand. Stated in both `CLAUDE.md` and the `redkiln:conventions` skill, and
enforced by a `PreToolUse` hook.

**Persona / journey — the product layer, not yet populated here.**
`.kb/product/README.md` defines a **persona** as "a real, evidenced audience —
their goal, their context, what they already do instead, and what they are
afraid of," and a **journey** as "the moment-by-moment path a persona takes
through a task." Explicitly out of scope for this project's own personas layer
are: unevidenced sketches (stay in `_discovery/distillation/` until closeout
promotes them), screens/flows/interaction patterns (that is `design/`), and
requirements or acceptance criteria (that is `.bklg/`)
(`.kb/product/README.md:1-44`). `userFacing` is set to `false` in the intake
brief for this initiative — "this is a library with no screen" — so the
persona/journey layer applies to this initiative only as **adapter authors**
and **application authors**, not end-user personas with a UI
(`.bklg/from-contract-to-published-library/_intake-brief.md:200-206`,
seed's "Who it is for" section).

**`_design.md` repurposed for a library.** CLAUDE.md is explicit that the
bundled design stage is reused here to mean "the public API surface —
signatures, visibility decisions, what the shape costs a caller, and a
doctest in place of a mock," and that `design.capture` is deliberately absent
from `.redkiln/config.yaml`, making the perceptual design review a *skip*
rather than a pass — confirmed by direct read: `.redkiln/config.yaml:15` reads
"Deterministic gate commands, and design capture. BOTH OPTIONAL, both
commented" and the `design.capture` key is not set.

**Tier and research angles.** The intake brief scores this initiative
`tier: standard`, "12/12 against the six-axis rubric"
(`.bklg/from-contract-to-published-library/_intake-brief.md:205-206`), and
lists nine approved research angles for Stage B discovery fan-out, explicitly
omitting the standing `interaction-pattern-prior-art` angle because there is
no screen — the API-shape review happens at the planning design stage instead
(`.bklg/from-contract-to-published-library/_intake-brief.md:187-206`).

## Where the vocabulary lives (files worth citing as Context anchors)

- `.kb/decisions/README.md` — decision-atom rules: immutability, repair vs.
  amendment, what belongs vs. doesn't.
- `.kb/concepts/README.md`, `.kb/governance/README.md`,
  `.kb/reference/README.md`, `.kb/open-questions/README.md`,
  `.kb/maps/README.md` — the other five kind-specific rulebooks; together with
  the decisions README these are the closest thing this repo has to a
  glossary, since no dedicated `glossary/` file exists.
- `.kb/maps/domain-map.md` — the one populated domain so far ("Specification
  governance & conformance" and "Contract ports, conformance, and the ADR
  corpus"), each with its own Reference/Playbook/Concept/Governance/Open-question
  rows and atom ids — a live example of every kind used correctly together.
- `.kb/maps/decision-map.md`, `.kb/maps/open-questions-index.md` — the two
  companion indexes; cite rather than duplicate per `.kb/maps/README.md`.
- `.kb/product/README.md` — persona/journey definitions and the "what does not
  belong here" boundary against `design/` and `.bklg/`.
- `.kb/decisions/0029-msrv-raised-to-1-97-1.md` — the one concrete
  amends-not-edits precedent in the tree.
- `spec/SPECIFICATION.md:98-227` — §1 Foundations: conformance language (RFC
  2119 usage, the `MAY` warning tied to position gaps), the four maturity
  markers, and the FROZEN/binding-vs-release-binding distinction per port.
- `CLAUDE.md` — repository map (skeleton/typed-layer/facade/worked-example
  definitions), "The rule that matters" (Fixture/Capability vocabulary), and
  the Redkiln section (atom, single-write-path rule, the six deliberate
  `template-drift` customisations, `_design.md`'s repurposing).
- `RUNBOOK.md:157`, `RUNBOOK.md:713` onward — the phase table and phase
  headings, for citing a specific phase by number and name.
- `.redkiln/config.yaml:15` — confirms `design.capture` is commented out.
- `.bklg/from-contract-to-published-library/_intake-brief.md` — this
  initiative's own accepted vocabulary: Clauses section (PS-\*/SY-\*/VT-\*/ES-\*/
  CF-\*/WF-\* naming), the port-freeze axis language ("the axis it is most
  likely to be wrong about"), and the `tier`/research-angles frontmatter-adjacent
  facts.
- `.bklg/from-contract-to-published-library/initiative.md`,
  `.bklg/support/initiative.md` — the only two initiative items in the tree;
  both unforged templates (`Summary` / `Outcomes` / `Definition of Done` /
  `Projects` / `Notes` headings), useful only as the current charter-body
  skeleton, not as prior-art prose.

## Gaps against the tasking's assumed layout

- No `.kb/03-reference/glossary/` — use the kind-specific READMEs above
  instead; there is no single glossary file to cite.
- No `.bklg/_archive/` — no prior initiative has closed out yet, so there is
  no archived-initiative vocabulary or precedent charter to draw house style
  from beyond the two unforged templates already cited.
- No `./scripts/kb-dotmd.sh` in this repo — grounding was done with `rg` over
  `.kb/` and `.bklg/` plus direct file reads instead.
