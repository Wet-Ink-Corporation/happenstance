---
item: HS-S0080
stage: spec
created: 2026-08-12T13:47:18.423Z
updated: 2026-08-12T13:47:18.423Z
template_sig: 87bbf1d0
rendered_sig: 2b49a2b2
---

# Spec — Package-complete the crate and hold the name

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | [`.bklg/from-contract-to-published-library/initiative.md`](../../initiative.md) |
| Initiative decomposition | [`.bklg/from-contract-to-published-library/_decomposition.md`](../../_decomposition.md) |
| Project charter | [`.bklg/from-contract-to-published-library/ladybug-projection-store/project.md`](../project.md) |
| This spec | `.bklg/from-contract-to-published-library/ladybug-projection-store/package-completeness-and-name-claim/spec.md` |
| Key briefs | [`../_decomposition.md`](../_decomposition.md) — *Architecture brief* §3 **M5** (the packaging registry) and §7 (docs.rs is a separate failure); *Testing brief* AC-010 row |
| Story map row | [`../_storymap.md`](../_storymap.md) — milestone `packaging-and-ci-shape`, and *Why the slices are cut here* on why the two stories in it are one surface |
| Signed-off design | [`../_design.md`](../_design.md) — **N/A, deliberately**: this project records no user-facing surface, approved as a no-surface determination on 2026-08-12 |
| Roadmap / phase of record | `RUNBOOK.md:4409-4413` (the claim), `RUNBOOK.md:4432-4438` (phase 11 exit criteria), `RUNBOOK.md:795-829` (the per-phase reservation policy and its command) |

## One-line PR slice

Copy both licences and a README into `crates/happenstance-ladybug/`, drop `publish = false`, add the
crate to `xtask/src/package.rs`'s `PUBLISHABLE`, and claim `happenstance-ladybug` on crates.io behind
a `cargo publish --dry-run`.

## Executive summary

This PR turns `happenstance-ladybug` from a crate Cargo refuses to publish into one the gate proves is
package-complete, and holds its name on the registry before anyone else can take it.

The delta is small and almost entirely non-Rust, but every part of it is load-bearing in a way that
fails silently if it is done by halves. `crates/happenstance-ladybug/` today has no `README.md` and no
licence files — only `Cargo.toml`, `src/` and `tests/` — while its manifest inherits
`license.workspace = true`, i.e. `MIT OR Apache-2.0`. That combination is precisely defect **D11**:
metadata promising two licences, neither of which is in the tarball a consumer unpacks
(`xtask/src/package.rs:1-22`). Removing `publish = false` without registering the crate in
`PUBLISHABLE` fails the gate; registering it without removing `publish = false` fails the gate the
other way, and the two messages are different on purpose (`xtask/src/package.rs:24-42`, `:188-204`).

The name claim is not the same act as publishing this crate. It publishes a standalone `0.0.0`
placeholder that `cargo xtask reserve happenstance-ladybug` generates, for reasons already argued in
`xtask/src/reserve.rs:12-30`; whether the *real* crate ships at `0.2.0` belongs to
`publication-and-positioning` (HS-P0016), not here. What this story delivers is a crate that is
**ready**, and a name that is **held**, so that HS-P0016's publish decision is never blocked on this
crate's readiness (project.md, AC-010).

It also deliberately leaves one thing broken: docs.rs cannot build `lbug` 0.19.1
(`crates/happenstance-ladybug/src/lib.rs:24-32`). That is a real consequence of dropping
`publish = false`, it is named here, and it is handed onward rather than patched over.

## Context pack

Read this section before touching anything. Everything below is a decision already taken elsewhere
that this story must honour; the deeper artifacts sit behind the anchors table.

**1. Package-completeness is a fact about the artifact, never about the manifest.** Cargo packages only
what lives inside the package directory and will not follow a path outside it. It does not warn when
`license = "MIT OR Apache-2.0"` is backed by no licence text, because the metadata is what crates.io
renders — so the omission is invisible until someone unpacks the tarball
(`xtask/src/package.rs:1-22`). Therefore: `LICENSE-MIT` and `LICENSE-APACHE` are **copied** from the
repository root into `crates/happenstance-ladybug/`, not symlinked and not referenced. The same rule is
already stated at the placeholder generator (`xtask/src/reserve.rs:158-160`) and already executed in
the three publishable crates, each of which carries its own physical pair.

**2. The manifest edit and the registry edit are one commit, and both directions of the mismatch
fail.** `PUBLISHABLE` at `xtask/src/package.rs:86` is the workspace's *intention*;
`publishable_members()` derives the *fact* from `cargo metadata`, and `reconcile()` fails when they
disagree — naming which one moved. That pairing is deliberate and argued at `package.rs:24-42`: a
derived-only step would report "a crate is missing its licences" for two unrelated bugs, one of which
(a `publish = false` deleted by accident) nothing else in the gate would ever notice. So deleting
`crates/happenstance-ladybug/Cargo.toml:12` alone fails with the *promoted* message
(`package.rs:188-195`), and adding the name alone fails with the *withdrawn* message
(`package.rs:198-204`). Neither is a valid state to leave on a merged tree.

**3. The name is claimed by publishing a placeholder, not by publishing this crate.**
`cargo xtask reserve happenstance-ladybug` writes a standalone `0.0.0` crate under
`target/reserve/happenstance-ladybug/`, carrying both licences and a README that says plainly it has no
functionality, then prints the dry-run and publish commands — it never publishes anything itself
(`xtask/src/reserve.rs:137-191`, `RUNBOOK.md:826-829`). The two rejected alternatives are on record and
are not to be relitigated: publishing the *real* crates at `0.0.0` cascades into every manifest that
names them, because a registry publish resolves version requirements against crates.io rather than
against the path; publishing them at their real version makes the API semver-binding, and the entire
plan is built on freezing the contract deliberately, against evidence, at phases 4–6
(`xtask/src/reserve.rs:12-30`). A reservation is not a reason to freeze an API.

**4. Claiming late is the justification, not a delay to route around.** crates.io's policy prohibits a
crate that exists only to park a name "without having any genuine functionality, purpose, or
significant development activity"; the mitigation adopted at phase 0 is one name per phase, claimed
when that phase starts — "the point being that by now there is a crate to justify it with"
(`RUNBOOK.md:802-813`, `RUNBOOK.md:4409-4413`). This story sits *after*
`fill-the-bodies-and-ps-34-disposition`, so the justification is stronger than the policy needs: a
crate with no `todo!()` left in it, driving the real driver. Do not pull the claim earlier to unblock
something; the ordering **is** the answer to a policy challenge. It also costs nothing to wait —
crates.io has no prefix reservation, so owning `happenstance` protects none of the rest and the
exposure is identical whenever the name is claimed (`xtask/src/reserve.rs:64-66`,
`RUNBOOK.md:817-823`).

**5. This name's availability has never been checked.** `RUNBOOK.md:798-801` records that
`happenstance`, `happenstance-core`, `happenstance-testkit` and `happenstance-cloudflare` were verified
free on 2026-08-06. `happenstance-ladybug` is **not** in that set. Confirm it against the registry
before the irreversible step, because `cargo publish --dry-run` packages and compiles but does not
upload and does not reserve. If the name is gone, **halt and surface it** — it is written into
`xtask/src/reserve.rs:110-115`, `RUNBOOK.md`'s per-phase list and CLAUDE.md's repository map, so a
silent rename here is a rename of the project's own vocabulary.

