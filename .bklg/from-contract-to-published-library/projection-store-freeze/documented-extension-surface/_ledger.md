---
item: HS-S0015
stage: implement
created: 2026-08-12T13:46:09.930Z
updated: 2026-08-12T13:46:09.930Z
---

# Acceptance ledger — DT-8's arm discharged — the suite's bar held for the author it was chosen for

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Rows tagged **A** are live on DT-8's outside-author arm and rows tagged **B** on the narrow arm. A row
belonging to the arm **not** in force is still satisfied, by citing `_design.md`'s recorded resolution
as the evidence that it does not apply — the arm is therefore visible in this ledger, and no row is
ever left blank or deleted. AC-001 must carry its evidence before any other row may flip: it is the
row that records which arm was read.

```yaml
- id: AC-001
  criterion: "**A+B ·** GIVEN P2's protection rests on a scope decision a human signed off, WHEN the implementer's *first* act is to open `.bklg/from-contract-to-published-library/projection-store-freeze/_design.md` before writing any code, THEN the DT-8 arm in force is recorded — verbatim, with a `file:line` citation into that file — in this story's `_ledger.md` and at the head of `_extension-surface-gaps.md`, and every arm-conditional row below is discharged on that arm's terms; and IF `_design.md` carries no DT-8 resolution, THEN the story halts and reports an unmet dependency on `projection-api-design-record` (HS-S0014) rather than choosing an arm, because a sign-off that ratifies a decision the implementer already made is not a sign-off."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/projection-store-freeze/documented-extension-surface/_extension-surface-gaps.md — the recorded arm at the head of the gap note, citing .bklg/from-contract-to-published-library/projection-store-freeze/_design.md by file:line"
  verifying_test: "Reviewed diff over .bklg/from-contract-to-published-library/projection-store-freeze/documented-extension-surface/_extension-surface-gaps.md plus `redkiln verify --grain story` (no code test — the check is that the decision was read, and only a diff can show that)"
- id: AC-002
  criterion: "**A+B ·** GIVEN P2 meets this library on `docs.rs` and never opens a `.bklg/` artefact, WHEN they land on `happenstance-testkit`'s crate page, THEN the arm's deliverable is *there* as a composed section of the crate documentation — its own `#` heading taking its place in the existing six-heading hierarchy (`crates/happenstance-testkit/src/lib.rs:15`, `:31`, `:55`, `:84`, `:134`, `:154`), the extension surface written up in ordered steps that name the `conformance` feature flag and the `__private` export's role (Arm A) or the scope limit stated as a qualification rather than an apology (Arm B) — and it renders under `cargo doc --all-features` with every intra-doc link resolving and every code block compiling; a resolution recorded only in `_design.md` **fails** this criterion, which is precisely the failure DT-8 names."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/src/lib.rs — the crate-level `//!` documentation block (:8-135), which is the docs.rs page an outsider meets"
  verifying_test: "`cargo doc --workspace --all-features` and the nightly `--cfg docsrs` step inside `cargo xtask ci`, plus `cargo test --doc -p happenstance-testkit`; the composition half (heading, ordered steps, density budget) is the design-stage review over the rendered page, per .bklg/from-contract-to-published-library/projection-store-freeze/_decomposition.md:777"
- id: AC-003
  criterion: "**A ·** GIVEN P2 has a projection store and wants to know when they are finished, WHEN they add `happenstance-core` with `conformance` to `[dependencies]`, `happenstance-testkit` and `tokio` to `[dev-dependencies]` only, and write the single line `projection_store_conformance!(OutsideFixture::new())` in their own `tests/`, THEN the macro expands inside a foreign crate — which is the only thing in this workspace that exercises the `__private` re-export (`crates/happenstance-testkit/src/lib.rs:359-364`) — one test per registered projection rule is emitted, and **every one passes**, inside the gate's ordinary `tests` step (`xtask/src/main.rs:143-153`) rather than a bespoke command, with no fork of the harness, no vendored copy of a rule and no edit to the rule set."
  satisfied: false
  evidence: ""
  mount_point: "examples/outside-projection-adapter/ — a `publish = false` workspace member picked up by Cargo.toml:3's members glob; store and both trait impls in src/lib.rs, fixture and suite invocation in tests/"
  verifying_test: "examples/outside-projection-adapter/tests/outside_projection_conformance.rs, run by `cargo test --locked --workspace --all-features` inside `cargo xtask ci`. On the narrow arm: discharged by citing .bklg/from-contract-to-published-library/projection-store-freeze/_design.md's DT-8 resolution"
