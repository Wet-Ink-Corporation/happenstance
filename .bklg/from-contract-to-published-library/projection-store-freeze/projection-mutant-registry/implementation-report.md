---
item: "HS-S0009"
stage: implement
created: "2026-08-14"
updated: "2026-08-14"
---

# Implementation Report — CheckpointOnlyStore fails the suite by name

> **STATUS: eleven of eleven ACs satisfied.** The projection suite can now
> **fail** a store. `CheckpointOnlyStore` — a real `ProjectionStore` +
> `ProjectionProbe` + `ProjectionFixture` that commits the checkpoint and
> discards the write set — is driven through `commit_is_atomic_with_the_read_model`
> and rejected by it, at that rule's both-present-or-both-absent assertion and
> nowhere else. Project DoD 1's first half is observable, by name, from a
> `cargo test` line a reviewer can paste.
>
> **What a green run in this binary means, stated before anything else.** It means
> two named defects are caught and that neither mutant is broken in more ways than
> it claims. It does **not** mean the projection suite is strong: seven of §4.11's
> seventeen rules are still owed, CF-5's conformant variant is deferred by name to
> `buffering-conformant-variant`, and no fraction is quoted anywhere over the
> mutant set — the denominator is an author's choice, and a fraction reports how
> representative the author was while reading as though it reported how good the
> suite is (ADR-0010 §1).

EC-013's halt condition was checked before the first edit and did not obtain:
`for_each_projection_store_rule!` exists at
`crates/happenstance-testkit/src/projection.rs:326-337` and takes a raw
token-tree callback, `__emit_rule_names` resolves through it (the event-store
registry already uses the pair at `tests/mutation_coverage.rs:2196-2200`), and
`ProjectionFixture` carries `SECOND_HANDLE` and `RESET_REFUSAL` at
`crates/happenstance-testkit/src/contract.rs:519-596`. No second rule list was
written; `all_projection_rules()` takes the universe from the one enumeration.

## TDD Evidence

Two reds, both for the behaviour they name and neither a compile or import
failure. The order is the spec's: the registry and the four meta-tests landed
*before* the second mutant, so the first red seen was CF-1 demanding a store for
`commit_advances_the_checkpoint` rather than a sentence promising it would.

| AC | Test | Red → Green |
| -- | ---- | ----------- |
| AC-001 | `projection_mutation_coverage::projection_mutants_fail_exactly_their_declared_rules` (declared direction) | **red**: with `CheckpointOnlyStore`'s row declaring `commit_is_atomic_with_the_read_model` and its `impl Defect` overriding **nothing**, the run reported ``\`CheckpointOnlyStore\` declares that it fails \`commit_is_atomic_with_the_read_model\`, but the rule passed``. **green** once `commit_writes` at `tests/projection_mutation_coverage/mutants.rs:45-59` recorded the checkpoint and skipped `apply`. The red is the falsifier, run forwards. |
| AC-002 | the same test, undeclared direction (`assert_undeclared_outcome`) | **green throughout and load-bearing**: `CheckpointOnlyStore` passes `commit_advances_the_checkpoint`, and a skip there would have failed rather than counted. It was never red because the defect is exact; the assertion that would have caught inexactness is `tests/projection_mutation_coverage.rs:570-596`. |
| AC-003 | the same test, both cells of `UncommittedTransactionStore` | **red**: `every_projection_rule_has_a_mutant` reported `["commit_advances_the_checkpoint"]` as decorative. **green** once the store landed at `mutants.rs:85-101` with its row at `tests/projection_mutation_coverage.rs:280-300`. Both cells are asserted: it fails the progress rule and **passes** the atomicity rule, which is the whole reason it is the store that separates the two. |
| AC-004 | `projection_mutation_coverage::projection_mutant_registry_is_exhaustive` (the pin check) | **green by construction, and the construction is the claim**: `expect` is `&[(rule, substring)]` from the first commit, and a pin naming an undeclared rule fails at `:505-516`. Verified by hand at authoring time: moving `CheckpointOnlyStore`'s pin onto `commit_advances_the_checkpoint` reproduces that failure. |
| AC-005 | `projection_mutation_coverage::every_projection_rule_has_a_mutant` | **red**: `these projection rules have no mutant … ["commit_advances_the_checkpoint"]`, naming where to write the wrong implementation. **green** when `UncommittedTransactionStore` was registered. The universe came from the enumeration in both states, so the test would have gone red the same way for a rule added by a later story — which is the coupling `commit-rollback-and-drop-rules` inherits. |
| AC-006 | `projection_mutation_coverage::projection_mutant_registry_is_exhaustive` | **green**, seven assertions. Each was exercised by hand while authoring rather than merely written: adding a second `CheckpointOnlyStore` row fires the duplicate-name assertion; adding the type twice to `for_each_projection_mutant!` fires the doubly-enumerated one; misspelling a `fails` entry fires the typo catcher. |
| AC-007 | `projection_mutation_coverage::projection_mutants_fail_exactly_their_declared_rules` (`Origin` + `RUNTIME_PANICS` arms) | **green**, and the origin check is positive: `Origin::is_a_projection_rule_body` matched the panic raised at `crates/happenstance-testkit/src/projection.rs:292` in the AC-001 green run. The check was authored against the path `projection-suite-entry-point` actually landed, not an assumed filename — see `## Notes` for the one place it deliberately differs from the event-store sibling. |
| AC-008 | `projection_mutation_coverage::every_projection_mutant_states_its_provenance` | **green**. Both provenance strings name an adapter shape a reader can picture; neither is "a store that is wrong". |
| AC-009 | Static review of the module doc + a corroborating grep | **not a runtime claim, and the ledger says so**: the prohibition, the covered/uncovered account and the named CF-5 deferral are at `tests/projection_mutation_coverage.rs:25-32`, `:34-45`, `:235-261`. The grep for a fraction over `REGISTRY` returns nothing. |
| AC-010 | `cargo test --workspace --all-features` | **green** after the scope-note edit at `tests/mutation_coverage.rs:197-204`; the event-store binary's own meta-tests are undisturbed. |
| AC-011 | `cargo check -p happenstance-testkit --tests --target wasm32-unknown-unknown` | **green**, run explicitly because `--fast` omits it. Removing the crate-level `#![cfg(not(target_arch = "wasm32"))]` is what makes it red — `catch_unwind` has no unwinder there — and that gate is the first line of the file. |

