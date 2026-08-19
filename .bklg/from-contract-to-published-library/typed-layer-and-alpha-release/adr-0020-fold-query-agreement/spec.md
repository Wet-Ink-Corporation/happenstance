---
item: HS-S0018
stage: spec
created: 2026-08-12T13:46:13.343Z
updated: 2026-08-12T13:46:13.343Z
template_sig: 87bbf1d0
rendered_sig: 4ba1afb6
---

# Spec — ADR-0020 — fold/query agreement, and DT-2's signature answer

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` |
| Project | `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/project.md` |
| This spec | `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/adr-0020-fold-query-agreement/spec.md` |
| Key briefs | `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_decomposition.md` (*The one signature question DT-2 actually is*; *The ADR route, and who invokes it*; Testing brief AC-014/AC-016 rows) |
| Signed-off design (**binding**) | `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_design.md` — `## Shape decision`, `## Signatures`, `## Visibility and stability`, `## The doctest`, `## Sign-off` (Approved, 2026-08-12) |
| Story map (this row) | `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_storymap.md:49` |
| Roadmap pointer | `RUNBOOK.md:299` — ADR-0020, phase 7: *"How does a decision model guarantee that its query and its fold cannot disagree?"*; `RUNBOOK.md:157` (phase 7's row and proof artefact) |
| KB corpus rules | `.kb/decisions/README.md` (the immutability rule), `.kb/_intake/README.md` (the ingest contract) |

## One-line PR slice

Stage into `.kb/_intake/` the decision that a decision model folds a domain enum and *derives* its `Query` from `EVENT_TYPES` + tag constraints — resolving DT-2's concrete form, *is `query()` infallible and where did the validation go* — and hand off to `/redkiln:kb-ingest` so ADR-0020 exists as an atom before the code it governs.

## Executive summary

This PR lands **one staged document and one durable record, and no Rust**.

The delta is narrow and entirely in the knowledge base. `.kb/decisions/` today holds
seventeen atoms — `0001`…`0016` and `0029` — and there is no `0020`. `RUNBOOK.md:299`
has been holding a slot for it since the plan was written. `_design.md` answered the
question at the design gate and a human signed it off on 2026-08-12; what does not yet
exist is the *record*: the immutable artefact that says which alternatives lost, so the
next person to meet this fork knows it was a fork and not an accident
(`.kb/decisions/README.md`, *What belongs here*).

So this story writes the decision down, in the two forms the corpus keeps on purpose —
a staged intake document that `/redkiln:kb-ingest` turns into the atom, and the
long-form record under `references/adr/` that survives the intake directory being
cleared — and then **stops**. It writes no trait, no signature, no doctest. `M2`'s
`domain-event-and-decision-model` is the shape this record governs, and AC-016's
"written first" is the only reason this milestone exists at all: an accepted atom is
immutable, so an ADR written *after* the code records what was built instead of
deciding it, and every later correction becomes a second atom
(`_decomposition.md`, *The ADR route, and who invokes it*).

Pointer, not restatement: the project charter's scope, ACs and risks are at
`project.md`; the surface this decision governs is at `_design.md`. Neither is
re-litigated here.

## Context pack

**Read this section and you can start. Everything deeper is an anchor below.**

### The hazard this decision closes

A DCB handler names its event set **twice** — once in the query it reads with, once in
the fold that interprets what came back — and nothing checks that the two agree. That is
not hypothetical here: `examples/course-subscriptions/src/main.rs:114-125` builds a
two-item `Query` naming `COURSE_DEFINED`, `STUDENT_SUBSCRIBED`, `STUDENT_UNSUBSCRIBED`,
and the fold immediately below it interprets those same names again, with no compiler,
clippy or conformance signal against a divergence between them. A query that selects an
event the fold ignores silently weakens a consistency boundary. **This is the defect
ADR-0020 exists to make unwritable**, and the decision is that the query is *derived*
and there is nowhere to put a hand-maintained one (project `DR-01`, `DR-02`).

### The decision this story records (already made, at the design gate)

`_design.md`'s `## Shape decision` resolved DT-2 toward **explicit declaration**, and the
sign-off names that resolution as one of three things the approver was specifically asked
to accept. This story does **not** re-decide it. The record must state it as the
following, and must not contradict any row of it:

1. **`query()` is not on `DecisionModel`.** The derivation is `Boundary::query`, on a
   **sealed** trait blanket-implemented for every `DecisionModel` and macro-implemented
   for tuples of arity 2..=8. A *provided method* on `DecisionModel` loses because a
   caller can override it, and an overridden derivation is a hand-maintained query —
   precisely `DR-02`'s prohibition. A free `derive_query::<M>()` loses because a caller
   can ignore it and build their own.
2. **`Boundary::query` returns `Result<Query, InvalidQuery>`**, and an *empty*
   `EVENT_TYPES` is a **compile error**, not a runtime one. Infallible is unreachable
   without an `unwrap`: `QueryItem::new` returns `Result` (`crates/happenstance-core/src/query.rs:56`)
   and `happenstance-core` is frozen. Fallible *without* the const assertion loses too —
   it pushes a compile-time-decidable mistake to the first read, which is DT-2's
   minimal-ceremony pole.
3. **`DecisionModel::scope(&self) -> &Tags`** — the model *holds* validated tags.
   `Tags::from_pairs` is fallible and is the only way in
   (`crates/happenstance-core/src/tag.rs:304`), so the validation is paid **once**, in the
   constructor the caller already writes. Returning `Tags` by value from an infallible
   signature would force an `unwrap`; taking `&[(&str, &str)]` would revalidate on every
   read and move the error to the read.
4. **`DecisionModel: Clone`, not `Default`.** `Default` would force `scope`'s validated
   `Tags` into a default-constructible field and re-open the invalid-value hole;
   `Clone` lets the command loop re-fold from the *pristine* model on retry.
5. **Composition is `impl Boundary for (B1, B2)`…** by an *internal* `macro_rules!`,
   arity 2..=8 — zero caller-visible syntax, because a macro in the caller's face is the
   ceremony DT-2 is being measured on.

### Where the fallibility went — and the residual, which is a recorded defect, not a shrug

`Boundary::query` builds a `QueryItem` out of values that are *already* validated, and
still meets a `Result`. The record must say what was done about that, because it is the
single most likely thing for an implementer to "fix" wrongly:

- **`Err` is kept and given a real meaning.** `InvalidQuery::UnconstrainedItem`
  (`crates/happenstance-core/src/error.rs:99-102`) becomes *"this boundary constrains
  nothing, which is `Query::all()` and must be said out loud"* — a refusal worth making.
- **The other route to it — an empty `EVENT_TYPES` — becomes a compile error**, via a
  `const` item evaluated per-monomorphisation (`standards/rust/61-compile-time-assertions.md:249`,
  RS-61-4).
- **`commit`/`commit_with` absorb the `Result`** into `CommandError::Boundary`, so the
  first program writes no extra `?` for it.
- **No `unwrap`. No edit to `happenstance-core`.** The shortfall is logged as
  **defect candidate D-1** — *`happenstance-core` has no infallible `QueryItem`
  constructor for pre-validated inputs* — routed to a decision record under AC-012's
  route, never a line edit of a frozen crate (project AC-012; `_design.md`, *The
  residual*).

### The two-constructor trap, stated in the contract's own words

The brief's instruction is that the answer be **one** shape, not two. The contract crate
already names why, in prose, about its own `ProjectionId`: *"two constructors enforcing
different rules is the defect that makes an invalid value reachable through the weaker
one"* (`crates/happenstance-core/src/projection.rs:47-61`). A record that admits both an
infallible and a fallible path has not resolved DT-2; it has deferred it into the type
system.

### What "written first" actually obliges

`/redkiln:kb-ingest` is **human-invoked** and is a handoff, not a step inside this
story's implementation (`_decomposition.md`, *The ADR route, and who invokes it*;
`CLAUDE.md`, *Where the work lives*). Atoms are authored **by the ingest path, never by
hand** — hand-writing them produces "the directory layout of the process without the
process", which is why the first attempt at this was reverted at `0269720`. So the unit
of work here is the **staged document**, and the atom is the wave's output.

