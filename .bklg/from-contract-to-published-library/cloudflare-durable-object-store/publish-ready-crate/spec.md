---
item: HS-S0059
stage: spec
created: 2026-08-12T13:46:57.689Z
updated: 2026-08-12T13:46:57.689Z
template_sig: 87bbf1d0
rendered_sig: d4720b05
---

# Spec — happenstance-cloudflare becomes a crate someone can depend on

## Scope lock

| What | Path |
| ---- | ---- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` — Goals, AC-01…AC-15, **Definition of Done** 1–16 |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` — the ten projects, the DAG, the four gate decisions |
| Project | `.bklg/from-contract-to-published-library/cloudflare-durable-object-store/project.md` — **AC-012** is this story's sole trace |
| This spec | `.bklg/from-contract-to-published-library/cloudflare-durable-object-store/publish-ready-crate/spec.md` |
| Key briefs | `.bklg/from-contract-to-published-library/cloudflare-durable-object-store/_decomposition.md` — Architecture brief **ARCH-AC-08** (`:81-85`), Deployment brief **DEPLOY-AC-03** (`:713-717`) and its §4 dependency-surface and §5 release-path notes (`:834-866`) |
| Signed-off design | `.bklg/from-contract-to-published-library/cloudflare-durable-object-store/_design.md` — **no user-facing surface**, approved 2026-08-12; nothing in this story renders |
| This story's discover | `.bklg/from-contract-to-published-library/cloudflare-durable-object-store/publish-ready-crate/discover.md` — the signal ledger, the five answered questions, and **the wrong implementation** this story rejects |
| Story map row | `.bklg/from-contract-to-published-library/cloudflare-durable-object-store/_storymap.md` — the `publish-readiness` row (`:58`) and **Merge order** §5 (`:162-165`) |
| Roadmap pointer | `RUNBOOK.md:4241-4307` — phase 9's goal, work, proof artefact and exit criteria; `:4254-4261` the crate-name note and the phase-0 reservation rule; `:4301` the exit box this story ticks |

## One-line PR slice

Make the crate depend-on-able: `LICENSE-MIT`, `LICENSE-APACHE` and `README.md` beside
`Cargo.toml`, `publish = false` removed so `cargo xtask package-check` covers it, the
name reserved per `xtask/src/reserve.rs`, and `cargo xtask ci` green including every
`wasm32` step.

## Executive summary

**What this PR lands.** `crates/happenstance-cloudflare/` today holds exactly two
entries — `Cargo.toml` and `src/` — while every other publishable crate in the
workspace carries both licence texts and a README beside them
(`crates/happenstance-core/`, `crates/happenstance/`, `crates/happenstance-testkit/`).
This PR adds those three files, deletes `publish = false`
(`crates/happenstance-cloudflare/Cargo.toml:12`), admits the crate to
`PUBLISHABLE` in `xtask/src/package.rs:86` so the packaging assertion actually covers
it, rewrites the crate's front page so it stops saying `# Status: not implemented`
(`crates/happenstance-cloudflare/src/lib.rs:4-10`), reserves the crates.io name
through the documented `0.0.0` mechanism (`xtask/src/reserve.rs:92-97`), and observes
`cargo xtask ci` green on the result.

**The delta, not the restatement.** The project charter and the story map already say
*what* publish-readiness is; three things this story decides that they do not:

1. **Removing `publish = false` is not a manifest edit — it is a registration.**
   `package.rs` derives the publishable set from `cargo metadata` and *fails* when the
   derivation disagrees with the hand list (`xtask/src/package.rs:172-218`). Deleting
   the flag without adding the name makes the gate red with "Cargo will publish
   happenstance-cloudflare but this step does not check it"; adding the name without
   the three files makes it red with the missing filenames. The two halves land
   together or not at all, and that coupling is the mount, not a chore.
2. **The claim the README makes is the only artefact in this project no test reads.**
   `cargo package --list` asserts three *filenames* and reads none of their contents.
   So the compliance sentence is written in the same words the `workerd` run prints —
   the fixture's own declined-capability reasons and the stated non-invocation of the
   concurrency family — and the usage example is a compiled doctest, not a fenced
   block. Both are inherited facts from slice-mates, not new claims invented here.
3. **AC-012's "all four `wasm32` steps" is superseded by "every `wasm32` step".**
   `wasm_steps()` names four today (`xtask/src/main.rs:784-791`); `wasm-execution-seam`
   adds the fifth, the one that *executes* rather than checks. The higher bar wins
   (`_storymap.md` **Merge order** §5), and the green-gate claim is scoped to a tree
   that contains it.

**Where it stops.** At *publish-ready*. The version, the registry landing page, the
MSRV promise, the semver diff and the clause-ledger audit are
`publication-and-positioning` (HS-P0016)'s (`project.md`, **Out of scope**).

## Context pack

Read this section and you can start. Everything below it is detail; everything deeper
than it is linked, not pasted.

### The decision this story exists to execute

`happenstance-cloudflare` is currently outside the one gate step that would notice it
shipping without a licence. That is not an accident of the tree, it is the meaning of
`publish = false`: `package.rs` scans `cargo metadata --no-deps` and treats
`publish = []` as "not published, not checked" (`xtask/src/package.rs:342-352`). The
runbook already recorded the failure mode this creates in its own history — D11 shipped
three crates whose metadata promised two licences neither of which was in the tarball,
and the fix's own exit box was ticked before the step that keeps it true existed
(`RUNBOOK.md:898-906`). This story moves the crate across that line **in one change**,
because either half alone is a red gate.

### What is decided elsewhere and is binding here

- **Publication is not this story's, and the boundary is load-bearing rather than
  bureaucratic.** `project.md` **Out of scope** hands publishing, positioning, the MSRV
  promise, the semver diff and the clause-ledger audit to HS-P0016; this story produces
  the artefacts that make those possible and makes none of those claims. Concretely:
  no version is chosen, no `0.2.0` is cut, and no sentence is written into the README
  that commits a minimum supported Rust version.
- **The name is reserved by the documented `0.0.0` mechanism, never by publishing the
  real crate.** `xtask/src/reserve.rs:12-30` states both reasons and the second is the
  one that binds: publishing the real crate at its real version makes the API
  semver-binding, and the whole runbook is built around freezing the contract
  deliberately, against evidence. `RESERVABLE` already carries the
  `happenstance-cloudflare` row at phase 9 with the description the real crate is meant
  to end up with (`xtask/src/reserve.rs:92-97`) — so the reservation needs no code
  change, only a run, a `--dry-run` and a publish of the placeholder.
- **The reservation is independent of this crate's content and must not block on a
  registry round trip.** `RUNBOOK.md:4261` says claim the name *at the start of the
  phase*; the placeholder shares nothing with the workspace but its metadata. If the
  name was already claimed when phase 9 opened, this story records that and moves on
  rather than re-claiming it.
- **`worker`'s MSRV and `cargo deny` cost were priced on the way in, by
  `worker-binding-layer`** (`_storymap.md:48`, Deployment brief **DEPLOY-AC-04**,
  `_decomposition.md:718-720`, `:834-850`). This story inherits that outcome; if
  `worker` moved the floor, that is ADR-0029's amendment and it already exists
  (`.kb/decisions/0029-msrv-raised-to-1-97-1.md`). Discovering it here means an upstream
  story did not finish.
- **The crate-level `#![allow(clippy::todo)]` is documented to disappear with the last
  `todo!()`, not to outlive it** (`crates/happenstance-cloudflare/src/lib.rs:121-126`),
  and project DoD 3 is a grep for both. This story *verifies* that grep comes back
  empty; it does not delete the last `todo!()`. If it is still there, `worker-binding-layer`,
  `durable-object-write-path` or `durable-object-read-path` did not finish and this
  story stops rather than tidying up after them.
- **Nothing `[FROZEN]` is touched.** This story adds files, removes a manifest flag and
  rewrites documentation. It changes no clause, no maturity marker and no behaviour, so
  `cargo xtask spec-trace` should be green *before and after* with no citation moved
  (`discover.md`, Gate box 7).

### The persona-journey slice this realizes

`_storymap.md`'s backbone names three observers, and this story is the whole of
activity **E — *depend on the crate*, observed by the library consumer**
(`_storymap.md:30`). The consumer here is not running the suite and not reading
`xtask`; they are looking at a crate directory, a manifest and a front page, and
deciding whether this is a thing they can build on. That is the *only* journey in this
project whose observable output is prose, which is exactly why its failure mode is
editorial rather than mechanical.

