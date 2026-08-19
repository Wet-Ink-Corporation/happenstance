---
item: HS-S0096
stage: spec
created: 2026-08-12T13:47:34.177Z
updated: 2026-08-12T13:47:34.177Z
template_sig: 87bbf1d0
rendered_sig: 5f6b9641
---

# Spec — 0.2.0 is published, with the whole gate green on the exact commit

## Scope lock

| | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` — DoD 13, *"the gate is green on the assembled whole … on the exact tree that was published"* (`:396-397`); DoD 9, 10, 11, 12 are made reachable here and turned green elsewhere |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` — the DAG (`:150`), which puts this project at rank 3 behind five blockers |
| Project | `.bklg/from-contract-to-published-library/publication-and-positioning/project.md` — AC-001, AC-014, AC-016; DR-1, DR-14, DR-15; Definition of done items 1, 3 and 5 |
| This spec | `.bklg/from-contract-to-published-library/publication-and-positioning/publish-0-2-0/spec.md` |
| Key briefs | `.bklg/from-contract-to-published-library/publication-and-positioning/_decomposition.md` — **deployment brief** (`### Migration, backfill and rollback posture`, `### Release path if a published version changes`; AC-DEP-003, AC-DEP-004) and **testing brief** (`### Test mix` AC-014/AC-016 rows; AC-TEST-004's committed full-gate artefact). No `architecture` brief exists for this project, deliberately |
| Signed-off design | `.bklg/from-contract-to-published-library/publication-and-positioning/_design.md` — binding. This story renders **no** new surface; it is the act that makes all seven of them resolve, and `## What it costs a caller` is where AC-014's *asserted, not assumed* wording comes from |
| Grounding | `.bklg/from-contract-to-published-library/publication-and-positioning/_grounding.md` |
| Story map / merge order | `.bklg/from-contract-to-published-library/publication-and-positioning/_storymap.md` — milestone `the-release-event`, position 4.2, strictly after `rendered-page-preflight` (`:179-184`) |
| Prior art in this initiative | `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/publish-0-2-0-alpha-1/spec.md` — the alpha cut. Its ordering, its `_release-log.md` and its error catalogue are **reused, not re-derived** |

## One-line PR slice

Publish the decided three crates at the version `registry-surface-diff` implies, having run the
*whole* `cargo xtask ci` — not `--fast` — on the literal publish commit and committed its dated
output, with the four mandatory `wasm32` steps asserted against the published tree **and the
published feature set**, so the `!Send` flavour is proven rather than assumed.

## Executive summary

**What lands.** One commit that makes the release tree the release tree — the version strings, the
`CHANGELOG.md` section, the re-observed preconditions — then one dated record of three irreversible
acts, and one small permanent addition to the gate. Concretely: `Cargo.toml`'s version root and
requirement strings returned from the alpha's pre-release number to `0.2.0`; a `## [0.2.0]` section
in `CHANGELOG.md`; a fifth mandatory `wasm32` step covering the crate a consumer actually installs;
`spec/audits/clause-maturity-<date>.md` re-run at *this* commit; and
`.bklg/.../publish-0-2-0/_release-log.md` carrying the full-gate transcript, the dry runs, the
publish transcripts, the tag, and the post-publish resolution check.

**The delta against what the tree already does.** Almost everything this story needs already exists
and has been proven at the alpha: `cargo xtask ci` is one command defined once
(`xtask/src/main.rs:1-7`), `package-check` already asserts both licences and the README are inside
each `.crate` (`xtask/src/package.rs:88-94`), `reconcile` already fails when the intention list and
the derived set disagree, and `publish-0-2-0-alpha-1` already discovered — before it could cost
anything — that the requirement strings at `Cargo.toml:24-26` must carry the release number *before*
the first `cargo publish`, because the failure is silent until the second one. Three things are
genuinely absent, and they are what this PR is for.

1. **Nothing in the gate ever compiles `happenstance` for `wasm32`.** The four mandatory steps cover
   `happenstance-core` (at `--no-default-features --features std`, which is *not* its published
   default set), the testkit's harnesses, and two adapters that are `publish = false`
   (`xtask/src/main.rs:203-282`). The optional powerset widens `happenstance-core`,
   `happenstance-neon` and `happenstance-testkit` on that target (`:558-593`) — and still never names
   `happenstance`. So the crate a `cargo add` reader installs, at the feature set they get by
   default, has never been type-checked on the target ADR-0001 exists to protect. AC-014's *asserted,
   not assumed* is a real instruction, not a formality.
2. **No release has ever been recorded at this grain.** Project DoD 3 wants dated committed
   artefacts rather than remembered observations, and AC-TEST-004 wants the full-gate run to be an
   artefact *distinct from* any `--fast` run — and `--fast` is what `.redkiln/config.yaml:55` fires
   automatically at this project's integration stage, so the two will otherwise look alike in the
   log and only one of them proves anything about the release.
3. **Nothing composes the five upstream preconditions into one refusal.** Each blocker produced a
   dated artefact; nothing re-reads them *at the publish commit* and stops the act when one names a
   different tree. This story is the composition point, and the composition is the deliverable.

**What this PR is not.** It writes no README copy, resolves no design tension, adds no clause,
repairs no ledger row, and does not build the surface-diff or clause-audit instruments — it *runs*
them. It also does not follow `RUNBOOK.md:4475`'s instruction to strip `provisional` from ADR-0004:
that atom is accepted and immutable, and the promise is `msrv-promise-atom`'s new atom instead.

## Context pack

Everything an implementer needs to start. Deeper material sits behind the signposted anchors.

**One act is irreversible and it runs last.** A crates.io publish cannot be edited or deleted
(`RUNBOOK.md:4481-4484`), and `cargo yank` removes a version from *future resolution* while leaving
every already-rendered page exactly as it was
(`standards/rust/51-features-and-no-std.md:226-231`). The deployment brief draws the only conclusion
that follows: this project's whole instrument set **is** the rollback strategy, front-loaded so that
being wrong blocks a release instead of requiring a correction after one. The mechanical consequence
for this story is an ordering, and it is not negotiable: every check that could have caught a mistake
runs *before* the first `cargo publish`, and the recovery route is written down *before* the act,
because after it there is nothing left to write it against.

**The version is derived from a report, never chosen.** Project AC-002 and DR-2 make the published
number the one `registry-surface-diff`'s committed report implies. This story reads that report; it
does not re-derive it and it does not round it. If the report implies something outside `0.2.0` —
a break the crate set was not meant to make — that is a **re-plan**, not a rounded-down version
number (`_storymap.md`:186-191). Stating it here so nobody meets that finding for the first time with
a publish command already typed.

**The number has to be in the tree before the first publish, in five places.** The alpha cut moved
`[workspace.package] version` (`Cargo.toml:6`) and all three `[workspace.dependencies]` requirement
strings (`Cargo.toml:24-26`) to the pre-release, plus both lock files (`Cargo.lock`,
`experiments/wire-format/Cargo.lock:69`). A requirement of `version = "0.2.0-alpha.1"` does not match
a `0.2.0` release any more than the reverse, and the failure surfaces on the *second*
`cargo publish`, by which time `happenstance-core` is live and unremovable. Every gate step passes
`--locked` (`xtask/src/main.rs:52-55`), so a lock file disagreeing with the manifests fails the gate
rather than the registry — that is the guard, and it only works if the edit lands before the gate
run, not between it and the publish.

**The gate that counts is the whole one, and the distinction is load-bearing rather than
ceremonial.** `--fast` is `REQUIRED` without `OPTIONAL`: it drops the two feature powersets,
`cargo deny` and the nightly `--cfg docsrs` rustdoc build (`xtask/src/main.rs:834-853`). Two of those
four are exactly what this release depends on. The nightly `--cfg docsrs` build is the only thing
that compiles the `doc(cfg)` attributes docs.rs will set — *"a cfg nobody sets except docs.rs, which
builds after publication"* (`xtask/src/main.rs:602-605`), so a broken one fails where it can no
longer be fixed. And the `wasm32` feature powerset is the only step that compiles
`happenstance-core`'s default `std + memory` combination on that target at all. Running `--fast` and
calling it the release bar would drop both, silently, and `.redkiln/config.yaml:55` fires `--fast`
automatically at this project's integration stage — so the mistake is one keystroke away and looks
identical in the log. That is why AC-TEST-004 asks for a *distinct, dated* artefact.

**AC-014 names a gap, and this story is where it gets closed.** The four mandatory `wasm32` steps
build `happenstance-core` with `--no-default-features --features std` (`xtask/src/main.rs:203-217`),
type-check the testkit's harnesses (`:231-243`), and build two `publish = false` adapters
(`:252-282`). None of them is the published crate at the published feature set. `happenstance`'s
default is `["std", "memory"]`, forwarded to `happenstance-core`
(`crates/happenstance/Cargo.toml:22-26`), and the crate's own README will tell a `wasm32` reader they
can self-identify from one line in the Guarantees block (`_design.md`, `### DT-1`). The fix is one
mandatory `Step`, not a new instrument. **If it does not compile, that is a finding, not a feature
edit**: it would mean the published default feature set does not admit the constrained runtime, which
is a claim `_design.md`'s `## What it costs a caller` makes in writing — so it blocks the release and
becomes a decision atom and a re-plan, exactly as DR-15 requires for a frozen clause found wrong.

**Five upstream artefacts must be re-observed at *this* commit, not remembered.** Every one of this
story's blockers produces a dated file, and a dated file proves something about the tree it was
written against:

- `crate-set-decision` — the atom recording three crates, and `package-check` reconciling
  `PUBLISHABLE` (`xtask/src/package.rs:86`) against what `cargo metadata` derives. Re-run, not cited.
- `registry-surface-diff` — the committed report against the `0.2.0-alpha.1` baseline, and the
  version it implies.
- `clause-maturity-audit` — `cargo xtask clause-audit --write --date <YYYY-MM-DD>`, whose own spec
  states plainly that *"`publish-0-2-0` writes the next one at the publish commit; that one, not this
  one, is the release's artefact of record"*.
- `deferred-clause-reread` — the ten dated deferral reasons, checked inside the same audit step.
- `rendered-page-preflight` — the dated read of the rendered pages, which `_storymap.md`:179-184
  makes a strict predecessor because reversibility must be bought *before* the act.

A precondition whose artefact names a different commit is a precondition that has not been observed
on the tree being published. Fail closed and say which one — the discipline is ADR-0010's *a skip is
reported, never silent* (`.kb/decisions/0010-the-suite-must-prove-itself.md`), which this project's
deployment brief already applies to its two new gate steps by name.

**The alpha is not yanked by this release, and that is a decision rather than an omission.** The
alpha's own README states the pre-release policy as *"only one pre-release resolves at a time; each
alpha is yanked when the next lands"* — a rule about an alpha superseding an alpha. A stable `0.2.0`
does not need it: Cargo does not select a pre-release for an ordinary requirement, so `0.2.0-alpha.1`
stops resolving the moment `0.2.0` exists. And it is the baseline AC-002's instrument diffs against.
Yanking it buys nothing and risks the one artefact this release's central check reads.

**`cargo publish --dry-run` is asymmetric, and the alpha already paid to learn it.** The dry run
packages the crate and builds it with **registry-form** dependencies, so
`cargo publish --dry-run -p happenstance` cannot succeed until `happenstance-core` is live at the
same number. Treat it as *unavailable*, never as *failing* — a dry run that cannot run is not
evidence of a defect, and mistaking the two is how a correct release gets abandoned mid-sequence.

**A half-published release is a resumable state, not a corrupt one.** If the registry fails partway —
rate limit, index lag, network — resume at the crate that failed. Never restart at a new number
because part of it succeeded: only what has already been published is frozen, and burning `0.2.1` to
escape a transient error spends a version number on nothing.

**The persona slice.** Backbone activity **A4**, *"take the one irreversible act, and check it from
outside"* (`_storymap.md`). This story has no user intent of its own — U7 belongs to
`stranger-install-smoke`, which is the only check that can only run *after* — and that is precisely
its shape: it is the moment every claim the `published-surface-copy` milestone wrote becomes a
promise Persona 4 will meet on a page nobody can edit. Getting the sequence wrong does not produce a
bug report; it produces a permanent first impression.

## Integration contract

| | |
| --- | --- |
| **Archetype** | `capability` — a maintainer takes one irreversible act, and afterwards a stranger's `cargo add happenstance` resolves. |
| **Slice / milestone** | `the-release-event`. Slice-mates, implemented in one context and mounted together: `rendered-page-preflight` (strictly before — `_storymap.md`:179-184) and `stranger-install-smoke` (strictly after; the only check that can only run post-publish). |
| **Mount point** | **`Cargo.toml`** — the release's composition root. `[workspace.package] version` (`:6`) is the single field that decides what number is published, and `[workspace.dependencies]`'s three requirement strings (`:24-26`) decide whether the published crates can resolve each other. Nothing is "mounted" for a release until those four lines say `0.2.0` and both lock files agree; that is what `--locked` turns into a gate failure rather than a registry failure. |
| **Second mount** | **`xtask/src/main.rs`** — the gate's composition root, for AC-014's step. Three edits in one change: one `Step` appended to `REQUIRED` (`:105`ff, `probe: None`, following the contract-crate `wasm32` entry at `:203-217`); its name added to `wasm_steps()` (`:784-790`) so `cargo xtask wasm` covers it too; and the module doc's *"What the gate proves"* paragraph (`:8-24`) — the one place in this repository that states in prose what the gate proves, which the deployment brief requires be updated in the same change as any mandatory addition (AC-DEP-005). |
| **Wires into** | `xtask/src/package.rs` — `PUBLISHABLE` (`:86`), `REQUIRED_FILES` (`:88-94`) and `reconcile`, reached through the mandatory `package-check` step (`xtask/src/main.rs:515-532`, dispatched at `:680`); `cargo xtask clause-audit --write --date <…>` and `spec/audits/`, from `clause-maturity-audit` and `deferred-clause-reread`; `registry-surface-diff`'s committed report and the version it implies; `CHANGELOG.md`; `Cargo.lock` and `experiments/wire-format/Cargo.lock`; `.redkiln/config.yaml:55, :60` (what fires automatically, and what deliberately does not). |
| **Renders surfaces** | **None newly authored — and all seven newly *resolving*.** `_design.md`'s `crates-io-*` and `docs-rs-*` `route` fields (`:104-169`) are explicit that they do not resolve until this story lands. The copy is `published-surface-copy`'s and the rendered read is `rendered-page-preflight`'s; this story changes no byte of any README and must not, because a surface edited after its preflight read is a surface nobody read. |
| **Public items** | None added. `_design.md`'s `## Items` block carries two entries and this story implements neither: the `unstable-projection` feature is `projection-port-ship-shape`'s and the `[package.metadata.docs.rs]` block is `guarantees-and-docs-rs-presentation`'s. This story *verifies* both are in the published artifacts and blocks if they are not. |
| **Conformance rule(s)** | **None, and it is not adapter-observable.** No rule in `crates/happenstance-testkit/src/suite.rs` can observe that a version exists on a registry. Stated explicitly because a story that names no rule must say which of the two it is. The equivalent obligation — an instrument that can fail — is discharged by the mandatory `wasm32` step, whose named wrong implementation is *a published default feature set that does not build on `wasm32`*. |
| **Clause(s)** | **Amends none, reads all 200.** The clause audit at this commit fingerprints all 139 `[FROZEN]` clauses against the tree this project received; a difference is a **stop** (AC-015, DR-15), a new decision atom and a re-plan, never an edit. |
| **Advances DoD scenario** | Initiative **DoD 13** — *"The gate is green on the assembled whole. `cargo xtask ci` passes, including the specification cross-reference step, on the exact tree that was published"* (`initiative.md`:396-397). It also makes **DoD 9** (a stranger installs it) reachable for the first time, and supplies the tree that **DoD 10, 11 and 12** are read against. |

## PR boundary

The paths this story may touch. `redkiln verify --grain story` reads the first fenced block under
this heading and fails on any file changed outside it.

```
Cargo.toml
Cargo.lock
experiments/wire-format/Cargo.lock
CHANGELOG.md
xtask/src/main.rs
spec/audits/**
.bklg/from-contract-to-published-library/publication-and-positioning/publish-0-2-0/**
```

**In this PR**

- `Cargo.toml` — `[workspace.package] version` (`:6`) and the three `[workspace.dependencies]`
  requirement strings (`:24-26`) set to the version the surface-diff report implies, before any gate
  run and long before any publish.
- `Cargo.lock`, `experiments/wire-format/Cargo.lock` — regenerated so `--locked` holds.
- `CHANGELOG.md` — `## [Unreleased]` becomes `## [0.2.0] — <date>`, with a fresh empty
  `## [Unreleased]` above it and the link references updated.
- `xtask/src/main.rs` — the AC-014 `Step`, its `wasm_steps()` entry, and the module-doc sentence.
- `spec/audits/clause-maturity-<publish-date>.md` — the audit re-run at this commit, committed. This
  one, not `clause-maturity-audit`'s first report, is the release's artefact of record.
- `.bklg/.../publish-0-2-0/_release-log.md` — the dated record: preconditions re-observed, the full
  gate transcript, the dry runs, the rollback route (written **before** the publish transcripts), the
  publish transcripts in order, the tag, the resolution check, and the docs.rs observation.

**Explicitly not in this PR**

- **Any README, rustdoc or manifest-`description` copy.** `landing-copy-and-status-truth`,
  `compliance-claim-and-gaps-promise` and `guarantees-and-docs-rs-presentation` own it, and
  `rendered-page-preflight` has already read the rendered result. A copy edit here silently invalidates
  that read.
- **`crates/happenstance/Cargo.toml`'s `[package.metadata.docs.rs]` block** —
  `guarantees-and-docs-rs-presentation`'s. Verified present here; not authored here.
- **Stripping `provisional` from `.kb/decisions/0004-edition-and-msrv.md`**, which `RUNBOOK.md:4475`
  instructs. It is an accepted, immutable atom and `redkiln validate --kb` checks accepted atoms
  against `HEAD`; the MSRV promise is `msrv-promise-atom`'s **new** atom (project DR-6). The runbook
  line is stale, like `RUNBOOK.md:4450-4451`'s four-crate goal and `:4495`'s `#the-46-provisional-clauses`
  anchor — this story follows the briefs, and reports the staleness rather than acting on it.
- **Repointing `cargo-semver-checks` to keep both baselines** (`RUNBOOK.md:4476`) —
  `registry-surface-diff`'s.
- **The status table, the badge rows and the "batteries" tagline** (`RUNBOOK.md:4477-4478`) —
  `landing-copy-and-status-truth`'s.
- **The stranger-install smoke** — `stranger-install-smoke`, the slice-mate that runs after and is
  this project's proof artefact. This story's own resolution check is deliberately narrower: *does the
  index serve all three at this number*, not *does a stranger succeed*.
- **Publishing `happenstance-sqlite` or any adapter.** The set is three, decided by
  `crate-set-decision`; `package-check` fails if the tree disagrees.

**Merge DoD.** The full `cargo xtask ci` — not `--fast` — is green on the literal publish commit and
its dated transcript is committed; all three crates resolve from crates.io at the published number
with no unresolvable internal dependency; `_release-log.md` carries the rollback route dated *before*
the publish transcripts; and the new `wasm32` step is in `REQUIRED`, in `wasm_steps()`, and named in
the module doc.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| The version is read from a report, never chosen | The published number is the one `registry-surface-diff`'s committed dated report implies. This story quotes the report's finding and the number in `_release-log.md` before editing a manifest. A report implying anything outside `0.2.0` **halts the story** and becomes a re-plan; there is no rounding rule and no default. | `project.md` AC-002 / DR-2; `_storymap.md`:186-191 |
| The number lands in the tree in five places, before the gate runs | `Cargo.toml:6` (`[workspace.package] version`), the three requirement strings at `Cargo.toml:24-26`, `Cargo.lock` and `experiments/wire-format/Cargo.lock`. Verified with `cargo metadata --format-version 1` showing **no** internal requirement carrying the alpha's pre-release, transcript in `_release-log.md`. The ordering is the point: edit, then gate, then publish — a tree gated before this edit is not the tree being published. | `Cargo.toml`:6, :24-26; `experiments/wire-format/Cargo.lock`:69; `publish-0-2-0-alpha-1/spec.md` AC-001 and EC-001 |
| Five preconditions are re-observed at this commit | For each of `crate-set-decision`, `registry-surface-diff`, `clause-maturity-audit`, `deferred-clause-reread` and `rendered-page-preflight`: the artefact exists, names a commit, and that commit is an ancestor of this one with no intervening change to what it observed. A missing, unreadable or stale artefact **fails closed, naming which one and what to do** — never an empty check that passes. | `.kb/decisions/0010-the-suite-must-prove-itself.md`; `_decomposition.md`, deployment brief *CI implication* |
| The clause audit is re-run here and its report committed | `cargo xtask clause-audit --write --date <YYYY-MM-DD>` at the publish commit, writing `spec/audits/clause-maturity-<date>.md`. The date is an argument, not a clock read. Any `[FROZEN]` fingerprint difference is a **stop**: a decision atom and a re-plan, never an edit and never `--rebaseline`. | `clause-maturity-audit/spec.md` (`--write`/`--rebaseline` contract; *"`publish-0-2-0` writes the next one at the publish commit"*); `project.md` AC-015 / DR-15 |
| The whole gate runs on the literal publish commit | `cargo xtask ci`, no flags. Its full output — every step name, including the four `wasm32` steps, `spec-trace`, `package-check`, and the optional steps that must **run** rather than print `skipped` — is committed to `_release-log.md` with the commit SHA and the date. It is a distinct artefact from any `--fast` run recorded at the project's integration stage. | `xtask/src/main.rs:8-24`, `:834-853`; `.redkiln/config.yaml:55`, `:60`; `_decomposition.md`, testing brief AC-TEST-004 |
| `--fast` is not the release bar, and the reason is named | `--fast` is `REQUIRED` without `OPTIONAL`: it drops the two feature powersets, `cargo deny` and the nightly `--cfg docsrs` rustdoc build. Two of the four are load-bearing here — the `docsrs` build is the only thing that compiles the `doc(cfg)` attributes docs.rs will set *after* publication, and the `wasm32` powerset is the only step that compiles `happenstance-core`'s default `std + memory` on that target. A transcript showing `skipped` for either is not a passing release gate. | `xtask/src/main.rs:558-593`, `:600-612`, `:834-853`; `project.md` DoD 5 |
| A fifth mandatory `wasm32` step: the installed crate at its published default features | `cargo check --locked -p happenstance --target wasm32-unknown-unknown` appended to `REQUIRED`, added to `wasm_steps()`, and named in the module doc. It closes a real hole: today nothing in the gate — mandatory or optional — ever names `happenstance` on that target, so the crate a consumer installs, at the features they get by default, has never been type-checked where `Send` is unavailable. Named wrong implementation: a published default feature set that does not build on `wasm32` while the Guarantees block tells a `wasm32` reader it does. | `xtask/src/main.rs:203-282` (the four that exist), `:558-593` (the powerset that also omits it), `:784-790`; `crates/happenstance/Cargo.toml:22-26`; `_design.md`, `## What it costs a caller` |
| If that step fails, it blocks — it does not get a feature edit | A default set that cannot build on `wasm32` contradicts a claim `_design.md` puts on a published page. The response is a decision atom and a re-plan, matching DR-15's posture for a frozen clause found wrong, not a quiet change to `default` to make the step green. | `project.md` DR-14, DR-15; `.kb/decisions/0001-async-port-flavours.md` |
| No stub survives into a published crate | `clippy::todo` is already `deny` workspace-wide (`Cargo.toml:128`) and the six scoped `#![allow(clippy::todo)]` exemptions all sit in `publish = false` skeletons. This story asserts, at the publish commit, that none of the three published crates carries such an exemption — the check RUNBOOK phase 12 asks for, reduced to what is actually still open. | `Cargo.toml`:124-128; `crates/happenstance-sqlite/src/lib.rs`:76-80 (the exemption's documented shape); `RUNBOOK.md`:4472-4473 |
| `CHANGELOG.md` gains a dated `## [0.2.0]` section | `## [Unreleased]` becomes `## [0.2.0] — <date>`; a fresh empty `## [Unreleased]` goes above it; link references updated. The file has accumulated since phase 0 rather than being reconstructed from `git log`, and that property is preserved — entries are re-headed, not rewritten. | `CHANGELOG.md`:1-25; `RUNBOOK.md`:4466 |
| Dry runs before the act, with one asymmetry stated | `cargo publish --dry-run -p happenstance-core` and `-p happenstance-testkit` are run and their transcripts committed. `cargo publish --dry-run -p happenstance` **cannot** run until `happenstance-core` is live at this number, because the dry run builds against registry-form dependencies. Treat it as unavailable, never as failing. | `publish-0-2-0-alpha-1/spec.md`, *Implementation notes* and AC-001; `CONTRIBUTING.md`:47-49 |
| Publish order follows the dependency graph, and is resumable | `happenstance-core` → `happenstance-testkit` → `happenstance`; the testkit is unordered with `happenstance` once core is live, since both depend only on core. A human handoff, not an automated step. A registry failure partway is resumed at the crate that failed — never restarted at a new number, because only what is already published is frozen. | `RUNBOOK.md`:4469-4471; `crates/happenstance-testkit/Cargo.toml`:29 and `crates/happenstance/Cargo.toml`:20 (both depend on core, neither on the other); `publish-0-2-0-alpha-1/spec.md` EC-002 |
| The rollback route is written before the act | `_release-log.md` carries, dated ahead of the first publish transcript: what `yank` does and does not do, that a correction ships as a new forward version rather than an edit, and that yanking is reserved for a broken artefact rather than for wording. Written first because after the act there is nothing left to write it against. | `standards/rust/51-features-and-no-std.md`:226-231; `RUNBOOK.md`:4481-4484; `_decomposition.md`, deployment brief *Migration, backfill and rollback posture* |
| `0.2.0-alpha.1` is not yanked, deliberately | Cargo does not select a pre-release for an ordinary requirement, so the alpha stops resolving as soon as `0.2.0` exists; and it is the baseline AC-002's instrument diffs against. The alpha's *"each alpha is yanked when the next lands"* is a rule about alphas superseding alphas. Recorded as a decision in `_release-log.md` so a later reader does not read the absence as an oversight. | `publish-0-2-0-alpha-1/spec.md` AC-004 (the stated yank policy); `project.md` AC-002 |
| Tag and GitHub release, after the crates are live | An annotated tag on the publish commit and a GitHub release whose body points at `CHANGELOG.md`'s new section. After, not before: a tag on a commit whose publish failed is a tag that has to be moved, and there are no tags in this repository yet to inherit a convention from — the one chosen is recorded in `_release-log.md`. | `RUNBOOK.md`:4474 |
| The post-publish resolution check | From outside any lock file this repository owns: all three crates resolve at `0.2.0` from the index and no internal dependency is unresolvable. Narrower than `stranger-install-smoke` on purpose — this asks whether the *index* is right, which is the last thing this story can still be wrong about; whether a *stranger* succeeds is the slice-mate's, and it is the proof artefact. | `publish-0-2-0-alpha-1/spec.md` AC-007; `_storymap.md` (`stranger-install-smoke`) |
| docs.rs is observed, and its failure is routed rather than yanked | The three docs.rs builds are checked within the hour and the result recorded. A failed docs.rs build is **not** a blocker and **not** a yank: yanking a correct crate because its documentation renderer failed spends a yank on nothing. It routes to `guarantees-and-docs-rs-presentation`'s configuration and to project AC-007. | `publish-0-2-0-alpha-1/spec.md` EC-006; `project.md` AC-007 / DR-7 |
| No step in the gate reaches a registry | Every registry-touching act in this story is manual and recorded, because `cargo xtask ci` must stay runnable offline and hermetic. That is why the evidence is a committed transcript rather than a green step. | `_decomposition.md`, testing brief *Fixtures and seams to mock*; `xtask/src/package.rs`:69-74 (`--allow-dirty`, on the gate running against uncommitted trees) |

## Data and migrations

**No database, no schema, no runtime data.** This library defines no storage schema of its own —
`MemoryEventStore` and the adapters under construction each own theirs — and this is the first stable
publish of these three names, so there is no prior production data shape for any consumer to migrate
and nothing to backfill. The three names currently sit at `0.0.0` placeholders, and Cargo treats
every `0.0.x` version as incompatible with every other, so there is no compatible predecessor a
consumer could have depended on (`CONTRIBUTING.md`:296-300; `xtask/src/reserve.rs`:12-30). That is
AC-DEP-004's *N/A with the reason stated*, recorded here rather than left implicit.

**The one migration-shaped thing is a version string, and it is the highest-cost mistake available.**

| Artefact | Path | Lifecycle |
| --- | --- | --- |
| The version root | `Cargo.toml`:6 | Moves from the alpha's pre-release to `0.2.0` in this PR, before the gate run. One field; every crate inherits it except the testkit, which carries its own number under CF-32 (`crates/happenstance-testkit/Cargo.toml`:4-14) and moves on its own schedule. |
| The internal requirement strings | `Cargo.toml`:24-26 | Move in the same edit. A requirement that does not match the published number fails on the *second* `cargo publish`, after the first crate is live and unremovable. `cargo metadata` verification is a criterion, not a step. |
| Both lock files | `Cargo.lock`, `experiments/wire-format/Cargo.lock`:69 | Regenerated in the same commit. Every gate invocation passes `--locked`, so a lock file disagreeing with the manifests fails the gate rather than the registry — the guard only works if the edit precedes the gate run. |
| The release's audit of record | `spec/audits/clause-maturity-<publish-date>.md` | Written once, at the publish commit, never rewritten. A snapshot, not a mirror: no check reads it back, so a stale report cannot make a gate green. |
| The release log | `.bklg/.../publish-0-2-0/_release-log.md` | Append-only within the story, and ordered: preconditions → gate transcript → dry runs → **rollback route** → publish transcripts → tag → resolution check → docs.rs observation. The ordering is the evidence that the recovery route was written before the irreversible act, not reconstructed after it. |

**Rollback.** Every edit above is an ordinary commit and reverts cleanly — right up until the first
`cargo publish`, after which nothing does. A yank removes the version from future resolution and
leaves every rendered page exactly as it was
(`standards/rust/51-features-and-no-std.md`:226-231), so the posture is front-loaded by design: the
surface diff, the clause audit, the rendered-page read and the full gate are the rollback, and they
all run before the act. A defect found afterwards ships as a new forward-dated version carrying a
changelog entry naming what broke — never an edit in place, never a re-publish of the same number,
which the registry does not offer.

## Acceptance criteria

Eight criteria, each written from the goal of a person this initiative named rather than from the
capability that satisfies it. **P4** is *the evaluator (pre-adoption)* — one-shot and time-boxed in
a way the other three personas are not, with no second pass to correct a first impression, and
unable to run our suite (`_discovery/distillation/personas-and-journeys.md`:249-313, `:262-266`,
`:333-338`). **P1** is the application author choosing a contract before a database. **P2** is the
adapter author who pins the conformance suite exactly. **P3** is the constrained-runtime developer
checking whether `wasm32` is a first-class target or a footnote (`initiative.md`:203-220), whose
intent is **U5** — *"tell me whether it runs where I run"* (`_decomposition.md`, UX brief). The
maintainer standing behind all four is the actor of backbone activity **A4** — *"take the one
irreversible act, and check it from outside"* (`_storymap.md`:43).

Evidence no gate step can reach — a registry resolution, a `cargo publish` transcript, a docs.rs
build observed at a particular minute — is recorded in this story's own folder as `_release-log.md`,
which the PR boundary already admits. That is the shape the deployment brief set and the testing
brief repeated: AC-016 *"sits outside that automatic wiring on purpose… the proof artefact is the
committed gate output, not a passing CI badge from an earlier commit"* (`_decomposition.md`, testing
brief, *Merge-gate commands*).

| id | criterion | verification |
| --- | --- | --- |
| **AC-001** | **GIVEN** P4 has decided to depend and types `cargo add happenstance`, and P1 will inherit whatever number resolves, **WHEN** the maintainer prepares the tree for release, **THEN** the version is the one `registry-surface-diff`'s committed dated report *implies* — quoted into `_release-log.md` with the report's finding **before** a manifest is touched, never chosen and never rounded — and it lands in **five** places in one edit that precedes the gate run: `Cargo.toml`:6, all three `[workspace.dependencies]` requirement strings at `Cargo.toml`:24-26, `Cargo.lock` and `experiments/wire-format/Cargo.lock`; **AND** `CHANGELOG.md`'s `## [Unreleased]` becomes `## [0.2.0] — <publish date>` with a fresh empty `## [Unreleased]` above it and the link references moved, dated with the day the publish runs so no one-line edit moves the tree afterwards | Static: `cargo metadata --format-version 1` over the release tree shows **no** internal requirement carrying the alpha's pre-release — transcript in `_release-log.md`. Gate: every step runs `--locked` (`xtask/src/main.rs`:52-55), so a lock file disagreeing with the manifests fails the gate rather than the registry. Lint: `cargo xtask lints`' `changelog_names_every_rule` (CF-29) survives the heading rename, proving no entry was orphaned or merged. Human: the report's quoted finding and the derived number appear in `_release-log.md` above the first manifest edit |
| **AC-002** | **GIVEN** five blocking stories each produced a dated artefact about the tree *they* were written against, and P4 will read the result of all five on a page nobody can edit, **WHEN** the maintainer reaches the publish commit, **THEN** each of `crate-set-decision`, `registry-surface-diff`, `clause-maturity-audit`, `deferred-clause-reread` and `rendered-page-preflight` is re-observed *here* — the artefact exists, names a commit, and that commit is an ancestor of this one with no intervening change to what it observed — **AND** the tree's own two preconditions hold: `package-check`'s `reconcile` agrees with `cargo metadata` on exactly the three decided crates, and **none of the three published crates carries a `#![allow(clippy::todo)]` exemption**; a missing, unreadable or stale precondition **fails closed naming which one and what to do**, never an empty check that passes | Gate: `cargo xtask ci`'s `package-check` step (`xtask/src/main.rs`:515-532, dispatched `:680`) over `PUBLISHABLE` (`xtask/src/package.rs`:86) — EC-005. Static: a repository-wide read for `allow(clippy::todo)` showing every hit inside a `publish = false` skeleton (`crates/happenstance-sqlite/src/lib.rs`:76-80 is the documented shape). Human, recorded: a five-row precondition table in `_release-log.md`, one row per blocker, each carrying the artefact path, its stated commit and the ancestry check — written **before** the gate run. The fail-closed discipline is `.kb/decisions/0010-the-suite-must-prove-itself.md`'s *a skip is reported, never silent* |
| **AC-003** | **GIVEN** P4 cannot run our suite and has only what the specification says about how strong each promise is (**U4**), **WHEN** the release is cut, **THEN** `cargo xtask clause-audit --write --date <YYYY-MM-DD>` is re-run **at the publish commit** and `spec/audits/clause-maturity-<publish-date>.md` is committed as the release's artefact of record — the date passed as an argument, not read from a clock — reporting every clause's maturity across all 200 rather than only the frozen ones, and **every `[FROZEN]` fingerprint identical** to the tree this project received; any difference **stops the release** and becomes a decision atom and a re-plan, never an edit and never `--rebaseline` | Gate: the mandatory clause-audit step `clause-maturity-audit` added, whose own `#[cfg(test)] mod tests` (the `xtask/src/package.rs`:408-457 shape) already carries a seeded §1.3 disagreement and a seeded frozen-clause edit it must fail on (`_storymap.md`, `clause-maturity-audit` row). Artefact: the committed report, whose totals reconcile against `spec/SPECIFICATION.md`:219-221's hand-computed 200 / 198 / 139 / 49 / 10 / 2. EC-007 governs a difference |
| **AC-004** | **GIVEN** initiative DoD 13 asks for the gate green *"on the exact tree that was published"* (`initiative.md`:396-397), and P2 will pin this exact release, **WHEN** the maintainer runs the release bar, **THEN** the **whole** `cargo xtask ci` — no flags — is green on the literal publish commit and its **dated** output is committed to `_release-log.md` beside `git rev-parse HEAD`, as an artefact **distinct from** any `cargo xtask ci --fast` run `.redkiln/config.yaml`:55 fired at the project's integration stage; **AND** no `OPTIONAL` step in that transcript reports `skipped` — the two feature powersets, `cargo deny` and the nightly `--cfg docsrs` rustdoc build must have *run*, because the `docsrs` build is the only thing that compiles the `doc(cfg)` attributes docs.rs will set after publication and the `wasm32` powerset is the only step that compiles `happenstance-core`'s default `std + memory` on that target | Release gate: `run_ci` whole (`xtask/src/main.rs`:828-834), whose sibling `run_fast`'s own doc comment says it unprompted — *"the project-scoped bar, not the release bar"* (`:834-853`). Artefact: the transcript in `_release-log.md` with the SHA, satisfying AC-TEST-004. Regression: the four existing `wasm32` steps (`:203-282`) are inside that run, discharging AC-DEP-003. EC-011 governs any edit after the run |
| **AC-005** | **GIVEN** P3 reads one line in the Guarantees block to self-identify as a `wasm32` consumer and then installs `happenstance` at the features they get by default, **WHEN** the gate runs on the release tree, **THEN** a **fifth mandatory** `wasm32` step compiles *that* crate at *those* features — `cargo check --locked -p happenstance --target wasm32-unknown-unknown` appended to `REQUIRED` with `probe: None`, its name added to `wasm_steps()` so `cargo xtask wasm` covers it, and the module doc's *"what the gate proves"* paragraph updated in the same change; **AND** if it does not compile, the release **blocks** and it becomes a decision atom and a re-plan — never a quiet edit to `default` to make the step green | Gate: `cargo xtask ci` and `cargo xtask wasm` both list the new step by name; `steps_named` (`xtask/src/main.rs`:816) resolves names against `REQUIRED`, so a mis-registered step is a hard failure rather than a silent omission. Named wrong implementation it rejects: **a published default feature set that does not build on `wasm32` while the Guarantees block tells a `wasm32` reader it does** — the hole is real today, since neither the four mandatory steps (`:203-282`) nor the optional powerset (`:558-593`) ever names `happenstance`. Doc: the module-doc sentence (`:8-24`), which AC-DEP-005 requires move with any mandatory addition. EC-004 governs a failure |
| **AC-006** | **GIVEN** P4 will meet these three crates on a rendered page that cannot be edited after the fact, and `rendered-page-preflight` has already read those pages, **WHEN** the `.crate` artifacts are built, **THEN** what ships **is** what was read: each published crate's packaged `README.md` is **byte-identical** to the file preflight read, both licence files and the README are inside every tarball, all three manifests carry a `[package.metadata.docs.rs]` block so docs.rs builds under all features with `doc(cfg)` on every gated item, the `happenstance` manifest `description` is **≤ 120 characters** and true of *this* tree (today's is ~131 and claims typed events, decision models and projection runners — `crates/happenstance/Cargo.toml`:3), and **none** of the four known-stale strings appears on any packaged surface; this story changes **no byte** of any README, because a surface edited after its preflight read is a surface nobody read | Gate: the `package-check` step's `REQUIRED_FILES` assertion over each `.crate` (`xtask/src/package.rs`:88-94). Static: a hash of each packaged `README.md` against the preflighted file, recorded in `_release-log.md`; a grep for the four stale strings (`_design.md`, AP-9) over the tarball contents; a character count of `description`. Human review against the signed-off `_design.md` — `## Surfaces` (`:104-169`), the density budget's per-item table (`:439-499`) and the anti-patterns (`:641-680`). Every machine check here is satisfied by a page carrying the right words in the wrong composition; this row is why the review is named as an instrument |
| **AC-007** | **GIVEN** a crates.io publish cannot be edited or deleted and a `yank` removes a version from future resolution while leaving every rendered page exactly as it was, **WHEN** the maintainer takes the act, **THEN** reversibility is bought **first and in writing**: `cargo publish --dry-run` is run and its transcript committed for `happenstance-core` and `happenstance-testkit` (the `-p happenstance` dry run is recorded as **unavailable**, never as failing, because it builds against registry-form dependencies and cannot resolve until core is live); the rollback route — what `yank` does and does not do, that a correction ships as a new forward `0.(2+n).0` rather than an edit, and that yanking is reserved for a broken artefact rather than for wording — is written into `_release-log.md` **dated ahead of the first publish transcript**; then `happenstance-core` → `happenstance-testkit` → `happenstance` in dependency order, as a human handoff; a registry failure partway is **resumed at the crate that failed**, never restarted at a new number; and `0.2.0-alpha.1` is **deliberately not yanked**, recorded as a decision with its reason so a later reader does not read the absence as an oversight | Artefact: `_release-log.md`'s ordering is itself the evidence — dry runs, then rollback route, then publish transcripts, in that file order with dates. Static: `crates/happenstance-testkit/Cargo.toml`:29 and `crates/happenstance/Cargo.toml`:20 both depend on core and neither on the other, which is what makes the last two unordered. EC-001, EC-002 and EC-010 govern the three ways this goes wrong |
| **AC-008** | **GIVEN** the maintainer has just done something they cannot undo, and P4's whole first impression now lives on pages nobody in this repository controls, **WHEN** the crates are live, **THEN** the act is checked **from outside**: from a scratch directory owning none of this repository's lock files, all three crates resolve at the published number from the index with **no unresolvable internal dependency**; an annotated tag on the publish commit and a GitHub release whose body points at `CHANGELOG.md`'s new section land **after** the crates are live, never before, with the tag convention recorded because this repository has none to inherit; and the three docs.rs builds are observed within the hour and recorded — a failed docs.rs build is **not a blocker and not a yank**, it routes to `guarantees-and-docs-rs-presentation`'s configuration and to project AC-007 | Manual, registry: transcripts in `_release-log.md`, run from outside the workspace. Deliberately **narrower** than `stranger-install-smoke`: this asks whether the *index* is right — the last thing this story can still be wrong about — while whether a *stranger succeeds* is the slice-mate's and is this project's proof artefact (`_storymap.md`, `stranger-install-smoke` row). NF-004 sets the window and the reason. EC-009 governs a docs.rs failure |

