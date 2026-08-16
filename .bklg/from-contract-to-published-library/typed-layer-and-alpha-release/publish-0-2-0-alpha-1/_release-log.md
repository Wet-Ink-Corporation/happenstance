---
item: "HS-S0033"
stage: implement
created: "2026-08-16"
updated: "2026-08-16"
---

# Release log — 0.2.0-alpha.1

The evidence no gate step can reach (DEP-006): a packaging dry run, a resolution
check, a rendered page, a publish transcript. Kept as it happened rather than
reconstructed, because a terminal that has scrolled is evidence that no longer
exists.

**Read the status line first.** The tree is cut and gated, and **the publish has now
run** — 2026-08-16, by the repository owner, as the human handoff the story map
names (`_storymap.md:163-166`). §5 carries the three transcripts verbatim. The
other three checklist items — the outside-workspace resolution, the rendered
crates.io page and the docs.rs check — are **not yet done**, and AC-007 stays
`satisfied: false` until they are in this file.

| | |
| --- | --- |
| Version | `0.2.0-alpha.1` — a **pre-release** |
| Crates | `happenstance-core`, `happenstance`, `happenstance-testkit` |
| Gate | `cargo xtask ci` **whole**, green, all four `OPTIONAL` steps **ran** |
| Dry run | `cargo publish --dry-run -p happenstance-core` — green |
| Published | **yes** — all three live, 2026-08-16 21:36–21:37 UTC. See §5 |
| Published from | `f90982b`, whose tracked tree outside `.bklg/` and `.redkiln/` is **byte-identical** to the gated `952a870` |

---

## 1. Rollback, written before the cut

**This section is dated before the publish transcript on purpose.** After an
irrevocable act there is nothing left to write the instruction against, and the
person who needs it will be under time pressure.

**For whoever cuts `0.2.0-alpha.2`, or has to undo `0.2.0-alpha.1`:**

> **Yank the mistake, publish the fix at a new number, never edit in place.**
>
> ```console
> cargo yank --version 0.2.0-alpha.1 happenstance
> cargo yank --version 0.2.0-alpha.1 happenstance-core
> cargo yank --version 0.2.0-alpha.1 happenstance-testkit
> ```
>
> A yank does **not** delete anything. A lock file that already names the version
> keeps resolving it; nothing new can select it. That is the whole mechanism, and
> it is why the README's `## Stability` section says *only one alpha resolves at a
> time: each is yanked when the next lands* — the policy is a promise this
> repository has to keep by hand, and this is the hand.
>
> **Yank in reverse dependency order** — `happenstance` first, then
> `happenstance-testkit`, then `happenstance-core` — so no live crate is left with
> an unresolvable requirement in the window between commands.
>
> **Never** `cargo publish` over a version. crates.io refuses it, and the reflex to
> try is what produces the next mistake.

**The one thing that cannot be rolled back** is a wrong requirement string. If
`happenstance-core` is live at `0.2.0-alpha.1` and `Cargo.toml`'s
`[workspace.dependencies]` still says `version = "0.2.0"`, then `happenstance`'s
publish fails, the fix is a **new** version number, and the tree that was gated is
no longer the tree that publishes. §2 is the check that stops this, and it is the
reason it is a criterion rather than a step.

---

## 2. The version move, and its verification

Five manifest edits, and a sixth that the spec's boundary list did not anticipate.

| File | Line | Change |
| --- | --- | --- |
| `Cargo.toml` | `[workspace.package] version` | `0.2.0` → `0.2.0-alpha.1` |
| `Cargo.toml` | `[workspace.dependencies]` ×3 | all three internal requirement strings → `0.2.0-alpha.1` |
| `crates/happenstance-testkit/Cargo.toml` | `[package] version` | `0.2.0` → `0.2.0-alpha.1`, a **literal** key (CF-32), comment preserved and extended |
| `examples/outside-projection-adapter/Cargo.toml` | ×2 | `0.2.0` → `0.2.0-alpha.1` — **forced**, see below |
| `Cargo.lock` | | regenerated |
| `experiments/wire-format/Cargo.lock` | | regenerated from that workspace's own directory |

**The sixth edit was forced, and finding it is exactly what §2 exists for.**
`examples/outside-projection-adapter/Cargo.toml` carries its requirements
long-hand on purpose — it models an outside author's manifest — and a `"0.2.0"`
requirement does not match a `0.2.0-alpha.1` candidate. The whole workspace stopped
resolving:

```console
$ cargo metadata --format-version 1
error: failed to select a version for the requirement `happenstance-core = "^0.2.0"`
candidate versions found which didn't match: 0.2.0-alpha.1
required by package `outside-projection-adapter v0.2.0-alpha.1`
help: if you are looking for the prerelease package it needs to be specified explicitly
    happenstance-core = { version = "0.2.0-alpha.1" }
```

