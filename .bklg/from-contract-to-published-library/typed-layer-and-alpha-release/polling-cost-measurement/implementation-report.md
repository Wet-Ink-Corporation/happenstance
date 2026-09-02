---
item: "HS-S0028"
stage: implement
created: "2026-08-16"
updated: "2026-08-16"
---

# Implementation Report — The polling cost ES-32 imposes, as a number

**All ten ACs are satisfied. Nothing is blocked and nothing is deferred.**

There is now a number. In the arm where every view wants every event, the polling runner
delivers each event **once per view** — delivery amplification of **32.00 at 32 views**,
exactly the fan-out, with no economy of scale anywhere in the range — and issues **64
reads** where a tail seam would deliver once. In the arm where the views' queries do not
overlap, amplification is **1.00** at every fan-out and the read count is still 2N.
Observation latency is the poll interval and nothing else, at every fan-out from 1 to 32,
and no backlog ever accumulated.

Every figure carries the conditions it was taken under, in the record rather than in prose,
and the README says in its own section what the measurement does **not** reach — both of
ES-32's falsifier terms, quoted.

## TDD Evidence

The harness is an instrument, so the tests are tests **of the harness** and the red was
staged where a red could be real.

**Stage one — the artefacts that did not exist yet.** All ten test files were written
before the README, the schemas and `results/` existed. Running them then:

```console
$ cargo test --manifest-path experiments/polling-cost/Cargo.toml
test result: FAILED. 0 passed; 6 failed      # readme.rs — no README
... record_conditions.rs, schema.rs — no corpus, no schema
```

| AC | test file | the red |
| --- | --- | --- |
| AC-006, AC-009 | `readme.rs` | six failures, all on `reading README.md` |
| AC-003 | `record_conditions.rs` | `no committed pass under results/` |
| AC-005 | `schema.rs` | `reading …/schema/result-record.schema.json` |
| AC-010 | `rerun_is_additive.rs` | red until `write_pass` refused an existing tag |

**Stage two — two reds that were about the harness, not about a missing file.** Both are
worth recording because both changed the code:

1. `no_verdict.rs::the_harness_holds_no_threshold` went red on `assert!(` inside
   `harness_sources()`. The fix was **not** an exception inside the scan — an exception
   inside a negative assertion is where the next real threshold hides. The helper moved to
   its own module, `src/sources.rs`, and the scan skips that one file by name and says why.
2. `one_cell.rs::…binds_the_bare_flavour` went red on the words `SendEventStore` — which
   appear in the harness's own doc comment saying it does not bind them. Comments are now
   stripped before the assertion, in both files that make one.

**The named mutant, written down and failed.** AC-004's discriminator is
`staleness.rs::the_subtracting_mutant_fails_the_same_comparison`. The mutant —
`head.saturating_sub(through)` — is written into the test file, fed the same
three-appended/one-observed pattern through a dense store and through
`happenstance_testkit::GappyMemoryStore` at stride 7, and asserted to answer **2 and 14**.
The observer answers **2 and 2**. Without that assertion the gap test would pass against an
implementation that subtracts, and it would be decorative.

**Green.** All 33 tests across ten files:

```console
$ cargo test --manifest-path experiments/polling-cost/Cargo.toml
# 14 targets, every one `test result: ok`
```

## Commits

One checkpoint commit, carrying the whole story:

```
feat(typed-layer-and-alpha-release): Polling cost measurement
Story: typed-layer-and-alpha-release/polling-cost-measurement
```

Its short SHA is reported in this run's slice digest and is recoverable here with
`git log --grep "Story: typed-layer-and-alpha-release/polling-cost-measurement" --oneline`.

## Changes

Everything is new, and everything is under `experiments/polling-cost/`.