**Coverage of the traced project ACs.** Project **AC-001** (*published, for a decided crate set*) —
story AC-002 (the set reconciles here, not only where it was decided), AC-007 (the act) and AC-008
(it actually resolves). Project **AC-014** (*the constrained-runtime flavour survives*) — story
AC-004 (the four existing `wasm32` steps run on the published tree, AC-DEP-003) and AC-005 (the
published crate at the published feature set, which nothing checked before), with AC-006 keeping the
claim that says so honest. Project **AC-016** (*the full gate is green on the exact tree that was
published*) — story AC-001 (the tree is the release tree before it is gated), AC-003 and AC-004.

## Interaction quality

This story **authors no surface and renders seven**. `_design.md`'s `route` fields — the crates.io
and docs.rs URLs for all three crates — *"do not resolve until `publish-0-2-0` lands"*
(`_design.md`:104-111). So the composition family is binding here in an unusual and strict form:
this story does not compose anything, and its obligation is that **what was composed and read
survives into the artifact unchanged**, plus the one composed value it is still the last chance to
get right — the manifest `description`, which is published, unfixable, and the only text crates.io
shows in a search result (`_design.md`:460-464).

Every invariant below is carried by an **AC-### row in the table above**, never by a bullet here.
`redkiln verify` extracts acceptance criteria by matching a leading `| AC-001 |` table cell or a
`- AC-001:` bullet — a composition invariant living as prose in this section reaches no ledger row,
is never gated, and is never tested.

