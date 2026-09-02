---
item: HS-S0118
stage: spec
created: 2026-08-12T13:47:58.766Z
updated: 2026-08-12T13:47:58.766Z
template_sig: 87bbf1d0
rendered_sig: 913db039
---

# Spec — Two readers meet the hole: read_decision_model and IngestStore::holds

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` |
| Project | `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/project.md` |
| This spec | `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/decision-model-and-ingest-observed/spec.md` |
| Key briefs | `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/_decomposition.md` — architecture: *Composition roots* 4 (`:115-123`) and 6 (`:135-142`), DA-7 (`:363-379`), DA-8 (`:381-397`), the reader data-flow (`:413-434`); testing: the AC-006 row (`:608`), *Fixtures and seams* (`:670-685`), AC-T04 (`:704-706`), the observation note (`:728-736`) |
| Signed-off design | `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/_design.md` — **surfaces: N/A, approved 2026-08-12** (`:38-48`, `:86-95`). This project renders none; the design's binding content for this story is its framing (`:21-36`), which names these three readers as *"Rust code paths, not a screen"*. |
| Grounding | `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/_grounding.md` §4.3 (`:164-173`, the replication seam and its unresolved write path), §5 (`:187-207`, what has not landed yet) |
| Story map row | `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/_storymap.md:72` — slice `readers-against-the-hole`, row 1; backbone activity A4 (`:56`) |
| Depends on | `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/retained-set-instrument-and-conformance-mount/spec.md` (HS-S0114) — the instrument, its `RetainedSet` and its mount |
| Roadmap pointer | `RUNBOOK.md:4626-4674` (phase 14): *Work* (`:4644-4650`), **Proof artefact** (`:4658-4662`) — *"a runner and an ingest path that both fail against it"* |

## One-line PR slice

Run composition roots 4 and 6 against the completeness instrument and record their **actual** output:
`happenstance_core::read_decision_model` returning the last *retained* match, whose
`AppendCondition::after_opt` then admits an append the destroyed history would have rejected — silently, in
existing code, with no new type — and the membership question `IngestStore::holds` asks, answering `false` for
an event this store minted and acknowledged, so a peer re-sends forever (DA-8); each observation captured as
the wrong value itself rather than as a note that something failed, and any reader that cannot be made to fail
loudly without a surface the port lacks written up with the missing surface named.

## Executive summary

This PR lands **no production code**. It extends the test target the instrument story created
(`crates/happenstance-testkit/tests/completeness_instrument.rs`) with a `readers` module that drives two
*existing* production code paths over the instrument and asserts on the values they actually return, plus one
committed evidence file in this story's own folder. Nothing under any `src/` changes: no rule, no registry
line, no `Fixture` item, no signature, no rustdoc, no `todo!()` replaced.

The **pointer**: the loop being run is the DCB command loop as already written —
`read_decision_model` (`crates/happenstance-core/src/store.rs:321-331`) → `AppendCondition::after_opt`
(`crates/happenstance-core/src/append.rs:201`) → `EventStore::append` (`:213-217`). Every line of it is
production code exercised over a real, if forgetful, `EventStore` (`_decomposition.md:670-673`).

The **delta** is four decisions this spec makes rather than leaves to the bench. Every observation is a
**pair** — the identical events, query and code path run once against the control configuration and once
against a hole — because an admitted append is evidence only against a run that rejected. What is committed is
the **value**, not the verdict: the `Option<SequencePosition>` `read_decision_model` returned, the position the
admitted append was assigned, the `false` `contains_event_id` gave for an id this store minted; a
`#[should_panic]`-shaped test records only *that* something failed, which is the arrangement CF-2 rejects by
name (`spec/SPECIFICATION.md:7192-7196`). The **silence** is itself an enumerated observation — every channel
the reader could have consulted, and what each returned — because DR-6 asks whether the failure is loud, and
"no error anywhere" is the answer only if you list where you looked. And the ingest half is **reached or
recorded, never faked**: `IngestStore::holds`'s only production body is `todo!()` today
(`crates/happenstance-sync/src/ingest.rs:223-225`), so this spec sets a precondition check and two branches,
neither of which is a hand-written `IngestStore` impl.

What this PR deliberately does **not** produce is a fix. The missing-surface finding it writes is fed to
`surface-diff-and-the-ac-012-escalation` (AC-012) and to ADR-0028; making the change here would be the exact
move AC-012 exists to prevent (`project.md:240-243`).

## Context pack

Everything below is a decision this story must honour. Nothing here is a reading list; the deeper artifacts
sit behind the anchors table the second pass appends.

**The gap is not that a pruned store is wrong — it is that nothing can *ask*.** §3.7 states it in those words
(`spec/SPECIFICATION.md:4263-4272`): a store holding a scattered subset of its own log *"is indistinguishable
from a complete one at every seam an ingest path or a runner can see"*, and the contract asserts completeness
nowhere. This story is the half of the project where that sentence stops being a claim and becomes a recorded
value. The instrument (HS-S0114) built the store; this story runs the readers; ADR-0028 writes the answer.

**Decision — the readers are production code, and a reader nobody ships proves nothing.** The architecture
brief's closing note binds this story as hard as it binds the projection-runner sibling: a hand-written stand-in
would demonstrate a hazard against a reader nobody ships (`_decomposition.md:674-679`). `read_decision_model`
is real, exported, and needs nothing built (`:115-123`). `contains_event_id`'s body is real
(`crates/happenstance-core/src/memory.rs:414-419`). `IngestStore::holds`'s is not, and that asymmetry is
handled below rather than papered over.

**Decision — every observation is a differential against the control.** The instrument story committed
`RetainedSet::All` as the *control* mount precisely so a later difference is attributable to forgetting rather
than to the decorator (`retained-set-instrument-and-conformance-mount/spec.md:136-145`). This story inherits
that discipline at the reader grain: the same fixture type, the same seeded events, the same `Query`, the same
call sequence, differing **only** in the retained set. A recorded "the append was admitted" with no run that
rejected is not evidence of forgetting — it is evidence that a condition matched nothing, which is also what
happens on a young store, which is §3.7's entire point.

**Decision — record the wrong value, not the fact that something went wrong.** CF-2's `Rejects` is the
governing discipline (`spec/SPECIFICATION.md:7192-7196`): a `should_panic` harness *"records only **that**
something failed, so a mutant which fails the right rule for the wrong reason … reads as proof"*. The testing
brief says the same one level down — AC-T04 requires the reader observations *"committed as recorded data …
never summarised as 'it worked' in prose"* (`_decomposition.md:704-706`) — and its note warns against reaching
for `#[should_panic]` here at all, because the likely failure mode is a **silent wrong answer**, not a panic
(`:728-736`). So each observation asserts on the concrete value and the same value is written into a committed
evidence file. `RUNBOOK.md:4658-4662` is the bar: the proof artefact is the readers' behaviour, not a
description of it.

**Decision — the read half's wrong answer has an exact shape, and that shape is what gets asserted.**
`read_decision_model` collects the matching events and returns `events.last().map(|e| e.position)`
(`crates/happenstance-core/src/store.rs:321-331`). Over a store that has forgotten matching history, `last` is
the last **retained** match, and that value is what `AppendCondition::after_opt` expects
(`crates/happenstance-core/src/append.rs:201`). The follow-up conditional append is then evaluated by
`Guard::is_violated_by`, whose two-arm `match` has no third arm (`:239-253`) — so a condition over destroyed
history passes **vacuously** (ES-40, `spec/SPECIFICATION.md:4351-4379`) and the append is **admitted** where
the destroyed history would have rejected it. The instrument makes this reachable because it evaluates the
condition over its retained view rather than delegating it to the inner store
(`retained-set-instrument-and-conformance-mount/spec.md:113-124`). Two values are therefore the observation:
the `Option<SequencePosition>` handed to `after_opt`, and the `Ok(position)` the admitted append returned.

**Decision — the vacuous pass is specified behaviour being *observed*, not a bug being reported.** ES-40 is
`[PROVISIONAL]` with this project's instrument as its named falsifier, and DA-5 settles that the rule asserting
it belongs to `condition-over-removed-history-does-not-reject`, with the discharging sentence landing as
rustdoc (`_decomposition.md:310-326`). This story writes **no rule and no rustdoc**. It records what a reader
experiences when the specified behaviour meets a hole — which is the input ADR-0028 needs and the sibling
story's rule cannot supply, because a green rule asserting a vacuous pass says nothing about what the caller
downstream of it then does.

