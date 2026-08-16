---
item: "HS-S0033"
stage: implement
created: "2026-08-16"
updated: "2026-08-16"
---

# Implementation Report — 0.2.0-alpha.1 on the registry, with its churn mitigations

**Six of seven ACs are satisfied. AC-007 is BLOCKED, and the blocker is the publish
itself.**

`cargo publish` against the live crates.io index is a **human handoff**, declared as one
by the story map (`_storymap.md:163-166`) and unreachable by any gate step (DEP-006). It
has not been run. No registry resolution has been observed, so none is claimed: AC-007's
ledger row stays `satisfied: false` and says why.

Everything up to the cut is done and green. The version has moved, both lock files agree,
the README carries `## Stability` above the first code block, `## [Unreleased]` has become
the alpha's dated section with its `unstable-projection` claim reconciled against the
manifests read at cut time, `cargo publish --dry-run -p happenstance-core` is green, and
**`cargo xtask ci` whole — not `--fast` — reports `all checks passed` with all four
`OPTIONAL` steps having RUN rather than skipped.**

## TDD Evidence

**This story writes no code, so there is no red-then-green in the usual sense** — the
testing brief's own row for AC-011 is *"Static (packaging) + manual (registry)"*. What
replaces it is a set of checks that each ran, and **three of them came back red first**.
That is the substance of this section.

### Red 1 — the workspace stopped resolving, and this is the story's most valuable failure

The version move went in, and:

```console
$ cargo metadata --format-version 1
error: failed to select a version for the requirement `happenstance-core = "^0.2.0"`
candidate versions found which didn't match: 0.2.0-alpha.1
required by package `outside-projection-adapter v0.2.0-alpha.1`
help: if you are looking for the prerelease package it needs to be specified explicitly
    happenstance-core = { version = "0.2.0-alpha.1" }
```

`examples/outside-projection-adapter/Cargo.toml` carries its requirements **long-hand on
purpose** — the crate exists to model an outside author's manifest, and its own comment
says `workspace = true` would misreport the cost. So it has two `version = "0.2.0"`
requirement strings that the spec's PR-boundary list did not enumerate, and a `"0.2.0"`
requirement does not match a `0.2.0-alpha.1` candidate.

**This is EC-001's failure class arriving locally instead of on the registry.** The same
mistake in `Cargo.toml`'s three `[workspace.dependencies]` strings would have surfaced on
the *second* `cargo publish` — after `happenstance-core` was live and unremovable, with a
new version number as the only way out. Finding it here, for free, is exactly why AC-001
makes the `cargo metadata` read a criterion rather than a step.

Green after the two lines moved. The example is `publish = false`, so no published
artefact changes. The boundary widening is declared in *Notes*.

### Red 2 — `lint-constitution`, twice

Adding comment lines to `Cargo.toml` and to the testkit's manifest moved two cited
anchors:

```console
standards/rust/52-wasm32-and-target-cfg.md:106 — `crates/happenstance-testkit/Cargo.toml:67`
  no longer has `optional = true` within 10 lines
standards/rust/90-skeletons-and-todo.md:269 — `./Cargo.toml:154`
  no longer has `the allow protected nothing` within 10 lines
```

Re-anchored at `:83` and `:169`; `27 atoms, all consistent`. Separate commit, following
this branch's precedent (`c4e36c4`, `7abff7d`) — `standards/rust/**` is outside this
story's fence and re-anchoring a citation at the text it already named changes no claim.

### Green 1 — CF-29 survives the changelog rename, and that was verified rather than assumed

AC-005's verification column says this step *"is what **verifies** it rather than
assuming it"*:

```console
$ cargo run -p xtask -- lint-changelog
CF-29: all 112 rules in 4 file(s) have a changelog entry
```

`changelog_names_every_rule`'s parser starts a new entry at **any** heading, so
`## [Unreleased]` → `## [0.2.0-alpha.1] — 2026-08-16` is safe by construction. This is the
run that shows it.

### Green 2 — the pre-publish dry run

```console
   Packaging happenstance-core v0.2.0-alpha.1
    Packaged 26 files, 434.4KiB (123.9KiB compressed)
   Verifying happenstance-core v0.2.0-alpha.1
   Compiling happenstance-core v0.2.0-alpha.1 (target\package\happenstance-core-0.2.0-alpha.1)
    Finished `dev` profile
warning: aborting upload due to dry run
```

`happenstance` and `happenstance-testkit` **cannot** be dry-run yet, and the spec says to
treat that as unavailable rather than failing: `--dry-run` verifies against the *registry*
form of each dependency, and `happenstance-core@0.2.0-alpha.1` is not live.

### Green 3 — the release gate, whole

`cargo xtask ci`: **all checks passed**, 23 sections. **All four `OPTIONAL` steps ran** —
`feature powerset`, `wasm32 feature powerset`, `licences and advisories`,
`docs.rs configuration (nightly)` — none printed `skipped:`. Five `wasm32` sections. Full
transcript in `_release-log.md` §4.

## Commits

- `<sha1>` — `feat(typed-layer-and-alpha-release): Publish 0.2.0-alpha.1`
- `<sha2>` — `fix(typed-layer-and-alpha-release): re-anchor two constitution citations`