**State family.**

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **In place, not a context jump** (IQ-1) | AC-006, AC-008 | P4 answers *"is this for me"* on the page they landed on; evidence links deepen a claim and never carry it, at **0 hops to read a claim, ≤ 1 hop to its evidence**. The publish must not manufacture a hop: a relative link that dies at the packaging boundary renders as a 404 for every reader of that version, permanently (AP-6). Verified by the tarball read at AC-006 and the rendered observation at AC-008 |
| **Non-occlusion** (IQ-2) | AC-003, AC-005, AC-006 | Two forms, both structural rather than cosmetic. *Maturity annotates, never subtracts*: the audit reports all 200 clauses, so no surface can present 139 frozen guarantees while the 49 provisional ones quietly disappear (AP-4). *Feature gating annotates, never subtracts*: the docs.rs block builds `all-features` with `doc(cfg)`, so a gated item ships **with its gate** rather than absent (AP-8) — and AC-005 is the check that the published default set does not silently exclude the target it claims |
| **Preserved position, focus and selection** | AC-006 | The medium's true analogue is the **inbound anchor** somebody already holds — `README.md`:9 → `#licence`, `README.md`:16 → `#status` — and the reader's place in a page they may return to. This story preserves both by the strongest available mechanism: it changes **zero bytes** of any published surface after preflight, and byte-identity is asserted rather than assumed. Focus and scroll have no analogue in a rendered markdown page with no interactive state; their intent — *the reader's place survives the change* — is what byte-identity discharges |
| **Reversibility** (IQ-4) | AC-007, and bought by AC-001, AC-002, AC-003, AC-004, AC-006 | Every edit in this PR is an ordinary commit and reverts cleanly. **Exactly one act does not**, so reversibility is discharged by writing the recovery route down *before* the act — dated ahead of the first publish transcript in the same file — and by front-loading every instrument that could have caught the mistake. The deployment brief states the conclusion plainly: this project's whole instrument set *is* the rollback strategy |
| **Reachable without running anything** (IQ-5) | AC-004, AC-003, AC-008 | No claim this release makes may have *"run our CI"* as its only evidence — P4 cannot run the suite. Each becomes a **committed artefact**: the dated full-gate transcript, the clause-audit report, the registry resolution and the docs.rs observation. A terminal that has scrolled is not evidence |
| **Keyboard reachability** | **N/A, stated so the silence is not read as an oversight** | There is no interactive control on any of the seven surfaces; every one is host-rendered markdown or rustdoc. The accessibility floor that *does* apply to these pages is AC-UX-006's and it was discharged at `rendered-page-preflight`, which this story lists as a re-observed precondition (AC-002) rather than re-running |

