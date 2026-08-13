---
item: HS-S0030
stage: implement
created: 2026-08-12
updated: 2026-08-12
---

# Acceptance ledger — The compile-fail case, its negative control, and its gate row

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Two notes specific to this story, both of which change what counts as evidence:

- **`cargo xtask affected --base main` is not evidence for AC-005.** It runs no proof-artefact step
  (`xtask/src/affected.rs:119-125`). Every row whose verifying test runs through the gate must cite a
  `cargo xtask proof-artefact` (or `cargo xtask ci --fast`) invocation.
- **AC-004 has no automated tier by design.** Its evidence is the recorded mutation transcript — the
  `_ =>` wildcard added to the fail fixture, the command, the observed red output, and the revert —
  reproduced in `implementation-report.md`. A green mutation is EC-004, not a satisfied row.

```yaml
- id: AC-001
  criterion: "P1's domain grows and the build stops. GIVEN P1 has modelled a consistency boundary on the typed layer and shipped it, WHEN a variant is later added to the domain enum and `DecisionModel::apply`'s fold is not extended, THEN the build stops at `error[E0004]: non-exhaustive patterns` instead of compiling and silently ignoring the new fact — and this repository *asserts* that rather than believing it: a `trybuild` `compile_fail` fixture restates the example's enum plus one variant, keeps the fold otherwise verbatim with **no `_ =>` arm**, and is pinned by a checked-in `.stderr` matched byte for byte."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/proof.rs — the new `Artefact` row in `ARTEFACTS` (:133-149), consumed by the REQUIRED gate step at xtask/src/main.rs:178-190"
  verifying_test: "examples/course-subscriptions/tests/ui.rs::ui::an_unhandled_variant_fails_to_compile (fixture + .stderr under examples/course-subscriptions/tests/ui/), run by `cargo xtask proof-artefact`"

- id: AC-002
  criterion: "The diagnostic names a file P1 wrote, and it is the first thing they read. GIVEN the build has stopped, WHEN P1 reads the compiler's output top-down, THEN the **first** error block is the non-exhaustive-match error and its `-->` span names a path under `examples/course-subscriptions/` at the `match` arm they own — never a macro body, never anything under `crates/` — and nothing precedes it: no unrelated error, no warning from an unused import in the fixture. A guarantee whose diagnostic names a file the user did not write is a guarantee they cannot act on (AC-U07; anti-pattern 13; the surface's own selector)."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/proof.rs — the new `Artefact` row in `ARTEFACTS` (:133-149); the rendered surface is `compile-fail-diagnostic` (_design.md:78-83)"
  verifying_test: "examples/course-subscriptions/tests/ui/*.stderr, asserted byte-for-byte by examples/course-subscriptions/tests/ui.rs::ui::an_unhandled_variant_fails_to_compile; read as an artefact at the slice review against _design.md:999-1001"

- id: AC-003
  criterion: "A red build means what it says (the negative control). GIVEN a reader wants to know the build was stopped by the *missing arm* and not by a typo, a renamed import or an item moved behind a feature, WHEN the pair runs, THEN a second fixture — identical except that the added variant **is** handled by a real arm — compiles and passes, so any unrelated breakage turns both cases red and is legible as breakage rather than banked as the guarantee. One edit is the whole delta between the two files."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/proof.rs — the new `Artefact` row in `ARTEFACTS` (:133-149), consumed by the REQUIRED gate step at xtask/src/main.rs:178-190"
  verifying_test: "examples/course-subscriptions/tests/ui.rs::ui::the_negative_control_compiles (a `trybuild` `pass` entry over the control fixture)"

- id: AC-004
  criterion: "The discriminator is observed to discriminate, not assumed to. GIVEN this repository has already shipped one proof artefact that could not fail and found out two phases later, WHEN this story closes, THEN the protection has been deliberately removed — a `_ =>` wildcard added to the **fail** fixture — `cargo xtask proof-artefact` has been run and observed **red**, the edit reverted, and the exact command plus the observed output recorded. AC-002 of the project is a claim about discrimination, and a discriminator nobody ran is an assertion."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/proof.rs — the `ARTEFACTS` row is what goes red; the observation itself is recorded in this story's implementation-report.md"
  verifying_test: "Recorded mutation transcript: `_ =>` added to examples/course-subscriptions/tests/ui/<fail-fixture>.rs, `cargo xtask proof-artefact` observed red, edit reverted — cited here and reproduced in implementation-report.md (discipline: .kb/decisions/0010-the-suite-must-prove-itself.md)"

- id: AC-005
  criterion: "The instrument survives a tidy-minded maintainer. GIVEN a maintainer six months out meets two fixtures that nothing else in the tree references, WHEN they delete the target, truncate it, `#[ignore]` a test or rename one, THEN the gate fails and *names what is missing* — because the new `ARTEFACTS` row names the **tests**, asserted out of `cargo test -- --list` before anything runs, not the target. A later third test needs no gate edit, because the check is a subset one."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/proof.rs — the new `Artefact` row plus its name `const` in `ARTEFACTS` (:133-149, shape at :57-70), reached by the existing REQUIRED step at xtask/src/main.rs:178-190"
  verifying_test: "`cargo xtask proof-artefact` (the name assertion at xtask/src/proof.rs:199-216), exercised end to end by `cargo xtask ci --fast`"