**6. Publishing is one-way.** `xtask/src/reserve.rs:181` states it at the point of use: *"This is
irreversible — a version can be yanked, never removed."* The dry run is therefore a rule, not a
courtesy, and everything that will be permanently visible on the registry page — the description, the
README, the licence — is settled *before* it, not after.

**7. Ready is not published.** Removing `publish = false` makes Cargo *willing* to publish this crate.
Whether `happenstance-ladybug` actually ships at `0.2.0` is `publication-and-positioning`'s decision,
and the decomposition's working default is three crates only (project.md, *Out of scope*;
[`../../_decomposition.md`](../../_decomposition.md), *Carried into the briefs*). `PUBLISHABLE`'s own word
for what it holds is "intends" (`package.rs:81-86`), which is exactly the right strength of claim for
this story to make.

**8. The registry page's description is a claim, and today it is false.**
`crates/happenstance-ladybug/Cargo.toml:3` reads *"LadybugDB graph projection store adapter for
happenstance. Not yet implemented."* By the time this story runs, the second sentence is untrue.
`xtask/src/reserve.rs:110-115` already carries the corrected string, and the struct's own docs say why
that string lives there: *"The description is the one the real crate will carry, so that the
placeholder and the eventual release describe the same thing to anyone browsing"*
(`xtask/src/reserve.rs:41-43`). Align the manifest to the reservation, not the other way round — the
placeholder is the thing being published today.

**9. State `readme = "README.md"` rather than relying on discovery.** Cargo would find the file anyway;
the key is what crates.io renders on the package page, "and a silent default is a poor thing to rely on
for the first impression anyone gets of the crate" (`crates/happenstance-core/Cargo.toml:12-15`).
`happenstance-core/` is the layout to copy wholesale.

**10. The persona slice.** This is the **evaluator's** first contact with the crate, one layer below
the initiative's AC-08 ("check the compliance claim instead of trusting it") and AC-10, and it advances
initiative DoD 10 — *"the registry page carries licence, description and README as rendered, checked by
looking at them."* The bar for the README is set by the three that already exist:
`crates/happenstance-testkit/README.md:9-22` leads with a plain **Status** block that names what is
*not* yet true, which is why that crate reads as honest rather than unfinished. A ladybug README that
reads like the generated placeholder blurb would be a regression against the crate's own module docs
(`crates/happenstance-ladybug/src/lib.rs:1-22`), which already explain both what it is and why it
offers projections only.

**11. docs.rs is a known, separate failure and is not fixed here.** `lbug` 0.19.1 does not build on
docs.rs — *"the same cost seen from outside"* (`crates/happenstance-ladybug/src/lib.rs:24-32`).
Dropping `publish = false` turns that note into a real consequence. The nightly `--cfg docsrs` gate step
names crates one at a time precisely so skeletons stay out of it (`xtask/src/main.rs:615-621`), and this
story does **not** add `happenstance-ladybug` to it and does **not** add a
`[package.metadata.docs.rs]` table. That question, with the build number beside it, is handed to
HS-P0016 by the sibling story `verdict-ordering-and-publication-handoff`
([`../_storymap.md`](../_storymap.md), AC-011).

**12. The gate step this story arms is cheap, and must stay cheap.** `cargo xtask ci`'s
*"packaged artifacts carry their licences and README"* step shells out to `cargo package -p <name>
--list --allow-dirty --locked` per crate and parses the listing (`xtask/src/main.rs:517-532`,
`xtask/src/package.rs:110-149`). It lists files; it is not a build, and it must not become one — the
sibling story `cold-build-cost-and-ci-shape` is measuring `lbug`'s cold C++ build against this same
gate, and a packaging step that quietly compiles would corrupt that measurement. Note also that the
step is *not* one of the `--workspace` steps, so whatever `--exclude happenstance-ladybug` lever that
sibling story chooses will not remove this check.

## Integration contract

- **Archetype**: `capability` — the evaluator-facing slice of "this crate is a real, installable
  thing", crossing manifest, artifact, gate registry and the public registry.
- **Slice / milestone**: `packaging-and-ci-shape`. Slice-mate: **`cold-build-cost-and-ci-shape`**.
  Both stories are the same surface — *what the workspace pays* — and they are unordered with respect
  to each other ([`../_storymap.md`](../_storymap.md), *Merge order* step 4).
- **Mount point**: **`xtask/src/package.rs`** — specifically the `PUBLISHABLE` constant at
  `xtask/src/package.rs:86`. This is the composition root for this capability in the strict sense the
  architecture brief's **M5** gives it: package-completeness is *registered*, not asserted in prose,
  and `reconcile()` (`package.rs:172-218`) is what makes the registration load-bearing rather than
  decorative. A crate that carries three files nothing reads is not mounted.
- **Wires into** (real, existing contracts consumed — none of these is invented here):
  - `crates/happenstance-ladybug/Cargo.toml` — `publish = false` at `:12`, `description` at `:3`.
  - `xtask/src/package.rs:94` `REQUIRED_FILES` and `:110-149` — the per-crate assertion this crate
    must now satisfy; `:151-157` is the failure message that names the remedy.
  - `xtask/src/main.rs:517-532` — the gate step that invokes `package-check`.
  - `xtask/src/reserve.rs:110-115` — the `Reservable` row for `happenstance-ladybug` (phase 11).
    **Read, not edited**: the row already exists and already carries the correct description.
  - `LICENSE-MIT`, `LICENSE-APACHE` at the repository root — the source of the copies
    (`xtask/src/reserve.rs:161-165` copies from exactly these).
  - `crates/happenstance-core/Cargo.toml:12-15` and `crates/happenstance-core/README.md` — the layout
    and the `readme` key to copy; `crates/happenstance-testkit/README.md:1-22` is the tone to match.
  - `RUNBOOK.md:4409-4413` and `RUNBOOK.md:4432-4438` — phase 11's claim item and the
    `publish = false` exit box, ticked in place.
- **Renders surfaces**: **none.** [`../_design.md`](../_design.md) records N/A for every surface
  section and was signed off on that basis on 2026-08-12; `design.capture` is a declared skip because
  there is no app to screenshot (CLAUDE.md; `.redkiln/config.yaml`, the commented `design:` block).
  The nearest thing to a rendered surface here is the crates.io package page, which is produced by the
  registry from the manifest and README this story writes — reviewed by looking at it, per initiative
  DoD 10, not by a capture harness.
- **Conformance rule(s)**: **none, and that is correct.** This story changes no port, no value type and
  no adapter behaviour, so nothing here is observable to `happenstance-testkit`. Its analogue of a
  conformance rule is the gate step at `xtask/src/main.rs:517-532`, which has a named wrong
  implementation already in this workspace: `cargo package -p happenstance-sqlite --list` exits 0,
  lists seven files, and none of them is one of the three (`xtask/src/package.rs:20-22`).
- **Clause(s)**: none amended, none discharged. No `spec/SPECIFICATION.md` clause is touched, and in
  particular no `[FROZEN]` marker or clause text moves — that is project AC-008, carried by every story
  in this project.
- **Advances DoD scenario**: initiative **DoD 10** — *"The published crate looks finished … the registry
  page carries licence, description and README as rendered, checked by looking at them"*
  ([`../../initiative.md`](../../initiative.md), Definition of Done). It is a precondition for **DoD 9**
  (a stranger installs from the registry) for this crate, and it carries the standing **DoD 13**
  obligation (`cargo xtask ci` green) that every story in this project shares. It discharges project
  **AC-010** in full and closes one of phase 11's four exit boxes (`RUNBOOK.md:4432-4438`).

## PR boundary