Two consequences follow, and both are acceptance criteria below:

- **A successful ingest clears `_intake`** (`.kb/_intake/README.md:13-19`). The staged
  document is therefore *not* a durable home for the reasoning. `CLAUDE.md` states the
  corpus's two-places-on-purpose rule directly: the atom is ~100 lines, and the long
  record carries the transcripts, the rejected alternatives and the tables a summary
  cannot hold — deleting the second "would discard about 78% of the corpus". Hence
  `references/adr/0020-fold-query-agreement.md`, which is exactly where ADR-0029's atom
  points its own `source_paths` (`.kb/decisions/0029-msrv-raised-to-1-97-1.md`).
- **The atom, once accepted, is immutable.** A correction after M2 is written is a
  *second, superseding atom* — never an edit (`.kb/decisions/README.md`, *The
  immutability rule*, checked against `HEAD` by `redkiln validate --kb`).

### The persona-journey slice

Activity **A1** of this project's backbone: *"Decide the shape before writing it — the
two answers this layer is built on exist as records naming the alternatives that lost"*
(`_storymap.md:23`). The reader served is not the `cargo add` application author — it is
the maintainer six months out who meets the same fork, plus the implementer of M2 who
needs to know which shape is licensed. The audience constraint that gives DT-2 its weight
is still in force and belongs in the record's reasoning: the repository owner and the
audience are **new to idiomatic Rust** (`CLAUDE.md`, *Who you are working with*), so
ceremony that is cheap for a Rust expert is the first-hour cost — and the design accepted
a measured **2.4:1** ceremony-to-domain ratio in the first program (`_design.md:1105-1109`)
with the falsifiable prediction that AC-013's verdict will be *"`happenstance-macros` is
in scope for 0.1"*. That prediction belongs in the record as a consequence, not as a
separate decision.

### What this story must not do

Write Rust. Hand-author `.kb/decisions/0020-*.md`. Re-open a signed-off `_design.md` row.
Decide ADR-0021's codec-tag home (the slice-mate's, and the public surface is invariant
under it). Edit any `[FROZEN]` clause in `spec/SPECIFICATION.md`.

## Integration contract

- **Archetype**: `foundation` — real in-tree substrate (a decision record) consumed by
  the capability slice `domain-event-and-decision-model` in this same project. Not a
  double, not a fixme.
- **Slice / milestone**: **M1 `decision-records`** (`_storymap.md:49-50`, merge order
  step 1). **Slice-mate**: `adr-0021-payload-evolution-and-codec-tag` — implemented in
  the same context and staged into the **same** `/redkiln:kb-ingest` wave, which is the
  integrated surface this milestone mounts.
- **Mount point**: **`.kb/_intake/0020-fold-query-agreement.md`** — the intake directory
  is the KB's real composition root and the *only* input path to `/redkiln:kb-ingest`
  (`.kb/_intake/README.md:3-5`: *"the default input to `/redkiln:kb-ingest`"*). A
  decision document that lands anywhere else is unmounted by construction: it is never
  ingested, never becomes an atom, and never appears in the corpus. The wave's output —
  `.kb/decisions/0020-fold-query-agreement.md`, its row in `.kb/maps/decision-map.md`
  and its entry in `.kb/maps/domain-map.md` — is written by the ingest run on its own
  worktree branch, not by this PR.
- **Wires into**:
  - `.kb/decisions/0003-opaque-payloads.md` — payloads stay opaque `Bytes` at the port;
    the derived `Query` never depends on payload shape.
  - `.kb/decisions/0006-bare-name-to-the-typed-layer.md` — why this decision's subject
    lives in `happenstance` and not in the contract crate.
  - `.kb/decisions/0007-projection-runner-decodes.md` — `Query` is the *only* nomination
    vocabulary; the derivation must not invent a second filter language.
  - `crates/happenstance-core/src/query.rs` (`QueryItem::new`, `Query::from_items`),
    `crates/happenstance-core/src/tag.rs` (`Tags::from_pairs`, `Tag::from_static`),
    `crates/happenstance-core/src/event.rs:104-112` (`EventType::from_static`, `const`),
    `crates/happenstance-core/src/error.rs:95-105` (`InvalidQuery`) — the frozen
    constructors the decision is constrained by and does not change.
  - `standards/rust/40-public-surface-and-evolution.md:73` (RS-40-2, the blanket ext
    trait) and `standards/rust/61-compile-time-assertions.md:249` (RS-61-4, put the claim
    in a `const` item) — the two house rules the chosen shape is built on.
- **Renders surfaces**: **none.** This story renders no surface id from `_design.md`'s
  `## Surfaces` manifest. It *governs* `first-program-doctest` and the `compile_fail`
  companion on `DomainEvent`'s item page, both of which `domain-event-and-decision-model`
  renders in M2. The record must therefore stay consistent with `_design.md`'s
  `## The doctest`; it must not restate it.
- **Conformance rule(s)**: **not adapter-observable, and deliberately.** `DecisionModel`,
  `Boundary` and the derived `Query` live entirely above the port in `happenstance`; no
  store can observe whether a query was derived or hand-written, so there is no rule in
  `crates/happenstance-testkit/src/suite.rs` this story could add that any adapter could
  fail. Adding one would be decorative (`CLAUDE.md`, *A rule that no adapter can fail is
  decorative*). The instrument is `redkiln validate --kb`, plus M2's unit test of the
  chosen failure mode (`_decomposition.md`, Testing brief, AC-014 row).
- **Clause(s)**: **none discharged, none amended.** No `spec/SPECIFICATION.md` clause is
  touched by this story, and none may be — a defect this decision surfaces (D-1) routes
  to a decision record, never to a line edit (project AC-012, AC-A02). Defect candidate
  D-1's nearest clause subject is `VT-18` (*constructors accept values the caller already
  holds, and their errors compose*); the record names it as context, not as an amendment.
- **Advances DoD scenario**: initiative **DoD 2** — *"@smoke — the compiler protects the
  domain"* (`initiative.md:363-365`). This story decides the shape that makes the
  protection structural; M2 builds it and M6's `compile-fail-proof-artefact` observes it.
  It also directly discharges the project's own **DoD 3** (*ADR-0020 and ADR-0021 are
  accepted atoms under `.kb/decisions/`, written before the code they govern, and
  `redkiln validate --kb` is clean*) jointly with its slice-mate, and unblocks
  initiative **DoD 1** through M2 → M6.

