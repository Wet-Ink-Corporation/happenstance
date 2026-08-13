---
item: HS-S0003
stage: spec
created: 2026-08-12T13:45:57.874Z
updated: 2026-08-12T13:45:57.874Z
template_sig: 87bbf1d0
rendered_sig: 5072c158
---

# Spec — DT-3 and DT-8 resolved in the public-API design record

## Scope lock

| Artefact | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` — DT-3 / DT-8 at the *Open design tensions* table; AC-04, AC-05 and BR-13 are the promises this story's answers are judged against; DoD 7 is the scenario it moves toward |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` — warrants `ux` for this project *because* DT-3 is "literally what does the suite's output say to a human" |
| Project charter | `.bklg/from-contract-to-published-library/projection-store-freeze/project.md` — AC-006, AC-007, DR-07, DR-08, DoD 6 and DoD 7 |
| **This spec** | `.bklg/from-contract-to-published-library/projection-store-freeze/projection-api-design-record/spec.md` |
| **Signed-off design (BINDING)** | `.bklg/from-contract-to-published-library/projection-store-freeze/_design.md` — the mount point. Its no-screen determination, its `surfaces: []` manifest and its existing sign-off block are binding and are not re-decided here |
| Key briefs | `.bklg/from-contract-to-published-library/projection-store-freeze/_decomposition.md` — **UX brief** AC-U01 – AC-U12 (the bar both answers are judged against, and the whole of the type/text surface criteria), **Architecture brief** AC-A02 / AC-A03 / AC-A04 / AC-A06 / AC-A09 and Notes 1, 4, 5, 7, 9, **Testing brief** rows AC-006 / AC-007 (which name this the one thing no test tier proves) |
| Grounding | `.bklg/from-contract-to-published-library/projection-store-freeze/_grounding.md` |
| Story map | `.bklg/from-contract-to-published-library/projection-store-freeze/_storymap.md` — slice `decisions-and-design-record`, merge position 1 of 8 |
| Roadmap pointer | `RUNBOOK.md:3848-3965` (phase 6 in full); `RUNBOOK.md:588-610` (the provisional PS groups and PS-2's thirteen rows) |
| Specification (wins on conflict) | `spec/SPECIFICATION.md` §4.0 at `:4632-4731` (the target trait, given verbatim), `:4977-5031` (`ProjectionProbe` and the coherence argument) |

## One-line PR slice

Resolve DT-3 (one authoritative source for "this guarantee does not apply", citing
`.kb/open-questions/cf-40-fixture-limits-ownership.md` rather than minting a second declension policy)
and DT-8 (whose adapter-author bar the suite holds) in `_design.md`, alongside the public projection
API surface review the repurposed `.redkiln/templates/_design.md` asks for — signatures, visibility,
what the shape costs a caller, and a doctest in place of a mock.

## Executive summary

This PR lands **prose, in one file**: the projection port's public-API design record, written into the
project's already-signed-off `_design.md`.

The delta is exact. `_design.md` today answers the *screen* question and only that: it establishes
that this project ships nothing a person looks at (`surfaces: []`), names the two non-visual surfaces
a human meets instead — **a type surface** (`ProjectionStore`, its batch vocabulary, the new
`Checkpoint` / `Authority` / `CommitError` / `ResetError` types) and **a text surface** (the one line a
run prints for a declined capability) — and then marks eleven sections `N/A — no user-facing surface`
(`_design.md:52-94`). Those eleven sections are exactly the review `CLAUDE.md` says the repurposed
template exists to perform, and exactly what project DoD 7 requires before this port is frozen. This
story fills them, **for the two surfaces the same file already enumerates**, and settles the two
tensions parked there: DT-3 and DT-8.

Nothing approved is reversed. The no-screen determination, the `surfaces: []` manifest and the
existing sign-off block are preserved verbatim; the record is additive and goes back in front of the
human at the design gate as an amendment, unsigned.

Why it is a *foundation* story with three dependents rather than documentation: three later stories
read decisions out of this file rather than making them. `projection-capability-skips` (HS-S0008)
takes "the projection capability set fixed by DT-3's recorded resolution rather than invented here";
`documented-extension-surface` (HS-S0015) is scoped entirely by which DT-8 arm is recorded here;
`owned-batch-port-shape` (HS-S0004) implements the signature block this record transcribes. Those are
the `blocks:` edges in this story's own frontmatter, and they are the reason an unwritten section here
becomes a decision made silently by whoever needs it next.

## Context pack

**Read this section and you can start.** Everything below it is either an obligation on the record or
a boundary; the deeper artefacts are behind the anchors.

### The two questions this story answers, and the exact form the answers must take

**DT-3 — how a consumer learns a guarantee does not apply to their implementation.** The options are
(a) the suite's own output, (b) per-adapter documentation, (c) both with one authoritative
(`initiative.md`, *Open design tensions*). The failure mode is symmetric and both halves are already
documented: a silent skip is one, two sources of truth is the other. The bar (UX brief AC-U01) is not
"name a winner" — it is that the resolution **disposes of every reason-writer that already exists in
this tree**, of which there are three:

1. **The fixture-written reason.** `Capability::declined(reason)` — an opaque struct whose private
   field forces the reason through a `const fn` `assert!`, because `Declined("")` would otherwise be
   a perfectly good value (`crates/happenstance-testkit/src/contract.rs:355-419`). The reason is the
   adapter's account of a trade only the adapter can describe.
2. **The testkit-written reason.** `NO_STORE_LIMITS` / `NO_CEILING_REASON`
   (`contract.rs:435-456`), whose own doc says it is *"the one place the skip machinery here differs"*
   — because "this store has no ceiling" is the same sentence for every store that says it, and asking
   each fixture to phrase it buys a paraphrase per adapter and no information.
3. **Per-adapter prose**, in the adapter's own documentation.

A resolution that names one authoritative source and leaves the other two undisposed has minted the
second divergent declension policy `project.md`'s risk list forbids — *"one policy, or a stated reason
for two"*. And `.kb/open-questions/cf-40-fixture-limits-ownership.md` is **cited as authoritative, not
re-litigated**: that atom records a contradiction imported from two accepted, immutable decisions
(ADR-0015 claims and disclaims CF-40 in the same document, ADR-0012 is the adjacent claimant), and it
is explicit that no later wave has standing to prefer one reading. This story does not resolve CF-40.
It states what the projection fixture does *given* that CF-40's ownership is unresolved, and says so.

**DT-8 — whose adapter-author bar the suite holds.** Arms are (a) internal only for now, (b) hold the
bar for an external author. The bar (UX brief AC-U02) is that the answer **states its cost to an
author this repository did not write**:

- If the outside-author arm is taken, the extension surface is the documented pair
  `projection_store_conformance!` + `ProjectionProbe`, and its cost must remain *one feature flag on a
  dependency the adapter already has, and no new edge in the dependency graph* — which is precisely
  why `ProjectionProbe` lives in `happenstance-core` behind `conformance` rather than in the testkit
  (Architecture brief AC-A02; the coherence argument is spelled out at
  `spec/SPECIFICATION.md:5015-5031`: an adapter's `tests/` directory is a different crate, where
  neither a testkit trait nor the adapter's type is local, so the impl is rejected by the orphan rule).
  Taking this arm creates a live obligation on `documented-extension-surface` (HS-S0015): a fixture
  built from the documentation alone — not copied from `crates/happenstance-testkit/src/fixtures.rs` —
  clearing the mutant-registry exactness check and the capability-skip rule.
- If the narrow arm is taken, the recorded scope must appear **where an outsider meets it** — the
  testkit's own crate doc — and not only in `_design.md`, because a bar that is internal-only in fact
  and unqualified in public is the failure DT-8 names.

Either way the arm must be legible enough that HS-S0015 can be scoped from this file alone.

### What is binding, and what is genuinely open

| Already decided elsewhere — do not re-decide | Where |
| --- | --- |
| The second batch shape is a testkit-internal CF-5 buffering variant, not `happenstance-sqlite` | Architecture brief AC-A03 |
| `ProjectionProbe` lives in `happenstance-core` behind a new `conformance` feature | Architecture brief AC-A02; `spec/SPECIFICATION.md:4998-5031` |
| PS-2's bar is not clearable in-project, so AC-014 takes its second arm: `unstable-projection` with a stated reason | Architecture brief AC-A04 |
| `Capability` and `RuleOutcome` are reused **unchanged**; no projection-local skip type, no second line shape | Architecture brief AC-A06; UX brief AC-U08 |
| `ProjectionId::new` stays infallible; the identifier is not hardened as a side effect of freezing the port around it | Architecture brief AC-A09; `.kb/open-questions/projection-id-is-unvalidated.md` |
| The target trait's shape — `type Batch;`, non-async infallible `begin`, three-variant `Checkpoint`, two error enums, `reset` | `spec/SPECIFICATION.md:4632-4731`; the specification wins on conflict |
| No borrowing GAT anywhere in the fixture (five-ingredient rustc ICE, still reproducing on 1.97.1) | `crates/happenstance-testkit/src/contract.rs:97-111`; `experiments/rustc-ice-gat-foreign-trait/` |

| Genuinely this story's to decide | Consumed by |
| --- | --- |
| **Which projection fixture capability constants exist**, and whether any is a MUST in `SECOND_HANDLE`'s sense | `projection-capability-skips` (HS-S0008); Architecture brief Note 5 hands it here by name |
| **Who writes the reason** for each of those constants — fixture, testkit, or adapter prose (DT-3 applied) | HS-S0008 |
| **DT-8's arm**, and the obligation it creates | `documented-extension-surface` (HS-S0015) |
| **Whether `ResetError::Refused` carries the store's stated reason or stays bare** — UX brief AC-U07 says silence is the failure, not a passing default | `owned-batch-port-shape` (HS-S0004); `reset-rules` (HS-S0012) |
| The signature block, visibility, feature gates and doctest the implementer builds against | HS-S0004, `memory-projection-store`, HS-S0008 |

### The one thing that makes this record hard to write honestly

`RuleOutcome::report` is a **no-op on `wasm32-unknown-unknown`** — measured under
`wasm-bindgen-test-runner` with `--nocapture`, not assumed, and documented as such at
`crates/happenstance-testkit/src/contract.rs:511-531`. That target's `std` has no host stdio; the wasm
emitter calls `skip_line` and hands the string to `console_log!` instead. Natively, libtest suppresses
a *passing* test's stdout unless `--show-output` is passed, which is why the gate passes it — and the
same doc is blunt that this *"makes the line reachable by a human; it does not make anyone read it"*.

So if DT-3's answer is "the suite's own output", the answer is incomplete until it says what the
projection wasm harness owes (UX brief AC-U11) and until it accepts that the machine-checked half is a
`RuleOutcome`-**value** assertion, never stdout (UX brief AC-U09, project AC-005). This is the single
place where a plausible-sounding DT-3 resolution is actually wrong on the target BR-12 exists to
protect, and P3 — the local-first / edge developer — is the persona who pays for it.

### The persona slice

Two readers, from the UX brief's own table, and they meet different halves of this record:

- **P2, the adapter author** (`_discovery/distillation/personas-and-journeys.md:114-153`) wants *"an
  executable definition of 'correct' they can run against their own storage system, rather than a
  prose specification they have to interpret"*, and fears that the port quietly assumed something their
  storage cannot provide. DT-8 is literally the question of whether they are in the room. The type
  surface sections are what they read before writing an impl; the `E0195` trap
  (`references/adapter-shapes.md:186-194`) is what they hit ten minutes later with, today, nothing to
  copy from.
- **P3, the edge developer** owns AC-016 and is the one the text surface fails silently on.

Both citations are **secondary evidence** — no persona in this initiative has been directly observed
(`…/personas-and-journeys.md:361-363`), and promotion into `.kb/product/` is HS-P0019's. Carry the
qualification; do not upgrade it.

### Authority order, applied

Accepted decision atoms under `.kb/decisions/` beat this record; the specification beats the briefs
(`_intake-brief.md` Gate: Intake, last box); the briefs beat the implementer's preference. Concretely:
ADR-0017, ADR-0018 and ADR-0019 land in `projection-decision-atoms` (HS-S0002), which is this story's
`blocked_by`. **They are accepted and therefore immutable before this record is written.** Where the
record and an atom disagree, the atom wins and the record says which sentence yielded — it does not
edit the atom, and it does not quietly restate the atom's rejected alternative as a design choice.

### What this story must not do

`CLAUDE.md` is explicit that `.kb/` atoms are authored by `/redkiln:kb-ingest` from `.kb/_intake/`,
not by hand — *"hand-writing them produces the directory layout of the process without the process"*,
which is why the first attempt was reverted (`0269720`). This story therefore mints **no** decision
atom, resolves no open-question atom's status, and edits nothing under `.kb/`. If the record's work
turns up an answer that deserves an ADR, it is recorded here as a named gap for the runbook's ADR pass
to pick up, not written as a side effect.

Likewise it edits no `[FROZEN]` clause (a change there is a new ADR and a re-plan), makes no freeze
verdict (that is `ladybug-projection-store`, HS-P0015) and makes no call on the `unstable-projection`
exposure (that is `publication-and-positioning`, HS-P0016).

## Integration contract

- **Archetype**: `foundation` — real in-tree substrate consumed by capability slices in this
  initiative. The substrate here is a decision record three later stories read from, not a component.
- **Slice / milestone**: `decisions-and-design-record`. Slice-mates, implemented in one context and
  mounted as one surface: `ps-clause-pairing-sweep`, `projection-decision-atoms`. This story is last
  within the slice — the sweep scopes the ADRs, the ADRs are accepted, and only then is the record
  written against them.
- **Mount point**: `.bklg/from-contract-to-published-library/projection-store-freeze/_design.md`.
  A library has no render tree; for a *decision* the composition root is the artifact the implement
  stage and the design-stage review actually read. `_design.md` is that file by construction: the
  repurposed `.redkiln/templates/_design.md` is what `CLAUDE.md` says asks for the public API surface,
  every downstream story's **Surface quality** section is written against `_design.md`'s `## Signatures`,
  `## Visibility and stability` and `## Anti-patterns` blocks (see the rendered template at
  `.redkiln/templates/spec.md`), and project DoD 6 and DoD 7 name it. A record written into this
  story's own folder instead would be constructed-but-unmounted: nothing downstream reads there.
- **Wires into** (real siblings this record binds to, by path):
  - `crates/happenstance-testkit/src/contract.rs:355-456` — `Capability`, `NO_STORE_LIMITS`,
    `NO_CEILING_REASON`. Reused unchanged; DT-3's answer is written in their vocabulary.
  - `crates/happenstance-testkit/src/contract.rs:458-537` — `RuleOutcome`, `skip_line`, `report` and
    its two honest per-target limitations.
  - `crates/happenstance-core/src/projection.rs:1-139` — the port as it stands, its provisional
    header and its stated invariant; the record's signature block is the diff against it.
  - `spec/SPECIFICATION.md:4632-4731` and `:4977-5031` — the target trait and `ProjectionProbe`,
    transcribed rather than re-designed.
  - `.kb/open-questions/cf-40-fixture-limits-ownership.md` — cited as authoritative for the
    declension-policy question, per project AC-006.
  - `.kb/decisions/0010-the-suite-must-prove-itself.md` — the proof obligation any capability the
    record invents must still satisfy.
  - `standards/rust/40-public-surface-and-evolution.md` (RS-40-5),
    `standards/rust/70-rustdoc-obligations.md` (RS-70-5),
    `standards/rust/91-adapter-authoring-recipe.md` (RS-91-3) — the three atoms the UX brief names,
    and only these.
- **Renders surfaces**: **none** in the `_design.md` `## Items` sense — that block reads
  `N/A — no user-facing surface` and the manifest is `surfaces: []` (`_design.md:48-54`), which stays
  true. What this story *authors* is the record of the two non-visual surfaces the same file already
  enumerates at `_design.md:24-38`: the **type surface** (`ProjectionStore` + the four new types +
  `ProjectionProbe`) and the **text surface** (``SKIP {rule}: fixture declines `{capability}` — {reason}``).
  After this PR the `## Items` block is populated for the type surface, so downstream stories have
  item ids to claim; that is the point of the story.
- **Public items** (the `## Items` ids this record creates for later stories to claim, not to
  implement here): `happenstance_core::Checkpoint`, `happenstance_core::Authority`,
  `happenstance_core::CommitError`, `happenstance_core::ResetError`,
  `happenstance_core::ProjectionStore` (amended), `happenstance_core::ProjectionProbe` (behind
  `conformance`), `happenstance_core::MemoryProjectionStore` (behind `memory`),
  `happenstance_testkit::projection_store_conformance!` and the projection fixture trait.
- **Conformance rule(s)**: **none, and that is not a gap.** This story adds no rule and changes no
  port, so it is not adapter-observable; the Testing brief already records AC-006 as
  *"None (design-stage artefact)"* and AC-007 as conditional on the arm chosen, and says so precisely
  *"so no story tries to invent a test for it"*. What this story does is fix the inputs to rules other
  stories write — the capability set (`projection-capability-skips`) and the signatures
  (`owned-batch-port-shape`). Its instrument is the design-stage review (project DoD 7), plus
  `redkiln verify --grain story` on the PR boundary below.
- **Clause(s)**: discharges none and amends none. It **transcribes** PS-5, PS-6, PS-7, PS-11, PS-12,
  PS-15, PS-16 – PS-22, PS-24, PS-34, PS-36 as they already stand and cites `[FROZEN]` / `[PROVISIONAL]`
  markers as-is. Nothing `[FROZEN]` is line-edited here; the PS-1 / PS-19 pairing repair belongs to
  `ps-clause-pairing-sweep` and `unstable-projection-gate-and-clause-disposition`.
- **Advances DoD scenario**: initiative **DoD 7** — *"The projection suite discriminates."* Moved
  toward green, not reached: the capability set and the probe/extension surface every discriminating
  rule is written against are fixed here. It fully discharges project **DoD 6** (DT-3 and DT-8 each
  carry a recorded resolution in `_design.md`) and puts project **DoD 7** — the design-stage review of
  the public projection API surface — in front of the human for the first time.

## PR boundary

The narrowest set that is honestly true. `redkiln verify --grain story` reads the first fenced block
below and fails on any file changed outside it.

```
.bklg/from-contract-to-published-library/projection-store-freeze/_design.md
.bklg/from-contract-to-published-library/projection-store-freeze/projection-api-design-record/**
```

**In this PR**

- The DT-3 resolution and the DT-8 resolution, written into `_design.md`.
- The projection fixture's capability set, with reason-authorship stated per constant.
- The public API surface record for the type surface: `## Items`, `## Signatures`, `## Shape decision`,
  `## Placement and re-export`, `## Visibility and stability`, `## What it costs a caller`,
  `## The states the API must express`, `## Anti-patterns`, `## The doctest` — filled for the two
  non-visual surfaces `_design.md:24-38` already names.
- An amendment sign-off block, left **unsigned**, naming what is being added and what stays untouched.
- This story's own `_ledger.md` and stage artifacts.

**Explicitly not in this PR**

- Any file under `crates/**`. No trait is edited, no feature added, no doctest compiled. The record
  *specifies* the doctest; `memory-projection-store` is where it becomes a gate-run example.
- Any file under `.kb/**` — no atom hand-written, no open question's status changed, no ADR minted
  (`CLAUDE.md`, *Where the work lives*).
- `spec/SPECIFICATION.md` — no clause edited, no maturity marker moved
  (`unstable-projection-gate-and-clause-disposition` owns that).
- The testkit crate doc sentence DT-8's narrow arm would require: named here as an obligation,
  discharged by `documented-extension-surface`.
- Any re-decision of the `surfaces: []` no-screen determination or the existing sign-off block.
- The freeze verdict (HS-P0015) and the `unstable-projection` exposure verdict (HS-P0016).

**Merge DoD (one line):** `_design.md` carries a DT-3 resolution naming one authoritative source and
disposing of all three reason-writers, a DT-8 resolution naming its arm and its cost to an outside
author, and a complete API surface record for the type and text surfaces — with the prior sign-off
preserved verbatim, `redkiln validate` green on this story's item, and `redkiln verify --grain story`
green on the boundary above.

## Behavior and interfaces

The observable behaviour of a prose deliverable is what a later reader can do with it without asking a
question this file should have answered. Each row is a contract on the record.

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **DT-3 names exactly one authoritative source** | One of: the suite's own output, per-adapter documentation, or both-with-one-authoritative. Not a preference — a statement of which one a consumer is entitled to rely on when the two disagree. | `.bklg/from-contract-to-published-library/initiative.md` (*Open design tensions*, DT-3); project `AC-006` |
| **…and disposes of all three reason-writers** | For each of `Capability::declined`, the testkit-written `NO_CEILING_REASON`, and per-adapter prose: which defers to which, and why. One policy, or a stated reason for two. | `crates/happenstance-testkit/src/contract.rs:355-419`, `:435-456`; `project.md` *Risks* ("one policy, or a stated reason for two"); UX brief AC-U01 |
| **…citing CF-40's atom as authoritative, not re-litigating it** | The record states what the projection fixture does *given* that CF-40's ownership is contradicted by two accepted, immutable decisions, and does not prefer a reading. | `.kb/open-questions/cf-40-fixture-limits-ownership.md` |
| **The projection capability set is enumerated, not implied** | Which constants the projection fixture declares, whether any is a MUST in `SECOND_HANDLE`'s sense, and who writes each reason. At minimum a reset-refusal capability (`refused_reset_changes_nothing`) and the statement that PS-12's gate is a `ProjectionProbe` const (`READS_THROUGH_BATCH`), not a fixture const. | Architecture brief Note 5 (hands this to `_design.md` by name); `spec/SPECIFICATION.md:5200-5217`, `:5052-5074`; `crates/happenstance-testkit/src/contract.rs:120-125` |
| **A "nothing declinable" outcome is a finding, not a licence** | If the projection fixture ends with no declinable capability at all, the reporting discipline is not quietly dropped: the outcome is recorded in DT-3's resolution, because a suite with nothing to decline cannot demonstrate initiative AC-05 and this project owns it. | UX brief, *What would make this brief wrong* |
| **DT-3's answer survives the constrained target** | If the suite's output is authoritative, the record states that `RuleOutcome::report` is a measured no-op on `wasm32`, that the projection wasm harness must route `skip_line` to `console_log!` as `__emit_wasm` already does, and that the machine-checked half is a `RuleOutcome`-value assertion rather than stdout. | `crates/happenstance-testkit/src/contract.rs:511-531`; UX brief AC-U09, AC-U11; project `AC-005` |
| **DT-8 names its arm and its cost** | Outside-author arm: the extension surface is the documented pair `projection_store_conformance!` + `ProjectionProbe`, cost bounded at one feature flag on a dependency the adapter already has and **no new edge in the dependency graph**. Narrow arm: the scope is stated where an outsider meets it — the testkit crate doc — and the record names that file as HS-S0015's obligation. | `spec/SPECIFICATION.md:5015-5031`; UX brief AC-U02; `_storymap.md` row `documented-extension-surface` |
| **`## Items` + `## Signatures` transcribe the target trait item for item** | `type Batch;` (no lifetime), `fn begin(&self) -> Self::Batch` (neither `async` nor fallible), `checkpoint -> Checkpoint`, `commit(batch, id, position, Authority) -> Result<(), CommitError<E>>`, `reset(batch, id) -> Result<(), ResetError<E>>`, `rollback`, plus `ProjectionProbe`'s five items. Each signature carries its PS clause id. Where the record and an accepted ADR-0017/0018/0019 atom disagree, **the atom wins** and the record says which sentence yielded. | `spec/SPECIFICATION.md:4632-4731`, `:4998-5013`; `crates/happenstance-core/src/projection.rs:87-139`; `.kb/decisions/` (immutability of accepted atoms) |
| **`## What it costs a caller` names the alternative that lost, at the call site** | `Checkpoint` is the three-variant enum, not `(Option<SequencePosition>, bool)` — the tuple can spell `(None, true)`, *"authoritative, never run — which means nothing"*. `CommitError` and `ResetError` stay two enums so `commit`'s caller never matches `Refused`; each carries the adapter error as a type parameter, never a `String`. `begin` is infallible because `async` + fallible implies a round trip Neon's one-shot transport cannot afford. `reset` takes the caller's own deletes because the port has no idea what the read model is. Each appears **once**, in the item's rustdoc, not only in an ADR. | `spec/SPECIFICATION.md:4643-4658`, `:4722-4727`, `:4692-4694`, `:4884-4897`, `:5165-5170`; `standards/rust/30-error-taxonomy.md:15,71`; RS-70-5 at `standards/rust/70-rustdoc-obligations.md:243` |
| **`ResetError::Refused`'s payload is decided out loud** | Either the variant carries the store's stated reason, or it stays bare and the record says where a caller is expected to find the reason instead. Silence is the failure, not a passing default. | `spec/SPECIFICATION.md:4676-4682`, `:5210-5216`; UX brief AC-U07 |
| **`## Visibility and stability` is specific enough to be violated** | `#[non_exhaustive]` on all four new types; re-export placement in `crates/happenstance-core/src/lib.rs:98-124`; `ProjectionProbe` behind `conformance` with `#[cfg_attr(docsrs, doc(cfg(…)))]` copying the `memory` pattern; `MemoryProjectionStore` behind `memory`; `unstable-projection` recorded as AC-A04's taken arm; and the rustdoc hazard — an intra-doc link into a feature-gated module is a hard error this workspace has already paid for once. | Architecture brief Note 1 (seam map) and Note 9; `crates/happenstance-testkit/src/lib.rs:112-132` |
| **The doctest is written **and** assigned a compiling home** | `## The doctest` carries the example text, and names the source item it will live on and the gate step that compiles it (`cargo test --doc`, inside `cargo xtask ci`). A described example is not a checked one; this record cannot compile it, so it must say who does. | `CLAUDE.md` (*Where the work lives*, "a doctest in place of a mock"); Testing brief row AC-012; `_storymap.md` row `memory-projection-store` |
| **`## Anti-patterns` lists forbidden moves with their evidence** | No `#[async_trait]` (ADR-0001); no `type Batch: Send;` (PS-36 — `trait_variant` copies the bound into the `!Send` flavour and breaks wasm32); no borrowing GAT on the fixture (the rustc ICE); no merging the two error enums; no `async fn begin`; no second skip vocabulary or projection-local skip type; no literal position assertions; `compile_fail` spelled bare, never `compile_fail,E0080`; no hardening of `ProjectionId::new`. | `.kb/decisions/0001-async-port-flavours.md`; `spec/SPECIFICATION.md:5601-5627`; `crates/happenstance-testkit/src/contract.rs:97-111`, `:400-403`; Architecture brief AC-A06, AC-A09; `CLAUDE.md` *The rule that matters* |
| **The amendment is additive and re-gated, never a rewrite** | The no-screen determination, the `surfaces: []` manifest and the existing sign-off block (`_design.md:10-101`) survive verbatim; new content fills the sections that read `N/A — no user-facing surface`, for the two non-visual surfaces the file itself names. A new sign-off block is appended **unsigned** — this story never signs on the reviewer's behalf. | `_design.md:10-38`, `:48-50`, `:92-101` |
| **Escalation, not absorption** | If the design-stage reviewer holds that the `N/A` determination was meant to cover the type surface as well, then project DoD 7 cannot be met by any story in this project and that is reported at the story boundary as a re-plan — not resolved by widening a section quietly. | `project.md` DoD 7; `_decomposition.md` Architecture brief Note 10 (report rather than absorb) |
| **Scope discipline is observable in the diff** | No file under `crates/**`, `.kb/**` or `spec/**` changes. Any answer the record turns up that deserves an ADR is written as a named gap for the runbook's ADR pass, not minted here. | `CLAUDE.md` (*Where the work lives*); the PR boundary block above |

## Data and migrations

**N/A.** This story ships no code, no schema, no persisted state and no runtime data path: its entire
deliverable is markdown in one artifact under `.bklg/`, plus this story's own stage files. There is
nothing to migrate, no fixture data to seed and no serialisation format in play — `happenstance-core`
carries no `serde` in its default features by binding constraint (ADR-0003), and nothing here changes
that.

The one thing with a migration *shape* is the artifact edit itself, and it is handled in the PR
boundary rather than here: `_design.md` is amended additively, its prior sign-off preserved verbatim,
with a new unsigned sign-off block appended for the added material.

## Acceptance criteria

Framed from user intent. The personas are the UX brief's own three readers — **P2, the adapter
author** (`_discovery/distillation/personas-and-journeys.md:114-153`), **P3, the local-first / edge
developer** (`:182-189`), and the two in-repo readers the brief separates out at
`_decomposition.md:215-219`: the **rule author** inside the testkit and **whoever reads the run**. Both
persona citations are secondary evidence; no persona in this initiative has been directly observed
(`…/personas-and-journeys.md:361-363`).

"Verification" for a prose deliverable is not a euphemism for "someone reads it". Each row names either
a deterministic check with a real path and command, or the **design-stage review** (project `DoD 7`) —
which the Testing brief deliberately assigns as the *only* instrument for project AC-006, *"named here
only so no story tries to invent a test for it"* (`_decomposition.md:776`). Where a machine check
exists downstream, the row names it and says which story runs it, so a deferred check is visibly
deferred rather than quietly absent.

| id | criterion | verification |
| --- | --- | --- |
| **AC-001** | **GIVEN** P2 has built a projection fixture whose storage genuinely cannot provide one of the port's guarantees, **WHEN** they open `_design.md` to learn who is entitled to write the sentence a consumer reads, **THEN** the DT-3 resolution names **exactly one** authoritative source and, for each of the three reason-writers that already exist in this tree — the fixture-written `Capability::declined` (`crates/happenstance-testkit/src/contract.rs:355-419`), the testkit-written `NO_STORE_LIMITS` / `NO_CEILING_REASON` (`:435-456`), and per-adapter prose — states which defers to which and why, so P2 is never left choosing between two policies; and it **cites** `.kb/open-questions/cf-40-fixture-limits-ownership.md` as authoritative rather than preferring a reading of it. | Design-stage review of `_design.md` § DT-3 (project `DoD 7`), against UX brief `AC-U01` (`_decomposition.md:84-96`). Deterministic half: `rg -n "Capability::declined\|NO_CEILING_REASON\|per-adapter" .bklg/from-contract-to-published-library/projection-store-freeze/_design.md` returns all three, and `rg -n "cf-40-fixture-limits-ownership" …/_design.md` returns a citation with no restatement of the atom's body. |
| **AC-002** | **GIVEN** P3 runs the suite on `wasm32-unknown-unknown`, the target BR-12 exists to protect, **WHEN** a projection capability is declined there, **THEN** DT-3's resolution has already accounted for it: the record states that `RuleOutcome::report` is a **measured** no-op on that target (`crates/happenstance-testkit/src/contract.rs:511-531`), that the projection wasm harness routes `skip_line` through `console_log!` exactly as `__emit_wasm` already does (`crates/happenstance-testkit/tests/memory_conformance_wasm.rs:25`), and that the machine-checked half of "told, with a reason" is a `RuleOutcome`-**value** assertion — a projection sibling of `capability_skips_are_reported` (`crates/happenstance-testkit/tests/mutation_coverage.rs:3184`) — never stdout. | Design-stage review against UX brief `AC-U09` / `AC-U11` and project `AC-005`. Deterministic half: `rg -n "wasm32\|__emit_wasm\|RuleOutcome" …/_design.md` shows all three claims present in DT-3's resolution. Machine half deferred, by design, to `projection-capability-skips` (HS-S0008) and `projection-suite-entry-point`'s wasm harness. |
| **AC-003** | **GIVEN** the implementer of `projection-capability-skips` (HS-S0008), whose story explicitly takes *"the projection capability set fixed by DT-3's recorded resolution rather than invented here"* (`_storymap.md:60`), **WHEN** they open this record, **THEN** the projection fixture's capability constants are **enumerated**, each carrying whether it is a MUST in `SECOND_HANDLE`'s sense and **who writes its reason** under AC-001's policy — at minimum the reset-refusal capability `refused_reset_changes_nothing` needs (`spec/SPECIFICATION.md:5200-5217`) — with PS-12's gate recorded as the `ProjectionProbe` const `READS_THROUGH_BATCH` rather than a fixture const; and if the set ends with **nothing declinable at all**, that outcome is recorded inside DT-3's resolution as a finding, not treated as licence to drop the reporting discipline. | Design-stage review against Architecture brief Note 5 (`_decomposition.md:526-534`), UX brief *What would make this brief wrong* (`:252-259`) and `.kb/decisions/0010-the-suite-must-prove-itself.md`. Deterministic half: HS-S0008's spec can name every constant it implements by quoting this record; a constant that story has to invent is this AC failing late. |
| **AC-004** | **GIVEN** an adapter author this repository did not write and does not supervise — the seventh adapter P2's persona generalises to (`…/personas-and-journeys.md:115-121`) — **WHEN** they ask whether the suite's bar is held for them, **THEN** `_design.md` names DT-8's arm **and its cost to them**: on the outside-author arm the extension surface is the documented pair `projection_store_conformance!` + `ProjectionProbe`, bounded at *one feature flag on a dependency the adapter already has and no new edge in the dependency graph* (`spec/SPECIFICATION.md:5015-5031`); on the narrow arm the recorded scope is placed **where an outsider meets it** — the testkit's own crate doc — and that file is named as `documented-extension-surface`'s (HS-S0015) obligation. Either way the arm is legible enough that HS-S0015 is scopable from this file alone. | Design-stage review against UX brief `AC-U02` (`_decomposition.md:97-108`) and project `AC-007`. Deterministic half: the arm and its obligation are quotable into HS-S0015's spec without opening `initiative.md`. Machine half conditional and deferred: if the outside-author arm is taken, HS-S0015 runs a fixture built from the documentation alone against the mutant-registry exactness check and the capability-skip rule (Testing brief row AC-007, `_decomposition.md:777`). |
| **AC-005** | **GIVEN** the rule author inside the testkit and the implementer of `owned-batch-port-shape` (HS-S0004), **WHEN** they open `## Items` and `## Signatures` to build against a shape someone agreed to, **THEN** the target trait is transcribed item for item — `type Batch;` with no lifetime, `fn begin(&self) -> Self::Batch` neither `async` nor fallible, `checkpoint -> Checkpoint`, `commit(batch, id, position, Authority) -> Result<(), CommitError<E>>`, `reset(batch, id) -> Result<(), ResetError<E>>`, `rollback`, plus `ProjectionProbe`'s five items — each signature carrying its `PS-*` clause id, and where any sentence disagrees with accepted ADR-0017 / ADR-0018 / ADR-0019, the **atom wins** and the record says which sentence yielded rather than editing the atom. | Design-stage review against `spec/SPECIFICATION.md:4632-4731`, `:4998-5013` (the specification wins on conflict) and `crates/happenstance-core/src/projection.rs:87-139`. Deterministic half: every signature in `## Signatures` carries a `PS-` id; `rg -n "PS-" …/_design.md` shows no unattributed signature. Compile-time proof is deferred to HS-S0004, whose Surface quality section is written against this block. |
| **AC-006** | **GIVEN** `CLAUDE.md`'s reader — fluent in the domain, new to idiomatic Rust — **WHEN** they meet a shape that looks gratuitously unusual, **THEN** `## What it costs a caller` has already named the alternative that lost, at the call site, for each of the four load-bearing choices: `Checkpoint` as a three-variant enum rather than `(Option<SequencePosition>, bool)` because the tuple can spell *"authoritative, never run — which means nothing"* (`spec/SPECIFICATION.md:4643-4658`); `CommitError` and `ResetError` staying two enums so `commit`'s caller never matches `Refused` (`:4722-4727`); `begin` infallible because `async` + fallible implies a round trip Neon's one-shot transport cannot afford (`:4884-4897`); `reset` taking the caller's own deletes because *"the port has no idea what the read model is"* (`:5165-5170`) — each stated **once**, sited in the item's own rustdoc rather than only in an ADR (RS-70-5, `standards/rust/70-rustdoc-obligations.md:243`), and each error carrying the adapter error as a type parameter, never a `String` (`standards/rust/30-error-taxonomy.md:15,71`). | Design-stage review against UX brief `AC-U03` – `AC-U05`. Deterministic half: four rationale sentences, four items, no sentence repeated across sections (the RS-70-5 density budget in NF-003). |
| **AC-007** | **GIVEN** an operator whose `reset` was declined by a store's own policy, **WHEN** they ask *which* policy declined it, **THEN** `_design.md` has decided out loud: either `ResetError::Refused` carries the store's stated reason, or it stays bare (`spec/SPECIFICATION.md:4676-4682`) and the record says where a caller is expected to find the reason instead, given the clause's own reasoning that *"the port supplies the mechanism; the domain decides what to protect"* (`:5210-5216`). Silence is the failure, not a passing default. | Design-stage review against UX brief `AC-U07` (`_decomposition.md:148-157`). Deterministic half: `rg -n "ResetError::Refused" …/_design.md` returns a decision sentence, not only a signature. Behavioural half is `reset-rules` (HS-S0012)'s `refused_reset_changes_nothing`. |
| **AC-008** | **GIVEN** a downstream story about to land one of the new items, **WHEN** it reads `## Visibility and stability`, **THEN** the record is specific enough to be **violated**: `#[non_exhaustive]` on all four new types; re-export placement at `crates/happenstance-core/src/lib.rs:98-124`; `ProjectionProbe` behind `conformance` with `#[cfg_attr(docsrs, doc(cfg(…)))]` copying the `memory` pattern; `MemoryProjectionStore` behind `memory`; `unstable-projection` recorded as the arm AC-A04 takes; **both** halves of the mount named — the export block *and* the feature table, because an item mounted at one and not the other is an item no adapter can name (Architecture brief Note 1, `_decomposition.md:339-349`); and the rustdoc hazard stated, an intra-doc link into a feature-gated module being a hard error this workspace has already paid for once (`crates/happenstance-testkit/src/lib.rs:112-132`). | Design-stage review against RS-40-5 (`standards/rust/40-public-surface-and-evolution.md:212`) and Architecture brief Notes 1 and 9. Deterministic half: every id in the Integration contract's **Public items** list has a visibility line, a feature line and a placement line — three lines per item, no item short. Compile-time proof deferred to HS-S0004 and `projection-probe-conformance-feature`. |
| **AC-009** | **GIVEN** P2 ten minutes into writing an impl, meeting `error[E0195] lifetime parameters or bounds on method 'commit' do not match the trait declaration` with, today, *nothing to copy from* (`references/adapter-shapes.md:186-194`), **WHEN** they open the port's page, **THEN** `## The doctest` carries the example text **and names the source item it will live on and the gate step that compiles it** (`cargo test --doc`, inside `cargo xtask ci`), because this record cannot compile it and a described example is not a checked one; and any `compile_fail` doctest demonstrating a reasonless declension is spelled **bare**, never `compile_fail,E0080` (`crates/happenstance-testkit/src/contract.rs:400-403`). | Design-stage review against UX brief `AC-U06` / `AC-U12` and `CLAUDE.md`'s *"a doctest in place of a mock"*. Deterministic half: `## The doctest` names a real target path (`crates/happenstance-core/src/projection.rs` and/or the `MemoryProjectionStore` module) and a real gate step. Compilation itself is `memory-projection-store`'s, under Testing brief row AC-012 (`_decomposition.md:782`). |
| **AC-010** | **GIVEN** an implementer in a later slice reaching for a shape that would compile, **WHEN** they check `## Anti-patterns` first, **THEN** each forbidden move is listed **with its evidence**: no `#[async_trait]` (`.kb/decisions/0001-async-port-flavours.md`); no `type Batch: Send;` (PS-36 — `trait_variant` copies the bound into the `!Send` flavour and breaks wasm32, `spec/SPECIFICATION.md:5601-5627`); no borrowing GAT anywhere on the fixture (five-ingredient rustc ICE, still reproducing on 1.97.1, `crates/happenstance-testkit/src/contract.rs:97-111`); no merging the two error enums; no `async fn begin`; no second skip vocabulary and no projection-local skip type (Architecture brief `AC-A06`, UX brief `AC-U08`); no literal position assertions; no hardening of `ProjectionId::new` as a side effect (`.kb/open-questions/projection-id-is-unvalidated.md`). | Design-stage review against `CLAUDE.md` *Binding constraints* and *The rule that matters*, plus RS-91-3 (`standards/rust/91-adapter-authoring-recipe.md:156-158`). Deterministic half: every entry carries a path or an atom id — an anti-pattern without evidence is an opinion, and `rg -n "Anti-patterns" -A 60 …/_design.md` shows none. |
| **AC-011** | **GIVEN** the repository owner, who signed the no-screen determination at the `/redkiln:plan` design gate on 2026-08-12 (`_design.md:96-101`), **WHEN** this record returns to that gate, **THEN** they can approve or decline the **amendment** without unpicking what they already approved: the no-screen determination, the `surfaces: []` manifest and the prior sign-off block survive **verbatim**; the added content occupies the bundled template's own section slots in the template's own order (`.redkiln/templates/_design.md`), replacing only the lines that read `N/A — no user-facing surface`, and only for the two non-visual surfaces the same file already enumerates at `_design.md:24-38`; and a **new sign-off block is appended unsigned**, naming what is added and what is untouched. | Design-stage review; `git diff` on `_design.md` shows the prior sign-off block and the `surfaces: []` fence unchanged (an added-lines-only diff over `:10-50` and `:92-101`), and the new block carries no signature. |
| **AC-012** | **GIVEN** this repository's rule that `.kb/` atoms are authored by `/redkiln:kb-ingest` from `.kb/_intake/` and that hand-writing them *"produces the directory layout of the process without the process"* (`CLAUDE.md`, reverted at `0269720`), **WHEN** this story merges, **THEN** the scope discipline is observable in the diff rather than asserted: no file under `crates/**`, `.kb/**` or `spec/**` changes, every changed path falls inside the PR-boundary block above, and any answer the record turned up that deserves an ADR appears **as a named gap for the runbook's ADR pass**, never as a minted decision. | `redkiln verify --grain story` (reads the first fenced block under **PR boundary** and fails on any file outside it) plus `git diff --name-only main...HEAD`; `redkiln validate` green on this story's item. |

## Interaction quality — the surface-quality substitution

This repository's `.redkiln/templates/spec.md` deliberately replaces the bundled interaction-quality
list with **Surface quality**, on the stated grounds that the bundled list is about screens and *"a Rust
library has the same hole in a different medium"* (`.redkiln/templates/spec.md:50-59`). This section is
that substitution, applied to a story whose medium is narrower still: a decision record. The RFC's two
families survive the translation intact — one is about how the *reader's* state is treated when the
record lands, the other about whether the record is genuinely composed or merely present — and the
substitution is not a softening: every invariant below is something a green `cargo xtask ci`, a green
`redkiln validate` and a filled-in template are all perfectly satisfied by.

**Every invariant that applies is an `AC-###` row in the table above.** `redkiln verify` extracts
acceptance criteria by matching a leading `| AC-001 |` cell or a `- AC-001:` bullet; a prose bullet here
would match neither, get no ledger row, and never be gated. This section only says which ids carry which
invariant and how each is checked.

### State invariants — how the reader's place is treated

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **In place, not a context jump** — the record lands *inside* the artifact the design review and the downstream stories already open, not in a new sibling file that sends the reader somewhere else. A record in this story's own folder would be constructed-but-unmounted. | **AC-011** (and the Integration contract's mount point) | `git diff --name-only` shows `_design.md` amended; no new record file under the story folder claims the same content. |
| **Non-occlusion** — added material never overwrites or hides what is already approved: the no-screen determination, the `surfaces: []` fence and the prior sign-off survive verbatim. | **AC-011** | Added-lines-only diff over `_design.md:10-50` and `:92-101`. |
| **Preserved position** — the bundled template's section headings and their order are kept, so a reader (and every downstream story's Surface quality section, which cites `## Signatures`, `## Visibility and stability` and `## Anti-patterns` by name) lands where it expects. | **AC-011**, **AC-005**, **AC-008**, **AC-010** | Heading order in `_design.md` matches `.redkiln/templates/_design.md`; no heading renamed, none reordered, none dropped. |
| **Reversibility** — the amendment is a separately appended, **unsigned** sign-off block, so declining it costs the reviewer nothing already banked. | **AC-011** | The new block exists, names what is added and what is untouched, and carries no signature. |
| **Reachability** — the library analogue of keyboard reachability: every public item the record creates is named at **both** places it can be reachable or invisible — the `lib.rs` export block and the feature table that gates it. | **AC-008** | Three lines per item (visibility, feature, placement) for every id in the Integration contract's **Public items** list. |
| **Escalation over absorption** — if the reviewer holds that the `N/A` determination was meant to cover the type surface too, the story reports a re-plan at its boundary rather than widening a section quietly. | **EC-001** (raised at the story boundary; not a silent pass) | Reported to the human at the design gate; `_decomposition.md:709-727` Note 10 is the standing instruction. |