```
crates/happenstance-ladybug/Cargo.toml
crates/happenstance-ladybug/README.md
crates/happenstance-ladybug/LICENSE-MIT
crates/happenstance-ladybug/LICENSE-APACHE
xtask/src/package.rs
RUNBOOK.md
.bklg/from-contract-to-published-library/ladybug-projection-store/package-completeness-and-name-claim/**
```

**In this PR**

- Both licence files copied verbatim from the repository root into `crates/happenstance-ladybug/`.
- A `README.md` authored for the crate, at the standard the three publishable crates already set.
- `crates/happenstance-ladybug/Cargo.toml`: `publish = false` deleted; `description` aligned to
  `xtask/src/reserve.rs:110-115`; `readme = "README.md"` stated explicitly with the rationale
  `crates/happenstance-core/Cargo.toml:12-15` records.
- `xtask/src/package.rs:86`: `PUBLISHABLE` gains `"happenstance-ladybug"`, in the **same commit** as the
  manifest change.
- The crates.io claim itself: availability confirmed, `cargo xtask reserve happenstance-ladybug`, the
  printed `cargo publish --dry-run`, then the publish — recorded with date and version in this story's
  ledger and against `RUNBOOK.md`'s phase 11 claim item.
- `RUNBOOK.md` phase 11: the *"Claim `happenstance-ladybug` on crates.io"* work box and the
  *"`publish = false` removed"* exit box ticked in place, with no other phase-11 box touched.

**Explicitly not in this PR**

- **Publishing the real `happenstance-ladybug` crate**, at `0.2.0` or any version — HS-P0016's call
  (project.md, *Out of scope*). What is published today is the `0.0.0` placeholder.
- **`[package.metadata.docs.rs]` for this crate, and adding it to the nightly `--cfg docsrs` step**
  (`xtask/src/main.rs:615-635`) — `lbug` does not build on docs.rs; that is handed to HS-P0016 by
  `verdict-ordering-and-publication-handoff`.
- **The build-cost measurement and any `--exclude` / dedicated-job CI shape** — the slice-mate
  `cold-build-cost-and-ci-shape` owns `xtask/src/main.rs` and `.github/workflows/ci.yml`
  ([`../_storymap.md`](../_storymap.md), AC-009).
- **Editing `xtask/src/reserve.rs`.** Its `Reservable` row for this crate already exists and is already
  correct. Bumping the workspace-wide `PUBLISHABLE`/`RESERVABLE` machinery, or the placeholder's
  generated `edition`/`rust-version` literals (`reserve.rs:216-239`), is not this story's — see the
  disposition requirement in the behaviour table below.
- **Any `crates/happenstance-ladybug/src/**` change.** The bodies, the driver swap and `stand_in`'s
  deletion are the `real-adapter` slice's, already merged when this story starts.
- **Any `spec/SPECIFICATION.md`, `.kb/` or `references/evaluation/` change.**

**Merge DoD.** `cargo xtask ci` is green on this tree with `happenstance-ladybug` in `PUBLISHABLE`; the
name is held on crates.io; the crate directory carries all three of `xtask/src/package.rs:94`'s
`REQUIRED_FILES`; nothing outside the boundary block above changed.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| Both licences ship **inside** the artifact | `LICENSE-MIT` and `LICENSE-APACHE` copied byte-for-byte from the repository root into `crates/happenstance-ladybug/`. Copies, never links or paths — Cargo packages only what is inside the package directory, and the failure is silent because manifest metadata is what crates.io renders. This is defect D11 recurring on a fourth crate. | `xtask/src/package.rs:1-22`, `:94`, `:151-157`; `xtask/src/reserve.rs:158-165`; `LICENSE-MIT`, `LICENSE-APACHE` |
| A README that describes a crate that exists | Authored, not generated. States what the crate is, that it is a **projection store only** and why (an event log needs a monotonic append with a conditional write; forcing the event store port onto a graph engine "would produce something that satisfies the trait and not the specification"), the LadybugDB/Cypher context, and a plain status block naming what is not yet true — including that `lbug` builds C++ through `cxx`/`cmake` and that docs.rs cannot build it. | `crates/happenstance-ladybug/src/lib.rs:1-32`; `crates/happenstance-testkit/README.md:1-22`; `crates/happenstance-core/README.md` |
| `readme = "README.md"` stated in the manifest | Explicit rather than auto-discovered, for the reason already recorded one crate over: the key is what crates.io renders, and a silent default is a poor thing to rely on for a first impression. | `crates/happenstance-core/Cargo.toml:12-15` |
| `description` stops claiming the crate is unimplemented | `Cargo.toml:3`'s trailing *"Not yet implemented."* is deleted so the manifest matches the reservation's string exactly — the reservation carries the description *"the real crate will carry, so that the placeholder and the eventual release describe the same thing to anyone browsing."* | `crates/happenstance-ladybug/Cargo.toml:3`; `xtask/src/reserve.rs:41-43`, `:110-115` |
| `publish = false` removed and `PUBLISHABLE` extended, atomically | One commit. `reconcile()` compares the hand-written intention against the set derived from `cargo metadata`, and fails in either direction with a *different* message, because they are two different bugs with two different remedies. | `crates/happenstance-ladybug/Cargo.toml:12`; `xtask/src/package.rs:86`, `:172-218` |
| The gate now asserts this crate's artifact, by name, on every run | `cargo package -p happenstance-ladybug --list --allow-dirty --locked` is parsed and all three `REQUIRED_FILES` are asserted present. `--allow-dirty` because the gate must run on an uncommitted tree — the only tree anyone runs it against before pushing. | `xtask/src/main.rs:517-532`; `xtask/src/package.rs:69-74`, `:103-161` |
| The packaging step stays a listing, not a build | `cargo package --list` enumerates files; it must not trigger `lbug`'s C++ compile. The implementer records the observed wall-clock delta this step adds to `cargo xtask ci`, because the slice-mate is measuring the cold `lbug` build against the same gate and a packaging step that quietly compiled would corrupt that number. | `xtask/src/package.rs:110-124`; [`../_storymap.md`](../_storymap.md), AC-009 |
| Name availability confirmed **before** the irreversible step | `happenstance-ladybug` is not in the set verified free on 2026-08-06. Confirm against the registry; `--dry-run` packages and compiles but does not upload and does not reserve. A taken name halts the story and is surfaced — never silently renamed. | `RUNBOOK.md:798-801`; `xtask/src/reserve.rs:110-115`, `:174-188`; CLAUDE.md, repository map |
| The claim is a `0.0.0` placeholder, generated not hand-rolled | `cargo xtask reserve happenstance-ladybug` writes `target/reserve/happenstance-ladybug/` with both licences, a README that says plainly it has no functionality, and a `src/lib.rs` exposing only `STATUS`. The command exists because a procedure run once every few weeks from memory drifts, and the crate carrying the project's first impression then ships without licences permanently. | `xtask/src/reserve.rs:1-10`, `:137-191`, `:242-287`; `RUNBOOK.md:826-829` |
| Dry run precedes publish, and the generated metadata is dispositioned first | The printed `cargo publish --manifest-path … --dry-run` runs and is green before the publish. Before that, the implementer **reads** the generated manifest and records a disposition for its literal `edition = "2021"` / `rust-version = "1.85"` (`reserve.rs:216-239`) against the workspace's current `edition = "2024"` / `rust-version = "1.97.1"` — either accepted as true of a standalone, dependency-free `0.0.0` crate, or raised as a finding routed to the `support` initiative. It is not fixed inline: `reserve.rs` is outside this story's boundary, and a version can be yanked but never removed. | `xtask/src/reserve.rs:181`, `:214-239`; `Cargo.toml:5-8`; `.kb/decisions/0029-msrv-raised-to-1-97-1.md`; `.redkiln/config.yaml` (`support_initiative`) |
| The claim is recorded where it can be found again | Date, registry version and the command run, in this story's ledger, with `RUNBOOK.md` phase 11's claim work-box and `publish = false` exit-box ticked in place. A reservation that is not traceable to the work justifying it is the thing crates.io's policy objects to. | `RUNBOOK.md:4409-4413`, `:4432-4438`; `xtask/src/reserve.rs:51-53`, `:56-66` |
| Nothing frozen moves, and no clause is touched | No `spec/SPECIFICATION.md` edit is in scope; `cargo xtask spec-trace` runs inside `cargo xtask ci` and must stay green. | project.md AC-008; `xtask/src/main.rs` (`specification traceability` step) |