**Do not flatten this to "the files are present."** The story's discover names the wrong
implementation precisely (`discover.md:89-103`): a crate that is publishable and *lies* —
`package-check` green, `ci` green, and the front page still opening with "Status: not
implemented … every body is a `todo!()`", or a README that says "passes the DCB
conformance suite" flat on a crate whose concurrency family is **not invoked at all**,
whose fixture may decline `REOPEN`, and whose positions are bounded by 2^53 rather than
2^64. Every check in the tree is green and the first thing a consumer meets is false.
The intent is *the crate tells the truth about itself to someone who has not read this
repository*, and "the three filenames exist" is a proper subset of that.

### The two detectors this story builds, and their honest limits

Neither is a conformance rule, and deliberately so: no store fails anything here, and a
rule asserting on prose is the decorative rule `CLAUDE.md`'s corollary forbids.

1. **The README's usage example is a doctest.** The workspace already has the
   mechanism and the reasoning: `#![cfg_attr(doctest, doc = include_str!("../README.md"))]`
   compiles every README code block while keeping the prose out of the rendered docs, and
   the comment above it says why — "a README example that does not compile is worse than
   no example: it is the first thing a reader tries" (`crates/happenstance/src/lib.rs:1-10`,
   D10; same line in `happenstance-core` and `happenstance-testkit`). This is also the
   one lever the Rust constitution names for making a documentation claim mechanically
   checkable (`standards/rust/70-rustdoc-obligations.md`).
   *Constraint that comes with it:* the doctest must be compiled by something the gate
   runs. If `worker` has left the crate unable to build for the host, the example is
   marked with the narrowest attribute that is honest and the reason is written into the
   spec's error conditions — never silently downgraded to a fenced block that nothing
   compiles.
2. **The capability statement is written in the run's own words**, so it can be diffed
   against the `workerd` output rather than trusted: the `SKIP <rule>: <reason>` lines
   the suite emits (`crates/happenstance-testkit/src/contract.rs:473-483`), the fixture's
   declared `Capability` reasons from `measured-store-limits`, and the documented
   non-invocation of `event_store_concurrency_conformance!` — whose bound is
   `F::Store: EventStore + Send` and which is `cfg`-ed off `wasm32`
   (`crates/happenstance-testkit/src/lib.rs:101-118`), from `every-rule-under-workerd`.

**What neither catches** is a true sentence that is narrower than it sounds. That is a
review obligation, and the standing requirement it answers to is the initiative's AC-08:
a published compliance claim names which implementations it was checked against
(`project.md`, **How this advances the initiative**).

### The doctest hazard nobody else will catch

The README doctest is compiled through `include_str!` from *inside the package*. The
repository README lives outside it and would not resolve once published — the workspace
already learned this and wrote it down (`crates/happenstance/src/lib.rs:6-9`). Include
**only this crate's own README**, and confirm it by looking at the packaging list, which
this story is already running.

## Integration contract

- **Archetype**: `capability` — the library consumer's slice through manifest, files,
  front page and gate. Not a foundation: nothing downstream in this project consumes it.
- **Slice / milestone**: `publish-readiness`. **Slice-mates: none** — this story is the
  whole milestone (`_storymap.md:58`, **Merge order** §5). It is merged last so the final
  green gate is observed against a tree whose atoms are already accepted and whose
  `redkiln validate --kb` is clean.
- **Mount point**: `xtask/src/package.rs` — the `PUBLISHABLE` const at `:86`,
  reconciled against `cargo metadata` by `reconcile` at `:172-218` and executed by
  `run` at `:103-161`. This is the real composition root for "this crate is one of the
  crates we ship": adding the name is what makes the crate's licences and README a thing
  the gate holds true forever rather than a fact about today's working tree. The
  manifest half of the same mount is `crates/happenstance-cloudflare/Cargo.toml:12`
  (`publish = false`, deleted). **Neither is reachable only through a test** — both are
  read by the `packaged artifacts carry their licences and README` step in `REQUIRED`
  (`xtask/src/main.rs:515-532`), which is to say by every `cargo xtask ci` and every
  `cargo xtask ci --fast`.
- **Wires into** (real siblings, by path):
  - `xtask/src/package.rs:86` `PUBLISHABLE`, `:94` `REQUIRED_FILES`, `:226-241`
    `publishable_members` — the derived/declared pair whose disagreement names *which
    one moved*.
  - `xtask/src/main.rs:515-532` — the gate step that runs it; `:784-791` `wasm_steps()`,
    which this story reads to enumerate "every `wasm32` step" by name rather than by count.
  - `xtask/src/reserve.rs:92-97` — the `RESERVABLE` row for `happenstance-cloudflare`
    (phase 9) and the description the placeholder and the real crate must share;
    `:158-165` — licences copied into the generated crate, not linked.
  - `Cargo.toml:5-14` — the `[workspace.package]` inheritance the manifest already uses
    for `license`, `repository`, `keywords` and `categories`; only `description` and
    `readme` are per-crate here.
  - `crates/happenstance-core/Cargo.toml:12-15` — the precedent for stating
    `readme = "README.md"` rather than relying on auto-discovery, with the reason.
  - `crates/happenstance/src/lib.rs:1-10` — the `cfg(doctest)` README-include pattern.
  - `crates/happenstance-testkit/README.md:8-22` — the precedent for a README whose
    status paragraph is honest about what has *not* happened.
  - The `workerd` run's output and the fixture's `Capability` reasons, produced by
    `every-rule-under-workerd` and `measured-store-limits` (`depends_on`).
- **Renders surfaces**: **none.** `_design.md` records no user-facing surface for this
  project and that determination is what was approved on 2026-08-12; its `## Items` and
  `## Signatures` blocks are `N/A`. This story therefore claims no design item and adds
  no public Rust item — its output is three files, one manifest edit, one const entry and
  a rustdoc rewrite.
- **Conformance rule(s)**: **none, and the reason is structural.** No store fails
  anything here and no port changes, so there is nothing adapter-observable to add a rule
  for; a rule asserting on prose would be exactly the decorative rule `CLAUDE.md` forbids
  (`discover.md:105-119`). The two mechanisms that *do* fail are the packaging assertion
  (`xtask/src/package.rs`) and the README doctest — both already in the gate.
- **Clause(s)**: **none discharged and none amended.** `cargo xtask spec-trace` must stay
  green with no citation moved.
- **Advances DoD scenario**: initiative **DoD 10 — "The published crate looks
  finished"** (`initiative.md:388-390`): licence, description and README as rendered. This
  story makes DoD 10 *reachable* for this crate — the observation itself belongs to
  `closeout-and-durable-audience` (HS-P0019), the only project wired to the terminal
  `verify.e2e` grain. It also holds **DoD 13 — "the gate is green on the assembled
  whole"** by being the point at which `cargo xtask ci` is run against the finished
  project, including the `wasm32` execution step `wasm-execution-seam` added.

## PR boundary

```
crates/happenstance-cloudflare/Cargo.toml
crates/happenstance-cloudflare/LICENSE-MIT
crates/happenstance-cloudflare/LICENSE-APACHE
crates/happenstance-cloudflare/README.md
crates/happenstance-cloudflare/src/lib.rs
xtask/src/package.rs
RUNBOOK.md
.bklg/from-contract-to-published-library/cloudflare-durable-object-store/publish-ready-crate/**
```

`RUNBOOK.md` is in the list for exactly one line: the phase-9 exit box at `:4301`
(*"`publish = false` removed; `cargo xtask ci` green including the wasm32 step"*). The
ES-32 ledger paragraph in the same phase is `deferral-re-reads-and-es-32-verdict`'s and
must not be written here. `xtask/src/package.rs` is in the list for one const entry;
its scanner, its reconcile logic and its tests are not this story's to change.

**In this PR**

- `LICENSE-MIT` and `LICENSE-APACHE` copied into the crate directory from the repository
  root, byte-identical.
- `README.md` written: what the crate is, how it is used (as a compiled doctest), and a
  conformance claim carrying the run's own qualifiers.
- `Cargo.toml`: `publish = false` deleted; `readme = "README.md"` stated; `description`
  corrected so it no longer says "Not yet implemented." and matches
  `xtask/src/reserve.rs:94`; `[package.metadata.docs.rs]` declared to match what the
  crate actually builds on.
- `src/lib.rs`: the `# Status: not implemented` section replaced. The four findings, the
  two capability limits and the `not_send_probe` module and its tests (`:137-259`) survive
  untouched.
- `xtask/src/package.rs:86`: `"happenstance-cloudflare"` added to `PUBLISHABLE`.
- The crates.io name reservation performed out of tree via
  `cargo xtask reserve happenstance-cloudflare`, its `--dry-run` and its publish recorded
  in the implementation report.
- `RUNBOOK.md:4301` ticked.

**Explicitly not in this PR**

- Publishing `happenstance-cloudflare` at any real version, choosing a version, the
  registry landing page, the MSRV promise, the semver diff, the clause-ledger audit →
  `publication-and-positioning` (HS-P0016) (`project.md`, **Out of scope**;
  `_decomposition.md:852-866`).
