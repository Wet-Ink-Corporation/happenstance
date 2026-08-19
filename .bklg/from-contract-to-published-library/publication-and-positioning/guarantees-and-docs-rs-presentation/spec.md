---
item: HS-S0094
stage: spec
created: 2026-08-12T13:47:32.202Z
updated: 2026-08-12T13:47:32.202Z
template_sig: 87bbf1d0
rendered_sig: 3dda5530
---

# Spec — What depending costs, stated where a consumer reads it, on a page where nothing is invisible

## Scope lock

| | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` — BR-08, BR-12, BR-17; DoD 9, DoD 10; DT-1's demoted edge line |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` — the DAG and this project's rank |
| Project | `.bklg/from-contract-to-published-library/publication-and-positioning/project.md` — AC-006, AC-007, AC-008, AC-014; DR-6, DR-7, DR-14 |
| This spec | `.bklg/from-contract-to-published-library/publication-and-positioning/guarantees-and-docs-rs-presentation/spec.md` |
| Key briefs | `.bklg/from-contract-to-published-library/publication-and-positioning/_decomposition.md` — **ux** brief (U5, U6, IQ-1…IQ-7, AC-UX-007/009/010/012), **testing** brief (the AC-007 / AC-008 rows of the test-mix table), **deployment** brief (feature gating, the wasm32 posture, the rollback posture) |
| Signed-off design (**binding**) | `.bklg/from-contract-to-published-library/publication-and-positioning/_design.md` — `## Items`, `## Signatures`, `## Surfaces`, `## Composition` R7/R9, `## Density budget`, `## Transience policy`, `## Anti-patterns`, `## The doctest`, `## Sign-off` |
| Story map | `.bklg/from-contract-to-published-library/publication-and-positioning/_storymap.md`:66 (this row), :89-90 (why the slice is one page), :186-191 (where the release blocks) |
| Roadmap pointer | `RUNBOOK.md`:163 — phase 12's row, whose proof artefact is *"docs.rs green under `--all-features` and the `docsrs` cfg"* |

## One-line PR slice

State the cost of depending in `crates/happenstance/README.md`:41-49's Guarantees slot — the MSRV as a
promise linking the new atom, minor-bump-is-breaking under 0.x, and which feature set a `wasm32` reader
takes — add the missing `[package.metadata.docs.rs]` block to `crates/happenstance/Cargo.toml` so all three
crates build docs under all features with `doc(cfg)` on every gated item, and make the quick-start snippet
the crate's own doctest via `crates/happenstance/src/lib.rs`:10 so it is the same text the stranger-install
smoke runs.

## Executive summary

This PR lands the half of the published surface that answers *"what does depending on you cost me, and
where do I stop being supported?"* — U6 and U5 of the UX brief — plus the two mechanical facts that
decide whether the docs.rs page an evaluator opens is complete or quietly missing things.

Four deltas against the tree as it stands, each verified rather than assumed:

1. **The Guarantees block on `crates/happenstance/README.md`:41-49 carries three bullets today**, and its
   MSRV bullet links `ADR-0029` — an accepted, immutable atom that records a *measurement*, not a promise.
   It is rewritten in place to state the floor as a promise, what a bump costs a consumer, and that under
   0.x the minor bump is the breaking-change signal, linking the **new** atom `msrv-promise-atom` authors.
2. **Neither packaged README says the word `wasm32`, `Send` or "flavour" anywhere** — verified by grep
   over `crates/happenstance/README.md` and `crates/happenstance-core/README.md`, which return nothing.
   The two-flavours copy exists only at `README.md`:159-164, on the repo root, which is *not* what ships
   in the `.crate`. One line moves to the packaged surface so a constrained-runtime reader can
   self-identify without reading source.
3. **`crates/happenstance/Cargo.toml` has no `[package.metadata.docs.rs]` block**, while
   `crates/happenstance-core/Cargo.toml`:54-56 and `crates/happenstance-testkit/Cargo.toml`:56-58 both do.
   The crate a `cargo add` evaluator actually meets is the one whose docs.rs build is unconfigured. And
   the gate's own nightly `--cfg docsrs` step names only two crates
   (`xtask/src/main.rs`:621-635) — so nothing in this repository has ever compiled `happenstance`'s
   documentation the way docs.rs will.
4. **The quick-start fence at `crates/happenstance/README.md`:30-39 is a read-only `count_everything`**,
   nine lines that never write an event. `_design.md`'s `## The doctest` replaces it with the 19-line
   write-then-read cycle that `stranger-install-smoke` must run against the registry version — one text,
   three places.

