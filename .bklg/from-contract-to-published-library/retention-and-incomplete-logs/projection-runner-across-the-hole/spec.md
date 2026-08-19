---
item: HS-S0119
stage: spec
created: 2026-08-12T13:47:59.860Z
updated: 2026-08-12T13:47:59.860Z
template_sig: 87bbf1d0
rendered_sig: d54b6740
---

# Spec — The projection runner resumed across the hole

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` |
| Project | `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/project.md` |
| This spec | `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/projection-runner-across-the-hole/spec.md` |
| Key briefs | `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/_decomposition.md` — architecture: *Composition roots* 5 (`:124-134`), DA-7 (`:363-379`), DA-8 (`:381-397`), the readers data-flow (`:413-434`), the closing notes (`:556-570`); testing: the AC-006 row (`:608`) and the observation-tier note (`:728-736`) |
| Signed-off design | `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/_design.md` — **surfaces: N/A, approved 2026-08-12** (`:38-48`, `:90-95`). This project renders none, so no surface obligation falls on this story. |
| Grounding | `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/_grounding.md` §5 (`:187-206`) — every sibling project was still at `stage: storymap` when this was planned, so a dependency is cited *by id and by the mechanism that will produce it*, never by content |
| Story discover | `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/projection-runner-across-the-hole/discover.md` — the signal ledger, the four answered questions, and the named wrong implementations this spec is written against |
| Story map row | `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/_storymap.md:73` — slice `readers-against-the-hole`, row 2; the split reason at `:92`; merge order at `:137-140` |
| Roadmap pointer | `RUNBOOK.md:4626-4674` (phase 14; the proof artefact at `:4658-4662`, the "at least two rules" line at `:4667`), `RUNBOOK.md:165` (its status row) |

## One-line PR slice

Run composition root 5 — the projection runner in `happenstance` over
`crates/happenstance-core/src/projection.rs`'s checkpoint pump, arriving from HS-P0011 per
[ADR-0007](.kb/decisions/0007-projection-runner-decodes.md) — resuming from a checkpoint **across** the hole in
HS-S0114's instrument, in both the suffix and the scattered configuration, with its actual state recorded
against the hazard `crates/happenstance-testkit/src/suite.rs:1523-1531` already names (*"a projection resuming
across a gap then stalls forever with no error anywhere"*); and if no runner exists in the tree when this
story starts, that is a dependency failure to halt on and report, never a licence to write a bespoke one.

## Executive summary

This PR lands **an observation, not a capability**. It adds one native-only test target in
`happenstance-testkit` (shared with the slice-mate), one dev-dependency line, and one committed record of what
a real projection runner actually did when it resumed from a checkpoint that sits below a hole. It adds no
production code, no conformance rule, no error variant, no `spec/SPECIFICATION.md` edit and no port surface.

The **pointer**: `crates/happenstance-testkit/src/suite.rs:1490-1531` already writes the hazard down, in the
specification's own voice, inside `read_from_a_gap_position`'s assertion message. Nothing has ever been able to
*stage* it, because every store in this workspace holds its whole log. HS-S0114's instrument is the first one
that does not.

The **delta** is four decisions this spec makes rather than leaves open. The runner is **HS-P0011's**, driven
the way an application drives it, and its absence is a **halt**, not a licence — a hazard demonstrated against
a reader nobody ships proves nothing (`_decomposition.md:130-134`). The checkpoint is **derived from positions
the store actually assigned** and must sit *below* the hole, because a resume that never crosses the hole is
the sharpest way to satisfy this story's letter while exercising nothing. The evidence of a stall is
**deterministic** — an unchanged checkpoint after a bounded number of drive iterations, plus a captured empty
read — never a wall-clock wait, because a stall proved by a timeout is a flake waiting to happen. And the
expected result is **negative and recorded**: a checkpoint is a bare `SequencePosition`, `EventStore::read` has
no channel through which a store can say that history below it was destroyed, so this reader cannot be made
loud without one of DA-7's port primitives — which is ES-39, which is an escalation
(`.bklg/.../surface-diff-and-the-ac-012-escalation/`), and which this story states as a finding and does not fix.

What this PR is emphatically **not** is a rule. `suffix_store_is_distinguishable_from_a_young_store` is
HS-S0122's and is written only if ADR-0028 decides; a rule written here would be a rule written to the answer
someone expected, which is the sequencing note the whole project is arranged around (`_decomposition.md:543-546`).

## Context pack

Everything below is a decision this story must honour. Nothing here is optional and nothing here is a reading
list; the deeper artifacts sit behind the anchors table the second pass appends.

**The subject is a hazard already written down, and never once staged.** `read_from_a_gap_position`'s backwards
assertion says it in terms: *"`from` names a position, not an index: reading backwards from a position NOTHING
OCCUPIES must yield the next matching event below it. An equality seek or a `rowid` offset returns empty here,
and a projection resuming across a gap then stalls forever with no error anywhere"*
(`crates/happenstance-testkit/src/suite.rs:1523-1531`). That sentence is the whole motive for this story, and
the **predicted failure mode is a stall — silent, indefinite, and invisible to every existing check**, not a
panic. Any harness that expects a panic here is testing the wrong thing.

**Decision — the runner is HS-P0011's, and its absence halts this story.** Composition root 5 is *"the
projection runner in `happenstance`, over the checkpoint pump in `crates/happenstance-core/src/projection.rs`"*
(`_decomposition.md:124-134`), arriving from `typed-layer-and-alpha-release` (HS-P0011, rank 1; this project is
rank 5) per [ADR-0007](.kb/decisions/0007-projection-runner-decodes.md) — specifically its
`projection-trait-and-runner` story (`.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_storymap.md:57`),
which lands the application-facing `Projection` trait and its runner in `crates/happenstance/`. **This project
consumes that runner; it does not build one.** Today `crates/happenstance/src/lib.rs` is still 75 lines of
facade — `pub use happenstance_core::*;` at `:75` — and lists the typed projection runner under *Planned*
(`:49-52`). If that is still true when this story starts, the story **halts and reports the blocker**; it does
not write a bespoke runner, because a hazard demonstrated against a reader nobody ships proves nothing
(`_storymap.md:73`, `_decomposition.md:130-134`). The halt is part of the story, not an exception to it.

**Decision — the checkpoint pump has a second owner, and it is a second halt condition.** A runner commits its
read-model write and its checkpoint in one `ProjectionStore::commit`
(`crates/happenstance-core/src/projection.rs:126-132`), so running one needs a `ProjectionStore`
implementation. `MemoryProjectionStore` does not exist in the tree today and is **HS-P0010
`projection-store-freeze`'s AC-012** (`.bklg/from-contract-to-published-library/projection-store-freeze/project.md:218`),
already recorded as a project-level dependency by HS-P0011 itself
(`.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_storymap.md:141-146`). Writing a
throwaway one here would be freezing a fixture shape this project does not own — the same defect as writing a
throwaway runner, one crate down. Probe for **both**, halt on either.

**Decision — the resume must cross the hole, from a position the store actually assigned.** The checkpoint is
a `SequencePosition` and nothing else (`crates/happenstance-core/src/projection.rs:101-110`), fed to
`ReadOptions::from` *after advancing past it* (`:104-106`). Two ways to make that vacuous, both forbidden here:
resuming from the instrument's **first retained position** (the runner never meets the hole, the model looks
complete, the observation is empty), and **hard-coding** the checkpoint (which proves the harness resumed where
the harness said). So: the checkpoint is derived from what `append` actually returned and from the retained
predicate the instrument was built with, it sits **below** the hole, and at least one forgotten position lies
strictly between it and the head. This is `discover.md`'s literal-position tick, and it binds harder here than
in most stories because *a checkpoint is a position* and the obvious harness writes one down.

**Decision — the hole opens after the checkpoint, and it opens as a view change.** The real-world order is:
projection runs, prune runs, projection restarts. So the harness must narrow the instrument's retained view
*between* the first run and the resume. Two constructions are available and the discriminator is DA-2: a
second handle over the **same inner `MemoryEventStore`** carrying a narrower retained set, or a narrowing seam
on the instrument itself. Either is acceptable; **rebuilding the inner store through
`MemoryEventStore::restore` is not**, because that renumbers (`crates/happenstance-core/src/memory.rs:277-282`,
`:386-389`) and is ES-38's registered mutant, owned by `positions-are-not-reused-after-removal`. Whatever seam
this needs is **test-target-local and adds nothing to the `Fixture` trait**: DA-4's defaulted-`Capability`
shape exists for *conformance rules*, which reach a store only through `Fixture`; this story drives the
instrument's concrete type directly in the same crate's test target, so it needs no trait item at all — and
DA-4's closing instruction (*"Add nothing if the rules can be written without it"*, `_decomposition.md:307-309`)
therefore applies with full force.

**Decision — both configurations are run, and the comparison is the deliverable for DT-7.** DA-1's widening is
load-bearing here too: a suffix prune and a scattered regulated purge with survivors *below* the hole are
different journeys for this reader, and a suffix-only resume can never reach a survivor below the hole. The
question `dt-7-signal-shape-and-the-redaction-answer` is owed is precise: **does the runner's observable state
differ at all between the two?** If it stalls or silently completes identically under both, that is a concrete
argument that the three-way transient / benign-permanent / meaningful-permanent distinction cannot be
*delivered* to this reader without a new primitive — DT-7's cost side stated as a measurement rather than an
opinion (`discover.md`, *Questions*).

**Decision — the observation is captured as actual state, not as "it failed".** The tier is *observation*, the
closest thing this repository has to e2e (`_decomposition.md:608`), and `RUNBOOK.md:4658-4668` asks for what a
reader **actually does**. Five things are captured per configuration: the checkpoint resumed from, the events
actually received (their positions), the checkpoint held afterwards, whether it advanced, and whether anything
anywhere produced an error or a log line. **A stall is evidenced by two absences** — no progress and no error —
so both are captured deliberately rather than inferred from a test that simply did not fail. That is
`mutation_coverage.rs`'s own discipline against tests that record only that something failed, and CF-2's one
level up.

**Decision — the two wrong runner shapes are observed, not asserted.** `discover.md` names them and neither is
hypothetical. **`SilentlyResumingRunner`** is what any correct-looking runner does today: it resumes above the
hole, advances, and materialises a read model built from a log missing the events that define it — internally
consistent, monotonic checkpoint, intact transaction discipline, and wrong in exactly E2E-47's way
(`spec/E2E-CASES.md:1232-1256`). **`StallingRunner`** is the suite's own prediction: a `from` treated as an
index or an equality seek comes back empty across the gap and the runner stalls forever with no error anywhere.
Both are captured as **the actual materialised state and the actual stalled checkpoint**, in the harness, not
as prose about what would happen.

**Decision — no loudness is invented, and the negative result is the honest outcome.** `discover.md` answers
the project's sharpest question for this reader with the strongest "no" in the project: a checkpoint is a bare
`SequencePosition`, and `EventStore::read` (`crates/happenstance-core/src/store.rs:119-123`) has no channel to
say that history below or between was destroyed. Making it loud requires one of DA-7's four options
(`_decomposition.md:363-379`) — a floor, retained ranges, a third condition outcome, or a tri-state
`contains_event_id` — every one a port surface and a `0.3.0` this initiative's exit criteria do not contemplate.
So three moves are forbidden: adding an error variant, adding a panic to make the harness look loud, and
`warn!("gap detected")` — **loudness the caller cannot act on is not loudness**. The finding is written down
against project AC-011 with the missing surface named, and handed to `surface-diff-and-the-ac-012-escalation`
(project AC-012). This story stops at the finding.

**The fence, stated out loud because this is the story most likely to breach it.**
`read_from_a_gap_position`'s **ownership** is the sibling half of the open-question atom this project resolves
(`.kb/open-questions/es-38-and-gap-read-rules-are-unowned.md`), and it is deliberately not adopted
(`project.md:124-127`, `_decomposition.md:556-560`). The rule already exists and is already registered
(`crates/happenstance-testkit/src/suite.rs:1490`); its narrative will look exactly like this story's subject
while this story is being written. This story may **cite** its documented hazard as prior art — that is what it
is for — and may not extend it, re-home it, retire it, or close its ownership question. HS-S0121's narrowed
successor atom is what keeps that half open.

**The persona-journey slice.** The reader is an application author who runs a projection over an event store
that has been pruned — the device slice of E2E-46 and the regulated purge of E2E-47
(`spec/E2E-CASES.md:1203-1256`) — and the repository owner assembling ADR-0028's evidence. What this PR gives
them is the first honest answer to *"what does my dashboard actually do when the log under it has a hole?"*,
recorded as values rather than as reassurance. What they must **not** be able to conclude from it is that the
problem is now detected: it is not, and saying so plainly is this story's product.

## Integration contract

- **Archetype**: `capability` — a user-observable slice: a real application-facing runner, driven the way an
  application drives it, over a real store, producing a recorded observation an ADR cites. Not a component,
  not a double.
- **Slice / milestone**: `readers-against-the-hole`. Slice-mate: **`decision-model-and-ingest-observed`**
  (`.bklg/from-contract-to-published-library/retention-and-incomplete-logs/decision-model-and-ingest-observed/`),
  which owns composition roots 4 (`happenstance_core::read_decision_model`,
  `crates/happenstance-core/src/store.rs:321-331`) and 6 (`IngestStore::holds`,
  `crates/happenstance-sync/src/ingest.rs:164`). The two are implemented in **one context** and land as one
  integrated observation set — three readers, one finding. The split exists so that this story's HS-P0011 /
  HS-P0010 dependency is *"an isolated blocker, not a risk to the other two observations"* (`_storymap.md:92`);
  the slice-mate merges first (`_storymap.md:137-140`) and this story must not block it.
- **Mount point**: **`crates/happenstance-testkit/tests/readers_against_the_hole.rs`** — the slice's shared
  native-only observation target, with this story's body at
  `crates/happenstance-testkit/tests/readers_against_the_hole/projection_runner.rs`, included from the target
  root by `#[path = "readers_against_the_hole/projection_runner.rs"] mod projection_runner;`. That is the
  existing in-tree idiom and its rationale is written out at
  `crates/happenstance-testkit/tests/mutation_coverage.rs:50-65` — a test target resolves `mod foo;` against
  `tests/` itself, so without `#[path]` the support files land beside the targets and cargo compiles them as
  targets of their own. **One target for the slice, not two**: if the slice-mate has already created the file
  under a different name, adopt *its* name and add this module to it rather than opening a second target.
  It **must be reached only from this target** — never promoted into `src/`, never named by a published item.
