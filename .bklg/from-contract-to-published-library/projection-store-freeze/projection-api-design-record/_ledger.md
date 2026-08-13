---
item: HS-S0003
stage: implement
created: 2026-08-12T13:45:57.874Z
updated: 2026-08-12T13:45:57.874Z
---

# Acceptance ledger — DT-3 and DT-8 resolved in the public-API design record

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

**A note on evidence for this story specifically.** This is a prose deliverable: the Testing brief
assigns project AC-006 the tier *None (design-stage artefact)* and project AC-007 a tier conditional on
the DT-8 arm taken (`.bklg/from-contract-to-published-library/projection-store-freeze/_decomposition.md:776-777`).
Evidence is therefore a `file:line` range in the amended `_design.md` plus the named deterministic
check's output — never a test id invented to fill the field. Where the machine half of a criterion runs
in a later story, the `verifying_test` column names that check and the story that runs it; the evidence
cited here is the record's own half.

```yaml
- id: AC-001
  criterion: "**GIVEN** P2 has built a projection fixture whose storage genuinely cannot provide one of the port's guarantees, **WHEN** they open `_design.md` to learn who is entitled to write the sentence a consumer reads, **THEN** the DT-3 resolution names **exactly one** authoritative source and, for each of the three reason-writers that already exist in this tree — the fixture-written `Capability::declined` (`crates/happenstance-testkit/src/contract.rs:355-419`), the testkit-written `NO_STORE_LIMITS` / `NO_CEILING_REASON` (`:435-456`), and per-adapter prose — states which defers to which and why, so P2 is never left choosing between two policies; and it **cites** `.kb/open-questions/cf-40-fixture-limits-ownership.md` as authoritative rather than preferring a reading of it."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/projection-store-freeze/_design.md"
  verifying_test: "Design-stage review of `_design.md` § DT-3 (project.md DoD 7), against UX brief AC-U01 (.bklg/from-contract-to-published-library/projection-store-freeze/_decomposition.md:84-96); deterministic half: rg -n \"Capability::declined|NO_CEILING_REASON|per-adapter\" .bklg/from-contract-to-published-library/projection-store-freeze/_design.md and rg -n \"cf-40-fixture-limits-ownership\" over the same file"

- id: AC-002
  criterion: "**GIVEN** P3 runs the suite on `wasm32-unknown-unknown`, the target BR-12 exists to protect, **WHEN** a projection capability is declined there, **THEN** DT-3's resolution has already accounted for it: the record states that `RuleOutcome::report` is a **measured** no-op on that target (`crates/happenstance-testkit/src/contract.rs:511-531`), that the projection wasm harness routes `skip_line` through `console_log!` exactly as `__emit_wasm` already does (`crates/happenstance-testkit/tests/memory_conformance_wasm.rs:25`), and that the machine-checked half of \"told, with a reason\" is a `RuleOutcome`-**value** assertion — a projection sibling of `capability_skips_are_reported` (`crates/happenstance-testkit/tests/mutation_coverage.rs:3184`) — never stdout."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/projection-store-freeze/_design.md"
  verifying_test: "Design-stage review against UX brief AC-U09/AC-U11 and project.md AC-005; deterministic half: rg -n \"wasm32|__emit_wasm|RuleOutcome\" .bklg/from-contract-to-published-library/projection-store-freeze/_design.md. Machine half deferred to the projection sibling of crates/happenstance-testkit/tests/mutation_coverage.rs:3184 (`capability_skips_are_reported`) at story `projection-capability-skips`"

- id: AC-003
  criterion: "**GIVEN** the implementer of `projection-capability-skips` (HS-S0008), whose story explicitly takes *\"the projection capability set fixed by DT-3's recorded resolution rather than invented here\"* (`_storymap.md:60`), **WHEN** they open this record, **THEN** the projection fixture's capability constants are **enumerated**, each carrying whether it is a MUST in `SECOND_HANDLE`'s sense and **who writes its reason** under AC-001's policy — at minimum the reset-refusal capability `refused_reset_changes_nothing` needs (`spec/SPECIFICATION.md:5200-5217`) — with PS-12's gate recorded as the `ProjectionProbe` const `READS_THROUGH_BATCH` rather than a fixture const; and if the set ends with **nothing declinable at all**, that outcome is recorded inside DT-3's resolution as a finding, not treated as licence to drop the reporting discipline."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/projection-store-freeze/_design.md"
  verifying_test: "Design-stage review against Architecture brief Note 5 (.bklg/from-contract-to-published-library/projection-store-freeze/_decomposition.md:517-544), the UX brief's `What would make this brief wrong` (:252-259) and .kb/decisions/0010-the-suite-must-prove-itself.md; deterministic half: every constant `projection-capability-skips` implements is quotable from `_design.md`"

- id: AC-004
  criterion: "**GIVEN** an adapter author this repository did not write and does not supervise — the seventh adapter P2's persona generalises to (`…/personas-and-journeys.md:115-121`) — **WHEN** they ask whether the suite's bar is held for them, **THEN** `_design.md` names DT-8's arm **and its cost to them**: on the outside-author arm the extension surface is the documented pair `projection_store_conformance!` + `ProjectionProbe`, bounded at *one feature flag on a dependency the adapter already has and no new edge in the dependency graph* (`spec/SPECIFICATION.md:5015-5031`); on the narrow arm the recorded scope is placed **where an outsider meets it** — the testkit's own crate doc — and that file is named as `documented-extension-surface`'s (HS-S0015) obligation. Either way the arm is legible enough that HS-S0015 is scopable from this file alone."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/projection-store-freeze/_design.md"
  verifying_test: "Design-stage review against UX brief AC-U02 (.bklg/from-contract-to-published-library/projection-store-freeze/_decomposition.md:97-108) and project.md AC-007; conditional machine half deferred to story `documented-extension-surface` (Testing brief row AC-007, _decomposition.md:777) — a fixture built from the documentation alone run against the mutant-registry exactness check and the capability-skip rule"

- id: AC-005
  criterion: "**GIVEN** the rule author inside the testkit and the implementer of `owned-batch-port-shape` (HS-S0004), **WHEN** they open `## Items` and `## Signatures` to build against a shape someone agreed to, **THEN** the target trait is transcribed item for item — `type Batch;` with no lifetime, `fn begin(&self) -> Self::Batch` neither `async` nor fallible, `checkpoint -> Checkpoint`, `commit(batch, id, position, Authority) -> Result<(), CommitError<E>>`, `reset(batch, id) -> Result<(), ResetError<E>>`, `rollback`, plus `ProjectionProbe`'s five items — each signature carrying its `PS-*` clause id, and where any sentence disagrees with accepted ADR-0017 / ADR-0018 / ADR-0019, the **atom wins** and the record says which sentence yielded rather than editing the atom."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/projection-store-freeze/_design.md"
  verifying_test: "Design-stage review against spec/SPECIFICATION.md:4632-4731 and :4998-5013 and crates/happenstance-core/src/projection.rs:87-139; deterministic half: rg -n \"PS-\" .bklg/from-contract-to-published-library/projection-store-freeze/_design.md shows no unattributed signature. Compile-time proof deferred to story `owned-batch-port-shape`"

