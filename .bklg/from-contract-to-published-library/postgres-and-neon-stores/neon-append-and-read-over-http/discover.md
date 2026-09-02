---
item: HS-S0069
stage: discover
created: 2026-08-12T13:02:37.629Z
updated: 2026-08-12T13:02:37.629Z
template_sig: 86ce4036
rendered_sig: c8ee18bf
---

# Discover — The suite runs over one-shot HTTP

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| The slice's one line: `event_store_conformance!` runs to completion against a store with no connection, no interactive transaction and no cursor, every rule reporting pass, fail, or a skip carrying the fixture's stated reason | `_storymap.md`, *Slices* table, `neon-append-and-read-over-http` row | The deliverable is a **complete run**, not a green run. A rule reporting `fail` is a result; a rule that is absent is a defect |
| **AC-006** — the suite is invoked against a `NeonEventStore` fixture backed by a real Neon `/sql` endpoint | `project.md`, *Acceptance criteria*, AC-006 | This story owns "the suite run against it"; the endpoint being reachable is `neon-sql-transport`'s |
| **AC-007** — for each rule Neon does not pass there is exactly one of: a declared capability with a written reason, or an accepted decision record amending the clause with the suite re-run | `project.md`, *Acceptance criteria*, AC-007 | This story owns "every rule present in the run and reporting". No rule is `#[cfg]`-ed out, and no rule is absent |
| **AC-010** — both stores declare their real limits | `project.md`, *Acceptance criteria*, AC-010 | The Neon half: `MAX_RESPONSE_BYTES`-anchored ceilings exercised, and VT-21 – VT-24 either passing or refused with `AppendError::ExceedsStoreLimit` rather than an opaque store error |
| `depends_on: neon-fixture-and-live-job` (HS-S0068) | manifest; `_storymap.md`, *Merge order* item 4 | Supplies `NeonFixture`, its capability constants and ceilings, and the credentialed job this run happens inside |
| `depends_on: postgres-append-and-frontier-head` (HS-S0062) | manifest; `_storymap.md`, *Slices* table, this row | "Neon inherits migration 1, the tag storage and the append-condition SQL — the reason these are one project and not two". Also the frontier answer `head` is blocked on: the body says so in terms (`crates/happenstance-neon/src/event_store.rs:210-217`) |
| The `todo!()` bodies this story faces, by line | `crates/happenstance-neon/src/event_store.rs:115`, `:164`, `:218`, `:228`, `:254`, `:262` | `read_request`, `conditional_append_request`, `head`, `contains_event_id`, `decode_append_response`, `decode_read_response`. The story map names three; the crate has six on the event-store path (see question 1) |
| The impl above them is already wired: it refuses the empty batch first, maps a transport failure to `AppendError::Store`, and maps `Conflict` to `AppendError::ConditionViolated(ConditionViolated::at(position))` | `crates/happenstance-neon/src/event_store.rs:183-206` | The control flow is not this story's to invent. What is missing is request construction and decoding |
| `NeonReadStream` is "a *shape*, not a capability" — one round trip, then a buffer drained — and its laziness is load-bearing on `wasm32`, where issuing a `fetch` outside a polled future happens off the runtime's event loop | `crates/happenstance-neon/src/event_store.rs:265-282` | The honest part of the crate. ES-11 comes free — one round trip *is* one snapshot — and ES-42's no-`Unpin` return is the shape being preserved |
| Bare `EventStore` only, on both targets; a second `SendEventStore` impl is `error[E0119]` against `trait_variant`'s blanket impl | `crates/happenstance-neon/src/lib.rs:79-89` | Compiled fact, not preference. The concurrency family is Postgres-only by design (`crates/happenstance-testkit/src/lib.rs:101-104`) and its absence here is not a gap |
| A declined capability still emits a test returning `RuleOutcome::Skipped`; the forbidden move is `#[cfg]`-ing a rule out of the expansion | `crates/happenstance-testkit/src/contract.rs:26-42`; **DR-5** (`project.md`) | "never a silent pass and never a `#[cfg]`-ed-out test" |
| Where Neon cannot pass a rule, "the rule's clause is what has to give, and that is a specification amendment, not an adapter workaround" | `RUNBOOK.md:4361-4363` | Initiative DoD 6 contemplates the contract giving. DR-7: that is a second decision atom with the suite re-run, never an edit |
| The whole macro expansion must run, not a hand-picked subset | `_decomposition.md`, *Testing brief* → *Notes* §6 | "or AC-007's 'no rule is absent from the run' is unproven by construction" |
| ES-11 and ES-12 are `[PROVISIONAL]`, and this store is the transport axis's far end for both | `spec/SPECIFICATION.md:8593-8594`; `RUNBOOK.md:606`, `:688` | Whatever this run reports is what `far-end-discharge-record` (HS-S0073) writes down for the publication audit |
| The ceilings are facts the fixture asserted, and a refusal must be `AppendError::ExceedsStoreLimit { limit: StoreLimit::… }`, never `AppendError::Store` | `crates/happenstance-testkit/src/contract.rs:253`, `:262`, `:279` | An opaque store error at the ceiling is a **defect**, not a decline. That mapping is this story's to write |
| CF-6's linter scans only the testkit's three rule files | `xtask/src/lints.rs:631`; `xtask/src/spec_trace.rs:85-89` | Any assertion this story writes in `crates/happenstance-neon/tests/` is unpoliced. Compare against positions the store assigned |

