---
item: HS-S0043
stage: spec
created: 2026-08-12T13:46:41.648Z
updated: 2026-08-12T13:46:41.648Z
template_sig: 87bbf1d0
rendered_sig: 864e41b8
---

# Spec — The reopen rule gets a negative control, and the durability clauses get verdicts

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` |
| Project | `.bklg/from-contract-to-published-library/sqlite-durable-store/project.md` |
| This spec | `.bklg/from-contract-to-published-library/sqlite-durable-store/reopen-negative-control-and-durability-verdicts/spec.md` |
| Key briefs | `.bklg/from-contract-to-published-library/sqlite-durable-store/_decomposition.md` — architecture §7 (schema, migration, identity), §9 (tensions recorded not settled), §10 (testkit blast radius); testing §2 (AC-to-tier), §5 (the missing negative control), §6 (merge-gate commands) |
| Signed-off design | `.bklg/from-contract-to-published-library/sqlite-durable-store/_design.md` — **no user-facing surface**, approved 2026-08-12; this story renders none |
| Story map (this row, this slice) | `.bklg/from-contract-to-published-library/sqlite-durable-store/_storymap.md` — Slices table, `race-model-and-durability` |
| Roadmap pointer | `RUNBOOK.md:4166-4237` (phase 8), `RUNBOOK.md:3205-3207` (the gap this story closes), `RUNBOOK.md:605` (CF-17/CF-34/ES-35 owned by phase 8) |

## One-line PR slice

Give `recorded_time_survives_a_reopen` the negative control it has lacked since
phase 4 — a permanent registry row encoding a re-stamped `recorded_at` — and land
the recorded verdicts for CF-17's rule shape, ES-35's marker and CF-14's
deferral.

## Executive summary

Two deliverables, one theme: **a durability claim that can be checked.**

The first is an instrument. `recorded_time_survives_a_reopen`
(`crates/happenstance-testkit/src/suite.rs:2473-2523`) carries a headline
assertion — a `RecordedAt` is persisted alongside the event, not recomputed when
the store is opened — and **nothing in the workspace has ever reached it**. The
one registered mutant that fails the rule, `LosingFixture`, fails it at the
*survival* anchor: its events are gone, so the stamp comparison is never
executed (`crates/happenstance-testkit/tests/mutation_coverage.rs:1193-1230`).
`RUNBOOK.md:3205-3207` records that as an open debt owed to phase 8. This PR pays
it by adding one longhand fixture to the mutant registry whose reopen restores
every store-assigned fact **except** the stamp, which it recomputes — so the rule
goes red at its own sentence, by name, forever.

The second is a verdict. Three clauses — ES-35, CF-17, CF-14 — have carried
maturity markers whose falsifier was "the first file-backed adapter". That
adapter now exists and passes, courtesy of `sqlite-fixture-and-whole-suite`. This
PR reads each marker against what actually happened and writes the answer into
`spec/SPECIFICATION.md`, including correcting the sentences that have become
false. It does **not** move any marker's *level*: that is an ADR's act, escalated
to the runbook's ADR queue rather than settled in passing.

Delta against the project charter: `project.md` AC-004's third clause ("gets one")
and AC-010 in full. Nothing else about the SQLite adapter changes here — the
fixture, the schema and the suite arrived with this story's dependency.

## Context pack

The load-bearing decisions, stated as decisions. Everything deeper is a
signposted anchor below.

### 1. The rule this story arms, and the exact assertion that must fail

`recorded_time_survives_a_reopen` appends one tagged event, reads it back to
capture `(position, recorded_at)`, drops the handle, calls `fixture.reopen()`,
connects again and asserts **three** things in order: the event survived at all;
it is at the same position; and `all[0].recorded_at == recorded_at`. Only the
third is the rule's reason for existing. A negative control that dies at the
first or second assertion proves the setup works and proves nothing about the
sentence — that is precisely `LosingFixture`'s shape, and it is why phase 4
closed with the debt open.

**Decision: the new control must reach and fail the third assertion, and must
fail nothing else.** Its `Declared` row pins that with an `expect` entry quoting
the headline message, exactly as `LosingFixture`'s row pins its three.

### 2. Why the control must be written longhand, and why it needs a generation counter

Two traps, both already documented in the tree, both easy to walk into.

**It cannot be a `Defect` step.** `Defect` is a trait of *store* steps and every
mutant that overrides one inherits the rest
(`crates/happenstance-testkit/tests/mutation_coverage/mutants.rs:96-130`). There
is no reopen step, because reopen is a *fixture* operation. `mutants.rs`'s module
doc names eight stores written out longhand for exactly this class of reason and
`LosingFixture` is one of them, "because its defect is what a reopen finds"
(`mutants.rs:29-43`). The new control joins that list, with the same one-sentence
justification in its own doc comment.

**A naive restamp is invisible in this binary.** `correct::stamp` spends the
constant `TEST_RECORDED_AT`
(`crates/happenstance-testkit/tests/mutation_coverage/correct.rs:348-358`), fixed
rather than read from a clock because CF-33 forbids one. `GappedPositionStore`'s
doc comment already records the consequence in terms: it *does* restamp on
replay, and passes `recorded_time_survives_a_reopen` anyway, "and only because
`correct::stamp` spends a **constant** … so the restamp lands on the same value
it replaced and nothing can see it"
(`crates/happenstance-testkit/tests/mutation_coverage/variants.rs:85-104`). A
control that replays through `correct::stamp` is therefore a control that passes.

**Decision: the new stamp is a deterministic function of a per-fixture reopen
generation** — `RecordedAt::from_millis(TEST_RECORDED_AT.as_millis() + generation)`
or equivalent (`crates/happenstance-core/src/identity.rs:160-172`). Deterministic,
so the mutant harness stays reproducible; monotone per reopen, so the new value
is *never* equal to the old one; and not a clock, so CF-33 is untouched and the
row cannot go flaky on a fast machine. **A wall-clock stamp is forbidden here**,
and not only on CF-33 grounds: millisecond resolution makes "the two stamps
differ" a race the test would lose intermittently.

### 3. The defect must be one a real adapter author would ship

CF-4's bar, restated in `mutants.rs:1-9`: "Every store here is a defect somebody
would ship … `struct AlwaysWrong` satisfies `every_rule_has_a_mutant`
mechanically and proves nothing." The provenance this row declares is the schema
mistake this project's own architecture brief spent a paragraph forbidding: a
migration that stores payload, type and tags but **no `recorded_at` column**, so
`open` reconstructs `SequencedEvent`s by replaying rows and stamping them at open
time. Every event still reads back; every position is still right; only the one
clock reading whose provenance the log itself attested is silently replaced. It
is the mirror image of the decision at architecture brief §7 — `recorded_at`
persisted and **read back, not re-stamped** — and of `DurableFixture`'s own
choice to carry `SequencedEvent` rather than `Event` on its durable side
(`crates/happenstance-testkit/tests/fixture_instruments.rs:12-21, 57-73`).

### 4. Registering a mutant is a three-place obligation, held together mechanically

`mutants.rs:60-73` states the procedure and says why steps 2 and 3 are separate:
"The type enumeration and the claim about it are the two lists that can drift,
and `mutant_registry_is_exhaustive` exists to hold them together." A fourth place
is not in that list and is required anyway, because the model family measures
*every* store in the binary:

1. the fixture type, in `mutation_coverage/mutants.rs`;
2. the type added to `for_each_mutant!`
   (`crates/happenstance-testkit/tests/mutation_coverage.rs:2100-2148`);
3. the `Declared` row in `REGISTRY` (`mutation_coverage.rs:324`) — `fails`,
   `provenance`, `mode`, and the `expect` pin;
4. the `MODEL_COVERAGE` row (`mutation_coverage.rs:2326-2435`), whose doc comment
   states a running count of the model's misses and whose "defects that are only
   visible across a reopen" bullet gains a second name.

**Decision: the count in that doc comment is updated in the same change.** It is
prose that goes stale silently, and the heading currently reads "The twenty it
does not catch are three shapes, not twenty"
(`mutation_coverage.rs:2276-2282`) — a number that has already been wrong once,
which the comment itself records.

### 5. This story adds no conformance rule — unless CF-17's verdict adds one

Architecture brief §10 is explicit: "this project adds **no** conformance rule as
such — but AC-010 settles **CF-17's rule shape**, and if that reshapes or adds a
rule, `mutation_coverage::every_rule_has_a_mutant` fails until a `REGISTRY` row
exists". The expected verdict is that the rule shape is already right and nothing
is added. **Decision: if the verdict does reshape or add a rule, CF-1's registry
row and CF-29's `CHANGELOG.md` entry land in the same change, not as a
follow-up** (`spec/SPECIFICATION.md:8141-8165`).

### 6. The three verdicts, and the frame each is decided in

The project's AC-010 permits a range; the architecture brief §9 recommends within
it. Both are binding here, and the recommended verdicts are stated so the
implementer decides against evidence rather than from scratch.

- **ES-35 (durability, `[PROVISIONAL]`).** This adapter supplies the *reopen* far
  end and does **not** supply the *fault* far end. `MID_BATCH_FAULT` stays
  declined with the fixture's real reason
  (`crates/happenstance-testkit/src/contract.rs`, and the shape `MemoryFixture`
  uses at `fixtures.rs:275-284`). **Recommended verdict: stays `[PROVISIONAL]`,
  with the marker text restated so the falsifier that remains is named exactly —
  a store that loses a write to a *fault* rather than to an instruction.** What
  must also change is the clause's closing paragraph, which asserts "no fixture in
  that binary has a medium outside the process": that sentence describes the
  testkit's own binary and is still true there, but the clause offers it as the
  reason the axis has "an instrument at one end and nothing at the other", and
  that half is now false (`spec/SPECIFICATION.md:4142-4180`).
- **CF-17 (the `REOPEN` capability, `[PROVISIONAL]`).** Its falsifier is "a
  legitimate adapter that is durable and cannot express even a reopen through this
  contract". A real file-backed adapter now expresses it. **Recommended verdict:
  the rule shape is confirmed — the contract names the weaker operation and that
  is sufficient; the stronger host-restart split stays unbought and un-clauseed.**
  The sentence naming `DurableFixture` as "the only fixture in the workspace that
  supplies this capability" has become false and must be corrected
  (`spec/SPECIFICATION.md:7570-7596`).
- **CF-14 (the durability rule's obligation, `[DEFERRED]`).** What is deferred is
  not the rule — it landed early as a named exception — but the *experiment*:
  whether one `reopen` shape serves rusqlite, a Durable Object and a one-shot HTTP
  client, or whether "durable" needs grading. One of the three is now in.
  **Recommended verdict: the deferral is confirmed and narrowed** — its falsifier
  becomes the two implementations that have not yet answered, with their owning
  projects named (`cloudflare-durable-object-store` HS-P0013,
  `postgres-and-neon-stores` HS-P0014) (`spec/SPECIFICATION.md:7471-7500`).

**Decision, and it is the one most likely to be violated by accident: no maturity
marker changes *level* in this PR.** `.kb/open-questions/es-7-and-vt-9-provisional-markers.md`
records the standing rule — "Moving a maturity marker is an ADR's act, so both are
recorded rather than moved" — and `CLAUDE.md` says changing a `[FROZEN]` clause
takes a new ADR. Restating a falsifier *within* a level, and correcting a sentence
that has become factually false, are edits this story owns. Promoting ES-35 or
CF-17 to `[FROZEN]` is not; it is escalated to the runbook's ADR queue as a named
item, with this story's evidence attached. A verdict recorded and escalated
discharges AC-010; a marker flipped in passing does not.

### 7. The persona slice this realizes

`_storymap.md` names the user as "an adapter author and the library's consumer",
and the increment either can observe is the same: *a durability claim they can
check rather than take on trust*. The adapter author reads
`recorded_time_survives_a_reopen`, sees a registry row that fails it, and knows
the rule bites before they run it against their own store. The consumer reads
ES-35's marker and finds a falsifier that names something still missing rather
than a blank. Both are the initiative's premise in miniature: six things look like
evidence and are not.

### 8. What this story is *not* allowed to do

The rule may not be weakened. `variants.rs:96-104` already anticipates the
temptation and answers it — the day this binary's stamp varies,
`GappedPositionStore` starts failing the rule "and the answer will be to give the
durable side `SequencedEvent`s rather than to weaken the rule." If the new control
makes an existing conformant variant go red, that variant is fixed at its durable
side, not the assertion.

## Integration contract

- **Archetype**: `capability`. It is user-observable in the only medium this
  project has — a rule that now demonstrably bites, and three clause markers a
  reader can act on.
- **Slice / milestone**: `race-model-and-durability`. Slice-mates, implemented in
  one context and mounted together: `concurrency-family-and-contender-count`,
  `model-family-and-mutant-pass-column`. Merge order within the slice puts those
  two in sequence and this one in parallel with either
  (`_storymap.md`, *Merge order* 3).
- **Mount point**: `crates/happenstance-testkit/tests/mutation_coverage.rs` — the
  composition root of the mutant harness. `for_each_mutant!` (`:2100-2148`)
  enumerates every subject, `reports()` (`:2150-2160`) drives each through every
  registered rule, `REGISTRY` (`:324`) declares what each is claimed to fail and
  `MODEL_COVERAGE` (`:2326`) records what the model family sees. A fixture that
  exists in `mutants.rs` and is absent from this file is a control nothing runs —
  and `mutant_registry_is_exhaustive` (`:2702-2790`) is the test that says so.
- **Wires into**:
  - `crates/happenstance-testkit/tests/mutation_coverage/mutants.rs` — the eight
    longhand fixtures and the `Defect` trait the new control deliberately does not
    use; `LosingFixture` (`:3600-3645`) is the shape to sit beside.
  - `crates/happenstance-testkit/tests/mutation_coverage/correct.rs:348-358` —
    `TEST_RECORDED_AT` and `correct::stamp`, the constant the new stamp must
    deliberately differ from.
  - `crates/happenstance-testkit/tests/mutation_coverage/harness.rs` — `Subject`,
    `run_subject`, and the `UnwindSafe` constraint the store shape must respect.
  - `crates/happenstance-testkit/src/suite.rs:2473-2523` — the rule under test and
    its three assertions, read but not edited.
  - `crates/happenstance-core/src/identity.rs:160-172` — `RecordedAt::from_millis`
    / `as_millis`, the only arithmetic seam the control needs.
  - `crates/happenstance-sqlite/tests/conformance.rs` — the slice dependency's
    mount, *run* here as AC evidence and not edited.
  - `spec/SPECIFICATION.md` — ES-35 (`:4142-4180`), CF-14 (`:7471-7500`), CF-17
    (`:7570-7596`), plus the clause-status tables at `:8617`, `:8725`, `:8728`
    that `cargo xtask spec-trace` reads.
- **Renders surfaces**: **none.** `_design.md` records "N/A — no user-facing
  surface" for this project, approved at the `/redkiln:plan` sign-off gate, and
  this story does not create one.
- **Public items**: none. `_design.md`'s `## Items` block is `N/A`; nothing here
  is `pub` outside a `tests/` target. The new control is `pub(crate)` in the
  integration-test crate, matching every sibling in `mutants.rs`.