- id: AC-006
  criterion: "**GIVEN** `CLAUDE.md`'s reader — fluent in the domain, new to idiomatic Rust — **WHEN** they meet a shape that looks gratuitously unusual, **THEN** `## What it costs a caller` has already named the alternative that lost, at the call site, for each of the four load-bearing choices: `Checkpoint` as a three-variant enum rather than `(Option<SequencePosition>, bool)` because the tuple can spell *\"authoritative, never run — which means nothing\"* (`spec/SPECIFICATION.md:4643-4658`); `CommitError` and `ResetError` staying two enums so `commit`'s caller never matches `Refused` (`:4722-4727`); `begin` infallible because `async` + fallible implies a round trip Neon's one-shot transport cannot afford (`:4884-4897`); `reset` taking the caller's own deletes because *\"the port has no idea what the read model is\"* (`:5165-5170`) — each stated **once**, sited in the item's own rustdoc rather than only in an ADR (RS-70-5, `standards/rust/70-rustdoc-obligations.md:243`), and each error carrying the adapter error as a type parameter, never a `String` (`standards/rust/30-error-taxonomy.md:15,71`)."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/projection-store-freeze/_design.md"
  verifying_test: "Design-stage review against UX brief AC-U03 – AC-U05 (.bklg/from-contract-to-published-library/projection-store-freeze/_decomposition.md:112-138); deterministic half: four rationale sentences over four items, none repeated across sections (the RS-70-5 density budget, standards/rust/70-rustdoc-obligations.md:243)"

