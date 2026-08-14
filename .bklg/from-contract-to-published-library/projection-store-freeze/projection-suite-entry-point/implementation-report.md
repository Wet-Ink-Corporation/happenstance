---
item: "HS-S0007"
stage: implement
created: "2026-08-14"
updated: "2026-08-14"
---

# Implementation Report — projection_store_conformance!, one enumeration, one test per rule

> **STATUS: ten of ten ACs satisfied.** The `ProjectionStore` port has a
> conformance suite for the first time: one line in an adapter's own `tests/`
> expands to one test per projection rule, on three runtimes, through a rule set
> written in exactly one place.
>
> **A green run here proves the machinery runs, not that it discriminates.**
> `MemoryProjectionStore` is the oracle and is supposed to pass. That the suite
> can *fail* a wrong store needs `CheckpointOnlyStore`, which is
> `projection-mutant-registry`'s (HS-S0008) — the next story in merge order and
> this story's direct dependant. Every sentence in this report that could be read
> as "the projection suite passes" is qualified for that reason
> (`.kb/decisions/0010-the-suite-must-prove-itself.md`).

The three preconditions EC-013 makes a halt condition were checked before the
first edit and all three held: `crates/happenstance-core/src/projection.rs:389`
carries `type Batch;` with no lifetime and `:399` a `begin` that is neither
`async` nor fallible; `ProjectionProbe` exists at `:527-578` behind
`feature = "conformance"` with all four members and `READS_THROUGH_BATCH`; and
`MemoryProjectionStore` implements it at
`crates/happenstance-core/src/projection_memory.rs:364-384`. Nothing under
`crates/happenstance-core/**` was touched by this story.

EC-011 (an edit to the event-store emitters or an existing harness) and EC-012
(a rule unwritable through `ProjectionProbe` alone) **did not fire**. Note 4's
option 2 was taken as the spec's Context pack recorded it, and the five existing
harnesses compile untouched.

## TDD Evidence

RED first, in the order the story's implementation notes set out. The harness
files were written before any of the machinery they name, because for a library
the `tests/` directory *is* the reachability assertion: a `tests/` file can name
only public items, so a harness that compiles is proof the entry point is
mounted at the crate root and a harness that does not is the failing test.

**RED — AC-001, AC-007** (`cargo test -p happenstance-testkit --test projection_conformance`):

```text
error[E0432]: unresolved import `happenstance_testkit::fixtures::MemoryProjectionFixture`
  --> crates\happenstance-testkit\tests\projection_conformance.rs:27:5
   |
27 | use happenstance_testkit::fixtures::MemoryProjectionFixture;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `MemoryProjectionFixture` in `fixtures`

error[E0433]: cannot find `projection_store_conformance` in `happenstance_testkit`
  --> crates\happenstance-testkit\tests\projection_conformance.rs:29:23
   |
29 | happenstance_testkit::projection_store_conformance!(MemoryProjectionFixture::new());
   |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ could not find `projection_store_conformance` in `happenstance_testkit`
```

Both errors are the *stated* verification of their ACs rather than an incidental
compile failure: AC-001 says a macro "unreachable from the crate root … does not
satisfy this", and AC-007 says "a fixture reachable only from inside the
testkit's own tests fails this".

**GREEN — AC-001, AC-005, AC-006:**

```text
running 2 tests
test projection_conformance::commit_advances_the_checkpoint ... ok
test projection_conformance::commit_is_atomic_with_the_read_model ... ok
```

**GREEN — AC-003, AC-008** (the blocking harness, same enumeration, no runtime):

```text
running 2 tests
test projection_conformance_blocking::commit_is_atomic_with_the_read_model ... ok
test projection_conformance_blocking::commit_advances_the_checkpoint ... ok
```

**AC-004 — the deliberate-orphan transcript.** A meta-test that has never been
seen to fail is decorative, so it was made to fail. `pub async fn
deliberately_unregistered_rule` was added to `projection::rules` and left out of
the enumeration:

```text
running 1 test
test projection::no_orphan_projection_rules ... FAILED

---- projection::no_orphan_projection_rules stdout ----
thread 'projection::no_orphan_projection_rules' panicked at
crates\happenstance-testkit\src\projection.rs:412:5:
these rules exist in `projection::rules` but are absent from
`for_each_projection_store_rule!`, so no harness runs them:
["deliberately_unregistered_rule"]
```

