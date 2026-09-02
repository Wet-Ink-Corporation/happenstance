---
item: HS-S0071
stage: discover
created: 2026-08-12T13:02:39.966Z
updated: 2026-08-12T13:02:39.966Z
template_sig: 86ce4036
rendered_sig: 2e3dc04a
---

# Discover — PostgresProjectionStore against the frozen Batch

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| The slice's one line: `PostgresProjectionStore` written against the `Batch` shape `projection-store-freeze` freezes, passing the projection suite in the live Postgres job, with any declined capability reported by name and reason | `_storymap.md`, *Slices* table, `postgres-projection-store` row | A slice of one story, sequenced last so it can slide right without moving anything else |
| **AC-005** — passes the projection suite against the owned `Batch` frozen by `projection-store-freeze`, with any declined capability reported by name and reason | `project.md`, *Acceptance criteria*, AC-005 | Sole owner |
| `depends_on: postgres-schema-and-live-fixture` (HS-S0061) | manifest; `_storymap.md`, *Merge order* item 5 | Supplies the migration, the `PgPool`, the whole-invocation gating and the live Postgres CI job the suite runs inside. The checkpoint table joins the same migration |
| **The real gate is a project-level `blocked_by`, not a story edge**: HS-P0010 reads `stage: storymap`, `status: planning` today | `_grounding.md`, *Dependency status*; `_decomposition.md`, *Architecture brief* §3 tension 2 | "`PostgresProjectionStore` cannot be written against a `Batch` shape that is not frozen, and writing it early and reworking it is the failure `project.md`'s risk table names" |
| The batch shape is already recorded, and it is an **owned** type bound to a GAT: `type Batch<'a> = Transaction<'static, Postgres> where Self: 'a` | `crates/happenstance-postgres/src/projection_store.rs:100-103`; `references/adapter-shapes.md` | "The lifetime parameter is accepted and then ignored", which is PS-5's working hypothesis stated as an adapter that does not need the parameter |
| Why `'static` is not a convenience: `PgPool::begin` takes the `PoolConnection` **by value** and returns it to the pool on drop — "a driver with every opportunity to hand back a handle borrowed from `&self` chose to hand back an owned one" | `crates/happenstance-postgres/src/projection_store.rs:12-29` | Evidence phase 6 needs, and the reason `Transaction<'static, Postgres>` "confirmed the projection freeze" (`RUNBOOK.md:4317-4319`) |
| The invariant: "a read-model write and its checkpoint write must be **one transaction**, or a restart either replays applied events or skips unapplied ones" — `commit` does the checkpoint `UPSERT` on the same `Transaction` the caller has been writing through, then commits once | `crates/happenstance-postgres/src/projection_store.rs:43-49` | Documented on the module and on the trait. Documented, and — see below — checked by nothing that exists today |
| Four `todo!()` bodies: `checkpoint`, `begin`, `commit`, `rollback` | `crates/happenstance-postgres/src/projection_store.rs:109`, `:113`, `:122`, `:126` | The whole port surface. `commit` is where the invariant lives or dies |
| The intended checkpoint schema is already written out | `crates/happenstance-postgres/src/projection_store.rs:52-58` | `projection_checkpoint (projection_id text PRIMARY KEY, position bigint NOT NULL)`. Not invented here |
| The projection store port "has no conformance suite yet, and a port without one is a guess" | `CLAUDE.md`, *Open questions, deliberately unresolved* | The suite this story is run against does not exist. It arrives from HS-P0010, and until then nothing can fail this adapter |
| The features already exist: `default = ["event-store", "projection-store"]` | `crates/happenstance-postgres/Cargo.toml` `[features]`; `_decomposition.md`, *Deployment brief* → *Notes* | This story "fills the second feature's body rather than adding a new one" |
| Freezing `ProjectionStore`, writing the projection suite and setting the capability-declension policy are all **out of scope**; this project is a consumer of all three | `project.md`, *Out of scope* | Consume, do not re-decide. If the shape moves after merge that is a re-plan input, not a silent fix |
| The global-versus-per-boundary visibility question rests on the projection checkpoint being global, and is **owned by phase 6** | `.kb/open-questions/global-versus-per-boundary-visibility-invariant.md`; `project.md`, *Out of scope* | Read for context. This story is the place it would be settled by accident, and must not be |
| ADR-0009: two independent `#[non_exhaustive]` `thiserror` enums, one per port | `.kb/decisions/0009-error-send-sync.md`; `crates/happenstance-postgres/src/error.rs` | `PostgresProjectionStoreError` is already declared separately from the event store's. Extend it rather than merging the two |

## Questions

**Answered.**

