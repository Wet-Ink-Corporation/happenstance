---
item: HS-S0046
stage: spec
created: 2026-08-12T13:46:44.722Z
updated: 2026-08-12T13:46:44.722Z
template_sig: 87bbf1d0
rendered_sig: 9ff7fa0d
---

# Spec — The name reserved and the manifest publishable, with publishing left to someone else

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | [`.bklg/from-contract-to-published-library/initiative.md`](../../initiative.md) — BR-09 (registry-facing completeness verified, not assumed), BR-05 (`0.2.0-alpha.1` then `0.2.0`), DoD 10 |
| Initiative decomposition | [`.bklg/from-contract-to-published-library/_decomposition.md`](../../_decomposition.md) — the publishable-set fork: three crates only, and the adapters' `publish = false` is `publication-and-positioning`'s to change |
| Project | [`.bklg/from-contract-to-published-library/sqlite-durable-store/project.md`](../project.md) — **AC-015**, and *Out of scope* → `publication-and-positioning` (HS-P0016) |
| This spec | `.bklg/from-contract-to-published-library/sqlite-durable-store/crates-io-name-and-packaging-facts/spec.md` |
| Key briefs | [`../_decomposition.md`](../_decomposition.md) — architecture **AC-A05** (`:55-57`), architecture **§1 mount table** (`:89`, the packaging-facts row), architecture **§8** (`:379-397`, what this project must not touch); testing **§2 AC-015 row** (`:641`) |
| Story map row | [`../_storymap.md`](../_storymap.md) — slice `publishable-and-reconciled`, and *Merge order* 5 (`:180-185`): the reservation has no code dependency, the manifest facts do |
| Design | [`../_design.md`](../_design.md) — **no user-facing surface**, signed off 2026-08-12. Nothing here renders a surface |
| Roadmap pointer | `RUNBOOK.md:4190-4192` (claim the name when the phase starts) and `RUNBOOK.md:4230` (the **stale** `publish = false` exit checkbox this story records as superseded) |

## One-line PR slice

Reserve `happenstance-sqlite` on crates.io and give the crate a real description, README and both licence
files so `cargo package --list --allow-dirty` shows them — while `publish = false` and `PUBLISHABLE` stay
untouched.

## Executive summary

Everything before this story made the crate *worth* publishing. This one makes it *packageable* and
leaves the publishing decision where the decomposition put it.

The delta is four files and one external act:

- **`crates/happenstance-sqlite/Cargo.toml`** — the description still ends `"Not yet implemented."`
  (`:3`), which stopped being true one story ago, and there is no `readme` key at all. Both are fixed.
- **`crates/happenstance-sqlite/{README.md,LICENSE-MIT,LICENSE-APACHE}`** — none of the three exists
  today. Cargo packages only what lives inside the package directory, so a `license` field backed by no
  licence text in the artifact is the exact defect D11 was (`xtask/src/package.rs:1-22`).
- **`xtask/src/reserve.rs`** — the generator already carries a `happenstance-sqlite` row claimed by
  phase 8 (`:86-91`). What it does *not* carry is a version claim this initiative still believes: its
  placeholder text promises `0.1.0-alpha.1` (`:258`, `:276`) while BR-05 says the first published
  release is `0.2.0-alpha.1`. A crates.io version can be yanked and never removed, so that sentence is
  reconciled *before* the upload, not after.
- **The reservation itself** — `cargo publish` against the generated `0.0.0` placeholder. It touches no
  tracked file (the generator writes under `target/`), which is precisely why its evidence has to be
  recorded deliberately rather than inferred from a diff.

What this PR deliberately does **not** land is the promotion: `publish = false` stays, `PUBLISHABLE`
stays at three crates, and `cargo xtask package-check` therefore still does not look at this crate. That
is not an oversight to tidy up later — it is AC-015's second half, and §8 of the architecture brief
explains why deleting the line here turns CI red on a contradiction nobody asked for.

## Context pack

Read this section and you can start. Everything deeper is a signposted anchor in the second half of this
spec.

**1. Publishing is not this project's decision, and the runbook disagrees.** `RUNBOOK.md:4230`'s phase-8
exit checkbox says "`publish = false` removed". It is **stale**. Project AC-015 says publishing "stays
someone else's decision" and the initiative decomposition assigns that decision to
`publication-and-positioning` (HS-P0016). The architecture brief settles the conflict in this project's
favour and states the mechanism (`../_decomposition.md:379-395`): with `PUBLISHABLE` in
`xtask/src/package.rs:86` still naming only `happenstance-core`, `happenstance` and
`happenstance-testkit`, deleting `publish = false` here makes `reconcile()` (`package.rs:172-218`) fail
the gate on a *promoted-but-unreconciled* crate — the failure that module's own doc comment was written
to produce. So: **`publish = false` stays, `xtask/src/package.rs` is not edited, and the stale runbook
checkbox is recorded as superseded rather than silently diverged from.**

**2. The consequence that follows, and it is the uncomfortable one.** Because the crate is unpublished,
`cargo xtask package-check` skips it — the gate cannot prove this story's deliverable. AC-015's evidence
is therefore a *recorded* `cargo package -p happenstance-sqlite --list --allow-dirty` run showing all
three files, not a green gate step (`../_decomposition.md:641`). This is the one story in the project
whose primary proof is an artefact in `_ledger.md` rather than a passing test, and the honest response is
to record the command's actual output, not a claim that it was run.

**3. The licences must be *copied*, not referenced.** Cargo does not follow a path outside the package
directory and does not warn when it can't. `LICENSE-MIT` and `LICENSE-APACHE` at the workspace root are
invisible to `cargo package -p happenstance-sqlite`. The three publishable crates each hold their own
byte-identical copies; `xtask/src/reserve.rs:158-165` copies them for the same reason and says so in a
comment. Do the same here.

**4. The README is the crate's front page, and this one has facts to carry.** The story map is explicit
that the manifest facts "cannot land until the README can state the adapter's real durability settings
and enforced ceiling" (`../_storymap.md:184-185`) — which is exactly why this story depends on
`instrument-markers-removed-and-gate-green` rather than being done at the project's start alongside the
reservation. Those facts now exist and are knowable: the journal mode, the `synchronous` setting and the
finite busy timeout settled by `schema-migration-and-identity`, and the three ceilings
(`MAX_EVENT_DATA_LEN`, `MAX_TAGS_PER_EVENT`, `MAX_EVENTS_PER_BATCH`,
`crates/happenstance-testkit/src/contract.rs:253-279`) that `append` now enforces. State them as the
numbers the code actually uses; a README that describes a different store is worse than none.

**5. A README example that does not compile is the first impression the crate makes (D10).** The
convention is already set and is not uniform by accident: `crates/happenstance/src/lib.rs:1-10` compiles
its own README's fences as doctests under `cfg(doctest)`; the sibling READMEs mark fences `rust,ignore`
where the example cannot stand alone (`crates/happenstance-core/README.md:58`,
`crates/happenstance-testkit/README.md:25,47,70`). Pick one deliberately. **Prefer `rust,ignore`** —
adding `include_str!` to `crates/happenstance-sqlite/src/lib.rs` reaches into a file the predecessor
story owns and buys nothing while the crate is unpublished.