**Decision — the ingest half is DA-8's, and DA-8 locates the lie at `contains_event_id`.**
`EventStore::contains_event_id` is *"the membership question a replicating peer must be able to ask"*, and its
own rustdoc explains why every store answers cheaply: *"if the identifier's store half is not its own
incarnation the answer is `false`, and if it is, the question reduces to whether that position exists"*
(`crates/happenstance-core/src/store.rs:250-269`). Under forgetting that reduction is **exactly wrong**: the
position existed, this store minted it and acknowledged it, and `false` now means *"never had it"* to every
caller. A peer that re-sends on `false` re-sends forever. `IngestStore::holds`
(`crates/happenstance-sync/src/ingest.rs:164`) is the same question one crate up and inherits the same lie
(`_decomposition.md:381-397`). This is ES-39's question visible on a surface that already exists — no new type,
an observed wrong answer rather than an asserted one.

**Decision — `IngestStore::holds` is reached or recorded, and a hand-written impl is neither.** The planning-time
fact is on the record: `impl SendIngestStore for MemoryEventStore`'s `holds` is
`todo!("contains_event_id would answer this, once the placeholder EventId is unified")`
(`crates/happenstance-sync/src/ingest.rs:223-225`), the write path is HS-P0017's
(`_grounding.md:164-173`, `:187-207`), and `happenstance-sync` carries its own placeholder `EventId`
(`crates/happenstance-sync/src/identity.rs:26-39`, `:98`) that core's is not yet unified with. Three
consequences, each a decision:

1. **Calling `holds` today observes a `todo!()` panic**, which is a fact about an unfinished write path and
   says nothing about forgetting. It is not the observation AC-006 asks for.
2. **Writing an `IngestStore` impl for the instrument would make the body ours**, and observing our own body
   proves nothing — the same reason the projection-runner sibling may not write a bespoke runner. The testing
   brief forbids it in terms: *"never a fake ingest built to produce an observation"* (`_decomposition.md:684-685`).
3. **So the story checks the precondition first and branches.** If HS-P0017 has landed a real `holds` body and
   a unified `EventId`, the observation is direct, in a new target under `crates/happenstance-sync/tests/`. If
   it has not, the ingest half is observed at `contains_event_id` — the surface DA-8 actually names, with a
   real body, through the instrument — and `holds`'s inheritance of that answer is recorded from the source by
   `file:line`, as a **finding naming HS-P0017 and the missing write path**, not as a gap in this story.

**Decision — the testkit does not grow a dependency on `happenstance-sync`.** It is the obvious shortcut and it
is closed: `happenstance-sync` is `publish = false` (`crates/happenstance-sync/Cargo.toml:12`),
`happenstance-testkit` is one of the three publishable crates, and the gate asserts `cargo package --list` over
each of them (`CLAUDE.md`, *Commands*). A dev-dependency from a publishable crate onto an unpublishable one is
a publish hazard this project is explicitly not allowed to create (AC-011). That constraint is *why* the two
halves of this story may end up in two targets, and stating it here is what stops it being rediscovered as a
compile error.

**Decision — silence is enumerated, not asserted.** DR-6 asks whether a reader *"fails loudly"*
(`project.md:160-164`). Recording "it did not" is a claim; recording *where you looked* is evidence. So the
observation includes the channel list and each channel's value under the hole: the `Result` is `Ok`;
`read_decision_model`'s return type is `(Vec<SequencedEvent>, Option<SequencePosition>)` and carries no
incompleteness channel; `head()` returns the highest retained position, which a young store also returns;
`contains_event_id` returns `false`, which "never had it" also returns; and `AppendError`
(`crates/happenstance-core/src/error.rs:214-225`) has `ConditionViolated`, `NoEvents`, a capacity variant and a
store variant — and no variant meaning *"I cannot judge this condition"*. That enumeration is the evidence base
for the missing-surface finding and feeds DA-7's option table directly.

**Decision — the finding stops at a finding.** DA-7 pre-computed the four options and their version
consequence (`_decomposition.md:363-379`): a floor `earliest_position()` (which ES-39 argues against by name,
`spec/SPECIFICATION.md:4336-4341`), a set of retained ranges, a third outcome on condition evaluation, and —
the row this story's observations bear on most directly — a **tri-state `contains_event_id`**. All four are a
`0.3.0` this initiative's exit criteria do not contemplate. This story names which row each observation points
at and **makes no change**; the escalation artefact is `surface-diff-and-the-ac-012-escalation`'s
(`_storymap.md:98`).

**The persona-journey slice.** Two people meet this PR. The **ingest author** learns, from a committed value
rather than a warning, that a condition they evaluate against a peer's pruned slice is admitted rather than
refused — which under the unconditional-ingest decision is the *normal* path and not an edge case
(`spec/SPECIFICATION.md:4373-4379`). The **repository owner**, writing ADR-0028 two slices later, gets the
second half of its evidence base: the pass list said what the *suite* cannot see, and this story says what a
*reader* does about it. Neither is served by prose that predicts the outcome; both are served by a table whose
cells are the values the code returned.

## Integration contract

- **Archetype**: `capability` — a user-observable slice through the layers this project has: a real store
  (the instrument), real readers (`happenstance-core`, and `happenstance-sync` under the reached branch), and
  a committed observation an ADR consumes. No double, no flag, no `todo!()` written or replaced.
- **Slice / milestone**: `readers-against-the-hole`. Slice-mate: `projection-runner-across-the-hole`
  (`.bklg/from-contract-to-published-library/retention-and-incomplete-logs/projection-runner-across-the-hole/`),
  which runs composition root 5. The two are implemented in one context and mounted as one integrated
  surface — the same instrument, the same recording discipline, the same "reached or recorded, never faked"
  rule for a reader that has not landed. They are split only so the runner's HS-P0011 dependency is an
  isolated blocker rather than a risk to these two observations (`_storymap.md:92`). This story merges first
  (`_storymap.md:137-140`). It has no edge to the `owed-rules-and-mutants` slice and may be interleaved with
  it.
- **Mount point**: **`crates/happenstance-testkit/tests/completeness_instrument.rs`** — the native-only
  integration-test target HS-S0114 creates, extended here with a `readers` module (inline, or as
  `crates/happenstance-testkit/tests/completeness_instrument/readers.rs`, the shape
  `crates/happenstance-testkit/tests/mutation_coverage.rs` + `mutation_coverage/` already uses; a
  subdirectory without `main.rs` is not itself a target, so no manifest edit). This is the **only** place
  root 4 can run against *the* instrument: integration-test targets are separate compilation units and are not
  importable by another crate, so a reader that runs anywhere else runs against a copy — and a copy of the
  instrument is a second instrument nobody validated. Read the target as HS-S0114 landed it before writing
  anything; its type names (`RetainedSet`, the forgetting fixture and handle) are that story's to fix, and this
  story uses whatever it committed.
  - **Second mount, conditional**: `crates/happenstance-sync/tests/` — a new target, entered **only** under
    the precondition in the Context pack (a real `holds` body and a unified `EventId`). `happenstance-sync`
    already carries four integration targets (`crates/happenstance-sync/tests/`), including
    `ingest_reaches_a_foreign_store.rs`, whose module docs are the model for writing down what compiling
    something did and did not prove.
