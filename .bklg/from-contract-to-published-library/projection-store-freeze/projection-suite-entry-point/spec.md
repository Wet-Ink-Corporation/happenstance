---
item: HS-S0007
stage: spec
created: 2026-08-12T13:46:01.460Z
updated: 2026-08-12T13:46:01.460Z
template_sig: 87bbf1d0
rendered_sig: 24598a15
---

# Spec — projection_store_conformance! — one enumeration, one test per rule

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` — **DoD 7** is the scenario this story serves; AC-04 and AC-07 are the acceptance criteria it moves |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` — this project is rank 0, the substrate root |
| Project | `.bklg/from-contract-to-published-library/projection-store-freeze/project.md` — **AC-001** and **AC-016** are this story's; DR-02, DR-03 and DR-09 are the derived requirements behind them |
| This spec | `.bklg/from-contract-to-published-library/projection-store-freeze/projection-suite-entry-point/spec.md` |
| Key briefs | `.bklg/from-contract-to-published-library/projection-store-freeze/_decomposition.md` — Architecture **AC-A01** (one enumeration), **AC-A06** (reuse `Capability`/`RuleOutcome`, no borrowing GAT), Notes **1** (seam map), **4** (**the two seams: write vs rule-running** — this story's central problem), **5** (the fixture contract), **6** (what one rule does); UX **AC-U08 – AC-U11** (the text surface); Testing brief rows **AC-001**, **AC-005**, **AC-016** |
| Grounding | `.bklg/from-contract-to-published-library/projection-store-freeze/_grounding.md` §3 — the event-store suite, the pattern to mirror |
| Story map | `.bklg/from-contract-to-published-library/projection-store-freeze/_storymap.md` — slice `projection-conformance-suite`, merge position 3, first green run of a projection rule against a real store |
| Signed-off design | `.bklg/from-contract-to-published-library/projection-store-freeze/_design.md` — `surfaces: []`, no user-facing surface, signed off as a determination (`_design.md:92-101`); see the Integration contract for what that does and does not license |
| Roadmap / plan of record | `RUNBOOK.md:3848-3965` (phase 6 in full); `RUNBOOK.md:3882-3892` (the probe, and why a suite without it is a checkpoint test a broken store passes) |
| Normative source for the suite | `spec/SPECIFICATION.md:5652-5704` — §4.11's seventeen rules and their clauses; CF-22 (`:7775`), CF-23 (`:7920`), CF-24 (`:7950`), CF-29 (`:8141`) |

## One-line PR slice

An adapter author writes one line and gets one test per rule: `ProjectionFixture` in the testkit's `contract.rs`, the single enumeration `for_each_projection_store_rule!`, the emitter seam Note 4 leaves open, `projection_store_conformance!` with its `__private` export, an orphan meta-test over the projection rules module, the crate-doc rule-family table, and three harness files — tokio, blocking and `#![cfg(target_arch = "wasm32")]` — running the baseline pair `commit_advances_the_checkpoint` and `commit_is_atomic_with_the_read_model` against `MemoryProjectionStore`.

## Executive summary

This PR lands the **fourth rule family** in `happenstance-testkit` and the machinery that makes it a family rather than two loose tests. It is the first story in slice `projection-conformance-suite` and the first moment any projection rule executes against a real store.

Pointer plus delta against the tree as it stands:

- `crates/happenstance-testkit/src/` gains one module (the projection rules, their enumeration, their emitters and their entry macro) and one trait in `contract.rs` (`ProjectionFixture`). `Capability`, `RuleOutcome`, `block_on` and the `Declared` mutant-registry shape are **reused unchanged** — nothing in the event-store family is forked (Architecture brief AC-A06, `_decomposition.md:517-544`).
- `crates/happenstance-testkit/src/lib.rs` (364 lines today) mounts it: the module list at `:165-189`, the public re-export at `:187`, `__private` at `:359-364`, and the two crate-doc sections that today describe **three** families and must describe four — "Where the rule set lives" (`:84-92`, which says "exactly one place is per rule **family** (CF-22), and there are three") and the "What is checked" table (`:134-151`).
- `crates/happenstance-testkit/tests/` gains three harness files. The wasm one is type-checked by a gate step that already exists and already runs (`xtask/src/main.rs:231-243`), which is the whole of AC-016's mechanism — a new harness file, not a new gate step.
- `xtask/` gains one array element. `RULE_FILES` (`xtask/src/spec_trace.rs:85-89`) is the set CF-29's changelog lint and spec-trace's check 6 sweep; a fourth rule file that is not in it is a family nothing holds to the changelog obligation, which is exactly the defect stage 6's review found and recorded (`spec/SPECIFICATION.md:8950-8956`).

Two rules land, not seventeen. `commit_advances_the_checkpoint` and `commit_is_atomic_with_the_read_model` are the baseline PS-1 assigns and the rest of §4.11 is differential against (`spec/SPECIFICATION.md:4735-4741`). Everything the suite is *for* — a store that fails by name, a mutant per rule, reset, rebuild, the second batch shape — is downstream of this story and named in the PR boundary.

What this PR does **not** claim: that the suite discriminates. `CheckpointOnlyStore` is `projection-mutant-registry`'s, one slice later. A green run here proves the machinery runs, not that it can fail — and the difference is the whole of ADR-0010 (`.kb/decisions/0010-the-suite-must-prove-itself.md`).

## Context pack

Read this section and you can start. Everything deeper is behind a signposted anchor row (second pass).

### The three dependencies are preconditions, and they are checkable in one minute

This story compiles against work three earlier stories land. Before the first edit, confirm all three, and stop if any is absent — a fixture written against a port that has not changed shape is a rewrite, not a start:

- `crates/happenstance-core/src/projection.rs` carries `type Batch;` with **no** lifetime, `fn begin(&self) -> Self::Batch` (neither `async` nor fallible), `checkpoint → Checkpoint`, `commit(batch, id, position, Authority) -> Result<(), CommitError<E>>` and `reset` — `owned-batch-port-shape`.
- `ProjectionProbe` exists in `happenstance-core` behind `feature = "conformance"`, with `READS_THROUGH_BATCH`, `probe_write`, `probe_delete_all`, `probe_read`, `probe_read_through` — `projection-probe-conformance-feature`.
- `MemoryProjectionStore` exists behind `feature = "memory"` and implements `ProjectionProbe` under `conformance` — `memory-projection-store`.

Today's `projection.rs` is 139 lines and still carries the GAT (`crates/happenstance-core/src/projection.rs:97-99`) and the provisional header (`:1-11`). That header stays. This story does not touch it; AC-014 disposes of it in `unstable-projection-gate-and-clause-disposition`.

### The rule-running seam: the tree has already answered Note 4, twice

Architecture brief Note 4 (`_decomposition.md:488-515`) names the trap correctly — the three event-store emitters hard-code `$crate::rules::$name` (`crates/happenstance-testkit/src/registry.rs:229-238, 248-257, 277-287`), so a projection rule living anywhere else cannot reuse them as written — and leaves three ways out with none pre-selected. **It does not cite the two families already in the tree that faced the same choice, and both took option 2.**

- `crates/happenstance-testkit/src/model.rs:712-800` — `for_each_model_rule!`, `__emit_model_tokio`, `__emit_model_blocking`, `event_store_model_conformance!`, all beside the rules they name.
- `crates/happenstance-testkit/src/concurrency.rs:1024-1160` — `for_each_concurrency_rule!`, `__emit_concurrency_tokio`, `__emit_concurrency_blocking`, `event_store_concurrency_conformance!`, likewise. Its emitters spell `$crate::concurrency::rules::$name` and its entry macro names the fixture function `__conformance_fixture` **identically to `event_store_conformance!`'s**, "so a caller-supplied emitter — CF-23's extension point — drives any of the three families without knowing which one it was handed" (`concurrency.rs:1140-1145`).

**Decision recorded here, on that evidence: the projection family owns its own enumeration, its own three emitters and its own entry macro, beside its own rules.** This is Note 4's option 2, and it is not a fallback — it is the house pattern for a second family, taken twice, and it makes Note 10 item 3 (*"if option 1 cannot keep `local_conformance.rs`, `memory_conformance*.rs` and `fixture_instruments.rs` compiling, take option 2 and say why"*) moot rather than risky: option 2 does not touch the event-store emitters at all, so those five harnesses cannot break.

Two consequences to carry, not discover. `for_each_projection_store_rule!` living beside its rules **is** what the story map means by "beside `for_each_event_store_rule!`": the concurrency family's own doc gives the reason a rule may not be written into `suite.rs` (`concurrency.rs:1027-1031`), and there is a sharper one here — `no_orphan_rules` scans `suite.rs` textually for `    pub async fn ` (`registry.rs:346-366`), so a projection rule in that file would be reported as an orphan of the *event-store* enumeration. And every emitter is `#[macro_export]`ed, so three new emitters are three new public macros on a crate that carries its own version precisely because a suite change can turn a passing adapter's CI red (`crates/happenstance-testkit/Cargo.toml`, the `version` comment).

### The fixture is a second trait, and the shape it must not have

`Fixture::Store: EventStore` (`crates/happenstance-testkit/src/contract.rs:120-125`) binds the wrong port in the associated type, so the projection suite needs `ProjectionFixture`. Mirror `contract.rs` closely and take these as settled:

- **`type Store: ProjectionProbe`**, not `ProjectionStore`. `ProjectionProbe: ProjectionStore` (`spec/SPECIFICATION.md:5003`), so one bound buys both, and PS-11 makes the probe an obligation on any adapter that invokes the suite: *"an adapter that does not implement it cannot invoke the suite, and by CLAUDE.md's rule it does not exist"* (`:4977-4986`). Binding the probe at the fixture is what makes it a compile error rather than a convention.
- **The bare flavour, never `SendProjectionStore`.** It is the weaker requirement and accepts both kinds of adapter, and only one of the two names may be in scope per module (CLAUDE.md binding constraint 4; the same reasoning `Fixture::Store: EventStore` records at `contract.rs:123-125`).
- **No borrowing GAT anywhere.** `type Store<'a> where Self: 'a` on a foreign trait is one of five independently necessary ingredients of a rustc ICE this repository already minimised and which still reproduces on 1.97.1 (`contract.rs:97-111`; `experiments/rustc-ice-gat-foreign-trait/`). Hand back an **owned** handle holding a refcount, which is what `MemoryFixture` does with an `Arc` clone (`crates/happenstance-testkit/src/fixtures.rs:243-292`).
- **`connect()` is `async` and panics rather than returning `Result`.** A fixture that cannot connect is a broken test environment, not a non-conformant adapter, and a `Result` would put "the database is down" into the same channel as "the adapter is wrong" (`contract.rs:309-321`).
- **Rules take `impl AsyncFn() -> F`, not a made fixture** (`registry.rs:45-49`; the signature at `crates/happenstance-testkit/src/suite.rs:265-267`). A rule that can call `open()` twice can make two *isolated* stores, which `commit_rejects_a_foreign_batch` needs later and which no meta-test over the testkit's own fixture could substitute for.

**Which capability constants the projection fixture declares is not this story's call.** Architecture brief Note 5 assigns it to `_design.md` via `projection-api-design-record`, and `projection-capability-skips` is the story that lands the set and its skip assertions. This story declares the trait and whatever constants the two baseline rules actually gate on — which is **none**, so the honest shape here is a fixture trait with a capability *surface* that is exercised by its slice-mate, not by this PR. Say so rather than inventing a constant to look symmetric: a capability no rule reads is the decorative shape CLAUDE.md's corollary names.

### What one rule does, and why the probe is the load-bearing part

The shape both baseline rules share (Architecture brief Note 6, `_decomposition.md:546-567`):

```
rule(open: impl AsyncFn() -> F)
  → open().await                          one isolated projection store
  → fixture.connect().await               one handle (F::Store: ProjectionProbe)
  → store.begin()                         PS-6: not async, not fallible → Self::Batch (owned, PS-5)
  → store.probe_write(&mut batch, k, v)   ProjectionProbe, contract crate
  → store.commit(batch, &id, position, Authority::Live).await
  → open a *fresh* handle
  → store.probe_read(k).await   AND   store.checkpoint(&id).await
  → assert both present, or both absent — never one
```

`commit_advances_the_checkpoint` is the baseline the other fifteen rules are differential against; `commit_is_atomic_with_the_read_model` is the one `CheckpointOnlyStore` fails at the last line and nowhere else. Without `probe_write`/`probe_read` the second rule cannot observe the read model at all and the suite degenerates into a checkpoint test a broken store passes (`RUNBOOK.md:3882-3892`).

Two house rules bind both rules' bodies. **Never assert a literal position value** — the specification permits gaps, and `GappedPositionStore` exists to convict a rule that forgets (CLAUDE.md, "The rule that matters"); compare against positions the store actually assigned. And **no clock**: a conformance rule does not read time (CF-33).

### The rule that no adapter can fail is decorative — and the mutant is one story away

CLAUDE.md's first corollary says: before adding a rule, name a plausible wrong implementation it rejects, and write that implementation into the testkit's own `tests/` if one does not already exist there. `CheckpointOnlyStore` is named by the specification (`spec/SPECIFICATION.md:5680-5685`) and by PS-2 (`:4760-4775`), and it is **`projection-mutant-registry`'s deliverable**, the next story in merge order and this story's direct dependant (`blocks: HS-S0008`).

The discipline this story carries instead of the store: **each rule's rustdoc names the wrong implementation it rejects, in the `suite.rs` idiom** (see `suite.rs:253-264`, where the rule's doc names the registered mutant and the two others that fail it for unrelated reasons). Landing a rule whose doc names no defect is how a decorative rule survives review. This is a knowingly-carried, one-story debt with a named discharger — not an exemption, and not a licence to add a third rule here because it was easy.

