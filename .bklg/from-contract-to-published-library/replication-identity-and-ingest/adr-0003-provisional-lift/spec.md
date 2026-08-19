---
item: HS-S0108
stage: spec
created: 2026-08-12T13:47:48.799Z
updated: 2026-08-12T13:47:48.799Z
template_sig: 87bbf1d0
rendered_sig: 4be91bd3
---

# Spec — ADR-0003 loses provisional by a new atom, never by an edit

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` — DoD 14, AC-13, BR-10 |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` — the DAG edge `publication → replication → retention` |
| Project | `.bklg/from-contract-to-published-library/replication-identity-and-ingest/project.md` — AC-006, DR-3, DR-5, DoD 5, DoD 7 |
| This spec | `.bklg/from-contract-to-published-library/replication-identity-and-ingest/adr-0003-provisional-lift/spec.md` |
| This story's discover | `.bklg/from-contract-to-published-library/replication-identity-and-ingest/adr-0003-provisional-lift/discover.md` — the three named wrong implementations this spec is written against |
| Key brief (architecture) | `.bklg/from-contract-to-published-library/replication-identity-and-ingest/_decomposition.md` — *Tension 5*, *The seam, in package terms* (row `.kb/_intake/`), AC-A09 |
| Key brief (design) | `.bklg/from-contract-to-published-library/replication-identity-and-ingest/_design.md` — **no user-facing surface**, signed off 2026-08-12. This story renders none and re-decides none |
| Story map | `.bklg/from-contract-to-published-library/replication-identity-and-ingest/_storymap.md` — slice `wire-message-set-and-round-trip`, row 3; *Merge order* step 4 |
| Roadmap pointer | `RUNBOOK.md:391-392` (the standing provisional-marker ledger), `RUNBOOK.md:4601` (the byte-identity half is what lifts it), `RUNBOOK.md:4613` (phase 13's exit criterion), `RUNBOOK.md:287` (the precedent row for an ADR number the queue never carried) |

## One-line PR slice

Lift ADR-0003's `provisional` marker with a **new** atom through kb-ingest that `depends_on` it
and cites the round trip as the evidence its own Status section asks for — never an edit to the
accepted atom — or, if the round trip cannot be made byte-identical, record why it cannot lift.

## Executive summary

`.kb/decisions/0003-opaque-payloads.md:87-95` says, in the atom's own words, that its payoff
*"has not yet been exercised, because no replication code exists yet"* and that *"It lifts at
phase 13, when `happenstance-sync` round-trips an event between two stores without deserialising
its payload."* The story before this one (HS-S0107,
`byte-identical-round-trip-and-idempotent-replay`) produces exactly that observation. **This PR is
the record of the discharge and nothing else.** No `.rs` file changes; no `spec/SPECIFICATION.md`
line changes; `.kb/decisions/0003-opaque-payloads.md` is byte-identical in the diff.

The delta over what the project brief already decided is four choices the brief left open and this
spec now makes:

1. **Standalone, not folded into ADR-0026.** `_decomposition.md`'s *Tension 5* recommends riding
   inside ADR-0026 and calls a standalone atom "equally legitimate". The sequencing settles it:
   ADR-0026 merges in slice 1 and this evidence does not exist until slice 4, so an ADR-0026 that
   asserted the lift would be citing a measurement that had not been taken when it was accepted.
   The atom is **ADR-0030**, on ADR-0029's unscheduled-number precedent (`RUNBOOK.md:287`).
2. **Amendment, not supersession** — `supersedes: null`, `depends_on: [kb-decision-0003]`, and
   ADR-0003 receives *no* edit at all, not even the metadata flip.
3. **The measurement is a `reference` atom the decision cites**, because
   `.kb/decisions/README.md:35-38` says the evidence a decision rests on does not live in the
   decision.
4. **The citation names the assertion, not the story.** "AC-006 green" is not a claim anyone can
   check; the byte-equality assertion, the replay witness and the `PayloadTouchingSuite` control,
   named by path, are.

## Context pack

Everything an implementer needs to start is here. The deeper artefacts stay behind the anchors.

### The decision this story records is already made, and its wording is not this story's to write

ADR-0003 named its own lift condition and set two conjuncts, both of which must be evidenced:
a round trip **between two stores**, and **without deserialising its payload**
(`.kb/decisions/0003-opaque-payloads.md:91-93`). That sentence *is* AC-006 — cite it, do not
restate it as a new condition (`_decomposition.md`, *The Accepted atoms that constrain this*).
Quote it; a paraphrase is a second condition nobody agreed to.

### The lift cannot be an edit, and the reason is sharper than "atoms are immutable"

`redkiln validate --kb` checks every `status: accepted` decision atom against `HEAD` and fails the
gate on a changed body (`.kb/decisions/README.md:7-13`). That is the loud tripwire. The
*interesting* constraint is the one that survives someone arguing the marker is a trivial
correction: `.kb/governance/rewrite-the-referent-never-the-reasoning.md` permits rewriting a
decision **in place** when the edit does not change what the document asserts — ADR-0003 itself was
rewritten at phase 0 to say `happenstance-core`, because renaming an identifier reverses nothing.
Deleting "and provisional" fails that test outright: ADR-0003's Status *asserts* that the payoff
has not been exercised, and after HS-S0107 that assertion is false. Changing what a signed decision
asserts is a new atom's job, always.

### Amending is not superseding, and the distinction is invisible to every automated check

`supersedes: kb-decision-0003` passes `validate --kb`, passes `doctor`, never touches ADR-0003's
body, and produces an exemplary-looking diff — while retiring the atom that says `Event::data` is
opaque `Bytes`, that `happenstance-core` carries no `serde` in its default features, and that the
`serde` feature covers envelope types only. `CLAUDE.md`'s binding constraint 2 would then cite a
superseded decision. The correct shape is already in the tree: ADR-0029 amends ADR-0004 with
`supersedes: null`, `depends_on: [kb-decision-0004]` (`:25-26`), and a summary saying in its own
words *"This amends ADR-0004 rather than superseding it - that decision's body stays verbatim"*
(`:12-14`). `.kb/maps/decision-map.md:76-79` records why `superseded_by` is left `null` on ADR-0004
and the edge carried by `depends_on` instead. Copy that shape exactly. **ADR-0003 receives no edit
at all** — `.kb/decisions/README.md:9-13` calls the metadata flip "the only edit an accepted
decision ever receives", and an amendment does not earn even that.

### One decision per title, which is also the argument against folding this into ADR-0026

`.kb/playbooks/one-decision-per-adr-title.md` says the split stops being worth it only *"when both
halves are settled by the same evidence"*. ADR-0026's subject — what a peer is, what the port may
assume about an unseen transport, what ingest promises — is not settled by a byte-equality
assertion. Folding the lift in would produce precisely the "and" title the playbook was written
about: a strong decision and a weaker rider settled by different evidence at different times. The
cost of standalone is honest and small: one ADR number the `RUNBOOK.md:285-308` queue does not
carry, taken as **ADR-0030** in the same "unscheduled — the queue had no number for it" form
ADR-0029 uses at `RUNBOOK.md:287`, and one row added to that table so the number is not invisible.