- **Wires into**:
  - **`happenstance`'s projection runner and `Projection` trait** (`crates/happenstance/`) — HS-P0011's, per
    [ADR-0007](.kb/decisions/0007-projection-runner-decodes.md). Reached by a **new dev-dependency** on
    `happenstance` in `crates/happenstance-testkit/Cargo.toml`, added under the existing
    `[target.'cfg(not(target_arch = "wasm32"))'.dev-dependencies]` block (`:50-51`) so the four mandatory
    `wasm32` steps are provably untouched. The workspace entry already exists
    (`Cargo.toml:25`, `default-features = false`), so the member must name `features = ["std", "memory"]`
    explicitly, matching how testkit already spells `happenstance-core` (`crates/happenstance-testkit/Cargo.toml:29`).
  - **`happenstance_core::ProjectionStore`** (`crates/happenstance-core/src/projection.rs:88-139`) —
    `checkpoint` (`:110`), `begin` (`:117`), `commit` (`:126-132`), `rollback` (`:138`), plus `ProjectionId`
    (`:40-77`). The implementation is HS-P0010's `MemoryProjectionStore` behind the `memory` feature, which
    testkit already enables.
  - **HS-S0114's completeness instrument** (`crates/happenstance-testkit/tests/completeness_instrument.rs`) —
    the retained-set type, the decorating handle and its fixture. Shared, never re-created: if it landed as a
    single file, this story performs the bounded, behaviour-preserving extraction of its types into
    `crates/happenstance-testkit/tests/completeness_instrument/instrument.rs` (leaving the `event_store_conformance!`
    mounts where they are, so the suite is **not** re-expanded in a second target) and includes them by
    `#[path]`. The control mount stays green and unchanged.
  - **`happenstance_core::{EventStore, ReadOptions, SequencePosition, Query}`**
    (`crates/happenstance-core/src/store.rs:119-123`, `crates/happenstance-core/src/query.rs:267-289`) — the
    read half the runner drives, and the `from` whose position-not-index meaning is the hazard.
  - **`tokio`** (`crates/happenstance-testkit/Cargo.toml:50-51`) — already a native-only dev-dependency; the
    harness is `#[tokio::test]`, native-only, exactly as `fixture_instruments.rs` and `mutation_coverage.rs` are.