- id: AC-006
  criterion: "P4's sitting and P2's pin both cost nothing. GIVEN P4 has one bounded sitting and P2 pins `happenstance-core` and must not be disturbed, WHEN `trybuild` enters the tree, THEN it enters **once** in `[workspace.dependencies]` under the `# --- dev / tooling only ---` header and is consumed only as a `[dev-dependencies]` of the `publish = false` example — so no published crate's graph, feature set or MSRV floor moves — `cargo deny check licenses` and `cargo deny check advisories` pass against the **unmodified** `deny.toml`, and no `pub` item, re-export or feature is added anywhere."
  satisfied: false
  evidence: ""
  mount_point: "Cargo.toml `[workspace.dependencies]` dev/tooling block (:80-98) and examples/course-subscriptions/Cargo.toml `[dev-dependencies]`; no row added to _design.md's `## Items` block (:229-379)"
  verifying_test: "`cargo deny check licenses` + `cargo deny check advisories` against an unmodified deny.toml:8-19; the packaging step at xtask/src/main.rs:519-529 via `cargo xtask ci --fast`"

- id: AC-007
  criterion: "The pinned diagnostic is a composed artefact, not bare output, and it stays where it belongs. GIVEN the only thing a reviewer can read to judge this surface is the committed text, WHEN they open it, THEN: the `.stderr` is rustc's own render, **generated** under the pinned `1.97.1` channel and regenerated only via `TRYBUILD=overwrite` reviewed as a diff — never hand-written and never hand-edited to make a case pass; the fixture is a complete program in the shape P1 would actually write, using the example's own domain and `rustfmt`-clean at the 100-column budget, not a synthetic minimal repro; every fixture source line the diagnostic quotes is ≤ 80 columns, so the quoted line and its caret do not wrap at the terminal budget this design sets for text read in a terminal; and the surface's disposition stays **opened on demand** — this PR adds no region, bullet or link to `crate-root-rustdoc` or `crate-readme`."
  satisfied: false
  evidence: ""
  mount_point: "The `compile-fail-diagnostic` surface (_design.md:78-83), rendered as examples/course-subscriptions/tests/ui/ and its committed .stderr; opened-on-demand, so no persistent-chrome surface is touched"
  verifying_test: "`cargo fmt --check` and `cargo xtask affected --base main` for the 100-column budget and lint cleanliness; the committed .stderr and fixture read at the slice review against _design.md:210-214, :810-816 and :845-864; an empty crates/happenstance/** diff, enforced by the spec's PR boundary"
```
