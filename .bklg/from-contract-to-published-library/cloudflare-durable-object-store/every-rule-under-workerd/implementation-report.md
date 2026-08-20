---
item: "HS-S0054"
stage: implement
created: "2026-08-19"
updated: "2026-08-19"
---

# Implementation Report — Every event-store conformance rule executed on the target, in the same gate run

> **Amended 2026-08-19, slice repair pass.** This report originally opened *"All eight ACs
> are satisfied"* and claimed initiative DoD 4 closed. Both sentences were withdrawn on
> review, and the correction is below rather than in a footnote, because the claim is the
> thing the story exists to make. **Seven of eight ACs are satisfied as written. AC-001 is
> satisfied except for its words *inside a real Durable Object runtime*, and project
> AC-002, project AC-004 and initiative DoD 4 are therefore reported OPEN, not closed** —
> see *Blocking finding* below. Nothing about the execution changed; what changed is the
> sentence describing what it executed against.

**The headline is one line of output:**

```text
happenstance-cloudflare/durable_object_conformance: 89 rules enumerated, 10 named, executing on wasm32-unknown-unknown
…
test result: ok. 89 passed; 0 failed; 0 ignored; 0 filtered out; finished in 0.18s
```

Eighty-nine of eighty-nine, first run, no failure to triage. It is the first time any
conformance rule in this workspace has run against a real adapter's `!Send` store on
`wasm32-unknown-unknown`, executed inside the gate rather than asserted in prose. What it
executed *against* is a `DurableObjectState`-shaped shim shipped in
`crates/happenstance-cloudflare/src/host.rs`, backed by Node's own `node:sqlite` and
reached through `worker`'s real `wasm-bindgen` externs — real SQLite, the production
adapter path, and **not `workerd`**.

The diff that produced it is deliberately thin: three lines of the shipped macro, one
registry row, one documentation section, one changelog entry. Everything else is the
negative space — after this merges there is no bespoke emitter, no `#[cfg]` over any
individual rule, no wasm-only rule list anywhere in the tree, and no configuration in which
the Cloudflare target compiles to nothing while the gate prints green.

## Blocking finding — no `workerd`-class runner exists inside `cargo xtask ci`, and one cannot be made to at acceptable cost

