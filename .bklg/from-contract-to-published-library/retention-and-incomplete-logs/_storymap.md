---
item: HS-P0018
stage: storymap
created: 2026-08-12T03:30:23.116Z
updated: 2026-08-12T03:30:23.116Z
template_sig: 1c63534a
rendered_sig: ab27f95c
---

# Story Map — What a store may forget, and how a reader finds out

Eleven stories in five slices. The spine is [`project.md`](project.md)'s AC-001 – AC-016;
the slicing is [`_decomposition.md`](_decomposition.md)'s architecture brief (*Composition
roots*, DA-1 – DA-9) and testing brief (*Test mix, mapped to every project AC*).

Four shaping facts, stated once so no story re-litigates them:

1. **The instrument comes before the rules, and the experiment before the decision.**
   The architecture brief's own sequencing note is binding: *"a rule written first will
   be written to the answer someone expected"*. The pass list is the evidence base for
   ADR-0028 (AC-002), so slice 1 runs the experiment and slices 2–4 consume its result.
   This is the inverse of `replication-identity-and-ingest`, where the decisions gate the
   code — here the instrument gates the decision, because CF-27
   (`spec/SPECIFICATION.md:8034-8060`) is an *experiment* and a pre-decided result is not
   evidence.
2. **AC-003's two failures are owed by the lying store, not by the honest one.** DA-3
   settles this: §3.7 and CF-27 both predict a pruned store passes every rule unchanged,
   so the named failures come from the two registered mutants AC-004 and AC-005 already
   require. No story is permitted to buy AC-003 by pre-seeding the fixture or by making
   it forget eagerly — both are named in DA-3 as the dishonest resolutions.
3. **Every mechanism already exists.** New rule bodies go in
   `crates/happenstance-testkit/src/suite.rs` and are registered exactly once in
   `for_each_event_store_rule!` (`crates/happenstance-testkit/src/registry.rs:94`); wrong
   implementations are `REGISTRY` rows in
   `crates/happenstance-testkit/tests/mutation_coverage.rs:324`; the instrument mounts
   through `event_store_conformance!` in the shape of
   `crates/happenstance-testkit/tests/fixture_instruments.rs:201-205`. No story invents a
   parallel harness.
4. **No surface published at `0.2.0` changes, and that is checked rather than intended.**
   ES-40 is a documentation clause (DA-5), ES-38's rule is a testkit gap and not a core
   gap (`_grounding.md` §6), and any fixture-side seam takes the **defaulted** shape
   `MID_BATCH_FAULT` set (`crates/happenstance-testkit/src/contract.rs:207-211`, `:297-307`).
   A conclusion that the honest answer needs a port surface is DA-7's escalation — a
   finding, never a change.

## Backbone

The activities an adapter author and a reader walk, left to right. Each column names the
slice that makes it real.

| # | Activity | Outcome a reader can observe | Slice |
| --- | --- | --- | --- |
| A1 | **Build a store that forgets** | A store holding an arbitrary retained set of its own log runs the whole conformance suite, mounted like every other instrument | `forgetting-instrument` |
| A2 | **Find out what the suite cannot see** | An enumerated list of rule names — per configuration — that a pruned store passes exactly as a young one does | `forgetting-instrument` |
| A3 | **Catch a store that lies about what it forgot** | Two rules in the registry, each red against a registered wrong implementation and green against the honest instrument | `owed-rules-and-mutants` |
| A4 | **Watch a reader meet a hole** | Three real readers run against the instrument, each with its *actual* output recorded — a wrong answer, a stall, or a `false` that means "never had it" | `readers-against-the-hole` |
| A5 | **Get a written answer** | ADR-0028 accepted under `.kb/decisions/`, one question, alternatives that lost named, the open question resolved rather than deleted | `the-retention-decision` |
| A6 | **Leave the record and the surface true** | `cargo xtask spec-trace` green with ES-39 and CF-27 no longer `[DEFERRED]`, and a surface diff that reports no breaking change | `clause-exit-and-surface-record` |

## Slices

Grouped by milestone. A `foundation` story lands real in-tree substrate that a capability
story in this same project consumes and demonstrates; none is a double, a flag or a
`todo!()`.