- **Renders surfaces**: **none.** `_design.md:38-48` records `## Surfaces`, `## Items` and `## Signatures` as
  *"N/A — no user-facing surface"*, approved by the repository owner on 2026-08-12 at the `/redkiln:plan`
  design sign-off gate (`:90-95`), with `design.capture` a **declared** skip. The design's binding content for
  this story is its framing paragraph (`:21-36`), which names this project's "readers" as `read_decision_model`,
  a projection runner and `IngestStore::holds` — Rust code paths, not a screen — and records that *"no route,
  no DOM selector, no component exists to enumerate."*
- **Conformance rule(s)**: **none added, and none may be.** This is an observation target, not a rule: nothing
  is written into `crates/happenstance-testkit/src/suite.rs` and nothing is registered in
  `for_each_event_store_rule!` (`crates/happenstance-testkit/src/registry.rs:94`). Two independent reasons, and
  both bind: a rule written before ADR-0028 is a rule written to the answer someone expected
  (`_decomposition.md:543-546`), and CF-27's own rule is `cf-27-rule-or-recorded-refusal`'s to write **or to
  record as unwritable** (project AC-007). `read_from_a_gap_position` is cited as prior art and is not touched.
- **Clause(s)**: discharges none and amends none. It **supplies evidence** to ES-39
  (`spec/SPECIFICATION.md:4325-4349`, `[DEFERRED]`, the primitive question) and to ES-40 (`:4351-4379`,
  `[PROVISIONAL]`, whose named falsifier is CF-27's instrument), and it is one of the three readers CF-27's
  experiment (`:8034-8060`) exists to make possible. **No `spec/SPECIFICATION.md` edit is in this PR** — marker
  movement is `marker-moves-and-spec-trace-green`'s, and a marker moved before its rule exists fails
  `spec-trace` checks 4 and 6 (`xtask/src/spec_trace.rs:700-711`, `:727`). ES-37 and ES-38 (`:4274-4297`,
  `:4299-4323`) are `[FROZEN]` context only and are not edited.
- **Advances DoD scenario**: initiative DoD **15** — *"Incomplete logs have an answer on disk … a store that
  holds only a suffix of its own log is **exercised against a reader**, and the reader either fails loudly or
  the refusal to define this is recorded as a decision"*
  (`.bklg/from-contract-to-published-library/initiative.md:402-404`). This story lands the projection reader —
  and, on the evidence `discover.md` already assembled, it lands the *"or"* branch: the reader does **not** fail
  loudly, and that is recorded rather than repaired. It also supplies half of `RUNBOOK.md:4658-4662`'s proof
  artefact (*"the suffix store, committed, with a runner and an ingest path that both fail against it"*).

## PR boundary

```
crates/happenstance-testkit/tests/**
crates/happenstance-testkit/Cargo.toml
.bklg/from-contract-to-published-library/retention-and-incomplete-logs/projection-runner-across-the-hole/**
```

**In this PR**

- The slice's shared observation target `crates/happenstance-testkit/tests/readers_against_the_hole.rs` (or an
  additional module inside it, if the slice-mate created it first) plus this story's module
  `crates/happenstance-testkit/tests/readers_against_the_hole/projection_runner.rs`.
- One manifest line: `happenstance` as a **native-only dev-dependency** of `happenstance-testkit`
  (`crates/happenstance-testkit/Cargo.toml`, the `cfg(not(target_arch = "wasm32"))` dev-dependencies block),
  with a one-line comment saying why it is dev-only and native-only, in the manifest's established voice.
- If, and only if, HS-S0114 landed the instrument as a single file: the bounded, behaviour-preserving
  extraction of its types into `crates/happenstance-testkit/tests/completeness_instrument/instrument.rs`, with
  the `event_store_conformance!` mounts left exactly where they are and green.
- The committed observation record and the finding, in this story's own backlog folder.

**Explicitly not in this PR**

- **No production code anywhere.** No `crates/happenstance/src/**`, no `crates/happenstance-core/src/**` — no
  runner, no `Projection` impl shipped as library code, no convenience seam added to `happenstance` to make the
  runner easier to point at the instrument.
- **No `crates/happenstance-testkit/src/**` change** — no rule body in `suite.rs`, no line in
  `for_each_event_store_rule!`, no item on `Fixture`, nothing in `src/fixtures.rs`.
- **No mutant and no `REGISTRY` row.** `mutation_coverage.rs`'s registry is the `owed-rules-and-mutants`
  slice's; the two wrong runner shapes here are *observed subjects*, not registered `Defect`s.
- **No `spec/SPECIFICATION.md` edit**, no marker move, no `(new)` removal.
- **No `.kb/` file and no `.kb/_intake/` staging.** ADR-0028 is `adr-0028-and-the-open-question-wave`'s, and it
  cites this story's record.
- **No change to `read_from_a_gap_position`** — not its body, not its registration, not its ownership.
- **No port-surface change of any kind**, and no proposal made in code. The missing-surface finding is prose
  handed to `surface-diff-and-the-ac-012-escalation`.
- **The ingest and decision-model observations** (composition roots 4 and 6) — the slice-mate's, including
  whatever target the ingest half needs in `crates/happenstance-sync/tests/`.

**Merge DoD (one line)** — `cargo xtask affected --base main` (`.redkiln/config.yaml:40`) green with the new
observation target running and every existing conformance target unchanged, the observation record and the
finding committed, and `git diff --stat` touching nothing outside the three globs above.

## Behavior and interfaces