**Composition family** — from the signed-off `_design.md`, which this story implements and does not
re-decide.

| Invariant | Design source | Carried by |
| --- | --- | --- |
| **Presentation exists at all** | `_design.md`:104-169 (`## Surfaces`), `xtask/src/package.rs`:88-94 | AC-006. A `.crate` carrying correct metadata and no composed page satisfies every static check in this repository and gives P4 nothing to read. Each tarball must contain the README *and* both licences, and each manifest the `[package.metadata.docs.rs]` block without which the docs.rs surface renders from defaults rather than from all-features with `doc(cfg)` |
| **Composition and placement** | `_design.md`:347-405 (`## Composition`), `:104-111` (the ordering is load-bearing — `crates/*/README.md` first, the root README **third**) | AC-006. The composed order is preserved by the only mechanism available to a release story: nothing is touched. `include_str!` cannot reach outside a package, so copy written only at the repo root never ships — a "small fix" applied here to the root README would not reach the pages P4 sees and would still invalidate the preflight read |
| **Transience** | `_design.md`:406-438 | AC-006 + AC-008. The policy — persistent chrome above the fold, revealed on scroll, opened on demand at exactly one hop — is a property of pages this story must not perturb. What this story *does* change is that the "opened on demand" targets become reachable at all: the `route` fields resolve only after the act, which is precisely what AC-008 checks |
| **Density budget, with its real numbers** | `_design.md`:439-499 | AC-006. The binding viewport is **1024×768 ≈ 14 rendered lines**, and the first-screen allocation is exactly 14 (identity 3 + blank 1 + callout 3 + blank 1 + triad 6). This story is the last owner of exactly one figure in that table — `description` **≤ 120 characters**, one sentence, true standalone — because it is published in the manifest and unfixable afterwards. The rest (quick-start fence ≤ 20 source lines, Guarantees ≤ 7 bullets, compliance block ≤ 6 lines, census ≤ 3 lines) were checked at `rendered-page-preflight` and are re-observed, not re-measured (AC-002) |
| **Hierarchy** | `_design.md`:500-523, `:599-616` | AC-006. What a `cargo add happenstance` reader meets first — H1 and one identity line, a dated status callout, a three-way "which crate do I want" — is the hierarchy this release freezes into a page nobody can edit. Byte-identity is what keeps it |
| **Named anti-patterns** | `_design.md`:641-680 | AC-006 carries **AP-9** (any of the four known-stale strings on a published page), **AP-6** (a relative link or a 404 on a packaged page), **AP-11** (the 8-row status table on a packaged crates.io page) and **AP-7** (raw HTML, colour or meaning-carrying images). AC-005 and AC-006 together carry **AP-8** (a gated item absent rather than `doc(cfg)`-annotated). AC-003 carries **AP-4** (frozen guarantees listed with no mention of the 49 provisional) and **AP-3** (a maturity count with no date or version — the audit report is dated by argument) |

