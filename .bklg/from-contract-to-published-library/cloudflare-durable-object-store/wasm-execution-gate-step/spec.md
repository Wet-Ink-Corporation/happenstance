---
item: HS-S0048
stage: spec
created: 2026-08-12T13:46:46.451Z
updated: 2026-08-12T13:46:46.451Z
template_sig: 87bbf1d0
rendered_sig: f5a43fb7
---

# Spec — A wasm32 conformance run inside cargo xtask ci, not beside it

## Scope lock

| Layer | Path |
| --- | --- |
| Initiative | [`.bklg/from-contract-to-published-library/initiative.md`](../../initiative.md) — Goal 6 ("the `!Send` / edge case survives contact"), DoD 4 |
| Project | [`.bklg/from-contract-to-published-library/cloudflare-durable-object-store/project.md`](../project.md) — AC-004, DoD 1, risk register entries 1–2 |
| This spec | `.bklg/from-contract-to-published-library/cloudflare-durable-object-store/wasm-execution-gate-step/spec.md` |
| Key briefs | [`../_decomposition.md`](../_decomposition.md) — Architecture §1 (the seam table), §4e (the gate root), §4f (the anti-vacuity root), §5 (a probe-gated step is not a guard); Testing §4 (merge-gate commands and the local cost); Deployment ARCH of record: DEPLOY-AC-01, -02, -05, Notes §2 and §3 |
| Story map row | [`../_storymap.md`](../_storymap.md) — Backbone activity **A**, milestone `wasm-execution-seam`, merge order step 1 |
| Design | [`../_design.md`](../_design.md) — **no user-facing surface**, signed off 2026-08-12. This story renders nothing and re-decides nothing there |
| Roadmap | `RUNBOOK.md:160`, `:4241-4307` (phase 9), and `:4267-4268` — the `vitest-pool-workers`-in-its-own-CI-job hypothesis this story is explicitly measured *against*, not by |

## One-line PR slice

Add a named, non-skippable `Step` to `xtask` that **executes** `crates/happenstance-testkit/tests/memory_conformance_wasm.rs` on `wasm32` inside one `cargo xtask ci`, guarded by a fourth `xtask/src/proof.rs` `Artefact`-shaped row so an emptied target fails instead of passing on `running 0 tests`, and settle the fate of `.github/workflows/ci.yml`'s standalone `wasm-conformance` job.

## Executive summary

Today the gate *compiles* the wasm32 conformance harnesses and runs nothing on that target. `xtask/src/main.rs:219-244` says so in its own comment — the step exists so `__emit_wasm` cannot rot into a macro nobody has compiled — and the only place in the repository where a conformance rule actually executes on `wasm32` is `.github/workflows/ci.yml`'s `wasm-conformance` job (`:204-237`), which is a GitHub Actions job rather than an `xtask` step and runs on `ubuntu-latest` only (`:206`) while the `gate` job matrices across three runners (`:34-39`).

This PR lands the execution seam: a fifth `wasm32` row in `const REQUIRED: &[Step]` (`xtask/src/main.rs:105`) that drives the already-green `memory_conformance_wasm` target through `wasm-bindgen-test-runner`, selected by **name** in `wasm_steps()` (`:784-791`); an anti-vacuity guard in the `xtask/src/proof.rs` shape so that emptying, renaming or `#[ignore]`-ing the executed tests fails the gate instead of passing on `running 0 tests`; a stated disposition for the standalone CI job; and a recorded verdict on whether the runner is available and deterministic on all three matrix runners — or, if it is not, a **blocking finding escalated** rather than AC-004 quietly reduced to a `cargo check`.

The delta is narrow and deliberate. **No adapter code, no fixture, no Cloudflare target.** Its proof subject is a harness that already exists and already passes, so the step cannot be "finished" by moving a goalpost; the Cloudflare conformance target arrives later as a *registered row* in the same seam (`every-rule-under-workerd`, HS-S0054, which this story blocks), not as a second bespoke step.

## Context pack

**Decision 1 — the gate's composition root is `const REQUIRED: &[Step]`, and selection is by name, forever.** `wasm_steps()` used to be `&REQUIRED[3..4]`; an index is silent about what it selects, and inserting a step above it once pointed `cargo xtask wasm` at clippy while still printing green (`xtask/src/main.rs:769-782`). The new step is a `REQUIRED` row and its `name` string is added to `wasm_steps()`; `steps_named` panics on an unresolvable name (`:816-826`) and that panic is the intended failure mode, not something to defend against. Project AC-004 names this defect twice (`RUNBOOK.md:735-737`, `:761-762`).

**Decision 2 — executing is a different claim from type-checking, and the existing steps are not what this replaces.** `#[tokio::test]` type-checks for `wasm32` and then cannot run there — precisely the failure CF-23 is about (`spec/SPECIFICATION.md:7920-7948`). The two existing wasm32 steps stay: the `cargo check --tests` of the testkit harnesses (`xtask/src/main.rs:219-244`) and the `cargo check` of the Cloudflare crate (`:245-264`). This is **new infrastructure**, not a rewiring (Architecture §4e).

**Decision 3 — "silently does not run" must be a gate failure, which forecloses one of the three honest shapes.** `xtask` skips a step whose probe fails and prints `skipped: …` (`xtask/src/main.rs:873-878`), and `:192-202` states the rule this story must not break: *a constraint whose only check is skippable is unguarded on every machine that lacks one tool.* The admissible shapes are therefore **(i)** a mandatory `REQUIRED` step with `probe: None`, the gate failing outright where the runner is absent, or **(ii)** a probe-gated step **paired with a mandatory, non-skippable assertion** that the target and its rule enumeration exist and are non-empty. A bare probe-gated step with no compensating mandatory check is **not admissible** — it is the exact configuration AC-004 forbids. **(iii)**, if neither prices acceptably, is a documented blocking finding escalated to the human, never a degradation absorbed here (`project.md`, Risks 1; `_intake-brief.md`, Open Questions). Choosing between (i) and (ii) and *stating why* is this story's work; the choice is recorded in the code's own comment and in the story's implementation report as material for ADR-0023, which is authored by `adr-0023-and-atom-resolutions` (HS-S0057) and **not** written here as a side effect.

**Decision 4 — `cargo test` exits 0 on `running 0 tests`, so naming the target is checking the filename.** That is `xtask/src/proof.rs`'s whole argument (`:9-23`), and `Artefact { package, target, tests }` (`:58-70`) is the machinery: it asserts the named tests out of `cargo test -- --list` *before* running them (`:298-325`). Register the executed wasm target as a fourth guard in that shape. Three things about the existing machinery are load-bearing and will not transfer unchanged:

- `cargo_args` returns a fixed `[&'static str; 7]` (`proof.rs:164-174`) with no room for `--target wasm32-unknown-unknown` and no way to set the runner environment variable. Widening it is expected work, not scope drift.
- `--all-features` is on those args for a *fingerprint-sharing* reason (`proof.rs:151-163`), and it is **wrong on `wasm32`**: `happenstance-testkit`'s `proptest` feature maps to a dependency declared only under `[target.'cfg(not(target_arch = "wasm32"))'.dependencies]` (`crates/happenstance-testkit/Cargo.toml`), which is why the existing wasm32 steps pass no feature flags at all. The wasm entry must not inherit that flag.
- Whether `wasm-bindgen-test-runner` honours `-- --list` at all is **unverified**. If it does not, the anti-vacuity assertion must be derived from the executed run's own reported count against the rule enumeration — never from exit status alone. Which mechanism is used is an outcome to record; *that* an emptied target fails is not negotiable (DoD 1; `project.md` AC-004).

**Decision 5 — reuse the demonstrated runner shape rather than invent one.** `.github/workflows/ci.yml:213-237` already drives `#[wasm_bindgen_test]` tests for real: `wasm-bindgen-cli` installed at the version resolved out of `Cargo.lock` (a mismatched schema is refused by the runner, and rightly), with `CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER` set as the per-target runner variable. `Step.env` exists for exactly this kind of per-step variable — it was added for the rustdoc steps because a process-wide variable leaks (`xtask/src/main.rs:80-88`). Wherever the version-from-`Cargo.lock` resolution ends up living, it must survive: hard-coding the version means a `cargo update` leaves a runner that no longer matches, failing as a confusing runtime error instead of a version bump.

**Decision 6 — `--nocapture` is not noise on this step.** A declined capability's `SKIP <rule>: <reason>` line reaches `console_log!` under `__emit_wasm`, and `println!` is a silent discard on `wasm32-unknown-unknown` (`CHANGELOG.md:1094-1102`, measured under `wasm-bindgen-test-runner --nocapture` before and after). The reason the gate's `tests` step carries `--show-output` (`xtask/src/main.rs:131-142`) applies here in the target's own idiom: without it, the trade an adapter author made by declining a capability leaves no record in the gate's output, and the project's AC-003 — owned downstream — becomes unobservable through the seam this story builds.

**Decision 7 — build the seam for its consumer, and only its consumer.** `every-rule-under-workerd` (HS-S0054) must be able to add the Cloudflare conformance target by *registering a row*, the way `ARTEFACTS` takes a row today, not by writing a second execution step with its own runner wiring. An implementation that hard-codes `-p happenstance-testkit --test memory_conformance_wasm` into `Step.args` satisfies every other criterion here and forces its consumer to duplicate the whole design; that is the named wrong implementation this story's AC-006 rejects.

**Decision 8 — the local cost is real and lands on this project immediately.** `run_fast` runs `REQUIRED` unmodified (`xtask/src/main.rs:853-860`), and `cargo xtask ci --fast` is the bar `.redkiln/config.yaml`'s `verify.integration_scoped` holds this non-terminal project to. If the step is mandatory, every subsequent story in this project needs a working runner **locally**, on Windows first-class (`.github/workflows/ci.yml:34-39` and its comment). Testing §4 flags this as a workflow consequence to decide deliberately rather than meet as a surprise.

**Decision 9 — the standalone CI job may not be left unexplained.** DEPLOY-AC-05: `.github/workflows/ci.yml`'s `wasm-conformance` job is either retired in favour of the in-gate step, or its continuing purpose is stated in its own comment. Two weaker-and-stronger wasm32 execution paths coexisting with no stated division of labour is the outcome that brief forecloses. Note that the job's comment currently reads *"No Cloudflare and no `workerd`. Those arrive at phase 9"* (`:201-203`) — that sentence is about to become stale whichever way the disposition falls.

**Decision 10 — the persona slice.** Two of the three people the story map names observe this directly (`_storymap.md`, Backbone): the **gate reader**, who must be able to see from one `cargo xtask ci` that rules *ran* on the target the two-flavour port design exists for; and the **adapter author**, who must get a pass or a named failure per rule from one entry point on the target their adapter targets. Nothing here is observed by the library consumer, and this story ships no public API.

**What this story does not decide.** The Cloudflare fixture, the Durable Object host, whether `workerd`/`vitest-pool-workers` proper is ever needed above `wasm-bindgen-test-runner`, the `SqlStorage` mapping, and ADR-0023's authorship. This story produces the evidence and the recorded choice; the atom is minted once, through the ingest path, by HS-S0057.

## Integration contract

- **Archetype**: `foundation` — real in-tree substrate (a `Step` row, a `wasm_steps()` name, an anti-vacuity guard), consumed inside this project by `every-rule-under-workerd`. Not a double and not a fixme: it is demonstrated the day it merges against a target that already exists and already passes.
- **Slice / milestone**: `wasm-execution-seam`. **Slice-mates: none** — this story is the milestone's sole member (`_storymap.md`, Slices), and it is merge order step 1 (`_storymap.md`, Merge order). `depends_on: []`. It **blocks** HS-S0054 (`story.md` frontmatter).
- **Mount point**: `xtask/src/main.rs` — `const REQUIRED: &[Step]` at `:105`, with the new step's `name` added to `wasm_steps()` at `:784-791` (resolved through `steps_named`, `:816-826`). This is the gate's composition root: `run_ci` (`:828-833`) and `run_fast` (`:853-860`) both iterate it, so a row here is mounted in the one command CI runs and contributors run. A step constructed but absent from `REQUIRED`, or present in `REQUIRED` but missing from `wasm_steps()`, is unmounted.
- **Wires into**: `xtask/src/proof.rs` (`Artefact`, `ARTEFACTS`, `cargo_args`, `list` — the anti-vacuity contract, `:58-174`, `:298-325`); `crates/happenstance-testkit/tests/memory_conformance_wasm.rs` (the executed target and its module doc, which currently claims the gate only type-checks it, `:15-17`); `happenstance_testkit::event_store_conformance!` with `emit = happenstance_testkit::__emit_wasm` and the single `for_each_event_store_rule!` enumeration (`crates/happenstance-testkit/src/lib.rs:55-89`); `happenstance-testkit`'s `[target.'cfg(target_arch = "wasm32")'.dev-dependencies]` on `wasm-bindgen-test`; `.github/workflows/ci.yml` (`gate` matrix `:34-39`, `wasm-conformance` `:204-237`); `.redkiln/config.yaml`'s `verify.affected_gate` and `verify.integration_scoped`.
- **Renders surfaces**: **none.** `_design.md` records **N/A — no user-facing surface** for this project, approved 2026-08-12, and its `## Items` block is empty. This story claims no `path` id because there are none to claim, and it adds **no public API**: every item it touches is `pub(crate)` inside `xtask` or a test target.
- **Conformance rule(s)**: **none added, and none is owed.** This story adds no rule to `suite.rs`; its whole subject is *executing* the rules that already exist, from the one enumeration (`crates/happenstance-testkit/src/lib.rs:84-89`). A `wasm`-only rule list anywhere in the tree would fail project AC-002 by construction, so the correct outcome here is that `registry::no_orphan_rules` and `for_each_event_store_rule!` are untouched. The rules this step *observes* are the full event-store family emitted by `memory_conformance_wasm`; because no rule is added, no CF-29 changelog entry is owed for a rule — a `CHANGELOG.md` entry for the gate step itself is house practice and is separate.
- **Clause(s)**: **CF-23** (`spec/SPECIFICATION.md:7920-7948`) is the clause this story strengthens the observation of — its `Rule:` line reads "the wasm32 build step of `cargo xtask ci`, extended to compile a `wasm-bindgen-test` harness over the same rule set", and after this story the gate does more than compile it. CF-23 is `[FROZEN]`: **its normative sentences and its marker are not edited here.** If the `Rule:` line is updated to name the executing step, `cargo xtask spec-trace` must stay green across the change and the edit must be confined to that line; anything that would alter what CF-23 *requires* is a new ADR and a re-plan, not a line edit (`CLAUDE.md`, Open questions). Cases E2E-52 and E2E-30 are cited by the clause and are unchanged.
- **Advances DoD scenario**: initiative **DoD 4** — *"The constrained-runtime store passes the suite on its own target. Every rule is green under the edge runtime on `wasm32`, executed in the gate rather than asserted in prose"* (`initiative.md`, Definition of done). This story lands the *executed-in-the-gate* half of that sentence and nothing else; the *constrained-runtime store* half is HS-S0054's. It also discharges the "step exists, is named not indexed, and is non-vacuous" half of project AC-004 (`_storymap.md`, Coverage).