Names below are proposals except where marked **binding**. The harness drives HS-P0011's runner through its own
public API; where that API's exact spelling is not knowable until it lands, the row states the **obligation**
the harness must still satisfy rather than a signature it cannot yet cite.

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **The dependency probe runs first, and its failure is a halt** | **Binding.** Before anything is written, confirm (1) an application-facing projection runner exists in `crates/happenstance/` — today `lib.rs` is a 75-line facade whose only item is `pub use happenstance_core::*;` (`:75`) and which lists the runner under *Planned* (`:49-52`) — and (2) an in-memory `ProjectionStore` exists, HS-P0010's `MemoryProjectionStore` behind the `memory` feature. If either is absent, **halt**: report the blocker by story id (HS-P0011 `projection-trait-and-runner`; HS-P0010 AC-012), leave the story's spec and this finding in place, and write no runner and no throwaway projection store. A bespoke runner would demonstrate the hazard perfectly and prove nothing, and would quietly convert a dependency failure into a finished story | `crates/happenstance/src/lib.rs:49-52`, `:75`; `.bklg/.../typed-layer-and-alpha-release/_storymap.md:57`, `:141-146`; `.bklg/.../projection-store-freeze/project.md:218`; `_decomposition.md:130-134` (and `:568-570`, the same refusal stated for the ingest reader); `_storymap.md:73` |
| **The runner is driven as an application drives it** | A real `Projection` implementation over decoded events, a real `ProjectionStore` (`MemoryProjectionStore`), and the instrument as the `EventStore`. The harness supplies the domain type and the projection body and nothing else — it does not reimplement nomination, decoding, `begin`/`commit` ordering or checkpoint advancement, because a harness that re-implements the runner observes the harness | `crates/happenstance-core/src/projection.rs:88-139`; [ADR-0007](.kb/decisions/0007-projection-runner-decodes.md); `_decomposition.md:124-134` |
| **The checkpoint is derived, and it sits below the hole** | **Binding.** The resume position is taken from what `append` actually returned during the seeding phase and from the retained predicate the instrument was built with — never a literal, never the instrument's first retained position. At least one **forgotten** position must lie strictly between the checkpoint and the head, so the resume genuinely crosses the hole. A run that cannot establish that invariant fails the harness loudly rather than recording a vacuous observation | `crates/happenstance-core/src/projection.rs:101-110` (`checkpoint` is a bare `SequencePosition`), `:104-106` (`from`, after advancing past it); `discover.md` *Gate: Discover*, the literal-position tick; CF-6 (no literal position values) |
| **The hole opens *after* the checkpoint, as a view change** | Seed with the full view, run the runner to a checkpoint, **then** narrow the retained view and resume — the real order (projection runs, prune runs, projection restarts). Implemented either as a second handle over the same inner `MemoryEventStore` with a narrower retained set, or as a narrowing seam on the instrument; **never** by rebuilding through `MemoryEventStore::restore`, which renumbers and is ES-38's registered mutant. Any seam is local to this test target and adds **nothing** to the `Fixture` trait — DA-4's defaulted-const shape is for conformance rules reaching a store through `Fixture`, and this harness holds the concrete type | `crates/happenstance-core/src/memory.rs:277-282`, `:386-389`, `:163`; `spec/SPECIFICATION.md:4321-4323`; `_decomposition.md` DA-2 (`:173-232`), DA-4 (`:276-309`) |
| **Both configurations are run** | The **suffix** case (a prune keeping only the top of the log) and the **scattered** case (a regulated purge with survivors *below* the hole), constructed from HS-S0114's retained-set type without editing it. A suffix-only run cannot reach a survivor below the hole, which is the case E2E-46/E2E-47 are about and the one a floor would ship looking correct against | `spec/SPECIFICATION.md:4336-4341`, `spec/E2E-CASES.md:1203-1256`; `_decomposition.md` DA-1 (`:150-171`) |
| **Five values are captured per configuration** | **Binding.** (1) the checkpoint resumed from; (2) the positions of the events the runner actually received; (3) the checkpoint held afterwards; (4) whether it advanced; (5) whether anything anywhere produced an error or a log line. Items (4) and (5) are **absences** in the stall case and must be captured deliberately — asserted as absences — not inferred from a test that merely did not fail | `RUNBOOK.md:4658-4668`; `_decomposition.md:608`, `:728-736`; `crates/happenstance-testkit/tests/mutation_coverage.rs` module docs (*"Why the wrong implementations are no longer here"*) |
| **A stall is proved deterministically, never by waiting** | **Binding.** Evidence of a stall is: the runner driven a bounded number of iterations, the read across the gap captured as its actual (empty or short) result, and the checkpoint asserted **unchanged** afterwards. No `sleep`, no wall-clock timeout, no "it didn't finish in 5s". A stall proved by a timeout is a flake, and reading the clock is the discipline CF-33 imposes on rules and this harness inherits by choice | `crates/happenstance-testkit/src/suite.rs:1523-1531`; CF-33 (no clock); `crates/happenstance-testkit/tests/fixture_instruments.rs` (native-only `#[tokio::test]` precedent) |
| **The silent-success shape is captured as a value, not a verdict** | Where the runner *does* advance — the `SilentlyResumingRunner` case — the recorded artefact is the **materialised read-model state itself**, shown to be built from a log missing the events that define it, beside the state the same projection produces over the unforgotten inner log. "The model was wrong" is not an observation; the two values side by side are | `spec/E2E-CASES.md:1232-1256` (E2E-47); `discover.md` *The wrong implementation*; `_decomposition.md:728-736` |
| **The two configurations are compared, and the comparison is DT-7's evidence** | Whether the runner's observable state differs **at all** between the suffix prune and the scattered purge is recorded either way. "Identical" is a result, not a null result: it is the concrete argument that a three-way transient / benign-permanent / meaningful-permanent distinction cannot be delivered to this reader without a new primitive, and it is handed to `dt-7-signal-shape-and-the-redaction-answer` as a measurement | `project.md:228-231` (AC-009); `_storymap.md:74`; `discover.md` *Questions*, DT-7 |
| **No loudness is invented** | **Binding.** No new error variant, no panic added to make the harness look loud, no `warn!("gap detected")`. Loudness the caller cannot act on is not loudness; a log line inside a harness reads as loudness in a report and is invisible to the caller, who still receives a stream of events and a checkpoint and can branch on neither | `discover.md` *The wrong implementation*, third bullet; `crates/happenstance-core/src/store.rs:119-123` (no channel exists); `_decomposition.md` DA-7 (`:363-379`) |
| **The finding names the missing surface and stops** | The written outcome states: this reader cannot be told loudly without a port primitive it does not have; the primitive is ES-39's and the option set is DA-7's four rows — a floor, retained ranges, a third condition outcome, a tri-state `contains_event_id` — with the version consequence (a `0.3.0` this initiative's exit criteria do not contemplate); and **no such change is made here**. It is recorded against project AC-011 and handed to `surface-diff-and-the-ac-012-escalation` (project AC-012) | `project.md:215-218` (AC-006's second sentence), `:236-243` (AC-011, AC-012); `_decomposition.md:363-379`; `.bklg/from-contract-to-published-library/_decomposition.md` gate decision 4 |
| **`read_from_a_gap_position` is cited, never touched** | **Binding.** Its body (`crates/happenstance-testkit/src/suite.rs:1490`) and its registration are unchanged; its documented hazard is quoted as prior art and nothing more. Its **ownership** stays open — that is the sibling half of the open-question atom, kept open by HS-S0121's narrowed successor. `git diff` over `crates/happenstance-testkit/src/**` is empty | `project.md:124-127`; `_decomposition.md:556-560`; `.kb/open-questions/es-38-and-gap-read-rules-are-unowned.md` |
| **Native-only, and `wasm32` is provably unaffected** | The target opens with `#![cfg(not(target_arch = "wasm32"))]` and the new dev-dependency sits in the `cfg(not(target_arch = "wasm32"))` block, so the four mandatory `wasm32` steps in `cargo xtask ci --fast` are green and unchanged. The runner is an async, native, tokio-driven thing; nothing here claims the edge target | `crates/happenstance-testkit/Cargo.toml:50-53`; `crates/happenstance-testkit/tests/mutation_coverage.rs:46`; `CLAUDE.md` *Commands* |
| **The dev-dependency is additive and not a surface** | A dev-dependency is invisible to downstream consumers and to `cargo-semver-checks`, so it is not an AC-011 breaking change — but it is recorded rather than assumed, together with its one real consequence: `happenstance-testkit` is publishable, so `happenstance` must be on the registry before testkit publishes, which is already HS-P0016's publish order. The workspace entry (`Cargo.toml:25`) carries `default-features = false`, so the member names `features = ["std", "memory"]` | `crates/happenstance-testkit/Cargo.toml:4-13` (the crate's own semver note), `:29`; `Cargo.toml:25` |
| **The instrument is shared, never re-created, never promoted** | One instrument in the tree. It stays in `crates/happenstance-testkit/tests/`, is reached by `#[path]` module include from this target, and nothing in the public API of `happenstance-core`, `happenstance` or `happenstance-testkit` names it. A second copy of the instrument written "to keep the targets independent" is the same defect as a bespoke runner, one layer over | `crates/happenstance-testkit/tests/mutation_coverage.rs:50-65`; `crates/happenstance-testkit/Cargo.toml:23-26`; `project.md:244-247` (AC-013) |

## Data and migrations

**N/A — no schema, no persisted format, no migration.** Everything this story touches is in-memory for the
lifetime of one test: the instrument's inner `MemoryEventStore` (`crates/happenstance-core/src/memory.rs`), the
`MemoryProjectionStore` the runner commits into, and the checkpoint itself, which is a `SequencePosition`
(`crates/happenstance-core/src/projection.rs:101-110`) and not a stored row. Nothing this story writes is read
by a later process, and no `serde` derive, envelope type or feature is added — the wire format of
[ADR-0016](.kb/decisions/0016-the-wire-format.md) is untouched.

**One artifact is committed, and it is data rather than schema.** The observation record — five captured values
per configuration, plus the side-by-side read-model states and the comparison between the suffix and scattered
runs — lands as a companion note in this story's own backlog folder,
`.bklg/from-contract-to-published-library/retention-and-incomplete-logs/projection-runner-across-the-hole/`,
where `adr-0028-and-the-open-question-wave` and `dt-7-signal-shape-and-the-redaction-answer` read it. It is
committed **in addition to**, never instead of, the harness assertions on the actual values: a record a test
does not also assert is a record that rots silently, which is the failure `RUNBOOK.md:4658-4668` and
`_decomposition.md:728-736` are both written against.

## Acceptance criteria

Every criterion below is stated from a persona's goal crossing the whole stack, not from a capability. The
personas are the initiative's own, carried from
`.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md` — **Persona 1, the
application author** (`:42`), whose journey *"Choose a contract before a database"* ends in a projection they
run in production, and **Persona 4, the evaluator** (`:249`), whose journey *"Decide in one sitting"* ends in
adopt or decline **for a stated reason** (`.bklg/from-contract-to-published-library/initiative.md:227-252`).
This story's product for both is the same artefact: an honest answer on disk about what a projection does when
the log under it has a hole (initiative DoD 15, `initiative.md:402-404`).