- Any adapter body, fixture, capability constant or conformance target — those are
  `worker-binding-layer`, `durable-object-*`, `durable-object-host-and-fixture`,
  `every-rule-under-workerd` and `measured-store-limits`. Deleting a surviving `todo!()`
  here is scope drift *and* a signal that an upstream story is unfinished.
- Any `.kb/` atom, including ADR-0023 → `adr-0023-and-atom-resolutions` owns every KB
  write in this project (`_storymap.md:94-102`).
- Any new or changed `xtask` `Step`, including the `wasm32` execution step →
  `wasm-execution-gate-step`.
- Promoting any *other* skeleton crate. `PUBLISHABLE` gains exactly one name.

**Merge DoD.** `crates/happenstance-cloudflare/` carries both licence files and a README,
`cargo xtask package-check` names the crate in its "publishable set agrees with the
manifests" line and lists all three files as packaged, the crate's front page no longer
claims it is unimplemented, the name is reserved through the `0.0.0` placeholder, and
`cargo xtask ci` is green on a clean checkout including every step `wasm_steps()` names.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| -------------------- | ------- | ------------- |
| The three files sit **inside** the package directory | Cargo packages only what lives inside the package directory, does not follow a path outside it, and does not warn when `license = "MIT OR Apache-2.0"` is backed by no licence text — the omission is invisible until someone unpacks the tarball. Copied, never linked, for the same reason `reserve.rs` copies them | `xtask/src/package.rs:1-23`; `xtask/src/reserve.rs:158-165`; `crates/happenstance-core/`, `crates/happenstance/`, `crates/happenstance-testkit/` as the three existing exemplars |
| The manifest flag and the const entry move **together** | `publish = false` deleted from `Cargo.toml:12`; `"happenstance-cloudflare"` added to `PUBLISHABLE`. `reconcile` fails in *both* directions and its message names which artefact moved: a crate Cargo will publish that the step does not check, versus a name in the list Cargo will not publish | `crates/happenstance-cloudflare/Cargo.toml:12`; `xtask/src/package.rs:86`, `:172-218`; the derived-vs-declared argument at `:24-42`; `RUNBOOK.md:904-906` |
| The packaging assertion now covers a fourth crate | `cargo package -p happenstance-cloudflare --list --allow-dirty --locked` lists `LICENSE-MIT`, `LICENSE-APACHE`, `README.md`. `--list` does not build, so a `wasm32`-oriented crate is listable on any host; `--allow-dirty` is required because the gate must run on an uncommitted tree | `xtask/src/package.rs:69-74`, `:110-149`; gate step at `xtask/src/main.rs:515-532` |
| `readme = "README.md"` is stated, not auto-discovered | Cargo would find it anyway, but the key is what crates.io renders, and a silent default is a poor thing to rely on for a first impression. Same reasoning, verbatim precedent | `crates/happenstance-core/Cargo.toml:12-15` |
| The manifest `description` stops being false | Today: *"…Not yet implemented."* (`Cargo.toml:3`). It becomes the description the placeholder already promises — *"Cloudflare Durable Object event store adapter for happenstance."* — so the reservation and the eventual release describe the same thing to anyone browsing | `crates/happenstance-cloudflare/Cargo.toml:3`; `xtask/src/reserve.rs:41-43`, `:92-97` |
| The crate's front page stops saying it is unimplemented | `# Status: not implemented` and "every body is `todo!()`" are the first two hundred characters docs.rs renders. Replaced by what the crate now is. The four findings (`:31-95`), the two capability limits (`:96-113`) and the `Targets` note (`:114-119`) are retained and updated where the bindings changed them; the `not_send_probe` module and its four tests (`:137-259`) are untouched | `crates/happenstance-cloudflare/src/lib.rs:1-135`; `_decomposition.md:102` ("crate docs rewritten from 'not implemented'") |
| The README's usage example is compiled | `#![cfg_attr(doctest, doc = include_str!("../README.md"))]` on the crate root: every README code block is type-checked, the prose stays out of the rendered docs, and only *this crate's own* README is included because the repository README would not resolve once published | `crates/happenstance/src/lib.rs:1-10`; `crates/happenstance-core/src/lib.rs:7`; `crates/happenstance-testkit/src/lib.rs:7`; `standards/rust/70-rustdoc-obligations.md` |
| The doctest contains **no literal position values** | A doctest printing `[1, 2, 3]` as expected output would publish the literal-position assumption the repository forbids, on the most-read page the crate has. Positions in any example are compared against what the store assigned | `CLAUDE.md`, *The rule that matters*; `discover.md:137-141` |
| The conformance claim carries the run's own qualifiers | Not "passes the DCB conformance suite" flat. It names: the event-store family as what was run; the `event_store_concurrency_conformance!` family as a **documented non-invocation** with its reason (`F::Store: EventStore + Send`, `cfg`-ed off `wasm32`); each declined `Capability` in the fixture's own words; and the 2^53 position ceiling as a declared store limit. Written so it can be diffed against the run's `SKIP <rule>: <reason>` lines | `crates/happenstance-testkit/src/lib.rs:101-118`; `crates/happenstance-testkit/src/contract.rs:473-483`; `crates/happenstance-cloudflare/src/lib.rs:96-113`; `project.md` AC-003; `initiative.md` AC-08 via `project.md`, **How this advances the initiative** |
| The name is reserved by the `0.0.0` placeholder | `cargo xtask reserve happenstance-cloudflare` writes `target/reserve/happenstance-cloudflare/` with both licences, a README that says plainly it contains no functionality, and a `[workspace]` table making it its own workspace. Verified with `cargo publish --manifest-path … --dry-run`, then published. **Never** the real crate at a real version: that would cascade version pins through every dependent manifest, or freeze the API before the contract-freeze phases | `xtask/src/reserve.rs:12-30`, `:130-191`, `:214-269`; `RUNBOOK.md:4254-4261`, `:4301`; `_decomposition.md:852-866` |
| Reservation timing is decoupled from this diff | The placeholder shares nothing with the workspace but its metadata, so the claim may already have happened at phase start. This story records the state it found and does not block on a registry round trip; a re-claim of an already-owned name is not attempted | `RUNBOOK.md:4261`; `discover.md:35-40` |
| docs.rs builds something legible | `[package.metadata.docs.rs]` is declared to match what the crate actually builds on after `worker` landed, following the two existing blocks in the workspace. A crate whose docs.rs build fails is a registry page that renders an error where the front page should be | `crates/happenstance-core/Cargo.toml:54-56`; `crates/happenstance-testkit/Cargo.toml:56`; `initiative.md` DoD 10 |
| "Every `wasm32` step", enumerated by name | The claim is checked against `wasm_steps()`' `steps_named([…])` list as it stands at merge — four today, five once `wasm-execution-gate-step` merges — never against a remembered count. An index is silent about what it selects, which is the defect that file already names | `xtask/src/main.rs:769-791`; `_storymap.md:162-165`; `project.md` AC-012 |
| The `todo!()` grep comes back empty | Project DoD 3 greps `crates/happenstance-cloudflare/` for `todo!()` and for `#![allow(clippy::todo)]` (`src/lib.rs:121-126`). This story **verifies** both, and halts rather than deleting a survivor — a survivor means an upstream story in `real-worker-bindings` did not finish | `project.md`, **Definition of done** 3; `crates/happenstance-cloudflare/src/lib.rs:121-126`; `discover.md:27` |
| The gate is run at both grains | `cargo xtask affected --base main` at the story grain and `cargo xtask ci` at the end. `--fast` is the bar this non-terminal project's integration gate holds, but AC-012's claim is about the full gate and this story runs it | `CLAUDE.md`, **Commands**; `_decomposition.md:649-666`; `xtask/src/main.rs:835-860` |

## Data and migrations

**N/A — no runtime data and no schema change.** Three things that could be mistaken for
migrations, each named so the absence is stated rather than skipped:

1. **The Durable Object schema is not touched here.** `CloudflareEventStore::migrate`
   is schema *creation* for a fresh Durable Object, not a backfill — nothing has ever
   written through this adapter because it has never run, so there is no prior schema
   version to migrate away from. Its implementation belongs to
   `durable-object-write-path` (`_decomposition.md:739-742`, `_storymap.md:49`).
2. **The one irreversible state transition here is a registry publish, not a database
   one.** `cargo publish` of the `0.0.0` placeholder cannot be undone — a version can be
   yanked, never removed (`xtask/src/reserve.rs:181`). The mitigation is the mechanism
   itself: the placeholder shares nothing with the workspace but its metadata, states in
   its own README that it contains no functionality, and carries both licence files, so
   the irreversible artefact is one nobody can be harmed by depending on. The reverse
   direction — restoring `publish = false` — is a one-line manifest edit plus the
   matching `PUBLISHABLE` removal, and `reconcile` refuses to let the pair drift.
