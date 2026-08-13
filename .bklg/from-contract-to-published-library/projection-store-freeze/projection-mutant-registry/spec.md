---
item: HS-S0009
stage: spec
created: 2026-08-12T13:46:03.590Z
updated: 2026-08-12T13:46:03.590Z
template_sig: 87bbf1d0
rendered_sig: 0d949cec
---

# Spec — CheckpointOnlyStore fails the suite by name

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` |
| Project item | `.bklg/from-contract-to-published-library/projection-store-freeze/project.md` |
| This spec | `.bklg/from-contract-to-published-library/projection-store-freeze/projection-mutant-registry/spec.md` |
| Key briefs | `.bklg/from-contract-to-published-library/projection-store-freeze/_decomposition.md` — Architecture brief AC-A06, AC-A07 and Notes 6 and 9; Testing brief rows AC-002 and AC-003 |
| Signed-off design | `.bklg/from-contract-to-published-library/projection-store-freeze/_design.md` — **no user-facing surface**, approved 2026-08-12. Renders nothing; the design sign-off binds this story only by its no-surface determination. |
| Story map / roadmap | `.bklg/from-contract-to-published-library/projection-store-freeze/_storymap.md` — slice 4, `commit-atomicity-and-mutants`, merge position 4 of 8 |
| Grounding | `.bklg/from-contract-to-published-library/projection-store-freeze/_grounding.md` |

## One-line PR slice

`CheckpointOnlyStore` fails the suite by name: a projection `REGISTRY: &[Declared]` with the same
shape as the event-store one, the three exactness meta-tests (every rule has a mutant, the registry
is exhaustive, each mutant fails **exactly** its declared rules), non-empty provenance per entry,
ADR-0010's no-pass-rate warning in the module doc, and the stale "neither a suite nor a mutant yet"
scope note replaced.

## Executive summary

The slice-mate before this one (`projection-suite-entry-point`) landed
`projection_store_conformance!` and two rules running green against `MemoryProjectionStore`. Green
against the reference store is exactly the state ADR-0010 was written about: a rule that asserts
what the contract says passes forever and is indistinguishable from a good rule until an adapter
with the matching bug passes it too
(`.kb/decisions/0010-the-suite-must-prove-itself.md`, Context).

**This PR lands the instrument that makes the projection suite falsifiable**, as the *second
instance* of a mechanism the event-store family already runs rather than a parallel invention
(Architecture brief, Intent). Concretely, its delta over the current tree:

- A new cargo test target, `crates/happenstance-testkit/tests/projection_mutation_coverage.rs`,
  carrying a projection `REGISTRY: &[Declared]` with the same six fields the event-store registry
  uses (`crates/happenstance-testkit/tests/mutation_coverage.rs:125-186`).
- Two hostile projection stores. `CheckpointOnlyStore` — commits the checkpoint, discards the
  write set — which is the phase's whole bar (`spec/SPECIFICATION.md:5678-5685`;
  `project.md` "In scope" item 5). And a second store that discharges CF-1 for the *other*
  baseline rule, which §4.11's table leaves with an em-dash in its `Rejects:` column
  (`spec/SPECIFICATION.md:5659`) and CF-1 admits no exemption from
  (`spec/SPECIFICATION.md:7161-7164`).
- Four meta-tests over that registry — the three exactness ones plus provenance — asserting on
  `Verdict` values, not on stdout.
- The event-store registry's scope note, which currently reads "the projection store port owes its
  own CF-1 – CF-5 obligation and has neither a suite nor a mutant yet"
  (`crates/happenstance-testkit/tests/mutation_coverage.rs:197-201`), corrected to point at the new
  registry. That sentence becomes false the moment this PR merges.

What it deliberately does **not** land: any new conformance rule (its slice-mate
`commit-rollback-and-drop-rules` owns those), the CF-5 conformant variant
(`buffering-conformant-variant` owns it), and any edit to a `[FROZEN]` clause.

## Context pack

The load-bearing decisions this story must honour. Read this section before writing code; everything
deeper sits behind the signposted anchors in the second half of this spec.

### The persona-journey slice this realises

The user is **the adapter author** (`_discovery/distillation/personas-and-journeys.md:114-181`) and
this story serves step 3 of their journey — *"runs the suite"*. Their stated fear is discovering
late that the port quietly assumed something their storage system cannot provide; the honest reading
of that journey today is that "a green run is evidence about one storage shape wearing four hats".
A projection suite that no store has ever failed does not move that fear at all — it produces a
green run whose meaning is unknown. This story's deliverable is what converts the projection suite's
green from *unverified* to *discriminating*, which is the initiative's DoD 7, first half:
*a deliberately wrong implementation that writes a checkpoint without its read model fails it, by
name* (`initiative.md:376-379`).

### Decision 1 — the registry is data, hand-written, and lives away from the store it describes

ADR-0010 is accepted and its shape is not re-decided here. Three consequences bind this story
directly, each of which the existing `Declared` doc comment argues for at length
(`crates/happenstance-testkit/tests/mutation_coverage.rs:125-186`):

- **A `const` table, not per-mutant `#[should_panic]` drivers.** A `#[should_panic]` driver records
  only that *something* failed, so a mutant failing the right rule for the wrong reason reads as
  proof — the hazard CF-2 names by name (`spec/SPECIFICATION.md:7186-7197`).
- **Never generated from observed outcomes.** A generated table is a snapshot test, and a snapshot
  of a mutant registry asserts nothing: every future regression is "expected".
- **Never an associated const on the impl.** The distance between the store and the claim about it
  *is* the mechanism; putting them in one keystroke's reach stops the claim being a check.

### Decision 2 — the same six fields, with the same semantics, and `expect` keyed by rule

`Declared` carries `name`, `kind`, `fails`, `provenance`, `mode`, `expect`. The projection registry
uses the same shape (Architecture brief AC-A07). Two of those fields carry hard-won semantics that a
fresh implementation would get wrong:

- **`mode: FailureMode`** distinguishes *the rule's assertion fired* from *the store panicked*, and
  the `Assertion` arm is checked **positively** — by where the panic was raised, not by a substring
  denylist. Measured consequence in the event-store family: deleting a mutant's modelled defect made
  the panic come from the fixture contract's own "not implemented" body, which a denylist passed
  silently, leaving a whole axis vacuous and green
  (`crates/happenstance-testkit/tests/mutation_coverage.rs:2966-2999`). For the projection family
  the positive check must name the file the projection rules live in, not `suite.rs`.
- **`expect: &[(rule, substring)]`** is keyed **by rule**, not one pin per mutant. It was
  `Option<&'static str>` until a mutant acquired a second declared failure with a different message
  and the pin came off rather than the shape changing — the wrong way round
  (`:167-185`). Start keyed.

### Decision 3 — a declared failure that skips is a hole in the map, never a pass

CF-3's second direction is the load-bearing half: a mutant must fail every rule it declares **and**
pass every rule it does not (`spec/SPECIFICATION.md:7198-7208`). "Pass" has one accounted-for
exception and no others: a rule may **skip** if — and only if — the skip cites a `(capability,
reason)` pair the fixture itself declined. A skip arriving from anywhere else means a rule stopped
running and nothing noticed, and the earlier spelling of this test read a blank cell as a pass
(`crates/happenstance-testkit/tests/mutation_coverage.rs:2871-2887`, `:2908-2935`). On the declared
side a skip is never a pass either (`:2939-2964`). This story inherits both readings verbatim; it
does not soften them because the projection family is young.

### Decision 4 — the rule universe comes from the single enumeration, never a second list

`every_rule_has_a_mutant` takes its universe from `for_each_event_store_rule!` via
`__emit_rule_names` so there is no second list of rule names to keep in step
(`crates/happenstance-testkit/tests/mutation_coverage.rs:2803-2810`, `:2735`). The projection
sibling takes its universe from `for_each_projection_store_rule!` the same way — the enumeration
`projection-suite-entry-point` lands. Hard-coding the two baseline rule names in this file would
make the meta-test go green on the day a rule is added, which is the day it must go red.

### Decision 5 — the forcing function is deliberate, and later stories inherit it

`every_projection_rule_has_a_mutant` has **no exemption list and must never acquire one**: a tracker
with entries is CF-1 being *observed* rather than enforced
(`crates/happenstance-testkit/tests/mutation_coverage.rs:2720-2732`). The consequence is that this
PR makes the slice-mate and the two `reset-and-rebuild-rules` stories unable to land a rule without
its mutant — that is the intended coupling
(`_storymap.md` Coverage, AC-003 row), not an accident to be worked around. If a rule ever turns up
that is clearly right and has no plausible failing implementation, ADR-0010's honest response is to
record that in the clause and **retire the rule**, never to invent a saboteur — which the provenance
meta-test would reject anyway.