All eight tests live in `crates/happenstance-testkit/tests/readers_against_the_hole/projection_runner.rs`,
reached only through the slice's shared target `crates/happenstance-testkit/tests/readers_against_the_hole.rs`.
Test names are proposals; the obligation each carries is binding.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | GIVEN the application author's dashboard is driven by the projection runner `happenstance` actually ships — HS-P0011's, per [ADR-0007](.kb/decisions/0007-projection-runner-decodes.md) — WHEN this story runs its observation, THEN the harness drives **that** runner and HS-P0010's `MemoryProjectionStore` and constructs neither itself; and WHEN either is absent from the tree, THEN the story **halts**, reports the blocker by story id (HS-P0011 `projection-trait-and-runner`; HS-P0010 `projection-store-freeze` AC-012) as a note in this story's own folder, and writes no bespoke runner, no throwaway projection store and no code at all — because a hazard demonstrated against a reader nobody ships tells the application author nothing about the runner they will actually deploy | `dependency_probe_binds_the_shipped_runner` — the harness's runner and projection-store types resolve through `happenstance`'s and `happenstance_core`'s public API, so a local substitute cannot compile in their place; plus the boundary check `rg -n "impl .*ProjectionStore|struct .*Runner" crates/happenstance-testkit/tests/readers_against_the_hole/` returning nothing. On the halt branch: the blocker note exists in the story folder and `git diff --stat` shows no file under `crates/**` |
| AC-002 | GIVEN the application author ran a projection, then pruned the store, then restarted the projection, WHEN the runner resumes, THEN it resumes from a checkpoint **derived** from the positions `append` actually returned and from the retained predicate the instrument was built with — never a literal, never the instrument's first retained position — and at least one **forgotten** position lies strictly between that checkpoint and the head, so the resume genuinely crosses the hole rather than starting above it and reporting a complete-looking model | `resume_checkpoint_is_derived_and_below_the_hole` — asserts the checkpoint is one the store assigned, asserts a non-empty set of forgotten positions strictly between it and the head, and fails the harness loudly rather than recording a vacuous observation if that invariant cannot be established. No literal position value appears in the target (CF-6; `discover.md` *Gate: Discover*) |
| AC-003 | GIVEN the real-world order is *projection runs, prune runs, projection restarts*, WHEN the harness stages the hole, THEN it narrows the instrument's retained **view** between the first run and the resume — a second handle over the same inner `MemoryEventStore` with a narrower retained set, or a target-local narrowing seam — and never rebuilds through `MemoryEventStore::restore`, so the positions the runner already checkpointed still mean the same thing after the prune; the seam adds nothing to the `Fixture` trait and the existing conformance mounts stay green and unchanged | `hole_opens_after_the_checkpoint_without_renumbering` — asserts the positions observed before the narrowing are byte-identical to the same events' positions after it, and that the checkpoint taken pre-prune is still a valid `from` post-prune. Control: `cargo test -p happenstance-testkit --test completeness_instrument` green and unchanged; `git diff` over `crates/happenstance-testkit/src/**` empty |
| AC-004 | GIVEN two journeys the application author actually lives — the device slice that keeps only a suffix (E2E-46) and the regulated purge that leaves survivors *below* the hole (E2E-47) — WHEN the runner is resumed under each, THEN five values are captured **and asserted** per configuration: the checkpoint resumed from, the positions of the events the runner actually received, the checkpoint held afterwards, whether it advanced, and whether anything anywhere produced an error or a log line — recorded as values the evaluator can read, never as "the reader failed" | `observation_captures_five_values_under_both_configurations` — one parameterised observation per configuration, each asserting on all five captured values, with the captured record written to this story's folder as data (`_decomposition.md:608`, `:728-736`; `RUNBOOK.md:4658-4668`) |
| AC-005 | GIVEN the suite already predicts that a projection resuming across a gap *"stalls forever with no error anywhere"* (`crates/happenstance-testkit/src/suite.rs:1523-1531`), WHEN the runner meets that shape, THEN the stall is evidenced **deterministically** — the runner driven a bounded number of iterations, the read across the gap captured as its actual empty or short result, the checkpoint asserted **unchanged**, and the absence of any error or log line asserted as an absence — with no `sleep`, no wall-clock timeout and no "it didn't finish in time", because a stall proved by a clock is a flake the application author will later be told to ignore | `stall_is_evidenced_by_unchanged_checkpoint_and_no_error` — bounded-iteration drive loop, asserts checkpoint equality before/after, asserts the captured read result, asserts the error-and-log channel is empty. `rg -n "sleep|Duration::from|Instant::now|timeout" crates/happenstance-testkit/tests/readers_against_the_hole/projection_runner.rs` returns nothing (CF-33's no-clock discipline, inherited by choice) |
| AC-006 | GIVEN the application author whose dashboard looks perfectly healthy after a prune, WHEN the runner does advance across the hole instead of stalling, THEN the recorded artefact is the **materialised read-model state itself**, placed beside the state the same projection produces over the unforgotten inner log, so the two values exhibit the divergence — internally consistent, monotonic checkpoint, intact transaction discipline, and wrong in exactly E2E-47's way — rather than a sentence claiming the model was wrong | `read_model_across_the_hole_differs_from_the_unforgotten_model` — runs the same `Projection` twice, once over the narrowed view and once over the undecorated inner store, asserts on both materialised states and on the specific difference, and commits both into the observation record (`spec/E2E-CASES.md:1232-1256`) |
| AC-007 | GIVEN the repository owner must resolve DT-7 — one undifferentiated incompleteness signal, or the transient / benign-permanent / meaningful-permanent distinction — WHEN they read this story's record, THEN they find whether the runner's observable state differs **at all** between the suffix prune and the scattered purge, recorded either way; *identical* is a result, not a null result, and is handed to `dt-7-signal-shape-and-the-redaction-answer` as the measurement that a three-way distinction cannot be delivered to this reader without a new primitive | `suffix_and_scattered_observations_are_compared` — asserts on the comparison of the two captured observation sets (equal or differing, with the difference named), and the committed record states the comparison in one sentence the DT-7 story can cite (`project.md:228-231`; `_storymap.md:74`) |
| AC-008 | GIVEN the evaluator deciding in one sitting, WHEN they read this story's finding, THEN they learn that this reader **cannot** be told loudly without a port primitive it does not have — a checkpoint is a bare `SequencePosition` and `EventStore::read` has no channel to say history below it was destroyed — with the primitive named as ES-39's and the option set as DA-7's four rows (a floor, retained ranges, a third condition outcome, a tri-state `contains_event_id`) and its version consequence (a `0.3.0` this initiative's exit criteria do not contemplate); AND no such change is made here — no new error variant, no panic added to make the harness look loud, no `warn!("gap detected")`, no port-surface edit — the finding being recorded against project AC-011 and handed to `surface-diff-and-the-ac-012-escalation` (project AC-012) | `finding_names_the_missing_surface_and_changes_nothing` — the committed finding note in this story's folder names the surface, the option row and the version consequence; the boundary assertion is `git diff --stat` touching nothing under `crates/happenstance-core/src/**`, `crates/happenstance/src/**` or `crates/happenstance-testkit/src/**`, and `rg -n "warn!|error!|panic!" crates/happenstance-testkit/tests/readers_against_the_hole/projection_runner.rs` returning nothing outside an assertion message |