**Not applicable, and why.** AP-1, AP-2, AP-5, AP-10, AP-12, AP-13, AP-14 and AP-15 are properties
of *copy this story does not write*: they belong to `landing-copy-and-status-truth`,
`compliance-claim-and-gaps-promise` and `guarantees-and-docs-rs-presentation`, and their rendered
observation belongs to `rendered-page-preflight`. Listing them here as this story's obligations would
duplicate a bar already gated elsewhere and, worse, invite a copy edit inside a release commit — the
one change this story must not make.

## Error conditions

| id | Condition | Required behaviour |
| --- | --- | --- |
| **EC-001** | `cargo publish -p happenstance` fails on an unresolvable `happenstance-core` because a `[workspace.dependencies]` requirement string still reads the alpha's `0.2.0-alpha.1` — **after** `happenstance-core@0.2.0` is already live and unremovable | The published core is not wrong; leave it. Fix `Cargo.toml`:24-26, **re-run the full gate** (the tree that was gated is no longer the tree being published — AC-004, NF-003), then publish `happenstance` at the **same** number. Do **not** bump to `0.2.1` to escape the mistake: only what has already been published is frozen, and burning a version number to escape a fixable edit spends it on nothing |
| **EC-002** | `cargo publish` fails midway on a registry error — rate limit, network, index lag — with one or two of the three crates live | Resume the sequence at the crate that failed. A half-published release is a **resumable state, not a corrupt one**. Record the failure and the resumption in `_release-log.md` with times, because index lag is the one failure that looks identical to a defect |
| **EC-003** | `cargo deny check` fails during AC-004's full gate — a licence, an advisory or a duplicate major | **The cut does not happen.** A licence violation found after publish is a legal problem rather than a follow-up commit. This is one of the two steps `--fast` drops, which is the whole reason the release bar is the full gate |
| **EC-004** | The new mandatory `wasm32` step (AC-005) fails: `happenstance` at its **published default** features does not compile for `wasm32-unknown-unknown` | **Block the release.** It means the published default feature set does not admit the constrained runtime while `_design.md`'s `## What it costs a caller` says in writing that both flavours cost identically. Write a decision atom and re-plan, matching DR-15's posture for a frozen clause found wrong. Do **not** edit `default` in `crates/happenstance/Cargo.toml` to make the step green — that changes the artefact being cut and moves the tree out from under the gate run |
| **EC-005** | `package-check`'s `reconcile` fails: the set derived from `cargo metadata` disagrees with `PUBLISHABLE` (`xtask/src/package.rs`:86) | The cut does not happen. This fires when a skeleton lost its `publish = false` — the one bug no other gate step would notice — and publishing through it ships a `todo!()` crate to strangers under a name this project reserved |
| **EC-006** | One of the five upstream artefacts is missing, unreadable, or names a commit that is not an ancestor of the publish commit | **Fail closed, naming which one and what to do**, and stop. Do not proceed on a remembered observation: a dated file proves something about the tree it was written against, and the whole point of AC-002 is that this tree is a different one. The stated-reason discipline is `.kb/decisions/0010-the-suite-must-prove-itself.md`'s |
| **EC-007** | The clause audit reports a `[FROZEN]` fingerprint differing from the tree this project received | **Stop the release.** It becomes a new decision atom and a re-plan (AC-015, DR-15). Never an edit to the clause, and never `--rebaseline` to make the audit agree — the rebaseline arm exists for a deliberate, separately-decided move, not for making a release date |
| **EC-008** | `registry-surface-diff`'s report implies a version outside `0.2.0` — a break the crate set was not meant to make | **Halt the story and re-plan** (`_storymap.md`:186-191). There is no rounding rule and no default; the version follows the report. Nobody should meet this finding for the first time with a publish command already typed, which is why it is stated in the *Context pack* as well as here |
| **EC-009** | A docs.rs build for one of the three fails after publish | **Not a blocker and not a yank.** Record it in `_release-log.md` within the hour and route it to `guarantees-and-docs-rs-presentation`'s `[package.metadata.docs.rs]` configuration and to project AC-007. Yanking a correct crate because its documentation renderer failed spends a yank on nothing and leaves the rendered page exactly as it was anyway |
| **EC-010** | A defect in the published `0.2.0` is found after the fact | Under 0.x the minor bump **is** the breaking-change boundary, so the correction ships as a new forward-dated `0.(2+n).0` carrying a `CHANGELOG.md` entry naming what broke. Never an edit in place, never a re-publish of the same number, which the registry does not offer. A `yank` is reserved for a **broken artefact** — one that fails to build, or is missing a licence — not for correcting prose |
| **EC-011** | Any edit lands after the full gate run and before the first `cargo publish` — including a one-line changelog date fix | The transcript no longer describes the tree being published. **Re-run the whole gate** and record the new SHA. This is why AC-001 puts the changelog date edit *before* the gate run rather than after it: it costs one gate run and closes the gap that DoD 13's *"exact tree"* wording exists to catch |
| **EC-012** | `cargo publish --dry-run -p happenstance` cannot resolve `happenstance-core` at the release number | **Unavailable, not failing.** The dry run packages the crate and builds it against **registry-form** dependencies, so it cannot succeed until core is live. Record it as unavailable in `_release-log.md`. Mistaking unavailable for failing is how a correct release gets abandoned mid-sequence |

