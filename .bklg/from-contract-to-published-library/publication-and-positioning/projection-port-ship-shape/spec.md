---
item: HS-S0085
stage: spec
created: 2026-08-12T13:47:23.229Z
updated: 2026-08-12T13:47:23.229Z
template_sig: 87bbf1d0
rendered_sig: "55173473"
---

# Spec — PS-3 gets a verdict on evidence: frozen, or behind unstable-projection

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` — BR-15 (every settled answer is a decision atom), DoD 8 (the freeze verdict is written) |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md`:140-153 — the DAG that puts both projection siblings upstream of this project; `:115-121` — the 0017–0028 ADR allocation this story's number must clear |
| Project | `.bklg/from-contract-to-published-library/publication-and-positioning/project.md` — DR-12 (`:205-206`), DR-13 (`:207-212`), DR-15 (`:217-219`), AC-012 (`:270-272`), AC-013 (`:273-276`), and the Out-of-scope lines that keep the port's *design* elsewhere (`:123-130`) |
| Key brief — deployment | `.bklg/from-contract-to-published-library/publication-and-positioning/_decomposition.md`:615-641 — *Feature flags / config gating*, where PS-3's inheritance and the RS-51-5 obligation are stated; `:756-759` — **AC-DEP-002** |
| Key brief — testing | `.bklg/from-contract-to-published-library/publication-and-positioning/_decomposition.md`:475 — AC-012's instrument (*static, citation check*) and its named wrong implementation |
| Signed-off design (**binding**) | `.bklg/from-contract-to-published-library/publication-and-positioning/_design.md`:59-74 (`## Items`, the conditional feature entry), `:84-102` (`## Signatures`), `:524-539` (`## Shape decision`, PS-3 row + *Provisional clauses carried into implementation*), `:566-579` (`## Visibility and stability`), `:632-639` (`### The states the API must express`), `:660-661` (**AP-8**) |
| Story map row | `.bklg/from-contract-to-published-library/publication-and-positioning/_storymap.md`:57; the archetype note at `:110-114`; merge order `:156-160` |
| Discover stage | `.bklg/from-contract-to-published-library/publication-and-positioning/projection-port-ship-shape/discover.md` — the signal ledger, the three deferred questions (`:30-32`) and the named wrong implementation (`:40`) |
| This spec | `.bklg/from-contract-to-published-library/publication-and-positioning/projection-port-ship-shape/spec.md` |
| Roadmap pointer | `RUNBOOK.md`:601 — the provisional ledger's placeholder verdict, *"6, decided at 12"*; `RUNBOOK.md`:4448-4499 — phase 12, which is this project |

## One-line PR slice

Settle PS-3 — the projection port ships frozen, or behind `unstable-projection` —
by applying PS-2's frozen bar to the three sibling evidence documents rather than
re-deriving it, record the verdict and the arm that lost as an ingested decision
atom, and make the gated arm *visible with its gate* on the docs.rs page of the
crate a consumer installs, which nothing in the gate compiles today.

## Executive summary

**What already exists when this story starts, and is not this PR's delta.** The
feature is not being invented here. `projection-store-freeze` lands
`unstable-projection` in `happenstance-core` at phase 6 — gating `pub mod
projection;` and its four re-exports with `#[cfg(…)]` plus the matching
`#[cfg_attr(docsrs, doc(cfg(…)))]`, off by default — and explicitly declines to
give the facade a passthrough, because at that point `happenstance` re-exported
nothing from the module
(`.bklg/from-contract-to-published-library/projection-store-freeze/unstable-projection-gate-and-clause-disposition/spec.md`:375,
`:155-161`). `typed-layer-and-alpha-release` then gives `happenstance` its own
`unstable-projection` at phase 7, gating the typed `Projection` trait and
`run_projection` and forwarding the core feature
(`.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/projection-trait-and-runner/spec.md`:41-60).
Both are inside the `0.2.0-alpha.1` baseline this project diffs against. That
answers `discover.md`:32's deferred question mechanically: **both crates carry the
feature, and both already do by the time this story opens.**

**The delta is three things, and none of them is the feature.**

1. **A verdict, taken by applying a `[FROZEN]` bar to evidence.** PS-3 is a
   *SHOULD with a condition* — *"Until PS-2's bar is met"* (`spec/SPECIFICATION.md`:4776-4784) —
   and PS-2's bar is frozen text with three named parts (`:4760-4775`). Nothing
   in the tree has ever evaluated it against a real adapter set. This PR does,
   part by part, citing the sibling reports, and records what it found.
2. **The verdict as an accepted decision atom**, numbered 0030 or above, naming
   the arm that lost and its cost, authored through `.kb/_intake/` and
   `/redkiln:kb-ingest` in the `release-decisions` slice's single wave
   (`_storymap.md`:73-78). AC-013 has no other discharge.
3. **The gated arm made checkable where it is actually read.** `xtask/src/main.rs`'s
   nightly `docs.rs configuration (nightly)` step names `happenstance-core` and
   `happenstance-testkit` only (`:621-635`). `happenstance` — the crate a `cargo
   add` reader installs, and the one whose docs.rs page is `primary: true` in the
   signed-off design (`_design.md`:138-144) — acquired a `doc(cfg)`-gated public
   surface at phase 7 and **nothing in this repository has ever compiled that
   spelling for it**. The step's own comment states the joining rule in its own
   words: `happenstance-testkit` *"joined the list when it acquired
   `#![cfg_attr(docsrs, feature(doc_cfg))]`"* (`:617-620`). This PR applies that
   rule to the third publishable crate, which is what turns *"renders with its
   gate"* from an intention into a gate step.

Nothing about the port's behaviour, signature or feature graph changes. What
changes is whether a caller opting in is a recorded decision, and whether the
page they meet says so.

## Context pack

Everything here is load-bearing and stated as a decision. Depth is behind the
anchors table; do not open an anchor to learn something this section already says.

**1. The verdict is not this spec's to make, and it is not the implementer's
preference either. It is an evaluation of frozen text against three documents.**
PS-2 is `[FROZEN]` (`spec/SPECIFICATION.md`:4760-4763) and its bar has three
conjunct parts, all of which must hold before the port may be frozen:

- `CheckpointOnlyStore` — a testkit-internal hostile store that commits the
  checkpoint and discards the read-model write — **fails**
  `commit_is_atomic_with_the_read_model`;
- the projection suite is green against **one adapter holding a live
  transaction** (rusqlite or `sqlx`);
- the projection suite is green against **one adapter that cannot hold anything
  across an await** (Workers `SqlStorage`, or Neon over one-shot HTTP).

Read the third part exactly as written. It names a *transport* far end, not
merely a second implementation. `LadybugProjectionStore` is a third batch shape
and a genuinely unlike one, but it holds a real write handle — it is not the
adapter PS-2's third part names, and PS-2's **Rejects** field
(`:4770-4775`) is a standing warning against counting two handle-holding stores as
two ends of the axis. **Reinterpreting a frozen bar generously so that the
evidence clears it is the single most damaging move available in this story**, and
AC-015 / DR-15 (`project.md`:281-283, `:217-219`) exist to make it a stop rather
than a judgement call.

**2. The decision space is two arms, and one of them is a halt.**

- **Arm A — ship gated.** `unstable-projection` stays, off by default, on both
  published crates, carrying the documented semver exemption and a statement of
  what retires it. Available in *every* evidence state: if the bar is unmet, PS-3
  makes it the SHOULD; if the bar is met, PS-3's condition lapses and shipping
  gated becomes a deliberate deferral of stabilisation rather than an obligation.
  Either way the atom must say **which of those two sentences is true**, because
  they are different promises to a consumer.
- **Arm B — freeze and un-gate at `0.2.0`.** Reachable only if all three parts of
  PS-2's bar hold on the evidence. It removes a public feature and re-exposes
  gated items unconditionally — a change to the published surface against the
  `0.2.0-alpha.1` baseline that `registry-surface-diff` (AC-002) is built to
  report, and an API-surface change `project.md`:123-126 reserves for the crate
  that owns it *"and a re-plan, not an edit made here."* So **arm B is recorded
  and then blocks**: the atom states the verdict, the release stops, and the
  un-gating is planned by whoever owns the surface. Executing arm B inside this
  story is out of scope by the project's own charter, and doing it anyway is the
  named wrong implementation's twin.

