---
item: "HS-S0048"
stage: report
created: "2026-08-19"
updated: "2026-08-19"
---

# Report — A wasm32 conformance run inside cargo xtask ci, not beside it

## Findings Ledger

**Outcome: seven of seven ACs satisfied. Nothing deferred, nothing blocked, no rule added
and no `.kb/` atom written.** The project's own named blocking question — EC-007, *can a
wasm32 runner exist inside `cargo xtask ci` at acceptable cost* — is answered **yes**, so
no escalation is owed and AC-004 was not downgraded to a `cargo check`.

One measurement decided the design and was taken before any code, as the spec asked:
`wasm-bindgen-test-runner` **does** honour `-- --list`. EC-003's weaker fallback (reading a
count out of the run's own output) is therefore not in the diff, and the gate asserts test
names *before* running them, which is the full `proof.rs` contract rather than an
approximation of it.

### AC by AC

| AC | result | what proves it | mount point |
| --- | --- | --- | --- |
| **AC-001** — one `cargo xtask ci` **executes** wasm32 conformance rules | satisfied | `cargo xtask ci --fast` prints `=== wasm32 run of the conformance rules ===`, then `89 rules enumerated, 9 named, executing on wasm32-unknown-unknown`, then `test result: ok. 89 passed; 0 failed`, and ends `all required checks passed`. Test `xtask/src/main.rs:1538` rejects a sixth `cargo check` by asserting `!args.contains("check")`. RED: *no step named `wasm32 run of the conformance rules`* | `REQUIRED` row at `xtask/src/main.rs:412`, iterated by `run_ci` and `run_fast` |
| **AC-002** — selected by name, never by index | satisfied | `wasm_steps()` (`:1009`) carries both names through `steps_named`, which panics on a miss. `xtask/src/main.rs:1455` asserts the resolved list equals `WASM_STEPS` exactly, with the five checks still at indices 0–4 (RED: five against seven). Help text rewritten (`:934-942`) and held by `:1633` (RED: *still describes the family as builds and checks alone*) | `wasm_steps()`; `cargo xtask wasm` runs seven sections |
| **AC-003** — an emptied or renamed target fails the gate | satisfied | Two mechanisms. `wasm_run` (`proof.rs:748`) asserts **all 89** enumerated rules out of `--list` before running; `wasm_enumeration` (`:847`) re-asserts the target with no runner at all. Three hand-run controls in the implementation report: emptied target (both halves fail by name), renamed rule (fails by name), and a rename the parser could not read — which **passed** until the parser was tightened to refuse rather than silently shrink | `probe: None` row at `xtask/src/main.rs:354` |
| **AC-004** — silent non-execution is a failure, not a skip | satisfied | Shape **(ii)**, argued in the step's own comment (`xtask/src/main.rs:373-411`) with shape (i) named and priced as the rejected alternative. `xtask/src/main.rs:1580` asserts the compensator's `probe` is `None` *and* that the comment names both `mandatory` and `rust-toolchain.toml`, so the argument cannot be deleted while the code survives. Control with the runner off `PATH`: the mandatory row reported, the probed row printed `skipped: … did not succeed` | the `probe` fields of the two rows |
| **AC-005** — declined-capability reasons stay legible; rule set single-sourced | satisfied | Measured: 0 `SKIP` lines without the flag, **6** with `--nocapture`, and `--show-output` — the host step's flag — makes this runner exit 1. No rule added, removed or `#[cfg]`-ed; `registry::no_orphan_rules` green; `proof.rs:1318` forbids the target naming any rule of its own, and `wasm_run` asserts the enumeration is reached *exhaustively*, closing the subset question from both sides | `proof.rs:826-828` |
| **AC-006** — the seam takes a second target by registration | satisfied | `WASM_TARGETS` (`proof.rs:383`) is a list of rows carrying package, target, module, source and rules; the runner wiring, version check and both gate steps are target-independent. `xtask/src/main.rs:1664` walks `REQUIRED` **and** `OPTIONAL` asserting no step's `args` names a registered target — the named wrong implementation's detector. HS-S0054's exact entry point is written out in the implementation report | `proof.rs:383` |
| **AC-007** — the standalone CI job has a stated disposition | satisfied | **Retired.** `.github/workflows/ci.yml:189-214` is a retirement note in place of the job, saying what it proved, why the in-gate step subsumes it (three runners instead of one, the same command a contributor runs, and an anti-vacuity guard the old job never had) and what moved rather than being deleted. The stale *"Those arrive at phase 9"* sentence is corrected and points at HS-S0054 as a **row**. The `Cargo.lock`-resolved version and install moved into the `gate` job (`:87-121`, `shell: bash` for Windows), never hard-coded — and the gate now makes the same comparison itself (`proof.rs:675`). `cargo xtask spec-trace` green after the CF-23 `Rule:` line edit | `.github/workflows/ci.yml`; `spec/SPECIFICATION.md:8374-8377` |

### The three-runner verdict, with its honest limit

No platform restriction is imposed. **windows-latest** is verified directly — every
measurement here was taken on Windows, on purpose, because it is the runner the retired job
never covered. **ubuntu-latest** is proven by the retired job's own history.
**macos-latest is not verified by me** and is not claimed: there is no known obstacle
(`taiki-e/install-action` publishes the tool for macOS; the runner drives node, which the
image carries), and the first `gate` run on that leg is the evidence. Under shape (ii) that
residual is a legible `skipped:` line on one leg with the mandatory guard still running,
rather than a red gate — which was an argument *for* the shape, not a consolation after it.

### Deferred, blocked, or deliberately not done

Nothing deferred and nothing blocked. Deliberately out of this diff, and checked rather
than assumed: no file under `crates/happenstance-cloudflare/`; no `.kb/` write (ADR-0023,
CF-40 and WF-11 stay HS-S0057's, minted through `/redkiln:kb-ingest`); no conformance rule
added or edited; no `[FROZEN]` clause's normative text or marker altered.

### Things a reviewer should look at first

1. **The probe, at `xtask/src/main.rs:412-414`.** It is the one place this story departs
   from the reflex *mandatory or nothing*, AC-004 explicitly admits it, and the comment
   above it is where the argument has to hold up. The pairing with `:354` is what makes it
   admissible; judge them together, never the probed row alone.
2. **The two-sided single-sourcing check.** `wasm_run` asserts every enumerated rule
   *reaches* the wasm target, and `the_executed_wasm_targets_name_no_rule_of_their_own`
   asserts the target *names none itself*. Either alone leaves a way to build a subset.
3. **The parser-refusal control.** A rename to a spelling the enumeration scan could not
   read shrank the rule set from 89 to 88 and passed. It fails now. It is the most
   instructive failure this story found, because it is this repository's own vacuity
   defect reproduced one level inside the tool written to prevent it.
4. **Five files under `standards/rust/` in the diff.** Thirteen line-number citations only,
   forced by insertions into `main.rs` and repaired against their anchors; no atom prose
   changed. `cargo xtask lint-constitution` reports `27 atoms, all consistent`.

### Gate evidence

`cargo xtask affected --base main` → `affected gate passed`. `cargo xtask ci --fast` →
`all required checks passed (--fast: 4 optional step(s) not run)`, with both new sections in
the scroll. `cargo fmt --all --check` green, run last. `cargo test -p xtask --bins` →
`62 passed; 0 failed`. `cargo xtask spec-trace` →
`traceability: no problems found; §7.1–§7.2 matches the checker`.
