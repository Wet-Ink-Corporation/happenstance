# Runbook

The plan for finishing happenstance, and the record of how far it has got.

**Starting a session? Read [`handover.md`](handover.md) first**, then the status
table below. Those two are the whole of what a session needs before it picks up
work; everything else in this directory is read when the work in hand reaches it.

## Why this is a directory

Until 2026-09-28 the runbook was one file, `RUNBOOK.md`, and it had reached 5,704
lines. Every session loaded it to find one table, and the table went stale anyway:
phase 12 read `not started` across four published releases, because the prose
around it was too long to re-read and the check on it skipped every release
without a row. The split is a response to both.

**`RUNBOOK.md` is kept, frozen, with its line numbers intact.** About 2,250
`RUNBOOK.md:N` citations across the repository resolve against it — one of them in
ADR-0033's accepted atom, which may not be edited to repoint it — and a file that
shrank would make every one of them point at the wrong text. It is the record of
phases 0–12, and the history behind every ledger here. Read it when a citation
sends you there; do not edit it except to correct a single line in place.

| File | What it carries | Read it |
|---|---|---|
| [`handover.md`](handover.md) | Where the work actually is, the next action, what is waiting on the owner, and the traps | at the start of every session |
| this file | How to use the runbook, the session protocol, the status table | at the start of every session |
| [`roadmap.md`](roadmap.md) | Why the remaining phases are in the order they are, and the decisions pending the owner | when choosing or re-ordering work |
| [`ledgers.md`](ledgers.md) | The ADR queue, the open decisions, and the provisional and deferred clause ledgers `cargo xtask lints` holds to the specification | when a clause moves, an ADR is written, or a phase exits |
| [`log.md`](log.md) | Dated session log, newest first | when you need to know what happened, not what is planned |
| `phases/NN-*.md` | One file per open or future phase: goal, work, proof artefact, exit criteria, its own session log | when working that phase |
| [`../RUNBOOK.md`](../RUNBOOK.md) | The frozen monolith: phases 0–12 in full, and every settled row | when a citation points there |

## How to use this

1. **The runbook is updated in the same commit as the work it describes** — the
   phase file's boxes and session log, the status row, and the handover.
2. **A phase is done when its proof artefact exists in the repository and every
   exit criterion is ticked.** Not when the gate is green. A green gate is a
   precondition for looking at the exit criteria, never one of them.
3. **The specification is current truth; ADRs are history.** Changing a
   `[FROZEN]` clause takes a new ADR, not an edit. Where any file here and
   `spec/SPECIFICATION.md` disagree, the specification wins and the file here is
   fixed in the same commit.
4. **Nothing is deleted from a ledger.** A row the specification now binds cites
   its clause and stays. The monolith keeps every row that existed before the
   split.
5. **A count is spelled with its members or computed by a tool.** A bare number in
   a document this often read is a number nobody re-reads; `cargo xtask
   spec-trace`, `ls .kb/decisions/` and `grep -n '^publish' crates/*/Cargo.toml`
   are the answers that cannot go stale.

## Session protocol

```
1. Read handover.md, then the status table, then `git log --oneline -10`.
2. Run `cargo xtask ci`. Establish the baseline is green before touching anything.
3. Take the handover's next action. If there is none, pick the first phase that is
   not `done` and whose dependencies are `done`.
4. Write the phase's ADRs first. Then the code they constrain.
5. Build the phase's proof artefact. If you cannot, the phase is not done — say so
   in its session log rather than ticking the box.
6. Reconcile the specification against the code the phase just wrote — the prose
   and citations under the MUSTs, not only the MUSTs.
7. Re-run the gate. Tick the phase file's exit criteria.
8. Before committing, write the handover (the whole file, in its template), add a
   dated line to the phase's session log and to log.md, and move the status row.
9. Commit the code and the runbook together.
```

