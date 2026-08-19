---
item: HS-S0115
stage: spec
created: 2026-08-12T13:47:56.007Z
updated: 2026-08-12T13:47:56.007Z
template_sig: 87bbf1d0
rendered_sig: b37da497
---

# Spec — CF-27's experiment run, and the pass list recorded as data

## Scope lock

| | |
| --- | --- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` — BR-11 (`:294`), AC-14 (`:347-349`), **DoD 15** (`:402-404`) |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` — the DAG, the scope seams, gate decision 4 (`:239-249`) |
| Project | `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/project.md` — **AC-002** (`:198-201`), DR-2 (`:147-149`) |
| This spec | `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/cf-27-experiment-and-recorded-pass-list/spec.md` |
| Key briefs | `…/retention-and-incomplete-logs/_decomposition.md` — architecture **DA-1** (`:150-171`), **DA-2** (`:173-232`), **DA-3** (`:234-274`), composition roots 1–2 (`:92-106`); testing brief AC-002/AC-003 rows (`:604-605`), AC-T04 (`:704-706`) |
| Signed-off design | `…/retention-and-incomplete-logs/_design.md` — **no user-facing surface**, approved 2026-08-12 (`:38-48`, `:86-95`). This story renders none and adds none |
| Story map row | `…/retention-and-incomplete-logs/_storymap.md:69` (this story), `:130-132` (merge order inside the slice) |
| This story's own discover | `…/cf-27-experiment-and-recorded-pass-list/discover.md` — the signal ledger, the deferred artefact-form question (`:35`), and the wrong implementation (`:44-51`) |
| Roadmap pointer | `RUNBOOK.md:165` (phase 14 status row), `RUNBOOK.md:4626-4674` (phase 14 in full), `RUNBOOK.md:307` (ADR-0028's queue row) |

## One-line PR slice

Run CF-27's experiment: the full conformance suite against the completeness instrument in **both**
the suffix and the scattered configuration, with the enumerated list of rule names that pass
committed per configuration as recorded data — stated as what it is, the set of rules that cannot
tell a pruned store from a young one — plus the DA-3 finding on which of the three outcomes holds,
and confirmation that no rule in the recorded failure set (if any) fails for a seeding reason.

## Executive summary

HS-S0114 lands the instrument and its `event_store_conformance!` mount. This PR **runs it and writes
down what happened**, in a form ADR-0028 can cite and a later reader can re-derive.

The delta over HS-S0114 is three things, and none of them is a store:

1. **A second mount.** HS-S0114 proves the instrument is conformance-reachable; this PR guarantees
   *both* of DA-1's configurations are actually mounted and run — a suffix (the prune case) and a
   scattered hole with survivors below it (the regulated-purge case). One suffix run structurally
   cannot falsify a floor, which is the outcome ES-39's `Rejects:` names as the path of least
   resistance (`spec/SPECIFICATION.md:4336-4341`, `:4347-4349`).
2. **A census, three-valued.** A harness in the same target drives every rule the registry knows
   about against each configuration *and against `MemoryFixture` as the young-store control*, and
   turns each result into one of `PASS` / `SKIP(capability, reason)` / `FAIL(message, origin)`. The
   suite's existing harnesses cannot produce this: a rule that returned
   `RuleOutcome::Skipped` is a **passing `#[test]`** carrying a `SKIP` line on suppressed stdout
   (`crates/happenstance-testkit/src/contract.rs:471-484`, `:517-521`), so "green" and "observed
   nothing" are the same colour, and a two-valued pass list would silently count skips as evidence.
3. **The recorded artefact and the finding.** The censuses land under `experiments/` as committed
   data with the command that regenerates them, and the writeup states which of DA-3's three
   outcomes holds with the other two named — a finding, deliberately not a conclusion, because
   ADR-0028 (HS-S0121) is where the conclusion is owed.

This PR writes **no conformance rule, no registry line, no mutant and no production code**. That is
the sequencing the architecture brief makes binding: *"a rule written first will be written to the
answer someone expected"* (`_decomposition.md:543-546`, `_storymap.md:18-25`).

## Context pack

Everything below is a decision this story must honour. Depth is behind the anchors; nothing here
needs another file to be actionable.

**CF-27 is an experiment, and its stated outcome is a list.** The clause's own words:
*"the outcome that matters is the list of rules that pass, which is the list of rules that cannot
tell a pruned store from a young one"* (`spec/SPECIFICATION.md:8036-8041`). Use that wording in the
artefact. A summary — "the suite passed apart from a couple of rules" — is DR-2's named defect
(`project.md:147-149`) and cannot be diffed, re-derived or cited.

**The specification predicts the answer this project's AC-003 says it does not want, and that
collision is the finding.** CF-27's `Rejects:` predicts a pruned store *"passes every rule
unchanged, including `query_all_matches_every_event`, whose contract is store-relative by wording
and therefore accidentally correct"* (`:8049-8055`). §3.7 says the same thing
(`:4260-4272`). **A pass list reading "all of them" is CF-27's predicted result and the strongest
available evidence for ES-39 — it is a success of this story, not a failure of it.** Nothing in this
PR may re-tune the instrument until two rules go red; DA-3 names that as the failure mode the
decision exists to prevent (`_decomposition.md:272-274`).

**AC-003's two failures are owed by the lying store, not the honest one.** They come from the two
registered mutants HS-S0116 and HS-S0117 already require (DA-3(2), `_decomposition.md:262-267`).
This story therefore does not need — and must not manufacture — a red rule.

**The two dishonest resolutions, named so they are recognisable** (DA-3, `:241-253`). *Pre-seeding*:
a `connect()` that hands back a store already holding history fails
`reading_an_empty_store_yields_nothing`, `head_of_an_empty_store_is_none`,
`condition_against_an_empty_store_admits_the_append` and
`two_fixture_instances_observe_none_of_each_others_appends` — four red rules, AC-003 satisfied by
the letter, and not one of them about forgetting. *An eager sliding window*: dozens of red rules for
the boring reason that a rule writes three events and reads them back. Either destroys AC-002
entirely, because **the pass list is evidence only if every failure excluded from it is about
completeness**. This story owns the audit that says so, and the audit is mechanical, not editorial:
see the control run below.

**The control is what makes the list mean what CF-27 says it means.** "Cannot tell a pruned store
from a *young* one" is a comparison, so the census runs against `happenstance_testkit::fixtures::MemoryFixture`
(`crates/happenstance-testkit/src/fixtures.rs:243`, `:270-292`) in the same binary and the same run.
A rule that fails in the control fails for a harness or seeding reason and is disqualified from
either list; a rule that skips in the control must skip in the instrument for the *same declared
capability and the same stated reason* (`MemoryFixture` declines `REOPEN` with its real reason,
`:279-283`) or the difference is itself a finding. This is `mutants_fail_exactly_their_declared_rules`'
discipline reused: a skip is *"neither a pass nor a failure — it is a hole in the map"*
(`crates/happenstance-testkit/tests/mutation_coverage.rs:2939-2958`).

**The census is derived from the registry, never transcribed.**
`for_each_event_store_rule!` (`crates/happenstance-testkit/src/registry.rs:94`) is the only place the
rule set is written down, and `__emit_rule_names` (`:293`) is how a list of names is obtained from
it. There is to be **no second list of rule names anywhere in this PR** — that is the invariant
`no_orphan_rules` (`:410`) exists to protect one level up, and it is what keeps the recorded list
honest when the rule set moves.

**The recorded list is data, not an assertion.** The testing brief is explicit: *"the recorded pass
list as a committed artifact, **not a test assertion**"* (`_decomposition.md:604`), and discover
gives the reason (`discover.md:26`): a golden `assert_eq!` against the census goes red on every
unrelated rule addition and teaches the next author to edit the evidence until the gate is green.
So the artefact is regenerated by a committed command and reviewed by a human; the only thing the
gate asserts about it is that the census is *complete* — every registered rule appears exactly once
with exactly one verdict — which is a check the census can genuinely fail if it is ever built from
anything but the macro.

**It goes stale on purpose, and says so.** HS-S0116 and HS-S0117 each add a rule to
`for_each_event_store_rule!` and change the census's denominator (`_storymap.md:70-71`,
`discover.md:36`). The artefact therefore carries the rule count it was produced against, so a later
reader can tell a **stale** list from a **wrong** one, and each of those two stories re-runs the
command rather than hand-editing the file.