**6. The name is reserved through the repository's own generator, and the act is irreversible.**
`cargo xtask reserve happenstance-sqlite` builds a standalone `0.0.0` placeholder under
`target/reserve/` (`xtask/src/reserve.rs:137-191`). Its design is deliberate and should not be
second-guessed: the real crates are not published at `0.0.0` (the dependency graph cascades) and not at
their real version (that would freeze an API this initiative freezes deliberately, `reserve.rs:12-30`).
Run the `--dry-run` the command prints first. Then treat the upload as a **human-authorised act**: it
needs a crates.io token in `$CARGO_HOME/credentials.toml`, and a version can be yanked but never
removed. If credentials are absent, the correct outcome is a recorded handoff — never a claim the name
was taken.

**7. The placeholder must not publish a claim the initiative has already superseded.** The generator's
README and `lib.rs` templates both say "the first functional release will be `0.1.0-alpha.1`"
(`reserve.rs:258`, `:276`). BR-05 says `0.2.0-alpha.1` (`initiative.md:288`), and
`typed-layer-and-alpha-release` owns shipping it. Reconcile the generator's text with BR-05 **before**
the upload, or record why the divergence is deliberate. This is the one edit outside
`crates/happenstance-sqlite/` this story makes, and it is in scope because this story is the first to
use the generator since BR-05 was written — the remaining six reservations inherit whatever is decided
here.

**8. The persona slice.** The evaluator (initiative AC-08/AC-09) and the application author (AC-03) both
meet a crate through its registry page before they meet its code. BR-09's whole premise is that
registry-facing completeness "fails silently" — nothing stops a crate shipping in a state that looks
unfinished on its own landing page. This story makes the artefact complete; DoD 10's *looking at the
rendered page* is `publication-and-positioning`'s, and cannot happen until this exists.

## Integration contract

- **Archetype**: `capability` — the observable outcome is a `.crate` artefact a publisher could act on,
  produced by a real Cargo command over the real manifest.
- **Slice / milestone**: `publishable-and-reconciled`. Slice-mates:
  `instrument-markers-removed-and-gate-green` (this story's `depends_on`, HS-S0045) and
  `spec-and-code-reconciliation`. The three are implemented in one context and land as one integrated
  hand-over surface; this story and `spec-and-code-reconciliation` are parallel behind the first
  (`../_storymap.md:180-185`).
- **Mount point**: **`crates/happenstance-sqlite/Cargo.toml`**. A library has no render tree, so the
  composition root is the file Cargo actually reads to compose the published artefact — `description`,
  `readme = "README.md"`, `license` and `publish` all resolve here, and `cargo package -p
  happenstance-sqlite --list --allow-dirty` is the render path that shows what came out. Named as the
  packaging-facts row of the architecture brief's §1 mount table (`../_decomposition.md:89`).
- **Wires into**:
  - `xtask/src/package.rs:86` (`PUBLISHABLE`), `:94` (`REQUIRED_FILES`), `:172-218` (`reconcile`) —
    consumed as a **constraint**, read and left unchanged. `cargo xtask package-check` must still print
    the same three-crate reconciliation after this story as before it.
  - `xtask/src/reserve.rs:67-128` (`RESERVABLE`) — the `happenstance-sqlite` row at `:86-91` is the
    source of truth for the description the manifest carries, so the placeholder and the eventual
    release describe the same thing (`reserve.rs:40-43`).
  - `LICENSE-MIT`, `LICENSE-APACHE` at the workspace root — the texts copied in, per
    `reserve.rs:158-165`.
  - `crates/happenstance-core/README.md`, `crates/happenstance-testkit/README.md`,
    `crates/happenstance/README.md` — the house shape for an adapter README: title, one-sentence
    identity, a status blockquote, "which crate do I want", `rust,ignore` fences, a Licence section.
  - `crates/happenstance-testkit/src/contract.rs:253-279` — the three ceiling constants whose real
    values the README states.
- **Renders surfaces**: **none.** `../_design.md` records a signed-off no-surface determination for the
  whole project (`:10-42`), so there is no surface id to claim and no perceptual review owed.
- **Conformance rule(s)**: **none, and this is not adapter-observable.** Packaging metadata is invisible
  to `EventStore`, so no rule in `crates/happenstance-testkit/src/registry.rs` can observe it and adding
  one would be the decorative rule `CLAUDE.md` forbids. The substitute instrument is the recorded
  `cargo package --list` output, whose *named wrong implementation* already exists in the workspace and
  is quoted by `xtask/src/package.rs:20-22`: today's `happenstance-sqlite` packages seven files, none of
  which is one of the three.
- **Clause(s)**: **none.** This story discharges no `SPECIFICATION.md` clause and amends none; the
  clause work in this slice is `spec-and-code-reconciliation`'s (AC-016). `cargo xtask spec-trace`
  must still pass — it runs unconditionally in the story grain (`.redkiln/config.yaml:36-40`).
- **Advances DoD scenario**: **DoD 10 — "the published crate looks finished"** (`initiative.md:387-389`),
  toward green rather than to green: this story makes licence, description and README *present in the
  artefact*; the rendered-page check remains `publication-and-positioning`'s. It also removes the last
  packaging obstacle in front of DoD 9 for any future consumer of this adapter.

## PR boundary

The paths this story may touch. `redkiln verify --grain story` reads the first fenced block under this
heading and fails on any file changed outside it.

```
crates/happenstance-sqlite/Cargo.toml
crates/happenstance-sqlite/README.md
crates/happenstance-sqlite/LICENSE-MIT
crates/happenstance-sqlite/LICENSE-APACHE
xtask/src/reserve.rs
.bklg/from-contract-to-published-library/sqlite-durable-store/crates-io-name-and-packaging-facts/**
```

Narrow on purpose. `xtask/src/reserve.rs` is the one entry that is not obviously this crate's, and it is
here for exactly one reason: §7 of the context pack. If the implementation finds itself wanting to widen
this list, that is a finding to record, not a diff to make.

**In this PR**

- The crates.io reservation for `happenstance-sqlite`, dry-run first, upload human-authorised, evidence
  recorded (version, date, registry URL) in `_ledger.md`.
- `xtask/src/reserve.rs`'s placeholder text reconciled with BR-05's `0.2.0-alpha.1`, or the divergence
  recorded with a reason — decided *before* the irreversible upload.
- `description` in `crates/happenstance-sqlite/Cargo.toml` replaced with `RESERVABLE`'s wording
  (`reserve.rs:88`), and `readme = "README.md"` stated rather than left to auto-discovery, with the same
  rationale `crates/happenstance-core/Cargo.toml:12-15` gives.
- A new `crates/happenstance-sqlite/README.md` carrying the adapter's real durability settings, its
  three enforced ceilings, its conformance status, and which crate a reader actually wants.
- `LICENSE-MIT` and `LICENSE-APACHE` copied into `crates/happenstance-sqlite/`.
- The recorded `cargo package -p happenstance-sqlite --list --allow-dirty` output.

**Explicitly not in this PR**

- **Deleting `publish = false`** (`crates/happenstance-sqlite/Cargo.toml:12`) — architecture brief
  AC-A05 and §8; `publication-and-positioning` (HS-P0016) owns it.
- **`PUBLISHABLE` or `REQUIRED_FILES` in `xtask/src/package.rs`** — unchanged, same owner.
- **`RUNBOOK.md:4230`'s stale exit checkbox** — recorded in `_ledger.md` as superseded by AC-015, not
  edited. The runbook is the plan of record and its phase-8 exit criteria are read at phase closeout.
- **`crates/happenstance-sqlite/src/lib.rs`** — its status banner and module docs, including the
  `#![allow(clippy::todo)]` line, belong to `instrument-markers-removed-and-gate-green`. Adding
  `#![cfg_attr(doctest, doc = include_str!("../README.md"))]` would reach into that file; use
  `rust,ignore` fences instead.
- **`[package.metadata.docs.rs]`** — inert for an unpublished crate, and docs.rs rendering is BR-09's
  registry half.
- **The generated placeholder's `rust-version = "1.85"`** (`reserve.rs:221`) — it binds nothing (the
  placeholder has no dependencies and no code), and the MSRV *promise* is BR-08's, owned by
  `publication-and-positioning`. Note it; do not fix it here.
