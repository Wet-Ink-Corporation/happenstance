---
item: "HS-S0033"
stage: implement
created: "2026-08-12T13:46:31.358Z"
updated: "2026-08-12T13:46:31.358Z"
---

# Acceptance ledger — 0.2.0-alpha.1 on the registry, with its churn mitigations

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

**Three notes specific to this story, because it is the project's release story.**

1. **Four rows are verified by evidence that no gate step can produce.** AC-001's `cargo metadata`
   read, AC-004's cold read of `## Stability`, AC-006's gate transcript with its SHA and AC-007's
   registry resolution all land in
   `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/publish-0-2-0-alpha-1/_release-log.md`,
   which does not exist until the implementer creates it. That is DEP-006's stated shape
   (`_decomposition.md:964-977`), not a weakened bar: `cargo xtask ci` cannot reach a live registry.
2. **AC-007 is the only row whose evidence cannot be regenerated.** A `cargo publish` transcript and
   a first resolution against the index are one-shot. Paste them; do not summarise them.
3. **No row may be flipped before all four blocking stories are merged.** The tree this story
   publishes is the tree they left behind — see the spec's *Dependencies*.

```yaml
- id: AC-001
  criterion: "GIVEN P4 has finished evaluating and types `cargo add happenstance@0.2.0-alpha.1` in a project that has never seen this workspace, WHEN Cargo resolves the manifest against the registry, THEN all three crates resolve at `0.2.0-alpha.1` and no internal dependency is unresolvable — which requires that `Cargo.toml:6`, all three `[workspace.dependencies]` requirement strings at `Cargo.toml:24-26`, and both lock files (`Cargo.lock`, `experiments/wire-format/Cargo.lock:70`) carry the pre-release before any publish runs, because a `version = \"0.2.0\"` requirement never matches a `0.2.0-alpha.1` release and the mistake is only discovered after `happenstance-core` is permanently live."
  satisfied: false
  evidence: ""
  mount_point: "Cargo.toml — `[workspace.package] version` (:6), with `[workspace.dependencies]` (:24-26), Cargo.lock and experiments/wire-format/Cargo.lock:70"
  verifying_test: "`cargo metadata --format-version 1` over the release tree (no internal `^0.2.0` requirement) + every `--locked` step in `xtask/src/main.rs` + `cargo publish --dry-run -p happenstance-core`; transcripts in .bklg/from-contract-to-published-library/typed-layer-and-alpha-release/publish-0-2-0-alpha-1/_release-log.md"

- id: AC-002
  criterion: "GIVEN P2 pins `happenstance-testkit` exactly because a new conformance rule is \"thirty-four new ways for a previously passing adapter to go red\" (crates/happenstance-testkit/Cargo.toml:5-13), WHEN they read the number this release publishes, THEN it is `0.2.0-alpha.1` — a pre-release, so the suite's maturity matches the port it measures and no stable promise is made about a rule set over a moving contract — and it is still written as a literal `version` key in `[package]`, never `version.workspace = true`."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/Cargo.toml:14 (its own CF-32 number, independent of the workspace)"
  verifying_test: "`cargo xtask ci` step \"the testkit carries its own version\" (CF-32, xtask/src/main.rs:425-435)"

- id: AC-003
  criterion: "GIVEN P4 lands on the crates.io page with one pass and no second chance, WHEN they scan it top to bottom, THEN they meet `## Stability` before the first code block they would copy — region order `# happenstance` → the DCB sentence (lines 1-4 verbatim) → `## Which crate do I want?` → `## Stability` → `## What DCB buys you` → `## Guarantees` → `## Design` → `## Licence` — the facade blockquote (:6-11) is deleted rather than annotated, the word facade appears nowhere on the page, exactly one stability claim exists, the first fence sits no lower than it does today (crates/happenstance/README.md:30) and no fence exceeds 80 columns."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/README.md — the `crate-readme` surface (_design.md:54-58), rendered on crates.io and compiled as a doctest via crates/happenstance/src/lib.rs:10"
  verifying_test: "`cargo test -p happenstance --doc` (the README is compiled) + `rg -n \"facade\" crates/happenstance/README.md` returning nothing + heading-order and anti-pattern 7/8 review against _design.md:782-791 and :987-989; rendered page recorded in _release-log.md"