**3. The evidence is three documents, cited by path, and the default is inherited
rather than preferred.** `RUNBOOK.md`:601 carries the ledger's placeholder —
*"Ship behind `unstable-projection` | PS-3 | the two batch shapes disagreeing at
phase 6 | 6, decided at 12"* — and the deployment brief is explicit that this
project **inherits it absent the siblings' evidence overriding it**
(`_decomposition.md`:621-626). A verdict written off that line alone is precisely
`discover.md`:40's named wrong implementation. The three citable documents are:

- `references/evaluation/projection-batch-shape-evidence.md` — the PS-3 batch-shape
  finding, *including* the "they agreed everywhere" null result, which is a finding
  about two **testkit** shapes and is explicitly not a verdict
  (`…/projection-store-freeze/ps3-batch-shape-finding/spec.md`:26-45);
- `references/evaluation/phase-11-freeze-verdict.md` — the dated *held / did not
  hold* verdict, naming the implementation, the rules run, the PS clause ids and
  the commit (`…/ladybug-projection-store/freeze-verdict-document/spec.md`:36-40);
- `references/evaluation/phase-11-publication-handover.md` — the ordering claim
  and the three residues phase 11 hands this project, one of which is stated as a
  question and must stay one
  (`…/ladybug-projection-store/verdict-ordering-and-publication-handoff/spec.md`:38-48).

Each is `references/evaluation/` genre: *immutable evidence… dated, pinned to a
commit… superseded rather than edited* (`references/evaluation/README.md`:9-13).
Cite them by `file:line`; do not restate their findings as this story's own.

**4. A gated item that vanishes from the page is a false claim, and the primary
docs surface is the one nothing checks.** IQ-2's non-occlusion invariant and
**AP-8** (`_design.md`:660-661) forbid a feature-gated public item rendering
*without* its gate annotation or absent entirely — an absence reads as "not
supported", which is a different and untrue statement. RS-51-5
(`standards/rust/51-features-and-no-std.md`:188-236) is the mechanism, and its
whole argument is that the failure is invisible everywhere except docs.rs, *after*
the one act that cannot be undone: a yank removes the version from the resolver
and leaves the page exactly as it is. `happenstance` is not in the nightly
`--cfg docsrs` step today (`xtask/src/main.rs`:621-635), so for the facade this
class of defect currently has no detector at all.

**5. The presentation is bound by the design and is not re-decided here.**
`_design.md`:530 binds it in one row: whichever arm AC-012 picks, the port renders
with its gate — `#![cfg_attr(docsrs, feature(doc_cfg))]` plus
`#[cfg_attr(docsrs, doc(cfg(…)))]`, under `all-features = true` — and the same row
rejects *"decide PS-3 here"* as out of the design stage's authority. The
corresponding obligation on the *stability* side is `_design.md`:570: the semver
exemption is **stated on the item, not only in an ADR**, and *"if AC-012 freezes it
instead, the exemption text does not ship."* Two published crates carry gated
items; the consumer installs `happenstance`.

**6. PS-3's marker is recorded, not moved — and its falsifier has a defect worth
recording.** Moving a maturity marker is a decision's act, and this story does hold
a decision — but moving PS-3 changes §1.3's hand-computed census (200 / 198 / 139 /
49 / 10 / 2, `spec/SPECIFICATION.md`:219-222) and the clause-ID set of `RUNBOOK.md`'s
provisional ledger, which `clause-maturity-audit` asserts equal to `spec-trace`'s
parsed `[PROVISIONAL]` list and which DT-5's published census sentence quotes
verbatim (`_design.md`:290-296). **This story moves neither.** Separately: PS-3's
falsifier reads *"falsified the moment PS-2's bar is met before 0.1"*, and 0.1 was
never published — a falsifier whose triggering event can no longer occur is the
exact pattern `.kb/open-questions/es-7-and-vt-9-provisional-markers.md` records as
*"a marker that has quietly become decoration."* Record that observation in the
intake document and let the ingest wave route it; do **not** hand-edit an
open-question atom, and do not repair the falsifier in passing.

**7. Atoms are ingested, never hand-written, and the slice shares one wave.**
`.kb/` atoms are authored by `/redkiln:kb-ingest` from `.kb/_intake/`; hand-writing
them produces the directory layout of the process without the process, and the
first attempt was reverted (`0269720`, `CLAUDE.md`). The implementer's deliverable
is a **staged intake document plus a handoff** — the redkiln SDLC commands are
user-invoked. `.kb/_intake/README.md` is dropped at the wave's approval gate rather
than ingested, and a successful wave clears the directory (`.kb/_intake/README.md`:3-5,
`:13-19`). Numbers: `.kb/decisions/` holds 0001–0016 and 0029; 0017–0028 are
allocated to siblings; this story's atom takes the next free number above the corpus
in the allocation `crate-set-decision` states for the slice (`project.md`:207-212,
`_storymap.md`:73-78).

**8. The persona slice.** Backbone activity A1 has no user intent of its own
(`_storymap.md`:45-50), but this story is a **capability** rather than a foundation
for a stated reason: its outcome is visible to a consumer either way
(`_storymap.md`:110-114). The person on the other side is the application author in
their evaluator moment — the fifteen minutes `_design.md`:214-220 folds into
Persona 1 — reading `docs-rs-happenstance` and deciding whether the projection
surface is something they can build on. Their humane outcome is that the answer is
an annotation on the item they are looking at, with the cost of depending on it
stated beside it, rather than an item that silently is not there.

## Integration contract

