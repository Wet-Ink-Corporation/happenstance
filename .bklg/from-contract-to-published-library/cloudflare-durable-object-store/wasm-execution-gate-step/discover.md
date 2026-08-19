---
item: HS-S0048
stage: discover
created: 2026-08-12T13:02:10.619Z
updated: 2026-08-12T13:02:10.619Z
template_sig: 86ce4036
rendered_sig: f306288a
---

# Discover — A wasm32 conformance run inside cargo xtask ci, not beside it

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| The slice's one-line: add a named, non-skippable `Step` that *executes* the existing wasm32 harness inside one `cargo xtask ci`, guarded by a fourth `proof.rs` `Artefact` row, and settle the fate of the standalone `wasm-conformance` job | `_storymap.md`, **Slices**, `wasm-execution-seam` row | The deliverable is three in-tree edits — a `Step`, a `wasm_steps()` name, an `Artefact` row — not a new test |
| AC-004: the step is registered **by name, not by index**, and a configuration in which it silently does not run fails the gate rather than skipping | `project.md`, **Acceptance criteria**, AC-004 | Two obligations, and the second is the hard one: "green because nothing ran" must be impossible |
| `dependsOn` is empty; this is the project's first merge and the only one that can escalate a blocking finding early | `_storymap.md`, **Merge order** §1, and **Why these milestones and not others**, first bullet | Nothing supplies this story. Its consumer is `every-rule-under-workerd`, so it is not an unproven island |
| No step in `xtask/src/main.rs` executes anything on `wasm32` today. The conformance-harness step is `cargo check --tests`, and its own comment says type-checking is what stops `__emit_wasm` rotting — explicitly not a run | `xtask/src/main.rs:219-244`; the Cloudflare step at `:245-264` is likewise a `cargo check` | AC-004 is new infrastructure, not a rewiring of an existing runner |
| The only place a rule actually executes on `wasm32` is a GitHub Actions job, on `ubuntu-latest` only, while the `gate` job matrices ubuntu/windows/macos | `.github/workflows/ci.yml:204-206`, `:34-39` | `cargo xtask ci` is the gate; a job beside it is not the same artefact, and it covers one third of the runners |
| That job already demonstrates the mechanism: `wasm-bindgen-cli` pinned to the version resolved out of `Cargo.lock`, driven through `CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER=wasm-bindgen-test-runner` | `.github/workflows/ci.yml:213-237` | Reuse over novelty. The runner question is about `workerd`, not about whether a wasm32 test can be run at all |
| A probe-gated step is not a guard: a constraint whose only check is skippable is unguarded on every machine that lacks one tool | `xtask/src/main.rs:192-202` | Stated in the tree already. This story is where it stops being prose |
| `cargo test` exits 0 on `running 0 tests`, so a file truncated to its `#![cfg(…)]` attributes passes a step that only names the target | `xtask/src/proof.rs:9-23` | Naming the target catches deletion; only asserting the test names out of `--list` catches emptying |
| `ARTEFACTS` carries three rows and `check` asserts each row's names out of `--list` before running them | `xtask/src/proof.rs:133-148`, `:191-217` | The anti-vacuity machinery exists; this story adds the fourth row rather than inventing a mechanism |
| `cargo_args` builds no `--target` and sets no runner env var | `xtask/src/proof.rs:164-173` | A wasm32 `Artefact` row needs plumbing `proof.rs` does not have. That is in this story, not a follow-on |
| Steps are selected by **name** through `steps_named`, which panics on an unresolvable name; the module doc records an index-selected step that once pointed at clippy while printing green | `xtask/src/main.rs:784-791`, `:816-826`, `:769-782` | The intended failure mode already exists; the story must use it rather than a numeric index |
| `run_fast` runs `REQUIRED` unmodified, and `cargo xtask ci --fast` is the bar this non-terminal project is held to | `xtask/src/main.rs:853-860`; `CLAUDE.md`, **Commands** | A `REQUIRED` placement puts the runner toolchain in every contributor's inner loop for the life of the project |
| CF-23 `[FROZEN]` requires three harnesses in-tree and names its Rule as the wasm32 build step "extended to compile a `wasm-bindgen-test` harness over the same rule set" | `spec/SPECIFICATION.md:7920-7951`, ledger row `:8734` | The clause names a compile. This story makes the check strictly stronger; it changes nothing the clause says |
| The target this step will run already exists and already passes | `crates/happenstance-testkit/tests/memory_conformance_wasm.rs:19-27` | The story cannot be "finished" by moving a goalpost — its proof is a file it did not write |
| The runbook proposes `vitest-pool-workers` **as its own CI job**, which is not the same artefact as "the same run as the rest of the gate" | `RUNBOOK.md:4267-4268`; `_decomposition.md`, Deployment brief §2 | A plan-of-record pointer and a starting hypothesis, not a settled decision |

