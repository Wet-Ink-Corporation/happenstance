# `experiments/gate-vacuity`

**How much of `cargo xtask ci` is load-bearing?** Three counts, taken against
the pinned commit `56ef6c5`, none of which changes shipped code.

The whole pre-publication review rests on one sentence — *"`cargo xtask ci` is
green at `56ef6c5` with all four optional steps run"* — so it is worth knowing
what that sentence excludes. This is the measurement that decides how much of
the rest of the review should be believed, which is why it was run first.

It is not a crate. There is no `Cargo.toml`, so cargo cannot see this directory
at all and no gate step can grow a dependency on it by accident;
`experiments/position-visibility/` is the precedent for a shell-driven
experiment with no crate of its own. **`run.sh` is not a gate step and must
never become one** (CF-34): nothing in `.redkiln/config.yaml` or
`xtask/src/main.rs` invokes it.

**It writes nothing inside the repository except its own `results/`.** Every
edit is applied to a detached `git worktree` of `56ef6c5` outside the tree
(`/d/gv-wt` by default, `$GATE_VACUITY_WT` to move it) and reverted with `git
checkout -- .` between arms. `run.sh` creates that worktree if it is absent and
leaves it in place afterwards; the run that produced the figures below removed
it again with `git worktree remove --force /d/gv-wt`, because it carries a
second full `target/`. Re-running any phase recreates it, at the cost of one
cold build.

## Conditions

Every figure below was produced under these, and every one is an **exit status
or a count** — this experiment times nothing, so the house rule that timed runs
are `--release` does not apply to it and would be wrong if it did: `cargo xtask
ci` builds in the dev profile by definition, and forcing `--release` would
measure a different gate.

| | |
| --- | --- |
| Machine | 13th Gen Intel Core i9-13905H, 20 logical cores, 32 GB RAM |
| OS | Windows 11 Home (10.0.26200) |
| Filesystem | NTFS, local NVMe (`D:`) |
| Shell | Git Bash; `python 3.14.6` drives the six source edits |
| Toolchain | `rustc 1.97.1 (8bab26f4f 2026-07-14)`, `cargo 1.97.1`, `x86_64-pc-windows-msvc` — `rust-toolchain.toml`'s pin, read back from `rustc --version` rather than assumed |
| Build profile | dev, because that is what the gate uses |
| `core.autocrlf` | `true` — **read back off the live checkout**, and load-bearing; see below |
| Optional tools | `cargo-hack 0.6.45`, `cargo-deny 0.20.2`, `wasm-bindgen-test-runner 0.2.126`, Node `v24.18.0`, `nightly-x86_64-pc-windows-msvc` — all present, so **no step skipped in any arm** |
| Commit | `56ef6c5`, in a detached worktree; `results/raw/worktree-clean.txt` records the SHA and an empty `git status --porcelain` |
| Command | `bash experiments/gate-vacuity/run.sh` |
| Wall clock | dominated by one cold build of a second `target/`; the arms after it are incremental. Not instrumented — no figure here is a duration |

**`core.autocrlf` is this experiment's `PRAGMA synchronous`.** It is read off
the checkout rather than trusted from the config that set it, because it decides
whether the instrument works at all. Every file in the worktree is CRLF while the
blob in git is LF; `cargo fmt --check` is the gate's *first* step and rustfmt's
default `newline_style = "Auto"` infers the ending from the file, so an inserted
bare `\n` turns the gate red at step 0 for a reason that has nothing to do with
`#[ignore]`. Getting the answer "red" for the wrong reason is the exact failure
mode this experiment exists to name, so `mutate.py` reproduces each file's
existing ending and each `fn` line's indentation.

## Controls before counts

An arm that is fast and wrong wins every benchmark; a harness that cannot fail
reports every arm green. Five controls run before anything is counted, and all
five fired.

| control | expects | got | raw |
| --- | --- | --- | --- |
| **GREEN** — the unedited worktree under `cargo xtask ci` | exit 0 | **exit 0**, 33 steps, none skipped | `baseline-ci.txt` |
| **RED** — rename `mutants_fail_exactly_their_declared_rules` so `--list` stops naming it | non-zero | **exit 1** at step 4, *"`mutation_coverage` is missing 1 of the tests the gate names"* | `control-rename-ci.txt` |
| **mechanism** — `cargo test -- --list` with and without an `#[ignore]` | shows the difference, if there is one | **byte-identical**; `diff` exit 0 | `list-diff.txt` |
| **reachability** — a *type error* at count 3's insertion point | docs step red | **exit 101**, naming `00-prime-directives.md (line 32)` | `fence-control-typeerror.txt` |
| **lint** — count 3's snippet under plain `rustc` | warns | **`#[warn(non_snake_case)]` … on by default**, exit 0 | `lint-control.txt` |

The RED control is what makes the greens below mean something: the same gate,
the same worktree, one different edit, and it fails at exactly the step whose
job it is.

