---
item: HS-S0117
stage: spec
created: 2026-08-12T13:47:57.927Z
updated: 2026-08-12T13:47:57.927Z
template_sig: 87bbf1d0
rendered_sig: c9cb6546
---

# Spec — ES-40's rule, the vacuous pass as specified behaviour, and its documentation

## Scope lock

| | |
| --- | --- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` — BR-11, AC-14, **DoD 15**, DT-7, exit criterion 5 |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` — the DAG, the scope seams, **gate decision 4** (`:239-249`, no published-surface change) |
| Project | `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/project.md` — **AC-005** (`:210-214`), AC-003 (`:202-204`), AC-011 (`:236-239`), DR-5, DR-3 |
| This spec | `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/condition-over-removed-history-does-not-reject/spec.md` |
| Key briefs | `…/retention-and-incomplete-logs/_decomposition.md` — architecture **DA-5** (`:310-326`), DA-3 (`:234-274`), DA-4 (`:276-308`), DA-7 (`:363-379`), composition roots 2–3 (`:92-106`), *Gate mechanics* (`:472-497`); testing brief **AC-005 row** (`:606`), AC-003 row (`:604-605`), AC-011 row (`:613`) |
| Signed-off design | `…/retention-and-incomplete-logs/_design.md` — **no user-facing surface**, approved 2026-08-12 (`:38-48`, `:86-95`). This story renders none and adds none |
| Story map row | `…/retention-and-incomplete-logs/_storymap.md:71` (this story), `:137-140` (merge order inside the slice), `:18-25` (the four shaping facts) |
| This story's own discover | `…/condition-over-removed-history-does-not-reject/discover.md` — the signal ledger, the four questions it answers or defers (`:33-37`), and the named wrong implementation (`:41-49`) |
| Roadmap pointer | `RUNBOOK.md:165` (phase 14 status row), `RUNBOOK.md:4626-4674` (phase 14 in full), `RUNBOOK.md:307` (ADR-0028's queue row) |

## One-line PR slice

Write ES-40's rule (`spec/SPECIFICATION.md:4351-4379`) asserting the **vacuous pass** as *specified*
behaviour — `Guard::is_violated_by` has two arms and no third
(`crates/happenstance-core/src/append.rs:236-253`) — green against the completeness instrument,
registered once in `for_each_event_store_rule!`, with a `Defect` that rejects or errors instead (the
adapter that "helpfully" fails closed on its own gap) carrying its own `REGISTRY` row; and discharge
ES-40's *"the port's documentation MUST state…"* obligation as rustdoc beside
`AppendCondition::is_violated_by` and `EventStore::append`, so an adapter author reads the contract
and an ingest author reads the hazard, with **no signature change anywhere in `happenstance-core`**.

## Executive summary

HS-S0114 built the store that forgets and HS-S0115 recorded what the suite could not see about it.
This PR converts one line of that finding into a rule the registry runs forever, and writes the
sentence ES-40 has been asking for since it was drafted.

The delta is four things, and none of them is a behaviour change:

1. **One rule body in `suite.rs` and one name in the registry.** `condition_over_removed_history_does_not_reject`
   asserts that a condition whose query ranges over *removed* history admits the append. The
   assertion is of what ES-40 **specifies**, not of what a first reader wants — that inversion is the
   named wrong implementation this story must not write (`discover.md:47`).
2. **One registered wrong implementation.** `GapAwareRejectingStore` — an otherwise-conformant store
   that notices its own gap and fails closed. It is the *sympathetic* mutant, which is what makes it
   the dangerous one, and it is the second of project AC-003's two named failures (DA-3(2)).
3. **The documentation sentence, on both surfaces.** ES-40 is a documentation clause, so discharging
   it costs rustdoc and nothing else — this is the story that *proves* gate decision 4 is affordable
   rather than merely asserting it (DA-5, `_decomposition.md:310-326`; `project.md:316-318`).
4. **One regenerated table cell.** Writing the rule makes it resolvable, so §7.2's ES-40 row loses its
   `†` and `cargo xtask spec-trace` reports the generated region stale until `--write` runs. That is a
   consequence of *this* commit and is fixed in it; the `[PROVISIONAL]` marker and the `(new)` on
   ES-40's `Rule:` line are **not** touched — they are HS-S0123's.

What this PR does **not** do: change `Guard`, `AppendCondition` or `EventStore` in any way a compiler
can see; make the silence loud; or decide ES-39. The vacuity is the contract, and the loudness owed to
project AC-006 belongs to the reader stories (HS-S0118, HS-S0119).

## Context pack

Everything below is a decision this story must honour. Depth is behind the anchors; nothing here needs
another file to be actionable.

**ES-40 is a documentation clause, and its rule asserts the specified outcome.** The normative text:
*"The port's documentation MUST state that an `AppendCondition` is a claim about the log the
evaluating store holds, not about the world… A store that has had matching history removed MAY admit
an append that would have been rejected, and the contract MUST NOT imply otherwise"*
(`spec/SPECIFICATION.md:4353-4357`). So the rule asserts **admission**, and a rule asserting rejection
would be green against the mutant and red against every conformant store — inverting the suite. AC-005
says *"asserts the specified outcome"* for exactly this reason (`project.md:210-214`).

**The vacuous pass is not a defect to be fixed here, and there is nowhere to put a fix.**
`Guard::is_violated_by` is a two-arm `match` over events that still exist and has no third arm
(`crates/happenstance-core/src/append.rs:236-253`); `AppendCondition::is_violated_by` fans out to it
with `.any()` (`:225-234`). A third outcome is a new variant on the condition-evaluation result — DA-7
row 3, a new public type reaching every caller's `match`, and a `0.3.0` this initiative's exit criteria
do not contemplate (`_decomposition.md:363-379`). **This story does not reach for it.** If the
implementer concludes one is required, that is a finding handed to HS-S0124 (project AC-012), never a
change made here.

**E2E-47 wants something this rule deliberately does not give it, and that gap is recorded rather than
closed.** The case's `THEN` asks for *"a third thing — 'this log cannot evaluate this condition' —
rather than 'no match'"* (`spec/E2E-CASES.md:1232-1246`). ES-40, written later and against the same
code, says the store MAY admit and the contract MUST NOT imply otherwise. The rule follows ES-40. So
E2E-47 stays **unsatisfied by construction**, and this PR says so in one sentence pointing at DA-7 row
3 — the alternative is a reader who assumes the case is discharged because a rule bearing its number
is green.

**The rule cannot be a property of every fixture, so it is capability-gated.** Registering a name in
`for_each_event_store_rule!` runs it against `MemoryFixture`, `LocalMemoryEventStore`,
`DurableFixture`, the instrument, **and every registered mutant** — none of which except the
instrument and this story's own mutant can be made to forget. The gate is `require!(F: <CAPABILITY>)`
(`crates/happenstance-testkit/src/suite.rs:37`, used at `:2709`), which returns `RuleOutcome::Skipped`
and **still emits the test**, reporting the fixture's stated reason. `#[cfg]`-ing the rule out is the
arrangement `contract.rs`'s module docs reject by name (`:31-42`).

**The removal seam is `MID_BATCH_FAULT`'s shape, it arrives from the slice-mate, and there is exactly
one of it.** DA-4 fixes the shape: a **defaulted** `Capability` const whose default is
`Capability::declined(<reason>)` (`crates/happenstance-testkit/src/contract.rs:207-211`) plus a
**defaulted method whose body panics** naming both ways of reaching it (`:297-307`). A *required*
trait item breaks every `Fixture` impl in and out of the workspace and is the AC-011 violation the
testing brief names (`_decomposition.md:613`). HS-S0116 (`positions-are-not-reused-after-removal`) is
this story's slice-mate and merges first; **if it landed the seam, this story consumes it and adds
none.** Only if HS-S0116 concluded its rule needed no seam does this story land exactly one, in that
shape, with its reason written as a trade (a `Capability` is a trade and owes a reason; a limit is a
fact and does not — `contract.rs:44-54`).

**A new capability owes a line in the mutation harness's `declines()`.** That function enumerates
`SECOND_HANDLE`, `REOPEN` and `MID_BATCH_FAULT` by hand — *"the one list in the binary with no
mechanical backstop"* — and its own docs record that omitting `MID_BATCH_FAULT` when it arrived would
have made every mutant's skip of that rule unaccountable
(`crates/happenstance-testkit/tests/mutation_coverage/harness.rs:520-546`). If this story adds a
capability, it adds that line in the same commit.

**The mutant must fail *exactly* this rule.** `mutants_fail_exactly_their_declared_rules`
(`crates/happenstance-testkit/tests/mutation_coverage.rs:2889`) checks both directions, and
`mutant_registry_is_exhaustive` (`:2754`) rejects a `Kind::Mutant` row with an empty `fails` list.
`GapAwareRejectingStore` is therefore an otherwise-conformant store whose defect is reachable **only
after removal has been requested through the declared seam** — it declares the capability
`SUPPORTED` the way `mutants.rs:2199` and `:3330` already do for `MID_BATCH_FAULT`, and behaves
identically to a conformant store until then. A mutant that rejects conditions unconditionally fails
half the append-condition family and cannot be declared honestly.

**Reject *or* error — pick one, and the other is a second row if it is wanted.** A `Defect` that
returns `AppendError::ConditionViolated` models the adapter that fails closed on a detected gap; one
that returns `AppendError::Store` models the adapter that treats its own gap as infrastructure
failure. Both are plausible; they are different `FailureMode`s (`Assertion` versus
`StorePanic`/error-shaped assertion, `mutation_coverage.rs:110-123`) and must not be two behaviours on
one store (`discover.md:36`).

**CF-6 is live in this rule and easy to violate.** The condition's `after` boundary must come from a
position the store actually assigned — captured from the append or the read, never written down. The
instrument's first retained position differs between DA-1's suffix and scattered configurations, so a
rule naming a literal would be green here and meaningless elsewhere (`discover.md:56-59`). The grep
(`xtask/src/lints.rs:628`) catches two spellings; `GappedPositionStore` catches the rest.

**Two gate steps fire on this diff for reasons that are not obvious.** *CF-29*: every rule name in the
rule files must appear in `CHANGELOG.md` with at least 120 characters of prose naming the defect it
detects (`spec/SPECIFICATION.md:8141-8146`; `xtask/src/lints.rs:525`). *Check 9*: writing the body
makes the name resolvable, so §7.2's ES-40 cell must lose its `†` (`xtask/src/spec_trace.rs:1174-1179`)
— and `cargo xtask affected` runs `spec-trace` unconditionally at the **story** grain
(`.redkiln/config.yaml:36-40`; `xtask/src/affected.rs:119-125`), so an un-regenerated table is a red
story gate, not a problem deferred to HS-S0123.

**No surface change, and here it is provable rather than intended.** The rustdoc is prose; the rule is
an additive `pub fn` through `pub use suite::rules` (`crates/happenstance-testkit/src/lib.rs:189`);
any fixture item is defaulted. The one sentence the finding must also carry: adding a conformance rule
is semver-MINOR **and can still turn a passing adapter's CI red**, which is the mechanism
`CLAUDE.md`'s rule that matters exists to produce and not a violation of AC-011
(`crates/happenstance-testkit/Cargo.toml:4-13`). HS-S0124 records both sentences; this story keeps
every addition in the shape that makes the first one true.

**Persona-journey slice.** Two readers, one sentence, and they are reading for opposite reasons. The
**adapter author** meets the vacuous pass and asks "is my store broken?" — the answer must be *no, this
is the contract, and here is why the predicate cannot say otherwise*. The **ingest author** meets the
same sentence and must recognise their own situation: re-evaluating an origin condition against a
pruned slice and concluding "no match", which under the unconditional-ingest decision is *"not merely
a hazard but the **normal** path"* (`spec/SPECIFICATION.md:4373-4379`). A sentence that says
"conditions are evaluated over the local log" is true, gate-green and useless to the second reader —
that is `discover.md:48`'s named wrong implementation, and it is the one that will pass review if
nobody is looking for it.

**Do not re-explain the neighbouring mechanism — link it.** `.kb/concepts/torn-reads-and-the-append-condition-boundary.md`
already carries *why a condition derived from a read cannot catch what the read did not see*. This is
the same mechanism one cause along: there the events exist and were not seen, here they do not exist
at all.

## Integration contract

- **Archetype**: `capability` — a slice through the layers a caller meets: a registered rule an
  adapter author's CI runs, a wrong implementation that proves it can fail, and the documentation the
  same author reads before either.
- **Slice / milestone**: **`owed-rules-and-mutants`**. Slice-mate, implemented in one context and
  merged first: **`positions-are-not-reused-after-removal`** (HS-S0116) — ES-38's rule, DA-2(a)'s
  renumbering mutant, and the removal seam if one is needed. Ordering is `_storymap.md:137-140`.
- **Mount point**: **`crates/happenstance-testkit/src/registry.rs:94`** — the
  `for_each_event_store_rule!` invocation list, the *only* place the rule set is written down. One
  line there, in the `--- Append conditions ---` block beside
  `condition_after_beyond_the_last_matching_position_admits_the_append` (`:189-203`), is what makes
  the rule run under all four emitters (`__emit_tokio`, `__emit_blocking`, `__emit_wasm`,
  `__emit_rule_names`) and, through `for_each_mutant!` (`crates/happenstance-testkit/tests/mutation_coverage.rs:2056`),
  against every registered mutant. A body in `suite.rs` that is not on that list is invisible to all
  of them and to `spec-trace`.
- **Wires into**:
  - `crates/happenstance-testkit/src/suite.rs` — the rule body, in the append-condition family beside
    `condition_after_beyond_the_last_matching_position_admits_the_append` (`:4832`) and
    `condition_against_an_empty_store_admits_the_append` (`:4713`), which is the *shape* to copy: an
    admission asserted with a liveness mirror that must reject.
  - `crates/happenstance-testkit/src/contract.rs` — `Fixture`, `Capability`, `RuleOutcome`; the
    `MID_BATCH_FAULT` precedent at `:207-211` and `:297-307`; the trade-versus-fact line at `:44-54`.
  - `crates/happenstance-testkit/src/suite.rs:37` — `require!`, the capability gate that yields
    `RuleOutcome::Skipped` while still emitting the test.
  - `crates/happenstance-testkit/tests/mutation_coverage.rs:324` (`REGISTRY`), `:2056`
    (`for_each_mutant!`) and `crates/happenstance-testkit/tests/mutation_coverage/mutants.rs` — the
    three edits one wrong implementation costs.
  - `crates/happenstance-testkit/tests/mutation_coverage/harness.rs:520-546` — `declines()`, if a new
    capability arrives.
  - The completeness instrument and its target from HS-S0114 — `crates/happenstance-testkit/tests/completeness_instrument.rs`
    per HS-S0115's spec, **or whatever file name that story actually landed**; this story mounts into
    it and creates no second target.
  - `crates/happenstance-core/src/append.rs` (`AppendCondition::is_violated_by`, `:225-234`) and
    `crates/happenstance-core/src/store.rs:213` (`EventStore::append`) — rustdoc only.
- **Renders surfaces**: **none.** `_design.md` records no user-facing surface for this project and the
  sign-off is on that determination (`_design.md:38-48`, `:86-95`).
- **Public items**: `_design.md`'s `## Items` is `N/A`. The additive items this story creates are one
  `pub async fn` rule re-exported through `pub use suite::rules` (`crates/happenstance-testkit/src/lib.rs:189`)
  and, only if HS-S0116 did not already land it, one defaulted `Fixture` const plus one defaulted
  `Fixture` method. Both are enumerated for HS-S0124's surface diff; neither is breaking.
- **Conformance rule(s)**: **`condition_over_removed_history_does_not_reject`** — written, registered,
  green against the instrument in both DA-1 configurations, `Skipped` with a stated reason on every
  fixture that cannot forget, and red against `GapAwareRejectingStore`.
- **Clause(s)**: **ES-40** (`spec/SPECIFICATION.md:4351-4379`, `[PROVISIONAL]`) — this story
  discharges its documentation MUST and supplies the assertion its falsifier line asks for. The
  marker itself is **not** moved and the `(new)` on its `Rule:` line is **not** removed (HS-S0123,
  project AC-014); the gate tolerates both because check 4 skips a clause that schedules a new rule
  (`xtask/src/spec_trace.rs:695-696`). **ES-37** (`[FROZEN]`, `:4274-4297`) is honoured untouched: no
  store operation is added. No `[FROZEN]` clause changes, so no new ADR is owed by this story.
- **Advances DoD scenario**: **DoD 15** — *"Incomplete logs have an answer on disk … a store that
  holds only a suffix of its own log is exercised against a reader"* (`initiative.md:402-404`). This
  PR lands the half that makes the answer *checkable by an adapter's CI* rather than only recorded,
  and supplies the assertion ES-40's `[PROVISIONAL]` marker names as the thing that discharges it.

## PR boundary

```
crates/happenstance-testkit/src/suite.rs
crates/happenstance-testkit/src/registry.rs
crates/happenstance-testkit/src/contract.rs
crates/happenstance-testkit/tests/**
crates/happenstance-core/src/append.rs
crates/happenstance-core/src/store.rs
spec/SPECIFICATION.md
CHANGELOG.md
.bklg/from-contract-to-published-library/retention-and-incomplete-logs/condition-over-removed-history-does-not-reject/**
```

**In this PR**

- `condition_over_removed_history_does_not_reject` in `crates/happenstance-testkit/src/suite.rs`, with
  its rustdoc explaining *why the assertion looks wrong and is not*, and its one line in
  `for_each_event_store_rule!`.
- `GapAwareRejectingStore` in `crates/happenstance-testkit/tests/mutation_coverage/mutants.rs`, its
  type in `for_each_mutant!`, and its `Declared` row in `REGISTRY` with a non-empty `fails`, a real
  `provenance`, a `mode`, and an `expect` pin naming the exact assertion it trips.
- The `declines()` line in `mutation_coverage/harness.rs`, **only if** this story adds the capability.
- The removal seam on `Fixture` — one defaulted const, one defaulted panicking method — **only if**
  HS-S0116 did not land it.
- ES-40's documentation sentence as rustdoc on `AppendCondition::is_violated_by` and on
  `EventStore::append`, one linking the other, plus the link to
  `.kb/concepts/torn-reads-and-the-append-condition-boundary.md`.
- `CHANGELOG.md` under `[Unreleased]` — one entry naming the defect the rule detects (CF-29).
- `spec/SPECIFICATION.md`: **only** the §7.2 cell regenerated by `cargo xtask spec-trace --write`, and
  ES-40's `(append.rs:239-253)` citation corrected if this story's rustdoc moved those lines.
- This story's own backlog folder: the `_ledger.md` the second pass authors.

**Explicitly not in this PR**

- Any change to `Guard::is_violated_by`, `AppendCondition`, `EventStore` or `AppendError` that a
  compiler can see. A third condition-evaluation outcome is DA-7 row 3 and HS-S0124's escalation.
- ES-40's `[PROVISIONAL]` marker, the `(new)` on its `Rule:` line, ES-39's or CF-27's markers, and
  §1.3's hand census — all HS-S0123's (project AC-014).
- ES-38's rule, its registry line and its renumbering mutant — HS-S0116, the slice-mate.
- `suffix_store_is_distinguishable_from_a_young_store` — HS-S0122, and writing it here would pre-decide
  ADR-0028.
- Any reader run — `read_decision_model`, the projection runner, `IngestStore::holds` — HS-S0118 and
  HS-S0119. This story asserts that the silence is specified; it does not try to make it loud.
- Any `.kb/` atom, any DT-7 resolution, any redaction answer — HS-S0120 and HS-S0121.
- Any change to the instrument's semantics or reachability: no `pub use`, no promotion out of
  `crates/happenstance-testkit/tests/`, no version bump on `happenstance-testkit`.
- Regenerating `experiments/completeness-pass-list/` — HS-S0115's artefact goes stale by one rule when
  this lands, which its own header's rule count is designed to reveal; re-running it is that story's
  recorded procedure, not this story's edit.

**Merge DoD**: the rule is in `for_each_event_store_rule!` and green against the instrument in both
configurations, `GapAwareRejectingStore` fails exactly it with both meta-tests green in both
directions, ES-40's sentence is on both core surfaces with no signature changed, `CHANGELOG.md` names
the defect, and `cargo xtask affected --base main` (`.redkiln/config.yaml:40`) — which runs `spec-trace`
— is green.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **The rule asserts admission, not rejection** | Over a store whose retained set no longer contains the events the condition's query ranges over, `append` with that condition returns `Ok`. This is ES-40's `MAY admit` read as the suite's `MUST NOT reject` — a store that rejects has converted a `MAY` into a prohibition and shows every writer a spurious conflict for a decision model it cannot evaluate | `spec/SPECIFICATION.md:4353-4357`; `project.md:210-214`; `discover.md:41-46` |
| **It is not vacuously green** | The admission is paired with a **liveness mirror** in the same body: the identical condition, evaluated *before* removal (or over a retained match), must be rejected. Without it the rule is satisfied by a store whose `append` never rejects anything, which is the shape `condition_after_beyond_the_last_matching_position_admits_the_append` already carries a mirror for and says why | `crates/happenstance-testkit/src/suite.rs:4864-4884`; `CLAUDE.md` (*a rule that no adapter can fail is decorative*) |
| **No literal position values** | The condition's `after` is `AppendCondition::after_opt` fed from a position the store assigned — the append's return value, or the `last_seen` of a read — never a written-down number. The instrument's first retained position differs between DA-1's two configurations, so a literal would be green here and meaningless on any other store | CF-6, `spec/SPECIFICATION.md:8141`-adjacent; `xtask/src/lints.rs:600-655`; `discover.md:56-59`; `crates/happenstance-core/src/append.rs:200-212` |
| **It is capability-gated, and a decline is reported rather than hidden** | `require!(F: <REMOVAL CAPABILITY>)` at the top of the body. Every fixture that cannot be made to forget returns `RuleOutcome::Skipped` carrying its own stated reason, and the test is still emitted — `#[cfg]`-ing it out is rejected by name in `contract.rs`'s module docs | `crates/happenstance-testkit/src/suite.rs:37`, `:2709`; `crates/happenstance-testkit/src/contract.rs:31-42`, `:471-484` |
| **The seam is consumed, not duplicated** | If HS-S0116 landed a removal `Capability` + defaulted method, this rule uses it and adds nothing. If not, this story adds exactly one, in `MID_BATCH_FAULT`'s shape: defaulted const defaulting to `Capability::declined(<reason>)`, defaulted method whose body panics naming both ways of reaching it, plus the `declines()` line | `_decomposition.md:276-308`; `crates/happenstance-testkit/src/contract.rs:207-211`, `:297-307`; `crates/happenstance-testkit/tests/mutation_coverage/harness.rs:520-546` |
| **`GapAwareRejectingStore` is conformant until it is asked to forget** | It declares the removal capability `SUPPORTED` (as `mutants.rs:2199` and `:3330` do for `MID_BATCH_FAULT`), behaves exactly like the conformant store beforehand, and only after removal rejects a condition whose query ranges over the hole — one behaviour, one defect. A store that rejects unconditionally fails half the append-condition family and cannot be declared honestly | `crates/happenstance-testkit/tests/mutation_coverage.rs:141-186` (`Declared`); `_decomposition.md:310-326` |
| **Reject or error — exactly one** | Choose the arm that fails *exactly* this rule and record why the other lost. `ConditionViolated` models the adapter that fails closed; a store-level error models the adapter that treats its gap as infrastructure failure. If both are wanted, the second is a **second row** with its own store, never a second behaviour on the first | `discover.md:36`; `crates/happenstance-testkit/tests/mutation_coverage.rs:110-123` (`FailureMode` is per store, not per rule) |
| **Both meta-tests green in both directions** | `mutant_registry_is_exhaustive` (`:2754`) rejects a `Kind::Mutant` row with an empty `fails` list and a pin on a rule the mutant does not declare; `mutants_fail_exactly_their_declared_rules` (`:2889`) rejects both an undeclared failure and a declared one that does not occur. The `expect` pin names the exact assertion, so a later edit that trips the mirror instead cannot stay green | `crates/happenstance-testkit/tests/mutation_coverage.rs:154-186`, `:2754`, `:2889` |
| **CF-1 is satisfied by construction** | `every_rule_has_a_mutant` requires a mutant per rule; this story's rule and its mutant land in the same commit, which is also CF-29's *"same release as its mutant"* half | `crates/happenstance-testkit/tests/mutation_coverage.rs:2734`; `spec/SPECIFICATION.md:8141-8143` |
| **The documentation serves two readers, and is checked against the second** | On `AppendCondition::is_violated_by`: a condition is a claim about the log **the evaluating store holds**, not about the world; a conditional append is sound only where that store holds every event the query ranges over; where matching history was removed the condition passes vacuously and the append is admitted. On `EventStore::append`: the same fact stated as the hazard an ingest path meets — re-evaluating an origin condition against a pruned slice and concluding "no match" is the *normal* path under unconditional ingest, not an edge case. The review question is whether an ingest author reading it recognises their own situation | `spec/SPECIFICATION.md:4353-4357`, `:4368-4371`, `:4373-4379`; `discover.md:48` |
| **The documentation links rather than re-derives** | The torn-read atom already explains why a condition derived from a read cannot catch what the read did not see; this is the same mechanism with the events destroyed rather than unseen. Link it; do not restate it | `.kb/concepts/torn-reads-and-the-append-condition-boundary.md` |
| **No signature changes in `happenstance-core`** | Doc comments only, on two existing items. The check is mechanical: no line in the diff for `crates/happenstance-core/src/append.rs` or `store.rs` falls outside a `///` block | gate decision 4, `../../_decomposition.md:239-249`; `_decomposition.md:613`, `:641-643` |
| **E2E-47 is recorded as unsatisfied, not quietly claimed** | The case asks for a third outcome; ES-40 forbids implying one exists. One sentence in the rule's rustdoc (and in the ledger) states that E2E-47's `THEN` is **not** met by this rule, names DA-7 row 3 as the only shape that would meet it, and routes the escalation to HS-S0124 | `spec/E2E-CASES.md:1232-1246`; `_decomposition.md:363-379`; `project.md:240-243` |
| **CF-29's changelog half** | One `[Unreleased]` entry naming the defect the rule detects — the adapter that fails closed on its own gap — carrying at least 120 characters of prose. The lint is described in its own clause as the weakest check in the gate, so the entry is written for the reader, not for the character count | `spec/SPECIFICATION.md:8141-8156`; `xtask/src/lints.rs:525`; `CHANGELOG.md:24` |
| **CF-33: the rule reads no clock** | Nothing in the body sleeps, times out or samples time; removal is requested through the fixture seam and observed synchronously | `xtask/src/lints.rs:265-280` |
| **§7.2 is regenerated in this commit** | Writing the body makes the name resolvable, so ES-40's rule cell must lose its `†`. `cargo xtask spec-trace` reports the generated region stale otherwise, and `cargo xtask affected` runs it at the story grain. Run `cargo xtask spec-trace --write`, commit the one-cell diff, and change nothing else in the file except ES-40's `append.rs` citation if the rustdoc moved it. §1.3's census is untouched — no maturity marker moves here | `xtask/src/spec_trace.rs:1174-1179`, `:740-748`; `xtask/src/main.rs:671-674`; `xtask/src/affected.rs:119-125`; `.redkiln/config.yaml:36-40` |
| **The additive surface is enumerated for HS-S0124** | One `pub async fn` through `pub use suite::rules`, plus at most one defaulted `Fixture` const and method. Recorded with the sentence the crate itself states: adding a conformance rule is semver-MINOR **and can still turn a passing adapter's CI red** — the mechanism, not a violation. No `happenstance-testkit` version bump in this PR | `crates/happenstance-testkit/src/lib.rs:189`; `crates/happenstance-testkit/Cargo.toml:4-13`; `_storymap.md:80` |
| **Both port flavours, and the bare bound** | The rule is generic over `F: Fixture` whose `Store: EventStore` — the weaker bound, which accepts both flavours — and only one of `EventStore` / `SendEventStore` is in scope in the module. The rule runs under all four emitters, `__emit_wasm` included, from the one registry line | `CLAUDE.md` constraints 1, 3 and 4; [ADR-0001](../../../../.kb/decisions/0001-async-port-flavours.md); `crates/happenstance-testkit/src/contract.rs:125` |

## Data and migrations

**N/A — no schema, no storage, no wire format, no migration.** This story adds a conformance rule, a
test-only wrong implementation, doc comments and a changelog entry. Nothing it writes is persisted,
serialised or version-negotiated: the instrument keeps its log in a `MemoryEventStore` behind a
decorator that lives only for the duration of a rule, and the mutant is `Rc`-backed like every other
store in that binary.

Two adjacent things that are **not** data contracts and must not be mistaken for them:

- **`AppendCondition`'s `serde` mirror** (`crates/happenstance-core/src/append.rs:257-270`) is the wire
  form governed by WF-2/WF-4 and ADR-0016. This story adds no field, changes no spelling and touches
  no `#[cfg(feature = "serde")]` code. A third condition-evaluation outcome *would* reach it, which is
  one more reason DA-7 row 3 is an escalation rather than an option.
- **`experiments/completeness-pass-list/`** is HS-S0115's recorded data, and this story invalidates its
  denominator by one rule. That is by design — the artefact carries the rule count it was produced
  against so a stale list is distinguishable from a wrong one — and it is re-run by its own recorded
  command, never hand-edited here.

## Acceptance criteria

The personas are this initiative's own, carried from
`.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md` because the
`.kb/product/` layer is still empty (`initiative.md:227-233`): the **adapter author** on the journey
*Learn when you are finished*, the **application/ingest author** on *Choose a contract before a
database*, the **constrained-runtime developer** on *Event-source at the edge without hand-rolling it*,
and the **evaluator** on *Decide in one sitting*. Every criterion below is one of them crossing the
stack — from the command they type to the answer they get — not a capability restated.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **The adapter author's suite says "this is the contract", not "your store is broken".** GIVEN a store that has had matching history removed — HS-S0114's instrument, in **both** DA-1 configurations (suffix and scattered) — WHEN the adapter author runs `event_store_conformance!` over it, THEN `condition_over_removed_history_does_not_reject` **passes**, because an `append` carrying a condition whose query ranges over the removed events returned `Ok`; and no assertion anywhere in that body demands rejection for the removed-history case. The rule asserts ES-40's *specified* outcome (`spec/SPECIFICATION.md:4353-4357`), never the outcome a first reader wants | `cargo test -p happenstance-testkit --test <HS-S0114's instrument target>` green in both configurations; review confirms the removed-history arm asserts `Ok(..)` and that no `AppendError::ConditionViolated` is expected of it |
| AC-002 | **The constrained-runtime developer's wasm32 CI runs the same rule, from the same one line.** GIVEN the rule body exists in `suite.rs`, WHEN its name is added exactly once to `for_each_event_store_rule!` at `crates/happenstance-testkit/src/registry.rs:94`, THEN it is emitted by all four emitters — `__emit_tokio`, `__emit_blocking`, `__emit_wasm`, `__emit_rule_names` — and by `for_each_mutant!`, with no second registration and no `#[cfg]` narrowing it to a subset of targets; the body is generic over `F: Fixture` whose `Store: EventStore` (the weaker bound, both flavours) with only one of `EventStore`/`SendEventStore` in scope | `cargo xtask wasm` green; the name appears exactly once in `registry.rs`; the blocking and wasm conformance targets (`crates/happenstance-testkit/tests/memory_conformance_blocking.rs`, `memory_conformance_wasm.rs`) list it; `cargo xtask affected --base main` |
| AC-003 | **The rule cannot pass by doing nothing.** GIVEN a store whose `append` never rejects any condition at all, WHEN the rule runs against it, THEN the rule **fails** — because the body pairs the admission with a liveness mirror asserting that the *same* condition, evaluated where the matching history is still retained, is rejected. GIVEN the capability gate means most registered mutants skip this rule, THEN the mirror plus this story's own mutant are the only things standing between it and decoration, and that is stated in the rule's rustdoc | `cargo test -p happenstance-testkit --test mutation_coverage` — `every_rule_has_a_mutant` (`mutation_coverage.rs:2734`) green; the implementation report records the inversion demonstration: deleting the mirror leaves the rule green against a never-rejecting store, deleting the admission arm turns it red against the instrument |
| AC-004 | **The rule stays true on a store whose positions look nothing like the instrument's.** GIVEN a conformant store that leaves gaps or starts numbering anywhere it likes, WHEN the rule builds its condition, THEN every boundary it uses (`AppendCondition::after_opt`) came from a position that store actually assigned — captured from the append's return or from a read — and no literal position value appears anywhere in the body. CF-6 is live here because the instrument's first retained position differs between the two DA-1 configurations | `cargo xtask lints` — the CF-6 grep (`xtask/src/lints.rs:600-655`) catches two spellings; `GappedPositionStore` in `mutation_coverage` catches the rest; review confirms no numeric literal is compared against a position |
| AC-005 | **A store that cannot forget says so, and no existing `Fixture` impl breaks.** GIVEN a fixture that cannot be made to remove history (`MemoryFixture`, `LocalMemoryEventStore`, `DurableFixture`, every mutant but this story's), WHEN the suite runs, THEN the rule still emits a test which reports `RuleOutcome::Skipped` carrying that fixture's own stated reason — never `#[cfg]`-ed out of the binary — and the removal seam it gates on is HS-S0116's if HS-S0116 landed one, or otherwise exactly one **defaulted** `Capability` const plus one **defaulted** panicking method in `MID_BATCH_FAULT`'s shape, with its matching line in `declines()`. No required trait item is added | `cargo test -p happenstance-testkit --test memory_conformance --test local_conformance` — the emitted test present and reporting its skip reason; `cargo test -p happenstance-testkit --test mutation_coverage` with `declines()` (`tests/mutation_coverage/harness.rs:520-546`) enumerating the capability; `cargo-semver-checks` at the PR grain reports no breaking change |
| AC-006 | **A store that fails closed on its own gap is caught by name, not by "conformance failed".** GIVEN `GapAwareRejectingStore` — otherwise conformant, declaring the removal capability `SUPPORTED`, and rejecting (or erroring on; exactly one of the two, with the loser's reason recorded) a condition whose query ranges over the hole once removal has been requested — WHEN the mutation harness runs, THEN it fails **exactly** `condition_over_removed_history_does_not_reject`, its `REGISTRY` row carries a non-empty `fails`, a real `provenance`, a `mode` and an `expect` pin naming the exact assertion tripped, and both meta-tests are green in both directions | `cargo test -p happenstance-testkit --test mutation_coverage` — `mutant_registry_is_exhaustive` (`:2754`) and `mutants_fail_exactly_their_declared_rules` (`:2889`); the `expect` pin fails the run if a later edit trips the mirror instead of the target assertion |
| AC-007 | **The ingest author recognises their own situation in the sentence, and the adapter author recognises theirs.** GIVEN ES-40's documentation MUST, WHEN the two rustdoc blocks land — on `AppendCondition::is_violated_by` (`crates/happenstance-core/src/append.rs:225-234`) stating that a condition is a claim about the log **the evaluating store holds**, and on `EventStore::append` (`crates/happenstance-core/src/store.rs:213`) stating the same fact as the hazard that re-evaluating an origin condition against a pruned slice and concluding "no match" is the **normal** path under unconditional ingest — THEN each links the other and both link `.kb/concepts/torn-reads-and-the-append-condition-boundary.md` rather than restating it; and the diff for those two files contains **no line outside a `///` block**, so no signature in `happenstance-core` changed | `cargo doc -p happenstance-core` and `cargo test -p happenstance-core --doc` green; `git diff` over `crates/happenstance-core/src/append.rs` and `store.rs` shows only doc-comment lines; review answers the two-audience question in the affirmative, per `discover.md:53` |
| AC-008 | **The evaluator is not told a case is discharged when it is not.** GIVEN E2E-47 asks for a third outcome — *"this log cannot evaluate this condition"* rather than "no match" (`spec/E2E-CASES.md:1232-1246`) — and ES-40 forbids implying one exists, WHEN this rule lands, THEN one sentence in its rustdoc and one row in this story's evidence state that E2E-47's `THEN` is **not** met, name DA-7 row 3 (a new condition-evaluation outcome, a `0.3.0`) as the only shape that would meet it, and route the escalation to HS-S0124; and no such type, variant or signature is added here | review against `spec/E2E-CASES.md:1232-1246` and `_decomposition.md:363-379`; `git diff` shows no new public type or variant in `happenstance-core`; the implementation report carries the sentence verbatim |
| AC-009 | **The record the evaluator reads is true on the day this merges.** GIVEN writing the body makes ES-40's rule name resolvable, WHEN the commit lands, THEN §7.2's ES-40 cell has lost its `†` via `cargo xtask spec-trace --write` (nothing else in the file changed except ES-40's `append.rs` citation if the rustdoc moved those lines), `CHANGELOG.md`'s `[Unreleased]` carries an entry of at least 120 characters naming the defect the rule detects, and ES-40's `[PROVISIONAL]` marker, the `(new)` on its `Rule:` line and §1.3's hand census are **untouched** — those are HS-S0123's | `cargo xtask spec-trace` green (checks 4, 6, 8, 9); `cargo xtask lints` — CF-29 (`xtask/src/lints.rs:525`); `cargo xtask affected --base main` (`.redkiln/config.yaml:36-40`) green, which runs `spec-trace` unconditionally at the story grain; `cargo xtask ci --fast` at the integration grain |

Coverage of the traced project ACs: **AC-003** (two named failing rules, owed by the mutants) → AC-006;
**AC-005** (ES-40's rule asserts the specified outcome, with a registered wrong implementation) →
AC-001, AC-003, AC-004, AC-006, AC-007; **AC-011** (no breaking change at `0.2.0`) → AC-005, AC-007,
AC-008.

## Interaction quality

This story **renders no user-facing surface**: `_design.md` records `N/A` for `## Surfaces`, `## Items`
and `## Anti-patterns`, and the sign-off (2026-08-12, `_design.md:86-95`) is on that determination
itself. So the composition family — placement, transience, density budget, hierarchy, the design's
named anti-patterns — has **no binding content to inherit** here, and inventing a density budget for a
project whose approved design says there is no screen would contradict the sign-off rather than honour
it.

What does bind is the state family, read at the grain this story actually has a reader in: the
conformance run an adapter author watches, and the rustdoc two authors read. Each invariant below is
carried by an AC row in the table above — none is a loose bullet, because a bullet here would get no
ledger row and never be gated.

| Invariant (state family) | The form it takes here | Carried by | How it is verified |
| --- | --- | --- | --- |
| **Non-occlusion** — nothing the reader needs is hidden | A fixture that cannot forget yields `RuleOutcome::Skipped` with its stated reason and the test is **still emitted**; `#[cfg]`-ing the rule out of the binary is the arrangement `contract.rs:31-42` rejects by name | AC-005 | the emitted test is present and reports its reason in `memory_conformance` / `local_conformance` |
| **In-place, not a context jump** — the answer arrives where the reader already is | One line in `for_each_event_store_rule!` puts the rule in the run the author already types; no second harness, no separate command, no parallel target to remember | AC-002 | the name appears exactly once in `registry.rs`; all four emitters carry it |
| **Preserved identity** — the thing the reader was looking at is still the thing they get back | The failure is reported as a **rule name** with an `expect` pin naming the exact assertion tripped, not as "conformance failed"; a later edit that trips the mirror instead cannot stay green under the same name | AC-006 | `mutants_fail_exactly_their_declared_rules`, both directions |
| **Reversibility / no silent commitment** — nothing is decided behind the reader's back | No third condition-evaluation outcome is introduced; E2E-47 is recorded unsatisfied and the escalation routed, so the open shape stays open and visible | AC-008 | no new public type or variant in the diff; the sentence present in rustdoc and report |
| **Presentation exists at all** — the control is not bare markup | The prose-medium analogue, and the one this story is most likely to fail: ES-40 is discharged by *prose*, and prose compiles. A sentence saying "conditions are evaluated over the local log" is true, gate-green and useless. The bar is that an **ingest author** reading it recognises their own situation, and an **adapter author** reads the vacuous pass as the contract | AC-007 | the two-audience review question, `discover.md:53`; the rustdoc links rather than restates the torn-read atom |
| **Reachability** — the reader can get there without knowing the trick | The rule runs on every target including `wasm32`, from the one registry line, under the weaker `EventStore` bound so both flavours reach it | AC-002 | `cargo xtask wasm`; the blocking and wasm conformance targets |

Keyboard reachability, focus/scroll/selection preservation and density have no referent in this story
and are recorded as **not applicable** rather than silently omitted.

## Error conditions

| id | Condition | Required behaviour |
| --- | --- | --- |
| EC-001 | The fixture under test cannot be made to remove history | `require!(F: <REMOVAL CAPABILITY>)` returns `RuleOutcome::Skipped` with the fixture's own stated reason; the test is emitted and the run is green. A skip is never a pass and is never invisible (AC-005) |
| EC-002 | `GapAwareRejectingStore` trips the **mirror** assertion rather than the target assertion | The run fails: the `REGISTRY` row's `expect` pin names the exact assertion, so a mutant that fails the right rule for the wrong reason is red, not green (AC-006) |
| EC-003 | An existing registered mutant starts failing the new rule because it supports the removal seam and trips the mirror | Its `fails` list is extended in this same commit, or the mutant's declaration is corrected — `mutants_fail_exactly_their_declared_rules` rejects an undeclared failure in either direction. Do **not** weaken the mirror to make it stop |
| EC-004 | HS-S0114's instrument target does not exist, or is named differently from `completeness_instrument.rs` | Mount into whatever target that story actually landed. Creating a second instrument target, or a bespoke forgetting store, is a dependency failure to report — not a licence to write one (`_storymap.md:111-115`) |
| EC-005 | HS-S0116 already landed a removal `Capability` and method | Consume it. Adding a second seam for the same concept is the failure this ordering exists to prevent; the `declines()` line is then already present and is not duplicated |
| EC-006 | The implementer concludes the honest rule needs a third condition-evaluation outcome | Stop. That is DA-7 row 3 and a `0.3.0` this initiative's exit criteria do not contemplate. Record the finding, name the surface and the version consequence, route it to HS-S0124 (project AC-012), and make **no** change to `Guard`, `AppendCondition`, `EventStore` or `AppendError` |
| EC-007 | `cargo xtask spec-trace` reports §7.2 stale after the body lands | Run `cargo xtask spec-trace --write` and commit the one-cell diff in this PR. Deferring it to HS-S0123 is a red story gate today, because `affected` runs `spec-trace` unconditionally |
| EC-008 | The CF-29 changelog lint fails for length | Write for the reader — the defect is "an adapter that fails closed on its own gap". The clause calls this the weakest check in the gate; padding to the character count satisfies the lint and fails the clause |
| EC-009 | The rustdoc addition shifts the lines ES-40 cites in `spec/SPECIFICATION.md` | Correct ES-40's `append.rs` citation in the same commit; `spec-trace`'s citation check (`xtask/src/spec_trace.rs:729-733`) resolves clause `file:line` references and will fail otherwise. Correct the citation only — the marker and the `(new)` stay |

## Non-functional

| id | Requirement | Why it binds here |
| --- | --- | --- |
| NF-001 | **The rule reads no clock.** No sleep, timeout, deadline or wall-clock sample; removal is requested through the fixture seam and observed synchronously | CF-33's lint (`xtask/src/lints.rs:265-280`) fails the gate on a rule that waits, and a timing-dependent conformance rule is flaky on every adapter but the one it was written against |
| NF-002 | **`wasm32` clean.** Nothing in the body reaches for `std::time`, threads, or a `Send` bound; the rule compiles and runs under `__emit_wasm` | `CLAUDE.md` constraint 1 and [ADR-0001](../../../../.kb/decisions/0001-async-port-flavours.md); `standards/rust/52-wasm32-and-target-cfg.md` |
| NF-003 | **Additive-only public surface, no version bump.** One `pub async fn` through `pub use suite::rules`, at most one defaulted `Fixture` const and one defaulted method. `happenstance-testkit`'s version is not touched in this PR | project AC-011; the enumeration is handed to HS-S0124's surface diff (`_storymap.md:78`) |
| NF-004 | **Rustdoc meets the constitution's obligations**, not merely the compiler's: the rule's own docs say *why the assertion looks wrong and is not*, and the two core blocks name the hazard rather than only the mechanism | `standards/rust/70-rustdoc-obligations.md`; `standards/rust/00-prime-directives.md` |
| NF-005 | **Compiles and passes at the MSRV floor, 1.97.1** — let-chains are available and used freely where they read better; nothing reaches past the floor | [ADR-0029](../../../../.kb/decisions/0029-msrv-raised-to-1-97-1.md); CI's `msrv` job |
| NF-006 | **The rule's cost is bounded and small** — one fixture, one removal request, two appends, no retry loop, no unbounded read. It runs once per fixture per emitter, on every adapter's CI, forever | `standards/rust/60-what-a-test-must-prove.md`; the suite is the thing adapter authors run in their own pipelines |

## Implementation notes (non-prescriptive)

**Read HS-S0115's recorded pass list first, then write the rule.** The whole reason this story sits
behind `cf-27-experiment-and-recorded-pass-list` is that a rule written before the experiment is
written to the answer someone expected (`_storymap.md:18-25`). The pass list tells you, by name,
whether anything already distinguishes the pruned store — and if `condition_*` rules are on it, that is
the empirical statement of why this rule is owed.

**Copy the shape, not the assertion.** `condition_against_an_empty_store_admits_the_append`
(`suite.rs:4713`) and `condition_after_beyond_the_last_matching_position_admits_the_append`
(`:4832`) are both admissions with a liveness mirror. The second is the closer sibling: it already
carries the mirror and already explains in its own docs why an admission needs one.

**The capability gate has a consequence worth stating out loud in the rustdoc.** Because every
registered mutant declines the removal capability by default, `for_each_mutant!` will *skip* this rule
on nearly all of them. That is correct, and it means the usual safety net — "some existing mutant will
catch a decorative rule" — is not present here. The liveness mirror and `GapAwareRejectingStore` are
the entire defence. Say so where the next reader will find it.

**Choosing reject versus error.** `AppendError::ConditionViolated` models the adapter that fails
closed; a store-level error models the adapter that treats its own gap as infrastructure failure.
`FailureMode` is a property of the store, not of the rule (`mutation_coverage.rs:110-123`), so the two
cannot be one row. Pick the one that fails *exactly* this rule, record the loser's reason in the
mutant's rustdoc, and if both are genuinely wanted the second is a second `Defect` with its own row —
never a second behaviour on the first.

**Where the two rustdoc blocks differ.** They are not the same paragraph pasted twice. The
`AppendCondition::is_violated_by` block is the *contract*: what the predicate is a claim about, and why
a pure predicate over events that still exist has nowhere to put a third answer. The
`EventStore::append` block is the *hazard*: what happens to a caller — specifically an ingest path —
that re-evaluates an origin condition against a pruned slice. One links the other; both link the
torn-read atom rather than re-deriving it.

**Do not touch the instrument's semantics.** No `pub use`, no promotion out of `tests/`, no change to
its retained predicate to make this rule easier to write. If the rule needs a configuration the
instrument does not offer, that is a conversation with HS-S0114's spec, not an edit here.

## Tests and CI (merge gate)

Grounded in the project testing brief's *Test mix* rows for AC-003, AC-005 and AC-011
(`_decomposition.md:604-613`) and its *Merge-gate commands* section (`:648-676`). This project is rank
5 and **not** terminal, so `cargo xtask ci --fast` is its ceiling; the whole gate is HS-P0019's.

| Tier | Command / path | Proves |
| --- | --- | --- |
| integration (the suite itself) | `cargo test -p happenstance-testkit --test <HS-S0114's instrument target>` | AC-001 — the rule is green against the instrument in both DA-1 configurations, which is what "asserts the specified outcome" means operationally |
| integration (skip reporting) | `cargo test -p happenstance-testkit --test memory_conformance --test local_conformance` | AC-005 — the rule is emitted and reports `Skipped` with a stated reason on fixtures that cannot forget; it did not vanish from the binary |
| unit + mutation | `cargo test -p happenstance-testkit --test mutation_coverage` | AC-003, AC-006 — `every_rule_has_a_mutant` (`:2734`), `mutant_registry_is_exhaustive` (`:2754`) and `mutants_fail_exactly_their_declared_rules` (`:2889`); `GapAwareRejectingStore` fails exactly this rule with its `expect` pin honoured |
| cross-target | `cargo xtask wasm` | AC-002 — the one registry line reaches `__emit_wasm`; nothing in the body is host-only |
| doc | `cargo doc -p happenstance-core` + `cargo test -p happenstance-core --doc` | AC-007 — the two rustdoc blocks compile, their intra-doc links resolve, and any example in them runs |
| static (lints) | `cargo xtask lints` | AC-004 (CF-6, no literal position values, `xtask/src/lints.rs:600-655`), AC-009 (CF-29 changelog, `:525`), NF-001 (CF-33 clock, `:265-280`) |
| static (spec) | `cargo xtask spec-trace`, then `cargo xtask spec-trace --write` | AC-009 — §7.2's ES-40 cell loses its `†`, ES-40's citations still resolve, and no marker moved |
| static (surface) | `cargo-semver-checks` at the PR grain (`CONTRIBUTING.md:291-296`) | AC-005, AC-007 — every addition is additive; no required trait item, no changed signature in `happenstance-core` |
| review (content) | the diff over `crates/happenstance-core/src/append.rs` and `store.rs` | AC-007, AC-008 — every changed line is inside a `///` block; the E2E-47 sentence is present and names DA-7 row 3 |
| **story grain (merge gate)** | `cargo xtask affected --base main` (`.redkiln/config.yaml:36-40`) | the whole story bar: fmt, clippy `-D warnings` and tests for `happenstance-testkit` + dependents, the five file-reading lints, and `spec-trace` unconditionally |
| **integration grain** | `cargo xtask lints && cargo xtask spec-trace` (`.redkiln/config.yaml:48`), then `cargo xtask ci --fast` (`:55`) | the project's ceiling: fmt, clippy, tests, the four mandatory `wasm32` steps, `spec-trace`, the doc builds |

## Risks and coupling (PR-scoped)

| Risk | Likelihood / impact | Mitigation, in this PR |
| --- | --- | --- |
| **The rule is written to the desired outcome instead of the specified one** — asserting rejection because the vacuous pass feels like a bug | Medium / High — it inverts the suite: green against the mutant, red against every conformant store | AC-001 states the assertion as `Ok`; the rule's own rustdoc explains why it looks wrong and is not; `discover.md:52` names this as the wrong implementation to reject; review reads the assertion before the prose |
| **The documentation sentence describes the mechanism and never names the hazard** — true, gate-green and useless to the ingest author | Medium / High — prose passes every compiler, and this is the half ES-40 actually requires | AC-007's verification is the two-audience question, not the presence of a doc comment; the `EventStore::append` block is specified as the *hazard* block, distinct from the contract block |
| **The seam is duplicated because HS-S0116 landed one under a different name** | Medium / Medium — two capabilities for one concept, and a `declines()` list that reads as complete but is not | EC-005; slice-mates are implemented in one context and HS-S0116 merges first (`_storymap.md:133-136`); check `contract.rs` before adding anything |
| **An existing mutant that supports the removal seam trips the mirror and goes undeclared** | Low / Medium — `mutants_fail_exactly_their_declared_rules` goes red and the tempting fix is to weaken the mirror | EC-003 requires extending the `fails` list, never softening the assertion |
| **The rustdoc shifts `append.rs`'s lines and breaks ES-40's citation** | Medium / Low — `spec-trace`'s citation check fails and the fix looks like a marker edit | EC-009: correct the `append.rs` citation only; the `[PROVISIONAL]` marker and the `(new)` are HS-S0123's |
| **HS-S0115's recorded pass list goes stale by one rule the moment this merges** | Certain / Low — by design | The artefact carries the rule count it was produced against, so stale is distinguishable from wrong; re-running it is HS-S0115's recorded procedure, explicitly out of this PR |
| **Adding a conformance rule turns a passing adapter's CI red** | Certain for any adapter that fails closed / Low | That is the mechanism `CLAUDE.md`'s *rule that matters* exists to produce, and `crates/happenstance-testkit/Cargo.toml:4-13` says so in the crate's own words. It is recorded for HS-S0124, not avoided |
| **Coupling to HS-S0123** — the marker, the `(new)`, and §1.3's census | Certain / Low if respected | This PR regenerates exactly one §7.2 cell and nothing else; check 4 tolerates a clause that schedules a new rule (`xtask/src/spec_trace.rs:695-696`) |

## Dependencies

**Blocks on**

- `cf-27-experiment-and-recorded-pass-list` (HS-S0115) — the recorded pass list per DA-1 configuration,
  and transitively `retained-set-instrument-and-conformance-mount` (HS-S0114), which is the store this
  rule is run against and the only forgetting store this project is permitted to have. If either is
  absent when this story starts, that is a blocker to raise, never a stand-in to write.

**Sequenced with (same slice, merges first)**

- `positions-are-not-reused-after-removal` (HS-S0116) — slice-mate in `owed-rules-and-mutants`,
  implemented in the same context and merged first (`_storymap.md:133-136`). It owns the removal seam
  if one is needed; this story consumes whatever it landed and adds a second only if it landed none.

**Unlocks**

- `adr-0028-and-the-open-question-wave` (HS-S0121) — names this story among its `depends_on`; ES-40's
  discharged documentation MUST and the vacuous-pass assertion are evidence in the retention decision.
- `marker-moves-and-spec-trace-green` (HS-S0123) — cannot move ES-40's `[PROVISIONAL]` marker or strip
  the `(new)` from its `Rule:` line until the rule this story writes exists.
- `surface-diff-and-the-ac-012-escalation` (HS-S0124) — consumes this story's enumerated additive items
  and any E2E-47 / third-outcome finding raised under EC-006.

## Anchors (progressive disclosure)

Open these when the row says to — not before, and not all at once. Everything needed to *start* is in
the context pack above.

| Anchor | Why it is load-bearing | When to open | Serves |
| --- | --- | --- | --- |
| `spec/SPECIFICATION.md` (ES-40, `:4351-4379`) | The normative text this story discharges: the `MAY admit`, the `MUST NOT imply otherwise`, the two-audience `Rule:` line and the `Rejects:` ingest case. The rule's assertion and both rustdoc blocks are written *from* it | Before writing the assertion, and again before writing the `EventStore::append` block | AC-001, AC-007, AC-009 |
| `crates/happenstance-core/src/append.rs` (`:225-234`, `:236-253`) | `AppendCondition::is_violated_by` fanning out to `Guard::is_violated_by`'s two-arm match — the code that makes the vacuous pass structural rather than accidental, and one of the two rustdoc sites | Before writing the rule's rustdoc, and before concluding anything about a third outcome | AC-001, AC-007, AC-008 |
| `crates/happenstance-core/src/store.rs` (`EventStore::append`, `:213`) | The second rustdoc site — the hazard block the ingest author reads. Its surrounding docs set the register the new block must match | When writing the hazard half of AC-007 | AC-007 |
| `crates/happenstance-testkit/src/suite.rs` (`:37` `require!`, `:4713`, `:4832-4884`) | The `require!` gate's exact semantics, and the two sibling admission rules whose shape — admission plus liveness mirror — this rule copies | Before writing the body; the mirror is not optional | AC-001, AC-003, AC-005 |
| `crates/happenstance-testkit/src/registry.rs` (`:94`, `:189-203`) | The single invocation list. One line here feeds all four emitters and `for_each_mutant!`; the append-condition block is where this name belongs | At the moment of registration — one line, exactly once | AC-002 |
| `crates/happenstance-testkit/src/contract.rs` (`:31-42`, `:44-54`, `:207-211`, `:297-307`) | `Capability`'s trade-versus-fact rule, the rejection of `#[cfg]`-ing rules out, and `MID_BATCH_FAULT`'s defaulted-const + defaulted-panicking-method shape — the only permitted seam shape | Only if HS-S0116 landed no removal seam | AC-005 |
| `crates/happenstance-testkit/tests/mutation_coverage.rs` (`:110-123`, `:141-186`, `:324`, `:2056`, `:2734`, `:2754`, `:2889`) | `FailureMode`, the `Declared` row's fields, the `REGISTRY`, `for_each_mutant!` and the three meta-tests that decide whether the mutant is honest | Before writing the `REGISTRY` row, and again when the `expect` pin is chosen | AC-003, AC-006 |
| `crates/happenstance-testkit/tests/mutation_coverage/mutants.rs` | Where `GapAwareRejectingStore` lives, and forty-odd worked examples of a `Defect` that is conformant everywhere but one place | While writing the mutant; copy a neighbour in the condition family | AC-006 |
| `crates/happenstance-testkit/tests/mutation_coverage/harness.rs` (`:520-546`) | `declines()` — the one list in the binary with no mechanical backstop, and its own docs on what omitting a capability costs | Only if this story adds a capability; in the same commit | AC-005 |
| `crates/happenstance-testkit/tests/fixture_instruments.rs` (`:201-205`) | The `event_store_conformance!` mount shape every instrument in this repo uses; the reference for how the run is reached rather than re-invented | If the instrument's mount needs reading; do not create a second target | AC-001 |
| `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/cf-27-experiment-and-recorded-pass-list/spec.md` | The blocking story: where the recorded pass list lands, and the real name of the instrument target this rule mounts into | First, before any code — it tells you what the suite already cannot see | AC-001 |
| `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/positions-are-not-reused-after-removal/spec.md` | The slice-mate that merges first and may already have landed the removal seam and its `declines()` line | Before adding any `Fixture` item | AC-005 |
| `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/_decomposition.md` (DA-3 `:234-274`, DA-4 `:276-308`, DA-5 `:310-326`, DA-7 `:363-379`, testing rows `:604-613`) | The architecture decisions this story executes rather than re-makes: who owes AC-003's failures, the seam's shape, the mutant's identity, and the four-row escalation table | DA-5 before the mutant; DA-4 before any seam; DA-7 only if tempted by a third outcome | AC-005, AC-006, AC-008 |
| `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/condition-over-removed-history-does-not-reject/discover.md` (`:36-39`, `:46-53`) | The four questions this story answered or deferred, and the three named wrong implementations — including the two that leave the gate green | Before the assertion, and before review of the rustdoc | AC-001, AC-006, AC-007 |
| `spec/E2E-CASES.md` (E2E-47, `:1232-1246`) | The case whose `THEN` this rule deliberately does not satisfy — the exact text the recorded-unsatisfied sentence must be true against | When writing AC-008's sentence | AC-008 |
| `.kb/concepts/torn-reads-and-the-append-condition-boundary.md` | The adjacent mechanism, already explained: a condition derived from a read cannot catch what the read did not see. This is that mechanism one cause along | When writing the rustdoc — to link, never to restate | AC-007 |
| `.kb/decisions/0010-the-suite-must-prove-itself.md` | Why a rule with no wrong implementation is decorative, and why the mutation harness exists at all — the standard AC-003 and AC-006 are measured against | Before deciding the mirror is optional | AC-003, AC-006 |
| `.kb/decisions/0001-async-port-flavours.md` | The two-flavour port: why the rule binds `EventStore` rather than `SendEventStore`, and why `__emit_wasm` is not optional | If the body's generics or bounds need a decision | AC-002 |
| `standards/rust/70-rustdoc-obligations.md` | The bar the two core doc blocks and the rule's own docs are held to — this story is mostly prose, so this is the atom that governs most of its diff | Before writing any of the three doc blocks | AC-007, NF-004 |
| `standards/rust/60-what-a-test-must-prove.md` | What a test must reject to have earned its place — the liveness mirror's justification in the house constitution's own words | Before writing the mirror | AC-003 |
| `xtask/src/lints.rs` (`:265-280` CF-33, `:525` CF-29, `:600-655` CF-6) | The three file-reading lints that fire on this diff and exactly what each matches — including the two spellings CF-6 catches | If a lint fails, and before writing the changelog entry | AC-004, AC-009 |
| `xtask/src/spec_trace.rs` (`:695-696`, `:727-733`, `:1174-1179`) | Why check 4 tolerates a clause scheduling a new rule, how citations are resolved, and what makes §7.2's cell stale | When `spec-trace` complains, and before touching `SPECIFICATION.md` | AC-009 |
| `.redkiln/config.yaml` (`:36-40`, `:48`, `:55`) | The commands that fire automatically at each stage transition — the merge gate is not a checklist someone remembers to run | Before claiming the story is done | AC-009 |
| `crates/happenstance-testkit/Cargo.toml` (`:4-13`) | The crate's own sentence about adding a conformance rule being semver-MINOR and still able to turn a passing adapter's CI red — the sentence HS-S0124 needs and this story must keep true | When enumerating the additive surface | AC-005, AC-007 |
| `.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md` | The four personas and journeys the acceptance criteria are framed from, with the qualification that all four rest on secondary evidence | If an acceptance criterion's user intent needs grounding | AC-007 |

## Clarifications resolved during spec

1. **The AC set is exactly the nine the first pass enumerated** — AC-001 through AC-009. None added,
   none dropped. The ledger matches.
2. **Where the rustdoc goes** (`discover.md:38`, deferred to spec): **both** sites, not one.
   `AppendCondition::is_violated_by` carries the *contract* block and `EventStore::append` carries the
   *hazard* block, each linking the other, because ES-40 names two audiences and a single block placed
   on either surface is read by only one of them. Resolved in AC-007.
3. **Reject or error** (`discover.md:39`, deferred to spec): **one** `Defect`, and the choice is the
   implementer's on the criterion "fails *exactly* this rule" — with the loser's reason recorded in the
   mutant's rustdoc. A second is a second `REGISTRY` row with its own store, never a second behaviour
   on the first, because `FailureMode` is per store. Resolved in AC-006 and EC-002.
4. **The liveness mirror is mandatory, and the reason sharpened during this pass.** Because the rule is
   capability-gated and every registered mutant declines that capability by default, `for_each_mutant!`
   *skips* it almost everywhere. The usual backstop — some existing mutant catching a decorative rule —
   is therefore absent, which makes the mirror and `GapAwareRejectingStore` the entire defence.
   Recorded in AC-003 and in the implementation notes.
5. **§7.2 is regenerated in this PR, not deferred to HS-S0123.** `cargo xtask affected` runs
   `spec-trace` unconditionally at the story grain, so an un-regenerated table is a red gate on *this*
   story. The `[PROVISIONAL]` marker, the `(new)` on ES-40's `Rule:` line and §1.3's census remain
   HS-S0123's. Resolved in AC-009 and EC-007.
6. **No composition invariants are inherited, and that is a finding rather than an omission.**
   `_design.md` records `N/A` for surfaces, items and anti-patterns, and the human sign-off is on that
   determination. The *Interaction quality* section therefore carries the state family only, mapped
   onto the reader this story actually has, and records keyboard/focus/density as not applicable
   explicitly.
7. **E2E-47 is not silently claimed.** The rule bears a number adjacent to a case it does not satisfy,
   so AC-008 requires the sentence that says so and names DA-7 row 3 — the alternative is an evaluator
   who reads a green rule as a discharged case.
8. **The instrument target's file name is treated as a claim, not a fact.**
   `crates/happenstance-testkit/tests/completeness_instrument.rs` is the target HS-S0114 creates and
   HS-S0115's ledger mounts on; it does not exist in this tree yet. EC-004 therefore makes mounting
   into whatever those stories actually landed the requirement, and forbids a second target or a
   bespoke forgetting store. The tests table names it by role for the same reason.