## Questions

**The off-tokio harness shape (ADR-0023's live question) — answered for this
story's scope, deferred for the project's.** This story's target is
`memory_conformance_wasm.rs`, which needs no platform SDK: `wasm-bindgen-test`
under node is the demonstrated mechanism and is sufficient here. Whether the
*Cloudflare* target additionally needs `workerd` or `vitest-pool-workers` cannot
be answered until `worker-binding-layer` lands the real `SqlStorage` API, so it
is **deferred to `every-rule-under-workerd` and to ADR-0023**. One constraint
binds this story regardless: it must not pick a step shape the Cloudflare target
cannot reuse — the same `Step`, extended, not a second one beside it.

**`REQUIRED` or `OPTIONAL` plus a mandatory assertion?** Leading candidate is
option (ii) of the deployment brief's three (`_decomposition.md`, Architecture
brief Notes §5, Deployment brief §3): a probe-gated *run* paired with a
**mandatory, non-skippable** assertion that the target exists and holds the rule
names. AC-004's own wording forbids a bare probe with no compensating check, and
`run_fast` (`xtask/src/main.rs:853-860`) means a bare `REQUIRED` placement makes
a node toolchain a prerequisite for every `ci --fast` this project runs. Priced
and settled at **spec**, and recorded in ADR-0023 per DEPLOY-AC-02 — not left as
an implicit default.

**Windows and macOS determinism.** Deferred to **spec**. The existing job runs on
`ubuntu-latest` only (`.github/workflows/ci.yml:206`) while the gate matrices
three runners (`:34-39`). A documented platform restriction is permitted; a
silent one is not.

**The fate of `.github/workflows/ci.yml`'s `wasm-conformance` job (DEPLOY-AC-05).**
Answered in shape: it is retired only once the in-gate step subsumes what it
proves on all three runners, and until then its comment states the division of
labour. It must not be left as an unexplained second, weaker path.

**`proof.rs` has no `--target` plumbing.** Answered: adding it is in scope here,
because the fourth `Artefact` row is this story's anti-vacuity guarantee and it
cannot be added without it.

**CF-39 / CF-40's fixture-limits ownership.** Explicitly **not this story's** and
named so nobody settles it in passing: the numbers are `measured-store-limits`'
and the atom is `adr-0023-and-atom-resolutions`', coordinated with
`sqlite-durable-store` (HS-P0012) so the answer is minted once
(`.kb/open-questions/cf-40-fixture-limits-ownership.md`).

## Decision