### Decision 6 — the baseline rule §4.11 gives an em-dash still owes a mutant

`spec/SPECIFICATION.md:5659` lists `commit_advances_the_checkpoint` with `—` in the `Rejects:`
column, glossed as "the baseline the rest are differential against". CF-1 is `[FROZEN]` and admits
no such exemption. So this story writes the store that fails it. The natural one is already
described, precisely, in the Architecture brief's Note 8: **a `commit` returning `Ok` that makes
neither write durable** satisfies PS-1's "or not at all" arm, passes
`commit_is_atomic_with_the_read_model`, and fails `commit_advances_the_checkpoint`
(`_decomposition.md` Architecture brief, Note 8; `spec/SPECIFICATION.md:4744-4759`).

**This is the trap in this story.** That store's existence is *evidence* for
`.kb/open-questions/ps-1-states-no-progress-obligation.md`, and it is tempting to fix PS-1 while
holding the demonstration in your hand. Do not. PS-1 is `[FROZEN]`; the repair is a new decision
atom under the repair-frozen-clause discipline, the sweep that scopes it belongs to
`ps-clause-pairing-sweep`, and the clause disposition belongs to
`unstable-projection-gate-and-clause-disposition`. This story **registers the mutant and records the
observation in its provenance string**, and stops there.

### Decision 7 — `CheckpointOnlyStore` fails at exactly one line, and only because the probe exists

The rule shape every projection rule shares ends with a fresh handle reading the read model *and*
the checkpoint and asserting both present or both absent
(`_decomposition.md` Architecture brief, Note 6). `CheckpointOnlyStore` fails at that last line and
nowhere else — which is why it is the mutant that makes the suite non-decorative. Without
`ProjectionProbe`'s `probe_write` / `probe_read` the rule cannot observe the read model at all and
the whole suite reduces to a checkpoint test a broken store passes (`RUNBOOK.md:3882-3892`).
Corollary for this story: `CheckpointOnlyStore` must be a **real, if minimal, `ProjectionStore` +
`ProjectionProbe` implementation** driven through `begin` → probe-write → `commit` → probe-read, not
a stub that returns canned answers. The Testing brief calls this tier Integration for exactly that
reason (`_decomposition.md` Testing brief, AC-002 row).

### Decision 8 — reuse `Capability` and `RuleOutcome` unchanged, and declare no borrowing GAT

AC-A06 is binding: the projection fixture reuses `Capability` and `RuleOutcome` from
`crates/happenstance-testkit/src/contract.rs:355-537` **unchanged**, and declares no borrowing GAT
anywhere. `type Store<'a> where Self: 'a` on a foreign trait is one of five ingredients of a rustc
ICE this repository already minimised and which still reproduces on 1.97.1
(`crates/happenstance-testkit/src/contract.rs:97-111`;
`experiments/rustc-ice-gat-foreign-trait/`). The hostile stores in this PR are `ProjectionFixture`
implementations and inherit that constraint.

### Decision 9 — no `[FROZEN]` clause is edited, and spec-trace does not force one

CF-1 – CF-6 are `[FROZEN]` and their `Rule:` fields cite `mutation_coverage::<name>` in the
event-store binary (`spec/SPECIFICATION.md:7161-7250`). Adding a projection family does **not**
require amending them, and the mechanism is checkable rather than hoped for: `spec-trace` marks any
`Rule:` line containing the words `meta-test` as `elsewhere` and does not attempt to resolve it, so
an unresolved projection meta-test name is not a gate failure
(`xtask/src/spec_trace.rs:1601-1625`). §4.11 already states the projection obligation in prose —
"they are mutants in CF-1's sense and take CF-1 through CF-5 unchanged"
(`spec/SPECIFICATION.md:5685-5693`). Verify the claim, then leave the clauses alone.

### Decision 10 — the binary is gated off `wasm32`, and the cost is stated rather than discovered

`std::panic::catch_unwind` cannot catch on a target with no unwinder, so a meta-test that cannot
observe a panic is the whole mechanism gone. The event-store binary carries one crate-level
`#![cfg(not(target_arch = "wasm32"))]` and states the cost — the stores in it are never type-checked
for that target (`crates/happenstance-testkit/tests/mutation_coverage.rs:33-46`). The projection
binary takes the same gate and states the same cost. This is not cosmetic: `cargo xtask ci`'s
mandatory wasm step runs `cargo check -p happenstance-testkit --tests --target
wasm32-unknown-unknown`, so an ungated projection mutant binary breaks the gate.

### Decision 11 — the CF-5 half is deferred by name, not landed empty

`mutant_registry_is_exhaustive` asserts at least one conformant variant is registered, and
`conformant_variants_pass_everything` is the positive control that keeps the other meta-tests from
being satisfied by a harness that reports failure unconditionally
(`crates/happenstance-testkit/tests/mutation_coverage.rs:2852-2859`, `:3071-3121`). The projection
family's conformant variant is the buffering, replay-at-commit store, and it belongs to
`buffering-conformant-variant` in slice 6 (`_storymap.md`; Architecture brief AC-A03). So this
story lands `Kind::ConformantVariant` in the shape and **does not** land either of those two
assertions in a form that would pass vacuously today. The hole is named in the registry's module doc
together with the story that closes it. A positive control asserting over an empty set is the
vacuity CF-5 exists to prevent, reintroduced one level up — the exhaustiveness meta-test says so in
those words.

## Integration contract

- **Archetype**: `capability` — a slice through the adapter author's whole stack: a real
  `ProjectionStore` implementation, driven through the real suite entry point, observed by real
  meta-tests, run by the real gate.
- **Slice / milestone**: `commit-atomicity-and-mutants`. Slice-mate: **`commit-rollback-and-drop-rules`**
  (HS-S0010). Both are implemented in one context and mounted as one integrated surface; this story
  lands first within the slice, because the slice-mate's seven rules each need a registry row and
  the exactness meta-tests are what force them.
- **Mount point**: **`crates/happenstance-testkit/tests/projection_mutation_coverage.rs`** — a new
  cargo test target. Mounting is by cargo target auto-discovery over `tests/*.rs`: verified that
  `crates/happenstance-testkit/Cargo.toml` declares no `[[test]]` sections and does not set
  `autotests = false`, so the file is discovered and run by `cargo test --workspace --all-features`,
  which is `cargo xtask ci`'s `tests` step. No manifest edit is required and none should be made.
  Support modules go in `crates/happenstance-testkit/tests/projection_mutation_coverage/` reached by
  `#[path]` attributes, for the reason the event-store binary already documents: a test target's root
  resolves `mod foo;` against `tests/`, and without `#[path]` cargo would compile the support files
  as targets of their own (`crates/happenstance-testkit/tests/mutation_coverage.rs:50-65`).
  **Second wiring point, and it is not optional**: the event-store registry's scope note at
  `crates/happenstance-testkit/tests/mutation_coverage.rs:188-206`, whose sentence "has neither a
  suite nor a mutant yet" this PR falsifies.
- **Wires into**:
  - `projection_store_conformance!` and `for_each_projection_store_rule!` + `__emit_rule_names`
    (landed by `projection-suite-entry-point`, HS-S0007) — the single enumeration this story's rule
    universe comes from, and the entry point the hostile stores are driven through.
  - `ProjectionFixture` in `crates/happenstance-testkit/src/contract.rs` (same story) — what each
    hostile store implements.
  - `Capability` and `RuleOutcome`, `crates/happenstance-testkit/src/contract.rs:355-537` — reused
    unchanged (AC-A06); `RuleOutcome` values are what the meta-tests assert on.
  - `ProjectionStore`, `ProjectionProbe`, `Checkpoint`, `Authority`, `CommitError`, `ResetError` in
    `crates/happenstance-core/src/projection.rs` (landed by `owned-batch-port-shape` and
    `projection-probe-conformance-feature`) — the port each hostile store implements, and the probe
    without which `CheckpointOnlyStore`'s defect is unobservable.
  - `MemoryProjectionStore` (landed by `memory-projection-store`) — the oracle each hostile store is
    a named deviation from.
  - The `Declared` / `Kind` / `FailureMode` shape at
    `crates/happenstance-testkit/tests/mutation_coverage.rs:73-186` — the design being reused. Field
    names and semantics must match so a reviewer reads one shape across both families; whether the
    types are duplicated or extracted into a `#[path]`-shared module is the implementer's call,
    recorded in the ledger either way.
- **Public items**: **none.** `_design.md` records no user-facing surface for this project, and this
  story adds nothing to any crate's `lib.rs` export block or feature table. Everything it lands is
  test-target-private in `happenstance-testkit`'s own `tests/`, which is where CF-1 requires mutant
  stores to live (`spec/SPECIFICATION.md:7161-7163`). Nothing here is a semver promise.
