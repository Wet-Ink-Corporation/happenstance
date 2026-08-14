---
item: "HS-S0003"
stage: report
created: "2026-08-13"
updated: "2026-08-13"
---

# Report — DT-3 and DT-8 resolved in the public-API design record

## Findings Ledger

Twelve ACs, all satisfied. Nothing blocked, nothing deferred, nothing stubbed. The
deliverable is one artifact — the project's already-signed-off `_design.md` — and
the mount is the artifact itself: three later stories read decisions out of this
file rather than making them, so a record written anywhere else would be
constructed-but-unmounted.

| Finding | Evidence | Follow-up |
| ------- | -------- | --------- |
| **DT-3 is resolved with one authoritative source and all three reason-writers disposed of.** The suite's own output is authoritative; per-adapter documentation may repeat it and may not contradict it. The fixture-written `Capability::declined` is the default; the testkit-written `NO_CEILING_REASON` survives as a **stated exception** for a fact no adapter should paraphrase; per-adapter prose is subordinate and non-normative. One policy, one stated exception — not two policies by omission. | `_design.md` § *DT-3*; `crates/happenstance-testkit/src/contract.rs:355-419`, `:435-456` | `projection-capability-skips` (HS-S0008) implements the table. |
| **CF-40 is cited as authoritative and not re-litigated.** The record states only what the projection fixture does *given* that CF-40's ownership is contradicted by two accepted, immutable decisions: it declares no numeric-limit constants at all, and the projection port has no `ExceedsStoreLimit` analogue, so the surface never arises. No reading is preferred. | `.kb/open-questions/cf-40-fixture-limits-ownership.md`; `spec/SPECIFICATION.md:4667-4690` | None. The question stays open with its own owner. |
| **The projection capability set is enumerated, not implied.** `RESET_REFUSAL` (fixture, declinable, fixture-written reason, gates `refused_reset_changes_nothing`); `SECOND_HANDLE` (fixture, **a MUST** — a fixture that cannot open a second handle cannot observe PS-1 at all); `READS_THROUGH_BATCH` (a **`ProjectionProbe`** const, not a fixture const, declinable, **testkit-written** reason on the `NO_CEILING_REASON` precedent). | `_design.md` § *The projection fixture's capability set*; `spec/SPECIFICATION.md:5200-5217`, `:5052-5074` | HS-S0008 names every constant by quoting this table; a constant that story has to invent would be this AC failing late. |
| **The "nothing declinable" failure was checked and does not obtain — and is recorded anyway.** Two of the three constants are declinable, so the reporting discipline is not decorative on this port and initiative AC-05 stays demonstrable. Written into DT-3's resolution as the finding it would have been. | `_design.md`, note 3 under the capability table; `_decomposition.md` UX brief, *What would make this brief wrong* | None. |
| **DT-3 survives the constrained target.** `RuleOutcome::report` is a **measured** no-op on `wasm32-unknown-unknown`; the projection wasm harness routes `skip_line` through `console_log!` as `__emit_wasm` already does; the machine-checked half is a `RuleOutcome`-**value** assertion, a sibling of `capability_skips_are_reported`, never stdout. | `crates/happenstance-testkit/src/contract.rs:511-531`; `…/tests/memory_conformance_wasm.rs:25`; `…/tests/mutation_coverage.rs:3184` | The value assertion is `projection-capability-skips`'; the harness is `projection-suite-entry-point`'s. This is the single place a plausible-sounding DT-3 answer is wrong on the target BR-12 protects. |
| **DT-8 takes the outside-author arm, and pays for it.** The extension surface is `projection_store_conformance!` + `ProjectionProbe`, bounded at *one feature flag on a dependency the adapter already has and no new edge in the dependency graph*, with the orphan-rule argument quoted and a concrete `[dev-dependencies]` block. | `_design.md` § *DT-8*; `spec/SPECIFICATION.md:5015-5031` | **A live obligation:** `documented-extension-surface` (HS-S0015) must build a fixture from the documentation alone and clear the mutant-registry exactness check and the capability-skip rule. HS-S0015 is scopable from this section alone. |
| **The type surface is transcribed, not redesigned.** `## Items` carries nine ids ordered as the trait declares them; `## Signatures` transcribes the target trait item for item plus `ProjectionProbe`'s five items, with **56** `PS-` ids and no unattributed signature. | `_design.md` §§ *Items*, *Signatures*; `spec/SPECIFICATION.md:4632-4731`, `:4998-5013` | `owned-batch-port-shape` (HS-S0004) implements the block; its Surface quality section is written against it. |
| **Four unusual shapes each name the alternative that lost, once, at the call site.** The tuple that can spell `(None, true)`; two error enums so `commit`'s caller never matches `Refused`; `begin` infallible because async + fallible implies a round trip Neon's one-shot transport cannot afford; `reset` taking the caller's deletes because the port has no idea what the read model is. Each is assigned the item's own rustdoc as its home, not only an ADR. | `_design.md` § *What it costs a caller*; RS-70-5 at `standards/rust/70-rustdoc-obligations.md:243` | The rustdoc itself is `owned-batch-port-shape`'s. |
| **`ResetError::Refused` is decided out loud: it stays bare.** With where the reason lives instead, the argument (a `&'static str` would push a domain sentence through a port type that cannot validate, localise or synchronise it), the cost to an operator (one more hop), and the reversal path (`#[non_exhaustive]`). Silence would have been the failure. | `_design.md` § *The states the API must express*, final paragraph; `spec/SPECIFICATION.md:4676-4682`, `:5210-5216` | Behaviour is `reset-rules` (HS-S0012)'s `refused_reset_changes_nothing`. |
| **`## Visibility and stability` is specific enough to be violated.** Three lines per item for all eight; `#[non_exhaustive]` on all four new types; both halves of the mount named — the `lib.rs:98-124` export block **and** the feature table — because an item mounted at one and not the other is an item no adapter can name; `unstable-projection` recorded as AC-A04's arm; and the intra-doc-link-into-a-feature-gated-module hazard stated with the shape that survives it. | `_design.md` §§ *Placement and re-export*, *Visibility and stability*; Architecture brief Notes 1 and 9; `crates/happenstance-testkit/src/lib.rs:112-132` | Compile-time proof deferred to HS-S0004 and `projection-probe-conformance-feature`. |
| **The doctest is written and assigned a compiling home.** Example text in full, home named (`crates/happenstance-core/src/projection.rs` plus the `MemoryProjectionStore` module), gate step named (`cargo test --doc` inside `cargo xtask ci`), and the record says plainly that it cannot compile it. The owed `compile_fail` is specified **bare**, never `compile_fail,E0080`, with where an associated const's check actually fires. | `_design.md` § *The doctest*; `crates/happenstance-testkit/src/contract.rs:400-419` | Compilation is `memory-projection-store`'s (Testing brief row AC-012). |
| **Ten anti-patterns, every one with a path or an atom id.** Including the two that most need writing down because they compile: `type Batch: Send;` (which `trait_variant` copies into the `!Send` flavour and breaks wasm32) and any borrowing GAT on the fixture (one of five ingredients of a rustc ICE still reproducing on 1.97.1). | `_design.md` § *Anti-patterns* | None. Flagged to the reviewer as an addition inside a section the prior sign-off referenced while it read `N/A`. |
| **The amendment is additive and re-gated, never a rewrite.** Exactly **ten** deletions in the whole file, all the placeholder `N/A — no user-facing surface.`; lines 1–50 byte-identical to HEAD; `## Sign-off` untouched; a new `## Amendment sign-off` appended and explicitly **unsigned**. | `git diff -U0` over `_design.md`; `cmp` over lines 1–50 | Back to the human at the design gate as an amendment. Declining it costs nothing already banked. |
| **Scope discipline is observable in the diff.** Empty `git diff --stat main...HEAD` under `crates/`, `spec/`, and `.kb/` outside `_intake/`. No atom hand-written, no open question's status changed, no ADR minted, no clause or maturity marker edited. | `design-checks.sh` AC-012 assertions | None. |

