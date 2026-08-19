---
item: "HS-S0048"
stage: implement
created: "2026-08-19"
updated: "2026-08-19"
---

# Implementation Report — A wasm32 conformance run inside cargo xtask ci, not beside it

**All seven ACs are satisfied. Nothing is blocked, nothing is deferred, and EC-007 was
not reached.** The blocking question the project sequenced this story first to answer —
*can a wasm32 runner be made to exist inside `cargo xtask ci` at acceptable cost* — is
answered yes, and the second unverified question underneath it (*does
`wasm-bindgen-test-runner` honour `-- --list`*) is answered yes as well, which is what
kept EC-003's weaker fallback out of the diff.

The measurement came first, as the spec's implementation notes asked. Before a line was
written: `wasm-bindgen-test-runner` was driven by hand on **Windows** — deliberately, since
that is the runner the retired ubuntu-only job never covered — and it enumerated all 89
rules through `-- --list` and then executed all 89 green. That single fact decided the
whole shape: the full `Artefact`-style *assert the names before you run them* contract was
available, so the gate did not have to fall back to reading a count out of the run's own
output.

## TDD Evidence

Two beats of RED, both behavioural rather than compile-level, then GREEN.

**RED beat 1 — the scaffold, with nothing registered.** `WasmTarget` and an **empty**
`WASM_TARGETS` were declared so the `proof.rs` assertions could execute and fail on their
own subject rather than on a missing symbol.

**RED beat 2 — the whole suite, before any step existed.** `cargo test --locked -p xtask
--bins`, five failures, each asserting the missing behaviour:

```
---- proof::tests::every_executed_wasm_target_carries_its_own_package_and_target stdout ----
no conformance target is registered for execution on wasm32, so the gate's wasm32 story is still a `cargo check`

---- tests::the_wasm32_conformance_run_executes_rather_than_checks stdout ----
no step named `wasm32 run of the conformance rules`

---- tests::the_wasm32_run_is_probed_and_its_compensator_never_is stdout ----
no step named `wasm32 run of the conformance rules`

---- tests::wasm_steps_resolve_and_the_check_family_keeps_its_order stdout ----
assertion `left == right` failed: `cargo xtask wasm` no longer runs the designed family in the designed order
  left: [… five names …]
 right: [… seven names, ending "wasm32 conformance targets are non-vacuous", "wasm32 run of the conformance rules"]

---- tests::the_help_text_says_the_wasm32_tasks_execute_rules stdout ----
the `wasm` help still describes the family as builds and checks alone: … "for wasm32-unknown-unknown. five checks; …"
```

**GREEN.** `cargo test --locked -p xtask --bins` → `62 passed; 0 failed`.

| AC | test(s) | red → green |
| --- | --- | --- |
| AC-001 | `xtask/src/main.rs:1538` `the_wasm32_conformance_run_executes_rather_than_checks` | RED *no step named …*; the test rejects a sixth `cargo check` by asserting `!args.contains("check")` and requires the comment to name `--nocapture` and execution |
| AC-002 | `xtask/src/main.rs:1455` `wasm_steps_resolve_and_the_check_family_keeps_its_order`; `:1633` `the_help_text_says_the_wasm32_tasks_execute_rules` | RED five names against seven, and the help text still saying *five checks* |
| AC-003 | `xtask/src/proof.rs:1277` `every_named_wasm_rule_is_one_the_enumeration_declares`; `:1239` `every_executed_wasm_target_carries_its_own_package_and_target` | RED *no conformance target is registered for execution on wasm32*; plus the three hand-run controls below |
| AC-004 | `xtask/src/main.rs:1580` `the_wasm32_run_is_probed_and_its_compensator_never_is` | RED *no step named …*; asserts the compensator's `probe` is `None` **and** that the run's comment names `mandatory` and `rust-toolchain.toml`, so the argument cannot be dropped while the code stays |
| AC-005 | `xtask/src/proof.rs:1318` `the_executed_wasm_targets_name_no_rule_of_their_own`; the measured `--nocapture` delta below | the target may name the emitter, the fixture and the module, and no rule at all |
| AC-006 | `xtask/src/main.rs:1664` `no_gate_step_hard_codes_an_executed_wasm_target`; `xtask/src/proof.rs:1239` | walks `REQUIRED` **and** `OPTIONAL` against every registered target name — the named wrong implementation's detector |
| AC-007 | the workflow diff; `cargo xtask spec-trace` green | — |

### Negative control 1 — the emptied target (AC-003)

`crates/happenstance-testkit/tests/memory_conformance_wasm.rs` truncated to its
`#![cfg(target_arch = "wasm32")]` line. Both halves fail, and the mandatory one fails
without a runner:

```
=== wasm32 conformance targets are non-vacuous ===
xtask failed: crates/happenstance-testkit/tests/memory_conformance_wasm.rs no longer
invokes the conformance suite; an emptied target exits 0 on `running 0 tests`
(looked for `event_store_conformance!`)

=== wasm32 run of the conformance rules ===
xtask failed: `happenstance-testkit`'s `memory_conformance_wasm` on wasm32-unknown-unknown
is missing 89 of the 89 rules `for_each_event_store_rule!` declares: … and 79 more
```

Before this story that same truncation exited 0 everywhere in the gate.

### Negative control 2 — a renamed rule (AC-003)

`acknowledged_writes_survive_a_reopen` renamed to `…_a_restart` in `registry.rs`:

```
xtask failed: memory_conformance_wasm's row names `acknowledged_writes_survive_a_reopen`,
which `for_each_event_store_rule!` does not declare. Either the rule was renamed and this
list was not, or this list names a rule that never existed.
```

### Negative control 3 — a rename the parser cannot read

Found by running control 2 first with `append_is_atomic_RENAMED`: the enumeration scan
**silently dropped** the unreadable name, shrank from 89 rules to 88, and passed. That is
the vacuity failure one level in from the one this story is about, so the parser was
tightened to refuse rather than skip:

```
xtask failed: crates/happenstance-testkit/src/registry.rs lists `append_is_atomic_RENAMED`,
which is not a snake_case rule name. The enumeration's shape has changed; a name this scan
cannot read is a rule silently missing from every check built on it.
```

### Negative control 4 — the runner absent (AC-004)

`wasm-bindgen-test-runner` moved off `PATH`, then `cargo xtask wasm`:

```
=== wasm32 build of the typed layer ===
=== wasm32 conformance targets are non-vacuous ===
happenstance-testkit/memory_conformance_wasm: 9 named rules, all declared by the one enumeration of 89
=== wasm32 run of the conformance rules ===
skipped: `wasm-bindgen-test-runner --version` did not succeed
```

The mandatory row ran and reported; the probed row skipped and named the exact command that
answered. That is shape (ii) behaving as designed, and the binary was restored immediately
afterwards.

### The `--nocapture` measurement (AC-005)

Same command, same target, one flag apart:

| invocation | `SKIP <rule>: <reason>` lines reaching the terminal |
| --- | --- |
| `cargo test … --target wasm32-unknown-unknown` | **0** |
| `… -- --nocapture` | **6** |
| `… -- --show-output` | *runner exits 1* — `--show-output` is not this runner's flag |

The third row is the one worth keeping: the host `tests` step's flag is **not** a synonym
here, and copying it would have failed the gate outright. With `--nocapture` the gate's own
scroll carries lines like

```
SKIP acknowledged_writes_survive_a_reopen: fixture declines `REOPEN` — MemoryEventStore is
a Vec behind an RwLock, so there is no durable medium to reopen over: discarding process
state is indistinguishable from discarding the events
```

## Commits

One checkpoint commit, on `initiative/from-contract-to-published-library`, not pushed:

```
feat(cloudflare-durable-object-store): A wasm32 conformance step inside the gate, not beside it

    Story: cloudflare-durable-object-store/wasm-execution-gate-step
```

Resolve it with
`git log --grep "Story: cloudflare-durable-object-store/wasm-execution-gate-step"`.
The trailer is cited rather than a hash because this report is *inside* the commit it
describes, so any hash written here is the hash of a commit that no longer exists the
moment the report is added to it — and the trailer is the same identifier the workflow's
own double-commit guard resolves by. The 14 files it carries are the ones in ## Changes,
plus this story's `_ledger.md`, `report.md` and the run's telemetry.

## Changes