- **Publishing the real `happenstance-sqlite` crate at any version.**

**Merge DoD**: the name is held on crates.io, `cargo package -p happenstance-sqlite --list --allow-dirty`
lists `README.md`, `LICENSE-MIT` and `LICENSE-APACHE`, `publish = false` and `xtask/src/package.rs` are
byte-identical to their pre-PR state, and `cargo xtask affected --base main` is green.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| The name is claimed through the repository's own generator | `cargo xtask reserve happenstance-sqlite` writes a standalone `0.0.0` placeholder under `target/reserve/happenstance-sqlite/` with both licences copied in, a README and a `lib.rs` that both say plainly it has no functionality. The command prints the `--dry-run` line first and the upload line second, in that order, deliberately. | `xtask/src/reserve.rs:137-191`; the row claiming this name for phase 8 at `:86-91`; dispatch at `xtask/src/main.rs:670` |
| The upload is irreversible and human-authorised | `cargo publish` needs a token in `$CARGO_HOME/credentials.toml`; a published version can be yanked, never removed. An agent without credentials records a handoff. Never a claim the name was taken. | `xtask/src/reserve.rs:181-188`; `RUNBOOK.md:4190-4192` |
| The placeholder's text is true at the moment it is published | Generator templates promise `0.1.0-alpha.1`; the initiative's first published release is `0.2.0-alpha.1`. Reconcile, or record the divergence with a reason, before uploading — the six unreserved names inherit the choice. | `xtask/src/reserve.rs:258`, `:276`; `.bklg/from-contract-to-published-library/initiative.md:288` (BR-05) |
| The manifest describes the adapter that now exists | `description` drops `" Not yet implemented."` and becomes `RESERVABLE`'s wording verbatim, so the placeholder and the eventual release describe the same thing to anyone browsing. | `crates/happenstance-sqlite/Cargo.toml:3` (today); `xtask/src/reserve.rs:88` (target wording), `:40-43` (why they must agree) |
| `readme` is stated, not auto-discovered | Cargo would find `README.md` anyway; the key is what crates.io renders, and a silent default is a poor thing to rely on for a crate's first impression. Same rationale, same wording as the contract crate. | `crates/happenstance-core/Cargo.toml:12-15`; `crates/happenstance-testkit/Cargo.toml:22` |
| Licence texts live **inside** the package directory | Copies, not links or relative paths. Cargo will not follow a path outside the package and does not warn when the `license` field is backed by nothing — D11 was three crates whose metadata promised two licences neither of which shipped. | `xtask/src/package.rs:1-22`, `:88-94`; `xtask/src/reserve.rs:158-165` |
| The README states this adapter's real, current facts | Journal mode, the `synchronous` setting and the finite busy timeout as `schema-migration-and-identity` set them; the three enforced ceilings as `append` enforces them; the shape this crate deliberately represents (serialising, `Send`, native, one `Mutex`-guarded connection); and its conformance status now that the suite is green. Not the aspirational text the module doc carried while it was an instrument. | `../_storymap.md:184-185`; `crates/happenstance-testkit/src/contract.rs:253-279` (the three ceilings); `crates/happenstance-sqlite/src/lib.rs:26-34` (the shape statement to carry forward) |
| The README follows the house shape | Title, one-sentence identity linking the repository and the DCB specification, a status blockquote, a "which crate do I want" pointer, a Licence section. Rust fences are `rust,ignore` unless the crate opts into doctesting its own README — which it does not, here. | `crates/happenstance-core/README.md:1-24,32,58`; `crates/happenstance-testkit/README.md:1-27`; `crates/happenstance/src/lib.rs:1-10` (the alternative, and why it is not taken here) |
| The artefact is inspected, not assumed | `cargo package -p happenstance-sqlite --list --allow-dirty` is run and its output recorded. `--allow-dirty` because the check must work on an uncommitted tree — the only tree anyone runs it against before pushing. | `xtask/src/package.rs:69-74`, `:110-122`; `../_decomposition.md:641` |
| The packaging gate still does not check this crate — by design | `cargo xtask package-check` derives the publishable set from `cargo metadata` and reconciles it against the hand list. With `publish = false` standing, `happenstance-sqlite` appears in neither, and the step prints the same three-crate reconciliation as before. That invariance is itself evidence. | `xtask/src/package.rs:24-42`, `:86`, `:172-218`; architecture brief `../_decomposition.md:379-397` |
| The story grain gate is green | `cargo xtask affected --base main` — it re-derives the changed file set from git including untracked files, maps it to packages plus dependents, and runs the file-reading lints and `spec-trace` unconditionally, which is what keeps a metadata-only story from compiling nothing and calling it green. | `.redkiln/config.yaml:28-40`; `xtask/src/affected.rs` |
| Nothing here is adapter-observable | No conformance rule can see packaging metadata, and inventing one would be a rule no adapter can fail. The falsifier is the workspace's own current state: `cargo package -p happenstance-sqlite --list` exits 0 today, lists seven files, and none is one of the three. | `xtask/src/package.rs:20-22`; `CLAUDE.md`, *The rule that matters* |

## Data and migrations

**N/A.** This story touches no schema, no database and no persisted state. The SQLite schema and its
migration 1 are `schema-migration-and-identity`'s (`../_storymap.md:51`), already landed by the time this
story runs; the projection store's independently-migrated checkpoint schema is
`projection-store-passes-the-borrowed-suite`'s. The only artefacts this story produces are a manifest,
three text files in a crate directory, and one row in a public registry — and the registry entry is the
closest thing here to an irreversible migration, which is why the dry-run precedes it.

## Acceptance criteria