**The instrument is an instrument first and a target never.** Nothing here may make it easier to
reach: no `pub use`, no promotion out of `crates/happenstance-testkit/tests/`, no convenience export
so the experiment can be driven from elsewhere (`project.md:142-146`, `:244-247`; `discover.md:51`).
A reproducible experiment does not require a published fixture.

**No surface change, and here it is trivially true.** This PR runs code and writes markdown and text
(gate decision 4, `../../_decomposition.md:239-249`). Its real contribution to AC-011 is evidential:
the pass list is what tells HS-S0124 whether the honest answer needs a port surface at all. Any such
conclusion is a **finding fed to DA-7's option table** (`_decomposition.md:363-379`) and never a
change made here.

**Persona-journey slice.** The reader this serves is the adapter author and the maintainer standing
where DoD 15 is judged (`initiative.md:402-404`): *a store that holds only a suffix of its own log is
exercised, and the outcome is on disk*. What they get from this PR is the enumerated proof that
"fails loudly" is not expressible in today's vocabulary — every name on the pass list is a rule the
suite runs and cannot use to tell the two stores apart (`discover.md:33`). Not an argument. A list.

## Integration contract

- **Archetype**: `capability` — the user-observable slice is the recorded evidence a reader can open,
  re-derive and cite; the substrate it runs on is HS-S0114's.
- **Slice / milestone**: `forgetting-instrument`. Slice-mates, implemented in one context:
  `retained-set-instrument-and-conformance-mount` (HS-S0114, the dependency — the instrument, its
  `Fixture`, its retained predicate and its first mount).
- **Mount point**: **`crates/happenstance-testkit/tests/completeness_instrument.rs`** — the test
  target HS-S0114 creates, in the shape of
  `crates/happenstance-testkit/tests/fixture_instruments.rs:201-205`. Both
  `event_store_conformance!` invocations and the census `#[test]` live in that target. *If the
  sibling landed the target under a different file name, this story mounts into **that** file* — it
  does not create a second target for the same instrument, because two targets holding one
  instrument is how the two configurations drift apart.
- **Wires into**:
  - `happenstance_testkit::event_store_conformance!` (`crates/happenstance-testkit/src/lib.rs:346-353`)
    — one invocation per configuration, distinct `mod_name`, `fixture = <instrument fixture>`.
  - `happenstance_testkit::for_each_event_store_rule!` (`crates/happenstance-testkit/src/registry.rs:94`)
    and `__emit_rule_names` (`:293`) — the census's only source of rule names.
  - `happenstance_testkit::rules::*` + `happenstance_testkit::block_on` (`crates/happenstance-testkit/src/registry.rs:330`) — how a
    rule is driven by name without a runtime, exactly as `probes` does
    (`crates/happenstance-testkit/tests/mutation_coverage/harness.rs:352-370`).
  - `happenstance_testkit::{RuleOutcome, Capability, Fixture}` (`crates/happenstance-testkit/src/contract.rs:471-484`)
    — the two-variant outcome that `catch_unwind` turns into three verdicts.
  - `happenstance_testkit::fixtures::MemoryFixture` (`crates/happenstance-testkit/src/fixtures.rs:243`)
    — the young-store control.
- **Renders surfaces**: **none.** `_design.md` records no user-facing surface for this project and
  the sign-off is on that determination (`_design.md:38-48`, `:86-95`). This story adds no `pub`
  item to any crate.
- **Public items**: none. `_design.md`'s `## Items` is `N/A`; this PR's only new items are private to
  a `tests/` target.
- **Conformance rule(s)**: **none written, and that is the point.** This story is not
  adapter-observable: it *runs* the registered rule set and records the outcome. The diff must add no
  name to `for_each_event_store_rule!`; CF-27's own rule
  (`suffix_store_is_distinguishable_from_a_young_store`) is HS-S0122's, and writing it here would
  fail `spec-trace` check 6 semantics for the wrong reason and pre-decide ADR-0028
  (`_decomposition.md:479-482`).
- **Clause(s)**: **CF-27** (`spec/SPECIFICATION.md:8034-8060`) — this PR *executes* it and leaves its
  `[DEFERRED]` marker alone; the marker move is HS-S0123's. Evidence is produced about **ES-38**
  (`:4299-4323`, `[FROZEN]`), **ES-39** (`:4325-4349`, `[DEFERRED]`) and **CF-25/CF-26**
  (`:8003-8032`, `[FROZEN]`) without editing any of them. No `[FROZEN]` clause is touched, so no new
  ADR is owed by this story.
- **Advances DoD scenario**: **DoD 15** — *"Incomplete logs have an answer on disk … a store that
  holds only a suffix of its own log is exercised against a reader"* (`initiative.md:402-404`). This
  PR lands the *exercised* half's evidence; the reader half is HS-S0118/HS-S0119 and the answer is
  HS-S0121's.

## PR boundary

```
crates/happenstance-testkit/tests/**
experiments/completeness-pass-list/**
.bklg/from-contract-to-published-library/retention-and-incomplete-logs/cf-27-experiment-and-recorded-pass-list/**
```

**In this PR**