That is EC-001's failure class arriving **early and locally** instead of late and
on the registry. The example is `publish = false`, so no published artefact
changes.

**The assertion, after the fix.** No internal requirement is left on `^0.2.0`:

```console
$ cargo metadata --format-version 1 --no-deps | <filter internal deps>
internal requirements not on the pre-release: NONE
$ cargo metadata --format-version 1 > /dev/null ; echo $?
0
```

**Arithmetic check.** `rg -n '0\.2\.0"' Cargo.toml crates/*/Cargo.toml
examples/*/Cargo.toml xtask/Cargo.toml` returns only **comment** lines that quote
the old string while explaining the move — no requirement, no version key. The
`version = "0.2.0"` in `standards/rust/50-dependency-hygiene.md:64` is an
illustrative TOML block, deliberately out of scope, and is untouched.

---

## 3. Pre-publish dry run

`cargo publish --dry-run -p happenstance-core`, run against the release tree. The
`--allow-dirty` flag is present only because the run was taken before the
checkpoint commit; the packaged content is the committed content.

```console
    Updating crates.io index
   Packaging happenstance-core v0.2.0-alpha.1
    Updating crates.io index
    Packaged 26 files, 434.4KiB (123.9KiB compressed)
   Verifying happenstance-core v0.2.0-alpha.1
   Compiling happenstance-core v0.2.0-alpha.1 (target\package\happenstance-core-0.2.0-alpha.1)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.94s
   Uploading happenstance-core v0.2.0-alpha.1
warning: aborting upload due to dry run
```

**`happenstance` and `happenstance-testkit` cannot be dry-run yet, and that is
expected rather than a failure.** `--dry-run` verifies the build against the
*registry* form of each dependency, and `happenstance-core@0.2.0-alpha.1` is not
live. That unavailability is precisely why §2's requirement-string arithmetic had
to be right up front.

---

## 4. The release gate

`cargo xtask ci` **whole** — not `--fast`, which drops the two feature powersets,
`cargo deny` and the nightly `--cfg docsrs` build.

**Run on the exact tree that publishes**, with nothing uncommitted:

```console
$ git status --porcelain     # empty
$ git rev-parse HEAD
952a87032b2cc04527fdb878a85e013513a696bf
```

That SHA is the tree a `cargo publish` from this checkout packages. Only `.bklg/`
artefacts move after it, and `cargo package` ships none of them — so the artefact
gated below is the artefact published.

```
=== formatting ===                                   === specification traceability ===
=== clippy (all targets, all features) ===           === no retired rule is still live ===
=== tests ===                                        === no conformance rule reads a clock ===
=== each phase's proof artefacts ===                 === no literal position values in the suite ===
=== wasm32 build of the contract crate ===           === every conformance rule has a changelog entry ===
=== wasm32 check of the conformance harnesses ===    === every stated rule count matches the suite ===
=== wasm32 build of the Cloudflare adapter ===       === the testkit carries its own version ===
=== wasm32 build of the Neon adapter ===             === happenstance-core names serde/alloc and base64/alloc ===
=== wasm32 build of the typed layer ===              === the Rust constitution is internally consistent ===
=== documentation ===                                === the constitution's examples compile ===
=== documentation (no default features) ===          === documentation (default features) ===
=== packaged artifacts carry their licences and README ===
=== feature powerset ===                             === wasm32 feature powerset ===
=== licences and advisories ===                      === docs.rs configuration (nightly) ===

all checks passed
```

**Four of four `OPTIONAL` steps RAN**, none printed `skipped:` — `cargo hack`,
`cargo deny` and a nightly toolchain all resolve on this machine. That is the
difference between the release bar and the project bar, and it is the reason
AC-006 asks for the full run rather than `--fast`.

**Five `wasm32` sections**, the fifth being `edge-flavour-and-wasm-claim`'s typed
layer step. The crate a Workers application installs is compiled for the target it
claims before anything is published that claims it.

The two steps this story leans on by name: *packaged artifacts carry their licences
and README* (D11) and *licences and advisories* (`cargo deny check`, which found no
licence problem in `ciborium`'s subtree — the conditional the design left open for
`Cbor` resolves in favour of shipping it).

**The gate first came back red, twice, and both were repaired rather than
tolerated.** `cargo deny check bans` was already failing on this tree before the
release story began (a wildcard path dependency), fixed inside
`edge-flavour-and-wasm-claim`'s own fence; and `lint-constitution` reported two
citations whose anchors moved when this story added comment lines to `Cargo.toml`
and to the testkit's manifest, re-anchored in their own commit.

---

## 5. The handoff — what a human ran, in this order