3. **KB "rollback" is not a revert.** Nothing under `.kb/` is written by this story, but
   for the record: reversing an accepted atom is a new superseding atom, never an edit to
   the old one's body, and `redkiln validate --kb` checks accepted atoms against `HEAD`
   (`CLAUDE.md`, **Where the work lives**; `_decomposition.md:753-757`).

No feature flag and no config gating: `happenstance-cloudflare` has no Cargo features
today and this story adds none (`_decomposition.md:731-738`).

## Acceptance criteria

Framed from the intent of the two personas this slice serves, both carried from
`initiative.md`, **Referenced personas & journeys** (`:227-250`): the
**constrained-runtime developer** on *event-source at the edge without hand-rolling it*,
who arrives at this crate directory because their platform is Workers and needs to know
whether this is a dependency or a lab notebook; and the **evaluator** on *decide in one
sitting*, whose whole method is a bounded look at public artefacts ending in adopt or
decline for a stated reason. `_storymap.md:30` names the observer of activity **E** the
*library consumer*; these are the two shapes that consumer takes.

Every criterion crosses the full stack of what this story has: the manifest, the files
on disk, the packaged tarball, the rendered front page, and the gate that keeps each
true. None of them is satisfied by "the file exists".

| id | criterion | verification |
| -- | --------- | ------------ |
| AC-001 | **GIVEN** an evaluator whose adoption question includes "may my employer use this", **WHEN** they unpack the crate's published artifact — not the repository — **THEN** both `LICENSE-MIT` and `LICENSE-APACHE` are inside it, byte-identical to the repository root's copies, because `license = "MIT OR Apache-2.0"` is a choice the consumer makes and a choice needs both texts to be makeable (`xtask/src/package.rs:88-94`) | `cargo package -p happenstance-cloudflare --list --allow-dirty --locked` lists both, run by the `packaged artifacts carry their licences and README` step (`xtask/src/main.rs:519`, dispatching `xtask/src/package.rs:103-161`); byte-identity by `git diff --no-index LICENSE-MIT crates/happenstance-cloudflare/LICENSE-MIT` (and the Apache pair) reporting no difference |
| AC-002 | **GIVEN** the constrained-runtime developer landing on the crate's registry page as their first contact, **WHEN** the page renders, **THEN** a `README.md` that belongs to *this* crate is there to render — present beside `Cargo.toml`, named explicitly by `readme = "README.md"` rather than left to auto-discovery, and inside the packaged artifact, because a `readme` key pointing outside the artifact fails silently (`xtask/src/package.rs:90-94`; precedent and its reason at `crates/happenstance-core/Cargo.toml:12-15`) | the same `cargo package --list` assertion lists `README.md`; `crates/happenstance-cloudflare/Cargo.toml` carries `readme = "README.md"` |
| AC-003 | **GIVEN** a maintainer who will later ship this crate, **WHEN** `publish = false` is deleted from `crates/happenstance-cloudflare/Cargo.toml:12`, **THEN** `"happenstance-cloudflare"` is added to `PUBLISHABLE` in the same change, so the crate never exists in the state "Cargo will publish it and nothing checks it" — `reconcile` fails in both directions and names which artefact moved (`xtask/src/package.rs:172-218`) | `cargo xtask package-check` exits zero; its `publishable set agrees with the manifests:` line (`xtask/src/package.rs:210-214`) names four crates including `happenstance-cloudflare`. Failure of either half alone is EC-001 / EC-002 |
| AC-004 | **GIVEN** a future contributor who adds a file to this crate a year from now, **WHEN** they run the gate, **THEN** the licence-and-README guarantee is held for `happenstance-cloudflare` by the same mechanism that holds it for the other three — not by a fact about today's working tree — because the crate is now inside `REQUIRED`'s packaging step and therefore inside every `cargo xtask ci` *and* every `cargo xtask ci --fast` (`xtask/src/main.rs:515-532`, `:853-860`) | `cargo xtask ci --fast` green on a tree where one of the three files has been temporarily removed **fails**, naming the missing filename (`xtask/src/package.rs:150-161`); restored, it passes. The negative half is run once during implementation and recorded in the implementation report |
| AC-005 | **GIVEN** the evaluator scanning search results, **WHEN** they read the one-line description crates.io shows beside the name, **THEN** it does not say *"Not yet implemented."* (`crates/happenstance-cloudflare/Cargo.toml:3`) and it is the same sentence the reserved placeholder already promises — *"Cloudflare Durable Object event store adapter for happenstance."* (`xtask/src/reserve.rs:93`) — so the reservation and the crate describe one thing, not two | read the manifest `description` against `xtask/src/reserve.rs:93`; the string appears in the `cargo package --list` run's manifest and in `cargo publish --dry-run` output for the placeholder |
| AC-006 | **GIVEN** the constrained-runtime developer following the docs.rs link, **WHEN** the front page renders, **THEN** the first thing they read is what the crate *is*, not `# Status: not implemented … every body is a todo!()` (`crates/happenstance-cloudflare/src/lib.rs:4-10`), and the page renders at all — `[package.metadata.docs.rs]` is declared to match what the crate actually builds on after `worker` landed, following `crates/happenstance-core/Cargo.toml:54-56`. The four findings, the two capability limits and the `Targets` note survive, updated where the bindings changed them | the `docs` step of `cargo xtask ci` green with `RUSTDOCFLAGS=-D warnings`; the nightly `--cfg docsrs` `OPTIONAL` step green where the toolchain is present; `rg -n "Status: not implemented" crates/happenstance-cloudflare/` returns nothing |
| AC-007 | **GIVEN** the constrained-runtime developer doing the one thing every reader does — copying the README's example into their Worker — **WHEN** they paste it, **THEN** it compiles, because it was compiled by the gate: `#![cfg_attr(doctest, doc = include_str!("../README.md"))]` on the crate root type-checks every README code block while keeping the prose out of the rendered docs, and includes **only this crate's own README** because the repository README would not resolve once published (`crates/happenstance/src/lib.rs:1-10`) | `cargo test -p happenstance-cloudflare --doc` compiles and runs the README's blocks; the attribute is present at `crates/happenstance-cloudflare/src/lib.rs`; `cargo package --list` confirms the included path is inside the package. Degradation path is EC-004 |
| AC-008 | **GIVEN** an adapter author who learns this repository's conventions from its most-read page, **WHEN** they read the README example, **THEN** it teaches them nothing false about positions: no assertion compares against a literal position value such as `[1, 2, 3]`, because the specification permits gaps and a conformant adapter may leave them — every position in the example is bound from what the store actually assigned (`CLAUDE.md`, *The rule that matters*; `discover.md:137-141`) | `cargo test -p happenstance-cloudflare --doc` passes with assertions written against bindings returned by `append`/`head`; reviewer grep for literal position arrays and bare numeric position comparisons in `crates/happenstance-cloudflare/README.md` returns nothing. The mechanical half is the doctest running; the "no literal" half is a review obligation and is stated as such |
| AC-009 | **GIVEN** the evaluator whose entire method is checking a compliance claim instead of trusting it (`initiative.md:330-332`, AC-08), **WHEN** they read the README's conformance sentence, **THEN** it is not "passes the DCB conformance suite" flat: it names the event-store family as what was run, the `event_store_concurrency_conformance!` family as a **documented non-invocation** with its reason (`F::Store: EventStore + Send`, `cfg`-ed off `wasm32` — `crates/happenstance-testkit/src/lib.rs:101-118`), each declined `Capability` in the fixture's own words, and the 2^53 position ceiling as a declared store limit — written in the same words the run prints, so it can be diffed rather than believed | diff the README's qualifier list against the `SKIP <rule>: <reason>` lines the `workerd` execution step emits (`crates/happenstance-testkit/src/contract.rs:473-483`) and against `CloudflareFixture`'s `Capability` constants from `measured-store-limits`; every qualifier in the README appears in the run's output or in a fixture constant, and every declined capability in the run appears in the README. Reviewed against the diff, recorded in the implementation report |
| AC-010 | **GIVEN** the maintainer protecting the project's own name, **WHEN** phase 9 is under way, **THEN** `happenstance-cloudflare` on crates.io is held by the documented `0.0.0` placeholder — a standalone crate sharing nothing with the workspace but its metadata, carrying both licence texts and a README that says plainly it contains no functionality — and **never** by the real crate at a real version, which would cascade version pins through every dependent manifest or freeze the API before the contract-freeze phases (`xtask/src/reserve.rs:12-30`). If the name was already claimed when the phase opened, that state is recorded and not re-claimed | `cargo xtask reserve happenstance-cloudflare` writes `target/reserve/happenstance-cloudflare/`; `cargo publish --manifest-path target/reserve/happenstance-cloudflare/Cargo.toml --dry-run` green; the publish (or the already-held finding) recorded verbatim in the implementation report. Out of tree by construction — `target/` is not in the PR boundary |
| AC-011 | **GIVEN** the adapter author who was promised a real adapter by this project, **WHEN** they grep the crate before depending on it, **THEN** neither `todo!()` nor `#![allow(clippy::todo)]` is left — the scoped allow is documented to disappear *with* the last `todo!()` rather than outlive it (`crates/happenstance-cloudflare/src/lib.rs:122-126`) — and this story **verifies** that rather than causing it: a survivor means `worker-binding-layer`, `durable-object-write-path` or `durable-object-read-path` did not finish, and this story halts (EC-006) | `rg -n "todo!\(\)" crates/happenstance-cloudflare/` and `rg -n "allow\(clippy::todo\)" crates/happenstance-cloudflare/` both return nothing; independently, `cargo clippy --workspace --all-targets --all-features -- -D warnings` fails on any `todo!()` once the scoped allow is gone, which is the standing detector (`project.md`, **Definition of done** 3) |
| AC-012 | **GIVEN** the maintainer about to hand this project to `publication-and-positioning`, **WHEN** the full gate is run on a clean checkout of the merged tree, **THEN** `cargo xtask ci` is green including **every** step `wasm_steps()` names — enumerated by reading `steps_named([…])` at `xtask/src/main.rs:784-791` at merge time, four today and five once `wasm-execution-gate-step` has landed, never against a remembered count — and `cargo xtask spec-trace` is green with no citation moved | one `cargo xtask ci` run on a clean checkout, its step list transcribed into the implementation report and checked name-by-name against `wasm_steps()`; `cargo xtask affected --base main` green at the story grain during implementation (`xtask/src/main.rs:835-860`) |