- **Renders surfaces**: none. The project's `_design.md` declares `surfaces: []` and states the
  no-surface determination as the thing that was approved.
- **Conformance rule(s)**: this story adds **no** conformance rule. It is observed by four
  meta-tests it authors — the projection siblings of `every_rule_has_a_mutant`,
  `mutant_registry_is_exhaustive`, `mutants_fail_exactly_their_declared_rules` and
  `every_mutant_states_its_provenance` — and by the two existing projection rules
  `commit_is_atomic_with_the_read_model` and `commit_advances_the_checkpoint`, which are what its
  two mutants are driven through and rejected by.
- **Clause(s)**: discharges **CF-1, CF-2, CF-3, CF-4** for the projection family, exactly as
  `spec/SPECIFICATION.md:5685-5693` says they apply to it, and discharges §4.11's
  `CheckpointOnlyStore` obligation (`:5678-5685`). **Amends nothing.** CF-1 – CF-6 stay untouched
  (Decision 9), PS-1 stays untouched (Decision 6), and CF-5/CF-6's projection half is deferred to
  `buffering-conformant-variant` (Decision 11).
- **Advances DoD scenario**: initiative **DoD 7, first half** — *"a deliberately wrong implementation
  that writes a checkpoint without its read model fails it, by name"* (`initiative.md:377-379`),
  which is also project DoD 1 and the runbook's stated phase bar — *"if it passes, the port is not
  frozen"* (`RUNBOOK.md:3890-3892`). This story is what makes that observable; the whole-gate run
  that *reads* it is `whole-gate-run-and-proof-artefact`'s, and a green gate is a precondition for
  looking at DoD 1, never a substitute for it (`project.md` DoD 8).

## PR boundary

**In this PR**

- The new test target `crates/happenstance-testkit/tests/projection_mutation_coverage.rs` and its
  `#[path]`-reached support modules.
- The projection `REGISTRY: &[Declared]`, its `Kind` / `FailureMode` / `Declared` shape, and its
  module documentation including ADR-0010's no-pass-rate prohibition and the named CF-5 deferral.
- Two hostile projection stores — `CheckpointOnlyStore` and the neither-write-durable store — each a
  real `ProjectionStore` + `ProjectionProbe` + `ProjectionFixture` implementation with a non-empty
  provenance string.
- The four meta-tests and the harness they need (a projection `Subject`, a `Verdict`, an `Origin`
  positive check naming the projection rules file, a `RUNTIME_PANICS` list, and a `run_subject`
  driving one store through `for_each_projection_store_rule!`), mirroring
  `crates/happenstance-testkit/tests/mutation_coverage/harness.rs:56-230, 472-520`.
- The corrected scope note in `crates/happenstance-testkit/tests/mutation_coverage.rs:188-206`.
- This story's own backlog folder — its `_ledger.md` and implementation report.
- The composition-root wiring named in the Integration contract, which for this story is exactly the
  two files above. That is not scope drift.

**Explicitly not in this PR**

- **Any new conformance rule.** The seven commit-side rules are the slice-mate's
  (`commit-rollback-and-drop-rules`); reset and read-through rules are slice 5's. This story ships
  the registry that makes them unlandable without mutants, not the rules.
- **The CF-5 conformant variant and `conformant_variants_pass_everything`** — `buffering-conformant-variant`
  (Decision 11).
- **Any edit to a `[FROZEN]` clause**, in particular PS-1 and CF-1 – CF-6 (Decisions 6 and 9). The
  PS-1 sweep is `ps-clause-pairing-sweep`'s and the disposition is
  `unstable-projection-gate-and-clause-disposition`'s.
- **A `CHANGELOG.md` entry under CF-29.** CF-29's lint asserts that every rule name in `suite.rs`
  appears in the changelog (`spec/SPECIFICATION.md:8141-8153`); this story adds no rule and touches
  no rule file, so the obligation lands on the stories that add rules.
- **Any change to the event-store registry beyond its scope note** — no re-declaration, no re-scoping
  of `Declared`, no touching `for_each_mutant!`.
- **`ProjectionProbe`, `MemoryProjectionStore`, the port shape, or the suite entry point** — all
  landed by earlier stories and consumed here as-is.

**Merge DoD one-liner**: `cargo xtask affected --base main` and `cargo xtask ci --fast` are green,
and a reviewer can name the test that `CheckpointOnlyStore` fails and see it fail by deleting one
line of that store's `commit`.

```
crates/happenstance-testkit/tests/projection_mutation_coverage.rs
crates/happenstance-testkit/tests/projection_mutation_coverage/**
crates/happenstance-testkit/tests/mutation_coverage.rs
.bklg/from-contract-to-published-library/projection-store-freeze/projection-mutant-registry/**
```

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| The registry is a hand-written `const` table of claims | `REGISTRY: &[Declared]` with fields `name`, `kind`, `fails`, `provenance`, `mode`, `expect`. `name` matches the store's own `Subject::NAME`. `fails` is the **exact** rule set, empty iff `Kind::ConformantVariant`. Never generated from observed outcomes; never an associated const on the impl. | `crates/happenstance-testkit/tests/mutation_coverage.rs:125-186`; `spec/SPECIFICATION.md:7186-7197` (CF-2) |
| The registered-store list is separate from the claims about it | Adding a store is three edits: write the defect, add its type to the projection `for_each_mutant!`-equivalent, add its row to `REGISTRY`. The two lists are the ones that drift, and the exhaustiveness meta-test is what holds them together. | `crates/happenstance-testkit/tests/mutation_coverage.rs:188-193` |
| `CheckpointOnlyStore` commits the checkpoint and discards the write set | A real `ProjectionStore` whose `commit` durably advances the checkpoint and never applies the batch's read-model writes. Driven through `begin` → `probe_write` → `commit` → fresh handle → `probe_read` + `checkpoint`. It fails `commit_is_atomic_with_the_read_model` at the both-present-or-both-absent assertion and passes everything else it does not declare. | `spec/SPECIFICATION.md:5678-5685`; `_decomposition.md` Architecture brief Note 6; `RUNBOOK.md:3882-3892` |
| A second mutant discharges CF-1 for `commit_advances_the_checkpoint` | A store whose `commit` returns `Ok` and makes **neither** write durable. It satisfies PS-1's "or not at all" arm, so it passes the atomicity rule and fails the progress rule — the one store that separates the two. Its provenance records that this is also the shape `.kb/open-questions/ps-1-states-no-progress-obligation.md` describes, **without** proposing a clause change. | `spec/SPECIFICATION.md:5659` (the em-dash), `:4744-4759` (PS-1), `:7161-7164` (CF-1); `_decomposition.md` Architecture brief Note 8 |
| Provenance names a real adapter shape, never a saboteur | Non-empty per entry, naming the adapter shape or scenario that makes the store plausible. `struct AlwaysWrong` satisfies CF-1 mechanically and proves nothing, because no author would have written it. | `spec/SPECIFICATION.md:7209-7220` (CF-4); `crates/happenstance-testkit/tests/mutation_coverage.rs:3047-3069` |
| `every_projection_rule_has_a_mutant` — CF-1 | Universe from `for_each_projection_store_rule!` + `__emit_rule_names`; covered set from the `fails` lists of `Kind::Mutant` rows. Fails naming the uncovered rules and pointing at where to write the wrong implementation. **No exemption list, ever.** | `crates/happenstance-testkit/tests/mutation_coverage.rs:2711-2749`; `.kb/decisions/0010-the-suite-must-prove-itself.md` |
| `projection_mutant_registry_is_exhaustive` — CF-2 | Six assertions, all of them earned by a real defect in the event-store family: every enumerated store has a row; every row is enumerated; no duplicate row name (shadowed by lookup); no duplicate enumerated type (driven twice, counted twice); every `fails` name is a real rule (the typo catcher, invisible to the other meta-tests); every `expect` pin names a rule the mutant declares (an unreachable pin reads as coverage and is not); and `kind` agrees with `fails` being empty or not. | `crates/happenstance-testkit/tests/mutation_coverage.rs:2751-2850` |
| `projection_mutants_fail_exactly_their_declared_rules` — CF-3 | Both directions per `(store, rule)` cell. **Declared**: must be `Verdict::Panicked`; a pass means the declaration is wrong, a skip is a hole in the map and never a pass. Under `FailureMode::Assertion` the panic origin must be *positively* inside the projection rules file, and the message must not match `RUNTIME_PANICS`. Under `StorePanic` the origin must be outside it and the message must contain the declared needle. **Undeclared**: `Passed`, or `Skipped` whose `(capability, reason)` the fixture itself declines. | `crates/happenstance-testkit/tests/mutation_coverage.rs:2862-3045`; `spec/SPECIFICATION.md:7198-7208` |
| `every_projection_mutant_states_its_provenance` — CF-4 | Trimmed provenance is non-empty for every row, conformant variants included: a variant owes an account of *why it is legally different*, or it is a second copy of the reference store. | `crates/happenstance-testkit/tests/mutation_coverage.rs:3047-3069` |
| `expect` pins are keyed by rule from the start | `&[(rule, substring)]`, optional per `(mutant, rule)`. A pin naming an undeclared rule is an error, not a no-op. | `crates/happenstance-testkit/tests/mutation_coverage.rs:167-185` |
| No pass rate, anywhere | The registry's module doc carries ADR-0010's prohibition — the denominator is an author's choice, so a fraction reports how representative the author was while reading as though it reported how good the suite is. Nothing in the binary computes or prints a fraction over the mutant set; what it reports is which defects the set covers and which axes it leaves uncovered. | `crates/happenstance-testkit/tests/mutation_coverage.rs:26-31`, `:203-206`; `.kb/decisions/0010-the-suite-must-prove-itself.md` |
| The event-store registry stops lying about the projection port | `crates/happenstance-testkit/tests/mutation_coverage.rs:197-201`'s "has neither a suite nor a mutant yet" is replaced by an accurate scope note naming the projection registry's file, keeping the "Scope: the event-store port only" framing intact. | `_decomposition.md` Architecture brief AC-A07 |
| The binary is off on `wasm32`, with the cost stated | One crate-level `#![cfg(not(target_arch = "wasm32"))]`, plus the stated cost that the stores in it are never type-checked for that target. Keeps `cargo check -p happenstance-testkit --tests --target wasm32-unknown-unknown` green. | `crates/happenstance-testkit/tests/mutation_coverage.rs:33-46`; `xtask/src/main.rs` wasm steps |
| No borrowing GAT, and `Capability` / `RuleOutcome` reused unchanged | The hostile stores are `ProjectionFixture` implementations; no `type Store<'a> where Self: 'a` appears anywhere. Meta-tests assert on `RuleOutcome` / `Verdict` values, never on stdout. | `crates/happenstance-testkit/src/contract.rs:97-111`, `:355-537`; `_decomposition.md` Architecture brief AC-A06 |
| CF-5's projection half is a named deferral, not an empty assertion | `Kind::ConformantVariant` exists in the shape and no row uses it yet; neither the "at least one variant is registered" assertion nor a `conformant_variants_pass_everything` sibling is landed in a form that passes over an empty set. The registry's module doc names `buffering-conformant-variant` as the story that closes it. | `crates/happenstance-testkit/tests/mutation_coverage.rs:2852-2859`, `:3071-3121`; `_storymap.md` slice 6 |

