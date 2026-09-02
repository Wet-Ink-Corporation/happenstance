---
item: "HS-S0017"
stage: report
created: "2026-08-15"
updated: "2026-08-15"
---

# Report — One clean cargo xtask ci run, and the proof artefact recorded

## Findings Ledger

**Verdict: ten of ten ACs satisfied. Nothing blocked, nothing deferred, nothing
stubbed.** The recorded run is one whole `cargo xtask ci` — not `--fast` — at
`7620481010f7bc7a3eb50b241b1dbbea77aaa68c`, exit 0, on a tree with no uncommitted
changes, with **four of four `OPTIONAL` steps run and none skipped**.

### AC by AC

| AC | result | what proves it | where it is mounted |
| --- | --- | --- | --- |
| **AC-001** | **Met** | The run itself: exit 0, `git status --porcelain` empty before and after, wall clock and toolchain recorded, `7620481` a descendant of `984e7fd` and `d6496cd`. Skeletons compiled with every affected body still `todo!()`. | `references/evaluation/phase-6-projection-proof.md` §Provenance |
| **AC-002** | **Met** | Two names, and the document says which is which: the rule `commit_is_atomic_with_the_read_model` and the meta-test `projection_mutation_coverage::projection_mutants_fail_exactly_their_declared_rules`, with two pasteable `--locked` commands. `proof::tests::the_artefact_names_both_tests_and_the_mutant` (red → green). | `xtask/src/proof.rs` `ARTEFACTS` + the artefact §1 |
| **AC-003** | **Met** | `MemoryProjectionFixture` and `BufferingProjectionFixture`, how each was observed, and that both were observed in the **same** invocation. `proof::tests::the_artefact_names_both_batch_shapes` (red → green). | the artefact §2 |
| **AC-004** | **Met** | Two `ARTEFACTS` rows, the seven names transcribed from `-- --list`. Negative control: a renamed meta-test made the step bail *before running anything* with `is missing 1 of the tests the gate names`. Still a subset check. | `xtask/src/proof.rs` `ARTEFACTS` |
| **AC-005** | **Met** | `Artefact::registry` keys the count to the artefact; `registry_len(file)` keeps its three error paths for both. Gate stdout: `79 registry rows` beside `mutation_coverage`, `19` beside `projection_mutation_coverage`. `proof::tests::a_registry_count_belongs_to_the_target_it_is_printed_beside` asserts over **all** artefacts (red: `found 1`). | `xtask/src/proof.rs` |
| **AC-006** | **Met** | `projection_harness_parity.rs`, two tests, three negative controls run and reverted — a hand-listed rule, a missing invocation, a doubled invocation — each failing by name. Its own names are an `ARTEFACTS` row. | `crates/happenstance-testkit/tests/projection_harness_parity.rs` + `xtask/src/proof.rs` |
| **AC-007** | **Met** | One row per `OPTIONAL` step with its probe and outcome; **four ran, zero skipped** (`grep -c '^skipped: ' = 0` over the run log, 25 `=== step ===` lines). Both powersets stated as the only steps that would catch `conformance` coupled to `memory`. | the artefact §4 |
| **AC-008** | **Met** | Limits section names the MSRV job, the `conformance on wasm32` job, PS-2's unmet bar, the CI-side advisories and the registry's durability limit; no verdict of any kind and no ratio over the mutant set. Two assertions hold it, one of which fired during authoring. | the artefact §5–§6 |
| **AC-009** | **Met** | Registered in *Later additions, which are neither* with its date, its pinned commit and its supersede-rather-than-edit lifecycle, following `review-citation-drift.md`'s precedent. | `references/evaluation/README.md:114-138` |
| **AC-010** | **Met** | One `[Unreleased]` entry naming what the gate now holds and that harness parity is enforced by a test; no rule names (CF-29's scope), no implied version. | `CHANGELOG.md:1273-1290` |

### The defect this story made reachable, and fixed

`check()` selected the registry row-count by **package** (`if package ==
REGISTRY_PACKAGE`) and `registry_len` read one hard-coded path. A second
`happenstance-testkit` row would therefore have printed the event-store registry's
`79` beside the projection target — a number that is not about that target at all,
and the same *"a number quoted in two documents and computed nowhere"* failure
`registry_len`'s own doc comment was written about, one level up. Keyed to the
artefact instead; the event-store row's count and its missing-registry error path
are both unchanged, which AC-005's test asserts by sweeping every artefact rather
than only the new ones.

### Deferred, routed, and owned elsewhere

Nothing is deferred. **EC-007 did not fire**: the run revealed no defect owned by
an earlier story, so nothing was reported against one and nothing was repaired
outside this story's boundary.

Three claims are attributed elsewhere rather than made here, which is AC-008's
whole content:

| Claim | Whose |
| --- | --- |
| `wasm32` **execution** of the projection suite | CI's `conformance on wasm32` job; the local gate only type-checks |
| The MSRV | CI's `minimum supported Rust version` job; the local gate never checks it |
| Whether the port is frozen, and whether 0.1 ships behind `unstable-projection` | `ladybug-projection-store` and `publication-and-positioning` |

### Two commits, and why

`7620481` is the mechanism and is the commit the recorded run was made on; the
evidence is the commit on top of it. A document that records a run cannot be in the
tree whose run it records, and AC-001 requires the tree to be clean. Both commits
carry the same `Story:` trailer.
