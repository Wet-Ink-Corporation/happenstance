---
item: HS-S0097
stage: spec
created: 2026-08-12T13:47:35.384Z
updated: 2026-08-12T13:47:35.384Z
template_sig: 87bbf1d0
rendered_sig: 7001c068
---

# Spec — A stranger installs it from the registry and it works

## Scope lock

| | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` — **DoD 9**, tagged `@smoke`: *"From a scratch project outside this workspace, adding the published crate from the registry and running the smallest write-then-read cycle succeeds against the published version, not a path dependency"* (`:383-386`) |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` — the DAG (`:150`) placing this project at rank 3 |
| Project | `.bklg/from-contract-to-published-library/publication-and-positioning/project.md` — **AC-008** (`:254-257`), **DR-8** (`:188-190`), Definition of done items **2** and **3** |
| This spec | `.bklg/from-contract-to-published-library/publication-and-positioning/stranger-install-smoke/spec.md` |
| Key briefs | `.bklg/from-contract-to-published-library/publication-and-positioning/_decomposition.md` — **testing brief**: the AC-008 row of the test-mix table (the only `e2e` instrument in the project, with its named wrong implementation), *Intent* (`:433-441`) and *Fixtures and seams to mock* (`:520-523`, *"the one seam that must not be mocked"*); **UX brief**: **U7** (`:88`), **IQ-5**'s U7 corollary (`:289-295`), **AC-UX-012** (`:371-376`). No `architecture` brief exists for this project, deliberately (`project.md`:72-76) |
| Signed-off design | `.bklg/from-contract-to-published-library/publication-and-positioning/_design.md` — **binding**. `## The doctest` (*"the same text three times over… the program `stranger-install-smoke` runs against the registry version"*), the `## Shape decision` row for the quick-start snippet, and **AP-12** (a fence that *"differs by one character from the text the stranger-install smoke ran"*) |
| Story discovery | `.bklg/from-contract-to-published-library/publication-and-positioning/stranger-install-smoke/discover.md` — the signal ledger, the two deferred questions this spec answers, and the named wrong implementation (`:40`) |
| Grounding | `.bklg/from-contract-to-published-library/publication-and-positioning/_grounding.md` |
| Story map / merge order | `.bklg/from-contract-to-published-library/publication-and-positioning/_storymap.md`:69 — milestone `the-release-event`, position **4.3**, *"the proof artefact, and the only step that can only run after"* (`:173`) |
| Immediate predecessor | `.bklg/from-contract-to-published-library/publication-and-positioning/publish-0-2-0/spec.md` — its `_release-log.md` convention, its post-publish resolution check (deliberately narrower than this one), and its recorded rollback route |

## One-line PR slice

The proof artefact: from a scratch project outside this workspace, `cargo add` the published crate
from the registry — no path dependency, no workspace feature unification — run the README's own
quick-start write-then-read cycle against the resolved version, and record the run.

## Executive summary

**What lands.** No library code and no published copy. One dated, verbatim-reproducible record —
`.bklg/.../stranger-install-smoke/_smoke-log.md` — of a run that happens *outside* this repository:
a `cargo new` scratch project with no ancestor `[workspace]` above it, one `cargo add happenstance`
that resolves from the index, the packaged README's own quick-start text compiled and **executed** to
completion, and a `wasm32` type-check of the same installed crate. The record carries the manifest
verbatim, the resolved `Cargo.lock` source lines, the toolchain versions, the whole resolved graph,
every command and its transcript, and the date. It follows the `_release-log.md` shape
`publish-0-2-0` established for evidence no gate step can reach.

**The delta against what the tree already does.** Every existing instrument in this repository reads
*the workspace*. `cargo xtask ci` is hermetic and passes `--locked` at every step
(`xtask/src/main.rs`:52-55); `package-check` proves files are *inside* a `.crate`
(`xtask/src/package.rs`:88-94); the doctests compile the README's fence from the source tree;
`publish-0-2-0`'s post-publish check asks only *does the index serve all three at this number*. Four
things nothing in the tree can do, and they are what this PR is for.

1. **Nothing has ever built these crates from a registry tarball, outside a workspace.** A path
   dependency never fetches a tarball at all, and Cargo's feature unification inside one workspace
   resolve means a workspace-local check can pass under a feature combination no `cargo add` user
   would ever get (`discover.md`:27, `:40`).
2. **Nothing has ever *run* the quick start.** Today's fence at `crates/happenstance/README.md`:30-39
   declares `async fn count_everything()` and never awaits it; the module doctest at
   `crates/happenstance/src/lib.rs`:58-67 does the same behind hidden lines. Both *compile*. AC-008
   asks for a cycle that *succeeds*, which is a different claim and this story is the only thing that
   makes it.
3. **Nothing checks the promise from the outside.** IQ-5 forbids *"run our CI"* as a claim's only
   evidence; the `wasm32` line in the Guarantees slot (`crates/happenstance/README.md`:41-49) and the
   MSRV line at `:46-49` are both claims a stranger cannot currently check without cloning.
4. **This is the only check in the project whose failure cannot block anything.** It runs after the
   irreversible act, so its output is a routed finding, never a fix applied in place.

**What this PR is not.** It edits no README, no rustdoc, no manifest, no clause and no gate step. It
adds no `xtask` subcommand — the gate must stay offline and hermetic, and the testing brief already
records that this seam *"should be recorded as such rather than folded into `cargo xtask ci`"*.

## Context pack

The load-bearing decisions, stated as decisions. Deeper material sits behind the signposted anchors.

**The act has already happened, so failure routes forward and never backward.** This is the only
story in the project that runs after `cargo publish`, and the deployment brief is explicit that every
other instrument exists to move the cost of being wrong to *before* the act. The mechanical
consequence: a failing smoke is **recorded as a finding, in the record, at the time it failed** — it
is never made green by editing a published surface (impossible), by re-running until it passes, by
adding a `path` dependency, a `[patch]` or a vendored source (which would falsify the very thing
under test), or by deleting the run. A defect ships as a new forward-dated `0.(2+n).0`. A `yank` is
reserved for the one case the deployment brief names: **the published artefact itself is broken** —
it does not build, or a required file is missing — which is precisely a failure mode this smoke is
capable of detecting and nothing else is. That is the single decision that shapes every criterion
below.

**"Outside this workspace" is a mechanical property, not an intention.** Four independent things are
each individually sufficient to falsify the run, and each is checked and recorded rather than assumed:
(a) no ancestor directory of the scratch project holds a `Cargo.toml` with a `[workspace]` table —
which is why the project cannot live anywhere under this repository, including under a `.gitignore`d
directory; (b) the scratch `Cargo.toml`'s dependency line carries **no `path` and no `git` key**;
(c) the generated `Cargo.lock` records `source = "registry+https://github.com/rust-lang/crates.io-index"`
for every `happenstance*` package; (d) the environment applies no source replacement, `[patch]`,
`[replace]` or vendored directory — `$CARGO_HOME/config.toml` and every `.cargo/config.toml` on the
path from the scratch directory to the filesystem root are read and their relevant contents recorded.
The repository's own `.cargo/config.toml` declares only the `xtask` alias and is out of reach from
outside the tree, but *recording that it was checked* is what makes the claim falsifiable rather than
plausible. This is the discover stage's named wrong implementation (`discover.md`:40) restated as
four checks.

**The text is the same text, and what the fence does not show is named rather than folded in.**
AC-UX-012 makes the packaged README's quick-start fence, `happenstance`'s own doctest and this
smoke's program one text; AP-12 fails a fence that *"differs by one character from the text the
stranger-install smoke ran"*. So the program is not authored here: it is **extracted from the
packaged tarball's `README.md`**, not from the working tree and not from the design document, and the
identity is recorded as a hash of both sides. But a copier does not get the doctest's hidden lines,
and the fence is an `async fn` nobody awaits — so the scratch project must add an executor and a
`main` the page never shows. Those additions are **scaffolding around the fence, listed explicitly in
the record**, and the gap between *"a reader who copies what they see gets what the smoke proved"*
and *"a reader who copies what they see gets a program that does not run"* is a **finding this story
reports** against AC-UX-012, routed to `guarantees-and-docs-rs-presentation`'s owner and a forward
version. It is not repaired here: copy is frozen once `rendered-page-preflight` has read it and
`publish-0-2-0` has shipped it, and a surface edited after its preflight read is a surface nobody
read.

