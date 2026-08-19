---
item: HS-S0122
stage: implement
created: 2026-08-13T00:00:00.000Z
updated: 2026-08-13T00:00:00.000Z
---

# Acceptance ledger — CF-27's own rule, written or refused in writing

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

**Branch note (spec *Clarifications resolved during spec* §1).** AC-001 and AC-008 hold on both
branches. AC-002 – AC-004 are the **decide** branch; AC-005 – AC-007 are the **refusal** branch. The
rows for the branch ADR-0028 did *not* select are flipped with evidence of the form
`not-applicable — branch <A|B>; ADR-0028 <path>:<line> "<quoted decision statement>"` — a real
citation, not a placeholder — so no row is silently dropped.

```yaml
- id: AC-001
  criterion: "**The branch is read from ADR-0028, not chosen here.** GIVEN a maintainer who in two years wants to know *why* CF-27 ended the way it did, WHEN this story lands, THEN ADR-0028's decision statement is quoted **verbatim** into this story's record together with the accepted atom's `.kb/decisions/` path, the branch executed is stated as following from that quote, and no file under `.kb/` appears in this story's diff; AND if the accepted atom is not on disk when the story starts, the story **halts and reports a dependency failure** rather than inferring a branch from what the gate would tolerate."
  satisfied: false
  evidence: ""
  mount_point: "spec/SPECIFICATION.md:8034-8060 (CF-27's clause, read by `cargo xtask spec-trace`) — the branch determination recorded alongside it in this story's implementation report"
  verifying_test: "content review of the implementation report against .bklg/from-contract-to-published-library/retention-and-incomplete-logs/_storymap.md:75-76 and project.md:219-222; mechanical `git diff --name-only main...HEAD` (no `.kb/` path); `redkiln validate --kb && redkiln doctor` clean"
- id: AC-002
  criterion: "**Decide branch — the rule exists where a rule is allowed to exist, and runs everywhere rules run.** GIVEN an adapter author who runs `happenstance_testkit::event_store_conformance!` against their store, WHEN ADR-0028 decides the retention question, THEN `suffix_store_is_distinguishable_from_a_young_store` is a rule body in `crates/happenstance-testkit/src/suite.rs` (and in no `tests/` file) whose assertion is the one ADR-0028 fixes, registered **exactly once** in `for_each_event_store_rule!` (`crates/happenstance-testkit/src/registry.rs:94`) so it reaches `__emit_tokio`, `__emit_blocking`, `__emit_wasm` and `__emit_rule_names`, and `cargo xtask spec-trace` check 6 passes because CF-27 already claims the name — with nothing added to `UNCLAIMED_PENDING_ADR`."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/src/registry.rs:94 — `for_each_event_store_rule!` (architecture-brief composition root 2), with the body in crates/happenstance-testkit/src/suite.rs"
  verifying_test: "cargo test -p happenstance-testkit --all-targets (rule name present in crates/happenstance-testkit/tests/memory_conformance.rs, memory_conformance_blocking.rs, memory_conformance_wasm.rs); cargo xtask spec-trace (checks 4 and 6, xtask/src/spec_trace.rs:695-711, :725-727, :765-800)"
- id: AC-003
  criterion: "**Decide branch — the rule can be failed, and something in the tree fails it.** GIVEN a maintainer who must trust that a green suite means something, WHEN the rule ships, THEN it is green against `MemoryFixture` and against the honest completeness instrument mounted by `retained-set-instrument-and-conformance-mount`, and **red** against a registered wrong implementation — one `Defect` impl under `crates/happenstance-testkit/tests/mutation_coverage/mutants.rs` plus one `REGISTRY` row (`crates/happenstance-testkit/tests/mutation_coverage.rs:324`) whose `fails` list names this rule and is non-empty — with `mutant_registry_is_exhaustive` and `mutants_fail_exactly_their_declared_rules` green in both directions and no parallel harness invented."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/tests/mutation_coverage.rs:324 (`REGISTRY`), reached through `for_each_event_store_rule!` at crates/happenstance-testkit/src/registry.rs:94"
  verifying_test: "cargo test -p happenstance-testkit --test mutation_coverage (`mutant_registry_is_exhaustive`, `mutants_fail_exactly_their_declared_rules`); cargo test -p happenstance-testkit --test memory_conformance; the honest instrument's target in the shape of crates/happenstance-testkit/tests/fixture_instruments.rs:201-205"
- id: AC-004
  criterion: "**Decide branch — the rule asks the *port*, not the instrument.** GIVEN an application that holds only an `impl EventStore` — the reader CF-27's `Rejects:` is about — WHEN the rule runs, THEN its body uses only `Fixture`'s declared surface and the `EventStore` port: no method on a concrete instrument type, no downcast, and no `Capability` standing in as a completeness report; AND if it genuinely cannot be written without a fixture seam, that seam is DA-4's shape and nothing else — a **defaulted** `Capability` associated const defaulting to `Capability::declined(<reason>)` plus a **defaulted** method that panics naming both ways of reaching it — with no **required** trait item added. A rule that distinguishes the instrument by *asking the instrument* (`SelfReportingSuffixStore`) fails this criterion even with every gate green."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/src/suite.rs — the rule body, registered at crates/happenstance-testkit/src/registry.rs:94; any seam at crates/happenstance-testkit/src/contract.rs:207-211, :297-307"
  verifying_test: "adversarial review of the body against .bklg/from-contract-to-published-library/retention-and-incomplete-logs/cf-27-rule-or-recorded-refusal/discover.md:45 and _decomposition.md:299-305; cargo test -p happenstance-testkit --test memory_conformance; cargo-semver-checks at the PR grain (CONTRIBUTING.md:291-296)"
- id: AC-005
  criterion: "**Refusal branch — CF-27 itself says why the rule cannot exist.** GIVEN an adapter author who opens CF-27 looking for the completeness instrument's teeth, WHEN ADR-0028 refuses, THEN **CF-27's own clause body** — in place, not a sibling clause, not an appendix, not a backlog note — states that distinguishability is a *reported* property; that `EventStore`'s surface (`read`, `append`, `head`, `contains_event_id`) carries no completeness channel; that the clause's own assertion is fixed by ES-39's primitive and no primitive has been chosen; and that every candidate primitive is a change to a surface published at `0.2.0`, which gate decision 4 forbids."
  satisfied: false
  evidence: ""
  mount_point: "spec/SPECIFICATION.md:8034-8060 — CF-27's clause body (architecture-brief composition root 7)"
  verifying_test: "content review against project.md:219-222, spec/SPECIFICATION.md:8042-8044, :4325-4349 and crates/happenstance-core/src/store.rs:119, :213, :268; cargo xtask spec-trace; cargo xtask affected --base main (.redkiln/config.yaml:40)"
- id: AC-006
  criterion: "**Refusal branch — the clause names what a reader is therefore on its own against.** GIVEN a reader who will now build on a store that may have forgotten, WHEN they read CF-27, THEN the clause names three concrete readers with their **observed** outcomes cited to this project's recorded observations rather than predicted afresh: a conditional append admitted over a last-*retained* match from `read_decision_model` (`crates/happenstance-core/src/store.rs:321-331`) feeding `AppendCondition::after_opt`; `IngestStore::holds` (`crates/happenstance-sync/src/ingest.rs:164`) and `EventStore::contains_event_id` answering `false` for an event this store minted, so a peer re-sends forever (DA-8); and the projection runner resuming across the hole with its actual recorded state."
  satisfied: false
  evidence: ""
  mount_point: "spec/SPECIFICATION.md:8034-8060 — CF-27's clause body, citing composition roots 4-6 (_decomposition.md:113-141)"
  verifying_test: "citation check that each of the three resolves to a recorded observation from `decision-model-and-ingest-observed` / `projection-runner-across-the-hole` (project AC-006, testing brief row at _decomposition.md:609); content review against project.md:219-222"
- id: AC-007
  criterion: "**Refusal branch — the marker is kept *and* explained, and nothing enters the registry.** GIVEN the next reader, who must be able to tell a deliberate non-rule from an oversight, WHEN the refusal lands, THEN CF-27's `Rule:` line keeps its `(new)`/`†` marker **and** the clause says in words that the marker is deliberate and why; AND no rule name is added to `for_each_event_store_rule!` (`crates/happenstance-testkit/src/registry.rs:94`) and no entry is added to `UNCLAIMED_PENDING_ADR` (`xtask/src/spec_trace.rs:1978`), which exists for a rule that *exists* and is claimed by nobody — the inverse of this case, and a ratchet that can only shrink. This is the criterion `SilentNonRule` fails."
  satisfied: false
  evidence: ""
  mount_point: "spec/SPECIFICATION.md:8034-8060 — CF-27's `Rule:` line and clause body; crates/happenstance-testkit/src/registry.rs:94 confirmed untouched"
  verifying_test: "cargo xtask spec-trace (check 4, xtask/src/spec_trace.rs:695-711 and the `schedules_new` predicate at :1626-1631); mechanical `git diff` showing crates/happenstance-testkit/src/registry.rs and xtask/src/spec_trace.rs untouched; content review against discover.md:49"
- id: AC-008
  criterion: "**Both branches — the `0.2.0` surface is not moved, the markers are not moved, and a decision that needs a surface escalates instead of landing.** GIVEN the maintainer of three crates published at `0.2.0`, WHEN this story lands on either branch, THEN no diff touches the public signatures in `crates/happenstance-core/src/store.rs` or `crates/happenstance-core/src/append.rs`; CF-27's `[DEFERRED]` marker, ES-39's and ES-40's markers, §7.1/§7.2 and §1.3's prose census are **unchanged** (all HS-S0123's); and if ADR-0028's decision can only be executed by adopting one of DA-7's four primitives, the finding — naming the surface, the version consequence (a `0.3.0` this initiative's exit criteria do not contemplate) and which DA-7 row it is — is handed to `surface-diff-and-the-ac-012-escalation` and **the change is not made here**."
  satisfied: false
  evidence: ""
  mount_point: "spec/SPECIFICATION.md:8034-8060 (CF-27, markers unchanged) plus the read-only boundary at crates/happenstance-core/src/store.rs and crates/happenstance-core/src/append.rs (_decomposition.md:614)"
  verifying_test: "git diff main...HEAD -- crates/happenstance-core/src/store.rs crates/happenstance-core/src/append.rs (no public-signature change); git diff on spec/SPECIFICATION.md (no marker or census line changed); cargo xtask affected --base main (.redkiln/config.yaml:40); cargo xtask ci --fast (:55); cargo-semver-checks at the PR grain (CONTRIBUTING.md:291-296)"
```
