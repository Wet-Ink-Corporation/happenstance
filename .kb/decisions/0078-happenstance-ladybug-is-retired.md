---
id: kb-decision-0078
title: happenstance-ladybug is retired — excluded from the workspace, kept in the tree as a frozen record
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0078
reversibility: high
phase: 17
supersedes: null
superseded_by: null
summary: >-
  The owner abandoned happenstance-ladybug at phase 17 (Weigh-In wi-630032): "there are just too
  many issues with it". The crate is excluded from the workspace rather than deleted. Its
  directory stays as a frozen record because the specification, the decision records, the
  references and the frozen backlog cite its files by path and line, and a deleted file breaks
  every one of those citations at once. The exclusion takes lbug and its build graph out of
  Cargo.lock and off every build, removes the gate's two Ladybug steps, the ladybug-configured
  probe, the four --exclude arguments that held the driver off --all-features, and the frozen
  ladybug CI job. What it was is the graph projection store, conformant at phase 11 against the
  real driver, publish = false because lbug does not render on docs.rs, and outside 1.0 by
  ADR-0066. What is lost is the only non-SQL projection batch among the adapters and the only
  real adapter mounted under the runtime-free blocking emitter; PS-16's falsifier is now watched by
  SQL engines alone. No clause loses its only instrument, and no PROVISIONAL clause rested on it.
  The crates.io 0.0.0 name reservation is not yanked. Supersedes nothing; ADR-0025 and ADR-0066 stay accepted.
depends_on:
  - kb-decision-wi-630032
related:
  - kb-decision-wi-630032
  - kb-decision-0025
  - kb-decision-0066
  - kb-decision-wi-2798d5
  - kb-decision-0063
source_paths:
  - Cargo.toml
  - crates/happenstance-ladybug/Cargo.toml
  - crates/happenstance-ladybug/src/lib.rs
  - xtask/src/main.rs
  - xtask/src/affected.rs
  - .github/workflows/ci.yml
  - spec/SPECIFICATION.md
  - crates/happenstance-postgres/tests/port_shape.rs
last_reviewed: 2026-09-30
---

# happenstance-ladybug is retired — excluded from the workspace, kept in the tree as a frozen record

## The question

Keep `happenstance-ladybug` in the workspace, or abandon it? The owner answered on Weigh-In item
`wi-630032`, recorded beside this atom as `kb-decision-wi-630032`:

> "we are going to abandon happenstance-ladybug - there are just too many issues with it. You can
> exclude it or delete it - whatever is easier"

The decision to abandon is the owner's. This record decides the mechanism, says what the crate was,
and says what its absence costs.

## Decision

**The crate is excluded, not deleted.** The workspace manifest carries
`exclude = ["crates/happenstance-ladybug"]` beside its `members` glob. The directory stays unchanged
apart from two one-line notes: the crate root's first line and the manifest's `publish` line. Both
notes keep every line number where it was.

Exclusion is the easier route here, and the reason is citations. `crates/happenstance-ladybug/src/`
is cited by `path:line` from the long-form ADRs under `references/adr/` (ADR-0066's record among
them), from the frozen backlog under `.bklg/`, from the governance waves, and from
`spec/SPECIFICATION.md`, which cites `projection_store.rs:530` and `:808` as the evidence for two
clauses' history. Deleting the directory would break every one of those citations at once. Some of
them sit in text this repository forbids rewriting, so they could not be repointed either. An
excluded directory resolves them all and costs nothing to build.

What the exclusion removes:

- `lbug` and its whole build graph (`cxx`, `cmake`, `link-cplusplus`, `rust_decimal`, `uuid` and the
  rest) leave `Cargo.lock`. Nothing builds the driver's 1.44 GB static archive or links OpenSSL for
  it again.
- From `cargo xtask ci`: the mandatory "happenstance-ladybug without its driver" step, the probed
  "LadybugDB projection conformance" step, and the `ladybug-configured` subcommand behind its probe.
  It also drops the four `--exclude happenstance-ladybug` arguments that ADR-0025 §9 put on the
  `--workspace --all-features` steps. They named a package no longer in the workspace.
- From `cargo xtask affected`: the special case that compiled the crate at its default features. The
  member scan now honours the manifest's `exclude` key, and a change under the frozen directory
  selects nothing.
- From `.github/workflows/ci.yml`: the `ladybug` job, which had been `if: false` since 2026-09-08,
  and the `msrv` job's `--exclude`.

## What it was

It was the graph projection store. It was real and conformant at phase 11 against the real driver:
the projection suite, mounted twice under ADR-0025 §3, plus its own tests. It implemented only the
projection role, because a graph engine is a poor event log. It was `publish = false` because it
could not be published: `lbug`'s build script returns early under `DOCS_RS` before it emits the
`cargo:rustc-env` lines its own `lib.rs` needs, so docs.rs cannot render it. ADR-0066 and
`wi-2798d5` had already put it outside 1.0, with "its own `0.x` line". This record leaves that line
with nothing to carry. ADR-0066 is not superseded by that, because its promise covers the nine crates
and this one was never among them.

## What is lost

Three instruments, and none was any clause's only one.

