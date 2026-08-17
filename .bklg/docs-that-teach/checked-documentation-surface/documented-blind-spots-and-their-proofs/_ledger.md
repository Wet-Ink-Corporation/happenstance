---
item: HS-S0145
stage: implement
created: "2026-08-17T13:16:06.300Z"
updated: "2026-08-17T13:16:06.300Z"
---

# Acceptance ledger — What the check does not verify is stated first, and executed where it can be

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Four notes for whoever flips these rows.

**Two mount points, in two Cargo targets, and mixing them up is this project's named most-likely
error.** The primary mount is `xtask/src/narrative.rs`, the lib-target harness declared from
`xtask/src/lib.rs:28` — the only file this story touches whose module docs a gate step actually
renders. The second mount is the bin-crate checker module declared from `xtask/src/main.rs:64-70`;
its sibling ledgers record that HS-S0138 landed it as `xtask/src/lint_narrative.rs`. If either file
landed under another name, correct the paths below to the file that actually exists and cite it —
that is a path correction, not a scope change.

**Three rows are not satisfied by a `#[test]` and must not be flipped as if they were.** AC-005,
AC-006 and AC-010 are discharged by recorded evidence in
`.bklg/docs-that-teach/checked-documentation-surface/documented-blind-spots-and-their-proofs/_limits-evidence.md`
— verbatim transcripts naming their exact command and the toolchain pin. A unit test asserting that
the docs *mention* a probe is not the probe. AC-011 is likewise a prose review over the whole diff,
by construction (`project.md` DoD item 8).

**AC-004 is satisfied by an absence.** Its evidence is the documented inherently-untestable clause
plus the fact that no test, fixture, badge or metric anywhere in the PR gestures at covering limit 1.
Adding one to make the row feel better is the exact failure RS-81-1 names.

**If EC-003 fires — the `text` fixture is already flagged, so limit 5 is closed — AC-006 and AC-007
lose their subject.** Record the disposition in `_limits-evidence.md` and cite it as the evidence
here, annotating both rows explicitly. Do not silently satisfy them against a fixture that no longer
proves anything, and do not weaken the fence walk to make the documented limit true.

```yaml
- id: AC-001
  criterion: "GIVEN a contributor opens `xtask/src/narrative.rs` to find out what the compiling step actually buys them, WHEN they read from the top of the file, THEN the very first `#`-level heading in the module docs is `# What this does not verify` — persistent prose composed as a real rustdoc heading with a bulleted body, never a `//` comment, never a bullet nested under a \"how it works\" preamble, and never behind a `<details>` — AND the `documentation` step stays green under `RUSTDOCFLAGS=-D warnings`, which the section earns by carrying no intra-doc link to a `#[cfg(doctest)]` page module (those modules do not exist under `cargo doc`, so such a link is a hard error)."
  satisfied: false
  evidence: ""
  mount_point: "`xtask/src/narrative.rs` — the lib-target harness declared from `xtask/src/lib.rs:28` beside `mod constitution;`; rendered and lint-denied by the `documentation` step (`xtask/src/main.rs:284-302`) inside `cargo xtask ci`"
  verifying_test: "the section-shape test in the bin-crate checker module (`xtask/src/lint_narrative.rs` → `mod tests`), reading `HARNESS` as text through `workspace_root()` (`cargo test -p xtask`), plus `cargo xtask ci --fast` → `documentation` green"
- id: AC-002
  criterion: "GIVEN the same contributor opens the bin-crate narrative checker module instead — the half that reads pages rather than compiling them — WHEN they read from the top, THEN they meet `# What this does not verify` first there too, and its reference to the harness's four limits is a plain path in prose, not an intra-doc link, because the bin crate has no path to the lib target's modules (`xtask/src/lint_constitution.rs:5` is the standing proof that such a link fails nothing and therefore protects nothing)."
  satisfied: false
  evidence: ""
  mount_point: "the bin-crate narrative checker module (`xtask/src/lint_narrative.rs`), declared from `xtask/src/main.rs:64-70` beside `mod lint_constitution;`"
  verifying_test: "the section-shape test's second arm, reading the checker module's own file as text (`xtask/src/lint_narrative.rs` → `mod tests`, `cargo test -p xtask`)"