It failed **by name**, naming the orphan and saying no harness runs it. The
injection was then reverted and the three unit tests re-run green:

```text
running 3 tests
test projection::no_orphan_projection_rules ... ok
test projection::two_opens_make_two_isolated_stores ... ok
test registry::no_orphan_rules ... ok
```

The opposite direction (a registered rule the module no longer declares) is free
by construction — every emitter expands to `$crate::projection::rules::$name`,
so it is `error[E0425]` in every harness — and the empty-scan direction is
fail-loud: the second assertion reports every registered rule as missing.

**AC-002** — `two_opens_make_two_isolated_stores` was written against the
fixture trait and passes; it carries a control assertion that the commit landed
in the first store, without which the isolation assertions would hold for a
fixture whose `commit` silently did nothing.

## Commits

`feat(projection-store-freeze): The projection suite entry point` — one
checkpoint commit, the first of slice `projection-conformance-suite`, on
`initiative/from-contract-to-published-library`, immediately after `5fd62c6`
(`MemoryProjectionStore, the oracle`) and immediately before this slice's second
story, `projection-capability-skips`.

Named by subject and by predecessor rather than by hash, for the reason
`memory-projection-store`'s report gives: this file is committed *inside* the
commit it describes, so no hash it quoted could survive being written into it.
`git log --grep "Story: projection-store-freeze/projection-suite-entry-point"`
resolves it.

## Changes

| File | Shape of the change |
| --- | --- |
| `crates/happenstance-testkit/src/contract.rs` | **New: `ProjectionFixture`** (`:443-468`) — owned `type Store: ProjectionProbe`, desugared `fn connect(&self) -> impl Future`, no capability constants, no GAT. Module doc gains a *Two fixture traits, one contract* section saying why it is a second trait rather than a second associated type. `Capability` and `RuleOutcome` are **not** touched: the whole diff of this file is the new trait, its doc, one import and one rustdoc-link fix. |
| `crates/happenstance-testkit/src/projection.rs` | **New file, the fourth rule family.** `pub mod rules` with the two baseline rules and three non-`pub` helpers; `for_each_projection_store_rule!`; `__emit_projection_tokio` / `_blocking` / `_wasm`; `no_orphan_projection_rules`; `two_opens_make_two_isolated_stores`. |
| `crates/happenstance-testkit/src/fixtures.rs` | **New: `MemoryProjectionHandle`** (a delegating newtype over `Arc<MemoryProjectionStore>` implementing `SendProjectionStore` + `ProjectionProbe`) and **`MemoryProjectionFixture`**, both published items with rustdoc. |
| `crates/happenstance-testkit/src/lib.rs` | The mount: `pub mod projection;` (unconditional, with the reason), `ProjectionFixture` in the public re-export block, `projection_store_conformance!` with its three arms, `__private` gaining the trait, and the two family-scoped crate-doc sections moving from three families to four. |
| `crates/happenstance-testkit/Cargo.toml` | `happenstance-core`'s dependency gains `conformance`, with a comment on why it is unconditional. No new dependency, no new feature of the testkit's own. |
| `crates/happenstance-testkit/README.md` | A *The projection suite* section, and the status note stops saying the port "has no suite at all". |
| `crates/happenstance-testkit/tests/projection_conformance{,_blocking,_wasm}.rs` | Three harnesses, one macro invocation each, zero rule names. |
| `xtask/src/spec_trace.rs` | `RULE_FILES` becomes four entries with its doc restated; `BARE_NAME_MAP` gains `projection.rs` (see Notes). |
| `CHANGELOG.md` | Three `[Unreleased]` entries: the family and its four macros, and one per rule naming the defect it detects. |
| `spec/SPECIFICATION.md` | The generated §7.1–§7.2 region only, via `cargo xtask spec-trace --write`: four rows lose the `†` that meant "does not exist yet". |
| `standards/rust/*.md` (11 files) | Line numbers in 23 `file:line` citations, repaired after the cited files moved. See Notes. |

## Gates

Every command below was run from the worktree root at the end of the story.