## PR boundary

**In this PR**

- The staged decision document at `.kb/_intake/0020-fold-query-agreement.md`.
- The long-form record at `references/adr/0020-fold-query-agreement.md`, which the staged
  document cites and which survives `_intake` being cleared by the wave.
- This story's own backlog folder — its `_ledger.md` and implementation report.

**Explicitly not in this PR**

- **Any file under `crates/`, `examples/` or `xtask/`.** The whole point of M1 is that no
  code exists yet for the record to describe (AC-016, "written first").
- **`.kb/decisions/0020-fold-query-agreement.md` and the map atoms.** Those are written by
  `/redkiln:kb-ingest`, on its own worktree branch, from the staged input. Hand-authoring
  an atom is the reverted anti-pattern (`0269720`).
- **`spec/SPECIFICATION.md`**, in any form.
- **ADR-0021's subject matter** — payload evolution, versioned `EventType`, upcasting, and
  the codec tag's home. Slice-mate's, staged in the same wave; the public surface is
  invariant under that choice (`_design.md`, *Open, and deliberately not settled here*).
- Re-deciding any row of the signed-off `_design.md`.

**Allowed globs** — `redkiln verify --grain story` reads the first fenced block under
this heading and fails on any file changed outside it.

```
.kb/_intake/0020-fold-query-agreement.md
references/adr/0020-fold-query-agreement.md
.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/adr-0020-fold-query-agreement/**
```

**Merge DoD one-liner** — the staged document and its long-form record state one shape
for `Boundary::query`/`DecisionModel::scope` that matches `_design.md` row for row, name
the alternatives that lost and where the fallibility went, and carry everything
`/redkiln:kb-ingest` needs to mint a conformant atom; `cargo xtask ci --fast` and
`redkiln doctor` stay green (nothing compiled changed).

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| A staged decision document exists at the ingest mount point | `.kb/_intake/0020-fold-query-agreement.md`, titled for ADR-0020, answering `RUNBOOK.md:299`'s question in its own words. Raw material, **not** held to `KbFrontmatter` — `redkiln validate --kb` skips `_`-prefixed directories by design | `.kb/_intake/README.md:3-5`, `:21-27` |
| The decision is stated as one shape, not a menu | `Boundary::query(&self) -> Result<Query, InvalidQuery>` on a **sealed** trait, blanket-implemented for every `DecisionModel` and macro-implemented for tuples 2..=8; `DecisionModel::scope(&self) -> &Tags`; `DecisionModel: Clone` | `_design.md` `## Shape decision` rows 1–5; `_design.md` `## Signatures` (`pub trait Boundary: sealed::Sealed`) |
| Empty `EVENT_TYPES` is a compile error, not a runtime one | A `const` item evaluated per-monomorphisation; the reader is meant to see a post-monomorphisation const-eval error naming the event-type set, not a trait-resolution error | `standards/rust/61-compile-time-assertions.md:249` (RS-61-4); `_design.md` `## The doctest`, *The `compile_fail` companion* |
| The record says **where the validation went** | Paid once, in the model's constructor, because `Tags::from_pairs` is fallible and is the only way in; not on every read, and never behind an `unwrap` | `crates/happenstance-core/src/tag.rs:304`; `crates/happenstance-core/src/query.rs:50-60` |
| The residual `Result` is explained and kept | `InvalidQuery::UnconstrainedItem` is re-read as *"this boundary constrains nothing"*; `commit`/`commit_with` absorb it into `CommandError::Boundary` so the first program writes no extra `?` | `crates/happenstance-core/src/error.rs:99-102`; `_design.md`, *The residual* |
| Defect candidate **D-1** is recorded and routed | *`happenstance-core` has no infallible `QueryItem` constructor for pre-validated inputs.* Named in the record with its routing to a decision record; **never** a line edit of the frozen crate | project `AC-012`; `_design.md`, *Defect candidate D-1* |
| The alternatives that lost are named, each with its failure mode | Provided method on `DecisionModel` (overridable ⇒ hand-maintained query); free `derive_query::<M>()` (ignorable); infallible `query()` (needs `unwrap`); fallible-without-const-assertion (defers a compile-time-decidable mistake); `scope() -> Tags` by value (`unwrap` inside an infallible signature); `&[(&str, &str)]` (revalidates, moves the error to the read); `Default` supertrait (re-opens the invalid-`Tags` hole); exported `compose!` macro (caller-visible ceremony) | `.kb/decisions/README.md`, *What belongs here*; `_design.md` `## Shape decision`, *Rejected* column |
| One shape, not two constructors | The record states explicitly that both poles were live and that exactly one is licensed, in the contract's own words about the same defect class | `crates/happenstance-core/src/projection.rs:47-61`; `_decomposition.md`, *The one signature question DT-2 actually is* |
| The hazard is stated against real code, not in the abstract | The twice-named event set in the worked example today: the `Query` at `:114-125` and the fold that re-interprets the same names below it, with no signal against divergence | `examples/course-subscriptions/src/main.rs:114-173` |
| The consequences carried are DT-2's cost, honestly | Explicit declaration is the pole chosen; the measured first-program ratio is 2.4:1 ceremony to domain, recorded as the falsifiable prediction that AC-013 lands *"in scope"* — a consequence of this decision, not a second decision | `_design.md:1105-1109`; `RUNBOOK.md:524` |
| The long-form record outlives the wave | `references/adr/0020-fold-query-agreement.md` carries the full reasoning; the atom will cite it in `source_paths`, exactly as `ADR-0029`'s does | `CLAUDE.md`, *Where the work lives* (two places on purpose); `.kb/decisions/0029-msrv-raised-to-1-97-1.md` (`source_paths`) |
| The staged document carries what the wave needs | Intended `adr_id: ADR-0020`, `kind: decision`, `authority_tier: decision`, `status: accepted`, `phase: 7`, a stated `reversibility`, `depends_on`/`related` naming real atom ids (`kb-decision-0003`, `kb-decision-0006`, `kb-decision-0007`), and `source_paths` that all resolve — proposed *for* the ingest run, which authors the frontmatter | `.kb/decisions/0029-msrv-raised-to-1-97-1.md` (frontmatter shape); `.kb/decisions/README.md` |
| Ingest is a human handoff, not an implementation step | The story ends at "staged and ready"; `/redkiln:kb-ingest` is invoked by a human, in one wave with the slice-mate, on its own worktree branch, and it clears `_intake` | `_decomposition.md`, *The ADR route, and who invokes it*; `.kb/_intake/README.md:13-19` |
| Post-wave, the atom is real and reachable | `.kb/decisions/0020-fold-query-agreement.md` exists, `.kb/maps/decision-map.md` carries its row, and `redkiln validate --kb` is clean — verified as an explicit acceptance step, not an automated assertion inside this PR | `.kb/maps/decision-map.md`; project DoD 3 |
| Nothing compiled changes | No file under `crates/`, `examples/`, `xtask/` or `spec/` is touched; the affected-gate and `cargo xtask ci --fast` are unchanged by construction | `.redkiln/config.yaml` (`affected_gate`, `integration_scoped`) |