**Coverage of the traced project AC.** This story traces to exactly one:
**AC-012 — the crate is publish-ready and the gate is green** (`project.md:243`). It is
covered by AC-001…AC-012 above as a whole, and specifically: publish-readiness by
AC-001…AC-007 and AC-010, the truthfulness that makes "ready" mean something by
AC-005, AC-006, AC-008, AC-009 and AC-011, and the green-gate half by AC-004 and AC-012.
The testing brief maps project AC-012 to the **static** tier and names
`cargo xtask package-check` plus `cargo xtask ci` as its gate commands
(`_decomposition.md:527`); every row above resolves to one of those two, a doctest, or a
stated review obligation.

## Interaction quality

RFC §6.7/D6. **This story renders no surface.** `_design.md`'s `## Surfaces` section is
`N/A — no user-facing surface`, approved 2026-08-12 against this project's scope, and its
`## Items` and `## Signatures` blocks are `N/A`. The **composition family is therefore not
applicable** — there is no design-system primitive to compose, no density budget, no
persistent-versus-revealed chrome and no named anti-pattern to avoid, because there is
nothing to place. Asserting a composition invariant here would be inventing a design the
signed-off document deliberately declined to have, which is exactly the free-residual
failure `_design.md` exists to close in the other direction.

What this story does have is the one artefact in the whole project that a human *reads
rather than runs*: the crate's rendered front page and registry landing page. The
state-family invariants map onto it in a real, non-metaphorical way, and each one that
applies is already an AC row in the table above. This section only says which row carries
which invariant and how each is verified — no invariant is stated here as a prose bullet,
because `redkiln verify` extracts ACs from table cells and bullets in this section would
never be gated.

**State family — which AC carries it**

| Invariant | Carried by | How it is verified |
| --------- | ---------- | ------------------ |
| **Presentation exists at all** — the crate's front matter is composed documentation, not raw status notes left over from a skeleton. The rendered page is what is checked, not the source | **AC-006** | the `docs` step with `RUSTDOCFLAGS=-D warnings`, plus the nightly `--cfg docsrs` build; `rg` for the not-implemented header |
| **Non-occlusion** — nothing false sits in front of the thing the reader came for. `# Status: not implemented` occupies the first two hundred characters docs.rs renders and occludes every true sentence behind it | **AC-006** | as above; the replacement is read, not merely diffed |
| **In-place, not context-jump** — the reader is not sent elsewhere to learn what the crate is or to see it used. The usage example lives on the page they are already on, and it is the compiled one | **AC-007** | `cargo test -p happenstance-cloudflare --doc` |
| **Reversibility** — every state this story enters can be left. The manifest half reverses by restoring `publish = false` plus the matching `PUBLISHABLE` removal, and `reconcile` refuses to let the pair drift apart while it happens | **AC-003** | `cargo xtask package-check` after each direction; the drift failure is EC-001 / EC-002 |
| **Irreversibility, isolated and declared** — the one step that cannot be undone is a registry publish, and it is deliberately confined to an artefact nobody can be harmed by depending on: metadata only, both licences, a README that says it contains no functionality | **AC-010** | `--dry-run` before the publish; the placeholder's own generated README (`xtask/src/reserve.rs:130-191`) |
| **Preserved context on a partial change** — the reader's existing knowledge of this crate survives the rewrite: the four findings, the two capability limits, the `Targets` note and the `not_send_probe` module are retained rather than swept away with the status header | **AC-006** | the diff is reviewed for retention, not only for removal; the probe module's four tests still run under `cargo test -p happenstance-cloudflare` |
| **The claim is checkable, not merely stated** — the reader can verify the compliance sentence from public output instead of trusting it | **AC-009** | diff against the `workerd` run's `SKIP` lines and the fixture's `Capability` constants |

**Keyboard reachability, focus/scroll/selection preservation, transience policy, density
budget, hierarchy, and the design's named anti-patterns**: **N/A**, and not by oversight —
there is no focusable control, no scroll container and no revealed chrome in a Cargo
manifest, a licence text or a rustdoc page rendered by docs.rs. `_design.md`'s own
determination is the authority for that, and it is cited above rather than re-derived.

## Error conditions

| id | condition | required behaviour |
| -- | --------- | ------------------ |
| EC-001 | `publish = false` is deleted but `"happenstance-cloudflare"` is not added to `PUBLISHABLE` | `reconcile` fails the gate with *"Cargo will publish happenstance-cloudflare but this step does not check it…"* (`xtask/src/package.rs:191-200`). This is correct behaviour and must not be worked around by re-adding the flag — the two halves land together (AC-003) |
| EC-002 | The name is added to `PUBLISHABLE` while `publish = false` survives | `reconcile` fails with *"PUBLISHABLE names … but Cargo will not publish it"* (`xtask/src/package.rs:202-208`). Same remedy: land both halves |
| EC-003 | The crate is publishable and one of `LICENSE-MIT`, `LICENSE-APACHE`, `README.md` is not inside the packaged artifact | `run` collects and reports the missing filenames per crate and fails (`xtask/src/package.rs:150-161`). The fix is to copy the file **into the crate directory** — never to link it, never to add it to `.gitignore`'s exceptions, and never to drop the crate back out of `PUBLISHABLE` |
| EC-004 | After `worker` landed, `cargo test -p happenstance-cloudflare --doc` cannot link on the host, so the README doctest cannot be compiled by an ordinary gate run | Mark the example with the **narrowest attribute that is honest** (`no_run` if it type-checks but cannot link; `ignore` only as a last resort and never silently), write the reason into the README and into the implementation report, and record which target *does* compile it. **Never** downgrade the example to a fenced block that nothing compiles — that discards AC-007's entire mechanism. The testing brief already flags this target question as open and unsettled at implementation grain (`_decomposition.md:587-616`) |
| EC-005 | The crates.io name is already claimed | If claimed by this project's earlier phase-start reservation: record the state found and proceed — a re-claim is not attempted and the diff does not block on a registry round trip (`RUNBOOK.md:4261`). If claimed by a third party: **halt and escalate**. Renaming the crate is not this story's decision, it is not this project's, and it invalidates `xtask/src/reserve.rs`'s `RESERVABLE` row and `RUNBOOK.md`'s phase plan |
| EC-006 | `rg` finds a surviving `todo!()` or the scoped `#![allow(clippy::todo)]` in `crates/happenstance-cloudflare/` | **Halt.** Do not delete the survivor and do not remove the allow. A survivor means an upstream story in `real-worker-bindings` did not finish; deleting it here is scope drift *and* it destroys the signal (`project.md`, **Definition of done** 3) |
| EC-007 | `cargo publish --manifest-path target/reserve/happenstance-cloudflare/Cargo.toml --dry-run` fails | Do not publish. Diagnose against `xtask/src/reserve.rs:130-191` and record. The reservation is decoupled from this diff, so a failure here does not block the rest of the story — it blocks only AC-010 |
| EC-008 | A step named in `wasm_steps()` is red, or `steps_named` panics because a name it selects no longer exists | AC-012's claim is not made and the story does not merge. `steps_named` panicking is the designed failure — the steps are compile-time constants, so a miss is a bug in `xtask/src/main.rs` and never a user error (`:769-791`). A red `wasm32` step belongs to whichever story owns it, not to this one |
| EC-009 | `[package.metadata.docs.rs]` declares a target or feature set the crate cannot build on, so the registry page renders a build error where the front page should be | Declare only what the crate is observed to build on after `worker` landed, following `crates/happenstance-core/Cargo.toml:54-56` and `crates/happenstance-testkit/Cargo.toml:56`. Verified by the nightly `--cfg docsrs` `OPTIONAL` step locally; a green local `docs` step alone does not prove the docs.rs configuration |
| EC-010 | `cargo xtask spec-trace` goes red, or a citation's line range moves | Stop. This story changes no clause and touches nothing `[FROZEN]`; a red `spec-trace` means the diff reached further than the PR boundary states, or `RUNBOOK.md:4301`'s edit disturbed a cited range (`discover.md:143-147`) |

