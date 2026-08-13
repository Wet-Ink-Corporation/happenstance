---
item: HS-S0072
stage: spec
created: 2026-08-12T13:47:10.616Z
updated: 2026-08-12T13:47:10.616Z
template_sig: 87bbf1d0
rendered_sig: 020b1c0a
---

# Spec — Neither crate is a skeleton any more

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` — DoD 10, DoD 5, DoD 6 |
| Project | `.bklg/from-contract-to-published-library/postgres-and-neon-stores/project.md` — AC-012, DR-1, DR-9 |
| This spec | `.bklg/from-contract-to-published-library/postgres-and-neon-stores/deskeleton-and-package-readiness/spec.md` |
| Key brief — architecture | `.bklg/from-contract-to-published-library/postgres-and-neon-stores/_decomposition.md:145-157` — *Root C, the gate*, and the three-part shape of the `publish = false` deletion |
| Key brief — deployment | `.bklg/from-contract-to-published-library/postgres-and-neon-stores/_decomposition.md:684-787` — the AC-012 row, the Hyperdrive note, "readiness, not release" |
| Key brief — testing | `.bklg/from-contract-to-published-library/postgres-and-neon-stores/_decomposition.md:478-665` — the live-job gating this story must not disturb |
| Signed-off design | `.bklg/from-contract-to-published-library/postgres-and-neon-stores/_design.md` — **no user-facing surface**, approved 2026-08-12; nothing here renders a screen |
| Story map row | `.bklg/from-contract-to-published-library/postgres-and-neon-stores/_storymap.md:71` and `:197-202` |
| Roadmap | `RUNBOOK.md:4344-4390` — phase 10 work items, proof artefact and exit criteria; `RUNBOOK.md:4383` is this story's exit line verbatim |

## One-line PR slice

Neither crate is a skeleton any more: no `todo!()`, no scoped
`#![allow(clippy::todo)]`, no `publish = false`, `PUBLISHABLE` grown to five with
licences and READMEs in place, and the Hyperdrive note written as explicitly
unsupported.

## Executive summary

This PR lands the **reconciliation** between what the two crates now are and what
the repository still says they are. Every earlier story in this project changed
behaviour; this one changes *claims* — the manifests, the crate-level prose, the
publishable set, and the three files crates.io renders — so that the tree stops
describing `happenstance-postgres` and `happenstance-neon` as instruments that do
not work.

The delta against the four stories it depends on:

- **Delta on `postgres-append-and-frontier-head`, `postgres-projection-store`,
  `neon-append-and-read-over-http`, `neon-conflicting-position-verdict`** — those
  removed the `todo!()` *bodies*. This one removes the two `#![allow(clippy::todo)]`
  lines that made them tolerable (`crates/happenstance-postgres/src/lib.rs:59-63`,
  `crates/happenstance-neon/src/lib.rs:102-105`), the `#[expect(dead_code)]` that
  existed only because a decoder was `todo!()`
  (`crates/happenstance-neon/src/event_store.rs:232-247`), and the "# Status: not
  implemented" sections that are now false
  (`crates/happenstance-postgres/src/lib.rs:3-8`,
  `crates/happenstance-neon/src/lib.rs:4-8`).
- **Delta on `claim-crate-names`** — that story *held* the two names on crates.io.
  This one spends the claim: `publish = false` comes out of both manifests
  (`crates/happenstance-postgres/Cargo.toml:12`,
  `crates/happenstance-neon/Cargo.toml:12`) only because the names are held.
- **Delta on the gate** — `PUBLISHABLE` grows from three names to five
  (`xtask/src/package.rs:86`), which makes an *already-`REQUIRED`* step
  (`xtask/src/main.rs:520-532`) start asserting licences and a README for five
  crate directories instead of three. Neither directory holds any of the three
  files today.

What it does **not** land: any `cargo publish`, any version bump, any release
train entry, and any change to the specification's clause ledger. Those are
`publication-and-positioning` (HS-P0016) and the slice-mate
`far-end-discharge-record` respectively.

## Context pack

The decisions below are the ones this story must honour. They are stated here as
decisions, not as reading; the deeper material is behind the anchors.

**1. Readiness, not release — and the line is `cargo package --list`.** By the
time this story closes, `cargo package -p happenstance-postgres --list` and
`cargo package -p happenstance-neon --list` both succeed and both list
`LICENSE-MIT`, `LICENSE-APACHE` and `README.md`. Nothing more is claimed. The
actual publish, the `0.2.0` release, the semver diff and the clause-ledger audit
are HS-P0016's and are out of scope by name
(`_decomposition.md:773-787`, `project.md` *Out of scope*). The workspace is
already at `0.2.0` (`Cargo.toml:6`, `version.workspace = true`), so **neither
crate takes a version of its own** — removing `publish = false` inherits the
version the rest of the workspace already carries.

**2. The deletion is a three-part change, and the gate fails in both directions.**
`xtask/src/package.rs`'s `reconcile` (`:172-218`) compares the hand-written
`PUBLISHABLE` list against what `cargo metadata` says Cargo *will* publish, and
fails when either set holds a name the other does not — deliberately, because the
two directions are two different bugs with two different remedies (`:163-167`).
So editing the manifest without editing `PUBLISHABLE` fails the gate ("Cargo will
publish X but this step does not check it"), and editing `PUBLISHABLE` without
the manifest fails it the other way. Both, plus the three files, or none.

**3. The licence and README files must be *copies inside each crate directory*.**
Not a symlink, not a `readme = "../../README.md"`, not a workspace-level
`include`. `package.rs`'s own failure message says why: "Copy the file into the
crate directory — Cargo will not follow a path outside it" (`:152-157`). The
three publishable crates each already carry their own copies
(`crates/happenstance-core/`, `crates/happenstance/`, `crates/happenstance-testkit/`),
and that is the pattern to follow, not to improve on. Declare `readme = "README.md"`
explicitly in each manifest for the reason `happenstance-core` gives at
`crates/happenstance-core/Cargo.toml:12-15`: Cargo would infer it, but the key is
what crates.io renders, and a silent default is a poor thing to rest a first
impression on.

**4. The README is a real front page, not the reservation placeholder.**
`xtask/src/reserve.rs:243-269` generates a README for the `0.0.0` name-holding
publish — "**This `0.0.0` is a placeholder. It contains no functionality.**" That
text belongs to `claim-crate-names`, and copying it here would make this story
land the exact claim it exists to retract. Model the two new READMEs on
`crates/happenstance-testkit/README.md` and `crates/happenstance/README.md`.

**5. The manifest `description` is a claim, and it is the one crates.io searches.**
Both descriptions currently end "Not yet implemented."
(`crates/happenstance-postgres/Cargo.toml:3`,
`crates/happenstance-neon/Cargo.toml:3`). `reserve.rs` already holds the
description the real crate is meant to carry, and says so in terms — "the one the
real crate will carry, so that the placeholder and the eventual release describe
the same thing to anyone browsing" (`xtask/src/reserve.rs:41-49`, entries at
`:97-110`). Reconcile the manifests to those strings rather than inventing a third
wording.

**6. Removing the `allow` is compiler-enforced, and one of the two removals
self-destructs.** `clippy::todo` is denied workspace-wide; the two
`#![allow(clippy::todo)]` lines were scoped to their crate *specifically so they
would disappear with the last `todo!()` rather than outlive it* — both comments
say so (`crates/happenstance-postgres/src/lib.rs:59-62`,
`crates/happenstance-neon/src/lib.rs:102-104`). Separately,
`crates/happenstance-neon/src/event_store.rs:237-240` uses `#[expect(dead_code)]`
rather than `#[allow]` for exactly this moment: once `decode_append_response`
constructs an `AppendOutcome`, the expectation is unfulfilled and
`unfulfilled_lint_expectation` fires, which under `-D warnings` is a gate failure.
That is the Rust-specific point worth internalising here: `expect` is `allow` with
an expiry date, and it is why leaving the attribute behind is not a tidiness
problem but a red build.

**7. De-skeletoning is a claim change, never a behaviour weakening.** A `todo!()`
replaced by `unimplemented!()`, by a `panic!`, or by an `Err(…)` stub that no rule
exercises is a skeleton wearing a different hat — and it would pass `clippy::todo`
while making the crate *less* honest than it is now. The four stories this one
depends on are what earn the removal (`RUNBOOK.md:3061-3068`: a `todo!()` body
type-checks against any signature, so "it compiles" counts for nothing). If any
body is still unreachable-by-design after those stories, that is a finding to
record, not a marker to delete.