## Data and migrations

**N/A for in-repo data.** This story adds no schema, no stored format, no serialized type and no
runtime state; the four files it writes are two licence texts, a README and a manifest edit, plus one
constant in `xtask`. Nothing reads a database and nothing is versioned on disk.

**One irreversible external state transition, and it has no rollback.** Publishing the `0.0.0`
placeholder to crates.io is a one-way migration of a global namespace: *"This is irreversible — a
version can be yanked, never removed"* (`xtask/src/reserve.rs:181`). The mitigations are ordering, not
reversal, and all three are already stated above — confirm availability first, settle every
permanently-rendered field (description, README, licence) before the run, and execute the printed
`--dry-run` green before the real publish. The only post-hoc remedy available is `cargo yank`, which
withdraws a version from resolution while leaving it published; treat it as damage control, not as an
undo.

**One in-repo state change that behaves like a migration.** After this PR every subsequent
`cargo xtask ci` run — for every contributor, on every commit — invokes
`cargo package -p happenstance-ladybug --list` where it previously did not
(`xtask/src/main.rs:517-532`). That is a permanent widening of the shared gate's per-run cost and is
the reason the behaviour table requires the delta to be observed and recorded rather than assumed.

## Acceptance criteria

The persona throughout is **the evaluator**, "deciding from a bounded look at public evidence whether
'DCB-compliant', 'storage-agnostic' and 'edge-capable' are real claims before adopting … often one
look at a registry page in which to decide" ([`../../initiative.md`](../../initiative.md), *Who this
is for*), on the journey *Decide in one sitting*
([`../../_discovery/distillation/personas-and-journeys.md`](../../_discovery/distillation/personas-and-journeys.md)).
The second actor is **the adapter author** on *Learn when you are finished* — the contributor who runs
`cargo xtask ci` and needs it to say, out loud, whether this crate is shippable. Every criterion below
is one of those two crossing the full stack: manifest → packaged artifact → gate registry → registry
page.

| id | criterion | verification |
| --- | --- | --- |
| **AC-001** | **GIVEN** the evaluator has downloaded the crate and unpacked its `.crate` tarball to check that `MIT OR Apache-2.0` is a real offer rather than a metadata string, **WHEN** they list the archive, **THEN** `LICENSE-MIT` and `LICENSE-APACHE` are both inside it, byte-identical to the repository-root texts — a copy, never a link or a path out of the package directory, because Cargo will not follow one and reports no error when it cannot (`xtask/src/package.rs:1-22`, `:151-157`). | `cargo package -p happenstance-ladybug --list --allow-dirty --locked` lists both names; the same assertion runs inside the gate via `cargo run --locked --quiet -p xtask -- package-check` (`xtask/src/main.rs:517-532`; `xtask/src/package.rs:94`, `:103-161`). Byte-equality against `LICENSE-MIT` / `LICENSE-APACHE` at the repository root checked with a hash comparison recorded in the ledger. |
| **AC-002** | **GIVEN** the evaluator lands on the crates.io page for `happenstance-ladybug` with one look in which to decide, **WHEN** the page renders its front matter, **THEN** a `README.md` written for *this* crate is there — naming that it is a **projection store only** and why an event log cannot be forced onto a graph engine, naming LadybugDB/Cypher, and carrying a plain status block that states what is **not** yet true (including that `lbug` compiles C++ through `cxx`/`cmake`, and that docs.rs cannot build it) — at the standard `crates/happenstance-testkit/README.md:1-22` already sets, not the generated placeholder blurb. | `cargo package -p happenstance-ladybug --list` includes `README.md` (gate-enforced, as AC-001). Content is a **review-tier** check against `crates/happenstance-ladybug/src/lib.rs:1-32` (every claim the module docs make is present and none contradicted) and `crates/happenstance-testkit/README.md:9-22` (a Status block naming what is not yet true). Recorded in the ledger as the reviewer's named sign-off. |
| **AC-003** | **GIVEN** the evaluator reads the one-line description and the rendered front page — the two fields crates.io shows before anything else — **WHEN** they compare them to what the crate now does, **THEN** neither lies: `description` no longer ends *"Not yet implemented."* and matches `xtask/src/reserve.rs:110-115` **exactly**, so the placeholder published today and the eventual release describe the same thing to anyone browsing (`xtask/src/reserve.rs:41-43`), and `readme = "README.md"` is stated in the manifest rather than left to Cargo's silent default (`crates/happenstance-core/Cargo.toml:12-15`). | String-equality check of `crates/happenstance-ladybug/Cargo.toml`'s `description` against the `Reservable` row's `description` at `xtask/src/reserve.rs:110-115`, run by eye and recorded; `readme = "README.md"` present in `crates/happenstance-ladybug/Cargo.toml`; `cargo package -p happenstance-ladybug --list` confirms the file the key points at is in the artifact. |
| **AC-004** | **GIVEN** the adapter author intends this crate to become shippable, **WHEN** they remove `publish = false` from `crates/happenstance-ladybug/Cargo.toml:12`, **THEN** the workspace's stated intention moves with it in the **same commit** — `"happenstance-ladybug"` joins `PUBLISHABLE` at `xtask/src/package.rs:86` — and neither half alone is a mergeable state: the promoted direction fails with `package.rs:188-195`'s message and the withdrawn direction with `:198-204`'s, because they are two different bugs with two different remedies (`package.rs:24-42`). | `cargo run -p xtask -- package-check` prints `publishable set agrees with the manifests: …, happenstance-ladybug` (`xtask/src/package.rs:216-219`). The named wrong implementations are pinned by unit tests added to `xtask/src/package.rs`'s existing `mod tests` (`xtask/src/package.rs:409-456`): one asserting `reconcile` errors with the *promoted* wording when the derived set holds a name `PUBLISHABLE` does not, one asserting the *withdrawn* wording for the inverse. Both run under `cargo test --workspace --all-features`. |
| **AC-005** | **GIVEN** the adapter author runs `cargo xtask ci` before pushing — on an uncommitted tree, the only tree anyone runs it against (`xtask/src/package.rs:69-74`) — **WHEN** the packaging step runs, **THEN** it names `happenstance-ladybug` where it previously did not and asserts all three `REQUIRED_FILES`, **AND** it stays a *file listing*: the observed wall-clock delta this step adds to the gate is measured and recorded, because `cold-build-cost-and-ci-shape` is measuring `lbug`'s cold C++ build against this same gate and a packaging step that quietly compiled would corrupt that number. | `cargo xtask ci` green on this tree, with the step's stdout line `happenstance-ladybug: N files packaged, including LICENSE-MIT, LICENSE-APACHE, README.md` captured (`xtask/src/package.rs:138-144`). Delta measured as two timed runs of `cargo run -p xtask -- package-check` (before/after the `PUBLISHABLE` edit, warm target dir), the seconds recorded in the ledger evidence and handed to the slice-mate. |
| **AC-006** | **GIVEN** the project's vocabulary — `RUNBOOK.md`, `xtask/src/reserve.rs:110-115` and CLAUDE.md's repository map — already says `happenstance-ladybug`, and **GIVEN** that name is **not** among the four verified free on 2026-08-06 (`RUNBOOK.md:798-801`), **WHEN** the implementer reaches the irreversible step, **THEN** availability has already been confirmed against the registry directly — `cargo publish --dry-run` packages and compiles but neither uploads nor reserves — and a name found taken **halts the story and is surfaced**, never silently renamed. | A dated registry availability check recorded in the ledger before any publish (evidence: the date, the method, and the observed result). The halt path is a written escalation to this project's owner and `RUNBOOK.md`'s phase 11 claim item; it is not a code change. Reviewed, not scripted — the testing brief places the name claim outside every test tier ([`../_decomposition.md`](../_decomposition.md), *Testing brief*, AC-010 row). |
| **AC-007** | **GIVEN** the evaluator will meet whatever is on the registry permanently — *"a version can be yanked, never removed"* (`xtask/src/reserve.rs:181`) — **WHEN** the name is claimed, **THEN** it is claimed by the **generated `0.0.0` placeholder** that `cargo xtask reserve happenstance-ladybug` writes (both licences, a README that says plainly it has no functionality, `src/lib.rs` exposing only `STATUS`), not by publishing the real crate at any version, **AND** the printed `cargo publish --manifest-path … --dry-run` has run green first, **AND** the generated manifest's literal `edition = "2021"` / `rust-version = "1.85"` (`xtask/src/reserve.rs:216-239`) has a recorded disposition against the workspace's `edition = "2024"` / MSRV 1.97.1 — accepted as true of a standalone, dependency-free `0.0.0` crate, or raised as a finding routed to the `support` initiative. Not fixed inline: `xtask/src/reserve.rs` is outside this story's boundary. | The `cargo xtask reserve happenstance-ladybug` output and the green `--dry-run` transcript captured into the ledger evidence, in that order, with the publish transcript third. The disposition is a written paragraph in the ledger citing `xtask/src/reserve.rs:214-239` and `.kb/decisions/0029-msrv-raised-to-1-97-1.md`; if raised, it names the `support` initiative row it was routed to (`.redkiln/config.yaml:5`). |
| **AC-008** | **GIVEN** crates.io's policy objects to a name held *"without having any genuine functionality, purpose, or significant development activity"* (`RUNBOOK.md:802-813`), and **GIVEN** this story deliberately sits after `fill-the-bodies-and-ps-34-disposition` so the justification is a crate with no `todo!()` left in it, **WHEN** the claim is made, **THEN** it is traceable to the work that justifies it: date, registry version and the exact command are recorded in this story's ledger, and `RUNBOOK.md` phase 11's *"Claim `happenstance-ladybug` on crates.io"* work box and its *"`publish = false` removed"* exit box are ticked **in place**, with no other phase-11 box touched. | `git diff` on `RUNBOOK.md` shows exactly two `- [ ]` → `- [x]` transitions, at `RUNBOOK.md:4409-4413` and within `RUNBOOK.md:4432-4438`, and no other line changed. The ledger row for this AC carries the claim date, the published version (`0.0.0`) and the command string. Reviewed at merge; the ordering claim itself is checked by `git log` showing this story's commits after `fill-the-bodies-and-ps-34-disposition`'s. |

