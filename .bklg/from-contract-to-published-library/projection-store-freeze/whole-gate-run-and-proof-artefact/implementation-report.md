---
item: "HS-S0017"
stage: implement
created: "2026-08-15"
updated: "2026-08-15"
---

# Implementation Report — One clean cargo xtask ci run, and the proof artefact recorded

> **STATUS: ten of ten ACs satisfied. Nothing blocked, nothing stubbed.**
> `cargo xtask ci` green, run whole, on a clean tree at
> `7620481010f7bc7a3eb50b241b1dbbea77aaa68c` — **four of four `OPTIONAL` steps
> ran, nothing skipped**.

Three things landed, and the first two are what stop the third from being a colour.

1. **The names are held by the gate.** `xtask/src/proof.rs`'s `ARTEFACTS` gained
   the projection family's two rows, so the meta-test the freeze verdict rests on
   cannot be renamed, `#[ignore]`d or emptied in silence.
2. **AC-016's second half is verified rather than assumed.**
   `crates/happenstance-testkit/tests/projection_harness_parity.rs` compares the
   enumeration's own names against all three harness sources. A hand-listed
   harness type-checks exactly as well as a generated one, so the mandatory
   `wasm32` step could never have seen this.
3. **The artefact.** `references/evaluation/phase-6-projection-proof.md` — dated,
   commit-pinned, registered in its directory's README, and written from the run's
   actual stdout rather than reconstructed.

One defect was reachable only because this story made it reachable, and it was
fixed here: `check()` selected the registry row-count by **package**, so a second
`happenstance-testkit` row would have printed the event-store registry's `79`
beside the projection target.

## TDD Evidence

| AC | Test | Red (before) | Green (after) |
| --- | --- | --- | --- |
| **AC-005** | `proof::tests::a_registry_count_belongs_to_the_target_it_is_printed_beside` | `the defect this test exists for is only reachable with two registries; found 1` — the assertion, on a tree with one registry keyed by package | passes once `Artefact::registry` exists and both rows name their own file; gate stdout shows `79` and `19` beside their own targets |
| **AC-004** | `proof::tests::the_phase_six_targets_are_held` | ``projection_mutation_coverage` is in no `ARTEFACTS` row, so its test names can be renamed, `#[ignore]`d or emptied in silence` | passes once both rows land |
| **AC-002** | `proof::tests::the_artefact_names_both_tests_and_the_mutant` | `reading references/evaluation/phase-6-projection-proof.md: The system cannot find the file specified. (os error 2)` | passes once the artefact names the rule, the mutant and the meta-test |
| **AC-003** | `proof::tests::the_artefact_names_both_batch_shapes` | same missing-document failure | passes once both fixture names are in the document |
| **AC-008** | `proof::tests::the_artefact_states_what_the_run_does_not_cover` | same missing-document failure | passes once the limits section names both CI jobs and PS-2 |
| **AC-008** | `proof::tests::the_artefact_quotes_no_pass_rate_over_the_mutant_set` | same missing-document failure, **and then a second, more interesting red**: `a pass rate over the mutant set: - **no pass rate over the mutant set, in any form.**` — it fired on the prohibition bullet itself | passes once the bullet is worded as *"no ratio over the mutant set"*; the incident is recorded **in the artefact** rather than tidied away |
| **AC-006** | `projection_harness_parity::no_harness_lists_a_rule_by_hand` | **negative control**, injected into `projection_conformance_wasm.rs` and reverted: ``projection_conformance_wasm.rs:30 names `commit_advances_the_checkpoint` `` | green on the real harnesses |
| **AC-006** | `projection_harness_parity::each_harness_invokes_the_suite_exactly_once` | **two negative controls**: commenting the invocation out gave ``projection_conformance_wasm.rs carries no `projection_store_conformance!` invocation — it compiles, it exits 0, and it covers nothing``; adding a second gave ``carries 2 … invocations; a second one beside the generated call is a hand-scoped rule set wearing a macro`` | green on the real harnesses |
| **AC-004** (EC-001) | the gate step itself | **negative control**: renaming `projection_mutants_fail_exactly_their_declared_rules` made `cargo run -p xtask -- proof-artefact` bail *before running anything* with ``is missing 1 of the tests the gate names: […]`` and exit 1 | restored; step green |

**Why three of the four guards needed a negative control rather than a red-first
file.** `projection_harness_parity.rs` asserts a property the tree already has, so
writing it produces a green test on the first run — which is exactly the shape
CLAUDE.md calls decorative unless a plausible wrong implementation is named *and
written*. So each was written, run, observed failing by name, and reverted. The
scripts are in the session scratchpad; the messages above are transcribed from
their output.

**No test was written for AC-001, AC-007, AC-009 or AC-010.** *Was the tree clean*,
*did four optional steps run*, *is the document registered*, and *does the
changelog entry stay out of CF-29's scope* are facts about a run and a diff. AC-001
and AC-007 are transcribed from the run's own stdout with the exit status and the
skip count quoted; AC-009 and AC-010 are diff review. Inventing a test that reads
`git status` would assert something about the machine it ran on.