**8. Removing `publish = false` is only honest once the name is held.** That is
the entire reason `claim-crate-names` is a `depends_on` edge rather than a
neighbour: `xtask/src/reserve.rs:99,105` knows both names and phase 0's rule is
that a name is reserved when its phase starts (`RUNBOOK.md:4346-4348`). Verify the
claim landed before flipping the manifests; do not re-run the reservation here.

**9. The gate gains no step, and one name must not move.** `package-check` is
already in `REQUIRED` (`xtask/src/main.rs:105`, step at `:520-532`), so growing
`PUBLISHABLE` changes what an existing default-gate step checks — it does not add
a job, a step, Docker, or a network call. AC-011's Docker-free, network-free
default gate is therefore preserved by *not touching the step list*. Related trap
in the same file: `wasm_steps()` selects steps **by name and panics on a miss**
(`xtask/src/main.rs:771-800`), so renaming "wasm32 build of the Neon adapter" while
tidying is a gate change, not a cosmetic one. Do not rename it.

**10. `cargo deny` is already walking both graphs; what changes is the excuse.**
`deny.toml:1-2` sets `all-features = true` at the graph level and both crates are
workspace members, so the licence allowlist (`deny.toml:9-19`) already covers them
today. What removing `publish = false` removes is the ability to treat a licence
failure as acceptable in an unpublishable instrument. If a transport dependency
taken by `neon-sql-transport` fails the allowlist — the known live risk is `sqlx`'s
`tls-rustls` failing on `webpki-roots` (`CDLA-Permissive-2.0`), not on `ring`
(`crates/happenstance-postgres/src/lib.rs:40-47`, `Cargo.toml:54-60`) — this story
is where that becomes a blocking fact rather than a note. It is a finding to
report, not an allowlist to widen.

**11. The Hyperdrive note is documentation and is explicitly *not* a supported
configuration.** Hyperdrive plus a `worker::Socket`-backed driver needs a forked
driver with unnamed-statement support and a hand-rolled binding
(`RUNBOOK.md:4364-4366`). The deployment brief is unambiguous about the grain:
"No code, no CI job, no test" (`_decomposition.md:690`). What it leaves open is
placement — rustdoc or README — and this spec decides it: the note goes in
`happenstance-postgres`'s crate-level rustdoc, beside the existing "# Not the Neon
adapter" section (`crates/happenstance-postgres/src/lib.rs:49-56`) which already
holds this crate's "how do I reach Postgres from somewhere unusual" prose, with a
one-line cross-reference from the Neon crate. An implementer who places it
elsewhere records the reason; the one thing that is not open is whether it reads as
supported.