- The second `event_store_conformance!` invocation, so both DA-1 configurations are mounted and run
  (and the first, if the sibling's target arrives with only one).
- The census harness, private to that target: three verdicts, `catch_unwind`, probes built from
  `for_each_event_store_rule!`, one run per configuration plus the `MemoryFixture` control.
- One completeness `#[test]` over the census — every registered rule appears exactly once with
  exactly one verdict — which is the only thing the gate asserts about it.
- `experiments/completeness-pass-list/` — `README.md` (the finding, in CF-27's wording),
  `results/*.txt` (the three censuses as committed text), `run.sh` (the regeneration command), in the
  shape of `experiments/position-visibility/` (`README.md:23-26`, `run.sh:1-13`).
- This story's own backlog folder: the `_ledger.md` the second pass authors.

**Explicitly not in this PR**

- Any conformance rule body, any `for_each_event_store_rule!` line, any `REGISTRY` row or `Defect` —
  HS-S0116 (ES-38) and HS-S0117 (ES-40).
- Any change to the instrument's *semantics*. If the run exposes a decorator bug — DA-2's named trap,
  `ReadOptions` applied above the filter so `limit` returns short pages
  (`_decomposition.md:226-232`) — that is a fix to HS-S0114's code inside this slice's shared
  context, and the census is re-run; it is never a reason to relax what the census records.
- Any `.kb/` atom, any `spec/SPECIFICATION.md` edit, any marker move — HS-S0121, HS-S0123.
- Any reader run (`read_decision_model`, the projection runner, `IngestStore::holds`) — HS-S0118,
  HS-S0119.
- Any surface change anywhere, and any escalation *decision*: a "this needs a port surface"
  conclusion is recorded as a finding in the writeup and consumed by HS-S0124 (`project.md:240-243`).
- Any `pub` export, re-export or manifest change that would make the instrument reachable outside its
  test target.

**Merge DoD**: both configurations run under `event_store_conformance!`, the three censuses are
committed and regenerable by the recorded command, the DA-3 finding is stated with the other two
outcomes named, and `cargo xtask affected --base main` (`.redkiln/config.yaml:40`) is green.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **Both DA-1 configurations are mounted and run** | Two `event_store_conformance!` invocations in the instrument's target, distinct `mod_name`s (e.g. `suffix_conformance`, `scattered_conformance`), each over the same instrument parameterised by its retained predicate: a **suffix** (survivors are a contiguous tail; the device-prune case) and a **scattered** hole with survivors *below* it (the regulated-purge case, E2E-46/E2E-47). A suffix-only run makes `earliest_position()` look correct, which is exactly what ES-39 argues against | `.../_decomposition.md:150-171`; `spec/SPECIFICATION.md:4336-4341`; mount shape at `crates/happenstance-testkit/tests/fixture_instruments.rs:201-205` |
| **The census is three-valued, never two** | `PASS` (rule ran, every assertion held), `SKIP` carrying the fixture's declined `(capability, reason)` pair verbatim, `FAIL` carrying the panic message and its origin. `RuleOutcome` has no `Failed` variant — a failing rule **panics** — so the census wraps each probe in `std::panic::catch_unwind` and installs a quiet panic hook, the same mechanism and for the same stated reason as the mutation harness | `crates/happenstance-testkit/src/contract.rs:471-484`; `crates/happenstance-testkit/tests/mutation_coverage/harness.rs:96-133`, `:449-470` |
| **Rule names come only from the registry** | Probes are built by invoking `for_each_event_store_rule!` with a local emitter macro that mentions the fixture type parameter (hygiene applies to locals, not type parameters). No hand-written rule-name list appears in this PR, in the census, or in the recorded files' *generation* path | `crates/happenstance-testkit/src/registry.rs:94`, `:293`, `:410-425`; `crates/happenstance-testkit/tests/mutation_coverage/harness.rs:346-370` |
| **The control run** | The identical census runs against `MemoryFixture` in the same binary. The pass list is the **differential**: rules that `PASS` on both the control and a configuration are the answer CF-27 asks for. A `FAIL` in the control disqualifies that rule from every list and is reported as a harness fault. A `SKIP` must match the control's `(capability, reason)` exactly, or the difference is recorded as a finding rather than absorbed | `spec/SPECIFICATION.md:8036-8041`; `crates/happenstance-testkit/src/fixtures.rs:270-292`; skip-is-a-hole discipline at `crates/happenstance-testkit/tests/mutation_coverage.rs:2941-2956` |
| **The seeding audit is mechanical** | The instrument's `Fixture` presents an **empty** store at `connect()` like every shipped fixture, so the four seeding-sensitive rules DA-3 names appear as `PASS` in every census; any `FAIL` is then traced to the retained predicate and stated. This is the AC-A04 confirmation, and it is part of the artefact rather than a reviewer's inference | `.../_decomposition.md:241-253`, `:512-513`; `crates/happenstance-testkit/src/fixtures.rs:270-292` |
| **The gate asserts completeness, not content** | One `#[test]`: every name from `__emit_rule_names` appears exactly once in each census with exactly one verdict. It fails if the census is ever built from a second list or a probe is silently dropped. It does **not** assert which rules pass — a golden comparison would go red on every unrelated rule addition and train the next author to edit the evidence | `.../_decomposition.md:604`; `discover.md:26`; precedent `crates/happenstance-testkit/src/registry.rs:410-435` |
| **The record is regenerable, and stamped** | `run.sh` holds the one command that regenerates the three `results/*.txt` files (the census printed to stdout under `--nocapture`, redirected). Each file's header carries the configuration, the retained predicate in words, and the **rule count** it was produced against, so a later reader distinguishes a stale list from a wrong one. The census is built from `RuleOutcome` values, never by parsing libtest output, and reads no clock — repeated runs on one commit are byte-identical | `CLAUDE.md` (`experiments/` — measurements, reproducible, not in the gate); `experiments/position-visibility/run.sh:1-13`; `discover.md:36`; no-watchdog reasoning at `crates/happenstance-testkit/tests/mutation_coverage/harness.rs:31-35` |
| **The artefact's home is `experiments/`** | `experiments/completeness-pass-list/{README.md,results/,run.sh}`. It is not a crate, not a workspace member, not a gate step and adds no dependency to any `Cargo.toml`. *Alternatives that lost*: the story's own backlog folder (it is archived at closeout, so ADR-0028 would cite a path that moves — `CLAUDE.md`, `.bklg/` is work in motion); `references/` (right for a transcript kept only for citation, like `references/adapter-shapes.md`, wrong for a measurement with a rerun command) | `CLAUDE.md` repository map; `experiments/position-visibility/README.md:23-26`; `references/adapter-shapes.md:5-19`; `discover.md:35` |
| **The finding is stated as a finding** | `README.md` names which of DA-3's three outcomes holds — (1) everything passes, CF-27's predicted result and the strongest evidence for ES-39; (2) something fails for a completeness reason, named; (3) the honest answer needs an ES-39 primitive, which is the AC-012 escalation fed to DA-7's four-row option table — **with the other two named as the outcomes that did not hold**. It draws no conclusion ADR-0028 owes, and it does not draw DT-7's: the suffix-vs-scattered comparison is *recorded* for HS-S0120 to weigh | `.../_decomposition.md:255-274`, `:363-379`; `discover.md:32`, `:34` |
| **Wording is the clause's wording** | The recorded lists are titled as *the rules that cannot tell a pruned store from a young one*, not "rules that passed". A reader must not be able to mistake the census for a conformance result | `spec/SPECIFICATION.md:8036-8041`; `project.md:198-201` |
| **`wasm32` gating, and its stated cost** | `catch_unwind` cannot catch where there is no unwinder, so the census carries `#![cfg(not(target_arch = "wasm32"))]` — which `fixture_instruments.rs:43` already applies to the whole file this story mounts into, so nothing new is lost. If HS-S0114's target is *not* gated (i.e. the instrument is being type-checked for wasm32 by `cargo xtask wasm`), do not gate it: put the census in its own target and share the instrument as a `#[path]`-included module, and say so in the writeup | `crates/happenstance-testkit/tests/mutation_coverage.rs:33-46`; `crates/happenstance-testkit/tests/fixture_instruments.rs:43` |
| **The census harness is local, not borrowed** | Do not `#[path]`-include `mutation_coverage/harness.rs`: its items are `pub(crate)` to *that* binary and the unused ones (`model_probes`, `concurrency_probes`, `Origin`'s classifiers) become `dead_code` warnings in a new binary, which `-D warnings` turns into a failed gate — and it would couple two proof artefacts to one file. Duplicating ~60 lines of *mechanism* is the cheaper trade; duplicating the rule-name *data* is forbidden | `crates/happenstance-testkit/tests/mutation_coverage/harness.rs:60-63`, `:338-370`; `CLAUDE.md` (clippy `-D warnings`) |
| **Nothing is written that pre-decides ADR-0028** | No rule body, no registry line, no mutant, no marker move, no `.kb/` atom, no port surface. The instrument is not made more reachable | `.../_decomposition.md:543-546`; `project.md:142-146`; `discover.md:51` |

## Data and migrations

**N/A for schema, storage or migration** — this story adds no persistent store, no wire format and no
serialised type; the instrument keeps everything in a `MemoryEventStore` behind the decorator and the
run leaves nothing behind.

There is exactly one **data contract**, and it is the deliverable: the census file format under
`experiments/completeness-pass-list/results/`. It is read by humans and by ADR-0028, and it will be
regenerated at least twice more (HS-S0116, HS-S0117), so its shape is fixed here rather than
per-author:

- **One file per subject**: the suffix configuration, the scattered configuration, and the
  `MemoryFixture` control. Three files, same format, so any two are diffable against each other.
- **A header block** naming: the subject, the retained predicate stated in words (not as positions —
  a position recorded as though it were a property of the design is ES-39's floor argument
  reproduced in miniature, `discover.md:69`), the rule count the run was produced against, and the
  one-line statement of what the list *is*, in CF-27's wording.
- **One line per rule, in `for_each_event_store_rule!` order** — the suite's own order, which carries
  its grouping; alphabetical sorting was rejected because a reordering of the registry is itself
  information and sorting hides it. Line shape: `PASS <rule>`, `SKIP <rule> — <CAPABILITY>: <reason>`,
  `FAIL <rule> — <first line of the panic message> @ <file>:<line>`.
- **Stable across runs on one commit**: no clock, no elapsed time, no counts of anything the runtime
  chooses, no absolute paths from the build machine. A diff between two runs of the same commit must
  be empty, and a diff after a rule is added must show exactly the added lines.

## Acceptance criteria

Each criterion is stated from the intent of a reader on one of the initiative's named journeys
(`initiative.md:241-250`): the **adapter author** on *Learn when you are finished*, whose loop ends in
a suite that says pass or fail and names why; the **evaluator** on *Decide in one sitting*, who opens
public evidence cold and leaves with a stated reason; and the **maintainer** standing where DoD 15 is
judged (`initiative.md:402-404`). All nine together discharge project **AC-002** (`project.md:198-201`)
and its testing-brief row (`_decomposition.md:604`), plus AC-T04 (`_decomposition.md:704-706`).

| id | criterion | verification |
| --- | --- | --- |
| **AC-001** | **GIVEN** the maintainer must be able to say a store that holds only a suffix of its own log was *exercised* against the full contract, and DA-1 requires two shapes because a suffix-only run makes a floor look correct, **WHEN** the instrument's test target is run, **THEN** the entire registered rule set has executed under `event_store_conformance!` **twice** — once over a suffix-retained configuration and once over a scattered hole with survivors *below* it — as two distinctly named modules in the one target, with neither run's rule set a subset of the other's. | `cargo test -p happenstance-testkit --test completeness_instrument` — both `event_store_conformance!` modules present and green in `-- --list`; mount shape asserted against `crates/happenstance-testkit/tests/fixture_instruments.rs:201-205` |
| **AC-002** | **GIVEN** the evaluator must never be able to mistake *"the suite observed nothing"* for *"the suite observed and agreed"* — a declined rule is a **passing `#[test]`** carrying a `SKIP` line on suppressed stdout — **WHEN** the census runs a rule the fixture declines by capability, or a rule whose assertions panic, **THEN** the first is recorded as `SKIP <rule> — <CAPABILITY>: <reason>` carrying the fixture's own declined reason verbatim and is **excluded from the pass list**, and the second is recorded as `FAIL <rule> — <message> @ <file>:<line>` without aborting the remaining probes. | `census_distinguishes_pass_skip_and_fail` in `crates/happenstance-testkit/tests/completeness_instrument.rs` — drives one declined capability and one deliberately panicking probe through the same `catch_unwind` path and asserts three distinct verdicts |
| **AC-003** | **GIVEN** HS-S0116 and HS-S0117 each add a rule to `for_each_event_store_rule!` and change this census's denominator, **WHEN** the rule set moves, **THEN** the census's rule names move with it **without any edit to this story's code**, because the only source of names is `for_each_event_store_rule!` feeding `__emit_rule_names`, and no literal rule-name list exists anywhere in this diff's generation path. | `census_rule_names_match_the_registry_exactly` asserting the census's name vector equals `for_each_event_store_rule!(__emit_rule_names)` element-for-element and in order; plus a reviewer check that the added files contain no hand-written rule-name array (`rg -n '"[a-z_]+_[a-z_]+"' crates/happenstance-testkit/tests/completeness_instrument.rs`) |
| **AC-004** | **GIVEN** CF-27's question is comparative — the rules that cannot tell a pruned store *from a young one* — so a list produced without a control is not an answer to it, **WHEN** the census runs, **THEN** it also runs unchanged against `MemoryFixture` in the same binary and the same run; the recorded pass list is the set of rules that `PASS` on **both** the control and the configuration; any rule that `FAIL`s in the control is disqualified from every list and reported as a harness fault rather than as a finding; and any rule whose `SKIP` `(capability, reason)` pair differs between the control and a configuration is **recorded as a finding rather than absorbed**. | `control_census_reports_no_failure` and `skip_reasons_agree_with_the_control` in the same target; the differential visible as a diff between `results/control.txt` and each configuration file |
| **AC-005** | **GIVEN** DA-3's two dishonest resolutions would satisfy AC-003 by the letter while destroying AC-002 — the pass list is evidence *only if every failure excluded from it is about completeness* — **WHEN** the census is produced, **THEN** the four seeding-sensitive rules DA-3 names (`reading_an_empty_store_yields_nothing`, `head_of_an_empty_store_is_none`, `condition_against_an_empty_store_admits_the_append`, `two_fixture_instances_observe_none_of_each_others_appends`) appear as `PASS` in **all three** censuses, and the artefact states that confirmation in its own text rather than leaving it to a reviewer's inference. | `connect_presents_an_empty_store_in_every_configuration` asserting those four names carry `PASS` in all three censuses; the confirmation sentence present in `experiments/completeness-pass-list/README.md` (AC-A04, `_decomposition.md:512-513`) |
| **AC-006** | **GIVEN** the next author adds an unrelated rule and must not be taught to edit the evidence until the gate goes green, **WHEN** the census is generated, **THEN** exactly one `#[test]` fails unless **every registered rule appears exactly once in each census with exactly one verdict**, and that test asserts **nothing about which rules pass** — no golden comparison against recorded content exists anywhere in the diff. | `census_covers_every_registered_rule_exactly_once`; plus the reviewer check that no `assert_eq!` in the target compares against a recorded `results/*.txt` file or an inline expected pass list (`_decomposition.md:604`, `discover.md:26`) |
| **AC-007** | **GIVEN** the evaluator opens the evidence cold and the maintainer must tell a **stale** list from a **wrong** one, **WHEN** they open `experiments/completeness-pass-list/`, **THEN** they find exactly three same-format `results/*.txt` files (suffix, scattered, control), each opening with a header block of five labelled lines — subject, the retained predicate **stated in words and never as positions**, the rule count it was produced against, the one-line statement of what the list is in CF-27's wording, and the regeneration command — followed by **exactly one line per registered rule in `for_each_event_store_rule!` order and nothing else**; and `run.sh` regenerates all three byte-identically on the same commit. | `census_render_is_stable_across_runs` (renders each census twice in-process and asserts byte-equality — no clock, no absolute path); running `experiments/completeness-pass-list/run.sh` twice leaves `git diff --stat experiments/completeness-pass-list` empty; header-block shape reviewed against the *Data and migrations* contract above |
| **AC-008** | **GIVEN** ADR-0028 (HS-S0121) must inherit **evidence**, not a conclusion, and DT-7 is HS-S0120's to settle, **WHEN** the writeup is read, **THEN** `README.md` names which of DA-3's three outcomes holds **with the other two named as the outcomes that did not hold**; records the suffix-versus-scattered comparison as data for DT-7 without drawing DT-7's conclusion; and, if outcome (1) holds, states it as CF-27's *predicted* result and the strongest available evidence for ES-39 rather than as an error or a reason to re-tune the instrument. | Content review of `experiments/completeness-pass-list/README.md` against `_decomposition.md:255-274` and `:363-379`, and against `spec/SPECIFICATION.md:8049-8055`; the finding's wording checked against `project.md:198-201` (*"stated as what it is"*) |
| **AC-009** | **GIVEN** the maintainer must be able to say the answer came from the run and not from the run being **arranged**, and that the instrument is an instrument first and a target never, **WHEN** this PR's diff is inspected, **THEN** it adds no name to `for_each_event_store_rule!`, no `REGISTRY` row, no `Defect`, no marker move, no `.kb/` atom, no port surface and no `pub`/`pub use` that reaches the instrument out of `crates/happenstance-testkit/tests/`; and the story gate is green on the result. | `cargo xtask affected --base main` (`.redkiln/config.yaml:40`) green; `no_orphan_rules` (`crates/happenstance-testkit/src/registry.rs:410`) green and its registered set unchanged; `git diff --name-only main` within the *PR boundary*; `cargo package --list` output for the three publishable crates unchanged (`project.md:142-146`, `discover.md:51`) |

## Interaction quality

**The composition family does not bind by name here, and that is a signed-off determination rather
than an omission.** `_design.md` records `## Surfaces`, `## Items` and `## Anti-patterns` as `N/A —
no user-facing surface`, and the sign-off (`_design.md:38-48`, `:86-95`) is on *that determination*.
This story renders no screen, adds no `pub` item and adds no route, so no design-system primitive,
theme token or DOM composition invariant applies to it.

What it does render is an **artefact a human reads** — three text files and a README — and the
project's own briefs treat that artefact's legibility as load-bearing, because DR-2's named defect
(`project.md:147-149`) is precisely a *presentation* failure: a summary instead of an enumeration. So
the composition family is discharged in that medium, against the format contract fixed in *Data and
migrations*, and it is carried by **AC-007** and **AC-008** as table rows — not as bullets here.

Every invariant below is already an `AC-###` row above. This section says only which row carries it
and how it is verified.

**State-family invariants**

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **In place, not a context jump** — regeneration rewrites the same three committed paths; it never emits a timestamped or run-numbered directory a reader would have to choose between. | AC-007 | `run.sh` writes fixed paths; a second run leaves `git diff` empty |
| **Non-occlusion** — a `SKIP` is never hidden behind a `PASS`, and a `FAIL` never truncates the remaining probes. Three verdicts, one line each, all present. | AC-002, AC-006 | `census_distinguishes_pass_skip_and_fail`; `census_covers_every_registered_rule_exactly_once` |
| **Preserved position** — the reader's place in the list survives regeneration: registry order, never alphabetical, so a diff after a rule lands shows exactly the added lines and nothing reflows. | AC-003, AC-007 | `census_rule_names_match_the_registry_exactly` (order-sensitive); byte-stability test |
| **Reversibility** — the census is *derived*; any hand-edit is undone by re-running the recorded command, and nothing in the gate rewards editing it. | AC-006, AC-007 | no golden assertion exists to edit toward; `run.sh` reproduces |
| **Reachability without ceremony** — the whole artefact is regenerable from one committed command with no interactive step, no network and no tool outside the workspace toolchain. | AC-007 | `experiments/completeness-pass-list/run.sh`, in the shape of `experiments/position-visibility/run.sh:1-13` |

**Composition-family invariants, in the artefact's medium**

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **Presentation exists at all** — the deliverable is a composed, self-describing document, not a raw dump: each file states its subject, its predicate in words and what the list *is* before the first rule name. A bare list of names is the "bare markup" analogue and fails this. | AC-007 | header-block review; `census_render_is_stable_across_runs` renders the header as part of the compared bytes |
| **Placement** — the finding lives in `README.md`; the `results/*.txt` files carry data and the header block only. Prose interleaved with rule lines makes the files undiffable. | AC-007, AC-008 | format contract in *Data and migrations*; content review |
| **Transience** — nothing is revealed on demand or generated at read time. All three censuses are persistent committed chrome; a reader who cannot run `cargo` still gets the whole answer. | AC-007 | files committed under `experiments/`, not produced by a gate step |
| **Density budget, with numbers** — three `results/*.txt` files; a header block of **exactly five** labelled lines; **exactly one** line per registered rule and no other line; one `README.md` carrying the finding and the three outcomes. | AC-007 | `census_covers_every_registered_rule_exactly_once` bounds the body; header shape reviewed |
| **Hierarchy** — the one-line statement of what the list is comes **before** the enumeration, in CF-27's wording, so a reader who stops after the header has still read the claim correctly. | AC-007, AC-008 | wording checked against `spec/SPECIFICATION.md:8036-8041` |
| **Named anti-patterns** (this story's, since `_design.md` records none): a prose summary in place of the enumeration (DR-2); a pass list with no control; a list titled *"rules that passed"* rather than *"rules that cannot tell a pruned store from a young one"*; positions recorded as though they were a property of the design. | AC-004, AC-007, AC-008 | control tests; title and header review; `discover.md:69` |

## Error conditions

| id | condition | required behaviour |
| --- | --- | --- |
| **EC-001** | A rule `FAIL`s against the **`MemoryFixture` control**. | The census records it, the run is a **harness fault**, and the rule is disqualified from every pass list *and* from every failure list. It is never reported as a completeness finding. Fix the harness and re-run before committing any `results/*.txt`. (AC-004) |
| **EC-002** | A rule's `SKIP` `(capability, reason)` differs between the control and a configuration. | Recorded as a **finding** in `README.md`, not absorbed into either list — a skip is *"neither a pass nor a failure — it is a hole in the map"* (`crates/happenstance-testkit/tests/mutation_coverage.rs:2941-2956`). A divergence usually means the instrument's `Fixture` declined a capability the control accepts, which is information about the instrument. |
| **EC-003** | One of DA-3's four seeding-sensitive rules `FAIL`s in any census. | **Halt.** The instrument is pre-seeding, which is DA-3's first dishonest resolution. Trace the failure to the retained predicate or to `connect()`, fix it in the slice's shared context (it is HS-S0114's code), and re-run. Never record the failure as a completeness result. (AC-005) |
| **EC-004** | The run exposes a decorator bug — DA-2's named trap, `ReadOptions` applied *above* the filter so `limit` returns short pages (`_decomposition.md:226-232`). | Fix HS-S0114's decorator inside this slice's shared context and re-run the census. It is **never** a reason to relax what the census records, and never a reason to narrow the configuration set. |
| **EC-005** | A probe panics with a payload that is neither `&str` nor `String`, or the panic carries no location. | The verdict is still `FAIL`, with a stated placeholder for the missing part (`FAIL <rule> — <non-string panic payload> @ <unknown>`). A census line is never dropped because the panic was awkward — dropping it would break AC-006. |
| **EC-006** | The completeness `#[test]` reports a rule missing from a census, or appearing twice. | **Halt.** This is the check that catches a census built from anything but the macro, or a probe silently dropped. It is not to be relaxed, `#[ignore]`d, or narrowed to one configuration. (AC-006) |
| **EC-007** | The sibling's target is named something other than `completeness_instrument.rs`, or is gated differently for `wasm32`. | Mount into **that** file rather than creating a second target for the same instrument. If the sibling's target is *not* `cfg`-gated off `wasm32` (i.e. the instrument is being type-checked by `cargo xtask wasm`), put the census in its own gated target and share the instrument through a `#[path]`-included module, and say so in `README.md`. Two targets holding one instrument is how the two configurations drift apart. |
| **EC-008** | HS-S0114 has not landed when this story starts, or landed only one configuration. | The missing configuration is added here (it is inside the slice). A missing *instrument* is a dependency failure to halt on and report — never a licence to write a second instrument, which would make the pass list a measurement of something nobody else runs (`_storymap.md`, *Three notes*). |

## Non-functional

| id | requirement | why, and how it is held |
| --- | --- | --- |
| **NF-001** | **Determinism.** Two runs of the census on one commit produce byte-identical output. No clock, no elapsed time, no thread counts, no absolute build-machine paths, no hash-map iteration order. | The artefact is diffed across commits by later stories; a jittering file makes every diff meaningless. Held by `census_render_is_stable_across_runs` and by rendering from `RuleOutcome` values rather than by parsing libtest output. |
| **NF-002** | **Runtime.** The target runs the registered rule set three times (two configurations plus the control) plus the two `event_store_conformance!` mounts. It must stay a normal `cargo test` — no watchdog thread, no timeout harness, no `#[ignore]`d slow tier. | The mutation harness records why a watchdog was rejected in this repository (`crates/happenstance-testkit/tests/mutation_coverage/harness.rs:31-35`); the same reasoning applies. If the target becomes slow enough to want one, that is a finding for the writeup, not a new mechanism. |
| **NF-003** | **`wasm32`.** `catch_unwind` cannot catch where there is no unwinder, so the census carries `#![cfg(not(target_arch = "wasm32"))]`, exactly as `crates/happenstance-testkit/tests/mutation_coverage.rs:33-46` does and as `crates/happenstance-testkit/tests/fixture_instruments.rs:43` already applies to the file this story mounts into. The cost is stated in `README.md` rather than left implicit. | `cargo xtask wasm` must stay green and must not silently lose coverage it had. |
| **NF-004** | **No new dependency, anywhere.** No `Cargo.toml` in the workspace gains a line; `experiments/completeness-pass-list/` is not a crate and not a workspace member. | `experiments/` is *"measurements. reproducible, and not in the gate"* (`CLAUDE.md`). A dependency added for an experiment is a dependency three publishable crates then carry. |
| **NF-005** | **`-D warnings` clean, with no `#[allow]` bought to get there.** In particular: do not `#[path]`-include `mutation_coverage/harness.rs` — its unused `pub(crate)` items become `dead_code` in a new binary and fail the gate. Duplicate ~60 lines of *mechanism*; never duplicate the rule-name *data*. | `crates/happenstance-testkit/tests/mutation_coverage/harness.rs:60-63`, `:338-370`; `CLAUDE.md` *Commands*. |
| **NF-006** | **The experiment adds no gate step.** `xtask` is not modified; `run.sh` is run by a human or by a later story, never by `cargo xtask ci`. | A recorded measurement that the gate re-derives is a test with extra steps, and it would make every unrelated rule addition a red build — the exact dynamic `_decomposition.md:604` rules out. |
| **NF-007** | **Legibility to a cold reader.** A reader who has never seen this repository can, from `README.md` alone, name what the list is, which configuration each file describes, what the control is for, and how to regenerate all three. | The evaluator's journey is *Decide in one sitting* (`initiative.md:249-250`); evidence that needs a guide is not evidence at that grain. |

## Implementation notes (non-prescriptive)

Shape only. Anything here loses to the compiler, to `_decomposition.md`'s DA-1/DA-2/DA-3, or to what
HS-S0114 actually landed.

- **The census as a value, not a print.** Build `struct Census { subject: String, predicate: String,
  rule_count: usize, rows: Vec<(&'static str, Verdict)> }` with `enum Verdict { Pass, Skip { capability:
  &'static str, reason: String }, Fail { message: String, origin: String } }`, and a `fn render(&self)
  -> String`. Everything else — the completeness test, the stability test, the file written by
  `run.sh` — reads that one value. Printing directly is what forces the stability check to parse
  stdout, which is how a clock or a path leaks in.
- **Probes from the macro, as `probes` already does.** Invoke `for_each_event_store_rule!` with a local
  emitter that expands to one entry per rule holding the name and a closure calling
  `happenstance_testkit::rules::$name`, driven by `happenstance_testkit::block_on`
  (`crates/happenstance-testkit/src/registry.rs:330`). `crates/happenstance-testkit/tests/mutation_coverage/harness.rs:346-370`
  is the working precedent for the hygiene question (a macro-local emitter may mention the fixture
  *type* parameter; it may not mention a local binding).
- **Three verdicts from a two-variant type.** `RuleOutcome` has `Ran` and `Skipped` and no `Failed` —
  a failing rule **panics**. So wrap each probe in `std::panic::catch_unwind`, install a quiet hook for
  the duration that captures `PanicHookInfo`'s location, and restore it afterwards; `harness.rs:96-133`
  and `:449-470` do exactly this and record why.
- **The control is a third subject, not a special case.** Run the same probe table against
  `MemoryFixture` and produce a `Census` with the same shape. The comparison then falls out as a diff
  between two `Census` values (and, on disk, between two files) rather than as bespoke assertion code.
- **`run.sh`, in the existing shape.** `experiments/position-visibility/run.sh:1-13` is the model: a
  handful of lines, no arguments, writes into `results/`. Here it is one `cargo test … --nocapture`
  invocation whose output is split into the three files, or three focused invocations — whichever
  makes the byte-stability property easiest to hold.
- **Write the `README.md` last, from the files.** The finding is read *off* the censuses. Writing it
  first is how a prose summary comes back in through the front door.

## Tests and CI (merge gate)

Tiers are the project testing brief's (`_decomposition.md:598-635`), not invented here. This project
is rank 5 and **not** terminal, so `cargo xtask ci` is a local sanity check and never this story's
proof artefact (AC-T05).

| tier | command / path | proves |
| --- | --- | --- |
| integration (conformance) | `cargo test -p happenstance-testkit --test completeness_instrument` | AC-001 — both `event_store_conformance!` mounts run the full registered rule set; the mount is the macro, never a hand-rolled driver (AC-T02) |
| integration (census) | the same target's `census_distinguishes_pass_skip_and_fail`, `census_rule_names_match_the_registry_exactly`, `control_census_reports_no_failure`, `skip_reasons_agree_with_the_control`, `connect_presents_an_empty_store_in_every_configuration` | AC-002, AC-003, AC-004, AC-005 — three verdicts; names from the registry only; the control differential; no seeding artefact |
| integration (completeness gate) | the same target's `census_covers_every_registered_rule_exactly_once` | AC-006 — the only thing the gate asserts about the census, and the one it can genuinely fail |
| unit (determinism) | the same target's `census_render_is_stable_across_runs` | AC-007, NF-001 — byte-identical renders; no clock, no absolute path |
| meta (existing, must stay green) | `cargo test -p happenstance-testkit no_orphan_rules` (`crates/happenstance-testkit/src/registry.rs:410`) | AC-003, AC-009 — the registry is still the single list, and this PR added nothing to it |
| recorded data (**not** a gate step) | `experiments/completeness-pass-list/run.sh`, then `git diff --stat experiments/completeness-pass-list` | AC-007 — the three censuses are regenerable and committed; a second run is a no-op |
| static / content review | `experiments/completeness-pass-list/README.md` | AC-005 (the seeding confirmation in the artefact's own words), AC-008 (the DA-3 finding, the other two outcomes named, DT-7 recorded not concluded) |
| static / surface | `git diff --name-only main` against the *PR boundary*; `cargo package --list` for `happenstance-core`, `happenstance`, `happenstance-testkit` unchanged | AC-009 — no rule, no registry line, no mutant, no marker, no `.kb/` atom, no `pub` reach |
| story grain (wired) | `cargo xtask affected --base main` (`.redkiln/config.yaml:40`) | fmt, clippy `-D warnings`, tests for the affected set, plus the five file-reading lints and `spec-trace` unconditionally — NF-003, NF-005, AC-009 |
| integration grain, cheap tripwire (wired) | `cargo xtask lints && cargo xtask spec-trace` (`.redkiln/config.yaml:48`) | the clauses this story executes still cite what exists; no marker moved |
| integration grain, project (wired, not this story) | `cargo xtask ci --fast` (`.redkiln/config.yaml:55`) | the project's ceiling per AC-T05; run at `integration_scoped`, including the four mandatory `wasm32` steps |

## Risks and coupling (PR-scoped)

| risk | likelihood / impact | mitigation, in this PR |
| --- | --- | --- |
| **The temptation to tune.** The pass list reads "all of them", it looks like a failed experiment, and someone adjusts the retained predicate until two rules go red. | Medium / **Severe** — it destroys AC-002 entirely | AC-008 states outcome (1) as CF-27's *predicted* result up front; DA-3 names re-tuning as the failure the decision exists to prevent (`_decomposition.md:272-274`); AC-003's two failures are owed by HS-S0116/HS-S0117's mutants, not by this story |
| **The sibling's target differs** in name or `wasm32` gating from what this spec assumes. | High / Low | EC-007: mount into whatever HS-S0114 landed; never a second target for one instrument. Both stories are in one slice and one context |
| **A decorator bug surfaces mid-run** (DA-2's `ReadOptions`-above-the-filter trap). | Medium / Medium | EC-004: fix HS-S0114's code inside the slice and re-run; the census is never relaxed to accommodate it |
| **The artefact goes stale** when HS-S0116 and HS-S0117 land their rules. | Certain / Low **by design** | The rule-count stamp in every header (AC-007) lets a reader tell stale from wrong; both stories re-run `run.sh` rather than hand-editing (`_storymap.md:70-71`, `discover.md:36`) |
| **Borrowing `mutation_coverage/harness.rs`** to save sixty lines, and taking `dead_code` warnings plus a coupling between two proof artefacts. | Medium / Medium | NF-005 forbids it explicitly and states the trade: duplicate mechanism, never data |
| **Scope creep into ADR-0028.** The run produces a strong opinion and the writeup starts drawing the conclusion. | Medium / High | AC-008 and AC-009: findings only, with the other outcomes named; any "this needs a port surface" conclusion is a finding fed to DA-7's option table (`_decomposition.md:363-379`) and consumed by HS-S0124 (`project.md:240-243`) |
| **Coupling downstream.** Four stories read this output: HS-S0116, HS-S0117 and — via `_storymap.md`'s `depends_on` — ADR-0028's story. A format change after they start is expensive. | Low / Medium | The format is fixed in *Data and migrations* here rather than per-author, and the three files share one shape so any two are diffable |
| **A census that costs more than the suite it observes**, slowing the affected gate for every later story in this crate. | Low / Medium | NF-002: it stays a plain `cargo test`; if it does not, that is a recorded finding, not a new harness |

## Dependencies

**Blocks on** (must be merged first — and, per the slice discipline, implemented in the same context):

- `retained-set-instrument-and-conformance-mount` (HS-S0114) — the completeness instrument, its
  `Fixture`, its retained-predicate parameterisation, its filtering non-`async` `read`, and the
  `crates/happenstance-testkit/tests/` target carrying the first `event_store_conformance!` mount.
  This story runs that instrument; it does not build one. (`_storymap.md:68`, `:130-132`)

**Unlocks** (their `depends_on` names this story):

- `positions-are-not-reused-after-removal` (HS-S0116) — ES-38's rule and its renumbering mutant,
  written *after* the run so it is not written to the answer someone expected.
- `condition-over-removed-history-does-not-reject` (HS-S0117) — ES-40's rule, its mutant, and the
  documentation sentence.
- `adr-0028-and-the-open-question-wave` (HS-S0121) — the decision whose evidence base is this pass
  list, and which cites `experiments/completeness-pass-list/` by path.

**Feeds, without blocking**: `dt-7-signal-shape-and-the-redaction-answer` (HS-S0120) weighs the
suffix-versus-scattered comparison recorded here; `surface-diff-and-the-ac-012-escalation` (HS-S0124)
consumes any "the honest answer needs a surface" finding.

## Anchors (progressive disclosure)

Everything above is actionable without opening these. Open each at the moment named — they carry the
depth this spec deliberately did not paste.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/_decomposition.md` | DA-1 (`:150-171`) fixes *why two configurations and which two*; DA-2 (`:173-232`) carries the `ReadOptions`-over-filter trap; DA-3 (`:234-274`) names the three outcomes and the two dishonest resolutions; the testing brief (`:604-605`, `:704-706`) sets the "committed artifact, not a test assertion" bar | Before mounting the second configuration (DA-1), and again before writing the finding (DA-3) | AC-001, AC-005, AC-008 |
| `spec/SPECIFICATION.md` | CF-27 (`:8034-8060`) is the experiment being executed and supplies the exact wording the artefact must use (`:8036-8041`) and the prediction that makes "all of them" a result (`:8049-8055`); ES-39 (`:4325-4349`) is what a suffix-only run cannot falsify | Before writing the header block's one-line statement, and before naming outcome (1) in `README.md` | AC-001, AC-004, AC-007, AC-008 |
| `crates/happenstance-testkit/src/registry.rs` | `for_each_event_store_rule!` (`:94`) is the single rule list; `__emit_rule_names` (`:293`) is how names are obtained from it; `block_on` (`:330`) drives a rule without a runtime; `no_orphan_rules` (`:410`) is the invariant this story must leave green | Before writing the probe table — this is the file that decides the census cannot be transcribed | AC-003, AC-006, AC-009 |
| `crates/happenstance-testkit/tests/mutation_coverage/harness.rs` | The working precedent for every mechanism this census needs: the quiet panic hook and `catch_unwind` (`:96-133`, `:449-470`), the macro-local emitter and its hygiene constraint (`:346-370`), the recorded reason there is no watchdog (`:31-35`), and the `pub(crate)` shape that makes `#[path]`-including it a gate failure (`:60-63`) | Before implementing the three-verdict harness — copy the mechanism, do not include the file | AC-002, AC-006, NF-002, NF-005 |
| `crates/happenstance-testkit/src/contract.rs` | `RuleOutcome`'s two variants and `skip_line` (`:471-484`, `:517-521`) are why a skipped rule is a *passing test* and why the census must be three-valued; `Capability` and `Fixture` are what a declined reason is read from | Before deciding what a `SKIP` line records | AC-002 |
| `crates/happenstance-testkit/src/fixtures.rs` | `MemoryFixture` (`:243`) is the young-store control and the reference `connect()` shape; its declined `REOPEN` with a real reason (`:270-292`) is the pattern the skip comparison is written against | Before wiring the control run and the skip-agreement assertion | AC-004, AC-005 |
| `crates/happenstance-testkit/tests/fixture_instruments.rs` | `:201-205` is the exact `event_store_conformance!` mounting shape this story adds a second invocation in; `:43` shows the file-level `wasm32` gate already in force on the sibling's target | Before adding the second mount, and when resolving EC-007 | AC-001, NF-003 |
| `crates/happenstance-testkit/tests/mutation_coverage.rs` | `:33-46` is the `wasm32` gate with its stated reason; `:2941-2956` is the "a skip is a hole in the map" discipline this census reuses; `:2889` shows what a meta-test over the registry looks like | When gating the target, and when deciding what to do with a divergent skip (EC-002) | AC-004, NF-003 |
| `experiments/position-visibility/run.sh` and `experiments/position-visibility/README.md` | The repository's existing shape for a reproducible measurement kept out of the gate — `run.sh:1-13` for the command, `README.md:23-26` for how a result is stated | Before creating `experiments/completeness-pass-list/` | AC-007 |
| `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/project.md` | AC-002 (`:198-201`) is the criterion this story discharges; DR-2 (`:147-149`) names "it passed" as the defect; `:142-146` and `:240-247` are the instrument-never-a-target and escalation-not-licence constraints | Before writing `README.md`, and before adding any export | AC-008, AC-009 |
| `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/cf-27-experiment-and-recorded-pass-list/discover.md` | This story's own signal ledger; `:26` gives the reason a golden assertion is forbidden, `:35` records the artefact-form question this spec settled, `:36` the staleness plan, `:44-51` the wrong implementation — a pass list produced by tuning | When tempted to assert on the recorded content, and before finalising the artefact's location | AC-006, AC-007, AC-009 |
| `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/_design.md` | The signed-off no-surface determination (`:38-48`, `:86-95`) — the reason the composition family is discharged in the artefact's medium rather than against design-system primitives | Before writing anything that looks like a surface, or if a reviewer asks why `_design.md` is silent | AC-007 |
| `.bklg/from-contract-to-published-library/initiative.md` | DoD 15 (`:402-404`) is the scenario this run advances; the named journeys (`:241-250`) are who the acceptance criteria are written from | When judging whether `README.md` reads to a cold evaluator (NF-007) | AC-007, AC-008 |
| `.redkiln/config.yaml` | `:40` is the story-grain gate this PR must pass; `:48` and `:55` are the integration grains, and `:60` is the terminal-project gate that is deliberately *not* this story's | Before declaring done | AC-009 |

## Clarifications resolved during spec

1. **The artefact's form and home** — deferred by discover (`discover.md:35`) and settled here:
   `experiments/completeness-pass-list/` with `README.md`, `results/*.txt` and `run.sh`. The story's
   own backlog folder lost because it is archived at closeout and ADR-0028 would cite a path that
   moves; `references/` lost because it is for evidence kept only for citation, not for a measurement
   with a rerun command (`CLAUDE.md` repository map).
2. **The census is three-valued, not two.** Not stated in any brief, and load-bearing: `RuleOutcome`
   makes a skipped rule a *passing* `#[test]`, so a two-valued pass list would silently count skips as
   evidence. AC-002 exists because of this.
3. **A `MemoryFixture` control run was added.** No brief asks for it by name. CF-27's own wording is
   comparative — *"cannot tell a pruned store from a **young** one"* — so the pass list is a
   differential or it is not an answer to the clause. It is also what makes AC-005's seeding audit
   mechanical rather than editorial.
4. **The AC count is nine, exactly as the first pass enumerated.** None added, none dropped. AC-004
   (the control) and AC-005 (the seeding audit) were the two that could have been folded into one row;
   they are kept separate because they fail for different reasons and one is a harness fault while the
   other is a halt.
5. **The gate asserts completeness, never content.** The alternative — a golden `assert_eq!` over the
   recorded pass list — was rejected in `_decomposition.md:604` and `discover.md:26` and is restated
   here as AC-006's second clause, because it is the single easiest thing for an implementer to add in
   good faith.
6. **`_design.md` binds nothing on this story, and that is recorded rather than assumed.** It declares
   no surface and the sign-off is on that determination. The RFC §6.7 composition family is therefore
   discharged against the *artefact's* format contract, which the project's own DR-2 already treats as
   load-bearing.
7. **Test names in this spec are indicative.** The verification column names the test each AC must
   have; if HS-S0114's target already carries a differently-named equivalent, extend it rather than
   adding a near-duplicate, and record the substitution in the ledger's `verifying_test` field.