- **Wires into**:
  - The instrument from HS-S0114 — its retained-set value, its `Fixture` and its handle
    (`crates/happenstance-testkit/tests/completeness_instrument.rs`); consumed as built, never re-shaped. If
    it is absent when this story starts, that is a dependency failure to halt on and report.
  - `happenstance_core::read_decision_model` (`crates/happenstance-core/src/store.rs:321-331`) and
    `happenstance_core::collect` (`:285-307`) — the read half, run directly, nothing mocked.
  - `happenstance_core::AppendCondition` — `after_opt` (`crates/happenstance-core/src/append.rs:201`),
    `is_violated_by` (`:225-234`), `Guard::is_violated_by` (`:239-253`).
  - `happenstance_core::EventStore` — `append` (`crates/happenstance-core/src/store.rs:213-217`), `head`
    (`:248`), `contains_event_id` (`:250-269`). **Bind `EventStore`, not `SendEventStore`** (`CLAUDE.md`
    constraint 4), and import only one of the two names in the module.
  - `happenstance_core::{AppendError, ConditionViolated}` (`crates/happenstance-core/src/error.rs:135-165`,
    `:214-225`) — the error vocabulary the silence enumeration is taken against.
  - `happenstance_core::MemoryEventStore` (`crates/happenstance-core/src/memory.rs`) — the inner store behind
    the instrument; `snapshot()` (`:186`) is how an `EventId` is resolved to a position for the membership
    observation, and `restore` (`:163`) is **never** called (it is ES-38's mutant).
  - `happenstance_sync::IngestStore::holds` (`crates/happenstance-sync/src/ingest.rs:164`) and its only
    production impl (`:206-229`, the `memory`-gated coherence proof) — read for the recorded branch, called
    for the reached branch.
- **Renders surfaces**: **none.**
  `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/_design.md:38-80` records
  `## Surfaces`, `## Items` and `## Signatures` as *"N/A — no user-facing surface"*, approved by the
  repository owner on 2026-08-12 with `design.capture` a declared skip (`:86-95`). There is no surface id to
  claim and no `## Signatures` block to match. The design's binding content for this story is its framing
  (`:21-36`): the readers here are *"`read_decision_model`, a projection runner, and `IngestStore::holds` —
  Rust code paths, not a screen"*, and DT-7 is resolved in prose by
  `dt-7-signal-shape-and-the-redaction-answer`, which this story blocks and feeds.
- **Conformance rule(s)**: **none added, and none may be.** This story is not adapter-observable and that is
  the finding rather than an omission: what it observes is precisely the behaviour every conformant store is
  permitted to have, so a rule asserting it would either restate ES-40's vacuous pass (which is
  `condition-over-removed-history-does-not-reject`'s, per `_storymap.md:71`) or assert a distinguishability
  the port has no surface for — which is CF-27's own
  `suffix_store_is_distinguishable_from_a_young_store`, settled either way by `cf-27-rule-or-recorded-refusal`
  after ADR-0028, never before it (`_storymap.md:76`). Writing a rule here would be a rule written before the
  evidence it is supposed to follow, which is the sequencing note the whole project is arranged around
  (`_storymap.md:18-25`). The existing rules the instrument already runs green
  (`contains_event_id_reports_membership`, the `condition_*` family, `crates/happenstance-testkit/src/registry.rs`)
  are the guard that these observations are about forgetting and not about a decorator bug.
- **Clause(s)**: discharges none and amends none. It **exercises** ES-40's hazard
  (`spec/SPECIFICATION.md:4351-4379`, `[PROVISIONAL]`, this project's instrument its named falsifier) from the
  caller's side, and supplies evidence for ES-39 (`:4325-4349`, `[DEFERRED]`) and CF-27 (`:8034-8060`,
  `[DEFERRED]`). **No `spec/SPECIFICATION.md` edit is in this PR** — every marker move is
  `marker-moves-and-spec-trace-green`'s (`_storymap.md:77`), and a marker moved before its rule exists fails
  `spec-trace` (`xtask/src/spec_trace.rs`).
- **Advances DoD scenario**: initiative DoD **15** — *"Incomplete logs have an answer on disk … a store that
  holds only a suffix of its own log is exercised against a reader, and the reader either fails loudly or the
  refusal to define this is recorded as a decision"* (`.bklg/from-contract-to-published-library/initiative.md:402-404`).
  HS-S0114 landed the store half; this story is the *exercised against a reader* half for two of the three
  readers, and it establishes on the record which of the scenario's two branches the project is on. It is also
  the direct input to phase 14's proof artefact (`RUNBOOK.md:4658-4662`).

## PR boundary

```
crates/happenstance-testkit/tests/**
crates/happenstance-sync/tests/**
.bklg/from-contract-to-published-library/retention-and-incomplete-logs/decision-model-and-ingest-observed/**
```

The second glob is honest rather than speculative: it is entered only on the reached branch of the ingest
precondition, and it is written here so that taking that branch is not a boundary widening decided under
gate pressure. On the recorded branch it stays untouched, and `git diff --stat` shows it.

**In this PR**

- A `readers` module in `crates/happenstance-testkit/tests/completeness_instrument.rs` (inline or as a
  `completeness_instrument/readers.rs` submodule) holding the paired control/hole observations of
  `read_decision_model` → `after_opt` → `append`, and of `contains_event_id` under forgetting.
- The precondition check on `IngestStore::holds`, and exactly one of its two branches: a new
  `crates/happenstance-sync/tests/` target that calls the production `holds`, **or** the recorded finding
  citing `crates/happenstance-sync/src/ingest.rs:223-225` and naming HS-P0017.
- `_observations.md` in this story's own folder — the committed evidence file: one row per observation
  carrying reader, configuration, call, control value and hole value, plus the enumerated silence channels and
  the missing-surface finding mapped onto DA-7's rows.
- Module rustdoc on the `readers` module stating what these observations are and are not: production readers
  over a real store, evidence for ADR-0028, and **not** an assertion that any store is misbehaving.

**Explicitly not in this PR**

- **No `src/` change in any crate.** No rule body, no `for_each_event_store_rule!` line, no `Fixture` item, no
  signature, no rustdoc sentence. ES-40's documentation sentence is
  `condition-over-removed-history-does-not-reject`'s (`_storymap.md:71`).
- **No `todo!()` replaced and no `IngestStore` impl written.** The write path is HS-P0017's
  (`_grounding.md:164-173`); a hand-written impl would make the observed body ours.
- **No surface change of any kind**, and no `cargo-semver-checks`-visible delta. The finding names the surface;
  it does not make it (`project.md:240-243`).
- **No change to the instrument.** If an observation needs a retained set the instrument cannot express, that
  is a defect report to HS-S0114's shape, decided with it — never a second decorator here.
- **No `spec/SPECIFICATION.md` edit**, no marker move, no `(new)` removal.
- **No ADR, no `.kb/` file, no `.kb/_intake/` staging.** ADR-0028 is `adr-0028-and-the-open-question-wave`'s,
  and it consumes this file rather than being drafted in it.
- **No projection-runner observation.** Composition root 5 is the slice-mate's, including its halt-on-missing-
  dependency rule (`_storymap.md:73`).
- **No `Cargo.toml` edit**, in particular no `happenstance-sync` dev-dependency on `happenstance-testkit` or
  the reverse.

**Merge DoD (one line)** — `cargo xtask affected --base main` (`.redkiln/config.yaml:40`) green with the
readers running inside the instrument's own target, `_observations.md` carrying a value in every cell rather
than a description, and `git diff --stat` touching nothing outside the three globs above.

## Behavior and interfaces

Read the instrument as HS-S0114 committed it before writing anything; the names below in *italics* are that
story's to fix and are referred to by role here.

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **The read loop is production code, run twice, differing only in the retained set** | Seed the same events through the same fixture, build the same `Query`, then run `read_decision_model(&store, &query)` → `AppendCondition::new(query).after_opt(last)` → `store.append(&events, Some(condition))` once with the control retained set (hides nothing) and once with a hole over matching history. Nothing in the loop is re-implemented locally: a locally inlined "equivalent" of `read_decision_model` would observe this story's code rather than the library's. | `crates/happenstance-core/src/store.rs:321-331`; `crates/happenstance-core/src/append.rs:201`, `:213-217`; `_decomposition.md:115-123`, `:670-673` |
| **The recorded outcome is the value, never the verdict** | Each observation asserts on and records: the `Option<SequencePosition>` `read_decision_model` returned, the length of the `Vec<SequencedEvent>` it returned, and the `Result` of the follow-up append — `Ok(position)` under the hole against `Err(AppendError::ConditionViolated(_))` under the control. A `#[should_panic]` or an "it failed" boolean is forbidden: CF-2's `Rejects` names that arrangement, and the testing brief's note repeats it because the likely failure mode here is a *silent wrong answer* rather than a panic. | `spec/SPECIFICATION.md:7192-7196`; `_decomposition.md:704-706`, `:728-736`; `RUNBOOK.md:4658-4662` |
| **The hole is over *matching* history, and the control proves it** | The forgotten events must be ones the `Query` matches and whose positions are above nothing the condition already excludes — otherwise the append is admitted because nothing matched, which a young store also does, and the observation collapses into §3.7's own point instead of demonstrating it. The control run rejecting is what establishes the hole is load-bearing. | `spec/SPECIFICATION.md:4263-4272`; `retained-set-instrument-and-conformance-mount/spec.md:136-145` |
| **The vacuous pass is observed, not judged** | `Guard::is_violated_by`'s two-arm `match` has no third arm, so a condition over destroyed history passes vacuously — ES-40's *specified* behaviour. The module rustdoc and `_observations.md` both say so in terms: this is the contract working as written, and the finding is about what the *caller* can then conclude, not about a store misbehaving. | `crates/happenstance-core/src/append.rs:239-253`; `spec/SPECIFICATION.md:4351-4379`; `_decomposition.md:310-326` |
| **The membership observation asks about an id the store minted and acknowledged** | Append an event, keep the `EventId` the store returned for it, open a hole over its position, then call `contains_event_id` with that same id. The recorded value is `false` under the hole and `true` under the control. Resolving the id must go through the inner store's `snapshot()` inside the instrument, never through the id's own position half — that half belongs to the *origin* store for an ingested event, and that reduction is the one DA-8 shows to be wrong. | `crates/happenstance-core/src/store.rs:250-269`; `crates/happenstance-core/src/memory.rs:186`, `:414-419`; `_decomposition.md:381-397` |
| **`IngestStore::holds` — precondition, then one of two branches, never a third** | Check first: does `impl SendIngestStore for MemoryEventStore` have a real `holds` body, and is `happenstance_sync::EventId` unified with core's? **Reached branch**: a new `crates/happenstance-sync/tests/` target calls the production `holds` over a forgetting store and records `false` for a minted-and-acknowledged id. **Recorded branch**: the finding cites `crates/happenstance-sync/src/ingest.rs:223-225` verbatim — the `todo!()` names `contains_event_id` as its future answer, which *is* the inheritance DA-8 asserts — names HS-P0017 as the owner of the write path, and states what remains unobserved. **Forbidden in both**: a hand-written `IngestStore` impl, a stubbed body, or a `todo!()` replaced to make a test runnable. | `crates/happenstance-sync/src/ingest.rs:120-164`, `:206-229`; `crates/happenstance-sync/src/identity.rs:26-39`, `:98`; `_grounding.md:164-173`, `:187-207`; `_decomposition.md:680-685` |
| **The silence is enumerated channel by channel** | For each reader, list every channel that could have carried an incompleteness signal and the value it carried under the hole: the `Result` (`Ok`), the return tuple (no incompleteness field), `head()` (highest retained — same shape a young store returns), `contains_event_id` (`false` — same value "never had it" returns), and `AppendError`'s variants (none means *"I cannot judge this"*). "Fails loudly: no" is a claim; this table is the evidence for it. | `crates/happenstance-core/src/store.rs:248`, `:250-269`, `:321-331`; `crates/happenstance-core/src/error.rs:214-225`; `project.md:160-164` |
| **Each observation names the DA-7 row it bears on, and nothing is changed** | The membership observation points at DA-7's fourth row (a tri-state `contains_event_id`, smallest signature delta); the admitted-append observation points at rows 2 and 3 (retained ranges; a third outcome on condition evaluation). The finding records the surface, the version consequence (`0.3.0`, outside this initiative's exit criteria) and the option — and stops. The escalation artefact belongs to `surface-diff-and-the-ac-012-escalation`. | `_decomposition.md:363-379`; `spec/SPECIFICATION.md:4336-4341`; `project.md:240-243`; `_storymap.md:98` |
| **Mounted where the instrument is; no crate gains a dependency** | The readers live in the instrument's own test target because an integration-test target is a separate compilation unit and cannot be imported. `happenstance-testkit` must not take a dev-dependency on `happenstance-sync` (`publish = false`, and testkit is publishable and `cargo package`-asserted by the gate), which is why the ingest half's reached branch lives in `happenstance-sync`'s own tests instead. | `crates/happenstance-sync/Cargo.toml:12`; `crates/happenstance-testkit/Cargo.toml`; `CLAUDE.md` (*Commands*, the `cargo package --list` assertion) |
| **Port-generic code binds `EventStore`, and no `#[async_trait]` appears** | Helper functions in the readers module are generic over `S: EventStore` — the weaker bound, which accepts both flavours — and only one of `EventStore` / `SendEventStore` is imported per module, because both in scope makes the method calls ambiguous. `read_decision_model` is itself `S: EventStore`, so the readers inherit this for free and can only break it by adding a bound they do not need. | `CLAUDE.md` constraints 1 and 4; [ADR-0001](.kb/decisions/0001-async-port-flavours.md); `crates/happenstance-core/src/store.rs:321-331` |
| **The evidence file is data, and it is this story's, not the project's** | `_observations.md` lives in this story's folder, carries one row per observation with control and hole values in separate columns, and is cited by ADR-0028 rather than summarised into it. The slice-mate records its runner observation in its own folder the same way; there is no shared file two stories both write. | `_decomposition.md:704-706`; `_storymap.md:56`, `:72-73`; `RUNBOOK.md:4658-4662` |

