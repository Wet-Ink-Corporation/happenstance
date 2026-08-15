---
item: "HS-S0015"
stage: implement
created: "2026-08-15"
updated: "2026-08-15"
---

# Implementation Report — DT-8's arm discharged — the suite's bar held for the author it was chosen for

> **STATUS: eight of eight ACs satisfied. Nothing blocked, nothing stubbed.**
> Arm A was read out of `_design.md`, not chosen. One new workspace member, one
> new crate-doc section, nine recorded gaps — **five of them the point of the
> story**. `cargo xtask ci` green, run whole.

The deliverable is two things and they are of unequal size. The visible half is
`examples/outside-projection-adapter/`: a projection adapter built from the
rendered documentation, passing all sixteen rules through one line of
`projection_store_conformance!`, with a wrong sibling beside it that
`commit_is_atomic_with_the_read_model` rejects by name. The half that matters more
is `_extension-surface-gaps.md`, because a fixture proves that one author got
through and only the record says where the documentation ran out for them.

The exercise's most important result is not that the fixture passed. It is that
**the port's own page tells an adapter author the conformance suite does not
exist** (G1), and that **`spec-trace` cannot see a citation rot** (G2) while
`lint-constitution`, one crate over, catches the same class of defect instantly
(G9). Neither would have been found by anything already in the tree, because
everything already in the tree is written by people who know the answer.

## TDD Evidence

Red first, and the red was real: a store that applied the read-model rows and
never touched the checkpoint. Every failure below is an assertion inside a
conformance rule naming the behaviour that was missing — no compile error, no
import error, no typo.

| AC | test | red → green |
| --- | --- | --- |
| **AC-003** | `examples/outside-projection-adapter/tests/outside_projection_conformance.rs:19` — `projection_store_conformance!(OutsideFixture::new())` | **RED: 10 of 16 rules failed.** `commit_advances_the_checkpoint`: *left `NeverRun`, right `Live { through: SequencePosition(1) }`*. `commit_is_atomic_with_the_read_model`: *row `Some(12)`, checkpoint `NeverRun`*. `commit_accepts_a_position_the_batch_did_not_write`, `commit_rejects_a_regressing_position`, `distinct_projections_advance_independently`, `dropped_batch_leaves_store_usable`, `reset_clears_rows_and_checkpoint_together`, `reset_is_not_commit_at_first`, `rebuilding_is_distinguishable_from_live`, `failed_commit_leaves_both_unchanged` likewise. **GREEN: 16 passed, 0 failed** after `commit` gained the regression check, the fault path and the checkpoint write, and `reset` gained the caller's deletes and the return to `NeverRun`. |
| **AC-004** | `.../tests/outside_projection_discrimination.rs:80` `commit_is_atomic_with_the_read_model_fails_the_checkpoint_only_store` | Passed at the **red** commit already, which is the point: the wrong store was wrong from the start and the rule said so. Its control at `:96` was **RED** at that moment — the conformant store failed the same rule — and went green with the store. A discrimination test whose control is green throughout is a test that never checked the harness. |
| **AC-004** | `.../tests/outside_projection_discrimination.rs:69` `the_checkpoint_only_store_passes_the_rule_that_watches_only_the_checkpoint` | Green throughout, deliberately. It asserts the **trap** rather than the fix: a store that never writes a row satisfies `commit_advances_the_checkpoint`. If this one ever goes red the demonstration beside it has stopped being about what it claims. |
| **AC-005** | `.../tests/outside_projection_capability_skip.rs:27` `a_declined_capability_returns_a_skip_carrying_this_fixtures_own_reason` | Green at red — the skip path never depended on the store working, which is itself worth knowing. |
| **AC-005** | `.../tests/outside_projection_capability_skip.rs:66` `a_declared_capability_runs_the_rule_it_gates` | **RED**: *a fixture declaring `COMMIT_FAULT` supported MUST arm a fault its store cannot absorb … and the commit succeeded*. **GREEN** once `commit` consulted the armed fault *after* applying the rows and put them back. The testkit refused to let a declared capability be satisfied vacuously, which is exactly what it is for. |
| **AC-005** | `.../tests/outside_projection_capability_skip.rs:76` `an_ungated_rule_runs` | **RED** with the same checkpoint failure as AC-003, **GREEN** with it. |
| **AC-002** | `cargo test --doc -p happenstance-testkit --all-features` | Deliberately broken and repaired: the new section's hidden `macro_rules! ignore` line was corrupted, the doctest went **RED**, and the repair returned it to green. That is what proves the block is collected — and, in the same breath, that the invocation inside it is never type-checked (gap G4). |
| **AC-006** | `cargo test -p outside-projection-adapter --test orphan_probe_placement` | **RED on purpose and never repaired** — `error[E0117]`. The file was deleted after its transcript was captured; the permanent artefact is the transcript. Its sibling, `error[E0432]` from naming a dev-dependency in `src/`, was produced the same way. |
| **AC-001 / AC-007** | Reviewed diff plus commit order | Not a code test, and could not be: the check is that `_design.md` was *read* and that the denylist was written *before* the fixture. Only the commit graph can show either. `546a8fe` carries the arm, the allowlist and the denylist and touches no crate. |
| **AC-008** | `cargo xtask ci` (whole) | **RED**: `lint-constitution` — *8 problem(s) in standards/rust*, each naming a phrase that had moved. **GREEN** after re-anchoring. Recorded as G9, and it is the story's best evidence for G2. |

