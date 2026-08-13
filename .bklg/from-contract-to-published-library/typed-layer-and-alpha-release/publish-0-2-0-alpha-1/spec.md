---
item: HS-S0033
stage: spec
created: 2026-08-12T13:46:31.358Z
updated: 2026-08-12T13:46:31.358Z
template_sig: 87bbf1d0
rendered_sig: ece50cdb
---

# Spec — 0.2.0-alpha.1 on the registry, with its churn mitigations

## Scope lock

| Layer | Path | What it fixes for this story |
| --- | --- | --- |
| Initiative | [`.bklg/from-contract-to-published-library/initiative.md`](../../initiative.md) | BR-05 (a published release must exist at `0.2.0-alpha.1`, `:288`); DoD 9, 11, 13 (`:354-402`) |
| Project | [`.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/project.md`](../project.md) | AC-011 (`:200-203`); DoD 7 (`:238-240`); risk row 1 — *the alpha becomes an argument against changing things* |
| This spec | `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/publish-0-2-0-alpha-1/spec.md` | the source of truth for this story; the item card carries no spec body |
| Deployment brief | [`_decomposition.md`](../_decomposition.md) `:877-1090` | DEP-001…DEP-007, and the two open items it flags rather than assumes |
| Architecture brief | [`_decomposition.md`](../_decomposition.md) `:452-487` | *Composition roots* items 7 (`crates/happenstance/README.md`) and 8 (`CHANGELOG.md`) |
| UX brief | [`_decomposition.md`](../_decomposition.md) `:227-232`, `:296-305` | AC-U18 (stability legible before the reader commits); DT-1 stays HS-P0016's |
| Signed-off design | [`_design.md`](../_design.md) `:782-791`, `:729-751`, `:969-1013`, `:1224-1256` | `crate-readme` composition, the visibility table this release ships, anti-patterns 7/8/15, and the three sign-off overrides |
| Story map row | [`_storymap.md`](../_storymap.md) `:64`, `:130-132`, `:163-166` | the one-line slice, M7's ordering, and *`cargo publish` is a human handoff* |
| Roadmap pointer | `RUNBOOK.md:4108-4126` and `RUNBOOK.md:4131-4162` | why the first number is `0.2.0` and not `0.1.0`; why the alpha lands here and its three mitigations |

## One-line PR slice

Cut `0.2.0-alpha.1` for `happenstance-core` then `happenstance` (`happenstance-testkit`
on its own CF-32 number), after one full `cargo xtask ci` — not `--fast`, which drops the
feature powerset and `cargo deny` (`xtask/src/main.rs:835`) — with the README `## Stability`
section landing above the first code block, `CHANGELOG.md`'s `## [Unreleased]` becoming the
alpha's section *and* its `unstable-projection` claim reconciled with the manifest, the yank
policy stated, and post-publish resolution verified with an explicit pre-release requirement.

## Executive summary

This PR turns a tree into an artefact a stranger can resolve. Everything M1–M6 built is
already in the working copy; nothing here adds a public Rust item — [`_design.md`](../_design.md)'s
`## Items` block (`:229-379`) assigns this story none, and that is the point. The delta is
four manifest edits, two documents, one full-gate run and one irrevocable act.

The delta, stated as a diff against the tree as it stands today:

- **`0.2.0` → `0.2.0-alpha.1`** in `Cargo.toml:6` (the workspace number all three publishable
  crates inherit), in the three `[workspace.dependencies]` **requirement strings** at
  `Cargo.toml:24-26`, and in `crates/happenstance-testkit/Cargo.toml:14` (its own number, which
  moves down from a stable `0.2.0` per the design's sign-off).
- **`crates/happenstance/README.md`** loses its facade blockquote (`:6-11`) and gains
  `## Stability` above the first code block.
- **`CHANGELOG.md`**'s single `## [Unreleased]` heading (`:24`) becomes the alpha's dated
  section, and its assertion that *"`ProjectionStore` ships behind an off-by-default
  `unstable-projection` feature"* (`:19-22`) is reconciled with what the manifests actually
  carry.
- **`cargo xtask ci` whole**, then `cargo publish` in dependency order, then a resolution check
  from outside this workspace.

What this PR is *not*: the phase-12 stable `0.2.0` (`RUNBOOK.md:163`), the MSRV promise, the
`cargo-semver-checks` verdict, registry presentation or DT-1 — all HS-P0016's
([`project.md`](../project.md), *Out of scope*). This release **creates the baseline those are
diffed against**; it makes none of the promises.

## Context pack

The load-bearing decisions, stated as decisions. Read this section before touching anything.

**1. The number is `0.2.0-alpha.1`, and the reason is on the record.** Phase 4 was a breaking
change to a version that had never moved — `cargo-semver-checks` reported seven major lints
against `happenstance-core`, every one of them a decision an ADR had already taken. Nothing is
published, so nothing downstream broke; the number still moved, because *"the finding was true,
and a true finding is not silenced."* In `0.x` the breaking position is the minor, so `0.1.0`
became `0.2.0` and the planned alpha became `0.2.0-alpha.1` (`RUNBOOK.md:4108-4124`). Do not
re-derive this; do not round it to `0.1.0` on the reasoning that nothing shipped yet.

**2. Three crates publish, six do not, and the order is not a preference.** `happenstance`,
`happenstance-core` and `happenstance-testkit` carry no `publish = false`; the six skeletons do
(`crates/happenstance-{cloudflare,ladybug,neon,postgres,sqlite,sync}/Cargo.toml:12`) and are
untouched by this release (DEP-001). `happenstance` depends on `happenstance-core`
(`crates/happenstance/Cargo.toml:20`) and `happenstance-testkit` does too
(`crates/happenstance-testkit/Cargo.toml:29`), so **`happenstance-core` publishes first**;
crates.io rejects a manifest whose dependency cannot resolve against the registry yet. The
testkit has no ordering relation to `happenstance` and may go either side of it once core is
live (DEP-004).

**3. A `version = "0.2.0"` requirement does not match a `0.2.0-alpha.1` release, and this is
where the irrevocability bites.** `Cargo.toml:24-26` declares all three workspace dependencies
with an explicit `version = "0.2.0"` alongside their `path`. Cargo's pre-release rule is that a
requirement without a pre-release component never matches a version with one — so publishing
`happenstance-core` at `0.2.0-alpha.1` and then running `cargo publish -p happenstance` fails on
an unresolvable dependency **after the first crate is already live and unremovable**. The
requirement strings move with the version, in the same edit, before any publish runs. This is
the single highest-cost mistake available in this story.

**4. `happenstance-testkit` publishes `0.2.0-alpha.1`, not a stable `0.2.0`.** The deployment
brief flagged this as an open decision and assigned it (`_decomposition.md:1056-1072`);
[`_design.md`](../_design.md) settled it at sign-off, item 3 (`:1252`), with the reasoning:
**CF-32 gives the testkit an independent *number*, not an independent *maturity*** — a stable
conformance suite over a port that is still moving is exactly the promise this project refuses
to make. Its in-tree `0.2.0` (`crates/happenstance-testkit/Cargo.toml:14`) moves *down* to
`0.2.0-alpha.1`. What must not change is that the key is written literally rather than as
`version.workspace = true`: CF-32 is `[FROZEN]` (`spec/SPECIFICATION.md:8200`) and
`cargo xtask ci` checks it (`xtask/src/main.rs:425-435`).

**5. `## Stability` goes above the first code block — two briefs say otherwise and the design
overrides them.** The architecture brief's *Composition roots* item 7 and DEP-003 both place it
between `## Guarantees` (`:41`) and `## Design` (`:51`). [`_design.md`](../_design.md) rejects
that placement in terms (`:128-135`, and again at sign-off `:1249-1251`): AC-U18 requires the
stability posture to be legible *"above the first code block a reader will copy"*, and the
brief's line was a mount-point convenience while ordering is a composition decision. **AC-U18
wins on ordering.** The resulting page, top to bottom: `# happenstance` → the DCB sentence
(untouched) → `## Which crate do I want?` → **`## Stability`** → `## What DCB buys you` →
`## Guarantees` → `## Design` → `## Licence` (`_design.md:782-791`).

**6. The facade blockquote is deleted, not annotated.** `crates/happenstance/README.md:6-11`
asserts *"this crate is currently a facade… adds nothing yet"*, which is **false the moment this
project ships**, and it additionally claims the crate is already published, which it is not
(the three names were verified *free* on 2026-08-06, `RUNBOOK.md:798-801`). Two stability claims
on one page, one of them wrong, is worse than either (`_design.md:136-140`). Anti-pattern 7
(`_design.md:975-977`) fails a README that shows two stability claims, shows the words *"currently
a facade"* anywhere, or puts `## Stability` below the first code block.

**7. `## Stability` carries three things and nothing else** (`_design.md:789-791`): the phase at
which the API stops moving, **in the reader's terms**; a pointer to `CHANGELOG.md`; and the yank
policy — *"only one pre-release resolves at a time; each alpha is yanked when the next lands."*
Lines 1-4 of the README are **not touched**: which claim leads on first contact is DT-1, which is
HS-P0016's, and `## Stability` is this project's only claim on that page
(`_decomposition.md:296-305`; anti-pattern 8, `_design.md:978`).

**8. The changelog already asserts a feature no manifest has, and this release must not publish
that discrepancy.** `CHANGELOG.md:19-22` states *"`ProjectionStore` ships behind an off-by-default
`unstable-projection` feature"*; `crates/happenstance-core/Cargo.toml:34-52` carries `default`,
`std`, `serde` and `memory` only. The deployment brief makes reconciling this **release-blocking**
(`_decomposition.md:1013-1023`) and anti-pattern 15 (`_design.md:991-993`) fails *"the changelog
claims a feature the manifest does not have."* Two admissible resolutions and no third: the
feature exists in the manifests at publish time (M5's `projection-trait-and-runner` gates the
runner on `unstable-projection` in `happenstance`, forwarded to core only if HS-P0010 lands it —
`_design.md:647`), in which case the changelog line is made accurate about *which* crate carries
it; or it does not, in which case the changelog line is corrected before the cut. Reconciling by
adding a feature to `crates/happenstance-core/Cargo.toml` from inside this story is not one of
them — see *PR boundary*.

**9. The gate for this story is `cargo xtask ci` whole, and the reason is in the gate's own
source.** `run_fast`'s doc comment says it unprompted: *"`REQUIRED` without `OPTIONAL` — the
project-scoped bar, **not the release bar**"* (`xtask/src/main.rs:835`). `--fast` drops the two
feature powersets, `cargo deny` and the nightly `--cfg docsrs` build. Two of those matter directly
here: the **feature powerset** is the only instrument that proves the codec features hold *in
combination* (what a `cargo add happenstance --no-default-features --features cbor` consumer
depends on the day this ships), and **`cargo deny check`** is a licence gate a published crate
cannot un-publish its way out of (`_decomposition.md:1025-1054`). The project's own DoD-6 bar stays
`--fast` (`.redkiln/config.yaml:50-55`); this story adds the full run at the release boundary
because nothing else in the wiring forces it there. `package-check` (D11,
`xtask/src/main.rs:519-529`) already asserts each publishable crate's tarball carries both licence
files and a README — it is required green before publish, not re-described here (DEP-005).

**10. The publish is a human handoff and it is irrevocable.** [`_storymap.md`](../_storymap.md)`:163-166`
names `cargo publish` as one of the project's two human handoffs, not an implementation step.
`cargo yank` hides a version from *new* resolution; it does not delete it, and a published version
cannot be edited (`RUNBOOK.md:4483-4484`, DEP-006). Rollback posture is therefore **yank the
mistake, publish the fix at a new number** — `cargo yank --version 0.2.0-alpha.1` followed by a
`0.2.0-alpha.2` and a changelog entry saying what broke between alphas (DEP-007). Never edit in
place, and never assume a re-publish of the same number is available.

**11. Nobody gets the alpha by accident, and no gate can check that they got it deliberately.**
Cargo will not resolve a pre-release without an explicit pre-release requirement in the consumer's
manifest — the version string *is* the gate, and it is Cargo's mechanism, not something this
project builds (DEP-002). The consequence is that verification is a **manual resolution check from
a clean environment outside this workspace**, against the registry, not a path dependency: no step
inside `cargo xtask ci` can reach a live registry (DEP-006). [`_storymap.md`](../_storymap.md)`:100-102`
says so explicitly — *"the story owns it as an explicit acceptance step, not as an automated
assertion."*

**12. The persona slice this realizes: P4's one bounded sitting.** The journey is *"Choose a
contract before a database"* and this activity is A7 — *get it from the registry, and know what it
promises* ([`_storymap.md`](../_storymap.md)`:29`). P4's beat *"is one-shot and time-boxed — they
do not get a second pass"* (`_design.md:102-106`). `## Stability` is the one heading they are
scanning for (`_design.md:886-889`); the failure mode is not *reject*, it is **cannot tell**
(`_decomposition.md:231-232`).

**13. The failure mode these mitigations guard against is the author, not the users.** *"An alpha
in the wild makes 'we can't change that now' available as an argument… treating an unpublished API
as something to protect biases every decision toward the option that changes least, which is not
the same as the option that is best"* (`RUNBOOK.md:4157-4162`). This is why the three mitigations
are acceptance criteria rather than intentions ([`project.md`](../project.md), risk row 1). A
`## Stability` section that hedges instead of naming a phase has satisfied the letter and failed
the reason.

## Integration contract

- **Archetype**: `capability` — the user-observable slice is *a stranger resolves the crate from
  the registry and can tell what it promises*. Nothing here is substrate for a later story in this
  project; this is the project's last story.
- **Slice / milestone**: `alpha-release` (M7). Slice-mates, implemented in the same context and
  mounted as one surface: [`edge-flavour-and-wasm-claim`](../edge-flavour-and-wasm-claim/spec.md)
  and [`defect-log-and-macros-verdict`](../defect-log-and-macros-verdict/spec.md). Both are
  independent of each other; **this story runs last** — *"the publish is the one irrevocable act in
  the project"* ([`_storymap.md`](../_storymap.md)`:130-132`).
- **Mount point**: **`Cargo.toml`** — `[workspace.package] version` (`:6`). It is the single root
  that decides what `happenstance` and `happenstance-core` publish as, inherited at
  `crates/happenstance/Cargo.toml:4` and `crates/happenstance-core/Cargo.toml:4`; the registry
  artefact does not exist until that line moves. Co-mounts, each named as a composition root by the
  architecture brief (`_decomposition.md:476-480`) or forced by the mount point itself:
  `Cargo.toml:24-26` (the `[workspace.dependencies]` requirement strings — see context pack 3),
  `crates/happenstance-testkit/Cargo.toml:14` (CF-32's independent number),
  `crates/happenstance/README.md` (item 7) and `CHANGELOG.md` (item 8).
- **Wires into**: `xtask/src/main.rs`'s `REQUIRED` + `OPTIONAL` (`:105`, `:535`) via
  `cargo xtask ci`, and specifically its `lint-testkit-version` step (`:425-435`, CF-32) and
  `package-check` step (`:519-529`, D11, implemented at `xtask/src/package.rs`);
  `xtask/src/lints.rs`'s `changelog_names_every_rule` (`:525`, CF-29), whose parser starts a new
  entry at any heading (`:456-479`) and is therefore indifferent to the section being renamed —
  which is the thing to verify rather than assume; `Cargo.lock` and
  `experiments/wire-format/Cargo.lock:70`, both of which pin the workspace version by path.
- **Renders surfaces**: **`crate-readme`** (`_design.md:54-58`) — `route:
  crates/happenstance/README.md`, `selector: "## Stability (new, replaces the blockquote at lines
  6-11)"`, states `rendered-on-crates-io`, `compiled-as-doctest`, `narrow-1024`. The
  `compiled-as-doctest` state is not decorative: the README is included as a doctest at
  `crates/happenstance/src/lib.rs:10`, so an edit to it is compiled by the gate. No other surface
  is rendered or changed here — `crate-root-rustdoc`'s composition is M2's and this story must not
  touch it.
- **Public items**: **none.** [`_design.md`](../_design.md)'s `## Items` block (`:229-379`) assigns
  this story no `path`. What this story owes the design is the opposite obligation: the
  `## Visibility and stability` table (`:729-751`) describes the surface that is *about to become
  a published artefact* — and this release ships it as-is rather than adding to it. An item that
  became `pub` during this story is a semver promise made by a release story, which is the one
  place it can never be reviewed.
- **Conformance rule(s)**: **none, and it is not adapter-observable.** Nothing in
  `crates/happenstance-testkit/src/suite.rs` can observe a version string, a README section or a
  registry state, and adding a rule that could would be the decorative rule CLAUDE.md forbids. The
  machine checks that *do* observe this story are gate steps, not conformance rules:
  `lint-testkit-version` (CF-32) and `package-check` (D11), both named above.
- **Clause(s)**: discharges none and amends none. It is **held to** three, all `[FROZEN]`:
  **CF-32** (`spec/SPECIFICATION.md:8200`, the testkit's own `version` key), **CF-29** (`:8141`, a
  conformance rule lands with a changelog entry naming a defect — the constraint the changelog edit
  must not break), and **CF-34** (`:8263`, performance is measured by a harness outside the gate —
  the reason `experiments/` is untouched here). Changing any of them takes a new ADR, not an edit,
  and this story has no licence to write one.
- **Advances DoD scenario**: initiative **DoD 9** — *"@smoke — a stranger can install it"*
  (`initiative.md:383-386`). This story does not turn it green: DoD 9 says *the published crate*
  and its full form is HS-P0016's stable `0.2.0`. It moves it from unreachable to observable —
  after this story there is a version on the registry that a scratch project outside this workspace
  can add and resolve. It also creates the prior published baseline **DoD 11** (`:390-392`) diffs
  against, which does not exist until this publish happens, and it observes **DoD 13**'s bar
  (`:396-398`, `cargo xtask ci` green on the exact tree that was published) at the release
  boundary rather than at closeout.

## PR boundary

`redkiln verify --grain story` reads the first fenced block below and fails on any file changed
outside it.

```
Cargo.toml
Cargo.lock
crates/happenstance-testkit/Cargo.toml
crates/happenstance/README.md
CHANGELOG.md
experiments/wire-format/Cargo.lock
.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/publish-0-2-0-alpha-1/**
```

**In this PR**

- The version move to `0.2.0-alpha.1`: `Cargo.toml:6`, the three requirement strings at
  `Cargo.toml:24-26`, `crates/happenstance-testkit/Cargo.toml:14`, and the two lock files that
  record the result (`Cargo.lock`, `experiments/wire-format/Cargo.lock:70`).
- The README edit: blockquote `:6-11` deleted, `## Stability` inserted above the first fence with
  its three contents, lines 1-4 untouched.
- The `CHANGELOG.md` edit: `## [Unreleased]` (`:24`) becomes the alpha's dated section, the link
  reference at the file's end (`:1166`) is updated to match, and the `unstable-projection` claim
  (`:19-22`) is made true of the manifests as they stand.
- One full `cargo xtask ci` run, immediately before the cut, on the tree that is published.
- The `cargo publish` handoff and the post-publish resolution check, recorded as evidence in this
  story's own folder.

**Explicitly not in this PR**

- **Adding, removing or gating any public Rust item.** No `crates/*/src/**` path is in the
  boundary. If the `unstable-projection` reconciliation turns out to need a manifest feature rather
  than a changelog correction, that edit is
  [`projection-trait-and-runner`](../projection-trait-and-runner/spec.md)'s (`_design.md:647`) and
  this story **blocks on it** rather than widening — a release story that grows a feature flag has
  changed the artefact it was cutting.
- **The README's opening two lines** (DT-1, HS-P0016's) and the `## What DCB buys you` code block,
  whose replacement by the first program is M2's per `_design.md:785-787`.
- `crate-root-rustdoc`'s composition (M2), the `wasm32` claim (`edge-flavour-and-wasm-claim`), the
  defect log and the macros verdict (`defect-log-and-macros-verdict`).
- The stable `0.2.0`, the MSRV promise, the `cargo-semver-checks` verdict, the clause-ledger audit
  and registry presentation — all HS-P0016's ([`project.md`](../project.md), *Out of scope*).
- `standards/rust/50-dependency-hygiene.md:64`'s illustrative `version = "0.2.0"` TOML block. It is
  an example, not a citation `cargo xtask lint-constitution` resolves, and rewriting the
  constitution from a release story is how a standards atom acquires an edit nobody reviewed.
- Any `.github/workflows/` change. No workflow references `publish` or `crates.io` today and this
  story does not add one (`_decomposition.md:1074-1083`).

**Merge DoD (one line):** `cargo xtask ci` green whole on the merged tree, `0.2.0-alpha.1`
resolvable from a clean environment outside this workspace under an explicit pre-release
requirement, and the README/changelog mitigations present as written.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| The workspace number becomes the alpha | `Cargo.toml:6` `version = "0.2.0"` → `"0.2.0-alpha.1"`; inherited unchanged by `crates/happenstance/Cargo.toml:4` and `crates/happenstance-core/Cargo.toml:4` (`version.workspace = true`) | `Cargo.toml:5-6`; `RUNBOOK.md:4108-4124` |
| The workspace **requirements** move with it | `Cargo.toml:24-26` — all three `version = "0.2.0"` requirement strings become `"0.2.0-alpha.1"`. A requirement with no pre-release component never matches a version with one, so leaving these is a publish failure on `happenstance` *after* `happenstance-core` is live and unremovable | `Cargo.toml:24-26`; DEP-004 (`_decomposition.md:945-952`) |
| The testkit moves on its own key, downward | `crates/happenstance-testkit/Cargo.toml:14` `"0.2.0"` → `"0.2.0-alpha.1"`, still written literally and never `version.workspace = true` | `_design.md:649`, `:1252`; CF-32 `spec/SPECIFICATION.md:8200`; `xtask/src/main.rs:425-435` |
| The six skeletons are untouched | `publish = false` stays at `crates/happenstance-{cloudflare,ladybug,neon,postgres,sqlite,sync}/Cargo.toml:12`; `package-check` derives the publishable set from `cargo metadata` and fails when the derived set disagrees with its hand list, so a stray promotion is caught rather than shipped | DEP-001 (`_decomposition.md:909-923`); `xtask/src/package.rs:24-43` |
| Both lock files record the new number | `Cargo.lock`, and `experiments/wire-format/Cargo.lock:70`, which pins `happenstance-core` by path from a separate workspace and will otherwise disagree with the tree | `experiments/wire-format/Cargo.lock:68-70` |
| The README states the stability posture above the first fence | Blockquote `:6-11` deleted; `## Stability` inserted between `## Which crate do I want?` (`:13`) and `## What DCB buys you` (`:22`), carrying exactly three things: the phase at which the API stops moving in the reader's terms, a pointer to `CHANGELOG.md`, and the yank policy | `_design.md:782-791`, `:125-140`, `:1249-1251`; AC-U18 (`_decomposition.md:227-232`) |
| The README edit stays compiled | The file is included as a doctest at `crates/happenstance/src/lib.rs:10` (AC-U19), so the gate compiles what the edit leaves behind; `## Stability` adds prose and no fence | `crates/happenstance/src/lib.rs:10`; `_decomposition.md:236-244` |
| The README makes no claim that is DT-1's | Lines 1-4 unchanged; the word *facade* appears nowhere; no second stability claim anywhere on the page | anti-patterns 7 and 8, `_design.md:975-978`; `_decomposition.md:296-305` |
| The changelog gets a released section | `## [Unreleased]` (`:24`) becomes `## [0.2.0-alpha.1] — <ISO date>`; the link reference at `:1166` is updated and a new `[Unreleased]` may sit above it with no entries. `entries()` starts a new entry at any heading, so the rename cannot merge or orphan an entry — verify CF-29 stays green, do not assume it | `CHANGELOG.md:24`, `:1166`; `xtask/src/lints.rs:456-479`, `:525`; CF-29 `spec/SPECIFICATION.md:8141` |
| The changelog's feature claim is true of the manifests | `CHANGELOG.md:19-22` asserts an off-by-default `unstable-projection` feature; `crates/happenstance-core/Cargo.toml:34-52` has `default`, `std`, `serde`, `memory` only. Either the line is corrected, or it is made accurate about which crate carries the feature M5 landed. Release-blocking either way | `_decomposition.md:1013-1023`; anti-pattern 15, `_design.md:991-993`; `_design.md:647` |
| The release gate is the whole gate | `cargo xtask ci` — including both feature powersets, `cargo deny` and the nightly `--cfg docsrs` build — run once, immediately before the cut, on the exact tree that publishes. `--fast` is the project bar and says of itself that it is *not the release bar* | `xtask/src/main.rs:828-848`; `_decomposition.md:1025-1054`; DoD 13 (`initiative.md:400-402`) |
| Publish order follows the dependency graph | `cargo publish -p happenstance-core`, then `-p happenstance`; `-p happenstance-testkit` any time after core. A human handoff, not an automated step | DEP-004; [`_storymap.md`](../_storymap.md)`:163-166` |
| Post-publish verification is a resolution check | From a scratch project **outside** this workspace: add each of the three at an explicit pre-release requirement and resolve against the registry, not a path dependency; confirm the crates.io page renders the README's `## Stability`; check docs.rs best-effort within the hour (a docs.rs failure is worth knowing, not a blocker) | DEP-006 (`_decomposition.md:964-977`); [`_storymap.md`](../_storymap.md)`:100-102`; DoD 9 (`initiative.md:386-389`) |
| The yank policy is stated where a reader meets it, and its trigger is recorded | The README's `## Stability` states *only one pre-release resolves at a time; each alpha is yanked when the next lands*; the rollback posture — `cargo yank --version 0.2.0-alpha.1` then a `0.2.0-alpha.2` with a changelog entry saying what broke, never an edit in place — is recorded in this story's own folder as the standing instruction for whoever cuts the next alpha | `_design.md:789-791`; DEP-003, DEP-007 (`_decomposition.md:931-944`, `:978-990`); `RUNBOOK.md:4148-4155` |

## Data and migrations

**N/A for persisted data, and the reason is structural rather than incidental.** This release
touches nothing durable: `MemoryEventStore` is in-process and non-durable by construction, and
every adapter that could hold data — `happenstance-sqlite`, `-cloudflare`, `-postgres`, `-neon`,
`-ladybug` — carries `publish = false` and is outside this story's boundary. There is nothing to
migrate, nothing to backfill, and no future release of this project has a migration to reconcile
against this one (`_decomposition.md:994-1001`).

**There is exactly one irreversible state transition, and it is not in a database.** It is the
registry. A crates.io publish cannot be undone or edited (`RUNBOOK.md:4483-4484`); `cargo yank`
removes a version from *new* resolution and leaves existing lockfiles resolving it. So the
forward-only discipline a migration would normally get is spent here instead, in this order and no
other:

1. Every manifest and requirement string carries `0.2.0-alpha.1` **before** any publish runs
   (context pack 3 — the ordering failure is silent until the second `cargo publish`, by which
   point the first is permanent).
2. `cargo xtask ci` whole, green, on that tree.
3. `cargo publish -p happenstance-core`, then `-p happenstance`; `-p happenstance-testkit` at any
   point after core.
4. Resolution verified from outside the workspace under an explicit pre-release requirement.

A defect found after step 3 is not repaired at `0.2.0-alpha.1`. It is yanked and republished at
`0.2.0-alpha.2` with a changelog entry naming what broke between the alphas — which is the
artefact DEP-003 already requires to exist, for precisely this reason (DEP-007,
`_decomposition.md:978-990`).

## Acceptance criteria

Seven criteria, each written from the goal of a persona this initiative named rather than from the
capability that satisfies it. **P4** is *the evaluator (pre-adoption)* — *"time-boxed and one-shot
in a way the others are not… whatever they spend reading a registry page and a README once"*
(`_discovery/distillation/personas-and-journeys.md:249-260`, `:333-338`). **P1** is the application
author who will `cargo add` this. **P2** is the adapter author who pins the conformance suite
exactly. The activity is A7 — *get it from the registry, and know what it promises*
([`_storymap.md`](../_storymap.md)`:29`).

Evidence that no gate step can reach — a registry resolution, a rendered crates.io page, a
`cargo publish` transcript — is recorded in this story's own folder as `_release-log.md`, which the
PR boundary already admits. That is not a weakening of the bar; it is DEP-006's stated shape
(`_decomposition.md:964-977`) and [`_storymap.md`](../_storymap.md)`:100-102` says so in terms: the
story owns it *"as an explicit acceptance step, not as an automated assertion."*

| id | criterion | verification |
| --- | --- | --- |
| **AC-001** | **GIVEN** P4 has finished evaluating and types `cargo add happenstance@0.2.0-alpha.1` in a project that has never seen this workspace, **WHEN** Cargo resolves the manifest against the registry, **THEN** all three crates resolve at `0.2.0-alpha.1` and no internal dependency is unresolvable — which requires that `Cargo.toml:6`, **all three** `[workspace.dependencies]` requirement strings at `Cargo.toml:24-26`, and both lock files (`Cargo.lock`, `experiments/wire-format/Cargo.lock:70`) carry the pre-release **before any publish runs**, because a `version = "0.2.0"` requirement never matches a `0.2.0-alpha.1` release and the mistake is only discovered after `happenstance-core` is permanently live | Static: `cargo metadata --format-version 1` over the release tree shows **no** internal requirement of `^0.2.0` — transcript in `_release-log.md`. Gate: every `cargo xtask ci` step runs `--locked`, so a lock file that disagrees with the manifests fails the gate (`xtask/src/main.rs`). Pre-publish: `cargo publish --dry-run -p happenstance-core` green, transcript in `_release-log.md` |
| **AC-002** | **GIVEN** P2 pins `happenstance-testkit` exactly because a new conformance rule is *"thirty-four new ways for a previously passing adapter to go red"* (`crates/happenstance-testkit/Cargo.toml:5-13`), **WHEN** they read the number this release publishes, **THEN** it is `0.2.0-alpha.1` — a pre-release, so the suite's maturity matches the port it measures and no stable promise is made about a rule set over a moving contract — and it is still written as a **literal** `version` key in `[package]`, never `version.workspace = true` | Gate: `cargo xtask ci`'s *"the testkit carries its own version"* step (CF-32, `xtask/src/main.rs:425-435`), which fails on an absent key or one mentioning `workspace`. Static: `crates/happenstance-testkit/Cargo.toml:14` reads `version = "0.2.0-alpha.1"`; the manifest comment above it is preserved, not rewritten |
| **AC-003** | **GIVEN** P4 lands on the crates.io page with one pass and no second chance, **WHEN** they scan it top to bottom, **THEN** they meet `## Stability` **before the first code block they would copy** — region order `# happenstance` → the DCB sentence (lines 1-4 **verbatim**) → `## Which crate do I want?` → **`## Stability`** → `## What DCB buys you` → `## Guarantees` → `## Design` → `## Licence` — the facade blockquote (`:6-11`) is **deleted** rather than annotated, the word *facade* appears nowhere on the page, exactly **one** stability claim exists, the first fence sits no lower than it does today (`crates/happenstance/README.md:30`) and no fence exceeds 80 columns | Doctest: `cargo test -p happenstance --doc` — the README is compiled via `crates/happenstance/src/lib.rs:10`, so an edit that breaks the page breaks the gate. Static: `rg -n "facade" crates/happenstance/README.md` returns nothing; heading order read against `_design.md:782-791`. Review: anti-patterns 7 and 8 (`_design.md:987-989`) checked against the page as a reviewer sees it. Rendered: the crates.io page after publish, recorded in `_release-log.md` |
| **AC-004** | **GIVEN** P4 has found the section and is deciding whether the number is safe to depend on, **WHEN** they read `## Stability`, **THEN** it carries **exactly three things and no fourth**: the phase at which the API stops moving stated **in the reader's terms** (what changes for them, not an internal phase token), a link to `CHANGELOG.md`, and the yank policy — *"only one pre-release resolves at a time; each alpha is yanked when the next lands"* — and it names that boundary rather than hedging it, because a section that hedges has satisfied the letter of the mitigation and failed the reason it exists | Review against `_design.md:789-791` (the three-item cap) and AC-U18 (`_decomposition.md:227-232`): the criterion is *legible before the reader commits*, so a reviewer who is not the author reads the section cold and states what the crate promises. Static: the `CHANGELOG.md` link resolves from the rendered page (`_release-log.md`), and the phase sentence contains no hedge token (*may*, *hope*, *aim*) |
| **AC-005** | **GIVEN** P1 is upgrading between alphas and needs *"what broke, and why"* rather than a `git log`, **WHEN** they follow the README's link into `CHANGELOG.md`, **THEN** `## [Unreleased]` (`:24`) has become `## [0.2.0-alpha.1] — <ISO date>` with the link reference at `:1166` updated to match, no entry has been orphaned or merged by the rename, and **the file makes no claim the manifests do not honour**: the `unstable-projection` assertion (`:19-22`) is either corrected or made accurate about which crate carries the feature M5 landed | Gate: `cargo xtask lints` → `changelog_names_every_rule` (CF-29, `xtask/src/lints.rs:525`); its `entries()` parser starts a new entry at **any** heading (`:456-479`), so the rename is expected to be safe and this step is what **verifies** it rather than assuming it. Static: the feature claim checked against `crates/happenstance-core/Cargo.toml:34-52` and `crates/happenstance/Cargo.toml`'s `[features]`, both read at cut time and recorded in `_release-log.md`. Review: anti-pattern 15 (`_design.md:1005-1006`) |
| **AC-006** | **GIVEN** a licence violation or a broken feature combination is something a published crate **cannot un-publish its way out of**, **WHEN** the release is cut, **THEN** one **full** `cargo xtask ci` — not `--fast`, which drops exactly the two feature powersets, `cargo deny` and the nightly `--cfg docsrs` build — has run green on the **exact tree that publishes**, and its transcript names that tree's commit SHA | Gate: `cargo xtask ci` (`run_ci`, `xtask/src/main.rs`), run once immediately before the cut; transcript with `git rev-parse HEAD` in `_release-log.md`. Inside it, the two steps this story leans on by name: *"packaged artifacts carry their licences and README"* (D11, `xtask/src/main.rs:519-529`, implemented at `xtask/src/package.rs`), whose `reconcile` also fails if a skeleton lost its `publish = false`; and `cargo deny check` from `OPTIONAL`, which must **run** rather than skip (`--fast`'s own doc comment: *"the project-scoped bar, not the release bar"*, `:835`) |
| **AC-007** | **GIVEN** a stranger with no clone of this repository, **WHEN** they create a scratch project **outside this workspace** and add `happenstance`, `happenstance-core` and `happenstance-testkit` at an explicit pre-release requirement, **THEN** all three resolve **from the registry rather than from a path dependency** and the smallest write-then-read cycle compiles and runs against the published `happenstance` — the alpha's observable form of DoD 9 (`initiative.md:383-386`), which this story makes reachable rather than turns green. **AND** the publish itself ran `happenstance-core` first (`happenstance` cannot resolve otherwise), **AND** the yank-and-republish instruction for whoever cuts `0.2.0-alpha.2` — *yank the mistake, publish the fix at a new number, never edit in place* — is written into this story's folder **before** the cut, because after it there is nothing left to write it against | Manual, and deliberately so: **no step inside `cargo xtask ci` can reach a live registry** (DEP-006, `_decomposition.md:964-977`). The scratch project's `Cargo.toml`, its `cargo build` output and its program's stdout are pasted into `_release-log.md`, alongside the publish order transcript and a docs.rs check within the hour (best-effort — a docs.rs failure is worth knowing, not a blocker). The rollback instruction is a section of `_release-log.md` dated before the publish transcript |

