---
item: HS-S0042
stage: discover
created: 2026-08-12T13:02:05.086Z
updated: 2026-08-12T13:02:05.086Z
template_sig: 86ce4036
rendered_sig: e81e7ac0
---

# Discover — The model family green, and SqliteEventStore in the mutant pass column

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Slice one-line: take `event_store_model_conformance!` green and put `SqliteEventStore` in the mutant harness's pass column, adding the `BEGIN DEFERRED` probe-then-insert row to `mutation_coverage/mutants.rs`'s `REGISTRY` so the racing rules are shown to be falsifiable. | `_storymap.md`, *Slices* table, row `race-model-and-durability` / `model-family-and-mutant-pass-column` | The table named is wrong and this story is where that is discovered — see *The wrong implementation*. The intent (a permanent, re-runnable negative control for the racing rules) is right. |
| **AC-006** — the model family and the mutant harness both accept it: `event_store_model_conformance!` green, and "the phase-3 mutant harness is re-run with `SqliteEventStore` in the pass column". | `project.md`, AC-006; `RUNBOOK.md:4221-4222` | "Pass column" is the phrase to interrogate: the pass column is `conformant_variants_pass_everything`, whose stores are built from `correct.rs`'s primitives inside the testkit's own test binary. |
| **AC-007** — atomic append, probe-then-insert rejected. This story's half is "the registry row that proves the rejection is live". | `project.md`, AC-007; `_storymap.md`, *Coverage* | The implementation is HS-S0037's, the family that rejects the wrong one is HS-S0041's, and the permanent control is this story's. |
| `dependsOn: concurrency-family-and-contender-count` (HS-S0041) — the racing rules must be green against the real adapter before a control that claims to falsify them is meaningful, and the contender count must be settled before a racer is written against it. | `_storymap.md`, *Merge order* item 3 | A control written against 8 contenders and then run at 64 is a control whose window may no longer close. |
| `event_store_model_conformance!` is the proptest family's macro. | `crates/happenstance-testkit/src/model.rs:782` | It is a separate macro with its own enumeration, for the reason `concurrency.rs:138-144` gives: `spec-trace` scans `suite.rs` for `pub async fn`, so a rule written there needs a clause. |
| The five meta-tests: `every_rule_has_a_mutant`, `mutant_registry_is_exhaustive`, `mutants_fail_exactly_their_declared_rules`, `every_mutant_states_its_provenance`, `conformant_variants_pass_everything` — named by string in the proof harness. | `xtask/src/proof.rs:85-90`, `:137` | They run inside `cargo xtask ci --fast`, so this story's obligations are gate-enforced rather than conventional. |
| `every_mutant_states_its_provenance` requires a non-empty provenance naming "the adapter shape or the scenario that makes it plausible". | `crates/happenstance-testkit/tests/mutation_coverage.rs:3059-3068` | A `BEGIN DEFERRED` racer's provenance is this adapter, written from its own architecture brief §6. That is the strongest provenance the registry has ever been able to state. |
| `conformant_variants_pass_everything` carries two assertions: no variant failed, **and** every rule executed against at least one of them — "a control that skipped half the suite is a control over half the suite". | `crates/happenstance-testkit/tests/mutation_coverage.rs:3071-3081` | This is the positive control the AC calls "the pass column", and it is about the testkit's own `Rc`/`RefCell` variants. Reading AC-006 as "register `SqliteEventStore` there" has a real architectural cost. |
| Architecture brief §10's recommended reading: **discharge AC-006 from the adapter's own test target.** Registering `SqliteEventStore` in the testkit's mutation-coverage binary "would make the published testkit dev-depend on an adapter that dev-depends on it, and would drag `rusqlite` plus a tokio runtime into a harness deliberately built to need neither." | `_decomposition.md`, *Architecture brief*, §10; `crates/happenstance-testkit/tests/mutation_coverage/harness.rs:19-35` | If a reviewer insists on testkit-side registration it must be `cfg(not(target_arch = "wasm32"))` dev-dependencies, and ADR-0022's option (a) becomes mandatory rather than recommended. |
| The harness's no-hang mitigation is **structural**: every store in the binary is built from `correct.rs`'s primitives with one step perturbed, and none of them blocks — "there is no channel, no I/O and no custom `Future` in the correct core, so a mutant can only hang by introducing one deliberately." There is deliberately no watchdog. | `crates/happenstance-testkit/tests/mutation_coverage/harness.rs:19-35` | Introducing a `rusqlite`-backed store into that binary removes the property the whole mitigation rests on. This is the concrete cost, not a stylistic objection. |
| A concurrency mutant goes in `racers.rs` and `RACERS`, **not** `mutants.rs`/`REGISTRY`: "`mutant_registry_is_exhaustive` rejects a row whose `fails` list is empty, so a mutant cannot land before the rule that catches it. Every store here fails **no** rule of the event-store family — that is the point of them." | `crates/happenstance-testkit/tests/mutation_coverage/racers.rs:10-18`; `crates/happenstance-testkit/README.md:108-113` | The storymap's instruction to add the `BEGIN DEFERRED` row to `REGISTRY` will fail its own meta-test. Correcting it is this story's, in the same change. |
| Adding a racer is three steps: write the store here wrong in exactly one way, `Send + Sync`, window closed by a rendezvous; add its fixture to `for_each_racer!` in `tests/mutation_coverage.rs`; add its row to `RACERS` naming the exact set of concurrency rules it fails and the real adapter shape it comes from. | `crates/happenstance-testkit/tests/mutation_coverage/racers.rs:36-43` | The procedure is written down; this story follows it rather than inventing one. |
| The rendezvous, not a sleep: "an atomic counter and `std::thread::yield_now`, with the wait bounded by a number of yields so that a store which is never joined proceeds instead of hanging… A sleep would be both slower and less reliable." | `crates/happenstance-testkit/tests/mutation_coverage/racers.rs:20-34` | "A wrong store that is *sometimes* caught is worse than useless in a proof artefact: it turns the meta-test into a coin toss." |
| AC-T04: the mutation registry gains a row **before** AC-006 is claimed done, and the row names a real defect this schema can produce, not a placeholder. | `_decomposition.md`, *Testing brief*, AC-T04 | Order matters and is checkable in the diff. |