**12. The persona-journey slice.** There is no screen (`_design.md` — "N/A, no
user-facing surface", approved). The user here is an **adapter author or a
consuming application** arriving from crates.io, and the moment this story owns is
the first ten seconds of that arrival: the package page carries a licence, a
description that is true, and a README that says what the adapter is and what it
cannot do. That is initiative **DoD 10** ("the registry page carries licence,
description and README as rendered, checked by looking at them",
`initiative.md:387-389`) — this story builds the artefact; HS-P0016 does the
looking.

## Integration contract

- **Archetype**: `capability`. It is user-observable in this repository's only
  medium — the package a consumer meets and the gate that asserts it.
- **Slice / milestone**: `publish-readiness-and-audit`. Slice-mate:
  **`far-end-discharge-record`**, implemented in the same context and mounted with
  this one. That story reports on what the project discharged; this one makes the
  crates it reports on stop being skeletons. Order within the slice is this story
  first (`_storymap.md:262-265`).
- **Mount point**: **`xtask/src/package.rs`** — the `PUBLISHABLE` const at `:86`,
  which `reconcile` (`:172-218`) and `run` (`:103-161`) consume. It is reached from
  `xtask/src/main.rs`'s `REQUIRED` array (`:105`) via the `package-check` step
  (`:520-532`), which is the repository's Root C composition root
  (`_decomposition.md:145-157`). Growing that const is what makes this story
  *mounted* rather than a pair of manifest edits nothing observes: the moment
  `PUBLISHABLE` names five crates, every subsequent `cargo xtask ci` and
  `cargo xtask ci --fast` run asserts the five-crate surface.
- **Wires into**:
  - `crates/happenstance-postgres/Cargo.toml` and
    `crates/happenstance-neon/Cargo.toml` — `publish`, `description`, `readme`.
  - `crates/happenstance-postgres/src/lib.rs` and
    `crates/happenstance-neon/src/lib.rs` — the crate-level rustdoc, the two
    `#![allow(clippy::todo)]` lines, and the Hyperdrive note.
  - `crates/happenstance-neon/src/event_store.rs:232-247` — the `#[expect(dead_code)]`
    whose expiry this story collects.
  - `xtask/src/reserve.rs:97-110` — read-only, as the source of the descriptions and
    the record that both names are phase-10 claims.
  - `deny.toml:1-19` — read-only; the allowlist both graphs must already satisfy.
  - `crates/happenstance-core/Cargo.toml:12-15`,
    `crates/happenstance-testkit/README.md`, `crates/happenstance/README.md` — the
    in-tree pattern for a publishable crate's three files.
- **Renders surfaces**: **none.** `_design.md` records this project as having no
  user-facing surface and its `## Items` block is `N/A`; the sign-off approved that
  determination. There is no surface id for this story to claim and no perceptual
  review is owed (`.redkiln/config.yaml` deliberately omits `design.capture`).
- **Conformance rule(s)**: **none, and deliberately so.** This story changes no
  port, no store behaviour and no rule; nothing it touches is observable through
  `happenstance-testkit`'s `suite.rs`, because a conformance rule observes a store
  through the `EventStore` trait and cannot see a manifest key, a crate-level lint
  attribute or a README. Its falsifier is the gate — `cargo xtask package-check`
  and `cargo clippy -D warnings` — not a rule. Per CLAUDE.md's corollary, adding a
  decorative rule here that no adapter could fail would be worse than naming none.
- **Clause(s)**: **none discharged or amended.** No `spec/SPECIFICATION.md` clause
  changes; the far-end clause status (ES-10, ES-11, ES-12, ES-41, ES-42,
  VT-21 – VT-24) is written down by the slice-mate `far-end-discharge-record`, and
  `cargo xtask spec-trace` must remain green across this change.
- **Advances DoD scenario**: initiative **DoD 10** — "the published crate looks
  finished … the registry page carries licence, description and README as rendered"
  (`initiative.md:387-389`). This story builds the readiness half of it for the two
  crates the project owns. It is also the point at which **DoD 5** and **DoD 6**
  stop being rhetorical: a store that "passes the suite" while its own crate docs
  say "Status: not implemented" and its manifest says `publish = false` is a claim
  the tree contradicts.

## PR boundary

`redkiln verify --grain story` reads the fenced block below and fails on any file
changed outside it.

```
crates/happenstance-postgres/**
crates/happenstance-neon/**
xtask/src/package.rs
.bklg/from-contract-to-published-library/postgres-and-neon-stores/deskeleton-and-package-readiness/**
```

The two crate globs are wide on purpose and are narrower than they look: the
manifests, the two `lib.rs` files, one `event_store.rs` attribute, and six new
files (three per crate) all live under them, and this story is the one that
touches both crates by definition. `xtask/src/package.rs` is the mount point.
Nothing else — in particular **not** `xtask/src/main.rs`, because `package-check`
is already `REQUIRED` and no step is added.

**In this PR**

- Both manifests: `publish = false` removed, `description` reconciled to the
  reserved wording, `readme = "README.md"` declared.
- Six files created: `LICENSE-MIT`, `LICENSE-APACHE`, `README.md` in each of
  `crates/happenstance-postgres/` and `crates/happenstance-neon/`.
- `xtask/src/package.rs:86`: `PUBLISHABLE` grown to five names.
- Both `lib.rs` files: `#![allow(clippy::todo)]` deleted with its comment; the
  "# Status: not implemented" section replaced by prose that is true.
- `crates/happenstance-neon/src/event_store.rs`: the `#[expect(dead_code)]` on
  `AppendOutcome` removed.
- The Hyperdrive note, written as unsupported, with its placement recorded.
- Any residual `todo!()` removed — or, if one survives, reported as a blocking
  finding rather than deleted.

**Explicitly not in this PR**

- `cargo publish`, a release tag, a version bump, or any crates.io write. The name
  reservation is `claim-crate-names`'; the release is HS-P0016's.
- A new CI job, a new `REQUIRED`/`OPTIONAL` step, or any change to
  `.github/workflows/ci.yml` — the two live-infrastructure jobs belong to the
  fixture stories.
- The far-end clause discharge record and any `spec/SPECIFICATION.md` edit — the
  slice-mate's.
- Any change to store behaviour, schema, SQL, transport or fixture. If a body is
  found wanting here, it routes to the owning story or to the `support` initiative
  (`.redkiln/config.yaml`), never to a marker deletion.
- Widening `deny.toml`'s allowlist to make a newly-scrutinised dependency pass.

**Merge DoD**: `cargo xtask ci --fast` is green on a clean checkout with no Docker,
no network and no credentials, with `package-check` reporting five crates each
carrying `LICENSE-MIT`, `LICENSE-APACHE` and `README.md`, and `grep -r 'todo!'`
finding no invocation in either crate.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| No `todo!()` invocation survives in either crate (AC-001) | Every `todo!("…")` call in both crates is gone, having been *implemented* by the four depended-on stories rather than deleted here. A surviving one is a blocking finding routed to its owning story, not a marker removed. Prose that mentions the word in past tense is fine; an invocation is not. | `crates/happenstance-postgres/src/{event_store.rs,projection_store.rs,read_stream.rs}`, `crates/happenstance-neon/src/{event_store.rs,projection_store.rs}` |
| The two scoped `clippy::todo` allows are deleted, with their comments (AC-001) | `clippy::todo` is denied workspace-wide; each allow was written to "disappear with the last `todo!()` rather than outliving it". Deleting the attribute and leaving the three-line comment behind leaves a false explanation in the file. | `crates/happenstance-postgres/src/lib.rs:59-63`, `crates/happenstance-neon/src/lib.rs:102-105` |
| The `#[expect(dead_code)]` on `AppendOutcome` is removed (AC-002) | `expect` is `allow` with an expiry: once `decode_append_response` constructs the enum, the expectation is unfulfilled and `unfulfilled_lint_expectation` fires, which `-D warnings` turns into a gate failure. The attribute's own `reason` string names phase 10 as its end. | `crates/happenstance-neon/src/event_store.rs:232-247` |
| Crate-level rustdoc stops claiming "not implemented" (AC-002) | Both crates open with a `# Status: not implemented` section asserting `publish = false` and `todo!()` bodies. Replace with what is now true, keeping the *instrument* framing that both crates earn — the Postgres crate is still the workspace's only non-serialising writer, the Neon crate still the least capable store and the `ProbeThenWriteStore` trap is still the point. Do not delete the capability table or the CTE discussion. | `crates/happenstance-postgres/src/lib.rs:1-56`, `crates/happenstance-neon/src/lib.rs:1-99` |
| Manifest `description` reconciled to the reserved wording (AC-002) | Both currently end "Not yet implemented." `reserve.rs` holds the string the real crate is meant to carry so the placeholder and the release describe the same thing to a browser. Adopt those, not a third wording. | `crates/happenstance-postgres/Cargo.toml:3`, `crates/happenstance-neon/Cargo.toml:3`, `xtask/src/reserve.rs:97-110` |
| `publish = false` removed from both manifests (AC-003) | One line per manifest. No version key is added: `version.workspace = true` already resolves to `0.2.0`. `readme = "README.md"` is added explicitly for the reason `happenstance-core` states — the key is what crates.io renders, and a silent inference is a poor first impression to rest on. | `crates/happenstance-postgres/Cargo.toml:12`, `crates/happenstance-neon/Cargo.toml:12`, `crates/happenstance-core/Cargo.toml:12-15`, `Cargo.toml:6` |
| Both names are held on crates.io before the manifests flip (AC-008) | The precondition, consumed from `claim-crate-names` and verified rather than assumed. Removing `publish = false` on a name someone else can take is the failure this dependency edge exists to prevent; phase 0's rule is that the name is reserved when its phase starts. | `xtask/src/reserve.rs:99,105`, `RUNBOOK.md:4346-4348`, `.bklg/from-contract-to-published-library/postgres-and-neon-stores/claim-crate-names/` |
| `PUBLISHABLE` grows from three names to five (AC-004) | The mount. `reconcile` fails in **both** directions and names which side moved, so this edit and the manifest edit are one change: a manifest flip alone reports "Cargo will publish X but this step does not check it"; a list edit alone reports "PUBLISHABLE names X but Cargo will not publish it". | `xtask/src/package.rs:86`, `:172-218` |
| Three required files exist in each crate directory (AC-005) | `LICENSE-MIT`, `LICENSE-APACHE`, `README.md` copied *into* `crates/happenstance-postgres/` and `crates/happenstance-neon/` — Cargo will not follow a path outside the crate directory, which is why the other three publishable crates each hold their own copies. Licence texts are byte-identical to the workspace roots. | `xtask/src/package.rs:88-94`, `:131-157`, `crates/happenstance-testkit/`, `crates/happenstance/`, `LICENSE-MIT`, `LICENSE-APACHE` |
| `cargo xtask package-check` passes over five crates (AC-005) | Already a `REQUIRED` step, so this is a change in what an existing default-gate step asserts, not a new step. `cargo package -p <crate> --list --allow-dirty --locked` must succeed for both new crates and the listing must contain all three files. | `xtask/src/main.rs:105`, `:520-532`, `xtask/src/package.rs:103-161` |
| Each README is a real front page, not the reservation placeholder (AC-006) | Says what the adapter is, which port flavour it implements, what it cannot do (Neon: no connection, no interactive transaction, no cursor, 64 MiB cap; Postgres: `head` is a frontier, no read-your-own-writes), and how to run its live conformance suite. The `0.0.0` placeholder text — "This `0.0.0` is a placeholder. It contains no functionality." — must not appear. | `xtask/src/reserve.rs:243-269`, `crates/happenstance-testkit/README.md`, `crates/happenstance/README.md`, `crates/happenstance-neon/src/lib.rs:10-29` |
| The Hyperdrive note exists and reads as unsupported (AC-007) | Documentation only — no code, no CI job, no test. States Hyperdrive plus a `worker::Socket`-backed driver and why it is not supported: a forked driver with unnamed-statement support and a hand-rolled binding. Placement decided here (Postgres crate rustdoc, beside "# Not the Neon adapter", cross-referenced from the Neon crate); a different placement is a recorded decision, not a silent one. | `RUNBOOK.md:4364-4366`, `_decomposition.md:690`, `crates/happenstance-postgres/src/lib.rs:49-56` |
| The default gate stays Docker-free, network-free and credential-free (AC-009) | No step is added to `REQUIRED`/`OPTIONAL`, no job to `.github/workflows/ci.yml`, and no name in `wasm_steps()` is changed — it selects by name and panics on a miss. `cargo xtask ci --fast` green on a clean checkout is the bar this project's non-terminal grain sets; the Neon `wasm32` build stays green throughout (DR-8). | `xtask/src/main.rs:771-800`, `:105`, `.redkiln/config.yaml` *verify* block, `project.md` DR-8, DR-9 |
| `cargo deny` and `spec-trace` stay green under the new scrutiny (AC-009) | `deny.toml` already sets `all-features = true` at graph level, so both graphs are already walked; what changes is that a licence failure can no longer be excused as "an unpublishable instrument". A failure here is reported, never fixed by widening the allowlist. `spec-trace` must remain green: no clause or citation moves in this story. | `deny.toml:1-19`, `crates/happenstance-postgres/src/lib.rs:40-47`, `Cargo.toml:54-60` |
| No public API item is added, removed or changed | The crates' public surface is whatever the four depended-on stories left. This story touches manifests, lints, docs and packaging metadata only, so a semver diff across it is empty by construction — which is what makes it safe to land last in the project. | `_design.md` (`## Items` — N/A), `crates/happenstance-neon/src/lib.rs:107-129`, `crates/happenstance-postgres/src/lib.rs:65-78` |

## Data and migrations

**N/A — no schema change, no migration, no backfill.**

Three reasons, each verified rather than assumed:

1. **The schema is another story's.** `happenstance-postgres` owns exactly one
   migration, migration 1, authored by `postgres-schema-and-live-fixture` and
   carrying phase 4's `EventId` and `recorded_at` columns plus whichever column
   ADR-0024's chosen mechanism adds. This story adds no column, no index and no
   statement (`_decomposition.md:713-724`).
2. **There is nothing to backfill.** Both crates are `publish = false` today and
   have never shipped a schema to a real consumer, so there is no prior deployment
   to migrate data out of (`_decomposition.md:717-720`).
3. **The only "data" this story writes is six files.** `LICENSE-MIT`,
   `LICENSE-APACHE` and `README.md` per crate directory — copies, not links,
   because `cargo package` will not follow a path outside the crate directory
   (`xtask/src/package.rs:152-157`).

One adjacent fact worth stating so it is not rediscovered as a surprise:
`crates/happenstance-postgres/Cargo.toml:22` enables `sqlx` with `postgres` and
`runtime-tokio` and deliberately **no** `migrate` feature. Whether that feature is
turned on is Architecture §9.3's open question and belongs to the schema story;
this story neither turns it on nor requires it, and must not flip it while tidying
the manifest.

## Acceptance criteria

Nine criteria. Each is stated from the intent of a persona this initiative named
(`initiative.md:200-224`, journeys at `initiative.md:243-250`), because "the crate
is no longer a skeleton" is only worth anything as something a person meets: the
**evaluator**, whose journey *Decide in one sitting* is explicitly "often one look
at a registry page in which to decide" (`initiative.md:222-224`); the **adapter
author**, whose journey *Learn when you are finished* runs "from a signature that
type-checks to a suite that says pass or fail" (`initiative.md:245-247`) and who is
this repository's own next contributor; and the **local-first / edge developer**
whose stated fear is "being on the less-supported path and being orphaned"
(`initiative.md:216-219`) — the fear a crate that ships with no licence text
confirms.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **GIVEN** an adapter author who has been told by `CLAUDE.md`'s repository map that `happenstance-postgres` and `happenstance-neon` are 🔩 skeletons — "real associated types and `todo!()` bodies… and a scoped `#![allow(clippy::todo)]` naming the phase that removes it" — **WHEN** they open either crate after this story, **THEN** neither crate contains a single `todo!()` invocation (34 across the two `src/` trees at the time of writing) and neither `lib.rs` carries `#![allow(clippy::todo)]` or the three-line comment that explained it, **SO THAT** what holds the claim is the workspace-wide denial of `clippy::todo` rather than a note promising a future phase. A `todo!()` swapped for `unimplemented!()`, a bare `panic!`, or an `Err(…)` no rule exercises fails this criterion even though it passes the lint. | `cargo clippy --workspace --all-targets --all-features -- -D warnings`, the gate's clippy step (`xtask/src/main.rs:115-130`), which becomes a **hard error** on any surviving `todo!()` the moment the two allows are deleted — deleting the suppression *is* the falsifier. Belt to that suspender: `rg -n 'todo!\(' crates/happenstance-postgres/src crates/happenstance-neon/src` returns nothing, per the testing brief's AC-012 row (`_decomposition.md:478-665`). |
| AC-002 | **GIVEN** an evaluator taking their one bounded look — the registry blurb, then the front page of the docs — **WHEN** they read either crate's `description` and its crate-level rustdoc, **THEN** nothing there still says the crate is unimplemented: both descriptions have lost the trailing "Not yet implemented." and read as `xtask/src/reserve.rs:97-110` already declares them, both `# Status: not implemented` sections are replaced by prose that is true of the code beneath them, and `crates/happenstance-neon/src/event_store.rs`'s `#[expect(dead_code)]` on `AppendOutcome` — whose `reason` names "phase 10" as its expiry — is gone, **SO THAT** the three expiring claims in this pair of crates expire together rather than one outliving the code that justified it. The *instrument* framing both crates earn is kept: the capability table, the CTE discussion and the `ProbeThenWriteStore` trap are not deleted while tidying. | `cargo clippy … -D warnings` (`xtask/src/main.rs:115-130`) fails on the retained `#[expect]` via `unfulfilled_lint_expectation` once `decode_append_response` constructs the enum — the attribute is `allow` with an expiry, per `standards/rust/90-skeletons-and-todo.md` RS-90-3. The documentation step (`xtask/src/main.rs:284-302`) proves the rewritten rustdoc builds; the description strings are diffed against `xtask/src/reserve.rs:97-110` by review, and `cargo xtask package-check` re-reads both manifests. |
| AC-003 | **GIVEN** an application author who has decided the contract first and is now choosing a database (`initiative.md:202-208`) and who reaches for `cargo add happenstance-postgres` — **WHEN** the manifests are read by Cargo after this story, **THEN** `publish = false` is absent from both (`crates/happenstance-postgres/Cargo.toml:12`, `crates/happenstance-neon/Cargo.toml:12`), each declares `readme = "README.md"` explicitly for the reason `crates/happenstance-core/Cargo.toml:12-15` gives, and **neither takes a `version` key of its own** — `version.workspace = true` already resolves to the `0.2.0` at `Cargo.toml:6`, **SO THAT** the two crates join the release the workspace already carries instead of forking a version line nobody decided. | `cargo xtask package-check` (`xtask/src/main.rs:519-535` → `xtask/src/package.rs:103-161`): `publishable_members` reads the fact out of `cargo metadata` and the crates must appear in the derived set. A stray `version = "…"` is caught by review against `Cargo.toml:6`; `cargo xtask ci --fast` proves the workspace still resolves. |
| AC-004 | **GIVEN** a maintainer who wants the tree to notice when a crate is promoted by accident — the second of the two bugs `xtask/src/package.rs:36-42` says no other step in the gate would ever catch — **WHEN** `PUBLISHABLE` at `xtask/src/package.rs:86` grows from three names to five in the same change that flips the manifests, **THEN** `reconcile` (`:172-218`) reports the declared set agreeing with the derived one, **SO THAT** the intention and the fact are reconciled rather than one silently overtaking the other. Half the change alone must fail, and fail differently in each direction. | `cargo xtask package-check`. Falsified deliberately during implementation, both ways: flip a manifest without the const and confirm the message *"Cargo will publish … but this step does not check it"* (`xtask/src/package.rs:188-196`); grow the const without the manifest and confirm *"PUBLISHABLE names … but Cargo will not publish it"* (`:198-204`). Both transcripts are the evidence, cited into `_ledger.md`. The existing unit tests at `xtask/src/package.rs:408-457` must stay green. |
| AC-005 | **GIVEN** the local-first / edge developer whose fear is being orphaned on the less-supported path, and who unpacks a `.crate` tarball before depending on it — **WHEN** `cargo package -p happenstance-postgres --list` and `cargo package -p happenstance-neon --list` are run, **THEN** each listing contains `LICENSE-MIT`, `LICENSE-APACHE` and `README.md` (`REQUIRED_FILES`, `xtask/src/package.rs:94`) as **copies inside the crate directory** — not a symlink, not a `readme = "../../README.md"`, not a workspace `include` — with licence bodies byte-identical to the workspace-root `LICENSE-MIT` and `LICENSE-APACHE`, and `cargo xtask package-check` reports all five crates clean, **SO THAT** the metadata promise of `MIT OR Apache-2.0` is backed by both texts a consumer needs to make the choice. | `cargo xtask package-check` (`xtask/src/main.rs:519-535`), which parses the listing and asserts the three files **by name, per crate** (`xtask/src/package.rs:131-149`) rather than discarding the output. The wrong implementation it rejects is already in-tree and cited by its own module docs: `cargo package -p happenstance-sqlite --list` exits 0, lists seven files, and none is one of the three (`xtask/src/package.rs:19-22`). Byte-identity checked with `git diff --no-index`. |
| AC-006 | **GIVEN** the evaluator with one look, arriving on the crates.io page rather than in this repository — **WHEN** they read `crates/happenstance-postgres/README.md` and `crates/happenstance-neon/README.md`, **THEN** each is a real front page in the register of `crates/happenstance-testkit/README.md` and `crates/happenstance/README.md`: what the adapter is, which port flavour it implements, **what it cannot do** stated as a limit rather than omitted (Neon: no connection, no interactive transaction, no cursor, the 64 MiB-anchored ceilings; Postgres: `head` is a frontier, no read-your-own-writes, the staleness bound `postgres-structural-bill` wrote), and how to run its live conformance suite — and the reservation placeholder text **"This `0.0.0` is a placeholder. It contains no functionality."** (`xtask/src/reserve.rs:243-269`) appears in neither, **SO THAT** this story retracts the claim `claim-crate-names` deliberately published rather than re-landing it. | Review against the two in-tree exemplars plus `rg -n 'is a placeholder' crates/happenstance-postgres crates/happenstance-neon` returning nothing. Rendering is proven by the documentation step (`xtask/src/main.rs:284-302`) for the rustdoc half and by `cargo xtask package-check` for the file's presence in the artifact; the *content* is a human read, which is exactly what initiative DoD 10 asks for ("checked by looking at them", `initiative.md:387-389`) and what HS-P0016 repeats against the rendered page. |
| AC-007 | **GIVEN** the edge developer who reaches Postgres from a Cloudflare Worker and will otherwise spend a day discovering Hyperdrive's limits themselves — **WHEN** they read `happenstance-postgres`'s crate-level rustdoc beside its existing "# Not the Neon adapter" section (`crates/happenstance-postgres/src/lib.rs:49-56`), **THEN** they meet a note stating Hyperdrive plus a `worker::Socket`-backed driver and stating plainly that it is **not a supported configuration** — it needs a forked driver with unnamed-statement support and a hand-rolled binding (`RUNBOOK.md:4364-4366`) — with a one-line cross-reference from `crates/happenstance-neon/src/lib.rs`, **SO THAT** the configuration is documented as a dead end rather than discovered as one. Documentation only: no code, no CI job, no test (`_decomposition.md:690`). A different placement is permitted; a placement chosen silently is not, and neither is prose that reads as an endorsement. | The documentation step (`xtask/src/main.rs:284-302`) proves it builds and that the cross-reference resolves; `rg -n -i 'hyperdrive' crates/happenstance-postgres/src crates/happenstance-neon/src` locates it, and review confirms the "not supported" reading and that any placement other than the one this spec decided is recorded under *Clarifications* in the implementation report. |
| AC-008 | **GIVEN** a maintainer about to make two crate names publicly claimable by removing `publish = false` — **WHEN** the manifests are flipped, **THEN** it has first been *verified* (not assumed) that `claim-crate-names` landed and both `happenstance-postgres` and `happenstance-neon` are held on crates.io by this project's owner, **SO THAT** the window in which the workspace intends to publish a name it does not hold never opens. The reservation is **not re-run** here; phase 0's rule is that a name is reserved when its phase starts (`RUNBOOK.md:4346-4348`), and `xtask/src/reserve.rs:99,105` already knows both names. | Evidence cited into `_ledger.md`: the `claim-crate-names` story's own ledger/implementation report under `.bklg/from-contract-to-published-library/postgres-and-neon-stores/claim-crate-names/`, plus a recorded check of each name's registry page. Blocking: if either name is unheld, this story stops before AC-003 rather than proceeding and reporting it afterwards. |
| AC-009 | **GIVEN** every contributor who runs the gate on a laptop with no Docker, no credentials and no network — the bar `project.md` AC-011 and DR-9 set for this whole project — **WHEN** this story lands, **THEN** `cargo xtask ci --fast` is green on a clean checkout with none of those present; `REQUIRED`/`OPTIONAL` in `xtask/src/main.rs` gain no step and `.github/workflows/ci.yml` gains no job; **no existing step's `name` string changes**, because `wasm_steps()`/`steps_named` select by name and *panic on a miss* (`xtask/src/main.rs:771-816`); and `cargo deny` and `cargo xtask spec-trace` are both still green, **SO THAT** growing the publishable surface tightens what the gate asserts without changing what the gate costs to run. If a transport dependency now fails `deny.toml`'s allowlist — the known live risk being `sqlx`'s `tls-rustls` on `webpki-roots` (`CDLA-Permissive-2.0`), not on `ring` (`crates/happenstance-postgres/src/lib.rs:40-47`) — that is reported as a blocking finding, never fixed by widening `deny.toml:9-19`. | `cargo xtask ci --fast` (`.redkiln/config.yaml` `verify.integration_scoped`) on a clean checkout; `cargo xtask ci` for the `cargo deny` step (`xtask/src/main.rs:595-601`, `OPTIONAL`); `cargo xtask spec-trace` (`verify.reachability_static`); `git diff --stat` over `xtask/src/main.rs` and `.github/workflows/ci.yml` showing zero changed lines, which is also what the PR boundary enforces via `redkiln verify --grain story`. |

Coverage of the traced project AC: **AC-012** ("No `todo!()` on either path, the
scoped `#![allow(clippy::todo)]` naming this phase is deleted from both crates,
`publish = false` is removed, and both names are claimed on crates.io",
`project.md:271-273`) is covered by AC-001 (the bodies and the allows), AC-003
(`publish = false`) and AC-008 (the names), with AC-002 and AC-004 – AC-007
carrying the parts of the storymap slice and the deployment brief's AC-012 row
that `project.md`'s one sentence compresses. AC-009 is the guard that the whole
of it costs the default gate nothing, which is project AC-011's constraint
consumed rather than re-owned.

## Interaction quality

**The composition family is formally not applicable here, and that is a signed-off
determination rather than an omission.** `_design.md` records `## Surfaces` as
"N/A — no user-facing surface" and every subsequent slot — Items, Signatures,
Shape decision, Placement, Visibility, What a user meets first, States,
Anti-patterns — as `N/A`, approved 2026-08-12. There is no surface id to claim, no
inherited density budget in pixels, no transience policy, and no perceptual
review is owed (`.redkiln/config.yaml` deliberately omits `design.capture`). This
story adds no public API item either, so the repurposed design stage's own subject
— the signatures a `cargo add` user meets — is untouched.

What that leaves is not nothing. This repository's medium is the package and the
gate, and both have a presentation layer that an unstyled equivalent would pass
every structural assertion of. The invariants below therefore **all** ride on
existing AC rows in the table above; none is stated here as a free-floating bullet,
because a bullet in this section gets no ledger row and is never gated.

**State-family invariants, and the AC that carries each**

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **In-place, not a context jump** — the author running the gate stays in the same command. No new step, no new job, no Docker daemon to start, no credential to fetch, no second tool to install. | AC-009 | `git diff --stat` over `xtask/src/main.rs` and `.github/workflows/ci.yml` is empty; `cargo xtask ci --fast` green with no infrastructure. |
| **Non-occlusion** — a failure names which of the two artefacts moved rather than reporting "they differ", and the packaging failure names the file *and* the remedy. One fault never hides the other: `reconcile` accumulates both directions into `faults` before bailing (`xtask/src/package.rs:186-211`) and `run` collects every missing file across every crate (`:106,143-158`). | **AC-004**, **AC-005** | Both failure directions are provoked and their messages transcribed as ledger evidence. |
| **Preserved selection** — the identity CI selects steps by is the step `name` string; `steps_named` panics on a miss (`xtask/src/main.rs:771-816`). Renaming "wasm32 build of the Neon adapter" while tidying is a gate change wearing the clothes of a cosmetic one. | AC-009 | Zero changed lines in `xtask/src/main.rs`. |
| **Reversibility** — everything this story lands is revertible in the tree: two manifest lines, one const, six files, two attributes. Nothing irreversible is done to a registry; `cargo publish` is HS-P0016's and is excluded by the PR boundary. | **AC-003**, **AC-004** | The PR boundary block, enforced by `redkiln verify --grain story`; no crates.io write appears in the diff. |
| **Keyboard reachability** | **N/A** | No interactive surface exists to be reached (`_design.md`, `## Surfaces`). |

**Composition-family analogues that do bind, and the AC that carries each**

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **Presentation exists at all** — in this medium, the composed presentation a consumer meets is the README and the crate-level rustdoc. The bare-markup failure has an exact analogue and it is already written: `xtask/src/reserve.rs:243-269`'s placeholder README, which is real markup, renders perfectly, and says the crate contains no functionality. Shipping that is the unstyled render passing every assertion. | **AC-006**, **AC-002**, **AC-007** | Human read against `crates/happenstance-testkit/README.md` and `crates/happenstance/README.md`; `rg` for the placeholder sentence returning nothing; the documentation step proving the rustdoc builds. |
| **Density budget, in this story's real units** — three files per crate directory (`REQUIRED_FILES`, `xtask/src/package.rs:94`), five names in `PUBLISHABLE` (up from three), **zero** `todo!()` invocations (down from 34 across the two `src/` trees), **zero** `#![allow(clippy::todo)]` (down from two), one `#[expect(dead_code)]` removed. Every one of those numbers is asserted, not eyeballed. | **AC-001**, **AC-004**, **AC-005** | `cargo xtask package-check`; the clippy step; `rg` counts. |
| **Hierarchy** — the Hyperdrive note is subordinate prose beside "# Not the Neon adapter", not a top-level section that reads as a supported deployment mode. Placement is decided, and a different placement is recorded rather than defaulted. | AC-007 | Review of the rendered rustdoc heading level and surrounding section. |
| **Named anti-patterns** — the design named none (N/A), so the binding ones are this repository's own: a check that cannot fail (`CLAUDE.md`, *the rule that matters*; `xtask/src/package.rs:13-18,29-31`), a suppression without an expiry (`standards/rust/90-skeletons-and-todo.md` RS-90-3), and a skeleton wearing a different hat — `todo!()` traded for `unimplemented!()` or an unexercised `Err(…)` (Context pack item 7). | **AC-001**, **AC-004** | The clippy step; the two provoked `reconcile` failures; review against RS-90-3. |

## Error conditions

| id | condition | required behaviour |
| --- | --- | --- |
| **EC-001** | A manifest loses `publish = false` while `PUBLISHABLE` still names three crates. | `reconcile` bails: *"Cargo will publish … but this step does not check it. If the crate was promoted on purpose, copy LICENSE-MIT, LICENSE-APACHE, README.md into its directory and add its name to PUBLISHABLE…"* (`xtask/src/package.rs:188-196`). The gate fails; the remedy is the other half of the change, never a suppression. |
| **EC-002** | `PUBLISHABLE` grows while a manifest still carries `publish = false`. | `reconcile` bails the other way: *"PUBLISHABLE names … but Cargo will not publish it — the crate has gained a `publish = false`, or the name here is stale"* (`:198-204`). Two directions, two remedies, deliberately distinguished (`:163-167`). |
| **EC-003** | A required file is absent from a packaged artifact — including the subtle case where the file exists at the workspace root but not inside the crate directory. | `run` collects every absence across every crate and bails once, naming each and the remedy: *"Copy the file into the crate directory — Cargo will not follow a path outside it"* (`xtask/src/package.rs:151-158`). A symlink or a `readme` pointing outside the directory is this condition, not a fix for it. |
| **EC-004** | A `todo!()` survives in either crate after the scoped allow is deleted. | Hard build failure at the clippy step under `-D warnings` — the intended mechanism, not an accident. **Blocking finding.** Route it to the owning story (`postgres-append-and-frontier-head`, `postgres-projection-store`, `neon-append-and-read-over-http`, `neon-conflicting-position-verdict`) or to the `support` initiative. Restoring the allow, deleting the body, or replacing it with `unimplemented!()`/`panic!`/an unexercised `Err(…)` are all failures of AC-001. |
| **EC-005** | The `#[expect(dead_code)]` on `AppendOutcome` is left in place once `decode_append_response` constructs it. | `unfulfilled_lint_expectation` fires and `-D warnings` turns it into a gate failure. This is `expect`'s whole purpose (`standards/rust/90-skeletons-and-todo.md` RS-90-3); the remedy is deleting the attribute, never converting it to `#[allow]`. |
| **EC-006** | `cargo deny` reports a licence-allowlist failure in a graph that was previously excusable as an unpublishable instrument — the known live candidate being `webpki-roots` (`CDLA-Permissive-2.0`) reached through `sqlx`'s `tls-rustls` (`crates/happenstance-postgres/src/lib.rs:40-47`). | **Blocking finding, reported not fixed.** `deny.toml:9-19` is not widened in this story. The options — take roots from the platform trust store, or grow the allowlist by decision record — are a decision this story records and does not take. |
| **EC-007** | Either crate name turns out not to be held on crates.io when AC-008's check runs. | Stop before AC-003. Do not flip `publish`, do not run `cargo xtask reserve` here (that is `claim-crate-names`'), and do not proceed and report afterwards — the whole point of the dependency edge is that the unheld-name window never opens. |
| **EC-008** | `cargo package -p <crate> --list --allow-dirty --locked` fails outright rather than listing files. | `run` bails with cargo's own stderr attached (`xtask/src/package.rs:116-122`). The likely causes are a path dependency without a `version` key or a lockfile that `--locked` rejects; note that `happenstance-core` already carries `version = "0.2.0"` at `Cargo.toml:24`, so the contract dependency resolves. A `dev-dependency` on `happenstance-testkit` is not packaged content and does not gate `--list`. |
| **EC-009** | An existing gate step's `name` string is changed while tidying. | `steps_named` panics on the miss (`xtask/src/main.rs:771-816`) — a loud failure, and the correct one. Revert the rename; `wasm_steps()` and the CI job matrix both select by that string. |

