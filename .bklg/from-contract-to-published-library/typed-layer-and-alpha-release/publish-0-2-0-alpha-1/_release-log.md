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

**Read the status line first.** The tree is cut and gated, **the publish has run** —
2026-08-16, by the repository owner, as the human handoff the story map names
(`_storymap.md:163-166`) — and **all four checklist items are now filled in**. §5.1
carries the three publish transcripts verbatim, §5.3 the outside-workspace
resolution and the write-then-read cycle, §5.4 the rendered crates.io page and §5.5
docs.rs. AC-007 is `satisfied: true` against those transcripts.

§5.6 records the one thing that **read back badly**, because a release log that only
records what worked is a press release. It is not a defect in the artefact and not a
yank: every documentation link on the rendered page points into a repository that is
not publicly reachable. It is routed, not swallowed.

| | |
| --- | --- |
| Version | `0.2.0-alpha.1` — a **pre-release** |
| Crates | `happenstance-core`, `happenstance`, `happenstance-testkit` |
| Gate | `cargo xtask ci` **whole**, green, all four `OPTIONAL` steps **ran** |
| Dry run | `cargo publish --dry-run -p happenstance-core` — green |
| Published | **yes** — all three live, 2026-08-16 21:36–21:37 UTC. See §5 |
| Published from | `f90982b`, whose tracked tree outside `.bklg/` and `.redkiln/` is **byte-identical** to the gated `952a870` |
| Read back | Resolves, builds and runs outside the workspace (§5.3) · page renders as designed (§5.4) · docs.rs green (§5.5) · **one finding**, routed (§5.6) |

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
- [x] a scratch project **outside this workspace** — its `Cargo.toml`, its
      `cargo build` output and its program's stdout — adding all three crates at
      an **explicit pre-release requirement** (`happenstance = "0.2.0-alpha.1"`;
      a bare `"0.2"` will not select a pre-release), resolving them **from the
      registry rather than from a path**, and running the smallest
      write-then-read cycle against the published `happenstance` — **§5.3**;
- [x] the rendered crates.io page, before anyone touches it again — checking that
      `## Stability` appears above the first code block and that the word *facade*
      appears nowhere — **§5.4**;
- [x] a docs.rs check within the hour. Best-effort: a docs.rs failure is worth
      knowing and is not a blocker — **§5.5**, green.

**All four are now in, and AC-007 is `satisfied: true` against them.** The three that
were outstanding are the three that involve *reading back* what was published, and none
of them is discharged by §5.1 — a successful upload proves the registry accepted a
tarball, not that a stranger can resolve it, not that the page renders as intended, and
not that docs.rs built it. Each was run separately and each is pasted below rather than
summarised.

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

### 5.3 The stranger's project — resolution, build, and the write-then-read cycle

**This is AC-007's own criterion, and it is the only one of the four that runs a
compiler.** §5.1 proves the registry accepted three tarballs. This proves a stranger can
*use* what it accepted.