## Changes

| File | Shape of the change |
| --- | --- |
| `Cargo.toml` | `[workspace.package] version` → `0.2.0-alpha.1` with the reason for the pre-release; all three `[workspace.dependencies]` requirement strings moved with it, under a comment naming the failure mode of getting one wrong |
| `crates/happenstance-testkit/Cargo.toml` | Literal `version = "0.2.0-alpha.1"` (CF-32). The existing comment is **preserved** and extended by the reason the number moves *down* |
| `examples/outside-projection-adapter/Cargo.toml` | Two requirement strings, **forced** — see *Red 1*. `publish = false`, so no published artefact changes |
| `crates/happenstance/Cargo.toml` | One comment line: it asserted `[workspace.dependencies]` carries `version = "0.2.0"`, which the version move made false. Prose only |
| `Cargo.lock`, `experiments/wire-format/Cargo.lock` | Regenerated; the second from its own workspace directory |
| `crates/happenstance/README.md` | Facade blockquote **deleted**; `## Stability` inserted above the first fence with exactly three claims. Lines 1–4 untouched; the first fence stays at `:30` |
| `CHANGELOG.md` | `## [Unreleased]` → `## [0.2.0-alpha.1] — 2026-08-16` with a lead paragraph; the link reference at the file's end updated; the `unstable-projection` claim reconciled to name both crates |
| `standards/rust/52-…`, `standards/rust/90-…` | Two re-anchored citations. Separate commit |
| `.bklg/.../publish-0-2-0-alpha-1/` | `_release-log.md` (new), `_ledger.md`, and these two reports |

**No `crates/*/src/**` path is in the diff**, so no public Rust item was added, removed or
gated. A release story that grows a feature flag has changed the artefact it was cutting.

## Gates

| Gate | Result |
| --- | --- |
| `cargo metadata --format-version 1` | exit 0; **no** internal requirement on `^0.2.0` |
| `cargo publish --dry-run -p happenstance-core` | green — packaged, verified, compiled from its own tarball |
| `cargo run -p xtask -- lint-changelog` | `CF-29: all 112 rules in 4 file(s) have a changelog entry` |
| `cargo run -p xtask -- lint-constitution` | `27 atoms, all consistent` |
| `cargo fmt --all --check` | clean |
| **`cargo xtask ci` (whole)** | **all checks passed** — four of four `OPTIONAL` steps RAN |
| `cargo xtask affected --base main` | affected gate passed |
| `cargo publish` | **NOT RUN — human handoff** |

The project's own DoD-6 bar (`cargo xtask ci --fast`) is subsumed by the full run and was
not run separately, exactly as the spec's merge gate says.

## Notes

**The blocker, stated plainly.** AC-007 needs three things and has two. The publish order
is fixed and its reason recorded; the yank-and-republish instruction is written **before**
the cut, which is the half that becomes unwritable afterwards. What is missing is the act:
three `cargo publish` invocations against a live index, a scratch project outside this
workspace resolving all three at an explicit pre-release requirement, the rendered
crates.io page, and a docs.rs check. `_release-log.md` §5 is a four-item checklist for
whoever runs it, and the ledger row is flipped only when those four are in the file.

**One boundary widening, forced and declared.**
`examples/outside-projection-adapter/Cargo.toml` is not in the spec's fenced list and had
to change, because otherwise the workspace does not resolve at all and there is no tree to
gate. Two requirement strings, in a `publish = false` example. The alternative — reverting
the version move — is not available, and improvising a different version number under
pressure is precisely what EC-001 exists to prevent.

**A second, smaller one.** `crates/happenstance/Cargo.toml` gained one **comment** edit:
its NF-006 paragraph asserted that `[workspace.dependencies]` carries `version = "0.2.0"`
on this crate, which the version move made false. Leaving a false sentence in a manifest
that explains a deliberate spelling is worse than the one-line diff. No key changed.

**`Cbor` ships.** The design's visibility table left it **conditional** on
`cargo deny check licenses` passing for `ciborium`'s subtree. The full gate ran `cargo deny
check` and it passed against the **unmodified** allowlist, so EC-003's fallback — ship
`json` + `postcard` and record the reason — did not fire.

**The `## Stability` section is terser than first drafted, and the reason is a criterion.**
AC-003 requires the first fence to sit **no lower than `:30`**. A three-paragraph section
put it at `:38`. The section was compressed to a three-item list and the fence is back at
exactly `:30` — the blockquote's removal paying for the new section, which is what deleting
rather than annotating it was for. The mechanism a reader may want next (a yank keeps
existing lock files working) is one hop away in `_release-log.md` §1 rather than a fourth
claim the design's cap forbids.

**What was deliberately not touched.** No `.github/workflows/` change: no workflow
references `publish` or `crates.io` today and this story does not add one.
`standards/rust/50-dependency-hygiene.md:64`'s illustrative `version = "0.2.0"` TOML block
is an example rather than a resolved citation and is untouched. The README's opening two
lines and the `## What DCB buys you` code block are DT-1's and M2's respectively.
`## Guarantees`' MSRV sentence is left alone: it is an MSRV statement, not a second API
stability claim, and turning it into a supported-versions promise is HS-P0016's.