## Non-functional

| id | requirement | bound / evidence |
| --- | --- | --- |
| **NF-001** | The gate gets no slower in a way anyone notices. | `package-check` grows from three to five iterations of `cargo package --list --allow-dirty --locked` — two extra process launches on a step that already launches four, with no compilation and no dependency resolution beyond what `--locked` reads. No new step, so `cargo xtask ci --fast`'s step count is unchanged (AC-009). |
| **NF-002** | No dependency is added, removed or re-featured. | `crates/happenstance-postgres/Cargo.toml:22` keeps `sqlx` at `["postgres", "runtime-tokio"]` with **no** `migrate` — that flag is Architecture §9.3's open question and the schema story's to answer. `crates/happenstance-neon/Cargo.toml`'s dependency block is untouched. `[features]` in both is untouched. |
| **NF-003** | The MSRV does not move. | `rust-version.workspace = true` in both manifests; the floor stays 1.97.1 per `.kb/decisions/0029-msrv-raised-to-1-97-1.md`. Nothing this story adds is a language feature. |
| **NF-004** | The `wasm32` position of `happenstance-neon` is unchanged. | The "wasm32 build of the Neon adapter" step (`xtask/src/main.rs:265-283`) stays green and keeps its name (AC-009, `project.md` DR-8). Removing a lint attribute and editing docs cannot change target support, and the step is the standing proof of that. |
| **NF-005** | The licence texts are the workspace's, exactly. | `crates/happenstance-postgres/LICENSE-MIT` and `LICENSE-APACHE` and their Neon counterparts are byte-identical to the workspace-root `LICENSE-MIT` / `LICENSE-APACHE`. Two copies that have drifted are two licences, which is the failure the dual-licence promise cannot survive. |
| **NF-006** | The public API surface is unchanged, so a semver diff across this story is empty. | `_design.md` `## Items` is N/A; this story touches manifests, lint attributes, docs and packaging metadata only. This is what makes it safe to land last in the project and is the precondition HS-P0016's `semver` job inherits. |
| **NF-007** | Documentation builds green in both feature configurations the gate checks. | The documentation step (`xtask/src/main.rs:284-302`) and the `--no-default-features` doc build of `happenstance-core` (`:494-514`) both pass; the rewritten crate-level prose must not introduce an intra-doc link that resolves only under one feature set. |