| file | shape of the change |
| --- | --- |
| `xtask/src/proof.rs` | The deliverable. `WasmTarget` (:320) and `WASM_TARGETS` (:383) — a declared list of rows; `MEMORY_WASM_RULES` (:364) — nine named rules with the argument for each group; `wasm_cargo_args` (:620) — **no `--all-features`**, EC-004; `locked_wasm_bindgen_version` (:647) and `check_wasm_runner_version` (:675) — EC-002, derived from `Cargo.lock`, never pinned; `wasm_run` (:748) — enumerate, assert exhaustively and by name, then execute under `--nocapture`; `wasm_enumeration` (:847) — the runner-free compensator; `enumerated_rules` (:456) — the one enumeration, parsed and **refusing** what it cannot read. `list` was generalised from `&Artefact` to `(&[&str], &[(&str,&str)], &str)` so the wasm rows share the libtest parsing; the host rows' arguments are byte-identical, which the risk register asked for explicitly. |
| `xtask/src/main.rs` | Two `REQUIRED` rows — `wasm32 conformance targets are non-vacuous` (:354, `probe: None`) and `wasm32 run of the conformance rules` (:412, probed) — both names added to `wasm_steps()` (:1009); two subcommands (:884-885); the `wasm` help rewritten (:934-942); the module doc's account of the gate updated; five new `#[cfg(test)]` tests. |
| `crates/happenstance-testkit/tests/memory_conformance_wasm.rs` | Module doc only. Its claim that `cargo xtask wasm` merely type-checks it is now false; and its opening *"The same 30 rules"* was stale against 89, so it now states no count at all and says why. |
| `.github/workflows/ci.yml` | The `wasm-conformance` job retired in place, replaced by a retirement note that says what it proved and what subsumes it; its `Cargo.lock` version resolution and `wasm-bindgen-cli` install moved into the three-runner `gate` job, with `shell: bash` for Windows. |
| `CHANGELOG.md` | One `[Unreleased] / Added` entry. |
| `spec/SPECIFICATION.md` | CF-23's `Rule:` line only, plus the regenerated §7.1 row. `[FROZEN]` marker and every normative sentence untouched (NF-006). |
| `standards/rust/*.md` (5 files, 13 citations) | Line-number repointing only, forced by the insertions into `main.rs`. See ## Notes. |

## Gates

| gate | result |
| --- | --- |
| `cargo test --locked -p xtask --bins` | `62 passed; 0 failed` |
| `cargo clippy --locked -p xtask -p happenstance-testkit -p happenstance-cloudflare --all-targets --all-features -- -D warnings` | clean |
| `cargo test --locked -p xtask -p happenstance-testkit -p happenstance-cloudflare --all-features` | all targets green, `0 failed` throughout |
| `cargo fmt --all --check` | green (formatter run last, after the lint pass) |
| `cargo xtask affected --base main` | `affected gate passed` |
| `cargo xtask ci --fast` | `all required checks passed (--fast: 4 optional step(s) not run)`, with `=== wasm32 conformance targets are non-vacuous ===` and `=== wasm32 run of the conformance rules ===` in the scroll |
| `cargo xtask spec-trace` | `traceability: no problems found; §7.1–§7.2 matches the checker` |
| `cargo xtask lint-constitution` | `27 atoms, all consistent` |
| `redkiln validate --kb && redkiln doctor` | see below — no `.kb/` write in this diff |

## Notes

**The shape decision, and it is ADR-0023 material rather than an ADR.** Shape **(ii)**:
a probed execution row paired with a mandatory, `probe: None` compensator. The argument is
written into the step's own comment at `xtask/src/main.rs:373-411`, per the house habit,
and is summarised here because AC-004 asks for the choice *and its rejected alternative*.

The deciding distinction is a **class of tool**, not a preference. Every mandatory row in
`REQUIRED` needs only what `rust-toolchain.toml` pins — and that file pins the
`wasm32-unknown-unknown` *target*, which is why the five existing wasm32 `cargo check` rows
are `probe: None` and correctly so. `wasm-bindgen-cli` is a separately `cargo install`ed
binary that must match the `wasm-bindgen` schema version in `Cargo.lock` **exactly**, so a
`cargo update` can invalidate an already-installed one. rustup cannot supply it. That puts
it in the same class as `cargo hack` and nightly, both of which this gate probes.

Shape (i) — mandatory, `probe: None` — was close, and was declined on cost: it makes
`cargo xtask ci --fast` unpassable until every contributor has installed the runner and a
node host, a bar this project's remaining eleven stories would pay at every seam, and one
that goes red on a *dependency bump* rather than on a code change. What made declining it
safe is that the compensator recovers the part of shape (i) that is about this
repository's own code — an emptied target, one rewired away from `__emit_wasm`, one
carrying a hand-written subset, or a deleted registration all fail on every machine — and
CI recovers the rest, because the `gate` job now installs the runner on all three matrix
runners. `xtask/src/main.rs`'s comment and `.github/workflows/ci.yml`'s install step point
at each other for exactly that reason: deleting the install does not turn the gate red, it
turns a claim into a lie, so it is the one thing that had to be written down at both ends.

**NF-005, the local cost, in one actionable sentence:** a contributor without
`wasm-bindgen-cli` sees one `skipped:` line and a still-green `cargo xtask ci --fast`, and
to run the wasm32 conformance rules locally they need node plus
`cargo install wasm-bindgen-cli --version <the wasm-bindgen version in Cargo.lock> --locked`
— which the gate itself will name for them, including the exact version, the moment an
installed runner disagrees with the lock file.

**NF-001, the cost:** warm, the compensator is **0.4 s** and the run is **5.8 s** (two
cargo invocations — the enumeration and the run — against one already-built wasm32 test
target). Cold, it adds one *build* of `memory_conformance_wasm` for `wasm32`, which the
existing `cargo check --tests` step cannot share a fingerprint with because check and build
are different profiles. The wasm row deliberately passes the same flags as the two existing
wasm32 steps — and in particular **not** `--all-features` — which is both EC-004 and what
keeps the dependency graph shared rather than rebuilt.