### The evidence layers: a `reference` atom carries the measurement, the decision cites it

`.kb/decisions/README.md:35-38` is explicit that the measurement a decision rests on is a
`reference` atom, *"so that a decision can rest on a number without restating how the number was
obtained"*, and `.kb/reference/README.md` adds the dating rule: a measurement without a commit sha
or a date *"is not a weaker reference, it is a false one"*. The worked precedent is
`.kb/reference/wire-format-encoding-measurements.md`, which carries ADR-0016's numbers. So this
wave stages **two** documents: a reference atom recording what was run, on which commit, and what
it returned; and the ADR-0030 decision atom that cites it and lifts the marker. If the wave's
adjudication merges them, the layering is a preference — but naming the assertion is not.

### The third mutant: lifting on evidence that never looked at a byte

HS-S0107's own discover names `DecodedValueRoundTrip`, a comparison on decoded values that is green
against a codec which is not byte-identical — including `InvertedHumanReadable`, the measured WF-11
branch that renders `[de ad be ef]` as `[222,173,190,239]` in JSON while round-tripping perfectly
(`spec/SPECIFICATION.md:2315-2322`). A lift atom citing "the round trip is green" cannot tell the
two apart. **The citation must name the assertion and the negative control**: the payload `Bytes`
compared for equality at the receiver *after commit*, and `PayloadTouchingSuite` — the conformant
peer that fails any assertion which decodes a payload — passing. That is a claim someone can check.

### Nothing is retired by this lift, and the atom must say so

All three of ADR-0003's constraints stand unchanged: `Event::data` is `bytes::Bytes`;
`happenstance-core` carries no `serde` in its default features; the `serde` feature covers envelope
types only. What changed is that the claim behind them was exercised. Read `CLAUDE.md`'s binding
constraint 2 carefully while writing this — after ADR-0006 it constrains `happenstance-core`, and
`happenstance`, the typed layer, is the crate that *will* depend on `serde`. A lift atom that reads
as loosening constraint 2 has landed the wrong decision.

### The long form must not move under SY-12

`spec/SPECIFICATION.md:6153-6172` (SY-12, `[FROZEN]`) quotes ADR-0003's lift condition by line from
the long-form record at `references/adr/0003-opaque-payloads.md:14-15` and concludes that a
metadata-borne identity *"fails the ADR's own test"*. `cargo xtask spec-trace` resolves that
citation by a windowed search for a subject string rather than an exact line match
(`.kb/playbooks/anchoring-citations-in-a-long-lived-document.md`), so text inserted *above* it is
tolerated and moving or rewording the quoted sentence is not. Do not touch
`references/adr/0003-opaque-payloads.md`. The new long-form record lands beside it as
`references/adr/0030-*.md`.

### The second arm is a real outcome, not a fallback

AC-006 admits two endings: the marker lifts, or *the reason it cannot be lifted is recorded*. DR-3
says a refusal is a legitimate answer and a silence is not. Both endings are the same shape — a new
atom, through `.kb/_intake/` and `/redkiln:kb-ingest`, naming the assertion that failed and what
would have to be true to lift. What is forbidden is a weakened assertion upstream so that this
story can report the happier ending.

### Everything arrives through the ingest path

`.kb/` atoms are authored by `/redkiln:kb-ingest` from `.kb/_intake/`, never by hand; hand-authoring
was reverted once already (`0269720`, cited in `CLAUDE.md`, *Where the work lives*). AC-A09 states
it for this project by name and adds the mechanical check: `.kb/decisions/0003-opaque-payloads.md`
is unchanged in the diff.

### The persona slice this realises

The library author's journey ends at *"nobody has to guess"* (initiative AC-13). For a reader who
arrives at `CLAUDE.md`'s binding constraint 2 and follows it to ADR-0003, the observable outcome of
this story is that the atom they land on is no longer hedged by a marker saying the claim was never
tested — and the atom that lifts it tells them, by path, which assertion tested it and on what
commit. That is the whole user-visible delta, and it is the last one AC-006 owes.

## Integration contract

- **Archetype**: `foundation` — real in-tree substrate (the decision record every downstream reader
  of constraint 2 lands on), consumed by this same project's exit audit. Not a double and not a
  placeholder atom.
- **Slice / milestone**: `wire-message-set-and-round-trip`. Slice-mates, in merge order:
  `message-set-on-the-envelope` (HS-S0106) → `byte-identical-round-trip-and-idempotent-replay`
  (HS-S0107) → **this story** (HS-S0108). This story merges **last** in the slice, because it cites
  evidence that does not exist until HS-S0107 merges (`_storymap.md`, *Merge order* 4).
- **Mount point**: **`.kb/maps/decision-map.md`** — the decision corpus's composition root, and the
  one place the whole supersession graph is visible at a glance. Its ADR-0003 row reads
  `accepted (provisional)` with `—` in the supersession column today (`:51`); after this story it
  reflects the lift and carries the amendment edge, in the shape line `:65` already uses for
  ADR-0029 → ADR-0004 and line `:76-79` already explains. An atom absent from this map is an atom
  nobody can find from the index, which is the KB analogue of a constructed-but-unmounted component.
  The map is a `kind: map` atom and therefore editable; the ingest wave's Maps phase is what writes
  it (`.kb/maps/decision-map.md:8-10`).
- **Wires into**:
  - `.kb/decisions/0003-opaque-payloads.md` — cited by id and quoted by line; **untouched**.
  - `.kb/decisions/README.md:7-23` — the immutability rule and the repair/amendment discriminator.
  - `.kb/decisions/0029-msrv-raised-to-1-97-1.md:12-14`, `:25-26` — the frontmatter shape to copy.
  - `.kb/governance/rewrite-the-referent-never-the-reasoning.md` — the "does the edit change what
    the document asserts" test that rules out an in-place rewrite here.
  - `.kb/playbooks/one-decision-per-adr-title.md` — the standalone-versus-folded argument.
  - `.kb/reference/wire-format-encoding-measurements.md` — the reference-atom precedent and shape.
  - `.kb/_intake/` and `/redkiln:kb-ingest` — the only authoring path (`.kb/_intake/README.md`).
  - `.kb/_governance/integration-waves/` — where the wave records itself.
  - `references/adr/` — the long-form record beside `0003-opaque-payloads.md`, which stays put.
  - HS-S0107's landed test artefacts under `crates/happenstance-sync-testkit/` — **read-only**;
    they are what the citation names.
- **Renders surfaces**: **none.** `_design.md` records `N/A — no user-facing surface` for this
  whole project, signed off 2026-08-12, and every `## Items` entry is `N/A`. This story adds no
  public Rust item, so it claims no `path` id and changes none.
