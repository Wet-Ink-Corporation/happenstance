---
item: "HS-S0097"
stage: implement
created: "2026-08-12T13:47:35.384Z"
updated: "2026-08-12T13:47:35.384Z"
---

# Acceptance ledger — A stranger installs it from the registry and it works

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

**Two notes specific to this story.** Every `verifying_test` below is a **recorded transcript**, not
a compiled test: this is the project's one `e2e` seam and the testing brief places it outside
`cargo xtask ci` by name (*Fixtures and seams to mock*), because the gate must stay offline and
hermetic. And the story runs **after** the irreversible act — so a row whose arm failed is flipped by
recording the failure and its route, never by re-running until it passes or by editing a published
surface (spec EC-011).

```yaml
- id: AC-001
  criterion: "**GIVEN** P4 has decided not to clone anything and will type `cargo add happenstance` into a project of their own, and a `path` dependency never fetches a tarball at all — so a workspace-local smoke passes even if `0.2.0` was never published — **WHEN** the maintainer runs the smoke after the act, **THEN** the subject is proven to be the **registry artefact** four independent ways, each *recorded* rather than assumed: the scratch project is created by `cargo new` in a directory with **no ancestor `Cargo.toml` carrying a `[workspace]` table**; its dependency line carries **no `path` and no `git` key**; the generated `Cargo.lock` records `source = \"registry+https://github.com/rust-lang/crates.io-index\"` for **every** `happenstance*` package at the published number; and the whole resolved graph is recorded — any one of the four missing makes the run **void**, not passing"
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/publication-and-positioning/stranger-install-smoke/_smoke-log.md"
  verifying_test: ".bklg/from-contract-to-published-library/publication-and-positioning/stranger-install-smoke/_smoke-log.md § Provenance — the scratch `Cargo.toml` verbatim, every `happenstance*` `[[package]]` block from the generated `Cargo.lock`, and the `cargo metadata --format-version 1 --manifest-path <scratch>/Cargo.toml` transcript showing the registry source"

- id: AC-002
  criterion: "**GIVEN** a machine whose ambient cargo configuration replaces crates.io with a local mirror produces a passing run that is *a path dependency wearing a registry URL*, and P4's machine has no such configuration, **WHEN** the maintainer prepares to run, **THEN** before the first `cargo` invocation, `$CARGO_HOME/config.toml` **and every** `.cargo/config.toml` on the path from the scratch directory to the filesystem root are read for `[source]` replacement, `[patch]`, `[replace]` and any vendored directory, and the finding is written into the record as an explicit **none** — stated, never implied by an absent section — with each path listed whether or not the file existed"
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/publication-and-positioning/stranger-install-smoke/_smoke-log.md"
  verifying_test: ".bklg/from-contract-to-published-library/publication-and-positioning/stranger-install-smoke/_smoke-log.md § Environment attestation — one row per config path inspected (`$CARGO_HOME/config.toml` plus every `.cargo/config.toml` from the scratch directory to the filesystem root), each carrying its relevant contents or `absent`, and a note that this repository's own `.cargo/config.toml` is unreachable from the scratch directory"

- id: AC-003
  criterion: "**GIVEN** AC-UX-012 promises P4 that *a reader who copies what they see gets what the smoke proved*, and **AP-12** fails a fence that differs by one character from the text this smoke ran, **WHEN** the program is assembled, **THEN** it is **extracted from the downloaded `.crate` tarball's `README.md`** — the artefact a consumer receives, never the working tree and never `_design.md` — and the record carries the extracted fence verbatim, a digest of it, a digest of the program body that actually ran, and the two are **equal**; if they cannot be made equal without editing the fence, the difference is recorded as a **finding** and nothing is edited"
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/publication-and-positioning/stranger-install-smoke/_smoke-log.md"
  verifying_test: ".bklg/from-contract-to-published-library/publication-and-positioning/stranger-install-smoke/_smoke-log.md § The region as shipped — two recorded `sha256` digests plus byte counts (the fence extracted from the `.crate` tarball's `README.md`, and the program body that ran), with the tarball's own provenance path or `cargo download` transcript"

- id: AC-004
  criterion: "**GIVEN** P4 tries a library by copying **what the page actually shows**, and the signed-off design sites the quick start at **R7, directly under the compliance block R6**, as a `toml` fence *plus* a `rust` fence, *revealed on scroll* on the packaged surface, budgeted at **≤ 20 source lines** (the design's own text is 19), **WHEN** the smoke reads the packaged README out of the tarball, **THEN** it records that region **as shipped** and checks the composition it depends on: a `toml` fence exists and the scratch manifest's dependency line **is the line that fence tells a reader to write**; the `rust` fence **declares its language** and is **≤ 20 source lines**; the region is on the packaged README itself — 0 hops, not one hop behind a link; the compliance block precedes it; the region contains **no raw HTML** (**AP-7**); **AND** everything the fence does not show but the program needed — a `main`, an executor, and the extra registry dependency the page never names — is listed **item by item** as scaffolding, with the gap against AC-UX-012 recorded as a finding routed to `guarantees-and-docs-rs-presentation` and a forward version, never repaired here"
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/publication-and-positioning/stranger-install-smoke/_smoke-log.md"
  verifying_test: ".bklg/from-contract-to-published-library/publication-and-positioning/stranger-install-smoke/_smoke-log.md § The region as shipped — the packaged quick-start region reproduced verbatim with its neighbours, the `rust` fence's source-line count, both fence info strings, and the scaffolding list; reviewed against .bklg/from-contract-to-published-library/publication-and-positioning/_design.md `## Composition` (R6→R7), `## Transience policy`, `## Density budget` and AP-12 / AP-7"