| file | what it is |
| --- | --- |
| `Cargo.toml` | the mount: bare `[workspace]`, `publish = false`, the path dependency on `crates/happenstance` with `unstable-projection`. Both facts — how it binds to the real runner, and how it stays out of the workspace — in one file. |
| `src/lib.rs` | the grid, the measured domain and projection, the two phases, the record contract, the environment capture, `write_pass`, and a fifteen-line `block_on` so the harness takes no async runtime. |
| `src/counting.rs` | the instrument: an `EventStore` wrapping `MemoryEventStore` that counts reads, deliveries and distinct positions as the stream is pulled. |
| `src/observer.rs` | staleness by comparison. `pending` filters on `position > through`; nothing subtracts. |
| `src/progress.rs` | plain-line console output, truncated at 80 columns rather than wrapped. |
| `src/validate.rs` | the small schema checker `tests/schema.rs` holds the corpus to. |
| `src/sources.rs` | reads the harness's own source, so a test can hold it to its negative requirements. The only module allowed an `assert!`. |
| `src/main.rs` | the sweep driver. `HS_TAG`, `HS_CELLS`, no arguments, no threshold. |
| `run.sh` | the reproducer: toolchain, measured revision, host, `--release` build, sweep, listing. |
| `schema/` | `result-record.schema.json` and `manifest.schema.json`. |
| `results/pass-001/records.ndjson` | the committed pass: one manifest line, 144 records. |
| `README.md` | six sections, the headline figure, the conditions, the staleness table, what it does not prove, the verdict, and how to re-run. |
| `tests/` | ten files, 33 tests, of the harness and never of a cost. |

## Gates

| command | result |
| --- | --- |
| `bash experiments/polling-cost/run.sh` | complete pass written from a clean-under-`crates/` checkout |
| `cargo test --manifest-path experiments/polling-cost/Cargo.toml` | green — 33 tests, 14 targets |
| `cargo fmt --check` (harness) | green |
| `cargo clippy --all-targets -- -D warnings` (harness) | green |
| `cargo xtask affected --base main` | **affected gate passed**, no package selected by this diff |
| `cargo xtask ci --fast` | **all required checks passed (--fast: 4 optional step(s) not run)** |
| `cargo fmt --all --check` (workspace) | green |
| `cargo xtask spec-trace` | **traceability: no problems found** |

### The reproducer transcript, trimmed

```console
$ HS_TAG=verify HS_CELLS=4 bash experiments/polling-cost/run.sh
polling-cost: tag verify
--- toolchain ---
rustc 1.97.1 (8bab26f4f 2026-07-14)
--- measured tree ---
55a237000c0f148efed6663ad94925f81ac0a09c
--- host ---
…
polling-cost: 4 cells, tag verify
measured tree 55a237000c0f148efed6663ad94925f81ac0a09c dirty=false
[1/4] amplification n=1 log=1000 poll=10ms arm=overlapping
…
--- written ---
records.ndjson
```

And the refusal, demonstrated rather than asserted:

```console
$ HS_TAG=pass-001 bash experiments/polling-cost/run.sh
harness failed: writing the pass failed: the pass `pass-001` already exists at …
error: process didn't exit successfully: … (exit code: 1)
```

**Boundary.** `git status --porcelain -- crates Cargo.toml Cargo.lock xtask spec` is empty
for this story. The only paths it touches are `experiments/polling-cost/**` and its own
backlog folder.

## Notes

### The headline's denominator is not the one AC-002 spells, and the reason is a finding

AC-002 defines delivery amplification as *"events the store yielded across all views ÷
events the projections applied"*. Under this tree that ratio is **identically 1.0**, in
both arms, at every fan-out: the query is *derived* rather than supplied, the store filters
by it, and the runner applies every event it is handed — so what is yielded to a view is
exactly what that view applies, and the ratio measures nothing. It would also make the two
selectivity arms report the same figure, which would defeat the reason the spec insists on
having two.