This story does **not** decide PS-3, does not touch a `[FROZEN]` clause, does not write the compliance
block or the positions-and-gaps bullet (both are the slice-mate's), and does not run the release.

## Context pack

The load-bearing decisions, stated as decisions. Everything here is already settled somewhere else; this
section is what an implementer must hold in their head before touching a file.

**C1 — The Guarantees slot is the designated site, and it is nearly full.** `_design.md`'s
`## Density budget` budgets it at **≤ 7 bullets, each ≤ 3 rendered lines, exactly one link per bullet**,
and `## What it costs a caller` says plainly that this project spends four of them. Three bullets exist
today (`crates/happenstance/README.md`:41-49). Therefore: the MSRV bullet is **rewritten in place, not
appended to** — a second MSRV bullet is both a budget overrun and two sentences that can disagree. The
DT-4 positions-and-gaps bullet is *not* yours; it is `compliance-claim-and-gaps-promise`'s, landing in the
same list in the same slice.

**C2 — The MSRV link points at the new atom, and editing either existing atom is forbidden.**
`.kb/decisions/0004-edition-and-msrv.md` and `.kb/decisions/0029-msrv-raised-to-1-97-1.md` are both
`status: accepted` and therefore immutable; `redkiln validate --kb` checks accepted atoms against `HEAD`
and fails on exactly this edit (project `DR-6`). The atom you link is the one `msrv-promise-atom` authors
at **0030 or above** — its number is not knowable from this spec, so take the path from that story's
own report before writing the URL, and do not guess a number. On a packaged surface the link is an
**absolute** GitHub URL, never repo-relative (`_design.md` `## Placement and re-export`; anti-pattern
AP-6).

**C3 — What the promise says has three parts, and the third is the one nobody has written down for a
consumer.** `_design.md` `## Visibility and stability`: the floor as a promise (1.97.1), what an MSRV bump
means to a consumer, and that **under 0.x the minor bump is the breaking-change signal**. The third part
is the semver contract for the whole release and it currently lives only in project prose. AC-UX-010 is
explicit that it must be *stated where a consumer reads it*, which is this block.

**C4 — The `wasm32` line is DT-1's loser, deliberately demoted to exactly one line.** `_design.md`'s DT-1
resolution rejected leading with the edge story on an asymmetry: *"the constrained-runtime reader can
self-identify from one line… whereas the general evaluator cannot self-identify from an edge lead at
all."* So this is one Guarantees bullet — supported, and which feature set to take — not a section, not a
callout, not the lead. Its content comes from `README.md`:159-164, and it must be true of the
**published** tree and the **published** feature set. Proving that is `publish-0-2-0`'s four mandatory
`wasm32` steps (project AC-014); your obligation is that the sentence does not claim more than those
steps assert.

**C5 — Nothing gated may be invisible, and the manifest is what decides it.** IQ-2's non-occlusion
invariant, sharpened by `_design.md` AP-8: a feature-gated public item that renders **without** its
`doc(cfg)` annotation, or does not render at all, is a *false* claim of "not supported" — worse than a
missing page, because it reads as an answer. The fix is mechanical and byte-identical to the two manifests
that already have it (RS-51-5, `standards/rust/51-features-and-no-std.md`:188-236):
`all-features = true` plus `rustdoc-args = ["--cfg", "docsrs"]`.

**C6 — The gate does not currently compile this crate's docs the way docs.rs will, and its own comment
says what to do about it.** The nightly step at `xtask/src/main.rs`:621-635 names `happenstance-core` and
`happenstance-testkit` only, and its comment states the joining rule in terms: testkit *"joined the list
when it acquired `#![cfg_attr(docsrs, feature(doc_cfg))]`… it is one of the three publishable crates, so
its rendering fails in the same yankable-but-not-removable place."* `happenstance` is the third. Add
`-p happenstance` in the same change that adds the manifest block. Two consequences an implementer will
otherwise trip on: that step is **optional** — probed for nightly, and `--fast` drops it entirely
(`.redkiln/config.yaml`, `xtask/src/main.rs`:45-52) — so it must be run and recorded deliberately here,
not left to the story-grain gate; and it runs with `RUSTDOCFLAGS="--cfg docsrs -D warnings"`, so a warning
is a failure.

**C7 — The facade is a glob re-export, and un-globbing it is out of bounds.**
`crates/happenstance/src/lib.rs`:76 is `pub use happenstance_core::*;` and `_design.md`
`## Placement and re-export` says it is unchanged and that `#![doc(html_no_source)]` (`:73`) must not be
flipped in passing. If the `--cfg docsrs` render shows a gated re-export arriving on the facade's page
without its gate pill, that is a **finding to record, not a thing to fix by naming re-exports
individually** — it is an API-shape change, and this project carries no `architecture` brief precisely so
that surface changes go back to the owning crate and a re-plan (`project.md`, *Out of scope*).

**C8 — PS-3 is decided elsewhere; you inherit whichever arm it took.** `projection-port-ship-shape` (a
declared dependency) either freezes the port or ships it behind `unstable-projection` with RS-51-5's
`doc(cfg)` treatment. `_design.md` `## Shape decision` binds only the *presentation*, both ways. If the
gated arm was taken, `happenstance` forwards the feature like every other one it has
(`crates/happenstance/Cargo.toml`:22-26 — the facade defines no feature of its own, so the two crates
cannot disagree about `default-features = false`) and the item renders **with** its pill under
`all-features`. Read that story's report; do not re-derive the verdict, and do not add the flag if it
froze.

**C9 — The quick start is one text in three places, and that identity is the point.** AC-UX-012:
the fence on `crates/happenstance/README.md` is compiled as this crate's own doctest by
`#![cfg_attr(doctest, doc = include_str!("../README.md"))]` (`crates/happenstance/src/lib.rs`:10 — already
wired, do not re-plumb it), and it is the program `stranger-install-smoke` runs against the registry
version. `_design.md` `## The doctest` gives the exact 19-line text; it is a write-then-read cycle because
that is what DoD 9 requires a stranger to be able to run, and its one comment carries DT-4's gaps promise
where a copier will see it. Budget: **≤ 20 source lines** (AP-12 — a fence that scrolls inside itself at
1440×900 fails), fenced `rust` and never `rust,ignore` (RS-62-5's out-of-package pattern exists so a
packaged example need not be ignored, `standards/rust/62-doctests-and-harnesses.md`:228-236).

**C10 — The first screen is full, so everything you write is below the fold, on purpose.** The sign-off
(`_design.md` `## Sign-off`) accepted a measured **≈343 px against a 340 px budget** at 1024×768: any sixth
first-screen region, or a callout growing back to three rendered lines, makes AP-1 fail. R7 (quick start)
and R9 (Guarantees) are both *revealed on scroll* in `## Transience policy`, and a scroll is not a hop
under IQ-1. Nothing this story writes may be promoted into the first screen to make it more visible.

**C11 — The persona slice.** U5 (*"tell me whether it runs where I run"*) and U6 (*"tell me what depending
on you costs me, and when that cost can change"*), read by the evaluator — who `_design.md`'s DT-1
resolution settles is **the first fifteen minutes of the application author's journey, not a fifth
persona**: time-boxed, one sitting, cannot run the suite. Budget: **0 hops to read a claim, ≤ 1 hop to its
evidence, and that hop lands on a specific anchor** (IQ-1). A Guarantees bullet whose answer is *"see the
specification"* fails this story.

**C12 — Truth at the publish commit, and the version string is a promise like any other.** IQ-7: no
sentence on a published surface may be false on the tree that shipped, and a published page cannot be
edited afterwards — `cargo yank` removes a version from resolution and leaves every rendered page exactly
as it was (`standards/rust/51-features-and-no-std.md`:226-231). The version in the `toml` fence follows
what `registry-surface-diff` reports; `0.2.0` is the ceiling, and a report implying something outside it
is a re-plan rather than a rounded-down number (`_storymap.md`:186-191). Every sentence here is re-read on
the rendered page by `rendered-page-preflight` before the irreversible act.

## Integration contract

- **Archetype**: `capability` — a user-observable slice of the published surface, mounted on the page an
  evaluator actually lands on.
- **Slice / milestone**: `published-surface-copy`. Slice-mates, implemented in the same context:
  `landing-copy-and-status-truth` (R1–R3, the four stale strings, the status table) and
  `compliance-claim-and-gaps-promise` (R6's claim-with-evidence block, and the DT-4 bullet that lands in
  *your* Guarantees list). The slice is cut this way because **it is one page**: IQ-1's hop budget can only
  be held by whoever sees the whole surface at once (`_storymap.md`:88-90).
- **Mount point**: `crates/happenstance/README.md` — the render path of the primary surface
  `crates-io-happenstance`, packaged by `readme = "README.md"` (`crates/happenstance/Cargo.toml`:12),
  asserted contained by `xtask/src/package.rs`'s `REQUIRED_FILES` (`:94`), and compiled into the crate as a
  doctest by `crates/happenstance/src/lib.rs`:10. Copy written only at the repo root is invisible to a
  registry reader (`README.md`:131-137).
- **Co-mounts this PR must also wire** (not scope drift — they are the composition root for this
  capability):
  - `crates/happenstance/Cargo.toml` — the `[package.metadata.docs.rs]` block, which is the single largest
    determinant of what `docs-rs-happenstance` renders (`_design.md` `## Items`).
  - `xtask/src/main.rs`:621-635 — `-p happenstance` added to the nightly `--cfg docsrs` step, per its own
    stated joining rule.
  - `crates/happenstance/src/lib.rs` — the module doc **is** the `docs-rs-happenstance` surface
    (`_design.md` `## Surfaces`, `selector`), so the ladder order lives there.
  - `crates/happenstance-core/README.md`:43-54 — one bullet only, so the two packaged surfaces do not
    disagree about whether the MSRV is a promise.
- **Wires into**: the decision atom `msrv-promise-atom` authors under `.kb/decisions/` (link target, number
  taken from that story's report — C2); `projection-port-ship-shape`'s PS-3 verdict (which arm renders —
  C8); `crates/happenstance-core/src/lib.rs`'s existing `#![cfg_attr(docsrs, feature(doc_cfg))]` (`:85`) and
  `doc(cfg)` items (`:102`, `:121`) as the reference treatment; `stranger-install-smoke`, which consumes the
  fence text verbatim; `rendered-page-preflight`, which reads the resulting page.
- **Renders surfaces** (ids from `_design.md` `## Surfaces`): `crates-io-happenstance` (regions **R7**
  quick start and **R9** Guarantees), `docs-rs-happenstance` (the whole module-doc ladder and its
  `doc(cfg)` treatment), `crates-io-happenstance-core` (R9, the MSRV bullet only). Verified-not-changed:
  `docs-rs-happenstance-core`, `docs-rs-happenstance-testkit`, `crates-io-happenstance-testkit`,
  `github-landing`.
- **Public items** (`_design.md` `## Items`): `crates/happenstance/Cargo.toml [package.metadata.docs.rs]`
  — **claimed and implemented by this story**. `happenstance/unstable-projection` — **not claimed**: it is
  conditional on PS-3 and owned by `projection-port-ship-shape`; this story verifies its *presentation*
  (present with its pill) and adds the forwarding entry only if that story landed the flag.
- **Conformance rule(s)**: **none, and it is not adapter-observable.** This story changes no port, no value
  type and no testkit rule, so there is nothing an adapter could implement wrongly — adding a rule here
  would be decorative by `CLAUDE.md`'s own test. Its observers are instead: the doctest harness (the fence
  compiles or the gate is red), the nightly `--cfg docsrs` rustdoc build with `-D warnings`, `cargo doc`
  under all features, and the human read at `rendered-page-preflight`.
- **Clause(s)**: **none discharged, none amended.** Nothing in this diff touches `spec/SPECIFICATION.md`;
  project AC-015 therefore holds trivially over this story's changes, and no `[FROZEN]` clause is in scope.
  PS-3 is a clause this story deliberately leaves alone (C8).
- **Advances DoD scenario**: initiative **DoD 10** — *"the published crate looks finished: the rendered
  documentation build is green under all features, and the registry page carries licence, description and
  README as rendered"* — which is this story's primary target. It also supplies the text **DoD 9**
  (*"a stranger can install it"*) is proven with, and contributes the crate that **DoD 13**'s nightly
  docsrs step was not previously building.

## PR boundary

**In this PR**

- Rewrite the MSRV bullet in `crates/happenstance/README.md`'s Guarantees block into the three-part
  promise, linking the new atom (C1–C3).
- Add the one-line `wasm32` self-identification bullet to the same block (C4).
- Mirror the MSRV bullet's link target onto `crates/happenstance-core/README.md`:43-54 — one bullet, no new
  bullets, no new sections.
- Add `[package.metadata.docs.rs]` to `crates/happenstance/Cargo.toml`, byte-identical to
  `crates/happenstance-core/Cargo.toml`:54-56 (C5).
- Add `-p happenstance` to the nightly `--cfg docsrs` step in `xtask/src/main.rs` (C6), and run that step.
- Replace the quick-start fence with `_design.md` `## The doctest`'s text verbatim, with the accompanying
  `toml` fence naming the published version and default features (C9), and confirm it compiles through
  `crates/happenstance/src/lib.rs`:10's existing `include_str!` wiring.
- Bring `crates/happenstance/src/lib.rs`'s module doc to the ladder order `_design.md` `## Composition`
  specifies for `docs-rs-*`, and add `#![cfg_attr(docsrs, feature(doc_cfg))]` **only if** an item on this
  crate's own page needs it (C7).
- This story's own backlog folder (ledger, report).

**Explicitly not in this PR**

- The status callout, the identity line, the disambiguation triad, the status table and the four stale
  strings → `landing-copy-and-status-truth`.
- The compliance claim block (R6) and the DT-4 positions-and-gaps Guarantees bullet →
  `compliance-claim-and-gaps-promise`. Your list has a hole reserved for it; do not fill it.
- Deciding PS-3, or adding `unstable-projection` to `crates/happenstance-core` →
  `projection-port-ship-shape`.
- Authoring the MSRV decision atom, or editing `.kb/decisions/0004-edition-and-msrv.md` or
  `0029-msrv-raised-to-1-97-1.md` → `msrv-promise-atom`, and forbidden respectively.
- Reading the rendered pages, publishing, or running the stranger install → `rendered-page-preflight`,
  `publish-0-2-0`, `stranger-install-smoke`.
- Any change to `pub use happenstance_core::*;`, `#![doc(html_no_source)]`, or any public signature.
- `spec/SPECIFICATION.md`, `RUNBOOK.md`, and the two new gate instruments in
  `publish-time-gate-instruments`.

```
crates/happenstance/README.md
crates/happenstance/Cargo.toml
crates/happenstance/src/lib.rs
crates/happenstance-core/README.md
xtask/src/main.rs
.bklg/from-contract-to-published-library/publication-and-positioning/guarantees-and-docs-rs-presentation/**
```

**Merge DoD**: the packaged `happenstance` page states the MSRV as a promise, the 0.x breaking-change
signal and the `wasm32` answer inside a Guarantees block still within its 7-bullet / 1-link budget; the
quick-start fence is the design's doctest text and compiles as this crate's own doctest; and
`cargo doc --all-features` plus the nightly `--cfg docsrs -D warnings` build — now including
`happenstance` — are both green with every gated item rendering *with* its gate.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| The MSRV reads as a promise, not a measurement (**AC-001**) | The Guarantees bullet states the 1.97.1 floor as a promise and what a bump costs a consumer under 0.x; its single link is the new decision atom, whose number is taken from `msrv-promise-atom`'s report rather than guessed. ADR-0029 stops being the link because it records the *trade*, not the promise | `crates/happenstance/README.md`:41-49 · `.kb/decisions/0004-edition-and-msrv.md`:83-86 · `.kb/decisions/0029-msrv-raised-to-1-97-1.md`:94-95 · `_design.md` `## Visibility and stability` |
| The 0.x breaking-change signal is stated on the surface (**AC-002**) | *Under 0.x the minor bump is the breaking-change boundary* appears in the Guarantees block, in the caller's register — it is the semver contract for the whole release and today it exists only in backlog prose | `project.md`, *Out of scope* (*"under 0.x the minor bump **is** the breaking-change boundary and that is the promise being made"*) · `_decomposition.md` AC-UX-010 |
| A `wasm32` reader self-identifies from one line (**AC-003**) | One bullet: supported, and which feature set to take. Sourced from `README.md`:159-164, carried to the **packaged** surface, and claiming no more than `publish-0-2-0`'s four mandatory wasm32 steps assert. Neither packaged README mentions it today | `README.md`:159-164 · `_decomposition.md` AC-UX-009 · `xtask/src/main.rs`:203-289 (the four wasm32 steps) · `.kb/decisions/0001-async-port-flavours.md` |
| The Guarantees block stays inside its budget (**AC-004**) | ≤ 7 bullets, each ≤ 3 rendered lines, **exactly one link per bullet**, every link absolute; the MSRV bullet is rewritten in place; one slot is left for the slice-mate's DT-4 bullet. No nested bullets except the DT-4 non-ownership line, which is not yours | `_design.md` `## Density budget`, `## Hierarchy` (R9), AP-6 · `crates/happenstance/README.md`:41-49 |
| docs.rs is configured for the crate an evaluator installs (**AC-005**) | `[package.metadata.docs.rs]` with `all-features = true` and `rustdoc-args = ["--cfg", "docsrs"]`, byte-identical to the two manifests that have it. Verified absent today | `crates/happenstance/Cargo.toml` (no such block) · `crates/happenstance-core/Cargo.toml`:54-56 · `crates/happenstance-testkit/Cargo.toml`:56-58 · `standards/rust/51-features-and-no-std.md`:188-236 |
| Nothing gated is invisible, and the gate proves it (**AC-006**) | Every feature-gated public item on the three published crates renders **with** its `doc(cfg)` pill under `--all-features`; `-p happenstance` joins the nightly `--cfg docsrs -D warnings` step, which is run and recorded here because it is optional and `--fast` drops it. A gated re-export arriving without its pill is a recorded finding, not an un-globbing | `xtask/src/main.rs`:602-636 · `crates/happenstance-core/src/lib.rs`:85,102,121 · `crates/happenstance/src/lib.rs`:76 · `_design.md` AP-8, `## Placement and re-export` |
| The quick start is the crate's own doctest (**AC-007**) | The fence is `_design.md` `## The doctest`'s 19-line write-then-read text, verbatim, fenced `rust`, ≤ 20 source lines, compiled by the existing `#![cfg_attr(doctest, doc = include_str!("../README.md"))]`. Replaces the current 9-line read-only `count_everything` | `_design.md` `## The doctest` · `crates/happenstance/src/lib.rs`:1-10 · `crates/happenstance/README.md`:30-39 · `standards/rust/62-doctests-and-harnesses.md`:228-236 |
| The snippet is copy-pasteable into an empty project (**AC-008**) | The accompanying `toml` fence names the published version and the default feature set (`default = ["std", "memory"]`, which is what `MemoryEventStore` and `collect` need), with no path dependency and no workspace feature unification implied; the same text is handed to `stranger-install-smoke` | `crates/happenstance/Cargo.toml`:22-26 · `_decomposition.md` IQ-5, AC-UX-012 · `_storymap.md`:69 |
| No accepted atom is edited to get here (**AC-009**) | `git diff` touches neither `.kb/decisions/0004-edition-and-msrv.md` nor `0029-msrv-raised-to-1-97-1.md`; `redkiln validate --kb` is clean at the checkpoint, and `redkiln doctor` still reports exactly the six expected `template-drift` advisories | `project.md` DR-6 · `.kb/maps/decision-map.md`:30-34 · `_decomposition.md` AC-TEST-003 |
| The docs.rs ladder is navigable (**AC-010**) | `crates/happenstance/src/lib.rs`'s module doc reads: one-line summary (≤ 2 rendered lines) → `# Status` → `# Using it today` (runnable) → the rest, with the first `#` heading inside 12 rendered lines so the generated sidebar is a section list rather than one entry. `#![doc(html_no_source)]` is left alone; the Status paragraph is true of the tree that ships | `_design.md` `## Composition` (`docs-rs-*`), `## Density budget` (docs.rs lead paragraph) · `crates/happenstance/src/lib.rs`:11-73 |
| Nothing enters the first screen | R7 and R9 are *revealed on scroll* by policy; the 1024×768 first screen is measured full at ≈343 px against 340 px, so a promotion breaks AP-1 for the whole slice | `_design.md` `## Transience policy`, `## Sign-off` |
| Ordering into the release | This story is one of three in `published-surface-copy`, all unordered with each other; the whole slice precedes `rendered-page-preflight` → `publish-0-2-0` → `stranger-install-smoke` | `_storymap.md`:166-184 |

## Data and migrations

**N/A — and the reason is the one that governs this whole project.** This story ships no schema, no
persisted state, no stored format and no runtime data path: its entire diff is prose, one manifest key, one
gate argument and one code fence. The deployment brief already records the project-level posture —
*"Migration / backfill: N/A, stated explicitly… this is the first `0.2.0` publish of these three crates,
there is no prior production data shape for any consumer to migrate, and this library defines no schema of
its own to backfill"* — and the reserved `0.0.0` names are a registry-reservation fact rather than a
compatible predecessor (`CONTRIBUTING.md`:298-300).

What replaces a migration plan here is the **irreversibility posture**, and it does bind this story. A
published page cannot be edited: `cargo yank` removes a version from resolution and leaves every rendered
crates.io and docs.rs page exactly as it was
(`standards/rust/51-features-and-no-std.md`:226-231). So every sentence this story writes is
forward-only — a wrong MSRV promise, a wrong feature-set line or a fence that does not compile ships as a
correction in `0.(2+n).0`, never as an edit. That is why the verification for each behavior above happens
*before* `publish-0-2-0`, and why the `toml` fence's version string follows `registry-surface-diff`'s
report rather than this spec's assumption (`_storymap.md`:186-191).

## Acceptance criteria

Twelve criteria. Each is framed from the person on the other side — the **evaluator**, who
`_design.md`'s DT-1 resolution settles is *the first fifteen minutes of the application author's
journey*: one sitting, time-boxed, no checkout, cannot run the suite
(`.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md`:249-313,
333-338) — and the **constrained-runtime developer** arriving at the same page
(`_decomposition.md`:63-66). The tier labels (T1…T8) are defined in *Tests and CI (merge gate)* below.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **GIVEN** an evaluator reading `https://crates.io/crates/happenstance` with no checkout and no way to run our suite, **WHEN** they scroll to **Guarantees** to answer *"what does depending on you cost me, and when can that cost change?"* (U6), **THEN** one bullet states **1.97.1 as a promise this release makes** — not a measurement someone took — says what an MSRV bump will cost them, and carries exactly one **absolute** link, to the **new** decision atom `msrv-promise-atom` authored (path taken from that story's report, never a guessed number); ADR-0029 is no longer the link, because it records the trade rather than the promise | **T4** — the recorded static read asserts: exactly **one** MSRV bullet in the block; the word *promise* or an equivalent commitment verb present; the bullet's single href equals the atom path in `msrv-promise-atom`'s implementation report, resolved as `https://github.com/Wet-Ink-Corporation/happenstance/blob/main/.kb/decisions/<atom>`; neither `0004-edition-and-msrv.md` nor `0029-msrv-raised-to-1-97-1.md` appears as the MSRV bullet's target. **T6**. Deferred **T8** re-reads it on the rendered page |
| AC-002 | **GIVEN** the same reader, now deciding whether a future upgrade can break them, **WHEN** they read the same Guarantees block, **THEN** they learn without leaving the page that **under 0.x the minor bump is the breaking-change boundary** — the semver contract for the whole release, which today exists only in backlog prose (`project.md`, *Out of scope*) and in `_decomposition.md`'s deployment brief (AC-DEP-006) | **T4** — the static read asserts the 0.x minor-bump sentence is present in `crates/happenstance/README.md`'s Guarantees block, is stated in the caller's register (a consequence for them, not a policy citation), and appears **once** per surface. **T8** |
| AC-003 | **GIVEN** a constrained-runtime developer who runs on `wasm32` and is checking whether that is a first-class target or a footnote (U5), **WHEN** they scan the packaged page — the only surface they have — **THEN** **one bullet** tells them they are supported and **which feature set to take**, without reading source and without a section, callout or lead being spent on it (DT-1's demotion), and the sentence claims no more than `publish-0-2-0`'s four mandatory `wasm32` steps assert | **T4** — `wasm32` appears in `crates/happenstance/README.md` in **exactly one** Guarantees bullet and in no new heading; the feature set named matches `crates/happenstance/Cargo.toml`:22-26. **T6** plus `cargo xtask wasm` (`xtask/src/main.rs`:652) run and recorded, so the sentence is bounded by a check that ran rather than by intent. **T8** |
| AC-004 | **GIVEN** an evaluator scanning Guarantees in a few seconds, **WHEN** the block has absorbed this story's two bullets and the slice-mate's DT-4 bullet, **THEN** it is still **scannable**: ≤ **7** bullets, each ≤ **3** rendered lines, **exactly one** link per bullet, **every** link absolute, a **flat** list with no nested bullet except the DT-4 non-ownership line (which is not this story's), and the MSRV bullet **rewritten in place** so no two sentences on the page can disagree about the floor | **T4** — the static read counts bullets, rendered lines per bullet and links per bullet in the block, and asserts every href starts `https://`. Run at the **end of the slice**, after `compliance-claim-and-gaps-promise` has landed its bullet, because the budget is a property of the finished list. **T8** measures it against `_design.md` `## Density budget` |
| AC-005 | **GIVEN** an evaluator who `cargo add`s the crate and clicks through to docs.rs, **WHEN** docs.rs builds the crate they actually installed, **THEN** it builds it the way the other two are built: `crates/happenstance/Cargo.toml` carries `[package.metadata.docs.rs]` with `all-features = true` and `rustdoc-args = ["--cfg", "docsrs"]`, byte-identical to `crates/happenstance-core/Cargo.toml`:54-56 and `crates/happenstance-testkit/Cargo.toml`:56-58 — closing the gap where the one crate most people install was the one whose docs build was unconfigured | **T4** — the three manifests' blocks are extracted and compared for byte equality including key order. **T3** — the nightly `--cfg docsrs` build now covers this crate and is green |
| AC-006 | **GIVEN** a reader deciding whether a capability exists, **WHEN** they look at the docs.rs page for any of the three published crates, **THEN** **no feature-gated public item is invisible**: every gated item renders **with** its `doc(cfg)` pill under `--all-features`, never absent and never bare — because an item that vanishes reads as *"not supported"*, which is a different and **false** claim (IQ-2, AP-8). **AND** `-p happenstance` has joined the nightly `--cfg docsrs -D warnings` step at `xtask/src/main.rs`:621-635, per that step's own stated joining rule, with the step **run and its output recorded here** rather than left to a gate that may skip it | **T3** — run explicitly with `RUSTDOCFLAGS="--cfg docsrs -D warnings"`, output committed; a warning is a failure. **T2**. **T4** — a recorded read of the generated `target/doc/happenstance/index.html` and `target/doc/happenstance_core/index.html` confirming each gated item carries its pill; a gated re-export arriving on the facade page **without** a pill is recorded as a finding under EC-004, never fixed by naming re-exports individually |
| AC-007 | **GIVEN** an evaluator who has decided the crate is real and now wants to try it (U7), **WHEN** they copy the quick-start fence off the page, **THEN** they copy a **write-then-read cycle that compiles** — `_design.md` `## The doctest`'s text, fenced `rust` (never `rust,ignore`), ≤ **20** source lines so it does not scroll inside itself at 1440×900 (AP-12) — and it is *the crate's own doctest*, compiled by the existing `#![cfg_attr(doctest, doc = include_str!("../README.md"))]` at `crates/happenstance/src/lib.rs`:10, replacing today's nine-line read-only `count_everything` that never writes an event | **T1** — `cargo test -p happenstance --doc` compiles the fence; a fence that does not compile fails the gate rather than the reader. **T4** — the fence is diffed against `_design.md` `## The doctest`, its source lines counted, and its info string asserted to be exactly `rust` |
| AC-008 | **GIVEN** a stranger with an empty `cargo new` project and no access to this workspace, **WHEN** they paste the `toml` fence and then the `rust` fence, **THEN** it resolves and runs: the manifest fence names the published crate and the **default** feature set (`default = ["std", "memory"]` — what `MemoryEventStore` and `collect` need, `crates/happenstance/Cargo.toml`:22-26), with **no path dependency** and nothing that only works under workspace feature unification, and it is the **same text** handed to `stranger-install-smoke` (AC-UX-012 — one text in three places) | **T4** — the `toml` fence is asserted to contain no `path =`, no `git =`, no `[patch]`, and a version requirement inside `0.2.x`; the `rust` fence is byte-compared with the one this story wrote. **T8** — `stranger-install-smoke` runs this exact text against the registry version and is the proof artefact; a divergence there is this AC failing late |
| AC-009 | **GIVEN** a maintainer auditing how the MSRV became a promise, **WHEN** they diff this story's changes, **THEN** **no accepted decision atom was edited to get there**: `.kb/decisions/0004-edition-and-msrv.md` and `.kb/decisions/0029-msrv-raised-to-1-97-1.md` are byte-identical to the tree this story received (project DR-6; `.kb/maps/decision-map.md`:30-34), and the backlog and knowledge base are clean at the checkpoint | `git diff --exit-code <merge-base> -- .kb/decisions/0004-edition-and-msrv.md .kb/decisions/0029-msrv-raised-to-1-97-1.md`, recorded in **T4**. **T5** — `redkiln validate --kb && redkiln doctor`, clean, with exactly the six expected `template-drift` advisories (AC-TEST-003) |
| AC-010 | **GIVEN** an evaluator who landed on docs.rs rather than crates.io, **WHEN** the page renders, **THEN** the sidebar is a **section list rather than a single entry** and the ladder answers their questions in their order: one-line summary (≤ 2 rendered lines) → `# Status` → `# Using it today` (runnable) → the rest, with the first `#` heading inside **12** rendered lines (`_design.md` `## Composition`, `docs-rs-*`); the Status paragraph is true of the tree that ships; `#![doc(html_no_source)]` (`crates/happenstance/src/lib.rs`:73) is left alone | **T2** and **T3** build it. **T4** — a recorded read of the generated `target/doc/happenstance/index.html` sidebar listing ≥ 3 sections, plus a rendered-line count from the module doc's first line to its first `#` heading. **T8** |
| AC-011 | **GIVEN** the reader whose whole first screen is 14 rendered lines at 1024×768 and measured **full** at ≈343 px against a 340 px budget (`_design.md` `## Sign-off`), **WHEN** this story's copy lands, **THEN** **nothing it writes is promoted into the first screen and nothing already there is displaced**: R7 (quick start) and R9 (Guarantees) stay *revealed on scroll*, the region order R6 → R7 → R8 → R9 is preserved, no sixth first-screen region appears, and **no heading this story touches is renamed** — so every inbound anchor a reader may already hold still resolves (IQ-3, AP-1, AP-14, AP-15) | **T4** — the ordered list of `##` headings and their generated slugs is captured before and after; the set of slugs must not shrink and the R6…R9 order must be unchanged; the count of regions above the first `##` heading is unchanged. **T8** compares the rendered page against `.bklg/from-contract-to-published-library/publication-and-positioning/design/reference/crates-io-happenstance@1024x768.png` |
| AC-012 | **GIVEN** a reader on a page whose CSS we do not own, with images blocked, in either theme, **WHEN** they read the regions this story wrote, **THEN** those regions carry **real composed presentation from this repository's own prose forms** and not bare markup: the Guarantees flat bulleted list under its `##` (`_design.md` `## Hierarchy`, R9), the claim-with-evidence form for the MSRV promise (`README.md`:227-234's form), **every** fence declaring its language (`rust`, `toml`), **every** link carrying meaningful text — never *here*, *this*, or a bare URL — and **no** raw HTML, inline `style`, image, glyph-alone signal or animated media anywhere in them (AP-7, AP-2, the UX brief's accessibility floor) | **T4** — the static read asserts, over the diffed regions only: zero `<` HTML tags, zero image links, every fence has a non-empty info string, every link's text is ≥ 2 words and is not a URL, and the Guarantees list has no nested bullet other than the reserved DT-4 line. **T8** re-reads the same regions on the rendered page with images disabled |

Every traced project AC is covered: **AC-006** by AC-001 and AC-002 (the sentence a consumer reads, the
atom itself being `msrv-promise-atom`'s half); **AC-007** by AC-005, AC-006, AC-010, AC-011 and AC-012
(the docs.rs configuration and the page that looks finished); **AC-008** by AC-007 and AC-008 (the
snippet being the same text the smoke runs); **AC-014** by AC-003 (stated so a `wasm32` reader can
self-identify, with `publish-0-2-0` owning the assertion). AC-009 discharges project DR-6's prohibition
inside this diff.

## Interaction quality

RFC §6.7/D6, in this medium. **Every invariant below is carried by an `AC-###` row in the table above**
— this section says *which* row carries it and how it is verified, and adds nothing that is not gated.
The medium translation is the UX brief's: the reader's interaction is *navigating a claim to its
evidence and back inside one sitting* (`_decomposition.md`, IQ-1…IQ-7).

**State invariants**

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **In place, not a context jump** (IQ-1) — each of U5 and U6 is answerable on the page the reader landed on; a link deepens a claim, it never *carries* it. Budget: **0 hops to read a claim, ≤ 1 hop to its evidence**, and that hop lands on a specific artefact | AC-001, AC-002, AC-003, AC-004 | T4 asserts each bullet states its answer in full and carries **exactly one** link; a Guarantees bullet whose answer is *"see the specification"* fails AC-004's one-link-per-bullet read and AC-001's promise read |
| **Non-occlusion** (IQ-2) — a filter must not hide what it filters; a gated item annotates, it never subtracts | AC-006 | T3 under `--all-features` plus the recorded read of the generated page for the `doc(cfg)` pill. An item absent because a feature was off is the false claim this rejects |
| **Preserved position** (IQ-3) — inbound anchors survive; a heading either keeps its slug or every inbound reference moves in the same change | AC-011 | T4's before/after slug capture: the slug set may grow, never shrink |
| **Reversibility** (IQ-4) — bought *before* the act, because a published page cannot be edited and `yank` leaves every rendered page exactly as it was | AC-007, AC-008, AC-009 | Everything this story writes is checked pre-publish: the fence by a compiler (T1), the atom links by T4, the immutability by `git diff --exit-code`. `rendered-page-preflight` then reads the rendered page before `publish-0-2-0` |
| **Reachable without running anything** (IQ-5) — the medium's keyboard-reachability analogue: no claim may require the reader to run our CI, clone the repo, or use a pointer to reveal it. Plain text is the floor, not the fallback | AC-008, AC-012 | T4 asserts no path dependency and no *"run our CI"* evidence; T8 re-reads with images disabled |
| **Truth at the publish commit** (IQ-7) | AC-002, AC-003, AC-010 | T4's feature-set comparison against the real manifest, `cargo xtask wasm`, and the Status paragraph read against the shipping tree |

**Composition invariants** — taken from the signed-off `_design.md`, which is binding on this story

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **Presentation exists at all** — every region is composed from the repository's own prose forms (flat bulleted Guarantees under its `##`, the claim-with-evidence form, languaged fences), never bare markup or a naked sentence | AC-012 | T4 over the diffed regions; T8 on the rendered page |
| **Composition and placement** — R6 → R7 → R8 → R9 order preserved; the compliance block above the quick start; no new section opened where a slot already exists | AC-011 | T4's heading-order capture; T8 against the signed-off reference frame |
| **Transience** — R7 and R9 are *revealed on scroll*, never promoted to persistent chrome; the `wasm32` line is one bullet, not a callout; the MSRV atom is 1 hop, not inlined | AC-003, AC-011 | T4 (region count above the first `##`, bullet count for `wasm32`); T8 at 1024×768 |
| **Density budget, with its real numbers** — Guarantees ≤ **7** bullets × ≤ **3** rendered lines × **exactly 1** link; quick-start fence ≤ **20** source lines; docs.rs lead ≤ **2** rendered lines with the first `#` inside **12** | AC-004, AC-007, AC-010 | T4 counts each, at the end of the slice for AC-004; T1 for the fence's compilability; T2/T3 for the docs ladder |
| **Hierarchy** — Guarantees is **secondary**: `##` heading plus a flat list, no nested bullets except the reserved DT-4 line, no bold competing with R2's blockquote; at most one primary-ranked element per screenful | AC-004, AC-012 | T4's nesting and emphasis read; T8's screenful check |
| **Named anti-patterns** — **AP-1** (first screen still says who it is for), **AP-2** (no glyph-alone), **AP-6** (no relative link on a packaged surface), **AP-7** (no raw HTML/CSS/JS/image-carried meaning), **AP-8** (no gated item without its pill), **AP-12** (no self-scrolling fence, no divergence from the smoke's text), **AP-14** (nothing decorative promoted above the callout), **AP-15** (one primary per screenful) | AC-004 (AP-6), AC-006 (AP-8), AC-007 (AP-12), AC-011 (AP-1, AP-14, AP-15), AC-012 (AP-2, AP-7) | Each is phrased in `_design.md` so it is checkable against a rendered screenshot; T4 catches the mechanical ones in source, T8 catches all of them on the page |

An unstyled render satisfies every mechanical assertion above perfectly. AC-004, AC-011 and AC-012 are
what make that fail — they are the rows that fail when the copy is *correct and unreadable*.

## Error conditions

| id | condition | required behaviour |
| --- | --- | --- |
| **EC-001** | `msrv-promise-atom` has not landed, or its report does not name the atom's final path | **Stop; do not guess a number.** A wrong `.kb/decisions/00NN-…` URL is a permanently dead link on a page that cannot be edited (AP-6). The story is blocked, not worked around; record the block and wait |
| **EC-002** | No nightly toolchain is available, so `cargo xtask ci`'s docs.rs step **skips** | AC-006 is **not** discharged by a skip. `--fast` drops the step entirely (`.redkiln/config.yaml`; `xtask/src/main.rs`:45-52) and a probe failure is a skip, not a pass. Either install nightly and record the run, or leave AC-006's ledger row `satisfied: false` and do not advance |
| **EC-003** | The nightly build emits a **warning** | It is a failure — the step runs with `RUSTDOCFLAGS="--cfg docsrs -D warnings"`. Fix the cause. Do **not** add an `#[allow]`, and do not narrow the crate list to make it pass |
| **EC-004** | Under `--cfg docsrs`, a gated item re-exported through `pub use happenstance_core::*;` renders on the facade's page **without** its gate pill | **Record it as a finding; do not un-glob the re-export.** Naming re-exports individually is an API-shape change, and this project carries no `architecture` brief precisely so a surface change goes back to the owning crate and a re-plan (`project.md`, *Out of scope*; C7) |
| **EC-005** | `projection-port-ship-shape`'s PS-3 arm is unknown, or its report is unread | Do **not** add an `unstable-projection` forwarding entry speculatively, and do **not** re-derive the verdict. Read that story's report; if the port froze, no feature is added and AC-006 is discharged over the crates' existing gated items only |
| **EC-006** | With the slice-mate's DT-4 bullet in place, the Guarantees list would exceed 7 bullets or a bullet would exceed 3 rendered lines | Apply `_design.md` `## Density budget`'s demotion order — badges, then the census, then the identity line's second sentence — and **never** the status callout or the triad. If the order cannot absorb it, something was added that should not have been: stop and escalate rather than shrinking a promise to fit |
| **EC-007** | `_design.md` `## The doctest`'s text does not compile against the real API (an `Event::new`, `Tags::from_pairs`, `with_tags` or `collect` signature differs) | The compiler wins on the *literal*, the design wins on the *intent*: keep the write-then-read cycle, the single gaps comment and the ≤ 20-line budget, adjust the calls, and record the delta in the implementation report so `stranger-install-smoke` runs the corrected text. Never demote the fence to `rust,ignore` (RS-62-5 exists so a packaged example need not be) |
| **EC-008** | `registry-surface-diff` has not yet reported, so the exact published patch version is unknown | Write the requirement as `happenstance = "0.2"`, which is `^0.2` and admits any `0.2.x`, and let `rendered-page-preflight` confirm it against the report. If the report implies a version **outside** `0.2.x`, that is a re-plan, not a rounded-down number (`_storymap.md`:186-191) |
| **EC-009** | A stale string this story did not write is found inside a region it is editing (AP-9) | Fix it only if it is inside this story's diff boundary; otherwise leave it and note it for `landing-copy-and-status-truth`, which owns the four named stale strings. Two stories editing the same sentence in one slice is a merge conflict by construction |

## Non-functional

| id | requirement | why, and where it is checked |
| --- | --- | --- |
| **NF-001** | **Forward-only prose.** Every sentence this story writes must be one that can be *superseded by a later dated claim* rather than needing a silent rewrite — because `cargo yank` leaves every rendered page exactly as it was (`standards/rust/51-features-and-no-std.md`:226-231) | The wording discipline of IQ-4; read at T8 before the irreversible act |
| **NF-002** | **Zero cost to a caller.** No dependency added, no feature added, no public signature changed. `[package.metadata.docs.rs]` is build metadata for one renderer and costs a consumer nothing (`_design.md` `## What it costs a caller`) | T6; `cargo xtask package-check` (T7) still passes with the three publishable crates unchanged |
| **NF-003** | **Bounded gate cost.** Adding `-p happenstance` adds one facade crate's rustdoc build to an already-optional nightly step; no mandatory step gains work | Observed in the recorded T3 run |
| **NF-004** | **Accessibility floor holds on the regions written** — plain-text legible, correct with images blocked, legible in light, dark and ayu because we set no colour | AC-012 in source; the dated read at `rendered-page-preflight` (AC-UX-006) on the rendered page |
| **NF-005** | **The MSRV does not move.** This story *states* the floor; it does not raise or lower it. `rust-toolchain.toml`'s pin and `rust-version.workspace` are untouched | T6; any change would surface as a `cargo hack --rust-version` failure in CI's `msrv` job |
| **NF-006** | **No lint escape hatches.** No new `#[allow]`, no `clippy.toml` edit, no `rust,ignore` fence, no narrowing of a gate step's crate list | T6 with `-D warnings`; T3 with `-D warnings` for rustdoc |

## Implementation notes (non-prescriptive)

**Read the two dependency reports first, before writing a single character of prose.** Two facts this
spec deliberately does not carry are only knowable from them: the MSRV atom's final path
(`msrv-promise-atom`) and which arm PS-3 took (`projection-port-ship-shape`). Writing the Guarantees
bullet before the first is how a dead link ships.

**A plausible order** — mechanical first, so the gate is proving something while the prose is being
weighed:

1. `crates/happenstance/Cargo.toml` — paste the block from `crates/happenstance-core/Cargo.toml`:54-56
   unchanged, key order included.
2. `xtask/src/main.rs`:621-635 — add `-p happenstance` to the argument list and extend the step's
   comment, which already states the joining rule in its own words. Then run it: `cargo +nightly doc
   --locked -p happenstance-core -p happenstance -p happenstance-testkit --all-features --no-deps` with
   `RUSTDOCFLAGS="--cfg docsrs -D warnings"`, and keep the output.
3. `crates/happenstance/src/lib.rs` — the module-doc ladder (AC-010). It already opens with a one-line
   summary and `# Status`, so this is a re-ordering and a truth pass rather than a rewrite. Add
   `#![cfg_attr(docsrs, feature(doc_cfg))]` **only** if an item on this crate's own page needs it.
4. The fence, next-to-last, so it is compiled against the API as it actually is: `cargo test -p
   happenstance --doc` is the fastest loop available and is the same mechanism the gate uses.
5. The prose bullets last, when the atom path and the PS-3 arm are both known and the density count can
   be taken against a finished list.

**Reading the rendered artefact locally is cheap and is the only way to see AC-006 and AC-010.**
`cargo doc -p happenstance --all-features --no-deps` then open `target/doc/happenstance/index.html`: the
sidebar's section list is AC-010's evidence and the `doc(cfg)` pills are AC-006's.
`crates/happenstance-core/src/lib.rs`:85, 102 and 121 are the reference treatment already in the tree —
copy the shape, do not invent one.

**The static checks are cheap to write and must be recorded, not remembered.** Commit the exact
commands and their output as `_checks.md` in this story's folder; several ACs have no compiled test and
a remembered observation is not evidence a ledger row can cite.

**Two things it is easy to do by reflex and must not.** Appending a second MSRV bullet instead of
rewriting the one that is there (two sentences that can disagree, and a budget overrun). And filling the
hole the DT-4 bullet is reserved for — the list will look short by one until the slice-mate lands, and
that is correct.

## Tests and CI (merge gate)

Grounded in `_decomposition.md`'s testing brief, whose AC-007 row is explicit that the rendered-page
half of this work is **human observation** and cannot be replaced by an automated check
(`xtask/src/package.rs`:4-18 — containment is not presentation), and whose AC-014 row is explicit that
the `wasm32` obligation is **regression only, no new instrument**. This story therefore adds **no new
gate step and no new lint**: the deployment brief allocates the project's only two new instruments to
`registry-surface-diff` and `clause-maturity-audit` (AC-DEP-005), and inventing a third here would be
the mechanism-invention the grounding forbids.

| tier | command / path | proves |
| --- | --- | --- |
| **T1 — doctest** | `cargo test -p happenstance --doc` (compiles `crates/happenstance/README.md`'s fence through `crates/happenstance/src/lib.rs`:10) | AC-007: the quick start is the crate's own doctest and it compiles. A fence that does not compile fails the gate rather than the reader — the wrong implementation this rejects is a hand-written snippet that drifted from the API |
| **T2 — docs, all features** | `cargo doc --workspace --all-features --no-deps --locked` (a mandatory `REQUIRED` step, `xtask/src/main.rs`) | AC-006, AC-010: the documentation builds under every feature and no intra-doc link is broken in that configuration (RS-70-2) |
| **T3 — nightly `--cfg docsrs`** | `cargo +nightly doc --locked -p happenstance-core -p happenstance -p happenstance-testkit --all-features --no-deps` with `RUSTDOCFLAGS="--cfg docsrs -D warnings"` (`xtask/src/main.rs`:621-635, after this story adds `-p happenstance`) | AC-005, AC-006: the crate an evaluator installs is finally compiled the way docs.rs will compile it, with `doc(cfg)` live and warnings fatal. **Optional in the gate and dropped by `--fast`, so it is run and its output committed here deliberately** (EC-002) |
| **T4 — recorded static checks** | `.bklg/from-contract-to-published-library/publication-and-positioning/guarantees-and-docs-rs-presentation/_checks.md` — the committed transcript of each `rg`/`git diff` invocation named in the acceptance table, with its output and the date | AC-001 … AC-005, AC-007 … AC-012's mechanical halves: bullet counts, link counts, absolute-link check, byte-equality of the three manifest blocks, fence line count and info string, heading-slug before/after, accepted-atom immutability |
| **T5 — backlog and KB** | `redkiln validate --kb && redkiln doctor` | AC-009: no accepted atom was edited, KbFrontmatter conformance holds, and `doctor` reports exactly the six expected `template-drift` advisories and no `dependency-cycle` (AC-TEST-003) |
| **T6 — story grain** | `cargo xtask affected --base main` (wired to redkiln's story grain in `.redkiln/config.yaml`), plus `cargo xtask wasm` for AC-003 | fmt, clippy `-D warnings`, tests for the affected packages and their dependents, the five file-reading lints and `spec-trace` unconditionally; the four `wasm32` steps bounding AC-003's sentence |
| **T7 — packaging containment** | `cargo xtask package-check` (`xtask/src/package.rs`) | NF-002: the three publishable crates each still contain `LICENSE-MIT`, `LICENSE-APACHE` and `README.md` (`:94`) after the README and manifest edits. Containment only — it says nothing about presentation, which is the whole reason T8 exists |
| **T8 — deferred human read** | `rendered-page-preflight` (this project's next story), against `.bklg/from-contract-to-published-library/publication-and-positioning/design/reference/crates-io-happenstance@1024x768.png` and `…@1440x900.png`, and `…/design/mock.html` | AC-004, AC-011, AC-012 and the rendered half of AC-001 … AC-003 and AC-010. It is a *deferred* verification, not an absent one: the ledger rows those ACs carry cite this story's source-level evidence, and `rendered-page-preflight` is where the same claims are re-read on the page before the irreversible act |

**Integration grain**: `cargo xtask ci --fast` at the project's integration stage
(`.redkiln/config.yaml`), which is the bar a non-terminal project meets — and which drops T3, which is
exactly why T3 is run and recorded by hand here. The full `cargo xtask ci` on the publish commit belongs
to `publish-0-2-0` (project AC-016).

## Risks and coupling (PR-scoped)

| risk | likelihood / impact | mitigation, inside this PR |
| --- | --- | --- |
| The MSRV atom's number is guessed rather than read, producing a dead link on a page that cannot be edited | Medium / **High** | EC-001 makes it a hard stop. AC-001's verification compares the href against the path in `msrv-promise-atom`'s report rather than against a pattern |
| Three stories edit `crates/happenstance/README.md`'s Guarantees list in one slice and collide, or blow the 7-bullet budget between them | **High** / Medium | The slice is implemented in **one context** by design (`_storymap.md`:88-90). Ownership is explicit: this story owns the MSRV and `wasm32` bullets and rewrites the MSRV bullet in place; `compliance-claim-and-gaps-promise` owns the DT-4 bullet. AC-004 is counted **once, at the end of the slice** |
| A green local gate hides AC-006, because the nightly step probed absent and skipped | Medium / **High** | EC-002. T3 is run explicitly and its output committed; a skipped step may not flip AC-006's ledger row |
| PS-3's arm is assumed instead of read, and an `unstable-projection` entry is added to a crate that froze the port (or omitted from one that gated it) | Medium / High | EC-005; the public-items list in *Integration contract* marks the feature **not claimed** by this story, conditional on the dependency's report |
| `_design.md`'s doctest text does not compile against the real API, and the fence is quietly demoted to `rust,ignore` to make the gate green | Medium / **High** | EC-007 forbids the demotion outright (RS-62-5, `standards/rust/62-doctests-and-harnesses.md`:228-236); T1 is the loop that finds it early; the delta is recorded so `stranger-install-smoke` runs the corrected text |
| A missing `doc(cfg)` pill on a glob re-export is "fixed" by naming re-exports individually — an API-shape change smuggled into a documentation story | Low / **High** | EC-004 and C7: record the finding, escalate to the owning crate and a re-plan. `pub use happenstance_core::*;` and `#![doc(html_no_source)]` are named as untouchable in *PR boundary* |
| The `toml` fence names a version the release does not take, and the sentence is unfixable after publish | Medium / High | EC-008: write `^0.2`, confirm at `rendered-page-preflight`, treat anything outside `0.2.x` as a re-plan |
| Copy is written at the repo root instead of the packaged README and never reaches a registry reader | Low / High | The mount point is `crates/happenstance/README.md` and the *PR boundary* file list contains no root `README.md`. `include_str!` cannot reach outside the package (`README.md`:131-137) |

**Coupling, stated once.** Inbound: `msrv-promise-atom` (link target) and `projection-port-ship-shape`
(which arm renders). Sideways: two slice-mates on the same page, one shared bullet list. Outbound:
`rendered-page-preflight` reads what this writes, and `stranger-install-smoke` *runs* what this writes —
the fence is the only artefact in this story that a later story executes rather than reads.

## Dependencies

**Blocks on** (matches this story's `depends_on`, and the item frontmatter the CLI owns):

- **`msrv-promise-atom`** — authors the new decision atom at 0030-or-above through `.kb/_intake/` and
  `/redkiln:kb-ingest`. AC-001's link target is that atom's path, taken from its implementation report.
  Without it this story cannot write its central sentence (EC-001).
- **`projection-port-ship-shape`** — settles PS-3. Decides whether `happenstance` forwards an
  `unstable-projection` feature at all, and therefore what AC-006 must find rendering *with* its pill
  (EC-005).

**Unordered with** (same slice, same page, one context): `landing-copy-and-status-truth`,
`compliance-claim-and-gaps-promise` (`_storymap.md`:166-169).

**Unlocks**:

- **`rendered-page-preflight`** — directly; it reads the rendered pages this story configures and writes,
  and it is the T8 verification several ACs defer to. It also blocks on the two slice-mates.
- **`publish-0-2-0`** and **`stranger-install-smoke`** — transitively. The smoke runs this story's fence
  text against the registry version, which is project AC-008's split: *the run* versus *the snippet being
  the same text the page shows* (`_storymap.md`:131).

## Anchors (progressive disclosure)

Link, do not paste. Each row says why it is load-bearing, the moment to open it, and the AC it serves.
Every path was confirmed to exist before it was cited.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `.bklg/from-contract-to-published-library/publication-and-positioning/_design.md` | The **signed-off, binding** design. Carries `## The doctest`'s exact 19-line text, `## Density budget`'s real numbers, `## Transience policy`, `## Hierarchy` and the fifteen anti-patterns. This story implements it and may not re-decide it | Before writing any copy, and again before the fence — `## The doctest` is a literal to copy, not a paraphrase | AC-004, AC-007, AC-010, AC-011, AC-012 |
| `.bklg/from-contract-to-published-library/publication-and-positioning/_decomposition.md` | The three briefs. UX brief AC-UX-007/009/010/012 are this story's criteria in the project's own words; IQ-1…IQ-7 are the invariants; the testing brief's AC-007/AC-008/AC-014 rows fix the test tiers; the deployment brief fixes the feature-gating and `wasm32` posture | When drafting the acceptance evidence, and before adding any gate step (it forbids a third instrument) | AC-001, AC-002, AC-003, AC-005, AC-008 |
| `.bklg/from-contract-to-published-library/publication-and-positioning/_storymap.md` | The slice boundary and the merge order: why these three stories are one page, and where the release blocks if a report implies something outside `0.2.0` | When coordinating with the slice-mates, and when writing the `toml` fence's version | AC-004, AC-008 |
| `.bklg/from-contract-to-published-library/publication-and-positioning/design/mock.html` | 51 self-contained frames of all seven surfaces at both viewports, including `images-blocked` and a dark-theme frame. The only place the intended composition can be *seen* rather than read | When judging whether a bullet's third line pushes the triad across the fold, before committing the copy | AC-004, AC-011, AC-012 |
| `.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md` | The evaluator's two decisive properties — one-shot and time-boxed, cannot run our suite — which are why a claim may not be carried by a link | Before deciding how much a bullet may lean on its link | AC-001, AC-003 |
| `crates/happenstance/README.md` | The mount point. Lines 41-49 are the Guarantees block being rewritten; 30-39 is the fence being replaced; 13-20 is the triad that must not move | Immediately, and before every static count | AC-001, AC-002, AC-003, AC-004, AC-007, AC-011 |
| `crates/happenstance/Cargo.toml` | The manifest missing `[package.metadata.docs.rs]`; lines 22-26 are the feature set the `toml` fence and the `wasm32` bullet must name accurately | Step 1 of implementation, and again when writing AC-003's and AC-008's sentences | AC-005, AC-003, AC-008 |
| `crates/happenstance/src/lib.rs` | Line 10 is the `include_str!` wiring that makes the README fence a doctest (already present — do not re-plumb); 73 is `#![doc(html_no_source)]` (do not flip); 76 is the glob re-export (do not un-glob); 11-71 is the ladder AC-010 reorders | Before touching the module doc, and before reacting to a missing pill | AC-007, AC-010, AC-006 |
| `crates/happenstance-core/Cargo.toml` | Lines 54-56 are the manifest block to copy **byte-identically**, key order included | Step 1 | AC-005 |
| `crates/happenstance-core/src/lib.rs` | Lines 85, 102 and 121 are the in-tree reference treatment: `#![cfg_attr(docsrs, feature(doc_cfg))]` and `doc(cfg)` on gated items. Copy the shape rather than inventing one | When reading the generated page for pills, and if an item on the facade needs a gate | AC-006 |
| `crates/happenstance-core/README.md` | Lines 43-54 are the sibling Guarantees block whose MSRV bullet must gain the same link, so the two packaged surfaces cannot disagree about whether the MSRV is a promise | When the atom path is known, in the same commit as the `happenstance` bullet | AC-001 |
| `xtask/src/main.rs` | Lines 621-635 are the nightly `--cfg docsrs` step and its comment stating the joining rule this story satisfies; 8-24 is the gate's prose description of what it proves; 43-52 explains that optional means *skipped when the tool is absent*, never *ignored when it fails* | Step 2, and again when interpreting a skip | AC-006 |
| `xtask/src/package.rs` | Lines 4-18 state why containment is not presentation — the reason T8 is a human read and not an automated one; `:94` is `REQUIRED_FILES`, which the README edit must not invalidate | When tempted to automate the rendered-page check, and before the packaging step | AC-012, AC-005 |
| `standards/rust/51-features-and-no-std.md` | RS-51-5 (`:188-236`) is the manifest-plus-`doc(cfg)` treatment verbatim; `:226-231` is the yank semantics that makes every sentence forward-only | Step 1, and when writing anything that cannot be edited later | AC-005, AC-006 |
| `standards/rust/62-doctests-and-harnesses.md` | RS-62-5 (`:228-236`) is the out-of-package doctest pattern — the reason a packaged example never needs `rust,ignore` | Before the fence, and immediately if it fails to compile | AC-007 |
| `standards/rust/70-rustdoc-obligations.md` | RS-70-1's section vocabulary, RS-70-2 on feature-conditional intra-doc links, RS-70-3 on `doc_markdown`, RS-70-5 on naming the alternative that lost — the house voice for every claim on this surface | Before rewriting the module doc | AC-010, AC-012 |
| `standards/rust/40-public-surface-and-evolution.md` | RS-40-5 (`:212`) — a declined capability's constructor rejects an empty reason, which is the mechanism IQ-2 borrows: **no surface may state an absence without a reason** | When wording the `wasm32` line or any limitation | AC-003, AC-012 |
| `.kb/decisions/0004-edition-and-msrv.md` | Lines 83-86 state the MSRV is *"a preference until first publish and a promise afterward"* — the sentence this story is the other half of. **Accepted and immutable: read only** | Before writing the MSRV bullet; never with an editor | AC-001, AC-009 |
| `.kb/decisions/0029-msrv-raised-to-1-97-1.md` | Lines 94-95 close with *"phase 12 must revisit the floor at first publish."* It records the measurement and the trade — which is why it stops being the Guarantees link. **Accepted and immutable: read only** | Alongside 0004, to see what the new atom must *not* duplicate | AC-001, AC-009 |
| `.kb/maps/decision-map.md` | Lines 30-34 carry the immutability rule `redkiln validate --kb` enforces against `HEAD` | If any instinct arises to "just update" an ADR | AC-009 |
| `.kb/decisions/0001-async-port-flavours.md` | The two-flavour design the `wasm32` bullet is claiming on behalf of: `EventStore` without a `Send` bound, `SendEventStore` derived by `trait_variant` | Before writing AC-003's sentence, so it claims the flavour rather than the platform | AC-003 |
| `README.md` | Lines 159-164 are the existing two-flavours copy being reduced to one packaged bullet; 131-137 explains why root copy never reaches a registry reader | When drafting the `wasm32` bullet, and if tempted to edit the root README instead | AC-003, AC-008 |
| `.redkiln/config.yaml` | The `verify:` block wiring `cargo xtask affected` to the story grain and `cargo xtask ci --fast` to the integration grain — and what `--fast` drops, which is exactly T3 | When deciding whether the automatic gate has discharged AC-006 (it has not) | AC-006 |

## Clarifications resolved during spec

1. **Two acceptance criteria were added to the ten the front half enumerated: AC-011 and AC-012.** The
   front half's *Behavior and interfaces* table carried two rows with no `AC-###` id — *"Nothing enters
   the first screen"* and *"Ordering into the release"*. The first is a blocking composition invariant
   from the signed-off `_design.md` (`## Transience policy`, `## Sign-off`'s measured ≈343/340 px
   finding), and `redkiln verify` extracts ACs only from a leading `| AC-001 |` cell or an `- AC-001:`
   bullet — so as prose it would never have been gated or tested. It is now **AC-011**, widened to carry
   placement and anchor survival with it. **AC-012** was added for the same reason on the other axis:
   nothing in AC-001…AC-010 fails when the copy is correct but composed as bare markup — no languaged
   fence, a bare-URL link, a glyph with no words — which is precisely the failure an unstyled render
   hides. The second unnumbered row, *"Ordering into the release"*, is **not** promoted to an AC: it is a
   fact about the merge order rather than a property of this diff, and it is recorded under
   *Dependencies*.
2. **This story adds no new gate step, lint or conformance rule, and that is a decision rather than an
   omission.** The deployment brief allocates the project's only two new mandatory instruments to
   `registry-surface-diff` and `clause-maturity-audit` (AC-DEP-005), the testing brief's AC-007 row says
   the rendered-page check *cannot* be automated (`xtask/src/package.rs`:4-18), and `CLAUDE.md`'s rule is
   that a rule no adapter can fail is decorative. Adding `-p happenstance` to an existing step is an
   argument change, not a new instrument.
3. **The `toml` fence's version requirement is `happenstance = "0.2"`, not a pinned patch version.** The
   exact published patch level follows `registry-surface-diff`'s report, which may not exist when this
   story lands; `^0.2` is true of every `0.2.x` and false of nothing the release may take. Anything
   outside `0.2.x` is a re-plan (EC-008).
4. **AC-006 covers all three published crates, not just `happenstance`.** The nightly step is one
   command over a crate list, so extending the list is what discharges it; narrowing it later to make a
   build pass is forbidden by NF-006.
5. **The MSRV atom's number is deliberately absent from this spec.** `msrv-promise-atom` takes 0030 or
   above and the exact number is not knowable here; every reference is to the story's report rather than
   to a number, and guessing is EC-001.
6. **`_design.md`'s doctest text is binding as intent, and the compiler is binding as fact.** If the
   19-line snippet does not compile against the real API, the write-then-read shape, the single gaps
   comment and the ≤ 20-line budget survive and the calls change, with the delta recorded so
   `stranger-install-smoke` runs the corrected text (EC-007). This is the one place this story is
   permitted to differ in letter from a signed-off design, and only because a fence that does not compile
   would fail AC-007 outright.
7. **T8 is a deferred verification, not a missing one.** Four criteria (AC-004, AC-011, AC-012 and the
   rendered halves of AC-001…AC-003 and AC-010) can only be finally judged on a rendered page, which
   `rendered-page-preflight` owns. Their ledger rows are satisfied here by source-level evidence — counts,
   diffs, generated-HTML reads — and re-read there before the irreversible act. Nothing is left to a
   story that runs *after* publish.