| Field | Value |
| --- | --- |
| **Archetype** | `capability` — the outcome is consumer-visible either way: a gate rendered *with* its annotation on the docs.rs page of the installed crate, or a frozen port and a blocked release (`_storymap.md`:110-114) |
| **Slice / milestone** | `release-decisions`. Slice-mates, implemented in one context and mounted as one integrated wave: `crate-set-decision` (blocker, first in merge order), `msrv-promise-atom`, `first-contact-design-resolutions` (`_storymap.md`:156-160) |
| **Mount point** | `xtask/src/main.rs` — the `docs.rs configuration (nightly)` step (`:596-638`, name at `:621`), extended with `-p happenstance`. That step is the only thing in the tree that sets `--cfg docsrs`, so it is the real render path where *"ships with its gate"* becomes an executed check rather than an intention; its own comment (`:617-620`) states the crate-joins-the-list rule this applies. Second mount, in the knowledge tree: the new atom at `.kb/decisions/00NN-…md` plus its row in `.kb/maps/decision-map.md` — an atom with no map row is an atom the corpus cannot see (`.kb/maps/decision-map.md`:81-86) |
| **Wires into** | `crates/happenstance-core/src/lib.rs`'s gated `pub mod projection;` and its four re-exports, and `crates/happenstance-core/src/projection.rs`'s module doc (phase 6's landing); `crates/happenstance/Cargo.toml`'s `[features]` block (`:22-26`) and the forwarded `unstable-projection` (phase 7's landing); `crates/happenstance/src/lib.rs`'s crate-root ladder and its `pub use happenstance_core::*;` (`:76`); the two feature-powerset steps (`xtask/src/main.rs`:546-556, `:564-591`) that must still compile every combination; `references/evaluation/` as the evidence inlet and `.kb/_intake/` as the knowledge inlet |
| **Renders surfaces** | `docs-rs-happenstance` (**primary**) and `docs-rs-happenstance-core` — `_design.md`:138-152 — in their `all-features` state. This story changes what those two pages render for every `unstable-projection`-gated item. It renders **no** crates.io surface: the copy on `crates-io-*` belongs to `landing-copy-and-status-truth` and `guarantees-and-docs-rs-presentation` |
| **Conformance rule(s)** | **none, and it is not adapter-observable.** No rule in `crates/happenstance-testkit/src/suite.rs` can observe whether a port ships behind a feature — a rule that could would be observing the manifest, not the store. The observing instruments are the nightly `--cfg docsrs` step, the two feature powersets and the AC-002 surface diff. Stated explicitly because a story that names no rule must say which of the two reasons applies (`CLAUDE.md`) |
| **Clause(s)** | **PS-3 discharged, not amended** (`spec/SPECIFICATION.md`:4776-4790): its `[PROVISIONAL]` marker, its falsifier text and its owning-phase row stay byte-identical; what changes is that the decision its schedule assigned to phase 12 has been taken and is citable. **PS-2 is `[FROZEN]` and is read, never touched** (`:4760-4775`). No other clause is entered, and `spec/SPECIFICATION.md` is expected to be unmodified by this PR |
| **Advances DoD scenario** | Initiative **DoD 8** — *"the freeze verdict is written"* — by converting the two projection projects' evidence into the exposure decision that verdict was owed to; and **DoD 10** (*the published crate looks finished*), because a gated item rendering without its gate is the specific way this release would look unfinished on the page a consumer lands on. At project grain: AC-012 (sole owner), AC-013 (one of four atoms), and it unblocks `landing-copy-and-status-truth` and `guarantees-and-docs-rs-presentation` (`story.md` frontmatter `blocks:`) |

## PR boundary

```
.kb/_intake/**
.kb/decisions/**
.kb/maps/decision-map.md
.kb/_governance/integration-waves/**
.redkiln/telemetry/**
crates/happenstance/src/lib.rs
crates/happenstance-core/src/projection.rs
xtask/src/main.rs
CHANGELOG.md
RUNBOOK.md
.bklg/from-contract-to-published-library/publication-and-positioning/projection-port-ship-shape/**
```

**In this PR**

- The evaluation of PS-2's three-part bar against the three sibling evidence
  documents, written down part by part with its citations, and the verdict that
  follows from it.
- The staged intake document under `.kb/_intake/`, and the atom, decision-map row
  and wave record the `/redkiln:kb-ingest` run writes from it.
- `-p happenstance` added to the nightly `docs.rs configuration (nightly)` step,
  with one sentence in that step's comment saying why the third publishable crate
  joined — matching the sentence already there for `happenstance-testkit`.
- The semver-exemption statement and what retires it, on the gated items of the
  crate a consumer installs, if phase 7's landing left it only in the changelog and
  the ADR. Documentation text on an existing item, not a surface change.
- One pointer at `RUNBOOK.md`'s phase-12 body naming the atom as PS-3's verdict of
  record, and the `[Unreleased]` `CHANGELOG.md` entry recording the exposure
  decision for `0.2.0` (the standing claim at `:19-22` gains a verdict, not a
  rewrite).
- This story's `spec.md` and `_ledger.md`, and the captured runs the ledger cites.

**Explicitly not in this PR**

- **Any change to a feature's existence or membership.** `unstable-projection` is
  not added, renamed or removed on either crate, and no feature is moved into or
  out of `default`. If the feature is *missing* when this story opens, that is an
  upstream story that did not land — halt and report; do not invent it here.
- **`[package.metadata.docs.rs]` in `crates/happenstance/Cargo.toml`** —
  `guarantees-and-docs-rs-presentation` owns it (`_design.md`:68-73,
  `_storymap.md`:66). The two are separable on purpose: the nightly gate step
  passes `--all-features` and `RUSTDOCFLAGS` itself, so this story's check runs
  without that block and that block ships without waiting on this story.
- **Un-gating the port, or any edit under `crates/happenstance-core/src/projection.rs`
  beyond its module documentation.** Arm B is recorded and blocks; the surface
  change belongs to the crate that owns it plus a re-plan (`project.md`:123-126).
- **Any marker move in `spec/SPECIFICATION.md`**, any edit to §1.3's census, and
  any entry into `RUNBOOK.md`'s provisional ledger table at `:588-610` —
  `falsifier-ledger-repair` owns that table and must land before anything reads it
  (`_storymap.md`:179-184).
- **The published copy.** The maturity census, the Guarantees bullets, the
  disambiguation triad and the `description` field's truth re-read are
  `landing-copy-and-status-truth`'s and `guarantees-and-docs-rs-presentation`'s. If
  this story's verdict makes a published sentence false — `crates/happenstance/README.md`:11
  and the manifest `description`'s *"projection runners"* are the two candidates —
  it is **handed over as a named finding**, not fixed here.
- **A long-form record under `references/adr/`.** Nothing cites a long form for this
  decision; the atom is canonical for it, and the evidence it rests on already lives
  in `references/evaluation/`.

**Merge DoD (one line).** The atom exists and is accepted, states the verdict with
PS-2's bar evaluated part by part against three cited evidence documents and names
the arm that lost with its cost; `redkiln validate --kb` and `redkiln doctor` are
clean; and a nightly `RUSTDOCFLAGS="--cfg docsrs -D warnings"` build of all three
publishable crates under `--all-features` is green with every gated item carrying
its `doc(cfg)` annotation.

## Behavior and interfaces

**There is no Rust signature change and no feature-graph change.** `_design.md`'s
`## Items` block carries one conditional feature entry and one manifest key, and
both are already landed or owned elsewhere; `## Signatures` (`:84-102`) describes
the attribute treatment on an *unchanged* trait. The interfaces this story touches
are the gate's (one step's crate list), the documentation's, and the knowledge
base's.

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **B1 — the shipped state is observed before it is decided** | Before writing anything, the implementer records the tree as it actually is: `unstable-projection` present and **absent from `default`** in both `crates/happenstance-core/Cargo.toml` and `crates/happenstance/Cargo.toml`; the gated items and their `doc(cfg)` attributes present in core; the facade's forwarding present. Absence of any of these is a halt with a named upstream story, never a local fix | `crates/happenstance/Cargo.toml`:22-26; `crates/happenstance-core/Cargo.toml` `[features]`; `…/projection-store-freeze/unstable-projection-gate-and-clause-disposition/spec.md`:375; `…/typed-layer-and-alpha-release/projection-trait-and-runner/spec.md`:41-60 |
| **B2 — PS-2's bar is evaluated part by part, against frozen text** | Three rows, each answered *met / not met / no evidence* with a `file:line` citation: `CheckpointOnlyStore` failing `commit_is_atomic_with_the_read_model`; the suite green against an adapter holding a live transaction; the suite green against an adapter that cannot hold anything across an await. The third row is answered as written — a third unlike shape that holds a handle does not satisfy it | `spec/SPECIFICATION.md`:4760-4775; `references/evaluation/phase-11-freeze-verdict.md`; `references/evaluation/projection-batch-shape-evidence.md` |
| **B3 — the verdict follows the bar mechanically, and says which sentence is true** | Bar unmet → arm A, and the atom says *PS-3's SHOULD still binds*. Bar met → the atom says *PS-3's condition has lapsed* and records the exposure choice as a deliberate deferral (arm A) or a freeze (arm B). The two arm-A sentences are different promises and the atom must not blur them | `spec/SPECIFICATION.md`:4776-4790; `_decomposition.md`:617-626 |
| **B4 — the arm that lost is named with its cost** | Arm A's cost: a caller writes one manifest line and the typed projection runner is opt-in. Arm B's cost, stated even when it wins: removing a public feature from the `0.2.0-alpha.1` baseline is a surface change `registry-surface-diff` reports, and freezing a port whose bar is unmet is the larger and less visible cost (`_design.md`:583-586) | `_design.md`:581-590; `.bklg/…/publication-and-positioning/_decomposition.md`:615-633 |
| **B5 — arm B halts rather than un-gates** | If the bar is met and the verdict is *freeze*, the atom is written, the release path stops, and the un-gating is routed to the crate that owns the surface plus a re-plan. The story does not delete a feature, does not un-gate a module, and does not ship an atom that says "frozen" beside a manifest that says `unstable-projection` | `project.md`:123-126, `:217-219`; `_decomposition.md`:657-672 (block-and-re-plan posture) |
| **B6 — the atom cites evidence, never the placeholder alone** | Every claim of fact in the atom resolves to one of the three `references/evaluation/` documents by `file:line`. `RUNBOOK.md`:601 may be cited as *the default that was inherited*, never as the source of the verdict — that substitution is `discover.md`:40's named wrong implementation | `discover.md`:40; `_decomposition.md`:475 (testing brief, AC-012 row); `references/evaluation/README.md`:9-13 |
| **B7 — the atom is valid `KbFrontmatter` of the decision shape, ingested** | One staged file at `.kb/_intake/<nnnn>-the-projection-port-ship-shape-at-0-2-0.md`, then a handoff: the human runs `/redkiln:kb-ingest`, whose wave authors the atom, syncs the maps, wires backlinks, runs `validate --kb`, clears `_intake/` and commits. `kind: decision`, `status: accepted`, `phase: 12`, `supersedes: null`, `related` naming the projection-port atoms the wave finds, `source_paths` citing the three evidence documents | `.kb/decisions/0029-msrv-raised-to-1-97-1.md`:1-33 as the shape of record; `.kb/_intake/README.md`:3-5, `:13-19`; `CLAUDE.md` (*Where the work lives*) |
| **B8 — the number is free and the slice's wave does not collide** | The atom takes the number `crate-set-decision`'s intake document allocated to it — the lowest free number ≥ 0030 not taken by the 0017–0028 sibling allocation or by 0029. The id the wave actually assigns is what the ledger cites | `project.md`:207-212; `.bklg/from-contract-to-published-library/_decomposition.md`:115-121; `_storymap.md`:73-78 |
| **B9 — every gated public item renders with its gate, on all three crates** | A nightly `RUSTDOCFLAGS="--cfg docsrs -D warnings" cargo +nightly doc --all-features --no-deps` build over `happenstance-core`, `happenstance` and `happenstance-testkit` is green, and the rendered `happenstance` and `happenstance-core` pages are **looked at**: every `unstable-projection` item carries its annotation and none is absent. AP-8 is the bar and it is phrased to be checkable from the page rather than from the source | `_design.md`:660-661, `:392-397`; `standards/rust/51-features-and-no-std.md`:188-236; `xtask/src/main.rs`:596-638 |
| **B10 — the facade's rendering is verified, not assumed, and the remedy is presentation-only** | `happenstance` re-exports through `pub use happenstance_core::*;` (`crates/happenstance/src/lib.rs`:76), and whether a `doc(cfg)` annotation survives a cross-crate glob re-export is a fact to observe, not to reason about. If the pill does not appear, the remedy is an explicit gated re-export naming exactly what the glob already re-exports, plus `#![cfg_attr(docsrs, feature(doc_cfg))]` on the facade — the reachable set is unchanged, so `registry-surface-diff` must report no item added or removed | `crates/happenstance/src/lib.rs`:10, `:73`, `:76`; `standards/rust/51-features-and-no-std.md`:196-215; `project.md`:230-232 (AC-002's instrument) |
| **B11 — the gate acquires the check it lacked** | `-p happenstance` joins `xtask/src/main.rs`'s nightly step and the step's comment gains one sentence naming why, in the form already used for `happenstance-testkit`. The module doc at `:8-24` is not entered — its optional list names steps, not crate lists | `xtask/src/main.rs`:596-638, `:617-620`, `:43-52` |
| **B12 — the exemption is stated where the consumer meets it, with what retires it** | The gated items in the crate a consumer installs carry, in prose on the item: that the surface is exempt from semver while gated, and that PS-2's bar is what retires the exemption. Not only in `CHANGELOG.md`:19-22 and not only in the atom. In arm B this text does not ship at all | `_design.md`:566-579; `CHANGELOG.md`:19-22; `spec/SPECIFICATION.md`:4760-4775 |
| **B13 — every feature combination still compiles** | Both powerset steps stay green, including `--no-default-features --features unstable-projection` on each crate. Neither is skipped: `cargo-hack` resolves on this machine, and a skipped step is a weaker claim that must read as one | `xtask/src/main.rs`:546-556, `:564-591`; `.kb/decisions/0010-the-suite-must-prove-itself.md`; `CLAUDE.md` (*Commands*) |
| **B14 — PS-3's marker, §1.3's census and the ledger's clause set are untouched** | `git diff` shows `spec/SPECIFICATION.md` unmodified and no line inside `RUNBOOK.md`:588-610. The falsifier defect — a condition naming *"before 0.1"*, an event that can no longer occur — is written into the intake document for the wave to route against `.kb/open-questions/es-7-and-vt-9-provisional-markers.md`, and is not hand-repaired anywhere | `spec/SPECIFICATION.md`:219-222, `:4776-4790`; `RUNBOOK.md`:601, `:622-635`; `.kb/open-questions/es-7-and-vt-9-provisional-markers.md` |
| **B15 — what this verdict makes false elsewhere is handed over, not fixed** | Any published sentence the verdict contradicts is recorded as a named finding for its owning story — in particular `crates/happenstance/README.md`:11 (*"projection runner — arrives here"*) and the `description` field's *"projection runners"* claim, both of which `landing-copy-and-status-truth` and the design's IQ-7 truth re-read already own | `_design.md`:462-466 (the `description` budget), `:187`; `_storymap.md`:64, `:66` |