## Commits

| SHA | What |
| --- | --- |
| `7620481` | The mechanism: `ARTEFACTS`' two phase-6 rows, `Artefact::registry` and the keying fix, the parity guard, and the `[Unreleased]` changelog entry. **This is the commit the recorded gate run was made on**, and it is why the mechanism landed first: the run has to be on a clean tree, and a document that records a run cannot be in the tree the run was made on without describing itself. |
| `__STORY2_SHA__` | The evidence: the artefact, its README registration, the four document-reading assertions in `xtask/src/proof.rs`, the ledger and these reports. |

## Changes

| File | Change |
| --- | --- |
| `xtask/src/proof.rs` | `Artefact::registry: Option<&'static str>`; `EVENT_STORE_REGISTRY` and `PROJECTION_REGISTRY` replacing the single `REGISTRY_FILE`; `registry_len(file)` taking the path and keeping all three error paths for both; `check()` selecting on the field rather than on the package; two new `ARTEFACTS` rows; six `#[cfg(test)]` assertions. |
| `crates/happenstance-testkit/tests/projection_harness_parity.rs` | New host-only target, two tests, `include_str!` over the three harnesses, identifier-boundary matching. |
| `references/evaluation/phase-6-projection-proof.md` | New. Provenance, the two test names, the two fixtures, sixteen rules across three emitters, the full ran/skipped ledger, the limits section, and the four things it deliberately does not say. |
| `references/evaluation/README.md` | One registration row in *Later additions, which are neither*. |
| `CHANGELOG.md` | One `[Unreleased]` entry. |

**`PROJECTION_META_TESTS` was transcribed, not derived.**
`cargo test --locked -p happenstance-testkit --all-features --test
projection_mutation_coverage -- --list` printed seven names; all seven are in the
row, in listing order. The target *does* wrap them in a `mod` of its own name, so
EC-008's deviation report was not needed.

**The parity guard's matcher is identifier-boundary, not substring, and the reason
is in the tree.** `projection_conformance.rs`'s module documentation names
`commit_is_atomic_with_the_read_model` in an English sentence about which mutant
fails it — which is exactly the prose the guard must keep. A hit counts only when
the characters either side are neither alphanumeric, `_`, nor a backtick; the
backtick clause is what excludes documentation, because every rule name in prose in
this repository is written `` `like_this` ``.

## Gates

`cargo xtask ci`, **run whole**, twice.

**The recorded run**, at `7620481`, on a clean tree (`git status --porcelain` empty,
verified before and after), 2026-08-15 01:14:59–01:15:59 PDT, **exit 0**:
25 steps, **zero** `skipped:` lines. Its full ledger is section 4 of the artefact;
the figures a reader will want are `201 clauses … 111 conformance rules, 58 e2e
cases, 379 citations checked`, `27 atoms, all consistent`, and the proof step's
five rows with `79` and `19` registry rows beside their own targets.

**The merge run**, on the tree this commit produces — the artefact, the README row
and the four document assertions added — also **exit 0**, 25 steps, zero skips.
That second run is what makes this story's own diff clear the gate it records; the
first is the evidence.

`redkiln verify --grain story` (HS-S0017): **pass** — affected-gate ok, ledger ok.

**NF-001 was honoured.** Both new `ARTEFACTS` rows use `cargo_args`' existing flag
set unchanged, so the step reuses the `tests` step's build artifacts; the proof
step took about two seconds of the sixty-second run.

## Notes

**The run is last, and the two-commit shape is what makes that literally true.**
AC-001 requires a clean tree; the document records a run; a document cannot be in
the tree whose run it records. So the mechanism landed as `7620481`, the gate ran
on that clean tree, and the evidence landed on top pinned to it. `7620481` is a
descendant of both dependency stories' commits, which is the other half of AC-001
and was checked rather than assumed.

**EC-007 did not fire.** The run revealed no defect owned by an earlier story: no
missing mutant, no rule asserting a literal position, no fixture that is the oracle
wearing a hat. The one defect it did reveal — the package-keyed registry count — is
`proof.rs`'s own and was in this story's boundary by construction, because this
story is the one that makes a second registry exist.

**Two clippy reds inside this story's own diff, both `doc_markdown` on the bare
word `DoD`.** Fixed by writing *Definition-of-Done* out. Recorded because it is the
NF-006 shape: a story whose diff fails the gate it is recording is self-refuting,
and both were caught by the very run the story exists to make.

**What this story did not do.** It added no conformance rule, no mutant, no fixture
and no registry row; it touched no port, no probe, no feature table and no
`lib.rs` export block; it edited no clause and no maturity marker —
`spec/SPECIFICATION.md` is not in its diff. It runs `spec-trace` and records that
it was green; making it green was HS-S0016's.
