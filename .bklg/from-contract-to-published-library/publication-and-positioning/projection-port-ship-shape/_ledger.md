---
item: "HS-S0085"
stage: implement
created: "2026-08-12T13:47:23.229Z"
updated: "2026-08-12T13:47:23.229Z"
---

# Acceptance ledger — PS-3 gets a verdict on evidence: frozen, or behind unstable-projection

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Three notes specific to this story, none of which relaxes a row:

- **AC-004's verifying step is *probed*.** A machine without a nightly toolchain skips the
  `docs.rs configuration (nightly)` step. A skip is a skip — it is never this row's evidence, and
  `cargo xtask ci --fast` drops the step by design (`.redkiln/config.yaml`:55).
- **AC-005 and AC-006 are graded by looking at a rendered page.** Their evidence is a dated
  observation naming the `target/doc/**` paths that were read, because `xtask/src/package.rs`:4-18
  is explicit that presentation cannot be replaced by an automated containment check.
- **AC-002's `verifying_test` runs after a human invokes `/redkiln:kb-ingest`.** The implementer
  stages `.kb/_intake/` and hands off; the atom id the wave actually assigns is what this row cites.

```yaml
- id: AC-001
  criterion: "GIVEN the maintainer must settle PS-3 at phase 12 and the three sibling evidence documents exist under references/evaluation/, WHEN they evaluate PS-2's frozen three-part bar, THEN the record answers each part separately — hostile CheckpointOnlyStore failing commit_is_atomic_with_the_read_model; the suite green against an adapter holding a live transaction; the suite green against an adapter that cannot hold anything across an await — as met / not met / no evidence, each carrying a file:line citation into one of those three documents, and the arm follows from the three answers mechanically rather than from preference"
  satisfied: false
  evidence: ""
  mount_point: ".kb/decisions/00NN-…md (the atom the ingest wave authors) plus its row in .kb/maps/decision-map.md"
  verifying_test: "static citation check: review of .kb/_intake/<nnnn>-the-projection-port-ship-shape-at-0-2-0.md and the resulting atom against spec/SPECIFICATION.md:4760-4775 — three answered bar rows, each citing references/evaluation/**, none resting on RUNBOOK.md:601"

- id: AC-002
  criterion: "GIVEN a maintainer six months later asking why is the projection port gated (or frozen), WHEN they open .kb/maps/decision-map.md and follow one row, THEN they reach an accepted decision atom, numbered above the corpus and clear of the 0017–0028 sibling allocation, that states the verdict, says explicitly whether PS-3's SHOULD still binds or its condition has lapsed, and names the arm that lost with its cost — and the atom was produced by /redkiln:kb-ingest from .kb/_intake/, never hand-written"
  satisfied: false
  evidence: ""
  mount_point: ".kb/decisions/00NN-…md plus its row in .kb/maps/decision-map.md — an atom with no map row is an atom the corpus cannot see (.kb/maps/decision-map.md:81-86)"
  verifying_test: "process: `redkiln validate --kb && redkiln doctor` clean (exactly six template-drift advisories, zero dependency-cycle), plus the ingest wave's commit showing the atom authored and .kb/_intake/ cleared"

- id: AC-003
  criterion: "GIVEN the maintainer's standing obligation that nothing frozen is amended to make a release date, WHEN the verdict is written — including the case where the bar is met and the verdict is freeze — THEN the release path halts rather than un-gating: no feature is deleted, no gated module is exposed unconditionally, and spec/SPECIFICATION.md is byte-identical (PS-3's [PROVISIONAL] marker, its falsifier text and §1.3's 200 / 198 / 139 / 49 / 10 / 2 census all unmoved), with no line inside RUNBOOK.md:588-610 touched; PS-3's dead falsifier — a condition naming before 0.1, an event that can no longer occur — is written into the intake document for the wave to route, not repaired in passing"
  satisfied: false
  evidence: ""
  mount_point: "the PR's own diff: spec/SPECIFICATION.md absent from the changed set; RUNBOOK.md hunks outside :588-610; the falsifier observation staged in .kb/_intake/"
  verifying_test: "static diff: `cargo xtask spec-trace` green plus a recorded `git diff --stat main...HEAD` showing spec/SPECIFICATION.md unmodified and no hunk inside RUNBOOK.md:588-610"

- id: AC-004
  criterion: "GIVEN the evaluator's docs.rs page for happenstance is built by a renderer that runs after the one act that cannot be undone, WHEN cargo +nightly is present on the machine or the CI gate job, THEN xtask/src/main.rs's `docs.rs configuration (nightly)` step names all three publishable crates and its comment states why the third joined, and the build is green under RUSTDOCFLAGS='--cfg docsrs -D warnings' cargo +nightly doc --locked --all-features --no-deps — so a doc(cfg) spelling that does not compile fails here, not on a version that can be yanked but never removed"
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs — the `docs.rs configuration (nightly)` step (:596-638, name at :621), extended with `-p happenstance`"
  verifying_test: "build (nightly, probed): `RUSTDOCFLAGS=\"--cfg docsrs -D warnings\" cargo +nightly doc --locked -p happenstance-core -p happenstance -p happenstance-testkit --all-features --no-deps`, captured and committed under this story's directory"