## Commits

| sha | subject | why it is separate |
| --- | --- | --- |
| `546a8fe` | `docs(projection-store-freeze): name the reading discipline before the fixture exists` | **AC-007's check is partly this commit's existence and position.** It carries the DT-8 arm read out of `_design.md:359-363`, the allowlist and the denylist, and nothing else — no manifest, no store, no test. A denylist landing in the same commit as the fixture is indistinguishable from one written to describe it. |
| *the checkpoint* | `feat(projection-store-freeze): The outside-author extension surface` — the commit carrying the `Story: projection-store-freeze/documented-extension-surface` trailer, and the only other commit in `6ce1cf3..HEAD` | Everything else: the crate, the crate-doc section, the two fixed gaps, the record, the ledger and these reports. Its own SHA is not written here because a report inside a commit cannot name that commit; it is recorded in `links.commits` by `redkiln record-links`, which the CLI owns. |

## Changes

**The new workspace member** — `examples/outside-projection-adapter/`, `publish = false`, picked up by
`Cargo.toml:3`'s members glob with no root-manifest edit.

- `Cargo.toml` — one `[dependencies]` entry (`happenstance-core`, `features = ["conformance"]`) and
  two `[dev-dependencies]`. It deliberately declines `[workspace.dependencies]`, and says why in the
  file: the workspace entry sets `default-features = false`, which would have made `std` look like a
  second feature an outsider had to ask for and falsified the very measurement the crate exists for.
- `src/lib.rs` — `OutsideProjectionStore` (a `BTreeMap` read model and a checkpoint map under one
  `Mutex`) with **both** `impl ProjectionStore` and `impl ProjectionProbe`, because that is where the
  orphan rule puts them; `CheckpointOnlyStore` beside it with the same two impls and one line of
  defect; a hand-written `Display`/`Error` pair rather than a `thiserror` derive, so `[dependencies]`
  stays at one crate.
- `tests/support/mod.rs` — the two `ProjectionFixture` impls, which are out here because
  `happenstance-testkit` is a dev-dependency and does not exist for the library build at all.
- `tests/outside_projection_conformance.rs`, `..._discrimination.rs`, `..._capability_skip.rs` — the
  three demonstrations.

**The mount** — `crates/happenstance-testkit/src/lib.rs:175-239`, a new `#` section between
`# Where the rule set lives` and `# What is checked`: six ordered steps, the `conformance` flag named,
the orphan-rule trap named, the `__private` mechanics explained last because that is the order P2
needs them in. 64 doc lines against a budget of 70.