## Data and migrations

**N/A — no schema, no store, no persisted data.** This story writes two markdown files
and changes no runtime artefact.

The one migration-shaped obligation worth naming, so it is not mistaken for one: the
`_intake` → `.kb/decisions/` transition is a **process handoff performed by
`/redkiln:kb-ingest`**, not a data migration this story executes. It is one-way and
effectively irreversible in the sense that matters — once the atom is accepted its body
is immutable and a correction is a new superseding atom carrying `supersedes:`, with the
old atom's frontmatter flipped to `status: superseded` + `superseded_by:`. That metadata
flip is the only edit an accepted decision ever receives (`.kb/decisions/README.md`, *The
immutability rule*). Which is exactly why the wording is worth getting right **before**
the wave runs rather than after M2 has been written against it.

## Acceptance criteria

The reader served by every row below is one of two people, and neither is the
`cargo add` application author directly: **the M2 implementer**, who needs to know which
shape is licensed before writing `DecisionModel`, and **the maintainer six months out**,
who meets the same fork and must be able to tell it *was* a fork
(`.kb/decisions/README.md`, *What belongs here*: *"a decision recorded without its
rejected options is indistinguishable from an accident"*). P1 — the application author
(`_decomposition.md:47`) — is served transitively: the shape this record licenses is the
shape their first hour costs them.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **GIVEN** the M2 implementer is about to write `DecisionModel` and needs the licensed shape, **WHEN** they look for ADR-0020, **THEN** a staged decision document exists at `.kb/_intake/0020-fold-query-agreement.md`, composed as a decision record — hazard/context, the decision, the alternatives that lost, the consequences, the residual — and carrying a **proposed** frontmatter block for the ingest run to author from (`adr_id: ADR-0020`, `kind: decision`, `authority_tier: decision`, `status: accepted`, `phase: 7`, a stated `reversibility`, `depends_on`/`related` naming the real atom ids `kb-decision-0003`/`kb-decision-0006`/`kb-decision-0007`, and `source_paths` that all resolve); it lands at no other path, because `_intake` is `/redkiln:kb-ingest`'s only input. | Static. `test -f .kb/_intake/0020-fold-query-agreement.md`; `cargo xtask affected --base main` (the story-grain gate, which for a no-package diff runs the five file-reading lints and `spec-trace`); allowed-globs check by `redkiln verify --grain story` against this spec's *PR boundary* fence. Frontmatter shape reviewed against the precedent at `.kb/decisions/0029-msrv-raised-to-1-97-1.md:1-30`. `redkiln validate --kb` deliberately does **not** cover it — `_`-prefixed directories are skipped by design (`.kb/_intake/README.md`, *`_`-prefixed directories are reserved*). |
| AC-002 | **GIVEN** a human approved `_design.md` on 2026-08-12 with DT-2 resolved toward explicit declaration, **WHEN** the record states the decision, **THEN** it states **one** shape and it matches that design row for row — `Boundary::query(&self) -> Result<Query, InvalidQuery>` on a **sealed** trait blanket-implemented for every `DecisionModel` and macro-implemented for tuples of arity 2..=8; `DecisionModel::scope(&self) -> &Tags`; `DecisionModel: Clone`, not `Default`; an *empty* `EVENT_TYPES` a **compile error** via a `const` item evaluated per-monomorphisation — and it contradicts no row of the signed-off design and re-decides none of them. | Human review at the story's report gate, side by side against `_design.md` `## Shape decision` (the five rows named in this spec's *Context pack*), `## Signatures` and `## Sign-off`. A divergence is a defect in this story, never a design change. Consequence-side proof is M2's: the testing brief's AC-014 row (`_decomposition.md`, Testing brief) requires a unit test asserting the *chosen* failure mode once the code exists. |
| AC-003 | **GIVEN** a DCB handler names its event set twice — once in the query, once in the fold — and nothing checks the two agree, **WHEN** the maintainer asks *why this trait and not a provided method*, **THEN** the record states the hazard against **real code in this tree**, citing the two-item `Query` at `examples/course-subscriptions/src/main.rs:114-125` and the fold below it that re-interprets the same names, and states the resolution structurally: the query is **derived** and there is nowhere to put a hand-maintained one (`DR-01`, `DR-02`). | Human review; every `file:line` citation in the record is re-opened and confirmed to say what is claimed before sign-off — the citation-repair discipline `_design.md` `## Sign-off` performed on itself. Reviewer opens `examples/course-subscriptions/src/main.rs:114-173`. |
| AC-004 | **GIVEN** `Boundary::query` builds a `QueryItem` out of values that are already validated and *still* meets a `Result`, **WHEN** the M2 implementer reaches for the obvious "fix", **THEN** the record has already answered where the validation went and what was done about the shortfall: paid **once** in the model's constructor because `Tags::from_pairs` is the only (fallible) way in; `Err` kept and re-read as `InvalidQuery::UnconstrainedItem` = *"this boundary constrains nothing"*; the other route to it turned into a compile error; `commit`/`commit_with` absorbing the `Result` so the first program writes no extra `?`; **no `unwrap`, no edit to `happenstance-core`**; and the shortfall logged as **defect candidate D-1** with its routing to a decision record and `VT-18` named as its nearest clause subject. | Human review against `crates/happenstance-core/src/tag.rs:304`, `crates/happenstance-core/src/query.rs:56`, `crates/happenstance-core/src/error.rs:99-102` and `_design.md`, *The residual*. Negative assertion, mechanical: `git diff --name-only` for this PR shows **no** path under `crates/`, `examples/`, `xtask/` or `spec/` (project AC-A02, AC-012). |
| AC-005 | **GIVEN** the maintainer six months out meets this fork again, **WHEN** they read the record, **THEN** every alternative that lost is named **with the wrong implementation it admits** — provided method on `DecisionModel` (overridable ⇒ a hand-maintained query); free `derive_query::<M>()` (ignorable); infallible `query()` (unreachable without an `unwrap`); fallible-without-the-const-assertion (defers a compile-time-decidable mistake to the first read); `scope() -> Tags` by value (`unwrap` inside an infallible signature); `&[(&str, &str)]` (revalidates, moves the error to the read); a `Default` supertrait (re-opens the invalid-`Tags` hole); an exported `compose!` macro (caller-visible ceremony) — and the record says explicitly that **exactly one** path is licensed, in the contract crate's own words about the same defect class. | Human review; the enumeration is checked to be complete against `_design.md` `## Shape decision`'s *Rejected* column, and the one-shape claim against `crates/happenstance-core/src/projection.rs:47-61` (*"two constructors enforcing different rules is the defect that makes an invalid value reachable through the weaker one"*) and `_decomposition.md`, *The one signature question DT-2 actually is*. |
| AC-006 | **GIVEN** the audience is fluent in the domain and **new to idiomatic Rust** (`CLAUDE.md`, *Who you are working with*), so ceremony is the first-hour cost, **WHEN** the record states what this shape costs, **THEN** it carries DT-2's price honestly: the measured **2.4:1** ceremony-to-domain ratio in the first program is recorded as a **consequence with a falsifiable prediction** — that AC-013's verdict lands *"`happenstance-macros` is in scope for 0.1"* — and **not** as a second decision the record is taking on the derive's behalf. | Human review against `_design.md:1105-1109` and `RUNBOOK.md:524`; the record is checked to contain no commitment (*must*/*shall*/*must not*) beyond DT-2's resolution, and to name AC-013's verdict as a later obligation owned by the project's closeout (project DoD 8), not settled here. |
| AC-007 | **GIVEN** a successful ingest **clears** `_intake`, so the staged document is not a durable home for the reasoning, **WHEN** the wave runs, **THEN** the long-form record already exists at `references/adr/0020-fold-query-agreement.md` carrying the compiler-facing detail, the rejected alternatives and the trade tables a ~100-line atom cannot hold, and the staged document names it in the `source_paths` it proposes — exactly the two-places-on-purpose pairing `ADR-0029` already demonstrates. | Static + review. `test -f references/adr/0020-fold-query-agreement.md`; the staged document's proposed `source_paths` lists both `.kb/_intake/0020-fold-query-agreement.md` and `references/adr/0020-fold-query-agreement.md`, matching `.kb/decisions/0029-msrv-raised-to-1-97-1.md`'s own `source_paths`; reviewed against `CLAUDE.md`, *Where the work lives* (atom ≈ 100 lines, long record up to ~1,508 — deleting the second would discard about 78% of the corpus). |
| AC-008 | **GIVEN** AC-016 is a *sequencing* obligation — the record must exist before the code it governs — and atoms are authored by the ingest path and never by hand, **WHEN** a human runs `/redkiln:kb-ingest` over this document and its slice-mate in **one wave** on its own worktree branch, **THEN** `.kb/decisions/0020-fold-query-agreement.md` exists as an accepted atom, `.kb/maps/decision-map.md` carries its row, `redkiln validate --kb` and `redkiln doctor` are clean, and `git log --diff-filter=A` shows that atom added by the **wave's** commit and not by this story's PR. | Handoff-verified, and deliberately outside this PR's diff. `redkiln validate --kb && redkiln doctor` run on the ingest branch; `git log --diff-filter=A -- .kb/decisions/0020-fold-query-agreement.md` names the wave commit. Ledger evidence for this row is that commit sha plus the validate output. This is the project's DoD 3, discharged jointly with the slice-mate (`project.md`, *Definition of done*, item 3). |