## Questions

Open questions to resolve before specifying.

1. **Does `SqliteEventStore` get registered in the testkit's mutation-coverage
   binary?** *Answered: no.* AC-006 is discharged from the adapter's own test
   target — every registered rule driven against a conformant `SqliteFixture` is
   exactly what `conformant_variants_pass_everything` asserts, and that is what
   `event_store_conformance!(SqliteFixture::new())` already does. The cost of the
   alternative is concrete and stated above: a published crate dev-depending on
   an adapter that dev-depends on it, `rusqlite` and a tokio runtime inside a
   harness whose no-hang guarantee is that it contains no I/O. If a reviewer
   overrides this, ADR-0022's option (a) stops being a recommendation.
2. **Which table does the `BEGIN DEFERRED` mutant go in?** *Answered: `RACERS`,
   in `crates/happenstance-testkit/tests/mutation_coverage/racers.rs`* — not
   `REGISTRY`, which the storymap says. A store wrong only in parallel fails no
   event-store rule, and an empty `fails` list is rejected by
   `mutant_registry_is_exhaustive`. The correction is made here, with the reason,
   in the same change.
3. **Which concurrency rules does the racer declare it fails?** Deferred to
   `spec`. `RACERS` requires the **exact** set, checked in both directions — every
   rule it declares, and no rule it does not. Getting that set right is a
   measurement against the family, not a guess.
4. **How is the racer's window closed?** Answered as mechanism: a rendezvous —
   atomic counter plus `std::thread::yield_now`, bounded by a count of yields, so
   a store that is never joined proceeds instead of hanging. Never
   `thread::sleep`. CF-33 constrains rules rather than instruments, but its reason
   applies here too (`racers.rs:20-34`).
5. **Does the model family need anything from the adapter beyond a green
   sequential suite?** Deferred to `spec`. The model family is proptest-driven and
   generates operation sequences; the open risk is runtime and cost against a real
   file rather than semantics. If it is slow enough to matter, that is a finding
   for the ledger, not a reason to reduce the case count silently.
6. **Does this story reshape or add a conformance rule?** Answered: **no.** It
   adds a *racer* and mounts an existing family. CF-17's rule shape — which could
   add or reshape one — is `reopen-negative-control-and-durability-verdicts`
   (HS-S0043) and is named there, together with the
   `mutation_coverage::every_rule_has_a_mutant` obligation that follows.
7. **The append-condition SQL strategy.** Consumed from ADR-0022 and deliberately
   not revisited; the racer encodes the *rejected* shape, which is not the same as
   reopening the choice.

## Decision