## Non-functional

| id | Requirement | Why it is a number and not a preference |
| --- | --- | --- |
| **NF-001** | **Exactly one irreversible transition per crate, and it runs last.** The ordered sequence is: version and changelog edit → preconditions re-observed → clause audit → full gate green → dry runs → rollback route written → `cargo publish -p happenstance-core` → `-p happenstance-testkit` / `-p happenstance` → tag → resolution check → docs.rs. No step that cannot be undone runs before the check that would have caught it | A crates.io publish cannot be edited or deleted (`RUNBOOK.md`:4481-4484), and `cargo yank` leaves every rendered page exactly as it was (`standards/rust/51-features-and-no-std.md`:226-231). The forward-only discipline a database migration would normally get is spent here instead |
| **NF-002** | **The release gate is the full gate, and its cost is accepted.** `cargo xtask ci` whole, once, on the published tree — including both feature powersets, `cargo deny` and the nightly `--cfg docsrs` build, none of which may report `skipped` | `run_fast`'s own doc comment says it unprompted: *"the project-scoped bar, not the release bar"* (`xtask/src/main.rs`:834). The project's DoD-6 bar stays `--fast` (`.redkiln/config.yaml`:55); this is an addition at the release boundary, not a replacement, and the two must be distinguishable in the record (AC-TEST-004) |
| **NF-003** | **The published tree and the gated tree are the same SHA.** `git rev-parse HEAD` is recorded next to the gate transcript, and any edit after that point invalidates the run | DoD 13 asks for the gate green *"on the exact tree that was published"* (`initiative.md`:396-397). A gate run against a tree that has since moved proves the wrong thing, and a changelog date is exactly the kind of one-line edit that quietly moves it — EC-011 |
| **NF-004** | **Post-publish verification happens within the hour, from a clean environment** that owns none of this repository's lock files: index resolution for all three crates, then a best-effort docs.rs check | The index is eventually consistent and docs.rs builds asynchronously. A check run too early reports a false failure; one deferred to closeout is a check nobody ran. Both errors are expensive here because the act they follow cannot be undone |
| **NF-005** | **No gate step reaches a registry, and the new step probes nothing.** The `wasm32` step added at AC-005 carries `probe: None`, so it can never print `skipped`; every registry-touching act in this story is manual and recorded | `cargo xtask ci` must stay runnable offline and hermetic — that is why the evidence is a committed transcript rather than a green step. And optional means *skipped when the probed tool is absent*, never *skipped when the check would fail* (`xtask/src/main.rs`:33-38); gating a step with no external tool behind a probe would invent the escape hatch CLAUDE.md's decorative-gate rule forbids |
| **NF-006** | **The manifest `description` is ≤ 120 characters, one sentence, true standalone.** Today's is ~131 (`crates/happenstance/Cargo.toml`:3) | It is the *only* text crates.io shows in a search result and on the crate card — the README never renders there — and it is published in the manifest, so it is unfixable for that version. `_design.md`:460-464 sets the budget and requires it be **re-read for truth at the publish commit** (IQ-7), because it is a published sentence like any other |
| **NF-007** | **`_release-log.md` is written as the release happens, not reconstructed afterwards**, and its section order is the evidence: preconditions → gate transcript → dry runs → **rollback route** → publish transcripts → tag → resolution check → docs.rs | Every acceptance step ending in *"recorded in `_release-log.md`"* has evidence that is unreproducible once the terminal scrolls: a publish transcript, an index resolution at a particular minute, a docs.rs queue state. And the file's ordering is the only proof that the recovery route was written **before** the irreversible act rather than back-filled after it |

## Implementation notes (non-prescriptive)

Suggestions, not requirements — the acceptance criteria are the contract.

- **Read `registry-surface-diff`'s report and write its finding into `_release-log.md` before you
  open a manifest.** The number follows the report. Doing this first costs nothing and makes EC-008
  a decision made calmly rather than one made with a publish command already typed.
- **Do the version move as one edit, verified immediately.** `Cargo.toml`:6 and the three requirement
  strings at `:24-26` are four lines in one file. A `cargo metadata --format-version 1` read straight
  afterwards is the only thing standing between the tree and EC-001. `rg -n '0\.2\.0-alpha' Cargo.toml
  crates/*/Cargo.toml` is a useful arithmetic check — but read the hits rather than counting them.
- **Let the lock files regenerate rather than hand-editing them.** Any `--locked` build after the
  manifest edit will refuse a stale lock, which is the signal you want.
  `experiments/wire-format/Cargo.lock`:69 belongs to a **separate workspace** that pins
  `happenstance-core` by path, so it needs its own command run from that directory.
- **Sequence the changelog date edit before the gate run, not after.** If the merge and the cut fall
  on different days, editing the date afterwards moves the tree out from under NF-003 — one gate run
  is cheaper than an argument about whether the transcript still applies.
- **Write the five-row precondition table before anything else in `_release-log.md`.** One row per
  blocker, each with the artefact path, the commit it names and the ancestry check. If a row cannot
  be filled in honestly, that is EC-006 and the release stops there — which is far cheaper at the top
  of the file than three sections later.
- **Add the `wasm32` step by copying the contract-crate entry, not by inventing a shape.** The step at
  `xtask/src/main.rs`:203-217 is the model: a name, `probe: None`, and a doc comment stating what it
  guards. Remember all three edits — `REQUIRED`, `wasm_steps()` (`:784-790`) and the module doc
  (`:8-24`) — because `steps_named` (`:816`) resolves by name and the second edit is the one that is
  easy to forget until `cargo xtask wasm` silently covers four things instead of five.
- **Hash the packaged READMEs rather than eyeballing them.** `cargo package --list` proves
  containment, never content; extracting each `.crate` and hashing the README against the file
  `rendered-page-preflight` read is a two-line check that makes AC-006's byte-identity claim real
  rather than asserted.
- **Treat `cargo publish --dry-run -p happenstance` as unavailable, not as failing** (EC-012), and
  record it that way. The dry run is a real pre-check for `happenstance-core` alone.
- **Do not automate the publish.** It is a human handoff, and the alpha's story map says so in terms.
  The value of the sequence is that a person reads the previous step's output before taking the next
  one; a script that runs all three removes exactly that.

## Tests and CI (merge gate)

Grounded in the testing brief's rows for AC-014 (*"integration — regression only, no new
instrument"*) and AC-016 (*"e2e, the whole gate"*), and its four wired grains
(`.redkiln/config.yaml`:28-60).