**Traceability.** Project **AC-001** ← AC-002, AC-003, AC-005. Project **AC-014** ←
AC-002, AC-004, AC-005, AC-006. Project **AC-016** ← AC-001, AC-007, AC-008. No project
AC traced by this story is left without a row, and no row here reaches outside those
three.

## Interaction quality

**This story renders no surface id from `_design.md`'s `## Surfaces` manifest.** The
six surfaces there — `crate-root-rustdoc`, `crate-readme`, `first-program-doctest`,
`worked-example-transcript`, `dsl-failure-message`, `compile-fail-diagnostic` — are all
routed at `crates/` or `examples/` paths this PR may not touch, and are mounted by M2–M6.
So the design's **rendered-surface** budgets (72-column doc fences, ≤ 12 prose lines to
the first fence, ≤ 35-line first program, 80-column transcript) do not bind this
deliverable; they bind the stories that render those surfaces, and this record must not
restate them.

What *does* bind here is two things: (a) `_design.md`'s `## Shape decision`, `## The
doctest` and `## Anti-patterns` as **content this record may not contradict**, and (b) the
KB corpus's own composition rules, which are this artefact's real presentation layer. Both
families are carried as AC rows in the table above — none of the invariants below is a
prose-only bullet, because `redkiln verify` extracts ACs from table cells and a bullet here
would never be gated.

**STATE invariants** — read in this medium, where "the surface" is the record a reader
opens and "state" is what survives the ingest wave.

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **In place, not a context jump** — the record answers its own question where the reader is, and does not defer the decision to another document (the `AC-U10` rule, *"a doc comment that says 'see the specification' for the policy fails this"*, `_decomposition.md:155-163`) | **AC-002**, **AC-004** | Review: the record states the shape and where the fallibility went in its own words. Citing `_design.md` as *evidence* is correct; citing it *instead of stating the decision* fails the row |
| **Non-occlusion — a record must not hide what it filtered** — the direct analogue of `AC-U11`: a decision that shows only the winner has occluded its own alternatives and is indistinguishable from an accident | **AC-005**, **AC-003** | Review: the eight rejected shapes are enumerated with the wrong implementation each admits, and the hazard is shown against real code rather than described abstractly |
| **Preserved state across the handoff** — the wave preserves the audit trail: the slice-mate's document is ingested in the **same** wave, each atom's `source_paths` still resolves after `_intake` is cleared, and a second wave does not overwrite the first's trail | **AC-007**, **AC-008** | `source_paths` resolution checked before the wave; after it, `git log --diff-filter=A` and `redkiln validate --kb` |
| **Reversibility, and its real cost** — this is the one invariant that *inverts* here. An accepted atom is immutable, so the record is one-way: reversal is a **new superseding atom**, never an edit, and D-1's route is a decision record, never a line edit of a frozen crate | **AC-004**, **AC-008** | Review: the record states its own `reversibility` in the proposed frontmatter and names the supersession route; `redkiln validate --kb`'s accepted-decision-immutability check against `HEAD` is the enforcement |
| **Reachability without a search engine** (`AC-U14`'s analogue) — the atom is reachable from `.kb/maps/decision-map.md` and the long record from the atom's `source_paths`; nothing load-bearing is findable only by grep | **AC-008**, **AC-007** | The decision-map row and the resolving `source_paths` are both explicit acceptance steps |

**COMPOSITION invariants** — taken from `_design.md` where it governs content, and from
the corpus rules where it governs this artefact's own shape.

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **Presentation exists at all** — the deliverable is a *composed decision record* (hazard, decision, rejected alternatives with failure modes, consequences, residual, proposed frontmatter), not a bare bullet dump or a pasted design excerpt | **AC-001** | Review against `.kb/decisions/README.md`, *What belongs here*, and the precedent pair `.kb/decisions/0029-…` / `references/adr/0029-…` |
| **Placement / composition root** — the staged document lands at `.kb/_intake/0020-fold-query-agreement.md` and nowhere else; a decision document at any other path is unmounted by construction (never ingested, never an atom) | **AC-001** | `test -f` plus the allowed-globs fence in *PR boundary* |
| **Transience** — the same three-way disposition `_design.md`'s `## Transience policy` gives ADR links (*"opened on demand — off-site… the long record is a click away, and only for readers who want it"*): the **atom** is the persistent summary a reader meets in the map; the **long record** is opened on demand; the **staged document** is transient by contract and is erased by the wave | **AC-007** | The pairing exists before the wave; the atom's `source_paths` is the click |
| **Density budget, with the corpus's real numbers** — atom ≈ **100 lines**; long-form records run **up to ~1,508 lines**; the two exist on purpose and collapsing them would discard about **78%** of the corpus (`CLAUDE.md`, *Where the work lives*). The staged document is sized to distil to an atom of that order, not to be one | **AC-007**, **AC-001** | `wc -l` at review against the corpus's own distribution; frontmatter `summary` sized like `0029`'s |
| **Hierarchy** — the decision is primary (stated first, in one shape), the rejected alternatives secondary, the residual and D-1 recessive but present; the record never opens with the residual, which would read as a hedge on a decision that was actually taken | **AC-002**, **AC-004** | Review |
| **The design's named anti-patterns, as content constraints** — the record must not license anything `_design.md` `## Anti-patterns` forbids, and must not restate the rendered-surface ones (items 1–15 are checks against a *rendered page*, which this story does not render). The standing set at the foot of that section is what binds here: no `#[async_trait]`, no `serde` in `happenstance-core`'s defaults, `read` returns the stream at the top level, generic code binds `EventStore` not `SendEventStore`, **no `unwrap`/`expect` in library code** | **AC-002**, **AC-004** | Review: the shape the record licenses is checked against that standing list, and the `unwrap` prohibition is exactly what makes AC-004's residual the answer it is rather than a convenience |

## Error conditions

| id | condition | required handling |
| --- | --- | --- |
| **EC-001** | The wave runs and `.kb/_intake/0020-fold-query-agreement.md` is **still there** afterwards | That is the contract's own signal: *"a file still sitting here after a run is a file that run did not ingest"* (`.kb/_intake/README.md`). Do not re-run blind and do not hand-author the atom; read the wave's manifest, fix the staged document, re-run. AC-008 stays unsatisfied |
| **EC-002** | The wave's default glob (`.kb/_intake/*.md`, run with no argument) sweeps in `.kb/_intake/README.md` | Drop it at the wave's approval gate. It is the directory's own documentation, not raw material, and ingesting it mints an atom about the staging area |
| **EC-003** | A second ingest wave reuses the first wave's id | Suffix the id so the second wave cannot overwrite the first's audit trail. The trail is what makes `source_paths` and the wave commit legible after `_intake` is cleared |
| **EC-004** | The atom the wave mints contradicts a signed-off `_design.md` row | Not fixable by editing the atom — an accepted decision's body is never reworded (`.kb/decisions/README.md`, *The immutability rule*). The correction is a **new superseding atom**, and the repair/amendment distinction there is mechanical: unchanged admitted-implementation set ⇒ repair, otherwise a new decision |
| **EC-005** | `redkiln validate --kb` fails on the new atom's frontmatter after the wave | The defect is in the *staged* document's proposed frontmatter (a dangling `source_paths` entry, an atom id in `depends_on` that does not exist, a missing `reversibility`). Fix the staged document and re-run the wave; never hand-edit the minted atom into conformance |
| **EC-006** | An implementer "fixes" the residual by adding an `unwrap`, by adding an infallible constructor to `happenstance-core`, or by editing a `[FROZEN]` clause | Refused on all three counts: no `unwrap` in library code (`_design.md` `## Anti-patterns`, standing set), no edit to the frozen crate (project AC-A02), and a defect routes to a decision record rather than a clause edit (project AC-012). D-1 is the recorded route |
| **EC-007** | ADR-0021's subject matter (payload evolution, versioned `EventType`, the codec tag's home) starts being decided inside this document | Scope leak. It is the slice-mate's, staged in the same wave; the public surface is invariant under that choice (`_design.md`, *Open, and deliberately not settled here*). Cut it and hand it across |
| **EC-008** | A citation in the record does not say what the record claims it says | Blocks sign-off. Re-open the range, repair the citation and record the repair — the precedent is `_design.md` `## Sign-off`, which repaired one citation before the human approved it |