**Coverage of the traced project AC.** Project **AC-006** (*"a projection runner and an ingest path are each run
against the instrument, and each terminates with a named, non-silent outcome; where either cannot be made to
fail loudly without a surface the port does not have, that fact is recorded against AC-011 rather than papered
over"*, `project.md:215-218`) is discharged in both halves: the run and its named outcome by AC-001 through
AC-007, and the second sentence — the recorded fact — by AC-008. The ingest and decision-model halves of project
AC-006 are the slice-mate's (`_storymap.md`, *Coverage*, AC-006 row).

## Interaction quality

**No surface is rendered, and that is a signed-off determination rather than an omission.** The project's
`_design.md` records `## Surfaces`, `## Items`, `## Signatures`, `## The states the API must express` and
`## Anti-patterns` as *"N/A — no user-facing surface"* (`:38-88`), approved by the repository owner on
2026-08-12 at the `/redkiln:plan` design sign-off gate with `design.capture` a **declared** skip (`:90-95`).
Its framing paragraph names this project's readers as `read_decision_model`, a projection runner and
`IngestStore::holds` — *"Rust code paths, not a screen a human looks at… no route, no DOM selector, no
component exists to enumerate"* (`:21-36`). So the RFC §6.7/D6 **COMPOSITION** family — presentation exists at
all, composition and placement, transience, density budget, hierarchy, the design's named anti-patterns — has
**no applicable invariant** in this story, and inventing one would contradict a design a human has already
approved.

The **STATE** family does not vanish; it changes medium. This story's "surface" is a committed observation
record that three later stories and one evaluator read, and the same five concerns bind on it. Each is already
carried by an AC row above — none is a prose-only bullet, because a bullet here would never be extracted into
the ledger and never gated.

| RFC §6.7 state invariant | Its form here | Carried by | How it is verified |
| --- | --- | --- | --- |
| **In-place, not a context jump** | The hole opens as a narrowing of the live view the runner is already reading, between the first run and the resume — not by rebuilding a different store and presenting it as the same one | **AC-003** | `hole_opens_after_the_checkpoint_without_renumbering` |
| **Preserved selection / position** | The positions the runner already checkpointed still identify the same events after the prune; the checkpoint taken pre-prune is still a valid `from` post-prune | **AC-003**, **AC-002** | `hole_opens_after_the_checkpoint_without_renumbering`, `resume_checkpoint_is_derived_and_below_the_hole` |
| **Non-occlusion** | The observation must not obscure what it sits beside: `read_from_a_gap_position` keeps its body, its registration and its open ownership question, and the existing conformance mounts stay green and unchanged | **AC-003**, **AC-008** | control run of `--test completeness_instrument`; empty `git diff` over `crates/happenstance-testkit/src/**` |
| **Reversibility** | Nothing this story lands is load-bearing on production code; the whole deliverable is one native-only test target and one dev-dependency line, removable without touching a published surface | **AC-008** | `git diff --stat` boundary assertion; `cargo-semver-checks` at the PR grain reporting no breaking change |
| **Reachability without guessing** | Every captured value is reachable by the next reader without re-running anything: the record is committed as data in this story's folder, and each value in it is also asserted by a named test, so a rotted record fails the gate | **AC-004**, **AC-006**, **AC-007** | `observation_captures_five_values_under_both_configurations`, `read_model_across_the_hole_differs_from_the_unforgotten_model`, `suffix_and_scattered_observations_are_compared` |

The one anti-pattern this section does add teeth to is the design's own governing instinct, restated for a
record rather than a screen: **a verdict is not an observation.** "The reader failed" and "the model was wrong"
are the unstyled render of this story — they satisfy every structural check and show the reader nothing. AC-004
and AC-006 are what make that fail.

## Error conditions

| id | condition | required behaviour |
| --- | --- | --- |
| EC-001 | HS-P0011's application-facing projection runner has not landed in `crates/happenstance/` when this story starts — `lib.rs` is still the 75-line facade listing the runner under *Planned* (`:49-52`, `:75`) | **Halt.** Report the blocker by story id (HS-P0011 `projection-trait-and-runner`, `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_storymap.md:57`) as a note in this story's folder, leave the spec in place, write no code. Do **not** write a bespoke runner: it would demonstrate the hazard perfectly, prove nothing, and quietly convert a dependency failure into a finished story (`_decomposition.md:130-134`) |
| EC-002 | HS-P0010's `MemoryProjectionStore` (`projection-store-freeze` AC-012, `.bklg/from-contract-to-published-library/projection-store-freeze/project.md:218`) does not exist behind the `memory` feature | **Halt**, the same way and in the same note. A throwaway `ProjectionStore` here would freeze a fixture shape this project does not own — the same defect as EC-001, one crate down |
| EC-003 | The retained predicate leaves no forgotten position strictly between the derived checkpoint and the head, so the resume would not cross the hole | **Fail the harness loudly** with a message naming the predicate and the derived checkpoint. Do not lower the checkpoint to the first retained position and do not record the run: a vacuous observation is worse than a red test, because it is committed and cited |
| EC-004 | The runner returns a real error, or panics, instead of stalling or silently advancing | **Record it as the actual outcome and revise the finding.** A genuinely loud failure is a *better* result than the predicted negative one and must be reported as such — AC-008's finding text then states which channel carried it. It must not be forced into the predicted shape, and `#[should_panic]` must not stand in for capturing the state (`_decomposition.md:728-736`) |
| EC-005 | The slice-mate `decision-model-and-ingest-observed` has already created the shared observation target under a different file name | **Adopt its name** and add this story's module to it. One target for the slice, never two (Integration contract, *Mount point*) |
| EC-006 | HS-S0114 landed the instrument as a single file, so its types are not reachable by `#[path]` include | Perform the **bounded, behaviour-preserving** extraction into `crates/happenstance-testkit/tests/completeness_instrument/instrument.rs`, leaving the `event_store_conformance!` mounts where they are so the suite is not re-expanded in a second target. If the extraction cannot be done without changing what the control mount runs, **halt and report** rather than copying the instrument — a second copy is a bespoke instrument by another name |
| EC-007 | The runner's public API cannot be driven without a seam that does not exist in `happenstance` | **Halt and report** as a dependency finding against HS-P0011. Adding a convenience seam to `happenstance` to make the runner easier to point at the instrument is explicitly out of this PR's boundary (`discover.md`, standing constraint) |
| EC-008 | An observation would require narrowing the view by rebuilding through `MemoryEventStore::restore` | **Forbidden.** `restore` renumbers (`crates/happenstance-core/src/memory.rs:277-282`, `:386-389`) and is ES-38's registered mutant, owned by `positions-are-not-reused-after-removal`. Use a second handle or a target-local narrowing seam (AC-003) |

## Non-functional

| id | requirement | how it is held |
| --- | --- | --- |
| NF-001 | **Deterministic.** The observation produces the same captured values on every run and under any `--test-threads` setting: bounded drive iterations, no clock read, no `sleep`, no timeout, no reliance on task scheduling order | AC-005's `rg` check plus repeated local runs; a stall proved by a clock is a flake, and this target's whole value is that it is cited by an ADR |
| NF-002 | **Native-only, `wasm32` provably unaffected.** The target opens with `#![cfg(not(target_arch = "wasm32"))]` and the new dev-dependency sits inside `crates/happenstance-testkit/Cargo.toml`'s existing `[target.'cfg(not(target_arch = "wasm32"))'.dev-dependencies]` block (`:54-55`), beside `tokio` | The four mandatory `wasm32` steps in `cargo xtask ci --fast` green and unchanged (`CLAUDE.md` *Commands*) |
| NF-003 | **Cheap.** Everything is in-memory for the lifetime of one test; the target adds no measurable wall-clock to `cargo xtask affected --base main`, and the bounded iteration count is small enough to state in a comment | timing of the affected gate before and after, recorded informally in the implementation report |
| NF-004 | **No published surface moves.** A dev-dependency is invisible to downstream consumers and to `cargo-semver-checks`; no `pub` item is added, removed or changed in any of the three publishable crates | `cargo-semver-checks` at the PR grain (`CONTRIBUTING.md:291-296`) reporting no breaking change; the `git diff --stat` boundary assertion of AC-008. Its one real consequence is recorded rather than assumed: `happenstance-testkit` is publishable, so `happenstance` must reach the registry first — already HS-P0016's publish order |
| NF-005 | **Gate-clean.** `cargo fmt`, `clippy -D warnings`, and the MSRV floor: the new dev-dependency introduces no crate that raises 1.97.1 ([ADR-0029](.kb/decisions/0029-msrv-raised-to-1-97-1.md)) — `happenstance` is a workspace member and adds no third-party edge | `cargo xtask affected --base main`; CI's `msrv` job |
| NF-006 | **The record is legible to a reader who did not write it.** The committed observation states the two configurations, the five values each, the two read-model states and the comparison, in the repository's own voice, without requiring the reader to run anything | content review at the story's review gate; it is what `adr-0028-and-the-open-question-wave` and `dt-7-signal-shape-and-the-redaction-answer` consume |

## Implementation notes (non-prescriptive)

**Probe before you write anything.** The first action is EC-001/EC-002's probe, not a test file. Read
`crates/happenstance/src/lib.rs` and `crates/happenstance-core/src/projection.rs`, confirm both the
application-facing runner and an in-memory `ProjectionStore` exist, and if either does not, stop there — the
halt is a legitimate completion of this story's first acceptance criterion, not a failure to finish.

**A workable order once the probe passes.** Seed the inner `MemoryEventStore` through the instrument with the
full view and keep every `SequencePosition` `append` returned. Run the runner once to a real checkpoint.
Compute the resume checkpoint from those returned positions and the retained predicate, asserting the
forgotten-position invariant (AC-002) *before* the resume rather than after. Narrow the view. Resume, driving a
bounded number of iterations. Capture the five values. Repeat for the second configuration. Compare. Then write
the record and the finding.

**Where the code goes.** `crates/happenstance-testkit/tests/readers_against_the_hole.rs` is the slice's shared
target; this story's body is a module included by
`#[path = "readers_against_the_hole/projection_runner.rs"] mod projection_runner;`. The rationale for `#[path]`
is written out at `crates/happenstance-testkit/tests/mutation_coverage.rs:50-65` — a test target resolves
`mod foo;` against `tests/` itself, so support files without it become targets of their own. Follow
`crates/happenstance-testkit/tests/fixture_instruments.rs` for the native-only `#[tokio::test]` shape.

**The projection to use.** A small domain type the harness owns, whose read model is *defined by* the events
that will be forgotten — a running count or a set membership is enough, provided the missing events change the
answer visibly. A projection whose model happens to be insensitive to the hole would produce two identical
states in AC-006 and prove nothing; choose one where the divergence is legible in a single printed value.

**On the runner's exact API.** HS-P0011's spelling is not knowable from here. Where this spec states an
obligation rather than a signature, satisfy the obligation with whatever the runner actually exposes, and note
the mismatch in the implementation report if the runner's shape makes an obligation awkward — that note is
useful feedback to HS-P0011 and is not a licence to route around it.

**Two things to resist.** Writing `warn!` anywhere in the harness so the report reads as though something was
detected; and reaching for `#[should_panic]` because it makes the test look decisive. Both convert an
observation into a verdict, which is the exact failure `_decomposition.md:728-736` names.

## Tests and CI (merge gate)

Grounded in the project testing brief's *Merge-gate commands* (`_decomposition.md:665-700`) and its AC-006 row
(`:608`), whose tier is **observation** — the closest thing this repository has to e2e, and explicitly *not*
assertion-in-prose.

| tier | command / path | proves |
| --- | --- | --- |
| observation (this story's tier) | `cargo test -p happenstance-testkit --test readers_against_the_hole` over `crates/happenstance-testkit/tests/readers_against_the_hole/projection_runner.rs` | AC-001 – AC-007: the shipped runner is bound, the resume crosses the hole from a derived checkpoint, the hole opens without renumbering, five values are captured and asserted under both configurations, the stall is proved without a clock, and the two read-model states diverge |
| integration control (unchanged) | `cargo test -p happenstance-testkit --test completeness_instrument` | HS-S0114's `event_store_conformance!` mounts are still green and still run the same rules after any `#[path]` extraction — the non-occlusion invariant |
| integration control (unchanged) | `cargo test -p happenstance-testkit --test mutation_coverage` | `mutants_fail_exactly_their_declared_rules` and `mutant_registry_is_exhaustive` are untouched; no mutant and no `REGISTRY` row is added by this story |
| unit / compile | `cargo test -p happenstance-testkit --all-features` | the new dev-dependency resolves with `features = ["std", "memory"]` and the target compiles against `happenstance`'s public API only |
| story grain (automatic) | `cargo xtask affected --base main` (`.redkiln/config.yaml:40`) | fmt, clippy `-D warnings` and tests for `happenstance-testkit` plus dependents, and the five file-reading lints and `spec-trace` unconditionally |
| integration tripwire | `cargo xtask lints && cargo xtask spec-trace` (`.redkiln/config.yaml:48`) | no clause cites a rule that does not exist; this story adds none and moves no marker, so `spec-trace` must be green **unchanged** |
| integration grain, non-terminal | `cargo xtask ci --fast` (`.redkiln/config.yaml:55`) | the project's ceiling bar, including all four mandatory `wasm32` steps — NF-002's evidence that the edge target is untouched |
| static — boundary | `git diff --stat` against the three globs in *PR boundary* | AC-008 and NF-004: nothing under `crates/happenstance-core/src/**`, `crates/happenstance/src/**`, `crates/happenstance-testkit/src/**`, `spec/**` or `.kb/**` |
| static — surface | `cargo semver-checks check-release` at the PR grain (`CONTRIBUTING.md:291-296`) | NF-004: no breaking change to any of the three publishable crates. The `0.2.0` registry baseline is HS-P0016's and is cited, never synthesised (`_decomposition.md` testing brief, *Notes*) |
| static — record | content review of the committed observation and finding in this story's folder | AC-004, AC-006, AC-007, AC-008 and NF-006: values not verdicts, the comparison stated, the missing surface named |
| **not run as this story's proof** | `cargo xtask ci` (the whole gate, `.redkiln/config.yaml:60`) | HS-P0019 `closeout-and-durable-audience`'s bar. Useful locally; never this project's recorded proof artefact (`_decomposition.md`, *Merge-gate commands*, final bullet) |

## Risks and coupling (PR-scoped)

| Risk | Likelihood / impact | Containment |
| --- | --- | --- |
| **HS-P0011's runner has not landed** when this story starts, and the temptation is to write one | Medium / High — it would produce a green, cited, worthless observation | EC-001's halt is an acceptance criterion (AC-001), not an exception. The split from the slice-mate exists precisely so this blocker is isolated (`_storymap.md:92`); the slice-mate merges first (`:137-140`) and must not be blocked by this story |
| **The resume never actually crosses the hole** — the sharpest way to satisfy this story's letter while exercising nothing | Medium / High | AC-002's derived checkpoint plus the asserted forgotten-position invariant, and EC-003's loud failure rather than a recorded vacuous run |
| **`read_from_a_gap_position`'s ownership is settled in passing** — its narrative will look exactly like this story's subject while this story is being written | Medium / High — it silently closes the sibling half of `.kb/open-questions/es-38-and-gap-read-rules-are-unowned.md`, which HS-S0121's narrowed successor exists to keep open | The fence is stated in the Context pack, in the PR boundary and in AC-003's control check: `git diff` over `crates/happenstance-testkit/src/**` must be empty. Cite the rule's documented hazard as prior art; extend, re-home or retire nothing |
| **The observation becomes a verdict** — "the runner failed" instead of the captured values | Medium / Medium — it passes every structural check and tells ADR-0028 nothing | AC-004 and AC-006 assert on the values themselves; NF-006 is reviewed at the gate |
| **The stall is proved by a timeout** and later becomes a flaky test someone disables | Low / High — the flake would be inside the artefact an ADR cites | AC-005's bounded-iteration construction and its `rg` check for clock APIs |
| **A second copy of the instrument** is written "to keep the targets independent" | Medium / Medium | EC-006: extract and `#[path]`-include, or halt. One instrument in the tree (`project.md:244-247`, AC-013) |
| **A second observation target** is opened because the slice-mate named the file differently | Medium / Low | EC-005: adopt the slice-mate's name. The two stories are implemented in one context |
| **The dev-dependency edge is read as a publish-order surprise** | Low / Low | NF-004 records it: `happenstance` must be on the registry before `happenstance-testkit` publishes, which is already HS-P0016's order — recorded, not discovered |
| **Scope creep into the port** because the honest answer is a negative one | Medium / High — it converts an escalation into a `0.3.0` | AC-008 and initiative gate decision 4 (`.bklg/from-contract-to-published-library/_decomposition.md:239-249`): the finding is prose handed to `surface-diff-and-the-ac-012-escalation`, and no code proposal is made |

## Dependencies

**Blocks on**

- **`retained-set-instrument-and-conformance-mount`** (HS-S0114, this project, slice `forgetting-instrument`) —
  supplies the completeness instrument, its retained-set type and its two configurations (suffix and
  scattered). This story consumes it and never re-creates it. This is the story's only in-project `depends_on`.

**Blocks on, outside this project** — both are siblings in this initiative, consumed as built, never stubbed
(`_storymap.md`, *Three notes*, first bullet):

- **HS-P0011 `typed-layer-and-alpha-release` → `projection-trait-and-runner`**
  (`.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_storymap.md:57`) — the
  application-facing `Projection` trait and its runner. Absence is EC-001's halt.
- **HS-P0010 `projection-store-freeze` AC-012**
  (`.bklg/from-contract-to-published-library/projection-store-freeze/project.md:218`) — `MemoryProjectionStore`
  behind the `memory` feature. Absence is EC-002's halt.

**Runs beside**

- **`decision-model-and-ingest-observed`** (the slice-mate) — same slice `readers-against-the-hole`, same
  context, same mount target, and it merges **first** (`_storymap.md:137-140`). Nothing in this story may block
  it.

**Unlocks**

- **`adr-0028-and-the-open-question-wave`** — names this story explicitly in its `depends_on`
  (`_storymap.md`, `the-retention-decision` slice); the observation record is part of ADR-0028's evidence.
- **`dt-7-signal-shape-and-the-redaction-answer`** — consumes AC-007's suffix-versus-scattered comparison as
  DT-7's cost side stated as a measurement. (Its own `depends_on` edge is to the slice-mate; this story feeds it
  evidence, not sequence.)
- **`surface-diff-and-the-ac-012-escalation`** — consumes AC-008's finding, with the missing surface and DA-7's
  option row already named, as input to project AC-012's escalation artefact.

## Anchors (progressive disclosure)

Open these when the row says to, not before. Every path was confirmed to exist in this worktree.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `crates/happenstance-testkit/src/suite.rs` (`:1490`, `:1523-1531`) | `read_from_a_gap_position`'s assertion message is the hazard this story stages, in the specification's own voice — and the rule itself is the fence this story must not touch | Before writing the harness, to take the hazard's exact wording; and again before committing, to confirm the file is unchanged | AC-005, AC-008 |
| `crates/happenstance-core/src/projection.rs` (`:40-77`, `:88-139`) | The checkpoint pump: `checkpoint` returns a bare `Option<SequencePosition>` (`:101-110`), `commit` binds the read-model write and the checkpoint advance into one unit (`:126-132`). It is why no loudness exists to be found | Before deriving the resume checkpoint (AC-002) and before writing the finding (AC-008) | AC-002, AC-004, AC-008 |
| `crates/happenstance/src/lib.rs` (`:49-52`, `:75`) | The dependency probe's subject: today a 75-line facade whose only item is `pub use happenstance_core::*;`, listing the typed projection runner under *Planned* | **First**, before any file is created — this read decides whether the story proceeds or halts | AC-001 |
| `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/projection-runner-across-the-hole/discover.md` | The four answered questions, the halt decision, and the two named wrong runner shapes (`SilentlyResumingRunner`, `StallingRunner`) this spec is written against | Before designing the harness; the *Gate: Discover* closing paragraph explains why the checkpoint may not be a literal | AC-001, AC-002, AC-005, AC-006 |
| `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/_decomposition.md` (`:124-134`, `:150-171`, `:173-232`, `:276-309`, `:363-379`) | Composition root 5 and its halt note; DA-1 (the suffix/scattered widening); DA-2 (forget by hiding, never by rebuilding); DA-4 (add nothing to `Fixture` if the rules do not need it); DA-7's four-row option table for the finding | DA-1/DA-2 before staging the hole; DA-4 before adding any seam; DA-7 while writing the finding | AC-003, AC-004, AC-008 |
| `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/_decomposition.md` (`:608`, `:728-736`) | The testing brief's AC-006 row and its *Notes*: the tier is observation, and a `#[should_panic]`-shaped test is the named wrong instrument where the failure is a silent wrong answer | Before writing the first assertion, and again when deciding how to record a silent success | AC-004, AC-006 |
| `crates/happenstance-core/src/memory.rs` (`:277-282`, `:386-389`) | `MemoryEventStore::restore` renumbers on rebuild — the construction this story is forbidden to use, and ES-38's registered mutant owned by a sibling story | Before choosing how to narrow the retained view | AC-003 |
| `crates/happenstance-testkit/tests/mutation_coverage.rs` (`:46`, `:50-65`, module docs) | The `#[path]` module-include idiom with its rationale written out, the native-only `cfg` header, and *"Why the wrong implementations are no longer here"* — the discipline against tests that record only that something failed | When creating the shared target and its module; and before writing the record | AC-004, AC-006 |
| `crates/happenstance-testkit/tests/fixture_instruments.rs` | The in-tree precedent for a native-only `#[tokio::test]` instrument target wrapping a live `MemoryEventStore` — copy its ergonomics, not `DurableFixture::reopen`'s rebuild | While scaffolding the target | AC-003 |
| `crates/happenstance-testkit/Cargo.toml` (`:4-13`, `:29`, `:54-55`) | The crate's own semver note, how it already spells `happenstance-core`'s features, and the `cfg(not(target_arch = "wasm32"))` dev-dependency block the new line belongs in | When adding the one manifest line | AC-001, AC-008 |
| `spec/E2E-CASES.md` (`:1203-1256`) | E2E-46 (the device slice) and E2E-47 (the regulated purge) — the two journeys the two configurations are, and the shape of the silently-wrong read model | Before choosing the projection and the two retained predicates | AC-004, AC-006 |
| `spec/SPECIFICATION.md` (`:4325-4349`, `:4351-4379`, `:8034-8060`) | ES-39 (`[DEFERRED]`, the primitive question this story supplies evidence to), ES-40 (`[PROVISIONAL]`), CF-27's experiment. Read-only context: no clause text is edited here | While writing the finding, to cite ES-39 by id correctly | AC-008 |
| `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/project.md` (`:124-127`, `:215-218`, `:236-243`) | The out-of-scope fence on `read_from_a_gap_position`'s ownership; project AC-006's exact wording; AC-011 and AC-012, where this story's finding is recorded and to whom it is handed | Before the first commit, and while writing the finding | AC-003, AC-008 |
| `.kb/open-questions/es-38-and-gap-read-rules-are-unowned.md` | The atom whose *sibling* half must stay open. Reading it is how you recognise the boundary you are near while writing about the same hazard | Once, before writing any prose about gap reads | AC-008 |
| `.kb/decisions/0007-projection-runner-decodes.md` | Accepted: the projection runner decodes and lives in the typed layer — which is why the runner is HS-P0011's and not this project's to build | With the dependency probe | AC-001 |
| `.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md` (`:42`, `:249`) | Persona 1 (the application author) and Persona 4 (the evaluator) — whose goals the acceptance criteria above are framed from, and whose journeys the record must serve | If an acceptance criterion's intent is unclear while implementing | AC-004, AC-006, AC-008 |
| `RUNBOOK.md` (`:4626-4674`) | Phase 14's proof artefact and the *"observed… not asserted in prose"* instruction this story's tier comes from | Before writing the observation record | AC-004, AC-007 |

## Clarifications resolved during spec

1. **The AC set is exactly the eight the front half enumerated** — AC-001 … AC-008 — and none was added or
   dropped. Project **AC-006** is the only traced project criterion, and it is covered in both its sentences:
   the run and its named outcome (AC-001 – AC-007) and the recorded fact where loudness is impossible (AC-008).
2. **The halt is an acceptance criterion, not an escape hatch.** AC-001 is written so that halting is a *passing*
   outcome with its own evidence (a blocker note and an empty code diff), because the alternative — an
   implementer treating a missing dependency as licence to improvise — is the single most likely way this story
   produces something green and worthless.
3. **The interaction-quality COMPOSITION family is N/A by signed-off design, and the STATE family was
   translated rather than skipped.** `_design.md:38-95` records no surface and a declared `design.capture` skip,
   so no composition invariant applies. The five state invariants were mapped onto the observation record and
   the live view, and **each is carried by an existing AC row** — no invariant is a prose-only bullet in that
   section, which would never reach the ledger.
4. **The expected outcome is negative, and the spec says so up front.** `discover.md` already answered whether
   loudness is expressible for this reader with the strongest "no" in the project. AC-008 is therefore written
   to accept a negative finding as success — while EC-004 keeps the door open for a genuinely loud failure and
   requires it to be recorded as the better result rather than forced into the predicted shape.
5. **Test names are proposals; obligations are binding.** HS-P0011's runner API is not knowable from here, so
   every row states what must be true rather than a signature that cannot yet be cited. Where the runner's real
   shape makes an obligation awkward, the mismatch is reported back to HS-P0011 rather than routed around.
6. **Nothing here settles `read_from_a_gap_position`.** It is cited as prior art in AC-005 and AC-008 and is
   otherwise untouched; the empty `git diff` over `crates/happenstance-testkit/src/**` is the mechanical proof,
   and HS-S0121's narrowed successor atom is what keeps its ownership question open.