**The feature set under test is the one a `cargo add` reader gets, and only that.** `happenstance`'s
default is `["std", "memory"]`, forwarded wholesale to `happenstance-core` so the two cannot disagree
about what `default-features = false` means (`crates/happenstance/Cargo.toml`:14-26). The scratch
project takes the default — no `features`, no `default-features = false` — because the unification
trap this story exists to avoid is *extra* features leaking in, and hand-picking a set would be the
same mistake with better manners. The `wasm32` arm type-checks that same installed crate at that same
set, which is the first time the claim P3 self-identifies from (**U5**, *"tell me whether it runs
where I run"*) is checked against the artefact a consumer receives rather than against the workspace
tree. `publish-0-2-0` adds a fifth mandatory `wasm32` gate step over the *workspace*; this is the
same question asked of the *tarball*, from outside, and the two are not the same evidence.

**The floor is a property of the resolved graph, so only a registry resolve can check it.** ADR-0029
raised the MSRV to 1.97.1 because of *a dependency's build script*, not this workspace's code, and
records that five of the five database crates here declare no `rust-version` at all — so neither
`cargo hack --rust-version` nor `resolver = "3"` can protect a floor against them, and only running
the compiler finds it (`CLAUDE.md`, binding constraint 5). A stranger's `cargo add` produces a graph
this repository's `Cargo.lock` never contained. The scratch run therefore happens on **1.97.1** — the
number the Guarantees slot promises at `crates/happenstance/README.md`:46-49, and the channel
`rust-toolchain.toml` already pins — and `rustc --version` is recorded, so the claim under test is the
published one and not "whatever stable was installed that day". A failure here is a finding about the
promise, routed to `msrv-promise-atom`'s atom and a forward version; it is never a silent bump.

**This can never become a gate step, and saying so is part of the deliverable.** `cargo xtask ci`
must stay runnable offline and hermetic; every one of its cargo invocations passes `--locked`
precisely so no step resolves something nobody committed. A step that reaches crates.io would make
the gate fail on a train, and the testing brief already ruled it out by name. The evidence is
therefore a committed dated transcript, exactly as project DoD 3 asks and as AC-016's own carve-out
established (*"the proof artefact is the committed gate output, not a passing CI badge"*).

**Who ran it is recorded honestly, and reproducibility is what carries the claim.** Project DoD 2
wants the smoke *"run against it by someone who did not build it"*. The instrument that survives
contact with reality is not an assertion about the runner — it is a recipe a third party can execute
verbatim from `_smoke-log.md` alone, on a machine with no checkout of this repository, and get the
same result. The record states plainly who ran it and in what environment. A record that implies a
stranger ran it when the maintainer did is the same class of unfalsifiable claim this whole project
exists to remove.

**The persona slice.** Backbone activity **A4** — *"take the one irreversible act, and check it from
outside"* (`_storymap.md`:43) — and the only user intent in the milestone: **U7**, *"Let me try it
without cloning the repository."* Persona 4 is the evaluator: one-shot, time-boxed, unable to run the
project's own suite and unwilling to take the maintainer's word
(`_discovery/distillation/personas-and-journeys.md`:258-270). Every other check in this project is
one the evaluator cannot perform. This is the one that stands in for them, which is why it is the
proof artefact rather than a formality.

## Integration contract

| | |
| --- | --- |
| **Archetype** | `capability` — a person who did not build the library installs it the way a stranger would, and the smallest useful thing works. |
| **Slice / milestone** | `the-release-event`. Slice-mates, implemented in one context: `rendered-page-preflight` (4.1, buys reversibility before the act) and `publish-0-2-0` (4.2, the act). This story is **4.3**, strictly last, and is the only step that can only run *after* (`_storymap.md`:170-184). |
| **Mount point** | **`.bklg/from-contract-to-published-library/publication-and-positioning/stranger-install-smoke/_smoke-log.md`** — the dated, committed proof artefact, and the only place this capability can mount, because the thing it exercises runs outside every tree this repository owns. It follows `publish-0-2-0`'s `_release-log.md` convention for evidence no gate step can reach (project DoD 3). **The out-of-tree composition root is the scratch project's own `Cargo.toml`** — the one file that decides whether the subject is the registry crate or a path dependency — and it is reproduced *verbatim* into the mount point precisely because it cannot be committed where it runs. A record that paraphrases that manifest has not mounted anything. |
| **Wires into** | The packaged `happenstance` `.crate` from crates.io at the published version — its `README.md` quick-start fence (`crates/happenstance/README.md`:30-39 is the source of record), its Guarantees slot (`:41-49`) and its default feature set (`crates/happenstance/Cargo.toml`:14-26, `default = ["std", "memory"]`); the public items the fence names, re-exported through `crates/happenstance/src/lib.rs`:75 (`pub use happenstance_core::*`) — `EventStore`, `MemoryEventStore`, `Query`, `ReadOptions`, `collect` (`crates/happenstance-core/src/store.rs`:285), and whatever append-side items the published fence carries (`crates/happenstance-core/src/event.rs`:351, `:372`; `crates/happenstance-core/src/tag.rs`:304); `publish-0-2-0`'s `_release-log.md` for the published number, the publish transcripts and the recorded rollback route; the crates.io sparse index and the `rust-toolchain.toml` floor (`:2`). |
| **Renders surfaces** | **None authored; one *consumed*.** `_design.md`'s `crates-io-happenstance` surface — `selector: crates/happenstance/README.md` — is the source of record for the text this story runs, and this story changes **no byte** of it. It is the only story in the project that exercises a designed surface without owning it, and the one anti-pattern it can trip is **AP-12**, which it detects rather than commits. |
| **Public items** | **None added or changed.** `_design.md`'s `## Items` block carries two entries and this story implements neither: the conditional `unstable-projection` feature is `projection-port-ship-shape`'s, the `[package.metadata.docs.rs]` block is `guarantees-and-docs-rs-presentation`'s. This story *observes* whichever of them shipped, from the outside. |
| **Conformance rule(s)** | **None, and it is not adapter-observable.** No rule in `crates/happenstance-testkit/src/suite.rs` can observe a registry resolution, and none should be added to try — a rule that reaches the network breaks the suite for every adapter author offline. Stated explicitly because a story that names no rule must say which of the two it is. The equivalent obligation — *an instrument that can fail* — is discharged by four independently falsifiable properties of the run itself (registry provenance, text identity, actual execution, environment cleanliness), each with a named wrong implementation in the behavior table. |
| **Clause(s)** | **Amends none, reads none.** `spec/SPECIFICATION.md` is untouched, and `spec-trace` sees nothing new. The one specification-adjacent thing the smoke can observe is the positions-and-gaps promise a copier meets in the fence's comment, which `compliance-claim-and-gaps-promise` owns; this story reports what the published text says, it does not adjudicate it. |
| **Advances DoD scenario** | Initiative **DoD 9**, tagged `@smoke` — *"a stranger can install it"* (`initiative.md`:383-386) — which nothing before this story could even attempt. It also closes project **Definition of done item 2** (*"`0.2.0` resolves from crates.io and the stranger-install smoke has been run against it by someone who did not build it"*) and contributes this project's third dated committed artefact under **DoD item 3**. |

## PR boundary

The paths this story may touch. `redkiln verify --grain story` reads the first fenced block under
this heading and fails on any file changed outside it. The block is one line, and that narrowness is
the point rather than an oversight: everything this story observes is either already published and
uneditable, or lives outside every tree this repository owns.

```
.bklg/from-contract-to-published-library/publication-and-positioning/stranger-install-smoke/**
```

**In this PR**

- `.bklg/.../stranger-install-smoke/_smoke-log.md` — the dated record, and the whole deliverable: the
  environment attestation, the scratch project's `Cargo.toml` and `Cargo.lock` source lines verbatim,
  the extracted fence and both hashes, the scaffolding named, every command with its transcript, the
  full resolved dependency graph, `rustc`/`cargo` versions, the `wasm32` arm, the observed output, and
  the findings — including a finding of *"none"*, stated, rather than an absent section.
- `.bklg/.../stranger-install-smoke/` — nothing else. No script, no vendored copy of the example, no
  second `main.rs` maintained in this tree: a maintained second copy of the fence is exactly the
  drifting text `_design.md`'s `## Shape decision` rejected and **AP-12** fails on. The record carries
  the program as a *dated transcript* — a snapshot, not a mirror — and nothing reads it back.

**Explicitly not in this PR**

- **Any published copy.** No README, rustdoc, `description` or Guarantees edit. `landing-copy-and-status-truth`,
  `compliance-claim-and-gaps-promise` and `guarantees-and-docs-rs-presentation` own it; `rendered-page-preflight`
  has already read the rendered result and `publish-0-2-0` has shipped it. Everything this story finds
  wrong on a published surface is a **finding routed to a forward version**, never an edit.
- **Any `xtask` subcommand, gate step or CI job.** The gate stays offline and hermetic
  (`xtask/src/main.rs`:52-55; testing brief, *Fixtures and seams to mock*). A "check the smoke record
  exists" lint would be a rule nothing meaningful can fail, which `CLAUDE.md`'s decorative-gate
  corollary forbids.
- **Any conformance rule, mutant or testkit change.** Not adapter-observable, and a network-reaching
  rule would break the suite for every offline adapter author.
- **`RUNBOOK.md`'s phase-12 row and exit checkboxes**, and any `.kb/` atom. The runbook row is the
  project's integration/closeout business — `publish-0-2-0`'s boundary excludes it for the same
  reason — and every atom this project authors goes through `.kb/_intake/` and `/redkiln:kb-ingest`
  (project DR-13), never from a story like this one.
- **Yanking, re-publishing or bumping anything.** A finding is recorded here and acted on elsewhere,
  by a decision, not by a story whose job is to observe.
- **A second stranger environment, a matrix of toolchains, or a CI-hosted repeat.** One clean run at
  the promised floor, recorded so a third party can repeat it, is the artefact. A matrix would be
  breadth bought before the first data point exists.

**Merge DoD.** `_smoke-log.md` is committed and dated; it shows a `Cargo.lock` sourcing every
`happenstance*` package from `registry+https://github.com/rust-lang/crates.io-index` at the published
number, a manifest with no `path`/`git` key and no ancestor `[workspace]`, byte-identical hashes for
the packaged fence and the program that ran, a **non-zero-work run that exited 0** with its observed
output, a `wasm32` check of the same installed crate, `rustc 1.97.1`, and a findings section that is
present whether or not it is empty.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| The subject is the registry artefact, proven four ways | The scratch project is created by `cargo new` in a directory with **no ancestor `Cargo.toml` carrying a `[workspace]` table**; the dependency is added by `cargo add happenstance` and the resulting line carries **no `path` and no `git` key**; the generated `Cargo.lock` records `source = "registry+https://github.com/rust-lang/crates.io-index"` for every `happenstance*` package at the published number; and the transitive graph is recorded in full. Named wrong implementation: a workspace member or `examples/` crate depending by `path`, which never fetches a tarball and would pass even if `0.2.0` had never been published. | `project.md` AC-008 / DR-8; `discover.md`:27, `:40`; `_decomposition.md`, testing brief AC-008 row |
| The environment is stranger-shaped, and the attestation is part of the record | Before the run: `$CARGO_HOME/config.toml` and every `.cargo/config.toml` from the scratch directory to the filesystem root are read for `[source]` replacement, `[patch]`, `[replace]` or a vendored directory, and the finding is recorded — *"none"* stated, not implied. Named wrong implementation: a passing run on a machine whose ambient config silently replaced crates.io with a local mirror, which is a path dependency wearing a registry URL. | `.cargo/config.toml` (this repo's, which declares only the `xtask` alias — out of reach from outside, and recorded as checked); `project.md` DoD item 3 |
| The program is the packaged fence, extracted rather than authored | The quick-start fence is extracted from the **`.crate` tarball's** `README.md` — the artefact a consumer receives — not from the working tree and not from `_design.md`. A hash of the extracted fence and a hash of the program body that ran are both recorded and equal. Named wrong implementation: a hand-copied example that has drifted by one character, which is **AP-12** exactly. | `_design.md`, `## The doctest` and `## Shape decision` (quick-start row), **AP-12**; `_decomposition.md`, UX brief **AC-UX-012** (`:371-376`); `crates/happenstance/README.md`:30-39 |
| The scaffolding the page does not show is named, and its absence from the page is a finding | The fence is an `async fn` nobody awaits (`crates/happenstance/README.md`:32; the module doctest hides the same thing at `crates/happenstance/src/lib.rs`:61-66). Running it needs a `main` and an executor the published page never names, and one extra registry dependency the reader is not told to add. The record lists **exactly** what was added and why; the discrepancy with AC-UX-012's *"a reader who copies what they see gets what the smoke proved"* is recorded as a finding routed to `guarantees-and-docs-rs-presentation` and a forward version. The executor must itself be an ordinary registry crate — never a workspace member, never a `path`. | `_decomposition.md`, UX brief **AC-UX-012**, **IQ-5** (`:289-295`); `standards/rust/62-doctests-and-harnesses.md` RS-62-5 (why a packaged example may not reach outside its package) |
| It **runs**, and running is a different claim from compiling | The write-then-read cycle executes to completion and the process exits `0`: at least one event is appended, the read returns it, and the assertion in the published fence holds. Observed stdout/stderr and the exit status are recorded. Named wrong implementation: a smoke that only `cargo build`s or `cargo check`s the example — which is what every existing instrument in this repository already does, and what today's never-awaited fence would let pass. | `project.md` AC-008 (*"running the smallest write-then-read cycle **succeeds**"*); `initiative.md`:383-386 |
| The feature set is the default a `cargo add` reader gets | No `features` key and no `default-features = false` in the scratch manifest. `happenstance`'s `default = ["std", "memory"]` forwards wholesale to `happenstance-core`, so the two cannot disagree about `default-features = false`. The resolved feature set is recorded from `cargo tree -e features` (or equivalent) so a later reader can see there was no unification. | `crates/happenstance/Cargo.toml`:14-26; `_decomposition.md`, UX brief (facade forwards every feature) |
| The `wasm32` claim is checked against the tarball, from outside | `rustup target add wasm32-unknown-unknown`, then `cargo check --target wasm32-unknown-unknown` in the same scratch project over a target that exercises `happenstance` at its default features. If the chosen executor does not type-check on that target, the arm swaps to a `--lib` target that omits it and **the substitution is recorded** — the claim under test is about `happenstance`, not about the executor. This is a `check`, never a `run`; nothing here pretends to execute on that target. | `crates/happenstance/README.md`:41-49 (the Guarantees slot P3 self-identifies from); `_decomposition.md`, UX brief **U5**; `publish-0-2-0/spec.md` AC-005 (the workspace-side step this complements rather than duplicates); `.kb/decisions/0001-async-port-flavours.md` |
| The toolchain is the promised floor, on a graph only a registry resolve produces | The run happens on **1.97.1** — the number the Guarantees slot promises and `rust-toolchain.toml`:2 pins — and `rustc --version` / `cargo --version` are recorded. The floor is a property of the *resolved graph*: ADR-0029 moved it because of a dependency's build script, and five of five database crates here declare no `rust-version`, so only running the compiler on the graph a stranger actually gets can check it. A failure is a finding routed to `msrv-promise-atom`, never a bump made here. | `.kb/decisions/0029-msrv-raised-to-1-97-1.md`; `CLAUDE.md` binding constraint 5; `crates/happenstance/README.md`:46-49 |
| Failure routes forward; it never edits, re-runs or falsifies | A failing smoke is recorded **as it failed**, with the transcript. It is not made green by editing a published surface, by re-running until it passes, or by adding a `path`, `[patch]` or vendored source. A defect ships as a new forward-dated `0.(2+n).0`. A `yank` is on the table **only** for the deployment brief's stated case — the artefact itself is broken (does not build, a required file is missing) — which is a failure mode this smoke is uniquely able to detect. | `_decomposition.md`, deployment brief *Migration, backfill and rollback posture* and *Release path if a published version changes*; `standards/rust/51-features-and-no-std.md`:224-231 (a yank leaves the page exactly as it was) |
| It is never a gate step, and the reason is recorded | No `xtask` subcommand, no `REQUIRED`/`OPTIONAL` entry, no CI job. `cargo xtask ci` stays hermetic and `--locked` throughout; the testing brief already places this seam outside it. The evidence is a committed dated transcript, the same shape AC-016 and `publish-0-2-0` use for evidence a gate cannot reach. | `xtask/src/main.rs`:52-55; `_decomposition.md`, testing brief *Fixtures and seams to mock* (`:520-523`) and *Merge-gate commands*; `.redkiln/config.yaml`:50-60 |
| The record is reproducible by a third party, and honest about who ran it | `_smoke-log.md` carries every command verbatim in order, the manifest and lock excerpts, both hashes, the full resolved graph, the toolchain versions, the date, the OS/arch, and a plain statement of **who ran it and in what environment** — no implication of a stranger where there was none. Project DoD 2's *"someone who did not build it"* is carried by the recipe's verbatim reproducibility, and any gap between that and who actually ran it is stated rather than papered over. | `project.md` Definition of done items 2 and 3; `publish-0-2-0/spec.md` (the `_release-log.md` ordering-as-evidence pattern) |
| A skip is reported, never silent | If any arm cannot run — the index is unreachable, `rustup` cannot fetch the `wasm32` target, the floor toolchain is unavailable — the record says **which arm, why, and what it would have proven**, and the story does not claim it. An arm quietly omitted is worse than one that failed. | `.kb/decisions/0010-the-suite-must-prove-itself.md`; `_decomposition.md`, deployment brief *CI implication* (fail closed with a stated reason) |
| The scratch project is destroyed, and that is deliberate | Nothing under the scratch directory is copied into this repository as a live file. Its content survives only as transcript inside `_smoke-log.md`. A committed copy would be a second text to keep true, would drift from the published fence, and would immediately trip **AP-12** — the exact failure the design rejected a separate `examples/` file to avoid. | `_design.md`, `## Shape decision` (quick-start row), **AP-12** |

## Data and migrations

**N/A for schema, and the reason is not "nothing happened to be needed".** This story defines no
storage schema, writes no persistent state, and touches no consumer data. The only store it exercises
is `MemoryEventStore`, which lives for the duration of one process and is gone when it exits — that
is *why* it is the right subject for a write-then-read cycle a stranger can run with no database,
no container and no credentials. Consumer-facing migration for this release is recorded as N/A with
its reason by `publish-0-2-0` under AC-DEP-004 (first stable publish; the `0.0.0` reservations are
incompatible with everything, so there is no compatible predecessor — `CONTRIBUTING.md`:296-300,
`xtask/src/reserve.rs`:12-30), and this story does not restate it as its own.

The one lifecycle question this story does own is **which artefacts survive the run**, and it has a
sharp answer, because the wrong one creates a second text that drifts:

| Artefact | Where it lives | Lifecycle |
| --- | --- | --- |
| The scratch Cargo project (`Cargo.toml`, `src/main.rs`, `Cargo.lock`, `target/`) | Outside this repository, in a directory with no ancestor `[workspace]` | **Ephemeral by design.** Created for the run, destroyed after it. It may never be committed here: outside-the-workspace is the property under test, and a committed copy is a copy that stops being outside. |
| The manifest and the lock's `source` lines | Reproduced **verbatim** into `_smoke-log.md` | A dated transcript, not a live file. Verbatim rather than summarised, because *"declares `happenstance = "0.2.0"` with no `path` key"* is the whole claim and a paraphrase does not carry it. |
| The extracted quick-start fence and the program body | Recorded in `_smoke-log.md` with both hashes | A **snapshot, never a mirror**. Nothing reads it back, so it cannot rot into a passing check; and because nothing maintains it, it cannot drift into the second copy AP-12 forbids. |
| `_smoke-log.md` | `.bklg/.../stranger-install-smoke/_smoke-log.md` | **Written once, never rewritten.** If the smoke is re-run against a later version, that is a later story's record — appending a second run to this one would make the file a mirror of the newest state instead of evidence about `0.2.0`. |
| The resolved dependency graph and toolchain versions | Recorded in `_smoke-log.md` | The graph is the thing the MSRV promise is actually about; recording it is what lets a later reader tell whether a floor failure came from this library or from a dependency that moved, which is the distinction ADR-0029 was written to make. |

**Rollback.** This PR is one markdown file in the backlog tree and reverts cleanly and completely.
Nothing it does is irreversible — the irreversible act was `publish-0-2-0`'s, and this story's entire
purpose is to find out, from the outside, what that act actually shipped.

## Acceptance criteria

Eight criteria, each written from what a person is trying to do rather than from a capability the
record happens to have. **P4** is the evaluator — one-shot, time-boxed, cannot run this repository's
suite and will not take the maintainer's word (`_discovery/distillation/personas-and-journeys.md`:249-270);
**P1** the application author, **P2** the adapter author, **P3** the local-first / edge developer
(`:42`, `:114`, `:182`). Every row names the wrong implementation it rejects, because a criterion
nothing can fail is decorative (`CLAUDE.md`, the conformance-suite corollary, applied one level up).

| id | criterion | verification |
| --- | --- | --- |
| **AC-001** | **GIVEN** P4 has decided not to clone anything and will type `cargo add happenstance` into a project of their own, and a `path` dependency never fetches a tarball at all — so a workspace-local smoke passes even if `0.2.0` was never published — **WHEN** the maintainer runs the smoke after the act, **THEN** the subject is proven to be the **registry artefact** four independent ways, each *recorded* rather than assumed: the scratch project is created by `cargo new` in a directory with **no ancestor `Cargo.toml` carrying a `[workspace]` table**; its dependency line carries **no `path` and no `git` key**; the generated `Cargo.lock` records `source = "registry+https://github.com/rust-lang/crates.io-index"` for **every** `happenstance*` package at the published number; and the whole resolved graph is recorded — any one of the four missing makes the run **void**, not passing | Record, static: `_smoke-log.md` § *Provenance* carries the scratch `Cargo.toml` **verbatim** and every `happenstance*` `[[package]]` block from the lock; a `cargo metadata --format-version 1 --manifest-path <scratch>/Cargo.toml` transcript showing the registry source and the resolved version. **Wrong implementation rejected:** the discover stage's named one (`discover.md`:40) — a workspace member or `examples/` crate depending by `path`, which produces no `source =` line at all |
| **AC-002** | **GIVEN** a machine whose ambient cargo configuration replaces crates.io with a local mirror produces a passing run that is *a path dependency wearing a registry URL*, and P4's machine has no such configuration, **WHEN** the maintainer prepares to run, **THEN** before the first `cargo` invocation, `$CARGO_HOME/config.toml` **and every** `.cargo/config.toml` on the path from the scratch directory to the filesystem root are read for `[source]` replacement, `[patch]`, `[replace]` and any vendored directory, and the finding is written into the record as an explicit **none** — stated, never implied by an absent section — with each path listed whether or not the file existed | Record, static: `_smoke-log.md` § *Environment attestation*, one row per config path inspected carrying its relevant contents or `absent`; a note that this repository's own `.cargo/config.toml` (which declares only the `xtask` alias) is unreachable from the scratch directory, recorded as *checked*. **Wrong implementation rejected:** an omitted section read as a pass — precisely the silent skip `.kb/decisions/0010-the-suite-must-prove-itself.md` forbids |
| **AC-003** | **GIVEN** AC-UX-012 promises P4 that *a reader who copies what they see gets what the smoke proved*, and **AP-12** fails a fence that differs by one character from the text this smoke ran, **WHEN** the program is assembled, **THEN** it is **extracted from the downloaded `.crate` tarball's `README.md`** — the artefact a consumer receives, never the working tree and never `_design.md` — and the record carries the extracted fence verbatim, a digest of it, a digest of the program body that actually ran, and the two are **equal**; if they cannot be made equal without editing the fence, the difference is recorded as a **finding** and nothing is edited | Static, in-record: two recorded `sha256` digests plus byte counts, and the tarball's own provenance (its path under `$CARGO_HOME/registry/cache` or the `cargo download` transcript that produced it). **Wrong implementation rejected:** a hand-copied example maintained in this tree — forbidden by the PR boundary and named by **AP-12** (`_design.md`:641-680) |
| **AC-004** | **GIVEN** P4 tries a library by copying **what the page actually shows**, and the signed-off design sites the quick start at **R7, directly under the compliance block R6**, as a `toml` fence *plus* a `rust` fence, *revealed on scroll* on the packaged surface, budgeted at **≤ 20 source lines** (the design's own text is 19), **WHEN** the smoke reads the packaged README out of the tarball, **THEN** it records that region **as shipped** and checks the composition it depends on: a `toml` fence exists and the scratch manifest's dependency line **is the line that fence tells a reader to write**; the `rust` fence **declares its language** and is **≤ 20 source lines**; the region is on the packaged README itself — 0 hops, not one hop behind a link; the compliance block precedes it; the region contains **no raw HTML** (**AP-7**); **AND** everything the fence does not show but the program needed — a `main`, an executor, and the extra registry dependency the page never names — is listed **item by item** as scaffolding, with the gap against AC-UX-012 recorded as a finding routed to `guarantees-and-docs-rs-presentation` and a forward version, never repaired here | Human review against `_design.md` `## Composition` (R6→R7), `## Transience policy` (quick-start fence = *revealed on scroll*), `## Density budget` (fence ≤ 20 source lines) and **AP-12** / **AP-7**, recorded in `_smoke-log.md` § *The region as shipped*: the quick-start region reproduced verbatim with its neighbours, a source-line count, the fence info strings, and the scaffolding list with one line of justification each. **Wrong implementation rejected:** a fence that is byte-identical (AC-003 green) and **unusable** — every digest check passes on a snippet nobody can run, which is why this is a separate instrument |
| **AC-005** | **GIVEN** P4's sitting ends the moment the example does not work, and **every** existing instrument in this repository only ever *compiles* the fence — the tree's own declares `async fn` and never awaits it (`crates/happenstance/README.md`:30-39; the module doctest hides the same at `crates/happenstance/src/lib.rs`:58-67) — **WHEN** the scratch project is built and executed, **THEN** the write-then-read cycle **runs to completion and the process exits `0`**: at least one event appended, the read returning it, the published fence's own assertion holding, with stdout, stderr and the exit status recorded; **AND** it does so at **exactly the feature set a `cargo add` reader gets** — no `features` key, no `default-features = false` — with the resolved set recorded so a later reader can see that no unification occurred | Manual, out-of-tree: the `cargo run` transcript with its exit status in `_smoke-log.md` § *The run*; `cargo tree -e features` for `happenstance` and `happenstance-core` showing `std` + `memory` and nothing beyond (`crates/happenstance/Cargo.toml`:14-26, where the facade forwards its default wholesale so the two cannot disagree). **Wrong implementation rejected:** a smoke that stops at `cargo build` / `cargo check` — which is what the whole tree already does, and what a never-awaited fence lets pass |
| **AC-006** | **GIVEN** P3 reads one line in the Guarantees slot to self-identify as a `wasm32` consumer and cannot check it without cloning (IQ-5), and `publish-0-2-0` proved that claim only over the **workspace** (its AC-005's fifth mandatory step), **WHEN** the same *installed* crate is targeted, **THEN** `rustup target add wasm32-unknown-unknown` followed by `cargo check --target wasm32-unknown-unknown` runs in the same scratch project over a target that exercises `happenstance` at its default features, recorded verbatim; it is a **check, never a run** and nothing pretends to execute there; **AND** if the chosen executor does not type-check on that target the arm swaps to a `--lib` target that omits it and **the substitution and its reason are recorded**, because the claim under test is about `happenstance`, not about the executor | Manual, cross-target: the `cargo check --target wasm32-unknown-unknown` transcript in `_smoke-log.md` § *The wasm32 arm*, plus the manifest excerpt for whatever target shape was checked and, if substituted, the stated reason. Different subject from `publish-0-2-0` AC-005 — tarball versus workspace — so different evidence, not a duplicate. `.kb/decisions/0001-async-port-flavours.md` is why the flavour exists to be checked at all |
| **AC-007** | **GIVEN** the Guarantees slot promises P1 and P2 a **1.97.1** floor (`crates/happenstance/README.md`:46-49), and ADR-0029 records that the floor moved because of *a dependency's build script* while five of five database crates here declare no `rust-version` — so neither `cargo hack --rust-version` nor `resolver = "3"` can protect it and only running the compiler finds it — **WHEN** the smoke runs, **THEN** it runs on **1.97.1**, the channel `rust-toolchain.toml`:2 already pins, with `rustc --version` and `cargo --version` recorded and the **full resolved dependency graph** recorded beside them, because a stranger's `cargo add` produces a graph this repository's `Cargo.lock` never contained; **AND** a failure at that floor is a **finding routed to `msrv-promise-atom`'s atom and a forward version** — never a silent bump, never an edit to the published promise | Toolchain, in-record: `rustc --version` / `cargo --version` transcripts showing 1.97.1, and `cargo tree` for the whole graph, in `_smoke-log.md` § *Toolchain and graph*. **Wrong implementation rejected:** a run on *whatever stable was installed that day*, which tests a different promise than the one published, and reports success about a floor nobody checked. `.kb/decisions/0029-msrv-raised-to-1-97-1.md` is why the graph rather than the workspace is the subject |
| **AC-008** | **GIVEN** project DoD 2 asks for a run *by someone who did not build it* and DoD 3 for a dated committed artefact, and P4 is exactly the person who cannot take an assertion on trust, **WHEN** the run is written up, **THEN** `_smoke-log.md` is a **recipe a third party can execute verbatim from the record alone** — on a machine with no checkout of this repository, no credentials and nothing beyond `cargo` and `rustup`: every command in order, the manifest and lock excerpts, both digests, the graph, the toolchain versions, the OS/arch, the date and the elapsed time since `publish-0-2-0`'s publish transcript, and a plain statement of **who ran it and in what environment**, with any gap against DoD 2 stated rather than papered over; **every arm that could not run is named with why and what it would have proven**; a **findings section is present whether or not it is empty**, with *none* written out; a failing arm is recorded **as it failed** and never made green by editing a published surface, by re-running until it passes, or by adding a `path`, `[patch]`, `[replace]` or vendored source; nothing outside this story's folder changes and no gate step is added; the scratch project is **destroyed**, surviving only as transcript; and the file is **written once** — a later version's smoke is a later story's record | Static, in-record + PR: `redkiln verify --grain story` against the one-line PR-boundary block; a read of `_smoke-log.md` for all eight named sections; `git diff --name-only` showing exactly one added file under this story's folder; `cargo xtask affected --base main` green, and `cargo xtask ci --fast` unchanged in step count — the proof that no gate step was added. **Wrong implementation rejected:** a record that omits a skipped arm, or that implies a stranger ran it when the maintainer did — the unfalsifiable claim this whole project exists to remove |

**Coverage of the traced project AC.** All eight rows serve project **AC-008** (*a stranger can
install it*, `project.md`:254-257) and its derived requirement **DR-8** (`:188-190`): AC-001/AC-002
carry *not a path dependency* and *no workspace feature unification*; AC-003/AC-004 carry *the
README's own quick start*; AC-005 carries *the smallest write-then-read cycle **succeeds***;
AC-006/AC-007 carry the two claims a stranger could otherwise only take on trust; AC-008 carries
*this is the project's proof artefact* — which is a statement about the **record**, not about the
run, since a run nobody can repeat proves nothing to P4. Together they close project **Definition of
done item 2** and contribute this project's third dated artefact under **item 3**, and they are the
only path to initiative **DoD 9** (`initiative.md`:383-386).

## Interaction quality

The blocking invariants, in two families, each named against the **AC rows** that carry it. Nothing
in this section is an obligation of its own: every invariant below is written into the acceptance
table above, because `redkiln verify` extracts ACs from `| AC-### |` table cells and a bullet here
would be gated by nothing.

**This story renders no surface and consumes one.** It is the only story in the project in that
position (*Integration contract*, `Renders surfaces`). So the composition family is read against
`_design.md`'s **`crates-io-happenstance`** as it actually shipped, in the tarball, and every finding
is **reported, never repaired** — the surface is frozen once `rendered-page-preflight` read it and
`publish-0-2-0` shipped it.

### State invariants

| Invariant | Carried by | How it is verified, and why it is not decorative |
| --- | --- | --- |
| **In place, not a context jump** | AC-001, AC-008 | The observation happens **outside every tree this repository owns** and lands as one file inside this story's own folder. Nothing in `crates/`, `xtask/`, `spec/` or `.kb/` moves, and no gate step is added. Verified by `git diff --name-only` (exactly one added path) against the one-line PR boundary. The jump this forbids is real and tempting: "fix it while you are there" on a surface that has already shipped |
| **Non-occlusion** | AC-002, AC-006, AC-008 | Three concrete forms, all structural. An attestation that finds nothing writes **none** rather than omitting the section (AC-002); a `wasm32` arm that had to substitute its target **says so with the reason** (AC-006); an arm that could not run at all is named with *what it would have proven* (AC-008). An empty findings section is written out as empty. `.kb/decisions/0010-the-suite-must-prove-itself.md` — *a skip is reported, never silent* — is the standard, and the failure it rejects is a record that reads green because a section is missing |
| **The subject's state is preserved** | AC-003, AC-004 | The published-surface analogue of preserved focus and selection: the smoke **reads and hashes** the shipped fence and changes **no byte** of it. `publish-0-2-0` AC-006 already established that a surface edited after its preflight read is *a surface nobody read*; this story is downstream of that and inherits the prohibition without weakening it |
| **Reversibility** | AC-008 | This PR is one markdown file and reverts cleanly. The act it observes does not — so reversibility here means the **routing**: a failure ships as a new forward-dated `0.(2+n).0`, a `yank` is reserved for the deployment brief's one case (the artefact itself is broken — which is a mode this smoke is uniquely able to detect), and *never* a re-run until green, an edit to a published surface, or a `path` / `[patch]` / vendored source that falsifies the thing under test |
| **Reachable without privilege** | AC-008 | The keyboard-reachability analogue for this medium, and it is IQ-5 exactly (`_decomposition.md`, UX brief `:289-295`): the evidence must be executable by someone with **no checkout, no credentials and no token** — `cargo` and `rustup` and nothing else. A recipe that needs this repository is *"check out the repo"* wearing a transcript, which is the one thing IQ-5 forbids |

### Composition invariants

Taken from the signed-off `_design.md`, which this story does not re-decide. Each is checked against
the packaged README **inside the `.crate`**, which is the copy a consumer receives and the one no
other story in this project reads.

| Invariant | Carried by | The real number or rule, and its source |
| --- | --- | --- |
| **Presentation exists at all** | AC-004 | The quick start must ship as **real composed markdown** — a `toml` fence *and* a `rust` fence, each declaring its language — not prose telling the reader what to type. A dependency line the reader has to synthesise is a control with no presentation, and the smoke proves the composed form is what it used: the scratch manifest's dependency line **is** the line the `toml` fence shows (`_design.md` `## Composition`, R7) |
| **Composition and placement** | AC-004 | **R6 → R7**: the compliance claim block precedes the quick start, *because R6 earns the try* — the evaluator's order is *is it real → do I try it*, and the design states that reversal as one of three decisions no brief made. Recorded as shipped, region order and all |
| **Transience** | AC-004 | The quick-start fence is **revealed on scroll** — same page, **0 hops**, below the fold — never *opened on demand*. A fence that has moved behind a link puts a hop in front of the only copy-paste path IQ-5 requires, and the smoke is the instrument that would notice (`_design.md` `## Transience policy`) |
| **Density budget, with its numbers** | AC-004 | The `rust` fence is **≤ 20 source lines** (20 × ~21 px mono ≈ 420 px: one screen at 1440×900 with **no inner scrollbar**). The design's own text is **19**. Counted on the packaged copy, not on the tree (`_design.md` `## Density budget`, per-item table) |
| **Hierarchy** | AC-004 | The quick start sits under its own heading in the packaged README's region order, and nothing in the region carries a primary label in anything smaller than the host's body text — no raw HTML, no `<small>`, no image doing the work of a heading (`_design.md` `## Hierarchy`, `## Density budget` → *minimum legible size*) |
| **Named anti-patterns** | AC-003, AC-004 | **AP-12**, both halves and both are this story's: a fence that *differs by one character from the text the stranger-install smoke ran* (AC-003, by digest) and one that *scrolls inside itself at 1440×900* — > 20 source lines (AC-004, by line count). This story is the **only** thing in the project that can detect the first half, because it is the only thing that runs the text. **AP-7** (raw HTML, inline style, JavaScript, meaning-carrying image or animated media) is checked over the region it reads and reported if present |

**Why the composition rows are not redundant with the digest check.** An unstyled, uncomposed page
passes AC-003 perfectly: the fence hashes equal because the smoke ran what the tarball contained,
whatever that was. AC-004 is what fails when the shipped text is byte-identical and **unusable** —
the fence 30 lines long, the `toml` block missing so the reader has to invent the dependency line,
the region behind a link, or the scaffolding gap unnamed. That is the whole reason the composition
read is a separate row rather than a clause in AC-003.

## Error conditions

| id | Condition | Required handling |
| --- | --- | --- |
| **EC-001** | `cargo add happenstance` cannot resolve: the index has not propagated yet, or the network is unavailable | **Not a failure of the library, and not a finding against it.** Wait and retry, recording each attempt with its time. Index lag is the one failure that looks identical to a defect (`publish-0-2-0` EC-002 records the same trap for the publish sequence); the distinguishing evidence is *the same command succeeding minutes later, both attempts recorded* |
| **EC-002** | Resolution succeeds but the **build of the dependency itself** fails — a file missing from the tarball, a module not packaged, a manifest that does not resolve | **This is the deployment brief's yank case and the reason this smoke exists.** The published artefact itself is broken. Record the transcript verbatim, **halt** the story, and route it to a decision — `yank` plus a forward `0.(2+n).0` — never to a repair in this PR. `xtask/src/package.rs`'s `REQUIRED_FILES` check proves containment for the files it names; this is the failure mode outside its reach |
| **EC-003** | The extracted fence does not **compile** as extracted | **AP-12 realised.** Record the compiler output and the fence verbatim as a finding. Do **not** edit the program to make it compile: an edited program falsifies AC-003's identity claim, and a green smoke over a text no reader can compile is worse than a red one. Routes to `guarantees-and-docs-rs-presentation` and a forward version |
| **EC-004** | The program compiles and runs but the published fence's own **assertion fails** or the process exits non-zero | The strongest finding this story can produce: the published quick start is wrong about the published library. Record the observed output and the exit status. Do **not** weaken the assertion, and do not substitute a program that passes — that is the difference between an instrument and a formality |
| **EC-005** | The environment attestation (AC-002) finds a `[source]` replacement, `[patch]`, `[replace]` or vendored directory anywhere on the path | The run is **void, not passing**, whatever its result. Correct the environment and re-run from a fresh `cargo new`, recording **both** attestations and the fact that the first run was discarded. A result obtained over a mirrored source is a path dependency wearing a registry URL |
| **EC-006** | An ancestor `Cargo.toml` with a `[workspace]` table is discovered **after** the run | Same posture: **void and re-run** in a directory that has none, and record that it happened. This is the discover stage's named wrong implementation (`discover.md`:40) caught late rather than avoided, and hiding the near-miss removes the only evidence that the check has teeth |
| **EC-007** | `rustup` cannot add `wasm32-unknown-unknown`, or the target is otherwise unavailable | The arm is **skipped, named, and its loss stated** — which arm, why, and what it would have proven (AC-008). The story does **not** claim AC-006. An arm quietly omitted is worse than one that failed |
| **EC-008** | The **1.97.1** toolchain is unavailable on the machine | The run does not silently proceed on whatever stable is installed. Either install the pinned channel, or record the run as **not testing the published promise**, naming the version that did run. A floor claim checked on a newer compiler is not a floor claim |
| **EC-009** | The graph fails to build at 1.97.1 but builds on a newer compiler | An **MSRV promise finding**, routed to `msrv-promise-atom`'s atom and a forward version. Never a silent bump and never an edit to the Guarantees line. Record which crate in the graph produced the failure, because ADR-0029's whole point is that the floor is a property of the **graph**, not of this workspace's code |
| **EC-010** | A dependency in the resolved graph is yanked, or moves, between `publish-0-2-0`'s transcript and this run | Not an error — record the **lock, the graph and both dates**. The evidence is time-stamped by construction, which is what lets a later reader tell a library defect from an ecosystem movement |
| **EC-011** | A finding is produced and the temptation is to fix it before committing the record | **Forbidden, and named here so it is refused rather than rationalised.** The act has happened; the record is of what shipped. Every repair route runs through a forward version and another story. Committing a record of the *second, fixed* attempt as though it were the first is falsification, not tidying |

## Non-functional

| id | Requirement | Why |
| --- | --- | --- |
| **NF-001** | **No step this story produces runs inside `cargo xtask ci`, `--fast` or full, and none reaches a network from any gate.** The gate stays offline, hermetic and `--locked` at every invocation | `xtask/src/main.rs`:52-55 passes `--locked` precisely so no step resolves something nobody committed; a step reaching crates.io makes the gate fail on a train. The testing brief already ruled it out by name (*Fixtures and seams to mock*), and a *check the record exists* lint would be the decorative gate `CLAUDE.md` forbids |
| **NF-002** | **The run happens within 24 hours of `publish-0-2-0`'s publish transcript**, and the elapsed time is recorded | The subject is a graph resolved at a moment. A smoke run weeks later measures a different graph and cannot distinguish a library defect from an ecosystem movement (EC-010). `publish-0-2-0` sets the same kind of window on its docs.rs observation and for the same reason |
| **NF-003** | **Verbatim reproducibility.** Every command appears in the record in execution order, copy-pasteable, with no elided argument and no `<placeholder>` that only the author can fill | This is what discharges project DoD 2's *someone who did not build it*. An instrument whose recipe cannot be re-executed is an assertion about the runner, which is the class of claim IQ-5 removes |
| **NF-004** | **Written as it happens, never reconstructed.** Sections are appended in run order: attestation → provenance → the region as shipped → the run → the wasm32 arm → toolchain and graph → findings | Every piece of evidence here is unreproducible once the terminal scrolls, and the file's ordering is the only proof the attestation was taken **before** the run rather than back-filled after it. `publish-0-2-0` NF-007 states the same discipline for the release log |
| **NF-005** | **One clean run, one platform.** OS and architecture recorded; no toolchain matrix, no second stranger environment, no CI-hosted repeat | Breadth bought before the first data point exists. One reproducible run at the promised floor is the artefact this project needs; a matrix is a later decision with its own cost |
| **NF-006** | **No credentials of any kind.** The run needs no crates.io token, no authentication and no private index; if any step turns out to require one, **that is itself a finding** | A stranger has none. A smoke that quietly used the maintainer's credentials has tested a path P4 cannot walk |
| **NF-007** | **The record is legible to someone who has never seen this repository**: paths spelled out, abbreviations expanded once, every finding stating what it is a finding *about* and where it routes | The audience for this file is the reader who wants to check the claim rather than re-derive it. Internal shorthand makes the evidence unreachable in exactly the way IQ-5 was written to prevent |

## Implementation notes (non-prescriptive)

Reasoning to reuse, not a script to follow. Where this contradicts what the run actually shows, the
run wins and the record says so.

- **Read `publish-0-2-0`'s `_release-log.md` first, and take the published number from it** — not
  from a manifest, not from memory. It also carries the rollback route, the resolution check and the
  tag, and its section ordering is the model for this file's.
- **Do the attestation before the first `cargo` command, not after the run fails.** An attestation
  taken afterwards is indistinguishable from one taken to explain a result away, and NF-004's
  ordering rule exists to make the difference visible in the file.
- **Choose the scratch directory so the ancestor check is trivially true** — a fresh temporary
  directory well away from any repository. Then still *perform* the check and record it: the point is
  the recorded evidence, not the confidence.
- **Extract the fence from the tarball, not from `crates/happenstance/README.md`.** They should be
  identical; if they are not, that gap is the single most valuable thing this story can report, and
  extracting from the tree destroys the only instrument that could have seen it.
- **Expect the scaffolding gap and write it down rather than smoothing it.** The design's own text is
  an `async fn` nobody awaits (`_design.md` `## The doctest`), so a `main`, an executor and one extra
  registry dependency will be needed. Naming them precisely is what makes AC-UX-012's promise
  checkable at all; hiding them inside a tidy program is what makes it unfalsifiable.
- **Keep the executor an ordinary registry crate.** Never a workspace member, never a `path` — the
  scaffolding must not reintroduce the exact defect AC-001 exists to exclude.
- **Record `cargo tree -e features` before drawing any conclusion about unification.** It is the
  cheapest evidence in the whole run and the only one that speaks directly to DR-8's second clause.
- **If the `wasm32` arm needs a different target shape, say which and why in the same paragraph as
  the transcript.** A substitution recorded next to its result is evidence; a substitution recorded
  in a footnote reads as an excuse.
- **Write the findings section even when it says `none`.** The absent section is the failure mode
  ADR-0010 names, and a reader cannot tell an empty finding from an unasked question.
- **Destroy the scratch project and resist the urge to keep it.** A kept copy becomes a second text
  to maintain, drifts from the published fence, and trips **AP-12** — the exact outcome the design
  rejected a separate example file to avoid.

## Tests and CI (merge gate)

Grounded in the testing brief's AC-008 row (*the only true `e2e` instrument in this project*) and its
*Fixtures and seams to mock* (*the one seam that must not be mocked*). The first four tiers are
**manual and out-of-tree by construction**; the last three are what the merge gate actually runs on a
PR that adds one markdown file.

| tier | command / path | proves |
| --- | --- | --- |
| **e2e, manual, out-of-tree** | `cargo new`, `cargo add happenstance`, `cargo run` in a directory with no ancestor `[workspace]`; transcripts in `.bklg/.../stranger-install-smoke/_smoke-log.md` | **AC-001, AC-005** — the subject resolves from the index and the write-then-read cycle **exits 0**. The project's proof artefact and the one instrument no compiled suite can stand in for (`_decomposition.md`, testing brief *Intent*) |
| **static, in-record** | `_smoke-log.md` § *Environment attestation* and § *Provenance*: the config-path table, the scratch `Cargo.toml` verbatim, every `happenstance*` `[[package]]` block, `cargo metadata --format-version 1` | **AC-001, AC-002** — no `path`/`git` key, `source = "registry+…crates.io-index"` for every package, no source replacement anywhere on the path |
| **static, digest + composition read** | two `sha256` digests over the extracted fence and the program body; a source-line count of the `rust` fence; the region reproduced verbatim against `_design.md` `## Composition` / `## Transience policy` / `## Density budget` | **AC-003, AC-004** — **AP-12** in both halves, plus the composition the copy-paste path depends on. The digest alone is satisfied by an unusable fence, which is why the read is a separate line here |
| **manual, cross-target** | `rustup target add wasm32-unknown-unknown` then `cargo check --target wasm32-unknown-unknown` in the scratch project; `rustc --version`, `cargo --version`, `cargo tree`, `cargo tree -e features` | **AC-005, AC-006, AC-007** — the `wasm32` claim and the MSRV floor checked against the **tarball's** graph rather than the workspace's, which is a different subject from `publish-0-2-0` AC-004/AC-005 |
| **story grain (automatic)** | `cargo xtask affected --base main` — wired at `.redkiln/config.yaml`'s story grain | The PR touches no workspace package, so this proves it: the affected set is empty and the five file-reading lints plus `spec-trace` still pass unconditionally. It is also what would catch an accidental edit outside the PR boundary |
| **integration grain (automatic)** | `cargo xtask ci --fast` — the non-terminal project's bar (`.redkiln/config.yaml`:50-56) | The gate is **unchanged in step count and still offline** (NF-001). A `--fast` run that now needs a network is the signature of the failure this story is forbidden to cause |
| **backlog / KB** | `redkiln validate --kb && redkiln doctor`, and `redkiln verify --grain story` over the ledger | Exactly six `template-drift` advisories and zero `dependency-cycle`; every AC row in `_ledger.md` satisfied with cited evidence before `implement → report` |

**Not a merge gate, stated deliberately.** No `xtask` subcommand, no `REQUIRED`/`OPTIONAL` entry, no
CI job, and no lint asserting `_smoke-log.md` exists. The evidence is a committed dated transcript —
the same shape project AC-016 and `publish-0-2-0` use for evidence a gate cannot reach.

## Risks and coupling (PR-scoped)

| Risk | How it shows up here | Mitigation in this PR |
| --- | --- | --- |
| **The finding cannot be fixed, so the pressure is to not have one** | This is the only check in the project running *after* the irreversible act. Every incentive points at re-running, quietly adjusting the program, or trimming the record until it reads green | EC-011 names the temptation and refuses it; AC-008 requires the failing transcript **as it failed**; the PR boundary makes a published-surface edit a boundary violation `redkiln verify` fails on rather than a judgement call |
| **Coupling to `publish-0-2-0`** — the number, the ancestry, the timing | If the published number is not `0.2.0`, or the publish sequence was resumed midway (its EC-002), this story's manifest and lock must quote **whatever actually resolved**, not what was planned | AC-001 records the resolved version from the lock rather than asserting it; NF-002 bounds the gap between the two records; the mount point follows `_release-log.md`'s conventions so the two read as one sequence |
| **Coupling to `guarantees-and-docs-rs-presentation`** — AC-UX-012's text identity | That story is what makes the README fence, the doctest and this program one text. If it landed a different fence, or landed none, AC-003's digests still hold (the smoke runs what shipped) but the scaffolding gap widens | AC-004 requires the scaffolding be enumerated and the gap **recorded as a finding with a route**, so a weaker identity than AC-UX-012 promised becomes visible evidence rather than a silent adjustment |
| **AP-12 is live and one character wide** | Any edit to the packaged README between preflight and publish breaks byte identity, and this story is where that shows up rather than where it can be fixed | `publish-0-2-0` AC-006 already forbids the post-preflight edit; this story detects the residual case by digest and reports it |
| **Ambient environment on the maintainer's machine** | A proxy, a corporate mirror or a stale vendored directory silently converts the whole exercise into a local build | AC-002's attestation is taken **before** the first command and lists every path checked; EC-005 voids the run rather than annotating it |
| **The honesty gap in *someone who did not build it*** | The maintainer runs it, and a record that implies otherwise is exactly the unfalsifiable claim this project exists to remove | AC-008 requires the runner and environment stated plainly, with the gap named; NF-003 makes the **recipe** carry DoD 2 rather than an assertion about the person |
| **Index propagation timing** | An early run reports a resolution failure that is nobody's defect | EC-001 separates lag from defect by requiring both the failed and the succeeding attempt with times |
| **Scope creep into a permanent instrument** | *While we are here, let us make this a CI job* — which would put a network call inside a hermetic gate | NF-001 and the PR boundary forbid it; the testing brief already recorded the seam as post-publish verification rather than a gate step |

## Dependencies

**Blocks on** — `publish-0-2-0`. Structural rather than conventional: there is nothing on the
registry to `cargo add` until that story has taken the act, so this story **cannot be started early
in a degraded form**. It also consumes three things that story produces: the published version number
(read from `_release-log.md`, never chosen here), the publish transcripts that date the window NF-002
bounds, and the recorded rollback route every finding in this spec routes into.

Transitively, through `publish-0-2-0`: `rendered-page-preflight`, `registry-surface-diff`,
`clause-maturity-audit`, `deferred-clause-reread` and `crate-set-decision`; and through
`guarantees-and-docs-rs-presentation`, the text identity AC-003 and AC-004 read. None of those is
re-checked here — each is re-observed by `publish-0-2-0`'s own five-row precondition table, and this
story reads the result rather than the inputs.

**Unlocks** — no story. `stranger-install-smoke` is **4.3**, the last node in the project's computed
merge order and the terminal story of the `the-release-event` slice (`_storymap.md`:170-184). What it
unlocks is not a story but a **verdict**: project Definition of done item 2, initiative DoD 9
(`@smoke`), and the findings the initiative's terminal project inherits at closeout — which is why
the record's audience is a reader who has never seen this repository (NF-007) rather than the next
implementer.

## Anchors (progressive disclosure)

Deferred depth, signposted and bound. Open each at the moment named — not in advance, and not
optionally.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `.bklg/from-contract-to-published-library/publication-and-positioning/_design.md` | **Binding, signed off.** `## Composition` (R6→R7), `## Transience policy` (quick-start fence = *revealed on scroll*), `## Density budget` (fence ≤ 20 source lines), `## The doctest` (the 19-line text, *the same text three times over*) and **AP-12** / **AP-7**. This story does not re-decide any of it; it reads the shipped surface against it | Before extracting the fence, and again before writing § *The region as shipped* | AC-003, AC-004 |
| `.bklg/from-contract-to-published-library/publication-and-positioning/_decomposition.md` | The **testing brief**'s AC-008 row (the project's only `e2e` instrument, with its named wrong implementation) and *Fixtures and seams to mock* (`:520-523`, *the one seam that must not be mocked*); the **UX brief**'s IQ-5 (`:289-295`) and AC-UX-012 (`:371-376`); the **deployment brief**'s rollback posture and the one case a `yank` is for | Before deciding anything becomes a gate step, and before routing any finding | AC-004, AC-006, AC-008 |
| `.bklg/from-contract-to-published-library/publication-and-positioning/publish-0-2-0/spec.md` | The immediate predecessor: the published number, `_release-log.md`'s section ordering and its *written as it happens* rule (NF-007), its deliberately **narrower** post-publish resolution check (its AC-008), the fifth mandatory `wasm32` step (its AC-005), and the recorded rollback route | Before writing the first line of `_smoke-log.md`, and before improvising any recovery | AC-001, AC-006, AC-008 |
| `.bklg/from-contract-to-published-library/publication-and-positioning/stranger-install-smoke/discover.md` | The named wrong implementation in full (`:40`) — why a `path` dependency never fetches a tarball and how workspace feature unification lets a smoke pass under a feature set no `cargo add` user gets — and the two questions this spec answers | First, before creating the scratch project | AC-001, AC-005 |
| `.bklg/from-contract-to-published-library/publication-and-positioning/project.md` | **AC-008** (`:254-257`) and **DR-8** (`:188-190`) in their own words, plus Definition of done items 2 and 3 — the *someone who did not build it* clause and the dated-committed-artefact pattern | Before writing the acceptance ledger's evidence, and before claiming DoD 2 | AC-008 |
| `.bklg/from-contract-to-published-library/initiative.md` | **DoD 9**, tagged `@smoke` (`:383-386`) — the initiative-level sentence this story is the only path to | When stating what the record discharges | AC-008 |
| `.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md` | **Persona 4**, the evaluator (`:249-270`): one-shot, time-boxed, cannot run the suite, will not take the maintainer's word — and explicitly the least-evidenced persona (`:349-355`), so carry the intents and derive nothing finer | Before writing any finding, to check it is stated in terms the evaluator can act on | AC-004, AC-008 |
| `.kb/decisions/0029-msrv-raised-to-1-97-1.md` | Why the floor is a property of the **resolved graph** rather than of this workspace's code, and why only running the compiler finds it — the reasoning an MSRV finding must be written in | Before choosing the toolchain, and before writing any MSRV finding | AC-007 |
| `.kb/decisions/0010-the-suite-must-prove-itself.md` | *A skip is reported, never silent* — the discipline the whole non-occlusion family inherits, and the standard an omitted section fails | Before recording any arm that could not run, and before omitting any section | AC-002, AC-006, AC-008 |
| `.kb/decisions/0001-async-port-flavours.md` | Why a `wasm32` target exists to be checked at all: ports are defined without a `Send` bound and `trait_variant` derives the `Send` flavour, so a `wasm32` failure is a statement about the flavour design, not a build accident | Before the `wasm32` arm, and before interpreting its failure | AC-006 |
| `crates/happenstance/README.md` | The **source of record** for the shipped quick start (`:30-39`) and the Guarantees slot (`:41-49`, the MSRV promise at `:46-49`). The subject is the tarball's copy; this is what it was rendered from, and the diff between them is the finding | When comparing the extracted fence against the tree it came from | AC-003, AC-004, AC-007 |
| `crates/happenstance/Cargo.toml` | `default = ["std", "memory"]` forwarded wholesale to `happenstance-core` (`:14-26`), so the two cannot disagree about what `default-features = false` means — the reason the scratch project takes the default and hand-picks nothing | Before writing the scratch manifest, and when reading `cargo tree -e features` | AC-005 |
| `xtask/src/main.rs` | `--locked` at every cargo invocation (`:52-55`) and the Mandatory/Optional line (`:33-38`, where *optional* means *the tool was absent*, never *the check would fail*) — together, why this can never be a gate step | Before any impulse to add an `xtask` subcommand | AC-008 |
| `xtask/src/package.rs` | `REQUIRED_FILES` and the containment assertion (`:88-94`), and the module doc's *containment is not presentation* (`:4-18`) — which is exactly the boundary EC-002's broken-artefact case falls outside | When a dependency build fails and the question is whether the tarball itself is broken | AC-001 |
| `standards/rust/62-doctests-and-harnesses.md` | Why a packaged example cannot reach outside its own package, and what a doctest's hidden lines do that a copier never receives — the reasoning behind naming the scaffolding rather than folding it in | Before enumerating the scaffolding for AC-004 | AC-004 |
| `standards/rust/51-features-and-no-std.md` | What a `yank` does and does **not** do (`:224-231`): it removes a version from future resolution and leaves every rendered page exactly as it was — the constraint that makes *route forward* the only correct handling | Before recommending any remediation for a finding | AC-008 |

## Clarifications resolved during spec

1. **Where the scratch project lives** (deferred by `discover.md`:32). A throwaway `cargo new` in a
   temporary directory **outside this repository**, with no ancestor `Cargo.toml` carrying a
   `[workspace]` table, destroyed after the run. It may not live under a `.gitignore`d directory in
   this tree: *outside the workspace* is the property under test, and `.gitignore` does not change
   what Cargo's ancestor walk finds.
2. **Manual or automated** (the same deferred question). **Manual, and recorded** — never an `xtask`
   subcommand, a gate step or a CI job. The testing brief had already placed the seam outside the
   gate; this spec pins the consequence: the artefact is a committed dated transcript.
3. **Which text runs.** The fence **extracted from the `.crate` tarball's `README.md`**, not from the
   working tree and not from `_design.md`. Both are expected to be identical; the extraction site is
   chosen so that a difference is *detectable* rather than assumed away.
4. **What *someone who did not build it* means in practice** (project DoD 2). Carried by **verbatim
   reproducibility from the record alone** (NF-003), with the actual runner and environment stated
   plainly and any gap named. A claim about who ran it is not checkable; a recipe is.
5. **The scaffolding gap is a finding, not a repair.** The published fence is an `async fn` nobody
   awaits, so a `main`, an executor and one extra registry dependency are needed. They are enumerated
   in the record and the discrepancy with AC-UX-012 is routed to
   `guarantees-and-docs-rs-presentation` and a forward version. Copy is frozen once
   `rendered-page-preflight` has read it and `publish-0-2-0` has shipped it.
6. **The `wasm32` arm is a `check`, never a `run`**, and the executor may be substituted out of the
   checked target if it does not type-check there — with the substitution recorded, because the claim
   under test is about `happenstance`.
7. **The toolchain is exactly 1.97.1**, the number promised at `crates/happenstance/README.md`:46-49
   and pinned at `rust-toolchain.toml`:2 — not *current stable*, and not *whatever is installed*.
8. **The feature set is the default and only the default.** No `features` key, no
   `default-features = false`. Hand-picking a set would be the unification mistake with better
   manners.
9. **This story reads the packaged surface's composition, and only reports.** It is the sole
   instrument in the project that reads the **tarball's** README rather than the tree's or the
   rendered page; **AP-12** and **AP-7** findings are routed, never repaired here.
10. **`_smoke-log.md` is written once.** A re-run against a later version is a later story's record.
    Appending would turn evidence about `0.2.0` into a mirror of the newest state.
11. **AC enumeration is unchanged from the front half**: exactly **AC-001 … AC-008**, none added and
    none dropped. The composition invariants the design binds are carried inside AC-003 and AC-004
    rather than as new rows, so `redkiln verify`'s extraction and this spec's traceability agree.