The suite can only be trusted to accept this adapter if it can also be shown to
reject the adapter's most plausible wrong twin, and for a racing rule that
demonstration cannot come from the adapter itself — a correct store proves nothing
about whether the rule bites. This slice does both halves: it takes
`event_store_model_conformance!` green against `SqliteFixture`, and it lands a
permanent, re-runnable negative control for the racing rules that AC-007 depends
on. The spec will cover: mounting the model family from
`crates/happenstance-sqlite/tests/`; discharging AC-006's "pass column" from the
adapter's own conformance run rather than by registering a `rusqlite`-backed store
inside the testkit's mutation-coverage binary, with the reason recorded because
the AC's wording invites the other reading; and adding a probe-then-insert racer
to `crates/happenstance-testkit/tests/mutation_coverage/racers.rs` with its
fixture in `for_each_racer!` and its row in `RACERS` — **not** in `REGISTRY`,
which is what `_storymap.md` says and which cannot pass
`mutant_registry_is_exhaustive`. That correction is made here with its reason
given in the same change, per this gate's last box. No conformance rule is added
or reshaped, so the literal-position bar is vacuous; the racer's assertions, like
the family's, are set properties over positions the store assigned. No `[FROZEN]`
clause is amended.

## The wrong implementation

**The mutant this story must *build*, and where it lives:** an `Arc`/`Mutex`
`SendEventStore` that probes its append condition outside the transaction that
assigns the position — SQLite's `BEGIN DEFERRED`, expressed in the racers' idiom
as a store that reads its snapshot, yields at a rendezvous, and then inserts
unconditionally. Its home is
`crates/happenstance-testkit/tests/mutation_coverage/racers.rs`, with its fixture
in `for_each_racer!` and its row in `RACERS`
(`crates/happenstance-testkit/tests/mutation_coverage.rs:2524`, `:2616`), and its
provenance is this adapter's own architecture brief §6. Its window is closed by an
atomic counter and `std::thread::yield_now`, bounded by a yield count, so it is
caught every run rather than most runs — `racers.rs:20-34` is explicit that a
sometimes-caught mutant "turns the meta-test into a coin toss and teaches the next
reader to re-run".

**The wrong implementation of *this story* is following `_storymap.md` literally
and putting that row in `REGISTRY`.** It does not compile past its own gate:
`mutant_registry_is_exhaustive` rejects a row whose `fails` list is empty
(`racers.rs:10-18`), and a probe-then-insert store fails **no** event-store rule —
that is the entire reason the racers table exists. The failure is loud and cheap,
so the cost is an hour rather than a defect; it is recorded here so the corrected
instruction reaches the spec instead of the compile error reaching the
implementer.

**The mutant that would pass every check and still be wrong: discharging AC-006 by
registering `SqliteEventStore` in the testkit's mutation-coverage binary.** It is
the literal reading of "the phase-3 mutant harness re-run with `SqliteEventStore`
in the pass column", it is green, and `conformant_variants_pass_everything` passes
with one more conformant variant. What it costs is not stylistic. The published
`happenstance-testkit` acquires a dev-dependency on `happenstance-sqlite`, which
dev-depends on `happenstance-testkit`. `rusqlite`'s bundled C library and a tokio
multi-threaded runtime enter a binary whose no-hang guarantee is *structural* —
"there is no channel, no I/O and no custom `Future` in the correct core, so a
mutant can only hang by introducing one deliberately"
(`crates/happenstance-testkit/tests/mutation_coverage/harness.rs:24-30`) — and
there is deliberately no watchdog, so the first time a SQLite lock is contended in
that binary the meta-tests hang naming no rule. The check that would catch this
does not exist, because "the testkit gained a dependency" is not a test failure.
The control is the recorded decision in this story's spec plus the review reading
of `crates/happenstance-testkit/Cargo.toml`'s dependency tables, and it is why
architecture brief §10 wrote the recommendation down before anyone reached the
AC's wording.

**The fourth mutant, quiet and specific to this story: a `RACERS` row whose
declared `fails` set is wrong in the permissive direction** — declaring the racer
fails one racing rule when it in fact fails three. Both meta-test directions are
checked ("every store fails every rule it declares, and passes every rule it does
not", `racers.rs:15-18`), so an under-declaration fails immediately and is
self-correcting. The genuinely silent version is a racer whose rendezvous is too
weak, so it passes the rules it declared it fails — caught, again, by the same
two-direction check. This is worth stating because it is the rare case where the
existing machinery is sufficient and this story owes nothing new: the obligation
is to use it correctly, not to extend it.

## Gate: Discover

Tick each box once the sections above satisfy it. `redkiln advance` reads these
against `.redkiln/templates/gates/discover.md` — where each box's rationale is
written — and will not leave `discover` until every one is ticked. Keep each box on
one line; the parser matches line by line.

- [x] The problem is framed in one paragraph.
- [x] Prior art and constraints are recorded in the signal ledger.
- [x] Open questions are either answered or explicitly deferred.
- [x] The next stage (spec) has a clear starting point.
- [x] The wrong implementation this work rejects is named.
- [x] No conformance rule added here asserts a literal position value.
- [x] Any `[FROZEN]` clause this touches is changed by a new ADR, written first.
- [x] If a rule here seems wrong, it is fixed and the reason given in the same change.