## Data and migrations

**N/A for schema and persistence.** This story defines no stored format, touches no wire type, adds no `serde`
derive and no feature. The store under observation is the instrument's inner `MemoryEventStore` — a
`Vec<SequencedEvent>` behind a lock that lives for the duration of one test
(`crates/happenstance-core/src/memory.rs`) — and nothing written here is read back by a later process.

One committed artifact is worth pinning down, because "recorded data" is an acceptance criterion rather than a
convenience: `_observations.md` in this story's folder. Its shape is fixed here so the second pass can gate it
and so ADR-0028 can cite cells rather than paragraphs — one row per observation carrying **reader**,
**configuration** (the retained-set value, printed via its `Debug`, which HS-S0114 made binding for exactly
this reason), **call**, **control value** and **hole value**; then the enumerated silence channels; then the
missing-surface finding with its DA-7 row and version consequence. Every cell holds a value the code returned.
A cell holding a description of a value is the failure mode AC-T04 and `RUNBOOK.md:4658-4662` are written
against.

## Acceptance criteria

Every criterion is framed from a persona's goal crossing the whole stack this project has — a real store, a
real reader, and a recorded value an ADR consumes. The personas are the initiative's own
(`.bklg/from-contract-to-published-library/initiative.md:203-226`); the **application author**'s stated fear
is *"discovering a contract defect in production — particularly through a second, independent reader building
a wrong answer from a torn or gapped log"* (`:203-208`), which is this story's subject verbatim.

Names in *italics* are HS-S0114's to fix; the test names below are this story's, and are the names the ledger
carries.