- **Conformance rule(s)**: `recorded_time_survives_a_reopen` (the rule this story
  arms), `acknowledged_writes_survive_a_reopen` and
  `reopened_store_does_not_reissue_an_event_id` (which the new control must **not**
  fail — that is what makes it one defect rather than a second `LosingFixture`).
  The meta-tests that observe the mount: `mutant_registry_is_exhaustive`,
  `mutants_fail_exactly_their_declared_rules`, `every_rule_has_a_mutant`. **No new
  rule is added** — see Context pack §5 for the one branch where that changes.
- **Clause(s)**: discharges the phase-8 half of **ES-35**, **CF-17** and **CF-14**
  by recording a verdict and restating each falsifier; supplies the missing
  negative control behind **VT-9**'s second named rule
  (`spec/SPECIFICATION.md:903-930`), whose own `[PROVISIONAL]` marker is owned by
  phase 9 and is **not** touched here. No `[FROZEN]` clause is amended. No marker
  changes level; level changes are escalated to the ADR queue.
- **Advances DoD scenario**: **DoD 3** — "The durable store passes the suite for
  real … an acknowledged write surviving a process reopen" — by making the reopen
  half of that pass falsifiable rather than merely green. Contributes to **DoD 12**
  ("no clause is provisional with an empty falsifier"), which
  `publication-and-positioning` audits and which reads whatever this story writes.