## Data and migrations

**N/A — no schema, no migration, no backfill.** This library defines no storage
schema of its own; each adapter owns its own, and none of them is in this story's
boundary. `0.2.0` is the first non-alpha publish of these three crates, and the
deployment brief records migration and backfill as N/A with the reason: there is no
prior production data shape, and the reserved `0.0.0` names are a registry
reservation with no compatible predecessor
(`.bklg/from-contract-to-published-library/publication-and-positioning/_decomposition.md`:643-655;
`CONTRIBUTING.md`:298-300).

Two state changes this story *does* make, each with a one-way discipline rather
than a migration:

- **Knowledge-base state.** The ingest wave adds an atom, adds a decision-map row,
  records the wave, and clears `.kb/_intake/`. There is no reverse migration for an
  accepted decision atom: the correction path is a new atom carrying `supersedes`,
  never an edit (`.kb/decisions/README.md`). Before the wave merges, rollback is an
  ordinary branch discard.
- **Consumer-visible feature state, which is deliberately *not* changed.** The
  feature graph a consumer resolves is identical before and after this PR in arm A,
  which is why it needs no compatibility story at all; arm B would change it, and
  that is precisely why arm B is a halt rather than an edit. The only durable
  artefact either way is the atom and the gate step.

## Acceptance criteria

