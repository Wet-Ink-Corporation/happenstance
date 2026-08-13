---
item: HS-P0015
stage: storymap
created: 2026-08-12T03:30:18.110Z
updated: 2026-08-12T03:30:18.110Z
template_sig: 1c63534a
rendered_sig: e89e3b4b
---

# Story Map — The unlike batch shape, and the freeze verdict

## Backbone

The activities, left to right, in the order the evidence has to be produced. The actor is whoever is
holding the freeze: an adapter author meeting `ProjectionStore` for the third time, and the reviewer who
will be asked to believe the answer.

| # | Activity | Outcome the actor gets | ACs under it |
|---|---|---|---|
| **A** | **Decide before you run** | The axes the verdict will be judged on, and the three open questions, are on disk at an earlier commit than any result | AC-001, AC-002 |
| **B** | **Make the adapter real** | `happenstance-ladybug` drives the real `lbug` engine — no `todo!()`, no `stand_in`, no allow — and the port's ergonomics are recorded by a fresh pair of hands | AC-003, AC-006, AC-008 (citations) |
| **C** | **Run the suite on an unlike shape** | Every projection rule reports an outcome against a graph batch, and read-your-own-writes has an answer rather than a hypothesis | AC-004, AC-005 |
| **D** | **Pay the build cost, and be publishable** | The C++ build cost is a number with a CI shape behind it, and the crate is package-complete under a held name | AC-009, AC-010 |
| **E** | **Return the verdict** | A dated *held* / *did not hold*, naming what it was checked against — merged before publication can open | AC-007, AC-008 (routing), AC-011 |

Two things about this backbone are load-bearing rather than decorative. **A precedes C by commit order,
not by intention** — AC-001 is an ordering claim ([`project.md`](project.md), AC-001; DR-3), so activity A
is a separate merge, never a section added to the verdict later. And **E is unconditional**: the artefact
exists whichever way C goes, which is the asymmetry that makes it evidence
([`_intake-brief.md`](_intake-brief.md), Proof artefact).

## Slices

Five milestones. Each is one working, integrated surface: the substrate, the thing that consumes it, and
the registry row that makes the gate notice it, delivered together. Cross-slice `depends_on` edges run
strictly left to right in the merge order below and are acyclic.

| Milestone | Story | Archetype | One-line slice | depends_on | traces_to |
| --------- | ----- | --------- | -------------- | ---------- | --------- |
| `preflight-and-decisions` | `preflight-and-unlike-axes` | foundation | Assert the merged port really shipped `type Batch;`, the write seam and `projection_store_conformance!`, then commit the dated "structurally unlike" axes to `references/evaluation/` as its own merge, before any body is filled in | — | AC-001 |
| `preflight-and-decisions` | `adr-0025-three-answers` | foundation | Author ADR-0025 through `.kb/_intake/` — checkpoint placement, graph-mutation vocabulary, blocking-API bridge — each question with its losers named, plus the long-form record and the decision-map entry | `preflight-and-unlike-axes` | AC-002 |
| `real-adapter` | `real-lbug-driver-swap` | foundation | Replace the `lbug` NOTE in `Cargo.toml` with the real dependency, re-point `Value`/`Error` off `stand_in`, delete `stand_in.rs`, retire `live_handle.rs` with the `port_shape.rs` lines that instantiate it, and rewrite `lib.rs`'s driver-absent and open-decisions sections at ADR-0025 | `adr-0025-three-answers` | AC-003 |
| `real-adapter` | `fill-the-bodies-and-ps-34-disposition` | capability | Fill the four `projection_store.rs` bodies against the real driver, delete `#![allow(clippy::todo)]`, unit-test the `INT64`/`NonZeroU64` narrowing both ways, repair the six `SPECIFICATION.md` citations in the same commit, and record PS-34's disposition from a context that has not read the old transcript | `real-lbug-driver-swap` | AC-003, AC-006, AC-008 |
| `conformance-run` | `ladybug-fixture-and-conformance-run` | capability | Write `LadybugFixture` (a temp directory per instance), invoke `projection_store_conformance!` from a new `tests/` target, extend `port_shape.rs`, and register the target in `xtask/src/proof.rs`'s `ARTEFACTS` so an emptied file cannot pass the gate | `fill-the-bodies-and-ps-34-disposition` | AC-004 |
| `conformance-run` | `read-your-own-writes-projection` | capability | Execute a projection that `MATCH`es a node it created earlier in the same batch, and record the result as a supported behaviour or a declared capability limit of the deferred write set — never as a hypothesis | `ladybug-fixture-and-conformance-run` | AC-005 |
| `packaging-and-ci-shape` | `package-completeness-and-name-claim` | capability | Copy both licences and a README into the crate directory, drop `publish = false`, add the crate to `xtask/src/package.rs`'s `PUBLISHABLE`, and claim `happenstance-ladybug` on crates.io behind a `cargo publish --dry-run` | `fill-the-bodies-and-ps-34-disposition` | AC-010 |
| `packaging-and-ci-shape` | `cold-build-cost-and-ci-shape` | capability | Measure the cold and warm `lbug` build on each matrix runner, then implement and record the CI shape that number buys — excluded-plus-dedicated-job, or accepted in the three-OS gate — in `xtask/src/main.rs`, `.github/workflows/ci.yml` and `RUNBOOK.md`'s phase 11 body | `ladybug-fixture-and-conformance-run` | AC-009 |
| `freeze-verdict` | `freeze-verdict-document` | capability | Write the dated verdict — implementation, rules run, PS clause ids, commit SHA, *held* or *did not hold* — carrying the T1, GAT/ICE and `WriteTransactionInUse` findings, routing any "did not hold" to a decision atom and a re-plan while amending nothing `[FROZEN]` | `read-your-own-writes-projection`, `cold-build-cost-and-ci-shape`, `package-completeness-and-name-claim` | AC-007, AC-008 |
| `freeze-verdict` | `verdict-ordering-and-publication-handoff` | capability | Merge the verdict to the initiative branch ahead of `publication-and-positioning`'s gate, state that ordering explicitly rather than leaving it to the DAG, and hand HS-P0016 the build number, the docs.rs failure and the `[package.metadata.docs.rs]` question | `freeze-verdict-document` | AC-011 |