### CF-29 and spec-trace: the fourth rule file has obligations the first three learned the hard way

`RULE_FILES` (`xtask/src/spec_trace.rs:71-89`) is a three-element array whose doc says *"Three, not one"* and explains why: until stage 6's review, CF-29's changelog lint read `suite.rs` alone and printed `all 55 suite rules have a changelog entry` — *"a sentence that is true and answers a different question than the one the step's name asks"* (`xtask/src/lints.rs:511-519`). A projection rules module absent from that array reproduces the defect exactly, one family later.

So this story adds the file to `RULE_FILES` and writes **one CHANGELOG entry per rule naming the defect it detects**. The lint requires at least 120 characters of prose per rule an entry names, and divides an entry's prose by the number of rules it names (`xtask/src/lints.rs:524-580`), so one bullet naming both rules must be twice as substantial or they take an entry each.

Three second-order effects, in the order they will be met:

1. **Check 6 then sweeps the projection rules** — every rule in `RULE_FILES` must be claimed by a clause, disposed of by one, or listed in `UNCLAIMED_PENDING_ADR` (`spec_trace.rs:726-800`). PS-1 claims both baseline names in its own `Rule:` field (`spec/SPECIFICATION.md:4736-4741`), so this should pass — **verify it, do not assume it**, because what check 6 reads is the parsed field, not §4.11's table.
2. **Check 4 is not affected.** Clauses whose ids start `PS-` are skipped by `has_suite` (`spec_trace.rs:1735-1737`), which exists so that PS and SY names are not reported missing against a suite that does not exist. Widening `has_suite` to `PS-` is **not** this story's: PS clauses name fifteen rules that will still not exist when this PR merges, and every one would be reported. That widening belongs with the clause disposition, in `unstable-projection-gate-and-clause-disposition`.
3. **§7.1–§7.2 is generated and will move.** The rendered table marks a rule that does not exist with `†` (`spec_trace.rs:1114`); two of them start existing here. `cargo xtask spec-trace` compares the committed region against the computed one and fails on a mismatch, and the remedy is always `--write`, never editing the row (`spec_trace.rs:1240-1290`, `:1317-1320`). **The regenerated region is the only edit to `spec/SPECIFICATION.md` this story is permitted** — no clause body, no maturity marker, nothing `[FROZEN]` line-edited. And note the timing trap: `spec-trace` is one of the two steps `cargo xtask ci --fast` omits (CLAUDE.md, Commands), so a story-grain green does not see this. Run `cargo xtask spec-trace` explicitly before calling the story done.

### The wasm harness is where the reason disappears if nobody looks

`RuleOutcome::report` is a **no-op** on `wasm32-unknown-unknown` — measured under `wasm-bindgen-test-runner`, not assumed (`contract.rs:525-531`). The wasm emitter therefore calls `skip_line` and hands the result to `console_log!` (`registry.rs:274-288`). The projection wasm emitter does the same, unchanged in shape, so that AC-016's run is not the one where a stated reason silently disappears (UX brief AC-U11, `_decomposition.md:189-195`).

Nothing new is needed in the gate: `cargo check --locked -p happenstance-testkit --tests --target wasm32-unknown-unknown` (`xtask/src/main.rs:231-243`) already type-checks every file under `tests/`, and the wasm harness is `#![cfg(target_arch = "wasm32")]` so it compiles only there. The blocking harness takes the mirror-image gate — `#![cfg(not(target_arch = "wasm32"))]` — because `__emit_blocking` emits a plain `#[test]` and libtest does not exist on that target (`tests/memory_conformance_blocking.rs:14-21`).

### Persona slice

The reader served is **P2, the adapter author** (`_decomposition.md:55-58`, `.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md:114-153`), who wants *"an executable definition of 'correct' they can run against their own storage system, rather than a prose specification they have to interpret"*. Today they have that for `EventStore` and nothing at all for `ProjectionStore`: the port's own header says a port without a conformance suite is a guess (`crates/happenstance-core/src/projection.rs:3-11`). This story gives them the one line to write and the harness choice to make. It does not yet tell them they are finished — that needs the rules — which is why this story's own proof is a green run of two rules against the reference store, and initiative AC-04 is *advanced*, not met.

**P3, the local-first / edge developer** is served by the third harness file and by nothing else here: AC-016 is their rule, and the text surface is weakest exactly there (`_decomposition.md:58`).

## Integration contract

