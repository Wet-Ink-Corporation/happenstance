---
item: "HS-S0009"
stage: report
created: "2026-08-14"
updated: "2026-08-14"
---

# Report — CheckpointOnlyStore fails the suite by name

## Findings Ledger

**Eleven of eleven ACs satisfied, each by real reachable behaviour with a real
test.** Four meta-tests in a new cargo test target, two hostile projection stores
that are genuine `ProjectionStore` + `ProjectionProbe` implementations driven
through the real `projection_store_conformance!` rule set, and both of the steps
the spec singled out as ones a story-grain gate could hide — the `wasm32`
conformance-harness check and `spec-trace` — run explicitly and cited.

**The claim this story does make.** The projection suite discriminates. Before
this commit every projection rule was green against the oracle and had never
rejected anything; a rule in that state is indistinguishable from a good rule
until an adapter with the matching bug passes it. After it,
`commit_is_atomic_with_the_read_model` rejects a store that commits the
checkpoint and discards the write set, and `commit_advances_the_checkpoint`
rejects a store whose `commit` returns `Ok` having written neither half — and
each passes the other, which is what makes the pair differential rather than
duplicated.

**The claims it does not make.** The projection port is not frozen and this story
does not say it is. Seven of §4.11's seventeen rules are still owed; CF-5's
conformant variant is deferred by name; nothing here has a medium outside the
process, so no defect whose observation needs a real restart or a real pool is
modelled. All three are written into the registry's own "what this set covers,
and what it does not" section rather than into this report alone, because that is
where the next reader will be.

| Finding | Evidence | Follow-up |
| ------- | -------- | --------- |
| **AC-001 — a wrong store fails by name.** `CheckpointOnlyStore` is rejected by `commit_is_atomic_with_the_read_model`, at that rule's both-present-or-both-absent assertion and nowhere else. | `tests/projection_mutation_coverage/mutants.rs:40-60`; asserted by `projection_mutation_coverage::projection_mutants_fail_exactly_their_declared_rules`, pinned at `"must become durable together or not at all"`. RED first: the same test failed with "declares that it fails … but the rule passed" until the one-line defect landed. | None. The falsifier is one deletion and the method body names it. |
| **AC-002 — the red rule is a diagnosis, not noise.** Every cell `CheckpointOnlyStore` does not declare comes back `Passed`; a skip would fail rather than count. | `tests/projection_mutation_coverage.rs:570-596`, checked against the fixture's own declines list (`harness.rs:426-437`). | None. |
| **AC-003 — §4.11's em-dash now has a store.** `UncommittedTransactionStore` fails `commit_advances_the_checkpoint` and **passes** `commit_is_atomic_with_the_read_model`. | `mutants.rs:85-101`; registry row `tests/projection_mutation_coverage.rs:280-300`; both cells asserted by the exactness meta-test. | **Reported, not repaired.** This store is precisely the evidence `.kb/open-questions/ps-1-states-no-progress-obligation.md` wants. PS-1 is `[FROZEN]`; the observation is recorded in the store's `provenance` and doc comment, and the sweep is `ps-clause-pairing-sweep`'s. No file under `spec/` is in this diff. |
| **AC-004 — one shape across both families.** Six fields, same names, same semantics, `expect` keyed by rule from the first commit. | `tests/projection_mutation_coverage.rs:88-220` against `tests/mutation_coverage.rs:73-186`. | **Open choice resolved and recorded**: the types are *duplicated*, because sharing them would mean moving them out of `tests/mutation_coverage.rs`, which this story's PR boundary forbids. Reasoning is in the type's doc at `:170-183` and in the ledger. |
| **AC-005 — the forcing function exists and has no exemption list.** | `projection_mutation_coverage::every_projection_rule_has_a_mutant` (`:404-420`), universe from `for_each_projection_store_rule!` + `__emit_rule_names`. Observed red naming `["commit_advances_the_checkpoint"]`. | **Intended coupling, now live.** `commit-rollback-and-drop-rules`, `reset-rules` and `read-through-and-rebuild-rules` cannot land a rule without a mutant. If a rule ever has no plausible failing implementation, ADR-0010's answer is to retire the rule — not to add an exemption. |
| **AC-006 — a wrong edit says which row and which field.** Seven assertions, each naming the offender. | `projection_mutation_coverage::projection_mutant_registry_is_exhaustive` (`:429-541`). | None. |
| **AC-007 — failing for the *right* reason is checked positively.** | `Origin::is_a_projection_rule_body` (`harness.rs:172-186`) plus `RUNTIME_PANICS`, with the mirror check on the `StorePanic` arm. | **Divergence worth a reviewer's eye:** the check matches the last *three* path components rather than the leaf, because `happenstance-core` also has a `projection.rs` and a leaf-only match would read a panic in the contract crate as a rule rejecting the store. |
| **AC-008 — provenance names a real adapter shape.** | `projection_mutation_coverage::every_projection_mutant_states_its_provenance` (`:708-718`); the two strings at `:266-273` and `:284-298`. | None. |
| **AC-009 — no pass rate, and CF-5 deferred by name.** | Module doc `:25-32`, `:34-45`, `:235-261`; `Kind::ConformantVariant` unused behind an `#[expect(dead_code)]` naming HS-S0014. A grep for a fraction over `REGISTRY` returns nothing. | **Named hole**: `buffering-conformant-variant` lands the variant and the positive control. Until then neither the "at least one variant" assertion nor `conformant_variants_pass_everything` exists here, because over an empty set both pass while asserting nothing. |
| **AC-010 — the event-store registry stops lying.** | `tests/mutation_coverage.rs:197-204`; `cargo test --workspace --all-features` green afterwards. | None. |
| **AC-011 — the wasm gate holds and no GAT was declared.** | `#![cfg(not(target_arch = "wasm32"))]` at `:61` with the cost stated at `:47-59`; `cargo check -p happenstance-testkit --tests --target wasm32-unknown-unknown` clean; `ProjectionFixture::Store` is the owned `MutantStore<D>`. | **Stated cost, not a defect**: the stores in this binary are never type-checked for `wasm32`. |