- id: AC-007
  criterion: "**GIVEN** an operator whose `reset` was declined by a store's own policy, **WHEN** they ask *which* policy declined it, **THEN** `_design.md` has decided out loud: either `ResetError::Refused` carries the store's stated reason, or it stays bare (`spec/SPECIFICATION.md:4676-4682`) and the record says where a caller is expected to find the reason instead, given the clause's own reasoning that *\"the port supplies the mechanism; the domain decides what to protect\"* (`:5210-5216`). Silence is the failure, not a passing default."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/projection-store-freeze/_design.md"
  verifying_test: "Design-stage review against UX brief AC-U07 (.bklg/from-contract-to-published-library/projection-store-freeze/_decomposition.md:148-157); deterministic half: rg -n \"ResetError::Refused\" .bklg/from-contract-to-published-library/projection-store-freeze/_design.md returns a decision sentence, not only a signature. Behavioural half deferred to `refused_reset_changes_nothing` at story `reset-rules`"

- id: AC-008
  criterion: "**GIVEN** a downstream story about to land one of the new items, **WHEN** it reads `## Visibility and stability`, **THEN** the record is specific enough to be **violated**: `#[non_exhaustive]` on all four new types; re-export placement at `crates/happenstance-core/src/lib.rs:98-124`; `ProjectionProbe` behind `conformance` with `#[cfg_attr(docsrs, doc(cfg(…)))]` copying the `memory` pattern; `MemoryProjectionStore` behind `memory`; `unstable-projection` recorded as the arm AC-A04 takes; **both** halves of the mount named — the export block *and* the feature table, because an item mounted at one and not the other is an item no adapter can name (Architecture brief Note 1, `_decomposition.md:339-349`); and the rustdoc hazard stated, an intra-doc link into a feature-gated module being a hard error this workspace has already paid for once (`crates/happenstance-testkit/src/lib.rs:112-132`)."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/projection-store-freeze/_design.md"
  verifying_test: "Design-stage review against RS-40-5 (standards/rust/40-public-surface-and-evolution.md:212) and Architecture brief Notes 1 and 9; deterministic half: three lines per public item (visibility, feature, placement) for every id in the spec's Integration contract `Public items` list. Compile-time proof deferred to stories `owned-batch-port-shape` and `projection-probe-conformance-feature`"

- id: AC-009
  criterion: "**GIVEN** P2 ten minutes into writing an impl, meeting `error[E0195] lifetime parameters or bounds on method 'commit' do not match the trait declaration` with, today, *nothing to copy from* (`references/adapter-shapes.md:186-194`), **WHEN** they open the port's page, **THEN** `## The doctest` carries the example text **and names the source item it will live on and the gate step that compiles it** (`cargo test --doc`, inside `cargo xtask ci`), because this record cannot compile it and a described example is not a checked one; and any `compile_fail` doctest demonstrating a reasonless declension is spelled **bare**, never `compile_fail,E0080` (`crates/happenstance-testkit/src/contract.rs:400-403`)."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/projection-store-freeze/_design.md"
  verifying_test: "Design-stage review against UX brief AC-U06/AC-U12 and CLAUDE.md's `a doctest in place of a mock`; deterministic half: `## The doctest` names a real target path (crates/happenstance-core/src/projection.rs and/or the MemoryProjectionStore module) and a real gate step. Compilation deferred to `cargo test --doc -p happenstance-core` at story `memory-projection-store` (Testing brief row AC-012, _decomposition.md:782)"