**AC-007's three-runner verdict, stated with its limits.** No platform restriction is
imposed and none appears to be needed.

- **windows-latest** — verified directly. Every measurement in this report was taken on
  Windows, deliberately, because it is the runner the retired ubuntu-only job never
  covered. Enumeration and execution both work; `-- --list` is honoured; 89/89 green.
- **ubuntu-latest** — proven by the job being retired, which drove this exact runner
  through `taiki-e/install-action` on every push.
- **macos-latest** — **not verified by me**, and I will not claim it. `taiki-e/install-action`
  publishes `wasm-bindgen-cli` for macOS and the runner drives node, which the image
  carries, so there is no known obstacle; the first `gate` run on that matrix leg is the
  evidence. This residual risk is materially smaller under shape (ii) than it would have
  been under shape (i): if macOS turns out to lack the runner, the outcome is a legible
  `skipped:` line on one leg with the mandatory compensator still running, not a red gate
  on a runner nobody can reproduce locally. That property was a reason for the choice, not
  a consolation after it.

**HS-S0054's entry point, so the seam's consumer does not have to reverse-engineer it.**
Add one row to `WASM_TARGETS` (`xtask/src/proof.rs:383`):

```rust
WasmTarget {
    package: "happenstance-cloudflare",
    target:  "<the Cloudflare conformance target>",
    module:  "<its mod_name>",
    source:  "crates/happenstance-cloudflare/tests/<that file>.rs",
    rules:   <the rules worth naming for it>,
}
```

Nothing else. No second gate step, no second `Step.env`, no second runner wiring, no second
version check — and `no_gate_step_hard_codes_an_executed_wasm_target` is the test that fails
if a later change makes that untrue. Note one thing the shared code already does that will
matter there: `wasm_run` asserts the **whole** `for_each_event_store_rule!` enumeration
against each registered target's listing, so a Cloudflare harness that reaches only some of
the rules fails on registration rather than passing quietly.

**Deviations from the plan, stated rather than buried.**

1. **`Step.env` was not used for the runner variable**, though the spec's behaviour table
   proposed it. `Step.env` would set `CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER` on the
   `cargo run -p xtask` process and reach the real `cargo test` only by inheritance — which
   works inside the gate and leaves `cargo xtask wasm-conformance` broken as a standalone
   command, against the *reachability without the full ceremony* invariant AC-002 and AC-003
   carry. It is set per-`Command` in `proof.rs:396-414` instead, which honours Decision 5's
   own reason (*a per-target variable must not become a process-wide one*) more strictly:
   the variable never enters this process's environment at all. The step's `env` is `&[]`.
2. **A separate `WASM_TARGETS` array rather than optional fields on `Artefact`.** The spec
   offered both. The risk register's *stop and take the separate-shape route* condition was
   met on sight: `--all-features` is load-bearing for the host rows' fingerprint sharing and
   does not compile on `wasm32`, so widening `Artefact` would have put a per-row conditional
   inside the argument list the host rows depend on being identical. The host rows' flags
   are byte-for-byte unchanged.
3. **Two rows, not one.** AC-004's shape (ii) is literally a pair, and splitting them is
   what lets one be skippable and the other never be.
4. **`wasm_steps_resolve_and_number_five` was renamed** to
   `wasm_steps_resolve_and_the_check_family_keeps_its_order`. Every assertion it carried is
   still there — the four originals at indices 0–3, the typed layer at 4, their arguments
   and their absent probes — plus the two new names. A count in a test name is a second copy
   of the list it describes. HS-S0031's own report and spec cite the old name; this note is
   the forwarding address.
5. **13 line-number citations in `standards/rust/` were repointed.** Inserting ~90 lines
   into `main.rs` moved every anchor below them and `lint-constitution` went red with
   `the citation points at the wrong place`. `--write` only regenerates the router's
   region, so the citations were repaired by hand against the anchors' new lines; no atom's
   prose was edited. RS-80-2's claim that *CI installs every probed tool on every runner, so
   the step is not skipped there* stays true precisely because the `wasm-bindgen-cli` install
   was added to the `gate` job rather than left in the retired one.

**What this story did not do, checked rather than assumed.** No file under
`crates/happenstance-cloudflare/`. No `.kb/` write — ADR-0023, CF-40 and WF-11 remain
HS-S0057's, through the ingest path. No conformance rule added, removed or `#[cfg]`-ed, and
no second enumeration: `registry::no_orphan_rules` and `for_each_event_store_rule!` are
untouched by this diff, which is what AC-005 asks for and what the derived check now
enforces from the other direction.