- id: AC-003
  criterion: "GIVEN a downstream author who must decide whether a green run licenses them to stop checking a claim by hand, WHEN they read both sections, THEN they find all six of Note 10's limits — each stated where it actually holds (1–4 in the harness, 5 in the checker, 6 unhedged in both) and each written in the precedent's two-part bullet shape: what is not verified, and what the real instrument is — so no limit is discoverable only by reading the other target's file."
  satisfied: false
  evidence: ""
  mount_point: "both mounts: `xtask/src/narrative.rs` (limits 1–4, 6) and `xtask/src/lint_narrative.rs` (limits 5, 6)"
  verifying_test: "the per-limit presence test asserting each module carries its assigned limits and does not restate the others (`xtask/src/lint_narrative.rs` → `mod tests`, `cargo test -p xtask`)"
- id: AC-004
  criterion: "GIVEN a reader who wants to know whether the headline gap — code that still compiles while no longer demonstrating the surrounding claim — is covered by anything, WHEN they read limit 1, THEN it says the gap exists, says no mechanical test can close it and why (it is semantic, not mechanical), and names HS-P0024's friction log as the non-substitutable instrument — AND no test, fixture, metric or count anywhere in this PR gestures at covering it, because a decorative instrument is what makes a reader delete the real one."
  satisfied: false
  evidence: ""
  mount_point: "`xtask/src/narrative.rs` — limit 1 lives in the harness, following `xtask/src/constitution.rs:20-25`'s worked precedent for a limit documented with no test behind it"
  verifying_test: "the per-limit presence test asserts limit 1 carries an explicit inherently-untestable clause (`cargo test -p xtask`); the absence half is verified by diff review — no test is named for limit 1"
- id: AC-005
  criterion: "GIVEN a contributor who has read two mutually contradicting claims about `RUSTDOCFLAGS` in this repository and upstream, WHEN they read limit 3, THEN it states what the probe did, re-run against the narrative compile step as `REQUIRED` declares it on the pinned 1.97.1 toolchain, cites `_limits-evidence.md` for the three verbatim transcripts (default-on lint under the step's `RUSTDOCFLAGS`; the same fence with it removed; a clippy-only lint), and names the upstream reports as context rather than as the finding — AND every probe fence is reverted in the same PR, so the transcripts persist and the fences do not."
  satisfied: false
  evidence: ""
  mount_point: "`xtask/src/narrative.rs` (limit 3's prose) measured against the narrative compile step as wired in `REQUIRED` (`xtask/src/main.rs:105`, shape copied from `:488-492`)"
  verifying_test: "recorded evidence, not a `#[test]`: three verbatim transcripts in `.bklg/docs-that-teach/checked-documentation-surface/documented-blind-spots-and-their-proofs/_limits-evidence.md`, each naming its exact command and `rust-toolchain.toml`'s pin; plus the presence test asserting limit 3 cites the record path, and `cargo xtask ci --fast` green with no probe fence remaining"
- id: AC-006
  criterion: "GIVEN a reader who is told a Rust example tagged `text` is invisible to the check, WHEN they want to know whether that is a real hole or a cautious sentence, THEN a retained fixture page under `TREE` whose ` ```text ` fence is deliberately false about the library has been walked through the whole gate and observed to pass — the compile step never sees it, the fence walk permits the tag, no problem line is emitted — with the run recorded verbatim in `_limits-evidence.md`, reachable by one non-interactive command with no manual step."
  satisfied: false
  evidence: ""
  mount_point: "the retained fixture page under `TREE` (`docs/`), registered by `include_str!` in `xtask/src/narrative.rs`, walked by both narrative steps inside `cargo xtask ci`"
  verifying_test: "end-to-end: `cargo xtask ci --fast` over the tree containing the fixture, output recorded verbatim in `_limits-evidence.md`; narrower: `cargo test --locked -p xtask --doc` and the narrative checker subcommand, both green"
- id: AC-007
  criterion: "GIVEN a future contributor who teaches the fence walk to inspect `text` fences and closes limit 5, WHEN they run the gate, THEN a `#[cfg(test)]` test that runs the real checker over the fixture and asserts no problem is reported for the `text` fence fails, with a message saying the limits section is now wrong and limit 5 must be deleted in the same change — so the section rots loudly rather than silently."
  satisfied: false
  evidence: ""
  mount_point: "the `#[cfg(test)]` block of the bin-crate checker module (`xtask/src/lint_narrative.rs`), run by the `tests` step (`xtask/src/main.rs:143`)"
  verifying_test: "the positive pinning test in `xtask/src/lint_narrative.rs` → `mod tests` running the real checker over the fixture and asserting no problem for the `text` fence (`cargo test -p xtask`); its failure message reviewed against `standards/rust/81-checks-that-cannot-be-types.md:11`"