**Two process findings, neither of which changed a line of code.**

1. The ledger's planned `mount_point` for the projection enumeration names
   `crates/happenstance-testkit/src/registry.rs`. The enumeration actually lives
   in `crates/happenstance-testkit/src/projection.rs:326-337`, beside its rules —
   `registry.rs` holds the *event-store* family's. The story consumed it where it
   is; no frontmatter was edited.
2. Two rustdoc paragraphs in `crates/happenstance-testkit/src/projection.rs`
   became false with this commit — each still describes `CheckpointOnlyStore` as a
   carried debt arriving with a later story. That file is outside this story's PR
   boundary and inside the slice-mate's, so the correction lands there.

## Acceptance

| AC | Status | Verified by |
| -- | ------ | ----------- |
| AC-001 | satisfied | `projection_mutation_coverage::projection_mutants_fail_exactly_their_declared_rules` (declared direction), red-then-green |
| AC-002 | satisfied | the same test, undeclared direction |
| AC-003 | satisfied | the same test, both cells of `UncommittedTransactionStore`; plus review that no `spec/` file is in the diff |
| AC-004 | satisfied | review against `tests/mutation_coverage.rs:73-186`, plus the pin assertion in `projection_mutant_registry_is_exhaustive` |
| AC-005 | satisfied | `projection_mutation_coverage::every_projection_rule_has_a_mutant`, observed red then green |
| AC-006 | satisfied | `projection_mutation_coverage::projection_mutant_registry_is_exhaustive` |
| AC-007 | satisfied | the exactness meta-test's `Origin` and `RUNTIME_PANICS` arms |
| AC-008 | satisfied | `projection_mutation_coverage::every_projection_mutant_states_its_provenance` |
| AC-009 | satisfied | static review of the module doc + corroborating grep (an absence has nothing to assert against) |
| AC-010 | satisfied | static review of the scope-note diff + `cargo test --workspace --all-features` |
| AC-011 | satisfied | `cargo check -p happenstance-testkit --tests --target wasm32-unknown-unknown` + `cargo xtask ci --fast` |

Merge gate: `cargo xtask affected --base main` **green**, `cargo xtask ci --fast`
**green**, `cargo xtask spec-trace` **exit 0**, `cargo fmt --check` **clean**.

## Knowledge Harvest

Three candidates for `.kb` at closeout. None is proposed as a decision here; each
is a *measured* fact this story produced.

- **A positive panic-origin check must be specific enough to name one file.**
  The event-store family's leaf-only match was correct because `suite.rs` is
  unique; the projection family's is not, and the same code would have silently
  accepted a panic from the contract crate as a rule rejecting a store. The
  transferable rule is that a positive check's *specificity* has to be re-derived
  per family rather than copied — its failure mode is silent in exactly one
  direction. Candidate: a playbook line under the mutation-registry practice.
- **Deferring half a clause is a shape, not a promise.** `Kind::ConformantVariant`
  and `FailureMode::StorePanic` are declared, unused, and carried by
  `#[expect(dead_code)]` with the closing story named in the reason. `#[expect]`
  rather than `#[allow]` makes the deferral *self-retiring*: the attribute is
  itself a warning the day the arm is used. Candidate: a `standards/rust/` note.
- **A step seam earns a step when a rule can see the difference.** The projection
  `Defect` has one step, and `commit_writes` covers both halves of a commit
  deliberately — because PS-1's obligation is the coupling, and a two-step split
  would spell "neither write durable" as two overrides and break
  one-defect-per-store for the one store that most needs it to hold. Candidate: an
  amendment to ADR-0010's practice notes, if closeout finds the same shape twice.