## Commits

`feat(projection-store-freeze): The mutant registry` — one checkpoint commit,
the first of slice `commit-atomicity-and-mutants`, on
`initiative/from-contract-to-published-library`, immediately after `42144d2`
(`seal slice projection-conformance-suite approved`) and immediately before this
slice's second story, `commit-rollback-and-drop-rules`.

Named by subject and by predecessor rather than by hash, for the reason the
slice-1 reports give: this file is committed *inside* the commit it describes, so
no hash it quoted could survive being written into it.
`git log --grep "Story: projection-store-freeze/projection-mutant-registry"`
resolves it.

## Changes

| File | Shape of the change |
| ---- | ------------------- |
| `crates/happenstance-testkit/tests/projection_mutation_coverage.rs` | **New test target**, mounted by cargo's auto-discovery over `tests/*.rs` — verified that the manifest declares no `[[test]]` section and does not set `autotests = false`, so no manifest edit was made or needed. Carries the module doc (no pass rate, covered/uncovered axes, the named CF-5 deferral, the `wasm32` gate and its cost), the duplicated `Kind` / `FailureMode` / `Declared` shape, `REGISTRY`'s two rows, `for_each_projection_mutant!`, four helpers, and the four meta-tests inside `mod projection_mutation_coverage`. |
| `crates/happenstance-testkit/tests/projection_mutation_coverage/harness.rs` | **New.** The projection half of `tests/mutation_coverage/harness.rs`: `ProjectionSubject`, the three-valued `Verdict`, `Origin` with a *three-component* positive check, `RUNTIME_PANICS`, the quiet `Once` hook, `Probe` as a bare `fn` pointer, `run_probe`, `SubjectReport` and `run_subject`. A second copy rather than a shared module, and the file says why in its first paragraph. |
| `crates/happenstance-testkit/tests/projection_mutation_coverage/correct.rs` | **New.** The correct projection store in the pieces a defect replaces: `State` behind one `RefCell`, an **uninhabited** `MutantError`, the `Defect` seam (`NAME` plus one step, `commit_writes`), `MutantStore<D>` implementing the **bare** `ProjectionStore` and `ProjectionProbe`, and `MutantFixture<D>` implementing `ProjectionFixture` + `ProjectionSubject`. `Rc<RefCell<_>>`, so these stores are `!Send` and exercise the flavour ADR-0001 exists for. |
| `crates/happenstance-testkit/tests/projection_mutation_coverage/mutants.rs` | **New.** Two hostile stores, each an `impl Defect` overriding exactly one step: `CheckpointOnlyStore` and `UncommittedTransactionStore`. |
| `crates/happenstance-testkit/tests/mutation_coverage.rs` | **One doc paragraph.** The `REGISTRY` scope note at `:197-204`: "has neither a suite nor a mutant yet" replaced by a pointer to the new registry, with the "**Scope: the event-store port only.**" framing kept. Nothing else in the file was touched — no re-declaration, no re-scoping of `Declared`, no change to `for_each_mutant!`. |
| `.bklg/…/projection-mutant-registry/_ledger.md` | Eleven rows flipped `false` → `true`, each with a `file:line` and a test id. No criterion re-worded. |

No `Cargo.toml` was edited, no dependency added, no public item created, and no
`[FROZEN]` clause touched — `spec/SPECIFICATION.md` does not appear in this
story's diff.

## Gates

