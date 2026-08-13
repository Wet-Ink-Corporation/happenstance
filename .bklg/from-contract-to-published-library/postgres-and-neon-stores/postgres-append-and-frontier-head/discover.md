---
item: HS-S0062
stage: discover
created: 2026-08-12T13:02:30.344Z
updated: 2026-08-12T13:02:30.344Z
template_sig: 86ce4036
rendered_sig: 303cbefd
---

# Discover — A real Postgres append path and a frontier head

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| The slice's one line: a real `append` / `head` / `contains_event_id` that buys ES-10's visibility invariant, refuses over-limit batches with `ExceedsStoreLimit`, and turns `event_store_conformance!` and `event_store_model_conformance!` green against a live pinned Postgres | `_storymap.md`, *Slices* table, `postgres-append-and-frontier-head` row | Root A is the deliverable: the four methods on the port impl, not a helper module beside them |
| **AC-002** — the visibility rule is one the adapter had to work to pass | `project.md`, *Acceptance criteria*, AC-002 | This story owns the **pass** of `nothing_below_an_observed_position_appears_later`; `postgres-rule-controls` (HS-S0064) owns the proof it was not free (`_storymap.md`, *Coverage*, AC-002 row) |
| **AC-004** — Postgres passes the whole suite for real | `project.md`, *Acceptance criteria*, AC-004 | This story owns the two conformance families; the mutant-control column is HS-S0064's |
| **AC-010** — both stores declare their real limits | `project.md`, *Acceptance criteria*, AC-010 | This story owns the Postgres half: the ceilings, the `ExceedsStoreLimit` refusal, and VT-21 – VT-24. Neon's half is `neon-append-and-read-over-http`'s |
| `depends_on: postgres-schema-and-live-fixture` (HS-S0061) | manifest; `_storymap.md`, *Merge order* item 2 | Supplies migration 1 with its identity, time and mechanism columns, `PostgresFixture`, the whole-invocation gating, the live-Postgres CI job, and the `proptest` dev-dependency feature the model family needs to compile |
| Three of the four port methods are `todo!()`; `read` is the one that is already real | `crates/happenstance-postgres/src/event_store.rs:136-144`, `:146-161`, `:163-174`, `:124-134` | A `todo!()` body type-checks against any signature (`RUNBOOK.md:3061-3068`), so "it compiles" counts for nothing. `read` must stay lazy — nothing is acquired until the first poll |
| `head` must return the **visibility frontier**, not `max(position)`; under `xid8` it is `max(position) WHERE xid < pg_snapshot_xmin(pg_current_snapshot())` and trails the maximum | `crates/happenstance-postgres/src/event_store.rs:146-161` | Writing the cheap version "would encode the answer by accident". The module already says so in terms |
| ES-30's `head_is_the_highest_visible_position` asserts a **bound**, not an equality, precisely to admit a frontier | `spec/SPECIFICATION.md:2841-2842`; `crates/happenstance-testkit/src/suite.rs:1798` | The rule that would seem to police `head` cannot distinguish a frontier from a high-water mark. That is deliberate, and it is why the mutant below is invisible |
| ADR-0013's binding caveats: the invariant is global, `head()` is a frontier not an identity, read-your-own-writes does **not** hold, staleness is bounded by the longest open write transaction anywhere in the cluster | `.kb/decisions/0013-position-assignment-and-visibility.md` | Not open for re-litigation by this story. `postgres-structural-bill` (HS-S0066) writes them where a consumer meets them |
| The read half: the snapshot comes from `BEGIN ISOLATION LEVEL REPEATABLE READ` + `DECLARE CURSOR`, held open across the stream's life; keyset pagination is named and rejected | `crates/happenstance-postgres/src/read_stream.rs:59-69`, and the three-step owner shape at `:42-50` | The frontier predicate belongs *inside* that `DECLARE`. ES-11 requires one `read` to be evaluated against a single consistent state fixed no later than the first poll |
| The ceilings are `Option<usize>` facts, and refusals go through `AppendError::ExceedsStoreLimit { limit: StoreLimit::… }`, never `AppendError::Store` | `crates/happenstance-testkit/src/contract.rs:253`, `:262`, `:279`; `_decomposition.md`, *Architecture brief*, AC-010 row | Stating a number commits the store: the rule appends exactly that many bytes and requires acceptance, then one more and requires the typed refusal |
| VT-21 – VT-24 are `[PROVISIONAL]` and their rules are named in the clause table | `spec/SPECIFICATION.md:8547-8550`; `crates/happenstance-testkit/src/suite.rs:3775` | AC-013's audit reads these rows. A store MAY state a ceiling below the VT-21 floor and will then correctly fail the minimum-payload rule — the two are independent and both owed an answer |
| `contains_event_id` needs `origin_store` / `origin_position` columns the intended schema does not have, and inherits a replication question, not only a blockage | `crates/happenstance-postgres/src/event_store.rs:163-174` | Under a frontier mechanism, `true` for a committed row above the frontier disagrees with `read`; `false` makes ingest re-accept an event the store holds. ES-41 is `[PROVISIONAL]` (`spec/SPECIFICATION.md:8623`) |
| CF-6's linter scans only the testkit's three rule files | `xtask/src/lints.rs:631`; `xtask/src/spec_trace.rs:85-89` | `cargo xtask lint-position-literals` never reads `crates/happenstance-postgres/tests/`. In the one project where gaps stop being hypothetical, CF-6 is enforced here by discipline, not by a tool |

