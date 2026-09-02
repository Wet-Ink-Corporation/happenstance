---
item: HS-S0072
stage: discover
created: 2026-08-12T13:02:41.012Z
updated: 2026-08-12T13:02:41.012Z
template_sig: 86ce4036
rendered_sig: ee622fb9
---

# Discover — Neither crate is a skeleton any more

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| The slice's one line: no `todo!()`, no scoped `#![allow(clippy::todo)]`, no `publish = false`, `PUBLISHABLE` grown to five with licences and READMEs in place, and the Hyperdrive note written as explicitly unsupported | `_storymap.md`, *Slices* table, `deskeleton-and-package-readiness` row | Readiness, not release. Publishing is HS-P0016's |
| **AC-012** — no `todo!()` on either path, the scoped allow deleted from both crates, `publish = false` removed, and both names claimed on crates.io | `project.md`, *Acceptance criteria*, AC-012 | This story owns the skeleton-marker removal and the package-surface reconciliation; `claim-crate-names` owns the names |
| `depends_on: claim-crate-names`, `postgres-structural-bill`, `neon-conflicting-position-verdict`, `postgres-projection-store` | manifest; `_storymap.md`, *Merge order* item 6 | Terminal by construction: it "cannot honestly run until the last `todo!()` in either crate is gone", and it cannot honestly remove `publish = false` from a name nobody holds |
| The change is three-part and `reconcile` fails in **both** directions | `xtask/src/package.rs:86` (`PUBLISHABLE`, three names today), `:94` (`REQUIRED_FILES`), `:172-212` (`reconcile`) | Deleting `publish = false` without growing `PUBLISHABLE` fails as "Cargo will publish X but this step does not check it"; growing it without deleting fails as "the crate has gained a `publish = false`" |
| **Neither crate directory holds any of the three required files today** | `crates/happenstance-postgres/` and `crates/happenstance-neon/` contain only `Cargo.toml` and `src/`; compare `crates/happenstance-core/` | `LICENSE-MIT`, `LICENSE-APACHE` and `README.md` must be copied in. Both licences because `MIT OR Apache-2.0` "is a choice the consumer makes and a choice needs both texts" (`xtask/src/package.rs:88-94`) |
| `readme = "README.md"` pointing at a file outside the artifact "fails silently" | `xtask/src/package.rs:90-93` | Which is why `package-check` asserts the files are inside what `cargo package --list` reports, rather than merely that they exist somewhere |
| `cargo xtask package-check` is already `REQUIRED`, so it starts asserting five crates the moment the list grows | `_decomposition.md`, *Deployment brief* → *Notes*, "CI implication, concretely" | A change in what an **existing** default-gate step checks, not a new step |
| The scoped allow names this phase in both crates, and "disappears with the last `todo!()` rather than outliving it" | `crates/happenstance-postgres/src/lib.rs:58-63`; `crates/happenstance-neon/src/lib.rs:101-105` | `clippy -D warnings` catches a stray allow only if a `todo!()` remains under it — the allow itself is not linted for being unnecessary |
| **Twenty-three `todo!()` call sites exist across the two crates today** — Neon 10 in `event_store.rs` and 4 in `projection_store.rs`, Postgres 3 in `event_store.rs`, 4 in `projection_store.rs` and 2 in `read_stream.rs` | grep over `crates/happenstance-postgres/src/` and `crates/happenstance-neon/src/` | The four at `crates/happenstance-neon/src/projection_store.rs:126`, `:136`, `:209`, `:214` are claimed by **no story in this project** and by no AC: AC-005 is Postgres-only. See question 1 — this is the story that discovers it if `spec` does not |
| `NeonProjectionStore` already carries two recorded findings against the port — `begin` and `rollback` are `async` and fallible with nothing to open and nothing to fail | `crates/happenstance-neon/src/projection_store.rs:16-23` | "recorded as findings rather than fixed here". Removing its `todo!()`s must not quietly convert a finding into an invented body |
| The workspace version is already `0.2.0`, inherited | `Cargo.toml` `[workspace.package]`; `_decomposition.md`, *Deployment brief* → *Notes*, "Release path" | Neither crate needs a version bump when `publish = false` goes |
| `cargo xtask wasm` selects steps **by name** and panics on a miss | `xtask/src/main.rs:785-790`; the Neon wasm32 step at `:264-282` | Renaming the step while tidying the crate is a gate change. DR-8 keeps that step green regardless |
| `xtask/src/proof.rs`'s `ARTEFACTS` asserts each phase's proof artefact still holds the tests its clauses name | `xtask/src/proof.rs:133`; `_decomposition.md`, *Architecture brief* §7 | In the blast radius of any test-target reshuffle this story performs |
| The Hyperdrive note is documentation only: "a note, not a supported configuration: it needs a forked driver with unnamed-statement support and a hand-rolled binding" | `RUNBOOK.md:4364-4366`; `_decomposition.md`, *Deployment brief*, unlabeled AC row | No code, no CI job, no test |
| `cargo deny check` walks with `[graph] all-features = true` against an allowlist of MIT / Apache-2.0 / Apache-2.0-WITH-LLVM-exception / BSD-2 / BSD-3 / ISC / Unicode-3.0 / Zlib | `deny.toml:1-19` | Whatever `neon-sql-transport` settled about a shipped transport is what `cargo deny` now judges as publishable dependency surface |