## Non-functional

| id | requirement | why |
| --- | --- | --- |
| **NF-001** | **Nothing compiled changes.** No path under `crates/`, `examples/`, `xtask/` or `spec/` is touched, so `cargo xtask affected --base main` maps this diff to no package and `cargo xtask ci --fast` is unchanged by construction | The whole point of M1 is that no code exists yet for the record to describe (project AC-016, *"written first"*) |
| **NF-002** | The staged document is written to distil into an atom of roughly **100 lines** with a `summary` of the density `0029`'s carries; anything longer belongs in the long-form record | `CLAUDE.md`, *Where the work lives*; `.kb/decisions/0029-msrv-raised-to-1-97-1.md:12-25` |
| **NF-003** | Every `file:line` citation resolves and is re-read before sign-off | `.kb` and `spec/` citations are load-bearing across the corpus; `cargo xtask spec-trace` catches specification citations but nothing catches a wrong `crates/**` line range except a reader |
| **NF-004** | The record is **history, not current truth**: it states what was decided and when, it does not describe code that does not exist yet as though it did, and it will never be updated to match what M2 builds | `.kb/decisions/README.md`, *What does not belong here* — *"where the two disagree, the specification wins. An ADR is never updated to match the code."* |
| **NF-005** | The proposed frontmatter states a `reversibility` value and an honest one | It is the field a later reader uses to judge how expensive the supersession would be; `0029` states `medium` and explains why in its `summary` |
| **NF-006** | The record is legible to a reader **new to idiomatic Rust**: sealing, blanket impls, per-monomorphisation `const` evaluation and `NonZero`-style niches are each explained by what the alternative was and why it lost, not by naming the construct | `CLAUDE.md`, *Who you are working with*; `standards/rust/70-rustdoc-obligations.md` RS-70-5 is the same rule one level down |

## Implementation notes (non-prescriptive)

Nothing below is binding; the ACs are. This is the order that makes the ACs cheap.

- **Write the long record first, then distil.** `references/adr/0020-fold-query-agreement.md`
  is the place the compiler-facing detail, the eight rejected shapes and the trade table
  actually fit; the staged document is then a distillation of it plus the proposed
  frontmatter. Doing it the other way round tends to produce a staged document that is
  already atom-shaped and a long record that is a copy of it, which is how the 78% gets
  lost quietly.
- **Use the `0029` pair as the working template**, not as prose to imitate:
  `.kb/decisions/0029-msrv-raised-to-1-97-1.md` for the frontmatter fields and the
  `summary`'s density (it names the rejected options *inside* the summary), and
  `references/adr/0029-msrv-raised-to-1-97-1.md` for what a long record carries that an
  atom cannot.
- **The `## Shape decision` table is the source, and it is already in the right shape.**
  Chosen / rejected-and-why / evidence / resolves. Lifting its five relevant rows into
  prose that keeps all four columns is most of AC-002 and AC-005.
- **State the hazard with the code open.** `examples/course-subscriptions/src/main.rs:114-125`
  and the fold below it are the whole of AC-003's evidence; quote the two event-type lists
  rather than describing them.
- **`.kb/decisions/README.md` separates a decision from its evidence** — *"the measurement
  or compilation a decision rests on is a `reference` atom that this one cites"*. The
  precedent (`0029`) discharges that with `source_paths` pointing at the long record rather
  than a separate `reference` atom. Follow the precedent, and let the wave adjudicate
  whether this one also warrants a `reference` atom; that adjudication is
  `/redkiln:kb-ingest`'s, not this story's.
- **Then stop, and hand off.** The story is done at *staged and ready*. A human invokes
  `/redkiln:kb-ingest` over both M1 documents in one wave, on its own worktree branch
  (`_decomposition.md`, *The ADR route, and who invokes it*). AC-008 is verified against
  that wave, not inside this PR.

## Tests and CI (merge gate)