Seven criteria. Each is written from the intent of a person crossing the whole
stack, not from a capability: the **maintainer deciding once** (backbone A1,
`_storymap.md`:45-50) and the **application author in their evaluator moment** —
Persona 4 folded into Persona 1 by `_design.md`:214-220, whose window is one
sitting and who cannot run this repository's suite
(`.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md`:249-313,
`:333-338`). AC-001–AC-003 are the maintainer's; AC-004–AC-006 are the
evaluator's; AC-007 is the consumer already on `0.2.0-alpha.1`.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **GIVEN** the maintainer must settle PS-3 at phase 12 and the three sibling evidence documents exist under `references/evaluation/`, **WHEN** they evaluate PS-2's frozen three-part bar, **THEN** the record answers each part separately — hostile `CheckpointOnlyStore` failing `commit_is_atomic_with_the_read_model`; the suite green against an adapter holding a live transaction; the suite green against an adapter that cannot hold anything across an await — as *met / not met / no evidence*, each carrying a `file:line` citation into one of those three documents, and the arm follows from the three answers mechanically rather than from preference | **static, citation check** (`_decomposition.md`:475). A reviewer reads the intake document and the atom and confirms three answered rows with resolving `references/evaluation/**` citations, and that the third row is answered as written — a third unlike shape that holds a write handle does not satisfy it. Rejects the testing brief's named wrong implementation: *a verdict asserted without citing either sibling project's report*, i.e. written off `RUNBOOK.md`:601 alone (`discover.md`:40) |
| AC-002 | **GIVEN** a maintainer six months later asking *why is the projection port gated (or frozen)*, **WHEN** they open `.kb/maps/decision-map.md` and follow one row, **THEN** they reach an **accepted** decision atom, numbered above the corpus and clear of the 0017–0028 sibling allocation, that states the verdict, says explicitly whether PS-3's SHOULD still binds or its condition has lapsed, and names the arm that lost **with its cost** — and the atom was produced by `/redkiln:kb-ingest` from `.kb/_intake/`, never hand-written | **process** (`_decomposition.md`:475, AC-013 row). `redkiln validate --kb` and `redkiln doctor` clean — `doctor` reporting exactly six `template-drift` advisories and zero `dependency-cycle`; `git log` shows the atom arriving in the ingest wave's commit and `.kb/_intake/` cleared by it; the decision-map row resolves. Rejects: an atom hand-written under `.kb/decisions/`, which is the shape reverted at `0269720` |
| AC-003 | **GIVEN** the maintainer's standing obligation that nothing frozen is amended to make a release date, **WHEN** the verdict is written — including the case where the bar **is** met and the verdict is *freeze* — **THEN** the release path halts rather than un-gating: no feature is deleted, no gated module is exposed unconditionally, and `spec/SPECIFICATION.md` is byte-identical (PS-3's `[PROVISIONAL]` marker, its falsifier text and §1.3's 200 / 198 / 139 / 49 / 10 / 2 census all unmoved), with no line inside `RUNBOOK.md`:588-610 touched; PS-3's dead falsifier — a condition naming *before 0.1*, an event that can no longer occur — is written into the intake document for the wave to route, not repaired in passing | **static + gate**. `cargo xtask spec-trace` green, and `git diff --stat main...HEAD` recorded in the ledger evidence showing `spec/SPECIFICATION.md` absent from the changed set and `RUNBOOK.md`'s diff hunks outside `:588-610`. Rejects: DR-15's failure — a frozen bar reinterpreted, or a marker moved, so that the evidence clears (`project.md`:217-219, `:281-283`) |
| AC-004 | **GIVEN** the evaluator's docs.rs page for `happenstance` is built by a renderer that runs **after** the one act that cannot be undone, **WHEN** `cargo +nightly` is present on the machine or the CI gate job, **THEN** `xtask/src/main.rs`'s `docs.rs configuration (nightly)` step names all **three** publishable crates and its comment states why the third joined, and the build is green under `RUSTDOCFLAGS="--cfg docsrs -D warnings" cargo +nightly doc --locked --all-features --no-deps` — so a `doc(cfg)` spelling that does not compile fails **here**, not on a version that can be yanked but never removed | **build (nightly, probed)**. The captured run is committed under this story's directory and cited by the ledger. Because the step carries `probe: Some(&["cargo", "+nightly", "--version"])` (`xtask/src/main.rs`:635), a machine without nightly **skips** it, and a skip is recorded as a skip and never counted as this AC's evidence — the same discipline `.kb/decisions/0010-the-suite-must-prove-itself.md` binds in the testkit. Rejects: adding the crate to the list without ever running the step, and `cargo xtask ci --fast` being offered as the proof (it drops this step by construction, `.redkiln/config.yaml`:50-56) |
| AC-005 | **GIVEN** the evaluator has one sitting and lands on `docs-rs-happenstance` (**primary**) looking for the projection surface, **WHEN** the page renders in its `all-features` state, **THEN** every `unstable-projection`-gated public item is **present and carries its rendered `doc(cfg)` gate annotation naming the feature** — on the facade as well as on `docs-rs-happenstance-core` — so the reader learns *gated*, never *not supported*; no gated item is absent, and no item's gate is carried by prose alone or by a glyph or colour | **human observation of the rendered page** (`_decomposition.md`:475, AC-007 row: *presentation is not metadata*; `xtask/src/package.rs`:4-18). The locally rendered `target/doc/happenstance/` and `target/doc/happenstance_core/` pages from AC-004's build are **looked at** and the observation recorded with its date. This is AP-8 (`_design.md`:660-661) and IQ-2's feature-gating form (`_decomposition.md`:241-258). Rejects: the pill surviving on `happenstance-core` and silently not surviving the facade's `pub use happenstance_core::*;` glob (`crates/happenstance/src/lib.rs`:76) — the remedy is an explicit gated re-export plus `#![cfg_attr(docsrs, feature(doc_cfg))]` on the facade, and it must add and remove no reachable item |
| AC-006 | **GIVEN** the evaluator has just read that the port is gated and now needs to know what depending on it costs, **WHEN** they read the same item — **0 hops**, no link followed — **THEN** the item's own prose states that the surface is exempt from semver while gated and that PS-2's bar is what retires the exemption, in **≤ 3 rendered lines**, in words rather than glyph or colour, adding **no** second maturity count to the page and **no** second primary-ranked element to a screenful, with the module-doc ladder's first `#` heading still inside 12 rendered lines; **and in arm B this text does not ship at all**, which is recorded as a deliberate absence rather than an omission | **human observation + density count**, against `_design.md`:566-579 (the exemption is *stated on the item, not only in an ADR*), `:475` (docs.rs lead paragraph and first-heading budget), `:520` (one primary per screenful), `:641-676` (AP-3, AP-7, AP-15) and IQ-1/IQ-6 (`_decomposition.md`:232-240, `:297-304`). Rejects: the exemption living only in `CHANGELOG.md`:19-22 and the atom — true, recorded, and invisible to the person it is a promise to |
| AC-007 | **GIVEN** a consumer already resolving `0.2.0-alpha.1`, **WHEN** they take `0.2.0` after this story, **THEN** the feature graph they resolve is identical — no feature added, renamed or removed on either published crate and none moved into or out of `default` — both feature-powerset steps stay green including `--no-default-features --features unstable-projection`, the reachable public item set is unchanged so `registry-surface-diff` reports nothing added or removed, and any **published sentence this verdict makes false** is handed over as a named finding to its owning story rather than edited here | **feature matrix + static diff + handover list**. `cargo hack --feature-powerset` on host and `wasm32` (`xtask/src/main.rs`:546-556, `:564-591`) green, neither skipped (`cargo-hack` resolves on this machine); `git diff` over the `[features]` blocks of `crates/happenstance/Cargo.toml` and `crates/happenstance-core/Cargo.toml` empty; the implementation report lists each falsified published sentence with its owner — the two known candidates being `crates/happenstance/README.md`:11 and the manifest `description`'s *projection runners* claim. Rejects: fixing published copy inside this story (AP-9's strings belong to `landing-copy-and-status-truth`) and inventing the feature here if it is missing upstream — that is a halt (`B1`) |

Coverage of the traced project ACs: **AC-012** is discharged by AC-001 (the verdict
on cited evidence) with AC-003 as its stop condition; **AC-013** by AC-002. AC-004,
AC-005 and AC-006 discharge the deployment brief's **AC-DEP-002** second sentence —
*if feature-gated, the gate carries `doc(cfg)` treatment per RS-51-5*
(`_decomposition.md`:756-759) — which no other story in this project owns for the
facade. AC-007 is the regression floor that keeps this story from becoming the
surface change `project.md`:123-126 reserves for a re-plan.

## Interaction quality

RFC §6.7/D6. This story renders two of the project's seven surfaces —
`docs-rs-happenstance` (**primary**) and `docs-rs-happenstance-core`
(`_design.md`:138-152) — so the signed-off `_design.md` is binding here and is not
re-decided. **Every invariant below is carried by an `AC-###` row in the table
above**; this section says which id carries which and how it is verified, because a
bullet here would get no ledger row and would never be gated.

The medium is unusual and the design says so: no DOM selector is nameable, the
renderer is third-party, and `selector` is the *source of record* in this tree
(`_design.md`:110-116). The invariants translate exactly, and translating them is
what stops "renders with its gate" from being an intention.

**State invariants**

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **In place, not a context jump** (IQ-1, `_decomposition.md`:232-240) — the cost of depending on a gated item is readable on the item, 0 hops, not one link into an ADR or a changelog | **AC-006** | Rendered-page read: the exemption sentence and its retirement condition are inside the item's own documentation block |
| **Non-occlusion** (IQ-2, `_decomposition.md`:241-258) — feature gating **annotates, it never subtracts**; an item that vanishes because a feature is off reads as *not supported*, a different and false claim | **AC-005** | Rendered-page read of both docs.rs surfaces in the `all-features` state; AP-8 is the checkable phrasing |
| **Preserved position** (IQ-3, `_decomposition.md`:260-271) — every citation somebody already holds still resolves: PS-3's clause ID, §1.3's census figures and the provisional ledger's rows are all inbound anchors | **AC-003** | `cargo xtask spec-trace` plus the recorded `git diff --stat` showing `spec/SPECIFICATION.md` unmodified |
| **Reversibility, bought before the act** (IQ-4, `_decomposition.md`:272-284) — `cargo yank` removes a version from resolution and leaves the rendered page exactly as it was, so the render must be compiled *before* publish, not observed after | **AC-004** | The nightly `--cfg docsrs -D warnings` build runs in this story's own PR, ahead of `publish-0-2-0`; a skipped probe is not evidence |
| **Reachability without a pointing device** — the docs.rs sidebar is generated from `#` headings, so heading placement *is* the keyboard/screen-reader navigation for this surface | **AC-006** | The first `#` heading stays within 12 rendered lines (`_design.md`:475); adding exemption prose must not push it down |
| **Truth at the publish commit** (IQ-7, `_decomposition.md`:305-311) — a verdict that falsifies a published sentence must not leave the sentence standing | **AC-007** | The handover list; the sentence is fixed by its owning story, not here |