- **Conformance rule(s)**: **none, and this is not adapter-observable.** No peer, store or fixture
  behaviour changes; nothing an adapter does can make this story pass or fail. The instruments are
  `redkiln validate --kb` (accepted-decision immutability against `HEAD`, atom schema, link
  resolution), `redkiln doctor`, and `git diff --exit-code` over the one file that must not move.
  Stated explicitly because a story that names no rule is normally a port change nothing can fail;
  here there is no port change at all.
- **Clause(s)**: **discharges none and amends none.** `spec/SPECIFICATION.md` is not in this diff.
  SY-12 (`:6153-6172`, `[FROZEN]`) is *read* — it cites ADR-0003's lift condition from the long form
  at `references/adr/0003-opaque-payloads.md:14-15`, so this story's obligation to that clause is
  purely conservative: leave the cited sentence exactly where it is and `cargo xtask spec-trace`
  stays green.
- **Advances DoD scenario**: initiative **DoD 14** — *"Replication has an answer on disk… and
  `redkiln validate --kb` passes"* (`initiative.md:398-402`) — and project **DoD 5**. It closes the
  third and last limb of project AC-006, and ticks `RUNBOOK.md:4613`'s exit criterion, whose second
  clause is literally *"and ADR-0003 loses `provisional`"*.

## PR boundary

`redkiln verify --grain story` reads the first fenced block below and fails on any file changed
outside it.

```
.kb/_intake/**
.kb/decisions/**
.kb/reference/**
.kb/maps/**
.kb/_governance/integration-waves/**
references/adr/**
RUNBOOK.md
.bklg/from-contract-to-published-library/replication-identity-and-ingest/adr-0003-provisional-lift/**
```

`.kb/decisions/**` is inside the boundary because the wave *writes* `0030-*.md` there. That the
boundary permits touching `0003-opaque-payloads.md` is not a licence: `redkiln validate --kb` is the
check that it did not, and AC-001 asserts it by `git diff` rather than by intention.

**In this PR**

- The staged intake documents under `.kb/_intake/` — the lift's raw material — and the atoms
  `/redkiln:kb-ingest` writes from them: the ADR-0030 decision atom and the round-trip reference
  atom.
- The long-form record `references/adr/0030-*.md`, in the corpus's existing long-form shape.
- The map rows the wave's Maps phase writes in `.kb/maps/decision-map.md` — the ADR-0003 row's
  status and edge, and a row for ADR-0030.
- One row in `RUNBOOK.md`'s ADR queue (`:285-308`) for ADR-0030, in the unscheduled form `:287`
  already uses for ADR-0029. This is the single non-`.kb`, non-`references/` file in the diff, and
  it is here so that an ADR number invisible to the queue cannot happen twice.
- This story's own `_ledger.md`.

**Explicitly not in this PR**

- Any edit to `.kb/decisions/0003-opaque-payloads.md`, in body or in frontmatter. Not the metadata
  flip either — this is an amendment, and ADR-0003 keeps `status: accepted`, `superseded_by: null`.
- Any edit to `references/adr/0003-opaque-payloads.md`, whose lines 14-15 SY-12 cites.
- Any `.rs` file, any `Cargo.toml`, any `xtask` step, any `CHANGELOG.md` entry. The assertions this
  atom cites are HS-S0107's and already merged; weakening or re-writing them here would be lifting
  the marker on evidence this story wrote for itself.
- Any `spec/SPECIFICATION.md` line, including SY-12's `Rejects:` paragraph. Clause repairs are
  `frozen-clause-repairs` (HS-S0114); the clause arithmetic is
  `clause-arithmetic-and-deferral-renewals` (HS-S0115).
- Ticking `RUNBOOK.md:4613` or any phase-13 exit checkbox, and the ADR-0004 row at
  `RUNBOOK.md:392`/`:4475` — phase 12's marker, not this one's.
- ADR-0026's and ADR-0027's bodies. They merged in slice 1 and are not reopened to hold a lift.
- Re-litigating ADR-0003's rejected alternatives — the payload type parameter and
  `serde_json::Value` in the contract crate (`.kb/decisions/0003-opaque-payloads.md:76-85`). The
  lift records that the winner's claim was exercised; it does not re-run the fork.