- **Archetype**: `capability`. A user-observable slice through every layer the library has — an adapter author's one-line invocation reaches the port, the probe, the reference store and back out as one test per rule, on three runtimes.
- **Slice / milestone**: `projection-conformance-suite`. Slice-mate implemented in the same context and mounted as one integrated surface: **`projection-capability-skips`** (HS-S0009). Order within the slice is this story first — the mate asserts on `RuleOutcome` values produced by the machinery landed here.
- **Mount point**: **`crates/happenstance-testkit/src/lib.rs`**. For a library that file is the composition root, and mounting means both halves of Architecture brief Note 1 (`_decomposition.md:339-360`): the export block and the feature table. Concretely — the entry macro `projection_store_conformance!` beside `event_store_conformance!` (`:265-357`); `__private` gaining `ProjectionFixture` (`:359-364`), without which the macro expansion cannot name the trait in the adapter's crate; the module list (`:165-189`) and the public re-export line (`:187`); and the crate doc's two family-scoped sections (`:84-92`, `:134-151`), which today both describe a world with three families. The feature-table half is `crates/happenstance-testkit/Cargo.toml`, whose `happenstance-core` dependency must gain `conformance` beside `std` and `memory`. An item that compiles and is mounted at neither is an item no adapter can name.
- **Wires into**:
  - `crates/happenstance-testkit/src/contract.rs` — `Capability` (`:355-433`), `RuleOutcome` (`:458-537`) and `skip_line` (`:500-507`), **reused unchanged** (AC-A06); the new `ProjectionFixture` trait lands in this file, mirroring `Fixture` (`:120-353`).
  - `crates/happenstance-testkit/src/registry.rs` — `block_on` (`:330-344`), consumed by the blocking emitter; the three event-store emitters (`:222-295`) are the shape to copy and are **not modified**.
  - `crates/happenstance-testkit/src/fixtures.rs` — `MemoryFixture`'s owned-handle pattern (`:243-292`) is what the new `MemoryProjectionFixture` copies; the public `fixtures` module is where an adapter author looks for the reference implementation (`lib.rs:52-53`).
  - `crates/happenstance-core/src/projection.rs` — `ProjectionStore`, `ProjectionId`, `Checkpoint`, `Authority`, `CommitError`, and `ProjectionProbe` behind `conformance`; `MemoryProjectionStore` behind `memory`.
  - `xtask/src/spec_trace.rs:71-89` (`RULE_FILES`) and `xtask/src/lints.rs:504-580` (CF-29's lint) — the two consumers that must learn about a fourth rule file.
  - The five existing harnesses that consume the event-store emitters and must keep compiling untouched: `tests/local_conformance.rs:464-528`, `tests/memory_conformance.rs:27`, `tests/memory_conformance_blocking.rs:24-28`, `tests/memory_conformance_wasm.rs:23-27`, `tests/fixture_instruments.rs:201`.
- **Renders surfaces**: **none.** `_design.md` records `surfaces: []` and the no-surface determination is what was signed off (`_design.md:48-50, 92-101`). That is binding as far as it goes and is not a licence to skip the API review: what this story renders instead are the project's two non-visual surfaces — the **type surface** (`projection_store_conformance!`, `ProjectionFixture`, the three emitters, `MemoryProjectionFixture`) and, downstream of it, the **text surface** (`SKIP {rule}: fixture declines …`), which stays reused verbatim and whose assertions belong to the slice-mate `projection-capability-skips` (UX brief AC-U08, `_decomposition.md:161-169`).
- **Conformance rule(s)**: `commit_advances_the_checkpoint` and `commit_is_atomic_with_the_read_model`, both new, both in the new projection rules module, both registered in `for_each_projection_store_rule!`. The machinery around them is observed by the orphan meta-test (CF-24's projection sibling) rather than by a rule.
- **Clause(s)**: discharges CF-22 (`spec/SPECIFICATION.md:7775`) and CF-24 (`:7950`) for the projection family, and CF-23 (`:7920`) via three emitters; implements the suite half of PS-1 (`:4733-4759`) and PS-11 (`:4977-4986`); implements §4.11's first two rows (`:5661-5662`). **Amends none**, moves no maturity marker, and line-edits nothing `[FROZEN]`. The only `spec/SPECIFICATION.md` edit permitted is the generated §7.1–§7.2 region.
- **Advances DoD scenario**: initiative **DoD 7** — *"The projection suite discriminates."* This story lands the suite that a wrong store will later fail; it does not move DoD 7 to green on its own, because discrimination needs `CheckpointOnlyStore` (next story) and two batch shapes (slice 6). It also advances initiative **AC-04** (one entry point, a named failure per rule) and **AC-07** (every rule on the constrained target in the same run), and discharges project **AC-001** in full and **AC-016**'s harness half (`_storymap.md` Coverage, rows AC-001 and AC-016).

## PR boundary

```
crates/happenstance-testkit/src/**
crates/happenstance-testkit/tests/projection_conformance.rs
crates/happenstance-testkit/tests/projection_conformance_blocking.rs
crates/happenstance-testkit/tests/projection_conformance_wasm.rs
crates/happenstance-testkit/Cargo.toml
crates/happenstance-testkit/README.md
xtask/src/spec_trace.rs
CHANGELOG.md
spec/SPECIFICATION.md
standards/rust/**
.bklg/from-contract-to-published-library/projection-store-freeze/projection-suite-entry-point/**
```

`spec/SPECIFICATION.md` is in the boundary for **one** reason and it is mechanical: `cargo xtask spec-trace --write` regenerates §7.1–§7.2, and two rules starting to exist changes what that region renders. Any other hunk in that file is out of boundary — clause text, maturity markers and rule citations are `unstable-projection-gate-and-clause-disposition`'s.

`standards/rust/**` is in the boundary for **one** reason and it is the same mechanical one, widened deliberately on **2026-08-14** after `redkiln verify --grain story` rejected `79df6b7` for writing eleven atoms this list did not admit. The constitution cites `crates/happenstance-testkit/src/**` by `file:line`; this story adds to those files; the cited lines therefore move; and `cargo xtask lint-constitution` — a gate step — fails on a stale citation. The repair is **compelled by an entry the boundary already admits**, exactly as `owned-batch-port-shape`'s `stand_in.rs` edit was compelled by `broken_intra_doc_links` at `deny` (settled in `b7c1600`, ratified in `_slices.md`). Only **line-number re-pointing** is admitted, and the diff must show equal insertions and deletions per atom: `79df6b7` is 23 and 23. **Rule text, evidence selection, a `## Retired` section or a new atom are out of boundary** and belong to the story that changes the rule, not to whichever story happened to move a line. Widening this list to match what an implementer wrote, without that argument, would make the check a rubber stamp for anything it touched. `crates/happenstance-testkit/tests/**` is deliberately **not** globbed: the three new files are named individually, because `mutation_coverage*` under the same directory is the next story's and a wide glob would hide a boundary violation rather than prevent one.

**In this PR**

- `ProjectionFixture` in `crates/happenstance-testkit/src/contract.rs` — owned `type Store: ProjectionProbe`, `async fn connect`, no borrowing GAT, `Capability`/`RuleOutcome` reused.
- A new projection module under `crates/happenstance-testkit/src/` holding `pub mod rules` with the two baseline rules, `for_each_projection_store_rule!`, three `#[macro_export]` emitters (`__emit_projection_tokio`, `__emit_projection_blocking`, `__emit_projection_wasm`) and the orphan meta-test.
- `projection_store_conformance!` in `lib.rs`, three arms, `__conformance_fixture` named exactly as the other two entry macros name it; `__private` gains `ProjectionFixture`; `ProjectionFixture` also re-exported publicly beside `Fixture`.
- `MemoryProjectionFixture` in `fixtures.rs`, over `MemoryProjectionStore`, in `MemoryFixture`'s owned-handle idiom.
- Three harness files, and the `happenstance-core` dependency gaining `conformance` in `crates/happenstance-testkit/Cargo.toml`.
- Crate-doc updates: the family count and the "What is checked" table; module names of gated modules spelled plainly, never as intra-doc links (`lib.rs:112-132`).
- `RULE_FILES` gaining the projection rules file, its doc comment restated, and a CHANGELOG entry per rule naming the defect it detects.

**Explicitly not in this PR**

- `CheckpointOnlyStore`, `TruncatingResetStore`, `ValidatingCommitStore`, the projection `REGISTRY: &[Declared]` and its three exactness meta-tests — `projection-mutant-registry` (HS-S0008).
- Any capability constant on `ProjectionFixture` that no rule here reads, and the `RuleOutcome::Skipped` assertions — `projection-capability-skips` (HS-S0009), which also owns DT-3's recorded capability set.
- The other fifteen §4.11 rules — the `commit-atomicity-and-mutants` and `reset-and-rebuild-rules` slices.
- The CF-5 buffering conformant variant and any second batch shape — `buffering-conformant-variant`.
- Any port-shape change, any `ProjectionProbe` change, `MemoryProjectionStore` itself — the three dependency stories. If one of them needs a change to satisfy a rule here, that is a **finding to report**, not an edit to absorb.
- Every maturity marker, the `unstable-projection` gate, deleting `projection.rs:1-11`, and widening `has_suite` to `PS-` — `unstable-projection-gate-and-clause-disposition`.
- Any change to `for_each_event_store_rule!`, the three event-store emitters, `event_store_conformance!`'s expansion, or any existing harness. If the design ends up needing one, stop: Note 10 item 3 makes that a reportable reshape, and the option chosen here exists precisely so it cannot happen quietly.

The implementer **may** touch the mount/wiring files named in the Integration contract — `lib.rs`, `Cargo.toml`, `README.md`, `RULE_FILES` — to mount this slice. That is mounting, not scope drift.

**Merge DoD one-liner**: `projection_store_conformance!(MemoryProjectionFixture::new())` expands to two named tests on all three runtimes, the orphan meta-test can fail, `cargo xtask affected --base main` and `cargo xtask ci --fast` are green, and `cargo xtask spec-trace` plus the mandatory wasm32 conformance-harness check are green when run explicitly.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| One line, one test per rule | `happenstance_testkit::projection_store_conformance!(MyProjectionFixture::new())` expands to one test per projection rule. Three arms, in the order `event_store_conformance!` uses so arm matching never backs out of `fixture = $fixture:expr`: `(mod_name, emit, fixture)`, `(mod_name, fixture)`, `($fixture:expr)`. Default `mod_name` is distinct from `dcb_conformance` so both suites can be invoked from one file; default emitter is the tokio one | `crates/happenstance-testkit/src/lib.rs:265-357`; the second-family precedent at `crates/happenstance-testkit/src/concurrency.rs:1129-1160` |
| The fixture expression is hoisted, and named identically | The expansion defines `async fn __conformance_fixture() -> impl $crate::__private::ProjectionFixture { $fixture }`. **The name is not free**: both existing entry macros use `__conformance_fixture`, so a caller-supplied emitter — CF-23's extension point — drives any family without knowing which it was handed. `macro_rules!` hygiene applies to local variables, not items, which is what lets one macro define it and another refer to it | `lib.rs:321-341`; `concurrency.rs:1140-1147`; `registry.rs:28-49` |
| `__private` gains the trait | `pub mod __private { pub use crate::contract::{Fixture, ProjectionFixture}; }`. Without it the expansion cannot name the trait in the adapter's crate, which is what the seam map calls out as a **must** | `lib.rs:359-364`; `_decomposition.md:352` |
| `ProjectionFixture` mirrors `Fixture` and reuses its reporting types | `type Store: ProjectionProbe` (owned, no GAT, bare flavour); `fn connect(&self) -> impl Future<Output = Self::Store>`; capabilities as `Capability`; outcomes as `RuleOutcome`. `async fn` is **not** spelled in the declaration — `async_fn_in_trait` fires on a public trait and the gate runs `-D warnings`; the desugared form also makes the absence of `+ Send` visible at the declaration, which is ADR-0001's whole point | `contract.rs:88-125`, `:309-321`, `:355-537`; `.kb/decisions/0001-async-port-flavours.md` |
| No borrowing GAT, anywhere in the fixture | `type Store<'a> where Self: 'a` on a foreign trait is one of five independently necessary ingredients of a rustc ICE minimised in this repository, still reproducing on 1.97.1. Hand back an owned handle holding a refcount | `contract.rs:97-111`; `experiments/rustc-ice-gat-foreign-trait/`; `_decomposition.md:535-540` |
| Rules are handed how to make a fixture, not a fixture | `pub async fn <rule><F: ProjectionFixture>(open: impl AsyncFn() -> F) -> RuleOutcome`, at one level of indentation inside `pub mod rules`. The indentation and the `pub async fn` prefix are load-bearing twice over: the orphan scan matches them, and so does `spec_trace::collect_rules` | `suite.rs:265-267`; `registry.rs:346-366`; `xtask/src/spec_trace.rs:1755-1766` |
| One enumeration, and only one | `for_each_projection_store_rule!` takes raw token trees (`$($callback:tt)+`), never `$cb:path` — a parsed `path` fragment cannot sit in callee position inside an expression, which would forbid the `let names = …` form the meta-test needs. It is the only place the projection rule set is written; CF-22's "exactly one place" is per rule **family** | `registry.rs:93-102`; `concurrency.rs:1043-1057`; `spec/SPECIFICATION.md:7775` |
| Three emitters, beside the rules | `__emit_projection_tokio` (`#[tokio::test]`), `__emit_projection_blocking` (`#[test]` + `block_on`), `__emit_projection_wasm` (`#[wasm_bindgen_test]`), each spelling the rules module path itself, each `#[doc(hidden)] #[macro_export]`. The event-store emitters are **not** parameterised and not touched | `registry.rs:222-295`; `model.rs:731-780`; `concurrency.rs:1058-1102`; `_decomposition.md:488-515` (Note 4) |
| The outcome must be reported, and the wasm one differently | Every emitter calls `.report(name)` except the wasm one, which calls `skip_line` and hands the result to `console_log!` — `println!` writes nowhere on `wasm32-unknown-unknown`, measured under `wasm-bindgen-test-runner`. `RuleOutcome` is `#[must_use]`, so an emitter that drops it warns and the gate denies warnings | `registry.rs:260-288`; `contract.rs:509-536` |
| The orphan meta-test can fail, in both directions | A projection sibling of `no_orphan_rules`: `include_str!` the projection rules file, strip `    pub async fn `, and assert the declared set and the registered set are equal each way. The registered→declared direction is nearly free anyway (an unregistered path is `error[E0425]` in every harness); the orphan direction is the one CF-24 is about. It must be **fail-loud** on an empty scan, as its sibling is — a scan matching nothing reports every registered rule as missing rather than passing silently | `registry.rs:368-436`; `spec/SPECIFICATION.md:7950` |
| `commit_advances_the_checkpoint` | Open a store, `begin`, `probe_write`, `commit(batch, id, P, Authority::Live)`, then read `checkpoint(&id)` through a **fresh** handle and assert it reports the position the store actually assigned. Never a literal position value. Rustdoc names the wrong implementation it rejects — a `commit` returning `Ok` that makes neither write durable, which PS-1's own text records as admitted by the clause's "or not at all" arm | `spec/SPECIFICATION.md:4733-4759`, `:5661`; CLAUDE.md, "The rule that matters" |
| `commit_is_atomic_with_the_read_model` | Same setup; then read the probe row **and** the checkpoint through fresh handles and assert both present or both absent, never one. Rustdoc names `CheckpointOnlyStore` — a store that commits the checkpoint and discards the write set — as the implementation it rejects, and names the story that lands it | `spec/SPECIFICATION.md:4736-4741`, `:5662`, `:5678-5684`; `_decomposition.md:546-567` |
| A rule whose mutant is one story away is documented, not excused | Each rule's doc names the defect it rejects and the registry entry that will convict it. The mutant itself is `projection-mutant-registry`'s (HS-S0008), which this story `blocks`. Adding a third rule here because it was cheap widens the debt and is out of boundary | `.kb/decisions/0010-the-suite-must-prove-itself.md`; CLAUDE.md, "The rule that matters", first corollary |
| No clock, no literal positions | A conformance rule does not read time (CF-33), and never asserts on `[1, 2, 3]`: the specification permits gaps and a conformant adapter may leave them | CLAUDE.md, "The rule that matters"; `spec/SPECIFICATION.md` §7's clock lint |
| `MemoryProjectionFixture` is the reference implementation | Over `MemoryProjectionStore` behind an `Arc`, `connect` returning an owned handle by refcount bump — `core::future::ready`, not `async move`, so it does not pretend to do I/O. It is a **published item** in `fixtures`, not a test helper: it is what an adapter author reads before writing their own | `fixtures.rs:243-292`; `lib.rs:52-53` |
| Three harnesses, three targets | `tests/projection_conformance.rs` (tokio, default arm), `tests/projection_conformance_blocking.rs` (`#![cfg(not(target_arch = "wasm32"))]`, `emit = __emit_projection_blocking` — libtest does not exist on wasm32), `tests/projection_conformance_wasm.rs` (`#![cfg(target_arch = "wasm32")]`, `emit = __emit_projection_wasm`). No new gate step: the mandatory `cargo check -p happenstance-testkit --tests --target wasm32-unknown-unknown` already type-checks all three | `tests/memory_conformance*.rs`; `xtask/src/main.rs:231-243`; `_decomposition.md:511-515` |
| The feature-table half of the mount | `crates/happenstance-testkit/Cargo.toml`'s `happenstance-core` dependency gains `conformance` beside `std` and `memory`, unconditionally — the testkit gains **no feature of its own**, which keeps every new item ungated and keeps the crate doc free of the intra-doc-link hazard this workspace has already paid for once | `crates/happenstance-testkit/Cargo.toml`; `lib.rs:112-132`; `_decomposition.md:694-699` |
| The crate doc stops describing three families | "Where the rule set lives" says *"'exactly one place' is per rule **family** (CF-22), and there are three"* — it becomes four, and the projection family's paragraph spells module names **plainly**, never as intra-doc links, for the same D13 reason the model and concurrency paragraphs do. The "What is checked" table gains a projection row | `lib.rs:84-92`, `:112-132`, `:134-151` |
| CF-29: a fourth rule file, and the lint that must see it | `RULE_FILES` becomes four entries and its doc is restated. One CHANGELOG entry per rule naming the defect it detects, each carrying ≥120 characters of prose per rule it names — an entry naming both rules is divided between them | `xtask/src/spec_trace.rs:71-89`; `xtask/src/lints.rs:504-580`; `spec/SPECIFICATION.md:8141-8168`, `:8950-8956` |
| spec-trace: the generated region moves, and nothing else may | Two rules starting to exist changes the `†` markers §7.2 renders, so the committed region no longer matches the computed one. `cargo xtask spec-trace --write` is the remedy; editing a row never is. Check 6 then sweeps the projection rules — PS-1 claims both by name, **verify rather than assume**. Check 4 is unaffected: `has_suite` skips `PS-` clauses, and widening it belongs to AC-014's story | `xtask/src/spec_trace.rs:69-89`, `:679-733`, `:1240-1290`, `:1735-1737` |
| Public-surface cost, stated rather than discovered | Three new `#[macro_export]` emitters plus one entry macro are public surface on a crate that carries **its own version** because a suite change can turn a passing adapter's CI red. The version bump itself is not taken here — `publication-and-positioning` (HS-P0016) owns release numbering — but the change is recorded in `CHANGELOG.md` under `[Unreleased]` so the decision has something to read | `crates/happenstance-testkit/Cargo.toml` (the `version` comment); `CHANGELOG.md:13-18`; `_decomposition.md:497-501` |
| The gate this story is measured by | `cargo xtask affected --base main` and `cargo xtask ci --fast` at the story grain, **plus** two steps `--fast` omits and this story can break: `cargo xtask spec-trace` and the wasm32 conformance-harness check. Run both explicitly. `cargo test --workspace --all-features -- --show-output` is what makes a `SKIP` line reachable at all | `CLAUDE.md` Commands; `xtask/src/main.rs:131-155`, `:231-243`; `_decomposition.md:828-853` |
| What to report rather than absorb | (a) The chosen emitter shape needing any edit to the event-store emitters or an existing harness; (b) a baseline rule that cannot be written without reaching past `ProjectionProbe` into an adapter's internals — which would mean PS-11's seam is wrong, an ADR paragraph rather than a quiet widening; (c) check 6 rejecting a rule PS-1 appears to claim; (d) either baseline rule passing against a store that should fail it | `_decomposition.md:709-727` (Note 10); `project.md` Risks |

## Data and migrations

**N/A — no persisted data, no schema and no stored format this story owns or reads.**

Four things that could be mistaken for migrations, and what each actually is:

1. **No storage schema.** The only store this story drives is `MemoryProjectionStore`, and the port is deliberately ignorant of what a read model is — `reset` takes the caller's own deletes for exactly that reason (`spec/SPECIFICATION.md:5165-5170`), and the suite observes rows only through `ProjectionProbe`'s opaque `(key: &str, value: u64)` pairs (`:5003-5013`). No DDL exists in this workspace to migrate.
2. **No wire or on-disk format.** `Checkpoint` and `Authority` cross no boundary here; nothing is serialised, and `serde` stays out of `happenstance-core`'s default features (CLAUDE.md binding constraint 2). The projection suite adds no dependency to any crate — `conformance` is a feature flag on a dependency the testkit already has, which is the coherence argument that put the probe in the contract crate (`spec/SPECIFICATION.md:5015-5031`).
3. **The three data-shaped edits are source registries, not data.** `for_each_projection_store_rule!`'s list, `RULE_FILES`'s array and `CHANGELOG.md`'s entries are all read at compile time or by a lint, and each has a check that fails when it drifts: the orphan meta-test, spec-trace's check 6, and CF-29's lint respectively. That is the point of writing them where a checker can see them rather than in prose.
4. **No compatibility shim for the new public macros.** `happenstance-testkit` is `publish = false`'s opposite — it is one of the three publishable crates — but nothing has been published yet, so there is no prior baseline to be compatible with and no deprecated arm is owed. The same reasoning removed the old `factory =` spelling outright at phase 3 (`lib.rs:305-310`).

## Acceptance criteria

The persona throughout is **P2, the adapter author**
(`.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md:114-153`),
whose stated goal is *"an executable definition of 'correct' they can run against
their own storage system, rather than a prose specification they have to
interpret"*. Today they have exactly that for `EventStore` and **nothing** for
`ProjectionStore` — the port's own header says a port without a conformance suite
is a guess (`crates/happenstance-core/src/projection.rs:3-11`). Where a criterion
is written from **P3, the local-first / edge developer** (`_decomposition.md:58`),
the row says so: P3 is the reader of the third harness and the person for whom a
guarantee that only holds on tokio is not a guarantee.

Two file names are **settled here**, not left to the implementer, so the ledger's
`verifying_test` paths are real on landing: the projection family's module is
`crates/happenstance-testkit/src/projection.rs` (one file holding `pub mod rules`,
the enumeration, the three emitters and the orphan meta-test — the shape
`concurrency.rs` and `model.rs` already have), and the three harnesses are
`crates/happenstance-testkit/tests/projection_conformance{,_blocking,_wasm}.rs`.
If the implementer chooses differently, that is a naming change to record in the
report and reflect in the ledger — not a licence to change what is proven.

Unlike `concurrency` (`lib.rs:172-173`) and `model` (`:180-182`), the projection
module is declared **unconditionally**: both of those are `#[cfg(not(target_arch
= "wasm32"))]`, and a projection module gated the same way would make AC-008
unsatisfiable by construction.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **GIVEN** an adapter author (P2) who has written a `ProjectionStore` impl and wants to know whether it is correct, **WHEN** they add `happenstance-testkit` as a dev-dependency and write the single line `happenstance_testkit::projection_store_conformance!(MyProjectionFixture::new());` in a file under their own `tests/`, **THEN** it expands to **one `#[tokio::test]` per projection rule, each named after the rule**, with no rule name written by the caller and no second line of setup — and **WHEN** a rule fails, the failing test is named after the rule, not after the macro. A macro that compiles but is unreachable from the crate root, or whose expansion cannot name `ProjectionFixture` in the caller's crate, does not satisfy this. | Unit: `crates/happenstance-testkit/tests/projection_conformance.rs` compiling at all — a `tests/` file can name only public items, so `projection_store_conformance!`'s export and the `__private::ProjectionFixture` re-export (`crates/happenstance-testkit/src/lib.rs:359-364`) are load-bearing for the whole file — and `cargo test -p happenstance-testkit --test projection_conformance -- --list` showing exactly `commit_advances_the_checkpoint` and `commit_is_atomic_with_the_read_model`. Static: review that all three arms exist in the order `event_store_conformance!` uses (`lib.rs:265-357`) and that the hoisted fixture fn is spelled `__conformance_fixture`. |
| AC-002 | **GIVEN** the same author, who must declare *how to make an isolated store* rather than hand over one store, **WHEN** they implement `ProjectionFixture`, **THEN** the trait asks for an **owned** `type Store: ProjectionProbe` (never a borrowing GAT, never `SendProjectionStore`) and an `async fn connect` that panics rather than returning `Result` — so "the database is down" can never be reported through the same channel as "the adapter is wrong"; and **WHEN** a rule calls `open()` twice, **THEN** it gets two genuinely isolated backing stores, which is what `commit_rejects_a_foreign_batch` will need one slice later. Binding `ProjectionStore` instead of `ProjectionProbe` in the associated type does not satisfy this: PS-11 makes the probe the thing that turns "you must be observable" into a compile error. | Unit: `crates/happenstance-testkit/src/projection.rs::two_opens_make_two_isolated_stores` — a `#[cfg(test)] #[tokio::test]` that calls the open-closure twice, commits to one, and asserts the other's checkpoint is `NeverRun` and its probe row absent. Static: `cargo check -p happenstance-testkit --all-features` with the trait declared in `crates/happenstance-testkit/src/contract.rs`; review against `contract.rs:97-125` (no GAT, bare flavour) and `:309-321` (panicking `connect`), and that `async fn` is **not** spelled in the public trait declaration (`async_fn_in_trait` under `-D warnings`). |
| AC-003 | **GIVEN** a rule author adding the seventeenth projection rule six months from now, **WHEN** they add it, **THEN** there is **exactly one** place to write its name — `for_each_projection_store_rule!` — and every harness on every runtime picks it up with no edit; **AND GIVEN** a reader of any of the three harness files, **WHEN** they open one, **THEN** it contains **zero** rule names, which is the observable form of CF-22's "exactly one place, per rule family". A second list — in a harness, in the crate doc as the authority, or in a `const RULES: &[&str]` — is the defect this criterion forbids. | Unit: `crates/happenstance-testkit/tests/projection_conformance_wasm.rs` and `..._blocking.rs` each containing one macro invocation and no rule identifier, both picking up the same two rules the tokio harness does. Static: `rg -n "commit_advances_the_checkpoint" crates/happenstance-testkit` returning hits only in `src/projection.rs` (declaration, enumeration, rustdoc) and `CHANGELOG.md` — never in a harness. Review against `spec/SPECIFICATION.md:7775` (CF-22) and `crates/happenstance-testkit/src/concurrency.rs:1043-1057` (the family precedent for raw token trees over `$cb:path`). |
| AC-004 | **GIVEN** a rule author who writes a rule and forgets to register it — the exact failure CF-24 exists for — **WHEN** the test suite runs, **THEN** a meta-test fails **by name**, listing the orphaned rule and saying no harness runs it; **AND WHEN** the enumeration lists a rule the module no longer declares (renamed, moved), **THEN** the same meta-test fails in the other direction; **AND** if the source scan ever matches nothing, it reports every registered rule as missing rather than passing silently. A meta-test that cannot fail in a demonstrable way is decorative. | Unit: `crates/happenstance-testkit/src/projection.rs::no_orphan_projection_rules`, a sibling of `crates/happenstance-testkit/src/registry.rs:410-436`, consuming `for_each_projection_store_rule!(crate::__emit_rule_names)` (that emitter is family-agnostic — `registry.rs:290-295` — and is reused unchanged per AC-A06). Evidence of failability recorded in the implementation report: add a `pub async fn` to the rules module without registering it, observe the named failure, revert. |
| AC-005 | **GIVEN** an adapter author whose `commit` returns `Ok` without making anything durable — legal under PS-1's "or not at all" arm, and therefore an implementation the clause admits and the suite must still reject — **WHEN** they run the suite against a fresh store, **THEN** `commit_advances_the_checkpoint` fails; and **WHEN** their store is correct, **THEN** the rule opens a store, `begin`s, writes through the probe, commits at a position, and reads `checkpoint(&id)` back **through a fresh handle**, asserting it reports the position **the store actually assigned** — never a literal `1`, `2`, `3`, because the specification permits gaps. The rule's rustdoc names the wrong implementation it rejects. | Unit/Integration: `crates/happenstance-testkit/tests/projection_conformance.rs::commit_advances_the_checkpoint` (and its blocking sibling), driving `MemoryProjectionStore`. Static: review that no literal position appears in the rule body (CLAUDE.md, "The rule that matters"), that no clock is read (CF-33), and that the rustdoc names the defect in the `crates/happenstance-testkit/src/suite.rs:253-264` idiom. The clause-attribution caveat is recorded, not resolved, per `.kb/open-questions/ps-1-states-no-progress-obligation.md`. |
| AC-006 | **GIVEN** an adapter that advances the checkpoint and silently drops the read-model write — `CheckpointOnlyStore`, the defect this whole project exists to catch — **WHEN** the suite runs, **THEN** `commit_is_atomic_with_the_read_model` reads the probe row **and** the checkpoint through fresh handles and asserts **both present or both absent, never one**, failing at that assertion and nowhere else; and **GIVEN** a reviewer asking whether the rule is decorative, **THEN** its rustdoc names `CheckpointOnlyStore` and the story that lands it (`projection-mutant-registry`, HS-S0008) — a knowingly-carried one-story debt with a named discharger, not an exemption. Without `probe_write`/`probe_read` the rule cannot see the read model at all and the suite degenerates into a checkpoint test a broken store passes. | Unit/Integration: `crates/happenstance-testkit/tests/projection_conformance.rs::commit_is_atomic_with_the_read_model` against `MemoryProjectionStore` (which commits under one guard, so it passes). Static: review of the rustdoc against `.kb/decisions/0010-the-suite-must-prove-itself.md` and `spec/SPECIFICATION.md:5678-5685`; confirm the read-back goes through `ProjectionProbe` only, never an inherent method on the concrete store. |
| AC-007 | **GIVEN** an adapter author who has never written a fixture and wants something to copy, **WHEN** they look in `happenstance_testkit::fixtures` — the module the specification names by path and the crate doc points at — **THEN** they find `MemoryProjectionFixture` as a **published item with rustdoc**, not a test helper buried in a `#[cfg(test)]` block, handing back an owned handle by refcount bump (`core::future::ready`, not `async move`, so it does not pretend to do I/O), in the same idiom `MemoryFixture` already uses. A fixture reachable only from inside the testkit's own tests fails this: the reference implementation is the documentation. | Unit: `crates/happenstance-testkit/tests/projection_conformance.rs` naming `happenstance_testkit::fixtures::MemoryProjectionFixture` from outside the crate — it compiles only if the item is public. Static: review against `crates/happenstance-testkit/src/fixtures.rs:243-292` (the owned-handle idiom) and `crates/happenstance-testkit/src/lib.rs:52-53`; `cargo doc -p happenstance-testkit` renders it with a doc comment that says what one fixture instance is (one isolated backing store) and what `connect()` is (one handle onto it). |
| AC-008 | **GIVEN** P3, the local-first / edge developer, whose store runs on `wasm32-unknown-unknown` where there is no libtest and no stdout, **WHEN** they pick a harness, **THEN** all three exist and all three run the *same* rule set: `projection_conformance.rs` (tokio, default arm), `projection_conformance_blocking.rs` (`#![cfg(not(target_arch = "wasm32"))]`, `emit = __emit_projection_blocking`, driving `block_on`) and `projection_conformance_wasm.rs` (`#![cfg(target_arch = "wasm32")]`, `emit = __emit_projection_wasm`); **AND WHEN** a rule is skipped on wasm, **THEN** its stated reason reaches a human through `console_log!` rather than through `RuleOutcome::report`, which is a measured no-op on that target. Costs **no new gate step**: the existing mandatory wasm32 conformance-harness check type-checks the new file. | Static: `cargo check --locked -p happenstance-testkit --tests --target wasm32-unknown-unknown` (`xtask/src/main.rs:231-243`) with the new wasm harness present — the whole of project AC-016's harness half. Unit: `cargo test -p happenstance-testkit --test projection_conformance_blocking` passing the same two rules as the tokio harness. Static: review that the wasm emitter calls `skip_line` + `console_log!` and never `report` (`crates/happenstance-testkit/src/registry.rs:274-288`, `crates/happenstance-testkit/src/contract.rs:525-531`; UX brief AC-U11). |
| AC-009 | **GIVEN** an adapter author landing on the testkit's front page — the crate doc and the README are the only orientation they get before writing code — **WHEN** they read "Where the rule set lives" and the "What is checked" table, **THEN** both describe **four** rule families rather than three, the projection family's paragraph says which one line to write and which harness to pick, and every gated module name is spelled **plainly rather than as an intra-doc link**, because a link into a `cfg`-absent module is a hard error under the `--no-default-features` doc build this workspace has already paid for once. A correct macro documented as if it did not exist is the library equivalent of a component that is built and never mounted. | Static: `cargo doc -p happenstance-testkit` and the gate's `--no-default-features` doc build, both clean (a broken intra-doc link is a hard error, not a warning). Review of `crates/happenstance-testkit/src/lib.rs:84-92` (family count), `:134-151` (the table gains a projection row), `:112-132` (the plain-spelling rule and why), and of `crates/happenstance-testkit/README.md`. |
| AC-010 | **GIVEN** the maintainer who found, at stage 6, that CF-29's changelog lint read `suite.rs` alone and printed *"all 55 suite rules have a changelog entry"* — a sentence that is true and answers a different question — **WHEN** a fourth rule file lands, **THEN** it is in `RULE_FILES` and every new rule has a `CHANGELOG.md` entry **naming the defect it detects**, carrying at least 120 characters of prose per rule the entry names; **AND WHEN** `cargo xtask spec-trace` runs, **THEN** check 6 finds both rules claimed by PS-1 and the regenerated §7.1–§7.2 region is the **only** hunk in `spec/SPECIFICATION.md` — no clause body, no maturity marker, nothing `[FROZEN]` line-edited. A rules file absent from `RULE_FILES` reproduces the stage-6 defect one family later. | Static: `cargo xtask spec-trace` green (**run explicitly** — it is one of the two steps `cargo xtask ci --fast` omits); `cargo xtask ci` CF-29 lint step green (`xtask/src/lints.rs:504-580`). Review of `git diff spec/SPECIFICATION.md` showing only the generated region, and of `xtask/src/spec_trace.rs:71-89` showing four entries with the doc comment restated. If check 6 rejects a rule PS-1 appears to claim, that is EC-008 — a finding, not an `UNCLAIMED_PENDING_ADR` entry. |

**Coverage of the traced project ACs.** Project **AC-001** (*one enumeration,
meta-test over orphans*) is discharged in full by AC-003 and AC-004, with AC-001
proving the enumeration is reachable through the one-line entry point. Project
**AC-016** (*every projection rule under the `wasm32` emitter in the same run*)
has its **harness half** discharged by AC-008; the whole-rule-set half is
`whole-gate-run-and-proof-artefact`'s (HS-S0017), per `_storymap.md`'s Coverage
table. AC-002, AC-005 – AC-007, AC-009 and AC-010 are not separately traced to a
project AC: they are what makes this a rule *family* rather than two loose tests,
and each is load-bearing for a project AC a later story owns.

## Interaction quality

**This story renders no visual surface, and that is a signed-off determination
rather than an assumption.** `_design.md` records `surfaces: []`, every one of its
`## Items` / `## Signatures` / `## Anti-patterns` blocks is N/A, and the sign-off
says what was approved is *"the no-surface determination itself"*
(`.bklg/from-contract-to-published-library/projection-store-freeze/_design.md:48-50,92-101`).
`design.capture` is absent from `.redkiln/config.yaml`, so the perceptual review
is a **declared skip**, not a silent pass.

The RFC §6.7/D6 families therefore do not evaporate — they land in the two
non-visual surfaces `_design.md` *does* name (`:24-39`): the **type surface** an
adapter author writes Rust against, and the **text surface** a run prints to a CI
log. Composition authority for those two, in the absence of `_design.md`
`## Items`, is the UX brief's AC-U03 – AC-U12
(`_decomposition.md:110-208`) and `standards/rust/70-rustdoc-obligations.md`,
which is where the intake gate sends us. **Every invariant below is already an
AC-### row in the table above**; this section says which row carries it and how it
is verified — nothing here is a free-floating bullet.

**State family.**

| Invariant | Translated to this medium | Carried by | Verified by |
| --- | --- | --- | --- |
| In-place, not a context jump | The author stays in their own crate and their own `tests/` directory. One line, no vendored harness, no copied rule list, no fork of the testkit. The failure they read is a test name in their own `cargo test` output. | **AC-001** | `tests/projection_conformance.rs` compiling; `-- --list` naming the rules |
| Preserved selection / state across the operation | Each `open()` yields an isolated backing store and each `connect()` a handle onto it, so one rule's writes never disturb another's — the analogue of acting on one item without disturbing the rest. | **AC-002** | `two_opens_make_two_isolated_stores` |
| Reversibility | A registration mistake is recoverable and *told to you*: the orphan meta-test names the rule and says no harness runs it, in both directions, and is fail-loud when the scan matches nothing. Silence is what makes a mistake irreversible. | **AC-004** | `no_orphan_projection_rules` |
| Non-occlusion | No target and no feature set hides the surface: the projection module is declared unconditionally (unlike `concurrency` and `model`), the crate doc's gated names are spelled plainly, and the `--no-default-features` doc build stays green. | **AC-008**, **AC-009** | the wasm32 `--tests` check; `cargo doc --no-default-features` |
| Keyboard reachability → reachability without a special mode | The entry point is reachable from the crate root under default features, by a caller who can name only public items — not behind a `--cfg`, not through a private path, not through `__private` written by hand. | **AC-001**, **AC-007** | the harness files compiling |
| Focus preserved on failure | A failing rule surfaces as a test named after the rule, at the assertion that failed — not as a macro-expansion error or a single opaque `conformance` test. | **AC-001**, **AC-005**, **AC-006** | `-- --list`; the deliberate-orphan transcript in the report |

**Composition family.** For a library, "an unstyled render satisfies every
data-attribute and ARIA assertion" has an exact counterpart: **a correct macro
with a bare doc comment satisfies every compiler check and teaches nobody.** These
are the rows that make that fail.

| Invariant | Translated to this medium | Carried by | Verified by |
| --- | --- | --- | --- |
| Presentation exists at all | Every new public item carries real composed rustdoc, not a restated signature: the entry macro shows the one line, `ProjectionFixture` says what one instance and one `connect()` mean, and each rule's doc **names the wrong implementation it rejects** in the `suite.rs:253-264` idiom. A rule doc that names no defect is how a decorative rule survives review. | **AC-005**, **AC-006**, **AC-007** | review against `standards/rust/70-rustdoc-obligations.md`; `cargo doc` |
| Composition / placement — at the item, not in an ADR | The `#[must_use]` reporting obligation, the wasm no-op, and the "panic rather than `Result`" argument are documented on the items an implementer already has open, not only in this spec. Per UX brief AC-U06, a trap documented only in a decision record is documented where the person who needs it is not. | **AC-002**, **AC-008** | review of `contract.rs`'s new trait docs and the wasm emitter's doc |
| Transience — persistent chrome, revealed, opened on demand | Three tiers, deliberately: the crate doc's family table and README are **persistent chrome**, seen by everyone; the fixture's capability surface and the rules' named defects are **revealed** at the item you are implementing; the seventeen-rule table, the ADRs and §4.11 stay **opened on demand** behind links. | **AC-009** (chrome), **AC-005**/**AC-006** (revealed) | review of `lib.rs:84-92`, `:134-151` |
| Density budget, with its real numbers | **Two rules, not seventeen. Four families, not "several". Three emitters, not a fourth mechanism. One CHANGELOG entry per rule, ≥120 characters of prose per rule an entry names** (`xtask/src/lints.rs:524-580` divides an entry's prose by the number of rules it names, so one bullet naming both must be twice as substantial). Adding a third rule here because it was cheap widens a debt whose discharger is one story away. | **AC-010**, and the PR boundary | the CF-29 lint; review of the rule count |
| Hierarchy | Crate doc → module doc → item doc, each answering a different question and none repeating the one below: the crate doc says a fourth family exists and which line to write; the module doc says why a projection rule may not live in `suite.rs`; the item doc says what this rule rejects. Mirrors `concurrency.rs:1027-1031`. | **AC-009** | review against `concurrency.rs`'s structure |
| Named anti-pattern — the second rule list | A `const RULES: &[&str]`, a hand-written list in a harness, or a crate-doc table treated as the authority. CF-22 is "exactly one place" *per family*; a second list is the thing the orphan meta-test exists because nobody notices. | **AC-003** | the `rg` sweep; `no_orphan_projection_rules` |
| Named anti-pattern — the gated intra-doc link | A link into a `cfg`-absent module is a **hard** rustdoc error under the gate's `--no-default-features` doc build. This workspace paid for it once and wrote down why (`crates/happenstance-testkit/src/lib.rs:112-132`); the projection paragraph inherits the plain spelling. | **AC-009** | `cargo doc -p happenstance-testkit --no-default-features` |
| Named anti-pattern — the reason that disappears on the constrained target | `RuleOutcome::report` is a measured no-op on `wasm32-unknown-unknown`. A wasm emitter that calls `report` compiles, runs, passes, and silently discards every stated skip reason — AC-016's run would be the one where the text surface vanishes. | **AC-008** | review of the wasm emitter against `registry.rs:274-288`; `contract.rs:525-531` |
| Named anti-pattern — a capability constant no rule reads | Declaring capabilities on `ProjectionFixture` "for symmetry" when the two baseline rules gate on none is exactly the decorative shape CLAUDE.md's corollary names. The capability set is DT-3's, recorded in `_design.md`, and landed by `projection-capability-skips`. | **AC-002**, and the PR boundary | review of the trait's declared constants against the rules that read them |
| Named anti-pattern — the fourth mechanism | Reaching for a new registration mechanism (a build script, a `linkme`-style distributed slice, a proc macro) instead of the three-emitter shape both existing second families took. AC-A01 requires the way the emitters are made to drive it be a **recorded decision**, which the Context pack made. | **AC-003** | review against `model.rs:712-800`, `concurrency.rs:1024-1160` |

## Error conditions

| id | condition | required behaviour |
| --- | --- | --- |
| **EC-001** | A name is registered in `for_each_projection_store_rule!` with no `pub async fn` behind it | `error[E0425]` in **every** harness — this direction is free, because each emitter expands to `$crate::projection::rules::$name`. It must stay free: an emitter that resolves rules through a `HashMap<&str, fn>` or any string indirection converts a compile error into a run-time one and is forbidden. AC-003. |
| **EC-002** | A rule exists in the module and is **not** registered | `no_orphan_projection_rules` fails, naming the rule and stating that no harness runs it. This is CF-24's whole subject; it is the direction Rust's lack of reflection makes non-free. AC-004. |
| **EC-003** | The textual scan matches nothing (the rules module is renamed, reformatted, or `include_str!` points at the wrong file) | **Fail loud, never fail open.** The second assertion reports every registered rule as missing, exactly as `registry.rs:385-386` describes for the sibling. A scan that silently matches nothing is a meta-test that has quietly stopped existing. AC-004. |
| **EC-004** | A fixture's `connect()` cannot reach its backing store | **Panic**, do not return `Result`. A broken test environment is not a non-conformant adapter, and one channel for both makes every CI failure ambiguous (`crates/happenstance-testkit/src/contract.rs:309-321`). AC-002. |
| **EC-005** | A borrowing GAT (`type Store<'a> where Self: 'a`) is introduced on the fixture trait | A rustc **ICE**, still reproducing on 1.97.1 — one of five independently necessary ingredients minimised in `experiments/rustc-ice-gat-foreign-trait/`. Not a workaround to find: hand back an owned handle holding a refcount. AC-002. |
| **EC-006** | An emitter drops the `RuleOutcome` a rule returns | `RuleOutcome` is `#[must_use]`, so this is a warning — and the gate runs `-D warnings`, so it is a build failure. Do not silence it with `let _ =`; the outcome is the only thing that carries a skip's reason. AC-008. |
| **EC-007** | The wasm emitter calls `.report(name)` | Compiles, runs, passes — and discards every stated reason, because `report` is a measured no-op on `wasm32-unknown-unknown`. Route `skip_line` to `console_log!` instead (`registry.rs:274-288`). This failure is invisible to every automated check; the review is the instrument. AC-008. |
| **EC-008** | `cargo xtask spec-trace` check 6 reports a projection rule as unclaimed by any clause | **Report, do not absorb.** PS-1 names both baseline rules in its own `Rule:` field (`spec/SPECIFICATION.md:4736-4741`), and check 6 reads the parsed field rather than §4.11's table — so a rejection means the field and the table disagree. Adding the rule to `UNCLAIMED_PENDING_ADR` to get green is the wrong move; the finding belongs to `unstable-projection-gate-and-clause-disposition` (HS-S0016). AC-010. |
| **EC-009** | CF-29's lint fails: a rule with no CHANGELOG entry, or an entry with less than 120 characters of prose per rule it names | Write the prose. The lint divides an entry's prose by the number of rules named, so one bullet covering both baseline rules needs roughly twice the substance of one covering a single rule (`xtask/src/lints.rs:524-580`). Splitting into two entries is the other legal answer; deleting the rule from `RULE_FILES` is not. AC-010. |
| **EC-010** | The committed §7.1–§7.2 region no longer matches the computed one | Expected, and the remedy is **always** `cargo xtask spec-trace --write` — never editing a row by hand (`xtask/src/spec_trace.rs:1240-1290`, `:1317-1320`). Two rules starting to exist changes the `†` markers; that is the only permitted hunk in `spec/SPECIFICATION.md`. AC-010. |
| **EC-011** | The chosen emitter shape turns out to need an edit to `for_each_event_store_rule!`, the three event-store emitters, `event_store_conformance!`'s expansion, or any of the five existing harnesses | **Halt and report.** Note 10 item 3 makes this a reportable reshape (`_decomposition.md:709-727`), and the option this spec recorded (a family that owns its own emitters) exists precisely so it cannot happen quietly. Do not "just parameterise" the existing emitters mid-story. |
| **EC-012** | A baseline rule cannot be written without reaching past `ProjectionProbe` into an adapter's internals | **Halt and report.** That would mean PS-11's seam is wrong, which is an ADR paragraph and a finding against `projection-probe-conformance-feature` (HS-S0005) — not a quiet widening of the probe or an inherent method on the concrete store. `_storymap.md`, "What would reshape this map", item 3. |
| **EC-013** | A dependency's shape is wrong when the story starts — `type Batch` still carries a lifetime, `ProjectionProbe` is absent, or `MemoryProjectionStore` does not implement it | **Stop before the first edit.** A fixture written against a port that has not changed shape is a rewrite, not a start. The three preconditions are checkable in one minute (Context pack) and are findings against HS-S0004 / HS-S0005 / HS-S0006, not edits to absorb here. |
| **EC-014** | Either baseline rule passes against a store that ought to fail it | A finding, and the sharpest one this story can produce: it means the rule is decorative *before* the mutant arrives. Record it in the implementation report and hand it to `projection-mutant-registry` (HS-S0008), whose exactness meta-tests are the mechanism that would otherwise discover it three weeks later. `.kb/decisions/0010-the-suite-must-prove-itself.md`. |

## Non-functional

| id | requirement | why, and how it is held |
| --- | --- | --- |
| **NF-001** | **No new dependency on any crate, in any feature combination.** | The projection suite costs `conformance` on a `happenstance-core` dependency `happenstance-testkit` already has — which is the coherence argument that put the probe in the contract crate in the first place (`spec/SPECIFICATION.md:5015-5031`) and the cost DT-8's outside-author arm is priced at (UX brief AC-U02). `cargo deny check` and the `[dependencies]` diff are the check. |
| **NF-002** | **`happenstance-testkit` gains no feature of its own.** | Every new item stays ungated, which keeps the crate doc free of the intra-doc-link hazard (AC-009, EC-009's sibling) and keeps the feature powerset the same size. If a projection item ever *needs* a gate, that is a decision to record, not a convenience. `_decomposition.md:694-699`. |
| **NF-003** | **Everything new compiles for `wasm32-unknown-unknown`.** | The projection module is declared unconditionally, so it is compiled on the constrained target — no threads, no clock, no `std::thread`, nothing `concurrency.rs` is target-gated for. `standards/rust/52-wasm32-and-target-cfg.md`; the mandatory `--tests` check at `xtask/src/main.rs:231-243`. |
| **NF-004** | **No `#[async_trait]`, anywhere, ever.** | It injects `+ Send` and makes the wasm32 target impossible. `ProjectionFixture` spells the desugared `fn connect(&self) -> impl Future<Output = Self::Store>`, which also makes the *absence* of `+ Send` visible at the declaration. ADR-0001 (`.kb/decisions/0001-async-port-flavours.md`), and CLAUDE.md binding constraint 1. |
| **NF-005** | **`Capability`, `RuleOutcome`, `block_on` and `__emit_rule_names` are reused unchanged — not forked, not widened.** | An author reading one CI log must not have to learn two skip vocabularies (UX brief AC-U08), and a widened `RuleOutcome` changes a type five harnesses already depend on. Architecture brief AC-A06. Check: the `git diff` of `contract.rs` contains **only** the new trait. |
| **NF-006** | **`-D warnings` clean, including `async_fn_in_trait`.** | `async fn` in a public trait declaration fires the lint; the desugared form is not a style preference here but the thing that keeps the gate green. `cargo clippy --workspace --all-targets --all-features -D warnings`. |
| **NF-007** | **Public-surface growth is stated, not discovered.** | Four new exported macros (`projection_store_conformance!`, `for_each_projection_store_rule!`, three `#[doc(hidden)]` emitters) on a crate that carries **its own version** precisely because a suite change can turn a passing adapter's CI red. Recorded under `CHANGELOG.md`'s `[Unreleased]`; the version number itself is `publication-and-positioning`'s (HS-P0016), not taken here. `standards/rust/40-public-surface-and-evolution.md`. |
| **NF-008** | **MSRV 1.97.1, and no silent movement.** | ADR-0029 raised it deliberately; what stays forbidden is moving it without an ADR (CLAUDE.md binding constraint 5). Let-chains are available. Nothing in a macro family and two rules should approach the floor; the `msrv` CI job is the check. |
| **NF-009** | **The suite's own run time stays a rounding error.** | Two rules against an in-memory store, three harnesses. If a rule ever needs a sleep, a timeout or a retry loop to be reliable, it is reading a clock (CF-33) and is the wrong rule — a conformance rule asserts on values the store returned, not on elapsed time. |

## Implementation notes (non-prescriptive)

Shape only. The signatures of the port, the probe and the reference store are
HS-S0004's, HS-S0005's and HS-S0006's respectively, and this story implements
*against* them.

**Order that keeps the feedback tight.** (1) Precondition check — the three
dependency shapes, one minute, per EC-013. (2) `ProjectionFixture` in
`contract.rs` and `MemoryProjectionFixture` in `fixtures.rs`, with nothing calling
them; `cargo check` proves the trait is inhabitable before a rule depends on it.
(3) `src/projection.rs` with `pub mod rules` holding **one** rule, the
enumeration, and the tokio emitter; the tokio harness; green. (4) The second rule
— now the enumeration's one-line-per-rule claim is observable rather than
asserted. (5) The blocking and wasm emitters and their two harnesses. (6) The
orphan meta-test, and the deliberate-orphan transcript for AC-004. (7) The mount:
`lib.rs` module list, public re-export, `__private`, `Cargo.toml`'s `conformance`
feature. (8) Crate doc, README, `RULE_FILES`, CHANGELOG. (9) `cargo xtask
spec-trace --write`, then `cargo xtask spec-trace`. Doing (7) before (3) makes a
mount failure and a rule failure look alike; doing (8) first makes the CF-29 lint
fail against rules that do not exist yet.

**Copy `concurrency.rs`, not `registry.rs` + `suite.rs`.** The event-store family
is split across two files for historical reasons; the two *second* families are
each one file, and `crates/happenstance-testkit/src/concurrency.rs:1024-1160` is
the closest match to what this story needs — an enumeration, three emitters and an
entry macro sitting beside the rules they name, with the reasoning already written
down at `:1027-1031` (why a rule may not live in `suite.rs`) and `:1140-1147` (why
`__conformance_fixture` is spelled identically across families). Read those two
comment blocks before writing the first macro.

**The orphan scan's prefix is load-bearing twice.** `    pub async fn ` at one
level of indentation inside `pub mod rules` is what the meta-test matches *and*
what `xtask/src/spec_trace.rs:1755-1766`'s `collect_rules` matches for check 6. A
shared helper between two rules is spelled `    async fn` without `pub`, which the
prefix correctly excludes — and a helper made public would be reported as an
orphan, which is the right answer rather than a false positive
(`registry.rs:388-392`).

**Two rules, and resist the third.** The rest of §4.11 is differential against
these two (`spec/SPECIFICATION.md:4735-4741`). A third rule landed here is a third
rule with no mutant, and `projection-mutant-registry` — the very next story — has
an exhaustiveness meta-test that will demand one. The debt is bounded because the
count is.

**Where `MemoryProjectionFixture` goes, and where it does not.** `fixtures.rs`, as
a published item beside `MemoryFixture`, because it *is* the documentation an
adapter author copies. Not in `tests/`, not `#[cfg(test)]`, not a private helper
in `projection.rs`.

**What to write in each rule's rustdoc before writing its body.** The wrong
implementation it rejects, in one sentence, in `suite.rs:253-264`'s idiom. If that
sentence is hard to write, the rule is probably decorative and the difficulty is
the signal — not something to work around by writing the body first and the doc
after.

## Tests and CI (merge gate)

Tier vocabulary is this project's own (`_decomposition.md` Testing brief:
**Static** reads source/config without executing the code under test; **Unit** is
`cargo test` in process, including meta-tests over registries; **Integration** is a
conformance rule actually driving a `ProjectionStore` implementation; **E2E** is
`cargo xtask ci` run whole). This is the first story in the project with an
**Integration** row at all — before it, no projection rule existed to drive
anything.

| tier | command / path | proves |
| --- | --- | --- |
| Static | `cargo fmt --check`; `cargo clippy --workspace --all-targets --all-features -D warnings` | House style and the lint floor; catches `async_fn_in_trait` on the public trait and a dropped `#[must_use]` `RuleOutcome` (NF-006, EC-006). |
| Static | `cargo check -p happenstance-testkit --all-features` | `ProjectionFixture` is inhabitable and the three emitters expand; an unregistered rule name is `error[E0425]` here (AC-002, EC-001). |
| Static | `cargo check --locked -p happenstance-testkit --tests --target wasm32-unknown-unknown` (`xtask/src/main.rs:231-243`) | **Project AC-016's harness half.** The new `projection_conformance_wasm.rs` type-checks on the constrained target, inside a gate step that already exists — no new CI job (AC-008, NF-003). |
| Static | `cargo doc -p happenstance-testkit`; the gate's `--no-default-features` doc build | The crate doc describes four families and no gated name is an intra-doc link — a hard error, not a warning (AC-009). |
| Static | `cargo xtask spec-trace` — **run explicitly**, it is one of the two steps `--fast` omits | Check 6 sweeps the projection rules and PS-1 claims both; the regenerated §7.1–§7.2 region matches (AC-010, EC-008, EC-010). |
| Static | the CF-29 changelog lint inside `cargo xtask ci` (`xtask/src/lints.rs:504-580`) | `RULE_FILES` has four entries and every new rule has an entry naming the defect it detects, with ≥120 characters of prose per rule named (AC-010, EC-009). |
| Static | `git diff spec/SPECIFICATION.md`; `git diff crates/happenstance-testkit/src/contract.rs` | The only spec hunk is the generated region; the only `contract.rs` hunk is the new trait — nothing `[FROZEN]` line-edited, `Capability`/`RuleOutcome` unforked (AC-010, NF-005). |
| Static | `cargo deny check`; `[dependencies]` diff of `crates/happenstance-testkit/Cargo.toml` | The only dependency change is `conformance` added to an existing `happenstance-core` entry (NF-001, NF-002). |
| Unit | `crates/happenstance-testkit/src/projection.rs::no_orphan_projection_rules` | CF-24 for the projection family, in both directions, fail-loud on an empty scan (AC-004, EC-002, EC-003). |
| Unit | `crates/happenstance-testkit/src/projection.rs::two_opens_make_two_isolated_stores` | The fixture really makes isolated stores rather than handing back one shared handle — the property `commit_rejects_a_foreign_batch` will depend on (AC-002). |
| Unit | `cargo test -p happenstance-testkit --test projection_conformance -- --list` | Exactly two tests, each named after its rule; the caller wrote one line and named no rule (AC-001, AC-003). |
| Integration | `crates/happenstance-testkit/tests/projection_conformance.rs::commit_advances_the_checkpoint` | A real `begin` → `probe_write` → `commit` → fresh-handle `checkpoint` round trip against `MemoryProjectionStore`, compared against the position the store assigned (AC-005). |
| Integration | `crates/happenstance-testkit/tests/projection_conformance.rs::commit_is_atomic_with_the_read_model` | The read model is observed through `ProjectionProbe` and coupled to the checkpoint — both or neither. This is the assertion `CheckpointOnlyStore` will fail, one story later (AC-006). |
| Integration | `cargo test -p happenstance-testkit --test projection_conformance_blocking` | The same two rules under `block_on` rather than `#[tokio::test]` — the runtime-independence claim, on the one target where libtest exists and tokio may not be wanted (AC-008). |
| Unit | `cargo test --workspace --all-features -- --show-output` | Nothing in the five existing harnesses, `mutation_coverage.rs` or the examples regressed from a fourth family entering the crate (EC-011). `--show-output` is what makes any reported line reachable by a human at all. |
| E2E | `cargo xtask affected --base main` and `cargo xtask ci --fast` | **This story's merge bar** — the story/slice grain the project's non-terminal `--fast` rule sets (`_decomposition.md` Testing brief, "Merge-gate commands"). |
| E2E | `cargo xtask ci` (whole) | The **project** boundary's bar, met at `whole-gate-run-and-proof-artefact` (HS-S0017), not here. Named so the grains are not confused: `--fast` omits `spec-trace` and the mandatory wasm32 steps, and **two of this story's ten ACs (AC-008, AC-010) are proven by exactly those two** — so run them directly even at story grain. |

**Deliberately absent.** No mutant-registry meta-test row (`every_rule_has_a_mutant`
and its two siblings are HS-S0008's), no `RuleOutcome::Skipped` assertion row
(HS-S0009's), and no second-fixture row (HS-S0011's). Inventing any of the three
here would mean building the next three stories inside this PR.

## Risks and coupling (PR-scoped)

| risk | why it bites here | the move |
| --- | --- | --- |
| **The three dependencies land in the same project and a signature is still warm.** `owned-batch-port-shape`, `projection-probe-conformance-feature` and `memory-projection-store` merge in the slice immediately before this one. | The temptation is to "just fix" a port or probe signature while writing the first thing that consumes it — which would make the port's shape a residue of what was convenient for a rule, the exact failure AC-008's ADR-before-port ordering exists to prevent. | Any signature change is a finding against HS-S0004/HS-S0005/HS-S0006 and an ADR-0017 clause. Report and stop (EC-012, EC-013); do not edit `crates/happenstance-core/src/projection.rs` at all in this PR. |
| **Note 4's option 1 looks cheaper mid-implementation.** Parameterising the three event-store emitters over a rules-module path is two lines of `$path:path` and removes three macro definitions. | It touches macros that five harnesses and `mutation_coverage.rs` already expand. A subtle arm-matching change there fails at a distance, in files this PR is not allowed to edit. | The Context pack already recorded option 2 on the evidence of two families that took it. If option 1 is attempted anyway and anything existing needs an edit, that is EC-011 — halt and report, per `_storymap.md`, "What would reshape this map", item 2. |
| **`spec-trace` is not in `--fast`, so a green story-grain gate hides AC-010 entirely.** | The failure then arrives at the project boundary, attributed to whichever story happens to be running the full gate — probably HS-S0017, three slices later. | Run `cargo xtask spec-trace` explicitly before calling this story done, and put its output in the implementation report. Same for the wasm32 `--tests` check. |
| **The two rules pass on the first run and nobody asks whether they can fail.** | `MemoryProjectionStore` is the oracle: it is *supposed* to pass. A green run proves the machinery runs, not that it discriminates — and that difference is the whole of ADR-0010. | Record the deliberate-orphan transcript for AC-004, and state plainly in the report that discrimination is unproven until HS-S0008. Do not write "the projection suite passes" without that sentence. |
| **PS-1's clause admits an implementation `commit_advances_the_checkpoint` rejects** (`.kb/open-questions/ps-1-states-no-progress-obligation.md`). | A reviewer may read the mismatch as a defect in *this rule* and either weaken it or re-attribute the clause in passing — widening a `[FROZEN]` clause by side effect, which AC-A08 forbids. | Write the rule as §4.11's table specifies, cite the open question in its rustdoc as *known and owned elsewhere*, and change nothing in `spec/SPECIFICATION.md` beyond the generated region. The disposition is HS-S0016's, scoped by `ps-clause-pairing-sweep`. |
| **The model and concurrency families have no orphan meta-test.** Grep confirms `no_orphan_rules` covers `suite.rs` alone (`registry.rs:410-436`). | Writing the projection sibling makes the gap obvious and the two extra copies feel like a ten-minute win — inside a PR whose boundary is already wide. | Out of boundary. Report it as a finding; CF-24's coverage across all four families is a decision someone should take deliberately, not a side effect of this story. |
| **Three new `#[macro_export]` macros are permanent public surface.** | `happenstance-testkit` carries its own version precisely because a suite change can turn a passing adapter's CI red; macros are the hardest surface to walk back. | Name them in `CHANGELOG.md` under `[Unreleased]` with what each is for, and mark the two family-internal emitters `#[doc(hidden)]` as the existing three are. The version decision stays HS-P0016's (NF-007). |
| **Scope creep into HS-S0009.** | A capability constant on `ProjectionFixture` is one line and looks like symmetry with `Fixture`'s `SECOND_HANDLE` / `REOPEN`. | A capability no rule reads is decorative. The set is DT-3's, recorded in `_design.md`, landed by `projection-capability-skips`. Declare only what the two baseline rules gate on — which is none — and say so. |

## Dependencies

**Blocks on** — all three are `foundation` stories in the preceding slice
`projection-port-and-probe`, all three are in-tree, none is a double, and
`_storymap.md`'s Merge order 2 → 3 puts every one of them before this story:

- **`owned-batch-port-shape`** (HS-S0004) — supplies `type Batch;` with **no
  lifetime**, the non-`async` infallible `begin`, and `Checkpoint`, `Authority`,
  `CommitError`, `ResetError`. Both baseline rules call `begin` and `commit`
  directly; with the GAT still on the port there is nothing a fixture can hold.
- **`projection-probe-conformance-feature`** (HS-S0005) — supplies
  `ProjectionProbe` and `happenstance-core`'s `conformance` feature.
  `ProjectionFixture::Store` is bound on the probe, and without
  `probe_write`/`probe_read` AC-006 cannot observe the read model at all.
- **`memory-projection-store`** (HS-S0006) — supplies `MemoryProjectionStore`
  behind `memory`, implementing `ProjectionProbe` under `conformance`. It is what
  `MemoryProjectionFixture` wraps and what both rules run against; without it
  there is a suite and nothing to run it on.

The slice also depends transitively on `decisions-and-design-record` completing
first — project AC-008's "ADRs accepted *before* the port change lands" ordering
is itself the check — so ADR-0017/0018/0019 are accepted atoms before this story
starts, and nothing here writes or amends one.

**Unlocks** —

- **`projection-capability-skips`** (HS-S0009) — this story's slice-mate, mounted
  as the same integrated surface and implemented immediately after it. It asserts
  on `RuleOutcome` values produced by the machinery landed here, and lands the
  capability constants this story deliberately does not invent.
- **`projection-mutant-registry`** (HS-S0008) — names this story in its
  `depends_on`; `CheckpointOnlyStore` fails `commit_is_atomic_with_the_read_model`
  by name, which is the first moment DoD 7 becomes readable. It is also what
  discharges this story's carried debt (each rule's doc names a defect; the store
  that embodies it arrives there).
- **`commit-rollback-and-drop-rules`**, **`reset-rules`**,
  **`read-through-and-rebuild-rules`** — every later rule is registered in the
  enumeration this story creates and emitted by the three emitters it writes.
- **`buffering-conformant-variant`** (HS-S0011) — the second batch shape is a
  second `ProjectionFixture` impl; the trait declared here is what it implements.
- **`whole-gate-run-and-proof-artefact`** (HS-S0017) — completes project AC-016 by
  running the *whole* rule set under the wasm32 emitter, through the harness file
  this story adds.

**No cycle.** Every edge runs forward through `_storymap.md`'s merge order:
slice 2 → this story (slice 3, position 1) → slice 3 position 2 → slices 4–8.

## Anchors (progressive disclosure)

Open these when the row says to. Everything load-bearing is linked rather than
pasted — but nothing load-bearing is optional.

| anchor | why it is load-bearing | when to open | serves AC-### |
| --- | --- | --- | --- |
| `crates/happenstance-testkit/src/concurrency.rs` | **The template.** The closest existing match to what this story builds: `:311` (`pub mod rules`), `:1024-1160` (enumeration, three emitters, entry macro, all beside the rules), `:1027-1031` (why a rule may not live in `suite.rs`), `:1043-1057` (why the enumeration takes raw token trees, not `$cb:path`), `:1140-1147` (why `__conformance_fixture` is spelled identically across families). Copying without reading those four comment blocks reproduces the shape and loses the reasons. | Before writing the first macro — this is step 3 of the implementation order. | AC-001, AC-003, AC-008 |
| `crates/happenstance-testkit/src/registry.rs` | The event-store originals and the two mechanisms this story mirrors: `:93-220` the enumeration, `:222-295` the three emitters (**read, do not edit** — EC-011), `:274-288` the wasm emitter's `skip_line` + `console_log!` routing, `:290-295` `__emit_rule_names` (family-agnostic, reused unchanged), `:330-344` `block_on`, `:346-366` the scan prefix, `:368-436` `no_orphan_rules` with its long doc on what a textual scan can and cannot catch. | `:346-436` before writing the orphan meta-test; `:222-295` before writing the emitters. | AC-003, AC-004, AC-008 |
| `crates/happenstance-testkit/src/contract.rs` | Where `ProjectionFixture` lands and what it must mirror: `:88-125` (`Fixture`, and why `Store: EventStore` is the wrong bound here), `:97-111` (the GAT ICE, five ingredients, still live on 1.97.1), `:309-321` (why `connect` panics rather than returning `Result`), `:355-433` `Capability`, `:458-537` `RuleOutcome` including `:500-507` `skip_line` and `:525-531` the measured wasm no-op. All of it is reused; only the new trait is added. | Before writing the trait (step 2), and again before the wasm emitter. | AC-002, AC-007, AC-008 |
| `crates/happenstance-testkit/src/lib.rs` | **The mount, both halves.** `:84-92` the "three families" sentence that becomes four, `:112-132` the plain-spelling rule for gated module names *and the reason*, `:134-151` the "What is checked" table, `:160-189` the module list (note `concurrency` and `model` are target/feature-gated and the projection module must **not** be), `:265-357` `event_store_conformance!`'s three arms in the order arm-matching requires, `:359-364` `__private`. | At the mount step (step 7), and `:265-357` before writing the entry macro. | AC-001, AC-009 |
| `crates/happenstance-testkit/src/suite.rs` | `:89` (`pub mod rules`) for the module shape the scan depends on, `:253-264` for the rule-rustdoc idiom — a rule's doc naming the registered mutant *and* the two others that fail it for unrelated reasons — and `:265-267` for the rule signature (`impl AsyncFn() -> F`, not a made fixture). | Before writing either rule's doc comment and signature. | AC-005, AC-006 |
| `crates/happenstance-testkit/src/fixtures.rs` | `:243-292` — `MemoryFixture`'s owned-handle idiom: an `Arc` clone through `core::future::ready`, not `async move`, so the fixture does not pretend to do I/O. This is what `MemoryProjectionFixture` copies, and it is the item an adapter author reads before writing their own. | Before writing `MemoryProjectionFixture` (step 2). | AC-002, AC-007 |
| `crates/happenstance-testkit/tests/memory_conformance_wasm.rs` and `..._blocking.rs` | The two `cfg` gates that are mirror images and are easy to get backwards: the blocking harness is `#![cfg(not(target_arch = "wasm32"))]` because `__emit_blocking` emits a plain `#[test]` and libtest does not exist on wasm32; the wasm one is `#![cfg(target_arch = "wasm32")]`. `..._blocking.rs:14-28` carries the explanation. | Before writing the two non-default harness files (step 5). | AC-008 |
| `xtask/src/spec_trace.rs` | The three consumers a fourth rule file acquires: `:71-89` `RULE_FILES` and its *"Three, not one"* doc, `:679-733` and `:726-800` check 6's clause sweep, `:1114` the `†` marker for a rule that does not exist, `:1240-1290` and `:1317-1320` the regenerate-never-edit rule, `:1735-1737` why `has_suite` skips `PS-` clauses (and why widening it is **not** this story's), `:1755-1766` `collect_rules`'s prefix, which must match the orphan scan's. | Before editing `RULE_FILES` (step 8), and immediately if `spec-trace` fails. | AC-010 |
| `xtask/src/lints.rs` | `:504-580` — CF-29's changelog lint, including `:511-519`'s record of the stage-6 defect (*"all 55 suite rules have a changelog entry"* — true, and an answer to a different question) and `:524-580`'s ≥120-characters-per-rule arithmetic, which divides an entry's prose by the number of rules it names. | Before writing the CHANGELOG entries (step 8). | AC-010 |
| `xtask/src/main.rs` | `:131-155` the `tests` step and its `--show-output`; `:231-243` the mandatory wasm32 conformance-harness check — the step that makes AC-008 cost a file rather than a CI job. Tells you exactly which command reproduces a failure locally. | When claiming AC-008, and when an AC-008 or AC-010 check fails. | AC-008, AC-010 |
| `spec/SPECIFICATION.md` §4.11 and the PS clauses | The normative source, read by range and never whole: `:4733-4759` PS-1 (`[FROZEN]`, the baseline both rules serve), `:4736-4741` PS-1's `Rule:` field (what check 6 actually reads), `:4977-4986` PS-11 (the probe as an obligation, "cannot invoke the suite"), `:5003-5031` `ProjectionProbe` and the coherence argument, `:5652-5704` §4.11's seventeen rules, `:5661-5662` the two rows this story lands, `:5678-5685` `CheckpointOnlyStore`. | Open the specific range as you implement each behaviour. | AC-002, AC-005, AC-006 |
| `spec/SPECIFICATION.md` CF clauses | `:7775` CF-22 (exactly one place, **per family**), `:7920` CF-23 (the caller-supplied emitter extension point), `:7950` CF-24 (the orphan obligation), `:8141-8168` CF-29 (the changelog obligation), `:8950-8956` the stage-6 review finding that widened it to `RULE_FILES`. | Before the enumeration (CF-22), the emitters (CF-23), the meta-test (CF-24) and the changelog (CF-29). | AC-003, AC-004, AC-008, AC-010 |
| `.kb/open-questions/ps-1-states-no-progress-obligation.md` | The thing that will look like a bug while writing `commit_advances_the_checkpoint`: PS-1's MUST is a **coupling**, not a progress obligation, so a `commit` returning `Ok` that makes neither write durable satisfies the clause and fails the rule. Open it so the impulse to "fix" the clause or weaken the rule is spent in thirty seconds rather than in a PR. | The moment PS-1's text and §4.11's rule table appear to disagree. | AC-005 |
| `.kb/decisions/0010-the-suite-must-prove-itself.md` | Accepted and immutable. The reason a green run here is **not** evidence the suite discriminates, and the reason each rule's doc must name the wrong implementation it rejects even though the store that embodies it is one story away. | Before writing either rule's doc comment, and before writing the implementation report's summary sentence. | AC-005, AC-006 |
| `.kb/decisions/0001-async-port-flavours.md` | Accepted and immutable. Why `#[async_trait]` is never an option and why the fixture's `connect` is spelled as a desugared `-> impl Future`, with the absence of `+ Send` visible at the declaration. | Before writing the trait declaration. | AC-002 |
| `.bklg/from-contract-to-published-library/projection-store-freeze/_decomposition.md` | The briefs. Architecture **Note 1** (`:339-360`, the two-place mount for a library), **Note 4** (`:488-515`, the write-vs-rule-running seam this story's Context pack resolved), **Note 5** (`:517-544`, the fixture contract and who owns the capability set), **Note 6** (`:546-567`, what one rule does step by step), **Note 10** (`:709-727`, what to report rather than absorb); UX **AC-U08 – AC-U12** (`:159-208`, the text surface); Testing brief rows **AC-001**, **AC-005**, **AC-016**. | Note 6 before writing a rule body; Note 5 before the trait; Note 1 at the mount; Note 10 the moment something does not fit. | AC-001, AC-002, AC-005, AC-006, AC-008 |
| `.bklg/from-contract-to-published-library/projection-store-freeze/_design.md` | The signed-off design. `:10-50` (the two non-visual surfaces, and `surfaces: []`) and `:92-101` (the sign-off, which approved the no-surface determination itself). Read it to confirm the Interaction-quality section is written against a decided thing rather than re-deciding one. | Before any rustdoc or crate-doc work (step 8). | AC-007, AC-009 |
| `.bklg/from-contract-to-published-library/projection-store-freeze/_storymap.md` | This story's row and its dependencies, the Coverage table's AC-001 and AC-016 rows (which half belongs here), Merge order item 3, and "What would reshape this map" items 2 and 3 — the two reshapes this story is most likely to trigger. | Once before starting; again if EC-011 or EC-012 fires. | AC-003, AC-008 |
| `.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md` | `:114-153` — P2's journey, the sentence every AC in this story is written from: an executable definition of correct, not a prose specification to interpret. `_decomposition.md:58` for P3, whose whole share of this story is the third harness. | Before writing or reviewing any AC, and before the crate-doc paragraph. | AC-001, AC-008, AC-009 |
| `standards/rust/41-declarative-macros.md` | The house rules for `macro_rules!`: fragment specifiers and what a parsed `path` forbids in callee position, arm ordering, `#[macro_export]`'s hygiene consequences, and `$crate`. Four new macros is the largest macro surface added in one PR in this repository. | Before writing the enumeration and the entry macro. | AC-001, AC-003 |
| `standards/rust/40-public-surface-and-evolution.md` | What exporting a macro commits the crate to, and RS-40-5 (the capability-with-a-reason shape the fixture trait inherits without declaring constants here). | At the mount step, and when writing the CHANGELOG entry for the new macros. | AC-002, AC-010 |
| `standards/rust/60-what-a-test-must-prove.md` | The atom behind AC-004 and EC-014 — a test must be able to fail, and a meta-test that matches nothing must fail loud rather than pass. Directly applicable to the orphan scan. | Before writing the orphan meta-test, and before claiming AC-004. | AC-004 |
| `standards/rust/24-the-blocking-bridge.md` and `standards/rust/52-wasm32-and-target-cfg.md` | The blocking emitter's `block_on` bridge and its constraints; the `cfg` discipline for a module that must compile on `wasm32-unknown-unknown` (no threads, no clock) and the mirror-image harness gates. | Before writing the blocking and wasm emitters and their harnesses. | AC-008 |
| `standards/rust/70-rustdoc-obligations.md` | RS-70-5 — the alternative that lost is named **once**, at the item — which is the density budget the Interaction-quality composition table cites, and the obligation behind every rule doc naming its defect. | While writing every doc comment in this PR. | AC-005, AC-006, AC-007, AC-009 |
| `crates/happenstance-core/src/projection.rs` | The port being driven. `:1-11` is the provisional header — **it stays**, AC-014 disposes of it elsewhere; `:97-99` is the GAT that must be **gone** before this story starts (EC-013's one-minute precondition check). Also the module doc for `Checkpoint`, `Authority` and `CommitError`, whose arms both rules assert on. | First, as the precondition check; then per behaviour while writing rule bodies. | AC-005, AC-006 |
| `RUNBOOK.md:3848-3965` | Phase 6 in full, and `:3882-3892` specifically — the probe, and why a suite without it is *"a checkpoint test a broken store passes"*. The plan-of-record framing this story's rules are distilled from. | Once before starting; re-open `:3882-3892` when writing `commit_is_atomic_with_the_read_model`. | AC-006 |
| `experiments/rustc-ice-gat-foreign-trait/` | The minimised rustc ICE and its bisect script — five independently necessary ingredients, one of which is a borrowing GAT on a foreign trait, still reproducing on 1.97.1. Cite this, not a memory of it, if a reviewer proposes `type Store<'a>`. | If and only if a borrowing associated type is proposed (EC-005). | AC-002 |
| `crates/happenstance-testkit/tests/mutation_coverage.rs` | **Read, do not extend.** `:197-206` is the scope note that still says the projection port has *"neither a suite nor a mutant yet"* — half of which stops being true here and whose replacement is HS-S0008's (AC-A07), not this story's. Knowing where the boundary sits prevents a helpful edit. | Once, to see where the next story picks up; before any impulse to add a projection mutant here. | AC-006 |

## Clarifications resolved during spec

1. **The AC set is exactly the ten the front half enumerated** — AC-001 through
   AC-010, none added, none dropped. The ledger matches one-for-one.

2. **Note 4 is resolved in this spec, on evidence, and it is option 2.** The
   Architecture brief left three ways out with none pre-selected. The Context pack
   recorded the decision — the projection family owns its own enumeration, three
   emitters and entry macro beside its own rules — on the ground that both
   existing *second* families (`model.rs:712-800`, `concurrency.rs:1024-1160`)
   took exactly that shape, which the brief did not cite. This makes Note 10 item
   3 moot rather than risky: option 2 does not touch the event-store emitters, so
   the five existing harnesses cannot break. AC-A01's requirement that the choice
   be "a recorded decision rather than a discovery" is satisfied by that
   paragraph, not by this one.

3. **File names settled here, not deferred**: the projection family lives in
   `crates/happenstance-testkit/src/projection.rs` (one file, `concurrency.rs`'s
   shape) and the harnesses are
   `crates/happenstance-testkit/tests/projection_conformance{,_blocking,_wasm}.rs`.
   Settled so the ledger's `verifying_test` paths are real on landing; a different
   choice is a naming change to record in the report and mirror in the ledger, not
   a change to what is proven.

4. **The projection module is declared unconditionally**, unlike its two nearest
   templates. `concurrency` is `#[cfg(not(target_arch = "wasm32"))]`
   (`lib.rs:172-173`) and `model` adds a `proptest` feature gate
   (`:180-182`) — a projection module gated either way would make AC-008
   unsatisfiable by construction, and the mandatory wasm32 `--tests` check would
   pass while proving nothing.

5. **`ProjectionFixture` declares no capability constants in this PR.** Note 5
   assigns the capability *set* to `_design.md` via `projection-api-design-record`,
   and `projection-capability-skips` lands it with its skip assertions. The two
   baseline rules gate on no capability, so declaring one here for symmetry with
   `Fixture`'s `SECOND_HANDLE`/`REOPEN` would be exactly the decorative shape
   CLAUDE.md's corollary names. Stated as an absence rather than left implicit.

6. **The carried debt is named and bounded: two rules whose mutants arrive one
   story later.** CLAUDE.md's first corollary asks for the wrong implementation to
   be written into the testkit's own `tests/` alongside the rule. Here it is
   discharged as *documentation now, store next story* — each rule's rustdoc names
   the defect it rejects and the story that lands it — because `CheckpointOnlyStore`
   is `projection-mutant-registry`'s deliverable and this story's direct
   dependant. That is a knowingly-carried debt with a named discharger, and the
   bound is the rule count: two, not three.

7. **PS-1's clause/rule mismatch is cited, not settled.** `commit_advances_the_checkpoint`
   is written as §4.11's table specifies and its rustdoc points at
   `.kb/open-questions/ps-1-states-no-progress-obligation.md`. Whether the progress
   obligation joins PS-1 or gets its own clause changes the set of implementations
   the specification admits — an ADR's call, scoped by `ps-clause-pairing-sweep`
   and executed by `unstable-projection-gate-and-clause-disposition`. Nothing
   `[FROZEN]` is line-edited here (AC-A08).

8. **The model and concurrency families' missing orphan meta-tests are a finding,
   not a fix.** `no_orphan_rules` (`registry.rs:410-436`) covers `suite.rs` alone;
   neither second family has a sibling. Project AC-001 requires one for the
   projection family, which this story writes. Extending CF-24's coverage to the
   other two is a deliberate decision someone should take with its own reasoning —
   reported here, out of boundary.

9. **`spec/SPECIFICATION.md` is in the PR boundary for exactly one mechanical
   reason** and the spec says so twice on purpose: `cargo xtask spec-trace --write`
   regenerates §7.1–§7.2 because two rules start existing. Any other hunk in that
   file is out of boundary. The redundancy is deliberate — a wide-looking boundary
   entry is how a clause edit slips in under cover of a generated one.
