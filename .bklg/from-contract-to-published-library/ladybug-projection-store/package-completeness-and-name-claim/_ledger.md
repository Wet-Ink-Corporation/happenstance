---
item: HS-S0080
stage: implement
created: "2026-08-12"
updated: "2026-08-12"
---

# Acceptance ledger — Package-complete the crate and hold the name

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Three of these rows (AC-006, AC-007, AC-008) have no automatable oracle by the project's own testing
brief — the crates.io name claim sits outside every test tier
(`.bklg/from-contract-to-published-library/ladybug-projection-store/_decomposition.md`, *Testing
brief*, AC-010 row). Their evidence is a dated transcript or a `git diff`, cited by path, not a test
id. That is the standard this story is held to; an empty `evidence` field is not satisfied by
"manual".

```yaml
- id: AC-001
  criterion: "GIVEN the evaluator has downloaded the crate and unpacked its `.crate` tarball to check that `MIT OR Apache-2.0` is a real offer rather than a metadata string, WHEN they list the archive, THEN `LICENSE-MIT` and `LICENSE-APACHE` are both inside it, byte-identical to the repository-root texts — a copy, never a link or a path out of the package directory, because Cargo will not follow one and reports no error when it cannot (xtask/src/package.rs:1-22, :151-157)."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/package.rs — the PUBLISHABLE constant at xtask/src/package.rs:86, asserted per crate by xtask/src/package.rs:103-161"
  verifying_test: "cargo package -p happenstance-ladybug --list --allow-dirty --locked (isolated), and the same assertion in the gate: cargo run --locked --quiet -p xtask -- package-check → xtask/src/package.rs:103-161, invoked at xtask/src/main.rs:517-532"

- id: AC-002
  criterion: "GIVEN the evaluator lands on the crates.io page for `happenstance-ladybug` with one look in which to decide, WHEN the page renders its front matter, THEN a `README.md` written for this crate is there — naming that it is a projection store only and why an event log cannot be forced onto a graph engine, naming LadybugDB/Cypher, and carrying a plain status block that states what is not yet true (including that `lbug` compiles C++ through `cxx`/`cmake`, and that docs.rs cannot build it) — at the standard crates/happenstance-testkit/README.md:1-22 already sets, not the generated placeholder blurb."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-ladybug/README.md, reached through xtask/src/package.rs:94 REQUIRED_FILES and rendered by crates.io from crates/happenstance-ladybug/Cargo.toml's `readme` key"
  verifying_test: "cargo run --locked --quiet -p xtask -- package-check (README.md present in the artifact) plus a review-tier read against crates/happenstance-ladybug/src/lib.rs:1-32 and crates/happenstance-testkit/README.md:9-22, with the reviewer named"

- id: AC-003
  criterion: "GIVEN the evaluator reads the one-line description and the rendered front page — the two fields crates.io shows before anything else — WHEN they compare them to what the crate now does, THEN neither lies: `description` no longer ends \"Not yet implemented.\" and matches xtask/src/reserve.rs:110-115 exactly, so the placeholder published today and the eventual release describe the same thing to anyone browsing (xtask/src/reserve.rs:41-43), and `readme = \"README.md\"` is stated in the manifest rather than left to Cargo's silent default (crates/happenstance-core/Cargo.toml:12-15)."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-ladybug/Cargo.toml:3 (description) and its `readme` key; the reservation row it must match at xtask/src/reserve.rs:110-115"
  verifying_test: "String-equality check of crates/happenstance-ladybug/Cargo.toml's description against xtask/src/reserve.rs:110-115, recorded in this ledger; `readme = \"README.md\"` present in crates/happenstance-ladybug/Cargo.toml; cargo package -p happenstance-ladybug --list confirms the file the key points at is in the artifact"

- id: AC-004
  criterion: "GIVEN the adapter author intends this crate to become shippable, WHEN they remove `publish = false` from crates/happenstance-ladybug/Cargo.toml:12, THEN the workspace's stated intention moves with it in the same commit — \"happenstance-ladybug\" joins PUBLISHABLE at xtask/src/package.rs:86 — and neither half alone is a mergeable state: the promoted direction fails with package.rs:188-195's message and the withdrawn direction with :198-204's, because they are two different bugs with two different remedies (package.rs:24-42)."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/package.rs:86 — the PUBLISHABLE constant, reconciled against cargo metadata by xtask/src/package.rs:172-218"
  verifying_test: "cargo run -p xtask -- package-check prints the agreement line (xtask/src/package.rs:216-219); plus two new unit cases in xtask/src/package.rs's `mod tests` (xtask/src/package.rs:409-456) asserting reconcile's promoted and withdrawn wordings, run by cargo test --workspace --all-features"