| id | criterion | verification |
| --- | --- | --- |
| **AC-001** | **GIVEN** the application author, whose stated fear is a second independent reader building a wrong answer from a gapped log (`initiative.md:203-208`), **WHEN** they look for whether that fear is real in *this* library rather than in general, **THEN** the DCB command loop exactly as shipped — `read_decision_model` → `AppendCondition::after_opt` → `EventStore::append` — has been run **twice** inside `crates/happenstance-testkit/tests/completeness_instrument.rs` over the same fixture type, the same seeded events, the same `Query` and the same call sequence, differing **only** in the retained set; the forgotten events are ones that `Query` **matches**; the control run **rejects** and the hole run **admits**; and no locally inlined re-implementation of `read_decision_model` or of the guard exists anywhere in the target, so what was observed is the library's code and not this story's. | `cargo test -p happenstance-testkit --test completeness_instrument readers::` — the paired target-local test `decision_model_loop_admits_under_a_hole_and_rejects_under_the_control`, which fails if either arm's outcome inverts or if the two arms differ in anything but the retained set. Boundary check: a search of `crates/happenstance-testkit/tests/` finds no local definition of `read_decision_model` or `is_violated_by`, only calls. |
| **AC-002** | **GIVEN** the evaluator deciding in one sitting from public evidence (`initiative.md:224-226`), who cannot act on "a test failed", **WHEN** they open this story's record, **THEN** every read-half observation is the **value itself**: the `Option<SequencePosition>` `read_decision_model` returned under each configuration (the hole's being the last *retained* match, and provably different from the control's), the length of the `Vec<SequencedEvent>` it returned under each, and the append `Result` — `Ok(position)` under the hole with that position recorded, `Err(AppendError::ConditionViolated(_))` under the control with the violating position recorded — **AND** no `#[should_panic]`, no "it failed" boolean, and no assertion whose failure message would not name the observed value. | The same paired test, asserting on the four concrete values (two `Option<SequencePosition>`, two `Result` discriminants and their carried positions) with messages that print them. Negative check: no `should_panic` attribute anywhere in `crates/happenstance-testkit/tests/completeness_instrument.rs`. Discipline anchors: `spec/SPECIFICATION.md:7192-7196` (CF-2's `Rejects`) and `crates/happenstance-testkit/tests/mutation_coverage.rs`'s module doc. |
| **AC-003** | **GIVEN** the adapter author, who needs an executable definition of "correct" rather than prose to interpret (`initiative.md:210-215`), and who must not read this observation as "a store is misbehaving", **WHEN** they open the `readers` module or `_observations.md`, **THEN** both state in terms that the admitted append is ES-40's **specified** behaviour — `Guard::is_violated_by` has two arms and no third, so a condition over destroyed history passes vacuously — that the finding concerns what the *caller* may then conclude, and that the rule and rustdoc which discharge ES-40 belong to `condition-over-removed-history-does-not-reject`; **AND** the diff shows **no file under any crate's `src/`** changed by this PR: no rule body, no `for_each_event_store_rule!` line, no `Fixture` item, no signature, no rustdoc sentence. | Content review of the `readers` module rustdoc and `_observations.md` against those three sentences, at the story review gate. Mechanical half: `git diff --stat main...HEAD -- crates` shows nothing under any `src/`, and the whole diff is confined to the three PR-boundary globs. Clause anchor `spec/SPECIFICATION.md:4351-4379`; code anchor `crates/happenstance-core/src/append.rs:239-253`. |
| **AC-004** | **GIVEN** the local-first / edge developer replicating between two stores (`initiative.md:217-222`), whose peer must be able to ask *"do you already have this event?"*, **WHEN** an event **this store minted and acknowledged** falls inside the hole, **THEN** `contains_event_id` returns `false` for that exact `EventId` under the hole and `true` for the same id under the control, both recorded as values; the id is resolved through the inner store's `snapshot()` and **never** through `EventId::position()` — that half belongs to the *origin* store for an ingested event, and that reduction is the one DA-8 shows to be wrong; and `_observations.md` states that this `false` is byte-for-byte the answer *"never had it"* returns, so a peer that re-sends on `false` re-sends forever. | `cargo test -p happenstance-testkit --test completeness_instrument readers::` — `contains_event_id_answers_false_for_an_id_this_store_minted`, paired control/hole, recording both answers and the id. The `snapshot()` route is checked by the absence of any `EventId::position()` call in the readers module. Anchors: `crates/happenstance-core/src/store.rs:250-269`; `crates/happenstance-core/src/memory.rs:186`, `:414-419`. |
| **AC-005** | **GIVEN** the same developer, for whom `IngestStore::holds` is the ingest-side spelling of that same question, **WHEN** this story runs, **THEN** the precondition is checked **first** and recorded — does `impl SendIngestStore for MemoryEventStore` have a real `holds` body, and is `happenstance_sync::EventId` unified with core's — and **exactly one** branch is taken: **reached**, a new `crates/happenstance-sync/tests/` target calling the **production** `holds` over a forgetting store and recording its `false`; or **recorded**, a finding citing `crates/happenstance-sync/src/ingest.rs:223-225` verbatim (whose `todo!()` names `contains_event_id` as its future answer, which *is* DA-8's inheritance), naming HS-P0017 as the owner of the write path, and stating what remains unobserved. **In neither branch** does a hand-written `IngestStore` impl, a stubbed body, or a replaced `todo!()` appear. | Branch-dependent, and the branch itself is evidence. **Reached** → `cargo test -p happenstance-sync --test ingest_holds_across_a_hole`. **Recorded** → content review of `_observations.md`'s precondition block against `crates/happenstance-sync/src/ingest.rs:223-225` as it stands at merge. Both branches: no `impl … IngestStore` appears under `crates/happenstance-testkit/tests/` or `crates/happenstance-sync/tests/`, and `git diff main...HEAD -- crates/happenstance-sync/src` is empty. |
| **AC-006** | **GIVEN** the repository owner asking DR-6's question — *does a reader fail loudly?* (`project.md:160-164`) — who cannot accept "no" as a claim, **WHEN** they read this story's record, **THEN** the silence is **enumerated**: for each reader, every channel that could have carried an incompleteness signal is listed with the value it carried under the hole — the `Result` (`Ok`), `read_decision_model`'s return tuple (no incompleteness field), `head()` (the highest **retained** position, the same shape a young store returns), `contains_event_id` (`false`, the same value *"never had it"* returns), and `AppendError`'s variants (`ConditionViolated`, `NoEvents`, the capacity variant, the store variant — **none** meaning *"I cannot judge this condition"*) — so "fails loudly: no" is supported by where the author looked rather than asserted. | Content review of `_observations.md`'s silence table against `crates/happenstance-core/src/error.rs:214-225` and `crates/happenstance-core/src/store.rs:248`, `:250-269`, `:321-331` — a channel present in the source and absent from the table fails the row. Mechanically fed by the AC-001, AC-002 and AC-004 tests, whose recorded `Ok`, `false` and `head()` values are the cells. |
| **AC-007** | **GIVEN** the repository owner, who has already pre-computed DA-7's four options and their version consequence and must not have one of them quietly implemented, **WHEN** this PR is read, **THEN** each observation **names the DA-7 row it bears on** — the membership observation at row 4 (a tri-state `contains_event_id`, smallest signature delta), the admitted-append observation at rows 2 and 3 (retained ranges; a third outcome on condition evaluation) — records the version consequence (`0.3.0`, outside this initiative's exit criteria) and **stops**: no public signature in `crates/happenstance-core/src/store.rs` or `crates/happenstance-core/src/append.rs` is touched, no `spec/SPECIFICATION.md` edit or marker move is in the diff, and the escalation artefact itself is left to `surface-diff-and-the-ac-012-escalation`. | Content review of `_observations.md`'s finding block against `_decomposition.md:363-379` — a finding naming a surface but no DA-7 row fails, as does one naming an option without its version consequence. Mechanical half: `git diff main...HEAD -- crates/happenstance-core spec` empty, and `cargo xtask spec-trace` green and unchanged under `cargo xtask affected --base main`. |
| **AC-008** | **GIVEN** the repository owner writing ADR-0028 two slices later, who needs to cite **cells** rather than paraphrase paragraphs (`RUNBOOK.md:4658-4662`), **WHEN** they open this story's folder, **THEN** `_observations.md` exists there — not in a shared project file two stories both write — carries **one row per observation** with **reader**, **configuration** (the retained-set value printed via its `Debug`, which names variant *and* bound), **call**, **control value** and **hole value** in separate columns, followed by the silence enumeration and the missing-surface finding — and **every cell holds a value the code returned**, never a description of one. | Content review at the story gate: each cell cross-checked against the assertion that produced it in `crates/happenstance-testkit/tests/completeness_instrument.rs`; a cell whose text does not appear as a literal or a printed value in the test output fails the row. Bar: `_decomposition.md:704-706` (AC-T04) and `RUNBOOK.md:4658-4662`. |

**Coverage of the traced project AC.** All eight rows serve project **AC-006** — *"A reader fails loudly,
observed"* (`project.md:157-163`) — which this story owns for composition roots **4** and **6**, while
`projection-runner-across-the-hole` owns root **5** (`_storymap.md`, the AC-006 *Coverage* row). AC-006's
second sentence — *"Where either cannot be made to fail loudly without a surface the port does not have, that
fact is recorded against AC-011 rather than papered over"* — is discharged by **AC-006** (the enumeration that
establishes it cannot) and **AC-007** (the recording, stopping at a finding).

## Interaction quality

**This story renders no surface, and that is a signed-off determination rather than an omission.** The
project's `_design.md` records `## Surfaces`, `## Items`, `## Signatures`, `## The states the API must
express` and `## Anti-patterns` each as *"N/A — no user-facing surface"*, approved by the repository owner on
2026-08-12 with `design.capture` a **declared** skip
(`.bklg/from-contract-to-published-library/retention-and-incomplete-logs/_design.md:38-95`). Its binding
content for this story is the framing (`:21-36`): the readers here are *"`read_decision_model`, a projection
runner, and `IngestStore::holds` — Rust code paths, not a screen a human looks at… No route, no DOM selector,
no component exists to enumerate."*

The two families therefore resolve as follows, and **no AC row is invented for an invariant that has no
subject**.

**STATE invariants — N/A with reason.** There is no in-place-versus-context-jump, no occlusion, no
focus/scroll/selection to preserve, no reversibility affordance and no keyboard path, because there is no
rendered state and no interactive control. The nearest analogue this story does owe is **non-destructiveness
of the observation itself**: the run must not mutate the instrument, the rule registry or any `src/` file.
That is carried by **AC-003** (no `src/` diff) and **AC-007** (no signature and no `spec/` edit), and gated by
the PR boundary — not by prose here.

**COMPOSITION invariants — N/A for UI; substituted in the design's own medium.** `_design.md` is explicit that
DT-7 — *"how a reader is told the log it is reading is incomplete, and why"* — is resolved as a vocabulary and
documentation decision in prose and in ADR-0028, **not** as a rendered UI (`:21-36`). This story does not
resolve DT-7; it supplies the evidence `dt-7-signal-shape-and-the-redaction-answer` resolves it from. The one
composition-shaped obligation that does bind is that the evidence artifact has a **fixed composition** —
reader, configuration, call, control value, hole value, then the silence enumeration, then the finding — which
is **AC-008**, with its configuration column depending on HS-S0114's `Debug` naming variant *and* bound.

**Which AC ids carry the residual invariants, and how each is verified.** Every one of these is a row in the
acceptance-criteria table above; this list is a map, not a second set of criteria.

| Invariant that applies | Carried by | Verified by |
| --- | --- | --- |
| Presentation exists at all — the record is composed data, not bare notes | **AC-008** | content review against the fixed five-column shape; a free-prose paragraph in place of a row fails |
| Density budget, with its real numbers — one row per observation, five columns, and every cell a value | **AC-008** | cell-by-cell cross-check against the assertion that produced it |
| Legibility of failure — an assertion that fails names the value it saw, not merely that it saw a wrong one | **AC-002** | the paired test's failure messages print both `Option<SequencePosition>` values and both `Result`s; no `should_panic` anywhere in the target |
| Hierarchy — an observation is distinguishable from the finding it supports, and neither is mistaken for a decision | **AC-003**, **AC-007** | module rustdoc and `_observations.md` state what these observations are *and are not*; the finding block names a DA-7 row and stops |
| Transience — nothing here is a scratch note: the record is persistent, committed, and cited by id from ADR-0028 | **AC-008** | `_observations.md` committed in this story's folder, in the PR boundary |
| Named anti-pattern — *a winner stated without what lost*, applied here as **a verdict recorded without the value** (`_decomposition.md:704-706`) | **AC-002**, **AC-008** | every cell traceable to a literal or printed value in the test output |
| Named anti-pattern — **a hazard demonstrated against a reader nobody ships** (`_decomposition.md:674-685`) | **AC-001**, **AC-005** | no local re-implementation of the read loop; no hand-written `IngestStore` impl in either branch |