### Why the slices are cut here

- **`preflight-and-decisions` is two stories, not one, and both are foundation.** They land real
  in-tree artefacts — an evidence document under `references/evaluation/` and an accepted atom under
  `.kb/decisions/` — that the `real-adapter` and `freeze-verdict` slices consume by name. Neither is a
  double or a placeholder, and neither is terminal. The preflight rides with the axes because the axes
  wording *depends on what actually merged*: if `type Batch;` landed, "no transaction handle type" and
  "no lifetime" are different axes ([`_decomposition.md`](_decomposition.md), AC-A01; `_grounding.md`,
  *The dependency this project cannot see past*).
- **`real-adapter`'s two stories cannot be one PR and cannot be two slices.** `stand_in.rs` cannot be
  deleted without re-pointing `Driver(#[from] stand_in::Error)`, and `live_handle.rs` imports that same
  enum — the two modules cannot sit in different type universes
  ([`_decomposition.md`](_decomposition.md), T3). So the swap merges first and compiles with `todo!()`
  bodies still in place; the bodies follow and take the `#![allow(clippy::todo)]` with them. They share a
  slice because the second is the only thing that proves the first: a driver swap with `todo!()` bodies
  behind it is exactly the "adapter" this project exists to stop counting.
- **The six citation repairs ride with the bodies, not in a slice of their own.** `cargo xtask spec-trace`
  is a gate step, so a bodies-only merge leaves the gate red; the architecture brief requires them in the
  same commit, and all six sit in non-normative framing prose
  ([`_decomposition.md`](_decomposition.md), M7).
- **The fixture, the macro invocation and the `ARTEFACTS` row are one story.** Splitting "write the
  fixture" from "wire it into the proof registry" is exactly the split M4 forbids: `cargo test` exits 0 on
  `running 0 tests`, so an emptied target passes the step a deleted one fails. The test target *is* the
  mount, and the registry row is what makes it noticeable.
- **`read-your-own-writes-projection` is its own story inside that slice** because it is the one behaviour
  the deferred write set can plausibly fail (PS-12, `spec/SPECIFICATION.md`; §4.1a question 1 names the
  Ladybug skeleton as the confirming instrument). Merging it with the harness would let a green harness
  stand in for an answer.
- **`packaging-and-ci-shape` is one slice because both stories are the same surface — what the workspace
  pays.** `PUBLISHABLE` and `publish = false` fail the gate together (M5), and the gate steps that make
  the build cost workspace-wide are `--workspace --all-features` in the same `xtask/src/main.rs` (M6).
  A feature flag is not one of the levers.
- **`freeze-verdict` is last and unconditional.** It depends on all three preceding capability slices
  because it must name the rules run, the commit, and the measured number.

### PS-34, and who is allowed to write it