**Coverage of the traced project AC.** [`project.md`](../project.md)'s **AC-011** (`:200-203`) asks
for four things: *the published version resolves* (AC-001, AC-007), *a README `## Stability` section
naming the phase at which the API stops moving* (AC-003 placement, AC-004 content), *a
`CHANGELOG.md`* (AC-005), and *the yank policy for superseded alphas* (AC-004 states it to the
reader, AC-007 records how it is executed). Its closing clause — *"a later run of
`cargo-semver-checks` has a baseline to compare against"* — is discharged by AC-007's publish
existing at all; the run itself is HS-P0016's ([`project.md`](../project.md), *Out of scope*).
AC-002 and AC-006 carry no project-AC text of their own: AC-002 is the design's sign-off item 3
(`_design.md:1252`) and CF-32's standing constraint, and AC-006 is DEP-005's *"required green before
publish"* (`_decomposition.md:953-963`) and DoD 13's bar observed at the release boundary.

## Interaction quality

This story **renders a surface** — `crate-readme` (`_design.md:54-58`) — so the composition family
is binding, taken from [`_design.md`](../_design.md), which a human approved on 2026-08-12 with no
conditions (`:1224-1232`). The medium has no DOM: *"there is no screen… what a human meets here is a
**type surface** and **four text surfaces**"* (`_design.md:12-20`). The invariants below are
therefore stated in the units this medium has — heading order, column counts, what is on the page
before the first fence — and **every one of them is carried by an AC-### row in the table above**,
never by a bullet here. A bullet in this section reaches no ledger row and is never gated.