## PR boundary

### In this PR

- One new longhand fixture in `mutation_coverage/mutants.rs`: a durable-medium
  store whose reopen restores positions, identities and payloads and re-stamps
  `recorded_at`.
- Its four registrations: `for_each_mutant!`, `REGISTRY` (with `expect` pin),
  `MODEL_COVERAGE`, and the model doc-comment's miss count and reopen bullet.
- The `CHANGELOG.md` entry naming the defect the new control encodes.
- The ES-35, CF-17 and CF-14 marker restatements and false-sentence corrections in
  `spec/SPECIFICATION.md`, at their existing maturity levels.
- The story's own `_ledger.md` and any companion notes in its backlog folder.
- Any ADR-queue escalation *note* required by Context pack §6 — recorded in this
  story's folder and in the ledger, **not** authored as an ADR here.

### Explicitly not in this PR

- **Authoring or amending an ADR.** ADR-0022 is
  `adr-0022-append-condition-strategy`'s, and an accepted decision atom is
  immutable (`CLAUDE.md`, *Where the work lives*). Record the gap; the runbook's
  ADR pass decides it.
- **Moving any maturity marker's level**, including promoting ES-35 or CF-17 to
  `[FROZEN]`.
- **The `SqliteFixture` itself** — its `REOPEN` / `SECOND_HANDLE` declarations, its
  ceilings and its `MID_BATCH_FAULT` decline are `sqlite-fixture-and-whole-suite`'s
  and are consumed here, not edited.
- **The concurrency family, `CONTENDERS`, or the mutant pass column for
  `SqliteEventStore`** — slice-mates
  `concurrency-family-and-contender-count` and `model-family-and-mutant-pass-column`.
- **Any SQL, schema or `happenstance-sqlite/src/**` change**, including the
  `recorded_at` column, which is `schema-migration-and-identity`'s.
- **Removing `todo!()` or the scoped `#![allow(clippy::todo)]`**, and the
  `spec-trace` citation-count reconciliation as a whole — those are
  `instrument-markers-removed-and-gate-green` and `spec-and-code-reconciliation`.
- **VT-9's own provisional marker**, owned by phase 9's Workers skeleton
  (`.kb/open-questions/es-7-and-vt-9-provisional-markers.md`).

The implementer may also touch the composition-root files named in the
Integration contract in order to mount this control; that is not scope drift.

### Merge DoD

The new control is registered, fails `recorded_time_survives_a_reopen` at its
headline assertion and nothing else, the three durability clauses carry written
verdicts at unchanged maturity levels, and
`cargo xtask affected --base {{base}}` plus `cargo xtask lints && cargo xtask spec-trace`
are green.

```
crates/happenstance-testkit/tests/mutation_coverage.rs
crates/happenstance-testkit/tests/mutation_coverage/**
spec/SPECIFICATION.md
CHANGELOG.md
standards/rust/**
.bklg/from-contract-to-published-library/sqlite-durable-store/reopen-negative-control-and-durability-verdicts/**
```

**`standards/rust/**` was added on 2026-08-17, and it admits citation
re-anchoring ONLY** — the same terms as `benchmark-harness`'s entry (`e020276`),
which is this initiative's precedent for the class. Two constitution atoms cite
this slice's files by `file:line`:
`23-streams-and-state-machines.md:191` cites
`crates/happenstance-testkit/tests/mutation_coverage.rs` for `RefetchingPagedStore`,
and `41-declarative-macros.md:197` cites
`crates/happenstance-testkit/src/concurrency.rs` for the `ConcurrentFixture`
impl. This slice's `Declared` row and its slice-mate's `CONTENDERS` doc comment
moved both subjects, and `cargo xtask lint-constitution` — reached by
`cargo xtask lints`, the first half of the `reachability_static` grain — fails on
a citation that no longer lands within ten lines of its subject. The story is
therefore forced across its boundary or into a red gate, with no third option.
This entry permits **line-number repair to existing citations and nothing else**:
rule text, evidence selection, rule retirement and new atoms all stay outside, so
the widening cannot later be cited to justify editing a rule. Actual use is four
citations in four atoms, each a balanced one-line insertion and deletion with no
prose changed: `23-streams-and-state-machines.md:191`
(`mutation_coverage.rs:1585→1619`), `41-declarative-macros.md:197`
(`concurrency.rs:1144→1176`), and — because the same review's correction to
§6.5's summary and §1.3's qualification moved lines in the specification itself —
`01-standard-of-evidence.md:127` (`SPECIFICATION.md:5910→5926`) and
`51-features-and-no-std.md:55` (`SPECIFICATION.md:355→366`). The last two are the
same mechanism arriving from the other side: this story edits `spec/SPECIFICATION.md`
inside its boundary, and two atoms cite that file by `file:line`.