AC-006's constraint is on *who*, not only *what*: the first two pairs of hands wrote `happenstance-sqlite`
and `live_handle.rs` in phase 2, so the record must come from a context that has not read
`crates/happenstance-ladybug/src/live_handle.rs:68-83` first, and must state what documentation it did have
([`_decomposition.md`](_decomposition.md), T2). That is why the disposition is captured *inside*
`fill-the-bodies-and-ps-34-disposition` rather than as a later story: freshness cannot be reconstructed
after the impl is written. Both discharges are valid and the merged port picks which — "the trap is
retired, no `error[E0195]`, no `where Self: 'a` in any impl" if `type Batch;` landed, or the literal
re-test if the GAT survived. This project *reports* on PS-34; moving its marker is
`projection-store-freeze`'s (`RUNBOOK.md:602`).

## Coverage

Every project AC is claimed by at least one story, and no two stories own the same responsibility. Where an
AC appears twice, it is a compound criterion split along a real seam, named in the right-hand column.

| AC | Story / stories | Responsibility split |
|---|---|---|
| **AC-001** | `preflight-and-unlike-axes` | sole owner — an ordering claim about commits |
| **AC-002** | `adr-0025-three-answers` | sole owner |
| **AC-003** | `real-lbug-driver-swap`; `fill-the-bodies-and-ps-34-disposition` | the driver/`stand_in` half; the `todo!()`/allow half |
| **AC-004** | `ladybug-fixture-and-conformance-run` | sole owner |
| **AC-005** | `read-your-own-writes-projection` | sole owner |
| **AC-006** | `fill-the-bodies-and-ps-34-disposition` | sole owner — inseparable from writing the impl |
| **AC-007** | `freeze-verdict-document` | sole owner |
| **AC-008** | `fill-the-bodies-and-ps-34-disposition`; `freeze-verdict-document` | the six citation repairs with `spec-trace` green; the "nothing `[FROZEN]` moved" review and the "did not hold" routing |
| **AC-009** | `cold-build-cost-and-ci-shape` | sole owner |
| **AC-010** | `package-completeness-and-name-claim` | sole owner |
| **AC-011** | `verdict-ordering-and-publication-handoff` | sole owner |

Eleven ACs, ten stories, no orphans and no shared ownership. The Definition of done maps onto the same set:
DoD 1 and 7 are gate obligations every story carries; DoD 2 → `ladybug-fixture-and-conformance-run` +
`read-your-own-writes-projection`; DoD 3 → `adr-0025-three-answers`; DoD 4 → `freeze-verdict-document`;
DoD 5 → `fill-the-bodies-and-ps-34-disposition`; DoD 6 → `cold-build-cost-and-ci-shape` (build number and
CI decision) with the remaining phase 11 boxes ticked by `freeze-verdict-document`.

## Merge order

Foundation before the capability slices that consume it, one slice at a time.

1. **`preflight-and-decisions`** — `preflight-and-unlike-axes`, then `adr-0025-three-answers`.
   *Gate:* the preflight is red-or-green, not negotiable. If `crates/happenstance-core/src/projection.rs`
   has not shipped `type Batch;`, the write seam and `projection_store_conformance!`, this project halts
   and surfaces it to `projection-store-freeze` (HS-P0010) — it is not worked around locally
   ([`_decomposition.md`](_decomposition.md), AC-A01). The `LiveHandleProjectionStore` incompatibility
   (T1) is surfaced upstream from here too, because the crate will otherwise not compile.
2. **`real-adapter`** — `real-lbug-driver-swap`, then `fill-the-bodies-and-ps-34-disposition`.
3. **`conformance-run`** — `ladybug-fixture-and-conformance-run`, then
   `read-your-own-writes-projection`.
4. **`packaging-and-ci-shape`** — `package-completeness-and-name-claim` and
   `cold-build-cost-and-ci-shape`. Unordered with respect to each other; both after slice 3 so the CI
   shape that is chosen is measured against a gate that already runs the conformance target.
5. **`freeze-verdict`** — `freeze-verdict-document`, then
   `verdict-ordering-and-publication-handoff`. Slice 5 merges before `publication-and-positioning`
   (HS-P0016) opens its gate; that edge is the whole of DR-8 and AC-011, and inverting it under schedule
   pressure is what breaks the initiative's own exit criterion
   ([`../_decomposition.md`](../_decomposition.md), Dependency DAG).

`cargo xtask ci` — the whole gate, not `--fast` — is the bar for every slice, because this is the one
place in the portfolio where the wasm32, `cargo-hack`, package-completeness and `spec-trace` steps meet a
new dependency graph ([`_decomposition.md`](_decomposition.md), *Merge-gate commands*).
