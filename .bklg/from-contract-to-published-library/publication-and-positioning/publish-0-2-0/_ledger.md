---
item: HS-S0096
stage: implement
created: 2026-08-12
updated: 2026-08-12
---

# Acceptance ledger — 0.2.0 is published, with the whole gate green on the exact commit

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

**A note this story's ledger needs and no other in the project does.** Several rows below are
discharged by a **committed transcript** in `_release-log.md` rather than by a compiled test, because
no gate step in this repository may reach a live registry (NF-005) and the acts recorded there are
unreproducible once the terminal scrolls (NF-007). That is the deployment brief's stated shape, not
a weakening of the bar: the evidence cited must be a `file:line` into the committed log, never a
remembered observation.

```yaml
- id: AC-001
  criterion: "GIVEN P4 has decided to depend and types `cargo add happenstance`, and P1 will inherit whatever number resolves, WHEN the maintainer prepares the tree for release, THEN the version is the one `registry-surface-diff`'s committed dated report implies — quoted into `_release-log.md` with the report's finding before a manifest is touched, never chosen and never rounded — and it lands in five places in one edit that precedes the gate run: `Cargo.toml`:6, all three `[workspace.dependencies]` requirement strings at `Cargo.toml`:24-26, `Cargo.lock` and `experiments/wire-format/Cargo.lock`; AND `CHANGELOG.md`'s `## [Unreleased]` becomes `## [0.2.0] — <publish date>` with a fresh empty `## [Unreleased]` above it and the link references moved, dated with the day the publish runs so no one-line edit moves the tree afterwards"
  satisfied: false
  evidence: ""
  mount_point: "Cargo.toml"
  verifying_test: "cargo metadata --format-version 1 over the release tree (transcript in .bklg/from-contract-to-published-library/publication-and-positioning/publish-0-2-0/_release-log.md) + cargo xtask lints (changelog_names_every_rule, CF-29) + any --locked step of cargo xtask ci (xtask/src/main.rs:52-55)"

- id: AC-002
  criterion: "GIVEN five blocking stories each produced a dated artefact about the tree they were written against, and P4 will read the result of all five on a page nobody can edit, WHEN the maintainer reaches the publish commit, THEN each of `crate-set-decision`, `registry-surface-diff`, `clause-maturity-audit`, `deferred-clause-reread` and `rendered-page-preflight` is re-observed here — the artefact exists, names a commit, and that commit is an ancestor of this one with no intervening change to what it observed — AND the tree's own two preconditions hold: `package-check`'s `reconcile` agrees with `cargo metadata` on exactly the three decided crates, and none of the three published crates carries a `#![allow(clippy::todo)]` exemption; a missing, unreadable or stale precondition fails closed naming which one and what to do, never an empty check that passes"
  satisfied: false
  evidence: ""
  mount_point: "Cargo.toml"
  verifying_test: "cargo xtask ci step `package-check` (xtask/src/main.rs:515-532; xtask/src/package.rs:86) + the five-row precondition table in .bklg/from-contract-to-published-library/publication-and-positioning/publish-0-2-0/_release-log.md + a static read for allow(clippy::todo) across the three published crates"

- id: AC-003
  criterion: "GIVEN P4 cannot run our suite and has only what the specification says about how strong each promise is (U4), WHEN the release is cut, THEN `cargo xtask clause-audit --write --date <YYYY-MM-DD>` is re-run at the publish commit and `spec/audits/clause-maturity-<publish-date>.md` is committed as the release's artefact of record — the date passed as an argument, not read from a clock — reporting every clause's maturity across all 200 rather than only the frozen ones, and every `[FROZEN]` fingerprint identical to the tree this project received; any difference stops the release and becomes a decision atom and a re-plan, never an edit and never `--rebaseline`"
  satisfied: false
  evidence: ""
  mount_point: "Cargo.toml"
  verifying_test: "cargo xtask clause-audit --write --date <YYYY-MM-DD> → spec/audits/clause-maturity-<publish-date>.md, run inside the mandatory clause-audit gate step; totals reconciled against spec/SPECIFICATION.md:219-221"

- id: AC-004
  criterion: "GIVEN initiative DoD 13 asks for the gate green \"on the exact tree that was published\" (`initiative.md`:396-397), and P2 will pin this exact release, WHEN the maintainer runs the release bar, THEN the whole `cargo xtask ci` — no flags — is green on the literal publish commit and its dated output is committed to `_release-log.md` beside `git rev-parse HEAD`, as an artefact distinct from any `cargo xtask ci --fast` run `.redkiln/config.yaml`:55 fired at the project's integration stage; AND no `OPTIONAL` step in that transcript reports `skipped` — the two feature powersets, `cargo deny` and the nightly `--cfg docsrs` rustdoc build must have run, because the `docsrs` build is the only thing that compiles the `doc(cfg)` attributes docs.rs will set after publication and the `wasm32` powerset is the only step that compiles `happenstance-core`'s default `std + memory` on that target"
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs"
  verifying_test: "cargo xtask ci whole (run_ci, xtask/src/main.rs:828-834) on the publish SHA, transcript committed to .bklg/from-contract-to-published-library/publication-and-positioning/publish-0-2-0/_release-log.md with git rev-parse HEAD"