**Executed 2026-08-16 by the repository owner**, in the session that re-entered this
slice. `cargo publish` is the one irrevocable act in the project and it was a human
step by design; the commands below are what was run, and §5.1 is what came back.

```console
# 1. core FIRST. `happenstance` cannot resolve until this is live.
cargo publish -p happenstance-core

# 2. wait for the index (usually seconds), then:
cargo publish -p happenstance
cargo publish -p happenstance-testkit
```

`happenstance-core` first is not a preference: `happenstance`'s manifest carries
`happenstance-core = { version = "0.2.0-alpha.1", … }`, and cargo resolves that
from the registry at publish time. `happenstance-testkit` has no ordering
constraint against `happenstance` — `happenstance` dev-depends on it *by path with
no version*, which cargo strips from the published manifest entirely, and that is
the whole reason the spelling is what it is (NF-006, CF-32).

**Then, before closing the terminal**, record here:

- [x] the three `cargo publish` transcripts, in the order they ran — **§5.1**;
- [ ] a scratch project **outside this workspace** — its `Cargo.toml`, its
      `cargo build` output and its program's stdout — adding all three crates at
      an **explicit pre-release requirement** (`happenstance = "0.2.0-alpha.1"`;
      a bare `"0.2"` will not select a pre-release), resolving them **from the
      registry rather than from a path**, and running the smallest
      write-then-read cycle against the published `happenstance`;
- [ ] the rendered crates.io page, before anyone touches it again — checking that
      `## Stability` appears above the first code block and that the word *facade*
      appears nowhere;
- [ ] a docs.rs check within the hour. Best-effort: a docs.rs failure is worth
      knowing and is not a blocker.

**Until those four are filled in, AC-007 is not satisfied**, and this story's
ledger says so rather than claiming a resolution nobody has observed. One of four
is in. The three that remain are the three that involve *reading back* what was
published, and none of them is discharged by §5.1 — a successful upload proves the
registry accepted a tarball, not that a stranger can resolve it, not that the page
renders as intended, and not that docs.rs built it.

### 5.1 The transcripts, as they came back

Pasted, not summarised, per this story's own ledger note: *"A `cargo publish`
transcript and a first resolution against the index are one-shot."* Recorded by the
orchestrating session, which is where the terminal was.

```console
$ cargo publish -p happenstance-core
    Updating crates.io index
   Packaging happenstance-core v0.2.0-alpha.1 (D:\repos\happenstance\.claude\worktrees\from-contract-to-published-library\crates\happenstance-core)
    Updating crates.io index
    Packaged 26 files, 434.5KiB (123.9KiB compressed)
   Verifying happenstance-core v0.2.0-alpha.1 (D:\repos\happenstance\.claude\worktrees\from-contract-to-published-library\crates\happenstance-core)
   Compiling happenstance-core v0.2.0-alpha.1 (D:\repos\happenstance\.claude\worktrees\from-contract-to-published-library\target\package\happenstance-core-0.2.0-alpha.1)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 4.52s
   Uploading happenstance-core v0.2.0-alpha.1 (D:\repos\happenstance\.claude\worktrees\from-contract-to-published-library\crates\happenstance-core)
    Uploaded happenstance-core v0.2.0-alpha.1 to registry `crates-io`
note: waiting for happenstance-core v0.2.0-alpha.1 to be available at registry `crates-io`
help: you may press ctrl-c to skip waiting; the crate should be available shortly
   Published happenstance-core v0.2.0-alpha.1 at registry `crates-io`
```

```console
$ cargo publish -p happenstance
    Updating crates.io index
   Packaging happenstance v0.2.0-alpha.1 (D:\repos\happenstance\.claude\worktrees\from-contract-to-published-library\crates\happenstance)
    Updating crates.io index
    Packaged 33 files, 387.0KiB (104.0KiB compressed)
   Verifying happenstance v0.2.0-alpha.1 (D:\repos\happenstance\.claude\worktrees\from-contract-to-published-library\crates\happenstance)
 Downloading crates ...
  Downloaded happenstance-core v0.2.0-alpha.1
   Compiling happenstance-core v0.2.0-alpha.1
   Compiling happenstance v0.2.0-alpha.1 (D:\repos\happenstance\.claude\worktrees\from-contract-to-published-library\target\package\happenstance-0.2.0-alpha.1)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.71s
   Uploading happenstance v0.2.0-alpha.1 (D:\repos\happenstance\.claude\worktrees\from-contract-to-published-library\crates\happenstance)
    Uploaded happenstance v0.2.0-alpha.1 to registry `crates-io`
note: waiting for happenstance v0.2.0-alpha.1 to be available at registry `crates-io`
help: you may press ctrl-c to skip waiting; the crate should be available shortly
   Published happenstance v0.2.0-alpha.1 at registry `crates-io`
```