**Where it was run, and why that matters.** A directory outside `D:\repos\happenstance`
entirely — this session's scratchpad on `C:` — created with `cargo new stranger-smoke`.
There is no `[workspace]` table above it, no `[patch]` section anywhere, no
`path = ` on any dependency, and no user-level `~/.cargo/config.toml` at all
(`C:\Users\ryanm\.cargo\` contains `credentials.toml` and the registry cache and no
`config.toml`), so nothing can redirect a requirement at the checkout. That absence is
the point of the exercise: a `path` override would make the whole cycle pass while
proving nothing about the registry.

**One environment caveat, stated because it is a deviation from what a stranger types.**
`CARGO_TARGET_DIR` was pointed at `D:\_stranger-smoke-target`. Linking any executable
under this machine's `%TEMP%\claude\…` tree fails with `LINK : fatal error LNK1104:
cannot open file …build_script_build-*.exe` — reproducibly, for `serde`'s build script
before any happenstance code is reached, and identically with the tool sandbox disabled.
That is a local policy on that directory, not a fact about these crates: the manifest,
the lockfile, the resolution and the sources are untouched by where the object files
land. Everything below is otherwise exactly what `cargo build` and `cargo run` printed.

**The manifest.**

```toml
[package]
name = "stranger-smoke"
version = "0.1.0"
edition = "2024"

# No `[patch]`, no `path = `, no `[workspace]` above this directory.
# Every requirement below is an explicit pre-release: a bare "0.2" never
# selects a `0.2.0-alpha.1` candidate.
[dependencies]
happenstance = "0.2.0-alpha.1"
happenstance-core = "0.2.0-alpha.1"
happenstance-testkit = "0.2.0-alpha.1"
serde = { version = "1", features = ["derive"] }
tokio = { version = "1", features = ["macros", "rt"] }
```

**The build.** Run after `cargo clean` **and** deleting `Cargo.lock`, so the resolution
below is a fresh one against the index rather than a replay of a lockfile:

```console
$ cargo clean && rm Cargo.lock && cargo build
   Compiling proc-macro2 v1.0.107
   Compiling unicode-ident v1.0.24
   Compiling quote v1.0.47
   Compiling serde_core v1.0.229
   Compiling thiserror v2.0.20
   Compiling zmij v1.0.23
   Compiling serde v1.0.229
   Compiling futures-core v0.3.34
   Compiling serde_json v1.0.151
   Compiling bytes v1.12.1
   Compiling itoa v1.0.18
   Compiling memchr v2.8.3
   Compiling pin-project-lite v0.2.17
   Compiling syn v3.0.3
   Compiling syn v2.0.119
   Compiling trait-variant v0.1.3
   Compiling thiserror-impl v2.0.20
   Compiling serde_derive v1.0.229
   Compiling tokio-macros v2.7.2
   Compiling tokio v1.53.1
   Compiling happenstance-core v0.2.0-alpha.1
   Compiling happenstance-testkit v0.2.0-alpha.1
   Compiling happenstance v0.2.0-alpha.1
   Compiling stranger-smoke v0.1.0 (C:\Users\ryanm\AppData\Local\Temp\claude\…\scratchpad\stranger-smoke)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 10.22s
```

Note what carries **no path in parentheses**: every `happenstance*` line. Cargo prints the
manifest directory beside a path or workspace member and nothing beside a registry
package, so the three lines above are the registry's copies. The compiler error messages
from the first, failing draft of this program pointed at
`C:\Users\ryanm\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\happenstance-testkit-0.2.0-alpha.1\src\contract.rs`,
which is the same fact stated by a different tool.

**The resolved lockfile names the registry**, which is the criterion's *"from the
registry rather than from a path dependency"* in the form cargo records it:

```toml
[[package]]
name = "happenstance"
version = "0.2.0-alpha.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "0d744b36377a7497b7365041dc114ed11cef2d38ab48b69ae4eceb134e351b94"

[[package]]
name = "happenstance-core"
version = "0.2.0-alpha.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "eafacd2d9d0e9370cc17e2ce81e2ccc242e3611da649f0ef9540e8827e7912ff"

[[package]]
name = "happenstance-testkit"
version = "0.2.0-alpha.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "caad8916648ca76d86d433667dfd7a3ea1335cbc14f1a361d91c98c26bdbfd0c"
```

**Stated as an exhaustive check rather than three quoted entries**, because three quoted
entries prove three things and the criterion is about the whole graph:

```console
total packages in Cargo.lock: 24
packages with NO registry source: ['stranger-smoke']
occurrences of the substring "path" anywhere in Cargo.lock: 0

$ cargo tree --depth 1
stranger-smoke v0.1.0 (…\scratchpad\stranger-smoke)
├── happenstance v0.2.0-alpha.1
├── happenstance-core v0.2.0-alpha.1
├── happenstance-testkit v0.2.0-alpha.1
├── serde v1.0.229
└── tokio v1.53.1
```

**The program.** The smallest cycle that is still a cycle: one `commit` through the
command loop, then the event read straight back out of the store and decoded through the
same codec, then an assertion that the position the reader sees is the position the
writer was handed. `happenstance-testkit` is linked and *used* rather than merely
resolved, because a dependency that is only in the lockfile has not been shown to
compile against anything.

```rust
use happenstance::bytes::Bytes;
use happenstance::{Codec, CodecError, DecisionModel, DomainEvent, EventStore, EventType};
use happenstance::{Json, MemoryEventStore, Query, ReadOptions, Retry, Tags, collect, commit};
use happenstance_testkit::{Fixture, fixtures::MemoryFixture};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
enum Seat {
    Taken,
}

impl DomainEvent for Seat {
    const EVENT_TYPES: &'static [EventType] = &[EventType::from_static("SeatTaken")];

    fn event_type(&self) -> EventType {
        Self::EVENT_TYPES[0].clone()
    }

    fn tags(&self) -> Tags {
        Tags::empty()
    }

    fn encode<C: Codec>(&self, c: &C) -> Result<Bytes, CodecError> {
        c.encode(self)
    }

    fn decode<C: Codec>(c: &C, _t: &EventType, d: &Bytes) -> Result<Self, CodecError> {
        c.decode(d)
    }
}

#[derive(Clone)]
struct Seats {
    scope: Tags,
    taken: u32,
}

impl DecisionModel for Seats {
    type Event = Seat;

    fn scope(&self) -> &Tags {
        &self.scope
    }

    fn apply(&mut self, event: Seat) {
        match event {
            Seat::Taken => self.taken += 1,
        }
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let store = MemoryEventStore::new();

    // WRITE. The command loop: read what the boundary selects, decide, append
    // conditioned on nothing new having appeared.
    let seats = Seats { scope: Tags::empty(), taken: 0 };
    let take = |_: &Seats| Ok::<_, core::convert::Infallible>(vec![Seat::Taken]);
    let done = commit(&store, seats, Retry::attempts(3.try_into()?), take).await?;
    println!("committed: position={} attempts={}", done.position, done.attempts);

    // READ. Straight back out of the store, decoded through the same codec.
    let read = collect(store.read(&Query::all(), ReadOptions::new())).await?;
    println!("read back: {} event(s)", read.len());
    for sequenced in &read {
        let event = &sequenced.event;
        let decoded = Seat::decode(&Json, event.event_type(), event.data())?;
        println!(
            "  position={} type={} decoded={:?}",
            sequenced.position,
            event.event_type().as_str(),
            decoded
        );
    }
    assert_eq!(read.len(), 1, "one commit, one event");
    assert_eq!(read[0].position, done.position);

    // The third published crate, linked and used: the reference fixture an
    // adapter author runs the conformance suite against.
    let fixture = MemoryFixture::new();
    let handle = fixture.connect().await;
    println!(
        "testkit: MemoryFixture SECOND_HANDLE supported={} head={:?}",
        MemoryFixture::SECOND_HANDLE.is_supported(),
        handle.head().await?
    );

    println!("write-then-read cycle: OK, against happenstance 0.2.0-alpha.1 from crates.io");
    Ok(())
}
```

**Its stdout**, which is what DoD 9 is actually about — a stranger installed it and it
did the thing:

```console
$ cargo run --quiet
committed: position=1 attempts=1
read back: 1 event(s)
  position=1 type=SeatTaken decoded=Taken
testkit: MemoryFixture SECOND_HANDLE supported=true head=None
write-then-read cycle: OK, against happenstance 0.2.0-alpha.1 from crates.io
```

`head=None` on the last line is not a defect and is worth a sentence: `MemoryFixture`
opens its **own** empty store, so it is right that it sees none of the commit above. That
is fixture isolation being observable from outside the testkit.

**The negative control, because a resolution that would have succeeded anyway proves
nothing about the pre-release.** A second throwaway project asking for `"0.2"`:

```console
$ cargo generate-lockfile          # with happenstance = "0.2"
    Updating crates.io index
error: failed to select a version for the requirement `happenstance = "^0.2"`
candidate versions found which didn't match: 0.2.0-alpha.1, 0.0.0
location searched: crates.io index
required by package `bare-req v0.1.0 (D:\_stranger-smoke-bare)`
help: if you are looking for the prerelease package it needs to be specified explicitly
    happenstance = { version = "0.2.0-alpha.1" }
```

This is the README's `## Stability` claim — *the pre-release suffix says it again in a
form cargo enforces* (§6) — arriving as a compiler transcript rather than as prose, and
it is why the criterion says **explicit** pre-release requirement.

### 5.4 The rendered crates.io page

Read through `https://crates.io/api/v1/crates/happenstance/0.2.0-alpha.1/readme`, which
returns crates.io's **own rendered HTML** rather than the markdown source — the
distinction matters, because AC-003 is about what a reader meets on the page, and reading
the repository's `README.md` back would have checked the input rather than the output.

**Region order, extracted from the rendered HTML in document order.** `<pre>` is the
first code block:

```html
<h1 id="user-content-happenstance"
<h2 id="user-content-which-crate-do-i-want"
<h2 id="user-content-stability"
<h2 id="user-content-what-dcb-buys-you"
<pre>
<h2 id="user-content-guarantees"
<h2 id="user-content-design"
<h2 id="user-content-licence"
```

`## Stability` renders **above the first code block**, which is the mitigation's whole
purpose: the reader meets the stability posture before the first thing they would copy.
The order is `# happenstance` → `## Which crate do I want?` → `## Stability` →
`## What DCB buys you` → `## Guarantees` → `## Design` → `## Licence`, exactly as AC-003
states it.

**The word *facade* appears 0 times on the rendered page** (`grep -c -i facade` over the
returned HTML). The blockquote was deleted rather than annotated, and the deletion
survived the round trip through the registry.

**The section as a reader sees it**, stripped of tags:

```text
Stability

The API moves until the first stable 0.2.0 — expect a small edit at each
upgrade, and pin the exact version you built against.
What changed is in CHANGELOG.md, per release, in a caller's terms.
Only one alpha resolves at a time: each is yanked when the next lands.
```

Three claims, one per bullet, no fourth — AC-004's cap, held on the rendered page and not
only in the source.

**The version metadata the page is built from:**

```json
num          = 0.2.0-alpha.1
yanked       = false
rust_version = 1.97.1
license      = MIT OR Apache-2.0
created_at   = 2026-08-16T21:36:55.886102Z
```

### 5.5 docs.rs

**Green, within the hour.** `https://docs.rs/happenstance/0.2.0-alpha.1/happenstance/`
serves the built documentation: build status successful, version `0.2.0-alpha.1`, the
crate-root page rendering from *"DCB-compliant event sourcing, with batteries."*
downward with the item pages present. EC-006 (`spec.md:469`) is **not** triggered and
there is nothing to route to HS-P0016 on this account.

That the nightly `--cfg docsrs` step in the release gate (§4) and docs.rs itself agree is
worth one line: the gate step is the prediction and this is the observation, and this is
the first release at which the two could be compared at all.

### 5.6 What read back badly — one finding, routed

**Every documentation link on the published page points into a repository a stranger
cannot open.** Checked unauthenticated, the way P4 would meet them:

```console
https://github.com/Wet-Ink-Corporation/happenstance                          404
https://github.com/…/blob/main/CHANGELOG.md                                  404
https://github.com/…/blob/main/spec/SPECIFICATION.md                         404
https://github.com/…/blob/main/.kb/decisions/0029-msrv-raised-to-1-97-1.md   404

$ curl https://api.github.com/repos/Wet-Ink-Corporation/happenstance
404 {"message":"Not Found"}
```

The repository root itself is 404 to an anonymous client, so this is **visibility, not a
wrong URL**: `Cargo.toml:19`'s `repository` field and the remote this tree pushes to are
the same string, and it will resolve for everyone the moment the repository is public.
Every link listed above is a consequence of that one fact, not four separate mistakes.

**What it costs, stated rather than minimised.** `## Stability`'s second bullet — *"What
changed is in `CHANGELOG.md`"* — is the one that hurts, because AC-004's own verifying
test names *"the `CHANGELOG.md` link resolving from the rendered page"*. A reader who
follows the mitigation's own pointer lands on a 404. The section's other two claims are
self-contained and unaffected, and the yank policy is legible on the page itself.

**Why this is not a yank, and not `0.2.0-alpha.2`.** EC-007 (`spec.md:470`) covers *"a
defect in the published `0.2.0-alpha.1` found after the fact"*, and the test it implies is
whether a **new number** fixes it. A new number does not: the links are identical strings
in `0.2.0-alpha.2` and stay 404 until the repository is public, at which point
`0.2.0-alpha.1`'s links start working with no republish at all. Yanking a crate that
resolves, builds and runs — §5.3 — to change nothing would spend the yank the next alpha
needs, which is EC-006's own reasoning about docs.rs applied to the case it did not
anticipate.

**Routed to HS-P0016.** This story's PR boundary places *"registry presentation"*
explicitly out of scope and in HS-P0016's, and the decision this finding actually asks
for — publish the repository, or point the published links somewhere that resolves while
it is private — is a repository-visibility decision this story has no licence to take.
The record it needs is this paragraph plus the transcript above, and it now has both.

**Two things this finding does not do.** It does not flip AC-003 or AC-004 back: both are
`satisfied: true` on evidence that is still true — the page's region order, the absence of
*facade*, and the section's three claims are all confirmed by §5.4's *rendered* HTML, and
a ledger row is never flipped back regardless. And it does not block AC-007, whose
criterion is resolution, the write-then-read cycle, the publish order and the pre-cut
rollback instruction — four things, all discharged, none of them about a hyperlink.

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