1. *What is the `Batch` shape?* `sqlx::Transaction<'static, Postgres>`, owned,
   bound to today's GAT with a `where Self: 'a` clause for a lifetime nothing reads
   (`crates/happenstance-postgres/src/projection_store.rs:100-103`). Recorded at
   phase 2 and unchanged; this story does not rediscover it.
2. *Where does the invariant live?* In `commit`, and only there: the checkpoint
   `UPSERT` runs on the caller's own `Transaction`, and the commit happens once
   (`crates/happenstance-postgres/src/projection_store.rs:43-49`).
3. *Is a new feature needed?* No. `projection-store` already exists and is
   default-on; this story fills its body.

**Deferred — and this one gates the story rather than the spec.**

4. *The frozen `Batch` shape, the projection suite, and the capability-declension
   policy* all arrive from `projection-store-freeze` (HS-P0010), which is at
   `stage: storymap`. `spec` may be written against the recorded shape, but the
   story cannot be **done** until the freeze lands, and the slice is deliberately
   last so it can slide right. If HS-P0010 changes the shape after this story
   merges, that is a re-plan input rather than a silent fix.
5. *Whether the suite has a declinable capability this store should decline*, and
   under what policy it is reported. Consumed from HS-P0010, not minted here.

**Deferred to the owning stories and projects.**

6. *How the adapter buys ES-10's visibility invariant when `nextval()` allocates
   outside the transaction.* `adr-0024-position-visibility-mechanism` (HS-S0065).
   It reaches this story only through the *reading* side of a projection runner,
   which is not this port's surface — but note that under a frontier `head` a
   projection checkpointing on `after` is exactly the caller ADR-0013's caveats are
   about, and `postgres-structural-bill` (HS-S0066) is where that is written down.
7. *Whether the visibility invariant should have been per-boundary rather than
   global.* Owned by phase 6 — `projection-store-freeze` — and named in *Out of
   scope*. This is the story where it could be settled by accident, by shaping the
   checkpoint table per boundary; `spec` states the global shape explicitly so the
   choice is visible.
8. *Whether `conflicting_position` is a promise every adapter owes or a hint one may
   omit.* `neon-conflicting-position-verdict` (HS-S0070). No bearing here — this
   port has no append condition.

## Decision

The problem this slice solves is that `happenstance-postgres` claims to be a
projection store and implements none of it: `checkpoint`, `begin`, `commit` and
`rollback` are all `todo!()`, and the crate's real contribution so far has been a
*type* — the finding that a pooled, networked driver hands back an owned
`Transaction<'static, Postgres>` rather than one borrowed from `&self`, which is the
evidence phase 6's `Batch` freeze rests on. This story turns that finding into an
implementation: the checkpoint table in the same migration, the four bodies, and the
one invariant that matters — the read-model write and the checkpoint write commit
together or not at all — then runs whatever suite HS-P0010 freezes against it inside
the live Postgres job. The spec will cover: the checkpoint schema and its migration;
each of the four bodies; `commit`'s single-transaction discharge of the invariant;
error-enum growth on `PostgresProjectionStoreError` rather than on the event store's;
the suite invocation and any declined capability with its stated reason; and an
explicit note that the checkpoint is global rather than per-boundary, so that phase
6's open question is not answered here by the shape of a table. No `[FROZEN]` clause
is touched — the `PS` clauses this exercises are provisional and owned by HS-P0010 —
so no ADR is owed by this story.

## The wrong implementation

**`commit` that writes the checkpoint in its own transaction.** Take the caller's
`Transaction`, commit it, then `UPSERT` the checkpoint through a fresh pool
connection. It is the natural shape if `commit`'s signature is read as "finish the
caller's batch, then record progress", the borrow checker is entirely happy with it,
and — this is the part that matters — **nothing in this repository can fail it
today**. `ProjectionStore` has no conformance suite (`CLAUDE.md`, *Open questions*),
the invariant is prose on a module and on the trait
(`crates/happenstance-postgres/src/projection_store.rs:43-49`), and prose is not an
instrument. `cargo xtask ci` is green, the live Postgres job is green, AC-005 reads
as satisfied because the store "passes the projection suite" — and a process that
dies in the window between the two commits leaves a read model that has applied
events its checkpoint does not know about, so the restart replays them. For a
projection that is not idempotent, that is silent corruption of a read model with no
error anywhere in the log. This is the far end of "a port without a suite is a
guess": the guess here is that an adapter author will read the module doc.

The rejection is not something this story may build — writing the projection suite is
`projection-store-freeze`'s, explicitly out of scope (`project.md`, *Out of scope*) —
which is precisely why the story is sequenced last and gated on HS-P0010. What `spec`
owes instead is a **named requirement on the upstream suite**: it must contain a rule
that fails a two-transaction `commit`, with a crash injected between them, and this
adapter is the first implementation capable of failing it for real. If HS-P0010's
suite arrives without such a rule, that is a finding to raise there, in that project,
before this story is called done — not a rule this project adds to a port it does not
own.

**The second mutant is a schedule mutant, and the risk table already names it:**
writing `PostgresProjectionStore` now, against the shape recorded at phase 2, and
reworking it when the freeze lands. Every check passes at every intermediate point,
because the recorded shape compiles and always did. What it destroys is the evidence:
`Transaction<'static, Postgres>` is one of the inputs phase 6 weighs when deciding
whether `Batch` carries a lifetime at all, and an adapter already written against one
answer is a thumb on that scale that nobody records.

**And the quiet one: a per-projection checkpoint table shaped per boundary.** It looks
like a schema detail. It is the premise ADR-0013's global visibility invariant rests
on (`.kb/open-questions/global-versus-per-boundary-visibility-invariant.md`), it is
owned by phase 6, and choosing it here would settle a live open question by picking a
primary key. `spec` states the global shape and the reason, so that if phase 6 goes
the other way the change is visible rather than pre-empted.

This story adds no conformance rule, so nothing is owed to
`crates/happenstance-testkit/tests/`; the rule the first mutant needs belongs to
HS-P0010's suite, and naming it there is this story's contribution to it.

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