## Implementation notes (non-prescriptive)

Not instructions — the traps this story is likely to fall into, and where the
in-tree answer already is.

**Do the three-part change as one commit.** The manifest, `PUBLISHABLE`, and the
six files are a single atomic edit by construction: any two of the three leave the
gate red (EC-001, EC-002, EC-003). Provoking each failure *deliberately* before
completing the change is worth the two minutes, because it is the only evidence
AC-004 can honestly carry — a green gate proves the pair agrees, not that it would
have noticed disagreeing.

**Copy the licences with a copy, and check them with `git diff --no-index`.** The
three publishable crates already carry theirs; look at
`crates/happenstance-testkit/` and `crates/happenstance/` for the layout rather
than inventing one. On Windows a `git config core.symlinks` accident can silently
turn a copy into something Cargo will not package — the `--list` assertion catches
it, which is why the assertion is the deliverable (`xtask/src/package.rs:4-18`).

**Write the READMEs against what the crate now honestly is.** The Neon crate's own
rustdoc already contains the material (`crates/happenstance-neon/src/lib.rs:10-29`
holds the capability framing), and `postgres-structural-bill` has by then written
the frontier-`head`/no-RYOW/staleness prose that the Postgres README should point
at rather than paraphrase. Duplicated prose in a README and a rustdoc is two things
to keep true; prefer a short front page that names the limit and links the long
form.