**The only non-SQL projection batch.** The projection port was frozen (ADR-0063) on the spread of its
implementers along the batch-shape axis. Ladybug sat at the buffered end, with `GraphWriteSet`, a
Cypher write set replayed at commit. It was the only engine in that set that is not SQL. The axis
keeps both ends without it. `SqliteBatch`, `NeonWriteBatch` and `PostgresProjectionStore`'s buffered
set hold the buffered end, and `LivePostgresProjectionStore` holds the live-transaction end. PS-2's
bar, two adapters at opposite ends that pass a suite which can fail, is still met. What goes is the
evidence that the port is not shaped around SQL. The clauses that cite Ladybug for this are frozen,
and none rests on it alone:

- **PS-4** names LadybugDB as the falsifier's candidate. `LivePostgresProjectionStore` is the adapter
  that confirmed the MAY.
- **PS-9**'s Rejects argues from "a graph store and a relational store". ADR-0074 froze it on
  `experiments/apply-shape` and restated its falsifier without Ladybug.
- **PS-12**'s decline arm keeps three adapters: SQLite, Postgres and Neon.

**The only engine that was not SQL behind PS-16's falsifier.** See the `[PROVISIONAL]` paragraph
below: reset joining the checkpoint transaction is now observed on SQL engines alone.

**The only real adapter mounted under the runtime-free emitter.** ADR-0025 §3 made
runtime-agnosticism falsifiable by mounting the suite under the blocking emitter as well as the tokio
one. The blocking mount survives in the testkit's own
`crates/happenstance-testkit/tests/projection_conformance_blocking.rs`, against the reference store.
No shipped adapter is mounted that way now.

**Compiled evidence is moved, not left behind.** `crates/happenstance-ladybug/tests/port_shape.rs`
held the workspace's only compiled instance of two things. One is PS-5's caller-side benefit: a
generic `tokio::spawn` that holds `S::Batch` across an await under a `where` clause reading
`S::Batch: Send`. The other is the remedy PS-36 and `happenstance-core`'s projection docs give a
caller, which is that same bound written by hand. Nothing else in `crates/` or `examples/` compiled
that pattern. The typed runner does not spawn a batch, and the only other `Batch: Send` in the tree
is prose. So the file moves to `crates/happenstance-postgres/tests/port_shape.rs`, and its `const _`
block is now instantiated at both ends of PS-2's axis: `PostgresProjectionStore`'s buffered set and
`LivePostgresProjectionStore`'s live transaction. Deleting `S::Batch: Send` there fails to compile
(`future cannot be sent between threads safely`), which was checked rather than assumed. Constitution
atoms `standards/rust/20-two-flavour-ports.md`, `21-send-is-not-inherited.md` and
`61-compile-time-assertions.md` now cite the moved file. The Ladybug copy stays in the frozen
directory as the record of where the measurement was first taken.

**No `[PROVISIONAL]` clause relied on Ladybug as its instrument.** The provisional `PS` clauses are
PS-6, PS-16, PS-25 and PS-38. PS-6's rules are Neon's `begin_makes_no_round_trip` and the contract
crate's `begin_resolves_at_its_first_poll_without_a_runtime`, and the other three have no Ladybug
rule. That is true only rule by rule, and the watch did narrow. Ladybug mounted the whole
projection suite twice, so it ran the rules behind PS-16 (`reset_clears_rows_and_checkpoint_together`)
and PS-38 (`commit_advances_the_checkpoint`, `fresh_projection_has_no_checkpoint`). PS-16's falsifier
is a store whose read-model clearing cannot join the checkpoint transaction. Ladybug was the only
engine that was not SQL to run that rule, carrying a Cypher `DELETE` of the caller's nodes and of the
checkpoint node inside one graph transaction. Every remaining observation of PS-16's falsifier is now
a SQL engine: SQLite, Postgres twice, and Neon. No marker changes for this, but whoever moves PS-16
toward `[FROZEN]` should know that the spread behind it is narrower than it was.
The specification's places that describe Ladybug as a current adapter now say it is retired.
The historical evidence beside them is kept.

## What is not done

- **The crates.io `0.0.0` reservation is not yanked.** The name stays claimed, as every reservation
  does, and `xtask/src/reserve.rs` keeps its row with a comment that says so.
- **Nothing is deleted.** Not the directory, not its tests, and not `.kb/reference/ladybug-driver-probes-2026-09.md`.
  ADR-0025 stays accepted, because it records how the adapter was built, and the adapter was built.
- **No clause changes marker.** The spec edits are notes on existing lines, and line counts are
  unchanged.

## Consequences

- A contributor who wants a graph read model writes a projection adapter against the frozen port, as
  `examples/outside-projection-adapter` shows. The frozen directory is a worked example of one, not
  a dependency.
- Reversing this is cheap and is the `high` in `reversibility`. Drop the `exclude` entry and restore
  the removed gate steps from git history. `lbug` would still not render on docs.rs, so the crate
  would still be unpublishable.
- `grep -n '^publish' crates/*/Cargo.toml` still returns two lines. One of them is now a crate
  outside the workspace, and `CLAUDE.md` says so.