| Command | Result |
| --- | --- |
| `cargo test -p happenstance-testkit --test projection_conformance` | 2 passed |
| `cargo test -p happenstance-testkit --test projection_conformance_blocking` | 2 passed |
| `cargo test -p happenstance-testkit --lib` | 3 passed (`no_orphan_projection_rules`, `two_opens_make_two_isolated_stores`, `registry::no_orphan_rules`) |
| `cargo xtask affected --base main` | **`affected gate passed`** |
| `cargo xtask ci --fast` | **`all required checks passed (--fast: 4 optional step(s) not run)`** — including all four `wasm32` steps, `spec-trace`, the CF-6/CF-29/CF-33 lints, `lint-constitution` and the three doc builds |
| `cargo xtask spec-trace` *(run explicitly, AC-010)* | `traceability: no problems found; §7.1–§7.2 matches the checker` |
| `cargo check --locked -p happenstance-testkit --tests --target wasm32-unknown-unknown` *(run explicitly, AC-008)* | `Finished dev profile` |
| `cargo fmt --all --check` | clean |

The two `UNCLAIMED_PENDING_ADR` rules `spec-trace` still reports —
`k_disjoint_boundaries_admit_exactly_k_commits` and `ops_agree_with_the_model` —
are pre-existing and unrelated; neither projection rule joined them, which is
AC-010's substantive result.

## Notes

**Two deviations from the plan, both reported rather than absorbed.**

1. **`BARE_NAME_MAP` needed an entry, and the spec did not predict it.** Naming
   the family's module `projection.rs` — a name the spec settled deliberately —
   made that basename ambiguous in the workspace for the first time, and 13
   citations in `SPECIFICATION.md` that had resolved uniquely for phases failed
   at once. The remedy is the one `spec_trace.rs` documents for exactly this
   case: one `BARE_NAME_MAP` entry with the evidence for which file is meant
   (`xtask/src/spec_trace.rs:2056-2069`). Every one of the 13 predates this file
   and names the **port** — the GAT that used to be at `:97-99`, `ProjectionId`,
   `rollback`'s declaration — and the anchor check is a real backstop here
   rather than a formality, because the two files overlap in length. The
   alternative was renaming the module away from the name the spec settled,
   which would have made the ledger's `verifying_test` paths wrong for a
   cosmetic reason.

2. **23 `standards/rust/` citations were repaired.** Inserting a ~100-line trait
   into `contract.rs` and a macro into `lib.rs` moved every line below them, and
   `lint-constitution` anchors its citations with a needle and ten lines of
   slack. `--write` repairs the router but not citations, so each was recomputed
   from its own needle (the needle is unchanged in every case; only the integer
   after the colon moved). This is outside the letter of the PR boundary and is
   named here rather than left for a reviewer to find: the gate step
   `the Rust constitution is internally consistent` is red without it, and no
   alternative exists that does not weaken a check.

**Three things deliberately *not* done**, each named in the spec's boundary:

- **No capability constant on `ProjectionFixture`.** The two baseline rules gate
  on none, and a constant no rule reads is the decorative shape CLAUDE.md's
  corollary names. The set is `_design.md`'s and lands with
  `projection-capability-skips` — the slice-mate implemented immediately after
  this story, in the same context.
- **No third rule**, however cheap. Each rule landed here carries a documented
  debt (its mutant is one story away) and the bound on that debt is the rule
  count.
- **No `require!`/`must!` reuse yet**, because there is nothing to gate.

**One finding, out of boundary, reported rather than fixed.** The model and
concurrency families still have **no orphan meta-test** — `no_orphan_rules`
covers `suite.rs` alone and `no_orphan_projection_rules` covers `projection.rs`,
so two of the four families have a rule set nothing checks for orphans. Writing
the projection sibling made the gap obvious. Extending CF-24's coverage to all
four families is a decision someone should take deliberately, with its own
reasoning; it is not a side effect of this story.

**One measured surprise worth recording.** `cargo xtask ci --fast` does *not*
omit `spec-trace` — the step is in `REQUIRED`, not `OPTIONAL`
(`xtask/src/main.rs:315-329`), and `run_fast` drops `OPTIONAL` only. The spec
says twice that `--fast` omits it and that a story-grain green therefore hides
AC-010. That is no longer true; the advice to run it explicitly was followed
anyway, and both AC-008's and AC-010's commands were run standalone.