**Where this block sits, and why it is not moved.** `redkiln verify --grain
story` reads the **first fenced block under the `## PR boundary` heading**, and
it stops at the next heading — so this block, which lives under the `### Merge
DoD` subheading, is not read: the check reports `no boundary declared` and skips,
and so does the `provenance` check behind it. Moving the block up to sit directly
under `## PR boundary` makes both run and both **fail** while `links.commits` is
empty, because the boundary check then diffs the whole initiative branch against
`main` and every sibling story's file is outside this story's globs. Recording
the checkpoint SHA is `redkiln record-links`' job and the orchestrating command's
to run — an implementer may not write an item's `links` — so the placement is
left as planning authored it and the parser behaviour is **routed** rather than
worked around here: it belongs to whoever owns the story template, alongside the
observation in this story's implementation report that `cargo xtask affected`
does not run `lint-constitution`. Both are the same class of gap — a check that
reports green because it never looked.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| A registered mutant fails `recorded_time_survives_a_reopen` at its **headline** assertion | A longhand fixture with a durable side of `SequencedEvent`. `reopen()` restores every store-assigned fact except `RecordedAt`, which it recomputes. The `Declared` row's `expect` pin quotes the headline message ("a `RecordedAt` is persisted alongside the event, not recomputed when …"), so a failure at the survival anchor is reported as the wrong failure rather than as a pass. | `crates/happenstance-testkit/src/suite.rs:2473-2523` (the three assertions); `crates/happenstance-testkit/tests/mutation_coverage.rs:1192-1230` (`LosingFixture`'s row — the pin shape, and the row that fails at the anchor instead) |
| The restamp is deterministic and monotone, never a clock | New stamp derived from the reopen generation, e.g. `RecordedAt::from_millis(TEST_RECORDED_AT.as_millis() + generation)`. Replaying through `correct::stamp` would re-spend the same constant and the mutant would pass; a wall clock would make it intermittent at millisecond resolution and would contradict CF-33. | `crates/happenstance-testkit/tests/mutation_coverage/correct.rs:348-358`; `crates/happenstance-testkit/tests/mutation_coverage/variants.rs:85-104`; `crates/happenstance-core/src/identity.rs:160-172` |
| One defect, one rule | The control must **pass** `acknowledged_writes_survive_a_reopen` (events and positions survive) and `reopened_store_does_not_reissue_an_event_id` (incarnation minted once, positions restored). `mutants_fail_exactly_their_declared_rules` is what makes that assertion rather than a hope. | `crates/happenstance-testkit/tests/mutation_coverage.rs:2702-2860`; `mutants.rs:11-28` (one-step discipline and the eight longhand exceptions) |
| It is written longhand, not as a `Defect` step | `Defect` has no reopen step because reopen is a fixture operation; `LosingFixture` is longhand for the same reason and says so. The new type's doc comment carries the same one-sentence justification and is added to the module doc's list of longhand exceptions. | `crates/happenstance-testkit/tests/mutation_coverage/mutants.rs:29-43, 96-130, 3600-3645` |
| Registration is complete in all four places | Type in `mutants.rs`; type in `for_each_mutant!`; `Declared` row in `REGISTRY`; `MODEL_COVERAGE` row (`ModelOutcome::Agreed` — the model never reopens). Two lists that can drift are held together by `mutant_registry_is_exhaustive`; the fourth is held by the model-coverage test. | `crates/happenstance-testkit/tests/mutation_coverage.rs:324, 2100-2148, 2326-2435, 2702-2790`; `mutants.rs:60-73` |
| The model doc comment's arithmetic is corrected | The "twenty it does not catch" heading and its "defects only visible across a reopen" bullet both change. That prose is the file's own answer to *what is this test blind to*, and it has been wrong before. | `crates/happenstance-testkit/tests/mutation_coverage.rs:2265-2300` |
| No conformance rule is added — with one named branch | If CF-17's verdict reshapes or adds a rule, `every_rule_has_a_mutant` (CF-1) and the `CHANGELOG.md` lint (CF-29) both fire, and both are satisfied in the same change. The expected verdict adds none. | `.bklg/from-contract-to-published-library/sqlite-durable-store/_decomposition.md` architecture §10; `spec/SPECIFICATION.md:8141-8165`; `crates/happenstance-testkit/README.md` |
| The adapter's reopen pass is observed against the armed rule | `recorded_time_survives_a_reopen` and `acknowledged_writes_survive_a_reopen` green against `SqliteFixture` — the same target, re-run after the control lands, so the pass is a pass *of a rule shown to be failable*. `MID_BATCH_FAULT` is observed as declined with a stated reason, which is what keeps ES-35's fault far end honestly open. | `crates/happenstance-sqlite/tests/conformance.rs` (owned by `sqlite-fixture-and-whole-suite`); `crates/happenstance-testkit/src/contract.rs`; `crates/happenstance-testkit/src/fixtures.rs:275-284` |
| ES-35 leaves with a restated falsifier | Stays `[PROVISIONAL]`; the marker names the remaining falsifier as a store that loses a write to a *fault* rather than to an instruction; the closing paragraph's claim that the axis has "nothing at the other end" is corrected to name the adapter that now sits there. | `spec/SPECIFICATION.md:4142-4180`; architecture brief §9 |
| CF-17's rule shape leaves settled | The weaker reopen operation is confirmed sufficient against a real file-backed durable adapter; the stronger host-restart split stays unbought. The sentence naming `DurableFixture` as the only supplier of the capability is corrected. | `spec/SPECIFICATION.md:7570-7596`; `crates/happenstance-testkit/tests/fixture_instruments.rs:1-45` |
| CF-14's deferral leaves confirmed and narrowed | One of the three implementations named in the deferral has answered. The falsifier becomes the two that have not, with owning projects named. | `spec/SPECIFICATION.md:7471-7500`; `.bklg/from-contract-to-published-library/_decomposition.md` (HS-P0013, HS-P0014) |
| No marker changes level here | Level changes are an ADR's act. The recommended verdicts keep every level as-is; any promotion is recorded as an ADR-queue item with this story's evidence attached, in the story folder and the ledger. | `.kb/open-questions/es-7-and-vt-9-provisional-markers.md`; `CLAUDE.md`, *Open questions, deliberately unresolved* |
| The record survives the gate | `cargo xtask spec-trace` green with no citation lost — every rule name the edited clauses cite still resolves; `cargo xtask affected --base {{base}}` green over `happenstance-testkit` and its dependents. | `.redkiln/config.yaml` `affected_gate` / `reachability_static`; testing brief §6 |

## Data and migrations

**N/A.** This story adds no schema, no migration and no persisted format.

Two things that look like data and are not:

- The new control's "durable medium" is an in-process `Vec<SequencedEvent>` behind
  a lock, in the same shape `DurableFixture` uses
  (`crates/happenstance-testkit/tests/fixture_instruments.rs:57-90`). The testkit's
  dependency graph is `happenstance-core` and `futures-core` and nothing else, and
  architecture brief §1 forbids putting a `rusqlite`-backed fixture anywhere inside
  it — so the control cannot and must not acquire a file.
- The **real** `recorded_at` column, the persisted `StoreId` and the `UNIQUE`
  origin pair belong to migration 1 and are `schema-migration-and-identity`'s
  (architecture brief §7). They are this story's *precondition* — they are what
  makes the rule's question askable against the adapter at all — and are read here,
  never changed.

## Acceptance criteria