## PR boundary

**In this PR**

- One new `Step` in `xtask/src/main.rs`'s `REQUIRED`, its name in `wasm_steps()`, its argument for mandatory-versus-probe-plus-compensator written as a comment in the file (the house habit: the gate carries its own argument), and `print_help`'s `wasm` description (`:735-737`) corrected to say what now executes.
- The anti-vacuity guard: a fourth registered entry in the `xtask/src/proof.rs` shape, plus whatever widening of `Artefact`/`cargo_args` a `--target`-bearing, feature-flag-free, env-carrying invocation needs — or, if the runner cannot enumerate its tests, an equivalent mandatory assertion derived from the run's own output, with the reason recorded in the file.
- The executed target's module doc (`crates/happenstance-testkit/tests/memory_conformance_wasm.rs:15-17`), whose claim that `cargo xtask wasm` only type-checks it stops being true.
- `.github/workflows/ci.yml`: the `wasm-conformance` job retired in favour of the in-gate step (with the `wasm-bindgen-cli`-version-from-`Cargo.lock` resolution and any toolchain install preserved wherever the work now lives), or kept with its continuing purpose stated in its own comment. Whichever way, the stale *"Those arrive at phase 9"* sentence is corrected.
- A `CHANGELOG.md` entry under `[Unreleased]` naming what the gate now does that it did not.
- Optionally, CF-23's `Rule:` line in `spec/SPECIFICATION.md`, under the constraint stated in the Integration contract, with `cargo xtask spec-trace` green.
- This story's own backlog folder: `_ledger.md`, the implementation report, and the recorded verdict material for ADR-0023.

**Explicitly not in this PR**

- Any change under `crates/happenstance-cloudflare/` — no `worker` dependency, no fixture, no `todo!()` removed. That is `worker-binding-layer` and its milestone.
- The Cloudflare conformance target itself, and any claim that *every rule runs under a Durable Object runtime* — `every-rule-under-workerd` (HS-S0054).
- Any new or edited conformance rule, any `#[cfg]` over a rule, and any second rule enumeration.
- Any `.kb/` write. ADR-0023, CF-40 and WF-11 are `adr-0023-and-atom-resolutions`'s, minted through `/redkiln:kb-ingest`, never hand-written (`_storymap.md`, *Why these milestones and not others*).
- `publish = false`, licence files, README, name reservation — `publish-ready-crate`.
- Amending any `[FROZEN]` clause's normative text or marker.

**Merge DoD.** One `cargo xtask ci` on a clean checkout executes the wasm32 conformance rules and is green; emptying the executed target or removing the step fails the gate with a legible message; `cargo xtask affected --base main` and `cargo xtask ci --fast` are green; and the mandatory-versus-probe choice is recorded in-tree with its reason.