Grounded in `_decomposition.md`'s testing brief — whose AC-016 row is explicit that this
criterion's tier is **Static (`redkiln validate --kb`)** and that atoms are not authored
by the brief, the story, or any test.

| tier | command / path | proves |
| --- | --- | --- |
| Static, story grain | `cargo xtask affected --base main` (`.redkiln/config.yaml:40`) | The diff maps to **no** workspace package, so the command's package half compiles nothing — and its unconditional half still runs the five file-reading lints and `spec-trace`. That is exactly why the grain is wired this way: *"a story whose whole deliverable is an edit to `SPECIFICATION.md` maps to no package, and a purely package-shaped gate would compile nothing, read nothing, and call it green"* (`.redkiln/config.yaml:36-39`) |
| Static, reachability | `cargo xtask lints && cargo xtask spec-trace` (`.redkiln/config.yaml:48`) | No specification citation rotted; no clause marker moved. This story amends no clause, so the value here is the **negative** proof |
| Static, KB + backlog | `redkiln validate --kb && redkiln doctor` | Before the wave: the corpus is clean and this PR added no atom. After the wave (AC-008): `KbFrontmatter` conformance on the new atom and accepted-decision immutability against `HEAD`. `_intake` is skipped by design, so this command says nothing about the staged document itself — AC-001's instrument is `test -f` plus review |
| Static, existence | `test -f .kb/_intake/0020-fold-query-agreement.md`; `test -f references/adr/0020-fold-query-agreement.md` | AC-001 and AC-007's mount points exist at the paths the wave and the atom's `source_paths` will look for |
| Integration, project bar | `cargo xtask ci --fast` (`.redkiln/config.yaml:55`) | This project's declared integration bar (`project.md` DoD 6) — green **and unchanged**, because nothing compiled was touched. Its four `wasm32` steps and its packaging assertion are unaffected by construction (NF-001) |
| Human review, report gate | This spec's AC table vs `_design.md` `## Shape decision` / `## Signatures` / `## Sign-off` | AC-002, AC-003, AC-004, AC-005, AC-006 — the substance of the record. There is no compiled assertion that a decision was recorded correctly, and the testing brief says so for this whole class: *"Record, not a test… verified at closeout, not by a compiled test"* (AC-012 row) |
| Handoff, post-wave | `/redkiln:kb-ingest` (human-invoked) then `git log --diff-filter=A -- .kb/decisions/0020-fold-query-agreement.md` | AC-008: the atom exists, and it was added by the wave's commit rather than by hand — the reverted anti-pattern at `0269720` is exactly what this check discriminates |
| Deferred to M2 | Unit test of the chosen failure mode, beside the decision-model code (`_decomposition.md`, Testing brief, AC-014 row) | The record's *consequence* becomes testable only once `DecisionModel` exists. Named here so it is not mistaken for a gap in this story |

**Not applicable, and deliberately.** No conformance rule
(`crates/happenstance-testkit/src/suite.rs`): `DecisionModel`, `Boundary` and the derived
`Query` live entirely above the port, so no adapter can observe whether a query was derived
or hand-written, and a rule no adapter can fail is decorative (`CLAUDE.md`, *The rule that
matters*). No `trybuild` case: AC-002's compile-fail instrument is the project's, blocked on
`trybuild` landing, and is `compile-fail-proof-artefact`'s in M6.

## Risks and coupling (PR-scoped)

| Risk | Why it bites here | Mitigation in this PR |
| --- | --- | --- |
| **The story can be "done" while the atom does not exist.** Ingest is a human handoff on its own branch | AC-016 is a *sequencing* obligation on M2, and M2 starts from the atom. A PR that merges with the wave un-run leaves `domain-event-and-decision-model` reading a document that is about to be deleted from `_intake` | AC-008 is written as an explicit, ledger-carried acceptance step with the wave commit as its evidence, rather than left implicit in a DoD sentence |
| **Immutability makes wording expensive.** A phrasing that admits two shapes cannot be edited out later | `.kb/decisions/README.md`'s repair/amendment test is mechanical: if the admitted-implementation set changes, it is a new decision, not a fix | AC-002 and AC-005 require the record to match a design a human already signed off, and require the one-shape claim to be stated in the contract's own words (`projection.rs:47-61`) |
| **Slice coupling with `adr-0021-payload-evolution-and-codec-tag`** — same milestone, same wave, no `depends_on` edge | A wave containing one of the two mints one atom and satisfies half of project DoD 3; a wave containing a document that strays into the other's subject mints a conflicting one | EC-007 draws the line; the *Integration contract*'s slice-mate row names the shared wave as the integrated surface |
| **Design drift after sign-off.** `_design.md` was approved 2026-08-12; a record written from memory rather than from the file will diverge | The design is the input; the record is the durable artefact. Once accepted, the atom is what M2 reads, so a divergence propagates into code | AC-002's verification is a *side-by-side* against named sections, not a recollection |
| **The residual invites a core edit.** `Boundary::query`'s unreachable `Err` is the most "obviously fixable" thing in the design | Editing `happenstance-core` would be an unrecorded change to a frozen contract, and an `unwrap` would violate the standing anti-pattern set | AC-004 makes D-1's *routing* part of the criterion, and NF-001's negative diff assertion is mechanical |
| **Nothing in the compiled gate can fail on this PR** — a green gate proves almost nothing here | The temptation is to treat green as done | The merge-gate table above says what each green step actually proves (mostly a negative), and the substantive rows are explicitly human-reviewed |

## Dependencies

**Blocks on** — none. This story's `depends_on` is `[]` (`_storymap.md:49`), and that is
structural rather than incidental: M1 exists precisely so that nothing it needs has been
written yet.

**Unlocks**

- `domain-event-and-decision-model` (M2) — its `depends_on` names this story directly
  (`_storymap.md:51`). It is the story that builds the shape this record licenses.
- `decision-model-composition` (M2), transitively through the above — the tuple `Boundary`
  impls this record fixes at arity 2..=8.
- Project **DoD 3** (*ADR-0020 and ADR-0021 are accepted atoms… written before the code
  they govern, and `redkiln validate --kb` is clean*), discharged **jointly** with
  `adr-0021-payload-evolution-and-codec-tag`. Not a dependency edge in either direction —
  a shared wave.

## Anchors (progressive disclosure)