## Non-functional

| id | requirement | why, and how it is held |
| -- | ----------- | ----------------------- |
| NF-001 | The two licence texts are **byte-identical copies** of the repository root's, placed inside the crate directory | Cargo packages only what lives inside the package directory and will not follow a path outside it; a symlink or a "see the repository root" pointer is exactly defect D11, which shipped licence metadata with no licence text (`RUNBOOK.md:898-906`). `xtask/src/reserve.rs:158-165` copies for the same stated reason |
| NF-002 | The packaging step's cost rises by one `cargo package --list` invocation, and no more | `--list` does not build, so a `wasm32`-oriented crate is listable on any host and the step stays host-agnostic. `--allow-dirty` remains required because the gate must run on an uncommitted tree (`xtask/src/package.rs:69-74`). No new process launch beyond the fourth `--list` |
| NF-003 | This story adds **no dependency and no Cargo feature** to `happenstance-cloudflare` | `cargo deny`'s licence/advisory surface and the `cargo hack` feature powerset are therefore unchanged by *this* diff. `worker`'s cost was priced by `worker-binding-layer` against DEPLOY-AC-04 (`_decomposition.md:834-850`); discovering it here means an upstream story did not finish |
| NF-004 | The front page's first paragraph earns its place | It is what docs.rs and crates.io render above the fold, and it is the only artefact in this project a human reads rather than runs. Concretely: the opening states what the crate is and which runtime it targets before any caveat, and every qualifier that follows is one of AC-009's — not new prose invented here |
| NF-005 | `cargo xtask spec-trace` is green **before and after**, with no citation moved | This story changes no clause, no maturity marker and no behaviour (`discover.md:143-147`). A moved citation is a signal the diff left its boundary |
| NF-006 | The MSRV is neither raised nor promised | Raising it is ADR-0029's business and would need a new ADR amending it (`.kb/decisions/0029-msrv-raised-to-1-97-1.md`; `CLAUDE.md`, binding constraint 5). *Promising* one in the README is `publication-and-positioning`'s (`project.md`, **Out of scope**). Neither is done here, and the README contains no sentence that commits a minimum supported Rust version |
| NF-007 | The full `cargo xtask ci` is run once, on a clean checkout, at the end | `--fast` is the bar `verify.integration_scoped` holds this non-terminal project to (`_decomposition.md:649-666`), but AC-012's claim is about the *full* gate, and a claim about the full gate is only true if the full gate was run (`xtask/src/main.rs:835-860`) |

## Implementation notes (non-prescriptive)

Directional, not binding. Where these disagree with an AC or a cited path, the AC wins.

- **Land the manifest flag and the `PUBLISHABLE` entry in the same commit as the three
  files.** Any intermediate commit that has one half is a commit at which the gate is red,
  and `cargo xtask affected --base main` will say so at the story grain. The coupling is
  the mount, not a chore (`Executive summary`, delta 1).
- **Copy the licences first, then run `cargo xtask package-check` alone** before touching
  the manifest. That gives a clean "the files are in place" observation with the crate
  still outside the set, so when the step goes red after the flag is deleted the message
  can only be about the const entry.
- **Write the README backwards from AC-009's qualifier list.** Run the `workerd`
  execution step with output shown first, capture its `SKIP <rule>: <reason>` lines and the
  fixture's `Capability` constants, and write the conformance paragraph from that transcript
  rather than from memory of what the adapter does. The diff in AC-009 is then a diff
  against something that already exists, not a re-derivation.
- **The usage example is the smallest thing that is true**: construct the store over
  `SqlStorage`, append, read back, and bind every position from what the store returned.
  Resist the urge to demonstrate the whole DCB append-condition story — the example has to
  compile in the gate forever, and each extra line is another thing that can rot. Look at
  `crates/happenstance-testkit/README.md:8-22` for the register a status paragraph should
  be written in when something has *not* happened yet.
- **Check the `include_str!` path against the packaging list, not against the local
  filesystem.** `../README.md` from `src/lib.rs` resolves locally whether or not the file is
  inside the package; only `cargo package --list` distinguishes the two, and this story is
  already running it (`Context pack`, *The doctest hazard nobody else will catch*).
- **Do the reservation out of tree and early.** `cargo xtask reserve happenstance-cloudflare`
  writes into `target/`, which is not in the PR boundary; the `--dry-run` and the publish
  are recorded in the implementation report as transcripts. If the name is already held by
  this project, that recording is the whole of AC-010.
- **Run the negative half of AC-004 deliberately.** Temporarily remove one of the three
  files, observe the named failure, restore it. That is the only evidence that the crate is
  actually *inside* the check rather than merely adjacent to it, and it takes about a minute.
- **Read `wasm_steps()` at merge time, not from this spec.** The number in any prose here is
  four-or-five depending on whether `wasm-execution-gate-step` has landed; `steps_named` is
  the authority and it is deliberately name-based rather than index-based
  (`xtask/src/main.rs:769-791`).

## Tests and CI (merge gate)

Grounded in the project testing brief (`_decomposition.md:482-534`), which maps project
AC-012 to the **static** tier with `cargo xtask package-check` and `cargo xtask ci` as its
gate commands (`:527`). This story adds one artefact to a tier the brief does not enumerate
for AC-012 — the README doctest — because the brief's static tier cannot read prose and the
doctest is the one lever that makes a documentation claim mechanically checkable
(`standards/rust/70-rustdoc-obligations.md`).

| tier | command / path | proves |
| ---- | -------------- | ------ |
| **Static — packaging** | `cargo xtask package-check` → `xtask/src/package.rs:103-161`, dispatched by `xtask/src/main.rs:515-532` | AC-001, AC-002, AC-003, AC-004. Four crates named in the agreement line; three filenames listed inside each artifact. The negative run (one file removed) is what proves the coverage is real rather than incidental |
| **Static — packaging, unit** | `cargo test -p xtask` → `xtask/src/package.rs:409-459` | The scanner still reads all three `publish` spellings and ignores non-member `name` keys. **Pre-existing and unchanged by this story** — listed so it is not accidentally edited: adding a name to a `const` needs no new test there, and adding one would be the decorative test `CLAUDE.md`'s corollary forbids |
| **Static — lints and format** | `cargo clippy --workspace --all-targets --all-features -- -D warnings`; `cargo fmt --all --check` | AC-011's independent half: once the scoped `#![allow(clippy::todo)]` is gone, any surviving `todo!()` fails the gate outright (`_decomposition.md:516`) |
| **Static — grep (DoD 3)** | `rg -n "todo!\(\)" crates/happenstance-cloudflare/`; `rg -n "allow\(clippy::todo\)" crates/happenstance-cloudflare/` | AC-011. Both empty. A non-empty result is EC-006 and halts the story rather than being fixed here |
| **Static — front page** | `rg -n "Status: not implemented" crates/happenstance-cloudflare/` | AC-006's negative half: the header is gone from the source, not merely from the rendered page |
| **Docs** | the `docs` step of `cargo xtask ci` (`RUSTDOCFLAGS=-D warnings`), plus the nightly `--cfg docsrs` `OPTIONAL` step where the toolchain resolves | AC-006, EC-009. The page renders, under the feature set docs.rs will use |
| **Doctest** | `cargo test -p happenstance-cloudflare --doc`, driven by `#![cfg_attr(doctest, doc = include_str!("../README.md"))]` at `crates/happenstance-cloudflare/src/lib.rs` | AC-007, AC-008. The README's example compiles and runs; its position assertions are bound from what the store assigned. Degradation path is EC-004 and it is written down, never silent |
| **Inherited integration (not run by this story)** | the `workerd` execution step added by `wasm-execution-gate-step`, run by `every-rule-under-workerd` | AC-009's source material. This story *reads* its output to write the conformance claim and diffs the claim against it; it does not add, change or re-run a conformance rule |
| **Process gate** | `cargo xtask spec-trace`; `redkiln validate --kb && redkiln doctor` | NF-005, EC-010. Green before and after with no citation moved. No `.kb/` atom is written here — `adr-0023-and-atom-resolutions` owns every KB write in this project (`_storymap.md:94-102`) |
| **Registry (out of tree)** | `cargo xtask reserve happenstance-cloudflare`; `cargo publish --manifest-path target/reserve/happenstance-cloudflare/Cargo.toml --dry-run` | AC-010. Neither runs in CI and neither is in the PR boundary; both are recorded as transcripts in the implementation report |
| **Story grain** | `cargo xtask affected --base main` (`xtask/src/main.rs:835-860`) | The `redkiln advance` seam. Run on every commit of this story |
| **Merge gate** | `cargo xtask ci` — full, on a clean checkout, including every step `wasm_steps()` names (`xtask/src/main.rs:784-791`) | AC-012, and initiative DoD 13. `--fast` is the standing bar for this non-terminal project (`_decomposition.md:649-666`); the full run is what AC-012's claim is *about*, so it is run once here and its step list transcribed |

