---
item: HS-S0044
stage: discover
created: 2026-08-12T13:02:06.784Z
updated: 2026-08-12T13:02:06.784Z
template_sig: 86ce4036
rendered_sig: 8a62e459
---

# Discover — SqliteProjectionStore against the suite it did not write

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Slice one-line: give `SqliteProjectionStore` real bodies against its owned `Batch` on an independently-migrated connection, and mount it at `crates/happenstance-sqlite/tests/projection.rs` against the projection suite `projection-store-freeze` froze. | `_storymap.md`, *Slices* table, row `sqlite-projection-store` / `projection-store-passes-the-borrowed-suite` | "The suite it did not write" is the point: this project **consumes** HS-P0010's suite and amends nothing about the port. |
| **AC-011** — `SqliteProjectionStore` is green against `projection-store-freeze`'s projection suite, and this project's architecture brief records — before code — whether the second unlike batch shape DoD 7 owes lives here or in the testkit. | `project.md`, AC-011 | The second half was answered in the brief: it does **not** live here. |
| `dependsOn: schema-migration-and-identity` (HS-S0036) — the checkpoint migration must be idempotent on its own connection, and "an event store and a projection store on one file are two connections, not one." | `_storymap.md`, *Merge order* item 4; `_decomposition.md`, *Architecture brief*, §7 | The dependency is on migration discipline, not on the event tables — the projection store shares no schema with the event store. |
| Only slice 2 gates it: this story "is independent of `race-model-and-durability`" and "may be pulled forward beside slice 3 if `projection-store-freeze` has already shipped its suite." | `_storymap.md`, *Slice coherence notes*, *Merge order* item 4 | The real schedule risk is external: HS-P0010 owes the suite, and this story cannot mount a macro that does not exist. |
| The port's invariant, which SQLite is unusually well placed to honour: "the read-model writes and the checkpoint update share one transaction, so there is no window in which they can disagree." | `crates/happenstance-sqlite/src/projection_store.rs:5-7` | This is the whole of what the adapter owes. The read-model tables "are the application's business; this adapter owns only the checkpoint and the transaction that carries it" (`:41-42`). |
| Two compiled results the skeleton already banked: `type Batch<'a> = rusqlite::Transaction<'a>` is unavailable on `SendProjectionStore` for two independent reasons — `Connection` is `Send` and not `Sync`, and `Transaction<'_>` is itself `!Send`; and binding an **owned** type to the GAT does not exempt the impl from spelling `Self::Batch<'_>` literally (`error[E0195]`). | `crates/happenstance-sqlite/src/projection_store.rs:9-31`, `:211-225` | The batch shape is settled and is not this story's to revisit: an owned, `Send`, replayable write set opened into a real transaction inside `commit`. |
| `begin` and `rollback` already have **real bodies** — a `Vec` allocation and a drop — because "an owned buffer batch is created and discarded without the database being involved at all." | `crates/happenstance-sqlite/src/lib.rs:5-13`; `crates/happenstance-sqlite/src/projection_store.rs:235-259` | Only `checkpoint` and `commit` are `todo!()`. The remaining work is narrower than "implement the projection store". |
| PS-6 (`begin`'s needless `async` + `Result`) and PS-7 (whether dropping a batch must roll back) are already marked "not this crate's to settle" in the code. | `crates/happenstance-sqlite/src/projection_store.rs:236-238`, `:256-259`; `_decomposition.md`, *Architecture brief*, §8 | "That stays true." The rollback body is *evidence* for PS-7, deliberately recorded rather than elided. |
| The projection port is **provisional**: it has no conformance suite yet, "and a port without one is a guess." It gets frozen when the first real projection adapter can be built against it. | `CLAUDE.md`, *Open questions*; `project.md`, *Out of scope* | This story is that adapter — but the freeze verdict belongs to `projection-store-freeze` and `ladybug-projection-store`, not here. |
| DoD 7's second unlike batch shape: answered in the architecture brief **before code** — it does not live in this project. Taking it here "would give the DAG an edge it does not carry and invert the 6 → 8 order." If HS-P0010's own brief assigns it here instead, that is a **blocking re-plan**, not something this story absorbs quietly. | `_decomposition.md`, *Architecture brief*, §9; `project.md`, risk 3; `.bklg/from-contract-to-published-library/_decomposition.md` (*DoD 7 watched edge*) | The single most important thing this story must **not** do. |
| The mount is `crates/happenstance-sqlite/tests/projection.rs`, wiring to a macro **owned by HS-P0010** — the brief deliberately does not name the macro "because it does not exist yet". | `_decomposition.md`, *Architecture brief*, §1 mount table | The spec cannot cite a macro name until HS-P0010 ships it; the story must be written so that the mount is the only thing that changes when it does. |
| ADR-0009: `Error` stays exactly `core::error::Error + 'static` on both flavours; `Send + Sync` must not migrate onto the port. | `.kb/decisions/0009-error-send-sync.md`; `crates/happenstance-sqlite/src/projection_store.rs:179-206` | `SqliteProjectionStoreError` already enumerates the real failure modes, including `NoRuntime` — the same runtime seam ADR-0022 settles for the event store applies to this connection too. |

## Questions

Open questions to resolve before specifying.

1. **Does the second unlike batch shape live here?** *Answered: no*, and answered
   in writing before code, which is exactly what AC-011's second clause asks for.
   `SqliteProjectionStore` supplies the owned-buffer SQL shape as a **consumer**
   of HS-P0010's suite. If `projection-store-freeze`'s own architecture brief
   assigns the second shape here, that is a blocking re-plan — a new dependency
   edge before implementation starts — and must be escalated rather than absorbed.
2. **What is the projection suite's macro called, and what does it require of a
   fixture?** *Deferred, necessarily* — HS-P0010 owns it and it does not exist.
   The spec should be written so that the only thing waiting on HS-P0010 is the
   mount in `crates/happenstance-sqlite/tests/projection.rs`; the store's bodies,
   the migration and the transaction discipline do not depend on the suite's
   shape.
3. **Does the projection store share the event store's connection?** *Answered:
   no.* Two connections, each migrating its own schema idempotently
   (`_decomposition.md`, *Architecture brief*, §7). Sharing would collapse a real
   axis — the checkpoint invariant is interesting precisely because it is a
   separate transaction on a separate handle.
4. **Where does `commit`'s transaction get its runtime?** Consumed from ADR-0022:
   the same captured-`Handle` seam the event store uses.
   `SqliteProjectionStoreError::NoRuntime` exists for the same reason and, if
   ADR-0022 chose option (b), must be removed or rewritten in the same change
   rather than left documenting a state that cannot occur
   (`_decomposition.md`, *Architecture brief*, §3).
5. **PS-6 and PS-7.** Explicitly deferred, and not to `spec` — to
   `projection-store-freeze`. The code already says "not this crate's to settle"
   and this story keeps that comment true rather than quietly acting on it.
6. **Does this story contribute to the port's freeze verdict?** Deferred. It
   supplies evidence — the first real SQL projection adapter — and the verdict is
   HS-P0010's and `ladybug-projection-store`'s. Recording the evidence is in
   scope; writing the verdict is not.
7. **The append-condition SQL strategy.** Not touched; the projection store has no
   append condition.

## Decision

`SqliteProjectionStore` is the first real implementation of a port that has never
had a suite run against it, and the port's single invariant — read-model write and
checkpoint write move together — is one SQLite can honour trivially and lose
silently. This slice gives `checkpoint` and `commit` real bodies against the owned
`SqliteBatch` the skeleton's compiler transcripts already settled, migrates the
`projection_checkpoint` schema idempotently on its **own** connection, and mounts
the store at `crates/happenstance-sqlite/tests/projection.rs` against the suite
`projection-store-freeze` freezes. The spec will cover: `commit` opening one
transaction, replaying every buffered statement, writing the checkpoint row and
committing — one transaction, never two; `checkpoint` reading the row back;
independent idempotent migration of `projection_checkpoint`; the runtime seam
consumed from ADR-0022 with `NoRuntime`'s doc kept true; PS-6 and PS-7 left
explicitly unsettled with their existing comments intact; and the mount, which is
the only part that waits on HS-P0010. It records, before code and per AC-011, that
DoD 7's second unlike batch shape does **not** live here. This story adds no
conformance rule — it runs one it did not write — so the literal-position bar is
vacuous, and it amends no `[FROZEN]` clause; the projection port is provisional
and its freeze is another project's.

## The wrong implementation

**The mutant: a `commit` that writes the read-model rows and the checkpoint in two
separate transactions** — or, more likely, with no explicit transaction at all,
letting `rusqlite` autocommit each `execute` in the replay loop and then writing
the checkpoint row afterwards. It is the natural shape when the batch is already a
buffer of statements: iterate, execute, then update the checkpoint. Nothing about
the code looks wrong.

It passes every sequential projection rule that can be written. `commit` returns
`Ok`, `checkpoint` reads back the position that was just written, a replay from
that checkpoint produces the right read model, and the store's own module doc
still *claims* the invariant at
`crates/happenstance-sqlite/src/projection_store.rs:5-7`. The defect is a window
that only a fault opens: a crash between the last read-model write and the
checkpoint update leaves the read model ahead of its checkpoint, so the next run
replays events already applied. Whether that is corruption or a no-op depends on
the read model's idempotence — which is the application's business, not the
adapter's, which is exactly why the adapter is not allowed to leave the window
open. No rule can arm it: `MID_BATCH_FAULT` defaults declined
(`crates/happenstance-testkit/src/contract.rs:207-211`), this project's fixture
declines it explicitly with a stated reason, and the projection suite is
HS-P0010's to write.

**Where the control lives, and the honest limit on it.** The projection suite is
not this project's, so the *rule* that would catch this — a fault armed between
the read-model write and the checkpoint write — is something this story can only
**report** to `projection-store-freeze`, not add. What this story owes instead,
in `crates/happenstance-sqlite/tests/`, is an adapter-owned test that inspects the
mechanism rather than the outcome: buffer a batch containing a statement that will
**fail** at replay time (a constraint violation on a read-model table the test
creates), call `commit`, assert it returns `Err`, and then assert the checkpoint
row is **unchanged** and none of the batch's earlier statements is visible. A
two-transaction `commit` fails that immediately; a one-transaction `commit` passes
it. That is a real negative control for the invariant, reachable without a fault
injection seam, and it belongs here because it is a fact about *this adapter's*
`commit` rather than about the port.

**The second mutant, which is a schema mistake wearing a performance argument:
the projection store sharing the event store's `rusqlite::Connection`.** One
connection, one `Mutex`, fewer file descriptors, and — apparently — a bonus: the
event append and the projection commit could then share a transaction. Every
projection rule passes, and the checkpoint invariant is honoured, more strongly
than required. What it destroys is the axis: the port exists because a projection
store is a *separate* store that may live behind a separate handle, possibly a
separate process, and an adapter that can only honour the invariant by sharing the
writer's connection has not demonstrated the invariant at all — it has
demonstrated that one transaction is atomic. `_decomposition.md`'s architecture
brief §7 states the requirement flatly ("two connections, not one") and the
control is review against the store's constructor, since no rule in any suite can
tell how many file handles an adapter opened.

**The third, and it is a process mutant rather than a code one: absorbing DoD 7's
second unlike batch shape into this story** because HS-P0010 is late and the SQL
shape is right here. Everything stays green — a second shape passing the suite is
strictly more evidence — and the DAG silently acquires an edge it does not carry,
inverting the runbook's 6 → 8 order and making `projection-store-freeze`'s freeze
verdict depend on the project it was supposed to precede. `project.md` risk 3 and
architecture brief §9 both name this as a **blocking re-plan**, not an absorption.
The control is that this discover file says so before implementation starts.

## Gate: Discover

Tick each box once the sections above satisfy it. `redkiln advance` reads these
against `.redkiln/templates/gates/discover.md` — where each box's rationale is
written — and will not leave `discover` until every one is ticked. Keep each box on
one line; the parser matches line by line.

- [x] The problem is framed in one paragraph.
- [x] Prior art and constraints are recorded in the signal ledger.
- [x] Open questions are either answered or explicitly deferred.
- [x] The next stage (spec) has a clear starting point.
- [x] The wrong implementation this work rejects is named.
- [x] No conformance rule added here asserts a literal position value.
- [x] Any `[FROZEN]` clause this touches is changed by a new ADR, written first.
- [x] If a rule here seems wrong, it is fixed and the reason given in the same change.