## Questions

**Answered, and the first is a scope gap this story exists to surface.**

1. *Does "no `todo!()` on either path" include `NeonProjectionStore`?* By the words
   of AC-012 and `RUNBOOK.md:4383`, yes — and **no story in this project owns those
   four bodies.** AC-005 names `PostgresProjectionStore` only; `postgres-projection-store`
   (HS-S0071) is Postgres-scoped; `neon-append-and-read-over-http` (HS-S0069) is the
   event-store path. There are three admissible resolutions and `spec` must pick one
   explicitly rather than discover it under schedule pressure: implement the four
   bodies against HS-P0010's frozen `Batch` (scope this project did not accept);
   ship `happenstance-neon` with its `projection-store` feature off by default and
   the module excluded from the published artifact (a manifest decision with
   consequences for what the crate claims); or record the residue and defer AC-012's
   completion for that module to a named successor. **What is not admissible is
   inventing bodies here** — see the first mutant.
2. *Is the package change one edit or three?* Three, and `reconcile` fails in both
   directions, so a partial change is caught by the existing gate
   (`xtask/src/package.rs:172-212`).
3. *Does either crate need a version bump?* No; both inherit the workspace's
   `0.2.0`.
4. *Does this story publish?* No. `cargo package -p … --list` succeeding with the
   required files is the bar. `cargo publish`, the `0.2.0` release, the semver diff
   and the clause-ledger audit are `publication-and-positioning` (HS-P0016)'s.

**Deferred to `spec`.**

5. *The two READMEs' content*, against what crates.io renders as each crate's front
   page, and whether the licence texts are copied or symlinked (symlinks and
   `cargo package` interact badly enough to be decided rather than assumed).
6. *Where the Hyperdrive note lives* — crate rustdoc or `README.md` — and its exact
   "not supported" wording.

**Deferred to the owning stories.**

7. *How the adapter buys ES-10's visibility invariant.*
   `adr-0024-position-visibility-mechanism` (HS-S0065), consumed transitively.
8. *Whether `conflicting_position` is a promise every adapter owes or a hint one may
   omit.* `neon-conflicting-position-verdict` (HS-S0070), which is a dependency of
   this story precisely so its `todo!()`s are gone before the markers come off.

## Decision

The problem this slice solves is that a skeleton marker is a promise about the
crate's honesty, and removing it is the moment that promise is either kept or
quietly broken. Both crates today say the same three things — every body that would
speak to a server is `todo!()`, `clippy::todo` is allowed at crate scope with a
comment naming this phase, and `publish = false` — and all three are load-bearing:
they are how a reader knows the associated types are real and the behaviour is not.
This story removes all three, in both crates, and reconciles the package surface
that removal implies: `PUBLISHABLE` from three names to five, both licence texts and
a README copied into each crate directory where neither holds any today, and the
Hyperdrive deployment note written as explicitly unsupported. It is readiness, not
release. The spec will cover: an enumeration of every remaining `todo!()` with its
owning story, including an explicit resolution for `NeonProjectionStore`'s four,
which no story in this project currently claims; the three-part package change; the
two READMEs and the licence copies; the `cargo package --list` evidence for both
crates; the Hyperdrive note; and confirmation that the Neon wasm32 step's name is
unchanged. No `[FROZEN]` clause is read or changed, so no ADR is owed.