**Coverage of the traced project AC.** Project **AC-010** — *"`happenstance-ladybug` is claimed on
crates.io, and the crate carries both licence files and a README so that `publication-and-positioning`'s
publish decision is never blocked on this crate's readiness"* ([`../project.md`](../project.md), AC-010)
— is discharged in full and by no other story ([`../_storymap.md`](../_storymap.md), *Coverage*): the
licence half by AC-001, the README half by AC-002 and AC-003, "package-complete" as the gate understands
it by AC-004 and AC-005, and "claimed" by AC-006, AC-007 and AC-008.

## Interaction quality

This story **renders no application surface**. [`../_design.md`](../_design.md) records N/A for every
surface section and was signed off on that basis on 2026-08-12; `design.capture` is a declared skip
because there is no app to screenshot (CLAUDE.md; `.redkiln/config.yaml`). So the **state family** —
in-place vs context-jump, non-occlusion, preserved focus/scroll/selection, keyboard reachability — has
no referent here and is recorded as N/A rather than invented. There is exactly one state-family
invariant with a real analogue, and it is **reversibility**, which is *absent* and cannot be added:
publishing is one-way. Its only available substitute is rehearsal, and that is why AC-007 makes the
green `--dry-run` a precondition rather than a courtesy.

The **composition family** does have a referent, and it is not a screen: it is the two artifacts a human
actually looks at — the **crates.io package page** (rendered by the registry from the manifest and
README this story writes) and the **unpacked `.crate` tarball**. Initiative **DoD 10** is explicit that
this is *"checked by looking at them"* ([`../../initiative.md`](../../initiative.md)). Applied here, and
each already carried as an AC row above rather than as a bullet in this section:

| Invariant (composition family) | Carried by | How it is verified |
| --- | --- | --- |
| **Presentation exists at all** — the page is not bare metadata over a missing front page | **AC-002** | `cargo package --list` proves `README.md` is in the artifact; the review-tier read proves it is authored for this crate rather than generated blurb |
| **Composition / placement** — what the evaluator meets first is the description and the README, in that order | **AC-003** | `description` matches the reservation string exactly; `readme = "README.md"` stated rather than defaulted, because the key is what crates.io renders |
| **Transience** — the licence is persistent chrome of the artifact, not an on-demand link | **AC-001** | both texts inside the tarball, copied not referenced; Cargo will not follow a path outside the package directory |
| **Density budget** — three files, exactly: `xtask/src/package.rs:94`'s `REQUIRED_FILES`, no more and no fewer, asserted per crate | **AC-005** | the gate step prints the packaged file count and names all three; a missing one bails with `package.rs:151-157` |
| **Hierarchy** — the status block leads, so what is *not* yet true is read before the claims | **AC-002** | reviewed against `crates/happenstance-testkit/README.md:9-22`, the crate that reads as honest rather than unfinished for exactly this reason |
| **Named anti-pattern: metadata promising what the artifact does not contain** (defect D11) | **AC-001**, **AC-004** | the whole of `xtask/src/package.rs` exists to reject it; the named wrong implementation already in this workspace is `cargo package -p happenstance-sqlite --list`, which exits 0, lists seven files, and none of them is one of the three (`xtask/src/package.rs:20-22`) |
| **Named anti-pattern: a description that describes a different crate than the one published** | **AC-003** | the trailing *"Not yet implemented."* is untrue by the time this story runs; the reservation's string is the one to align to, not the other way round |

An unstyled render satisfies every mechanical assertion here — `cargo package --list` is perfectly happy
with a one-line README that says "TODO". AC-002 and AC-003 are the rows that make that fail, and both are
deliberately review-tier: no command in this repository can tell an honest README from a placeholder one.

## Error conditions