```
xtask/src/**
crates/happenstance-testkit/tests/memory_conformance_wasm.rs
.github/workflows/ci.yml
spec/SPECIFICATION.md
CHANGELOG.md
.bklg/from-contract-to-published-library/cloudflare-durable-object-store/wasm-execution-gate-step/**
```

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| One `cargo xtask ci` **executes** wasm32 conformance rules | A new `Step` drives the `memory_conformance_wasm` target on `wasm32-unknown-unknown` through `wasm-bindgen-test-runner`, set via `Step.env`'s `CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER`. Rules run; they are not type-checked. The run is part of `run_ci` and `run_fast`, not a separate command | `xtask/src/main.rs:105`, `:80-88`, `:828-860`; `.github/workflows/ci.yml:234-237`; `crates/happenstance-testkit/tests/memory_conformance_wasm.rs:19-27` |
| Selection by name, never by index | The step is a `REQUIRED` row; `wasm_steps()` gains its `name` string; `steps_named` resolves it or panics. `cargo xtask wasm` therefore runs it too, and `print_help`'s description is updated to match | `xtask/src/main.rs:769-791`, `:816-826`, `:735-737` |
| An emptied or renamed target fails the gate | The executed target is registered in the `xtask/src/proof.rs` shape: named tests asserted out of `--list` before the run, or — if the runner cannot enumerate — an equivalent mandatory assertion over the run's own reported count. A truncated file, a renamed test or an `#[ignore]` fails with a legible message instead of `running 0 tests` | `xtask/src/proof.rs:9-23`, `:58-70`, `:133-149`, `:191-240`, `:298-325`; `xtask/src/main.rs:156-191` |
| The anti-vacuity invocation is target-correct | `--target wasm32-unknown-unknown`, **no** `--all-features` (the testkit's `proptest` feature maps to a dependency that does not exist on this target), and the runner variable carried into both the enumeration and the run so the two agree | `xtask/src/proof.rs:151-174`; `crates/happenstance-testkit/Cargo.toml` (`[features]`, the two target-scoped dependency blocks); `xtask/src/main.rs:219-244` |
| Silent non-execution is a failure, not a skip | Either `probe: None` (mandatory, the gate fails where the runner is absent) or a probe **plus** a mandatory non-skippable assertion. The chosen shape and its reason are written into the step's own comment and into the story's report as ADR-0023 material — the ADR itself is not authored here | `xtask/src/main.rs:89-103`, `:192-202`, `:873-878`; `../_decomposition.md` Architecture §5, Deployment DEPLOY-AC-02 and Notes §3 |
| Declined-capability reasons stay legible | The step passes the runner's `--nocapture` equivalent, so `__emit_wasm`'s `console_log!` `SKIP <rule>: <reason>` lines reach the gate's output rather than a silent discard | `CHANGELOG.md:1094-1102`; `xtask/src/main.rs:131-142`; `crates/happenstance-testkit/src/lib.rs:46-50` |
| The seam takes a second target by registration | The executed targets are a declared list, not a hard-coded pair of `-p`/`--test` arguments in `Step.args`, so HS-S0054 adds the Cloudflare conformance target as a row. The `ARTEFACTS` array is the in-tree precedent for that shape | `xtask/src/proof.rs:133-149`; `_storymap.md` (`every-rule-under-workerd`'s row, "register it in the gate step from `wasm-execution-seam`") |
| The runner's toolchain story is answered, not assumed | Whether `wasm-bindgen-test-runner` is available and deterministic on `ubuntu-latest`, `windows-latest` and `macos-latest`, and what a contributor pays locally, is answered with evidence and recorded. A platform restriction, if one is needed, is documented with its reason — never silent. If no runner can exist inside `cargo xtask ci` at acceptable cost, the outcome is an escalated blocking finding, never a downgrade of AC-004 to a `cargo check` | `.github/workflows/ci.yml:34-39`, `:204-237`; `../_decomposition.md` Deployment Notes §2.2 and §3; `project.md` Risks 1–2; `_intake-brief.md` Open Questions |
| The standalone CI job has a stated disposition | `wasm-conformance` is retired in favour of the in-gate step, or its continuing purpose is stated in its own comment. Its stale phase-9 sentence is corrected either way | `.github/workflows/ci.yml:189-237`; `../_decomposition.md` DEPLOY-AC-05 and Deployment Notes §2 |
| The rule set stays single-sourced | No rule added, none removed, none `#[cfg]`-ed out, no wasm-only subset list introduced anywhere. `registry::no_orphan_rules` and `for_each_event_store_rule!` are untouched by this diff | `crates/happenstance-testkit/src/lib.rs:84-89`; `project.md` DR-3 |
| Existing gate steps and detectors stay green | The two existing wasm32 `cargo check` steps remain; `cargo xtask spec-trace`, the five file-reading lints, and `cargo xtask affected --base main` stay green; `cargo xtask ci --fast` (this project's integration bar) passes end to end | `xtask/src/main.rs:219-264`, `:303-374`, `:835-860`; `.redkiln/config.yaml` `verify:` |

## Data and migrations

**N/A.** This story touches no persistent data, no schema and no wire format. It edits build-and-gate tooling (`xtask`), one CI workflow, one test target's module documentation and two documents; the store it causes to run is `MemoryEventStore` behind `MemoryFixture`, which is in-process and constructed fresh per fixture instance (`crates/happenstance-testkit/tests/memory_conformance_wasm.rs:21-27`). There is no prior state to migrate, no backfill, and nothing deployed to roll back — the deployment brief's Notes §1 states the same conclusion for the project as a whole, and it holds a fortiori for a story that changes no adapter code.

## Acceptance criteria

Seven criteria, each written as a goal one of the three people in `_storymap.md`'s
Backbone is trying to reach, crossing the whole stack from the command they type to the
evidence they get back. The personas are this initiative's own (`initiative.md`, **Who
this is for**, and its journey *Learn when you are finished*, carried from
`_discovery/distillation/personas-and-journeys.md`); the **gate reader** and the
**adapter author** are the two who observe this story, and the library consumer observes
nothing here.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **GIVEN** a gate reader on a clean checkout who has been told this workspace's `!Send` port design is real, **WHEN** they run one `cargo xtask ci` (and one `cargo xtask ci --fast`) and read the output top to bottom, **THEN** a named wasm32 step reports conformance rules that actually **executed** on `wasm32-unknown-unknown` under `wasm-bindgen-test-runner` — a per-rule pass or a named failure, not a compile line — and the gate is green on every runner the `gate` job matrices over | `cargo xtask ci` and `cargo xtask ci --fast` end to end over the new `REQUIRED` row (`xtask/src/main.rs:105`, `:828-833`, `:853-860`); the CI `gate` job on ubuntu, windows and macos (`.github/workflows/ci.yml:34-39`); the executed target is `crates/happenstance-testkit/tests/memory_conformance_wasm.rs` |
| AC-002 | **GIVEN** an adapter author who will later insert a step above this one, **WHEN** they run `cargo xtask wasm` or read `cargo xtask --help`, **THEN** the execution step is selected **by name and not by index** — it is a `REQUIRED` row whose `name` string appears in `wasm_steps()`, an unresolvable name panics in `steps_named`, `cargo xtask wasm` runs it too, and `print_help`'s `wasm` description no longer claims the wasm32 tasks merely check that things build | new `#[cfg(test)]` unit tests in `xtask/src/main.rs` (precedent: `xtask/src/package.rs:408`, `xtask/src/affected.rs:596`) asserting `wasm_steps()` resolves five steps and contains the execution step's exact `name`, run by `cargo test -p xtask`; plus `cargo xtask wasm` executing the rules |
| AC-003 | **GIVEN** a gate reader who has been burned by a step that a deletion fails and an emptying passes (`xtask/src/proof.rs:9-23`), **WHEN** the executed target is truncated to its `#![cfg(…)]` attribute, or a rule test is renamed or `#[ignore]`d, **THEN** the gate **fails** with a message naming the missing test rather than exiting 0 on `running 0 tests` — because the executed wasm32 target is registered in the `xtask/src/proof.rs` shape and its named rules are asserted out of `--list` before the run, or, where the runner cannot enumerate, an equivalent mandatory assertion is made over the run's own reported count against the rule enumeration | `cargo xtask proof-artefact` (`xtask/src/main.rs:681`) covering the new fourth entry; a unit test in `xtask/src/proof.rs`'s `#[cfg(test)]` module asserting the wasm entry's expectation list is non-empty and that each listed name appears in the enumeration source (file-read precedent: `proof.rs:76-82`); the emptied-target negative control performed and its failure output pasted into the implementation report |
| AC-004 | **GIVEN** an adapter author on a machine without `wasm-bindgen-cli` installed, **WHEN** they run `cargo xtask ci --fast`, **THEN** what happens is a stated decision they can read in the file, and in no configuration does the wasm32 execution silently not run while the gate still prints green: either the step is `probe: None` and the gate fails outright, or it is probe-gated **and** a mandatory non-skippable assertion over the target and its rule enumeration runs regardless — with the choice, its cost and its rejected alternative written into the step's own comment as ADR-0023 material | a unit test in `xtask/src/main.rs` asserting the execution step's `probe` is `None`, or — under shape (ii) — that a `probe: None` compensator row exists in `REQUIRED`; a manual `cargo xtask ci --fast` with the runner removed from `PATH`, its output recorded in the implementation report; code review of the in-file comment against `xtask/src/main.rs:192-202` and `:873-878` |
| AC-005 | **GIVEN** a gate reader asking *what could this runtime not do, and why*, **WHEN** they read the wasm32 step's output in the same terminal scroll as the rest of the gate, **THEN** every rule the run touched is attributable to the one `for_each_event_store_rule!` enumeration with no wasm-only subset list anywhere in the tree, and any declined capability's `SKIP <rule>: <reason>` line is **visible** rather than swallowed — the step passes the runner's `--nocapture` equivalent, because `println!` is a silent discard on this target | the executed step's captured output in `cargo xtask ci`, inspected and pasted into the implementation report; `registry::no_orphan_rules` still green under `cargo test --workspace --all-features` (`crates/happenstance-testkit/src/registry.rs:413-424`); a diff review confirming no rule added, removed or `#[cfg]`-ed and no second enumeration introduced; evidence for the discard: `CHANGELOG.md:1094-1102` |
| AC-006 | **GIVEN** the adapter author of `every-rule-under-workerd` (HS-S0054), **WHEN** they come to run the Cloudflare conformance target under the same runner, **THEN** they add it by **registering a row** in a declared list of executed targets — the way `ARTEFACTS` takes a row — and write no second execution step, no second runner wiring and no second `Step.env`; an implementation that hard-codes `-p happenstance-testkit --test memory_conformance_wasm` into `Step.args` is the named wrong implementation this criterion rejects | a unit test in `xtask` walking the declared executed-target list and asserting it is a list of rows each carrying its own package and target rather than fixed arguments, run by `cargo test -p xtask`; code review against the wrong implementation named above (`xtask/src/proof.rs:133-149` is the in-tree precedent); the entry point stated in the implementation report for HS-S0054 |
| AC-007 | **GIVEN** a gate reader looking at CI after this merges, **WHEN** they compare `.github/workflows/ci.yml` against the in-gate step, **THEN** there are not two unexplained wasm32 execution paths: the standalone `wasm-conformance` job is either retired in favour of the in-gate step — with the `wasm-bindgen-cli`-version-resolved-from-`Cargo.lock` behaviour preserved wherever the work now lives, never hard-coded — or kept with its continuing purpose stated in its own comment, its stale *Those arrive at phase 9* sentence corrected either way; and the recorded verdict says whether the runner is available and deterministic on all three matrix runners, any platform restriction carrying its reason, with an escalated blocking finding rather than a downgrade of AC-004 to a `cargo check` if it is not | review of the `.github/workflows/ci.yml` diff (`:189-237`) against `../_decomposition.md` DEPLOY-AC-05 and Deployment Notes §2; the three-runner `gate` job green, or the documented restriction with its reason in the file; `cargo xtask spec-trace` green; the verdict recorded in the implementation report as ADR-0023 material |

**Coverage of the traced project AC.** Project **AC-004** (`project.md:209-213`) is
discharged across AC-001 (executed inside one `cargo xtask ci`), AC-002 (registered by
name, not by index — the defect `RUNBOOK.md:735-737` and `:761-762` name twice), and
AC-003 with AC-004 (a configuration in which the step silently does not run fails the gate
rather than skipping it). That is exactly the half `_storymap.md`'s Coverage table assigns
to this story; *the Cloudflare target is what it runs* stays with `every-rule-under-workerd`.
AC-005 through AC-007 discharge the deployment brief's DEPLOY-AC-01, -02 and -05 and the
seam's obligation to its consumer.

## Interaction quality

**COMPOSITION family — N/A, declared rather than skipped.** The project's signed-off
`../_design.md` records **N/A — no user-facing surface**, approved by the repository owner
on 2026-08-12, with an empty `## Items` block and `N/A` under Placement, States and
Anti-patterns (`../_design.md:10-64`, `:106-111`). There is no presentation to compose, no
density budget, no transience policy and no named design anti-pattern binding on this
story, and `design.capture` is a declared skip for this repository (`CLAUDE.md`, *Where the
work lives*). Nothing is waived here by omission — it is waived by a signed-off
determination this story is forbidden to re-decide.

**STATE family — it applies, in the medium this story actually has.** The gate's own
terminal output is the only thing a human perceives, and the invariants translate exactly.
Each is carried by an `AC-###` **row in the table above** so `redkiln verify` extracts and
gates it; this section only records which row carries which invariant.

- **In-place, not a context jump** — the answer arrives in the run the reader already
  started, not by opening a second tool (a GitHub Actions job) on a machine they do not
  have. Carried by **AC-001** and **AC-007**; verified by the end-to-end gate run and the
  workflow diff review.
- **Non-occlusion** — a declined capability's reason and a failing rule's name must not be
  hidden behind a capture layer; on this target the default *is* occlusion, because
  `println!` is a silent discard. Carried by **AC-005**; verified by inspecting the step's
  captured output for `SKIP <rule>: <reason>` lines.
- **Reversibility, in the sense a build tool has one** — a failure must be reproducible
  from the message alone. `run_steps` prints the program and arguments it ran
  (`xtask/src/main.rs:862-906`), so the reader can re-run exactly it. Carried by **AC-003**
  (a legible named-test failure rather than a silent pass) and **AC-004** (a readable,
  stated reason for the chosen shape); verified by the two recorded negative controls.
- **Preserved position, no silent re-ordering** — inserting a row into `REQUIRED` must not
  re-point any other selector at the wrong step, the index-selection defect this workspace
  has already paid for once (`xtask/src/main.rs:769-782`). Carried by **AC-002**; verified
  by the `wasm_steps()` unit test and by `cargo xtask wasm`.
- **Reachability without the full ceremony** — the capability must be reachable from a
  small command as well as from the whole gate, the reason `lint_steps()` exists
  (`xtask/src/main.rs:793-808`). Carried by **AC-002** (`cargo xtask wasm`) and **AC-003**
  (`cargo xtask proof-artefact`).
- **Keyboard reachability, focus, scroll and selection** — N/A. There is no focusable
  surface; stated rather than silently dropped.

## Error conditions

| id | condition | required behaviour |
| --- | --- | --- |
| EC-001 | `wasm-bindgen-test-runner`, or the node host it drives, is absent on the machine running the gate | Under shape (i) the step fails with a message naming the missing tool and how to install it — never a `skipped:` line (`xtask/src/main.rs:873-878`). Under shape (ii) the step may skip **only if** the mandatory anti-vacuity assertion still runs and still fails an emptied target. A bare probe-gated step with no compensator is precisely the configuration project AC-004 forbids (`project.md:209-213`) |
| EC-002 | The installed `wasm-bindgen-cli` does not match the `wasm-bindgen` version resolved in `Cargo.lock` | The runner refuses the module with a schema mismatch, and rightly. The gate surfaces that as a legible failure, and the version is **derived** from `Cargo.lock` — the shape `.github/workflows/ci.yml:213-237` already demonstrates — never hard-coded, so a `cargo update` produces a version bump rather than a confusing runtime error |
| EC-003 | `wasm-bindgen-test-runner` does not honour `-- --list`, so the `Artefact` enumeration cannot be taken before the run | Fall back to a mandatory assertion derived from the executed run's own reported outcome count against `for_each_event_store_rule!`'s enumeration, and record in `xtask/src/proof.rs` why the shape differs. Exit status alone is **not** an admissible substitute — that is the exact failure `proof.rs:9-23` exists to prevent |
| EC-004 | The wasm32 invocation inherits `--all-features` from `cargo_args` (`xtask/src/proof.rs:164-174`) | Compilation fails: `happenstance-testkit`'s `proptest` feature maps to a dependency declared only under `[target.'cfg(not(target_arch = "wasm32"))'.dependencies]` (`crates/happenstance-testkit/Cargo.toml`). The wasm entry passes no feature flags, matching the two existing wasm32 steps (`xtask/src/main.rs:231-243`) |
| EC-005 | The step's `name` is added to `wasm_steps()` but its row is not added to `REQUIRED`, or one side is renamed | `steps_named` panics with `REQUIRED must contain the … step` (`xtask/src/main.rs:816-826`). That is the **intended** failure mode and must not be softened into a fallible lookup; a silent `None` reproduces the index defect one level further in |
| EC-006 | The runner is available on `ubuntu-latest` but non-deterministic or unavailable on `windows-latest` or `macos-latest` | A documented platform restriction with its stated reason in the step's own comment, plus the recorded verdict (AC-007). A silent narrowing to one runner — the shape the standalone job already has at `.github/workflows/ci.yml:206` — is not admissible, because the `gate` job matrices over three |
| EC-007 | No runner can be made to work inside `cargo xtask ci` at acceptable cost | **Escalate a blocking finding to the human and stop.** Reducing AC-004 to a `cargo check` and calling the story done is the one outcome the deployment brief forecloses (`../_decomposition.md` Deployment Notes §1 *Rollback posture*; `project.md` Risks 1; `_intake-brief.md` Open Questions) |
| EC-008 | The executed run reports a rule failure on `wasm32` that passes on the host | Do **not** `#[cfg]` the rule out and do not introduce a wasm-only subset — that fails project AC-002 by construction (`../_decomposition.md` Architecture Notes §7.4). Either the rule or the fixture is wrong; report it as a finding naming the rule and the divergence |

## Non-functional

| id | requirement | why, and where it comes from |
| --- | --- | --- |
| NF-001 | The step must not inflate the gate's wall clock beyond the cost of building and running one already-existing test target for `wasm32`. Share fingerprints with the existing wasm32 steps by matching *their* flags — the `--all-features` reasoning at `xtask/src/proof.rs:151-163` is about reuse, and the wasm32 equivalent is emphatically not to copy the host flags | `cargo xtask ci --fast` runs at every story seam in this project (`.redkiln/config.yaml`, `verify.integration_scoped`); a step that doubles the loop's cost is paid eleven more times |
| NF-002 | Determinism: the same verdict on repeat invocations and across the three `gate` runners, or a documented restriction carrying its reason | `.github/workflows/ci.yml:34-39`; `../_decomposition.md` Deployment Notes §2.2 |
| NF-003 | Offline and reproducible: the invocation keeps `--locked`, like every other `REQUIRED` step | `xtask/src/main.rs:180-188`, `:205-215`, `:233-241` |
| NF-004 | No new dependency lands in any **publishable** crate. Anything added is `xtask`-local, a dev-dependency, or a target-scoped dev-dependency — the target-scoped-not-feature-scoped reasoning `crates/happenstance-testkit/src/lib.rs:105-113` already documents | `../_decomposition.md` Deployment Notes §1 (*Feature flags / config gating*); the `cargo package --list` assertion over the three publishable crates (`xtask/src/main.rs:519`) |
| NF-005 | The local toolchain cost of the chosen shape is stated in the implementation report in one sentence a contributor can act on, rather than discovered mid-project | `../_decomposition.md` Testing Notes §4 flags it explicitly as a workflow consequence to decide deliberately |
| NF-006 | No `[FROZEN]` clause's normative text or maturity marker is altered. If CF-23's `Rule:` line is updated to name the executing step, the edit is confined to that line and `cargo xtask spec-trace` stays green | `CLAUDE.md`, *Open questions*; `spec/SPECIFICATION.md:7920-7948` |

## Implementation notes (non-prescriptive)

One workable route, not a mandate — the latitude the architecture brief reserves for the
implementer (`../_decomposition.md` Architecture Notes §6) applies to the harness question
as much as to the SQL one.

- **Answer the toolchain question before writing the step.** Install `wasm-bindgen-cli` at
  the `Cargo.lock`-resolved version and run
  `cargo test -p happenstance-testkit --test memory_conformance_wasm --target wasm32-unknown-unknown`
  by hand with `CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER` set — on Windows first, because
  that is the runner the standalone job never covered. Whether `-- --list` works there is
  the single fact that decides between the `Artefact` shape and EC-003's fallback, and it is
  cheap to learn. Everything else here is downstream of that measurement.
- **Then choose the shape, and write the argument down first.** Options (i) and (ii) are
  both admissible; the trade is local cost against machinery (`../_decomposition.md`
  Deployment Notes §3). Write the comment before the code — the house habit is that the gate
  carries its own argument, and `xtask/src/main.rs:192-202` and `:169-178` are the two
  nearest examples of it.
- **On widening `proof.rs`.** `cargo_args` returning a fixed `[&'static str; 7]` is the
  narrowest thing in the way. Building a `Vec<&'static str>` from optional fields on
  `Artefact` — a target triple, an all-features flag, an env slice — keeps the
  borrowed-not-owned property the doc comment argues for while making room for a wasm row;
  a separate shape beside `ARTEFACTS` is equally honest if it reads better. What matters is
  AC-006: whichever lands, adding the Cloudflare target later must be a row, not a step.
- **On the env entry.** `CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER` is per-target by
  construction, which is why `Step.env` is its home rather than a process-wide variable —
  the reason `env` was added for the rustdoc steps in the first place
  (`xtask/src/main.rs:80-88`).
- **On the standalone job.** The deployment brief's leading candidate is to retire it once
  the in-gate step subsumes what it proved, and to add the Cloudflare execution later as a
  new row rather than a rewrite (`../_decomposition.md` Deployment Notes §2). A candidate,
  not an instruction: if it is kept, its comment must say what it proves that the gate does
  not.
- **Keep the diff inside the PR boundary.** No file under `crates/happenstance-cloudflare/`,
  and no `.kb/` write of any kind.

## Tests and CI (merge gate)

Tiers as the project's testing brief defines them (`../_decomposition.md` Testing Notes §1),
with the merge-gate commands from its Notes §4 and `.redkiln/config.yaml`'s `verify:` wiring.

| tier | command / path | proves |
| --- | --- | --- |
| static | `cargo fmt --all --check`; `cargo clippy --workspace --all-targets --all-features -- -D warnings` | The `xtask` and workflow edits meet the house bar; `-D warnings` is what makes an unused field or a stray `allow` fail rather than accumulate |
| unit (`xtask`) | `cargo test -p xtask`, over new `#[cfg(test)]` modules in `xtask/src/main.rs` and `xtask/src/proof.rs` (precedent: `xtask/src/package.rs:408`, `xtask/src/affected.rs:596`) | AC-002 (`wasm_steps()` resolves five names and contains the execution step), AC-004 (the step's `probe`, or its compensator's, is `None`), AC-006 (the executed-target list is a list of rows, not fixed arguments) |
| static (file read) | the wasm entry's expectation list cross-checked against `crates/happenstance-testkit/tests/memory_conformance_wasm.rs` and `crates/happenstance-testkit/src/registry.rs:94-140`, in the `proof.rs` file-read style (`proof.rs:76-82`) | AC-003's other half: the names the gate asserts are the names the one enumeration actually emits, so a rename fails here instead of drifting |
| integration (the deliverable) | `cargo xtask proof-artefact` (`xtask/src/main.rs:681`), then the new step inside `cargo xtask ci` | AC-001 and AC-003: rules enumerated, then **executed** on `wasm32-unknown-unknown`, with an emptied or renamed target failing before the run starts |
| integration (targeted, manual, recorded) | the emptied-target negative control and the runner-removed-from-`PATH` run, output pasted into the implementation report | AC-003 and AC-004 as *observed* failures rather than asserted ones — `CLAUDE.md`'s corollary that a rule no adapter can fail is decorative, applied to a gate step |
| story grain (merge gate) | `cargo xtask affected --base main` (`.redkiln/config.yaml`, `verify.affected_gate`) | The five file-reading lints and `spec-trace` run unconditionally — which is why a story whose deliverable is an `xtask`, workflow and specification edit is still gated |
| integration grain (merge gate) | `cargo xtask ci --fast` (`.redkiln/config.yaml`, `verify.integration_scoped`) | `REQUIRED` including the new row, at this non-terminal project's bar. NF-001's cost lands here |
| reachability (static) | `cargo xtask lints && cargo xtask spec-trace` (`.redkiln/config.yaml`, `verify.reachability_static`) | NF-006 and AC-007: CF-23's citations still resolve and its marker has not rotted into decoration, whatever its `Rule:` line now says |
| standing detectors | `cargo test --workspace --all-features` — `registry::no_orphan_rules`, the single `for_each_event_store_rule!`, and the two `read`-shape tests in `crates/happenstance-core/src/memory.rs` | AC-005's single-sourcing half, and `_storymap.md`'s *Standing detectors these stories must not silence* items 3 and 4 |
| process gate | `redkiln validate --kb && redkiln doctor` | That this story wrote **no** `.kb/` atom — ADR-0023 is HS-S0057's, minted through `/redkiln:kb-ingest` |

Not run here: the full `cargo xtask ci` as a release bar (that is the terminal project's
`verify.e2e` grain, `closeout-and-durable-audience`'s), and anything under
`crates/happenstance-cloudflare/`.

## Risks and coupling (PR-scoped)

| risk | likelihood / impact | containment inside this PR |
| --- | --- | --- |
| No runner can be made to work inside `cargo xtask ci` at acceptable cost — the project's own named blocking question | Medium / High | EC-007: escalate, do not absorb. The story is sequenced first (`_storymap.md`, Merge order 1) precisely so this is answered before eleven stories depend on it, and its proof subject already passes, so the question cannot be dodged by moving a goalpost |
| The runner behaves differently on windows or macos than on the ubuntu-only standalone job | Medium / Medium | EC-006 and AC-007: a restriction is allowed, a *silent* one is not. Test Windows first, since it is the runner the existing job never covered |
| The `proof.rs` widening turns into a refactor of the three existing `ARTEFACTS` rows and their fingerprint-sharing property | Medium / Medium | The `--all-features` argument at `proof.rs:151-163` is about host fingerprint reuse: the wasm row must not inherit it (EC-004) and the three host rows must not lose it. If the widening starts changing what the host rows pass, stop and take the separate-shape route |
| Every subsequent story in this project pays a new local toolchain cost under shape (i) | High if (i) / Medium | NF-005: state it in one actionable sentence in the report. The testing brief asked for this to be surfaced deliberately rather than met as a surprise |
| CF-23 is `[FROZEN]`, and the pull to make its `Rule:` line describe the new reality shades into amending what it requires | Low / High | NF-006 and the Integration contract: the edit is confined to the `Rule:` line with `spec-trace` green, or it is not made at all. Anything altering the requirement is a new ADR and a re-plan |
| The step is written for `memory_conformance_wasm` alone, and HS-S0054 has to rebuild the wiring | Medium / High | AC-006 is the guard and names the wrong implementation. `every-rule-under-workerd`'s story-map row already says *register it in the gate step from `wasm-execution-seam`*; if that sentence is not true after this merges, the story is not done |
| Coupling outward: nothing depends on this story's *code*, but HS-S0054 depends on its *shape* | — | The only outgoing edge in this project's graph is to `every-rule-under-workerd`. AC-006 is the contract on that edge, and the implementation report names the entry point it will use |

## Dependencies

- **Blocks on:** *(none)* — `depends_on: []`. This story is milestone `wasm-execution-seam`'s
  sole member and merge order step 1 (`_storymap.md`, Slices and Merge order). Its proof
  subject, `crates/happenstance-testkit/tests/memory_conformance_wasm.rs`, already exists and
  already passes, so nothing has to land before it.
- **Unlocks:** `every-rule-under-workerd` (HS-S0054), which registers the Cloudflare
  conformance target in the seam this story builds and is where project AC-004 finally closes.
  Transitively, everything downstream of that: `measured-store-limits`,
  `wf-11-memory-ceiling-falsifier`, `deferral-re-reads-and-es-32-verdict`,
  `adr-0023-and-atom-resolutions`, and `publish-ready-crate` — whose AC-012 claim that the
  gate is green *including every `wasm32` step* only becomes meaningful once this row is in
  the array.
- **Independent of:** the whole `real-worker-bindings` milestone. No edge is declared between
  them and inventing one would be dishonest (`_storymap.md`, Merge order 2); they may be
  worked in either order.

## Anchors (progressive disclosure)

Open these when the row says to, not before. Each is linked rather than pasted — the Context
pack already carries the decisions, so nothing below is required merely to *start*.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `xtask/src/main.rs` | The composition root itself: `Step` (`:75-103`), `REQUIRED` (`:105`), the two existing wasm32 steps whose flag discipline you copy (`:219-264`), `wasm_steps`/`steps_named` (`:769-826`), `run_ci`/`run_fast`/`run_steps` (`:828-906`), and `print_help`'s `wasm` text (`:735-737`) | First, before writing a line — it is both the file you edit and the argument for how | AC-001, AC-002, AC-004 |
| `xtask/src/proof.rs` | The anti-vacuity contract in full: why naming a target is checking the filename (`:9-23`), the `Artefact` shape (`:58-70`), `ARTEFACTS` (`:133-149`), the `--all-features` fingerprint argument that must **not** transfer (`:151-174`), and the file-read style for cross-checking a list against its source (`:76-82`) | Immediately after learning whether the runner honours `-- --list` — that answer decides which half of this file you reuse | AC-003, AC-006 |
| `crates/happenstance-testkit/tests/memory_conformance_wasm.rs` | The executed target: 27 lines, and its module doc's claim that `cargo xtask wasm` only *type-checks* it (`:15-17`) is a sentence this story falsifies and must update | Before writing the step, and again before the diff is final so the doc edit is not forgotten | AC-001, AC-003 |
| `crates/happenstance-testkit/src/registry.rs` | `for_each_event_store_rule!` (`:94-140`) is the one enumeration, and `no_orphan_rules` (`:413-424`) is the detector a wasm-only subset would trip. The rule names your expectation list quotes come from here | When building the anti-vacuity expectation list, and again if any rule fails only on wasm32 (EC-008) | AC-003, AC-005 |
| `crates/happenstance-testkit/src/lib.rs` | The three emitters and why the wrapper is a parameter (`:55-89`), `__emit_wasm`'s reporting of a declined capability's reason, and the target-scoped-not-feature-scoped dependency argument (`:105-113`) that NF-004 rests on | When wiring the output flags, and when deciding where any new dev-dependency goes | AC-005, AC-006 |
| `.github/workflows/ci.yml` | The `gate` matrix (`:34-39`) this step must survive, and the standalone `wasm-conformance` job (`:189-237`) — the demonstrated runner shape to reuse, its `Cargo.lock`-resolved version, and the job whose fate you must settle | When choosing the runner wiring, and again when writing the disposition | AC-001, AC-007 |
| `crates/happenstance-testkit/Cargo.toml` | The two target-scoped dependency blocks and the `proptest` feature that make `--all-features` wrong on `wasm32` — EC-004 in its primary source rather than in summary | The moment you consider reusing `cargo_args` unchanged | AC-003 |
| `.bklg/from-contract-to-published-library/cloudflare-durable-object-store/_decomposition.md` | Architecture §5 (a probe-gated step is not a guard, `:402-422`), Testing §4 (merge-gate commands and the local cost, `:649-666`), Deployment DEPLOY-AC-01/-02/-05 (`:701-725`) and Notes §2–§3 (`:759-825`) — options (i)/(ii)/(iii) priced in full, including the cost side this spec only summarises | Before committing to the mandatory-versus-probe shape, which is AC-004's whole content | AC-004, AC-007 |
| `.bklg/from-contract-to-published-library/cloudflare-durable-object-store/project.md` | AC-004's exact wording (`:209-213`), the boundary-level DoD, and the risk register whose first entry makes EC-007 an escalation rather than a judgement call | At the start, and again at self-review before claiming the story is done | AC-001, AC-004 |
| `.bklg/from-contract-to-published-library/cloudflare-durable-object-store/_storymap.md` | The Backbone's three observers (`:24-38`), the coverage split that hands *the Cloudflare target is what it runs* to HS-S0054 (`:110-123`), and the standing detectors (`:171-189`) | When framing evidence for the personas, and before touching anything that might silence a detector | AC-005, AC-006 |
| `spec/SPECIFICATION.md` | CF-23 (`:7920-7948`) — the `[FROZEN]` clause whose observation this story strengthens, and the marker-and-citation discipline `spec-trace` enforces | Only if you decide to touch the `Rule:` line; otherwise read once to confirm you are not obliged to | AC-007 |
| `.bklg/from-contract-to-published-library/cloudflare-durable-object-store/_design.md` | The signed-off **no user-facing surface** determination (`:10-64`, `:106-111`) — the authority for this story rendering nothing and claiming no surface id | Only if a surface, report page or dashboard for the gate's output is ever proposed; the answer is already recorded | AC-005 |
| `.bklg/from-contract-to-published-library/initiative.md` | *Who this is for* (`:200-226`) and the referenced journeys (`:227-258`) — the adapter author's *Learn when you are finished* loop the acceptance criteria are framed from | When a criterion's framing starts reading as a bare capability rather than someone's goal | AC-001, AC-005 |
| `.bklg/from-contract-to-published-library/cloudflare-durable-object-store/_intake-brief.md` | The project's own open question in its original words: whether a runner can be made to exist in CI at acceptable cost, *a blocking finding, not a degradation to absorb* | Only if the answer is turning out to be no — read it before considering weakening anything | AC-004 |
| `CHANGELOG.md` | `:1094-1102` — the measured record that `println!` is a silent discard on `wasm32-unknown-unknown` and `console_log!` is not, which is the evidence behind AC-005's output requirement | When wiring the step's output flags, and when writing this story's own `[Unreleased]` entry | AC-005 |
| `RUNBOOK.md` | `:735-737` and `:761-762` (the named-not-indexed defect, named twice) and `:4241-4307` (phase 9, including the `vitest-pool-workers`-in-its-own-CI-job hypothesis at `:4267-4268`) | When writing AC-002's test, and when writing AC-007's verdict — that hypothesis is what your evidence is measured against | AC-002, AC-007 |

## Clarifications resolved during spec

1. **The AC set is the seven the front half enumerated — none added, none dropped.**
   AC-001 through AC-007 are exactly the ids the first pass decided, and `_ledger.md` carries
   one row per id.
2. **Project AC-004 is split, and this story owns three quarters of it.** *The step exists,
   is named not indexed, and is non-vacuous* is here; *the Cloudflare target is what it runs*
   is `every-rule-under-workerd`'s (`_storymap.md`, Coverage). A reviewer should not expect a
   Durable Object anywhere in this diff.
3. **The mandatory-versus-probe choice is deliberately left open by this spec and closed by
   the implementation.** AC-004 gates the *stating* of the choice and the non-negotiable
   property (silent non-execution fails), not which of (i) or (ii) is picked. Pre-deciding it
   here would be planning making an engineering call it has no measurement for — the
   deployment brief prices both and declines to pick, and this spec follows it.
4. **Whether `wasm-bindgen-test-runner` honours `-- --list` is unverified and stays
   unverified in planning.** EC-003 names the fallback so that discovering *no* cannot become
   an excuse for asserting on exit status. It is the single measurement to take first.
5. **`workerd` versus `wasm-bindgen-test-runner`.** The project's briefs use `workerd` as
   shorthand for *a real wasm32 single-threaded runtime*. This story deliberately builds on
   `wasm-bindgen-test-runner`, because it is the mechanism this repository has already
   demonstrated working (`.github/workflows/ci.yml:213-237`) and because this story's proof
   subject is `MemoryFixture`, which needs no platform SDK. Whether a Durable Object host
   ultimately needs more is `durable-object-host-and-fixture`'s question, and AC-006 is what
   keeps the seam able to answer it without a rewrite.
6. **No conformance rule and no `.kb/` atom is authored here.** Both were checked against the
   PR boundary rather than assumed: the rule set stays single-sourced and untouched (AC-005),
   and ADR-0023 belongs to HS-S0057 through the ingest path. This story records evidence and
   a choice; it mints nothing.
7. **The composition family of interaction quality is N/A by a signed-off determination**,
   not by this story's judgement — `../_design.md` was approved on 2026-08-12 recording no
   user-facing surface. The state family still applies, translated into the gate's terminal
   output, and every applicable invariant is carried by an `AC-###` row rather than by a
   prose bullet.