## Questions

**Answered.**

1. *How many `todo!()` bodies does this story own?* The story map names three
   (`conditional_append_request`, `decode_append_response`, `decode_read_response`).
   The event-store path actually carries six: add `read_request`
   (`crates/happenstance-neon/src/event_store.rs:115`), `head` (`:218`) and
   `contains_event_id` (`:228`). Two more — `probe_request` (`:122`),
   `insert_request` (`:127`) — plus `decode_probe_response` (`:497`) and
   `decode_last_position` (`:502`) exist **only** for `ProbeThenWriteStore` and
   belong to `neon-conflicting-position-verdict` (HS-S0070), which consumes that
   type. `spec` enumerates the split explicitly, because HS-S0072 asserts that no
   `todo!()` survives and an unowned body is discovered there far too late.
2. *Which flavour, and does the concurrency family apply?* Bare `EventStore` only,
   on both targets, and no. A second `SendEventStore` impl does not compile
   (`crates/happenstance-neon/src/lib.rs:79-89`), and the concurrency family is
   opt-in with an `F::Store: Send` bound that a `!Send` adapter is not expected to
   meet.
3. *What does `head` return here?* Whatever `happenstance-postgres` settled, which
   is why this story depends on HS-S0062 and why the body's own comment refuses to
   write `SELECT max(position)` before then
   (`crates/happenstance-neon/src/event_store.rs:210-217`). Neon is Postgres over
   one-shot HTTP; it inherits the frontier rather than choosing one.
4. *What happens when a rule genuinely cannot pass?* Exactly one of two things: a
   `Capability` decline carrying the fixture's stated reason, which still emits a
   test reporting `RuleOutcome::Skipped`; or an accepted decision record amending
   the clause, with the suite re-run afterwards. Never a `#[cfg]`, never a
   hand-picked subset, never a silent pass.

**Deferred to `spec`.**

5. *The CTE's exact parameter shape* — how `Tags` and `Query` items compile into the
   `unnest`/`@>` forms, inherited from the Postgres append-condition SQL rather than
   invented, and how hex `bytea` rendering interacts with the batch ceiling.
6. *Whether `contains_event_id` is answerable at all*, given the schema carries no
   origin columns and ES-41 is `[PROVISIONAL]` (`spec/SPECIFICATION.md:8623`). If it
   is a decline or an amendment rather than a body, that is a result to record, not
   a gap to hide.
7. *Whether the one-statement CTE actually runs at `Serializable`*, given the
   isolation header "applies only when… more than one statement"
   (`crates/happenstance-neon/src/transport.rs:56-60`). Surfaced by
   `neon-sql-transport` (HS-S0067), consumed here, and the verdict is HS-S0070's.

**Deferred to the owning stories.**

8. *How the adapter buys ES-10's visibility invariant when `nextval()` allocates
   outside the transaction.* `adr-0024-position-visibility-mechanism` (HS-S0065),
   through HS-S0062, which this story depends on precisely so it inherits an answer
   rather than improvising one over HTTP.
9. *Whether `conflicting_position` is a promise every adapter owes or a hint one may
   omit.* `neon-conflicting-position-verdict` (HS-S0070). This story writes the
   decoder that *can* report it and must not quietly return `None` from
   `decode_append_response` to make a rule pass — that would settle AC-008 by
   accident, in the direction the ledger already assumes and the crate already
   disputes.

## Decision