## Data and migrations

**N/A.** This story adds no persistent schema, no stored format and no migration. Everything it
lands is in-process test code: two hostile `ProjectionStore` implementations whose backing store is
an in-memory map, and a `const` table of claims about them. The one shape that could be mistaken for
data — `REGISTRY` — is deliberately *not* a serialised or generated artefact, because a mutant
registry read back from a snapshot asserts nothing
(`crates/happenstance-testkit/tests/mutation_coverage.rs:127-135`). Nothing in this PR reads or
writes a file at run time, so there is no fixture data, no seed and no rollback step.

## Acceptance criteria

The persona is **the adapter author** (`.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md:114-181`),
at step 3 of their journey — *"runs the suite"* — and their goal is not "a green run" but
*"I can tell whether a green run means anything."* Every criterion below is stated from that goal
and crosses the whole stack it needs: a real `ProjectionStore` implementation, driven through the
real `projection_store_conformance!` entry point, observed by a real meta-test, run by the real gate.
A criterion that could be satisfied by inspecting source alone is marked as such and says why.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **GIVEN** an adapter author who has just wired a store that advances the checkpoint but forgets to flush its read-model writes, **WHEN** they run `projection_store_conformance!` against it, **THEN** the run fails at a **named** rule — `commit_is_atomic_with_the_read_model` — and the failure message identifies the store, not merely "1 test failed". Concretely: `CheckpointOnlyStore` is a real `ProjectionStore` + `ProjectionProbe` + `ProjectionFixture` implementation driven `begin` → `probe_write` → `commit` → *fresh handle* → `probe_read` + `checkpoint`, and it fails at the both-present-or-both-absent assertion and nowhere else. | **Integration.** `crates/happenstance-testkit/tests/projection_mutation_coverage.rs` — `run_subject::<CheckpointOnlyStore>()` returns `Verdict::Panicked` for `commit_is_atomic_with_the_read_model`, asserted by `projection_mutation_coverage::projection_mutants_fail_exactly_their_declared_rules`. Manual falsifier at review: deleting the one line of that store's `commit` that suppresses the write set turns the failure green. |
| AC-002 | **GIVEN** the same author, **WHEN** the suite reports that failure, **THEN** they can trust it is a *diagnosis* and not noise — `CheckpointOnlyStore` **passes** every projection rule it does not declare, so the one red rule is the one true statement about the defect. A skip is never counted as one of those passes: a rule that skipped for any reason other than a `(capability, reason)` pair this fixture itself declined fails the check as a hole in the map. | **Unit** over **Integration** results. `projection_mutation_coverage::projection_mutants_fail_exactly_their_declared_rules`, undeclared direction — every non-declared `(store, rule)` cell must be `Verdict::Passed`, or `Verdict::Skipped` whose `(capability, reason)` appears in the fixture's own declines list (`crates/happenstance-testkit/tests/mutation_coverage.rs:2908-2935` is the shape). |
| AC-003 | **GIVEN** an author reading `spec/SPECIFICATION.md` §4.11 and seeing an em-dash where `commit_advances_the_checkpoint`'s rejecting store should be, **WHEN** they ask what would fail that rule, **THEN** the registry answers with a store rather than a shrug: a `commit` that returns `Ok` and makes **neither** write durable — which satisfies PS-1's "or not at all" arm, therefore **passes** `commit_is_atomic_with_the_read_model` and **fails** `commit_advances_the_checkpoint`. It is the one store that separates the two rules. Its `provenance` records that this shape is also the evidence `.kb/open-questions/ps-1-states-no-progress-obligation.md` describes, **without** proposing a clause change. | **Integration** + **Unit.** Same binary; the store is driven through both baseline rules and its two-cell verdict pattern is asserted by `projection_mutation_coverage::projection_mutants_fail_exactly_their_declared_rules`. Review check: the `PS-1` observation appears in `provenance` and **no** diff touches `spec/SPECIFICATION.md`'s PS-1 clause. |
| AC-004 | **GIVEN** an author who already knows the event-store mutant registry, **WHEN** they open the projection one, **THEN** they read *one* shape, not a second dialect: a hand-written `const REGISTRY: &[Declared]` carrying `name`, `kind`, `fails`, `provenance`, `mode`, `expect` with the same semantics, `name` matching the store's own `Subject::NAME`, `fails` exact and empty **iff** `Kind::ConformantVariant`, and `expect: &[(rule, substring)]` keyed **by rule** from the first commit rather than `Option<&'static str>` retrofitted later. It is never generated from observed outcomes and never an associated const on the store's own impl. | **Static** (review of the declared shape against `crates/happenstance-testkit/tests/mutation_coverage.rs:125-186`) + **Unit** — the keyed-`expect` claim is enforced, not merely written: `projection_mutation_coverage::projection_mutant_registry_is_exhaustive` rejects any `expect` pin naming a rule the mutant does not declare. |
| AC-005 | **GIVEN** the author's slice-mate about to add a new projection rule, **WHEN** they add it to `for_each_projection_store_rule!` without a store that fails it, **THEN** the build goes **red before their PR merges**, naming every uncovered rule (all of them, not the first) and pointing at where to write the wrong implementation. The rule universe comes from the single enumeration via `__emit_rule_names`, never a second hand-kept list, and the test has **no exemption list and must never acquire one**. | **Unit.** `projection_mutation_coverage::every_projection_rule_has_a_mutant`. Falsifier available today: temporarily adding a name to `for_each_projection_store_rule!` must turn it red. Sibling shape at `crates/happenstance-testkit/tests/mutation_coverage.rs:2711-2749`, universe derivation at `:2803-2810`. |
| AC-006 | **GIVEN** an author adding a store to the projection binary, **WHEN** they get one of the three edits wrong — the type is enumerated but unregistered, registered but not enumerated, named twice, enumerated twice, declared to fail a misspelled rule, or pinned to a rule it does not declare — **THEN** the registry says which row and which field, rather than the store silently going undriven or a claim silently going unchecked. `kind` must also agree with `fails` being empty or not. | **Unit.** `projection_mutation_coverage::projection_mutant_registry_is_exhaustive`. Every assertion in it is inherited from a defect the event-store family actually hit (`crates/happenstance-testkit/tests/mutation_coverage.rs:2751-2850`); the message names the offending row(s), not just a count. |
| AC-007 | **GIVEN** an author who suspects a mutant is "failing for the wrong reason", **WHEN** the exactness meta-test runs, **THEN** it distinguishes *the rule's assertion fired* from *the store panicked* **positively** — under `FailureMode::Assertion` the panic origin must be inside the projection rules file and the message must not match `RUNTIME_PANICS`; under `FailureMode::StorePanic` the origin must be outside it and the message must contain the declared `expect` needle. A denylist over panic text is not sufficient and is not what is written: deleting a mutant's modelled defect once made the panic come from the fixture contract's own "not implemented" body, which a denylist passed silently. Assertions are on `Verdict` values, never on stdout. | **Unit** over **Integration** results. `projection_mutation_coverage::projection_mutants_fail_exactly_their_declared_rules`, declared direction; the `Origin` positive check is authored against the projection rules module's path. Measured precedent: `crates/happenstance-testkit/tests/mutation_coverage.rs:2966-2999`. |
| AC-008 | **GIVEN** a reviewer asking whether the mutant set is *representative* or merely *large*, **WHEN** they read any row, **THEN** they find a non-empty `provenance` naming the adapter shape or scenario that makes the defect plausible — a mistake an author would really make — because `struct AlwaysWrong` satisfies CF-1 mechanically and proves nothing. This holds for conformant-variant rows too: a variant owes an account of why it is *legally* different or it is a second copy of the reference store. | **Unit.** `projection_mutation_coverage::every_projection_mutant_states_its_provenance` — trimmed `provenance` non-empty for every row, no `Kind` exemption. Sibling at `crates/happenstance-testkit/tests/mutation_coverage.rs:3047-3069`; clause CF-4 at `spec/SPECIFICATION.md:7209-7220`. |
| AC-009 | **GIVEN** an author or reader trying to judge the projection suite's strength, **WHEN** they read the registry's module doc or any output of the binary, **THEN** they are told **which defects the set covers and which axes it leaves uncovered**, and are never handed a fraction: ADR-0010's prohibition is stated in the module doc and nothing in the binary computes or prints a pass rate over the mutant set. In the same doc, CF-5's projection half is a **named** deferral — `Kind::ConformantVariant` exists in the shape, no row uses it yet, `buffering-conformant-variant` is named as the story that closes it, and neither the "at least one variant is registered" assertion nor a `conformant_variants_pass_everything` sibling is landed in a form that would pass over an empty set. | **Static (review of an absence).** There is nothing to assert against a fraction that was not printed, so this is a doc-comment review — as the Testing brief states for project AC-003 (`_decomposition.md` Testing brief, AC-003 row). Backed mechanically by a repo grep for a percentage/ratio computed over `REGISTRY` in the new binary returning empty. Precedent text: `crates/happenstance-testkit/tests/mutation_coverage.rs:26-31`, `:203-206`; deferral shape: `:2852-2859`, `:3071-3121`. |
| AC-010 | **GIVEN** an author who reads the event-store registry first (it is the older and larger one), **WHEN** they reach its scope note, **THEN** it no longer tells them the projection port "has neither a suite nor a mutant yet" — a sentence this PR makes false — but points at `tests/projection_mutation_coverage.rs` while keeping the "Scope: the event-store port only" framing intact. | **Static.** Review of the diff at `crates/happenstance-testkit/tests/mutation_coverage.rs:188-206`; `cargo test --workspace --all-features` proves the edit did not disturb the event-store binary. Required by `_decomposition.md` Architecture brief AC-A07. |
| AC-011 | **GIVEN** the same author targeting Cloudflare Workers, **WHEN** the workspace gate runs, **THEN** the new binary does not break their target: it carries one crate-level `#![cfg(not(target_arch = "wasm32"))]` — because `catch_unwind` cannot catch where there is no unwinder, and a meta-test that cannot observe a panic is the whole mechanism gone — **and states the cost**, that the stores in it are never type-checked for `wasm32`. In the same breath, the hostile stores reuse `Capability` and `RuleOutcome` unchanged and declare **no** borrowing GAT anywhere (`type Store<'a> where Self: 'a` on a foreign trait is one ingredient of a rustc ICE this repository already minimised and which still reproduces on 1.97.1). | **Static** + **E2E.** `cargo check -p happenstance-testkit --tests --target wasm32-unknown-unknown` (the mandatory wasm conformance-harness step in `xtask/src/main.rs`) stays green; `cargo xtask ci --fast` green at the story grain. GAT absence is a review + compile fact (`crates/happenstance-testkit/src/contract.rs:97-111`); reuse-unchanged is AC-A06, checked by the absence of any diff to `crates/happenstance-testkit/src/contract.rs:355-537`. |