- id: AC-005
  criterion: "GIVEN the adapter author runs `cargo xtask ci` before pushing — on an uncommitted tree, the only tree anyone runs it against (xtask/src/package.rs:69-74) — WHEN the packaging step runs, THEN it names `happenstance-ladybug` where it previously did not and asserts all three REQUIRED_FILES, AND it stays a file listing: the observed wall-clock delta this step adds to the gate is measured and recorded, because `cold-build-cost-and-ci-shape` is measuring `lbug`'s cold C++ build against this same gate and a packaging step that quietly compiled would corrupt that number."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs:517-532 — the `packaged artifacts carry their licences and README` gate step that invokes package-check"
  verifying_test: "cargo xtask ci green on this tree with the step's stdout line `happenstance-ladybug: N files packaged, including LICENSE-MIT, LICENSE-APACHE, README.md` captured (xtask/src/package.rs:138-144); plus two timed runs of cargo run -p xtask -- package-check (warm target dir, before/after the PUBLISHABLE edit) with the seconds recorded here"

- id: AC-006
  criterion: "GIVEN the project's vocabulary — RUNBOOK.md, xtask/src/reserve.rs:110-115 and CLAUDE.md's repository map — already says `happenstance-ladybug`, and GIVEN that name is not among the four verified free on 2026-08-06 (RUNBOOK.md:798-801), WHEN the implementer reaches the irreversible step, THEN availability has already been confirmed against the registry directly — `cargo publish --dry-run` packages and compiles but neither uploads nor reserves — and a name found taken halts the story and is surfaced, never silently renamed."
  satisfied: false
  evidence: ""
  mount_point: "RUNBOOK.md:4409-4413 — phase 11's `Claim happenstance-ladybug on crates.io` work item, against the per-phase reservation policy at RUNBOOK.md:795-829"
  verifying_test: "No test tier by the project's own testing brief (.bklg/from-contract-to-published-library/ladybug-projection-store/_decomposition.md, Testing brief, AC-010 row). Evidence is a dated registry availability check recorded in this ledger — date, method, observed result — captured before any publish transcript exists"

- id: AC-007
  criterion: "GIVEN the evaluator will meet whatever is on the registry permanently — \"a version can be yanked, never removed\" (xtask/src/reserve.rs:181) — WHEN the name is claimed, THEN it is claimed by the generated `0.0.0` placeholder that `cargo xtask reserve happenstance-ladybug` writes (both licences, a README that says plainly it has no functionality, src/lib.rs exposing only STATUS), not by publishing the real crate at any version, AND the printed `cargo publish --manifest-path … --dry-run` has run green first, AND the generated manifest's literal `edition = \"2021\"` / `rust-version = \"1.85\"` (xtask/src/reserve.rs:216-239) has a recorded disposition against the workspace's `edition = \"2024\"` / MSRV 1.97.1 — accepted as true of a standalone, dependency-free `0.0.0` crate, or raised as a finding routed to the `support` initiative. Not fixed inline: xtask/src/reserve.rs is outside this story's boundary."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/reserve.rs:110-115 — the Reservable row for happenstance-ladybug (read, not edited); the generated placeholder at target/reserve/happenstance-ladybug/"
  verifying_test: "No test tier. Evidence is three captured transcripts in order — cargo xtask reserve happenstance-ladybug; cargo publish --manifest-path target/reserve/happenstance-ladybug/Cargo.toml --dry-run (green); cargo publish --manifest-path target/reserve/happenstance-ladybug/Cargo.toml — plus a written EC-007 disposition citing xtask/src/reserve.rs:214-239 and .kb/decisions/0029-msrv-raised-to-1-97-1.md"

- id: AC-008
  criterion: "GIVEN crates.io's policy objects to a name held \"without having any genuine functionality, purpose, or significant development activity\" (RUNBOOK.md:802-813), and GIVEN this story deliberately sits after `fill-the-bodies-and-ps-34-disposition` so the justification is a crate with no `todo!()` left in it, WHEN the claim is made, THEN it is traceable to the work that justifies it: date, registry version and the exact command are recorded in this story's ledger, and RUNBOOK.md phase 11's \"Claim happenstance-ladybug on crates.io\" work box and its \"publish = false removed\" exit box are ticked in place, with no other phase-11 box touched."
  satisfied: false
  evidence: ""
  mount_point: "RUNBOOK.md:4409-4413 (the claim work box) and RUNBOOK.md:4432-4438 (the phase 11 exit list holding `publish = false` removed)"
  verifying_test: "git diff -- RUNBOOK.md shows exactly two `- [ ]` → `- [x]` transitions at those two locations and no other changed line; git log shows this story's commits after fill-the-bodies-and-ps-34-disposition's; the claim date, published version (0.0.0) and command string recorded in this row's evidence"
```