The whole `!Send` port design is paid for by a target nothing in the gate has
ever executed on: `cargo xtask ci` builds `happenstance-cloudflare` for `wasm32`
and type-checks the testkit's wasm harness, and a `todo!()` body type-checks
against any signature — so the design's premise is currently guarded by a
compiler that was never asked the question. This slice buys the missing verb. It
adds one named `Step` to `xtask` that *runs* the wasm32 conformance harness that
already exists and already passes, plumbs `xtask/src/proof.rs` for a wasm32
target so a fourth `Artefact` row can assert the rule names out of `cargo test
--list` before anything executes, and states what happens to the standalone
`wasm-conformance` job. The spec will cover: the `Step` row and its exact
argv/env; its placement in `REQUIRED` or `OPTIONAL` with the compensating
mandatory assertion, decided rather than defaulted; the `wasm_steps()` name and
the `steps_named` resolution that makes a typo a panic instead of a silent
no-op; the `Artefact` row and the `--target`/runner arguments `cargo_args` must
grow; the three-runner story; and the escalation path if no runner can be made to
exist inside one `cargo xtask ci` at acceptable cost — which is a **blocking
finding** to raise, not a degradation to absorb. Nothing `[FROZEN]` is amended:
CF-23 names a compile step and this makes it a run, which is strictly stronger.

## The wrong implementation

**Mutant A — the step that reports green by not running.** A `Step` in
`OPTIONAL` whose `probe` is `["wasm-bindgen-test-runner", "--version"]` and
nothing else. On every machine and every CI runner without the tool,
`cargo xtask ci` prints `skipped` and exits 0. It passes fmt, clippy with `-D
warnings`, the workspace tests, all four existing `wasm32` `cargo check` steps,
docs, `spec-trace`, `package-check`, `cargo hack` and `cargo deny` — every check
in the tree — and the constraint it exists to guard is unguarded in exactly the
place it is unguarded today. `xtask/src/main.rs:192-202` already names this shape
in prose, which is why the mandatory `wasm32` check of `happenstance-core` is a
plain `cargo check` with the powerset widening left optional above it.

**Mutant B — the target that is nothing.** Point the step at a test target
reduced to its `#![cfg(target_arch = "wasm32")]` attribute, or with its emitted
rules renamed or `#[ignore]`d. `cargo test` prints `running 0 tests` and exits 0.
`xtask/src/proof.rs:9-23` names this defect exactly and explains why naming the
target was not enough: a step that a *deletion* fails and an *emptying* passes is
checking the filename.

**Where each detector must live.** Mutant B's is a fourth row in
`xtask/src/proof.rs`'s `ARTEFACTS` (`:133-148`) naming the wasm target and its
fully-qualified rule names, asserted out of `--list` before the run
(`:191-217`) — plus the `--target wasm32-unknown-unknown` and runner-env
arguments `cargo_args` does not currently build (`:164-173`). Mutant A's detector
cannot be a conformance rule at all: it is a property of the gate, not of an
adapter, so it belongs to `xtask`'s own tests as an assertion that the execution
step resolves through `steps_named` and is in the set the gate cannot skip —
reusing the panic `steps_named` already has (`xtask/src/main.rs:816-826`).
Neither belongs in `crates/happenstance-testkit/tests/mutation_coverage.rs`: every
row in that registry is a *store* that fails named *conformance rules*, and a
gate step is neither.

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

**Box 6.** No conformance rule is added here. This story runs the existing
`for_each_event_store_rule!` enumeration unchanged and asserts on rule *names*
out of `cargo test --list`, never on a position — the specification permits gaps
and a conformant adapter may leave them.

**Box 7.** CF-23 is `[FROZEN]` and this story touches it. It is **not changed**:
CF-23's Rule names the wasm32 build step "extended to compile a
`wasm-bindgen-test` harness over the same rule set" (`spec/SPECIFICATION.md:7920-7951`),
and running that harness is strictly stronger than compiling it. If no runner can
be made to exist inside `cargo xtask ci` at acceptable cost, the outcome is a
blocking finding escalated and, if it changes what the clause can promise, a new
decision atom and a re-plan — never a line edit to a frozen clause.

**Box 8.** No conformance rule here seems wrong. The one thing in the tree that
does read as wrong — a gate step whose only check is skippable — is the defect
this story removes, and `xtask/src/main.rs:192-202`'s comment is the reason,
already given in the same file.
