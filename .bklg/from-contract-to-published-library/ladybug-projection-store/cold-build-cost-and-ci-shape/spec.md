---
item: HS-S0081
stage: spec
created: 2026-08-12T13:47:19.286Z
updated: 2026-08-12T13:47:19.286Z
template_sig: 87bbf1d0
rendered_sig: 4defa628
---

# Spec — Measure the C++ build cost and implement the CI shape it buys

## Scope lock

| Artifact | Path |
| --- | --- |
| Initiative (gold source) | [`.bklg/from-contract-to-published-library/initiative.md`](../../initiative.md) |
| Initiative decomposition | [`.bklg/from-contract-to-published-library/_decomposition.md`](../../_decomposition.md) |
| Project charter | [`.bklg/from-contract-to-published-library/ladybug-projection-store/project.md`](../project.md) |
| This spec | `.bklg/from-contract-to-published-library/ladybug-projection-store/cold-build-cost-and-ci-shape/spec.md` |
| Key brief — architecture | [`../_decomposition.md`](../_decomposition.md) §3 *Mount points* (M4, M6) and §7 *Build cost is a workspace-wide fact* (`:375-406`); AC-A06 (`:59-61`) |
| Key brief — testing | [`../_decomposition.md`](../_decomposition.md), the **AC-009** row (`:479`) and *Merge-gate commands* (`:517-539`) |
| Grounding | [`../_grounding.md`](../_grounding.md)`:193-198` — no precedent in this workspace builds C++ |
| Story map row | [`../_storymap.md`](../_storymap.md)`:47`, merge order `:136-138` |
| This story's discover | [`discover.md`](discover.md) — the signal ledger, the four wrong implementations, the two questions it deferred here |
| Signed-off design | [`../_design.md`](../_design.md) — records **no user-facing surface** for this project; nothing here renders a surface, and its `## Anti-patterns` are `N/A` by that sign-off |
| Roadmap | `RUNBOOK.md:4393-4444` — phase 11; the work item at `:4420-4425` and the exit criterion at `:4437` are this story's |

## One-line PR slice

Measure the cold and warm `lbug` build on each matrix runner, then implement and record the CI shape that number buys — excluded-plus-dedicated-job, or accepted in the three-OS gate — in `xtask/src/main.rs`, `.github/workflows/ci.yml` and `RUNBOOK.md`'s phase 11 body.

## Executive summary

`real-lbug-driver-swap` (HS-S0075) already put `lbug` in the dependency graph, and
`ladybug-fixture-and-conformance-run` (HS-S0078) already put a conformance target behind it. From that
merge onward every contributor pays a `cmake`-driven C++ build on every `cargo xtask ci`, because the
gate's `clippy` and `tests` steps are `--workspace --all-features` (`xtask/src/main.rs:116-129`,
`:143-151`) over `members = ["crates/*", …]`. **This PR is the pass that decides what to do about
that, on a number rather than on a feeling.**

The delta it lands is three commits in order: a protocol with a **threshold stated before any figure
exists**; the stamped measurements; then the implementation of whichever lever the threshold selects,
plus the decision recorded in `RUNBOOK.md`'s phase 11 body with the number behind it.

What is *not* new here: the dependency, the crate, the conformance target and the `ARTEFACTS` row all
arrive upstream. What is new is a build-configuration decision and its evidence. Nothing `[FROZEN]`
is in the blast radius, no conformance rule is added or changed, and no crate source is touched —
this story edits the gate, the workflow, the runbook and two evidence directories.

## Context pack

Read this before opening anything. It is the set of decisions this story must honour; each is stated
as a decision, with its citation, not as a pointer to go and re-derive.

**1. A feature flag is not one of the levers, and saying so is load-bearing.** Putting `lbug` behind
an off-by-default feature saves nothing, because every gate step turns features on: `clippy` and
`tests` are `--all-features` (`xtask/src/main.rs:116-129`, `:143-151`) and `cargo hack check
--workspace --feature-powerset --no-dev-deps` runs above them (`xtask/src/main.rs:546-556`) and
`cargo-hack` **does** resolve on this machine (`CLAUDE.md`, *Commands*). `default-members` does not
help either — `--workspace` selects every member regardless. Worse, the flag *does* buy one thing:
a configuration in which the adapter is never compiled, where `cargo test` exits 0 on `running 0
tests` (`xtask/src/main.rs:170-178`) and a contributor's local run reports success against a crate
that was never built. The menu is **two** items ([`../_decomposition.md`](../_decomposition.md)`:381-393`),
and this spec's job is to keep the refuted third off it.

**2. The two levers, stated honestly about what each actually saves.**

- **Accept it in the three-OS gate.** `.github/workflows/ci.yml:31-40` already runs `cargo xtask ci`
  on `ubuntu-latest`, `windows-latest` and `macos-latest`. Cost: every contributor and every runner
  pays the C++ build on every commit.
- **`--exclude happenstance-ladybug` on the shared gate steps, plus a dedicated job.** Cost: the
  adapter leaves the gate everyone runs, and a job that is not required, or is behind a `paths:`
  filter, or carries `continue-on-error: true`, stops compiling the adapter while the CI page stays
  green — `project.md:284-287`'s stated risk, realised.

Note the asymmetry the implementer must not blur: excluding the crate from the shared steps but
adding a dedicated **step to the same `xtask` gate** saves only the duplicate all-features and
powerset passes, not the build itself. Only a CI-*job* split removes the cost from a contributor's
local `cargo xtask ci` — and that is precisely the shape that can stop running. Whichever is chosen,
the recorded rationale must address the failure mode of the option **taken**, not only the one
avoided (`project.md:284-287`; [`../_decomposition.md`](../_decomposition.md)`:388-393`).

**3. The threshold is stated before the number is taken.** `discover.md:41-48` carried this forward as
a recommendation for `spec` to accept or refuse with reasons. **This spec accepts it.** Neither
`project.md` nor `RUNBOOK.md` fixes what makes a cost "material", and AC-009's text is satisfied word
for word by a shape chosen first and a figure written underneath it afterwards — which is the same
failure as a verdict written to match its result, in a smaller register, in the same project
(`discover.md:96-108`). The guard is commit order, on the discipline `project.md`'s AC-001 already
uses for the "structurally unlike" axes: the protocol and the threshold merge in a commit that
precedes the commit carrying any figure, and both precede the commit carrying the `xtask` and
workflow edits.

**4. The number that decides is a gate delta, not only a per-crate build.** The brief's protocol — a
genuinely cold `cargo build -p happenstance-ladybug --locked` with a fresh `CARGO_TARGET_DIR` on each
of the three matrix runners plus one warm incremental figure, each stamped with toolchain and machine
([`../_decomposition.md`](../_decomposition.md)`:394-399`) — is required and is the figure AC-009 asks
to see recorded. It is not sufficient on its own to choose a lever, because the lever operates on
*gate steps*: what `--exclude` buys is the difference in `cargo xtask ci` wall-clock with and without
the crate, across steps that build it under more than one profile and feature set. Both figures are
measured; the per-crate cold build is the headline number, the gate delta is the one the decision is
argued from.