Seven criteria. Each is framed from the goal of a persona the initiative carries
(`.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md`)
— the **adapter author** in *Learn when you are finished*, the **evaluator** in
*Decide in one sitting*, the **application author** in *Choose a contract before a
database* — and crosses the whole stack this project has: rule, registry, adapter,
specification, gate. Together they discharge project **AC-004**'s third clause and
**AC-010** in full.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **GIVEN** an adapter author reading `recorded_time_survives_a_reopen` to learn whether their own store's reopen is good enough, **WHEN** they run the mutant harness, **THEN** a registered subject fails that rule at its **headline** assertion — the `recorded_at` comparison, not the survival anchor — and its `Declared` row's `expect` entry pins the headline message, so a failure at an earlier anchor is reported as the wrong failure rather than counted as a pass. | `cargo test -p happenstance-testkit --all-features --test mutation_coverage mutants_fail_exactly_their_declared_rules`, with the new row's `expect` quoting a substring of `crates/happenstance-testkit/src/suite.rs:2517-2523` ("a `RecordedAt` is persisted alongside the event, not recomputed when"). Contrast row: `LosingFixture` (`mutation_coverage.rs:1192-1230`), whose pin for the same rule quotes the *second anchor* instead. |
| AC-002 | **GIVEN** the same adapter author re-running the harness on a fast machine and on a slow one, **WHEN** the control reopens, **THEN** the stamp it recomputes is a deterministic function of a per-fixture reopen generation — never re-spent from `correct::stamp`'s constant, never read from a clock — so the failure is identical run to run and can never be silently invisible or intermittently absent. | `mutants_fail_exactly_their_declared_rules` green across repeated runs; review that the new stamp is derived through `RecordedAt::from_millis` / `as_millis` (`crates/happenstance-core/src/identity.rs:160-172`) from `TEST_RECORDED_AT` plus a generation counter, and a search for `SystemTime`, `Instant` or `now()` under `crates/happenstance-testkit/tests/mutation_coverage` returns nothing new (CF-33). |
| AC-003 | **GIVEN** an evaluator judging in one sitting whether the three reopen rules are *independently* falsifiable, **WHEN** they read the registry, **THEN** the new row declares exactly one rule — `recorded_time_survives_a_reopen` — and the subject **passes** `acknowledged_writes_survive_a_reopen` and `reopened_store_does_not_reissue_an_event_id`, so what bites is the stamp sentence alone and not the survival sentences `LosingFixture` already covers. | `mutants_fail_exactly_their_declared_rules` (a subject that fails an undeclared rule is a failure of this test, not a bonus) and `every_rule_has_a_mutant`, both named at `xtask/src/proof.rs:84-96`. |
| AC-004 | **GIVEN** a maintainer who adds a fixture and forgets one of the lists it must appear in, **WHEN** the gate runs, **THEN** the control is present in all four places — the type in `mutation_coverage/mutants.rs`, the type in `for_each_mutant!`, the `Declared` row in `REGISTRY` with a real provenance sentence, and the `MODEL_COVERAGE` row — **and** the model doc comment's miss count and its "defects that are only visible across a reopen" bullet both name the second subject, so the file's own answer to *what is this test blind to* stays true rather than going stale in prose. | `mutant_registry_is_exhaustive`, `every_mutant_states_its_provenance` and `the_model_rule_rejects_exactly_what_it_claims` green under `--all-features`; review of `crates/happenstance-testkit/tests/mutation_coverage.rs:2276-2300` showing the count and the bullet updated in this diff. |
| AC-005 | **GIVEN** an evaluator who reads a `[PROVISIONAL]` marker to decide adopt-or-decline, **WHEN** they read ES-35, CF-17 and CF-14 after this PR, **THEN** each carries a written verdict against this adapter's evidence — ES-35's remaining falsifier restated as a store that loses a write to a *fault* rather than to an instruction, CF-17's rule shape confirmed against a real file-backed adapter, CF-14's deferral confirmed and narrowed to the two implementations that have not answered — every sentence that has become factually false is corrected, and **no marker changes level**; any promotion is recorded as a named ADR-queue item in this story's folder with its evidence attached, never made here. | Review against `spec/SPECIFICATION.md:4142-4180` (ES-35), `:7471-7500` (CF-14), `:7570-7596` (CF-17); the clause-status rows at `:8617`, `:8725`, `:8728` still read `PROVISIONAL` / `DEFERRED` / `PROVISIONAL`; `cargo xtask spec-trace` green; the escalation note exists under this story's backlog folder and is cited from `_ledger.md`. |
| AC-006 | **GIVEN** the application author who was told an acknowledged write survives a reopen, **WHEN** the SQLite conformance target is run *after* the control lands, **THEN** `recorded_time_survives_a_reopen` and `acknowledged_writes_survive_a_reopen` are green against `SqliteFixture` and the `MID_BATCH_FAULT`-gated rule is reported as `Skipped` carrying the fixture's stated reason — so the pass is a pass of a rule now demonstrably failable, and the fault far end stays visibly open rather than looking covered. | `cargo test -p happenstance-sqlite --test conformance` (the mount `sqlite-fixture-and-whole-suite` creates), reading the run's per-rule output for both `Ran` lines and the `Skipped{reason}` line; capability mechanism at `crates/happenstance-testkit/src/contract.rs`, decline shape at `crates/happenstance-testkit/src/fixtures.rs:275-284`. |
| AC-007 | **GIVEN** a reader who never opens a test file, **WHEN** this change merges, **THEN** `CHANGELOG.md` carries an entry naming the defect the new control encodes in CF-29's stated shape, and the story-grain gate is green with `spec-trace`'s citation count not fallen — so the change to what the suite can prove is discoverable from the record alone. | `cargo xtask affected --base {{base}}` and `cargo xtask lints && cargo xtask spec-trace` (`.redkiln/config.yaml:40,48`) both green; the `CHANGELOG.md` entry read against `spec/SPECIFICATION.md:8141-8165`. |

## Interaction quality

RFC §6.7/D6. **This story renders no surface**, and that is a signed-off fact, not
an omission: `_design.md` records `N/A — no user-facing surface` in every one of its
blocks (`## Surfaces`, `## Items`, `## Signatures`), approved at the
`/redkiln:plan` gate, and `initiative.md:196` puts any screen, UI or documentation
site outside the initiative. The **state** family (in-place vs context-jump,
non-occlusion, preserved focus/scroll/selection, reversibility, keyboard
reachability) and the **composition** family (placement, transience, density
budget, hierarchy) as *rendered* invariants therefore have no referent here, and
inventing a DOM to assert against would be exactly the decorative check
`CLAUDE.md` forbids.

What this story does have is a **diagnostic surface** — the text a failing run
prints, the row a reviewer reads, the clause an evaluator reads — and the project's
own briefs make several of these families' analogues blocking rather than advisory.
Each is carried by an AC row in the table above; none is carried by a bullet here.

| Family | The invariant, in this medium | Carried by |
| --- | --- | --- |
| **Presentation exists at all** | A failure is not a bare `assert_eq!` diff. The `Declared` row's `expect` entry names *which sentence* failed, so a reader learns which claim broke rather than that something did — the same reason `LosingFixture` pins three messages for three rules. | AC-001 |
| **Non-occlusion** | The right failure is not hidden behind an earlier one. A control that dies at the survival anchor occludes the stamp assertion completely — that is the phase-4 debt itself, and the `expect` pin is what turns the occlusion into an error instead of a green run. | AC-001, AC-003 |
| **Reversibility** | Nothing this PR writes is a one-way door. Verdicts are recorded at unchanged maturity levels and a promotion is escalated as a note; a marker moved in passing would be an irreversible edit to a record only a new ADR could undo. | AC-005 |
| **State preserved across the change** | The existing subjects keep their meaning. `GappedPositionStore` restamps on replay and passes today only because the binary's stamp is a constant; if the new control makes that variant go red, the variant is fixed at its **durable side** and the rule is not weakened (`variants.rs:96-104`). | AC-003, AC-004 |
| **Density budget, with its real numbers** | The model doc comment states a running count of what the model family misses. It is prose that goes stale silently and has already been wrong once; the count and the reopen bullet are updated in the same diff as the subject that changes them. | AC-004 |
| **Hierarchy** | A reader of a clause meets the verdict, then the falsifier, then the evidence — not a marker whose falsifier is blank. The initiative's DoD 12 is exactly this shape, and it reads whatever AC-005 writes. | AC-005 |
| **Named anti-patterns** | Three, from the project's own briefs rather than a design system: a mechanically-satisfying subject (`struct AlwaysWrong`, `mutants.rs:1-9`); a control registered in `mutants.rs` but absent from `for_each_mutant!`, which is a control nothing runs; and a rule relaxed to accommodate a new subject. | AC-001, AC-003, AC-004 |