**State family.**

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **In place, not a context jump** | AC-003, AC-005 | The stability posture is inserted into the page the reader is already on. A `STABILITY.md` the README links to would satisfy every word of AC-011 and fail P4, whose entire budget is *"a registry page and a README once"* (`personas-and-journeys.md:333-338`). Likewise the changelog is the existing `CHANGELOG.md` edited, not a new release-notes file |
| **Non-occlusion** | AC-003 | The new section must not push the first fence below where it sits today (`README.md:30`) and must not displace `## Which crate do I want?`. Deleting the 6-line blockquote is what pays for the section's lines — that is the arithmetic, not a coincidence |
| **Preserved context** | AC-003, AC-005 | Lines 1-4 of the README survive **verbatim** (DT-1 is HS-P0016's — anti-pattern 8, `_design.md:989`); every other README heading keeps its text and relative order; in the changelog, renaming one heading must orphan or merge no entry, which `changelog_names_every_rule` is run to prove rather than assume |
| **Reversibility, and its one exception** | AC-007 | Every edit in this PR is an ordinary commit. Exactly one act is irreversible — the publish — so the invariant is discharged by writing the yank-and-republish route down **before** the act, not after it (`RUNBOOK.md:4483-4484`) |
| **Reachable without a tool** | AC-004 | A reader reaches the stability posture and the changelog by following what is on the page. Requiring them to open `Cargo.toml`, clone the repository or read `spec/SPECIFICATION.md` to learn what the number promises fails this — it is AC-U14's rule (*"a reader must be able to tell what a feature turns on without opening `Cargo.toml`"*, `_design.md:830`) applied to the version string |

**Composition family** — from the signed-off `_design.md`, which this story implements and does not
re-decide.

| Invariant | Design source | Carried by |
| --- | --- | --- |
| **Presentation exists at all** | `_design.md:786-791` | AC-003 + AC-004. `## Stability` is a composed section — a heading and prose a stranger reads cold — not a badge, not a bullet appended to `## Guarantees`, not a bare `version = 0.2.0-alpha.1` line. A page carrying the *facts* with no composed section satisfies every string search and fails P4 |
| **Composition and placement** | `_design.md:782-791` | AC-003. The eight-region order, with `## Stability` **above the first code block**. Two briefs place it between `## Guarantees` and `## Design` (`_decomposition.md`, architecture item 7; DEP-003) and the design overrides them in terms at `:128-135` and again at sign-off `:1249-1251`: **AC-U18 wins on ordering** |
| **Transience** | `_design.md:810-833` (the `CHANGELOG.md` row is `:833`) | AC-004. `## Stability` is **persistent chrome** — a heading always visible on the page, never revealed by a click. `CHANGELOG.md` is **opened on demand**, linked from it. Inlining the changelog into the README inverts both |
| **Density budget, with its real numbers** | `_design.md:837-856` (the README fence row is `:850`) | AC-003 + AC-004. README code fence: **80 columns** (this story adds prose and no fence, so the budget is preserved rather than spent). Section content: **exactly three claims, no fourth** — the density budget of this region *is* its three-item cap, and a fourth claim is what yields. Page: the first fence no lower than `README.md:30` |
| **Hierarchy** | `_design.md:886-890` | AC-003. `## Stability` is **primary** on this page — *"the only region whose heading a P4 reader is scanning for and it sits above the first fence."* `## Guarantees`, `## Design` and `## Licence` stay **recessive**. Demoting the section to a sub-bullet under a recessive region satisfies "the README has a stability claim" and inverts the hierarchy that made it findable |
| **Named anti-patterns** | `_design.md:987-989`, `:1005-1006` | AC-003 carries **7** (two stability claims / the words *currently a facade* / `## Stability` below the first code block) and **8** (the opening two lines changed). AC-005 carries **15** (*the changelog claims a feature the manifest does not have*) |

**Not applicable, and why** — stated so the silence is not read as an oversight. Anti-patterns 1-6
belong to `crate-root-rustdoc` and `first-program-doctest` (M2's), 9-13 to
`worked-example-transcript` and `dsl-failure-message` (M4/M6's), 14 to the re-export table (M2's).
Focus, scroll and selection have no analogue in a rendered markdown page with no interactive state;
their intent — *the reader's place survives the change* — is what the **preserved context** row
above discharges.

## Error conditions

| id | Condition | Required behaviour |
| --- | --- | --- |
| **EC-001** | `cargo publish -p happenstance` fails on an unresolvable `happenstance-core` because a `[workspace.dependencies]` requirement string still reads `version = "0.2.0"` — **after** `happenstance-core` is already live and unremovable | The published core stays at `0.2.0-alpha.1`; nothing about it is wrong. Fix `Cargo.toml:24-26` in the tree, **re-run the full gate** (the tree that was gated is no longer the tree being published — AC-006), then publish `happenstance` at the same number. Do **not** bump to `-alpha.2` to escape the mistake: only what has already been published is frozen |
| **EC-002** | `cargo publish` fails midway on a registry error — rate limit, network, index lag — with one or two of the three crates live | Resume the sequence at the crate that failed. Never restart the release at a new number because part of it succeeded; a half-published alpha is a resumable state, not a corrupt one. Record the failure and the resumption in `_release-log.md` |
| **EC-003** | `cargo deny check` fails during AC-006's full gate — a licence, an advisory or a duplicate major | **The cut does not happen.** A licence violation found after publish is a legal problem rather than a follow-up commit (`_decomposition.md:1043-1046`). The most likely concrete instance is already named by the design: `Cbor`'s crate is **conditional** on `cargo deny check licenses` passing, and if it is refused *"the alpha ships `json` + `postcard` and `cbor` is deferred with the reason recorded"* (`_design.md:739`) — in which case the README and changelog must not claim it, which folds back into AC-005 |
| **EC-004** | `changelog_names_every_rule` (CF-29) goes red after the heading rename | Treat it as a real finding, not as parser noise. `entries()` starts an entry at any heading (`xtask/src/lints.rs:456-479`), so a red build here means an entry actually lost the rule name it was carrying. Repair the entry; do not relax the lint, and do not revert the rename to make the lint quiet — CF-29 is `[FROZEN]` (`spec/SPECIFICATION.md:8141`) |
| **EC-005** | `package-check`'s `reconcile` fails: the set derived from `cargo metadata` disagrees with its hand list (`xtask/src/package.rs:33-43`) | The cut does not happen. This fires when a skeleton lost its `publish = false` — the one bug *"no other step in the gate would ever notice"* — and publishing through it ships a `todo!()` crate to strangers under a name this project reserved |
| **EC-006** | The docs.rs build for one of the three fails after publish | **Not a blocker and not a yank.** Record it in `_release-log.md` within the hour and route it as an input to HS-P0016, whose DoD-10 bar is *"the rendered documentation build is green under all features"* (`initiative.md:387-389`). Yanking a correct crate because its documentation renderer failed spends the yank the next alpha needs |
| **EC-007** | A defect in the published `0.2.0-alpha.1` is found after the fact | `cargo yank --version 0.2.0-alpha.1`, then `0.2.0-alpha.2` carrying the fix and a `CHANGELOG.md` entry naming what broke between the alphas (DEP-007, `_decomposition.md:978-990`). Never an edit in place; never a re-publish of the same number, which the registry does not offer |
| **EC-008** | The `unstable-projection` reconciliation turns out to need a **manifest feature** rather than a changelog correction | This story **blocks** rather than widens. No `crates/*/src/**` or `crates/*/Cargo.toml` feature edit is in the boundary except the testkit's version line; the feature edit is [`projection-trait-and-runner`](../projection-trait-and-runner/spec.md)'s (`_design.md:647`). A release story that grows a feature flag has changed the artefact it was cutting |

## Non-functional

| id | Requirement | Why it is a number and not a preference |
| --- | --- | --- |
| **NF-001** | **Exactly one irreversible transition per crate, and it runs last.** The ordered sequence is: manifests and requirement strings → full gate green → `cargo publish -p happenstance-core` → `-p happenstance` (`-p happenstance-testkit` any time after core) → resolution check. No step that cannot be undone runs before the check that would have caught it | A crates.io publish cannot be edited or deleted (`RUNBOOK.md:4483-4484`). The forward-only discipline a database migration would normally get is spent here instead — see *Data and migrations* |
| **NF-002** | **The release gate is the full gate, and its cost is accepted.** `cargo xtask ci` whole, once, on the published tree — including the two feature powersets and `cargo deny`, which `--fast` drops | `run_fast`'s own doc comment says it unprompted: *"the project-scoped bar, **not the release bar**"* (`xtask/src/main.rs:835`). The project's DoD-6 bar stays `--fast` (`.redkiln/config.yaml:50-55`); this is an addition at the release boundary, not a replacement |
| **NF-003** | **Post-publish verification happens within the hour, from a clean environment.** Resolution from a scratch project outside this workspace, the crates.io page's rendered `## Stability`, and a best-effort docs.rs check | The index is eventually consistent and docs.rs builds asynchronously; a check run too early reports a false failure, and one deferred to closeout is a check nobody ran. DEP-006 sets the shape and the window (`_decomposition.md:964-977`) |
| **NF-004** | **The README stays inside its budgets.** No fence exceeds 80 columns; `## Stability` adds no fence; the first fence sits no lower than `README.md:30`; the section carries three claims and no fourth | `_design.md:837-856`. crates.io renders a comparably narrow column to docs.rs, and an over-wide fence **scrolls** rather than wraps — the single most avoidable failure on a page P4 reads once |
| **NF-005** | **The published tree and the gated tree are the same SHA.** `git rev-parse HEAD` is recorded next to the gate transcript, and any edit after that point invalidates the run | DoD 13 asks for the gate green *"on the exact tree that was published"* (`initiative.md:396-398`). A gate run against a tree that has since moved proves the wrong thing, and the changelog's own ISO date is exactly the kind of one-line edit that quietly moves it |

## Implementation notes (non-prescriptive)

Suggestions, not requirements — the acceptance criteria are the contract.

- **Do the version move as one edit, verified before anything else.** `Cargo.toml:6` and the three
  requirement strings at `:24-26` are four lines in one file; the testkit's is a fifth in another.
  A `cargo metadata` read immediately afterwards is cheap and is the only thing standing between the
  tree and EC-001. Consider `rg -n '0\.2\.0"' Cargo.toml crates/*/Cargo.toml` as the arithmetic
  check that nothing was missed — but read the hits, because
  `standards/rust/50-dependency-hygiene.md:64` carries an illustrative `version = "0.2.0"` that is
  deliberately **out of scope** (see *PR boundary*).
- **Let the lock files regenerate rather than hand-editing them.** `cargo metadata` or any `--locked`
  build after the manifest edit will refuse a stale lock, which is the signal you want;
  `experiments/wire-format/Cargo.lock:70` belongs to a **separate workspace** that pins
  `happenstance-core` by path, so it needs its own command run from that directory.
- **Write `## Stability` for someone who has never read this repository.** The phase number is an
  internal token; what P4 needs is the shape of the promise — that the API is expected to move until
  the first stable `0.2.0`, that what moved is written down, and that the previous alpha stops
  resolving when the next one lands. Three sentences is enough, and the design's cap says a fourth
  claim is what yields.
- **Reconcile the changelog's feature claim by reading both manifests at cut time, not from memory.**
  `crates/happenstance-core/Cargo.toml:34-52` is the file that decides whether the sentence at
  `CHANGELOG.md:19-22` is true, and M5 may have put the feature in `happenstance` instead
  (`_design.md:647`) — in which case the sentence needs to name the right crate rather than be
  deleted.
- **Date the changelog section with the day the publish runs, then run the gate.** If the merge and
  the cut fall on different days, editing the date afterwards moves the tree out from under NF-005.
  Sequencing the date edit *before* AC-006's gate run costs one gate run and closes the gap.
- **Keep `_release-log.md` as you go, not afterwards.** Every AC that ends in *"recorded in
  `_release-log.md`"* is an acceptance step whose evidence is unreproducible once the terminal
  scrolls: a `cargo publish` transcript, an index resolution at a particular minute, a rendered page
  before anyone touches it again.
- **Treat `cargo publish --dry-run -p happenstance` as unavailable, not as failing.** It packages the
  crate and verifies the build against the *registry* form of its dependencies, so before
  `happenstance-core@0.2.0-alpha.1` is live it cannot resolve. The dry run is a real pre-check for
  `happenstance-core` alone; the other two are checked by AC-007 after the fact, which is why the
  requirement-string arithmetic has to be right up front.

## Tests and CI (merge gate)

Grounded in the testing brief's own row for AC-011 — *"Static (packaging) + manual (registry)"*
(`_decomposition.md`, *Testing brief*, AC-011) — and its four wired grains
(`.redkiln/config.yaml:28-60`).

| Tier | Command / path | Proves |
| --- | --- | --- |
| Story grain | `cargo xtask affected --base main` (`.redkiln/config.yaml:28-40`) | fmt, `clippy -D warnings` and tests for the packages this diff touches plus their dependents. `crates/happenstance` is touched by the README edit, so its doctests are inside this grain |
| Doctest | `cargo test -p happenstance --doc`, via `crates/happenstance/src/lib.rs:10` | AC-003 — the edited README still compiles. The `compiled-as-doctest` state on the `crate-readme` surface is not decorative: prose changes on this page are checked by the compiler |
| Reachability, static | `cargo xtask lints && cargo xtask spec-trace` (`.redkiln/config.yaml:42-48`) | AC-005 — `changelog_names_every_rule` (CF-29, `xtask/src/lints.rs:525`) survives the `## [Unreleased]` rename, and `spec/SPECIFICATION.md`'s citations still resolve |
| Manifest gate | `cargo xtask ci` step *"the testkit carries its own version"* (`xtask/src/main.rs:425-435`) | AC-002 — CF-32: the testkit's key is literal, present, and does not mention `workspace` |
| Packaging gate | `cargo xtask ci` step *"packaged artifacts carry their licences and README"* (`xtask/src/main.rs:519-529`; `xtask/src/package.rs`) | AC-006 — D11: each publishable tarball actually contains both licences and a README, and `reconcile` (`package.rs:33-43`) fails if the derived publishable set has changed (EC-005) |
| Release gate | `cargo xtask ci` **whole** (`run_ci`, `xtask/src/main.rs:828-834`) | AC-006 — the two feature powersets, `cargo deny check`, the nightly `--cfg docsrs` build and all four `wasm32` steps, run once on the published SHA. `--fast` is explicitly not this bar (`:835`) |
| Pre-publish dry run | `cargo publish --dry-run -p happenstance-core` | AC-001 — the packaged contract crate builds from its own tarball with registry-form dependencies. Not available for `happenstance` until core is live (see *Implementation notes*) |
| Manual, registry | a scratch project **outside** this workspace; transcripts in `.bklg/.../publish-0-2-0-alpha-1/_release-log.md` | AC-007 — resolution at an explicit pre-release requirement against the index, a write-then-read cycle compiled and run, the rendered crates.io page, docs.rs best-effort. **No gate step can reach a live registry** (DEP-006) |
| Human review | the story's review gate, against [`_design.md`](../_design.md)`:782-791`, `:837-856`, `:886-890`, `:987-1006` | AC-003, AC-004 — the composition invariants. Every machine check in this repository is satisfied by a README that carries the right words in the wrong order; this is the instrument that is not |

**Merge gate for this story:** the story grain green, the reachability grain green, and
`cargo xtask ci` **whole** green on the merge SHA. The project's own DoD-6 bar
(`cargo xtask ci --fast`) is subsumed by the full run and is not run separately.

## Risks and coupling (PR-scoped)

| Risk | Blast radius | Mitigation in this PR |
| --- | --- | --- |
| **The requirement strings are forgotten** (`Cargo.toml:24-26`) | The highest-cost mistake available here. `happenstance-core` is live and unremovable before the failure surfaces on the second `cargo publish` | AC-001 makes the four-line edit and its `cargo metadata` verification a criterion rather than a step; EC-001 states the recovery so nobody improvises a version bump under pressure |
| **The alpha becomes an argument against changing things** ([`project.md`](../project.md), risk row 1) | The whole remaining runbook. *"An alpha in the wild makes 'we can't change that now' available as an argument"* (`RUNBOOK.md:4157-4162`) — and the failure mode is the author, not the users | The three mitigations are AC-003, AC-004 and AC-005 — observable artefacts a reviewer can fail, not intentions. AC-004's no-hedge clause is the specific guard: a section that names no boundary has satisfied the letter and lost the reason |
| **The `unstable-projection` claim needs a manifest edit** | Would pull `crates/*/Cargo.toml` feature blocks into a release story's diff | EC-008: this story blocks on [`projection-trait-and-runner`](../projection-trait-and-runner/spec.md) rather than widening. The PR boundary's fenced list is what enforces it — `redkiln verify --grain story` fails on any file outside it |
| **`cargo deny` refuses the CBOR crate at the release gate** | `Cbor` is already **conditional** in the design's visibility table (`_design.md:739`); a late refusal changes what the README and changelog may claim | EC-003 routes it: the alpha ships `json` + `postcard`, the reason is recorded, and AC-005's reconciliation absorbs the claim. The gate runs `deny` **before** the cut precisely so this is a document edit and not a legal problem |
| **The testkit's number moves *down*** (`0.2.0` → `0.2.0-alpha.1`) | Reads as a mistake to anyone who does not know CF-32 gives it an independent number and the design's sign-off item 3 chose this deliberately | AC-002 requires the manifest comment at `crates/happenstance-testkit/Cargo.toml:5-13` to be preserved rather than rewritten, so the file explains itself; the reasoning is in the *Context pack*, item 4 |
| **The published tree drifts from the gated tree** | DoD 13 is asserted rather than observed | NF-005 pins `git rev-parse HEAD` next to the transcript; the changelog date is edited *before* the gate run, not after |
| **Coupling: all four blocking stories must be merged, not merely written** | The published tree **is** the merged tree — publishing early ships an artefact whose defect log, wasm claim, projection verdicts and compile-fail proof do not exist in it | *Dependencies* below, and [`_storymap.md`](../_storymap.md)`:130-132`: this story runs **last** in M7 because *"the publish is the one irrevocable act in the project"* |
| **Coupling forward: HS-P0016 inherits this number as its baseline** | `cargo-semver-checks`, the MSRV promise and DoD 11 all diff against whatever this release actually contains | Nothing is done about it here beyond publishing something honest — but AC-006's full gate and AC-007's resolution check are what make the baseline trustworthy rather than merely present |

## Dependencies

**Blocks on** — all four are M7 or earlier and all four must be **merged**, not merely specified,
because the tree this story publishes is the tree they left behind:

| Story slug | Why this story cannot cut before it |
| --- | --- |
| [`compile-fail-proof-artefact`](../compile-fail-proof-artefact/spec.md) | AC-002's proof artefact is registered in `xtask/src/proof.rs`'s `ARTEFACTS` and is therefore **inside `cargo xtask ci`**. A full gate run that does not include it is not the release bar this story's AC-006 claims to have run |
| [`projection-clause-verdicts`](../projection-clause-verdicts/spec.md) | It edits `spec/SPECIFICATION.md`, which `cargo xtask spec-trace` — a gate step — reads. Publishing before it lands ships a specification whose verdicts are missing from the artefact strangers will cite |
| [`edge-flavour-and-wasm-claim`](../edge-flavour-and-wasm-claim/spec.md) | It settles whether a fifth `wasm32` step compiling `happenstance` joins `REQUIRED` (`xtask/src/main.rs:105`). If it does, that step is part of the full gate AC-006 must run; if it does not, the recorded statement of what the typed layer claims on `wasm32` is part of what this release publishes |
| [`defect-log-and-macros-verdict`](../defect-log-and-macros-verdict/spec.md) | BR-01's defect log is the record this project exists to produce, and it is staged for `/redkiln:kb-ingest` from the tree. Cutting first would publish the artefact before the findings it produced were written down |

**Unlocks** — nothing inside this project; it is HS-P0011's last story
([`_storymap.md`](../_storymap.md)`:130-132`). Downstream:

- **`publication-and-positioning` (HS-P0016)** —
  `.bklg/from-contract-to-published-library/publication-and-positioning/project.md`. Its
  `cargo-semver-checks` verdict, MSRV promise, registry presentation and DT-1 all need a **prior
  published baseline**, which does not exist until this story runs.
- **Initiative DoD 11** (`initiative.md:390-392`) — *"the release is diffed, not asserted"* — has
  nothing to diff against before this publish.
- **Initiative DoD 9** (`:383-386`) moves from unreachable to observable: after this story a version
  exists on the registry that a scratch project can add and resolve. Its full form, against the
  stable `0.2.0`, stays HS-P0016's.

## Anchors (progressive disclosure)

Load-bearing depth, deferred rather than pasted. Every path verified to exist in the worktree.

| Anchor | Why it is load-bearing | When to open it | Serves |
| --- | --- | --- | --- |
| `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_design.md` | The **signed-off, binding** composition of `crate-readme`: region order (`:782-791`), the three-item cap on `## Stability` (`:789-791`), the transience policy (`:810-833`), the density budgets (`:837-856`), the hierarchy that makes the section primary (`:886-890`), the anti-patterns (`:969-1006`, with 7/8 at `:987-989` and 15 at `:1005-1006`), and the sign-off overrides (`:1249-1252`) that beat two briefs and set the testkit's number | **Before writing a single character of the README edit**, and again before the testkit's version line | AC-002, AC-003, AC-004 |
| `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_decomposition.md` | The **deployment brief** (`:877-1090`) — DEP-001…DEP-007 in full: the publishable set, the pre-release-is-unreachable-by-accident mechanism, publish order, the packaging gate, the resolution check and the rollback posture. Also AC-U18 (`:227-232`), the criterion that put `## Stability` above the fence | Before the version edit (DEP-001/DEP-004) and again before the publish handoff (DEP-006/DEP-007) | AC-001, AC-004, AC-006, AC-007 |
| `RUNBOOK.md` | Why the number is `0.2.0-alpha.1` and not `0.1.0` (`:4108-4124`); the three churn mitigations in the words that generated them (`:4148-4155`); *the failure mode is the author, not the users* (`:4157-4162`); a crates.io release cannot be edited (`:4483-4484`) | Before writing `## Stability`, and before anyone proposes rounding the number down | AC-001, AC-004, AC-007 |
| `crates/happenstance/README.md` | The page being edited. Lines 1-4 are untouchable, `:6-11` is the blockquote to delete, `:30` is the first fence the section must sit above | The moment the README edit starts | AC-003, AC-004 |
| `CHANGELOG.md` | `:19-22` is the `unstable-projection` claim to reconcile; `:24` is the heading to date; `:1166` is the link reference that must move with it | Before the changelog edit | AC-005 |
| `crates/happenstance-core/Cargo.toml` | `[features]` at `:34-52` — `default`, `std`, `serde`, `memory` and nothing else. The file that decides whether `CHANGELOG.md:19-22` is true | At cut time, read rather than remembered, alongside `crates/happenstance/Cargo.toml`'s own `[features]` | AC-005 |
| `Cargo.toml` | The mount point: `[workspace.package] version` at `:6`, and the three internal requirement strings at `:24-26` whose omission is EC-001 | First edit of the story | AC-001 |
| `crates/happenstance-testkit/Cargo.toml` | `:14` is the number that moves; `:5-13` is the comment explaining why the key is literal, which must survive the edit | With the version edit | AC-002 |
| `xtask/src/main.rs` | `run_ci` (`:828-834`) and `run_fast`'s doc comment (`:835-848`) — the gate's own statement that `--fast` is *not the release bar*; the CF-32 step (`:425-435`); the D11 step (`:519-529`); `REQUIRED` (`:105`) | Before running the release gate, and when deciding whether a `--fast` run could substitute (it cannot) | AC-002, AC-006 |
| `xtask/src/package.rs` | `:20-43` — why the publishable set is **derived** from `cargo metadata` and reconciled against a hand list, and the one bug (`publish = false` deleted by accident) no other gate step would notice | If `package-check` fails (EC-005), before assuming it is a false positive | AC-006 |
| `xtask/src/lints.rs` | `entries()` (`:454-479`) — the changelog parser starts a new entry at **any** heading, which is why the `## [Unreleased]` rename is expected to be safe; `changelog_names_every_rule` (`:525`) is the check that proves it | Before the changelog heading edit, and immediately after it | AC-005 |
| `spec/SPECIFICATION.md` | CF-32 at `:8200` (the testkit's own `version` key) and CF-29 at `:8141` (a rule lands with its changelog entry) — both `[FROZEN]`, so both are constraints the edits must not break rather than clauses this story may amend | When the testkit version or the changelog headings are touched | AC-002, AC-005 |
| `crates/happenstance/src/lib.rs` | `:1-10` — the `cfg(doctest)` include that compiles this README. It is why an edit to a markdown file is checked by the compiler, and why a malformed fence in the new section fails the gate | Before assuming a README edit is untested | AC-003 |
| `.redkiln/config.yaml` | `:28-60` — the four wired verify grains, including `integration_scoped: cargo xtask ci --fast` at `:55`, the project bar this story deliberately supersedes at the release boundary | When justifying why the full gate runs here and nowhere else in the project | AC-006 |
| `.bklg/from-contract-to-published-library/initiative.md` | DoD 9 (`:383-386`), DoD 11 (`:390-392`) and DoD 13 (`:396-402`), plus BR-05 (`:288`) — what this release is measured against at initiative level, and precisely how far it gets | When writing the resolution check, so it is DoD 9's shape rather than an ad-hoc smoke test | AC-006, AC-007 |
| `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_storymap.md` | `:130-132` (this story runs last in M7 and why), `:100-102` (the resolution check is an explicit acceptance step, not an automated assertion), `:163-166` (`cargo publish` is a **human handoff**) | Before sequencing the cut, and before anyone tries to automate the publish | AC-007 |
| `.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md` | `:249-263` and `:333-338` — P4, the evaluator, whose one-shot time-boxed pass is the reason `## Stability` has a position rather than merely a presence | When judging whether the README edit reads cold to someone who has never seen this repository | AC-003, AC-004 |
| `.bklg/from-contract-to-published-library/publication-and-positioning/project.md` | The project that inherits this release as its baseline — the boundary of what this story must **not** promise (MSRV, semver verdict, registry presentation, DT-1) | If a reviewer asks why the alpha does not do more | AC-007 |

## Clarifications resolved during spec

1. **The seven AC ids the front half decided are kept exactly, and their mapping is stated here.**
   AC-001 version and requirement strings; AC-002 the testkit's number; **AC-003 the README's
   composition and placement**; **AC-004 the README's content and transience**; AC-005 the changelog
   (dated section *and* the `unstable-projection` reconciliation, folded into one row because both
   are the same claim — *the changelog is true and released*); AC-006 the full gate; AC-007 publish
   order, resolution check and the recorded rollback route. Splitting the README across two rows and
   folding the changelog into one is deliberate: RFC §6.7/D6 requires every composition invariant to
   be a **gated ledger row**, and `redkiln verify` extracts ACs only from a leading `| AC-001 |` cell
   or a `- AC-001:` bullet — a composition invariant living as prose would never be tested. Nothing
   was added or dropped.
2. **Two briefs place `## Stability` between `## Guarantees` and `## Design`; this spec follows the
   design instead.** `_decomposition.md`'s architecture item 7 and DEP-003 both say the lower
   position; [`_design.md`](../_design.md) rejects it in terms (`:128-135`) and the human sign-off
   accepted the override explicitly as item 2 (`:1249-1251`). **AC-U18 wins on ordering.** This is
   recorded rather than silently resolved because a future reader will find the briefs first.
3. **`happenstance-testkit` publishes `0.2.0-alpha.1`, not a stable `0.2.0`.** The deployment brief
   flagged it as open (`_decomposition.md:1056-1072`); sign-off item 3 (`_design.md:1252`) settled
   it. CF-32 gives the testkit an independent **number**, not an independent **maturity**.
4. **The changelog's `unstable-projection` claim has exactly two admissible resolutions, and the
   third is refused.** Correct the sentence, or make it accurate about which crate carries the
   feature M5 landed. **Adding a feature to a manifest from inside this story is not available** —
   EC-008 blocks on [`projection-trait-and-runner`](../projection-trait-and-runner/spec.md) instead.
5. **Whether the alpha's section replaces `## [Unreleased]` or sits beneath a new empty one is left
   open, deliberately.** Both are conformant Keep-a-Changelog and neither changes what CF-29 sees.
   What is **not** open: the link reference at `CHANGELOG.md:1166` must match whichever shape is
   chosen, which AC-005 requires.
6. **The changelog's ISO date is the publish date, and it is edited before AC-006's gate run.** The
   alternative — dating it at merge and correcting it at the cut — moves the tree after it was
   gated, which is exactly what NF-005 and DoD 13 forbid. One extra gate run is the price.
7. **A static lint asserting `[workspace.package] version` equals every internal requirement string
   would be the durable fix for EC-001, and it is not written here.** `xtask/**` is outside this
   story's PR boundary, and a release story that grows a gate step has changed the gate it was
   running. Recorded as a residual for HS-P0016, which owns the next release's mechanics.
8. **AC-007's write-then-read cycle is the alpha's form of DoD 9, not DoD 9 itself.** DoD 9 names
   *the published crate* and its full form is HS-P0016's stable `0.2.0`
   ([`project.md`](../project.md), *Out of scope*). This story makes it observable; it does not claim
   it green, and the closeout should not record it as such.
9. **`_release-log.md` is named as this story's evidence artefact.** The front half committed to
   *"recorded as evidence in this story's own folder"* without naming a file; every manual
   acceptance step above now cites the same one, so the ledger's evidence column has a real path to
   point at rather than a description.
10. **Three anti-pattern line references in the front half point one section short, and the correct
    ones are used from *Acceptance criteria* onward.** The front half cites anti-pattern 7 at
    `_design.md:975-977`, anti-pattern 8 at `:978` and anti-pattern 15 at `:991-993`; re-read
    against the file, 7 is at `:987-988`, 8 at `:989` and 15 at `:1005-1006` (`:975-993` lands on
    anti-patterns 1-10). The front half is settled and is left as written — the *content* of every
    one of those citations is correct and the anti-pattern **numbers** are what actually bind. This
    note exists so an implementer who opens the cited range and finds the crate-root page's
    anti-patterns does not conclude the design changed underneath them. Open `## Anti-patterns`
    (`_design.md:969`) and read by number.