Every criterion is framed from the goal of a person named in `initiative.md`'s persona ACs — the
evaluator (AC-08, AC-09), the application author (AC-03), and the downstream publisher who owns
`publication-and-positioning` (HS-P0016). Together they discharge project **AC-015**
(`../project.md:268-272`), both halves of it: the crate becomes publishable, *and* publishing stays
someone else's decision.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **GIVEN** an evaluator who has met the name `happenstance-sqlite` in `spec/SPECIFICATION.md` and `RUNBOOK.md` and goes looking for it, **WHEN** they open `https://crates.io/crates/happenstance-sqlite`, **THEN** the name resolves to this project — because `cargo xtask reserve happenstance-sqlite` generated the `0.0.0` placeholder, its `--dry-run` was verified first, and the upload was performed under explicit human authorisation, with version, date and registry URL recorded in `_ledger.md`. **AND** if no crates.io credential is available, the ledger row records a *named handoff* — who holds the token and what remains — and never a claim that the name was taken. | Manual, recorded. `cargo xtask reserve happenstance-sqlite` output and the `cargo publish --manifest-path target/reserve/happenstance-sqlite/Cargo.toml --dry-run` transcript pasted verbatim into `_ledger.md` (`xtask/src/reserve.rs:137-191`; dispatch `xtask/src/main.rs:670`); reviewer resolves the registry URL. |
| AC-002 | **GIVEN** an evaluator reading the placeholder's own front page to learn when this crate becomes worth depending on, **WHEN** it names the first functional release, **THEN** it names the version this initiative will actually ship — `0.2.0-alpha.1` per BR-05 (`../../initiative.md:288`) — because `xtask/src/reserve.rs:258` and `:276` were reconciled *before* the irreversible upload, **OR** the divergence is recorded in `_ledger.md` with a stated reason. Whichever is chosen, the six unreserved names inherit it. | `rg -n "0\.1\.0-alpha\.1" xtask/src/reserve.rs` returns nothing (or the ledger carries the reasoned divergence); re-run `cargo xtask reserve happenstance-sqlite` and `rg -n "0\.2\.0-alpha\.1" target/reserve/happenstance-sqlite/README.md target/reserve/happenstance-sqlite/src/lib.rs` matches both; `cargo test -p xtask` green. Ordering proved by the ledger: the reconciliation's evidence predates AC-001's upload timestamp. |
| AC-003 | **GIVEN** an application author scanning crates.io search results for a SQLite event store (initiative AC-03), **WHEN** `happenstance-sqlite` appears, **THEN** its one-line description says what the adapter *is* and no longer ends `"Not yet implemented."` of a crate that now passes the conformance suite — `crates/happenstance-sqlite/Cargo.toml:3` carries `RESERVABLE`'s wording verbatim (`xtask/src/reserve.rs:88`) — **AND** `readme = "README.md"` is stated in the manifest rather than left to Cargo's auto-discovery, for the reason `crates/happenstance-core/Cargo.toml:12-15` gives. | `rg -n "Not yet implemented" crates/happenstance-sqlite/Cargo.toml` empty; `rg -n '^readme = "README.md"' crates/happenstance-sqlite/Cargo.toml` matches; the description string compared character-for-character against `xtask/src/reserve.rs:88` in review; `cargo xtask affected --base main` green proves the manifest still parses. |
| AC-004 | **GIVEN** a consumer who unpacks the `.crate` to exercise the choice `license = "MIT OR Apache-2.0"` invites them to make, **WHEN** they look inside the package directory, **THEN** both licence texts are there as byte-identical copies of the workspace-root files — not references, not relative paths outside the package, which is precisely the shape D11 shipped (`xtask/src/package.rs:1-22`). | `cargo package -p happenstance-sqlite --list --allow-dirty` lists `LICENSE-MIT` and `LICENSE-APACHE`; `git diff --no-index LICENSE-MIT crates/happenstance-sqlite/LICENSE-MIT` and the same for `LICENSE-APACHE` both produce no output. |
| AC-005 | **GIVEN** an evaluator who wants to know what this store promises about durability and limits *before* reading a line of its source (initiative AC-09), **WHEN** they read the crate's front page, **THEN** it states this adapter's real, current facts: the journal mode, the `synchronous` setting and the finite busy timeout `schema-migration-and-identity` configured; the three ceilings `append` enforces (`MAX_EVENT_DATA_LEN`, `MAX_TAGS_PER_EVENT`, `MAX_EVENTS_PER_BATCH`, declared per `crates/happenstance-testkit/src/contract.rs:253-279`); and that the conformance suite is green against it — as numbers read out of the landed code, never the aspirational text the module doc carried while the crate was an instrument. | Fact-by-fact citation review recorded in `_ledger.md`: every number in the README paired with the `crates/happenstance-sqlite/src/` or `crates/happenstance-sqlite/tests/` line it comes from. A fact with no citable line is deleted, not softened. `cargo test -p happenstance-sqlite` green on the same tree is what makes the conformance sentence true. |
| AC-006 | **GIVEN** an application author who arrived by following `crates/happenstance-core/README.md:14-22`'s "which crate do I want" pointer, **WHEN** they land on this crate's page, **THEN** it has the same shape its siblings have — title, a one-sentence identity linking the repository and the DCB specification, a status blockquote, a which-crate pointer, and a Licence section — within the sibling density budget, **AND** every Rust fence is marked `rust,ignore`, so no fence advertises an example that cannot compile (D10). No `include_str!` is added to `crates/happenstance-sqlite/src/lib.rs`, which belongs to the predecessor story. | ``rg -n '^```rust$' crates/happenstance-sqlite/README.md`` empty (only `rust,ignore` fences); `rg -n "include_str" crates/happenstance-sqlite/src/lib.rs` empty; `git diff main -- crates/happenstance-sqlite/src/` empty; density checked against the measured budget in *Interaction quality* below. |
| AC-007 | **GIVEN** the publisher who will later add this crate to `PUBLISHABLE` and needs the packaging gate to pass on the first attempt rather than rediscover D11, **WHEN** they read this story's ledger, **THEN** it carries the *verbatim* output of `cargo package -p happenstance-sqlite --list --allow-dirty`, including the command line, showing `README.md`, `LICENSE-MIT` and `LICENSE-APACHE` in the listing — the run, not a claim that it was run. | The transcript in `_ledger.md`, reproducible by re-running the command. Its falsifier is not hypothetical and is already quoted at `xtask/src/package.rs:20-22`: the same command on the pre-PR tree exits 0, lists seven files, and none of them is one of the three. |
| AC-008 | **GIVEN** the maintainer of `publication-and-positioning` (HS-P0016), whose project owns whether this crate ships at all, **WHEN** this PR merges, **THEN** the decision is still theirs — `crates/happenstance-sqlite/Cargo.toml:12`'s `publish = false` stands, `xtask/src/package.rs` is byte-identical to its pre-PR state, `cargo xtask package-check` prints the same three-crate reconciliation as before, **AND** `RUNBOOK.md:4230`'s stale "`publish = false` removed" checkbox is recorded in `_ledger.md` as superseded by AC-015 rather than edited or silently diverged from. | `git diff main -- xtask/src/package.rs RUNBOOK.md` empty; `rg -n "^publish = false" crates/happenstance-sqlite/Cargo.toml` matches; `cargo xtask package-check` (`xtask/src/main.rs:680`) prints `publishable set agrees with the manifests: happenstance-core, happenstance, happenstance-testkit` (`xtask/src/package.rs:213-216`); the supersession note present in `_ledger.md`. |