## Error conditions

| id | condition | required behaviour |
| --- | --- | --- |
| EC-001 | The control dies at the survival anchor (its reopen loses events) instead of at the stamp comparison. | `mutants_fail_exactly_their_declared_rules` fails on the `expect` pin mismatch. Fix the control's durable side — it must restore payloads, types, tags, identities and positions — not the pin. |
| EC-002 | The control passes every rule, because its reopen replays through `correct::stamp` and re-spends `TEST_RECORDED_AT`. | The declared-rules test fails with a "declared to fail `recorded_time_survives_a_reopen` and passed it" shape. Derive the new stamp from a reopen generation instead (Context pack §2). |
| EC-003 | The control also fails `acknowledged_writes_survive_a_reopen` or `reopened_store_does_not_reissue_an_event_id` — it has become a second `LosingFixture`. | The same test fails on an undeclared rule. One defect, one rule (AC-003): restore everything except the stamp. |
| EC-004 | The type exists in `mutation_coverage/mutants.rs` and is absent from `for_each_mutant!`. | `mutant_registry_is_exhaustive` fails — this is the exact drift `mutants.rs:60-73` says the test exists to catch. Nothing else in the binary would notice. |
| EC-005 | The `MODEL_COVERAGE` row is missing, and local runs are green because `proptest` is off. | The model-coverage test is behind the `proptest` feature; run with `--all-features`, which is what `xtask/src/proof.rs`'s `cargo_args` already does. A green `cargo test -p happenstance-testkit` without the flag is not evidence for AC-004. |
| EC-006 | An existing conformant variant goes red because the binary's stamp is no longer invariant across a reopen. | Expected and pre-answered: give that variant's durable side `SequencedEvent`s (`variants.rs:96-104`, `fixture_instruments.rs:12-21`). Weakening `recorded_time_survives_a_reopen` is forbidden by Context pack §8. |
| EC-007 | A specification edit drops or renames a rule name a clause cites. | `cargo xtask spec-trace` fails, or its citation count falls. All three clauses cite `acknowledged_writes_survive_a_reopen`; restating a falsifier must not touch a `Rule:` line. |
| EC-008 | A maturity marker's level is changed in this PR (ES-35 or CF-17 promoted to `[FROZEN]`). | Rejected at review and at the ledger. Record the case as an ADR-queue item with this story's evidence; the level moves in an ADR pass, not here (`.kb/open-questions/es-7-and-vt-9-provisional-markers.md:14-15`). |
| EC-009 | CF-17's verdict turns out to reshape or add a conformance rule. | Then CF-1's registry row and CF-29's `CHANGELOG.md` entry land in the **same** change (`spec/SPECIFICATION.md:8141-8165`); `every_rule_has_a_mutant` fails until the row exists. Not a follow-up, and not a reason to defer the verdict. |
| EC-010 | Prose counts of the registry elsewhere in the tree go stale (`RUNBOOK.md:1762`'s "fifty mutants", `CHANGELOG.md:481`'s subject count). | Nothing fails: `registry_len()` is **printed, not asserted** (`xtask/src/proof.rs:267-290`), deliberately. Do not rewrite historical session logs; state the new count where this change's own record lands, and read the gate's printed count rather than trusting a quoted one. |
| EC-011 | The SQLite conformance target does not exist yet, or is red, when AC-006 is attempted. | This story's `depends_on` is unmet. Stop and report — AC-006 cannot be discharged against a mount `sqlite-fixture-and-whole-suite` has not landed, and a locally-stubbed fixture would be the doubled driver AC-T03 forbids. |

## Non-functional