The problem this slice solves is that the contract has never been asked to work
without the three things every implementation so far has had: a connection, an
interactive transaction, and a cursor. Neon has none of them — one round trip per
operation, a result set that is one buffered JSON document, and a hard 64 MiB
response cap — and the workspace's claim that the port is well designed rests on
whether a store shaped like that can pass the suite as written. This story runs it.
It writes the request builders and decoders the event-store path still lacks, keeps
`read`'s laziness intact because on `wasm32` issuing a `fetch` outside a polled
future happens off the event loop, maps ceiling refusals to `ExceedsStoreLimit`
rather than to an opaque store error, and invokes `event_store_conformance!`'s full
expansion inside the credentialed job — accepting a `fail` as a result and never a
disappearance as a convenience. The spec will cover: each `todo!()` body it owns and
the split against HS-S0070's; the CTE inherited from the Postgres append-condition
SQL; the decode paths and their error-enum growth; the ceiling refusal mapping; the
macro invocation and the requirement that it is the whole expansion; and the format
in which every rule's outcome is captured for AC-007 and for HS-S0073's audit.
**ES-11 and ES-12 are `[PROVISIONAL]` and ES-25 is `[FROZEN]`; this story changes
neither.** If a rule genuinely cannot pass, DR-7 applies: a new accepted decision
atom amending the clause, written first, with the suite re-run — never an edit, and
never a workaround in the adapter.

## The wrong implementation

**`ProbeThenWriteStore`**, and it is already in the tree, written out in full for
exactly this purpose (`crates/happenstance-neon/src/event_store.rs:388-503`). An
`append` that probes for a condition violation and then writes, in two statements,
**type-checks against `EventStore` perfectly** — nothing in the signature forbids two
round trips — so a shape table recording only compiler errors ranks this crate the
most compatible adapter in the workspace when it is the least. It is silently
unsound here because Neon has no interactive transaction: each round trip is its own
implicit transaction, the window between them is a full network round trip, and a
concurrent append committing inside it is invisible to the probe and unopposed by
the insert. That is a **lost update** — no error, no failing test, indistinguishable
after the fact from a legitimate append. Do not delete it, do not fix it, and do not
leave it merely documented: `neon-conflicting-position-verdict` (HS-S0070) runs it
through the public rule functions and requires at least one rule to *fail* against
it that the real store passes, using `catch_unwind` over a non-capturing probe
(`crates/happenstance-testkit/tests/mutation_coverage/harness.rs:329-332`). It stays
in `crates/happenstance-neon/src/`, not in `crates/happenstance-testkit/tests/`,
because it is generic over `SqlTransport` and needs a live endpoint to be wrong in
the way it is wrong.

**This story's own mutant is quieter: a decoder that returns `None` where it should
return a position, or an `Ok` where it should return a conflict.** Write
`decode_append_response` to map "no `appended` value" to
`AppendOutcome::Conflict(…)` with a position dredged from anywhere convenient, or —
more likely — to treat the `conflict` column being absent as "no conflict". The
suite is *mostly* insensitive to this: `ConditionViolated::conflicting_position` is
informational, an adapter "MUST be permitted to report `None`", and "callers MUST
NOT depend on it" (`spec/SPECIFICATION.md:3746-3748`). So a decoder that loses the
conflicting position passes every rule, the run is green, AC-006 and AC-007 are
satisfied — and AC-008 has been answered in the direction the decision ledger
already assumed, by a decode bug, in the story that was not supposed to touch it.
The crate's own doc says the opposite is true and calls it out as "contrary to the
standing assumption in the decision ledger"
(`crates/happenstance-neon/src/lib.rs:46-77`). `spec` therefore requires the decoder
to distinguish "the endpoint reported no conflicting position" from "the decoder did
not look", and routes the verdict to HS-S0070 rather than letting it fall out of an
implementation detail.

**And the completeness mutant: a hand-picked subset.** Invoke the rules that pass,
or split the invocation "for readability", or gate the ones that fail behind a
`#[cfg]` while a fix is pending. The job goes green, and AC-007's "no rule is absent
from the run" becomes unprovable by construction — nothing in the testkit counts
rules per adapter, and `capability_skips_are_reported`
(`crates/happenstance-testkit/src/suite.rs:58`) reads *capabilities*, so a rule
removed by `#[cfg]` declines nothing and simply is not there. DR-5 names this and it
is the one gating shape the project forbids outright.

**On literal positions.** No conformance rule is added here, and every assertion this
story writes compares against the positions the store actually assigned. That is
discipline, not enforcement: `cargo xtask lint-position-literals` iterates
`RULE_FILES` (`xtask/src/lints.rs:631`), which is three files under
`crates/happenstance-testkit/src/` (`xtask/src/spec_trace.rs:85-89`) and never reads
`crates/happenstance-neon/tests/`.

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