**Merge DoD.** `redkiln validate --kb && redkiln doctor` green, `cargo xtask lints &&
cargo xtask spec-trace` green, `git diff --exit-code HEAD~..HEAD -- .kb/decisions/0003-opaque-payloads.md`
reports no change, and `.kb/maps/decision-map.md`'s ADR-0003 row no longer reads `provisional`.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| The lift is a **new atom**, authored through the ingest path | Documents are staged under `.kb/_intake/` and `/redkiln:kb-ingest` writes the atoms; nothing under `.kb/` is hand-written. A successful ingest clears `_intake` and commits the wave on its own branch | `.kb/_intake/README.md`; `CLAUDE.md`, *Where the work lives*; `_decomposition.md`, AC-A09 |
| ADR-0003 is **byte-identical** in the diff | `status: accepted`, `superseded_by: null`, body and frontmatter untouched. `redkiln validate --kb` checks every accepted decision atom against `HEAD` and fails on a changed body — the loud tripwire for the illegal route | `.kb/decisions/README.md:7-13`; `.kb/decisions/0003-opaque-payloads.md` |
| The relationship is **amendment**, spelled in frontmatter | `supersedes: null`, `depends_on: [kb-decision-0003]`, `related: [<the reference atom's id>]`. The summary says in its own words that ADR-0003's body stays verbatim and its constraints stay in force | `.kb/decisions/0029-msrv-raised-to-1-97-1.md:12-14`, `:25-26`; `.kb/maps/decision-map.md:76-79` |
| The atom is **standalone**, numbered **ADR-0030** | Not folded into ADR-0026: ADR-0026 merged in slice 1 and this evidence arrives in slice 4. The number is unscheduled, in ADR-0029's precedent form, and gets a queue row so it is not invisible | `_decomposition.md`, *Tension 5*; `_storymap.md`, *Merge order* 4; `RUNBOOK.md:285-308`, `:287` |
| One decision, one title | The title states the lift and nothing else. It does not also settle what `serde` may be used for, what a peer is, or how a payload evolves | `.kb/playbooks/one-decision-per-adr-title.md` |
| The **evidence layers**: a `reference` atom carries the measurement | What was run, on which commit sha, on what date, and what it returned. A measurement without a sha or a date reads as current and nothing can detect that it stopped being true | `.kb/decisions/README.md:35-38`; `.kb/reference/README.md`, *The dating rule*; `.kb/reference/wire-format-encoding-measurements.md` |
| The citation **names the assertion, not the story** | The payload `Bytes` compared for equality at the receiver after commit; the replay witness; `PayloadTouchingSuite` passing. Never "AC-006 green" — that is a claim nobody can check, and it is green against `DecodedValueRoundTrip` too | HS-S0107 `discover.md`, *The wrong implementation*; `spec/SPECIFICATION.md:2315-2322` (WF-11's measured inverted branch) |
| **Both conjuncts** of the lift condition are shown discharged | *"round-trips an event between two stores"* — a real store boundary, not two handles onto one store; *"without deserialising its payload"* — no decode on any path in the suite (DR-5). The sentence is quoted, not paraphrased | `.kb/decisions/0003-opaque-payloads.md:91-93`; `project.md`, DR-5; `spec/SPECIFICATION.md:6153-6172` |
| **Nothing is retired** | The atom restates that `Event::data` stays `bytes::Bytes`, that `happenstance-core` carries no `serde` in default features, and that the `serde` feature covers envelope types only — and that `happenstance`, the typed layer, is unaffected and still may depend on `serde` | `.kb/decisions/0003-opaque-payloads.md`; `CLAUDE.md`, binding constraint 2 |
| The record is **mounted in the index** | `.kb/maps/decision-map.md`'s ADR-0003 row stops reading `accepted (provisional)` and carries the amendment edge; ADR-0030 and the reference atom get their own rows, written by the wave's Maps phase | `.kb/maps/decision-map.md:8-10`, `:51`, `:65`, `:76-79` |
| The long form lands **beside**, not over | `references/adr/0030-*.md` is new; `references/adr/0003-opaque-payloads.md` does not move, because SY-12 cites its lines 14-15 and `spec-trace` resolves that by windowed subject search — insertion above is tolerated, a moved or reworded sentence is not | `spec/SPECIFICATION.md:6165-6172`; `references/adr/0003-opaque-payloads.md:13-15`; `.kb/playbooks/anchoring-citations-in-a-long-lived-document.md` |
| **The second arm** is the same shape | If HS-S0107 did not achieve byte-identity, the wave lands an atom recording *why it cannot lift*, naming the failing assertion and what would have to be true. A refusal is an answer; a silence is not, and a weakened upstream assertion is neither | `project.md`, AC-006 second arm and DR-3; `_decomposition.md`, *Tension 5* final sentence |
| No code, no clause, no rule | This story adds no conformance rule, changes no port, and edits no `[FROZEN]` clause. `cargo xtask lints && cargo xtask spec-trace` still runs at story grain, unconditionally, so a spec-only story is still checked | `.redkiln/config.yaml:48`; `_decomposition.md`, testing brief |

## Data and migrations

**N/A — no schema, no store, no runtime data.** This story writes no `.rs`, defines no table, and
adds no serialised type; nothing in it reaches a store, a wire or a migration file.

The nearest analogue is worth naming so it is not mistaken for one: the **corpus** has a state that
changes, and it changes by append. `.kb/decisions/0003-opaque-payloads.md` is a row that is never
UPDATEd — the corpus's only legal write here is the INSERT of a new atom plus an edit to the
*index* (`.kb/maps/decision-map.md`), which is a derived view and is regenerated by the wave's Maps
phase rather than hand-patched. There is no down-migration and none is owed: the record that the
marker was once provisional is itself worth keeping, which is the same rule under which an
open-question atom is resolved rather than deleted (`.kb/maps/open-questions-index.md:7-13`,
`project.md` DR-10).

## Acceptance criteria

Each criterion is written from the reader's side of the corpus, because that is the only place
this story is observable. The persona is **the evaluator (pre-adoption)** —
`.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md:249-317`
— who cannot run this repository's suite, has no external body to check "DCB-compliant" against,
and judges the project by what it chose to write down; and, where the criterion is about a
constraint staying in force, **the adapter author** (`:114-181`) who arrives via `CLAUDE.md`'s
binding constraint 2. The initiative's framing is AC-13, *nobody has to guess*
(`initiative.md:344-346`).

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **GIVEN** an evaluator who follows `CLAUDE.md` binding constraint 2 to `.kb/decisions/0003-opaque-payloads.md` to judge whether this project's signed decisions are stable, **WHEN** this PR has merged, **THEN** the atom they read is byte-identical to the one that was there before — body *and* frontmatter, `status: accepted` and `superseded_by: null` included — so no signed decision was rewritten under them to make a later claim true. | `git diff --exit-code HEAD~..HEAD -- .kb/decisions/0003-opaque-payloads.md` reports no change, **and** `redkiln validate --kb` passes (it checks every `status: accepted` decision atom against `HEAD`, `.kb/decisions/README.md:7-13`). Both, because the diff assertion alone would pass on a branch that never had the atom. |
| AC-002 | **GIVEN** the same evaluator asking whether this knowledge base is a process output or a hand-maintained pile of markdown, **WHEN** they read the diff, **THEN** every atom it adds under `.kb/` was written by `/redkiln:kb-ingest` from documents staged in `.kb/_intake/`, the wave records itself under `.kb/_governance/integration-waves/`, `.kb/_intake/` is left cleared, and `redkiln validate --kb && redkiln doctor` are green. | `redkiln validate --kb && redkiln doctor` green; a new wave directory exists under `.kb/_governance/integration-waves/` alongside `2026-08-10-intake` and `2026-08-10-intake-2`; `.kb/_intake/` holds only its `README.md`. |
| AC-003 | **GIVEN** the next contributor who wants `serde` in `happenstance-core` and follows binding constraint 2 to see what stands against it, **WHEN** they land on ADR-0003 after this PR, **THEN** it is still in force and not retired: the new atom declares `supersedes: null` and `depends_on: [kb-decision-0003]`, ADR-0003 keeps `superseded_by: null`, and the new atom's summary says **in its own words** that ADR-0003's body stays verbatim and its constraints stay in force. | Read the new atom's frontmatter: `rg -n "^supersedes:\|^depends_on:" .kb/decisions/0030-*.md` shows `null` and `kb-decision-0003`; `rg -n "^superseded_by:" .kb/decisions/0003-opaque-payloads.md` still reads `null`; `redkiln validate --kb` resolves the `depends_on` link. The shape is ADR-0029's, `.kb/decisions/0029-msrv-raised-to-1-97-1.md:12-14`, `:25-26`. |
| AC-004 | **GIVEN** a reader scanning the decision corpus for what each atom decides, **WHEN** they reach the lift, **THEN** it is a **standalone** atom numbered **ADR-0030** whose title states the lift and nothing else — it does not also settle what a peer is, what ingest promises, or what `serde` may be used for — and `RUNBOOK.md`'s ADR queue carries a row for ADR-0030 in the same "unscheduled — the queue had no number for it" form `RUNBOOK.md:287` already uses for ADR-0029, so an ADR number invisible to the queue cannot happen twice. | The atom's `title:` and `# ` heading name one subject (`.kb/playbooks/one-decision-per-adr-title.md`); `rg -n "ADR-0030" RUNBOOK.md` returns a queue row inside `RUNBOOK.md:285-308`; `rg -n "0030" .kb/decisions/0026-*.md .kb/decisions/0027-*.md` returns nothing — the lift did not ride inside either. |
| AC-005 | **GIVEN** an evaluator who cannot run the suite and has to trust what the project published, **WHEN** they open the evidence the lift rests on, **THEN** a `kind: reference` atom records what was run, **on which commit sha**, on what date, and what it returned, and the ADR-0030 decision atom cites it by id rather than restating how the number was obtained — so a measurement that later stops being true is detectable rather than reading as current. | `rg -n "^kind: reference" .kb/reference/<new-atom>.md`; the atom body carries a commit sha and a date (`.kb/reference/README.md`, the dating rule; precedent `.kb/reference/wire-format-encoding-measurements.md`); ADR-0030's `related:` names the reference atom's id and `redkiln validate --kb` resolves it. |
| AC-006 | **GIVEN** ADR-0003's own lift condition — *"round-trips an event between two stores without deserialising its payload"* (`.kb/decisions/0003-opaque-payloads.md:91-93`) — quoted verbatim rather than paraphrased, **WHEN** ADR-0030 asserts it discharged, **THEN** it names the assertion and the negative control **by path**: the payload `Bytes` compared for equality at the receiver *after commit*, the replay witness, and the conformant undecodable-payload variant (`PayloadTouchingSuite`, `fails: &[]`) passing — never *"AC-006 green"*, which is equally true of a comparison on decoded values. **OR**, if either conjunct failed, ADR-0030 lifts nothing and instead records which assertion failed and what would have to be true to lift. | The quoted sentence matches ADR-0003 exactly (`rg -F` the quote against `.kb/decisions/0003-opaque-payloads.md`); every evidence citation in ADR-0030 resolves to a path that exists at that commit under `crates/happenstance-sync-testkit/`; `rg -n "AC-006" .kb/decisions/0030-*.md` returns no line where an AC id stands in for an assertion. |
| AC-007 | **GIVEN** an adapter author who reads a "provisional lifted" atom and asks what it loosened, **WHEN** they read ADR-0030, **THEN** it states that nothing was retired: `Event::data` stays `bytes::Bytes`, **`happenstance-core`** carries no `serde` in its default features, and the `serde` feature covers envelope types only — and that `happenstance`, the typed layer, is unaffected and may still depend on `serde`. | `rg -n "happenstance-core\|bytes::Bytes\|envelope types only\|typed layer" .kb/decisions/0030-*.md` finds all four claims restated as still in force; the atom nowhere says the constraint is relaxed. Cross-read against `CLAUDE.md`, binding constraint 2. |
| AC-008 | **GIVEN** a reader who starts at the index rather than at an ADR number, **WHEN** they open `.kb/maps/decision-map.md` after this PR, **THEN** ADR-0003's row no longer reads `accepted (provisional)` and carries the amendment edge in the shape `:65` and `:76-79` already use for ADR-0029 → ADR-0004, ADR-0030 and the reference atom have their own rows, and the long-form record lands **beside** as `references/adr/0030-*.md` while `references/adr/0003-opaque-payloads.md` does not move — so SY-12's line citation still resolves. | `rg -n "ADR-0003\|ADR-0030" .kb/maps/decision-map.md` shows the updated status and the edge plus a new row; `git diff --exit-code HEAD~..HEAD -- references/adr/0003-opaque-payloads.md` reports no change; `cargo xtask spec-trace` green. |

Coverage of the traced project AC: **AC-006**'s third limb (*"ADR-0003's `provisional` marker is
lifted against the ADR's own stated lift condition — or the reason it cannot be lifted is
recorded"*, `project.md:209-213`) is covered by AC-001 – AC-008 together; the first two limbs are
`byte-identical-round-trip-and-idempotent-replay`'s and are consumed here as evidence, not
re-proved.

## Interaction quality

**Composition family: not applicable, and the determination is signed off.**
`.bklg/from-contract-to-published-library/replication-identity-and-ingest/_design.md` records
**N/A — no user-facing surface** for this whole project (signed off 2026-08-12), with every
`## Items` entry `N/A`, and the initiative charter carries `userFacing: false` and no
`interaction-patterns.md` (`initiative.md:411`). There is no composition, transience policy,
density budget, hierarchy or named anti-pattern for this story to honour or contradict, and this
story adds no public Rust item either, so the design's substitute obligation — a reviewed API
surface — has nothing to bind to. Nothing in this section may be read as re-deciding that.

**State family: applicable by analogue, and the analogue is exact.** The corpus *is* the surface
this story changes, and the state invariants a screen would carry have literal referents here.
Each is carried by a numbered AC in the table above, not by a bullet in this section — a bullet
here would get no ledger row and would never be gated.

| State invariant | The referent in this story | Carried by | How it is verified |
| --- | --- | --- | --- |
| **In place, not a context jump** | The reader following constraint 2 lands on ADR-0003 and it is still the atom they were sent to — same id, same body, same `status: accepted`. The lift arrives *beside* it, not by relocating it. | AC-001, AC-003 | `git diff --exit-code` over the atom; `rg` over `superseded_by:` |
| **Non-occlusion** | The amendment must not hide what it amends. `supersedes:` would occlude ADR-0003 — every automated check passes and the atom reads as retired. `depends_on:` leaves both visible. | AC-003 | frontmatter read + `redkiln validate --kb` link resolution |
| **Preserved selection / scroll** | The long-form record's cited sentence keeps its position: SY-12 quotes `references/adr/0003-opaque-payloads.md:14-15` and `spec-trace` resolves it by windowed subject search, so insertion above is tolerated and a moved or reworded sentence is not. | AC-008 | `cargo xtask spec-trace`; `git diff --exit-code` over the long form |
| **Reversibility** | Nothing is destroyed: the record that the marker was once provisional survives in ADR-0003's own body, which is why the corpus's only legal write here is an append. There is no down-migration and none is owed. | AC-001 | the same diff assertion; *Data and migrations*, above |
| **Reachable without knowing the address** | An atom absent from `.kb/maps/decision-map.md` is reachable only by someone who already knows its number — the KB analogue of a constructed-but-unmounted component. The map is the mount point. | AC-008 | `rg` over `.kb/maps/decision-map.md`; the `RUNBOOK.md` queue row is the same invariant for the ADR number (AC-004) |
| **Legible without the tooling** | The evaluator persona reads the published corpus, not the backlog. A citation that only resolves inside `.bklg/` is unreadable to them; assertions and commits are cited by repository path and sha. | AC-005, AC-006 | NF-004, below; `rg -n "\.bklg/" .kb/decisions/0030-*.md` returns nothing load-bearing |

## Error conditions

| id | condition | required behaviour |
| --- | --- | --- |
| EC-001 | **HS-S0107's round trip did not come out byte-identical.** | AC-006's second arm fires: ADR-0030 lifts nothing and records *why it cannot lift* — the failing assertion by name, the hop that mutated the bytes, and what would have to be true. Same shape, same ingest path, same map rows. `.kb/maps/decision-map.md`'s ADR-0003 row then keeps `provisional` and the atom says so explicitly. What is forbidden is weakening the upstream assertion so the happier ending can be reported (`project.md` DR-3). |
| EC-002 | **The illegal route is attempted** — `.kb/decisions/0003-opaque-payloads.md` is edited to delete "and provisional". | `redkiln validate --kb` fails on accepted-decision immutability. The repair is to revert the edit and author the new atom; it is never to bypass the gate. This is the loud mutant and it is a tripwire, not a hazard (`discover.md`, *The wrong implementation*). |
| EC-003 | **The ingest wave's adjudication proposes `supersedes: kb-decision-0003`.** | **Halt and correct before the wave integrates.** Every automated check passes on this shape and ADR-0003 is retired anyway, leaving `CLAUDE.md`'s binding constraint 2 citing a superseded decision. `supersedes: null` + `depends_on:` is the only admissible relationship here. |
| EC-004 | **The wave adjudicates a MERGE of the reference document into the decision atom.** | Acceptable. The layering (`.kb/decisions/README.md:35-38`) is a strong preference, not a gate; what is **not** negotiable is that the commit sha, the date and the named assertion survive the merge (AC-005, AC-006). If they would not, decline the merge. |
| EC-005 | **ADR-0030 is already taken** by a wave that landed between planning and implementation. | Re-read `.kb/maps/decision-map.md` and `RUNBOOK.md:285-308` at authoring time, take the next free number, and update this spec's `## Clarifications` and the ledger's criterion text through a scope note — never reuse a number, and never leave the queue row pointing at the old one. |
| EC-006 | **`cargo xtask spec-trace` fails after the long-form record lands.** | The cause is a moved or reworded sentence in `references/adr/0003-opaque-payloads.md`, not a bad new file. Revert the long form to byte-identical and re-run; do not re-anchor SY-12, which is `[FROZEN]` and whose repairs belong to `frozen-clause-repairs` (HS-S0114). |
| EC-007 | **HS-S0107 has not merged when this story starts.** | Blocker, raised, not routed around. The whole content of this story is a citation to evidence produced elsewhere; producing that evidence here would make the lift rest on a measurement the lift's own author wrote. |

## Non-functional

| id | requirement | why it is here |
| --- | --- | --- |
| NF-001 | **Atom grain.** ADR-0030 is a canonical atom of roughly the corpus's usual ~100 lines carrying frontmatter, status and the supersession graph; the transcript, the rejected alternatives and the measurement table belong to the long-form record under `references/adr/`. | The two-places-on-purpose rule (`CLAUDE.md`, *Where the work lives*). An atom that swells into a record makes the atom unscannable; a record that shrinks into an atom loses ~78% of the corpus's substance. |
| NF-002 | **Zero gate-time cost and zero code.** No `.rs`, no `Cargo.toml`, no `xtask` step, no new CI job. `cargo xtask affected --base main` should resolve to no affected package, and the story-grain gate still runs `cargo xtask lints && cargo xtask spec-trace` unconditionally. | `.redkiln/config.yaml`, `reachability_static` and its comment: a story whose whole deliverable maps to no package must still be checked by something that reads files. |
| NF-003 | **Citation durability.** Atoms are cited by **id** (`kb-decision-0003`); the long-form record and `spec/SPECIFICATION.md` are cited by **`file:line`**. Never a line range into another atom. | Atom bodies are short and stable but their line numbers are not load-bearing anywhere; the long form's are, and `spec-trace` is what keeps them honest (`.kb/playbooks/anchoring-citations-in-a-long-lived-document.md`). |
| NF-004 | **Readable by someone outside this repository's process.** ADR-0030's body must stand up with no `.bklg/` path load-bearing in it — no story slug, no AC id, no ledger reference as the substance of a claim. | Persona 4 reads the corpus, not the backlog (`personas-and-journeys.md:249-317`). A decision whose evidence is a backlog id is, to that reader, an unsourced assertion. |
| NF-005 | **Tone parity with the corpus.** The atom names what the lift does *not* change as prominently as what it does, in the corpus's own register — the way ADR-0029 opens by saying what it is not. | AC-007 is the machine-checkable half of this; NF-005 is the half a reviewer reads for. |

## Implementation notes (non-prescriptive)

A plausible order, offered because the ingest path has a shape and getting it out of order costs a
re-run — not because any of it is prescribed.

1. **Read HS-S0107's landed `_ledger.md` first**, not its spec. Its evidence rows are written to be
   quoted verbatim (that spec's *Behavior and interfaces*, last-but-one row): rule name, test path,
   commit. If those rows do not carry a sha, get one before writing anything — NF-003 and AC-005
   both fail late otherwise.
2. **Draft two documents into `.kb/_intake/`**: the measurement (what was run, on which sha, on
   what date, what it returned) and the decision (the lift, its frontmatter shape, what it does not
   change). Write them as documents, not as atoms with frontmatter guessed at — `/redkiln:kb-ingest`
   is what turns them into atoms, and that is the whole point of AC-002.
3. **Write the long-form record by hand** under `references/adr/0030-*.md`. That directory is *not*
   the KB and is not ingest's output; hand-authoring there is normal (`CLAUDE.md`, *Where the work
   lives*). Keep `references/adr/0003-opaque-payloads.md` closed while you do it.
4. **Let the wave's Maps phase write `.kb/maps/decision-map.md`.** The map is a `kind: map` atom and
   therefore editable, but hand-patching it around the wave is how a row and an atom disagree.
5. **Add the `RUNBOOK.md` queue row last**, by hand, copying the form at `RUNBOOK.md:287`. It is the
   only non-`.kb`, non-`references/` file in the diff and it exists so the number is not invisible.
6. **Then run `redkiln validate --kb && redkiln doctor`**, then the diff assertions. If validate
   fails on immutability, something touched ADR-0003 — find it before doing anything else.

Two things worth resisting: the urge to "tidy" ADR-0003's Status while you are in the file, and the
urge to make ADR-0030 also say something useful about `serde` in the typed layer. Both are how a
one-decision atom becomes an "and" title (`.kb/playbooks/one-decision-per-adr-title.md`).

## Tests and CI (merge gate)

Grounded in the project testing brief's *The test mix, tier by tier*
(`_decomposition.md:625-663`), whose **Static** tier names `redkiln validate --kb && redkiln doctor`
as the merge gate for AC-006's lift specifically. There is no unit or integration tier here: this
story compiles nothing.

| tier | command / path | proves |
| --- | --- | --- |
| Static — KB conformance | `redkiln validate --kb` | AC-001 (accepted-decision immutability against `HEAD`), AC-002 (atom schema), AC-003 (the `depends_on` edge resolves), AC-005 (the `related` edge resolves) |
| Static — backlog/KB health | `redkiln doctor` | AC-002. Expect exactly the six standing `template-drift` advisories and no seventh (`CLAUDE.md`, *Where the work lives*) |
| Static — immutability, asserted not intended | `git diff --exit-code HEAD~..HEAD -- .kb/decisions/0003-opaque-payloads.md` | AC-001. The mechanical form of AC-A09's *"unchanged in the diff"* (`_decomposition.md:563-568`) |
| Static — long form unmoved | `git diff --exit-code HEAD~..HEAD -- references/adr/0003-opaque-payloads.md` | AC-008, and the precondition for the next row |
| Static — specification cross-references | `cargo xtask spec-trace` (`xtask/src/spec_trace.rs`) | AC-008. SY-12's citation into the long form still resolves; no clause cites a rule that does not exist |
| Static — the five file-reading lints | `cargo xtask lints` (`xtask/src/lints.rs`) | Nothing new, and that is the point: `.redkiln/config.yaml`'s `reachability_static` runs it unconditionally per story, so a KB-only story is still read by something |
| Story grain — affected packages | `cargo xtask affected --base main` | NF-002 — resolves to no affected package, which is the mechanical statement that this diff touches no code |
| Textual — the atom says what it must | `rg -n "^supersedes:\|^depends_on:\|^kind:" .kb/decisions/0030-*.md .kb/reference/*.md` | AC-003, AC-005 |
| Textual — the index reflects it | `rg -n "ADR-0003\|ADR-0030" .kb/maps/decision-map.md`; `rg -n "ADR-0030" RUNBOOK.md` | AC-004, AC-008 |
| Textual — the citation is checkable | `rg -F` ADR-0003's lift sentence against ADR-0030; each cited test path exists under `crates/happenstance-sync-testkit/` | AC-006 |
| Textual — nothing was retired | `rg -n "happenstance-core\|bytes::Bytes\|envelope types only" .kb/decisions/0030-*.md` | AC-007 |
| Project integration grain | `cargo xtask ci --fast` | project DoD 1. Not this story's to make green, but it must not be made red — and a KB-only diff that reddens it means something outside the PR boundary moved |
| Ledger gate | `redkiln verify --grain story` over `_ledger.md` | project DoD 3 (`.redkiln/config.yaml`, `require_ledger: true`) — every AC-### carries cited evidence |

## Risks and coupling (PR-scoped)

| Risk | Note |
| --- | --- |
| **`supersedes` instead of `depends_on`.** | The one failure mode no gate catches: it validates, it doctors, the diff looks exemplary, and ADR-0003 is retired — leaving `CLAUDE.md` constraint 2 citing a superseded decision and nothing standing against `serde` in the contract crate. AC-003 and EC-003 are the whole guard, and the guard is a reviewer reading two frontmatter lines. |
| **Lifting on evidence that never looked at a byte.** | Structurally invited by this project: a decoded-value round trip is green against `InvertedHumanReadable`, the measured WF-11 branch (`spec/SPECIFICATION.md:2315-2322`). Coupling to HS-S0107 is what makes this survivable — the citation names *its* assertion and *its* negative control, so a weakened upstream assertion is visible here rather than laundered. AC-006. |
| **HS-S0107 slips or lands with the second arm.** | This story cannot proceed on its own evidence (EC-007) and must not manufacture any. If HS-S0107 records a non-byte-identical outcome, this story still merges — with EC-001's shape. The slice's merge order (`_storymap.md`, *Merge order* 4) exists for this. |
| **ADR-0030 collides with a concurrent wave.** | The number is unscheduled and the queue does not reserve it. Cheap to detect (EC-005), expensive to discover after the map rows and the long-form filename are written. Check the map and the queue immediately before authoring, not at planning time. |
| **The long form moves under SY-12.** | `spec-trace` resolves the citation by windowed subject search, so this fails *late* and reads as an unrelated gate break. EC-006. The mitigation is procedural: do not open `references/adr/0003-opaque-payloads.md`. |
| **Scope creep into ADR-0026/ADR-0027.** | Both merged in slice 1 and both are adjacent in subject. Reopening either to hold the lift would put a slice-4 measurement inside a slice-1 atom — the sequencing argument this spec settled (*Executive summary*, item 1) run backwards. |
| **`RUNBOOK.md` is the only non-KB file in the diff.** | It widens the PR boundary by one file and one file only. Any other `RUNBOOK.md` edit — ticking phase 13's exit checkbox, touching the ADR-0004 row at `:392`/`:4475` — is out of scope and belongs to the project's own exit. |

## Dependencies

**Blocks on**

- `byte-identical-round-trip-and-idempotent-replay` (HS-S0107) — hard, and the reason this story is
  last in the slice. It produces the byte-equality assertion, the replay witness and the conformant
  undecodable-payload control that AC-006 cites by path; without it there is nothing to cite and
  ADR-0030 would be asserting its own evidence (`_storymap.md`, *Merge order* 4).

Transitively, through HS-S0107: `message-set-on-the-envelope` (HS-S0106), and slice 1's
`adr-0026-peer-ingest-and-transport` / `adr-0027-merge-compensation-and-message-set`, which are the
atoms this one must **not** be folded into.

**Unlocks**

- No story in `_storymap.md` declares `depends_on: adr-0003-provisional-lift`, and that is correct:
  this is a record, not substrate. What it unblocks is the *project's exit* — it closes the third
  and last limb of project **AC-006**, satisfies project **DoD 5** and initiative **DoD 14**
  (`initiative.md:398-402`), and ticks `RUNBOOK.md:4613`'s exit criterion, whose second clause reads
  *"and ADR-0003 loses `provisional`"*. `spec-repairs-and-clause-exit` reads the corpus this story
  leaves behind; it does not consume an API from it.

## Anchors (progressive disclosure)

Everything load-bearing is stated in the *Context pack*. These are the artefacts to open at a
specific moment, and not before.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `.kb/decisions/0003-opaque-payloads.md` | The atom being amended. Its lines 87-95 carry the lift condition that AC-006 quotes verbatim; its frontmatter is what AC-001 and AC-003 assert about. | Before writing a single word of the intake documents — read it, then do not edit it. | AC-001, AC-006 |
| `.kb/decisions/0029-msrv-raised-to-1-97-1.md` | The in-tree worked precedent for amendment-not-supersession: `supersedes: null`, `depends_on: [kb-decision-0004]` (`:25-26`) and a summary saying so in its own words (`:12-14`). Copy the shape rather than invent one. | While drafting ADR-0030's frontmatter and summary. | AC-003 |
| `.kb/decisions/README.md` | `:7-13` is the immutability rule and the "only edit an accepted decision ever receives" sentence; `:35-38` is why the measurement lives in a `reference` atom rather than in the decision. | Before deciding how many documents to stage in `_intake`. | AC-001, AC-005 |
| `.kb/governance/rewrite-the-referent-never-the-reasoning.md` | The discriminator that survives someone arguing the marker is a trivial correction: an in-place rewrite is allowed only when it does not change what the document *asserts*. Deleting "and provisional" changes exactly that. | Only if anyone proposes editing ADR-0003 in place. Then it is the whole answer. | AC-001 |
| `.kb/playbooks/one-decision-per-adr-title.md` | The argument for standalone over folding into ADR-0026, and the test for whether a title has quietly become two decisions settled by different evidence. | While naming ADR-0030 and if the wave proposes a merge. | AC-004 |
| `.kb/reference/wire-format-encoding-measurements.md` | The worked `reference`-atom precedent — how ADR-0016's numbers are carried, sourced and dated. Shape to imitate. | While drafting the measurement document. | AC-005 |
| `.kb/reference/README.md` | The dating rule: a measurement with no commit sha or date *"is not a weaker reference, it is a false one"*. | Same moment as the row above; it is the acceptance bar for it. | AC-005 |
| `.kb/maps/decision-map.md` | The mount point. `:51` is ADR-0003's current row; `:65` and `:76-79` are the ADR-0029 → ADR-0004 amendment edge and the explanation of why `superseded_by` stays `null`. Also where the next free ADR number is checked. | Immediately before authoring (to check the number) and again when reviewing the wave's Maps phase output. | AC-004, AC-008 |
| `.kb/_intake/README.md` | The authoring path and what a staged document is expected to look like. Hand-authoring under `.kb/` was reverted once already (`0269720`). | Before creating the first file in `.kb/_intake/`. | AC-002 |
| `.kb/playbooks/anchoring-citations-in-a-long-lived-document.md` | Why `spec-trace` tolerates insertion above a cited line but not a moved or reworded sentence — the mechanism behind EC-006. | Before touching anything under `references/adr/`. | AC-008 |
| `references/adr/0003-opaque-payloads.md` | Lines 13-15 are the sentence SY-12 cites. Read-only: this file's stability is the acceptance condition, not its content. | Only to confirm it is unchanged, at review. | AC-008 |
| `spec/SPECIFICATION.md` | `:6153-6172` is SY-12 `[FROZEN]`, which quotes ADR-0003's lift condition from the long form and concludes a metadata-borne identity fails the ADR's own test; `:2315-2322` is WF-11's measured inverted branch — the codec that round-trips perfectly while being wrong, and the reason AC-006 names an assertion rather than a story. | When writing the evidence citation, and again if `spec-trace` fails. | AC-006, AC-008 |
| `RUNBOOK.md` | `:287` is the unscheduled-ADR-number precedent to copy; `:285-308` is the queue that gets the new row; `:4613` is phase 13's exit criterion whose second clause this story ticks (but does not tick *here*). | When adding the queue row, last. | AC-004 |
| `.bklg/from-contract-to-published-library/replication-identity-and-ingest/byte-identical-round-trip-and-idempotent-replay/spec.md` | The upstream story. Its *Behavior and interfaces* last-but-one row states that its `_ledger.md` carries rule name, test path and commit in a form this story can quote verbatim. Read the ledger; this spec explains what the ledger is for. | First, before drafting anything. | AC-006 |
| `.bklg/from-contract-to-published-library/replication-identity-and-ingest/_decomposition.md` | *Tension 5* (`:389-407`) is the brief's own framing of the amendment and the standalone/folded trade; AC-A09 (`:563-568`) is the ingest-path requirement in its mechanical form; `:625-663` is the testing brief's Static tier. | When the standalone choice is questioned, and when assembling the merge gate. | AC-002, AC-004 |
| `.bklg/from-contract-to-published-library/replication-identity-and-ingest/adr-0003-provisional-lift/discover.md` | The three named wrong implementations in full — the loud edit, the `supersedes` mutant, and the lift-on-decoded-values mutant — each with the reason it is plausible. | At review, as the checklist for what this PR must not be. | AC-003, AC-006 |
| `.bklg/from-contract-to-published-library/replication-identity-and-ingest/_design.md` | The signed-off determination that this project renders no user-facing surface, with the verification behind it. It is why the *Interaction quality* composition family is N/A rather than skipped. | Only if someone asks for a surface obligation here. | AC-008 |
| `.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md` | `:249-317` is Persona 4, the evaluator who cannot run the suite and judges the project by what it published — the reader every AC in this spec is framed for, and the source of NF-004. | While writing ADR-0030's body, to check it reads as evidence rather than as an internal note. | AC-005 |
| `CLAUDE.md` | Binding constraint 2, and the caution that after ADR-0006 it constrains **`happenstance-core`** while `happenstance` is the crate that *will* depend on `serde`. Getting this backwards lands the wrong decision. | While writing AC-007's "nothing is retired" paragraph. | AC-007 |

## Clarifications resolved during spec

1. **Standalone atom, not folded into ADR-0026** — the brief's *Tension 5* recommended riding
   inside ADR-0026 and called standalone "equally legitimate". Settled for standalone on an argument
   the brief does not make: ADR-0026 merges in slice 1 and this evidence does not exist until slice
   4, so an ADR-0026 asserting the lift would cite a measurement not taken when it was accepted.
   Cost accepted and paid: one unscheduled ADR number, plus a `RUNBOOK.md` queue row (AC-004) so the
   number is not invisible.
2. **The number is ADR-0030**, on ADR-0029's unscheduled-number precedent (`RUNBOOK.md:287`).
   EC-005 covers a collision; the number is re-checked at authoring time, not trusted from planning.
3. **Amendment, not supersession, and ADR-0003 receives no edit at all** — not even the metadata
   flip `.kb/decisions/README.md:9-13` calls "the only edit an accepted decision ever receives". An
   amendment does not earn it, and ADR-0029 → ADR-0004 is the shape in the tree.
4. **Two staged documents, not one** — a `reference` atom for the measurement and a `decision` atom
   for the lift (`.kb/decisions/README.md:35-38`). EC-004 records that a wave-adjudicated merge is
   acceptable; the sha, the date and the named assertion are not negotiable either way.
5. **The citation names the assertion and the negative control, never the AC id.** "AC-006 green" is
   equally true of a decoded-value comparison; the byte-equality assertion at the receiver after
   commit, the replay witness and `PayloadTouchingSuite` passing are claims a reader can check.
6. **`RUNBOOK.md` is in the PR boundary, for one row only.** Everything else in that file — the
   phase-13 exit checkbox at `:4613`, the ADR-0004 row at `:392`/`:4475` — stays out.
7. **Interaction quality is answered, not skipped.** The composition family is N/A on the signed-off
   `_design.md`; the state family has exact referents in the corpus and every applicable invariant is
   carried by a numbered AC (AC-001, AC-003, AC-004, AC-005, AC-006, AC-008), not by a prose bullet.
8. **AC ids are exactly AC-001 – AC-008 as the front half enumerated them.** None added, none
   dropped; the ledger matches one-for-one.
9. **The second arm is inside AC-006 rather than a ninth AC**, because it is the same criterion's
   other ending — the same atom, the same ingest path, the same map rows — and splitting it would
   let a story satisfy the "lift" AC while leaving the "record why not" AC unsatisfied, or the
   reverse. EC-001 carries the operational detail.