Every project AC this story traces to is covered: **project AC-002** by AC-001, AC-002 and AC-007
(fails by name; passes what it does not declare; fails for the *right* reason), and **project AC-003**
by AC-003, AC-004, AC-005, AC-006, AC-008 and AC-009 (every rule has a mutant, provenance stated, no
pass rate quoted). AC-010 and AC-011 are the wiring and gate obligations that make the rest
observable rather than local.

## Interaction quality

This project's signed-off `_design.md` declares `surfaces: []` and records the **no-surface
determination itself** as the thing that was approved
(`.bklg/from-contract-to-published-library/projection-store-freeze/_design.md:48-50, 96-101`). So the
**composition family is N/A here** — there is no control, no placement, no density budget and no
transience policy, because there is nothing rendered. That is a determination this story consumes; it
does not re-open it, and it must not invent a surface to have one.

What *is* live is the other non-visual surface `_design.md` does name: **text a human reads out of a
CI log** (`_design.md:33-38`). For this story that text is a meta-test's assertion message and a
conformance run's named failure — the only thing the adapter author actually meets. The state-family
invariants apply to it in their honest translation, and **each one is carried by an AC row in the
table above**, not by a bullet here:

| Invariant (state family) | Its honest translation for a CI-log surface | Carried by |
| --- | --- | --- |
| **In place, not a context jump** | The failure surfaces where the author already is — a named test in `cargo test` / `cargo xtask ci` output — never as a document they must be told to go and read. The rule name *is* the diagnosis. | AC-001, AC-005 |
| **Non-occlusion** | One failure never hides the others: CF-1's message lists **every** uncovered rule and the exhaustiveness check names **every** offending row, rather than aborting at the first. A first-failure-only message makes the second defect invisible until the first is fixed, which is occlusion in a log. | AC-005, AC-006 |
| **Preserved context (the log's equivalent of focus/selection)** | The message carries the store name, the rule name and `Verdict::describe()`'s rendering of what actually happened, so the reader never has to re-derive which of the *N × M* `(store, rule)` cells failed. | AC-001, AC-007 |
| **Reversibility / falsifiability** | The claim is reversible by hand in one step: deleting the single line of `CheckpointOnlyStore::commit` that suppresses the write set turns the red rule green — which is what makes it a demonstration rather than an assertion. Stated as the Merge DoD one-liner. | AC-001 |
| **Reachability (the log's equivalent of keyboard reach)** | Every meta-test is reachable by its own name — `cargo test projection_mutation_coverage::<name>` resolves, because the meta-tests live in a `mod` whose name matches the binary, exactly as the event-store family does it (`crates/happenstance-testkit/tests/mutation_coverage.rs:2695-2699`). Nothing is reachable only by running the whole gate. | AC-005, AC-006, AC-007, AC-008 |

**Named anti-patterns, inherited rather than invented.** `_design.md` records no per-surface
anti-pattern list (it has no surface), so the binding ones here come from ADR-0010 and CF-1 – CF-4 and
are each an AC row: quoting a **pass rate** over the mutant set (AC-009); an **exemption list** on the
CF-1 test, which is CF-1 observed rather than enforced (AC-005); a **saboteur** with no plausible
provenance (AC-008); a **denylist** standing in for the positive origin check (AC-007); and a
**positive control asserting over an empty set**, which is the vacuity CF-5 exists to prevent
reintroduced one level up (AC-009).

## Error conditions

These are the states the instrument itself must fail loudly in. Each is a *deliberate* red, not a
bug: the whole point of this story is that certain mistakes become unlandable.

| id | condition | required behaviour |
| --- | --- | --- |
| EC-001 | A new rule is added to `for_each_projection_store_rule!` with no `Kind::Mutant` row declaring it | `every_projection_rule_has_a_mutant` fails, listing every uncovered rule and pointing at where to write the wrong implementation. **No exemption list is added.** This is the forcing function the slice-mate and both `reset-and-rebuild-rules` stories inherit by design (`_storymap.md` Coverage, AC-003 row). |
| EC-002 | A `fails` entry names a rule that does not exist (a typo) | `projection_mutant_registry_is_exhaustive` fails naming the row and the unknown name. Invisible to every other meta-test — a typo'd rule name silently covers nothing and CF-1 would still be red for the real rule, which reads as an unrelated failure. |
| EC-003 | An `expect` pin names a rule the mutant does not declare in `fails` | Exhaustiveness fails. An unreachable pin reads as coverage and is not; it is a claim no code path can ever check. |
| EC-004 | A store is enumerated but has no `REGISTRY` row, or has a row but is not enumerated; or a row name is duplicated; or a type is enumerated twice | Exhaustiveness fails naming the row/type. The two lists are the ones that drift (`crates/happenstance-testkit/tests/mutation_coverage.rs:188-193`); a shadowed duplicate row silently loses a claim and a doubly-enumerated type is driven and counted twice. |
| EC-005 | A **declared** `(store, rule)` cell comes back `Verdict::Passed` | Exactness fails: the declaration is wrong, or the defect no longer exists. Never softened to a warning. |
| EC-006 | A **declared** cell comes back `Verdict::Skipped` | Exactness fails — a declared failure that skips is **a hole in the map, never a pass**. It means the rule stopped running and nothing noticed (`crates/happenstance-testkit/tests/mutation_coverage.rs:2939-2964`). |
| EC-007 | An **undeclared** cell comes back `Verdict::Skipped` with a `(capability, reason)` the fixture does **not** itself decline | Exactness fails. The single accounted-for exception is a skip citing the fixture's own declared decline; a skip from anywhere else is the vacuity this instrument exists to detect (`:2871-2887`, `:2908-2935`). |
| EC-008 | Under `FailureMode::Assertion`, the panic originated **outside** the projection rules file, or its message matches `RUNTIME_PANICS` | Exactness fails: the mutant failed for the wrong reason. Checked **positively** by origin, never by message denylist (`:2966-2999`). |
| EC-009 | Under `FailureMode::StorePanic`, the panic originated inside the projection rules file, or the message lacks the declared needle | Exactness fails, symmetrically. |
| EC-010 | Any `REGISTRY` row has empty or whitespace-only `provenance` | `every_projection_mutant_states_its_provenance` fails, `Kind::ConformantVariant` rows included. |
| EC-011 | A `Kind::Mutant` row has an empty `fails`, or a `Kind::ConformantVariant` row has a non-empty one | Exhaustiveness fails — `kind` and `fails` must agree, or a mutant that declares nothing passes every check vacuously. |
| EC-012 | The new binary is compiled for `wasm32-unknown-unknown` | It compiles to nothing: the crate-level `#![cfg(not(target_arch = "wasm32"))]` removes it, and the module doc states the cost (its stores are never type-checked for that target). Without the gate, `cargo check -p happenstance-testkit --tests --target wasm32-unknown-unknown` — a **mandatory** `cargo xtask ci` step — fails on `catch_unwind`. |
| EC-013 | `for_each_projection_store_rule!` or `__emit_rule_names` is not yet available in the shape this story assumes | **Stop and report**, do not hand-write a second rule list. The enumeration is `projection-suite-entry-point`'s deliverable and Note 4 of the Architecture brief is where its emitter question is settled; a local rule list would make CF-1 go green on the day it must go red (Decision 4). |

## Non-functional

| id | requirement | why, and how it is observed |
| --- | --- | --- |
| NF-001 | **No new dependency.** The binary adds nothing to `happenstance-testkit`'s `[dependencies]` or `[dev-dependencies]`, and no `[[test]]` section or `autotests = false` to its `Cargo.toml`. | Mounting is by cargo target auto-discovery (Integration contract). `cargo deny` and `cargo hack check --workspace --feature-powerset --no-dev-deps` stay green with no new combination to explain. |
| NF-002 | **No public surface, no semver promise.** Nothing lands in any crate's `lib.rs` export block or feature table; every item is test-target-private, which is where CF-1 requires mutant stores to live (`spec/SPECIFICATION.md:7161-7163`). | `cargo package --list` for the three publishable crates is unchanged; `cargo doc` gains no item. |
| NF-003 | **Deterministic and self-contained.** No wall-clock dependence, no randomness, no sleeps, no ordering dependence between meta-tests, no shared mutable global between subjects — each `Subject::open()` is called **inside** the caught closure so every rule gets a fresh, isolated backing store (`crates/happenstance-testkit/tests/mutation_coverage/harness.rs:66-70`). | Repeated `cargo test -p happenstance-testkit` runs produce identical results; the meta-tests pass under cargo's default parallel test threads. |
| NF-004 | **Quiet under expected panics.** The binary installs the same quiet panic hook the event-store family uses, so a run in which every mutant fails as designed does not bury the real output in backtraces. | `crates/happenstance-testkit/tests/mutation_coverage/harness.rs:286-311`; observed by reading one `cargo xtask ci --fast` run's output. |
| NF-005 | **Gate-neutral cost.** The binary drives *every registered store* through *every projection rule* — an N × M product that grows with each later story in this project — so it must stay in-memory and allocation-cheap, with no I/O and no per-cell process spawn. At today's 2 stores × 2 rules and the slice's projected ~4 stores × 9 rules, the added wall-clock in `cargo test --workspace --all-features` must remain in the noise. | Compared against the same command's runtime on `main` at review; the event-store binary at far larger N × M is the existence proof. |
| NF-006 | **House-style clean.** `cargo fmt --check` and `cargo clippy --workspace --all-targets --all-features -D warnings` green, on MSRV 1.97.1; every non-obvious construct carries the *why*, per `standards/rust/` — in particular the `#[path]` module attributes, which look gratuitous until the reason is stated. | `cargo xtask ci --fast`. |
| NF-007 | **`spec-trace` untouched and unbroken.** No `[FROZEN]` clause is edited; the new meta-test names need no `Rule:` citation because `spec-trace` marks any `Rule:` line containing `meta-test` as `elsewhere` and does not resolve it. | `cargo xtask spec-trace` green (`xtask/src/spec_trace.rs:1601-1625`) — verify the claim before relying on it. |

## Implementation notes (non-prescriptive)

Directions, not instructions. Where the event-store family already made a choice and paid for it, the
note says so; where it is genuinely open, the note says that instead and asks for the reasoning in the
ledger.

- **Read the event-store binary before writing a line of this one.** Not to copy it — to notice which
  of its assertions look redundant and are not. Each of the exhaustiveness checks exists because
  something drifted; the six-assertion shape is a scar map, and re-deriving it from first principles
  reliably produces four of the six.
- **Open question left to the implementer: duplicate or share `Declared` / `Kind` / `FailureMode`.**
  Extracting them into a `#[path]`-shared module makes the two families provably one shape and
  couples their evolution; duplicating them keeps the families independent and risks silent drift in
  semantics the meta-tests do not compare. Both are defensible. Field **names and semantics must
  match either way** so a reviewer reads one shape across both families — that part is not open.
  Record the choice and the reason in the ledger's evidence for AC-004.
- **Write `CheckpointOnlyStore` as the smallest honest store, not the smallest store.** It must be a
  real `ProjectionStore` + `ProjectionProbe` implementation over an in-memory map that genuinely
  commits a checkpoint; a stub returning canned answers would pass or fail by construction and prove
  nothing about the rule. The Testing brief classifies this row Integration for exactly that reason.
- **The `Origin` positive check needs the projection rules module's real path.** Take it from what
  `projection-suite-entry-point` actually landed rather than assuming a filename; getting this wrong
  is silent — the check still compiles and simply never matches, which is the vacuity of
  `:2966-2999` reintroduced.
- **Order within the PR.** Land the registry shape and the four meta-tests *before* the second
  mutant, so that the first red you see is CF-1 telling you `commit_advances_the_checkpoint` has no
  mutant. Watching the instrument demand the store is worth more than reading that it would.
- **Resist two adjacent repairs.** PS-1's progress obligation (Decision 6) and any CF-1 – CF-6
  amendment (Decision 9) are both out of scope and both owned elsewhere. If either looks necessary
  rather than tempting, that is a finding to report at the story boundary, not a diff.
- **The module doc is a deliverable, not decoration.** Three things must be in it and are checked by
  reading: ADR-0010's no-pass-rate prohibition, the covered/uncovered axes account, and the named CF-5
  deferral pointing at `buffering-conformant-variant`.

## Tests and CI (merge gate)

Tier vocabulary is the project Testing brief's
(`.bklg/from-contract-to-published-library/projection-store-freeze/_decomposition.md:753-767`):
**Static** reads source/config without executing the code under test; **Unit** is an in-process
`#[test]`, including meta-tests over registries; **Integration** is a conformance rule actually
driving a `ProjectionStore` through `begin` / write / `commit` / read-back; **E2E** is `cargo xtask ci`
run whole.

| tier | command / path | proves |
| --- | --- | --- |
| Integration | `cargo test -p happenstance-testkit --all-features --test projection_mutation_coverage` → `crates/happenstance-testkit/tests/projection_mutation_coverage.rs` | AC-001, AC-003 — the hostile stores are really driven through `projection_store_conformance!`'s rules (`begin` → `probe_write` → `commit` → fresh handle → `probe_read` + `checkpoint`), so the failure is observed by execution, not by inspection (Testing brief, AC-002 row). |
| Unit | `cargo test -p happenstance-testkit --all-features projection_mutation_coverage::every_projection_rule_has_a_mutant` | AC-005 — CF-1 for the projection family, universe taken from `for_each_projection_store_rule!` + `__emit_rule_names`, no exemption list. |
| Unit | `… projection_mutation_coverage::projection_mutant_registry_is_exhaustive` | AC-004, AC-006 — CF-2: enumeration ↔ registry agreement, no duplicates, every `fails` name real, every `expect` pin reachable, `kind` agrees with `fails`. |
| Unit | `… projection_mutation_coverage::projection_mutants_fail_exactly_their_declared_rules` | AC-002, AC-007 — CF-3 both directions, with the positive `Origin` check and the accounted-for skip exception. Asserts on `Verdict`, never stdout. |
| Unit | `… projection_mutation_coverage::every_projection_mutant_states_its_provenance` | AC-008 — CF-4: non-empty provenance on every row, conformant variants included. |
| Unit (regression) | `cargo test --workspace --all-features` | AC-010 — the event-store binary still passes after its scope note is corrected; nothing else in the workspace moved. |
| Static | Review of `crates/happenstance-testkit/tests/projection_mutation_coverage.rs`'s module doc; repo grep for a fraction computed over the projection `REGISTRY` returns empty | AC-009 — the no-pass-rate prohibition and the named CF-5 deferral. An absence has nothing to assert against, so the Testing brief makes this a review-time check (AC-003 row) rather than a runtime one. |
| Static | Review of the diff at `crates/happenstance-testkit/tests/mutation_coverage.rs:188-206` | AC-010 — the "neither a suite nor a mutant yet" sentence is replaced and the "Scope: the event-store port only" framing survives. |
| Static | `cargo check -p happenstance-testkit --tests --target wasm32-unknown-unknown` (mandatory step inside `cargo xtask ci`, `xtask/src/main.rs`) | AC-011 — the `wasm32` gate is present and correct. This step already exists; the story costs it nothing new. |
| Static | `cargo fmt --check`; `cargo clippy --workspace --all-targets --all-features -D warnings` | NF-006. |
| Static | `cargo xtask spec-trace` | NF-007 — no `[FROZEN]` clause moved and the new meta-test names resolve as `elsewhere`. |
| E2E (story grain) | `cargo xtask affected --base main` then `cargo xtask ci --fast` | The merge bar for a non-terminal project's story (`CLAUDE.md` Commands; Testing brief, "Merge-gate commands"). |
| E2E (project grain — **not** this story's bar) | `cargo xtask ci` whole | Recorded here only so it is not mistaken for a story obligation: project AC-013/AC-014/AC-016 are proven by the two steps `--fast` omits, and that whole-gate run belongs to `whole-gate-run-and-proof-artefact`. |

**Merge gate for this story**: `cargo xtask affected --base main` green, `cargo xtask ci --fast`
green, every `_ledger.md` row `satisfied: true` with cited evidence, and a reviewer able to name the
test `CheckpointOnlyStore` fails and watch it go green by deleting one line of that store's `commit`.

## Risks and coupling (PR-scoped)

| risk | why it is real here | containment inside this PR |
| --- | --- | --- |
| **The shared-versus-duplicated `Declared` shape is decided by momentum.** | It is the one genuinely open design choice in the PR, and the easy default (copy-paste) is also the one that drifts silently — the meta-tests compare a registry against an enumeration, never one family's field semantics against the other's. | Named as an explicit implementer decision (Implementation notes) with the reasoning recorded in the ledger's AC-004 evidence. Field names and semantics matching is non-negotiable either way. |
| **PS-1's defect is demonstrated in this PR and repaired in another.** | The second mutant is *precisely* the evidence `.kb/open-questions/ps-1-states-no-progress-obligation.md` wants, and it will be sitting compiled in front of the implementer. Fixing a `[FROZEN]` clause with the demonstration in hand feels like diligence. | Decision 6 forbids it; the observation goes in `provenance`; `unstable-projection-gate-and-clause-disposition` and `ps-clause-pairing-sweep` own the repair. A diff touching PS-1 fails review regardless of gate colour. |
| **The forcing function lands on the slice-mate, in the same context.** | From merge, `commit-rollback-and-drop-rules` cannot add any of its seven rules without a mutant each. Under time pressure the cheapest escape is an exemption list or a saboteur store. | This is the **intended** coupling, stated in `_storymap.md`'s Coverage AC-003 row; AC-005 and AC-008 are what make both escapes fail. ADR-0010's honest alternative — retire the rule and record why — is written into Decision 5 so it is available when needed. |
| **`for_each_projection_store_rule!` / `__emit_rule_names` may not have the exact shape assumed.** | This story is the first consumer of a macro landed one story earlier, and the Architecture brief's Note 4 left the emitter question to that story. | EC-013: stop and report rather than hand-write a rule list. A second list would make CF-1 green on the day it must be red — the failure mode is silent, which is why it is a halt rather than a workaround. |
| **`Origin`'s positive check silently never matches.** | It compares against a module path; a wrong path compiles fine and simply classifies every assertion panic as "not from the rules file", which either fails everything loudly (fine) or, if inverted, passes everything (not fine). | Verify against the real path `projection-suite-entry-point` landed, and confirm the check *fails* when a mutant's defect is temporarily removed — the falsifier the event-store family learned to run (`:2966-2999`). |
| **An ungated binary breaks the wasm gate for the whole workspace.** | `catch_unwind` has no unwinder on `wasm32`, and the conformance-harness wasm check is a **mandatory** `cargo xtask ci` step, not a skip-if-absent one. | AC-011; a single crate-level `#![cfg(not(target_arch = "wasm32"))]` at the top of the binary, with the cost stated in the same doc comment. |
| **Touching the event-store registry at all.** | AC-010 requires editing one doc comment inside a 3,526-line file whose surrounding assertions are load-bearing. | Scope note only. The PR boundary lists "no re-declaration, no re-scoping of `Declared`, no touching `for_each_mutant!`" explicitly, and `cargo test --workspace --all-features` is the regression check. |
| **`_design.md` binds by its absence, which is easy to over-read.** | A story that renders nothing can drift into inventing output "for the user". | The design's approved determination is `surfaces: []`; the Interaction quality section translates the state-family invariants onto the CI-log text surface `_design.md` *does* name and adds no other. |

## Dependencies

**Blocks on** (must be merged first):

- **`projection-suite-entry-point`** (HS-S0007) — matches this story's `depends_on` exactly. It lands
  `ProjectionFixture` in the testkit's `contract.rs`, the single enumeration
  `for_each_projection_store_rule!` with `__emit_rule_names`, `projection_store_conformance!` and its
  `__private` export, and the two baseline rules `commit_advances_the_checkpoint` and
  `commit_is_atomic_with_the_read_model`. Without it there is no rule universe to take a mutant
  registry's denominator from and no entry point to drive a hostile store through.

Transitively (through that story, not directly declared here): `owned-batch-port-shape`,
`projection-probe-conformance-feature` and `memory-projection-store` — the port, the probe that makes
`CheckpointOnlyStore`'s defect observable at all, and the oracle each hostile store is a named
deviation from.

**Unlocks:**

- **`commit-rollback-and-drop-rules`** (slice-mate, same milestone) — its seven commit-side rules each
  need a registry row, and the exactness meta-tests are what force them.
- **`reset-rules`** and **`read-through-and-rebuild-rules`** (slice 5) — both name this story in their
  `depends_on` for the same reason.
- **`buffering-conformant-variant`** (slice 6) — inherits the `Kind::ConformantVariant` shape landed
  here and closes the CF-5 hole this story names in the module doc.
- **`whole-gate-run-and-proof-artefact`** (slice 8) — its proof artefact quotes the name of the test
  `CheckpointOnlyStore` fails, which is AC-001's output.

## Anchors (progressive disclosure)

Deferred depth, not optional depth. Each row says why the artefact is load-bearing and the moment to
open it; nothing here is pasted into the Context pack in bulk. Every path was confirmed to exist.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `crates/happenstance-testkit/tests/mutation_coverage.rs` | The sibling registry in full: `Declared` / `Kind` / `FailureMode` at `:73-186`, the scope note to correct at `:188-206`, and the four meta-tests at `:2711-3069`. Its assertions are a scar map — each exists because something drifted — and re-deriving them from first principles reliably produces fewer. | **Before writing any line of the new binary**, and again at `:188-206` when making the AC-010 edit. | AC-004, AC-005, AC-006, AC-007, AC-008, AC-010 |
| `crates/happenstance-testkit/tests/mutation_coverage/harness.rs` | The machinery to mirror: `Subject` with `open()` called *inside* the caught closure (`:56-70`), the three-outcome `Verdict` and why conflating a skip breaks CF-3 (`:84-112`), the quiet panic hook (`:286-311`), and `declines()` (`:536`), which is what makes the accounted-for-skip exception checkable. | **Before implementing the projection `run_subject` and `Verdict`** — AC-002's and AC-007's assertions are defined in terms of these types. | AC-001, AC-002, AC-007 |
| `crates/happenstance-testkit/tests/mutation_coverage/mutants.rs` | How a hostile store is actually written in this repository — minimal, real, and a *named deviation* from the reference store rather than a stub. Read one before writing `CheckpointOnlyStore`. | **Before writing the two hostile stores** for AC-001 and AC-003. | AC-001, AC-003 |
| `crates/happenstance-testkit/src/contract.rs` | `Capability` and `RuleOutcome` at `:355-537`, reused **unchanged** per AC-A06; and at `:97-111` the recorded reason no borrowing GAT may appear — an ingredient of a rustc ICE this repo minimised and which still reproduces on 1.97.1. | **Before declaring the hostile stores' `ProjectionFixture` impls**; the GAT prohibition is easiest to violate while writing an associated type. | AC-011 |
| `.kb/decisions/0010-the-suite-must-prove-itself.md` | The accepted decision this story is the second instance of: why the registry is data and hand-written, why a pass rate is forbidden, and the honest response when a rule has no plausible failing implementation (retire the rule, never invent a saboteur). | **Before writing the module doc** (AC-009) and again if AC-005's forcing function starts to feel like an obstacle. | AC-005, AC-008, AC-009 |
| `spec/SPECIFICATION.md` | The clauses being discharged, in their own words: §4.11's `CheckpointOnlyStore` obligation and the em-dash row at `:5659`, `:5678-5693`; PS-1's "or not at all" arm at `:4744-4759`; CF-1 – CF-4 at `:7161-7220`. Clause text wins over any summary in this spec. | **Before AC-003** especially — its whole justification is that CF-1 admits no exemption where §4.11 shows an em-dash. | AC-001, AC-003, AC-005, AC-006, AC-007, AC-008 |
| `.kb/open-questions/ps-1-states-no-progress-obligation.md` | The open question the second mutant is evidence *for*, and the reason this story records rather than repairs. Read it so the boundary is a decision, not an oversight. | **When writing the second mutant's `provenance` string** — and then close the file. | AC-003 |
| `.bklg/from-contract-to-published-library/projection-store-freeze/_decomposition.md` | Architecture brief AC-A06/AC-A07 and Notes 6, 8 and 9 (the rule shape, the neither-write-durable store, the hostile-store set); Testing brief rows AC-002 and AC-003 with the tier assignment and the "no test tier proves an absence" reasoning at `:731-786`. | **At spec-read time for the brief's AC-A07 shape requirement**, and again when justifying AC-009's Static tier. | AC-003, AC-004, AC-009, AC-010, AC-011 |
| `.bklg/from-contract-to-published-library/projection-store-freeze/_storymap.md` | The slice boundary and the coupling this story deliberately creates: slice 4's ordering (`:110`) and the Coverage AC-003 row (`:81`) stating that each later rule story carries its own mutants *because* the exhaustiveness meta-test forces them. | **When tempted to land a rule here, or to soften AC-005 for a slice-mate.** | AC-005 |
| `.bklg/from-contract-to-published-library/projection-store-freeze/_design.md` | The signed-off no-surface determination (`surfaces: []`, `:48-50`) and the one text surface a human actually meets (`:33-38`), approved 2026-08-12. It binds this story by what it forbids inventing. | **Before writing anything that produces output for a human** — including the meta-tests' assertion messages. | AC-009 |
| `.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md` | The adapter author at `:114-181`, step 3 "runs the suite", and the stated fear this story addresses: finding out late that the port assumed something their storage cannot provide. Every AC above is framed from it. | **When an AC's framing looks like a bare capability** — return here to restate it as the author's goal. | AC-001, AC-002 |
| `RUNBOOK.md` | Phase 6's own bar in the runbook's words — the probe's necessity and *"if it passes, the port is not frozen"* (`:3882-3892`). The plan of record; where it and a brief disagree, report rather than choose. | **At the start**, to see the bar stated by the phase that owns it. | AC-001 |
| `xtask/src/main.rs` | The gate as actually defined, including the mandatory wasm32 conformance-harness step this story must not break and the `tests` step that auto-discovers the new target. | **Before assuming the new file needs a manifest entry or a new CI step** — it needs neither. | AC-011 |
| `xtask/src/spec_trace.rs` | The `meta-test` → `elsewhere` handling at `:1601-1625` — the mechanism that makes "no `[FROZEN]` clause needs amending" checkable rather than hoped for. | **If the temptation to add a `Rule:` citation for a new meta-test arises.** | AC-011 |
| `experiments/rustc-ice-gat-foreign-trait/README.md` | The minimised ICE behind AC-A06's no-borrowing-GAT prohibition, with its reproduction. Evidence, not a rule — open it only if the prohibition looks like superstition. | **Only if you are about to write `type Store<'a> where Self: 'a`.** | AC-011 |

## Clarifications resolved during spec

1. **AC count is exactly the eleven the first pass enumerated.** Nothing was added or dropped. The
   mapping is: project AC-002 → AC-001, AC-002, AC-007; project AC-003 → AC-003, AC-004, AC-005,
   AC-006, AC-008, AC-009; with AC-010 (the scope-note correction) and AC-011 (the wasm gate, the
   unchanged `Capability`/`RuleOutcome` reuse and the GAT prohibition) carrying the wiring and gate
   obligations the Architecture brief attaches to this story.
2. **Interaction quality has no composition family, and that is a determination rather than an
   omission.** `_design.md` is signed off with `surfaces: []` and records the no-surface finding
   itself as what was approved. The state-family invariants are translated onto the CI-log text
   surface the design *does* name, and every one of them is carried by an AC row in the table — none
   is left as a prose bullet, which would never be extracted, gated or tested.
3. **AC-009 is a Static, review-time criterion by design.** "No pass rate is quoted" is a property of
   what the binary does **not** print, and there is nothing to assert against an absence except
   reading it — the project's Testing brief says so in its AC-003 row. The repo grep is a
   corroborating check, not the proof, and the ledger's evidence for this row is expected to be a
   `file:line` into the module doc plus the reviewer's note.
4. **Whether `Declared` / `Kind` / `FailureMode` are shared or duplicated is left open on purpose.**
   Both are defensible and the meta-tests do not compare the two families' field semantics either
   way, so the choice is the implementer's and the *reasoning* is the deliverable — recorded in the
   ledger's AC-004 evidence. What is not open: field names and semantics must match across the two
   families.
5. **The second mutant is named by behaviour, not by identifier.** The Architecture brief's Note 8
   describes it precisely (a `commit` returning `Ok` that makes neither write durable) but does not
   name it, and the event-store family's names are earned by what the store does. Naming is the
   implementer's; the ledger records the name so later stories can cite it.
6. **`cargo xtask ci --fast` is this story's E2E bar, not `cargo xtask ci`.** The project is
   `terminal: false` and the full gate at the project boundary belongs to
   `whole-gate-run-and-proof-artefact`. The one exception is stated as a Static row rather than
   smuggled in: the wasm32 conformance-harness check must be run for AC-011 even though `--fast`
   omits it, because this story adds the file that could break it.
7. **`spec-trace`'s `meta-test` exemption is stated as a claim to verify, not an assumption.** The
   Context pack cites `xtask/src/spec_trace.rs:1601-1625`; if the behaviour differs from that
   reading, the correct response is to report it at the story boundary — not to amend a `[FROZEN]`
   CF clause to make a citation resolve.