## Interaction quality

This story renders **no user-facing surface**: `../_design.md:10-42` records a signed-off no-surface
determination for the whole project, and `initiative.md:196` puts any screen, UI or documentation site
out of the initiative's scope. That disposes of most of the STATE family honestly — but it does **not**
dispose of the COMPOSITION family, and waving it away here would be the exact failure BR-09 names
(`../../initiative.md:292`): *registry-facing completeness fails silently*.

The reason is that this story's artefact **is** rendered, by something the repository does not control.
crates.io renders `description` as the search line, `license` as a pair of chips, and `README.md` as the
crate's front page; docs.rs renders the same metadata again. The library analogue of "an unstyled render
satisfies every data-attribute assertion" is exact and already has a name in this workspace: a `.crate`
whose *manifest* is perfect and whose *tarball* carries no licence text and no README. That is D11
(`xtask/src/package.rs:1-22`), and it passed every check anyone had.

So the composition invariants below are real and blocking — and each one is **carried by an AC-### row in
the table above**, not by a bullet here. This section only says which row carries which invariant.

**STATE family**

| Invariant | Disposition | Carried by |
| --- | --- | --- |
| In-place vs context-jump; non-occlusion; preserved focus / scroll / selection; keyboard reachability | **N/A** — no navigation, no view, no input. `../_design.md:10-42`, signed off 2026-08-12. | — |
| **Reversibility** | **Applies, and is the sharpest invariant in the story.** One action in this PR cannot be undone: a crates.io version can be yanked but never removed. The invariant is discharged by *ordering* — the mandatory `--dry-run`, and every text claim reconciled before the upload — not by an undo path, because there is none. | **AC-001** (dry-run first, human-authorised, handoff is a legitimate outcome), **AC-002** (the version claim reconciled *before* the upload, proved by ledger timestamps) |

**COMPOSITION family** — the design's own N/A determination applies to screens; these are the
registry-render analogues it does not cover, and they are taken from what the three sibling crates
already do.

| Invariant | What it means here | Carried by | How verified |
| --- | --- | --- | --- |
| **Presentation exists at all** | Every "control" the registry renders carries real composed content, not bare markup: a description that describes, a README with the adapter's actual facts, licence chips backed by licence text. The named anti-pattern is metadata that promises what the tarball does not carry. | **AC-003**, **AC-004**, **AC-005** | `cargo package --list` listing (AC-007's transcript) plus the fact-by-fact citation review |
| **Composition / placement** | The licence texts must sit *inside* the package directory. A correct path pointing outside it is a placement failure that Cargo will not warn about (`xtask/src/package.rs:5-11`). | AC-004 | `git diff --no-index` against the workspace-root files; the packaging listing |
| **Transience** | Nothing here is transient. The placeholder's text is **permanent chrome** — published once, yankable, never removable — which is why its content is an acceptance criterion rather than a detail. | AC-002 | ledger ordering; `rg` over `xtask/src/reserve.rs` |
| **Density budget (real numbers)** | The sibling READMEs set it and it is measured, not guessed: `crates/happenstance/README.md` is **60** lines, `crates/happenstance-core/README.md` is **67** lines with **5** `##` sections, `crates/happenstance-testkit/README.md` is **130** lines. Budget for this one: **60–130 lines, 4–6 `##` sections, identity in one sentence, at most one fenced block above the "which crate do I want" pointer.** | AC-006 | line and `##` count read off the file and recorded in `_ledger.md` |
| **Hierarchy** | Identity before status before pointer before detail. `crates/happenstance-core/README.md:1-22` puts the one-sentence identity, then the status blockquote, then "Which crate do I want?" above everything technical; this crate's status line is the one that genuinely changed (it is no longer "a documented stub", `:8-9`). | **AC-006**, **AC-005** | review against `crates/happenstance-core/README.md:1-24` |
| **Named anti-patterns** | Three, each already named in the repository: **(i)** metadata promising licences the artefact does not carry — D11, `xtask/src/package.rs:9-11`; **(ii)** a `rust` fence that does not compile as the crate's first impression — D10, which is why `rust,ignore` is chosen deliberately at `crates/happenstance-core/README.md:58`; **(iii)** a parked placeholder with no stated purpose, which crates.io policy prohibits and `xtask/src/reserve.rs:56-66` is written to avoid. | **(i)** AC-004/AC-007, **(ii)** AC-006, **(iii)** AC-002 | as above |

## Error conditions

| id | condition | required behaviour |
| --- | --- | --- |
| **EC-001** | No crates.io token in `$CARGO_HOME/credentials.toml`, so `cargo publish` cannot run (`xtask/src/reserve.rs:187-188`). | **AC-001's row stays `satisfied: false`** and records a named handoff: who holds the token, the exact command awaiting them, and the dry-run transcript proving the placeholder is ready. Every other AC still lands in this PR. A ledger row claiming the name was taken without a resolvable registry URL is the one outcome this story treats as a defect rather than a delay. |
| **EC-002** | `cargo publish` fails because the name is already held by someone else. | **Blocking finding, escalate — do not route around it.** It invalidates `xtask/src/reserve.rs:86-91` and reaches BR-09 and the initiative's naming premise. Halt the story, record the registry response verbatim, and raise it to the initiative rather than choosing a substitute name inside an adapter story. |
| **EC-003** | `cargo package -p happenstance-sqlite --list --allow-dirty` fails instead of listing — most likely `readme = "README.md"` (AC-003) landing before `README.md` (AC-005) exists. | Fix the manifest or add the file. **Do not** reach for `--no-verify` or drop the `readme` key: the failing command is the story's only instrument, and disabling it is the decorative-gate failure `CLAUDE.md` forbids. |
| **EC-004** | The upload succeeded while the placeholder still claimed `0.1.0-alpha.1`. | **Unrecoverable by edit** — yank does not remove. AC-002's ordering exists to make this unreachable. If it happens: record it as a defect in `_ledger.md`, fix `xtask/src/reserve.rs` immediately so the six unreserved names do not inherit it, and hand the published-text divergence to `publication-and-positioning`. |
| **EC-005** | An implementer, reading `RUNBOOK.md:4230` in good faith, deletes `publish = false` so the packaging gate will check this crate. | `cargo xtask package-check` fails inside `reconcile()` (`xtask/src/package.rs:188-195`) naming the promoted-but-unreconciled crate. The correct response is to **restore the line**, not to add the name to `PUBLISHABLE` — the promotion is HS-P0016's to make (`../_decomposition.md:379-395`). AC-008 exists so this is a criterion, not an omission. |
| **EC-006** | The README states a durability fact the code does not implement — a journal mode the migration never sets, a ceiling `append` does not enforce. | **Silent by construction: nothing compiles a README.** Caught only by AC-005's fact-by-fact citation. A fact that cannot be cited to a real `src/` or `tests/` line is deleted rather than reworded — a README describing a different store is worse than no README. |
| **EC-007** | The copied licence files differ from the workspace-root originals (CRLF translation on a Windows checkout is the realistic cause). | Treated as a failure of AC-004, not a formatting nit. Divergent licence text across crates in one workspace is a legal defect; re-copy and re-verify with `git diff --no-index`. |