| Milestone | Story | Archetype | One-line slice | depends_on | traces_to |
| --- | --- | --- | --- | --- | --- |
| `forgetting-instrument` | `retained-set-instrument-and-conformance-mount` | foundation | Land the completeness instrument as a decorator over a live inner `MemoryEventStore` parameterised by a retained predicate (suffix **and** scattered, per DA-1), forgetting by hiding at the port rather than by rebuilding the inner store (DA-2 — the rebuild is ES-38's mutant), with a `Fixture` that presents an *empty* store at `connect()` like `MemoryFixture`, a filtering `read` that stays non-`async` and returns the stream at the top level (`CLAUDE.md` constraint 3, [ADR-0001](../../../.kb/decisions/0001-async-port-flavours.md)) with `ReadOptions` applied over the retained set, positions unique and strictly monotonic across the hole, and the whole thing mounted through `happenstance_testkit::event_store_conformance!` in a new `crates/happenstance-testkit/tests/` target in the shape of `crates/happenstance-testkit/tests/fixture_instruments.rs:201-205` — never `src/fixtures/`, never named by any of the three publishable crates' public API, and documented as an instrument that is never a target | — | AC-001, AC-013 |
| `forgetting-instrument` | `cf-27-experiment-and-recorded-pass-list` | capability | Run CF-27's experiment: the full suite against the instrument in **both** the suffix and the scattered configuration, with the enumerated list of rule names that pass committed per configuration as recorded data — stated as what it is, the set of rules that cannot tell a pruned store from a young one — plus the DA-3 finding on which of the three outcomes holds, and confirmation that no rule in the recorded failure set (if any) fails for a seeding reason | `retained-set-instrument-and-conformance-mount` | AC-002 |
| `owed-rules-and-mutants` | `positions-are-not-reused-after-removal` | capability | Write ES-38's owed rule (`spec/SPECIFICATION.md:4299-4323`) into `suite.rs`, register it in `for_each_event_store_rule!` (`crates/happenstance-testkit/src/registry.rs:94`), keep it green against `MemoryFixture` and the honest instrument, and land its registered wrong implementation — DA-2(a)'s store rebuilt through `MemoryEventStore::restore` from a truncated snapshot, which reassigns `position_at` indices (`crates/happenstance-core/src/memory.rs:277-282`, `:386-389`) and is ES-38's *"adapter that renumbers on compaction"* verbatim — as a `Defect` plus a `REGISTRY` row with a non-empty `fails` list, with `mutants_fail_exactly_their_declared_rules` and `mutant_registry_is_exhaustive` green in both directions; if the rule needs the store to lose something *during* the rule, the fixture seam is a **defaulted** `Capability` const plus a **defaulted** panicking method mirroring `MID_BATCH_FAULT` (`crates/happenstance-testkit/src/contract.rs:207-211`, `:297-307`) and nothing else, and if it does not need one, none is added | `cf-27-experiment-and-recorded-pass-list` | AC-003, AC-004, AC-011 |
| `owed-rules-and-mutants` | `condition-over-removed-history-does-not-reject` | capability | Write ES-40's rule (`spec/SPECIFICATION.md:4351-4379`) asserting the **vacuous pass** as *specified* behaviour — `Guard::is_violated_by` has two arms and no third (`crates/happenstance-core/src/append.rs:239-253`) — green against the instrument, registered in the same macro, with a `Defect` that rejects or errors instead (the adapter that "helpfully" fails closed on its own gap) carrying its own `REGISTRY` row; and discharge ES-40's *"the port's documentation MUST state…"* obligation as rustdoc beside `AppendCondition::is_violated_by` and/or `EventStore::append` (`crates/happenstance-core/src/store.rs:213`), so an adapter author reads the contract and an ingest author reads the hazard, with no signature change anywhere in `happenstance-core` | `cf-27-experiment-and-recorded-pass-list` | AC-003, AC-005, AC-011 |
| `readers-against-the-hole` | `decision-model-and-ingest-observed` | capability | Run composition roots 4 and 6 against the instrument and record their **actual** output: `happenstance_core::read_decision_model` (`crates/happenstance-core/src/store.rs:321-331`) returning the last *retained* match, whose `AppendCondition::after_opt` then admits an append the destroyed history would have rejected — silently, in existing code, with no new type — and `IngestStore::holds` (`crates/happenstance-sync/src/ingest.rs:164`) answering `false` for an event this store minted and acknowledged, so a peer re-sends forever (DA-8); each observation captured as the wrong value itself rather than as a note that something failed, and any reader that cannot be made to fail loudly without a surface the port lacks written up with the missing surface named | `retained-set-instrument-and-conformance-mount` | AC-006 |
| `readers-against-the-hole` | `projection-runner-across-the-hole` | capability | Run composition root 5 — the projection runner in `happenstance` over `crates/happenstance-core/src/projection.rs`'s checkpoint pump, arriving from HS-P0011 per [ADR-0007](../../../.kb/decisions/0007-projection-runner-decodes.md) — resuming from a checkpoint across the hole, with its actual state recorded against the hazard `crates/happenstance-testkit/src/suite.rs:1490-1531` already names (*"a projection resuming across a gap then stalls forever with no error anywhere"*); if no runner exists in the tree when this story starts, that is a dependency failure to halt on and report, never a licence to write a bespoke one — a hazard demonstrated against a reader nobody ships proves nothing | `retained-set-instrument-and-conformance-mount` | AC-006 |
| `the-retention-decision` | `dt-7-signal-shape-and-the-redaction-answer` | foundation | Resolve DT-7 in this project's `_design.md` — one undifferentiated incompleteness signal, or the transient / benign-permanent / meaningful-permanent distinction — with the option that lost and the reason, weighed against the production evidence in `_discovery/research/07-retention-deletion-and-incomplete-log-semantics.md` and against the reader complexity it costs; and answer the redaction question (E2E-49, `spec/E2E-CASES.md:1288-1311`) against the code DA-9 reads: `Tag(Cow<'static, str>)` with hand-written `PartialEq`/`Eq`/`Ord`/`Hash` all delegating to `as_str()` (`crates/happenstance-core/src/tag.rs:79`, `:170-193`), no digest field to swap in, and no store-side update path — landing either a decision or an explicit deferral naming a real `experiments/` path, because CF-38 makes an unnamed deferral a gate failure | `decision-model-and-ingest-observed` | AC-009, AC-010 |
| `the-retention-decision` | `adr-0028-and-the-open-question-wave` | foundation | Author ADR-0028 as **one question** — what is a store permitted to forget, and how does it say so — long form under `references/adr/` and the atom staged in `.kb/_intake/` and landed by `/redkiln:kb-ingest`, never hand-written into `.kb/decisions/` (`CLAUDE.md`; the revert at `0269720`): the pass list as its evidence, the alternatives that lost named including `earliest_position()` and why a floor is the wrong shape for a purge that is scattered rather than a prefix (`spec/SPECIFICATION.md:4336-4341`), DA-1's widening of CF-27's wording recorded with its reason, the CF-25/CF-26 residual exposure recorded open rather than implied closed (`:8003-8021`), SY-32 cited **by id** as answered by `replication-identity-and-ingest` with ADR-0026/ADR-0027 untouched, and — under the refusal branch — the refusal stated in terms, with the instrument as its illustration (DR-9); the same three-artifact intake wave carries the open-question resolution per DA-6: sub-question 3 of [`es-38-and-gap-read-rules-are-unowned`](../../../.kb/open-questions/es-38-and-gap-read-rules-are-unowned.md) answered on the record, a **narrowed successor** `open_question` atom carrying only the `read_from_a_gap_position` thread so that half is not closed in passing, the original moved to `superseded` with `related` naming both, `.kb/maps/open-questions-index.md`'s bullet updated, and `redkiln validate --kb` clean | `cf-27-experiment-and-recorded-pass-list`, `positions-are-not-reused-after-removal`, `condition-over-removed-history-does-not-reject`, `decision-model-and-ingest-observed`, `projection-runner-across-the-hole`, `dt-7-signal-shape-and-the-redaction-answer` | AC-008, AC-010, AC-015, AC-016 |
| `clause-exit-and-surface-record` | `cf-27-rule-or-recorded-refusal` | capability | Settle CF-27's own rule either way: if ADR-0028 decides, write `suffix_store_is_distinguishable_from_a_young_store` into `suite.rs` with its assertion fixed by the retention decision and registered in `for_each_event_store_rule!` (CF-27 already claims it, so `spec-trace` check 6 at `xtask/src/spec_trace.rs:727` is satisfied); if ADR-0028 refuses, the rule is **not** written, CF-27's `Rule:` line keeps its `(new)`/`†` marker (`xtask/src/spec_trace.rs:1626-1631`) and the clause text records in writing why it cannot exist and what a reader is therefore on its own against | `adr-0028-and-the-open-question-wave` | AC-007 |
| `clause-exit-and-surface-record` | `marker-moves-and-spec-trace-green` | capability | Move the markers and prove it: ES-39 and CF-27 leave `[DEFERRED]`, ES-40's `[PROVISIONAL]` falsifier is discharged or renewed against a named experiment (never simply left), the `(new)` markers come off the `Rule:` lines whose rules now exist, `cargo xtask spec-trace --write` (`xtask/src/main.rs:671-674`) regenerates §7.1/§7.2 and §1.3's prose census is **hand**-reconciled in the same commit — the one table the tool deliberately does not generate — with `cargo xtask spec-trace` and `cargo xtask ci --fast` (`.redkiln/config.yaml:55`) green on the result | `cf-27-rule-or-recorded-refusal`, `positions-are-not-reused-after-removal`, `condition-over-removed-history-does-not-reject` | AC-014 |
| `clause-exit-and-surface-record` | `surface-diff-and-the-ac-012-escalation` | capability | Run the public-surface comparison against the `0.2.0` registry baseline `publication-and-positioning` (HS-P0016) built, plus `cargo-semver-checks` at the PR grain (`CONTRIBUTING.md:291-296`), and record the result in two sentences rather than one: no breaking change to `happenstance-core`, `happenstance` or `happenstance-testkit`, every additive item enumerated (each new `pub` rule function, any defaulted `Fixture` const) and shipping as `0.2.x`, **and** the fact `crates/happenstance-testkit/Cargo.toml:4-13` states in the crate's own words — that adding a conformance rule is semver-MINOR and can still turn a passing adapter's CI red, which is the mechanism `CLAUDE.md`'s rule that matters exists to produce and not a violation; and if the honest answer needs a port surface, raise the escalation naming the surface, the version consequence (a `0.3.0` this initiative's exit criteria do not contemplate) and DA-7's four-row option table — floor, retained ranges, third condition outcome, tri-state `contains_event_id` — and make no such change, confirmed by no diff touching the public signatures in `crates/happenstance-core/src/store.rs` or `append.rs` | `marker-moves-and-spec-trace-green` | AC-011, AC-012 |

## Coverage

Every project AC-001 – AC-016 is claimed by at least one story, and where two stories
share an AC the second column says which part each owns.

| Project AC | Stories | Split of responsibility |
| --- | --- | --- |
| AC-001 | `retained-set-instrument-and-conformance-mount` | the instrument exists **and** is reachable by the suite — a type that merely compiles does not meet it |
| AC-002 | `cf-27-experiment-and-recorded-pass-list` | the run, in two configurations, and the enumerated pass list committed as data |
| AC-003 | `positions-are-not-reused-after-removal`, `condition-over-removed-history-does-not-reject` | one named failing rule each, both red against a registered mutant per DA-3(2) — not a property of the honest instrument |
| AC-004 | `positions-are-not-reused-after-removal` | ES-38's rule, its registry line, its mutant, and both meta-tests green |
| AC-005 | `condition-over-removed-history-does-not-reject` | ES-40's rule, its mutant, and the documentation sentence that discharges the clause |
| AC-006 | `decision-model-and-ingest-observed`, `projection-runner-across-the-hole` | roots 4 and 6 (in-tree today) versus root 5 (arrives from HS-P0011) — split so the runner's availability is an isolated blocker, not a risk to the other two observations |
| AC-007 | `cf-27-rule-or-recorded-refusal` | the rule written, or CF-27's text recording why it cannot be |
| AC-008 | `adr-0028-and-the-open-question-wave` | the atom, through the ingest path, answering one question with its losers named |
| AC-009 | `dt-7-signal-shape-and-the-redaction-answer` | DT-7 in `_design.md`, with the option that lost |
| AC-010 | `dt-7-signal-shape-and-the-redaction-answer`, `adr-0028-and-the-open-question-wave` | the first decides the redaction answer or names the experiment; the second records it in ADR-0028 where AC-010 asks for it |
| AC-011 | `positions-are-not-reused-after-removal`, `condition-over-removed-history-does-not-reject`, `surface-diff-and-the-ac-012-escalation` | the two rule stories keep every addition in the non-breaking shape (defaulted `Fixture` items; rustdoc, not signatures); the last story *proves* it by diff against the `0.2.0` baseline and enumerates the additive items |
| AC-012 | `surface-diff-and-the-ac-012-escalation` | the escalation artefact itself, fed by the two reader stories' "missing surface" findings and by DA-7's pre-computed option table |
| AC-013 | `retained-set-instrument-and-conformance-mount` | location in `tests/`, absent from every publishable public API, documented as never a target |
| AC-014 | `marker-moves-and-spec-trace-green` | the marker moves, the regenerated tables, the hand-reconciled census, the green gate |
| AC-015 | `adr-0028-and-the-open-question-wave` | the same intake wave: resolved, narrowed, superseded — never deleted |
| AC-016 | `adr-0028-and-the-open-question-wave` | SY-32 cited by id; ADR-0026/ADR-0027 not touched by this project's diff |

**No AC is orphaned, and no two stories own the same responsibility for one.** The three
ACs spread across stories (AC-003, AC-006, AC-010, AC-011) are split by *mechanism*, not
duplicated: a rule and its mutant per failure, a reader per composition root, the decision
versus its record, and the shape of an addition versus the diff that proves it.

Three notes on what this map deliberately does **not** slice:

- **No story depends on substrate owned outside this initiative.** The projection runner is
  HS-P0011's and the `0.2.0` baseline is HS-P0016's — both siblings in this initiative,
  consumed as built. If either is absent when its story starts, that is a blocker to raise,
  never a stand-in to write (the architecture brief's closing note, and the testing brief's
  *"do not construct a synthetic stand-in registry snapshot"*).
- **No "build it / wire it in" split.** The instrument and its `event_store_conformance!`
  mount are one story, because under `CLAUDE.md`'s rule that matters an unmounted
  instrument does not exist; each rule ships with its registry line and its mutant row in
  the same story, because a rule absent from `for_each_event_store_rule!` is invisible to
  all four emitters and a rule with no wrong implementation is decorative.
- **No E2E story.** `.redkiln/config.yaml:60` reserves `cargo xtask ci` for the terminal
  project; `cargo xtask ci --fast` (`:55`) is this project's ceiling, and the whole gate on
  the assembled tree is `closeout-and-durable-audience`'s.

## Merge order

Slice by slice; within a slice, top to bottom. The foundation stories precede every
capability that consumes them, and the experiment precedes the decision it is evidence for.

1. **`forgetting-instrument`** — `retained-set-instrument-and-conformance-mount` →
   `cf-27-experiment-and-recorded-pass-list`. Nothing else in this project can merge first:
   both rules, all three readers and ADR-0028 are written against what this run reports.
2. **`owed-rules-and-mutants`** — `positions-are-not-reused-after-removal` →
   `condition-over-removed-history-does-not-reject`. Order within the slice is by the
   mutant each needs: the renumbering store is a by-product of DA-2's rejected
   construction and is already in hand when the instrument lands.
3. **`readers-against-the-hole`** — `decision-model-and-ingest-observed` →
   `projection-runner-across-the-hole`. No edge to slice 2; it may be interleaved with it,
   and is sequenced second only so the runner's HS-P0011 dependency is discovered while
   there is still slack.
4. **`the-retention-decision`** — `dt-7-signal-shape-and-the-redaction-answer` →
   `adr-0028-and-the-open-question-wave`. This slice is where the project's answer is
   written, and it cannot open until slices 1–3 have produced the evidence; the refusal
   branch is not cheaper, because DR-9 keeps the instrument and the illustration either
   way.
5. **`clause-exit-and-surface-record`** — `cf-27-rule-or-recorded-refusal` →
   `marker-moves-and-spec-trace-green` → `surface-diff-and-the-ac-012-escalation`. Last on
   purpose: a marker can only move once the rule it names exists (or is recorded as
   never-to-exist), and the surface diff is an exit computation over the assembled tree.