**Composition invariants** (from the signed-off `_design.md`; this story renders a surface)

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **Presentation exists at all** — a bare `#[cfg(feature = "unstable-projection")]` is *markup*; the composed presentation is the rendered pill, which requires all three of `#![cfg_attr(docsrs, feature(doc_cfg))]`, `#[cfg_attr(docsrs, doc(cfg(…)))]` and a build that actually sets `--cfg docsrs` under `--all-features` (`_design.md`:84-102, `:524-530`) | **AC-004** (the build that composes it) + **AC-005** (the pill observed) | The two are deliberately separate ACs: a green build with the pill missing from the facade satisfies AC-004 and fails AC-005 |
| **Composition and placement** — the pattern is the *module-doc ladder with `doc(cfg)` gate annotations*: one-line summary → `# Status` → `# Using it today` → the rest, gated items shown with their gate (`_design.md`:392-397) | **AC-005**, **AC-006** | Rendered-page read against the named pattern; no new heading is introduced by this story |
| **Transience** — the gate annotation is **persistent chrome on the item**, never revealed on hover, never opened on demand, never relegated to a collapsed section | **AC-005** | It is rustdoc's own always-rendered pill; the verification is that it is present in a static screenshot of the page |
| **Density budget, with its real numbers** — docs.rs lead paragraph ≤ 2 rendered lines, first `#` heading within 12 rendered lines, above-the-fold ≈ 29 lines at 1440×900 and ≈ **22** at 1024×768 (`_design.md`:449-476); the exemption prose spends ≤ 3 of them | **AC-006** | Counted at the rendered page, as a budget checked at the render rather than a figure asserted here (`_design.md`:439-444) |
| **Hierarchy** — at most **one** primary-ranked element per screenful (`_design.md`:520); a gate annotation is an annotation, not a heading, and must not be promoted into one to make it noticeable | **AC-006** | Rendered-page read; AP-15 is the failing form |
| **Named anti-patterns** — **AP-8** (gated item without its annotation, or absent) → **AC-005**; **AP-7** (raw HTML, inline style, colour or glyph carrying meaning) and **AP-3** (a second maturity count on one surface) → **AC-006**; **AP-9** (the four known-stale published strings) → **AC-007**, as a handover rather than an edit | as listed | `_design.md`:641-676, each phrased to be checkable from a screenshot by someone who cannot read the code |

**The reason this section is not optional here.** A rustdoc build that emits every
item, every ARIA landmark and every anchor perfectly — and drops one `doc(cfg)` pill
because the facade's glob re-export did not carry the attribute — passes every
structural check in this repository and publishes a false statement to the primary
surface, permanently. AC-005 is the only assertion in the project that fails on it.

## Error conditions

| id | Condition | Required behaviour |
| --- | --- | --- |
| **EC-001** | One or more of the three `references/evaluation/` evidence documents does not exist when the story opens (both projection siblings were still at `stage: design` at discovery, `discover.md`, signal ledger) | **Halt and report**, naming the missing document and its owning sibling story. Do not write a verdict, do not stage an intake document, do not substitute `RUNBOOK.md`:601. A blocked story is the correct state; a verdict on absent evidence is `discover.md`:40 |
| **EC-002** | The evidence answers some parts of PS-2's bar and is silent on others | The silent part is recorded as **no evidence**, which means the bar is **not met** — a conjunction with an unknown term is not satisfied. Arm A follows, and the atom says *PS-3's SHOULD still binds*. Recording *no evidence* as *met* is the AC-003 failure with a softer face |
| **EC-003** | `unstable-projection` is missing from `crates/happenstance-core/Cargo.toml` or `crates/happenstance/Cargo.toml`, or the `doc(cfg)` attributes are absent, at the B1 preflight | **Halt**, naming the upstream story that should have landed it (`…/projection-store-freeze/unstable-projection-gate-and-clause-disposition/spec.md` or `…/typed-layer-and-alpha-release/projection-trait-and-runner/spec.md`). Inventing the feature here is out of the PR boundary and would make this story a surface change |
| **EC-004** | No nightly toolchain on the machine, so the `docs.rs configuration (nightly)` step probes false and skips | The skip is **reported as a skip**, never as a pass, and AC-004 is not satisfiable from that run. Evidence must come from a machine or CI job with nightly installed — the gate job installs it on all three runners (`xtask/src/main.rs`:611-614) |
| **EC-005** | The `doc(cfg)` pill does not survive `pub use happenstance_core::*;` onto the facade's page | Add an **explicit gated re-export** naming exactly what the glob already re-exports, plus `#![cfg_attr(docsrs, feature(doc_cfg))]` on `crates/happenstance/src/lib.rs`. The reachable set must be unchanged; if AC-007's surface check reports an item added or removed, the remedy was wrong and is reverted |
| **EC-006** | The atom's intended number is taken by a slice-mate inside the same `/redkiln:kb-ingest` wave | The wave assigns; the ledger and this spec cite **the id the wave actually assigned**, not the one planned. Renumbering an already-accepted atom is not available — the correction path is supersession (`.kb/decisions/README.md`) |
| **EC-007** | The nightly build fails under `-D warnings` because `doc_cfg`'s spelling changed on that nightly | Report it and record the nightly version; this is precisely the failure the step exists to find early (`xtask/src/main.rs`:596-602). Do **not** delete the attribute or drop `-D warnings` to get green — that trades a build failure now for a permanently wrong published page |
| **EC-008** | The bar **is** met and the verdict is *freeze* (arm B) | The atom is written, the release **blocks**, and the un-gating is routed to the crate that owns the surface plus a re-plan (`project.md`:123-126). The story never ships an atom saying *frozen* beside a manifest saying `unstable-projection` |
| **EC-009** | `redkiln validate --kb` fails after the wave — a malformed atom, a broken link, or an edit to an accepted atom's body | Fix the **new** atom or the map row. An accepted atom's body is never edited to make validation pass; that failure is itself the signal that the wave did the wrong thing |

## Non-functional

| id | Requirement | Why, and how it is observed |
| --- | --- | --- |
| **NF-001** | **Zero runtime cost, and zero resolution cost in arm A.** The dependency graph, the feature graph and the compiled artefacts a consumer resolves are identical before and after | Observed as AC-007's empty `[features]` diff. It is what lets this story ship inside a release rather than ahead of one |
| **NF-002** | **The gate grows by one `-p`, not by one step.** No new process is spawned, no new tool is probed, and `cargo xtask ci`'s step count is unchanged | `xtask/src/main.rs`'s `STEPS` array gains two `&str` entries inside an existing `Step`; the mandatory/optional split at `:33-38` is untouched |
| **NF-003** | **The `!Send` flavour is not disturbed.** Nothing here adds a bound or changes a default feature, so both port flavours cost identically — the only acceptable answer under ADR-0001 (`_design.md`:587-591) | The four mandatory `wasm32` steps stay green; `AC-014` is `publish-0-2-0`'s to assert on the published tree, and this story's obligation is not to regress it |
| **NF-004** | **The decision is auditable and immutable.** The atom is accepted, checked against `HEAD` by `redkiln validate --kb`, and correctable only by a superseding atom | The mechanism is the KB's, not this story's; the obligation is to use it rather than to hand-edit (`CLAUDE.md`, *Where the work lives*) |
| **NF-005** | **The evidence stays durable and dated.** The verdict cites `references/evaluation/` documents, whose genre is *immutable evidence, dated, pinned to a commit, superseded rather than edited* (`references/evaluation/README.md`:9-13) | Citations are `file:line` into those documents; the atom paraphrases none of them as its own finding |
| **NF-006** | **The rendered check is repeatable by a stranger.** AC-004's command is a single copyable line with its `RUSTDOCFLAGS`, and AC-005's observation names the two `target/doc/**` paths that were looked at | Recorded in the captured run under this story's directory, so `rendered-page-preflight` can re-run it against the published pages rather than re-derive it |

## Implementation notes (non-prescriptive)

- **Do the preflight before the reading.** B1's observation of the shipped state is
  cheap and it is the only thing that distinguishes *the feature is gated* from
  *the feature was never landed*. Two `grep`s over the two manifests and one over
  `crates/happenstance-core/src/lib.rs` settle it.
- **Write the three-row bar table first, then the verdict.** If the table is written
  after the verdict, it becomes a justification. The order is the whole
  anti-pattern defence: the atom's own structure should make the verdict fall out
  of the rows rather than precede them.
- **Expect the third row to be the hard one.** It names a *transport* far end
  (Workers `SqlStorage`, or Neon over one-shot HTTP), and the workspace's most
  likely available third shape is Ladybug, which holds a real write handle. If the
  evidence names Ladybug as the second unlike shape, that is a *no evidence* on row
  three, not a generous *met*.
- **The nightly step is a one-line edit and a one-sentence comment.** Match the
  existing sentence's form for `happenstance-testkit` (`xtask/src/main.rs`:617-620)
  rather than inventing a new rationale; the rule was already stated there and this
  is its third application. The module doc at `:8-24` names steps, not crate lists,
  so it does not change.