| Tier | Command / path | Proves |
| --- | --- | --- |
| Story grain | `cargo xtask affected --base main` (`.redkiln/config.yaml`:40) | fmt, `clippy -D warnings` and tests for the packages this diff touches plus dependents. The `xtask` change is inside this grain; the manifest version move touches every package |
| Reachability, static | `cargo xtask lints && cargo xtask spec-trace` (`:48`) | AC-001 — `changelog_names_every_rule` (CF-29) survives the `## [Unreleased]` → `## [0.2.0]` rename, so no entry was orphaned or merged; and `spec/SPECIFICATION.md`'s citations still resolve on the tree being published |
| `wasm32` grain | `cargo xtask wasm` (`xtask/src/main.rs`:784-790) | AC-005 — the new step is registered by name in `wasm_steps()` as well as in `REQUIRED`; `steps_named` (`:816`) makes a mis-registration a hard failure rather than a silent omission |
| Manifest / lock gate | any `--locked` step in `cargo xtask ci` (`xtask/src/main.rs`:52-55) | AC-001 — a lock file disagreeing with the manifests fails the **gate** rather than the registry. This is the guard, and it only works because the version edit precedes the gate run |
| Packaging gate | `cargo xtask ci` step *"packaged artifacts carry their licences and README"* (`xtask/src/main.rs`:515-532; `xtask/src/package.rs`:88-94) | AC-006 — each publishable tarball actually contains both licences and a README; and `reconcile` (`xtask/src/package.rs`:86) fails if the derived publishable set disagrees with the decided three (AC-002, EC-005) |
| Clause audit | `cargo xtask clause-audit --write --date <YYYY-MM-DD>` → `spec/audits/clause-maturity-<date>.md` | AC-003 — every clause's maturity at **this** commit, totals reconciled against §1.3's hand-computed census, and every `[FROZEN]` fingerprint compared against the tree the project received (EC-007) |
| Surface diff | the mandatory step `registry-surface-diff` added, against the `0.2.0-alpha.1` registry baseline | AC-001 — the report whose finding the published number is derived from. Run here as an input; built and proven-falsifiable there |
| **Release gate** | `cargo xtask ci` **whole** (`run_ci`, `xtask/src/main.rs`:828-834) on the literal publish commit | AC-004 — both feature powersets, `cargo deny`, the nightly `--cfg docsrs` build and all **five** `wasm32` steps, once, on the published SHA. `--fast` is explicitly not this bar (`:834-853`), and the transcript must show no `OPTIONAL` step reporting `skipped` |
| Pre-publish dry run | `cargo publish --dry-run -p happenstance-core`, `-p happenstance-testkit` | AC-007 — the packaged crates build from their own tarballs with registry-form dependencies. Not available for `happenstance` until core is live (EC-012) |
| Manual, registry | a scratch project **outside** this workspace; transcripts in `.bklg/.../publish-0-2-0/_release-log.md` | AC-008 — resolution of all three at the published number against the index, the tag and release, and the docs.rs observation. **No gate step can reach a live registry** (NF-005) |
| Human review | the story's review gate, against [`_design.md`](../_design.md):104-169, `:439-499`, `:641-680` | AC-006 — the composition invariants. Every machine check in this repository is satisfied by a `.crate` carrying the right files with the wrong page inside them; this is the instrument that is not |

**Merge gate for this story:** the story grain green, the reachability grain green, and
`cargo xtask ci` **whole** green on the merge SHA with its transcript committed. The project's own
non-terminal bar (`cargo xtask ci --fast`, `.redkiln/config.yaml`:55) is subsumed by the full run and
is not run separately — but the two records must remain distinguishable in `_release-log.md`, which
is AC-TEST-004's entire point.

## Risks and coupling (PR-scoped)

| Risk | Blast radius | Mitigation in this PR |
| --- | --- | --- |
| **The requirement strings are forgotten** (`Cargo.toml`:24-26) | The highest-cost mistake available here: `happenstance-core` is live and unremovable before the failure surfaces on the *second* `cargo publish` | AC-001 makes the four-line edit and its `cargo metadata` verification a **criterion**, not a step; EC-001 states the recovery so nobody improvises a version bump under pressure. The alpha already paid to learn this — it is reused, not re-derived |
| **`--fast` is mistaken for the gate** | The release ships without `cargo deny`, without the nightly `docsrs` build that is the only thing compiling the `doc(cfg)` attributes docs.rs will set, and without the only step that compiles `happenstance-core`'s default features on `wasm32` | `.redkiln/config.yaml`:55 fires `--fast` automatically at this project's integration stage, so the mistake is one keystroke away and looks identical in the log. AC-004 requires a **distinct, dated** artefact and forbids `skipped` on any `OPTIONAL` step; NF-002 quotes `run_fast`'s own doc comment saying it is *not the release bar* |
| **The new `wasm32` step fails and gets a feature edit** | The published crate would build on `wasm32` because `default` was narrowed to make it, contradicting the Guarantees claim P3 reads to self-identify — and the tree would have moved after the gate run | EC-004 makes it a **block**, a decision atom and a re-plan. `crates/happenstance/Cargo.toml` is deliberately **outside** the PR boundary, so `redkiln verify --grain story` fails on the edit rather than trusting discipline |
| **A copy edit sneaks into the release commit** | `rendered-page-preflight`'s dated read stops describing the pages that shipped, and AC-007 (project) is asserted rather than observed | The PR boundary's fenced list excludes every `crates/**` path; AC-006 asserts packaged-README **byte-identity** against the preflighted files. Both must fail for the mistake to land |
| **The gated tree drifts from the published tree** | DoD 13 becomes an assertion rather than an observation | NF-003 pins `git rev-parse HEAD` beside the transcript; AC-001 sequences the changelog date edit *before* the gate run; EC-011 states the required re-run rather than leaving it to judgement |
| **A precondition is remembered rather than re-observed** | Any of five upstream findings could describe a tree that no longer exists, and the release would be built on a dated file that was true elsewhere | AC-002 makes the five-row table an artefact written **before** the gate run, and EC-006 makes a stale row a stop. Fail closed with the name — never an empty check that passes |
| **The release is restarted at a new number after a partial failure** | A version number spent on a transient registry error, and a permanently confusing history for anyone reading the index | EC-002 states the resumption rule in the spec so it is not improvised at the keyboard, and AC-007 makes resumability a criterion rather than a hope |
| **Coupling: all five blocking stories must be *merged*, not merely written** | The published tree **is** the merged tree. Publishing early ships an artefact whose crate-set decision, surface diff, clause audit, deferred-clause reasons and rendered-page read do not exist in it | *Dependencies* below. `_storymap.md`:179-184 makes `rendered-page-preflight` a strict predecessor for the reason IQ-4 gives: reversibility is bought before the act, not after |
| **Coupling forward: `stranger-install-smoke` inherits whatever this publishes** | It is this project's proof artefact and it can only run after. If the index is wrong, it fails on something this story could have caught | AC-008 is the narrower check that runs first — *does the index serve all three at this number* — deliberately scoped so the two are not the same test run twice |
| **`RUNBOOK.md` is stale in four places this story passes through** | An implementer following the runbook would strip `provisional` from an immutable accepted atom (`:4475`), plan a four-crate release (`:4450-4451`), or chase a dead anchor (`:4495`) | The *PR boundary* names all three exclusions with the story that owns each; the staleness is **reported** in `_release-log.md` rather than acted on. `redkiln validate --kb` would fail the atom edit, but only after the diff exists |

## Dependencies

**Blocks on** — all five, and all five must be **merged**, not merely specified, because the tree
this story publishes is the tree they left behind (`_storymap.md`:167-175).

| Story slug | Why this story cannot cut before it |
| --- | --- |
| [`crate-set-decision`](../crate-set-decision/spec.md) | It decides *what* is published and lands the atom naming the four-crate alternative as rejected. `package-check`'s `reconcile` is the assertion that the tree agrees; without the decision there is no intention list for it to reconcile against (AC-002) |
| [`registry-surface-diff`](../registry-surface-diff/spec.md) | Its committed report is where the **version number comes from** (AC-001). Publishing before it exists means choosing a number, which project DR-2 forbids in terms |
| [`clause-maturity-audit`](../clause-maturity-audit/spec.md) | It builds the instrument this story *runs*, and its own spec says so: *"`publish-0-2-0` writes the next one at the publish commit; that one, not this one, is the release's artefact of record"* (AC-003). It also carries the `[FROZEN]` fingerprint comparison that makes EC-007 possible |
| [`deferred-clause-reread`](../deferred-clause-reread/spec.md) | It extends the same audit so an inherited, undated deferral reason fails the gate. Cutting between the two would publish a specification whose ten deferrals are still inherited while the check that catches it is half-landed |
| [`rendered-page-preflight`](../rendered-page-preflight/spec.md) | **Strictly** before, and it is the one ordering constraint `_storymap.md`:179-184 says a later re-plan must not reorder: IQ-4 puts the whole of this project's reversibility before the act, and `xtask/src/package.rs`:4-18 is explicit that containment is not presentation. It is also the read AC-006's byte-identity hashes are compared against |

**Unlocks**

- [`stranger-install-smoke`](../stranger-install-smoke/spec.md) — the slice-mate that can only run
  after, and this project's **proof artefact** (project AC-008, initiative DoD 9). Nothing it needs
  exists until the index serves all three crates at the published number.
- **The project's remaining DoD.** DoD items 1 and 2 are read against the tree this story publishes;
  DoD 10, 11 and 12 (`initiative.md`) are read against its rendered result.
- **`replication-identity-and-ingest`**, and transitively `retention-and-incomplete-logs` and
  `closeout-and-durable-audience`, which the project card records as this project's downstream
  (`project.md`, *Dependencies*).

## Anchors (progressive disclosure)

Load-bearing depth, deferred rather than pasted. Every path verified to exist in this worktree.