## The wrong implementation

**A body conjured to delete a `todo!()`.** This is the story where the pressure to do
it is highest, and `NeonProjectionStore` is where it will land: four `todo!()`s
(`crates/happenstance-neon/src/projection_store.rs:126`, `:136`, `:209`, `:214`) that
AC-012's wording requires gone and that no acceptance criterion in this project
requires *working*. The cheapest resolution is to replace each with something that
compiles and returns — an `Err(NeonError::…)` "not supported", a `Vec::new()` for
`begin` because the module doc already observes there is nothing to open, an `Ok(())`
for `rollback` because dropping the statement list *is* the rollback. Every check
passes, and passes more cleanly than before: `clippy -D warnings` is green because no
`todo!()` remains under the allow, the allow is deleted, `cargo xtask package-check`
reconciles five crates, `cargo package --list` succeeds, `cargo deny` is clean, the
wasm32 build is green. And `happenstance-neon` is now a published crate under a
claimed name at a semver-binding version whose `ProjectionStore` impl fails, or
silently no-ops, for every caller who enables its default `projection-store` feature.
The skeleton marker existed to make exactly that state unreachable; deleting the
marker as the *goal* inverts it. The module's own doc records `begin` and `rollback`
as **findings against the port** — "recorded as findings rather than fixed here"
(`crates/happenstance-neon/src/projection_store.rs:16-23`) — and a conjured body
converts a finding into an implementation without anyone deciding to.

Nothing mechanical rejects this: `todo!()` is the only marker the gate can see, and
it is precisely what the mutant removes. The rejection is the enumeration `spec`
owes — every remaining `todo!()`, with its owning story, and an explicit,
recorded resolution for the four that have none. A body written because a checklist
said "no `todo!()`" is the same class of act as an adapter that compiles and has
never run the suite.

**The second mutant is the same move at manifest scale: `publish = false` removed
from a crate whose default features expose an unimplemented port.** `PUBLISHABLE`
agrees, the files are in place, `reconcile` is happy in both directions
(`xtask/src/package.rs:172-212`), and the publishable set has grown to five with a
green gate. The check verifies that Cargo *will* publish what the list says; it has
no opinion about whether what will be published works. AC-012 is the last gate
before HS-P0016 turns the crate into a promise, and `cargo package --list` is not an
instrument for that.

**And the near-miss worth recording for contrast:** deleting `publish = false`
without touching `PUBLISHABLE`, or renaming the Neon wasm32 step while tidying. Both
fail loudly and immediately — `reconcile` names the direction and the remedy
(`xtask/src/package.rs:186-201`), and `wasm_steps()` panics on a name miss
(`xtask/src/main.rs:785-790`). The gate is strong exactly where the mutants above are
invisible, which is why they are the ones written down.

This story adds no conformance rule and defines no store, so nothing is owed to
`crates/happenstance-testkit/tests/`.

## Gate: Discover

Tick each box once the sections above satisfy it. `redkiln advance` reads these
against `.redkiln/templates/gates/discover.md` — where each box's rationale is
written — and will not leave `discover` until every one is ticked. Keep each box on
one line; the parser matches line by line.

- [x] The problem is framed in one paragraph.
- [x] Prior art and constraints are recorded in the signal ledger.
- [x] Open questions are either answered or explicitly deferred.
- [x] The next stage (spec) has a clear starting point.
- [x] The wrong implementation this work rejects is named.
- [x] No conformance rule added here asserts a literal position value.
- [x] Any `[FROZEN]` clause this touches is changed by a new ADR, written first.
- [x] If a rule here seems wrong, it is fixed and the reason given in the same change.