- id: AC-010
  criterion: "**GIVEN** an implementer in a later slice reaching for a shape that would compile, **WHEN** they check `## Anti-patterns` first, **THEN** each forbidden move is listed **with its evidence**: no `#[async_trait]` (`.kb/decisions/0001-async-port-flavours.md`); no `type Batch: Send;` (PS-36 — `trait_variant` copies the bound into the `!Send` flavour and breaks wasm32, `spec/SPECIFICATION.md:5601-5627`); no borrowing GAT anywhere on the fixture (five-ingredient rustc ICE, still reproducing on 1.97.1, `crates/happenstance-testkit/src/contract.rs:97-111`); no merging the two error enums; no `async fn begin`; no second skip vocabulary and no projection-local skip type (Architecture brief `AC-A06`, UX brief `AC-U08`); no literal position assertions; no hardening of `ProjectionId::new` as a side effect (`.kb/open-questions/projection-id-is-unvalidated.md`)."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/projection-store-freeze/_design.md"
  verifying_test: "Design-stage review against CLAUDE.md `Binding constraints` and `The rule that matters`, plus RS-91-3 (standards/rust/91-adapter-authoring-recipe.md:156-158); deterministic half: rg -n \"Anti-patterns\" -A 60 .bklg/from-contract-to-published-library/projection-store-freeze/_design.md shows a path or atom id on every entry"

- id: AC-011
  criterion: "**GIVEN** the repository owner, who signed the no-screen determination at the `/redkiln:plan` design gate on 2026-08-12 (`_design.md:96-101`), **WHEN** this record returns to that gate, **THEN** they can approve or decline the **amendment** without unpicking what they already approved: the no-screen determination, the `surfaces: []` manifest and the prior sign-off block survive **verbatim**; the added content occupies the bundled template's own section slots in the template's own order (`.redkiln/templates/_design.md`), replacing only the lines that read `N/A — no user-facing surface`, and only for the two non-visual surfaces the same file already enumerates at `_design.md:24-38`; and a **new sign-off block is appended unsigned**, naming what is added and what is untouched."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/projection-store-freeze/_design.md"
  verifying_test: "git diff -- .bklg/from-contract-to-published-library/projection-store-freeze/_design.md shows an added-lines-only diff over :10-50 and :92-101 with the `surfaces: []` fence and the prior sign-off block byte-identical, and the appended block carrying no signature; confirmed at the design-stage review"

- id: AC-012
  criterion: "**GIVEN** this repository's rule that `.kb/` atoms are authored by `/redkiln:kb-ingest` from `.kb/_intake/` and that hand-writing them *\"produces the directory layout of the process without the process\"* (`CLAUDE.md`, reverted at `0269720`), **WHEN** this story merges, **THEN** the scope discipline is observable in the diff rather than asserted: no file under `crates/**`, `.kb/**` or `spec/**` changes, every changed path falls inside the PR-boundary block above, and any answer the record turned up that deserves an ADR appears **as a named gap for the runbook's ADR pass**, never as a minted decision."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/projection-store-freeze/projection-api-design-record/spec.md"
  verifying_test: "redkiln verify --grain story (reads the first fenced block under `PR boundary` in .bklg/from-contract-to-published-library/projection-store-freeze/projection-api-design-record/spec.md and this ledger) plus git diff --name-only main...HEAD; redkiln validate green on this story's item"
```