**The two gaps fixed in place** — `crates/happenstance-testkit/src/contract.rs:891-905` (G6:
`RuleOutcome::Skipped`'s field docs named only one of the two fixture traits and did not say the value
is the const's own identifier) and eight citations re-anchored in `standards/rust/` (G9).

**The record** — `_extension-surface-gaps.md`, ~340 lines: the arm, the allowlist, the denylist, what
was built, the counted graph, both `error` transcripts, nine numbered gaps, the stated limits, and a
closing table of what is handed to HS-S0016.

**The bookkeeping** — `CLAUDE.md:40-43` (the repository map gains the member), `CHANGELOG.md` (two
entries under `[Unreleased] / Added`, both stating that no item became `pub`), `Cargo.lock` (the new
member, so `--locked` holds).

## Gates

| gate | result |
| --- | --- |
| `cargo xtask ci` — **run whole, not `--fast`** | **`all checks passed`**. Includes the `tests` step over the new crate, the feature powerset (widened by one crate at one combination, since the example declares no features — NF-001), the wasm32 powerset, both doc builds, `spec-trace`, `lint-constitution`, `package-check`, `cargo deny` and the nightly docsrs build. |
| `cargo test -p outside-projection-adapter --no-fail-fast` | 16 + 3 + 3 passed, 0 failed. |
| `cargo clippy -p outside-projection-adapter --all-targets --all-features -- -D warnings` | Clean. No `#[async_trait]`, the bare `ProjectionStore` flavour bound, one flavour name imported per module (NF-003). |
| `cargo fmt --all --check` | Clean, run last, after every other edit. |
| `cargo run -p xtask -- lint-constitution` | `27 atoms, all consistent` — after G9's repair. |
| `redkiln verify --item HS-S0015 --grain story --base 6ce1cf3` | `affected-gate` **ok**, `boundary` **ok**, `ledger` **ok**. `--base 6ce1cf3` is this story's own base — the commit before `546a8fe`; run against the default `main` the boundary check diffs the whole branch and lists every sibling story's files, which answers a different question. `boundary` is green because the two compelled entries were **admitted into the fenced block** with the argument stated and scoped (`spec.md:256-257`, `:261-283`), not because the diff shrank. `provenance` stays red until `links.commits` carries the checkpoint sha, which `redkiln record-links` closes. |

## Notes

**This story's exact diff, against the declared boundary.** `git diff --name-only 6ce1cf3..HEAD`
gave nineteen files, of which four fell outside the block as first written. Both classes were
**compelled by entries the block already admitted**, so the block itself was amended — the repository's
own precedent (`aef8990`, after `redkiln verify` bounced `7fcb378` and `79df6b7` for the same class) is
that the boundary is widened with the argument stated and scoped, never that a prose note stands in for
the check:

| file | why the block now admits it, and what the admission does *not* cover |
| --- | --- |
| `standards/rust/41-declarative-macros.md`, `62-doctests-and-harnesses.md`, `91-adapter-authoring-recipe.md` | G9. Eight citations anchored to phrases below the mount point; `lint-constitution` — a gate step — fails without the repair, and no placement of the new section avoids it. Admitted as `standards/rust/**` at `spec.md:256`, scoped at `:261-275` to **line-number re-pointing only**, with the equal-insertions/deletions property this diff has: **+8/-8**, 4/4 + 3/3 + 1/1. Rule text, evidence selection, retirement and new atoms stay out. |
| `Cargo.lock` | EC-007 *requires* the lock file to be committed with the new member so the gate's `cargo test --locked` holds — a requirement the spec's own Data-and-migrations section already called "covered by the PR boundary" (`spec.md:365-367`) when the block did not list it. Admitted at `spec.md:257`, argued at `:277-283`, scoped to the entries the new member adds; an unrelated bump or a `cargo update` sweep is not in boundary. |

`redkiln verify --item HS-S0015 --grain story --base 6ce1cf3` now reports `boundary` **green** over
that amended block. Against the default `main` base it reports the whole branch — every sibling
story's files included — which is a different question and not this story's boundary.

**One deviation, and it is a boundary widening that landed in the boundary.** The block as planned
named `crates/happenstance-testkit/src/lib.rs` as a mount point but did not name what cites *into*
it. Adding any section to that crate doc shifts the lines below it, and eight citations in
`standards/rust/` are anchored to phrases there; `lint-constitution` failed the gate on all eight.
The repair is `+65` on eight line numbers. It could not be avoided by placement — appending the
section to the end of the `//!` block shifts the same eight lines, and is the anti-pattern AC-002
names. Recorded as G9, argued in the fenced block where the check reads it, and cited from the
ledger — a widening recorded only in prose is a widening the gate cannot enforce.

**Three things the plan expected that did not happen, all recorded as gap G7 rather than passed
over.** EC-003 did not fire — the `__private` re-export carried the foreign expansion first time.
AC-005's "gap #1" did not fire — `happenstance_testkit::projection::rules` is already a public path.
So **no testkit item became `pub`**, CF-32's MINOR event does not arise, and the testkit's version does
not move. A record that only lists what went wrong is a record that cannot be trusted about what went
right.

**One thing the plan expected that did happen.** EC-005: the exactness meta-tests are unreachable
from a foreign crate, so `_design.md:388-395`'s obligation is **half discharged** — the capability-skip
rule is cleared, the mutant-registry exactness check is not, and cannot be by anyone outside this
workspace. That is G5, reported to HS-S0016. Neither forbidden remedy was taken.

**Two capabilities were declared rather than declined, deliberately.** The reference fixture declines
both `COMMIT_FAULT` and `RESET_REFUSAL`, so `failed_commit_leaves_both_unchanged` had never run
outside the mutant harness. This fixture declares `COMMIT_FAULT` and supplies a store that half-applies
a commit and rolls it back, so PS-1's second conjunct is now exercised by an ordinary adapter. It
declines `RESET_REFUSAL`, which is AC-005's one skip.

**What was not done.** No `_design.md` edit, no `.kb/` atom, no `spec/SPECIFICATION.md` edit, no rule
added or changed, no mutant registered, no port signature touched, and no registry re-implemented in
the example crate. Initiative DoD 9 — a stranger installing from the registry — is explicitly **not**
claimed; a workspace member depends by path, and the record says so under *The stated limits of the
exercise*.