## Questions

**Answered.**

1. *Which visibility mechanism does this story wire?* The one ADR-0013 already
   established as affordable — `xid8` + `pg_snapshot_xmin` — because a working
   append path is the *precondition* of the measurement ADR-0024 owes, not its
   consequence (`_decomposition.md`, *Architecture brief* §8 point 2;
   `.kb/open-questions/postgres-arm-c-structural-cost.md`, which says plainly that
   the experiment "measured four SQL strategies, not four implementations of
   `SendEventStore`"). The **choice** stays HS-S0065's, and `spec` must say so
   rather than let this story's SQL become an unrecorded decision.
2. *What does `head` return?* The frontier. `spec` states it as a signature-level
   promise with the staleness consequence attached, because ES-30's rule cannot tell
   the two apart and therefore cannot be relied on to enforce it.
3. *Where does the frontier predicate live on the read path?* Composed into the
   cursor's `DECLARE`, inside the same `REPEATABLE READ` transaction the stream
   already holds open (`crates/happenstance-postgres/src/read_stream.rs:59-69`).

**Answered, and correcting the story map's count.**

3a. *How many `todo!()` bodies does this story own?* **Five, not three.** Beyond
    `append` (`crates/happenstance-postgres/src/event_store.rs:143`), `head` (`:160`)
    and `contains_event_id` (`:173`), the read path carries two more —
    `crates/happenstance-postgres/src/read_stream.rs:301` (`DECLARE … CURSOR FOR`
    the query) and `:313` (decoding a row into a `SequencedEvent`). `read`'s
    *structure* is real; its two SQL-facing leaves are not, so no read rule can pass
    until they are written, and the frontier predicate this story composes into the
    cursor lands in the first of them. `spec` enumerates all five, because
    `deskeleton-and-package-readiness` (HS-S0072) asserts that **no** `todo!()`
    survives on either path and an unowned body is found there far too late.

**Deferred to `spec`.**

4. *Tag-matching strategy* — join table, `text[]` with GIN, or `jsonb`, with `Tags`
   canonically sorted so `@>` stays available
   (`crates/happenstance-postgres/src/lib.rs:37-41`;
   `_decomposition.md`, *Architecture brief* §9.3).
5. *Whether the append-condition SQL reuses Neon's CTE shape or exploits the
   interactive transaction Postgres has and Neon does not* (§9.3). Postgres can
   report `conflicting_position` either way; the trade is one aggregate index scan
   per append against a second statement inside a transaction that already exists.
6. *The three ceilings' actual numbers*, which need a live server to establish.

**Deferred to the owning stories.**

7. *How the adapter buys ES-10's visibility invariant, decided on numbers.*
   `adr-0024-position-visibility-mechanism` (HS-S0065). This story is the instrument
   that decision is measured against, and it deliberately lands first.
8. *Whether `conflicting_position` is a promise every adapter owes or a hint one may
   omit.* `neon-conflicting-position-verdict` (HS-S0070). ES-25 already answers it
   at the port level — it "is informational", an adapter that detects the conflict
   without learning which event caused it "MUST be permitted to report `None`", and
   callers "MUST NOT depend on it" (`spec/SPECIFICATION.md:3746-3748`). Postgres,
   with an interactive transaction, can supply it cheaply and should; that is a
   quality of this adapter, not a promise this story may make on the port's behalf.
9. *Whether `contains_event_id` answers `true` or `false` for a committed row above
   the frontier.* Record the answer and its reasoning; do not settle replication
   semantics here. `replication-identity-and-ingest` (HS-P0017) owns it and ES-41 is
   `[PROVISIONAL]`.
10. *CF-40's contested ownership of the fixture-limits surface.* DR-6: consume
    whatever a sibling settles; state the ceilings honestly and record the
    unresolved ownership rather than minting a policy
    (`.kb/open-questions/cf-40-fixture-limits-ownership.md`).

## Decision

The problem this slice solves is that `nextval()` allocates outside the
transaction, so a Postgres store violates ES-10 by construction and the failure is
silent: a writer takes 99, a writer that started later takes 100 and commits first,
and a reader that conditioned on `after: Some(100)` never sees the 99 that would
have invalidated its decision — no error, no rejected append, no failing test, and
a read model that is correct about a state the business forbids. This story writes
the five `todo!()` bodies against that reality: an `append` that allocates and
records visibility in one transaction, a `head` that reports the frontier rather
than the maximum, a `contains_event_id` whose answer is recorded as a replication
question rather than guessed, the cursor `DECLARE` and the row decoder the read path
still lacks, and the two conformance families green against a live
pinned Postgres. The spec will cover: the append transaction and its statements;
`head`'s frontier expression and the staleness it implies; the frontier predicate's
composition into the existing cursor `DECLARE`; the three ceilings and the
`ExceedsStoreLimit` refusal path with its `StoreLimit` discriminant; the error-enum
growth for the mechanism's new decode failures (extending the existing
`#[non_exhaustive]` enums rather than flattening `sqlx::Error::Database` per
`SQLSTATE`, per ADR-0009); and the two macro invocations in
`crates/happenstance-postgres/tests/`. ES-10 is `[FROZEN]` and this story
*implements* it — it changes no clause, so no ADR is owed by this story; if the
frontier proves unimplementable, that is ADR-0024's record or an amendment record,
never an edit.