- **Render locally and look, before trusting the build.** `-D warnings` proves the
  attributes compile; it proves nothing about whether the pill appears on the
  facade's page. Open `target/doc/happenstance/index.html` and the item page.
- **Stage the intake document in the shape `crate-set-decision` uses**, so the
  slice's single wave sees four consistent inputs. Include: the verdict, the
  three-row bar table with its citations, the losing arm and its cost, the PS-3
  falsifier observation for routing, and `source_paths` naming the three evidence
  documents. Then stop and hand off — `/redkiln:kb-ingest` is user-invoked.
- **If the verdict falsifies a published sentence, write the finding down where the
  owning story will find it** — the implementation report and the handover, not a
  quiet edit to `crates/happenstance/README.md`.

## Tests and CI (merge gate)

Grounded in the testing brief (`_decomposition.md`:456-499 for the AC table,
`:519-534` for the merge-gate commands). **This story adds no compiled test**, and
that is a deliberate consequence of what it is: AC-012's instrument is *static,
citation check* and AC-013's is *process*. What it does add is one crate to an
existing nightly step and one rendered-page observation that nothing in the tree
performed for the facade before.

| Tier | Command / path | Proves |
| --- | --- | --- |
| **static (citation)** | Review of `.kb/_intake/<nnnn>-…md` and the resulting atom against `spec/SPECIFICATION.md`:4760-4775 | AC-001 — three bar parts answered, each citing a `references/evaluation/` document; no claim resting on `RUNBOOK.md`:601 |
| **process (KB)** | `redkiln validate --kb && redkiln doctor` | AC-002 — KbFrontmatter conformance, accepted-atom immutability against `HEAD`, six `template-drift` advisories and zero `dependency-cycle`; AC-003's *no accepted atom edited* |
| **static (diff)** | `git diff --stat main...HEAD` and `cargo xtask spec-trace` | AC-003 — `spec/SPECIFICATION.md` unmodified, no hunk inside `RUNBOOK.md`:588-610, clause citations still resolve |
| **build (nightly, probed)** | `RUSTDOCFLAGS="--cfg docsrs -D warnings" cargo +nightly doc --locked -p happenstance-core -p happenstance -p happenstance-testkit --all-features --no-deps` — i.e. `xtask/src/main.rs`'s `docs.rs configuration (nightly)` step after this story's edit | AC-004 — the `doc(cfg)` spelling compiles for all three publishable crates, in the one place it can still be fixed |
| **human observation (rendered)** | `target/doc/happenstance/**` and `target/doc/happenstance_core/**` from that build, read and recorded with a date | AC-005 — every gated item present **with** its pill on both surfaces; AC-006 — the exemption prose, its density and its hierarchy. `xtask/src/package.rs`:4-18 is explicit that this cannot be replaced by an automated check |
| **feature matrix** | `cargo hack --feature-powerset` host and `wasm32` steps (`xtask/src/main.rs`:546-556, `:564-591`) | AC-007 — every combination still compiles, including `--no-default-features --features unstable-projection`; neither step skipped, since `cargo-hack` resolves here |
| **story grain (automatic)** | `cargo xtask affected --base {{base}}` (`.redkiln/config.yaml`:40) | The redkiln story gate: fmt, clippy `-D warnings` and tests for the affected packages, plus the five file-reading lints and `spec-trace` unconditionally — which is what gates a story whose diff maps to almost no package |
| **integration grain (automatic)** | `cargo xtask ci --fast` (`.redkiln/config.yaml`:55) | The project's non-terminal bar. **It does not cover AC-004**: `--fast` drops the two feature powersets, `cargo deny` and the nightly `--cfg docsrs` build by design. Named here so nobody offers a green `--fast` as this story's evidence |
| **deferred to the release event** | Full `cargo xtask ci` on the publish commit; the rendered *published* pages | AC-016 and AC-007 at project grain — `publish-0-2-0` and `rendered-page-preflight` own these; this story's job is to make sure the docs.rs step they run already names all three crates |

**No conformance rule is added, and the reason is the second of the two permitted
ones**: this is not adapter-observable. No rule in `crates/happenstance-testkit/src/suite.rs`
can see whether a port ships behind a feature — a rule that could would be observing
the manifest, not the store (`CLAUDE.md`, the conformance-suite corollaries).

## Risks and coupling (PR-scoped)

| Risk | Likelihood / impact | Mitigation, inside this PR |
| --- | --- | --- |
| **The evidence is not there yet.** Both projection siblings were `stage: design` at this story's discovery pass, and their three `references/evaluation/` documents do not exist in the tree today | Likely / blocking | EC-001: halt and name the missing document. This is the story's expected blocked state, not a defect. The anchors below point at the sibling **specs** that produce them, which do exist, so the citation format can be settled before the evidence lands |
| **The frozen bar gets read generously** because the third part is the one the workspace is least likely to satisfy, and arm A is available either way | Medium / severe and quiet | AC-001's three-row structure and AC-003's stop; the *Rejects* field of PS-2 (`spec/SPECIFICATION.md`:4770-4775) is a standing warning against counting two handle-holding stores as two ends of one axis |
| **This story's own central check is not run by the project's own integration gate.** `cargo xtask ci --fast` drops the nightly `--cfg docsrs` build | Certain / moderate | Named in the Tests table and in AC-004's verification: the evidence must be a captured run of the full step, and `publish-0-2-0`'s full `cargo xtask ci` is the second net |
| **`doc(cfg)` may not survive the facade's glob re-export**, which is the exact defect this story exists to detect and the one nobody has looked for | Medium / severe | EC-005's explicit gated re-export, bounded by AC-007's requirement that the reachable set does not change. It is a presentation remedy, not a surface change |
| **Number collision inside the slice's single ingest wave** — four stories, four atoms, one wave | Medium / low | B8 plus EC-006: the wave assigns and the ledger cites what it assigned; `crate-set-decision` is the blocker precisely so the allocation is stated once |
| **The verdict falsifies published copy** — `crates/happenstance/README.md`:11 and the manifest `description` both currently claim projection runners | Medium / moderate | AC-007's handover list. Fixing it here would cross into `landing-copy-and-status-truth`'s and `guarantees-and-docs-rs-presentation`'s boundary and put two stories in one file |
| **Arm B looks like a small edit.** Deleting a feature and un-gating a module is a two-line change and a published-surface break | Low / severe | EC-008 and `project.md`:123-126: recorded, then blocked, then re-planned |
| **Coupling upward:** `landing-copy-and-status-truth` and `guarantees-and-docs-rs-presentation` both wait on this verdict, and the latter also owns `[package.metadata.docs.rs]` for `happenstance` | Certain / by design | Deliberately separable: the nightly gate step passes `--all-features` and `RUSTDOCFLAGS` itself, so this story's check runs without that manifest block and that block ships without waiting on this story |

## Dependencies

**Blocks on**

- **`crate-set-decision`** *(story, this slice, first in merge order)* — the ship
  shape is a property of a crate in the decided set, and it is the story that states
  the slice's ADR-number allocation this atom draws from
  (`_storymap.md`:156-160, `project.md`:207-212).

**Also required before implementation can start, at project grain rather than story
grain** — these are not `depends_on` entries because they are other projects, and
`project.md`'s Dependencies section already carries them:

- **`projection-store-freeze`** — produces `references/evaluation/projection-batch-shape-evidence.md`.
- **`ladybug-projection-store`** — produces `references/evaluation/phase-11-freeze-verdict.md`
  and `references/evaluation/phase-11-publication-handover.md`.

**Unlocks**

- **`landing-copy-and-status-truth`** — needs the verdict before it can write status
  copy that is true of the tree that shipped.
- **`guarantees-and-docs-rs-presentation`** — needs it before the Guarantees block
  can state what depending on the projection surface costs, and it lands
  `[package.metadata.docs.rs]` for `happenstance` on top of the `doc(cfg)` treatment
  this story compiles.

**Not a dependency, deliberately:** `falsifier-ledger-repair` owns `RUNBOOK.md`'s
provisional ledger table. This story does not read or write that table (AC-003), so
the two are unordered.

## Anchors (progressive disclosure)

Open these **when the bound AC says to**, not before. Everything the Context pack
states is already sufficient to start; these carry the depth a summary cannot hold.