| id | condition | required behaviour |
| --- | --- | --- |
| **EC-001** | `happenstance-ladybug` is already taken on crates.io (it was never in the set verified free on 2026-08-06, `RUNBOOK.md:798-801`). | **Halt the story and surface it.** Do not rename locally: the name is written into `xtask/src/reserve.rs:110-115`, `RUNBOOK.md`'s per-phase list and CLAUDE.md's repository map, so a silent rename is a rename of the project's own vocabulary. AC-001 – AC-005 may still merge; AC-006 – AC-008 block pending a decision made above this story. |
| **EC-002** | The printed `cargo publish --dry-run` fails — packaging or compiling the generated placeholder. | Do not publish. Fix, regenerate with `cargo xtask reserve happenstance-ladybug` (it removes the previous output first, `reserve.rs:154-157`), and re-run the dry run to green. A publish attempted past a red dry run is the failure `reserve.rs:174-188` orders the two commands to prevent. |
| **EC-003** | `publish = false` is removed but `PUBLISHABLE` is not extended. | `cargo xtask ci` fails at `package-check` with the **promoted** message (`xtask/src/package.rs:188-195`), which names the remedy and the alternative reading — that a `publish = false` went missing by accident. Not to be silenced by re-adding `publish = false` unless that is the actual intent. |
| **EC-004** | `PUBLISHABLE` is extended but `publish = false` remains. | Fails with the **withdrawn** message (`xtask/src/package.rs:198-204`). The two messages are deliberately different; a merged tree in either state is invalid. |
| **EC-005** | `cargo package -p happenstance-ladybug --list` exits 0 but one of the three `REQUIRED_FILES` is absent. | The step bails with `xtask/src/package.rs:151-157` — *"Copy the file into the crate directory — Cargo will not follow a path outside it."* Remedy is a copy, never a manifest `include`/path edit pointing outside the package directory. |
| **EC-006** | `cargo package -p happenstance-ladybug --list` fails outright (e.g. the `lbug` dependency will not resolve under `--locked`). | Distinct from EC-005 and bails with its own message (`xtask/src/package.rs:117-123`). This is a dependency-graph fault inherited from `real-lbug-driver-swap`, not a packaging fault; report it against that story's merge rather than working around it here. |
| **EC-007** | The generated placeholder's `edition = "2021"` / `rust-version = "1.85"` (`xtask/src/reserve.rs:216-239`) disagrees with the workspace's `edition = "2024"` / MSRV 1.97.1. | **Disposition, do not patch.** Either accept it in writing as true of a standalone, dependency-free `0.0.0` crate, or raise it as a finding routed to the `support` initiative (`.redkiln/config.yaml:5`). Editing `xtask/src/reserve.rs` is outside this story's PR boundary, and the placeholder cannot be corrected after publication — a version can be yanked, never removed. |
| **EC-008** | `cargo publish` fails on credentials (no token in `$CARGO_HOME/credentials.toml`). | Run `cargo login` out of band. **Never commit a token**, and never add one to the repository-local path — `reserve.rs:189-191` states this at the point of use and `.gitignore` already covers it. |
| **EC-009** | The publish succeeds but a permanently-rendered field (description, README, licence) is wrong. | There is no undo. `cargo yank` withdraws the version from resolution while leaving it published; treat it as damage control. Record what shipped, and hand the correction to `publication-and-positioning` (HS-P0016) with the real release. This is precisely why AC-002, AC-003 and AC-007 order the review *before* the run. |

## Non-functional

| id | requirement | why, and where it is anchored |
| --- | --- | --- |
| **NF-001** | The packaging step remains a **listing**, not a build. The wall-clock delta it adds to `cargo xtask ci` is measured and recorded (AC-005), not assumed. | `cargo package --list` enumerates files (`xtask/src/package.rs:110-124`). The slice-mate `cold-build-cost-and-ci-shape` is measuring `lbug`'s cold C++ build against this same gate ([`../_storymap.md`](../_storymap.md), AC-009); a packaging step that quietly compiled would corrupt that measurement, and the cost is paid by every contributor on every commit thereafter. |
| **NF-002** | The manifest edit and the `PUBLISHABLE` edit land in **one commit**. | `reconcile()` is a cross-file invariant (`xtask/src/package.rs:172-218`); split across commits, the intermediate commit is a red tree that `cargo xtask affected --base main` will surface at the worst moment. |
| **NF-003** | The two licence files are **byte-identical** to the repository-root originals. | `xtask/src/reserve.rs:161-165` copies from exactly those paths for the placeholder; the three publishable crates each carry a physical pair. A reworded licence is a different licence. |
| **NF-004** | Nothing `[FROZEN]` moves and no `spec/SPECIFICATION.md` line changes; `cargo xtask spec-trace` stays green. | Project **AC-008**, carried by every story in this project ([`../project.md`](../project.md), *Definition of done* 7). `spec-trace` runs inside `cargo xtask ci`. |
| **NF-005** | The bar is the **whole gate**, `cargo xtask ci`, never `--fast`. | This project is the one place in the portfolio where the wasm32, `cargo-hack`, package-completeness and `spec-trace` steps meet a new dependency graph ([`../_decomposition.md`](../_decomposition.md), *Merge-gate commands*; [`../_storymap.md`](../_storymap.md), *Merge order*). |
| **NF-006** | No credential, token or `credentials.toml` path enters the tree. | `xtask/src/reserve.rs:189-191`. |
| **NF-007** | The claim is a **one-time, human-run** act. No CI job, hook or script publishes to crates.io as a side effect of this story. | `cargo xtask reserve` deliberately prints the commands and publishes nothing itself (`xtask/src/reserve.rs:174-191`); the testing brief places the claim outside every test tier ([`../_decomposition.md`](../_decomposition.md), AC-010 row). |

## Implementation notes (non-prescriptive)

Shape, not instructions. The implementer owns the order within each step.

- **`crates/happenstance-core/` is the layout to copy wholesale** — its `Cargo.toml:12-15` for the
  `readme` key and the comment recording why it is explicit, its physical licence pair, its README as a
  structural model. `crates/happenstance-testkit/README.md:1-22` is the *tone* to match, not the
  structure: it leads with a Status block that names what is not yet true.
- **Write the README from the module docs, not from the placeholder generator.**
  `crates/happenstance-ladybug/src/lib.rs:1-32` already explains what the crate is, why it offers
  projections only, and what `lbug` costs (C++ through `cxx`/`cmake`, and docs.rs cannot build it). A
  README that says less than the module docs is a regression the gate cannot see.
- **Do the two-line edit and the constant together, then run `package-check` alone before the whole
  gate.** `cargo run -p xtask -- package-check` is seconds; `cargo xtask ci` on this tree is not, now
  that `lbug` is in the graph. It is also the exact command the gate runs (`xtask/src/main.rs:517-532`),
  so a green isolated run is not an approximation of the gate — it is the gate's step.
- **The two `reconcile` unit tests are cheap and belong beside the three already there**
  (`xtask/src/package.rs:409-456`). They are the only mechanism in this story that pins the *messages*
  rather than the outcome, and the messages are the whole reason the pair of sets exists
  (`package.rs:24-42`).
- **Sequence the irreversible step last, and in this order**: confirm availability → `cargo xtask
  reserve happenstance-ladybug` → read the generated manifest and write the EC-007 disposition → run
  the printed `--dry-run` to green → publish → record date, version and command. Nothing in that
  sequence is reorderable, and the last item is not optional (AC-008).
- **`xtask/src/reserve.rs` is read, never edited.** Its `Reservable` row for this crate already exists
  and already carries the corrected description. If something about it is wrong, that is EC-007's
  disposition path, not an inline fix.
- **Leave docs.rs broken, visibly.** No `[package.metadata.docs.rs]` table, and do not add this crate to
  the nightly `--cfg docsrs` step (`xtask/src/main.rs:615-635`), which names crates one at a time
  precisely so skeletons stay out of it. Say so in the README's status block; the question goes to
  HS-P0016 via `verdict-ordering-and-publication-handoff`.