**5. Evidence has two homes here and they are not interchangeable.** `discover.md:69-72` deferred
this. **Decision:** the reproducible harness and the raw timings live in
`experiments/ladybug-build-cost/`, on the layout `experiments/position-visibility/` already uses
(`README.md`, a runner, a `results/` directory) — `CLAUDE.md` defines `experiments/` as
"measurements. reproducible, and not in the gate", which is exactly what a re-runnable timing harness
is. The dated, decision-grade document lives in `references/evaluation/`, whose README states the
lifecycle it must obey: "immutable evidence… dated, pinned to a commit… must be superseded rather
than edited" (`references/evaluation/README.md:8-13`). A single figure with no toolchain, runner,
target-directory state or feature set satisfies "the build cost is a number" and is unreproducible and
uncomparable — that is the fourth wrong implementation `discover.md:135-140` names.

**6. Any shape that stops compiling this crate also stops running phase 11's proof artefact.** The
gate runs `cargo xtask proof-artefact` rather than `cargo test` for exactly one reason: `cargo test`
exits 0 on `running 0 tests`, so an emptied target passes a step a deleted one fails
(`xtask/src/main.rs:156-191`; `xtask/src/proof.rs:132-149`). `ladybug-fixture-and-conformance-run`
(HS-S0078) adds this crate's `ARTEFACTS` row. If this story excludes the crate from a step the proof
artefact depends on, it must say where the artefact still runs and what makes that unskippable —
which is why the row and this decision are coupled across slices (`discover.md:110-121`).

**7. The gate's step table resolves by literal name.** `steps_named` panics when a name in
`wasm_steps()` or `lint_steps()` no longer resolves in `REQUIRED` (`xtask/src/main.rs:810-826`), and
that panic is deliberate. Adding a step is safe; **renaming** an existing one, or reordering in a way
that changes what a name selects, is how `cargo xtask wasm` was once pointed at clippy
(`xtask/src/main.rs:769-783`). Add; do not rename.

**8. `--fast` is not this story's bar, and it is the one story where that bites.** `run_fast` drops
`OPTIONAL` — both feature powersets, `cargo deny`, the nightly `docsrs` build
(`xtask/src/main.rs:835-860`) — and the workspace feature powerset is one of the steps that pays the
`lbug` cost. The merge bar is `cargo xtask ci` whole ([`../_storymap.md`](../_storymap.md)`:145-147`;
[`../_decomposition.md`](../_decomposition.md)`:535-539`).

**9. docs.rs is a known separate failure, and this story's obligation to it is a handoff, not a fix.**
`crates/happenstance-ladybug/src/lib.rs:31-32` records that docs.rs itself fails to build `lbug`
0.19.1. Removing `publish = false` — `package-completeness-and-name-claim`'s (HS-S0080) work, this
story's slice-mate — turns that from a note into a consequence, and the call belongs to
`publication-and-positioning` (HS-P0016), whose proof artefact is docs.rs green under `--all-features`
(`RUNBOOK.md:163` — note the architecture brief cites this as `:162`, which is phase **11**'s row;
`:163` is phase 12's, and that is the one that names docs.rs). This story produces the number, states the docs.rs consequence and poses the
`[package.metadata.docs.rs]` question modelled on `crates/happenstance-core/Cargo.toml:54-56`, at a
path `verdict-ordering-and-publication-handoff` (HS-S0083) can cite. It does not decide it.

**The persona slice.** Two actors meet this work. The **contributor** runs `cargo xtask ci` before
saying "done" (`CLAUDE.md`, *Commands*) and either pays a multi-minute C++ build every time or does
not — and if they do not, they need to know where the adapter *is* compiled. The **reviewer holding
the freeze** is asked to believe a decision; what makes it believable is that the threshold was on
disk before the number, and that the rationale names what the chosen option breaks. Both are served
by the same artefacts, and neither is served by a green tick.

## Integration contract

- **Archetype**: `capability` (per `story.md` frontmatter `archetype: capability`).
- **Slice / milestone**: `packaging-and-ci-shape`. Slice-mate: **`package-completeness-and-name-claim`**
  (HS-S0080). The two are one surface — *what the workspace pays* — and are unordered with respect to
  each other ([`../_storymap.md`](../_storymap.md)`:80-82`, `:136-138`).
- **Mount point**: **`xtask/src/main.rs`** — the `REQUIRED` / `OPTIONAL` step tables (`:105`, `:535`).
  This is the gate, defined once, and it is AC-009's composition root by the architecture brief's own
  words ([`../_decomposition.md`](../_decomposition.md)`:185-191`). A CI shape that exists only in a
  workflow file, or only in prose, is not mounted.
- **Co-mount**: **`.github/workflows/ci.yml`** — the `gate` job and its three-OS matrix (`:31-40`).
  Whichever lever is chosen, this file is where "required, unfiltered, fails the merge" is either true
  or false.
- **Wires into**: `xtask/src/proof.rs`'s `ARTEFACTS` (`:132-149`, the row HS-S0078 adds — read, not
  edited here); `xtask/src/main.rs`'s `run_ci` / `run_fast` / `steps_named` (`:810-860`);
  `.redkiln/config.yaml`'s `verify.e2e` and `verify.integration_scoped` (`:55`, `:60`), which are what
  make the gate's shape a backlog-visible fact rather than a local habit;
  `crates/happenstance-ladybug/Cargo.toml` (read for its feature set and its `publish` key — edited by
  HS-S0080 and HS-S0075, not here); `references/evaluation/README.md:8-13` (the evidence lifecycle);
  `experiments/position-visibility/` (the harness layout to copy).
- **Renders surfaces**: **none.** [`../_design.md`](../_design.md) records no user-facing surface for
  this project and no `## Items`; this story adds no public Rust item.
- **Advances DoD scenario**: initiative **DoD 13** — *"The gate is green on the assembled whole.
  `cargo xtask ci` passes … on the exact tree that was published"*
  ([`../../initiative.md`](../../initiative.md)`:396-397`). A gate whose cost has not been decided is
  a gate that gets narrowed under pressure later. It also supplies one of the three facts **DoD 8**'s
  verdict is assembled from ([`../../initiative.md`](../../initiative.md)`:380-382`) and ticks the
  project's own DoD 6 (`project.md:237-238`; `RUNBOOK.md:4437`).

This story is delivered **mounted**: the chosen shape is live in `xtask/src/main.rs` and
`.github/workflows/ci.yml` on merge, not described in `RUNBOOK.md` for someone else to wire.

## PR boundary

`redkiln verify --grain story` reads the first fenced block below and fails on any file changed
outside it.

```
xtask/src/main.rs
.github/workflows/ci.yml
RUNBOOK.md
experiments/ladybug-build-cost/**
references/evaluation/**
.bklg/from-contract-to-published-library/ladybug-projection-store/cold-build-cost-and-ci-shape/**
```