## Error conditions

| id | condition | required handling |
| --- | --- | --- |
| **EC-001** | `crates/happenstance-testkit/tests/completeness_instrument.rs` is absent, or HS-S0114's retained-set type cannot express a hole over *matching* history. | **Halt and report as a dependency failure.** Do not build a second decorator, and do not widen this story's boundary into HS-S0114's file to reshape it — a copy of the instrument is a second instrument nobody validated. A shape defect is a defect report against HS-S0114, decided with it. |
| **EC-002** | The **control** run does not reject — the append is admitted under both configurations. | The observation is **void**, not "interesting". A condition that matched nothing is also what a young store produces, which is §3.7's own point (`spec/SPECIFICATION.md:4263-4272`). Re-seed so the `Query` matches events the hole then destroys; never record the pair, and never relabel the control. |
| **EC-003** | `read_decision_model` returns the **same** `Option<SequencePosition>` under both configurations. | The hole did not cover the last matching event. Fix the seeding or the hole arm's retained set; do not weaken the assertion, and do not substitute an inequality on something else. |
| **EC-004** | Calling `IngestStore::holds` panics with `todo!(…)`. | That is a fact about an unfinished write path (HS-P0017), **not** about forgetting, and it is not AC-006's observation. Take AC-005's **recorded** branch, cite `crates/happenstance-sync/src/ingest.rs:223-225`, and state what remains unobserved. Never replace the `todo!()` to make a test runnable. |
| **EC-005** | A `happenstance-sync` dev-dependency on `happenstance-testkit`, or the reverse, would make the ingest half convenient. | **Forbidden.** `happenstance-sync` is `publish = false` (`crates/happenstance-sync/Cargo.toml:12`) and `happenstance-testkit` is one of the three publishable crates the gate runs `cargo package --list` over. Use the conditional second mount in `crates/happenstance-sync/tests/` instead; no `Cargo.toml` edit is in this PR. |
| **EC-006** | An observation appears to need a port surface that does not exist — a way to say *"I cannot judge this"*, or a tri-state membership answer. | Record it as a **finding** against the DA-7 row it matches, with the version consequence, and stop (**AC-007**). Adding the surface here is the exact move project AC-012 exists to prevent (`project.md:240-243`). |
| **EC-007** | A rule in the instrument's existing `event_store_conformance!` mounts goes red while the readers are added. | That is a decorator or seeding defect, not an observation. Triage to HS-S0114 and to `cf-27-experiment-and-recorded-pass-list`; never absorb it into this story, and never harvest evidence from a red mount. |
| **EC-008** | The hole arm's append returns an `Err` other than the expected one, or the control arm returns an unexpected `Ok`. | Record the **actual** value first — an unexpected result is itself an observation under AC-002 — then explain it in `_observations.md`. Do not narrow the assertion to match the surprise, and do not delete the arm. |
| **EC-009** | The `EventId` minted for the membership observation cannot be resolved to a position through the inner store's `snapshot()`. | Halt on it rather than falling back to `EventId::position()`. That fallback is the wrong reduction DA-8 exists to expose, and using it would make the observation circular (`_decomposition.md:381-397`). |
| **EC-010** | The precondition check for AC-005 is ambiguous — a `holds` body exists but the two `EventId` types are still distinct, or vice versa. | Both halves must hold for the reached branch. A partially met precondition takes the **recorded** branch and says so, naming which half is missing; "close enough" here produces an observation about type-punning rather than about forgetting. |

## Non-functional

| id | requirement | why, and how it is checked |
| --- | --- | --- |
| **NF-001** | **No crate gains a dependency, and no `Cargo.toml` is edited.** | A dev-dependency from a publishable crate onto a `publish = false` one is a publish hazard this initiative is explicitly not allowed to create (project AC-011). Checked by `git diff --stat main...HEAD` showing no `Cargo.toml`. |
| **NF-002** | **Port-generic helpers bind `EventStore`, never `SendEventStore`; `#[async_trait]` appears nowhere; only one of the two trait names is imported per module.** | `CLAUDE.md` constraints 1 and 4, and [ADR-0001](.kb/decisions/0001-async-port-flavours.md): `EventStore` is the weaker bound and accepts both flavours, and both names in scope makes the method calls ambiguous. `read_decision_model` is itself `S: EventStore`, so this is inherited for free and can only be broken by adding a bound the story does not need. Checked by the absence of `async_trait` and `SendEventStore` in the readers module, and by clippy `-D warnings` under `cargo xtask affected`. |
| **NF-003** | **The four mandatory `wasm32` steps stay green and unchanged.** | The readers live in a native-only integration target, as HS-S0114's does; this story adds no `wasm32` obligation and must not remove one. Checked by `cargo xtask ci --fast` at the project's integration grain (`.redkiln/config.yaml:55`). |
| **NF-004** | **Observations are deterministic and self-contained.** No sleeps, no wall-clock dependence, no cross-test shared state; each pair constructs its own fixture instance, and control and hole run in the same process from the same seeding routine. | A flaky observation is not evidence, and ADR-0028 will cite these cells for years. Checked by running the target twice in succession, and by the absence of `sleep`, `Instant` and `SystemTime` in the readers module. |
| **NF-005** | **The observation is cheap enough to stay in the story gate** — the readers module adds no measurable time beyond a handful of in-memory appends. | The story grain runs on every diff (`.redkiln/config.yaml:40`); an expensive observation gets skipped, and a skipped observation stops being a guard. |
| **NF-006** | **No `unsafe`, no new lint allowance, no `#[allow]` beyond what the target already carries.** | `standards/rust/00-prime-directives.md`; clippy `-D warnings` is part of `cargo xtask affected`. A `todo!()` allowance in particular is a phase-scoped device for skeletons and has no business in a test target. |
| **NF-007** | **The record is readable without the repository open.** Every cell in `_observations.md` that names a code path carries a `file:line`, so ADR-0028 can cite it and a reader six months later can re-run it. | `RUNBOOK.md:4658-4662`'s proof-artefact bar. Checked at content review: a cell naming a call with no path fails. |

## Implementation notes (non-prescriptive)

Non-binding; the ACs are the contract. These are the traps this spec expects.

- **Read HS-S0114's target first, in full.** Its type names, its `Fixture`, its handle and its retained-set
  constructors are that story's to fix, and this story uses whatever it committed. Everything italicised in
  the *Behavior and interfaces* table is a role, not a name.
- **A `readers` submodule is probably the right shape.** `crates/happenstance-testkit/tests/mutation_coverage.rs`
  plus its `mutation_coverage/` directory is the precedent already in the tree: a subdirectory without
  `main.rs` is not itself a target, so `crates/happenstance-testkit/tests/completeness_instrument/readers.rs`
  needs no manifest edit. Inline is fine if it stays short.
- **Write the seeding once and share it between the arms.** The likeliest way to lose this story is two
  seeding routines that drift, at which point the difference between the arms stops being attributable to the
  retained set. One helper returning the seeded ids and the `Query`, used by both arms, makes the differential
  structural rather than disciplinary.
- **Capture the values before asserting on them.** Build the observation record — the two
  `Option<SequencePosition>`s, the two `Result`s, the two `contains_event_id` answers — and assert against the
  record. It makes the failure message carry the whole pair, and it makes `_observations.md` a transcription
  rather than a second authoring.
- **`_observations.md` is written from the test output, not from this spec.** If a cell is easier to write
  from the spec than from the run, the run is not producing it — and that is AC-008's failure mode exactly.
- **Do the ingest precondition check first, before writing anything else.** It decides whether the PR
  boundary's second glob is entered, and discovering it last is how a boundary gets widened under gate
  pressure.
- **Do not reach for `#[should_panic]`.** The testing brief warns against it by name here because the likely
  failure mode is a *silent wrong answer*, not a panic (`_decomposition.md:728-736`). If a reader genuinely
  panics, record the panic's message as the value — that is still a value.
- **`MemoryEventStore::restore` is never called.** It is ES-38's mutant and belongs to
  `positions-are-not-reused-after-removal`; calling it here would make the observation about renumbering
  rather than about forgetting (`crates/happenstance-core/src/memory.rs:163`).
- **Write the module rustdoc before the tests, not after.** The sentence *"this is the contract working as
  written; the question is what the caller can then conclude"* is the one that stops a reviewer reading the
  whole PR as a bug report, and it is much harder to write once the assertions exist.

## Tests and CI (merge gate)

Grounded in the project testing brief's tier table (the AC-006 row) and its *Merge-gate commands* section.
AC-006's tier is **observation** on purpose — *"the closest tier this repo has to e2e"* — and the wrong
implementation every row below rejects is *a predicted outcome written in prose instead of the actual returned
value*.