| Anchor | Why it is load-bearing | When to open it | Serves |
| --- | --- | --- | --- |
| `.bklg/from-contract-to-published-library/publication-and-positioning/_design.md` | The **signed-off, binding** composition. `## Surfaces` (`:104-169`) — seven pages, their source-of-record files, and the statement that the `route` fields do not resolve until this story lands. The transience policy (`:406-438`), the density budget with its real numbers including the ≤ 120-character `description` (`:439-499`), `## What it costs a caller` (`:581-598`) which is where AC-014's *asserted, not assumed* wording comes from, and the fifteen anti-patterns (`:641-680`) | **Before the packaged-artifact inspection**, and again before anyone proposes any edit to a `crates/**` file inside this PR | AC-005, AC-006 |
| `.bklg/from-contract-to-published-library/publication-and-positioning/_decomposition.md` | The **deployment brief** (`### Migration, backfill and rollback posture`, `### CI implication`, `### Release path if a published version changes`; AC-DEP-003/004/005) and the **testing brief** (the AC-014 and AC-016 rows of the test-mix table; AC-TEST-004's committed full-gate artefact; *Fixtures and seams to mock*, which is why no gate step reaches a registry) | Before the gate run (AC-DEP-003), before adding the `wasm32` step (AC-DEP-005), and before writing the rollback section of `_release-log.md` | AC-004, AC-005, AC-007 |
| `xtask/src/main.rs` | The gate's composition root and the second mount. `REQUIRED` (`:105`ff); the module doc's *what the gate proves* paragraph (`:8-24`) and the Mandatory/Optional distinction (`:33-38`); the contract-crate `wasm32` step to copy (`:203-217`) and the other three (`:231-282`); the `--locked` convention (`:52-55`); `package-check` (`:515-532`); the powerset that also omits `happenstance` (`:558-593`); the `docsrs` build and its *"a cfg nobody sets except docs.rs"* comment (`:600-612`); `wasm_steps()` (`:784-790`) and `steps_named` (`:816`); `run_ci` (`:828-834`) and `run_fast`'s *not the release bar* doc comment (`:834-853`) | Before writing the new step, and again before anyone argues a `--fast` run could substitute | AC-004, AC-005 |
| `xtask/src/package.rs` | Why the publishable set is **derived** from `cargo metadata` and reconciled against a hand list (`:1-42`), `PUBLISHABLE` (`:86`), `REQUIRED_FILES` (`:88-94`), and the `--allow-dirty` note on the gate running against uncommitted trees (`:69-74`). Also `:4-18`, the passage that says containment is not presentation — the reason AC-006 hashes READMEs instead of trusting `--list` | If `package-check` fails (EC-005), before assuming it is a false positive; and before designing the tarball inspection | AC-002, AC-006 |
| `Cargo.toml` | The mount point. `[workspace.package] version` (`:6`), the three internal requirement strings (`:24-26`) whose omission is EC-001, and the workspace lint table making `clippy::todo` deny (`:124-128`) | The first edit of the story, and again when checking for stub exemptions | AC-001, AC-002 |
| `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/publish-0-2-0-alpha-1/spec.md` | **Prior art, reused rather than re-derived**: the alpha's own AC-001 (the five-place version edit), its error catalogue EC-001…EC-008, its ordering and its `_release-log.md` shape. Everything this story does that is not new was proven there, at a cost this story does not have to pay again | Before writing `_release-log.md`, and before improvising any recovery under pressure | AC-001, AC-007, AC-008 |
| `.bklg/from-contract-to-published-library/publication-and-positioning/clause-maturity-audit/spec.md` | The instrument this story runs, and its `--write` / `--rebaseline` contract — including its own statement that *this* story writes the release's artefact of record at the publish commit | Immediately before running `clause-audit --write --date` | AC-003 |
| `.bklg/from-contract-to-published-library/publication-and-positioning/registry-surface-diff/spec.md` | Where the version number comes from, and the fail-closed-with-a-reason behaviour when the baseline is unreachable. Read the report it produced, not this spec, for the number — but read this spec to know what the report's shape means | Before the manifest edit, alongside the committed report | AC-001 |
| `.bklg/from-contract-to-published-library/publication-and-positioning/rendered-page-preflight/spec.md` | The dated read AC-006's byte-identity is compared against, and the strict predecessor `_storymap.md`:179-184 forbids reordering | When assembling the precondition table, and before hashing the packaged READMEs | AC-002, AC-006 |
| `.bklg/from-contract-to-published-library/publication-and-positioning/crate-set-decision/spec.md` | The decision `package-check` reconciles against, and the record of the four-crate alternative that lost | When the precondition table's first row is written | AC-002 |
| `.bklg/from-contract-to-published-library/publication-and-positioning/deferred-clause-reread/spec.md` | The ten dated deferral reasons and the audit extension that fails on an inherited one — checked inside the same audit step, so a green audit is evidence for both | With the clause audit | AC-002, AC-003 |
| `.bklg/from-contract-to-published-library/publication-and-positioning/stranger-install-smoke/spec.md` | The slice-mate that runs after. Read it to keep AC-008 **narrower** — the index, not the stranger — so the two are not the same test run twice | When writing the resolution check | AC-008 |
| `RUNBOOK.md` | Phase 12's own list: the stub check (`:4472-4473`), publish order (`:4469-4471`), the tag (`:4474`), and *a crates.io release cannot be edited or deleted* (`:4481-4484`). Also the three **stale** lines this story must report rather than obey — `:4450-4451` (four crates), `:4475` (strip `provisional` from an immutable atom), `:4495` (a dead anchor) | Before the publish sequence, and whenever the runbook seems to instruct something the briefs exclude | AC-002, AC-007 |
| `.redkiln/config.yaml` | `:40`, `:48`, `:55`, `:60` — the four wired verify grains, including `integration_scoped: cargo xtask ci --fast` at `:55`, the bar this story deliberately supersedes at the release boundary, and `e2e` at `:60`, which is the terminal project's and not this one's | When justifying why the full gate runs here and nowhere else in the project | AC-004 |
| `.bklg/from-contract-to-published-library/initiative.md` | DoD 9, 10, 11, 12 and 13 — what this release is measured against at initiative level, and precisely how far this story gets (`:396-397` is AC-004's source) | When writing the resolution check, so it takes DoD 9's shape rather than an ad-hoc smoke test | AC-004, AC-008 |
| `.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md` | P4 the evaluator — one-shot, time-boxed, unable to run the suite (`:249-313`, `:262-266`, `:333-338`) — and the constrained-runtime reader. The reason every criterion above is written from a goal rather than a capability | When judging whether a recorded observation would mean anything to someone who did not build this | AC-006, AC-008 |
| `standards/rust/51-features-and-no-std.md` | `:226-231` — what `yank` actually does: removes a version from **future resolution** and leaves every rendered page exactly as it was. The single fact the whole rollback posture is derived from | Before writing the rollback route, and before anyone proposes a yank as a fix for wording | AC-007 |
| `.kb/decisions/0001-async-port-flavours.md` | Why the `!Send` flavour exists and why `#[async_trait]` is forbidden — the constraint the `wasm32` steps are the standing guard on, and the reason EC-004 is a block rather than a feature edit | Before adding the `wasm32` step, and immediately if it fails | AC-005 |
| `.kb/decisions/0010-the-suite-must-prove-itself.md` | *A skip is reported, never silent* — the discipline the fail-closed precondition check borrows, and the one the deployment brief already applies by name to this project's new gate steps | When designing what happens if a precondition artefact is missing | AC-002 |
| `crates/happenstance/Cargo.toml` | `:3` — the `description` published in the manifest, ~131 characters today against a ≤ 120 budget; `:22-26` — `default = ["std", "memory"]` forwarded to `happenstance-core`, which is the exact feature set AC-005's new step compiles. **Read-only in this PR**: it is outside the boundary | When counting the description, and when writing the `wasm32` step's doc comment | AC-005, AC-006 |
| `CHANGELOG.md` | `:1-25` — the `## [Unreleased]` heading to re-head, and the link references that move with it. The file has accumulated since phase 0 rather than being reconstructed from `git log`, and that property is preserved | Before the changelog edit, and before the gate run that follows it | AC-001 |

## Clarifications resolved during spec

1. **The eight AC ids the front half decided are kept exactly; this half states what each one is.**
   AC-001 the release tree (version derived from the report, five places, plus the dated changelog
   section — all *before* the gate); AC-002 the five preconditions re-observed plus the tree's own
   two (crate-set reconcile, no stub exemption); AC-003 the clause audit at this commit; AC-004 the
   whole gate on the literal publish commit; AC-005 the fifth mandatory `wasm32` step; **AC-006 the
   packaged artifact's composition** — byte-identity with the preflighted pages, the docs.rs block,
   the `description` budget, the stale strings; AC-007 the irreversible act with reversibility bought
   first; AC-008 the check from outside. Nothing was added and nothing was dropped.
2. **The changelog is folded into AC-001 rather than given its own row.** It is not a separate
   deliverable but the second half of the same act — *make the tree the release tree, then gate it*.
   The alpha's NF-005 taught the specific reason: a date edited after the gate run silently moves the
   tree, so the changelog edit belongs on the same side of the gate as the version edit, and putting
   the two in one criterion is what makes that ordering gated rather than advisory.
3. **AC-006 exists because RFC §6.7/D6 requires a composition invariant to be a ledger row.** This
   story authors no surface, which made it tempting to write the composition family as prose. But
   `redkiln verify` extracts ACs only from a leading `| AC-001 |` cell or a `- AC-001:` bullet: a
   composition invariant living in the *Interaction quality* section would reach no ledger row and
   would never be tested. The invariants this story genuinely owns — *what shipped is what was read*,
   and the one composed value it is the last owner of — are therefore a criterion, and the
   *Interaction quality* section only names which id carries which.
4. **The `description` budget is claimed by this story deliberately, and it is the one place this
   spec touches copy.** `_design.md`:460-464 sets ≤ 120 characters and requires the sentence be
   re-read for truth at the publish commit. It is a **manifest** field, not README prose, it is
   published and unfixable, and it is the only text crates.io shows in a search result. If it needs
   *rewriting* rather than confirming, that is `landing-copy-and-status-truth`'s and it blocks this
   story — AC-006 asserts the property, it does not author the sentence.
5. **AC-008 is narrower than the stranger-install smoke on purpose, and the two are not redundant.**
   This story asks *does the index serve all three crates at this number* — the last thing it can
   still be wrong about. `stranger-install-smoke` asks *does a stranger succeed*, which is the
   project's proof artefact and needs a scratch project, the README's own quick start and a
   write-then-read cycle. Collapsing them would make the release's own check depend on copy this
   story does not own.
6. **`0.2.0-alpha.1` is not yanked, and that is recorded as a decision rather than left silent.** The
   alpha's stated policy — *"each alpha is yanked when the next lands"* — is a rule about an alpha
   superseding an alpha. Cargo does not select a pre-release for an ordinary requirement, so the
   alpha stops resolving the moment `0.2.0` exists; and it is the baseline `registry-surface-diff`
   diffs against. Yanking it buys nothing and risks the artefact this release's central check reads.
7. **Three `RUNBOOK.md` instructions are deliberately not followed, and the staleness is reported
   rather than repaired.** `:4475` would strip `provisional` from an accepted, immutable atom —
   `msrv-promise-atom` writes a **new** atom instead (project DR-6); `:4450-4451` still describes a
   four-crate release that `crate-set-decision` closed at three; `:4495` cites a dead anchor. A
   release commit is the wrong place to repair a runbook, so `_release-log.md` records all three for
   whoever owns the next pass.
