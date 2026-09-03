# Count 1 — how many of the gate's named proof tests can be silenced

Read off `results/raw/`. Every row is an exit status from a command in
`run.sh`; nothing here is a duration.

## The mechanism, before the counts

`xtask/src/proof.rs:29` states the argument the whole file rests on:

> So the names are asserted, out of `--list`, before the tests run.

`results/raw/list-diff.txt` is empty. libtest's discovery output for
`projection_harness_parity` is **byte-identical** with and without an
`#[ignore]` on `no_harness_lists_a_rule_by_hand`:

```
projection_harness_parity::each_harness_invokes_the_suite_exactly_once: test
projection_harness_parity::no_harness_lists_a_rule_by_hand: test
```

An assertion over that listing cannot observe the attribute. Everything below
follows from this one fact.

## The two spellings

| spelling | `cargo xtask ci` | where it stopped |
| --- | --- | --- |
| `#[ignore]` on all 31 | **exit 1** | step 2, `clippy (all targets, all features)` — 31 × `error: #[ignore] without reason` |
| `#[ignore = "…"]` on all 31 | **exit 0** | ran all 33 steps |

The refusal is `clippy::ignore_without_reason`, a **pedantic** lint this
workspace enables at `Cargo.toml:168` (`pedantic = { level = "warn", priority =
-1 }`) and `-D warnings` promotes to an error. Nothing in the gate's own
documentation mentions it, and it is not what any of the seven places below
claims is doing the work. Clippy's diagnostic ends:

```
= help: add a reason with `= ".."`
```

so the spelling that survives is the one the tool tells the author to reach for,
in the same keystroke.

## The green arm, per artefact

`#[ignore = "…"]` on all 31 names, under the gate's own `proof-artefact` step
(`results/raw/ignore-all-ignore-reason-ci.txt`). The left column is what the
step **printed**; the right is what it **ran**.

| package / target | named | reported | ran |
| --- | ---: | --- | --- |
| `happenstance-testkit` / `mutation_coverage` | 8 | `8 named tests present, 80 registry rows` | 2 passed, **8 ignored** |
| `happenstance-testkit` / `projection_mutation_coverage` | 7 | `7 named tests present, 21 registry rows` | **0 passed**, 7 ignored |
| `happenstance-testkit` / `projection_harness_parity` | 2 | `2 named tests present` | **0 passed**, 2 ignored |
| `happenstance-core` / `wire` | 3 | `3 named tests present` | 16 passed, 3 ignored |
| `happenstance-sync` / `wire` | 2 | `2 named tests present` | **0 passed**, 2 ignored |
| `course-subscriptions` / `runs` | 2 | `2 named tests present` | 8 passed, 2 ignored |
| `course-subscriptions` / `ui` | 2 | `2 named tests present` | **0 passed**, 2 ignored |
| `transfers-on-sqlite` / `runs` | 3 | `3 named tests present` | 7 passed, 3 ignored |
| `transfers-on-sqlite` / `contention` | 2 | `2 named tests present` | 1 passed, 2 ignored |

**Four of the nine proof artefacts ran zero tests**, and the step said
*"N named tests present"* about each of them and exited 0. Among the four:
`projection_harness_parity`, whose own entry in `proof.rs:217` says it is listed
precisely because *"a guard nothing holds can be `#[ignore]`d by the same change
that would have failed it"*; and `course-subscriptions/ui`, the `trybuild`
compile-fail pair whose whole value is that it can fail.

## Per name

`run.sh ignore-each`: one name at a time, `#[ignore = "…"]`, then `cargo run -p
xtask -- proof-artefact` — the exact command `REQUIRED`'s fourth step runs.
`results/raw/ignore-each.tsv`, one row per name, exit status in the last column.

**31 of 31 exit 0.** No name is an exception.

```
happenstance-testkit  mutation_coverage             every_rule_has_a_mutant                              0
happenstance-testkit  mutation_coverage             mutant_registry_is_exhaustive                        0
happenstance-testkit  mutation_coverage             mutants_fail_exactly_their_declared_rules            0
happenstance-testkit  mutation_coverage             every_mutant_states_its_provenance                   0
happenstance-testkit  mutation_coverage             conformant_variants_pass_everything                  0
happenstance-testkit  mutation_coverage             capability_skips_are_reported                        0
happenstance-testkit  mutation_coverage             the_model_rule_rejects_exactly_what_it_claims        0
happenstance-testkit  mutation_coverage             the_concurrency_rules_reject_exactly_what_they_claim 0
happenstance-testkit  projection_mutation_coverage  every_projection_rule_has_a_mutant                   0
happenstance-testkit  projection_mutation_coverage  projection_mutant_registry_is_exhaustive             0
happenstance-testkit  projection_mutation_coverage  projection_mutants_fail_exactly_their_declared_rules 0
happenstance-testkit  projection_mutation_coverage  every_projection_mutant_states_its_provenance        0
happenstance-testkit  projection_mutation_coverage  projection_conformant_variants_pass_everything       0
happenstance-testkit  projection_mutation_coverage  a_batch_with_no_read_path_is_reported_as_a_skip      0
happenstance-testkit  projection_mutation_coverage  the_second_batch_shape_answers_every_rule_with_a_pass 0
happenstance-testkit  projection_harness_parity     no_harness_lists_a_rule_by_hand                      0
happenstance-testkit  projection_harness_parity     each_harness_invokes_the_suite_exactly_once          0
happenstance-core     wire   negative_controls::length_checked_event_wire_rejects_a_65_kib_payload       0
happenstance-core     wire   negative_controls::option_shaped_query_is_indistinguishable_from_none       0
happenstance-core     wire   negative_controls::skipped_event_wire_fails_the_postcard_round_trip         0
happenstance-sync     wire   rejects_an_unknown_format_version                                          0
happenstance-sync     wire   version_is_readable_before_the_message                                     0
course-subscriptions  runs   the_binary_completes_the_dcb_cycle                                         0
course-subscriptions  runs   the_transcript_is_the_designed_composition                                 0
course-subscriptions  ui     an_unhandled_variant_fails_to_compile                                      0
course-subscriptions  ui     the_negative_control_compiles                                              0
transfers-on-sqlite   runs   the_example_runs_end_to_end                                                0
transfers-on-sqlite   runs   the_balances_survive_the_reopen                                            0
transfers-on-sqlite   runs   the_checkpoint_survives_the_reopen                                         0
transfers-on-sqlite   contention  a_contended_commit_retries_rather_than_failing                        0
transfers-on-sqlite   contention  no_update_is_lost_under_contention                                    0
```