| tier | command / path | proves |
| --- | --- | --- |
| **observation (this repo's nearest-to-e2e tier)** | `cargo test -p happenstance-testkit --test completeness_instrument readers::` — `decision_model_loop_admits_under_a_hole_and_rejects_under_the_control`, `contains_event_id_answers_false_for_an_id_this_store_minted` | AC-001, AC-002, AC-004: the production loop ran over a real forgetting store in both configurations, and the recorded values are the ones the code returned. Rejects a prose write-up with no run behind it, and a run whose arms differ in more than the retained set. |
| **observation — ingest half, reached branch only** | `cargo test -p happenstance-sync --test ingest_holds_across_a_hole` | AC-005 (reached): the **production** `IngestStore::holds` answered `false` for a minted-and-acknowledged id. Rejects a hand-written `IngestStore` impl standing in for the real one. |
| **static — ingest half, recorded branch only** | content review of `_observations.md`'s precondition block against `crates/happenstance-sync/src/ingest.rs:223-225` as it stands at merge | AC-005 (recorded): the precondition was checked and the branch taken deliberately, with HS-P0017 named. Rejects silence about why the ingest half was not run, and a `todo!()` replaced to make a test runnable. |
| **regression guard (existing, unchanged)** | the `event_store_conformance!` mounts already in `crates/happenstance-testkit/tests/completeness_instrument.rs`, run by the same `cargo test` invocation — including `contains_event_id_reports_membership` and the `condition_*` family (`crates/happenstance-testkit/src/registry.rs`) | that these observations are about **forgetting** and not about a decorator bug. Rejects an observation harvested from a mount that was red (EC-007). |
| **meta-test guard (existing, must stay unchanged)** | `mutants_fail_exactly_their_declared_rules` and `mutant_registry_is_exhaustive` (`crates/happenstance-testkit/tests/mutation_coverage.rs`) | AC-003: no `REGISTRY` row and no rule were added by this story. Rejects a rule quietly written here that belongs to `condition-over-removed-history-does-not-reject`. |
| **static — content** | review of `_observations.md` against the fixed five-column shape, the silence enumeration and the finding block | AC-006, AC-007, AC-008. Rejects "it worked" prose (AC-T04, `_decomposition.md:704-706`), a channel present in `crates/happenstance-core/src/error.rs:214-225` but absent from the table, and a finding with no DA-7 row. |
| **static — boundary** | `git diff --stat main...HEAD`, asserted confined to the three PR-boundary globs, with nothing under any crate's `src/`, no `Cargo.toml` and no `spec/` file | AC-003, AC-007, NF-001. Rejects the fix made in passing, and the dev-dependency added for convenience (EC-005). |
| **story grain (fires automatically)** | `cargo xtask affected --base main` (`.redkiln/config.yaml:40`) | fmt, clippy `-D warnings` and tests for `happenstance-testkit` plus dependents — and `happenstance-sync` on the reached branch — together with the five file-reading lints and `spec-trace`, which run unconditionally, so NF-002 and NF-006 are gated even though this story writes no `src/`. |
| **integration grain, cheap tripwire** | `cargo xtask lints && cargo xtask spec-trace` (`.redkiln/config.yaml:48`) | `spec/SPECIFICATION.md` is untouched and still traces — AC-007's "no marker move" half. |
| **integration grain, non-terminal (project, not this story)** | `cargo xtask ci --fast` (`.redkiln/config.yaml:55`) | NF-003: the four mandatory `wasm32` steps and the doc builds stay green. Per AC-T05 this is the project's bar; the full `cargo xtask ci` is HS-P0019's and is at most a local sanity check here. |

## Risks and coupling (PR-scoped)

| risk | likelihood / impact | containment inside this PR |
| --- | --- | --- |
| **HS-S0114's retained-set type cannot express a hole over exactly the matching history this observation needs.** | Medium / High — it blocks both halves. | EC-001: halt and report as a defect against HS-S0114's shape, decided with it. The slice is implemented in one context with its mate, so that conversation is available rather than deferred. Never a second decorator here. |
| **The ingest precondition (HS-P0017's write path, the unified `EventId`) is unmet at implementation time.** | High / Low — the recorded branch is a first-class outcome, not a shortfall. | AC-005 pre-authorises both branches and forbids the third. `_grounding.md:187-207` already records the write path as not landed, so the recorded branch is a planned outcome and not a scope cut. |
| **The observation quietly becomes an assertion** — someone adds a conformance rule because the value looks assertable. | Medium / High — it pre-empts ADR-0028 and pulls `condition-over-removed-history-does-not-reject`'s work forward. | The Integration contract forbids adding a rule, AC-003 gates it by diff, and the meta-test guard row catches a `REGISTRY` addition. The sequencing reason is the story map's own: a rule written before its evidence is written to the answer someone expected. |
| **The fix gets made in passing.** A tri-state `contains_event_id` is a small, obvious, tempting diff. | Medium / High — it is a `0.3.0` this initiative's exit criteria do not contemplate. | EC-006 and AC-007: name the DA-7 row, record the version consequence, stop. Gated by the empty `crates/happenstance-core/` diff, which is the same check project AC-012's own testing row names. |
| **`_observations.md` degrades into a summary** under gate pressure, and ADR-0028 inherits prose. | Medium / High — the exact failure AC-T04 and `RUNBOOK.md:4658-4662` are written against. | AC-002 and AC-008: every cell traceable to a literal or printed value in the test output, checked cell by cell at review. Writing the file *from the run* is the cheap defence. |
| **Coupling to the slice-mate.** `projection-runner-across-the-hole` is blocked on HS-P0011 and may halt. | Medium / Low for this story. | The slice was split precisely so the runner's dependency is an isolated blocker rather than a risk to these two observations (`_storymap.md`, the AC-006 coverage row). This story merges first (`_storymap.md:137-140`) and none of its ACs references root 5. |
| **Coupling downstream.** `dt-7-signal-shape-and-the-redaction-answer` and `adr-0028-and-the-open-question-wave` both consume this file; `surface-diff-and-the-ac-012-escalation` consumes its finding. | Certain / High if the record is thin. | The record's shape is fixed here (AC-008) so consumers cite cells rather than re-derive, and the finding's shape is fixed by AC-007 so the escalation has a DA-7 row to carry. |
| **A reader run outside the instrument's target silently runs against a copy.** | Low / High — the evidence would then be about a second, unvalidated instrument. | The mount point is singular and stated in the Integration contract: an integration-test target is a separate compilation unit and is not importable, so the readers live in HS-S0114's own file. The only second target permitted observes a *different reader*, not a copy of the store. |
| **Clippy or fmt failures in a test target block a story that changes no production code.** | Medium / Low. | NF-002 and NF-006 are stated up front precisely so the `-D warnings` gate is designed for rather than discovered; the readers module inherits `read_decision_model`'s bounds and needs no `#[allow]`. |

## Dependencies

**Blocks on** — must be merged before this story starts:

- **`retained-set-instrument-and-conformance-mount`** (HS-S0114) — the completeness instrument, its
  retained-set type (including the `Debug` that names variant *and* bound, which is AC-008's configuration
  column), its `Fixture`, its handle, and its `event_store_conformance!` mounts in
  `crates/happenstance-testkit/tests/completeness_instrument.rs`. This story extends that file and consumes
  the instrument **as built**; if it is absent, EC-001 applies.

**Deliberately not dependencies:**

- `cf-27-experiment-and-recorded-pass-list` — these observations do not read the pass list; the two may run
  concurrently once the instrument lands.
- the `owed-rules-and-mutants` slice — no edge in either direction (`_storymap.md:130-134`); it may be
  interleaved with this slice.
- `projection-runner-across-the-hole` — the slice-mate. Implemented in the same context and mounted as one
  integrated surface, but with **no edge**: this story merges first and does not wait on HS-P0011.

**Unlocks:**

- **`dt-7-signal-shape-and-the-redaction-answer`** — names this story in its `depends_on`. DT-7 is *"how a
  reader is told the log it is reading is incomplete"*, and the silence enumeration (AC-006) is the evidence
  that there is presently no such telling.
- **`adr-0028-and-the-open-question-wave`** — names this story in its `depends_on`. It cites
  `_observations.md`'s cells as half its evidence base; the pass list is the other half.
- **`surface-diff-and-the-ac-012-escalation`** (project AC-012) — consumes this story's missing-surface
  finding and its DA-7 row assignment. Not a `depends_on` edge in the story map — its edge is to
  `marker-moves-and-spec-trace-green` — but it is the artefact this story's finding is written *for*.

## Anchors (progressive disclosure)

Deferred, not optional. Open each at the moment named; link, never bulk-paste.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/retained-set-instrument-and-conformance-mount/spec.md` | Fixes the instrument this story runs everything against: the retained-set type and its `Debug` (`:136-145`), and the condition evaluated over the *retained view* rather than delegated (`:113-124`) — the mechanism that makes the admitted append reachable at all. | **First, before writing any line**, alongside the file HS-S0114 actually committed, since names may differ from the spec. | AC-001 |
| `crates/happenstance-core/src/store.rs` | The two production readers, verbatim: `read_decision_model` (`:321-331`) returns the last matching event's position, which under a hole is the last *retained* match; `contains_event_id` (`:250-269`) carries the rustdoc explaining the cheap reduction that forgetting makes exactly wrong; `head` (`:248`) and `append` (`:213-217`) are two of the silence channels. | Before writing the read arm (AC-001), and again when building the silence table (AC-006). | AC-001 |
| `crates/happenstance-core/src/append.rs` | `after_opt` (`:201`) is what consumes `read_decision_model`'s return, and `Guard::is_violated_by` (`:239-253`) is the two-arm `match` with no third arm — the exact reason the condition passes vacuously. Reading it is what stops the observation being written up as a bug. | Before writing the `readers` module rustdoc and `_observations.md`'s framing sentence. | AC-003 |
| `crates/happenstance-core/src/memory.rs` | `snapshot()` (`:186`) is the **only** sanctioned route from an `EventId` to a position for the membership observation; `contains_event_id`'s real body (`:414-419`) is what the instrument decorates; `restore` (`:163`) is ES-38's mutant and must not be called. | Before writing the membership arm. | AC-004 |
| `crates/happenstance-core/src/error.rs` | `AppendError`'s variants (`:214-225`) and `ConditionViolated` (`:135-165`) are the vocabulary the silence enumeration is taken against — the point being that **no** variant means *"I cannot judge this condition"*. | When building the silence table; a channel in this file and absent from the table fails AC-006. | AC-006 |
| `crates/happenstance-sync/src/ingest.rs` | `IngestStore::holds` (`:164`) is the surface AC-005 is about, and its only production impl (`:206-229`) holds the `todo!("contains_event_id would answer this, once the placeholder EventId is unified")` at `:223-225` that *is* DA-8's inheritance argument in the source's own words. | At the **precondition check**, before deciding which branch AC-005 takes and therefore whether the PR boundary's second glob is entered. | AC-005 |
| `crates/happenstance-sync/src/identity.rs` | The placeholder `EventId` (`:26-39`, `:98`) that core's is not yet unified with — the second half of AC-005's precondition, and the reason the reached branch cannot be assumed. | With the anchor above, at the precondition check. | AC-005 |
| `crates/happenstance-sync/tests/ingest_reaches_a_foreign_store.rs` | The model for the conditional second mount: an integration target whose module docs write down what compiling something did *and did not* prove — the register AC-005's reached branch should be written in. | Only on the reached branch, before creating the new `crates/happenstance-sync/tests/` target. | AC-005 |
| `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/_grounding.md` | §4.3 (`:164-173`) the replication seam and its unresolved write path; §5 (`:187-207`) what has not landed — the recorded, planning-time basis for AC-005's two branches, so taking the recorded branch is a planned outcome rather than a discovery. | At the precondition check, to phrase the recorded branch's finding correctly. | AC-005 |
| `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/_decomposition.md` | The architecture brief holds composition roots 4 (`:115-123`) and 6 (`:135-142`), DA-7's four-row option table with its version consequence (`:363-379`), DA-8's location of the lie at `contains_event_id` (`:381-397`), the reader data-flow (`:413-434`), the "never a fake ingest" rule (`:670-685`), AC-T04 (`:704-706`) and the note against `#[should_panic]` (`:728-736`). | DA-8 before the membership arm; DA-7 before writing the finding; AC-T04 and the note before writing `_observations.md`. | AC-007 |
| `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/project.md` | DR-6 (`:160-164`) is the question this story answers and the reason "fails loudly: no" must be evidenced rather than claimed; DR-12 and project AC-012 (`:240-243`) are why the finding stops at a finding. | Before the silence table (DR-6), and before the finding block (AC-012). | AC-006 |
| `spec/SPECIFICATION.md` | §3.7 (`:4263-4272`) states the gap in the exact terms this story instantiates; ES-39 (`:4325-4349`, with `:4336-4341` the argument against a floor) and ES-40 (`:4351-4379`, `[PROVISIONAL]`, this instrument its named falsifier) are the clauses being exercised; CF-2 (`:7192-7196`) is the discipline that forbids recording only *that* something failed; CF-27 (`:8034-8060`) is what a later story settles. | ES-40 before the read arm's framing; CF-2 before choosing the assertion shape; §3.7 when writing `_observations.md`'s opening. | AC-002 |
| `crates/happenstance-testkit/tests/mutation_coverage.rs` | Its module doc, *"Why the wrong implementations are no longer here"*, is the in-tree statement of CF-2's discipline; its `REGISTRY` + `for_each_mutant!` mechanism is the thing this story must leave **unchanged**. | Before choosing the assertion shape (AC-002), and again at the boundary check. | AC-002 |
| `crates/happenstance-testkit/src/registry.rs` | The single rule registry — the names of the rules already running green against the instrument (`contains_event_id_reports_membership`, the `condition_*` family). They are the guard that these observations are about forgetting rather than a decorator bug, and this is the file this story must **not** add a line to. | When triaging EC-007, and when citing the regression-guard tier. | AC-003 |
| `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/_design.md` | The signed-off no-surface determination (`:38-95`) and the framing naming these three readers as *"Rust code paths, not a screen"* (`:21-36`) — binding, and the reason this story owes a composed **record** rather than a composed screen. | Before writing `_observations.md`'s structure; and whenever a UI-shaped obligation seems to apply. | AC-008 |
| `RUNBOOK.md` | Phase 14 (`:4626-4674`): the *Work* (`:4644-4650`) and the **Proof artefact** (`:4658-4662`) — *"a runner and an ingest path that both fail against it"* — the bar `_observations.md` is measured against. | Before writing `_observations.md`, and again at the story review gate. | AC-008 |
| `.bklg/from-contract-to-published-library/initiative.md` | The four personas and their fears (`:203-226`) — in particular the application author's *"a second, independent reader building a wrong answer from a torn or gapped log"* — and DoD scenario 15 (`:402-404`), which this story is half of. | When writing `_observations.md`'s opening and the finding's audience framing. | AC-002 |
| `.kb/decisions/0001-async-port-flavours.md` | The accepted decision behind `CLAUDE.md` constraints 1 and 4: why no `#[async_trait]`, and why generic code binds `EventStore` rather than `SendEventStore`. | Only if a helper in the readers module needs a bound — that is, the moment a `Send` bound looks necessary. | AC-001 |

## Clarifications resolved during spec

1. **The AC set is exactly the eight the first pass enumerated** — AC-001 … AC-008. None added, none dropped;
   the ledger carries the same eight ids and no others.
2. **Project AC-006 is the only traced AC, and this story owns two of its three readers.** Root 5 (the
   projection runner) is `projection-runner-across-the-hole`'s, per the story map's Coverage row for AC-006.
   AC-006's *"recorded against AC-011 rather than papered over"* clause is discharged here as a **finding**
   (AC-007); the escalation artefact itself belongs to `surface-diff-and-the-ac-012-escalation`.
3. **Interaction quality is N/A-with-reason, not skipped.** `_design.md` records every surface section as
   *"N/A — no user-facing surface"*, approved 2026-08-12 with `design.capture` a declared skip. No composition
   AC is invented for a screen that does not exist; the residual invariants are mapped onto AC-002, AC-003,
   AC-007 and AC-008 in the *Interaction quality* section rather than duplicated as new rows.
4. **AC-005 has two legitimate outcomes and this spec pre-authorises both**, because whether HS-P0017 has
   landed is not knowable at planning time (`_grounding.md:187-207`). The recorded branch is a first-class
   pass, not a waiver; what is forbidden is a third outcome — a hand-written `IngestStore` impl or a replaced
   `todo!()`. EC-010 settles the ambiguous case: a partially met precondition takes the recorded branch.
5. **The mount and no-dependency constraints are NF-001 rather than an AC row**, because they are properties
   of the diff rather than persona outcomes. They are still gated: every ledger row's `mount_point` names the
   instrument's own target, and the boundary tier asserts no `Cargo.toml` is touched.
6. **"Silence" is an enumeration with a fixed membership**, taken from `crates/happenstance-core/src/error.rs:214-225`
   and `store.rs:248`, `:250-269`, `:321-331`. That makes AC-006 checkable — a channel present in the source
   and absent from the table fails the row — rather than a matter of how thorough the author felt.
7. **`_observations.md` lives in this story's own folder.** The slice-mate records its runner observation in
   its own folder the same way; there is no shared project-level file two stories both write, which would make
   merge order load-bearing for no benefit.
8. **The verifying tests are named here, not left to the bench.** `decision_model_loop_admits_under_a_hole_and_rejects_under_the_control`,
   `contains_event_id_answers_false_for_an_id_this_store_minted` and (reached branch only)
   `ingest_holds_across_a_hole` are this story's to create; the ledger cites them, so renaming one is a ledger
   edit and therefore a visible decision rather than a silent drift.