**Rewriting the crate-level rustdoc is a deletion of two paragraphs, not a
rewrite of the file.** Both crates open with a `# Status: not implemented` section
and then continue with material that is still true and still valuable — the TLS
measurement, the "# Not the Neon adapter" section, the capability table, the CTE
discussion. `standards/rust/70-rustdoc-obligations.md` is the atom to pull if the
replacement prose needs shaping; do not load the corpus.

**The `#[expect(dead_code)]` will tell you when it is time.** If removing it
produces a `dead_code` warning rather than nothing, `decode_append_response` is
not actually constructing both variants and one of the depended-on stories left
something undone — that is EC-004's neighbour and routes the same way.

**Verify the crate names, cheaply.** AC-008 needs a recorded check, not a
re-reservation. The `claim-crate-names` story's ledger is the first place to look;
a fetch of each name's registry page is the second. Do not run `cargo xtask
reserve` from this story — it publishes a `0.0.0` placeholder, which is exactly the
claim AC-006 exists to retract.

**If `cargo deny` goes red, stop and write it down.** The licence question is a
decision this repository takes by ADR, and this spec has no authority to widen an
allowlist. Record what failed, through which feature path, and hand it to the
runbook's ADR pass.

## Tests and CI (merge gate)

Grounded in the testing brief (`_decomposition.md:478-665`), whose AC-012 row names
`cargo xtask package-check`, `cargo clippy -D warnings` and a `todo!()` grep as
this story's three instruments. Every command below already exists; this story adds
no tier, no job and no test target — which is itself AC-009.

| tier | command / path | proves |
| --- | --- | --- |
| **Static / gate (story bar)** | `cargo xtask ci --fast` — `.redkiln/config.yaml` `verify.integration_scoped`, the non-terminal project bar | The whole story, end to end, on a clean checkout with no Docker, no network and no credentials. **AC-009**, and the umbrella under which AC-001 – AC-005 fail if they are wrong. |
| **Static / gate step** | `cargo xtask package-check` → `xtask/src/main.rs:515-535` → `xtask/src/package.rs:103-161` | `reconcile` agrees in both directions; `cargo package --list` succeeds for five crates and each listing carries all three of `REQUIRED_FILES`. **AC-003, AC-004, AC-005.** |
| **Static / gate step** | clippy, `xtask/src/main.rs:115-130` (`--workspace --all-targets --all-features -- -D warnings`) | No `todo!()` survives once the two crate-scoped allows are deleted; no `#[expect]` outlives its reason. **AC-001, AC-002.** |
| **Static / gate step** | documentation, `xtask/src/main.rs:284-302` | The rewritten crate-level prose, the Hyperdrive note and its cross-reference all build with no broken intra-doc link. **AC-002, AC-006, AC-007, NF-007.** |
| **Unit** | `cargo test -p xtask` → `xtask/src/package.rs:408-457` (`reads_every_publish_spelling`, `ignores_names_that_are_not_workspace_members`, `rejects_a_publish_value_it_does_not_understand`) | The metadata scanner still reads `publish` correctly as two members change spelling from `[]` to `null` — the exact transition these tests were written for. Regression guard, unchanged by this story. |
| **Static / process** | `cargo xtask spec-trace` — `.redkiln/config.yaml` `verify.reachability_static` | No clause, marker or `file:line` citation moved. The far-end discharge belongs to the slice-mate. **AC-009.** |
| **Static / supply chain** | `cargo deny` step, `xtask/src/main.rs:595-601` (`OPTIONAL`, reached by full `cargo xtask ci`) | Both graphs — already walked, `deny.toml:1-2` sets `all-features = true` — still satisfy the allowlist now that a failure can no longer be excused. **AC-009, EC-006.** |
| **Story grain** | `cargo xtask affected --base main` — `.redkiln/config.yaml` `verify.affected_gate` | Only what this diff could break is rebuilt and retested; combined with the fenced PR boundary, that nothing outside the four declared globs changed. |
| **Ledger** | `redkiln verify --grain story` against `_ledger.md` (`verify.require_ledger: true`) | Every AC-### above is present, `satisfied: true`, and carries non-placeholder cited evidence. This is the gate that makes "the gate is green" insufficient on its own. |
| **Human read** | The two READMEs, the two rustdoc front pages, the two registry pages | Content, not existence. Initiative **DoD 10** is explicit that this one is "checked by looking at them" (`initiative.md:387-389`). **AC-006, AC-007, AC-008.** |
| **Conformance (live, not owned here)** | `cargo test -p happenstance-postgres --all-features -- --ignored --show-output`; the same for `-p happenstance-neon` — the two project-level CI jobs | Regression guard only. This story changes no store behaviour, so both live jobs must be exactly as green after it as before. A change here is a defect in this story, not a finding about the adapters. |