- id: AC-004
  criterion: "**A ·** GIVEN P2's real fear is that \"it compiles\" gets mistaken for \"it is correct\" (`personas-and-journeys.md:129-135`), WHEN a second store *in the same outside crate* commits the checkpoint and silently drops the read-model write — an outside author's own analogue of `CheckpointOnlyStore` — and is run through the same documented entry point, THEN the suite fails it **by a named rule**, that failing test's name is written into `_extension-surface-gaps.md`, and the failure is reached from the published surface rather than from the testkit's private mutant machinery; and IF the exactness meta-tests (`mutants_fail_exactly_their_declared_rules`, `mutant_registry_is_exhaustive`) prove unreachable from outside, THEN that unreachability is recorded as a first-class gap and **not** worked around by registering the outside fixture in `crates/happenstance-testkit/tests/mutation_coverage.rs`, which would invert the dependency edge this story exists to count."
  satisfied: false
  evidence: ""
  mount_point: "examples/outside-projection-adapter/ — the deliberately wrong store beside the conformant one in src/lib.rs, driven through the same documented entry point in tests/"
  verifying_test: "examples/outside-projection-adapter/tests/outside_projection_discrimination.rs, with the failing rule's name recorded in .bklg/from-contract-to-published-library/projection-store-freeze/documented-extension-surface/_extension-surface-gaps.md; bar set by .bklg/from-contract-to-published-library/projection-store-freeze/project.md:183-186. On the narrow arm: discharged by citing _design.md"
- id: AC-005
  criterion: "**A ·** GIVEN P2's storage genuinely cannot provide one guarantee and they need to say so without opting out of the bar, WHEN their fixture declines exactly one projection capability with its own stated reason and the suite runs, THEN the rule requiring it is **still emitted as a test**, returns `RuleOutcome::Skipped` carrying the fixture's own words, and the outside crate asserts on that **value** (`crates/happenstance-testkit/src/contract.rs:458-537`) — never on captured stdout — which requires the projection rules module to be publicly reachable from a foreign crate the way `pub use suite::rules;` (`lib.rs:189`) makes the event-store family reachable; the rendered `SKIP {rule}: fixture declines …` line (`contract.rs:500-507`) additionally appears under the gate's `--show-output` (`xtask/src/main.rs:132-153`) and is legibility, not the check; and IF that re-export is missing, THEN it is recorded as gap #1 and added in this PR with a `CHANGELOG.md` entry under CF-32's MINOR reasoning."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/src/lib.rs:189 — the `pub use suite::rules;` re-export pattern the projection family must match for a foreign crate to assert on a rule's returned value; consumed from examples/outside-projection-adapter/tests/"
  verifying_test: "examples/outside-projection-adapter/tests/outside_projection_capability_skip.rs — asserting the returned `RuleOutcome::Skipped` value with the reason compared against the fixture's own `const`, never against captured stdout; the printed line observed in the same `cargo xtask ci` run. On the narrow arm: discharged by citing _design.md"