- id: AC-004
  criterion: "GIVEN P4 has found the section and is deciding whether the number is safe to depend on, WHEN they read `## Stability`, THEN it carries exactly three things and no fourth: the phase at which the API stops moving stated in the reader's terms (what changes for them, not an internal phase token), a link to `CHANGELOG.md`, and the yank policy — \"only one pre-release resolves at a time; each alpha is yanked when the next lands\" — and it names that boundary rather than hedging it, because a section that hedges has satisfied the letter of the mitigation and failed the reason it exists."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/README.md — the `## Stability` section itself (persistent chrome; `CHANGELOG.md` stays opened-on-demand behind its link, _design.md:833)"
  verifying_test: "Cold read by a reviewer who is not the author, against _design.md:789-791 (the three-item cap) and AC-U18 (_decomposition.md:227-232); the CHANGELOG.md link resolving from the rendered page and the absence of a hedge token, both recorded in _release-log.md"

- id: AC-005
  criterion: "GIVEN P1 is upgrading between alphas and needs \"what broke, and why\" rather than a `git log`, WHEN they follow the README's link into `CHANGELOG.md`, THEN `## [Unreleased]` (:24) has become `## [0.2.0-alpha.1] — <ISO date>` with the link reference at :1166 updated to match, no entry has been orphaned or merged by the rename, and the file makes no claim the manifests do not honour: the `unstable-projection` assertion (:19-22) is either corrected or made accurate about which crate carries the feature M5 landed."
  satisfied: false
  evidence: ""
  mount_point: "CHANGELOG.md — the `## [Unreleased]` heading at :24 and its link reference at :1166"
  verifying_test: "`cargo xtask lints` → `changelog_names_every_rule` (CF-29, xtask/src/lints.rs:525, parser at :454-479) + the feature claim read against crates/happenstance-core/Cargo.toml:34-52 and crates/happenstance/Cargo.toml's `[features]` at cut time + anti-pattern 15 review (_design.md:1005-1006)"

- id: AC-006
  criterion: "GIVEN a licence violation or a broken feature combination is something a published crate cannot un-publish its way out of, WHEN the release is cut, THEN one full `cargo xtask ci` — not `--fast`, which drops exactly the two feature powersets, `cargo deny` and the nightly `--cfg docsrs` build — has run green on the exact tree that publishes, and its transcript names that tree's commit SHA."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs — `run_ci` (:828-833), REQUIRED + OPTIONAL, run at the release boundary rather than the project's `--fast` bar (.redkiln/config.yaml:55)"
  verifying_test: "`cargo xtask ci` whole, green, with `git rev-parse HEAD` beside it; inside it the D11 step \"packaged artifacts carry their licences and README\" (xtask/src/main.rs:519-529, xtask/src/package.rs:33-43) and `cargo deny check` running rather than skipping; transcript in _release-log.md"

- id: AC-007
  criterion: "GIVEN a stranger with no clone of this repository, WHEN they create a scratch project outside this workspace and add `happenstance`, `happenstance-core` and `happenstance-testkit` at an explicit pre-release requirement, THEN all three resolve from the registry rather than from a path dependency and the smallest write-then-read cycle compiles and runs against the published `happenstance` — the alpha's observable form of DoD 9 (initiative.md:383-386), which this story makes reachable rather than turns green. AND the publish itself ran `happenstance-core` first (`happenstance` cannot resolve otherwise), AND the yank-and-republish instruction for whoever cuts `0.2.0-alpha.2` — yank the mistake, publish the fix at a new number, never edit in place — is written into this story's folder before the cut, because after it there is nothing left to write it against."
  satisfied: false
  evidence: ""
  mount_point: "crates.io itself — the registry entries for the three publishable crates, reached through `cargo publish -p happenstance-core` then `-p happenstance` (`-p happenstance-testkit` any time after core)"
  verifying_test: "Manual and unautomatable by construction (DEP-006, _decomposition.md:964-977): a scratch project outside this workspace — its Cargo.toml, its `cargo build` output and its program's stdout — plus the publish-order transcript, the docs.rs check within the hour, and the dated rollback instruction, all in .bklg/from-contract-to-published-library/typed-layer-and-alpha-release/publish-0-2-0-alpha-1/_release-log.md"
```