### Composition invariants — taken from the signed-off `_design.md`

The signed-off design's binding content for this story is narrow and exact, and it is honoured rather
than re-decided: the **no-screen determination**, the **`surfaces: []` manifest**, and the enumeration of
the **two non-visual surfaces** — a type surface and a text surface — at `_design.md:24-38`. Every
composition invariant below is scoped to those two surfaces and no others.

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **Presentation exists at all** — the sections that read `N/A — no user-facing surface` are filled with real composed content: signatures with clause ids, rationale sited at the item, anti-patterns with evidence. A section filled with a bare list of type names is the prose equivalent of unstyled markup and fails this. | **AC-005**, **AC-006**, **AC-008**, **AC-010** | Design-stage review; plus the per-row deterministic checks (a `PS-` id on every signature, three lines per item, a path on every anti-pattern). |
| **Composition and placement** — content goes in the template's own slot: signatures in `## Signatures`, cost-to-caller in `## What it costs a caller`, states in `## The states the API must express`, forbidden moves in `## Anti-patterns`. Rationale that lands only in a narrative preamble is content in the wrong slot. | **AC-005**, **AC-006**, **AC-008**, **AC-010**, **AC-011** | Heading-by-heading review against `.redkiln/templates/_design.md`. |
| **Transience** — three tiers, decided rather than defaulted. *Persistent chrome*: the signature block and the visibility table, always in front of the reader. *Revealed at the point of use*: the alternative-that-lost sentence, which RS-70-5 puts in the item's own rustdoc where the reader meets it, once. *Opened on demand*: the long-form ADRs and `spec/SPECIFICATION.md` clause ranges, **cited by `file:line`, never pasted** — `CLAUDE.md`'s two-places rule keeps the atom canonical and the record's line citations resolvable to the long form. | **AC-005**, **AC-006**, **AC-010** | Review: no clause body or ADR body is inlined in bulk; every deep artefact appears as a citation. |
| **Density budget, with its numbers** | **AC-006** (four rationale sentences, four items, each stated **once**), **AC-005** (one `PS-` id per signature; nine `## Items` ids, matching the Integration contract's list exactly), **AC-008** (exactly three lines per public item), **AC-003** (at least one declinable capability, or a stated finding that there is none), **AC-009** (one doctest, with one named home and one named gate step) | Counted at review; each number is stated in the AC that owns it, so an over- or under-run is visible without judgement. |
| **Hierarchy** — the manifest stays `surfaces: []` at the top; beneath it exactly two named tiers, type surface and text surface, in the order `_design.md:24-38` already establishes; the type surface's items ordered as the trait declares them, not alphabetically. | **AC-005**, **AC-011** | Review against `_design.md:24-38`; the fenced `surfaces: []` block is unchanged. |
| **The design's named anti-patterns** — the signed-off block's approval covers *"the anti-patterns recorded above"* (`_design.md:96-101`), which today is an `N/A` line. This story fills that section from `CLAUDE.md`'s binding constraints and the Architecture brief rather than inventing one, and flags the gap to the reviewer instead of papering over it. | **AC-010**, and the flag itself under **Clarifications resolved during spec** | Review; each entry carries a path or atom id. |

## Error conditions

| id | Condition | Required response |
| --- | --- | --- |
| **EC-001** | The design-stage reviewer holds that `_design.md`'s `N/A — no user-facing surface` determination was meant to cover the **type** surface as well, not only the absent screen. | Then project `DoD 7` cannot be met by any story in this project. Report it at the story boundary as a **re-plan**; do not widen a section quietly to make the story land (`project.md` DoD 7; Architecture brief Note 10, `_decomposition.md:709-727`). |
| **EC-002** | An accepted ADR-0017 / ADR-0018 / ADR-0019 atom from `projection-decision-atoms` (HS-S0002) contradicts a sentence the record was about to write. | The **atom wins**. Write the atom's answer and state which sentence yielded. Never edit the accepted atom's body — `redkiln validate --kb` checks accepted decision atoms against `HEAD` and a correction is a new superseding atom, which is not this story's to write (`CLAUDE.md`, *Where the work lives*). |
| **EC-003** | The projection fixture's capability set comes out empty — every rule runnable by every conformant store, nothing declinable. | Record it **as a finding inside DT-3's resolution** (AC-003). It is not licence to drop the reporting discipline: a suite with nothing to decline cannot demonstrate initiative AC-05, and this project owns that promise (`_decomposition.md:252-259`). |
| **EC-004** | The DT-3 answer being drafted would require widening `Capability` or `RuleOutcome`, or minting a projection-local skip type or a second line shape. | Stop. Those types are shared with the event-store suite (`crates/happenstance-testkit/src/contract.rs:355-537`), so changing one is a change to a contract with an existing user; Architecture brief `AC-A06` and UX brief `AC-U08` both forbid the fork. Report it rather than absorbing it. |
| **EC-005** | The record's reasoning heads toward a **boundary-scoped** projection checkpoint. | Stop and file. That reopens ADR-0013's globally frozen visibility invariant (`.kb/open-questions/global-versus-per-boundary-visibility-invariant.md`; `project.md` Risks) — a new decision, not a paragraph in a design record. |
| **EC-006** | CF-40's atom is read as settled, or one of its two contradictory accepted readings is preferred in order to make DT-3's answer tidy. | AC-001 fails. The atom is explicit that no later wave has standing to prefer a reading; the record states what the projection fixture does *given* the contradiction, and says so. |
| **EC-007** | `redkiln verify --grain story` fails because a changed file falls outside the PR boundary. | Widen the boundary **in this spec, deliberately**, or split the story. A boundary widened to make a failing gate pass has stopped meaning anything (`.redkiln/templates/spec.md:93-104`). |
| **EC-008** | `ps-clause-pairing-sweep` returns *"systematic"* and the `PS-*` maturity markers this record transcribes are about to move. | Transcribe the markers **as they stand** and cite them; do not pre-apply the repair. The frozen-clause repair is `unstable-projection-gate-and-clause-disposition`'s and lands as a new decision atom, never a line edit (`_storymap.md:126`). |
| **EC-009** | The record's work produces an answer that deserves its own ADR. | Record it as a **named gap** for the runbook's ADR pass. Minting an ADR as a side effect is exactly the failure `CLAUDE.md` reverted once (`0269720`). |

## Non-functional

| id | Requirement | Why, and how it is observed |
| --- | --- | --- |
| **NF-001** | **Downstream self-sufficiency.** `owned-batch-port-shape` (HS-S0004), `projection-capability-skips` (HS-S0008) and `documented-extension-surface` (HS-S0015) must each be startable from `_design.md` plus their own spec, without opening `spec/SPECIFICATION.md` §4.0 to recover a signature or `initiative.md` to recover DT-8's arm. | These are the story's three `blocks:` edges. An unwritten section here is not a gap — it is a decision made silently by whoever needs it next. Observed by whether HS-S0004's Surface quality section resolves against this file's headings. |
| **NF-002** | **Citation precision.** Every normative sentence carries a real `file:line` range or an atom path. No invented ADR number, no invented API, no path that does not resolve. | The record's whole value is that a later reader can check it. Observed by spot-resolving citations at review; `cargo xtask spec-trace` protects the specification's own line ranges independently. |
| **NF-003** | **RS-70-5 density.** Each alternative-that-lost sentence appears **exactly once**, at the item, not repeated across `## Shape decision`, `## What it costs a caller` and the anti-patterns list. | Repetition is how a rationale rots: three copies are three things to update and two that go stale — the same argument `CLAUDE.md` uses for not restating the style guide. Observed at review (`standards/rust/70-rustdoc-obligations.md:243`). |
| **NF-004** | **Two-places discipline.** Decisions are linked to their `.kb/decisions/` atom; long-form records and clauses are cited by `file:line`. Nothing is pasted in bulk from either. | `CLAUDE.md` *Where the work lives*: deleting or duplicating the long form discards roughly 78% of the corpus, and `spec-trace` cites line ranges that exist only there. |
| **NF-005** | **Zero gate cost.** This story adds no CI step, no build time and no dependency; the affected set for `cargo xtask affected --base main` is empty. | Which is why the merge gate below is `redkiln`-side plus `--fast`, not a full `cargo xtask ci` — the full run is the *project* boundary's bar (`_decomposition.md:828-853`). Observed by the affected-set output being empty. |
| **NF-006** | **Evidence provenance stays honest.** Persona claims remain marked secondary; promotion into `.kb/product/` is HS-P0019's, not this story's. | `…/personas-and-journeys.md:361-363`. Observed at review: no persona sentence in the record is stated as observed fact. |

## Implementation notes (non-prescriptive)

Sequence, not prescription — the record's content is the implementer's, the ordering is what keeps it
honest.

- **Read the accepted atoms first, then draft.** ADR-0017 / 0018 / 0019 land in `projection-decision-atoms`
  and are accepted before this story starts (`blocked_by`). Drafting first and reconciling after is how a
  rejected alternative gets restated as a design choice — EC-002's failure mode, in slow motion.
- **Write DT-3's answer as a disposition table, not a verdict sentence.** Three reason-writers, three
  rows, one "defers to" column. The bar is disposal, not selection (UX brief `AC-U01`); a table makes an
  undisposed writer visible, a paragraph hides it.
- **Take the surfaces in the order `_design.md` already lists them** — type surface, then text surface —
  so the file reads as one document rather than two records stapled together.
- **Transcribe the trait from `spec/SPECIFICATION.md:4632-4731`, then diff it against
  `crates/happenstance-core/src/projection.rs:87-139`** and record the delta explicitly. The delta *is*
  HS-S0004's work order; a transcription that does not say what changes leaves that story to re-derive it.
- **`compile_fail` is spelled bare.** If the record specifies a `compile_fail` doctest for a reasonless
  declension, `compile_fail,E0080` is the *weaker* check on 1.97.1 — rustdoc silently ignores an error
  code it cannot match (`crates/happenstance-testkit/src/contract.rs:400-403`).
- **Do not "improve" `begin` back to `async fn begin() -> Result<…>`** for symmetry with `EventStore`.
  PS-6's infallibility is load-bearing for exactly one deployment and the reason is written down
  (`_decomposition.md:700-704`).
- **Leave the seventeen-versus-subset rule-count question where it is.** How many of §4.11's adapter
  rules ship is a scoping decision AC-014 audits (`_decomposition.md:672-676`), not something to settle in
  passing while writing signatures.
- **Append the unsigned sign-off block last**, after the sections are filled, and name in it exactly what
  is added and what is untouched — that block is what makes AC-011 checkable by a reviewer in one read.

## Tests and CI (merge gate)

Grounded in the Testing brief (`_decomposition.md:731-853`), which assigns project **AC-006** the tier
**None (design-stage artefact)** and project **AC-007** an Integration tier *conditional on the DT-8 arm
taken* — both stated there *"so nobody looks for a test that was never going to exist"*. The gate below
is therefore deterministic where it can be and explicitly review-shaped where it cannot, with every
deferred machine check named along with the story that runs it.

| Tier | Command / path | Proves |
| --- | --- | --- |
| **Static** | `redkiln validate` (this story's item and `_ledger.md`) | Frontmatter conformance; one ledger row per spec `AC-###`. |
| **Static** | `redkiln verify --grain story` — reads the first fenced block under **PR boundary** and this story's `_ledger.md` | **AC-012**: no file changed outside the boundary; every AC carries a satisfied row with non-placeholder evidence. |
| **Static** | `git diff --name-only main...HEAD` | **AC-012**: no path under `crates/**`, `.kb/**` or `spec/**`. |
| **Static** | `git diff -- .bklg/from-contract-to-published-library/projection-store-freeze/_design.md` | **AC-011**: an added-lines-only diff over `:10-50` and `:92-101`; the `surfaces: []` fence and the prior sign-off block byte-identical. |
| **Static (review-time greps)** | `rg -n "Capability::declined\|NO_CEILING_REASON\|cf-40-fixture-limits-ownership" …/_design.md`; `rg -n "PS-" …/_design.md`; `rg -n "wasm32\|__emit_wasm" …/_design.md` | **AC-001**, **AC-002**, **AC-005**: the three reason-writers named, the CF-40 atom cited, every signature clause-attributed, the constrained target accounted for. Cheap, mechanical, and each one fails loudly when a section was filled with a summary. |
| **Static** | `cargo xtask affected --base main` | **NF-005**: the affected set is empty — this diff cannot break a crate, which is the evidence that a full gate run is not this story's bar. |
| **Static** | `cargo xtask ci --fast` | The non-terminal project's standing bar (`CLAUDE.md` *Commands*); green here means the tree this record describes is still the tree that exists. |
| **Review (the only instrument for project AC-006)** | Design-stage review of `_design.md` at the `/redkiln:plan` design gate — `project.md` DoD 7 | **AC-001** – **AC-011**. The Testing brief names no test tier for AC-006 on purpose (`_decomposition.md:776`); this row is that decision made visible rather than worked around. |
| **Deferred — `memory-projection-store`** | `cargo test --doc -p happenstance-core`, inside `cargo xtask ci` | **AC-009**'s doctest actually compiles. This story specifies it and names its home; it cannot compile it, and says so. |
| **Deferred — `projection-capability-skips` (HS-S0008)** | A projection sibling of `capability_skips_are_reported` (`crates/happenstance-testkit/tests/mutation_coverage.rs:3184`), asserting on `RuleOutcome` values | **AC-002**'s and **AC-003**'s machine half: the declined capability is emitted, reasoned and distinguishable from a pass without reading stdout. |
| **Deferred — `projection-suite-entry-point`** | The new `projection_conformance_wasm.rs` harness under the mandatory wasm32 conformance-harness step (`xtask/src/main.rs`) | **AC-002** on the constrained target: the reason survives where `report` is a no-op. |
| **Deferred — `documented-extension-surface` (HS-S0015)** | A fixture built from the documentation alone, run against the mutant-registry exactness check and the capability-skip rule | **AC-004**, if the outside-author arm is taken (Testing brief row AC-007). |
| **Deferred — `owned-batch-port-shape` (HS-S0004)** | `cargo check` / `cargo xtask ci` over the landed trait | **AC-005** and **AC-008**: the signatures and visibility this record specifies are the ones that compile. |

## Risks and coupling (PR-scoped)

- **The mount point is an already-signed artifact.** Amending `_design.md` risks reading as a rewrite of
  something a human approved on 2026-08-12. Mitigated structurally, not by care: additive edits only,
  prior sign-off preserved verbatim, new block appended unsigned (**AC-011**), so the reviewer's decision
  surface is the amendment alone.
- **Ordering coupling with `projection-decision-atoms`.** This story is last in the
  `decisions-and-design-record` slice for a reason. If it is drafted before ADR-0017/0018/0019 are
  accepted, the record will restate a rejected alternative and an accepted, **immutable** atom will then
  contradict it (**EC-002**). The slice is implemented in one context, which makes the ordering easy to
  collapse by accident.
- **Three dependents, and the failure is silent.** `owned-batch-port-shape`, `projection-capability-skips`
  and `documented-extension-surface` each read a decision out of this file rather than making it. An
  unwritten section does not fail here; it fails as an invented answer three stories later, with no
  reviewer in the room (**NF-001**).
- **DT-8's arm sizes another story.** Choosing the narrow arm shrinks HS-S0015 substantially. That is a
  legitimate outcome and an illegitimate *motive* — the record must state the arm's reason in terms of the
  outside author's cost (**AC-004**), not in terms of work avoided.
- **CF-40's contradiction is inherited, not resolvable.** Two accepted, immutable decisions disagree
  (ADR-0015 claims and disclaims it; ADR-0012 is the adjacent claimant). The tidy-looking move — pick a
  reading — is precisely the one with no standing (**EC-006**).
- **Clause markers may move under the record's feet.** `ps-clause-pairing-sweep` runs first in this slice
  and could return *"systematic"*. The record transcribes markers as they stand and cites them; it does
  not pre-apply the repair (**EC-008**).
- **The template-drift tripwire.** `_design.md` is one of the six deliberately customised templates whose
  drift the `backlog` CI job asserts is **exactly** six. Amend the *artifact*, never
  `.redkiln/templates/_design.md`, and never run `redkiln adopt --templates` — it would overwrite all six
  customisations silently (`CLAUDE.md`, *Where the work lives*).
- **`_design.md`'s existing sign-off already refers to anti-patterns that are not there.** The approved
  block says what was approved includes *"the anti-patterns recorded above"*, while `## Anti-patterns`
  reads `N/A`. This story fills that section (**AC-010**) and flags the inconsistency to the reviewer; it
  does not edit the sign-off to match, and it does not treat the gap as authority to write whatever it
  likes there.

## Dependencies

**Blocks on** (must be merged and accepted first):

- `projection-decision-atoms` — ADR-0017 (batch ownership and write vocabulary, PS-4 – PS-15), ADR-0018
  (reset and checkpoint scope, PS-16 – PS-20) and ADR-0019 (apply-failure policy, PS-26 – PS-30) must be
  **accepted atoms** before this record is written, because accepted atoms outrank it and are immutable
  (**EC-002**). That story in turn blocks on `ps-clause-pairing-sweep`, which is why this story is third
  and last in the `decisions-and-design-record` slice (`_storymap.md:107`).

**Unlocks** (each reads a decision out of this record rather than making it):

- `owned-batch-port-shape` — implements the signature block **AC-005** transcribes and the visibility
  decisions **AC-008** records.
- `projection-capability-skips` — takes the capability set and reason-authorship fixed by **AC-003** under
  DT-3's resolution, *"rather than invented here"* (`_storymap.md:60`).
- `documented-extension-surface` — is scoped entirely by which DT-8 arm **AC-004** records.

Indirectly downstream through those three: `memory-projection-store` (the doctest home **AC-009** names),
`reset-rules` (**AC-007**'s `ResetError::Refused` decision) and `projection-probe-conformance-feature`
(**AC-008**'s `conformance` gate).

## Anchors (progressive disclosure)

Load-bearing depth, deferred rather than pasted. The Context pack above is sufficient to start; open
these at the moment named, and never in bulk.

| Anchor (real path) | Why it is load-bearing | When to open | Serves AC |
| --- | --- | --- | --- |
| `.bklg/from-contract-to-published-library/projection-store-freeze/_design.md` | The mount point and the binding signed-off design: the no-screen determination, the `surfaces: []` manifest, the two non-visual surfaces at `:24-38`, the eleven `N/A` sections at `:52-94` and the sign-off at `:92-101`. Everything this story writes goes here, additively. | First, before writing a word — it defines both the slots to fill and the lines that must survive verbatim. | AC-011 |
| `.bklg/from-contract-to-published-library/projection-store-freeze/_decomposition.md` | The UX brief's `AC-U01` – `AC-U12` (`:82-208`) is the bar both answers are judged against; Architecture brief Note 1's seam map (`:339-360`) and Note 5's fixture contract (`:517-544`) hand the capability-set question here by name; Note 9 (`:667-707`) lists what is left to this record on purpose. | Before drafting DT-3 (AC-U01) and DT-8 (AC-U02); again before enumerating capability constants. | AC-001, AC-003, AC-004 |
| `.kb/open-questions/cf-40-fixture-limits-ownership.md` | The declension-policy question's authority. It records a contradiction imported from two accepted, immutable decisions and is explicit that no later wave has standing to prefer a reading — the difference between citing it (required) and re-litigating it (AC-001 failing). | While writing DT-3's resolution, before choosing the authoritative source. | AC-001 |
| `crates/happenstance-testkit/src/contract.rs` | The three reason-writers in code: `Capability::declined`'s private-field `const fn assert!` (`:355-419`), the testkit-written `NO_STORE_LIMITS` / `NO_CEILING_REASON` and the doc explaining why they are *"the one place the skip machinery here differs"* (`:435-456`), `RuleOutcome` / `skip_line` / `report` with its two honest per-target limitations (`:458-537`), the ICE prohibition (`:97-111`) and the bare-`compile_fail` note (`:400-403`). | Open the specific range as each is cited; do not read the file whole. | AC-001, AC-002, AC-003, AC-010 |
| `spec/SPECIFICATION.md` | The target trait given verbatim at `:4632-4731`, `ProjectionProbe` and the orphan-rule coherence argument at `:4977-5031`, `Checkpoint`'s rejected tuple at `:4643-4658`, the two-enum split at `:4722-4727`, infallible `begin` at `:4884-4897`, `reset`'s deletes at `:5165-5170`, `Refused` at `:4676-4682` / `:5210-5216`, PS-36 at `:5601-5627`. The specification wins on conflict. | While filling `## Signatures` and `## What it costs a caller` — transcribe, do not re-design. | AC-005, AC-006, AC-007 |
| `crates/happenstance-core/src/projection.rs` | The port as it stands (`:1-139`) — its provisional header, its stated invariant, the current `Batch<'a>` and the infallible `ProjectionId::new` at `:44-64`. The record's signature block is the diff against this file, and that diff is HS-S0004's work order. | Immediately after transcribing the target trait, to compute and record the delta. | AC-005 |
| `.redkiln/templates/_design.md` | The section slots and their order — the composition the amendment must honour, and the file `CLAUDE.md` says asks for the public API surface, signatures, visibility, cost to a caller and a doctest in place of a mock. Amend the artifact, never this template. | Before adding any heading to `_design.md`. | AC-011 |
| `standards/rust/70-rustdoc-obligations.md` | RS-70-5 (`:243`) — the alternative that lost is named **once**, in the item's own rustdoc, where the reader meets it. This is what stops the four rationale sentences becoming twelve. | While writing `## What it costs a caller` and deciding where each rationale is sited. | AC-006 |
| `standards/rust/40-public-surface-and-evolution.md` | RS-40-5 (`:212`) — the capability-with-a-reason shape and the public-surface evolution rules (`#[non_exhaustive]`, what a `pub` item promises) that make `## Visibility and stability` specific enough to be violated. | While writing `## Visibility and stability`. | AC-008 |
| `standards/rust/91-adapter-authoring-recipe.md` | RS-91-3 (`:156-158`) — an `assert!` on an **associated** const is evaluated lazily and fails at codegen: `cargo build` and `cargo test` catch a reasonless declension, `cargo check` and `cargo clippy` do not. The fixture trait's rustdoc has to say this. | While recording reason-authorship and the anti-patterns list. | AC-003, AC-010 |
| `standards/rust/30-error-taxonomy.md` | `:15,71` — the adapter error travels as a type parameter, never a `String`. The single rule that keeps `CommitError<E>` / `ResetError<E>` from collapsing into stringly-typed errors under review pressure. | While writing the two error enums into `## Signatures`. | AC-006, AC-007 |
| `crates/happenstance-testkit/src/lib.rs` | The export-block and `__private` conventions (`:265-364` region) and, at `:112-132`, the intra-doc-link hazard this workspace already paid for once — the precedent that makes AC-008's rustdoc clause concrete rather than cautionary. | While writing placement and feature-gate lines for `ProjectionProbe` and `MemoryProjectionStore`. | AC-008 |
| `crates/happenstance-testkit/tests/mutation_coverage.rs` | `capability_skips_are_reported` at `:3184` — the existing `RuleOutcome`-value assertion the projection family must get a sibling of, and the proof that "distinguishable from a pass" is checkable without stdout. | While writing DT-3's constrained-target paragraph and naming the deferred machine check. | AC-002 |
| `crates/happenstance-testkit/tests/memory_conformance_wasm.rs` | `:25` — `emit = happenstance_testkit::__emit_wasm`, the existing pattern the projection wasm harness copies so the stated reason reaches `console_log!` rather than vanishing. | Same moment as the row above; cite the precedent instead of inventing a mechanism. | AC-002 |
| `.kb/decisions/0010-the-suite-must-prove-itself.md` | The proof obligation any capability this record invents must still satisfy — and the no-pass-rate discipline. A capability no fixture can decline is the reporting-machinery analogue of a rule no adapter can fail. | While enumerating the capability set, before declaring it settled. | AC-003 |
| `.kb/decisions/0001-async-port-flavours.md` | The `#[async_trait]` prohibition and the two-flavour derivation, which is the evidence behind two of the anti-patterns (no `async_trait`, no `type Batch: Send;`). Anti-patterns without evidence are opinions. | While writing `## Anti-patterns`. | AC-010 |
| `.kb/open-questions/projection-id-is-unvalidated.md` | Why `ProjectionId::new` stays infallible here: the inconsistency is real, owned elsewhere, and explicitly not repaired as a side effect of freezing the port around it. | While writing `## Anti-patterns` and resisting the tidy-up. | AC-010 |
| `.bklg/from-contract-to-published-library/initiative.md` | DT-3 and DT-8 in their original wording at the *Open design tensions* table, plus AC-04, AC-05, BR-13 and DoD 7 — the promises both answers are ultimately judged against, including *"a suite's own permissiveness becomes publicly scrutinised the moment outsiders make claims with it"* (`:427`). | Before writing DT-8's arm, to state the cost in the initiative's own terms. | AC-004 |
| `.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md` | P2's goal and fear (`:114-153`) and P3 (`:182-189`) — and the secondary-evidence qualification at `:361-363` that must be carried, not upgraded. | When framing who each half of the record is written for. | AC-004, AC-002 |
| `references/adapter-shapes.md` | `:186-194` — the `error[E0195]` transcript an implementer hits ten minutes in, with, today, nothing to copy from. The concrete thing `## The doctest` has to answer. | While writing `## The doctest` and its named home. | AC-009 |

## Clarifications resolved during spec

1. **Section naming.** This repository's `.redkiln/templates/spec.md` replaces the bundled
   *Interaction quality* list with **Surface quality** on stated grounds (`:50-59`). The section above is
   titled to satisfy both — it is the interaction-quality section, written in the medium this repository
   actually ships. No invariant was dropped in the translation; each was mapped and is carried by a
   numbered AC.
2. **AC count unchanged.** The twelve ids the first pass decided (`AC-001` – `AC-012`) are enumerated
   exactly, none added and none dropped. The `_ledger.md` carries one row per id.
3. **`_design.md`'s sign-off refers to anti-patterns that do not exist yet.** The approved block states
   that what was approved is *"the no-surface determination itself together with the anti-patterns
   recorded above"* (`:96-101`), while `## Anti-patterns` reads `N/A — no user-facing surface`. Resolved as
   follows: this story fills that section from `CLAUDE.md`'s binding constraints and the Architecture
   brief (**AC-010**) and **flags the inconsistency to the reviewer in the appended block**. It does not
   edit the existing sign-off to match, and it does not read the gap as licence to record anti-patterns of
   its own choosing.
4. **Why the record is not a `.kb/` atom.** Considered and rejected: `CLAUDE.md` is explicit that atoms
   are authored by `/redkiln:kb-ingest` from `.kb/_intake/`, and that hand-writing them produces the
   directory layout of the process without the process (reverted at `0269720`). The record therefore lives
   in `_design.md`, and any ADR-worthy finding it turns up is a **named gap** for the runbook's ADR pass
   (**EC-009**).
5. **What "verification" means for a prose deliverable.** The Testing brief assigns project AC-006 the
   tier *None (design-stage artefact)* and project AC-007 a tier conditional on the DT-8 arm
   (`_decomposition.md:776-777`). Rather than inventing a test to satisfy the template, every AC names
   either a deterministic `redkiln` / `git` / `rg` check or the design-stage review, and every machine
   check that exists only downstream is listed with the story that runs it. A row that could name neither
   would be decorative, and none does.
6. **DT-3's and DT-8's answers are not pre-empted here.** This spec fixes the **form** each answer must
   take and the obligations it creates; the answers themselves are the implementing context's, subject to
   the design-stage review. A spec that chose the arm would have made the design gate ceremonial.
7. **No conformance rule is named, and that is not an omission.** The story changes no port and adds no
   rule, so it is not adapter-observable; the Integration contract says so explicitly, and the Testing
   brief pre-authorises the absence so no story tries to invent a test for it.
