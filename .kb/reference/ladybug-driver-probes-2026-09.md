---
id: kb-reference-ladybug-driver-probes-001
title: What lbug 0.20.3 actually does — the four probes ADR-0025 could not be written without
kind: reference
status: accepted
authority_tier: note
summary: >-
  The four driver probes run on 2026-09-08 against lbug 0.20.3 on
  x86_64-pc-windows-msvc, rustc 1.97.1, out of the workspace and out of the gate.
  P0: a second Database::new on one directory is refused (IO exception, Error:
  33) while two Connections over one Arc<Database> both succeed and connection B
  observes a write connection A committed — the out-of-connection observability
  ADR-0025's Arc<Database>-plus-second-Connection shape is only visible through.
  P1: UINT64 round-trips u64::MAX - 1 exactly, so SequencePosition's NonZeroU64
  needs no narrowing. The build story is most of the finding: build.rs tries a
  prebuilt download first and only falls back to cmake, what it downloads is a
  single 1,444,941,838-byte static archive, the link then fails on libssl.lib
  because link_openssl() emits its link directives unconditionally with no
  feature or environment variable to turn it off, and a user-space vcpkg build
  of openssl:x64-windows-static-md took 5.6 minutes. And the trap worth more
  than the fix: lbug's build.rs emits no cargo:rerun-if-env-changed for
  OPENSSL_DIR, so setting it after a failed build changes nothing until
  cargo clean -p lbug. Recorded with two caveats the numbers do not carry
  themselves — the cited raw output
  experiments/ladybug-driver-probes/results/probe-output.txt is not in the
  tree, ignored by .gitignore:69's *-output.txt pattern, so the figures here
  are the README's verbatim transcription rather than a citable log; and the
  file-lock refusal is one run on one platform, which cannot distinguish an
  engine property from a Windows one.
depends_on: []
related:
  - kb-decision-0017
  - kb-open-question-reset-refusal-declension-001
  - kb-decision-0025
  - kb-open-question-experiment-raw-output-ignored-001
source_paths:
  - .kb/_intake/2026-09-08-adr-0025-ladybug-projection-adapter.md
  - experiments/ladybug-driver-probes/README.md
  - experiments/ladybug-driver-probes/probes.rs
last_reviewed: 2026-09-09
---

# What lbug 0.20.3 actually does — the four probes ADR-0025 could not be written without

## What this is a pointer to

Four standalone probes against the real `lbug` 0.20.3 driver, run 2026-09-08 on
`x86_64-pc-windows-msvc` with rustc 1.97.1, outside the workspace and outside
the gate, in `experiments/ladybug-driver-probes/`. ADR-0025's checkpoint type,
its handle-sharing shape, and its off-by-default feature gate each rest on one
of these; this atom is the citable summary a decision is not allowed to carry
itself.

## P0 — a second `Database::new` is refused; a second `Connection` is not

Opening a second `Database::new` on a directory an existing `Database` already
holds is refused outright: an IO exception carrying `Error: 33`, the engine's
own file-lock code. Opening a second `Connection` over one shared
`Arc<Database>` succeeds, and — the load-bearing half — a write committed on
connection A is observable on connection B with no hand-off beyond the shared
`Arc`. That second fact is the only reason `Arc<Database>` plus a second
`Connection` is a viable shape for concurrent access at all; a design that
tried to serialize through a second `Database` handle instead would have hit
the lock on its first line.

Read narrowly: this is one process, one run, on Windows. It shows the file
lock is real and that connection-level sharing works around it. It does not
show that the lock is an engine property rather than a Windows filesystem
behaviour — a second platform could refuse or permit differently, and nothing
here has tested that.

## P1 — `UINT64` round-trips `u64::MAX - 1`

Written and read back, `u64::MAX - 1` survives exactly through `UINT64`. This
is why `SequencePosition`'s `NonZeroU64` needs no narrowing conversion when
stored as a checkpoint value in the graph — the type has room for every value
`SequencePosition` can hold.

## The build story

`lbug`'s `build.rs` tries a prebuilt binary download first and falls back to
`cmake` only if that fails. What it downloads is one static archive,
1,444,941,838 bytes. Linking then fails on `libssl.lib`, because
`link_openssl()` inside the build script emits its link directives
unconditionally — there is no feature flag and no environment variable that
turns it off, so any consumer of this driver inherits an OpenSSL link
requirement whether or not it uses TLS. A user-space `vcpkg` build of
`openssl:x64-windows-static-md` resolved it, taking 5.6 minutes.

The sharper trap is what happens next: `build.rs` emits no
`cargo:rerun-if-env-changed=OPENSSL_DIR`. Setting that variable after a first
failed build changes nothing on the next `cargo build` — Cargo sees no reason
to rerun the script — until `cargo clean -p lbug` forces it. This is why
`lbug` ships behind an off-by-default feature: the cost is not just the
archive size, it is a toolchain dependency that fails silently-stale on retry.

## What is not in the tree, and why

`experiments/ladybug-driver-probes/results/probe-output.txt` is not committed.
`git check-ignore -v` names it against `.gitignore:69`'s `*-output.txt`
pattern — the same rule that keeps every experiment's raw run output out of a
repository headed public. The figures above are therefore transcribed from the
experiment's own `README.md` rather than read from a citable log, which is
worth stating rather than presenting as if a log backed it.