**Review obligations, stated because no runner covers them.** AC-008's "no literal position
value" (a doctest can be green and still teach the wrong thing) and AC-009's "true but
narrower than it sounds" (`discover.md:116-119`). Both are named in the implementation
report and checked in the adversarial review against the initiative's AC-08
(`initiative.md:330-332`).

## Risks and coupling (PR-scoped)

| risk | likelihood / impact | mitigation, inside this PR |
| ---- | ------------------- | -------------------------- |
| **The crate is publishable and lies** — every check green, the front page still saying "not implemented", or a flat "passes the DCB conformance suite" on a crate whose concurrency family was never invoked | Medium / High. It is the story's named wrong implementation (`discover.md:89-103`) and the packaging assertion reads no word of any file's contents | AC-006 and AC-009 exist for exactly this; the two detectors (the doctest, the diffable qualifier list) plus the stated review obligation are the whole answer, and their limits are written down rather than papered over |
| **Half the mount lands** — the flag deleted without the const entry, or the reverse | Medium / Low. Easy to do while iterating | `reconcile` fails in both directions with a message naming which artefact moved (`xtask/src/package.rs:172-218`); EC-001 and EC-002 record the correct response, which is to land the other half rather than revert |
| **The doctest cannot be compiled on the host after `worker` landed**, and gets quietly downgraded to a fenced block | Medium / High. The testing brief flags this target question as genuinely open (`_decomposition.md:587-616`) and the tempting fix is the one that destroys the mechanism | EC-004 forbids the silent downgrade and requires the narrowest honest attribute plus a written reason. If it bites, the story is still mergeable — the *record* of why is what AC-007 then carries |
| **An upstream story is unfinished and this story tidies up after it** — a surviving `todo!()` deleted here to get the gate green | Low / High. It would erase the signal DoD 3 exists to produce | EC-006 halts rather than fixes; the PR boundary names adapter bodies as explicitly not in this PR |
| **The registry round trip blocks the diff**, or a `--dry-run` failure stalls the merge | Low / Medium | AC-010 is decoupled by construction: the placeholder shares nothing with the workspace but its metadata, and `RUNBOOK.md:4261` claims names at phase start. EC-007 scopes a failure to AC-010 alone |
| **The name is held by a third party** | Low / High. It would invalidate `RESERVABLE`'s row and the phase plan | EC-005 escalates rather than renaming. A rename is neither this story's nor this project's decision |
| **`RUNBOOK.md` edits collide** — `deferral-re-reads-and-es-32-verdict` writes the ES-32 ledger paragraph in the same phase-9 section this story ticks a box in | Medium / Low | The PR boundary confines this story to the exit box at `RUNBOOK.md:4301` and says so explicitly. Merge order puts `evidence-and-verdicts` before `publish-readiness` (`_storymap.md:162-165`), so the paragraph exists before the box is ticked |
| **The green-gate claim is scoped to the wrong tree** — "all four `wasm32` steps" asserted against a tree that does not yet contain the execution step | Medium / High. AC-012's own wording predates the fifth step | AC-012 requires enumeration by name against `steps_named` at merge time; the merge-order note makes the higher bar the operative one (`_storymap.md:162-165`) |
| **Scope creep into `publication-and-positioning`** — a version, a landing page, an MSRV sentence written "while we are here" | Medium / Low | NF-006 and the PR boundary's *Explicitly not in this PR* list; `project.md`, **Out of scope** is the authority |
| **Another skeleton crate gets promoted in passing** because the same three files are obviously missing there too | Low / Medium | The PR boundary states `PUBLISHABLE` gains exactly one name. Each other skeleton has its own phase and its own `RESERVABLE` row |

## Dependencies

**Blocks on** (must be merged first; `depends_on` in the story item, `_storymap.md:58`):

| story slug | why this story cannot start without it |
| ---------- | -------------------------------------- |
| `worker-binding-layer` | There must be something real to depend on. It is also where `worker`'s MSRV and `cargo deny` cost were priced against DEPLOY-AC-04 (`_decomposition.md:834-850`) and where the last `todo!()` and the scoped allow disappear — AC-011 *verifies* that outcome and halts if it is absent |
| `every-rule-under-workerd` | AC-009's conformance claim has no evidence behind it until the rules have actually executed on the target. The `SKIP <rule>: <reason>` transcript this story diffs against is that story's output |
| `measured-store-limits` | The fixture's `Capability` constants and their stated reasons are what AC-009's qualifier list is written from, in the fixture's own words. Without them the claim would be written from memory, which is the failure mode AC-009 exists to prevent |

**Unlocks.** No story in this project depends on this one — `publish-readiness` is the last
milestone and has no outbound edge inside `cloudflare-durable-object-store`
(`_storymap.md:162-165`). Downstream, at project grain:

- **`publication-and-positioning` (HS-P0016)** — the version, the registry landing page,
  the MSRV promise, the semver diff and the clause-ledger audit all presuppose a crate that
  is publish-ready and named. This story is what makes that presupposition true
  (`project.md`, **Out of scope**; **Dependencies**, *Unlocks*).
- **`closeout-and-durable-audience` (HS-P0019)** — initiative **DoD 10**, *the published
  crate looks finished*, is observed there on the terminal `verify.e2e` grain
  (`initiative.md:388-390`). This story makes DoD 10 *reachable* for this crate; it does not
  observe it.

**Milestones 4 and 5 have no edge between them** and could merge in either order; the story
map puts this one last so the final green gate is observed against a tree whose atoms are
already accepted and whose `redkiln validate --kb` is clean (`_storymap.md:167-169`).

## Anchors (progressive disclosure)

Load-bearing depth, deferred rather than pasted. The `## Context pack` above is sufficient
to start; open these at the moment named.