**Mount point:** `.bklg/from-contract-to-published-library/projection-store-freeze/_design.md`
— the artifact the design-stage review and every downstream story already open.
`projection-capability-skips` takes the capability set from it,
`documented-extension-surface` is scoped entirely by which DT-8 arm it records, and
`owned-batch-port-shape` implements the signature block it transcribes.

**Deferred:** nothing of this story's. Four machine checks are *visibly* deferred
with the story that runs each: the `RuleOutcome`-value assertion and the wasm
harness (HS-S0008, `projection-suite-entry-point`), the compiled doctest
(`memory-projection-store`), the documentation-only fixture (HS-S0015), and
compile-time proof of the signature and visibility blocks (HS-S0004).

**One dependency handled rather than hidden:** ADR-0017 / 0018 / 0019 exist as
long-form records but their `.kb/decisions/` atoms are **staged, not yet accepted**
— `projection-decision-atoms` is blocked on a human-invoked `/redkiln:kb-ingest`
wave. `## Signatures` states the authority order explicitly: the record was written
against the records and the specification, agrees with both, **no sentence has had
to yield**, and if the wave's atoms differ this block yields and the yielding
sentence is named rather than the atom edited.

## Acceptance

| AC | Status | Verification as run |
| --- | --- | --- |
| AC-001 | satisfied | Static: DT-3 section present; `Capability::declined`, `NO_CEILING_REASON`, `per-adapter` and `cf-40-fixture-limits-ownership` all present inside it. Red: none present. Design-stage review is the other half (project DoD 7). |
| AC-002 | satisfied | Static: `wasm32`, `__emit_wasm`, `RuleOutcome`, `capability_skips_are_reported` all present in DT-3's resolution. Machine half deferred by design to HS-S0008 and the wasm harness. |
| AC-003 | satisfied | Static: `refused_reset_changes_nothing`, `READS_THROUGH_BATCH`, `SECOND_HANDLE` all present; three constants enumerated with MUST-ness, reason-author and gated rule; the empty-set outcome recorded. |
| AC-004 | satisfied | Static: DT-8 section, `projection_store_conformance!`, the *no new edge in the graph* bound, `documented-extension-surface` all present. Conditional machine half is HS-S0015's. |
| AC-005 | satisfied | Static: `type Batch;`, `fn begin(&self) -> Self::Batch`, `fn reset(`, `ProjectionProbe`; **56** `PS-` ids, no unattributed signature. Compile-time proof deferred to HS-S0004. |
| AC-006 | satisfied | Static: four rationale markers present (`(None, true)`, the two-enums sentence, `one-shot`, *no idea what the read model is*); four items, each stated once, each with a rustdoc home. |
| AC-007 | satisfied | Static: `ResetError::Refused` carries a decision sentence, not only a signature — bare, with where the reason lives instead and what it costs. |
| AC-008 | satisfied | Static: `#[non_exhaustive]`, `lib.rs:98-124`, `[features]`, `doc(cfg(`, `unstable-projection`, `intra-doc` all present; three lines per item for all eight; both halves of the mount named. |
| AC-009 | satisfied | Static: `## The doctest` names `crates/happenstance-core/src/projection.rs` and `cargo test --doc`, and states the bare-`compile_fail` rule. Compilation is `memory-projection-store`'s. |
| AC-010 | satisfied | Static: `async_trait`, `type Batch: Send`, the ICE, `ProjectionId::new` all present; ten entries, each with a path or atom id. |
| AC-011 | satisfied | `git diff -U0`: exactly ten deletions, all the `N/A` placeholder; `cmp` over lines 1–50 identical; `## Sign-off` untouched; new block appended and marked `**Unsigned.**`. |
| AC-012 | satisfied | `git diff --stat main...HEAD` empty under `crates/`, `spec/`, `.kb/` outside `_intake/`; every changed path inside the PR boundary; `redkiln validate` green on this item. |