## Risks and coupling (PR-scoped)

| risk | why it is live in *this* PR | containment |
| --- | --- | --- |
| **A `todo!()` survives and the pressure is to restore the allow.** | This story's whole value is deleting the suppression; a red build at the last story of the project is the moment discipline is cheapest to lose. | EC-004 makes it a blocking finding with a named routing target. The four `depends_on` edges exist so this cannot be discovered here first; if it is, the dependency was declared satisfied prematurely. |
| **`cargo deny` fails on a dependency the transport work took.** | `neon-sql-transport` and the Postgres TLS story were free to take dependencies while both crates were unpublishable instruments. This is the PR where that stops being free. | EC-006: report, do not widen. The failure is a real finding about the release, and HS-P0016 would have found it later at higher cost. |
| **The three-part change lands as two parts in review.** | It is genuinely three files in three directories, and a partial revert during rebase produces a red gate whose message points at the *other* half. | `reconcile`'s two-direction message is the containment and is why it was written that way (`xtask/src/package.rs:163-167`). Land as one commit. |
| **The README is written from the reservation template.** | `xtask/src/reserve.rs:243-269` is the nearest README-shaped thing in the repository and it is the exact wrong one. | AC-006 names the sentence that must not appear and names the two right exemplars. |
| **`postgres-projection-store` slides right and takes this story with it.** | That story is gated on HS-P0010 (`projection-store-freeze`) freezing the `Batch` shape — a project-level dependency, not a story edge (`_storymap.md` merge order, step 5). If it slips, `todo!()` bodies remain in `crates/happenstance-postgres/src/projection_store.rs` and AC-001 cannot be met. | Declared as a `depends_on` edge so the slip is visible rather than discovered. Do **not** partially de-skeleton one crate to make progress: `PUBLISHABLE` growing to four is a state nothing in this project's plan describes, and `package-check` would then start asserting a surface for a crate that still has `todo!()` in it. |
| **Tidying spreads past the boundary.** | Both `lib.rs` files are being edited anyway, and `xtask/src/main.rs` is one hop from the mount point. | The fenced PR boundary excludes `xtask/src/main.rs` explicitly, and EC-009 explains what a "harmless" rename there actually costs. |
| **Coupling to the slice-mate.** | `far-end-discharge-record` runs in the same context and reports on what these crates discharged. It reads this story's result; it must not be allowed to write into this story's files. | Order within the slice is this story first (`_storymap.md:262-265`), and the two PR boundaries are disjoint: nothing under `spec/` appears in this one. |

## Dependencies

**Blocks on** — all four are hard, and each for a different reason:

- **`claim-crate-names`** — the names must be *held* before `publish = false` comes
  out; AC-008 verifies rather than assumes it (EC-007).
- **`postgres-structural-bill`** — the last Postgres story in merge order
  (`_storymap.md` step 3). Its frontier-`head`/no-RYOW/staleness prose is what the
  Postgres README's "what it cannot do" section points at (AC-006), and it
  transitively carries `postgres-append-and-frontier-head`,
  `postgres-concurrency-family`, `postgres-rule-controls` and
  `adr-0024-position-visibility-mechanism` — the stories that removed the event-store
  `todo!()` bodies AC-001 needs gone.
- **`neon-conflicting-position-verdict`** — the last Neon story in merge order. It
  is where `decode_append_response` finally constructs `AppendOutcome`, which is
  what makes the `#[expect(dead_code)]` unfulfilled and therefore removable
  (AC-002, EC-005), and it carries `neon-append-and-read-over-http`,
  `neon-fixture-and-live-job` and `neon-sql-transport` behind it.
- **`postgres-projection-store`** — not covered by the Postgres chain above: it
  hangs off `postgres-schema-and-live-fixture` and is gated on HS-P0010. It owns
  the `todo!()` bodies in `crates/happenstance-postgres/src/projection_store.rs`,
  which AC-001 counts.

**Unlocks**

- **`far-end-discharge-record`** (slice-mate, same context, immediately after) —
  it writes the ES-10/ES-11/ES-12/ES-41/ES-42/VT-21 – VT-24 status that this story
  makes true, and traces to project AC-013.
- **`publication-and-positioning` (HS-P0016)**, at the initiative level — the
  actual `cargo publish`, the version decision, the semver diff against the prior
  baseline, the clause-ledger audit, and the human look at the rendered registry
  page that closes initiative DoD 10. This story is the readiness half of that and
  none of the release half.

## Anchors (progressive disclosure)