## The wrong implementation

**`head()` written as `SELECT max(position) FROM event`.** It compiles, it is one
line, and it passes everything. `head_is_the_highest_visible_position` asserts a
*bound* rather than an equality precisely so a frontier is admissible
(`spec/SPECIFICATION.md:2841-2842`), so a maximum satisfies it too — the rule that
looks like it polices `head` structurally cannot tell the two apart. Every other
family agrees, because with one writer the frontier *is* the maximum, and neither
`event_store_conformance!` nor `event_store_model_conformance!` creates a second
writer holding a transaction open. The live job is green; `cargo xtask ci` is green.
The defect surfaces only in production, as a projection checkpointing on `after`
that skips the events appearing below its checkpoint: ADR-0013's caveat 2 and the
module's own note at `crates/happenstance-postgres/src/event_store.rs:146-161`
describe exactly this, which is why that body is `todo!()` rather than the obvious
`SELECT`. It is a **high-water mark where the contract wants a frontier**, and the
whole reason this crate is in the tree is that no existing adapter can tell the
difference.

Its read-side twin: **compute the frontier from a pooled connection outside the read
transaction, once per `FETCH`.** The pool makes this feel natural — acquiring is
cheap, and the predicate looks like a filter rather than part of the snapshot. Every
rule still passes on a quiet store. Under a concurrent writer the frontier advances
between chunks and a row *below* an already-yielded position appears in a later chunk
of the same `read`, which is simultaneously an ES-10 inversion and an ES-11 snapshot
violation — and it is invisible to the suite because
`read_result_is_stable_under_concurrent_append` cannot manufacture the specific
interleaving. `crates/happenstance-postgres/src/read_stream.rs:59-69` already argues
the general form of this against keyset pagination; the frontier is the same mistake
one layer in.

Neither mutant belongs in `crates/happenstance-testkit/tests/` — both require a live
server, which DR-9 keeps out of `cargo test --workspace`, and the testkit may not
depend on an adapter. Both belong in `crates/happenstance-postgres/tests/`, driven
through the public rule functions (`crates/happenstance-testkit/src/lib.rs:189`) as
`postgres-rule-controls` (HS-S0064) establishes, so that "the adapter passed" is
distinguishable from "no rule could have failed it".

**On literal positions.** No conformance rule is added by this story, and every
assertion it does write compares against positions the store actually assigned. That
commitment is not backed by a tool here: `cargo xtask lint-position-literals` iterates
`RULE_FILES` (`xtask/src/lints.rs:631`), which is three files under
`crates/happenstance-testkit/src/` (`xtask/src/spec_trace.rs:85-89`) and does not
include any adapter's `tests/`. A `assert_eq!(positions, [1, 2, 3])` in this story's
test target would pass the gate and then fail against any conformant store that
leaves a gap — and this adapter leaves them by construction, because a rolled-back
append consumes allocations. `spec` states the rule explicitly for that reason.

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