## Non-functional

| id | requirement | why, and where it is checked |
| --- | --- | --- |
| **NF-001** | The story-grain gate stays green: `cargo xtask affected --base main`. | `.redkiln/config.yaml:28-40`. Worth stating because this diff is mostly non-code: it maps to `happenstance-sqlite` (the manifest) and `xtask` (`reserve.rs`), so both packages and their dependents compile under `-D warnings`, and the five file-reading lints plus `spec-trace` run unconditionally — which is exactly what stops a metadata-only story compiling nothing and calling it green (`config.yaml:36-39`). |
| **NF-002** | No new dependency, no new feature, no change to the crate's feature matrix (`crates/happenstance-sqlite/Cargo.toml:28-33`). | A `readme` key and three text files must not move the powerset `cargo hack` walks in the full gate. Anything that would is out of this story's boundary. |
| **NF-003** | The licence copies are byte-identical to the workspace-root texts — copied, never reflowed or re-wrapped. | The three publishable crates already hold byte-identical copies and `xtask/src/reserve.rs:158-165` copies for the same reason and says so. |
| **NF-004** | `README.md` is UTF-8 with LF endings and fence tags matching the siblings, so `cargo package`, crates.io's renderer and the reviewer all see the same bytes. | Windows checkout; EC-007 is the same hazard on the licence files. |
| **NF-005** | The reservation costs the workspace nothing at build time. | The placeholder is generated under `target/reserve/` and declares an empty `[workspace]` table so it is its own workspace (`xtask/src/reserve.rs:151`, `:229-233`). No workspace-member edit and no `Cargo.lock` change may appear in this PR. |
| **NF-006** | `cargo xtask spec-trace`'s citation count does not fall. | This story amends no clause; the clause work in this slice is `spec-and-code-reconciliation`'s (AC-016). `reachability_static` runs it at the integration grain (`.redkiln/config.yaml:48`) and `affected` runs it unconditionally at the story grain. |

## Implementation notes (non-prescriptive)

**Do the irreversible act last.** The ordering is the story's main design decision and it follows from
context-pack §6 and §7: reconcile `xtask/src/reserve.rs` (AC-002) → generate and `--dry-run` → land the
manifest, README and licences (AC-003 – AC-006) → record the packaging listing (AC-007) → confirm the
non-promotion invariance (AC-008) → **only then** upload (AC-001). AC-001 is the last ledger row flipped,
and it is flipped by a human decision.

**Source the README's numbers from the code that landed, not from the trait.** `contract.rs:253`, `:262`
and `:279` are all `None` — those are the *defaults* a fixture overrides. The real ceilings live in
`SqliteFixture` under `crates/happenstance-sqlite/tests/`, put there by `sqlite-fixture-and-whole-suite`,
and the journal mode / `synchronous` / busy-timeout settings live in the migration and connection setup
from `schema-migration-and-identity`. Read them; do not restate the brief's expectations of them.

**Borrow the README's shape rather than inventing one.** Read `crates/happenstance-core/README.md` and
`crates/happenstance-testkit/README.md` first. The single section whose content genuinely changes for this
crate is the status blockquote: `happenstance-core`'s says "every storage adapter is a documented stub"
(`:8-9`), and this story is the first time that sentence is wrong about one of them. Saying so plainly is
this story's most visible outcome — and note that correcting the *sibling's* sentence is not in this PR's
boundary.

**Keep the description in one place.** It must match `xtask/src/reserve.rs:88` verbatim, because the
placeholder and the eventual release are supposed to describe the same thing to anyone browsing
(`reserve.rs:40-43`). If that wording now reads wrong for a crate that also ships a projection store,
change it in `reserve.rs` *first* and copy it across, so the two cannot drift.

**Copy the licences the same way the generator does.** A plain byte copy from the workspace root
(`reserve.rs:161-165` is the reference), then verify with `git diff --no-index`. On this Windows checkout,
confirm no CRLF translation crept in before recording AC-004.