Link, do not paste. Each of these is deferred on purpose — the Context pack above
carries the decisions; these carry the detail that would not survive summarising.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `xtask/src/package.rs` | The mount point and, in its 75-line module header, the full argument for why the derived set *and* the hand list both exist and why the failure names a direction. `PUBLISHABLE` is `:86`, `REQUIRED_FILES` `:94`, `run` `:103-161`, `reconcile` `:172-218`. | Before touching `PUBLISHABLE` — first edit of the story. | AC-004, AC-005 |
| `xtask/src/reserve.rs` | Holds the two crate descriptions the real crates are meant to carry (`:97-110`), the record that both names are this phase's claims (`:99,105`), and — as the thing to avoid — the `0.0.0` placeholder README generator (`:243-269`). | Before rewriting either `description` (AC-002) and again before writing either README (AC-006). | AC-002, AC-006, AC-008 |
| `xtask/src/main.rs` | The step list. `REQUIRED` at `:105`, the `package-check` step at `:515-535`, clippy at `:115-130`, docs at `:284-302`, `cargo deny` at `:595-601`, and `steps_named`/`wasm_steps` at `:771-816` — the by-name selection that turns a rename into a gate change. | Read before assuming a step must be added; re-read if tempted to rename anything. Never edited by this story. | AC-009 |
| `.bklg/from-contract-to-published-library/postgres-and-neon-stores/_decomposition.md` | The three briefs. Architecture's *Root C, the gate* at `:145-157`; the testing brief's AC-012 row and the live-job gating this story must not disturb at `:478-665`; the deployment brief's AC-012 row, the Hyperdrive row and "readiness, not release" at `:684-787`. | The deployment section before AC-007; the testing section before claiming any tier. | AC-007, AC-009 |
| `.bklg/from-contract-to-published-library/postgres-and-neon-stores/project.md` | The charter. AC-012 verbatim at `:271-273`, AC-011's Docker-free bar at `:267-270`, DR-9 at `:212`, and the *Out of scope* block at `:128` that keeps publishing out of this project. | When a scope question arises — it wins over any brief. | AC-009 |
| `.bklg/from-contract-to-published-library/initiative.md` | The four personas at `:200-224` and their journeys at `:243-250`, which every AC above is framed from, and DoD 10 at `:387-389` — the sentence this story builds the artefact for and HS-P0016 checks by looking. | Before writing either README, to write for the evaluator's one look rather than for a maintainer. | AC-006 |
| `.bklg/from-contract-to-published-library/postgres-and-neon-stores/_design.md` | The signed-off determination that this project has no user-facing surface, slot by slot. It is what makes the composition family N/A rather than skipped, and it is binding. | If any question about surfaces, screens or perceptual review arises. | Interaction quality |
| `standards/rust/90-skeletons-and-todo.md` | RS-90-3, *scope every skeleton suppression to the crate and give it an expiry* — the rule both `#![allow(clippy::todo)]` lines and the `#[expect(dead_code)]` were written under, and the reason removal is a compiler event rather than a tidy-up. | Before deleting the attributes, and if any suppression seems worth keeping. | AC-001, AC-002 |
| `standards/rust/70-rustdoc-obligations.md` | What crate-level documentation owes a reader in this repository. The `# Status: not implemented` sections are being replaced, not deleted, and this is the shape the replacement is held to. | While rewriting either crate's crate-level prose. | AC-002, AC-006 |
| `crates/happenstance-postgres/src/lib.rs` | The prose being reconciled: `# Status: not implemented` at `:1-8`, the measured TLS/`webpki-roots` finding at `:40-47`, "# Not the Neon adapter" at `:49-56` (where the Hyperdrive note goes), the scoped allow at `:59-63`. | Open at the start of the crate-prose work; the TLS paragraph again if `cargo deny` goes red. | AC-001, AC-002, AC-007 |
| `crates/happenstance-neon/src/lib.rs` | The Neon capability framing at `:10-29` — no connection, no interactive transaction, no cursor — which is the raw material for the README's "what it cannot do", plus the scoped allow at `:102-105`. | Before writing the Neon README and before deleting its allow. | AC-001, AC-006 |
| `crates/happenstance-neon/src/event_store.rs` | The `#[expect(dead_code)]` on `AppendOutcome` at `:232-247`, whose `reason` names phase 10 and whose removal this story collects. | At the moment `decode_append_response` is confirmed to construct both variants. | AC-002 |
| `crates/happenstance-testkit/README.md` and `crates/happenstance/README.md` | The two in-tree exemplars of a publishable crate's front page — the register, length and structure to match rather than improve on. | Immediately before writing either new README. | AC-006 |
| `crates/happenstance-core/Cargo.toml` | `:12-15` states why `readme = "README.md"` is declared explicitly even though Cargo would infer it. The reasoning is copied, not re-derived. | While editing either manifest. | AC-003 |
| `Cargo.toml` (workspace root) | `version = "0.2.0"` at `:6`, and `happenstance-core = { version = "0.2.0", … }` at `:24` — the reason neither crate needs a version key and the reason `cargo package --list` will not reject the path dependency. | If tempted to add a `version` key, or if EC-008 fires. | AC-003 |
| `deny.toml` | `all-features = true` at `:1-2` and the licence allowlist at `:9-19`. Establishes that both graphs are already walked, so what changes is the excuse and not the coverage. | Only if `cargo deny` fails — to confirm the finding, never to edit. | AC-009 |
| `RUNBOOK.md` | Phase 10 at `:4344-4390`: the work items, the crates.io claim rule at `:4346-4348`, the Hyperdrive note's exact wording at `:4364-4366`, and the exit line "No `todo!()` on either path; `publish = false` removed" at `:4383`. | Before AC-007 and AC-008; and at the end, to read the exit criteria against what landed. | AC-007, AC-008 |
| `.redkiln/config.yaml` | The `verify:` block — `integration_scoped`, `affected_gate`, `reachability_static`, `require_ledger`, `require_commit_provenance` — the commands that run whether or not anyone types them, and the deliberate absence of `design.capture`. | Before running anything, to run what the gate will run. | AC-009 |
| `.bklg/from-contract-to-published-library/postgres-and-neon-stores/claim-crate-names/` | The upstream story's own artifacts — where AC-008's verification evidence comes from. | At the very start: if this is not landed, the story stops (EC-007). | AC-008 |

## Clarifications resolved during spec

1. **The AC set is exactly the nine the first pass enumerated** — AC-001 through
   AC-009. None added, none dropped. The `_ledger.md` rows match one-for-one.
2. **Where the Hyperdrive note lives was left open by the deployment brief
   ("rustdoc or `README.md`", `_decomposition.md:684-787`) and is decided here:**
   `happenstance-postgres`'s crate-level rustdoc, beside the existing "# Not the
   Neon adapter" section, with a one-line cross-reference from the Neon crate.
   Reason: that section already holds this crate's "how do I reach Postgres from
   somewhere unusual" prose, and a note about an unsupported deployment reads as a
   caveat there and as a feature on a README front page. An implementer who places
   it elsewhere records why; what is not open is that it must not read as
   supported.
3. **No conformance rule is named, deliberately.** A rule observes a store through
   the `EventStore` trait and cannot see a manifest key, a lint attribute or a
   README, so there is nothing here for `happenstance-testkit` to assert. Adding
   one would be the decorative rule `CLAUDE.md` forbids. The falsifier is the
   gate — specifically `reconcile`'s two-direction failure and clippy's `-D
   warnings` — and both are provoked on purpose during implementation so AC-004
   carries evidence rather than a green run.
4. **The composition family of interaction quality is N/A by signed-off design,
   not skipped.** `_design.md` records `N/A` in every slot; the analogues that do
   bind in this repository's medium (the README and rustdoc a consumer meets) are
   carried on AC-002, AC-006 and AC-007 rather than stated as prose, so each one
   has a ledger row and is gated.
5. **Neither crate takes a `version` key.** `version.workspace = true` resolves to
   the `0.2.0` at `Cargo.toml:6`; removing `publish = false` inherits the release
   line the workspace already carries. Choosing a version is HS-P0016's, and doing
   it here would pre-empt the semver diff that decides it.
6. **A partial de-skeleton is not an acceptable fallback.** If
   `postgres-projection-store` slips, this story slips with it. Growing
   `PUBLISHABLE` to four names describes no state in this project's plan and would
   have `package-check` asserting a publishable surface for a crate that still
   contains `todo!()`.
7. **A `cargo deny` failure is a finding, not a fix.** This spec has no authority
   to widen `deny.toml`'s allowlist, and per this repository's practice an ADR is
   never written as a side effect of implementation work. Record it and hand it to
   the runbook's decision pass.
