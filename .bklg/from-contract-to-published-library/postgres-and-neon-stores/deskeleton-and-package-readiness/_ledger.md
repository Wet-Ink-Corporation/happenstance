---
item: "HS-S0072"
stage: implement
created: "2026-08-12"
updated: "2026-08-12"
---

# Acceptance ledger — Neither crate is a skeleton any more

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

```yaml
- id: AC-001
  criterion: "GIVEN an adapter author who has been told by CLAUDE.md's repository map that happenstance-postgres and happenstance-neon are skeletons — real associated types and todo!() bodies, and a scoped #![allow(clippy::todo)] naming the phase that removes it — WHEN they open either crate after this story, THEN neither crate contains a single todo!() invocation (34 across the two src/ trees at the time of writing) and neither lib.rs carries #![allow(clippy::todo)] or the three-line comment that explained it, SO THAT what holds the claim is the workspace-wide denial of clippy::todo rather than a note promising a future phase. A todo!() swapped for unimplemented!(), a bare panic!, or an Err(…) no rule exercises fails this criterion even though it passes the lint."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/package.rs — PUBLISHABLE:86, reached from xtask/src/main.rs REQUIRED:105 via the package-check step :515-535; the deletion sites are crates/happenstance-postgres/src/lib.rs:59-63 and crates/happenstance-neon/src/lib.rs:102-105"
  verifying_test: "the gate's clippy step — xtask/src/main.rs:115-130 (`cargo clippy --workspace --all-targets --all-features -- -D warnings`), plus `rg -n 'todo!\\(' crates/happenstance-postgres/src crates/happenstance-neon/src` returning nothing"

- id: AC-002
  criterion: "GIVEN an evaluator taking their one bounded look — the registry blurb, then the front page of the docs — WHEN they read either crate's description and its crate-level rustdoc, THEN nothing there still says the crate is unimplemented: both descriptions have lost the trailing 'Not yet implemented.' and read as xtask/src/reserve.rs:97-110 already declares them, both '# Status: not implemented' sections are replaced by prose that is true of the code beneath them, and crates/happenstance-neon/src/event_store.rs's #[expect(dead_code)] on AppendOutcome — whose reason names 'phase 10' as its expiry — is gone, SO THAT the three expiring claims in this pair of crates expire together rather than one outliving the code that justified it. The instrument framing both crates earn is kept: the capability table, the CTE discussion and the ProbeThenWriteStore trap are not deleted while tidying."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-postgres/src/lib.rs:1-8 and crates/happenstance-neon/src/lib.rs:4-8 (crate-level rustdoc); crates/happenstance-neon/src/event_store.rs:232-247; both manifests' `description` key, re-read by xtask/src/package.rs on every package-check"
  verifying_test: "the gate's clippy step — xtask/src/main.rs:115-130 (unfulfilled_lint_expectation fires on the retained #[expect]) — and the documentation step xtask/src/main.rs:284-302"

- id: AC-003
  criterion: "GIVEN an application author who has decided the contract first and is now choosing a database and who reaches for `cargo add happenstance-postgres` — WHEN the manifests are read by Cargo after this story, THEN `publish = false` is absent from both (crates/happenstance-postgres/Cargo.toml:12, crates/happenstance-neon/Cargo.toml:12), each declares `readme = \"README.md\"` explicitly for the reason crates/happenstance-core/Cargo.toml:12-15 gives, and neither takes a `version` key of its own — `version.workspace = true` already resolves to the 0.2.0 at Cargo.toml:6 — SO THAT the two crates join the release the workspace already carries instead of forking a version line nobody decided."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/package.rs — publishable_members()/scan_publishable read the fact out of `cargo metadata`, reached from xtask/src/main.rs REQUIRED:105 via package-check :515-535; the edit sites are crates/happenstance-postgres/Cargo.toml:12 and crates/happenstance-neon/Cargo.toml:12"
  verifying_test: "`cargo xtask package-check` — xtask/src/main.rs:515-535 → xtask/src/package.rs:103-161 (both crates must appear in the derived publishable set)"