- id: AC-006
  criterion: "**A ·** GIVEN P2 must decide whether conforming is affordable before they start, WHEN they read the outside crate's manifest and run `cargo tree -p outside-projection-adapter --edges normal`, THEN what they find is exactly what the specification promised — **one feature** (`conformance`) added to a dependency the adapter already has, **no `happenstance-testkit` node anywhere in the normal-edge graph**, and the testkit present only under `[dev-dependencies]` — with the gap note stating plainly that the claim is about the *non-dev* graph, so the documented `tokio` dev-dependency cost (`crates/happenstance-testkit/src/lib.rs:28-29`) is not misreported as a falsification; and the counterfactual is convicted by transcript rather than assertion: the probe impl is written in `tests/` once, deliberately, the resulting `error[E0117]` is captured verbatim into the note, and the impl then lives in `src/lib.rs` where the store type is local."
  satisfied: false
  evidence: ""
  mount_point: "examples/outside-projection-adapter/Cargo.toml — the committed manifest whose `[dependencies]` / `[dev-dependencies]` split is the evidence for spec/SPECIFICATION.md:5019-5025's cost claim"
  verifying_test: "Static: the committed examples/outside-projection-adapter/Cargo.toml plus the `cargo tree -p outside-projection-adapter --edges normal` output and the verbatim `error[E0117]` transcript, both quoted in .bklg/from-contract-to-published-library/projection-store-freeze/documented-extension-surface/_extension-surface-gaps.md. On the narrow arm: discharged by citing _design.md"
- id: AC-007
  criterion: "**A+B ·** GIVEN no tool can distinguish a fixture written from documentation from the same fixture copied out of `crates/happenstance-testkit/src/fixtures.rs`, WHEN the work begins, THEN `_extension-surface-gaps.md` **already** carries the named allowlist (rendered rustdoc only) and denylist (`fixtures.rs` foremost, `MemoryProjectionStore`'s source, HS-S0013's buffering variant, the testkit's own conformance harnesses) — a denylist written afterwards describes what happened, one written first constrains it — and WHEN the work ends, THEN every point at which the allowlist was insufficient is recorded with what was missing, which denylisted file or source read answered it, and the documentation change that would have prevented it; including the workspace-membership limit and the explicit non-claim on initiative DoD 9 (`initiative.md:383-386`), and including the standing tension that the crate doc today points an author straight at the reference fixture (`crates/happenstance-testkit/src/lib.rs:52-53`), which is the file the denylist forbids; and a run that found **no** gaps says so in those words, because \"a fixture built with no gaps found is either a triumph or a copy, and only the record tells you which\" (`discover.md:89-91`)."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/projection-store-freeze/documented-extension-surface/_extension-surface-gaps.md — the gap record, which is this story's primary output on both arms"
  verifying_test: "Reviewed diff over the gap note checked against commit order — the allowlist/denylist commit must precede the fixture commit, the ordering being the check, as it is for AC-008's ADR sequencing in .bklg/from-contract-to-published-library/projection-store-freeze/_decomposition.md:778; criterion set by the Testing brief row at _decomposition.md:777 (\"not copied from `fixtures.rs`\")"
- id: AC-008
  criterion: "**A+B ·** GIVEN this repository's rule that a wrong rule is fixed with its reason in the same change, WHEN a gap is found, THEN a **documentation** gap or a **missing public re-export** is fixed in this PR and cited from the gap record — with `CHANGELOG.md` updated if any testkit item became `pub` (`spec/SPECIFICATION.md:8200-8222`, CF-32) and one line added to `CLAUDE.md`'s repository map if Arm A adds the example crate, because a map that omits a workspace member is a map that lies — while a gap requiring a **port signature change, a new conformance rule, or a `[FROZEN]` clause amendment** is reported at the story boundary for HS-S0016 and the runbook's ADR pass and is **not** absorbed; and the whole gate is green: `cargo xtask ci` run whole (not `--fast`, which omits the widened feature powerset this story enlarges), `Cargo.lock` committed with the new member so `--locked` holds, `redkiln verify --grain story` green on the PR boundary, and no `_design.md` edit, no `.kb/**` atom and no `spec/SPECIFICATION.md` edit in the diff."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/src/lib.rs (and contract.rs) for the fixed-in-place documentation gaps, CHANGELOG.md for any newly `pub` item, and the PR boundary block in spec.md that `redkiln verify --grain story` enforces"
  verifying_test: "`cargo xtask ci` run whole — tests at xtask/src/main.rs:143-153, feature powerset at :546-556, docs and spec-trace — plus `redkiln verify --grain story` over the PR boundary, plus the reviewed diff for the fixed-versus-reported split and the CHANGELOG.md / CLAUDE.md lines"
```