| Command | Result |
| ------- | ------ |
| `cargo test -p happenstance-testkit --all-features --test projection_mutation_coverage` | `4 passed; 0 failed` |
| `cargo check -p happenstance-testkit --tests --target wasm32-unknown-unknown` | clean — **run explicitly**, because `cargo xtask ci --fast` omits the mandatory wasm conformance-harness step and this story adds the file that could break it (AC-011, spec Clarifications item 6) |
| `cargo xtask affected --base main` | `affected gate passed` |
| `cargo xtask ci --fast` | `all required checks passed (--fast: 4 optional step(s) not run)` — fmt, clippy `-D warnings`, the whole workspace test run, docs, the `--no-default-features` doc build, and the `cargo package --list` licence assertion |
| `cargo xtask spec-trace` | exit 0, `traceability: no problems found; §7.1–§7.2 matches the checker`. NF-007 verified rather than assumed: no `Rule:` citation was added for the new meta-test names and none is owed, because this story lands no conformance rule. The two rules the checker reports as "claimed by no clause and owing a decision" are pre-existing and belong to the event-store and model families. |
| `cargo fmt --check` | clean. The formatter's `--write` pass ran **last**, after clippy, and collapsed one wrapped `state.checkpoints.insert(…)` chain. |

## Notes

**One deliberate divergence from the event-store harness, and it is a
correction rather than a preference.** `Origin::is_in` over there compares the
panic location's **leaf** file name, because the path rustc records is
platform-shaped (`crates\…` here, `crates/…` on CI) and `suite.rs` is unique in
the workspace. `projection.rs` is **not** unique: `happenstance-core` carries one
too. A leaf-only check would therefore classify a panic raised inside the
*contract crate* as a projection rule rejecting the store — the exact
substitution CF-2 forbids, arriving through the check that exists to prevent it.
So `is_a_projection_rule_body` compares the last three path components
(`happenstance-testkit`, `src`, `projection.rs`) via `Path::components`, which is
still separator-agnostic because `Path` splits on whichever separator the host
writes. This is worth a reviewer's attention because getting it wrong is silent
in one direction: a check that never matches fails everything loudly, but a check
that matches too much passes everything quietly.

**`Declared` / `Kind` / `FailureMode` are duplicated rather than shared**, and
the reasoning is recorded in the ledger's AC-004 evidence and in the type's own
doc comment. Short version: the three types are declared inside
`tests/mutation_coverage.rs` itself rather than in one of its `#[path]` support
modules, so sharing them would mean **moving them out of that file** — which this
story's PR boundary forbids in as many words ("no re-declaration, no re-scoping
of `Declared`"). Field names and semantics match one for one, which is the part
that was never open.

**Two steps that a reader will expect and will not find.** The `Defect` seam has
exactly one step, `commit_writes`, because exactly one defect needs it and one
more can be expressed by it. That is the event-store family's discipline, stated
in its own words there: a step earns its place when *a conformance rule can
observe the difference*, and the family grew from three steps to nine that way.
`commit_writes` is deliberately **one** step covering both halves rather than an
`apply_rows` step and a `record_checkpoint` step: PS-1's obligation is the
coupling, so the interesting defects are the ways the halves come apart, and a
two-step split would spell `UncommittedTransactionStore` as *two* overrides and
break "one defect per store" for the one store that most needs it to hold.

**`FailureMode::StorePanic` and `Kind::ConformantVariant` are declared and
unused**, each behind an `#[expect(dead_code)]` naming what will use it.
`#[expect]` rather than `#[allow]` is the point: an unfulfilled expectation is
itself a warning, so each attribute goes red on the day the deferral ends and
has to be removed in the same change that closes it. `ConformantVariant`'s
reason names `buffering-conformant-variant` (HS-S0014).

**Two adjacent repairs were resisted, both by name.** PS-1's missing progress
obligation is *demonstrated* by `UncommittedTransactionStore` and is not repaired
here — the observation lives in that store's doc comment and in its `provenance`
string, and the clause is `[FROZEN]` with `ps-clause-pairing-sweep` owning the
sweep. No CF-1 – CF-6 clause was amended either; `spec-trace` marks a `Rule:`
line containing "meta-test" as `elsewhere`, and in any case this story adds no
rule and needs no citation. Both claims were **verified** rather than assumed:
`cargo xtask spec-trace` exits 0 with the specification unmodified.

**One inaccuracy inherited from planning, recorded rather than silently
corrected.** The ledger's `mount_point` fields for the slice-mate name
`crates/happenstance-testkit/src/registry.rs` as the home of
`for_each_projection_store_rule!`. It is not: the projection family's enumeration
lives in `crates/happenstance-testkit/src/projection.rs:326-337`, beside its rules,
for the reason that file's module doc gives (a projection rule in `suite.rs` is
reported as an orphan of the event-store enumeration). This story consumes it
where it actually is; no backlog frontmatter was edited to say so.

**One stale sentence left standing, on purpose.** The rustdoc on
`commit_is_atomic_with_the_read_model` still says `CheckpointOnlyStore` "lands
with `projection-mutant-registry` (HS-S0008)", and the doc on
`commit_advances_the_checkpoint` still calls its mutant a carried debt. Both are
now false. `crates/happenstance-testkit/src/**` is outside **this** story's PR
boundary and inside the slice-mate's, so the correction lands there rather than
as scope drift here.