- id: AC-004
  criterion: "GIVEN a maintainer who wants the tree to notice when a crate is promoted by accident — the second of the two bugs xtask/src/package.rs:36-42 says no other step in the gate would ever catch — WHEN PUBLISHABLE at xtask/src/package.rs:86 grows from three names to five in the same change that flips the manifests, THEN reconcile (:172-218) reports the declared set agreeing with the derived one, SO THAT the intention and the fact are reconciled rather than one silently overtaking the other. Half the change alone must fail, and fail differently in each direction."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/package.rs:86 — the PUBLISHABLE const, consumed by reconcile :172-218 and run :103-161, reached from xtask/src/main.rs REQUIRED:105 via the package-check step :515-535"
  verifying_test: "`cargo xtask package-check` — xtask/src/package.rs:172-218; both failure directions provoked deliberately and their messages transcribed (:188-196 promoted, :198-204 withdrawn). The existing unit tests at xtask/src/package.rs:408-457 stay green under `cargo test -p xtask`."

- id: AC-005
  criterion: "GIVEN the local-first / edge developer whose fear is being orphaned on the less-supported path, and who unpacks a .crate tarball before depending on it — WHEN `cargo package -p happenstance-postgres --list` and `cargo package -p happenstance-neon --list` are run, THEN each listing contains LICENSE-MIT, LICENSE-APACHE and README.md (REQUIRED_FILES, xtask/src/package.rs:94) as copies inside the crate directory — not a symlink, not a `readme = \"../../README.md\"`, not a workspace include — with licence bodies byte-identical to the workspace-root LICENSE-MIT and LICENSE-APACHE, and `cargo xtask package-check` reports all five crates clean, SO THAT the metadata promise of `MIT OR Apache-2.0` is backed by both texts a consumer needs to make the choice."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/package.rs:94 (REQUIRED_FILES) asserted per crate by run() :131-149, reached from xtask/src/main.rs REQUIRED:105 via package-check :515-535; the six new files land in crates/happenstance-postgres/ and crates/happenstance-neon/"
  verifying_test: "`cargo xtask package-check` — xtask/src/package.rs:131-158 (parses the listing and asserts the three files by name, per crate); byte-identity by `git diff --no-index` against the workspace-root LICENSE-MIT and LICENSE-APACHE"

- id: AC-006
  criterion: "GIVEN the evaluator with one look, arriving on the crates.io page rather than in this repository — WHEN they read crates/happenstance-postgres/README.md and crates/happenstance-neon/README.md, THEN each is a real front page in the register of crates/happenstance-testkit/README.md and crates/happenstance/README.md: what the adapter is, which port flavour it implements, what it cannot do stated as a limit rather than omitted (Neon: no connection, no interactive transaction, no cursor, the 64 MiB-anchored ceilings; Postgres: head is a frontier, no read-your-own-writes, the staleness bound postgres-structural-bill wrote), and how to run its live conformance suite — and the reservation placeholder text 'This `0.0.0` is a placeholder. It contains no functionality.' (xtask/src/reserve.rs:243-269) appears in neither, SO THAT this story retracts the claim claim-crate-names deliberately published rather than re-landing it."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-postgres/README.md and crates/happenstance-neon/README.md — packaged because xtask/src/package.rs:94 requires them and the manifests declare `readme = \"README.md\"`; presence asserted through the package-check step at xtask/src/main.rs:515-535"
  verifying_test: "`cargo xtask package-check` (presence in the artifact) plus `rg -n 'is a placeholder' crates/happenstance-postgres crates/happenstance-neon` returning nothing; content by human read against crates/happenstance-testkit/README.md and crates/happenstance/README.md, which is what initiative.md:387-389 (DoD 10) asks for"