- id: AC-005
  criterion: "GIVEN the evaluator has one sitting and lands on docs-rs-happenstance (primary) looking for the projection surface, WHEN the page renders in its all-features state, THEN every unstable-projection-gated public item is present and carries its rendered doc(cfg) gate annotation naming the feature — on the facade as well as on docs-rs-happenstance-core — so the reader learns gated, never not supported; no gated item is absent, and no item's gate is carried by prose alone or by a glyph or colour"
  satisfied: false
  evidence: ""
  mount_point: "the rendered surfaces docs-rs-happenstance and docs-rs-happenstance-core (_design.md:138-152), read locally at target/doc/happenstance/** and target/doc/happenstance_core/** from AC-004's build"
  verifying_test: "human observation of the rendered page, dated and recorded: every unstable-projection item present with its doc(cfg) pill on both surfaces — AP-8 (_design.md:660-661), IQ-2 (_decomposition.md:241-258)"

- id: AC-006
  criterion: "GIVEN the evaluator has just read that the port is gated and now needs to know what depending on it costs, WHEN they read the same item — 0 hops, no link followed — THEN the item's own prose states that the surface is exempt from semver while gated and that PS-2's bar is what retires the exemption, in ≤ 3 rendered lines, in words rather than glyph or colour, adding no second maturity count to the page and no second primary-ranked element to a screenful, with the module-doc ladder's first # heading still inside 12 rendered lines; and in arm B this text does not ship at all, which is recorded as a deliberate absence rather than an omission"
  satisfied: false
  evidence: ""
  mount_point: "the gated public items of crates/happenstance (the crate a consumer installs) and crates/happenstance-core/src/projection.rs's module documentation, as rendered on the two docs.rs surfaces"
  verifying_test: "human observation plus density count against _design.md:566-579 (exemption stated on the item), :475 (docs.rs lead paragraph and first-heading budget), :520 (one primary per screenful) and :641-676 (AP-3, AP-7, AP-15)"

- id: AC-007
  criterion: "GIVEN a consumer already resolving 0.2.0-alpha.1, WHEN they take 0.2.0 after this story, THEN the feature graph they resolve is identical — no feature added, renamed or removed on either published crate and none moved into or out of default — both feature-powerset steps stay green including --no-default-features --features unstable-projection, the reachable public item set is unchanged so registry-surface-diff reports nothing added or removed, and any published sentence this verdict makes false is handed over as a named finding to its owning story rather than edited here"
  satisfied: false
  evidence: ""
  mount_point: "the [features] blocks of crates/happenstance/Cargo.toml (:22-26) and crates/happenstance-core/Cargo.toml, plus the two feature-powerset steps in xtask/src/main.rs (:546-556, :564-591)"
  verifying_test: "feature matrix plus static diff: `cargo hack --feature-powerset` host and wasm32 steps green (neither skipped), an empty `git diff` over both [features] blocks, and the implementation report's named-finding handover list"
```