- id: AC-008
  criterion: "GIVEN a contributor who has just watched the gate fail and is trying to map the filename in the output onto the page they broke, WHEN they read limit 4, THEN its wording is quoted from the run `observed-failure-falsification` recorded — which file the failure names, that the page is resolved by module name and the line inside the page — and where that record disagrees with the architecture brief's forecast (Note 3), the limit says what the record says."
  satisfied: false
  evidence: ""
  mount_point: "`xtask/src/narrative.rs` — limit 4 in the harness, since the degraded pointer is a property of the `include_str!` compile mechanism"
  verifying_test: "traceability review: limit 4's wording matched against the verbatim output recorded by `.bklg/docs-that-teach/checked-documentation-surface/observed-failure-falsification/`, with the forecast-vs-record reconciliation written into `_limits-evidence.md`"
- id: AC-009
  criterion: "GIVEN a later change that quietly weakens the honesty section — moves it below another heading, drops a limit, or hedges limit 6 with a \"but the gate does check…\" clause — WHEN the gate runs, THEN it fails, because the section tests were written against those three named wrong implementations rather than against the mere existence of a heading."
  satisfied: false
  evidence: ""
  mount_point: "the `#[cfg(test)]` block of `xtask/src/lint_narrative.rs`, run by the `tests` step (`xtask/src/main.rs:143`)"
  verifying_test: "three negative tests (or three assertions with distinguishable messages) in `xtask/src/lint_narrative.rs` → `mod tests`, one per named wrong implementation (`cargo test -p xtask`), in the house test shape of `xtask/src/lint_constitution.rs:828-878`"
- id: AC-010
  criterion: "GIVEN the project is being called done, WHEN a reviewer asks whether every delivered check has its limit on the record, THEN `_limits-evidence.md` carries the reconciliation — pinned tree, empty-tree guard, bidirectional registration, fence discipline, allowance sweep, hidden markers, citation resolution, frozen-MUST pin, count agreement — each with its limit in the section or a written reason it has none, plus a written disposition for the `compile_fail` candidate seventh (added, or not-applicable because the delivered fence walk does not permit `compile_fail`)."
  satisfied: false
  evidence: ""
  mount_point: "`.bklg/docs-that-teach/checked-documentation-surface/documented-blind-spots-and-their-proofs/_limits-evidence.md`, reconciled against the limits sections at both mounts"
  verifying_test: "review artefact: the reconciliation table in `_limits-evidence.md`, one row per delivered check, checked against `_decomposition.md` Note 10 and `_design.md` `## The states the API must express`"
- id: AC-011
  criterion: "GIVEN any reader of anything this project produced — module docs, the fixture page, `docs/README.md`, a step name, a commit message — WHEN they look for a claim that the surface proves a page teaches, THEN there is none: limit 6 is one unhedged sentence in both modules, and no badge, tick, shield or \"verified\" mark appears anywhere, nor any `book.toml`, `book/`, `site/` or `.css` that would create a second rendered surface able to carry one."
  satisfied: false
  evidence: ""
  mount_point: "the whole PR surface: both module docs, the fixture page and `docs/README.md` under `TREE`"
  verifying_test: "prose scan of the PR diff against `project.md` DoD item 8 and `_design.md` `## Anti-patterns` 9 and 10 (review, by construction not a `#[test]`), plus the presence test asserting limit 6 present and unhedged in both modules (`cargo test -p xtask`)"
- id: AC-012
  criterion: "GIVEN a reader who lands on `docs/README.md` and follows a row to the new fixture page, WHEN the page renders at the narrow width, THEN it is a conforming `narrative-page` in the signed-off design's terms: registered in the harness in both directions, no `HIDDEN_MARKERS` token anywhere, its repo-relative path ≤ 32 characters with no third directory level, H1 ≤ 40 characters, fences ≤ 80 columns, page ≤ 250 source lines, an index row in the two-column table above the pointer-out table — and it adds one row without reordering the rows already there."
  satisfied: false
  evidence: ""
  mount_point: "the fixture page under `TREE` (`docs/`), its `include_str!` registration in `xtask/src/narrative.rs`, and its row in `docs/README.md`'s two-column narrative table"
  verifying_test: "the checker's own registration, hidden-marker and path-budget checks run over the fixture via `cargo xtask ci --fast` (mechanical half); review against `_design.md` `## Density budget`, `## Composition` and `## Anti-patterns` 1 and 11 (the H1, fence-width and page-length caps, which are review rules by `_design.md` `## Open questions` 2)"
```