## Knowledge Harvest

- **DT-3's resolution is a reusable declension policy**, not a projection-local
  one: *the run is authoritative; the fixture writes the reason; the testkit writes
  it only where every store would say the same sentence; adapter prose is
  subordinate.* That is a `concept` candidate for closeout, because the third clause
  is the general form of the argument `NO_CEILING_REASON` won once and
  `READS_THROUGH_BATCH` has now won again.
- **DT-8's arm creates a testable definition of "held for an outsider"** — a fixture
  built from the documentation alone, clearing the mutant registry. Worth promoting
  as a `playbook`: it is the only instrument that distinguishes a bar held in fact
  from a bar claimed in prose, and it generalises to every extension surface this
  library ships.
- **A `concept` candidate on mounting in a library.** *"Composition root" means two
  places — the `lib.rs` export block and the feature table that gates it — and an
  item at one and not the other is invisible.* It came from the Architecture brief
  and is applied item-by-item here for the first time; the intra-doc-link-into-a-
  gated-module hard error belongs with it.
- **A process observation, not yet an atom.** This record had to be written while
  its upstream ADR atoms were staged rather than accepted, and the authority order
  held only because the record said in writing which way it would yield. Whether a
  design record should be allowed to precede the atoms it defers to — or whether the
  dependency should be on the *records* rather than the atoms — is a candidate open
  question for closeout.