## Tests and CI (merge gate)

Tiers as the project's testing brief defines them ([`../_decomposition.md`](../_decomposition.md),
*Testing brief* — AC-010 is a **Static** row, with the name claim explicitly outside every test tier).

| tier | command / path | proves |
| --- | --- | --- |
| **Static (isolated)** | `cargo package -p happenstance-ladybug --list --allow-dirty --locked` | `LICENSE-MIT`, `LICENSE-APACHE`, `README.md` are in the artifact — the same assertion the gate makes, checked alone so a failure is legible (AC-001, AC-002, AC-003) |
| **Static (gate step)** | `cargo run --locked --quiet -p xtask -- package-check` → `xtask/src/package.rs:103-161`, invoked at `xtask/src/main.rs:517-532` | `reconcile()` agrees, and the crate's artifact carries all three `REQUIRED_FILES`, by name, on every run thereafter (AC-004, AC-005) |
| **Unit** | `cargo test --workspace --all-features` → new cases in `xtask/src/package.rs`'s `mod tests` (`xtask/src/package.rs:409-456`) | `reconcile` fails with the **promoted** wording when the derived set holds a name `PUBLISHABLE` does not, and with the **withdrawn** wording for the inverse — the two named wrong implementations of AC-004 |
| **Static** | `cargo fmt --check`; `cargo clippy --workspace --all-targets --all-features -D warnings` | the `xtask/src/package.rs` edit and its tests are house-style clean; nothing regressed in the crate this story makes publishable |
| **Static** | `cargo xtask spec-trace` | no `spec/SPECIFICATION.md` citation or `[FROZEN]` marker moved (NF-004, project AC-008) |
| **Integration (whole gate)** | `cargo xtask ci` — **not** `--fast` | the merge DoD: fmt, clippy, tests, four wasm32 steps, docs, `spec-trace`, the `--no-default-features` doc build, `package-check` with this crate now in it, plus `cargo-hack` and `cargo-deny` which both resolve on this machine (CLAUDE.md, *Commands*; [`../_decomposition.md`](../_decomposition.md), *Merge-gate commands*) — NF-005 |
| **Measurement** | two timed runs of `cargo run -p xtask -- package-check`, warm target dir, before and after the `PUBLISHABLE` edit | the packaging step stayed a listing; the recorded delta is handed to `cold-build-cost-and-ci-shape` (AC-005, NF-001) |
| **Process (manual, one-time)** | registry availability check → `cargo xtask reserve happenstance-ladybug` → `cargo publish --manifest-path target/reserve/happenstance-ladybug/Cargo.toml --dry-run` → `cargo publish --manifest-path target/reserve/happenstance-ladybug/Cargo.toml` | the name is held, behind a green dry run, by a `0.0.0` placeholder (AC-006, AC-007) — outside every test tier by the testing brief's own instruction, so the evidence is the captured transcripts |
| **Review** | the README read against `crates/happenstance-ladybug/src/lib.rs:1-32` and `crates/happenstance-testkit/README.md:9-22`; `git diff -- RUNBOOK.md`; `git diff -- crates/happenstance-ladybug/Cargo.toml` | AC-002's honesty bar, AC-003's exact description match, AC-008's two ticked boxes and nothing else in phase 11 touched |
| **Not run here** | `redkiln validate --kb`, `redkiln doctor` | no `.kb/` atom is authored or amended by this story; listed so the omission is deliberate rather than forgotten |

## Risks and coupling (PR-scoped)

- **The claim is irreversible and this PR contains it.** Everything permanently visible — description,
  README, licence — must be settled before the publish, and there is no revert commit that undoes it
  (`xtask/src/reserve.rs:181`). *Mitigation:* the AC-006 → AC-007 ordering, the green dry run, and
  EC-009's honest statement that `cargo yank` is damage control rather than an undo.
- **The name may be gone.** It was never checked (`RUNBOOK.md:798-801`). *Mitigation:* EC-001 halts and
  surfaces; the packaging half of the story (AC-001 – AC-005) is independently mergeable, so the story
  degrades rather than blocks the slice.
- **Coupling to the slice-mate's measurement.** `cold-build-cost-and-ci-shape` is timing the gate while
  this story widens it. The two are unordered ([`../_storymap.md`](../_storymap.md), *Merge order* step 4),
  so whichever merges second measures a gate the first has changed. *Mitigation:* NF-001's recorded
  delta, handed over explicitly rather than left to be inferred from a total. Note also that
  `package-check` is **not** a `--workspace` step, so whatever `--exclude happenstance-ladybug` lever
  that story picks will not remove this check — a fact worth stating to it, not assuming it knows.
- **Coupling to `real-lbug-driver-swap` through the dependency graph.** `cargo package --list` runs with
  `--locked`; if `lbug` will not resolve, this story's gate step fails for a reason that is not this
  story's (EC-006). *Mitigation:* the `depends_on` edge already orders them, and EC-006 names the
  correct owner so the failure is not absorbed here.
- **`Cargo.lock` churn.** Dropping `publish = false` changes no dependency, but a `cargo package` run
  can touch the lockfile if the tree is not already resolved. *Mitigation:* run `cargo xtask ci` on a
  resolved tree and keep any lockfile change out of the diff unless it is genuinely required — the PR
  boundary block does not list `Cargo.lock`.
- **The README is the one deliverable no command can grade.** A one-line placeholder passes every
  mechanical check in this story. *Mitigation:* AC-002 is explicitly review-tier with a named
  comparison target, and the ledger records the reviewer.
- **Scope creep toward HS-P0016.** The temptation while holding a publishable manifest is to fix
  docs.rs, add `[package.metadata.docs.rs]`, or bump the version. All three are out of boundary and
  belong to `publication-and-positioning` via `verdict-ordering-and-publication-handoff`
  ([`../_storymap.md`](../_storymap.md), AC-011). *Mitigation:* the PR boundary's *Explicitly not in
  this PR* list, checked at review.

## Dependencies

**Blocks on**

- **`fill-the-bodies-and-ps-34-disposition`** — the only edge, and it is the justification, not a
  technicality. Claiming the name after the bodies are filled means the crate offered to crates.io as
  justification has no `todo!()` left in it and drives the real driver, which is precisely what
  `RUNBOOK.md:802-813`'s policy mitigation asks for. Do **not** pull this story earlier to unblock
  something; the ordering *is* the answer to a policy challenge.

**Unlocks**

- **`freeze-verdict-document`** — names this story among its three prerequisites
  ([`../_storymap.md`](../_storymap.md), *Slices*), because the verdict states the commit and the crate
  it was run against.
- **`verdict-ordering-and-publication-handoff`** — inherits the docs.rs failure and the
  `[package.metadata.docs.rs]` question this story deliberately leaves open, and hands them to HS-P0016
  (AC-011).
- **`publication-and-positioning` (HS-P0016)**, transitively — its publish decision for this crate is no
  longer blocked on this crate's readiness ([`../project.md`](../project.md), AC-010).

**Unordered with respect to**

- **`cold-build-cost-and-ci-shape`** — the slice-mate. Same milestone `packaging-and-ci-shape`, same
  surface (*what the workspace pays*), no edge in either direction; both merge after slice 3
  ([`../_storymap.md`](../_storymap.md), *Merge order* step 4). It owns `xtask/src/main.rs` and
  `.github/workflows/ci.yml`; this story owns `xtask/src/package.rs`. Disjoint files, one shared number.

## Anchors (progressive disclosure)