- id: AC-005
  criterion: "GIVEN P3 reads one line in the Guarantees block to self-identify as a `wasm32` consumer and then installs `happenstance` at the features they get by default, WHEN the gate runs on the release tree, THEN a fifth mandatory `wasm32` step compiles that crate at those features — `cargo check --locked -p happenstance --target wasm32-unknown-unknown` appended to `REQUIRED` with `probe: None`, its name added to `wasm_steps()` so `cargo xtask wasm` covers it, and the module doc's \"what the gate proves\" paragraph updated in the same change; AND if it does not compile, the release blocks and it becomes a decision atom and a re-plan — never a quiet edit to `default` to make the step green"
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs"
  verifying_test: "cargo xtask wasm and cargo xtask ci both listing the new step by name (xtask/src/main.rs:784-790 wasm_steps, :816 steps_named, REQUIRED at :105ff)"

- id: AC-006
  criterion: "GIVEN P4 will meet these three crates on a rendered page that cannot be edited after the fact, and `rendered-page-preflight` has already read those pages, WHEN the `.crate` artifacts are built, THEN what ships is what was read: each published crate's packaged `README.md` is byte-identical to the file preflight read, both licence files and the README are inside every tarball, all three manifests carry a `[package.metadata.docs.rs]` block so docs.rs builds under all features with `doc(cfg)` on every gated item, the `happenstance` manifest `description` is ≤ 120 characters and true of this tree (today's is ~131 — `crates/happenstance/Cargo.toml`:3), and none of the four known-stale strings appears on any packaged surface; this story changes no byte of any README, because a surface edited after its preflight read is a surface nobody read"
  satisfied: false
  evidence: ""
  mount_point: "Cargo.toml"
  verifying_test: "cargo xtask ci step \"packaged artifacts carry their licences and README\" (xtask/src/main.rs:515-532; xtask/src/package.rs:88-94) + packaged-README hashes against the preflighted files and a description character count, both recorded in .bklg/from-contract-to-published-library/publication-and-positioning/publish-0-2-0/_release-log.md + human review against .bklg/from-contract-to-published-library/publication-and-positioning/_design.md:104-169, :439-499, :641-680"

- id: AC-007
  criterion: "GIVEN a crates.io publish cannot be edited or deleted and a `yank` removes a version from future resolution while leaving every rendered page exactly as it was, WHEN the maintainer takes the act, THEN reversibility is bought first and in writing: `cargo publish --dry-run` is run and its transcript committed for `happenstance-core` and `happenstance-testkit` (the `-p happenstance` dry run is recorded as unavailable, never as failing, because it builds against registry-form dependencies and cannot resolve until core is live); the rollback route — what `yank` does and does not do, that a correction ships as a new forward `0.(2+n).0` rather than an edit, and that yanking is reserved for a broken artefact rather than for wording — is written into `_release-log.md` dated ahead of the first publish transcript; then `happenstance-core` → `happenstance-testkit` → `happenstance` in dependency order, as a human handoff; a registry failure partway is resumed at the crate that failed, never restarted at a new number; and `0.2.0-alpha.1` is deliberately not yanked, recorded as a decision with its reason so a later reader does not read the absence as an oversight"
  satisfied: false
  evidence: ""
  mount_point: "Cargo.toml"
  verifying_test: "cargo publish --dry-run -p happenstance-core and -p happenstance-testkit, then the ordered publish transcripts, all committed in dated order in .bklg/from-contract-to-published-library/publication-and-positioning/publish-0-2-0/_release-log.md (rollback route dated ahead of the first publish transcript)"

- id: AC-008
  criterion: "GIVEN the maintainer has just done something they cannot undo, and P4's whole first impression now lives on pages nobody in this repository controls, WHEN the crates are live, THEN the act is checked from outside: from a scratch directory owning none of this repository's lock files, all three crates resolve at the published number from the index with no unresolvable internal dependency; an annotated tag on the publish commit and a GitHub release whose body points at `CHANGELOG.md`'s new section land after the crates are live, never before, with the tag convention recorded because this repository has none to inherit; and the three docs.rs builds are observed within the hour and recorded — a failed docs.rs build is not a blocker and not a yank, it routes to `guarantees-and-docs-rs-presentation`'s configuration and to project AC-007"
  satisfied: false
  evidence: ""
  mount_point: "Cargo.toml"
  verifying_test: "manual registry resolution from a scratch project outside this workspace, plus the tag/release and docs.rs observations, all transcribed in .bklg/from-contract-to-published-library/publication-and-positioning/publish-0-2-0/_release-log.md"
```