**Resist widening the boundary.** `[package.metadata.docs.rs]`, the generated placeholder's
`rust-version = "1.85"` (`reserve.rs:221` — it binds nothing, since the placeholder has no dependencies
and no code, and the MSRV *promise* is BR-08's), and `keywords`/`categories` tuning are all named out of
scope in the front half. Record them as findings; do not fix them here. Wanting to widen the PR boundary
is itself the finding.

**`--allow-dirty` is not a shortcut.** It is in the command because the check must work on an uncommitted
tree, which is the only tree anyone runs it against before pushing (`xtask/src/package.rs:69-74`).

## Tests and CI (merge gate)

Grounded in the project's testing brief, §1's tier table (`../_decomposition.md:598-605`) and §2's AC-015
row (`:641`). Note what the brief says plainly: this story's primary proof is a **recorded manual
packaging run**, not a gate step, because `xtask/src/package.rs`'s `PUBLISHABLE` does not name an
unpublished crate. Everything else below is a *guard* that this story broke nothing.

| Tier | Command / path | Proves |
| --- | --- | --- |
| **Static (story grain)** | `cargo xtask affected --base main` (`.redkiln/config.yaml:40`) | The manifest still parses and `happenstance-sqlite` + `xtask` + dependents build clean under `-D warnings`; the five file-reading lints and `spec-trace` run unconditionally so a metadata-shaped diff cannot compile nothing and report green. **NF-001, NF-006, AC-003.** |
| **Static (integration grain)** | `cargo xtask lints && cargo xtask spec-trace` (`reachability_static`, `.redkiln/config.yaml:48`) | No clause citation rotted; this story amends none, so the count must be unchanged. **NF-006.** |
| **Static (targeted greps)** | `rg -n "Not yet implemented" crates/happenstance-sqlite/Cargo.toml`; ``rg -n '^```rust$' crates/happenstance-sqlite/README.md``; `rg -n "include_str" crates/happenstance-sqlite/src/lib.rs`; `rg -n "0\.1\.0-alpha\.1" xtask/src/reserve.rs` | Each returns empty. **AC-003, AC-006, AC-002.** |
| **Diff invariance** | `git diff main -- xtask/src/package.rs RUNBOOK.md crates/happenstance-sqlite/src/`; `git diff --no-index LICENSE-MIT crates/happenstance-sqlite/LICENSE-MIT` (and `LICENSE-APACHE`) | The three files this story must not touch are untouched, and the licence copies are byte-identical. **AC-008, AC-004, AC-006.** |
| **Unit / xtask** | `cargo test -p xtask` — including `package::tests::{reads_every_publish_spelling, ignores_names_that_are_not_workspace_members, rejects_a_publish_value_it_does_not_understand}` (`xtask/src/package.rs:427-456`) | The packaging gate's own scanner still works after the `reserve.rs` edit. |
| **Unit / type-level** | `cargo test -p happenstance-sqlite --test shapes` (`crates/happenstance-sqlite/tests/shapes.rs`) | Unchanged shape guard — this story adds no code, so a failure here means the boundary was widened. |
| **Conformance** | `cargo test -p happenstance-sqlite` | The README's conformance sentence (AC-005) is only true if the suite is green on the same tree that ships it. Owned by the predecessor story; re-run here because this story *asserts* its result in public text. |
| **Packaging — the deliverable, manual and recorded** | `cargo package -p happenstance-sqlite --list --allow-dirty`, output pasted verbatim into `_ledger.md` | `README.md`, `LICENSE-MIT` and `LICENSE-APACHE` are in the artefact. **AC-007, AC-004, AC-005.** Falsifier: the same command pre-PR lists seven files and none of the three (`xtask/src/package.rs:20-22`). |
| **Packaging invariance** | `cargo xtask package-check` (`xtask/src/main.rs:680`) | Prints the same three-crate reconciliation as before this PR — the evidence that publishing was *not* decided here. **AC-008.** |
| **Registry (manual, human-gated)** | `cargo publish --manifest-path target/reserve/happenstance-sqlite/Cargo.toml --dry-run`, then the upload | **AC-001, AC-002.** The dry-run is mandatory and its transcript is ledger evidence; the upload is a human act, and EC-001's handoff is a legitimate terminal state for this row. |
| **Slice bar (not this story's own)** | `cargo xtask ci --fast` (`integration_scoped`, `.redkiln/config.yaml:55`) | Owned by `instrument-markers-removed-and-gate-green`; named here only so this story is checked against not regressing it. The whole `cargo xtask ci` belongs to `closeout-and-durable-audience` (HS-P0019). |

## Risks and coupling (PR-scoped)

- **R1 — one action in this PR cannot be undone.** A crates.io version can be yanked, never removed.
  *Mitigation:* AC-002 is ordered before AC-001, the `--dry-run` is mandatory, the upload is
  human-authorised, and EC-001 makes "not uploaded, handed off" an acceptable outcome rather than a
  pressure to guess. If any of those three feels like ceremony during implementation, that is the risk
  materialising.
- **R2 — the runbook actively misleads.** `RUNBOOK.md:4230` instructs the opposite of AC-015. An
  implementer working the phase's exit criteria in good faith will delete `publish = false`.
  *Mitigation:* AC-008 turns the invariance into a criterion with its own ledger row; EC-005 names the CI
  failure it would cause; and the supersession is *recorded* rather than the runbook silently edited,
  because phase exit criteria are read at closeout by someone who was not here.
- **R3 — the README has no compiler behind it.** It is the only artefact in this PR that can be wrong in a
  way nothing detects. *Mitigation:* AC-005's fact-by-fact citation, and the deliberate `rust,ignore`
  choice (AC-006) — which trades away doctest coverage to avoid reaching into
  `crates/happenstance-sqlite/src/lib.rs`, a file the predecessor story owns. That trade is a real cost,
  accepted knowingly, and it is reversible later by `publication-and-positioning` when the crate is
  actually published.
- **R4 — editing a shared generator from inside an adapter story.** `xtask/src/reserve.rs` is not this
  crate's file. *Mitigation:* the edit is bounded to one sentence's version number in two templates, and
  the reason it belongs here is that this is the first use of the generator since BR-05 was written — the
  remaining six reservations inherit the decision either way, so making it silently is the worse option.
- **R5 — slice-mate collision.** `spec-and-code-reconciliation` runs in parallel behind the same
  dependency and touches `spec/SPECIFICATION.md` and possibly `crates/happenstance-sqlite/src/`. *The PR
  boundaries are disjoint by construction:* this story's boundary names no `src/` path and no `spec/`
  path. If a merge conflict appears there, the boundary was violated, not merely unlucky.
- **R6 — two of eight ACs are proved by a transcript, not a re-runnable gate.** AC-001 and AC-007 are
  ledger artefacts. A paraphrased or summarised transcript makes this story's central claim
  unfalsifiable — which is exactly the failure mode `../_decomposition.md:389-393` was written to
  prevent. Paste the command line and the output verbatim.
- **R7 — coupling to facts owned upstream.** The README's durability numbers come from
  `schema-migration-and-identity` and its ceilings from `sqlite-fixture-and-whole-suite`. If either
  changes after this story lands, the README goes stale silently. *Accepted:* the cost of preventing it
  (a doctest-backed README) is the `include_str!` this story deliberately declines; record the coupling
  and hand it to `publication-and-positioning`, which re-reads the page before publishing anyway.

## Dependencies

**Blocks on**

- **`instrument-markers-removed-and-gate-green`** (HS-S0045) — the story's declared `depends_on`. The
  README cannot state the adapter's real durability settings, enforced ceilings and conformance status
  while a `todo!()` is still on a SQLite path; the story map says exactly this
  (`../_storymap.md:180-185`). Note the asymmetry the map also records: the *reservation half* (AC-001,
  AC-002) has **no** code dependency and `RUNBOOK.md:4191-4193` says a name is claimed when its phase
  starts. If the slice is pulled forward, do the reservation early and land the manifest facts on the
  dependency — the ordering constraint that matters is AC-002 before AC-001, not the story's position in
  the slice.

**Parallel with**

- **`spec-and-code-reconciliation`** — same slice `publishable-and-reconciled`, same predecessor, disjoint
  PR boundary (`../_storymap.md:180-185`). The three slice-mates are implemented in one context and land
  as one hand-over surface.

**Unlocks**

- **`publication-and-positioning`** (HS-P0016) — AC-015's second half is theirs to spend. Deleting
  `publish = false`, adding the name to `PUBLISHABLE` (`xtask/src/package.rs:86`) and DoD 10's
  *looking at the rendered registry page* (`../../initiative.md:387-389`) all become possible only once
  this story's artefact exists, and none of them can begin before it.
- **`closeout-and-durable-audience`** (HS-P0019) — its whole-gate `cargo xtask ci` run inherits a crate
  that no longer describes itself as unimplemented, and a reservation that no longer needs claiming
  under time pressure at release.

## Anchors (progressive disclosure)

Everything above is enough to start. Open these at the moment named, and never in bulk.

| Anchor | Why it is load-bearing | When to open | Serves |
| --- | --- | --- | --- |
| `xtask/src/reserve.rs` | The generator that performs the reservation, the `RESERVABLE` row claiming this name (`:86-91`), the description the manifest must match (`:88`), the licence-copy rationale (`:158-165`), and the two placeholder templates carrying the stale `0.1.0-alpha.1` claim (`:258`, `:276`). Its module doc (`:12-30`) explains why the placeholder is standalone — do not second-guess that design. | Before touching anything: the version reconciliation must precede the upload. | AC-001, AC-002, AC-003 |
| `xtask/src/package.rs` | Read as a **constraint, never edited**. `:1-22` is D11 stated as a defect and names this story's falsifier verbatim; `:86` is `PUBLISHABLE`; `:94` is `REQUIRED_FILES`; `:172-218` is `reconcile()`, whose promoted-but-unreconciled failure is what deleting `publish = false` would trigger. | Before deciding what "publishable" means here, and again before flipping AC-008. | AC-004, AC-007, AC-008 |
| `.bklg/from-contract-to-published-library/sqlite-durable-store/_decomposition.md` | The architecture brief's §8 (`:379-397`) is the ruling that settles the runbook conflict in this project's favour, with the mechanism spelled out; the testing brief's AC-015 row (`:641`) is why the proof is a recorded run rather than a gate step. | Before implementing AC-008, and before writing AC-007's ledger evidence. | AC-007, AC-008 |
| `RUNBOOK.md` | `:4189-4193` is the instruction to claim the name when the phase starts; `:4224-4230` is the phase-8 exit list containing the **stale** `publish = false` checkbox this story records as superseded. Read both, so the supersession note is accurate about what it supersedes. | When writing AC-008's supersession note in `_ledger.md`. | AC-008, AC-001 |
| `crates/happenstance-core/README.md` | The house shape, measured: 67 lines, 5 `##` sections, identity in one sentence (`:1-6`), status blockquote (`:8-12`), "Which crate do I want?" (`:14-22`), a `rust,ignore` fence (`:58`), a Licence section (`:65-67`). Also the sentence this story makes wrong about one adapter (`:8-9`). | Immediately before writing `crates/happenstance-sqlite/README.md`. | AC-005, AC-006 |
| `crates/happenstance-testkit/README.md` | The upper end of the density budget (130 lines) and the second example of `rust,ignore` fences used where an example cannot stand alone. | Alongside the above, when the README threatens to outgrow the budget. | AC-006 |
| `crates/happenstance-testkit/src/contract.rs` | `:253-279` declares the three ceiling constants and — importantly — shows them defaulting to `None`. The README must state the fixture's *overridden* values, not these. `:249-252` explains what stating a number commits the store to. | When writing the ceilings paragraph of the README. | AC-005 |
| `crates/happenstance-sqlite/Cargo.toml` | The mount point. `:3` is the stale description, `:12` is the `publish = false` that must survive, `:28-33` is the feature matrix NF-002 forbids moving, and there is no `readme` key to edit — it is added. | First file opened for the manifest work. | AC-003, AC-008 |
| `crates/happenstance-core/Cargo.toml` | `:12-15` is the precedent for stating `readme = "README.md"` explicitly rather than relying on auto-discovery, with the rationale attached. | When adding the `readme` key. | AC-003 |
| `.bklg/from-contract-to-published-library/sqlite-durable-store/_storymap.md` | `:180-185` is the merge-order entry that explains the split personality of this story — reservation with no code dependency, manifest facts that need the predecessor — and names the parallel slice-mate. | If the slice order is questioned, or if the reservation is being pulled forward. | AC-001, AC-005 |
| `.bklg/from-contract-to-published-library/initiative.md` | `:288` is BR-05 (`0.2.0-alpha.1`), `:292` is BR-09 (registry completeness verified, not assumed), `:314-316` and `:330-334` are the personas this story's criteria are written from, `:387-389` is DoD 10. | When writing AC-002's reconciliation and when justifying that the README's content is not optional. | AC-002, AC-005, AC-006 |
| `.bklg/from-contract-to-published-library/sqlite-durable-store/_design.md` | `:10-42` is the signed-off no-surface determination, with its evidence. It is what makes the STATE family N/A here — and what does **not** excuse the composition invariants. | Before writing anything that looks like a UI concern, and to justify the *Interaction quality* split. | AC-005, AC-006 |
| `crates/happenstance-sqlite/src/lib.rs` | Read-only for this story. It carries the crate's shape statement (serialising, `Send`, native, one `Mutex`-guarded connection) worth carrying into the README, and it is the file `git diff main --` must show unchanged. | When drafting the README's identity sentence; again when verifying AC-006. | AC-005, AC-006 |
| `.redkiln/config.yaml` | `:28-40` is the story-grain gate this story is measured by and why it runs the lints unconditionally; `:48` and `:55` are the integration-grain commands owned by others; `:67` is `require_ledger: true`, which is why `_ledger.md` is a deliverable and not a courtesy. | Before running the gate, and before assuming any command runs that is not listed here. | AC-007, AC-008 |

## Clarifications resolved during spec

1. **The eight AC ids the front half decided are kept exactly** — AC-001 … AC-008, none added, none
   dropped. The one place a reading was needed was the README, which carries two independent obligations:
   *the facts are true and current* (AC-005) and *the page is composed like its siblings and advertises no
   fence that cannot compile* (AC-006). They fail independently — a truthful README in the wrong shape,
   and a beautifully shaped README describing a different store — so they are two rows, not one.
2. **Interaction quality is not declared N/A, despite `_design.md`'s signed-off no-surface
   determination.** The STATE family genuinely is N/A except for reversibility, which is the sharpest
   invariant in the story. The COMPOSITION family maps cleanly onto the artefact crates.io actually
   renders, and BR-09 exists precisely because that render fails silently. Every composition invariant is
   carried by an AC row in the table, per the extraction rule — none is left as a prose bullet.
3. **The density budget is measured, not asserted.** 60 / 67 / 130 lines and 5 `##` sections come from
   `wc -l` and a read of the three sibling READMEs in this worktree, not from a convention someone
   remembers.
4. **`rust,ignore` over `include_str!`, decided rather than defaulted.** The alternative is real and is
   used in this workspace (`crates/happenstance/src/lib.rs:1-10` compiles its own README under
   `cfg(doctest)`). It loses here on one ground only: it requires editing
   `crates/happenstance-sqlite/src/lib.rs`, which belongs to `instrument-markers-removed-and-gate-green`,
   and it buys nothing while the crate is unpublished. R3 and R7 record the cost so
   `publication-and-positioning` can revisit it.
5. **The `xtask/src/reserve.rs` edit stays in the PR boundary.** It is the single entry that is not this
   crate's. It is in scope because this is the first use of the generator since BR-05 was written and the
   six unreserved names inherit whatever is decided; and it is *ordered before* the irreversible upload
   because a published sentence cannot be corrected.
6. **`RUNBOOK.md:4230` is recorded as superseded, not edited.** The runbook is the plan of record and its
   phase-8 exit criteria are read at closeout by someone who was not in this story. AC-008 requires
   `git diff main -- RUNBOOK.md` to be empty *and* the supersession note to exist in `_ledger.md`; both,
   not either.
7. **AC-001 has a legitimate non-published terminal state.** EC-001 makes a recorded handoff an
   acceptable outcome when no crates.io credential exists. What is never acceptable is a satisfied ledger
   row without a resolvable registry URL — the whole point of BR-09 is that this class of claim fails
   silently.
8. **No conformance rule is added, and that is a decision, not an omission.** Packaging metadata is
   invisible to `EventStore`, so any rule written for it would be a rule no adapter can fail — the
   decorative gate `CLAUDE.md` forbids. The substitute instrument is the recorded `cargo package --list`
   run, whose named wrong implementation is the workspace's own current state.