- id: AC-007
  criterion: "GIVEN the edge developer who reaches Postgres from a Cloudflare Worker and will otherwise spend a day discovering Hyperdrive's limits themselves — WHEN they read happenstance-postgres's crate-level rustdoc beside its existing '# Not the Neon adapter' section (crates/happenstance-postgres/src/lib.rs:49-56), THEN they meet a note stating Hyperdrive plus a worker::Socket-backed driver and stating plainly that it is not a supported configuration — it needs a forked driver with unnamed-statement support and a hand-rolled binding (RUNBOOK.md:4364-4366) — with a one-line cross-reference from crates/happenstance-neon/src/lib.rs, SO THAT the configuration is documented as a dead end rather than discovered as one. Documentation only: no code, no CI job, no test. A different placement is permitted; a placement chosen silently is not, and neither is prose that reads as an endorsement."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-postgres/src/lib.rs:49-56 — the crate-level rustdoc beside '# Not the Neon adapter', with a cross-reference from crates/happenstance-neon/src/lib.rs; both compiled by the documentation step at xtask/src/main.rs:284-302"
  verifying_test: "the gate's documentation step — xtask/src/main.rs:284-302 (builds, and the cross-reference resolves) — plus `rg -n -i 'hyperdrive' crates/happenstance-postgres/src crates/happenstance-neon/src` locating it, and review that it reads as unsupported"

- id: AC-008
  criterion: "GIVEN a maintainer about to make two crate names publicly claimable by removing `publish = false` — WHEN the manifests are flipped, THEN it has first been verified (not assumed) that claim-crate-names landed and both happenstance-postgres and happenstance-neon are held on crates.io by this project's owner, SO THAT the window in which the workspace intends to publish a name it does not hold never opens. The reservation is not re-run here; phase 0's rule is that a name is reserved when its phase starts (RUNBOOK.md:4346-4348), and xtask/src/reserve.rs:99,105 already knows both names."
  satisfied: false
  evidence: ""
  mount_point: "precondition of the mount: the PUBLISHABLE growth at xtask/src/package.rs:86 and the `publish = false` deletion at crates/happenstance-postgres/Cargo.toml:12 and crates/happenstance-neon/Cargo.toml:12 must not land until this is verified"
  verifying_test: "the upstream story's own artifacts under .bklg/from-contract-to-published-library/postgres-and-neon-stores/claim-crate-names/ (its _ledger.md and implementation report), plus a recorded check of each name's registry page; xtask/src/reserve.rs:99,105 names both crates"

- id: AC-009
  criterion: "GIVEN every contributor who runs the gate on a laptop with no Docker, no credentials and no network — the bar project.md AC-011 and DR-9 set for this whole project — WHEN this story lands, THEN `cargo xtask ci --fast` is green on a clean checkout with none of those present; REQUIRED/OPTIONAL in xtask/src/main.rs gain no step and .github/workflows/ci.yml gains no job; no existing step's `name` string changes, because wasm_steps()/steps_named select by name and panic on a miss (xtask/src/main.rs:771-816); and `cargo deny` and `cargo xtask spec-trace` are both still green, SO THAT growing the publishable surface tightens what the gate asserts without changing what the gate costs to run. If a transport dependency now fails deny.toml's allowlist — the known live risk being sqlx's tls-rustls on webpki-roots (CDLA-Permissive-2.0), not on ring — that is reported as a blocking finding, never fixed by widening deny.toml:9-19."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs — the REQUIRED array at :105 and the step list, which this story reads and must not edit; the package-check step at :515-535 is the only one whose assertions change, and it changes because xtask/src/package.rs:86 grew"
  verifying_test: "`cargo xtask ci --fast` (.redkiln/config.yaml verify.integration_scoped) on a clean checkout with no Docker, network or credentials; `cargo xtask spec-trace` (verify.reachability_static); the `cargo deny` step at xtask/src/main.rs:595-601; and `git diff --stat` over xtask/src/main.rs and .github/workflows/ci.yml showing zero changed lines"
```
