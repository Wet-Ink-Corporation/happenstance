---
item: HS-S0081
stage: implement
created: 2026-08-12T13:47:19.286Z
updated: 2026-08-12T13:47:19.286Z
---

# Acceptance ledger — Measure the C++ build cost and implement the CI shape it buys

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Note for this story: five of the six criteria are proven by commit order, file presence and review
rather than by a test-framework assertion — project **AC-009**'s tier is *process (measurement), not
a suite tier* (`.bklg/from-contract-to-published-library/ladybug-projection-store/_decomposition.md:479`).
`verifying_test` therefore names the real command or artefact path that discharges the row, which is
what the evidence must cite.

```yaml
- id: AC-001
  criterion: "GIVEN an evaluator who has been handed a CI-shape decision and no reason to trust it, WHEN they run `git log` over this story's paths, THEN the commit introducing `experiments/ladybug-build-cost/README.md` — which states, in that commit, what makes the cost *material* (a wall-clock figure or ratio, the configuration it is read against, and which of the two levers each side of it selects) and contains **no measured figure** — strictly precedes every commit carrying a result, and both strictly precede the commit that edits `xtask/src/main.rs` or `.github/workflows/ci.yml`."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs"
  verifying_test: "git log --reverse --format='%h %ad %s' -- experiments/ladybug-build-cost references/evaluation xtask/src/main.rs .github/workflows/ci.yml (plus git show <threshold-sha>:experiments/ladybug-build-cost/README.md), transcript pasted into references/evaluation/<dated>.md"
- id: AC-002
  criterion: "GIVEN an adapter author deciding whether they can afford to work in this workspace at all, WHEN they open `experiments/ladybug-build-cost/`, THEN they find, for each of `ubuntu-latest`, `windows-latest` and `macos-latest`, a genuinely cold `cargo build -p happenstance-ladybug --locked` figure taken against a fresh `CARGO_TARGET_DIR` **and** a warm incremental figure beside it — each stamped with toolchain version, runner image, target-directory state and feature set — plus a runner and a `results/` directory that let them re-take the number themselves without asking anyone."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs"
  verifying_test: "experiments/ladybug-build-cost/ — its runner re-run by the reviewer on one runner, and experiments/ladybug-build-cost/results/ inspected for all four stamps on every figure (layout per experiments/position-visibility/README.md)"
- id: AC-003
  criterion: "GIVEN an adapter author who runs `cargo xtask ci` before saying \"done\", WHEN the number lands on one side of the pre-stated threshold, THEN the lever that side selects is **live at the mount point** — `xtask/src/main.rs`'s `REQUIRED`/`OPTIONAL` step tables (`:105`, `:535`) and `.github/workflows/ci.yml`'s `gate` job (`:31-40`) — with the **gate delta** (`cargo xtask ci` wall-clock with and without the crate in the shared steps, on at least one runner) recorded as the figure the choice was argued from; steps are **added, never renamed**, and the decision is not merely described in `RUNBOOK.md` for someone else to wire."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs"
  verifying_test: "cargo xtask ci (the whole gate, not --fast) green on this tree; cargo xtask wasm and cargo xtask lints still resolving their step names against xtask/src/main.rs:816-826; gate-delta figures present in experiments/ladybug-build-cost/results/"
- id: AC-004
  criterion: "GIVEN an adapter author who pushes a change under `crates/happenstance-ladybug/**` six months from now and never read this story, WHEN CI runs, THEN something that **fails the merge** compiles that crate and executes its conformance proof artefact — and it is impossible for the crate to stop being compiled while the CI page stays green: if the crate was excluded from the shared steps, the dedicated job carries no `continue-on-error`, no `paths:` filter that could miss `crates/happenstance-ladybug/**` or `Cargo.lock`, and sits in the branch-protection required set."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs"
  verifying_test: "cargo xtask proof-artefact (xtask/src/proof.rs:132-149 ARTEFACTS row asserted out of --list), plus .github/workflows/ci.yml reviewed against discover.md:122-133 and the branch-protection required-set check dated in references/evaluation/<dated>.md"
- id: AC-005
  criterion: "GIVEN an evaluator who must decide in one sitting whether this project's CI decision was reasoned or rationalised, WHEN they open the dated document under `references/evaluation/`, THEN it gives them, pinned to a commit, the headline cold figure per runner, the warm figure, the gate delta, the threshold as stated **beforehand**, the lever it selected — and a rationale that names what the option **taken** breaks and what was done about it (a split-out job's skippability, or the per-commit tax and who pays it), not only what the option avoided fixes."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs"
  verifying_test: "references/evaluation/<dated>.md present, non-empty, carrying date and commit SHA per references/evaluation/README.md:8-13, read against the symmetry obligation at .bklg/from-contract-to-published-library/ladybug-projection-store/project.md:284-287"
- id: AC-006
  criterion: "GIVEN the next two readers who will look for this decision — a maintainer opening `RUNBOOK.md` at phase 11, and `verdict-ordering-and-publication-handoff` (HS-S0083) assembling the handoff to `publication-and-positioning` (HS-P0016) — WHEN they look where they naturally look, THEN `RUNBOOK.md`'s phase 11 body carries the decision **with the number behind it** (the figure, the runner it was taken on, the shape chosen) with the work item at `:4420-4425` and the exit criterion at `:4437` ticked in place, and the evaluation document states the three handoff facts — the headline number, that docs.rs fails to build `lbug` 0.19.1, and the open `[package.metadata.docs.rs]` question modelled on `happenstance-core`'s keys — **posed, not answered**."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs"
  verifying_test: "git diff -- RUNBOOK.md showing :4420-4425 and :4437 ticked in place with a literal figure in the phase-11 body, plus the handoff heading in references/evaluation/<dated>.md checked against crates/happenstance-ladybug/src/lib.rs:31-32 and crates/happenstance-core/Cargo.toml:54-56 with no metadata key added here"
```