Linked, not pasted. Open each at the moment named.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `xtask/src/package.rs` | The module docs at `:1-42` argue defect D11 and why `PUBLISHABLE` and the derived set both exist; `:79-101` are the two constants; `:103-161` is the assertion; `:172-218` is `reconcile` and its two messages verbatim; `:409-456` is the existing `mod tests` to extend. | Before the first edit — this is the mount point, and the messages the ACs pin are here. | AC-001, AC-004, AC-005 |
| `xtask/src/reserve.rs` | `:1-30` argues why a placeholder rather than the real crate at `0.0.0` or at its real version — both rejected, not to be relitigated. `:110-115` is this crate's `Reservable` row and the exact description string AC-003 must match. `:137-191` is what the command writes and prints, in order. `:214-239` is the generated manifest whose `edition`/`rust-version` need EC-007's disposition. Read-only. | Immediately before the claim, and again when writing the EC-007 disposition. | AC-003, AC-006, AC-007 |
| `xtask/src/main.rs` | `:517-532` is the gate step this story arms; `:615-635` is the nightly `--cfg docsrs` step this story must **not** add the crate to. | When confirming the gate wiring, and when resisting the docs.rs temptation. | AC-005 |
| `crates/happenstance-ladybug/Cargo.toml` | `:3` is the description that currently ends *"Not yet implemented."*; `:12` is the `publish = false` to delete. Four lines of edit, both load-bearing. | At the manifest edit. | AC-003, AC-004 |
| `crates/happenstance-ladybug/src/lib.rs` | `:1-22` already explains what the crate is and why projections only — the README's source material. `:24-32` is the docs.rs/`lbug` C++ note that becomes a real consequence the moment `publish = false` goes. | While writing the README, before writing a word of it. | AC-002 |
| `crates/happenstance-testkit/README.md` | `:1-22` is the tone bar: a plain Status block naming what is *not* yet true, which is why that crate reads as honest rather than unfinished. | While writing the README's status block. | AC-002 |
| `crates/happenstance-core/Cargo.toml` | `:12-15` carries the `readme = "README.md"` key **and** the recorded reason it is explicit rather than defaulted — a silent default is a poor thing to rely on for a first impression. | At the manifest edit, alongside the description change. | AC-003 |
| `RUNBOOK.md` | `:795-829` is the per-phase reservation policy, the crates.io policy quote, and why prefix reservation buys nothing. `:798-801` is the dated availability check that does **not** include this name. `:4409-4413` is the claim work box; `:4432-4438` is the phase-11 exit list holding *"`publish = false` removed"*. | `:795-829` before the claim; `:4409-4438` when ticking the boxes. | AC-006, AC-008 |
| `.bklg/from-contract-to-published-library/ladybug-projection-store/_decomposition.md` | Architecture brief §3 **M5** (packaging is *registered*, not asserted in prose), §7 (docs.rs is a separate failure), and the *Testing brief*'s AC-010 row, which is the source of this story's tier assignment and of "the name claim is outside every test tier". | Before writing the test plan, and when tempted to add a tier. | AC-004, AC-005 |
| `.bklg/from-contract-to-published-library/ladybug-projection-store/_storymap.md` | *Why the slices are cut here* on why `PUBLISHABLE` and `publish = false` are one surface; *Merge order* step 4 on the unordered slice-mate; AC-011's row for where the docs.rs question goes. | When coordinating with `cold-build-cost-and-ci-shape` or handing anything to HS-P0016. | AC-005, AC-008 |
| `.bklg/from-contract-to-published-library/ladybug-projection-store/project.md` | AC-010's exact wording — the criterion this story discharges in full — and *Definition of done* item 7, the "nothing `[FROZEN]` was amended" obligation NF-004 carries. | At the start, and again at the review that closes the ledger. | AC-001 – AC-008 |
| `.bklg/from-contract-to-published-library/ladybug-projection-store/_design.md` | The signed-off no-surface determination (`## Surfaces`, N/A), which is why the *Interaction quality* section has no state family to satisfy. Do not re-decide it. | If anyone asks for a rendered surface or a capture. | AC-002 |
| `.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md` | The evaluator persona and the *Decide in one sitting* journey the ACs are framed from; the initiative's own note that these are carried here rather than from `.kb/product/`, which is still empty. | When judging whether the README and description actually serve the one-look decision. | AC-002, AC-003 |
| `.kb/decisions/0029-msrv-raised-to-1-97-1.md` | The accepted atom that makes the workspace MSRV 1.97.1, and the reasoning EC-007's disposition is written against. | When dispositioning the generated placeholder's `rust-version = "1.85"`. | AC-007 |
| `crates/happenstance-core/README.md` | The structural model for a crate README that already passes this gate and already renders on a registry page this project is happy with. | While drafting, for structure — the tone comes from testkit. | AC-002 |

## Clarifications resolved during spec

1. **The AC set is exactly the eight the front half named** — AC-001 through AC-008, unchanged. Nothing
   was added or dropped; the ledger matches.
2. **"Package-complete" and "published" are separated, and only the first is claimed.** AC-004 makes
   Cargo *willing* to publish this crate and registers that intention; whether `happenstance-ladybug`
   ever ships at `0.2.0` is HS-P0016's decision, and the decomposition's working default remains three
   crates ([`../project.md`](../project.md), *Out of scope*). `PUBLISHABLE`'s own word is "intends"
   (`xtask/src/package.rs:81-86`), which is the right strength of claim.
3. **The name claim gets ACs even though it is outside every test tier.** The testing brief says so
   explicitly for AC-010 ([`../_decomposition.md`](../_decomposition.md)). Resolved by making AC-006,
   AC-007 and AC-008 *evidence*-verified rather than test-verified: dated transcripts and a `git diff`,
   recorded in the ledger. An AC with no automatable oracle is still an AC; an AC with no evidence is not.
4. **`xtask/src/reserve.rs` is read, not edited — including its generated `edition`/`rust-version`.**
   The mismatch against the workspace's `edition = "2024"` / MSRV 1.97.1 is real, but the file is
   outside the PR boundary and the placeholder cannot be corrected post-publication. Resolved as
   EC-007: a written disposition, and a `support`-initiative finding if it is not accepted. This is
   deliberately not an ADR — ADR authorship belongs to the runbook's own pass.
5. **The `reconcile` failure *messages* are pinned, not just the failure.** The front half establishes
   that the two directions are two different bugs with two different remedies (`package.rs:24-42`).
   Resolved by naming two unit tests in AC-004's verification that assert the wording, added to the
   existing `mod tests` — otherwise "a rule no implementation can fail" applies: after this story
   `reconcile` is green and nothing exercises either branch.
6. **The packaging-step cost is a recorded number, not a shrug.** The slice-mate is measuring the same
   gate and the two stories are unordered. Resolved by folding the measurement into AC-005 and NF-001
   with a stated method (two timed `package-check` runs, warm target dir), so the hand-off is a figure
   rather than an impression.
7. **Interaction quality is answered, not skipped.** The signed-off design records no surface, so the
   state family is genuinely N/A — but the crates.io page and the unpacked tarball *are* composed
   artifacts a human looks at (initiative DoD 10). Resolved by mapping the composition family onto
   existing AC rows (AC-001, AC-002, AC-003, AC-005) rather than inventing prose invariants that
   `redkiln verify` would never extract.
8. **docs.rs stays broken on purpose, and is said out loud.** Not fixed, not hidden: named in the
   README's status block (AC-002), kept out of the nightly `--cfg docsrs` step, and handed to HS-P0016
   through `verdict-ordering-and-publication-handoff`.