Everything above is enough to start. Open these at the moment named, and link them rather
than pasting them.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_design.md` | The **binding** signed-off design. `## Shape decision` is the source table (chosen / rejected-and-why / evidence / resolves); `## Signatures` has the exact `pub trait Boundary: sealed::Sealed`; *The residual* and *Defect candidate D-1* are the text AC-004 must not paraphrase away; `## Sign-off` names the three things the approver accepted | Before writing a word of the long record — it is the input the record distils, and AC-002's verification is a side-by-side against it | AC-002, AC-004, AC-006 |
| `.kb/decisions/0029-msrv-raised-to-1-97-1.md` | The precedent atom: every frontmatter field the wave will author (`adr_id`, `authority_tier`, `reversibility`, `phase`, `depends_on`, `source_paths`), and a `summary` that names its rejected options inline — the density NF-002 asks for | When drafting the staged document's **proposed frontmatter** block | AC-001, AC-007 |
| `references/adr/0029-msrv-raised-to-1-97-1.md` | The matching long record. Shows what the long form carries that a ~100-line atom cannot, and therefore where the line between the two files falls | When starting `references/adr/0020-fold-query-agreement.md`, before deciding what to leave out of the staged document | AC-007 |
| `.kb/decisions/README.md` | The corpus's own rules: what belongs (*"state the alternatives that lost"*), what does not (evidence; a decision not yet taken; current truth), the immutability rule, and the mechanical repair-vs-amendment test | Before writing, and again before sign-off as a checklist against the finished record | AC-005, AC-008 |
| `.kb/_intake/README.md` | The ingest contract: `_intake` is the default input and the *only* mount point; a successful run clears it; `_`-prefixed directories are skipped by `redkiln validate --kb`, which is why the staged document is not held to `KbFrontmatter` | When siting the staged document, and when interpreting EC-001/EC-002 after a wave | AC-001, AC-008 |
| `examples/course-subscriptions/src/main.rs` | The hazard in real code: the two-item `Query` at `:114-125` and the fold below it naming the same event set a second time, with no signal against divergence. Lines `:114-173` are the span AC-003 cites | While writing the record's hazard section — quote it, do not describe it | AC-003 |
| `crates/happenstance-core/src/query.rs` | `QueryItem::new` at `:56` is fallible and the crate is frozen; this is *the* fact that makes an infallible `query()` unreachable without an `unwrap`, and therefore the fact AC-004's whole answer rests on | When writing *where the fallibility went* — before deciding the residual can be "fixed" | AC-004 |
| `crates/happenstance-core/src/tag.rs` | `Tags::from_pairs` at `:304` is the only way in and it is fallible — which is why `scope(&self) -> &Tags` pays the validation once, in the constructor the caller already writes | Same moment as `query.rs`; the two together are the `scope` row's evidence | AC-004, AC-002 |
| `crates/happenstance-core/src/error.rs` | `InvalidQuery` at `:95-105`, with `UnconstrainedItem` at `:99-102` — the variant the record re-reads as *"this boundary constrains nothing, which is `Query::all()`"* | When writing the *`Err` is kept and given a real meaning* paragraph | AC-004 |
| `crates/happenstance-core/src/projection.rs` | `:47-61` states the two-constructor defect in the contract crate's own words. AC-005's one-shape claim is strongest quoted from here, because it is the repository already condemning the failure mode | When writing the one-shape-not-two paragraph | AC-005 |
| `standards/rust/40-public-surface-and-evolution.md` | RS-40-2 at `:73` — *put a derivable convenience on the blanket ext trait*. The house rule the `Boundary` shape is built on, and the reason a provided method on `DecisionModel` lost | When justifying *why a separate sealed trait* to a reader new to idiomatic Rust (NF-006) | AC-002, AC-005 |
| `standards/rust/61-compile-time-assertions.md` | RS-61-4 at `:249` — *put the claim in a `const` item*. The mechanism that turns an empty `EVENT_TYPES` into a compile error rather than a first-read failure | When writing the empty-`EVENT_TYPES` paragraph, and when explaining why fallible-without-the-assertion lost | AC-002, AC-005 |
| `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_decomposition.md` | *The one signature question DT-2 actually is* (`:526`) frames the question the record answers; *The ADR route, and who invokes it* (`:643`) fixes the handoff; the Testing brief's AC-014 and AC-016 rows fix the tiers this spec's gate table inherits | Read `:526` before drafting; read `:643` before assuming ingest is a step you run | AC-002, AC-008 |
| `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/project.md` | The project ACs this story traces to, verbatim (AC-001 `:159`, AC-012 `:204`, AC-014 `:210`, AC-016 `:218`), and DoD 3 `:229` — the joint obligation AC-008 discharges | When checking traceability before the report gate | AC-002, AC-008 |
| `.bklg/from-contract-to-published-library/initiative.md` | DoD 2 (*@smoke — the compiler protects the domain*, `:363-365`) is the initiative-level outcome this decision makes structural; the personas-and-journeys references are the audience the record's reasoning is written for | When writing the *consequences* section, to state the outcome the shape buys rather than the shape itself | AC-003, AC-006 |
| `RUNBOOK.md` | `:299` is ADR-0020's slot and its question in the plan's own words — *"How does a decision model guarantee that its query and its fold cannot disagree?"*; `:524` is AC-013's falsifiable prediction the record carries as a consequence | `:299` when titling the record; `:524` when writing the 2.4:1 consequence | AC-006 |
| `.redkiln/config.yaml` | The `verify:` block wires the four grains regardless of who types them — `affected_gate` `:38`, `reachability_static` `:47`, `integration_scoped` `:55`. Explains why a no-package diff is still gated | When interpreting a green gate, so it is not mistaken for proof of the record's substance | AC-001 |

## Clarifications resolved during spec

1. **The AC set is exactly the front half's eight** — AC-001 … AC-008. None added, none
   dropped. AC-001 and AC-007 carry the two artefacts; AC-002, AC-003, AC-004, AC-005 and
   AC-006 carry the record's substance; AC-008 carries the post-wave state. The ledger
   matches this set row for row.
2. **AC-008 straddles the PR boundary on purpose.** The atom is minted by
   `/redkiln:kb-ingest` on its own branch, so it cannot be an assertion inside this PR's
   diff — and it cannot be dropped either, because project DoD 3 is written against the
   atom's existence, not the staged document's. It is therefore a handoff-verified row
   whose evidence is the wave's commit sha. An implementer who cannot produce that sha
   leaves it `satisfied: false`; that is the correct outcome, not a blocker to work around
   by hand-authoring the atom.
3. **This story renders no design surface, and that is a finding rather than an
   omission.** All six ids in `_design.md`'s `## Surfaces` manifest route at `crates/` or
   `examples/` paths this PR may not touch. The composition invariants that do bind were
   therefore re-derived from the corpus's own rules (`.kb/decisions/README.md`,
   `.kb/_intake/README.md`, `CLAUDE.md`'s two-places-on-purpose paragraph) and from
   `_design.md`'s content-governing sections, and every one of them is carried as an AC row
   above rather than as prose.
4. **`_design.md`'s anti-pattern list splits.** Items 1–15 are checks against a rendered
   page or a terminal screenshot and belong to M2–M6. The *standing* set at the foot of
   that section — no `#[async_trait]`, no `serde` in `happenstance-core`'s defaults,
   top-level stream from `read`, bind `EventStore` not `SendEventStore`, no
   `unwrap`/`expect` in library code — is what constrains this record's content, and the
   last of those is precisely why AC-004's residual is answered the way it is.
5. **Whether the long-form record should also become a `reference` atom is left to the
   wave.** `.kb/decisions/README.md` says evidence belongs in a `reference` atom the
   decision cites; the corpus's actual precedent (`0029`) discharges that with
   `source_paths` alone. This spec follows the precedent and flags the question for
   `/redkiln:kb-ingest`'s adjudication rather than pre-empting it — consistent with atoms
   being the ingest path's to author.
6. **No conformance rule and no `trybuild` case is in scope**, and both absences are
   reasoned rather than deferred: nothing below the port can observe a derived query, and
   AC-002's compile-fail instrument is blocked on `trybuild` and owned by M6's
   `compile-fail-proof-artefact`.