Step 8 is the one the monolith did not have, and it is the one that failed: the
monolith's session logs were appended to and its status table was not. The
handover is rewritten whole each time, so a stale one is visibly stale — its
`As of` line names a commit that is not `HEAD`.

## Status

| # | Phase | Depends on | State | Proof artefact |
|---|---|---|---|---|
| 0 | [Ground clear](../RUNBOOK.md#phase-0--ground-clear) | — | done | a `.crate` carrying its licences and README; `cargo xtask spec-trace` failing on a broken clause |
| 1 | [The `!Send` proof](../RUNBOOK.md#phase-1--the-send-proof-and-the-derivation-decision) | 0 | done | one provided body type-checking under both flavours; every rule green against a `!Send` store on `wasm32` |
| 2 | [The instrument portfolio](../RUNBOOK.md#phase-2--the-instrument-portfolio) | 1 | done | six crates compiling on their real targets with real associated types |
| 3 | [The suite becomes an instrument](../RUNBOOK.md#phase-3--the-suite-becomes-an-instrument) | 1 | done | the mutant registry: every rule has a mutant that fails it |
| 4 | [Freeze the contract](../RUNBOOK.md#phase-4--freeze-the-contract-signatures-value-types-and-identity) | 2, 3 | done | `frozen_signatures.rs`, and a `compile_fail` doctest pinning what does not compile |
| 5 | [Freeze the wire format](../RUNBOOK.md#phase-5--freeze-the-wire-format) | 4 | done | two `wire.rs` files across `happenstance-core` and `happenstance-sync`, with negative controls asserted by name |
| 6 | [Freeze `ProjectionStore`](../RUNBOOK.md#phase-6--freeze-projectionstore) | 4 | done | `CheckpointOnlyStore` failing the projection suite, and two unlike batch shapes passing it |
| 7 | [The typed layer and the example](../RUNBOOK.md#phase-7--the-typed-layer-and-the-worked-example) | 4, 6 | done | a `trybuild` compile-fail case: add an event variant, and the fold stops compiling |
| — | **`0.2.0-alpha.1`** | 7 | — | released 2026-08-16, since yanked |
| 8 | [`happenstance-sqlite`](../RUNBOOK.md#phase-8--happenstance-sqlite) | 4, 6, 7 | done | the concurrency macro green at 64 contenders; an acknowledged write surviving a reopen |
| 9 | [Cloudflare Durable Object](../RUNBOOK.md#phase-9--cloudflare-durable-object) | 2, 4 | done | every rule green on `wasm32` against a `node:sqlite` `DurableObjectState` shim — not `workerd`, whose run is phase 17's sibling job (ADR-0066) — with a real `worker::Error`-carrying error type |
| 10a | [Postgres event store](../RUNBOOK.md#phase-10--happenstance-postgres-and-happenstance-neon) | 2, 4, 6 | done | the concurrency macro green on a store that does not serialise its writers, with the visibility cost measured |
| 10b | [Postgres projections, and Neon](../RUNBOOK.md#phase-10--happenstance-postgres-and-happenstance-neon) | 2, 4, 6 | done | no `todo!()` left on either crate; Neon's capability declines stated; `LivePostgresProjectionStore` passing all seventeen rules |
| 11 | [Ladybug projection store](../RUNBOOK.md#phase-11--ladybug-projection-store) | 6 | done | the projection suite green on a non-SQL batch against the real driver. Finished, and cannot publish: `lbug` does not build on docs.rs |
| 12 | [Publish `0.2.0`](../RUNBOOK.md#phase-12--publish-020) | 7, 8, 10a | done | seven crates on crates.io and rendering on docs.rs; `cargo-semver-checks` against a registry baseline |
| — | **`0.2.0`** | 12 | — | released 2026-09-10 — seven crates |
| — | **`0.3.0`** | 10b, 12 | — | released 2026-09-11 — the projection port frozen (ADR-0063) |
| — | **`0.3.1`** | 12 | — | released 2026-09-11 — descriptions only |
| — | **`0.3.2`** | 12 | — | released 2026-09-20 — a dependency advisory (`rustls`) |
| 15 | [Reconcile the record](phases/15-reconcile.md) | 12 | done | the status lint failing on the pre-split table and passing on this one |
| 16 | [Define 1.0](phases/16-define-1-0.md) | 15 | done | the 1.0 charter, with a disposition for every non-frozen clause on a promised surface |
| 17 | [The breaking window — `0.4.0`](phases/17-breaking-window.md) | 16 | in progress | `0.4.0` released, every semver break traced to a decision |
| 17b | [After the window — the additive half](phases/17b-after-the-window.md) | 17 | not started | VT-14, VT-30 and ES-7 frozen; the minimal-versions and floating-dependency jobs watched failing once |
| 18 | [The typed runner leaves its gate](phases/18-typed-runner.md) | 17 | not started | the rebuild example compiled with no unstable feature in its graph |
| 13 | [`happenstance-sync` and its testkit](phases/13-sync.md) | 5, 8, 9, 10a, 10b, 12, 17, 18 | not started | one suite green against three peers, two of them unlike, and a byte-identical round trip |
| 14 | [Retention and completeness](phases/14-retention.md) | 13, 17 | not started | a store that holds only a suffix of its own log, and a runner that fails loudly against it |
| 19a | [SQLite on `wasm32` — skeleton](phases/19-sqlite-on-wasm.md) | 15 | not started | a skeleton building for `wasm32` in the gate, with a verdict on driver, storage and CI |
| 19b | [SQLite on `wasm32` — the adapter](phases/19-sqlite-on-wasm.md) | 17, 19a | not started | both conformance suites green on `wasm32`, in the gate |
| 20 | [Documentation that teaches](phases/20-docs-that-teach.md) | 15 | in progress | the docs initiative's Definition of Done, re-observed from a clean checkout |
| 21 | [`1.0.0`](phases/21-one-point-oh.md) | 13, 14, 16, 17, 17b, 18, 20 | not started | the promised crates at `1.0.0`, and the clause audit clean |

State is one of `not started`, `in progress`, `blocked`, `done`. Edit it in place.
A milestone row names a released version and the phase it waited on;
`cargo xtask lints` refuses a released version with no row here, and a milestone
whose prerequisites are not all `done`.

Rows are in the order the work is expected to run, not in number order: phases 13
and 14 were numbered before the decision window in front of them existed, and
renumbering them would break every citation that names them. Sync is inside
1.0, so phase 21 waits on 13 and 14 — see the [roadmap](roadmap.md#decisions-taken).

## Standing constraints

Carried from [`CLAUDE.md`](../CLAUDE.md), whose wording wins where the two differ.

1. **Never introduce `#[async_trait]`.** It injects `+ Send`, which makes the
   `wasm32` target impossible. Ports are defined once without a `Send` bound and
   `trait_variant` derives the `Send` flavour (ADR-0001, ADR-0008).
2. **Never put `serde` in `happenstance-core`'s default features.** Payloads are
   opaque `Bytes`. This binds the contract crate, not `happenstance`, whose job is
   encoding (ADR-0003, ADR-0006).
3. **`EventStore::read` returns the stream at the top level and is not `async`.**
   Two tests in `memory.rs` hold this and it takes both (ADR-0001, ADR-0008).
4. **Bind `EventStore`, not `SendEventStore`, in generic code**, and import only
   one of the two names per module.
5. **The MSRV is 1.97.1, and since `0.2.0` it is a promise** (ADR-0029,
   ADR-0037). Raising it needs a decision record, not a commit message.

And the rules that outrank the rest:

- **An adapter that has not run the conformance suite is not an adapter.** If a
  rule looks wrong, fix the rule and say why in the same change.
- **A rule no adapter can fail is decorative.** Name the wrong implementation it
  rejects before adding it.
- **A port whose adapters all share a shape is a port shaped like that shape.**
  The instrument portfolio (`RUNBOOK.md:799-846`) is that check.