Raised as **blocking**, on the escalation path this story's own EC-006 and `project.md`'s
risk register pre-committed to (`project.md:301-308`, `_decomposition.md:410-422` option
iii, this spec's EC-006). It is recorded here rather than absorbed, and it is the reason
the ACs above are reported as they are.

**What was attempted, and what it costs.** The gate's execution seam is
`wasm-bindgen-test-runner` driving `wasm32-unknown-unknown` — one runner, one target, no
Node packages, resolved from `Cargo.lock` and installed by `cargo install`. A
`workerd`-class runner is a different artefact in every dimension that matters here:

| | in-gate today | `workerd`-class |
| --- | --- | --- |
| Toolchain | `wasm-bindgen-cli`, pinned by `Cargo.lock` | Node + `npm`/`pnpm` lockfile + `wrangler`/`miniflare`/`vitest-pool-workers`, versioned by nothing this repository owns |
| Target | `wasm32-unknown-unknown` | `wasm32-unknown-unknown` **plus** a `worker` entrypoint, a `wrangler.toml`, a Durable Object binding and a migration tag |
| Harness | `#[wasm_bindgen_test]`, the rules unchanged | a JS test file driving a `fetch` into the object; the rules would have to be re-expressed across an HTTP boundary or an RPC shim, which is a **second enumeration** and fails project AC-002 by construction |
| Gate shape | one mandatory step on three runner OSes | probe-gated at best — `workerd` is an external binary with no Windows-native story, and `xtask/src/main.rs:197-202` is explicit that a constraint whose only check is skippable is unguarded on every machine that lacks the tool |

The runbook's own proposal (`RUNBOOK.md:4267-4268`) is `vitest-pool-workers` **in its own
CI job**, which is precisely the artefact AC-07/DoD 4 rejects: not the same run, not the
same scroll, not reachable by the command a contributor types.

**What this story did instead, and why it is not the same thing.** It executed every rule
against real SQLite through the real `worker` bindings on the real target. That is a
strictly stronger result than the `cargo check` this project started from, and strictly
weaker than the AC's words. The substitution is a **trade**, and a trade has to be ratified
before it is claimed:

- **Owner.** `adr-0023-and-atom-resolutions` (HS-S0058). `project.md:301-308` already
  names reconciling the runbook's separate-job shape with the initiative's same-run
  requirement as **ADR-0023's first job**; this finding is the input to that.
- **Until then.** Project AC-002 (*every rule runs and passes*) is satisfied on
  `wasm32-unknown-unknown` and **not** under `workerd`; project AC-004 (*inside the gate,
  not beside it*) is satisfied for the step and **not** for the runtime; initiative DoD 4
  is **open**. No artefact in the tree may say otherwise, and
  `crates/happenstance-cloudflare/src/host.rs` and `src/lib.rs` now carry the sentence
  naming what is owed and who owns it — restored after an earlier pass deleted it.
- **What would close it.** Either ADR-0023 ratifies the substitution explicitly, with this
  cost table as its evidence, or the slice is re-planned around a runner that meets the
  AC's words.

## TDD Evidence

| AC | Test | Red | Green |
| --- | --- | --- | --- |
| AC-003 | `xtask/src/proof.rs::tests::the_cloudflare_conformance_target_is_a_row_and_not_a_second_step` | `the Cloudflare conformance target has no row in WASM_TARGETS, so cargo xtask ci compiles it and executes nothing — which is the exact position this project started in` | the row added; the test also counts `REQUIRED` steps matching `wasm32 run of` and requires exactly 1, so the "second Step with hard-coded args" shape fails rather than passing |
| AC-004 | the two negative controls, plus `::every_named_wasm_rule_is_one_the_enumeration_declares` | both controls performed against a *working* target and both failed before the run — output below | controls reverted; `89 rules enumerated, 10 named` |
| AC-001, AC-005 | `dcb_conformance_wasm::*` under `cargo run -p xtask -- wasm-conformance` | with no registry row the target compiled and executed nowhere — the failure this whole seam exists to end, and the one that leaves no output at all | 89 passed, three SKIP lines visible **at this story's own HEAD**; zero at slice HEAD once `measured-store-limits` had declared all three ceilings and `MID_BATCH_FAULT` — see *The skip lines, verbatim* below for where the live instance moved to |
| AC-002 | `rg -n "macro_rules!" crates/happenstance-cloudflare/` (empty); `registry::no_orphan_rules`; `::the_executed_wasm_targets_name_no_rule_of_their_own` | — structural, and the point is that it is *mechanical* rather than reviewed: the `--list` derivation is what makes a subset impossible, not a promise in this report | green |
| AC-006 | `cargo doc -p happenstance-cloudflare --no-deps` + review | — | the section landed at `src/lib.rs:28-73` |
| AC-007 | `cargo tree -p happenstance-cloudflare -e normal --depth 1` | — | identical before and after: `futures-core`, `happenstance-core`, `thiserror`, `worker` |
| AC-008 | `spec-trace`, `lint-constitution`, the four `!Send` probes, `git status` | — | all green; no `.kb/` and no `spec/SPECIFICATION.md` path |

### The two negative controls, run by hand

**A — the target emptied to its attributes** (the macro invocation replaced by a `const`):

```text
xtask failed: crates/happenstance-cloudflare/tests/durable_object_conformance.rs no longer
invokes the conformance suite; an emptied target exits 0 on `running 0 tests`
(looked for `event_store_conformance!`)
```

and, from the executing guard:

```text
xtask failed: `happenstance-cloudflare`'s `durable_object_conformance` on
wasm32-unknown-unknown is missing 89 of the 89 rules `for_each_event_store_rule!` declares:
dcb_conformance_wasm::two_fixture_instances_observe_none_of_each_others_appends, … and 79 more

The target builds, so `cargo test` would have exited 0 with nothing to say. … Listed: 0 name(s).
```

**B — the whole target `cfg`-ed away from its own target** (`#![cfg(not(target_arch = "wasm32"))]`):
the same 89-of-89 failure, naming the missing rules, before anything ran.

Both were reverted and the guards are green again. Note what the first control demonstrates
that the second does not: the *runner-free* guard fails too, so the anti-vacuity property
holds on a machine with no `wasm-bindgen-test-runner` installed at all.

### The skip lines, verbatim

Three, and no more — because this fixture's `REOPEN` is `SUPPORTED`, so the three reopen
rules **ran** rather than skipping. That absence is itself the evidence that the skip channel
is reporting rather than blanketing.

```text
SKIP arming_a_mid_batch_fault_makes_the_append_fail: fixture declines `MID_BATCH_FAULT` —
this Durable Object host can throw on a chosen statement, so the mechanism exists; what has
not been settled against an executed conformance run is which statement of this adapter's
write path is the k-th row's, and CF-39 requires a fixture claiming the capability to name
the mechanism rather than to hope
SKIP append_is_atomic_under_a_mid_batch_fault: fixture declines `MID_BATCH_FAULT` — …
SKIP append_reports_exceeded_store_limits: fixture declines `MAX_EVENT_DATA_LEN,
MAX_TAGS_PER_EVENT, MAX_EVENTS_PER_BATCH` — this fixture states no ceiling for any store
limit, so there is no capacity refusal for a rule to observe; a store with no ceiling is
reporting a fact about itself rather than declining to co-operate
```

The first two are the fixture's own words about *this* runtime. The third is the testkit's
`NO_CEILING_REASON`, and it is exactly the line `measured-store-limits` exists to delete.

**Amended 2026-08-19 — and `measured-store-limits` deleted all three.** At *slice* HEAD
`CloudflareFixture` declines nothing, so the conformance run prints **zero** `SKIP` lines
and the lines above are history rather than current gate output. That left project AC-003's
*emits* half and project DoD 2 with no live instance anywhere: the reporting path was
demonstrable only by declining something in a scratch build and reverting, which leaves
nothing a later change can break. This spec's AC-005 named that fallback and it is not
good enough on its own.

The repair pass made it standing instead. `tests/fixture_contract.rs` now defines a
`DecliningFixture` beside the real one and hands it to two of the **shipped**
capability-gated rules, in the same wasm32 target the gate already executes; the lines it
prints through the same `console_log!` sink `__emit_wasm` uses are asserted on, so the
format, the reason plumbing and the emitter all have something that fails when they break:

```text
SKIP append_is_atomic_under_a_mid_batch_fault: fixture declines `MID_BATCH_FAULT` — this
fixture arms no trigger on the object's `event` table, so there is nothing to make the k-th
row of a batch fail inside the store's own write path
SKIP acknowledged_writes_survive_a_reopen: fixture declines `REOPEN` — this fixture holds
no Durable Object `state` to re-derive a binding from, so it cannot discard handle state
without discarding the object's storage with it
```

Captured from `cargo test -p happenstance-cloudflare --target wasm32-unknown-unknown
--test fixture_contract -- --nocapture`, which is the row `xtask`'s `WASM_UNIT_TARGETS`
already drives inside the gate.

## Commits

One checkpoint, on `initiative/from-contract-to-published-library`, not pushed:

- `feat(cloudflare-durable-object-store): Every rule, executed under workerd`, carrying the
  trailer `Story: cloudflare-durable-object-store/every-rule-under-workerd` and this
  report's own body.

The SHA is deliberately not transcribed here, for the reason the sibling story's report
gives: this file is inside the commit it would name. `git log --grep "Story:
cloudflare-durable-object-store/every-rule-under-workerd"` is the lookup that cannot go
stale, and `redkiln record-links` settles `story.md`'s `links.commits`.

## Changes

| Path | Shape of the change |
| --- | --- |
| `crates/happenstance-cloudflare/tests/durable_object_conformance.rs` | **New.** `#![cfg(target_arch = "wasm32")]`, `mod support;`, and the three-line `event_store_conformance!` invocation with `mod_name = dcb_conformance_wasm` and `emit = happenstance_testkit::__emit_wasm`. The module documentation is the only substantial part, and it is what a reader needs: why the file is three lines, what a bespoke emitter would defeat, and why the detector is in `xtask` |
| `xtask/src/proof.rs` | One `WasmTarget` row for the package/target/module/family, plus `CLOUDFLARE_WASM_RULES` — ten transcribed names, each with its reason. One unit test asserting the row's identity *and* that `REQUIRED` still holds exactly one execution step. One repair: `unregistered_wasm_harnesses`' row-count guard now compares against the rows that live in the directory it scans |
| `crates/happenstance-cloudflare/src/lib.rs` | A new section, *Conformance: what has run, and what is deliberately not asked to* — the reasoned non-invocation of the concurrency family, the model family's target/feature status, and the status heading corrected to "conformant on its own target" |
| `CHANGELOG.md` | An `[Unreleased] / Added` entry naming what now executes that did not, and what the run does not claim |
| `standards/rust/25-…`, `50-…`, `52-…`, `61-…` | Eight `file:line` citations repointed — see **Notes** |

`xtask/src/main.rs` is untouched. No adapter body, no fixture constant, no testkit file, no
clause, no `.kb/` atom.

## Gates

| Command | Result |
| --- | --- |
| `CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER=wasm-bindgen-test-runner cargo test -p happenstance-cloudflare --test durable_object_conformance --target wasm32-unknown-unknown` | **89 passed, 0 failed** |
| `cargo run -p xtask -- wasm-conformance` | every registered target executed: 89 (memory) + 89 (local) + 17 (projection) + **89 (Cloudflare)** + 81 (cloudflare `--lib`) + 7 (fixture contract) |
| `cargo run -p xtask -- wasm-conformance-enumeration` | four rows, each held to its own enumeration, runner-free |
| `cargo test -p xtask` | 68 unit + 62 doctests, green |
| `cargo clippy -p happenstance-cloudflare -p happenstance-testkit -p xtask --all-targets --all-features -- -D warnings` | clean |
| `cargo test -p happenstance-cloudflare -p happenstance-testkit -p xtask --all-features` | green |
| `cargo run -p xtask -- lints` | six checks green, 27 constitution atoms consistent |
| `cargo run -p xtask -- spec-trace` | `traceability: no problems found` |
| `cargo doc -p happenstance-cloudflare --no-deps` | clean |
| `cargo tree -p happenstance-cloudflare -e normal --depth 1` | unchanged |
| `cargo fmt --all --check` | clean, run last |

**NF-002, the measured cost.** The Cloudflare conformance row adds ~0.18 s of execution to
the `wasm32 run of the conformance rules` step, on top of a build that was already happening
because the existing `wasm32 build of the Cloudflare adapter` step carries `--tests`. The
step's total is now six targets. This is not a second toolchain acquisition and not a second
`--target` build tree; it is one more `cargo test` invocation reusing the same artifacts.

## Notes

**One finding against `wasm-execution-gate-step`, repaired here rather than escalated, and a
reviewer should decide whether that was right.** EC-003 says that if registering a row is not
enough, the answer is a blocking finding against HS-S0048 rather than a local workaround.
Registering the row *was* enough for the step, the runner, the `--target` plumbing and both
guards — the contract held. What it was not enough for was one line inside
`unregistered_wasm_harnesses`:

```rust
if capable.len() < WASM_TARGETS.len() { bail!(…) }
```

That scan reads **one directory** (`crates/happenstance-testkit/tests`) by design, and
compared its findings against the length of the whole registry. With three testkit harnesses
and four rows it fires — accusing the scan of "reading somewhere the rows are not" on a
registry that is exactly right. The repair is three lines: count the rows whose `source`
starts with the directory being scanned. The guarded property is unchanged and still the
important one — every harness the scan *can* see must be registered.

This is a defect in HS-S0048's row-taking claim rather than in this diff, and it is recorded
as such. It was repaired rather than escalated because the alternative was to leave the gate
red on a correct registry, and because the fix is inside the same function whose message was
already telling the reader what had gone wrong.

**Eight `standards/rust/` citations were repaired, and that is the shape the
`real-worker-bindings` fence amendments established.** The documentation section AC-006 adds
to `src/lib.rs` moves every line below it, and four constitution atoms cite
`crates/happenstance-cloudflare/src/lib.rs` by `file:line` with an anchor string.
`lint-constitution` is a gate step, so a stale citation is red and repairing one the diff
itself invalidated has no in-bounds spelling. Only line numbers changed; not one atom's
normative text, and nothing in `spec/SPECIFICATION.md`.

**What this run does *not* discharge, said plainly because the temptation is real.** It ran
at the ceilings HS-S0053 *declared*, and all three are `None`. `append_reports_exceeded_store_limits`
therefore **skipped**, printing the testkit's own `NO_CEILING_REASON` — which is a green run
in which the workspace's one capacity-capped runtime contributed nothing at all to CF-40.
**This is not CF-40's discharge**, and `measured-store-limits` (HS-S0055) is where those
numbers become facts. The same holds for CF-39: `MID_BATCH_FAULT` is declined, so both
atomicity-under-fault rules skipped.

**Material for ADR-0023, recorded while it is in front of me.** The runner shape that worked
is `wasm-bindgen-test-runner` over Node with the `node:sqlite`-backed `DurableObjectState`
shim, driven by the existing gate step with `--nocapture`; no `wrangler` and no standalone
`workerd` binary was needed or present. The `SqlStorage` mapping behaved under real storage
with no surprises across all 89 rules — including the three `REOPEN` rules, which no adapter
in this workspace had ever executed, and `read_result_is_stable_under_concurrent_append`,
which is ADR-0011's ceiling-and-page under execution rather than under a `cargo check`. There
is **no** wasm-only divergence to report. None of this is written to `.kb/`; it is
`adr-0023-and-atom-resolutions`' to mint through `/redkiln:kb-ingest`.