| Anchor | Why it is load-bearing | When to open | Serves |
| --- | --- | --- | --- |
| `spec/SPECIFICATION.md` (PS-2 `:4760-4775`, PS-3 `:4776-4790`) | The frozen bar's exact three conjuncts and its *Rejects* field — the text AC-001 evaluates and AC-003 forbids amending. Reading a paraphrase is how the third part gets softened | **Before writing the three-row bar table**, first thing after the preflight | AC-001, AC-003 |
| `.bklg/from-contract-to-published-library/projection-store-freeze/ps3-batch-shape-finding/spec.md` | Defines what `references/evaluation/projection-batch-shape-evidence.md` will contain, including that the *they agreed everywhere* null result is a finding about two **testkit** shapes and explicitly not a verdict (`:26-45`) | **When citing batch-shape evidence** — and to check whether the document has landed at all | AC-001 |
| `.bklg/from-contract-to-published-library/ladybug-projection-store/freeze-verdict-document/spec.md` | Defines the dated *held / did not hold* verdict document's contents: the implementation, the rules run, the PS clause ids and the commit (`:36-40`). It is what makes a `file:line` citation possible | **When answering bar rows two and three** | AC-001 |
| `.bklg/from-contract-to-published-library/ladybug-projection-store/verdict-ordering-and-publication-handoff/spec.md` | The ordering claim and the three residues phase 11 hands this project — one of which is stated as a question and **must stay one** (`:38-48`). Converting a handed-over question into an answer is a silent scope grab | **Before writing the atom**, to check nothing open is being closed by accident | AC-001, AC-003 |
| `standards/rust/51-features-and-no-std.md` (RS-51-5, `:188-236`) | The mechanism and the argument: the failure is invisible everywhere except docs.rs, *after* the act that cannot be undone; a yank removes the version and leaves the page. Also the exact attribute spellings | **Before editing the nightly step and before reading the rendered page** | AC-004, AC-005 |
| `xtask/src/main.rs` (`:596-638`, comment `:617-620`, mandatory/optional split `:33-38`) | The step being edited, and the crate-joins-the-list rule already stated in its own words for `happenstance-testkit`. Copy the form; do not invent a rationale | **At the moment of the edit** | AC-004 |
| `.bklg/from-contract-to-published-library/publication-and-positioning/_design.md` (`:84-102`, `:392-397`, `:439-476`, `:520`, `:566-579`, `:641-676`) | The **binding** signed-off design: the attribute treatment, the module-doc ladder pattern, the density numbers, the one-primary rule, the exemption's placement, and the fifteen anti-patterns phrased to be checked from a screenshot | **Before rendering and reading the page** — AC-005 and AC-006 are graded against these | AC-005, AC-006 |
| `.bklg/from-contract-to-published-library/publication-and-positioning/_decomposition.md` (UX brief IQ-1 `:232-240`, IQ-2 `:241-258`, IQ-3 `:260-271`, IQ-4 `:272-284`, IQ-7 `:305-311`; testing AC table `:475`; deployment `:615-641`, `:756-759`) | The interaction-quality invariants in full, the AC-012 instrument with its named wrong implementation, and AC-DEP-002's second sentence | **When writing the verification column into the ledger**, and before the rendered read | AC-001, AC-005, AC-006, AC-007 |
| `.kb/decisions/0029-msrv-raised-to-1-97-1.md` (`:1-33`) | The shape of record for a phase-scoped decision atom: frontmatter fields, how the losing option is named, how a source path is cited | **While drafting the intake document** | AC-002 |
| `.kb/_intake/README.md` (`:3-5`, `:13-19`) | How a wave consumes staging: what is dropped at the approval gate, that a successful wave clears the directory, and the id-suffix discipline when a second wave follows | **Before staging the intake file**, so the slice's four inputs are consistent | AC-002 |
| `.kb/maps/decision-map.md` (`:81-86`) | The second mount. An atom with no map row is an atom the corpus cannot see; this is the row's required shape | **Immediately after the wave runs**, to confirm the row exists | AC-002 |
| `.kb/open-questions/es-7-and-vt-9-provisional-markers.md` | Records the pattern PS-3's falsifier now matches — *a marker that has quietly become decoration*. It is the routing target for the observation, and it is an atom you must **not** hand-edit | **When writing the falsifier observation into the intake document** | AC-003 |
| `crates/happenstance/src/lib.rs` (`:10`, `:73`, `:76`) | The facade's crate-root ladder, the existing `#![doc(html_no_source)]` choice that must not be flipped in passing, and the glob re-export whose `doc(cfg)` behaviour EC-005 turns on | **When the pill is missing from the facade's page** | AC-005 |
| `references/evaluation/README.md` (`:9-13`) | The genre contract for the evidence being cited: immutable, dated, commit-pinned, superseded rather than edited — which is why the atom cites it instead of restating it | **Before paraphrasing anything from an evidence document** | AC-001, NF-005 |
| `.bklg/from-contract-to-published-library/publication-and-positioning/projection-port-ship-shape/discover.md` (`:30-32`, `:40`) | The three deferred questions this spec answers and the named wrong implementation, in the form the story's own gate recorded them | **At the start**, once, to confirm nothing deferred was dropped | AC-001, AC-002 |
| `.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md` (`:249-313`, `:333-338`) | Persona 4's evidence base: one sitting, cannot run the suite, judges on what the project chose to publish. It is why AC-005 and AC-006 are graded on the rendered page rather than the source | **When judging whether the rendered page answers the reader**, not before | AC-005, AC-006 |
| `.bklg/from-contract-to-published-library/publication-and-positioning/crate-set-decision/spec.md` | The blocking story: the decided crate set and the slice's ADR-number allocation this atom draws its number from | **Before choosing the atom's number** | AC-002 |

## Clarifications resolved during spec

1. **`discover.md`:32's deferred question — does `happenstance-core` gain the
   feature, or does `happenstance` re-export it too? — is answered mechanically and
   is not this story's decision.** Both crates already carry `unstable-projection`
   by the time this story opens: `happenstance-core` from phase 6, `happenstance`
   with its own forwarding feature from phase 7. This story therefore treats the
   feature graph as **given** and out of its boundary (AC-007), which is why the
   PR boundary forbids adding or renaming a feature and EC-003 makes an absence a
   halt rather than a fix.
2. **The AC set is exactly the seven ids the first pass decided** — AC-001 … AC-007.
   None was added or dropped. What the second pass fixed is the *split* between
   AC-004 and AC-005: compiling the `doc(cfg)` spelling and observing the rendered
   pill are two different failures with two different detectors, and folding them
   into one row would let a green nightly build stand in for a page nobody looked at
   — which is the exact substitution `xtask/src/package.rs`:4-18 already forbids for
   containment versus presentation.
3. **AC-012's discharge is AC-001, not the atom.** The atom (AC-002) is AC-013's.
   Reading AC-012 as *an atom exists* would let a well-formed, uncited verdict
   satisfy it, which is the testing brief's named wrong implementation.
4. **The nightly `--cfg docsrs` step stays *optional* (probed), and this story does
   not promote it to mandatory.** AC-DEP-005's mandatory-list requirement is written
   for the two *new* instruments — the surface diff and the clause audit — which have
   no external tool to probe for. This step probes for a nightly toolchain, which is
   a real absence on a contributor's machine, and `xtask/src/main.rs`:33-38 draws the
   line on exactly that basis. The consequence — that `cargo xtask ci --fast` does
   not cover AC-004 — is stated in the Tests table rather than fixed by moving the
   step, because moving it would mean every contributor needs a second toolchain to
   run the gate.
5. **"Human observation" here means the *locally rendered* pages, not docs.rs.**
   docs.rs does not render anything until after publication, and IQ-4 puts the whole
   of this project's reversibility before the act. AC-005 and AC-006 are graded on
   `target/doc/**` from AC-004's build; `rendered-page-preflight` and
   `stranger-install-smoke` are the stories that read the published pages.
6. **PS-3's falsifier is recorded as defective and left alone.** It reads *falsified
   the moment PS-2's bar is met before 0.1*, and 0.1 was never published — an event
   that can no longer occur. Repairing it would move a provisional marker's text,
   which changes §1.3's census inputs and the ledger `clause-maturity-audit` reads.
   The observation goes into the intake document for the ingest wave to route
   (AC-003); it is not this story's to fix, and it is not silently dropped either.
7. **The three evidence documents do not exist in the tree at spec time**, so no
   anchor cites them by `file:line`. The anchors table points at the three sibling
   **specs** that define their contents, which do exist and are stable; the atom's
   citations resolve into the evidence documents themselves once they land, and
   EC-001 makes their absence a halt rather than an improvisation.