## The wasm32 rows, because three places say the runner catches it

`proof.rs:1726` (`wasm_enumeration`'s stated limit), `proof.rs:2408`
(`WasmUnitTarget::tests`) and `main.rs:403` all say an `#[ignore]` on a wasm32
target is caught by the *runner*, in `wasm_run` / `wasm_unit_run`, rather than
by the enumeration. `main.rs:403`, in full:

> Its stated limit: it cannot say the rules *passed*, and it cannot see an
> `#[ignore]` on a macro-generated test. Both need the runner, and both are the
> step below's.

`#[ignore = "…"]` on `wasm_tests::the_probe_is_not_vacuous`, then the step below
— `cargo run -p xtask -- wasm-conformance`, `results/raw/wasm-ignore-conformance.txt`:

```
happenstance-cloudflare/--lib: 81 tests listed, 16 named, executing on wasm32-unknown-unknown
test wasm_tests::the_probe_is_not_vacuous ... ignored, measured by experiments/gate-vacuity
test result: ok. 80 passed; 0 failed; 1 ignored; 0 filtered out
EXIT=0
```

The runner enumerates the ignored test, prints it, skips it, and exits 0. It is
not a second line of defence; it is the same line of defence.

## A related zero, free from the same log

Finding F1-04 says ES-13's regression pin — the ` ```compile_fail ` fence on
`escaping_cases_need_the_query_to_outlive_the_stream` in
`crates/happenstance-core/tests/frozen_signatures.rs:221` — has never been
compiled, because cargo collects doctests from library targets only. That needs
no new run: `results/raw/baseline-ci.txt` is the whole `cargo test --locked
--workspace --all-features` from the green control.

```console
$ grep -n escaping_cases_need_the_query_to_outlive_the_stream baseline-ci.txt
(no output)
```

`Doc-tests happenstance_core` (`:5374`) runs **33 tests, every one of them from
`src/`**, and contains exactly **one** `- compile fail` line —
`event.rs - event::EventType::from_static (line 95)`. The fence in `tests/` is
in none of them. The target itself builds and runs
(`Running tests\frozen_signatures.rs`, `:1306`); its doctest does not exist.

That is the same shape as the counts above, one level down: a target that is
compiled, run, green, and carrying an artefact nothing compiles.

## What the gate says about this, in seven places

| where | what it says |
| --- | --- |
| `xtask/src/proof.rs:24-31` | `cargo test` exits 0 on `running 0 tests`, and so does a target whose tests were "renamed or `#[ignore]`d" — **"So the names are asserted, out of `--list`, before the tests run."** |
| `xtask/src/proof.rs:217` | the parity guard is listed because "a guard nothing holds can be `#[ignore]`d by the same change that would have failed it" |
| `xtask/src/proof.rs:1726` | the runner-free enumeration "cannot claim … that any individual rule is not `#[ignore]`d. Those need the runner, and `wasm_run` is where they are checked" |
| `xtask/src/proof.rs:2050` | a unit test's own message: a target in no `ARTEFACTS` row can have its names "renamed, `#[ignore]`d or emptied in silence" |
| `xtask/src/proof.rs:2408` | the `--list` assertion "is what makes a deleted, renamed, `#[ignore]`d or `cfg`-ed-away probe fail the gate rather than pass it" |
| `xtask/src/main.rs:403` | quoted above |
| `standards/rust/81-checks-that-cannot-be-types.md:322-323` (**RS-81-4**) | **Rejects.** "A proof artefact truncated to its `#![cfg(…)]` attributes, or whose meta-tests have been renamed or marked `#[ignore]`." |

The atom is the one that matters most, because a constitution atom sits above
`CLAUDE.md` on the precedence ladder and the house rule inside an atom is that
the compiled example beats the prose. RS-81-4's compiled example
(`:276-303`) models exactly two listings — the full one, and `""`, the target
emptied to its attributes (`:299-301`):

```rust
    // The target emptied to its attributes. `cargo test` exits 0 over this.
    assert_eq!(missing("").len(), 2);
```

There is no third case in it, and the third case is the one the `Rejects:` line
claims. The rename half of that line is true and the control above demonstrates
it; the `#[ignore]` half is not.