The denominator implemented is the **distinct** events those deliveries carried, observed
by the instrument at the moment each event was handed over rather than assumed. That gives
32.00 in the overlapping arm and 1.00 in the disjoint one — the two ends the spec named,
told apart. Both raw counts (`events_delivered`, `events_applied_total`) and the distinct
count are on every record, so a reader can recompute either ratio; the schema documents the
choice on the field, and so does §2 of the README. This is a deviation from an AC's literal
words in service of the AC's stated purpose, and it is recorded here rather than absorbed.

### The read count is the figure most likely to matter, and this harness cannot charge for it

`reads_issued` is 2N in **both** arms — one delivering read per view plus one that confirms
there is nothing new. Selectivity buys back deliveries and buys back no reads at all. On an
in-process `Vec` a read is a filtered scan and costs almost nothing, so the harness's
timings barely notice; on a store where a read is a round trip it is the reads that will
dominate, and this measurement charges nothing for them. §4 of the README says so in the
same terms. It is the single most important thing the number does not settle, and it
belongs to `sqlite-durable-store` (HS-P0012).

### Project AC-012 defect-log entry: `run_projection`'s `&mut P` shapes the fan-out cost

This story is the first consumer of `happenstance::run_projection` from **outside** the
workspace, and the finding is about the signature rather than about a bug.
`run_projection(events, models, projection: &mut P, codec, chunk)` takes one projection by
unique reference, so N views are N calls and therefore N reads — which is exactly the cost
ES-32 imposes and exactly what this harness measures, so the signature is *correct* for the
alpha. What is worth recording is that the shape makes the N-reads cost **unavoidable at
the API level**: there is no way for a caller to express "these N projections want the same
read" even where their queries are identical, and the design already knows why (a batch is
owned and cannot cross a task, so a fan-out runner is not buildable today). Routed to
`defect-log-and-macros-verdict` (HS-S0032) as a project AC-012 entry, not patched around
locally and not fixed by editing `crates/happenstance`.

### Two things renamed for reasons outside this directory

`src/store.rs` became `src/counting.rs`. `cargo xtask spec-trace` resolves bare `file:line`
citations by filename across the whole repository, and `crates/happenstance-core/src/store.rs`
is cited ten times in `spec/SPECIFICATION.md`; a second `store.rs` anywhere in the tree made
every one of those citations ambiguous and produced **19 traceability problems** in the
affected gate. The alternative was `BARE_NAME_MAP` in `xtask/src/spec_trace.rs`, which is
outside this story's PR boundary and would have been the wrong fix anyway — the harness has
no claim on the name. Worth noting as a property of the checker a future experiment will
also meet.

`git_dirty` is scoped to the **measured** tree — `crates/`, `Cargo.toml`, `Cargo.lock` —
rather than to the whole worktree, and the schema says so on the field. What makes a figure
irreproducible is the library moving under it; this directory gaining a paragraph does not.
Without that scoping no committed pass could ever carry `git_dirty: false`, because the
pass and the README that quotes it are necessarily uncommitted at the moment the sweep
runs.

### The trim, and what was not trimmed

The declared grid is a full cross product of fan-out × log size × poll interval × arm. This
pass runs it as two phases — amplification over fan-out × log size × arm, staleness over
fan-out × poll interval × arm at a fixed 1 000-event log. **No axis value is dropped and
neither arm is dropped.** What is dropped is the cross product between log size and poll
interval, because neither quantity moves with the other axis: replay amplification does not
move with the poll interval (the runner resumes past its checkpoint, so an idle poll
delivers nothing), and staleness does not move with the log size once the views have caught
up. Recorded in README §1 with that reason, which is what the spec permits.

### One measurement artefact, found and fixed before the committed pass

The first sweep reported inflated disjoint-arm staleness and a non-zero backlog, because
every view's observer was told about every appended event — including events whose shard
that view's query excludes. The observer was then measuring the harness waiting for
something that was never coming. Fixed by telling only the views that **nominated** the
event, using the same two inputs the runner derives its query from. The committed pass is
the corrected run; the first was discarded rather than kept and explained.