```console
$ cargo publish -p happenstance-testkit
    Updating crates.io index
   Packaging happenstance-testkit v0.2.0-alpha.1 (D:\repos\happenstance\.claude\worktrees\from-contract-to-published-library\crates\happenstance-testkit)
    Updating crates.io index
    Packaged 48 files, 1.4MiB (397.6KiB compressed)
   Verifying happenstance-testkit v0.2.0-alpha.1 (D:\repos\happenstance\.claude\worktrees\from-contract-to-published-library\crates\happenstance-testkit)
   Compiling happenstance-core v0.2.0-alpha.1
   Compiling happenstance-testkit v0.2.0-alpha.1 (D:\repos\happenstance\.claude\worktrees\from-contract-to-published-library\target\package\happenstance-testkit-0.2.0-alpha.1)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.14s
   Uploading happenstance-testkit v0.2.0-alpha.1 (D:\repos\happenstance\.claude\worktrees\from-contract-to-published-library\crates\happenstance-testkit)
    Uploaded happenstance-testkit v0.2.0-alpha.1 to registry `crates-io`
note: waiting for happenstance-testkit v0.2.0-alpha.1 to be available at registry `crates-io`
help: you may press ctrl-c to skip waiting; the crate should be available shortly
   Published happenstance-testkit v0.2.0-alpha.1 at registry `crates-io`
```

**The publish order proved itself rather than merely being asserted.** §5's ordering
argument was written before the cut; the middle transcript is the observation that
confirms it. Verifying `happenstance` printed `Downloaded happenstance-core
v0.2.0-alpha.1` and compiled against it — cargo resolved the requirement **from the
registry**, not from the path, which is only possible because core went first. Had
`[workspace.dependencies]` still said `version = "0.2.0"`, that line is where it
would have failed, with core already permanently live. §2 is the check that made it
a non-event.

`happenstance-testkit` compiled `happenstance-core` and **not** `happenstance`,
confirming NF-006/CF-32 from the other side: cargo strips the versionless path
dev-dependency from the published manifest, so the testkit carries no ordering
constraint against `happenstance` at all.

### 5.2 Index state immediately after, and what it does not prove

Read from the sparse index (`https://index.crates.io/ha/pp/<crate>`) minutes after
the third publish. This is a **registry read**, and it is deliberately filed apart
from the checklist: it is not the outside-workspace resolution AC-007 asks for,
because it never invokes the resolver and never builds anything. It is recorded
because two facts in it are one-shot-adjacent and worth pinning.

| Crate | Versions live | `rust_version` | `pubtime` |
| --- | --- | --- | --- |
| `happenstance-core` | `0.0.0`, **`0.2.0-alpha.1`** | `1.97.1` | `2026-08-16T21:36:30Z` |
| `happenstance` | `0.0.0`, **`0.2.0-alpha.1`** | `1.97.1` | `2026-08-16T21:36:55Z` |
| `happenstance-testkit` | `0.0.0`, **`0.2.0-alpha.1`** | `1.97.1` | `2026-08-16T21:37:19Z` |

The `0.0.0` rows are the 2026-08-06 name reservations, untouched and unyanked.

Two entries in the published metadata are the shipped form of decisions this project
argued for, now fixed on the registry and no longer editable:

- `happenstance`'s dependency list carries `happenstance-core` at
  `req: "^0.2.0-alpha.1"` — the requirement string §2 exists to get right, in the
  form the registry will hand every future resolver.
- `happenstance-testkit`'s dependency list contains **no entry for `happenstance`**.
  The dev-dependency was stripped, exactly as NF-006 predicted.

The MSRV shipped as `1.97.1` on all three, which is ADR-0029's floor arriving on a
published artefact for the first time — the event ADR-0004's **provisional** marker
is scheduled to lose at phase 12. §6's *No MSRV promise* still stands: the number is
in the metadata, and turning it into a supported-versions promise is HS-P0016's.

---

## 6. What this release does not promise

Recorded here because a published artefact is where promises are read into
existence whether or not anyone made them.

- **No MSRV promise.** The README states the floor and CI checks it; turning that
  into a supported-versions promise is HS-P0016's.
- **No `cargo-semver-checks` verdict.** This release **creates the baseline** a
  later run diffs against. There is nothing to diff against yet, which is the
  point of cutting it.
- **No stable API.** The `## Stability` section says so in three lines, and the
  pre-release suffix says it again in a form cargo enforces: a bare `"0.2"`
  requirement does not select this version.
- **No `happenstance-macros`.** Measured and answered *out* by the slice-mate;
  `references/evaluation/phase-7-macros-verdict.md` carries the classification.
- **No execution on an edge runtime.** The fifth `wasm32` step is a `cargo check`.
  Running the typed layer's tests on Workers is HS-P0013's and initiative DoD 4's.