| anchor | why it is load-bearing | when to open | serves |
| ------ | ---------------------- | ------------ | ------ |
| `xtask/src/package.rs` | The mount. `PUBLISHABLE` (`:86`), `REQUIRED_FILES` (`:94`), `run` (`:103`), `reconcile` (`:172`) and `publishable_members` (`:226`) — and the module docs at `:1-74` explaining `--allow-dirty`, the derived-versus-declared pair, and why both sets exist so a failure can name which one moved | Before touching the manifest at all — read `reconcile`'s two failure messages first so the red gate you are about to cause is one you already understand | AC-003, AC-004 |
| `crates/happenstance-cloudflare/Cargo.toml` | The manifest being changed: `publish = false` at `:12`, the false `description` at `:3`, and the `[workspace.package]` inheritance that means only `description` and `readme` are per-crate here. Its `worker` note at `:19-25` is the trade `worker-binding-layer` reversed | First edit of the story, together with the licence copies | AC-002, AC-003, AC-005 |
| `crates/happenstance-cloudflare/src/lib.rs` | The front page being rewritten: `# Status: not implemented` at `:4-10`, the four findings, the two capability limits, the `Targets` note, the scoped `#![allow(clippy::todo)]` at `:122-126`, and the `not_send_probe` module that must survive untouched | Before writing a word of the new front matter — the retention list is as binding as the removal | AC-006, AC-011 |
| `crates/happenstance/src/lib.rs` | The `#![cfg_attr(doctest, doc = include_str!("../README.md"))]` pattern at `:1-10` with the comment stating why a non-compiling README example is worse than none, and why only the crate's *own* README may be included | Immediately before adding the attribute; it is the whole of AC-007's mechanism and its hazard | AC-007 |
| `standards/rust/70-rustdoc-obligations.md` | The house rule the doctest answers to — the one lever the Rust constitution names for making a documentation claim mechanically checkable. Pull this atom, not the corpus (`CLAUDE.md`, **House style**) | When writing the README's example and deciding what it must demonstrate | AC-007, AC-008 |
| `crates/happenstance-testkit/src/lib.rs` | `:101-118` — `event_store_concurrency_conformance!`'s `F::Store: EventStore + Send` bound and its `cfg` off `wasm32`. This is the *reason* the non-invocation is documented rather than apologised for, and AC-009's claim has to state it accurately | While drafting the conformance paragraph | AC-009 |
| `crates/happenstance-testkit/src/contract.rs` | `:473-483` — the `SKIP <rule>: <reason>` emission the README's qualifiers are diffed against, and `Capability::declined`'s rejection of an empty reason at `:115-117` | With the `workerd` transcript open, writing AC-009's qualifier list | AC-009 |
| `xtask/src/reserve.rs` | The reservation mechanism and its argument: `:12-30` states both reasons the real crate is never published early; `:92-97` is `happenstance-cloudflare`'s `RESERVABLE` row with the description the manifest must match; `:158-165` copies the licences rather than linking them; `:130-191` is what the generated placeholder contains | Before running `cargo xtask reserve`, and again when correcting the manifest `description` | AC-005, AC-010 |
| `xtask/src/main.rs` | `:515-532` the packaging gate step; `:769-791` `wasm_steps()`/`steps_named` with the comment explaining why selection is by name and never by index; `:835-860` `run_fast` versus the full `ci` | At the end, enumerating "every `wasm32` step" by name for AC-012 — and before assuming a count | AC-004, AC-012 |
| `crates/happenstance-core/Cargo.toml` | `:12-15` the precedent for stating `readme = "README.md"` rather than relying on auto-discovery, with its reason; `:54-56` the `[package.metadata.docs.rs]` block to follow | While editing the manifest and deciding the docs.rs configuration | AC-002, AC-006 |
| `crates/happenstance-testkit/README.md` | `:8-22` — the precedent for a status paragraph that is honest about what has *not* happened. The register to write AC-009's qualifiers in | While drafting the README's status and conformance paragraphs | AC-009 |
| `.bklg/from-contract-to-published-library/cloudflare-durable-object-store/publish-ready-crate/discover.md` | This story's own signal ledger, the five answered questions, and the named wrong implementation at `:89-119` including the honest limits of both detectors | At the start, and again at self-review before claiming AC-008 and AC-009 | AC-008, AC-009 |
| `.bklg/from-contract-to-published-library/cloudflare-durable-object-store/_decomposition.md` | ARCH-AC-08 (`:81-85`); the testing brief's AC-012 row (`:527`) and its merge-gate commands (`:649-666`); the open target question for the unit tier after the `worker` swap (`:587-616`); DEPLOY-AC-03/04 (`:713-720`) and §4–§5's dependency-surface and release-path notes (`:834-866`) | `:587-616` the moment the doctest will not link (EC-004); `:649-666` before deciding which gate to run; `:834-866` if `worker`'s MSRV or licence surface looks unpriced | AC-007, AC-012 |
| `.bklg/from-contract-to-published-library/cloudflare-durable-object-store/project.md` | AC-012 verbatim (`:243`), the **Out of scope** boundary that keeps publication out, and **Definition of done** 3's grep | Before writing anything that could be a version, a landing page or an MSRV promise; and at AC-011 | AC-011, AC-012 |
| `.bklg/from-contract-to-published-library/cloudflare-durable-object-store/_storymap.md` | Activity **E**, *depend on the crate*, and its observer (`:30`); this story's row (`:58`); **Merge order** §5 and the note that the higher `wasm32` bar wins (`:162-169`) | When scoping the green-gate claim, and when confirming there are no slice-mates | AC-012 |
| `.bklg/from-contract-to-published-library/cloudflare-durable-object-store/_design.md` | The signed-off determination that this project has **no user-facing surface** — the authority for the composition family being N/A in *Interaction quality* rather than merely unaddressed | Only if someone proposes a rendered surface, or when reviewing the interaction-quality section | AC-006 |
| `.bklg/from-contract-to-published-library/initiative.md` | `:227-250` the four personas and journeys the ACs are framed from; `:330-332` AC-08, the standing requirement the compliance claim answers to; `:388-390` DoD 10 | Before writing the README's conformance sentence, and when reviewing whether an AC is framed as intent rather than capability | AC-009 |
| `RUNBOOK.md` | `:898-906` defect D11 — three crates whose metadata promised two licences neither of which was in the tarball, and an exit box ticked before the step that keeps it true existed; `:4241-4307` phase 9, with `:4254-4261` the crate-name note and phase-start reservation rule and `:4301` the exit box this story ticks | `:898-906` at the start, as the reason the copies are copies; `:4254-4261` before the reservation; `:4301` at the end | AC-001, AC-010, AC-012 |
| `.kb/decisions/0029-msrv-raised-to-1-97-1.md` | The accepted decision that the MSRV floor is 1.97.1 and moves only by a recorded trade. NF-006's authority for neither raising nor promising it here | Only if `worker` appears to have moved the floor — which would mean `worker-binding-layer` did not finish | AC-012 |

## Clarifications resolved during spec

1. **The AC set is exactly AC-001…AC-012**, as the first pass decided. No id was added or
   dropped. Two things that could have been extra rows are folded rather than split:
   the `[package.metadata.docs.rs]` declaration sits inside **AC-006** (it is part of "the
   front page renders at all", and splitting it would have produced a row whose only
   verification is another row's command), and the byte-identity of the licence copies sits
   inside **AC-001** rather than becoming an NF — it is the substance of the criterion, not a
   quality attribute of it. The numeric coincidence with project AC-012 is exactly that: this
   story's AC-012 is the green-gate criterion, the project's AC-012 is the whole story.

2. **The composition family of RFC §6.7/D6 is N/A, and that is a citation rather than a
   judgement.** `_design.md`'s `## Surfaces` is `N/A — no user-facing surface`, approved
   2026-08-12, with its `## Items` and `## Signatures` blocks empty. The state family *does*
   apply, to the one artefact a human reads rather than runs — the rendered front page — and
   every invariant that applies is an AC row (AC-003, AC-006, AC-007, AC-009, AC-010), not a
   bullet. The *Interaction quality* section maps them and adds nothing new, so no invariant
   escapes the ledger.

3. **"All four `wasm32` steps" is read as "every step `wasm_steps()` names at merge time".**
   Project AC-012's wording predates `wasm-execution-gate-step`. The story map's merge-order
   note settles it — the higher bar wins (`_storymap.md:162-169`) — and AC-012 here requires
   enumeration by name against `steps_named` (`xtask/src/main.rs:784-791`) rather than against
   any number written in this spec, which would be stale the moment the fifth step lands.

4. **The full gate, not `--fast`, is run once.** `_decomposition.md:649-666` holds this
   non-terminal project to `cargo xtask ci --fast` at the integration grain. That stays true
   for every other story here; this one runs the full `cargo xtask ci` because its acceptance
   criterion is a claim *about* the full gate (NF-007). The two are not in conflict — one is
   the standing bar, the other is this story's deliverable.

5. **AC-010 does not block the diff.** The discover stage asked whether the name is reserved
   in this story or at project start and answered "project start"
   (`discover.md:35-40`, `RUNBOOK.md:4261`). Resolved here as: the reservation is an AC of this
   story because this story is where it is *recorded*, but it is decoupled from the code
   change — the placeholder shares nothing with the workspace but its metadata, runs out of
   tree in `target/`, and a failure scopes to AC-010 alone (EC-007). If the name was already
   held at phase start, recording that state satisfies the criterion; a re-claim is not
   attempted.

6. **AC-008 and AC-009 are honest about what is mechanical and what is review.** AC-008's
   doctest is fully mechanical; its "no literal position value" half is a reviewer grep,
   because a doctest can be green and still teach the literal-position assumption `CLAUDE.md`
   forbids. AC-009's qualifier diff is mechanical against the `workerd` transcript; "true but
   narrower than it sounds" is not, and `discover.md:116-119` already named that limit. Both
   review obligations are written into the criteria rather than left to the reviewer's
   discretion, and both answer to the initiative's AC-08 (`initiative.md:330-332`).

7. **EC-004 is a spec-level decision, not an implementation detail.** The testing brief flags
   the post-`worker` unit-tier target as genuinely open (`_decomposition.md:587-616`). This
   spec does not settle which target compiles the doctest — it settles what must happen if the
   host cannot: the narrowest honest attribute plus a written reason, never a silent downgrade
   to a fenced block. That is the difference between a degraded mechanism and a deleted one.

8. **No `.kb/` atom and no conformance rule is authored here.** Every KB write in this project
   belongs to `adr-0023-and-atom-resolutions` (`_storymap.md:94-102`), and a rule asserting on
   prose would be the decorative rule `CLAUDE.md`'s corollary forbids — no store fails anything
   in this story and no port changes (`discover.md:105-119`). The two things that *can* fail
   are the packaging assertion and the README doctest, and both are already in the gate.