Deliberately narrow. `crates/**` is absent because this story adds no Rust source and no dependency:
`lbug` arrives with `real-lbug-driver-swap` (HS-S0075) and the conformance target with
`ladybug-fixture-and-conformance-run` (HS-S0078). `xtask/src/proof.rs` is absent for the same reason —
its `ARTEFACTS` row is HS-S0078's, and this story reads it. `spec/SPECIFICATION.md` is absent because
no clause is touched.

**In this PR**

- `experiments/ladybug-build-cost/` — the re-runnable protocol, the pre-stated materiality threshold,
  and the raw stamped results.
- A dated document under `references/evaluation/` — the headline cold figure per runner, the warm
  figure, the gate delta, the decision the threshold selects, the rationale addressing what the chosen
  option breaks, and the docs.rs / `[package.metadata.docs.rs]` handoff facts.
- `xtask/src/main.rs` — the chosen shape implemented in the step table.
- `.github/workflows/ci.yml` — the job shape that matches it, including whatever makes it unskippable.
- `RUNBOOK.md` phase 11 — the work item at `:4420-4425` and the exit criterion at `:4437`, ticked in
  place with the number and the decision in the body.
- This story's own backlog folder (`_ledger.md`, the implementation report).

**Explicitly not in this PR**

- Adding or upgrading the `lbug` dependency, or any edit under `crates/**` (HS-S0075, HS-S0080).
- The `ARTEFACTS` row or any conformance rule (HS-S0078; the suite is `projection-store-freeze`'s).
- `publish = false`, the licence files, the README, the crates.io claim (HS-S0080).
- The docs.rs decision and the `[package.metadata.docs.rs]` key itself (HS-P0016); this story poses
  the question and hands over the number.
- The freeze verdict document and its ordering claim (HS-S0082, HS-S0083).
- Any `spec/SPECIFICATION.md` clause text or marker, and any ADR.

**Merge DoD.** `cargo xtask ci` — the whole gate, not `--fast` — is green on this tree; the three
commits are in the stated order; `RUNBOOK.md`'s phase 11 body carries the decision **with the number
behind it**; and the adapter is provably still compiled and its conformance target still executed by
something that fails a merge.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **The threshold precedes the number** | `experiments/ladybug-build-cost/README.md` states, before any figure exists, what makes the cost "material" — a stated wall-clock figure or ratio, the configuration it is read against, and which lever each side of it selects. Merged in its own commit, ahead of every commit carrying a result. A threshold written after the number is a justification, not a criterion. | `discover.md:41-48`, `:96-108`; `project.md:180-184` (the same ordering discipline, applied to AC-001) |
| **Cold build, per matrix runner, stamped** | A genuinely cold `cargo build -p happenstance-ladybug --locked` with a fresh `CARGO_TARGET_DIR` on `ubuntu-latest`, `windows-latest` and `macos-latest`. Windows is not optional in the sample: it is first-class here by explicit comment and is the runner most likely to be worst for a `cmake` build. Each figure stamped with toolchain, runner image, target-directory state and feature set. | [`../_decomposition.md`](../_decomposition.md)`:394-399`; `.github/workflows/ci.yml:34-39`; `discover.md:31-39` |
| **Warm incremental figure** | One warm figure alongside each cold one. Cold is what a CI runner and a first-time contributor pay; warm is what a contributor pays on every subsequent commit. They argue for different things and neither substitutes for the other. | `discover.md:31-39`; [`../_decomposition.md`](../_decomposition.md)`:394-399` |
| **Gate delta** | `cargo xtask ci` wall-clock measured with and without the crate in the shared steps, on at least one runner. Required because the lever operates on gate steps — `clippy`, `tests` and the feature powerset build the crate under different profiles and feature sets, so a per-crate figure alone cannot say what `--exclude` buys. | `xtask/src/main.rs:116-129`, `:143-151`, `:546-556`; [`../_decomposition.md`](../_decomposition.md)`:381-387` |
| **The harness is re-runnable** | `experiments/ladybug-build-cost/` carries a runner and a `results/` directory on the layout `experiments/position-visibility/` uses, so the number can be re-taken when `lbug` is upgraded. `CLAUDE.md` defines `experiments/` as reproducible measurements that are **not in the gate** — this adds no gate step and no test target. | `CLAUDE.md`, repository map; `experiments/position-visibility/` |
| **The decision-grade document is immutable evidence** | One dated file under `references/evaluation/`, pinned to a commit, carrying the figures, the decision and the rationale. Superseded rather than edited, per the directory's own stated lifecycle. It is the artefact `RUNBOOK.md` and the freeze verdict cite. | `references/evaluation/README.md:8-13`; [`../_decomposition.md`](../_decomposition.md)`:210-218` |
| **The lever is implemented at the mount point** | Whichever side of the threshold the number falls, the shape is live in `xtask/src/main.rs`'s step table — either the shared steps are unchanged (cost accepted) or they carry `--exclude happenstance-ladybug` and a dedicated step/job appears. Steps are **added**, never renamed: `steps_named` resolves by literal name and panics on a miss. | `xtask/src/main.rs:105`, `:535`, `:810-826`; [`../_decomposition.md`](../_decomposition.md)`:185-191` |
| **A split-out job is required, unfiltered, and fails the merge** | If the crate is excluded, the acceptance condition is not "the job exists". It is: no `continue-on-error`, no `paths:` filter that could miss `crates/happenstance-ladybug/**` or `Cargo.lock`, and the job is in the branch-protection required set. Three well-known green-looking spellings of a job that never runs. | `discover.md:122-133`; `project.md:284-287`; `.github/workflows/ci.yml:28-40` |
| **The proof artefact still runs, provably** | Under the chosen shape, phase 11's conformance target is still executed by `cargo xtask proof-artefact`, which asserts the named tests out of `--list` before running them — because `cargo test` exits 0 on `running 0 tests`. If exclusion moves where that happens, the document says where and what makes it unskippable. | `xtask/src/main.rs:156-191`; `xtask/src/proof.rs:132-149`; `discover.md:110-121` |
| **The rationale addresses the option taken** | Symmetrical obligation: excluding must say what makes the dedicated job unskippable; keeping it in the matrix must say what makes the per-commit tax acceptable and to whom. A rationale that addresses only its own upside is incomplete. | `project.md:284-287`; [`../_decomposition.md`](../_decomposition.md)`:388-393`; `discover.md:50-55` |
| **`RUNBOOK.md` phase 11 records the decision with its number** | The work item at `:4420-4425` and the exit criterion at `:4437` are ticked in place, and the phase body carries the figure, the runner it was taken on, and the decision — not a link standing in for the number. A decision recorded without its number does not discharge AC-009. | `RUNBOOK.md:4409-4438`; `project.md:214-217`; `discover.md:17` |
| **The publication handoff facts exist at a citable path** | The evaluation document states the headline number, that docs.rs fails to build `lbug` 0.19.1, and the open `[package.metadata.docs.rs]` question modelled on `happenstance-core`'s keys — in a form HS-S0083 can hand to HS-P0016 by path. Posed, not answered. | `crates/happenstance-ladybug/src/lib.rs:31-32`; `crates/happenstance-core/Cargo.toml:54-56`; [`../_decomposition.md`](../_decomposition.md)`:401-406`; [`../_storymap.md`](../_storymap.md)`:49` |
| **Commit order is the evidence** | Three commits, in order: (1) protocol + threshold, no figures; (2) raw results + the dated evaluation document; (3) `xtask` + workflow + `RUNBOOK.md`. `verify.require_commit_provenance` is on, so the commits are recorded on the item anyway. | `.redkiln/config.yaml:73`; `discover.md:104-108` |
| **No new public Rust surface, no gate weakening** | No item under `crates/**` changes; no conformance rule is added, removed or `#[cfg]`-ed out; no `[FROZEN]` clause marker or text moves; the four `wasm32` steps and the five file-reading lints are untouched. A gate step is never deleted to make a number look better. | [`../_design.md`](../_design.md) (no surface); `xtask/src/main.rs:192-283`, `:799-808`; `project.md:210-213` |

Interfaces, precisely: the only machine-readable "interface" this story changes is the **argument
vector of gate steps** in `xtask/src/main.rs` and the **job graph** in `.github/workflows/ci.yml`.
Everything else it produces is prose evidence read by a human — which is why the acceptance criteria
below lean on commit order, on file presence at named paths, and on `cargo xtask ci` staying green,
rather than on assertions in a test target.

## Data and migrations

**N/A — no runtime data and no migration.** This story adds no schema, no persisted format, no
serialisation and no stored state; it edits build configuration, a workflow, a runbook body and two
evidence directories. The only durable artefacts it creates are markdown, and their "migration"
policy is the one `references/evaluation/README.md:8-13` already states: a document there is dated,
pinned to a commit, and **superseded rather than edited** when a later `lbug` upgrade changes the
number. The `experiments/ladybug-build-cost/` harness is the opposite lifecycle by design — it is
re-run and its `results/` grow, which is what makes the superseding document writable at all.

## Acceptance criteria

Six criteria. Every one is framed from a persona goal crossing the whole stack — the personas are
this initiative's own, carried from
[`../../_discovery/distillation/personas-and-journeys.md`](../../_discovery/distillation/personas-and-journeys.md)
and named at [`../../initiative.md`](../../initiative.md)`:227-250`: the **adapter author** on the
*learn when you are finished* journey (here in their contributor guise, running `cargo xtask ci`
before saying "done"), and the **evaluator** on *decide in one sitting*, who reads public evidence and
either believes a claim or does not. Together they discharge project **AC-009**
([`../project.md`](../project.md)`:214-217`) and the phase-11 exit criterion at `RUNBOOK.md:4437`.

| id | criterion | verification |
| --- | --- | --- |
| **AC-001** | **GIVEN** an evaluator who has been handed a CI-shape decision and no reason to trust it, **WHEN** they run `git log` over this story's paths, **THEN** the commit introducing `experiments/ladybug-build-cost/README.md` — which states, in that commit, what makes the cost *material* (a wall-clock figure or ratio, the configuration it is read against, and which of the two levers each side of it selects) and contains **no measured figure** — strictly precedes every commit carrying a result, and both strictly precede the commit that edits `xtask/src/main.rs` or `.github/workflows/ci.yml`. | Process/static. `git log --reverse --format='%h %ad %s' -- experiments/ladybug-build-cost references/evaluation xtask/src/main.rs .github/workflows/ci.yml` on the story branch, and `git show <threshold-sha>:experiments/ladybug-build-cost/README.md` showing a threshold and no figure. The transcript is pasted into the dated evaluation document and cited in `_ledger.md`. Same instrument `project.md:180-184` uses for AC-001's axes; `.redkiln/config.yaml:73`'s `require_commit_provenance` records the commits on the item independently. |
| **AC-002** | **GIVEN** an adapter author deciding whether they can afford to work in this workspace at all, **WHEN** they open `experiments/ladybug-build-cost/`, **THEN** they find, for each of `ubuntu-latest`, `windows-latest` and `macos-latest`, a genuinely cold `cargo build -p happenstance-ladybug --locked` figure taken against a fresh `CARGO_TARGET_DIR` **and** a warm incremental figure beside it — each stamped with toolchain version, runner image, target-directory state and feature set — plus a runner and a `results/` directory that let them re-take the number themselves without asking anyone. | Process/static, re-runnable. File presence and content review of `experiments/ladybug-build-cost/README.md`, its runner and `results/`; every recorded figure carries all four stamps (a figure missing one is the fourth wrong implementation, `discover.md:135-140`). Reproducibility is proven by the reviewer re-running the harness on one runner and getting a figure of the same order. Layout mirrors `experiments/position-visibility/README.md`. |
| **AC-003** | **GIVEN** an adapter author who runs `cargo xtask ci` before saying "done", **WHEN** the number lands on one side of the pre-stated threshold, **THEN** the lever that side selects is **live at the mount point** — `xtask/src/main.rs`'s `REQUIRED`/`OPTIONAL` step tables (`:105`, `:535`) and `.github/workflows/ci.yml`'s `gate` job (`:31-40`) — with the **gate delta** (`cargo xtask ci` wall-clock with and without the crate in the shared steps, on at least one runner) recorded as the figure the choice was argued from; steps are **added, never renamed**, and the decision is not merely described in `RUNBOOK.md` for someone else to wire. | Integration + static. `cargo xtask ci` green on this tree (the whole gate, not `--fast`, per [`../_storymap.md`](../_storymap.md)`:145-147`); `cargo xtask wasm` and `cargo xtask lints` still resolve their step names, proving no rename tripped `steps_named`'s deliberate panic (`xtask/src/main.rs:816-826`); `git diff` over `xtask/src/main.rs` reviewed for added-not-renamed; the gate-delta figures present in `experiments/ladybug-build-cost/results/` with the run configuration stated (including whether the `cargo hack` powerset step ran). |
| **AC-004** | **GIVEN** an adapter author who pushes a change under `crates/happenstance-ladybug/**` six months from now and never read this story, **WHEN** CI runs, **THEN** something that **fails the merge** compiles that crate and executes its conformance proof artefact — and it is impossible for the crate to stop being compiled while the CI page stays green: if the crate was excluded from the shared steps, the dedicated job carries no `continue-on-error`, no `paths:` filter that could miss `crates/happenstance-ladybug/**` or `Cargo.lock`, and sits in the branch-protection required set. | Integration + reviewed. `cargo xtask proof-artefact` runs the crate's `ARTEFACTS` row (`xtask/src/proof.rs:132-149`) and asserts the named tests out of `--list` before running them, so an emptied target fails where `cargo test`'s `running 0 tests` would pass (`xtask/src/main.rs:170-191`). Workflow review against `discover.md:122-133`'s three green-looking spellings. Branch protection is not in the tree: the check is run against the repository's settings and its result **and date** recorded in the evaluation document. |
| **AC-005** | **GIVEN** an evaluator who must decide in one sitting whether this project's CI decision was reasoned or rationalised, **WHEN** they open the dated document under `references/evaluation/`, **THEN** it gives them, pinned to a commit, the headline cold figure per runner, the warm figure, the gate delta, the threshold as stated **beforehand**, the lever it selected — and a rationale that names what the option **taken** breaks and what was done about it (a split-out job's skippability, or the per-commit tax and who pays it), not only what the option avoided fixes. | Static + review. File presence at `references/evaluation/<dated>.md`, non-empty, carrying the commit SHA and date the directory's lifecycle requires (`references/evaluation/README.md:8-13`); reviewer reads the rationale against the symmetry obligation at `project.md:284-287` and [`../_decomposition.md`](../_decomposition.md)`:388-393` — a rationale addressing only its own upside fails this criterion even when every figure is present. |
| **AC-006** | **GIVEN** the next two readers who will look for this decision — a maintainer opening `RUNBOOK.md` at phase 11, and `verdict-ordering-and-publication-handoff` (HS-S0083) assembling the handoff to `publication-and-positioning` (HS-P0016) — **WHEN** they look where they naturally look, **THEN** `RUNBOOK.md`'s phase 11 body carries the decision **with the number behind it** (the figure, the runner it was taken on, the shape chosen) with the work item at `:4420-4425` and the exit criterion at `:4437` ticked in place, and the evaluation document states the three handoff facts — the headline number, that docs.rs fails to build `lbug` 0.19.1, and the open `[package.metadata.docs.rs]` question modelled on `happenstance-core`'s keys — **posed, not answered**. | Static + review. `git diff` over `RUNBOOK.md` showing the two boxes ticked in place and a body carrying a literal figure (a link standing in for the number does not discharge project AC-009, `discover.md:17`); the handoff paragraph present at a stable heading HS-S0083 can cite by path, checked against `crates/happenstance-ladybug/src/lib.rs:31-32` and `crates/happenstance-core/Cargo.toml:54-56`; reviewer confirms no `[package.metadata.docs.rs]` key was actually added here (that is HS-P0016's). |

Every criterion above maps to project **AC-009** and to no other; AC-009's own verification tier is
"process (measurement), not a suite tier" by the testing brief's own row
([`../_decomposition.md`](../_decomposition.md)`:479`), which is why five of the six are proven by
commit order, file presence and review, and only AC-003 and AC-004 are proven by a command that
compiles anything.

## Interaction quality

**Composition family: N/A, by sign-off — not by omission.** [`../_design.md`](../_design.md) records
`N/A — no user-facing surface` under every heading including `## Anti-patterns` (`:40-95`), and states
that the no-surface determination *is* the design decision a human signed off. This story renders no
screen, no documentation page and no public Rust item, so there is no presentation, placement,
transience, density budget, hierarchy or named anti-pattern to hold it to. Writing composition ACs
here would be inventing a surface the sign-off says does not exist.

**State family: applicable in its library analogue, and carried by AC rows — not by these bullets.**
The developer-facing surface this story genuinely changes is the *gate*: what `cargo xtask ci` does,
what it prints, and what a contributor can conclude from a green run. The invariants that survive that
translation, and the AC that carries each:

| State invariant | Library analogue in this story | Carried by | Verified by |
| --- | --- | --- | --- |
| **In place, not a context jump** | The chosen shape is live in `xtask/src/main.rs` and `.github/workflows/ci.yml` on merge; it is not described in `RUNBOOK.md` for a later story to wire. | **AC-003** | `cargo xtask ci` green with the shape in effect; `git diff` over the mount point. |
| **Non-occlusion** — nothing important hidden by the change | A shape that stops compiling the adapter while the CI page stays green is the occlusion failure in this medium. The crate's compilation and its conformance proof artefact must stay visible to something that fails a merge. | **AC-004** | `cargo xtask proof-artefact`; workflow review against `discover.md:122-133`. |
| **Preserved state across the change** — nothing silently dropped | No gate step is deleted, renamed or weakened to make the number look better: the four `wasm32` steps, the five file-reading lints and `spec-trace` are untouched, and `steps_named`'s panic is the mechanism that would catch a rename. | **AC-003** | `cargo xtask wasm`, `cargo xtask lints`, `cargo xtask spec-trace` all still resolve their step names; `xtask/src/main.rs:816-826`. |
| **Reversibility** | The measurement is a re-runnable harness with a `results/` directory, so a later `lbug` upgrade can re-take the number and supersede the decision rather than being stuck with a one-off figure nobody can reproduce. | **AC-002** | Reviewer re-runs the harness on one runner. |
| **Reachability / discoverability** | The decision is findable where each reader already looks — `RUNBOOK.md` phase 11 for a maintainer, a citable path for the publication handoff — rather than only in a merged PR description. | **AC-006** | `git diff` over `RUNBOOK.md`; the handoff heading resolvable by path from HS-S0083. |

No invariant in this section exists only as a bullet: each is written into the criterion text of the
AC named beside it, so `redkiln verify --grain story` extracts it as a ledger row and it is gated.

## Error conditions

| id | condition | required handling |
| --- | --- | --- |
| **EC-001** | A "cold" build was not actually cold — a warm `CARGO_TARGET_DIR`, a shared cache, a registry already populated, or CI cache restoration in front of the build. | The figure is invalid and is re-taken. The harness must state *how* coldness was established (fresh target directory, and whether the registry/`~/.cargo` was primed), because "cold" without that is unfalsifiable. Any CI-run measurement must have cache restoration disabled for the measured step and say so. |
| **EC-002** | The gate delta was measured with the `cargo hack` feature-powerset step skipped (the probe found no tool) while the real gate runs it, or vice versa. | The recorded delta names which steps ran. `cargo-hack` resolves on this machine and the powerset step is one of the steps that pays the `lbug` cost (`xtask/src/main.rs:546-556`; `CLAUDE.md`, *Commands*) — a delta taken without it understates the saving and must not be compared against one taken with it. |
| **EC-003** | The crate's feature set changes after measurement — most plausibly ADR-0025 Q3 landing an optional runtime feature — so the figure is a figure for a configuration that no longer exists. | The figure states the feature set it was taken under. When the configuration moves, the `references/evaluation/` document is **superseded by a new dated document**, never edited (`references/evaluation/README.md:8-13`); the `experiments/` harness is re-run and its `results/` grow. |
| **EC-004** | `lbug`'s `cmake`/C++ build fails outright on one runner — `windows-latest` being the most likely, and the one this workspace treats as first-class (`.github/workflows/ci.yml:34-35`). | That is a **result**, not a blocked story. It is recorded as the measurement outcome for that runner, and its consequence is routed to `freeze-verdict-document` (HS-S0082) and, for publication, to HS-S0083 — never resolved by quietly dropping the runner from the matrix or from the sample. |
| **EC-005** | An implementer renames an existing gate step (rather than adding one) while wiring the lever. | `steps_named` panics at runtime when a name in `wasm_steps()` or `lint_steps()` no longer resolves in `REQUIRED` (`xtask/src/main.rs:816-826`). That panic is the designed alarm and must be treated as a stop, not routed around by editing the name lists — it is how `cargo xtask wasm` was once pointed at clippy (`:769-783`). |
| **EC-006** | Branch protection cannot be read or changed from the working tree, so "the dedicated job is in the required set" cannot be proven by any file in this PR. | The check is performed against the repository's settings and its **result and date** are recorded in the evaluation document as an explicit, human-attested line. An unverifiable claim is not written as if it were verified; if the required set cannot be confirmed, that is itself an argument against the split-out lever and must appear in the rationale. |
| **EC-007** | The threshold's own commit is amended, rebased or squashed after a figure exists, destroying the ordering evidence. | The ordering proof is the point (AC-001). Once the threshold commit is pushed, it is not amended; a correction to the threshold is a **new** commit stating why, still ahead of the figures. Squash-merging this PR would collapse all three commits — the merge must preserve them, and the `git log` transcript is captured in the evaluation document as a durable record either way. |

## Non-functional

| id | requirement | why |
| --- | --- | --- |
| **NF-001** | The harness adds **no gate step and no test target**. `experiments/` is defined as "measurements. reproducible, and not in the gate" (`CLAUDE.md`, repository map), and a timing harness wired into `cargo xtask ci` would make the gate flaky on shared runners for no proof gained. | The number is evidence, not a check. |
| **NF-002** | No gate step is deleted, disabled, `--fast`-only or feature-gated to improve the measured figure. The bar for this story is `cargo xtask ci` whole, not `--fast` — `run_fast` drops the two feature powersets, which are among the steps that pay the cost being measured (`xtask/src/main.rs:853-860`). | Measuring a gate you have first made cheaper measures nothing. |
| **NF-003** | Reproducibility: a third party with the repository and a runner can re-take every headline figure from `experiments/ladybug-build-cost/` alone, without reading this spec. | The comparison that matters happens later, when someone asks whether an `lbug` upgrade made things worse. |
| **NF-004** | Evidence immutability: the `references/evaluation/` document is dated, pinned to a commit, and superseded rather than edited, per that directory's stated lifecycle (`references/evaluation/README.md:8-13`). | It is cited by `RUNBOOK.md` and by the freeze verdict; an edited record breaks both citations silently. |
| **NF-005** | Whatever shape is chosen, a contributor can tell **from the run** whether the adapter was compiled in it. A configuration in which `cargo test` reports success against a crate that was never built is forbidden (`xtask/src/main.rs:170-178`; `discover.md:110-121`). | This is the failure the refuted feature-flag lever would have introduced; the chosen lever must not reintroduce it. |
| **NF-006** | The PR touches no file outside the fenced boundary above — in particular nothing under `crates/**` and no `spec/SPECIFICATION.md` clause, marker or ADR. | Enforced by `redkiln verify --grain story` against the PR boundary block. |

## Implementation notes (non-prescriptive)

These are aids, not instructions; the criteria above are the contract.

**Sequence.** Three commits, and the order is itself AC-001's evidence: (1) `experiments/ladybug-build-cost/README.md` with the protocol and the threshold and nothing else; (2) `results/` plus the dated `references/evaluation/` document; (3) `xtask/src/main.rs`, `.github/workflows/ci.yml` and `RUNBOOK.md`. Do not squash on merge.

**Getting three runners' figures.** The three matrix runners are GitHub-hosted; a `workflow_dispatch`-triggered temporary workflow (or a one-off branch push) is the honest way to take the numbers on the same images `.github/workflows/ci.yml:36-39` names, rather than substituting three local machines. If a temporary workflow is used to collect figures, it is not part of the delivered shape and should not survive the PR — the evidence it produced does. Disable cache restoration in front of the measured step (EC-001).

**Where `--exclude` would go, if the number selects it.** The shared steps that build the crate are `clippy` (`xtask/src/main.rs:116-129`), `tests` (`:143-151`) and the `cargo hack` powerset (`:546-556`); the lever is an argument added to those `Step` argv arrays, plus a dedicated job in the workflow. Recall the asymmetry from the context pack: excluding from the shared steps but adding a dedicated **step to the same gate** saves only the duplicate all-features/powerset passes — only a CI **job** split removes the cost from a contributor's local `cargo xtask ci`, and that is exactly the shape that can stop running (AC-004).

**If the split is chosen, prefer the same three-OS matrix for the dedicated job.** A single-runner dedicated job hides the runner most likely to be worst for a `cmake` build, which is `windows-latest` and which this workspace declares first-class by explicit comment. Deviating from three is allowed, with the reason recorded — see Clarifications.

**Measuring the gate delta honestly.** Two `cargo xtask ci` runs on the same runner in the same state (both cold, or both warm — say which), one with the crate in the shared steps and one with it excluded. Report wall-clock and the step list that ran. One runner is sufficient for the delta; three are required for the per-crate cold figure.

**The `RUNBOOK.md` edit is in place.** Tick `- [ ]` → `- [x]` at `:4420-4425` and `:4437` and put the figure and the decision in the phase-11 body; do not append a new section beside the phase. `cargo xtask spec-trace` does not read `RUNBOOK.md`, so nothing catches a citation that drifts here — quote the number rather than only linking to the evaluation document.

**Watch the line-number citations you add.** Several artefacts in this project cite `RUNBOOK.md` and `xtask/src/main.rs` by `file:line`; editing `RUNBOOK.md` in place shifts nothing below phase 11 only if the edit is box-ticks and body text of equal-or-greater length. If it shifts, `cargo xtask spec-trace` protects `spec/SPECIFICATION.md`'s citations but nothing protects `.bklg/` prose — re-check any citation you personally moved.

## Tests and CI (merge gate)

Grounded in the project testing brief's **AC-009** row ([`../_decomposition.md`](../_decomposition.md)`:479`, which classifies this work as *process (measurement), not a suite tier*) and its *Merge-gate commands* block (`:517-539`, which withholds a `--fast` bar from this project precisely because `--fast` may not exercise the new `lbug` build path at all).

| tier | command / path | proves |
| --- | --- | --- |
| Merge gate (whole) | `cargo xtask ci` | The chosen shape compiles, the adapter is still built by something in the gate, the four `wasm32` steps and `spec-trace` are unaffected. **AC-003**, NF-002. |
| Story grain | `cargo xtask affected --base main` (`.redkiln/config.yaml:40`) | The story-grain gate redkiln runs on both sides of `implement`; catches an `xtask` edit that breaks the file-reading lints or `spec-trace` before the whole gate is run. **AC-003**. |
| Integration (proof artefact) | `cargo xtask proof-artefact` (`xtask/src/main.rs:156-191`; `xtask/src/proof.rs:132-149`) | The conformance target named in `ARTEFACTS` is asserted out of `--list` and then run, so an excluded or emptied target fails rather than reporting `running 0 tests`. **AC-004**, NF-005. |
| Static (step-name resolution) | `cargo xtask wasm` and `cargo xtask lints` | Every name in `wasm_steps()`/`lint_steps()` still resolves in `REQUIRED` — proof that steps were added, not renamed (`xtask/src/main.rs:816-826`). **AC-003**, EC-005. |
| Process (commit order) | `git log --reverse --format='%h %ad %s' -- experiments/ladybug-build-cost references/evaluation xtask/src/main.rs .github/workflows/ci.yml`, plus `git show <threshold-sha>:experiments/ladybug-build-cost/README.md` | The threshold existed, figureless, before any measurement, and both preceded the implementation. **AC-001**, EC-007. |
| Process (measurement) | `experiments/ladybug-build-cost/` — its runner re-run on one runner by the reviewer; `results/` inspected for the four stamps | The figures are real, stamped and reproducible. **AC-002**, NF-003. |
| Static (evidence presence) | `references/evaluation/<dated>.md` present, non-empty, carrying a commit SHA and date | The decision-grade record exists on the lifecycle the directory requires. **AC-005**, NF-004. |
| Review (workflow shape) | `.github/workflows/ci.yml` read against `discover.md:122-133`; branch-protection required set checked against the repository settings and the result dated in the evaluation document | No `continue-on-error`, no `paths:` filter that misses `crates/happenstance-ladybug/**` or `Cargo.lock`, and the job actually blocks a merge. **AC-004**, EC-006. |
| Review (rationale symmetry) | The evaluation document read against `project.md:284-287` | The rationale addresses what the option **taken** breaks. **AC-005**. |
| Review (record) | `git diff -- RUNBOOK.md` | Phase 11's two boxes ticked in place and the body carrying a literal figure plus the handoff facts. **AC-006**. |
| Ledger gate | `redkiln verify --grain story` against `_ledger.md` (`.redkiln/config.yaml:67`, `:73`) | Every AC above carries cited evidence and a recorded work commit before `implement → report`. |

No new test target and no new gate step is added by this story (NF-001): the harness lives in `experiments/`, which `CLAUDE.md` defines as out of the gate.

## Risks and coupling (PR-scoped)

- **The split-out job that silently stops running.** The headline risk, named at `project.md:284-287` and spelled three ways at `discover.md:122-133`. Mitigation is AC-004's acceptance condition — *required, unfiltered, fails the merge* — rather than "the job exists", plus EC-006's dated human attestation where the tree cannot prove it.
- **The number written to fit a shape already chosen.** Mitigated by AC-001's commit order and by EC-007's no-amend rule. This is the same failure the freeze verdict faces one slice later, in a smaller register (`discover.md:96-108`).
- **Cross-slice coupling with HS-S0078's `ARTEFACTS` row.** This story *reads* `xtask/src/proof.rs` and must not edit it, yet AC-004 depends on that row existing. If `ladybug-fixture-and-conformance-run` merged without the row, AC-004 cannot be discharged — halt and say so rather than adding the row here and widening the PR boundary.
- **Cross-slice coupling with HS-S0080 (`package-completeness-and-name-claim`).** Its removal of `publish = false` turns the known docs.rs failure at `crates/happenstance-ladybug/src/lib.rs:31-32` from a note into a consequence. The two stories are unordered ([`../_storymap.md`](../_storymap.md)`:80-82`), so this story must state the docs.rs fact and pose the `[package.metadata.docs.rs]` question **whether or not** HS-S0080 has landed, and must not pre-empt HS-P0016's decision.
- **Measurement confound from ADR-0025 Q3.** If the blocking-API bridge landed an optional runtime feature, the powerset step compiles configurations the plain build does not, and a figure that does not name its feature set is not comparable (EC-003; `discover.md:57-62`).
- **`RUNBOOK.md` line drift.** Editing phase 11 in place can move line numbers that other `.bklg/` prose cites; nothing in the gate protects those. Scoped to this PR by keeping the edit to box-ticks and body text.
- **Runner variance and one-shot figures.** GitHub-hosted runners vary; a single sample per runner can mislead by tens of percent. Not fatal — the decision hinges on a threshold, not on a precise figure — but the document should say how many runs each figure summarises.
- **Not in the blast radius, deliberately:** any `[FROZEN]` clause, any conformance rule, any file under `crates/**`, any ADR. If implementing this story appears to require one of those, the boundary is wrong and the story stops.

## Dependencies

**Blocks on**

- **`ladybug-fixture-and-conformance-run`** (HS-S0078) — matches this story's `depends_on`. It supplies the `projection_store_conformance!` target and the `xtask/src/proof.rs` `ARTEFACTS` row, which are what make the measured gate the *real* workload rather than a build-only figure, and what AC-004 proves is still running ([`../_storymap.md`](../_storymap.md)`:47`, `:136-138`).

Transitively, through that story: `fill-the-bodies-and-ps-34-disposition` → `real-lbug-driver-swap` (which is what puts `lbug` in the graph at all) → `adr-0025-three-answers` → `preflight-and-unlike-axes`.

**Slice-mate, unordered**

- **`package-completeness-and-name-claim`** (HS-S0080) — same `packaging-and-ci-shape` slice, mounted as one surface (*what the workspace pays*), with no ordering constraint between them ([`../_storymap.md`](../_storymap.md)`:80-82`, `:136-138`).

**Unlocks**

- **`freeze-verdict-document`** (HS-S0082) — lists this story among its dependencies; the verdict is assembled from facts this story produces ([`../_storymap.md`](../_storymap.md)`:49`).
- **`verdict-ordering-and-publication-handoff`** (HS-S0083) — hands HS-P0016 the build number, the docs.rs failure and the `[package.metadata.docs.rs]` question, all three of which this story writes to a citable path (AC-006).

## Anchors (progressive disclosure)

Open these when the bound AC is the one being worked. Do not preload the set — the context pack above already carries every decision needed to start.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| [`.bklg/from-contract-to-published-library/ladybug-projection-store/cold-build-cost-and-ci-shape/discover.md`](discover.md) | The signal ledger and the four named wrong implementations, including the three green-looking spellings of a dedicated job that never runs (`:122-133`) and the unconfigured-figure failure (`:135-140`). | Before writing the threshold (AC-001) and again before reviewing the workflow shape (AC-004). | AC-001, AC-004 |
| [`.bklg/from-contract-to-published-library/ladybug-projection-store/_decomposition.md`](../_decomposition.md) | §7 *Build cost is a workspace-wide fact* (`:375-406`) is the measurement protocol and the two-item lever menu, verbatim; the **AC-009** row (`:479`) is the tier classification; *Merge-gate commands* (`:517-539`) is why `--fast` is not the bar. | Before designing the harness, and before choosing the lever. | AC-002, AC-003 |
| [`xtask/src/main.rs`](../../../../xtask/src/main.rs) | The mount point. `REQUIRED` (`:105`) and `OPTIONAL` (`:535`) are where the lever is implemented; `:116-129` and `:143-151` are the `--all-features` steps that pay the cost; `:546-556` is the powerset; `:816-826` is the panic that punishes a rename; `:853-860` is `run_fast`. | Immediately before editing the step tables. | AC-003 |
| [`xtask/src/proof.rs`](../../../../xtask/src/proof.rs) | `ARTEFACTS` (`:132-149`) is the mechanism that makes an emptied or excluded conformance target loud instead of `running 0 tests`-green. Read only — the Ladybug row is HS-S0078's. | When arguing that the chosen shape still runs the conformance target. | AC-004 |
| [`.github/workflows/ci.yml`](../../../../.github/workflows/ci.yml) | The co-mount. `:31-40` is the three-OS `gate` job and the runner images the measurement must use; the file is where "required, unfiltered, fails the merge" becomes true or false. | While taking the per-runner figures, and while wiring any dedicated job. | AC-002, AC-003, AC-004 |
| [`experiments/position-visibility/README.md`](../../../../experiments/position-visibility/README.md) | The layout to copy for a re-runnable measurement in this repository — README, runner, `results/`. There is no other precedent, because nothing else here builds C++. | Before creating `experiments/ladybug-build-cost/`. | AC-002 |
| [`references/evaluation/README.md`](../../../../references/evaluation/README.md) | `:8-13` states the lifecycle the decision-grade document must obey: immutable, dated, pinned to a commit, superseded rather than edited. It also distinguishes that genre from the four mutable speculation documents, so the new file lands in the right one. | Before writing the dated evaluation document. | AC-005 |
| [`RUNBOOK.md`](../../../../RUNBOOK.md) | Phase 11 at `:4393-4444`: the work item at `:4420-4425` states the conditional this story supplies the antecedent for, and the exit criterion at `:4437` is the box being ticked. | When recording the decision, last of the three commits. | AC-006 |
| [`crates/happenstance-ladybug/src/lib.rs`](../../../../crates/happenstance-ladybug/src/lib.rs) | `:31-32` records that docs.rs itself fails to build `lbug` 0.19.1 — the fact the publication handoff turns on. Read only; this story edits nothing under `crates/**`. | When writing the handoff paragraph. | AC-006 |
| [`crates/happenstance-core/Cargo.toml`](../../../../crates/happenstance-core/Cargo.toml) | `:54-56` is the `[package.metadata.docs.rs]` shape the open question is modelled on, so the question is posed concretely rather than abstractly. | Alongside the previous anchor. | AC-006 |
| [`.bklg/from-contract-to-published-library/ladybug-projection-store/project.md`](../project.md) | AC-009's exact text (`:214-217`) and the symmetry obligation on the rationale (`:284-287`) — the criterion a reviewer will read the evaluation document against. | Before writing the rationale. | AC-005, AC-006 |
| [`.redkiln/config.yaml`](../../../../.redkiln/config.yaml) | `verify.affected_gate` (`:40`), `verify.e2e` / `integration_scoped` (`:55`, `:60`), `require_ledger` (`:67`) and `require_commit_provenance` (`:73`) — the commands that will actually run against this story and the mechanism that records the three commits. | Before the first commit, so the ordering evidence is recorded rather than reconstructed. | AC-001 |
| [`.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md`](../../_discovery/distillation/personas-and-journeys.md) | The adapter author's *learn when you are finished* loop and the evaluator's *decide in one sitting* — the two goals every criterion above is framed from, and the only adjudicated audience this initiative has (`.kb/product/` is still empty). | If an acceptance criterion needs re-grounding in whose problem it solves. | AC-003, AC-005 |
| [`.bklg/from-contract-to-published-library/ladybug-projection-store/_design.md`](../_design.md) | The signed-off `N/A — no user-facing surface` determination that makes the composition family of interaction quality inapplicable here — and makes that inapplicability a decision rather than an omission. | If tempted to add a rendered surface, a CLI flag or a public item. | AC-003 |

## Clarifications resolved during spec

1. **The threshold is stated before the number — `discover.md:41-48`'s recommendation is accepted, not declined.** `discover.md` flagged it as a proposal that `spec` could refuse with reasons. It is accepted and promoted to **AC-001**, because AC-009's text is otherwise satisfied word-for-word by a shape chosen first and a figure written under it afterwards. The reason it is *procedural* rather than a fixed number is that no artefact in this project fixes what "material" means, and inventing a workspace-wide wall-clock constant in a spec would be less honest than requiring the implementer to state one, in public, before measuring.

2. **Where the evidence lives — both, and they are not interchangeable** (`discover.md:69-72` deferred this). The re-runnable harness and raw timings go to `experiments/ladybug-build-cost/`, which `CLAUDE.md` defines as reproducible measurement outside the gate; the dated decision-grade record goes to `references/evaluation/`, on that directory's immutable-evidence lifecycle. Putting the harness in `references/evaluation/` would make a re-run an edit to immutable evidence; putting the decision only in `experiments/` would leave `RUNBOOK.md` and the freeze verdict citing something explicitly designed to change.

3. **If the split is chosen, the dedicated job runs on all three runners by default** (`discover.md:69-72`'s second deferred question). One runner would hide `windows-latest`, which this workspace declares first-class by explicit comment (`.github/workflows/ci.yml:34-35`) and which is the runner most likely to be worst for a `cmake` build. A narrower matrix is permitted only with the reason recorded in the evaluation document under AC-005's rationale obligation.

4. **The gate delta is added to the required measurement set, above the brief's protocol.** [`../_decomposition.md`](../_decomposition.md)`:394-399` requires a per-crate cold build plus a warm figure. That is necessary and, on its own, insufficient: the lever operates on *gate steps*, which build the crate under more than one profile and feature set, so only a `cargo xtask ci` wall-clock delta says what `--exclude` actually buys. Both are required by AC-002/AC-003; the per-crate cold build stays the headline figure AC-009 asks to see recorded.

5. **A feature flag is explicitly not a lever, and the refutation is written into the spec rather than left implicit.** It is the remedy that feels obviously right, it saves nothing because every gate step turns features on, and the one thing it does buy is a configuration where `cargo test` exits 0 on `running 0 tests` against a crate that was never built. Keeping the refutation on the page is what stops it being re-proposed in review.

6. **"The job exists" is not the acceptance condition for the split lever; "required, unfiltered, fails the merge" is** (AC-004). Branch protection is not representable in this tree, so EC-006 makes the check a dated human attestation inside the evaluation document rather than an unverifiable claim.

7. **The AC set is exactly the six the front half enumerated — AC-001 … AC-006 — with none added or dropped.** The `_ledger.md` authored beside this spec carries one row per id, all `satisfied: false`.

8. **docs.rs is posed here and decided elsewhere.** This story writes the number, the known `lbug` 0.19.1 docs.rs failure and the `[package.metadata.docs.rs]` question to a citable path; the decision belongs to `publication-and-positioning` (HS-P0016), whose own proof artefact is docs.rs green under `--all-features`. Adding the metadata key here would be deciding another project's question with this project's evidence.