- id: AC-005
  criterion: "**GIVEN** P4's sitting ends the moment the example does not work, and **every** existing instrument in this repository only ever *compiles* the fence — the tree's own declares `async fn` and never awaits it (`crates/happenstance/README.md`:30-39; the module doctest hides the same at `crates/happenstance/src/lib.rs`:58-67) — **WHEN** the scratch project is built and executed, **THEN** the write-then-read cycle **runs to completion and the process exits `0`**: at least one event appended, the read returning it, the published fence's own assertion holding, with stdout, stderr and the exit status recorded; **AND** it does so at **exactly the feature set a `cargo add` reader gets** — no `features` key, no `default-features = false` — with the resolved set recorded so a later reader can see that no unification occurred"
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/publication-and-positioning/stranger-install-smoke/_smoke-log.md"
  verifying_test: ".bklg/from-contract-to-published-library/publication-and-positioning/stranger-install-smoke/_smoke-log.md § The run — the `cargo run` transcript with its exit status, plus `cargo tree -e features` for `happenstance` and `happenstance-core` showing `std` + `memory` and nothing beyond (`crates/happenstance/Cargo.toml`:14-26)"

- id: AC-006
  criterion: "**GIVEN** P3 reads one line in the Guarantees slot to self-identify as a `wasm32` consumer and cannot check it without cloning (IQ-5), and `publish-0-2-0` proved that claim only over the **workspace** (its AC-005's fifth mandatory step), **WHEN** the same *installed* crate is targeted, **THEN** `rustup target add wasm32-unknown-unknown` followed by `cargo check --target wasm32-unknown-unknown` runs in the same scratch project over a target that exercises `happenstance` at its default features, recorded verbatim; it is a **check, never a run** and nothing pretends to execute there; **AND** if the chosen executor does not type-check on that target the arm swaps to a `--lib` target that omits it and **the substitution and its reason are recorded**, because the claim under test is about `happenstance`, not about the executor"
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/publication-and-positioning/stranger-install-smoke/_smoke-log.md"
  verifying_test: ".bklg/from-contract-to-published-library/publication-and-positioning/stranger-install-smoke/_smoke-log.md § The wasm32 arm — the `rustup target add wasm32-unknown-unknown` and `cargo check --target wasm32-unknown-unknown` transcripts, the manifest excerpt for the checked target shape, and any substitution stated with its reason"

- id: AC-007
  criterion: "**GIVEN** the Guarantees slot promises P1 and P2 a **1.97.1** floor (`crates/happenstance/README.md`:46-49), and ADR-0029 records that the floor moved because of *a dependency's build script* while five of five database crates here declare no `rust-version` — so neither `cargo hack --rust-version` nor `resolver = \"3\"` can protect it and only running the compiler finds it — **WHEN** the smoke runs, **THEN** it runs on **1.97.1**, the channel `rust-toolchain.toml`:2 already pins, with `rustc --version` and `cargo --version` recorded and the **full resolved dependency graph** recorded beside them, because a stranger's `cargo add` produces a graph this repository's `Cargo.lock` never contained; **AND** a failure at that floor is a **finding routed to `msrv-promise-atom`'s atom and a forward version** — never a silent bump, never an edit to the published promise"
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/publication-and-positioning/stranger-install-smoke/_smoke-log.md"
  verifying_test: ".bklg/from-contract-to-published-library/publication-and-positioning/stranger-install-smoke/_smoke-log.md § Toolchain and graph — `rustc --version` and `cargo --version` transcripts showing 1.97.1 (the channel pinned at `rust-toolchain.toml`:2 and promised at `crates/happenstance/README.md`:46-49), and `cargo tree` over the whole resolved graph"

- id: AC-008
  criterion: "**GIVEN** project DoD 2 asks for a run *by someone who did not build it* and DoD 3 for a dated committed artefact, and P4 is exactly the person who cannot take an assertion on trust, **WHEN** the run is written up, **THEN** `_smoke-log.md` is a **recipe a third party can execute verbatim from the record alone** — on a machine with no checkout of this repository, no credentials and nothing beyond `cargo` and `rustup`: every command in order, the manifest and lock excerpts, both digests, the graph, the toolchain versions, the OS/arch, the date and the elapsed time since `publish-0-2-0`'s publish transcript, and a plain statement of **who ran it and in what environment**, with any gap against DoD 2 stated rather than papered over; **every arm that could not run is named with why and what it would have proven**; a **findings section is present whether or not it is empty**, with *none* written out; a failing arm is recorded **as it failed** and never made green by editing a published surface, by re-running until it passes, or by adding a `path`, `[patch]`, `[replace]` or vendored source; nothing outside this story's folder changes and no gate step is added; the scratch project is **destroyed**, surviving only as transcript; and the file is **written once** — a later version's smoke is a later story's record"
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/publication-and-positioning/stranger-install-smoke/_smoke-log.md"
  verifying_test: "`redkiln verify --grain story` against the one-line PR-boundary block, `git diff --name-only` showing exactly one added file under .bklg/from-contract-to-published-library/publication-and-positioning/stranger-install-smoke/, `cargo xtask affected --base main` green and `cargo xtask ci --fast` unchanged in step count; plus a read of _smoke-log.md for all eight named sections and a findings section present even when empty"
```