| id | requirement | why, and where it is checked |
| --- | --- | --- |
| NF-001 | The control acquires **no** new dependency, no file and no crate. Its durable medium is an in-process collection in the shape `DurableFixture` uses. | The testkit's dependency graph is `happenstance-core` + `futures-core`, and architecture brief §1 forbids a `rusqlite`-backed fixture inside it. Checked by the diff touching no `Cargo.toml`. |
| NF-002 | No clock, no sleep, no timeout, no retry loop anywhere in the control or its registration. | CF-33 (`crates/happenstance-testkit/src/concurrency.rs:24-43`) and testing brief §4: a hang is a finding, not something a test papers over. A millisecond-resolution clock would also make AC-002's inequality a race. |
| NF-003 | The control stays `Rc`/`RefCell`-shaped and `UnwindSafe`, driven by the harness's `block_on` like every sibling subject. | `harness.rs`'s `Subject` / `run_subject` constraints, and the reason `mutation_coverage` exercises the `!Send` flavour ADR-0001 exists for (`mutation_coverage.rs`'s `REGISTRY` doc comment). This is also DR-08 of the story map — nothing added to the testkit costs the `!Send` flavour anything. |
| NF-004 | One added subject must not materially lengthen the `mutation_coverage` target. It runs every registered rule against every subject, so the cost is one column, not one cell. | Observed in the gate's own output; `cargo xtask ci --fast` remains this project's integration-grain command (`.redkiln/config.yaml:55`). |
| NF-005 | `cargo xtask spec-trace`'s citation count does not fall across the specification edits. | `reachability_static` (`.redkiln/config.yaml:48`); it is also the mechanism `spec-and-code-reconciliation` re-checks at AC-016. |
| NF-006 | Every rustdoc obligation the constitution places on a new item is met: the control's doc comment states the one-sentence reason it is longhand rather than a `Defect` step, and its provenance sentence names a defect somebody would ship. | `standards/rust/70-rustdoc-obligations.md`; CF-4's bar as restated in `mutants.rs:1-9`; `every_mutant_states_its_provenance`. |

## Implementation notes (non-prescriptive)

Shape suggestions, not instructions. Where a brief already decided, the decision is
in the Context pack above and is not restated here as a suggestion.

- **Sit beside `LosingFixture`, not inside it.** The two are opposites, and reading
  them together is what makes each legible: `LosingFixture` loses everything, the
  new one loses exactly one field. Placing the new type adjacent in
  `mutation_coverage/mutants.rs` and cross-referencing both doc comments costs
  nothing and is the cheapest thing a future reader can be given.
- **A generation counter on the fixture, not on the store.** `reopen()` is a fixture
  operation, so the counter that makes AC-002's stamp monotone naturally lives where
  the reopen does; a connected store reads it when it replays. Whether that is a
  `Cell<u64>` on the fixture or a shared handle passed to each store is the
  implementer's call.
- **Name it for what it does.** The sibling names describe the defect
  (`LosingFixture`, `CachedHeadFixture`, `SharedBackingFixture`). Something in the
  shape of `RestampingFixture` reads correctly in the `MODEL_COVERAGE` table and in
  a failure line without further explanation.
- **Write the specification edits last.** The verdicts in Context pack §6 are
  *recommended*, and the recommendation rests on evidence that only exists once
  AC-006 has been observed. Writing the clause text before running the adapter's
  reopen rules would be recording a prediction as a verdict.
- **Keep each clause edit inside its own paragraph.** ES-35's and CF-17's false
  sentences are single sentences inside larger paragraphs whose surrounding text is
  still exactly right; a wholesale rewrite would make `spec-trace`'s diff unreadable
  and would risk EC-007 for no gain.
- **The ADR-queue escalation is a note, not a record.** One short file in this
  story's own backlog folder naming the clause, the evidence, and what an ADR would
  have to decide. Authoring an ADR here is out of scope by the PR boundary and by
  `CLAUDE.md`'s two-places rule.

## Tests and CI (merge gate)

Tiers as the project's testing brief §1 defines them, with the commands its §6 pins.
No tier here is invented for this story, and no family this story does not touch is
re-run for decoration.

| tier | command / path | proves |
| --- | --- | --- |
| Static | `cargo xtask lints && cargo xtask spec-trace` (`reachability_static`, `.redkiln/config.yaml:48`) | The specification's citations still resolve after the three clause edits and the count has not fallen — AC-005, AC-007, NF-005. Also the `CHANGELOG.md` lint behind CF-29. |
| Static | `cargo xtask affected --base {{base}}` (`affected_gate`, `.redkiln/config.yaml:40`) | The story grain: `happenstance-testkit` and everything downstream of it still builds and passes — AC-007. This is the command `redkiln verify --grain story` runs whether or not anyone types it. |
| Mutation coverage (the check on the checker) | `cargo test -p happenstance-testkit --all-features --test mutation_coverage` — `mutants_fail_exactly_their_declared_rules`, `mutant_registry_is_exhaustive`, `every_rule_has_a_mutant`, `every_mutant_states_its_provenance`, `conformant_variants_pass_everything`, `capability_skips_are_reported`, `the_model_rule_rejects_exactly_what_it_claims` (`xtask/src/proof.rs:84-96`) | The whole of AC-001 – AC-004, and EC-001 – EC-006. `--all-features` is not optional: the model-coverage test is behind `proptest` and is silently absent without it. |
| Conformance (this project's real proof) | `cargo test -p happenstance-sqlite --test conformance` — the mount `sqlite-fixture-and-whole-suite` creates | AC-006: the adapter's two reopen rules `Ran` and green against a real file through a real reopen, and the fault-gated rule `Skipped` with its stated reason. Read the run's per-rule output, not just the exit code. |
| Review (a decision, not a behaviour) | Reading `spec/SPECIFICATION.md:4142-4180`, `:7471-7500`, `:7570-7596` and the status rows at `:8617`, `:8725`, `:8728`; reading the ADR-queue note in this story's folder | AC-005. Testing brief §2 names AC-010's CF-14 half explicitly as "a reviewed paragraph, not a runnable test", listed there so it is not silently skipped. The clause-status rows are the mechanical half. |
| Integration grain (this project's, not this story's) | `cargo xtask ci --fast` (`integration_scoped`, `.redkiln/config.yaml:55`) | Run at the slice's close. It is a *precondition* for reading the criteria above, never a substitute for them (`RUNBOOK.md:38-42`, restated at `project.md` DoD 4). |
| Ledger | `.bklg/from-contract-to-published-library/sqlite-durable-store/reopen-negative-control-and-durability-verdicts/_ledger.md` (`require_ledger: true`, `.redkiln/config.yaml:67`) | AC-T05: every row flipped only with cited evidence — a `file:line` and/or a test id. AC-T06 additionally requires the ledger to state **which** of the two negative-control options was used; this story chose the permanent registry row, which testing brief §5 prefers. |

## Risks and coupling (PR-scoped)

- **The control passes and nobody notices.** The highest-probability failure is
  EC-002 — a restamp that lands on the same constant. It is not caught by reading
  the code; it is caught by the declared-rules test, and only because the row
  declares a failure. Mitigation: write the `Declared` row and watch it go red
  **before** the control is finished, then make it red at the right assertion. A row
  added after a green run proves nothing.
- **Coupling to `sqlite-fixture-and-whole-suite` is real but one-directional.**
  AC-001 – AC-005 and AC-007 need nothing from the dependency; only AC-006 does. If
  the dependency slips, six of the seven criteria can still be discharged and AC-006
  blocks — but the story must not be reported complete with AC-006 unflipped, and
  must not substitute a locally-built fixture (EC-011, AC-T03).
- **Slice-mate collision in one file.** `model-family-and-mutant-pass-column` also
  edits `mutation_coverage.rs`'s `REGISTRY` and `MODEL_COVERAGE`, for a different
  row. The story map sequences that story after `concurrency-family-and-contender-count`
  and puts this one in parallel with either (`_storymap.md`, *Merge order* 3), so a
  textual conflict in those two tables is expected. It is a merge conflict, not a
  design conflict: two independent rows. Rebase, do not renumber, and re-run
  `mutant_registry_is_exhaustive` after the merge, because it is the only thing that
  checks the two lists still agree.
- **The verdicts are the part most likely to overreach.** Three clauses whose markers
  a reader would like resolved, and a real adapter finally in the tree, is precisely
  the situation in which a level gets moved "since we are here". EC-008 and AC-005
  exist for that moment. The failure is silent, and only an ADR could undo it.
- **Downstream readers of this story's output.** `spec-and-code-reconciliation`
  re-reads whatever AC-005 writes, `instrument-markers-removed-and-gate-green`
  depends on this story landing, and the initiative's DoD 12 audit in
  `publication-and-positioning` reads the same clause text. A falsifier written
  loosely here becomes three stories' problem later.
- **What this PR cannot break.** No `src/` of any shipped crate changes, no public
  API moves, and no schema is touched. The blast radius is the testkit's own test
  targets, the specification and `CHANGELOG.md` — which is why the story grain is
  `cargo xtask affected` rather than the whole gate.

## Dependencies

**Blocks on** (must be merged first):

- `sqlite-fixture-and-whole-suite` — supplies `SqliteFixture` and the
  `crates/happenstance-sqlite/tests/conformance.rs` mount, its `REOPEN` and
  `SECOND_HANDLE` declarations, its real ceilings and its `MID_BATCH_FAULT`
  decline. **AC-006 alone** consumes it; the other six criteria do not. This is the
  story's only `depends_on` edge (`_storymap.md`, Slices table).

**Unlocks** (these declare this story in their own `depends_on`):

- `instrument-markers-removed-and-gate-green` — takes `cargo xtask ci --fast` green
  across the whole crate, which includes this story's registry and clause edits.
- `spec-and-code-reconciliation` — reads every clause this story rewrites and checks
  `spec-trace`'s citation count has not fallen.

**Parallel, same slice, no ordering edge:**
`concurrency-family-and-contender-count` and `model-family-and-mutant-pass-column`
(`race-model-and-durability`). They share the two registry tables and nothing else;
see *Risks and coupling*.

**Not a dependency, deliberately:** `adr-0022-append-condition-strategy`. Its
verdicts are about the append path; this story's three clauses are about durability
and reopen, and none of them cites ADR-0022.

## Anchors (progressive disclosure)

Everything the Context pack distilled, kept behind a link. Open each at the moment
named — not before, and not instead of the Context pack.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `crates/happenstance-testkit/src/suite.rs` (`:2473-2523`) | The rule under test, its three assertions in order, and the exact headline message the `expect` pin must quote. Its doc comment already names phase 8 as where the missing control arrives, and states the instrument obligation a `REOPEN` fixture carries. | First, before writing anything — the pin string is copied from here. | AC-001 |
| `crates/happenstance-testkit/tests/mutation_coverage.rs` (`:324`, `:1192-1230`, `:2100-2148`, `:2276-2435`, `:2702-2790`) | The mount. `REGISTRY`, `LosingFixture`'s row as the pin-shape exemplar, `for_each_mutant!`, `MODEL_COVERAGE` with the doc comment whose arithmetic changes, and the meta-tests that observe all of it. | While registering — all four places live here or are enforced from here. | AC-001, AC-003, AC-004 |
| `crates/happenstance-testkit/tests/mutation_coverage/mutants.rs` (`:1-9`, `:29-43`, `:60-73`, `:96-130`) | CF-4's "a defect somebody would ship" bar; the module doc's list of longhand exceptions and why `LosingFixture` is one; the three-place registration procedure and why steps 2 and 3 are separate; the `Defect` trait the control deliberately does not use. | Before choosing between a `Defect` step and a longhand store — the answer is here, and it is not a preference. | AC-001, AC-004 |
| `crates/happenstance-testkit/tests/mutation_coverage/correct.rs` (`:348-358`) | `TEST_RECORDED_AT` and `correct::stamp` — the constant a naive replay re-spends, which is why a naive restamp is invisible in this binary. | Before writing the reopen path of the control. | AC-002 |
| `crates/happenstance-testkit/tests/mutation_coverage/variants.rs` (`:85-104`) | `GappedPositionStore`'s doc comment, which already records that it restamps on replay, passes anyway, and what the answer is the day the stamp varies — "give the durable side `SequencedEvent`s rather than weaken the rule." | Immediately if a conformant variant goes red (EC-006), and before that as the reason AC-002 is worded as it is. | AC-002, AC-003 |
| `crates/happenstance-testkit/tests/mutation_coverage/harness.rs` | `Subject`, `run_subject` and the `UnwindSafe` / single-threaded `block_on` constraints every store in this binary must satisfy. | When the new type does not compile into `for_each_mutant!`. | AC-003 |
| `crates/happenstance-testkit/tests/fixture_instruments.rs` (`:12-21`, `:57-90`) | `DurableFixture` — the reference shape for a durable side that carries `SequencedEvent` rather than `Event`, and the fixture named in CF-17's now-false "only fixture in the workspace" sentence. | While shaping the control's durable medium, and again while correcting CF-17. | AC-002, AC-005 |
| `crates/happenstance-core/src/identity.rs` (`:160-172`) | `RecordedAt::from_millis` / `as_millis` — the only arithmetic seam the generation-derived stamp needs. | While writing the stamp. | AC-002 |
| `spec/SPECIFICATION.md` (`:4142-4180` ES-35, `:7471-7500` CF-14, `:7570-7596` CF-17, status rows `:8617`, `:8725`, `:8728`, CF-29 at `:8141-8165`) | The three clauses to be edited, their exact marker text, the false sentences named in the Context pack, the rows `spec-trace` reads, and CF-29's changelog obligation. | After AC-006 is observed — the verdicts are written against evidence, not before it. | AC-005, AC-007 |
| `.kb/open-questions/es-7-and-vt-9-provisional-markers.md` (`:14-15`) | The standing rule in one sentence: "Moving a maturity marker is an ADR's act, so both are recorded rather than moved." The authority behind AC-005's hardest constraint. | Before touching any `[PROVISIONAL]` / `[DEFERRED]` marker. | AC-005 |
| `xtask/src/proof.rs` (`:84-96`, `:133-150`, `:267-290`) | The seven meta-tests by name, the `--all-features` invocation that makes the model test visible, and `registry_len()`'s doc comment explaining why the subject count is printed and never pinned. | When deciding what to run for AC-004, and when a stale prose count tempts an edit (EC-010). | AC-004, AC-007 |
| `.bklg/from-contract-to-published-library/sqlite-durable-store/_decomposition.md` (architecture §7, §9, §10; testing §2, §5, §6) | The `recorded_at`-read-back precondition the control mirrors; the tensions recorded not settled; the testkit blast radius and the one branch where a rule is added; the AC-to-tier matrix; AC-T06's negative-control obligation and its stated preference for a permanent registry row; the merge-gate commands. | Testing §5 before starting; architecture §9 before writing the verdicts. | AC-005, AC-006, AC-007 |
| `.bklg/from-contract-to-published-library/sqlite-durable-store/_design.md` | The signed-off statement that this project has no user-facing surface, in every block. It is what makes the *Interaction quality* section's state and composition families N/A rather than skipped. | Only if someone proposes rendering something. | AC-005 |
| `.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md` | The four personas and the journeys the acceptance criteria are framed from — *Learn when you are finished* and *Decide in one sitting* in particular. | When an AC's framing needs a check against the audience it claims. | AC-001, AC-003, AC-006 |
| `RUNBOOK.md` (`:3205-3207`, `:605`, `:4166-4237`, `:1762`) | The recorded debt this story pays, in the plan of record's own words; phase 8's ownership of CF-17 / CF-34 / ES-35; the phase plan; and at `:1762` a quoted subject count, as the example of the prose EC-010 says not to chase. | At the start, to see the obligation stated by the runbook rather than by this spec. | AC-001, AC-005 |
| `.redkiln/config.yaml` (`:40`, `:48`, `:55`, `:67`) | The four commands and the ledger requirement, wired rather than typed — `affected_gate`, `reachability_static`, `integration_scoped`, `require_ledger`. | Before running anything, and before writing the ledger. | AC-007 |

## Clarifications resolved during spec

1. **The AC set is exactly the front half's seven.** AC-001 – AC-007 as enumerated
   above; none added, none dropped. `_ledger.md` carries the same seven ids.
2. **Project AC-004 is split three ways and this story owns the third.**
   `_storymap.md`'s Coverage table assigns "the schema persists the `StoreId` and
   reads `recorded_at` back" to `schema-migration-and-identity`, "the fixture
   declares `REOPEN` and the rules pass" to `sqlite-fixture-and-whole-suite`, and
   "proves the rule **can** fail" here. AC-006 above re-*observes* the second half
   rather than owning it — the observation is what makes this story's own claim
   meaningful, and its ledger evidence cites the dependency's target, not a new one.
3. **The negative control is the permanent registry row, not a reverted local
   mutation.** Testing brief §5 permits either and states the preference: "a
   permanent registry row is evidence a reviewer can re-run forever, while a
   reverted local mutation is not." Recorded here so the ledger's AC-T06 statement
   has a decided answer rather than an implementer's coin-flip.
4. **The three verdicts are recommended, not mandated.** Context pack §6 states each
   recommendation with its evidence, and AC-005 requires *a written verdict against
   this adapter's evidence at an unchanged level* — not specifically the recommended
   one. An implementer who reads the evidence differently writes a different verdict
   and says why. What is not open is moving a level (EC-008).
5. **The mutant subject count is not a criterion.** `registry_len()` is printed by
   the gate and deliberately not asserted (`xtask/src/proof.rs:267-290`), and
   `CLAUDE.md` forbids quoting a pass rate over the mutant set because the
   denominator is a choice. EC-010 records what to do about the stale prose counts
   elsewhere in the tree: nothing, except where this change's own record lands.
6. **Interaction quality is answered in the medium this story has.** The rendered
   state and composition families are N/A on the signed-off `_design.md`; rather than
   dropping the section, the seven analogues that *are* blocking — presentation of a
   failure, non-occlusion of the right assertion, reversibility, preserved meaning of
   existing subjects, the density budget's real numbers, hierarchy in a clause, and
   three named anti-patterns — are each mapped to an AC row that already exists. No
   invariant is carried by a bullet in that section.
7. **`crates/happenstance-sqlite/tests/conformance.rs` does not exist in the tree
   yet.** It is created by `sqlite-fixture-and-whole-suite`, so it is named in the
   Integration contract as a future mount rather than cited in the anchors table,
   every row of which resolves against the tree today. EC-011 says what to do if it
   is still absent when AC-006 is attempted.