## Count 1 — silencing the named proof tests

`xtask/src/proof.rs`'s `ARTEFACTS` names **31 tests across 9 targets** and
asserts each out of `cargo test -- --list` before running it. Six places in the
gate's own source and one constitution atom say that closes the `#[ignore]`
hole.

**It does not.** The answer, in one line: **31 of 31 — but only with the
spelling clippy tells you to use.**

| arm | gate result | raw |
| --- | --- | --- |
| all 31 with bare `#[ignore]` | **red**, exit 1 at step 2 (clippy), 31 × `error: #[ignore] without reason` | `ignore-all-ignore-ci.txt` |
| all 31 with `#[ignore = "…"]` | **green, exit 0**, all 33 steps | `ignore-all-ignore-reason-ci.txt` |
| each of the 31 alone, under `cargo xtask proof-artefact` | **31 of 31 exit 0** | `ignore-each.tsv` |
| one `wasm32` name under `cargo xtask wasm-conformance` | **green, exit 0**; runner listed 81, ran 80, reported 1 ignored | `wasm-ignore-conformance.txt` |

The full per-target reading is in [`results/gate-vacuity.md`](results/gate-vacuity.md).
The sentence to take away is the one the green arm's own log prints: **four of
the nine** proof artefacts ran **zero tests** while the step reported *"N named
tests present"* about each of them and exited 0.

## Count 2 — the seven substrings that switch off a clause's rule-name check

`spec_trace.rs`'s check 4 asks *does the rule this clause names exist?* and
abstains on any clause whose `Rule:` text contains one of seven substrings.
Deleting all seven — and **only** from `schedules_new`, leaving `elsewhere`
computing exactly what it computed before, so the delta is attributable to check
4 and to nothing else — takes `cargo xtask spec-trace` from *"no problems found"*
to **45 unresolved rule names across 28 clauses**.

| family | clauses | rule names |
| --- | ---: | ---: |
| ES | 10 | 12 |
| VT | 11 | 24 |
| PS | 6 | 6 |
| WF | 1 | 3 |
| **total** | **28** | **45** |

Nineteen of the 28 are `[FROZEN]`. Three of the seven substrings are **inert at
this commit** — they guard nothing. The per-term attribution, the maturity
markers and the four names that are not rule names at all are in
[`results/spec-trace-guard.md`](results/spec-trace-guard.md).

## Count 3 — a warn-by-default rustc lint inside a constitution fence

`xtask/src/constitution.rs:31` states that `RUSTDOCFLAGS=-D warnings` recovers
rustc's default-on lints inside a doctest, *"a probe confirmed `non_snake_case`
fails the build under it"*. `xtask/src/narrative.rs:42-54` records a
re-measurement on the pinned 1.97.1 saying the opposite, for the narrative tree.

Re-run against the **constitution** corpus, with both controls above firing:

| arm | result |
| --- | --- |
| `RUSTDOCFLAGS=-D warnings cargo test --locked -p xtask --doc` | **exit 0**, 62 doctests pass |
| the whole `cargo xtask ci` | **exit 0**, 33 steps |

`constitution.rs:31` is false on this toolchain and `narrative.rs` is right; the
detail is in [`results/constitution-fence.md`](results/constitution-fence.md).

## Two more numbers, taken because they are the same shape of question

**ES-13's regression pin has never been compiled**, read for free out of the
green control's own log: `Doc-tests happenstance_core` runs 33 doctests, all
from `src/`, and `escaping_cases_need_the_query_to_outlive_the_stream` — the
` ```compile_fail ` fence in `crates/happenstance-core/tests/frozen_signatures.rs:221`
— appears in none of them. Cargo collects doctests from library targets only.
Detail in [`results/gate-vacuity.md`](results/gate-vacuity.md).



**CF-37 is `[FROZEN]`**: *every E2E case MUST name the clause or clauses it
exercises.* Over the pinned documents (`cases.py`, no build):

* **58** `### E2E-nn` cases; **0** carry a `Clauses:` field of any kind.
* **54 of 58** case bodies contain no clause identifier anywhere.
* **58 of 58** are claimed by at least one clause's `Cases:` line — **0
  orphans**, once `field_line`'s continuation-line joining is reproduced. A
  first cut of `cases.py` read the marker line alone and reported four orphans
  that are not orphans; that is recorded here because it is the same class of
  error the counts above are about.

## Files

```
run.sh        the driver. phases: baseline control mechanism ignore-all
              ignore-each wasm guard fence cases   (default: all, in that order)
mutate.py     the only thing that edits source. six edits, CRLF-preserving.
cases.py      the two text counts over spec/. no worktree, no build.
names.txt     the 31 names, transcribed from proof.rs's ARTEFACTS.
results/raw/  untouched command output. every table below is read off it.
results/*.md  the tables, written by hand from results/raw/.
```
